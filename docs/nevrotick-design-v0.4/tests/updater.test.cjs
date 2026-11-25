'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const M = require('../src/model.js');

const action = (state, type, fields = {}) => M.reduce(state, {type, ...fields});
const about = () => action(M.initialState(), 'page', {page:'about'});
const step = state => action(state, 'update-step', {generation:state.updater.generation, phase:state.updater.phase});
const check = (sample = 'available', state = about()) => step(action(action(state, 'sample-update-result', {value:sample}), 'update-check'));
const accept = state => action(state, 'update-accept', {generation:state.updater.generation, targetId:state.updater.target.id});
const installed = (sample = 'available') => step(step(step(accept(check(sample)))));

test('UP-UI-01: one exact-target Yes automatically reaches installation through three distinct stages', () => {
  const initial = about();
  assert.equal(initial.updater.phase, 'not-checked');
  const checking = action(initial, 'update-check');
  assert.equal(checking.updater.phase, 'checking');
  assert.equal(checking.updater.packageStarted, false);
  assert.equal(checking.updater.intent, null);
  const offered = step(checking);
  assert.equal(offered.updater.phase, 'available');
  assert.equal(offered.modal, 'update-offer');
  assert.equal(offered.updater.packageStarted, false);
  const downloading = accept(offered);
  assert.equal(downloading.updater.phase, 'downloading');
  assert.equal(downloading.modal, null);
  assert.equal(downloading.updater.consentCount, 1);
  assert.equal(downloading.updater.intent.targetId, offered.updater.target.id);
  const verifying = step(downloading);
  assert.equal(verifying.updater.phase, 'verifying');
  assert.equal(verifying.updater.installStarted, false);
  const installing = step(verifying);
  assert.equal(installing.updater.phase, 'installing');
  assert.equal(installing.updater.installStarted, true);
  assert.equal(installing.updater.installedIdentityKnown, false);
  const complete = step(installing);
  assert.equal(complete.updater.phase, 'installed');
  assert.equal(complete.updater.installedVersion, '1.1.0');
  assert.equal(complete.updater.runningVersion, '1.0.0');
  assert.equal(complete.updater.installedIdentityKnown, true);
  assert.equal(complete.updater.maintenanceComplete, true);
  assert.equal(complete.updater.serviceReadiness, 'ready');
  assert.equal(complete.updater.consentCount, 1);
  assert.equal(complete.updater.intent, null);
  for (const state of [downloading, verifying, installing, complete]) assert.equal(state.modal, null);
  for (const key of ['scenario','serviceControl','timeManagement','notifications']) assert.deepEqual(complete[key], initial[key]);
});

test('UP-UI-02: Later and Escape close only the offer and never start package work', () => {
  for (const type of ['update-later','close']) {
    const declined = action(check(), type);
    assert.equal(declined.modal, null);
    assert.equal(declined.updater.phase, 'available');
    assert.equal(declined.updater.packageStarted, false);
    assert.equal(declined.updater.installStarted, false);
    assert.equal(declined.updater.consentCount, 0);
    assert.equal(declined.updater.intent, null);
    assert.strictEqual(accept(declined), declined);
    assert.equal(accept(action(declined, 'update-offer')).updater.phase, 'downloading');
  }
});

test('UP-UI-04: target fixtures and accepted intent are immutable; stale offer tokens grant no intent', () => {
  const offered = check();
  assert.ok(Object.isFrozen(M.UPDATE_TARGETS));
  assert.ok(Object.isFrozen(offered.updater.target));
  assert.throws(() => { offered.updater.target.bytes = 1; }, TypeError);
  assert.throws(() => { offered.updater.target.termsRevision = 'changed'; }, TypeError);
  assert.strictEqual(action(offered, 'update-accept'), offered);
  assert.strictEqual(action(offered, 'update-accept', {generation:offered.updater.generation - 1,targetId:offered.updater.target.id}), offered);
  assert.strictEqual(action(offered, 'update-accept', {generation:offered.updater.generation,targetId:'different-target'}), offered);
  const accepted = accept(offered);
  assert.ok(Object.isFrozen(accepted.updater.intent));
  assert.throws(() => { accepted.updater.intent.targetId = 'other'; }, TypeError);
  assert.strictEqual(accept(accepted), accepted, 'a duplicate Yes cannot restart an accepted transaction');
});

