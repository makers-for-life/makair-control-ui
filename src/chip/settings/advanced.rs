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
    #[cfg(feature = "simulator")]
    pub resistance: i32,
    #[cfg(feature = "simulator")]
    pub compliance: i32,
    #[cfg(feature = "simulator")]
    pub spontaneous_breath_rate: i32,
    #[cfg(feature = "simulator")]
    pub spontaneous_breath_effort: i32,
    #[cfg(feature = "simulator")]
    pub spontaneous_breath_duration: i32,
    #[cfg(feature = "simulator")]
    pub acceleration_percent: i32,
}

#[derive(Debug, PartialEq)]
pub enum SettingsAdvancedGroupTab {
    Statistics,
    Settings,
    #[cfg(feature = "simulator")]
    Simulator,
}

impl SettingsAdvancedGroupTab {
    pub fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::Statistics),
            1 => Some(Self::Settings),
            #[cfg(feature = "simulator")]
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
            #[cfg(feature = "simulator")]
            resistance: makair_simulator::SimulatorSettingKind::Resistance.default(),
            #[cfg(feature = "simulator")]
            compliance: makair_simulator::SimulatorSettingKind::Compliance.default(),
            #[cfg(feature = "simulator")]
            spontaneous_breath_rate: makair_simulator::SimulatorSettingKind::SpontaneousBreathRate
                .default(),
            #[cfg(feature = "simulator")]
            spontaneous_breath_effort:
                makair_simulator::SimulatorSettingKind::SpontaneousBreathEffort.default(),
            #[cfg(feature = "simulator")]
            spontaneous_breath_duration:
                makair_simulator::SimulatorSettingKind::SpontaneousBreathDuration.default(),
            #[cfg(feature = "simulator")]
            acceleration_percent: makair_simulator::SimulatorSettingKind::AccelerationPercent
                .default(),
        }
    }

    #[cfg(feature = "simulator")]
    pub fn new_event(
        &self,
        event: makair_simulator::SimulatorSettingKind,
    ) -> Vec<makair_simulator::SimulatorSetting> {
        use makair_simulator::{SimulatorSetting, SimulatorSettingKind};

        let value = match event {
            SimulatorSettingKind::AccelerationPercent => self.acceleration_percent,
            SimulatorSettingKind::Resistance => self.resistance,
            SimulatorSettingKind::Compliance => self.compliance,
            SimulatorSettingKind::SpontaneousBreathRate => self.spontaneous_breath_rate,
            SimulatorSettingKind::SpontaneousBreathEffort => self.spontaneous_breath_effort,
            SimulatorSettingKind::SpontaneousBreathDuration => self.spontaneous_breath_duration,
        };

        vec![SimulatorSetting { kind: event, value }]
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

    #[cfg(feature = "simulator")]
    pub fn switch_resistance(&mut self, action: super::SimulatorSettingActionRange) {
        let current_resistance = makair_simulator::SimulatorSetting {
            kind: makair_simulator::SimulatorSettingKind::Resistance,
            value: self.resistance,
        };
        let next_resistance = makair_simulator::SimulatorSetting {
            kind: makair_simulator::SimulatorSettingKind::Resistance,
            value: action.to_new_value(&current_resistance),
        };

        if next_resistance != current_resistance {
            self.resistance = next_resistance.value;
        }
    }

    #[cfg(feature = "simulator")]
    pub fn switch_compliance(&mut self, action: super::SimulatorSettingActionRange) {
        let current_compliance = makair_simulator::SimulatorSetting {
            kind: makair_simulator::SimulatorSettingKind::Compliance,
            value: self.compliance,
        };
        let next_compliance = makair_simulator::SimulatorSetting {
            kind: makair_simulator::SimulatorSettingKind::Compliance,
            value: action.to_new_value(&current_compliance),
        };

        if next_compliance != current_compliance {
            self.compliance = next_compliance.value;
        }
    }

    #[cfg(feature = "simulator")]
    pub fn switch_spontaneous_breath_rate(&mut self, action: super::SimulatorSettingActionRange) {
        let current_spontaneous_breath_rate = makair_simulator::SimulatorSetting {
            kind: makair_simulator::SimulatorSettingKind::SpontaneousBreathRate,
            value: self.spontaneous_breath_rate,
        };
        let next_spontaneous_breath_rate = makair_simulator::SimulatorSetting {
            kind: makair_simulator::SimulatorSettingKind::SpontaneousBreathRate,
            value: action.to_new_value(&current_spontaneous_breath_rate),
        };

        if next_spontaneous_breath_rate != current_spontaneous_breath_rate {
            self.spontaneous_breath_rate = next_spontaneous_breath_rate.value;
        }
    }

    #[cfg(feature = "simulator")]
    pub fn switch_spontaneous_breath_effort(&mut self, action: super::SimulatorSettingActionRange) {
        let current_spontaneous_breath_effort = makair_simulator::SimulatorSetting {
            kind: makair_simulator::SimulatorSettingKind::SpontaneousBreathEffort,
            value: self.spontaneous_breath_effort,
        };
        let next_spontaneous_breath_effort = makair_simulator::SimulatorSetting {
            kind: makair_simulator::SimulatorSettingKind::SpontaneousBreathEffort,
            value: action.to_new_value(&current_spontaneous_breath_effort),
        };

        if next_spontaneous_breath_effort != current_spontaneous_breath_effort {
            self.spontaneous_breath_effort = next_spontaneous_breath_effort.value;
        }
    }

    #[cfg(feature = "simulator")]
    pub fn switch_spontaneous_breath_duration(
        &mut self,
        action: super::SimulatorSettingActionRange,
    ) {
        let current_spontaneous_breath_duration = makair_simulator::SimulatorSetting {
            kind: makair_simulator::SimulatorSettingKind::SpontaneousBreathDuration,
            value: self.spontaneous_breath_duration,
        };
        let next_spontaneous_breath_duration = makair_simulator::SimulatorSetting {
            kind: makair_simulator::SimulatorSettingKind::SpontaneousBreathDuration,
            value: action.to_new_value(&current_spontaneous_breath_duration),
        };

        if next_spontaneous_breath_duration != current_spontaneous_breath_duration {
            self.spontaneous_breath_duration = next_spontaneous_breath_duration.value;
        }
    }

    #[cfg(feature = "simulator")]
    pub fn switch_acceleration_percent(&mut self, action: super::SimulatorSettingActionRange) {
        let current_acceleration_percent = makair_simulator::SimulatorSetting {
            kind: makair_simulator::SimulatorSettingKind::AccelerationPercent,
            value: self.acceleration_percent,
        };
        let next_acceleration_percent = makair_simulator::SimulatorSetting {
            kind: makair_simulator::SimulatorSettingKind::AccelerationPercent,
            value: action.to_new_value(&current_acceleration_percent),
        };

        if next_acceleration_percent != current_acceleration_percent {
            self.acceleration_percent = next_acceleration_percent.value;
        }
    }
}
