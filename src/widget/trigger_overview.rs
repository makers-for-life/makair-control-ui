// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{self, Id as WidgetId},
    Positionable, Widget,
};

use crate::chip::settings::trigger::{Trigger, TriggerState};
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::physics::units::{convert_mmh2o_to_cmh2o, ConvertMode};
use crate::APP_I18N;

pub struct TriggerOverviewWidgetConfig<'a> {
    pub parent: WidgetId,
    pub container: WidgetId,
    pub border: WidgetId,
    pub title_widget: WidgetId,
    pub status_widget: WidgetId,
    pub inspiration_trigger_offset_widget: WidgetId,
    pub configure_widget: WidgetId,
    pub expiratory_term_widget: WidgetId,
    pub plateau_duration_widget: WidgetId,
    pub width: f64,
    pub height: f64,
    pub x_position: f64,
    pub y_position: f64,
    pub background_color: Color,
    pub trigger_settings: &'a Trigger,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: TriggerOverviewWidgetConfig) -> f64 {
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

fn title<'a>(master: &mut ControlWidget<'a>, config: &TriggerOverviewWidgetConfig) {
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

fn status<'a>(master: &mut ControlWidget<'a>, config: &TriggerOverviewWidgetConfig) {
    // Initialize text style
    let mut text_style = widget::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.regular));
    text_style.color = Some(color::WHITE);
    text_style.font_size = Some(TELEMETRY_WIDGET_LABELS_FONT_SIZE);

    let status = if config.trigger_settings.state == TriggerState::Enabled {
        APP_I18N.t("trigger-state-enabled")
    } else {
        APP_I18N.t("trigger-state-disabled")
    };

    // Create text
    widget::Text::new(&format!("{} {}", APP_I18N.t("trigger-label-state"), status))
        .with_style(text_style)
        .down_from(config.title_widget, 13.0)
        .set(config.status_widget, &mut master.ui);
}

fn offset<'a>(master: &mut ControlWidget<'a>, config: &TriggerOverviewWidgetConfig) {
    // Initialize text style
    let mut text_style = widget::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.regular));
    text_style.color = Some(color::WHITE);
    text_style.font_size = Some(TELEMETRY_WIDGET_LABELS_FONT_SIZE);

    // Create text
    widget::Text::new(&format!(
        "{} {:.1} {}",
        APP_I18N.t("trigger-label-offset"),
        convert_mmh2o_to_cmh2o(
            ConvertMode::Rounded,
            config.trigger_settings.inspiratory_trigger_offset as f64
        ),
        APP_I18N.t("telemetry-unit-cmh2o")
    ))
    .with_style(text_style)
    .down_from(config.status_widget, 5.0)
    .set(config.inspiration_trigger_offset_widget, &mut master.ui);
}

fn configure<'a>(master: &mut ControlWidget<'a>, config: &TriggerOverviewWidgetConfig) {
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
