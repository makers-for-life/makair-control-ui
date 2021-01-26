// MakAir Control UI
//
// Copyright: 2021, Makers For Life
// License: Public Domain License

use makair_telemetry::control::{ControlMessage, ControlSetting};
use makair_telemetry::structures::PatientGender;

use crate::chip::settings::SettingActionRange;

const SIZE_STEP: usize = 1;

const SIZE_BASE_CHILD: usize = 110;
const SIZE_BASE_TEENAGER: usize = 150;
const SIZE_BASE_ADULT: usize = 170;

const SIZE_FALLBACK_DEFAULT: usize = SIZE_BASE_ADULT;

#[derive(Debug)]
pub enum SettingsPresetEvent {
    CommitIgnore,
    CommitSubmit,
}

#[derive(Debug)]
pub struct SettingsPreset {
    pub gender: SettingsPresetGender,
    pub age: SettingsPresetAge,
    pub size: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum SettingsPresetGender {
    Male,
    Female,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SettingsPresetAge {
    Child,
    Teenager,
    Adult,
}

impl Default for SettingsPresetGender {
    fn default() -> Self {
        Self::Male
    }
}

impl Default for SettingsPresetAge {
    fn default() -> Self {
        Self::Adult
    }
}

impl SettingsPresetGender {
    fn list_all() -> [Self; 2] {
        [Self::Male, Self::Female]
    }
}

impl From<&PatientGender> for SettingsPresetGender {
    fn from(gender: &PatientGender) -> Self {
        match gender {
            PatientGender::Male => Self::Male,
            PatientGender::Female => Self::Female,
        }
    }
}

impl From<&SettingsPresetGender> for PatientGender {
    fn from(gender: &SettingsPresetGender) -> Self {
        match gender {
            SettingsPresetGender::Male => Self::Male,
            SettingsPresetGender::Female => Self::Female,
        }
    }
}

impl SettingsPresetAge {
    fn list_all() -> [Self; 3] {
        [Self::Child, Self::Teenager, Self::Adult]
    }

    fn base_size(&self) -> usize {
        match self {
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

        SettingsPreset {
            gender: SettingsPresetGender::default(),
            age,
            size,
        }
    }

    pub fn new_event(&self, event: SettingsPresetEvent) -> Vec<ControlMessage> {
        match event {
            SettingsPresetEvent::CommitIgnore => self.commit(false),
            SettingsPresetEvent::CommitSubmit => self.commit(true),
        }
    }

    pub fn commit(&self, submit: bool) -> Vec<ControlMessage> {
        let mut events = Vec::new();

        // Acquire selected gender and size values, or use defaults?
        let (patient_gender, patient_size) = if submit {
            (self.gender, self.size)
        } else {
            (SettingsPresetGender::default(), SIZE_FALLBACK_DEFAULT)
        };

        // Acquire internal value for gender
        let patient_gender_internal = PatientGender::from(&patient_gender);

        // Append all events
        events.push(ControlMessage {
            setting: ControlSetting::PatientGender,
            value: u8::from(&patient_gender_internal) as _,
        });
        events.push(ControlMessage {
            setting: ControlSetting::PatientHeight,
            value: patient_size as _,
        });

        events
    }

    pub fn switch_gender(&mut self, action: SettingActionRange) {
        let genders = SettingsPresetGender::list_all();
        let genders_size = genders.len() as i16;

        if genders_size > 1 {
            // Get index of current gender in list of genders
            let current_index = genders
                .iter()
                .position(|gender| gender == &self.gender)
                .unwrap_or(0);

            // Increment or decrement next gender index
            let next_index = action.to_next_index(current_index as _);

            if next_index >= 0 && next_index < genders_size {
                self.gender = genders[next_index as usize].to_owned();
            }
        }
    }

    pub fn switch_age(&mut self, action: SettingActionRange) {
        let ages = SettingsPresetAge::list_all();
        let ages_size = ages.len() as i16;

        if ages_size > 1 {
            // Get index of current age in list of ages
            let current_index = ages.iter().position(|age| age == &self.age).unwrap_or(0);

            // Increment or decrement next age index
            let next_index = action.to_next_index(current_index as _);

            if next_index >= 0 && next_index < ages_size {
                // Update age
                self.age = ages[next_index as usize].to_owned();

                // Re-assign base size (age group changed as a shortcut to prefill the size)
                self.size = self.age.base_size();
            }
        }
    }

    pub fn change_size(&mut self, action: SettingActionRange) {
        self.size = action.to_new_value(&ControlSetting::PatientHeight, self.size, SIZE_STEP);
    }
}
