'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const M = require('../src/model.js');
const V = require('../src/views.js');
const step = (state, type, extra={}) => M.reduce(state,{type,...extra});
const power = (page='sources') => ({...M.initialState(),mode:'power',page,powerPage:page});
const edited = (state=power()) => step(step(state,'recovery-edit'),'recovery-input',{count:'2',helpers:['one.example','unfinished /']});

test('recovery captures one entry context and restores filters, section, source dialog, focus and scroll',()=>{
  for (const [page,label] of [['sources','sources'],['diagnostics','diagnostics'],['settings','time policy']]) {
    const original={...power(page),settingsSection:'policy',sourceFilter:'other',sourceQuery:'delta',eventFilter:'issues',modal:page==='sources'?'source-details':null,selectedSource:page==='sources'?'delta':null};
    const context={scroll:{x:12,y:418},focusKey:{action:'page',value:'recovery'},dialogScroll:71};
    let state=step(original,'page',{page:'recovery',context});
    assert.equal(state.page,'recovery');
    assert.match(V.recovery(state),new RegExp(`Back to ${label}`));
    const saved=state.returnContext;
    assert.deepEqual(saved.scroll,context.scroll);
    assert.deepEqual(saved.focusKey,context.focusKey);
    state=step(state,'power-page',{page:'recovery',context:{scroll:{x:0,y:0}}});
    assert.deepEqual(state.returnContext,saved,'nested entry must not overwrite the origin');
    state=step({...state,sourceFilter:'all',sourceQuery:'',eventFilter:'all'},'recovery-back');
    for (const key of ['mode','page','settingsSection','sourceFilter','sourceQuery','eventFilter','modal','selectedSource']) assert.equal(state[key],original[key],key);
    assert.equal(state.returnContext,null);
  }
});

test('Simple help enters Power recovery explicitly and back restores Simple Status',()=>{
  const original={...M.initialState('reserve'),modal:'status-help'};
  assert.match(V.modal(original),/Review date recovery in Power user/);
  const entered=step(original,'power-page',{page:'recovery'});
  assert.equal(entered.mode,'power'); assert.equal(entered.page,'recovery');
  const returned=step(entered,'recovery-back');
  assert.equal(returned.mode,'simple'); assert.equal(returned.page,'status'); assert.equal(returned.modal,null);
  assert.equal(returned.scenario,original.scenario);
});

test('mode roundtrip preserves recovery, explicit page selection finishes it',()=>{
  let state=step(power('diagnostics'),'page',{page:'recovery'});
  state=step(state,'view-mode',{mode:'simple'});
  assert.equal(state.page,'status'); assert.ok(state.returnContext);
  state=step(state,'view-mode',{mode:'power'});
  assert.equal(state.page,'recovery');
  state=step(state,'view-mode',{mode:'simple'});
  state=step(state,'page',{page:'settings'});
  assert.equal(state.returnContext,null);
  state=step(state,'view-mode',{mode:'power'});
  assert.equal(state.page,'settings');
  assert.match(V.recovery({...state,returnContext:null}),/Back to status/);
});

test('a missing source returns to Sources without reviving a missing dialog',()=>{
  let state=step({...power(),modal:'source-details',selectedSource:'delta'},'page',{page:'recovery'});
  state={...state,scenario:{...state.scenario,sources:state.scenario.sources.filter(x=>x.id!=='delta')}};
  state=step(state,'recovery-back');
  assert.equal(state.page,'sources'); assert.equal(state.modal,null); assert.equal(state.selectedSource,null);
});

test('leaving edited helpers offers one dialog warning and Escape stays without losing input',()=>{
  let state=edited();
  state=step(state,'close');
  assert.equal(state.modal,'recovery-editor'); assert.equal(state.leaveGuard.kind,'recovery-editor');
  const html=V.modal(state);
  for (const label of ['Keep draft and leave','Discard edits','Stay here']) assert.ok(html.includes(label));
  assert.equal((html.match(/<dialog/g)||[]).length,1);
  state=step(state,'close');
  assert.equal(state.leaveGuard,null); assert.equal(state.modal,'recovery-editor');
  assert.deepEqual(state.recoveryInput.helpers,['one.example','unfinished /']);
});

