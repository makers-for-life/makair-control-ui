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
    pub expiration_term_value_widget: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create container
    gen_widget_container!(
        master,
        config.expiration_term_container_widget,
        color::TRANSPARENT,
        config.width,
        config.height,
        top_left_of[
            config.expiration_term_container_parent,
        ]
    );

    // Append contents
    term(master, &config);

    0 as _
}

pub fn term<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate term label
    gen_widget_label!(
        master,
        config.expiration_term_text_widget,
        &APP_I18N.t("modal-expiration-term-expiratory-term"),
        top_left_of[
            config.expiration_term_container_widget,
        ]
    );

    // Generate term navigation buttons
    gen_widget_button_navigate!(
        master,
        config.expiration_term_less_button_widget,
        config.expiration_term_less_button_text_widget,
        config.expiration_term_more_button_widget,
        config.expiration_term_more_button_text_widget,
        config.expiration_term_value_widget,
        &format!(
            "{:.1}",
            convert_mmh2o_to_cmh2o(
                ConvertMode::WithDecimals,
                config.expiration_term_settings.expiratory_term as f64,
            )
        ),
        top_left_with_margins_on[
            config.expiration_term_text_widget,
            -2.0,
            EXPIRATION_TERM_SETTINGS_MODAL_FORM_PADDING_LEFT,
        ]
    );
}
