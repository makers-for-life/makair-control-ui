// MakAir Control UI
//
// Copyright: 2021, Makers For Life
// License: Public Domain License

use std::cmp::Ordering;

use conrod_core::{
    color::{self, Color},
    widget::{self, id::List, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::APP_I18N;

const CONTENT_BOX_DEFAULT_BORDER_COLOR: Color =
    Color::Rgba(42.0 / 255.0, 42.0 / 255.0, 42.0 / 255.0, 1.0);
const CONTENT_BOX_ERROR_BORDER_COLOR: Color =
    Color::Rgba(219.0 / 255.0, 16.0 / 255.0, 16.0 / 255.0, 1.0);
const CONTENT_BOX_SUCCESS_BORDER_COLOR: Color =
    Color::Rgba(6.0 / 255.0, 174.0 / 255.0, 33.0 / 255.0, 1.0);
const CONTENT_DETAILS_BOX_BORDER_COLOR: Color = Color::Rgba(1.0, 1.0, 1.0, 0.3);

pub struct Config<'a> {
    pub title_wrapper: WidgetId,
    pub title_separator: WidgetId,
    pub title_primary: WidgetId,
    pub title_secondary: WidgetId,

    pub steps_wrapper: WidgetId,
    pub steps_items: WidgetId,
    pub steps_progress: &'a List,
    pub steps_circles: &'a List,
    pub steps_indexes: &'a List,

    pub content_wrapper: WidgetId,
    pub content_box: WidgetId,
    pub content_icon: WidgetId,
    pub content_text_wrapper: WidgetId,
    pub content_text_title: WidgetId,
    pub content_text_message: WidgetId,
    pub content_details_box: WidgetId,
    pub content_details_text: WidgetId,
    pub content_button: WidgetId,
    pub content_button_text: WidgetId,

    pub error: bool,
    pub success: bool,
    pub confirm: bool,

    pub step: u8,

    pub icon: conrod_core::image::Id,

    pub title: String,
    pub message: String,
    pub details: String,
}

pub fn render(master: &mut ControlWidget, config: Config) -> f64 {
    // Create title
    title_wrapper(master, &config);
    title_separator(master, &config);
    title_primary(master, &config);
    title_secondary(master, &config);

    // Create steps
    steps_wrapper(master, &config);
    steps_items(master, &config);
    steps_progress(master, &config);
    steps_circles(master, &config);
    steps_indexes(master, &config);

    // Create content
    content_wrapper(master, &config);
    content_box(master, &config);
    content_icon(master, &config);
    content_text_wrapper(master, &config);
    content_text_title(master, &config);
    content_text_message(master, &config);
    content_details_box(master, &config);
    content_details_text(master, &config);
    content_button(master, &config);

    0 as _
}

fn title_wrapper<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    gen_widget_container!(
        master,
        container_id: config.title_wrapper,
        color: color::RED,
        width: END_OF_LINE_TITLE_WIDTH,
        height: END_OF_LINE_TITLE_HEIGHT,
        positions: top_left[]
    );
}

fn title_separator<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    widget::Rectangle::fill_with(
        [1.0, (END_OF_LINE_TITLE_FONT_SIZE + 4) as _],
        color::WHITE.alpha(0.15),
    )
    .middle_of(config.title_wrapper)
    .y_relative(1.0)
    .set(config.title_separator, &mut master.ui);
}

fn title_primary<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize text style
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::WHITE);
    text_style.font_size = Some(END_OF_LINE_TITLE_FONT_SIZE);

    // Create text
    widget::Text::new(&APP_I18N.t("end-of-line-title-primary").to_uppercase())
        .mid_right_with_margin_on(config.title_separator, END_OF_LINE_TITLE_SEPARATOR_SPACING)
        .y_relative(2.0)
        .with_style(text_style)
        .set(config.title_primary, &mut master.ui);
}

fn title_secondary<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize text style
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.regular));
    text_style.color = Some(color::WHITE.alpha(0.65));
    text_style.font_size = Some(END_OF_LINE_TITLE_FONT_SIZE);

    // Create text
    widget::Text::new(&APP_I18N.t("end-of-line-title-secondary"))
        .mid_left_with_margin_on(config.title_separator, END_OF_LINE_TITLE_SEPARATOR_SPACING)
        .y_relative(0.0)
        .with_style(text_style)
        .set(config.title_secondary, &mut master.ui);
}

