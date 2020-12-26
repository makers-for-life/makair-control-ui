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
pub struct SettingsMode {
    // Group
    pub group: SettingsModeGroupTab,

    // Settings
    pub live: SettingsModeSettings,
    pub draft: Option<SettingsModeSettings>,
}

#[derive(Debug, PartialEq)]
pub enum SettingsModeGroupTab {
    General,
    Alarms,
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

    fn clear_draft(&mut self) {
        // Ensure draft is cleared
        self.draft = None;
    }

    fn switch_mode(&mut self, mode: VentilationMode) {
        let old_mode = gen_get_mode_value!(self, mode);

        gen_set_mode_draft!(self, mode, old_mode, mode);

        self.group = SettingsModeGroupTab::default();
    }
}

gen_mode_impls!(
    // Commands
    TiMin
        |-> inspiratory_time_minimum [INSPIRATORY_TIME_STEP],
    TiMax
        |-> inspiratory_time_maximum [INSPIRATORY_TIME_STEP],
    CyclesPerMinute
        |-> cycles_per_minute [CYCLES_PER_MINUTE_STEP],
    TriggerOffset
        |-> trigger_inspiratory_offset [TRIGGER_OFFSET_STEP],
    ExpiratoryTriggerFlow
        |-> trigger_expiratory_flow [TRIGGER_FLOW_STEP],
    PlateauPressure
        |-> pressure_plateau [PRESSURE_STEP],
    PEEP
        |-> pressure_expiratory [PRESSURE_STEP],
    TargetTidalVolume
        |-> volume_tidal [VOLUME_STEP],
    TargetInspiratoryFlow
        |-> flow_inspiration [FLOW_STEP],
    InspiratoryDuration
        |-> duration_inspiration [DURATION_STEP],
    PlateauDuration
        |-> duration_plateau [DURATION_STEP],

    // Alarms
    LowInspiratoryMinuteVolumeAlarmThreshold
        |-> alarm_threshold_low_inspiratory_minute_volume [TRIGGER_FLOW_STEP],
    HighInspiratoryMinuteVolumeAlarmThreshold
        |-> alarm_threshold_high_inspiratory_minute_volume [TRIGGER_FLOW_STEP],
    LowExpiratoryMinuteVolumeAlarmThreshold
        |-> alarm_threshold_low_expiratory_minute_volume [TRIGGER_FLOW_STEP],
    HighExpiratoryMinuteVolumeAlarmThreshold
        |-> alarm_threshold_high_expiratory_minute_volume [TRIGGER_FLOW_STEP],
    LowRespiratoryRateAlarmThreshold
        |-> alarm_threshold_low_respiratory_rate [CYCLES_PER_MINUTE_STEP],
    HighRespiratoryRateAlarmThreshold
        |-> alarm_threshold_high_respiratory_rate [CYCLES_PER_MINUTE_STEP],
    LowTidalVolumeAlarmThreshold
        |-> alarm_threshold_low_tidal_volume [VOLUME_STEP],
    HighTidalVolumeAlarmThreshold
        |-> alarm_threshold_high_tidal_volume [VOLUME_STEP],
    LeakAlarmThreshold
        |-> alarm_threshold_leak [VOLUME_STEP],
);
