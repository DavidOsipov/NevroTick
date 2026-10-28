export const BRAND = 'NevroTick';
export const ASSETS = {
  icon: './assets/app-icon-concept.png',
  logo: './assets/logo-light-transparent.png',
  character: './assets/mascot-selected.png',
};

export const INITIAL_SOURCES = [
  { id: 'alpha', host: 'alpha.example', protocol: 'nts', port: '4460', name: 'Alpha', group: 'Operator A', scale: 'utc', role: 'Principal', age: '18 s', lastValidReplyAge: '18 seconds ago', lastAcceptedMeasurementAge: '18 seconds ago', packetAuthentication: 'Authenticated', certificateTimeBasis: 'Saved authenticated model', timeEndpoint: 'alpha-time.example:123 · UDP', offset: '−0.7 ms', delay: '12.4 ms', uncertainty: '±4.8 ms', reason: 'Selected with two compatible supporting sources. Minimum agreement and the majority requirement are both met.' },
  { id: 'beta', host: 'beta.example', protocol: 'nts', port: '4460', name: 'Beta', group: 'Operator B', scale: 'utc', role: 'Supporting', age: '21 s', lastValidReplyAge: '21 seconds ago', lastAcceptedMeasurementAge: '21 seconds ago', packetAuthentication: 'Authenticated', certificateTimeBasis: 'Trusted local estimate', timeEndpoint: 'beta.example:123 · UDP', offset: '−0.9 ms', delay: '19.2 ms', uncertainty: '±6.1 ms', reason: 'Agrees with the selected reference interval.' },
  { id: 'gamma', host: 'gamma.example', protocol: 'nts', port: '4460', name: 'Gamma', group: 'Operator C', scale: 'utc', role: 'Supporting', age: '16 s', lastValidReplyAge: '16 seconds ago', lastAcceptedMeasurementAge: '16 seconds ago', packetAuthentication: 'Authenticated', certificateTimeBasis: 'Saved authenticated model', timeEndpoint: 'gamma.example:123 · UDP', offset: '−0.5 ms', delay: '23.1 ms', uncertainty: '±7.0 ms', reason: 'Agrees with the selected reference interval.' },
  { id: 'delta', host: 'delta.example', protocol: 'nts', port: '4460', name: 'Delta', group: 'Operator D', scale: 'utc', role: 'Rejected', age: '8 s', lastValidReplyAge: '8 seconds ago', lastAcceptedMeasurementAge: '4 minutes ago', packetAuthentication: 'Authenticated', certificateTimeBasis: 'Trusted local estimate', timeEndpoint: 'delta.example:123 · UDP', offset: '+42.8 ms', delay: '9.2 ms', uncertainty: '±5.3 ms', reason: 'The reported time does not overlap the agreeing sources. Recent authenticated replies are not accepted measurements.' },
];

