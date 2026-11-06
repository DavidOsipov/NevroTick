'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const M = require('../src/model.js');
const V = require('../src/views.js');
const pending = ['roughtime','roughtime-date-ready','roughtime-unavailable','roughtime-rejected','signed-conflict'];

test('Roughtime v1 scenes preserve the boundary between signed dates, NTS and clock authority', () => {
  for (const id of pending) {
    const s = M.getScenario(id);
    assert.equal(s.id,id);
    assert.equal(s.future,false,id);
    assert.equal(s.agree,0); assert.equal(s.groups,0);
    assert.equal(s.offset,null); assert.equal(s.uncertainty,null);
    assert.match(s.appliedDetail,/not changed|No clock change/);
    assert.ok(s.sources.every(x => x.protocol === 'NTS' && x.role === 'Waiting' && !x.authenticated && x.offset === null && x.uncertainty === null && x.accepted === 'Not yet accepted' && x.lastAuthenticatedMeasurement === 'Not established'));
    assert.equal(s.syncResponse,'waiting');
    assert.deepEqual(M.syncOptions(s),['waiting','checking','already']);
    for (const key of ['state','reason','signatureStatus','approximateDateInterval','receivedAge','ntsHandoffState','ancestry','clockEffect','sources']) assert.ok(key in s.roughtimeEvidence,id+': '+key);
    assert.doesNotMatch(V.simpleStatus(M.initialState(id)),/Future design/);
    assert.match(V.simpleStatus(M.initialState(id)),/Date recovery has not changed your clock\. NTS checks must finish, enough time sources must agree, and permission is still required/);
    if (id.startsWith('roughtime-')) assert.match(V.simpleStatus(M.initialState(id)),/Secure NTS time checks are not ready yet/);
    assert.doesNotMatch(V.status(M.initialState(id)),/Future recovery design/);
  }
});

test('checking, usable date, unavailability, signature rejection and contradiction are distinct evidence facts', () => {
  const evidence = id => M.getScenario(id).roughtimeEvidence;
  assert.equal(evidence('roughtime').state,'checking');
  assert.equal(evidence('roughtime').approximateDateInterval,null);
  assert.equal(evidence('roughtime-date-ready').state,'date-ready');
  assert.ok(evidence('roughtime-date-ready').approximateDateInterval);
  assert.match(evidence('roughtime-date-ready').ntsHandoffState,/isolated NTS verification/i);
  assert.equal(evidence('roughtime-unavailable').state,'unavailable');
  assert.equal(evidence('roughtime-unavailable').approximateDateInterval,null);
  assert.match(evidence('roughtime-unavailable').reason,/no usable|No usable/);
  assert.equal(evidence('roughtime-rejected').state,'rejected');
  assert.equal(evidence('roughtime-rejected').approximateDateInterval,null);
  assert.match(evidence('roughtime-rejected').signatureStatus,/not verified/i);
  assert.equal(evidence('signed-conflict').state,'conflict');
  assert.match(evidence('signed-conflict').signatureStatus,/verified/i);
  assert.match(evidence('signed-conflict').reason,/active.*bootstrap/i);
  assert.equal(evidence('signed-conflict').approximateDateInterval,null);
  for (const id of pending) {
    assert.doesNotMatch(M.getScenario(id).title,/server lied|malicious|plain.NTP fallback/i);
  }
  assert.match(evidence('roughtime-rejected').reason,/does not prove the server lied/);
  assert.match(M.getScenario('roughtime-rejected').recoveryNote,/not proof of a malicious server/);
});

test('a healthy NTS path requests no Roughtime evidence and never depends on its historical result', () => {
  const state = M.initialState();
  const e = state.scenario.roughtimeEvidence;
  assert.equal(e.state,'not-needed'); assert.deepEqual(e.sources,[]);
  assert.equal(e.approximateDateInterval,null); assert.equal(e.receivedAge,null);
  assert.match(e.signatureStatus,/not requested/i);
  for (const id of ['roughtime-unavailable','roughtime-rejected','signed-conflict']) {
    const historical = {...state,scenario:{...state.scenario,roughtimeEvidence:{...M.getScenario(id).roughtimeEvidence,scope:'historical'}}};
    assert.equal(V.simpleStatus(historical),V.simpleStatus(state),'history must not change healthy status');
    assert.equal(V.status(historical),V.status(state));
    assert.equal(M.recoveryPresentation(historical.scenario).stage,1,'usable NTS has priority over historical recovery evidence');
  }
});

test('Roughtime success keeps signed date ancestry separate from fresh NTS and reserve ancestry', () => {
  const s = M.getScenario('roughtime-recovered');
  assert.equal(s.id,'roughtime-recovered'); assert.equal(s.future,false);
  assert.equal(s.title,'Time sync active'); assert.equal(s.authentication,'NTS authenticated');
  assert.equal(s.agree,3); assert.equal(s.total,4);
  assert.equal(s.roughtimeEvidence.state,'recovered');
  assert.equal(s.recovery,'Signed Roughtime date basis');
  assert.match(s.roughtimeEvidence.ancestry,/Signed Roughtime date basis/);
  assert.match(V.simpleStatus(M.initialState(s.id)),/Your clock is being kept in sync automatically/);
  assert.doesNotMatch(V.simpleStatus(M.initialState(s.id)),/Future design|recovery history retained/);
  for (const view of [V.recovery,V.diagnostics]) assert.match(view(M.initialState(s.id)),/Signed Roughtime date basis/);
  assert.match(M.summary(M.initialState(s.id)),/Signed Roughtime date basis/);
  for (const id of ['reserve','recovered']) {assert.equal(M.getScenario(id).future,true); assert.match(M.getScenario(id).recovery,/Unauthenticated NTP reserve/);}
});