test('UP-UI-04/10: target, terms, expiry or installed-state changes invalidate the old Yes', () => {
  for (const value of ['target','terms','expired','installed']) {
    const accepted = accept(check());
    const old = {generation:accepted.updater.generation,phase:accepted.updater.phase};
    const changed = action(accepted, 'sample-update-invalidate', {value});
    assert.equal(changed.updater.phase, 'available', value);
    assert.equal(changed.updater.intent, null, value);
    assert.equal(changed.updater.installStarted, false, value);
    assert.ok(changed.updater.generation > accepted.updater.generation, value);
    assert.strictEqual(action(changed, 'update-step', old), changed, value);
    assert.strictEqual(action(changed, 'update-accept', {generation:old.generation,targetId:accepted.updater.target.id}), changed, value);
    assert.equal(accept(changed).updater.consentCount, 2, value);
  }
  const blocked = action(accept(check()), 'sample-update-invalidate', {value:'policy'});
  assert.equal(blocked.updater.phase, 'failed');
  assert.equal(blocked.updater.reason, 'policy-blocked');
  assert.equal(blocked.updater.installStarted, false);
  assert.strictEqual(action(blocked, 'update-offer'), blocked);
});

test('UP-UI-04: a changed target or installed identity cannot pass the receiving phase guard', () => {
  const verifying = step(accept(check()));
  for (const changed of [
    {...verifying,updater:{...verifying.updater,target:{...verifying.updater.target,bytes:1}}},
    {...verifying,updater:{...verifying.updater,installedVersion:'9.0.0'}},
    {...verifying,updater:{...verifying.updater,intent:{...verifying.updater.intent,targetId:'different'}}}
  ]) {
    const refused = step(changed);
    assert.equal(refused.updater.phase, 'failed');
    assert.equal(refused.updater.reason, 'intent-invalidated');
    assert.equal(refused.updater.installStarted, false);
  }
  assert.equal(step(verifying).updater.phase, 'installing', 'unchanged revalidation adds no confirmation');
});

test('UP-UI-05: cancellation invalidates callbacks at every pre-install phase and permits a fresh attempt', () => {
  const checking = action(about(), 'update-check');
  const downloading = accept(step(checking));
  const verifying = step(downloading);
  for (const pending of [checking,downloading,verifying]) {
    assert.equal(M.updateCanCancel(pending), true);
    const cancelled = action(pending, 'update-cancel');
    assert.equal(cancelled.updater.phase, 'cancelled');
    assert.equal(cancelled.updater.intent, null);
    assert.equal(cancelled.updater.installedVersion, '1.0.0');
    assert.equal(cancelled.updater.installStarted, false);
    assert.strictEqual(action(cancelled, 'update-step', {generation:pending.updater.generation,phase:pending.updater.phase}), cancelled);
    const fresh = action(cancelled, 'update-check');
    assert.equal(fresh.updater.phase, 'checking');
    assert.ok(fresh.updater.generation > cancelled.updater.generation);
    assert.strictEqual(action(fresh, 'update-step', {generation:pending.updater.generation,phase:pending.updater.phase}), fresh);
    assert.equal(step(fresh).updater.phase, 'available');
  }
});

test('UP-UI-05: repeated, out-of-order and malformed callbacks cannot skip stages', () => {
  const pending = accept(check());
  for (const fields of [{}, {generation:-1,phase:'downloading'}, {generation:pending.updater.generation,phase:'installing'}, {generation:String(pending.updater.generation),phase:'downloading'}]) {
    assert.strictEqual(action(pending, 'update-step', fields), pending);
  }
  const verifying = step(pending);
  assert.strictEqual(action(verifying, 'update-step', {generation:pending.updater.generation,phase:'downloading'}), verifying);
  assert.strictEqual(action(pending, 'sample-update-result', {value:'package-refused'}), pending);
  assert.equal(step(verifying).updater.phase, 'installing');
});

test('UP-UI-03/06: fixed verification and UAC refusals cannot be overridden by a UI success flag', () => {
  for (const sample of ['package-refused','uac-denied']) {
    const verifying = step(accept(check(sample)));
    const refused = action(verifying, 'update-step', {generation:verifying.updater.generation,phase:'verifying',verified:true,result:'installed'});
    assert.equal(refused.updater.phase, 'failed');
    assert.equal(refused.updater.reason, sample);
    assert.equal(refused.updater.installStarted, false);
    assert.equal(refused.updater.installedVersion, '1.0.0');
    assert.equal(refused.updater.installedIdentityKnown, true);
    assert.equal(refused.updater.intent, null);
    assert.strictEqual(step(refused), refused);
    assert.equal(accept(check('available', refused)).updater.phase, 'downloading');
  }
});

