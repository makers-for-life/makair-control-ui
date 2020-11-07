// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;

pub struct Config {
    pub container: WidgetId,
    pub wrapper: WidgetId,
    pub run_button: WidgetId,
    pub snooze_button: WidgetId,
    pub advanced_button: WidgetId,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Notice: all button circles must be transparent here, as they are drawn via the header \
    //   layout texture, which is more optimal CPU-wise than rending sub-images of icons here.

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
    widget::primitive::shape::circle::Circle::fill_with(CONTROLS_BUTTON_RADIUS, color::TRANSPARENT)
        .top_left_of(config.wrapper)
        .set(config.run_button, &mut master.ui);

    // Create snooze control button
    widget::primitive::shape::circle::Circle::fill_with(CONTROLS_BUTTON_RADIUS, color::TRANSPARENT)
        .x_y_relative_to(config.run_button, CONTROLS_BUTTON_FOOTPRINT_WIDTH, 0.0)
        .set(config.snooze_button, &mut master.ui);

    // Create advanced control button
    widget::primitive::shape::circle::Circle::fill_with(CONTROLS_BUTTON_RADIUS, color::TRANSPARENT)
        .x_y_relative_to(config.snooze_button, CONTROLS_BUTTON_FOOTPRINT_WIDTH, 0.0)
        .set(config.advanced_button, &mut master.ui);

    CONTROLS_WRAPPER_WIDTH
}
