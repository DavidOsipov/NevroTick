# Свежий браузерный аудит статусов — исходный кандидат

5 ноября 2026. Read-only аудит исходного кандидата `ed523ef5aff4e6d04619419398dfaba7c60f6005`, до добавления уведомлений. Обновлённый макет должен пройти отдельную финальную приёмку. Производственные файлы агент не менял.

Идентичность HTML при захвате и после него совпала:

- `index.html`: `837677ac278a8c9a01297387dfbe7415d78f8d4052769bf80bfe62b66794b7ef`.
- `NevroTick-UI-v0.4.html`: `5970857e19b26f467c67e35a84f5b22dac91a1310939b445418afcd3e28328bb`.

Chromium открывает оба HTML через локальный HTTP, макет виден, изображения загружаются. Реальные Windows-операции и уведомления не выполнялись.

## Охват и доказательства

- Все 30 сцен, оба HTML, Simple/Power user, 360/1280 px: **240 захватов Status** с DOM, действиями, бейджами и геометрией.
- Все 30 сцен в standalone Power на 360 px: **180 дополнительных поверхностей** Sources, Statistics, Diagnostics, Settings, Background service и Date recovery.
- 11 сервисных/ownership-состояний на 360/1280: **88 поверхностей** Settings/Power Status/Diagnostics/Simple Status; 66 дополнительных снимков. Stop Pending/Unknown; full disable; cleanup-unconfirmed; Windows unavailable после освобождения; handover unknown; Windows owner; running/stopped Nv observer; reverse partial; startup unknown.
- Отдельный lostACK после включения Nv observer при Windows owner: ещё два захвата и before/after факты.
- **120 проверок точного текстового соответствия** двух сборок для одинаковых Status/ширины/режима.
- **0 ошибок JavaScript**, **0 неожиданных удалённых запросов**, **0 переполнений документа**. Встроенные таблицы с осознанной горизонтальной прокруткой не выдаются за ошибку ширины документа.

Каждый Status прочитан через свежий DOM и геометрию; ключевые сохранённые снимки ниже открыты и осмотрены визуально. Не заявляется отдельная ручная визуальная инспекция всех 308 файлов. Текст/действия всех сцен находятся в `status-evidence.json`; lifecycle — `lifecycle-evidence.json`; отдельный lostACK — `windows-observer-unknown.json`. Скрипты, логи и снимки сохранены рядом.

## Пройденные шаги

| Шаг | Состояние | Оценка и осмотренное доказательство |
| --- | --- | --- |
| 1 | Здоровая синхронизация | Ясно отделены NTS-аутентификация, согласие, оценка/неопределённость, возраст. [Simple 360](screenshots/local-360-simple-healthy.png). |
| 2 | Частичное ограничение/отказ источника, holdover, offline | Не объявляется остановка службы или гарантированная причина сети; usable prior model обозначен отдельно. [DENY 360](screenshots/standalone-360-simple-server-access-refused.png). |
| 3 | Конфликт источников | Новые коррекции приостановлены; authenticated не выдаётся за agreement. [Power 1280](screenshots/standalone-1280-power-conflict.png). |
| 4 | Потеря текущего статуса | Служба может продолжать работу; last snapshot и отсутствие текущей оценки обозначены. Основное действие нуждается в уточнении, см. V01. [Simple 360](screenshots/local-360-simple-stale.png). |
| 5 | Roughtime rejected/conflict | Нет ложного «атака доказана», принятой даты, NTS-согласия или изменения часов. [Rejected 360](screenshots/standalone-360-simple-roughtime-rejected.png), [Conflict 360](screenshots/local-360-simple-signed-conflict.png). |
| 6 | Clock-change outcome unknown | Не объявляется отказ или отсутствие эффекта; нужен reconcile, нет текущего offset/votes. Название основного действия неоднозначно, см. V01. [Power 360](screenshots/standalone-360-power-outcome-unknown.png). |
| 7 | Runtime Pending/Unknown | Текущая синхронизация/health не утверждается, исторический snapshot квалифицирован; enabled Sync now не соответствует доступному действию. [Pending Simple 360](screenshots/lifecycle-360-stop-pending-simple.png). |
| 8 | Windows Time owner / запуск и остановка observer | Windows owner отдельно от synchronization; текущего NTS-offset/votes нет. [Windows Power 1280](screenshots/lifecycle-1280-windows-status.png). LostACK Nv observer требует исправления V02. |
| 9 | Partial cleanup / нет synchronizer | Не объявляется full success; uncertainty и отсутствие автоматической синхронизации различаются. [Cleanup Simple 360](screenshots/lifecycle-360-cleanup-unconfirmed-simple.png). |
| 10 | Startup-only unknown | Текущая здоровая синхронизация и её данные сохраняются; неопределённость конфигурации не превращается в runtime outage. DOM/lifecycle evidence. |

