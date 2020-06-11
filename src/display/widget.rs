// MakAir
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::cmp::{max, min};

use conrod_core::widget::Id as WidgetId;
use conrod_core::{
    color::{self, Color},
    widget::{
        self, canvas, id::List, primitive::shape::Style, rounded_rectangle::RoundedRectangle,
    },
    Colorable, Positionable, Sizeable, Widget,
};

use telemetry::alarm::AlarmCode;
use telemetry::structures::AlarmPriority;

use crate::chip::{
    settings::trigger_inspiratory::{TriggerInspiratory, TriggerInspiratoryState},
    ChipState,
};
use crate::config::environment::*;
use crate::physics::pressure::process_max_allowed_pressure;
use crate::physics::types::DataPressure;
use crate::APP_I18N;

use super::fonts::Fonts;

pub struct BackgroundWidgetConfig {
    color: conrod_core::color::Color,
    id: WidgetId,
}

impl BackgroundWidgetConfig {
    pub fn new(color: conrod_core::color::Color, id: WidgetId) -> BackgroundWidgetConfig {
        BackgroundWidgetConfig { color, id }
    }
}

pub struct LayoutWidgetConfig {
    parent: WidgetId,
    top: f64,
    height: f64,
    layout: WidgetId,
}

impl LayoutWidgetConfig {
    pub fn new(parent: WidgetId, top: f64, height: f64, layout: WidgetId) -> LayoutWidgetConfig {
        LayoutWidgetConfig {
            parent,
            top,
            height,
            layout,
        }
    }
}

pub struct LayoutConfig {
    header: LayoutWidgetConfig,
    body: LayoutWidgetConfig,
    footer: LayoutWidgetConfig,
}

impl LayoutConfig {
    pub fn new(
        header: LayoutWidgetConfig,
        body: LayoutWidgetConfig,
        footer: LayoutWidgetConfig,
    ) -> LayoutConfig {
        LayoutConfig {
            header,
            body,
            footer,
        }
    }
}

pub struct BrandingWidgetConfig<'a> {
    parent: WidgetId,
    version_firmware: &'a str,
    version_control: &'a str,
    width: f64,
    height: f64,
    image: conrod_core::image::Id,
    ids: (WidgetId, WidgetId, WidgetId),
}

pub struct StatusWidgetConfig<'a> {
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

pub struct HeartbeatWidgetConfig<'a> {
    data_pressure: &'a DataPressure,
    peak_command: u8,
    container: WidgetId,
    ground: WidgetId,
    surround: WidgetId,
    inner: WidgetId,
}

pub struct TelemetryWidgetContainerConfig {
    width: f64,
    height: f64,
    parent: WidgetId,
    id: WidgetId,
}

impl TelemetryWidgetContainerConfig {
    pub fn new(
        width: f64,
        height: f64,
        parent: WidgetId,
        id: WidgetId,
    ) -> TelemetryWidgetContainerConfig {
        TelemetryWidgetContainerConfig {
            width,
            height,
            parent,
            id,
        }
    }
}

pub struct TelemetryWidgetConfig {
    pub title: String,
    pub value_measured: Option<String>,
    pub value_target: Option<String>,
    pub value_arrow: conrod_core::image::Id,
    pub unit: String,
    pub ids: (
        WidgetId,
        WidgetId,
        WidgetId,
        WidgetId,
        WidgetId,
        WidgetId,
        Option<WidgetId>,
    ),
    pub x_position: f64,
    pub y_position: f64,
    pub background_color: Color,
    pub width: f64,
    pub height: f64,
}

pub struct GraphWidgetConfig {
    width: f64,
    height: f64,
    image: conrod_core::image::Id,
    parent: WidgetId,
    id: WidgetId,
}

impl<'a> BrandingWidgetConfig<'a> {
    pub fn new(
        parent: WidgetId,
        version_firmware: &'a str,
        version_control: &'a str,
        width: f64,
        height: f64,
        image: conrod_core::image::Id,
        ids: (WidgetId, WidgetId, WidgetId),
    ) -> BrandingWidgetConfig<'a> {
        BrandingWidgetConfig {
            parent,
            version_firmware,
            version_control,
            width,
            height,
            image,
            ids,
        }
    }
}

#[allow(clippy::too_many_arguments)]
impl<'a> StatusWidgetConfig<'a> {
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
    ) -> StatusWidgetConfig<'a> {
        StatusWidgetConfig {
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

impl<'a> HeartbeatWidgetConfig<'a> {
    pub fn new(
        data_pressure: &'a DataPressure,
        peak_command: u8,
        container: WidgetId,
        ground: WidgetId,
        surround: WidgetId,
        inner: WidgetId,
    ) -> HeartbeatWidgetConfig<'a> {
        HeartbeatWidgetConfig {
            data_pressure,
            peak_command,
            container,
            ground,
            surround,
            inner,
        }
    }
}

impl GraphWidgetConfig {
    pub fn new(
        width: f64,
        height: f64,
        image: conrod_core::image::Id,
        parent: WidgetId,
        id: WidgetId,
    ) -> GraphWidgetConfig {
        GraphWidgetConfig {
            width,
            height,
            image,
            parent,
            id,
        }
    }
}

pub struct ErrorWidgetConfig {
    error: String,
    id: WidgetId,
}

impl ErrorWidgetConfig {
    pub fn new(error: String, id: WidgetId) -> ErrorWidgetConfig {
        ErrorWidgetConfig { error, id }
    }
}

pub struct ModalWidgetConfig {
    pub parent: WidgetId,
    pub background: WidgetId,
    pub container_borders: WidgetId,
    pub container: WidgetId,
    pub validate: Option<(WidgetId, WidgetId)>,
    pub width: f64,
    pub height: f64,
    pub padding: Option<f64>,
}

