// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use crate::chip::{settings::ChipSettings, ChipDataPressure, ChipError, ChipState};

pub struct DisplayDataBranding<'a> {
    pub firmware_version: &'a str,
    pub image_id: conrod_core::image::Id,
    pub width: f64,
    pub height: f64,
}

pub struct DisplayDataControls<'a> {
    pub run_image_id: conrod_core::image::Id,
    pub snooze_inactive_image_id: conrod_core::image::Id,
    pub snooze_active_image_id: conrod_core::image::Id,
    pub advanced_image_id: conrod_core::image::Id,
    pub chip_state: &'a ChipState,
    pub chip_settings: &'a ChipSettings,
}

pub struct DisplayDataStatus<'a> {
    pub battery_level: Option<u8>,
    pub chip_state: &'a ChipState,
    pub save_image_id: Option<conrod_core::image::Id>,
}

pub struct DisplayDataHeartbeat<'a> {
    pub data_pressure: &'a ChipDataPressure,
}

pub struct DisplayDataGraph {
    pub image_id: conrod_core::image::Id,
    pub width: f64,
    pub height: f64,
}

pub struct DisplayDataTelemetry {
    pub arrow_image_id: conrod_core::image::Id,
}

pub struct DisplayDataBootloader {
    pub image_id: conrod_core::image::Id,
    pub width: f64,
    pub height: f64,
    pub connecting: bool,
}

pub struct DisplayDataError<'a> {
    pub image_id: conrod_core::image::Id,
    pub width: f64,
    pub height: f64,
    pub error: &'a ChipError,
}
