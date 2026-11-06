# Независимая браузерная проверка исправлений статусов

**Статусная матрица и исправленные маршруты проходят; окончательное принятие условно до двух известных небольших delta-исправлений.** Проверен замороженный source `cd90ed0` (Qualify clock ownership and service status actions), отдельно от продолжающейся разработки уведомлений. Mutable worktree браузер не открывал.

5 ноября 2026. Независимая копия создана через `git archive cd90ed0` в `/tmp/nevrotick-status-fixed.z6A7mu`; сборка выполнена только там. Нет изменений product source, canonical NevroTick или реальной Windows. Тестовая навигация — local HTTP в Chromium.

## Идентичность и исполнение

- Standalone после сборки frozen source: SHA-256 `bf1adb4418be442d5b44bb9476f9d7532b9a3d43d25eaa11a05953760835bba7`.
- `index.html`: SHA-256 `837677ac278a8c9a01297387dfbe7415d78f8d4052769bf80bfe62b66794b7ef`; сам файл неизменен, external JS читается из frozen snapshot. Хеши model/views/app/styles сохранены в evidence.
- `node build.cjs` — exit 0, 30 сцен.
- `node --test tests/status-notifications.test.cjs` — **3/3 pass**, без пропусков. Это сфокусированные новые status tests, не заявление о повторном полном прогоне всех 178 unit tests.
- Независимый `status-fix-audit/review.cjs` — exit 0; **240** DOM-проверок всех 30 Status-сцен в обоих режимах, обоих HTML, 360/1280 px.
- Всего **296** свежих readbacks и **36** дополнительных workflow-групп. **148** точных проверок текстового соответствия двух сборок.
- **0** ошибок JavaScript, неожиданных внешних запросов и переполнений документа. Сохранены 28 выбранных новых снимков вместо повторного захвата всей исходной матрицы.

## Исправления в фактическом UI

| Шаг | Результат | Свежие доказательства |
| --- | --- | --- |
| 1. Confirmed stopped | В Simple и Power вместо невозможного Sync now есть Time management. Главный следующий шаг предлагает Background service/Start; package repair больше не навязывается. Кнопка ведёт в раздел с доступным Start. | [Power 360](status-fix-audit/screenshots/360-service-stopped-power.png), readbacks/workflow `stopped-route-start`. |
| 2. Stale, runtime Pending/Unknown | Проверка источников явно недоступна; Diagnose доступна и ведёт в Diagnostics с сохранённым локальным фокусом. Нет зелёного текущего NTS/голосов после runtime uncertainty. | [Pending Simple 360](status-fix-audit/screenshots/360-stop-pending-simple.png), workflow `stop-pending/unknown-diagnose-route`. |
| 3. Observe/blocked/clock-outcome-unknown | Название Check sources дополнено пояснением: measurements only, no clock change or repeat. Current owner в unknown/observe квалифицирован в Settings, policy и export. | [Unknown Power 1280](status-fix-audit/screenshots/1280-outcome-unknown-power.png), [Unknown owner facts](status-fix-audit/screenshots/1280-outcome-unknown-owner-facts.png), [Observe owner facts](status-fix-audit/screenshots/1280-observe-owner-facts.png). |
| 4. Windows owner + Nv observer lostACK | Для Start/Stop/Restart current Windows manager сохраняется отдельно от unknown Nv runtime. Simple/Power/Settings/export не превращают его в unknown и не утверждают Windows synchronization. | [Simple 360](status-fix-audit/screenshots/360-windows-observer-unknown-simple.png), девять lifecycle-readbacks на каждый file/width. Остаточная feedback-copy проблема — ниже. |
| 5. Startup-only unknown | Здоровая текущая синхронизация и доступность проверки сохраняются. Конфигурационная неопределённость не превращена в runtime outage. | workflow `startup-unknown-keeps-healthy-checks` для обоих HTML/ширин. |
| 6. Остальные 30 сцен | Нет неожиданных различий текста между сборками или проблем ширины после status changes; исходные смысловые различия healthy/holdover/expired/conflict/recovery сохранены. | 240 status readbacks и исходный отдельный baseline-аудит. |

Все связанные в таблице снимки сохранены и открыты для визуального осмотра. Главные кнопки/подписи помещаются на 360 px. Наличие disabled Check sources дополняется доступной Diagnose, поэтому disabled не оставляет пользователя без следующего шага.

## Известные остаточные требования для final delta

Эти замечания уже переданы координатором implementer; они не объявляются закрытыми этим frozen review.

1. **Windows-owned observer unknown feedback.** Текущие status/owner facts исправлены, но Settings → Service command sample всё ещё говорит: `The acknowledgement was lost. The command may have completed; service state and clock ownership are unknown.` Рядом подтверждён Windows manager. Нужно квалифицировать только NevroTick runtime; отдельно подтверждённый Windows manager не теряется. Точная строка есть в `evidence.json`, labels `windows-observer-*-unknown-facts`.
2. **Tray ARIA reference.** В Settings → Your preferences → включить tray → Preview tray menu, healthy Sync now имеет `aria-describedby="sync-response"`, но `sync-response` на этой странице отсутствует. Независимо воспроизведено в Chromium; `tray-aria.json` фиксирует missing target. Нужна локальная существующая reference или её отсутствие, а не ссылка на отсутствующий Status block.

После этих двух ограниченных правок достаточно соответствующего delta-review и bounded browser retest, если остальные status semantics и source predicates не меняются. Полную матрицу заново повторять оснований нет.

## Материалы и границы

Материалы: [сводка](status-fix-audit/verification-summary.json), `status-fix-audit/evidence.json`, `review.log`, `review.cjs`, `tray-aria.json`, selected `screenshots/`. Исходный доисправительный аудит — `status-audit/REPORT-RU.md`.

Уведомлений в `cd90ed0` ещё нет. Этот pass не подтверждает их delivery, suppression, dedupe, action routing или окончательный новый HTML. Screenshot/DOM-аудит также не устанавливает native Windows ownership, SCM, UAC, Narrator/UIA или фактическую синхронизацию. Нативные операции не выполнялись.
