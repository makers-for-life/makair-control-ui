// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};

use crate::chip::settings::SettingAction;

const EXPIRATORY_TERM_STEP: usize = 1;

#[derive(Debug)]
pub enum SettingsExpirationTermEvent {
    ExpiratoryTerm(SettingAction),
}

#[derive(Debug)]
pub struct SettingsExpirationTerm {
    pub expiratory_term: usize,
    pub cycles_per_minute: usize,
}

impl SettingsExpirationTerm {
    pub fn new(cycles_per_minute: usize) -> SettingsExpirationTerm {
        SettingsExpirationTerm {
            expiratory_term: ControlSetting::ExpiratoryTerm.default(),
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

    fn set_expiratory_term(&self, action: SettingAction) -> ControlMessage {
        let setting = ControlSetting::ExpiratoryTerm;
        let new_value = action.to_new_value(&setting, self.expiratory_term, EXPIRATORY_TERM_STEP);

        ControlMessage {
            setting,
            value: new_value as u16,
        }
    }
}
