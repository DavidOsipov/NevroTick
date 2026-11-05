'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const M = require('../src/model.js');
const V = require('../src/views.js');
const settings = (id='healthy') => M.reduce(M.initialState(id),{type:'service-settings'});
const request = (state,operation) => M.reduce(state,{type:'service-request',operation});
const submit = (state,operation) => M.reduce(request(state,operation),{type:'service-confirm'});
const deliver = (state,result='confirmed') => M.reduce(M.reduce(state,{type:'sample-service-response',value:result}),{type:'sample-service-deliver'});

test('Simple explains the independent background service and links to useful Power controls',()=>{
  const simple = V.simpleSettings(M.initialState());
  assert.match(simple,/Background service/);
  assert.match(simple,/Closing the app does not change the service state/);
  assert.match(simple,/Manage service in Power user/);
  assert.doesNotMatch(simple,/data-action="service-request"/);
  const advanced = settings();
  assert.equal(advanced.mode,'power'); assert.equal(advanced.settingsSection,'service');
  const html = V.render(advanced);
  for(const label of ['Start','Stop','Restart','Diagnose']) assert.match(html,new RegExp('>'+label+'</button>'));
  assert.match(html,/Administrator required/);
  assert.match(html,/time-measurement and clock-control services together/);
  assert.doesNotMatch(html,/Installation &amp; support/);
});

test('receiving model rejects arbitrary commands, unconfirmed operations and duplicate pending requests',()=>{
  const base = settings();
  assert.equal(M.reduce(base,{type:'service-confirm'}),base);
  for(const operation of ['delete','W32Time','start & calc.exe',null,{},'Start']) assert.equal(request(base,operation),base);
  assert.equal(request(base,'start'),base);
  assert.equal(request(M.initialState(),'stop').modal,null);
  const pending = submit(base,'stop');
  assert.equal(pending.serviceControl.phase,'pending');
  for(const operation of ['start','stop','restart']) assert.equal(request(pending,operation),pending);
  assert.equal(M.reduce(pending,{type:'service-confirm'}),pending);
  assert.match(M.presentState(pending).scenario.title,/pending/);
  assert.equal(M.presentState(pending).scenario.offset,null);
  assert.doesNotMatch(V.render(pending),/<h2 id="state-title">Time sync active/);
});

test('Stop requires meaningful confirmation and cancellation preserves previous service and clock facts',()=>{
  const base=settings(), confirming=request(base,'stop');
  const dialog=V.modal(confirming);
  assert.match(dialog,/clock may drift/);
  assert.match(dialog,/Windows Time will not be enabled automatically/);
  assert.match(dialog,/settings and history are retained/);
  assert.equal((dialog.match(/<select/g)||[]).length,0,'outcomes cannot be chosen inside the product confirmation');
  const cancelled=M.reduce(confirming,{type:'close'});
  assert.equal(cancelled.modal,null);
  assert.equal(cancelled.serviceControl.observation,'running');
  assert.equal(cancelled.serviceControl.presentation,null);
  assert.deepEqual(cancelled.scenario,base.scenario);
  assert.match(V.serviceFeedback(cancelled),/No request was submitted/);
});

test('confirmed Stop removes current synchronization while preserving all raw evidence and history',()=>{
  const base={...settings(),policyDraft:'observe',frequencyPreview:'600'};
  const frozen=JSON.stringify(base.scenario);
  const stopped=deliver(submit(base,'stop'));
  assert.equal(stopped.serviceControl.observation,'stopped');
  assert.equal(JSON.stringify(stopped.scenario),frozen);
  assert.equal(stopped.policyDraft,'observe'); assert.equal(stopped.frequencyPreview,'600');
  const shown=M.presentState(stopped).scenario;
  assert.equal(shown.id,'service-stopped'); assert.equal(shown.offset,null); assert.equal(shown.agree,0);
  assert.ok(shown.sources.every(x=>x.measurementHistorical&&x.role==='Waiting'));
  assert.match(shown.ownership,/NevroTick/);
  assert.equal(shown.applied,base.scenario.applied);
  const {historical,...retainedRecovery}=shown.roughtimeEvidence;
  assert.equal(historical,true);
  assert.deepEqual(retainedRecovery,base.scenario.roughtimeEvidence);
  for(const mode of ['simple','power']) {
    const html=V.render({...stopped,mode,page:'status'});
    assert.match(html,/Background service is stopped/);
    assert.doesNotMatch(html,/Time sync active|Your clock is being kept in sync automatically/);
  }
  assert.match(V.render({...stopped,page:'sources'}),/Historical source snapshot/);
  assert.equal(M.reduce(stopped,{type:'sync-now'}).syncFeedback,'service-stopped');
});

