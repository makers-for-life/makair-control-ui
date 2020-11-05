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
use crate::APP_I18N;

pub struct Config {
    container: WidgetId,
    icon: WidgetId,
    text_wrapper: WidgetId,
    text_title: WidgetId,
    text_message: WidgetId,

    width: f64,
    height: f64,

    image: conrod_core::image::Id,

    message: String,
}

impl Config {
    pub fn new(
        container_id: WidgetId,
        icon_id: WidgetId,
        text_wrapper_id: WidgetId,
        text_title_id: WidgetId,
        text_message_id: WidgetId,
        width: f64,
        height: f64,
        image: conrod_core::image::Id,
        message: String,
    ) -> Config {
        Config {
            container: container_id,
            icon: icon_id,
            text_wrapper: text_wrapper_id,
            text_title: text_title_id,
            text_message: text_message_id,
            width,
            height,
            image,
            message,
        }
    }
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create container
    gen_widget_container!(
        master,
        container_id: config.container,
        color: color::TRANSPARENT,
        width: ERROR_WIDTH,
        height: ERROR_HEIGHT,
        positions: middle[]
    );

    // Append image
    image(master, &config);

    // Append text contents
    text_wrapper(master, &config);
    text_title(master, &config);
    text_message(master, &config);

    0 as _
}

pub fn image<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Create image
    widget::Image::new(config.image)
        .w_h(config.width, config.height)
        .mid_top_of(config.container)
        .set(config.icon, &mut master.ui);
}

pub fn text_wrapper<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    gen_widget_container!(
        master,
        container_id: config.text_wrapper,
        color: color::TRANSPARENT,
        width: ERROR_WIDTH,
        height: ERROR_TEXT_BOX_HEIGHT,

        positions: mid_bottom_of[
            config.container,
        ]
    );
}

pub fn text_title<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize text style
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::WHITE);
    text_style.font_size = Some(ERROR_TITLE_FONT_SIZE);

    // Create text
    widget::Text::new(&APP_I18N.t("error-title"))
        .mid_top_of(config.text_wrapper)
        .with_style(text_style)
        .set(config.text_title, &mut master.ui);
}

pub fn text_message<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize text style
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.regular));
    text_style.color = Some(color::WHITE.with_alpha(0.5));
    text_style.font_size = Some(ERROR_MESSAGE_FONT_SIZE);

    // Create text
    widget::Text::new(&config.message)
        .mid_bottom_of(config.text_wrapper)
        .with_style(text_style)
        .set(config.text_message, &mut master.ui);
}
