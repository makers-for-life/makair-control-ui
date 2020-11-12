// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use chrono::{DateTime, NaiveDateTime, Utc};
use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};
use plotters::prelude::*;
use plotters::style::{Color, TextStyle};
use plotters_conrod::{ConrodBackend, ConrodBackendReusableGraph};
use telemetry::structures::MachineStateSnapshot;

use crate::chip::ChipDataPressure;
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
#[cfg(feature = "graph-scaler")]
use crate::utilities::pressure::process_max_allowed_pressure;

const GRAPH_LINE_COLOR: RGBColor = plotters::style::RGBColor(0, 196, 255);

pub struct Config<'a> {
    pub width: f64,
    pub height: f64,

    pub parent: WidgetId,
    pub id: WidgetId,

    pub data_pressure: &'a ChipDataPressure,
    pub machine_snapshot: &'a MachineStateSnapshot,

    pub plot_graph: &'a mut ConrodBackendReusableGraph,
}

lazy_static! {
    static ref GRAPH_AXIS_Y_FONT: TextStyle<'static> =
        TextStyle::from(("sans-serif", 15).into_font());
}

pub fn render<'a>(master: &mut ControlWidget<'a>, mut config: Config<'a>) -> f64 {
    // Create container
    gen_widget_container!(
        master,
        container_id: config.id,
        color: color::TRANSPARENT,
        width: config.width as _,
        height: config.height as _,
        positions: top_left_of[
            config.parent,
        ]
    );

    // Acquire values
    let peak_command_value = config.machine_snapshot.peak_command;

    // Create drawing
    let drawing = ConrodBackend::new(
        &mut master.ui,
        (config.width as u32, config.height as u32),
        config.id,
        master.fonts.regular,
        &mut config.plot_graph,
    )
    .into_drawing_area();

    // Acquire time range
    let newest_time = config
        .data_pressure
        .front()
        .unwrap_or(&(
            DateTime::from_utc(NaiveDateTime::from_timestamp(0, 0), Utc),
            0,
        ))
        .0;
    let oldest_time = newest_time - chrono::Duration::seconds(GRAPH_DRAW_SECONDS as _);

    // "Default" static graph maximum mode requested
    // Convert the "range high" value from cmH20 to mmH20, as this is the high-precision unit \
    //   we work with for graphing purposes only.
    #[cfg(not(feature = "graph-scaler"))]
    let range_high = {
        let range_high = (GRAPH_DRAW_RANGE_HIGH_STATIC_INITIAL as i32)
            * (TELEMETRY_POINTS_PRECISION_DIVIDE as i32);

        // Void statement to prevent the compiler from warning about unused \
        //   'machine_snapshot', which is indeed used under feature 'graph-scaler'.
        let _ = peak_command_value;

        range_high
    };

    // "Graph scaler" auto-scale mode requested, will auto-process graph maximum
    #[cfg(feature = "graph-scaler")]
    let range_high = {
        let peak_command_or_initial = if peak_command_value > 0 {
            peak_command_value
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
            let mut data_pressure_points_ordered = config
                .data_pressure
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
        .bold_line_style(&plotters::style::colors::WHITE.mix(0.22))
        .light_line_style(&plotters::style::colors::BLACK)
        .y_labels(GRAPH_DRAW_LABEL_NUMBER_MAX)
        .y_label_style(GRAPH_AXIS_Y_FONT.color(&WHITE.mix(0.75)))
        .y_label_formatter(&|y| {
            // Convert high-precision point in mmH20 back to cmH20 (which measurements & \
            //   targets both use)
            (y / TELEMETRY_POINTS_PRECISION_DIVIDE as i32).to_string()
        })
        .draw()
        .expect("failed to draw chart mesh");

    chart
        .draw_series(
            LineSeries::new(
                config.data_pressure.iter().map(|x| (x.0, x.1 as i32)),
                ShapeStyle::from(&GRAPH_LINE_COLOR)
                    .filled()
                    .stroke_width(GRAPH_DRAW_LINE_SIZE),
            )
            .point_size(0),
        )
        .expect("failed to draw chart data");

    config.width
}
