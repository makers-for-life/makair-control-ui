// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Colorable, Positionable, Widget,
};

use crate::display::widget::ControlWidget;
use crate::APP_I18N;

pub struct ErrorWidgetConfig {
    error: String,
    id: WidgetId,
}

impl ErrorWidgetConfig {
    pub fn new(error: String, id: WidgetId) -> ErrorWidgetConfig {
        ErrorWidgetConfig { error, id }
    }
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: ErrorWidgetConfig) -> f64 {
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::WHITE);
    text_style.font_size = Some(30);

    widget::Text::new(&format!("{}\n{}", APP_I18N.t("error-title"), config.error)) // using \n instead of the wrap methods because I couldn't make them work
        .color(color::WHITE)
        .align_top() // Aligned to top otherwise I can't make the line breaks work
        .with_style(text_style)
        .set(config.id, &mut master.ui);

    0 as _
}
