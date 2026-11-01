# Verus assessment for NevroTick

**Date:** 1 November 2026

**Disposition:** DEFER Verus integration on the assessed evidence.

**Evidence level:** source and documentation review; no Verus or application Kani proof executed.

**Task record:** [S1.1c assessment and review results](../../tasks/s1.1-time-domain.md#verus-proposal-assessment--1-november-2026).

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
