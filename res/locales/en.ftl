telemetry-label-peak = P(peak)
telemetry-label-plateau = P(plateau)
telemetry-label-expiratory = P(expiratory)
telemetry-label-cycles = Cycles/minute
telemetry-label-ratio = Inspiratory duration
telemetry-label-ratio-details = insp-exp. ratio of
telemetry-label-tidal = Tidal volume
telemetry-label-minute-volume = Minute volume

telemetry-unit-cmh2o = cmH2O
telemetry-unit-mmh2o = mmH2O
telemetry-unit-lpm = L/min
telemetry-unit-mlpm = mL/min
telemetry-unit-per-minute = /minute
telemetry-unit-milliliters = mL
telemetry-unit-milliseconds = ms
telemetry-unit-centimeters = cm

alarms-title = ALARMS
alarms-empty = No alarm is active.

alarms-message-plateau-pressure-not-reached = Plateau pressure is not reached
alarms-message-patient-unplugged = Patient is unplugged
alarms-message-peep-pressure-not-reached = PEEP pressure is not reached
alarms-message-battery-low = Battery is low
alarms-message-battery-very-low = Battery is very low
alarms-message-power-cable-unplugged = Power cable is unplugged
alarms-message-pressure-too-high = Pressure is too high
alarms-message-inspiratory-minute-volume-low = Insp. minute volume is too low
alarms-message-inspiratory-minute-volume-high = Insp. minute volume is too high
alarms-message-expiratory-minute-volume-low = Exp. minute volume is too low
alarms-message-expiratory-minute-volume-high = Exp. minute volume is too high
alarms-message-respiratory-rate-low = Respiratory rate is too low
alarms-message-respiratory-rate-high = Respiratory rate is too high
alarms-message-leak-high = Leak volume is too high
alarms-message-tidal-volume-low = Tidal volume is too low
alarms-message-tidal-volume-high = Tidal volume is too high
alarms-message-peak-pressure-high = Peak pressure is too high
alarms-message-expiratory-flow-too-low = Expiratory flow is too low
alarms-message-unknown = Unknown alarm (check code)

status-unit-stopped = Unit stopped
status-unit-active = Unit active
status-power-battery = Battery
status-power-ac = AC power

mode-class-pc = Pressure
mode-class-vc = Volume
mode-type-cmv = CMV
mode-type-ac = AC
mode-type-vsai = VSAI
mode-group-general = General
mode-group-alarms = Alarms

advanced-group-statistics = Statistics
advanced-group-settings = Settings

modal-close = Close
modal-apply = Apply
modal-cancel = Ignore

modal-preset-title = Hello. Please configure your patient details.
modal-preset-subtitle = Preset settings will be applied, which can be adjusted afterwards.
modal-preset-gender = Gender
modal-preset-gender-male = Male
modal-preset-gender-female = Female
modal-preset-age = Age group
modal-preset-size = Body size
modal-preset-age-child = Child
modal-preset-age-teenager = Teenager
modal-preset-age-adult = Adult

modal-run-status = Ventilator unit status
modal-run-status-started = Running — Tap to stop
modal-run-status-stopped = Stopped — Tap to start

modal-snooze-alarms = Snooze alarms
modal-snooze-alarms-active = Beeps enabled (default setting)
modal-snooze-alarms-inactive = Beeps disabled (for some time)

modal-mode-pressure-inspiratory = Inspiratory pressure
modal-mode-pressure-expiratory = Expiratory pressure
modal-mode-time-inspiratory = Inspiratory time
modal-mode-time-inspiratory-minimum = Inspiratory time (minimum)
modal-mode-time-inspiratory-maximum = Inspiratory time (maximum)
modal-mode-flow-inspiratory = Inspiratory flow
modal-mode-cycles-per-minute = Cycles per minute
modal-mode-tidal-volume = Tidal volume
modal-mode-plateau-duration = Plateau duration
modal-mode-trigger-offset = Trigger offset
modal-mode-trigger-expiratory = Expiratory trigger
modal-mode-alarm-low-inspiratory-minute-volume = Insp. minute volume (min)
modal-mode-alarm-high-inspiratory-minute-volume = Insp. minute volume (max)
modal-mode-alarm-low-expiratory-minute-volume = Exp. minute volume (min)
modal-mode-alarm-high-expiratory-minute-volume = Exp. minute volume (max)
modal-mode-alarm-low-respiratory-rate = Respiratory rate (min)
modal-mode-alarm-high-respiratory-rate = Respiratory rate (max)
modal-mode-alarm-low-tidal-volume = Tidal volume (min)
modal-mode-alarm-high-tidal-volume = Tidal volume (max)
modal-mode-alarm-leak = Leak volume (max)
modal-mode-alarm-peak-pressure = Peak pressure (max)

modal-advanced-locale = Language
modal-advanced-date = Date
modal-advanced-time = Time
modal-advanced-timezone = Timezone

initializing-connecting = Starting up...
initializing-connected = Initializing...

error-title-no-device = Oops. Could not reach core.
error-title-timed-out = Oops. Failed initializing core.
error-title-bad-protocol = General telemetry protocol error.
error-title-watchdog = Uh. The ventilation controller has crashed.
error-title-sensor-failure = A sensor is offline. Safety is compromised.
error-title-other = Oops. An unknown error occurred.

