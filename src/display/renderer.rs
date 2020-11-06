// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::borrow::Cow;
use std::time::{Duration, Instant};

use chrono::{DateTime, NaiveDateTime, Utc};
use conrod_core::Ui;
use glium::texture;
use image::load_from_memory;
use plotters::prelude::*;
use plotters::style::TextStyle;
use telemetry::structures::MachineStateSnapshot;

use crate::chip::settings::ChipSettingsEvent;
use crate::chip::{Chip, ChipDataPressure, ChipError, ChipState};
use crate::config::environment::*;
use crate::utilities::image::reverse_rgba;
use crate::utilities::parse::parse_version_number;
#[cfg(feature = "graph-scaler")]
use crate::utilities::pressure::process_max_allowed_pressure;
use crate::EmbeddedImages;
use crate::APP_ARGS;

use super::data::*;
use super::events::DisplayUIEvents;
use super::fonts::Fonts;
use super::identifiers::Ids;
use super::screen::{Screen, ScreenModalsOpen};
use super::support::GliumDisplayWinitWrapper;

const WAITING_FOR_DATA_TIMEOUT_AFTER: Duration = Duration::from_secs(10);
const DISPATCH_HEARTBEAT_EVERY: Duration = Duration::from_secs(1);

#[derive(Debug, PartialEq, Eq)]
pub enum DisplayRendererSettingsState {
    Opened,
    Closed,
}

pub struct DisplayRendererBuilder;

pub struct DisplayRenderer {
    fonts: Fonts,
    ids: Ids,
    states: DisplayRendererStates,
}

#[derive(Default)]
pub struct DisplayRendererStates {
    pub run_settings: DisplayRendererSettingsState,
    pub snooze_settings: DisplayRendererSettingsState,
    pub advanced_settings: DisplayRendererSettingsState,
    pub trigger_settings: DisplayRendererSettingsState,
    pub expiration_term_settings: DisplayRendererSettingsState,
    pub pressure_settings: DisplayRendererSettingsState,
    pub cycles_settings: DisplayRendererSettingsState,
}

lazy_static! {
    static ref IMAGE_TOP_LOGO_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("top-logo", BRANDING_WIDTH);
    static ref IMAGE_BOOTLOADER_LOGO_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("bootloader-logo", BOOTLOADER_LOGO_WIDTH);
    static ref IMAGE_ERROR_ICON_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("error-icon", ERROR_ICON_WIDTH);
    static ref IMAGE_TELEMETRY_ARROW_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("telemetry-arrow", TELEMETRY_ARROW_WIDTH);
    static ref IMAGE_CONTROLS_RUN_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("controls-run", CONTROLS_BUTTON_ICON_WIDTH);
    static ref IMAGE_CONTROLS_SNOOZE_INACTIVE_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("controls-snooze-inactive", CONTROLS_BUTTON_ICON_WIDTH);
    static ref IMAGE_CONTROLS_SNOOZE_ACTIVE_RGBA_RAW: Vec<u8> =
        gen_load_image_reverse!("controls-snooze-active", CONTROLS_BUTTON_ICON_WIDTH);
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

    pub fn is_open(&self) -> bool {
        self == &Self::Opened
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
            states: DisplayRendererStates::default(),
        }
    }
}

impl DisplayRenderer {
    pub fn render(
        &mut self,
        display: &GliumDisplayWinitWrapper,
        interface: &mut Ui,
        chip: &Chip,
    ) -> conrod_core::image::Map<texture::Texture2d> {
        let mut image_map = conrod_core::image::Map::<texture::Texture2d>::new();

        match &chip.state {
            // Waiting for data from the motherboard, treat it as a 'connecting...' state
            ChipState::WaitingData(started_time) => {
                // The UI has been waiting for data for too long? Show an error instead, though \
                //   we are still waiting for data, so this may fix by itself. This is done for UI \
                //   purposes, though the chip state is still 'ChipState::WaitingData'.
                if started_time.elapsed() >= WAITING_FOR_DATA_TIMEOUT_AFTER {
                    self.error(display, interface, &mut image_map, &ChipError::TimedOut)
                } else {
                    self.initializing(display, interface, &mut image_map, true)
                }
            }
            // Initializing, treat it as a 'connected' state
            ChipState::Initializing => self.initializing(display, interface, &mut image_map, false),
            // Running or stopped, handle data
            ChipState::Running | ChipState::Stopped => {
                self.data(display, interface, &mut image_map, chip)
            }
            // An error occured
            ChipState::Error(err) => self.error(display, interface, &mut image_map, err),
        };

        image_map
    }

