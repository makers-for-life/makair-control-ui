// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use lazy_static::lazy_static;

pub const RUNTIME_NAME: &str = env!("CARGO_PKG_NAME");
pub const RUNTIME_VERSION: &str = env!("CARGO_PKG_VERSION");

lazy_static! {
    pub static ref SIMULATOR_ENABLED: bool = {
        #[cfg(not(feature = "simulator"))]
        let enabled = false;

        #[cfg(feature = "simulator")]
        let enabled = crate::APP_ARGS.mode == crate::config::arguments::RunMode::Simulator;

        enabled
    };
    pub static ref ADVANCED_SETTINGS_GROUP_TABS_COUNT: usize =
        if *SIMULATOR_ENABLED { 3 } else { 2 };
}

pub const FACTORF64: f64 = 1.0;
pub const WINDOW_ICON_WIDTH: u32 = 512;
pub const WINDOW_ICON_HEIGHT: u32 = 512;

pub const DISPLAY_WINDOW_SIZE_WIDTH: u32 = (800_f64 * FACTORF64) as u32;
pub const DISPLAY_WINDOW_SIZE_HEIGHT: u32 = (480_f64 * FACTORF64) as u32;
pub const DISPLAY_FRAMERATE_SMOOTH_HEAVY: u64 = 30;
pub const DISPLAY_FRAMERATE_MODERATE_FAST: u64 = 20;
pub const DISPLAY_FRAMERATE_JERKY_FAST: u64 = 10;

pub const BOOTLOADER_LOGO_WIDTH: u32 = (98_f64 * FACTORF64) as u32;
pub const BOOTLOADER_LOGO_HEIGHT: u32 = (96_f64 * FACTORF64) as u32;

pub const ERROR_WIDTH: f64 = 180.0 * FACTORF64;
pub const ERROR_HEIGHT: f64 = 186.0 * FACTORF64;
pub const ERROR_ICON_WIDTH: u32 = (115_f64 * FACTORF64) as u32;
pub const ERROR_ICON_HEIGHT: u32 = (101_f64 * FACTORF64) as u32;
pub const ERROR_TEXT_BOX_HEIGHT: f64 = 50.0 * FACTORF64;
pub const ERROR_TITLE_FONT_SIZE: u32 = (19_f64 * FACTORF64) as u32;
pub const ERROR_MESSAGE_FONT_SIZE: u32 = (16_f64 * FACTORF64) as u32;

pub const END_OF_LINE_TITLE_WIDTH: f64 = DISPLAY_WINDOW_SIZE_WIDTH as f64;
pub const END_OF_LINE_TITLE_HEIGHT: f64 = 42.0 * FACTORF64;
pub const END_OF_LINE_TITLE_FONT_SIZE: u32 = (14_f64 * FACTORF64) as u32;
pub const END_OF_LINE_TITLE_SEPARATOR_SPACING: f64 = 15.0 * FACTORF64;
pub const END_OF_LINE_STEPS_WRAPPER_WIDTH: f64 = DISPLAY_WINDOW_SIZE_WIDTH as f64;
pub const END_OF_LINE_STEPS_WRAPPER_HEIGHT: f64 = 100.0 * FACTORF64;
pub const END_OF_LINE_STEPS_ITEMS_WIDTH: f64 = END_OF_LINE_STEPS_CIRCLE_SIZE
    * END_OF_LINE_STEPS_COUNT as f64
    + END_OF_LINE_STEPS_CIRCLE_MARGIN_RIGHT * (END_OF_LINE_STEPS_COUNT - 1) as f64;
pub const END_OF_LINE_STEPS_ITEMS_HEIGHT: f64 = END_OF_LINE_STEPS_CIRCLE_SIZE;
pub const END_OF_LINE_STEPS_PROGRESS_HEIGHT: f64 = 1.0 * FACTORF64;
pub const END_OF_LINE_STEPS_COUNT: u8 = 10;
pub const END_OF_LINE_STEPS_CIRCLE_SIZE: f64 = 34.0 * FACTORF64;
pub const END_OF_LINE_STEPS_CIRCLE_THICKNESS: f64 = 1.0 * FACTORF64;
pub const END_OF_LINE_STEPS_CIRCLE_MARGIN_RIGHT: f64 = 24.0 * FACTORF64;
pub const END_OF_LINE_STEPS_INDEX_FONT_SIZE: u32 = (14_f64 * FACTORF64) as u32;
pub const END_OF_LINE_CONTENT_WIDTH: f64 = DISPLAY_WINDOW_SIZE_WIDTH as f64;
pub const END_OF_LINE_CONTENT_HEIGHT: f64 =
    WINDOW_ICON_HEIGHT as f64 - END_OF_LINE_TITLE_HEIGHT - END_OF_LINE_STEPS_WRAPPER_HEIGHT;
