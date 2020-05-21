// MakAir
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use crate::config::environment::*;
use quick_error;
use rn2903::Rn2903;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use telemetry::structures::{AlarmPriority, DataSnapshot, MachineStateSnapshot, TelemetryMessage};

use std::sync::mpsc::channel;
use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

quick_error! {
    #[derive(Debug)]
    pub enum LoraControllerError {
        InitError {}
    }
}

pub struct LoraController {
    txvr: Rn2903,
    //lock: Arc<Rn2903>,
}

impl LoraController {
    pub fn new() {
        let (tx, rx) = channel();

        std::thread::spawn(move || {
            sleep(Duration::from_millis(2000));
            let mut txvr = None;
            while txvr.is_none() {
                let mylora = Pin::new(LORA_GPIO_PIN_NUMBER); // number depends on chip, etc.
                mylora
                    .with_exported(|| {
                        println!("set the pin direction");

                        mylora.set_direction(Direction::Out).unwrap();
                        println!("set the pin low");

                        mylora.set_value(0).unwrap();

                        sleep(Duration::from_millis(400));
                        println!("set the pin high");

                        mylora.set_value(1).unwrap();
                        sleep(Duration::from_millis(400));
                        Ok(());
                    })
                    .unwrap();

                match Rn2903::new_at(LORA_DEVICE_PATH) {
                    Ok(mut txvr2) => {
                        txvr = Some(txvr2);
                        txvr.mac_pause().unwrap();
                    }
                    Err(v) => return Err(LoraControllerError::InitError),
                }
                sleep(Duration::from_millis(15000));
            }
        });
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
