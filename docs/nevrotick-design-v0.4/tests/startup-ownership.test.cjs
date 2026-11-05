'use strict';
const test=require('node:test'), assert=require('node:assert/strict');
const M=require('../src/model.js'),V=require('../src/views.js');
const base=(id='healthy')=>M.reduce(M.initialState(id),{type:'service-settings'});
const request=(s,operation)=>M.reduce(s,{type:'lifecycle-request',operation});
const submit=(s,operation)=>M.reduce(request(s,operation),{type:'lifecycle-confirm'});
const deliver=(s,value='confirmed')=>M.reduce(M.reduce(s,{type:'sample-lifecycle-response',value}),{type:'sample-lifecycle-deliver'});
const run=(s,operation,value)=>deliver(submit(s,operation),value);

test('startup Off is configuration only through pending, success and unknown; active DEBUG and clock evidence survive',()=>{
  let s=M.reduce(M.reduce(base('roughtime-recovered'),{type:'sample-log-scope',value:'both'}),{type:'sample-log-start'});
  for(const response of ['confirmed','unknown','failed','uac-cancelled']){
    const pending=submit(s,'startup-off'), after=deliver(pending,response);
    for(const current of [pending,after]){
      assert.deepEqual(current.serviceControl,s.serviceControl);
      assert.deepEqual(current.sampleLog,s.sampleLog);
      assert.deepEqual(M.serviceLog(current),s.sampleLog);
      assert.deepEqual(M.presentState(current).scenario,s.scenario);
      assert.equal(current.timeManagement.owner,'nevrotick');
      assert.equal(current.timeManagement.generation,12);
    }
    assert.equal(M.startupLabel(after),response==='unknown'?'Unknown':response==='confirmed'?'Off':'On');
    if(response==='unknown') assert.match(M.lifecyclePermission(after,'startup-on'),/unknown/i);
  }
});

test('startup On does not start a stopped pair and ordinary Stop leaves autostart unchanged',()=>{
  let s=base('service-stopped');
  s=run(s,'startup-off');s=run(s,'startup-on');
  assert.equal(s.serviceControl.observation,'stopped');assert.equal(M.startupLabel(s),'On');
  const stopped=M.reduce(M.reduce(M.reduce(base(),{type:'service-request',operation:'stop'}),{type:'service-confirm'}),{type:'sample-service-deliver'});
  assert.equal(M.startupLabel(stopped),'On');
});

test('full Disable requires explicit consent and independently confirms stopped, disabled, restart inhibition and owned cleanup',()=>{
  const s=base('roughtime-recovered'), confirming=request(s,'disable');
  assert.match(V.modal(confirming),/Disable NevroTick/);assert.match(V.modal(confirming),/automatic synchronization will stop/i);
  const cancelled=M.reduce(confirming,{type:'close'});
  assert.deepEqual(cancelled.serviceControl,s.serviceControl);assert.equal(cancelled.timeManagement.owner,'nevrotick');
  const disabled=run(s,'disable');
  assert.equal(disabled.serviceControl.observation,'stopped');
  assert.deepEqual(disabled.timeManagement.startup,{measurement:'disabled',clock:'disabled'});
  assert.equal(disabled.timeManagement.restartInhibited,true);assert.equal(disabled.timeManagement.cleanup,'confirmed');
  assert.equal(disabled.timeManagement.fullyDisabled,true);assert.equal(disabled.timeManagement.owner,'none');
  assert.equal(disabled.timeManagement.windows.state,'stopped');
  assert.deepEqual(disabled.scenario,s.scenario);
  assert.match(M.presentState(disabled).scenario.title,/No active synchronizer/);
  assert.match(M.summary(disabled),/Automatic synchronization is off/);
});

test('Enable and start uses manual startup and never acquires a clock owner or fresh evidence',()=>{
  const s=run(base(),'disable');
  const confirm=M.reduce(s,{type:'service-request',operation:'start'});
  assert.match(V.modal(confirm),/Enable and start/);
  const started=M.reduce(M.reduce(confirm,{type:'service-confirm'}),{type:'sample-service-deliver'});
  assert.equal(started.serviceControl.observation,'running');assert.equal(M.startupLabel(started),'Off');
  assert.deepEqual(started.timeManagement.startup,{measurement:'manual',clock:'manual'});
  assert.equal(started.timeManagement.owner,'none');assert.equal(started.serviceControl.ownershipKnown,false);
  assert.equal(M.presentState(started).scenario.offset,null);assert.equal(M.presentState(started).scenario.agree,0);
});

