'use strict';
const test = require('node:test');
const assert = require('node:assert/strict');
const M = require('../src/model.js');
const V = require('../src/views.js');

test('named fixtures are independent; a caller cannot mutate the source catalogue', () => {
  assert.equal(new Set(M.SCENARIOS.map(s=>s.id)).size,M.SCENARIOS.length);
  assert.ok(M.SCENARIOS.some(s=>s.id==='all-rate-limited'));
  const s=M.getScenario('healthy');s.sources[0].host='changed.example';
  assert.equal(M.getScenario('healthy').sources[0].host,'alpha.example');
  assert.equal(M.initialState('unknown').scenario.id,'healthy');
});
test('large forward proposal is consistent with the three agreeing source observations',()=>{
  const s=M.getScenario('correction');
  assert.equal(s.offset,372000);
  const admitted=s.sources.filter(x=>['Main','Supporting'].includes(x.role));
  assert.equal(admitted.length,3);
  for(const x of admitted) assert.ok(Math.abs(x.offset-372000)<=1,'proposal is within 1 ms of each agreeing fixture');
  assert.equal(s.applied,'None for this proposal');
  assert.match(M.summary(M.initialState('correction')),/not applied/);
  assert.match(V.modal({...M.initialState('correction'),modal:'correction'}),/10:06:11\.992–10:06:12\.008 UTC/);
});
test('positive and negative offset signs explain Windows behind/ahead independently',()=>{
  assert.equal(M.offsetLabel(10).description,'Windows is behind the reference');
  assert.equal(M.offsetLabel(-10).description,'Windows is ahead of the reference');
  assert.equal(M.offsetLabel(null).value,'—');
  assert.equal(M.signed(-0.7),'−0.7');
});
test('NTS authentication does not create a majority when two groups of two conflict',()=>{
  const s=M.getScenario('conflict');
  assert.ok(s.sources.every(x=>x.authenticated));
  assert.ok(s.sources.every(x=>x.role==='Not used'));
  assert.equal(s.agree,2);assert.match(s.agreement,/not met/);
  assert.equal(s.offset,null);assert.match(s.correction,/withheld/);
});
test('explicit NTP contains no authenticated source or mixed NTS vote',()=>{
  const s=M.getScenario('ntp');
  assert.ok(s.sources.every(x=>x.protocol==='NTP'&&!x.authenticated&&x.port===123));
  assert.match(s.sources[3].reason,/Unauthenticated NTP/);
  assert.match(s.authNote,/never an automatic downgrade/);
  assert.equal(s.recovery,'Not applicable — plain NTP');
  assert.ok(!M.summary(M.initialState('ntp')).includes('Strict NTS verification'));
});
test('lost status and confirmed stopped service remain different observations',()=>{
  const stale=M.getScenario('stale'), stopped=M.getScenario('service-stopped');
  assert.equal(stale.service,'Service health unknown');assert.equal(stale.connection,'Disconnected');
  assert.ok(stale.historical);assert.match(stale.estimate,/last snapshot/);
  assert.equal(stopped.service,'Stopped · confirmed');assert.equal(stopped.offset,null);
  assert.match(stale.description,/may still/);
  assert.ok(stale.sources.every(x=>x.reply.includes('at last snapshot')));
  assert.match(V.modal({...M.initialState('stale'),modal:'source-details',selectedSource:'alpha'}),/Historical source evidence/);
});
test('expiry and resume do not reuse current-looking measurements',()=>{
  for(const id of ['expired','resume']){
    const s=M.getScenario(id);assert.equal(s.offset,null);assert.ok(s.sources.every(x=>x.offset===null));assert.equal(s.agree,0);
  }
  assert.match(M.getScenario('resume').correctionDetail,/asleep is not counted/);
  assert.ok(M.getScenario('resume').sources.every(x=>x.reply.includes('Prior generation')));
  assert.match(M.getScenario('resume').applied,/age unavailable/);
  assert.notEqual(M.getScenario('holdover').offset,null);
  assert.equal(M.getScenario('holdover').agree,0);
});
test('date recovery helpers add no ordinary NTS votes or clock authority',()=>{
  for(const id of ['roughtime','reserve','signed-conflict']){
    const s=M.getScenario(id);assert.equal(s.future,id==='reserve');assert.equal(s.agree,0);assert.ok(s.sources.every(x=>x.role==='Waiting'&&!x.authenticated));assert.equal(s.offset,null);
  }
  assert.match(M.getScenario('reserve').recovery,/Unauthenticated/);
  assert.match(M.getScenario('signed-conflict').recovery,/retained/);
  const recovered=M.getScenario('recovered');assert.equal(recovered.authentication,'NTS authenticated');assert.match(recovered.recovery,/Unauthenticated/);
});
test('unknown leap coverage keeps healthy tracking; guarded event has no invented correction',()=>{
  const normal=M.getScenario('leap-unknown'), guard=M.getScenario('leap-guard');
  assert.equal(normal.tone,'good');assert.equal(normal.title,'Time sync active');assert.equal(normal.agree,3);
  assert.ok(normal.sources.every(x=>x.leap==='No announcement (LI=0)'));
  assert.equal(guard.offset,null);assert.match(guard.correction,/No predetermined/);assert.match(guard.calendarNote,/not been confirmed/);
});
test('unknown effect does not claim failure, applied success, or automatic retry',()=>{
  const s=M.getScenario('outcome-unknown');
  assert.match(s.applied,/Unknown/);assert.match(s.description,/may have completed/);assert.match(s.next,/do not repeat/);
  assert.equal(s.offset,null);assert.equal(s.agree,0);assert.ok(s.sources.every(x=>x.offset===null&&x.role==='Waiting'));
  const view=V.modal({...M.initialState('outcome-unknown'),modal:'result'});
  assert.ok(!/>Retry</.test(view));assert.match(view,/Reconcile with service/);assert.match(view,/disabled/);
});
test('host admission rejects URLs, injection, oversize and malformed addresses but admits useful neighbors',()=>{
  for(const host of ['https://a.example','a.example/path','user@a.example','a.example:123',' a.example','a..example','-a.example','a-.example','999.2.3.4','01.2.3.4',':1:2:3:4:5:6:7:8','1:2:3:4:5:6:7:8:','2001:::1','1::2::3','1::2:',':1::2','<script>','a'.repeat(64)+'.example']) assert.equal(M.validHost(host),false,host);
  for(const host of ['time.example','TIME.EXAMPLE.','192.0.2.1','2001:db8::1','::1','::','2001:db8:0:0:0:0:0:1','a'.repeat(63)+'.example']) assert.equal(M.validHost(host),true,host);
});
test('invalid direct model input preserves active sources and all previously accepted drafts',()=>{
  const state=M.reduce(M.initialState(),{type:'add-draft',raw:{name:'Epsilon',host:'epsilon.example',protocol:'NTS',port:'4460'}});
  const before=JSON.stringify({sources:state.scenario.sources,drafts:state.drafts});
  for(const raw of [{name:'Epsilon',host:'https://a.example',protocol:'NTS',port:'4460'},{name:'ok',host:'other.example',protocol:'TLS',port:'4460'},{name:'ok',host:'other.example',protocol:'NTS',port:'0'},{name:' bad ',host:'other.example',protocol:'NTS',port:'4460'}]){
    const result=M.reduce(state,{type:'add-draft',raw});assert.ok(Object.keys(result.editorError).length);assert.equal(JSON.stringify({sources:result.scenario.sources,drafts:result.drafts}),before);
  }
});
test('local add/edit/removal never changes active membership, agreement or evidence ages',()=>{
  let state=M.initialState();const initial=JSON.stringify(state.scenario);
  state=M.reduce(state,{type:'add-draft',raw:{name:'NTP preview',host:'epsilon.example',protocol:'NTP',port:'123'}});
  assert.equal(state.drafts[0].role,'Draft');assert.equal(state.drafts[0].authenticated,false);
  state=M.reduce(state,{type:'add-draft',id:'alpha',raw:{name:'Alpha edited',host:'alpha-new.example',protocol:'NTS',port:'4460'}});
  assert.equal(state.drafts[1].kind,'edit');
  state=M.reduce(state,{type:'remove-source',id:'beta'});assert.equal(state.drafts[2].kind,'remove');
  assert.equal(JSON.stringify(state.scenario),initial);
  state=M.reduce(state,{type:'discard-drafts'});assert.equal(state.drafts.length,0);assert.equal(JSON.stringify(state.scenario),initial);
});
test('duplicate identity comparison normalizes case and trailing dot and allows editing same identity',()=>{
  const base=M.initialState();
  const raw={name:'Duplicate',host:'ALPHA.EXAMPLE.',protocol:'NTS',port:'4460'};
  assert.equal(M.validateSource(raw,base.scenario.sources).ok,false);
  assert.equal(M.validateSource(raw,base.scenario.sources,'alpha').ok,true);
  assert.equal(M.validateSource({...raw,protocol:'NTP',port:'123'},base.scenario.sources).ok,true);
});
test('navigation preserves drafts; explicit scenario reset discards them',()=>{
  let state=M.reduce(M.initialState(),{type:'add-draft',raw:{name:'Test',host:'test.example',protocol:'NTS',port:'4460'}});
  state=M.reduce(state,{type:'page',page:'diagnostics'});assert.equal(state.drafts.length,1);
  state=M.reduce(state,{type:'scenario',id:'healthy'});assert.equal(state.drafts.length,1);
  state=M.reduce(state,{type:'confirm-reset'});assert.equal(state.drafts.length,0);
});
test('search and filters select actual active rows without editing state',()=>{
  let state=M.initialState();assert.equal(M.visibleSources(state).length,4);
  state=M.reduce(state,{type:'source-filter',value:'used'});assert.equal(M.visibleSources(state).length,3);
  state=M.reduce(state,{type:'source-query',value:'Operator B'});assert.equal(M.visibleSources(state)[0].id,'beta');
  state=M.reduce(state,{type:'source-filter',value:'other'});assert.equal(M.visibleSources(state).length,0);
});
test('printable hostile labels are rendered literally, never as HTML',()=>{
  const state=M.reduce(M.initialState(),{type:'add-draft',raw:{name:'<img src=x onerror=alert(1)>',host:'safe.example',protocol:'NTS',port:'4460'}});
  assert.equal(state.drafts.length,1);
  const html=V.sources(state);assert.ok(html.includes('&lt;img src=x onerror=alert(1)&gt;'));assert.ok(!html.includes('<img src=x'));
  assert.equal(M.escapeHtml('a&b<>"\''),'a&amp;b&lt;&gt;&quot;&#39;');
});
test('every scenario renders all seven Power pages and applicable dialogs with fixture labelling',()=>{
  for(const s of M.SCENARIOS) for(const page of ['status','sources','statistics','recovery','diagnostics','settings','about']){
    const state={...M.initialState(s.id),mode:'power',page};const html=V.render(state);
    assert.match(html,/Prototype — sample data/);assert.match(html,/<main id="main"/);assert.match(html,/id="page-title"/);
    assert.ok(!html.includes('undefined'),s.id+'/'+page);
    for(const modal of ['source-details','add-source','review','export','result']) assert.match(V.render({...state,modal,selectedSource:'alpha'}),/PROTOTYPE — SAMPLE DATA/);
  }
});
test('diagnostic summary uses fixture data only and redacts endpoint identities',()=>{
  const text=M.summary(M.initialState('recovered'));
  assert.match(text,/DESIGN FIXTURE ONLY/);assert.match(text,/Unauthenticated NTP reserve/);
  assert.ok(!text.includes('alpha.example'));assert.ok(!text.includes('192.0.2.11'));
  assert.match(text,/No system data collected/);
});
test('expired correction, signed conflict, observation mode and logging retain their separate meanings',()=>{
  const expired=M.getScenario('correction-expired');assert.equal(expired.agree,0);assert.equal(expired.groups,0);
  const conflict=M.getScenario('signed-conflict');assert.ok(!conflict.authNote.includes('authentication is verified'));
  const observe=M.getScenario('observe');assert.match(observe.ownership,/NevroTick/);assert.match(observe.correctionDetail,/does not automatically re-enable Windows Time/);
  for(const id of ['stale','service-stopped']){
    const view=V.diagnostics(M.initialState(id));assert.match(view,id==='stale'?/Service collection status is unknown/:/Service collection is unavailable/);assert.ok(!view.includes('Normal logging is active'));
  }
});
test('current conflicting replies are distinct from an old aggregate estimate and recovery references claim no healthy NTS',()=>{
  const s=M.getScenario('conflict');assert.ok(s.historical);assert.equal(s.sourceHistorical,false);
  const sources=V.sources(M.initialState('conflict'));assert.match(sources,/Authenticated/);assert.ok(!sources.includes('Prior evidence only'));assert.ok(!sources.includes('Historical source snapshot'));
  for(const id of ['ntp','stale','expired','service-stopped','conflict']) assert.ok(!V.recovery(M.initialState(id)).includes('Healthy NTS stops here'),id);
  assert.match(V.recovery(M.initialState('healthy')),/Healthy NTS stops here/);
  assert.match(V.recovery(M.initialState('recovered')),/NTS tracking resumed in this fixture/);
});
test('editing an added draft and a name-only diff is visible without creating a duplicate',()=>{
  let state=M.reduce(M.initialState(),{type:'add-draft',raw:{name:'First name',host:'new.example',protocol:'NTS',port:'4460'}});
  const id=state.drafts[0].id;
  state=M.reduce(state,{type:'add-draft',id,raw:{name:'Better name',host:'new.example',protocol:'NTS',port:'4460'}});
  assert.equal(state.drafts.length,1);assert.equal(state.drafts[0].name,'Better name');
  state=M.reduce(state,{type:'add-draft',id:'alpha',raw:{name:'New Alpha name',host:'alpha.example',protocol:'NTS',port:'4460'}});
  const html=V.modal({...state,modal:'review'});assert.match(html,/Alpha — alpha.example/);assert.match(html,/New Alpha name — alpha.example/);
});