pub const END_OF_LINE_CONTENT_BOX_WIDTH: f64 = 680.0 * FACTORF64;
pub const END_OF_LINE_CONTENT_BOX_HEIGHT: f64 = 160.0 * FACTORF64;
pub const END_OF_LINE_CONTENT_BOX_OFFSET_TOP: f64 = 36.0 * FACTORF64;
pub const END_OF_LINE_CONTENT_ICON_WIDTH: u32 = (100_f64 * FACTORF64) as u32;
pub const END_OF_LINE_CONTENT_ICON_HEIGHT: u32 = END_OF_LINE_CONTENT_ICON_WIDTH;
pub const END_OF_LINE_CONTENT_ICON_SIZE: f64 = END_OF_LINE_CONTENT_ICON_WIDTH as f64;
pub const END_OF_LINE_CONTENT_ICON_SPACING_SIDES: f64 = 40.0 * FACTORF64;
pub const END_OF_LINE_CONTENT_TEXT_WRAPPER_MARGIN_RIGHT: f64 = 40.0 * FACTORF64;
pub const END_OF_LINE_CONTENT_TEXT_WRAPPER_WIDTH: f64 = END_OF_LINE_CONTENT_BOX_WIDTH
    - (END_OF_LINE_CONTENT_ICON_SIZE
        + (2.0 * END_OF_LINE_CONTENT_ICON_SPACING_SIDES)
        + END_OF_LINE_CONTENT_TEXT_WRAPPER_MARGIN_RIGHT);
pub const END_OF_LINE_CONTENT_TEXT_WRAPPER_HEIGHT: f64 = END_OF_LINE_CONTENT_TEXT_TITLE_FONT_SIZE
    as f64
    + END_OF_LINE_CONTENT_TEXT_MESSAGE_FONT_SIZE as f64
    + END_OF_LINE_CONTENT_TEXT_MESSAGE_MARGIN_TOP
    + 8.0;
pub const END_OF_LINE_CONTENT_TEXT_TITLE_FONT_SIZE: u32 = (19_f64 * FACTORF64) as u32;
pub const END_OF_LINE_CONTENT_TEXT_MESSAGE_FONT_SIZE: u32 = (14_f64 * FACTORF64) as u32;
pub const END_OF_LINE_CONTENT_TEXT_MESSAGE_MARGIN_TOP: f64 = 19.0 * FACTORF64;
pub const END_OF_LINE_CONTENT_DETAILS_BOX_WIDTH: f64 = END_OF_LINE_CONTENT_BOX_WIDTH;
pub const END_OF_LINE_CONTENT_DETAILS_BOX_HEIGHT: f64 = 34.0 * FACTORF64;
pub const END_OF_LINE_CONTENT_DETAILS_BOX_MARGIN_BOTTOM: f64 = 16.0 * FACTORF64;
pub const END_OF_LINE_CONTENT_DETAILS_TEXT_FONT_SIZE: u32 = (13_f64 * FACTORF64) as u32;
pub const END_OF_LINE_CONTENT_BUTTON_FONT_SIZE: u32 = MODAL_BUTTON_FONT_SIZE;
pub const END_OF_LINE_CONTENT_BUTTON_WIDTH: f64 = 128.0 * FACTORF64;
pub const END_OF_LINE_CONTENT_BUTTON_VALUE_TOP: f64 = MODAL_BUTTON_VALUE_TOP;
pub const END_OF_LINE_CONTENT_BUTTON_MARGIN_BOTTOM: f64 = 20.0 * FACTORF64;

pub const INITIALIZING_WIDTH: f64 = 160.0 * FACTORF64;
pub const INITIALIZING_HEIGHT: f64 = 138.0 * FACTORF64;
pub const INITIALIZING_MESSAGE_FONT_SIZE: u32 = (20_f64 * FACTORF64) as u32;

