// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

macro_rules! gen_override_snapshot_values_from_stopped {
    ($snapshot:expr, $stopped:expr, [$($key:ident),+]) => {
        $(
            if let Some(value) = $stopped.$key {
                $snapshot.$key = value;
            }
        )+
    };
}
