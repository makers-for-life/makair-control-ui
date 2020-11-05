// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use crate::config::environment::*;

pub fn process_max_allowed_pressure(peak_command: u8) -> u8 {
    let mut max_pressure = if peak_command > 0 {
        peak_command as f64
    } else {
        MAX_ALLOWED_PRESSURE_INITIAL_MINIMUM
    };

    max_pressure = max_pressure + max_pressure * MAX_ALLOWED_PRESSURE_ALERT_ERROR_RATIO;

    max_pressure as u8
}
