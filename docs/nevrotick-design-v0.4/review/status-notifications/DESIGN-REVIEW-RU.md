# Source/semantic review дизайна статусов и notifications

5 ноября 2026 года. Рассмотрены `SPEC-RU.md`, `windows-notifications-research.md` и ранее подготовленный `status-source-audit.md`. Это review требований до реализации; исходный код не менялся.

## Вердикт

**Основное решение подходит для реализации HTML-прототипа.** Сохраняются все значимые границы: local Windows app notifications вместо ненужного cloud WNS; невыдача browser/HTML preview за OS delivery; неизменные authority/trust границы; current status независимо от уведомления; per-user presenter без новой silently-installed службы; безопасное open-only activation; OS suppression без обхода. P2-S01/S02 и observe-only уточнение явно учтены.

Ниже одна материальная неоднозначность политики и три ограниченных уточнения для acceptance. Они не требуют расширять архитектуру или спрашивать владельца о рутинных технических вариантах.

## Уточнения

1. **Отказ до исполнения против неизвестного/частичного эффекта.** Формулировка SPEC «При отказе/неполном результате этих команд важная проблема уже требует уведомления» допускает toast на обычный отказ до исполнения. Research и прежний контракт различают Cancel/UAC-cancel/rejected-before-effect (inline outcome, факты сохранены) и unknown/partial effect (важный unresolved incident). Рекомендую прямо закрепить это различие. Исключение — отдельная реально опубликованная persistent/actionable проблема конфигурации, уведомление которой строго относится к ней. Startup-only unknown нельзя преобразовать в sync unknown.
2. **Не подавлять исходную проблему из-за отмены команды.** Cancel/UAC rejection блокирует только новый command-result incident. Если до команды уже существовали `expired`, `conflict`, `stale persistent` или другое основание, оно остаётся видимым и классифицируемым. Не делать ранний `return no problem` для любого cancelled/failed результата.
3. **Недоступность доставки не расходует новый preview.** Полезно явно определить: app Off, OS blocked/unknown, DND/no presenter → ни visible preview, ни «delivered», ни расходования права на первый актуальный preview после готовности. Повторять на каждое переключение готовности уже показанный инцидент нельзя. То есть observation identity и submitted/preview identity раздельны. Для DND реальная Windows может оставить запись в Notification Center; макет не должен утверждать такой результат, если он лишь моделирует подавленный banner.
4. **Тест и первичная готовность.** `Send test notification` следует той же explicit readiness и master preference либо явно объясняет отдельное ручное исключение. Самый ограниченный вариант — подчинить его master Off тоже. Тестовый ID не должен заменять/подавлять текущий problem ID, а нажатие Test при blocked/DND/Unknown не сообщает успех доставки. Рендер настройки сам ничего не отправляет. Исходная OS availability должна быть либо явно reviewer sample Allowed, либо Unknown; On не считается проверкой Windows.

## Draft acceptance для независимого source review

### A. Исправления статусов

- Исходный `outcome-unknown`: policy/Diagnostics/export/current-owner facts не называют подтверждённого NevroTick владельца; last-known квалифицирован, authority не расширена.
- Windows handover confirmed → observer Start/Stop/Restart → runtime unknown: NevroTick runtime unknown, его изменяемый startup unknown только при необходимости; Windows owner/Windows observation отдельно сохранены/квалифицированы, synchronization unverified. Никакой новый clock writer и никакого blind retry.
- Observe-only: clock writes disabled; не предполагаются ни Windows owner, ни synchronized. `Check sources` не коррекция и не handback.
- Confirmed stopped: главный action ведёт к управлению/диагностике; не enabled Sync now. Unknown/pending не получает false stopped или разрешение новой проверки/коррекции. Трей тот же смысл.
- Все старые NTS/Roughtime/история/возраста/бюджеты остаются raw-наблюдениями; настоящее current agreement не создаётся от Start/Running/handback.

### B. Классификатор

- Все 30 исходных сцен явно классифицированы. `tone` сам по себе не incident severity. Source RATE при qualified agreement, deliberate NTP/observe-only, expected resume/acquiring/recovery/guard не создают alarm.
- Persistent gate нужен для soft-degraded семейства; явный reviewer факт, не render count, число кликов или frozen age.
- Сначала учитывать current operational outcomes: startup unknown отдельно от runtime unknown, partial cleanup отдельно от fully disabled, Windows owner отдельно от stopped Nv. Failed-before-effect не становится sync outage, но и не скрывает прежний current incident.
- User-requested Stop/Disable/Windows handover не unexpected outage. Их unknown/partial effects могут требовать уведомления с точным смыслом.
- Исторический журнал или старый signed-conflict после перевода служб не создаёт новый current bootstrap incident.

### C. Episode и readiness

- Одинаковый incident identity при repeated publish, render/navigation/filter/mode/export: один preview. Dismiss не resolve и не разрешает следующий identical preview.
- Новое meaningful escalation/отдельный incident допустим; bounded запись. Формулировка «resolved» не создаётся от Running, Roughtime-only success или reset.
- Master Off, OS blocked/Unknown/DND/unsupported, no presenter: current app issue остаётся; no visible preview; no delivered/read claim; raw service/state unchanged. On не выдаёт Windows permission.
- Unsent eligible incident после Off→On/blocked→allowed может показаться по новому явному observation publish один раз; ранее submitted incident не повторяется по readiness toggle.
- Test использует отдельный ID, не изменяет problem scope/dedup/authority; неподтверждённая готовность не test success.

### D. Activation и границы

- Только known opaque/local activation IDs с проверкой current incident/generation. Поддельный/невалидный ID не открывает опасный dialog/URL и не создаёт mutation.
- Старое correction notification после expired/startup/handover открывает актуальный status/объяснение; не resurrect proposal, не Approve/Apply/Retry/Start/Restart.
- Safe destination достижим из Simple; переход в Power видим и фокус/return context осмысленны.
- Notification body generic, без source host/IP/точного offset/имён пользователя/путей/ключей/диагностических payload. Текущий статус читаем независимо от toast.
- Нет `Notification`, permission request, ServiceWorker/Push, native URI/API/SCM/network effect; фиксированная HTML-имитация явно подписана preview. Существующий native execution stop не подменяется новой mock qualification.

## Граница review

Точные data-shape, ordering reducer callbacks и native readiness остаются за implementer/source review. Fresh браузерные screenshots/keyboard/overflow выполняет назначенный browser QA; этот документ их не заменяет. На implementation freeze нужны новый immutable source commit и SHA готового standalone, после чего исходный смысл проверяется на реально написанном коде.
