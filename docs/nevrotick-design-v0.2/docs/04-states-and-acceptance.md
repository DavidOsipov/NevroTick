# NevroTick — состояния и проверка дизайна v0.2

**Дата:** 28 октября 2026 года.  
**Статус:** план состояний и проверки дизайн-прототипа; перечисление сценария не означает, что он реализован или проверен.  
**Архитектурная основа:** RWTC-ARCH-001 revision 1.4; presentation refinements — RWTC-FEATURES-001 revision 1.0, Proposed.  
**Связанный документ:** [02-ui-spec.md](02-ui-spec.md).

## 1. Область и происхождение решений

**Пользовательский выбор:** NevroTick, утверждённый персонаж, v1 времени и ручное добавление NTP/NTS-серверов. Будущий DNS не появляется в навигации v1.

**Архитектурные требования:** реальные состояния ядра и службы, отсутствие тихого downgrade, раздельное доверие/согласие/свежесть/неопределённость, административные границы и безопасное восстановление. Источник: [§9, Runtime state machine](../references/source-docs/windows-time-architecture.md#9-runtime-state-machine), [§4.2](../references/source-docs/windows-time-architecture.md#42-operation-authorization), [§7.5](../references/source-docs/windows-time-architecture.md#75-operating-profiles), [§8](../references/source-docs/windows-time-architecture.md#8-recovery-when-the-system-date-breaks-tls).

**Предложение v0.2:** ID сцен, English microcopy, визуальная группировка и сценарии прототипа ниже. Применены выбранные presentation refinements из [features revision 1.0](../references/source-docs/windows-time-features.md), чей исходный статус остаётся **Proposed**. Архитектура 1.4 сохраняет авторитет для trust, authority и state semantics. Это интерфейсная проекция, не новая state machine движка. TUF и автоматический updater не приняты; отдельного набора production Updates-состояний нет. Основание: [update assessment §1](../references/source-docs/windows-time-update-assessment.md#1-assessment).

**Фактический каталог поставки:** см. [индекс снимков](../screens/README.md). Перечень требований и review-сценариев ниже шире отдельных кадров; наличие кадра не доказывает выполнение каждого перехода. Итоговое число сцен/снимков определяется собранным прототипом, не этим планом.

Features ссылается на windows-time-development-hardening.md и windows-time-signing-guide.md; этих вложений в полученном комплекте нет. Их содержание и дополнительные проверки не оценены. Сводный разбор F-01–F-10: [06-features-review.md](06-features-review.md).

## 2. Независимые оси состояния

Один зелёный индикатор не может заменить следующие сведения:

| Ось | Примеры |
| --- | --- |
| Работа службы | Starting / Running / Stopping / Confirmed failure / Unknown |
| Связь UI со службой | Connected / Reconnecting / Incompatible / Last snapshot only |
| Управление часами | Product ownership / Observe only / Ownership conflict / Unknown |
| Этап синхронизации | Acquiring / Tracking / Holdover / Conflict / Expired / Blocked |
| Аутентификация | NTS / None — NTP / Not established |
| Основание сертификатного времени | Trusted local / saved authenticated model / plain bootstrap / operator approximate / unknown; отдельно от packet authentication |
| История восстановления | Не использовалось / одно- или двухпомощниковая подсказка с сохранённым происхождением; успешный Tracking не стирает историю |
| Согласие | Достаточное / недостаточное / противоречивое; большинство и минимум проверяются отдельно |
| Свежесть и качество | Last valid reply, Last qualified estimate, Last applied correction, Status snapshot age — четыре разных значения; оценка неопределённости и пригодность модели |
| Изменения пользователя | No draft / Draft / Awaiting approval / Applying / Applied / Rejected / Unknown result |

Например, служба может работать при Conflict; NTS-пакеты могут быть корректно аутентифицированы при противоречии времени; UI может потерять связь, пока службы продолжают синхронизацию. Доступные действия определяет действующая политика и актуальность снимка, а не цвет карточки.

Доказательств для состояния **Status connection unavailable** недостаточно для **Service failed/stopped**. Это различие сохраняется в headline, footer, Diagnostics, summary и export. Возраст qualified estimate не заменяется возрастом доставки snapshot, а запрос коррекции не превращается в applied correction. Если elapsed time неизвестно после разрыва continuity, нужен Unknown/reacquiring, а не расчёт по ненадёжному wall clock.

Источник: [§6, StatusSnapshot и происхождение данных](../references/source-docs/windows-time-architecture.md#6-domain-types-and-state-authority), [§9](../references/source-docs/windows-time-architecture.md#9-runtime-state-machine), [§16.2](../references/source-docs/windows-time-architecture.md#162-main-window).

Presentation contract: [features F-04](../references/source-docs/windows-time-features.md#f-04--show-truthful-status-immediately), [§3.1–3.2](../references/source-docs/windows-time-features.md#31-main-window), [§4](../references/source-docs/windows-time-features.md#4-state-and-freshness-presentation).

## 3. Основные состояния

| ID сцены | Архитектурное состояние | Предлагаемый English copy | Обязательное поведение и действие |
| --- | --- | --- | --- |
| status-initializing | Initializing | **Starting and checking configuration** | Нет утверждения о синхронизации; видна проверка готовности/владения. При конфликте — read-only/Blocked |
| status-acquiring | Acquiring | **Checking time sources** | Согласие ещё не установлено; прошлую пригодную модель можно показать с её возрастом |
| status | Tracking + NTS | **Time sync active** / **NTS only** | Подходящие источники, аутентификация, согласие, возраст и неопределённость показаны отдельно |
| status-ntp | Tracking + NTP | **Time sync active** / **NTP — unauthenticated** | Успешная дисциплина не скрывает отсутствие аутентификации |
| status-holdover | Holdover | **No fresh time check — using the previous model** | Нет новых поправок от отсутствующих источников; неопределённость растёт; видны возраст и повтор |
| status-conflict | Conflict источников | **Time sources disagree** | Нет новой коррекции по спорным источникам; поддерживается только допустимая прежняя модель |
| status-reacquiring | Reacquiring | **Rechecking after a change** | Старые измерения не выглядят текущими; объяснение сна/изменения часов/среды по фактической причине |
| status-expired | Expired | **No usable current time estimate** | Бюджет возраста/ошибки исчерпан; зелёный Tracking снят; bounded retries продолжаются |
| status-bootstrap-hints | BootstrapHints | **Recovering secure time using an unauthenticated approximate date** | Подсказки не меняют Windows-часы; текущая стадия сбора hints уточняется отдельно |
| status-recovery | BootstrapNts | **Recovering secure time using an unauthenticated approximate date** | Уточняется стадия NTS-проверки; нужны дальнейшие проверки времени, согласия и полномочий |
| status-blocked | Blocked | **Synchronization needs attention** | Конкретная причина, фактический режим часов и доступное действие; без универсального Fix now |
| status-stopping | Stopping | **Finishing background work** | Не сообщать Stopped до завершения очистки/подтверждения; ошибка очистки не маскируется |

Ни одна сцена не назначает безопасный срок holdover, допуск коррекции или диапазон дат. Иллюстративные значения в снимках не считаются измерениями либо дефолтами политики.

Tracking после одно- и двухпомощникового восстановления требует отдельных примеров данных: обычный healthy headline сочетается с сохранённой recovery history и фактическим certificate-time basis. Это тот же Tracking, не новое право доверять исходной подсказке.

Источник: [§9](../references/source-docs/windows-time-architecture.md#9-runtime-state-machine), [§10.2–10.4](../references/source-docs/windows-time-architecture.md#102-steps-and-large-errors).

## 4. Дополнительные обязательные варианты

| ID сцены | Ситуация | Текст и контракт |
| --- | --- | --- |
| status-observe | Observe only | **Measuring reference time — clock changes disabled.** Профиль Observe only и фактическая аутентификация всё равно показаны |
| status-no-sources | Первый запуск без источников | **No time sources configured.** Действие **Add source**; нет нулей, означающих «идеальную точность» |
| status-offline | Нет сети | **Network unavailable.** Возможны Holdover или Expired в зависимости от реальной модели; Offline не заменяет эту информацию |
| status-partial | Часть серверов недоступна | Количество подходящих источников и результат политики согласия; нельзя автоматически объявлять успех или failure лишь по одной строке |
| status-stale | UI потерял связь | **Status unavailable — last update was …** Исторические значения; **Service health unknown**, пока нет отдельного подтверждения отказа |
| status-incompatible | UI/служба несовместимы | **This app cannot communicate with the installed service version.** Поддержка/полное MSI maintenance; без выдуманного текущего статуса |
| status-ownership-conflict | W32Time/другой контроллер/GPO | **Another service or policy controls Windows time.** Новые записи часов прекращаются по архитектурной политике; администратора не обходят |
| status-approval-required | Большая коррекция | **Review clock correction — Administrator required.** NTS может быть уже получен, но изменение ещё не разрешено |
| status-date-range | Revision 1.4: недопустимая дата verifier | **System date is outside the supported verification range.** Recovery/Blocked; нет clamp, wrap или падения в цикл перезапуска |
| status-certificate-error | Ошибка идентичности/цепочки/отзыва | Понятная конкретная причина. Нет игнорирования сертификата или автоматического NTP |
| status-service-stopped | Подтверждённая остановка службы | **Time services are stopped.** Только если остановка действительно известна, а не при одном разрыве UI-соединения |

Полный набор причин Blocked также включает недействительную обязательную конфигурацию, неопределённый журнал полномочий/владения, отказ прав, исчерпанный разрешённый бюджет или неподтверждённую очистку после сбоя. На главном экране показываются причина и действие, низкоуровневые подробности — в Diagnostics.

Источники: [§6.4](../references/source-docs/windows-time-architecture.md#64-boundary-ownership-and-failures), [§7.5](../references/source-docs/windows-time-architecture.md#75-operating-profiles), [§8.2–8.4](../references/source-docs/windows-time-architecture.md#82-one-server-and-two-server-policies), [§11](../references/source-docs/windows-time-architecture.md#11-w32time-ownership-and-installation-lifecycle), [§18.7.2, extreme-date case](../references/source-docs/windows-time-architecture.md#1872-certificate-policy-is-a-separate-windows-integration-decision).

## 5. Состояния действий

### 5.1 Sync now

Видимые ответы: **Checking**, **Already checking**, **Waiting for enough agreeing sources**, **Try again after the current limit**, **Correction in progress**, конкретный blocked/unavailable outcome. Они отражают реальный результат запроса, а не обязательную анимацию успеха. Уровень headline может оставаться Time sync active во время разрешённой постепенной коррекции, если действующая политика это подтверждает.

- Повторные нажатия объединяются или ограничиваются; **Sync now** не выполняет произвольный step.
- Completion означает подтверждённый результат операции, а не доказанную абсолютную точность UTC.
- При потере подтверждения сначала выясняется результат. Новый необратимый эффект не запускается слепым повтором.
- Результат refresh/измерений подтверждает служба; завершение проверки само по себе не означает изменение часов и не обновляет Last applied correction. Только фактически применённая коррекция с broker acknowledgement обновляет это значение. Закрытие окна не отменяет принадлежащее службе измерение или разрешённую коррекцию.

Источники: [§5.3](../references/source-docs/windows-time-architecture.md#53-broker-command-surface), [features F-05](../references/source-docs/windows-time-features.md#f-05--request-a-fresh-check-with-sync-now).

### 5.2 Редактирование и применение

| ID | Состояние | Ожидаемый результат |
| --- | --- | --- |
| edit-valid | Корректный черновик | Можно перейти в Review; активная политика ещё прежняя |
| edit-invalid | Неверное поле/комбинация | Сохранены данные; ошибка связана с полем; фокус на первой ошибке; применение недоступно |
| edit-profile-mismatch | Обычный NTP в NTS-наборе | Явный выбор разрешённой роли либо отдельной смены профиля; никакого молчаливого перевода |
| edit-duplicate | Повтор идентичности/endpoint; несколько источников одной operator group | Дубликаты идентичности/endpoint отклоняются. Общая operator group сохраняет сведения о зависимости источников и сама по себе не является ошибкой; её участники не объявляются дополнительными независимыми голосами |
| edit-agreement-impossible | Удаление/отключение ломает правило | Минимум не снижается автоматически; необходимо исправить черновик |
| apply-awaiting-uac | Ожидается UAC | UI заранее объясняет действие; пароль вводится только системными средствами |
| apply-cancelled | Пользователь отменил UAC | Старая политика активна; черновик и фокус сохранены |
| apply-denied | Нет прав | Конкретное сообщение; нет частичного изменения |
| apply-conflict | Политика изменилась параллельно | Показать текущую версию и черновик для повторного рассмотрения; без force-overwrite |
| apply-persistence-failed | Ошибка сохранения | Прежняя политика остаётся действующей; новый набор не изображается активным |
| apply-result-unknown | Потеря связи после принятия операции | **Checking the result…**; сначала reconcile, затем решение о повторе |
| apply-complete | Подтверждённое применение | Убрана метка черновика; показана новая активная политика; новые данные собираются для неё |
| apply-with-controller-limit | Apply принят при Blocked, readiness checks или незавершённой остановке | Подтверждение конфигурации не снимает ограничение управления часами; новые источники Waiting, актуальное ограничение явно сохранено |

Источники: [§4.2](../references/source-docs/windows-time-architecture.md#42-operation-authorization), [§5.2](../references/source-docs/windows-time-architecture.md#52-message-contract), [§6.2–6.5](../references/source-docs/windows-time-architecture.md#62-exact-json-and-resource-contract), [§12.2](../references/source-docs/windows-time-architecture.md#122-configuration-semantics).

### 5.3 Bootstrap и одноразовое разрешение

У двух helpers один timeout, несовпадение либо дублирование не превращают политику в однопомощниковую. История слабого основания сертификатного времени остаётся доступной после последующего успешного handshake и возвращения к Tracking. Одно- и двухпомощниковый сценарии оба сохраняют это происхождение. Source details и Diagnostics показывают его по данным службы, не выводят из наличия NTS.

Если большое изменение потребовало отдельного разрешения, устаревшее предложение после сна, смены политики или новых данных отклоняется как устаревшее. Пользователь возвращается к свежим измерениям. **Approve once** не становится постоянной настройкой и не разрешает другой target.

В отдельной сцене **correction-expired** показано устаревшее предложение; **Request approval** недоступна. Проверяемое сообщение ведёт к новым измерениям, а не к повторному разрешению прежней коррекции.

Предложение коррекции согласовано с относящимися к нему Status и Source details. Знак/величина не противоречат свежим интервалам источников; текущие и исторические свидетельства различимы. Смена recovery-политики сохраняет происхождение предыдущей процедуры и её подтверждённый исход, но не изображает прерванную процедуру продолжающейся.

Источник: [§8.2–8.3](../references/source-docs/windows-time-architecture.md#82-one-server-and-two-server-policies), [§10.2](../references/source-docs/windows-time-architecture.md#102-steps-and-large-errors).

## 6. Сценарии проверки дизайна и будущего native UI

Это сценарии review и будущей native-приёмки, не отчёт об исполненных тестах. Реализация пересоздаётся на Rust/Win32 по каноническим документам; исправлять или воспроизводить внутреннюю логику React-прототипа для выполнения этих требований не нужно. HTML и JPG служат визуальными иллюстрациями и могут отставать от уточнений спецификации.

| ID | Сценарий | Что оценивать |
| --- | --- | --- |
| UX-01 | Первый запуск без серверов → Add NTS source | Пустое состояние, явные поля/протокол, черновик и переход к применению |
| UX-02 | NTS-source → ошибка hostname/port/profile | Ошибка рядом с полем, summary, сохранение ввода и клавиатурный фокус |
| UX-03 | Добавить NTP при активном NTS | Видимость ограничения профиля; bootstrap не смешивается с обычным набором |
| UX-04 | Удалить источник, нарушив согласие | Применение блокируется с понятной причиной; минимум не исправляется за пользователя |
| UX-05 | Review → UAC cancelled | Активные значения прежние, draft сохранён, фокус возвращён |
| UX-06 | Review → settings changed elsewhere | Нельзя молча перезаписать новую политику; доступна повторная проверка |
| UX-07 | Sync now при нормальной работе и при Conflict | Статусы запроса различимы; нет принудительного успеха |
| UX-08 | Потеря сети → Holdover → Expired | Возраст и неопределённость видимы; успех не сохраняется бессрочно |
| UX-09 | Потеря связи только между UI и службой | Stale не выдаётся за достоверную остановку фоновой синхронизации |
| UX-10 | Сон/внешнее изменение времени → Reacquiring | Старые цифры помечены; новое согласие нужно заново |
| UX-11 | Bootstrap с двумя helpers: один не отвечает | Нет тихого перехода к одному или записи подсказки в Windows-часы |
| UX-12 | NTS получен, коррекция вне политики | Показан отдельный запрос разрешения; NTS-success не выдаётся за уже выполненное изменение |
| UX-13 | Предложение коррекции устарело во время подтверждения | Нет повторного применения старого target; понятен путь к свежим измерениям |
| UX-14 | Неподдерживаемая дата и неверная идентичность сертификата | Разные причины и подходящие действия; ни один путь не отключает валидацию |
| UX-15 | Observe only / ownership conflict | Ясно, меняет ли приложение Windows-часы; организация/GPO не обходится |
| UX-16 | Diagnostics → Export preview → Save | Частные сведения отредактированы по умолчанию, секретов нет, отправка не выполняется |
| UX-17 | Settings: тема/трей против изменения источников | Личные предпочтения не требуют UAC; машинная политика требует |
| UX-18 | Закрытие окна с треем и без него | UI закрывается по выбранному поведению, служба продолжает работу |
| UX-19 | Полный MSI: install/update/repair/uninstall, включая отказ | Один продукт, достоверный исход и восстановление; нет страницы auto-update/TUF |
| UX-20 | Клавиатура, длинные имена, увеличение текста/окна | Доступные действия, предсказуемый focus, отсутствие обрезанного критического содержания |
| UX-21 | Ответ источника, qualified estimate, applied correction и snapshot имеют разные возраста | Разные подписи и данные; snapshot refresh не делает измерение или коррекцию новыми |
| UX-22 | Status connection потеряна; затем отдельно подтверждён отказ службы | Первый случай показывает Unknown, второй подтверждённый failure; headline, footer, Diagnostics и export согласованы |
| UX-23 | Успешный Tracking после одного и после двух helpers | Certificate-time basis/history сохраняются; аутентификация новых пакетов не переписывает историю |
| UX-24 | Diagnostics → Copy summary / Export; ошибка clipboard или сохранения | Предпросмотр соответствует фактическим полям; секреты исключены; ошибка не называется успешным копированием |
| UX-25 | Tray Off → запрос sign-in → Tray On → sign-in On → Tray Off | Sign-in относится к observer и зависит от tray; service startup не изменяется |
| UX-26 | Первое открытие и смена системной темы | По умолчанию Use Windows setting; светлая/тёмная презентация следует выбранному режиму без потери данных |
| UX-27 | Повторный Sync now, недостаточное согласие, rate limit, активная коррекция | Разные исходы Checking/Already checking/Waiting/limit/Correction in progress; нет выдуманного completed |
| UX-28 | Sources: Main/Supporting/Not used/Waiting и неприменённый Draft | Краткая причина доступна в обычной строке; Draft не выглядит действующим ожидающим источником |
| UX-29 | Неверный port/time scale в закрытой Advanced; неверный recovery helper | Группа раскрыта, ввод сохранён, поле и ошибка связаны, фокус и объявление ведут к исправлению |
| UX-30 | Light / Dark / System / contrast themes, включая выбранную строку и Changes applied | Текст и фокус читаемы на каждой поверхности; светлый фон успеха не остаётся под светлым текстом Dark; системные contrast colors не перекрыты |
| UX-31 | Выбор источника клавиатурой → Edit / Remove | Выбранный сервер и цель действия доступны без цвета и мыши; раскрытие подробностей не подменяет выбор |
| UX-32 | Minimize, Close с Tray Off/On, явный Quit UI при черновике | Выполняются отдельные последствия из §2 спецификации; Minimize сохраняет процесс/черновик, Quit UI завершает observer, ни одно действие не останавливает службы |
| UX-33 | Большая коррекция: Status → approval → Source details | Направление, величина, пересчёт единиц и свидетельства одного предложения согласованы; интервал −0.7 ms ±4.8 ms не служит основанием +372 s. Разные центральные оценки совместимых источников допустимы |
| UX-34 | Успешный Apply при ограничении управления часами | Новая конфигурация не изображается квалифицированной оценкой и не снимает Blocked/Stopping/readiness; текущая и новая evidence разделены |
| UX-35 | Подтверждённая смена recovery-политики при незавершённой процедуре | Прежняя процедура историческая/прерванная, исходная подсказка остаётся неаутентифицированной; неизвестный результат сначала reconcile |

Дополнительные данные для проверки редактора: IPv6, IDNA/Unicode display name, дубли после канонизации, запрещённая локальная/многоадресная конечная точка по политике, несовместимая шкала времени, превышение допустимого количества источников и неточные единицы. Не показывать raw parser dump или произвольный ввод в небезопасном сообщении ошибки.

## 7. Краткий контрольный список рецензента

1. Основной экран начинает с profile/headline, затем activity/remedy; evidence не заслоняет необходимое действие.
2. NTS не подменяет согласие и оценку неопределённости; число строк не подменяет независимость.
3. Есть Tracking, Holdover, Expired, оба вида Conflict, Observe only и различимые status-disconnected/service-failed.
4. Ручной редактор поддерживает NTP/NTS, явную роль и границу профилей.
5. Нет silent downgrade, certificate bypass, автоматического W32Time fallback или неутверждённого updater.
6. Машинное изменение имеет review, предупреждение о UAC и корректные cancellation/revision-conflict/unknown-result.
7. Sync now — измерение; большая коррекция — отдельное одноразовое актуальное разрешение.
8. Четыре возраста различимы; unknown/stale не выглядит как нулевая ошибка, актуальная оценка или подтверждённая коррекция.
9. Клавиатура, Escape, возврат фокуса, увеличение текста и длинные имена сохраняют работоспособность.
10. Цвет не единственный сигнал; цифровые обновления не создают шум для screen reader.
11. Diagnostics объясняет issue/impact/action; Copy summary и export имеют корректный предпросмотр, редактирование и recovery provenance.
12. System appearance, opt-in tray/sign-in, закрытие UI и обслуживание MSI имеют точные отдельные последствия.

## 8. Что может и чего не может подтвердить этот пакет

Дизайн-review может проверить полноту экранов, понятность текста, отсутствие противоречивых обещаний и соответствие предложенных переходов исходной архитектуре. Интерактивный HTML-прототип может продемонстрировать навигацию и часть валидации на фиктивных данных.

Он не подтверждает реальную работу NTP/NTS, настройки Windows-часов, UAC/SCM/IPC, корректность подписей MSI, возврат владения W32Time, фактический native UI Automation/Narrator, расход ресурсов или ASVS-conformity. Эти проверки остаются отдельными будущими gates: [§20, Verification strategy](../references/source-docs/windows-time-architecture.md#20-verification-strategy-and-acceptance-checks) и [§21, Delivery phases](../references/source-docs/windows-time-architecture.md#21-delivery-phases-and-readiness-gates).

[Features §8, AC-01–AC-20](../references/source-docs/windows-time-features.md#8-acceptance-scenarios-and-evidence) имеет статус **planned / not executed**. Boot-before-login, отсутствие UI-процесса по умолчанию, настоящий tray startup, broker-acknowledged clock trajectory, monotonic freshness, реальные ограничения запросов и native assistive technologies должны проверяться отдельно на поддерживаемой Windows. Событие в fixture или HTML-снимок не является таким свидетельством.

**Delta revision 1.4:** основной инвентарь UI, полномочия и MSI-lifecycle не изменены. В review добавляется явный сценарий unsupported verifier date; conditional HTTP retrieval и legacy-NTP entropy остаются открытыми инженерными вопросами. Детали см. [§18.7.2](../references/source-docs/windows-time-architecture.md#1872-certificate-policy-is-a-separate-windows-integration-decision) и [§22, OPEN-8/9](../references/source-docs/windows-time-architecture.md#22-open-decisions-and-release-blockers).
