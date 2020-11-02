// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Widget,
};

use crate::display::widget::ControlWidget;

pub struct Config {
    width: f64,
    height: f64,

    parent: WidgetId,
    id: WidgetId,
}

impl Config {
    pub fn new(width: f64, height: f64, parent: WidgetId, id: WidgetId) -> Config {
        Config {
            width,
            height,
            parent,
            id,
        }
    }
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create rectangle for container
    widget::rectangle::Rectangle::fill_with([config.width, config.height], color::TRANSPARENT)
        .right_from(config.parent, 0.0)
        .set(config.id, &mut master.ui);

    0.0
}
