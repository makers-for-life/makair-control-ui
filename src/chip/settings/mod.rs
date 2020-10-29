// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub mod expiration_term;
pub mod trigger;

use telemetry::control::ControlMessage;

use expiration_term::*;
use trigger::*;

#[derive(Debug)]
pub struct ChipSettings {
    pub trigger: SettingsTrigger,
    pub expiration_term: SettingsExpirationTerm,
}

impl ChipSettings {
    pub fn new(cycles_per_minute: usize) -> ChipSettings {
        ChipSettings {
            trigger: SettingsTrigger::new(),
            expiration_term: SettingsExpirationTerm::new(cycles_per_minute),
        }
    }

    pub fn new_settings_event(&mut self, event: ChipSettingsEvent) -> ControlMessage {
        match event {
            ChipSettingsEvent::Trigger(event) => self.trigger.new_event(event),
            ChipSettingsEvent::ExpirationTerm(event) => self.expiration_term.new_event(event),
        }
    }
}

#[derive(Debug)]
pub enum SettingAction {
    More,
    Less,
}

#[derive(Debug)]
pub enum ChipSettingsEvent {
    Trigger(SettingsTriggerEvent),
    ExpirationTerm(SettingsExpirationTermEvent),
}
