# Rust implementation contract

Use for every Rust implementation or review, including internal function/module
contracts, UI adapters and compatibility transitions. Repository instructions and
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

Treat user-entered product data and external programmatic data as potentially
malicious at admission, including UI typing/paste, direct IPC/automation, peers,
stored/imported records and external/native results where applicable. Local or
authenticated origin, administrator identity, UI validation and earlier acceptance
do not establish payload safety or current authority. Bound work before expensive
processing, preserve raw representation for strict checks, and reject without
partial protected effects. Test handlers directly as well as through UI paths.

Selected baseline: bounded direct `serde`/`serde_json` decoding and shared pure
`garde` validation invoked obligatorily inside private fallible domain
construction. The adopted garde candidate is 0.23.0, defaults disabled and only
`derive` initially; the actual locked feature/license/target graph still needs
admission. `serde_path_to_error` remains optional; schema generation is not adopted.
Read [contract enforcement](implement-rust-contracts/references/contract-enforcement.md)
for construction, field coverage and function/module obligations. A derive or
borrowed `validate()` call alone does not make later consumption safe.

Every function and module has explicit input requirements, output guarantees,
failure behavior and permitted effects/lifecycle transitions. Express guarantees
in signatures, private types and ownership where possible; document semantic
requirements that types cannot express beside the API/module. Bind each such
requirement to its actual enforcement and detecting check. Simple typed helpers
need no duplicated schema, boilerplate report or repeated runtime validation.

Boundaries change trust, representation, persistence or independently versioned
consumption. Typed calls between immutable validated values in one context need no
repeated parsing. Meaningful transformations still need invariants, checked outputs
and failure behavior. Policy, generation, ownership, time or destination changes
can invalidate contextual checks without type changes. Every transformation,
mutation, migration and FFI result must preserve or re-establish its output
invariants before publication. Required rejection must work in release builds;
`debug_assert!` or a comment is not its enforcement.

## Preserve meaning and failure atomicity

Changes to error propagation, diagnostic projection, retry/recovery, fallible
cleanup or panic boundaries must follow
[error handling](implement-rust/references/error-handling.md). Keep reusable
native capture in `windows-platform`, shared safe-field policy in `diagnostics`,
and domain failures/current recovery decisions at their owners. Error formatting
is not sanitization; uncertain effects require reconciliation before retry.

Specify units, sign, ranges, identity, source/provenance, presence/null semantics,
version, normalization, output guarantees and consumers. Keep UTC, monotonic,
active-awake time, NTP timestamps, phase and frequency distinct. Public positive
offset means Windows is behind reference time and needs to advance.

Consumers use checked output, never original DTO after validating another copy.
No unrestricted deserialization of invariant-bearing types or bypasses through
public fields, unchecked `From`, builders, mutation, interior mutability or
permissive defaults. Do not enable `garde(allow_unvalidated)`; every `skip` needs
its exact invariant/enforcement owner and independent review. Maintain each rule
once in the shared construction/validation path, not parallel UI annotations and
divergent constructor checks. Only real protocol verification constructs
authenticated evidence; only broker's actual
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

Make resource contracts explicit: name the allocation/handle/task owner, bounded
retained quantity, matching release operation and completion event. Use owning
wrappers for native resources; preserve borrowing and callback lifetimes across
FFI. Partial construction, cancellation and shutdown must retain custody until
the actual native operation completes. Safe Rust and input validation do not
exclude reference cycles, retained caches or leaked OS resources. Route changed
native ownership to [memory diagnostics](../testing/references/windows-memory.md)
and the applicable lifecycle/retention checks before claiming it is verified.

## Evidence and completion

Select checks through [testing contract](../testing/CONTRACT.md) and relevant QA
owner; add no second testing policy. Changed enforcement needs independent invalid
and valid neighboring controls through actual adapter/consumer. Confirm executed
command admits affected files. Green counts, installed tools/configuration alone
prove no behavior. Changed contracts need checks for real direct callers,
construction/consumption bypasses, transformed outputs and no prohibited effect
on rejection. Use real external compile-fail consumers for privacy/API guarantees
and a compiling valid neighbor; syntax/import failure proves no protection.
Never relax contracts/checks solely for green results.

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
