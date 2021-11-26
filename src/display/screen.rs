// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use chrono::{offset::Utc, DateTime};
use conrod_core::color::{self, Color};

use makair_telemetry::alarm::AlarmCode;
use makair_telemetry::structures::{
    AlarmPriority, DataSnapshot, MachineStateSnapshot, VentilationMode, VentilationModeClass,
};

use crate::chip::settings::{
    advanced::SettingsAdvanced, mode::SettingsMode, preset::SettingsPreset, run::SettingsRun,
    snooze::SettingsSnooze, ChipSettings,
};
use crate::config::environment::*;
use crate::locale::end_of_line::end_of_line_to_locales;
use crate::locale::error::error_to_locales;
use crate::utilities::units::{convert_ml_to_l, convert_mmh2o_to_cmh2o, ConvertMode};
use crate::widget::*;
use crate::{APP_ARGS, APP_I18N};

use super::data::*;
use super::fonts::Fonts;
use super::identifiers::Ids;
use super::renderer::DisplayRendererStates;
use super::widget::{ControlWidget, ControlWidgetType};

pub struct ScreenModalsOpen {
    preset: bool,
    run: bool,
    snooze: bool,
    advanced: bool,
    mode: bool,
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
            preset: states.preset_settings.is_open(),
            run: states.run_settings.is_open(),
            snooze: states.snooze_settings.is_open(),
            advanced: states.advanced_settings.is_open(),
            mode: states.mode_settings.is_open(),
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

    #[allow(clippy::too_many_arguments)]
    pub fn render_running(
        &mut self,
        layout_data: DisplayDataLayout,
        branding_data: DisplayDataBranding<'a>,
        status_data: DisplayDataStatus<'a>,
        heartbeat_data: DisplayDataHeartbeat<'a>,
        graph_data: DisplayDataGraph<'a>,
        settings_data: DisplayDataSettings<'a>,
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
        self.render_telemetry(&settings.mode);

        // Render modals (as needed)
        self.render_settings(settings, modals, settings_data);
    }

    #[allow(clippy::too_many_arguments)]
    pub fn render_stop(
        &mut self,
        layout_data: DisplayDataLayout,
        branding_data: DisplayDataBranding<'a>,
        status_data: DisplayDataStatus<'a>,
        heartbeat_data: DisplayDataHeartbeat<'a>,
        graph_data: DisplayDataGraph<'a>,
        settings_data: DisplayDataSettings<'a>,
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
            settings_data,
            settings,
            modals,
        );

