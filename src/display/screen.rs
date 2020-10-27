// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::color::{self, Color};
use conrod_core::widget::Id as WidgetId;

use telemetry::alarm::AlarmCode;
use telemetry::structures::{AlarmPriority, MachineStateSnapshot};

use crate::chip::{settings::trigger::Trigger, ChipState};
use crate::config::environment::*;
use crate::physics::types::DataPressure;
use crate::physics::units::{convert_mmh2o_to_cmh2o, ConvertMode};
use crate::widget::*;
use crate::APP_I18N;

use super::fonts::Fonts;
use super::identifiers::Ids;
use super::widget::{ControlWidget, ControlWidgetType};

pub struct Screen<'a> {
    ids: &'a Ids,
    machine_snapshot: Option<&'a MachineStateSnapshot>,
    ongoing_alarms: Option<&'a [(AlarmCode, AlarmPriority)]>,
    widgets: ControlWidget<'a>,
}

pub struct ScreenDataBranding<'a> {
    pub firmware_version: &'a str,
    pub image_id: conrod_core::image::Id,
    pub width: f64,
    pub height: f64,
}

pub struct ScreenDataStatus<'a> {
    pub battery_level: Option<u8>,
    pub chip_state: &'a ChipState,
    pub save_image_id: Option<conrod_core::image::Id>,
}

pub struct ScreenDataHeartbeat<'a> {
    pub data_pressure: &'a DataPressure,
}

pub struct ScreenDataGraph {
    pub image_id: conrod_core::image::Id,
    pub width: f64,
    pub height: f64,
}

pub struct ScreenDataTelemetry {
    pub arrow_image_id: conrod_core::image::Id,
}

pub struct ScreenBootLoader {
    pub image_id: conrod_core::image::Id,
    pub width: f64,
    pub height: f64,
}

impl<'a> Screen<'a> {
    pub fn new(
        ui: conrod_core::UiCell<'a>,
        ids: &'a Ids,
        fonts: &'a Fonts,
        machine_snapshot: Option<&'a MachineStateSnapshot>,
        ongoing_alarms: Option<&'a [(AlarmCode, AlarmPriority)]>,
    ) -> Screen<'a> {
        Screen {
            ids,
            machine_snapshot,
            ongoing_alarms,
            widgets: ControlWidget::new(ui, fonts),
        }
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_with_data(
        &mut self,
        branding_data: ScreenDataBranding<'a>,
        status_data: ScreenDataStatus<'a>,
        heartbeat_data: ScreenDataHeartbeat<'a>,
        graph_data: ScreenDataGraph,
        telemetry_data: ScreenDataTelemetry,
        trigger: &'a Trigger,
        trigger_open: bool,
        expiration_term_open: bool,
    ) {
        // Render common background
        self.render_background();
        self.render_layout();

        // Render top elements
        self.render_branding(
            branding_data.firmware_version,
            RUNTIME_VERSION,
            branding_data.image_id,
            branding_data.width,
            branding_data.height,
        );
        self.render_alarms();
        self.render_status(status_data);
        self.render_heartbeat(heartbeat_data);

        // Render middle elements
        self.render_graph(graph_data.image_id, graph_data.width, graph_data.height);

        // Render bottom elements
        self.render_telemetry(telemetry_data, trigger);

        if trigger_open {
            self.render_trigger_settings(trigger);
        } else if expiration_term_open {
            self.render_expiration_term_settings(trigger);
        }
    }

    pub fn render_background(&mut self) {
        self.widgets.render(ControlWidgetType::Background(
            background::BackgroundWidgetConfig::new(color::BLACK, self.ids.background),
        ));
    }

    pub fn render_layout(&mut self) {
        self.widgets
            .render(ControlWidgetType::Layout(layout::LayoutConfig::new(
                layout::LayoutWidgetConfig::new(
                    self.ids.background,
                    0.0,
                    LAYOUT_HEADER_SIZE_FULL_HEIGHT,
                    self.ids.layout_header,
                ),
                layout::LayoutWidgetConfig::new(
                    self.ids.background,
                    LAYOUT_HEADER_SIZE_HEIGHT,
                    LAYOUT_BODY_SIZE_HEIGHT,
                    self.ids.layout_body,
                ),
                layout::LayoutWidgetConfig::new(
                    self.ids.layout_body,
                    0.0,
                    LAYOUT_FOOTER_SIZE_HEIGHT,
                    self.ids.layout_footer,
                ),
            )));
    }

