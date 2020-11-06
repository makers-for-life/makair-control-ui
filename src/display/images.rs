// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::borrow::Cow;

use chrono::{DateTime, NaiveDateTime, Utc};
use image::load_from_memory;
use plotters::prelude::*;
use plotters::style::TextStyle;
use telemetry::structures::MachineStateSnapshot;

use crate::chip::ChipDataPressure;
use crate::config::environment::*;
use crate::utilities::image::reverse_rgba;
#[cfg(feature = "graph-scaler")]
use crate::utilities::pressure::process_max_allowed_pressure;
use crate::EmbeddedImages;

use super::support::GliumDisplayWinitWrapper;

pub struct DisplayImages;

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

impl DisplayImages {
    pub fn bootloader_logo(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_BOOTLOADER_LOGO_RGBA_RAW[
                BOOTLOADER_LOGO_WIDTH, BOOTLOADER_LOGO_HEIGHT
            ]
        )
    }

    pub fn error_icon(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_ERROR_ICON_RGBA_RAW[
                ERROR_ICON_WIDTH, ERROR_ICON_HEIGHT
            ]
        )
    }

    pub fn telemetry_arrow(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_TELEMETRY_ARROW_RGBA_RAW[
                TELEMETRY_ARROW_WIDTH, TELEMETRY_ARROW_HEIGHT
            ]
        )
    }

    pub fn controls_run_icon(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_CONTROLS_RUN_RGBA_RAW[
                CONTROLS_BUTTON_ICON_WIDTH, CONTROLS_BUTTON_ICON_HEIGHT
            ]
        )
    }

    pub fn controls_snooze_inactive_icon(
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_CONTROLS_SNOOZE_INACTIVE_RGBA_RAW[
                CONTROLS_BUTTON_ICON_WIDTH, CONTROLS_BUTTON_ICON_HEIGHT
            ]
        )
    }

    pub fn controls_snooze_active_icon(
        display: &GliumDisplayWinitWrapper,
    ) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_CONTROLS_SNOOZE_ACTIVE_RGBA_RAW[
                CONTROLS_BUTTON_ICON_WIDTH, CONTROLS_BUTTON_ICON_HEIGHT
            ]
        )
    }

    pub fn controls_advanced_icon(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_CONTROLS_ADVANCED_RGBA_RAW[
                CONTROLS_BUTTON_ICON_WIDTH, CONTROLS_BUTTON_ICON_HEIGHT
            ]
        )
    }

    pub fn status_save_icon(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_STATUS_SAVE_RGBA_RAW[
                STATUS_SAVE_ICON_WIDTH, STATUS_SAVE_ICON_HEIGHT
            ]
        )
    }

    pub fn branding(display: &GliumDisplayWinitWrapper) -> glium::texture::Texture2d {
        // Create image from raw buffer (cached)
        gen_draw_cached_image!(
            display <= IMAGE_TOP_LOGO_RGBA_RAW[
                BRANDING_WIDTH, BRANDING_HEIGHT
            ]
        )
    }

    pub fn data_chart(
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
