import test from 'node:test';
import assert from 'node:assert/strict';
import {SCENES, STATES, INITIAL_SOURCES, extraSource, validateConfiguration} from '../src/model.js';
import {makeScenePresentation, sourceRoleLabel, refreshFeedback} from '../src/presentation.js';

const scene = id => {
  const value = SCENES.find(item => item.id === id);
  assert.ok(value, 'Scene must be accessible: ' + id);
  return value;
};
const currentHealth = {connection: 'connected', service: 'running', basis: 'Current fixture snapshot'};

test('a missing connection and a confirmed service stop have different evidence', () => {
  const disconnected = makeScenePresentation(scene('status-stale'));
  const stopped = makeScenePresentation(scene('status-service-failed'));
  assert.equal(disconnected.health.connection, 'unavailable');
  assert.equal(disconnected.health.service, 'unknown');
  assert.equal(stopped.health.connection, 'unavailable');
  assert.equal(stopped.health.service, 'confirmed-stopped');
  assert.match(disconnected.health.basis, /does not establish/);
  assert.match(stopped.health.basis, /confirmed/);
  assert.match(disconnected.evidence.lastQualifiedEstimateAge, /historical/);
  assert.match(stopped.clockOwner, /Unknown/);
  assert.equal(STATES.stale.title, 'Status unavailable');
  assert.equal(makeScenePresentation(scene('status-stopping')).health.service, 'running');
});

test('replies, qualified estimates, acknowledged corrections and snapshots retain separate ages', () => {
  const result = makeScenePresentation(scene('status'));
  assert.equal(result.evidence.lastQualifiedEstimateAge, '18 seconds ago');
  assert.equal(result.evidence.lastAppliedCorrectionAge, '42 seconds ago');
  assert.equal(result.evidence.snapshotAge, '1 second ago');
  const rejected = result.sources.find(source => source.id === 'delta');
  assert.equal(rejected.lastValidReplyAge, '8 seconds ago');
  assert.equal(rejected.lastAcceptedMeasurementAge, '4 minutes ago');
  assert.equal(rejected.packetAuthentication, 'Authenticated');
  assert.equal(sourceRoleLabel(rejected.role), 'Not used');
  // A recent authenticated but rejected reply cannot become an estimate or correction.
  assert.notEqual(rejected.lastValidReplyAge, result.evidence.lastQualifiedEstimateAge);
  assert.notEqual(rejected.lastValidReplyAge, result.evidence.lastAppliedCorrectionAge);
});

test('generation changes do not turn old measurements into recent evidence', () => {
  const result = makeScenePresentation(scene('status-reacquiring'));
  assert.match(result.evidence.lastQualifiedEstimateAge, /^Unknown/);
  assert.match(result.evidence.lastAppliedCorrectionAge, /^Unknown/);
  assert.equal(result.evidence.snapshotAge, '1 second ago');
  assert.ok(result.sources.every(source => source.role === 'Waiting'));
  assert.ok(result.sources.every(source => source.lastValidReplyAge.startsWith('Unknown')));
});

test('authenticated Tracking retains the unauthenticated date basis in both recovery modes', () => {
  for (const [id, helpers] of [['status-after-one-helper', 1], ['status-after-two-helpers', 2]]) {
    const fixture = scene(id);
    const result = makeScenePresentation(fixture);
    assert.equal(fixture.state, 'tracking');
    assert.equal(result.recoveryRecord.helpers, helpers);
    assert.equal(result.recoveryRecord.originalHintAuthenticated, false);
    assert.equal(result.recoveryRecord.completed, true);
    assert.match(result.recoveryRecord.outcome, /original date hint remains unauthenticated/);
    assert.ok(result.sources.every(source => source.packetAuthentication === 'Authenticated'));
    assert.ok(result.sources.every(source => source.certificateTimeBasis === result.recoveryRecord.certificateTimeBasis));
  }
  const ordinary = makeScenePresentation(scene('status'));
  assert.equal(ordinary.recoveryRecord, null);
  // Identical NTS authentication can have different certificate-time provenance.
  assert.equal(ordinary.sources[0].packetAuthentication, ordinary.sources[1].packetAuthentication);
  assert.notEqual(ordinary.sources[0].certificateTimeBasis, ordinary.sources[1].certificateTimeBasis);
});

test('recovery in progress cannot be presented as completed authenticated acquisition', () => {
  const current = makeScenePresentation(scene('status-recovery'));
  assert.equal(current.recoveryRecord.completed, false);
  assert.equal(current.recoveryRecord.originalHintAuthenticated, false);
  assert.match(current.recoveryRecord.outcome, /in progress/);
  assert.ok(current.sources.every(source => source.packetAuthentication === 'Not established'));
  assert.equal(current.evidence.lastQualifiedEstimateAge, 'Not established');
  assert.equal(current.evidence.lastAppliedCorrectionAge, 'Not applied');
});

test('observe-only and clock ownership are independent fixture facts', () => {
  const fixture = scene('status-observe');
  assert.equal(fixture.state, 'tracking');
  const observation = makeScenePresentation(fixture);
  const active = makeScenePresentation(scene('status'));
  assert.equal(observation.observe, true);
  assert.equal(active.observe, false);
  assert.equal(observation.clockOwner, 'NevroTick');
  assert.equal(active.clockOwner, 'NevroTick');
  assert.equal(makeScenePresentation({...fixture, clockOwner: 'Windows Time'}).clockOwner, 'Windows Time');
  assert.equal(makeScenePresentation(scene('status-blocked')).clockOwner, 'Windows Time');
});

