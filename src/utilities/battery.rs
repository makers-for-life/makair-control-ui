// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub fn estimate_lead_acid_12v_soc(voltage: f64) -> u16 {
    // Notice: this is a rough estimation of the battery SoC for a lead-acid battery, regardless \
    //   of the discharge rate, temperature and ageing of the battery. Super rough, but gives \
    //   an estimation of the battery SoC for the end-user, when running on battery. This is \
    //   based on threshold points taken from a typical lead-acid battery discharge curve, used \
    //   in nominal conditions at 1C at 20C.

    // Fallback on 0% SoC if no threshold is reached
    gen_number_threshold_check!(voltage, 0, [
        13.0 => 100,
        12.75 => 85,
        12.5 => 75,
        12.25 => 65,
        12.0 => 55,
        11.75 => 50,
        11.5 => 40,
        11.25 => 35,
        11.0 => 25,
        10.75 => 15,
        10.5 => 10,
        10.25 => 5,
    ])
}
