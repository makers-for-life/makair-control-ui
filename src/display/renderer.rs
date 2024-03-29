// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::time::{Duration, Instant};

use conrod_core::Ui;
use plotters_conrod::ConrodBackendReusableGraph;

use crate::chip::settings::{ChipSettingsEvent, ChipSettingsIntent};
use crate::chip::{Chip, ChipEndOfLine, ChipEndOfLineEnd, ChipEndOfLineStep, ChipError, ChipState};
use crate::config::environment::*;
use crate::utilities::{
    index::{index_from_end_of_line_failure, index_from_end_of_line_step},
    parse::parse_version_number,
};

use super::data::*;
use super::events::DisplayUiEvents;
use super::fonts::Fonts;
use super::identifiers::{Ids, ImageIds};
use super::screen::{Screen, ScreenModalsOpen};

const WAITING_FOR_DATA_TIMEOUT_AFTER: Duration = Duration::from_secs(10);
const DISPATCH_HEARTBEAT_EVERY: Duration = Duration::from_secs(1);

pub struct DisplayRendererSettingsState {
    visibility: DisplayRendererSettingsStateVisibility,
    last_close: Option<Instant>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum DisplayRendererSettingsStateVisibility {
    Opened,
    Closed,
}

pub struct DisplayRendererBuilder;

pub struct DisplayRenderer {
    fonts: Fonts,
    ids: Ids,
    states: DisplayRendererStates,
    plot_graphs: (ConrodBackendReusableGraph, ConrodBackendReusableGraph),
    pub images: ImageIds,
}

#[derive(Default)]
pub struct DisplayRendererStates {
    pub preset_settings: DisplayRendererSettingsState,
    pub run_settings: DisplayRendererSettingsState,
    pub snooze_settings: DisplayRendererSettingsState,
    pub advanced_settings: DisplayRendererSettingsState,
    pub mode_settings: DisplayRendererSettingsState,
}

impl DisplayRendererSettingsState {
    pub fn open(&mut self) {
        self.update_to(DisplayRendererSettingsStateVisibility::Opened);
    }

    pub fn close(&mut self) {
        self.update_to(DisplayRendererSettingsStateVisibility::Closed);
    }

    pub fn update_to(&mut self, visibility: DisplayRendererSettingsStateVisibility) {
        // Update values? (if changed)
        if self.visibility != visibility {
            self.visibility = visibility;

            // Update last close value? (if went from opened to closed visibility)
            if self.visibility == DisplayRendererSettingsStateVisibility::Closed {
                self.last_close = Some(Instant::now());
            }
        }
    }

    pub fn has_change(&self, new_visibility: &DisplayRendererSettingsStateVisibility) -> bool {
        &self.visibility != new_visibility
    }

    pub fn is_open(&self) -> bool {
        self.visibility == DisplayRendererSettingsStateVisibility::Opened
    }

    pub fn is_debounced(
        &self,
        new_visibility: &DisplayRendererSettingsStateVisibility,
        delay: Duration,
    ) -> bool {
        // Only check debounce status if new visibility is 'opened', the goal being avoiding \
        //   re-opening the modal as soon as it gets closed by the user, due to the telemetry \
        //   channel not having yet acknowledged submitted data, and still broadcasting telemetry \
        //   snapshots w/ the previous data. It should only be a matter of milliseconds, hence why \
        //   this debounce is an efficient yet simple way to fix that UI blink 'glitch'.
        // Notice: 'closed' visibility updates do not need to be debounced, hence why this method \
        //   returns 'false' all the time in such case.
        if new_visibility == &DisplayRendererSettingsStateVisibility::Opened {
            // Return whether modal is requested to be opened before last closed delay
            if let Some(last_close) = self.last_close {
                return last_close.elapsed() < delay;
            }
        }

        false
    }
}

impl Default for DisplayRendererSettingsState {
    fn default() -> Self {
        Self {
            visibility: DisplayRendererSettingsStateVisibility::Closed,
            last_close: None,
        }
    }
}

#[allow(clippy::new_ret_no_self)]
impl DisplayRendererBuilder {
    pub fn new(fonts: Fonts, ids: Ids, images: ImageIds) -> DisplayRenderer {
        DisplayRenderer {
            fonts,
            ids,
            states: DisplayRendererStates::default(),
            images,
            plot_graphs: (
                ConrodBackendReusableGraph::build(),
                ConrodBackendReusableGraph::build(),
            ),
        }
    }
}

impl DisplayRenderer {
    pub fn render(&mut self, interface: &mut Ui, chip: &Chip) {
        match &chip.state {
            // Waiting for data from the motherboard, treat it as a 'connecting...' state
            ChipState::WaitingData(started_time) => {
                // The UI has been waiting for data for too long? Show an error instead, though \
                //   we are still waiting for data, so this may fix by itself. This is done for UI \
                //   purposes, though the chip state is still 'ChipState::WaitingData'.
                if started_time.elapsed() >= WAITING_FOR_DATA_TIMEOUT_AFTER {
                    self.error(interface, &ChipError::TimedOut)
                } else {
                    self.initializing(interface, true)
                }
            }
            // Initializing, treat it as a 'connected' state
            ChipState::Initializing => self.initializing(interface, false),
            // Running or stopped, handle data
            ChipState::Running | ChipState::Stopped => self.data(interface, chip),
            // An error occured
            ChipState::Error(err) => self.error(interface, err),
            // End-of-line test mode active
            ChipState::EndOfLine(eol) => self.end_of_line(interface, eol),
        };
    }

