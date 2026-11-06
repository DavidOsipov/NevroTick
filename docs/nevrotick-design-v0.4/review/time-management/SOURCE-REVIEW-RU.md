# Независимый source/spec review: startup и владение временем

**PASS в пределах HTML-прототипа.** Материальных нерешённых замечаний в проверенном source-кандидате нет. Соответствие спецификации и инженерные инварианты подтверждены чтением изменённого кода и свежими исполняемыми проверками. Нативная передача управления Windows этим результатом не квалифицирована.

Финальный проверенный source commit: `c6e08010ee98e354ef6d01342449e057a06eba1a`. Полный review выполнен для его родителя `2435ea21417964803cfae471d56cd0e4a48f200c` относительно базы `351c1f240e4ef2e054f6d54739d27d9761d75d2a`; затем независимо проверена только финальная delta. Она меняет условный заголовок/подпись журнала в `src/views.js` и соответствующие два test-файла. Model, reducer, ownership/startup semantics и содержимое event entries неизменны; исходный полный review переносится на финальный кандидат с учётом проверенного ниже P2 fix. Основание требований: `SPEC-RU.md`, `windows-time-research.md` и уточнение координатора о разграничении сохранённой конфигурации NevroTick и действующей политики W32Time. Source/test-файлы совпадали с финальным HEAD; производственный код и тесты проверяющий не менял.

Финальный назначенный standalone проверен по SHA-256: `5970857e19b26f467c67e35a84f5b22dac91a1310939b445418afcd3e28328bb`. Сгенерированные изменения HTML в рабочем дереве не выдаются за чистый commit; source-вердикт относится к указанному immutable source commit, artifact-идентичность — к указанному хешу. Хеш исходного `source-review.patch`, покрывающего родительский full-review candidate: `5a6493826aa4585b8461944d8db7cf4876f77b83b28f6408cc46ec5fe1c666a7`; финальная delta дополнительно проверена непосредственно через git diff между двумя указанными commit.

| Требование / риск | Свежие доказательства | Результат |
| --- | --- | --- |
| Startup-only не меняет runtime, clock trust, authentication age или DEBUG | `runtimePending` отделяет configuration kind; reducer сохраняет service/scenario/log факты; независимые pending/confirmed/unknown/refusal пробы на healthy, Roughtime-recovered и DEBUG-partial | PASS |
| Off не останавливает пару; On не запускает остановленную пару | Сфокусированные lifecycle tests и чтение startup result branch | PASS |
| Полное отключение требует stopped + disabled + restart inhibition + подтверждённого cleanup | `fullDisableConfirmed` выводит результат из подтверждённых фактов и исключает pending/unknown; Disable и partial ветки проверены | PASS |
| Неизвестный результат повторного включения не оставляет ложное fully disabled / disabled startup | Disable → startup-on unknown и Disable → Enable and start unknown проверены отдельно; отказ до исполнения восстанавливает прежние факты | PASS |
| Start и автозапуск при Windows owner не перехватывают часы | Start/Stop/Restart, startup-on и Disable наблюдающего NevroTick сохраняют Windows owner и Windows configuration; Start даёт manual и наблюдение | PASS |
| Handover не выводит cleanup/владельца из Running/Stopped | Отдельные cleanup, ownerKnown, Windows service и synchronization facts; cleanup-unconfirmed не запускает Windows; Windows-unavailable даёт none без повторного запуска NevroTick | PASS |
| Reverse — явная транзакция с Stop + Disable W32Time | План подтверждения и reducer явно останавливают/disable Windows Time, сохраняют restoreMode/policy, создают новую product generation без готовой синхронизации | PASS |
| GPO сам по себе не запрещает передачу | managed-allowed завершается; конкретные policy/dependency/ownership blockers блокируют подтверждение без изменений runtime/startup/owner | PASS |
| Ограничения действуют в принимающем reducer, включая stale confirmation | Повторная проверка permission/preflight в confirm; независимые пробы смены Settings section, закрытия диалога, потери ownerKnown и конкурирующих запросов; UI-only обхода не выявлено | PASS |
| Windows owner согласован во всех views/export | Actual rendered Status, Settings, Sources, Statistics, Diagnostics, Recovery, Simple, dialogs и export; нет ложного глобального outage при stopped NevroTick; runtime pending квалифицирован отдельно | PASS |
| Nv NTS profile/quorum/poll preference не представляются политикой W32Time | Actual rendered inactive/stored labels в Sources/Time policy/Statistics, scope hint у интервала и других previews; export содержит Stored NevroTick profile/source policy | PASS |
| Raw NTS/Roughtime/history/budgets не сбрасываются и не освежаются | Raw scenario deep-equality после Disable/forward/reverse; historical evidence, age annotations, signature/interval сохранены; current votes/offset отсутствуют, recovery stage недоступна, correction proposal expired | PASS |
| Исторические Status events не выглядят свежим current claim | Финальная delta локально маркирует журнал как Retained events · before service operation и объясняет scope age/wording; 17 состояний подтвердили неизменные raw/rendered entries, исходный startup-only scope и сохранение history после последующего startup | PASS |
| Нет угаданного списка зависимостей, универсального Automatic или native side effects | Source/standalone inspection: sample Manual/trigger-start, сохранённая Windows policy, отсутствие фиксированных prerequisite identities и network/native command invocation | PASS |

