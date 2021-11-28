// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use makair_simulator::MakAirSimulator;
use makair_telemetry::control::ControlMessage;
use makair_telemetry::TelemetryChannelType;
use std::sync::mpsc::{Receiver, Sender};

pub fn init(
    tx: Sender<TelemetryChannelType>,
    control_rx: Receiver<ControlMessage>,
) -> MakAirSimulator {
    let mut simulator = MakAirSimulator::new(tx);
    simulator.initialize(false);

    let control_message_sender = simulator.control_message_sender();

    std::thread::spawn(move || loop {
        while let Ok(message) = control_rx.recv() {
            control_message_sender.send(message).unwrap();
        }
    });

    simulator
}
