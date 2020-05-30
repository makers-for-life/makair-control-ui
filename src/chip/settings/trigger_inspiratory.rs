use std::time::Duration;
use crate::chip::settings::SettingAction;

const INSPIRATORY_TRIGGER_OFFSET_MAX: usize = 100;
const INSPIRATORY_TRIGGER_OFFSET_STEP: usize = 1;

const PLATEAU_DURATION_MAX: Duration = Duration::from_millis(3000);
const PLATEAU_DURATION_STEP: Duration = Duration::from_millis(50);

#[derive(Debug)]
pub enum TriggerInspiratoryEvent {
    Toggle,
    InspiratoryTriggerOffset(SettingAction),
    PlateauDuration(SettingAction)
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
    pub plateau_duration: Duration
}

impl TriggerInspiratory {
    pub fn new() -> TriggerInspiratory {
        TriggerInspiratory {
            state: TriggerInspiratoryState::Disabled,
            inspiratory_trigger_offset: 20,
            plateau_duration: Duration::from_millis(1000)
        }
    }

    pub fn new_event(&mut self, event: TriggerInspiratoryEvent) {
        match event {
            TriggerInspiratoryEvent::Toggle => self.toggle(),
            TriggerInspiratoryEvent::InspiratoryTriggerOffset(action) => self.set_inspiratory_trigger_offset(action),
            TriggerInspiratoryEvent::PlateauDuration(action) => self.set_plateau_duration(action)
        };
    }

    fn toggle(&mut self) {
        self.state = match self.state {
            TriggerInspiratoryState::Enabled => TriggerInspiratoryState::Disabled,
            TriggerInspiratoryState::Disabled => TriggerInspiratoryState::Enabled
        };
    }

    fn set_inspiratory_trigger_offset(&mut self, action: SettingAction) {
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
        }
    }

    fn set_plateau_duration(&mut self, action: SettingAction) {
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
        }
    }
}