test('UP-UI-07: accepted deployment is not success; unknown requires read-only reconciliation', () => {
  const installing = step(step(accept(check('outcome-unknown'))));
  assert.equal(M.updateCanCancel(installing), false);
  assert.strictEqual(action(installing, 'update-cancel'), installing);
  const unknown = step(installing);
  assert.equal(unknown.updater.phase, 'outcome-unknown');
  assert.equal(unknown.updater.installedIdentityKnown, false);
  assert.equal(unknown.updater.installedVersion, '1.0.0');
  assert.equal(unknown.updater.maintenanceComplete, false);
  assert.equal(unknown.updater.intent, null);
  assert.equal(M.updateCanCheck(unknown), false);
  assert.strictEqual(action(unknown, 'update-check'), unknown);
  const reconciling = action(unknown, 'update-reconcile');
  assert.equal(reconciling.updater.phase, 'reconciling');
  assert.strictEqual(action(reconciling, 'update-step', {generation:installing.updater.generation,phase:'installing'}), reconciling);
  const result = step(reconciling);
  assert.equal(result.updater.phase, 'installed');
  assert.equal(result.updater.installedVersion, '1.1.0');
  assert.equal(result.updater.consentCount, 1);
  assert.equal(result.updater.intent, null);
});

test('UP-UI-07/08: no-effect reconciliation, restart and recovery have distinct result truth', () => {
  const noEffect = step(action(installed('unknown-no-effect'), 'update-reconcile'));
  assert.equal(noEffect.updater.phase, 'failed');
  assert.equal(noEffect.updater.reason, 'no-effect-confirmed');
  assert.equal(noEffect.updater.installedVersion, '1.0.0');
  assert.equal(noEffect.updater.installedIdentityKnown, true);
  const restart = installed('restart-required');
  assert.equal(restart.updater.phase, 'restart-required');
  assert.equal(restart.updater.installedVersion, '1.1.0');
  assert.equal(restart.updater.runningVersion, '1.0.0');
  assert.equal(restart.updater.maintenanceComplete, false);
  assert.equal(restart.updater.serviceReadiness, 'pending-restart');
  assert.equal(restart.updater.restartScope, 'windows');
  const recovery = installed('recovery-required');
  assert.equal(recovery.updater.phase, 'recovery-required');
  assert.equal(recovery.updater.installedIdentityKnown, false);
  assert.strictEqual(action(recovery, 'update-check'), recovery);
});

test('restarting the sample app neither installs again nor clears a required Windows restart', () => {
  for (const sample of ['available','restart-required']) {
    const complete = installed(sample);
    const restarted = action(complete, 'update-restart');
    assert.equal(restarted.updater.runningVersion, '1.1.0');
    assert.deepEqual(restarted.updater, {...complete.updater,runningVersion:'1.1.0'});
    assert.equal(restarted.updater.intent, null);
    assert.strictEqual(action(restarted, 'update-restart'), restarted);
  }
  for (const state of [about(),action(about(),'update-check'),accept(check()),installed('outcome-unknown')]) assert.strictEqual(action(state,'update-restart'), state);
});

test('changing the next sample does not rewrite the current retained offer or its operation outcome', () => {
  const declined = action(check('package-refused'), 'update-later');
  const selected = action(declined, 'sample-update-result', {value:'available'});
  assert.equal(selected.updater.sample, 'available');
  assert.equal(selected.updater.operationSample, 'package-refused');
  const result = step(step(accept(action(selected, 'update-offer'))));
  assert.equal(result.updater.phase, 'failed');
  assert.equal(result.updater.reason, 'package-refused');
  const fresh = check('available', result);
  assert.equal(step(step(accept(fresh))).updater.phase, 'installing');
});

