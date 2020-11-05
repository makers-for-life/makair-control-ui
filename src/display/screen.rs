// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::color::{self, Color};
use conrod_core::widget::Id as WidgetId;

use telemetry::alarm::AlarmCode;
use telemetry::structures::{AlarmPriority, DataSnapshot, MachineStateSnapshot};

use crate::chip::settings::{
    cycles::SettingsCycles, expiration_term::SettingsExpirationTerm, pressure::SettingsPressure,
    run::SettingsRun, snooze::SettingsSnooze, trigger::SettingsTrigger, ChipSettings,
};
use crate::chip::ChipError;
use crate::config::environment::*;
use crate::utilities::units::{convert_mmh2o_to_cmh2o, ConvertMode};
use crate::widget::*;
use crate::APP_I18N;

use super::data::*;
use super::fonts::Fonts;
use super::identifiers::Ids;
use super::renderer::DisplayRendererStates;
use super::widget::{ControlWidget, ControlWidgetType};

pub struct ScreenModalsOpen {
    run: bool,
    snooze: bool,
    advanced: bool,
    trigger: bool,
    expiration_term: bool,
    pressure: bool,
    cycles: bool,
}

pub struct Screen<'a> {
    ids: &'a Ids,
    last_tick: Option<u64>,
    machine_snapshot: Option<&'a MachineStateSnapshot>,
    data_snapshot: Option<&'a DataSnapshot>,
    ongoing_alarms: Option<&'a [(AlarmCode, AlarmPriority)]>,
    widgets: ControlWidget<'a>,
}

impl ScreenModalsOpen {
    pub fn from_states(states: &DisplayRendererStates) -> Self {
        ScreenModalsOpen {
            run: states.run_settings.is_open(),
            snooze: states.snooze_settings.is_open(),
            advanced: states.advanced_settings.is_open(),
            trigger: states.trigger_settings.is_open(),
            expiration_term: states.expiration_term_settings.is_open(),
            pressure: states.pressure_settings.is_open(),
            cycles: states.cycles_settings.is_open(),
        }
    }
}

