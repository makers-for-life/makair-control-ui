// MakAir
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use quick_error;
use rn2903::Rn2903;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use telemetry::structures::{AlarmPriority, DataSnapshot, MachineStateSnapshot, TelemetryMessage};

quick_error! {
    #[derive(Debug)]
    pub enum LoraControllerError {
        InitError {}
    }
}

pub struct LoraController {
    port: String,
    txvr: Rn2903,
    //lock: Arc<Rn2903>,
}

impl LoraController {
    pub fn new() -> Result<Self, LoraControllerError> {
        let port = "/dev/ttyAMA0";
        match Rn2903::new_at("/dev/ttyAMA0") {
            Ok(mut txvr) => {
                txvr.mac_pause().unwrap();
                return Ok(Self {
                    port: port.to_string(),
                    txvr: txvr,
                    //lock: Arc::new(txvr),
                });
            }
            Err(v) => return Err(LoraControllerError::InitError),
        }
    }
    pub fn sendMessage(&mut self, mss: MachineStateSnapshot) {
        /*  let lock2 = self.lock.clone();
        std::thread::spawn(move || {*/
        let message = format!(
            "{},{},{}",
            mss.previous_peak_pressure, mss.previous_peep_pressure, mss.previous_plateau_pressure
        );
        self.txvr.radio_tx(message);
        // });
    }

    pub fn sendHello(&mut self) {
        self.txvr.radio_tx("hello".to_string());
    }
}
