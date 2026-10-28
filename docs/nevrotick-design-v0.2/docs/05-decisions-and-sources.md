# NevroTick — решения, источники и открытые вопросы

**Дата:** 28 октября 2026. **Редакция дизайна:** 0.2.

## 1. Приоритет источников

1. Прямые решения пользователя в этом обсуждении: NevroTick, выбранный третий маскот, time-only v1, будущий DNS, ручной ввод серверов с выбором протокола.
2. [Каноническая архитектура репозитория](../../windows-time-architecture.md), сейчас revision 1.4, — техническая основа. [Архивная копия](../references/source-docs/windows-time-architecture.md) остаётся неизменённым свидетельством полученного комплекта.
3. [RWTC-FEATURES-001 revision 1.0](../../windows-time-features.md) — текущий feature-план со статусом **Proposed v1 product requirements and implementation slices**; [архивная копия](../references/source-docs/windows-time-features.md) сохранена. В v0.2 приняты его уточнения UX; архитектура контролирует технические ограничения.
4. Документы данного пакета — предлагаемый UX поверх этой основы; не новые полномочия сервисов. Матрица F-01–F-10: [06-features-review.md](06-features-review.md).
5. React-прототип и изображения — иллюстрация поведения и визуальной структуры; данные фиктивные.

При текущей сверке исправляется спецификация будущего Rust/Win32 UI. Перенос React-логики или достижение корректности прототипа не являются условием реализации. Исходные HTML/JPG и их QA-отчёт сохраняются как исторические иллюстрации; обновлённые критерии дизайна перечислены в UX-29–UX-35.

Исходный файл архитектуры 1.3 был использован до получения нового архива. После получения архива проверены изменения влияющих на UI разделов; актуальна 1.4. В исходниках сохранён прежний descriptive working title. Название NevroTick установлено более поздним пользовательским решением; исходные исследовательские документы не переписывались.

Features не заменяет архитектуру, не принимает TUF/updater и не закрывает native/signing gates. Принятие его presentation/default choices в дизайне не превращает planned acceptance в выполненные проверки.

## 2. Реестр

| ID | Решение | Статус / основание |
| --- | --- | --- |
| B-01 | NevroTick; техническое имя nevrotick | Выбрано пользователем |
| B-02 | Маскот третьего варианта: белая прядь слева, неодинаковые глаза, синяя куртка, два вида часов | Выбрано пользователем |
| B-03 | Общая подпись System essentials, under control. | Направление принято в обсуждении |
| P-01 | V1: время; DNS позднее | Явное требование пользователя |
| U-01 | Status / Sources / Settings; Diagnostics и About отдельно | Сохранено в дизайне v0.2 |
| U-02 | Ручной DNS/IP-адрес и явный выбор NTS/NTP | Требование пользователя; протокольная модель соответствует архитектуре |
| U-03 | Save создаёт черновик; Review показывает diff; Apply требует системной авторизации | Архитектура §§4–6, предложенная форма UX |
| U-04 | Несовместимый протокол блокирует сохранение; смена профиля не преобразует серверы автоматически | Архитектура §7.5; UX без скрытых профилей |
| U-05 | NTS-KE port и NTP port подписаны по-разному | RFC 8915 / 5905 |
| U-06 | Подсказки bootstrap находятся в Recovery отдельно от нормальных источников | Архитектура §8 |
| U-07 | Нет сетевого Test для произвольного несохранённого адреса | Существующая поверхность команд §§4.2 / 5.3 |
| U-08 | Смещение = reference − Windows; положительное означает, что Windows отстаёт | Архитектура §6 |
| U-09 | Неизвестные и устаревшие значения не отображаются как 0 ms или текущий успех | Архитектура §§9,16 |
| U-10 | UI-прототип на React служит просмотру дизайна; runtime продукта остаётся native Win32/Rust | Архитектура §16 |
| U-11 | Проверка числа строк черновика только структурная; реальный выбор требует eligible agreement minimum **и** строгого большинства | Architecture §7.2; features F-03. Численные defaults не квалифицированы макетом |
| U-12 | Use Windows setting по умолчанию; tray Off; отдельный sign-in observer только при включённом tray | Features F-10/§5; личные preferences не меняют service startup или машинную политику |
| U-13 | Недопустимая дата verifier → понятное Blocked/Recovery | Уточнение архитектуры 1.4 |
| U-14 | Автообновление/TUF не добавляется в продуктовые окна v1 как принятая функция | Update assessment: candidate, not adopted |
| U-15 | Приняты UX-уточнения Features 1.0; исходный документ остаётся Proposed | Новая редакция дизайна v0.2; не принятие всех implementation slices как выполненных |
| U-16 | Profile/headline → activity/remedy → evidence; Time sync active вместо внутреннего Tracking в headline | Features F-04 и §§3.1/4; архитектурная state machine не изменена |
| U-17 | Last valid reply, Last qualified estimate, Last applied correction и Status snapshot age разделены | Features §4/AC-05; requested correction не является broker-acknowledged correction |
| U-18 | Потеря status connection означает Status unavailable и при недостатке доказательств Service health unknown | Features F-04/§4; не утверждать service failure по отсутствию соединения UI |
| U-19 | Certificate-time basis и сохранённая recovery history отдельно от текущего NTS | Architecture §8.3; features §3.2/F-08; один и два helpers сохраняют происхождение после Tracking |
| U-20 | Diagnostics: issue → impact → action; previewable Copy summary и Export | Features F-09/§3.4; факты отделены от гипотез, секреты исключены |
| U-21 | Main / Supporting / Not used / Waiting; неприменённая строка — Draft | Features F-03/§3.2; внутренние названия и wire-schema не переопределяются |
| U-22 | Single-source mode и raw controller tuning отложены из basic v1 | Features F-06/§10; UI не снижает требования при отказах/удалении источников |
| U-23 | Release page — только официальный определённый адрес; пока URL не выдумывается | Features §3.4; открытие будущей страницы не авторизует установку |
| U-24 | Minimize, Close и Quit UI имеют разные последствия для окна, observer и черновика | Уточнение дизайна по F-10; службы продолжают работу во всех трёх случаях |
| U-25 | Apply подтверждает конфигурацию, а не Tracking или clock ownership; correction review согласован со свидетельствами одного предложения | Уточнение дизайна по F-04/F-06 и архитектуре §§5–6, 9–10; UX-33/34 |
| U-26 | Ошибка в Advanced раскрывает поле; выбранный источник и цель Edit/Remove доступны без цвета | Уточнение дизайна по F-10 и архитектуре §16; UX-29/31, будущая native-проверка |
| U-27 | Тема задаёт пары текст/фон/фокус для всех исходов; светлая палитра не переносится буквально в Dark | Критерии дизайна в 01 §4 и 02 §9; UX-30, не заявление о выполненной accessibility-проверке |
| U-28 | Историческая прерванная recovery-процедура отличается от активной; происхождение подсказки не стирается | Уточнение дизайна по F-08/§8; UX-35 |