    pub fn run_events(
        &mut self,
        interface: &mut Ui,
        last_heartbeat: &Instant,
        tick_time: &Instant,
    ) -> (bool, bool, Vec<ChipSettingsEvent>) {
        // Run all UI events (defer to sub-handler)
        let (has_user_events, user_events) =
            DisplayUIEvents::run(interface, &self.ids, &mut self.states);

        // Check if should run heartbeat? (ie. if it should be sent to the firmware)
        let mut has_heartbeat = false;

        if tick_time.duration_since(*last_heartbeat) >= DISPATCH_HEARTBEAT_EVERY {
            has_heartbeat = true;
        }

        (has_heartbeat, has_user_events, user_events)
    }

    pub fn has_state_moderate_framerate(&self) -> bool {
        // Returns whether a current state value should result in a moderate framerate, so that \
        //   resource usage is at a minimum.

        // The advanced settings modal is quite heavy to re-render every time, and we do not need \
        //   the values to be shown at full framerate. Plus values keep changing every millisecond \
        //   or so. Throttle down FPS.
        if self.states.advanced_settings.is_open() {
            return true;
        }

        false
    }

    fn initializing(
        &mut self,
        display: &GliumDisplayWinitWrapper,
        interface: &mut Ui,
        image_map: &mut conrod_core::image::Map<texture::Texture2d>,
        is_connecting: bool,
    ) {
        // Draw bootloader logo
        let bootloader_logo_image_texture = self.draw_bootloader_logo(display);

        let (bootloader_logo_width, bootloader_logo_height) = (
            bootloader_logo_image_texture.get_width(),
            bootloader_logo_image_texture.get_height().unwrap(),
        );

        let image_id = image_map.insert(bootloader_logo_image_texture);

        // Create initializing screen
        let screen_bootloader = DisplayDataBootloader {
            image_id,
            width: bootloader_logo_width as _,
            height: bootloader_logo_height as _,
            connecting: is_connecting,
        };

        let mut screen = Screen::new(
            interface.set_widgets(),
            &self.ids,
            &self.fonts,
            None,
            None,
            None,
            None,
        );

        screen.render_initializing(screen_bootloader);
    }

    fn error(
        &mut self,
        display: &GliumDisplayWinitWrapper,
        interface: &mut Ui,
        image_map: &mut conrod_core::image::Map<texture::Texture2d>,
        error: &ChipError,
    ) {
        // Draw error icon
        let error_icon_image_texture = self.draw_error_icon(display);

        let (error_icon_width, error_icon_height) = (
            error_icon_image_texture.get_width(),
            error_icon_image_texture.get_height().unwrap(),
        );

        let image_id = image_map.insert(error_icon_image_texture);

        // Create error screen
        let screen_error = DisplayDataError {
            image_id,
            width: error_icon_width as _,
            height: error_icon_height as _,
            error,
        };

        let mut screen = Screen::new(
            interface.set_widgets(),
            &self.ids,
            &self.fonts,
            None,
            None,
            None,
            None,
        );

        screen.render_error(screen_error);
    }

