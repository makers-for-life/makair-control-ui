// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

#[derive(Debug)]
pub struct SettingsAdvanced {
    pub group: SettingsAdvancedGroupTab,
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
        }
    }
}
