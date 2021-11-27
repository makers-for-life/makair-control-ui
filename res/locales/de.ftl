telemetry-label-peak = P(spitzendruck)
telemetry-label-plateau = P(plateaudruck)
telemetry-label-expiratory = P(PEEP)
telemetry-label-cycles = Zyklen/Minuten
telemetry-label-ratio = Inspirationsdauer
telemetry-label-ratio-details = ein.-aus. verhältnis
telemetry-label-tidal = Atemzugvolumen
telemetry-label-minute-volume = Minutenlautstärke

telemetry-unit-cmh2o = cmH2O
telemetry-unit-mmh2o = mmH2O
telemetry-unit-lpm = L/min
telemetry-unit-mlpm = mL/min
telemetry-unit-per-minute = /minute
telemetry-unit-milliliters = mL
telemetry-unit-milliseconds = ms
telemetry-unit-centimeters = cm

alarms-title = ALARME
alarms-empty = Kein Alarm.

alarms-message-plateau-pressure-not-reached = Der Plateaudruck wird nicht erreicht
alarms-message-patient-unplugged = Der Patient ist ausgesteckt
alarms-message-peep-pressure-not-reached = Der exspiratorische Druck wird nicht erreicht
alarms-message-battery-low = Batterie ist fast leer
alarms-message-battery-very-low = Die Batterie ist sehr schwach
alarms-message-power-cable-unplugged = Das Netzkabel ist nicht angeschlossen
alarms-message-pressure-too-high = Der Druck ist zu hoch
alarms-message-inspiratory-minute-volume-low = Das inspiratorische Minutenvolumen ist zu niedrig
alarms-message-inspiratory-minute-volume-high = Das inspiratorische Minutenvolumen ist zu hoch
alarms-message-expiratory-minute-volume-low = Das exspiratorische Minutenvolumen ist zu gering
alarms-message-expiratory-minute-volume-high = Das exspiratorische Minutenvolumen ist zu hoch
alarms-message-respiratory-rate-low = Atemfrequenz ist zu niedrig
alarms-message-respiratory-rate-high = Atemfrequenz ist zu hoch
alarms-message-leak-high = Leckvolumen ist zu hoch
alarms-message-tidal-volume-low = Das Atemvolumen ist zu gering
alarms-message-tidal-volume-high = Das Atemvolumen ist zu hoch
alarms-message-peak-pressure-high = Der Spitzendruck ist zu hoch
alarms-message-expiratory-flow-too-low = Der exspiratorische Fluss ist zu gering
alarms-message-unknown = Unbekannter Alarm (Code prüfen)

status-unit-stopped = Gerät gestoppt
status-unit-active = Gerät aktiv
status-power-battery = Batterie
status-power-ac = Stromversorgung

mode-class-pc = Druck
mode-class-vc = Volumen
mode-type-cmv = CMV
mode-type-ac = AC
mode-type-vsai = VSAI
mode-group-general = Allgemeines
mode-group-alarms = Alarm

advanced-group-statistics = Statistiken
advanced-group-settings = Optionen
advanced-group-simulator = Simulator

modal-close = Schließen
modal-apply = Anwenden
modal-cancel = Stornieren

modal-preset-title = Hallo. Bitte konfigurieren Sie Ihre Patientendaten.
modal-preset-subtitle = Es werden voreingestellte Einstellungen angewendet, die anschließend angepasst werden können.
modal-preset-gender = Sex
modal-preset-gender-male = Männlich
modal-preset-gender-female = Weiblich
modal-preset-age = Altersgruppe
modal-preset-size = Körpergröße
modal-preset-age-child = Kind
modal-preset-age-teenager = Teenager
modal-preset-age-adult = Erwachsene

modal-run-status = Status der Beatmungseinheit
modal-run-status-started = Laufen - aufhören?
modal-run-status-stopped = Gestoppt - Starten?

modal-snooze-alarms = Alarm
modal-snooze-alarms-active = Signaltöne aktiviert
modal-snooze-alarms-inactive = Signaltöne deaktiviert

