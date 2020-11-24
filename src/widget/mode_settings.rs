// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};
use telemetry::structures::VentilationMode;

use crate::chip::settings::mode::SettingsMode;
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::locale::modes::{
    class_to_locale as mode_class_to_locale, kind_to_locale as mode_kind_to_locale,
};
use crate::utilities::units::{convert_mmh2o_to_cmh2o, ConvertMode};
use crate::APP_I18N;

const SELECTOR_BORDER_COLOR: Color = Color::Rgba(81.0 / 255.0, 81.0 / 255.0, 81.0 / 255.0, 1.0);
const SELECTOR_COLOR_DEFAULT: Color = Color::Rgba(0.0, 0.0, 0.0, 0.975);
const SELECTOR_COLOR_SELECTED: Color = Color::Rgba(26.0 / 255.0, 26.0 / 255.0, 26.0 / 255.0, 1.0);

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
    pub field_trigger_inspiratory_ids: FieldWidgetIds,
    pub field_trigger_expiratory_ids: FieldWidgetIds,

    pub form_wrapper: WidgetId,
}

struct Field {
    label_text: String,
    value_text: String,
    ids: FieldWidgetIds,
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

    // TODO: do not save values (mode + its options) right away once it gets changed
    // TODO: add save button

    // Append contents
    selector(master, &config);
    form(master, &config);

    0 as _
}

fn selector<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Pre-calculate sizes and styles
    let tab_width = config.width / MODE_SETTINGS_SELECTOR_TABS_COUNT as f64;

    let mut text_style = widget::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::WHITE);
    text_style.font_size = Some(14);

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
            if Some(config.mode_settings.mode) == index_mode {
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
            widget::Text::new(&format!(
                "{} {}",
                mode_class_to_locale(index_mode.class()),
                mode_kind_to_locale(index_mode.kind())
            ))
            .with_style(text_style)
            .middle_of(config.selector_tabs[index])
            .y_relative(2.0)
            .set(config.selector_texts[index], &mut master.ui);
        }
    }
}

fn form<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // TODO: option becomes eg. blue when it is changed and not saved
    // TODO: implement alarm settings (might need to make some space for them)

    // Create form wrapper
    gen_widget_container!(
        master,
        container_id: config.form_wrapper,
        color: color::TRANSPARENT,
        width: config.width - (2.0 * MODE_SETTINGS_MODAL_PADDING),
        height: config.height - MODE_SETTINGS_SELECTOR_TABS_HEIGHT - MODE_SETTINGS_MODAL_PADDING,
        positions: top_left_with_margins_on[
            config.container_widget, MODE_SETTINGS_SELECTOR_TABS_HEIGHT, 0.0,
        ]
    );

    // Append form depending on current ventilation mode
    match config.mode_settings.mode {
        VentilationMode::PC_CMV => form_pc_cmv(master, config),
        VentilationMode::PC_AC => form_pc_ac(master, config),
        VentilationMode::PC_VSAI => form_pc_vsai(master, config),
        VentilationMode::VC_CMV => form_vc_cmv(master, config),
        VentilationMode::VC_AC => form_vc_ac(master, config),
    }
}

fn form_pc_cmv<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    field_pressure_inspiratory(0, master, config);
    field_pressure_expiratory(1, master, config);
    field_duration_inspiration(2, master, config);
    field_cycles_per_minute(3, master, config);
}

fn form_pc_ac<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    field_pressure_inspiratory(0, master, config);
    field_pressure_expiratory(1, master, config);
    field_duration_inspiration(2, master, config);
    field_cycles_per_minute(3, master, config);
    field_trigger_offset(4, master, config);
}

fn form_pc_vsai<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    field_pressure_inspiratory(0, master, config);
    field_pressure_expiratory(1, master, config);
    field_time_inspiratory_minimum(2, master, config);
    field_time_inspiratory_maximum(3, master, config);
    field_cycles_per_minute(4, master, config);
    field_trigger_offset(5, master, config);
    field_trigger_expiratory(6, master, config);
}

fn form_vc_cmv<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    field_tidal_volume(0, master, config);
    field_pressure_expiratory(1, master, config);
    field_cycles_per_minute(2, master, config);
    field_duration_plateau(3, master, config);
    field_inspiratory_flow(4, master, config);
}

