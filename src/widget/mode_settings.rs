// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};
use makair_telemetry::structures::VentilationMode;

use crate::chip::settings::mode::{SettingsMode, SettingsModeGroupTab};
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::locale::modes::{
    class_to_locale as mode_class_to_locale, group_tab_to_locale as mode_group_tab_to_locale,
    kind_to_locale as mode_kind_to_locale,
};
use crate::utilities::units::{convert_cl_to_ml, convert_mmh2o_to_cmh2o, ConvertMode};
use crate::APP_I18N;

const SELECTOR_BORDER_COLOR: Color = Color::Rgba(81.0 / 255.0, 81.0 / 255.0, 81.0 / 255.0, 1.0);

const SELECTOR_COLOR_DEFAULT: Color = Color::Rgba(0.0, 0.0, 0.0, 0.975);
const SELECTOR_COLOR_SELECTED: Color = Color::Rgba(26.0 / 255.0, 26.0 / 255.0, 26.0 / 255.0, 1.0);

const SELECTOR_TEXT_COLOR_CHANGED: Color = Color::Rgba(110.0 / 255.0, 191.0 / 255.0, 1.0, 1.0);

type FieldWidgetIds = (
    WidgetId,
    WidgetId,
    WidgetId,
    WidgetId,
    WidgetId,
    WidgetId,
    WidgetId,
);

pub struct Config<'a> {
    pub width: f64,
    pub height: f64,

    pub mode_settings: &'a SettingsMode,

    pub container_parent: WidgetId,
    pub container_widget: WidgetId,

    pub selector_wrapper: WidgetId,
    pub selector_tabs: [WidgetId; MODE_SETTINGS_SELECTOR_TABS_COUNT],
    pub selector_texts: [WidgetId; MODE_SETTINGS_SELECTOR_TABS_COUNT],

    pub field_pressure_inspiratory_ids: FieldWidgetIds,
    pub field_pressure_expiratory_ids: FieldWidgetIds,
    pub field_inspiratory_duration_ids: FieldWidgetIds,
    pub field_time_inspiratory_minimum_ids: FieldWidgetIds,
    pub field_time_inspiratory_maximum_ids: FieldWidgetIds,
    pub field_cycles_per_minute_ids: FieldWidgetIds,
    pub field_tidal_volume_ids: FieldWidgetIds,
    pub field_inspiratory_flow_ids: FieldWidgetIds,
    pub field_plateau_duration_ids: FieldWidgetIds,
    pub field_trigger_offset_ids: FieldWidgetIds,
    pub field_trigger_expiratory_ids: FieldWidgetIds,

    pub field_alarm_threshold_low_inspiratory_minute_volume_ids: FieldWidgetIds,
    pub field_alarm_threshold_high_inspiratory_minute_volume_ids: FieldWidgetIds,
    pub field_alarm_threshold_low_expiratory_minute_volume_ids: FieldWidgetIds,
    pub field_alarm_threshold_high_expiratory_minute_volume_ids: FieldWidgetIds,
    pub field_alarm_threshold_low_respiratory_rate_ids: FieldWidgetIds,
    pub field_alarm_threshold_high_respiratory_rate_ids: FieldWidgetIds,
    pub field_alarm_threshold_low_tidal_volume_ids: FieldWidgetIds,
    pub field_alarm_threshold_high_tidal_volume_ids: FieldWidgetIds,
    pub field_alarm_threshold_leak_ids: FieldWidgetIds,
    pub field_alarm_threshold_peak_pressure_ids: FieldWidgetIds,

    pub group_wrapper: WidgetId,
    pub content_wrapper: WidgetId,
    pub form_wrapper: WidgetId,

    pub group_tab_buttons: [WidgetId; MODE_SETTINGS_GROUP_TABS_COUNT],
    pub group_tab_texts: [WidgetId; MODE_SETTINGS_GROUP_TABS_COUNT],
}

struct Field {
    label_text: String,
    value_text: String,
    ids: FieldWidgetIds,
}

struct FieldValues {
    current: usize,
    live: usize,
    draft: Option<usize>,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create container
    gen_widget_container!(
        master,
        container_id: config.container_widget,
        color: color::TRANSPARENT,
        width: config.width,
        height: config.height,
        positions: top_left_of[
            config.container_parent,
        ]
    );

