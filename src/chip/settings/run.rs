// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};

use crate::chip::settings::SettingActionState;

#[derive(Debug)]
pub enum SettingsRunEvent {
    RespirationEnabled,
}

#[derive(Debug)]
pub struct SettingsRun {
    pub state: SettingActionState,
}

impl SettingsRun {
    pub fn new() -> SettingsRun {
        SettingsRun {
            state: SettingActionState::from_value(ControlSetting::RespirationEnabled.default()),
        }
    }

    pub fn new_event(&self, event: SettingsRunEvent) -> ControlMessage {
        match event {
            SettingsRunEvent::RespirationEnabled => self.toggle_state(),
        }
    }

    fn toggle_state(&self) -> ControlMessage {
        ControlMessage {
            setting: ControlSetting::RespirationEnabled,
            value: self.state.to_toggled() as u16,
        }
    }
}