test('keep draft accepts partial input locally, reopening restores it and review still validates',()=>{
  const original=power();
  let state=step(edited(original),'close');
  state=step(state,'recovery-keep-leave');
  assert.equal(state.modal,null); assert.equal(state.recoveryDraft,null);
  assert.match(V.render(state),/Helper draft not applied/);
  state=step(state,'recovery-edit');
  assert.deepEqual(state.recoveryInput.helpers,['one.example','unfinished /']);
  state=step(state,'recovery-preview',{...state.recoveryInput});
  assert.equal(state.modal,'recovery-editor'); assert.ok(state.recoveryErrors.helper2);
  assert.equal(state.scenario,original.scenario);
});

test('discarding edits restores the saved preview and closing its review retains it without warning',()=>{
  let state=step(power(),'recovery-preview',{count:'1',helpers:['saved.example','']});
  const preview=state.recoveryDraft;
  state=step(state,'recovery-edit');
  state=step(state,'recovery-input',{count:'2',helpers:['edited.example','']});
  state=step(state,'close');
  state=step(state,'recovery-discard-edits');
  assert.deepEqual(state.recoveryDraft,preview); assert.equal(state.modal,null);
  state=step(state,'recovery-edit');
  assert.deepEqual(state.recoveryInput,{count:'1',helpers:['saved.example','']});
  state=step(state,'close'); assert.equal(state.modal,null); assert.equal(state.leaveGuard,null);
  state=step(state,'modal',{modal:'recovery-review'});
  state=step(state,'close'); assert.deepEqual(state.recoveryDraft,preview); assert.equal(state.leaveGuard,null);
  state=step(state,'recovery-discard'); assert.equal(state.recoveryDraft,null); assert.equal(state.recoverySavedInput,null);
});

test('editor navigation waits for a choice and then completes the requested route',()=>{
  let state=step(edited(),'page',{page:'diagnostics'});
  assert.equal(state.page,'sources'); assert.equal(state.leaveGuard.action.page,'diagnostics');
  state=step(state,'recovery-keep-leave');
  assert.equal(state.page,'diagnostics'); assert.equal(state.modal,null); assert.ok(state.recoverySavedInput);
});

test('a kept partial draft cannot bypass review validation through an older saved preview',()=>{
  let state=step(power(),'recovery-preview',{count:'1',helpers:['saved.example','']});
  const previous=state.recoveryDraft;
  state=step(edited(state),'close');
  state=step(state,'recovery-keep-leave');
  state=step(state,'modal',{modal:'recovery-review'});
  assert.equal(state.modal,'recovery-editor');
  assert.ok(state.recoveryErrors.helper2);
  assert.deepEqual(state.recoveryDraft,previous);
});

test('scene/reset guard discarded previews, cancel preserves them and confirmation starts a clean scene',()=>{
  for(const type of ['scenario','reset']) {
    let state=step(power(),'recovery-preview',{count:'1',helpers:['saved.example','']});
    state={...state,modal:'recovery-review',returnContext:{page:'sources'},sampleLog:{...state.sampleLog,state:'active'}};
    const original=state;
    state=step(state,type,{id:'reserve'});
    assert.equal(state.scenario.id,'healthy'); assert.equal(state.modal,'reset-confirm');
    state=step(state,'close');
    assert.equal(state.modal,original.modal); assert.deepEqual(state.recoveryDraft,original.recoveryDraft);
    state=step(state,type,{id:'reserve'});
    state=step(state,'confirm-reset');
    assert.equal(state.scenario.id,type==='scenario'?'reserve':'healthy');
    assert.equal(state.page,'status'); assert.equal(state.modal,null); assert.equal(state.returnContext,null);
    assert.equal(state.recoveryDraft,null); assert.equal(state.sampleLog.state,'idle');
  }
});
