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

use crate::chip::{ChipDataFlow, ChipDataGeneric, ChipDataPressure};
use crate::config::environment::*;
use crate::display::widget::ControlWidget;

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

struct PlotContext<'a, 'b> {
    value_range: Range<i32>,
    plot_id: WidgetId,
    precision_divide: i32,
    line_color: &'a RGBColor,
    data_values: &'b ChipDataGeneric,
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
    // Create pressure container
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

    // Draw pressure plot
    plot(
        master,
        size,
        time_range,
        &mut config.plot_graphs.0,
        PlotContext {
            value_range: GRAPH_DRAW_PRESSURE_RANGE_LOW_PRECISION_DIVIDED
                ..GRAPH_DRAW_PRESSURE_RANGE_HIGH_PRECISION_DIVIDED,
            plot_id: config.pressure_id,
            precision_divide: TELEMETRY_POINTS_PRESSURE_PRECISION_DIVIDE,
            line_color: &GRAPH_PRESSURE_LINE_COLOR,
            data_values: &config.data_pressure,
        },
    );
}

fn flow<'a>(
    master: &mut ControlWidget<'a>,
    config: &mut Config<'a>,
    size: (f64, f64),
    time_range: Range<DateTime<Utc>>,
) {
    // Create flow container
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

    // Draw flow plot
    plot(
        master,
        size,
        time_range,
        &mut config.plot_graphs.1,
        PlotContext {
            value_range: GRAPH_DRAW_FLOW_RANGE_LOW_PRECISION_DIVIDED
                ..GRAPH_DRAW_FLOW_RANGE_HIGH_PRECISION_DIVIDED,
            plot_id: config.flow_id,
            precision_divide: TELEMETRY_POINTS_FLOW_PRECISION_DIVIDE,
            line_color: &GRAPH_FLOW_LINE_COLOR,
            data_values: &config.data_flow,
        },
    );
}

fn plot<'a>(
    master: &mut ControlWidget<'a>,
    size: (f64, f64),
    time_range: Range<DateTime<Utc>>,
    plot_graph: &mut ConrodBackendReusableGraph,
    context: PlotContext,
) {
    // Create drawing backend
    let drawing = ConrodBackend::new(
        &mut master.ui,
        (size.0 as u32, size.1 as u32),
        context.plot_id,
        master.fonts.regular,
        plot_graph,
    )
    .into_drawing_area();

    // Configure chart
    let mut chart = ChartBuilder::on(&drawing)
        .margin_top(GRAPH_DRAW_MARGIN_TOP)
        .margin_bottom(GRAPH_DRAW_MARGIN_BOTTOM)
        .margin_left(GRAPH_DRAW_MARGIN_LEFT)
        .margin_right(GRAPH_DRAW_MARGIN_RIGHT)
        .x_label_area_size(0)
        .y_label_area_size(GRAPH_DRAW_LABEL_WIDTH)
        .build_cartesian_2d(time_range, context.value_range)
        .expect("failed to build chart");

    // Configure mesh
    let precision_divide = context.precision_divide;

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
            // Convert high-precision point to low-precision point (which measurements & targets \
            //   both use), eg. mL to L or mmH2O to cmH2O.
            (y / precision_divide).to_string()
        })
        .draw()
        .expect("failed to draw chart mesh");

    // Draw plot
    chart
        .draw_series(
            LineSeries::new(
                context.data_values.iter().map(|x| (x.0, x.1 as i32)),
                ShapeStyle::from(context.line_color)
                    .filled()
                    .stroke_width(GRAPH_DRAW_LINE_SIZE),
            )
            .point_size(0),
        )
        .expect("failed to draw chart data");
}
