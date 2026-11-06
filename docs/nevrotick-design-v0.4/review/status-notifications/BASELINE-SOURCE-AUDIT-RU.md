# Повторный аудит статусов: модель и представления

5 ноября 2026 года. **Read-only source/model/view audit**, не браузерная и не нативная проверка. Проверен локальный immutable package commit `ed523ef5aff4e6d04619419398dfaba7c60f6005`, source `c6e08010ee98e354ef6d01342449e057a06eba1a`, дерево `/workspace/nevrotick-ui-audit/ownership-work/NevroTick-UI-v0.4`. Product source и канонический `/workspace/NevroTick` не менялись.

Основания: canonical `docs/windows-time-features.md` F-03/F-04/F-05/F-07, §3.1–3.4, §6.8–6.9 и AC-02/03/08/17/18; архитектура §§6/9/10/11/16 и CHECK-14/22/25. Факт Running, аутентификация пакета, пригодность/свежесть оценки, разрешение менять часы и подтверждённая синхронизация — отдельные факты. Текущий запрос владельца добавляет уведомления в HTML-макет: прежняя отсрочка rich toast в документации не запрещает этот новый дизайн; она не становится доказательством native activation/delivery.

## Итог

Основное разделение статусов сохраняется: 30 исходных сцен, 300 исходных поверхностей, 599 допустимых сценариев команд/ответов и 9 600 дополнительных render-проверок прошли независимые source-пробы. Пробы не запускали встроенные unit suite и не считались визуальной проверкой. Найдены **два P2-пункта для рассмотрения координатором**; первый — подтверждённое противоречие текущего владельца, второй — слишком широкая потеря подтверждённых Windows-наблюдений при потерянном ответе команды NevroTick-наблюдателя. Дополнительный P3 касается терминологии observe-only.

Доказательства: `status-audit-probe.cjs` и `status-audit-probe.json` в этом каталоге. Неизменяемые raw scenario/source/recovery факты проверены отдельно от представления.

## Находки

### P2-S01 — неизвестный результат изменения часов одновременно показывает подтверждённого владельца NevroTick

**Точное воспроизведение:** открыть сцену `outcome-unknown`; Power user → Settings → Time policy, затем Diagnostics → Technical context & retained history; открыть Export preview.

В `src/model.js:151` сцене задаются prior-only authentication, withheld estimate и historical=true, но унаследованное `scenario.ownership` остаётся ровно `NevroTick`. `timeInitial` в `src/model.js:317–320` правильно даёт `ownerKnown=false`; `serviceControl.ownershipKnown` тоже false. Тем не менее `Settings → Time policy`, Diagnostics technical context (`src/views.js:234`) и строка export `Clock ownership: NevroTick` используют сырое `s.ownership` без квалификации. Другие строки того же экспорта и Background service показывают `Time manager: Unknown · reconcile current ownership`.

**Почему важно:** пользователь получает две несовместимые текущие истины, и уведомление, собранное из `scenario.ownership`, может повторить ложное подтверждение.

**Минимальное исправление:** в этой исходной сцене явно квалифицировать владельца как неподтверждённого/последнего известного, либо использовать общий presentation helper для всех current-owner полей. Не менять старое наблюдение на свежий подтверждённый факт. Зафиксировать regression для policy/Diagnostics/export и сохранять блокировку эффектов.

**Доказательство source-пробы:** `{rawOwner:"NevroTick", knownOwner:"Unknown · reconcile current ownership"}`. Это не означает подтверждённый native defect; это ошибка текста fixture view.

### P2-S02 — неизвестный ответ команды наблюдающего NevroTick обнуляет отдельное подтверждение Windows Time

**Точное воспроизведение:** Healthy → Settings → Background service → Use Windows Time → confirmed; Enable and start → Deliver sample service response: unknown. Аналогично возможны Stop/Restart уже запущенного NevroTick-наблюдателя.

`src/model.js:684` безусловно присваивает `timeManagement.owner='unknown'`, `ownerKnown=false`, `reconcileRequired=true` для неизвестного исхода **любой** runtime-команды. При этом `timeManagement.windows` побитово сохраняется как Running/Manual-trigger, а план команды обещает не менять W32Time и не передавать управление. Presentation начинает сообщать `Time management is unconfirmed / Clock cleanup and current ownership must be reconciled`, как будто запрашивалась передача управления.

**Оценка:** P2 для согласованности осей состояния/излишне широкого предупреждения, а не ложный успех. При неизвестном результате настоящей ownership-changing операции неизвестный владелец правилен. Здесь операция по контракту относится только к NevroTick observer, поэтому неопределённость следует ограничить затронутыми фактами либо явно назвать Windows прежним отдельно подтверждённым владельцем, а не изображать проверенный новый transfer.

