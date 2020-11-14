// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use plotters_conrod::ConrodBackendReusableGraph;
use telemetry::structures::MachineStateSnapshot;

use crate::chip::{ChipDataFlow, ChipDataPressure, ChipError, ChipState};

pub struct DisplayDataLayout {
    pub texture_header_image_id: conrod_core::image::Id,
}

pub struct DisplayDataBranding<'a> {
    pub firmware_version: &'a str,
    pub width: f64,
    pub height: f64,
}

pub struct DisplayDataStatus<'a> {
    pub battery_level: Option<u8>,
    pub chip_state: &'a ChipState,
}

pub struct DisplayDataHeartbeat<'a> {
    pub data_pressure: &'a ChipDataPressure,
}

pub struct DisplayDataGraph<'a> {
    pub width: f64,
    pub height: f64,
    pub data_pressure: &'a ChipDataPressure,
    pub data_flow: &'a ChipDataFlow,
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
