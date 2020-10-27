// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Colorable, Positionable, Sizeable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;

pub struct BrandingWidgetConfig<'a> {
    parent: WidgetId,
    version_firmware: &'a str,
    version_control: &'a str,
    width: f64,
    height: f64,
    image: conrod_core::image::Id,
    ids: (WidgetId, WidgetId, WidgetId),
}

impl<'a> BrandingWidgetConfig<'a> {
    pub fn new(
        parent: WidgetId,
        version_firmware: &'a str,
        version_control: &'a str,
        width: f64,
        height: f64,
        image: conrod_core::image::Id,
        ids: (WidgetId, WidgetId, WidgetId),
    ) -> BrandingWidgetConfig<'a> {
        BrandingWidgetConfig {
            parent,
            version_firmware,
            version_control,
            width,
            height,
            image,
            ids,
        }
    }
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: BrandingWidgetConfig) -> f64 {
    widget::rectangle::Rectangle::fill_with([config.width, config.height], color::TRANSPARENT)
        .top_left_with_margins_on(
            config.parent,
            BRANDING_IMAGE_MARGIN_TOP,
            BRANDING_IMAGE_MARGIN_LEFT,
        )
        .set(config.ids.0, &mut master.ui);

    // Display branding image
    widget::Image::new(config.image)
        .w_h(config.width, config.height)
        .top_left_of(config.ids.0)
        .set(config.ids.1, &mut master.ui);

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
        .set(config.ids.2, &mut master.ui);

    config.width
}
