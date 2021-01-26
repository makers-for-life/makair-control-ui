// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

#[macro_use]
mod macros;

pub mod settings;

use chrono::{offset::Utc, DateTime, Duration};
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;
use std::sync::mpsc::{self, Receiver, Sender};
use std::time::Instant;

use settings::{ChipSettings, ChipSettingsEvent, ChipSettingsIntent, SettingActionState};
use telemetry::alarm::{AlarmCode, RMC_SW_16};
use telemetry::control::{ControlMessage, ControlSetting};
use telemetry::serial::core;
use telemetry::structures::{
    AlarmPriority, ControlAck, DataSnapshot, FatalErrorDetails, HighLevelError,
    MachineStateSnapshot, StoppedMessage, TelemetryMessage, VentilationMode,
};

use crate::config::environment::*;
use crate::utilities::parse::parse_text_lines_to_single;
use crate::utilities::{
    battery::estimate_lead_acid_12v_2s_soc,
    units::{
        convert_cmh2o_to_mmh2o, convert_cv_to_v, convert_mmh2o_to_cmh2o, convert_sub_ppm_to_ppm,
        ConvertMode,
    },
};

const DATA_STORE_EVERY_MILLISECONDS: i64 = 1000 / TELEMETRY_POINTS_PER_SECOND as i64;

pub type ChipDataPoint = (DateTime<Utc>, i16);
pub type ChipDataPoints = VecDeque<ChipDataPoint>;
pub type ChipDataBound = ChipDataPoint;

