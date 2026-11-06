# Независимая browser QA: статусы и уведомления Windows

5 ноября 2026 года. **Pass в пределах HTML browser acceptance.** Открытых браузерных замечаний нет. Проверяющий не менял source/tests и не вызывал Windows/SCM/UAC/реестр/часы или настоящие уведомления.

Финальный production source: `62aa9713548f8bfefc7ea087a01d788bc6f39c10`; standalone SHA-256 `4d8c3d211ec19ddea21a4b37d6b164f453a8ab8352cca6c554c8c8ac28370f86`. Идентичность проверена до навигации. Полный проход неизменной логики выполнен на родительском `7f0a933c381f1092fde563fdd9c64c71f9f0d41e` / SHA `6c916c8b2ee7d0833b736641e27206b992727401018236485b3f5c1812c32cd0`; затем финальная ограниченная delta и новые снимки приняты отдельно. Финальная delta сохраняет текущие incidents отдельно от ответа последней отменённой/отклонённой команды. Она не требует полного повтора всех неизменных маршрутов.

Чтобы не смешать проверку со следующим rebuild, локальный HTTP отдавал отдельные frozen snapshots: `src/` и `assets/` сверены с точными git objects; standalone проверен по назначенному SHA, index SHA записан в `verification-summary.json`. `file://` в облачном Chromium ограничен политикой; проверялись HTML через localhost.

## Пройденные шаги

1. **Settings в Simple и Power — исправно.** Общий Notify me about problems доступен в Simple Settings; явная ссылка открывает Notifications в Power. Предпочтение сохраняется при смене режима. App On отдельно от Windows Allowed/blocked/Unknown и работающего приложения/трея. Страница Windows settings является честным navigation preview и не запускает URI.
2. **Обычные и проблемные статусы — исправно.** Через внешний publish проверены все30сцен в обоих HTML при360px. Healthy, единичный RATE при альтернативах, deliberate NTP/observe, нормальный startup/resume/Roughtime progress/slew/история не создают проблему по одному цвету. Expired, conflict, signed contradiction, ownership conflict, correction review, unknown clock outcome и неожиданно stopped дают правильный текущий incident. Stale не объявляется stopped.
3. **Ожидание persistent condition — исправно.** Offline, потеря status connection и отклонённая одиночная Roughtime-подпись не показывают toast от повторных кликов; отдельное опубликованное persistent/actionable наблюдение разрешает пример. Подпись не объявляется доказанной атакой. Frozen ages не используются как таймер.
4. **Готовность и тест — исправно.** Master Off, Windows app/policy block, Unknown, unsupported, DND и отсутствующее приложение/трей не показывают баннер и не расходуют ещё не показанный актуальный incident. Разрешение готовности само не создаёт события. Test отдельный, obeys master/readiness, не подтверждает здоровье часов или OS delivery и не поглощает problem incident.
5. **Повторы и завершение — исправно.** Publish/navigation/mode/export не создают повтор одного incident. Dismiss не устраняет проблему и не разрешает новое всплытие. Плановые Stop/Disable и успешная передача не превращаются в unexpected failure; Running/сброс сцены не порождают сообщение о восстановлении. Cancel/UAC/rejected-before-effect не создают новый incident и не скрывают существующий.
6. **Activation — исправно.** Current ID открывает актуальный контекст; invalid ID игнорируется; старый correction notification после runtime-запроса не воссоздаёт approval и не повторяет pending-команду. Внешний activation во время dirty helper editor проходит обычный leave guard; Stay here сохраняет введённое значение. Для эмуляции внешнего события использовался DOM click существующей кнопки вне modal, без доступа к внутреннему model/reducer. Настоящий OS activation не проверялся.
7. **Статусные P2 и действия — исправно.** Outcome-unknown не публикует bare «Clock ownership: NevroTick» в policy/Diagnostics/export. Lost ACK команды NevroTick-наблюдателя сохраняет отдельно подтверждённый Windows manager и sync-unverified, неизвестен runtime NevroTick. Confirmed stopped ведёт к управлению; pending/unknown не оставляют enabled Sync now. Startup-only unknown сохраняет текущий healthy status. Tray повторяет семантику; все ARIA-описания в его dialog разрешаются в единственный существующий элемент.
8. **Финальный durable-incident fix — исправно.** Оба partial (`windows-unavailable`, `product-unavailable`) проверены через повтор Close/UAC-cancel/rejected до эффекта, для уже показанного и ещё не показанного при OS block incident. No active synchronizer остаётся текущей проблемой: после возврата Allowed первый баннер появляется, ранее показанный повторно не всплывает. Startup-unknown аналогично переживает отмену не связанного с ним Stop. Нативных эффектов нет.

