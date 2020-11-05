// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};

use crate::chip::settings::{SettingActionRange, SettingActionState};

const TRIGGER_OFFSET_STEP: usize = 1;

#[derive(Debug)]
pub enum SettingsTriggerEvent {
    TriggerToggle,
    TriggerOffset(SettingActionRange),
}

#[derive(Debug)]
pub struct SettingsTrigger {
    pub state: SettingActionState,
    pub inspiratory_trigger_offset: usize,
}

impl SettingsTrigger {
    pub fn new() -> SettingsTrigger {
        SettingsTrigger {
            state: SettingActionState::from_value(ControlSetting::TriggerEnabled.default()),
            inspiratory_trigger_offset: ControlSetting::TriggerOffset.default(),
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
        ControlMessage {
            setting: ControlSetting::TriggerEnabled,
            value: self.state.to_toggled() as u16,
        }
    }

    fn set_inspiratory_trigger_offset(&self, action: SettingActionRange) -> ControlMessage {
        let setting = ControlSetting::TriggerOffset;

        let new_value = action.to_new_value(
            &setting,
            self.inspiratory_trigger_offset,
            TRIGGER_OFFSET_STEP,
        );

        ControlMessage {
            setting,
            value: new_value as u16,
        }
    }
}
