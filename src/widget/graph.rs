// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::display::widget::ControlWidget;

pub struct Config {
    pub width: f64,
    pub height: f64,

    pub image: conrod_core::image::Id,

    pub parent: WidgetId,
    pub id: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create image
    widget::Image::new(config.image)
        .w_h(config.width, config.height)
        .top_left_of(config.parent)
        .set(config.id, &mut master.ui);

    config.width
}
