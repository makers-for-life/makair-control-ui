// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{widget::Id as WidgetId, Ui};
use glium::glutin::{Event, EventsLoop, KeyboardInput, WindowEvent};

use crate::chip::settings::{
    cycles::SettingsCyclesEvent, expiration_term::SettingsExpirationTermEvent,
    pressure::SettingsPressureEvent, trigger::SettingsTriggerEvent, ChipSettingsEvent,
    SettingAction,
};

use super::identifiers::Ids;
use super::renderer::DisplayRendererSettingsState;
use super::support::{self, EventLoop, GliumDisplayWinitWrapper};

pub struct DisplayEventsBuilder;

pub struct DisplayEvents {
    event_loop: EventLoop,
}

pub struct DisplayUIEvents;

pub enum DisplayEventsHandleOutcome {
    Break,
    Continue,
}

#[allow(clippy::new_ret_no_self)]
impl DisplayEventsBuilder {
    pub fn new() -> DisplayEvents {
        DisplayEvents {
            event_loop: EventLoop::new(),
        }
    }
}

impl DisplayEvents {
    pub fn handle(
        &mut self,
        display: &GliumDisplayWinitWrapper,
        interface: &mut Ui,
        mut events_loop: &mut EventsLoop,
    ) -> DisplayEventsHandleOutcome {
        for event in self.event_loop.next(&mut events_loop) {
            // Use the `winit` backend feature to convert the winit event to a conrod one.
            if let Some(event) = support::convert_event(event.clone(), display) {
                interface.handle_event(event);
            }

            // Break from the loop upon `Escape` or closed window.
            if let Event::WindowEvent { event, .. } = event.clone() {
                match event {
                    WindowEvent::CloseRequested
                    | WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                        ..
                    } => {
                        return DisplayEventsHandleOutcome::Break;
                    }
                    _ => (),
                }
            }
        }

        DisplayEventsHandleOutcome::Continue
    }
}

impl DisplayUIEvents {
    pub fn run(
        interface: &mut Ui,
        ids: &Ids,
        trigger_settings_state: &mut DisplayRendererSettingsState,
        expiration_term_settings_state: &mut DisplayRendererSettingsState,
        pressure_settings_state: &mut DisplayRendererSettingsState,
        cycles_settings_state: &mut DisplayRendererSettingsState,
    ) -> (bool, Vec<ChipSettingsEvent>) {
        let (mut has_events, mut events) = (false, Vec::new());

        // Handle telemetry clicks
        if Self::run_telemetry_clicks(
            interface,
            ids,
            trigger_settings_state,
            expiration_term_settings_state,
            pressure_settings_state,
            cycles_settings_state,
        ) {
            has_events = true;
        }

        // Handle modal settings clicks
        if Self::run_modal_settings_clicks(
            interface,
            ids,
            trigger_settings_state,
            expiration_term_settings_state,
            pressure_settings_state,
            cycles_settings_state,
            &mut events,
        ) {
            has_events = true;
        }

        (has_events, events)
    }

    fn run_telemetry_clicks(
        interface: &mut Ui,
        ids: &Ids,
        trigger_settings_state: &mut DisplayRendererSettingsState,
        expiration_term_settings_state: &mut DisplayRendererSettingsState,
        pressure_settings_state: &mut DisplayRendererSettingsState,
        cycles_settings_state: &mut DisplayRendererSettingsState,
    ) -> bool {
        let mut has_events = false;

        // Generate all event handlers for telemetry clicks
        // Notice: if you click on a text, the text element will receive the click, not its \
        //   parent. Maybe there is a way to listen on a parent for childs clicks but we could not \
        //   find one. So we chain each iterator of every childs to be sure to capture the click.
        gen_ui_events_telemetry_settings_clicks!(
            interface, has_events,

            {
                "trigger", trigger_settings_state, [
                    ids.trigger_overview_container,
                    ids.trigger_overview_title,
                    ids.trigger_overview_status_label,
                    ids.trigger_overview_status_value,
                    ids.trigger_overview_offset_label,
                    ids.trigger_overview_offset_value,
                ]
            },

            {
                "expiratory term", expiration_term_settings_state, [
                    ids.ratio_parent,
                    ids.ratio_title,
                    ids.ratio_value_measured,
                    ids.ratio_unit,
                ]
            },

            {
                "cycles", cycles_settings_state, [
                    ids.cycles_parent,
                    ids.cycles_title,
                    ids.cycles_value_measured,
                    ids.cycles_value_arrow,
                    ids.cycles_value_target,
                    ids.cycles_unit,
                ]
            },

            {
                "pressure", pressure_settings_state, [
                    ids.peak_parent,
                    ids.peak_title,
                    ids.peak_value_measured,
                    ids.peak_value_arrow,
                    ids.peak_value_target,
                    ids.peak_unit,
                    ids.plateau_parent,
                    ids.plateau_title,
                    ids.plateau_value_measured,
                    ids.plateau_value_arrow,
                    ids.plateau_value_target,
                    ids.plateau_unit,
                    ids.peep_parent,
                    ids.peep_title,
                    ids.peep_value_measured,
                    ids.peep_value_arrow,
                    ids.peep_value_target,
                    ids.peep_unit,
                ]
            },
        );

        has_events
    }

