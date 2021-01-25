// MakAir Control UI
//
// Copyright: 2021, Makers For Life
// License: Public Domain License

use crate::chip::ChipError;

use crate::APP_I18N;

pub fn error_to_locales(error: &ChipError) -> (String, String) {
    // Acquire target locale attributes
    let (locale_key, error_details) = match error {
        ChipError::NoDevice => ("no-device", None),
        ChipError::TimedOut => ("timed-out", None),
        ChipError::BadProtocol => ("bad-protocol", None),
        ChipError::Watchdog => ("watchdog", None),
        ChipError::SensorFailure(ref details) => ("sensor-failure", Some(details)),
        ChipError::Other(ref details) => ("other", Some(details)),
    };

    // Acquire title
    let title = APP_I18N.t(&format!("error-title-{}", locale_key));

    // Generate full message (with optional details, if any)
    let mut message = APP_I18N.t(&format!("error-message-{}", locale_key));

    if let Some(error_details) = error_details {
        message.push_str(" '");
        message.push_str(error_details);
        message.push('\'');
    }

    (title, message)
}