    // Append contents
    selector(master, &config);
    content(master, &config);

    0 as _
}

fn selector<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Acquire selected mode
    let mode = selected_mode(config);

    // Pre-calculate sizes
    let tab_width = config.width / MODE_SETTINGS_SELECTOR_TABS_COUNT as f64;

    // Append selector wrapper
    gen_widget_container!(
        master,
        container_id: config.selector_wrapper,
        color: SELECTOR_BORDER_COLOR,
        width: config.width,
        height: MODE_SETTINGS_SELECTOR_TABS_HEIGHT,
        positions: top_left_with_margins_on[
            config.container_widget, -MODE_SETTINGS_MODAL_PADDING, -MODE_SETTINGS_MODAL_PADDING,
        ]
    );

    // Append selector tabs
    for index in 0..MODE_SETTINGS_SELECTOR_TABS_COUNT {
        let index_mode = tab_index_to_mode(index);

        // Create rectangle (selected if index mode matches ongoing mode)
        let (rectangle_color, rectangle_height_offset, rectangle_width_offset, selection_offset) =
            if Some(mode) == index_mode {
                (
                    SELECTOR_COLOR_SELECTED,
                    0.0,
                    0.0,
                    if index == 0 { 0.0 } else { 1.0 },
                )
            } else {
                (
                    SELECTOR_COLOR_DEFAULT,
                    1.0,
                    if index == 0 { 0.0 } else { 1.0 },
                    0.0,
                )
            };

        widget::rectangle::Rectangle::fill_with(
            [
                tab_width - rectangle_width_offset - selection_offset,
                MODE_SETTINGS_SELECTOR_TABS_HEIGHT - rectangle_height_offset,
            ],
            rectangle_color,
        )
        .top_left_with_margins_on(
            config.selector_wrapper,
            0.0,
            rectangle_width_offset + selection_offset + index as f64 * tab_width,
        )
        .set(config.selector_tabs[index], &mut master.ui);

        // Append text?
        if let Some(index_mode) = index_mode {
            let is_selected = mode == index_mode;

            // Generate text style
            let mut text_style = widget::text::Style::default();

            text_style.font_id = Some(Some(master.fonts.bold));
            text_style.color = Some(if is_selected && mode != config.mode_settings.live.mode {
                SELECTOR_TEXT_COLOR_CHANGED
            } else {
                color::WHITE
            });
            text_style.font_size = Some(14);

            // Append text
            // Notice: the text Y alignment must be adjusted in a super-dirty way, using a \
            //   demi-pixel fix, when selected. This is done to avoid the text from jumping up \
            //   when the user taps on a tab.
            widget::Text::new(&format!(
                "{} {}",
                mode_class_to_locale(index_mode.class()),
                mode_kind_to_locale(index_mode.kind())
            ))
            .with_style(text_style)
            .middle_of(config.selector_tabs[index])
            .y_relative(if is_selected { 2.5 } else { 2.0 })
            .set(config.selector_texts[index], &mut master.ui);
        }
    }
}

fn content<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    let size = (
        config.width - (2.0 * MODE_SETTINGS_MODAL_PADDING),
        config.height - MODE_SETTINGS_SELECTOR_TABS_HEIGHT - MODE_SETTINGS_MODAL_PADDING,
    );

    // Create content wrapper
    gen_widget_container!(
        master,
        container_id: config.content_wrapper,
        color: color::TRANSPARENT,
        width: size.0,
        height: size.1,
        positions: top_left_with_margins_on[
            config.container_widget, MODE_SETTINGS_SELECTOR_TABS_HEIGHT, 0.0,
        ]
    );

    group(master, config, size);
    form(master, config, size);
}

fn group<'a>(master: &mut ControlWidget<'a>, config: &Config, parent_size: (f64, f64)) {
    // Create group wrapper
    gen_widget_group!(
        master,
        parent_id: config.content_wrapper,
        group_id: config.group_wrapper,
        height: parent_size.1,
    );

    // Render all group tabs
    for index in 0..MODE_SETTINGS_GROUP_TABS_COUNT {
        group_tab(
            master,
            config,
            SettingsModeGroupTab::from_index(index).expect("invalid group tab index"),
            index,
        );
    }
}

