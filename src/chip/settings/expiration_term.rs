// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::time::Duration;
use telemetry::control::{ControlMessage, ControlSetting};

use crate::chip::settings::SettingAction;

const EXPIRATORY_TERM_MAX: usize = 60;
const EXPIRATORY_TERM_MIN: usize = 10;
const EXPIRATORY_TERM_STEP: usize = 1;

#[derive(Debug)]
pub enum SettingsExpirationTermEvent {
    ExpiratoryTerm(SettingAction),
}

#[derive(Debug)]
pub struct SettingsExpirationTerm {
    pub plateau_duration: Duration,
    pub expiratory_term: usize,
    cycles_per_minute: usize,
}

impl SettingsExpirationTerm {
    pub fn new(cycles_per_minute: usize) -> SettingsExpirationTerm {
        SettingsExpirationTerm {
            plateau_duration: Duration::from_millis(1000),
            expiratory_term: 20,
            cycles_per_minute,
        }
    }

    pub fn new_event(&self, event: SettingsExpirationTermEvent) -> ControlMessage {
        match event {
            SettingsExpirationTermEvent::ExpiratoryTerm(action) => self.set_expiratory_term(action),
        }
    }

    pub fn get_plateau_duration(&self) -> Option<usize> {
        if self.cycles_per_minute > 0 {
            Some(
                (1000.0
                    * (10.0 / (10.0 + self.expiratory_term as f64)
                        * (60.0 / self.cycles_per_minute as f64))) as usize,
            )
        } else {
            None
        }
    }

    pub fn set_cycles_per_minute(&mut self, cycles_per_minute: usize) {
        self.cycles_per_minute = cycles_per_minute;
    }

    fn set_expiratory_term(&self, action: SettingAction) -> ControlMessage {
        let new_value = match action {
            SettingAction::More => {
                let new_value = self.expiratory_term + EXPIRATORY_TERM_STEP;
                if new_value <= EXPIRATORY_TERM_MAX {
                    new_value
                } else {
                    self.expiratory_term
                }
            }
            SettingAction::Less => {
                let new_value = self.expiratory_term - EXPIRATORY_TERM_STEP;
                if new_value >= EXPIRATORY_TERM_MIN {
                    new_value
                } else {
                    self.expiratory_term
                }
            }
        };

        ControlMessage {
            setting: ControlSetting::ExpiratoryTerm,
            value: new_value as u16,
        }
    }
}
