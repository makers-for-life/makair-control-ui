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
    id: WidgetId,

    width: f64,
    height: f64,

    image: conrod_core::image::Id,

    connecting: bool,
}

impl Config {
    pub fn new(
        id: WidgetId,
        width: f64,
        height: f64,
        image: conrod_core::image::Id,
        connecting: bool,
    ) -> Config {
        Config {
            id,
            width,
            height,
            image,
            connecting,
        }
    }
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create image
    widget::Image::new(config.image)
        .w_h(config.width, config.height)
        .middle()
        .set(config.id, &mut master.ui);

    // TODO: if connecting, show text
    if config.connecting {
        // TODO: show connecting
    } else {
        // TODO: show connected
    }

    0 as _
}
