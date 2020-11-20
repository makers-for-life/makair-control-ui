// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};
use telemetry::structures::VentilationMode;

#[derive(Debug)]
pub enum SettingsModeEvent {
    ModePcCmv,
    ModePcAc,
    ModePcBipap,
    ModeVcCmv,
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
            SettingsModeEvent::ModePcCmv => self.switch_mode(VentilationMode::PC_CMV),
            SettingsModeEvent::ModePcAc => self.switch_mode(VentilationMode::PC_AC),
            SettingsModeEvent::ModePcBipap => self.switch_mode(VentilationMode::PC_BIPAP),
            SettingsModeEvent::ModeVcCmv => self.switch_mode(VentilationMode::VC_CMV),
        }
    }

    fn switch_mode(&self, mode: VentilationMode) -> ControlMessage {
        ControlMessage {
            setting: ControlSetting::VentilationMode,
            value: u8::from(&mode) as _,
        }
    }
}
