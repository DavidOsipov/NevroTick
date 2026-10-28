# NevroTick v0.2 — изображения и каталог сцен

В этой редакции **43 интерактивные сцены** и **13 снимков изменённых экранов и сценариев**. Не каждой сцене соответствует отдельный JPG. Скриншоты сняты с работающего браузерного прототипа при viewport 1363 × 936; длинные панели могут иметь внутреннюю прокрутку. Данные демонстрационные.

## Снимки v0.2

| Изображение | Что показано |
| --- | --- |
| [copy-summary.jpg](copy-summary.jpg) | Из status-service-failed → Diagnostics → Copy summary: редактированная сводка |
| [diagnostics-expired.jpg](diagnostics-expired.jpg) | Из status-expired → Diagnostics: нет пригодной текущей оценки |
| [diagnostics-failure.jpg](diagnostics-failure.jpg) | Из status-service-failed → Diagnostics: причина, влияние, действие |
| [diagnostics-recovery.jpg](diagnostics-recovery.jpg) | Из status-after-two-helpers → Diagnostics: возраста и происхождение даты |
| [recovery-active-policy.jpg](recovery-active-policy.jpg) | Из status-recovery → Settings → Date recovery: действующая политика с двумя helpers |
| [settings.jpg](settings.jpg) | Три группы настроек; системная тема, трей и sign-in |
| [source-details-recovery.jpg](source-details-recovery.jpg) | Из status-after-two-helpers → Sources → Alpha: подробности и certificate-time basis |
| [sources.jpg](sources.jpg) | Main / Supporting / Not used и причины выбора |
| [status-after-two-helpers.jpg](status-after-two-helpers.jpg) | Нормальная NTS-работа с сохранённой историей восстановления |
| [status-rate-limited.jpg](status-rate-limited.jpg) | Ограниченный Sync now сохраняет текущую оценку и объясняет ожидание |
| [status-service-failed.jpg](status-service-failed.jpg) | Отдельно подтверждённая остановка службы |
| [status-stale.jpg](status-stale.jpg) | Связь со статусом потеряна; здоровье службы неизвестно |
| [status.jpg](status.jpg) | Главный экран: профиль, статус, свидетельства и оценка |

## Все интерактивные сцены

Ссылки открывают автономный HTML на соответствующей сцене. Параметр `scene` задаёт демонстрационный исход; переходы и формы можно исследовать внутри макета. Встроенная панель также позволяет выбирать сцены без URL.

