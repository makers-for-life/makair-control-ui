// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::cmp::{max, min};

use makair_telemetry::alarm::{AlarmCode, RMC_SW_11, RMC_SW_12, RMC_SW_16};
use makair_telemetry::structures::AlarmPriority;

pub fn estimate_lead_acid_12v_2s_soc(voltage: f64, is_running: bool, blower_ppm: usize) -> u8 {
    // Notice: this is a rough estimation of the battery SoC for a lead-acid battery, regardless \
    //   of the discharge rate, temperature and ageing of the battery. Super rough, but gives \
    //   an estimation of the battery SoC for the end-user, when running on battery. This is \
    //   based on threshold points taken from a typical lead-acid battery discharge curve, used \
    //   in nominal conditions at C/5 in a room at 20C.

    // Apply an empiric voltage correction, based on current system load (estimated based on the \
    //   blower PPM speed, which is the most power hungry component of the system). The battery \
    //   measurements were done on a system with its blower running at full speed. Hence, a 180ppm \
    //   reference is used in the equation. A machine in stopped state or with a slower blower \
    //   speed will tend to show a higher battery voltage, as the power load is much reduced. \
    //   Thus, we need to apply a correction factor to lower this measured voltage, so that it \
    //   complies with the Voltage-to-SoC equation requirements below.
    let corrected_voltage = if is_running && blower_ppm > 0 {
        voltage - (0.327 - 0.0018 * blower_ppm as f64)
    } else {
        voltage - 0.3
    };

    // Kt equation for a 2S-12V-7Ah PbAc battery pack is: \
    //   SoC = 5,84 × POWER(VOLTAGE; 2) − 235,307 × VOLTAGE + 2355,3
    let unchecked_percent =
        5.84 * corrected_voltage.powf(2.0) - 235.307 * corrected_voltage + 2355.3;

    // Apply boundaries to the calculated percent value, as it may go into the negatives (eg. -1%) \
    //   or too high in positives (eg. 101%) at the extremes. Though it will not overflow \
    //   further than 1%-2% under nominal battery conditions.
    min(100, max(0, unchecked_percent as i8)) as u8
}

pub fn power_status_flags(alarms: &[(AlarmCode, AlarmPriority)]) -> &str {
    // Flags:
    //   - OB  = On: Battery
    //   - OL  = On: Line
    //   - ABL = Alarm: Battery Low
    //   - ABC = Alarm: Battery Critical

    // Map alarm code numbers
    let alarm_codes: Vec<u8> = alarms.iter().map(|alarm| alarm.0.code()).collect();

    // On battery, or on AC?
    if alarm_codes.contains(&RMC_SW_16) {
        if alarm_codes.contains(&RMC_SW_12) {
            // Battery very low
            "OB ABC"
        } else if alarm_codes.contains(&RMC_SW_11) {
            // Battery low
            "OB ABL"
        } else {
            // Battery charge is OK
            "OB"
        }
    } else {
        "OL"
    }
}
