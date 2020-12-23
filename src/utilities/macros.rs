// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

macro_rules! gen_number_threshold_check {
    ($value:ident, $default:expr, [$($voltage:expr => $soc:expr,)+]) => {{
        $(
            if $value >= $voltage {
                return $soc;
            }
        )+

        $default
    }};
}
