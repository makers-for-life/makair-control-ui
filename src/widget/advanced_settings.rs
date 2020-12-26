// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use chrono::offset::Local;
use conrod_core::{
    color::{self, Color},
    widget::{self, id::List as WidgetList, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use telemetry::structures::{DataSnapshot, MachineStateSnapshot};

use crate::chip::settings::advanced::{SettingsAdvanced, SettingsAdvancedGroupTab};
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::locale::advanced::group_tab_to_locale as advanced_group_tab_to_locale;
use crate::utilities::{
    parse::{parse_non_empty_number_to_string, parse_optional_number_to_string},
    units::convert_sub_ppm_to_ppm,
};
use crate::{APP_CONTEXT, APP_I18N};

type FieldWidgetIds = (
    WidgetId,
    WidgetId,
    WidgetId,
    WidgetId,
    WidgetId,
    WidgetId,
    WidgetId,
);

type TextWidgetIds = (WidgetId, WidgetId);

pub struct Config<'a> {
    pub width: f64,
    pub height: f64,

    pub advanced_settings: &'a SettingsAdvanced,

    pub last_tick: Option<u64>,
    pub machine_snapshot: &'a MachineStateSnapshot,
    pub data_snapshot: Option<&'a DataSnapshot>,

    pub advanced_container_parent: WidgetId,
    pub advanced_container_widget: WidgetId,
    pub advanced_container_line_labels: &'a WidgetList,
    pub advanced_container_line_values: &'a WidgetList,

    pub advanced_group_wrapper: WidgetId,
    pub advanced_form_wrapper: WidgetId,

    pub advanced_group_tab_buttons: [WidgetId; ADVANCED_SETTINGS_GROUP_TABS_COUNT],
    pub advanced_group_tab_texts: [WidgetId; ADVANCED_SETTINGS_GROUP_TABS_COUNT],

    pub field_locale_ids: FieldWidgetIds,

    pub text_date_ids: TextWidgetIds,
    pub text_time_ids: TextWidgetIds,
    pub text_timezone_ids: TextWidgetIds,
}

struct Field {
    label_text: String,
    value_text: String,
    ids: FieldWidgetIds,
}

struct Text {
    label_text: String,
    value_text: String,
    ids: TextWidgetIds,
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

    // Compute inner size
    let size = (
        config.width - (2.0 * ADVANCED_SETTINGS_MODAL_PADDING),
        config.height - ADVANCED_SETTINGS_MODAL_PADDING,
    );

    // Append contents
    group(master, &config, size);
    form(master, &config, size);

    0 as _
}

fn group<'a>(master: &mut ControlWidget<'a>, config: &Config, parent_size: (f64, f64)) {
    // Create group wrapper
    gen_widget_group!(
        master,
        parent_id: config.advanced_container_widget,
        group_id: config.advanced_group_wrapper,
        height: parent_size.1,
    );

    // Render all group tabs
    for index in 0..ADVANCED_SETTINGS_GROUP_TABS_COUNT {
        group_tab(
            master,
            config,
            SettingsAdvancedGroupTab::from_index(index).expect("invalid group tab index"),
            index,
        );
    }
}

fn group_tab<'a>(
    master: &mut ControlWidget<'a>,
    config: &Config,
    tab: SettingsAdvancedGroupTab,
    index: usize,
) {
    gen_widget_group_tab!(
        master,
        group_id: config.advanced_group_wrapper,
        button_ids: config.advanced_group_tab_buttons,
        text_ids: config.advanced_group_tab_texts,
        tab_active: config.advanced_settings.group,
        tab_current: tab,
        text_fn: advanced_group_tab_to_locale,
        index: index,
    );
}

