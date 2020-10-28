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
    width: f64,
    height: f64,
    image: conrod_core::image::Id,
    parent: WidgetId,
    id: WidgetId,
}

impl Config {
    pub fn new(
        width: f64,
        height: f64,
        image: conrod_core::image::Id,
        parent: WidgetId,
        id: WidgetId,
    ) -> Config {
        Config {
            width,
            height,
            image,
            parent,
            id,
        }
    }
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create image
    widget::Image::new(config.image)
        .w_h(config.width, config.height)
        .top_left_of(config.parent)
        .set(config.id, &mut master.ui);

    config.width
}
