// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::time::{Duration, Instant};

use conrod_core::Ui;

use crate::chip::settings::ChipSettingsEvent;
use crate::chip::{Chip, ChipError, ChipState};
use crate::config::environment::*;
use crate::utilities::parse::parse_version_number;

use super::data::*;
use super::events::DisplayUIEvents;
use super::fonts::Fonts;
use super::identifiers::{Ids, ImageIds};
use super::screen::{Screen, ScreenModalsOpen};

const WAITING_FOR_DATA_TIMEOUT_AFTER: Duration = Duration::from_secs(10);
const DISPATCH_HEARTBEAT_EVERY: Duration = Duration::from_secs(1);

#[derive(Debug, PartialEq, Eq)]
pub enum DisplayRendererSettingsState {
    Opened,
    Closed,
}

pub struct DisplayRendererBuilder;

pub struct DisplayRenderer {
    fonts: Fonts,
    ids: Ids,
    states: DisplayRendererStates,
    pub images: ImageIds,
}

#[derive(Default)]
pub struct DisplayRendererStates {
    pub run_settings: DisplayRendererSettingsState,
    pub snooze_settings: DisplayRendererSettingsState,
    pub advanced_settings: DisplayRendererSettingsState,
    pub trigger_settings: DisplayRendererSettingsState,
    pub expiration_term_settings: DisplayRendererSettingsState,
    pub pressure_settings: DisplayRendererSettingsState,
    pub cycles_settings: DisplayRendererSettingsState,
}

impl DisplayRendererSettingsState {
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Closed => Self::Opened,
            Self::Opened => Self::Closed,
        };
    }

    pub fn is_open(&self) -> bool {
        self == &Self::Opened
    }
}

impl Default for DisplayRendererSettingsState {
    fn default() -> Self {
        Self::Closed
    }
}

#[allow(clippy::new_ret_no_self)]
impl DisplayRendererBuilder {
    pub fn new(fonts: Fonts, ids: Ids, images: ImageIds) -> DisplayRenderer {
        DisplayRenderer {
            fonts,
            ids,
            images,
            states: DisplayRendererStates::default(),
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
        };
    }

    pub fn run_events(
        &mut self,
        interface: &mut Ui,
        last_heartbeat: &Instant,
        tick_time: &Instant,
    ) -> (bool, bool, Vec<ChipSettingsEvent>) {
        // Run all UI events (defer to sub-handler)
        let (has_user_events, user_events) =
            DisplayUIEvents::run(interface, &self.ids, &mut self.states);

        // Check if should run heartbeat? (ie. if it should be sent to the firmware)
        let mut has_heartbeat = false;

        if tick_time.duration_since(*last_heartbeat) >= DISPATCH_HEARTBEAT_EVERY {
            has_heartbeat = true;
        }

        (has_heartbeat, has_user_events, user_events)
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
            None,
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
            None,
            None,
            None,
            None,
        );

        screen.render_error(screen_error);
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
            Some(&ongoing_alarms),
            Some(chip.last_tick),
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
            battery_level: chip
                .last_data_snapshot
                .as_ref()
                .map(|data| data.battery_level),
        };
        let screen_data_heartbeat = DisplayDataHeartbeat {
            data_pressure: &chip.data_pressure,
        };

        let screen_data_graph = DisplayDataGraph {
            image_id: self.images.graph_pressure,
            width: GRAPH_WIDTH as _,
            height: GRAPH_HEIGHT as _,
        };

        // Render screen data (depending on state, running or stopped)
        match chip.state {
            ChipState::Running => screen.render_running(
                screen_data_layout,
                screen_data_branding,
                screen_data_status,
                screen_data_heartbeat,
                screen_data_graph,
                &chip.settings,
                &ScreenModalsOpen::from_states(&self.states),
            ),

            ChipState::Stopped => screen.render_stop(
                screen_data_layout,
                screen_data_branding,
                screen_data_status,
                screen_data_heartbeat,
                screen_data_graph,
                &chip.settings,
                &ScreenModalsOpen::from_states(&self.states),
            ),

            _ => unreachable!(),
        };
    }
}