test('Start from confirmed stopped with unknown ownership is available and remains read-only',()=>{
  const base=settings('service-stopped');
  assert.equal(base.serviceControl.ownershipKnown,false);
  assert.equal(M.servicePermission(base,'start'),'');
  const confirming=request(base,'start');
  assert.match(V.modal(confirming),/Start is read-only/);
  const started=deliver(M.reduce(confirming,{type:'service-confirm'}));
  assert.equal(started.serviceControl.observation,'running');
  assert.equal(started.serviceControl.ownershipKnown,false);
  assert.ok(started.serviceControl.ownership.includes(base.serviceControl.ownership));
  assert.match(M.servicePermission(started,'stop'),/ownership is unresolved/);
  assert.match(M.servicePermission(started,'restart'),/ownership is unresolved/);
  const shown=M.presentState(started).scenario;
  assert.equal(shown.offset,null); assert.equal(shown.agree,0);
  assert.match(shown.next,/Read-only startup/);
  assert.match(V.render({...started,mode:'simple',page:'status'}),/Reacquiring time/);
  assert.equal(M.reduce(started,{type:'sync-now'}).syncFeedback,'waiting');
  assert.deepEqual(started.scenario,base.scenario);
});

test('Restart requires fresh agreement and never refunds or rewrites retained authority/evidence',()=>{
  const base=settings('roughtime-recovered');
  const pending=submit(base,'restart');
  assert.match(V.modal(request(base,'restart')),/synchronization will be interrupted/);
  const restarted=deliver(pending);
  const shown=M.presentState(restarted).scenario;
  assert.equal(shown.agree,0); assert.equal(shown.offset,null);
  assert.equal(shown.applied,base.scenario.applied);
  assert.deepEqual(restarted.scenario,base.scenario);
  assert.equal(restarted.serviceControl.ownershipKnown,false);
  assert.match(restarted.serviceControl.ownership,/Unconfirmed after startup/);
  assert.ok(restarted.serviceControl.ownership.includes(base.serviceControl.ownership));
  const html=V.render({...restarted,page:'status'});
  assert.match(html,/Reacquiring time after service startup/);
  assert.doesNotMatch(html,/Your clock is being kept in sync automatically/);
});

test('external response cancellation or refusal restores prior facts; unknown cannot be retried',()=>{
  for(const result of ['uac-cancelled','failed']) {
    const base=settings(), after=deliver(submit(base,'restart'),result);
    assert.equal(after.serviceControl.observation,'running');
    assert.equal(after.serviceControl.presentation,null);
    assert.deepEqual(after.scenario,base.scenario);
    assert.match(V.serviceFeedback(after),/unchanged/);
    assert.equal(M.servicePermission(after,'restart'),'');
  }
  const base=settings(), unknown=deliver(submit(base,'stop'),'unknown');
  assert.equal(unknown.serviceControl.observation,'unknown');
  assert.equal(unknown.serviceControl.ownershipKnown,false);
  for(const operation of ['start','stop','restart']) assert.equal(request(unknown,operation),unknown);
  assert.match(V.serviceFeedback(unknown),/may have completed/);
  assert.equal(M.presentState(unknown).scenario.offset,null);
  assert.deepEqual(unknown.scenario,base.scenario);
  assert.equal(M.reduce(unknown,{type:'sync-now'}).syncFeedback,'status-unknown');
});

test('disconnection is unknown rather than stopped and Diagnose is a read-only evidence route',()=>{
  const base=settings('stale');
  assert.equal(base.serviceControl.observation,'unknown');
  for(const operation of ['start','stop','restart']) assert.ok(M.servicePermission(base,operation));
  const diagnosed=M.reduce(base,{type:'service-diagnose'});
  assert.equal(diagnosed.page,'diagnostics'); assert.equal(diagnosed.mode,'power');
  assert.equal(diagnosed.modal,null);
  assert.equal(diagnosed.serviceControl,base.serviceControl);
  assert.equal(diagnosed.scenario,base.scenario);
  const html=V.render(diagnosed);
  assert.match(html,/Background service evidence/);
  assert.match(html,/disconnected status channel does not prove/);
  assert.match(html,/Diagnose does not restart services/);
  assert.match(html,/Unknown ownership is preserved/);
});

