// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

pub enum ConvertMode {
    Rounded,
    WithDecimals,
}

pub fn convert_mmh2o_to_cmh2o(mode: ConvertMode, value: f64) -> f64 {
    match mode {
        ConvertMode::WithDecimals => value / 10.0,
        ConvertMode::Rounded => (value / 10.0).round(),
    }
}
