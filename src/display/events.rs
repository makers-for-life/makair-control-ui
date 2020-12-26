// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{widget::Id as WidgetId, Ui};
use glium::glutin::{Event, EventsLoop, KeyboardInput, WindowEvent};

use crate::chip::{
    settings::{
        advanced::SettingsAdvancedGroupTab,
        mode::{SettingsModeEvent, SettingsModeGroupTab, SettingsModeIntent},
        run::SettingsRunEvent,
        snooze::SettingsSnoozeEvent,
        ChipSettingsEvent, ChipSettingsIntent, SettingActionRange,
    },
    Chip,
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
        chip: &mut Chip,
        states: &mut DisplayRendererStates,
    ) -> (bool, Vec<ChipSettingsIntent>, Vec<ChipSettingsEvent>) {
        let (mut has_events, mut intents, mut events) = (false, Vec::new(), Vec::new());

        // Handle telemetry clicks
        if Self::run_opener_clicks(interface, ids, states) {
            has_events = true;
        }

        // Handle modal settings clicks
        if Self::run_modal_settings_clicks(interface, ids, states, &mut intents, &mut events) {
            has_events = true;
        }

        // Handle modal local clicks
        if Self::run_modal_local_clicks(interface, ids, chip, states) {
            has_events = true;
        }

        (has_events, intents, events)
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
                "mode", states.mode_settings, [
                    // From: 'mode'
                    ids.mode_overview_container,
                    ids.mode_overview_separator,
                    ids.mode_overview_text_class,
                    ids.mode_overview_text_type,

                    // From: 'expiratory term'
                    ids.ratio_parent,
                    ids.ratio_title,
                    ids.ratio_value_measured,
                    ids.ratio_unit,

                    // From: 'tidal volume'
                    ids.tidal_parent,
                    ids.tidal_title,
                    ids.tidal_value_measured,
                    ids.tidal_value_arrow_main,
                    ids.tidal_value_arrow_line,
                    ids.tidal_value_target,
                    ids.tidal_unit,

                    // From: 'minute volume'
                    ids.minute_volume_parent,
                    ids.minute_volume_title,
                    ids.minute_volume_value_measured,
                    ids.minute_volume_value_arrow_main,
                    ids.minute_volume_value_arrow_line,
                    ids.minute_volume_value_target,
                    ids.minute_volume_unit,

                    // From: 'cycles'
                    ids.cycles_parent,
                    ids.cycles_title,
                    ids.cycles_value_measured,
                    ids.cycles_value_arrow_main,
                    ids.cycles_value_arrow_line,
                    ids.cycles_value_target,
                    ids.cycles_unit,

                    // From: 'pressure'
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
        intents: &mut Vec<ChipSettingsIntent>,
        events: &mut Vec<ChipSettingsEvent>,
    ) -> bool {
        let mut has_events = false;

        // Generate all event handlers for modal settings clicks
        gen_ui_events_modal_settings_clicks!(
            interface, ids, has_events,

            {
                "run", states.run_settings, {}, {},

                [
                    "toggle",

                    [
                        ids.run_status_button,
                        ids.run_status_button_text,
                    ],

                    {
                        events.push(ChipSettingsEvent::Run(
                            SettingsRunEvent::RespirationEnabled
                        ));

                        // Auto-close the modal upon pressing the run state toggle button, as this \
                        //   is confusing to users otherwise, and is prone to user making mistakes \
                        //    by double tapping the button and thus cycling the respirator between \
                        //    states quickly, which is not intended and can be dangerous.
                        states.run_settings = DisplayRendererSettingsState::Closed;
                    }
                ]
            },

            {
                "snooze", states.snooze_settings, {}, {},

                [
                    "alarms",

                    [
                        ids.snooze_alarms_button,
                        ids.snooze_alarms_button_text,
                    ],

                    {
                        events.push(ChipSettingsEvent::Snooze(
                            SettingsSnoozeEvent::AlarmSnooze
                        ));

                        // Auto-close the modal upon pressing the snooze alarms toggle button, as \
                        //   this results in the user spending less time tapping on the UI as to \
                        //   proceed quick actions.
                        states.snooze_settings = DisplayRendererSettingsState::Closed;
                    }
                ]
            },

            {
                "advanced", states.advanced_settings, {}, {},
            },
        );

        gen_ui_events_modal_settings_intents_clicks!(
            interface, ids, intents, has_events,

            {
                "mode", Mode, states.mode_settings,

                {
                    // Clear all local draft changes, as the user pressed on the cancel button \
                    //   to cancel all pending changes and close the modal.
                    intents.push(
                        ChipSettingsIntent::Mode(SettingsModeIntent::ClearDraft)
                    );
                },

                {
                    // Commit all draft changes (push all draft changes to live settings, which \
                    //   will apply them immediately in the firmware runtime).
                    events.push(
                        ChipSettingsEvent::Mode(SettingsModeEvent::Commit)
                    );
                },

                [
                    "mode pc cmv",
                    SettingsModeIntent::ModePcCmv,

                    [
                        ids.mode_settings_selector_tab_pc_cmv,
                        ids.mode_settings_selector_texts_pc_cmv,
                    ]
                ],

                [
                    "mode pc ac",
                    SettingsModeIntent::ModePcAc,

                    [
                        ids.mode_settings_selector_tab_pc_ac,
                        ids.mode_settings_selector_texts_pc_ac,
                    ]
                ],

                [
                    "mode pc vsai",
                    SettingsModeIntent::ModePcVsai,

                    [
                        ids.mode_settings_selector_tab_pc_vsai,
                        ids.mode_settings_selector_texts_pc_vsai,
                    ]
                ],

                [
                    "mode vc cmv",
                    SettingsModeIntent::ModeVcCmv,

                    [
                        ids.mode_settings_selector_tab_vc_cmv,
                        ids.mode_settings_selector_texts_vc_cmv,
                    ]
                ],

                [
                    "mode vc ac",
                    SettingsModeIntent::ModeVcAc,

                    [
                        ids.mode_settings_selector_tab_vc_ac,
                        ids.mode_settings_selector_texts_vc_ac,
                    ]
                ],

                [
                    "inspiratory time minimum less",
                    SettingsModeIntent::InspiratoryTimeMinimum(SettingActionRange::Less),

                    [
                        ids.mode_settings_field_time_inspiratory_minimum_less,
                        ids.mode_settings_field_time_inspiratory_minimum_less_text,
                    ]
                ],

                [
                    "inspiratory time minimum more",
                    SettingsModeIntent::InspiratoryTimeMinimum(SettingActionRange::More),

                    [
                        ids.mode_settings_field_time_inspiratory_minimum_more,
                        ids.mode_settings_field_time_inspiratory_minimum_more_text,
                    ]
                ],

                [
                    "inspiratory time maximum less",
                    SettingsModeIntent::InspiratoryTimeMaximum(SettingActionRange::Less),

                    [
                        ids.mode_settings_field_time_inspiratory_maximum_less,
                        ids.mode_settings_field_time_inspiratory_maximum_less_text,
                    ]
                ],

                [
                    "inspiratory time maximum more",
                    SettingsModeIntent::InspiratoryTimeMaximum(SettingActionRange::More),

                    [
                        ids.mode_settings_field_time_inspiratory_maximum_more,
                        ids.mode_settings_field_time_inspiratory_maximum_more_text,
                    ]
                ],

                [
                    "cycles per minute less",
                    SettingsModeIntent::CyclesPerMinute(SettingActionRange::Less),

                    [
                        ids.mode_settings_field_cycles_per_minute_less,
                        ids.mode_settings_field_cycles_per_minute_less_text,
                    ]
                ],

                [
                    "cycles per minute more",
                    SettingsModeIntent::CyclesPerMinute(SettingActionRange::More),

                    [
                        ids.mode_settings_field_cycles_per_minute_more,
                        ids.mode_settings_field_cycles_per_minute_more_text,
                    ]
                ],

                [
                    "volume tidal less",
                    SettingsModeIntent::VolumeTidal(SettingActionRange::Less),

                    [
                        ids.mode_settings_field_tidal_volume_less,
                        ids.mode_settings_field_tidal_volume_less_text,
                    ]
                ],

                [
                    "volume tidal more",
                    SettingsModeIntent::VolumeTidal(SettingActionRange::More),

                    [
                        ids.mode_settings_field_tidal_volume_more,
                        ids.mode_settings_field_tidal_volume_more_text,
                    ]
                ],

                [
                    "flow inspiration less",
                    SettingsModeIntent::FlowInspiration(SettingActionRange::Less),

                    [
                        ids.mode_settings_field_inspiratory_flow_less,
                        ids.mode_settings_field_inspiratory_flow_less_text,
                    ]
                ],

                [
                    "flow inspiration more",
                    SettingsModeIntent::FlowInspiration(SettingActionRange::More),

                    [
                        ids.mode_settings_field_inspiratory_flow_more,
                        ids.mode_settings_field_inspiratory_flow_more_text,
                    ]
                ],

                [
                    "duration inspiration less",
                    SettingsModeIntent::DurationInspiration(SettingActionRange::Less),

                    [
                        ids.mode_settings_field_inspiratory_duration_less,
                        ids.mode_settings_field_inspiratory_duration_less_text,
                    ]
                ],

                [
                    "duration inspiration more",
                    SettingsModeIntent::DurationInspiration(SettingActionRange::More),

                    [
                        ids.mode_settings_field_inspiratory_duration_more,
                        ids.mode_settings_field_inspiratory_duration_more_text,
                    ]
                ],

                [
                    "duration plateau less",
                    SettingsModeIntent::DurationPlateau(SettingActionRange::Less),

                    [
                        ids.mode_settings_field_plateau_duration_less,
                        ids.mode_settings_field_plateau_duration_less_text,
                    ]
                ],

                [
                    "duration plateau more",
                    SettingsModeIntent::DurationPlateau(SettingActionRange::More),

                    [
                        ids.mode_settings_field_plateau_duration_more,
                        ids.mode_settings_field_plateau_duration_more_text,
                    ]
                ],

                [
                    "trigger inspiratory offset less",
                    SettingsModeIntent::TriggerInspiratoryOffset(SettingActionRange::Less),

                    [
                        ids.mode_settings_field_trigger_offset_less,
                        ids.mode_settings_field_trigger_offset_less_text,
                    ]
                ],

                [
                    "trigger inspiratory offset more",
                    SettingsModeIntent::TriggerInspiratoryOffset(SettingActionRange::More),

                    [
                        ids.mode_settings_field_trigger_offset_more,
                        ids.mode_settings_field_trigger_offset_more_text,
                    ]
                ],

                [
                    "trigger expiratory flow less",
                    SettingsModeIntent::TriggerExpiratoryFlow(SettingActionRange::Less),

                    [
                        ids.mode_settings_field_trigger_expiratory_less,
                        ids.mode_settings_field_trigger_expiratory_less_text,
                    ]
                ],

                [
                    "trigger expiratory flow more",
                    SettingsModeIntent::TriggerExpiratoryFlow(SettingActionRange::More),

                    [
                        ids.mode_settings_field_trigger_expiratory_more,
                        ids.mode_settings_field_trigger_expiratory_more_text,
                    ]
                ],

                [
                    "pressure plateau less",
                    SettingsModeIntent::PressurePlateau(SettingActionRange::Less),

                    [
                        ids.mode_settings_field_pressure_inspiratory_less,
                        ids.mode_settings_field_pressure_inspiratory_less_text,
                    ]
                ],

                [
                    "pressure plateau more",
                    SettingsModeIntent::PressurePlateau(SettingActionRange::More),

                    [
                        ids.mode_settings_field_pressure_inspiratory_more,
                        ids.mode_settings_field_pressure_inspiratory_more_text,
                    ]
                ],

                [
                    "pressure expiratory less",
                    SettingsModeIntent::PressureExpiratory(SettingActionRange::Less),

                    [
                        ids.mode_settings_field_pressure_expiratory_less,
                        ids.mode_settings_field_pressure_expiratory_less_text,
                    ]
                ],

                [
                    "pressure expiratory more",
                    SettingsModeIntent::PressureExpiratory(SettingActionRange::More),

                    [
                        ids.mode_settings_field_pressure_expiratory_more,
                        ids.mode_settings_field_pressure_expiratory_more_text,
                    ]
                ],

                [
                    "alarm low inspiratory minute volume more",
                    SettingsModeIntent::LowInspiratoryMinuteVolumeAlarm(SettingActionRange::More),

                    [
                        ids.mode_settings_alarm_threshold_low_inspiratory_minute_volume_more,
                        ids.mode_settings_alarm_threshold_low_inspiratory_minute_volume_more_text,
                    ]
                ],

                [
                    "alarm low inspiratory minute volume less",
                    SettingsModeIntent::LowInspiratoryMinuteVolumeAlarm(SettingActionRange::Less),

                    [
                        ids.mode_settings_alarm_threshold_low_inspiratory_minute_volume_less,
                        ids.mode_settings_alarm_threshold_low_inspiratory_minute_volume_less_text,
                    ]
                ],

                [
                    "alarm high inspiratory minute volume more",
                    SettingsModeIntent::HighInspiratoryMinuteVolumeAlarm(SettingActionRange::More),

                    [
                        ids.mode_settings_alarm_threshold_high_inspiratory_minute_volume_more,
                        ids.mode_settings_alarm_threshold_high_inspiratory_minute_volume_more_text,
                    ]
                ],

                [
                    "alarm high inspiratory minute volume less",
                    SettingsModeIntent::HighInspiratoryMinuteVolumeAlarm(SettingActionRange::Less),

                    [
                        ids.mode_settings_alarm_threshold_high_inspiratory_minute_volume_less,
                        ids.mode_settings_alarm_threshold_high_inspiratory_minute_volume_less_text,
                    ]
                ],

                [
                    "alarm low expiratory minute volume more",
                    SettingsModeIntent::LowExpiratoryMinuteVolumeAlarm(SettingActionRange::More),

                    [
                        ids.mode_settings_alarm_threshold_low_expiratory_minute_volume_more,
                        ids.mode_settings_alarm_threshold_low_expiratory_minute_volume_more_text,
                    ]
                ],

                [
                    "alarm low expiratory minute volume less",
                    SettingsModeIntent::LowExpiratoryMinuteVolumeAlarm(SettingActionRange::Less),

                    [
                        ids.mode_settings_alarm_threshold_low_expiratory_minute_volume_less,
                        ids.mode_settings_alarm_threshold_low_expiratory_minute_volume_less_text,
                    ]
                ],

                [
                    "alarm high expiratory minute volume more",
                    SettingsModeIntent::HighExpiratoryMinuteVolumeAlarm(SettingActionRange::More),

                    [
                        ids.mode_settings_alarm_threshold_high_expiratory_minute_volume_more,
                        ids.mode_settings_alarm_threshold_high_expiratory_minute_volume_more_text,
                    ]
                ],

                [
                    "alarm high expiratory minute volume less",
                    SettingsModeIntent::HighExpiratoryMinuteVolumeAlarm(SettingActionRange::Less),

                    [
                        ids.mode_settings_alarm_threshold_high_expiratory_minute_volume_less,
                        ids.mode_settings_alarm_threshold_high_expiratory_minute_volume_less_text,
                    ]
                ],

                [
                    "alarm low expiratory rate more",
                    SettingsModeIntent::LowRespiratoryRateAlarm(SettingActionRange::More),

                    [
                        ids.mode_settings_alarm_threshold_low_respiratory_rate_more,
                        ids.mode_settings_alarm_threshold_low_respiratory_rate_more_text,
                    ]
                ],

                [
                    "alarm low expiratory rate less",
                    SettingsModeIntent::LowRespiratoryRateAlarm(SettingActionRange::Less),

                    [
                        ids.mode_settings_alarm_threshold_low_respiratory_rate_less,
                        ids.mode_settings_alarm_threshold_low_respiratory_rate_less_text,
                    ]
                ],

                [
                    "alarm high expiratory rate more",
                    SettingsModeIntent::HighRespiratoryRateAlarm(SettingActionRange::More),

                    [
                        ids.mode_settings_alarm_threshold_high_respiratory_rate_more,
                        ids.mode_settings_alarm_threshold_high_respiratory_rate_more_text,
                    ]
                ],

                [
                    "alarm high expiratory rate less",
                    SettingsModeIntent::HighRespiratoryRateAlarm(SettingActionRange::Less),

                    [
                        ids.mode_settings_alarm_threshold_high_respiratory_rate_less,
                        ids.mode_settings_alarm_threshold_high_respiratory_rate_less_text,
                    ]
                ],

                [
                    "alarm low tidal volume more",
                    SettingsModeIntent::LowTidalVolumeAlarm(SettingActionRange::More),

                    [
                        ids.mode_settings_alarm_threshold_low_tidal_volume_more,
                        ids.mode_settings_alarm_threshold_low_tidal_volume_more_text,
                    ]
                ],

                [
                    "alarm low tidal volume less",
                    SettingsModeIntent::LowTidalVolumeAlarm(SettingActionRange::Less),

                    [
                        ids.mode_settings_alarm_threshold_low_tidal_volume_less,
                        ids.mode_settings_alarm_threshold_low_tidal_volume_less_text,
                    ]
                ],

                [
                    "alarm high tidal volume more",
                    SettingsModeIntent::HighTidalVolumeAlarm(SettingActionRange::More),

                    [
                        ids.mode_settings_alarm_threshold_high_tidal_volume_more,
                        ids.mode_settings_alarm_threshold_high_tidal_volume_more_text,
                    ]
                ],

                [
                    "alarm high tidal volume less",
                    SettingsModeIntent::HighTidalVolumeAlarm(SettingActionRange::Less),

                    [
                        ids.mode_settings_alarm_threshold_high_tidal_volume_less,
                        ids.mode_settings_alarm_threshold_high_tidal_volume_less_text,
                    ]
                ],

                [
                    "alarm leak more",
                    SettingsModeIntent::LeakAlarm(SettingActionRange::More),

                    [
                        ids.mode_settings_alarm_threshold_leak_more,
                        ids.mode_settings_alarm_threshold_leak_more_text,
                    ]
                ],

                [
                    "alarm leak less",
                    SettingsModeIntent::LeakAlarm(SettingActionRange::Less),

                    [
                        ids.mode_settings_alarm_threshold_leak_less,
                        ids.mode_settings_alarm_threshold_leak_less_text,
                    ]
                ]
            },
        );

        has_events
    }

    fn run_modal_local_clicks(
        interface: &mut Ui,
        ids: &Ids,
        chip: &mut Chip,
        states: &mut DisplayRendererStates,
    ) -> bool {
        let mut has_events = false;

        // Generate all event handlers for local modal clicks (ie. clicks that should not result \
        //   in a telemetry event being sent)
        gen_ui_events_modal_local_clicks!(
            interface, ids, has_events,

            {
                "mode", states.mode_settings,

                {
                    "group general",

                    [
                        ids.mode_settings_group_tab_general_button,
                        ids.mode_settings_group_tab_general_text,
                    ],

                    {
                        chip.settings.mode.group = SettingsModeGroupTab::General;
                    }
                },

                {
                    "group alarms",

                    [
                        ids.mode_settings_group_tab_alarms_button,
                        ids.mode_settings_group_tab_alarms_text,
                    ],

                    {
                        chip.settings.mode.group = SettingsModeGroupTab::Alarms;
                    }
                }
            },

            {
                "advanced", states.advanced_settings,

                {
                    "group statistics",

                    [
                        ids.advanced_group_tab_statistics_button,
                        ids.advanced_group_tab_statistics_text,
                    ],

                    {
                        chip.settings.advanced.group = SettingsAdvancedGroupTab::Statistics;
                    }
                },

                {
                    "group settings",

                    [
                        ids.advanced_group_tab_settings_button,
                        ids.advanced_group_tab_settings_text,
                    ],

                    {
                        chip.settings.advanced.group = SettingsAdvancedGroupTab::Settings;
                    }
                },

                {
                    "settings locale previous",

                    [
                        ids.advanced_field_locale_less,
                        ids.advanced_field_locale_less_text,
                    ],

                    {
                        chip.settings.advanced.switch_locale(SettingActionRange::Less);
                    }
                },

                {
                    "settings locale next",

                    [
                        ids.advanced_field_locale_more,
                        ids.advanced_field_locale_more_text,
                    ],

                    {
                        chip.settings.advanced.switch_locale(SettingActionRange::More);
                    }
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
