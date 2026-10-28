import {INITIAL_SOURCES, SCENES} from './model.js';

// Illustrative service-published fixtures, not a second runtime state machine.
// In particular, neither packet authentication nor an open status connection
// establishes clock ownership, certificate-time provenance or a correction.
const ROLE_LABELS = {
  Principal: 'Main',
  Supporting: 'Supporting',
  Rejected: 'Not used',
  Pending: 'Draft',
  Waiting: 'Waiting',
};

export function sourceRoleLabel(role) {
  return ROLE_LABELS[role] || 'Unknown';
}

function awaitingMeasurements(sources, reason, changedGeneration = false) {
  return sources.map(source => ({
    ...source,
    role: 'Waiting',
    age: '—',
    lastValidReplyAge: changedGeneration ? 'Unknown after a system change' : 'No current valid reply',
    lastAcceptedMeasurementAge: changedGeneration ? 'Unknown after a system change' : 'Not established',
    packetAuthentication: 'Not established',
    certificateTimeBasis: 'Not established',
    timeEndpoint: 'Not established',
    offset: '—', delay: '—', uncertainty: '—', reason,
  }));
}

export function makeScenePresentation(scene = {}) {
  const fixture = typeof scene === 'string' ? SCENES.find(item => item.id === scene) || {} : scene;
  const status = fixture.state || 'tracking';
  let sources = fixture.empty ? [] : INITIAL_SOURCES.map(source => ({...source}));
  let evidence = {
    lastQualifiedEstimateAge: '18 seconds ago',
    lastAppliedCorrectionAge: '42 seconds ago',
    snapshotAge: '1 second ago',
  };
  let health = {
    connection: 'connected', service: 'running',
    basis: 'The current service snapshot reports that the service is running.',
  };
  let clockOwner = 'NevroTick';
  let recoveryRecord = null;
  let refreshOutcome = 'idle';

  if (['initializing', 'acquiring', 'empty', 'recovery', 'date-range'].includes(status)) {
    evidence = {...evidence, lastQualifiedEstimateAge: 'Not established', lastAppliedCorrectionAge: 'Not applied'};
    sources = awaitingMeasurements(sources, 'Waiting for current measurements that satisfy the active policy.');
    refreshOutcome = status === 'empty' || status === 'date-range' ? 'blocked' : 'waiting';
  }
  if (status === 'initializing') {
    clockOwner = 'Not established';
    health.basis = 'The service is running readiness checks; clock ownership is not established yet.';
  }
  if (status === 'empty') clockOwner = 'Not established';
  if (status === 'reacquiring') {
    evidence = {
      lastQualifiedEstimateAge: 'Unknown after a system change',
      lastAppliedCorrectionAge: 'Unknown across the previous generation',
      snapshotAge: '1 second ago',
    };
    sources = awaitingMeasurements(sources, 'Previous-generation exchanges no longer count. Fresh measurements are required.', true);
    refreshOutcome = 'waiting';
  }
  if (status === 'holdover' || status === 'offline' || status === 'expired') {
    const expired = status === 'expired';
    evidence = {
      lastQualifiedEstimateAge: expired ? '2 hours ago' : '12 minutes ago',
      lastAppliedCorrectionAge: expired ? '2 hours 3 minutes ago' : '13 minutes ago',
      snapshotAge: '1 second ago',
    };
    sources = sources.map(source => ({
      ...source, role: 'Waiting', age: expired ? '2 h' : '12 min',
      lastValidReplyAge: expired ? '2 hours ago' : '12 minutes ago',
      lastAcceptedMeasurementAge: source.id === 'delta' ? (expired ? '2 hours 4 minutes ago' : '16 minutes ago') : evidence.lastQualifiedEstimateAge,
      reason: expired ? 'The previous estimate is no longer usable. Fresh agreement is required.' : 'No fresh measurement is available. Only the previously qualified model remains usable.',
    }));
  }
  if (status === 'conflict') {
    evidence = {...evidence, lastQualifiedEstimateAge: '4 minutes ago', lastAppliedCorrectionAge: '5 minutes ago'};
    // A fixed 2-versus-2 example. This is not a replacement for Chrony selection.
    sources = sources.map((source, index) => ({
      ...source, role: 'Rejected', age: '5 s', lastValidReplyAge: '5 seconds ago',
      lastAcceptedMeasurementAge: '4 minutes ago', offset: index < 2 ? '−18 ms' : '+42 ms',
      uncertainty: '±5 ms',
      reason: 'Two eligible sources agree with this interval and two disagree. No majority exists; a minimum of two alone is insufficient.',
    }));
    refreshOutcome = 'waiting';
  }
  if (status === 'stale' || status === 'service-failed') {
    const confirmed = status === 'service-failed';
    evidence = {
      lastQualifiedEstimateAge: '6 min 18 s ago · historical',
      lastAppliedCorrectionAge: '6 min 42 s ago · historical',
      snapshotAge: '6 minutes ago',
    };
    health = {
      connection: 'unavailable', service: confirmed ? 'confirmed-stopped' : 'unknown',
      basis: confirmed
        ? 'Windows service status confirmed that the time service stopped after recovery attempts.'
        : 'Only the status connection is unavailable. This does not establish whether the service stopped.',
    };
    clockOwner = 'Unknown — ownership must be checked';
    sources = sources.map(source => ({
      ...source, age: 'Historical', lastValidReplyAge: 'Over 6 minutes ago · historical',
      lastAcceptedMeasurementAge: source.id === 'delta' ? '10 minutes ago · historical' : 'Over 6 minutes ago · historical',
      reason: 'Historical source information from the last received snapshot. Current source status is unknown.',
    }));
    refreshOutcome = 'unavailable';
  }
  if (status === 'blocked') {
    clockOwner = 'Windows Time';
    refreshOutcome = 'blocked';
  }
  if (status === 'stopping') {
    health.basis = 'The current snapshot reports cleanup pending; a completed stop has not been confirmed.';
    refreshOutcome = 'blocked';
  }
  if (status === 'correction') evidence.lastAppliedCorrectionAge = 'Not applied';

  if (fixture.profile === 'ntp') {
    sources = sources.map(source => ({
      ...source, protocol: 'ntp', port: '123',
      packetAuthentication: source.packetAuthentication === 'Not established' ? 'Not established' : 'Unauthenticated',
      certificateTimeBasis: 'Not applicable — plain NTP',
      timeEndpoint: source.timeEndpoint === 'Not established' ? 'Not established' : source.host + ':123 · UDP',
    }));
  }

  const helpers = fixture.recoveryHelpers;
  if (helpers === 1 || helpers === 2) {
    const basis = 'Plain-NTP date hint · ' + helpers + (helpers === 1 ? ' helper' : ' helpers');
    recoveryRecord = {
      helpers, certificateTimeBasis: basis, originalHintAuthenticated: false, completed: true,
      outcome: 'Authenticated acquisition completed; the original date hint remains unauthenticated.',
      age: '4 minutes ago',
    };
    sources = sources.map(source => ({...source, certificateTimeBasis: basis}));
  }
  if (status === 'recovery') {
    sources = sources.map(source => ({...source, certificateTimeBasis: 'Plain-NTP date hint · 2 helpers'}));
    recoveryRecord = {
      helpers: 2, certificateTimeBasis: 'Plain-NTP date hint · 2 helpers', originalHintAuthenticated: false, completed: false,
      outcome: 'NTS verification is in progress; the hints have not changed Windows time.', age: 'Just received',
    };
  }
  if (fixture.refreshOutcome) refreshOutcome = fixture.refreshOutcome;
  if (fixture.id === 'status-gradual-correction') evidence.lastAppliedCorrectionAge = '2 seconds ago';
  // Observe-only and the known owner are independent fixture facts. In the
  // observe example NevroTick retains ownership; Windows Time is not re-enabled.
  if (fixture.clockOwner) clockOwner = fixture.clockOwner;
  return {sources, evidence, health, clockOwner, recoveryRecord, refreshOutcome, observe: fixture.observe === true};
}