pub const BRANDING_WIDTH: u32 = (121_f64 * FACTORF64) as u32;
pub const BRANDING_HEIGHT: u32 = (45_f64 * FACTORF64) as u32;
pub const BRANDING_IMAGE_MARGIN_TOP: f64 = 3.0 * FACTORF64;
pub const BRANDING_IMAGE_MARGIN_LEFT: f64 = 5.0 * FACTORF64;
pub const BRANDING_TEXT_MARGIN_TOP: f64 = 33.0 * FACTORF64;
pub const BRANDING_TEXT_MARGIN_LEFT: f64 = 61.0 * FACTORF64;
pub const BRANDING_FONT_SIZE: u32 = (10_f64 * FACTORF64) as u32;
pub const BRANDING_TEXT_VERSION_NONE: &str = "v0.0.0";

pub const CONTROLS_WRAPPER_HEIGHT: f64 = 44.0 * FACTORF64;
pub const CONTROLS_WRAPPER_WIDTH: f64 =
    (CONTROLS_BUTTON_FOOTPRINT_WIDTH * CONTROLS_BUTTONS_COUNT) - CONTROLS_BUTTON_MARGIN_LEFT;
pub const CONTROLS_WRAPPER_MARGIN_TOP: f64 = 4.0 * FACTORF64;
pub const CONTROLS_WRAPPER_MARGIN_RIGHT: f64 = 194.0 * FACTORF64;
pub const CONTROLS_BUTTONS_COUNT: f64 = 3.0;
pub const CONTROLS_BUTTON_DIAMETER: f64 = CONTROLS_WRAPPER_HEIGHT;
pub const CONTROLS_BUTTON_RADIUS: f64 = CONTROLS_BUTTON_DIAMETER / 2.0;
pub const CONTROLS_BUTTON_FOOTPRINT_WIDTH: f64 =
    CONTROLS_BUTTON_DIAMETER + CONTROLS_BUTTON_MARGIN_LEFT;
pub const CONTROLS_BUTTON_MARGIN_LEFT: f64 = 4.0 * FACTORF64;

pub const STATUS_WRAPPER_WIDTH: f64 = 112.0 * FACTORF64;
pub const STATUS_WRAPPER_HEIGHT: f64 = 42.0 * FACTORF64;
pub const STATUS_WRAPPER_MARGIN_TOP: f64 = 5.0 * FACTORF64;
pub const STATUS_WRAPPER_MARGIN_RIGHT: f64 = 66.0 * FACTORF64;
pub const STATUS_BOX_TEXT_MARGIN_TOP: f64 = 3.0 * FACTORF64;
pub const STATUS_BOX_RECORDING_PADDING_LEFT: f64 = 12.0 * FACTORF64;
pub const STATUS_BOX_RECORDING_PADDING_RIGHT: f64 = 5.0 * FACTORF64;
pub const STATUS_RECORDING_OUTER_RADIUS: f64 = 6.0 * FACTORF64;
pub const STATUS_RECORDING_INNER_RADIUS: f64 = 5.0 * FACTORF64;
pub const STATUS_FONT_SIZE: u32 = (11_f64 * FACTORF64) as u32;

pub const HEARTBEAT_GROUND_DIAMETER: f64 = 14.0 * FACTORF64;
pub const HEARTBEAT_SURROUND_MARGIN_TOP: f64 = 4.0 * FACTORF64;
pub const HEARTBEAT_SURROUND_MARGIN_RIGHT: f64 = 6.0 * FACTORF64;
pub const HEARTBEAT_SURROUND_DIAMETER: f64 = 44.0 * FACTORF64;
pub const HEARTBEAT_SURROUND_THICKNESS: f64 = 2.0 * FACTORF64;
pub const HEARTBEAT_INNER_MAX_OVERFLOW: u16 = 4;

pub const DISPLAY_ALARM_CONTAINER_WIDTH_BASE: f64 = 305.0 * FACTORF64;
pub const DISPLAY_ALARM_CONTAINER_MARGIN_LEFT_BASE: f64 = 20.0 * FACTORF64;
pub const DISPLAY_ALARM_CONTAINER_WIDTH_HAS_ALARMS_OFFSET: f64 =
    DISPLAY_ALARM_CONTAINER_MARGIN_LEFT_BASE;
