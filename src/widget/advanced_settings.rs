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

use telemetry::structures::{DataSnapshot, MachineStateSnapshot};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::utilities::{
    parse::{parse_non_empty_number_to_string, parse_optional_number_to_string},
    units::convert_sub_ppm_to_ppm,
};
use crate::APP_CONTEXT;

const CONTROL_UI_VERSION: &str = env!("CARGO_PKG_VERSION");

pub struct Config<'a> {
    pub width: f64,
    pub height: f64,

    pub last_tick: Option<u64>,
    pub machine_snapshot: &'a MachineStateSnapshot,
    pub data_snapshot: Option<&'a DataSnapshot>,

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
        // Telemetry version
        (
            "telemetry-version",
            &parse_non_empty_number_to_string(config.machine_snapshot.telemetry_version as usize),
        ),
        // Control UI version
        ("control-ui-version", CONTROL_UI_VERSION),
        (
            "control-ui-uptime-seconds",
            &parse_non_empty_number_to_string(APP_CONTEXT.start_time.elapsed().as_secs() as usize),
        ),
        // Firmware version
        ("firmware-version", &config.machine_snapshot.version),
        // Firmware device identifier (randomly generated at each boot of the motherboard)
        (
            "firmware-device-id",
            &config.machine_snapshot.device_id.to_string(),
        ),
        // Firmware uptime (in seconds), try to use the data snapshot systick which is refreshed \
        //   almost instantly when the machine is running, otherwise fallback on the less-often \
        //   refreshed data snapshot systick.
        (
            "firmware-uptime-seconds",
            &parse_optional_number_to_string(
                config
                    .last_tick
                    .map(|last_tick| (last_tick / 1000000) as usize),
            ),
        ),
        // Firmware CPU load (in percents)
        (
            "firmware-cpu-load-percent",
            &parse_optional_number_to_string(
                config.machine_snapshot.cpu_load.map(|value| value as usize),
            ),
        ),
        // Ventilation cycles count (ie. total number of respiratory cycles since system started)
        (
            "ventilation-cycles-count",
            &parse_non_empty_number_to_string(config.machine_snapshot.cycle as usize),
        ),
        // Time spent since the beginning of the current respiratory cycle (in milliseconds)
        (
            "ventilation-cycle-milliseconds",
            &parse_optional_number_to_string(
                config
                    .data_snapshot
                    .map(|data| (data.centile * 10) as usize),
            ),
        ),
        // Ventilation phase (ie. current respiration phase, called 'phase' internally)
        (
            "ventilation-phase",
            &config
                .data_snapshot
                .map(|data| format!("{:?}", data.phase))
                .unwrap_or_else(|| "".to_string()),
        ),
        // Pinch valve angle (inhale circuit pinch valve)
        (
            "pinch-angle-inhale-degrees",
            &parse_optional_number_to_string(
                config
                    .data_snapshot
                    .map(|data| data.blower_valve_position as usize),
            ),
        ),
        // Pinch valve angle (exhale circuit pinch valve)
        (
            "pinch-angle-exhale-degrees",
            &parse_optional_number_to_string(
                config
                    .data_snapshot
                    .map(|data| data.patient_valve_position as usize),
            ),
        ),
        // Blower speed (in PPM, ie. command sent to the ESC; the telemetry protocol calls this \
        //   RPM, although what is really sent is a PPM value divided by 10 as to fit on 1 octet, \
        //   where the final ESC PPM value goes from 300 to 1800).
        (
            "blower-speed-throttle-ppm",
            &parse_optional_number_to_string(
                config
                    .data_snapshot
                    .map(|data| convert_sub_ppm_to_ppm(data.blower_rpm)),
            ),
        ),
        // Battery level (in volts)
        (
            "battery-level-volts",
            &parse_optional_number_to_string(
                config.data_snapshot.map(|data| data.battery_level as usize),
            ),
        ),
    ];

    // Append lines
    lines(master, config, &line_data);

    0 as _
}

fn lines<'a>(master: &mut ControlWidget<'a>, config: Config, line_data: &[(&str, &str)]) {
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
}
