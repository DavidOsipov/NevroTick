# Независимый source/semantic review статусов и Windows notifications

5 ноября 2026 года. Scope: интерактивный HTML-прототип. Финальный production source проверен по immutable **`62aa9713548f8bfefc7ea087a01d788bc6f39c10`** над `7f0a933c381f1092fde563fdd9c64c71f9f0d41e` (status-fix `cd90ed04dbd26e8730204efd123fd22c80d01db6`, исходная база `ed523ef5aff4e6d04619419398dfaba7c60f6005`). Полный первоначальный review выполнен для `7f0a933`, затем прочитана вся receiving delta и повторены targeted/matrix проверки на финальном source. Основание: binding `SPEC-RU.md`, `windows-notifications-research.md`, `status-source-audit.md` и `status-design-review.md`.

Для review использован `git archive` snapshot `/tmp/nevrotick-notifications-review-62aa971`, не mutable рабочие файлы implementer. В snapshot выполнена воспроизводимая сборка, получен ровно назначенный standalone SHA-256 **`4d8c3d211ec19ddea21a4b37d6b164f453a8ab8352cca6c554c8c8ac28370f86`**. Production/source/test-файлы проверяющий не менял.

## Финальный вердикт

**PASS в пределах source/model/view проверки HTML-прототипа. Открытых материальных замечаний нет.** Основные status, classification, OS-readiness и activation границы соблюдены. Предыдущие две view-находки `cd90ed0` и две episode/outcome-находки `7f0a933` исправлены и повторно проверены. Нативной доставки Windows/доступности этим результатом не подтверждается.

В финальной delta текущая unresolved проблема хранится отдельно от последнего inline-ответа: `timeManagement.currentIssue`, `timeManagement.startupIssue`, `serviceControl.currentIssue`. Ключ привязан к породившему проблему `scopeSerial`, а не к последней любой попытке. Cancel/rejected-before-effect сохраняют саму проблему и её identity; настоящий подтверждённый owner transition очищает соответствующий прежний issue, не создавая fresh synchronization. Runtime Start без подтверждённого manager не очищает failed-handover проблему и не оставляет текст о прежнем Stopped как текущую истину.

## Что подтверждено

| Требование | Свежая независимая проверка | Результат |
| --- | --- | --- |
| P2-S01 unknown owner / observe-only | Qualified presentation без переписывания raw evidence; отсутствие мнимого Windows owner | PASS |
| P2-S02 model и feedback | Windows observer Start/Stop/Restart unknown сохраняет Windows owner/sync-unverified и квалифицирует только Nv runtime/startup; feedback больше не противоречит фактам | PASS |
| Tray action и ARIA | Все 30 tray-from-Settings renders: нет отсутствующего `sync-response` target; stopped/unknown/pending actions безопасны | PASS |
| Явная policy 30 сцен | 1 440 комбинаций scenes × persistence × OS availability × presenter × app preference; independent expected immediate/soft/none matrix | PASS |
| Planned actions и runtime axes | Stop/Restart/Disable/successful handover не unexpected outage; неизвестные/частичные исходы в своей области; старые голоса/offset/recovery не current | PASS в проверенных обычных переходах |
| Readiness / Off / DND / presenter | Не показывают preview, не расходуют первый incident, не выдают On за Windows permission; возврат готовности не повторяет уже показанный эпизод | PASS |
| Dedup / dismiss / navigation | Обычные повторные observations и navigation/dismiss не создают новый notification; dismissal не исправляет current problem | PASS для стабильного incident scope |
| Existing clock problem при no-effect retry | Conflict/expired и startup unknown → pending gap → cancel/rejected → publish не дублируются; все 12 partial retry-путей сохраняют первый ещё не показанный alert или announced identity | PASS |
| Activation | Невалидные IDs игнорируются; generation/proposal/stale IDs не оживляют clock approval; fresh action только открывает current context; dirty recovery editor guard сохраняется | PASS |
| Privacy и native boundary | Фиксированный generic payload, нет host/IP/keys/точного offset; нет privileged action из toast, browser Notification/ServiceWorker/Push/OS URI/native effect | PASS в пределах source/HTML |
| Raw facts | Notification actions не меняют scenario, service/time-management facts и diagnostic log; старые NTS/Roughtime данные не освежаются | PASS |

## Исправлено P2-N01: unresolved partial problem после no-effect повторной попытки

Ниже воспроизведение дефекта на прежнем `7f0a933`; на финальном `62aa971` все 12 путей проходят.

Repro:

1. Healthy → Use Windows Time → sample `windows-unavailable`: current **No active synchronizer**, eligible `handover-windows-unavailable`.
2. Ещё раз Use Windows Time → просто Close диалог **или** подтвердить, затем UAC-cancelled/rejected-before-effect.
3. Publish current status для notifications.