pub const DISPLAY_ALARM_CONTAINER_MARGIN_LEFT_HAS_ALARMS: f64 = -1.0 * BRANDING_WIDTH as f64;
pub const DISPLAY_ALARM_CONTAINER_PADDING_TOP: f64 = 12.5 * FACTORF64;
pub const DISPLAY_ALARM_CONTAINER_EMPTY_PADDING_LEFT: f64 = 22.0 * FACTORF64;
pub const DISPLAY_ALARM_CONTAINER_HAS_ALARMS_PADDING_LEFT: f64 = 16.0 * FACTORF64;
pub const DISPLAY_ALARM_CONTAINER_TITLE_TO_ALARM_EMPTY_SPACING: f64 = 38.0 * FACTORF64;
pub const DISPLAY_ALARM_CONTAINER_TITLE_TO_ALARM_HAS_ALARMS_SPACING: f64 = 20.0 * FACTORF64;

pub const DISPLAY_ALARM_TITLE_WRAPPER_WIDTH: f64 = 65.0 * FACTORF64;
pub const DISPLAY_ALARM_TITLE_WRAPPER_HEIGHT: f64 = 18.0 * FACTORF64;
pub const ALARM_TITLE_FONT_SIZE: u32 = (14_f64 * FACTORF64) as u32;
pub const ALARMS_EMPTY_FONT_SIZE: u32 = (12_f64 * FACTORF64) as u32;

pub const DISPLAY_ALARM_MESSAGE_WIDTH: f64 = 302.0 * FACTORF64;
pub const DISPLAY_ALARM_MESSAGE_HEIGHT: f64 = 30.0 * FACTORF64;
pub const DISPLAY_ALARM_MESSAGE_SPACING_TOP_INNER: f64 = 4.0 * FACTORF64;
pub const DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL_OFFSET: f64 = 5.0 * FACTORF64;
pub const DISPLAY_ALARM_MESSAGE_SPACING_TOP_INITIAL: f64 = 10.0 * FACTORF64;
pub const DISPLAY_ALARM_MESSAGE_SPACING_BOTTOM_INITIAL: f64 = 5.0 * FACTORF64;
pub const DISPLAY_ALARM_MESSAGE_FONT_SIZE: u32 = (17_f64 * FACTORF64) as u32;

pub const DISPLAY_ALARM_CODE_WIDTH: f64 = 32.0 * FACTORF64;
pub const DISPLAY_ALARM_CODE_HEIGHT: f64 = DISPLAY_ALARM_MESSAGE_HEIGHT;
pub const DISPLAY_ALARM_CODE_FONT_SIZE: u32 = (19_f64 * FACTORF64) as u32;

pub const DISPLAY_ROUNDED_RECTANGLES_ROUND: f64 = 2.0 * FACTORF64;

pub const DISPLAY_STOP_MESSAGE_CONTAINER_WIDTH: f64 = 380.0 * FACTORF64;
pub const DISPLAY_STOP_MESSAGE_CONTAINER_HEIGHT: f64 = 84.0 * FACTORF64;
pub const DISPLAY_STOP_MESSAGE_PADDING_TOP: f64 = 16.0 * FACTORF64;
pub const DISPLAY_STOP_MESSAGE_PADDING_BOTTOM: f64 = 22.0 * FACTORF64;
pub const DISPLAY_STOP_TITLE_FONT_SIZE: u32 = (19_f64 * FACTORF64) as u32;
pub const DISPLAY_STOP_FONT_SIZE: u32 = (14_f64 * FACTORF64) as u32;

pub const TELEMETRY_POINTS_PRESSURE_PRECISION_DIVIDE: i32 = 10;
pub const TELEMETRY_POINTS_FLOW_PRECISION_DIVIDE: i32 = 100;
pub const TELEMETRY_POINTS_PER_SECOND: usize = 40;
pub const TELEMETRY_POINTS_LOW_PASS_DEGREE: i16 = 2;

pub const TELEMETRY_WIDGET_BOTTOM_COUNT: f64 = 4.0;
pub const TELEMETRY_WIDGET_RIGHT_COUNT: f64 = 3.0;
pub const TELEMETRY_WIDGET_SPACING_SIDES: f64 = 4.0 * FACTORF64;
pub const TELEMETRY_WIDGET_BOTTOM_SIZE_WIDTH: f64 = DISPLAY_WINDOW_SIZE_WIDTH as f64
    / TELEMETRY_WIDGET_BOTTOM_COUNT
    - TELEMETRY_WIDGET_SPACING_SIDES * (TELEMETRY_WIDGET_BOTTOM_COUNT - 1.0)
        / TELEMETRY_WIDGET_BOTTOM_COUNT;
