// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::borrow::Cow;

use chrono::{DateTime, NaiveDateTime, Utc};
use conrod_core::Ui;
use glium::texture;
use image::load_from_memory;
use plotters::prelude::*;
use plotters::style::TextStyle;
use telemetry::alarm::AlarmCode;
use telemetry::structures::{AlarmPriority, MachineStateSnapshot};

use crate::chip::settings::{ChipSettings, ChipSettingsEvent};
use crate::chip::ChipState;
use crate::config::environment::*;
use crate::utilities::image::reverse_rgba;
use crate::utilities::parse::parse_version_number;
#[cfg(feature = "graph-scaler")]
use crate::utilities::pressure::process_max_allowed_pressure;
use crate::utilities::types::DataPressure;
use crate::EmbeddedImages;
use crate::APP_ARGS;

use super::data::*;
use super::events::DisplayUIEvents;
use super::fonts::Fonts;
use super::identifiers::Ids;
use super::screen::{Screen, ScreenModalsOpen};
use super::support::GliumDisplayWinitWrapper;

#[derive(Debug, PartialEq, Eq)]
pub enum DisplayRendererSettingsState {
    Opened,
    Closed,
}

pub struct DisplayRendererBuilder;

pub struct DisplayRenderer {
    fonts: Fonts,
    ids: Ids,
    run_settings_state: DisplayRendererSettingsState,
    advanced_settings_state: DisplayRendererSettingsState,
    trigger_settings_state: DisplayRendererSettingsState,
    expiration_term_settings_state: DisplayRendererSettingsState,
    pressure_settings_state: DisplayRendererSettingsState,
    cycles_settings_state: DisplayRendererSettingsState,
}

lazy_static! {
    static ref IMAGE_TOP_LOGO_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("top-logo", BRANDING_WIDTH);
    static ref IMAGE_BOOTLOADER_LOGO_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("bootloader-logo", BOOTLOADER_LOGO_WIDTH);
    static ref IMAGE_TELEMETRY_ARROW_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("telemetry-arrow", TELEMETRY_ARROW_WIDTH);
    static ref IMAGE_CONTROLS_RUN_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("controls-run", CONTROLS_BUTTON_ICON_WIDTH);
    static ref IMAGE_CONTROLS_ADVANCED_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("controls-advanced", CONTROLS_BUTTON_ICON_WIDTH);
    static ref IMAGE_STATUS_SAVE_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("save", STATUS_SAVE_ICON_WIDTH);
    static ref GRAPH_AXIS_Y_FONT: TextStyle<'static> =
        TextStyle::from(("sans-serif", 15).into_font());
}

impl DisplayRendererSettingsState {
    pub fn toggle(&mut self) {
        *self = match self {
            Self::Closed => Self::Opened,
            Self::Opened => Self::Closed,
        };
    }
}

impl Default for DisplayRendererSettingsState {
    fn default() -> Self {
        Self::Closed
    }
}

#[allow(clippy::new_ret_no_self)]
impl DisplayRendererBuilder {
    pub fn new(fonts: Fonts, ids: Ids) -> DisplayRenderer {
        DisplayRenderer {
            fonts,
            ids,
            run_settings_state: DisplayRendererSettingsState::default(),
            advanced_settings_state: DisplayRendererSettingsState::default(),
            trigger_settings_state: DisplayRendererSettingsState::default(),
            expiration_term_settings_state: DisplayRendererSettingsState::default(),
            pressure_settings_state: DisplayRendererSettingsState::default(),
            cycles_settings_state: DisplayRendererSettingsState::default(),
        }
    }
}