**Минимальное исправление:** отдельная ветка для ранее подтверждённого Windows owner при runtime-only операции: NevroTick runtime unknown; startup unknown только если Enable-and-start мог его менять; факт Windows-owner/Windows synchronization unverified сохранить с честной областью/возрастом наблюдения. Повтор NevroTick-команды по-прежнему блокировать до сверки. Не объявлять Windows synchronized. Проверить также Background service summary/service feedback/notification copy, чтобы диагностическая причина не называлась «освобождение часов», которого эта команда не запрашивала.

### P3-S03 — observe-only смешивает «кто наблюдает» и «кто имеет clock authority»

Исходная сцена `observe` использует `Clock ownership: NevroTick · observation without writes`, тогда как новая строка `Time manager` даёт Unknown. Clock-control disable и отсутствие неявного W32Time возврата правильно объяснены. Лучше назвать первую величину `Observation: NevroTick / Clock writes disabled` либо квалифицировать владельца как неподтверждённого; **нельзя** выдумать Windows owner или активную синхронизацию. Это терминологическая неоднозначность, не выявленное разрешение эффекта.

## Все 30 сцен и кандидатная политика уведомлений

«При возникновении» ниже означает новый наблюдённый инцидент/изменение причины, а не показ toast на каждый poll, открытие окна или переключение вкладки. Длительность предупреждения должна следовать реальному опубликованному условию/бюджету; конкретный production timeout этим аудитом не утверждается. Нельзя строить notification classification только по `tone`: amber может означать выбранный режим, нормальную ограниченную паузу или требующее внимания событие.

| Сцена | Главная истина / проверка | Уведомление о проблеме |
| --- | --- | --- |
| healthy | Qualified tracking, 3/4, NTS; не обещает UTC-точность | Нет; восстановление — только парное с ранее показанным инцидентом |
| server-rate-limited | Один RATE, остальные три дают agreement; measurement age не обновляется RATE | Обычно нет: инцидент отдельного источника не равен потере синхронизации |
| server-access-refused | DENY отдельно от возраста измерения; current agreement не выполнено, старая модель ещё пригодна | Одно предупреждение с Review sources при новой потере пригодной группы/необходимом действии |
| acquiring | Fresh agreement пока нет; текущий offset отсутствует | Не при штатном запуске; только отдельно опубликованный persistent acquisition failure |
| holdover | Старая модель ещё в бюджете, нет нового agreement | Одно предупреждение при подтверждённой деградации, затем escalation только при expiry |
| expired | Нет пригодной оценки; новые коррекции удержаны | Важное уведомление, открыть Diagnostics |
| offline | Нет ответов; причина не установлена; прежняя модель пока пригодна | Предупреждение о недоступных источниках, без неподтверждённого «сломался firewall» |
| conflict | Аутентификация прошла, majority нет; нет current offset/correction | Важное уведомление, Compare sources; не заявлять «атакован сервер» |
| stale | UI status channel потерян; службы могли продолжить работу | Предупреждение «Current status unavailable», без утверждения остановки |
| service-stopped | Остановка подтверждена, нет current estimate | Если неожиданная остановка — важно; явный Stop/Disable пользователем не должен сам порождать тревогу «сбой» |
| blocked | Измерения пригодны, но clock authority конфликтует | Важно; открыть Background service/policy, без кнопки автоматического takeover |
| resume | Старый mapping не текущий, новая проверка | Не при обычном resume; отдельное длительное failure condition — другой инцидент |
| observe | Измерения есть, writes отключены выбранным режимом | Нет: выбор пользователя не сбой; P3 owner terminology выше |
| ntp | Явно выбранный unauthenticated NTP | Нет повторных тревог за осознанный профиль; доверие остаётся явно видно в окне |
| slewing | Идёт finite correction; acknowledged delivery отдельно | Нет; не обещать выдуманное ETA |
| leap-unknown | Tracking пригоден вне защищённых границ; будущее календаря неизвестно | Нет аварии только из-за неизвестного будущего календаря |
| leap-guard | Краткая защищённая пауза/повторная проверка | Не каждый штатный guard; отдельная непригодность/исчерпанный бюджет — уведомляемая проблема |
| roughtime | Подписи/примерная дата проверяются, NTS пока нет | Нет штатного progress-toast |
| roughtime-date-ready | Signed date handed to NTS; normal votes/clock authority ещё нет | Нет success-toast «время синхронизировано» |
| roughtime-recovered | Tracking NTS квалифицирован; signed-date ancestry сохранена | Парное resolved, только если был уведомлён исходный сбой; не отдельный вечный success spam |
| roughtime-unavailable | Нет usable date; причина не установлена, NTS ждёт | Предупреждение при новом блокирующем восстановление инциденте, открыть Date recovery |
| roughtime-rejected | Непроверенная подпись отклонена; clock unchanged | Предупреждение о rejected evidence, без обвинения сервера; отдельный source отказ не обязан означать глобальный failure |
| reserve | Future unauthenticated provisional-date path, NTS не готов | Только явно помеченный sample будущего; не объявлять доступным v1 fallback |
| recovered | Future plain-NTP ancestry, текущий NTS может быть authenticated | Не стирать unauthenticated ancestry; resolved только для ранее уведомлённого инцидента |
| signed-conflict | Подписанные usable интервалы противоречат; blocked date basis | Важно, Date recovery evidence; без автоматического сброса lockout/перехода NTP |
| correction | Текущая qualified proposal требует single-use review | Action-needed, открыть review; toast не применяет часы/не повышает полномочия |
| correction-expired | Предложение/его evidence истекло; текущей approval нет | Обычно не повторять истёкший prompt; удалить/обновить старое приглашение, при необходимости новое informative состояние |
| outcome-unknown | Эффект часов неизвестен; withheld agreement/current offset | Важно, Review result/Diagnose, никаких Retry correction; исправить P2-S01 owner label |
| all-rate-limited | Все checks delayed; старая модель пригодна; контрольные ответы не measurements | Одно degraded warning; expiry — отдельная escalation без обхода backoff |
| debug-partial | Clock tracking здоров, частична только временная диагностика | Обычно inline результат инициированной операции; необязательная diagnostic notification, не clock failure |