pub const TELEMETRY_WIDGET_RIGHT_MODE_HEIGHT: f64 = 39.0 * FACTORF64;
pub const TELEMETRY_WIDGET_RIGHT_MODE_HEIGHT_WITH_SPACING: f64 =
    TELEMETRY_WIDGET_RIGHT_MODE_HEIGHT + TELEMETRY_WIDGET_SPACING_SIDES;
pub const TELEMETRY_WIDGET_RIGHT_MODE_FONT_SIZE: u32 = (14_f64 * FACTORF64) as u32;
pub const TELEMETRY_WIDGET_RIGHT_MODE_SEPARATOR_SPACING: f64 = 9.0 * FACTORF64;
pub const TELEMETRY_WIDGET_RIGHT_SIZE_WIDTH: f64 = (DISPLAY_WINDOW_SIZE_WIDTH - GRAPH_WIDTH) as f64;
pub const TELEMETRY_WIDGET_RIGHT_SIZE_HEIGHT: f64 = (GRAPH_HEIGHT as f64
    - TELEMETRY_WIDGET_RIGHT_MODE_HEIGHT_WITH_SPACING)
    / TELEMETRY_WIDGET_RIGHT_COUNT
    - TELEMETRY_WIDGET_SPACING_SIDES;
pub const TELEMETRY_WIDGET_RIGHT_POSITION_Y_BASE: f64 = GRAPH_HEIGHT as f64
    + LAYOUT_FOOTER_SIZE_HEIGHT
    - TELEMETRY_WIDGET_RIGHT_MODE_HEIGHT_WITH_SPACING;
pub const TELEMETRY_WIDGET_PADDING_LEFT: f64 = 7.0 * FACTORF64;
pub const TELEMETRY_WIDGET_UNIT_PADDING_BOTTOM_TOP: f64 = 8.0 * FACTORF64;
pub const TELEMETRY_WIDGET_UNIT_BORDER_RADIUS: f64 = 5.0 * FACTORF64;
pub const TELEMETRY_WIDGET_TITLE_FONT_SIZE: u32 = (16_f64 * FACTORF64) as u32;
pub const TELEMETRY_WIDGET_UNIT_FONT_SIZE: u32 = (12_f64 * FACTORF64) as u32;
pub const TELEMETRY_WIDGET_VALUE_FONT_SIZE: u32 = (40_f64 * FACTORF64) as u32;
pub const TELEMETRY_WIDGET_TARGET_VALUE_FONT_SIZE: u32 = (23_f64 * FACTORF64) as u32;
pub const TELEMETRY_WIDGET_VALUE_EMPTY: &str = "--";
pub const TELEMETRY_WIDGET_CYCLES_RATIO_INSPIRATION: u8 = 1;

pub const TELEMETRY_ARROW_MAIN_WIDTH: u32 = (10_f64 * FACTORF64) as u32;
pub const TELEMETRY_ARROW_MAIN_HEIGHT: u32 = (11_f64 * FACTORF64) as u32;
pub const TELEMETRY_ARROW_LINE_WIDTH: u32 = (6_f64 * FACTORF64) as u32;
pub const TELEMETRY_ARROW_LINE_HEIGHT: u32 = (3_f64 * FACTORF64) as u32;
pub const TELEMETRY_ARROW_SPACING_SIDES: f64 = 5.0 * FACTORF64;

pub const GRAPH_DRAW_SECONDS: i64 = 5;
pub const GRAPH_DRAW_PRESSURE_RANGE_LOW: i32 = -10;
pub const GRAPH_DRAW_PRESSURE_RANGE_LOW_PRECISION_DIVIDED: i32 =
    GRAPH_DRAW_PRESSURE_RANGE_LOW * TELEMETRY_POINTS_PRESSURE_PRECISION_DIVIDE;
pub const GRAPH_DRAW_PRESSURE_RANGE_LOW_PRECISION_DIVIDED_SMALL: i16 =
    GRAPH_DRAW_PRESSURE_RANGE_LOW_PRECISION_DIVIDED as i16;
pub const GRAPH_DRAW_PRESSURE_RANGE_HIGH: u8 = 45;
pub const GRAPH_DRAW_PRESSURE_RANGE_HIGH_PRECISION_DIVIDED: i32 =
    (GRAPH_DRAW_PRESSURE_RANGE_HIGH as i32) * TELEMETRY_POINTS_PRESSURE_PRECISION_DIVIDE;
