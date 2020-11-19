// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use chrono::{offset::Utc, DateTime};
use conrod_core::color::{self, Color};

use telemetry::alarm::AlarmCode;
use telemetry::structures::{AlarmPriority, DataSnapshot, MachineStateSnapshot};

use crate::chip::settings::{
    cycles::SettingsCycles, expiration_term::SettingsExpirationTerm, mode::SettingsMode,
    pressure::SettingsPressure, run::SettingsRun, snooze::SettingsSnooze, trigger::SettingsTrigger,
    ChipSettings,
};
use crate::chip::ChipError;
use crate::config::environment::*;
use crate::utilities::units::{convert_mmh2o_to_cmh2o, ConvertMode};
use crate::widget::*;
use crate::{APP_ARGS, APP_I18N};

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
    mode: bool,
    expiration_term: bool,
    pressure: bool,
    cycles: bool,
}

pub struct Screen<'a> {
    ids: &'a Ids,
    timers: (Option<DateTime<Utc>>, Option<u64>),
    ongoing_alarms: Option<&'a [(AlarmCode, AlarmPriority)]>,
    machine_snapshot: Option<&'a MachineStateSnapshot>,
    data_snapshot: Option<&'a DataSnapshot>,
    widgets: ControlWidget<'a>,
}

impl ScreenModalsOpen {
    pub fn from_states(states: &DisplayRendererStates) -> Self {
        ScreenModalsOpen {
            run: states.run_settings.is_open(),
            snooze: states.snooze_settings.is_open(),
            advanced: states.advanced_settings.is_open(),
            trigger: states.trigger_settings.is_open(),
            mode: states.mode_settings.is_open(),
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
        timers: (Option<DateTime<Utc>>, Option<u64>),
        ongoing_alarms: Option<&'a [(AlarmCode, AlarmPriority)]>,
        machine_snapshot: Option<&'a MachineStateSnapshot>,
        data_snapshot: Option<&'a DataSnapshot>,
    ) -> Screen<'a> {
        Screen {
            ids,
            timers,
            ongoing_alarms,
            machine_snapshot,
            data_snapshot,
            widgets: ControlWidget::new(ui, fonts),
        }
    }

    pub fn render_background(&mut self) {
        self.widgets
            .render(ControlWidgetType::Background(background::Config {
                color: color::BLACK,
                id: self.ids.background,
            }));
    }

    pub fn render_layout(&mut self, layout_data: DisplayDataLayout) {
        self.widgets
            .render(ControlWidgetType::Layout(layout::Config {
                width: DISPLAY_WINDOW_SIZE_WIDTH as _,
                height: DISPLAY_WINDOW_SIZE_HEIGHT as _,

                parent: self.ids.background,
                container: self.ids.layout_container,

                header: layout::Slice {
                    layout: self.ids.layout_header,
                    top: 0.0,
                    height: LAYOUT_HEADER_SIZE_HEIGHT,
                    texture: Some((
                        self.ids.layout_texture_header,
                        layout_data.texture_header_image_id,
                        (
                            LAYOUT_TEXTURE_HEADER_WIDTH as _,
                            LAYOUT_TEXTURE_HEADER_HEIGHT as _,
                        ),
                    )),
                },
                body: layout::Slice {
                    layout: self.ids.layout_body,
                    top: LAYOUT_HEADER_SIZE_HEIGHT,
                    height: LAYOUT_BODY_SIZE_HEIGHT,
                    texture: None,
                },
                footer: layout::Slice {
                    layout: self.ids.layout_footer,
                    top: LAYOUT_HEADER_SIZE_HEIGHT + LAYOUT_BODY_SIZE_HEIGHT,
                    height: LAYOUT_FOOTER_SIZE_HEIGHT,
                    texture: None,
                },
            }));
    }

