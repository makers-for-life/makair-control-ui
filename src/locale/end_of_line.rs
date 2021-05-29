// MakAir Control UI
//
// Copyright: 2021, Makers For Life
// License: Public Domain License

use crate::chip::{ChipEndOfLine, ChipEndOfLineEnd, ChipEndOfLineFailure, ChipEndOfLineStep};

use crate::APP_I18N;

pub fn end_of_line_to_locales(eol: &ChipEndOfLine) -> (String, String) {
    // Acquire target locale attributes
    let locale_keys = match eol {
        ChipEndOfLine::Ongoing(end_of_line_step) => (
            "step",
            match end_of_line_step {
                ChipEndOfLineStep::Start => "start",
                ChipEndOfLineStep::CheckFan => "check-fan",
                ChipEndOfLineStep::TestBatteryDead => "test-battery-dead",
                ChipEndOfLineStep::DisconnectMains => "disconnect-mains",
                ChipEndOfLineStep::ConnectMains => "connect-mains",
                ChipEndOfLineStep::CheckBuzzer => "check-buzzer",
                ChipEndOfLineStep::CheckAllButtons => "check-all-buttons",
                ChipEndOfLineStep::CheckUiScreen => "check-ui-screen",
                ChipEndOfLineStep::PlugAirTestSystem => "plug-air-test-system",
                ChipEndOfLineStep::ReachMaximumPressure => "reach-maximum-pressure",
                ChipEndOfLineStep::MaximumPressureReached => "maximum-pressure-reached",
                ChipEndOfLineStep::StartLeakMeasure => "start-leak-measure",
                ChipEndOfLineStep::ReachNullPressure => "reach-null-pressure",
                ChipEndOfLineStep::ConfirmBeforeOxygenTest => "confirm-before-oxygen-test",
                ChipEndOfLineStep::StartOxygenTest => "start-oxygen-test",
                ChipEndOfLineStep::WaitBeforeBlowerLongRun => "wait-before-blower-long-run",
                ChipEndOfLineStep::StartBlowerLongRun => "start-blower-long-run",
            },
        ),
        ChipEndOfLine::Failed(end_of_line_failure, _) => (
            "failure",
            match end_of_line_failure {
                ChipEndOfLineFailure::ExpanderNotConnected => "expander-not-connected",
                ChipEndOfLineFailure::BatteryDeeplyDischarged => "battery-deeply-discharged",
                ChipEndOfLineFailure::MaximumPressureNotReached => "maximum-pressure-not-reached",
                ChipEndOfLineFailure::LeakTooHigh => "leak-too-high",
                ChipEndOfLineFailure::MinimumPressureNotReached => "minimum-pressure-not-reached",
                ChipEndOfLineFailure::OxygenPressureNotReached => "oxygen-pressure-not-reached",
                ChipEndOfLineFailure::PressureNotStable => "pressure-not-stable",
                ChipEndOfLineFailure::FlowNotStable => "flow-not-stable",
            },
        ),
        ChipEndOfLine::Succeeded(end_of_line_end) => (
            "end",
            match end_of_line_end {
                ChipEndOfLineEnd::Confirm => "confirm",
                ChipEndOfLineEnd::DisplayPressure => "display-pressure",
                ChipEndOfLineEnd::DisplayFlow => "display-flow",
            },
        ),
    };

    // Acquire title
    let title = APP_I18N.t(&format!(
        "end-of-line-content-title-{}-{}",
        locale_keys.0, locale_keys.1
    ));

    // Generate full message (with optional details, if any)
    let mut message = APP_I18N.t(&format!(
        "end-of-line-content-message-{}-{}",
        locale_keys.0, locale_keys.1
    ));

    // Append failure details? (if failed state)
    if let ChipEndOfLine::Failed(_, end_of_line_details) = eol {
        if !end_of_line_details.is_empty() {
            message.push('\n');
            message.push_str("#> ");
            message.push_str(end_of_line_details);
        }
    }

    (title, message)
}