export const SCENES = [
  { id: 'status', label: 'Состояние времени', group: 'Основные окна', tab: 'Status', state: 'tracking', note: 'Главный экран. Оценка смещения и неопределённость имеют одинаковый вес.' },
  { id: 'sources', label: 'Серверы времени', group: 'Основные окна', tab: 'Sources', note: 'Ручное добавление, редактирование, удаление и понятная роль каждого источника.' },
  { id: 'source-details', label: 'Подробности сервера', group: 'Основные окна', tab: 'Sources', details: true, note: 'Почему источник отклонён; свежий ответ отдельно от принятого измерения.' },
  { id: 'settings', label: 'Настройки', group: 'Основные окна', tab: 'Settings', note: 'Предпочтения окна отделены от системной политики.' },
  { id: 'settings-policy', label: 'Политика времени', group: 'Основные окна', tab: 'Settings', policy: true, note: 'Явный выбор профиля NTS/NTP. Системные изменения проходят обзор и авторизацию.' },
  { id: 'source-editor', label: 'Добавить NTS-сервер', group: 'Редактор и диалоги', tab: 'Sources', modal: 'source', note: 'Адрес вводится вручную. Для NTS порт относится к установлению ключей.' },
  { id: 'source-editor-ntp', label: 'Добавить NTP-сервер', group: 'Редактор и диалоги', tab: 'Sources', modal: 'source', profile: 'ntp', note: 'Показан отдельный пример NTP-профиля: обычный NTP явно не аутентифицирован.' },
  { id: 'source-editor-error', label: 'Ошибка адреса', group: 'Редактор и диалоги', tab: 'Sources', modal: 'source', invalid: true, note: 'Ошибка связана с полем. URL, путь и порт в адресе не принимаются.' },
  { id: 'review-changes', label: 'Обзор изменений', group: 'Редактор и диалоги', tab: 'Sources', modal: 'review', draft: true, note: 'Черновик и действующая конфигурация различимы; авторизация следует после обзора.' },
  { id: 'apply-result', label: 'Изменения применены', group: 'Редактор и диалоги', tab: 'Sources', modal: 'result', outcome: 'success', note: 'Успех означает принятие конфигурации, а не уже достигнутую синхронизацию.' },
  { id: 'apply-cancelled', label: 'Авторизация отменена', group: 'Редактор и диалоги', tab: 'Sources', modal: 'result', outcome: 'cancelled', draft: true, note: 'Активная политика остаётся прежней, черновик сохранён в этом окне.' },
  { id: 'apply-conflict', label: 'Конфликт конфигурации', group: 'Редактор и диалоги', tab: 'Sources', modal: 'result', outcome: 'conflict', draft: true, note: 'Изменения другого администратора нельзя молча перезаписать.' },
  { id: 'recovery', label: 'Восстановление даты', group: 'Редактор и диалоги', tab: 'Settings', modal: 'recovery', note: 'NTP-подсказки отдельно от источников синхронизации; по умолчанию выключены.' },
  { id: 'correction-approval', label: 'Большая коррекция', group: 'Редактор и диалоги', tab: 'Status', state: 'correction', modal: 'correction', note: 'Одноразовое разрешение привязано к свежему предложению, величине и направлению.' },
  { id: 'correction-expired', label: 'Предложение устарело', group: 'Редактор и диалоги', tab: 'Status', state: 'correction', modal: 'correction', expiredProposal: true, note: 'Устаревшее предложение нельзя одобрить: нужны новые измерения.' },
  { id: 'diagnostics', label: 'Диагностика', group: 'Поддержка и запуск', tab: 'Status', modal: 'diagnostics', note: 'Журнал различает принятие данных, отказ и изменение конфигурации.' },
  { id: 'copy-summary', label: 'Копирование сводки', group: 'Поддержка и запуск', tab: 'Status', modal: 'copy-summary', note: 'Сначала виден редактированный текст; копирование не отправляет его кому-либо.' },
  { id: 'export', label: 'Экспорт диагностики', group: 'Поддержка и запуск', tab: 'Status', modal: 'export', note: 'Предпросмотр состава и скрытие частных имён. Передача кому-либо не выполняется.' },
  { id: 'about', label: 'О программе', group: 'Поддержка и запуск', tab: 'Settings', modal: 'about', note: 'Бренд, версия, лицензии и сведения для поддержки.' },
  { id: 'onboarding', label: 'Первая настройка', group: 'Поддержка и запуск', tab: 'Status', state: 'empty', modal: 'onboarding', empty: true, note: 'Мастер появляется только когда настройка не завершена установщиком.' },
  { id: 'maintenance', label: 'Установка и обслуживание', group: 'Поддержка и запуск', tab: 'Settings', modal: 'maintenance', note: 'Единый MSI: передача управления часами, восстановление и удаление одного продукта.' },
  { id: 'unsaved-changes', label: 'Несохранённый черновик', group: 'Поддержка и запуск', tab: 'Sources', modal: 'discard', draft: true, note: 'Закрытие редактора и выход из UI не должны неожиданно потерять введённые данные.' },
  { id: 'tray', label: 'Меню трея', group: 'Поддержка и запуск', tab: 'Status', modal: 'tray', note: 'Выход из UI не останавливает фоновые службы. Трей необязателен.' },
  { id: 'status-acquiring', label: 'Сбор измерений', group: 'Состояния', tab: 'Status', state: 'acquiring' },
  { id: 'status-holdover', label: 'Временная потеря источников', group: 'Состояния', tab: 'Status', state: 'holdover' },
  { id: 'status-conflict', label: 'Серверы расходятся', group: 'Состояния', tab: 'Status', state: 'conflict' },
  { id: 'status-expired', label: 'Оценка устарела', group: 'Состояния', tab: 'Status', state: 'expired' },
  { id: 'status-offline', label: 'Нет сети', group: 'Состояния', tab: 'Status', state: 'offline' },
  { id: 'status-no-sources', label: 'Нет серверов', group: 'Состояния', tab: 'Status', state: 'empty', empty: true },
  { id: 'status-stale', label: 'Связь со статусом потеряна', group: 'Состояния', tab: 'Status', state: 'stale', note: 'Последние значения исторические. Разрыв status connection не доказывает остановку службы.' },
  { id: 'status-blocked', label: 'Другой владелец часов', group: 'Состояния', tab: 'Status', state: 'blocked' },
  { id: 'status-observe', label: 'Только наблюдение', group: 'Состояния', tab: 'Status', state: 'tracking', observe: true, clockOwner: 'NevroTick', note: 'Квалифицированные измерения доступны, но NevroTick не меняет часы. Observe only не возвращает управление Windows Time автоматически.' },
  { id: 'status-ntp', label: 'NTP без аутентификации', group: 'Состояния', tab: 'Status', state: 'tracking', profile: 'ntp' },
  { id: 'status-recovery', label: 'Восстановление NTS', group: 'Состояния', tab: 'Status', state: 'recovery' },
  { id: 'status-date-range', label: 'Дата вне диапазона', group: 'Состояния', tab: 'Status', state: 'date-range' },
  { id: 'status-initializing', label: 'Запуск и проверка готовности', group: 'Состояния', tab: 'Status', state: 'initializing', note: 'Работающий процесс ещё не подтверждает готовность политики и владения часами.' },
  { id: 'status-reacquiring', label: 'Повторная проверка после изменения', group: 'Состояния', tab: 'Status', state: 'reacquiring', note: 'После смены поколения прежний возраст неизвестен; старые измерения не становятся свежими.' },
  { id: 'status-stopping', label: 'Завершение фоновой работы', group: 'Состояния', tab: 'Status', state: 'stopping', note: 'Очистка ещё не завершена. Остановка не объявляется до подтверждённого результата.' },
  { id: 'status-service-failed', label: 'Подтверждённая остановка службы', group: 'Состояния', tab: 'Status', state: 'service-failed', note: 'Остановка подтверждена после попыток восстановления. Доступен путь к восстановлению полного пакета MSI.' },
  { id: 'status-gradual-correction', label: 'Плавная коррекция часов', group: 'Состояния', tab: 'Status', state: 'tracking', refreshOutcome: 'correction-in-progress', note: 'Источники квалифицированы. Подтверждённое начало коррекции не означает её завершения.' },
  { id: 'status-rate-limited', label: 'Ограничение повторного запроса', group: 'Состояния', tab: 'Status', state: 'tracking', refreshOutcome: 'rate-limited', note: 'Ограничение Sync now не меняет состояние и возраст действующей оценки.' },
  { id: 'status-after-one-helper', label: 'Работа после одного помощника', group: 'Состояния', tab: 'Status', state: 'tracking', recoveryHelpers: 1, note: 'После Tracking сохранено неаутентифицированное основание сертификатной даты от одного помощника.' },
  { id: 'status-after-two-helpers', label: 'Работа после двух помощников', group: 'Состояния', tab: 'Status', state: 'tracking', recoveryHelpers: 2, note: 'Два согласных помощника не делают исходную подсказку аутентифицированной; история остаётся доступной.' },
];

