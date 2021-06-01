// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

#[macro_use]
mod macros;

pub mod advanced;
pub mod end_of_line;
pub mod mode;
pub mod preset;
pub mod run;
pub mod snooze;

use std::ops::RangeInclusive;

use makair_telemetry::control::{ControlMessage, ControlSetting};

use advanced::*;
use end_of_line::*;
use mode::*;
use preset::*;
use run::*;
use snooze::*;

#[derive(Debug)]
pub enum ChipSettingsEvent {
    Preset(SettingsPresetEvent),
    Run(SettingsRunEvent),
    Snooze(SettingsSnoozeEvent),
    Mode(SettingsModeEvent),
    EndOfLine(SettingsEndOfLineEvent),
}

#[derive(Debug)]
pub enum ChipSettingsIntent {
    Mode(SettingsModeIntent),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SettingActionState {
    Disabled = 0,
    Enabled = 1,
}

#[derive(Debug)]
pub enum SettingActionRange {
    More,
    Less,
}

impl SettingActionState {
    pub fn from_value(value: usize) -> Self {
        if value > 0 {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }

    fn to_toggled(&self) -> Self {
        match self {
            Self::Enabled => Self::Disabled,
            Self::Disabled => Self::Enabled,
        }
    }
}

impl SettingActionRange {
    fn to_new_value(&self, setting: &ControlSetting, value: usize, step: usize) -> usize {
        self.to_new_value_bounds(value, step, setting.bounds())
    }

    fn to_new_value_bounds(
        &self,
        value: usize,
        step: usize,
        bounds: RangeInclusive<usize>,
    ) -> usize {
        match self {
            SettingActionRange::More => {
                let new_value = value + step;

                if bounds.contains(&new_value) {
                    new_value
                } else {
                    value
                }
            }
            SettingActionRange::Less => {
                if value >= step {
                    let new_value = value - step;

                    if bounds.contains(&new_value) {
                        new_value
                    } else {
                        value
                    }
                } else {
                    value
                }
            }
        }
    }

    fn to_next_index(&self, current_index: i16) -> i16 {
        current_index as i16
            + match self {
                SettingActionRange::Less => -1,
                SettingActionRange::More => 1,
            }
    }
}

#[derive(Debug)]
pub struct ChipSettings {
    pub run: SettingsRun,
    pub snooze: SettingsSnooze,
    pub preset: SettingsPreset,
    pub advanced: SettingsAdvanced,
    pub mode: SettingsMode,
    pub end_of_line: SettingsEndOfLine,
}

impl ChipSettings {
    pub fn new() -> ChipSettings {
        ChipSettings {
            run: SettingsRun::new(),
            preset: SettingsPreset::new(),
            snooze: SettingsSnooze::new(),
            advanced: SettingsAdvanced::new(),
            mode: SettingsMode::new(),
            end_of_line: SettingsEndOfLine::new(),
        }
    }

    pub fn new_settings_event(&mut self, event: ChipSettingsEvent) -> Vec<ControlMessage> {
        match event {
            ChipSettingsEvent::Preset(event) => self.preset.new_event(event),
            ChipSettingsEvent::Run(event) => self.run.new_event(event),
            ChipSettingsEvent::Snooze(event) => self.snooze.new_event(event),
            ChipSettingsEvent::Mode(event) => self.mode.new_event(event),
            ChipSettingsEvent::EndOfLine(event) => self.end_of_line.new_event(event),
        }
    }

    pub fn new_settings_intent(&mut self, intent: ChipSettingsIntent) {
        match intent {
            ChipSettingsIntent::Mode(intent) => self.mode.new_intent(intent),
        }
    }
}
