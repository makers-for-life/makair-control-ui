// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::{
    color::{self, Color},
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};

use crate::chip::settings::preset::{SettingsPreset, SettingsPresetAge};
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
use crate::locale::preset::age_to_locale as preset_age_to_locale;
use crate::APP_I18N;

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

    pub preset_settings: &'a SettingsPreset,

    pub container_parent: WidgetId,
    pub container_widget: WidgetId,

    pub title_primary: WidgetId,
    pub title_secondary: WidgetId,

    pub content_wrapper: WidgetId,
    pub content_image: WidgetId,
    pub content_separator: WidgetId,
    pub content_form_wrapper: WidgetId,

    pub field_age_ids: FieldWidgetIds,
    pub field_height_ids: FieldWidgetIds,

    pub baby_image: conrod_core::image::Id,
    pub child_image: conrod_core::image::Id,
    pub teenager_image: conrod_core::image::Id,
    pub adult_image: conrod_core::image::Id,
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

    // Append contents
    title(master, &config);
    content(master, &config);

    0 as _
}

fn title<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Render all titles
    title_primary(master, config);
    title_secondary(master, config);
}

fn title_primary<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate text style
    let mut text_style = widget::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.bold));
    text_style.color = Some(color::WHITE);
    text_style.font_size = Some(19);

    // Append text
    widget::Text::new(&APP_I18N.t("modal-preset-title"))
        .with_style(text_style)
        .mid_top_of(config.container_widget)
        .set(config.title_primary, &mut master.ui);
}

fn title_secondary<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Generate text style
    let mut text_style = widget::text::Style::default();

    text_style.font_id = Some(Some(master.fonts.regular));
    text_style.color = Some(color::WHITE.alpha(0.5));
    text_style.font_size = Some(15);

    // Append text
    widget::Text::new(&APP_I18N.t("modal-preset-subtitle"))
        .with_style(text_style)
        .mid_bottom_with_margin_on(config.title_primary, -27.0)
        .set(config.title_secondary, &mut master.ui);
}

fn content<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    let size = (
        config.width,
        config.height - PRESET_SETTINGS_MODAL_TITLE_TOTAL_HEIGHT,
    );

    // Create content container
    gen_widget_container!(
        master,
        container_id: config.content_wrapper,
        color: color::TRANSPARENT,
        width: size.0,
        height: size.1,
        positions: bottom_left_of[
            config.container_widget,
        ]
    );

    // Render all contents
    content_image(master, config);
    content_separator(master, config, size);
    content_form(master, config, size);
}

fn content_image<'a>(master: &mut ControlWidget<'a>, config: &Config) {
    // Acquire image (depending on age group)
    let image = match config.preset_settings.age {
        SettingsPresetAge::Baby => config.baby_image,
        SettingsPresetAge::Child => config.child_image,
        SettingsPresetAge::Teenager => config.teenager_image,
        SettingsPresetAge::Adult => config.adult_image,
    };

    // Create image
    widget::Image::new(image)
        .w_h(
            PRESET_SETTINGS_MODAL_CONTENT_IMAGE_WIDTH,
            PRESET_SETTINGS_MODAL_CONTENT_IMAGE_HEIGHT,
        )
        .mid_left_of(config.content_wrapper)
        .set(config.content_image, &mut master.ui);
}

fn content_separator<'a>(master: &mut ControlWidget<'a>, config: &Config, size: (f64, f64)) {
    widget::Rectangle::fill_with([1.0, size.1], color::WHITE.alpha(0.035))
        .mid_right_with_margin_on(
            config.content_image,
            -PRESET_SETTINGS_MODAL_CONTENT_SEPARATOR_MARGIN_SIDES,
        )
        .set(config.content_separator, &mut master.ui);
}

fn content_form<'a>(master: &mut ControlWidget<'a>, config: &Config, size: (f64, f64)) {
    // Create form container
    gen_widget_container!(
        master,
        container_id: config.content_form_wrapper,
        color: color::TRANSPARENT,
        width: size.0 - PRESET_SETTINGS_MODAL_CONTENT_IMAGE_WIDTH - (2.0 * PRESET_SETTINGS_MODAL_CONTENT_SEPARATOR_MARGIN_SIDES),
        height: PRESET_SETTINGS_MODAL_CONTENT_FORM_FIELD_COUNT * PRESET_SETTINGS_MODAL_CONTENT_FORM_FIELD_HEIGHT_PADDED - PRESET_SETTINGS_MODAL_CONTENT_FORM_FIELD_HEIGHT_PADDED / 2.0,
        positions: mid_right_of[
            config.content_wrapper,
        ]
    );

    draw_field(
        0,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-preset-age"),
            value_text: preset_age_to_locale(&config.preset_settings.age),
            ids: config.field_age_ids,
        },
    );

    draw_field(
        1,
        master,
        config,
        Field {
            label_text: APP_I18N.t("modal-preset-size"),
            value_text: format!(
                "{} {}",
                config.preset_settings.size,
                APP_I18N.t("telemetry-unit-centimeters")
            ),
            ids: config.field_height_ids,
        },
    );
}

fn draw_field<'a>(index: usize, master: &mut ControlWidget<'a>, config: &Config, field: Field) {
    // Generate label
    gen_widget_label_form!(
        master,
        text_id: field.ids.0,
        value: &field.label_text,
        positions: top_left_with_margins_on[
            config.content_form_wrapper, index as f64 * PRESET_SETTINGS_MODAL_CONTENT_FORM_FIELD_HEIGHT_PADDED, 0.0,
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
        changed: false,
        positions: top_left_with_margins_on[
            field.ids.0,
            MODAL_BUTTON_NAVIGATE_LEFT_ALIGN_TOP,
            PRESET_SETTINGS_MODAL_CONTENT_FORM_PADDING_LEFT,
        ]
    );
}
