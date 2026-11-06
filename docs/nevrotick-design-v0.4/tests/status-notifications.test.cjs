'use strict';
const test=require('node:test'),assert=require('node:assert/strict');
const M=require('../src/model.js'),V=require('../src/views.js');
const base=(id='healthy')=>M.reduce(M.initialState(id),{type:'service-settings'});
const lifecycle=(state,operation,response='confirmed')=>M.reduce(M.reduce(M.reduce(M.reduce(state,{type:'lifecycle-request',operation}),{type:'lifecycle-confirm'}),{type:'sample-lifecycle-response',value:response}),{type:'sample-lifecycle-deliver'});
const runtime=(state,operation,response='confirmed')=>M.reduce(M.reduce(M.reduce(M.reduce(state,{type:'service-request',operation}),{type:'service-confirm'}),{type:'sample-service-response',value:response}),{type:'sample-service-deliver'});

test('unknown clock outcome and observation-only profiles qualify the current manager without rewriting source evidence',()=>{
 for(const id of ['outcome-unknown','observe']){
  const state=M.initialState(id),raw=JSON.stringify(state.scenario);
  const shown=M.presentState(state);
  assert.match(shown.scenario.ownership,id==='observe'?/Observation only.*no clock-writing authority/i:/Unconfirmed.*last known NevroTick/i);
  assert.equal(state.timeManagement.ownerKnown,false);
  assert.match(V.settings({...state,mode:'power',page:'settings',settingsSection:'policy'}),new RegExp(shown.scenario.ownership.replace(/[.*+?^${}()|[\]\\]/g,'\\$&')));
  assert.match(M.summary(state),/Clock ownership: (Unconfirmed|Observation only)/);
  assert.equal(JSON.stringify(state.scenario),raw);
 }
});

test('lost replies for Windows-owned observer commands scope uncertainty to NevroTick, not the separately confirmed Windows manager',()=>{
 const windows=lifecycle(base('roughtime-recovered'),'use-windows');
 for(const operation of ['start','stop','restart']){
  const observing=operation==='start'?windows:runtime(windows,'start');
  const raw=JSON.stringify(observing.scenario),before=JSON.stringify(observing.timeManagement.windows);
  const state=runtime(observing,operation,'unknown');
  assert.equal(state.serviceControl.observation,'unknown');
  assert.equal(state.timeManagement.owner,'windows');assert.equal(state.timeManagement.ownerKnown,true);
  assert.equal(state.serviceControl.ownershipKnown,true);
  assert.equal(JSON.stringify(state.timeManagement.windows),before);
  assert.equal(JSON.stringify(state.scenario),raw);
  assert.match(V.simpleStatus(state),/Windows Time manages time/);
  assert.match(M.summary(state),/Time manager: Windows Time/);
  assert.match(M.summary(state),/Windows synchronization: Not yet verified/);
 }
});

