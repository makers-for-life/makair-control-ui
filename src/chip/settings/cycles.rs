// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::time::Duration;
use telemetry::control::{ControlMessage, ControlSetting};

use crate::chip::settings::SettingAction;

const CYCLES_PER_MINUTE_MAX: usize = 35;
const CYCLES_PER_MINUTE_MIN: usize = 5;
const CYCLES_PER_MINUTE_STEP: usize = 1;

#[derive(Debug)]
pub enum SettingsCyclesEvent {
    CyclesPerMinute(SettingAction),
}

#[derive(Debug)]
pub struct SettingsCycles {
    pub cycles_per_minute: usize,
}

impl SettingsCycles {
    pub fn new() -> SettingsCycles {
        SettingsCycles {
            cycles_per_minute: 20,
        }
    }

    pub fn new_event(&self, event: SettingsCyclesEvent) -> ControlMessage {
        match event {
            SettingsCyclesEvent::CyclesPerMinute(action) => self.set_cycles_per_minute(action),
        }
    }

    fn set_cycles_per_minute(&self, action: SettingAction) -> ControlMessage {
        let new_value = match action {
            SettingAction::More => {
                let new_value = self.cycles_per_minute + CYCLES_PER_MINUTE_STEP;

                if new_value <= CYCLES_PER_MINUTE_MAX {
                    new_value
                } else {
                    self.cycles_per_minute
                }
            }
            SettingAction::Less => {
                let new_value = self.cycles_per_minute - CYCLES_PER_MINUTE_STEP;

                if new_value >= CYCLES_PER_MINUTE_MIN {
                    new_value
                } else {
                    self.cycles_per_minute
                }
            }
        };

        ControlMessage {
            setting: ControlSetting::CyclesPerMinute,
            value: new_value as u16,
        }
    }
}
