# NevroTick — сопоставление features и дизайна v0.2

**Дата:** 28 октября 2026 года.  
**Объект:** дизайн NevroTick v0.1 и его отдельная редакция v0.2. V0.1 сохраняется как предыдущая версия.  
**Новый источник:** RWTC-FEATURES-001 revision 1.0, Basic features and v1 delivery plan.  
**Статус результата:** согласование документов, текстов и демонстрационных состояний; не приёмка реального Windows-продукта.

## 1. Авторитет и пределы источников

1. [Architecture RWTC-ARCH-001 revision 1.4](../references/source-docs/windows-time-architecture.md) остаётся авторитетной для trust, полномочий, clock ownership, протоколов, correction/recovery и жизненного цикла.
2. [Features RWTC-FEATURES-001 revision 1.0](../references/source-docs/windows-time-features.md) сохранён с исходным статусом **Proposed v1 product requirements and implementation slices**. Его presentation/default proposals применены в дизайне v0.2; это не превращает все будущие feature gates в исполненные проверки.
3. Пользовательский выбор NevroTick, маскота, широкого бренда и v1 времени сохраняется. Будущий DNS не добавляется в интерфейс v1.
4. Features не принимает TUF, auto-updater, новые dependencies, signing scheme или native toolkit. [Update assessment](../references/source-docs/windows-time-update-assessment.md#1-assessment) также остаётся candidate.
5. Features ссылается на **windows-time-development-hardening.md** и **windows-time-signing-guide.md**. Эти два документа отсутствуют в полученном комплекте; их содержание, gates и утверждения **не проверены**. Ссылки внутри оригинального features сохранены как часть источника. Этот пакет не создаёт заменители и не приписывает им прочитанные требования.

Инвентарь доступного комплекта проверен локально. Наличие исходного документа и совпадение с его текстом не являются свидетельством работающего приложения.

## 2. Как читать статусы

| Статус | Значение |
| --- | --- |
| **Совпадает** | Дизайн уже выражал соответствующий архитектурный контракт. Это не значит, что native-поведение реализовано. |
| **Исправлено в дизайне v0.2** | Уточнены структура, подписи, fixture cases или отображаемые данные. Фактические проверки собранного прототипа учитываются отдельно в QA. |
| **Будущий native gate** | Нужны реальное приложение, поддерживаемая Windows, независимые измерения или platform/accessibility evidence. Статус остаётся planned / not executed. |

Одна feature может одновременно иметь совпадающую основу, исправление презентации и ещё не выполненный native gate.

## 3. Матрица F-01–F-10

| Feature и источник | Совпадает с прежним дизайном | Исправлено / уточнено в дизайне v0.2 | Будущий native gate |
| --- | --- | --- | --- |
| [F-01 — Install and maintain one product](../references/source-docs/windows-time-features.md#f-01--install-and-maintain-one-product) | Один полный MSI; normal-user UI; установка, update, repair и uninstall; явное восстановление применимого владельца часов; managed observation | Поддерживается complete-MSI maintenance, без отдельного updater. Privacy/About и будущий setup должны объяснять time-server и разрешённый DNS/certificate retrieval. Не обещается неизвестный официальный release URL | **AC-01/14/15:** реальная установка, boot without login, владение и rollback, сохранение бюджета, обслуживание без UI, подписи и interrupted restoration. Скопированные файлы не доказывают готовность |
| [F-02 — Synchronize automatically in the background](../references/source-docs/windows-time-features.md#f-02--synchronize-automatically-in-the-background) | NTS по умолчанию; NTP только явным профилем; observe-only; отсутствие silent fallback; закрытие UI не останавливает службы | Profile/headline делают режим видимым раньше чисел. Normal setup не требует polling/frequency tuning; read-only policy explanation сохраняется | **AC-01/02/04/08/19:** фактические NTS/NTP measurements и clock trajectory, zero writes при observe-only startup/restart, bounded polling и продолжение работы без UI |
| [F-03 — Select a principal source and compatible supporters](../references/source-docs/windows-time-features.md#f-03--select-a-principal-source-and-compatible-supporters) | Выбор автоматический; fastest reply не означает лучший источник; source independence и majority/minimum не заменяются количеством имён | **Main / Supporting / Not used / Waiting**, краткая причина в обычной строке. **Draft** отделён от действующего Waiting. Подробности показывают operator group/time scale; helpers имеют отдельную секцию без голоса | **AC-03:** независимые agreeing/wrong/stale/duplicate fixtures, реальное membership/reselection, качественная статистика и доказанная совместимость shipped provider set |
| [F-04 — Show truthful status immediately](../references/source-docs/windows-time-features.md#f-04--show-truthful-status-immediately) | Аутентификация, согласие, неопределённость и service operation концептуально разделены; данные при потере связи сохраняются как исторические | **Profile + Time sync active → activity/remedy → evidence**. Status connection failure больше не является доказательством service failure. Отдельно представлены **Last valid reply / Last qualified estimate / Last applied correction / Status snapshot age** | **AC-05:** authoritative snapshots, настоящие monotonic/generation ages, invalid/stale/incompatible handling, broker acknowledgement и запрет изготовления текущего успеха из старых данных |
| [F-05 — Request a fresh check with Sync now](../references/source-docs/windows-time-features.md#f-05--request-a-fresh-check-with-sync-now) | Запрос работает с применёнными источниками и не даёт нового clock-change authority; large correction отдельно | Уточнены исходы **Checking / Already checking / Waiting for enough agreeing sources / Try again after the current limit / Correction in progress** и blocked/unavailable. Feedback не означает автоматический успех | **AC-06:** реальный разрешённый ordinary-user IPC, storm/coalescing/rate-limit, сохранение budgets и успешный свежий запрос после снятия ограничения |
| [F-06 — Inspect and change sources and profiles safely](../references/source-docs/windows-time-features.md#f-06--inspect-and-change-sources-and-profiles-safely) | Read-only inspection без UAC; draft отдельно от active; review перед конкретным Apply; отмена и conflict сохраняют действующую политику; NTS/NTP не смешиваются | Основные группы **Time sources and trust / Clock correction / Appearance**. Helpers редактируются отдельно. Raw step/rate tuning и single-source mode исключены из basic UI. Observe-only явно не означает handback к W32Time | **AC-07/08:** реальные token/revision checks, атомарная persistence, valid-owner cleanup при переходе в observe-only; при ambiguous ownership никакого blind reset |
| [F-07 — Recover after ordinary interruptions](../references/source-docs/windows-time-features.md#f-07--recover-after-ordinary-interruptions) | Holdover, Expired, Reacquiring, ownership conflict и отсутствие NTP/W32Time fallback уже предусмотрены | Понятные headlines и remedy; unknown connection/age не маскируется reassuring цифрами. Восстановление не описывается как необходимость переустановки при обычном network outage | **AC-02/12/13:** sleep/Modern Standby, network/clock generation changes, late callbacks, реальные engine/broker crashes, ownership-aware cleanup и budgets across restarts |
| [F-08 — Recover from a bad date and review large changes](../references/source-docs/windows-time-features.md#f-08--recover-from-a-bad-date-and-review-large-changes) | Helpers Off; один/два явно; no two-to-one downgrade; hints не пишут Windows time; отдельное одноразовое разрешение большой коррекции и expired proposal | **Certificate-time basis** больше не выводится из protocol=NTS. Source details и Diagnostics сохраняют recovery history после Tracking для одного и двух helpers. Acquisition success отделён от applied correction | **AC-09/10/11:** квалифицированный Windows verifier, date/era/revocation checks, actual cached/strict-NTS precedence, отрицательные соседние случаи и исполнение single-use approval ровно для свежего предложения |
| [F-09 — Explain failures and export useful diagnostics](../references/source-docs/windows-time-features.md#f-09--explain-failures-and-export-useful-diagnostics) | Локальная ограниченная диагностика, explicit export, redaction по умолчанию, отсутствие автоматической отправки; complete-MSI repair как путь обслуживания | **Current issue → What it affects → Suggested next action**; наблюдение отделено от гипотезы. **Copy summary** и export имеют видимый redacted context, freshness и provenance. About включает privacy; официальный release URL не выдумывается | **AC-16:** реальные bounded logs/export, секреты исключены из native paths, пользовательский filename не даёт privileged file write; log flooding/full sink не препятствуют cleanup |
| [F-10 — Provide a small accessible native UI and optional tray](../references/source-docs/windows-time-features.md#f-10--provide-a-small-accessible-native-ui-and-optional-tray) | Компактное окно, English first, progressive disclosure, keyboard/focus intent; optional tray и обычные пользовательские preferences | **Use Windows setting** по умолчанию. Tray Off; отдельный Off-by-default **Start the tray observer when I sign in**, доступный только при tray On. Это observer, не главное окно и не watchdog. Немедленный warning при отсутствии наблюдателя не обещается | **AC-17/18/20:** native UIA/Narrator, contrast/scaling/DPI, отсутствие UI process при default close, настоящий per-user sign-in path и отдельные resource/wakeup measurements |

Архитектурные связи каждого native gate остаются в [features §8](../references/source-docs/windows-time-features.md#8-acceptance-scenarios-and-evidence) и [architecture §20](../references/source-docs/windows-time-architecture.md#20-verification-strategy-and-acceptance-checks). Матрица не означает, что AC-01–AC-20 были выполнены.

## 4. Почему изменена презентация

### 4.1 Исправления смысла состояния

**Connection unknown.** В v0.1 stale fixture имел headline Service unavailable, а footer/Diagnostics повторяли утверждение об отказе. Одна недоступная status connection этого не доказывает. V0.2 различает недоступность статуса, неизвестное здоровье службы и отдельно подтверждённый отказ. Источник: [features F-04](../references/source-docs/windows-time-features.md#f-04--show-truthful-status-immediately), [§4](../references/source-docs/windows-time-features.md#4-state-and-freshness-presentation).

**Четыре возраста.** Ранее общий age менял подпись между accepted observation и snapshot, а broker-acknowledged correction не имел отдельного представления. V0.2 разделяет значения и не выдаёт request за correction. На главном экране остаётся компактная summary; источник reply и подробные evidence доступны в соответствующей поверхности. Никакого универсального Last synced. Источник: [features §4](../references/source-docs/windows-time-features.md#4-state-and-freshness-presentation), AC-05.

**Recovery provenance.** У v0.1 NTS-source details подставлял trusted local estimate по наличию NTS, а успешный Tracking после helpers отдельно не представлялся. Теперь текущая packet authentication, certificate-time basis и сохранённая history различаются. Это предотвращает ложное улучшение происхождения подсказки после нового handshake. Источник: [features §3.2](../references/source-docs/windows-time-features.md#32-sources), [F-08](../references/source-docs/windows-time-features.md#f-08--recover-from-a-bad-date-and-review-large-changes).

Эти изменения существенны для правдивости интерфейса. Они не являются сообщением об эксплуатации или о найденной уязвимости работающего Windows-продукта.

### 4.2 Упрощение пользовательских текстов

| Было в v0.1 | В дизайне v0.2 | Причина |
| --- | --- | --- |
| Tracking / Acquiring как главные пользовательские строки | Time sync active / Checking time sources | Понятное действие без необходимости знать internal enum |
| Крупные offset/uncertainty перед объяснением | Profile/headline и activity/remedy перед evidence | Сначала понятны режим и требуемое действие |
| Principal / Rejected / Pending | Main / Not used / Waiting; Draft отдельно | Причина выбора понятнее, черновик не выдаётся за работающий источник |
| Общий recent-events список в Diagnostics | Current issue → What it affects → Suggested next action | Не требуется выводить причину самостоятельно из log codes |
| Export без Copy summary | Previewable Copy summary и Export diagnostics | Короткий безопасный путь передачи полезного контекста |
| Light по умолчанию | Use Windows setting | Следование поддерживаемым системным preferences |
| Open the window when I sign in, независимо от tray | Start the tray observer when I sign in, только при tray On | Точно соответствует выбранному visibility model |

Источник: [features §§3–5](../references/source-docs/windows-time-features.md#3-screens-and-information-hierarchy), [F-09–F-10](../references/source-docs/windows-time-features.md#f-09--explain-failures-and-export-useful-diagnostics).

## 5. Где выражены изменения

| Область | Файл / место |
| --- | --- |
| Headline, порядок чтения, четыре возраста, footer | [prototype/src/NevroTick.jsx](../prototype/src/NevroTick.jsx): App, statusView и service footer; [prototype/src/model.js](../prototype/src/model.js): status fixtures |
| Main/Supporting/Not used/Waiting, reason, details и helpers | [prototype/src/NevroTick.jsx](../prototype/src/NevroTick.jsx): sourcesView / source-detail; [prototype/src/model.js](../prototype/src/model.js): source fixtures и validation |
| Defaults, tray/sign-in, политика | [prototype/src/NevroTick.jsx](../prototype/src/NevroTick.jsx): App preferences / settingsView; [prototype/src/styles.css](../prototype/src/styles.css): system/contrast presentation |
| Cause/impact/action, Copy summary, export, About | [prototype/src/NevroTick.jsx](../prototype/src/NevroTick.jsx): diagnostics/export/about surfaces и локальные обработчики |
| Recovery history и сценарии обратной связи | [prototype/src/model.js](../prototype/src/model.js): fixtures / scenes; [prototype/src/NevroTick.jsx](../prototype/src/NevroTick.jsx): recovery, correction и status presentation |
| Поля и поведение | [02-ui-spec.md](02-ui-spec.md) |
| Обязательные состояния и review-сценарии | [04-states-and-acceptance.md](04-states-and-acceptance.md) |

Фиксированное число сцен и изображений здесь не задаётся: окончательный каталог формируется по собранному прототипу.

## 6. Что не меняется

- Никаких новых полномочий у UI; read-only inspection и permitted refresh остаются обычными действиями пользователя.
- Никакого смешанного NTS/NTP voting, тихого downgrade, certificate bypass, уменьшения согласия или однопомощникового fallback.
- Никакого произвольного Test server через обычный Refresh до применения адреса.
- Ни raw correction tuning, ни single-source profile, ни service-control/restart/stop/pause, ни handback-to-Windows button не входят в basic UI.
- Поставляемый набор провайдеров, time-scale compatibility, численные correction/holdover limits, verifier behavior и signing остаются открытыми native/release gates.
- Никаких fabricated product-version, production accuracy SLA, вероятности достоверности или установленного publisher trust.
- Нет автообновлятора/TUF и действующей release-ссылки без утверждённого официального адреса.
- Русские тексты документов не означают, что локализация приложения на русский уже принята: начальный UI остаётся English.

Источник: [features §10, defer list](../references/source-docs/windows-time-features.md#10-defer-from-the-basic-release), [§11, decisions](../references/source-docs/windows-time-features.md#11-decisions-still-needed-before-their-dependent-work).

## 7. Учёт проверки и ограничения

Этот review сопоставляет предоставленные локальные документы, UI-код и модельные состояния. Формулировка **исправлено в дизайне v0.2** описывает редакцию дизайна/fixtures, а не runtime acceptance. Отдельный QA-отчёт собранного прототипа должен перечислять только действительно выполненные проверки и обнаруженные ограничения.

Нативные операции здесь не выполняются: нет наблюдённого clock trajectory, real NTS trust establishment, UAC/SCM/IPC qualification, MSI signing/lifecycle evidence, измерений CPU/memory или native accessibility pass. Features прямо маркирует свои AC-кейсы **planned / not executed**; этот статус сохранён.

Будущие документы development-hardening и signing-guide нужно получить и отдельно сверить до утверждений об их требованиях либо выполнении. Их отсутствие не даёт основания обходить уже известные gates архитектуры 1.4.