pub const GRAPH_DRAW_PRESSURE_RANGE_HIGH_PRECISION_DIVIDED_SMALL: i16 =
    GRAPH_DRAW_PRESSURE_RANGE_HIGH_PRECISION_DIVIDED as i16;
pub const GRAPH_DRAW_FLOW_RANGE_LOW: i32 = -GRAPH_DRAW_FLOW_RANGE_HIGH;
pub const GRAPH_DRAW_FLOW_RANGE_LOW_PRECISION_DIVIDED: i32 =
    GRAPH_DRAW_FLOW_RANGE_LOW * TELEMETRY_POINTS_FLOW_PRECISION_DIVIDE;
pub const GRAPH_DRAW_FLOW_RANGE_LOW_PRECISION_DIVIDED_SMALL: i16 =
    GRAPH_DRAW_FLOW_RANGE_LOW_PRECISION_DIVIDED as i16;
pub const GRAPH_DRAW_FLOW_RANGE_HIGH: i32 = 70;
pub const GRAPH_DRAW_FLOW_RANGE_HIGH_PRECISION_DIVIDED: i32 =
    GRAPH_DRAW_FLOW_RANGE_HIGH * TELEMETRY_POINTS_FLOW_PRECISION_DIVIDE;
pub const GRAPH_DRAW_FLOW_RANGE_HIGH_PRECISION_DIVIDED_SMALL: i16 =
    GRAPH_DRAW_FLOW_RANGE_HIGH_PRECISION_DIVIDED as i16;
pub const GRAPH_DRAW_MARGIN_TOP: u32 = 0;
pub const GRAPH_DRAW_MARGIN_BOTTOM: u32 = (10_f64 * FACTORF64) as u32;
pub const GRAPH_DRAW_MARGIN_LEFT: u32 = 0;
pub const GRAPH_DRAW_MARGIN_RIGHT: u32 = 0;
pub const GRAPH_DRAW_LINE_SIZE: u32 = (2_f64 * FACTORF64) as u32;
pub const GRAPH_DRAW_AXIS_SIZE: u32 = FACTORF64 as u32;
pub const GRAPH_DRAW_AXIS_FONT_SIZE: u32 = (14_f64 * FACTORF64) as u32;
pub const GRAPH_DRAW_LABEL_WIDTH: u32 = (56_f64 * FACTORF64) as u32;
pub const GRAPH_DRAW_LABEL_NUMBER_MAX: usize = 5;
pub const GRAPH_LABEL_BOX_WIDTH: f64 = (GRAPH_DRAW_LABEL_WIDTH - 2 * GRAPH_DRAW_AXIS_SIZE) as f64;
pub const GRAPH_LABEL_BOX_HEIGHT: f64 = 36.0 * FACTORF64;
pub const GRAPH_LABEL_BOX_FONT_SIZE: u32 = (11_f64 * FACTORF64) as u32;
pub const GRAPH_SATURATE_LINE_THICKNESS: f64 = 3.0 * FACTORF64;
pub const GRAPH_NUMBER_OF_POINTS: usize = GRAPH_DRAW_SECONDS as usize * TELEMETRY_POINTS_PER_SECOND;
pub const GRAPH_WIDTH: u32 = (650_f64 * FACTORF64) as u32;
pub const GRAPH_HEIGHT: u32 = (325_f64 * FACTORF64) as u32;
pub const GRAPH_SPACING: f64 = 6.0 * FACTORF64;

pub const MAX_ALLOWED_PRESSURE_INITIAL_MINIMUM: f64 = 0.0;
pub const MAX_ALLOWED_PRESSURE_ALERT_ERROR_RATIO: f64 = 0.15;

pub const PRESET_SETTINGS_MODAL_PADDING: f64 = 20.0 * FACTORF64;
pub const PRESET_SETTINGS_MODAL_WIDTH: f64 = 630.0 * FACTORF64;
pub const PRESET_SETTINGS_MODAL_HEIGTH: f64 = 360.0 * FACTORF64;
pub const PRESET_SETTINGS_MODAL_TEXTURE_WIDTH: u32 = (110_f64 * FACTORF64) as u32;
pub const PRESET_SETTINGS_MODAL_TEXTURE_HEIGHT: u32 = (180_f64 * FACTORF64) as u32;
pub const PRESET_SETTINGS_MODAL_TITLE_TOTAL_HEIGHT: f64 = 78.0 * FACTORF64;
pub const PRESET_SETTINGS_MODAL_CONTENT_IMAGE_WIDTH: f64 = PRESET_SETTINGS_MODAL_TEXTURE_WIDTH as _;
pub const PRESET_SETTINGS_MODAL_CONTENT_IMAGE_HEIGHT: f64 =
    PRESET_SETTINGS_MODAL_TEXTURE_HEIGHT as _;