test('UP-UI-09: only a successful metadata check can report no eligible update', () => {
  const empty = check('no-update');
  assert.equal(empty.updater.phase, 'no-update');
  assert.equal(empty.updater.lastSuccessfulCheck.result, 'no-update');
  for (const sample of ['offline','metadata-refused','time-unavailable','identity-unavailable','policy-blocked','managed']) {
    const result = check(sample, empty);
    assert.equal(result.updater.phase, sample === 'managed' ? 'managed' : 'failed');
    assert.equal(result.updater.reason, sample);
    assert.equal(result.updater.packageStarted, false);
    assert.equal(result.updater.installStarted, false);
    assert.equal(result.updater.target, null);
    assert.deepEqual(result.updater.lastSuccessfulCheck, empty.updater.lastSuccessfulCheck);
  }
  assert.equal(step(action(installed(), 'update-check')).updater.phase, 'no-update', 'an already installed exact target is not offered again');
});

test('navigation keeps the operation alive without replacing another page or dialog with an offer', () => {
  const checking = action(about(), 'update-check');
  const settings = action(checking, 'page', {page:'settings'});
  const otherDialog = action(settings, 'modal', {modal:'profile'});
  const complete = step(otherDialog);
  assert.equal(complete.page, 'settings');
  assert.equal(complete.modal, 'profile');
  assert.equal(complete.updater.phase, 'available');
  assert.strictEqual(action(complete, 'update-offer'), complete);
  const reopened = action(action(action(complete, 'close'), 'page', {page:'about'}), 'update-offer');
  assert.equal(reopened.modal, 'update-offer');
  const underway = action(accept(reopened), 'page', {page:'status'});
  assert.equal(step(underway).updater.phase, 'verifying');
});

test('observer quit revokes pre-effect intent and leaves started installation unknown until reconciliation', () => {
  const quit = state => action(action(state, 'sample-tray-quit'), 'sample-tray-quit-confirm');
  const preEffect = accept(check());
  const cancelled = quit(preEffect);
  assert.equal(cancelled.updater.phase, 'cancelled');
  assert.equal(cancelled.updater.intent, null);
  assert.strictEqual(action(cancelled, 'update-step', {generation:preEffect.updater.generation,phase:preEffect.updater.phase}), cancelled);
  const underway = step(step(preEffect));
  const unknown = quit(underway);
  assert.equal(unknown.updater.phase, 'outcome-unknown');
  assert.equal(unknown.updater.installedIdentityKnown, false);
  assert.equal(unknown.updater.intent, null);
  const reopened = action(unknown, 'tray-observer-open');
  assert.strictEqual(action(reopened, 'update-check'), reopened);
  assert.equal(step(action(reopened, 'update-reconcile')).updater.phase, 'installed');
  const unavailable = action(preEffect, 'sample-notification-presenter', {value:'missing'});
  assert.equal(unavailable.updater.phase, 'cancelled');
  assert.equal(unavailable.updater.intent, null);
  assert.strictEqual(action(unavailable, 'update-step', {generation:preEffect.updater.generation,phase:preEffect.updater.phase}), unavailable);
});

test('scenario/reset generation cannot recycle an earlier callback identity', () => {
  for (const type of ['scenario','reset']) {
    const pending = action(about(), 'update-check');
    const guarded = action(pending, type, {id:'offline'});
    assert.equal(guarded.modal, 'reset-confirm');
    assert.strictEqual(action(guarded, 'update-step', {generation:pending.updater.generation,phase:'checking'}), guarded);
    const reset = action(guarded, 'confirm-reset');
    assert.ok(reset.updater.generation > pending.updater.generation);
    const fresh = action(action(reset, 'page', {page:'about'}), 'update-check');
    assert.strictEqual(action(fresh, 'update-step', {generation:pending.updater.generation,phase:'checking'}), fresh);
    assert.equal(step(fresh).updater.phase, 'available');
  }
});

test('closed sample IDs and exhausted generations preserve bounded state without fallback work', () => {
  const initial = about();
  for (const value of ['unknown','https://example.test/update.msix',null,{},'x'.repeat(4096)]) assert.strictEqual(action(initial, 'sample-update-result', {value}), initial);
  for (const generation of [Number.MAX_SAFE_INTEGER,-1,NaN]) {
    const exhausted = {...initial,updater:{...initial.updater,generation}};
    assert.strictEqual(action(exhausted, 'update-check'), exhausted);
  }
  const pending = accept(check());
  assert.strictEqual(action(pending, 'sample-update-invalidate', {value:'arbitrary-command'}), pending);
});