test('status and tray primary actions respect stopped, pending and unknown runtime while startup-only requests preserve healthy checks',()=>{
 const stopped=base('service-stopped'),healthy=base();
 const pending=M.reduce(M.reduce(healthy,{type:'service-request',operation:'stop'}),{type:'service-confirm'});
 for(const state of [stopped,runtime(healthy,'stop'),pending,runtime(healthy,'stop','unknown'),base('stale')]){
  for(const view of [V.simpleStatus,V.status]){
   const html=view(state);assert.doesNotMatch(html,/>Sync now<\/button>/);
   if(state.serviceControl.observation==='stopped'&&!M.runtimePending(state))assert.match(html,/data-action="service-settings"/);
   else assert.match(html,/Diagnose|disabled[^>]*aria-label="Check sources/);
  }
  assert.doesNotMatch(V.modal({...state,modal:'tray-sample'}),/>Sync now<\/button>/);
 }
 for(const state of [healthy,lifecycle(healthy,'startup-off','unknown'),M.reduce(M.reduce(healthy,{type:'lifecycle-request',operation:'startup-off'}),{type:'lifecycle-confirm'})])assert.match(V.status(state),/>Sync now<\/button>/);
 for(const id of ['observe','blocked','outcome-unknown']){
  assert.match(V.status(base(id)),/>Check sources<\/button>/);
  assert.match(V.status(base(id)),/does not apply.*clock|does not change.*clock/i);
 }
 assert.match(V.status(stopped),/Open Background service|Review Start/);
 assert.doesNotMatch(V.status(stopped),/supported repair route/);
});

const observe=state=>M.reduce(state,{type:'notification-observe'});
const withPersistent=state=>M.reduce(state,{type:'sample-notification-persistent',value:true});
const notificationOff=state=>M.reduce(state,{type:'notifications-enabled',value:false});
const setOS=(state,value)=>M.reduce(state,{type:'sample-notification-os',value});
const immediate=new Set(['expired','conflict','service-stopped','blocked','signed-conflict','correction','outcome-unknown']);
const persistent=new Set(['server-access-refused','holdover','offline','stale','leap-guard','roughtime-unavailable','roughtime-rejected','all-rate-limited']);

test('all 30 scenes have an explicit notification policy based on current evidence rather than their color or historical events',()=>{
 assert.deepEqual(Object.keys(M.NOTIFICATION_POLICIES).sort(),M.SCENARIOS.map(s=>s.id).sort());
 for(const scene of M.SCENARIOS){
  const state=M.initialState(scene.id),raw=JSON.stringify(state.scenario);
  const candidate=M.notificationCandidate(state),normal=observe(state),lasting=observe(withPersistent(state));
  assert.equal(!!candidate,immediate.has(scene.id)||persistent.has(scene.id),scene.id);
  assert.equal(!!normal.notifications.current,immediate.has(scene.id),scene.id+' immediate');
  assert.equal(!!lasting.notifications.current,immediate.has(scene.id)||persistent.has(scene.id),scene.id+' persistent');
  assert.equal(JSON.stringify(state.scenario),raw,scene.id);
  assert.deepEqual(normal.timeManagement,state.timeManagement);assert.deepEqual(normal.serviceControl,state.serviceControl);
 }
});

test('persistent-problem eligibility is a published fact, never the number of polls or renders',()=>{
 let state=M.initialState('offline');
 for(let i=0;i<12;i++)state=observe(state);
 assert.equal(state.notifications.current,null);assert.equal(state.notifications.nextId,1);
 assert.match(state.notifications.feedback,/published persistent-problem/);
 state=observe(withPersistent(state));assert.equal(state.notifications.items.length,1);
 const id=state.notifications.current.id;
 state=observe(M.reduce(state,{type:'sample-notification-persistent',value:false}));
 state=observe(withPersistent(state));assert.equal(state.notifications.items.length,1);assert.equal(state.notifications.nextId,2);
 assert.equal(state.notifications.items[0].id,id);
});

test('a repeated incident, navigation and dismissal do not create another notification or resolve the problem',()=>{
 let state=observe(M.initialState('conflict'));
 const raw=JSON.stringify(state.scenario),id=state.notifications.current.id;
 for(const action of [{type:'page',page:'settings'},{type:'view-mode',mode:'power'},{type:'settings',value:'notifications'},{type:'page',page:'statistics'}])state=M.reduce(state,action);
 assert.equal(state.notifications.items.length,1);
 state=observe(state);assert.equal(state.notifications.items.length,1);
 state=M.reduce(state,{type:'notification-dismiss',id});assert.equal(state.notifications.current,null);
 state=observe(state);assert.equal(state.notifications.current,null);assert.equal(state.notifications.items.length,1);
 assert.equal(JSON.stringify(state.scenario),raw);assert.match(state.notifications.feedback,/same incident|same.*incident/i);
});

test('Windows blocks, DND, unknown availability and a missing presenter do not consume the first eligible notification',()=>{
 for(const os of Object.keys(M.NOTIFICATION_OS).filter(x=>x!=='allowed')){
  let state=observe(setOS(M.initialState('expired'),os));
  assert.equal(state.notifications.current,null,os);assert.equal(state.notifications.announced,false,os);
  assert.equal(state.notifications.nextId,1);assert.equal(state.notifications.items.length,0);
  state=observe(setOS(state,'allowed'));assert.equal(state.notifications.current.id,'nt-1',os);
 }
 let state=M.reduce(M.initialState('expired'),{type:'sample-notification-presenter',value:'missing'});
 state=observe(state);assert.equal(state.notifications.current,null);assert.equal(state.notifications.announced,false);
 state=observe(M.reduce(state,{type:'sample-notification-presenter',value:'active'}));assert.equal(state.notifications.current.id,'nt-1');
});

test('returning readiness never repeats an already shown incident and master Off suppresses both real and test samples',()=>{
 let state=observe(M.initialState('expired'));const id=state.notifications.current.id;
 state=setOS(state,'blocked-policy');assert.equal(state.notifications.current,null);
 state=observe(setOS(state,'allowed'));assert.equal(state.notifications.current,null);assert.equal(state.notifications.items.length,1);
 state=notificationOff(state);const evidence=JSON.stringify(state.scenario);
 state=observe(state);state=M.reduce(state,{type:'notification-test'});
 assert.equal(state.notifications.current,null);assert.equal(state.notifications.items.length,1);assert.equal(JSON.stringify(state.scenario),evidence);
 state=observe(M.reduce(state,{type:'notifications-enabled',value:true}));assert.equal(state.notifications.current,null);
 assert.equal(state.notifications.items[0].id,id);
 let first=observe(notificationOff(M.initialState('expired')));assert.equal(first.notifications.announced,false);
 first=observe(M.reduce(first,{type:'notifications-enabled',value:true}));assert.equal(first.notifications.current.id,'nt-1');
});

test('test notifications use separate bounded identities and never acknowledge a real problem or verify time',()=>{
 let state=M.initialState('conflict');const raw=JSON.stringify(state.scenario);
 for(let i=0;i<10;i++)state=M.reduce(state,{type:'notification-test'});
 assert.equal(state.notifications.items.length,8);assert.equal(state.notifications.announced,false);assert.equal(state.notifications.incident,null);
 assert.match(state.notifications.current.title,/Test/);assert.match(state.notifications.current.detail,/does not verify clock synchronization/);
 assert.equal(JSON.stringify(state.scenario),raw);
 const testId=state.notifications.current.id;
 state=observe(state);assert.equal(state.notifications.items.length,8);assert.equal(state.notifications.current.test,false);assert.notEqual(state.notifications.current.id,testId);
});

test('fresh notification activation only opens current evidence and never submits service or clock operations',()=>{
 for(const [scene,page,section,modal] of [['conflict','statistics',null,null],['expired','diagnostics',null,null],['blocked','settings','service',null],['signed-conflict','recovery',null,null],['correction','status',null,'correction']]){
  const state=observe(M.initialState(scene)),raw=JSON.stringify(state.scenario),service=JSON.stringify(state.serviceControl),management=JSON.stringify(state.timeManagement);
  const opened=M.reduce(state,{type:'notification-open',id:state.notifications.current.id});
  assert.equal(opened.page,page,scene);if(section)assert.equal(opened.settingsSection,section);assert.equal(opened.modal,modal);
  assert.equal(JSON.stringify(opened.scenario),raw);assert.equal(JSON.stringify(opened.serviceControl),service);assert.equal(JSON.stringify(opened.timeManagement),management);
 }
 const tested=M.reduce(M.initialState(),{type:'notification-test'});
 const opened=M.reduce(tested,{type:'notification-open',id:tested.notifications.current.id});
 assert.equal(opened.page,'settings');assert.equal(opened.settingsSection,'notifications');assert.match(opened.notice,/does not confirm synchronization/);
});

test('stale and invalid activations cannot revive an expired correction, old ownership generation or expired notification identity',()=>{
 const old=observe(M.initialState('correction')),id=old.notifications.current.id;
 for(const updated of [
  {...M.initialState('correction-expired'),notifications:old.notifications},
  {...old,scenario:{...old.scenario,proposalExpired:true}},
  {...old,timeManagement:{...old.timeManagement,generation:13}},
  {...old,scenario:{...old.scenario,proposalId:'new-proposal'}}
 ]){
  const next=M.reduce(updated,{type:'notification-open',id});assert.equal(next.modal,null);assert.equal(next.page,'status');assert.match(next.notice,/earlier observation/);
 }
 for(const id of ['nt-0','nt-999','__proto__','constructor','https://example.test','<img>',{x:1},null])assert.equal(M.reduce(old,{type:'notification-open',id}),old);
});

test('a genuinely new ownership generation or reconciled operation episode is not deduplicated as the previous incident',()=>{
 const first=observe(M.initialState('correction')),oldId=first.notifications.current.id;
 let fresh={...first,timeManagement:{...first.timeManagement,generation:13}};
 fresh=observe(fresh);assert.notEqual(fresh.notifications.current.id,oldId);assert.equal(fresh.notifications.episode,2);
 const oldRuntime=observe(runtime(base(),'stop','unknown'));
 const reconciled={...base(),notifications:oldRuntime.notifications};
 const newRuntime=runtime(reconciled,'stop','unknown');
 assert.equal(M.reduce(newRuntime,{type:'notification-open',id:oldRuntime.notifications.current.id}).modal,null);
 const reported=observe(newRuntime);assert.equal(reported.notifications.items.length,2);assert.notEqual(reported.notifications.current.context,oldRuntime.notifications.current.context);
});

test('intentional transitions stay quiet while partial and unknown operation outcomes notify in their own scope',()=>{
 const healthy=base(),windows=lifecycle(healthy,'use-windows');
 for(const state of [runtime(healthy,'stop'),runtime(healthy,'restart'),lifecycle(healthy,'disable'),windows,lifecycle(windows,'use-nevrotick'),lifecycle(healthy,'startup-off')])assert.equal(observe(state).notifications.current,null);
 for(const [operation,response] of [['disable','cleanup-unconfirmed'],['use-windows','windows-unavailable'],['use-windows','unknown']]){
  const state=observe(lifecycle(healthy,operation,response));assert.match(state.notifications.current.title,/incomplete|unknown/);assert.equal(state.notifications.current.target,'service');
 }
 const unknownObserver=observe(runtime(windows,'start','unknown'));
 assert.match(unknownObserver.notifications.current.detail,/Windows Time remains.*manager/);assert.doesNotMatch(unknownObserver.notifications.current.detail,/synchronization is off/);
 const startup=observe(lifecycle(healthy,'startup-off','unknown'));
 assert.equal(startup.notifications.current.key,'autostart-outcome-unknown');assert.match(startup.notifications.current.detail,/timekeeping is unaffected/);
});

test('cancelled or rejected commands stay inline and never conceal an existing important clock incident',()=>{
 for(const result of ['uac-cancelled','failed']){
  assert.equal(observe(runtime(base(),'stop',result)).notifications.current,null);
  assert.equal(observe(lifecycle(base(),'disable',result)).notifications.current,null);
  assert.equal(observe(lifecycle(base(),'startup-off',result)).notifications.current,null);
 }
 for(const id of ['expired','conflict'])for(const response of ['uac-cancelled','failed','unknown']){
  const state=observe(lifecycle(base(id),'startup-off',response));
  assert.equal(state.notifications.current.key,id==='expired'?'estimate-unusable':'source-conflict');
 }
});

test('historical recovery records, an expired proposal and a future reserve fixture do not generate current problem prompts',()=>{
 for(const id of ['healthy','roughtime-recovered','recovered','reserve','correction-expired']){
  const state=M.initialState(id);
  state.scenario.roughtimeEvidence={...M.getScenario('signed-conflict').roughtimeEvidence,historical:true};
  assert.equal(observe(withPersistent(state)).notifications.current,null,id);
 }
 const signed=M.initialState('signed-conflict');signed.scenario.roughtimeEvidence.historical=true;
 assert.equal(observe(withPersistent(signed)).notifications.current,null);
});

test('scene/reset guards clear independent samples but preserve preferences and monotonic IDs so old activation cannot alias new content',()=>{
 let state=observe(M.initialState('conflict')),oldId=state.notifications.current.id;
 state=notificationOff(state);state=M.reduce(state,{type:'scenario',id:'expired'});
 assert.equal(state.modal,'reset-confirm');state=M.reduce(state,{type:'confirm-reset'});
 assert.equal(state.notifications.enabled,false);assert.equal(state.notifications.items.length,0);
 state=M.reduce(state,{type:'notifications-enabled',value:true});state=observe(state);
 assert.notEqual(state.notifications.current.id,oldId);assert.equal(M.reduce(state,{type:'notification-open',id:oldId}),state);
 const secondId=state.notifications.current.id;
 state=M.reduce(state,{type:'reset'});assert.equal(state.modal,'reset-confirm');state=M.reduce(state,{type:'confirm-reset'});
 state=observe(state);assert.notEqual(state.notifications.current.id,secondId);assert.equal(state.notifications.enabled,true);
});

test('notification activation uses the existing dirty recovery-editor leave guard and preserves a kept draft',()=>{
 let state=observe(base('conflict'));const id=state.notifications.current.id;
 state=M.reduce(state,{type:'power-page',page:'recovery'});state=M.reduce(state,{type:'recovery-edit'});
 state=M.reduce(state,{type:'recovery-input',count:'1',helpers:['helper.example','']});
 const guarded=M.reduce(state,{type:'notification-open',id});assert.equal(guarded.leaveGuard.kind,'recovery-editor');
 const cancelled=M.reduce(guarded,{type:'recovery-stay'});assert.equal(cancelled.modal,'recovery-editor');assert.equal(cancelled.notifications.current.id,id);
 const kept=M.reduce(guarded,{type:'recovery-keep-leave'});assert.equal(kept.leaveGuard,null);assert.equal(kept.page,'statistics');assert.deepEqual(kept.recoverySavedInput,{count:'1',helpers:['helper.example','']});
});

test('notification payloads omit private source data and expose no privileged action even for an actionable correction',()=>{
 for(const id of ['conflict','expired','correction','outcome-unknown']){
  const state=M.initialState(id);state.scenario.sources[0].host='private.corp.example';state.scenario.events[0].detail='private-user secret-token C:\\Users\\private';
  const result=observe(state),card=V.notificationPreview(result);
  assert.doesNotMatch(card,/private.corp|private-user|secret-token|Users/);
  assert.doesNotMatch(card,/data-action="(?:service-request|service-confirm|lifecycle-request|lifecycle-confirm|approve|apply)/);
  assert.match(card,/Windows notification preview/);assert.match(card,/no system notification/);
 }
});

test('Windows-observer unknown feedback and tray accessibility references agree with the narrowed state',()=>{
 const unknown=runtime(lifecycle(base(),'use-windows'),'start','unknown');
 assert.match(V.serviceFeedback(unknown),/Windows Time remains the independently confirmed clock manager/);
 assert.doesNotMatch(V.serviceFeedback(unknown),/clock ownership are unknown/);
 assert.match(V.simpleSettings(unknown),/Windows Time continues to manage time/);
 const tray=V.render({...M.initialState(),mode:'power',page:'settings',settingsSection:'preferences',modal:'tray-sample',sampleTray:true});
 const modal=tray.match(/<dialog[\s\S]*?<\/dialog>/)[0];assert.doesNotMatch(modal,/aria-describedby="sync-response"/);
});

test('pending gaps and cancelled or rejected attempts do not retire and reannounce an existing clock incident',()=>{
 for(const id of ['conflict','expired'])for(const result of ['uac-cancelled','failed']){
  let state=observe(base(id));const oldId=state.notifications.current.id;
  state=M.reduce(M.reduce(state,{type:'service-request',operation:'stop'}),{type:'service-confirm'});
  state=observe(state);assert.equal(state.notifications.current,null);assert.equal(state.notifications.announced,true);
  state=M.reduce(M.reduce(state,{type:'sample-service-response',value:result}),{type:'sample-service-deliver'});
  state=observe(state);assert.equal(state.notifications.current,null);assert.equal(state.notifications.items.length,1);assert.equal(state.notifications.items[0].id,oldId);
 }
 const first=observe(M.initialState('conflict'));
 const tracking=observe({...M.initialState('healthy'),notifications:first.notifications});
 assert.equal(tracking.notifications.announced,false);assert.equal(tracking.notifications.current,null);
 const newIncident=observe({...M.initialState('conflict'),notifications:tracking.notifications});
 assert.equal(newIncident.notifications.items.length,2);assert.notEqual(newIncident.notifications.current.id,first.notifications.current.id);
});

test('an unresolved partial handover survives cancelled and rejected retries without losing a first alert or duplicating an announced incident',()=>{
 for(const [operation,partial] of [['use-windows','windows-unavailable'],['use-nevrotick','product-unavailable']])for(const result of ['close','uac-cancelled','failed'])for(const shown of [false,true]){
  let state=operation==='use-windows'?base():lifecycle(base(),'use-windows');
  state=lifecycle(state,operation,partial);
  state=shown?observe(state):observe(setOS(state,'blocked-app'));
  const previous=state.notifications.items[0]?.id;
  state=M.reduce(state,{type:'lifecycle-request',operation});
  if(result==='close')state=M.reduce(state,{type:'close'});
  else{
   state=M.reduce(state,{type:'lifecycle-confirm'});state=observe(state);
   state=M.reduce(M.reduce(state,{type:'sample-lifecycle-response',value:result}),{type:'sample-lifecycle-deliver'});
  }
  assert.equal(M.presentState(state).scenario.id,'no-synchronizer');
  assert.equal(M.notificationCandidate(state).key,'handover-'+partial);
  state=observe(setOS(state,'allowed'));assert.equal(state.notifications.items.length,1);
  if(shown){assert.equal(state.notifications.items[0].id,previous);if(state.notifications.current)assert.equal(state.notifications.current.id,previous)}
  else assert.equal(state.notifications.current.key,'handover-'+partial);
 }
});

test('unrelated runtime attempts do not replace the identity of an unresolved autostart outcome',()=>{
 for(const result of ['close','uac-cancelled','failed'])for(const shown of [false,true]){
  let state=lifecycle(base(),'startup-off','unknown');
  state=shown?observe(state):observe(setOS(state,'unknown'));
  const original=state.notifications.items[0]?.id,context=M.notificationCandidate(state).scopeSerial;
  state=M.reduce(state,{type:'service-request',operation:'stop'});
  if(result==='close')state=M.reduce(state,{type:'close'});
  else{state=M.reduce(state,{type:'service-confirm'});state=observe(state);state=M.reduce(M.reduce(state,{type:'sample-service-response',value:result}),{type:'sample-service-deliver'});}
  assert.equal(M.notificationCandidate(state).scopeSerial,context);
  state=observe(setOS(state,'allowed'));assert.equal(state.notifications.items.length,1);
  if(shown){assert.equal(state.notifications.items[0].id,original);if(state.notifications.current)assert.equal(state.notifications.current.id,original)}
  else assert.equal(state.notifications.current.key,'autostart-outcome-unknown');
 }
});