    pub fn render_branding(
        &mut self,
        version_firmware: &'a str,
        version_control: &'a str,
        image_id: conrod_core::image::Id,
        width: f64,
        height: f64,
    ) {
        self.widgets.render(ControlWidgetType::Branding(
            branding::BrandingWidgetConfig::new(
                self.ids.layout_header,
                version_firmware,
                version_control,
                width,
                height,
                image_id,
                (
                    self.ids.branding_container,
                    self.ids.branding_image,
                    self.ids.branding_text,
                ),
            ),
        ));
    }

    pub fn render_alarms(&mut self) {
        self.widgets
            .render(ControlWidgetType::Alarms(alarms::AlarmsWidgetConfig {
                parent: self.ids.branding_container,
                container: self.ids.alarm_container,
                title: self.ids.alarm_title,
                empty: self.ids.alarm_empty,
                alarm_widgets: &self.ids.alarm_alarms,
                alarm_codes_containers: &self.ids.alarm_codes_containers,
                alarm_codes: &self.ids.alarm_codes,
                alarm_messages_containers: &self.ids.alarm_messages_containers,
                alarm_messages: &self.ids.alarm_messages,
                alarms: self.ongoing_alarms.unwrap(),
            }));
    }

    pub fn render_status(&mut self, status_data: ScreenDataStatus<'a>) {
        self.widgets
            .render(ControlWidgetType::Status(status::StatusWidgetConfig::new(
                self.ids.layout_header,
                self.ids.status_wrapper,
                self.ids.status_unit_box,
                self.ids.status_unit_text,
                self.ids.status_power_box,
                self.ids.status_power_text,
                self.ids.status_save_icon,
                status_data.battery_level,
                status_data.chip_state,
                self.ongoing_alarms.unwrap(),
                status_data.save_image_id,
            )));
    }

    pub fn render_heartbeat(&mut self, heartbeat_data: ScreenDataHeartbeat<'a>) {
        self.widgets.render(ControlWidgetType::Heartbeat(
            heartbeat::HeartbeatWidgetConfig::new(
                heartbeat_data.data_pressure,
                self.machine_snapshot.unwrap().peak_command,
                self.ids.layout_header,
                self.ids.heartbeat_ground,
                self.ids.heartbeat_surround,
                self.ids.heartbeat_inner,
            ),
        ));
    }

