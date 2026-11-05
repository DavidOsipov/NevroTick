'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const M = require('../src/model.js');
const V = require('../src/views.js');

test('one RATE source preserves healthy NTS agreement and its prior measurement/authentication ages', () => {
  const scene = M.getScenario('server-rate-limited');
  assert.equal(scene.id, 'server-rate-limited');
  assert.equal(scene.title, 'Time sync active');
  assert.equal(scene.agreement, 'Minimum + majority met');
  assert.equal(scene.profile, 'NTS only');
  assert.equal(scene.syncResponse, 'checking-available');
  assert.equal(scene.affectedSource, 'delta');
  const delta = scene.sources.find(x => x.id === 'delta');
  const original = M.getScenario('healthy').sources.find(x => x.id === 'delta');
  assert.equal(delta.role, 'Not used');
  assert.equal(delta.controlResponse, 'RATE');
  assert.equal(delta.requestStatus, 'Checks delayed by server');
  assert.equal(delta.lastControlResponse, '2 s ago');
  assert.equal(delta.reply, original.reply);
  assert.equal(delta.accepted, original.accepted);
  assert.equal(delta.lastAuthenticatedMeasurement, original.lastAuthenticatedMeasurement);
  assert.equal(delta.nextPermittedCheck, null);
  assert.deepEqual(scene.sources.filter(x => x.id !== 'delta'), M.getScenario('healthy').sources.filter(x => x.id !== 'delta'));
  const simple = V.simpleStatus(M.initialState(scene.id));
  assert.match(simple, /Time sync active/);
  assert.match(simple, /No settings change is needed/);
});

test('DENY, expired source evidence and failed agreement are separate published facts with a usable prior model', () => {
  const scene = M.getScenario('server-access-refused');
  assert.equal(scene.id, 'server-access-refused');
  assert.equal(scene.title, 'Using the last valid estimate');
  assert.equal(scene.agreement, 'Agreement not met');
  assert.match(scene.agreementNote, /published selection result/i);
  assert.equal(scene.profile, 'NTS only');
  assert.equal(scene.syncResponse, 'source-refused');
  assert.equal(scene.affectedSource, 'beta');
  assert.equal(scene.sourceHistorical, false);
  assert.equal(scene.retainedModel, true);
  assert.equal(scene.historical, false);
  assert.ok(Number.isFinite(scene.offset));
  const beta = scene.sources.find(x => x.id === 'beta');
  assert.equal(beta.role, 'Waiting');
  assert.equal(beta.controlResponse, 'DENY');
  assert.equal(beta.requestStatus, 'Access refused by server');
  assert.equal(beta.measurementHistorical, true);
  assert.equal(beta.measurementStatus, 'No longer usable by age');
  assert.equal(beta.reply, '7 min ago');
  assert.equal(beta.accepted, '7 min ago');
  assert.equal(beta.lastAuthenticatedMeasurement, '7 min ago');
  assert.equal(beta.lastControlResponse, '2 s ago');
  assert.equal(beta.nextPermittedCheck, null);
  assert.ok(scene.sources.filter(x => x.id !== 'beta').every(x => !x.measurementHistorical));
  const state = M.initialState(scene.id);
  assert.match(V.simpleStatus(state), /Based on the last valid time estimate/);
  assert.match(V.simpleStatus(state), /Review source in Power user/);
  assert.match(V.status({...state, mode:'power'}), /Retained model offset/);
  const sources = V.sources(state);
  assert.doesNotMatch(sources, /Historical source snapshot/);
  assert.equal((sources.match(/Prior evidence only/g) || []).length, 1);
  assert.equal((sources.match(/>Authenticated</g) || []).length, 3);
});

test('source details separate the control response from time and authentication evidence', () => {
  for (const [id, sourceId, code, next] of [
    ['server-rate-limited', 'delta', 'RATE', 'Scheduled by the service'],
    ['server-access-refused', 'beta', 'DENY', 'Not established']
  ]) {
    const state = {...M.initialState(id), modal:'source-details', selectedSource:sourceId};
    const view = V.modal(state);
    assert.match(view, /Request status<\/dt>/);
    assert.match(view, /Last control response<\/dt><dd>2 s ago/);
    assert.match(view, /Last accepted measurement<\/dt>/);
    assert.match(view, /Last authenticated time measurement<\/dt>/);
    assert.match(view, new RegExp(`Next permitted check<\\/dt><dd>${next}`));
    assert.match(view, new RegExp(`Control response<\\/dt><dd>${code}`));
    assert.match(view, /This response contains no time measurement\. It does not refresh the last accepted measurement\./);
    assert.match(view, /does not refresh.*authenticated time measurement/i);
    assert.doesNotMatch(view, /No rate-limit observation/);
  }
  assert.match(V.modal({...M.initialState('server-rate-limited'), modal:'source-details', selectedSource:'delta'}), /send requests less often/);
  assert.match(V.modal({...M.initialState('server-access-refused'), modal:'source-details', selectedSource:'beta'}), /Review its usage policy or configuration/);
  const alpha = V.modal({...M.initialState('server-access-refused'), modal:'source-details', selectedSource:'alpha'});
  assert.doesNotMatch(alpha, /Access refused by server|Historical source evidence|No longer usable by age/);
});

