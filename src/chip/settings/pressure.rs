// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};

use crate::chip::settings::SettingAction;

const PRESSURE_STEP: usize = 1;

const PRESSURE_PEAK_MAX: usize = 700;
const PRESSURE_PEAK_MIN: usize = 0;

const PRESSURE_PLATEAU_MAX: usize = 400;
const PRESSURE_PLATEAU_MIN: usize = 100;

const PRESSURE_PEEP_MAX: usize = 300;
const PRESSURE_PEEP_MIN: usize = 0;

#[derive(Debug)]
pub enum SettingsPressureEvent {
    Peak(SettingAction),
    Plateau(SettingAction),
    PEEP(SettingAction),
}

#[derive(Debug)]
pub struct SettingsPressure {
    pub peak: usize,
    pub plateau: usize,
    pub peep: usize,
}

impl SettingsPressure {
    pub fn new() -> SettingsPressure {
        SettingsPressure {
            peak: 0,
            plateau: 0,
            peep: 0,
        }
    }

    pub fn new_event(&self, event: SettingsPressureEvent) -> ControlMessage {
        match event {
            SettingsPressureEvent::Peak(action) => self.set_peak(action),
            SettingsPressureEvent::Plateau(action) => self.set_plateau(action),
            SettingsPressureEvent::PEEP(action) => self.set_peep(action),
        }
    }

    fn set_peak(&self, action: SettingAction) -> ControlMessage {
        // TODO: call generic
        let new_value = 0;

        ControlMessage {
            setting: ControlSetting::PeakPressure,
            value: new_value as u16,
        }
    }

    fn set_plateau(&self, action: SettingAction) -> ControlMessage {
        // TODO: call generic
        let new_value = 0;

        ControlMessage {
            setting: ControlSetting::PlateauPressure,
            value: new_value as u16,
        }
    }

    fn set_peep(&self, action: SettingAction) -> ControlMessage {
        // TODO: call generic
        let new_value = 0;

        ControlMessage {
            setting: ControlSetting::PEEP,
            value: new_value as u16,
        }
    }
}