modal-mode-pressure-inspiratory = Inspirationsdruck
modal-mode-pressure-expiratory = Ausatmungsdruck
modal-mode-time-inspiratory = Inspirationszeit
modal-mode-time-inspiratory-minimum = Inspirationszeit (minimum)
modal-mode-time-inspiratory-maximum = Inspirationszeit (maximal)
modal-mode-flow-inspiratory = Inspirationsfluss
modal-mode-cycles-per-minute = Zyklen pro Minute
modal-mode-tidal-volume = Atemzugvolumen
modal-mode-plateau-duration = Plateaudauer
modal-mode-trigger-offset = Offset auslösen
modal-mode-trigger-expiratory = Exspiratorischer Auslöser
modal-mode-alarm-low-inspiratory-minute-volume = Inspirationsvolumen (niedrig)
modal-mode-alarm-high-inspiratory-minute-volume = Inspirationsvolumen (hoch)
modal-mode-alarm-low-expiratory-minute-volume = Exspirationsvolumen (niedrig)
modal-mode-alarm-high-expiratory-minute-volume = Exspirationsvolumen (hoch)
modal-mode-alarm-low-respiratory-rate = Exspirationsrate (niedrig)
modal-mode-alarm-high-respiratory-rate = Exspirationsrate (hoch)
modal-mode-alarm-low-tidal-volume = Atemzugvolumen (gering)
modal-mode-alarm-high-tidal-volume = Atemzugvolumen (hoch)
modal-mode-alarm-leak = Leckvolumen (hoch)
modal-mode-alarm-peak-pressure = Spitzendruck (hoch)

modal-advanced-locale = Sprache
modal-advanced-date = Datum
modal-advanced-time = Zeit
modal-advanced-timezone = Zeitzone

modal-advanced-simulator-airway-resistance = Resistance
modal-advanced-simulator-airway-compliance = Compliance
modal-advanced-simulator-spontaneous-breath-rate = Breath rate
modal-advanced-simulator-spontaneous-breath-effort = Breath effort
modal-advanced-simulator-spontaneous-breath-duration = Breath duration
modal-advanced-simulator-acceleration-factor = Acceleration percent

initializing-connecting = Inbetriebnahme...
initializing-connected = Initialisierung...

error-title-no-device = Hoppla. Kern konnte nicht erreicht werden.
error-title-timed-out = Hoppla. Fehler beim Initialisieren des Kerns.
error-title-bad-protocol = Allgemeiner Telemetrieprotokollfehler.
error-title-watchdog = Äh. Der Lüftungsregler ist abgestürzt.
error-title-sensor-failure = Ein Sensor ist offline. Die Sicherheit ist gefährdet.
error-title-other = Hoppla. Ein unbekannter Fehler ist aufgetreten.

error-message-no-device = Der Telemetriemaster kann nicht erreicht werden. Ist es richtig konfiguriert?
error-message-timed-out = Einige Komponenten konnten möglicherweise nicht gestartet werden. Können Sie Power Cycling ausprobieren?
error-message-bad-protocol = Die Firmware verwendet ein nicht unterstütztes Telemetrieprotokoll. Bitte aktualisieren Sie die Software.
error-message-watchdog = Ein Watchdog-Timer wurde ausgelöst und somit der interne Mikrocontroller neu gestartet.
error-message-sensor-failure = Sensor ID:
error-message-other = Grund:

stop-title = Das Beatmungsgerät ist nicht aktiv
stop-description = Bitte aktivieren Sie es wieder, um zu lüften

end-of-line-title-primary = Service-Modus
end-of-line-title-secondary = End-of-Line-Test

end-of-line-content-title-step-start = Start des EOL-Tests...
end-of-line-content-title-step-check-fan = Bitte Lüfter prüfen.
end-of-line-content-title-step-test-battery-dead = Batteriezustand prüfen...
end-of-line-content-title-step-disconnect-mains = Bitte ziehen Sie das AC-Netzkabel ab.
end-of-line-content-title-step-connect-mains = Bitte stecken Sie das AC-Netzkabel ein.
end-of-line-content-title-step-check-buzzer = Test des Summers...
end-of-line-content-title-step-check-all-buttons = Bitte drücken Sie alle Tasten.
end-of-line-content-title-step-check-ui-screen = Bitte tippen Sie auf den Touchscreen.
end-of-line-content-title-step-plug-air-test-system = Bitte schließen Sie das Lungentestsystem an.
end-of-line-content-title-step-reach-maximum-pressure = Test des maximalen Drucks...
end-of-line-content-title-step-maximum-pressure-reached = Maximaler Druck erreicht. Ventile schließen...
end-of-line-content-title-step-start-leak-measure = Testen auf Lecks...
end-of-line-content-title-step-reach-null-pressure = Kein Leck entdeckt. Ventile öffnen...
end-of-line-content-title-step-confirm-before-oxygen-test = Bitte schließen Sie das Sauerstoff-Testsystem.
end-of-line-content-title-step-start-oxygen-test = Testen auf Sauerstoff...
end-of-line-content-title-step-wait-before-blower-long-run = Bitte ziehen Sie den Stecker des Sauerstoff-Testsystems heraus.
end-of-line-content-title-step-start-blower-long-run = Testen auf Gebläsestabilität...

end-of-line-content-title-failure-expander-not-connected = Der Expander ist nicht angeschlossen.
end-of-line-content-title-failure-battery-deeply-discharged = Die Batterie ist tiefentladen.
end-of-line-content-title-failure-maximum-pressure-not-reached = Der maximale Druck konnte nicht erreicht werden.

