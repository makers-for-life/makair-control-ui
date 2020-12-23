// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use confy;

use super::environment::RUNTIME_NAME;

#[derive(Serialize, Deserialize)]
pub struct ConfigSettings {
    pub locale: String,
}

impl Default for ConfigSettings {
    fn default() -> Self {
        Self {
            locale: "en".to_string(),
        }
    }
}

impl ConfigSettings {
    pub fn read() -> Self {
        if let Ok(configuration) = confy::load(RUNTIME_NAME) {
            configuration
        } else {
            Self::default()
        }
    }
}
