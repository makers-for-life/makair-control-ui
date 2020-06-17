use crate::chip::settings::SettingAction;
use std::time::Duration;
use telemetry::control::{ControlMessage, ControlSetting};

const INSPIRATORY_TRIGGER_OFFSET_MAX: usize = 100;
const INSPIRATORY_TRIGGER_OFFSET_STEP: usize = 1;

const EXPIRATORY_TERM_MAX: usize = 60;
const EXPIRATORY_TERM_MIN: usize = 10;
const EXPIRATORY_TERM_STEP: usize = 1;

#[allow(dead_code)]
const PLATEAU_DURATION_MAX: Duration = Duration::from_millis(3000);
const PLATEAU_DURATION_STEP: Duration = Duration::from_millis(50);

#[derive(Debug)]
pub enum TriggerInspiratoryEvent {
    Toggle,
    InspiratoryTriggerOffset(SettingAction),
    //PlateauDuration(SettingAction),
    ExpiratoryTerm(SettingAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TriggerInspiratoryState {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Debug)]
pub struct TriggerInspiratory {
    pub state: TriggerInspiratoryState,
    pub inspiratory_trigger_offset: usize,
    pub plateau_duration: Duration,
    pub expiratory_term: usize,
    cycles_per_minute: usize,
}

impl TriggerInspiratory {
    pub fn new(cycles_per_minute: usize) -> TriggerInspiratory {
        TriggerInspiratory {
            state: TriggerInspiratoryState::Disabled,
            inspiratory_trigger_offset: 20,
            plateau_duration: Duration::from_millis(1000),
            expiratory_term: 20,
            cycles_per_minute,
        }
    }

    pub fn new_event(&self, event: TriggerInspiratoryEvent) -> ControlMessage {
        match event {
            TriggerInspiratoryEvent::Toggle => self.toggle(),
            TriggerInspiratoryEvent::InspiratoryTriggerOffset(action) => {
                self.set_inspiratory_trigger_offset(action)
            }
            //TriggerInspiratoryEvent::PlateauDuration(action) => self.set_plateau_duration(action),
            TriggerInspiratoryEvent::ExpiratoryTerm(action) => self.set_expiratory_term(action),
        }
    }

    pub fn set_cycles_per_minute(&mut self, cycles_per_minute: usize) {
        self.cycles_per_minute = cycles_per_minute;
    }

    fn toggle(&self) -> ControlMessage {
        let new_state = match self.state {
            TriggerInspiratoryState::Enabled => TriggerInspiratoryState::Disabled,
            TriggerInspiratoryState::Disabled => TriggerInspiratoryState::Enabled,
        };

        ControlMessage {
            setting: ControlSetting::TriggerEnabled,
            value: new_state as u16,
        }
    }

    fn set_inspiratory_trigger_offset(&self, action: SettingAction) -> ControlMessage {
        let new_value = match action {
            SettingAction::More => {
                let new_value = self.inspiratory_trigger_offset + INSPIRATORY_TRIGGER_OFFSET_STEP;
                if new_value <= INSPIRATORY_TRIGGER_OFFSET_MAX {
                    new_value
                } else {
                    self.inspiratory_trigger_offset
                }
            }
            SettingAction::Less => {
                if self.inspiratory_trigger_offset != 0 {
                    self.inspiratory_trigger_offset - INSPIRATORY_TRIGGER_OFFSET_STEP
                } else {
                    self.inspiratory_trigger_offset
                }
            }
        };

        ControlMessage {
            setting: ControlSetting::TriggerOffset,
            value: new_value as u16,
        }
    }

    #[allow(dead_code, unreachable_code, unused_variables)]
    fn set_plateau_duration(&self, action: SettingAction) -> ControlMessage {
        unimplemented!("The ControlMessage for this setting isn't implemented");

        let new_value = match action {
            SettingAction::More => {
                let new_value = self.plateau_duration + PLATEAU_DURATION_STEP;
                if new_value <= PLATEAU_DURATION_MAX {
                    new_value
                } else {
                    self.plateau_duration
                }
            }
            SettingAction::Less => {
                if self.plateau_duration != Duration::from_millis(0) {
                    self.plateau_duration - PLATEAU_DURATION_STEP
                } else {
                    self.plateau_duration
                }
            }
        };
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

    pub fn get_plateau_duration(&self) -> usize {
        if self.cycles_per_minute > 0 {
            (1000.0
                * (10.0 / (10.0 + self.expiratory_term as f64)
                    * (60.0 / self.cycles_per_minute as f64))) as usize
        } else {
            0
        }
    }
}
