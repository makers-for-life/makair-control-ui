// MakAir Control UI
//
// Copyright: 2021, Makers For Life
// License: Public Domain License

use makair_telemetry::control::{ControlMessage, ControlSetting};

#[derive(Debug)]
pub enum SettingsEndOfLineEvent {
    Confirm,
}

#[derive(Debug)]
pub struct SettingsEndOfLine;

impl SettingsEndOfLine {
    pub fn new() -> SettingsEndOfLine {
        SettingsEndOfLine {}
    }

    pub fn new_event(&self, event: SettingsEndOfLineEvent) -> Vec<ControlMessage> {
        let event = match event {
            SettingsEndOfLineEvent::Confirm => self.confirm(),
        };

        vec![event]
    }

    fn confirm(&self) -> ControlMessage {
        ControlMessage {
            setting: ControlSetting::EolConfirm,
            value: 0,
        }
    }
}
