// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};

use crate::chip::settings::SettingActionRange;

const PRESSURE_STEP: usize = 10;

#[derive(Debug)]
pub enum SettingsPressureEvent {
    Peak(SettingActionRange),
    Plateau(SettingActionRange),
    PEEP(SettingActionRange),
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
            peak: ControlSetting::PeakPressure.default(),
            plateau: ControlSetting::PlateauPressure.default(),
            peep: ControlSetting::PEEP.default(),
        }
    }

    pub fn new_event(&self, event: SettingsPressureEvent) -> ControlMessage {
        match event {
            SettingsPressureEvent::Peak(action) => self.set_peak(action),
            SettingsPressureEvent::Plateau(action) => self.set_plateau(action),
            SettingsPressureEvent::PEEP(action) => self.set_peep(action),
        }
    }

    fn set_peak(&self, action: SettingActionRange) -> ControlMessage {
        let control = ControlSetting::PeakPressure;

        ControlMessage {
            setting: control,
            value: self.acquire_new_value(&control, action, self.peak) as u16,
        }
    }

    fn set_plateau(&self, action: SettingActionRange) -> ControlMessage {
        let control = ControlSetting::PlateauPressure;

        ControlMessage {
            setting: control,
            value: self.acquire_new_value(&control, action, self.plateau) as u16,
        }
    }

    fn set_peep(&self, action: SettingActionRange) -> ControlMessage {
        let control = ControlSetting::PEEP;

        ControlMessage {
            setting: control,
            value: self.acquire_new_value(&control, action, self.peep) as u16,
        }
    }

    fn acquire_new_value(
        &self,
        control: &ControlSetting,
        action: SettingActionRange,
        previous_value: usize,
    ) -> usize {
        action.to_new_value(control, previous_value, PRESSURE_STEP)
    }
}
