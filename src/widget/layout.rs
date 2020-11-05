// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;

pub struct Slice {
    pub parent: WidgetId,
    pub layout: WidgetId,

    pub top: f64,
    pub height: f64,
}

pub struct Config {
    pub header: Slice,
    pub body: Slice,
    pub footer: Slice,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // #1: Create body layout rectangle
    // Notice: this must be drawn first, so that the 'z-index' of this layout slice is lower than \
    //   others following.
    widget::Rectangle::fill_with(
        [DISPLAY_WINDOW_SIZE_WIDTH as _, config.body.height],
        color::TRANSPARENT,
    )
    .top_left_with_margins_on(config.body.parent, config.body.top, 0.0)
    .set(config.body.layout, &mut master.ui);

    // #2: Create header layout rectangle
    // Notice: this block is defined after the body because we want it to overflow and be on top \
    //   of the screen (this is important for alarms).
    widget::Rectangle::fill_with(
        [DISPLAY_WINDOW_SIZE_WIDTH as _, config.header.height],
        color::TRANSPARENT,
    )
    .top_left_with_margins_on(config.header.parent, config.header.top, 0.0)
    .set(config.header.layout, &mut master.ui);

    // #3: Create footer layout rectangle
    // Notice: this block is drawn at the very end, as we want to guarantee that telemetry values \
    //   are always visible, no matter how the header contents overflow.
    widget::Rectangle::fill_with(
        [DISPLAY_WINDOW_SIZE_WIDTH as _, config.footer.height],
        color::TRANSPARENT,
    )
    .top_left_with_margins_on(config.footer.parent, config.footer.top, 0.0)
    .set(config.footer.layout, &mut master.ui);

    0.0
}