fn form_vc_ac<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    field_tidal_volume(0, master, config);
    field_pressure_expiratory(1, master, config);
    field_cycles_per_minute(2, master, config);
    field_duration_plateau(3, master, config);
    field_trigger_offset(4, master, config);
    field_inspiratory_flow(5, master, config);
}

fn field_pressure_inspiratory<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-pressure-inspiratory"),
            value_text: format!(
                "{} {}",
                convert_mmh2o_to_cmh2o(
                    ConvertMode::Rounded,
                    config.mode_settings.pressure_plateau as f64
                ),
                APP_I18N.t("telemetry-unit-cmh2o")
            ),
            ids: config.field_pressure_inspiratory_ids,
        },
    )
}

fn field_pressure_expiratory<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-pressure-expiratory"),
            value_text: format!(
                "{} {}",
                convert_mmh2o_to_cmh2o(
                    ConvertMode::Rounded,
                    config.mode_settings.pressure_expiratory as f64
                ),
                APP_I18N.t("telemetry-unit-cmh2o")
            ),
            ids: config.field_pressure_expiratory_ids,
        },
    )
}

fn field_time_inspiratory_minimum<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
) {
    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-time-inspiratory-minimum"),
            value_text: format!(
                "{} {}",
                config.mode_settings.inspiratory_time_minimum,
                APP_I18N.t("telemetry-unit-milliseconds")
            ),
            ids: config.field_time_inspiratory_minimum_ids,
        },
    )
}

fn field_time_inspiratory_maximum<'a>(
    index: usize,
    master: &mut ControlWidget<'a>,
    config: &Config,
) {
    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-time-inspiratory-maximum"),
            value_text: format!(
                "{} {}",
                config.mode_settings.inspiratory_time_maximum,
                APP_I18N.t("telemetry-unit-milliseconds")
            ),
            ids: config.field_time_inspiratory_maximum_ids,
        },
    )
}

fn field_cycles_per_minute<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-cycles-per-minute"),
            value_text: format!(
                "{}{}",
                config.mode_settings.cycles_per_minute,
                APP_I18N.t("telemetry-unit-per-minute")
            ),
            ids: config.field_cycles_per_minute_ids,
        },
    )
}

fn field_tidal_volume<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-tidal-volume"),
            value_text: format!(
                "{} {}",
                config.mode_settings.volume_tidal,
                APP_I18N.t("telemetry-unit-milliliters")
            ),
            ids: config.field_tidal_volume_ids,
        },
    )
}

fn field_inspiratory_flow<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-flow-inspiratory"),
            value_text: format!(
                "{} {}",
                config.mode_settings.flow_inspiration,
                APP_I18N.t("telemetry-unit-lpm")
            ),
            ids: config.field_inspiratory_flow_ids,
        },
    )
}

fn field_duration_inspiration<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-time-inspiratory"),
            value_text: format!(
                "{} {}",
                config.mode_settings.duration_inspiration,
                APP_I18N.t("telemetry-unit-milliseconds")
            ),
            ids: config.field_inspiratory_duration_ids,
        },
    )
}

fn field_duration_plateau<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-plateau-duration"),
            value_text: format!(
                "{} {}",
                config.mode_settings.duration_plateau,
                APP_I18N.t("telemetry-unit-milliseconds")
            ),
            ids: config.field_plateau_duration_ids,
        },
    )
}

fn field_trigger_offset<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-trigger-offset"),
            value_text: format!(
                "{:.1} {}",
                convert_mmh2o_to_cmh2o(
                    ConvertMode::WithDecimals,
                    config.mode_settings.trigger_inspiratory_offset as f64
                ),
                APP_I18N.t("telemetry-unit-cmh2o")
            ),
            ids: config.field_trigger_offset_ids,
        },
    )
}

fn field_trigger_inspiratory<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    // TODO: this is not used atm, but will be re-enabled whenever algorithms get better and \
    //   we can use this option.
    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-trigger-inspiratory"),
            value_text: format!("{}%", config.mode_settings.trigger_inspiratory_flow),
            ids: config.field_trigger_inspiratory_ids,
        },
    )
}

fn field_trigger_expiratory<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config) {
    draw_field(
        index,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-mode-trigger-expiratory"),
            value_text: format!("{}%", config.mode_settings.trigger_expiratory_flow),
            ids: config.field_trigger_expiratory_ids,
        },
    )
}

fn draw_field<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config, field: Field) {
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
        positions: top_left_with_margins_on[
            field.ids.0,
            -2.0,
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