Clock/service facts не исправились: current Status по-прежнему No active synchronizer. Но `close` и `sample-lifecycle-deliver` заменяют `timeManagement.result` на cancelled/uac-cancelled/failed. `notificationCandidate` опирался только на этот last-command result; теперь он null, а `c.presentation/t.presentation` запрещают восстановить исходный partial incident. Если OS раньше блокировала показ, исходная важная проблема после возвращения готовности уже никогда не показывается.

Также воспроизведено для `product-unavailable` после обратной передачи. Независимая матрица: **2 partial outcomes × 3 no-effect paths × 2 prior-shown states = 12** воспроизведений потери current candidate.

В `62aa971` unresolved current operation problem сохранена независимо от последней попытки/отмены. No-effect result не меняет её scope, а показанный ранее incident не получает новый dedup key. Все 12 повторных проверок прошли.

## Исправлено P2-N02: старый startup unknown после чужой отменённой runtime-команды

Ниже воспроизведение дефекта на прежнем `7f0a933`; на финальном `62aa971` прежний effect-scope сохраняется, повторного preview нет.

Repro:

1. Healthy → startup-off → unknown → Publish: `autostart-outcome-unknown:attention:generation-12:request-1`, одна запись.
2. Runtime Stop → pending → Publish gap → UAC-cancelled или rejected-before-effect.
3. Publish: прежний startup unknown восстановлен без изменения startup, но notification key стал `...:request-2`; появляется вторая одинаковая запись.

Причина: `notificationContext` использует глобальный `notifications.requestSerial` для operation-derived incidents, а serial повышается при **любой** принятой попытке. Новая отменённая runtime-команда не создаёт новый startup incident. Pending/null candidate уже не сбрасывает announced, однако global serial всё равно обходит дедупликацию.

В `62aa971` ключ привязан к породившему проблему effect-scope, сохраняемому через no-effect попытки. Unrelated runtime попытки не создают новый startup incident. Проверены также показанные partial incidents после retry cancel: исправление P2-N01 не добавило повторных уведомлений.

## Свежая проверка

Все команды запускались в архивном source snapshot:

- `node --test tests/*.test.cjs` — **197/197 PASS**, skipped/fail=0. Лог `/tmp/nevrotick-notifications-review-62aa971-tests.log`.
- `node build.cjs` — exit 0, deterministic standalone **822 403 bytes**, SHA совпал с назначенным выше.
- `node tests/render-fixtures.cjs /tmp/nevrotick-notifications-review-62aa971-views.json` + `python3 tests/check_structure.py ...` после сборки — **1 247 documents, 0 structural errors**, CSP соответствует скрипту, external dependencies=0, original AVIF preserved.
- `node /workspace/nevrotick-ui-audit/notifications-work/final-semantic-probe.cjs /tmp/nevrotick-notifications-review-62aa971 /workspace/nevrotick-ui-audit/notifications-work/final-semantic-probe-62aa971.json` — независимые **1 440 matrix**, **120 dedup/readiness**, **32 activation**, **32 operation overlay**, **22 cancel-scope**, **33 status/ARIA** cases. Все assertions PASS, **findings=[]**. JSON: `final-semantic-probe-62aa971.json`. На предыдущем source этот же независимый probe фиксировал 14 воспроизведений двух failure families; финальная проверка не меняла expected result под реализацию.
- Дополнительная independent inline Node-проба — **8/8 PASS**: runtime Start после обоих partial outcomes сохраняет unresolved issue и дедупликацию, generic current-no-synchronizer copy не утверждает старый Stopped; разрешённые явные подтверждённые handovers к Windows/NevroTick очищают старую проблему без votes/offset/authentication refresh; important clock incident остаётся приоритетнее startup warning. JSON: `final-issue-retention-62aa971.json`.
- В рабочем prototype `git diff --name-only 62aa971 -- src` — пусто; рабочий designated standalone SHA совпал с независимо построенным snapshot. SHA source: model `6e56be98f88a1ba5d034217f3ebb2bd9bf833110b16c2a193736e97f1953f5bf`; views `181bb5ff1635e44128fbdc3220bdafa7bac8dcd12699bdd0a66481fe17cc949c`; app `911701dd27aa25e95a1758a41dfc6de3f203a72a1a9b45740b4921cf77cb1217`; styles `9a6f51b0e46e60b67a34ca41f575a33eef71c39b41b398a99307d5d4cdb81312`.
- Bounded source search for browser/native notification, ServiceWorker/Push, network calls and Windows-command invocation — совпадений нет; существующие безопасные external GitHub links и local download preview не являются такой доставкой.

Browser screenshots/layout/focus и actual delivery не проверялись этим агентом. Source/reducer evidence не подменяет независимый Chromium QA и будущую Windows-квалификацию. Этот вердикт относится к указанному source и exact standalone SHA; новые browser test-only коммиты без src drift его scope не расширяют и не отменяют. Browser и package owners принимают свои финальные артефакты отдельно.
