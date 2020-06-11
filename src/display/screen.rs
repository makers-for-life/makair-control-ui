// MakAir
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::color::{self, Color};
use conrod_core::widget::Id as WidgetId;

use telemetry::alarm::AlarmCode;
use telemetry::structures::{AlarmPriority, MachineStateSnapshot};

use crate::chip::{settings::trigger_inspiratory::TriggerInspiratory, ChipState};
use crate::config::environment::*;
use crate::physics::types::DataPressure;
use crate::APP_I18N;

use super::fonts::Fonts;
use super::widget::{
    AlarmsWidgetConfig, BackgroundWidgetConfig, BrandingWidgetConfig, ControlWidget,
    ControlWidgetType, ErrorWidgetConfig, ExpRatioSettingsWidgetConfig, GraphWidgetConfig,
    HeartbeatWidgetConfig, InitializingWidgetConfig, LayoutConfig, LayoutWidgetConfig,
    ModalWidgetConfig, NoDataWidgetConfig, StatusWidgetConfig, StopWidgetConfig,
    TelemetryWidgetConfig, TelemetryWidgetContainerConfig, TriggerInspiratoryOverview,
    TriggerInspiratoryWidgetConfig,
};

widget_ids!(pub struct Ids {
  layout_header,
  layout_body,
  layout_footer,

  alarm_container,
  alarm_title,
  alarm_empty,
  alarm_alarms[],
  alarm_codes_containers[],
  alarm_codes[],
  alarm_messages_containers[],
  alarm_messages[],

  background,

  pressure_graph,

  branding_container,
  branding_image,
  branding_text,

  status_wrapper,
  status_unit_box,
  status_unit_text,
  status_power_box,
  status_power_text,
  status_save_icon,

  heartbeat_ground,
  heartbeat_surround,
  heartbeat_inner,

  telemetry_widgets_right,
  telemetry_widgets_bottom,

  cycles_parent,
  cycles_title,
  cycles_value_measured,
  cycles_value_arrow,
  cycles_value_target,

  peak_parent,
  peak_title,
  peak_value_measured,
  peak_value_arrow,
  peak_value_target,

  plateau_parent,
  plateau_title,
  plateau_value_measured,
  plateau_value_arrow,
  plateau_value_target,

  peep_parent,
  peep_title,
  peep_value_measured,
  peep_value_arrow,
  peep_value_target,

  ratio_parent,
  ratio_title,
  ratio_value_measured,
  ratio_value_arrow,
  ratio_value_target,
  ratio_unit,

  exp_ratio_term_container,
  exp_ratio_term_more_button,
  exp_ratio_term_more_button_text,
  exp_ratio_term_less_button,
  exp_ratio_term_less_button_text,
  exp_ratio_term_text,
  exp_ratio_term_value,

  tidal_parent,
  tidal_title,
  tidal_value_measured,
  tidal_value_arrow,
  tidal_value_target,
  tidal_unit,

  trigger_inspiratory_overview_container,
  trigger_inspiratory_overview_title,
  trigger_inspiratory_overview_status,
  trigger_inspiratory_overview_offset,
  trigger_inspiratory_overview_expiratory_term,
  trigger_inspiratory_overview_plateau_duration,

  trigger_inspiratory_status_container,
  trigger_inspiratory_status_text,
  trigger_inspiratory_status_button,
  trigger_inspiratory_status_button_text,
  trigger_inspiratory_offset_container,
  trigger_inspiratory_offset_more_button,
  trigger_inspiratory_offset_more_button_text,
  trigger_inspiratory_offset_less_button,
  trigger_inspiratory_offset_less_button_text,
  trigger_inspiratory_offset_text,
  trigger_inspiratory_offset_value,

  modal_background,
  modal_container_borders,
  modal_container,
  modal_validate,
  modal_validate_text,

  stopped_title,
  stopped_message,

  no_data,
  error,

  initializing_logo,
});

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
        trigger_inspiratory: &'a TriggerInspiratory,
        trigger_inspiratory_open: bool,
        exp_ratio_open: bool,
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
        self.render_telemetry(telemetry_data, trigger_inspiratory);

        if trigger_inspiratory_open {
            self.render_trigger_settings(trigger_inspiratory);
        } else if exp_ratio_open {
            self.render_exp_ratio_settings(trigger_inspiratory);
        }
    }

    pub fn render_background(&mut self) {
        let config = BackgroundWidgetConfig::new(color::BLACK, self.ids.background);

        self.widgets.render(ControlWidgetType::Background(config));
    }

    pub fn render_layout(&mut self) {
        let header_config = LayoutWidgetConfig::new(
            self.ids.background,
            0.0,
            LAYOUT_HEADER_SIZE_FULL_HEIGHT,
            self.ids.layout_header,
        );
        let body_config = LayoutWidgetConfig::new(
            self.ids.background,
            LAYOUT_HEADER_SIZE_HEIGHT,
            LAYOUT_BODY_SIZE_HEIGHT,
            self.ids.layout_body,
        );
        let footer_config = LayoutWidgetConfig::new(
            self.ids.layout_body,
            0.0,
            LAYOUT_FOOTER_SIZE_HEIGHT,
            self.ids.layout_footer,
        );
        let config = LayoutConfig::new(header_config, body_config, footer_config);
        self.widgets.render(ControlWidgetType::Layout(config));
    }

    pub fn render_branding(
        &mut self,
        version_firmware: &'a str,
        version_control: &'a str,
        image_id: conrod_core::image::Id,
        width: f64,
        height: f64,
    ) {
        let config = BrandingWidgetConfig::new(
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
        );

        self.widgets.render(ControlWidgetType::Branding(config));
    }

    pub fn render_alarms(&mut self) {
        let config = AlarmsWidgetConfig {
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
        };

        self.widgets.render(ControlWidgetType::Alarms(config));
    }

    pub fn render_status(&mut self, status_data: ScreenDataStatus<'a>) {
        let config = StatusWidgetConfig::new(
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
        );

        self.widgets.render(ControlWidgetType::Status(config));
    }

    pub fn render_heartbeat(&mut self, heartbeat_data: ScreenDataHeartbeat<'a>) {
        let config = HeartbeatWidgetConfig::new(
            heartbeat_data.data_pressure,
            self.machine_snapshot.unwrap().peak_command,
            self.ids.layout_header,
            self.ids.heartbeat_ground,
            self.ids.heartbeat_surround,
            self.ids.heartbeat_inner,
        );

        self.widgets.render(ControlWidgetType::Heartbeat(config));
    }

    pub fn render_graph(&mut self, image_id: conrod_core::image::Id, width: f64, height: f64) {
        let config = GraphWidgetConfig::new(
            width,
            height,
            image_id,
            self.ids.layout_body,
            self.ids.pressure_graph,
        );

        self.widgets.render(ControlWidgetType::Graph(config));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_stop(
        &mut self,
        branding_data: ScreenDataBranding<'a>,
        status_data: ScreenDataStatus<'a>,
        heartbeat_data: ScreenDataHeartbeat<'a>,
        graph_data: ScreenDataGraph,
        telemetry_data: ScreenDataTelemetry,
        trigger_inspiratory: &'a TriggerInspiratory,
        trigger_inspiratory_open: bool,
        exp_ratio_open: bool,
    ) {
        // Render regular data as background
        self.render_with_data(
            branding_data,
            status_data,
            heartbeat_data,
            graph_data,
            telemetry_data,
            trigger_inspiratory,
            trigger_inspiratory_open,
            exp_ratio_open,
        );

        if !trigger_inspiratory_open && !exp_ratio_open {
            self.render_modal(
                DISPLAY_STOPPED_MESSAGE_CONTAINER_WIDTH,
                DISPLAY_STOPPED_MESSAGE_CONTAINER_HEIGHT,
                None,
                None,
            );

            let config = StopWidgetConfig {
                container: self.ids.modal_container,
                title: self.ids.stopped_title,
                message: self.ids.stopped_message,
            };

            // Render stop layer
            self.widgets.render(ControlWidgetType::Stop(config));
        }
    }

    pub fn render_no_data(&mut self) {
        let config = NoDataWidgetConfig::new(self.ids.no_data);

        self.widgets.render(ControlWidgetType::NoData(config));
    }

    pub fn render_error(&mut self, error: String) {
        let config = ErrorWidgetConfig::new(error, self.ids.error);

        self.render_background();

        self.widgets.render(ControlWidgetType::Error(config));
    }

    pub fn render_initializing(&mut self, config: ScreenBootLoader) {
        let config = InitializingWidgetConfig::new(
            self.ids.initializing_logo,
            config.width,
            config.height,
            config.image_id,
        );

        self.render_background();

        self.widgets.render(ControlWidgetType::Initializing(config));
    }

    pub fn render_telemetry(
        &mut self,
        telemetry_data: ScreenDataTelemetry,
        trigger_inspiratory: &'a TriggerInspiratory,
    ) {
        let machine_snapshot = self.machine_snapshot.unwrap();

        let widgets_right_width: f64 = (DISPLAY_WINDOW_SIZE_WIDTH - GRAPH_WIDTH) as f64;
        let widgets_right_height: f64 = GRAPH_HEIGHT as f64 / 3.0;

        let container_config = TelemetryWidgetContainerConfig::new(
            widgets_right_width,
            DISPLAY_WINDOW_SIZE_HEIGHT as f64 - LAYOUT_HEADER_SIZE_HEIGHT,
            self.ids.pressure_graph,
            self.ids.telemetry_widgets_right,
        );
        self.widgets
            .render(ControlWidgetType::TelemetryContainer(container_config));

        let peak_config = TelemetryWidgetConfig {
            title: APP_I18N.t("telemetry-label-peak"),
            value_measured: Some(
                (machine_snapshot.previous_peak_pressure as f64 / 10.0)
                    .round()
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
                None,
            ),
            x_position: 0.0,
            y_position: GRAPH_HEIGHT as f64 + LAYOUT_FOOTER_SIZE_HEIGHT - widgets_right_height,
            background_color: Color::Rgba(39.0 / 255.0, 66.0 / 255.0, 100.0 / 255.0, 1.0),
            width: widgets_right_width,
            height: widgets_right_height,
        };

        self.widgets
            .render(ControlWidgetType::Telemetry(peak_config));

        // Initialize the plateau widget
        let plateau_config = TelemetryWidgetConfig {
            title: APP_I18N.t("telemetry-label-plateau"),
            value_measured: Some(
                (machine_snapshot.previous_plateau_pressure as f64 / 10.0)
                    .round()
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
                None,
            ),
            x_position: 0.0,
            y_position: GRAPH_HEIGHT as f64 + LAYOUT_FOOTER_SIZE_HEIGHT
                - widgets_right_height * 2.0,
            background_color: Color::Rgba(66.0 / 255.0, 44.0 / 255.0, 85.0 / 255.0, 1.0),
            width: widgets_right_width,
            height: widgets_right_height,
        };

        self.widgets
            .render(ControlWidgetType::Telemetry(plateau_config));

        // Initialize the PEEP widget
        let peep_config = TelemetryWidgetConfig {
            title: APP_I18N.t("telemetry-label-expiratory"),
            value_measured: Some(
                (machine_snapshot.previous_peep_pressure as f64 / 10.0)
                    .round()
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
                None,
            ),
            x_position: 0.0,
            y_position: GRAPH_HEIGHT as f64 + LAYOUT_FOOTER_SIZE_HEIGHT
                - widgets_right_height * 3.0,
            background_color: Color::Rgba(76.0 / 255.0, 73.0 / 255.0, 25.0 / 255.0, 1.0),
            width: widgets_right_width,
            height: widgets_right_height,
        };

        self.widgets
            .render(ControlWidgetType::Telemetry(peep_config));

        // Initialize the cycles widget
        let cycles_config = TelemetryWidgetConfig {
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
                None,
            ),
            x_position: 0.0,
            y_position: 0.0,
            background_color: Color::Rgba(47.0 / 255.0, 74.0 / 255.0, 16.0 / 255.0, 1.0),
            width: TELEMETRY_WIDGET_SIZE_WIDTH,
            height: LAYOUT_FOOTER_SIZE_HEIGHT,
        };

        self.widgets
            .render(ControlWidgetType::Telemetry(cycles_config));

        // Initialize the ratio widget
        let ratio_config = TelemetryWidgetConfig {
            title: APP_I18N.t("telemetry-label-ratio"),
            value_measured: Some(format!(
                "{}:{}",
                CYCLE_RATIO_INSPIRATION,
                (trigger_inspiratory.expiratory_term as f64 / 10.0)
            )),
            value_target: None,
            value_arrow: telemetry_data.arrow_image_id,
            unit: format!(
                "Plateau duration: {}ms",
                trigger_inspiratory.get_plateau_duration()
            ),
            ids: (
                self.ids.cycles_parent,
                self.ids.ratio_parent,
                self.ids.ratio_title,
                self.ids.ratio_value_measured,
                self.ids.ratio_value_arrow,
                self.ids.ratio_value_target,
                Some(self.ids.ratio_unit),
            ),
            x_position: TELEMETRY_WIDGET_SIZE_WIDTH,
            y_position: 0.0,
            background_color: Color::Rgba(52.0 / 255.0, 52.0 / 255.0, 52.0 / 255.0, 1.0),
            width: TELEMETRY_WIDGET_SIZE_WIDTH,
            height: LAYOUT_FOOTER_SIZE_HEIGHT,
        };

        self.widgets
            .render(ControlWidgetType::Telemetry(ratio_config));

        // Initialize the tidal widget
        let previous_volume = machine_snapshot
            .previous_volume
            .map(|v| format!("{}", v))
            .unwrap_or_else(|| "n/a".to_string());

        let tidal_config = TelemetryWidgetConfig {
            title: APP_I18N.t("telemetry-label-tidal"),
            value_measured: Some(previous_volume),
            value_target: None,
            value_arrow: telemetry_data.arrow_image_id,
            unit: APP_I18N.t("telemetry-unit-milliliters"),
            ids: (
                self.ids.ratio_parent,
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
        };

        self.widgets
            .render(ControlWidgetType::Telemetry(tidal_config));

        let trigger_inspiratory_config = TriggerInspiratoryOverview {
            parent: self.ids.tidal_parent,
            container: self.ids.trigger_inspiratory_overview_container,
            title_widget: self.ids.trigger_inspiratory_overview_title,
            status_widget: self.ids.trigger_inspiratory_overview_status,
            inspiration_trigger_offset_widget: self.ids.trigger_inspiratory_overview_offset,
            expiratory_term_widget: self.ids.trigger_inspiratory_overview_expiratory_term,
            plateau_duration_widget: self.ids.trigger_inspiratory_overview_plateau_duration,
            background_color: color::BLUE,
            width: TELEMETRY_WIDGET_SIZE_WIDTH,
            height: LAYOUT_FOOTER_SIZE_HEIGHT,
            x_position: TELEMETRY_WIDGET_SIZE_WIDTH,
            y_position: 0.0,
            trigger_inspiratory_settings: trigger_inspiratory,
        };

        self.widgets
            .render(ControlWidgetType::TriggerInspiratoryOverview(
                trigger_inspiratory_config,
            ));
    }

    fn render_modal(
        &mut self,
        width: f64,
        height: f64,
        padding: Option<f64>,
        validate: Option<(WidgetId, WidgetId)>,
    ) {
        let modal_config = ModalWidgetConfig {
            parent: self.ids.background,
            background: self.ids.modal_background,
            container_borders: self.ids.modal_container_borders,
            container: self.ids.modal_container,
            validate,
            width,
            height,
            padding,
        };

        self.widgets.render(ControlWidgetType::Modal(modal_config));
    }

    fn render_trigger_settings(&mut self, settings: &'a TriggerInspiratory) {
        let padding = 20.0;
        self.render_modal(
            TRIGGER_SETTINGS_MODAL_WIDTH,
            TRIGGER_SETTINGS_MODAL_HEIGTH,
            Some(padding),
            Some((self.ids.modal_validate, self.ids.modal_validate_text)),
        );

        let config = TriggerInspiratoryWidgetConfig {
            width: TRIGGER_SETTINGS_MODAL_WIDTH,
            height: TRIGGER_SETTINGS_MODAL_HEIGTH - MODAL_VALIDATE_BUTTON_HEIGHT - (padding * 2.0),
            trigger_inspiratory_settings: settings,

            status_container_parent: self.ids.modal_container,
            status_container_widget: self.ids.trigger_inspiratory_status_container,
            status_enabled_text_widget: self.ids.trigger_inspiratory_status_text,
            status_enabled_button_widget: self.ids.trigger_inspiratory_status_button,
            status_enabled_button_text_widget: self.ids.trigger_inspiratory_status_button_text,

            inspiratory_offset_container_parent: self.ids.trigger_inspiratory_offset_container,
            inspiratory_offset_more_button_widget: self.ids.trigger_inspiratory_offset_more_button,
            inspiratory_offset_more_button_text_widget: self
                .ids
                .trigger_inspiratory_offset_more_button_text,
            inspiratory_offset_less_button_widget: self.ids.trigger_inspiratory_offset_less_button,
            inspiratory_offset_less_button_text_widget: self
                .ids
                .trigger_inspiratory_offset_less_button_text,
            inspiratory_offset_text_widget: self.ids.trigger_inspiratory_offset_text,
            inspiratory_offset_value_widget: self.ids.trigger_inspiratory_offset_value,
        };

        self.widgets
            .render(ControlWidgetType::TriggerInspiratorySettings(config));
    }

    fn render_exp_ratio_settings(&mut self, settings: &'a TriggerInspiratory) {
        let padding = 20.0;
        self.render_modal(
            EXP_RATIO_SETTINGS_MODAL_WIDTH,
            EXP_RATIO_SETTINGS_MODAL_HEIGTH,
            Some(padding),
            Some((self.ids.modal_validate, self.ids.modal_validate_text)),
        );

        let config = ExpRatioSettingsWidgetConfig {
            width: EXP_RATIO_SETTINGS_MODAL_WIDTH,
            height: EXP_RATIO_SETTINGS_MODAL_HEIGTH
                - MODAL_VALIDATE_BUTTON_HEIGHT
                - (padding * 2.0),
            trigger_inspiratory_settings: settings,

            exp_ratio_container_parent: self.ids.modal_container,
            exp_ratio_container_widget: self.ids.exp_ratio_term_container,
            exp_ratio_more_button_widget: self.ids.exp_ratio_term_more_button,
            exp_ratio_more_button_text_widget: self.ids.exp_ratio_term_more_button_text,
            exp_ratio_less_button_widget: self.ids.exp_ratio_term_less_button,
            exp_ratio_less_button_text_widget: self.ids.exp_ratio_term_less_button_text,
            exp_ratio_text_widget: self.ids.exp_ratio_term_text,
            exp_ratio_value_widget: self.ids.exp_ratio_term_value,
        };

        self.widgets
            .render(ControlWidgetType::ExpRatioSettings(config));
    }
}
