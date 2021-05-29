// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use crate::widget::*;

use super::fonts::Fonts;

pub struct ControlWidget<'a> {
    pub ui: conrod_core::UiCell<'a>,
    pub fonts: &'a Fonts,
}

impl<'a> ControlWidget<'a> {
    pub fn new(ui: conrod_core::UiCell<'a>, fonts: &'a Fonts) -> ControlWidget<'a> {
        ControlWidget { ui, fonts }
    }
}

gen_widget_impls!(
    Alarms -> alarms['a],
    Background -> background,
    Error -> error,
    Branding -> branding['a],
    Controls -> controls,
    Status -> status['a],
    Heartbeat -> heartbeat['a],
    Initializing -> initializing,
    EndOfLine -> end_of_line['a],
    Graph -> graph['a],
    Modal -> modal,
    Stop -> stop,
    TelemetryContainer -> telemetry_container,
    TelemetryView -> telemetry_view,
    Layout -> layout,
    PresetSettings -> preset_settings['a],
    RunSettings -> run_settings['a],
    SnoozeSettings -> snooze_settings['a],
    AdvancedSettings -> advanced_settings['a],
    ModeSettings -> mode_settings['a],
    ModeOverview -> mode_overview['a],
);
