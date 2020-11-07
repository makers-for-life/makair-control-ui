// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::borrow::Cow;

use chrono::{DateTime, NaiveDateTime, Utc};
use glium::texture;
use plotters::prelude::*;
use plotters::style::TextStyle;
use telemetry::structures::MachineStateSnapshot;

use crate::chip::{Chip, ChipDataPressure};
use crate::config::environment::*;
use crate::utilities::image::reverse_rgb_fast;
#[cfg(feature = "graph-scaler")]
use crate::utilities::pressure::process_max_allowed_pressure;

use super::renderer::DisplayRenderer;
use super::support::GliumDisplayWinitWrapper;

pub struct DisplayGraph;

lazy_static! {
    static ref GRAPH_AXIS_Y_FONT: TextStyle<'static> =
        TextStyle::from(("sans-serif", 15).into_font());
}

impl DisplayGraph {
    pub fn refresh(
        display: &GliumDisplayWinitWrapper,
        renderer: &DisplayRenderer,
        image_map: &mut conrod_core::image::Map<texture::Texture2d>,
        chip: &Chip,
    ) {
        // Re-render pressure graph
        Self::render_pressure(display, renderer, image_map, chip);
    }

    pub fn draw_pressure(
        display: &GliumDisplayWinitWrapper,
        data_pressure: &ChipDataPressure,
        machine_snapshot: Option<&MachineStateSnapshot>,
    ) -> glium::texture::Texture2d {
        let mut buffer_rgb: Vec<u8> = vec![0; (GRAPH_WIDTH * GRAPH_HEIGHT * 3) as usize];

        // Acquire values, or defaults (if not set)
        let peak_command_or_default = machine_snapshot
            .map(|snapshot| snapshot.peak_command)
            .unwrap_or(0);

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
            let _ = peak_command_or_default;

            range_high
        };

        // "Graph scaler" auto-scale mode requested, will auto-process graph maximum
        #[cfg(feature = "graph-scaler")]
        let range_high = {
            let peak_command_or_initial = if peak_command_or_default > 0 {
                peak_command_or_default
            } else {
                GRAPH_DRAW_RANGE_HIGH_DYNAMIC_INITIAL
            };

            // Convert the "range high" value from cmH20 to mmH20, as this is the high-precision \
            //   unit we work with for graphing purposes only.
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

        // Reverse the chart image in an efficient way
        let reversed_rgb = reverse_rgb_fast(&buffer_rgb, GRAPH_WIDTH, GRAPH_HEIGHT);

        // Build the final 2D texture from the raw image buffer in a 2D space
        // Notice: build the raw image directly using the texture internals, as to avoid cloning \
        //   the raw image bytes at every refresh.
        glium::texture::Texture2d::new(
            &display.0,
            glium::texture::RawImage2d {
                data: Cow::Borrowed(&reversed_rgb),
                width: GRAPH_WIDTH,
                height: GRAPH_HEIGHT,
                format: glium::texture::ClientFormat::U8U8U8,
            },
        )
        .unwrap()
    }

    fn render_pressure(
        display: &GliumDisplayWinitWrapper,
        renderer: &DisplayRenderer,
        image_map: &mut conrod_core::image::Map<texture::Texture2d>,
        chip: &Chip,
    ) {
        // Draw pressure graph
        // Notice: override last graph value in the image map
        // Important: do not insert a new graph at every pass, as this will result in a huge \
        //   memory leak, as all previous graphs will reside forever in the image map, which will \
        //   grow infinitely.
        image_map
            .replace(
                renderer.images.graph_pressure,
                Self::draw_pressure(
                    display,
                    &chip.data_pressure,
                    Some(&chip.last_machine_snapshot),
                ),
            )
            .expect("pressure graph image could not be replaced in the image map");
    }
}