pub const PRESET_SETTINGS_MODAL_CONTENT_SEPARATOR_MARGIN_SIDES: f64 = 30.0 * FACTORF64;
pub const PRESET_SETTINGS_MODAL_CONTENT_FORM_FIELD_COUNT: f64 = 3.0;
pub const PRESET_SETTINGS_MODAL_CONTENT_FORM_FIELD_HEIGHT_PADDED: f64 = 60.0 * FACTORF64;
pub const PRESET_SETTINGS_MODAL_CONTENT_FORM_PADDING_LEFT: f64 = 150.0 * FACTORF64;
pub const PRESET_SETTINGS_TITLE_FONT_SIZE: u32 = (19_f64 * FACTORF64) as u32;
pub const PRESET_SETTINGS_SECONDARY_TITLE_FONT_SIZE: u32 = (15_f64 * FACTORF64) as u32;

pub const RUN_SETTINGS_MODAL_PADDING: f64 = 20.0 * FACTORF64;
pub const RUN_SETTINGS_MODAL_WIDTH: f64 = 600.0 * FACTORF64;
pub const RUN_SETTINGS_MODAL_HEIGTH: f64 = 140.0 * FACTORF64;
pub const RUN_SETTINGS_MODAL_FORM_PADDING_LEFT: f64 = 240.0 * FACTORF64;
pub const RUN_SETTINGS_BUTTON_WIDTH: f64 = 280.0 * FACTORF64;

pub const SNOOZE_SETTINGS_MODAL_PADDING: f64 = 20.0 * FACTORF64;
pub const SNOOZE_SETTINGS_MODAL_WIDTH: f64 = 600.0 * FACTORF64;
pub const SNOOZE_SETTINGS_MODAL_HEIGTH: f64 = 140.0 * FACTORF64;
pub const SNOOZE_SETTINGS_MODAL_FORM_PADDING_LEFT: f64 = 200.0 * FACTORF64;

pub const ADVANCED_SETTINGS_MODAL_PADDING: f64 = 20.0 * FACTORF64;
pub const ADVANCED_SETTINGS_MODAL_WIDTH: f64 = 680.0 * FACTORF64;
pub const ADVANCED_SETTINGS_MODAL_HEIGTH: f64 = 102.0 * FACTORF64
    + (ADVANCED_SETTINGS_LINES_COUNT as f64
        * (ADVANCED_SETTINGS_LINE_FONT_SIZE as f64 + ADVANCED_SETTINGS_LINE_MARGIN_TOP)
        - ADVANCED_SETTINGS_LINE_MARGIN_TOP);
pub const ADVANCED_SETTINGS_MODAL_FORM_PADDING_LEFT: f64 = 192.0 * FACTORF64;
pub const ADVANCED_SETTINGS_MODAL_FORM_FIELD_HEIGHT_PADDED: f64 = 42.0 * FACTORF64;
pub const ADVANCED_SETTINGS_LINES_COUNT: usize = 15;
pub const ADVANCED_SETTINGS_LINE_MARGIN_TOP: f64 = 8.0 * FACTORF64;
pub const ADVANCED_SETTINGS_LINE_FONT_SIZE: u32 = (14_f64 * FACTORF64) as u32;
pub const ADVANCED_SETTINGS_LINE_VALUE_PADDING_LEFT: f64 = 240.0 * FACTORF64;
pub const ADVANCED_SETTINGS_LINE_VALUE_EMPTY: &str = "--";

