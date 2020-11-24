// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::chip::settings::expiration_term::SettingsExpirationTerm;
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::utilities::units::{convert_mmh2o_to_cmh2o, ConvertMode};
use crate::APP_I18N;

pub struct Config<'a> {
    pub width: f64,
    pub height: f64,

    pub expiration_term_settings: &'a SettingsExpirationTerm,

    pub expiration_term_container_parent: WidgetId,
    pub expiration_term_container_widget: WidgetId,
    pub expiration_term_text_widget: WidgetId,
    pub expiration_term_less_button_widget: WidgetId,
    pub expiration_term_less_button_text_widget: WidgetId,
    pub expiration_term_more_button_widget: WidgetId,
    pub expiration_term_more_button_text_widget: WidgetId,
    pub expiration_term_value_wrapper_widget: WidgetId,
    pub expiration_term_value_widget: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create container
    gen_widget_container!(
        master,
        container_id: config.expiration_term_container_widget,
        color: color::TRANSPARENT,
        width: config.width,
        height: config.height,
        positions: top_left_of[
            config.expiration_term_container_parent,
        ]
    );

    // Append contents
    term(master, &config);

    0 as _
}

fn term<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate term label
    gen_widget_label_form!(
        master,
        text_id: config.expiration_term_text_widget,
        value: &APP_I18N.t("modal-expiration-term-expiratory-term"),
        positions: top_left_of[
            config.expiration_term_container_widget,
        ]
    );

    // Generate term navigation buttons
    gen_widget_button_navigate!(
        master,
        button_less_id: config.expiration_term_less_button_widget,
        button_less_text_id: config.expiration_term_less_button_text_widget,
        button_more_id: config.expiration_term_more_button_widget,
        button_more_text_id: config.expiration_term_more_button_text_widget,
        value_wrapper_id: config.expiration_term_value_wrapper_widget,
        value_id: config.expiration_term_value_widget,
        value: &format!(
            "{:.1}",
            convert_mmh2o_to_cmh2o(
                ConvertMode::WithDecimals,
                config.expiration_term_settings.expiratory_term as f64,
            )
        ),
        positions: top_left_with_margins_on[
            config.expiration_term_text_widget,
            -2.0,
            EXPIRATION_TERM_SETTINGS_MODAL_FORM_PADDING_LEFT,
        ]
    );
}
