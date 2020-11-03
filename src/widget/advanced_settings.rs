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
    widget::{self, id::List as WidgetList, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use telemetry::structures::MachineStateSnapshot;

use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::utilities::parse::parse_non_empty_number_to_string;
use crate::APP_CONTEXT;

const CONTROL_UI_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Config<'a> {
    pub width: f64,
    pub height: f64,

    pub snapshot: &'a MachineStateSnapshot,

    pub advanced_container_parent: WidgetId,
    pub advanced_container_widget: WidgetId,
    pub advanced_container_line_labels: &'a WidgetList,
    pub advanced_container_line_values: &'a WidgetList,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create canvas
    gen_widget_container!(
        master,
        container_id: config.advanced_container_widget,
        color: color::TRANSPARENT,
        width: config.width,
        height: config.height,
        positions: top_left_of[
            config.advanced_container_parent,
        ]
    );

    // Generate line data
    let line_data: [(&str, &str); ADVANCED_SETTINGS_LINES_COUNT] = [
        (
            "telemetry-version",
            &parse_non_empty_number_to_string(config.snapshot.telemetry_version as usize),
        ),
        ("control-ui-version", CONTROL_UI_VERSION),
        (
            "control-ui-uptime-seconds",
            &parse_non_empty_number_to_string(APP_CONTEXT.start_time.elapsed().as_secs() as usize),
        ),
        ("firmware-version", &config.snapshot.version),
        ("firmware-target", ""), // TODO: BootMessage::mode
        ("runtime-device-id", &config.snapshot.device_id.to_string()),
        (
            "runtime-uptime-seconds",
            &parse_non_empty_number_to_string(config.snapshot.systick as usize / 1000000),
        ),
        (
            "runtime-cycles",
            &parse_non_empty_number_to_string(config.snapshot.cycle as usize),
        ),
        ("runtime-phase", ""),      // TODO: DataSnapshot::subphase
        ("pinch-angle-inhale", ""), // TODO: DataSnapshot::blower_valve_position
        ("pinch-angle-exhale", ""), // TODO: DataSnapshot::patient_valve_position
        ("blower-rpm", ""),         // TODO: DataSnapshot::blower_rpm
        ("battery-voltage", ""),    // TODO: DataSnapshot::battery_level
    ];

    // Append lines
    for (index, container_line) in config.advanced_container_line_labels.iter().enumerate() {
        let line_text = line_data[index].0;

        // Render line label (the positioning method varies if the line is the first one)
        if index == 0 {
            gen_widget_label_styled!(
                master,
                text_id: *container_line,
                value: line_text,
                font_size: ADVANCED_SETTINGS_LINE_FONT_SIZE,
                font_weight: bold,
                positions: top_left_of[
                    config.advanced_container_widget,
                ]
            );
        } else {
            gen_widget_label_styled!(
                master,
                text_id: *container_line,
                value: line_text,
                font_size: ADVANCED_SETTINGS_LINE_FONT_SIZE,
                font_weight: bold,
                positions: down_from[
                    config.advanced_container_line_labels[index - 1],
                    ADVANCED_SETTINGS_LINE_MARGIN_TOP,
                ]
            );
        }

        // Render line value
        let mut value_text_style = widget::text::Style::default();

        value_text_style.font_id = Some(Some(master.fonts.regular));
        value_text_style.color = Some(color::WHITE);
        value_text_style.font_size = Some(ADVANCED_SETTINGS_LINE_FONT_SIZE);

        // Create text
        widget::Text::new(if line_data[index].1.is_empty() {
            ADVANCED_SETTINGS_LINE_VALUE_EMPTY
        } else {
            &line_data[index].1
        })
        .with_style(value_text_style)
        .top_left_with_margins_on(
            *container_line,
            0.0,
            ADVANCED_SETTINGS_LINE_VALUE_PADDING_LEFT,
        )
        .set(config.advanced_container_line_values[index], &mut master.ui);
    }

    0 as _
}
