// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{self, Id as WidgetId},
    Positionable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

const MESSAGE_COLOR: Color = Color::Rgba(1.0, 1.0, 1.0, 0.75);

pub struct Config {
    pub container: WidgetId,
    pub title: WidgetId,
    pub message: WidgetId,
}

pub fn render(master: &mut ControlWidget, config: Config) -> f64 {
    // Initialize title style
    let mut title_style = widget::text::Style::default();

    title_style.color = Some(color::WHITE);
    title_style.font_size = Some(DISPLAY_STOP_TITLE_FONT_SIZE);
    title_style.font_id = Some(Some(master.fonts.bold));

    // Create title text
    widget::text::Text::new(&APP_I18N.t("stop-title"))
        .with_style(title_style)
        .mid_top_with_margin_on(config.container, DISPLAY_STOP_MESSAGE_PADDING_TOP)
        .set(config.title, &mut master.ui);

    // Initialize message style
    let mut message_style = widget::text::Style::default();

    message_style.color = Some(MESSAGE_COLOR);
    message_style.font_size = Some(DISPLAY_STOP_FONT_SIZE);
    message_style.font_id = Some(Some(master.fonts.regular));

    // Create message text
    widget::text::Text::new(&APP_I18N.t("stop-description"))
        .with_style(message_style)
        .mid_bottom_with_margin_on(config.container, DISPLAY_STOP_MESSAGE_PADDING_BOTTOM)
        .set(config.message, &mut master.ui);

    0 as _
}
