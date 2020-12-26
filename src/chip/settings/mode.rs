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
    Commit,
}

#[derive(Debug)]
pub enum SettingsModeIntent {
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

#[derive(Debug, Clone)]
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

    pub fn new_event(&mut self, event: SettingsModeEvent) -> Vec<ControlMessage> {
        match event {
            SettingsModeEvent::Commit => self.commit(),
        }
    }

    pub fn new_intent(&mut self, intent: SettingsModeIntent) {
        match intent {
            SettingsModeIntent::ModePcCmv => self.switch_mode(VentilationMode::PC_CMV),
            SettingsModeIntent::ModePcAc => self.switch_mode(VentilationMode::PC_AC),
            SettingsModeIntent::ModePcVsai => self.switch_mode(VentilationMode::PC_VSAI),
            SettingsModeIntent::ModeVcCmv => self.switch_mode(VentilationMode::VC_CMV),
            SettingsModeIntent::ModeVcAc => self.switch_mode(VentilationMode::VC_AC),
            SettingsModeIntent::InspiratoryTimeMinimum(action) => {
                self.set_inspiratory_time_minimum(action)
            }
            SettingsModeIntent::InspiratoryTimeMaximum(action) => {
                self.set_inspiratory_time_maximum(action)
            }
            SettingsModeIntent::CyclesPerMinute(action) => self.set_cycles_per_minute(action),
            SettingsModeIntent::TriggerInspiratoryOffset(action) => {
                self.set_trigger_inspiratory_offset(action)
            }
            SettingsModeIntent::TriggerExpiratoryFlow(action) => {
                self.set_trigger_expiratory_flow(action)
            }
            SettingsModeIntent::PressurePlateau(action) => self.set_pressure_plateau(action),
            SettingsModeIntent::PressureExpiratory(action) => self.set_pressure_expiratory(action),
            SettingsModeIntent::VolumeTidal(action) => self.set_volume_tidal(action),
            SettingsModeIntent::FlowInspiration(action) => self.set_flow_inspiration(action),
            SettingsModeIntent::DurationInspiration(action) => {
                self.set_duration_inspiration(action)
            }
            SettingsModeIntent::DurationPlateau(action) => self.set_duration_plateau(action),
            SettingsModeIntent::LowInspiratoryMinuteVolumeAlarm(action) => {
                self.set_low_inspiratory_minute_volume_alarm(action)
            }
            SettingsModeIntent::HighInspiratoryMinuteVolumeAlarm(action) => {
                self.set_high_inspiratory_minute_volume_alarm(action)
            }
            SettingsModeIntent::LowExpiratoryMinuteVolumeAlarm(action) => {
                self.set_low_expiratory_minute_volume_alarm(action)
            }
            SettingsModeIntent::HighExpiratoryMinuteVolumeAlarm(action) => {
                self.set_high_expiratory_minute_volume_alarm(action)
            }
            SettingsModeIntent::LowRespiratoryRateAlarm(action) => {
                self.set_low_respiratory_rate_alarm(action)
            }
            SettingsModeIntent::HighRespiratoryRateAlarm(action) => {
                self.set_high_respiratory_rate_alarm(action)
            }
            SettingsModeIntent::LowTidalVolumeAlarm(action) => {
                self.set_low_tidal_volume_alarm(action)
            }
            SettingsModeIntent::HighTidalVolumeAlarm(action) => {
                self.set_high_tidal_volume_alarm(action)
            }
            SettingsModeIntent::LeakAlarm(action) => self.set_leak_alarm(action),
        }
    }

    pub fn commit(&mut self) -> Vec<ControlMessage> {
        let mut events = Vec::new();

        // Generate events from changed draft values?
        if let Some(ref draft) = self.draft {
            // Append non-numeric mode value
            if draft.mode != self.live.mode {
                events.push(ControlMessage {
                    setting: ControlSetting::VentilationMode,
                    value: u8::from(&draft.mode) as _,
                });
            }

            // Append all other numeric values
            gen_commit_mode_events_numeric!(
                self, draft, events, {
                    TiMin -> inspiratory_time_minimum,
                    TiMax -> inspiratory_time_maximum,
                    CyclesPerMinute -> cycles_per_minute,
                    TriggerOffset -> trigger_inspiratory_offset,
                    ExpiratoryTriggerFlow -> trigger_expiratory_flow,
                    PlateauPressure -> pressure_plateau,
                    PEEP -> pressure_expiratory,
                    TargetTidalVolume -> volume_tidal,
                    TargetInspiratoryFlow -> flow_inspiration,
                    InspiratoryDuration -> duration_inspiration,
                    PlateauDuration -> duration_plateau,
                    LowInspiratoryMinuteVolumeAlarmThreshold ->
                        alarm_threshold_low_inspiratory_minute_volume,
                    HighInspiratoryMinuteVolumeAlarmThreshold ->
                        alarm_threshold_high_inspiratory_minute_volume,
                    LowExpiratoryMinuteVolumeAlarmThreshold ->
                        alarm_threshold_low_expiratory_minute_volume,
                    HighExpiratoryMinuteVolumeAlarmThreshold ->
                        alarm_threshold_high_expiratory_minute_volume,
                    LowRespiratoryRateAlarmThreshold -> alarm_threshold_low_respiratory_rate,
                    HighRespiratoryRateAlarmThreshold -> alarm_threshold_high_respiratory_rate,
                    LowTidalVolumeAlarmThreshold -> alarm_threshold_low_tidal_volume,
                    HighTidalVolumeAlarmThreshold -> alarm_threshold_high_tidal_volume,
                    LeakAlarmThreshold -> alarm_threshold_leak,
                }
            );
        }

        // Ensure draft is reset back to none
        self.draft = None;

        events
    }

