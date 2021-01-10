// MakAir Control UI
//
// Copyright: 2021, Makers For Life
// License: Public Domain License

const SIZE_BASE_BABY: u8 = 50;
const SIZE_BASE_CHILD: u8 = 110;
const SIZE_BASE_TEENAGER: u8 = 150;
const SIZE_BASE_ADULT: u8 = 170;

#[derive(Debug)]
pub struct SettingsPreset {
    pub age: SettingsPresetAge,
    pub size: u8,
}

impl SettingsPreset {
    pub fn new() -> SettingsPreset {
        let age = SettingsPresetAge::default();
        let size = age.base_size();

        SettingsPreset { age, size }
    }
}

#[derive(Debug, PartialEq)]
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
    fn base_size(&self) -> u8 {
        match self {
            Self::Baby => SIZE_BASE_BABY,
            Self::Child => SIZE_BASE_CHILD,
            Self::Teenager => SIZE_BASE_TEENAGER,
            Self::Adult => SIZE_BASE_ADULT,
        }
    }
}