test('control diagnostics group repeated replies and name the affected identity and consequences', () => {
  for (const [id, sourceId, name] of [['server-rate-limited','delta','Delta'], ['server-access-refused','beta','Beta']]) {
    const state = M.initialState(id);
    const events = state.scenario.events.filter(x => x.sourceId === sourceId);
    assert.equal(events.length, 1);
    assert.ok(events[0].occurrences > 1);
    assert.match(events[0].title, new RegExp(name));
    assert.match(events[0].detail, id === 'server-rate-limited' ? /other three.*agreement/i : /Agreement not met.*previous model/i);
    const view = V.diagnostics(state);
    assert.match(view, new RegExp(`data-action="affected-source" data-value="${sourceId}"[^>]*>View affected source`));
    assert.match(view, /matching control responses grouped/);
  }
});

test('affected-source navigation opens the exact identity and visibly clears only conflicting filters', () => {
  for (const [id, sourceId] of [['server-rate-limited','delta'], ['server-access-refused','beta']]) {
    let state = M.initialState(id);
    const snapshot = JSON.stringify(state.scenario);
    state = M.reduce(state, {type:'source-query', value:'alpha'});
    state = M.reduce(state, {type:'source-filter', value:'used'});
    state = M.reduce(state, {type:'affected-source', id:sourceId});
    assert.equal(state.mode, 'power');
    assert.equal(state.page, 'sources');
    assert.equal(state.powerPage, 'sources');
    assert.equal(state.modal, 'source-details');
    assert.equal(state.selectedSource, sourceId);
    assert.equal(state.highlightedSource, sourceId);
    assert.equal(state.sourceQuery, '');
    assert.equal(state.sourceFilter, 'all');
    assert.equal(state.notice, 'Filters cleared to show the affected source');
    assert.equal(JSON.stringify(state.scenario), snapshot);
    const view = V.render(state);
    assert.match(view, /role="status">Filters cleared to show the affected source/);
    assert.match(view, new RegExp(`id="source-row-${sourceId}" class="[^"]*affected-source[^\"]*" aria-describedby="affected-source-note"`));
    assert.match(view, /id="affected-source-note"[^>]*>Affected source/);
    assert.match(view, new RegExp(`<code>${sourceId}\\.example<\\/code>`));
    assert.equal(M.reduce(state, {type:'affected-source', id:'missing'}), state);
  }
  let state = M.reduce(M.initialState('server-rate-limited'), {type:'source-filter', value:'other'});
  state = M.reduce(state, {type:'source-query', value:'Delta'});
  state = M.reduce(state, {type:'affected-source', id:'delta'});
  assert.equal(state.sourceFilter, 'other');
  assert.equal(state.sourceQuery, 'Delta');
  assert.equal(state.notice, '');
});


test('all-source limits distinguish current control observations from old time measurements without softening disconnected warnings', () => {
  const state=M.initialState('all-rate-limited');
  for(const page of [V.sources(state),V.statistics(state),V.modal({...state,modal:'source-details',selectedSource:'alpha'})]) {
    assert.match(page,/Historical time measurements/);
    assert.match(page,/current (?:control responses|request restrictions|observations)/i);
    assert.doesNotMatch(page,/These (?:rows|values).*do not establish.*current source/);
  }
  const details=V.modal({...state,modal:'source-details',selectedSource:'alpha'});
  assert.match(details,/Checks delayed by server/);
  assert.doesNotMatch(details,/No rate-limit observation|No source restriction observed/);
  const stale=M.initialState('stale');
  assert.match(V.sources(stale),/These rows do not establish the current source, service or clock state/);
  assert.match(V.statistics(stale),/The service connection is lost/);
  assert.match(V.modal({...stale,modal:'source-details',selectedSource:'alpha'}),/They do not establish current source or clock state/);
});
