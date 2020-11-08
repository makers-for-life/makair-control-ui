// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{widget::Id as WidgetId, Ui};
use glium::glutin::{Event, EventsLoop, KeyboardInput, WindowEvent};

use crate::chip::settings::{
    cycles::SettingsCyclesEvent, expiration_term::SettingsExpirationTermEvent,
    pressure::SettingsPressureEvent, run::SettingsRunEvent, snooze::SettingsSnoozeEvent,
    trigger::SettingsTriggerEvent, ChipSettingsEvent, SettingActionRange,
};

use super::identifiers::Ids;
use super::renderer::{DisplayRendererSettingsState, DisplayRendererStates};
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
        states: &mut DisplayRendererStates,
    ) -> (bool, Vec<ChipSettingsEvent>) {
        let (mut has_events, mut events) = (false, Vec::new());

        // Handle telemetry clicks
        if Self::run_opener_clicks(interface, ids, states) {
            has_events = true;
        }

        // Handle modal settings clicks
        if Self::run_modal_settings_clicks(interface, ids, states, &mut events) {
            has_events = true;
        }

        (has_events, events)
    }

    fn run_opener_clicks(
        interface: &mut Ui,
        ids: &Ids,
        states: &mut DisplayRendererStates,
    ) -> bool {
        let mut has_events = false;

        // Generate all event handlers for settings opener clicks
        // Notice: if you click on a text, the text element will receive the click, not its \
        //   parent. Maybe there is a way to listen on a parent for childs clicks but we could not \
        //   find one. So we chain each iterator of every childs to be sure to capture the click.
        gen_ui_events_opener_settings_clicks!(
            interface, has_events,

            {
                "run", states.run_settings, [
                    ids.controls_button_run,
                ]
            },

            {
                "snooze", states.snooze_settings, [
                    ids.controls_button_snooze,
                ]
            },

            {
                "advanced", states.advanced_settings, [
                    ids.controls_button_advanced,
                ]
            },

            {
                "trigger", states.trigger_settings, [
                    ids.trigger_overview_container,
                    ids.trigger_overview_title,
                    ids.trigger_overview_status_label,
                    ids.trigger_overview_status_value,
                    ids.trigger_overview_offset_label,
                    ids.trigger_overview_offset_value,
                ]
            },

            {
                "expiratory term", states.expiration_term_settings, [
                    ids.ratio_parent,
                    ids.ratio_title,
                    ids.ratio_value_measured,
                    ids.ratio_unit,
                ]
            },

            {
                "cycles", states.cycles_settings, [
                    ids.cycles_parent,
                    ids.cycles_title,
                    ids.cycles_value_measured,
                    ids.cycles_value_arrow_main,
                    ids.cycles_value_arrow_line,
                    ids.cycles_value_target,
                    ids.cycles_unit,
                ]
            },

            {
                "pressure", states.pressure_settings, [
                    ids.peak_parent,
                    ids.peak_title,
                    ids.peak_value_measured,
                    ids.peak_value_arrow_main,
                    ids.peak_value_arrow_line,
                    ids.peak_value_target,
                    ids.peak_unit,
                    ids.plateau_parent,
                    ids.plateau_title,
                    ids.plateau_value_measured,
                    ids.plateau_value_arrow_main,
                    ids.plateau_value_arrow_line,
                    ids.plateau_value_target,
                    ids.plateau_unit,
                    ids.peep_parent,
                    ids.peep_title,
                    ids.peep_value_measured,
                    ids.peep_value_arrow_main,
                    ids.peep_value_arrow_line,
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
        states: &mut DisplayRendererStates,
        events: &mut Vec<ChipSettingsEvent>,
    ) -> bool {
        let mut has_events = false;

        // Generate all event handlers for modal settings clicks
        gen_ui_events_modal_settings_clicks!(
            interface, ids, has_events, events,

            {
                "run", Run, states.run_settings,

                {
                    SettingsRunEvent::RespirationEnabled, "toggle",

                    [
                        ids.run_status_button,
                        ids.run_status_button_text,
                    ],

                    // Auto-close the modal upon pressing the run state toggle button, as this \
                    //   is confusing to users otherwise, and is prone to user making mistakes by \
                    //   double tapping the button and thus cycling the respirator between states \
                    //   quickly, which is not intended and can be dangerous.
                    Some(DisplayRendererSettingsState::Closed)
                }
            },

            {
                "snooze", Snooze, states.snooze_settings,

                {
                    SettingsSnoozeEvent::AlarmSnooze, "alarms",

                    [
                        ids.snooze_alarms_button,
                        ids.snooze_alarms_button_text,
                    ],

                    None
                }
            },

            {
                "advanced", Advanced, states.advanced_settings,
            },

            {
                "trigger", Trigger, states.trigger_settings,

                {
                    SettingsTriggerEvent::TriggerToggle, "toggle",

                    [
                        ids.trigger_status_button,
                        ids.trigger_status_button_text,
                    ],

                    None
                },

                {
                    SettingsTriggerEvent::TriggerOffset(SettingActionRange::Less), "offset less",

                    [
                        ids.trigger_offset_less_button,
                        ids.trigger_offset_less_button_text,
                    ],

                    None
                },

                {
                    SettingsTriggerEvent::TriggerOffset(SettingActionRange::More), "offset more",

                    [
                        ids.trigger_offset_more_button,
                        ids.trigger_offset_more_button_text,
                    ],

                    None
                }
            },

            {
                "expiratory term", ExpirationTerm, states.expiration_term_settings,

                {
                    SettingsExpirationTermEvent::ExpiratoryTerm(SettingActionRange::Less),
                    "term less",

                    [
                        ids.expiration_term_less_button,
                        ids.expiration_term_less_button_text,
                    ],

                    None
                },

                {
                    SettingsExpirationTermEvent::ExpiratoryTerm(SettingActionRange::More),
                    "term more",

                    [
                        ids.expiration_term_more_button,
                        ids.expiration_term_more_button_text,
                    ],

                    None
                }
            },

            {
                "cycles", Cycles, states.cycles_settings,

                {
                    SettingsCyclesEvent::CyclesPerMinute(SettingActionRange::Less), "cycles less",

                    [
                        ids.cycles_less_button,
                        ids.cycles_less_button_text,
                    ],

                    None
                },

                {
                    SettingsCyclesEvent::CyclesPerMinute(SettingActionRange::More), "cycles more",

                    [
                        ids.cycles_more_button,
                        ids.cycles_more_button_text,
                    ],

                    None
                }
            },

            {
                "pressure", Pressure, states.pressure_settings,

                {
                    SettingsPressureEvent::Plateau(SettingActionRange::Less), "plateau less",

                    [
                        ids.pressure_plateau_less_button,
                        ids.pressure_plateau_less_button_text,
                    ],

                    None
                },

                {
                    SettingsPressureEvent::Plateau(SettingActionRange::More), "plateau more",

                    [
                        ids.pressure_plateau_more_button,
                        ids.pressure_plateau_more_button_text,
                    ],

                    None
                },

                {
                    SettingsPressureEvent::PEEP(SettingActionRange::Less), "peep less",

                    [
                        ids.pressure_peep_less_button,
                        ids.pressure_peep_less_button_text,
                    ],

                    None
                },

                {
                    SettingsPressureEvent::PEEP(SettingActionRange::More), "peep more",

                    [
                        ids.pressure_peep_more_button,
                        ids.pressure_peep_more_button_text,
                    ],

                    None
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