fn form<'a>(master: &mut ControlWidget<'a>, config: &Config, parent_size: (f64, f64)) {
    // Compute total tabs width
    let tabs_total_width = MODAL_GROUP_TABS_WIDTH + MODAL_GROUP_TABS_MARGIN_RIGHT;

    // Create form wrapper
    gen_widget_container!(
        master,
        container_id: config.advanced_form_wrapper,
        color: color::TRANSPARENT,
        width: parent_size.0 - tabs_total_width,
        height: parent_size.1,
        positions: top_left_with_margins_on[
            config.advanced_container_widget, 0.0, tabs_total_width,
        ]
    );

    // Append form depending on current group
    match config.advanced_settings.group {
        SettingsAdvancedGroupTab::Statistics => form_statistics(master, config),
        SettingsAdvancedGroupTab::Settings => form_settings(master, config),
    }
}

fn form_statistics<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate line data
    let line_data: [(&str, &str); ADVANCED_SETTINGS_LINES_COUNT] = [
        // Telemetry version
        (
            "telemetry-version",
            &parse_non_empty_number_to_string(config.machine_snapshot.telemetry_version as usize),
        ),
        // Control UI version
        ("control-ui-version", RUNTIME_VERSION),
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
        // Power input (in volts)
        (
            "power-input-volts",
            &parse_optional_number_to_string(
                config.data_snapshot.map(|data| data.battery_level as usize),
            ),
        ),
    ];

    // Append form lines
    form_statistics_lines(master, config, &line_data);
}

fn form_statistics_lines<'a>(
    master: &mut ControlWidget<'a>,
    config: &Config,
    line_data: &[(&str, &str)],
) {
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
                    config.advanced_form_wrapper,
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

fn form_settings<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    let now = Local::now();

    draw_field(
        0,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-advanced-locale"),
            value_text: config.advanced_settings.locale.to_name().to_string(),
            ids: config.field_locale_ids,
        },
    );

    draw_text(
        1,
        master,
        config,
        Text {
            label_text: APP_I18N.t("modal-advanced-date"),
            value_text: now.format("%d/%m/%Y").to_string(),
            ids: config.text_date_ids,
        },
    );

    draw_text(
        2,
        master,
        config,
        Text {
            label_text: APP_I18N.t("modal-advanced-time"),
            value_text: now.format("%I:%M:%S %p").to_string(),
            ids: config.text_time_ids,
        },
    );

    draw_text(
        3,
        master,
        config,
        Text {
            label_text: APP_I18N.t("modal-advanced-timezone"),
            value_text: now.format("UTC%:z").to_string(),
            ids: config.text_timezone_ids,
        },
    );
}

fn draw_field<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config, field: Field) {
    // Generate label
    gen_widget_label_form!(
        master,
        text_id: field.ids.0,
        value: &field.label_text,
        positions: top_left_with_margins_on[
            config.advanced_form_wrapper, index as f64 * ADVANCED_SETTINGS_MODAL_FORM_FIELD_HEIGHT_PADDED, 0.0,
        ]
    );

    // Generate navigation buttons
    gen_widget_button_navigate!(
        master,
        button_less_id: field.ids.5,
        button_less_text_id: field.ids.6,
        button_more_id: field.ids.3,
        button_more_text_id: field.ids.4,
        value_wrapper_id: field.ids.1,
        value_id: field.ids.2,
        value: &field.value_text,
        positions: top_left_with_margins_on[
            field.ids.0,
            -2.0,
            ADVANCED_SETTINGS_MODAL_FORM_PADDING_LEFT,
        ]
    );
}

fn draw_text<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config, text: Text) {
    // Generate label
    gen_widget_label_form!(
        master,
        text_id: text.ids.0,
        value: &text.label_text,
        positions: top_left_with_margins_on[
            config.advanced_form_wrapper, index as f64 * ADVANCED_SETTINGS_MODAL_FORM_FIELD_HEIGHT_PADDED, 0.0,
        ]
    );

    // Initialize text style for value
    let mut value_style = widget::text::Style::default();

    value_style.font_id = Some(Some(master.fonts.regular));
    value_style.color = Some(color::WHITE);
    value_style.font_size = Some(MODAL_TEXT_FONT_SIZE);

    // Create text for value
    widget::Text::new(&text.value_text)
        .with_style(value_style)
        .top_left_with_margins_on(text.ids.0, 0.0, ADVANCED_SETTINGS_MODAL_FORM_PADDING_LEFT)
        .set(text.ids.1, &mut master.ui);
}