test('Windows handover releases before activation and confirms owner separately from running or sync',()=>{
  const s=base('roughtime-recovered'), confirmed=run(s,'use-windows');
  assert.equal(confirmed.timeManagement.owner,'windows');assert.equal(confirmed.timeManagement.ownerKnown,true);
  assert.equal(confirmed.timeManagement.windows.state,'running');assert.equal(confirmed.timeManagement.windows.startup,'manual-trigger');
  assert.equal(confirmed.timeManagement.windows.synchronization,'unverified');assert.equal(confirmed.timeManagement.cleanup,'confirmed');
  assert.equal(confirmed.timeManagement.fullyDisabled,true);assert.equal(confirmed.timeManagement.restartInhibited,true);
  assert.equal(confirmed.serviceControl.observation,'stopped');assert.equal(M.startupLabel(confirmed),'Off');
  assert.deepEqual(confirmed.scenario,s.scenario);
  const shown=M.presentState(confirmed).scenario;
  assert.match(shown.title,/Windows Time manages time/);assert.equal(shown.offset,null);assert.equal(shown.agree,0);assert.match(shown.authentication,/Historical/);
  for(const page of ['status','settings','sources','statistics','diagnostics']){
    const html=V.render({...confirmed,page});assert.match(html,/Windows Time/);assert.doesNotMatch(html,/<h2 id="state-title">Time sync active/);
  }
  assert.match(M.summary(confirmed),/Clock ownership: Windows Time/);assert.match(M.summary(confirmed),/Windows synchronization: Not yet verified/);
  assert.match(V.modal({...confirmed,modal:'status-help'}),/Windows Time/);
});

test('host preflight blocks before effects and managed policy is allowed when it permits the transition',()=>{
  for(const condition of ['policy-blocked','dependency-unavailable','dependency-unknown','ownership-uncertain']){
    const s=M.reduce(base(),{type:'sample-lifecycle-preflight',value:condition}), confirming=request(s,'use-windows');
    assert.equal(confirming.modal,'lifecycle-confirm');
    assert.match(V.modal(confirming),/Cannot continue/);
    assert.equal(M.reduce(confirming,{type:'lifecycle-confirm'}),confirming);
    assert.deepEqual(confirming.serviceControl,s.serviceControl);assert.equal(confirming.timeManagement.owner,'nevrotick');
  }
  const managed=M.reduce(base(),{type:'sample-lifecycle-preflight',value:'managed-allowed'});
  assert.equal(run(managed,'use-windows').timeManagement.owner,'windows');
  assert.doesNotMatch(V.modal(request(base(),'use-windows')),/<select/);
});

test('cleanup partial never enables Windows or claims fully disabled; failure after release has no active synchronizer',()=>{
  const partial=run(base(),'use-windows','cleanup-unconfirmed');
  assert.equal(partial.serviceControl.observation,'stopped');assert.equal(partial.timeManagement.cleanup,'unconfirmed');
  assert.equal(partial.timeManagement.windows.state,'stopped');assert.equal(partial.timeManagement.owner,'unknown');
  assert.equal(partial.timeManagement.fullyDisabled,false);assert.match(V.lifecycleFeedback(partial),/Partial/);
  assert.ok(M.lifecyclePermission(partial,'use-windows'));
  const noWindows=run(base(),'use-windows','windows-unavailable');
  assert.equal(noWindows.timeManagement.cleanup,'confirmed');assert.equal(noWindows.timeManagement.owner,'none');
  assert.equal(noWindows.timeManagement.windows.state,'failed');assert.equal(noWindows.timeManagement.fullyDisabled,true);
  assert.match(M.presentState(noWindows).scenario.title,/No active synchronizer/);
  assert.match(V.lifecycleFeedback(noWindows),/Windows Time unavailable/);
});

