// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Colorable, Positionable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;

pub struct Config<'a> {
    pub parent: WidgetId,

    pub version_firmware: &'a str,
    pub version_control: &'a str,

    pub width: f64,
    pub height: f64,

    pub ids: (WidgetId, WidgetId),
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create rectangle
    widget::rectangle::Rectangle::fill_with([config.width, config.height], color::TRANSPARENT)
        .top_left_with_margins_on(
            config.parent,
            BRANDING_IMAGE_MARGIN_TOP,
            BRANDING_IMAGE_MARGIN_LEFT,
        )
        .set(config.ids.0, &mut master.ui);

    // Display branding text
    let branding_text = format!("F{} | C{}", config.version_firmware, config.version_control);

    widget::Text::new(&branding_text)
        .color(color::WHITE.with_alpha(0.45))
        .top_left_with_margins_on(
            config.parent,
            BRANDING_TEXT_MARGIN_TOP,
            BRANDING_TEXT_MARGIN_LEFT,
        )
        .font_size(10)
        .set(config.ids.1, &mut master.ui);

    config.width
}
