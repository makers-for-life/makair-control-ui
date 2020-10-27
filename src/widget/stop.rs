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

pub struct StopWidgetConfig {
    pub container: WidgetId,
    pub title: WidgetId,
    pub message: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: StopWidgetConfig) -> f64 {
    let mut title_style = widget::text::Style::default();

    title_style.color = Some(color::WHITE);
    title_style.font_size = Some(19);
    title_style.font_id = Some(Some(master.fonts.bold));

    widget::text::Text::new(&APP_I18N.t("stop-title"))
        .with_style(title_style)
        .mid_top_with_margin_on(config.container, DISPLAY_STOPPED_MESSAGE_PADDING_TOP)
        .set(config.title, &mut master.ui);

    let mut message_style = widget::text::Style::default();

    message_style.color = Some(Color::Rgba(1.0, 1.0, 1.0, 0.75));
    message_style.font_size = Some(14);
    message_style.font_id = Some(Some(master.fonts.regular));

    widget::text::Text::new(&APP_I18N.t("stop-description"))
        .with_style(message_style)
        .mid_bottom_with_margin_on(config.container, DISPLAY_STOPPED_MESSAGE_PADDING_BOTTOM)
        .set(config.message, &mut master.ui);

    0 as _
}
