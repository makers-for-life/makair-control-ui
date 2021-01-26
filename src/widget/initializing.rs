// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

const TEXT_COLOR: Color = Color::Rgba(1.0, 1.0, 1.0, 0.2);

pub struct Config {
    pub container: WidgetId,
    pub logo: WidgetId,
    pub text: WidgetId,

    pub width: f64,
    pub height: f64,

    pub image: conrod_core::image::Id,

    pub connecting: bool,
}

pub fn render(master: &mut ControlWidget, config: Config) -> f64 {
    // Generate initialization text
    let initialization_text = if config.connecting {
        APP_I18N.t("initializing-connecting")
    } else {
        APP_I18N.t("initializing-connected")
    };

    // Create container
    gen_widget_container!(
        master,
        container_id: config.container,
        color: color::TRANSPARENT,
        width: INITIALIZING_WIDTH,
        height: INITIALIZING_HEIGHT,
        positions: middle[]
    );

    // Create image
    widget::Image::new(config.image)
        .w_h(config.width, config.height)
        .mid_top_of(config.container)
        .set(config.logo, &mut master.ui);

    // Initialize text style
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.regular));
    text_style.color = Some(TEXT_COLOR);
    text_style.font_size = Some(INITIALIZING_MESSAGE_FONT_SIZE);

    // Create text
    widget::Text::new(&initialization_text)
        .mid_bottom_of(config.container)
        .with_style(text_style)
        .set(config.text, &mut master.ui);

    0 as _
}
