// MakAir
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use crate::config::environment::*;
use rn2903::Rn2903;
use std::sync::mpsc;
use std::sync::mpsc::{channel, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use telemetry::structures::TelemetryMessage;

use std::thread::sleep;
use std::time::Duration;
use sysfs_gpio::{Direction, Pin};

pub struct LoraController {}

impl LoraController {
    pub fn new() -> Sender<TelemetryMessage> {
        let (tx, rx) = channel();

        thread::spawn(move || {
            sleep(Duration::from_millis(2000));

            loop {
                let mylora = Pin::new(LORA_GPIO_PIN_NUMBER); // number depends on chip, etc.
                mylora
                    .with_exported(|| {
                        println!("set the pin direction");

                        mylora.set_direction(Direction::Out).unwrap();
                        println!("set the pin low");

                        mylora.set_value(0).unwrap();

                        sleep(Duration::from_millis(1000));
                        println!("set the pin high");

                        mylora.set_value(1).unwrap();
                        sleep(Duration::from_millis(1000));
                        Ok(())
                    })
                    .unwrap();
                let pair = Arc::new((Mutex::new(None), Condvar::new()));
                let pair2 = pair.clone();

                thread::spawn(move || {
                    let (lock, cvar) = &*pair2;
                    let mut device = lock.lock().unwrap();
                    *device = Some(Rn2903::new_at(LORA_DEVICE_PATH));
                    // We notify the condvar that the value has changed.
                    cvar.notify_one();
                });

                let (lock, cvar) = &*pair;
                let mut device_ready = lock.lock().unwrap();

                let result = cvar
                    .wait_timeout(device_ready, Duration::from_millis(800))
                    .unwrap();

                device_ready = result.0;

                //               if *device_ready.is_none() {
                //                 sleep(Duration::from_millis(15000));
                //           }else {

                //         }
                match device_ready.as_mut() {
                    Some(Err(error)) => match error {
                        rn2903::Error::WrongDevice(_) => {
                            error!(
                                "LORA device not compatible, will consume message but do nothing"
                            );
                            loop {
                                if rx.recv().is_err() {
                                    error!("Chanel on LORA close unexpectidly");
                                    break;
                                }
                            }
                        }
                        rn2903::Error::ConnectionFailed(e) => {
                            error!(
                                    "LORA device Connection failed for this reason {:?} : {} || I will empty reciver queue and try again in 15 seconds",
                                    e.kind, e.description
                                );
                            loop {
                                match rx.recv_timeout(Duration::from_millis(1)) {
                                    Ok(_) => continue,
                                    Err(mpsc::RecvTimeoutError::Timeout) => break,
                                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                                        error!("Chanel on LORA close unexpectidly");
                                        break;
                                    }
                                }
                            }
                            sleep(Duration::from_millis(15000));
                            continue;
                        }
                        _ => {
                            warn!("Unexpected error while connecting to the LORA device, will retry after fleushing queue in 15 seconds");
                            loop {
                                match rx.recv_timeout(Duration::from_millis(1)) {
                                    Ok(_) => continue,
                                    Err(mpsc::RecvTimeoutError::Timeout) => break,
                                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                                        error!("Chanel on LORA close unexpectidly");
                                        break;
                                    }
                                }
                            }
                            sleep(Duration::from_millis(15000));
                            continue;
                        }
                    },
                    None => {
                        info!(
                            "LORA module not ready, waiting for 15 seconds more and fulsh all data to send, because we cannot manage it"
                        );
                        loop {
                            match rx.recv_timeout(Duration::from_millis(1)) {
                                Ok(_) => continue,
                                Err(mpsc::RecvTimeoutError::Timeout) => break,
                                Err(mpsc::RecvTimeoutError::Disconnected) => {
                                    error!("Chanel on LORA close unexpectidly");
                                    break;
                                }
                            }
                        }
                        sleep(Duration::from_millis(15000));
                        continue;
                    }
                    Some(Ok(ref mut txvr_ready)) => {
                        let mut i = 0;
                        while txvr_ready.mac_pause().is_err() && i < 60 {
                            info!("cannot MAC pause LORA device, will try again in one second");
                            i = i + 1;
                            sleep(Duration::from_millis(1000));
                        }
                        if i >= 60 {
                            error!("Unable to MAC pause LORA device, rebuild the entire stack");
                            continue;
                        }
                        info!("LORA device initialisation completed");

                        // Here is the loop for message management
                        loop {
                            let messagewrap = rx.recv();
                            match messagewrap {
                                Ok(message) => match message {
                                    TelemetryMessage::MachineStateSnapshot(snapshot) => {
                                        let mes = format!(
                                            "{},{},{},{},{},{},{},{},{}",
                                            snapshot.device_id,
                                            snapshot.cycle,
                                            snapshot.peak_command,
                                            snapshot.plateau_command,
                                            snapshot.peep_command,
                                            snapshot.cpm_command,
                                            snapshot.previous_peak_pressure,
                                            snapshot.previous_plateau_pressure,
                                            snapshot.previous_peep_pressure
                                        );
                                        let transmission = txvr_ready.radio_tx(mes.to_string());
                                        while transmission.is_err() {
                                            unimplemented!();
                                        }
                                        sleep(Duration::from_millis(20));
                                    }
                                    _ => (),
                                },
                                Err(e) => match e {
                                    mpsc::RecvError => {
                                        error!("Chanel on LORA close unexpectidly");
                                        break;
                                    }
                                },
                            }
                        }
                    }
                }
            }
        });
        // return
        return tx;
    }
}
