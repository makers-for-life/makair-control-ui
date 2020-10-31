// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::chip::settings::trigger::{SettingsTrigger, SettingsTriggerState};
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
    pub inspiratory_offset_value_widget: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create container
    gen_widget_container!(
        master,
        config.status_container_widget,
        color::TRANSPARENT,
        config.width,
        config.height / 2.0,
        top_left_of[
            config.status_container_parent,
        ]
    );

    // Append contents
    status(master, &config);
    offset(master, &config);

    0 as _
}

pub fn status<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Append sub-contents
    status_label(master, &config);
    status_form(master, &config);
}

pub fn status_label<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate status label
    gen_widget_label!(
        master,
        config.status_enabled_text_widget,
        &APP_I18N.t("modal-trigger-inspiratory-status"),
        top_left_of[
            config.status_container_widget,
        ]
    );
}

pub fn status_form<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Acquire status button color & label
    let status_label = match config.trigger_settings.state {
        SettingsTriggerState::Enabled => APP_I18N.t("trigger-state-enabled"),
        SettingsTriggerState::Disabled => APP_I18N.t("trigger-state-disabled"),
    };
    let button_color = match config.trigger_settings.state {
        SettingsTriggerState::Enabled => color::DARK_GREEN,
        SettingsTriggerState::Disabled => color::DARK_RED,
    };

    // Append status button
    gen_widget_button!(
        master,
        config.status_enabled_button_widget,
        config.status_enabled_button_text_widget,
        button_color,
        16,
        200.0,
        4.0,
        &status_label,
        (top_left_with_margins_on[
            config.status_container_widget,
            -3.0,
            TRIGGER_SETTINGS_MODAL_FORM_PADDING_LEFT,
        ])
    );
}

pub fn offset<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Create container
    gen_widget_container!(
        master,
        config.inspiratory_offset_container_parent,
        color::TRANSPARENT,
        config.width,
        config.height / 2.0,
        down_from[
            config.status_container_widget,
            0.0,
        ]
    );

    // Append sub-contents
    offset_label(master, &config);
    offset_form(master, &config);
}

pub fn offset_label<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate offset label
    gen_widget_label!(
        master,
        config.inspiratory_offset_text_widget,
        &APP_I18N.t("modal-trigger-inspiratory-offset"),
        top_left_of[
            config.inspiratory_offset_container_parent,
        ]
    );
}

pub fn offset_form<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate offset navigation buttons
    gen_widget_button_navigate!(
        master,
        config.inspiratory_offset_less_button_widget,
        config.inspiratory_offset_less_button_text_widget,
        config.inspiratory_offset_more_button_widget,
        config.inspiratory_offset_more_button_text_widget,
        config.inspiratory_offset_value_widget,
        &format!(
            "{:.1} {}",
            convert_mmh2o_to_cmh2o(
                ConvertMode::WithDecimals,
                config.trigger_settings.inspiratory_trigger_offset as f64
            ),
            APP_I18N.t("telemetry-unit-cmh2o")
        ),
        top_left_with_margins_on[
            config.inspiratory_offset_text_widget,
            -2.0,
            TRIGGER_SETTINGS_MODAL_FORM_PADDING_LEFT,
        ]
    );
}
