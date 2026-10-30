# Data, state and effect contracts

Read for changed wire records, configuration, journals, snapshots, derived state or ownership. Architecture sections 5–6 and 12 control; [implement-rust-contracts](../../implementation/implement-rust-contracts/SKILL.md) owns implementation and [evolve-data-contracts](../../implementation/evolve-data-contracts/SKILL.md) owns compatibility decisions.

Each boundary: producer/consumer, canonical writer, representation/version, exact units, required/present/null/default semantics, invariants, maximum work/storage, failure result. Distinguish admitted wire data, private domain values, current service authority, stored form, redacted UI output. Equal field shapes need not mean equal facts.

Use selected chain: bounded bytes → strict inert Serde record → mandatory shared garde/domain validation inside fallible private construction → current trusted guard → serialized effect → actual result. No prior authority-bearing JSON normalization through generic value/map. Explicit fixtures: exact object shape, duplicate/unknown keys, integer spelling/range, trailing input, field/aggregate limits, diagnostic bounds; derive or `deny_unknown_fields` is incomplete. Ordinary calls preserving one immutable checked type need no repeated parsing.

All function/module interfaces need input requirements, postconditions, error and
effect/lifecycle guarantees. Map each to types, construction, checked operations
or current-state guards using
[contract enforcement](../../implementation/implement-rust-contracts/references/contract-enforcement.md).
Validation rules have one maintained owner; direct callers, builders, mutation
and migration cannot bypass that path. Document semantic obligations not captured
by signatures, and give QA independent invalid/bypass and valid controls.

Define signed/unsigned time, offset/frequency units, UTC versus monotonic/active time, source identity/independence, generation, evidence provenance. Wall time moves and is product repair target; untrusted wall timestamps cannot supply freshness/lease budgets. Check transformation guarantees/serialized consumer parity separately.

Name owners for stale/duplicate-request rejection, policy revision comparison, one-time approval consumption, cumulative correction accounting, ambiguous-effect reconciliation. Timeless `Validated<T>` cannot encode revocation/current ownership/remaining correction budget. Corrupt optional drift may trigger reacquisition; corrupt required policy/journals must not become permissive defaults. Transitions need compatible reads, atomic replacement, rollback/state-version policy, clear mixed-version behavior.

Clock effects follow [architecture §10.2.1](../../../../docs/windows-time-architecture.md#1021-durable-authority-reservation-and-uncertain-outcomes):
durable identity/authority reservation before the call, durable known outcome before
success acknowledgement, explicit charged unknowns and no blind duplicate step.
Separate actuator mode from rate, model prediction from fresh votes, and NTS key/full
authentication age from packet freshness. A protected but semantically wrong time
floor needs the narrow supersession contract, without clearing unrelated budgets.
