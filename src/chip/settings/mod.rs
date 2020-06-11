pub mod trigger_inspiratory;

use telemetry::control::ControlMessage;
use trigger_inspiratory::{TriggerInspiratory, TriggerInspiratoryEvent};

#[derive(Debug)]
pub struct ChipSettings {
    pub inspiratory_trigger: TriggerInspiratory,
}

impl ChipSettings {
    pub fn new(cycles_per_minute: usize) -> ChipSettings {
        ChipSettings {
            inspiratory_trigger: TriggerInspiratory::new(cycles_per_minute),
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
    InspiratoryTrigger(TriggerInspiratoryEvent),
}
