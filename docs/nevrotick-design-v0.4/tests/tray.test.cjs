'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const M = require('../src/model.js');
const enabled = id => M.reduce(M.initialState(id), {type:'sample-tray',value:true});
const lifecycle = (state,operation,response='confirmed') => M.reduce(M.reduce(M.reduce(M.reduce(state,{type:'lifecycle-request',operation}),{type:'lifecycle-confirm'}),{type:'sample-lifecycle-response',value:response}),{type:'sample-lifecycle-deliver'});

test('tray presentation covers every scene and preserves the observed service/evidence inputs', () => {
  assert.equal(typeof M.trayPresentation,'function');
  for (const scene of M.SCENARIOS) {
    const state=enabled(scene.id),before=JSON.stringify(state),icon=M.trayPresentation(state);
    assert.ok(['tracking','checking','attention','blocked','unknown','paused','observing','windows'].includes(icon.kind),scene.id);
    assert.ok(icon.label.length && icon.tooltip.includes(icon.label),scene.id);
    assert.equal(JSON.stringify(state),before);
  }
});

test('tray never paints stale, conflicting, unauthenticated or unknown clock outcomes as authenticated tracking', () => {
  for (const id of ['stale','conflict','ntp','outcome-unknown','expired','observe','service-stopped']) {
    assert.notEqual(M.trayPresentation(enabled(id)).kind,'tracking',id);
  }
  assert.equal(M.trayPresentation(enabled('healthy')).kind,'tracking');
  assert.equal(M.trayPresentation(enabled('debug-partial')).kind,'tracking');
  assert.equal(M.trayPresentation(enabled('acquiring')).kind,'checking');
});

test('confirmed Windows ownership stays visibly unverified and does not offer a Windows clock-write command', () => {
  const state=lifecycle(M.reduce(enabled(),{type:'service-settings'}),'use-windows');
  const icon=M.trayPresentation(state);
  assert.equal(icon.kind,'windows');
  assert.match(icon.tooltip,/Windows.*(not.*verified|unverified)/i);
  assert.notEqual(icon.checkLabel,'Sync now');
  const pending=M.reduce(M.reduce(state,{type:'service-request',operation:'start'}),{type:'service-confirm'});
  assert.match(M.trayPresentation(pending).tooltip,/Windows Time remains the confirmed manager/);
});

test('pending runtime and startup-only operations have different tray health', () => {
  const base=M.reduce(enabled(),{type:'service-settings'});
  const stop=M.reduce(M.reduce(base,{type:'service-request',operation:'stop'}),{type:'service-confirm'});
  const startup=M.reduce(M.reduce(base,{type:'lifecycle-request',operation:'startup-off'}),{type:'lifecycle-confirm'});
  assert.equal(M.trayPresentation(stop).kind,'checking');
  assert.equal(M.trayPresentation(stop).checkEnabled,false);
  assert.equal(M.trayPresentation(startup).kind,'tracking');
});

test('incomplete handovers with no confirmed synchronizer remain attention states rather than intentional pause', () => {
  const base=M.reduce(enabled(),{type:'service-settings'});
  const failedWindows=lifecycle(base,'use-windows','windows-unavailable');
  const failedNevroTick=lifecycle(lifecycle(base,'use-windows'),'use-nevrotick','product-unavailable');
  for (const state of [failedWindows,failedNevroTick]) {
    const icon=M.trayPresentation(state);
    assert.equal(icon.kind,'attention');
    assert.match(icon.tooltip,/incomplete|no active synchronizer/i);
  }
  assert.equal(M.trayPresentation(lifecycle(base,'disable')).kind,'paused');
});

test('opening and closing tray menu performs no check, notification or service operation', () => {
  const state=enabled(),open=M.reduce(state,{type:'tray-menu-open'});
  assert.equal(open.trayMenuOpen,true);
  for (const key of ['scenario','serviceControl','timeManagement','notifications','sampleSyncStarted']) assert.deepEqual(open[key],state[key]);
  const closed=M.reduce(open,{type:'tray-menu-close'});
  assert.equal(closed.trayMenuOpen,false);
  assert.equal(M.reduce(M.initialState(),{type:'tray-menu-open'}).trayMenuOpen,false);
});

test('tray cannot replace an in-progress editor or its unsaved raw form', () => {
  const state=M.reduce(enabled(),{type:'modal',modal:'edit-source',id:'alpha'});
  for (const type of ['tray-menu-open','tray-menu-toggle','tray-preview-enable','sample-tray-quit','tray-open-page','tray-check']) {
    assert.strictEqual(M.reduce(state,{type,page:'about'}),state,type);
  }
});

test('Quit UI requires a warning, cancellation retains the observer, and service-management opens the named settings section', () => {
  const base=enabled(),request=M.reduce(base,{type:'sample-tray-quit'});
  assert.equal(request.modal,'tray-quit-confirm');
  assert.equal(request.notifications.presenter,'active');
  assert.deepEqual(request.serviceControl,base.serviceControl);
  assert.strictEqual(M.reduce(base,{type:'sample-tray-quit-confirm'}),base);
  const cancelled=M.reduce(request,{type:'close'});
  assert.equal(cancelled.notifications.presenter,'active');
  const manage=M.reduce(request,{type:'tray-quit-manage'});
  assert.equal(manage.mode,'power');assert.equal(manage.page,'settings');assert.equal(manage.settingsSection,'service');
  assert.equal(manage.notifications.presenter,'active');
  assert.deepEqual(manage.timeManagement,base.timeManagement);
});

test('confirmed tray quit stops the sample presenter while preserving services, ownership, sign-in preference and evidence', () => {
  let base=M.reduce(enabled(),{type:'sample-signin',value:true});
  base=M.reduce(base,{type:'notification-test'});
  const state=M.reduce(M.reduce(base,{type:'sample-tray-quit'}),{type:'sample-tray-quit-confirm'});
  assert.equal(state.notifications.presenter,'missing');assert.equal(state.notifications.current,null);
  for (const key of ['scenario','serviceControl','timeManagement','sampleTray','sampleSignIn']) assert.deepEqual(state[key],base[key]);
  assert.deepEqual(state.notifications.items,base.notifications.items);
  assert.match(M.notificationAvailability(state),/not running|cannot.*send|unavailable/i);
  const reopened=M.reduce(state,{type:'tray-observer-open'});
  assert.equal(reopened.notifications.presenter,'active');assert.equal(reopened.notifications.current,null);
  assert.deepEqual(reopened.timeManagement,state.timeManagement);
});

test('tray navigation keeps About and Settings in Simple while Diagnostics enters Power user', () => {
  for (const page of ['status','settings','about','diagnostics']) {
    const state=M.reduce(M.reduce(enabled(),{type:'tray-menu-open'}),{type:'tray-open-page',page});
    assert.equal(state.page,page);assert.equal(state.mode,page==='diagnostics'?'power':'simple');
    assert.equal(state.trayMenuOpen,false);
  }
});