test('sample delivery is external, finite and inert until a pending command exists',()=>{
  const base=settings();
  assert.equal(M.reduce(base,{type:'sample-service-deliver'}),base);
  assert.equal(M.reduce(base,{type:'sample-service-response',value:'pretend-synced'}),base);
  assert.equal(M.reduce(base,{type:'sample-service-response',value:{toString:()=> 'confirmed'}}),base);
  const pending=submit(base,'stop');
  const html=V.render(pending);
  const reviewer=html.slice(0,html.indexOf('<div class="app-window'));
  const product=html.slice(html.indexOf('<div class="app-window'));
  assert.match(reviewer,/id="sample-service-response"/);
  assert.doesNotMatch(product,/id="sample-service-response"/);
  assert.equal(pending.serviceControl.phase,'pending');
  assert.deepEqual(pending.scenario,base.scenario);
});

test('all public presentation and export paths use historical service evidence after an operation',()=>{
  const stopped=deliver(submit(settings('correction'),'stop'));
  assert.match(V.status(stopped),/Background service is stopped/);
  assert.match(V.sources(stopped),/Historical source snapshot/);
  assert.match(V.statistics(stopped),/Historical source evidence/);
  assert.match(V.diagnostics(stopped),/Background service is stopped/);
  assert.match(V.settings(stopped),/current clock effects or cleanup/);
  const exportText=M.summary(stopped);
  assert.match(exportText,/State: Background service is stopped/);
  assert.match(exportText,/Current clock effects unavailable/);
  assert.match(exportText,/Last known before Stop/);
  assert.match(V.modal({...stopped,modal:'export'}),/State: Background service is stopped/);
  assert.match(V.modal({...stopped,modal:'correction'}),/Expired proposal/);
  assert.doesNotMatch(V.modal({...stopped,modal:'correction'}),/42 s remaining/);
  assert.match(V.modal({...stopped,modal:'source-details',selectedSource:'alpha'}),/NTS · prior evidence only/);
  assert.equal(M.presentState(stopped).scenario.sources[0].reply,stopped.scenario.sources[0].reply);
});

test('export keeps original fixture identity through direct modal and page presentation',()=>{
  const stopped=deliver(submit(settings('healthy'),'stop'));
  const shown=M.presentState(stopped);
  assert.equal(shown.fixtureScenarioId,'healthy');
  for(const html of [V.modal({...stopped,modal:'export'}),V.render({...stopped,modal:'export'}),M.summary(shown)]){
    assert.match(html,/Scenario: healthy/);
    assert.match(html,/State: Background service is stopped/);
    assert.doesNotMatch(html,/Scenario: service-stopped/);
  }
});

test('cancellation and rejection before effect do not expire the original correction proposal',()=>{
  const base=settings('correction');
  const states=[M.reduce(request(base,'restart'),{type:'close'}),...['failed','uac-cancelled'].map(result=>deliver(submit(base,'restart'),result))];
  for(const state of states){
    assert.equal(state.serviceControl.presentation,null);
    assert.deepEqual(state.scenario,base.scenario);
    const dialog=V.modal({...state,modal:'correction'});
    assert.match(dialog,/42 s remaining in fixture/);
    assert.match(dialog,/NTS · 3 of 4 identities · majority met/);
    assert.doesNotMatch(dialog,/Expired proposal/);
  }
});

test('pending and unknown service examples survive navigation and need a confirmed reset',()=>{
  for(const original of [submit(settings(),'stop'),deliver(submit(settings(),'restart'),'unknown')]){
    const navigated=M.reduce(original,{type:'page',page:'status'});
    assert.equal(navigated.serviceControl,original.serviceControl);
    const warning=M.reduce(navigated,{type:'reset'});
    assert.equal(warning.modal,'reset-confirm');
    const cancelled=M.reduce(warning,{type:'cancel-reset'});
    assert.equal(cancelled.serviceControl,original.serviceControl);
    const reset=M.reduce(M.reduce(cancelled,{type:'reset'}),{type:'confirm-reset'});
    assert.equal(reset.serviceControl.phase,'idle');
    assert.equal(reset.serviceControl.presentation,null);
    assert.equal(reset.serviceControl.result,null);
  }
});