    pub fn render_graph(&mut self, image_id: conrod_core::image::Id, width: f64, height: f64) {
        self.widgets
            .render(ControlWidgetType::Graph(graph::GraphWidgetConfig::new(
                width,
                height,
                image_id,
                self.ids.layout_body,
                self.ids.pressure_graph,
            )));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_stop(
        &mut self,
        branding_data: ScreenDataBranding<'a>,
        status_data: ScreenDataStatus<'a>,
        heartbeat_data: ScreenDataHeartbeat<'a>,
        graph_data: ScreenDataGraph,
        telemetry_data: ScreenDataTelemetry,
        trigger: &'a Trigger,
        trigger_open: bool,
        expiration_term_open: bool,
    ) {
        // Render regular data as background
        self.render_with_data(
            branding_data,
            status_data,
            heartbeat_data,
            graph_data,
            telemetry_data,
            trigger,
            trigger_open,
            expiration_term_open,
        );

        if !trigger_open && !expiration_term_open {
            self.render_modal(
                DISPLAY_STOPPED_MESSAGE_CONTAINER_WIDTH,
                DISPLAY_STOPPED_MESSAGE_CONTAINER_HEIGHT,
                None,
                None,
            );

            // Render stop layer
            self.widgets
                .render(ControlWidgetType::Stop(stop::StopWidgetConfig {
                    container: self.ids.modal_container,
                    title: self.ids.stopped_title,
                    message: self.ids.stopped_message,
                }));
        }
    }

    pub fn render_no_data(&mut self) {
        self.widgets
            .render(ControlWidgetType::NoData(no_data::NoDataWidgetConfig::new(
                self.ids.no_data,
            )));
    }

    pub fn render_error(&mut self, error: String) {
        self.render_background();

        self.widgets
            .render(ControlWidgetType::Error(error::ErrorWidgetConfig::new(
                error,
                self.ids.error,
            )));
    }

    pub fn render_initializing(&mut self, config: ScreenBootLoader) {
        self.render_background();

        self.widgets.render(ControlWidgetType::Initializing(
            initializing::InitializingWidgetConfig::new(
                self.ids.initializing_logo,
                config.width,
                config.height,
                config.image_id,
            ),
        ));
    }

    pub fn render_telemetry(&mut self, telemetry_data: ScreenDataTelemetry, trigger: &'a Trigger) {
        let machine_snapshot = self.machine_snapshot.unwrap();

        // Process shared values
        let widgets_right_width: f64 = (DISPLAY_WINDOW_SIZE_WIDTH - GRAPH_WIDTH) as f64;
        let widgets_right_height: f64 = GRAPH_HEIGHT as f64 / 3.0;

        // Initialize the pressure graph widget
        self.widgets.render(ControlWidgetType::TelemetryContainer(
            telemetry_container::TelemetryWidgetContainerConfig::new(
                widgets_right_width,
                DISPLAY_WINDOW_SIZE_HEIGHT as f64 - LAYOUT_HEADER_SIZE_HEIGHT,
                self.ids.pressure_graph,
                self.ids.telemetry_widgets_right,
            ),
        ));

        // Initialize the peak widget
        self.widgets.render(ControlWidgetType::Telemetry(
            telemetry_view::TelemetryWidgetConfig {
                title: APP_I18N.t("telemetry-label-peak"),
                value_measured: Some(
                    convert_mmh2o_to_cmh2o(
                        ConvertMode::Rounded,
                        machine_snapshot.previous_peak_pressure as f64,
                    )
                    .to_string(),
                ),
                value_target: Some(machine_snapshot.peak_command.to_string()),
                value_arrow: telemetry_data.arrow_image_id,
                unit: APP_I18N.t("telemetry-unit-cmh2o"),
                ids: (
                    self.ids.telemetry_widgets_right,
                    self.ids.peak_parent,
                    self.ids.peak_title,
                    self.ids.peak_value_measured,
                    self.ids.peak_value_arrow,
                    self.ids.peak_value_target,
                    Some(self.ids.peak_unit),
                ),
                x_position: 0.0,
                y_position: GRAPH_HEIGHT as f64 + LAYOUT_FOOTER_SIZE_HEIGHT - widgets_right_height,
                background_color: Color::Rgba(39.0 / 255.0, 66.0 / 255.0, 100.0 / 255.0, 1.0),
                width: widgets_right_width,
                height: widgets_right_height,
            },
        ));

        // Initialize the plateau widget
        self.widgets.render(ControlWidgetType::Telemetry(
            telemetry_view::TelemetryWidgetConfig {
                title: APP_I18N.t("telemetry-label-plateau"),
                value_measured: Some(
                    convert_mmh2o_to_cmh2o(
                        ConvertMode::Rounded,
                        machine_snapshot.previous_plateau_pressure as f64,
                    )
                    .to_string(),
                ),
                value_target: Some(machine_snapshot.plateau_command.to_string()),
                value_arrow: telemetry_data.arrow_image_id,
                unit: APP_I18N.t("telemetry-unit-cmh2o"),
                ids: (
                    self.ids.telemetry_widgets_right,
                    self.ids.plateau_parent,
                    self.ids.plateau_title,
                    self.ids.plateau_value_measured,
                    self.ids.plateau_value_arrow,
                    self.ids.plateau_value_target,
                    Some(self.ids.plateau_unit),
                ),
                x_position: 0.0,
                y_position: GRAPH_HEIGHT as f64 + LAYOUT_FOOTER_SIZE_HEIGHT
                    - widgets_right_height * 2.0,
                background_color: Color::Rgba(66.0 / 255.0, 44.0 / 255.0, 85.0 / 255.0, 1.0),
                width: widgets_right_width,
                height: widgets_right_height,
            },
        ));

        // Initialize the PEEP widget
        self.widgets.render(ControlWidgetType::Telemetry(
            telemetry_view::TelemetryWidgetConfig {
                title: APP_I18N.t("telemetry-label-expiratory"),
                value_measured: Some(
                    convert_mmh2o_to_cmh2o(
                        ConvertMode::Rounded,
                        machine_snapshot.previous_peep_pressure as f64,
                    )
                    .to_string(),
                ),
                value_target: Some(machine_snapshot.peep_command.to_string()),
                value_arrow: telemetry_data.arrow_image_id,
                unit: APP_I18N.t("telemetry-unit-cmh2o"),
                ids: (
                    self.ids.telemetry_widgets_right,
                    self.ids.peep_parent,
                    self.ids.peep_title,
                    self.ids.peep_value_measured,
                    self.ids.peep_value_arrow,
                    self.ids.peep_value_target,
                    Some(self.ids.peep_unit),
                ),
                x_position: 0.0,
                y_position: GRAPH_HEIGHT as f64 + LAYOUT_FOOTER_SIZE_HEIGHT
                    - widgets_right_height * 3.0,
                background_color: Color::Rgba(76.0 / 255.0, 73.0 / 255.0, 25.0 / 255.0, 1.0),
                width: widgets_right_width,
                height: widgets_right_height,
            },
        ));

        // Initialize the cycles widget
        self.widgets.render(ControlWidgetType::Telemetry(
            telemetry_view::TelemetryWidgetConfig {
                title: APP_I18N.t("telemetry-label-cycles"),
                value_measured: None,
                value_target: Some(machine_snapshot.cpm_command.to_string()),
                value_arrow: telemetry_data.arrow_image_id,
                unit: APP_I18N.t("telemetry-unit-per-minute"),
                ids: (
                    self.ids.layout_footer,
                    self.ids.cycles_parent,
                    self.ids.cycles_title,
                    self.ids.cycles_value_measured,
                    self.ids.cycles_value_arrow,
                    self.ids.cycles_value_target,
                    Some(self.ids.cycles_unit),
                ),
                x_position: 0.0,
                y_position: 0.0,
                background_color: Color::Rgba(47.0 / 255.0, 74.0 / 255.0, 16.0 / 255.0, 1.0),
                width: TELEMETRY_WIDGET_SIZE_WIDTH,
                height: LAYOUT_FOOTER_SIZE_HEIGHT,
            },
        ));

        // Initialize the tidal widget
        let previous_volume = machine_snapshot
            .previous_volume
            .map(|v| format!("{}", v))
            .unwrap_or_else(|| APP_I18N.t("telemetry-value-empty"));

        self.widgets.render(ControlWidgetType::Telemetry(
            telemetry_view::TelemetryWidgetConfig {
                title: APP_I18N.t("telemetry-label-tidal"),
                value_measured: Some(previous_volume),
                value_target: None,
                value_arrow: telemetry_data.arrow_image_id,
                unit: APP_I18N.t("telemetry-unit-milliliters"),
                ids: (
                    self.ids.cycles_parent,
                    self.ids.tidal_parent,
                    self.ids.tidal_title,
                    self.ids.tidal_value_measured,
                    self.ids.tidal_value_arrow,
                    self.ids.tidal_value_target,
                    Some(self.ids.tidal_unit),
                ),
                x_position: TELEMETRY_WIDGET_SIZE_WIDTH,
                y_position: 0.0,
                background_color: Color::Rgba(52.0 / 255.0, 52.0 / 255.0, 52.0 / 255.0, 1.0),
                width: TELEMETRY_WIDGET_SIZE_WIDTH,
                height: LAYOUT_FOOTER_SIZE_HEIGHT,
            },
        ));

        // Initialize the ratio widget
        // Important: if the ratio has decimals, then show them (to the first decimal). If it \
        //   has no decimals (eg. '2.0'), then show it as an integer.
        let expiratory_term_value = convert_mmh2o_to_cmh2o(
            ConvertMode::WithDecimals,
            machine_snapshot.expiratory_term as f64,
        );

        self.widgets.render(ControlWidgetType::Telemetry(
            telemetry_view::TelemetryWidgetConfig {
                title: APP_I18N.t("telemetry-label-ratio"),
                value_measured: Some(if expiratory_term_value.fract() == 0.0 {
                    format!("{}:{}", CYCLE_RATIO_INSPIRATION, expiratory_term_value,)
                } else {
                    format!("{}:{:.1}", CYCLE_RATIO_INSPIRATION, expiratory_term_value,)
                }),
                value_target: None,
                value_arrow: telemetry_data.arrow_image_id,
                unit: format!(
                    "{} {}{}",
                    APP_I18N.t("telemetry-label-ratio-plateau"),
                    trigger.get_plateau_duration(),
                    APP_I18N.t("telemetry-unit-milliseconds")
                ),
                ids: (
                    self.ids.tidal_parent,
                    self.ids.ratio_parent,
                    self.ids.ratio_title,
                    self.ids.ratio_value_measured,
                    self.ids.ratio_value_arrow,
                    self.ids.ratio_value_target,
                    Some(self.ids.ratio_unit),
                ),
                x_position: TELEMETRY_WIDGET_SIZE_WIDTH,
                y_position: 0.0,
                background_color: color::BLUE,
                width: TELEMETRY_WIDGET_SIZE_WIDTH,
                height: LAYOUT_FOOTER_SIZE_HEIGHT,
            },
        ));

        // Initialize the trigger widget
        self.widgets.render(ControlWidgetType::TriggerOverview(
            trigger_overview::TriggerOverview {
                parent: self.ids.ratio_parent,
                container: self.ids.trigger_overview_container,
                border: self.ids.trigger_overview_border,
                title_widget: self.ids.trigger_overview_title,
                status_widget: self.ids.trigger_overview_status,
                inspiration_trigger_offset_widget: self.ids.trigger_overview_offset,
                configure_widget: self.ids.trigger_overview_configure,
                expiratory_term_widget: self.ids.trigger_overview_expiratory_term,
                plateau_duration_widget: self.ids.trigger_overview_plateau_duration,
                background_color: color::BLUE,
                width: TELEMETRY_WIDGET_SIZE_WIDTH,
                height: LAYOUT_FOOTER_SIZE_HEIGHT,
                x_position: TELEMETRY_WIDGET_SIZE_WIDTH,
                y_position: 0.0,
                trigger_settings: trigger,
            },
        ));
    }

    fn render_modal(
        &mut self,
        width: f64,
        height: f64,
        padding: Option<f64>,
        validate: Option<(WidgetId, WidgetId)>,
    ) {
        self.widgets
            .render(ControlWidgetType::Modal(modal::ModalWidgetConfig {
                parent: self.ids.background,
                background: self.ids.modal_background,
                container_borders: self.ids.modal_container_borders,
                container: self.ids.modal_container,
                validate,
                width,
                height,
                padding,
            }));
    }

    fn render_trigger_settings(&mut self, settings: &'a Trigger) {
        self.render_modal(
            TRIGGER_SETTINGS_MODAL_WIDTH,
            TRIGGER_SETTINGS_MODAL_HEIGTH,
            Some(TRIGGER_SETTINGS_MODAL_PADDING),
            Some((self.ids.modal_validate, self.ids.modal_validate_text)),
        );

        self.widgets.render(ControlWidgetType::TriggerSettings(
            trigger_settings::TriggerWidgetConfig {
                width: TRIGGER_SETTINGS_MODAL_WIDTH,
                height: TRIGGER_SETTINGS_MODAL_HEIGTH
                    - MODAL_VALIDATE_BUTTON_HEIGHT
                    - (TRIGGER_SETTINGS_MODAL_PADDING * 2.0),
                trigger_settings: settings,

                status_container_parent: self.ids.modal_container,
                status_container_widget: self.ids.trigger_status_container,
                status_enabled_text_widget: self.ids.trigger_status_text,
                status_enabled_button_widget: self.ids.trigger_status_button,
                status_enabled_button_text_widget: self.ids.trigger_status_button_text,

                inspiratory_offset_container_parent: self.ids.trigger_offset_container,
                inspiratory_offset_more_button_widget: self.ids.trigger_offset_more_button,
                inspiratory_offset_more_button_text_widget: self
                    .ids
                    .trigger_offset_more_button_text,
                inspiratory_offset_less_button_widget: self.ids.trigger_offset_less_button,
                inspiratory_offset_less_button_text_widget: self
                    .ids
                    .trigger_offset_less_button_text,
                inspiratory_offset_text_widget: self.ids.trigger_offset_text,
                inspiratory_offset_value_widget: self.ids.trigger_offset_value,
            },
        ));
    }

    fn render_expiration_term_settings(&mut self, settings: &'a Trigger) {
        self.render_modal(
            EXPIRATION_TERM_SETTINGS_MODAL_WIDTH,
            EXPIRATION_TERM_SETTINGS_MODAL_HEIGTH,
            Some(EXPIRATION_TERM_SETTINGS_MODAL_PADDING),
            Some((self.ids.modal_validate, self.ids.modal_validate_text)),
        );

        self.widgets
            .render(ControlWidgetType::ExpirationTermSettings(
                expiration_term_settings::ExpirationTermSettingsWidgetConfig {
                    width: EXPIRATION_TERM_SETTINGS_MODAL_WIDTH,
                    height: EXPIRATION_TERM_SETTINGS_MODAL_HEIGTH
                        - MODAL_VALIDATE_BUTTON_HEIGHT
                        - (EXPIRATION_TERM_SETTINGS_MODAL_PADDING * 2.0),
                    trigger_settings: settings,

                    expiration_term_container_parent: self.ids.modal_container,
                    expiration_term_container_widget: self.ids.expiration_term_term_container,
                    expiration_term_more_button_widget: self.ids.expiration_term_term_more_button,
                    expiration_term_more_button_text_widget: self
                        .ids
                        .expiration_term_term_more_button_text,
                    expiration_term_less_button_widget: self.ids.expiration_term_term_less_button,
                    expiration_term_less_button_text_widget: self
                        .ids
                        .expiration_term_term_less_button_text,
                    expiration_term_text_widget: self.ids.expiration_term_term_text,
                    expiration_term_value_widget: self.ids.expiration_term_term_value,
                },
            ));
    }
}
