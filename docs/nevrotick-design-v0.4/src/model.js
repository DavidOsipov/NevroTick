/* NevroTick design fixture model. No transport, persistence, clock or service authority. */
(function (root) {
  'use strict';
  const SOURCE_COMMIT = '8a66af924c509e898a9bab9bb3282f11b24fba93';
  const clone = value => JSON.parse(JSON.stringify(value));
  const BASE_SOURCES = [
    { id: 'alpha', name: 'Alpha', host: 'alpha.example', protocol: 'NTS', port: 4460, operator: 'Operator A', role: 'Main', offset: -0.8, uncertainty: 4.5, reply: '18 s ago', accepted: '18 s ago', reason: 'Selected reference. Agrees with two supporting sources.', authenticated: true, scale: 'Unsmeared UTC', leap: 'No announcement (LI=0)', endpoint: '192.0.2.11:123' },
    { id: 'beta', name: 'Beta', host: 'beta.example', protocol: 'NTS', port: 4460, operator: 'Operator B', role: 'Supporting', offset: -0.5, uncertainty: 5.1, reply: '21 s ago', accepted: '21 s ago', reason: 'Compatible with the selected reference interval.', authenticated: true, scale: 'Unsmeared UTC', leap: 'No announcement (LI=0)', endpoint: '192.0.2.12:123' },
    { id: 'gamma', name: 'Gamma', host: 'gamma.example', protocol: 'NTS', port: 4460, operator: 'Operator C', role: 'Supporting', offset: -0.7, uncertainty: 4.8, reply: '16 s ago', accepted: '16 s ago', reason: 'Compatible with the selected reference interval.', authenticated: true, scale: 'Unsmeared UTC', leap: 'No announcement (LI=0)', endpoint: '192.0.2.13:123' },
    { id: 'delta', name: 'Delta', host: 'delta.example', protocol: 'NTS', port: 4460, operator: 'Operator D', role: 'Not used', offset: 68, uncertainty: 3.2, reply: '8 s ago', accepted: 'Not accepted', reason: 'Authenticated, but disagrees with the selected interval.', authenticated: true, scale: 'Unsmeared UTC', leap: 'No announcement (LI=0)', endpoint: '192.0.2.14:123' }
  ];

  // Published sample evidence, never inferred from reply counts or provider names.
  // These reserved identities and key labels choose no production trust policy.
  const ROUGHTIME_SOURCES = [
    { host:'date-a.example', operatorGroup:'Demonstration operator A', keyIdentity:'DEMO-KEY-A · placeholder only' },
    { host:'date-b.example', operatorGroup:'Demonstration operator B', keyIdentity:'DEMO-KEY-B · placeholder only' }
  ];
  const DATE_INTERVAL = '2026-11-05 09:58–10:02 UTC · approximate sample';
  const NO_ROUGHTIME = {
    state:'not-needed', scope:'none', reason:'No Roughtime date recovery was requested in this scene.',
    signatureStatus:'Not requested', approximateDateInterval:null, receivedAge:null,
    ntsHandoffState:'No Roughtime handoff in this scene', ancestry:'No Roughtime date basis used',
    clockEffect:'No clock authority from Roughtime', sources:[]
  };
  function roughtimeEvidence(state, patch={}) {
    const usable = ['date-ready','recovered'].includes(state);
    const signatureStatus = usable || state === 'conflict' ? 'Verified in sample evidence' : state === 'rejected' ? 'Not verified · response rejected' : state === 'checking' ? 'Verification pending' : 'No verified response available';
    return {
      state, scope:state === 'recovered' ? 'retained' : 'active-bootstrap', signatureStatus,
      reason:'Signature and date usability are separate decisions in the sample.',
      approximateDateInterval:usable ? DATE_INTERVAL : null,
      receivedAge:usable || ['conflict','rejected'].includes(state) ? '12 s ago · sample' : null,
      ntsHandoffState:'NTS handoff is waiting for a usable date basis',
      ancestry:usable ? 'Signed Roughtime date basis' : 'No usable Roughtime date basis accepted',
      clockEffect:'Windows clock unchanged by date recovery; NTS and separate clock authority are still required',
      sources:ROUGHTIME_SOURCES.map((source,index) => ({...source,signatureStatus,
        approximateDateInterval:usable ? DATE_INTERVAL : state === 'conflict' ? (index ? '2026-11-07 09:58–10:02 UTC · sample' : DATE_INTERVAL) : null,
        receivedAge:usable || ['conflict','rejected'].includes(state) ? '12 s ago · sample' : null
      })), ...clone(patch)
    };
  }

  const BASE = {
    id: 'healthy', group: 'Ежедневная работа', label: 'Нормальная синхронизация', tone: 'good',
    title: 'Time sync active', description: 'Three independently operated sources agree. Your clock is being maintained under the current policy.',
    next: 'Next scheduled check in 40 s', action: 'View sources', actionTarget: 'sources',
    profile: 'NTS only', authentication: 'NTS authenticated', authNote: 'Packet authentication is verified. It does not guarantee absolute accuracy.',
    agree: 3, total: 4, groups: 3, agreement: 'Minimum + majority met', minimum: 2,
    estimate: '18 s ago', snapshot: '2 s old', applied: '3 min ago', appliedDetail: '−1.6 ms acknowledged by the clock broker',
    offset: -0.7, uncertainty: 4.8, interval: '−5.5 to +4.1 ms', historical: false,
    service: 'Background service available', connection: 'Connected', ownership: 'NevroTick',
    correction: 'No phase correction pending', correctionDetail: 'Only acknowledged effects count as applied correction.',
    calendar: 'Reviewed coverage available', calendarNote: 'No transition is pending in this fixture. Sources without a leap announcement remain eligible.',
    recovery: 'Strict NTS verification', recoveryNote: 'No date-recovery basis was needed for these associations.',
    roughtimeEvidence:NO_ROUGHTIME, recoveryStage:0, recoveryStageLabel:'',
    future: false, displayOnly: false, blocked: false, syncResponse: 'checking', diagnosticsResponse: { ui: 'active', service: 'active' }, sources: BASE_SOURCES,
    note: 'Сравните Simple и Power user. В Settings измените частоту проверок; в Power user откройте Statistics и подробности источника Delta.',
    events: [
      { seq: 104, level: 'info', component: 'Time engine', age: '18 s ago', title: 'Qualified estimate accepted', detail: 'Three independent operator groups satisfy the configured minimum and majority.' },
      { seq: 103, level: 'notice', component: 'Time engine', age: '24 s ago', title: 'Delta excluded from this estimate', detail: 'Authentication passed; its interval does not overlap the selected reference.' },
      { seq: 102, level: 'info', component: 'Clock broker', age: '3 min ago', title: 'Correction acknowledged', detail: 'Applied effect: −1.6 ms. No new phase correction is pending.' }
    ]
  };
  function make(id, label, patch) { return Object.assign(clone(BASE), { id, label }, clone(patch)); }
  const SCENARIOS = [clone(BASE),
    make('server-rate-limited', 'Сервер ограничил частоту запросов', {
      description: 'Delta asked for less frequent requests. The other three sources still provide the required agreement; your clock is being maintained under the current policy.',
      next: 'No settings change is needed. Other sources remain available for checks.',
      affectedSource: 'delta', syncResponse: 'checking-available',
      note: 'Delta прислал RATE. Ограничение касается только Delta; остальные три источника обеспечивают нужное согласие. Контрольный ответ не обновляет измерение или подтверждение NTS.'
    }),
    make('server-access-refused', 'Сервер отказал в обслуживании', {
      title: 'Using the last valid estimate', tone: 'warning',
      description: 'Beta refused access. Its prior accepted measurement is separately marked no longer usable by age. The published selection result is Agreement not met; the previous model remains usable.',
      next: 'Review Beta’s usage policy or configuration. No end to the refusal has been established.',
      action: 'Review sources', actionTarget: 'sources', affectedSource: 'beta', syncResponse: 'source-refused',
      agree: 2, groups: 2, agreement: 'Agreement not met',
      agreementNote: 'Published selection result from this fixture; the interface does not infer agreement from the number of remaining sources.',
      estimate: '4 min ago', applied: '5 min ago', offset: -1.1, uncertainty: 12.6, interval: '−13.7 to +11.5 ms',
      retainedModel: true, authentication: 'NTS · current and prior source evidence',
      authNote: 'Current source measurements remain authenticated. Beta’s authenticated measurement is prior evidence; DENY refreshes neither its measurement nor authentication age.',
      correction: 'Using a still-valid prior model', correctionDetail: 'No new qualified estimate is available. The previously learned model remains usable within its age and uncertainty budget.',
      note: 'Beta прислал DENY. Просроченность его прежнего измерения и Agreement not met заданы отдельно. Прежняя модель ещё пригодна; профиль NTS и правило согласия не меняются.'
    }),
    make('acquiring', 'Сбор первых измерений', { title: 'Checking time sources', tone: 'info', description: 'Waiting for enough fresh, compatible measurements. No current time estimate is ready.', next: 'Another bounded check is scheduled', agree: 0, groups: 0, agreement: 'Agreement not established', estimate: 'Not yet available', applied: 'None in this session', appliedDetail: 'No correction has been acknowledged.', offset: null, uncertainty: null, interval: 'Not available', correction: 'Waiting for a qualified estimate', note: 'Свежий ответ сервера ещё не означает готовую оценку времени.' }),
    make('holdover', 'Источники временно недоступны', { title: 'Using the last valid estimate', tone: 'warning', description: 'Fresh measurements are unavailable. The previous model remains usable within its current age and uncertainty budget.', next: 'Retrying sources automatically', agree: 0, groups: 0, agreement: 'No current agreement', estimate: '4 min ago', applied: '5 min ago', offset: -1.1, uncertainty: 12.6, interval: '−13.7 to +11.5 ms', correction: 'No new phase correction', correctionDetail: 'The valid learned frequency is retained. No NTP or Windows Time fallback is enabled.', note: 'Holdover — ограниченное использование прежней модели. Не новая синхронизация и не бесконечный зелёный статус.' }),
    make('expired', 'Оценка времени устарела', { title: 'No usable time estimate', tone: 'danger', description: 'The previous estimate exceeded its permitted age or uncertainty. New clock corrections are withheld.', next: 'Check network access and the configured sources', action: 'Open diagnostics', actionTarget: 'diagnostics', agree: 0, groups: 0, agreement: 'No current agreement', estimate: '24 min ago · expired', applied: '25 min ago', offset: null, uncertainty: null, interval: 'Expired', historical: true, correction: 'New correction withheld', note: 'После исчерпания бюджета числовая оценка не выглядит актуальной.' }),
    make('offline', 'Нет сетевых ответов', { title: 'No replies from time sources', tone: 'warning', description: 'The latest attempts received no replies. Network access may be unavailable; the cause has not been established.', next: 'Check your connection; automatic retries continue', action: 'Open diagnostics', actionTarget: 'diagnostics', agree: 0, groups: 0, agreement: 'No current agreement', estimate: '2 min ago', offset: -0.9, uncertainty: 8.2, interval: '−9.1 to +7.3 ms', correction: 'Using a still-valid prior model', note: 'Наблюдение «нет ответов» не превращается в неподтверждённый диагноз «виноват firewall».' }),
    make('conflict', 'Аутентифицированные серверы расходятся', { title: 'Time sources disagree', tone: 'danger', description: 'Authentication passed, but no group meets the required agreement. Disputed observations cannot authorize a new correction.', next: 'Inspect the conflicting intervals', agree: 2, groups: 2, agreement: 'Majority not met', estimate: '42 s ago · previous estimate', offset: null, uncertainty: null, interval: 'No agreed interval', historical: true, correction: 'New correction withheld', note: 'NTS может пройти аутентификацию, но не дать согласованного времени. Требование большинства не снижается.' }),
    make('stale', 'Потеряна связь с состоянием', { title: 'Current status unavailable', tone: 'warning', description: 'Showing the last received snapshot. The service may still be synchronizing; its current health is unknown.', next: 'Waiting to reconnect to status', action: 'Open diagnostics', actionTarget: 'diagnostics', authentication: 'NTS · last snapshot', agreement: 'Last known: minimum + majority met', estimate: '18 s at last snapshot', snapshot: '2 min 14 s old', applied: '3 min at last snapshot', historical: true, service: 'Service health unknown', connection: 'Disconnected', ownership: 'Unknown · last known NevroTick', correction: 'Current correction unknown', correctionDetail: 'The last snapshot cannot establish current clock effects.', note: 'Потеря связи с UI не равна остановке службы. Все прежние значения помечены как исторические.' }),
    make('service-stopped', 'Служба подтверждённо остановлена', { title: 'Background service is stopped', tone: 'danger', description: 'A current service-state observation confirms that synchronization is unavailable.', next: 'Use the supported repair route for the installed package', action: 'Open diagnostics', actionTarget: 'diagnostics', agree: 0, groups: 0, agreement: 'Unavailable', estimate: 'Last known estimate only', applied: 'Last acknowledged effect only', offset: null, uncertainty: null, interval: 'Unavailable', historical: true, service: 'Stopped · confirmed', connection: 'Service observer only', ownership: 'Must be reconciled before recovery', correction: 'No live correction status', note: 'Отдельная сцена подтверждённой остановки. Тексты обслуживания согласованы с выбранным полным MSIX.' }),
    make('blocked', 'Конфликт владельца часов', { title: 'Another time controller is active', tone: 'danger', description: 'NevroTick cannot establish exclusive clock ownership. Time measurements do not grant permission to change the clock.', next: 'Resolve clock ownership through the supported setup flow', action: 'Review policy', actionTarget: 'settings', ownership: 'Conflict detected', blocked: true, correction: 'Clock writes blocked', correctionDetail: 'There is no automatic takeover or Windows Time fallback.', note: 'Пригодные измерения и право менять часы — разные условия.' }),
    make('resume', 'Повторная проверка после сна', { title: 'Rechecking time after resume', tone: 'info', description: 'The previous clock mapping was invalidated. Fresh observations are required before tracking resumes.', next: 'Collecting measurements for the new session', agree: 0, groups: 0, agreement: 'Fresh agreement required', estimate: 'Unknown after clock-generation change', offset: null, uncertainty: null, interval: 'Not yet available', correction: 'Transient correction cancelled', correctionDetail: 'Time spent asleep is not counted as delivered adjustment.', note: 'Сон не считается временем выполненной коррекции. Старые callback и измерения не получают новую свежесть.' }),
    make('observe', 'Только наблюдение', { title: 'Observing time', tone: 'info', description: 'Sources are measured and compared. This profile does not change the Windows clock.', profile: 'Observe only', ownership: 'Read-only observation', applied: 'Not applicable', appliedDetail: 'This profile has no clock-writing authority.', correction: 'Clock changes disabled by profile', note: 'Профиль Observe only даёт оценку времени без коррекции Windows.' }),
    make('ntp', 'Явный профиль NTP', { title: 'Tracking with unauthenticated NTP', tone: 'warning', description: 'This explicitly selected profile uses unauthenticated sources. Agreement does not authenticate the packets.', profile: 'NTP · unauthenticated', authentication: 'None · plain NTP', authNote: 'This is an explicit profile, never an automatic downgrade from NTS.', correction: 'Conservative NTP policy applies', correctionDetail: 'No unqualified 12-hour correction limit is presented as adopted policy.', note: 'Обычный NTP виден как неаутентифицированный профиль. В голосовании нет смеси NTS и plain NTP.' }),
    make('slewing', 'Плавная коррекция', { title: 'Gradually correcting the clock', tone: 'good', description: 'Fresh sources agree. A finite phase correction is in progress under the current policy.', offset: 12.4, uncertainty: 5.2, interval: '+7.2 to +17.6 ms', correction: '12.4 ms remains', correctionDetail: 'Remaining correction is measured against acknowledged active-time delivery; no completion time is promised.', applied: '8 s ago', appliedDetail: '+1.2 ms of active-time delivery acknowledged', note: 'Оставшаяся коррекция и подтверждённое выполнение показаны раздельно; ETA не выдумывается.' }),
    make('leap-unknown', 'Будущий leap-календарь неизвестен', { group: 'Календарь UTC', title: 'Time sync active', calendar: 'Future leap schedule unknown', calendarNote: 'Normal measurements remain usable outside protected boundaries. Missing LI announcements are not a source failure.', note: 'Неизвестное будущее календаря — уточнение к нормальной работе, а не красная авария.' }),
    make('leap-guard', 'Проверка около UTC-перехода', { group: 'Календарь UTC', title: 'Rechecking around a UTC transition', tone: 'warning', description: 'Ambiguous measurements are protected. Fresh compatible observations are needed after the transition.', next: 'Waiting for fresh post-transition agreement', agree: 0, groups: 0, agreement: 'Fresh agreement required', estimate: 'Previous model · protected interval', offset: null, uncertainty: 18.2, interval: 'Current interval withheld', historical: true, calendar: 'UTC transition guard active', calendarNote: 'Windows owns explicit leap events. Its actual event outcome has not been confirmed by this fixture.', correction: 'No predetermined one-second correction', correctionDetail: 'Only the measured residual may be corrected after fresh agreement and authority checks.', note: 'Макет не показывает прогноз события Windows как уже выполненный результат.' }),
    make('roughtime', 'Roughtime: восстановление даты', { group: 'Восстановление даты · v1', title: 'Recovering a usable date', tone: 'info', description: 'No usable retained or strict NTS certificate-time basis is available. Signed Roughtime evidence is being checked for an approximate date.', next: 'Check signed date evidence, then verify NTS in isolation', action: 'View recovery path', actionTarget: 'recovery', authentication: 'NTS not yet established', authNote: 'Roughtime signatures are date-recovery evidence, not NTS packet authentication.', agree: 0, groups: 0, agreement: 'No NTS agreement yet', estimate: 'Not yet available', applied: 'No recovery correction', appliedDetail: 'Date recovery has not changed Windows time.', offset: null, uncertainty: null, interval: 'No usable date interval yet', recovery: 'Roughtime date verification pending', recoveryNote: 'No date basis is accepted yet. Roughtime has no ordinary NTS vote or independent clock writer.', correction: 'Windows clock unchanged by recovery', correctionDetail: 'Date evidence alone grants no correction authority.', roughtimeEvidence:roughtimeEvidence('checking', {reason:'The active bootstrap is checking signed evidence because no usable NTS date basis is available.'}), note: 'Roughtime входит в v1: приблизительная дата проверяется отдельно от NTS и полномочий коррекции.' }),
    make('reserve', 'Условный резерв: дата из NTP', { group: 'Будущее восстановление', title: 'Checking a provisional date', tone: 'warning', description: 'Compatible Roughtime evidence was unavailable. The qualified future reserve would supply an unauthenticated provisional date for isolated NTS checks.', next: 'Full NTS verification and fresh agreement are still required', action: 'View recovery path', actionTarget: 'recovery', future: true, authentication: 'NTS not yet established', authNote: 'NTP reserve ancestry remains unauthenticated even if multiple samples agree.', agree: 0, groups: 0, agreement: 'No NTS agreement yet', estimate: 'Not yet available', applied: 'No recovery correction', appliedDetail: 'No preliminary global clock step.', offset: null, uncertainty: null, interval: 'Provisional date interval only', recovery: 'Unauthenticated NTP reserve', recoveryNote: 'Conditional future reserve; source policy and qualification remain open. Signed contradictions are not cleared.', correction: 'Windows clock unchanged by recovery', note: 'Будущий условный резерв Khronos/Ananke не считается обычным голосующим источником и не снимает прочие проверки сертификата.' }),
    make('recovered', 'NTS после восстановления через резерв', { group: 'Будущее восстановление', future: true, recovery: 'Unauthenticated NTP reserve', recoveryNote: 'Retained certificate-time ancestry. Current packets are NTS authenticated; recovery history remains visible.', description: 'Fresh NTS agreement is available. The earlier certificate-time basis remains recorded separately.', note: 'После успешного NTS история не стирается: аутентификация текущих пакетов не делает исходную подсказку аутентифицированной.' }),
    make('signed-conflict', 'Подписанные данные восстановления противоречат', { group: 'Восстановление даты · v1', title: 'Recovery evidence conflicts', tone: 'danger', description: 'Verified signed intervals conflict in this active Roughtime-dependent bootstrap. No usable date basis is accepted; switching to the reserve cannot erase this conflict.', next: 'Inspect the conflicting evidence; do not bypass verification', action: 'View recovery path', actionTarget: 'recovery', agree: 0, groups: 0, agreement: 'Not established', authentication: 'NTS not established', estimate: 'Not available', offset: null, uncertainty: null, interval: 'Conflicting intervals', applied: 'No recovery correction', appliedDetail: 'No clock change authorized by bootstrap evidence.', recovery: 'Signed conflict retained', recoveryNote: 'A convenient certificate-valid date or Pool agreement cannot resolve a signed contradiction.', correction: 'Clock correction withheld', correctionDetail: 'No clock change authorized in this active bootstrap session.', roughtimeEvidence:roughtimeEvidence('conflict', {reason:'Verified signatures carry contradictory intervals in this active Roughtime-dependent bootstrap session. No common usable date basis is accepted.'}), note: 'Подписанный конфликт не пропадает при таймауте или переходе к plain-NTP резерву.' }),
    make('correction', 'Проверка большой коррекции', { group: 'Защищённые действия · сцены', title: 'A clock correction needs review', tone: 'warning', description: 'Fresh NTS evidence places Windows about 6 min 12 s behind. The proposed step exceeds the current automatic correction envelope.', next: 'Review the exact proposal before administrative authorization', action: 'Review proposal', actionTarget: 'correction', offset: 372000, uncertainty: 8, interval: '+371.992 to +372.008 s', correction: '+6 min 12 s proposed · not applied', correctionDetail: 'Single-use proposal; bound to the current evidence, policy and clock generation.', applied: 'None for this proposal', appliedDetail: 'The proposed step has not changed Windows time.', displayOnly: true, note: 'Числа в Status, Sources и диалоге согласованы. Подтверждение отключено: это статический образец защищённого действия.' }),
    make('correction-expired', 'Предложение коррекции устарело', { group: 'Защищённые действия · сцены', title: 'The correction proposal expired', tone: 'warning', description: 'Its evidence, policy or clock generation is no longer current. The old proposal cannot be approved.', next: 'Fresh measurements and a new proposal are required', action: 'Review expired proposal', actionTarget: 'correction', offset: null, uncertainty: null, interval: 'Expired proposal', estimate: 'Previous proposal · expired', correction: 'Expired · not applied', applied: 'None for this proposal', appliedDetail: 'The old proposal is unusable.', displayOnly: true, historical: true, note: 'Просроченную коррекцию нельзя повторно использовать. Требуется новое предложение.' }),
    make('outcome-unknown', 'Результат изменения неизвестен', { group: 'Защищённые действия · сцены', title: 'The change result is unknown', tone: 'warning', description: 'The acknowledgement was lost. The operation may have completed; its actual result must be reconciled before any retry.', next: 'Check authoritative state; do not repeat the operation blindly', action: 'Inspect result details', actionTarget: 'result', correction: 'Outcome unknown', correctionDetail: 'A request is not proof of an applied effect. No automatic retry is offered.', applied: 'Unknown for the latest operation', appliedDetail: 'Awaiting reconciliation with the authoritative owner.', displayOnly: true, note: 'Неизвестный результат не подменяется Failed и не предлагает повторить потенциально уже выполненную операцию.' }),
    make('all-rate-limited', 'Все проверки отложены лимитами серверов', { group: 'Действия · тестовые ответы', title: 'Using the last valid estimate', tone: 'warning', description: 'All configured sources have asked for fewer requests. No current measurements are available; the previous model remains usable within its age and uncertainty budget.', next: 'Further checks are scheduled by the service', action: 'Review sources', actionTarget: 'sources', syncResponse: 'rate-limited', agree: 0, groups: 0, agreement: 'No current agreement', authentication: 'NTS · last qualified estimate', authNote: 'The retained estimate was authenticated. Control responses contain no time measurement or new authentication evidence.', estimate: '4 min ago', applied: '5 min ago', offset: -1.1, uncertainty: 12.6, interval: '−13.7 to +11.5 ms', correction: 'No new phase correction', correctionDetail: 'The previous qualified model remains usable; request limits grant no clock-writing authority.', note: 'Все четыре источника прислали RATE. Прежняя модель ещё пригодна; новые измерения не получены. Срок следующей проверки не опубликован.' }),
    make('debug-partial', 'Частичный сбор диагностических данных', { group: 'Поддержка', title: 'Time sync active', action: 'View collection status', actionTarget: 'diagnostics', note: 'Синхронизация и сбор DEBUG — разные состояния. Здесь UI-сбор доступен, сбор службы не подтверждён.', debug: true })
  ];

  // Explicit overrides prevent a healthy fixture's evidence from leaking into
  // scenarios where freshness, authentication or clock mapping is unavailable.
  const amend = (id, fields) => Object.assign(SCENARIOS.find(s => s.id === id), fields);
  const recoveryPending = (id,label,evidence,fields) => make(id,label,{
    ...clone(SCENARIOS.find(s => s.id === 'roughtime')), id, label,
    roughtimeEvidence:evidence, ...fields
  });
  const checkingIndex = SCENARIOS.findIndex(s => s.id === 'roughtime');
  SCENARIOS.splice(checkingIndex+1,0,
    recoveryPending('roughtime-date-ready','Roughtime: дата передана NTS',roughtimeEvidence('date-ready',{
      reason:'The sample evidence decision accepts this signed approximate interval as usable for isolated NTS verification. Reply counts do not define acceptance.',
      ntsHandoffState:'Date basis handed to isolated NTS verification · NTS not yet established'
    }),{title:'Verifying NTS with a signed date basis',description:'A usable signed approximate date interval was handed to isolated NTS verification. Fresh authenticated NTS agreement and correction authority are still required.',next:'Complete NTS verification and obtain fresh agreement',interval:'Approximate date basis only · no NTS estimate',recovery:'Signed Roughtime date basis',recoveryNote:'Accepted only as the date basis for isolated NTS checks; no ordinary synchronization vote or clock authority.'}),
    make('roughtime-recovered','NTS после восстановления через Roughtime',{
      group:'Восстановление даты · v1',recovery:'Signed Roughtime date basis',recoveryNote:'Retained signed date ancestry. Current packets are NTS authenticated; the earlier approximate date remains a separate fact.',
      roughtimeEvidence:roughtimeEvidence('recovered',{
        reason:'The signed approximate date basis was accepted earlier. Isolated NTS verification completed and fresh NTS agreement is now available.',
        ntsHandoffState:'Isolated NTS verification completed · fresh NTS agreement established',
        receivedAge:'Earlier in this session · sample',
        sources:ROUGHTIME_SOURCES.map(x => ({...x,signatureStatus:'Verified in sample evidence',approximateDateInterval:DATE_INTERVAL,receivedAge:'Earlier in this session · sample'})),
        clockEffect:'Roughtime supplied no clock authority. Current correction effects are reported separately by the clock broker.'
      })
    }),
    recoveryPending('roughtime-unavailable','Roughtime: пригодная дата недоступна',roughtimeEvidence('unavailable',{
      reason:'No usable signed date evidence is available in this sample. Missing replies do not establish an attack or authorize a plain-NTP downgrade.'
    }),{title:'Waiting for a usable date',tone:'warning',description:'No usable signed approximate date is available. The cause is not established. NTS verification is waiting; the system clock has not changed.',next:'Review date-recovery evidence; bounded recovery checks may continue',recovery:'No usable recovery date',recoveryNote:'Unavailability supplies neither a usable date nor evidence of malicious behavior.'}),
    recoveryPending('roughtime-rejected','Roughtime: подпись не подтверждена',roughtimeEvidence('rejected',{
      reason:'The response was rejected because its signature could not be verified. No usable date interval was accepted; this does not prove the server lied.',
      sources:[{...ROUGHTIME_SOURCES[0],signatureStatus:'Not verified · response rejected',approximateDateInterval:null,receivedAge:'12 s ago · sample'}]
    }),{title:'Signed date response rejected',tone:'warning',description:'A date response could not pass signature verification. No usable date interval was accepted, and the system clock has not changed.',next:'Inspect the rejected evidence before any NTS handoff',recovery:'No verified recovery date',recoveryNote:'An unverified signature is not proof of a malicious server. No automatic plain-NTP downgrade is authorized.'})
  );
  for (const id of ['healthy','server-rate-limited','leap-unknown','slewing','debug-partial']) amend(id,{recoveryStage:1,recoveryStageLabel:'Healthy NTS stops here'});
  amend('recovered',{recoveryStage:5,recoveryStageLabel:'NTS tracking resumed in this fixture'});
  amend('reserve',{recoveryStage:3,recoveryStageLabel:'Isolated NTS verification pending in future sample',roughtimeEvidence:roughtimeEvidence('unavailable',{scope:'future-reserve',sources:[],reason:'No compatible Roughtime date evidence was available in this future reserve sample.'})});
  amend('roughtime-recovered',{recoveryStage:5,recoveryStageLabel:'NTS tracking resumed in this fixture',routineStatus:true,simpleDescription:'Your clock is being kept in sync automatically.',simpleTrust:'Authenticated NTS · sources agree',recoveryHistoryAvailable:true});
  amend('ntp', { recovery: 'Not applicable — plain NTP', recoveryNote: 'This explicit NTP profile has no NTS certificate-time basis.' });
  amend('observe', { ownership: 'NevroTick · observation without writes', correctionDetail: 'This profile grants no clock writes and does not automatically re-enable Windows Time.' });
  amend('signed-conflict', { authNote: 'Signed bootstrap evidence conflicts; no NTS packet-authentication claim is established.' });
  amend('correction-expired', { agree: 0, groups: 0, agreement: 'No current agreement · proposal expired', authentication: 'NTS · expired evidence', authNote: 'The old authentication context does not make the expired correction proposal current.' });
  amend('resume', { applied: 'Prior generation · age unavailable', appliedDetail: 'No current-generation correction has been acknowledged.', authentication: 'NTS · prior evidence only', authNote: 'Fresh current-generation observations are still required.' });
  amend('outcome-unknown', { title: 'The clock-change result is unknown', agree: 0, groups: 0, agreement: 'Withheld pending reconciliation', authentication: 'NTS · prior evidence only', authNote: 'Prior authentication does not establish the clock mapping after an uncertain effect.', estimate: 'Withheld pending clock-state reconciliation', offset: null, uncertainty: null, interval: 'Not current after the uncertain operation', historical: true });
  for (const id of ['holdover', 'offline']) amend(id, { authentication: 'NTS · last qualified estimate', authNote: 'The retained estimate was authenticated. No fresh source agreement is currently available.' });
  for (const id of ['expired', 'service-stopped']) amend(id, { authentication: 'NTS · historical evidence', authNote: 'Previously verified evidence does not establish current tracking.' });

  // Command response facts are independent of clock-writing authority.
  for (const [id, syncResponse] of Object.entries({
    acquiring:'waiting', holdover:'waiting', expired:'waiting', offline:'waiting', conflict:'waiting',
    stale:'status-unknown', 'service-stopped':'service-stopped', resume:'waiting', slewing:'correction',
    'leap-guard':'waiting', roughtime:'waiting', 'roughtime-date-ready':'waiting', 'roughtime-unavailable':'waiting', 'roughtime-rejected':'waiting', reserve:'waiting', 'signed-conflict':'waiting',
    'correction-expired':'waiting', 'outcome-unknown':'waiting'
  })) amend(id, { syncResponse });
  amend('service-stopped', { diagnosticsResponse: { ui:'active', service:'unavailable' } });
  amend('stale', { diagnosticsResponse: { ui:'active', service:'unknown' } });
  amend('debug-partial', { diagnosticsResponse: { ui:'active', service:'refused' } });

  function deriveSources(s) {
    const sources = clone(BASE_SOURCES);
    if (['server-rate-limited','server-access-refused'].includes(s.id)) {
      const source = sources.find(x => x.id === s.affectedSource);
      Object.assign(source, s.id === 'server-rate-limited' ? {
        requestStatus: 'Checks delayed by server', controlResponse: 'RATE', lastControlResponse: '2 s ago', nextPermittedCheck: null,
        requestExplanation: 'The server asked NevroTick to send requests less often. Further checks of this source are delayed.'
      } : {
        requestStatus: 'Access refused by server', controlResponse: 'DENY', lastControlResponse: '2 s ago', nextPermittedCheck: null,
        requestExplanation: 'The server refused access. Review its usage policy or configuration.',
        role: 'Waiting', reply: '7 min ago', accepted: '7 min ago', measurementHistorical: true, measurementStatus: 'No longer usable by age',
        reason: 'The prior accepted measurement is no longer usable by age. Access refused by server; no new measurement is available.'
      });
      if (s.id === 'server-access-refused') sources.filter(x => ['alpha','gamma'].includes(x.id)).forEach(x => {
        x.role = 'Waiting';
        x.reason = 'Fresh authenticated measurement available. The published selection result is Agreement not met; no current reference has been selected.';
      });
      if (s.id === 'server-access-refused') sources.find(x => x.id === 'delta').reason = 'Authenticated measurement lies outside the compatible Alpha/Gamma interval. The published selection result is Agreement not met.';
    }
    if (s.id === 'ntp') sources.forEach(x => { x.protocol = 'NTP'; x.port = 123; x.authenticated = false; if (x.id === 'delta') x.reason = 'Unauthenticated NTP; disagrees with the selected interval.'; });
    if (['correction', 'slewing'].includes(s.id)) sources.forEach((x, i) => { x.offset = s.offset + [-1, 1, 0, 70][i]; x.uncertainty = s.uncertainty; });
    if (s.id === 'conflict') sources.forEach((x, i) => { x.offset = [-30, -29, 40, 41][i]; x.uncertainty = 2; x.role = 'Not used'; x.reason = 'Two groups of two disagree; the required majority is not met.'; x.accepted = 'Not accepted'; });
    const waiting = ['acquiring', 'resume', 'roughtime', 'roughtime-date-ready', 'roughtime-unavailable', 'roughtime-rejected', 'reserve', 'signed-conflict', 'leap-guard'];
    if (waiting.includes(s.id)) sources.forEach(x => { x.role = 'Waiting'; x.accepted = 'Not yet accepted'; x.reason = s.id === 'leap-guard' ? 'Fresh measurements required after the UTC guard.' : 'Waiting for fresh, admissible NTS measurements.'; x.offset = null; x.uncertainty = null; if (s.roughtimeEvidence.scope === 'active-bootstrap' || s.id === 'reserve') { x.authenticated = false; x.reply = 'No current NTS reply'; } });
    if (['holdover', 'offline', 'expired', 'service-stopped'].includes(s.id)) sources.forEach(x => { x.role = 'Waiting'; x.reason = 'No current reply. Historical measurements create no fresh vote.'; x.reply = 'No current reply'; x.accepted = 'Historical only'; x.offset = null; x.uncertainty = null; });
    if (s.id === 'correction-expired') sources.forEach(x => { x.role = 'Waiting'; x.offset = null; x.uncertainty = null; x.accepted = 'Expired evidence'; x.reason = 'Fresh evidence is required for a new proposal.'; });
    if (s.id === 'stale') sources.forEach(x => { x.reply = x.reply.replace(' ago', ' at last snapshot'); x.accepted = x.accepted.replace(' ago', ' at last snapshot'); });
    if (s.id === 'resume') sources.forEach(x => { x.reply = 'Prior generation · age unavailable'; x.accepted = 'No current-generation measurement'; });
    if (s.id === 'correction-expired') sources.forEach(x => { x.reply = 'Historical reply · proposal expired'; });
    if (s.id === 'outcome-unknown') sources.forEach(x => { x.role = 'Waiting'; x.offset = null; x.uncertainty = null; x.reply = 'Prior to uncertain clock effect'; x.accepted = 'Withheld pending reconciliation'; x.reason = 'The clock mapping must be reconciled before new sample admission.'; });
    sources.forEach(x => { x.lastAuthenticatedMeasurement = x.authenticated ? x.reply : 'Not established'; });
    if (s.id === 'all-rate-limited') sources.forEach(x => {
      x.role = 'Waiting'; x.offset = null; x.uncertainty = null;
      x.reply = '4 min ago'; x.accepted = x.id === 'delta' ? 'Not accepted' : '4 min ago';
      x.lastAuthenticatedMeasurement = '4 min ago'; x.measurementHistorical = true;
      x.measurementStatus = 'Historical measurement · no current vote';
      x.reason = 'Checks delayed by server. Earlier measurements do not establish current agreement.';
      x.requestStatus = 'Checks delayed by server'; x.controlResponse = 'RATE'; x.lastControlResponse = '12 s ago';
      x.nextPermittedCheck = null;
      x.requestExplanation = 'The server asked NevroTick to send requests less often. Further checks of this source are delayed.';
    });
    return sources;
  }
  SCENARIOS.forEach(s => { s.sourceHistorical = ['stale','expired','service-stopped','correction-expired','outcome-unknown','resume','leap-guard','holdover','offline','all-rate-limited'].includes(s.id); s.sources = deriveSources(s); if (s.id !== 'healthy') s.events = [
    { seq: 204, level: s.tone === 'danger' ? 'error' : s.tone === 'warning' ? 'warning' : 'info', component: 'Status', age: s.snapshot, title: s.title, detail: s.description },
    { seq: 203, level: 'notice', component: 'Time policy', age: 'Earlier in this fixture', title: s.correction, detail: s.correctionDetail },
    { seq: 202, level: 'info', component: 'Evidence', age: 'Earlier in this fixture', title: 'Certificate-time basis', detail: s.recovery + '. ' + s.recoveryNote }
  ]; });
  for (const id of ['server-rate-limited','server-access-refused']) {
    const s = SCENARIOS.find(x => x.id === id), source = s.sources.find(x => x.id === s.affectedSource);
    s.events.unshift({
      seq: 205, level: id === 'server-rate-limited' ? 'notice' : 'warning', component: 'Source requests', age: source.lastControlResponse,
      sourceId: source.id, occurrences: id === 'server-rate-limited' ? 3 : 2,
      title: source.name + (id === 'server-rate-limited' ? ' requested less frequent checks' : ' refused access'),
      detail: id === 'server-rate-limited'
        ? 'Delta (delta.example) sent RATE. Further checks of Delta are delayed; the other three sources still provide the required agreement. No time measurement or authenticated time evidence was refreshed.'
        : 'Beta (beta.example) sent DENY. Its prior measurement is separately no longer usable by age. The published selection result is Agreement not met; the previous model remains usable. No end to the refusal has been established.'
    });
  }
  function recoveryPresentation(s) {
    const e = s.roughtimeEvidence;
    const labels = {'not-needed':'Not needed',checking:'Checking signatures and date usability','date-ready':'Signed date ready · NTS verification pending',recovered:'NTS tracking active · signed date ancestry retained',unavailable:'No usable signed date available',rejected:'Signature not verified · response rejected',conflict:'Signed date intervals conflict'};
    if (e.historical) {
      const previous = {'not-needed':'No previous Roughtime recovery was requested',checking:'Previous signed-date check · completion unconfirmed','date-ready':'Previous signed-date basis · handoff recorded',recovered:'Previous NTS recovery completed · signed date ancestry retained',unavailable:'Previous recovery observation · no usable signed date',rejected:'Previous signature rejection',conflict:'Previous signed-date conflict · retained history'};
      return {label:previous[e.state] || 'Previous date-recovery observation',stage:0,stageLabel:'Current recovery stage unavailable after service operation'};
    }
    // Current NTS and recovery history are independent published facts. A past
    // Roughtime failure cannot veto an independently usable strict NTS path.
    const active = e.scope === 'active-bootstrap';
    return {label:labels[e.state] || e.state,stage:active ? (e.state === 'date-ready' ? 3 : 2) : s.recoveryStage,
      stageLabel:active ? (e.state === 'conflict' ? 'Conflicting evidence at this stage' : labels[e.state]) : s.recoveryStageLabel};
  }
  function getScenario(id) { return clone(SCENARIOS.find(s => s.id === id) || SCENARIOS[0]); }
  const FREQUENCIES = [
    { value: 'auto', label: 'Automatic', detail: 'Adapts to clock stability and network conditions' },
    { value: '120', label: 'Every 2 minutes', detail: 'Preferred interval: 120 seconds' },
    { value: '300', label: 'Every 5 minutes', detail: 'Preferred interval: 300 seconds' },
    { value: '900', label: 'Every 15 minutes', detail: 'Preferred interval: 900 seconds' }
  ];
  function validOwnMinutes(value) { return typeof value === 'string' && /^[0-9]+$/.test(value) && /[1-9]/.test(value); }
  function normalizedOwnMinutes(value) { return value.replace(/^0+/, ''); }
  function frequencyLabel(value) {
    if (typeof value === 'string' && value.startsWith('own:') && validOwnMinutes(value.slice(4))) {
      const minutes = normalizedOwnMinutes(value.slice(4));
      return `Every ${minutes} ${minutes === '1' ? 'minute' : 'minutes'}`;
    }
    return (FREQUENCIES.find(x => x.value === value) || FREQUENCIES[0]).label;
  }
  const SYNC_RESPONSES = {
    'observation-check': {label:'Checking sources · observation only',review:'Проверка источников без управления часами',detail:'Only NevroTick source measurements are requested in this sample. Windows Time remains the clock manager; no Windows resync or clock write is requested. No fresh measurement or synchronization is confirmed.'},
    checking: { label:'Checking', review:'Проверка принята', detail:'The sample request is accepted for measurement checks under the current policy. Acceptance does not establish a usable measurement or a clock change.' },
    'checking-available': { label:'Checking', review:'Проверка доступных источников', detail:'The sample request checks the other permitted sources. Checks of the affected source remain delayed by its server limit.' },
    already: { label:'Already checking', review:'Проверка уже выполняется', detail:'The repeated sample request joins the existing check. It does not start a parallel check or bypass source limits.' },
    waiting: { label:'Waiting for enough agreeing sources', review:'Ожидание достаточного согласия', detail:'The sample responses have not established the required agreement. Its rules remain unchanged; no new correction is authorized.' },
    'rate-limited': { label:'Checks delayed by server limits', review:'Все запросы отложены лимитами', detail:'All permitted sample requests are delayed by server limits. Next permitted check: Scheduled by the service; no time has been published. Control responses contain no time measurement.' },
    'source-refused': { label:'Source access refused', review:'Конкретный источник отказал', detail:'The affected source refused access. No end to the refusal has been established. Other sources are checked where permitted; a control response supplies no new time measurement.' },
    correction: { label:'Correction in progress', review:'Коррекция уже выполняется', detail:'The existing sample correction retains its current policy and authority. This request grants no new permission and does not acknowledge a clock change.' },
    'status-unknown': { label:'Status unknown', review:'Текущее состояние неизвестно', detail:'The status connection was lost. The service may still be working; command acceptance and current service state cannot be established.' },
    'service-stopped': { label:'Check unavailable', review:'Проверка недоступна: служба остановлена', detail:'A current observation confirms that the background service is stopped. Open Background service and review Start or choose a time manager. If startup fails, use Diagnostics.' }
  };
  function syncOptions(scenario) {
    const response = SYNC_RESPONSES[scenario.syncResponse] ? scenario.syncResponse : 'status-unknown';
    return ['checking','checking-available'].includes(response) ? [response,'already']
      : ['waiting','correction'].includes(response) ? [response,'checking','already'] : [response];
  }
  function logResponseOptions(context, component) {
    const scenario = context.scenario || context;
    if (component === 'service' && context.serviceControl) {
      const c = context.serviceControl;
      if (runtimePending(context) || c.observation === 'unknown') return ['unknown'];
      if (c.observation === 'stopped') return ['unavailable'];
      if (c.presentation === 'acquiring') return ['active','refused','failed'];
    }
    const response = scenario.diagnosticsResponse[component];
    return ['unavailable','unknown','refused'].includes(response) ? [response]
      : component === 'service' ? ['active','refused','failed'] : ['active','failed'];
  }
  const logRunning = log => ['active','partial','unconfirmed'].includes(log.state);
  function serviceLog(state) {
    const log = state.sampleLog, c = state.serviceControl;
    if ((runtimePending(state) || c.observation === 'unknown') && log.results.service === 'active') return {...log,state:'unconfirmed',results:{...log.results,service:'unknown'}};
    return log;
  }
  function endServiceLog(log) {
    if (!['active','unknown'].includes(log.results.service)) return log;
    return {...log,state:log.results.ui === 'active' ? 'partial' : 'expired',results:{...log.results,service:'expired'}};
  }
  // In-memory command examples. No timer, SCM, helper, UAC or clock API is called.
  const SERVICE_RESPONSES = Object.freeze({
    confirmed:'Confirmed completion', 'uac-cancelled':'Administrator prompt cancelled',
    failed:'Rejected before effect', unknown:'Acknowledgement lost · outcome unknown'
  });
  const LIFECYCLE_OPERATIONS = Object.freeze({
    'startup-on':'Turn autostart on', 'startup-off':'Turn autostart off', disable:'Disable NevroTick',
    'use-windows':'Use Windows Time', 'use-nevrotick':'Use NevroTick'
  });
  const LIFECYCLE_PREFLIGHTS = Object.freeze({
    ready:{label:'Ready · sample host',detail:'Readiness checked. No additional service changes are needed in this example.',reason:''},
    'managed-allowed':{label:'Managed policy · permitted',detail:'The effective organization policy permits this transition. Existing Windows time settings are preserved.',reason:''},
    'policy-blocked':{label:'Effective policy blocks the change',detail:'The effective Windows time policy conflicts with the requested transition.',reason:'Your organization’s current time policy does not permit this change. Contact your administrator.'},
    'dependency-unavailable':{label:'Required prerequisite unavailable',detail:'The observed Windows service configuration reports an unavailable prerequisite. No prerequisite change is authorized in this example.',reason:'A required Windows Time prerequisite is unavailable. Diagnose it before continuing.'},
    'dependency-unknown':{label:'Prerequisite readiness unknown',detail:'The sample prerequisite observation is incomplete. No service changes are proposed.',reason:'Windows Time readiness could not be confirmed. Diagnose before continuing.'},
    'ownership-uncertain':{label:'Current ownership unconfirmed',detail:'The current ownership check did not confirm a safe transfer.',reason:'Clock ownership could not be confirmed. Diagnose and reconcile before continuing.'}
  });
  const LIFECYCLE_RESPONSES = Object.freeze({
    confirmed:'Confirmed independent observations', 'uac-cancelled':'Administrator prompt cancelled',
    failed:'Rejected before effect', 'cleanup-unconfirmed':'Partial · cleanup not confirmed',
    'windows-unavailable':'Partial · Windows Time did not start', 'product-unavailable':'Partial · NevroTick did not start',
    unknown:'Acknowledgement lost · outcome unknown'
  });
  const startupPair = mode => ({measurement:mode,clock:mode});
  function timeInitial(scenario) {
    const known = !['stale','service-stopped','blocked','outcome-unknown','observe'].includes(scenario.id);
    return {startup:startupPair(scenario.id === 'stale' ? 'unknown' : 'automatic'),
      owner:known ? 'nevrotick' : 'unknown',ownerKnown:known,generation:known ? 12 : null,
      cleanup:'not-observed',restartInhibited:false,fullyDisabled:false,
      windows:{state:scenario.id === 'stale' || scenario.id === 'blocked' ? 'unknown' : 'stopped',startup:scenario.id === 'stale' || scenario.id === 'blocked' ? 'unknown' : 'disabled',restoreMode:'manual-trigger',synchronization:'unverified',policy:'Existing peer, domain and organization settings preserved'},
      phase:'idle',operation:null,kind:null,result:null,currentIssue:null,startupIssue:null,presentation:null,reconcileRequired:false};
  }
  function startupLabel(state) {
    const values = Object.values(state.timeManagement.startup);
    return values.includes('unknown') || values.some(mode => mode !== values[0]) ? 'Unknown' : values[0] === 'automatic' ? 'On' : 'Off';
  }
  function ownerLabel(state) {
    const t = state.timeManagement;
    return !t.ownerKnown ? 'Unknown · reconcile current ownership' : ({nevrotick:'NevroTick',windows:'Windows Time',none:'No active synchronizer'})[t.owner] || 'Unknown';
  }
  const mutationPending = state => state.serviceControl.phase === 'pending' || state.timeManagement.phase === 'pending';
  const runtimePending = state => state.serviceControl.phase === 'pending' || (state.timeManagement.phase === 'pending' && state.timeManagement.kind !== 'startup');
  function runtimeObservation(state) {
    const c = state.serviceControl;
    if (runtimePending(state)) return {label:'Operation pending · current health unconfirmed',short:'NevroTick operation pending',statement:'NevroTick operation is pending; last confirmed service state: ' + c.observation + '.',detail:'Last confirmed: ' + c.observation + '. Submission does not establish completion.'};
    if (c.observation === 'unknown') return {label:'Unknown · no current service observation',short:'NevroTick state unknown',statement:'Current NevroTick service state is unknown.',detail:'A disconnected status channel does not prove that services stopped.'};
    return c.observation === 'stopped'
      ? {label:'Both services stopped · confirmed sample',short:'NevroTick stopped',statement:'NevroTick background services are stopped.',detail:'Sample evidence only; no Windows service query is performed.'}
      : {label:'Both services running · sample',short:'NevroTick running',statement:'NevroTick services are running.',detail:'Sample evidence only; no Windows service query is performed.'};
  }
  function fullDisableConfirmed(state) {
    const t = state.timeManagement;
    return state.serviceControl.observation === 'stopped' && Object.values(t.startup).every(mode=>mode==='disabled') && t.restartInhibited && t.cleanup === 'confirmed' && t.ownerKnown && ['none','windows'].includes(t.owner) && !t.reconcileRequired && !mutationPending(state);
  }
  function lifecyclePermission(state, operation) {
    const t = state.timeManagement;
    if (typeof operation !== 'string' || !Object.hasOwn(LIFECYCLE_OPERATIONS,operation)) return 'Unsupported operation.';
    if (state.mode !== 'power' || state.page !== 'settings' || state.settingsSection !== 'service') return 'Open Background service in Power user.';
    if (mutationPending(state)) return 'Wait for the current request to be reconciled.';
    if (t.reconcileRequired || state.serviceControl.observation === 'unknown') return 'Current state is unknown. Diagnose and reconcile before another change.';
    if (startupLabel(state) === 'Unknown') return 'Startup configuration is unknown. Diagnose before changing it again.';
    if (operation === 'startup-on' && startupLabel(state) === 'On') return 'Autostart is already on.';
    if (operation === 'startup-off' && startupLabel(state) === 'Off') return 'Autostart is already off.';
    if (operation === 'use-windows' && t.ownerKnown && t.owner === 'windows') return 'Windows Time already manages time.';
    if (operation === 'use-nevrotick' && t.ownerKnown && t.owner === 'nevrotick') return 'NevroTick already manages time.';
    if (operation === 'disable' && fullDisableConfirmed(state)) return 'NevroTick is already fully disabled.';
    return '';
  }
  function lifecyclePreflight(state, operation) {
    if (operation?.startsWith('startup-')) return {label:'Configuration only',detail:'Both NevroTick services change startup mode together. Running services and current clock management are unaffected.',reason:''};
    const fixture = LIFECYCLE_PREFLIGHTS[state.sampleLifecyclePreflight] || LIFECYCLE_PREFLIGHTS.ready;
    if (fixture.reason) return fixture;
    if (!state.timeManagement.ownerKnown) return {...fixture,reason:'Current clock ownership is unconfirmed. Diagnose before changing time management.'};
    if (operation === 'use-nevrotick' && state.scenario.profile === 'Observe only') return {...fixture,reason:'The observe-only profile authorizes no clock control. Review time policy first.'};
    if (operation === 'use-windows' && state.timeManagement.windows.restoreMode !== 'manual-trigger') return {...fixture,reason:'The permitted Windows Time startup configuration is unconfirmed.'};
    return fixture;
  }
  function lifecycleResponseOptions(state) {
    const operation = state.timeManagement.operation;
    return ['confirmed','uac-cancelled','failed',...(!operation || operation.startsWith('startup-') ? [] : operation === 'use-nevrotick' ? ['product-unavailable'] : ['cleanup-unconfirmed',...(operation === 'use-windows' ? ['windows-unavailable'] : [])]),'unknown'];
  }
  function serviceInitial(scenario) {
    const observation = scenario.id === 'stale' ? 'unknown' : scenario.id === 'service-stopped' ? 'stopped' : 'running';
    const ownershipKnown = !['stale','service-stopped','blocked','outcome-unknown'].includes(scenario.id);
    const ownership = scenario.id === 'outcome-unknown' ? 'Unconfirmed · last known NevroTick'
      : scenario.id === 'observe' ? 'Observation only · no clock-writing authority; current time manager unconfirmed' : scenario.ownership;
    return { observation, ownershipKnown, ownership, phase:'idle', operation:null, result:null, currentIssue:null, presentation:null };
  }
  function servicePermission(state, operation) {
    const c = state.serviceControl;
    if (!['start','stop','restart'].includes(operation)) return 'Unsupported operation.';
    if (state.mode !== 'power' || state.page !== 'settings' || state.settingsSection !== 'service') return 'Open Background service in Power user.';
    if (mutationPending(state)) return 'Wait for the current operation to be reconciled.';
    if (state.timeManagement.reconcileRequired) return 'Current ownership must be reconciled before another command.';
    if (c.observation === 'unknown') return 'Service state is unknown. Diagnose before another command.';
    if (operation === 'start') return c.observation !== 'stopped' ? 'The services are already running.' : startupLabel(state) === 'Unknown' ? 'Startup configuration is unknown. Diagnose first.' : '';
    if (c.observation !== 'running') return 'The services are confirmed stopped.';
    if (!c.ownershipKnown) return 'Clock ownership is unresolved. Diagnose before cleanup or restart.';
    return '';
  }
  // A service transition supplies no new recovery observation. Keep values and
  // ancestry, annotate old observations, and never revive an active bootstrap.
  function historicalRecoveryEvidence(evidence) {
    if (evidence.state === 'not-needed') return {...evidence,historical:true};
    const previousAge = age => age ? age + ' · at previous observation before service operation; current age unavailable' : 'No received age at the previous service observation';
    return {...evidence,historical:true,scope:'retained',
      reason:'Previous observation before service operation: ' + evidence.reason,
      receivedAge:previousAge(evidence.receivedAge),
      ntsHandoffState:'Previous handoff before service operation: ' + evidence.ntsHandoffState + '. Current handoff is unconfirmed.',
      clockEffect:'Previous date-recovery observation: ' + evidence.clockEffect + '. Current clock effects require separate evidence.',
      sources:evidence.sources.map(source => ({...source,receivedAge:previousAge(source.receivedAge)}))
    };
  }
  function servicePresentedState(state) {
    const c = state.serviceControl;
    if (state.servicePresented || !c?.presentation) return state;
    const stopped = c.presentation === 'stopped', acquiring = c.presentation === 'acquiring';
    const pending = c.phase === 'pending';
    const title = stopped ? 'Background service is stopped' : acquiring ? 'Reacquiring time after service startup' : pending ? 'Service operation is pending' : 'Service operation result is unknown';
    const description = stopped ? 'The sample response confirms both NevroTick services stopped. Background synchronization is paused; Windows time may drift.' : acquiring ? 'The services are running in the sample. Fresh current-generation measurements and agreement are still required.' : pending ? 'A sample service request is pending. The last snapshot cannot establish current synchronization or completion.' : 'The command may have completed. Service state and clock ownership must be reconciled before another command.';
    const next = stopped ? 'Start the background service when ready; Windows Time was not enabled.' : acquiring ? c.ownershipKnown ? 'Wait for fresh agreement under the existing policy.' : 'Read-only startup: reconcile clock ownership before any clock write.' : 'Diagnose current state. Do not repeat an uncertain command.';
    const scenario = {
      ...state.scenario, id:stopped ? 'service-stopped' : acquiring ? 'acquiring' : 'stale',
      title, description, simpleDescription:description, simpleTrust:stopped ? 'Stopped · confirmed sample' : acquiring ? 'Fresh agreement required' : 'Current synchronization is unknown',
      next, simpleNext:next, routineStatus:false, tone:stopped ? 'danger' : 'warning',
      historical:true, sourceHistorical:true, retainedModel:false, offset:null, uncertainty:null, agree:0, groups:0,
      agreement:'No current agreement after service operation', agreementNote:'Previous source evidence is retained as history and supplies no current vote.',
      estimate:'No current qualified estimate', interval:'Unavailable until fresh agreement',
      authentication:'Historical evidence only', authNote:'Starting services establishes no new authentication or current agreement.',
      service:stopped ? 'Stopped · confirmed sample' : acquiring ? 'Running · confirmed sample' : pending ? 'Operation pending · current health unconfirmed' : 'Service health unknown',
      connection:acquiring ? 'Awaiting a fresh status snapshot' : 'No current synchronization snapshot',
      ownership:c.presentation === 'stopped' ? 'Last known before Stop: ' + c.ownership : c.ownership, correction:stopped ? 'Current clock effects unavailable' : acquiring ? 'Clock writes require current ownership and fresh authority' : 'Current correction unknown',
      correctionDetail:'Stopped services do not prove clock cleanup or adjustment readback. Last acknowledged effects are retained. Service commands grant no correction approval, budget reset or Windows Time handback.',
      snapshot:state.scenario.snapshot + ' · before service operation',
      recovery:'Previous date-recovery basis · ' + state.scenario.recovery,
      recoveryNote:'Retained history before service operation. ' + state.scenario.recoveryNote,
      recoveryStage:0,recoveryStageLabel:'Current recovery stage unavailable after service operation',
      roughtimeEvidence:historicalRecoveryEvidence(state.scenario.roughtimeEvidence),
      action:'Diagnose service', actionTarget:'diagnostics', affectedSource:null,
      sources:state.scenario.sources.map(x => ({...x,measurementHistorical:true,role:'Waiting',reason:'Historical role: ' + x.role + '. ' + x.reason + ' No current vote after the service operation.'}))
    };
    return {...state,scenario,fixtureScenarioId:state.scenario.id,sampleLog:serviceLog(state),servicePresented:true};
  }
  function presentState(state) {
    if (state.timePresented) return state;
    const qualified = !state.serviceControl.presentation && ['outcome-unknown','observe'].includes(state.scenario.id)
      ? {...state,scenario:{...state.scenario,ownership:state.serviceControl.ownership}}
      : !state.serviceControl.presentation && state.scenario.id === 'service-stopped'
        ? {...state,scenario:{...state.scenario,next:'Open Background service and review Start or choose a time manager; use Diagnostics if startup fails.'}} : state;
    const shown = servicePresentedState(qualified), t = state.timeManagement;
    if (!t || (!t.presentation && !(shown.serviceControl.presentation && t.owner === 'windows' && t.ownerKnown))) return shown;
    if (t.phase === 'pending' && t.kind !== 'startup') return shown;
    const windows = t.ownerKnown && t.owner === 'windows';
    const none = t.ownerKnown && t.owner === 'none';
    const reacquiring = t.ownerKnown && t.owner === 'nevrotick';
    const title = windows ? 'Windows Time manages time' : none ? 'No active synchronizer' : reacquiring ? 'Reacquiring time with NevroTick' : 'Time management is unconfirmed';
    const description = windows ? 'Windows Time is the independently confirmed clock manager. Windows synchronization is not yet verified.' : none ? 'Automatic synchronization is off. Your clock may drift until you explicitly choose a time manager.' : reacquiring ? 'NevroTick is the confirmed time manager. Fresh measurements and agreement are still required.' : 'Some actions may have completed. Clock cleanup and current ownership must be reconciled before another change.';
    const next = windows ? state.serviceControl.observation === 'running' ? 'NevroTick is observing only. Source checks do not resynchronize Windows.' : 'Windows time settings are preserved. Diagnose to review synchronization evidence.' : none ? 'Choose Use NevroTick or Use Windows Time in Background service.' : reacquiring ? 'Wait for fresh agreement; earlier measurements supply no current votes.' : 'Diagnose current state. Do not repeat an uncertain operation.';
    // The existing service presentation applies the shared history rules exactly once.
    const history = shown.servicePresented ? shown : servicePresentedState({...state,serviceControl:{...state.serviceControl,presentation:'acquiring'}});
    const scenario = {...history.scenario,id:windows ? 'windows-managed' : none ? 'no-synchronizer' : reacquiring ? 'acquiring' : 'ownership-unknown',
      title,description,simpleDescription:description,simpleTrust:windows ? 'Windows synchronization not yet verified' : none ? 'Automatic synchronization is off' : reacquiring ? 'Fresh agreement required' : 'Current owner unknown',
      next,simpleNext:next,tone:windows ? 'info' : 'warning',ownership:ownerLabel(state),
      service:runtimeObservation(state).short + ' · sample observation',
      correction:windows ? 'Windows Time manages the clock; NevroTick clock writes are blocked' : none ? 'No active automatic synchronization' : reacquiring ? 'Waiting for fresh current-generation authority' : 'Clock cleanup and effects unconfirmed',
      correctionDetail:'Clock ownership, running services and fresh synchronization evidence are separate observations.',
      authentication:'Historical NevroTick evidence only',action:'Manage time',actionTarget:'service-settings',routineStatus:false,
      offset:null,uncertainty:null,agree:0,groups:0,retainedModel:false};
    return {...history,scenario,sampleLog:serviceLog(state),timePresented:true};
  }
  // Local Windows notification UX only. These are published sample observations,
  // not timers, OS permissions, service commands or proof of toast delivery.
  const NOTIFICATION_OS = Object.freeze({allowed:'Allowed · sample', 'blocked-app':'Blocked in Windows app settings · sample', 'blocked-policy':'Blocked by organization policy · sample', suppressed:'Banner suppressed / Do Not Disturb · sample', unknown:'Windows availability unverified · sample', unsupported:'Notifications unavailable · sample'});
  const NOTIFICATION_PRESENTERS = Object.freeze({active:'App or tray is running · sample', missing:'App and tray are not running · sample'});
  const notificationPolicy = (key,title,detail,target='diagnostics',persistent=false,severity='important') => ({key,title,detail,target,persistent,severity});
  const NOTIFICATION_POLICIES = Object.freeze({
    healthy:null, 'server-rate-limited':null,
    'server-access-refused':notificationPolicy('source-refused','A time source refused access','Review the source configuration. Other permitted checks continue; synchronization has not been newly confirmed.','sources',true,'attention'),
    acquiring:null,
    holdover:notificationPolicy('estimate-degraded','Fresh time checks are unavailable','NevroTick is using its last valid estimate. Review the current status before that estimate becomes unusable.','diagnostics',true,'attention'),
    expired:notificationPolicy('estimate-unusable','No usable time estimate','Automatic clock corrections are withheld. Review network access and the configured time sources.'),
    offline:notificationPolicy('sources-unavailable','Time sources are not replying','Automatic retries continue. Review the connection and current time status.','diagnostics',true,'attention'),
    conflict:notificationPolicy('source-conflict','Time sources disagree','Automatic clock corrections are paused. Open NevroTick to compare the sources.','statistics'),
    stale:notificationPolicy('status-unavailable','Current time status is unavailable','NevroTick cannot confirm the current service status. The service may still be working.','diagnostics',true,'attention'),
    'service-stopped':notificationPolicy('unexpected-service-stop','Background synchronization is stopped','Open NevroTick to review the stopped service and choose the next action.','service'),
    blocked:notificationPolicy('clock-owner-conflict','Clock control needs attention','Another time controller is active. Review ownership before making any change.','service'),
    resume:null, observe:null, ntp:null, slewing:null, 'leap-unknown':null,
    'leap-guard':notificationPolicy('utc-recheck','Time checks need attention','A UTC-transition recheck remains unresolved. Review the current evidence; do not force a clock correction.','diagnostics',true,'attention'),
    roughtime:null, 'roughtime-date-ready':null, 'roughtime-recovered':null,
    'roughtime-unavailable':notificationPolicy('date-unavailable','A usable date is still unavailable','Secure time verification is waiting for a usable date basis. Review date recovery.','recovery',true,'attention'),
    'roughtime-rejected':notificationPolicy('date-rejected','Date-recovery evidence needs review','An unverifiable signed-date response was rejected. Review the evidence; no clock change was made.','recovery',true,'attention'),
    reserve:null, recovered:null,
    'signed-conflict':notificationPolicy('signed-date-conflict','Signed date evidence conflicts','Date recovery is blocked by conflicting verified evidence. Review it without bypassing verification.','recovery'),
    correction:notificationPolicy('correction-review','A clock correction needs review','Open NevroTick to review the current proposal. This notification does not apply or approve a correction.','correction'),
    'correction-expired':null,
    'outcome-unknown':notificationPolicy('clock-outcome-unknown','Clock change outcome is unknown','The change may have completed. Review current clock state; do not repeat the operation blindly.'),
    'all-rate-limited':notificationPolicy('checks-delayed','Time checks remain delayed','Server limits delay all permitted checks. The last valid estimate is retained; limits must not be bypassed.','diagnostics',true,'attention'),
    'debug-partial':null
  });
  function notificationInitial(nextId = 1) {
    return {enabled:true,os:'allowed',presenter:'active',persistent:false,nextId,requestSerial:0,episode:0,incident:null,announced:false,current:null,items:[],feedback:''};
  }
  function notificationCandidate(state) {
    const t = state.timeManagement, c = state.serviceControl;
    if (runtimePending(state)) return null;
    const issue = t.currentIssue;
    if (issue && ['cleanup-unconfirmed','windows-unavailable','product-unavailable'].includes(issue.code)) {
      const unresolved = issue.code === 'cleanup-unconfirmed' ? t.cleanup === 'unconfirmed' && !t.ownerKnown : t.ownerKnown && t.owner === 'none';
      if (unresolved) {
        const detail = issue.code === 'cleanup-unconfirmed' ? 'Clock cleanup is unconfirmed. A second time manager must not be activated before ownership is reconciled.' : 'No active synchronizer is confirmed after an incomplete handover. Open NevroTick to review the current services and choose the next action.';
        return {...notificationPolicy('handover-' + issue.code,'Time management change is incomplete',detail,'service'),scopeSerial:issue.scopeSerial};
      }
    }
    if (issue?.code === 'unknown' && (!t.ownerKnown || t.reconcileRequired)) return {...notificationPolicy('management-outcome-unknown','Time management outcome is unknown','Some actions may have completed. Diagnose current ownership before another change.','service'),scopeSerial:issue.scopeSerial};
    if (c.currentIssue?.code === 'unknown' && c.observation === 'unknown') return {...notificationPolicy('service-outcome-unknown','NevroTick service outcome is unknown',t.ownerKnown && t.owner === 'windows' ? 'NevroTick service state needs reconciliation. Windows Time remains the independently confirmed manager; Windows synchronization is unverified.' : 'The service command may have completed. Diagnose service state and clock ownership before another command.','service'),scopeSerial:c.currentIssue.scopeSerial};
    const shown = presentState(state).scenario;
    let candidate = NOTIFICATION_POLICIES[shown.id] || null;
    if (c.presentation || t.presentation) candidate = null;
    if (candidate && shown.id === 'correction' && (shown.proposalExpired || shown.historical)) candidate = null;
    if (candidate && shown.id === 'signed-conflict' && (shown.roughtimeEvidence.historical || shown.roughtimeEvidence.scope !== 'active-bootstrap')) candidate = null;
    // A configuration problem does not hide an independent, important clock issue.
    if (candidate?.severity === 'important') return {...candidate};
    if (t.startupIssue && startupLabel(state) === 'Unknown') return {...notificationPolicy('autostart-outcome-unknown','Autostart change is unconfirmed','The new startup configuration could not be confirmed. Current timekeeping is unaffected by this configuration-only request.','service',false,'attention'),scopeSerial:t.startupIssue.scopeSerial};
    // Planned Stop/Disable and successful transitions are not failures. Presented
    // evidence from before those operations must never become a new incident.
    if (c.presentation || t.presentation) return null;
    return candidate ? {...candidate} : null;
  }
  function notificationAvailability(state) {
    const n = state.notifications;
    if (!n.enabled) return 'Problem notifications are off in NevroTick.';
    if (n.presenter !== 'active') return 'NevroTick and its tray observer are not running. New local alerts are unavailable.';
    return ({allowed:'Windows may silence banners or keep notifications in Notification Center.', 'blocked-app':'Windows blocks this app’s notifications. Review Windows notification settings.', 'blocked-policy':'Organization policy blocks notifications. The app cannot override it.', suppressed:'Windows may suppress the banner or keep an item in Notification Center. A visible banner is not confirmed.', unknown:'Windows notification availability has not been checked. New alerts cannot be confirmed.', unsupported:'Windows notifications are unavailable in this sample.'})[n.os];
  }
  function notificationContext(state, candidate) {
    const generation = state.timeManagement.generation ?? 'unconfirmed';
    const operation = /^(service-|handover-|management-|autostart-)/.test(candidate.key) ? ':request-' + (candidate.scopeSerial ?? state.notifications.requestSerial) : '';
    const proposal = candidate.key === 'correction-review' && typeof state.scenario.proposalId === 'string' ? ':proposal-' + state.scenario.proposalId.slice(0,80) : '';
    return candidate.key + ':' + candidate.severity + ':generation-' + generation + operation + proposal;
  }
  function publishNotification(state, test = false) {
    const n = state.notifications, candidate = test ? notificationPolicy('test','Test notification','This is a notification preview. It does not verify clock synchronization.','notifications',false,'test') : notificationCandidate(state);
    const key = candidate ? notificationContext(state,candidate) : null;
    if (!candidate) {
      const shown = presentState(state).scenario;
      const verifiedTracking = !runtimePending(state) && !state.serviceControl.presentation && !state.timeManagement.presentation && state.timeManagement.ownerKnown && state.timeManagement.owner === 'nevrotick' && !shown.historical && shown.agree >= shown.minimum && ['healthy','server-rate-limited','slewing','leap-unknown','roughtime-recovered','recovered','debug-partial'].includes(shown.id);
      // Pending, Running or a cancelled command is not evidence that an earlier
      // incident ended. Only a fresh tracking fixture or a new explicit scope can
      // retire it; otherwise a temporary observation gap would cause spam.
      return {...state,notifications:{...n,...(verifiedTracking ? {incident:null,announced:false} : {}),current:null,feedback:'No new problem notification is needed for this observation. The in-app status is unchanged.'}};
    }
    const next = test ? {...n} : {...n,incident:key,episode:key === n.incident ? n.episode : n.episode + 1,announced:key === n.incident && n.announced};
    if (!test && candidate.persistent && !n.persistent) return {...state,notifications:{...next,current:null,feedback:'The problem remains visible in NevroTick. A notification waits for a published persistent-problem observation; no timeout is invented here.'}};
    if (!test && next.announced) return {...state,notifications:{...next,feedback:'The same incident was already previewed. No duplicate notification was created; dismissing it does not resolve the problem.'}};
    if (!n.enabled || n.os !== 'allowed' || n.presenter !== 'active') return {...state,notifications:{...next,current:null,feedback:'No notification preview shown. ' + notificationAvailability(state)}};
    if (!Number.isSafeInteger(n.nextId) || n.nextId < 1 || n.nextId >= Number.MAX_SAFE_INTEGER) return {...state,notifications:{...next,current:null,feedback:'No notification preview created: sample identity is unavailable.'}};
    const item = {...candidate,id:'nt-' + n.nextId,test,context:key,episode:next.episode};
    return {...state,notifications:{...next,nextId:n.nextId + 1,announced:test ? next.announced : true,current:item,items:n.items.concat(item).slice(-8),feedback:test ? 'Test notification preview only. No system notification was sent and clock health was not checked.' : 'Problem notification preview only. No system notification was sent.'}};
  }
  function notificationIsCurrent(state, item) {
    if (!item) return false;
    if (item.test) return true;
    const current = notificationCandidate(state);
    return !!current && notificationContext(state,current) === item.context && item.episode === state.notifications.episode;
  }
  function openNotification(state, id) {
    if (typeof id !== 'string' || !/^nt-[1-9][0-9]{0,15}$/.test(id)) return state;
    const item = state.notifications.items.find(entry => entry.id === id);
    if (!item) return state;
    const next = {...state,notifications:{...state.notifications,current:state.notifications.current?.id === id ? null : state.notifications.current}};
    if (!notificationIsCurrent(state,item)) return {...reduce(next,{type:'page',page:'status',confirmed:true}),notice:'This notification belongs to an earlier observation. Review the current status; no operation was repeated.'};
    if (item.test) return {...reduce(next,{type:'notification-settings',confirmed:true}),notice:'Test notification opened. It does not confirm synchronization or Windows delivery.'};
    if (item.target === 'correction') return reduce(next,{type:'modal',modal:'correction',confirmed:true});
    if (item.target === 'service') return reduce(next,{type:'service-settings',confirmed:true});
    return reduce(next,{type:'power-page',page:['sources','statistics','recovery','diagnostics'].includes(item.target) ? item.target : 'status',confirmed:true});
  }
  function trayPresentation(state) {
    const shown = presentState(state), s = shown.scenario, t = state.timeManagement;
    const windows = t.ownerKnown && t.owner === 'windows';
    let kind = ({healthy:'tracking','server-rate-limited':'tracking','server-access-refused':'attention',acquiring:'checking',holdover:'attention',expired:'blocked',offline:'attention',conflict:'blocked',stale:'unknown','service-stopped':'paused',blocked:'blocked',resume:'checking',observe:'observing',ntp:'attention',slewing:'tracking','leap-unknown':'tracking','leap-guard':'attention',roughtime:'checking','roughtime-date-ready':'checking','roughtime-recovered':'tracking','roughtime-unavailable':'attention','roughtime-rejected':'attention',reserve:'attention',recovered:'tracking','signed-conflict':'blocked',correction:'attention','correction-expired':'attention','outcome-unknown':'unknown','all-rate-limited':'attention','debug-partial':'tracking'})[s.id] || 'unknown';
    let label = s.title;
    let detail = s.simpleTrust || s.authentication;
    if (runtimePending(state)) { kind='checking';label='Operation pending';detail='Current service health is unconfirmed. Last confirmed: '+state.serviceControl.observation+'.'+(windows?' Windows Time remains the confirmed manager; Windows synchronization is not yet verified.':''); }
    else if (windows) { kind='windows';label='Windows Time manages time';detail='Windows synchronization is not yet verified. '+runtimeObservation(state).short+'.'; }
    else if (state.serviceControl.observation === 'unknown' || t.reconcileRequired || t.currentIssue?.code === 'cleanup-unconfirmed') { kind='unknown';label='Current state is unconfirmed';detail='Diagnose services and clock ownership before another change.'; }
    else if (t.ownerKnown && t.owner === 'none' && ['windows-unavailable','product-unavailable'].includes(t.currentIssue?.code)) { kind='attention';label='No active synchronizer';detail='The time management change is incomplete. Open Background service to review the next action.'; }
    else if (fullDisableConfirmed(state)) { kind='paused';label='NevroTick is disabled';detail='NevroTick services are stopped and startup is disabled. No automatic synchronization is confirmed.'; }
    else if (state.serviceControl.observation === 'stopped') { kind='paused';label='NevroTick services are stopped';detail='No NevroTick background synchronization is running. Review the current time manager.'; }
    if (s.connection === 'Disconnected' && !windows && !runtimePending(state)) { kind='unknown';label='Current status unavailable';detail='The retained snapshot does not establish current synchronization.'; }
    const styles = {tracking:['good','✓'],checking:['info','…'],attention:['warning','!'],blocked:['danger','×'],unknown:['neutral','?'],paused:['neutral','Ⅱ'],observing:['info','○'],windows:['info','W']};
    const [tone,symbol] = styles[kind];
    const stopped = state.serviceControl.observation === 'stopped' && !runtimePending(state);
    const checkEnabled = !runtimePending(state) && state.serviceControl.observation !== 'unknown';
    const checkLabel = stopped ? 'Time management…' : windows || ['observe','blocked','outcome-unknown'].includes(s.id) || !checkEnabled ? 'Check sources' : 'Sync now';
    const checkReason = checkEnabled ? '' : 'Current NevroTick service state is unconfirmed. Open Diagnostics before requesting another check.';
    const freshness = s.connection === 'Disconnected' || s.historical ? 'Last snapshot: '+s.snapshot+'.' : 'Snapshot: '+s.snapshot+'.';
    const startup = state.timeManagement.kind === 'startup' && state.timeManagement.phase === 'pending' ? ' Startup change pending; current timekeeping is independent.' : startupLabel(state) === 'Unknown' ? ' Startup configuration is unconfirmed.' : '';
    return {kind,tone,symbol,label,detail,tooltip:'NevroTick — '+label+'. '+detail+' '+freshness+startup,checkLabel,checkEnabled,checkReason,checkAction:stopped?'service-settings':'sync-now'};
  }
  function initialState(id = 'healthy') {
    const scenario = getScenario(id);
    return { scenario, notifications:notificationInitial(), timeManagement:timeInitial(scenario), sampleLifecyclePreflight:'ready', sampleLifecycleResponse:'confirmed', serviceControl:serviceInitial(scenario), sampleServiceResponse:'confirmed', serviceDiagnosis:false, mode: 'simple', page: 'status', powerPage: 'status', modal: null, sourceFilter: 'all', sourceQuery: '', eventFilter: 'all', drafts: [], selectedSource: null, editorReturnToDetails: false, policyDraft: null, recoveryDraft: null, recoverySavedInput:null, recoveryEditBaseline:null, returnContext:null, leaveGuard:null, recoveryInput: {count:'0',helpers:['','']}, recoveryErrors: {}, settingsSection: 'schedule', editorError: {}, notice: '', compact: false, reviewerOpen: false, frequencyPreview: 'auto', frequencyDraft: 'auto', frequencyCustom: '300', frequencyOwn: '1', frequencyOwnDraft: '1', frequencyError: '', sampleSync: scenario.syncResponse, syncFeedback: null, sampleSyncStarted: false, sampleTray: false, sampleSignIn: false, trayMenuOpen:false, sampleLogResponse: clone(scenario.diagnosticsResponse), sampleLog: { scope: 'ui', duration: '5', state: 'idle', stopRequested:false, results: { ui:'not-requested', service:'not-requested' } } };
  }
  function escapeHtml(value) { return String(value ?? '').replace(/[&<>"']/g, c => ({ '&': '&amp;', '<': '&lt;', '>': '&gt;', '"': '&quot;', "'": '&#39;' }[c])); }
  function signed(value, digits = 1) { if (value === null || !Number.isFinite(value)) return '—'; const n = Math.abs(value).toFixed(digits); return (value < 0 ? '−' : value > 0 ? '+' : '') + n; }
  function offsetLabel(value) { if (value === null) return { value: '—', unit: '', description: 'No current usable estimate' }; if (Math.abs(value) >= 60000) return { value: signed(value / 1000, 0), unit: 's', description: value > 0 ? 'Windows is behind the reference' : 'Windows is ahead of the reference' }; return { value: signed(value), unit: 'ms', description: value > 0 ? 'Windows is behind the reference' : value < 0 ? 'Windows is ahead of the reference' : 'Aligned with the estimated reference' }; }
  function validHost(raw) {
    if (typeof raw !== 'string' || !raw || raw.length > 253 || raw !== raw.trim() || /[^\x21-\x7e]/.test(raw) || /[\s/@?#\\]/.test(raw)) return false;
    if (/^\d+\.\d+\.\d+\.\d+$/.test(raw)) return raw.split('.').every(n => /^\d{1,3}$/.test(n) && Number(n) <= 255 && (n === '0' || !n.startsWith('0')));
    if (raw.includes(':')) {
      if (!/^[0-9a-f:]+$/i.test(raw) || (raw.match(/::/g) || []).length > 1 || raw.includes(':::')) return false;
      if ((raw.startsWith(':') && !raw.startsWith('::')) || (raw.endsWith(':') && !raw.endsWith('::'))) return false;
      const p = raw.split(':').filter(Boolean); if (p.some(x => x.length > 4)) return false;
      return raw.includes('::') ? p.length < 8 : p.length === 8 && !raw.startsWith(':') && !raw.endsWith(':');
    }
    const host = raw.endsWith('.') ? raw.slice(0, -1) : raw;
    return host.length > 0 && host.split('.').every(label => /^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$/i.test(label));
  }
  function validateSource(raw, existing, excludeId) {
    const errors = {};
    const name = typeof raw.name === 'string' ? raw.name : '';
    const host = typeof raw.host === 'string' ? raw.host : '';
    const protocol = raw.protocol;
    const portText = typeof raw.port === 'string' ? raw.port : String(raw.port ?? '');
    if (!/^[\x20-\x7e]{1,64}$/.test(name) || name !== name.trim()) errors.name = 'Use 1–64 printable ASCII characters, without leading or trailing spaces.';
    if (!validHost(host)) errors.host = 'Enter a hostname or IP address, without a URL, path or spaces.';
    if (!['NTS', 'NTP'].includes(protocol)) errors.protocol = 'Choose NTS or NTP.';
    if (!/^\d{1,5}$/.test(portText) || Number(portText) < 1 || Number(portText) > 65535) errors.port = 'Use a whole-number port from 1 to 65535.';
    const canonical = host.toLowerCase().replace(/\.$/, '');
    if (!errors.host && existing.some(x => x.id !== excludeId && x.host.toLowerCase().replace(/\.$/, '') === canonical && x.protocol === protocol)) errors.host = 'This protocol and server identity are already in the preview.';
    if (Object.keys(errors).length) return { ok: false, errors };
    return { ok: true, value: { name, host: canonical, protocol, port: Number(portText) } };
  }
  function addDraft(state, raw, editId) {
    const all = state.scenario.sources.concat(state.drafts.filter(x => x.kind !== 'remove'));
    const result = validateSource(raw, all, editId);
    if (!result.ok) return { ...state, editorError: result.errors };
    const source = state.scenario.sources.find(x => x.id === editId);
    const id = editId || 'draft-' + String(1 + state.drafts.reduce((n, x) => Math.max(n, Number(x.id.replace('draft-', '')) || 0), 0));
    const draft = { ...result.value, id, kind: source ? 'edit' : 'add', role: 'Draft', operator: 'Not established', authenticated: false, reason: 'Preview only. This entry does not participate in the active source set.' };
    return { ...state, drafts: state.drafts.filter(x => x.id !== id).concat(draft), editorError: {}, modal: null, notice: 'Preview updated. The active fixture and service configuration are unchanged.' };
  }
  function removeDraftSource(state, id) {
    const source = state.scenario.sources.find(x => x.id === id);
    const rest = state.drafts.filter(x => x.id !== id);
    return { ...state, drafts: source ? rest.concat({ ...source, kind: 'remove' }) : rest, modal: null, notice: source ? 'Removal added to preview. The active fixture is unchanged.' : 'Draft removed.' };
  }
  const helperInput = value => ({ count:value?.count || '0', helpers:[value?.helpers?.[0] || '',value?.helpers?.[1] || ''] });
  function recoveryEdited(state) {
    return state.modal === 'recovery-editor' && JSON.stringify(state.recoveryInput) !== JSON.stringify(state.recoveryEditBaseline || helperInput(state.recoverySavedInput || state.recoveryDraft));
  }
  function discardedByReset(state, full) {
    const collectionPending = logRunning(state.sampleLog) || Object.values(state.sampleLog.results).some(value => ['active','unknown'].includes(value));
    return !!(collectionPending || state.drafts.length || state.policyDraft || state.recoveryDraft || state.recoverySavedInput || recoveryEdited(state) || state.serviceControl.phase === 'pending' || state.timeManagement.phase === 'pending' || state.timeManagement.result || state.serviceControl.result || state.notifications.items.length || state.frequencyDraft !== state.frequencyPreview || (full && (state.frequencyPreview !== 'auto' || !state.notifications.enabled)));
  }
  function enterRecovery(state, context) {
    const returnContext = state.returnContext || (state.page === 'recovery' ? null : {
      mode:state.mode, page:state.page, settingsSection:state.settingsSection,
      sourceFilter:state.sourceFilter, sourceQuery:state.sourceQuery, eventFilter:state.eventFilter,
      sourceDialog:state.modal === 'source-details' ? state.selectedSource : null,
      ...(context || {})
    });
    return { ...state, mode:'power', page:'recovery', powerPage:'recovery', modal:null, returnContext, notice:'' };
  }
  function endRecovery(state) {
    return { ...state, returnContext:null, powerPage:state.powerPage === 'recovery' ? 'status' : state.powerPage };
  }
  function reduce(state, action) {
    const trayActions = ['tray-menu-open','tray-menu-toggle','tray-menu-close','tray-preview-enable','tray-observer-open','tray-open-page','tray-check','sample-tray-quit'];
    if (state.modal && state.modal !== 'tray-sample' && trayActions.includes(action.type)) return state;
    if (state.trayMenuOpen && !trayActions.includes(action.type) && !['sample-tray-quit-confirm','tray-quit-manage'].includes(action.type)) return reduce({...state,trayMenuOpen:false},action);
    if (state.leaveGuard) {
      const guard = state.leaveGuard;
      if (['close','recovery-stay','cancel-reset'].includes(action.type)) return { ...state, leaveGuard:null, modal:guard.previousModal };
      if (guard.kind === 'recovery-editor' && ['recovery-keep-leave','recovery-discard-edits'].includes(action.type)) {
        const keep = action.type === 'recovery-keep-leave';
        const recoveryInput = keep ? state.recoveryInput : clone(state.recoveryEditBaseline || helperInput(state.recoverySavedInput || state.recoveryDraft));
        return reduce({ ...state, leaveGuard:null, recoveryInput, recoverySavedInput:keep ? clone(recoveryInput) : state.recoverySavedInput, recoveryErrors:{} }, { ...guard.action, confirmed:true });
      }
      if (guard.kind === 'reset' && action.type === 'confirm-reset') return reduce({ ...state, leaveGuard:null }, { ...guard.action, confirmed:true });
      return state;
    }
    if (!action.confirmed && ['scenario','reset'].includes(action.type) && discardedByReset(state,action.type === 'reset')) {
      return { ...state, modal:'reset-confirm', leaveGuard:{kind:'reset',action,previousModal:state.modal} };
    }
    if (!action.confirmed && recoveryEdited(state) && ['close','recovery-cancel','page','power-page','policy-page','recovery-back','modal','notification-open','notification-settings'].includes(action.type)) {
      return { ...state, leaveGuard:{kind:'recovery-editor',action,previousModal:state.modal} };
    }
    switch (action.type) {
      case 'scenario': return { ...initialState(action.id), notifications:{...notificationInitial(state.notifications.nextId),enabled:state.notifications.enabled}, compact: state.compact, mode: state.mode, reviewerOpen: state.reviewerOpen, frequencyPreview: state.frequencyPreview, frequencyDraft: state.frequencyPreview, frequencyCustom: state.frequencyCustom, frequencyOwn: state.frequencyOwn, frequencyOwnDraft: state.frequencyOwn, sampleTray: state.sampleTray, sampleSignIn: state.sampleSignIn };
      case 'reset': return { ...initialState(state.scenario.id), notifications:notificationInitial(state.notifications.nextId), mode: state.mode, compact: state.compact, reviewerOpen: state.reviewerOpen };
      case 'view-mode': {
        if (!['simple','power'].includes(action.mode) || action.mode === state.mode || state.modal) return state;
        if (action.mode === 'simple') return { ...state, mode: 'simple', powerPage: state.page, page: ['status','settings','about'].includes(state.page) ? state.page : 'status', notice: '' };
        return { ...state, mode: 'power', page: ['settings','about'].includes(state.page) ? state.page : state.powerPage, notice: '' };
      }
      case 'power-page': {
        if (action.page === 'recovery') return enterRecovery(state,action.context);
        const page = ['status','sources','statistics','diagnostics','settings','recovery','about'].includes(action.page) ? action.page : 'status';
        return { ...endRecovery(state), mode: 'power', page, powerPage: page, modal: null, notice: '' };
      }
      case 'affected-source': {
        const id = action.id || state.scenario.affectedSource;
        if (!state.scenario.sources.some(x => x.id === id)) return state;
        const hidden = !visibleSources(state).some(x => x.id === id);
        return { ...endRecovery(state), mode: 'power', page: 'sources', powerPage: 'sources', modal: 'source-details', selectedSource: id, highlightedSource: id,
          sourceFilter: hidden ? 'all' : state.sourceFilter, sourceQuery: hidden ? '' : state.sourceQuery,
          notice: hidden ? 'Filters cleared to show the affected source' : '', editorError: {}, editorReturnToDetails: false };
      }
      case 'policy-page': return { ...endRecovery(state), mode: 'power', page: 'settings', powerPage: 'settings', settingsSection: 'policy', modal: null, notice: '' };
      case 'page': {
        if (action.page === 'recovery') return enterRecovery(state,action.context);
        const allowed = state.mode === 'simple' ? ['status','settings','about'] : ['status','sources','statistics','diagnostics','settings','recovery','about'];
        const page = allowed.includes(action.page) ? action.page : 'status';
        return { ...endRecovery(state), page, powerPage: state.mode === 'power' || state.returnContext ? page : state.powerPage, modal: null, notice: '' };
      }
      case 'recovery-back': {
        const context = state.returnContext || {mode:state.mode,page:'status'};
        const sourceAvailable = context.sourceDialog && state.scenario.sources.some(x => x.id === context.sourceDialog);
        return { ...state, mode:context.mode, page:context.sourceDialog && !sourceAvailable ? 'sources' : context.page,
          powerPage:context.sourceDialog && !sourceAvailable ? 'sources' : context.mode === 'power' ? context.page : 'status', settingsSection:context.settingsSection || state.settingsSection,
          sourceFilter:context.sourceFilter ?? state.sourceFilter, sourceQuery:context.sourceQuery ?? state.sourceQuery, eventFilter:context.eventFilter ?? state.eventFilter,
          modal:sourceAvailable ? 'source-details' : null, selectedSource:sourceAvailable ? context.sourceDialog : null, returnContext:null, notice:'' };
      }
      case 'frequency-draft': {
        const own = typeof action.value === 'string' && action.value.startsWith('own:');
        return FREQUENCIES.some(x => x.value === action.value) || own
          ? { ...state, frequencyDraft: action.value, frequencyCustom: action.value === 'auto' ? state.frequencyCustom : action.value, frequencyOwnDraft: own ? action.value.slice(4) : state.frequencyOwnDraft, frequencyError: '', notice: '' }
          : { ...state, frequencyError: 'Choose Automatic, a suggested interval or your own interval.' };
      }
      case 'frequency-save': {
        const own = state.frequencyDraft.startsWith('own:');
        if (own && !validOwnMinutes(state.frequencyDraft.slice(4))) return { ...state, frequencyError: 'Enter a whole number of minutes of at least 1.' };
        if (!own && !FREQUENCIES.some(x => x.value === state.frequencyDraft)) return { ...state, frequencyError: 'Choose a valid check interval.' };
        const value = own ? 'own:' + normalizedOwnMinutes(state.frequencyDraft.slice(4)) : state.frequencyDraft;
        return { ...state, frequencyDraft: value, frequencyPreview: value, frequencyCustom: own ? value : state.frequencyCustom, frequencyOwn: own ? value.slice(4) : state.frequencyOwn, frequencyOwnDraft: own ? value.slice(4) : state.frequencyOwnDraft, frequencyError: '', notice: 'Preview updated: ' + frequencyLabel(value) + '.' };
      }
      case 'frequency-cancel': return { ...state, frequencyDraft: state.frequencyPreview, frequencyCustom: state.frequencyPreview === 'auto' ? (state.frequencyCustom.startsWith('own:') ? 'own:' + state.frequencyOwn : state.frequencyCustom) : state.frequencyPreview, frequencyOwnDraft: state.frequencyOwn, frequencyError: '', notice: '' };
      case 'policy-draft': return ['nts','ntp','observe'].includes(action.value) ? { ...state, policyDraft: action.value, modal: 'policy-review', notice: '' } : state;
      case 'policy-discard': return { ...state, policyDraft: null, modal: null, notice: 'Profile preview discarded. The active fixture is unchanged.' };
      case 'recovery-edit': {
        const recoveryInput = helperInput(state.recoverySavedInput || state.recoveryDraft);
        return { ...state, modal: 'recovery-editor', recoveryInput, recoveryEditBaseline:clone(recoveryInput), recoveryErrors: {}, notice: '' };
      }
      case 'recovery-input': return { ...state, recoveryInput:helperInput(action), recoveryErrors:{} };
      case 'recovery-cancel': return { ...state, modal:null, recoveryErrors:{} };
      case 'recovery-preview': {
        const count = action.count;
        const helpers = Array.isArray(action.helpers) ? action.helpers.slice(0,2).map(x => typeof x === 'string' ? x : '') : [];
        while (helpers.length < 2) helpers.push('');
        const errors = {};
        if (!['0','1','2'].includes(count)) errors.count = 'Choose off, one or two helpers.';
        const used = count === '1' ? 1 : count === '2' ? 2 : 0;
        for (let i=0;i<used;i++) if (!validHost(helpers[i])) errors['helper'+(i+1)] = 'Enter a hostname or IP address without a URL, path or spaces.';
        if (used === 2 && !errors.helper1 && !errors.helper2 && helpers[0].toLowerCase().replace(/\.$/,'') === helpers[1].toLowerCase().replace(/\.$/,'')) errors.helper2 = 'Use two different helper identities.';
        const recoveryInput = {count,helpers};
        if (Object.keys(errors).length) return { ...state, recoveryInput, recoveryErrors: errors, modal: 'recovery-editor' };
        return { ...state, recoveryInput, recoveryEditBaseline:clone(recoveryInput), recoverySavedInput:null, recoveryErrors: {}, recoveryDraft: { count, helpers: helpers.slice(0,used) }, modal: 'recovery-review', notice: '' };
      }
      case 'recovery-discard': return { ...state, recoveryDraft: null, recoverySavedInput:null, recoveryEditBaseline:null, recoveryInput:helperInput(null), recoveryErrors: {}, modal: null, notice: 'Recovery helper preview discarded. The active fixture is unchanged.' };
      case 'modal': {
        if (action.modal === 'recovery-review' && state.recoverySavedInput) return reduce({ ...state, recoveryEditBaseline:clone(state.recoverySavedInput) }, { type:'recovery-preview', ...state.recoverySavedInput });
        return { ...state, modal: action.modal, selectedSource: action.id || null, editorReturnToDetails: action.modal === 'edit-source' && state.modal === 'source-details', editorError: {}, recoveryErrors: {}, notice: '' };
      }
      case 'close': {
        if (state.modal === 'lifecycle-confirm') return {...state,modal:null,timeManagement:{...state.timeManagement,operation:null,kind:null,result:'cancelled'}};
        if (state.modal === 'service-confirm') return {...state,modal:null,serviceControl:{...state.serviceControl,operation:null,result:'cancelled'}};
        return state.modal === 'edit-source' && state.editorReturnToDetails ? { ...state, modal: 'source-details', editorReturnToDetails: false, editorError: {} } : { ...state, modal: null, editorReturnToDetails: false, editorError: {} };
      }
      case 'add-draft': return addDraft(state, action.raw, action.id);
      case 'remove-source': return removeDraftSource(state, action.id);
      case 'discard-drafts': return { ...state, drafts: [], modal: null, notice: 'All preview changes discarded.' };
      case 'undo-draft': return { ...state, drafts: state.drafts.filter(x => x.id !== action.id), notice:'Preview change removed.' };
      case 'clear-source-filter': return { ...state, sourceQuery:'', sourceFilter:'all' };
      case 'source-filter': return { ...state, sourceFilter: action.value };
      case 'source-query': return { ...state, sourceQuery: String(action.value).slice(0, 253) };
      case 'event-filter': return { ...state, eventFilter: action.value };
      case 'settings': return { ...state, settingsSection: ['schedule','policy','preferences','service','notifications'].includes(action.value) ? action.value : 'schedule' };
      case 'service-settings': return {...endRecovery(state),mode:'power',page:'settings',powerPage:'settings',settingsSection:'service',modal:null,notice:''};
      case 'service-diagnose': return {...endRecovery(state),mode:'power',page:'diagnostics',powerPage:'diagnostics',modal:null,serviceDiagnosis:true,notice:''};
      case 'sample-service-response': return typeof action.value === 'string' && Object.hasOwn(SERVICE_RESPONSES,action.value) && !state.modal ? {...state,sampleServiceResponse:action.value} : state;
      case 'sample-lifecycle-preflight': return !state.modal && !mutationPending(state) && typeof action.value === 'string' && Object.hasOwn(LIFECYCLE_PREFLIGHTS,action.value) ? {...state,sampleLifecyclePreflight:action.value} : state;
      case 'sample-lifecycle-response': return !state.modal && typeof action.value === 'string' && lifecycleResponseOptions(state).includes(action.value) ? {...state,sampleLifecycleResponse:action.value} : state;
      case 'lifecycle-request': {
        if (state.modal || lifecyclePermission(state,action.operation)) return state;
        return {...state,modal:'lifecycle-confirm',timeManagement:{...state.timeManagement,operation:action.operation,kind:action.operation.startsWith('startup-') ? 'startup' : 'ownership'}};
      }
      case 'lifecycle-confirm': {
        const t = state.timeManagement;
        if (state.modal !== 'lifecycle-confirm' || lifecyclePermission(state,t.operation) || lifecyclePreflight(state,t.operation).reason) return state;
        const startup = t.kind === 'startup';
        return {...state,notifications:{...state.notifications,requestSerial:state.notifications.requestSerial+1},modal:null,...(startup ? {} : {syncFeedback:null,sampleSyncStarted:false,serviceControl:{...state.serviceControl,presentation:'pending'}}),
          timeManagement:{...t,phase:'pending',result:'pending',previousPresentation:state.serviceControl.presentation,previousTimePresentation:t.presentation,previousFullyDisabled:t.fullyDisabled,fullyDisabled:false,preflight:state.sampleLifecyclePreflight},
          sampleLifecycleResponse:lifecycleResponseOptions(state).includes(state.sampleLifecycleResponse) ? state.sampleLifecycleResponse : 'confirmed'};
      }
      case 'sample-lifecycle-deliver': {
        const t = state.timeManagement, c = state.serviceControl, result = state.sampleLifecycleResponse;
        if (t.phase !== 'pending' || !lifecycleResponseOptions(state).includes(result)) return state;
        const next = {...t,phase:'idle',result};
        if (['uac-cancelled','failed'].includes(result)) return {...state,timeManagement:{...next,fullyDisabled:t.previousFullyDisabled},serviceControl:t.kind === 'startup' ? c : {...c,presentation:t.previousPresentation}};
        if (t.kind === 'startup') return {...state,timeManagement:{...next,startupIssue:result === 'unknown' ? {scopeSerial:state.notifications.requestSerial} : null,startup:startupPair(result === 'unknown' ? 'unknown' : t.operation === 'startup-on' ? 'automatic' : 'manual'),fullyDisabled:false,restartInhibited:result === 'confirmed' && t.operation === 'startup-on' ? false : t.restartInhibited}};
        if (result === 'unknown') return {...state,timeManagement:{...next,currentIssue:{code:'unknown',scopeSerial:state.notifications.requestSerial},owner:'unknown',ownerKnown:false,startup:startupPair('unknown'),cleanup:'unconfirmed',fullyDisabled:false,reconcileRequired:true,presentation:'unknown',windows:t.operation === 'disable' ? t.windows : {...t.windows,state:'unknown',startup:'unknown'}},serviceControl:{...c,observation:'unknown',ownershipKnown:false,ownership:'Unknown · reconcile current ownership',presentation:'unknown'}};
        const reverse = t.operation === 'use-nevrotick';
        const cleanupUnknown = result === 'cleanup-unconfirmed';
        const windowsUnavailable = result === 'windows-unavailable';
        const productUnavailable = result === 'product-unavailable';
        const owner = cleanupUnknown ? 'unknown' : windowsUnavailable || productUnavailable ? 'none' : reverse ? 'nevrotick' : t.operation === 'use-windows' ? 'windows' : t.owner === 'windows' ? 'windows' : 'none';
        const windows = reverse ? {...t.windows,state:'stopped',startup:'disabled',synchronization:'unverified'} : t.operation === 'use-windows' && !cleanupUnknown ? {...t.windows,state:windowsUnavailable ? 'failed' : 'running',startup:t.windows.restoreMode,synchronization:'unverified'} : t.windows;
        const updated = {...next,currentIssue:result === 'confirmed' ? null : {code:result,scopeSerial:state.notifications.requestSerial},startupIssue:null,startup:reverse ? startupPair(startupLabel(state) === 'On' ? 'automatic' : 'manual') : startupPair('disabled'),owner,ownerKnown:!cleanupUnknown,
          generation:reverse && !productUnavailable ? (t.generation || 12)+1 : t.generation,
          cleanup:cleanupUnknown ? 'unconfirmed' : 'confirmed',restartInhibited:!reverse || productUnavailable,fullyDisabled:!reverse && !cleanupUnknown,
          reconcileRequired:cleanupUnknown,presentation:owner,windows};
        return {...state,timeManagement:updated,sampleLog:endServiceLog(state.sampleLog),sampleLogResponse:{...state.sampleLogResponse,service:reverse && !productUnavailable ? 'refused' : 'unavailable'},
          serviceControl:{...c,observation:reverse && !productUnavailable ? 'running' : 'stopped',ownershipKnown:!cleanupUnknown,ownership:ownerLabel({timeManagement:updated}),presentation:reverse && !productUnavailable ? 'acquiring' : owner === 'windows' ? 'windows' : 'disabled'}};
      }
      case 'service-request': {
        if (state.modal || servicePermission(state,action.operation)) return state;
        return {...state,modal:'service-confirm',serviceControl:{...state.serviceControl,operation:action.operation}};
      }
      case 'service-confirm': {
        const c = state.serviceControl;
        if (state.modal !== 'service-confirm' || servicePermission(state,c.operation)) return state;
        return {...state,notifications:{...state.notifications,requestSerial:state.notifications.requestSerial+1},modal:null,syncFeedback:null,sampleSyncStarted:false,timeManagement:{...state.timeManagement,result:null,fullyDisabled:false},serviceControl:{...c,enableStartup:c.operation === 'start' && Object.values(state.timeManagement.startup).every(mode=>mode==='disabled'),previousFullyDisabled:state.timeManagement.fullyDisabled,previousTimeFeedback:state.timeManagement.result,phase:'pending',result:'pending',previousPresentation:c.presentation,presentation:'pending'}};
      }
      case 'sample-service-deliver': {
        const c = state.serviceControl;
        if (c.phase !== 'pending' || typeof state.sampleServiceResponse !== 'string' || !Object.hasOwn(SERVICE_RESPONSES,state.sampleServiceResponse)) return state;
        const result = state.sampleServiceResponse;
        if (result === 'uac-cancelled' || result === 'failed') return {...state,timeManagement:{...state.timeManagement,result:c.previousTimeFeedback,fullyDisabled:c.previousFullyDisabled},serviceControl:{...c,phase:'idle',result,presentation:c.previousPresentation}};
        if (result === 'unknown') {
          const windowsOwner = state.timeManagement.ownerKnown && state.timeManagement.owner === 'windows';
          return {...state,timeManagement:{...state.timeManagement,...(windowsOwner ? {} : {owner:'unknown',ownerKnown:false,reconcileRequired:true}),...(c.enableStartup ? {startup:startupPair('unknown')} : {})},serviceControl:{...c,currentIssue:{code:'unknown',scopeSerial:state.notifications.requestSerial},phase:'idle',result,observation:'unknown',ownershipKnown:windowsOwner,ownership:windowsOwner ? 'Windows Time · independently confirmed; NevroTick runtime unknown' : 'Unknown · reconcile current ownership; previous: ' + c.ownership,presentation:'unknown'}};
        }
        const t = state.timeManagement, windowsOwner = t.ownerKnown && t.owner === 'windows';
        const enable = c.operation === 'start' && Object.values(t.startup).every(mode => mode === 'disabled');
        return {...state,timeManagement:{...t,...(enable ? {startup:startupPair('manual'),fullyDisabled:false,restartInhibited:false} : {}),
          ...(windowsOwner ? {} : {owner:t.owner === 'none' ? 'none' : 'unknown',ownerKnown:t.owner === 'none',presentation:t.presentation}),
          ...(c.operation !== 'stop' ? {fullyDisabled:false} : {})},
          sampleLog:['stop','restart'].includes(c.operation) ? endServiceLog(state.sampleLog) : state.sampleLog,sampleLogResponse:{...state.sampleLogResponse,service:c.operation === 'stop' ? 'unavailable' : 'refused'},
          serviceControl:{...c,currentIssue:null,phase:'idle',result,observation:c.operation === 'stop' ? 'stopped' : 'running',ownershipKnown:windowsOwner || (c.operation === 'stop' ? c.ownershipKnown : false),ownership:windowsOwner ? 'Windows Time' : c.operation === 'stop' ? c.ownership : 'Unconfirmed after startup · last known: ' + c.ownership,presentation:c.operation === 'stop' ? 'stopped' : 'acquiring'}};
      }
      case 'compact': return { ...state, compact: !state.compact };
      case 'sample-sync': return syncOptions(state.scenario).includes(action.value) ? { ...state, sampleSync: action.value, syncFeedback:null, sampleSyncStarted:false } : state;
      case 'sync-now': {
        const windowsOwner = state.timeManagement.ownerKnown && state.timeManagement.owner === 'windows';
        if (windowsOwner && state.serviceControl.observation === 'stopped') return state;
        const presentation = state.serviceControl.presentation;
        const response = windowsOwner && state.serviceControl.observation === 'running' && !runtimePending(state) ? 'observation-check' : state.serviceControl.observation === 'stopped' && !runtimePending(state) ? 'service-stopped' : presentation === 'stopped' ? 'service-stopped' : ['pending','unknown'].includes(presentation) ? 'status-unknown' : presentation === 'acquiring' ? 'waiting' : syncOptions(state.scenario).includes(state.sampleSync) ? state.sampleSync : state.scenario.syncResponse;
        const checking = ['checking','checking-available'].includes(response);
        return { ...state, page:'status', powerPage:state.mode === 'power' ? 'status' : state.powerPage, modal:null, syncFeedback:checking && state.sampleSyncStarted ? 'already' : response, sampleSyncStarted:checking || response === 'already' };
      }
      case 'notification-settings': return {...endRecovery(state),mode:'power',page:'settings',powerPage:'settings',settingsSection:'notifications',modal:null,notice:''};
      case 'notifications-enabled': return typeof action.value === 'boolean' ? {...state,notifications:{...state.notifications,enabled:action.value,current:action.value ? state.notifications.current : null,feedback:action.value ? state.notifications.feedback : 'Problem notifications are off. The in-app status is unchanged.'}} : state;
      case 'sample-notification-os': return !state.modal && typeof action.value === 'string' && Object.hasOwn(NOTIFICATION_OS,action.value) ? {...state,notifications:{...state.notifications,os:action.value,current:action.value === 'allowed' ? state.notifications.current : null,feedback:''}} : state;
      case 'sample-notification-presenter': return !state.modal && typeof action.value === 'string' && Object.hasOwn(NOTIFICATION_PRESENTERS,action.value) ? {...state,notifications:{...state.notifications,presenter:action.value,current:action.value === 'active' ? state.notifications.current : null,feedback:''}} : state;
      case 'sample-notification-persistent': return !state.modal && typeof action.value === 'boolean' ? {...state,notifications:{...state.notifications,persistent:action.value}} : state;
      case 'notification-observe': return state.modal ? state : publishNotification(state);
      case 'notification-test': return state.modal ? state : publishNotification(state,true);
      case 'notification-dismiss': return state.notifications.current && state.notifications.current.id === action.id ? {...state,notifications:{...state.notifications,current:null,feedback:'Notification preview dismissed. The problem remains in NevroTick; the next identical observation will not create another notification.'}} : state;
      case 'notification-open': return openNotification(state,action.id);
      case 'sample-tray': return typeof action.value === 'boolean' ? { ...state, sampleTray: action.value, trayMenuOpen:false, sampleSignIn:action.value && state.sampleSignIn } : state;
      case 'tray-preview-enable': return {...state,sampleTray:true,trayMenuOpen:true};
      case 'tray-observer-open': return {...state,trayMenuOpen:false,notifications:{...state.notifications,presenter:'active',current:null},notice:'Sample observer reopened. Service state and clock ownership are unchanged.'};
      case 'tray-menu-open':
      case 'tray-menu-toggle': return state.sampleTray && state.notifications.presenter === 'active' ? {...state,trayMenuOpen:action.type === 'tray-menu-open' || !state.trayMenuOpen,modal:state.modal === 'tray-sample' ? null : state.modal} : state;
      case 'tray-menu-close': return state.trayMenuOpen ? {...state,trayMenuOpen:false} : state;
      case 'tray-open-page': {
        if (!state.sampleTray || state.notifications.presenter !== 'active' || !['status','settings','about','diagnostics'].includes(action.page)) return state;
        return reduce({...state,trayMenuOpen:false,modal:null},{type:action.page === 'diagnostics'?'power-page':'page',page:action.page});
      }
      case 'tray-check': {
        if (!state.sampleTray || state.notifications.presenter !== 'active') return state;
        const check = trayPresentation(state);
        if (!check.checkEnabled) return state;
        const target = check.checkAction === 'sync-now' ? reduce({...state,trayMenuOpen:false,modal:null},{type:'page',page:'status'}) : {...state,trayMenuOpen:false,modal:null};
        return reduce(target,{type:check.checkAction});
      }
      case 'tray-quit-manage': return state.modal === 'tray-quit-confirm' ? reduce({...state,trayMenuOpen:false,modal:null},{type:'service-settings'}) : state;
      case 'sample-signin': return state.sampleTray && typeof action.value === 'boolean' ? { ...state, sampleSignIn: action.value } : state;
      case 'sample-log-scope': return !logRunning(state.sampleLog) && ['ui','service','both'].includes(action.value) ? { ...state, sampleLog: { ...state.sampleLog, scope: action.value, state:'idle', results:{ui:'not-requested',service:'not-requested'} } } : state;
      case 'sample-log-duration': return !logRunning(state.sampleLog) && ['5','15','30'].includes(action.value) ? { ...state, sampleLog: { ...state.sampleLog, duration: action.value, state:'idle', results:{ui:'not-requested',service:'not-requested'} } } : state;
      case 'sample-log-response': return !logRunning(serviceLog(state)) && ['ui','service'].includes(action.component) && logResponseOptions(state,action.component).includes(action.value) ? { ...state, sampleLogResponse: { ...state.sampleLogResponse, [action.component]:action.value } } : state;
      case 'sample-log-start': {
        if (logRunning(serviceLog(state))) return state;
        const scope = state.sampleLog.scope;
        if (scope === 'service' && (runtimePending(state) || state.serviceControl.observation !== 'running')) return state;
        const options = logResponseOptions(state,'service');
        const service = options.includes(state.sampleLogResponse.service) ? state.sampleLogResponse.service : options[0];
        const results = { ui:scope === 'service' ? 'not-requested' : state.sampleLogResponse.ui, service:scope === 'ui' ? 'not-requested' : service };
        const selected = Object.values(results).filter(value => value !== 'not-requested');
        const active = selected.filter(value => value === 'active').length;
        const result = selected.includes('unknown') ? 'unconfirmed' : active === selected.length ? 'active' : active ? 'partial' : selected.includes('failed') ? 'failed' : 'not-started';
        return { ...state, sampleLog: { ...state.sampleLog, state:result, stopRequested:false, results } };
      }
      case 'sample-log-stop': {
        const log = serviceLog(state);
        if (!logRunning(log) || state.sampleLog.stopRequested) return state;
        const results = Object.fromEntries(Object.entries(log.results).map(([key,value]) => [key,value === 'active' ? 'stopped' : value]));
        const stopRequested = Object.values(results).includes('unknown');
        return { ...state, sampleLog: { ...state.sampleLog, state:stopRequested ? 'unconfirmed' : 'stopped', stopRequested, results } };
      }
      case 'sample-tray-quit': return {...state,trayMenuOpen:false,modal:'tray-quit-confirm'};
      case 'sample-tray-quit-confirm': return state.modal === 'tray-quit-confirm' ? {...state,trayMenuOpen:false,modal:null,notifications:{...state.notifications,presenter:'missing',current:null},notice:'Preview only: the app and tray observer have exited. Service state and clock ownership are unchanged. New local alerts require reopening an observer.'} : state;
      default: return state;
    }
  }
  function visibleSources(state) {
    state = presentState(state);
    const q = state.sourceQuery.toLowerCase();
    return state.scenario.sources.filter(x => (state.sourceFilter === 'all' || (state.sourceFilter === 'used' ? ['Main','Supporting'].includes(x.role) : x.role === 'Not used' || x.role === 'Waiting')) && (x.host + ' ' + x.name + ' ' + x.operator).toLowerCase().includes(q));
  }
  function summary(state) {
    const fixtureId = state.fixtureScenarioId || state.scenario.id;
    state = presentState(state);
    const s = state.scenario;
    return ['NevroTick — DESIGN FIXTURE ONLY', 'Source commit: ' + SOURCE_COMMIT, 'Scenario: ' + fixtureId, 'State: ' + s.title, 'Evidence scope: ' + (state.serviceControl.presentation ? 'Retained history before service operation; no fresh time agreement' : 'Selected design fixture'), (state.timeManagement.ownerKnown && state.timeManagement.owner === 'windows' ? 'Stored NevroTick profile (inactive): ' : 'Profile: ') + s.profile, ...(state.timeManagement.ownerKnown && state.timeManagement.owner === 'windows' ? ['Stored NevroTick source policy (inactive): Minimum ' + s.minimum + ' identities plus majority', 'Windows configuration: Existing peer, domain and policy settings preserved; no replacement peers selected'] : []), 'Authentication: ' + s.authentication, 'Agreement: ' + s.agree + '/' + s.total + '; ' + s.agreement, 'Qualified estimate: ' + s.estimate, 'Last applied correction: ' + s.applied + '; ' + s.appliedDetail, 'Status snapshot: ' + s.snapshot, 'Status connection: ' + s.connection, 'Service: ' + s.service, 'Time management observation: ' + s.description, 'Clock ownership: ' + s.ownership, 'NevroTick autostart: ' + startupLabel(state), 'Startup configuration: measurements ' + state.timeManagement.startup.measurement + '; clock control ' + state.timeManagement.startup.clock, 'Time manager: ' + ownerLabel(state), 'Last NevroTick ownership generation: ' + (state.timeManagement.generation ?? 'Unconfirmed'), 'Clock cleanup: ' + state.timeManagement.cleanup, 'Restart paths inhibited: ' + state.timeManagement.restartInhibited, 'Fully disabled: ' + fullDisableConfirmed(state), 'Windows Time service: ' + state.timeManagement.windows.state, 'Windows startup: ' + state.timeManagement.windows.startup, 'Windows synchronization: Not yet verified', 'Windows policy: ' + state.timeManagement.windows.policy, 'Time management request: ' + (state.timeManagement.result || 'None'), 'Service command sample: ' + (state.serviceControl.result || 'No command submitted'), 'Correction: ' + s.correction, 'Calendar: ' + s.calendar, 'Certificate-time basis: ' + s.recovery, 'History: ' + s.recoveryNote, 'Roughtime evidence: ' + recoveryPresentation(s).label, 'Roughtime decision: ' + s.roughtimeEvidence.reason, 'Roughtime signatures: ' + s.roughtimeEvidence.signatureStatus, 'Approximate recovery date: ' + (s.roughtimeEvidence.approximateDateInterval || 'Not available'), 'Recovery evidence age: ' + (s.roughtimeEvidence.receivedAge || 'No received evidence'), (state.serviceControl.presentation ? 'Historical NTS handoff: ' : 'NTS handoff: ') + s.roughtimeEvidence.ntsHandoffState, 'Recovery ancestry: ' + s.roughtimeEvidence.ancestry, 'Date-recovery clock effect: ' + s.roughtimeEvidence.clockEffect, '', 'No system data collected. No hostnames, usernames, credentials, cookies, keys or TLS secrets included.'].join('\n');
  }
  const api = { trayPresentation, NOTIFICATION_OS, NOTIFICATION_PRESENTERS, NOTIFICATION_POLICIES, notificationCandidate, notificationAvailability, notificationIsCurrent, runtimeObservation, fullDisableConfirmed, LIFECYCLE_OPERATIONS, LIFECYCLE_PREFLIGHTS, LIFECYCLE_RESPONSES, lifecyclePermission, lifecyclePreflight, lifecycleResponseOptions, startupLabel, ownerLabel, mutationPending, runtimePending, SERVICE_RESPONSES, servicePermission, presentState, serviceLog, recoveryPresentation, SYNC_RESPONSES, syncOptions, logResponseOptions, logRunning, SOURCE_COMMIT, SCENARIOS, FREQUENCIES, frequencyLabel, getScenario, initialState, escapeHtml, signed, offsetLabel, validateSource, validHost, reduce, visibleSources, summary };
  if (typeof module === 'object' && module.exports) module.exports = api;
  else root.NevroTickModel = api;
})(typeof globalThis === 'object' ? globalThis : this);