fn group_tab<'a>(
    master: &mut ControlWidget<'a>,
    config: &Config,
    tab: SettingsModeGroupTab,
    index: usize,
) {
    gen_widget_group_tab!(
        master,
        group_id: config.group_wrapper,
        button_ids: config.group_tab_buttons,
        text_ids: config.group_tab_texts,
        tab_active: config.mode_settings.group,
        tab_current: tab,
        text_fn: mode_group_tab_to_locale,
        index: index,
    );
}

fn form<'a>(master: &mut ControlWidget<'a>, config: &Config, parent_size: (f64, f64)) {
    // Acquire selected mode
    let mode = selected_mode(config);

    // Compute total tabs width
    let tabs_total_width = MODAL_GROUP_TABS_WIDTH + MODAL_GROUP_TABS_MARGIN_RIGHT;

    // Create form wrapper
    gen_widget_container!(
        master,
        container_id: config.form_wrapper,
        color: color::TRANSPARENT,
        width: parent_size.0 - tabs_total_width,
        height: parent_size.1,
        positions: top_right_of[
            config.content_wrapper,
        ]
    );

    // Append form depending on current ventilation mode
    match mode {
        VentilationMode::PC_CMV => form_pc_cmv(master, config),
        VentilationMode::PC_AC => form_pc_ac(master, config),
        VentilationMode::PC_VSAI => form_pc_vsai(master, config),
        VentilationMode::VC_CMV => form_vc_cmv(master, config),
        VentilationMode::VC_AC => form_vc_ac(master, config),
    }
}

fn form_pc_cmv<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    match config.mode_settings.group {
        SettingsModeGroupTab::General => {
            field_pressure_inspiratory(0, master, config);
            field_pressure_expiratory(1, master, config);
            field_duration_inspiration(2, master, config);
            field_cycles_per_minute(3, master, config);
        }
        SettingsModeGroupTab::Alarms => {
            field_alarm_threshold_low_inspiratory_minute_volume(0, master, config);
            field_alarm_threshold_high_inspiratory_minute_volume(1, master, config);
            field_alarm_threshold_low_expiratory_minute_volume(2, master, config);
            field_alarm_threshold_high_expiratory_minute_volume(3, master, config);
            field_alarm_threshold_low_tidal_volume(4, master, config);
            field_alarm_threshold_high_tidal_volume(5, master, config);
        }
    }
}

fn form_pc_ac<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    match config.mode_settings.group {
        SettingsModeGroupTab::General => {
            field_pressure_inspiratory(0, master, config);
            field_pressure_expiratory(1, master, config);
            field_duration_inspiration(2, master, config);
            field_cycles_per_minute(3, master, config);
            field_trigger_offset(4, master, config);
        }
        SettingsModeGroupTab::Alarms => {
            field_alarm_threshold_low_inspiratory_minute_volume(0, master, config);
            field_alarm_threshold_high_inspiratory_minute_volume(1, master, config);
            field_alarm_threshold_low_expiratory_minute_volume(2, master, config);
            field_alarm_threshold_high_expiratory_minute_volume(3, master, config);
            field_alarm_threshold_low_respiratory_rate(4, master, config);
            field_alarm_threshold_high_respiratory_rate(5, master, config);
            field_alarm_threshold_low_tidal_volume(6, master, config);
            field_alarm_threshold_high_tidal_volume(7, master, config);
        }
    }
}

fn form_pc_vsai<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    match config.mode_settings.group {
        SettingsModeGroupTab::General => {
            field_pressure_inspiratory(0, master, config);
            field_pressure_expiratory(1, master, config);
            field_time_inspiratory_minimum(2, master, config);
            field_time_inspiratory_maximum(3, master, config);
            field_cycles_per_minute(4, master, config);
            field_trigger_offset(5, master, config);
            field_trigger_expiratory(6, master, config);
        }
        SettingsModeGroupTab::Alarms => {
            field_alarm_threshold_low_inspiratory_minute_volume(0, master, config);
            field_alarm_threshold_high_inspiratory_minute_volume(1, master, config);
            field_alarm_threshold_low_expiratory_minute_volume(2, master, config);
            field_alarm_threshold_high_expiratory_minute_volume(3, master, config);
            field_alarm_threshold_low_respiratory_rate(4, master, config);
            field_alarm_threshold_high_respiratory_rate(5, master, config);
            field_alarm_threshold_low_tidal_volume(6, master, config);
            field_alarm_threshold_high_tidal_volume(7, master, config);
        }
    }
}

