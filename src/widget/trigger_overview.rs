// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{self, Id as WidgetId},
    Positionable, Widget,
};

use crate::chip::settings::trigger::{SettingsTrigger, SettingsTriggerState};
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::utilities::units::{convert_mmh2o_to_cmh2o, ConvertMode};
use crate::APP_I18N;

pub struct Config<'a> {
    pub parent: WidgetId,
    pub container: WidgetId,
    pub border: WidgetId,
    pub title_widget: WidgetId,
    pub status_label_widget: WidgetId,
    pub status_value_widget: WidgetId,
    pub offset_label_widget: WidgetId,
    pub offset_value_widget: WidgetId,
    pub configure_widget: WidgetId,
    pub expiratory_term_widget: WidgetId,
    pub plateau_duration_widget: WidgetId,
    pub width: f64,
    pub height: f64,
    pub x_position: f64,
    pub y_position: f64,
    pub background_color: Color,
    pub trigger_settings: &'a SettingsTrigger,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create container
    widget::rectangle::Rectangle::fill_with([config.width, config.height], config.background_color)
        .bottom_left_with_margins_on(config.parent, config.y_position, config.x_position)
        .set(config.container, &mut master.ui);

    // Append left border
    widget::rectangle::Rectangle::fill_with([1.0, config.height], color::BLACK.with_alpha(0.25))
        .top_left_with_margins_on(config.container, 0.0, -1.0)
        .set(config.border, &mut master.ui);

    // Append contents
    title(master, &config);
    status(master, &config);
    offset(master, &config);
    configure(master, &config);

    0 as _
}

fn title<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize text style
    let mut text_style = widget::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::WHITE);
    text_style.font_size = Some(TELEMETRY_WIDGET_TITLE_FONT_SIZE);

    // Create text
    widget::Text::new(&APP_I18N.t("trigger-label-title"))
        .with_style(text_style)
        .top_left_with_margins_on(
            config.container,
            TELEMETRY_WIDGET_UNIT_PADDING_BOTTOM_TOP,
            TELEMETRY_WIDGET_PADDING_LEFT_DEEP,
        )
        .set(config.title_widget, &mut master.ui);
}

fn status<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize label text style
    let mut label_text_style = widget::text::Style::default();

    label_text_style.font_id = Some(Some(master.fonts.regular));
    label_text_style.color = Some(color::WHITE);
    label_text_style.font_size = Some(TELEMETRY_WIDGET_LABELS_FONT_SIZE);

    // Create label text
    widget::Text::new(&APP_I18N.t("trigger-label-state"))
        .with_style(label_text_style)
        .down_from(config.title_widget, 13.0)
        .set(config.status_label_widget, &mut master.ui);

    // Generate value text
    let value_text_status = if config.trigger_settings.state == SettingsTriggerState::Enabled {
        APP_I18N.t("trigger-state-enabled")
    } else {
        APP_I18N.t("trigger-state-disabled")
    };

    // Initialize value text style
    let mut value_text_style = widget::text::Style::default();

    value_text_style.font_id = Some(Some(master.fonts.bold));
    value_text_style.font_size = Some(TELEMETRY_WIDGET_LABELS_FONT_SIZE);

    value_text_style.color = Some(
        if config.trigger_settings.state == SettingsTriggerState::Enabled {
            color::WHITE
        } else {
            color::BLACK.with_alpha(0.8)
        },
    );

    // Create value text
    widget::Text::new(&value_text_status)
        .with_style(value_text_style)
        .mid_left_with_margin_on(
            config.status_label_widget,
            TELEMETRY_WIDGET_LABELS_LABEL_WIDTH,
        )
        .set(config.status_value_widget, &mut master.ui);
}

fn offset<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize label text style
    let mut label_text_style = widget::text::Style::default();

    label_text_style.font_id = Some(Some(master.fonts.regular));
    label_text_style.color = Some(color::WHITE);
    label_text_style.font_size = Some(TELEMETRY_WIDGET_LABELS_FONT_SIZE);

    // Create text
    widget::Text::new(&APP_I18N.t("trigger-label-offset"))
        .with_style(label_text_style)
        .down_from(config.status_label_widget, 5.0)
        .set(config.offset_label_widget, &mut master.ui);

    // Initialize value text style
    let mut value_text_style = widget::text::Style::default();

    value_text_style.font_id = Some(Some(master.fonts.regular));
    value_text_style.color = Some(color::WHITE);
    value_text_style.font_size = Some(TELEMETRY_WIDGET_LABELS_FONT_SIZE);

    // Create value text
    widget::Text::new(&format!(
        "{:.1} {}",
        convert_mmh2o_to_cmh2o(
            ConvertMode::WithDecimals,
            config.trigger_settings.inspiratory_trigger_offset as f64
        ),
        APP_I18N.t("telemetry-unit-cmh2o")
    ))
    .with_style(value_text_style)
    .mid_left_with_margin_on(
        config.offset_label_widget,
        TELEMETRY_WIDGET_LABELS_LABEL_WIDTH,
    )
    .set(config.offset_value_widget, &mut master.ui);
}

fn configure<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize text style
    let mut text_style = widget::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::BLACK.with_alpha(0.8));
    text_style.font_size = Some(TELEMETRY_WIDGET_UNIT_FONT_SIZE);

    // Create text
    widget::Text::new(&APP_I18N.t("trigger-label-configure"))
        .with_style(text_style)
        .bottom_left_with_margins_on(
            config.container,
            TELEMETRY_WIDGET_UNIT_PADDING_BOTTOM_TOP,
            TELEMETRY_WIDGET_PADDING_LEFT_DEEP,
        )
        .set(config.configure_widget, &mut master.ui);
}
