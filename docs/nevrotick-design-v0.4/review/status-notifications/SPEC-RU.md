# Статусы и уведомления Windows — HTML-прототип

Поручение владельца: ещё раз проверить все статусы и сообщать о проблемах через Windows notifications. Продолжаем существующий HTML-прототип; канонический /workspace/NevroTick, Windows, службы, реестр, часы и публикация вне изменений. База ed523ef (source c6e0801). Изолированный worktree notifications-work/NevroTick-UI-v0.4. Прежние артефакты сохраняются.

## Принятый объём и решения координатора

Пользователь уже делегировал реализацию макета без дополнительного согласования технических деталей. Применяется существующий стиль/стек, без нового фреймворка или backend. Два варианта решения: cloud WNS push потребовал бы сервер/каналы и не решает локальную задачу; выбираем локальные Windows app/toast notifications. Основание — windows-notifications-research.md (официальные Microsoft источники). Rich toast deferral прежних документов не блокирует текущее явное поручение для HTML; это не разрешение на нативную реализацию.

1. Исправить подтверждённые статусные дефекты из status-source-audit.md и status-audit: outcome-unknown не называет NevroTick подтверждённым owner; unknown обычного Start/Stop/Restart наблюдателя не стирает независимо подтверждённого Windows owner; observe-only не выдаёт себя за подтверждённого clock writer. Не предполагать Windows owner из Observe only.
2. Исправить действия Status: при confirmed stopped — переход к управлению службой, не enabled Sync now; при неизвестной службе — диагностика/недоступная проверка без ложного stopped; в observe/blocked/outcome-unknown применить понятное Check sources и пояснить, что это измерения, не коррекция/повтор опасной операции. Трей использует тот же смысл действия. Не превращать unknown в разрешение на запись/повтор коррекции.
3. Добавить Settings → Notifications в Power; в Simple Settings тот же переключатель и краткую сводку/переход к подробностям. Страница достижима из обоих режимов. Основное предпочтение Notify me about problems включено в макете по умолчанию. Оно не означает разрешение Windows. Нужны доступный Send test notification (явно test, не подтверждение здоровья) и безопасный пример перехода к настройкам Windows (честно preview, не реальная команда).
4. Разделять preference On/Off, опубликованную доступность Windows (Allowed, blocked app/policy, suppressed/DND, Unknown/unsupported) и наличие активного пользовательского presenter. Эти условия задаются во внешней reviewer-панели. Не запрашивать browser Notification permission, не вызывать Notification/ServiceWorker/Push/API/URI/SCM/реестр/сеть. Никаких настоящих toast в облаке.
5. Показать воспроизводимый образец системного уведомления вне окна приложения, с логотипом, названием NevroTick, кратким понятным заголовком, последствием и безопасным Open NevroTick/Review problem. Важно явно обозначить, что это Windows notification preview; не выдавать HTML-имитацию за Windows или факт доставки. Приложение не может гарантировать показ при Windows block/DND/отсутствии presenter; текущая ошибка остаётся видна в приложении независимо от уведомления.
6. Предварительный контракт будущей программы: службы публикуют ограниченное состояние по защищённому IPC; toast показывает активный unelevated per-user UI/tray presenter. Закрытие окна при работающем tray отличается от Quit последнего presenter. Не добавлять новую always-on службу/обещание фонового детектирования при Quit. Настройка запуска tray не меняет автозапуск служб.

## Политика событий

Единая явная классификация всех30сцен и operational overlays; не делать notification по одному цвету tone или наличию старой записи в журнале. Классификатор использует presentState/current evidence, старые NTS/Roughtime и сохранённые события не становятся новым инцидентом.

