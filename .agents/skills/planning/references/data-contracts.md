# Data, state and effect contracts

Read for changed wire records, configuration, journals, snapshots, derived state or ownership. Architecture sections 5–6 and 12 control; [implement-rust-contracts](../../implementation/implement-rust-contracts/SKILL.md) owns implementation and [evolve-data-contracts](../../implementation/evolve-data-contracts/SKILL.md) owns compatibility decisions.

Each boundary: producer/consumer, canonical writer, representation/version, exact units, required/present/null/default semantics, invariants, maximum work/storage, failure result. Distinguish admitted wire data, private domain values, current service authority, stored form, redacted UI output. Equal field shapes need not mean equal facts.

Use selected chain: bounded bytes → strict inert Serde record → fallible private domain construction → current trusted guard → serialized effect → actual result. No prior authority-bearing JSON normalization through generic value/map. Explicit fixtures: exact object shape, duplicate/unknown keys, integer spelling/range, trailing input, field/aggregate limits, diagnostic bounds; derive or `deny_unknown_fields` is incomplete. Ordinary calls preserving one immutable checked type need no repeated parsing.

Define signed/unsigned time, offset/frequency units, UTC versus monotonic/active time, source identity/independence, generation, evidence provenance. Wall time moves and is product repair target; untrusted wall timestamps cannot supply freshness/lease budgets. Check transformation guarantees/serialized consumer parity separately.

Name owners for stale/duplicate-request rejection, policy revision comparison, one-time approval consumption, cumulative correction accounting, ambiguous-effect reconciliation. Timeless `Validated<T>` cannot encode revocation/current ownership/remaining correction budget. Corrupt optional drift may trigger reacquisition; corrupt required policy/journals must not become permissive defaults. Transitions need compatible reads, atomic replacement, rollback/state-version policy, clear mixed-version behavior.
