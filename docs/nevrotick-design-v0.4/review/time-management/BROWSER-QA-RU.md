# Независимая browser QA: startup, отключение, ownership

Проверено 5 ноября 2026. Исходный кандидат: source commit `2435ea21417964803cfae471d56cd0e4a48f200c`, standalone SHA-256 `e6a69799b6a2651b044503beb08447306db39d09107aee795f405c70b7b062d9`. Оба идентификатора сверены до навигации. Требования: `SPEC-RU.md` и `WINDOWS-TIME-RESEARCH.md`, включая разрешённую GPO-передачу, остановку+Disabled W32Time при reverse и сохранение Windows owner при отключении наблюдающего NevroTick.

Итог: **Pass** в пределах HTML browser QA. Финальный source commit `c6e08010ee98e354ef6d01342449e057a06eba1a`; standalone SHA-256 `5970857e19b26f467c67e35a84f5b22dac91a1310939b445418afcd3e28328bb`. Перед финальным retest оба идентификатора сверены. P2-находка исправлена и повторно проверена; открытых замечаний browser QA нет. Изменение от исходного кандидата ограничено локальной подписью журнала и regression tests, поэтому неизменные lifecycle-сценарии не повторялись полностью.

## Исправленная находка P2: retained journal выглядел как текущая NTS-синхронизация

Уверенность высокая. На исходном кандидате `2435ea2` в `src/views.js:232`, после завершённой передачи Windows Time, Diagnostics → Recent events сохранял исходные события с заголовком «Time sync active», возрастом «2 s old» и описанием «Your clock is being maintained under the current policy». Событие Certificate-time basis также говорит «Current packets are NTS authenticated». Рядом с журналом или возрастом нет маркировки pre-operation/retained. Верхний общий banner корректно объясняет историю, однако на мобильном viewport при прокрутке журнала его уже не видно. Current observation и export при этом правильно показывают Windows Time / synchronization not yet verified; модель истории не повышает старые измерения до новых.

Воспроизведение: `NevroTick-UI-v0.4.html?scene=roughtime-recovered` → Power user → Settings → Background service → Use Windows Time → подтвердить → внешний reviewer Deliver sample lifecycle response (confirmed) → Diagnostics → прокрутить Recent events.

Доказательства: `events-360.html`, `events-1280.html` (точный outerHTML), `events-probe.log`, `screenshots/standalone-events-window-360.png`, `screenshots/standalone-events-window-1280.png`. Оба скриншота осмотрены. Исправление: условный локальный retained/pre-operation заголовок/подпись и квалификация возраста при runtime/ownership presentation; исходные записи сохранять. Startup-only не должен менять их текущую маркировку.

Исправление в `c6e0801`: локальный заголовок «Retained events · before service operation» и подпись «Event ages and status wording are from the previous snapshot; they do not describe current synchronization». Исходные title/detail/sequence записей сохранены. В финальном UI это различие видно рядом с журналом и на мобильном экране. Startup-only pending/confirmed сохраняет исходный журнал; startup после runtime сохраняет retained-маркировку; отказ до исполнения возвращает исходный журнал без маркировки. Все проверки прошли для обоих HTML на 360/480/1280. Скриншоты `local-journal-fixed-*` и `standalone-journal-fixed-*` и соответствующие HTML — after, `standalone-events-window-*` — before. After 360/1280 обоих HTML осмотрены.

## Свежие исполняемые проверки

