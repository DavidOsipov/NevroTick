# Unavoidable Rust contracts

Apply to every new or changed function and module, including private helpers,
methods, callbacks and trait implementations. [Architecture section 6](../../../../../docs/windows-time-architecture.md#6-domain-types-and-state-authority)
owns requirements.
Owner adopted garde for shared structured-input/settings validation; development
requirement, not evidence of an installed or qualified application dependency
graph. Do not scaffold a product workspace for this check.

## Assume hostile input at admission

User-entered product data and external programmatic input are potentially
malicious, including well-formed values. Cover typing/paste and UI-control bypass,
direct IPC/automation, peer responses, stored/imported records and external/native
results at implemented surfaces. Local/authenticated/admin origin, protected
storage or previous acceptance supplies no blanket payload-safety or
current-authorization exemption. Do not expand UI-M0 or add input surfaces to
exercise this policy.

Bound raw input and aggregate work before allocation/expensive processing; retain
representation needed for strict parsing. Run shared garde/domain admission
inside checked construction, then current trusted-owner guards at the sink.
Treat text as data, not format strings, executable commands or authority to choose
actions, paths or destinations. Errors must be bounded/redacted; rejection must
preserve prior valid state and release resources appropriately.

Challenge actual handlers independently of UI checks. Include malformed input,
well-formed unauthorized/stale input and repeated individually valid requests
exhausting aggregate budgets, alongside accepted neighboring controls. Stable
invariants in immutable internal domain values remain reusable below;
this rule does not require reparsing those values on every call.

## Define and enforce the contract

For each interface identify accepted values/units/provenance, preconditions,
success postconditions, error behavior, allowed effects and resource/lifecycle
obligations. Document beside the Rust owner; signatures and shared invariant
documentation may carry simple helpers' details. Currently trusted callers do
not exempt internal functions.

Apply the [Rust API-selection rule](../../implement-rust/SKILL.md): choose the
strongest applicable guarantees while preserving valid behavior; explain a
necessary weaker API form and where its missing guarantees remain enforced.

| Guarantee | Enforcement |
| --- | --- |
| Representation and resource admission | Bound raw reads/collections before allocation; strict complete decoding or protocol-specific parsing |
| Stable value invariants | Private domain types, checked construction and preserved immutable state |
| Relationships between inputs | Shared pure checks called by the constructor/operation before effects |
| Transformation postconditions | Checked arithmetic/conversion, finite-value and range checks, then checked result construction |
| Changing permission, generation or budgets | Current trusted-owner guard serialized with the effect; recheck after awaits or other invalidation |
| Errors, cancellation and effects | Explicit `Result`/state transitions, failure atomicity and actual native-operation completion/acknowledgement |

Pass immutable validated types between internal functions: enforced invariants
satisfy unchanged preconditions without repeated parsing or validation. Every transformation
must preserve or re-establish its promised result. Distinct units and authority
meanings need distinct types; bare primitives do not encode them. Retain existing
unit/role types through helper parameters, local differences and stored witnesses;
extract primitives only for calculations needing them. Unit types alone prove no
common origin or generation; retain corresponding owner/context checks.
Mutable/external/contextual facts need rechecking when they can change. Infallible signatures need
an invariant-based reason.

For freshness or deadlines, read the trusted clock afresh at each contract-required
transition, including after time-advancing work and immediately before the relevant
publication or effect. Production adapters own reads: no caller-supplied trusted
`now` or frozen entry instant across a multistage operation. Pure calculations and
fixtures may receive explicit observations under existing contracts; inputs alone
grant no production clock authority. Test through the same transition implementation
using a private or test-only bounded deterministic clock sequence. Fail clearly on
exhaustion; preserve units and order. Pair timely entry then later expiry with an
entirely timely control. Do not widen production injection or change real transition semantics.

Required checks must run in production builds. `debug_assert!`, comments,
successful earlier checks, generic `Validated<T>` labels or test-only assertions
cannot enforce runtime rejection. Handle invalid input and arithmetic failure
explicitly; do not turn expected rejection into panic, fallback default,
silent clamp or ignored `Result`. Test assertions still supply observations.

## Parse, don't validate: preserve established invariants

When admission establishes a stable property needed downstream, return the smallest
private domain value or immutable parsed view carrying it; require that representation
at consumers. Passing the original general-purpose value after successful checking
loses type-level evidence. Keep existing shared garde/domain rules inside checked
construction; this selects no new validation framework or blanket typestate conversion.

Design from the consumer's actual requirements:

1. Name the exact fact needed, its enforcing owner and lifetime.
   A numeric range, complete frame and current permission are different facts.
2. Choose a representation that excludes meaningful mistakes: private newtypes
   for units or identities, variants with their required data for exclusive states,
   or a constrained collection when a consumer requires its invariant.
   Do not wrap every primitive or invent a nonempty requirement to fit an example.
3. Establish that fact before any consumer or protected effect relying on it.
   Refine only the relevant branch when outcomes require different facts;
   inert wire types may retain fields whose semantics are not yet admitted.
4. Review transformations weakening guarantees. Filtering, removing, slicing,
   merging or extracting a raw inner value can lose nonemptiness, bounds, units or
   identity. Preserve the property by construction, expose an honestly weaker
   result, or use checked construction before another consumer relies on it.

Existing [era APIs](../../../../../crates/time-domain/src/era.rs) illustrate
checked construction followed by a consumer requiring the resulting type:

```rust
let window = EraWindow::new(earliest, latest)?;
let coordinate = unfold_ntp(seconds, fraction, &window)?;
```

`EraWindow` carries checked coordinate bounds and endpoint ordering, not trust in
the supplied time anchor. `unfold_ntp` still returns `Result`: the window may admit
zero or multiple era candidates; checked arithmetic remains. Preserve real failure
cases when stronger input types eliminate only some.

Existing [datagram parser](../../../../../crates/ntp-wire/src/datagram.rs)
provides another representation: `RawDatagram` borrows immutable input after
complete bounded extension-chain admission. Its iterator uses that invariant
without allocating a packet copy. Framing grants no authentication, exchange
association or sample eligibility. Preserve these layer-specific meanings when
changing raw or domain types.

A pure predicate inside construction may return `bool` or `Result<(), E>`;
an immediately consumed current-state guard can also return `Result<(), E>`.
Require checked values where another consumer needs a stable fact; do not ban
helper signatures. For example,
[`RequestTiming`](../../../../../crates/ntp-exchange/src/context.rs) establishes
interval shape; the
[association owner](../../../../../crates/ntp-exchange/src/association.rs) must
still check current deadline and generations when processing the response.

When admission and consumption must concern the same owner and value, bind them in
one owner-controlled operation or private consuming value carrying that relationship.
A returned boolean, ordinary lifetime or matching numeric label alone cannot
establish it. Obtain related context/metadata from the actual owner or validate
their association; checking one control cannot bind unrelated later arguments.
Keep historical committed facts distinct from current permission: detaching
history must not reopen a completed transaction.

Preserve input evidence during conversion. Collecting pairs into a `HashMap` can
discard duplicate-key values; unique results do not prove duplicate-free input.
Reject duplicates before loss when required by
[strict admission](strict-boundaries.md#decoding-without-representation-loss).

Keep constructor/mutation/deserialization review and existing consumer checks
below. State each type's exact guarantee and remaining dynamic conditions beside
its owner. Prefer moving or borrowing over redundant copies, account for retained
backing buffers, and use the existing
[performance procedure](../../optimize-rust-performance/SKILL.md) before claiming
CPU, allocation or memory savings. An added type alone is no measured improvement.

## Domain equivalence and trait contracts

Review invalid states and equivalent representations separately. "Tightness" is
a modeling question, not a numerical quality target: valid states need not have
one representation per domain value. Define equivalence for the actual consumer
before choosing comparison,
deduplication or collection keys. A lawful derived trait can still encode the
wrong domain identity.

Where the domain equates accepted forms, canonicalize through checked
construction, use a dedicated semantic key, or implement relevant traits
consistently over the same semantic projection. Preserve original form when its
bytes, presentation or provenance matter; canonical storage is not mandatory.
Derive traits only when stored fields and their comparison order express intended
meaning. For an unordered pair, a private sorted representation may make derivation
appropriate; an ordered interval must reject reversed bounds under its existing
contract. Sorting is not a universal repair.

For implemented traits, preserve documented laws:

- Equal values must hash equally from the same hasher initial state. Unequal
  values may share hashes; collisions permitted.
- `Ord::cmp` returning `Ordering::Equal` must agree with equality;
  `partial_cmp(a, b)` must agree with `Some(cmp(a, b))`. Implement `Ord` only when
  the intended relation is a total order; retain partial or explicitly scoped
  comparison where that is the actual contract.
- Keep equality, hash and ordering behavior stable while a value is a collection
  key. Review manual and derived implementations together when fields change.

Apply equivalence at its owning semantic layer. Do not normalize opaque tokens,
authentication-covered bytes or ordered histories merely to remove alternatives.
Preserve input needed for strict admission and authentication, and consumer-required
distinctions such as provenance, logical-source identity and generation.
Normalization collisions must follow the domain's explicit reject/coalesce
rule. Route changes to accepted meaning, wire or persisted representation through
[contract evolution](../../evolve-data-contracts/SKILL.md).

Use [contract tests](../../../testing/qa-contract-tests/SKILL.md) with equivalent
inputs and distinct controls at affected consumers. Standard-library hash output
is no stable serialization format or fixed test oracle; verify relevant
equality/hash/ordering relationships and collection behavior instead.

Preserve deliberate trait absence where the actual owner requires it, including
numerical equality, ordering and hashing on
[`RoundedRationalSeconds`](../../../../../crates/time-domain/src/rational_seconds.rs).
For each claimed absence among `PartialEq`, `Eq`, `PartialOrd`, `Ord`, `Hash`,
`Send` and `Sync`, retain a separate independent compiler-negative control and
legal use on admitted compiler, target and features. A combined bound can hide
an added capability behind another trait's refusal. Use external consumers for
public API claims and admitted in-crate scope naming private guarantee owners;
unrelated privacy/import errors prove none of these guarantees.

For thread affinity, test the actual guard, such as private
[`Apartment`](../../../../../crates/nevrotick-ui/src/native_notifications_windows.rs)
with its `PhantomData<Rc<()>>` marker; containing-port refusal alone is insufficient.
Challenge `Send` and `Sync` independently. Incidental non-thread-safe fields or
test payloads must not mask a lost marker; use otherwise capable generic payloads
where applicable and inspect the intended diagnostic cause. Another field's
restriction does not protect the guard. Apply at the next substantive owner edit;
no blanket trait ban. Preserve justified `Copy`/`Clone` and adapter extension points.

## Make structural changes require review

When a new field or variant needs a semantic decision, make the compiler identify
affected consumers. Apply at the relevant owner; full enumeration in every pattern
not required.

- In manual equality, hashing, ordering or `Debug` implementations, use exhaustive
  named-field destructuring when every new field needs review. Name deliberately
  ignored fields with `field: _` and explain their exclusion; avoid `..` at that
  decision point. Keep trait laws aligned and sensitive fields redacted. Derive
  only when all stored fields express intended semantics. Preserve required
  bitwise floating distinctions and allocation identity; ordinary value equality
  is no substitute. Equality-based whole-state rejection tests observe only
  included fields. Make additions force decisions; retain independent behavioral/effect
  observations. Fully redacted formatting needs no field enumeration unless new
  fields require formatting decisions.
- Initialize all fields explicitly where a new policy/invariant field needs an
  intentional value. Avoid `..Default::default()` there; reuse named defaults only
  with appropriate semantics. A valid `Default` remains useful when the complete
  default state satisfies the type's contract.
- Enumerate variants of closed internal state enums where each new state needs
  handling, grouping known variants with identical behavior. Required wildcard
  arms for external non-exhaustive APIs or intentional open input sets need
  explicit unknown-case behavior under the owner's contract.

For collection access, bind element and admission in one operation
with a slice pattern, `first`, `split_first` or `get` where appropriate. Handle
empty/multiple/missing cases under the real contract; do not replace a panic with
silent omission or an invented default. Existing immutable bounded views may
already carry the required access invariant.

Limit mutable initialization to a small scope before publishing the result.
An immutable binding does not freeze interior-mutating data or changing context.
Named option enums clarify call sites but do not exclude invalid combinations of
independent options; use checked construction or variants carrying required data
when the combinations themselves matter.

Before replacing phase plus optional payloads with variant-owned state, map legal
transitions, required data and failure custody. Remove a concrete maintenance
hazard through narrow changes; impose no blanket typestate rewrite.
Preserve irreversibly spent identifiers, archived cookies, charged work and
unknown-effect custody across preparation failure, panic and cancellation.
Dropping a value alone proves neither completion nor permission to refund.

When callers must correlate operation, termination and cleanup queries, prefer
one closed observation from the actual owner. Distinguish pending,
confirmed cleanup with retained operation success/failure, and unconfirmed
cleanup with its cause. Preserve latched terminal observations and allowed retry
behavior, distinguishing pre-attempt refusal from actual/indeterminate attempts.
Neither a boolean success test nor a terminal error alone proves cleanup.

Select lints by actual behavior on the admitted compiler.
`clippy::unneeded_field_pattern` recommends replacing named ignored fields with
`..`, defeating the exhaustive-field technique above. Do not enable it to enforce
that guarantee.

Mark important outcome enums or structs returned across wrappers with type-level
`#[must_use]` and a meaningful handling message. A function annotation alone does
not protect an ignored plain outcome returned by an unannotated wrapper. Cleanup
results such as
[`CloseOutcome`](../../../../../crates/nevrotick-ui/src/native_notifications_windows.rs)
need that obligation across return boundaries. At the next substantive owner edit,
pair an ignored-expression compiler-negative consumer at the actual wrapper with
legal outcome handling. Admit effective `unused_must_use` error level; require its
intended diagnostic. Deliberate discard such as `let _ = outcome;` suppresses it
and is no negative control. This diagnostic imposes no linear or exactly-once
consumption and proves neither correct handling or application nor lifetime or
cleanup completion. Preserve appropriate `Copy`/`Clone`; retain state and effect observations
for actual outcome behavior.
Use [static checks](../../../testing/qa-static-checks/SKILL.md) for claimed compiler
guarantees and existing runtime/proof owners for semantic correctness.

## Prepare and commit owned state

Use this ownership/ordering pattern when failed updates must preserve previous
valid state; it is no new transaction framework:

1. Identify live state, its single writer, complete update and exact
   commit point. Include metadata, retained histories, cookie stores, pending
   requests, generation and estimates that must change together.
2. Bound and validate candidates before eviction or mutation. Prepare transformed
   results and any fallible allocation in bounded operation-owned scratch.
   Failed preparation must release scratch without consuming live entries or
   publishing a partial result.
3. Validate prepared output and current context. A synchronous operation with
   exclusive ownership can reuse unchanged context; do not add redundant checks
   merely to mimic an async pattern. After an await, callback or unlocked
   computation, compare required generation/revision/authority again under
   the owner serializing publication. An early check followed by an unlocked
   commit leaves a race.
4. Commit complete prepared state with no remaining expected fallible work inside
   the in-memory transition. Account for destructors/callbacks and resource
   release; calling a swap atomic does not make arbitrary cleanup infallible.
   Publish acknowledgement/status only for the completed transition.

Example: a full bounded estimator history receives a new sample.
Admission and a successful staged fit must precede eviction; numerical failure
preserves old history and estimate, with honest existing age/status. A valid
replacement removes only the contract-selected oldest item, adds the sample once
and publishes the checked estimate together. If the contract permits retaining a
sample despite fit failure, that is a different explicit transition, not an
accidental half-update. The owning task, not this example, controls these choices.

For every promised failure-atomic transition, snapshot observable state and inject
failure at each reachable preparation/check stage. Observe equality on rejection
and the exact accepted change for a valid neighbor. Cover full capacity, failed
transformation, stale prepared output and repeated delivery where relevant. Do not
construct impossible invalid typed states to claim coverage. A fake observer
must not perform the transition or restore state before state inspection.

Current [exchange owner](../../../../../crates/ntp-exchange/src/association.rs)
illustrates staging a complete event before consuming its pending slot; the
[history proposal](../../../../../tasks/s1.2-regression-readiness.md#proposed-smallest-pure-contract-no-activation)
states validate-before-evict obligations. Neither is evidence for future NTS or
Windows effects. A clock write, durable publication or native I/O may precede an error:
use the existing
[error/effect recovery procedure](../../implement-rust/references/error-handling.md)
and actual acknowledgement/reconciliation contract. A local staged copy cannot
roll back external effects or prove crash durability.

## Put garde inside the only admission path

Adopted starting dependency declaration for the eventual owning Cargo manifest:

```toml
garde = { version = "=0.23.0", default-features = false, features = ["derive"] }
```

Admit its actual locked runtime/proc-macro graph, features, licenses and target
before use through [dependency updates](../../../environment/dependency-updates/SKILL.md).
Do not enable `full`, a second validation framework or unneeded rule dependencies.
This declaration permits no weakening of cargo-deny/cargo-vet gates.

1. Decode bounded bytes into inert wire data. Garde runs on values; it does not
   enforce raw duplicate-key, integer-spelling, full-consumption or allocation
   admission rules. See [strict boundaries](strict-boundaries.md).
2. A private fallible constructor invokes the single shared garde/pure domain
   validation path before creating the domain value. Deriving `Validate` alone
   performs no validation. Reuse one maintained implementation per rule;
   custom garde rules may call the same domain predicates used elsewhere.
3. UI feedback calls that same rule owner. A receiving service independently
   constructs its own domain value; UI success or sender-supplied validation
   markers convey no trusted evidence.
4. Consumers receive checked results. Mutations, builders, migrations and
   deserialization must return through checked construction. Validate a proposed
   replacement before publication; preserve prior valid state on failure.

Require a rule, nested `dive`, or justified exemption for every field of a
garde-derived type. Prohibit `#[garde(allow_unvalidated)]`. A `#[garde(skip)]`
needs an adjacent reason identifying actual invariant owner and enforcement,
plus independent review; it must not be an agent's shortcut to make derive compile.
Metadata-only annotations establish no validating rule even if derive accepts them.
Review actual rule execution and challenge it with invalid input.
Distinguish optional presence from validation of a supplied value. A declared
range does not prove correct units, finite floating values or related fields.

No public fields, tuple fields, mutable-inner getters, `DerefMut`, unchecked
`From`, permissive `Default`, unrestricted domain `Deserialize`, or alternate
factory may construct/publish invalid domain state. Audit defining-module code,
descendants and re-exports: descendants can access a parent's private fields for
direct construction or mutation. Private fields or seals constrain callers outside that scope;
`#[non_exhaustive]` supplies no construction restriction inside the defining crate.
Neither mechanism validates fields or blocks exposed mutation. Test-only
construction escapes stay outside production builds.

Where a concrete descendant-construction bypass risk warrants stronger privacy,
place the invariant-bearing type and admission implementation in a narrow leaf.
Siblings use checked
constructors and invariant-preserving read-only getters; raw fields stay private
to the leaf. Review visibility, re-exports and mutation paths; preserve supported
adapters. Apply conditionally at the next substantive owner edit: no blanket
module migration or new crates merely for privacy. Parent privacy alone establishes
no current bug. Pair sibling-scope forbidden construction/mutation with legal
checked use; privileged child tests inside the leaf cannot alone prove sibling
exclusion.

Expose only production routes actual consumers need. Keep general injection,
raw-result extraction and other invariant-sensitive helpers private when production
callers share their module. Use one implementation kernel; give sibling tests
forwarding adapters under `#[cfg(test)]` where needed. Child-module tests already
have private access. Preserve intentional adapter extension points; do not seal
them merely to reduce visibility. Test access must not widen production capabilities
or duplicate custody/validation logic.

`garde::Valid<T>` records that validation ran; it is not automatically a durable
invariant or permission. Shared dereference can expose `T`'s interior-mutating
methods or aliases; `into_inner` unwraps it. Use only with suitable owned,
invariant-preserving data and APIs. Borrowed validation cannot freeze another
owner's mutable data. Restrict setters, interior mutability, callbacks and FFI
writes to checked transitions; revalidate external/native results before use.

Keep validation pure and bounded. Check `is_finite()` explicitly for floating
values before range/relationship checks. Bound input collections before garde
iterates or accumulates errors. Translate reports into stable redacted product
codes and allowlisted paths within the architecture's error budget. Do not log
or serialize arbitrary reports or rejected values.

## Demonstrate that callers cannot bypass it

For each changed guarantee retain actual entry point, enforcement owner,
checked consumer and discriminating check with a valid neighboring control:

- External-consumer compile-fail checks challenge private fields/constructors and
  raw-type use at a checked-type API; a legal constructor/use example must compile.
  Unrelated build failure is no successful bypass check.
- Runtime cases challenge invalid fields, cross-field relations, deserialization,
  alternate factories, mutations and migrations; prove rejection has no protected
  effect and preserves old valid state.
- Transformation cases check output units/ranges, overflow, NaN/infinity and
  actual postconditions using independent expectations.
- Context/lifecycle cases invalidate permission, generation or budgets between
  entry and effect, including after awaits. Observe the protected sink, not only
  an intermediate validator return value.

Use [contract tests](../../../testing/qa-contract-tests/SKILL.md) and meaningful
property/fuzz controls. Apply [OD-10's first-release deferral](../../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026):
formal proofs and pending proof admission/metadata repairs resume after V1;
missing proofs do not block first-release acceptance. Retain exact historical
scope and post-V1 obligations without weakening runtime enforcement or tests.
Use the [proof workflow](../../../testing/references/formal-verification.md)
on resumption.
[CodeQL](../../../testing/references/codeql-data-flow.md) can investigate paths
around construction or from original DTOs to effects; inspect real callers and
challenge query coverage before claiming no bypass. Compiler docs/lints, skill
checks and source searches do not establish semantic correctness. Keep native
Windows and UI-M0 applicability limits from the canonical architecture.

## Primary implementation evidence

Reviewed garde 0.23.0 sources: [derive checks](https://github.com/jprochazk/garde/blob/v0.23.0/garde_derive/src/check.rs),
[validation wrappers](https://github.com/jprochazk/garde/blob/v0.23.0/garde/src/validate.rs),
[range rules](https://github.com/jprochazk/garde/blob/v0.23.0/garde/src/rules/range.rs)
and [error reports](https://github.com/jprochazk/garde/blob/v0.23.0/garde/src/error.rs).
These explain field coverage, explicit execution, wrapper limits, floating range
behavior and allocations; qualify the exact resolved implementation with real
controls before making application claims.

Rust primary references: [outcome diagnostics](https://doc.rust-lang.org/reference/attributes/diagnostics.html#the-must_use-attribute),
[privacy scopes](https://doc.rust-lang.org/reference/visibility-and-privacy.html),
[thread transfer](https://doc.rust-lang.org/std/marker/trait.Send.html) and
[shared references](https://doc.rust-lang.org/std/marker/trait.Sync.html).
These describe language mechanisms; qualify actual owner and build scope with the
controls above. They prove no runtime or native correctness.