export function refreshFeedback({status, previousOutcome = 'idle', health} = {}) {
  if (health?.service === 'confirmed-stopped') return {
    outcome: 'unavailable', message: 'The time service is confirmed stopped. Repair the complete installed product.',
  };
  if (health?.connection !== 'connected' || health?.service !== 'running') return {
    outcome: 'unavailable', message: 'Status is unavailable. No fresh check was confirmed; service health may be unknown.',
  };
  if (status === 'offline') return {
    outcome: 'unavailable', message: 'Network unavailable. A fresh check cannot complete until the connection returns.',
  };
  const blockedMessages = {
    blocked: 'Clock control is blocked. A fresh-check request cannot grant authority to change time.',
    'date-range': 'Certificate verification is blocked by the system date. Review the recovery options.',
    stopping: 'The service is finishing background work. A new check is not available yet.',
    empty: 'No time sources are configured. Add and apply sources before requesting a check.',
  };
  if (blockedMessages[status]) return {outcome: 'blocked', message: blockedMessages[status]};
  if (previousOutcome === 'rate-limited' || status === 'rate-limited') return {
    outcome: 'rate-limited', message: 'Try again after the current limit. Scheduled checks continue.',
  };
  if (previousOutcome === 'correction-in-progress' || status === 'adjusting' || status === 'gradual-correction') return {
    outcome: 'correction-in-progress', message: 'Correction in progress. A fresh-check request does not restart or extend it.',
  };
  if (previousOutcome === 'checking' || previousOutcome === 'already-checking') return {
    outcome: 'already-checking', message: 'Already checking. The current request is still in progress.',
  };
  if (['initializing', 'acquiring', 'reacquiring', 'conflict', 'recovery', 'waiting'].includes(status) || previousOutcome === 'waiting') return {
    outcome: 'waiting', message: 'Waiting for enough agreeing sources. The configured minimum and majority requirement both apply.',
  };
  if (['tracking', 'holdover', 'expired', 'observe', 'correction'].includes(status)) return {
    outcome: 'checking', message: status === 'correction'
      ? 'Checking the active sources. Any exceptional correction still requires separate administrator approval.'
      : 'Checking time sources using the active settings.',
  };
  return {outcome: 'blocked', message: 'A fresh check is not available for the current reported state. Open Diagnostics.'};
}
