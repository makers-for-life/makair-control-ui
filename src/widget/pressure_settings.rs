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
use crate::utilities::units::{convert_mmh2o_to_cmh2o, ConvertMode};
use crate::APP_I18N;

pub struct Config<'a> {
    pub width: f64,
    pub height: f64,

    pub pressure_settings: &'a SettingsPressure,

    pub pressure_container_parent: WidgetId,
    pub pressure_container_widget: WidgetId,

    pub pressure_plateau_text_widget: WidgetId,
    pub pressure_plateau_less_button_widget: WidgetId,
    pub pressure_plateau_less_button_text_widget: WidgetId,
    pub pressure_plateau_more_button_widget: WidgetId,
    pub pressure_plateau_more_button_text_widget: WidgetId,
    pub pressure_plateau_value_widget: WidgetId,

    pub pressure_peep_text_widget: WidgetId,
    pub pressure_peep_less_button_widget: WidgetId,
    pub pressure_peep_less_button_text_widget: WidgetId,
    pub pressure_peep_more_button_widget: WidgetId,
    pub pressure_peep_more_button_text_widget: WidgetId,
    pub pressure_peep_value_widget: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create canvas
    gen_widget_container!(
        master,
        container_id: config.pressure_container_widget,
        color: color::TRANSPARENT,
        width: config.width,
        height: config.height,
        positions: top_left_of[
            config.pressure_container_parent,
        ]
    );

    // Append contents
    plateau(master, &config);
    peep(master, &config);

    0 as _
}

fn plateau<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate plateau label
    gen_widget_label_form!(
        master,
        text_id: config.pressure_plateau_text_widget,
        value: &APP_I18N.t("modal-pressure-plateau"),
        positions: top_left_of[
            config.pressure_container_widget,
        ]
    );

    // Generate plateau navigation buttons
    gen_widget_button_navigate!(
        master,
        button_less_id: config.pressure_plateau_less_button_widget,
        button_less_text_id: config.pressure_plateau_less_button_text_widget,
        button_more_id: config.pressure_plateau_more_button_widget,
        button_more_text_id: config.pressure_plateau_more_button_text_widget,
        value_id: config.pressure_plateau_value_widget,
        value: &format!(
            "{} {}",
            convert_mmh2o_to_cmh2o(
                ConvertMode::Rounded,
                config.pressure_settings.plateau as f64
            ),
            APP_I18N.t("telemetry-unit-cmh2o")
        ),
        positions: top_left_with_margins_on[
            config.pressure_plateau_text_widget,
            -2.0,
            PRESSURE_SETTINGS_MODAL_FORM_PADDING_LEFT,
        ]
    );
}

fn peep<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate PEEP label
    gen_widget_label_form!(
        master,
        text_id: config.pressure_peep_text_widget,
        value: &APP_I18N.t("modal-pressure-expiratory"),
        positions: down_from[
            config.pressure_plateau_text_widget,
            PRESSURE_SETTINGS_MODAL_FORM_ROW_MARGIN_TOP,
        ]
    );

    // Generate PEEP navigation buttons
    gen_widget_button_navigate!(
        master,
        button_less_id: config.pressure_peep_less_button_widget,
        button_less_text_id: config.pressure_peep_less_button_text_widget,
        button_more_id: config.pressure_peep_more_button_widget,
        button_more_text_id: config.pressure_peep_more_button_text_widget,
        value_id: config.pressure_peep_value_widget,
        value: &format!(
            "{} {}",
            convert_mmh2o_to_cmh2o(ConvertMode::Rounded, config.pressure_settings.peep as f64),
            APP_I18N.t("telemetry-unit-cmh2o")
        ),
        positions: top_left_with_margins_on[
            config.pressure_peep_text_widget,
            -2.0,
            PRESSURE_SETTINGS_MODAL_FORM_PADDING_LEFT,
        ]
    );
}
