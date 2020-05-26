pub mod trigger_inspiratory;

use trigger_inspiratory::{TriggerInspiratory, TriggerInspiratoryEvent};

#[derive(Debug)]
pub struct ChipSettings {
    pub inspiratory_trigger: TriggerInspiratory
}

impl ChipSettings {
    pub fn new() -> ChipSettings {
        ChipSettings {
            inspiratory_trigger: TriggerInspiratory::new()
        }
    }

    pub fn new_settings_event(&mut self, event: ChipSettingsEvent) {
        match event {
            ChipSettingsEvent::InspiratoryTrigger(event) => self.inspiratory_trigger.new_event(event)
        };
    }
}

#[derive(Debug)]
pub enum SettingAction {
    More,
    Less
}

#[derive(Debug)]
pub enum ChipSettingsEvent {
    InspiratoryTrigger(TriggerInspiratoryEvent),
}