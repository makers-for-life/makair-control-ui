// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::chip::settings::pressure::SettingsPressure;
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

pub struct Config<'a> {
    pub width: f64,
    pub height: f64,
    pub pressure_settings: &'a SettingsPressure,
    pub pressure_container_parent: WidgetId,
    pub pressure_container_widget: WidgetId,
    pub pressure_peak_text_widget: WidgetId,
    pub pressure_peak_less_button_widget: WidgetId,
    pub pressure_peak_less_button_text_widget: WidgetId,
    pub pressure_peak_more_button_widget: WidgetId,
    pub pressure_peak_more_button_text_widget: WidgetId,
    pub pressure_peak_value_widget: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Initialize canvas style
    let mut canvas_style = widget::canvas::Style::default();

    canvas_style.color = Some(color::TRANSPARENT);
    canvas_style.border = Some(0.0);

    // Create canvas
    widget::Canvas::new()
        .with_style(canvas_style)
        .w_h(config.width, config.height)
        .top_left_of(config.pressure_container_parent)
        .set(config.pressure_container_widget, &mut master.ui);

    // Append contents
    peak(master, &config);
    plateau(master, &config);
    peep(master, &config);

    0 as _
}

pub fn peak<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize text style for label
    let mut label_text_style = widget::text::Style::default();

    label_text_style.font_id = Some(Some(master.fonts.regular));
    label_text_style.color = Some(color::WHITE);
    label_text_style.font_size = Some(MODAL_TEXT_FONT_SIZE);

    // Create text for label
    widget::Text::new(&APP_I18N.t("modal-pressure-peak"))
        .with_style(label_text_style)
        .top_left_of(config.pressure_container_widget)
        .set(config.pressure_peak_text_widget, &mut master.ui);

    // Initialize button style for less
    let less_button_style = widget::primitive::shape::Style::Fill(Some(color::WHITE));

    // Create less button
    widget::RoundedRectangle::styled([50.0, 30.0], 15.0, less_button_style)
        .top_left_with_margins_on(
            config.pressure_container_parent,
            -3.0,
            PRESSURE_SETTINGS_MODAL_FORM_PADDING_LEFT,
        )
        .set(config.pressure_peak_less_button_widget, &mut master.ui);

    // Initialize text style for buttons
    let mut more_less_buttons_text_style = widget::text::Style::default();

    more_less_buttons_text_style.font_id = Some(Some(master.fonts.bold));
    more_less_buttons_text_style.color = Some(color::BLACK);
    more_less_buttons_text_style.font_size = Some(MODAL_BUTTON_NAVIGATE_FONT_SIZE);

    // Create text for less button
    widget::Text::new(MODAL_BUTTON_NAVIGATE_VALUE_DECREASE)
        .with_style(more_less_buttons_text_style)
        .mid_top_with_margin_on(config.pressure_peak_less_button_widget, 2.0)
        .set(config.pressure_peak_less_button_text_widget, &mut master.ui);

    // Initialize text style for value
    let mut value_text_style = widget::text::Style::default();

    value_text_style.font_id = Some(Some(master.fonts.regular));
    value_text_style.color = Some(color::WHITE);
    value_text_style.font_size = Some(MODAL_TEXT_FONT_SIZE);

    // Create text for value
    widget::Text::new(&config.pressure_settings.peak.to_string())
        .with_style(value_text_style)
        .right_from(config.pressure_peak_less_button_widget, 20.0)
        .y_relative(0.0)
        .set(config.pressure_peak_value_widget, &mut master.ui);

    // Create more button
    widget::RoundedRectangle::styled([50.0, 30.0], 15.0, less_button_style)
        .right_from(config.pressure_peak_value_widget, 20.0)
        .y_relative(-3.0)
        .set(config.pressure_peak_more_button_widget, &mut master.ui);

    // Create text for more button
    widget::Text::new(MODAL_BUTTON_NAVIGATE_VALUE_INCREASE)
        .with_style(more_less_buttons_text_style)
        .mid_top_with_margin_on(config.pressure_peak_more_button_widget, 2.0)
        .set(config.pressure_peak_more_button_text_widget, &mut master.ui);
}

pub fn plateau<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // TODO: modal-pressure-plateau
}

pub fn peep<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // TODO: modal-pressure-expiratory
}