fn steps_wrapper<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    gen_widget_container!(
        master,
        container_id: config.steps_wrapper,
        color: color::TRANSPARENT,
        width: END_OF_LINE_STEPS_WRAPPER_WIDTH,
        height: END_OF_LINE_STEPS_WRAPPER_HEIGHT,
        positions: down_from[
            config.title_wrapper, 0.0,
        ]
    );
}

fn steps_items<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    gen_widget_container!(
        master,
        container_id: config.steps_items,
        color: color::TRANSPARENT,
        width: END_OF_LINE_STEPS_ITEMS_WIDTH,
        height: END_OF_LINE_STEPS_ITEMS_HEIGHT,
        positions: middle_of[
            config.steps_wrapper,
        ]
    );
}

fn steps_progress<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Acquire step details
    let ongoing_step = config.step as usize;

    for (index, steps_progress) in config.steps_progress.iter().enumerate() {
        if index + 1 < ongoing_step {
            widget::Rectangle::fill_with(
                [
                    END_OF_LINE_STEPS_CIRCLE_MARGIN_RIGHT,
                    END_OF_LINE_STEPS_PROGRESS_HEIGHT,
                ],
                color::WHITE.alpha(0.075),
            )
            .mid_left_with_margin_on(
                config.steps_items,
                END_OF_LINE_STEPS_CIRCLE_SIZE
                    + index as f64
                        * (END_OF_LINE_STEPS_CIRCLE_SIZE + END_OF_LINE_STEPS_CIRCLE_MARGIN_RIGHT),
            )
            .set(*steps_progress, &mut master.ui);
        }
    }
}

fn steps_circles<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    let circle_radius = END_OF_LINE_STEPS_CIRCLE_SIZE / 2.0;

    for (index, steps_circle) in config.steps_circles.iter().enumerate() {
        let margin_left =
            (END_OF_LINE_STEPS_CIRCLE_SIZE + END_OF_LINE_STEPS_CIRCLE_MARGIN_RIGHT) * index as f64;

        // Acquire step details
        let (current_step, ongoing_step) = (index + 1, config.step as usize);

        // Step is active?
        if current_step == ongoing_step {
            widget::primitive::shape::circle::Circle::fill_with(circle_radius, color::WHITE)
                .top_left_with_margins_on(config.steps_items, 0.0, margin_left)
                .set(*steps_circle, &mut master.ui);
        } else {
            let line_style = widget::primitive::line::Style::solid()
                .color(if current_step > ongoing_step {
                    color::WHITE.alpha(0.02)
                } else {
                    color::WHITE.alpha(0.45)
                })
                .thickness(END_OF_LINE_STEPS_CIRCLE_THICKNESS);

            widget::primitive::shape::circle::Circle::outline_styled(circle_radius, line_style)
                .top_left_with_margins_on(config.steps_items, 0.0, margin_left)
                .set(*steps_circle, &mut master.ui);
        }
    }
}

fn steps_indexes<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    for (index, steps_index) in config.steps_indexes.iter().enumerate() {
        // Acquire step details
        let (current_step, ongoing_step) = (index + 1, config.step as usize);

        // Generate text style
        let mut text_style = conrod_core::widget::primitive::text::Style::default();

        text_style.font_id = Some(Some(master.fonts.bold));

        text_style.color = Some(match current_step.cmp(&ongoing_step) {
            Ordering::Greater => color::WHITE.alpha(0.1),
            Ordering::Less => color::WHITE,
            Ordering::Equal => color::BLACK,
        });

        text_style.font_size = Some(END_OF_LINE_STEPS_INDEX_FONT_SIZE);

        // Create text
        widget::Text::new(&(index + 1).to_string())
            .middle_of(config.steps_circles[index])
            .y_relative(if index == 0 { 2.0 } else { 0.0 })
            .with_style(text_style)
            .set(*steps_index, &mut master.ui);
    }
}

fn content_wrapper<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    gen_widget_container!(
        master,
        container_id: config.content_wrapper,
        color: color::TRANSPARENT,
        width: END_OF_LINE_CONTENT_WIDTH,
        height: END_OF_LINE_CONTENT_HEIGHT,
        positions: down_from[
            config.steps_wrapper, 0.0,
        ]
    );
}

