# Rust implementation contract

Read this for data contracts and compatibility transitions. Ordinary Rust or UI edits
enter it only when a data-contract concern applies. Repository instructions and
[canonical architecture](../../../docs/windows-time-architecture.md), especially
sections 5–6 and 18, control; these skills add no approval round or authority.

Choose implementation, review, planning or tests-only mode from the request. Review
without editing; planning stops at the requested plan; tests-only work does not
authorize an unrelated production repair. Reuse the current plan and existing
evidence while its inputs remain unchanged. Record material missing decisions,
then continue independent work.

## One maintained owner for each guarantee

Trace bytes → inert wire record → domain construction → current authority/state
guard → serialized effect → actual acknowledgement. `ipc-contract` owns versioned
DTOs, `time-domain` owns private pure invariants, and the receiving service owns
current permission. These are architectural responsibilities and may initially be
modules, not separate crates. Keep JSON, UI, OS calls and effects out of `time-core`.

The selected prototype uses bounded direct `serde`/`serde_json` decoding and
fallible Rust constructors. No validation crate is an automatic Zod equivalent.
Garde remains held; `serde_path_to_error` is optional; schema generation is not
adopted. Follow architecture section 18.5 before admitting any candidate.

A boundary changes trust, representation, persistence or independently versioned
consumption. Typed calls between immutable validated values in one context do not
need repeated parsing. Meaningful transformations still need their own invariants,
checked outputs and failure behavior. Changes to policy, generation, ownership,
time or destination can invalidate contextual checks without changing a type.

## Preserve meaning and failure atomicity

Specify units, sign, ranges, identity, source/provenance, presence/null semantics,
version, normalization, output guarantees and consumers. Keep UTC, monotonic,
active-awake time, NTP timestamps, phase and frequency distinct. Public positive
offset means Windows is behind reference time and needs to advance.

Consumers use the checked output, never the original DTO after validating another
copy. Do not deserialize unrestricted invariant-bearing types or expose bypasses
through public fields, unchecked `From`, mutation or permissive defaults. Only the
real protocol-verification path constructs authenticated evidence; only the
broker's actual guard grants current authority.

Validate mandatory persisted policy/journals after protected bounded reads.
Invalid updates preserve the prior valid revision; invalid mandatory startup state
blocks protected effects. Optional statistical caches may be discarded under their
documented recovery contract. Failure never manufactures synchronized status,
resets authority budgets or silently enables plain NTP.

Pure validation is separate from authorization and sink safety. At the effect
owner, serialize guard and effect using trusted identity, current revision,
generation, freshness, replay state, ownership and cumulative budgets. Recheck after
asynchronous work. Ambiguous effects require reconciliation before retry.

## Evidence and completion

Use [testing contract](../testing/CONTRACT.md) and the relevant QA owner to select
checks, rather than reproducing a second testing policy. For changed enforcement,
use an independent invalid control and a valid neighboring control through the
actual adapter/consumer. Check the executed command admits the affected files;
green counts, installed tools and configuration alone establish no behavior.
Never relax a contract/check solely to obtain green results.

Use [evolution](evolve-data-contracts/SKILL.md) for compatibility decisions before
dependent edits, then [contract implementation](implement-rust-contracts/SKILL.md)
for bounded owner changes. [Planning](../planning/plan-code-change/SKILL.md),
[security](../security/CONTRACT.md), and
[dependency updates](../environment/dependency-updates/SKILL.md) retain their scopes.

Finish with the requested result, owner and checked-data path, failure/effect
behavior, affected consumers, executed checks and limitations. WSL/Linux evidence
does not prove Windows service, IPC, UI, installer or real clock behavior. A planned
CHECK entry, research finding, successful build and qualified release are separate
claims. Do not introduce a dependency, migration, publication or release merely to
satisfy this workflow.
