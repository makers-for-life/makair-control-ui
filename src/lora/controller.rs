// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use rn2903::Rn2903;
use sysfs_gpio::{Direction, Pin};
use telemetry::structures::TelemetryMessage;

use std::sync::mpsc::channel;
use std::sync::mpsc::{self, Sender};
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, sleep};
use std::time::Duration;

use crate::config::environment::*;
use crate::APP_ARGS;

pub struct LoraController;

impl LoraController {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Sender<TelemetryMessage> {
        let (tx, rx) = channel();

        thread::spawn(move || {
            sleep(Duration::from_millis(2000));

            loop {
                let lora_pin = Pin::new(LORA_GPIO_PIN_NUMBER);

                let lora_setup = lora_pin.with_exported(|| {
                    println!("set the pin direction");

                    lora_pin.set_direction(Direction::Out).unwrap();
                    println!("set the pin low");

                    lora_pin.set_value(0).unwrap();

                    sleep(Duration::from_millis(1000));
                    println!("set the pin high");

                    lora_pin.set_value(1).unwrap();
                    sleep(Duration::from_millis(1000));

                    Ok(())
                });

                match lora_setup {
                    Ok(_) => {}
                    Err(err) => {
                        error!("error setting up lora because: {:?}. retrying in 1s.", err);

                        std::thread::sleep(Duration::from_secs(1));

                        continue;
                    }
                };

                let pair = Arc::new((Mutex::new(None), Condvar::new()));
                let pair2 = pair.clone();

                thread::spawn(move || {
                    let (lock, cvar) = &*pair2;
                    let mut device = lock.lock().unwrap();
                    *device = Some(Rn2903::new_at(&APP_ARGS.lora_device));

                    // We notify the condvar that the value has changed.
                    cvar.notify_one();
                });

                let (lock, cvar) = &*pair;
                let mut device_ready = lock.lock().unwrap();

                let result = cvar
                    .wait_timeout(device_ready, Duration::from_millis(800))
                    .unwrap();

                device_ready = result.0;

                match device_ready.as_mut() {
                    Some(Err(error)) => match error {
                        rn2903::Error::WrongDevice(_) => {
                            error!(
                                "lora device not compatible, will consume message but do nothing"
                            );

                            loop {
                                if rx.recv().is_err() {
                                    error!("channel on lora closed unexpectedly");

                                    break;
                                }
                            }
                        }

                        rn2903::Error::ConnectionFailed(err) => {
                            error!(
                                    "lora device connection failed for this reason {:?}: {}. will empty receiver queue and try again in 15 seconds.",
                                    err.kind, err.description
                                );

                            loop {
                                match rx.recv_timeout(Duration::from_millis(1)) {
                                    Ok(_) => continue,
                                    Err(mpsc::RecvTimeoutError::Timeout) => break,
                                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                                        error!("channel on lora closed unexpectedly");

                                        break;
                                    }
                                }
                            }

                            sleep(Duration::from_millis(15000));

                            continue;
                        }

                        _ => {
                            warn!("unexpected error while connecting to the lora device, will retry after flushing queue in 15 seconds");

                            loop {
                                match rx.recv_timeout(Duration::from_millis(1)) {
                                    Ok(_) => continue,
                                    Err(mpsc::RecvTimeoutError::Timeout) => break,
                                    Err(mpsc::RecvTimeoutError::Disconnected) => {
                                        error!("channel on lora closed unexpectedly");

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
                            "lora module not ready, waiting for 15 seconds more and flushing all data to send, because we cannot manage it"
                        );

                        loop {
                            match rx.recv_timeout(Duration::from_millis(1)) {
                                Ok(_) => continue,
                                Err(mpsc::RecvTimeoutError::Timeout) => break,
                                Err(mpsc::RecvTimeoutError::Disconnected) => {
                                    error!("channel on lora closed unexpectedly");

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
                            info!("cannot mac pause lora device, will try again in one second");
                            i += 1;
                            sleep(Duration::from_millis(1000));
                        }

                        if i >= 60 {
                            error!("unable to mac pause lora device, rebuilding the entire stack");
                            continue;
                        }

                        info!("lora device initialization completed");

                        // Here is the loop for message management
                        loop {
                            let messagewrap = rx.recv();

                            match messagewrap {
                                Ok(message) => {
                                    if let TelemetryMessage::MachineStateSnapshot(snapshot) =
                                        message
                                    {
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
                                }

                                Err(err) => match err {
                                    mpsc::RecvError => {
                                        error!("channel on lora closed unexpectedly");

                                        break;
                                    }
                                },
                            }
                        }
                    }
                }
            }
        });

        tx
    }
}