fn content_box<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize container style
    let mut container_style = widget::canvas::Style::default();

    container_style.color = Some(if config.error || config.success {
        color::TRANSPARENT
    } else {
        color::WHITE.alpha(0.01)
    });
    container_style.border = Some(2.0);

    container_style.border_color = Some(if config.error {
        CONTENT_BOX_ERROR_BORDER_COLOR
    } else if config.success {
        CONTENT_BOX_SUCCESS_BORDER_COLOR
    } else {
        CONTENT_BOX_DEFAULT_BORDER_COLOR
    });

    // Create container
    widget::Canvas::new()
        .with_style(container_style)
        .w_h(
            END_OF_LINE_CONTENT_BOX_WIDTH,
            END_OF_LINE_CONTENT_BOX_HEIGHT,
        )
        .middle_of(config.content_wrapper)
        .y_relative(END_OF_LINE_CONTENT_BOX_OFFSET_TOP)
        .set(config.content_box, &mut master.ui);
}

fn content_icon<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Create image
    widget::Image::new(config.icon)
        .w_h(END_OF_LINE_CONTENT_ICON_SIZE, END_OF_LINE_CONTENT_ICON_SIZE)
        .mid_left_with_margin_on(config.content_box, END_OF_LINE_CONTENT_ICON_SPACING_SIDES)
        .set(config.content_icon, &mut master.ui);
}

fn content_text_wrapper<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    gen_widget_container!(
        master,
        container_id: config.content_text_wrapper,
        color: color::TRANSPARENT,
        width: END_OF_LINE_CONTENT_TEXT_WRAPPER_WIDTH,
        height: END_OF_LINE_CONTENT_TEXT_WRAPPER_HEIGHT,
        positions: mid_right_with_margin_on[
            config.content_box, END_OF_LINE_CONTENT_TEXT_WRAPPER_MARGIN_RIGHT,
        ]
    );
}

fn content_text_title<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize text style
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::WHITE);
    text_style.font_size = Some(END_OF_LINE_CONTENT_TEXT_TITLE_FONT_SIZE);

    // Create text
    widget::Text::new(&config.title)
        .top_left_of(config.content_text_wrapper)
        .with_style(text_style)
        .set(config.content_text_title, &mut master.ui);
}

fn content_text_message<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Initialize text style
    let mut text_style = conrod_core::widget::primitive::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.regular));
    text_style.color = Some(color::WHITE.alpha(0.5));
    text_style.font_size = Some(END_OF_LINE_CONTENT_TEXT_MESSAGE_FONT_SIZE);

    // Create text
    widget::Text::new(&config.message)
        .down_from(
            config.content_text_title,
            END_OF_LINE_CONTENT_TEXT_MESSAGE_MARGIN_TOP,
        )
        .with_style(text_style)
        .set(config.content_text_message, &mut master.ui);
}

fn content_details_box<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    if !config.details.is_empty() {
        // Initialize container style
        let mut container_style = widget::canvas::Style::default();

        container_style.color = Some(color::BLACK);
        container_style.border = Some(1.0);

        container_style.border_color = Some(CONTENT_DETAILS_BOX_BORDER_COLOR);

        // Create container
        widget::Canvas::new()
            .with_style(container_style)
            .w_h(
                END_OF_LINE_CONTENT_DETAILS_BOX_WIDTH,
                END_OF_LINE_CONTENT_DETAILS_BOX_HEIGHT,
            )
            .up_from(
                config.content_box,
                END_OF_LINE_CONTENT_DETAILS_BOX_MARGIN_BOTTOM,
            )
            .set(config.content_details_box, &mut master.ui);
    }
}

fn content_details_text<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    if !config.details.is_empty() {
        // Initialize text style
        let mut text_style = conrod_core::widget::primitive::text::Style::default();

        text_style.font_id = Some(Some(master.fonts.regular));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some(END_OF_LINE_CONTENT_DETAILS_TEXT_FONT_SIZE);

        // Create text
        widget::Text::new(&config.details)
            .middle_of(config.content_details_box)
            .y_relative(2.5)
            .with_style(text_style)
            .set(config.content_details_text, &mut master.ui);
    }
}

fn content_button<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Append button? (if confirm enabled)
    if config.confirm {
        gen_widget_button!(
            master,
            button_id: config.content_button,
            text_id: config.content_button_text,
            text_color: color::BLACK,
            text_font_size: END_OF_LINE_CONTENT_BUTTON_FONT_SIZE,
            width: END_OF_LINE_CONTENT_BUTTON_WIDTH,
            value_top: END_OF_LINE_CONTENT_BUTTON_VALUE_TOP,
            value: &APP_I18N.t("end-of-line-content-button-continue"),

            positions: (
                down_from[
                    config.content_box,
                    END_OF_LINE_CONTENT_BUTTON_MARGIN_BOTTOM,
                ],

                align_right_of[
                    config.content_box,
                ]
            )
        );
    }
}