export const STATES = {
  tracking: { title: 'Time sync active', tone: 'success', offset: '0.7 ms ahead', uncertainty: '±4.8 ms', auth: 'NTS', agreement: '3 of 4 eligible · majority met', age: '18 seconds ago', description: 'The configured minimum and majority requirement are both met. Windows is ahead of the estimated reference.', action: 'Sync now', next: 'Next scheduled check in 40 seconds' },
  acquiring: { title: 'Checking time sources', tone: 'info', offset: 'Not available', uncertainty: 'Not available', auth: 'Waiting for accepted data', agreement: 'Waiting for enough agreeing sources', age: 'No qualified estimate', description: 'Collecting fresh measurements. Both the configured minimum and majority agreement are required.', action: 'Sync now' },
  holdover: { title: 'No fresh time check', tone: 'warning', offset: 'Model estimate only', uncertainty: '±38 ms', auth: 'Last accepted data: NTS', agreement: 'No fresh agreement', age: '12 minutes ago', description: 'Using the previous model within its allowed budget. Estimated uncertainty is growing.', action: 'Sync now', next: 'Next retry in 20 seconds' },
  conflict: { title: 'Time sources disagree', tone: 'danger', offset: 'Not established', uncertainty: 'Not established', auth: 'NTS packets received', agreement: '2 vs 2 eligible · no majority', age: '4 minutes ago', description: 'Two sources agree on each side. The minimum of two is met, but there is no majority. No new correction is authorized.', action: 'Review sources' },
  expired: { title: 'No usable current time estimate', tone: 'danger', offset: 'Not available', uncertainty: 'Outside allowed budget', auth: 'Last accepted data: NTS', agreement: 'No current agreement', age: '2 hours ago', description: 'The previous estimate exceeded its age or quality budget. Bounded retries continue without a fallback.', action: 'Sync now' },
  offline: { title: 'Offline · holdover', tone: 'warning', offset: 'Model estimate only', uncertainty: '±38 ms', auth: 'Last accepted data: NTS', agreement: 'Network unavailable', age: '12 minutes ago', description: 'No network connection. The previous model remains within its allowed budget.', action: 'Retry now', next: 'Waiting for the network' },
  empty: { title: 'No time servers', tone: 'info', offset: 'Not available', uncertainty: 'Not available', auth: 'No accepted data', agreement: 'No sources configured', age: 'No accepted observation', description: 'Add time servers to start collecting measurements.', action: 'Add time server' },
  stale: { title: 'Status unavailable', tone: 'warning', offset: 'Last: 0.7 ms ahead', uncertainty: 'Last: ±4.8 ms', auth: 'Last snapshot: NTS', agreement: 'Historical: 3 of 4 eligible', age: 'Snapshot received 6 minutes ago', description: 'Showing the last received snapshot. Current synchronization and service health are unknown.', action: 'Reconnect' },
  blocked: { title: 'Clock control blocked', tone: 'danger', offset: '0.7 ms ahead', uncertainty: '±4.8 ms', auth: 'NTS', agreement: '3 of 4 sources', age: '18 seconds ago', description: 'Windows Time Service is controlling the clock. NevroTick has stopped new corrections.', action: 'View diagnostics' },
  observe: { title: 'Measuring reference time', tone: 'info', offset: '0.7 ms ahead', uncertainty: '±4.8 ms', auth: 'NTS', agreement: '3 of 4 eligible · majority met', age: '18 seconds ago', description: 'Clock changes by NevroTick are disabled. Observe only does not re-enable Windows Time.', action: 'Sync now' },
  recovery: { title: 'Recovering authenticated time', tone: 'warning', offset: 'Not established', uncertainty: 'Not established', auth: 'Checking NTS with a plain-NTP hint', agreement: 'Awaiting authenticated agreement', age: 'No current accepted observation', description: 'An unauthenticated date hint is being used to check certificates. The hint has not changed Windows time.', action: 'View recovery' },
  'date-range': { title: 'System date needs attention', tone: 'danger', offset: 'Not available', uncertainty: 'Not available', auth: 'Certificate verification blocked', agreement: 'Not established', age: 'No current accepted observation', description: 'The system date is outside the supported verification range. An administrator can review recovery options.', action: 'View recovery' },
  correction: { title: 'Correction needs approval', tone: 'warning', offset: '6 min 12 s behind', uncertainty: '±4.8 ms', auth: 'NTS', agreement: '3 of 4 sources', age: '18 seconds ago', description: 'Authenticated time is available. This correction exceeds the automatic policy.', action: 'Review correction' },
  initializing: { title: 'Starting and checking configuration', tone: 'info', offset: 'Not available', uncertainty: 'Not available', auth: 'Not established', agreement: 'Readiness checks in progress', age: 'No qualified estimate', description: 'Configuration and clock ownership must be ready before synchronization can begin.', action: 'View diagnostics' },
  reacquiring: { title: 'Rechecking after a change', tone: 'info', offset: 'Not available', uncertainty: 'Not available', auth: 'Awaiting current-generation evidence', agreement: 'Collecting new observations', age: 'Unknown after a system change', description: 'Earlier exchanges no longer count. Fresh agreement is required before new corrections.', action: 'Sync now' },
  stopping: { title: 'Finishing background work', tone: 'info', offset: 'Last: 0.7 ms ahead', uncertainty: 'Last: ±4.8 ms', auth: 'Last qualified estimate: NTS', agreement: 'Historical: 3 of 4 eligible', age: '18 seconds ago', description: 'Cleanup is still pending. The service has not yet confirmed that it stopped.', action: 'View diagnostics' },
  'service-failed': { title: 'Time service stopped', tone: 'danger', offset: 'Last: 0.7 ms ahead', uncertainty: 'Last: ±4.8 ms', auth: 'Last snapshot: NTS', agreement: 'Historical: 3 of 4 eligible', age: 'Snapshot received 6 minutes ago', description: 'The service is confirmed stopped after recovery attempts. Repair the installed product using the complete signed MSI.', action: 'View diagnostics' },
};

