---
name: "qa-contract-tests"
description: "Verify NevroTick function/module preconditions, postconditions, mandatory validation, strict wire admission and consumer compatibility through actual callers."
---

# Test function, module and wire contracts

Read [the testing contract](../CONTRACT.md).

Read applicable implementation contract/[strict contract cases](../references/numerical-protocol.md). Trace raw bytes → bounded map-only DTO → private fallible domain construction → current guard → effect/acknowledgement. Identify each owner/next-boundary consumed representation.

Apply [contract enforcement](../../implementation/implement-rust-contracts/references/contract-enforcement.md)
to every changed function/module: identify input requirements, output guarantees,
failure/effect/lifecycle behavior and enforcing type or operation. Exercise direct
in-process callers and wire paths. Immutable validated inputs need no reparsing;
transformations still owe checked postconditions.

Verify private constructor invokes shared garde/domain rules; all consumers use
its result. Challenge builders, defaults, `From`/deserialization, mutation,
interior mutability, migration and FFI conversion. Standalone
`validate()` unit test cannot rule out consumer bypass. Reject
`allow_unvalidated`; examine each `skip` against its independently reviewed owner.
Check actual rule/`dive` coverage: metadata-only attributes may compile without
checking invariants; challenge field's semantic rule through its consumer.
Use existing compiler-contract registry for field-omission, visibility and capability
claims. Require intended diagnostic plus compiling legitimate consumer under same
compiler/target/features. Test restricted scope: external rejection at an outer
private module cannot prove inner helper's sibling visibility. For narrowed
production helpers, use production sibling/parent fixture and legal facade;
distinguish `cfg(test)`
adapters and child-module access. Unrelated errors qualify no restriction.

For changed invariant-bearing types, challenge newly exposed construction,
mutation, re-export, `Clone`/`Copy`, `Default`, `From`/`Deserialize`, lifetime and
`Send`/`Sync` capabilities only where actual contract forbids them. Preserve
legitimate immutable copies, adapters and healthy uses; add no blanket trait ban.
Reuse existing harnesses; another framework needs concrete evidence gap.
Compiler rejection proves only attempted misuse.

Test unknown/duplicate/escaped keys, object-versus-array shape, missing/null/invalid values, complete consumption, versions, exact integers, finite computed values, limits, every constructor/migration bypass. Include valid neighboring/legitimate maximum-size controls; reject-all must fail.

Compatibility: freeze supported producer/consumer version/feature matrix. Compare meaning, units, presence/defaults, failure behavior; shape/round trips insufficient. Public Rust visibility/type/trait claims need minimal real external consumer; separate wire-version/Rust-source compatibility.

When identity, canonicalization or comparison/hashing changes, apply
[domain-equivalence cases](../../implementation/implement-rust-contracts/references/contract-enforcement.md#domain-equivalence-and-trait-contracts):
pass equivalent inputs and distinct controls through actual constructors and the
affected hash/ordered collection consumers. Check specified canonical output and
promised idempotence; add no collection or trait solely for this check.
When whole-state equality is rejection oracle, verify every intended field is
observed; future fields must force semantic decision. Preserve required floating
bit and allocation-identity comparisons; incomplete equality can hide corruption.
Retain independent observations of critical effects, not equality alone.

At changed integration seams, select relevant wrong-but-well-typed controls:
neighboring field, wrong owner/generation, stale snapshot, mismatched admitted
operation/result or altered unit/sign. Exercise real producer → adapter → consumer
with independently expected healthy neighbors. Separate helper passes or mutually
agreeing round trips do not prove correct composition. Include
applicable post-effect failure/uncertainty cases; preserve spent resources and
actual completion semantics. Reuse existing sensitivity mechanisms; do not count
these controls as generated mutation kills or reopen unrelated campaigns.

Verify invalid input causes no prohibited effect; valid-shaped unauthorized/stale input fails current guard. Cross-process Windows enforcement requires native checks. Disputed expectation: locate authoritative contract before changing tests/production.

Check post-transform bounds, invalid-update state preservation and release-active
rejection, including non-finite in-memory values and changed context after an
await. Verify bounded error generation on maximum admitted inputs, not just final
error truncation. Every failure case needs an independently expected valid neighbor.

For product mathematics, apply [OD-10’s first-release deferral](../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026):
formal work and pending proof admission/metadata repairs resume after V1.
Missing or invalidated proofs do not block V1 acceptance or release. Record
remaining obligations under existing source/CHECK owners as post-V1 work;
retain completed evidence only within exact historical scope. Continue
independent behavioral/property/boundary checks and all nonformal gates.
Use the [proof workflow](../references/formal-verification.md) on post-V1 resumption.

Use bounded cargo-fuzz for a real raw-admission/conversion gap. Kani checked-domain/arithmetic proofs resume after V1; retain the admitted
[guarded lanes](../references/configured-tooling.md#additional-rust-analysis)
and qualify actual harness reachability. For uncertain constructor/guard bypass
paths use [CodeQL data-flow analysis](../references/codeql-data-flow.md), then
confirm path through real bytes, consumers and protected-effect observations.
