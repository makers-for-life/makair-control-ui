// MakAir Control UI
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use std::sync::mpsc::{Receiver, TryRecvError};

use makair_telemetry::error::Error;
use makair_telemetry::structures::{HighLevelError, TelemetryMessage};
use makair_telemetry::{self, TelemetryChannelType};

pub struct SerialPollerBuilder;
pub struct SerialPoller;

#[derive(Debug)]
pub enum PollEvent {
    Ready(TelemetryMessage),
    Corrupted(HighLevelError),
    Pending,
}

#[derive(Debug)]
pub enum PollError {
    #[cfg(feature = "serial")]
    TelemetryError(makair_telemetry::error::Error),
    #[cfg(not(feature = "serial"))]
    DisconnectedChannel,
}

#[allow(clippy::new_ret_no_self)]
impl SerialPollerBuilder {
    pub fn new() -> SerialPoller {
        SerialPoller {}
    }
}

impl SerialPoller {
    pub fn poll(&mut self, rx: &Receiver<TelemetryChannelType>) -> Result<PollEvent, PollError> {
        match rx.try_recv() {
            // 1. Telemetry message can be handled and is valid
            Ok(Ok(message)) => Ok(PollEvent::Ready(message)),
            // 2. Telemetry message was received, but it could not be handled
            Ok(Err(Error::TelemetryError(message_error))) => {
                Ok(PollEvent::Corrupted(message_error))
            }
            // 3. A serial error occurred
            #[cfg(feature = "serial")]
            Ok(Err(error)) => Err(PollError::TelemetryError(error)),
            // 4. Empty data was received (this is expected)
            Err(TryRecvError::Empty) => Ok(PollEvent::Pending),
            // 6. The serial device is disconnected
            #[cfg(feature = "serial")]
            Err(TryRecvError::Disconnected) => Err(PollError::TelemetryError(
                makair_telemetry::serial::core::Error::new(
                    makair_telemetry::serial::core::ErrorKind::NoDevice,
                    "device is disconnected",
                )
                .into(),
            )),
            // 5. channel is disconnected
            #[cfg(not(feature = "serial"))]
            Err(TryRecvError::Disconnected) => Err(PollError::DisconnectedChannel),
        }
    }
}