## V01 — P2: недоступное или двусмысленное главное действие

**Подтверждённая часть:** `?scene=service-stopped` → Power user → Status: enabled primary **Sync now**, хотя рядом confirmed stopped и synchronization unavailable. При Stop → Pending/Unknown в обоих режимах также остаётся enabled **Sync now**, хотя инструкция требует Diagnose и запрет слепого повтора. Не было обнаружено настоящего clock retry: reducer создаёт ограниченную тестовую обратную связь. Проблема в обещании действия и приоритетах экрана.

Область: `src/views.js:23–30`, `syncButton`; stopped/unknown проверяются только при `timeManagement.presentation`, после чего остальные состояния получают общий Sync now. В обычном `service-stopped` Simple отдельно заменяет действие на Open diagnostics, а Power — нет.

Минимальное исправление: выводить главное действие по runtime availability/pending/unknown, независимо от наличия timeManagement presentation. Для confirmed stopped — открыть настройки службы; pending/unknown — явная диагностика или недоступная проверка с объяснением. Не выводить stopped из потери соединения. Для observe/blocked/clock-result-unknown предпочтительно **Check sources**, чтобы не обещать изменение часов или повтор неопределённой операции; это та же семья ясности, а не найденное право на запись.

Доказательства: `standalone-360-power-service-stopped.png`, `lifecycle-360-stop-pending-simple.png`, `standalone-360-power-outcome-unknown.png`, `status-evidence.json`, `lifecycle-evidence.json`.

## V02 — P2, согласовано с source P2-S02: Nv observer lostACK стирает Windows owner

Воспроизведение: Settings → Background service → Use Windows Time → confirmed sample → Enable and start NevroTick → неизвестный sample service response → Simple Status.

Результат: **Time management is unconfirmed / Current owner unknown**. При этом W32Time остаётся Running/Manual trigger-start, его конфигурация не менялась, передача владельца не запрашивалась. Неизвестный исход Nv-only команды должен остаться неопределённостью Nv runtime, сохранив отдельно подтверждённое Windows management и unverified Windows sync.

[Осмотренный снимок 360](screenshots/windows-observer-unknown-360.png), `windows-observer-unknown.json` содержит точные before/after факты. Тот же результат захвачен на 1280. Source-аудитор подтвердил P2-S02 и передал координатору; исправление включено в спецификацию этой итерации.

## Доступность и границы

Текстовые статусы, доступные навигационные кнопки и подсказки дополняют цвет; в осмотренных узких снимках тексты/действия читаются, контейнеры не обрезают содержимое. Скриншоты не устанавливают полную поддержку Narrator/UIA, контрастность всех комбинаций, Windows activation или delivery уведомлений. Нативная Windows и уведомления здесь не проверены. Новая функция уведомлений требует самостоятельной проверки маршрутов, дедупликации, подавления Windows, dismiss, актуальности и отсутствия привилегированного действия по нажатию.