## 3. Первичные технические источники

| ID | Источник | Использованное основание |
| --- | --- | --- |
| A | [Архитектура 1.4](../references/source-docs/windows-time-architecture.md) | Полномочия, NTS/NTP-профили, bootstrap, UI, состояние, MSI |
| F | [Features 1.0 — неизменённая копия](../references/source-docs/windows-time-features.md) | Proposed-план; принятые в v0.2 уточнения presentation/defaults и будущие AC-кейсы |
| A-U | [Оценка обновлений](../references/source-docs/windows-time-update-assessment.md) | Статус кандидата; отдельное принятие updater не выполнено |
| R1 | [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html), §§3–4, 4.1.7–4.1.8, 8.7 | NTS-KE/TLS, TCP4460, согласование NTP endpoint и отсутствие скрытого перехода на plain NTP |
| R2 | [RFC 5905](https://www.rfc-editor.org/rfc/rfc5905.html), §§7.2,9.1 | NTPv4 и UDP123; порт назначения в пределах политики |
| R3 | [RFC 9525](https://www.rfc-editor.org/rfc/rfc9525.html), §§6.1,6.3–6.4 | Проверяемая DNS/IP-идентичность сервера |
| D1 | [Microsoft Fluent System Icons](https://github.com/microsoft/fluentui-system-icons) | Официальный набор служебных иконок в интерактивном макете |
| V1 | [Knock-Knock, официальная галерея](https://knock.ice-pick.com/art.html) | Исследование общего рисованного визуального настроения |

Базовый пакет фиксирует проверку веб-источников 28 октября 2026. Ранее приведённые RFC, брендовые и прочие веб-источники повторно не проверялись; новые источники критериев контраста перечислены ниже. Текущая редакция сверяет локальные документы и дизайн. Выводы о характере маскота и композиции являются предложениями дизайна, а не внешними эмпирическими утверждениями.

### 3.1. Архивные вложения и текущий репозиторий

В полученном архиве не было **windows-time-development-hardening.md** и **windows-time-signing-guide.md**. Теперь они доступны в репозитории: [development hardening](../../windows-time-development-hardening.md) и [signing guide](../../windows-time-signing-guide.md). Для инженерных этапов используется их фактическое содержание; наличие файлов не доказывает принятия инструмента, выполнения gate или signing approval. Архивные копии и ссылки не переписываются.

Для уточнения визуальных критериев при текущем review использованы [Microsoft — Accessible text requirements](https://learn.microsoft.com/en-us/windows/apps/design/accessibility/accessible-text-requirements) и [W3C — Non-text Contrast](https://www.w3.org/WAI/WCAG22/understanding/non-text-contrast.html). Они обосновывают критерии контраста, а не выбор другого native toolkit или заявление о соответствии.

## 4. Что ещё требуется определить при разработке

- Значения и пределы clock correction, poll, holdover и временного диапазона. Макет не заменяет их квалификацию.
- Единый IDNA/address normalization contract между UI и сервисом. Ограниченный валидатор прототипа не переносится в доверенную границу как готовый сетевой парсер.
- Фактические NTS/сертификатные и Windows-политики, включая восстановление при неверной дате.
- Разрешённые сетевые области и управление локальными серверами.
- Native Win32 accessibility, high contrast, 200%+ scaling, multi-monitor DPI, keyboard/Narrator.
- Финальная векторная и пиксельная отрисовка значков; подготовка ICO.
- Полный MSI-набор экранов и фактическая проверка передачи/восстановления владельца часов.
- Реальные четыре freshness-значения, broker acknowledgement, сохранение recovery provenance и различение connection failure/confirmed service failure.
- Native tray/sign-in observer, default closure без UI process, фактическое следование Windows preferences и измеренный ресурсный бюджет.
- Официальный адрес страницы релизов и применимые gates доступных development-hardening/signing-guide перед соответствующими инженерными этапами.

Прототип не выполняет эти системные операции. Текущие проверки формы и браузерного поведения описаны отдельно в [design-qa.md](../prototype/design-qa.md).

Статус AC-кейсов нового features остаётся **planned / not executed** для реальной программы. Запись «исправлено в v0.2» в документации относится к дизайну/fixtures и не означает native Windows qualification.
