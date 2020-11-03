// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::chip::settings::{run::SettingsRun, SettingActionState};
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

pub struct Config<'a> {
    pub width: f64,
    pub height: f64,

    pub run_settings: &'a SettingsRun,

    pub status_container_parent: WidgetId,
    pub status_container_widget: WidgetId,
    pub status_enabled_text_widget: WidgetId,
    pub status_enabled_button_widget: WidgetId,
    pub status_enabled_button_text_widget: WidgetId,
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
        text_id: config.status_enabled_text_widget,
        value: &APP_I18N.t("modal-run-status"),
        positions: top_left_of[
            config.status_container_widget,
        ]
    );
}

pub fn status_form<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Acquire status button color & label
    let status_label = match config.run_settings.state {
        SettingActionState::Enabled => APP_I18N.t("modal-run-status-started"),
        SettingActionState::Disabled => APP_I18N.t("modal-run-status-stopped"),
    };
    let button_color = match config.run_settings.state {
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
        width: 280.0,
        value_top: 4.0,
        value: &status_label,

        positions: (
            top_left_with_margins_on[
                config.status_container_widget,
                -3.0,
                RUN_SETTINGS_MODAL_FORM_PADDING_LEFT,
            ]
        )
    );
}
