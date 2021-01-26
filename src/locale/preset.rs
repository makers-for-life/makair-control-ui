// MakAir Control UI
//
// Copyright: 2021, Makers For Life
// License: Public Domain License

use crate::chip::settings::preset::{SettingsPresetAge, SettingsPresetGender};

use crate::APP_I18N;

pub fn gender_to_locale(gender: &SettingsPresetGender) -> String {
    let locale_key = match gender {
        SettingsPresetGender::Male => "male",
        SettingsPresetGender::Female => "female",
    };

    APP_I18N.t(&format!("modal-preset-gender-{}", locale_key))
}

pub fn age_to_locale(age: &SettingsPresetAge) -> String {
    let locale_key = match age {
        SettingsPresetAge::Child => "child",
        SettingsPresetAge::Teenager => "teenager",
        SettingsPresetAge::Adult => "adult",
    };

    APP_I18N.t(&format!("modal-preset-age-{}", locale_key))
}
