// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::sync::mpsc::{Receiver, TryRecvError};

use telemetry::serial::core::{Error, ErrorKind};
use telemetry::structures::{HighLevelError, TelemetryMessage, TelemetryMessageOrError};
use telemetry::{self, TelemetryChannelType};

pub struct SerialPollerBuilder;
pub struct SerialPoller;

#[derive(Debug)]
pub enum PollEvent {
    Ready(TelemetryMessage),
    Corrupted(HighLevelError),
    Pending,
}

#[allow(clippy::new_ret_no_self)]
impl SerialPollerBuilder {
    pub fn new() -> SerialPoller {
        SerialPoller {}
    }
}

impl SerialPoller {
    pub fn poll(&mut self, rx: &Receiver<TelemetryChannelType>) -> Result<PollEvent, Error> {
        match rx.try_recv() {
            // 1. Telemetry message can be handled and is valid
            Ok(Ok(TelemetryMessageOrError::Message(message))) => Ok(PollEvent::Ready(message)),
            // 2. Telemetry message was received, but it could not be handled
            Ok(Ok(TelemetryMessageOrError::Error(message_error))) => {
                Ok(PollEvent::Corrupted(message_error))
            }
            // 3. A serial error occurred
            Ok(Err(serial_error)) => Err(serial_error),
            // 4. Empty data was received (this is expected)
            Err(TryRecvError::Empty) => Ok(PollEvent::Pending),
            // 5. The serial device is disconnected
            Err(TryRecvError::Disconnected) => {
                Err(Error::new(ErrorKind::NoDevice, "device is disconnected"))
            }
        }
    }
}
