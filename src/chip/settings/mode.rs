// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::control::{ControlMessage, ControlSetting};
use telemetry::structures::VentilationMode;

use crate::chip::settings::SettingActionRange;

const INSPIRATORY_TIME_STEP: usize = 10;
const CYCLES_PER_MINUTE_STEP: usize = 1;
const TRIGGER_OFFSET_STEP: usize = 1;
const TRIGGER_FLOW_STEP: usize = 1;
const PRESSURE_STEP: usize = 10;
const FLOW_STEP: usize = 1;
const VOLUME_STEP: usize = 10;
const DURATION_STEP: usize = 10;

#[derive(Debug)]
pub enum SettingsModeEvent {
    ModePcCmv,
    ModePcAc,
    ModePcVsai,
    ModeVcCmv,
    ModeVcAc,
    InspiratoryTimeMinimum(SettingActionRange),
    InspiratoryTimeMaximum(SettingActionRange),
    CyclesPerMinute(SettingActionRange),
    TriggerInspiratoryOffset(SettingActionRange),
    TriggerExpiratoryFlow(SettingActionRange),
    PressurePlateau(SettingActionRange),
    PressureExpiratory(SettingActionRange),
    VolumeTidal(SettingActionRange),
    FlowInspiration(SettingActionRange),
    DurationInspiration(SettingActionRange),
    DurationPlateau(SettingActionRange),
    LowInspiratoryMinuteVolumeAlarm(SettingActionRange),
    HighInspiratoryMinuteVolumeAlarm(SettingActionRange),
    LowExpiratoryMinuteVolumeAlarm(SettingActionRange),
    HighExpiratoryMinuteVolumeAlarm(SettingActionRange),
    LowRespiratoryRateAlarm(SettingActionRange),
    HighRespiratoryRateAlarm(SettingActionRange),
    LowTidalVolumeAlarm(SettingActionRange),
    HighTidalVolumeAlarm(SettingActionRange),
    LeakAlarm(SettingActionRange),
}

#[derive(Debug)]
pub struct SettingsMode {
    // Group
    pub group: SettingsModeGroupTab,

    // Settings
    pub live: SettingsModeSettings,
    pub draft: Option<SettingsModeSettings>,
}

#[derive(Debug)]
pub struct SettingsModeSettings {
    // Mode
    pub mode: VentilationMode,

    // Commands
    pub inspiratory_time_minimum: usize,
    pub inspiratory_time_maximum: usize,
    pub cycles_per_minute: usize,
    pub trigger_inspiratory_offset: usize,
    pub trigger_expiratory_flow: usize,
    pub pressure_plateau: usize,
    pub pressure_expiratory: usize,
    pub volume_tidal: usize,
    pub flow_inspiration: usize,
    pub duration_inspiration: usize,
    pub duration_plateau: usize,

    // Alarm thresholds
    pub alarm_threshold_low_inspiratory_minute_volume: usize,
    pub alarm_threshold_high_inspiratory_minute_volume: usize,
    pub alarm_threshold_low_expiratory_minute_volume: usize,
    pub alarm_threshold_high_expiratory_minute_volume: usize,
    pub alarm_threshold_low_respiratory_rate: usize,
    pub alarm_threshold_high_respiratory_rate: usize,
    pub alarm_threshold_low_tidal_volume: usize,
    pub alarm_threshold_high_tidal_volume: usize,
    pub alarm_threshold_leak: usize,
}

#[derive(Debug, PartialEq)]
pub enum SettingsModeGroupTab {
    General,
    Alarms,
}

