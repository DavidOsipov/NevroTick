# Unavoidable Rust contracts

Apply to every new or changed function and module, including private helpers,
methods, callbacks and trait implementations. [Architecture section 6](../../../../../docs/windows-time-architecture.md#6-domain-types-and-state-authority)
owns the requirements.
The owner has adopted garde for shared structured-input/settings validation.
This is a development requirement, not evidence of an installed or qualified
application dependency graph. Do not scaffold a product workspace for this check.

## Assume hostile input at admission

User-entered product data and external programmatic input are potentially
malicious, including well-formed values. Cover typing/paste and UI-control bypass,
direct IPC/automation, peer responses, stored/imported records and external/native
results at the surfaces actually implemented. Local/authenticated/admin origin,
protected storage or previous acceptance supplies no blanket payload-safety or
current-authorization exemption. Do not expand UI-M0 or add an input surface to
exercise this policy.

Bound raw input and aggregate work before allocation/expensive processing; retain
the representation needed for strict parsing. Run shared garde/domain admission
inside the checked construction path, then current trusted-owner guards at the
sink. Treat text as data, not as format strings, executable commands or authority
to choose actions, paths or destinations. Errors must be bounded/redacted and
rejection must preserve prior valid state and release resources appropriately.

Challenge actual handlers independently of UI checks. Include malformed input,
well-formed unauthorized/stale input and repeated individually valid requests
that exhaust aggregate budgets, alongside accepted neighboring controls. Stable
invariants carried by immutable internal domain values remain reusable below;
this hostile-input rule does not require reparsing those values on every call.

## Define and enforce the contract

For each interface identify accepted values/units/provenance, preconditions,
success postconditions, error behavior, allowed effects and resource/lifecycle
obligations. Put the contract beside its owner in Rust documentation; signatures
and shared invariant documentation may carry the details for simple helpers.
An internal function is not exempt because its caller is currently trusted.

| Guarantee | Enforcement |
| --- | --- |
| Representation and resource admission | Bound raw reads/collections before allocation; strict complete decoding or protocol-specific parsing |
| Stable value invariants | Private domain types, checked construction and preserved immutable state |
| Relationships between inputs | Shared pure checks called by the constructor/operation before effects |
| Transformation postconditions | Checked arithmetic/conversion, finite-value and range checks, then checked result construction |
| Changing permission, generation or budgets | Current trusted-owner guard serialized with the effect; recheck after awaits or other invalidation |
| Errors, cancellation and effects | Explicit `Result`/state transitions, failure atomicity and actual native-operation completion/acknowledgement |

Pass immutable validated types between internal functions. Their enforced
invariants satisfy unchanged preconditions without repeated parsing or validation
calls. Every transformation must preserve or re-establish its promised result.
Use distinct types for different units and authority meanings; a bare primitive
does not encode them. Mutable/external/contextual facts must be checked again
when they can change. Infallible signatures require an invariant-based reason.

Required checks must run in production builds. `debug_assert!`, comments, a
successful earlier check, a generic `Validated<T>` label or test-only assertions
cannot enforce a runtime rejection contract. Handle invalid input and arithmetic
failure explicitly; do not turn expected rejection into a panic, fallback default,
silent clamp or ignored `Result`. Test assertions still supply observations.

## Prepare and commit owned state

Use this pattern when a failed update must preserve the previous valid state.
It is an ownership/ordering method, not a new transaction framework:

1. Identify the live state, its single writer, the complete update and the exact
   commit point. Include metadata, retained histories, cookie stores, pending
   requests, generation and estimates that must change together.
2. Bound and validate the candidate before eviction or mutation. Prepare the
   transformed result and any fallible allocation in bounded scratch owned by
   the operation. Failed preparation must release scratch without consuming
   live entries or publishing a partial result.
3. Validate the prepared output and the current context. A synchronous operation
   holding exclusive ownership can reuse unchanged context; do not add redundant
   checks merely to mimic an async pattern. After an await, callback or unlocked
   computation, compare the required generation/revision/authority again under
   the owner that serializes publication. An early check followed by an unlocked
   commit leaves a race.
4. Commit the complete prepared state with no remaining expected fallible work
   inside the in-memory transition. Account for destructors/callbacks and resource
   release; calling a swap atomic does not make arbitrary cleanup infallible.
   Publish the resulting acknowledgement/status only for the completed transition.

Worked example: a bounded estimator history receives a new sample while full.
Admission and a successful staged fit must precede eviction; numerical failure
preserves the old history and estimate, with honest existing age/status. A valid
replacement removes only the contract-selected oldest item, adds the sample once
and publishes the checked estimate together. If the contract permits retaining a
sample despite fit failure, that is a different explicit transition, not an
accidental half-update. These choices belong to the owning task, not this example.

For every promised failure-atomic transition, snapshot observable state and inject
failure at each reachable preparation/check stage. Observe equality on rejection
and the exact accepted change for a valid neighbor. Cover full capacity, failed
transformation, stale prepared output and repeated delivery where relevant. Do not
construct impossible invalid typed states just to claim coverage. A fake observer
must not perform the transition or restore state before it is inspected.

The current [exchange owner](../../../../../crates/ntp-exchange/src/association.rs)
illustrates staging a complete event before consuming its pending slot; the
[history proposal](../../../../../tasks/s1.2-regression-readiness.md#proposed-smallest-pure-contract-no-activation)
states validate-before-evict obligations. Neither is evidence for future NTS or
Windows effects. A clock write, durable publication or native I/O may already have
happened when an error arrives: use the existing
[error/effect recovery procedure](../../implement-rust/references/error-handling.md)
and actual acknowledgement/reconciliation contract. A local staged copy cannot
roll back an external effect or prove crash durability.

## Put garde inside the only admission path

The adopted starting dependency declaration for the eventual owning Cargo
manifest is:

```toml
garde = { version = "=0.23.0", default-features = false, features = ["derive"] }
```

Admit its actual locked runtime/proc-macro graph, features, licenses and target
before use through [dependency updates](../../../environment/dependency-updates/SKILL.md).
Do not enable `full`, a second validation framework or unneeded rule dependencies.
The declaration above is not permission to weaken cargo-deny/cargo-vet gates.

1. Decode bounded bytes into inert wire data. Garde runs on values; it does not
   enforce raw duplicate-key, integer-spelling, full-consumption or allocation
   admission rules. See [strict boundaries](strict-boundaries.md).
2. A private fallible constructor invokes the single shared garde/pure domain
   validation path before creating the domain value. Deriving `Validate` alone
   performs no validation. Reuse one maintained implementation of each rule;
   custom garde rules may call the same domain predicates used elsewhere.
3. UI feedback calls that same rule owner. A receiving service independently
   constructs its own domain value; UI success or sender-supplied validation
   markers convey no trusted evidence.
4. Consumers receive the checked result. Mutations, builders, migrations and
   deserialization must return through checked construction. Validate a proposed
   replacement before publishing it; preserve the prior valid state on failure.

Require a rule, nested `dive`, or justified exemption for every field of a
garde-derived type. Prohibit `#[garde(allow_unvalidated)]`. A `#[garde(skip)]`
needs an adjacent reason identifying the actual invariant owner and enforcement,
plus independent review; it must not be an agent's shortcut to make derive compile.
Metadata-only annotations do not establish a validating rule even if derive
accepts them. Review actual rule execution and challenge it with invalid input.
Distinguish optional presence from validation of a supplied value. A declared
range does not prove the correct units, finite floating values or related fields.

No public fields, tuple fields, mutable-inner getters, `DerefMut`, unchecked
`From`, permissive `Default`, unrestricted domain `Deserialize`, or alternate
factory may construct/publish invalid domain state. Audit defining-module code
and re-exports too: Rust privacy does not prevent the defining module from adding
a bypass. A test-only construction escape stays outside production builds.

`garde::Valid<T>` records that validation ran; it is not automatically a durable
invariant or permission. Its shared dereference can expose `T`'s interior-mutating
methods or aliases; `into_inner` unwraps it. Use it only with suitable owned,
invariant-preserving data and APIs. Borrowed validation cannot freeze another
owner's mutable data. Restrict setters, interior mutability, callbacks and FFI
writes to checked transitions; revalidate external/native results before use.

Keep validation pure and bounded. Check `is_finite()` explicitly for floating
values before range/relationship checks. Bound input collections before garde
iterates or accumulates errors. Translate reports into stable redacted product
codes and allowlisted paths within the architecture's error budget. Do not log
or serialize arbitrary reports or rejected values.

## Demonstrate that callers cannot bypass it

For each changed guarantee retain the actual entry point, enforcement owner,
checked consumer and a discriminating check with a valid neighboring control:

- External-consumer compile-fail checks challenge private fields/constructors and
  raw-type use at a checked-type API; a legal constructor/use example must compile.
  A failed build for an unrelated reason is not a successful bypass check.
- Runtime cases challenge invalid fields, cross-field relations, deserialization,
  alternate factories, mutations and migrations; prove rejection has no protected
  effect and leaves the old valid state intact.
- Transformation cases check output units/ranges, overflow, NaN/infinity and
  actual postconditions using independent expectations.
- Context/lifecycle cases invalidate permission, generation or budgets between
  entry and effect, including after awaits. Observe the protected sink rather
  than only an intermediate validator return value.

Use [contract tests](../../../testing/qa-contract-tests/SKILL.md), meaningful
property/fuzz controls and bounded Kani proofs for suitable finite invariants.
[CodeQL](../../../testing/references/codeql-data-flow.md) can investigate paths
around construction or from original DTOs to effects; inspect real callers and
challenge query coverage before claiming no bypass. Compiler docs/lints, skill
checks and source searches do not establish semantic correctness. Keep native
Windows and UI-M0 applicability limits from the canonical architecture.

## Primary implementation evidence

The reviewed garde 0.23.0 sources are its [derive checks](https://github.com/jprochazk/garde/blob/v0.23.0/garde_derive/src/check.rs),
[validation wrappers](https://github.com/jprochazk/garde/blob/v0.23.0/garde/src/validate.rs),
[range rules](https://github.com/jprochazk/garde/blob/v0.23.0/garde/src/rules/range.rs)
and [error reports](https://github.com/jprochazk/garde/blob/v0.23.0/garde/src/error.rs).
These explain field coverage, explicit execution, wrapper limits, floating range
behavior and allocations; qualify the exact resolved implementation with real
controls before making application claims.
