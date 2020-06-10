// MakAir
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub mod settings;

use chrono::{offset::Utc, DateTime, Duration};
use std::collections::{HashMap, VecDeque};
use std::convert::TryFrom;

use crate::config::environment::*;
use crate::physics::types::DataPressure;
use std::sync::mpsc::Sender;
use settings::{ChipSettings, ChipSettingsEvent};
use telemetry::alarm::AlarmCode;
use telemetry::serial::core;
use telemetry::structures::{AlarmPriority, DataSnapshot, MachineStateSnapshot, TelemetryMessage};

#[derive(Debug, PartialEq, Eq)]
pub enum ChipState {
    Initializing,
    Running,
    Stopped,
    WaitingData,
    Error(String),
}

pub struct Chip {
    pub boot_time: Option<DateTime<Utc>>,
    pub last_tick: u64,
    pub data_pressure: DataPressure,
    pub last_machine_snapshot: MachineStateSnapshot,
    pub ongoing_alarms: HashMap<AlarmCode, AlarmPriority>,
    pub battery_level: Option<u8>,
    pub settings: ChipSettings,
    state: ChipState,
    tx_for_lora: Option<Sender<TelemetryMessage>>,
}

impl Chip {
    pub fn new(sender_for_lora: Option<Sender<TelemetryMessage>>) -> Chip {
        Chip {
            boot_time: None,
            last_tick: 0,
            data_pressure: VecDeque::with_capacity(GRAPH_NUMBER_OF_POINTS + 100),
            last_machine_snapshot: MachineStateSnapshot::default(),
            ongoing_alarms: HashMap::new(),
            battery_level: None,
            settings: ChipSettings::new(),
            state: ChipState::WaitingData,
            tx_for_lora: sender_for_lora,
        }
    }

    pub fn new_event(&mut self, event: TelemetryMessage) {
        // send to LORA - can be moved if usefull
        if let Some(tx_for_lora) = &self.tx_for_lora {
            if let Err(e) = tx_for_lora.send(event.clone()) {
                error!("problem while sending data to LORA {:?}", e);
            }
        };

        match event {
            TelemetryMessage::AlarmTrap(alarm) => {
                self.update_tick(alarm.systick);
                self.new_alarm(
                    alarm.alarm_code.into(),
                    alarm.alarm_priority,
                    alarm.triggered,
                );
            }

            TelemetryMessage::BootMessage(snapshot) => {
                self.reset(snapshot.systick);

                self.state = ChipState::Initializing;
            }

            TelemetryMessage::DataSnapshot(snapshot) => {
                self.clean_if_stopped();
                self.update_tick(snapshot.systick);

                self.add_pressure(&snapshot);

                self.battery_level = Some(snapshot.battery_level);
                self.state = ChipState::Running;
            }

            TelemetryMessage::MachineStateSnapshot(snapshot) => {
                self.clean_if_stopped();
                self.update_tick(snapshot.systick);

                for alarm in &snapshot.current_alarm_codes {
                    match AlarmPriority::try_from(*alarm) {
                        Ok(priority) => self.new_alarm((*alarm).into(), priority, true),
                        Err(e) => warn!(
                            "skip alarm {} because we couldn't get the priority: {:?}",
                            alarm, e
                        ),
                    };
                }

                self.last_machine_snapshot = snapshot;

                self.state = ChipState::Running;
            }

            TelemetryMessage::StoppedMessage(message) => {
                self.update_tick(message.systick);

                self.state = ChipState::Stopped;
            }
        };
    }

    pub fn new_settings_events(&mut self, events: Vec<ChipSettingsEvent>) {
        for event in events {
            self.settings.new_settings_event(event);
        }
    }

    pub fn new_error(&mut self, error: core::Error) {
        match error.kind() {
            core::ErrorKind::NoDevice => self.state = ChipState::WaitingData,
            err => self.state = ChipState::Error(format!("{:?}", err)),
        };
    }

    fn new_alarm(&mut self, code: AlarmCode, priority: AlarmPriority, triggered: bool) {
        if triggered {
            self.ongoing_alarms.insert(code, priority); // If we ever receive the same alarm, just replace the one we have
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

        // Low pass filter
        let new_point = last_pressure as i16
            - ((last_pressure as i16 - snapshot.pressure as i16)
                / TELEMETRY_POINTS_LOW_PASS_DEGREE as i16);

        // Points are stored as mmH20 (for more precision; though we do work in cmH20)
        self.data_pressure
            .push_front((snapshot_time, new_point as u16));
    }

    pub fn get_battery_level(&self) -> Option<u8> {
        self.battery_level
    }

    pub fn get_state(&self) -> &ChipState {
        &self.state
    }

    fn update_boot_time(&mut self) {
        let now = Utc::now();
        let duration = chrono::Duration::microseconds(self.last_tick as i64);

        self.boot_time = Some(now - duration);
    }

    fn update_tick(&mut self, tick: u64) {
        // Sometimes, a MachineStateSnapshot event can be received with an older systick
        // This is due to the way the systick is computed on the Makair's side. If we are too close of an ending millisecond
        // the micro() call will probably return the microseconds of the next millisecond, thus making a wrong order
        // If we have less than 1ms of difference between the messages, just ignore the systick update
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

    fn clean_if_stopped(&mut self) {
        if self.state == ChipState::Stopped {
            self.data_pressure.clear();
            self.last_machine_snapshot = MachineStateSnapshot::default();
        }
    }

    pub fn reset(&mut self, new_tick: u64) {
        self.last_tick = new_tick;
        self.data_pressure.clear();
        self.last_machine_snapshot = MachineStateSnapshot::default();
        self.ongoing_alarms.clear();

        self.update_boot_time();
    }

    pub fn clean_events(&mut self) {
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

    pub fn ongoing_alarms_sorted(&self) -> Vec<(&AlarmCode, &AlarmPriority)> {
        let mut vec_alarms = self
            .ongoing_alarms
            .iter()
            .collect::<Vec<(&AlarmCode, &AlarmPriority)>>();

        vec_alarms.sort_by(|(_, priority1), (_, priority2)| priority1.cmp(&priority2).reverse());

        vec_alarms
    }
}
