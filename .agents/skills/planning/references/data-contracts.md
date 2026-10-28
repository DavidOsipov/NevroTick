# Data, state and effect contracts

Read for changed wire records, configuration, journals, snapshots, derived state or ownership. Architecture sections 5–6 and 12 control; [implement-rust-contracts](../../implementation/implement-rust-contracts/SKILL.md) owns implementation and [evolve-data-contracts](../../implementation/evolve-data-contracts/SKILL.md) owns compatibility decisions.

For each boundary name producer/consumer, canonical writer, representation/version, exact units, required/present/null/default semantics, invariants, maximum work/storage and failure result. Distinguish admitted wire data, private domain values, current service authority, stored form and redacted UI output. Equal field shapes need not mean equal facts.

Use the selected chain: bounded bytes → strict inert Serde record → fallible private domain construction → current trusted guard → serialized effect → actual result. Do not first normalize authority-bearing JSON through a generic value/map. Exact object shape, duplicate/unknown keys, integer spelling/range, trailing input, field/aggregate limits and diagnostic bounds require explicit fixtures; a derive or `deny_unknown_fields` is not the whole contract. Ordinary calls preserving one immutable checked type do not need repeated parsing.

Define signed versus unsigned time, offset/frequency units, UTC versus monotonic/active time, source identity/independence, generation and evidence provenance. Wall time can move and is the product's repair target; freshness and lease budgets cannot be derived from untrusted wall timestamps. Transformation guarantees and serialized consumer parity need their own checks.

Name stale/duplicate-request rejection, policy revision comparison, one-time approval consumption, cumulative correction accounting and ambiguous-effect reconciliation at their owners. A timeless `Validated<T>` cannot encode revocation, current ownership or remaining correction budget. Corrupt optional drift may trigger reacquisition; corrupt required policy/journals must not become permissive defaults. Transitions need compatible reads, atomic replacement, rollback/state-version policy and clear mixed-version behavior.