impl DisplayRenderer {
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &mut self,
        data_pressure: &DataPressure,
        machine_snapshot: &MachineStateSnapshot,
        ongoing_alarms: &[(AlarmCode, AlarmPriority)],
        display: &GliumDisplayWinitWrapper,
        interface: &mut Ui,
        battery_level: Option<u8>,
        chip_state: &ChipState,
        chip_settings: &ChipSettings,
    ) -> conrod_core::image::Map<texture::Texture2d> {
        let image_map = conrod_core::image::Map::<texture::Texture2d>::new();

        match chip_state {
            // Waiting for data from the motherboard, treat it as a 'connecting...' state
            ChipState::WaitingData => self.initializing(display, interface, image_map, true),
            // Initializing, treat it as a 'connected' state
            ChipState::Initializing => self.initializing(display, interface, image_map, false),
            // Running or stopped, handle data
            ChipState::Running | ChipState::Stopped => self.data(
                display,
                interface,
                image_map,
                data_pressure,
                machine_snapshot,
                ongoing_alarms,
                battery_level,
                chip_state,
                chip_settings,
            ),
            // An error occured
            ChipState::Error(err) => self.error(interface, image_map, err.clone()),
        }
    }

    pub fn run_events(&mut self, interface: &mut Ui) -> (bool, Vec<ChipSettingsEvent>) {
        // Run all UI events (defer to sub-handler)
        DisplayUIEvents::run(
            interface,
            &self.ids,
            &mut self.run_settings_state,
            &mut self.advanced_settings_state,
            &mut self.trigger_settings_state,
            &mut self.expiration_term_settings_state,
            &mut self.pressure_settings_state,
            &mut self.cycles_settings_state,
        )
    }

    fn initializing(
        &mut self,
        display: &GliumDisplayWinitWrapper,
        interface: &mut Ui,
        mut image_map: conrod_core::image::Map<texture::Texture2d>,
        is_connecting: bool,
    ) -> conrod_core::image::Map<texture::Texture2d> {
        let bootloader_logo_image_texture = self.draw_bootloader_logo(display);

        let (bootloader_logo_width, bootloader_logo_height) = (
            bootloader_logo_image_texture.get_width(),
            bootloader_logo_image_texture.get_height().unwrap(),
        );

        let image_id = image_map.insert(bootloader_logo_image_texture);

        let ui = interface.set_widgets();

        let screen_bootloader = DisplayDataBootloader {
            image_id,
            width: bootloader_logo_width as _,
            height: bootloader_logo_height as _,
            connecting: is_connecting,
        };

        let mut screen = Screen::new(ui, &self.ids, &self.fonts, None, None);

        screen.render_initializing(screen_bootloader);

        image_map
    }

    fn error(
        &mut self,
        interface: &mut Ui,
        image_map: conrod_core::image::Map<texture::Texture2d>,
        error: String,
    ) -> conrod_core::image::Map<texture::Texture2d> {
        let ui = interface.set_widgets();

        let mut screen = Screen::new(ui, &self.ids, &self.fonts, None, None);

        screen.render_error(error);

        image_map
    }

    #[allow(clippy::too_many_arguments)]
    fn data(
        &mut self,
        display: &GliumDisplayWinitWrapper,
        interface: &mut Ui,
        mut image_map: conrod_core::image::Map<texture::Texture2d>,
        data_pressure: &DataPressure,
        machine_snapshot: &MachineStateSnapshot,
        ongoing_alarms: &[(AlarmCode, AlarmPriority)],
        battery_level: Option<u8>,
        chip_state: &ChipState,
        chip_settings: &ChipSettings,
    ) -> conrod_core::image::Map<texture::Texture2d> {
        // Create branding
        let branding_image_texture = self.draw_branding(display);

        let (branding_width, branding_height) = (
            branding_image_texture.get_width(),
            branding_image_texture.get_height().unwrap(),
        );

        let branding_image_id = image_map.insert(branding_image_texture);

        // Create graph
        let graph_image_texture = self.draw_data_chart(data_pressure, machine_snapshot, display);

        let (graph_width, graph_height) = (
            graph_image_texture.get_width(),
            graph_image_texture.get_height().unwrap(),
        );

        let graph_image_id = image_map.insert(graph_image_texture);

        // Create telemetry
        let telemetry_arrow_image_texture = self.draw_telemetry_arrow(display);
        let telemetry_arrow_image_id = image_map.insert(telemetry_arrow_image_texture);

        // Create widgets
        let mut ui = interface.set_widgets();

        let ongoing_alarms_len = ongoing_alarms.len();
        let widgets_alarms_len = self.ids.alarm_alarms.len();

        if ongoing_alarms_len > widgets_alarms_len {
            for i in widgets_alarms_len..ongoing_alarms_len {
                let index = i + 1;
                self.ids
                    .alarm_alarms
                    .resize(index, &mut ui.widget_id_generator());
                self.ids
                    .alarm_codes_containers
                    .resize(index, &mut ui.widget_id_generator());
                self.ids
                    .alarm_codes
                    .resize(index, &mut ui.widget_id_generator());
                self.ids
                    .alarm_messages_containers
                    .resize(index, &mut ui.widget_id_generator());
                self.ids
                    .alarm_messages
                    .resize(index, &mut ui.widget_id_generator());
            }
        } else {
            let diff = widgets_alarms_len - ongoing_alarms_len;
            let useless_id = &mut ui.widget_id_generator();

            if diff > 0 {
                self.ids.alarm_alarms.resize(ongoing_alarms_len, useless_id);
                self.ids
                    .alarm_codes_containers
                    .resize(ongoing_alarms_len, useless_id);
                self.ids.alarm_codes.resize(ongoing_alarms_len, useless_id);
                self.ids
                    .alarm_codes_containers
                    .resize(ongoing_alarms_len, useless_id);
                self.ids
                    .alarm_messages
                    .resize(ongoing_alarms_len, useless_id);
            }
        }

        let mut screen = Screen::new(
            ui,
            &self.ids,
            &self.fonts,
            Some(machine_snapshot),
            Some(ongoing_alarms),
        );

        let screen_data_branding = DisplayDataBranding {
            firmware_version: parse_version_number(if machine_snapshot.version.is_empty() {
                BRANDING_TEXT_VERSION_NONE
            } else {
                &machine_snapshot.version
            }),
            image_id: branding_image_id,
            width: branding_width as _,
            height: branding_height as _,
        };

        let save_image_id = if APP_ARGS.is_recording() {
            let save_icon_texture = self.draw_status_save_icon(display);

            Some(image_map.insert(save_icon_texture))
        } else {
            None
        };

        let screen_data_controls = DisplayDataControls {
            run_image_id: image_map.insert(self.draw_controls_run_icon(display)),
            advanced_image_id: image_map.insert(self.draw_controls_advanced_icon(display)),
            chip_state,
        };

        let screen_data_status = DisplayDataStatus {
            chip_state,
            battery_level,
            save_image_id,
        };
        let screen_data_heartbeat = DisplayDataHeartbeat { data_pressure };

        let screen_data_graph = DisplayDataGraph {
            image_id: graph_image_id,
            width: graph_width as _,
            height: graph_height as _,
        };

        let screen_data_telemetry = DisplayDataTelemetry {
            arrow_image_id: telemetry_arrow_image_id,
        };

        match chip_state {
            ChipState::Running => screen.render_with_data(
                screen_data_branding,
                screen_data_controls,
                screen_data_status,
                screen_data_heartbeat,
                screen_data_graph,
                screen_data_telemetry,
                chip_settings,
                &ScreenModalsOpen::from_states(
                    &self.run_settings_state,
                    &self.advanced_settings_state,
                    &self.trigger_settings_state,
                    &self.expiration_term_settings_state,
                    &self.pressure_settings_state,
                    &self.cycles_settings_state,
                ),
            ),

            ChipState::Stopped => screen.render_stop(
                screen_data_branding,
                screen_data_controls,
                screen_data_status,
                screen_data_heartbeat,
                screen_data_graph,
                screen_data_telemetry,
                chip_settings,
                &ScreenModalsOpen::from_states(
                    &self.run_settings_state,
                    &self.advanced_settings_state,
                    &self.trigger_settings_state,
                    &self.expiration_term_settings_state,
                    &self.pressure_settings_state,
                    &self.cycles_settings_state,
                ),
            ),

            _ => unreachable!(),
        };

        image_map
    }

    fn draw_bootloader_logo(
        &self,
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_BOOTLOADER_LOGO_RGBA_RAW[BOOTLOADER_LOGO_WIDTH, BOOTLOADER_LOGO_HEIGHT]
        )
    }

    fn draw_telemetry_arrow(
        &self,
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_TELEMETRY_ARROW_RGBA_RAW[TELEMETRY_ARROW_WIDTH, TELEMETRY_ARROW_HEIGHT]
        )
    }

    fn draw_controls_run_icon(
        &self,
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_CONTROLS_RUN_RGBA_RAW[CONTROLS_BUTTON_ICON_WIDTH, CONTROLS_BUTTON_ICON_HEIGHT]
        )
    }

    fn draw_controls_advanced_icon(
        &self,
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_CONTROLS_ADVANCED_RGBA_RAW[CONTROLS_BUTTON_ICON_WIDTH, CONTROLS_BUTTON_ICON_HEIGHT]
        )
    }

    fn draw_status_save_icon(
        &self,
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_STATUS_SAVE_RGBA_RAW[STATUS_SAVE_ICON_WIDTH, STATUS_SAVE_ICON_HEIGHT]
        )
    }

    fn draw_branding(&self, display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_TOP_LOGO_RGBA_RAW[BRANDING_WIDTH, BRANDING_HEIGHT]
        )
    }

    fn draw_data_chart(
        &self,
        data_pressure: &DataPressure,
        machine_snapshot: &MachineStateSnapshot,
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::Texture2d {
        let mut buffer_rgb: Vec<u8> = vec![0; (GRAPH_WIDTH * GRAPH_HEIGHT * 3) as usize];

        // Docs: https://docs.rs/plotters/0.2.12/plotters/drawing/struct.BitMapBackend.html
        let drawing = BitMapBackend::with_buffer(&mut buffer_rgb, (GRAPH_WIDTH, GRAPH_HEIGHT))
            .into_drawing_area();

        // Acquire time range
        let newest_time = data_pressure
            .front()
            .unwrap_or(&(
                DateTime::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
                0,
            ))
            .0;
        let oldest_time = newest_time - chrono::Duration::seconds(GRAPH_DRAW_SECONDS as _);

        // Docs: https://docs.rs/plotters/0.2.12/plotters/chart/struct.ChartBuilder.html

        // "Default" static graph maximum mode requested
        // Convert the "range high" value from cmH20 to mmH20, as this is the high-precision unit \
        //   we work with for graphing purposes only.
        #[cfg(not(feature = "graph-scaler"))]
        let range_high = {
            let range_high = (GRAPH_DRAW_RANGE_HIGH_STATIC_INITIAL as i32)
                * (TELEMETRY_POINTS_PRECISION_DIVIDE as i32);

            // Void statement to prevent the compiler from warning about unused \
            //   'machine_snapshot', which is indeed used under feature 'graph-scaler'.
            let _ = machine_snapshot.peak_command;

            range_high
        };

        // "Graph scaler" auto-scale mode requested, will auto-process graph maximum
        #[cfg(feature = "graph-scaler")]
        let range_high = {
            let peak_command_or_initial = if machine_snapshot.peak_command > 0 {
                machine_snapshot.peak_command
            } else {
                GRAPH_DRAW_RANGE_HIGH_DYNAMIC_INITIAL
            };

            // Convert the "range high" value from cmH20 to mmH20, as this is the high-precision unit \
            //   we work with for graphing purposes only.
            let mut range_high = (process_max_allowed_pressure(peak_command_or_initial) as u16
                * TELEMETRY_POINTS_PRECISION_DIVIDE) as i32;

            // Override "range high" with a larger value contained in graph (avoids \
            //   larger-than-range-high graph points to flat out)
            let graph_largest_point = {
                let mut data_pressure_points_ordered = data_pressure
                    .iter()
                    .map(|x| x.1 as i32)
                    .collect::<Vec<i32>>();

                data_pressure_points_ordered.sort_unstable();

                *data_pressure_points_ordered.last().unwrap_or(&0)
            };

            if graph_largest_point > range_high {
                range_high = graph_largest_point;
            }

            range_high
        };

        let mut chart = ChartBuilder::on(&drawing)
            .margin_top(GRAPH_DRAW_MARGIN_TOP)
            .margin_bottom(GRAPH_DRAW_MARGIN_BOTTOM)
            .margin_left(GRAPH_DRAW_MARGIN_LEFT)
            .margin_right(GRAPH_DRAW_MARGIN_RIGHT)
            .x_label_area_size(0)
            .y_label_area_size(GRAPH_DRAW_LABEL_WIDTH)
            .build_cartesian_2d(oldest_time..newest_time, GRAPH_DRAW_RANGE_LOW..range_high)
            .expect("failed to build chart");

        chart
            .configure_mesh()
            .bold_line_style(&plotters::style::colors::WHITE.mix(0.04))
            .light_line_style(&plotters::style::colors::BLACK)
            .y_labels(GRAPH_DRAW_LABEL_NUMBER_MAX)
            .y_label_style(GRAPH_AXIS_Y_FONT.color(&WHITE.mix(0.65)))
            .y_label_formatter(&|y| {
                // Convert high-precision point in mmH20 back to cmH20 (which measurements & \
                //   targets both use)
                (y / TELEMETRY_POINTS_PRECISION_DIVIDE as i32).to_string()
            })
            .draw()
            .expect("failed to draw chart mesh");

        // Docs: https://docs.rs/plotters/0.2.12/plotters/prelude/struct.LineSeries.html
        chart
            .draw_series(
                LineSeries::new(
                    data_pressure.iter().map(|x| (x.0, x.1 as i32)),
                    ShapeStyle::from(&plotters::style::RGBColor(0, 137, 255))
                        .filled()
                        .stroke_width(GRAPH_DRAW_LINE_SIZE),
                )
                .point_size(GRAPH_DRAW_POINT_SIZE),
            )
            .expect("failed to draw chart data");

        drop(chart);
        drop(drawing);

        // Convert chart from an RGB to an RGBA image buffer
        let (width_value, height_value) = (GRAPH_WIDTH as usize, GRAPH_HEIGHT as usize);

        let mut buffer_rgba: Vec<u8> = vec![0; width_value * height_value * 4];

        for row in 0..(height_value - 1) {
            let (row_start_rgb, row_start_rgba) =
                (row * width_value, (height_value - row - 1) * width_value);

            for column in 0..(width_value - 1) {
                let (rgb_index, rgba_index) =
                    ((row_start_rgb + column) * 3, (row_start_rgba + column) * 4);

                buffer_rgba[rgba_index] = buffer_rgb[rgb_index];
                buffer_rgba[rgba_index + 1] = buffer_rgb[rgb_index + 1];
                buffer_rgba[rgba_index + 2] = buffer_rgb[rgb_index + 2];
                buffer_rgba[rgba_index + 3] = 255;
            }
        }

        // Instantiate a raw image in a 2D space
        let raw_image =
            glium::texture::RawImage2d::from_raw_rgba(buffer_rgba, (GRAPH_WIDTH, GRAPH_HEIGHT));

        // Build the final 2D texture from the raw image buffer in a 2D space
        glium::texture::Texture2d::with_mipmaps(
            &display.0,
            raw_image,
            glium::texture::MipmapsOption::NoMipmap,
        )
        .unwrap()
    }
}
