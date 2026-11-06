'use strict';
const test=require('node:test');
const assert=require('node:assert/strict');
const M=require('../src/model.js');
const V=require('../src/views.js');

test('sync command exposes scene-specific inline outcomes without changing source data',()=>{
  const expectations=[['healthy','checking'],['stale','status-unknown'],['service-stopped','service-stopped'],['conflict','waiting'],['blocked','checking'],['slewing','correction']];
  for(const [id, expected] of expectations){
    let state=M.initialState(id);const original=JSON.stringify(state.scenario);
    state=M.reduce(state,{type:'sync-now'});
    assert.equal(state.syncFeedback,expected,id);
    assert.equal(state.modal,null,id);
    assert.equal(JSON.stringify(state.scenario),original,id);
    assert.match(V.render(state),new RegExp(`value="${expected}" selected`));
    state=M.reduce(state,{type:'sample-sync',value:'rate-limited'});
    assert.equal(state.sampleSync,expected);
  }
});

test('unknown command acceptance is distinct from a confirmed stopped service',()=>{
  const stale=V.simpleStatus(M.reduce(M.initialState('stale'),{type:'sync-now'}));
  const stopped=V.status(M.reduce(M.initialState('service-stopped'),{type:'sync-now'}));
  assert.match(stale,/status connection.*lost/i);
  assert.match(stopped,/Check unavailable/);
  assert.match(stopped,/service.*stopped/i);
});

test('simple conflict provides a direct and contextual route to comparison',()=>{
  const state=M.initialState('conflict');
  const page=V.simpleStatus(state);
  assert.match(page,/data-action="power-page" data-value="statistics"/);
  assert.match(page,/Compare sources/);
  const help=V.modal({...state,modal:'status-help'});
  assert.match(help,/Compare sources/);
  assert.match(help,/data-action="power-page" data-value="statistics"/);
  const next=M.reduce(state,{type:'power-page',page:'statistics'});
  assert.equal(next.mode,'power');assert.equal(next.page,'statistics');
});

test('simple explains authenticated NTS and qualifying uncertainty without false direction certainty',()=>{
  const healthy=V.simpleStatus(M.initialState('healthy'));
  assert.match(healthy,/NTS authenticates.*responses.*accuracy/i);
  assert.match(healthy,/direction is an estimate.*may be uncertain/i);
  assert.match(healthy,/0\.7 ms.*ahead/s);
  const correction=V.simpleStatus(M.initialState('correction'));
  assert.match(correction,/direction is an estimate.*may be uncertain/i);
});

test('interval uses measurement wording and keeps natural custom minutes',()=>{
  let state=M.initialState();
  const view=V.simpleSettings(state);
  assert.match(view,/Time check interval/);
  assert.doesNotMatch(view,/Sync frequency/);
  state=M.reduce(state,{type:'frequency-draft',value:'own:1'});
  state=M.reduce(state,{type:'frequency-save'});
  assert.equal(state.frequencyPreview,'own:1');
  state=M.reduce(state,{type:'frequency-draft',value:'own:0'});
  state=M.reduce(state,{type:'frequency-save'});
  assert.equal(state.frequencyPreview,'own:1');assert.ok(state.frequencyError);
});

test('scenario changes clear the command response while preserving saved frequency preview',()=>{
  let state=M.initialState();
  state=M.reduce(state,{type:'frequency-draft',value:'own:73'});
  state=M.reduce(state,{type:'frequency-save'});
  state=M.reduce(state,{type:'sync-now'});
  state=M.reduce(state,{type:'scenario',id:'stale'});
  assert.equal(state.sampleSync,'status-unknown');
  assert.equal(state.syncFeedback,null);
  assert.equal(state.frequencyPreview,'own:73');
  state=M.reduce(state,{type:'sample-sync',value:'checking'});
  state=M.reduce(state,{type:'sync-now'});
  assert.equal(state.syncFeedback,'status-unknown');
});

test('confirmed service stop shows contextual repair guidance within diagnostics',()=>{
  const state={...M.initialState('service-stopped'), mode:'power', page:'diagnostics'};
  const html=V.diagnostics(state);
  assert.match(html, /Restore background synchronization/);
  assert.match(html, /installed app.*repair options/);
  assert.match(html, /data-action="modal" data-value="export"[^>]*>Export preview/);
  assert.equal((html.match(/data-action="modal" data-value="export"/g)||[]).length,1);
  assert.doesNotMatch(html, /delivery-page|MSIX preparation|Store submission/);
  assert.doesNotMatch(V.diagnostics({...M.initialState('healthy'),mode:'power',page:'diagnostics'}), /Restore background synchronization/);
  assert.doesNotMatch(V.diagnostics({...M.initialState('stale'),mode:'power',page:'diagnostics'}), /Restore background synchronization/);
});

test('simple status leads to diagnostics when service is stopped and keeps normal sync available otherwise',()=>{
  const stopped=M.initialState('service-stopped');
  const page=V.simpleStatus(stopped);
  assert.match(page, /data-action="power-page" data-value="diagnostics"[^>]*>Open diagnostics<\/button>/);
  assert.match(page, /new check.*unavailable.*service is stopped/i);
  assert.doesNotMatch(page, />Sync now<\/button>/);
  const next=M.reduce(stopped,{type:'power-page',page:'diagnostics'});
  assert.equal(next.page,'diagnostics'); assert.equal(next.mode,'power');
  assert.match(V.diagnostics(next),/Restore background synchronization/);
  assert.match(V.simpleStatus(M.initialState('healthy')),/>Sync now<\/button>/);
  const power={...M.initialState('healthy'),mode:'power'};
  assert.match(V.status(power),/>Sync now<\/button>/);
});

test('simple never infers actual interval bounds from uncertainty alone',()=>{
  const state=M.initialState('healthy');
  state.scenario.interval='Unknown';
  const page=V.simpleStatus(state);
  assert.doesNotMatch(page,/range includes zero/i);
  assert.match(page,/direction is an estimate.*may be uncertain/i);
});

test('power offset direction is labelled as an estimate and historical values stay historical',()=>{
  const healthy=V.status({...M.initialState('healthy'),mode:'power'});
  const stale=V.status({...M.initialState('stale'),mode:'power'});
  assert.match(healthy,/Estimated direction: Windows ahead of the reference/);
  assert.match(stale,/Last known estimated direction: Windows ahead of the reference/);
  assert.doesNotMatch(stale,/>Windows is ahead of the reference</);
});