test('service operations expire only service DEBUG sessions; cancellation and pre-effect rejection preserve them',()=>{
  const active=scope=>M.reduce(M.reduce(settings(),{type:'sample-log-scope',value:scope}),{type:'sample-log-start'});
  for(const scope of ['ui','service','both']) {
    const base=active(scope);
    assert.equal(base.sampleLog.state,'active');
    for(const operation of ['stop','restart']) {
      const pending=submit(base,operation);
      const projected=M.serviceLog(pending);
      assert.equal(projected.results.ui,base.sampleLog.results.ui);
      assert.equal(projected.results.service,scope==='ui'?'not-requested':'unknown');
      assert.equal(M.reduce(request(base,operation),{type:'close'}).sampleLog,base.sampleLog);
      for(const result of ['failed','uac-cancelled']) {
        const after=deliver(pending,result);
        assert.equal(after.sampleLog,base.sampleLog);
        assert.deepEqual(M.serviceLog(after),base.sampleLog);
      }
      const done=deliver(pending);
      assert.equal(done.sampleLog.results.ui,base.sampleLog.results.ui);
      assert.equal(done.sampleLog.results.service,scope==='ui'?'not-requested':'expired');
      assert.equal(done.sampleLog.state,scope==='ui'?'active':scope==='both'?'partial':'expired');
      assert.equal(done.sampleLog.duration,base.sampleLog.duration);
      assert.equal(done.sampleLog.scope,base.sampleLog.scope);
      if(scope!=='ui') assert.match(V.diagnostics(done),/Prior session ended by service stop or restart/);
    }
  }
});

test('confirmed startup cannot promote a previous clock owner into current authority',()=>{
  const base=settings();
  const stopped=deliver(submit(base,'stop'));
  const started=deliver(submit(stopped,'start'));
  const restarted=deliver(submit(base,'restart'));
  for(const after of [started,restarted]) {
    assert.equal(after.serviceControl.observation,'running');
    assert.equal(after.serviceControl.ownershipKnown,false);
    assert.match(after.serviceControl.ownership,/Unconfirmed after startup · last known: NevroTick/);
    assert.match(M.presentState(after).scenario.next,/Read-only startup/);
    for(const operation of ['stop','restart']) assert.ok(M.servicePermission(after,operation));
    assert.equal(M.presentState(after).scenario.agree,0);
    assert.match(V.serviceFeedback(after),/Startup remains read-only/);
    assert.deepEqual(after.scenario,base.scenario);
  }
});

test('current service state gates new DEBUG collection and preserves uncertainty when stopping an unknown session',()=>{
  const stopped=deliver(submit(settings(),'stop'));
  const unknown=deliver(submit(settings(),'restart'),'unknown');
  for(const [base,response] of [[stopped,'unavailable'],[unknown,'unknown']]){
    assert.deepEqual(M.logResponseOptions(base,'service'),[response]);
    const only=M.reduce(base,{type:'sample-log-scope',value:'service'});
    assert.equal(M.reduce(only,{type:'sample-log-start'}),only);
    assert.match(V.modal({...only,modal:'debug-sample'}),/disabled[^>]*aria-label="Start example/);
    const both=M.reduce(M.reduce(base,{type:'sample-log-scope',value:'both'}),{type:'sample-log-start'});
    assert.equal(both.sampleLog.results.ui,'active');
    assert.equal(both.sampleLog.results.service,response);
    assert.equal(both.sampleLog.state,response==='unknown'?'unconfirmed':'partial');
  }
  const active=M.reduce(M.reduce(settings(),{type:'sample-log-scope',value:'both'}),{type:'sample-log-start'});
  const unknownActive=deliver(submit(active,'restart'),'unknown');
  assert.equal(unknownActive.sampleLog.results.service,'active','prior session remains in the raw sample history');
  assert.equal(M.serviceLog(unknownActive).results.service,'unknown','current presentation cannot assert it remains active');
  assert.doesNotMatch(V.modal({...unknownActive,modal:'debug-sample'}),/Sample DEBUG active<\/dd><\/div><div class="fact"><dt>Configured duration/);
  const stop=M.reduce(unknownActive,{type:'sample-log-stop'});
  assert.equal(stop.sampleLog.results.ui,'stopped');
  assert.equal(stop.sampleLog.results.service,'unknown');
  assert.equal(stop.sampleLog.state,'unconfirmed');
});