end-of-line-content-title-failure-leak-too-high = Die Leckluft ist zu hoch.
end-of-line-content-title-failure-minimum-pressure-not-reached = Minimaler Druck konnte nicht erreicht werden.
end-of-line-content-title-failure-oxygen-pressure-not-reached = Sauerstoffdruck konnte nicht erreicht werden.
end-of-line-content-title-failure-pressure-not-stable = Gebläsedruck ist nicht stabil.
end-of-line-content-title-failure-flow-not-stable = Gebläsedurchfluss ist nicht stabil.

end-of-line-content-title-end-confirm = Alle Tests erfolgreich!
end-of-line-content-title-end-display-pressure = Druckdetails werden angezeigt.
end-of-line-content-title-end-display-flow = Anzeige der Flow-Details.

end-of-line-content-message-step-start = Der EOL-Test wird in wenigen Augenblicken beginnen.
end-of-line-content-message-step-check-fan = Die beiden Kühlgebläse sollten arbeiten. Drücken Sie Start, um fortzufahren.
end-of-line-content-message-step-test-battery-dead = Der Zustand der Batterie wird geprüft. Bitte warten Sie.
end-of-line-content-message-step-disconnect-mains = Die Netzspannung verhindert den Batterietest. Bitte ziehen Sie den Netzstecker.
end-of-line-content-message-step-connect-mains = Der Zustand des Netzteils wird geprüft, sobald es an das Stromnetz angeschlossen ist.
end-of-line-content-message-step-check-buzzer = Der Buzzer sollte funktionieren. Drücken Sie zur Bestätigung die Pausetaste.
end-of-line-content-message-step-check-all-buttons = Jede Taste sollte nacheinander gedrückt werden.
end-of-line-content-message-step-check-ui-screen = Bitte ändern Sie den Beatmungsmodus auf dem Touchscreen.
end-of-line-content-message-step-plug-air-test-system = Die Testlunge sollte angeschlossen sein. Drücken Sie Start, um fortzufahren.
end-of-line-content-message-step-reach-maximum-pressure = Der Druck in der Lunge wird erhöht. Bitte warten Sie.
end-of-line-content-message-step-maximum-pressure-reached = Der maximale Druck wurde erreicht. Bitte warten Sie.
end-of-line-content-message-step-start-leak-measure = Das Luftsystem wird auf ein Leck geprüft. Bitte warten Sie.
end-of-line-content-message-step-reach-null-pressure = Es wurde kein Leck festgestellt. Bitte warten Sie.
end-of-line-content-message-step-confirm-before-oxygen-test = Der Sauerstoffanschluss sollte angeschlossen sein. Drücken Sie Start, um fortzufahren.
end-of-line-content-message-step-start-oxygen-test = Der Sauerstoffmischer wird getestet. Bitte warten Sie.
end-of-line-content-message-step-wait-before-blower-long-run = Das Gebläse wird auf Stabilität geprüft. Drücken Sie Start, um fortzufahren.
end-of-line-content-message-step-start-blower-long-run = Das Gebläse wird getestet. Dies wird einige Zeit dauern.

end-of-line-content-message-failure-expander-not-connected = Bitte schließen Sie die Spannungsversorgung an das Motherboard an.
end-of-line-content-message-failure-battery-deeply-discharged = Die Batterie sollte ausgetauscht werden, da sie sich in einem Unterspannungszustand befindet.
end-of-line-content-message-failure-maximum-pressure-not-reached = Der Solldruck konnte im Luftkreislauf nicht erreicht werden.
end-of-line-content-message-failure-leak-too-high = Aus dem Luftkreislauf entweicht zu viel Luft. Bitte überprüfen Sie den Luftkreislauf.
end-of-line-content-message-failure-minimum-pressure-not-reached = Die Ventile haben möglicherweise nicht richtig geöffnet. Bitte überprüfen Sie diese.
end-of-line-content-message-failure-oxygen-pressure-not-reached = Möglicherweise sind die Sauerstoffleitungen verstopft. Bitte überprüfen Sie diese.
end-of-line-content-message-failure-pressure-not-stable = Das Gebläse war nicht in der Lage, einen stabilen Luftdruck aufrechtzuerhalten.
end-of-line-content-message-failure-flow-not-stable = Das Gebläse war nicht in der Lage, einen stabilen Luftstrom aufrechtzuerhalten.

end-of-line-content-message-end-confirm = Dieses Beatmungsgerät hat alle Tests bestanden. Drücken Sie Start, um Details zu sehen.
end-of-line-content-message-end-display-pressure = Details zum Drucktest sind auf dem Debug-Bildschirm sichtbar.
end-of-line-content-message-end-display-flow = Details zum Durchflusstest sind auf dem Fehlerbehebungsbildschirm zu sehen.

end-of-line-content-button-continue = Fortsetzen
