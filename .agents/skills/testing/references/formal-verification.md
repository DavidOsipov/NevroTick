# Post-V1 mathematical Rust proofs

**Current disposition, 6 November 2026:** [OD-10’s first-release deferral](../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026)
defers formal proof work until after the first released software version (V1).
Do not initiate formal compiler/solver/proof jobs, author proofs, extend verifiers
or repair pending proof admission/metadata consumers before V1. Missing,
invalidated or model-only proofs do not block V1 implementation, acceptance or
release. Continue ordinary product development and all nonformal gates.

Keep completed proofs and failed/incomplete receipts within their exact
historical scope; maintain unfinished/newly affected obligations under existing
source/CHECK owners as post-V1 work. Source changes do not extend old evidence.
The sections below describe normal bounded post-V1 resumption, not permission
to restart the former proof-first queue. No extra release or owner-approval gate
is introduced here. Runtime validation, garde/private invariants, checked
arithmetic, independent behavioral/property/boundary checks, coverage/mutation,
security, native/package/release evidence and all existing holds remain.

Read the [testing contract](../CONTRACT.md) and
[configured analysis lanes](configured-tooling.md#additional-rust-analysis).
This reference admits no new tool, dependency, compiler, CI gate or product claim.
Keep candidate-specific decisions/results in the existing task record.

## Source-bound acceptance on post-V1 resumption

Apply [OD-10 §14.2](../../../../docs/nevrotick-owner-decisions.md#142-proof-purpose-and-completion--6-november-2026).
Start from one named useful guarantee at a bounded product-operation boundary,
with an independent specification and an existing requirement/CHECK owner. Trace
admitted input through actual validation/construction and computation to the
observable result, state or effect. Define required success, exact error behavior,
preserved state, lifecycle and machine/rounding semantics where applicable. This
can be a small operation; it need not prove the whole program at once.

Select a helper theorem when it closes a named real requirement or supplies a
necessary premise of that guarantee. Record its consuming operation, remaining
constructor/callee/composition obligations and a credible bounded route to their
completion in the existing task. Do not substitute an easier unrelated helper
for a blocked product guarantee. Preserve useful local proofs, but do not promote
them to the containing operation's assurance while its dependencies remain open.
Finding an actual product bug is not a condition of useful proof work; reducing
material uncertainty or closing the named guarantee is the purpose.

Reconcile the existing algorithm inventory with this full admitted domain,
invariants, work bounds and executable correspondence. Review assumptions and
unproved constructors/callers. Require nonempty completed results, independent
healthy/error/compiled-defect controls and restored health before claiming a new
proof. Subset/model proofs retain only their stated scope. Changed relevant
source/specification/dependencies/configuration/tool inputs invalidate evidence
unless reviewed as irrelevant; reuse unaffected historical proofs and source
hashes. Keep open guarantees under their existing owners and the
[post-V1 debt record](../../../../tasks/development-backlog.md#product-guarantees-still-open).

A deferred obligation is not completed assurance or a permanent waiver. The
remaining proof-acceptance methodology applies after V1; it creates no current
first-release proof prerequisite.

## After V1, select an adequate verifier and evaluate additional tools

The proof requirement is already adopted. The benefit evaluation below concerns
adding or replacing a verifier; it is not an option to omit a required proof
because behavioral tests look sufficient. A tool decision of `DEFER` or `REJECT`
leaves the associated property open until an adequate admitted route proves it.

Name one important property, its actual implementation/callers, admitted input
domain and current evidence gap. Compare the same contract and assumptions with
existing tools. Kani symbolically checks machine inputs: for a loop-free finite
domain it can potentially cover every input, rather than sample a few examples.
Account for loops in constructors, libraries and other callees as well as the
selected body. Feasibility and completion within the admitted budget require
execution; installed tool controls do not prove an application property.

Verus offers deductive contracts/invariants for supported Rust; that alone does
not establish additional value for an already well-tested small function.
Distinguish a pilot from adoption: one valuable contract and a justified,
testable hypothesis of better coverage, composition, compatibility, authoring/
review effort, maintenance or recurring verification cost can justify a bounded
pilot. Define observations that would support or reject the hypothesis; an
already measured advantage or failed Kani attempt is not a prerequisite.
Before installation, identify a meaningful property and a plausible benefit
hypothesis. In the existing task, record `ADOPT_LIMITED`, `DEFER`
or `REJECT`, the evidence basis and, for a deferral, an observable return trigger.
Adoption requires demonstrated benefit from an actual-source pilot with meaningful
controls and acceptable integration/maintenance cost; source inspection alone
establishes no successful proof. Kani also documents experimental function/loop
contracts, so composition alone is no distinguishing advantage; verify availability
and admission in the pinned profile before selecting those modes.

## Compare properties separately from toolchain assurance

Verus and F* can establish the same mathematical claim when the admitted domain,
observable success/error behavior, machine semantics, assumptions and termination
obligations match. The same postcondition alone does not establish that equivalence.
Compare the actual contracts and proof coverage; neither a language ranking nor a
percentage of another tool's "correctness" is evidence. Keep proof authoring and
maintenance cost separate from the strength of the established claim.

Also compare the path from verified source through erasure/extraction, generated
code, compiler/linker and runtime/FFI to the shipped implementation. For a proposed
F*/Low*/KaRaMeL route, identify its exact backend and trusted components; a published
semantics result does not verify every tool implementation or resulting binary.
A proof of a separate F* model establishes no Rust implementation property without
an applicable correspondence argument. A rewrite needs demonstrated project value.

## Specify the whole contract independently

Write input invariants, required successful outcomes, error classification,
output guarantees and effects before proof annotations. A conditional guarantee
about `Ok` results is insufficient when always returning an error would satisfy
it. Include required success/completeness and rejection behavior. For mutable
operations distinguish preserved invariants from unchanged state on rejection.

Relate inputs to required answers: a codomain/range, finite-result, normalization
or no-panic theorem alone can accept a wrong value. For example, a sign operation
returning zero for a positive input can satisfy a result-in-{-1,0,1} postcondition.
Challenge a wrong answer that preserves such weaker properties, as well as
reject-all behavior; use the actual promised relation as the oracle.

Review assumptions against negative values, exact endpoints and transitions.
Retain concrete reachable valid/error witnesses; these detect some vacuity but
do not establish specification completeness. A proof assumption about a valid
private type must identify its real constructor/invariant owner. Proving a helper
on valid values does not also prove their constructor or authenticate their origin.
Keep independent mathematical integers, checked machine operations, exact units
and the actual producer's range distinct. A private helper's full raw domain and
its reachable caller domain can have different error guarantees.

### Relate integer specifications to machine operations

Record widths, signedness, units, intermediate ranges, casts and the exact checked,
wrapping or ordinary operations. An unbounded-integer identity does not establish
machine representability. Handling overflow with an error does not prove that
overflow is unreachable on inputs for which the contract requires success.

Verus spec integer division/remainder use Euclidean semantics, whereas Rust's
signed `/` truncates toward zero and `%` has the dividend's sign when nonzero:

| Expression | Verus spec integers | Rust signed integer operators |
| --- | --- | --- |
| `(-3) / 2` | `-2` | `-1` |
| `(-3) % 2` | `1` | `-1` |

For a nonzero divisor, Euclidean remainder is nonnegative and smaller than the
divisor's absolute value; the quotient rounds down for a positive divisor and up
for a negative one. Match the actual Rust API: `div_euclid`/`rem_euclid` deliberately
choose Euclidean behavior. Check zero divisors and signed `MIN / -1` or `MIN % -1`
against the selected API's failure/overflow contract. Verus spec division by zero
has an unspecified value; this cannot justify executable division by zero.
Do not generalize spec operator widening to named fixed-width methods or casts;
check their selected semantics and prove required ranges explicitly.

Keep negative values, exact multiples, adjacent remainder boundaries and signed
limits in independent controls. Nonlinear arithmetic may need lemmas or explicit
proof modes; record those choices and any additional backend in the pinned proof
environment. Solver difficulty changes feasibility/cost, not the promised property.

## Bind the proof to the executable implementation

Record source/configuration/feature/target/dependency identity, tool/compiler,
solver, command and actual verified functions/obligations. Establish how the
verified executable body reaches the product build. A separately translated model
can specify expected results; proving that model does not prove the implementation.
Preserve private APIs; do not expose unchecked construction for a demonstration.

For custom equality, ordering, hashing, conversion or iteration, trace the proof
through the actual overloaded/public operation. Comparing extracted integers
alone cannot prove a custom trait body's laws. A defect confined to that body
must fail the intended obligation; state which laws and operation sequences are
covered. Do not invent a custom API or duplicate primitive/derived-comparison
campaign merely to exercise this check.

Verus ghost/spec code is erased. Its `verus_only` cfg is intended for ghost
imports and verification attributes; do not select different executable behavior
for verification and production. Ordinary `cargo build` runs no Verus proof.
`cargo verus` verifies opted-in scope; focused verification skips dependency
proof rechecks. Inspect selected functions/crates and relevant dependency proof identities
before claiming the whole declared scope passed. The upstream Cargo commands are
capability documentation, not admitted NevroTick execution entry points.

Audit frozen proof inputs without executing them when source/AST/JSON reads suffice.
For a necessary Python import, inspect import-time effects and disable bytecode
writes **before** loading any frozen module: use `python3 -B` or, in Context-mode
Python, begin with `import sys; sys.dont_write_bytecode = True`. A read-only intent
or importlib loader does not prevent `__pycache__` writes. Suppression covers only
bytecode, not other module effects. Preserve and report any observer-created drift;
never delete it or rewrite receipts to turn an incomplete campaign into a pass.
A native readonly namespace does not protect its host-side observers. Where a
consumer hardens outer input trees, keep generated outputs outside them and
check exact ownership, modes, types and link identities separately: a content
snapshot alone does not establish that permissions stayed unchanged. Validate
compatibility before adopting hardening in another consumer; existing qualified
receipts are not retroactively given this protection.

Inventory project-added `assume`, axioms, `external_body`, external specifications,
ignored relevant items/implementations, stubs and environment/type assumptions.
Explain their justification and consequences. Account for the toolchain/solver's
trusted base separately; zero trust is not a realistic admission criterion.
An external specification assumes a body's contract; it does not verify that
body. Assuming the central property in a wrapper defeats the intended proof.

Review contract changes separately from executable changes: stronger preconditions,
weaker postconditions, altered specification definitions/axioms, added trusted
bodies and reduced proof selection can turn a run green by changing its promise.
Reconcile each with the accepted domain and independent controls; require an actual
proof run for the claimed scope. Generated annotations or an ordinary build do not
establish a proof gate, and a green run alone does not approve a weaker contract.

Unverified Rust callers must still establish preconditions and relevant trait/
`Drop` obligations. Keep runtime validation, safe encapsulation and checked output
construction at those boundaries; proof `requires` clauses are not runtime guards.
Reuse guarantees already enforced by private checked types and invariant owners;
proof annotations add no need to revalidate unchanged immutable values at every
call. Recheck changed representations, context or authority at their actual owners.
Do not introduce unchecked operations or remove required rejection to fit a proof.
Any production refactor needs its own behavioral/API/performance review and checks.

## Challenge and bound the pilot

Choose the named product guarantee. Set one finite time/attempt/worker budget
covering source review, specification and harness preparation, acquisition and
admission, consumer/framework work, compilation, solver jobs, controls, review
and cleanup before starting the pilot. Keep memory and other adopted limits,
the original budget clock and shipping compiler. Record costs in the existing
task; do not add a new accounting framework for this rule.

At timeout, unsupported mechanism, unqualified result or budget exhaustion, stop
dispatch and mark the guarantee open. Preserve the failure and consumed budget.
Unused budget alone permits no retry. Do not reset the clock, split one attempt
across new cards to regain time, switch to unrelated easy proofs, repair a
toolchain or extend a verification framework as an automatic continuation.
Any later attempt needs a changed relevant condition and a finite plan explaining
how it can close the same useful guarantee; a new budget alone is not that change.
If no credible route remains, record the debt and return to the product queue.
All such formal work remains post-V1 under the current deferral.

Classify a non-passing proof before changing tools. Check the exact failed
obligation and admitted domain: a real implementation/contract counterexample
requires reconciliation and concrete replay where supported; a harness/specification
error requires correcting the check. Resource exhaustion, insufficient unwinding,
unsupported source/API and setup failures require their own bounded diagnosis.
They establish neither a production defect nor an alternative tool's advantage.
Retain confirmed counterexamples across comparisons; a passing alternative must
explain them under equivalent assumptions and obligations, not hide them through
weaker inputs, scope or required-success guarantees.

Validate report/schema consumers against genuine emitted fixtures from the pinned
tool or its documented exporter schema, recording the source identity. Distinguish
internal compiler metadata from fields actually exported in the consumed report.
Synthetic and mutated fixtures supplement this evidence; do not invent fields
that make mocks and consumers agree. Missing required metadata leaves integration
unqualified, even when those mocks pass.

Require a healthy actual implementation and two or three compiled, non-equivalent
defects violating distinct promised properties. Each defect needs an admitted
witness/reason, the intended failed obligation and a restored healthy pass.
Choose plausible defects from the independent contract and actual data/state
flow before tailoring them to the harness. For a composed guarantee, include a
producer or caller defect that correct helpers alone would not reject. A surviving
semantic defect exposes a specification, reachability or composition gap; repair
that gap without weakening the contract. A source-hash rejection is custody
evidence, not a semantic kill.
Always-error, incorrect signed-boundary arithmetic and wrong outcome classification
are useful controls only when the chosen contract must reject them. Syntax errors,
unsupported calls and deletion of a postcondition are not sensitivity controls.
Use independent review to challenge weak or implementation-derived specifications.

Report preparation/build and solver costs separately, nonempty executed scope,
failed/skipped/timed-out obligations, assumptions and a small maintenance exercise.
Compare tools only on equivalent properties/domains; an unexecuted alternative
remains a hypothesis. Retain earlier failures. Bind reuse to unchanged proof,
implementation, dependency, compiler and configuration inputs.

For proof groups, reconcile expected, discovered, selected, attempted, completed
and qualified jobs, including contracts used to replace callees. Report unique
product guarantees separately from helper lemmas, repeated/restored runs, defect
controls, covers and tool-consumer tests. A timeout is an attempted unqualified
job; its presence in a summary does not make it completed. Keep preparation,
code generation, model inspection, solver completion, defect sensitivity and
restored health distinct. Missing, skipped or timed-out obligations remain open
even when other jobs are green. Infrastructure success supplies no product proof;
a workflow file or harness name proves no execution.

### Fresh candidates and historical receipts

On post-V1 resumption, maintain a repeatable invocation for each newly reviewed
candidate: explicit source/configuration/dependency/tool identity, selected
obligations, current finite budget and fresh output. Keep immutable historical
receipts separate. Do not reset expired campaign clocks, overwrite old evidence
or reuse an old summary as a new proof. Source-hash guards remain necessary, but
rejecting every changed candidate is not semantic regression protection.

Use the maintenance exercise to establish that the reviewed invocation can check
a fresh candidate and that a compiled semantic defect reaches and fails its
intended proof obligation. Reuse existing runners where adequate; add no generic
framework or parallel receipt system without a concrete gap in this guarantee.
Claim automatic regression protection only for the configured candidate-specific
proofs actually run; otherwise report manual or historical evidence explicitly.

## Bound claims about floats, termination and platforms

Verus has floating-point types, but mathematical-real reasoning alone does not
establish binary64 rounding, NaN, exceptional values or platform behavior.
Review any project floating-point axioms for consistency and actual applicability;
prefer a meaningful integer property for an initial trial.

An inductive loop invariant establishes partial correctness. A termination claim
also needs progress plus the relevant callee/environment argument. Verus exec
`decreases` checking is conditional on called functions terminating; it is not a
complete end-to-end termination guarantee. Current Kani documentation includes
experimental loop contracts, but their availability and flags must be checked
against the pinned, admitted Kani profile before use. Never silently widen it.
F* comparisons must also identify the effect: `Tot`/`GTot` require termination,
whereas `Dv`/`DIV` permits divergence. Using F* alone is no total-correctness claim.

Upstream Verus supports Windows; Linux proof of portable arithmetic still proves
no Windows API/token/IPC/clock/package behavior. Matching Rust version strings
establish no macro/library/Cargo/target compatibility. Erasing ghost code alone
establishes no unchanged application performance after an integration refactor.
Formal proof complements applicable behavioral, fuzz, memory and native evidence.

## Primary sources

Checked 1 November 2026; pinned Verus source below is research, not an admitted
binary. Consult exact selected versions again when running a future pilot.

- [Verus toolchain](https://github.com/verus-lang/verus/blob/39095af5637daa994217da75b6c5ddacdbd0c520/rust-toolchain.toml), [supported hosts](https://github.com/verus-lang/verus/blob/39095af5637daa994217da75b6c5ddacdbd0c520/INSTALL.md) and [Cargo integration](https://github.com/verus-lang/verus/blob/39095af5637daa994217da75b6c5ddacdbd0c520/source/docs/guide/src/cargo_verus.md).
- [Ghost erasure](https://verus-lang.github.io/verus/guide/erasure.html), [trusted components](https://verus-lang.github.io/verus/guide/tcb.html) and [unverified callers](https://verus-lang.github.io/verus/guide/call-from-unverified-code.html).
- [Verus spec arithmetic](https://verus-lang.github.io/verus/guide/spec-arithmetic.html), [machine and mathematical integers](https://verus-lang.github.io/verus/guide/integers.html), [nonlinear reasoning](https://verus-lang.github.io/verus/guide/nonlinear.html) and [Rust operator semantics](https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators).
- [F* termination](https://fstar-lang.org/tutorial/book/part1/part1_termination.html), [divergence effects](https://fstar-lang.org/tutorial/book/part4/part4_div.html) and [Low*/KaRaMeL compilation route](https://fstarlang.github.io/lowstar/html/Introduction.html).
- [Floating-point example](https://github.com/verus-lang/verus/blob/39095af5637daa994217da75b6c5ddacdbd0c520/examples/float.rs) and [conditional executable termination](https://github.com/verus-lang/verus/blob/39095af5637daa994217da75b6c5ddacdbd0c520/source/docs/guide/src/exec_termination.md).
- [Kani proof results and reachability](https://model-checking.github.io/kani/verification-results.html), [experimental function contracts](https://model-checking.github.io/kani/reference/experimental/contracts.html) and [loop contracts](https://model-checking.github.io/kani/reference/experimental/loop-contracts.html). Rolling documentation does not admit features into the repository's pinned profile.
