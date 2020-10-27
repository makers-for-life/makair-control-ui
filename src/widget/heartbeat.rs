// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::cmp::{max, min};

use conrod_core::{
    color::{self, Color},
    widget::{self, Id as WidgetId},
    Positionable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::physics::pressure::process_max_allowed_pressure;
use crate::physics::types::DataPressure;

pub struct HeartbeatWidgetConfig<'a> {
    data_pressure: &'a DataPressure,
    peak_command: u8,
    container: WidgetId,
    ground: WidgetId,
    surround: WidgetId,
    inner: WidgetId,
}

impl<'a> HeartbeatWidgetConfig<'a> {
    pub fn new(
        data_pressure: &'a DataPressure,
        peak_command: u8,
        container: WidgetId,
        ground: WidgetId,
        surround: WidgetId,
        inner: WidgetId,
    ) -> HeartbeatWidgetConfig<'a> {
        HeartbeatWidgetConfig {
            data_pressure,
            peak_command,
            container,
            ground,
            surround,
            inner,
        }
    }
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: HeartbeatWidgetConfig) -> f64 {
    // Convert diameters to radius
    let (ground_radius, surround_radius) = (
        HEARTBEAT_GROUND_DIAMETER / 2.0,
        HEARTBEAT_SURROUND_DIAMETER / 2.0,
    );

    // #1: Create surround circle
    let surround_line_style = widget::primitive::line::Style::solid()
        .color(Color::Rgba(
            153.0 / 255.0,
            153.0 / 255.0,
            153.0 / 255.0,
            1.0,
        ))
        .thickness(HEARTBEAT_SURROUND_THICKNESS);

    widget::primitive::shape::circle::Circle::outline_styled(surround_radius, surround_line_style)
        .top_right_with_margins_on(
            config.container,
            HEARTBEAT_SURROUND_MARGIN_TOP,
            HEARTBEAT_SURROUND_MARGIN_RIGHT,
        )
        .set(config.surround, &mut master.ui);

    // #2: Create inner circle
    let last_pressure = if let Some(last_pressure_inner) = config.data_pressure.get(0) {
        // Convert high-precision point in mmH20 back to cmH20 (which measurements & targets \
        //   both use)
        last_pressure_inner.1 / TELEMETRY_POINTS_PRECISION_DIVIDE
    } else {
        0
    };

    let pressure_alert_threshold = process_max_allowed_pressure(config.peak_command) as f64;

    let last_pressure_ratio = last_pressure as f64 / pressure_alert_threshold;
    let last_pressure_radius = surround_radius * last_pressure_ratio;

    let inner_radius = min(
        max(last_pressure_radius as u16, ground_radius as u16 + 1),
        surround_radius as u16 + HEARTBEAT_INNER_MAX_OVERFLOW,
    ) as f64;

    let inner_color = if last_pressure_radius >= surround_radius {
        Color::Rgba(184.0 / 255.0, 1.0 / 255.0, 24.0 / 255.0, 1.0)
    } else {
        color::WHITE
    };

    widget::primitive::shape::circle::Circle::fill_with(inner_radius, inner_color)
        .middle_of(config.surround)
        .set(config.inner, &mut master.ui);

    // #3: Create ground circle
    let ground_color = if last_pressure_radius >= surround_radius {
        Color::Rgba(204.0 / 255.0, 204.0 / 255.0, 204.0 / 255.0, 1.0)
    } else {
        Color::Rgba(116.0 / 255.0, 116.0 / 255.0, 116.0 / 255.0, 1.0)
    };

    widget::primitive::shape::circle::Circle::fill_with(ground_radius, ground_color)
        .middle_of(config.surround)
        .set(config.ground, &mut master.ui);

    HEARTBEAT_GROUND_DIAMETER
}