test('Simple is the default and exposes two navigation destinations and one status section',()=>{
  const state=M.initialState(), html=V.render(state);
  assert.equal(state.mode,'simple'); assert.equal(state.page,'status');
  const nav=html.match(/<nav class="app-nav"[\s\S]*?<\/nav>/)[0];
  assert.deepEqual([...nav.matchAll(/data-value="([^"]+)"/g)].map(x=>x[1]),['status','settings','about']);
  const main=html.match(/<main[^>]*>([\s\S]*?)<\/main>/)[1];
  assert.equal((main.match(/<section /g)||[]).length,1);
  assert.ok(!/Time evidence|UTC CALENDAR|CLOCK ACTIVITY|Evidence ages/.test(main));
  assert.match(main,/0\.7 ms <span>ahead/); assert.match(main,/uncertainty ±4\.8 ms/);
  assert.match(main,/Last confirmed time check/); assert.match(main,/18 s ago/);
  assert.match(html,/data-action="view-mode" data-value="power"/);
  assert.ok(!/<details id="reviewer-controls"[^>]*\sopen[\s>]/.test(html));
});
test('mode switching preserves scenario, source drafts, filters, settings and unsaved cadence',()=>{
  let state=M.reduce(M.initialState('conflict'),{type:'view-mode',mode:'power'});
  state=M.reduce(state,{type:'add-draft',raw:{name:'Retained source',host:'new.example',protocol:'NTS',port:'4460'}});
  state=M.reduce(state,{type:'frequency-draft',value:'900'});
  state=M.reduce(state,{type:'source-filter',value:'other'});
  state=M.reduce(state,{type:'source-query',value:'Delta'});
  state=M.reduce(state,{type:'event-filter',value:'attention'});
  state=M.reduce(state,{type:'settings',value:'policy'});
  state=M.reduce(state,{type:'page',page:'statistics'});
  const retained=JSON.stringify({scenario:state.scenario,drafts:state.drafts,sourceFilter:state.sourceFilter,sourceQuery:state.sourceQuery,eventFilter:state.eventFilter,settingsSection:state.settingsSection,frequencyDraft:state.frequencyDraft,frequencyPreview:state.frequencyPreview});
  state=M.reduce(state,{type:'view-mode',mode:'simple'});assert.equal(state.page,'status');
  assert.match(V.simpleSettings(state),/1 source preview change is retained/);
  state=M.reduce(state,{type:'view-mode',mode:'power'});assert.equal(state.page,'statistics');
  assert.equal(JSON.stringify({scenario:state.scenario,drafts:state.drafts,sourceFilter:state.sourceFilter,sourceQuery:state.sourceQuery,eventFilter:state.eventFilter,settingsSection:state.settingsSection,frequencyDraft:state.frequencyDraft,frequencyPreview:state.frequencyPreview}),retained);
});
test('shared Settings stays open across modes and Simple cannot navigate silently to a detailed page',()=>{
  let state=M.reduce(M.initialState(),{type:'page',page:'settings'});
  state=M.reduce(state,{type:'view-mode',mode:'power'});assert.equal(state.page,'settings');
  state=M.reduce(state,{type:'view-mode',mode:'simple'});assert.equal(state.page,'settings');
  state=M.reduce(state,{type:'page',page:'sources'});assert.equal(state.page,'status');assert.equal(state.mode,'simple');
  state=M.reduce(state,{type:'power-page',page:'sources'});assert.equal(state.page,'sources');assert.equal(state.mode,'power');
  assert.equal(M.reduce(state,{type:'view-mode',mode:'invalid'}),state);
  state=M.reduce(state,{type:'power-page',page:'bad-input'});assert.equal(state.page,'status');assert.equal(state.powerPage,'status');
});
test('a presentation switch cannot rerender and destroy a partially edited modal',()=>{
  let state=M.reduce(M.initialState(),{type:'view-mode',mode:'power'});
  state=M.reduce(state,{type:'modal',modal:'edit-source',id:'alpha'});
  assert.equal(M.reduce(state,{type:'view-mode',mode:'simple'}),state);
  const html=V.render(state), group=html.match(/<div class="mode-switch"[\s\S]*?<\/div>/)[0];
  assert.equal((group.match(/\sdisabled/g)||[]).length,2);
});
test('preview cadence saves and cancels locally without changing any fixture timing or authority',()=>{
  let state=M.initialState();const fixture=JSON.stringify(state.scenario);
  state=M.reduce(state,{type:'frequency-draft',value:'300'});assert.equal(state.frequencyPreview,'auto');assert.equal(state.frequencyDraft,'300');
  assert.match(V.simpleSettings(state),/Unsaved selection/);
  state=M.reduce(state,{type:'frequency-save'});assert.equal(state.frequencyPreview,'300');
  assert.match(state.notice,/Preview updated: Every 5 minutes/);
  state=M.reduce(state,{type:'frequency-draft',value:'120'});
  state=M.reduce(state,{type:'frequency-cancel'});assert.equal(state.frequencyDraft,'300');assert.equal(state.frequencyCustom,'300');
  assert.equal(JSON.stringify(state.scenario),fixture);assert.equal(state.scenario.next,'Next scheduled check in 40 s');
  assert.doesNotMatch(V.settings({...state,mode:'power',settingsSection:'schedule'}),/Sample timing/);
  assert.match(V.settings({...state,mode:'power',settingsSection:'schedule'}),/Every 5 minutes/);
  assert.equal(state.scenario.profile,'NTS only');assert.equal(state.scenario.ownership,'NevroTick');
});
test('invalid cadence has no configuration effect and a prior manual preference survives Automatic',()=>{
  let state=M.reduce(M.initialState(),{type:'frequency-draft',value:'900'});
  state=M.reduce(state,{type:'frequency-draft',value:'auto'});assert.equal(state.frequencyCustom,'900');
  state=M.reduce(state,{type:'frequency-draft',value:state.frequencyCustom});assert.equal(state.frequencyDraft,'900');
  state=M.reduce(state,{type:'frequency-save'});
  for(const value of ['-1','0','60','300.0','3600',300,null,'<script>']){
    const result=M.reduce(state,{type:'frequency-draft',value});
    assert.equal(result.frequencyPreview,'900');assert.equal(result.frequencyDraft,'900');assert.ok(result.frequencyError);
    assert.equal(result.scenario,state.scenario);
  }
});
test('review scenario changes keep the mode and cadence; explicit reset clears previews only',()=>{
  let state=M.reduce(M.initialState(),{type:'view-mode',mode:'power'});
  state=M.reduce(state,{type:'frequency-draft',value:'900'});state=M.reduce(state,{type:'frequency-save'});
  state=M.reduce(state,{type:'add-draft',raw:{name:'Epsilon',host:'new.example',protocol:'NTS',port:'4460'}});
  state=M.reduce(state,{type:'scenario',id:'stale'});
  assert.equal(state.modal,'reset-confirm');state=M.reduce(state,{type:'confirm-reset'});
  assert.equal(state.mode,'power');assert.equal(state.frequencyPreview,'900');assert.equal(state.drafts.length,0);
  state=M.reduce(state,{type:'reset'});state=M.reduce(state,{type:'confirm-reset'});assert.equal(state.scenario.id,'stale');assert.equal(state.mode,'power');assert.equal(state.frequencyPreview,'auto');
});
test('Simple retains the consequence of each material degraded state without live-looking old values',()=>{
  const mustHave={ntp:/Unauthenticated NTP/,observe:/clock changes disabled/,conflict:/New clock corrections are paused/,blocked:/Clock changes blocked/,stale:/Current service status is unknown/,expired:/previous estimate is too old/,holdover:/last valid time estimate/,offline:/previous estimate is still usable/,resume:/Previous measurements are no longer current/,'service-stopped':/Service stopped · confirmed/,'leap-guard':/new clock change is paused/i,correction:/has not been applied/,'correction-expired':/too old to approve/,'outcome-unknown':/may have completed/,'signed-conflict':/remain blocked/};
  for(const [id,expected] of Object.entries(mustHave)) assert.match(V.simpleStatus(M.initialState(id)),expected,id);
  for(const id of ['stale','expired','conflict','resume','service-stopped','correction-expired','outcome-unknown','leap-guard']){
    const html=V.simpleStatus(M.initialState(id));assert.match(html,/Current clock difference/);assert.match(html,/Not available/);assert.ok(!html.includes('class="simple-offset"'),id);
  }
  for(const id of ['reserve','recovered']) assert.match(V.simpleStatus(M.initialState(id)),/Future design/);
  for(const id of ['roughtime','signed-conflict']) assert.doesNotMatch(V.simpleStatus(M.initialState(id)),/Future design/);
  assert.match(V.simpleStatus(M.initialState('reserve')),/unauthenticated date hint/);
  assert.match(V.simpleStatus(M.initialState('recovered')),/earlier recovery date came from unauthenticated NTP/);
  assert.match(V.simpleStatus(M.initialState('correction')),/6 min 12 s <span>behind/);
});
test('statistics separates current conflicting observations, stale snapshots and retained model estimates',()=>{
  const conflict=V.statistics(M.initialState('conflict'));
  assert.match(conflict,/Source offset comparison/);assert.match(conflict,/−30\.0 ms/);assert.match(conflict,/\+41\.0 ms/);assert.ok(!conflict.includes('Historical source evidence'));
  const stale=V.statistics(M.initialState('stale'));
  assert.match(stale,/Historical source offsets/);assert.match(stale,/18 s at last snapshot/);assert.match(stale,/Last known agreement/);
  for(const id of ['holdover','offline']){
    const html=V.statistics(M.initialState(id));assert.match(html,/Retained model offset/);assert.match(html,/No current source measurements/);assert.ok(!html.includes('<svg'));
  }
  const proposal=V.statistics(M.initialState('correction'));assert.match(proposal,/Scale is zoomed/);assert.match(proposal,/\+372000\.0 ms/);
  const ntp=V.statistics(M.initialState('ntp'));assert.match(ntp,/None · plain NTP/);
});

test('policy guidance opens Time policy even when Synchronization was last selected',()=>{
  let state={...M.initialState('blocked'),modal:'status-help'};
  assert.match(V.modal(state),/data-action="policy-page"/);
  assert.match(V.status({...state,modal:null,mode:'power'}),/data-action="policy-page"/);
  assert.match(V.sources(M.initialState()),/data-action="policy-page"/);
  state=M.reduce(state,{type:'policy-page'});
  assert.equal(state.mode,'power');assert.equal(state.page,'settings');assert.equal(state.settingsSection,'policy');assert.equal(state.modal,null);
  assert.match(V.render(state),/Clock ownership/);assert.match(V.render(state),/Conflict detected/);
});
test('check preferences do not claim a known active schedule for unavailable service state',()=>{
  const stale=V.frequencySettings({...M.initialState('stale'),mode:'power'});
  const stopped=V.frequencySettings({...M.initialState('service-stopped'),mode:'power'});
  assert.doesNotMatch(stale,/Sample timing|Next scheduled check/);assert.doesNotMatch(stopped,/Sample timing|Next scheduled check/);
  assert.ok(!stale.includes('Active sample schedule'));assert.ok(!stopped.includes('Active sample schedule'));
  assert.match(V.statistics(M.initialState('stale')),/measurements and source state below/);
});

test('own interval saves positive whole minutes without changing the fixture schedule', () => {
  let state = M.initialState();
  const next = state.scenario.next;
  state = M.reduce(state, {type:'frequency-draft', value:'own:1'});
  state = M.reduce(state, {type:'frequency-save'});
  assert.equal(state.frequencyPreview, 'own:1');
  assert.equal(M.frequencyLabel(state.frequencyPreview), 'Every 1 minute');
  assert.equal(state.scenario.next, next);
  state = M.reduce(state, {type:'frequency-draft', value:'own:123456789123456789'});
  state = M.reduce(state, {type:'frequency-save'});
  assert.equal(M.frequencyLabel(state.frequencyPreview), 'Every 123456789123456789 minutes');
});

test('invalid own intervals cannot replace an accepted preview', () => {
  let saved=M.reduce(M.initialState(),{type:'frequency-draft',value:'300'});
  saved=M.reduce(saved,{type:'frequency-save'});
  for (const value of ['', '0', '-1', '1.5', '1e2', '+1', ' 1']) {
    let state=M.reduce(saved,{type:'frequency-draft',value:'own:'+value});
    state=M.reduce(state,{type:'frequency-save'});
    assert.equal(state.frequencyPreview,'300',value);
    assert.ok(state.frequencyError, value);
  }
});

test('own interval survives presentation switching, cancels unsaved edits and resets', () => {
  let state = M.reduce(M.initialState(), {type:'frequency-draft',value:'own:17'});
  state = M.reduce(state, {type:'frequency-save'});
  state = M.reduce(state, {type:'frequency-draft',value:'own:25'});
  state = M.reduce(state, {type:'view-mode',mode:'power'});
  assert.equal(state.frequencyDraft, 'own:25');
  assert.equal(state.frequencyPreview, 'own:17');
  assert.match(V.settings({...state,settingsSection:'schedule'}), /value="25"/);
  state = M.reduce(state, {type:'frequency-cancel'});
  assert.equal(state.frequencyDraft, 'own:17');
  assert.equal(state.frequencyOwnDraft, '17');
  state = M.reduce(state, {type:'view-mode',mode:'simple'});
  assert.match(V.simpleSettings(state), /Every 17 minutes/);
  state = M.reduce(state, {type:'reset'});
  state = M.reduce(state, {type:'confirm-reset'});
  assert.equal(state.frequencyPreview, 'auto');
});

test('cancelling an unsaved custom value restores the last saved own interval', () => {
  let state=M.reduce(M.initialState(),{type:'frequency-draft',value:'own:7'});
  state=M.reduce(state,{type:'frequency-save'});
  state=M.reduce(state,{type:'frequency-draft',value:'auto'});
  state=M.reduce(state,{type:'frequency-save'});
  state=M.reduce(state,{type:'frequency-draft',value:'own:20'});
  state=M.reduce(state,{type:'frequency-cancel'});
  state=M.reduce(state,{type:'frequency-draft',value:state.frequencyCustom});
  assert.equal(state.frequencyDraft,'own:7');
  assert.equal(state.frequencyOwnDraft,'7');
});

test('profile preview is a draft and does not change the active sample', () => {
  const initial=M.initialState('healthy');
  const draft=M.reduce(initial,{type:'policy-draft',value:'ntp'});
  assert.equal(draft.policyDraft,'ntp');
  assert.equal(draft.scenario.profile,'NTS only');
  assert.match(V.settings({...draft,mode:'power',settingsSection:'policy'}),/NTP.*unauthenticated/i);
  const discarded=M.reduce(draft,{type:'policy-discard'});
  assert.equal(discarded.policyDraft,null);
  assert.equal(discarded.scenario.profile,'NTS only');
});

test('recovery helper preview requires valid distinct identities and never downgrades', () => {
  const original=M.initialState();
  const bad=M.reduce(original,{type:'recovery-preview',count:'2',helpers:['one.example','']});
  assert.equal(bad.recoveryDraft,null);
  assert.ok(Object.keys(bad.recoveryErrors).length);
  const duplicate=M.reduce(original,{type:'recovery-preview',count:'2',helpers:['one.example','ONE.example.']});
  assert.equal(duplicate.recoveryDraft,null);
  assert.ok(Object.keys(duplicate.recoveryErrors).length);
  const good=M.reduce(original,{type:'recovery-preview',count:'2',helpers:['one.example','two.example']});
  assert.deepEqual(good.recoveryDraft,{count:'2',helpers:['one.example','two.example']});
  assert.equal(good.scenario.recovery,original.scenario.recovery);
  assert.match(V.modal({...good,modal:'recovery-review'}),/both/i);
  assert.match(V.modal({...good,modal:'recovery-review'}),/unauthenticated/i);
  const discarded=M.reduce(good,{type:'recovery-discard'});
  assert.equal(discarded.recoveryDraft,null);
});

test('cancel editing a source opened from details returns to those details', () => {
  let state=M.reduce(M.initialState(),{type:'modal',modal:'source-details',id:'alpha'});
  state=M.reduce(state,{type:'modal',modal:'edit-source',id:'alpha'});
  state=M.reduce(state,{type:'close'});
  assert.equal(state.modal,'source-details');
  assert.equal(state.selectedSource,'alpha');
});

test('helper preview offers off and one helper without treating either as authenticated time', () => {
  const original=M.initialState();
  const off=M.reduce(original,{type:'recovery-preview',count:'0',helpers:['discarded.example','ignored.example']});
  assert.deepEqual(off.recoveryDraft,{count:'0',helpers:[]});
  const one=M.reduce(original,{type:'recovery-preview',count:'1',helpers:['one.example','unused.example']});
  assert.deepEqual(one.recoveryDraft,{count:'1',helpers:['one.example']});
  assert.match(V.modal({...one,modal:'recovery-review'}),/One helper is weaker/i);
  assert.match(V.modal({...one,modal:'recovery-review'}),/cannot vote.*or set the Windows clock/i);
  assert.equal(one.scenario.authentication,original.scenario.authentication);
});

test('Settings contains only settings and handles obsolete sections without an empty screen', () => {
  const state={...M.initialState(),mode:'power',page:'settings'};
  for (const section of ['schedule','policy','preferences','service','notifications']) {
    const html=V.settings({...state,settingsSection:section});
    assert.doesNotMatch(html,/Installation &amp; support|Installation & support|One complete installation|MSIX preparation|Store submission|Setup and recovery examples|Native qualification remains deferred/);
    assert.equal((html.match(/data-action="settings"/g)||[]).length,5);
  }
  const next=M.reduce(state,{type:'settings',value:'delivery'});
  assert.equal(next.settingsSection,'schedule');
  const stale=V.settings({...state,settingsSection:'delivery'});
  assert.match(stale,/id="frequency-form"/);
  assert.match(stale,/data-value="schedule" aria-current="page"/);
  assert.doesNotMatch(V.simpleSettings(M.initialState()),/installation details/);
});

test('power status keeps the first-screen essentials without duplicate diagnostic panels', () => {
  const html = V.status(M.initialState('healthy'));
  assert.match(html, /class="status-actions"/);
  assert.match(html, /data-action="sync-now"/);
  assert.match(html, /NTS authenticated/);
  assert.match(html, /3 of 4 identities/);
  assert.match(html, /Last qualified estimate/);
  assert.match(html, /Estimated offset/);
  assert.match(html, /Estimated uncertainty/);
  assert.ok(!html.includes('CLOCK ACTIVITY'));
  assert.ok(!html.includes('UTC CALENDAR'));
  assert.ok(!html.includes('Evidence ages &amp; recovery history'));
});

test('power status preserves impaired state, historical qualifiers and the right next destination', () => {
  const stale = V.status(M.initialState('stale'));
  assert.match(stale, /Current status unavailable/);
  assert.match(stale, /current health is unknown/);
  assert.match(stale, /Historical evidence/);
  assert.match(stale, /Last known offset/);
  assert.match(stale, /NTS · last snapshot/);
  assert.match(stale, /18 s at last snapshot/);
  const stopped = V.status(M.initialState('service-stopped'));
  assert.match(stopped, /Background service is stopped/);
  assert.match(stopped, /No live correction status/);
  const conflict = V.status(M.initialState('conflict'));
  assert.match(conflict, /NTS authenticated/);
  assert.match(conflict, /Majority not met/);
  assert.match(conflict, /data-action="page" data-value="statistics"/);
  assert.match(conflict, /Compare sources/);
  for (const id of ['leap-guard', 'leap-unknown']) assert.match(V.status(M.initialState(id)), /UTC/);
  assert.match(V.status(M.initialState('outcome-unknown')), /Outcome unknown/);
});

test('diagnostics keeps four distinct ages, recovery provenance and contextual action', () => {
  const normal = V.diagnostics(M.initialState('healthy'));
  for (const label of ['Last measurement reply','Qualified estimate','Applied correction','Status snapshot','Certificate-time basis','UTC calendar']) assert.ok(normal.includes(label), label);
  assert.ok(!normal.includes('Three independently operated sources agree. Your clock is being maintained under the current policy.'));
  assert.match(normal, /No current issue/);
  const conflict = V.diagnostics(M.initialState('conflict'));
  assert.match(conflict, /Time sources disagree/);
  assert.match(conflict, /New correction withheld/);
  assert.match(conflict, /data-action="page" data-value="statistics"/);
  const stale = V.diagnostics(M.initialState('stale'));
  assert.match(stale, /Service health unknown/);
  assert.match(stale, /Disconnected/);
  const recovered = V.diagnostics(M.initialState('recovered'));
  assert.match(recovered, /Unauthenticated NTP reserve/);
  assert.match(recovered, /Retained certificate-time ancestry/);
});

test('statistics presents the source comparison before optional summary', () => {
  const html = V.statistics(M.initialState('healthy'));
  assert.ok(html.indexOf('Source comparison') < html.indexOf('Agreement policy'));
  assert.match(html, /<details[^>]*>\s*<summary>Measurement context/);
  assert.ok(!html.includes('class="statistics-summary"'));
});

test('status and diagnostics keep situation-specific evidence and recovery access', () => {
  for (const scenario of M.SCENARIOS) {
    const status = V.status(M.initialState(scenario.id));
    for (const value of [scenario.title, scenario.description, scenario.authentication, scenario.agreement, scenario.estimate]) {
      assert.ok(status.includes(M.escapeHtml(value)), `${scenario.id}: missing ${value}`);
    }
    const diag = V.diagnostics(M.initialState(scenario.id));
    for (const value of [scenario.connection, scenario.service, scenario.recovery, scenario.recoveryNote]) {
      assert.ok(diag.includes(M.escapeHtml(value)), `${scenario.id}: missing diagnostics ${value}`);
    }
    assert.match(diag, /data-action="page" data-value="recovery"/);
  }
  assert.match(V.status(M.initialState('stale')), /Last snapshot · 2 min 14 s old/);
  assert.match(V.diagnostics(M.initialState('observe')), /does not automatically re-enable Windows Time/);
});