fn form_vc_cmv<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    match config.mode_settings.group {
        SettingsModeGroupTab::General => {
            field_tidal_volume(0, master, config);
            field_pressure_expiratory(1, master, config);
            field_cycles_per_minute(2, master, config);
            field_duration_plateau(3, master, config);
            field_inspiratory_flow(4, master, config);
        }
        SettingsModeGroupTab::Alarms => {
            field_alarm_threshold_leak(0, master, config);
            field_alarm_threshold_peak_pressure(1, master, config);
        }
    }
}

fn form_vc_ac<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    match config.mode_settings.group {
        SettingsModeGroupTab::General => {
            field_tidal_volume(0, master, config);
            field_pressure_expiratory(1, master, config);
            field_cycles_per_minute(2, master, config);
            field_duration_plateau(3, master, config);
            field_trigger_offset(4, master, config);
            field_inspiratory_flow(5, master, config);
        }
        SettingsModeGroupTab::Alarms => {
            field_alarm_threshold_leak(0, master, config);
            field_alarm_threshold_peak_pressure(1, master, config);
            field_alarm_threshold_low_inspiratory_minute_volume(2, master, config);
            field_alarm_threshold_high_inspiratory_minute_volume(3, master, config);
            field_alarm_threshold_low_expiratory_minute_volume(4, master, config);
            field_alarm_threshold_high_expiratory_minute_volume(5, master, config);
            field_alarm_threshold_low_respiratory_rate(6, master, config);
            field_alarm_threshold_high_respiratory_rate(7, master, config);
        }
    }
}

fn field_pressure_inspiratory<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    let field_values = gen_widget_mode_field_values!(config, pressure_plateau);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-pressure-inspiratory"),
            value_text: format!(
                "{} {}",
                convert_mmh2o_to_cmh2o(ConvertMode::Rounded, field_values.current as f64),
                APP_I18N.t("telemetry-unit-cmh2o")
            ),
            ids: config.field_pressure_inspiratory_ids,
        },
        field_values,
    )
}

fn field_pressure_expiratory<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    let field_values = gen_widget_mode_field_values!(config, pressure_expiratory);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-pressure-expiratory"),
            value_text: format!(
                "{} {}",
                convert_mmh2o_to_cmh2o(ConvertMode::Rounded, field_values.current as f64),
                APP_I18N.t("telemetry-unit-cmh2o")
            ),
            ids: config.field_pressure_expiratory_ids,
        },
        field_values,
    )
}

fn field_time_inspiratory_minimum<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
) {
    let field_values = gen_widget_mode_field_values!(config, inspiratory_time_minimum);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-time-inspiratory-minimum"),
            value_text: format!(
                "{} {}",
                field_values.current,
                APP_I18N.t("telemetry-unit-milliseconds")
            ),
            ids: config.field_time_inspiratory_minimum_ids,
        },
        field_values,
    )
}

fn field_time_inspiratory_maximum<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
) {
    let field_values = gen_widget_mode_field_values!(config, inspiratory_time_maximum);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-time-inspiratory-maximum"),
            value_text: format!(
                "{} {}",
                field_values.current,
                APP_I18N.t("telemetry-unit-milliseconds")
            ),
            ids: config.field_time_inspiratory_maximum_ids,
        },
        field_values,
    )
}

fn field_cycles_per_minute<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    let field_values = gen_widget_mode_field_values!(config, cycles_per_minute);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-cycles-per-minute"),
            value_text: format!(
                "{}{}",
                field_values.current,
                APP_I18N.t("telemetry-unit-per-minute")
            ),
            ids: config.field_cycles_per_minute_ids,
        },
        field_values,
    )
}