    pub fn run_events(
        &mut self,
        interface: &mut Ui,
        chip: &mut Chip,
        last_heartbeat: &Instant,
        tick_time: &Instant,
    ) -> (bool, bool, Vec<ChipSettingsIntent>, Vec<ChipSettingsEvent>) {
        // Run all UI events (defer to sub-handler)
        let (has_user_events, user_intents, user_events) =
            DisplayUiEvents::run(interface, &self.ids, chip, &mut self.states);

        // Check if should run heartbeat? (ie. if it should be sent to the firmware)
        let mut has_heartbeat = false;

        if tick_time.duration_since(*last_heartbeat) >= DISPATCH_HEARTBEAT_EVERY {
            has_heartbeat = true;
        }

        (has_heartbeat, has_user_events, user_intents, user_events)
    }

    pub fn has_state_moderate_framerate(&self) -> bool {
        // Returns whether a current state value should result in a moderate framerate, so that \
        //   resource usage is at a minimum.

        // The advanced settings modal is quite heavy to re-render every time, and we do not need \
        //   the values to be shown at full framerate. Plus values keep changing every millisecond \
        //   or so. Throttle down FPS.
        if self.states.advanced_settings.is_open() {
            return true;
        }

        false
    }

    fn initializing(&mut self, interface: &mut Ui, is_connecting: bool) {
        // Create initializing screen
        let screen_bootloader = DisplayDataBootloader {
            image_id: self.images.bootloader_logo,
            width: BOOTLOADER_LOGO_WIDTH as _,
            height: BOOTLOADER_LOGO_HEIGHT as _,
            connecting: is_connecting,
        };

        let mut screen = Screen::new(
            interface.set_widgets(),
            &self.ids,
            &self.fonts,
            (None, None),
            None,
            None,
            None,
        );

        screen.render_initializing(screen_bootloader);
    }

    fn error(&mut self, interface: &mut Ui, error: &ChipError) {
        // Create error screen
        let screen_error = DisplayDataError {
            image_id: self.images.error_icon,
            width: ERROR_ICON_WIDTH as _,
            height: ERROR_ICON_HEIGHT as _,
            error,
        };

        let mut screen = Screen::new(
            interface.set_widgets(),
            &self.ids,
            &self.fonts,
            (None, None),
            None,
            None,
            None,
        );

        screen.render_error(screen_error);
    }

    fn end_of_line(&mut self, interface: &mut Ui, eol: &ChipEndOfLine) {
        // Create end-of-line screen
        let screen_eol = DisplayDataEndOfLine {
            error: matches!(eol, ChipEndOfLine::Failed(_, _)),
            success: matches!(eol, ChipEndOfLine::Succeeded(_, _)),
            confirm: matches!(
                eol,
                ChipEndOfLine::Ongoing(ChipEndOfLineStep::Start, _)
                    | ChipEndOfLine::Ongoing(ChipEndOfLineStep::CheckFan, _)
                    | ChipEndOfLine::Ongoing(ChipEndOfLineStep::CheckBuzzer, _)
                    | ChipEndOfLine::Ongoing(ChipEndOfLineStep::CheckUiScreen, _)
                    | ChipEndOfLine::Ongoing(ChipEndOfLineStep::PlugAirTestSystem, _)
                    | ChipEndOfLine::Ongoing(ChipEndOfLineStep::ConfirmBeforeOxygenTest, _)
                    | ChipEndOfLine::Ongoing(ChipEndOfLineStep::WaitBeforeBlowerLongRun, _)
                    | ChipEndOfLine::Succeeded(ChipEndOfLineEnd::Confirm, _)
                    | ChipEndOfLine::Succeeded(ChipEndOfLineEnd::DisplayPressure, _)
                    | ChipEndOfLine::Succeeded(ChipEndOfLineEnd::DisplayFlow, _)
            ),
            step: match eol {
                ChipEndOfLine::Ongoing(eol_step, _) => {
                    index_from_end_of_line_step(&eol_step).unwrap_or(0)
                }
                ChipEndOfLine::Failed(eol_failure, _) => {
                    index_from_end_of_line_failure(&eol_failure).unwrap_or(0)
                }
                ChipEndOfLine::Succeeded(_, _) => END_OF_LINE_STEPS_COUNT,
            },
            icon_image_id: match eol {
                ChipEndOfLine::Ongoing(_, _) => self.images.end_of_line_ongoing_icon,
                ChipEndOfLine::Failed(_, _) => self.images.end_of_line_error_icon,
                ChipEndOfLine::Succeeded(_, _) => self.images.end_of_line_success_icon,
            },
            eol,
        };

        let mut screen = Screen::new(
            interface.set_widgets(),
            &self.ids,
            &self.fonts,
            (None, None),
            None,
            None,
            None,
        );

        screen.render_end_of_line(screen_eol);
    }