pub struct ChipData {
    pub points: ChipDataPoints,
    pub bounds_high: Option<ChipDataBound>,
    pub bounds_low: Option<ChipDataBound>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChipState {
    Initializing,
    Running,
    Stopped,
    WaitingData(Instant),
    Error(ChipError),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChipError {
    NoDevice,
    TimedOut,
    BadProtocol,
    Watchdog,
    SensorFailure(String),
    Other(String),
}

#[derive(PartialEq)]
pub enum ChipEventUpdate {
    May,
    MayNot,
}

struct ChipSettingsUpdate {
    // Commands
    plateau_command: Option<u8>,
    peep_command: Option<u8>,
    cpm_command: Option<u8>,
    trigger_offset: Option<u8>,
    alarm_snoozed: Option<bool>,
    ventilation_mode: Option<VentilationMode>,
    expiratory_trigger_flow: Option<u8>,
    ti_min: Option<u16>,
    ti_max: Option<u16>,
    target_tidal_volume: Option<u16>,
    target_inspiratory_flow: Option<u8>,
    inspiratory_duration_command: Option<u16>,
    plateau_duration: Option<u16>,

    // Alarm thresholds
    low_inspiratory_minute_volume_alarm_threshold: Option<u8>,
    high_inspiratory_minute_volume_alarm_threshold: Option<u8>,
    low_expiratory_minute_volume_alarm_threshold: Option<u8>,
    high_expiratory_minute_volume_alarm_threshold: Option<u8>,
    low_respiratory_rate_alarm_threshold: Option<u8>,
    high_respiratory_rate_alarm_threshold: Option<u8>,
    low_tidal_volume_alarm_threshold: Option<u16>,
    high_tidal_volume_alarm_threshold: Option<u16>,
    leak_alarm_threshold: Option<u16>,
    peak_pressure_alarm_threshold: Option<u16>,
}

pub struct Chip {
    pub boot_time: Option<DateTime<Utc>>,
    pub last_tick: u64,
    pub data_pressure: ChipData,
    pub data_flow: ChipData,
    pub last_machine_snapshot: MachineStateSnapshot,
    pub last_data_snapshot: Option<DataSnapshot>,
    pub ongoing_alarms: HashMap<AlarmCode, AlarmPriority>,
    pub estimated_soc: Option<u8>,
    pub settings: ChipSettings,
    pub state: ChipState,
    lora_tx: Option<Sender<TelemetryMessage>>,
    channel_for_settings: Option<Sender<ControlMessage>>,
}

impl ChipData {
    pub fn new() -> Self {
        Self {
            points: ChipDataPoints::with_capacity(GRAPH_NUMBER_OF_POINTS),
            bounds_high: None,
            bounds_low: None,
        }
    }

    pub fn reset(&mut self) {
        self.points.clear();

        self.clear_bounds();
    }

    pub fn clear_bounds(&mut self) {
        self.bounds_high = None;
        self.bounds_low = None;
    }
}

impl Chip {
    pub fn new(lora_sender: Option<Sender<TelemetryMessage>>) -> Chip {
        Chip {
            boot_time: None,
            last_tick: 0,
            data_pressure: ChipData::new(),
            data_flow: ChipData::new(),
            last_machine_snapshot: MachineStateSnapshot::default(),
            last_data_snapshot: None,
            ongoing_alarms: HashMap::new(),
            estimated_soc: None,
            settings: ChipSettings::new(),
            state: ChipState::WaitingData(Instant::now()),
            lora_tx: lora_sender,
            channel_for_settings: None,
        }
    }

    pub fn reset(&mut self, new_tick: u64) {
        self.last_tick = new_tick;

        self.reset_data();

        self.last_machine_snapshot = MachineStateSnapshot::default();
        self.last_data_snapshot = None;

        self.ongoing_alarms.clear();
        self.estimated_soc = None;

        self.update_boot_time();
    }

    pub fn dispatch_heartbeat_event(&mut self) {
        if let Some(tx) = &self.channel_for_settings {
            if let Err(err) = tx.send(ControlMessage {
                setting: ControlSetting::Heartbeat,
                value: 0,
            }) {
                error!("error sending heartbeat to the firmware: {:?}", err);
            } else {
                debug!("heartbeat sent to the firmware");
            }
        }
    }

    pub fn dispatch_settings_intents(&mut self, intents: Vec<ChipSettingsIntent>) {
        // Process all local intents immediately, while events will be dispatched to their \
        //   receiver by the event queue manager.
        for intent in intents {
            debug!("processed immediate setting intent: {:?}", &intent);

            self.settings.new_settings_intent(intent);
        }
    }

    pub fn dispatch_settings_events(&mut self, events: Vec<ChipSettingsEvent>) {
        for event in events {
            let messages = self.settings.new_settings_event(event);

            for message in messages {
                debug!(
                    "handled setting event: {:?}, sender: {:?}",
                    message, self.channel_for_settings
                );

                if let Some(tx) = &self.channel_for_settings {
                    if let Err(err) = tx.send(message.clone()) {
                        error!(
                            "error sending event {:?} to the control unit: {:?}",
                            message, err
                        );
                    } else {
                        debug!("setting event {:?} sent", message);
                    }
                }
            }
        }
    }

    pub fn ongoing_alarms_sorted(&self) -> Vec<(AlarmCode, AlarmPriority)> {
        // This acquires a sorted list of ongoing alarms. It also clears out related alarms. In \
        //   some cases, the list of alarms might contain the same alarm, at different priority \
        //   levels (high and medium). In such cases, we should only retain the high level alarm \
        //   in the list.

        // Prepare ongoing alarms object clone
        let mut ongoing_alarms = self.ongoing_alarms.clone();

        // Map adjacent alarm codes
        let mut adjacent_codes = Vec::new();

        for (alarm_code, _) in ongoing_alarms.iter() {
            // Is this alarm related to a lower-priority alarm? Attempt to remove it from the list \
            //   of alarms, if it exists.
            if let Some(alarm_adjacent_code) = alarm_code.adjacent() {
                if ongoing_alarms.contains_key(&alarm_adjacent_code) {
                    adjacent_codes.push(alarm_adjacent_code);
                }
            }
        }

        // Remove found adjacent alarms (if any)
        for adjacent_code in adjacent_codes {
            ongoing_alarms.remove(&adjacent_code);
        }

        // Map final alarm list
        let mut alarm_list: Vec<(AlarmCode, AlarmPriority)> = ongoing_alarms
            .iter()
            .map(|(code, priority)| (*code, priority.clone()))
            .collect();

        // Sort final alarm list by code, then priority (ensures codes are ordered within their \
        //   priority group, which are themselves sorted by priority)
        alarm_list.sort_by(|(code1, _), (code2, _)| code1.cmp(&code2));
        alarm_list.sort_by(|(_, priority1), (_, priority2)| priority2.cmp(&priority1));

        alarm_list
    }

    pub fn clean_expired_data(&mut self) {
        let time_now = Utc::now();

        self.clean_expired_data_pressure_from_time(time_now);
        self.clean_expired_data_flow_from_time(time_now);
    }

    pub fn init_settings_receiver(&mut self) -> Receiver<ControlMessage> {
        let channel = mpsc::channel();

        self.channel_for_settings = Some(channel.0);

        channel.1
    }

    pub fn new_core_error(&mut self, error: core::Error) {
        match error.kind() {
            core::ErrorKind::NoDevice => self.state = ChipState::Error(ChipError::NoDevice),
            err => {
                self.state = ChipState::Error(ChipError::Other(parse_text_lines_to_single(
                    &format!("{:?}", err),
                    "; ",
                )))
            }
        };
    }

    pub fn new_telemetry_error(&mut self, error: HighLevelError) {
        match error {
            HighLevelError::CrcError { .. } => {
                // CRC errors can be safely ignored as they may only happen once or twice for a \
                //   session, but we still want log an error about this.
                error!("a telemetry event had to be ignored because it raised a crc error");
            }
            HighLevelError::UnsupportedProtocolVersion { .. } => {
                // Protocol version errors are critical, and should always result in an error \
                //   screen as to warn the user.
                error!("got a telemetry event for an unsupported protocol version, erroring out");

                self.state = ChipState::Error(ChipError::BadProtocol);
            }
        };
    }

    pub fn new_event(&mut self, event: TelemetryMessage) -> ChipEventUpdate {
        // Send to LORA? (the 'lora' feature might be disabled, so this would be 'None')
        if let Some(lora_tx) = &self.lora_tx {
            if let Err(err) = lora_tx.send(event.clone()) {
                error!("an issue occured while sending data to lora: {:?}", err);
            }
        };

        // Handle actual telemetry event
        match event {
            TelemetryMessage::AlarmTrap(alarm) => {
                self.update_tick(alarm.systick);

                self.new_alarm(
                    alarm.alarm_code.into(),
                    alarm.alarm_priority,
                    alarm.triggered,
                );

                // An alarm trap should always trigger an UI refresh
                ChipEventUpdate::May
            }

            TelemetryMessage::BootMessage(snapshot) => {
                self.reset(snapshot.systick);

                // A boot message should only trigger an UI refresh when changed
                if self.state != ChipState::Initializing {
                    self.state = ChipState::Initializing;

                    ChipEventUpdate::May
                } else {
                    ChipEventUpdate::MayNot
                }
            }

            TelemetryMessage::DataSnapshot(snapshot) => {
                self.update_tick(snapshot.systick);

                // Append time-series data
                self.add_data_pressure(&snapshot);
                self.add_data_flow(&snapshot);

                // Store last data snapshot
                self.last_data_snapshot = Some(snapshot);

                self.update_state_running();

                // A data snapshot should always trigger an UI refresh (ie. instantaneous graph \
                //   update)
                ChipEventUpdate::May
            }

            TelemetryMessage::MachineStateSnapshot(snapshot) => {
                self.update_tick(snapshot.systick);
                self.update_settings_from_snapshot(&snapshot);
                self.update_alarms_from_snapshot(&snapshot);
                self.update_estimated_soc(snapshot.battery_level, true);

                self.last_machine_snapshot = snapshot;

                self.update_state_running();

                // A machine state snapshot should always trigger an UI refresh, as those are sent \
                //   at the end of each ventilation cycle, and thus are not super spammy.
                ChipEventUpdate::May
            }

            TelemetryMessage::StoppedMessage(message) => {
                self.update_tick(message.systick);
                self.update_settings_and_snapshot_from_stopped(&message);
                self.update_alarms_from_stopped(&message);
                self.update_estimated_soc(message.battery_level, false);

                // Last data snapshot is not relevant when the state went from running to stopped
                self.last_data_snapshot = None;

                // Clear saved data bounds, as to avoid bounds to show in some cases when \
                //   resuming a stopped ventilation that was overflowing.
                self.clear_data_bounds();

                // A stopped message should only trigger an UI refresh when changed
                if self.state != ChipState::Stopped {
                    self.update_state_stopped();

                    ChipEventUpdate::May
                } else {
                    ChipEventUpdate::MayNot
                }
            }

            TelemetryMessage::ControlAck(ack) => {
                self.update_settings_and_snapshot_from_control(ack);

                // A control acknowledgement should always trigger an UI refresh (as the user \
                //   interacted w/ the UI)
                ChipEventUpdate::May
            }

            TelemetryMessage::FatalError(err) => {
                // A fatal error should only trigger an UI refresh if the error state changed
                if self.update_state_error(err.error) {
                    ChipEventUpdate::May
                } else {
                    ChipEventUpdate::MayNot
                }
            }

            TelemetryMessage::EolTestSnapshot(_snapshot) => {
                // TODO: to be implemented
                // @see: https://github.com/makers-for-life/makair-control-ui/issues/81

                // An end-of-line test snapshot should only trigger an UI refresh if there are \
                //   changes
                ChipEventUpdate::MayNot
            }
        }
    }

    fn new_alarm(&mut self, code: AlarmCode, priority: AlarmPriority, triggered: bool) {
        if triggered {
            // If we ever receive the same alarm, just replace the one we have
            self.ongoing_alarms.insert(code, priority);
        } else {
            self.ongoing_alarms.remove(&code);
        }
    }

    fn update_estimated_soc(&mut self, battery_level: Option<u16>, is_running: bool) {
        // Are we battery-powered? (estimate SoC if so, otherwise reset SoC value)
        if self
            .ongoing_alarms
            .iter()
            .any(|alarm| alarm.0.code() == RMC_SW_16)
        {
            // Should we attempt to re-calculate SoC? (provided a battery level is set)
            if let Some(battery_level) = battery_level {
                let new_estimated_soc = estimate_lead_acid_12v_2s_soc(
                    convert_cv_to_v(ConvertMode::WithDecimals, battery_level as _),
                    is_running,
                    self.last_data_snapshot
                        .as_ref()
                        .map(|data| convert_sub_ppm_to_ppm(data.blower_rpm))
                        .unwrap_or(0),
                );

                if let Some(last_estimated_soc) = self.estimated_soc {
                    // Update stored estimated SoC only if the new one is lower than the previous \
                    //   one.
                    // Notice: this prevents SoC to jump up and down when the blower adjusts its \
                    //   PPM speed, or when starting or stopping the ventilation unit. The shown \
                    //   SoC could jump up to a transient (invalid) value, but could never jump \
                    //   lower than the actual value.
                    if new_estimated_soc < last_estimated_soc {
                        self.estimated_soc = Some(new_estimated_soc);
                    }
                } else {
                    // Initialize first SoC value (there was no previous SoC value)
                    self.estimated_soc = Some(new_estimated_soc);
                }
            }
        } else {
            // AC-powered, thus the battery is charging. We need to reset the lowest estimated SoC \
            //   as the battery SoC will start raising again while charging, and thus we want to \
            //   be able to re-estimate the higher battery SoC at a later point when unplugging \
            //   the power supply.
            self.estimated_soc = None;
        }
    }

    fn add_data_pressure(&mut self, snapshot: &DataSnapshot) {
        gen_add_data_generic!(
            self,
            PRESSURE,
            data_pressure,
            snapshot.pressure,
            snapshot.systick,
            clean_expired_data_pressure_from_time
        );
    }

    fn add_data_flow(&mut self, snapshot: &DataSnapshot) {
        let (inspiratory_flow, expiratory_flow) = (
            snapshot.inspiratory_flow.unwrap_or(0),
            snapshot.expiratory_flow.unwrap_or(0),
        );

        // Compute a value that is capped in case of an overflow, as this could result in a panic \
        //   if the telemetry channel sends unusual values.
        let net_flow = gen_cap_number_substract!(inspiratory_flow, expiratory_flow, i16, i32);

        gen_add_data_generic!(
            self,
            FLOW,
            data_flow,
            net_flow,
            snapshot.systick,
            clean_expired_data_flow_from_time
        );
    }

    fn clean_expired_data_pressure_from_time(&mut self, front_time: DateTime<Utc>) {
        gen_clean_expired_data_from_time_generic!(self, data_pressure, front_time);
    }

    fn clean_expired_data_flow_from_time(&mut self, front_time: DateTime<Utc>) {
        gen_clean_expired_data_from_time_generic!(self, data_flow, front_time);
    }

    fn reset_data(&mut self) {
        self.data_pressure.reset();
        self.data_flow.reset();
    }

    fn clear_data_bounds(&mut self) {
        self.data_pressure.clear_bounds();
        self.data_flow.clear_bounds();
    }

    fn update_boot_time(&mut self) {
        let (now, duration) = (
            Utc::now(),
            chrono::Duration::microseconds(self.last_tick as i64),
        );

        self.boot_time = Some(now - duration);
    }

    fn update_tick(&mut self, tick: u64) {
        // Sometimes, a 'MachineStateSnapshot' event can be received with an older systick
        // This is due to the way the systick is computed on the Makair's side. If we are too \
        //   close of an ending millisecond the micro() call will probably return the microseconds \
        //   of the next millisecond, thus making a wrong order. If we have less than 1ms of \
        //   difference between the messages, just ignore the systick update.

        let to_update = if tick < self.last_tick {
            self.last_tick - tick > 1000
        } else {
            true
        };

        if to_update {
            if tick < self.last_tick {
                self.reset(tick);
            } else {
                self.last_tick = tick;

                if self.boot_time.is_none() {
                    self.update_boot_time();
                }
            }
        }
    }

    fn update_state_running(&mut self) {
        // Clean expired pressure & flow data? (this is required so that the graphs does not \
        //   jitter when resuming from a stopped ventilation session)
        if self.state != ChipState::Running {
            self.clean_expired_data();
        }

        self.settings.run.state = SettingActionState::Enabled;
        self.state = ChipState::Running;
    }

    fn update_state_stopped(&mut self) {
        self.settings.run.state = SettingActionState::Disabled;
        self.state = ChipState::Stopped;
    }

    fn update_state_error(&mut self, details: FatalErrorDetails) -> bool {
        // Map fatal error to internal chip error
        let chip_error = ChipState::Error(match details {
            FatalErrorDetails::WatchdogRestart => ChipError::Watchdog,

            FatalErrorDetails::CalibrationError { .. } => {
                ChipError::Other("calibration-error".to_string())
            }

            FatalErrorDetails::BatteryDeeplyDischarged { battery_level } => {
                ChipError::Other(format!("battery-deeply-discharged <{}>", battery_level))
            }

            FatalErrorDetails::MassFlowMeterError => {
                ChipError::SensorFailure("mass-flow-meter".to_string())
            }

            FatalErrorDetails::InconsistentPressure { pressure } => {
                ChipError::SensorFailure(format!("inconsistent-pressure <{}>", pressure))
            }
        });

        // Update state? (only if changed)
        if self.state != chip_error {
            self.state = chip_error;

            // State updated
            true
        } else {
            // State not updated
            false
        }
    }

    fn update_settings_from_parameters(&mut self, update: ChipSettingsUpdate) {
        // Update expiratory term values
        if let Some(ti_min) = update.ti_min {
            self.settings.mode.live.inspiratory_time_minimum = ti_min as usize;
        }

        if let Some(ti_max) = update.ti_max {
            self.settings.mode.live.inspiratory_time_maximum = ti_max as usize;
        }

        // Update trigger values
        if let Some(trigger_offset) = update.trigger_offset {
            self.settings.mode.live.trigger_inspiratory_offset = trigger_offset as usize;
        }

        if let Some(expiratory_trigger_flow) = update.expiratory_trigger_flow {
            self.settings.mode.live.trigger_expiratory_flow = expiratory_trigger_flow as usize;
        }

        // Update cycle values
        if let Some(cpm_command) = update.cpm_command {
            self.settings.mode.live.cycles_per_minute = cpm_command as usize;
        }

        // Update pressure values
        if let Some(plateau_command) = update.plateau_command {
            self.settings.mode.live.pressure_plateau = convert_cmh2o_to_mmh2o(plateau_command);
        }
        if let Some(peep_command) = update.peep_command {
            self.settings.mode.live.pressure_expiratory = convert_cmh2o_to_mmh2o(peep_command);
        }

        // Update target tidal volume values
        if let Some(target_tidal_volume) = update.target_tidal_volume {
            self.settings.mode.live.volume_tidal = target_tidal_volume as usize;
        }

        // Update target inspiratory flow values
        if let Some(target_inspiratory_flow) = update.target_inspiratory_flow {
            self.settings.mode.live.flow_inspiration = target_inspiratory_flow as usize;
        }

        // Update inspiratory duration values
        if let Some(inspiratory_duration_command) = update.inspiratory_duration_command {
            self.settings.mode.live.duration_inspiration = inspiratory_duration_command as usize;
        }

        // Update plateau duration values
        if let Some(plateau_duration) = update.plateau_duration {
            self.settings.mode.live.duration_plateau = plateau_duration as usize;
        }

        // Update alarms snooze values
        if let Some(alarm_snoozed) = update.alarm_snoozed {
            self.settings.snooze.alarms = if alarm_snoozed {
                SettingActionState::Enabled
            } else {
                SettingActionState::Disabled
            };
        }

        // Update ventilation mode value
        if let Some(ventilation_mode) = update.ventilation_mode {
            self.settings.mode.live.mode = ventilation_mode;
        }

        // Update all alarm threshold values
        gen_settings_from_parameters_alarm_thresholds!(
            update,
            self.settings.mode.live,
            [
                low_inspiratory_minute_volume,
                high_inspiratory_minute_volume,
                low_expiratory_minute_volume,
                high_expiratory_minute_volume,
                low_respiratory_rate,
                high_respiratory_rate,
                low_tidal_volume,
                high_tidal_volume,
                leak,
                peak_pressure
            ]
        );
    }

    fn update_settings_from_snapshot(&mut self, snapshot: &MachineStateSnapshot) {
        // This updates all settings value, upon receiving a \
        //   'TelemetryMessage::MachineStateSnapshot' message from the firmware.

        self.update_settings_from_parameters(ChipSettingsUpdate {
            // Commands
            plateau_command: Some(snapshot.plateau_command),
            peep_command: Some(snapshot.peep_command),
            cpm_command: Some(snapshot.cpm_command),
            trigger_offset: Some(snapshot.trigger_offset),
            alarm_snoozed: snapshot.alarm_snoozed,
            ventilation_mode: Some(snapshot.ventilation_mode),
            expiratory_trigger_flow: snapshot.expiratory_trigger_flow,
            ti_min: snapshot.ti_min,
            ti_max: snapshot.ti_max,
            target_tidal_volume: snapshot.target_tidal_volume,
            target_inspiratory_flow: snapshot.target_inspiratory_flow,
            inspiratory_duration_command: snapshot.inspiratory_duration_command,
            plateau_duration: snapshot.plateau_duration,

            // Alarm thresholds
            low_inspiratory_minute_volume_alarm_threshold: snapshot
                .low_inspiratory_minute_volume_alarm_threshold,
            high_inspiratory_minute_volume_alarm_threshold: snapshot
                .high_inspiratory_minute_volume_alarm_threshold,
            low_expiratory_minute_volume_alarm_threshold: snapshot
                .low_expiratory_minute_volume_alarm_threshold,
            high_expiratory_minute_volume_alarm_threshold: snapshot
                .high_expiratory_minute_volume_alarm_threshold,
            low_respiratory_rate_alarm_threshold: snapshot.low_respiratory_rate_alarm_threshold,
            high_respiratory_rate_alarm_threshold: snapshot.high_respiratory_rate_alarm_threshold,
            low_tidal_volume_alarm_threshold: snapshot.low_tidal_volume_alarm_threshold,
            high_tidal_volume_alarm_threshold: snapshot.high_tidal_volume_alarm_threshold,
            leak_alarm_threshold: snapshot.leak_alarm_threshold,
            peak_pressure_alarm_threshold: snapshot.peak_pressure_alarm_threshold,
        });
    }

    fn update_settings_and_snapshot_from_stopped(&mut self, message: &StoppedMessage) {
        // This initializes all settings and snapshot values, upon receiving a \
        //   'TelemetryMessage::StoppedMessage' message from the firmware.

        // Update local settings from message
        self.update_settings_from_parameters(ChipSettingsUpdate {
            // Commands
            plateau_command: message.plateau_command,
            peep_command: message.peep_command,
            cpm_command: message.cpm_command,
            trigger_offset: message.trigger_offset,
            alarm_snoozed: message.alarm_snoozed,
            ventilation_mode: Some(message.ventilation_mode),
            expiratory_trigger_flow: message.expiratory_trigger_flow,
            ti_min: message.ti_min,
            ti_max: message.ti_max,
            target_tidal_volume: message.target_tidal_volume,
            target_inspiratory_flow: message.target_inspiratory_flow,
            inspiratory_duration_command: message.inspiratory_duration_command,
            plateau_duration: message.plateau_duration,

            // Alarm thresholds
            low_inspiratory_minute_volume_alarm_threshold: message
                .low_inspiratory_minute_volume_alarm_threshold,
            high_inspiratory_minute_volume_alarm_threshold: message
                .high_inspiratory_minute_volume_alarm_threshold,
            low_expiratory_minute_volume_alarm_threshold: message
                .low_expiratory_minute_volume_alarm_threshold,
            high_expiratory_minute_volume_alarm_threshold: message
                .high_expiratory_minute_volume_alarm_threshold,
            low_respiratory_rate_alarm_threshold: message.low_respiratory_rate_alarm_threshold,
            high_respiratory_rate_alarm_threshold: message.high_respiratory_rate_alarm_threshold,
            low_tidal_volume_alarm_threshold: message.low_tidal_volume_alarm_threshold,
            high_tidal_volume_alarm_threshold: message.high_tidal_volume_alarm_threshold,
            leak_alarm_threshold: message.leak_alarm_threshold,
            peak_pressure_alarm_threshold: message.peak_pressure_alarm_threshold,
        });

        // Assign same-type message values to snapshot (that must be cloned)
        gen_override_snapshot_values_from_stopped_identity_clone!(
            self.last_machine_snapshot,
            message,
            [version, device_id]
        );

        // Assign same-type message values to snapshot (ie. same type on both sides)
        gen_override_snapshot_values_from_stopped_identity!(
            self.last_machine_snapshot,
            message,
            [
                telemetry_version,
                alarm_snoozed,
                cpu_load,
                ventilation_mode,
                inspiratory_trigger_flow,
                expiratory_trigger_flow,
                ti_min,
                ti_max,
                low_inspiratory_minute_volume_alarm_threshold,
                high_inspiratory_minute_volume_alarm_threshold,
                low_expiratory_minute_volume_alarm_threshold,
                high_expiratory_minute_volume_alarm_threshold,
                low_respiratory_rate_alarm_threshold,
                high_respiratory_rate_alarm_threshold,
                battery_level,
                locale,
                patient_height,
                patient_gender
            ]
        );

        // Assign optional message values to snapshot
        gen_override_snapshot_values_from_stopped_optional!(
            self.last_machine_snapshot,
            message,
            [
                peak_command,
                plateau_command,
                peep_command,
                cpm_command,
                trigger_enabled,
                trigger_offset,
                expiratory_term,
                current_alarm_codes
            ]
        );
    }

    fn update_settings_and_snapshot_from_control(&mut self, ack: ControlAck) {
        // This updates the internal stored states whenever the UI receives a \
        //   'TelemetryMessage::ControlAck' message, which is sent to confirm an user-generated \
        //   control message has been accepted and applied in the firmware. Updating all internal \
        //   values straight away live-refreshes the UI w/ the up-to-date values, instead of \
        //   requiring a full wait until the next data snapshot comes (at the end of each cycle).
        match ack.setting {
            ControlSetting::Heartbeat => {
                // Ignore heartbeat acknowledgements (stateless)
            }

            ControlSetting::PlateauPressure => {
                self.settings.mode.live.pressure_plateau = ack.value as usize;
                self.last_machine_snapshot.plateau_command =
                    convert_mmh2o_to_cmh2o(ConvertMode::Rounded, ack.value as f64) as u8
            }

            ControlSetting::PEEP => {
                self.settings.mode.live.pressure_expiratory = ack.value as usize;
                self.last_machine_snapshot.peep_command =
                    convert_mmh2o_to_cmh2o(ConvertMode::Rounded, ack.value as f64) as u8
            }

            ControlSetting::CyclesPerMinute => {
                self.settings.mode.live.cycles_per_minute = ack.value as usize;
                self.last_machine_snapshot.cpm_command = ack.value as u8
            }

            ControlSetting::TriggerEnabled => {
                if ack.value == 0 {
                    self.last_machine_snapshot.trigger_enabled = false;
                } else {
                    self.last_machine_snapshot.trigger_enabled = true;
                }
            }

            ControlSetting::TriggerOffset => {
                self.settings.mode.live.trigger_inspiratory_offset = ack.value as usize;
                self.last_machine_snapshot.trigger_offset = ack.value as u8;
            }

            ControlSetting::ExpiratoryTerm => {
                self.last_machine_snapshot.expiratory_term = ack.value as u8;
            }

            ControlSetting::RespirationEnabled => {
                if ack.value == 0 {
                    self.update_state_stopped();
                } else {
                    self.update_state_running();
                }
            }

            ControlSetting::AlarmSnooze => {
                if ack.value == 0 {
                    self.settings.snooze.alarms = SettingActionState::Disabled;
                    self.last_machine_snapshot.alarm_snoozed = Some(false);
                } else {
                    self.settings.snooze.alarms = SettingActionState::Enabled;
                    self.last_machine_snapshot.alarm_snoozed = Some(true);
                }
            }

            ControlSetting::VentilationMode => {
                if let Ok(ventilation_mode) = VentilationMode::try_from(ack.value as u8) {
                    self.settings.mode.live.mode = ventilation_mode;
                    self.last_machine_snapshot.ventilation_mode = ventilation_mode;
                }
            }

            ControlSetting::InspiratoryTriggerFlow => {
                self.last_machine_snapshot.inspiratory_trigger_flow = Some(ack.value as _);
            }

            ControlSetting::ExpiratoryTriggerFlow => {
                self.settings.mode.live.trigger_expiratory_flow = ack.value as usize;
                self.last_machine_snapshot.expiratory_trigger_flow = Some(ack.value as _);
            }

            ControlSetting::TiMin => {
                self.settings.mode.live.inspiratory_time_minimum = ack.value as usize;
                self.last_machine_snapshot.ti_min = Some(ack.value);
            }

            ControlSetting::TiMax => {
                self.settings.mode.live.inspiratory_time_maximum = ack.value as usize;
                self.last_machine_snapshot.ti_max = Some(ack.value);
            }

            ControlSetting::LowInspiratoryMinuteVolumeAlarmThreshold => {
                self.settings
                    .mode
                    .live
                    .alarm_threshold_low_inspiratory_minute_volume = ack.value as usize;
                self.last_machine_snapshot
                    .low_inspiratory_minute_volume_alarm_threshold = Some(ack.value as _);
            }

            ControlSetting::HighInspiratoryMinuteVolumeAlarmThreshold => {
                self.settings
                    .mode
                    .live
                    .alarm_threshold_high_inspiratory_minute_volume = ack.value as usize;
                self.last_machine_snapshot
                    .high_inspiratory_minute_volume_alarm_threshold = Some(ack.value as _);
            }

            ControlSetting::LowExpiratoryMinuteVolumeAlarmThreshold => {
                self.settings
                    .mode
                    .live
                    .alarm_threshold_low_expiratory_minute_volume = ack.value as usize;
                self.last_machine_snapshot
                    .low_expiratory_minute_volume_alarm_threshold = Some(ack.value as _);
            }

            ControlSetting::HighExpiratoryMinuteVolumeAlarmThreshold => {
                self.settings
                    .mode
                    .live
                    .alarm_threshold_high_expiratory_minute_volume = ack.value as usize;
                self.last_machine_snapshot
                    .high_expiratory_minute_volume_alarm_threshold = Some(ack.value as _);
            }

            ControlSetting::LowRespiratoryRateAlarmThreshold => {
                self.settings.mode.live.alarm_threshold_low_respiratory_rate = ack.value as usize;
                self.last_machine_snapshot
                    .low_respiratory_rate_alarm_threshold = Some(ack.value as _);
            }

            ControlSetting::HighRespiratoryRateAlarmThreshold => {
                self.settings
                    .mode
                    .live
                    .alarm_threshold_high_respiratory_rate = ack.value as usize;
                self.last_machine_snapshot
                    .high_respiratory_rate_alarm_threshold = Some(ack.value as _);
            }

            ControlSetting::TargetTidalVolume => {
                self.settings.mode.live.volume_tidal = ack.value as usize;
                self.last_machine_snapshot.target_tidal_volume = Some(ack.value);
            }

            ControlSetting::LowTidalVolumeAlarmThreshold => {
                self.settings.mode.live.alarm_threshold_low_tidal_volume = ack.value as usize;
                self.last_machine_snapshot.low_tidal_volume_alarm_threshold = Some(ack.value as _);
            }

            ControlSetting::HighTidalVolumeAlarmThreshold => {
                self.settings.mode.live.alarm_threshold_high_tidal_volume = ack.value as usize;
                self.last_machine_snapshot.high_tidal_volume_alarm_threshold = Some(ack.value as _);
            }

            ControlSetting::PlateauDuration => {
                self.settings.mode.live.duration_plateau = ack.value as usize;
                self.last_machine_snapshot.plateau_duration = Some(ack.value);
            }

            ControlSetting::LeakAlarmThreshold => {
                self.settings.mode.live.alarm_threshold_leak = ack.value as usize;
                self.last_machine_snapshot.leak_alarm_threshold = Some(ack.value as _);
            }

            ControlSetting::TargetInspiratoryFlow => {
                self.settings.mode.live.flow_inspiration = ack.value as usize;
                self.last_machine_snapshot.target_inspiratory_flow = Some(ack.value as _);
            }

            ControlSetting::InspiratoryDuration => {
                self.settings.mode.live.duration_inspiration = ack.value as usize;
                self.last_machine_snapshot.inspiratory_duration_command = Some(ack.value);
            }

            ControlSetting::Locale => {
                // TODO: to be implemented
                // @see: https://github.com/makers-for-life/makair-control-ui/issues/78
            }

            ControlSetting::PatientHeight => {
                // TODO: to be implemented
                // @see: https://github.com/makers-for-life/makair-control-ui/issues/75
            }

            ControlSetting::PatientGender => {
                // TODO: to be implemented
                // @see: https://github.com/makers-for-life/makair-control-ui/issues/75
            }

            ControlSetting::PeakPressureAlarmThreshold => {
                self.settings.mode.live.alarm_threshold_peak_pressure = ack.value as usize;
                self.last_machine_snapshot.peak_pressure_alarm_threshold = Some(ack.value as _);
            }
        }
    }

    fn update_alarms_from_parameters(&mut self, alarms: Option<&Vec<u8>>) {
        // Notice: as alarms added and removed from the 'TelemetryMessage::AlarmTrap' message \
        //   may sometimes faill due to a missed telemetry packet (eg. out-of-order et al), we \
        //   need to re-synchronize all displayed alarms from the end-of-cycle machine state \
        //   snapshot messages, and stopped messages, as well. Hence why we first add the missing \
        //   alarms, and then remove the alarms that do not exist anymore. Usually, this handler \
        //   should do no work and leave the local alarms storage object untouched. That is why we \
        //   do not perform a simple clear of the storage map and full rebuild everytime, as this \
        //   would be sub-optimal.

        if let Some(alarms) = alarms {
            // Map received alarm codes
            let alarms: Vec<AlarmCode> = alarms.iter().map(|alarm| (*alarm).into()).collect();

            // Handle current alarms? (from truth source, ie. snapshot or stopped)
            if !alarms.is_empty() {
                for alarm_code in &alarms {
                    // Attempt to parse and insert alarm? (only if it does not exist yet in storage)
                    if !self.ongoing_alarms.contains_key(alarm_code) {
                        warn!(
                            "adding ongoing alarm: {:?}, which went desynchronized with its trap",
                            alarm_code
                        );

                        match AlarmPriority::try_from(alarm_code.code()) {
                            Ok(priority) => self.new_alarm(*alarm_code, priority, true),
                            Err(err) => warn!(
                                "skip alarm {:?} because we could not get the priority: {:?}",
                                alarm_code, err
                            ),
                        };
                    }
                }
            }

            // Flush non-existing alarms? (from truth source, ie. snapshot or stopped)
            if !self.ongoing_alarms.is_empty() {
                let cleared_alarm_codes: Vec<AlarmCode> = self
                    .ongoing_alarms
                    .iter()
                    .map(|(ongoing_alarm_code, _)| *ongoing_alarm_code)
                    .filter(|ongoing_alarm_code| !alarms.contains(ongoing_alarm_code))
                    .collect();

                if !cleared_alarm_codes.is_empty() {
                    warn!(
                        "clearing {} ongoing alarms, which went desynchronized with traps",
                        self.ongoing_alarms.len()
                    );

                    for cleared_alarm_code in &cleared_alarm_codes {
                        self.ongoing_alarms.remove(cleared_alarm_code);
                    }
                }
            }
        }
    }

    fn update_alarms_from_snapshot(&mut self, snapshot: &MachineStateSnapshot) {
        // This synchronizes all active alarms, upon receiving a \
        //   'TelemetryMessage::MachineStateSnapshot' message from the firmware.

        self.update_alarms_from_parameters(Some(&snapshot.current_alarm_codes));
    }

    fn update_alarms_from_stopped(&mut self, message: &StoppedMessage) {
        // This synchronizes all active alarms, upon receiving a \
        //   'TelemetryMessage::StoppedMessage' message from the firmware.

        self.update_alarms_from_parameters(message.current_alarm_codes.as_ref());
    }
}
