// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use makair_telemetry::alarm::AlarmCodeDescription;

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
        AlarmCodeDescription::InspiratoryMinuteVolumeLow => "inspiratory-minute-volume-low",
        AlarmCodeDescription::InspiratoryMinuteVolumeHigh => "inspiratory-minute-volume-high",
        AlarmCodeDescription::ExpiratoryMinuteVolumeLow => "expiratory-minute-volume-low",
        AlarmCodeDescription::ExpiratoryMinuteVolumeHigh => "expiratory-minute-volume-high",
        AlarmCodeDescription::RespiratoryRateLow => "respiratory-rate-low",
        AlarmCodeDescription::RespiratoryRateHigh => "respiratory-rate-high",
        AlarmCodeDescription::LeakHigh => "leak-high",
        AlarmCodeDescription::TidalVolumeLow => "tidal-volume-low",
        AlarmCodeDescription::TidalVolumeHigh => "tidal-volume-high",
        AlarmCodeDescription::PeakPressureHigh => "peak-pressure-high",
        AlarmCodeDescription::Unknown(_) => "unknown",
    };

    APP_I18N.t(&format!("alarms-message-{}", locale_key))
}
