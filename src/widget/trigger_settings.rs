// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::chip::settings::trigger::{Trigger, TriggerState};
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::physics::units::{convert_mmh2o_to_cmh2o, ConvertMode};
use crate::APP_I18N;

pub struct TriggerSettingsWidgetConfig<'a> {
    pub width: f64,
    pub height: f64,

    pub trigger_settings: &'a Trigger,
    pub status_container_parent: WidgetId,
    pub status_container_widget: WidgetId,
    pub status_enabled_text_widget: WidgetId,
    pub status_enabled_button_widget: WidgetId,
    pub status_enabled_button_text_widget: WidgetId,

    pub inspiratory_offset_container_parent: WidgetId,
    pub inspiratory_offset_text_widget: WidgetId,
    pub inspiratory_offset_less_button_widget: WidgetId,
    pub inspiratory_offset_less_button_text_widget: WidgetId,
    pub inspiratory_offset_more_button_widget: WidgetId,
    pub inspiratory_offset_more_button_text_widget: WidgetId,
    pub inspiratory_offset_value_widget: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: TriggerSettingsWidgetConfig) -> f64 {
    // Initialize canvas style
    let mut canvas_style = widget::canvas::Style::default();

    canvas_style.color = Some(color::TRANSPARENT);
    canvas_style.border = Some(0.0);

    // Create canvas
    widget::Canvas::new()
        .with_style(canvas_style)
        .w_h(config.width, config.height / 2.0)
        .top_left_of(config.status_container_parent)
        .set(config.status_container_widget, &mut master.ui);

    // Append contents
    status(master, &config);
    offset(master, &config);

    0 as _
}

pub fn status<'a>(master: &mut ControlWidget<'a>, config: &TriggerSettingsWidgetConfig) {
    // Append sub-contents
    status_label(master, &config);
    status_form(master, &config);
}

pub fn status_label<'a>(master: &mut ControlWidget<'a>, config: &TriggerSettingsWidgetConfig) {
    // Initialize text style for status
    let mut status_text_style = widget::text::Style::default();

    status_text_style.font_id = Some(Some(master.fonts.regular));
    status_text_style.color = Some(color::WHITE);
    status_text_style.font_size = Some(MODAL_TEXT_FONT_SIZE);

    // Create text for status
    widget::Text::new(&APP_I18N.t("trigger-inspiratory-status"))
        .with_style(status_text_style)
        .top_left_of(config.status_container_widget)
        .set(config.status_enabled_text_widget, &mut master.ui);
}

pub fn status_form<'a>(master: &mut ControlWidget<'a>, config: &TriggerSettingsWidgetConfig) {
    // Acquire status button label
    let status_label = match config.trigger_settings.state {
        TriggerState::Enabled => APP_I18N.t("trigger-state-enabled"),
        TriggerState::Disabled => APP_I18N.t("trigger-state-disabled"),
    };

    // Initialize button style for status
    let status_style = widget::primitive::shape::Style::Fill(Some(color::WHITE));

    // Create status button
    widget::RoundedRectangle::styled([200.0, 30.0], 15.0, status_style)
        .top_left_with_margins_on(
            config.status_container_widget,
            -3.0,
            TRIGGER_SETTINGS_MODAL_FORM_PADDING_LEFT,
        )
        .set(config.status_enabled_button_widget, &mut master.ui);

    // Initialize text style for status button
    let mut status_button_text_style = widget::text::Style::default();

    status_button_text_style.font_id = Some(Some(master.fonts.bold));
    status_button_text_style.color = Some(match config.trigger_settings.state {
        TriggerState::Enabled => color::DARK_GREEN,
        TriggerState::Disabled => color::DARK_RED,
    });
    status_button_text_style.font_size = Some(16);

    // Create text for status button
    widget::Text::new(&status_label)
        .with_style(status_button_text_style)
        .mid_top_with_margin_on(config.status_enabled_button_widget, 4.0)
        .set(config.status_enabled_button_text_widget, &mut master.ui);
}