impl Default for SettingsModeSettings {
    fn default() -> Self {
        Self {
            mode: VentilationMode::default(),
            inspiratory_time_minimum: ControlSetting::TiMin.default(),
            inspiratory_time_maximum: ControlSetting::TiMax.default(),
            cycles_per_minute: ControlSetting::CyclesPerMinute.default(),
            trigger_inspiratory_offset: ControlSetting::TriggerOffset.default(),
            trigger_expiratory_flow: ControlSetting::ExpiratoryTriggerFlow.default(),
            pressure_plateau: ControlSetting::PlateauPressure.default(),
            pressure_expiratory: ControlSetting::PEEP.default(),
            volume_tidal: ControlSetting::TargetTidalVolume.default(),
            flow_inspiration: ControlSetting::TargetInspiratoryFlow.default(),
            duration_inspiration: ControlSetting::InspiratoryDuration.default(),
            duration_plateau: ControlSetting::PlateauDuration.default(),
            alarm_threshold_low_inspiratory_minute_volume:
                ControlSetting::LowInspiratoryMinuteVolumeAlarmThreshold.default(),
            alarm_threshold_high_inspiratory_minute_volume:
                ControlSetting::HighInspiratoryMinuteVolumeAlarmThreshold.default(),
            alarm_threshold_low_expiratory_minute_volume:
                ControlSetting::LowExpiratoryMinuteVolumeAlarmThreshold.default(),
            alarm_threshold_high_expiratory_minute_volume:
                ControlSetting::HighExpiratoryMinuteVolumeAlarmThreshold.default(),
            alarm_threshold_low_respiratory_rate: ControlSetting::LowRespiratoryRateAlarmThreshold
                .default(),
            alarm_threshold_high_respiratory_rate:
                ControlSetting::HighRespiratoryRateAlarmThreshold.default(),
            alarm_threshold_low_tidal_volume: ControlSetting::LowTidalVolumeAlarmThreshold
                .default(),
            alarm_threshold_high_tidal_volume: ControlSetting::HighTidalVolumeAlarmThreshold
                .default(),
            alarm_threshold_leak: ControlSetting::LeakAlarmThreshold.default(),
        }
    }
}

impl SettingsModeGroupTab {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::General),
            1 => Some(Self::Alarms),
            _ => None,
        }
    }
}

impl Default for SettingsModeGroupTab {
    fn default() -> Self {
        Self::General
    }
}

impl SettingsMode {
    pub fn new() -> SettingsMode {
        SettingsMode {
            group: SettingsModeGroupTab::default(),
            live: SettingsModeSettings::default(),
            draft: None,
        }
    }

    pub fn new_event(&self, event: SettingsModeEvent) -> ControlMessage {
        match event {
            SettingsModeEvent::ModePcCmv => self.switch_mode(VentilationMode::PC_CMV),
            SettingsModeEvent::ModePcAc => self.switch_mode(VentilationMode::PC_AC),
            SettingsModeEvent::ModePcVsai => self.switch_mode(VentilationMode::PC_VSAI),
            SettingsModeEvent::ModeVcCmv => self.switch_mode(VentilationMode::VC_CMV),
            SettingsModeEvent::ModeVcAc => self.switch_mode(VentilationMode::VC_AC),
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
            SettingsModeEvent::TriggerExpiratoryFlow(action) => {
                self.set_trigger_expiratory_flow(action)
            }
            SettingsModeEvent::PressurePlateau(action) => self.set_pressure_plateau(action),
            SettingsModeEvent::PressureExpiratory(action) => self.set_pressure_expiratory(action),
            SettingsModeEvent::VolumeTidal(action) => self.set_volume_tidal(action),
            SettingsModeEvent::FlowInspiration(action) => self.set_flow_inspiration(action),
            SettingsModeEvent::DurationInspiration(action) => self.set_duration_inspiration(action),
            SettingsModeEvent::DurationPlateau(action) => self.set_duration_plateau(action),
            SettingsModeEvent::LowInspiratoryMinuteVolumeAlarm(action) => {
                self.set_low_inspiratory_minute_volume_alarm(action)
            }
            SettingsModeEvent::HighInspiratoryMinuteVolumeAlarm(action) => {
                self.set_high_inspiratory_minute_volume_alarm(action)
            }
            SettingsModeEvent::LowExpiratoryMinuteVolumeAlarm(action) => {
                self.set_low_expiratory_minute_volume_alarm(action)
            }
            SettingsModeEvent::HighExpiratoryMinuteVolumeAlarm(action) => {
                self.set_high_expiratory_minute_volume_alarm(action)
            }
            SettingsModeEvent::LowRespiratoryRateAlarm(action) => {
                self.set_low_respiratory_rate_alarm(action)
            }
            SettingsModeEvent::HighRespiratoryRateAlarm(action) => {
                self.set_high_respiratory_rate_alarm(action)
            }
            SettingsModeEvent::LowTidalVolumeAlarm(action) => {
                self.set_low_tidal_volume_alarm(action)
            }
            SettingsModeEvent::HighTidalVolumeAlarm(action) => {
                self.set_high_tidal_volume_alarm(action)
            }
            SettingsModeEvent::LeakAlarm(action) => self.set_leak_alarm(action),
        }
    }

