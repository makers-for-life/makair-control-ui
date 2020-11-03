// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::time::Instant;

pub struct ConfigContext {
    pub start_time: Instant,
}

impl ConfigContext {
    pub fn make() -> Self {
        ConfigContext {
            start_time: Instant::now(),
        }
    }
}
