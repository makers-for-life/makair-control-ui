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
                "preset", states.preset_settings, {}, {},
            },

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
                "mode", Mode, mode_settings, states.mode_settings, SettingsModeIntent,

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

                {
                    [
                        "mode pc cmv",
                        ModePcCmv,
                        pc_cmv
                    ],

                    [
                        "mode pc ac",
                        ModePcAc,
                        pc_ac
                    ],

                    [
                        "mode pc vsai",
                        ModePcVsai,
                        pc_vsai
                    ],

                    [
                        "mode vc cmv",
                        ModeVcCmv,
                        vc_cmv
                    ],

                    [
                        "mode vc ac",
                        ModeVcAc,
                        vc_ac
                    ]
                },

                {
                    [
                        "inspiratory time minimum",
                        TiMin,
                        field_time_inspiratory_minimum
                    ],

                    [
                        "inspiratory time maximum",
                        TiMax,
                        field_time_inspiratory_maximum
                    ],

                    [
                        "cycles per minute",
                        CyclesPerMinute,
                        field_cycles_per_minute
                    ],

                    [
                        "volume tidal",
                        TargetTidalVolume,
                        field_tidal_volume
                    ],

                    [
                        "flow inspiration",
                        TargetInspiratoryFlow,
                        field_inspiratory_flow
                    ],

                    [
                        "duration inspiration",
                        InspiratoryDuration,
                        field_inspiratory_duration
                    ],

                    [
                        "duration plateau",
                        PlateauDuration,
                        field_plateau_duration
                    ],

                    [
                        "trigger inspiratory offset",
                        TriggerOffset,
                        field_trigger_offset
                    ],

                    [
                        "trigger expiratory flow",
                        ExpiratoryTriggerFlow,
                        field_trigger_expiratory
                    ],

                    [
                        "pressure plateau",
                        PlateauPressure,
                        field_pressure_inspiratory
                    ],

                    [
                        "pressure expiratory",
                        PEEP,
                        field_pressure_expiratory
                    ],

                    [
                        "alarm low inspiratory minute volume",
                        LowInspiratoryMinuteVolumeAlarmThreshold,
                        alarm_threshold_low_inspiratory_minute_volume
                    ],

                    [
                        "alarm high inspiratory minute volume",
                        HighInspiratoryMinuteVolumeAlarmThreshold,
                        alarm_threshold_high_inspiratory_minute_volume
                    ],

                    [
                        "alarm low expiratory minute volume",
                        LowExpiratoryMinuteVolumeAlarmThreshold,
                        alarm_threshold_low_expiratory_minute_volume
                    ],

                    [
                        "alarm high expiratory minute volume",
                        HighExpiratoryMinuteVolumeAlarmThreshold,
                        alarm_threshold_high_expiratory_minute_volume
                    ],

                    [
                        "alarm low expiratory rate",
                        LowRespiratoryRateAlarmThreshold,
                        alarm_threshold_low_respiratory_rate
                    ],

                    [
                        "alarm high expiratory rate",
                        HighRespiratoryRateAlarmThreshold,
                        alarm_threshold_high_respiratory_rate
                    ],

                    [
                        "alarm low tidal volume",
                        LowTidalVolumeAlarmThreshold,
                        alarm_threshold_low_tidal_volume
                    ],

                    [
                        "alarm high tidal volume",
                        HighTidalVolumeAlarmThreshold,
                        alarm_threshold_high_tidal_volume
                    ],

                    [
                        "alarm leak",
                        LeakAlarmThreshold,
                        alarm_threshold_leak
                    ]
                }
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
                "preset", states.preset_settings,

                {
                    "preset age previous",

                    [
                        ids.preset_settings_field_age_less,
                        ids.preset_settings_field_age_less_text,
                    ],

                    {
                        chip.settings.preset.switch_age(SettingActionRange::Less);
                    }
                },

                {
                    "preset age next",

                    [
                        ids.preset_settings_field_age_more,
                        ids.preset_settings_field_age_more_text,
                    ],

                    {
                        chip.settings.preset.switch_age(SettingActionRange::More);
                    }
                },

                {
                    "preset height previous",

                    [
                        ids.preset_settings_field_height_less,
                        ids.preset_settings_field_height_less_text,
                    ],

                    {
                        chip.settings.preset.change_height(SettingActionRange::Less);
                    }
                },

                {
                    "preset height next",

                    [
                        ids.preset_settings_field_height_more,
                        ids.preset_settings_field_height_more_text,
                    ],

                    {
                        chip.settings.preset.change_height(SettingActionRange::More);
                    }
                }
            },

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
