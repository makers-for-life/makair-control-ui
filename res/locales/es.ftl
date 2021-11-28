telemetry-label-peak = P(Pico)
telemetry-label-plateau = P(meseta)
telemetry-label-expiratory = P(exhalación)
telemetry-label-cycles = Ciclos/minutos
telemetry-label-ratio = Duración inspiratoria
telemetry-label-ratio-details = relación insp-exp
telemetry-label-tidal = Volumen corriente
telemetry-label-minute-volume = Volumen minuto

telemetry-unit-cmh2o = cmH2O
telemetry-unit-mmh2o = mmH2O
telemetry-unit-lpm = L/min
telemetry-unit-mlpm = mL/min
telemetry-unit-per-minute = /minuto
telemetry-unit-milliliters = mL
telemetry-unit-milliseconds = ms
telemetry-unit-centimeters = cm

alarms-title = ALARMAS
alarms-empty = Ninguna alarma.

alarms-message-plateau-pressure-not-reached = No se alcanza la presión de meseta
alarms-message-patient-unplugged = El paciente está desenchufado
alarms-message-peep-pressure-not-reached = No se alcanza la presión espiratoria
alarms-message-battery-low = La batería está baja
alarms-message-battery-very-low = La batería está muy baja
alarms-message-power-cable-unplugged = El cable de alimentación está desenchufado
alarms-message-pressure-too-high = La presión es demasiado alta
alarms-message-inspiratory-minute-volume-low = El volumen minuto inspiratorio es demasiado bajo
alarms-message-inspiratory-minute-volume-high = El volumen minuto inspiratorio es demasiado alto
alarms-message-expiratory-minute-volume-low = El volumen minuto espiratorio es demasiado bajo
alarms-message-expiratory-minute-volume-high = El volumen minuto espiratorio es demasiado alto
alarms-message-respiratory-rate-low = La frecuencia respiratoria es demasiado baja
alarms-message-respiratory-rate-high = La frecuencia respiratoria es demasiado alta.
alarms-message-leak-high = El volumen de la fuga es demasiado alto
alarms-message-tidal-volume-low = El volumen corriente es demasiado bajo
alarms-message-tidal-volume-high = El volumen corriente es demasiado alto
alarms-message-peak-pressure-high = La presión máxima es demasiado alta
alarms-message-expiratory-flow-too-low = El flujo espiratorio es demasiado bajo
alarms-message-unknown = Alarma desconocida (código de verificación)

status-unit-stopped = Unidad detenida
status-unit-active = Unidad activa
status-power-battery = Batería
status-power-ac = Corriente alterna

mode-class-pc = Presión
mode-class-vc = Volumen
mode-type-cmv = CMV
mode-type-ac = AC
mode-type-vsai = VSAI
mode-group-general = General
mode-group-alarms = Alarmas

advanced-group-statistics = Estadísticas
advanced-group-settings = Configuraciones
advanced-group-simulator = Simulator


modal-close = Cerca
modal-apply = Aplicar
modal-cancel = Cancelar

modal-preset-title = Hola. Configure los datos de su paciente.
modal-preset-subtitle = Se aplicarán los ajustes preestablecidos, que se pueden ajustar posteriormente.
modal-preset-gender = Sexo
modal-preset-gender-male = Masculino
modal-preset-gender-female = Hembra
modal-preset-age = Grupo de edad
modal-preset-size = Tamaño corporal
modal-preset-age-child = Niño
modal-preset-age-teenager = Adolescente
modal-preset-age-adult = Adulto

modal-run-status = Estado de la unidad de ventilación
modal-run-status-started = Corriendo - ¿Parar?
modal-run-status-stopped = Detenido - ¿Empezar?

modal-snooze-alarms = Alarmas
modal-snooze-alarms-active = Pitidos habilitados
modal-snooze-alarms-inactive = Pitidos desactivados

