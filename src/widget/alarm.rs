// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{self, canvas},
    Colorable, Positionable, Sizeable, Widget,
};

use makair_telemetry::alarm::AlarmCode;
use makair_telemetry::structures::AlarmPriority;

use super::alarms;

use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::locale::alarms::description_to_locale as alarm_description_to_locale;

const ALARM_HIGH_CODE_COLOR: Color = Color::Rgba(1.0, 0.0 / 255.0, 3.0 / 255.0, 1.0);
const ALARM_MEDIUM_CODE_COLOR: Color = Color::Rgba(1.0, 135.0 / 255.0, 0.0, 1.0);
const ALARM_LOW_CODE_COLOR: Color = Color::Rgba(1.0, 195.0 / 255.0, 0.0, 1.0);

const ALARM_HIGH_MESSAGE_COLOR: Color = Color::Rgba(180.0 / 255.0, 24.0 / 255.0, 28.0 / 255.0, 1.0);
const ALARM_MEDIUM_MESSAGE_COLOR: Color = Color::Rgba(189.0 / 255.0, 93.0 / 255.0, 0.0, 1.0);
const ALARM_LOW_MESSAGE_COLOR: Color = Color::Rgba(174.0 / 255.0, 133.0 / 255.0, 0.0, 1.0);

pub fn render(
    master: &mut ControlWidget,
    config: &alarms::Config,

    alarm_code: AlarmCode,
    alarm_priority: &AlarmPriority,

    index: usize,
) {
    // Initialize style
    let mut style = canvas::Style::default();

    style.border = Some(0.0);
    style.border_color = Some(color::TRANSPARENT);
    style.color = Some(color::TRANSPARENT);

    // Compute top position for canvas
    let from_top = if index == 0 {
        DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL
    } else {
        DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL - 2.0
            + index as f64
                * (DISPLAY_ALARM_MESSAGE_HEIGHT + DISPLAY_ALARM_MESSAGE_SPACING_TOP_INNER)
    } + DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL_OFFSET;

    // Create canvas
    canvas::Canvas::new()
        .with_style(style)
        .y_place_on(
            config.container,
            conrod_core::position::Place::End(Some(from_top)),
        )
        .right_from(
            config.title_wrapper,
            DISPLAY_ALARM_CONTAINER_TITLE_TO_ALARM_HAS_ALARMS_SPACING,
        )
        .set(config.alarm_widgets[index], &mut master.ui);

    // Draw code & message
    code(master, &config, alarm_code, alarm_priority, index);
    message(master, &config, alarm_code, alarm_priority, index);
}

fn code_color(alarm_priority: &AlarmPriority) -> Color {
    // Map alarm code colors
    match alarm_priority {
        AlarmPriority::High => ALARM_HIGH_CODE_COLOR,
        AlarmPriority::Medium => ALARM_MEDIUM_CODE_COLOR,
        AlarmPriority::Low => ALARM_LOW_CODE_COLOR,
    }
}

fn message_color(alarm_priority: &AlarmPriority) -> Color {
    // Map alarm message colors
    match alarm_priority {
        AlarmPriority::High => ALARM_HIGH_MESSAGE_COLOR,
        AlarmPriority::Medium => ALARM_MEDIUM_MESSAGE_COLOR,
        AlarmPriority::Low => ALARM_LOW_MESSAGE_COLOR,
    }
}

fn code<'a>(
    master: &mut ControlWidget<'a>,
    config: &alarms::Config,
    alarm_code: AlarmCode,
    alarm_priority: &AlarmPriority,
    index: usize,
) {
    // Create canvas
    gen_widget_container!(
        master,
        container_id: config.alarm_codes_containers[index],
        color: code_color(alarm_priority),
        width: DISPLAY_ALARM_CODE_WIDTH,
        height: DISPLAY_ALARM_CODE_HEIGHT,
        positions: x_place_on[
            config.alarm_widgets[index],
            conrod_core::position::Place::Start(None),
        ]
    );

    // Insert text in canvas
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::WHITE);
    text_style.font_size = Some(DISPLAY_ALARM_CODE_FONT_SIZE);

    widget::text::Text::new(&format!("{}", alarm_code.code()))
        .with_style(text_style)
        .mid_top_with_margin_on(config.alarm_codes_containers[index], 2.0)
        .set(config.alarm_codes[index], &mut master.ui);
}

fn message<'a>(
    master: &mut ControlWidget<'a>,
    config: &alarms::Config,
    alarm_code: AlarmCode,
    alarm_priority: &AlarmPriority,
    index: usize,
) {
    // Create canvas
    gen_widget_container!(
        master,
        container_id: config.alarm_messages_containers[index],
        color: message_color(alarm_priority),
        width: DISPLAY_ALARM_MESSAGE_WIDTH,
        height: DISPLAY_ALARM_MESSAGE_HEIGHT,
        positions: x_place_on[
            config.alarm_widgets[index],
            conrod_core::position::Place::Start(Some(DISPLAY_ALARM_CODE_WIDTH)),
        ]
    );

    // Insert text in canvas
    widget::text::Text::new(&alarm_description_to_locale(alarm_code.description()))
        .color(color::WHITE)
        .font_size(DISPLAY_ALARM_MESSAGE_FONT_SIZE)
        .top_left_with_margins_on(config.alarm_messages_containers[index], 3.0, 8.0)
        .set(config.alarm_messages[index], &mut master.ui);
}