test('scene results are independent editable copies, including provenance and evidence', () => {
  const before = structuredClone(INITIAL_SOURCES);
  const first = makeScenePresentation(scene('status-after-two-helpers'));
  first.sources[0].host = 'changed.example';
  first.recoveryRecord.originalHintAuthenticated = true;
  first.evidence.lastQualifiedEstimateAge = 'Just now';
  first.health.service = 'confirmed-stopped';
  const second = makeScenePresentation(scene('status-after-two-helpers'));
  assert.equal(second.sources[0].host, 'alpha.example');
  assert.equal(second.recoveryRecord.originalHintAuthenticated, false);
  assert.equal(second.evidence.lastQualifiedEstimateAge, '18 seconds ago');
  assert.equal(second.health.service, 'running');
  assert.deepEqual(INITIAL_SOURCES, before);
});

test('refresh feedback coalesces checking and distinguishes waiting, limits and correction', () => {
  const cases = [
    ['tracking', 'idle', 'checking'],
    ['tracking', 'checking', 'already-checking'],
    ['tracking', 'already-checking', 'already-checking'],
    ['acquiring', 'idle', 'waiting'],
    ['tracking', 'waiting', 'waiting'],
    ['tracking', 'rate-limited', 'rate-limited'],
    ['tracking', 'correction-in-progress', 'correction-in-progress'],
    ['blocked', 'checking', 'blocked'],
    ['stopping', 'idle', 'blocked'],
    ['offline', 'idle', 'unavailable'],
  ];
  for (const [status, previousOutcome, expected] of cases) {
    const input = {status, previousOutcome, health: {...currentHealth}};
    const before = structuredClone(input);
    const result = refreshFeedback(input);
    assert.equal(result.outcome, expected, status + '/' + previousOutcome);
    assert.ok(result.message);
    assert.deepEqual(Object.keys(result).sort(), ['message', 'outcome']);
    assert.deepEqual(input, before);
  }
});

test('unavailable status outranks an old request outcome and never implies a confirmed stop', () => {
  const disconnected = makeScenePresentation(scene('status-stale'));
  const input = {status: 'tracking', previousOutcome: 'checking', health: disconnected.health};
  const result = refreshFeedback(input);
  assert.equal(result.outcome, 'unavailable');
  assert.match(result.message, /unknown/);
  const confirmed = refreshFeedback({status: 'service-failed', health: makeScenePresentation(scene('status-service-failed')).health});
  assert.equal(confirmed.outcome, 'unavailable');
  assert.match(confirmed.message, /confirmed stopped/);
  assert.equal(refreshFeedback({status: 'tracking'}).outcome, 'unavailable');
});

test('Sync now cannot acknowledge a correction or alter current evidence', () => {
  const presentation = makeScenePresentation(scene('correction-approval'));
  const before = structuredClone(presentation);
  const result = refreshFeedback({status: 'correction', previousOutcome: 'idle', health: presentation.health});
  assert.equal(result.outcome, 'checking');
  assert.match(result.message, /separate administrator approval/);
  assert.deepEqual(presentation, before);
  assert.equal(presentation.evidence.lastAppliedCorrectionAge, 'Not applied');
});

test('the two-versus-two conflict meets the example minimum but cannot claim a majority', () => {
  const result = makeScenePresentation(scene('status-conflict'));
  assert.equal(result.sources.filter(source => source.offset.startsWith('−')).length, 2);
  assert.equal(result.sources.filter(source => source.offset.startsWith('+')).length, 2);
  assert.ok(result.sources.every(source => source.role === 'Rejected'));
  assert.match(STATES.conflict.agreement, /no majority/);
  assert.match(STATES.conflict.description, /minimum of two is met/);
  // Draft cardinality checks are not a replacement for runtime interval selection.
  assert.deepEqual(validateConfiguration(result.sources.slice(0, 2), 'nts'), []);
});

test('existing internal roles and new source uncertainty remain compatible with the UI', () => {
  for (const [role, label] of Object.entries({Principal: 'Main', Supporting: 'Supporting', Rejected: 'Not used', Pending: 'Draft', Waiting: 'Waiting'})) {
    assert.equal(sourceRoleLabel(role), label);
  }
  const pending = extraSource();
  assert.equal(pending.role, 'Pending');
  assert.equal(pending.packetAuthentication, 'Not established');
  assert.equal(pending.timeEndpoint, 'Not established');
  const ntp = makeScenePresentation(scene('status-ntp'));
  assert.ok(ntp.sources.every(source => source.packetAuthentication === 'Unauthenticated'));
  assert.ok(ntp.sources.every(source => source.certificateTimeBasis === 'Not applicable — plain NTP'));
});

test('new scenes are reachable while refresh and observe variants preserve Tracking', () => {
  const ids = [
    'status-initializing', 'status-reacquiring', 'status-stopping', 'status-service-failed',
    'status-gradual-correction', 'status-rate-limited', 'status-after-one-helper', 'status-after-two-helpers',
  ];
  for (const id of ids) assert.ok(STATES[scene(id).state], id);
  assert.equal(new Set(SCENES.map(item => item.id)).size, SCENES.length);
  for (const id of ['status-gradual-correction', 'status-rate-limited']) assert.equal(scene(id).state, 'tracking');
  assert.equal(makeScenePresentation(scene('status-gradual-correction')).refreshOutcome, 'correction-in-progress');
  assert.equal(makeScenePresentation(scene('status-rate-limited')).refreshOutcome, 'rate-limited');
});