modal-mode-pressure-inspiratory = Presión inspiratoria
modal-mode-pressure-expiratory = Presión espiratoria
modal-mode-time-inspiratory = Tiempo inspiratorio
modal-mode-time-inspiratory-minimum = Tiempo inspiratorio (mínimo)
modal-mode-time-inspiratory-maximum = Tiempo inspiratorio (máximo)
modal-mode-flow-inspiratory = Flujo inspiratorio
modal-mode-cycles-per-minute = Ciclos por minuto
modal-mode-tidal-volume = Volumen corriente
modal-mode-plateau-duration = Duración de la meseta
modal-mode-trigger-offset = Desplazamiento del disparador
modal-mode-trigger-expiratory = Gatillo espiratorio
modal-mode-alarm-low-inspiratory-minute-volume = Volumen inspiratorio (bajo)
modal-mode-alarm-high-inspiratory-minute-volume = Volumen inspiratorio (alto)
modal-mode-alarm-low-expiratory-minute-volume = Volumen espiratorio (bajo)
modal-mode-alarm-high-expiratory-minute-volume = Volumen espiratorio (alto)
modal-mode-alarm-low-respiratory-rate = Tasa espiratoria (baja)
modal-mode-alarm-high-respiratory-rate = Tasa espiratoria (alta)
modal-mode-alarm-low-tidal-volume = Volumen corriente (bajo)
modal-mode-alarm-high-tidal-volume = Volumen corriente (alto)
modal-mode-alarm-leak = Volumen de fuga (alto)
modal-mode-alarm-peak-pressure = Presión pico (alto)

modal-advanced-locale = Idioma
modal-advanced-date = Fecha
modal-advanced-time = Hora
modal-advanced-timezone = Zona horaria

modal-advanced-simulator-airway-resistance = Resistance
modal-advanced-simulator-airway-compliance = Compliance
modal-advanced-simulator-spontaneous-breath-rate = Breath rate
modal-advanced-simulator-spontaneous-breath-effort = Breath effort
modal-advanced-simulator-spontaneous-breath-duration = Breath duration
modal-advanced-simulator-acceleration-factor = Acceleration percent

modal-advanced-simulator-airway-resistance-unit = cmH2O/L/s
modal-advanced-simulator-airway-compliance-unit = mL/cmH2O
modal-advanced-simulator-spontaneous-breath-rate-unit = cycle/min
modal-advanced-simulator-spontaneous-breath-effort-unit = cmH2O
modal-advanced-simulator-spontaneous-breath-duration-unit = ms
modal-advanced-simulator-acceleration-factor-unit = %

initializing-connecting = Empezando...
initializing-connected = Inicializando...

error-title-no-device = ¡Ups! No se pudo alcanzar el núcleo.
error-title-timed-out = ¡Ups! Error al inicializar el núcleo.
error-title-bad-protocol = Error general del protocolo de telemetría.
error-title-watchdog = Uh. El controlador de ventilación se ha bloqueado.
error-title-sensor-failure = Un sensor está fuera de línea. La seguridad está comprometida.
error-title-other = ¡Ups! Un error desconocido ocurrió.

error-message-no-device = No se puede conectar con el maestro de telemetría. ¿Está configurado correctamente?
error-message-timed-out = Es posible que algunos componentes no se hayan iniciado. ¿Puedes probar el ciclo de potencia?
error-message-bad-protocol = El firmware está usando un protocolo de telemetría no compatible. Actualice el software.
error-message-watchdog = Se ha activado un temporizador de vigilancia y, por tanto, se ha reiniciado el microcontrolador interno.
error-message-sensor-failure = ID del sensor:
error-message-other = Razón:

stop-title = Unidad de ventilador inactiva
stop-description = Por favor re-active para reanudar respiración

end-of-line-title-primary = Modo de servicio
end-of-line-title-secondary = Prueba de fin de línea

end-of-line-content-title-step-start = Iniciando la prueba de fin de línea...
end-of-line-content-title-step-check-fan = Por favor, compruebe los ventiladores.
end-of-line-content-title-step-test-battery-dead = Comprobando el estado de la batería...
end-of-line-content-title-step-disconnect-mains = Desenchufe el cable de alimentación de CA.
end-of-line-content-title-step-connect-mains = Por favor, enchufe el cable de alimentación de CA.
end-of-line-content-title-step-check-buzzer = Probando el zumbido...
end-of-line-content-title-step-check-all-buttons = Por favor, pulse todos los botones.
end-of-line-content-title-step-check-ui-screen = Pulse la pantalla táctil.
end-of-line-content-title-step-plug-air-test-system = Por favor, enchufe el sistema de prueba de pulmón.
end-of-line-content-title-step-reach-maximum-pressure = Probando la presión máxima...
end-of-line-content-title-step-maximum-pressure-reached = Presión máxima alcanzada. Cerrando válvulas...
end-of-line-content-title-step-start-leak-measure = Probando fugas...
end-of-line-content-title-step-reach-null-pressure = No se ha detectado ninguna fuga. Abriendo válvulas...
end-of-line-content-title-step-confirm-before-oxygen-test = Por favor, tapone el sistema de prueba de oxígeno.
end-of-line-content-title-step-start-oxygen-test = Probando el oxígeno...
end-of-line-content-title-step-wait-before-blower-long-run = Por favor, desenchufe el sistema de prueba de oxígeno.
end-of-line-content-title-step-start-blower-long-run = Probando la estabilidad del expansor...

