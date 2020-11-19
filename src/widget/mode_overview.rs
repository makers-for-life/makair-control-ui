// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::Color,
    widget::{self, Id as WidgetId},
    Positionable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

pub struct Config {
    pub background_color: Color,

    pub width: f64,
    pub height: f64,

    pub parent: WidgetId,
    pub container: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create container
    widget::rounded_rectangle::RoundedRectangle::fill_with(
        [config.width, config.height],
        TELEMETRY_WIDGET_UNIT_BORDER_RADIUS,
        config.background_color,
    )
    .top_left_of(config.parent)
    .set(config.container, &mut master.ui);

    // Append contents
    // TODO

    0 as _
}
