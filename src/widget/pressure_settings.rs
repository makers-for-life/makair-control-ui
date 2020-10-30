// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;

pub struct Config {
    pub width: f64,
    pub height: f64,
    pub pressure_container_parent: WidgetId,
    pub pressure_container_widget: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Initialize canvas style
    let mut canvas_style = widget::canvas::Style::default();

    canvas_style.color = Some(color::TRANSPARENT);
    canvas_style.border = Some(0.0);

    // Create canvas
    widget::Canvas::new()
        .with_style(canvas_style)
        .w_h(config.width, config.height)
        .top_left_of(config.pressure_container_parent)
        .set(config.pressure_container_widget, &mut master.ui);

    // Append contents
    // TODO

    0 as _
}
