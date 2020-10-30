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
    Status -> status['a],
    Heartbeat -> heartbeat['a],
    Initializing -> initializing,
    Graph -> graph,
    Modal -> modal,
    NoData -> no_data,
    Stop -> stop,
    TelemetryContainer -> telemetry_container,
    TelemetryView -> telemetry_view,
    Layout -> layout,
    TriggerSettings -> trigger_settings['a],
    TriggerOverview -> trigger_overview['a],
    ExpirationTermSettings -> expiration_term_settings['a],
    PressureSettings -> pressure_settings['a],
    CyclesSettings -> cycles_settings['a],
);