| Сцена | Раздел | Назначение |
| --- | --- | --- |
| [status](../NevroTick-UI-preview.html?scene=status) | Основные окна | Состояние времени |
| [sources](../NevroTick-UI-preview.html?scene=sources) | Основные окна | Серверы времени |
| [source-details](../NevroTick-UI-preview.html?scene=source-details) | Основные окна | Подробности сервера |
| [settings](../NevroTick-UI-preview.html?scene=settings) | Основные окна | Настройки |
| [settings-policy](../NevroTick-UI-preview.html?scene=settings-policy) | Основные окна | Политика времени |
| [source-editor](../NevroTick-UI-preview.html?scene=source-editor) | Редактор и диалоги | Добавить NTS-сервер |
| [source-editor-ntp](../NevroTick-UI-preview.html?scene=source-editor-ntp) | Редактор и диалоги | Добавить NTP-сервер |
| [source-editor-error](../NevroTick-UI-preview.html?scene=source-editor-error) | Редактор и диалоги | Ошибка адреса |
| [review-changes](../NevroTick-UI-preview.html?scene=review-changes) | Редактор и диалоги | Обзор изменений |
| [apply-result](../NevroTick-UI-preview.html?scene=apply-result) | Редактор и диалоги | Изменения применены |
| [apply-cancelled](../NevroTick-UI-preview.html?scene=apply-cancelled) | Редактор и диалоги | Авторизация отменена |
| [apply-conflict](../NevroTick-UI-preview.html?scene=apply-conflict) | Редактор и диалоги | Конфликт конфигурации |
| [recovery](../NevroTick-UI-preview.html?scene=recovery) | Редактор и диалоги | Восстановление даты |
| [correction-approval](../NevroTick-UI-preview.html?scene=correction-approval) | Редактор и диалоги | Большая коррекция |
| [correction-expired](../NevroTick-UI-preview.html?scene=correction-expired) | Редактор и диалоги | Предложение устарело |
| [diagnostics](../NevroTick-UI-preview.html?scene=diagnostics) | Поддержка и запуск | Диагностика |
| [copy-summary](../NevroTick-UI-preview.html?scene=copy-summary) | Поддержка и запуск | Копирование сводки |
| [export](../NevroTick-UI-preview.html?scene=export) | Поддержка и запуск | Экспорт диагностики |
| [about](../NevroTick-UI-preview.html?scene=about) | Поддержка и запуск | О программе |
| [onboarding](../NevroTick-UI-preview.html?scene=onboarding) | Поддержка и запуск | Первая настройка |
| [maintenance](../NevroTick-UI-preview.html?scene=maintenance) | Поддержка и запуск | Установка и обслуживание |
| [unsaved-changes](../NevroTick-UI-preview.html?scene=unsaved-changes) | Поддержка и запуск | Несохранённый черновик |
| [tray](../NevroTick-UI-preview.html?scene=tray) | Поддержка и запуск | Меню трея |
| [status-acquiring](../NevroTick-UI-preview.html?scene=status-acquiring) | Состояния | Сбор измерений |
| [status-holdover](../NevroTick-UI-preview.html?scene=status-holdover) | Состояния | Временная потеря источников |
| [status-conflict](../NevroTick-UI-preview.html?scene=status-conflict) | Состояния | Серверы расходятся |
| [status-expired](../NevroTick-UI-preview.html?scene=status-expired) | Состояния | Оценка устарела |
| [status-offline](../NevroTick-UI-preview.html?scene=status-offline) | Состояния | Нет сети |
| [status-no-sources](../NevroTick-UI-preview.html?scene=status-no-sources) | Состояния | Нет серверов |
| [status-stale](../NevroTick-UI-preview.html?scene=status-stale) | Состояния | Связь со статусом потеряна |
| [status-blocked](../NevroTick-UI-preview.html?scene=status-blocked) | Состояния | Другой владелец часов |
| [status-observe](../NevroTick-UI-preview.html?scene=status-observe) | Состояния | Только наблюдение |
| [status-ntp](../NevroTick-UI-preview.html?scene=status-ntp) | Состояния | NTP без аутентификации |
| [status-recovery](../NevroTick-UI-preview.html?scene=status-recovery) | Состояния | Восстановление NTS |
| [status-date-range](../NevroTick-UI-preview.html?scene=status-date-range) | Состояния | Дата вне диапазона |
| [status-initializing](../NevroTick-UI-preview.html?scene=status-initializing) | Состояния | Запуск и проверка готовности |
| [status-reacquiring](../NevroTick-UI-preview.html?scene=status-reacquiring) | Состояния | Повторная проверка после изменения |
| [status-stopping](../NevroTick-UI-preview.html?scene=status-stopping) | Состояния | Завершение фоновой работы |
| [status-service-failed](../NevroTick-UI-preview.html?scene=status-service-failed) | Состояния | Подтверждённая остановка службы |
| [status-gradual-correction](../NevroTick-UI-preview.html?scene=status-gradual-correction) | Состояния | Плавная коррекция часов |
| [status-rate-limited](../NevroTick-UI-preview.html?scene=status-rate-limited) | Состояния | Ограничение повторного запроса |
| [status-after-one-helper](../NevroTick-UI-preview.html?scene=status-after-one-helper) | Состояния | Работа после одного помощника |
| [status-after-two-helpers](../NevroTick-UI-preview.html?scene=status-after-two-helpers) | Состояния | Работа после двух помощников |

Снимки Diagnostics и Copy summary могут показывать контекст, полученный переходом из другой сцены. Точный путь указан в таблице снимков; например, отдельная сцена copy-summary открывает сводку исходного healthy-контекста, а сохранённый JPG показывает подтверждённую остановку.

[Спецификация интерфейса](../docs/02-ui-spec.md) · [Состояния и сценарии](../docs/04-states-and-acceptance.md) · [Отчёт проверок](../prototype/design-qa.md)