pub struct StopWidgetConfig {
    pub container: WidgetId,
    pub title: WidgetId,
    pub message: WidgetId,
}

pub struct NoDataWidgetConfig {
    id: WidgetId,
}

impl NoDataWidgetConfig {
    pub fn new(id: WidgetId) -> NoDataWidgetConfig {
        NoDataWidgetConfig { id }
    }
}

pub struct InitializingWidgetConfig {
    id: WidgetId,
    width: f64,
    height: f64,
    image: conrod_core::image::Id,
}

impl InitializingWidgetConfig {
    pub fn new(
        id: WidgetId,
        width: f64,
        height: f64,
        image: conrod_core::image::Id,
    ) -> InitializingWidgetConfig {
        InitializingWidgetConfig {
            id,
            width,
            height,
            image,
        }
    }
}

pub struct AlarmsWidgetConfig<'a> {
    pub parent: WidgetId,
    pub container: WidgetId,
    pub title: WidgetId,
    pub empty: WidgetId,
    pub alarm_widgets: &'a List,
    pub alarm_codes_containers: &'a List,
    pub alarm_codes: &'a List,
    pub alarm_messages_containers: &'a List,
    pub alarm_messages: &'a List,
    pub alarms: &'a [(AlarmCode, AlarmPriority)],
}

pub struct TriggerInspiratoryWidgetConfig<'a> {
    pub width: f64,
    pub height: f64,
    pub trigger_inspiratory_settings: &'a TriggerInspiratory,
    pub status_container_parent: WidgetId,
    pub status_container_widget: WidgetId,
    pub status_enabled_text_widget: WidgetId,
    pub status_enabled_button_widget: WidgetId,
    pub status_enabled_button_text_widget: WidgetId,

    pub inspiratory_offset_container_parent: WidgetId,
    pub inspiratory_offset_text_widget: WidgetId,
    pub inspiratory_offset_less_button_widget: WidgetId,
    pub inspiratory_offset_less_button_text_widget: WidgetId,
    pub inspiratory_offset_more_button_widget: WidgetId,
    pub inspiratory_offset_more_button_text_widget: WidgetId,
    pub inspiratory_offset_value_widget: WidgetId,
}

pub struct ExpRatioSettingsWidgetConfig<'a> {
    pub width: f64,
    pub height: f64,
    pub trigger_inspiratory_settings: &'a TriggerInspiratory,
    pub exp_ratio_container_parent: WidgetId,
    pub exp_ratio_container_widget: WidgetId,
    pub exp_ratio_text_widget: WidgetId,
    pub exp_ratio_less_button_widget: WidgetId,
    pub exp_ratio_less_button_text_widget: WidgetId,
    pub exp_ratio_more_button_widget: WidgetId,
    pub exp_ratio_more_button_text_widget: WidgetId,
    pub exp_ratio_value_widget: WidgetId,
}

pub struct TriggerInspiratoryOverview<'a> {
    pub parent: WidgetId,
    pub container: WidgetId,
    pub title_widget: WidgetId,
    pub status_widget: WidgetId,
    pub inspiration_trigger_offset_widget: WidgetId,
    pub expiratory_term_widget: WidgetId,
    pub plateau_duration_widget: WidgetId,
    pub width: f64,
    pub height: f64,
    pub x_position: f64,
    pub y_position: f64,
    pub background_color: Color,
    pub trigger_inspiratory_settings: &'a TriggerInspiratory,
}