pub const MODE_SETTINGS_MODAL_PADDING: f64 = 20.0 * FACTORF64;
pub const MODE_SETTINGS_MODAL_WIDTH: f64 = 740.0 * FACTORF64;
pub const MODE_SETTINGS_MODAL_HEIGTH: f64 = 470.0 * FACTORF64;
pub const MODE_SETTINGS_MODAL_FORM_PADDING_LEFT: f64 = 260.0 * FACTORF64;
pub const MODE_SETTINGS_MODAL_FORM_FIELD_HEIGHT_PADDED: f64 = 42.0 * FACTORF64;
pub const MODE_SETTINGS_SELECTOR_TABS_COUNT: usize = 5;
pub const MODE_SETTINGS_SELECTOR_TABS_HEIGHT: f64 = 48.0 * FACTORF64;
pub const MODE_SETTINGS_GROUP_TABS_COUNT: usize = 2;
pub const MODE_SETTINGS_FONT_SIZE: u32 = (14_f64 * FACTORF64) as u32;

pub const BUTTON_HEIGHT: f64 = 34.0 * FACTORF64;
pub const BUTTON_BORDER_RADIUS: f64 = BUTTON_HEIGHT / 2.0;

pub const MODAL_SIZE_ADJUST_OVERFLOW: u32 = (2_f64 * FACTORF64) as u32;
pub const MODAL_TEXT_FONT_SIZE: u32 = (18_f64 * FACTORF64) as u32;
pub const MODAL_BUTTON_FONT_SIZE: u32 = (16_f64 * FACTORF64) as u32;
pub const MODAL_BUTTON_VALUE_TOP: f64 = 6.0 * FACTORF64;
pub const MODAL_BUTTON_NAVIGATE_FONT_SIZE: u32 = (20_f64 * FACTORF64) as u32;
pub const MODAL_BUTTON_NAVIGATE_VALUE_WIDTH: f64 = 120.0 * FACTORF64;
pub const MODAL_BUTTON_NAVIGATE_VALUE_HEIGHT: f64 = BUTTON_HEIGHT;
pub const MODAL_BUTTON_NAVIGATE_VALUE_MARGIN_TOP: f64 = 4.0 * FACTORF64;
pub const MODAL_BUTTON_NAVIGATE_VALUE_DECREASE: &str = "<";
pub const MODAL_BUTTON_NAVIGATE_VALUE_INCREASE: &str = ">";
pub const MODAL_BUTTON_NAVIGATE_WIDTH: f64 = 50.0 * FACTORF64;
pub const MODAL_BUTTON_NAVIGATE_PADDING_INNER: f64 = 20.0 * FACTORF64;
pub const MODAL_BUTTON_NAVIGATE_LEFT_ALIGN_TOP: f64 = -4.0 * FACTORF64;
pub const MODAL_GROUP_TABS_WIDTH: f64 = 130.0 * FACTORF64;
pub const MODAL_GROUP_TABS_HEIGHT: f64 = 40.0 * FACTORF64;
pub const MODAL_GROUP_FONT_SIZE: u32 = (14_f64 * FACTORF64) as u32;
pub const MODAL_GROUP_TABS_MARGIN_RIGHT: f64 = 38.0 * FACTORF64;
pub const MODAL_GROUP_TABS_MARGIN_TOP: f64 = 14.0 * FACTORF64;
pub const MODAL_GROUP_TABS_BORDER_RADIUS: f64 = 3.0 * FACTORF64;
pub const MODAL_FINALIZE_BUTTON_WIDTH: f64 = 110.0 * FACTORF64;
pub const MODAL_FINALIZE_BUTTON_HEIGHT: f64 = 30.0 * FACTORF64;
pub const MODAL_FINALIZE_BUTTON_MARGIN_RIGHT: f64 = 12.0 * FACTORF64;
pub const MODAL_FINALIZE_BUTTON_VALUE_TOP: f64 = MODAL_BUTTON_VALUE_TOP;
pub const MODAL_FINALIZE_BUTTON_FONT_SIZE: u32 = MODAL_BUTTON_FONT_SIZE;

pub const LAYOUT_HEADER_SIZE_HEIGHT: f64 = 65.0 * FACTORF64;
pub const LAYOUT_BODY_SIZE_HEIGHT: f64 = GRAPH_HEIGHT as _;
pub const LAYOUT_FOOTER_SIZE_HEIGHT: f64 = 90.0 * FACTORF64;

pub const LAYOUT_TEXTURE_HEADER_WIDTH: u32 = (606_f64 * FACTORF64) as u32;
pub const LAYOUT_TEXTURE_HEADER_HEIGHT: u32 = (52_f64 * FACTORF64) as u32;

#[cfg(feature = "lora")]
pub const LORA_GPIO_PIN_NUMBER: u64 = 25;
