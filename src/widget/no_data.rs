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
use crate::APP_I18N;

pub struct Config {
    id: WidgetId,
}

impl Config {
    pub fn new(id: WidgetId) -> Config {
        Config { id }
    }
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Initialize text style
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::WHITE);
    text_style.font_size = Some(SPLASH_MESSAGE_FONT_SIZE);

    // Create text
    widget::Text::new(&APP_I18N.t("no-data-title"))
        .color(color::WHITE)
        .middle()
        .with_style(text_style)
        .set(config.id, &mut master.ui);

    0 as _
}
