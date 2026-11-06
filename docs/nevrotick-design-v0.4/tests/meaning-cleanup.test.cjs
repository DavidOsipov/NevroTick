'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const M = require('../src/model.js');
const V = require('../src/views.js');

test('product pages keep functional settings and omit implementation planning', () => {
  const state = {...M.initialState(), mode:'power'};
  const pages = [V.sources(state),V.recovery(state),...['schedule','policy','preferences'].map(settingsSection=>V.settings({...state,settingsSection}))];
  for (const html of pages) {
    assert.doesNotMatch(html,/Catalogue &amp; admission|Future source discovery|Implementation scope|RGT-0 qualification|Sample timing &amp; service policy|Shared scheduler|qualified production limits|Provider presets|exact correction bounds|unqualified universal threshold|automatic NTP reserve remains a future design/);
  }
  assert.match(V.settings(state),/Time check interval/);
  assert.match(V.settings(state),/Recovery and server limits may extend the actual interval/);
  const preferences = V.settings({...state,settingsSection:'preferences'});
  assert.doesNotMatch(preferences,/aria-label="Appearance"|<h3>Appearance<\/h3>/);
  assert.match(preferences,/Start NevroTick in the tray at sign-in/);
  assert.match(preferences,/id="sample-signin"[^>]*disabled/);
  assert.doesNotMatch(V.settings({...state,settingsSection:'preferences',sampleTray:true}),/id="sample-signin"[^>]*disabled/);
  assert.match(preferences,/Prototype — sample data/);
});

test('ordinary healthy recovery shows no unused evidence or empty ancestry card and collapses its explanation', () => {
  const state=M.initialState();
  for (const html of [V.recovery(state),V.diagnostics(state)]) {
    assert.doesNotMatch(html,/class="panel roughtime-evidence|class="recovery-summary|class="recovery-mascot|class="panel future-reserve/);
  }
  assert.match(V.recovery(state),/<details class="[^\"]*recovery-path[^\"]*"\s*><summary>Recovery path/);
  assert.match(V.recovery(state),/Try retained or strict NTS/);
  assert.match(V.recovery(state),/Explicit helper settings/);
});