pub fn offset<'a>(master: &mut ControlWidget<'a>, config: &TriggerSettingsWidgetConfig) {
    // Initialize offset canvas style
    let mut offset_canvas_style = widget::canvas::Style::default();

    offset_canvas_style.color = Some(color::TRANSPARENT);
    offset_canvas_style.border = Some(0.0);

    // Create offset canvas
    widget::Canvas::new()
        .with_style(offset_canvas_style)
        .w_h(config.width, config.height / 2.0)
        .down_from(config.status_container_widget, 0.0)
        .set(config.inspiratory_offset_container_parent, &mut master.ui);

    // Append sub-contents
    offset_label(master, &config);
    offset_form(master, &config);
}

pub fn offset_label<'a>(master: &mut ControlWidget<'a>, config: &TriggerSettingsWidgetConfig) {
    // Initialize text style for offset
    let mut offset_text_style = widget::text::Style::default();

    offset_text_style.font_id = Some(Some(master.fonts.regular));
    offset_text_style.color = Some(color::WHITE);
    offset_text_style.font_size = Some(MODAL_TEXT_FONT_SIZE);

    // Create text for offset
    widget::Text::new(&APP_I18N.t("trigger-inspiratory-offset"))
        .with_style(offset_text_style)
        .top_left_of(config.inspiratory_offset_container_parent)
        .set(config.inspiratory_offset_text_widget, &mut master.ui);
}

pub fn offset_form<'a>(master: &mut ControlWidget<'a>, config: &TriggerSettingsWidgetConfig) {
    // Initialize button style for less
    let less_button_style = widget::primitive::shape::Style::Fill(Some(color::WHITE));

    // Create less button
    widget::RoundedRectangle::styled([50.0, 30.0], 15.0, less_button_style)
        .top_left_with_margins_on(
            config.inspiratory_offset_container_parent,
            -3.0,
            TRIGGER_SETTINGS_MODAL_FORM_PADDING_LEFT,
        )
        .set(config.inspiratory_offset_less_button_widget, &mut master.ui);

    // Initialize text style for buttons
    let mut more_less_buttons_text_style = widget::text::Style::default();

    more_less_buttons_text_style.font_id = Some(Some(master.fonts.bold));
    more_less_buttons_text_style.color = Some(color::BLACK);
    more_less_buttons_text_style.font_size = Some(MODAL_BUTTON_NAVIGATE_FONT_SIZE);

    // Create text for less button
    widget::Text::new(MODAL_BUTTON_NAVIGATE_VALUE_DECREASE)
        .with_style(more_less_buttons_text_style)
        .mid_top_with_margin_on(config.inspiratory_offset_less_button_widget, 2.0)
        .set(
            config.inspiratory_offset_less_button_text_widget,
            &mut master.ui,
        );

    // Initialize text style for value
    let mut offset_value_style = widget::text::Style::default();

    offset_value_style.font_id = Some(Some(master.fonts.regular));
    offset_value_style.color = Some(color::WHITE);
    offset_value_style.font_size = Some(MODAL_TEXT_FONT_SIZE);

    // Create text for value
    widget::Text::new(
        format!(
            "{:.1} {}",
            convert_mmh2o_to_cmh2o(
                ConvertMode::Rounded,
                config.trigger_settings.inspiratory_trigger_offset as f64
            ),
            APP_I18N.t("telemetry-unit-cmh2o")
        )
        .as_str(),
    )
    .with_style(offset_value_style)
    .right_from(config.inspiratory_offset_less_button_widget, 20.0)
    .y_relative(0.0)
    .set(config.inspiratory_offset_value_widget, &mut master.ui);

    // Create more button
    widget::RoundedRectangle::styled([50.0, 30.0], 15.0, less_button_style)
        .right_from(config.inspiratory_offset_value_widget, 20.0)
        .y_relative(-3.0)
        .set(config.inspiratory_offset_more_button_widget, &mut master.ui);

    // Create text for more button
    widget::Text::new(MODAL_BUTTON_NAVIGATE_VALUE_INCREASE)
        .with_style(more_less_buttons_text_style)
        .mid_top_with_margin_on(config.inspiratory_offset_more_button_widget, 2.0)
        .set(
            config.inspiratory_offset_more_button_text_widget,
            &mut master.ui,
        );
}
