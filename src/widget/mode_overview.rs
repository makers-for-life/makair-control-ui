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

pub struct Config {
    pub background_color: Color,

    pub width: f64,
    pub height: f64,

    pub parent: WidgetId,
    pub container: WidgetId,
    pub separator: WidgetId,
    pub text_class: WidgetId,
    pub text_type: WidgetId,

    pub mode_settings: (),
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create container
    widget::rounded_rectangle::RoundedRectangle::fill_with(
        [config.width, config.height],
        TELEMETRY_WIDGET_UNIT_BORDER_RADIUS,
        config.background_color,
    )
    .top_left_of(config.parent)
    .set(config.container, &mut master.ui);

    // Append contents
    // TODO: pass ventilation mode from chip settings (which is itself synchronized from the last \
    //   snapshot)
    separator(master, &config);
    class(master, &config);
    kind(master, &config);

    0.0
}

fn separator<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    widget::Rectangle::fill_with(
        [1.0, TELEMETRY_WIDGET_RIGHT_MODE_FONT_SIZE as _],
        color::BLACK.alpha(0.35),
    )
    .middle_of(config.container)
    .x_relative(11.0)
    .set(config.separator, &mut master.ui);
}

fn class<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize text style
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::BLACK);
    text_style.font_size = Some(TELEMETRY_WIDGET_RIGHT_MODE_FONT_SIZE);

    // Create text
    // TODO: dynamic text please
    widget::Text::new(&APP_I18N.t("mode-class-pc"))
        .mid_right_with_margin_on(
            config.separator,
            TELEMETRY_WIDGET_RIGHT_MODE_SEPARATOR_SPACING,
        )
        .y_relative(2.0)
        .with_style(text_style)
        .set(config.text_class, &mut master.ui);
}

fn kind<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize text style
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.regular));
    text_style.color = Some(color::BLACK);
    text_style.font_size = Some(TELEMETRY_WIDGET_RIGHT_MODE_FONT_SIZE);

    // Create text
    // TODO: dynamic text please
    widget::Text::new(&APP_I18N.t("mode-type-cmv"))
        .mid_left_with_margin_on(
            config.separator,
            TELEMETRY_WIDGET_RIGHT_MODE_SEPARATOR_SPACING,
        )
        .y_relative(0.0)
        .with_style(text_style)
        .set(config.text_type, &mut master.ui);
}
