'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const M = require('../src/model.js');
const V = require('../src/views.js');

const action = (state, type, fields = {}) => M.reduce(state, {type, ...fields});
const about = (mode = 'simple') => ({...M.initialState(), page:'about', mode});
const step = state => action(state, 'update-step', {generation:state.updater.generation, phase:state.updater.phase});
const check = (sample = 'available', mode = 'simple') => step(action(action(about(mode), 'sample-update-result', {value:sample}), 'update-check'));
const accept = state => action(state, 'update-accept', {generation:state.updater.generation, targetId:state.updater.target.id});
const finish = sample => step(step(step(accept(check(sample)))));

test('About exposes the same clearly simulated update entry point in both modes', () => {
  for (const mode of ['simple','power']) {
    const html = V.render(about(mode));
    assert.match(html, /<h1[^>]*>About NevroTick<\/h1>/);
    assert.match(html, /data-action="update-check"[^>]*>Check for updates<\/button>/);
    assert.match(html, /Local demonstration · sample versions and results/);
    assert.match(html, /No package is downloaded or installed/);
    for (const label of ['Installed · sample','Running app · sample','Channel · sample']) assert.ok(html.includes(label));
    assert.doesNotMatch(V.updateCard(about(mode)), /<a\b|data-action="update-accept"/);
  }
});

test('The native offer dialog presents one exact-target choice with a safe default', () => {
  for (const mode of ['simple','power']) {
    const state = check('available', mode);
    const html = V.modal(state);
    assert.match(html, /<dialog\b[^>]*aria-labelledby="dialog-title"/);
    assert.match(html, /<h1 id="dialog-title">Update now\?<\/h1>/);
    assert.match(html, /NevroTick 1\.1\.0/);
    assert.match(html, /48 MiB · complete package/);
    assert.match(html, /You will not need to confirm installation a second time/);
    assert.match(html, /data-action="update-later" autofocus>Later<\/button>/);
    assert.match(html, new RegExp(`data-action="update-accept" data-generation="${state.updater.generation}" data-target-id="${state.updater.target.id}"`));
    assert.equal((html.match(/data-action="update-accept"/g) || []).length, 1);
    for (const label of ['Updating from','During the update','Windows permission','Restart','Network']) assert.ok(html.includes(label));
    assert.match(html, /Windows prompts and actual package verification are not performed/);
  }
});

test('Progress communicates ordered stages and removes cancellation when installation starts', () => {
  let state = accept(check());
  for (const phase of ['downloading','verifying','installing']) {
    assert.equal(state.updater.phase, phase);
    const html = V.updateCard(state);
    assert.match(html, /<label[^>]*for="update-progress"/);
    assert.match(html, /<progress id="update-progress"/);
    assert.doesNotMatch(html, /<progress[^>]*\bvalue=/, 'no invented numeric progress');
    assert.equal((html.match(/aria-current="step"/g) || []).length, 1);
    assert.ok(html.indexOf('Download</li>') < html.indexOf('Install</li>') || html.includes('Download<span class="sr-only"> completed in sample'));
    assert.doesNotMatch(html, /data-action="update-accept"|data-action="update-check"/);
    if (phase === 'installing') {
      assert.doesNotMatch(html, /data-action="update-cancel"/);
      assert.match(html, /cancellation is no longer offered/);
    } else assert.match(html, /data-action="update-cancel"[^>]*>Cancel update<\/button>/);
    state = step(state);
  }
});

test('Check failures and verification refusals never masquerade as no update or offer a bypass', () => {
  assert.match(V.updateCard(check('no-update')), /No update available/);
  for (const sample of ['offline','metadata-refused','time-unavailable','identity-unavailable','policy-blocked','managed','package-refused','uac-denied']) {
    const state = ['package-refused','uac-denied'].includes(sample) ? step(step(accept(check(sample)))) : check(sample);
    const html = V.updateCard(state);
    assert.doesNotMatch(html, /No update available|Update installed/);
    assert.doesNotMatch(html, /data-action="update-accept"|data-action="update-offer"|>Install anyway</);
    assert.match(html, /failed verification never offers a bypass/);
  }
});

test('Unknown and recovery outcomes cannot offer a blind repeat install; app and Windows restarts remain distinct', () => {
  const unknown = finish('outcome-unknown');
  const unknownHtml = V.updateCard(unknown);
  assert.match(unknownHtml, /Unknown — check status/);
  assert.match(unknownHtml, /data-action="update-reconcile"[^>]*>Check installation status<\/button>/);
  assert.doesNotMatch(unknownHtml, /data-action="update-check"|data-action="update-accept"|data-action="update-restart"/);
  const reconciled = step(action(unknown, 'update-reconcile'));
  assert.match(V.updateCard(reconciled), /data-action="update-restart"[^>]*>Restart app<\/button>/);
  const restarted = action(reconciled, 'update-restart');
  assert.doesNotMatch(V.updateCard(restarted), /data-action="update-restart"/);
  assert.match(V.updateCard(restarted), /installed and running app versions now match/);
  for (const sample of ['recovery-required','restart-required']) {
    const html = V.updateCard(finish(sample));
    assert.doesNotMatch(html, /data-action="update-check"|data-action="update-accept"|data-action="update-cancel"/);
  }
  assert.match(V.updateCard(finish('restart-required')), /Restarting the app alone does not complete this step/);
});

test('Only final updater outcomes produce a polite status message', () => {
  let state = action(about(), 'update-check');
  assert.equal(V.updateAnnouncement(state), '');
  state = step(state);
  assert.equal(V.updateAnnouncement(state), '');
  state = accept(state);
  for (const phase of ['downloading','verifying','installing']) {
    assert.equal(state.updater.phase, phase);
    assert.equal(V.updateAnnouncement(state), '');
    state = step(state);
  }
  assert.match(V.updateAnnouncement(state), /Update installed.*Prototype result only/);
  assert.match(V.updateAnnouncement(check('offline')), /could not continue.*Prototype result only/);
});

test('Release notes and offer facts remain escaped literal text', () => {
  const state = check();
  const malicious = '</p><script>alert(1)</script><a href="https://invalid.example">Open</a>&';
  const html = V.updateOffer({...state,updater:{...state.updater,target:{...state.updater.target,notes:malicious,network:malicious}}});
  assert.doesNotMatch(html, /<script|<a\b/);
  assert.match(html, /&lt;script&gt;alert\(1\)&lt;\/script&gt;/);
  assert.match(html, /&amp;/);
});

test('Reviewer choices are labelled, fixed, and disabled for an active or offered operation', () => {
  const html = V.updateReviewer(about());
  assert.match(html, /<label for="sample-update-result">/);
  assert.equal((html.match(/<option\b/g) || []).length, Object.keys(M.UPDATE_SAMPLES).length);
  assert.doesNotMatch(html, /sample-update-invalidate/);
  for (const state of [check(), accept(check())]) assert.match(V.updateReviewer(state), /<select id="sample-update-result" disabled>/);
});
