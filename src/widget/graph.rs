// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::ops::Range;

use chrono::{DateTime, Utc};
use conrod_core::{
    color,
    widget::{self, Id as WidgetId},
    Positionable, Sizeable, Widget,
};
use plotters::prelude::*;
use plotters::style::{Color, TextStyle};
use plotters_conrod::{ConrodBackend, ConrodBackendReusableGraph};
use telemetry::structures::MachineStateSnapshot;

use crate::chip::{ChipDataFlow, ChipDataPressure};
use crate::config::environment::*;
use crate::display::widget::ControlWidget;
#[cfg(feature = "graph-scaler")]
use crate::utilities::pressure::process_max_allowed_pressure;

const GRAPH_PRESSURE_LINE_COLOR: RGBColor = plotters::style::RGBColor(0, 196, 255);
const GRAPH_FLOW_LINE_COLOR: RGBColor = plotters::style::RGBColor(196, 37, 20);

const GRAPH_MESH_BOLD_COLOR_RGB: RGBColor = plotters::style::RGBColor(255, 255, 255);
const GRAPH_MESH_BOLD_COLOR_ALPHA: f64 = 0.22;

const GRAPH_MESH_LIGHT_COLOR_RGB: RGBColor = plotters::style::RGBColor(0, 0, 0);

const GRAPH_AXIS_Y_FONT_COLOR_RGB: RGBColor = plotters::style::RGBColor(255, 255, 255);
const GRAPH_AXIS_Y_FONT_COLOR_ALPHA: f64 = 0.75;

pub struct Config<'a> {
    pub width: f64,
    pub height: f64,

    pub parent: WidgetId,

    pub wrapper_id: WidgetId,
    pub pressure_id: WidgetId,
    pub flow_id: WidgetId,

    pub boot_time: Option<DateTime<Utc>>,
    pub last_tick: Option<u64>,

    pub data_pressure: &'a ChipDataPressure,
    pub data_flow: &'a ChipDataFlow,

    pub machine_snapshot: &'a MachineStateSnapshot,

    pub plot_graphs: &'a mut (ConrodBackendReusableGraph, ConrodBackendReusableGraph),
}

lazy_static! {
    static ref GRAPH_AXIS_Y_FONT: TextStyle<'static> =
        TextStyle::from(("sans-serif", 14).into_font());
}

pub fn render<'a>(master: &mut ControlWidget<'a>, mut config: Config<'a>) -> f64 {
    // Create wrapper
    gen_widget_container!(
        master,
        container_id: config.wrapper_id,
        color: color::TRANSPARENT,
        width: config.width as _,
        height: config.height as _,
        positions: top_left_of[
            config.parent,
        ]
    );

    // Acquire common graph size
    let size = (config.width, config.height / 2.0);

    // Acquire common graph time range
    let newest_time = if let Some(boot_time) = config.boot_time {
        boot_time + chrono::Duration::microseconds(config.last_tick.unwrap_or(0) as i64)
    } else {
        Utc::now()
    };
    let oldest_time = newest_time - chrono::Duration::seconds(GRAPH_DRAW_SECONDS);

    // Draw plots
    pressure(master, &mut config, size, oldest_time..newest_time);
    flow(master, &mut config, size, oldest_time..newest_time);

    config.width
}

