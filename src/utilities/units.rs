// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

const CONVERT_RATIO_MMH2O_TO_CMH2O: f64 = 10.0;
const CONVERT_RATIO_CL_TO_ML: f64 = 0.1;
const CONVERT_RATIO_ML_TO_L: f64 = 1000.0;
const CONVERT_RATIO_CV_TO_V: f64 = 100.0;

pub enum ConvertMode {
    Rounded,
    WithDecimals,
}

pub fn convert_mmh2o_to_cmh2o(mode: ConvertMode, value: f64) -> f64 {
    match mode {
        ConvertMode::WithDecimals => value / CONVERT_RATIO_MMH2O_TO_CMH2O,
        ConvertMode::Rounded => (value / CONVERT_RATIO_MMH2O_TO_CMH2O).round(),
    }
}

pub fn convert_cmh2o_to_mmh2o(value: u8) -> usize {
    (value as usize) * 10
}

pub fn convert_sub_ppm_to_ppm(value: u8) -> usize {
    (value as usize) * 10
}

pub fn convert_cl_to_ml(mode: ConvertMode, value: f64) -> f64 {
    match mode {
        ConvertMode::WithDecimals => value / CONVERT_RATIO_CL_TO_ML,
        ConvertMode::Rounded => (value / CONVERT_RATIO_CL_TO_ML).round(),
    }
}

pub fn convert_ml_to_l(mode: ConvertMode, value: f64) -> f64 {
    match mode {
        ConvertMode::WithDecimals => value / CONVERT_RATIO_ML_TO_L,
        ConvertMode::Rounded => (value / CONVERT_RATIO_ML_TO_L).round(),
    }
}

pub fn convert_cv_to_v(mode: ConvertMode, value: f64) -> f64 {
    match mode {
        ConvertMode::WithDecimals => value / CONVERT_RATIO_CV_TO_V,
        ConvertMode::Rounded => (value / CONVERT_RATIO_CV_TO_V).round(),
    }
}