    fn run_modal_settings_clicks(
        interface: &mut Ui,
        ids: &Ids,
        trigger_settings_state: &mut DisplayRendererSettingsState,
        expiration_term_settings_state: &mut DisplayRendererSettingsState,
        pressure_settings_state: &mut DisplayRendererSettingsState,
        cycles_settings_state: &mut DisplayRendererSettingsState,
        events: &mut Vec<ChipSettingsEvent>,
    ) -> bool {
        let mut has_events = false;

        // Generate all event handlers for modal settings clicks
        gen_ui_events_modal_settings_clicks!(
            interface, ids, has_events, events,

            {
                "trigger", Trigger, trigger_settings_state,

                {
                    SettingsTriggerEvent::TriggerToggle, "toggle", [
                        ids.trigger_status_button,
                        ids.trigger_status_button_text,
                    ]
                },

                {
                    SettingsTriggerEvent::TriggerOffset(SettingAction::Less), "offset less", [
                        ids.trigger_offset_less_button,
                        ids.trigger_offset_less_button_text,
                    ]
                },

                {
                    SettingsTriggerEvent::TriggerOffset(SettingAction::More), "offset more", [
                        ids.trigger_offset_more_button,
                        ids.trigger_offset_more_button_text,
                    ]
                }
            },

            {
                "expiratory term", ExpirationTerm, expiration_term_settings_state,

                {
                    SettingsExpirationTermEvent::ExpiratoryTerm(SettingAction::Less), "term less", [
                        ids.expiration_term_less_button,
                        ids.expiration_term_less_button_text,
                    ]
                },

                {
                    SettingsExpirationTermEvent::ExpiratoryTerm(SettingAction::More), "term more", [
                        ids.expiration_term_more_button,
                        ids.expiration_term_more_button_text,
                    ]
                }
            },

            {
                "cycles", Cycles, cycles_settings_state,

                {
                    SettingsCyclesEvent::CyclesPerMinute(SettingAction::Less), "cycles less", [
                        ids.cycles_less_button,
                        ids.cycles_less_button_text,
                    ]
                },

                {
                    SettingsCyclesEvent::CyclesPerMinute(SettingAction::More), "cycles more", [
                        ids.cycles_more_button,
                        ids.cycles_more_button_text,
                    ]
                }
            },

            {
                "pressure", Pressure, pressure_settings_state,

                {
                    SettingsPressureEvent::Peak(SettingAction::Less), "peak less", [
                        ids.pressure_peak_less_button,
                        ids.pressure_peak_less_button_text,
                    ]
                },

                {
                    SettingsPressureEvent::Peak(SettingAction::More), "peak more", [
                        ids.pressure_peak_more_button,
                        ids.pressure_peak_more_button_text,
                    ]
                },

                {
                    SettingsPressureEvent::Plateau(SettingAction::Less), "plateau less", [
                        ids.pressure_plateau_less_button,
                        ids.pressure_plateau_less_button_text,
                    ]
                },

                {
                    SettingsPressureEvent::Plateau(SettingAction::More), "plateau more", [
                        ids.pressure_plateau_more_button,
                        ids.pressure_plateau_more_button_text,
                    ]
                },

                {
                    SettingsPressureEvent::PEEP(SettingAction::Less), "peep less", [
                        ids.pressure_peep_less_button,
                        ids.pressure_peep_less_button_text,
                    ]
                },

                {
                    SettingsPressureEvent::PEEP(SettingAction::More), "peep more", [
                        ids.pressure_peep_more_button,
                        ids.pressure_peep_more_button_text,
                    ]
                }
            },
        );

        has_events
    }

    fn count_clicks(interface: &Ui, widget_ids: &[WidgetId]) -> usize {
        widget_ids
            .iter()
            .flat_map(|widget| {
                interface
                    .widget_input(*widget)
                    .clicks()
                    .map(|_| ())
                    .chain(interface.widget_input(*widget).taps().map(|_| ()))
            })
            .count()
    }
}