export function canonicalHost(value) {
  const raw = value.trim();
  if (!raw) throw new Error('Enter a server address.');
  if (/[\s/\\@?#%]/u.test(raw)) throw new Error('Enter a hostname or IP address, without a URL, path, space, or zone identifier.');
  if (raw.includes(':')) {
    const v6 = raw.startsWith('[') && raw.endsWith(']') ? raw.slice(1,-1) : raw;
    if ((v6.match(/:/g)||[]).length < 2) throw new Error('Enter the port in Advanced, not in the server address.');
    try {
      const h = new URL('https://[' + v6 + ']/').hostname;
      return h.slice(1,-1).toLowerCase();
    } catch { throw new Error('Enter a valid IPv6 address. Put the port in Advanced.'); }
  }
  if (/^[0-9.]+$/.test(raw)) {
    const octets = raw.split('.');
    if (octets.length !== 4 || octets.some(x=>!/^([0-9]|[1-9][0-9]{1,2})$/.test(x)||Number(x)>255)) throw new Error('Enter four IPv4 octets from 0 to 255, without leading zeros.');
    return octets.join('.');
  }
  let host;
  try { host = new URL('https://' + raw + '/').hostname.toLowerCase().replace(/\.$/,''); }
  catch { throw new Error('Enter a valid DNS hostname.'); }
  // WHATWG URL parsing accepts legacy IPv4 spellings. The strict IPv4
  // branch above is the only route by which an IPv4 literal may be admitted.
  if (/^[0-9]+(?:\.[0-9]+){3}$/.test(host)) throw new Error('Use a full dotted-decimal IPv4 address, without hex, octal, or integer forms.');
  if (host.length>253 || host.split('.').some(x=> !/^[a-z0-9](?:[a-z0-9-]{0,61}[a-z0-9])?$/.test(x))) throw new Error('Enter a valid DNS hostname; labels must not begin or end with a hyphen.');
  return host;
}

export function validateSource(form, sources, profile) {
  const errors={}; let host='';
  try { host=canonicalHost(form.host); } catch(e) { errors.host=e.message; }
  if (!/^\d+$/.test(String(form.port)) || Number(form.port)<1 || Number(form.port)>65535) errors.port='Use a whole port number from 1 to 65535.';
  if (form.protocol!==profile) errors.protocol=`The active draft uses ${profile.toUpperCase()}. Change the time profile in Settings before adding this protocol.`;
  if (host && sources.some(s=>s.id!==form.id && s.host===host)) errors.host='This logical server is already listed. Edit its existing entry.';
  if (form.scale!=='utc') errors.scale='The current profile uses unsmeared UTC. A different time scale needs a separately approved compatible policy.';
  return {errors,host};
}

export const newRecovery = () => ({enabled:false,mode:'two',helpers:['','']});
export const copyRecovery = recovery => ({...recovery,helpers:[...recovery.helpers]});

export function validateRecovery(recovery) {
  const errors=[];
  if (!recovery || typeof recovery.enabled!=='boolean' || !['one','two'].includes(recovery.mode) || !Array.isArray(recovery.helpers) || recovery.helpers.length!==2 || recovery.helpers.some(h=>typeof h!=='string')) {
    return {errors:['Recovery settings are incomplete. Review the helper policy.'],recovery:null};
  }
  const result=copyRecovery(recovery);
  if (recovery.enabled) {
    const required=recovery.mode==='two'?2:1;
    for(let i=0;i<required;i++) {
      try { result.helpers[i]=canonicalHost(recovery.helpers[i]); }
      catch(e) { errors.push(`Bootstrap helper ${i+1}: ${e.message}`); }
    }
    if (!errors.length && required===2 && result.helpers[0]===result.helpers[1]) errors.push('Use two distinct bootstrap helper addresses.');
  }
  return {errors,recovery:result};
}

export function validateConfiguration(sources,profile,recovery=newRecovery()) {
  const errors=[];
  if (sources.length<2) errors.push('The current policy requires at least 2 logical sources. Add sources; the minimum is not lowered automatically. Runtime selection also requires majority agreement.');
  if (sources.some(s=>s.protocol!==profile)) errors.push(`All normal sources must match the ${profile.toUpperCase()} profile. Edit or remove incompatible sources before applying.`);
  errors.push(...validateRecovery(recovery).errors);
  return errors;
}

const SOURCE_REVIEW_FIELDS = [
  ['host','Server address',s=>s.host],
  ['protocol','Protocol',s=>s.protocol==='nts'?'NTS · authenticated NTPv4':'NTPv4 · unauthenticated'],
  ['port','Port',s=>`${s.port} · ${s.protocol==='nts'?'TCP · NTS-KE':'UDP · NTP'}`],
  ['name','Display name',s=>s.name||'Not set'],
  ['group','Operator group',s=>s.group||'Not set'],
  ['scale','Time scale',s=>s.scale==='utc'?'UTC · no leap smear':s.scale],
];

export function sourceReviewChanges(before,after) {
  const label=before?.host||after.host;
  return SOURCE_REVIEW_FIELDS.filter(([key])=>!before||before[key]!==after[key]).map(([,field,format])=>({
    kind:`${label} · ${field}`,
    before:before?format(before):'Not configured',
    after:format(after),
  }));
}

export function recoveryReviewChanges(before,after) {
  const fields=[
    ['Bootstrap hints',r=>r.enabled?'Enabled':'Off'],
    ['Bootstrap policy',r=>r.mode==='two'?'Two helpers · both must agree':'One helper · weaker date basis'],
    ['Bootstrap helper 1',r=>r.helpers[0]||'Not configured'],
    ['Bootstrap helper 2',r=>r.helpers[1]||'Not configured'],
  ];
  return fields.map(([kind,read])=>({kind,before:read(before),after:read(after)})).filter(row=>row.before!==row.after);
}

export const newSource = protocol => ({id:'',host:'',protocol,port:protocol==='nts'?'4460':'123',name:'',group:'',scale:'utc'});
export const extraSource = () => ({...newSource('nts'),id:'epsilon',host:'epsilon.example',name:'Epsilon',group:'Operator E',role:'Pending',age:'—',lastValidReplyAge:'Not contacted',lastAcceptedMeasurementAge:'Not established',packetAuthentication:'Not established',certificateTimeBasis:'Not established',timeEndpoint:'Not established',offset:'—',delay:'—',uncertainty:'—',reason:'Not applied yet.'});

export const LOGS = [
  {id:1084,age:'18 s ago',level:'Info',code:'TIME_TRACKING',message:'Published a qualified NTS estimate; minimum agreement and majority are both met.'},
  {id:1083,age:'24 s ago',level:'Warning',code:'SOURCE_DISAGREEMENT',message:'A source was excluded: its time interval does not overlap the reference.'},
  {id:1082,age:'3 min ago',level:'Info',code:'CONFIG_APPLIED',message:'Configuration revision 18 was accepted.'},
  {id:1081,age:'4 min ago',level:'Info',code:'NTS_ESTABLISHED',message:'Authenticated key establishment completed.'},
  {id:1080,age:'12 min ago',level:'Warning',code:'NETWORK_CHANGED',message:'The network changed; collecting fresh observations.'},
];
