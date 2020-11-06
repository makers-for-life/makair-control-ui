// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{
        self, canvas, primitive::shape::Style, rounded_rectangle::RoundedRectangle, Id as WidgetId,
    },
    Positionable, Sizeable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

const CANVAS_COLOR: Color = Color::Rgba(0.0, 0.0, 0.0, 0.8);

const CONTAINER_BORDER_COLOR: Color = Color::Rgba(81.0 / 255.0, 81.0 / 255.0, 81.0 / 255.0, 1.0);
const CONTAINER_COLOR: Color = Color::Rgba(26.0 / 255.0, 26.0 / 255.0, 26.0 / 255.0, 1.0);

pub struct Config {
    pub parent: WidgetId,
    pub background: WidgetId,
    pub container_borders: WidgetId,
    pub container: WidgetId,
    pub validate: Option<(WidgetId, WidgetId)>,

    pub width: f64,
    pub height: f64,
    pub padding: Option<f64>,
    pub colors: Option<(Color, Color)>,
    pub background_sizes: Option<(u32, u32)>,
}

pub fn render<'a>(master: &mut ControlWidget<'a>, config: Config) -> f64 {
    // Create background
    gen_widget_container!(
        master,
        container_id: config.background,
        color: CANVAS_COLOR,
        width:
            config.background_sizes.map(|sizes| sizes.0).unwrap_or(DISPLAY_WINDOW_SIZE_WIDTH) as _,
        height:
            config.background_sizes.map(|sizes| sizes.1).unwrap_or(DISPLAY_WINDOW_SIZE_HEIGHT) as _,
        positions: middle_of[
            config.parent,
        ]
    );

    // Initialize container style for borders
    let container_borders_style = Style::Fill(
        config
            .colors
            .map(|colors| colors.1)
            .or(Some(CONTAINER_BORDER_COLOR)),
    );

    // Create rectangle for borders
    RoundedRectangle::styled(
        [config.width + 5.0, config.height + 5.0],
        DISPLAY_ROUNDED_RECTANGLES_ROUND,
        container_borders_style,
    )
    .middle_of(config.parent)
    .set(config.container_borders, &mut master.ui);

    // Initialize container style
    let mut container_style = canvas::Style::default();

    container_style.color = config
        .colors
        .map(|colors| colors.0)
        .or(Some(CONTAINER_COLOR));
    container_style.border = Some(0.0);
    container_style.border_color = Some(color::TRANSPARENT);

    // Create container canvas
    let mut container = canvas::Canvas::new()
        .with_style(container_style)
        .w_h(config.width, config.height)
        .middle_of(config.container_borders);

    if let Some(padding) = config.padding {
        container = container.pad(padding);
    }

    container.set(config.container, &mut master.ui);

    // Append validation button? (if any set)
    if let Some((validate_button, validate_text)) = config.validate {
        gen_widget_button!(
            master,
            button_id: validate_button,
            text_id: validate_text,
            text_color: color::BLACK,
            text_font_size: 16,
            width: MODAL_VALIDATE_BUTTON_WIDTH,
            value_top: 4.0,
            value: &APP_I18N.t("modal-close"),

            positions: (
                bottom_right_of[
                    config.container,
                ]
            )
        );
    }

    0 as _
}
