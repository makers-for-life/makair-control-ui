// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub mod trigger;

use telemetry::control::ControlMessage;
use trigger::{Trigger, TriggerEvent};

#[derive(Debug)]
pub struct ChipSettings {
    pub inspiratory_trigger: Trigger,
}

impl ChipSettings {
    pub fn new(cycles_per_minute: usize) -> ChipSettings {
        ChipSettings {
            inspiratory_trigger: Trigger::new(cycles_per_minute),
        }
    }

    pub fn new_settings_event(&mut self, event: ChipSettingsEvent) -> ControlMessage {
        match event {
            ChipSettingsEvent::InspiratoryTrigger(event) => {
                self.inspiratory_trigger.new_event(event)
            }
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
    InspiratoryTrigger(TriggerEvent),
}
