// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use telemetry::alarm::AlarmCodeDescription;

use crate::APP_I18N;

pub fn description_to_locale(description: AlarmCodeDescription) -> String {
    let locale_key = match description {
        AlarmCodeDescription::PlateauPressureNotReached => "plateau-pressure-not-reached",
        AlarmCodeDescription::PatientUnplugged => "patient-unplugged",
        AlarmCodeDescription::PEEPPressureNotReached => "peep-pressure-not-reached",
        AlarmCodeDescription::BatteryLow => "battery-low",
        AlarmCodeDescription::BatteryVeryLow => "battery-very-low",
        AlarmCodeDescription::PowerCableUnplugged => "power-cable-unplugged",
        AlarmCodeDescription::PressureTooHigh => "pressure-too-high",
        AlarmCodeDescription::Unknown(_) => "unknown",
    };

    APP_I18N.t(&format!("alarms-message-{}", locale_key))
}
