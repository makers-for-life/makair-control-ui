// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use confy::{self, ConfyError};

use crate::locale::locales::LocaleCode;

use super::environment::RUNTIME_NAME;

#[derive(Serialize, Deserialize)]
pub struct ConfigSettings {
    pub locale: String,
}

pub enum ConfigSettingsUpdateMay {
    ShouldSave,
    NoChange,
}

impl Default for ConfigSettings {
    fn default() -> Self {
        Self {
            locale: LocaleCode::default().to_code().to_string(),
        }
    }
}

impl ConfigSettings {
    pub fn read() -> Self {
        if let Ok(configuration) = confy::load::<Self>(RUNTIME_NAME) {
            Self::check_configuration(configuration)
        } else {
            Self::default()
        }
    }

    pub fn save(&self) -> Result<(), ConfyError> {
        confy::store(RUNTIME_NAME, self)
    }

    pub fn set_locale(&mut self, locale: String) -> ConfigSettingsUpdateMay {
        if self.locale != locale {
            self.locale = locale;

            ConfigSettingsUpdateMay::ShouldSave
        } else {
            ConfigSettingsUpdateMay::NoChange
        }
    }

    fn check_configuration(mut configuration: Self) -> Self {
        // Ensure configuration is still valid
        // Notice: as the UI may be resumed from an old saved state, some saved configuration \
        //   values may not be valid anymore, hence this check to ensure their validity.
        if LocaleCode::from_code(&configuration.locale.as_str()).is_none() {
            configuration.locale = LocaleCode::default().to_code().to_string();
        }

        configuration
    }
}