    fn data(&mut self, interface: &mut Ui, chip: &Chip) {
        // Create widgets
        let mut ui = interface.set_widgets();

        let ongoing_alarms = chip.ongoing_alarms_sorted();

        let (ongoing_alarms_count, widgets_alarms_count) =
            (ongoing_alarms.len(), self.ids.alarm_alarms.len());

        if ongoing_alarms_count > widgets_alarms_count {
            for i in widgets_alarms_count..ongoing_alarms_count {
                let index = i + 1;
                self.ids
                    .alarm_alarms
                    .resize(index, &mut ui.widget_id_generator());
                self.ids
                    .alarm_codes_containers
                    .resize(index, &mut ui.widget_id_generator());
                self.ids
                    .alarm_codes
                    .resize(index, &mut ui.widget_id_generator());
                self.ids
                    .alarm_messages_containers
                    .resize(index, &mut ui.widget_id_generator());
                self.ids
                    .alarm_messages
                    .resize(index, &mut ui.widget_id_generator());
            }
        } else {
            let (alarms_difference, alarm_id) = (
                widgets_alarms_count - ongoing_alarms_count,
                &mut ui.widget_id_generator(),
            );

            if alarms_difference > 0 {
                self.ids.alarm_alarms.resize(ongoing_alarms_count, alarm_id);
                self.ids
                    .alarm_codes_containers
                    .resize(ongoing_alarms_count, alarm_id);
                self.ids.alarm_codes.resize(ongoing_alarms_count, alarm_id);
                self.ids
                    .alarm_codes_containers
                    .resize(ongoing_alarms_count, alarm_id);
                self.ids
                    .alarm_messages
                    .resize(ongoing_alarms_count, alarm_id);
            }
        }

        // Create screen & its screen data
        let mut screen = Screen::new(
            ui,
            &self.ids,
            &self.fonts,
            (chip.boot_time, Some(chip.last_tick)),
            Some(&ongoing_alarms),
            Some(&chip.last_machine_snapshot),
            chip.last_data_snapshot.as_ref(),
        );

        let screen_data_layout = DisplayDataLayout {
            texture_header_image_id: match (
                &chip.state,
                chip.last_machine_snapshot.alarm_snoozed.unwrap_or(false),
            ) {
                (&ChipState::Running, false) => self.images.header_running,
                (&ChipState::Running, true) => self.images.header_running_snoozed,
                (_, false) => self.images.header_stopped,
                (_, true) => self.images.header_stopped_snoozed,
            },
        };

        let screen_data_branding = DisplayDataBranding {
            firmware_version: parse_version_number(
                if chip.last_machine_snapshot.version.is_empty() {
                    BRANDING_TEXT_VERSION_NONE
                } else {
                    &chip.last_machine_snapshot.version
                },
            ),
            width: BRANDING_WIDTH as _,
            height: BRANDING_HEIGHT as _,
        };

        let screen_data_status = DisplayDataStatus {
            chip_state: &chip.state,
            battery_soc: chip.estimated_soc,
        };
        let screen_data_heartbeat = DisplayDataHeartbeat {
            data_pressure: &chip.data_pressure,
        };

        let screen_data_graph = DisplayDataGraph {
            width: GRAPH_WIDTH as _,
            height: GRAPH_HEIGHT as _,
            data_pressure: &chip.data_pressure,
            data_flow: &chip.data_flow,
            chip_state: &chip.state,
            machine_snapshot: &chip.last_machine_snapshot,
            plot_graphs: &mut self.plot_graphs,
        };

        let screen_data_settings = DisplayDataSettings {
            images: &self.images,
        };

        // Render screen data (depending on state, running or stopped)
        match chip.state {
            ChipState::Running => screen.render_running(
                screen_data_layout,
                screen_data_branding,
                screen_data_status,
                screen_data_heartbeat,
                screen_data_graph,
                screen_data_settings,
                &chip.settings,
                &ScreenModalsOpen::from_states(&self.states),
            ),

            ChipState::Stopped => screen.render_stop(
                screen_data_layout,
                screen_data_branding,
                screen_data_status,
                screen_data_heartbeat,
                screen_data_graph,
                screen_data_settings,
                &chip.settings,
                &ScreenModalsOpen::from_states(&self.states),
            ),

            _ => unreachable!(),
        };
    }
}
