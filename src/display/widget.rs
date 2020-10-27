// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use crate::widget::*;

use super::fonts::Fonts;

pub enum ControlWidgetType<'a> {
    Alarms(alarms::AlarmsWidgetConfig<'a>),
    Background(background::BackgroundWidgetConfig),
    Error(error::ErrorWidgetConfig),
    Branding(branding::BrandingWidgetConfig<'a>),
    Status(status::StatusWidgetConfig<'a>),
    Heartbeat(heartbeat::HeartbeatWidgetConfig<'a>),
    Initializing(initializing::InitializingWidgetConfig),
    Graph(graph::GraphWidgetConfig),
    Modal(modal::ModalWidgetConfig),
    NoData(no_data::NoDataWidgetConfig),
    Stop(stop::StopWidgetConfig),
    TelemetryContainer(telemetry_container::TelemetryWidgetContainerConfig),
    Telemetry(telemetry_view::TelemetryWidgetConfig),
    Layout(layout::LayoutConfig),
    TriggerSettings(trigger_settings::TriggerWidgetConfig<'a>),
    TriggerOverview(trigger_overview::TriggerOverview<'a>),
    ExpirationRatioSettings(expiration_ratio_settings::ExpirationRatioSettingsWidgetConfig<'a>),
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
            ControlWidgetType::Telemetry(config) => telemetry_view::render(self, config),
            ControlWidgetType::Layout(config) => layout::render(self, config),
            ControlWidgetType::TriggerSettings(config) => trigger_settings::render(self, config),
            ControlWidgetType::TriggerOverview(config) => trigger_overview::render(self, config),
            ControlWidgetType::ExpirationRatioSettings(config) => {
                expiration_ratio_settings::render(self, config)
            }
        }
    }
}
