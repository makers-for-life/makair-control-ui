// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::chip::ChipState;
use crate::config::environment::*;
use crate::display::widget::ControlWidget;

const BUTTON_RUN_START_COLOR: Color = Color::Rgba(14.0 / 255.0, 132.0 / 255.0, 0.0, 1.0);
const BUTTON_RUN_STOP_COLOR: Color = Color::Rgba(143.0 / 255.0, 19.0 / 255.0, 19.0 / 255.0, 1.0);
const BUTTON_ADVANCED_COLOR: Color = Color::Rgba(52.0 / 255.0, 52.0 / 255.0, 52.0 / 255.0, 1.0);

pub struct Config<'a> {
    container: WidgetId,
    wrapper: WidgetId,
    run_button: WidgetId,
    advanced_button: WidgetId,
    run_icon: WidgetId,
    advanced_icon: WidgetId,

    run_icon_image: conrod_core::image::Id,
    advanced_icon_image: conrod_core::image::Id,

    chip_state: &'a ChipState,
}

#[allow(clippy::too_many_arguments)]
impl<'a> Config<'a> {
    pub fn new(
        container: WidgetId,
        wrapper: WidgetId,
        run_button: WidgetId,
        advanced_button: WidgetId,
        run_icon: WidgetId,
        advanced_icon: WidgetId,
        run_icon_image: conrod_core::image::Id,
        advanced_icon_image: conrod_core::image::Id,
        chip_state: &'a ChipState,
    ) -> Config<'a> {
        Config {
            container,
            wrapper,
            run_button,
            advanced_button,
            run_icon,
            advanced_icon,
            run_icon_image,
            advanced_icon_image,
            chip_state,
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

    // Create advanced control button
    widget::primitive::shape::circle::Circle::fill_with(
        CONTROLS_BUTTON_RADIUS,
        BUTTON_ADVANCED_COLOR,
    )
    .top_right_of(config.wrapper)
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
