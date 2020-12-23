// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use crate::chip::settings::SettingActionRange;
use crate::locale::{loader::LocaleLoader, locales::LOCALES};
use crate::{APP_I18N, APP_SETTINGS};

#[derive(Debug)]
pub struct SettingsAdvanced {
    pub group: SettingsAdvancedGroupTab,
    pub locale: String,
}

#[derive(Debug, PartialEq)]
pub enum SettingsAdvancedGroupTab {
    Statistics,
    Settings,
}

impl SettingsAdvancedGroupTab {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Statistics),
            1 => Some(Self::Settings),
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
            locale: APP_SETTINGS.read().unwrap().locale.to_owned(),
        }
    }

    pub fn switch_locale(&mut self, action: SettingActionRange) {
        let locales_size = LOCALES.len() as i16;

        if locales_size > 1 {
            // Get index of current locale in list of locales
            let current_index = LOCALES
                .iter()
                .position(|&locale| locale == self.locale)
                .unwrap_or(0);

            // Increment or decrement next locale index
            let mut next_index = current_index as i16
                + match action {
                    SettingActionRange::Less => -1,
                    SettingActionRange::More => 1,
                };

            // Roll circle?
            if next_index < 0 {
                next_index = locales_size - 1;
            } else if next_index >= locales_size {
                next_index = 0;
            }

            // Assign new locale value? (and save)
            let next_locale = LOCALES[next_index as usize].to_string();

            if next_locale != self.locale {
                self.locale = next_locale;

                // Save new locale value to settings
                APP_SETTINGS
                    .write()
                    .unwrap()
                    .set_locale(self.locale.to_owned());

                match APP_SETTINGS.read().unwrap().save() {
                    Ok(_) => info!("saved locale in settings: {}", self.locale),
                    Err(err) => error!("error saving locale in settings: {:?}", err),
                }

                // Replace current locale with new locale
                APP_I18N.replace(LocaleLoader::new(&self.locale).into_bundle());
            }
        }
    }
}
