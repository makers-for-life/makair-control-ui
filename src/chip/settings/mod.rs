// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub mod cycles;
pub mod expiration_term;
pub mod pressure;
pub mod run;
pub mod trigger;

use telemetry::control::{ControlMessage, ControlSetting};

use cycles::*;
use expiration_term::*;
use pressure::*;
use run::*;
use trigger::*;

#[derive(Debug)]
pub enum ChipSettingsEvent {
    Run(SettingsRunEvent),
    Trigger(SettingsTriggerEvent),
    ExpirationTerm(SettingsExpirationTermEvent),
    Cycles(SettingsCyclesEvent),
    Pressure(SettingsPressureEvent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingActionState {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Debug)]
pub enum SettingActionRange {
    More,
    Less,
}

impl SettingActionState {
    fn from_value(value: usize) -> Self {
        if value > 0 {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }

    fn to_toggled(&self) -> Self {
        match self {
            Self::Enabled => Self::Disabled,
            Self::Disabled => Self::Enabled,
        }
    }
}

impl SettingActionRange {
    fn to_new_value(&self, setting: &ControlSetting, value: usize, step: usize) -> usize {
        match self {
            SettingActionRange::More => {
                let new_value = value + step;

                if setting.bounds().contains(&new_value) {
                    new_value
                } else {
                    value
                }
            }
            SettingActionRange::Less => {
                if value >= step {
                    let new_value = value - step;

                    if setting.bounds().contains(&new_value) {
                        new_value
                    } else {
                        value
                    }
                } else {
                    value
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ChipSettings {
    pub run: SettingsRun,
    pub trigger: SettingsTrigger,
    pub expiration_term: SettingsExpirationTerm,
    pub cycles: SettingsCycles,
    pub pressure: SettingsPressure,
}

impl ChipSettings {
    pub fn new(cycles_per_minute: usize) -> ChipSettings {
        ChipSettings {
            run: SettingsRun::new(),
            trigger: SettingsTrigger::new(),
            expiration_term: SettingsExpirationTerm::new(cycles_per_minute),
            cycles: SettingsCycles::new(),
            pressure: SettingsPressure::new(),
        }
    }

    pub fn new_settings_event(&mut self, event: ChipSettingsEvent) -> ControlMessage {
        match event {
            ChipSettingsEvent::Run(event) => self.run.new_event(event),
            ChipSettingsEvent::Trigger(event) => self.trigger.new_event(event),
            ChipSettingsEvent::ExpirationTerm(event) => self.expiration_term.new_event(event),
            ChipSettingsEvent::Cycles(event) => self.cycles.new_event(event),
            ChipSettingsEvent::Pressure(event) => self.pressure.new_event(event),
        }
    }
}
