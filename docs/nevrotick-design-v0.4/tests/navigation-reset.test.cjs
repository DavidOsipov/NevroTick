'use strict';
const test=require('node:test');
const assert=require('node:assert/strict');
const M=require('../src/model.js');
const V=require('../src/views.js');

function collection(kind){
  let state=M.initialState(kind==='unconfirmed'?'stale':'healthy');
  state=M.reduce(state,{type:'sample-log-scope',value:kind==='active'?'ui':'both'});
  if(kind==='partial') state=M.reduce(state,{type:'sample-log-response',component:'service',value:'refused'});
  return M.reduce(state,{type:'sample-log-start'});
}
for(const kind of ['active','partial','unconfirmed']) for(const type of ['reset','scenario']) {
  test(`${type} confirms discarding an isolated ${kind} diagnostic example; cancel preserves it`,()=>{
    const original={...collection(kind),mode:'power',page:'diagnostics',powerPage:'diagnostics'};
    assert.equal(original.sampleLog.state,kind);
    const action={type,id:'holdover'};
    const guarded=M.reduce(original,action);
    assert.equal(guarded.modal,'reset-confirm');
    assert.equal(guarded.scenario.id,original.scenario.id);
    assert.deepEqual(guarded.sampleLog,original.sampleLog);
    assert.match(V.modal(guarded),/does not stop or confirm.*real service/i);
    for(const cancel of ['cancel-reset','close']) {
      const stayed=M.reduce(guarded,{type:cancel});
      assert.equal(stayed.leaveGuard,null);
      assert.equal(stayed.modal,original.modal);
      assert.equal(stayed.page,'diagnostics');
      assert.deepEqual(stayed.sampleLog,original.sampleLog);
    }
    const cleared=M.reduce(guarded,{type:'confirm-reset'});
    assert.equal(cleared.modal,null);
    assert.equal(cleared.leaveGuard,null);
    assert.equal(cleared.scenario.id,type==='scenario'?'holdover':original.scenario.id);
    assert.equal(cleared.sampleLog.state,'idle');
    assert.deepEqual(cleared.sampleLog.results,{ui:'not-requested',service:'not-requested'});
  });
}
test('active or unknown component results require confirmation even if the aggregate label is stale',()=>{
  for(const result of ['active','unknown']) {
    const original=M.initialState();
    const state={...original,sampleLog:{...original.sampleLog,state:'stopped',results:{ui:'not-requested',service:result}}};
    assert.equal(M.reduce(state,{type:'reset'}).modal,'reset-confirm');
    assert.equal(M.reduce(state,{type:'scenario',id:'holdover'}).modal,'reset-confirm');
  }
});
test('stopped or never-started examples alone do not interrupt scene navigation',()=>{
  const idle=M.initialState();
  const stopped=M.reduce(collection('active'),{type:'sample-log-stop'});
  for(const state of [idle,stopped]) {
    assert.equal(M.reduce(state,{type:'reset'}).modal,null);
    assert.equal(M.reduce(state,{type:'scenario',id:'holdover'}).scenario.id,'holdover');
  }
});
test('clear filters restores all sources and undo removes only the selected local draft',()=>{
  let state=M.reduce({...M.initialState(),sourceQuery:'nomatch',sourceFilter:'used'},{type:'clear-source-filter'});
  assert.equal(state.sourceQuery,'');assert.equal(state.sourceFilter,'all');
  assert.equal(M.visibleSources(state).length,4);
  const source=state.scenario;
  for(const name of ['one','two'])state=M.reduce(state,{type:'add-draft',raw:{name,host:name+'.example',protocol:'NTS',port:'4460'}});
  const keep=state.drafts[1],remove=state.drafts[0];
  state=M.reduce(state,{type:'undo-draft',id:remove.id});
  assert.deepEqual(state.drafts,[keep]);assert.equal(state.scenario,source);
});
test('an unconfirmed stop is acknowledged once while known UI collection stops and service remains unknown',()=>{
  const active=collection('unconfirmed');
  assert.equal(active.sampleLog.results.ui,'active');assert.equal(active.sampleLog.results.service,'unknown');
  const stopped=M.reduce(active,{type:'sample-log-stop'});
  assert.equal(stopped.sampleLog.state,'unconfirmed');
  assert.equal(stopped.sampleLog.results.ui,'stopped');assert.equal(stopped.sampleLog.results.service,'unknown');
  assert.equal(stopped.sampleLog.stopRequested,true);
  const view=V.modal({...stopped,modal:'debug-sample'});
  assert.match(view,/Stop requested/);assert.match(view,/service stop remains unconfirmed/i);
  assert.match(view,/<button[^>]*disabled[^>]*>Stop requested<\/button>/);
  assert.doesNotMatch(view,/data-action="sample-log-stop"/);
  assert.equal(M.reduce(stopped,{type:'sample-log-stop'}),stopped,'repeated stop does not create a second request');
  const guard=M.reduce(stopped,{type:'reset'});
  assert.deepEqual(M.reduce(guard,{type:'cancel-reset'}).sampleLog,stopped.sampleLog);
  const reset=M.reduce(guard,{type:'confirm-reset'});
  assert.equal(reset.sampleLog.stopRequested,false);
  assert.equal(reset.sampleLog.state,'idle');
});
