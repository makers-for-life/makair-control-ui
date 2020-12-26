// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};

use crate::chip::settings::SettingActionState;

#[derive(Debug)]
pub enum SettingsSnoozeEvent {
    AlarmSnooze,
}

#[derive(Debug)]
pub struct SettingsSnooze {
    pub alarms: SettingActionState,
}

impl SettingsSnooze {
    pub fn new() -> SettingsSnooze {
        SettingsSnooze {
            alarms: SettingActionState::from_value(ControlSetting::AlarmSnooze.default()),
        }
    }

    pub fn new_event(&self, event: SettingsSnoozeEvent) -> Vec<ControlMessage> {
        let event = match event {
            SettingsSnoozeEvent::AlarmSnooze => self.toggle_alarms(),
        };

        vec![event]
    }

    fn toggle_alarms(&self) -> ControlMessage {
        ControlMessage {
            setting: ControlSetting::AlarmSnooze,
            value: self.alarms.to_toggled() as u16,
        }
    }
}
