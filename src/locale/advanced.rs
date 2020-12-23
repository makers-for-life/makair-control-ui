// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use crate::chip::settings::advanced::SettingsAdvancedGroupTab;

use crate::APP_I18N;

pub fn group_tab_to_locale(group_tab: SettingsAdvancedGroupTab) -> String {
    let locale_key = match group_tab {
        SettingsAdvancedGroupTab::Statistics => "statistics",
        SettingsAdvancedGroupTab::Settings => "settings",
    };

    APP_I18N.t(&format!("advanced-group-{}", locale_key))
}