end-of-line-content-title-failure-expander-not-connected = El expansor no está conectado.
end-of-line-content-title-failure-battery-deeply-discharged = La batería está muy descargada.
end-of-line-content-title-failure-maximum-pressure-not-reached = No se ha podido alcanzar la presión máxima.

end-of-line-content-title-failure-leak-too-high = La fuga de aire es demasiado alta.
end-of-line-content-title-failure-minimum-pressure-not-reached = No se ha podido alcanzar la presión mínima.
end-of-line-content-title-failure-oxygen-pressure-not-reached = No se ha podido alcanzar la presión de oxígeno.
end-of-line-content-title-failure-pressure-not-stable = La presión del soplador no es estable.
end-of-line-content-title-failure-flow-not-stable = El flujo del soplador no es estable.

end-of-line-content-title-end-confirm = Todas las pruebas se han realizado con éxito.
end-of-line-content-title-end-display-pressure = Visualización de los detalles de la presión.
end-of-line-content-title-end-display-flow = Visualización de los detalles del flujo.

end-of-line-content-message-step-start = La prueba EOL comenzará en unos momentos.
end-of-line-content-message-step-check-fan = Los dos ventiladores de refrigeración deberían funcionar. Pulse el botón de inicio para continuar.
end-of-line-content-message-step-test-battery-dead = Se comprobará el estado de la batería. Por favor, espere.
end-of-line-content-message-step-disconnect-mains = La alimentación de CA impide la comprobación de las baterías. Desenchúfela.
end-of-line-content-message-step-connect-mains = La salud de la fuente de alimentación se comprobará una vez enchufada a la CA.
end-of-line-content-message-step-check-buzzer = El zumbador debería funcionar. Pulse la pausa para confirmar.
end-of-line-content-message-step-check-all-buttons = Cada botón debe ser presionado uno por uno.
end-of-line-content-message-step-check-ui-screen = Cambie el modo de ventilación en la pantalla táctil.
end-of-line-content-message-step-plug-air-test-system = El pulmón de prueba debe estar conectado. Pulse Inicio para continuar.
end-of-line-content-message-step-reach-maximum-pressure = La presión en el pulmón está aumentando. Por favor, espere.
end-of-line-content-message-step-maximum-pressure-reached = Se ha alcanzado la presión máxima. Espere.
end-of-line-content-message-step-start-leak-measure = Se está comprobando si el sistema de aire tiene alguna fuga. Por favor, espere.
end-of-line-content-message-step-reach-null-pressure = No se ha detectado ninguna fuga. Por favor, espere.
end-of-line-content-message-step-confirm-before-oxygen-test = El puerto de oxígeno debería estar conectado. Pulse Inicio para continuar.
end-of-line-content-message-step-start-oxygen-test = Se está probando el mezclador de oxígeno. Por favor, espere.
end-of-line-content-message-step-wait-before-blower-long-run = Se comprobará la estabilidad del soplador. Pulse Inicio para continuar.
end-of-line-content-message-step-start-blower-long-run = Se está probando el soplador. Esto llevará algún tiempo.

end-of-line-content-message-failure-expander-not-connected = Por favor, conecte la fuente de alimentación a la placa base.
end-of-line-content-message-failure-battery-deeply-discharged = La batería debe ser reemplazada, ya que se encuentra en un estado de bajo voltaje.
end-of-line-content-message-failure-maximum-pressure-not-reached = No se ha podido alcanzar la presión objetivo en el circuito de aire.
end-of-line-content-message-failure-leak-too-high = El sistema de aire pierde demasiado aire. Compruebe el circuito de aire.
end-of-line-content-message-failure-minimum-pressure-not-reached = Es posible que las válvulas no se hayan abierto correctamente. Compruébelas.
end-of-line-content-message-failure-oxygen-pressure-not-reached = Los tubos de oxígeno pueden estar bloqueados. Revíselos.
end-of-line-content-message-failure-pressure-not-stable = El soplador no ha podido mantener una presión de aire estable.
end-of-line-content-message-failure-flow-not-stable = El ventilador no pudo mantener un flujo de aire estable.

end-of-line-content-message-end-confirm = Este ventilador ha superado todas las pruebas. Pulse el botón de inicio para ver los detalles.
end-of-line-content-message-end-display-pressure = Los detalles de la prueba de presión son visibles en la pantalla de depuración.
end-of-line-content-message-end-display-flow = Los detalles de la prueba de flujo son visibles en la pantalla de depuración.

end-of-line-content-button-continue = Continuar
