// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use makair_telemetry::alarm::{AlarmCode, RMC_SW_16};
use makair_telemetry::structures::{AlarmPriority, DataSnapshot};

use crate::chip::ChipState;
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

const POWER_BOX_BATTERY_WEAK_THRESHOLD: u8 = 25;

const WRAPPER_COLOR: Color = Color::Rgba(52.0 / 255.0, 52.0 / 255.0, 52.0 / 255.0, 1.0);

const UNIT_STOPPED_COLOR: Color = Color::Rgba(180.0 / 255.0, 24.0 / 255.0, 28.0 / 255.0, 1.0);
const UNIT_ACTIVE_COLOR: Color = Color::Rgba(50.0 / 255.0, 186.0 / 255.0, 0.0, 1.0);

const POWER_BOX_BATTERY_NORMAL_COLOR: Color = Color::Rgba(208.0 / 255.0, 92.0 / 255.0, 0.0, 1.0);
const POWER_BOX_BATTERY_WEAK_COLOR: Color = Color::Rgba(1.0, 0.0 / 255.0, 3.0 / 255.0, 1.0);

pub struct Config<'a> {
    pub container: WidgetId,
    pub wrapper: WidgetId,
    pub unit_box: WidgetId,
    pub unit_text: WidgetId,
    pub power_box: WidgetId,
    pub power_text: WidgetId,
    pub recording: Option<(WidgetId, WidgetId)>,

    pub battery_soc: Option<u8>,
    pub chip_state: &'a ChipState,
    pub data_snapshot: Option<&'a DataSnapshot>,
    pub alarms: &'a [(AlarmCode, AlarmPriority)],
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Compute status box height and width
    let (box_height, box_width) = (STATUS_WRAPPER_HEIGHT / 2.0, STATUS_WRAPPER_WIDTH);

    // Check whether chip state is currently in stopped mode or active (running)
    let is_unit_stopped = config.chip_state == &ChipState::Stopped;

    // Check whether power is currently on AC or battery
    // Notice: the telemetry library reports this as an alarm
    let is_battery_powered = config
        .alarms
        .iter()
        .any(|alarm| alarm.0.code() == RMC_SW_16);

    // Create wrapper canvas
    gen_widget_container!(
        master,
        container_id: config.wrapper,
        color: WRAPPER_COLOR,
        width: STATUS_WRAPPER_WIDTH,
        height: STATUS_WRAPPER_HEIGHT,
        positions: top_right_with_margins_on[
            config.container,
            STATUS_WRAPPER_MARGIN_TOP,
            STATUS_WRAPPER_MARGIN_RIGHT,
        ]
    );

    // Create unit canvas
    gen_widget_container!(
        master,
        container_id: config.unit_box,
        color: if is_unit_stopped {
            UNIT_STOPPED_COLOR
        } else {
            UNIT_ACTIVE_COLOR
        },
        width: box_width,
        height: box_height,
        positions: top_left_of[
            config.wrapper,
        ]
    );

    // Compute unit text value & style
    let mut unit_text_style = conrod_core::widget::primitive::text::Style::default();

    unit_text_style.font_id = Some(Some(master.fonts.bold));
    unit_text_style.color = Some(color::WHITE);
    unit_text_style.font_size = Some(11);

    let unit_text_value = if is_unit_stopped {
        APP_I18N.t("status-unit-stopped")
    } else {
        APP_I18N.t("status-unit-active")
    };

    // Create unit status text
    if let Some(recording) = config.recording {
        widget::text::Text::new(&unit_text_value)
            .with_style(unit_text_style)
            .top_left_with_margins_on(
                config.unit_box,
                STATUS_BOX_TEXT_MARGIN_TOP,
                STATUS_BOX_RECORDING_PADDING_LEFT,
            )
            .set(config.unit_text, &mut master.ui);

        widget::primitive::shape::circle::Circle::fill_with(
            STATUS_RECORDING_OUTER_RADIUS,
            color::WHITE,
        )
        .mid_right_with_margin_on(config.unit_box, STATUS_BOX_RECORDING_PADDING_RIGHT)
        .set(recording.0, &mut master.ui);

        widget::primitive::shape::circle::Circle::fill_with(
            STATUS_RECORDING_INNER_RADIUS,
            color::RED,
        )
        .middle_of(recording.0)
        .set(recording.1, &mut master.ui);
    } else {
        widget::text::Text::new(&unit_text_value)
            .with_style(unit_text_style)
            .mid_top_with_margin_on(config.unit_box, STATUS_BOX_TEXT_MARGIN_TOP)
            .set(config.unit_text, &mut master.ui);
    }

    // Compute power box battery SoC (if any available)
    // Notice: the hardware may put multiple lead-acid batteries in series, hence why we first \
    //   need to divide battery level voltage by the number of series batteries.
    let battery_soc = if is_battery_powered {
        config.battery_soc
    } else {
        None
    };

    // Create power box canvas
    gen_widget_container!(
        master,
        container_id: config.power_box,
        color: if is_battery_powered {
            if battery_soc.unwrap_or(100) <= POWER_BOX_BATTERY_WEAK_THRESHOLD {
                POWER_BOX_BATTERY_WEAK_COLOR
            } else {
                POWER_BOX_BATTERY_NORMAL_COLOR
            }
        } else {
            color::TRANSPARENT
        },
        width: box_width,
        height: box_height,
        positions: bottom_left_of[
            config.wrapper,
        ]
    );

    // Create power box text
    let mut power_text_style = conrod_core::widget::primitive::text::Style::default();

    power_text_style.font_id = Some(Some(master.fonts.bold));
    power_text_style.color = Some(color::WHITE);
    power_text_style.font_size = Some(11);

    let power_text_value = if is_battery_powered {
        let mut value = APP_I18N.t("status-power-battery");

        if let Some(battery_soc) = battery_soc {
            value.push_str(&format!(" ({}%)", battery_soc));
        }

        value
    } else {
        APP_I18N.t("status-power-ac")
    };

    widget::text::Text::new(&power_text_value)
        .with_style(power_text_style)
        .mid_top_with_margin_on(config.power_box, STATUS_BOX_TEXT_MARGIN_TOP)
        .set(config.power_text, &mut master.ui);

    STATUS_WRAPPER_WIDTH
}
