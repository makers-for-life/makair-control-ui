// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};
use telemetry::structures::VentilationMode;

use crate::chip::settings::SettingActionRange;

const INSPIRATORY_TIME_STEP: usize = 1;
const CYCLES_PER_MINUTE_STEP: usize = 1;
const TRIGGER_OFFSET_STEP: usize = 1;
const TRIGGER_FLOW_STEP: usize = 1;
const PRESSURE_STEP: usize = 10;
const VOLUME_STEP: usize = 10;
const DURATION_STEP: usize = 10;

#[derive(Debug)]
pub enum SettingsModeEvent {
    ModePcCmv,
    ModePcAc,
    ModePcVsai,
    ModeVcCmv,
    ModeVcAc,
    InspiratoryTime(SettingActionRange),
    InspiratoryTimeMinimum(SettingActionRange),
    InspiratoryTimeMaximum(SettingActionRange),
    CyclesPerMinute(SettingActionRange),
    TriggerInspiratoryOffset(SettingActionRange),
    TriggerInspiratoryFlow(SettingActionRange),
    TriggerExpiratoryFlow(SettingActionRange),
    PressurePlateau(SettingActionRange),
    PressureExpiratory(SettingActionRange),
    VolumeTidal(SettingActionRange),
    DurationPlateau(SettingActionRange),
}

#[derive(Debug)]
pub struct SettingsMode {
    pub mode: VentilationMode,
    pub inspiratory_time: usize,
    pub inspiratory_time_minimum: usize,
    pub inspiratory_time_maximum: usize,
    pub cycles_per_minute: usize,
    pub trigger_inspiratory_offset: usize,
    pub trigger_inspiratory_flow: usize,
    pub trigger_expiratory_flow: usize,
    pub pressure_plateau: usize,
    pub pressure_expiratory: usize,
    pub volume_tidal: usize,
    pub duration_plateau: usize,
}

impl SettingsMode {
    pub fn new() -> SettingsMode {
        SettingsMode {
            mode: VentilationMode::default(),
            inspiratory_time: ControlSetting::ExpiratoryTerm.default(),
            inspiratory_time_minimum: ControlSetting::TiMin.default(),
            inspiratory_time_maximum: ControlSetting::TiMax.default(),
            cycles_per_minute: ControlSetting::CyclesPerMinute.default(),
            trigger_inspiratory_offset: ControlSetting::TriggerOffset.default(),
            trigger_inspiratory_flow: ControlSetting::InspiratoryTriggerFlow.default(),
            trigger_expiratory_flow: ControlSetting::ExpiratoryTriggerFlow.default(),
            pressure_plateau: ControlSetting::PlateauPressure.default(),
            pressure_expiratory: ControlSetting::PEEP.default(),
            volume_tidal: ControlSetting::TargetTidalVolume.default(),
            duration_plateau: ControlSetting::PlateauDuration.default(),
        }
    }

    pub fn new_event(&self, event: SettingsModeEvent) -> ControlMessage {
        match event {
            SettingsModeEvent::ModePcCmv => self.switch_mode(VentilationMode::PC_CMV),
            SettingsModeEvent::ModePcAc => self.switch_mode(VentilationMode::PC_AC),
            SettingsModeEvent::ModePcVsai => self.switch_mode(VentilationMode::PC_VSAI),
            SettingsModeEvent::ModeVcCmv => self.switch_mode(VentilationMode::VC_CMV),
            SettingsModeEvent::ModeVcAc => self.switch_mode(VentilationMode::VC_AC),
            SettingsModeEvent::InspiratoryTime(action) => self.set_inspiratory_time(action),
            SettingsModeEvent::InspiratoryTimeMinimum(action) => {
                self.set_inspiratory_time_minimum(action)
            }
            SettingsModeEvent::InspiratoryTimeMaximum(action) => {
                self.set_inspiratory_time_maximum(action)
            }
            SettingsModeEvent::CyclesPerMinute(action) => self.set_cycles_per_minute(action),
            SettingsModeEvent::TriggerInspiratoryOffset(action) => {
                self.set_trigger_inspiratory_offset(action)
            }
            SettingsModeEvent::TriggerInspiratoryFlow(action) => {
                self.set_trigger_inspiratory_flow(action)
            }
            SettingsModeEvent::TriggerExpiratoryFlow(action) => {
                self.set_trigger_expiratory_flow(action)
            }
            SettingsModeEvent::PressurePlateau(action) => self.set_pressure_plateau(action),
            SettingsModeEvent::PressureExpiratory(action) => self.set_pressure_expiratory(action),
            SettingsModeEvent::VolumeTidal(action) => self.set_volume_tidal(action),
            SettingsModeEvent::DurationPlateau(action) => self.set_duration_plateau(action),
        }
    }

    fn switch_mode(&self, mode: VentilationMode) -> ControlMessage {
        ControlMessage {
            setting: ControlSetting::VentilationMode,
            value: u8::from(&mode) as _,
        }
    }

    fn set_inspiratory_time(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::ExpiratoryTerm,
            action,
            self.inspiratory_time,
            INSPIRATORY_TIME_STEP
        )
    }

    fn set_inspiratory_time_minimum(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::TiMin,
            action,
            self.inspiratory_time_minimum,
            INSPIRATORY_TIME_STEP
        )
    }

    fn set_inspiratory_time_maximum(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::TiMax,
            action,
            self.inspiratory_time_maximum,
            INSPIRATORY_TIME_STEP
        )
    }

    fn set_cycles_per_minute(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::CyclesPerMinute,
            action,
            self.cycles_per_minute,
            CYCLES_PER_MINUTE_STEP
        )
    }

    fn set_trigger_inspiratory_offset(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::TriggerOffset,
            action,
            self.trigger_inspiratory_offset,
            TRIGGER_OFFSET_STEP
        )
    }

    fn set_trigger_inspiratory_flow(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::InspiratoryTriggerFlow,
            action,
            self.trigger_inspiratory_flow,
            TRIGGER_FLOW_STEP
        )
    }

    fn set_trigger_expiratory_flow(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::ExpiratoryTriggerFlow,
            action,
            self.trigger_expiratory_flow,
            TRIGGER_FLOW_STEP
        )
    }

    fn set_pressure_plateau(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::PlateauPressure,
            action,
            self.pressure_plateau,
            PRESSURE_STEP
        )
    }

    fn set_pressure_expiratory(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::PEEP,
            action,
            self.pressure_expiratory,
            PRESSURE_STEP
        )
    }

    fn set_volume_tidal(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::TargetTidalVolume,
            action,
            self.volume_tidal,
            VOLUME_STEP
        )
    }

    fn set_duration_plateau(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::PlateauDuration,
            action,
            self.duration_plateau,
            DURATION_STEP
        )
    }
}
