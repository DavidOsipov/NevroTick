'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const M = require('../src/model.js');
const V = require('../src/views.js');

test('Sync now keeps status evidence frozen and repeats as Already checking without a dialog', () => {
  let state = M.initialState();
  const original = JSON.stringify(state.scenario);
  state = M.reduce(state, {type:'sync-now'});
  assert.equal(state.syncFeedback, 'checking');
  assert.equal(state.modal, null);
  assert.match(V.simpleStatus(state), /role="status"[^>]*aria-live="polite"/);
  assert.match(V.simpleStatus(state), /Sample response.*Checking/s);
  state = M.reduce(state, {type:'sync-now'});
  assert.equal(state.syncFeedback, 'already');
  assert.equal(JSON.stringify(state.scenario), original);
  assert.doesNotMatch(V.render(state).split('<div class="app-window')[1], /sample-sync-result|Example response/);
});

test('reviewer outcomes are limited by explicit command facts and do not infer measurement denial from clock writes', () => {
  const expectations = {'healthy':'checking','blocked':'checking','stale':'status-unknown','service-stopped':'service-stopped','conflict':'waiting','slewing':'correction','all-rate-limited':'rate-limited'};
  for (const [id, expected] of Object.entries(expectations)) {
    let state = M.initialState(id);
    assert.equal(state.scenario.syncResponse, expected, id);
    state = M.reduce(state,{type:'sample-sync',value:'rate-limited'});
    assert.equal(state.sampleSync, expected === 'rate-limited' ? 'rate-limited' : expected, id);
    state = M.reduce(state,{type:'sync-now'});
    assert.equal(state.syncFeedback,expected,id);
  }
  for (const [syncResponse,affectedSource,heading] of [['checking-available','delta','Checking'],['source-refused','beta','Source access refused']]) {
    let state=M.initialState();
    state={...state,scenario:{...state.scenario,syncResponse,affectedSource},sampleSync:syncResponse};
    state=M.reduce(state,{type:'sync-now'});
    const view=V.simpleStatus(state);
    assert.match(view,new RegExp(heading));
    assert.match(view,new RegExp(`data-action="affected-source" data-value="${affectedSource}"`));
  }
});

test('temporary diagnostics aggregates actual selected component outcomes and preserves them when closed', () => {
  let state=M.initialState();
  state=M.reduce(state,{type:'sample-log-response',component:'service',value:'refused'});
  state=M.reduce(state,{type:'sample-log-scope',value:'service'});
  state=M.reduce(state,{type:'sample-log-duration',value:'15'});
  state=M.reduce(state,{type:'sample-log-start'});
  assert.equal(state.sampleLog.state,'not-started');
  assert.equal(state.sampleLog.results.service,'refused');
  assert.equal(state.sampleLog.results.ui,'not-requested');
  assert.match(V.modal({...state,modal:'debug-sample'}),/Not started/);
  assert.doesNotMatch(V.modal({...state,modal:'debug-sample'}),/sample-log-outcome|Partial|minutes remaining/);
  state=M.reduce(state,{type:'sample-log-scope',value:'both'});
  state=M.reduce(state,{type:'sample-log-start'});
  assert.equal(state.sampleLog.state,'partial');
  const before=JSON.stringify(state.sampleLog);
  state=M.reduce({...state,modal:'debug-sample'},{type:'close'});
  assert.equal(JSON.stringify(state.sampleLog),before);
  assert.match(V.diagnostics({...state,mode:'power',page:'diagnostics'}),/Partial/);
  state=M.reduce(state,{type:'sample-log-stop'});
  assert.equal(state.sampleLog.state,'stopped');
  state=M.reduce(state,{type:'sample-log-scope',value:'ui'});
  state=M.reduce(state,{type:'sample-log-start'});
  assert.equal(state.sampleLog.state,'active');
  assert.equal(state.sampleLog.results.service,'not-requested');
  const view=V.modal({...state,modal:'debug-sample'});
  assert.doesNotMatch(view,/Permission refused|Service collection failed/);
  assert.match(view,/15 minutes.*frozen sample/s);
});