| Команда / проверка | Охват | Результат |
| --- | --- | --- |
| `OWNERSHIP_SCREENSHOT_DIR=... node tests/startup-ownership.browser.cjs` | Оба HTML, 360/1280; startup pending/success/unknown и DEBUG, Disable/Enable, preflight, partial/unknown, forward/reverse, observation-only Start, focus/reset, all views/export | Pass; 4 matrix cases, без JS-ошибок/внешних запросов |
| `node tests/service-controls.browser.cjs` | Оба HTML, 360/1280; старые Start/Stop/Restart, UAC/rejected/unknown, read-only Start, source history/export, DEBUG и фокус | Pass; 4 cases |
| `NAV_TEST_FILES=index.html,NevroTick-UI-v0.4.html node tests/navigation-reset.browser.cjs` | Оба HTML, 360/980; source/filter/undo keyboard focus, reset guards active/partial/unknown, pending Stop | Pass; 4 cases |
| `node tests/roughtime.browser.cjs` | Оба HTML, 360/480/1280; 9 сцен, 6 поверхностей, ancestry, возврат, draft guard | Pass; 54 matrix cases |
| `node tests/service-recovery-history.browser.cjs` | Оба HTML, 360/1280; 4 сцены, pending/stopped/restarted | Pass; 48 stage cases, 96 surfaces |
| `node ../final-qa/journal-retest.cjs` (финальный `c6e0801`) | Оба HTML, 360/480/1280; fix, неизменный startup-only, startup-after-runtime retained, rejected pre-effect, сохранение raw events, Windows unverified, финальные screenshots | Pass; 6 cases, без JS-ошибок/внешних запросов |
| `node ../final-qa/independent-probe.cjs` | Независимая дополнительная проба обоих HTML, 360/480/1280 | Pass; 6 cases; ноль JS-ошибок/внешних запросов |

Логи каждой команды находятся рядом с отчётом. Независимая проба содержит UI-only действия без обращения к внутреннему state/reducer: точное сохранение DEBUG при startup UAC-cancel/rejected; сохранение pending при навигации; разрешённая managed-policy передача; Status/Sources/Statistics/Recovery/Diagnostics/Settings/policy/export; Windows running не превращается в synchronized; Start+startup On+Disable наблюдателя сохраняют Windows owner; reverse product-unavailable оставляет No active synchronizer и отключённый W32Time. Сырые тексты поверхностей — `independent-probe-evidence.json`.

В ранних версиях внешнего probe исправлены только ошибки самой проверки: регистр слова pending, маршрут Explore recovery, слишком широкая проверка текста historical events и формат отдельных runtime/startup фактов. Сохранены `independent-probe-attempt*.log`. Production-файлы и repository tests QA-агент не изменял.

## Осмотр скриншотов

Осмотрены свежие изображения обоих HTML на 360/1280: Windows owner в Settings/Status, full disable, policy blocker, cleanup partial, reverse reacquiring, startup Off при Running. На 480 дополнительно осмотрены Settings, Recovery, reverse partial и forward confirmation plan. Переносы текста, controls и факты читаемы; горизонтального overflow проверки не обнаружили. Единственное замечание к локальной маркировке событий исправлено; финальные focused screenshots обоих HTML на 360/1280 осмотрены. Windows Status360/Settings1280 пересняты с финального standalone hash (`*-final-windows-*`) и осмотрены.

Скриншоты в `screenshots/` и `navigation-screenshots/`. Baseline Roughtime suite дополнительно создала свой набор в `/tmp/nevrotick-roughtime-browser`.

## Граница доказательств

Это Chromium-проверка HTML-прототипа, не нативная Windows-квалификация. Реальные службы, реестр, UAC, конфигурация ОС, системные часы и сеть синхронизации не вызывались и не изменялись. Проверка source/security/CSP/AVIF/package выполняется отдельными участниками; этот отчёт не подменяет их выводы.

## Архивная копия

Полный рабочий каталог проверки: `/workspace/nevrotick-ui-audit/ownership-work/final-qa`. В комплекте сохранены выбранные before/after и финальные снимки в `screenshots/`; логи, точные HTML и JSON-доказательства — в `evidence/`. Скрипты независимых проб и невыбранные снимки остаются в полном рабочем каталоге; воспроизводимые интеграционные проверки находятся в корневом `tests/`.