    pub fn render_branding(
        &mut self,
        version_firmware: &'a str,
        version_control: &'a str,
        width: f64,
        height: f64,
    ) {
        self.widgets
            .render(ControlWidgetType::Branding(branding::Config {
                parent: self.ids.layout_header,
                version_firmware,
                version_control,
                width,
                height,
                ids: (self.ids.branding_container, self.ids.branding_text),
            }));
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
            .render(ControlWidgetType::Heartbeat(heartbeat::Config {
                data_pressure: heartbeat_data.data_pressure,
                peak_command: self.machine_snapshot.unwrap().peak_command,
                container: self.ids.layout_header,
                ground: self.ids.heartbeat_ground,
                surround: self.ids.heartbeat_surround,
                inner: self.ids.heartbeat_inner,
            }));
    }

    pub fn render_status(&mut self, status_data: DisplayDataStatus<'a>) {
        self.widgets
            .render(ControlWidgetType::Status(status::Config {
                container: self.ids.layout_header,
                wrapper: self.ids.status_wrapper,
                unit_box: self.ids.status_unit_box,
                unit_text: self.ids.status_unit_text,
                power_box: self.ids.status_power_box,
                power_text: self.ids.status_power_text,
                battery_level: status_data.battery_level,
                chip_state: status_data.chip_state,
                alarms: self.ongoing_alarms.unwrap(),
                recording: if APP_ARGS.is_recording() {
                    Some((
                        self.ids.status_recording_outer,
                        self.ids.status_recording_inner,
                    ))
                } else {
                    None
                },
            }));
    }

    pub fn render_controls(&mut self) {
        self.widgets
            .render(ControlWidgetType::Controls(controls::Config {
                container: self.ids.layout_header,
                wrapper: self.ids.controls_wrapper,
                run_button: self.ids.controls_button_run,
                snooze_button: self.ids.controls_button_snooze,
                advanced_button: self.ids.controls_button_advanced,
            }));
    }

