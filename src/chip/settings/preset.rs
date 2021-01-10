// MakAir Control UI
//
// Copyright: 2021, Makers For Life
// License: Public Domain License

use crate::chip::settings::SettingActionRange;

const SIZE_BASE_BABY: u8 = 50;
const SIZE_BASE_CHILD: u8 = 110;
const SIZE_BASE_TEENAGER: u8 = 150;
const SIZE_BASE_ADULT: u8 = 170;

#[derive(Debug)]
pub struct SettingsPreset {
    pub age: SettingsPresetAge,
    pub size: u8,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SettingsPresetAge {
    Baby,
    Child,
    Teenager,
    Adult,
}

impl Default for SettingsPresetAge {
    fn default() -> Self {
        Self::Adult
    }
}

impl SettingsPresetAge {
    fn list_all() -> [Self; 4] {
        [Self::Baby, Self::Child, Self::Teenager, Self::Adult]
    }

    fn base_size(&self) -> u8 {
        match self {
            Self::Baby => SIZE_BASE_BABY,
            Self::Child => SIZE_BASE_CHILD,
            Self::Teenager => SIZE_BASE_TEENAGER,
            Self::Adult => SIZE_BASE_ADULT,
        }
    }
}

impl SettingsPreset {
    pub fn new() -> SettingsPreset {
        let age = SettingsPresetAge::default();
        let size = age.base_size();

        SettingsPreset { age, size }
    }

    pub fn switch_age(&mut self, action: SettingActionRange) {
        let ages = SettingsPresetAge::list_all();
        let ages_size = ages.len() as i16;

        if ages_size > 1 {
            // Get index of current age in list of ages
            let current_index = ages.iter().position(|age| age == &self.age).unwrap_or(0);

            // Increment or decrement next age index
            let next_index = current_index as i16
                + match action {
                    SettingActionRange::Less => -1,
                    SettingActionRange::More => 1,
                };

            if next_index >= 0 && next_index < ages_size {
                // Update age
                self.age = ages[next_index as usize].to_owned();

                // Re-assign base size (age group changed as a shortcut to prefill the size)
                self.size = self.age.base_size();
            }
        }
    }

    pub fn change_height(&mut self, action: SettingActionRange) {
        // TODO
    }
}
