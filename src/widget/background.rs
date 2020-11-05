// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    widget::{self, Id as WidgetId},
    Colorable, Widget,
};

use crate::display::widget::ControlWidget;

pub struct Config {
    pub color: conrod_core::color::Color,
    pub id: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create canvas
    widget::Canvas::new()
        .color(config.color)
        .set(config.id, &mut master.ui);

    0 as _
}
