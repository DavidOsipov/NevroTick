/* Serialize real view functions for structural checks. This is not browser rendering. */
'use strict';
const fs=require('node:fs');
const M=require('../src/model.js'), V=require('../src/views.js');
const records=[];
for(const s of M.SCENARIOS){
  for(const mode of ['simple','power']){
    const pages=mode==='simple'?['status','settings','about']:['status','sources','statistics','recovery','diagnostics','settings','about'];
    for(const page of pages) records.push({name:mode+'/'+s.id+'/'+page,html:V.render({...M.initialState(s.id),mode,page})});
  }
  for(const modal of ['source-details','add-source','review','export','result']) records.push({name:'power/'+s.id+'/modal/'+modal,html:V.render({...M.initialState(s.id),mode:'power',modal,selectedSource:'alpha'})});
  records.push({name:'simple/'+s.id+'/modal/status-help',html:V.render({...M.initialState(s.id),modal:'status-help'})});
}
for(const mode of ['simple','power']){
  for(const s of ['correction','correction-expired']) records.push({name:mode+'/'+s+'/modal/correction',html:V.render({...M.initialState(s),mode,modal:'correction'})});
  for(const value of ['auto','120','300','900']){
    let state=M.reduce(M.initialState(),{type:'frequency-draft',value});
    records.push({name:mode+'/cadence/draft/'+value,html:V.render({...state,mode,page:'settings'})});
    state=M.reduce(state,{type:'frequency-save'});
    records.push({name:mode+'/cadence/saved/'+value,html:V.render({...state,mode,page:'settings'})});
  }
}
for(const section of ['policy','preferences','notifications','unknown-section']) records.push({name:'power/settings/'+section,html:V.render({...M.initialState(),mode:'power',page:'settings',settingsSection:section})});
for(const id of ['server-rate-limited','server-access-refused']) {
  let affected = {...M.initialState(id), sourceFilter:'used', sourceQuery:'alpha'};
  affected = M.reduce(affected,{type:'affected-source'});
  records.push({name:'power/'+id+'/affected-source',html:V.render(affected)});
}
for(const mode of ['simple','power']) for(const scenario of M.SCENARIOS) {
  for(const value of M.syncOptions(scenario)) {
    const selected=M.reduce(M.initialState(scenario.id),{type:'sample-sync',value});
    const state=M.reduce({...selected,mode},{type:'sync-now'});
    records.push({name:mode+'/'+scenario.id+'/sync/'+value,html:V.render(state)});
  }
}
for(const scope of ['ui','service','both']) for(const ui of ['active','failed']) for(const service of ['active','refused','failed']) {
  let state=M.reduce(M.initialState(),{type:'sample-log-scope',value:scope});
  state=M.reduce(state,{type:'sample-log-response',component:'ui',value:ui});
  state=M.reduce(state,{type:'sample-log-response',component:'service',value:service});
  state=M.reduce(state,{type:'sample-log-start'});
  records.push({name:'power/diagnostics/'+scope+'/'+ui+'/'+service,html:V.render({...state,mode:'power',page:'diagnostics',modal:'debug-sample'})});
}
const state=M.reduce(M.initialState(),{type:'add-draft',raw:{name:'A literal <label>',host:'new.example',protocol:'NTP',port:'123'}});
records.push({name:'power/draft/review',html:V.render({...state,mode:'power',page:'sources',modal:'review'})});
records.push({name:'simple/draft-retained/settings',html:V.render({...state,page:'settings'})});
let recovery={...M.initialState(),mode:'power',page:'recovery',powerPage:'recovery'};
recovery=M.reduce(recovery,{type:'recovery-edit'});
records.push({name:'power/recovery/editor',html:V.render(recovery)});
recovery=M.reduce(recovery,{type:'recovery-input',count:'2',helpers:['one.example','unfinished /']});
recovery=M.reduce(recovery,{type:'close'});
records.push({name:'power/recovery/leave-warning',html:V.render(recovery)});
recovery=M.reduce(recovery,{type:'recovery-keep-leave'});
for(const mode of ['simple','power']) records.push({name:mode+'/recovery/kept-partial-draft',html:V.render({...recovery,mode,page:'status'})});
records.push({name:'power/recovery/reset-warning',html:V.render(M.reduce(recovery,{type:'reset'}))});
recovery=M.reduce(recovery,{type:'modal',modal:'recovery-review'});
records.push({name:'power/recovery/partial-review-error',html:V.render(recovery)});
recovery=M.reduce(recovery,{type:'recovery-preview',count:'2',helpers:['one.example','two.example']});
records.push({name:'power/recovery/saved-review',html:V.render(recovery)});
for(const id of ['healthy','service-stopped','stale']) {
  const initial=M.reduce(M.initialState(id),{type:'service-settings'});
  records.push({name:'service/'+id+'/settings',html:V.render(initial)});
  records.push({name:'service/'+id+'/diagnose',html:V.render(M.reduce(initial,{type:'service-diagnose'}))});
  for(const operation of ['start','stop','restart']) {
    if(M.servicePermission(initial,operation)) continue;
    const confirm=M.reduce(initial,{type:'service-request',operation});
    records.push({name:'service/'+id+'/'+operation+'/confirm',html:V.render(confirm)});
    const pending=M.reduce(confirm,{type:'service-confirm'});
    records.push({name:'service/'+id+'/'+operation+'/pending',html:V.render(pending)});
    for(const response of Object.keys(M.SERVICE_RESPONSES)) {
      const selected=M.reduce(pending,{type:'sample-service-response',value:response});
      const result=M.reduce(selected,{type:'sample-service-deliver'});
      for(const page of ['status','sources','statistics','diagnostics','settings']) records.push({name:'service/'+id+'/'+operation+'/'+response+'/'+page,html:V.render({...result,page})});
      records.push({name:'service/'+id+'/'+operation+'/'+response+'/export',html:V.render({...result,modal:'export'})});
      records.push({name:'service/'+id+'/'+operation+'/'+response+'/source-details',html:V.render({...result,modal:'source-details',selectedSource:'alpha'})});
      records.push({name:'service/'+id+'/'+operation+'/'+response+'/diagnose',html:V.render(M.reduce(result,{type:'service-diagnose'}))});
    }
  }
}
const ownershipBase=M.reduce(M.initialState('roughtime-recovered'),{type:'service-settings'});
const ownershipComplete=(state,operation)=>M.reduce(M.reduce(M.reduce(state,{type:'lifecycle-request',operation}),{type:'lifecycle-confirm'}),{type:'sample-lifecycle-deliver'});
const windowsOwned=ownershipComplete(ownershipBase,'use-windows');
for(const [label,initial,operations] of [['initial',ownershipBase,['startup-off','disable','use-windows']],['windows',windowsOwned,['startup-on','use-nevrotick']]]) {
  for(const operation of operations) {
    const confirmation=M.reduce(initial,{type:'lifecycle-request',operation});
    records.push({name:`ownership/${label}/${operation}/confirmation`,html:V.render(confirmation)});
    const pending=M.reduce(confirmation,{type:'lifecycle-confirm'});
    records.push({name:`ownership/${label}/${operation}/pending`,html:V.render(pending)});
    for(const response of M.lifecycleResponseOptions(pending)) {
      const result=M.reduce(M.reduce(pending,{type:'sample-lifecycle-response',value:response}),{type:'sample-lifecycle-deliver'});
      for(const mode of ['simple','power'])for(const page of mode==='simple'?['status','settings']:['status','settings','sources','statistics','diagnostics','recovery'])records.push({name:`ownership/${label}/${operation}/${response}/${mode}/${page}`,html:V.render({...result,mode,page})});
      for(const modal of ['export','source-details','status-help','correction'])records.push({name:`ownership/${label}/${operation}/${response}/${modal}`,html:V.render({...result,modal,selectedSource:'alpha'})});
    }
  }
}
for(const preflight of Object.keys(M.LIFECYCLE_PREFLIGHTS)) {
  const state=M.reduce(ownershipBase,{type:'sample-lifecycle-preflight',value:preflight});
  records.push({name:`ownership/preflight/${preflight}`,html:V.render(M.reduce(state,{type:'lifecycle-request',operation:'use-windows'}))});
}
for(const scene of M.SCENARIOS) for(const persistent of [false,true]) for(const mode of ['simple','power']) {
  let state=M.reduce(M.initialState(scene.id),{type:'sample-notification-persistent',value:persistent});
  state=M.reduce(state,{type:'notification-observe'});
  records.push({name:`notifications/${scene.id}/${persistent}/${mode}`,html:V.render({...state,mode,page:'settings',settingsSection:'notifications'})});
}
for(const os of Object.keys(M.NOTIFICATION_OS)) for(const mode of ['simple','power']) {
  let state=M.reduce(M.initialState('expired'),{type:'sample-notification-os',value:os});
  state=M.reduce(state,{type:'notification-observe'});
  records.push({name:`notifications/readiness/${os}/${mode}`,html:V.render({...state,mode,page:'settings',settingsSection:'notifications'})});
}
for(const mode of ['simple','power']) {
  let state=M.reduce(M.initialState('conflict'),{type:'notification-observe'});
  for(const [label,change] of [['dismiss',{type:'notification-dismiss',id:state.notifications.current.id}],['off',{type:'notifications-enabled',value:false}],['missing',{type:'sample-notification-presenter',value:'missing'}],['test',{type:'notification-test'}]])records.push({name:`notifications/${label}/${mode}`,html:V.render({...M.reduce(state,change),mode,page:'settings',settingsSection:'notifications'})});
  records.push({name:`notifications/windows-settings-modal/${mode}`,html:V.render({...state,mode,page:'settings',settingsSection:'notifications',modal:'windows-notification-settings'})});
  records.push({name:`notifications/reset-guard/${mode}`,html:V.render({...M.reduce(state,{type:'reset'}),mode})});
  const previous=M.reduce(M.initialState('correction'),{type:'notification-observe'});
  records.push({name:`notifications/stale-correction/${mode}`,html:V.render({...M.initialState('correction-expired'),notifications:previous.notifications,mode})});
  for(const scene of ['healthy','observe','blocked','stale','service-stopped'])records.push({name:`tray/settings-preferences/${scene}/${mode}`,html:V.render({...M.initialState(scene),mode,page:'settings',settingsSection:'preferences',modal:'tray-sample',sampleTray:true})});
}
for(const result of ['uac-cancelled','failed']) {
  let state=M.reduce(ownershipBase,{type:'lifecycle-request',operation:'use-windows'});
  state=M.reduce(state,{type:'lifecycle-confirm'});
  state=M.reduce(M.reduce(state,{type:'sample-lifecycle-response',value:'windows-unavailable'}),{type:'sample-lifecycle-deliver'});
  state=M.reduce(state,{type:'notification-observe'});
  state=M.reduce(M.reduce(state,{type:'lifecycle-request',operation:'use-windows'}),{type:'lifecycle-confirm'});
  state=M.reduce(state,{type:'notification-observe'});
  state=M.reduce(M.reduce(state,{type:'sample-lifecycle-response',value:result}),{type:'sample-lifecycle-deliver'});
  state=M.reduce(state,{type:'notification-observe'});
  records.push({name:`notifications/partial-retained/${result}`,html:V.render({...state,page:'settings',settingsSection:'notifications'})});
}
for (const scenario of M.SCENARIOS) for (const mode of ['simple','power']) {
  const base={...M.initialState(scenario.id),mode};
  const open=M.reduce(base,{type:'tray-preview-enable'});
  for(const [label,state] of [['menu',open],['quit-warning',M.reduce(open,{type:'sample-tray-quit'})],['observer-closed',M.reduce(M.reduce(open,{type:'sample-tray-quit'}),{type:'sample-tray-quit-confirm'})]]) records.push({name:`tray/${mode}/${scenario.id}/${label}`,html:V.render(state)});
}
fs.writeFileSync(process.argv[2],JSON.stringify(records));
console.log(records.length+' serialized view fixtures');