error-message-no-device = Cannot reach the telemetry master. Is it properly configured?
error-message-timed-out = Some components may have failed to start. Can you try power cycling?
error-message-bad-protocol = The firmware is using an unsupported telemetry protocol. Please update software.
error-message-watchdog = A watchdog timer has been raised, and thus the internal microcontroller has restarted.
error-message-sensor-failure = Sensor ID:
error-message-other = Reason:

end-of-line-title-primary = Service Mode
end-of-line-title-secondary = End-of-Line Test

end-of-line-content-title-step-start = Starting EOL test...
end-of-line-content-title-step-check-fan = Please check fans.
end-of-line-content-title-step-test-battery-dead = Testing battery condition...
end-of-line-content-title-step-disconnect-mains = Please unplug the AC power cord.
end-of-line-content-title-step-connect-mains = Please plug the AC power cord.
end-of-line-content-title-step-check-buzzer = Testing buzzer...
end-of-line-content-title-step-check-all-buttons = Please press all buttons.
end-of-line-content-title-step-check-ui-screen = Please tap the touch screen.
end-of-line-content-title-step-plug-air-test-system = Please plug the lung test system.
end-of-line-content-title-step-reach-maximum-pressure = Testing maximum pressure...
end-of-line-content-title-step-maximum-pressure-reached = Maximum pressure reached. Closing valves...
end-of-line-content-title-step-start-leak-measure = Testing for leaks...
end-of-line-content-title-step-reach-null-pressure = No leak detected. Opening valves...
end-of-line-content-title-step-confirm-before-oxygen-test = Please plug the oxygen test system.
end-of-line-content-title-step-start-oxygen-test = Testing for oxygen...
end-of-line-content-title-step-wait-before-blower-long-run = Please unplug the oxygen test system.
end-of-line-content-title-step-start-blower-long-run = Testing for blower stability...

end-of-line-content-title-failure-expander-not-connected = The expander is not connected.
end-of-line-content-title-failure-battery-deeply-discharged = Battery is deeply discharged.
end-of-line-content-title-failure-maximum-pressure-not-reached = Maximum pressure could not be reached.
end-of-line-content-title-failure-leak-too-high = Air leak is too high.
end-of-line-content-title-failure-minimum-pressure-not-reached = Minimum pressure could not be reached.
end-of-line-content-title-failure-oxygen-pressure-not-reached = Oxygen pressure could not be reached.
end-of-line-content-title-failure-pressure-not-stable = Blower pressure is not stable.
end-of-line-content-title-failure-flow-not-stable = Blower flow is not stable.

end-of-line-content-title-end-confirm = All tests succeeded!
end-of-line-content-title-end-display-pressure = Displaying pressure details.
end-of-line-content-title-end-display-flow = Displaying flow details.

end-of-line-content-message-step-start = The EOL test will start in a few moments.
end-of-line-content-message-step-check-fan = The two cooling fans should be working. Press start to continue.
end-of-line-content-message-step-test-battery-dead = The battery health will be checked. Please wait.
end-of-line-content-message-step-disconnect-mains = AC power prevents testing batteries. Please unplug it.
end-of-line-content-message-step-connect-mains = The power supply health will be checked once plugged to AC.
end-of-line-content-message-step-check-buzzer = The buzzer should be working. Press pause to confirm.
end-of-line-content-message-step-check-all-buttons = Each button should be pressed one by one.
end-of-line-content-message-step-check-ui-screen = Please change ventilation mode on the touch screen.
end-of-line-content-message-step-plug-air-test-system = The test lung should be connected. Press start to continue.
end-of-line-content-message-step-reach-maximum-pressure = The pressure in the lung is being increased. Please wait.
end-of-line-content-message-step-maximum-pressure-reached = The maximum pressure has been reached. Please wait.
end-of-line-content-message-step-start-leak-measure = The air system is being tested for any leak. Please wait.
end-of-line-content-message-step-reach-null-pressure = No leak was detected. Please wait.
end-of-line-content-message-step-confirm-before-oxygen-test = The oxygen port should be connected. Press start to continue.
end-of-line-content-message-step-start-oxygen-test = The oxygen mixer is being tested. Please wait.
end-of-line-content-message-step-wait-before-blower-long-run = The blower will be checked for stability. Press start to continue.
end-of-line-content-message-step-start-blower-long-run = The blower is being tested. This will take some time.

end-of-line-content-message-failure-expander-not-connected = Please connect power supply to the motherboard.
end-of-line-content-message-failure-battery-deeply-discharged = The battery should be replaced, as they are in an under-voltage state.
end-of-line-content-message-failure-maximum-pressure-not-reached = The target pressure could not be reached in the air circuit.
end-of-line-content-message-failure-leak-too-high = The air system leaks too much air. Please check the air circuit.
end-of-line-content-message-failure-minimum-pressure-not-reached = Valves might not have opened correctly. Please check them.
end-of-line-content-message-failure-oxygen-pressure-not-reached = The oxygen pipes might be blocked. Please check them.
end-of-line-content-message-failure-pressure-not-stable = The blower was unable to sustain a stable air pressure.
end-of-line-content-message-failure-flow-not-stable = The blower was unable to sustain a stable air flow.

end-of-line-content-message-end-confirm = This ventilator passed all tests. Press start to see details.
end-of-line-content-message-end-display-pressure = Pressure test details are visible on the debug screen.
end-of-line-content-message-end-display-flow = Flow test details are visible on the debug screen.

stop-title = Ventilator unit inactive
stop-description = Tap any value to configure it, then press start.
