# Verus assessment for NevroTick

**Date:** 1 November 2026

**Disposition:** DEFER Verus integration on the assessed evidence.

**Evidence level:** source and documentation review; no Verus or application Kani proof executed.

**Task record:** [S1.1c assessment and review results](../../tasks/s1.1-time-domain.md#verus-proposal-assessment--1-november-2026).

**5 November update:** the earlier snapshot and DEFER conclusion below remain
historical. A [bounded Core addition pilot](../../tasks/s1.3-source-selection-readiness.md#verus-addition-pilot--5-november-2026)
is now justified by a named full-domain gap, an actual Kani timeout and David's
conditional instruction to use Verus if it is better. Broader Verus adoption
remains DEFER; no Verus payload or product proof has been admitted/executed yet.

## Conclusion and practical effect

The supplied proposal gives a sound method for evaluating Verus. It identifies
useful numerical properties, acknowledges existing tests, and requires proof of
the implementation actually used by the product. Two independent read-only
reviews supported those principles and deferring integration.

The assessment did not demonstrate enough additional assurance or lower
maintenance cost to justify adoption. The strongest initial candidate, NTP era
resolution, has a loop-free body over finite machine inputs. Kani could
potentially prove the same full contract. That alternative remains an unexecuted
hypothesis: compatibility and solver cost have not been measured.

Continue the existing development plan. This assessment creates no new phase
blocker, scheduled proof experiment or mandatory CI gate. No production defect
was established, so no product-code repair was justified. The assessment added
reusable [formal-verification guidance](../../.agents/skills/testing/references/formal-verification.md)
to the existing Rust-contract and static-check workflows.

This report summarizes the research and decision. The task owns assessment
history; the [architecture](../windows-time-architecture.md) owns product
contracts, and [TOOLING.md](../../TOOLING.md#additional-rust-analysis-tools)
owns installed-tool status and execution entry points.

## What was examined

The assessment inspected the working checkout at HEAD
`df6923e25bc557637c300dffb335a705d74111a0`, including current source and analysis
policy. HEAD alone did not identify concurrent dirty changes; unrelated S1.3
work was excluded from writable scope. The selected
[era implementation](../../crates/time-domain/src/era.rs) had SHA-256
`366397df0c6bcd260d566b5bac310c882796734887af886a5fdf99a023bb1dc8`.

Upstream Verus research used revision
`39095af5637daa994217da75b6c5ddacdbd0c520`. Its
[toolchain](https://github.com/verus-lang/verus/blob/39095af5637daa994217da75b6c5ddacdbd0c520/rust-toolchain.toml)
matches the project's Rust 1.99.1. This removes one possible version mismatch;
it establishes no complete Cargo, macro, library, feature or target compatibility.
No Verus binary or dependency graph was admitted or installed.

The [analysis policy](../../policy/analysis-tools.json) selects Kani 0.68.0 on
its separate nightly, with unwind 1, Cadical, sanity checks and 60 seconds per
harness. The [guarded checker](../../scripts/check-analysis.py) exercises healthy
and defective standalone conversion controls. At assessment time, no application
Kani harness was found under `crates/` or `tools/`. Installed controls therefore
establish no formal proof of the era implementation.

## Candidate properties and existing evidence

### Preferred candidate: complete NTP era classification

For the actual `unfold_ntp(seconds, fraction, window)`, define mathematical integers:

```text
r = seconds * 2^32 + fraction
E = 2^64
C = { r + k*E | k is an integer and L <= r + k*E <= U }
```

The domain is arbitrary raw `u32` parts and a valid ordered inclusive window
`[L,U]`, whose endpoints lie in `[-2^95, 2^95-1]`.

A useful complete proof would establish:

- Empty `C` produces `NoEraCandidate`.
- A singleton `C` produces `Ok` with that exact coordinate.
- Multiple candidates produce `AmbiguousEra`.
- Success preserves the raw 64 bits modulo `E` and stays within the window.
- On the admitted domain, no panic or arithmetic-overflow rejection occurs.

These are proposed obligations, not completed proof results. `EraWindow::new`
owns endpoint validation and ordering. A helper proof assuming valid windows
would leave constructor correctness as a separate dependency. Numerical era
resolution also establishes no trust in the supplied time anchor.

The [existing era tests](../../crates/time-domain/tests/era.rs) already include
2,112 independent candidate-enumeration combinations, negative eras, extreme
coordinates, inclusive edges, absent/ambiguous outcomes and construction checks.
They provide meaningful bounded evidence. A potential formal proof must add a
precisely stated guarantee beyond those observations.

### Reserve candidates

| Subject | Useful guarantee and integration boundary |
| --- | --- |
| [Nearest-even rounding](../../crates/time-domain/src/rounding.rs) | For `D` in `{2^32, 2^33}`, exact reconstruction `numerator * 10^9 = q*D + residual`, residual in `[-D/2, D/2]`, even signed-i64 `q` at exact ties, and correct success/error classification. Existing public-path tests cover 8,193 numerators plus boundary cases. Distinguish the full private `i128` domain from producer inputs bounded by `2^97-2`; their scaling fits `i128`, while signed-i64 output admissibility remains separate. |
| [Datagram validator and field iterator](../../crates/ntp-wire/src/datagram.rs) | Successful validation must establish the real iterator's suffix invariant, exact ordered field boundaries, progress and stable exhaustion. This plausible compositional proof requires materially broader work with slices, borrowing and iterator state. It should not automatically replace an inconvenient first candidate. |

Do not expose private APIs or verify a separately rewritten implementation just
to make a demonstration easier. Select one subject for any future pilot.

## Confirmed capabilities and consequential caveats

| Point | Assessment and primary basis |
| --- | --- |
| Windows support | Upstream [installation guidance](https://github.com/verus-lang/verus/blob/39095af5637daa994217da75b6c5ddacdbd0c520/INSTALL.md) supports Windows x64. Windows itself is not a valid rejection reason. Portable arithmetic proof still qualifies no Windows API, privilege, IPC, service, clock or package behavior. |
| Cargo integration | The [pinned Cargo guide](https://github.com/verus-lang/verus/blob/39095af5637daa994217da75b6c5ddacdbd0c520/source/docs/guide/src/cargo_verus.md) describes verification of opted-in crates. Ordinary Cargo builds run no Verus proof; focused verification skips dependency proof rechecks. Inspect actual verified scope and relevant dependency proof identities. |
| Executable-code linkage | [Ghost erasure](https://verus-lang.github.io/verus/guide/erasure.html) removes proof/spec code. Using `cfg(verus_only)` to select different executable behavior can break the proof-to-product link. Erasure alone establishes no performance parity after an integration refactor. |
| Trusted assumptions | [Trusted-component guidance](https://verus-lang.github.io/verus/guide/tcb.html) identifies assumptions, axioms and external specifications. Assuming the central property in a wrapper does not prove the underlying body. |
| Unverified callers | [Caller-boundary guidance](https://verus-lang.github.io/verus/guide/call-from-unverified-code.html) requires preconditions and relevant trait/destructor obligations to hold. Proof preconditions are not runtime guards; preserve checked admission and safe encapsulation. Existing private checked types may already establish an invariant; proof annotations do not require revalidating unchanged values at every call. |
| Floating point | The [pinned example](https://github.com/verus-lang/verus/blob/39095af5637daa994217da75b6c5ddacdbd0c520/examples/float.rs) demonstrates float types and discusses project axioms. Mathematical-real reasoning does not by itself prove binary64 rounding, NaN behavior or platform assumptions. |
| Checked arithmetic | [Pinned numeric specifications](https://github.com/verus-lang/verus/blob/39095af5637daa994217da75b6c5ddacdbd0c520/source/vstd/std_specs/num.rs) cover checked integer operations. Declaring checked arithmetic unsupported would be unjustified; the actual crate's complete operation/macro/callee graph remains unverified. |
| Termination | Verus [executable termination checking](https://github.com/verus-lang/verus/blob/39095af5637daa994217da75b6c5ddacdbd0c520/source/docs/guide/src/exec_termination.md) is conditional on callee termination. An invariant or local `decreases` annotation alone proves no complete end-to-end termination guarantee. |
| Kani comparison | Kani's [result semantics](https://model-checking.github.io/kani/verification-results.html) require attention to reachability and vacuity. Its experimental [function contracts](https://model-checking.github.io/kani/reference/experimental/contracts.html) support checking callers with verified callee contracts, while [loop contracts](https://model-checking.github.io/kani/reference/experimental/loop-contracts.html) extend beyond simple unwinding. Composition alone is no unique Verus advantage. These capabilities are not automatically admitted by NevroTick's pinned profile. A loop-free body can still call code containing loops. |

## When to reconsider and how to decide

Reconsider one bounded Verus pilot when a named valuable contract and a justified,
testable hypothesis suggest better coverage, composition, compatibility, proof
authoring/review effort, maintenance or recurring verification cost. Define the
observations that would support or reject that hypothesis. A relevant Kani
limitation is one possible trigger, not a prerequisite. The pilot can measure a
suspected benefit even when both tools might prove the same property; adoption
still requires demonstrated value. The present DEFER decision remains unchanged.

Classify a non-passing Kani attempt before treating it as a reason to compare tools:

| Observed cause | Appropriate response |
| --- | --- |
| Real implementation/contract counterexample on admitted inputs | Preserve and replay the witness where supported; reconcile implementation and requirements. Changing tools must not hide the discrepancy through weaker assumptions or scope. |
| Incorrect harness/specification or vacuous reachability | Correct the verification setup while preserving independently established requirements; inspect insufficient unwinding separately. |
| Exhausted resource budget | Record incomplete proof and consider another formulation or tool within explicit limits. Finite inputs do not guarantee cheap verification. |
| Unsupported source/API or installation/setup failure | Diagnose the compatibility/environment gap and compare integration options; establish no product defect or alternative tool advantage from the failure alone. |

For the small era function, Kani remains a reasonable first attempt when numerical
core work next warrants it. No practical cost or maintenance comparison has yet
been performed, and this clarification schedules no immediate pilot.

A future trial should use the existing task and
[shared procedure](../../.agents/skills/testing/references/formal-verification.md):

1. Freeze one independently specified contract, actual source/callers and input
   domain. Record integration time, solver timeout, memory, attempts and workers
   before acquisition or expensive execution.
2. Establish executable-body linkage and list project-added assumptions, external
   specifications, unverified dependencies and constructor/environment premises.
3. Require a healthy pass and two or three compiled, non-equivalent defects to
   fail their intended obligations, followed by a restored healthy pass. For era
   resolution, wrong signed boundaries, swapped classifications and always-error
   behavior are useful candidates with admitted witnesses.
4. Report nonempty verified scope, failures/timeouts/skips, preparation and solver
   costs separately, and one maintenance exercise. Compare equivalent domains
   and contracts; unexecuted alternatives remain hypotheses.
5. Choose `ADOPT_LIMITED` only after demonstrating useful additional assurance or
   acceptable maintenance savings on actual product code. Otherwise retain a
   specific deferral or record rejection of that integration approach.

This report adds no proof execution or performance result to the preceding
assessment. Existing behavioral, fuzz, memory, mutation and native qualification
requirements retain their own scope.

## Reconsideration triggered — 5 November 2026

The 1 November assertion that application Kani had not run is a dated snapshot.
Since then the source-bound rounding/era and named Core/domain helper checkpoints
have qualified their declared scopes. The actual `add_fractions` full checked-pair
proof stopped after 60,025 ms with zero checks; complement's preceding five-job
group qualified, while addition remains open. This is a resource-limit result,
not a mathematical counterexample or evidence that Verus will succeed.

Separate read-only source/specification, readiness and plan reviews support one
comparison: verify the actual constructor, comparison, complement and addition
bodies through deductive contracts and integer/power lemmas. Keep required
success, exact carry/residual, the complete domain, existing semantic fault
witnesses and restored health. Reject model-only substitution, product
`external_body`, assumed central properties and verification-only executable
changes. Upstream standard-library specifications and verifier/compiler/solver
trust must remain explicit.

The [Core task card](../../tasks/s1.3-source-selection-readiness.md#verus-addition-pilot--5-november-2026)
owns the precise source, candidate and budgets; the [development plan](../plans/development-plan.md#7-next-executable-task-for-a-development-agent)
and [backlog](../../tasks/development-backlog.md#p1-core-completion) route it.
The candidate is official release `0.2026.11.02.8bf68ee` at
`8bf68ee741df833ea5d0fe6a2255126fdea87e47`, requiring the already installed
Rust 1.99.1. The 36,022,359-byte Linux asset has an upstream SHA256 digest;
unpacked size, loader behavior and unchanged-API compatibility remain unmeasured.
The pinned launcher supports a no-rustup mode, allowing isolated execution
without shared toolchain/configuration changes. Source inspection alone does
not admit that runtime.

`checked_shl` is an unresolved first compatibility gate: not finding a model in
inspected files is not proof of unsupported behavior. Test the actual API using
the pinned candidate. The finite card permits no SDK/source build, verifier
patch, unsupported-operation rewrite or assumed product contract as a workaround.
Keep all completed Kani proofs and the failed addition receipt unchanged.

The new disposition is **DEFER broad adoption; bounded pilot queued**. Only a
complete equivalent-contract result, intended failures, restored health and
acceptable measured integration/maintenance cost justify `ADOPT_LIMITED`.
No installation, Verus proof, new CI requirement or release/native approval is
established by this addendum.

Current primary sources:

- [Pinned release and Linux asset metadata](https://github.com/verus-lang/verus/releases/tag/release/0.2026.11.02.8bf68ee).
- [Pinned installation guidance](https://github.com/verus-lang/verus/blob/8bf68ee741df833ea5d0fe6a2255126fdea87e47/INSTALL.md)
  and [toolchain](https://github.com/verus-lang/verus/blob/8bf68ee741df833ea5d0fe6a2255126fdea87e47/rust-toolchain.toml).
- [No-rustup launcher and bundled-solver selection](https://github.com/verus-lang/verus/blob/8bf68ee741df833ea5d0fe6a2255126fdea87e47/source/verus/src/main.rs).
- [Thread, SMT-option and no-cheating controls](https://github.com/verus-lang/verus/blob/8bf68ee741df833ea5d0fe6a2255126fdea87e47/source/rust_verify/src/config.rs),
  [solver option handling](https://github.com/verus-lang/verus/blob/8bf68ee741df833ea5d0fe6a2255126fdea87e47/source/air/src/context.rs)
  and [Z3 timeout units](https://microsoft.github.io/z3guide/programming/Parameters/).
