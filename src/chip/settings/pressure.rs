// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};

use crate::chip::settings::SettingAction;

const PRESSURE_STEP: usize = 10;

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
        ControlMessage {
            setting: ControlSetting::PeakPressure,
            value: self.acquire_new_value(action, self.peak, PRESSURE_PEAK_MAX, PRESSURE_PEAK_MIN)
                as u16,
        }
    }

    fn set_plateau(&self, action: SettingAction) -> ControlMessage {
        ControlMessage {
            setting: ControlSetting::PlateauPressure,
            value: self.acquire_new_value(
                action,
                self.plateau,
                PRESSURE_PLATEAU_MAX,
                PRESSURE_PLATEAU_MIN,
            ) as u16,
        }
    }

    fn set_peep(&self, action: SettingAction) -> ControlMessage {
        ControlMessage {
            setting: ControlSetting::PEEP,
            value: self.acquire_new_value(action, self.peep, PRESSURE_PEEP_MAX, PRESSURE_PEEP_MIN)
                as u16,
        }
    }

    fn acquire_new_value(
        &self,
        action: SettingAction,
        previous_value: usize,
        maximum: usize,
        minimum: usize,
    ) -> usize {
        match action {
            SettingAction::More => {
                let new_value = previous_value + PRESSURE_STEP;

                if new_value <= maximum {
                    new_value
                } else {
                    previous_value
                }
            }

            SettingAction::Less => {
                if previous_value >= PRESSURE_STEP {
                    let new_value = previous_value - PRESSURE_STEP;

                    if new_value >= minimum {
                        new_value
                    } else {
                        previous_value
                    }
                } else {
                    previous_value
                }
            }
        }
    }
}
