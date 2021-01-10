// MakAir Control UI
//
// Copyright: 2021, Makers For Life
// License: Public Domain License

use crate::chip::settings::preset::SettingsPresetAge;

use crate::APP_I18N;

pub fn age_to_locale(age: &SettingsPresetAge) -> String {
    let locale_key = match age {
        SettingsPresetAge::Baby => "baby",
        SettingsPresetAge::Child => "child",
        SettingsPresetAge::Teenager => "teenager",
        SettingsPresetAge::Adult => "adult",
    };

    APP_I18N.t(&format!("modal-preset-age-{}", locale_key))
}
