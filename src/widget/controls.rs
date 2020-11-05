// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::chip::{
    settings::{snooze::SettingsSnooze, SettingActionState},
    ChipState,
};
use crate::config::environment::*;
use crate::display::widget::ControlWidget;

const BUTTON_BASE_COLOR: Color = Color::Rgba(52.0 / 255.0, 52.0 / 255.0, 52.0 / 255.0, 1.0);

const BUTTON_RUN_START_COLOR: Color = Color::Rgba(14.0 / 255.0, 132.0 / 255.0, 0.0, 1.0);
const BUTTON_RUN_STOP_COLOR: Color = Color::Rgba(143.0 / 255.0, 19.0 / 255.0, 19.0 / 255.0, 1.0);
const BUTTON_SNOOZE_INACTIVE_COLOR: Color = BUTTON_BASE_COLOR;
const BUTTON_SNOOZE_ACTIVE_COLOR: Color = Color::Rgba(1.0, 1.0, 1.0, 1.0);
const BUTTON_ADVANCED_COLOR: Color = BUTTON_BASE_COLOR;

pub struct Config<'a> {
    container: WidgetId,
    wrapper: WidgetId,
    run_button: WidgetId,
    snooze_button: WidgetId,
    advanced_button: WidgetId,
    run_icon: WidgetId,
    snooze_icon: WidgetId,
    advanced_icon: WidgetId,

    run_icon_image: conrod_core::image::Id,
    snooze_active_icon_image: conrod_core::image::Id,
    snooze_inactive_icon_image: conrod_core::image::Id,
    advanced_icon_image: conrod_core::image::Id,

    chip_state: &'a ChipState,
    snooze_settings: &'a SettingsSnooze,
}

#[allow(clippy::too_many_arguments)]
impl<'a> Config<'a> {
    pub fn new(
        container: WidgetId,
        wrapper: WidgetId,
        run_button: WidgetId,
        snooze_button: WidgetId,
        advanced_button: WidgetId,
        run_icon: WidgetId,
        snooze_icon: WidgetId,
        advanced_icon: WidgetId,
        run_icon_image: conrod_core::image::Id,
        snooze_inactive_icon_image: conrod_core::image::Id,
        snooze_active_icon_image: conrod_core::image::Id,
        advanced_icon_image: conrod_core::image::Id,
        chip_state: &'a ChipState,
        snooze_settings: &'a SettingsSnooze,
    ) -> Config<'a> {
        Config {
            container,
            wrapper,
            run_button,
            snooze_button,
            advanced_button,
            run_icon,
            snooze_icon,
            advanced_icon,
            run_icon_image,
            snooze_inactive_icon_image,
            snooze_active_icon_image,
            advanced_icon_image,
            chip_state,
            snooze_settings,
        }
    }
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create wrapper canvas
    gen_widget_container!(
        master,
        container_id: config.wrapper,
        color: color::TRANSPARENT,
        width: CONTROLS_WRAPPER_WIDTH,
        height: CONTROLS_WRAPPER_HEIGHT,
        positions: top_right_with_margins_on[
            config.container,
            CONTROLS_WRAPPER_MARGIN_TOP,
            CONTROLS_WRAPPER_MARGIN_RIGHT,
        ]
    );

    // Create run control button
    widget::primitive::shape::circle::Circle::fill_with(
        CONTROLS_BUTTON_RADIUS,
        if config.chip_state == &ChipState::Stopped {
            BUTTON_RUN_START_COLOR
        } else {
            BUTTON_RUN_STOP_COLOR
        },
    )
    .top_left_of(config.wrapper)
    .set(config.run_button, &mut master.ui);

    // Append run icon
    widget::image::Image::new(config.run_icon_image)
        .w_h(
            CONTROLS_BUTTON_ICON_HEIGHT as _,
            CONTROLS_BUTTON_ICON_WIDTH as _,
        )
        .middle_of(config.run_button)
        .set(config.run_icon, &mut master.ui);

    // Create snooze control button
    widget::primitive::shape::circle::Circle::fill_with(
        CONTROLS_BUTTON_RADIUS,
        if config.snooze_settings.alarms == SettingActionState::Enabled {
            BUTTON_SNOOZE_ACTIVE_COLOR
        } else {
            BUTTON_SNOOZE_INACTIVE_COLOR
        },
    )
    .x_y_relative_to(config.run_button, CONTROLS_BUTTON_FOOTPRINT_WIDTH, 0.0)
    .set(config.snooze_button, &mut master.ui);

    // Append snooze icon
    widget::image::Image::new(
        if config.snooze_settings.alarms == SettingActionState::Enabled {
            config.snooze_active_icon_image
        } else {
            config.snooze_inactive_icon_image
        },
    )
    .w_h(
        CONTROLS_BUTTON_ICON_HEIGHT as _,
        CONTROLS_BUTTON_ICON_WIDTH as _,
    )
    .middle_of(config.snooze_button)
    .set(config.snooze_icon, &mut master.ui);

    // Create advanced control button
    widget::primitive::shape::circle::Circle::fill_with(
        CONTROLS_BUTTON_RADIUS,
        BUTTON_ADVANCED_COLOR,
    )
    .x_y_relative_to(config.snooze_button, CONTROLS_BUTTON_FOOTPRINT_WIDTH, 0.0)
    .set(config.advanced_button, &mut master.ui);

    // Append advanced icon
    widget::image::Image::new(config.advanced_icon_image)
        .w_h(
            CONTROLS_BUTTON_ICON_HEIGHT as _,
            CONTROLS_BUTTON_ICON_WIDTH as _,
        )
        .middle_of(config.advanced_button)
        .set(config.advanced_icon, &mut master.ui);

    CONTROLS_WRAPPER_WIDTH
}
