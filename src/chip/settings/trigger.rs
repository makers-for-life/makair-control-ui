// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};

use crate::chip::settings::SettingAction;

const TRIGGER_OFFSET_STEP: usize = 1;

#[derive(Debug)]
pub enum SettingsTriggerEvent {
    TriggerToggle,
    TriggerOffset(SettingAction),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingsTriggerState {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Debug)]
pub struct SettingsTrigger {
    pub state: SettingsTriggerState,
    pub inspiratory_trigger_offset: usize,
}

impl SettingsTrigger {
    pub fn new() -> SettingsTrigger {
        SettingsTrigger {
            state: SettingsTriggerState::Disabled,
            inspiratory_trigger_offset: 20,
        }
    }

    pub fn new_event(&self, event: SettingsTriggerEvent) -> ControlMessage {
        match event {
            SettingsTriggerEvent::TriggerToggle => self.toggle_enabled(),
            SettingsTriggerEvent::TriggerOffset(action) => {
                self.set_inspiratory_trigger_offset(action)
            }
        }
    }

    fn toggle_enabled(&self) -> ControlMessage {
        let new_state = match self.state {
            SettingsTriggerState::Enabled => SettingsTriggerState::Disabled,
            SettingsTriggerState::Disabled => SettingsTriggerState::Enabled,
        };

        ControlMessage {
            setting: ControlSetting::TriggerEnabled,
            value: new_state as u16,
        }
    }

    fn set_inspiratory_trigger_offset(&self, action: SettingAction) -> ControlMessage {
        let setting = ControlSetting::TriggerOffset;

        let new_value = match action {
            SettingAction::More => {
                let new_value = self.inspiratory_trigger_offset + TRIGGER_OFFSET_STEP;

                if setting.bounds().contains(&new_value) {
                    new_value
                } else {
                    self.inspiratory_trigger_offset
                }
            }
            SettingAction::Less => {
                if self.inspiratory_trigger_offset >= TRIGGER_OFFSET_STEP {
                    let new_value = self.inspiratory_trigger_offset - TRIGGER_OFFSET_STEP;

                    if setting.bounds().contains(&new_value) {
                        new_value
                    } else {
                        self.inspiratory_trigger_offset
                    }
                } else {
                    self.inspiratory_trigger_offset
                }
            }
        };

        ControlMessage {
            setting: setting,
            value: new_value as u16,
        }
    }
}