        // Render stop 'pseudo-modal'
        self.widgets.render(ControlWidgetType::Modal(modal::Config {
            parent: self.ids.graph_wrapper,
            background: self.ids.stop_background,
            container_borders: self.ids.stop_container_borders,
            container: self.ids.stop_container,
            close: None,
            save: None,
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
        // Generate error texts
        let error_texts = error_to_locales(config.error);

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
            title: error_texts.0,
            message: error_texts.1,
        }));
    }

    pub fn render_end_of_line(&mut self, config: DisplayDataEndOfLine<'a>) {
        // Generate end-of-line texts
        let end_of_line_texts = end_of_line_to_locales(config.eol);

        // Render background
        self.render_background();

        // Render end-of-line
        self.widgets
            .render(ControlWidgetType::EndOfLine(end_of_line::Config {
                title_wrapper: self.ids.end_of_line_title_wrapper,
                title_primary: self.ids.end_of_line_title_primary,
                title_secondary: self.ids.end_of_line_title_secondary,
                title_separator: self.ids.end_of_line_title_separator,
                steps_wrapper: self.ids.end_of_line_steps_wrapper,
                steps_items: self.ids.end_of_line_steps_items,
                steps_progress: &self.ids.end_of_line_steps_progress,
                steps_circles: &self.ids.end_of_line_steps_circles,
                steps_indexes: &self.ids.end_of_line_steps_indexes,
                content_wrapper: self.ids.end_of_line_content_wrapper,
                content_box: self.ids.end_of_line_content_box,
                content_icon: self.ids.end_of_line_content_icon,
                content_text_wrapper: self.ids.end_of_line_content_text_wrapper,
                content_text_title: self.ids.end_of_line_content_text_title,
                content_text_message: self.ids.end_of_line_content_text_message,
                content_details_box: self.ids.end_of_line_content_details_box,
                content_details_text: self.ids.end_of_line_content_details_text,
                content_button: self.ids.end_of_line_content_button,
                content_button_text: self.ids.end_of_line_content_button_text,
                error: config.error,
                success: config.success,
                confirm: config.confirm,
                step: config.step,
                icon: config.icon_image_id,
                title: end_of_line_texts.0,
                message: end_of_line_texts.1,
                details: end_of_line_texts.2,
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

    fn render_background(&mut self) {
        self.widgets
            .render(ControlWidgetType::Background(background::Config {
                color: color::BLACK,
                id: self.ids.background,
            }));
    }

    fn render_layout(&mut self, layout_data: DisplayDataLayout) {
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

    fn render_branding(
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

    fn render_alarms(&mut self) {
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

    fn render_heartbeat(&mut self, heartbeat_data: DisplayDataHeartbeat<'a>) {
        let machine_snapshot = self.machine_snapshot.unwrap();

        self.widgets
            .render(ControlWidgetType::Heartbeat(heartbeat::Config {
                mode: machine_snapshot.ventilation_mode,
                data_pressure: heartbeat_data.data_pressure,
                peak_command: machine_snapshot.peak_command,
                peak_alarm: machine_snapshot
                    .peak_pressure_alarm_threshold
                    .map(|value| convert_mmh2o_to_cmh2o(ConvertMode::Rounded, value as f64) as u8)
                    .unwrap_or(0),
                container: self.ids.layout_header,
                ground: self.ids.heartbeat_ground,
                surround: self.ids.heartbeat_surround,
                inner: self.ids.heartbeat_inner,
            }));
    }

    fn render_status(&mut self, status_data: DisplayDataStatus<'a>) {
        self.widgets
            .render(ControlWidgetType::Status(status::Config {
                container: self.ids.layout_header,
                wrapper: self.ids.status_wrapper,
                unit_box: self.ids.status_unit_box,
                unit_text: self.ids.status_unit_text,
                power_box: self.ids.status_power_box,
                power_text: self.ids.status_power_text,
                battery_soc: status_data.battery_soc,
                chip_state: status_data.chip_state,
                data_snapshot: self.data_snapshot,
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

    fn render_controls(&mut self) {
        self.widgets
            .render(ControlWidgetType::Controls(controls::Config {
                container: self.ids.layout_header,
                wrapper: self.ids.controls_wrapper,
                run_button: self.ids.controls_button_run,
                snooze_button: self.ids.controls_button_snooze,
                advanced_button: self.ids.controls_button_advanced,
            }));
    }

    fn render_graph(&mut self, graph_data: DisplayDataGraph<'a>) {
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

    fn render_telemetry(&mut self, mode: &'a SettingsMode) {
        let machine_snapshot = self.machine_snapshot.unwrap();

        // Check if at least a pressure value is known (otherwise, all pressure widgets should \
        //   show as empty)
        let has_target_pressure = machine_snapshot.peak_command > 0
            || machine_snapshot.plateau_command > 0
            || machine_snapshot.peep_command > 0;

        // Initialize the pressure graph widget
        self.render_telemetry_graph();

        // Initialize the mode widget
        self.render_telemetry_mode_overview(mode);

        // Initialize the peak widget
        self.render_telemetry_peak(machine_snapshot, has_target_pressure);

        // Initialize the plateau widget
        self.render_telemetry_plateau(machine_snapshot, mode, has_target_pressure);

        // Initialize the PEEP widget
        self.render_telemetry_peep(machine_snapshot, has_target_pressure);

        // Initialize the cycles widget
        self.render_telemetry_cycles(machine_snapshot);

        // Initialize the tidal widget
        self.render_telemetry_tidal(machine_snapshot, mode);

        // Initialize the minute volume widget
        self.render_telemetry_minute_volume(machine_snapshot);

        // Initialize the ratio widget
        self.render_telemetry_ratio(machine_snapshot, mode);
    }

    fn render_telemetry_graph(&mut self) {
        self.widgets.render(ControlWidgetType::TelemetryContainer(
            telemetry_container::Config {
                width: TELEMETRY_WIDGET_RIGHT_SIZE_WIDTH,
                height: DISPLAY_WINDOW_SIZE_HEIGHT as f64 - LAYOUT_HEADER_SIZE_HEIGHT,
                parent: self.ids.graph_wrapper,
                id: self.ids.telemetry_widgets_right,
            },
        ));
    }

    fn render_telemetry_mode_overview(&mut self, mode: &'a SettingsMode) {
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
    }

    fn render_telemetry_peak(
        &mut self,
        machine_snapshot: &MachineStateSnapshot,
        has_target_pressure: bool,
    ) {
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
                value_target: None,
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
    }

    fn render_telemetry_plateau(
        &mut self,
        machine_snapshot: &MachineStateSnapshot,
        mode: &'a SettingsMode,
        has_target_pressure: bool,
    ) {
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
                value_target: if !has_target_pressure
                    || mode.live.mode.class() != VentilationModeClass::Pressure
                {
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
    }

    fn render_telemetry_peep(
        &mut self,
        machine_snapshot: &MachineStateSnapshot,
        has_target_pressure: bool,
    ) {
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
    }

    fn render_telemetry_cycles(&mut self, machine_snapshot: &MachineStateSnapshot) {
        // Acquire measured cycles per minute
        let measured_cpm = machine_snapshot.previous_cpm.unwrap_or(0);

        self.widgets
            .render(ControlWidgetType::TelemetryView(telemetry_view::Config {
                title: APP_I18N.t("telemetry-label-cycles"),
                value_measured: if machine_snapshot.cpm_command == 0 {
                    None
                } else {
                    Some(measured_cpm.to_string())
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
    }

    fn render_telemetry_tidal(
        &mut self,
        machine_snapshot: &MachineStateSnapshot,
        mode: &'a SettingsMode,
    ) {
        // Acquire measured volume
        let measured_volume = machine_snapshot.previous_volume.unwrap_or(0);

        // Check if should show target tidal volume
        let has_target_volume_tidal =
            mode.live.volume_tidal > 0 && mode.live.mode.class() == VentilationModeClass::Volume;

        self.widgets
            .render(ControlWidgetType::TelemetryView(telemetry_view::Config {
                title: APP_I18N.t("telemetry-label-tidal"),
                value_measured: Some(if measured_volume > 0 || has_target_volume_tidal {
                    measured_volume.to_string()
                } else {
                    TELEMETRY_WIDGET_VALUE_EMPTY.to_owned()
                }),
                value_target: if has_target_volume_tidal {
                    Some(mode.live.volume_tidal.to_string())
                } else {
                    None
                },
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
    }

    fn render_telemetry_minute_volume(&mut self, machine_snapshot: &MachineStateSnapshot) {
        // Acquire measured volume & cycles per minute
        let measured_volume = machine_snapshot.previous_volume.unwrap_or(0);
        let measured_cpm = machine_snapshot.previous_cpm.unwrap_or(0);

        self.widgets
            .render(ControlWidgetType::TelemetryView(telemetry_view::Config {
                title: APP_I18N.t("telemetry-label-minute-volume"),
                value_measured: Some(if measured_cpm > 0 && measured_volume > 0 {
                    format!(
                        "{:.1}",
                        convert_ml_to_l(
                            ConvertMode::WithDecimals,
                            (measured_cpm as u16 * measured_volume) as f64,
                        )
                    )
                } else {
                    TELEMETRY_WIDGET_VALUE_EMPTY.to_owned()
                }),
                value_target: None,
                unit: APP_I18N.t("telemetry-unit-lpm"),
                ids: (
                    self.ids.tidal_parent,
                    self.ids.minute_volume_parent,
                    self.ids.minute_volume_title,
                    self.ids.minute_volume_value_measured,
                    (
                        self.ids.minute_volume_value_arrow_main,
                        self.ids.minute_volume_value_arrow_line,
                    ),
                    self.ids.minute_volume_value_target,
                    Some(self.ids.minute_volume_unit),
                ),
                x_position: TELEMETRY_WIDGET_BOTTOM_SIZE_WIDTH + TELEMETRY_WIDGET_SPACING_SIDES,
                y_position: 0.0,
                background_color: Color::Rgba(52.0 / 255.0, 52.0 / 255.0, 52.0 / 255.0, 1.0),
                width: TELEMETRY_WIDGET_BOTTOM_SIZE_WIDTH,
                height: LAYOUT_FOOTER_SIZE_HEIGHT,
            }));
    }

    fn render_telemetry_ratio(
        &mut self,
        machine_snapshot: &MachineStateSnapshot,
        mode: &'a SettingsMode,
    ) {
        // Acquire measured inspiratory duration & cycles per minute
        let measured_inspiratory_duration =
            machine_snapshot.previous_inspiratory_duration.unwrap_or(0);
        let measured_cpm = machine_snapshot.previous_cpm.unwrap_or(0);

        // Check if target inspiration duration can be shown
        let has_target_inspiration_duration = measured_inspiratory_duration > 0
            && (mode.live.mode == VentilationMode::PC_CMV
                || mode.live.mode == VentilationMode::PC_AC);

        // Compute internal values
        let computed_respiratory_time = if measured_cpm > 0 {
            60000.0 / measured_cpm as f64
        } else {
            0.0
        };
        let computed_inspiratory_duration = measured_inspiratory_duration as f64;

        let computed_expiratory_term =
            if computed_inspiratory_duration > 0.0 && computed_respiratory_time > 0.0 {
                (computed_respiratory_time - computed_inspiratory_duration)
                    / computed_inspiratory_duration
            } else {
                0.0
            };

        // Important: if the ratio has decimals, then show them (to the first decimal). If it \
        //   has no decimals (eg. '2.0'), then show it as an integer.
        self.widgets
            .render(ControlWidgetType::TelemetryView(telemetry_view::Config {
                title: APP_I18N.t("telemetry-label-ratio"),
                value_measured: Some(
                    if measured_inspiratory_duration > 0 || has_target_inspiration_duration {
                        measured_inspiratory_duration.to_string()
                    } else {
                        TELEMETRY_WIDGET_VALUE_EMPTY.to_owned()
                    },
                ),
                value_target: if has_target_inspiration_duration {
                    machine_snapshot
                        .inspiratory_duration_command
                        .map(|target_inspiratory_duration| target_inspiratory_duration.to_string())
                } else {
                    None
                },
                unit: if computed_expiratory_term < 1.0 {
                    APP_I18N.t("telemetry-unit-milliseconds")
                } else {
                    format!(
                        "{} ({} {})",
                        &APP_I18N.t("telemetry-unit-milliseconds"),
                        &APP_I18N.t("telemetry-label-ratio-details"),
                        format!(
                            "{}:{:.1}",
                            TELEMETRY_WIDGET_CYCLES_RATIO_INSPIRATION, computed_expiratory_term,
                        )
                    )
                },
                ids: (
                    self.ids.minute_volume_parent,
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
    }

    fn render_modal(&mut self, width: f64, height: f64, padding: Option<f64>, with_save: bool) {
        self.widgets.render(ControlWidgetType::Modal(modal::Config {
            parent: self.ids.background,
            background: self.ids.modal_background,
            container_borders: self.ids.modal_container_borders,
            container: self.ids.modal_container,
            close: Some((self.ids.modal_close, self.ids.modal_close_text)),
            save: if with_save {
                Some((self.ids.modal_save, self.ids.modal_save_text))
            } else {
                None
            },
            width,
            height,
            padding,
            colors: None,
            background_sizes: None,
        }));
    }

    fn render_settings(
        &mut self,
        settings: &'a ChipSettings,
        modals: &ScreenModalsOpen,
        settings_data: DisplayDataSettings<'a>,
    ) {
        if modals.preset {
            self.render_preset_settings(&settings.preset, settings_data);
        } else if modals.run {
            self.render_run_settings(&settings.run);
        } else if modals.snooze {
            self.render_snooze_settings(&settings.snooze);
        } else if modals.advanced {
            self.render_advanced_settings(&settings.advanced);
        } else if modals.mode {
            self.render_mode_settings(&settings.mode);
        }
    }

    fn render_preset_settings(
        &mut self,
        settings: &'a SettingsPreset,
        settings_data: DisplayDataSettings<'a>,
    ) {
        self.render_modal(
            PRESET_SETTINGS_MODAL_WIDTH,
            PRESET_SETTINGS_MODAL_HEIGTH,
            Some(PRESET_SETTINGS_MODAL_PADDING),
            true,
        );

        self.widgets
            .render(ControlWidgetType::PresetSettings(preset_settings::Config {
                width: PRESET_SETTINGS_MODAL_WIDTH - (PRESET_SETTINGS_MODAL_PADDING * 2.0),
                height: PRESET_SETTINGS_MODAL_HEIGTH
                    - MODAL_FINALIZE_BUTTON_HEIGHT
                    - (PRESET_SETTINGS_MODAL_PADDING * 3.0),
                preset_settings: settings,

                container_parent: self.ids.modal_container,
                container_widget: self.ids.preset_settings_container,

                title_primary: self.ids.preset_settings_title_primary,
                title_secondary: self.ids.preset_settings_title_secondary,

                content_wrapper: self.ids.preset_settings_content_wrapper,
                content_image: self.ids.preset_settings_content_image,
                content_separator: self.ids.preset_settings_content_separator,
                content_form_wrapper: self.ids.preset_settings_content_form_wrapper,

                field_gender_ids: gen_render_preset_settings_field_ids!(self, gender),
                field_age_ids: gen_render_preset_settings_field_ids!(self, age),
                field_height_ids: gen_render_preset_settings_field_ids!(self, height),

                child_image: settings_data.images.patient_child,
                teenager_image: settings_data.images.patient_teenager,
                adult_image: settings_data.images.patient_adult,
            }));
    }

    fn render_run_settings(&mut self, settings: &'a SettingsRun) {
        self.render_modal(
            RUN_SETTINGS_MODAL_WIDTH,
            RUN_SETTINGS_MODAL_HEIGTH,
            Some(RUN_SETTINGS_MODAL_PADDING),
            false,
        );

        self.widgets
            .render(ControlWidgetType::RunSettings(run_settings::Config {
                width: RUN_SETTINGS_MODAL_WIDTH,
                height: RUN_SETTINGS_MODAL_HEIGTH
                    - MODAL_FINALIZE_BUTTON_HEIGHT
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
            false,
        );

        self.widgets
            .render(ControlWidgetType::SnoozeSettings(snooze_settings::Config {
                width: SNOOZE_SETTINGS_MODAL_WIDTH,
                height: SNOOZE_SETTINGS_MODAL_HEIGTH
                    - MODAL_FINALIZE_BUTTON_HEIGHT
                    - (SNOOZE_SETTINGS_MODAL_PADDING * 2.0),
                snooze_settings: settings,

                container_parent: self.ids.modal_container,
                container_widget: self.ids.snooze_container,
                alarms_enabled_text_widget: self.ids.snooze_alarms_text,
                alarms_enabled_button_widget: self.ids.snooze_alarms_button,
                alarms_enabled_button_text_widget: self.ids.snooze_alarms_button_text,
            }));
    }

    fn render_advanced_settings(&mut self, settings: &'a SettingsAdvanced) {
        self.render_modal(
            ADVANCED_SETTINGS_MODAL_WIDTH,
            ADVANCED_SETTINGS_MODAL_HEIGTH,
            Some(ADVANCED_SETTINGS_MODAL_PADDING),
            false,
        );

        self.widgets.render(ControlWidgetType::AdvancedSettings(
            advanced_settings::Config {
                width: ADVANCED_SETTINGS_MODAL_WIDTH,
                height: ADVANCED_SETTINGS_MODAL_HEIGTH
                    - MODAL_FINALIZE_BUTTON_HEIGHT
                    - (ADVANCED_SETTINGS_MODAL_PADDING * 2.0),

                advanced_settings: settings,

                last_tick: self.timers.1,
                machine_snapshot: self.machine_snapshot.unwrap(),
                data_snapshot: self.data_snapshot,
                alarms: self.ongoing_alarms.unwrap(),

                advanced_container_parent: self.ids.modal_container,
                advanced_container_widget: self.ids.advanced_container,
                advanced_container_line_labels: &self.ids.advanced_line_labels,
                advanced_container_line_values: &self.ids.advanced_line_values,

                advanced_group_wrapper: self.ids.advanced_group_wrapper,
                advanced_form_wrapper: self.ids.advanced_form_wrapper,

                advanced_group_tab_buttons: [
                    self.ids.advanced_group_tab_statistics_button,
                    self.ids.advanced_group_tab_settings_button,
                    self.ids.advanced_group_tab_simulator_button,
                ],

                advanced_group_tab_texts: [
                    self.ids.advanced_group_tab_statistics_text,
                    self.ids.advanced_group_tab_settings_text,
                    self.ids.advanced_group_tab_simulator_text,
                ],

                field_locale_ids: gen_render_advanced_settings_field_ids!(self, locale),
                text_date_ids: gen_render_advanced_settings_text_ids!(self, date),
                text_time_ids: gen_render_advanced_settings_text_ids!(self, time),
                text_timezone_ids: gen_render_advanced_settings_text_ids!(self, timezone),

                field_resistance_ids: gen_render_advanced_settings_field_ids!(self, resistance),
                field_compliance_ids: gen_render_advanced_settings_field_ids!(self, compliance),
                field_spontaneous_breath_rate_ids: gen_render_advanced_settings_field_ids!(self, spontaneous_breath_rate),
                field_spontaneous_breath_effort_ids: gen_render_advanced_settings_field_ids!(self, spontaneous_breath_effort),
                field_spontaneous_breath_duration_ids: gen_render_advanced_settings_field_ids!(self, spontaneous_breath_duration),
                field_acceleration_factor_ids: gen_render_advanced_settings_field_ids!(self, acceleration_factor),
            },
        ));
    }

    fn render_mode_settings(&mut self, settings: &'a SettingsMode) {
        self.render_modal(
            MODE_SETTINGS_MODAL_WIDTH,
            MODE_SETTINGS_MODAL_HEIGTH,
            Some(MODE_SETTINGS_MODAL_PADDING),
            settings.draft.is_some(),
        );

        self.widgets
            .render(ControlWidgetType::ModeSettings(mode_settings::Config {
                width: MODE_SETTINGS_MODAL_WIDTH,
                height: MODE_SETTINGS_MODAL_HEIGTH
                    - MODAL_FINALIZE_BUTTON_HEIGHT
                    - (MODE_SETTINGS_MODAL_PADDING * 2.0),

                mode_settings: settings,

                container_parent: self.ids.modal_container,
                container_widget: self.ids.mode_settings_container,
                selector_wrapper: self.ids.mode_settings_selector_wrapper,

                selector_tabs: [
                    self.ids.mode_settings_selector_tab_pc_cmv,
                    self.ids.mode_settings_selector_tab_pc_ac,
                    self.ids.mode_settings_selector_tab_pc_vsai,
                    self.ids.mode_settings_selector_tab_vc_cmv,
                    self.ids.mode_settings_selector_tab_vc_ac,
                ],

                selector_texts: [
                    self.ids.mode_settings_selector_texts_pc_cmv,
                    self.ids.mode_settings_selector_texts_pc_ac,
                    self.ids.mode_settings_selector_texts_pc_vsai,
                    self.ids.mode_settings_selector_texts_vc_cmv,
                    self.ids.mode_settings_selector_texts_vc_ac,
                ],

                group_wrapper: self.ids.mode_settings_group_wrapper,
                content_wrapper: self.ids.mode_settings_content_wrapper,
                form_wrapper: self.ids.mode_settings_form_wrapper,

                group_tab_buttons: [
                    self.ids.mode_settings_group_tab_general_button,
                    self.ids.mode_settings_group_tab_alarms_button,
                ],

                group_tab_texts: [
                    self.ids.mode_settings_group_tab_general_text,
                    self.ids.mode_settings_group_tab_alarms_text,
                ],

                field_alarm_threshold_low_inspiratory_minute_volume_ids: gen_render_mode_settings_alarm_ids!(
                    self,
                    threshold_low_inspiratory_minute_volume
                ),
                field_alarm_threshold_high_inspiratory_minute_volume_ids: gen_render_mode_settings_alarm_ids!(
                    self,
                    threshold_high_inspiratory_minute_volume
                ),
                field_alarm_threshold_low_expiratory_minute_volume_ids: gen_render_mode_settings_alarm_ids!(
                    self,
                    threshold_low_expiratory_minute_volume
                ),
                field_alarm_threshold_high_expiratory_minute_volume_ids: gen_render_mode_settings_alarm_ids!(
                    self,
                    threshold_high_expiratory_minute_volume
                ),
                field_alarm_threshold_low_respiratory_rate_ids: gen_render_mode_settings_alarm_ids!(
                    self,
                    threshold_low_respiratory_rate
                ),
                field_alarm_threshold_high_respiratory_rate_ids: gen_render_mode_settings_alarm_ids!(
                    self,
                    threshold_high_respiratory_rate
                ),
                field_alarm_threshold_low_tidal_volume_ids: gen_render_mode_settings_alarm_ids!(
                    self,
                    threshold_low_tidal_volume
                ),
                field_alarm_threshold_high_tidal_volume_ids: gen_render_mode_settings_alarm_ids!(
                    self,
                    threshold_high_tidal_volume
                ),
                field_alarm_threshold_leak_ids: gen_render_mode_settings_alarm_ids!(
                    self,
                    threshold_leak
                ),
                field_alarm_threshold_peak_pressure_ids: gen_render_mode_settings_alarm_ids!(
                    self,
                    threshold_peak_pressure
                ),

                field_pressure_inspiratory_ids: gen_render_mode_settings_field_ids!(
                    self,
                    pressure_inspiratory
                ),
                field_pressure_expiratory_ids: gen_render_mode_settings_field_ids!(
                    self,
                    pressure_expiratory
                ),
                field_time_inspiratory_minimum_ids: gen_render_mode_settings_field_ids!(
                    self,
                    time_inspiratory_minimum
                ),
                field_time_inspiratory_maximum_ids: gen_render_mode_settings_field_ids!(
                    self,
                    time_inspiratory_maximum
                ),
                field_cycles_per_minute_ids: gen_render_mode_settings_field_ids!(
                    self,
                    cycles_per_minute
                ),
                field_tidal_volume_ids: gen_render_mode_settings_field_ids!(self, tidal_volume),
                field_inspiratory_flow_ids: gen_render_mode_settings_field_ids!(
                    self,
                    inspiratory_flow
                ),
                field_inspiratory_duration_ids: gen_render_mode_settings_field_ids!(
                    self,
                    inspiratory_duration
                ),
                field_plateau_duration_ids: gen_render_mode_settings_field_ids!(
                    self,
                    plateau_duration
                ),
                field_trigger_offset_ids: gen_render_mode_settings_field_ids!(self, trigger_offset),
                field_trigger_expiratory_ids: gen_render_mode_settings_field_ids!(
                    self,
                    trigger_expiratory
                ),
            }));
    }
}
