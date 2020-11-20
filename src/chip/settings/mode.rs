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
    pub inspiratory_time: usize,
    pub cycles_per_minute: usize,
    pub trigger_inspiratory_offset: usize,
    pub pressure_plateau: usize,
    pub pressure_peep: usize,
}

impl SettingsMode {
    pub fn new() -> SettingsMode {
        SettingsMode {
            mode: VentilationMode::default(),
            inspiratory_time: ControlSetting::ExpiratoryTerm.default(),
            cycles_per_minute: ControlSetting::CyclesPerMinute.default(),
            trigger_inspiratory_offset: ControlSetting::TriggerOffset.default(),
            pressure_plateau: ControlSetting::PlateauPressure.default(),
            pressure_peep: ControlSetting::PEEP.default(),
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
