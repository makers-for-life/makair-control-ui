// MakAir
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub const RUNTIME_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const WINDOW_ICON_WIDTH: u32 = 512;
pub const WINDOW_ICON_HEIGHT: u32 = 512;

pub const DISPLAY_WINDOW_SIZE_WIDTH: u32 = 800;
pub const DISPLAY_WINDOW_SIZE_HEIGHT: u32 = 480;

pub const BOOTLOADER_LOGO_WIDTH: u32 = 98;
pub const BOOTLOADER_LOGO_HEIGHT: u32 = 96;

pub const BRANDING_WIDTH: u32 = 121;
pub const BRANDING_HEIGHT: u32 = 45;
pub const BRANDING_IMAGE_MARGIN_TOP: f64 = 5.0;
pub const BRANDING_IMAGE_MARGIN_LEFT: f64 = 5.0;
pub const BRANDING_TEXT_MARGIN_TOP: f64 = 35.0;
pub const BRANDING_TEXT_MARGIN_LEFT: f64 = 63.0;

pub const STATUS_WRAPPER_WIDTH: f64 = 106.0;
pub const STATUS_WRAPPER_HEIGHT: f64 = 42.0;
pub const STATUS_WRAPPER_MARGIN_TOP: f64 = 5.0;
pub const STATUS_WRAPPER_MARGIN_RIGHT: f64 = 85.0;
pub const STATUS_BOX_TEXT_MARGIN_TOP: f64 = 3.0;
pub const STATUS_ALARM_CODE_POWER_BATTERY: u8 = 31;

pub const HEARTBEAT_GROUND_DIAMETER: f64 = 14.0;
pub const HEARTBEAT_SURROUND_MARGIN_TOP: f64 = 4.0;
pub const HEARTBEAT_SURROUND_MARGIN_RIGHT: f64 = 18.0;
pub const HEARTBEAT_SURROUND_DIAMETER: f64 = 44.0;
pub const HEARTBEAT_SURROUND_THICKNESS: f64 = 2.0;
pub const HEARTBEAT_INNER_MAX_OVERFLOW: u16 = 4;

pub const DISPLAY_ALARM_CONTAINER_WIDTH_NO_ALARMS: f64 = 400.0;
pub const DISPLAY_ALARM_CONTAINER_WIDTH_ALARMS: f64 = 580.0;
pub const DISPLAY_ALARM_CONTAINER_MARGIN_TOP_SINGLE_OR_NONE: f64 = 5.0;
pub const DISPLAY_ALARM_CONTAINER_MARGIN_TOP_MULTIPLE: f64 = 5.0;
pub const DISPLAY_ALARM_CONTAINER_PADDING_TOP: f64 = 11.5;
pub const DISPLAY_ALARM_CONTAINER_PADDING_LEFT: f64 = 22.0;

pub const DISPLAY_ALARM_MESSAGE_WIDTH: f64 = 440.0;
pub const DISPLAY_ALARM_MESSAGE_HEIGHT: f64 = 33.0;
pub const DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL_OFFSET: f64 = 5.0; // TODO: this is ugly, idk why the top_initial value below creates a visible offset when set to zero; offset empirically checked to be 5.0f
pub const DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL: f64 = 8.0;
pub const DISPLAY_ALARM_MESSAGE_SPACING_TOP_INNER: f64 = 6.0;

pub const DISPLAY_ALARM_CODE_WIDTH: f64 = 35.0;
pub const DISPLAY_ALARM_CODE_HEIGHT: f64 = DISPLAY_ALARM_MESSAGE_HEIGHT;
pub const DISPLAY_ALARM_CODE_IGNORES: &[u8] = &[STATUS_ALARM_CODE_POWER_BATTERY];

pub const DISPLAY_ROUNDED_RECTANGLES_ROUND: f64 = 2.0;

pub const DISPLAY_STOPPED_MESSAGE_CONTAINER_WIDTH: f64 = 360.0;
pub const DISPLAY_STOPPED_MESSAGE_CONTAINER_HEIGHT: f64 = 84.0;
pub const DISPLAY_STOPPED_MESSAGE_PADDING_TOP: f64 = 16.0;
pub const DISPLAY_STOPPED_MESSAGE_PADDING_BOTTOM: f64 = 22.0;

