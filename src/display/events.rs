// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{widget::Id as WidgetId, Ui};
use glium::glutin::{Event, EventsLoop, KeyboardInput, WindowEvent};

use crate::chip::settings::{
    cycles::SettingsCyclesEvent, expiration_term::SettingsExpirationTermEvent,
    mode::SettingsModeEvent, pressure::SettingsPressureEvent, run::SettingsRunEvent,
    snooze::SettingsSnoozeEvent, trigger::SettingsTriggerEvent, ChipSettingsEvent,
    SettingActionRange,
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
                "mode", states.mode_settings, [
                    ids.mode_overview_container,
                    ids.mode_overview_separator,
                    ids.mode_overview_text_class,
                    ids.mode_overview_text_type,
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

                    // Auto-close the modal upon pressing the snooze alarms toggle button, as this \
                    //   results in the user spending less time tapping on the UI as to proceed \
                    //   quick actions.
                    Some(DisplayRendererSettingsState::Closed)
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
                "mode", Mode, states.mode_settings,

                {
                    SettingsModeEvent::ModePcCmv, "mode pc cmv",

                    [
                        ids.mode_settings_selector_tab_pc_cmv,
                        ids.mode_settings_selector_texts_pc_cmv,
                    ],

                    None
                },

                {
                    SettingsModeEvent::ModePcAc, "mode pc ac",

                    [
                        ids.mode_settings_selector_tab_pc_ac,
                        ids.mode_settings_selector_texts_pc_ac,
                    ],

                    None
                },

                {
                    SettingsModeEvent::ModePcBipap, "mode pc bipap",

                    [
                        ids.mode_settings_selector_tab_pc_bipap,
                        ids.mode_settings_selector_texts_pc_bipap,
                    ],

                    None
                },

                {
                    SettingsModeEvent::ModeVcCmv, "mode vc cmv",

                    [
                        ids.mode_settings_selector_tab_vc_cmv,
                        ids.mode_settings_selector_texts_vc_cmv,
                    ],

                    None
                },

                {
                    SettingsModeEvent::InspiratoryTime(SettingActionRange::Less),
                    "inspiratory time less",

                    [
                        ids.mode_settings_field_time_inspiratory_less,
                        ids.mode_settings_field_time_inspiratory_less_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::InspiratoryTime(SettingActionRange::More),
                    "inspiratory time more",

                    [
                        ids.mode_settings_field_time_inspiratory_more,
                        ids.mode_settings_field_time_inspiratory_more_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::InspiratoryTimeMinimum(SettingActionRange::Less),
                    "inspiratory time minimum less",

                    [
                        ids.mode_settings_field_time_inspiratory_minimum_less,
                        ids.mode_settings_field_time_inspiratory_minimum_less_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::InspiratoryTimeMinimum(SettingActionRange::More),
                    "inspiratory time minimum more",

                    [
                        ids.mode_settings_field_time_inspiratory_minimum_more,
                        ids.mode_settings_field_time_inspiratory_minimum_more_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::InspiratoryTimeMaximum(SettingActionRange::Less),
                    "inspiratory time maximum less",

                    [
                        ids.mode_settings_field_time_inspiratory_maximum_less,
                        ids.mode_settings_field_time_inspiratory_maximum_less_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::InspiratoryTimeMaximum(SettingActionRange::More),
                    "inspiratory time maximum more",

                    [
                        ids.mode_settings_field_time_inspiratory_maximum_more,
                        ids.mode_settings_field_time_inspiratory_maximum_more_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::CyclesPerMinute(SettingActionRange::Less),
                    "cycles per minute less",

                    [
                        ids.mode_settings_field_cycles_per_minute_less,
                        ids.mode_settings_field_cycles_per_minute_less_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::CyclesPerMinute(SettingActionRange::More),
                    "cycles per minute more",

                    [
                        ids.mode_settings_field_cycles_per_minute_more,
                        ids.mode_settings_field_cycles_per_minute_more_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::TriggerInspiratoryOffset(SettingActionRange::Less),
                    "trigger inspiratory offset less",

                    [
                        ids.mode_settings_field_trigger_offset_less,
                        ids.mode_settings_field_trigger_offset_less_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::TriggerInspiratoryOffset(SettingActionRange::More),
                    "trigger inspiratory offset more",

                    [
                        ids.mode_settings_field_trigger_offset_more,
                        ids.mode_settings_field_trigger_offset_more_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::TriggerInspiratoryFlow(SettingActionRange::Less),
                    "trigger inspiratory flow less",

                    [
                        ids.mode_settings_field_trigger_inspiratory_less,
                        ids.mode_settings_field_trigger_inspiratory_less_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::TriggerInspiratoryFlow(SettingActionRange::More),
                    "trigger inspiratory flow more",

                    [
                        ids.mode_settings_field_trigger_inspiratory_more,
                        ids.mode_settings_field_trigger_inspiratory_more_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::TriggerExpiratoryFlow(SettingActionRange::Less),
                    "trigger expiratory flow less",

                    [
                        ids.mode_settings_field_trigger_expiratory_less,
                        ids.mode_settings_field_trigger_expiratory_less_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::TriggerExpiratoryFlow(SettingActionRange::More),
                    "trigger expiratory flow more",

                    [
                        ids.mode_settings_field_trigger_expiratory_more,
                        ids.mode_settings_field_trigger_expiratory_more_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::PressurePlateau(SettingActionRange::Less),
                    "pressure plateau less",

                    [
                        ids.mode_settings_field_pressure_inspiratory_less,
                        ids.mode_settings_field_pressure_inspiratory_less_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::PressurePlateau(SettingActionRange::More),
                    "pressure plateau more",

                    [
                        ids.mode_settings_field_pressure_inspiratory_more,
                        ids.mode_settings_field_pressure_inspiratory_more_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::PressureExpiratory(SettingActionRange::Less), "pressure expiratory less",

                    [
                        ids.mode_settings_field_pressure_expiratory_less,
                        ids.mode_settings_field_pressure_expiratory_less_text,
                    ],

                    None
                },

                {
                    SettingsModeEvent::PressureExpiratory(SettingActionRange::More), "pressure expiratory more",

                    [
                        ids.mode_settings_field_pressure_expiratory_more,
                        ids.mode_settings_field_pressure_expiratory_more_text,
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
