use std::time::Duration;
use telemetry::control::{ControlMessage, ControlSetting};
use crate::chip::settings::SettingAction;

const INSPIRATORY_TRIGGER_OFFSET_MAX: usize = 100;
const INSPIRATORY_TRIGGER_OFFSET_STEP: usize = 1;

const EXPIRATORY_TERM_MAX: usize = 60;
const EXPIRATORY_TERM_MIN: usize = 10;
const EXPIRATORY_TERM_STEP: usize = 1;

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
    Enabled,
    Disabled
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

    pub fn new_event(&mut self, event: TriggerInspiratoryEvent) -> ControlMessage {
        match event {
            TriggerInspiratoryEvent::Toggle => self.toggle(),
            TriggerInspiratoryEvent::InspiratoryTriggerOffset(action) => self.set_inspiratory_trigger_offset(action),
            //TriggerInspiratoryEvent::PlateauDuration(action) => self.set_plateau_duration(action),
            TriggerInspiratoryEvent::ExpiratoryTerm(action) => self.set_expiratory_term(action),
        }
    }

    pub fn set_cycles_per_minute(&mut self, cycles_per_minute: usize) {
        self.cycles_per_minute = cycles_per_minute;
    }

    fn toggle(&mut self) -> ControlMessage {
        self.state = match self.state {
            TriggerInspiratoryState::Enabled => TriggerInspiratoryState::Disabled,
            TriggerInspiratoryState::Disabled => TriggerInspiratoryState::Enabled
        };

        ControlMessage {
            setting: ControlSetting::TriggerEnabled,
            value: self.state as u16
        }
    }

    fn set_inspiratory_trigger_offset(&mut self, action: SettingAction) -> ControlMessage {
        match action {
            SettingAction::More => {
                let new_value = self.inspiratory_trigger_offset + INSPIRATORY_TRIGGER_OFFSET_STEP;
                if new_value <= INSPIRATORY_TRIGGER_OFFSET_MAX {
                    self.inspiratory_trigger_offset = new_value;
                }
            },
            SettingAction::Less => {
                if self.inspiratory_trigger_offset != 0 {
                    self.inspiratory_trigger_offset -= INSPIRATORY_TRIGGER_OFFSET_STEP;
                }
            }
        };

        ControlMessage {
            setting: ControlSetting::TriggerOffset,
            value: self.inspiratory_trigger_offset as u16
        }
    }

    fn set_plateau_duration(&mut self, action: SettingAction) -> ControlMessage {
        match action {
            SettingAction::More => {
                let new_value = self.plateau_duration + PLATEAU_DURATION_STEP;
                if new_value <= PLATEAU_DURATION_MAX {
                    self.plateau_duration = new_value;
                }
            },
            SettingAction::Less => {
                if self.plateau_duration != Duration::from_millis(0) {
                    self.plateau_duration -= PLATEAU_DURATION_STEP;
                }
            }
        };

        ControlMessage {
            setting: ControlSetting::PEEP,
            value: self.plateau_duration.as_millis() as u16
        }
    }

    fn set_expiratory_term(&mut self, action: SettingAction) -> ControlMessage {
        match action {
            SettingAction::More => {
                let new_value = self.expiratory_term + EXPIRATORY_TERM_STEP;
                if new_value <= EXPIRATORY_TERM_MAX {
                    self.expiratory_term = new_value;
                }
            },
            SettingAction::Less => {
                let new_value = self.expiratory_term - EXPIRATORY_TERM_STEP;
                if new_value >= EXPIRATORY_TERM_MIN {
                    self.expiratory_term = new_value;
                }
            }
        };

        ControlMessage {
            setting: ControlSetting::ExpiratoryTerm,
            value: self.expiratory_term as u16,
        }
    }

    pub fn get_plateau_duration(&self) -> usize {
        if self.expiratory_term > 0 && self.cycles_per_minute > 0 {
            1000 * (10 / (10 + self.expiratory_term) * (60 / self.cycles_per_minute))
        } else {
            0
        }
    }
}