test('unknown and rejected handovers neither infer success nor permit blind repeat',()=>{
  for(const result of ['uac-cancelled','failed']){
    const s=base(),after=run(s,'use-windows',result);
    assert.deepEqual(after.serviceControl,s.serviceControl);assert.deepEqual(after.scenario,s.scenario);
    assert.equal(after.timeManagement.owner,'nevrotick');assert.equal(after.timeManagement.windows.state,'stopped');
  }
  const unknown=run(base(),'use-windows','unknown');
  assert.equal(unknown.timeManagement.owner,'unknown');assert.equal(unknown.timeManagement.windows.state,'unknown');
  for(const op of ['disable','use-windows','use-nevrotick','startup-on']) assert.ok(M.lifecyclePermission(unknown,op));
  assert.match(V.lifecycleFeedback(unknown),/Outcome unknown/);assert.equal(M.presentState(unknown).scenario.offset,null);
});

test('Start and autostart while Windows owns time only start observation; Disable preserves Windows',()=>{
  const windows=run(base(),'use-windows');
  const started=M.reduce(M.reduce(M.reduce(windows,{type:'service-request',operation:'start'}),{type:'service-confirm'}),{type:'sample-service-deliver'});
  assert.equal(started.timeManagement.owner,'windows');assert.equal(started.timeManagement.windows.state,'running');
  assert.equal(started.timeManagement.windows.startup,'manual-trigger');assert.equal(M.startupLabel(started),'Off');
  assert.match(M.presentState(started).scenario.title,/Windows Time manages time/);
  const automatic=run(started,'startup-on');assert.equal(automatic.timeManagement.owner,'windows');
  const disabled=run(automatic,'disable');assert.equal(disabled.timeManagement.owner,'windows');assert.equal(disabled.timeManagement.windows.state,'running');
  assert.match(M.presentState(disabled).scenario.title,/Windows Time manages time/);
  assert.doesNotMatch(V.lifecycleFeedback(disabled),/Automatic synchronization is off/);
});

test('explicit reverse disables Windows triggers, preserves its allowed configuration and confirms a new owner generation only',()=>{
  const windows=run(base('roughtime-recovered'),'use-windows'), reverse=run(windows,'use-nevrotick');
  assert.equal(reverse.timeManagement.windows.state,'stopped');assert.equal(reverse.timeManagement.windows.startup,'disabled');
  assert.equal(reverse.timeManagement.windows.restoreMode,'manual-trigger');assert.equal(reverse.timeManagement.owner,'nevrotick');
  assert.equal(reverse.timeManagement.generation,13);assert.equal(reverse.serviceControl.ownershipKnown,true);
  assert.equal(reverse.serviceControl.observation,'running');assert.equal(M.startupLabel(reverse),'Off');
  assert.equal(M.presentState(reverse).scenario.offset,null);assert.equal(M.presentState(reverse).scenario.agree,0);
  assert.match(M.presentState(reverse).scenario.title,/Reacquiring time/);
  assert.deepEqual(reverse.scenario,windows.scenario);
  const partial=run(windows,'use-nevrotick','product-unavailable');
  assert.equal(partial.timeManagement.owner,'none');assert.equal(partial.timeManagement.windows.state,'stopped');
  assert.equal(partial.serviceControl.observation,'stopped');assert.equal(partial.timeManagement.generation,12);
});

test('one mutation across runtime/config/ownership; Diagnose, navigation and reset retain pending',()=>{
  for(const op of ['startup-off','disable','use-windows']){
    const pending=submit(base(),op);
    assert.ok(M.servicePermission(pending,'stop'));assert.equal(request(pending,'use-windows'),pending);
    const diagnosed=M.reduce(pending,{type:'service-diagnose'});assert.equal(diagnosed.timeManagement.phase,'pending');
    const guard=M.reduce(diagnosed,{type:'reset'});assert.equal(guard.modal,'reset-confirm');
    assert.equal(M.reduce(guard,{type:'cancel-reset'}).timeManagement.phase,'pending');
  }
});

