// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use crate::chip::settings::SettingActionRange;
use crate::locale::{loader::LocaleLoader, locales::LocaleCode};
use crate::{APP_I18N, APP_SETTINGS};

#[derive(Debug)]
pub struct SettingsAdvanced {
    pub group: SettingsAdvancedGroupTab,
    pub locale: LocaleCode,
    pub resistance: i32,
    pub compliance: i32,
    pub spontaneous_breath_rate: i32,
    pub spontaneous_breath_effort: i32,
    pub spontaneous_breath_duration: i32,
    pub acceleration_factor: f32,
}

#[derive(Debug, PartialEq)]
pub enum SettingsAdvancedGroupTab {
    Statistics,
    Settings,
    Simulator,
}

impl SettingsAdvancedGroupTab {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Statistics),
            1 => Some(Self::Settings),
            2 => Some(Self::Simulator),
            _ => None,
        }
    }
}

impl Default for SettingsAdvancedGroupTab {
    fn default() -> Self {
        Self::Statistics
    }
}

impl SettingsAdvanced {
    pub fn new() -> SettingsAdvanced {
        SettingsAdvanced {
            group: SettingsAdvancedGroupTab::default(),
            locale: LocaleCode::from_code(&APP_SETTINGS.read().unwrap().locale).unwrap_or_default(),
            resistance: 1,                  // TODO get value from simulator
            compliance: 1,                  // TODO get value from simulator
            spontaneous_breath_rate: 0,     // TODO get value from simulator
            spontaneous_breath_effort: 0,   // TODO get value from simulator
            spontaneous_breath_duration: 0, // TODO get value from simulator
            acceleration_factor: 1.0,       // TODO get value from simulator
        }
    }

    pub fn switch_locale(&mut self, action: SettingActionRange) {
        let locales = LocaleCode::list_available();
        let locales_size = locales.len() as i16;

        if locales_size > 1 {
            // Get index of current locale in list of locales
            let current_index = locales
                .iter()
                .position(|locale| locale == &self.locale)
                .unwrap_or(0);

            // Increment or decrement next locale index
            let mut next_index = action.to_next_index(current_index as _);

            // Roll circle?
            if next_index < 0 {
                next_index = locales_size - 1;
            } else if next_index >= locales_size {
                next_index = 0;
            }

            // Assign new locale value? (and save)
            let next_locale = locales[next_index as usize].to_owned();

            if next_locale != self.locale {
                self.locale = next_locale;

                // Save new locale value to settings
                APP_SETTINGS
                    .write()
                    .unwrap()
                    .set_locale(self.locale.to_code().to_string());

                match APP_SETTINGS.read().unwrap().save() {
                    Ok(_) => info!("saved locale in settings: {}", self.locale.to_code()),
                    Err(err) => error!("error saving locale in settings: {:?}", err),
                }

                // Replace current locale with new locale
                APP_I18N.replace(LocaleLoader::new(self.locale.to_code()).into_bundle());
            }
        }
    }

    pub fn switch_resistance(&mut self, action: SettingActionRange) {
        let next_resistance = action.to_next_index(self.resistance as _);
        // TODO : check if value is allowed and send it to simulator

        if next_resistance as i32 != self.resistance {
            self.resistance = next_resistance as i32;
        }
    }

    pub fn switch_compliance(&mut self, action: SettingActionRange) {
        let next_compliance = action.to_next_index(self.compliance as _);
        // TODO : check if value is allowed and send it to simulator

        if next_compliance as i32 != self.compliance {
            self.compliance = next_compliance as i32;
        }
    }

    pub fn switch_spontaneous_breath_rate(&mut self, action: SettingActionRange) {
        let next_spontaneous_breath_rate = action.to_next_index(self.spontaneous_breath_rate as _);
        // TODO : check if value is allowed and send it to simulator

        if next_spontaneous_breath_rate as i32 != self.spontaneous_breath_rate {
            self.spontaneous_breath_rate = next_spontaneous_breath_rate as i32;
        }
    }

    pub fn switch_spontaneous_breath_effort(&mut self, action: SettingActionRange) {
        let next_spontaneous_breath_effort =
            action.to_next_index(self.spontaneous_breath_effort as _);
        // TODO : check if value is allowed and send it to simulator

        if next_spontaneous_breath_effort as i32 != self.spontaneous_breath_effort {
            self.spontaneous_breath_effort = next_spontaneous_breath_effort as i32;
        }
    }

    pub fn switch_spontaneous_breath_duration(&mut self, action: SettingActionRange) {
        let next_spontaneous_breath_duration =
            action.to_next_index(self.spontaneous_breath_duration as _);
        // TODO : check if value is allowed and send it to simulator

        if next_spontaneous_breath_duration as i32 != self.spontaneous_breath_duration {
            self.spontaneous_breath_duration = next_spontaneous_breath_duration as i32;
        }
    }

    pub fn switch_acceleration_factor(&mut self, action: SettingActionRange) {
        let next_acceleration_factor = action.to_next_index(self.acceleration_factor as _);
        // TODO : check if value is allowed and send it to simulator

        if (next_acceleration_factor as f32 - self.acceleration_factor).abs() > f32::EPSILON {
            self.acceleration_factor = next_acceleration_factor as f32;
        }
    }
}
