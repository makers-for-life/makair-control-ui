telemetry-label-peak = Пик.давл
telemetry-label-plateau = Давление плато
telemetry-label-expiratory = ПДКВ
telemetry-label-cycles = Циклов в минуту
telemetry-label-ratio = Продолжительность вдоха
telemetry-label-ratio-details = соотн. вдох-выдох
telemetry-label-tidal = Дыхательный объем
telemetry-label-minute-volume = Минутный объем

telemetry-unit-cmh2o = cmH2O
telemetry-unit-mmh2o = mmH2O
telemetry-unit-lpm = Л/мин
telemetry-unit-mlpm = мл/мин
telemetry-unit-per-minute = /минута
telemetry-unit-milliliters = миллилитров
telemetry-unit-milliseconds = миллисекунды
telemetry-unit-centimeters = см

alarms-title = ТРЕВОГИ
alarms-empty = Тревоги нет.

alarms-message-plateau-pressure-not-reached = Давление плато не достигается
alarms-message-patient-unplugged = Пациент отключен от сети
alarms-message-peep-pressure-not-reached = Давление на выдохе не достигается
alarms-message-battery-low = Батарея разряжена
alarms-message-battery-very-low = Батарея очень разряжена
alarms-message-power-cable-unplugged = Кабель питания отключен
alarms-message-pressure-too-high = Давление слишком высокое
alarms-message-inspiratory-minute-volume-low = Минутный объем вдоха слишком низкий
alarms-message-inspiratory-minute-volume-high = Минутный объем вдоха слишком велик
alarms-message-expiratory-minute-volume-low = Минутный объем выдоха слишком мал
alarms-message-expiratory-minute-volume-high = Минутный объем выдоха слишком велик
alarms-message-respiratory-rate-low = Частота дыхания слишком низкая
alarms-message-respiratory-rate-high = Частота дыхания слишком высокая
alarms-message-leak-high = Объем утечки слишком велик
alarms-message-tidal-volume-low = Дыхательный объем слишком низкий
alarms-message-tidal-volume-high = Дыхательный объем слишком высокий
alarms-message-unknown = Неизвестный сигнал тревоги

status-unit-stopped = Остановлен
status-unit-active = Активный
status-power-battery = Батарея
status-power-ac = Кабель

mode-class-pc = Давление
mode-class-vc = Объем
mode-type-cmv = CMV
mode-type-ac = AC
mode-type-vsai = VSAI
mode-group-general = Генеральный
mode-group-alarms = Будильники

advanced-group-statistics = Статистика
advanced-group-settings = Настройки

modal-close = Закрыть
modal-apply = Применять
modal-cancel = Отменить

modal-preset-title = Здравствуйте. Пожалуйста, настройте данные вашего пациента.
modal-preset-subtitle = Будут применены предустановленные настройки, которые впоследствии можно будет изменить.
modal-preset-gender = Пол
modal-preset-gender-male = мужчина
modal-preset-gender-female = женский
modal-preset-age = Возрастная группа
modal-preset-size = Размер тела
modal-preset-age-child = Ребенок
modal-preset-age-teenager = Подросток
modal-preset-age-adult = Взрослый

modal-run-status = Состояние вентиляционной установки
modal-run-status-started = Бег - Стоп?
modal-run-status-stopped = Остановлен - Пуск?

modal-snooze-alarms = Будильники
modal-snooze-alarms-active = Звуковые сигналы включены
modal-snooze-alarms-inactive = Звуковые сигналы отключены

modal-mode-pressure-inspiratory = Давление на вдохе
modal-mode-pressure-expiratory = Давление на выдохе
modal-mode-time-inspiratory = Время вдоха
modal-mode-time-inspiratory-minimum = Время вдоха (минимум)
modal-mode-time-inspiratory-maximum = Время вдоха (максимальное)
modal-mode-flow-inspiratory = Вдыхаемый поток
modal-mode-cycles-per-minute = Циклов в минуту
modal-mode-tidal-volume = Дыхательный объем
modal-mode-plateau-duration = Продолжительность плато
modal-mode-trigger-offset = Смещение триггера
modal-mode-trigger-expiratory = Триггер выдоха
modal-mode-alarm-low-inspiratory-minute-volume = Объем вдоха (низкий)
modal-mode-alarm-high-inspiratory-minute-volume = Объем вдоха (высокий)
modal-mode-alarm-low-expiratory-minute-volume = Объем выдоха (низкий)
modal-mode-alarm-high-expiratory-minute-volume = Объем выдоха (высокий)
modal-mode-alarm-low-respiratory-rate = Частота выдоха (низкая)
modal-mode-alarm-high-respiratory-rate = Частота выдоха (высокая)
modal-mode-alarm-low-tidal-volume = Дыхательный объем (низкий)
modal-mode-alarm-high-tidal-volume = Дыхательный объем (высокий)
modal-mode-alarm-leak = Объем утечки
modal-mode-alarm-peak-pressure = Пиковое давление

modal-advanced-locale = Язык
modal-advanced-date = Свидание
modal-advanced-time = Время
modal-advanced-timezone = Часовой пояс

initializing-connecting = Начиная...
initializing-connected = Инициализация...

error-title-no-device = Ой. Не удалось достучаться до ядра.
error-title-timed-out = Ой. Не удалось инициализировать ядро.
error-title-bad-protocol = Общая ошибка протокола телеметрии.
error-title-watchdog = Эээ. Контроллер вентиляции сломался.
error-title-sensor-failure = Датчик отключен. Безопасность поставлена под угрозу.
error-title-other = Ой. Произошла неизвестная ошибка.

error-message-no-device = Невозможно связаться с мастером телеметрии. Это правильно настроено?
error-message-timed-out = Некоторые компоненты могли не запуститься. Можете ли вы попробовать велоспорт?
error-message-bad-protocol = Прошивка использует неподдерживаемый протокол телеметрии. Обновите программное обеспечение.
error-message-watchdog = Сработал сторожевой таймер, и, таким образом, внутренний микроконтроллер перезапустился.
error-message-sensor-failure = ID датчика:
error-message-other = Причина:

stop-title = Аппарат ИВЛ не активен
stop-description = Включите снова, чтобы проветрить
