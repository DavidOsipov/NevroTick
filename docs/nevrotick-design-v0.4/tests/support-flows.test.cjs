'use strict';
const test=require('node:test');
const assert=require('node:assert/strict');
const M=require('../src/model.js');
const V=require('../src/views.js');

test('server limit response is a labelled inline sample without changing source fixtures',()=>{
  let s=M.initialState('all-rate-limited');const source=JSON.stringify(s.scenario);
  s=M.reduce(s,{type:'sync-now'});
  assert.equal(s.syncFeedback,'rate-limited');
  assert.equal(JSON.stringify(s.scenario),source);
  assert.match(V.simpleStatus(s),/Checks delayed by server limits/);
  assert.match(V.simpleStatus(s),/Sample response/);
  assert.equal(M.reduce(s,{type:'sample-sync',value:'not-an-option'}).sampleSync,'rate-limited');
});

test('temporary collection is scoped with frozen duration, component responses and explicit stop',()=>{
  let s=M.initialState();
  s=M.reduce(s,{type:'sample-log-scope',value:'both'});
  s=M.reduce(s,{type:'sample-log-duration',value:'15'});
  s=M.reduce(s,{type:'sample-log-response',component:'service',value:'refused'});
  s=M.reduce(s,{type:'sample-log-start'});
  assert.equal(s.sampleLog.state,'partial');assert.equal(s.sampleLog.scope,'both');assert.equal(s.sampleLog.duration,'15');
  const v=V.modal({...s,modal:'debug-sample'});
  assert.match(v,/administrator/i);assert.match(v,/required events/i);assert.match(v,/Stop example/);
  s=M.reduce(s,{type:'sample-log-stop'});assert.equal(s.sampleLog.state,'stopped');
});

test('tray and sign-in are separate preview choices; tray menu never changes the service',()=>{
  let s=M.initialState();const source=JSON.stringify(s.scenario);
  assert.equal(s.sampleTray,false);assert.equal(s.sampleSignIn,false);
  s=M.reduce(s,{type:'sample-tray',value:true});
  assert.equal(s.sampleSignIn,false);
  s=M.reduce(s,{type:'sample-signin',value:true});
  assert.equal(s.sampleTray,true);assert.equal(s.sampleSignIn,true);
  const v=V.trayPreview(M.reduce(s,{type:'tray-menu-open'}));
  for(const label of ['Open NevroTick','Sync now','Quit UI']) assert.ok(v.includes(label));
  assert.match(v,/Quitting does not stop background services/i);assert.equal(JSON.stringify(s.scenario),source);
});

test('source delay is honestly unavailable when fixture contains no delay',()=>{
  const s={...M.initialState(),selectedSource:'alpha'};
  const v=V.modal({...s,modal:'source-details'});
  assert.match(v,/Delay/);assert.match(v,/Not available in this fixture/);
});