В ходе подготовки были найдены и переданы координатору ошибки представления Windows-owner как отсутствия общей синхронизации, ложного fully-disabled после неизвестного изменения startup/Enable-and-start, активной маркировки Nv policy при Windows owner и текущей маркировки старого runtime во время pending. Повторные проверки на указанном immutable кандидате подтвердили их исправление.

Дополнительное **P2 замечание browser QA закрыто** в `c6e0801`: после handover сохранённая Status-запись «Time sync active / 2 s old / Your clock is being maintained…» ранее не имела локального pre-operation контекста. Теперь непосредственно перед журналом указаны retained scope и привязка ages/status wording к предыдущему snapshot. Ветка включается только при `serviceControl.presentation`; первоначальный startup-only оставляет Recent events, а startup после runtime сохраняет ранее установленный historical scope. Raw titles/details/ages/sequence не переписаны. Локальная подпись достаточна для квалификации исторического сообщения; нерешённого current-state claim не осталось.

Исходные проверки полного review на `2435ea2`, переносимые по неизменённой семантике:

- `node --test tests/startup-ownership.test.cjs` — exit 0, **19/19 PASS**.
- Независимый `node` heredoc с `node:assert/strict` — exit 0, **6 групп PASS**: configuration-only trust/DEBUG; неизвестные результаты повторного включения; receiving reducer/stale-confirm/preflight; Windows runtime/pending/cross-view; сохранённая Nv configuration против W32Time policy; partial/reverse/history.
- `git diff --name-only HEAD -- src tests/startup-ownership.test.cjs tests/startup-ownership.browser.cjs` — пусто.
- `git rev-parse HEAD`, `sha256sum` source/patch/standalone — идентичность зафиксирована выше.
- `rg` по source и обоим HTML на network/native invocation terms — совпадений нет (ожидаемый exit 1).

Свежие ограниченные проверки финальной delta на `c6e0801`:

- `git diff 2435ea21417964803cfae471d56cd0e4a48f200c c6e08010ee98e354ef6d01342449e057a06eba1a -- src tests` — просмотрена полная delta: один render expression и два test-файла; model/reducer не менялись.
- `node --test --test-name-pattern='retained event journals' tests/startup-ownership.test.cjs` — exit 0, **1/1 PASS**; полный suite повторно не запускался.
- Независимый `node` heredoc с `node:assert/strict` — exit 0, **17 состояний PASS**: baseline; startup pending/success/unknown/refusal; runtime pending/restart; Windows handover/Disable/partial/unknown/refusal; startup pending/success/unknown после runtime/Windows. Проверены локальная подпись до event entries, исходный scope без runtime, сохранение history после runtime, byte-identical rendered event list и неизменный raw events JSON.
- `git diff --name-only HEAD -- src tests` — пусто; `git rev-parse HEAD` и SHA-256 финального standalone соответствуют идентификаторам выше.

Полный baseline suite, Chromium layout/focus/screenshots, CSP/AVIF/package validation не дублировались: их выполняют другие назначенные проверки. Этот source-вердикт не подменяет независимый browser QA. Linux/HTML-модель не доказывает реальный порядок SCM/clock операций, Windows readiness, действующую GPO, cleanup readback или реальную синхронизацию; таких действий в проверяемой реализации нет.