## Наложенные состояния команд и жизненного цикла

| Группа | Проверенный инвариант | Notification consequence |
| --- | --- | --- |
| Startup-only pending/confirmed/unknown/cancel/rejected | Runtime, owner, raw evidence и DEBUG не обнуляются; Unknown startup не fake Off | Не тревожить о потере синхронизации. Failed/unknown startup — отдельное сообщение конфигурации, не Time sync failure |
| Stop/Restart pending | Нет объявленного подтверждённого completion, не создаются измерения | Не посылать failure только за ожидание ответа; отдельный наблюдённый неизвестный/failed результат может требовать действия |
| Stop confirmed | При Nv owner — пауза; при Windows owner — только Nv observation остановлено | Suppress planned pause incident. Windows остаётся manager, не синхронизация-off |
| Start/Restart confirmed | Running не означает новый owner или synchronized; votes=0/current offset absent | Не посылать recovered только по Running |
| Disable confirmed | Both stopped+disabled+restart inhibition+qualified cleanup | Planned disable не ошибка; пользователю заранее объяснены последствия. Windows owner при отключении observer сохраняется |
| cleanup-unconfirmed | Частичный исход, full disable не подтверждён, второй writer не запускается | Важное сообщение, Diagnose/reconcile; не «NevroTick disabled successfully» |
| Windows unavailable after release | Nv stopped/disabled, no active synchronizer, без blind rollback | Важное сообщение о частичном переходе/отсутствии active synchronizer |
| Windows handover success | Nv evidence history, owner Windows, W32Time running, Windows sync unverified | Подтверждение результата без «синхронизировано». Ни stale Nv NTS, ни old votes не попадают в toast |
| Reverse success | W32Time stopped/disabled+settings retained, new Nv generation, current agreement not ready | Не успех синхронизации; пользователь видит только состоявшуюся смену manager |
| Reverse product-unavailable | W32Time остановлен, Nv не запущен, no active synchronizer | Важное уведомление, next diagnostic action, не безусловный Retry |
| Lost ACK ownership transaction | Owner/runtime/startup uncertain, сериализация и reconciliation | Важно, не повторять действие вслепую. Отличать от P2-S02 observer runtime-only |
| Cancel/UAC cancel/rejected before effect | Прежние runtime/owner/raw facts сохраняются | Не посылать «служба сломана/синхронизация остановлена»; локальный результат инициированной команды достаточен |

## Ограничения и требования к следующему слою

Это source/render evidence. Layout, keyboard/Narrator, lock-screen appearance, системная блокировка уведомлений, Focus/Do Not Disturb, реальная доставка, срок и activation Windows не проверены этим агентом. Нет полномочий создавать системные уведомления из настоящей службы или обещать их при остановленных UI/tray/agent. Нужны отдельные observer-availability и Windows delivery facts, а не выдуманный always-on watcher. Пользовательские имена серверов/ключи/endpoint/точный clock offset не должны по умолчанию выходить на lock screen. Нажатие на notification должно только открывать свежий status/review; устаревший action target не должен повторять неизвестную команду или автоматически одобрять correction.
