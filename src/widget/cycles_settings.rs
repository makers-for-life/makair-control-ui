// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::chip::settings::cycles::SettingsCycles;
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

pub struct Config<'a> {
    pub width: f64,
    pub height: f64,

    pub cycles_settings: &'a SettingsCycles,

    pub cycles_container_parent: WidgetId,
    pub cycles_container_widget: WidgetId,
    pub cycles_text_widget: WidgetId,
    pub cycles_less_button_widget: WidgetId,
    pub cycles_less_button_text_widget: WidgetId,
    pub cycles_more_button_widget: WidgetId,
    pub cycles_more_button_text_widget: WidgetId,
    pub cycles_value_widget: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create canvas
    gen_widget_container!(
        master,
        config.cycles_container_widget,
        color::TRANSPARENT,
        config.width,
        config.height,
        top_left_of[
            config.cycles_container_parent,
        ]
    );

    // Append contents
    cycles(master, &config);

    0 as _
}

pub fn cycles<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate cycles label
    gen_widget_label!(
        master,
        config.cycles_text_widget,
        &APP_I18N.t("modal-cycles-cycles-per-minute"),
        top_left_of[
            config.cycles_container_widget,
        ]
    );

    // Generate term navigation buttons
    gen_widget_button_navigate!(
        master,
        config.cycles_less_button_widget,
        config.cycles_less_button_text_widget,
        config.cycles_more_button_widget,
        config.cycles_more_button_text_widget,
        config.cycles_value_widget,
        &format!(
            "{}{}",
            config.cycles_settings.cycles_per_minute,
            APP_I18N.t("telemetry-unit-per-minute")
        ),
        top_left_with_margins_on[
            config.cycles_text_widget,
            -2.0,
            CYCLES_SETTINGS_MODAL_FORM_PADDING_LEFT,
        ]
    );
}