    fn data(
        &mut self,
        display: &GliumDisplayWinitWrapper,
        interface: &mut Ui,
        image_map: &mut conrod_core::image::Map<texture::Texture2d>,
        chip: &Chip,
    ) {
        // Create branding
        let branding_image_texture = self.draw_branding(display);

        let (branding_width, branding_height) = (
            branding_image_texture.get_width(),
            branding_image_texture.get_height().unwrap(),
        );

        let branding_image_id = image_map.insert(branding_image_texture);

        // Create graph
        let graph_image_texture =
            self.draw_data_chart(&chip.data_pressure, &chip.last_machine_snapshot, display);

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

        let ongoing_alarms = chip.ongoing_alarms_sorted();

        let (ongoing_alarms_count, widgets_alarms_count) =
            (ongoing_alarms.len(), self.ids.alarm_alarms.len());

        if ongoing_alarms_count > widgets_alarms_count {
            for i in widgets_alarms_count..ongoing_alarms_count {
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
            let (alarms_difference, alarm_id) = (
                widgets_alarms_count - ongoing_alarms_count,
                &mut ui.widget_id_generator(),
            );

            if alarms_difference > 0 {
                self.ids.alarm_alarms.resize(ongoing_alarms_count, alarm_id);
                self.ids
                    .alarm_codes_containers
                    .resize(ongoing_alarms_count, alarm_id);
                self.ids.alarm_codes.resize(ongoing_alarms_count, alarm_id);
                self.ids
                    .alarm_codes_containers
                    .resize(ongoing_alarms_count, alarm_id);
                self.ids
                    .alarm_messages
                    .resize(ongoing_alarms_count, alarm_id);
            }
        }

        // Create screen & its screen data
        let mut screen = Screen::new(
            ui,
            &self.ids,
            &self.fonts,
            Some(&ongoing_alarms),
            Some(chip.last_tick),
            Some(&chip.last_machine_snapshot),
            chip.last_data_snapshot.as_ref(),
        );

        let screen_data_branding = DisplayDataBranding {
            firmware_version: parse_version_number(
                if chip.last_machine_snapshot.version.is_empty() {
                    BRANDING_TEXT_VERSION_NONE
                } else {
                    &chip.last_machine_snapshot.version
                },
            ),
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
            run_image_id: image_map
                .insert(self.draw_controls_icon(display, &*IMAGE_CONTROLS_RUN_RGBA_RAW)),
            snooze_inactive_image_id: image_map.insert(
                self.draw_controls_icon(display, &*IMAGE_CONTROLS_SNOOZE_INACTIVE_RGBA_RAW),
            ),
            snooze_active_image_id: image_map
                .insert(self.draw_controls_icon(display, &*IMAGE_CONTROLS_SNOOZE_ACTIVE_RGBA_RAW)),
            advanced_image_id: image_map
                .insert(self.draw_controls_icon(display, &*IMAGE_CONTROLS_ADVANCED_RGBA_RAW)),
            chip_state: &chip.state,
            chip_settings: &chip.settings,
        };

        let screen_data_status = DisplayDataStatus {
            chip_state: &chip.state,
            battery_level: chip
                .last_data_snapshot
                .as_ref()
                .map(|data| data.battery_level),
            save_image_id,
        };
        let screen_data_heartbeat = DisplayDataHeartbeat {
            data_pressure: &chip.data_pressure,
        };

        let screen_data_graph = DisplayDataGraph {
            image_id: graph_image_id,
            width: graph_width as _,
            height: graph_height as _,
        };

        let screen_data_telemetry = DisplayDataTelemetry {
            arrow_image_id: telemetry_arrow_image_id,
        };

        // Render screen data (depending on state, running or stopped)
        match chip.state {
            ChipState::Running => screen.render_running(
                screen_data_branding,
                screen_data_controls,
                screen_data_status,
                screen_data_heartbeat,
                screen_data_graph,
                screen_data_telemetry,
                &chip.settings,
                &ScreenModalsOpen::from_states(&self.states),
            ),

            ChipState::Stopped => screen.render_stop(
                screen_data_branding,
                screen_data_controls,
                screen_data_status,
                screen_data_heartbeat,
                screen_data_graph,
                screen_data_telemetry,
                &chip.settings,
                &ScreenModalsOpen::from_states(&self.states),
            ),

            _ => unreachable!(),
        };
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

    fn draw_error_icon(&self, display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_ERROR_ICON_RGBA_RAW[ERROR_ICON_WIDTH, ERROR_ICON_HEIGHT]
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

    fn draw_controls_icon(
        &self,
        display: &GliumDisplayWinitWrapper,
        icon_rgba_raw: &[u8],
    ) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= icon_rgba_raw[CONTROLS_BUTTON_ICON_WIDTH, CONTROLS_BUTTON_ICON_HEIGHT]
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
        data_pressure: &ChipDataPressure,
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