- Немедленный пример: потерян пригодный estimate; конфликт пригодных источников; активный проверенный конфликт подписанных дат, мешающий текущему восстановлению; ownership conflict; актуальная коррекция требует review; неизвестный исход clock/service/handover операции; partial cleanup/no active synchronizer; действительно неожиданно остановленная служба.
- Мягкие проблемы holdover/offline/status connection lost/server refusal/all-server backoff/date recovery unavailable/единичная непроверяемая Roughtime-подпись/leap guard — уведомлять только после опубликованного признака persistent/actionable. В макете этот признак задаёт внешний reviewer как тестовое наблюдение. Не придумывать production timeout, не считать два клика или два опроса гарантом persistent. До подтверждения проблема видна в приложении, toast не нужен.
- Не уведомлять о здоровом NTS, постепенной штатной коррекции, нормальном acquiring/resume/Roughtime progress, истории успешного восстановления, полезном silent LI=0, осознанном plain NTP/observe-only, единичном RATE при работающих альтернативах, запрошенных пользователем Stop/Disable, успешном автозапуске/передаче/запуске и обычном Pending. Отмена, UAC-cancel и отказ до любых эффектов дают только inline-ответ и не создают новый ambient alert. Unknown, partial effects и отдельно опубликованная persistent/actionable ошибка требуют уведомления в своей узкой области.
- Windows Running/owner confirmed не означает synchronized/recovered; startup-only unknown не делает текущую здоровую синхронизацию неизвестной: уведомление касается только неподтверждённого автозапуска.
- Только сообщения о проблемах; отдельную опцию уведомлять о восстановлении сейчас не добавлять. Не создавать сообщение об устранении на основе Running или сброса сцены.

## Интерактивность и отсутствие спама

Внешняя reviewer-панель публикует текущий sample status для notification-классификатора и признак persistence. Это детерминированный пример работы service → presenter, без таймеров/сбора. Подтверждённые результаты существующих операций также могут пройти через тот же обработчик, если реализация сохраняет все существующие границы; рендер и навигация не создают новые события.

Повтор одинакового наблюдения в том же инциденте не создаёт новый toast; закрытие карточки не устраняет ошибку и не разрешает новое всплытие на следующем опросе. Существенное изменение/новый инцидент учитывается отдельно; число записей bounded. Навигация, режим, фильтр и экспорт не уведомляют повторно. Preference Off и Windows blocked/unknown/no presenter не показывают toast; DND не выдаётся за видимый баннер. Не использовать fake таймеры, бесконечные live-region объявления или громкий alarm/call/reminder обход системных настроек.

Самостоятельная смена целой сцены — новый независимый набор sample observations, не доказательство recovery. Можно сбросить notification episode при смене сцены, сохранив preference по примеру Time check interval; явно описать это в README. Полный reset возвращает defaults и предупреждает об утрате примеров согласно прежним guards. Тестовое уведомление имеет отдельный test ID, не расходует/не изменяет problem state; оно также подчиняется опубликованной готовности Windows.

Activation передаёт только локальный непривилегированный идентификатор; проверяет его и перечитывает текущие facts. Открывает актуальное Status/Diagnostics/нужную настройку (обозначает переход в Power там, где требуется). Устаревший notification не открывает просроченное предложение и не повторяет операцию. Никаких Approve/Apply/Restart/clock write непосредственно из notification. Нет публичных endpoint/offset/usernames/путей/токенов/diagnostic payload в lock-screen тексте.

## Проверка и завершение

- Исходный audit: все30 сцен, оба режима, crossviews и lifecycle overlays. Только свежие скриншоты/свежие проверки текущего кандидата; старые отчёты — история.
- Исправления статусов проверять содержательно на исходном дефекте + соседних healthy/observe/Windows/unknown случаях; сохранять raw evidence, бюджеты, NTS/RT возраста, rules и предыдущее175tests baseline.
- Новые проверки: матрица30сцен/overlays, persistent gate, дедупликация/dismiss/off/OS suppression/no presenter/invalid+stale activation/test, navigation/focus/reset, отсутствие network/native/browsernotifications и ложной доставки/восстановления.
- ОбаHTML в Chromium360/1280, выбранные480; настоящий visual inspection notification/settings/status screenshots. Independent source+browser acceptance на immutable source и exact standaloneSHA. CSP и AVIF byte equality. HTML/ZIP с checksums и review/report, не менять старые ссылки/публикацию.

## Уточнения независимого review

Одиночная непроверенная/непригодная подпись Roughtime не объявляется атакой: только persistent/actionable сообщение о непригодной основе, при отсутствии независимого пригодного NTS. Активный проверенный конфликт подписанных данных — отдельный immediate incident. Отмена/UAC-cancel/rejected-before-effects не являются новой ошибкой синхронизации. Native/OS Show не является доставкой или чтением.

Дополнение source review: отмена/rejected-before-effect не порождают новый incident, но не скрывают уже существующий expired/conflict. Off/OSblocked/unknown/no presenter не расходуют первый ещё не показанный eligible notification; повторная готовность не повторяет уже показанный эпизод. Test имеет отдельный ID, соблюдает также master Off и не расходует problem episode. Readiness сама по себе не является новым incident или доказательством здоровья.