test('active evidence stays visible, retained evidence is discoverable and future reserve stays in future scenes', () => {
  for (const id of ['roughtime','roughtime-date-ready','roughtime-unavailable','roughtime-rejected','signed-conflict']) {
    const state=M.initialState(id), before=JSON.stringify(state.scenario);
    const recovery=V.recovery(state), diagnostics=V.diagnostics(state);
    assert.match(recovery,/<details class="[^\"]*recovery-path[^\"]*" open>/);
    assert.match(recovery,/class="panel roughtime-evidence/);
    for (const html of [recovery,diagnostics]) {
      for (const fact of ['reason','signatureStatus','ntsHandoffState','ancestry','clockEffect']) assert.ok(html.includes(M.escapeHtml(state.scenario.roughtimeEvidence[fact])),id+': '+fact);
      assert.doesNotMatch(html,/class="panel future-reserve/);
    }
    assert.equal(JSON.stringify(state.scenario),before,'rendering must not change evidence or authority');
  }
  for (const view of [V.recovery,V.diagnostics]) {
    const html=view(M.initialState('roughtime-recovered'));
    assert.match(html,/<details class="[^\"]*roughtime-history[^\"]*"><summary>Date recovery history/);
    assert.match(html,/Signed Roughtime date basis/);
    assert.match(html,/fresh NTS agreement/);
  }
  for (const id of ['reserve','recovered']) assert.match(V.recovery(M.initialState(id)),/class="panel future-reserve/);
});

test('diagnostics distinguishes lost collection status from confirmed service unavailability and names every attention level', () => {
  const stale=V.diagnostics(M.initialState('stale'));
  const stopped=V.diagnostics(M.initialState('service-stopped'));
  assert.match(stale,/<h2>Service collection status is unknown<\/h2>/);
  assert.doesNotMatch(stale,/<h2>Service collection is unavailable<\/h2>|Unknown \/ unavailable/);
  assert.match(stopped,/<h2>Service collection is unavailable<\/h2>/);
  const state={...M.initialState('conflict'),eventFilter:'attention'};
  const html=V.diagnostics(state);
  assert.match(html,/Notices, warnings and errors/);
  assert.match(html,/<span class="event-level error">error<\/span>/,'the filter retains error events');
});

test('source preview rules and requested technology attribution remain without implementation jargon', () => {
  const editor=V.modal({...M.initialState(),modal:'add-source'});
  assert.match(editor,/Preview limit: 1–64 printable ASCII characters/);
  assert.doesNotMatch(editor,/UI-M0|production protocol rule|future service independently validates/);
  assert.equal(M.validateSource({name:'é',host:'time.example',protocol:'NTS',port:'4460'},[]).ok,false,'existing preview validation is unchanged');
  const about=V.about();
  for (const name of ['chrony','Tokio','garde','getrandom','windows-sys']) assert.ok(about.includes(name));
  assert.doesNotMatch(about,/Engine experiment|loopback/);
  assert.match(about,/synchronization is not yet available/);
});

test('Simple explains retained estimates, disagreement and recovery without architecture vocabulary', () => {
  for (const id of ['server-access-refused','all-rate-limited','holdover','offline']) {
    const state=M.initialState(id), before=JSON.stringify(state.scenario), html=V.simpleStatus(state);
    assert.match(html,/last valid time estimate|previous estimate is still usable/);
    assert.doesNotMatch(html,/previous model|previous time model|valid time model|Agreement not met/);
    assert.equal(JSON.stringify(state.scenario),before);
  }
  const refused=V.simpleStatus(M.initialState('server-access-refused'));
  assert.match(refused,/Beta refused access/);
  assert.match(refused,/Not enough time sources agree/);
  assert.match(V.simpleStatus(M.initialState('conflict')),/sources disagree/i);
  for (const id of ['roughtime','roughtime-date-ready','roughtime-unavailable','roughtime-rejected','signed-conflict']) {
    const state=M.initialState(id), before=JSON.stringify(state.scenario), html=V.simpleStatus(state);
    assert.match(html,/Date recovery has not changed your clock/);
    assert.match(html,/NTS checks must finish/);
    assert.match(html,/permission is still required/);
    assert.doesNotMatch(html,/NTS not yet established|separate correction authority|certificate-time basis|active.*bootstrap|admissible/);
    assert.equal(JSON.stringify(state.scenario),before);
  }
  assert.match(V.simpleStatus(M.initialState('leap-guard')),/leap-second transition/);
});

test('the Simple policy link names its actual Power user destination', () => {
  const state={...M.initialState(),page:'settings'};
  const html=V.simpleSettings(state);
  assert.match(html,/data-action="policy-page"[^>]*>Review time policy in Power user<\/button>/);
  assert.doesNotMatch(html,/Open advanced settings/);
  const destination=M.reduce(state,{type:'policy-page'});
  assert.equal(destination.mode,'power');
  assert.equal(destination.page,'settings');
  assert.equal(destination.settingsSection,'policy');
});

test('measurement reply and accepted measurement stay distinct and plain NTP authentication history is inapplicable', () => {
  const state=M.initialState(), before=JSON.stringify(state.scenario);
  assert.match(V.sources(state),/>Last measurement reply<\/th>/);
  const statistics=V.statistics(state);
  assert.match(statistics,/>Last measurement reply<\/th>/);
  assert.match(statistics,/>Last accepted measurement<\/th>/);
  assert.doesNotMatch(statistics,/Last valid reply|Preview check preference|The sample schedule is unchanged/);
  const delta=V.modal({...state,modal:'source-details',selectedSource:'delta'});
  assert.match(delta,/Last measurement reply<\/dt><dd>8 s ago/);
  assert.match(delta,/Last accepted measurement<\/dt><dd>Not accepted/);
  assert.match(delta,/Last authenticated time measurement<\/dt><dd>8 s ago/);
  assert.match(V.diagnostics(state),/Last measurement reply<\/dt>/);
  const ntp=M.initialState('ntp'), ntpBefore=JSON.stringify(ntp.scenario);
  const ntpDetails=V.modal({...ntp,modal:'source-details',selectedSource:'alpha'});
  assert.match(ntpDetails,/Last authenticated time measurement<\/dt><dd>Not applicable — plain NTP/);
  assert.equal(JSON.stringify(state.scenario),before);
  assert.equal(JSON.stringify(ntp.scenario),ntpBefore);
});

test('plain-NTP profile explicitly does not use date recovery while Roughtime remains available for NTS', () => {
  const ntp=M.initialState('ntp'), before=JSON.stringify(ntp.scenario);
  for (const html of [V.simpleSettings(ntp),V.settings({...ntp,mode:'power',settingsSection:'policy'}),V.recovery(ntp)]) {
    assert.match(html,/Date recovery is not used by the current plain-NTP profile/);
    assert.match(html,/used for secure NTS checks/);
  }
  const nts=M.initialState('roughtime-date-ready');
  assert.doesNotMatch(V.simpleSettings(nts),/not used by the current plain-NTP profile/);
  assert.match(V.recovery(nts),/Date basis handed to isolated NTS verification/);
  assert.equal(JSON.stringify(ntp.scenario),before);
});
