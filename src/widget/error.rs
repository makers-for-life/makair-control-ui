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
    pub container: WidgetId,
    pub icon: WidgetId,
    pub text_wrapper: WidgetId,
    pub text_title: WidgetId,
    pub text_message: WidgetId,

    pub width: f64,
    pub height: f64,

    pub image: conrod_core::image::Id,

    pub title: String,
    pub message: String,
}

pub fn render(master: &mut ControlWidget, config: Config) -> f64 {
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

fn image<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Create image
    widget::Image::new(config.image)
        .w_h(config.width, config.height)
        .mid_top_of(config.container)
        .set(config.icon, &mut master.ui);
}

fn text_wrapper<'a>(master: &mut ControlWidget<'a>, config: &Config) {
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

fn text_title<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize text style
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::WHITE);
    text_style.font_size = Some(ERROR_TITLE_FONT_SIZE);

    // Create text
    widget::Text::new(&config.title)
        .mid_top_of(config.text_wrapper)
        .with_style(text_style)
        .set(config.text_title, &mut master.ui);
}

fn text_message<'a>(master: &mut ControlWidget<'a>, config: &Config) {
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