fn field_tidal_volume<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    let field_values = gen_widget_mode_field_values!(config, volume_tidal);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-tidal-volume"),
            value_text: format!(
                "{} {}",
                field_values.current,
                APP_I18N.t("telemetry-unit-milliliters")
            ),
            ids: config.field_tidal_volume_ids,
        },
        field_values,
    )
}

fn field_inspiratory_flow<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    let field_values = gen_widget_mode_field_values!(config, flow_inspiration);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-flow-inspiratory"),
            value_text: format!(
                "{} {}",
                field_values.current,
                APP_I18N.t("telemetry-unit-lpm")
            ),
            ids: config.field_inspiratory_flow_ids,
        },
        field_values,
    )
}

fn field_duration_inspiration<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    let field_values = gen_widget_mode_field_values!(config, duration_inspiration);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-time-inspiratory"),
            value_text: format!(
                "{} {}",
                field_values.current,
                APP_I18N.t("telemetry-unit-milliseconds")
            ),
            ids: config.field_inspiratory_duration_ids,
        },
        field_values,
    )
}

fn field_duration_plateau<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    let field_values = gen_widget_mode_field_values!(config, duration_plateau);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-plateau-duration"),
            value_text: format!(
                "{} {}",
                field_values.current,
                APP_I18N.t("telemetry-unit-milliseconds")
            ),
            ids: config.field_plateau_duration_ids,
        },
        field_values,
    )
}

fn field_trigger_offset<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    let field_values = gen_widget_mode_field_values!(config, trigger_inspiratory_offset);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-trigger-offset"),
            value_text: format!(
                "{:.1} {}",
                convert_mmh2o_to_cmh2o(ConvertMode::WithDecimals, field_values.current as f64),
                APP_I18N.t("telemetry-unit-cmh2o")
            ),
            ids: config.field_trigger_offset_ids,
        },
        field_values,
    )
}

fn field_trigger_expiratory<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    let field_values = gen_widget_mode_field_values!(config, trigger_expiratory_flow);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-trigger-expiratory"),
            value_text: format!("{}%", field_values.current),
            ids: config.field_trigger_expiratory_ids,
        },
        field_values,
    )
}

fn field_alarm_threshold_low_inspiratory_minute_volume<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
) {
    let field_values =
        gen_widget_mode_field_values!(config, alarm_threshold_low_inspiratory_minute_volume);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-alarm-low-inspiratory-minute-volume"),
            value_text: format!(
                "{} {}",
                field_values.current,
                APP_I18N.t("telemetry-unit-lpm")
            ),
            ids: config.field_alarm_threshold_low_inspiratory_minute_volume_ids,
        },
        field_values,
    )
}

fn field_alarm_threshold_high_inspiratory_minute_volume<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
) {
    let field_values =
        gen_widget_mode_field_values!(config, alarm_threshold_high_inspiratory_minute_volume);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-alarm-high-inspiratory-minute-volume"),
            value_text: format!(
                "{} {}",
                field_values.current,
                APP_I18N.t("telemetry-unit-lpm")
            ),
            ids: config.field_alarm_threshold_high_inspiratory_minute_volume_ids,
        },
        field_values,
    )
}

fn field_alarm_threshold_low_expiratory_minute_volume<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
) {
    let field_values =
        gen_widget_mode_field_values!(config, alarm_threshold_low_expiratory_minute_volume);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-alarm-low-expiratory-minute-volume"),
            value_text: format!(
                "{} {}",
                field_values.current,
                APP_I18N.t("telemetry-unit-lpm")
            ),
            ids: config.field_alarm_threshold_low_expiratory_minute_volume_ids,
        },
        field_values,
    )
}

fn field_alarm_threshold_high_expiratory_minute_volume<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
) {
    let field_values =
        gen_widget_mode_field_values!(config, alarm_threshold_high_expiratory_minute_volume);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-alarm-high-expiratory-minute-volume"),
            value_text: format!(
                "{} {}",
                field_values.current,
                APP_I18N.t("telemetry-unit-lpm")
            ),
            ids: config.field_alarm_threshold_high_expiratory_minute_volume_ids,
        },
        field_values,
    )
}