    pub fn render_graph(&mut self, graph_data: DisplayDataGraph<'a>) {
        self.widgets.render(ControlWidgetType::Graph(graph::Config {
            width: graph_data.width,
            height: graph_data.height,
            parent: self.ids.layout_body,
            wrapper_id: self.ids.graph_wrapper,
            pressure_id: self.ids.graph_pressure,
            flow_id: self.ids.graph_flow,
            pressure_label_box_id: self.ids.graph_pressure_label_box,
            pressure_label_text_id: self.ids.graph_pressure_label_text,
            flow_label_box_id: self.ids.graph_flow_label_box,
            flow_label_text_id: self.ids.graph_flow_label_text,
            pressure_saturate_ids: (
                self.ids.graph_pressure_saturate_low,
                self.ids.graph_pressure_saturate_high,
            ),
            flow_saturate_ids: (
                self.ids.graph_flow_saturate_low,
                self.ids.graph_flow_saturate_high,
            ),
            boot_time: self.timers.0,
            last_tick: self.timers.1,
            data_pressure: graph_data.data_pressure,
            data_flow: graph_data.data_flow,
            chip_state: graph_data.chip_state,
            machine_snapshot: graph_data.machine_snapshot,
            plot_graphs: graph_data.plot_graphs,
        }));
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_running(
        &mut self,
        layout_data: DisplayDataLayout,
        branding_data: DisplayDataBranding<'a>,
        status_data: DisplayDataStatus<'a>,
        heartbeat_data: DisplayDataHeartbeat<'a>,
        graph_data: DisplayDataGraph<'a>,
        settings: &'a ChipSettings,
        modals: &ScreenModalsOpen,
    ) {
        // Render common background
        self.render_background();
        self.render_layout(layout_data);

        // Render top left elements
        self.render_branding(
            branding_data.firmware_version,
            RUNTIME_VERSION,
            branding_data.width,
            branding_data.height,
        );
        self.render_alarms();

        // Render top right elements
        self.render_heartbeat(heartbeat_data);
        self.render_status(status_data);
        self.render_controls();

        // Render middle elements
        self.render_graph(graph_data);

        // Render bottom elements
        self.render_telemetry(&settings.mode, &settings.trigger, &settings.expiration_term);

        // Render modals (as needed)
        self.render_settings(settings, modals);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_stop(
        &mut self,
        layout_data: DisplayDataLayout,
        branding_data: DisplayDataBranding<'a>,
        status_data: DisplayDataStatus<'a>,
        heartbeat_data: DisplayDataHeartbeat<'a>,
        graph_data: DisplayDataGraph<'a>,
        settings: &'a ChipSettings,
        modals: &ScreenModalsOpen,
    ) {
        // Render regular data as background (alias the running screen)
        self.render_running(
            layout_data,
            branding_data,
            status_data,
            heartbeat_data,
            graph_data,
            settings,
            modals,
        );

        // Render stop 'pseudo-modal'
        self.widgets.render(ControlWidgetType::Modal(modal::Config {
            parent: self.ids.graph_wrapper,
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
            background_sizes: Some((GRAPH_WIDTH, GRAPH_HEIGHT)),
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
        self.widgets.render(ControlWidgetType::Error(error::Config {
            container: self.ids.error_container,
            icon: self.ids.error_icon,
            text_wrapper: self.ids.error_text_wrapper,
            text_title: self.ids.error_text_title,
            text_message: self.ids.error_text_message,
            width: config.width,
            height: config.height,
            image: config.image_id,
            message,
        }));
    }

    pub fn render_initializing(&mut self, config: DisplayDataBootloader) {
        self.render_background();

        self.widgets
            .render(ControlWidgetType::Initializing(initializing::Config {
                container: self.ids.initializing_container,
                logo: self.ids.initializing_logo,
                text: self.ids.initializing_text,
                width: config.width,
                height: config.height,
                image: config.image_id,
                connecting: config.connecting,
            }));
    }

    pub fn render_telemetry(
        &mut self,
        mode: &'a SettingsMode,
        trigger: &'a SettingsTrigger,
        expiration_term: &'a SettingsExpirationTerm,
    ) {
        let machine_snapshot = self.machine_snapshot.unwrap();

        // Initialize the pressure graph widget
        self.widgets.render(ControlWidgetType::TelemetryContainer(
            telemetry_container::Config {
                width: TELEMETRY_WIDGET_RIGHT_SIZE_WIDTH,
                height: DISPLAY_WINDOW_SIZE_HEIGHT as f64 - LAYOUT_HEADER_SIZE_HEIGHT,
                parent: self.ids.graph_wrapper,
                id: self.ids.telemetry_widgets_right,
            },
        ));

        // Check if at least a pressure value is known (otherwise, all pressure widgets should \
        //   show as empty)
        let has_target_pressure = machine_snapshot.peak_command > 0
            || machine_snapshot.plateau_command > 0
            || machine_snapshot.peep_command > 0;

        // Initialize the mode widget
        self.widgets
            .render(ControlWidgetType::ModeOverview(mode_overview::Config {
                parent: self.ids.telemetry_widgets_right,
                container: self.ids.mode_overview_container,
                separator: self.ids.mode_overview_separator,
                text_class: self.ids.mode_overview_text_class,
                text_type: self.ids.mode_overview_text_type,
                background_color: Color::Rgba(1.0, 1.0, 1.0, 1.0),
                width: TELEMETRY_WIDGET_RIGHT_SIZE_WIDTH,
                height: TELEMETRY_WIDGET_RIGHT_MODE_HEIGHT,
                mode_settings: mode,
            }));

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
                unit: APP_I18N.t("telemetry-unit-cmh2o"),
                ids: (
                    self.ids.telemetry_widgets_right,
                    self.ids.peak_parent,
                    self.ids.peak_title,
                    self.ids.peak_value_measured,
                    (
                        self.ids.peak_value_arrow_main,
                        self.ids.peak_value_arrow_line,
                    ),
                    self.ids.peak_value_target,
                    Some(self.ids.peak_unit),
                ),
                x_position: 0.0,
                y_position: TELEMETRY_WIDGET_RIGHT_POSITION_Y_BASE
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
                unit: APP_I18N.t("telemetry-unit-cmh2o"),
                ids: (
                    self.ids.telemetry_widgets_right,
                    self.ids.plateau_parent,
                    self.ids.plateau_title,
                    self.ids.plateau_value_measured,
                    (
                        self.ids.plateau_value_arrow_main,
                        self.ids.plateau_value_arrow_line,
                    ),
                    self.ids.plateau_value_target,
                    Some(self.ids.plateau_unit),
                ),
                x_position: 0.0,
                y_position: TELEMETRY_WIDGET_RIGHT_POSITION_Y_BASE
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
                unit: APP_I18N.t("telemetry-unit-cmh2o"),
                ids: (
                    self.ids.telemetry_widgets_right,
                    self.ids.peep_parent,
                    self.ids.peep_title,
                    self.ids.peep_value_measured,
                    (
                        self.ids.peep_value_arrow_main,
                        self.ids.peep_value_arrow_line,
                    ),
                    self.ids.peep_value_target,
                    Some(self.ids.peep_unit),
                ),
                x_position: 0.0,
                y_position: TELEMETRY_WIDGET_RIGHT_POSITION_Y_BASE
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
                    Some(machine_snapshot.previous_cpm.unwrap_or(0).to_string())
                },
                value_target: Some(if machine_snapshot.cpm_command == 0 {
                    TELEMETRY_WIDGET_VALUE_EMPTY.to_owned()
                } else {
                    machine_snapshot.cpm_command.to_string()
                }),
                unit: APP_I18N.t("telemetry-unit-per-minute"),
                ids: (
                    self.ids.layout_footer,
                    self.ids.cycles_parent,
                    self.ids.cycles_title,
                    self.ids.cycles_value_measured,
                    (
                        self.ids.cycles_value_arrow_main,
                        self.ids.cycles_value_arrow_line,
                    ),
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
                unit: APP_I18N.t("telemetry-unit-milliliters"),
                ids: (
                    self.ids.cycles_parent,
                    self.ids.tidal_parent,
                    self.ids.tidal_title,
                    self.ids.tidal_value_measured,
                    (
                        self.ids.tidal_value_arrow_main,
                        self.ids.tidal_value_arrow_line,
                    ),
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
                        format!(
                            "{}:{}",
                            TELEMETRY_WIDGET_CYCLES_RATIO_INSPIRATION, expiratory_term_value,
                        )
                    } else {
                        format!(
                            "{}:{:.1}",
                            TELEMETRY_WIDGET_CYCLES_RATIO_INSPIRATION, expiratory_term_value,
                        )
                    }
                }),
                value_target: None,
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
                    (
                        self.ids.ratio_value_arrow_main,
                        self.ids.ratio_value_arrow_line,
                    ),
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

    fn render_modal(&mut self, width: f64, height: f64, padding: Option<f64>) {
        self.widgets.render(ControlWidgetType::Modal(modal::Config {
            parent: self.ids.background,
            background: self.ids.modal_background,
            container_borders: self.ids.modal_container_borders,
            container: self.ids.modal_container,
            validate: Some((self.ids.modal_validate, self.ids.modal_validate_text)),
            width,
            height,
            padding,
            colors: None,
            background_sizes: None,
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
        } else if modals.mode {
            self.render_mode_settings(&settings.mode);
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
        );

        self.widgets.render(ControlWidgetType::AdvancedSettings(
            advanced_settings::Config {
                width: ADVANCED_SETTINGS_MODAL_WIDTH,
                height: ADVANCED_SETTINGS_MODAL_HEIGTH
                    - MODAL_VALIDATE_BUTTON_HEIGHT
                    - (ADVANCED_SETTINGS_MODAL_PADDING * 2.0),

                last_tick: self.timers.1,
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

    fn render_mode_settings(&mut self, settings: &'a SettingsMode) {
        self.render_modal(
            MODE_SETTINGS_MODAL_WIDTH,
            MODE_SETTINGS_MODAL_HEIGTH,
            Some(MODE_SETTINGS_MODAL_PADDING),
        );

        // TODO: fill modal w/ form
    }

    fn render_expiration_term_settings(&mut self, settings: &'a SettingsExpirationTerm) {
        self.render_modal(
            EXPIRATION_TERM_SETTINGS_MODAL_WIDTH,
            EXPIRATION_TERM_SETTINGS_MODAL_HEIGTH,
            Some(EXPIRATION_TERM_SETTINGS_MODAL_PADDING),
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