test('component outcome controls respect confirmed service facts and scenario changes clear sessions', () => {
  for (const [id,result] of [['service-stopped','unavailable'],['stale','unknown'],['debug-partial','refused']]) {
    let state=M.initialState(id);
    state=M.reduce(state,{type:'sample-log-response',component:'service',value:'active'});
    assert.equal(state.sampleLogResponse.service,result,id);
    state=M.reduce(state,{type:'sample-log-scope',value:'service'});
    state=M.reduce(state,{type:'sample-log-start'});
    assert.equal(state.sampleLog.state,id === 'debug-partial' ? 'not-started' : 'idle',id);
    state=M.reduce(state,{type:'scenario',id:'healthy'});
    assert.equal(state.modal,null,'a blocked service-only request created no session to clear');
    assert.equal(state.sampleLog.state,'idle');
  }
});

test('all-source delay fixture retains a valid older model without refreshing source evidence', () => {
  const state=M.initialState('all-rate-limited');
  assert.equal(state.scenario.historical,false,'the retained aggregate model is still usable');
  assert.equal(state.scenario.agree,0);
  for(const source of state.scenario.sources) {
    assert.equal(source.controlResponse,'RATE');
    assert.equal(source.lastControlResponse,'12 s ago');
    assert.equal(source.reply,'4 min ago');
    assert.equal(source.lastAuthenticatedMeasurement,'4 min ago');
    assert.equal(source.measurementHistorical,true);
    assert.equal(source.nextPermittedCheck,null);
  }
  assert.match(V.simpleStatus(state),/Based on the last valid time estimate/);
  assert.match(V.status(state),/Retained model offset/);
});


test('lost service reply leaves collection unconfirmed through close and stop, including combined scope', () => {
  for(const scope of ['service','both']) {
    let state=M.reduce(M.initialState('stale'),{type:'sample-log-scope',value:scope});
    state=M.reduce(state,{type:'sample-log-start'});
    if(scope === 'service') {
      assert.equal(state.sampleLog.state,'idle','unknown service cannot start a service-only session');
      assert.match(V.modal({...state,modal:'debug-sample'}),/disabled[^>]*aria-label="Start example/);
      continue;
    }
    assert.equal(state.sampleLog.state,'unconfirmed',scope);
    assert.equal(state.sampleLog.results.service,'unknown');
    assert.equal(state.sampleLog.results.ui,scope === 'both' ? 'active' : 'not-requested');
    state=M.reduce({...state,modal:'debug-sample'},{type:'close'});
    assert.equal(state.sampleLog.state,'unconfirmed');
    state=M.reduce(state,{type:'sample-log-stop'});
    assert.equal(state.sampleLog.state,'unconfirmed','stop cannot confirm a missing service reply');
    assert.equal(state.sampleLog.results.service,'unknown');
    if(scope === 'both') assert.equal(state.sampleLog.results.ui,'stopped');
    const view=V.modal({...state,modal:'debug-sample'});
    assert.match(view,/Sample response · Unconfirmed/);
    assert.match(view,/Remaining duration<\/dt><dd>Unknown/);
    assert.doesNotMatch(view,/Not started · service|Sample response · (Not started|Partial|Stopped)/);
  }
});

test('changing duration after a completed response clears that result and starts a fresh configuration', () => {
  for(const outcome of ['active','failed']) {
    let state=M.reduce(M.initialState(),{type:'sample-log-duration',value:'5'});
    state=M.reduce(state,{type:'sample-log-response',component:'ui',value:outcome});
    state=M.reduce(state,{type:'sample-log-start'});
    if(outcome === 'active') state=M.reduce(state,{type:'sample-log-stop'});
    const completed=JSON.stringify(state.sampleLog);
    state=M.reduce(state,{type:'sample-log-duration',value:'30'});
    assert.equal(state.sampleLog.state,'idle');
    assert.equal(state.sampleLog.duration,'30');
    assert.deepEqual(state.sampleLog.results,{ui:'not-requested',service:'not-requested'});
    assert.notEqual(JSON.stringify(state.sampleLog),completed);
    assert.doesNotMatch(V.diagnostics({...state,mode:'power'}),/class="sample-log-summary"/);
    state=M.reduce(state,{type:'sample-log-start'});
    assert.equal(state.sampleLog.state,outcome);
    assert.equal(state.sampleLog.duration,'30');
  }
});
