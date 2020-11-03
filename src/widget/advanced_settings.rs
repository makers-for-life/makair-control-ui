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
    let line_data: [(&str, String); ADVANCED_SETTINGS_LINES_COUNT] = [
        // TODO: populate w/ real values + label translations / or not?
        ("motherboard-serial", "MFL-02X".to_string()),
        ("telemetry-version", "0.0.0".to_string()),
        ("control-ui-version", "0.0.0".to_string()),
        ("control-ui-uptime", "2 minutes".to_string()),
        ("firmware-version", "0.0.0".to_string()),
        ("firmware-target", "development".to_string()),
        ("runtime-uptime", "3 minutes".to_string()),
        ("runtime-cycles", "214 cycles".to_string()),
        ("runtime-phase", "exhale".to_string()),
        ("pinch-angle-inhale", "43.2 deg".to_string()),
        ("pinch-angle-exhale", "2.5 deg".to_string()),
        ("blower-rpm", "2091 rpm".to_string()),
        ("battery-voltage", "27V".to_string()),
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
        widget::Text::new(&line_data[index].1)
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
