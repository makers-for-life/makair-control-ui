// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::chip::settings::trigger::Trigger;
use crate::display::widget::ControlWidget;
use crate::physics::units::{convert_mmh2o_to_cmh2o, ConvertMode};
use crate::APP_I18N;

pub struct ExpirationRatioSettingsWidgetConfig<'a> {
    pub width: f64,
    pub height: f64,
    pub trigger_settings: &'a Trigger,
    pub expiration_ratio_container_parent: WidgetId,
    pub expiration_ratio_container_widget: WidgetId,
    pub expiration_ratio_text_widget: WidgetId,
    pub expiration_ratio_less_button_widget: WidgetId,
    pub expiration_ratio_less_button_text_widget: WidgetId,
    pub expiration_ratio_more_button_widget: WidgetId,
    pub expiration_ratio_more_button_text_widget: WidgetId,
    pub expiration_ratio_value_widget: WidgetId,
}

pub fn render<'a>(
    master: &mut ControlWidget<'a>,
    config: ExpirationRatioSettingsWidgetConfig,
) -> f64 {
    let mut canvas_style = widget::canvas::Style::default();

    canvas_style.color = Some(color::TRANSPARENT);
    canvas_style.border = Some(0.0);

    widget::Canvas::new()
        .with_style(canvas_style)
        .w_h(config.width, config.height)
        .top_left_of(config.expiration_ratio_container_parent)
        .set(config.expiration_ratio_container_widget, &mut master.ui);

    let mut plateau_text_style = widget::text::Style::default();

    plateau_text_style.font_id = Some(Some(master.fonts.regular));
    plateau_text_style.color = Some(color::WHITE);
    plateau_text_style.font_size = Some(20);

    widget::Text::new(&APP_I18N.t("trigger-expiratory-term"))
        .with_style(plateau_text_style)
        .top_left_of(config.expiration_ratio_container_widget)
        .set(config.expiration_ratio_text_widget, &mut master.ui);

    let less_button_style = widget::primitive::shape::Style::Fill(Some(color::WHITE));

    widget::RoundedRectangle::styled([50.0, 30.0], 15.0, less_button_style)
        .top_left_with_margins_on(config.expiration_ratio_container_parent, 0.0, 300.0)
        .set(config.expiration_ratio_less_button_widget, &mut master.ui);

    let mut more_less_buttons_text_style = widget::text::Style::default();

    more_less_buttons_text_style.font_id = Some(Some(master.fonts.bold));
    more_less_buttons_text_style.color = Some(color::BLACK);
    more_less_buttons_text_style.font_size = Some(20);

    widget::Text::new("<")
        .with_style(more_less_buttons_text_style)
        .mid_top_with_margin_on(config.expiration_ratio_less_button_widget, 2.0)
        .set(
            config.expiration_ratio_less_button_text_widget,
            &mut master.ui,
        );

    let mut plateau_value_style = widget::text::Style::default();

    plateau_value_style.font_id = Some(Some(master.fonts.regular));
    plateau_value_style.color = Some(color::WHITE);
    plateau_value_style.font_size = Some(20);

    widget::Text::new(
        format!(
            "{:.1}",
            convert_mmh2o_to_cmh2o(
                ConvertMode::WithDecimals,
                config.trigger_settings.expiratory_term as f64,
            )
        )
        .as_str(),
    )
    .with_style(plateau_value_style)
    .right_from(config.expiration_ratio_less_button_widget, 20.0)
    .set(config.expiration_ratio_value_widget, &mut master.ui);

    widget::RoundedRectangle::styled([50.0, 30.0], 15.0, less_button_style)
        .right_from(config.expiration_ratio_value_widget, 20.0)
        .set(config.expiration_ratio_more_button_widget, &mut master.ui);

    widget::Text::new(">")
        .with_style(more_less_buttons_text_style)
        .mid_top_with_margin_on(config.expiration_ratio_more_button_widget, 2.0)
        .set(
            config.expiration_ratio_more_button_text_widget,
            &mut master.ui,
        );

    0.0
}