fn field_alarm_threshold_low_respiratory_rate<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
) {
    let field_values = gen_widget_mode_field_values!(config, alarm_threshold_low_respiratory_rate);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-alarm-low-respiratory-rate"),
            value_text: format!(
                "{}{}",
                field_values.current,
                APP_I18N.t("telemetry-unit-per-minute")
            ),
            ids: config.field_alarm_threshold_low_respiratory_rate_ids,
        },
        field_values,
    )
}

fn field_alarm_threshold_high_respiratory_rate<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
) {
    let field_values = gen_widget_mode_field_values!(config, alarm_threshold_high_respiratory_rate);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-alarm-high-respiratory-rate"),
            value_text: format!(
                "{}{}",
                field_values.current,
                APP_I18N.t("telemetry-unit-per-minute")
            ),
            ids: config.field_alarm_threshold_high_respiratory_rate_ids,
        },
        field_values,
    )
}

fn field_alarm_threshold_low_tidal_volume<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
) {
    let field_values = gen_widget_mode_field_values!(config, alarm_threshold_low_tidal_volume);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-alarm-low-tidal-volume"),
            value_text: format!(
                "{} {}",
                field_values.current,
                APP_I18N.t("telemetry-unit-milliliters")
            ),
            ids: config.field_alarm_threshold_low_tidal_volume_ids,
        },
        field_values,
    )
}

fn field_alarm_threshold_high_tidal_volume<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
) {
    let field_values = gen_widget_mode_field_values!(config, alarm_threshold_high_tidal_volume);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-alarm-high-tidal-volume"),
            value_text: format!(
                "{} {}",
                field_values.current,
                APP_I18N.t("telemetry-unit-milliliters")
            ),
            ids: config.field_alarm_threshold_high_tidal_volume_ids,
        },
        field_values,
    )
}

fn field_alarm_threshold_leak<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    let field_values = gen_widget_mode_field_values!(config, alarm_threshold_leak);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-alarm-leak"),
            value_text: format!(
                "{} {}",
                convert_cl_to_ml(ConvertMode::Rounded, field_values.current as f64),
                APP_I18N.t("telemetry-unit-mlpm")
            ),
            ids: config.field_alarm_threshold_leak_ids,
        },
        field_values,
    )
}

fn field_alarm_threshold_peak_pressure<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
) {
    let field_values = gen_widget_mode_field_values!(config, alarm_threshold_peak_pressure);

    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-alarm-peak-pressure"),
            value_text: format!(
                "{} {}",
                convert_mmh2o_to_cmh2o(ConvertMode::Rounded, field_values.current as f64),
                APP_I18N.t("telemetry-unit-cmh2o")
            ),
            ids: config.field_alarm_threshold_peak_pressure_ids,
        },
        field_values,
    )
}

fn draw_field<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
    field: Field,
    values: FieldValues,
) {
    // Check if value has changed (ie. draft value is different than live value)
    let has_changed = if let Some(draft) = values.draft {
        draft != values.live
    } else {
        false
    };

    // Generate label
    gen_widget_label_form!(
        master,
        text_id: field.ids.0,
        value: &field.label_text,
        positions: top_left_with_margins_on[
            config.form_wrapper, index as f64 * MODE_SETTINGS_MODAL_FORM_FIELD_HEIGHT_PADDED, 0.0,
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
        changed: has_changed,
        positions: top_left_with_margins_on[
            field.ids.0,
            MODAL_BUTTON_NAVIGATE_LEFT_ALIGN_TOP,
            MODE_SETTINGS_MODAL_FORM_PADDING_LEFT,
        ]
    );
}

fn tab_index_to_mode(index: usize) -> Option<VentilationMode> {
    match index {
        0 => Some(VentilationMode::PC_CMV),
        1 => Some(VentilationMode::PC_AC),
        2 => Some(VentilationMode::PC_VSAI),
        3 => Some(VentilationMode::VC_CMV),
        4 => Some(VentilationMode::VC_AC),
        _ => None,
    }
}

fn selected_mode(config: &Config) -> VentilationMode {
    if let Some(ref draft) = config.mode_settings.draft {
        draft.mode
    } else {
        config.mode_settings.live.mode
    }
}
