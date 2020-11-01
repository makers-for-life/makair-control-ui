// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

pub struct Config {
    error: String,
    id: WidgetId,
}

impl Config {
    pub fn new(error: String, id: WidgetId) -> Config {
        Config { error, id }
    }
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Initialize style
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::WHITE);
    text_style.font_size = Some(ERROR_MESSAGE_FONT_SIZE);

    // Create text
    widget::Text::new(&format!(
        "{}\n\n{}",
        APP_I18N.t("error-title"),
        config.error
    ))
    .top_left_with_margins(ERROR_PADDING_TOP, ERROR_PADDING_LEFT)
    .with_style(text_style)
    .set(config.id, &mut master.ui);

    0 as _
}
