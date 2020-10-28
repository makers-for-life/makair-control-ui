// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{self, canvas, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use telemetry::alarm::AlarmCode;
use telemetry::structures::AlarmPriority;

use crate::chip::ChipState;
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

pub struct Config<'a> {
    container: WidgetId,
    wrapper: WidgetId,
    unit_box: WidgetId,
    unit_text: WidgetId,
    power_box: WidgetId,
    power_text: WidgetId,
    save_icon: WidgetId,
    battery_level: Option<u8>,
    chip_state: &'a ChipState,
    alarms: &'a [(AlarmCode, AlarmPriority)],
    save_icon_id: Option<conrod_core::image::Id>,
}

#[allow(clippy::too_many_arguments)]
impl<'a> Config<'a> {
    pub fn new(
        container: WidgetId,
        wrapper: WidgetId,
        unit_box: WidgetId,
        unit_text: WidgetId,
        power_box: WidgetId,
        power_text: WidgetId,
        save_icon: WidgetId,
        battery_level: Option<u8>,
        chip_state: &'a ChipState,
        alarms: &'a [(AlarmCode, AlarmPriority)],
        save_icon_id: Option<conrod_core::image::Id>,
    ) -> Config<'a> {
        Config {
            container,
            wrapper,
            unit_box,
            unit_text,
            power_box,
            power_text,
            save_icon,
            battery_level,
            chip_state,
            alarms,
            save_icon_id,
        }
    }
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
        .any(|alarm| alarm.0.code() == STATUS_ALARM_CODE_POWER_BATTERY);

    // Initialize wrapper canvas style
    let mut wrapper_style = canvas::Style::default();

    wrapper_style.color = Some(Color::Rgba(52.0 / 255.0, 52.0 / 255.0, 52.0 / 255.0, 1.0));
    wrapper_style.border = Some(0.0);
    wrapper_style.border_color = Some(color::TRANSPARENT);

    // Create wrapper canvas
    canvas::Canvas::new()
        .with_style(wrapper_style)
        .w_h(STATUS_WRAPPER_WIDTH, STATUS_WRAPPER_HEIGHT)
        .top_right_with_margins_on(
            config.container,
            STATUS_WRAPPER_MARGIN_TOP,
            STATUS_WRAPPER_MARGIN_RIGHT,
        )
        .set(config.wrapper, &mut master.ui);

    // Compute unit status style & text properties
    let (mut unit_box_style, mut unit_text_style) = (
        canvas::Style::default(),
        conrod_core::widget::primitive::text::Style::default(),
    );

    unit_text_style.font_id = Some(Some(master.fonts.bold));
    unit_text_style.color = Some(color::WHITE);
    unit_text_style.font_size = Some(11);

    if is_unit_stopped {
        unit_box_style.color = Some(Color::Rgba(180.0 / 255.0, 24.0 / 255.0, 28.0 / 255.0, 1.0));
    } else {
        unit_box_style.color = Some(Color::Rgba(50.0 / 255.0, 186.0 / 255.0, 0.0, 1.0));
    }

    unit_box_style.border = Some(0.0);
    unit_box_style.border_color = Some(color::TRANSPARENT);

    let unit_text_value = if is_unit_stopped {
        APP_I18N.t("status-unit-stopped")
    } else {
        APP_I18N.t("status-unit-active")
    };

    // Create unit status canvas & text
    canvas::Canvas::new()
        .with_style(unit_box_style)
        .w_h(box_width, box_height)
        .top_left_of(config.wrapper)
        .set(config.unit_box, &mut master.ui);

    widget::text::Text::new(&unit_text_value)
        .with_style(unit_text_style)
        .mid_top_with_margin_on(config.unit_box, STATUS_BOX_TEXT_MARGIN_TOP)
        .set(config.unit_text, &mut master.ui);

    if let Some(save_icon_id) = config.save_icon_id {
        widget::image::Image::new(save_icon_id)
            .w_h(15.0, 15.0)
            .right_from(config.unit_text, 3.0)
            .set(config.save_icon, &mut master.ui);
    }

    // Compute power box & text style properties
    let (mut power_box_style, mut power_text_style) = (
        canvas::Style::default(),
        conrod_core::widget::primitive::text::Style::default(),
    );

    power_text_style.font_id = Some(Some(master.fonts.bold));
    power_text_style.color = Some(color::WHITE);
    power_text_style.font_size = Some(11);

    if is_battery_powered {
        power_box_style.color = Some(Color::Rgba(208.0 / 255.0, 92.0 / 255.0, 0.0, 1.0));
    } else {
        power_box_style.color = Some(color::TRANSPARENT);
    }

    power_box_style.border = Some(0.0);
    power_box_style.border_color = Some(color::TRANSPARENT);

    // Create power box canvas & text
    canvas::Canvas::new()
        .with_style(power_box_style)
        .w_h(box_width, box_height)
        .bottom_left_of(config.wrapper)
        .set(config.power_box, &mut master.ui);

    let power_text_value = if is_battery_powered {
        let mut value = APP_I18N.t("status-power-battery");

        if let Some(battery_level) = config.battery_level {
            value.push_str(" (");
            value.push_str(&battery_level.to_string());
            value.push_str("V)");
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