    fn switch_mode(&self, mode: VentilationMode) -> ControlMessage {
        ControlMessage {
            setting: ControlSetting::VentilationMode,
            value: u8::from(&mode) as _,
        }
    }

    fn set_inspiratory_time_minimum(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::TiMin,
            action,
            self.live.inspiratory_time_minimum,
            INSPIRATORY_TIME_STEP
        )
    }

    fn set_inspiratory_time_maximum(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::TiMax,
            action,
            self.live.inspiratory_time_maximum,
            INSPIRATORY_TIME_STEP
        )
    }

    fn set_cycles_per_minute(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::CyclesPerMinute,
            action,
            self.live.cycles_per_minute,
            CYCLES_PER_MINUTE_STEP
        )
    }

    fn set_trigger_inspiratory_offset(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::TriggerOffset,
            action,
            self.live.trigger_inspiratory_offset,
            TRIGGER_OFFSET_STEP
        )
    }

    fn set_trigger_expiratory_flow(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::ExpiratoryTriggerFlow,
            action,
            self.live.trigger_expiratory_flow,
            TRIGGER_FLOW_STEP
        )
    }

    fn set_pressure_plateau(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::PlateauPressure,
            action,
            self.live.pressure_plateau,
            PRESSURE_STEP
        )
    }

    fn set_pressure_expiratory(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::PEEP,
            action,
            self.live.pressure_expiratory,
            PRESSURE_STEP
        )
    }

    fn set_volume_tidal(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::TargetTidalVolume,
            action,
            self.live.volume_tidal,
            VOLUME_STEP
        )
    }

    fn set_flow_inspiration(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::TargetInspiratoryFlow,
            action,
            self.live.flow_inspiration,
            FLOW_STEP
        )
    }

    fn set_duration_inspiration(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::InspiratoryDuration,
            action,
            self.live.duration_inspiration,
            DURATION_STEP
        )
    }

    fn set_duration_plateau(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::PlateauDuration,
            action,
            self.live.duration_plateau,
            DURATION_STEP
        )
    }

    fn set_low_inspiratory_minute_volume_alarm(
        &self,
        action: SettingActionRange,
    ) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::LowInspiratoryMinuteVolumeAlarmThreshold,
            action,
            self.live.alarm_threshold_low_inspiratory_minute_volume,
            TRIGGER_FLOW_STEP
        )
    }

    fn set_high_inspiratory_minute_volume_alarm(
        &self,
        action: SettingActionRange,
    ) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::HighInspiratoryMinuteVolumeAlarmThreshold,
            action,
            self.live.alarm_threshold_high_inspiratory_minute_volume,
            TRIGGER_FLOW_STEP
        )
    }

    fn set_low_expiratory_minute_volume_alarm(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::LowExpiratoryMinuteVolumeAlarmThreshold,
            action,
            self.live.alarm_threshold_low_expiratory_minute_volume,
            TRIGGER_FLOW_STEP
        )
    }

    fn set_high_expiratory_minute_volume_alarm(
        &self,
        action: SettingActionRange,
    ) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::HighExpiratoryMinuteVolumeAlarmThreshold,
            action,
            self.live.alarm_threshold_high_expiratory_minute_volume,
            TRIGGER_FLOW_STEP
        )
    }

    fn set_low_respiratory_rate_alarm(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::LowRespiratoryRateAlarmThreshold,
            action,
            self.live.alarm_threshold_low_respiratory_rate,
            CYCLES_PER_MINUTE_STEP
        )
    }

    fn set_high_respiratory_rate_alarm(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::HighRespiratoryRateAlarmThreshold,
            action,
            self.live.alarm_threshold_high_respiratory_rate,
            CYCLES_PER_MINUTE_STEP
        )
    }

    fn set_low_tidal_volume_alarm(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::LowTidalVolumeAlarmThreshold,
            action,
            self.live.alarm_threshold_low_tidal_volume,
            VOLUME_STEP
        )
    }

    fn set_high_tidal_volume_alarm(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::HighTidalVolumeAlarmThreshold,
            action,
            self.live.alarm_threshold_high_tidal_volume,
            VOLUME_STEP
        )
    }

    fn set_leak_alarm(&self, action: SettingActionRange) -> ControlMessage {
        gen_set_new_value!(
            ControlSetting::LeakAlarmThreshold,
            action,
            self.live.alarm_threshold_leak,
            VOLUME_STEP
        )
    }
}
