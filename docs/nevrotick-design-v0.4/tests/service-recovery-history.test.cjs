'use strict';
const test=require('node:test');
const assert=require('node:assert/strict');
const M=require('../src/model.js');
const V=require('../src/views.js');
const IDS=['healthy','roughtime','roughtime-date-ready','roughtime-recovered'];
function operation(id,outcome){
  const original=M.reduce(M.initialState(id),{type:'service-settings'});
  const requested=M.reduce(original,{type:'service-request',operation:outcome==='restart'?'restart':'stop'});
  let result=M.reduce(requested,{type:'service-confirm'});
  if(outcome!=='pending'){
    result=M.reduce(result,{type:'sample-service-response',value:outcome==='unknown'?'unknown':'confirmed'});
    result=M.reduce(result,{type:'sample-service-deliver'});
  }
  return {original,result};
}
for(const id of IDS) for(const outcome of ['pending','stop','restart','unknown']) {
  test(`${id} → ${outcome}: current recovery is unavailable and old evidence remains history`,()=>{
    const {original,result}=operation(id,outcome);
    const raw=original.scenario.roughtimeEvidence;
    const shown=M.presentState(result).scenario;
    const e=shown.roughtimeEvidence;
    assert.deepEqual(result.scenario,original.scenario,'raw fixture evidence/history is immutable');
    assert.equal(shown.recoveryStage,0);
    assert.match(shown.recoveryStageLabel,/Current recovery stage unavailable/);
    assert.equal(e.historical,true);
    const p=M.recoveryPresentation(shown);
    assert.equal(p.stage,0);
    assert.match(p.stageLabel,/Current recovery stage unavailable/);
    assert.doesNotMatch(p.label,/NTS tracking active|verification pending|Checking signatures/);
    if(id!=='healthy') {
      assert.equal(e.scope,'retained');
      assert.match(p.label,/^Previous /);
      assert.match(e.reason,/^Previous observation before service operation:/);
      assert.match(e.ntsHandoffState,/^Previous handoff before service operation:/);
      assert.match(e.receivedAge,/previous observation|previous service observation/);
      for(const source of e.sources) assert.match(source.receivedAge,/previous observation|previous service observation/);
    }
    assert.equal(e.state,raw.state);
    assert.equal(e.signatureStatus,raw.signatureStatus);
    assert.equal(e.approximateDateInterval,raw.approximateDateInterval);
    assert.equal(e.ancestry,raw.ancestry);
    assert.deepEqual(e.sources.map(x=>[x.host,x.keyIdentity,x.signatureStatus,x.approximateDateInterval]),raw.sources.map(x=>[x.host,x.keyIdentity,x.signatureStatus,x.approximateDateInterval]));
    for(const page of ['diagnostics','recovery']) {
      const html=V.render({...result,mode:'power',page});
      assert.doesNotMatch(html,/<li class="current">|Healthy NTS stops here|NTS tracking resumed in this fixture|NTS tracking active · signed date ancestry retained/);
      if(page==='recovery')assert.match(html,/Current recovery stage unavailable/);
      if(id==='healthy')assert.doesNotMatch(html,/class="detail-strip roughtime-history"/);
      else assert.match(html,/class="detail-strip roughtime-history"><summary>Date recovery history/);
    }
  });
}
test('a previous signed conflict remains historical evidence without becoming a new current-stage veto',()=>{
  const {original,result}=operation('signed-conflict','restart');
  const shown=M.presentState(result).scenario;
  const p=M.recoveryPresentation(shown);
  assert.equal(shown.roughtimeEvidence.state,'conflict');
  assert.equal(shown.roughtimeEvidence.scope,'retained');
  assert.equal(p.stage,0);
  assert.match(p.label,/Previous signed-date conflict/);
  assert.equal(shown.actionTarget,'diagnostics');
  assert.doesNotMatch(shown.title,/conflict/i);
  assert.deepEqual(result.scenario,original.scenario);
  assert.equal(shown.agree,0,'retaining history creates neither a current veto nor new agreement');
});
