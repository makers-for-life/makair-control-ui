// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub mod settings;

use chrono::{offset::Utc, DateTime, Duration};
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;
use std::sync::mpsc::{self, Receiver, Sender};

use settings::{ChipSettings, ChipSettingsEvent, SettingActionState};
use telemetry::alarm::AlarmCode;
use telemetry::control::{ControlMessage, ControlSetting};
use telemetry::serial::core;
use telemetry::structures::{
    AlarmPriority, ControlAck, DataSnapshot, MachineStateSnapshot, StoppedMessage, TelemetryMessage,
};

use crate::config::environment::*;
use crate::utilities::types::DataPressure;
use crate::utilities::units::{convert_cmh2o_to_mmh2o, convert_mmh2o_to_cmh2o, ConvertMode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChipState {
    Initializing,
    Running,
    Stopped,
    WaitingData,
    Error(String),
}

#[derive(PartialEq)]
pub enum ChipEventUpdate {
    May,
    MayNot,
}

struct ChipSettingsUpdate {
    peak_command: Option<u8>,
    plateau_command: Option<u8>,
    peep_command: Option<u8>,
    cpm_command: Option<u8>,
    expiratory_term: Option<u8>,
    trigger_enabled: Option<bool>,
    trigger_offset: Option<u8>,
}

pub struct Chip {
    pub boot_time: Option<DateTime<Utc>>,
    pub last_tick: u64,
    pub data_pressure: DataPressure,
    pub last_machine_snapshot: MachineStateSnapshot,
    pub last_data_snapshot: Option<DataSnapshot>,
    pub ongoing_alarms: HashMap<AlarmCode, AlarmPriority>,
    pub settings: ChipSettings,
    state: ChipState,
    lora_tx: Option<Sender<TelemetryMessage>>,
    channel_for_settings: Option<Sender<ControlMessage>>,
}

impl Chip {
    pub fn new(lora_sender: Option<Sender<TelemetryMessage>>) -> Chip {
        let last_machine_snapshot = MachineStateSnapshot::default();
        let cycles_per_minute = last_machine_snapshot.cpm_command as usize;

        Chip {
            boot_time: None,
            last_tick: 0,
            data_pressure: VecDeque::with_capacity(GRAPH_NUMBER_OF_POINTS + 100),
            last_machine_snapshot,
            last_data_snapshot: None,
            ongoing_alarms: HashMap::new(),
            settings: ChipSettings::new(cycles_per_minute),
            state: ChipState::WaitingData,
            lora_tx: lora_sender,
            channel_for_settings: None,
        }
    }

    pub fn reset(&mut self, new_tick: u64) {
        self.last_tick = new_tick;

        self.clean();

        self.ongoing_alarms.clear();

        self.update_boot_time();
    }

    pub fn clean_expired_pressure(&mut self) {
        if !self.data_pressure.is_empty() {
            let older = self.data_pressure.front().unwrap().0
                - chrono::Duration::seconds(GRAPH_DRAW_SECONDS as _);

            while self
                .data_pressure
                .back()
                .map(|p| p.0 < older)
                .unwrap_or(false)
            {
                self.data_pressure.pop_back();
            }
        }
    }

    pub fn handle_settings_events(&mut self, events: Vec<ChipSettingsEvent>) {
        for event in events {
            let message = self.settings.new_settings_event(event);

            debug!(
                "handled setting event: {:?}, sender: {:?}",
                message, self.channel_for_settings
            );

            if let Some(tx) = &self.channel_for_settings {
                if let Err(err) = tx.send(message.clone()) {
                    error!(
                        "error sending message {:?} to the control unit: {:?}",
                        message, err
                    );
                } else {
                    debug!("setting message {:?} sent", message);
                }
            }
        }
    }

    pub fn get_battery_level(&self) -> Option<u8> {
        self.last_data_snapshot
            .as_ref()
            .map(|snapshot| snapshot.battery_level)
    }

    pub fn get_state(&self) -> &ChipState {
        &self.state
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

        // Sort final alarm list
        alarm_list.sort_by(|(code1, _), (code2, _)| code1.cmp(&code2));

        alarm_list
    }

    pub fn init_settings_receiver(&mut self) -> Receiver<ControlMessage> {
        let channel = mpsc::channel();

        self.channel_for_settings = Some(channel.0);

        channel.1
    }

    pub fn new_error(&mut self, error: core::Error) {
        match error.kind() {
            core::ErrorKind::NoDevice => self.state = ChipState::WaitingData,
            err => self.state = ChipState::Error(format!("{:?}", err)),
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
                self.clean_if_stopped();

                self.update_tick(snapshot.systick);
                self.add_pressure(&snapshot);

                self.last_data_snapshot = Some(snapshot);

                self.update_state_running();

                // A data snapshot should always trigger an UI refresh (ie. instantaneous graph \
                //   update)
                ChipEventUpdate::May
            }

            TelemetryMessage::MachineStateSnapshot(snapshot) => {
                self.clean_if_stopped();

                self.update_tick(snapshot.systick);
                self.update_settings_from_snapshot(&snapshot);

                for alarm in &snapshot.current_alarm_codes {
                    match AlarmPriority::try_from(*alarm) {
                        Ok(priority) => self.new_alarm((*alarm).into(), priority, true),
                        Err(err) => warn!(
                            "skip alarm {} because we could not get the priority: {:?}",
                            alarm, err
                        ),
                    };
                }

                self.last_machine_snapshot = snapshot;

                self.update_state_running();

                // A machine state snapshot should always trigger an UI refresh, as those are sent \
                //   at the end of each ventilation cycle, and thus are not super spammy.
                ChipEventUpdate::May
            }

            TelemetryMessage::StoppedMessage(message) => {
                self.update_tick(message.systick);
                self.update_settings_from_stopped(&message);

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

    fn add_pressure(&mut self, snapshot: &DataSnapshot) {
        assert!(self.boot_time.is_some());

        let snapshot_time =
            self.boot_time.unwrap() + Duration::microseconds(snapshot.systick as i64);

        // Fetch last pressure value in order to reduce noise
        let last_pressure = if let Some(last_pressure_inner) = self.data_pressure.get(0) {
            last_pressure_inner.1
        } else {
            0
        };

        // Low-pass filter
        let new_point = last_pressure as i16
            - ((last_pressure as i16 - snapshot.pressure as i16)
                / TELEMETRY_POINTS_LOW_PASS_DEGREE as i16);

        // Points are stored as mmH20 (for more precision; though we do work in cmH20)
        self.data_pressure
            .push_front((snapshot_time, new_point as u16));
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

    fn clean(&mut self) {
        self.data_pressure.clear();

        self.last_machine_snapshot = MachineStateSnapshot::default();
        self.last_data_snapshot = None;
    }

    fn clean_if_stopped(&mut self) {
        if self.state == ChipState::Stopped {
            self.clean();
        }
    }

    fn update_state_running(&mut self) {
        self.settings.run.state = SettingActionState::Enabled;
        self.state = ChipState::Running;
    }

    fn update_state_stopped(&mut self) {
        self.settings.run.state = SettingActionState::Disabled;
        self.state = ChipState::Stopped;
    }

    fn update_settings_from_parameters(&mut self, update: ChipSettingsUpdate) {
        // Update expiratory term values
        if let Some(expiratory_term) = update.expiratory_term {
            self.settings.expiration_term.expiratory_term = expiratory_term as usize;
        }
        if let Some(cpm_command) = update.cpm_command {
            self.settings.expiration_term.cycles_per_minute = cpm_command as usize;
        }

        // Update trigger values
        if let Some(trigger_enabled) = update.trigger_enabled {
            self.settings.trigger.state = if trigger_enabled {
                SettingActionState::Enabled
            } else {
                SettingActionState::Disabled
            };
        }

        if let Some(trigger_offset) = update.trigger_offset {
            self.settings.trigger.inspiratory_trigger_offset = trigger_offset as usize;
        }

        // Update cycle values
        if let Some(cpm_command) = update.cpm_command {
            self.settings.cycles.cycles_per_minute = cpm_command as usize;
        }

        // Update pressure values
        if let Some(peak_command) = update.peak_command {
            self.settings.pressure.peak = convert_cmh2o_to_mmh2o(peak_command);
        }
        if let Some(plateau_command) = update.plateau_command {
            self.settings.pressure.plateau = convert_cmh2o_to_mmh2o(plateau_command);
        }
        if let Some(peep_command) = update.peep_command {
            self.settings.pressure.peep = convert_cmh2o_to_mmh2o(peep_command);
        }
    }

    fn update_settings_from_snapshot(&mut self, snapshot: &MachineStateSnapshot) {
        // This updates all settings value, upon receiving a \
        //   'TelemetryMessage::MachineStateSnapshot' message from the firmware.

        self.update_settings_from_parameters(ChipSettingsUpdate {
            peak_command: Some(snapshot.peak_command),
            plateau_command: Some(snapshot.plateau_command),
            peep_command: Some(snapshot.peep_command),
            cpm_command: Some(snapshot.cpm_command),
            expiratory_term: Some(snapshot.expiratory_term),
            trigger_enabled: Some(snapshot.trigger_enabled),
            trigger_offset: Some(snapshot.trigger_offset),
        });
    }

    fn update_settings_from_stopped(&mut self, message: &StoppedMessage) {
        // This initializes all settings value, upon receiving a \
        //   'TelemetryMessage::StoppedMessage' message from the firmware.

        self.update_settings_from_parameters(ChipSettingsUpdate {
            peak_command: message.peak_command,
            plateau_command: message.plateau_command,
            peep_command: message.peep_command,
            cpm_command: message.cpm_command,
            expiratory_term: message.expiratory_term,
            trigger_enabled: message.trigger_enabled,
            trigger_offset: message.trigger_offset,
        });
    }

    fn update_settings_and_snapshot_from_control(&mut self, ack: ControlAck) {
        // This updates the internal stored states whenever the UI receives a \
        //   'TelemetryMessage::ControlAck' message, which is sent to confirm an user-generated \
        //   control message has been accepted and applied in the firmware. Updating all internal \
        //   values straight away live-refreshes the UI w/ the up-to-date values, instead of \
        //   requiring a full wait until the next data snapshot comes (at the end of each cycle).
        match ack.setting {
            ControlSetting::PeakPressure => {
                self.settings.pressure.peak = ack.value as usize;
                self.last_machine_snapshot.peak_command =
                    convert_mmh2o_to_cmh2o(ConvertMode::Rounded, ack.value as f64) as u8
            }

            ControlSetting::PlateauPressure => {
                self.settings.pressure.plateau = ack.value as usize;
                self.last_machine_snapshot.plateau_command =
                    convert_mmh2o_to_cmh2o(ConvertMode::Rounded, ack.value as f64) as u8
            }

            ControlSetting::PEEP => {
                self.settings.pressure.peep = ack.value as usize;
                self.last_machine_snapshot.peep_command =
                    convert_mmh2o_to_cmh2o(ConvertMode::Rounded, ack.value as f64) as u8
            }

            ControlSetting::CyclesPerMinute => {
                self.settings.cycles.cycles_per_minute = ack.value as usize;
                self.last_machine_snapshot.cpm_command = ack.value as u8
            }

            ControlSetting::TriggerEnabled => {
                if ack.value == 0 {
                    self.settings.trigger.state = SettingActionState::Disabled;
                    self.last_machine_snapshot.trigger_enabled = false;
                } else {
                    self.settings.trigger.state = SettingActionState::Enabled;
                    self.last_machine_snapshot.trigger_enabled = true;
                }
            }

            ControlSetting::TriggerOffset => {
                self.settings.trigger.inspiratory_trigger_offset = ack.value as usize;
                self.last_machine_snapshot.trigger_offset = ack.value as u8;
            }

            ControlSetting::ExpiratoryTerm => {
                self.settings.expiration_term.expiratory_term = ack.value as usize;
                self.last_machine_snapshot.expiratory_term = ack.value as u8;
            }

            ControlSetting::RespirationEnabled => {
                if ack.value == 0 {
                    self.update_state_stopped();
                } else {
                    self.update_state_running();
                }
            }
        }
    }
}
