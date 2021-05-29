// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use makair_telemetry::structures::MachineStateSnapshot;
use plotters_conrod::ConrodBackendReusableGraph;

use crate::chip::{ChipData, ChipEndOfLine, ChipError, ChipState};

use super::identifiers::ImageIds;

pub struct DisplayDataLayout {
    pub texture_header_image_id: conrod_core::image::Id,
}

pub struct DisplayDataBranding<'a> {
    pub firmware_version: &'a str,
    pub width: f64,
    pub height: f64,
}

pub struct DisplayDataStatus<'a> {
    pub chip_state: &'a ChipState,
    pub battery_soc: Option<u8>,
}

pub struct DisplayDataHeartbeat<'a> {
    pub data_pressure: &'a ChipData,
}

pub struct DisplayDataGraph<'a> {
    pub width: f64,
    pub height: f64,
    pub data_pressure: &'a ChipData,
    pub data_flow: &'a ChipData,
    pub chip_state: &'a ChipState,
    pub machine_snapshot: &'a MachineStateSnapshot,
    pub plot_graphs: &'a mut (ConrodBackendReusableGraph, ConrodBackendReusableGraph),
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

pub struct DisplayDataEndOfLine<'a> {
    pub error: bool,
    pub success: bool,
    pub step: u8,
    pub icon_image_id: conrod_core::image::Id,
    pub eol: &'a ChipEndOfLine,
}

pub struct DisplayDataSettings<'a> {
    pub images: &'a ImageIds,
}
