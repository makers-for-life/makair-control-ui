// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{self, Id as WidgetId},
    Positionable, Widget,
};
use telemetry::structures::VentilationModeClass;

use crate::chip::settings::mode::SettingsMode;
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::locale::modes::{
    class_to_locale as mode_class_to_locale, kind_to_locale as mode_kind_to_locale,
};

const CLASS_PRESSURE_COLOR: color::Color =
    color::Color::Rgba(14.0 / 255.0, 112.0 / 255.0, 182.0 / 255.0, 1.0);
const CLASS_FLOW_COLOR: color::Color =
    color::Color::Rgba(150.0 / 255.0, 20.0 / 255.0, 6.0 / 255.0, 1.0);

pub struct Config<'a> {
    pub background_color: Color,

    pub width: f64,
    pub height: f64,

    pub parent: WidgetId,
    pub container: WidgetId,
    pub separator: WidgetId,
    pub text_class: WidgetId,
    pub text_type: WidgetId,

    pub mode_settings: &'a SettingsMode,
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
    separator(master, &config);
    class(master, &config);
    kind(master, &config);

    0.0
}

fn separator<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    widget::Rectangle::fill_with(
        [1.0, TELEMETRY_WIDGET_RIGHT_MODE_FONT_SIZE as _],
        color::BLACK.alpha(0.65),
    )
    .middle_of(config.container)
    .x_relative(11.0)
    .set(config.separator, &mut master.ui);
}

fn class<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Acquire mode class
    let mode_class = config.mode_settings.live.mode.class();

    // Initialize text style
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(match mode_class {
        VentilationModeClass::Pressure => CLASS_PRESSURE_COLOR,
        VentilationModeClass::Volume => CLASS_FLOW_COLOR,
    });
    text_style.font_size = Some(TELEMETRY_WIDGET_RIGHT_MODE_FONT_SIZE);

    // Create text
    widget::Text::new(&mode_class_to_locale(mode_class))
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

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::BLACK);
    text_style.font_size = Some(TELEMETRY_WIDGET_RIGHT_MODE_FONT_SIZE);

    // Create text
    widget::Text::new(&mode_kind_to_locale(config.mode_settings.live.mode.kind()))
        .mid_left_with_margin_on(
            config.separator,
            TELEMETRY_WIDGET_RIGHT_MODE_SEPARATOR_SPACING,
        )
        .y_relative(0.0)
        .with_style(text_style)
        .set(config.text_type, &mut master.ui);
}