test('every recovery surface uses scene evidence and demonstration identities without adding votes or policy controls', () => {
  for (const id of [...pending,'roughtime-recovered','healthy']) {
    const state = M.initialState(id), e = state.scenario.roughtimeEvidence;
    const recovery = V.recovery(state), diagnostics = V.diagnostics(state), sources = V.sources(state);
    assert.match(sources,/Date recovery sources/); assert.match(sources,/no synchronization votes/);
    assert.match(sources,/data-action="page" data-value="recovery"/);
    for (const html of [recovery,diagnostics]) {
      if (e.state === 'not-needed') {
        assert.doesNotMatch(html,/class="panel roughtime-evidence/);
        continue;
      }
      assert.ok(html.includes(M.escapeHtml(e.reason)));
      assert.ok(html.includes(M.escapeHtml(e.signatureStatus)));
      assert.ok(html.includes(M.escapeHtml(e.ntsHandoffState)));
      assert.match(html,/<summary>Roughtime technical identities/);
      assert.doesNotMatch(html,/type="checkbox"|trusted.key.editor/i);
    }
    assert.ok(recovery.indexOf('Try retained or strict NTS') < recovery.indexOf('Establish a date with Roughtime'));
    assert.ok(recovery.indexOf('Establish a date with Roughtime') < recovery.indexOf('Verify NTS in isolation'));
    assert.doesNotMatch(recovery,/Future design · automatic NTP reserve/);
    const list=recovery.match(/<ol class="steps">([\s\S]*?)<\/ol>/)[1];
    assert.doesNotMatch(list,/Khronos|Ananke|reserve/);
    for (const x of e.sources) {
      assert.match(x.host,/\.example$/); assert.match(x.keyIdentity,/DEMO/);
      assert.ok(recovery.includes(M.escapeHtml(x.host)));
      assert.ok(diagnostics.includes(M.escapeHtml(x.operatorGroup)));
      assert.ok(!state.scenario.sources.some(nts=>nts.host===x.host));
      assert.ok(!V.statistics(state).includes(x.host));
    }
  }
});

test('Simple settings and Power time policy enter recovery through existing guarded return context', () => {
  let state = {...M.initialState(),page:'settings'};
  const simple = V.simpleSettings(state);
  assert.match(simple,/Date recovery/); assert.match(simple,/Roughtime when needed/);
  assert.match(simple,/Review date recovery in Power user/);
  state=M.reduce(state,{type:'power-page',page:'recovery',context:{focusKey:{action:'power-page',value:'recovery'},scroll:{x:0,y:220}}});
  assert.equal(state.mode,'power'); assert.equal(state.page,'recovery');
  state=M.reduce(state,{type:'recovery-back'});
  assert.equal(state.mode,'simple'); assert.equal(state.page,'settings');
  assert.match(V.settings({...state,mode:'power',settingsSection:'policy'}),/Roughtime when needed/);
  const dirty=M.reduce(M.reduce({...state,mode:'power'}, {type:'recovery-edit'}),{type:'recovery-input',count:'1',helpers:['unfinished /','']});
  const guarded=M.reduce(dirty,{type:'power-page',page:'recovery'});
  assert.equal(guarded.leaveGuard.kind,'recovery-editor'); assert.equal(guarded.page,'settings');
  assert.match(V.about(),/v1.*Roughtime|Roughtime.*v1/);
});

test('evidence eligibility and presentation do not depend on scene names, reply counts or key labels', () => {
  const ready=M.initialState('roughtime-date-ready');
  for (const sources of [[],ready.scenario.roughtimeEvidence.sources.slice(0,1),ready.scenario.roughtimeEvidence.sources]) {
    const state={...ready,scenario:{...ready.scenario,id:'renamed-demonstration',roughtimeEvidence:{...ready.scenario.roughtimeEvidence,sources}}};
    assert.equal(M.recoveryPresentation(state.scenario).stage,3);
    assert.match(V.recovery(state),/Date basis handed to isolated NTS verification/);
    assert.equal(state.scenario.agree,0);
  }
  const copy=M.getScenario('roughtime-date-ready');
  copy.roughtimeEvidence.sources[0].keyIdentity='changed';
  assert.match(M.getScenario('roughtime-date-ready').roughtimeEvidence.sources[0].keyIdentity,/DEMO/);
  const hostile='<img src=x onerror=alert(1)>';
  const state={...ready,scenario:{...ready.scenario,roughtimeEvidence:{...ready.scenario.roughtimeEvidence,reason:hostile,sources:[{host:hostile,operatorGroup:hostile,keyIdentity:hostile,signatureStatus:hostile,approximateDateInterval:null,receivedAge:null}]}}};
  for (const html of [V.recovery(state),V.diagnostics(state)]) {
    assert.ok(html.includes(M.escapeHtml(hostile))); assert.ok(!html.includes(hostile));
  }
  const report=M.summary(ready);
  assert.match(report,/Roughtime signatures: Verified/);
  assert.match(report,/NTS handoff: Date basis handed/);
  assert.doesNotMatch(report,/date-[ab]\.example|DEMO-KEY/);
});
