// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::chip::settings::{trigger::SettingsTrigger, SettingActionState};
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::utilities::units::{convert_mmh2o_to_cmh2o, ConvertMode};
use crate::APP_I18N;

pub struct Config<'a> {
    pub width: f64,
    pub height: f64,

    pub trigger_settings: &'a SettingsTrigger,

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
    pub inspiratory_offset_value_wrapper_widget: WidgetId,
    pub inspiratory_offset_value_widget: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create container
    gen_widget_container!(
        master,
        container_id: config.status_container_widget,
        color: color::TRANSPARENT,
        width: config.width,
        height: config.height / 2.0,
        positions: top_left_of[
            config.status_container_parent,
        ]
    );

    // Append contents
    status(master, &config);
    offset(master, &config);

    0 as _
}

fn status<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Append sub-contents
    status_label(master, &config);
    status_form(master, &config);
}

fn status_label<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate status label
    gen_widget_label_form!(
        master,
        text_id: config.status_enabled_text_widget,
        value: &APP_I18N.t("modal-trigger-inspiratory-status"),
        positions: top_left_of[
            config.status_container_widget,
        ]
    );
}

fn status_form<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Acquire status button color & label
    let status_label = match config.trigger_settings.state {
        SettingActionState::Enabled => APP_I18N.t("modal-trigger-inspiratory-status-enabled"),
        SettingActionState::Disabled => APP_I18N.t("modal-trigger-inspiratory-status-disabled"),
    };
    let button_color = match config.trigger_settings.state {
        SettingActionState::Enabled => color::DARK_GREEN,
        SettingActionState::Disabled => color::DARK_RED,
    };

    // Append status button
    gen_widget_button!(
        master,
        button_id: config.status_enabled_button_widget,
        text_id: config.status_enabled_button_text_widget,
        text_color: button_color,
        text_font_size: 16,
        width: 200.0,
        value_top: 4.0,
        value: &status_label,

        positions: (
            top_left_with_margins_on[
                config.status_container_widget,
                -3.0,
                TRIGGER_SETTINGS_MODAL_FORM_PADDING_LEFT,
            ]
        )
    );
}

fn offset<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Create container
    gen_widget_container!(
        master,
        container_id: config.inspiratory_offset_container_parent,
        color: color::TRANSPARENT,
        width: config.width,
        height: config.height / 2.0,
        positions: down_from[
            config.status_container_widget,
            0.0,
        ]
    );

    // Append sub-contents
    offset_label(master, &config);
    offset_form(master, &config);
}

fn offset_label<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate offset label
    gen_widget_label_form!(
        master,
        text_id: config.inspiratory_offset_text_widget,
        value: &APP_I18N.t("modal-trigger-inspiratory-offset"),
        positions: top_left_of[
            config.inspiratory_offset_container_parent,
        ]
    );
}

fn offset_form<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate offset navigation buttons
    gen_widget_button_navigate!(
        master,
        button_less_id: config.inspiratory_offset_less_button_widget,
        button_less_text_id: config.inspiratory_offset_less_button_text_widget,
        button_more_id: config.inspiratory_offset_more_button_widget,
        button_more_text_id: config.inspiratory_offset_more_button_text_widget,
        value_wrapper_id: config.inspiratory_offset_value_wrapper_widget,
        value_id: config.inspiratory_offset_value_widget,
        value: &format!(
            "{:.1} {}",
            convert_mmh2o_to_cmh2o(
                ConvertMode::WithDecimals,
                config.trigger_settings.inspiratory_trigger_offset as f64
            ),
            APP_I18N.t("telemetry-unit-cmh2o")
        ),
        positions: top_left_with_margins_on[
            config.inspiratory_offset_text_widget,
            -2.0,
            TRIGGER_SETTINGS_MODAL_FORM_PADDING_LEFT,
        ]
    );
}
