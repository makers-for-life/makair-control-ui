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

use crate::chip::settings::{snooze::SettingsSnooze, SettingActionState};
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

pub struct Config<'a> {
    pub width: f64,
    pub height: f64,

    pub snooze_settings: &'a SettingsSnooze,

    pub container_parent: WidgetId,
    pub container_widget: WidgetId,

    pub alarms_enabled_text_widget: WidgetId,
    pub alarms_enabled_button_widget: WidgetId,
    pub alarms_enabled_button_text_widget: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create container
    gen_widget_container!(
        master,
        container_id: config.container_widget,
        color: color::TRANSPARENT,
        width: config.width,
        height: config.height / 2.0,
        positions: top_left_of[
            config.container_parent,
        ]
    );

    // Append contents
    alarms(master, &config);

    0 as _
}

fn alarms<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Append sub-contents
    alarms_label(master, config);
    alarms_form(master, config);
}

fn alarms_label<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate alarms label
    gen_widget_label_form!(
        master,
        text_id: config.alarms_enabled_text_widget,
        value: &APP_I18N.t("modal-snooze-alarms"),
        positions: top_left_of[
            config.container_widget,
        ]
    );
}

fn alarms_form<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Acquire alarms button color & label
    let alarms_label = match config.snooze_settings.alarms {
        SettingActionState::Disabled => APP_I18N.t("modal-snooze-alarms-active"),
        SettingActionState::Enabled => APP_I18N.t("modal-snooze-alarms-inactive"),
    };
    let button_color = match config.snooze_settings.alarms {
        SettingActionState::Disabled => color::DARK_BLUE,
        SettingActionState::Enabled => color::DARK_RED,
    };

    // Append alarms button
    gen_widget_button!(
        master,
        button_id: config.alarms_enabled_button_widget,
        text_id: config.alarms_enabled_button_text_widget,
        text_color: button_color,
        text_font_size: MODAL_BUTTON_FONT_SIZE,
        width: 320.0,
        value_top: MODAL_BUTTON_VALUE_TOP,
        value: &alarms_label,

        positions: (
            top_left_with_margins_on[
                config.container_widget,
                -4.5,
                SNOOZE_SETTINGS_MODAL_FORM_PADDING_LEFT,
            ]
        )
    );
}
