// MakAir Control UI
//
// Copyright: 2021, Makers For Life
// License: Public Domain License

use crate::chip::{ChipEndOfLineFailure, ChipEndOfLineStep};

pub fn index_from_end_of_line_step(end_of_line_step: &ChipEndOfLineStep) -> Option<u8> {
    match end_of_line_step {
        ChipEndOfLineStep::Start => None,
        ChipEndOfLineStep::CheckFan => Some(1),
        ChipEndOfLineStep::TestBatteryDead | ChipEndOfLineStep::DisconnectMains => Some(2),
        ChipEndOfLineStep::ConnectMains => Some(3),
        ChipEndOfLineStep::CheckBuzzer => Some(4),
        ChipEndOfLineStep::CheckAllButtons => Some(5),
        ChipEndOfLineStep::CheckUiScreen => Some(6),
        ChipEndOfLineStep::PlugAirTestSystem
        | ChipEndOfLineStep::ReachMaximumPressure
        | ChipEndOfLineStep::MaximumPressureReached => Some(7),
        ChipEndOfLineStep::StartLeakMeasure | ChipEndOfLineStep::ReachNullPressure => Some(8),
        ChipEndOfLineStep::ConfirmBeforeOxygenTest | ChipEndOfLineStep::StartOxygenTest => Some(9),
        ChipEndOfLineStep::WaitBeforeBlowerLongRun | ChipEndOfLineStep::StartBlowerLongRun => {
            Some(10)
        }
    }
}

pub fn index_from_end_of_line_failure(end_of_line_failure: &ChipEndOfLineFailure) -> Option<u8> {
    // Reflect index for failure from its origin step
    match end_of_line_failure {
        ChipEndOfLineFailure::ExpanderNotConnected => None,
        ChipEndOfLineFailure::BatteryDeeplyDischarged => {
            index_from_end_of_line_step(&ChipEndOfLineStep::TestBatteryDead)
        }
        ChipEndOfLineFailure::MaximumPressureNotReached => {
            index_from_end_of_line_step(&ChipEndOfLineStep::ReachMaximumPressure)
        }
        ChipEndOfLineFailure::LeakTooHigh => {
            index_from_end_of_line_step(&ChipEndOfLineStep::StartLeakMeasure)
        }
        ChipEndOfLineFailure::MinimumPressureNotReached => {
            index_from_end_of_line_step(&ChipEndOfLineStep::ReachNullPressure)
        }
        ChipEndOfLineFailure::OxygenPressureNotReached => {
            index_from_end_of_line_step(&ChipEndOfLineStep::StartOxygenTest)
        }
        ChipEndOfLineFailure::PressureNotStable | ChipEndOfLineFailure::FlowNotStable => {
            index_from_end_of_line_step(&ChipEndOfLineStep::StartBlowerLongRun)
        }
    }
}