impl<'a> Screen<'a> {
    pub fn new(
        ui: conrod_core::UiCell<'a>,
        ids: &'a Ids,
        fonts: &'a Fonts,
        ongoing_alarms: Option<&'a [(AlarmCode, AlarmPriority)]>,
        last_tick: Option<u64>,
        machine_snapshot: Option<&'a MachineStateSnapshot>,
        data_snapshot: Option<&'a DataSnapshot>,
    ) -> Screen<'a> {
        Screen {
            ids,
            last_tick,
            machine_snapshot,
            data_snapshot,
            ongoing_alarms,
            widgets: ControlWidget::new(ui, fonts),
        }
    }

    pub fn render_background(&mut self) {
        self.widgets
            .render(ControlWidgetType::Background(background::Config::new(
                color::BLACK,
                self.ids.background,
            )));
    }

    pub fn render_layout(&mut self) {
        self.widgets
            .render(ControlWidgetType::Layout(layout::Config::new(
                layout::Slice::new(
                    self.ids.background,
                    0.0,
                    LAYOUT_HEADER_SIZE_HEIGHT,
                    self.ids.layout_header,
                ),
                layout::Slice::new(
                    self.ids.background,
                    LAYOUT_HEADER_SIZE_HEIGHT,
                    LAYOUT_BODY_SIZE_HEIGHT,
                    self.ids.layout_body,
                ),
                layout::Slice::new(
                    self.ids.background,
                    LAYOUT_HEADER_SIZE_HEIGHT + LAYOUT_BODY_SIZE_HEIGHT,
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
        self.widgets
            .render(ControlWidgetType::Branding(branding::Config::new(
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
            )));
    }

    pub fn render_alarms(&mut self) {
        self.widgets
            .render(ControlWidgetType::Alarms(alarms::Config {
                parent: self.ids.branding_container,
                container: self.ids.alarm_container,
                title_wrapper: self.ids.alarm_title_wrapper,
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

    pub fn render_heartbeat(&mut self, heartbeat_data: DisplayDataHeartbeat<'a>) {
        self.widgets
            .render(ControlWidgetType::Heartbeat(heartbeat::Config::new(
                heartbeat_data.data_pressure,
                self.machine_snapshot.unwrap().peak_command,
                self.ids.layout_header,
                self.ids.heartbeat_ground,
                self.ids.heartbeat_surround,
                self.ids.heartbeat_inner,
            )));
    }

    pub fn render_status(&mut self, status_data: DisplayDataStatus<'a>) {
        self.widgets
            .render(ControlWidgetType::Status(status::Config::new(
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

    pub fn render_controls(&mut self, controls_data: DisplayDataControls<'a>) {
        self.widgets
            .render(ControlWidgetType::Controls(controls::Config::new(
                self.ids.layout_header,
                self.ids.controls_wrapper,
                self.ids.controls_button_run,
                self.ids.controls_button_snooze,
                self.ids.controls_button_advanced,
                self.ids.controls_image_run,
                self.ids.controls_image_snooze,
                self.ids.controls_image_advanced,
                controls_data.run_image_id,
                controls_data.snooze_inactive_image_id,
                controls_data.snooze_active_image_id,
                controls_data.advanced_image_id,
                controls_data.chip_state,
                &controls_data.chip_settings.snooze,
            )));
    }

    pub fn render_graph(&mut self, image_id: conrod_core::image::Id, width: f64, height: f64) {
        self.widgets
            .render(ControlWidgetType::Graph(graph::Config::new(
                width,
                height,
                image_id,
                self.ids.layout_body,
                self.ids.pressure_graph,
            )));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_running(
        &mut self,
        branding_data: DisplayDataBranding<'a>,
        controls_data: DisplayDataControls<'a>,
        status_data: DisplayDataStatus<'a>,
        heartbeat_data: DisplayDataHeartbeat<'a>,
        graph_data: DisplayDataGraph,
        telemetry_data: DisplayDataTelemetry,
        settings: &'a ChipSettings,
        modals: &ScreenModalsOpen,
    ) {
        // Render common background
        self.render_background();
        self.render_layout();

        // Render top left elements
        self.render_branding(
            branding_data.firmware_version,
            RUNTIME_VERSION,
            branding_data.image_id,
            branding_data.width,
            branding_data.height,
        );
        self.render_alarms();

        // Render top right elements
        self.render_heartbeat(heartbeat_data);
        self.render_status(status_data);
        self.render_controls(controls_data);

        // Render middle elements
        self.render_graph(graph_data.image_id, graph_data.width, graph_data.height);

        // Render bottom elements
        self.render_telemetry(telemetry_data, &settings.trigger, &settings.expiration_term);

        // Render modals (as needed)
        self.render_settings(settings, modals);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_stop(
        &mut self,
        branding_data: DisplayDataBranding<'a>,
        controls_data: DisplayDataControls<'a>,
        status_data: DisplayDataStatus<'a>,
        heartbeat_data: DisplayDataHeartbeat<'a>,
        graph_data: DisplayDataGraph,
        telemetry_data: DisplayDataTelemetry,
        settings: &'a ChipSettings,
        modals: &ScreenModalsOpen,
    ) {
        // Render regular data as background (alias the running screen)
        self.render_running(
            branding_data,
            controls_data,
            status_data,
            heartbeat_data,
            graph_data,
            telemetry_data,
            settings,
            modals,
        );

        // Render stop 'pseudo-modal'
        self.widgets.render(ControlWidgetType::Modal(modal::Config {
            parent: self.ids.pressure_graph,
            background: self.ids.stop_background,
            container_borders: self.ids.stop_container_borders,
            container: self.ids.stop_container,
            validate: None,
            width: DISPLAY_STOP_MESSAGE_CONTAINER_WIDTH,
            height: DISPLAY_STOP_MESSAGE_CONTAINER_HEIGHT,
            padding: None,
            colors: Some((
                Color::Rgba(0.0, 0.0, 0.0, 0.98),
                Color::Rgba(1.0, 1.0, 1.0, 0.075),
            )),
        }));

        self.widgets.render(ControlWidgetType::Stop(stop::Config {
            container: self.ids.stop_container,
            title: self.ids.stop_title,
            message: self.ids.stop_message,
        }));
    }

    pub fn render_error(&mut self, config: DisplayDataError<'a>) {
        // Generate error message
        let message = match config.error {
            ChipError::NoDevice => APP_I18N.t("error-message-no-device"),
            ChipError::TimedOut => APP_I18N.t("error-message-timed-out"),
            ChipError::BadProtocol => APP_I18N.t("error-message-bad-protocol"),
            ChipError::Other(details) => details.to_owned(),
        };

        // Render background
        self.render_background();

        // Render error
        self.widgets
            .render(ControlWidgetType::Error(error::Config::new(
                self.ids.error_container,
                self.ids.error_icon,
                self.ids.error_text_wrapper,
                self.ids.error_text_title,
                self.ids.error_text_message,
                config.width,
                config.height,
                config.image_id,
                message,
            )));
    }

    pub fn render_initializing(&mut self, config: DisplayDataBootloader) {
        self.render_background();

        self.widgets
            .render(ControlWidgetType::Initializing(initializing::Config::new(
                self.ids.initializing_container,
                self.ids.initializing_logo,
                self.ids.initializing_text,
                config.width,
                config.height,
                config.image_id,
                config.connecting,
            )));
    }

    pub fn render_telemetry(
        &mut self,
        telemetry_data: DisplayDataTelemetry,
        trigger: &'a SettingsTrigger,
        expiration_term: &'a SettingsExpirationTerm,
    ) {
        let machine_snapshot = self.machine_snapshot.unwrap();

        // Initialize the pressure graph widget
        self.widgets.render(ControlWidgetType::TelemetryContainer(
            telemetry_container::Config::new(
                TELEMETRY_WIDGET_RIGHT_SIZE_WIDTH,
                DISPLAY_WINDOW_SIZE_HEIGHT as f64 - LAYOUT_HEADER_SIZE_HEIGHT,
                self.ids.pressure_graph,
                self.ids.telemetry_widgets_right,
            ),
        ));

        // Check if at least a pressure value is known (otherwise, all pressure widgets should \
        //   show as empty)
        let has_target_pressure = machine_snapshot.peak_command > 0
            || machine_snapshot.plateau_command > 0
            || machine_snapshot.peep_command > 0;

        // Initialize the peak widget
        self.widgets
            .render(ControlWidgetType::TelemetryView(telemetry_view::Config {
                title: APP_I18N.t("telemetry-label-peak"),
                value_measured: Some(if !has_target_pressure {
                    TELEMETRY_WIDGET_VALUE_EMPTY.to_owned()
                } else {
                    convert_mmh2o_to_cmh2o(
                        ConvertMode::Rounded,
                        machine_snapshot.previous_peak_pressure as f64,
                    )
                    .to_string()
                }),
                value_target: if !has_target_pressure {
                    None
                } else {
                    Some(machine_snapshot.peak_command.to_string())
                },
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
                y_position: GRAPH_HEIGHT as f64 + LAYOUT_FOOTER_SIZE_HEIGHT
                    - TELEMETRY_WIDGET_RIGHT_SIZE_HEIGHT * 1.0,
                background_color: Color::Rgba(39.0 / 255.0, 66.0 / 255.0, 100.0 / 255.0, 1.0),
                width: TELEMETRY_WIDGET_RIGHT_SIZE_WIDTH,
                height: TELEMETRY_WIDGET_RIGHT_SIZE_HEIGHT,
            }));

        // Initialize the plateau widget
        self.widgets
            .render(ControlWidgetType::TelemetryView(telemetry_view::Config {
                title: APP_I18N.t("telemetry-label-plateau"),
                value_measured: Some(if !has_target_pressure {
                    TELEMETRY_WIDGET_VALUE_EMPTY.to_owned()
                } else {
                    convert_mmh2o_to_cmh2o(
                        ConvertMode::Rounded,
                        machine_snapshot.previous_plateau_pressure as f64,
                    )
                    .to_string()
                }),
                value_target: if !has_target_pressure {
                    None
                } else {
                    Some(machine_snapshot.plateau_command.to_string())
                },
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
                    - TELEMETRY_WIDGET_RIGHT_SIZE_HEIGHT * 2.0
                    - TELEMETRY_WIDGET_SPACING_SIDES,
                background_color: Color::Rgba(66.0 / 255.0, 44.0 / 255.0, 85.0 / 255.0, 1.0),
                width: TELEMETRY_WIDGET_RIGHT_SIZE_WIDTH,
                height: TELEMETRY_WIDGET_RIGHT_SIZE_HEIGHT,
            }));

        // Initialize the PEEP widget
        self.widgets
            .render(ControlWidgetType::TelemetryView(telemetry_view::Config {
                title: APP_I18N.t("telemetry-label-expiratory"),
                value_measured: Some(if !has_target_pressure {
                    TELEMETRY_WIDGET_VALUE_EMPTY.to_owned()
                } else {
                    convert_mmh2o_to_cmh2o(
                        ConvertMode::Rounded,
                        machine_snapshot.previous_peep_pressure as f64,
                    )
                    .to_string()
                }),
                value_target: if !has_target_pressure {
                    None
                } else {
                    Some(machine_snapshot.peep_command.to_string())
                },
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
                    - TELEMETRY_WIDGET_RIGHT_SIZE_HEIGHT * 3.0
                    - TELEMETRY_WIDGET_SPACING_SIDES * 2.0,
                background_color: Color::Rgba(76.0 / 255.0, 73.0 / 255.0, 25.0 / 255.0, 1.0),
                width: TELEMETRY_WIDGET_RIGHT_SIZE_WIDTH,
                height: TELEMETRY_WIDGET_RIGHT_SIZE_HEIGHT,
            }));

        // Initialize the cycles widget
        self.widgets
            .render(ControlWidgetType::TelemetryView(telemetry_view::Config {
                title: APP_I18N.t("telemetry-label-cycles"),
                value_measured: if machine_snapshot.cpm_command == 0 {
                    None
                } else {
                    machine_snapshot.previous_cpm.map(|value| value.to_string())
                },
                value_target: Some(if machine_snapshot.cpm_command == 0 {
                    TELEMETRY_WIDGET_VALUE_EMPTY.to_owned()
                } else {
                    machine_snapshot.cpm_command.to_string()
                }),
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
                width: TELEMETRY_WIDGET_BOTTOM_SIZE_WIDTH,
                height: LAYOUT_FOOTER_SIZE_HEIGHT,
            }));

        // Initialize the tidal widget
        let previous_volume = machine_snapshot
            .previous_volume
            .map(|v| format!("{}", v))
            .unwrap_or_else(|| TELEMETRY_WIDGET_VALUE_EMPTY.to_owned());

        self.widgets
            .render(ControlWidgetType::TelemetryView(telemetry_view::Config {
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
                x_position: TELEMETRY_WIDGET_BOTTOM_SIZE_WIDTH + TELEMETRY_WIDGET_SPACING_SIDES,
                y_position: 0.0,
                background_color: Color::Rgba(52.0 / 255.0, 52.0 / 255.0, 52.0 / 255.0, 1.0),
                width: TELEMETRY_WIDGET_BOTTOM_SIZE_WIDTH,
                height: LAYOUT_FOOTER_SIZE_HEIGHT,
            }));

        // Initialize the ratio widget
        // Important: if the ratio has decimals, then show them (to the first decimal). If it \
        //   has no decimals (eg. '2.0'), then show it as an integer.
        self.widgets
            .render(ControlWidgetType::TelemetryView(telemetry_view::Config {
                title: APP_I18N.t("telemetry-label-ratio"),
                value_measured: Some(if machine_snapshot.expiratory_term == 0 {
                    TELEMETRY_WIDGET_VALUE_EMPTY.to_owned()
                } else {
                    let expiratory_term_value = convert_mmh2o_to_cmh2o(
                        ConvertMode::WithDecimals,
                        machine_snapshot.expiratory_term as f64,
                    );

                    if expiratory_term_value.fract() == 0.0 {
                        format!("{}:{}", CYCLE_RATIO_INSPIRATION, expiratory_term_value,)
                    } else {
                        format!("{}:{:.1}", CYCLE_RATIO_INSPIRATION, expiratory_term_value,)
                    }
                }),
                value_target: None,
                value_arrow: telemetry_data.arrow_image_id,
                unit: format!(
                    "{} {}",
                    APP_I18N.t("telemetry-label-ratio-plateau"),
                    if let Some(plateau_duration) = expiration_term.get_plateau_duration() {
                        format!(
                            "{}{}",
                            plateau_duration,
                            APP_I18N.t("telemetry-unit-milliseconds")
                        )
                    } else {
                        TELEMETRY_WIDGET_VALUE_EMPTY.to_owned()
                    }
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
                x_position: TELEMETRY_WIDGET_BOTTOM_SIZE_WIDTH + TELEMETRY_WIDGET_SPACING_SIDES,
                y_position: 0.0,
                background_color: color::BLUE,
                width: TELEMETRY_WIDGET_BOTTOM_SIZE_WIDTH,
                height: LAYOUT_FOOTER_SIZE_HEIGHT,
            }));

        // Initialize the trigger widget
        self.widgets.render(ControlWidgetType::TriggerOverview(
            trigger_overview::Config {
                parent: self.ids.ratio_parent,
                container: self.ids.trigger_overview_container,
                border: self.ids.trigger_overview_border,
                title_widget: self.ids.trigger_overview_title,
                status_label_widget: self.ids.trigger_overview_status_label,
                status_value_widget: self.ids.trigger_overview_status_value,
                offset_label_widget: self.ids.trigger_overview_offset_label,
                offset_value_widget: self.ids.trigger_overview_offset_value,
                configure_widget: self.ids.trigger_overview_configure,
                expiratory_term_widget: self.ids.trigger_overview_expiratory_term,
                plateau_duration_widget: self.ids.trigger_overview_plateau_duration,
                background_color: color::BLUE,
                width: TELEMETRY_WIDGET_BOTTOM_SIZE_WIDTH,
                height: LAYOUT_FOOTER_SIZE_HEIGHT,
                x_position: TELEMETRY_WIDGET_BOTTOM_SIZE_WIDTH + TELEMETRY_WIDGET_SPACING_SIDES,
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
        wrapper: Option<WidgetId>,
        colors: Option<(Color, Color)>,
    ) {
        self.widgets.render(ControlWidgetType::Modal(modal::Config {
            parent: wrapper.unwrap_or(self.ids.background),
            background: self.ids.modal_background,
            container_borders: self.ids.modal_container_borders,
            container: self.ids.modal_container,
            validate,
            width,
            height,
            padding,
            colors,
        }));
    }

    pub fn render_settings(&mut self, settings: &'a ChipSettings, modals: &ScreenModalsOpen) {
        if modals.run {
            self.render_run_settings(&settings.run);
        } else if modals.snooze {
            self.render_snooze_settings(&settings.snooze);
        } else if modals.advanced {
            self.render_advanced_settings();
        } else if modals.trigger {
            self.render_trigger_settings(&settings.trigger);
        } else if modals.expiration_term {
            self.render_expiration_term_settings(&settings.expiration_term);
        } else if modals.pressure {
            self.render_pressure_settings(&settings.pressure);
        } else if modals.cycles {
            self.render_cycles_settings(&settings.cycles);
        }
    }

    fn render_run_settings(&mut self, settings: &'a SettingsRun) {
        self.render_modal(
            RUN_SETTINGS_MODAL_WIDTH,
            RUN_SETTINGS_MODAL_HEIGTH,
            Some(RUN_SETTINGS_MODAL_PADDING),
            Some((self.ids.modal_validate, self.ids.modal_validate_text)),
            None,
            None,
        );

        self.widgets
            .render(ControlWidgetType::RunSettings(run_settings::Config {
                width: RUN_SETTINGS_MODAL_WIDTH,
                height: RUN_SETTINGS_MODAL_HEIGTH
                    - MODAL_VALIDATE_BUTTON_HEIGHT
                    - (RUN_SETTINGS_MODAL_PADDING * 2.0),
                run_settings: settings,

                status_container_parent: self.ids.modal_container,
                status_container_widget: self.ids.run_status_container,
                status_enabled_text_widget: self.ids.run_status_text,
                status_enabled_button_widget: self.ids.run_status_button,
                status_enabled_button_text_widget: self.ids.run_status_button_text,
            }));
    }

    fn render_snooze_settings(&mut self, settings: &'a SettingsSnooze) {
        self.render_modal(
            SNOOZE_SETTINGS_MODAL_WIDTH,
            SNOOZE_SETTINGS_MODAL_HEIGTH,
            Some(SNOOZE_SETTINGS_MODAL_PADDING),
            Some((self.ids.modal_validate, self.ids.modal_validate_text)),
            None,
            None,
        );

        self.widgets
            .render(ControlWidgetType::SnoozeSettings(snooze_settings::Config {
                width: SNOOZE_SETTINGS_MODAL_WIDTH,
                height: SNOOZE_SETTINGS_MODAL_HEIGTH
                    - MODAL_VALIDATE_BUTTON_HEIGHT
                    - (SNOOZE_SETTINGS_MODAL_PADDING * 2.0),
                snooze_settings: settings,

                container_parent: self.ids.modal_container,
                container_widget: self.ids.snooze_container,
                alarms_enabled_text_widget: self.ids.snooze_alarms_text,
                alarms_enabled_button_widget: self.ids.snooze_alarms_button,
                alarms_enabled_button_text_widget: self.ids.snooze_alarms_button_text,
            }));
    }

    fn render_advanced_settings(&mut self) {
        self.render_modal(
            ADVANCED_SETTINGS_MODAL_WIDTH,
            ADVANCED_SETTINGS_MODAL_HEIGTH,
            Some(ADVANCED_SETTINGS_MODAL_PADDING),
            Some((self.ids.modal_validate, self.ids.modal_validate_text)),
            None,
            None,
        );

        self.widgets.render(ControlWidgetType::AdvancedSettings(
            advanced_settings::Config {
                width: ADVANCED_SETTINGS_MODAL_WIDTH,
                height: ADVANCED_SETTINGS_MODAL_HEIGTH
                    - MODAL_VALIDATE_BUTTON_HEIGHT
                    - (ADVANCED_SETTINGS_MODAL_PADDING * 2.0),

                last_tick: self.last_tick,
                machine_snapshot: &self.machine_snapshot.unwrap(),
                data_snapshot: self.data_snapshot,

                advanced_container_parent: self.ids.modal_container,
                advanced_container_widget: self.ids.advanced_container,
                advanced_container_line_labels: &self.ids.advanced_line_labels,
                advanced_container_line_values: &self.ids.advanced_line_values,
            },
        ));
    }

    fn render_trigger_settings(&mut self, settings: &'a SettingsTrigger) {
        self.render_modal(
            TRIGGER_SETTINGS_MODAL_WIDTH,
            TRIGGER_SETTINGS_MODAL_HEIGTH,
            Some(TRIGGER_SETTINGS_MODAL_PADDING),
            Some((self.ids.modal_validate, self.ids.modal_validate_text)),
            None,
            None,
        );

        self.widgets.render(ControlWidgetType::TriggerSettings(
            trigger_settings::Config {
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

    fn render_expiration_term_settings(&mut self, settings: &'a SettingsExpirationTerm) {
        self.render_modal(
            EXPIRATION_TERM_SETTINGS_MODAL_WIDTH,
            EXPIRATION_TERM_SETTINGS_MODAL_HEIGTH,
            Some(EXPIRATION_TERM_SETTINGS_MODAL_PADDING),
            Some((self.ids.modal_validate, self.ids.modal_validate_text)),
            None,
            None,
        );

        self.widgets
            .render(ControlWidgetType::ExpirationTermSettings(
                expiration_term_settings::Config {
                    width: EXPIRATION_TERM_SETTINGS_MODAL_WIDTH,
                    height: EXPIRATION_TERM_SETTINGS_MODAL_HEIGTH
                        - MODAL_VALIDATE_BUTTON_HEIGHT
                        - (EXPIRATION_TERM_SETTINGS_MODAL_PADDING * 2.0),
                    expiration_term_settings: settings,

                    expiration_term_container_parent: self.ids.modal_container,
                    expiration_term_container_widget: self.ids.expiration_term_container,
                    expiration_term_more_button_widget: self.ids.expiration_term_more_button,
                    expiration_term_more_button_text_widget: self
                        .ids
                        .expiration_term_more_button_text,
                    expiration_term_less_button_widget: self.ids.expiration_term_less_button,
                    expiration_term_less_button_text_widget: self
                        .ids
                        .expiration_term_less_button_text,
                    expiration_term_text_widget: self.ids.expiration_term_text,
                    expiration_term_value_widget: self.ids.expiration_term_value,
                },
            ));
    }

    fn render_pressure_settings(&mut self, settings: &'a SettingsPressure) {
        self.render_modal(
            PRESSURE_SETTINGS_MODAL_WIDTH,
            PRESSURE_SETTINGS_MODAL_HEIGTH,
            Some(PRESSURE_SETTINGS_MODAL_PADDING),
            Some((self.ids.modal_validate, self.ids.modal_validate_text)),
            None,
            None,
        );

        self.widgets.render(ControlWidgetType::PressureSettings(
            pressure_settings::Config {
                width: PRESSURE_SETTINGS_MODAL_WIDTH,
                height: PRESSURE_SETTINGS_MODAL_HEIGTH
                    - MODAL_VALIDATE_BUTTON_HEIGHT
                    - (PRESSURE_SETTINGS_MODAL_PADDING * 2.0),
                pressure_settings: settings,

                pressure_container_parent: self.ids.modal_container,
                pressure_container_widget: self.ids.pressure_container,
                pressure_plateau_more_button_widget: self.ids.pressure_plateau_more_button,
                pressure_plateau_more_button_text_widget: self
                    .ids
                    .pressure_plateau_more_button_text,
                pressure_plateau_less_button_widget: self.ids.pressure_plateau_less_button,
                pressure_plateau_less_button_text_widget: self
                    .ids
                    .pressure_plateau_less_button_text,
                pressure_plateau_text_widget: self.ids.pressure_plateau_text,
                pressure_plateau_value_widget: self.ids.pressure_plateau_value,
                pressure_peep_more_button_widget: self.ids.pressure_peep_more_button,
                pressure_peep_more_button_text_widget: self.ids.pressure_peep_more_button_text,
                pressure_peep_less_button_widget: self.ids.pressure_peep_less_button,
                pressure_peep_less_button_text_widget: self.ids.pressure_peep_less_button_text,
                pressure_peep_text_widget: self.ids.pressure_peep_text,
                pressure_peep_value_widget: self.ids.pressure_peep_value,
            },
        ));
    }

    fn render_cycles_settings(&mut self, settings: &'a SettingsCycles) {
        self.render_modal(
            CYCLES_SETTINGS_MODAL_WIDTH,
            CYCLES_SETTINGS_MODAL_HEIGTH,
            Some(CYCLES_SETTINGS_MODAL_PADDING),
            Some((self.ids.modal_validate, self.ids.modal_validate_text)),
            None,
            None,
        );

        self.widgets
            .render(ControlWidgetType::CyclesSettings(cycles_settings::Config {
                width: CYCLES_SETTINGS_MODAL_WIDTH,
                height: CYCLES_SETTINGS_MODAL_HEIGTH
                    - MODAL_VALIDATE_BUTTON_HEIGHT
                    - (CYCLES_SETTINGS_MODAL_PADDING * 2.0),
                cycles_settings: settings,

                cycles_container_parent: self.ids.modal_container,
                cycles_container_widget: self.ids.cycles_container,
                cycles_more_button_widget: self.ids.cycles_more_button,
                cycles_more_button_text_widget: self.ids.cycles_more_button_text,
                cycles_less_button_widget: self.ids.cycles_less_button,
                cycles_less_button_text_widget: self.ids.cycles_less_button_text,
                cycles_text_widget: self.ids.cycles_text,
                cycles_value_widget: self.ids.cycles_value,
            }));
    }
}