    fn switch_mode(&mut self, mode: VentilationMode) {
        let old_mode = gen_get_mode_value!(self, mode);

        gen_set_mode_draft!(self, mode, old_mode, mode);
    }

    fn set_inspiratory_time_minimum(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::TiMin,
            action,
            inspiratory_time_minimum,
            INSPIRATORY_TIME_STEP
        )
    }

    fn set_inspiratory_time_maximum(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::TiMax,
            action,
            inspiratory_time_maximum,
            INSPIRATORY_TIME_STEP
        )
    }

    fn set_cycles_per_minute(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::CyclesPerMinute,
            action,
            cycles_per_minute,
            CYCLES_PER_MINUTE_STEP
        )
    }

    fn set_trigger_inspiratory_offset(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::TriggerOffset,
            action,
            trigger_inspiratory_offset,
            TRIGGER_OFFSET_STEP
        )
    }

    fn set_trigger_expiratory_flow(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::ExpiratoryTriggerFlow,
            action,
            trigger_expiratory_flow,
            TRIGGER_FLOW_STEP
        )
    }

    fn set_pressure_plateau(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::PlateauPressure,
            action,
            pressure_plateau,
            PRESSURE_STEP
        )
    }

    fn set_pressure_expiratory(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::PEEP,
            action,
            pressure_expiratory,
            PRESSURE_STEP
        )
    }

    fn set_volume_tidal(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::TargetTidalVolume,
            action,
            volume_tidal,
            VOLUME_STEP
        )
    }

    fn set_flow_inspiration(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::TargetInspiratoryFlow,
            action,
            flow_inspiration,
            FLOW_STEP
        )
    }

    fn set_duration_inspiration(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::InspiratoryDuration,
            action,
            duration_inspiration,
            DURATION_STEP
        )
    }

    fn set_duration_plateau(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::PlateauDuration,
            action,
            duration_plateau,
            DURATION_STEP
        )
    }

    fn set_low_inspiratory_minute_volume_alarm(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::LowInspiratoryMinuteVolumeAlarmThreshold,
            action,
            alarm_threshold_low_inspiratory_minute_volume,
            TRIGGER_FLOW_STEP
        )
    }

    fn set_high_inspiratory_minute_volume_alarm(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::HighInspiratoryMinuteVolumeAlarmThreshold,
            action,
            alarm_threshold_high_inspiratory_minute_volume,
            TRIGGER_FLOW_STEP
        )
    }

    fn set_low_expiratory_minute_volume_alarm(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::LowExpiratoryMinuteVolumeAlarmThreshold,
            action,
            alarm_threshold_low_expiratory_minute_volume,
            TRIGGER_FLOW_STEP
        )
    }

    fn set_high_expiratory_minute_volume_alarm(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::HighExpiratoryMinuteVolumeAlarmThreshold,
            action,
            alarm_threshold_high_expiratory_minute_volume,
            TRIGGER_FLOW_STEP
        )
    }

    fn set_low_respiratory_rate_alarm(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::LowRespiratoryRateAlarmThreshold,
            action,
            alarm_threshold_low_respiratory_rate,
            CYCLES_PER_MINUTE_STEP
        )
    }

    fn set_high_respiratory_rate_alarm(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::HighRespiratoryRateAlarmThreshold,
            action,
            alarm_threshold_high_respiratory_rate,
            CYCLES_PER_MINUTE_STEP
        )
    }

    fn set_low_tidal_volume_alarm(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::LowTidalVolumeAlarmThreshold,
            action,
            alarm_threshold_low_tidal_volume,
            VOLUME_STEP
        )
    }

    fn set_high_tidal_volume_alarm(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::HighTidalVolumeAlarmThreshold,
            action,
            alarm_threshold_high_tidal_volume,
            VOLUME_STEP
        )
    }

    fn set_leak_alarm(&mut self, action: SettingActionRange) {
        gen_set_mode_new_draft_value!(
            self,
            ControlSetting::LeakAlarmThreshold,
            action,
            alarm_threshold_leak,
            VOLUME_STEP
        )
    }
}