pub enum ControlWidgetType<'a> {
    Alarms(AlarmsWidgetConfig<'a>),
    Background(BackgroundWidgetConfig),
    Error(ErrorWidgetConfig),
    Branding(BrandingWidgetConfig<'a>),
    Status(StatusWidgetConfig<'a>),
    Heartbeat(HeartbeatWidgetConfig<'a>),
    Initializing(InitializingWidgetConfig),
    Graph(GraphWidgetConfig),
    Modal(ModalWidgetConfig),
    NoData(NoDataWidgetConfig),
    Stop(StopWidgetConfig),
    TelemetryContainer(TelemetryWidgetContainerConfig),
    Telemetry(TelemetryWidgetConfig),
    Layout(LayoutConfig),
    TriggerInspiratorySettings(TriggerInspiratoryWidgetConfig<'a>),
    TriggerInspiratoryOverview(TriggerInspiratoryOverview<'a>),
    ExpRatioSettings(ExpRatioSettingsWidgetConfig<'a>),
}

pub struct ControlWidget<'a> {
    ui: conrod_core::UiCell<'a>,
    fonts: &'a Fonts,
}

impl<'a> ControlWidget<'a> {
    pub fn new(ui: conrod_core::UiCell<'a>, fonts: &'a Fonts) -> ControlWidget<'a> {
        ControlWidget { ui, fonts }
    }

    pub fn render(&mut self, widget_type: ControlWidgetType<'a>) -> f64 {
        match widget_type {
            ControlWidgetType::Alarms(config) => self.alarms(config),
            ControlWidgetType::Background(config) => self.background(config),
            ControlWidgetType::Error(config) => self.error(config),
            ControlWidgetType::Branding(config) => self.branding(config),
            ControlWidgetType::Status(config) => self.status(config),
            ControlWidgetType::Heartbeat(config) => self.heartbeat(config),
            ControlWidgetType::Initializing(config) => self.initializing(config),
            ControlWidgetType::Graph(config) => self.graph(config),
            ControlWidgetType::Modal(config) => self.modal(config),
            ControlWidgetType::NoData(config) => self.no_data(config),
            ControlWidgetType::Stop(config) => self.stop(config),
            ControlWidgetType::TelemetryContainer(config) => {
                self.telemetry_widgets_container(config)
            }
            ControlWidgetType::Telemetry(config) => self.telemetry_widget(config),
            ControlWidgetType::Layout(config) => self.layout(config),
            ControlWidgetType::TriggerInspiratorySettings(config) => {
                self.trigger_inspiratory_settings(config)
            }
            ControlWidgetType::TriggerInspiratoryOverview(config) => {
                self.trigger_inspiratory_overview(config)
            }
            ControlWidgetType::ExpRatioSettings(config) => self.exp_ratio_settings(config),
        }
    }

    fn alarms(&mut self, config: AlarmsWidgetConfig) -> f64 {
        // Rebuild alarms that should go for display, and count their total
        // Notice: ignored alarm codes are used in other more specific places, eg. code \
        //   31 for battery power usage indicator.
        let alarms_for_display = config
            .alarms
            .iter()
            .filter(|&alarm| !DISPLAY_ALARM_CODE_IGNORES.contains(&alarm.0.code()))
            .collect::<Vec<_>>();
        let alarms_count = alarms_for_display.len();

        let container_width = if alarms_count == 0 {
            DISPLAY_ALARM_CONTAINER_WIDTH_NO_ALARMS
        } else {
            DISPLAY_ALARM_CONTAINER_WIDTH_ALARMS
        };

        let container_margin_left = if alarms_count == 0 { 10.0 } else { -100.0 };

        let container_height = (max(1, alarms_count) as f64) * DISPLAY_ALARM_MESSAGE_HEIGHT
            + 2.0 * DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL
            + (alarms_count as f64) * DISPLAY_ALARM_MESSAGE_SPACING_TOP_INNER;

        // Draw container box
        let container_color = if alarms_count > 0 {
            Color::Rgba(42.0 / 255.0, 42.0 / 255.0, 42.0 / 255.0, 0.96)
        } else {
            Color::Rgba(17.0 / 255.0, 17.0 / 255.0, 17.0 / 255.0, 0.96)
        };

        RoundedRectangle::fill_with(
            [container_width, container_height],
            DISPLAY_ROUNDED_RECTANGLES_ROUND,
            container_color,
        )
        .right_from(config.parent, container_margin_left)
        .set(config.container, &mut self.ui);

        // Draw text
        // Notice: the first text layer needs to be positionned using relative coordinates, and \
        //   cannot be positionned using a 'mid' auto coordinate, as this has been seen to center \
        //   vertically with a slight offset, which would make the text look uncentered to the \
        //   human eye.
        let mut text_style = conrod_core::widget::primitive::text::Style::default();

        text_style.font_id = Some(Some(self.fonts.bold));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some(14);

        widget::text::Text::new(&APP_I18N.t("alarms-title"))
            .with_style(text_style)
            .top_left_with_margins_on(
                config.container,
                DISPLAY_ALARM_CONTAINER_PADDING_TOP,
                DISPLAY_ALARM_CONTAINER_PADDING_LEFT,
            )
            .set(config.title, &mut self.ui);

        // Append all alarms?
        // Notice: only draw alarms box if there are active alarms
        if alarms_count > 0 {
            for x in 0..alarms_count {
                let (code, alarm) = alarms_for_display.get(x).unwrap();

                self.alarm(&config, *code, alarm, x);
            }
        } else {
            widget::text::Text::new(&APP_I18N.t("alarms-empty"))
                .color(Color::Rgba(1.0, 1.0, 1.0, 0.5))
                .font_size(12)
                .right_from(config.title, 42.0)
                .set(config.empty, &mut self.ui);
        }

        0 as _
    }

    fn alarm(
        &mut self,
        config: &AlarmsWidgetConfig,
        code: AlarmCode,
        alarm_priority: &AlarmPriority,
        index: usize,
    ) {
        let mut style = canvas::Style::default();

        style.border = Some(0.0);
        style.border_color = Some(color::TRANSPARENT);
        style.color = Some(color::TRANSPARENT);

        let from_top = if index == 0 {
            DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL
        } else {
            DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL
                + index as f64
                    * (DISPLAY_ALARM_MESSAGE_HEIGHT + DISPLAY_ALARM_MESSAGE_SPACING_TOP_INNER)
        } + DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL_OFFSET;

        canvas::Canvas::new()
            .with_style(style)
            .y_place_on(
                config.container,
                conrod_core::position::Place::End(Some(from_top)),
            )
            .right_from(config.title, 15.0)
            .set(config.alarm_widgets[index], &mut self.ui);

        self.alarm_code(&config, code, alarm_priority, index);
        self.alarm_message(&config, code, alarm_priority, index);
    }

    fn alarm_code_color(&self, alarm_priority: &AlarmPriority) -> Color {
        match alarm_priority {
            AlarmPriority::High => Color::Rgba(1.0, 0.0 / 255.0, 3.0 / 255.0, 1.0),
            AlarmPriority::Medium => Color::Rgba(1.0, 135.0 / 255.0, 0.0, 1.0),
            AlarmPriority::Low => Color::Rgba(1.0, 195.0 / 255.0, 0.0, 1.0),
        }
    }

    fn alarm_message_color(&self, alarm_priority: &AlarmPriority) -> Color {
        match alarm_priority {
            AlarmPriority::High => Color::Rgba(180.0 / 255.0, 24.0 / 255.0, 28.0 / 255.0, 1.0),
            AlarmPriority::Medium => Color::Rgba(189.0 / 255.0, 93.0 / 255.0, 0.0, 1.0),
            AlarmPriority::Low => Color::Rgba(174.0 / 255.0, 133.0 / 255.0, 0.0, 1.0),
        }
    }

    fn alarm_code(
        &mut self,
        config: &AlarmsWidgetConfig,
        alarm_code: AlarmCode,
        alarm_priority: &AlarmPriority,
        index: usize,
    ) {
        let color = self.alarm_code_color(alarm_priority);

        // Draw canvas
        let mut style = canvas::Style::default();

        style.border = Some(0.0);
        style.border_color = Some(color::TRANSPARENT);
        style.color = Some(color);

        widget::Canvas::new()
            .with_style(style)
            .w_h(DISPLAY_ALARM_CODE_WIDTH, DISPLAY_ALARM_CODE_HEIGHT)
            .x_place_on(
                config.alarm_widgets[index],
                conrod_core::position::Place::Start(None),
            )
            .set(config.alarm_codes_containers[index], &mut self.ui);

        // Draw text
        let mut text_style = conrod_core::widget::primitive::text::Style::default();

        text_style.font_id = Some(Some(self.fonts.bold));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some(24);

        widget::text::Text::new(&format!("{}", alarm_code.code()))
            .with_style(text_style)
            .mid_top_of(config.alarm_codes_containers[index])
            .set(config.alarm_codes[index], &mut self.ui);
    }

    fn alarm_message(
        &mut self,
        config: &AlarmsWidgetConfig,
        code: AlarmCode,
        alarm_priority: &AlarmPriority,
        index: usize,
    ) {
        let color = self.alarm_message_color(alarm_priority);

        let mut style = canvas::Style::default();

        style.border = Some(0.0);
        style.border_color = Some(color::TRANSPARENT);
        style.color = Some(color);

        widget::Canvas::new()
            .with_style(style)
            .w_h(DISPLAY_ALARM_MESSAGE_WIDTH, DISPLAY_ALARM_MESSAGE_HEIGHT)
            .x_place_on(
                config.alarm_widgets[index],
                conrod_core::position::Place::Start(Some(DISPLAY_ALARM_CODE_WIDTH)),
            )
            .set(config.alarm_messages_containers[index], &mut self.ui);

        widget::text::Text::new(&code.description())
            .color(color::WHITE)
            .font_size(24)
            .top_left_with_margins_on(config.alarm_messages_containers[index], 0.0, 5.0)
            .set(config.alarm_messages[index], &mut self.ui);
    }

    fn background(&mut self, config: BackgroundWidgetConfig) -> f64 {
        widget::Canvas::new()
            .color(config.color)
            .set(config.id, &mut self.ui);

        0 as _
    }

    fn branding(&mut self, config: BrandingWidgetConfig) -> f64 {
        widget::rectangle::Rectangle::fill_with([config.width, config.height], color::TRANSPARENT)
            .top_left_with_margins_on(
                config.parent,
                BRANDING_IMAGE_MARGIN_TOP,
                BRANDING_IMAGE_MARGIN_LEFT,
            )
            .set(config.ids.0, &mut self.ui);

        // Display branding image
        widget::Image::new(config.image)
            .w_h(config.width, config.height)
            .top_left_of(config.ids.0)
            .set(config.ids.1, &mut self.ui);

        // Display branding text
        let branding_text = format!("F{} | C{}", config.version_firmware, config.version_control);

        widget::Text::new(&branding_text)
            .color(color::WHITE.with_alpha(0.45))
            .top_left_with_margins_on(
                config.parent,
                BRANDING_TEXT_MARGIN_TOP,
                BRANDING_TEXT_MARGIN_LEFT,
            )
            .font_size(10)
            .set(config.ids.2, &mut self.ui);

        config.width
    }

    fn status(&mut self, config: StatusWidgetConfig) -> f64 {
        let (box_height, box_width) = (STATUS_WRAPPER_HEIGHT / 2.0, STATUS_WRAPPER_WIDTH);

        // Check whether chip state is currently in stopped mode or active (running)
        let is_unit_stopped = config.chip_state == &ChipState::Stopped;

        // Check whether power is currently on AC or battery
        // Notice: the telemetry library reports this as an alarm
        let is_battery_powered = config
            .alarms
            .iter()
            .any(|alarm| alarm.0.code() == STATUS_ALARM_CODE_POWER_BATTERY);

        // Render canvas
        let mut wrapper_style = canvas::Style::default();

        wrapper_style.color = Some(Color::Rgba(52.0 / 255.0, 52.0 / 255.0, 52.0 / 255.0, 1.0));
        wrapper_style.border = Some(0.0);
        wrapper_style.border_color = Some(color::TRANSPARENT);

        canvas::Canvas::new()
            .with_style(wrapper_style)
            .w_h(STATUS_WRAPPER_WIDTH, STATUS_WRAPPER_HEIGHT)
            .top_right_with_margins_on(
                config.container,
                STATUS_WRAPPER_MARGIN_TOP,
                STATUS_WRAPPER_MARGIN_RIGHT,
            )
            .set(config.wrapper, &mut self.ui);

        // Display unit status text
        let mut unit_box_style = canvas::Style::default();
        let mut unit_text_style = conrod_core::widget::primitive::text::Style::default();

        unit_text_style.font_id = Some(Some(self.fonts.bold));
        unit_text_style.color = Some(color::WHITE);
        unit_text_style.font_size = Some(11);

        if is_unit_stopped {
            unit_box_style.color =
                Some(Color::Rgba(180.0 / 255.0, 24.0 / 255.0, 28.0 / 255.0, 1.0));
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

        canvas::Canvas::new()
            .with_style(unit_box_style)
            .w_h(box_width, box_height)
            .top_left_of(config.wrapper)
            .set(config.unit_box, &mut self.ui);

        widget::text::Text::new(&unit_text_value)
            .with_style(unit_text_style)
            .mid_top_with_margin_on(config.unit_box, STATUS_BOX_TEXT_MARGIN_TOP)
            .set(config.unit_text, &mut self.ui);

        if let Some(save_icon_id) = config.save_icon_id {
            widget::image::Image::new(save_icon_id)
                .w_h(15.0, 15.0)
                .right_from(config.unit_text, 3.0)
                .set(config.save_icon, &mut self.ui);
        }

        // Display power status text
        let mut power_box_style = canvas::Style::default();
        let mut power_text_style = conrod_core::widget::primitive::text::Style::default();

        power_text_style.font_id = Some(Some(self.fonts.bold));
        power_text_style.color = Some(color::WHITE);
        power_text_style.font_size = Some(11);

        if is_battery_powered {
            power_box_style.color = Some(Color::Rgba(208.0 / 255.0, 92.0 / 255.0, 0.0, 1.0));
        } else {
            power_box_style.color = Some(color::TRANSPARENT);
        }

        power_box_style.border = Some(0.0);
        power_box_style.border_color = Some(color::TRANSPARENT);

        canvas::Canvas::new()
            .with_style(power_box_style)
            .w_h(box_width, box_height)
            .bottom_left_of(config.wrapper)
            .set(config.power_box, &mut self.ui);

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
            .set(config.power_text, &mut self.ui);

        STATUS_WRAPPER_WIDTH
    }

    fn heartbeat(&mut self, config: HeartbeatWidgetConfig) -> f64 {
        // Convert diameters to radius
        let (ground_radius, surround_radius) = (
            HEARTBEAT_GROUND_DIAMETER / 2.0,
            HEARTBEAT_SURROUND_DIAMETER / 2.0,
        );

        // #1: Create surround circle
        let surround_line_style = widget::primitive::line::Style::solid()
            .color(Color::Rgba(
                153.0 / 255.0,
                153.0 / 255.0,
                153.0 / 255.0,
                1.0,
            ))
            .thickness(HEARTBEAT_SURROUND_THICKNESS);

        widget::primitive::shape::circle::Circle::outline_styled(
            surround_radius,
            surround_line_style,
        )
        .top_right_with_margins_on(
            config.container,
            HEARTBEAT_SURROUND_MARGIN_TOP,
            HEARTBEAT_SURROUND_MARGIN_RIGHT,
        )
        .set(config.surround, &mut self.ui);

        // #2: Create inner circle
        let last_pressure = if let Some(last_pressure_inner) = config.data_pressure.get(0) {
            // Convert high-precision point in mmH20 back to cmH20 (which measurements & targets \
            //   both use)
            last_pressure_inner.1 / TELEMETRY_POINTS_PRECISION_DIVIDE
        } else {
            0
        };

        let pressure_alert_threshold = process_max_allowed_pressure(config.peak_command) as f64;

        let last_pressure_ratio = last_pressure as f64 / pressure_alert_threshold;
        let last_pressure_radius = surround_radius * last_pressure_ratio;

        let inner_radius = min(
            max(last_pressure_radius as u16, ground_radius as u16 + 1),
            surround_radius as u16 + HEARTBEAT_INNER_MAX_OVERFLOW,
        ) as f64;

        let inner_color = if last_pressure_radius >= surround_radius {
            Color::Rgba(184.0 / 255.0, 1.0 / 255.0, 24.0 / 255.0, 1.0)
        } else {
            color::WHITE
        };

        widget::primitive::shape::circle::Circle::fill_with(inner_radius, inner_color)
            .middle_of(config.surround)
            .set(config.inner, &mut self.ui);

        // #3: Create ground circle
        let ground_color = if last_pressure_radius >= surround_radius {
            Color::Rgba(204.0 / 255.0, 204.0 / 255.0, 204.0 / 255.0, 1.0)
        } else {
            Color::Rgba(116.0 / 255.0, 116.0 / 255.0, 116.0 / 255.0, 1.0)
        };

        widget::primitive::shape::circle::Circle::fill_with(ground_radius, ground_color)
            .middle_of(config.surround)
            .set(config.ground, &mut self.ui);

        HEARTBEAT_GROUND_DIAMETER
    }

    fn graph(&mut self, config: GraphWidgetConfig) -> f64 {
        widget::Image::new(config.image)
            .w_h(config.width, config.height)
            .top_left_of(config.parent)
            .set(config.id, &mut self.ui);

        config.width
    }

    fn telemetry_widgets_container(&mut self, config: TelemetryWidgetContainerConfig) -> f64 {
        widget::rectangle::Rectangle::fill_with([config.width, config.height], color::WHITE)
            .right_from(config.parent, 0.0)
            .set(config.id, &mut self.ui);

        0.0
    }

    fn telemetry_widget(&mut self, config: TelemetryWidgetConfig) -> f64 {
        // Create rounded rectangle
        widget::rectangle::Rectangle::fill_with(
            [config.width, config.height],
            config.background_color,
        )
        .bottom_left_with_margins_on(config.ids.0, config.y_position, config.x_position)
        .set(config.ids.1, &mut self.ui);

        // Create title text
        widget::Text::new(&config.title)
            .color(color::WHITE)
            .top_left_with_margins_on(config.ids.1, 8.0, TELEMETRY_WIDGET_PADDING_LEFT)
            .font_size(16)
            .set(config.ids.2, &mut self.ui);

        // Initiate text style for measured value
        let mut value_text_style = conrod_core::widget::primitive::text::Style::default();

        value_text_style.font_id = Some(Some(self.fonts.regular));
        value_text_style.color = Some(color::WHITE);
        value_text_style.font_size = Some(45);

        // Create value text
        // Notice: there are different drawing cases depending on values provided
        match (config.value_measured, config.value_target) {
            (Some(value_measured), Some(value_target)) => {
                // Initiate text sub-style for target value
                let mut target_text_style = conrod_core::widget::primitive::text::Style::default();

                target_text_style.font_id = Some(Some(self.fonts.regular));
                target_text_style.color = Some(color::WHITE);
                target_text_style.font_size = Some(35);

                // Draw measured value
                widget::Text::new(&value_measured)
                    .with_style(value_text_style)
                    .bottom_left_with_margins_on(config.ids.1, 10.0, TELEMETRY_WIDGET_PADDING_LEFT)
                    .set(config.ids.3, &mut self.ui);

                // Draw arrow
                widget::Image::new(config.value_arrow)
                    .w_h(TELEMETRY_ARROW_WIDTH as f64, TELEMETRY_ARROW_HEIGHT as f64)
                    .right_from(config.ids.3, TELEMETRY_ARROW_SPACING_SIDES)
                    .y_relative_to(config.ids.3, -3.0)
                    .set(config.ids.4, &mut self.ui);

                // Draw target value
                widget::Text::new(&format!("({})", value_target))
                    .with_style(target_text_style)
                    .right_from(config.ids.4, TELEMETRY_ARROW_SPACING_SIDES)
                    .y_relative_to(config.ids.3, -1.0)
                    .set(config.ids.5, &mut self.ui);
            }
            (Some(value_measured), None) => {
                // Draw measured value
                widget::Text::new(&value_measured)
                    .with_style(value_text_style)
                    .mid_left_with_margin_on(config.ids.1, TELEMETRY_WIDGET_PADDING_LEFT)
                    .set(config.ids.3, &mut self.ui);
            }
            (None, Some(value_target)) => {
                // Draw target value
                widget::Text::new(&value_target)
                    .with_style(value_text_style)
                    .bottom_left_with_margins_on(config.ids.1, 10.0, TELEMETRY_WIDGET_PADDING_LEFT)
                    .set(config.ids.5, &mut self.ui);
            }
            _ => {}
        }

        if let Some(unit_id) = config.ids.6 {
            // Create unit text
            widget::Text::new(&config.unit)
                .color(color::WHITE.with_alpha(0.2))
                .bottom_left_with_margins_on(config.ids.1, 10.0, TELEMETRY_WIDGET_PADDING_LEFT)
                .font_size(12)
                .set(unit_id, &mut self.ui);
        }

        TELEMETRY_WIDGET_SIZE_WIDTH
    }

    fn error(&mut self, config: ErrorWidgetConfig) -> f64 {
        let mut text_style = conrod_core::widget::primitive::text::Style::default();

        text_style.font_id = Some(Some(self.fonts.bold));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some(30);

        widget::Text::new(&format!("{}\n{}", APP_I18N.t("error-title"), config.error)) // using \n instead of the wrap methods because I couldn't make them work
            .color(color::WHITE)
            .align_top() // Aligned to top otherwise I can't make the line breaks work
            .with_style(text_style)
            .set(config.id, &mut self.ui);

        0 as _
    }

    fn modal(&mut self, config: ModalWidgetConfig) -> f64 {
        let mut style = canvas::Style::default();

        style.color = Some(Color::Rgba(0.0, 0.0, 0.0, 0.75));
        style.border = Some(0.0);
        style.border_color = Some(color::TRANSPARENT);

        canvas::Canvas::new()
            .with_style(style)
            .w_h(
                DISPLAY_WINDOW_SIZE_WIDTH as _,
                DISPLAY_WINDOW_SIZE_HEIGHT as _,
            )
            .x_y(0.0, 0.0)
            .set(config.background, &mut self.ui);

        let container_borders_style = Style::Fill(Some(Color::Rgba(
            81.0 / 255.0,
            81.0 / 255.0,
            81.0 / 255.0,
            1.0,
        )));
        RoundedRectangle::styled(
            [config.width + 5.0, config.height + 5.0],
            DISPLAY_ROUNDED_RECTANGLES_ROUND,
            container_borders_style,
        )
        .middle_of(config.parent)
        .set(config.container_borders, &mut self.ui);

        let mut container_style = canvas::Style::default();
        container_style.color = Some(Color::Rgba(26.0 / 255.0, 26.0 / 255.0, 26.0 / 255.0, 1.0));
        container_style.border = Some(0.0);
        container_style.border_color = Some(color::TRANSPARENT);

        let mut container = canvas::Canvas::new()
            .with_style(container_style)
            .w_h(config.width, config.height)
            .middle_of(config.container_borders);

        if let Some(padding) = config.padding {
            container = container.pad(padding);
        }

        container.set(config.container, &mut self.ui);

        if let Some((validate_button, validate_text)) = config.validate {
            let button_style = widget::primitive::shape::Style::Fill(Some(color::WHITE));

            widget::RoundedRectangle::styled(
                [60.0, MODAL_VALIDATE_BUTTON_HEIGHT],
                15.0,
                button_style,
            )
            .bottom_right_of(config.container)
            .set(validate_button, &mut self.ui);

            let mut validate_text_style = widget::text::Style::default();
            validate_text_style.font_id = Some(Some(self.fonts.regular));
            validate_text_style.color = Some(color::BLACK);
            validate_text_style.font_size = Some(20);

            widget::Text::new("Save")
                .with_style(validate_text_style)
                .mid_top_with_margin_on(validate_button, 2.0)
                .set(validate_text, &mut self.ui);
        }

        0 as _
    }

    fn stop(&mut self, config: StopWidgetConfig) -> f64 {
        let mut title_style = widget::text::Style::default();
        title_style.color = Some(color::WHITE);
        title_style.font_size = Some(19);
        title_style.font_id = Some(Some(self.fonts.bold));

        widget::text::Text::new(&APP_I18N.t("stop-title"))
            .with_style(title_style)
            .mid_top_with_margin_on(config.container, DISPLAY_STOPPED_MESSAGE_PADDING_TOP)
            .set(config.title, &mut self.ui);

        let mut message_style = widget::text::Style::default();
        message_style.color = Some(Color::Rgba(1.0, 1.0, 1.0, 0.75));
        message_style.font_size = Some(14);
        message_style.font_id = Some(Some(self.fonts.regular));

        widget::text::Text::new(&APP_I18N.t("stop-description"))
            .with_style(message_style)
            .mid_bottom_with_margin_on(config.container, DISPLAY_STOPPED_MESSAGE_PADDING_BOTTOM)
            .set(config.message, &mut self.ui);

        0 as _
    }

    fn no_data(&mut self, config: NoDataWidgetConfig) -> f64 {
        let mut text_style = conrod_core::widget::primitive::text::Style::default();

        text_style.font_id = Some(Some(self.fonts.bold));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some(30);

        widget::Text::new(&APP_I18N.t("no-data-title"))
            .color(color::WHITE)
            .middle()
            .with_style(text_style)
            .set(config.id, &mut self.ui);
        0 as _
    }

    fn initializing(&mut self, config: InitializingWidgetConfig) -> f64 {
        widget::Image::new(config.image)
            .w_h(config.width, config.height)
            .middle()
            .set(config.id, &mut self.ui);

        0 as _
    }

    fn layout(&mut self, config: LayoutConfig) -> f64 {
        widget::Rectangle::fill_with(
            [DISPLAY_WINDOW_SIZE_WIDTH as _, config.body.height],
            color::TRANSPARENT,
        )
        .top_left_with_margins_on(config.body.parent, config.body.top, 0.0)
        .set(config.body.layout, &mut self.ui);

        widget::Rectangle::fill_with(
            [DISPLAY_WINDOW_SIZE_WIDTH as _, config.footer.height],
            color::TRANSPARENT,
        )
        .down_from(config.footer.parent, config.footer.top)
        .set(config.footer.layout, &mut self.ui);

        // This block is defined after the others because we want it to overflow and be on top of the screen
        widget::Rectangle::fill_with(
            [DISPLAY_WINDOW_SIZE_WIDTH as _, config.header.height],
            color::TRANSPARENT,
        )
        .top_left_of(config.header.parent)
        .set(config.header.layout, &mut self.ui);

        0.0
    }

    fn trigger_inspiratory_settings(&mut self, config: TriggerInspiratoryWidgetConfig) -> f64 {
        let sections_height = config.height / 2.0;
        let mut canvas_style = widget::canvas::Style::default();
        canvas_style.color = Some(color::TRANSPARENT);
        canvas_style.border = Some(0.0);

        widget::Canvas::new()
            .with_style(canvas_style)
            .w_h(config.width, sections_height)
            .top_left_of(config.status_container_parent)
            .set(config.status_container_widget, &mut self.ui);

        let mut status_text_style = widget::text::Style::default();
        status_text_style.font_id = Some(Some(self.fonts.regular));
        status_text_style.color = Some(color::WHITE);
        status_text_style.font_size = Some(20);

        widget::Text::new("Trigger inspiratory status:")
            .with_style(status_text_style)
            .top_left_of(config.status_container_widget)
            .set(config.status_enabled_text_widget, &mut self.ui);

        let status_label = match config.trigger_inspiratory_settings.state {
            TriggerInspiratoryState::Enabled => String::from("Enabled"),
            TriggerInspiratoryState::Disabled => String::from("Disabled"),
        };

        let status_style = widget::primitive::shape::Style::Fill(Some(color::WHITE));

        widget::RoundedRectangle::styled([200.0, 30.0], 15.0, status_style)
            .top_left_with_margins_on(config.status_container_widget, 0.0, 300.0)
            .set(config.status_enabled_button_widget, &mut self.ui);

        let mut status_button_text_style = widget::text::Style::default();
        status_button_text_style.font_id = Some(Some(self.fonts.regular));
        status_button_text_style.color = Some(color::BLACK);
        status_button_text_style.font_size = Some(20);

        widget::Text::new(&status_label)
            .with_style(status_button_text_style)
            .mid_top_with_margin_on(config.status_enabled_button_widget, 2.0)
            .set(config.status_enabled_button_text_widget, &mut self.ui);

        widget::Canvas::new()
            .with_style(canvas_style)
            .w_h(config.width, sections_height)
            .down_from(config.status_container_widget, 0.0)
            .set(config.inspiratory_offset_container_parent, &mut self.ui);

        let mut offset_text_style = widget::text::Style::default();
        offset_text_style.font_id = Some(Some(self.fonts.regular));
        offset_text_style.color = Some(color::WHITE);
        offset_text_style.font_size = Some(20);

        widget::Text::new("Inspiratory trigger offset:")
            .with_style(offset_text_style)
            .top_left_of(config.inspiratory_offset_container_parent)
            .set(config.inspiratory_offset_text_widget, &mut self.ui);

        let less_button_style = widget::primitive::shape::Style::Fill(Some(color::WHITE));

        widget::RoundedRectangle::styled([50.0, 30.0], 15.0, less_button_style)
            .top_left_with_margins_on(config.inspiratory_offset_container_parent, 0.0, 300.0)
            .set(config.inspiratory_offset_less_button_widget, &mut self.ui);

        let mut more_less_buttons_text_style = widget::text::Style::default();
        more_less_buttons_text_style.font_id = Some(Some(self.fonts.bold));
        more_less_buttons_text_style.color = Some(color::BLACK);
        more_less_buttons_text_style.font_size = Some(20);

        widget::Text::new("<")
            .with_style(more_less_buttons_text_style)
            .mid_top_with_margin_on(config.inspiratory_offset_less_button_widget, 2.0)
            .set(
                config.inspiratory_offset_less_button_text_widget,
                &mut self.ui,
            );

        let mut offset_value_style = widget::text::Style::default();
        offset_value_style.font_id = Some(Some(self.fonts.regular));
        offset_value_style.color = Some(color::WHITE);
        offset_value_style.font_size = Some(20);

        widget::Text::new(
            format!(
                "{} mmH2O",
                config
                    .trigger_inspiratory_settings
                    .inspiratory_trigger_offset
            )
            .as_str(),
        )
        .with_style(offset_value_style)
        .right_from(config.inspiratory_offset_less_button_widget, 20.0)
        .set(config.inspiratory_offset_value_widget, &mut self.ui);

        widget::RoundedRectangle::styled([50.0, 30.0], 15.0, less_button_style)
            .right_from(config.inspiratory_offset_value_widget, 20.0)
            .set(config.inspiratory_offset_more_button_widget, &mut self.ui);

        widget::Text::new(">")
            .with_style(more_less_buttons_text_style)
            .mid_top_with_margin_on(config.inspiratory_offset_more_button_widget, 2.0)
            .set(
                config.inspiratory_offset_more_button_text_widget,
                &mut self.ui,
            );

        0 as _
    }

    fn trigger_inspiratory_overview(&mut self, config: TriggerInspiratoryOverview) -> f64 {
        widget::rectangle::Rectangle::fill_with(
            [config.width, config.height],
            config.background_color,
        )
        .bottom_left_with_margins_on(config.parent, config.y_position, config.x_position)
        .set(config.container, &mut self.ui);

        self.trigger_inspiratory_overview_title(&config);
        self.trigger_inspiratory_overview_status(&config);
        self.trigger_inspiratory_overview_offset(&config);

        0 as _
    }

    fn trigger_inspiratory_overview_title(&mut self, config: &TriggerInspiratoryOverview) {
        let mut text_style = widget::text::Style::default();
        text_style.font_id = Some(Some(self.fonts.regular));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some(20);

        widget::Text::new("Trigger")
            .with_style(text_style)
            .top_left_of(config.container)
            .set(config.title_widget, &mut self.ui);
    }

    fn trigger_inspiratory_overview_status(&mut self, config: &TriggerInspiratoryOverview) {
        let mut text_style = widget::text::Style::default();
        text_style.font_id = Some(Some(self.fonts.regular));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some(15);

        let status =
            if config.trigger_inspiratory_settings.state == TriggerInspiratoryState::Enabled {
                "Enabled".to_string()
            } else {
                "Disabled".to_string()
            };

        widget::Text::new(&format!("State: {}", status))
            .with_style(text_style)
            .down_from(config.title_widget, 20.0)
            .set(config.status_widget, &mut self.ui);
    }

    fn trigger_inspiratory_overview_offset(&mut self, config: &TriggerInspiratoryOverview) {
        let mut text_style = widget::text::Style::default();
        text_style.font_id = Some(Some(self.fonts.regular));
        text_style.color = Some(color::WHITE);
        text_style.font_size = Some(15);

        widget::Text::new(&format!(
            "Offset: {} mmH2O",
            config
                .trigger_inspiratory_settings
                .inspiratory_trigger_offset
        ))
        .with_style(text_style)
        .down_from(config.status_widget, 20.0)
        .set(config.inspiration_trigger_offset_widget, &mut self.ui);
    }

    fn exp_ratio_settings(&mut self, config: ExpRatioSettingsWidgetConfig) -> f64 {
        let mut canvas_style = widget::canvas::Style::default();
        canvas_style.color = Some(color::TRANSPARENT);
        canvas_style.border = Some(0.0);

        widget::Canvas::new()
            .with_style(canvas_style)
            .w_h(config.width, config.height)
            .top_left_of(config.exp_ratio_container_parent)
            .set(config.exp_ratio_container_widget, &mut self.ui);

        let mut plateau_text_style = widget::text::Style::default();
        plateau_text_style.font_id = Some(Some(self.fonts.regular));
        plateau_text_style.color = Some(color::WHITE);
        plateau_text_style.font_size = Some(20);

        widget::Text::new("Expiratory Term")
            .with_style(plateau_text_style)
            .top_left_of(config.exp_ratio_container_widget)
            .set(config.exp_ratio_text_widget, &mut self.ui);

        let less_button_style = widget::primitive::shape::Style::Fill(Some(color::WHITE));

        widget::RoundedRectangle::styled([50.0, 30.0], 15.0, less_button_style)
            .top_left_with_margins_on(config.exp_ratio_container_parent, 0.0, 300.0)
            .set(config.exp_ratio_less_button_widget, &mut self.ui);

        let mut more_less_buttons_text_style = widget::text::Style::default();
        more_less_buttons_text_style.font_id = Some(Some(self.fonts.bold));
        more_less_buttons_text_style.color = Some(color::BLACK);
        more_less_buttons_text_style.font_size = Some(20);

        widget::Text::new("<")
            .with_style(more_less_buttons_text_style)
            .mid_top_with_margin_on(config.exp_ratio_less_button_widget, 2.0)
            .set(config.exp_ratio_less_button_text_widget, &mut self.ui);

        let mut plateau_value_style = widget::text::Style::default();
        plateau_value_style.font_id = Some(Some(self.fonts.regular));
        plateau_value_style.color = Some(color::WHITE);
        plateau_value_style.font_size = Some(20);

        widget::Text::new(
            format!("{}", config.trigger_inspiratory_settings.expiratory_term).as_str(),
        )
        .with_style(plateau_value_style)
        .right_from(config.exp_ratio_less_button_widget, 20.0)
        .set(config.exp_ratio_value_widget, &mut self.ui);

        widget::RoundedRectangle::styled([50.0, 30.0], 15.0, less_button_style)
            .right_from(config.exp_ratio_value_widget, 20.0)
            .set(config.exp_ratio_more_button_widget, &mut self.ui);

        widget::Text::new(">")
            .with_style(more_less_buttons_text_style)
            .mid_top_with_margin_on(config.exp_ratio_more_button_widget, 2.0)
            .set(config.exp_ratio_more_button_text_widget, &mut self.ui);

        0.0
    }
}
