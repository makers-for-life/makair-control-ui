// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::structures::{VentilationModeClass, VentilationModeKind};

use crate::APP_I18N;

pub fn class_to_locale(mode_class: VentilationModeClass) -> String {
    let locale_key = match mode_class {
        VentilationModeClass::Pressure => "pc",
        VentilationModeClass::Volume => "vc",
    };

    APP_I18N.t(&format!("mode-class-{}", locale_key))
}

pub fn kind_to_locale(mode_kind: VentilationModeKind) -> String {
    let locale_key = match mode_kind {
        VentilationModeKind::Cmv => "cmv",
        VentilationModeKind::Ac => "ac",
        VentilationModeKind::Bipap => "bipap",
    };

    APP_I18N.t(&format!("mode-type-{}", locale_key))
}