fn pressure<'a>(
    master: &mut ControlWidget<'a>,
    config: &mut Config<'a>,
    size: (f64, f64),
    time_range: Range<DateTime<Utc>>,
) {
    // Create container
    gen_widget_container!(
        master,
        container_id: config.pressure_id,
        color: color::TRANSPARENT,
        width: size.0,
        height: size.1,
        positions: top_left_of[
            config.wrapper_id,
        ]
    );

    // Create drawing
    let drawing = ConrodBackend::new(
        &mut master.ui,
        (size.0 as u32, size.1 as u32),
        config.pressure_id,
        master.fonts.regular,
        &mut config.plot_graphs.0,
    )
    .into_drawing_area();

    // "Default" static graph maximum mode requested
    // Convert the "range high" value from cmH20 to mmH20, as this is the high-precision unit \
    //   we work with for graphing purposes only.
    #[cfg(not(feature = "graph-scaler"))]
    let range_high = GRAPH_DRAW_RANGE_PRESSURE_HIGH_PRECISION_DIVIDED;

    // "Graph scaler" auto-scale mode requested, will auto-process graph maximum
    #[cfg(feature = "graph-scaler")]
    let range_high = {
        let peak_command_or_initial = if config.machine_snapshot.peak_command > 0 {
            config.machine_snapshot.peak_command
        } else {
            GRAPH_DRAW_RANGE_PRESSURE_HIGH_DYNAMIC_INITIAL
        };

        // Convert the "range high" value from cmH20 to mmH20, as this is the high-precision \
        //   unit we work with for graphing purposes only.
        let mut range_high = (process_max_allowed_pressure(peak_command_or_initial) as u16
            * TELEMETRY_POINTS_PRESSURE_PRECISION_DIVIDE) as i32;

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
        .build_cartesian_2d(
            time_range,
            GRAPH_DRAW_PRESSURE_RANGE_LOW_PRECISION_DIVIDED..range_high,
        )
        .expect("failed to build pressure chart");

    chart
        .configure_mesh()
        .bold_line_style(&GRAPH_MESH_BOLD_COLOR_RGB.mix(GRAPH_MESH_BOLD_COLOR_ALPHA))
        .light_line_style(&GRAPH_MESH_LIGHT_COLOR_RGB)
        .y_labels(GRAPH_DRAW_LABEL_NUMBER_MAX)
        .y_label_style(
            GRAPH_AXIS_Y_FONT
                .color(&GRAPH_AXIS_Y_FONT_COLOR_RGB.mix(GRAPH_AXIS_Y_FONT_COLOR_ALPHA)),
        )
        .y_label_formatter(&|y| {
            // Convert high-precision point in mmH20 back to cmH20 (which measurements & \
            //   targets both use)
            (y / TELEMETRY_POINTS_PRESSURE_PRECISION_DIVIDE as i32).to_string()
        })
        .draw()
        .expect("failed to draw pressure chart mesh");

    chart
        .draw_series(
            LineSeries::new(
                config.data_pressure.iter().map(|x| (x.0, x.1 as i32)),
                ShapeStyle::from(&GRAPH_PRESSURE_LINE_COLOR)
                    .filled()
                    .stroke_width(GRAPH_DRAW_LINE_SIZE),
            )
            .point_size(0),
        )
        .expect("failed to draw pressure chart data");
}

fn flow<'a>(
    master: &mut ControlWidget<'a>,
    config: &mut Config<'a>,
    size: (f64, f64),
    time_range: Range<DateTime<Utc>>,
) {
    // Create container
    gen_widget_container!(
        master,
        container_id: config.flow_id,
        color: color::TRANSPARENT,
        width: size.0,
        height: size.1,
        positions: bottom_left_of[
            config.wrapper_id,
        ]
    );

    // Create drawing
    let drawing = ConrodBackend::new(
        &mut master.ui,
        (size.0 as u32, size.1 as u32),
        config.flow_id,
        master.fonts.regular,
        &mut config.plot_graphs.1,
    )
    .into_drawing_area();

    let mut chart = ChartBuilder::on(&drawing)
        .margin_top(GRAPH_DRAW_MARGIN_TOP)
        .margin_bottom(GRAPH_DRAW_MARGIN_BOTTOM)
        .margin_left(GRAPH_DRAW_MARGIN_LEFT)
        .margin_right(GRAPH_DRAW_MARGIN_RIGHT)
        .x_label_area_size(0)
        .y_label_area_size(GRAPH_DRAW_LABEL_WIDTH)
        .build_cartesian_2d(
            time_range,
            GRAPH_DRAW_FLOW_RANGE_LOW_PRECISION_DIVIDED
                ..GRAPH_DRAW_FLOW_RANGE_HIGH_PRECISION_DIVIDED,
        )
        .expect("failed to build flow chart");

    chart
        .configure_mesh()
        .bold_line_style(&GRAPH_MESH_BOLD_COLOR_RGB.mix(GRAPH_MESH_BOLD_COLOR_ALPHA))
        .light_line_style(&GRAPH_MESH_LIGHT_COLOR_RGB)
        .y_labels(GRAPH_DRAW_LABEL_NUMBER_MAX)
        .y_label_style(
            GRAPH_AXIS_Y_FONT
                .color(&GRAPH_AXIS_Y_FONT_COLOR_RGB.mix(GRAPH_AXIS_Y_FONT_COLOR_ALPHA)),
        )
        .y_label_formatter(&|y| {
            // Convert high-precision point in mL back to L (which measurements & targets both use)
            (y / TELEMETRY_POINTS_FLOW_PRECISION_DIVIDE as i32).to_string()
        })
        .draw()
        .expect("failed to draw flow chart mesh");

    chart
        .draw_series(
            LineSeries::new(
                config.data_flow.iter().map(|x| (x.0, x.1 as i32)),
                ShapeStyle::from(&GRAPH_FLOW_LINE_COLOR)
                    .filled()
                    .stroke_width(GRAPH_DRAW_LINE_SIZE),
            )
            .point_size(0),
        )
        .expect("failed to draw flow chart data");
}