## Исполняемые доказательства

| Проверка | Кандидат | Охват / результат |
| --- | --- | --- |
| `independent-notifications-probe.cjs` | frozen `7f0a933` | Оба HTML ×360/1280/480 =6 matrix cases;80 workflow groups, включая60 initial scene classification checks; Pass. |
| `final-bounded-probe.cjs` | frozen `62aa971` / финальный SHA | Оба HTML ×360/1280/480 =6 matrix cases;90 detailed cases:50 partial retry/readiness,10 startup-unknown/unrelated Stop cancellation,30 tray semantics/ARIA; Pass. |
| Дополнительные финальные UI readbacks | frozen `62aa971` | Simple/Power Settings, important toast, DND и no presenter; Pass. |
| Ошибки/соединения | оба прохода | Ноль page errors и неожиданных remote requests. Broad probe дополнительно перехватывал native Notification API: ноль вызовов. |

Воспроизведение требует Playwright/Chromium. Для broad probe задаются `QA_SOURCE_COMMIT`, `QA_STANDALONE_SHA` и при сохранённом snapshot `QA_SOURCE_ROOT`; bounded probe самостоятельно снимает git-object snapshot указанного source и сверяет готовый HTML. Логи, JSON результатов и сырые тексты поверхностей лежат рядом с отчётом. Общая матрица — [acceptance.md](BROWSER-ACCEPTANCE.md), точная идентичность и числа — [verification-summary.json](browser-verification.json).

Ранние попытки исправляли только ошибки внешнего probe: лишнее требование глобально уникального ID между reload; требование сохранять видимую старую карточку при корректном suppress duplicate; неверный wrapper вместо input у helper. Их `attempt*-harness-*.log` сохранены, product-файлы не менялись. Первые скриншоты, где уведомление оказывалось за границей viewport из-за раскрытой reviewer-панели, отклонены для визуального вердикта; финальные области пересняты с правильной прокруткой.

## Осмотр снимков

Свежие финальные снимки осмотрены через image viewer: mobile important toast и Power/Simple Settings; desktop Power Settings и notification; standalone mobile partial retry, desktop Simple Settings и DND на480px. Переносы, основные действия и разделение app preference/Windows readiness читаемы; горизонтальный overflow в принятых случаях не обнаружен. Пример явно подписан HTML sample, не выдаётся за фактическую доставку.

Принятые примеры:

- [Simple Settings360](screenshots/local-360-final-simple-settings.png).
- [Power Settings360](screenshots/local-360-final-power-settings.png).
- [Power Settings1280](screenshots/local-1280-final-power-settings.png).
- [Important toast360](screenshots/local-360-final-important-toast.png).
- [Important toast1280](screenshots/standalone-1280-final-important-toast.png).
- [Partial после отменённой повторной попытки360](screenshots/standalone-360-final-windows-unavailable-retry-unshown.png).
- [Simple Settings1280](screenshots/standalone-1280-final-simple-settings.png).
- [DND480](screenshots/standalone-480-final-suppressed-settings.png).

## Граница доказательств

Это проверка Chromium HTML-прототипа через local HTTP, не настоящей Windows notification platform. Нельзя делать вывод о native app registration, фактической доставке/прочтении, OS permission/DND detection, Session0 IPC, per-user presenter lifetime, protected clock authority или доступности Narrator. Непосредственные CSS/layout/keyboard/ARIA проверки ограничены описанными браузерными поверхностями; полного accessibility compliance нет. Source semantics/CSP/AVIF/checksums и упаковка принадлежат отдельным проверкам координатора.

## Копия для комплекта

Выбранные принятые снимки включены в screenshots; логи и JSON — в evidence. Полный рабочий каталог: `/workspace/nevrotick-ui-audit/notifications-work/final-qa`. Неизбранные и отвергнутые снимки остаются там. Воспроизводимые интеграционные проверки включены в корневой tests/.
