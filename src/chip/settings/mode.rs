// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};
use telemetry::structures::VentilationMode;

#[derive(Debug)]
pub enum SettingsModeEvent {
    Mode,
}

#[derive(Debug)]
pub struct SettingsMode {
    pub mode: VentilationMode,
}

impl SettingsMode {
    pub fn new() -> SettingsMode {
        SettingsMode {
            mode: VentilationMode::default(),
        }
    }

    pub fn new_event(&self, event: SettingsModeEvent) -> ControlMessage {
        match event {
            SettingsModeEvent::Mode => self.toggle_mode(),
        }
    }

    fn toggle_mode(&self) -> ControlMessage {
        ControlMessage {
            setting: ControlSetting::VentilationMode,
            value: 0, // TODO
        }
    }
}