pub const TELEMETRY_POINTS_PRECISION_DIVIDE: u16 = 10;
pub const TELEMETRY_POINTS_PER_SECOND: usize = 100;
pub const TELEMETRY_POINTS_LOW_PASS_DEGREE: u16 = 4;

pub const TELEMETRY_WIDGET_SPACING_FROM_LEFT: f64 = 6.0;
pub const TELEMETRY_WIDGET_SPACING_FROM_BOTTOM: f64 = 16.0;
pub const TELEMETRY_WIDGET_SIZE_WIDTH: f64 = DISPLAY_WINDOW_SIZE_WIDTH as f64 / 4.0;
pub const TELEMETRY_WIDGET_SIZE_HEIGHT: f64 = 80.0;
pub const TELEMETRY_WIDGET_SIZE_SPACING: f64 = 8.0;
pub const TELEMETRY_WIDGET_PADDING_LEFT: f64 = 5.0;
pub const TELEMETRY_WIDGET_NUMBER_RIGHT: f64 = 3.0;

pub const TELEMETRY_ARROW_WIDTH: u32 = 15;
pub const TELEMETRY_ARROW_HEIGHT: u32 = 9;
pub const TELEMETRY_ARROW_SPACING_SIDES: f64 = 5.0;

pub const GRAPH_DRAW_SPACING_FROM_BOTTOM: f64 = 120.0;
pub const GRAPH_DRAW_SECONDS: usize = 5;
pub const GRAPH_DRAW_RANGE_LOW: i32 = 0;
pub const GRAPH_DRAW_MARGIN_TOP: u32 = 0;
pub const GRAPH_DRAW_MARGIN_BOTTOM: u32 = 10;
pub const GRAPH_DRAW_MARGIN_LEFT: u32 = 0;
pub const GRAPH_DRAW_MARGIN_RIGHT: u32 = 0;
pub const GRAPH_DRAW_LINE_SIZE: u32 = 2;
pub const GRAPH_DRAW_POINT_SIZE: u32 = 0;
pub const GRAPH_DRAW_LABEL_JITTER_FIX_WIDTH: u32 = 40;
pub const GRAPH_DRAW_LABEL_WIDTH: u32 = 28;
pub const GRAPH_DRAW_LABEL_NUMBER_MAX: usize = 5;
pub const GRAPH_NUMBER_OF_POINTS: usize = GRAPH_DRAW_SECONDS * TELEMETRY_POINTS_PER_SECOND;
pub const GRAPH_WIDTH: u32 = 650;
pub const GRAPH_HEIGHT: u32 = 315;

pub const PEAK_PRESSURE_INITIAL_MIN: f64 = 0.0;
pub const PEAK_PRESSURE_ALERT_ERROR_RATIO: f64 = 0.15;
pub const CYCLE_RATIO_INSPIRATION: u8 = 1;
pub const CYCLE_RATIO_EXPIRATION: u8 = 1;

pub const SETTINGS_MODAL_WIDTH: f64 = 600.0;
pub const SETTINGS_MODAL_HEIGTH: f64 = 400.0;

pub const LAYOUT_HEADER_SIZE_FULL_HEIGHT: f64 = DISPLAY_WINDOW_SIZE_HEIGHT as f64 - LAYOUT_FOOTER_SIZE_HEIGHT; // So the alarms can overflow the parent
pub const LAYOUT_HEADER_SIZE_HEIGHT: f64 = 65.0;
pub const LAYOUT_BODY_SIZE_HEIGHT: f64 = GRAPH_HEIGHT as _;
pub const LAYOUT_FOOTER_SIZE_HEIGHT: f64 = 100.0;

#[cfg(feature = "lora")]
pub const LORA_GPIO_PIN_NUMBER: u64 = 25;
#[cfg(feature = "lora")]
pub const LORA_DEVICE_PATH: &str = "/dev/ttyAMA0";

#[cfg(not(feature = "graph-scaler"))]
pub const GRAPH_DRAW_RANGE_HIGH_STATIC_INITIAL: u8 = 65;

#[cfg(feature = "graph-scaler")]
pub const GRAPH_DRAW_RANGE_HIGH_DYNAMIC_INITIAL: u8 = 20;