test('read-only profile and unknown ownership cannot be converted into clock authority',()=>{
  const observe=base('observe');assert.ok(M.lifecyclePreflight(observe,'use-nevrotick').reason);
  const unknown=base('stale');assert.ok(M.lifecyclePreflight(unknown,'use-windows').reason);
  const s=base();for(const operation of ['W32Time','use-arbitrary-service',{},null])assert.equal(request(s,operation),s);
  assert.equal(M.reduce(s,{type:'sample-lifecycle-response',value:'synchronized'}),s);
  const html=V.render(s),product=html.slice(html.indexOf('<div class="app-window'));
  assert.doesNotMatch(product,/sample-lifecycle-preflight|sample-lifecycle-response/);
});

test('startup re-enable pending or unknown withdraws full-disable confirmation without changing runtime or owner',()=>{
  const disabled=run(base(),'disable'), pending=submit(disabled,'startup-on');
  assert.equal(pending.timeManagement.fullyDisabled,false);
  const unknown=deliver(pending,'unknown');
  assert.equal(unknown.timeManagement.fullyDisabled,false);assert.match(M.summary(unknown),/Fully disabled: false/);
  assert.deepEqual(unknown.serviceControl,disabled.serviceControl);assert.equal(unknown.timeManagement.owner,'none');
  for(const result of ['uac-cancelled','failed'])assert.equal(deliver(pending,result).timeManagement.fullyDisabled,true);
  assert.equal(deliver(pending).timeManagement.fullyDisabled,false);
});

test('Simple and Stop/Restart confirmations preserve an existing Windows time manager without Start-only prose',()=>{
  const windows=run(base(),'use-windows');
  const simple=V.simpleSettings({...windows,mode:'simple'});
  assert.match(simple,/Windows Time continues to manage time/);assert.doesNotMatch(simple,/Automatic synchronization is paused/);
  const observer=M.reduce(M.reduce(M.reduce(windows,{type:'service-request',operation:'start'}),{type:'service-confirm'}),{type:'sample-service-deliver'});
  for(const operation of ['stop','restart']){
    const html=V.modal(M.reduce(observer,{type:'service-request',operation}));
    assert.match(html,/Windows Time continues to manage time/);
    assert.doesNotMatch(html,/clock may drift|NevroTick starts for observation only/);
  }
  assert.equal(M.reduce(windows,{type:'sync-now'}),windows);
  assert.equal(M.reduce(observer,{type:'sync-now'}).syncFeedback,'observation-check');
});

test('Windows-owned views scope NevroTick scheduling, profiles and recovery previews without editing Windows settings',()=>{
  const windows=run(base(),'use-windows');
  for(const section of ['schedule','policy','service']){
    const html=V.settings({...windows,settingsSection:section});
    assert.match(html,/Applies to NevroTick; Windows Time keeps its existing settings/);
  }
  assert.match(V.sources(windows),/NEVROTICK PROFILE · INACTIVE/);
  assert.match(V.recovery(windows),/Applies to NevroTick; Windows Time keeps its existing settings/);
  const draft=M.reduce({...windows,modal:'policy-editor'},{type:'policy-draft',value:'ntp'});
  assert.equal(draft.timeManagement.owner,'windows');assert.equal(draft.timeManagement.windows.startup,'manual-trigger');
  assert.equal(draft.policyDraft,'ntp');
});

test('disabled or cleanup-partial NevroTick cannot accept a measurement check or claim full disabling during Start',()=>{
  for(const s of [run(base(),'disable'),run(base(),'use-windows','cleanup-unconfirmed')]){
    assert.equal(M.reduce(s,{type:'sync-now'}).syncFeedback,'service-stopped');
    assert.doesNotMatch(V.status(s),/data-action="sync-now"/);
  }
  const disabled=run(base(),'disable'),pending=M.reduce(M.reduce(disabled,{type:'service-request',operation:'start'}),{type:'service-confirm'});
  assert.equal(pending.timeManagement.fullyDisabled,false);
  const rejected=M.reduce(M.reduce(pending,{type:'sample-service-response',value:'failed'}),{type:'sample-service-deliver'});
  assert.equal(rejected.timeManagement.fullyDisabled,true);
  assert.match(M.summary(run(base(),'use-windows')),/Stored NevroTick profile \(inactive\): NTS only/);
});

