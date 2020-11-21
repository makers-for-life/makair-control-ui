// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};

use crate::chip::settings::SettingActionRange;

const CYCLES_PER_MINUTE_STEP: usize = 1;

#[derive(Debug)]
pub enum SettingsCyclesEvent {
    CyclesPerMinute(SettingActionRange),
}

#[derive(Debug)]
pub struct SettingsCycles {
    pub cycles_per_minute: usize,
}

impl SettingsCycles {
    pub fn new() -> SettingsCycles {
        SettingsCycles {
            cycles_per_minute: ControlSetting::CyclesPerMinute.default(),
        }
    }

    pub fn new_event(&self, event: SettingsCyclesEvent) -> ControlMessage {
        match event {
            SettingsCyclesEvent::CyclesPerMinute(action) => self.set_cycles_per_minute(action),
        }
    }

    fn set_cycles_per_minute(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::CyclesPerMinute,
            action,
            self.cycles_per_minute,
            CYCLES_PER_MINUTE_STEP
        )
    }
}
