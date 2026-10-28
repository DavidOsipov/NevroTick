# Rust implementation contract

Use for data contracts and compatibility transitions; ordinary Rust/UI edits only
when a data-contract concern applies. Repository instructions and
[canonical architecture](../../../docs/windows-time-architecture.md), especially
sections 5–6 and 18, control. Skills add no approval round or authority.

Match requested mode: implementation, review, planning or tests-only. Review
without edits; planning ends at requested plan; tests-only grants no unrelated
production repair. Reuse current plan/evidence while inputs stay unchanged.
Record material missing decisions; continue independent work.

## One maintained owner for each guarantee

Trace bytes → inert wire record → domain construction → current authority/state
guard → serialized effect → actual acknowledgement. `ipc-contract` owns versioned
DTOs; `time-domain` owns private pure invariants; receiving service owns current
permission. Responsibilities may initially be modules, not separate crates.
Keep JSON, UI, OS calls and effects out of `time-core`.

Selected prototype: bounded direct `serde`/`serde_json` decoding, fallible Rust
constructors. No validation crate automatically replaces Zod. Garde remains held;
`serde_path_to_error` optional; schema generation not adopted. Follow architecture
section 18.5 before admitting any candidate.

Boundaries change trust, representation, persistence or independently versioned
consumption. Typed calls between immutable validated values in one context need no
repeated parsing. Meaningful transformations still need invariants, checked outputs
and failure behavior. Policy, generation, ownership, time or destination changes
can invalidate contextual checks without type changes.

## Preserve meaning and failure atomicity

Specify units, sign, ranges, identity, source/provenance, presence/null semantics,
version, normalization, output guarantees and consumers. Keep UTC, monotonic,
active-awake time, NTP timestamps, phase and frequency distinct. Public positive
offset means Windows is behind reference time and needs to advance.

Consumers use checked output, never original DTO after validating another copy.
No unrestricted deserialization of invariant-bearing types or bypasses through
public fields, unchecked `From`, mutation or permissive defaults. Only real
protocol verification constructs authenticated evidence; only broker's actual
guard grants current authority.

Validate mandatory persisted policy/journals after protected bounded reads.
Invalid updates preserve prior valid revision; invalid mandatory startup state
blocks protected effects. Discard optional statistical caches only under documented
recovery contract. Failure never manufactures synchronized status, resets authority
budgets or silently enables plain NTP.

Pure validation is separate from authorization and sink safety. At the effect
owner, serialize guard and effect using trusted identity, current revision,
generation, freshness, replay state, ownership and cumulative budgets. Recheck after
asynchronous work. Ambiguous effects require reconciliation before retry.

## Evidence and completion

Select checks through [testing contract](../testing/CONTRACT.md) and relevant QA
owner; add no second testing policy. Changed enforcement needs independent invalid
and valid neighboring controls through actual adapter/consumer. Confirm executed
command admits affected files. Green counts, installed tools/configuration alone
prove no behavior. Never relax contracts/checks solely for green results.

Use [evolution](evolve-data-contracts/SKILL.md) for compatibility decisions before
dependent edits, then [contract implementation](implement-rust-contracts/SKILL.md)
for bounded owner changes. [Planning](../planning/plan-code-change/SKILL.md),
[security](../security/CONTRACT.md), and
[dependency updates](../environment/dependency-updates/SKILL.md) retain their scopes.

Report requested result, owner, checked-data path, failure/effect behavior,
affected consumers, executed checks and limits. WSL/Linux evidence proves no
Windows service, IPC, UI, installer or real clock behavior. Planned CHECK entries,
research, builds and qualified releases remain separate claims. Add no dependency,
migration, publication or release merely to satisfy workflow.
