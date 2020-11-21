// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

macro_rules! gen_set_new_value {
    ($setting:expr, $action:ident, $value:expr, $step:ident) => {{
        let new_value = $action.to_new_value(&$setting, $value, $step);

        return ControlMessage {
            setting: $setting,
            value: new_value as u16,
        };
    }};
}
