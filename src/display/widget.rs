// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use crate::widget::*;

use super::fonts::Fonts;

pub enum ControlWidgetType<'a> {
    Alarms(alarms::Config<'a>),
    Background(background::Config),
    Error(error::Config),
    Branding(branding::Config<'a>),
    Status(status::Config<'a>),
    Heartbeat(heartbeat::Config<'a>),
    Initializing(initializing::Config),
    Graph(graph::Config),
    Modal(modal::Config),
    NoData(no_data::Config),
    Stop(stop::Config),
    TelemetryContainer(telemetry_container::Config),
    TelemetryView(telemetry_view::Config),
    Layout(layout::Config),
    TriggerSettings(trigger_settings::Config<'a>),
    TriggerOverview(trigger_overview::Config<'a>),
    ExpirationTermSettings(expiration_term_settings::Config<'a>),
}

pub struct ControlWidget<'a> {
    pub ui: conrod_core::UiCell<'a>,
    pub fonts: &'a Fonts,
}

impl<'a> ControlWidget<'a> {
    pub fn new(ui: conrod_core::UiCell<'a>, fonts: &'a Fonts) -> ControlWidget<'a> {
        ControlWidget { ui, fonts }
    }

    pub fn render(&mut self, widget_type: ControlWidgetType<'a>) -> f64 {
        match widget_type {
            ControlWidgetType::Alarms(config) => alarms::render(self, config),
            ControlWidgetType::Background(config) => background::render(self, config),
            ControlWidgetType::Error(config) => error::render(self, config),
            ControlWidgetType::Branding(config) => branding::render(self, config),
            ControlWidgetType::Status(config) => status::render(self, config),
            ControlWidgetType::Heartbeat(config) => heartbeat::render(self, config),
            ControlWidgetType::Initializing(config) => initializing::render(self, config),
            ControlWidgetType::Graph(config) => graph::render(self, config),
            ControlWidgetType::Modal(config) => modal::render(self, config),
            ControlWidgetType::NoData(config) => no_data::render(self, config),
            ControlWidgetType::Stop(config) => stop::render(self, config),
            ControlWidgetType::TelemetryContainer(config) => {
                telemetry_container::render(self, config)
            }
            ControlWidgetType::TelemetryView(config) => telemetry_view::render(self, config),
            ControlWidgetType::Layout(config) => layout::render(self, config),
            ControlWidgetType::TriggerSettings(config) => trigger_settings::render(self, config),
            ControlWidgetType::TriggerOverview(config) => trigger_overview::render(self, config),
            ControlWidgetType::ExpirationTermSettings(config) => {
                expiration_term_settings::render(self, config)
            }
        }
    }
}
