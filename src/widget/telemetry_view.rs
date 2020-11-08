// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::iter::once;

use conrod_core::{
    color::{self, Color},
    widget::{self, Id as WidgetId},
    Colorable, Positionable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;

const ARROW_COLOR: Color = Color::Rgba(1.0, 1.0, 1.0, 0.25);

pub struct Config {
    pub title: String,
    pub value_measured: Option<String>,
    pub value_target: Option<String>,
    pub unit: String,

    pub ids: (
        WidgetId,
        WidgetId,
        WidgetId,
        WidgetId,
        (WidgetId, WidgetId),
        WidgetId,
        Option<WidgetId>,
    ),

    pub x_position: f64,
    pub y_position: f64,
    pub background_color: Color,
    pub width: f64,
    pub height: f64,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create rounded rectangle
    widget::rounded_rectangle::RoundedRectangle::fill_with(
        [config.width, config.height],
        TELEMETRY_WIDGET_UNIT_BORDER_RADIUS,
        config.background_color,
    )
    .bottom_left_with_margins_on(config.ids.0, config.y_position, config.x_position)
    .set(config.ids.1, &mut master.ui);

    // Create title text
    widget::Text::new(&config.title)
        .color(color::WHITE)
        .top_left_with_margins_on(
            config.ids.1,
            TELEMETRY_WIDGET_UNIT_PADDING_BOTTOM_TOP,
            TELEMETRY_WIDGET_PADDING_LEFT,
        )
        .font_size(TELEMETRY_WIDGET_TITLE_FONT_SIZE)
        .set(config.ids.2, &mut master.ui);

    // Initiate text style for measured value
    let mut value_text_style = conrod_core::widget::primitive::text::Style::default();

    value_text_style.font_id = Some(Some(master.fonts.bold));
    value_text_style.color = Some(color::WHITE);
    value_text_style.font_size = Some(40);

    // Create value text
    // Notice: there are different drawing cases depending on values provided
    match (config.value_measured, config.value_target) {
        (Some(value_measured), Some(value_target)) => {
            // Initiate text sub-style for target value
            let mut target_text_style = conrod_core::widget::primitive::text::Style::default();

            target_text_style.font_id = Some(Some(master.fonts.regular));
            target_text_style.color = Some(color::WHITE);
            target_text_style.font_size = Some(29);

            // Draw measured value
            widget::Text::new(&value_measured)
                .with_style(value_text_style)
                .mid_left_with_margin_on(config.ids.1, TELEMETRY_WIDGET_PADDING_LEFT)
                .set(config.ids.3, &mut master.ui);

            // Draw arrow
            widget::polygon::Polygon::centred_styled(
                once([0.0, ((TELEMETRY_ARROW_MAIN_HEIGHT - 1) / 2) as _])
                    .chain(once([(TELEMETRY_ARROW_MAIN_WIDTH - 1) as _, 0.0]))
                    .chain(once([
                        (TELEMETRY_ARROW_MAIN_WIDTH - 1) as _,
                        (TELEMETRY_ARROW_MAIN_HEIGHT - 1) as _,
                    ])),
                widget::primitive::shape::Style::Fill(Some(ARROW_COLOR)),
            )
            .right_from(config.ids.3, TELEMETRY_ARROW_SPACING_SIDES)
            .y_relative_to(config.ids.3, -7.0)
            .set((config.ids.4).0, &mut master.ui);

            widget::rectangle::Rectangle::fill_with(
                [
                    TELEMETRY_ARROW_LINE_WIDTH as _,
                    TELEMETRY_ARROW_LINE_HEIGHT as _,
                ],
                ARROW_COLOR,
            )
            .right_from((config.ids.4).0, 0.0)
            .y_relative_to((config.ids.4).0, 0.0)
            .set((config.ids.4).1, &mut master.ui);

            // Draw target value
            widget::Text::new(&format!("({})", value_target))
                .with_style(target_text_style)
                .right_from((config.ids.4).1, TELEMETRY_ARROW_SPACING_SIDES)
                .y_relative_to(config.ids.3, -1.0)
                .set(config.ids.5, &mut master.ui);
        }
        (Some(value_measured), None) => {
            // Draw measured value
            widget::Text::new(&value_measured)
                .with_style(value_text_style)
                .mid_left_with_margin_on(config.ids.1, TELEMETRY_WIDGET_PADDING_LEFT)
                .set(config.ids.3, &mut master.ui);
        }
        (None, Some(value_target)) => {
            // Draw target value
            widget::Text::new(&value_target)
                .with_style(value_text_style)
                .mid_left_with_margin_on(config.ids.1, TELEMETRY_WIDGET_PADDING_LEFT)
                .set(config.ids.5, &mut master.ui);
        }
        _ => {}
    }

    if let Some(unit_id) = config.ids.6 {
        // Initiate text sub-style for target value
        let mut unit_text_style = conrod_core::widget::primitive::text::Style::default();

        unit_text_style.font_id = Some(Some(master.fonts.regular));
        unit_text_style.color = Some(color::WHITE.with_alpha(0.35));
        unit_text_style.font_size = Some(TELEMETRY_WIDGET_UNIT_FONT_SIZE);

        // Create unit text
        widget::Text::new(&config.unit)
            .with_style(unit_text_style)
            .bottom_left_with_margins_on(
                config.ids.1,
                TELEMETRY_WIDGET_UNIT_PADDING_BOTTOM_TOP,
                TELEMETRY_WIDGET_PADDING_LEFT,
            )
            .set(unit_id, &mut master.ui);
    }

    TELEMETRY_WIDGET_BOTTOM_SIZE_WIDTH
}
