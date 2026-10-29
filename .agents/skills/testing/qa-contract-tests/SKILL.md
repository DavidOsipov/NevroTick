---
name: "qa-contract-tests"
description: "Verify NevroTick function/module preconditions, postconditions, mandatory validation, strict wire admission and consumer compatibility through actual callers."
---

# Test function, module and wire contracts

Read [the testing contract](../CONTRACT.md).

Read applicable implementation contract/[strict contract cases](../references/numerical-protocol.md). Trace raw bytes → bounded map-only DTO → private fallible domain construction → current guard → effect/acknowledgement. Identify each owner/next-boundary consumed representation.

Apply [contract enforcement](../../implementation/implement-rust-contracts/references/contract-enforcement.md)
to every changed function/module: identify input requirements, output guarantees,
failure/effect/lifecycle behavior and the actual enforcing type or operation.
Exercise direct in-process callers as well as wire paths. Immutable validated
inputs need no repeated parsing; transformations still owe checked postconditions.

Verify the private constructor actually invokes the shared garde/domain rules and
all consumers use its result. Challenge builders, defaults, `From`/deserialization,
mutation, interior mutability, migration and FFI conversion. A standalone
`validate()` unit test does not prove consumers cannot bypass it. Reject
`allow_unvalidated`; examine each `skip` against its independently reviewed owner.
Check actual rule/`dive` coverage: metadata-only attributes may compile while
checking no invariant, so challenge the field's semantic rule through its consumer.
Field omission and external construction/mutation/privacy claims need the intended
compile diagnostic plus a compiling legitimate consumer; unrelated errors do not
qualify the restriction.

Test unknown/duplicate/escaped keys, object-versus-array shape, missing/null/invalid values, complete consumption, versions, exact integers, finite computed values, limits, every constructor/migration bypass. Include valid neighboring/legitimate maximum-size controls; reject-all must fail.

Compatibility: freeze supported producer/consumer version/feature matrix. Compare meaning, units, presence/defaults, failure behavior; shape/round trips insufficient. Public Rust visibility/type/trait claims need minimal real external consumer; separate wire-version/Rust-source compatibility.

Verify invalid input causes no prohibited effect; valid-shaped unauthorized/stale input fails current guard. Cross-process Windows enforcement requires native checks. Disputed expectation: locate authoritative contract before changing tests/production.

Check post-transform bounds, invalid-update state preservation and release-active
rejection, including non-finite in-memory values and changed context after an
await. Verify bounded error generation on maximum admitted inputs, not only final
error truncation. Every failure case needs an independently expected valid neighbor.

Use bounded cargo-fuzz for a real raw-admission/conversion target, or Kani for a
small checked-domain/arithmetic property, only when that evidence gap warrants
it; select their [guarded lanes](../references/configured-tooling.md#additional-rust-analysis)
and qualify actual harness reachability. For uncertain constructor/guard bypass
paths use [CodeQL data-flow analysis](../references/codeql-data-flow.md), then
confirm the path through real bytes, consumers and protected-effect observations.
