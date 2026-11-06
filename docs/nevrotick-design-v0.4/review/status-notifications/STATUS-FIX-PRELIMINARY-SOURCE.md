# Source review исправлений статусов `cd90ed0`

5 ноября 2026 года. Проверен **точный immutable source** `cd90ed04dbd26e8730204efd123fd22c80d01db6` относительно `ed523ef5aff4e6d04619419398dfaba7c60f6005`. Для исключения текущих изменений implementer использован отдельный `git archive` snapshot `/tmp/nevrotick-status-review-cd90ed0`. Production-файлы и mutable HEAD не менялись. Generated HTML из commit не выдаётся за актуальную сборку; standalone hash и browser acceptance здесь не проверялись.

## Вердикт

**Основные модельные исправления верны, но два оставшихся представления нужно поправить до финальной приёмки.** P2-S01 закрыт; P2-S02 закрыт в model/reducer, но не во всём feedback. Observe-only и главные status/tray actions стали яснее. Запрет blind retry, сохранение published facts и отсутствие нового clock authority не ослаблены.

## Подтверждённые результаты

| Требование | Независимая проверка | Итог |
| --- | --- | --- |
| P2-S01 outcome-unknown | `presentState` квалифицирует current owner; policy/Diagnostics/export не выдают подтверждённого NevroTick; raw scenario не переписывается | PASS |
| Observe-only | Presentation явно Observation only/no clock-writing authority/current time manager unconfirmed; Windows owner не предполагается | PASS |
| P2-S02 model | Windows handover → observer Start/Stop/Restart unknown: owner Windows known, Windows observations неизменны; Nv runtime unknown; потерянный Enable-and-start может сделать только затронутый Nv startup unknown | PASS |
| Неизвестная служба не даёт повторную mutation | `servicePermission` Start/Stop/Restart и `lifecyclePermission` все изменения запрещены для каждого observer unknown случая | PASS |
| Service-stopped primary action | Simple/Power/tray ведут к Time management; устаревший next repair-route заменён контекстным Start/manager/Diagnostics | PASS |
| Unknown/pending status action | Disabled Check sources + Diagnose вместо enabled Sync now; confirmed stopped не выводится из unknown | PASS |
| Observe/blocked/outcome-unknown | Check sources поясняет measurement-only; не даёт authority/повтор clock effect | PASS |
| Startup-only | Не блокирует здоровый Sync now и не стирает runtime/owner/DEBUG/историю | PASS |
| Общие 30 сцен и переходы | 300 base surfaces + 599 operation cases/9 600 view renders; raw facts неизменны, runtime/transfer не создаёт current votes/offset | PASS |

## Открытые находки на этом immutable кандидате

### P2-S02R — service feedback всё ещё объявляет Windows ownership неизвестным

`src/views.js`, `serviceFeedback`, ветка `unknown` (около строки 288); применяется к Background service и Service diagnosis.

Repro: healthy → Use Windows Time → confirmed → Enable and start → runtime response unknown. Аналогично Stop/Restart наблюдателя.

Факты правильно говорят:

> Clock ownership: Windows Time · independently confirmed; NevroTick runtime unknown

Однако соседняя карточка результата говорит:

> The acknowledgement was lost. The command may have completed; **service state and clock ownership are unknown**. Diagnose and reconcile before another command.

Это остаток исходного противоречия. Независимая проба воспроизводит его для всех **3** observer-команд. Три новые repository tests его не проверяют.

Минимально: Windows-owner-specific unknown feedback, ограничивающий неизвестность NevroTick runtime/его затронутым startup. Сохранить необходимость диагностики и запрет blind retry. Generic lost-ownership wording оставить для настоящей неизвестной ownership-changing операции или когда owner действительно unknown.

### P2-A01 — tray Sync now / Check sources ссылается на отсутствующее описание

`src/views.js:26,31`, `syncButton(state, tray=true)`; вызывается из `sampleTray` около строки 393.

Repro: Power → Settings → Your preferences → Keep NevroTick in the tray → Preview tray menu. Healthy/Windows-owned/default branches вставляют `aria-describedby="sync-response"`, но на странице Settings отсутствует `id="sync-response"` (он живёт на Status). В observe/blocked/outcome-unknown отдельная tray note корректна; stopped/unknown ветки не имеют этого target. Независимая проверка реального полного render для всех 30 исходных сцен выявила отсутствующий ARIA target в **25** сценах.

Минимально: в tray не ссылаться на main Status feedback либо создать локальный пригодный target; measurement-only tray note сохранить. Добавить structural fixture именно для tray, открытого с Settings/Your preferences, а не только tray modal поверх Status. `tests/check_structure.py:19–22` уже считает такие отсутствующие цели ошибкой.

## Свежие команды и доказательства

- В архивном snapshot `node --test tests/status-notifications.test.cjs` — exit 0, **3/3 PASS**.
- В архивном snapshot `node --test tests/*.test.cjs` — exit 0, **178/178 PASS**, без skipped/fail. Лог `/tmp/nevrotick-status-review-cd90ed0-tests.log`.
- `status-fix-generic-probe.cjs` — exit 0, **30 scenes, 300 base surfaces, 599 operation cases, 9 600 additional view renders**; старые модельные P2 не воспроизвелись. Результат `status-fix-generic-probe.json`.
- Независимая delta-проба — **35 cases** (3 Windows observer commands + 2 owner-qualifier cases + 30 tray-from-settings renders). Model fixes проходят; 3 contradictions feedback и 25 missing ARIA targets сохранены в `status-fix-delta-probe.json`.
- Snapshot SHA-256: `src/model.js` = `87cbf33ef3bb863a6f7df6e5f496a60fed2f1d9ec298588c56acdafb9c7494c5`; `src/views.js` = `cd60d8a0be3c9ecedb22427e8c922a420fda6e66c4897dea741d37870de917b3`; `tests/status-notifications.test.cjs` = `5341dcf6ea143856c41e948e3983f5da2685d8de491208da451084101d140d5d`.

Нужен ограниченный повтор этих двух view cases после исправления на финальном source candidate. Notification classifier/episode/readiness/activation новый слой ещё не входил в этот commit и этим review не принимается. Browser/layout/native Windows qualification остаются отдельными проверками.