test('lost acknowledgement for Enable and start invalidates only the startup facts it might have changed',()=>{
  const disabled=run(base(),'disable');assert.equal(M.fullDisableConfirmed(disabled),true);
  const start=s=>M.reduce(M.reduce(s,{type:'service-request',operation:'start'}),{type:'service-confirm'});
  const answer=(s,result)=>M.reduce(M.reduce(s,{type:'sample-service-response',value:result}),{type:'sample-service-deliver'});
  const pending=start(disabled);assert.equal(M.fullDisableConfirmed(pending),false);
  const unknown=answer(pending,'unknown');
  assert.deepEqual(unknown.timeManagement.startup,{measurement:'unknown',clock:'unknown'});
  assert.equal(M.fullDisableConfirmed(unknown),false);assert.match(M.summary(unknown),/Fully disabled: false/);
  for(const result of ['failed','uac-cancelled']){
    const rejected=answer(pending,result);assert.equal(M.fullDisableConfirmed(rejected),true);
    assert.deepEqual(rejected.timeManagement.startup,disabled.timeManagement.startup);
  }
  const ordinary=answer(start(base('service-stopped')),'unknown');
  assert.deepEqual(ordinary.timeManagement.startup,{measurement:'automatic',clock:'automatic'});
});

test('Windows ownership stays confirmed while NevroTick runtime observation becomes pending without presenting its last state as current',()=>{
  const windows=run(base(),'use-windows');
  const pending=M.reduce(M.reduce(windows,{type:'service-request',operation:'start'}),{type:'service-confirm'});
  assert.equal(pending.timeManagement.owner,'windows');assert.equal(pending.timeManagement.ownerKnown,true);
  assert.match(M.presentState(pending).scenario.service,/pending/);
  assert.match(M.summary(pending),/Service: NevroTick operation pending/);
  const simple=V.simpleSettings({...pending,mode:'simple'});
  assert.match(simple,/operation is pending; last confirmed service state: stopped/);
  assert.doesNotMatch(simple,/NevroTick background services are stopped/);
  assert.match(V.render(pending),/Windows Time · NevroTick operation pending/);
});

test('startup-only pending after full disable or Windows handover retains the same current runtime and controller presentation',()=>{
  for(const state of [run(base(),'disable'),run(base(),'use-windows')]){
    const pending=submit(state,'startup-on');
    assert.deepEqual(M.presentState(pending).scenario,M.presentState(state).scenario);
    assert.equal(M.runtimeObservation(pending).short,M.runtimeObservation(state).short);
  }
});

test('retained event journals qualify prior runtime snapshots while startup-only changes preserve the existing evidence scope and raw facts',()=>{
  const initial=base('roughtime-recovered'), raw=JSON.parse(JSON.stringify(initial.scenario.events));
  const runtimePending=M.reduce(M.reduce(initial,{type:'service-request',operation:'restart'}),{type:'service-confirm'});
  const runtime=M.reduce(runtimePending,{type:'sample-service-deliver'});
  const windows=run(initial,'use-windows');
  const cases=[
    [initial,false], [submit(initial,'startup-off'),false], [run(initial,'startup-off'),false],
    [runtimePending,true], [runtime,true], [windows,true],
    [run(runtime,'startup-off'),true], [submit(windows,'startup-on'),true]
  ];
  const eventList=html=>html.match(/<ol class="event-list">([\s\S]*?)<\/ol>/)[1];
  const original=eventList(V.diagnostics(initial));
  for(const [state,retained] of cases){
    const html=V.diagnostics(state);
    assert.match(html,new RegExp(retained?'Retained events · before service operation':'<h2>Recent events</h2>'));
    if(retained)assert.match(html,/Event ages and status wording are from the previous snapshot; they do not describe current synchronization\./);
    else assert.doesNotMatch(html,/Event ages and status wording are from the previous snapshot/);
    assert.deepEqual(state.scenario.events,raw);
    assert.equal(eventList(html),original,'event titles, details, ages and sequence remain exactly the original observation');
  }
});
