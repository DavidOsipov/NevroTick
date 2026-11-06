# Verus assessment for NevroTick

**Current owner amendment — 6 November 2026:** [OD-10 §14.1](../nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026)
defers formal-proof work until after the first released software version (V1).
This assessment and the experiments below are retained history/post-V1 backlog,
not first-version gates or authority for more verifier/source/metadata work now.
No proof is waived permanently or promoted beyond its actual scope; ordinary
product qualification and every existing execution hold remain.

**Date:** 1 November 2026

**Disposition:** DEFER Verus integration on the assessed evidence.

**Evidence level:** source and documentation review; no Verus or application Kani proof executed.

**Task record:** [S1.1c assessment and review results](../../tasks/s1.1-time-domain.md#verus-proposal-assessment--1-november-2026).

**5 November update:** the earlier snapshot and DEFER conclusion below remain
historical. The justified [bounded Core addition pilot](../../tasks/s1.3-source-selection-readiness.md#verus-addition-pilot--5-november-2026)
has now reached a readiness **DEFER**: the acquired pinned payload passed its
isolated version/loader check, but rejected the unchanged `checked_shl` API before
SMT execution. No product-proof command ran and no adoption follows. The
[executed result below](#bounded-readiness-result--5-november-2026) supersedes the
initial queued-pilot status; the full-domain addition obligation stays open.

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

This subsection records the pre-execution rationale and initial card. The
[readiness result below](#bounded-readiness-result--5-november-2026) supersedes its
queued status and unmeasured-runtime statements.

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

## Bounded readiness result — 5 November 2026

**Disposition: DEFER this candidate for the unchanged addition call graph.** The
official `0.2026.11.02.8bf68ee` payload was acquired and admitted for isolated
compatibility work; its version/loader command passed. The first healthy-control
command stopped on a harness expression's ghost/machine-integer type mismatch.
One causal harness repair used the third and final preparation command, combining
the two healthy controls with a separate, deliberately false assertion.

That final native command exited 1 with no runtime-custody failure. Verus reported
`encountered-vir-error: true` and an unsupported `u64::checked_shl` call at the
actual executable expression on line 35 of the frozen compatibility input. The
observed executable trace contains only the fixed environment helper, Verus
launcher and `rust_verify`; Z3 was not spawned. Translation stopped before SMT
execution, so neither healthy verification nor assertion sensitivity qualified.
All three preparation commands are consumed; zero product-proof commands ran.
The 42 passing Python controls cover acquisition and result interpretation, not
Verus compatibility or product mathematics.

The retained final receipt is
`~/.local/share/nevrotick/formal-verus-pilot-20261105-v1/compatibility-final.json`,
SHA256 `06c7b07021921461b8cbfba25520176491fbf8d064e6299e38baaddc124cd2a8`.
Its status is `DEFER`, `adopted` is false and both `product_proofs` and
`proof_commands_executed` are zero. This is an API-compatibility result for the
pinned candidate, not a product arithmetic defect or a claim that every Verus
version or approach is unsuitable. Other API compatibility remains unqualified.

**Return trigger:** an official candidate with source-supported specifications
for the same unchanged constructor/comparison/complement/addition call graph,
followed by a separate finite compatibility/proof card. Add no assumed project
specifications, runtime rewrites or verifier patches to bypass this result; do
not run a fourth preparation command under the exhausted card. Full equivalent
domain, actual-source correspondence, semantic fault controls and measured
maintenance/cost remain prerequisites for any `ADOPT_LIMITED` conclusion.

Completed Kani proofs and the preserved addition timeout remain valid within
their unchanged bindings. The existing Core task owns the independent split
fallback and the still-open addition/composition obligations; mandatory proof
requirements and Windows, fuzz, storage and release holds remain unchanged.

A later [ordinary-package Kani contract-mode pilot](../../tasks/s1.3-source-selection-readiness.md#addition-contract-mode-terminal-assessment--6-november-2026)
also ended DEFER at its first primitive control: CBMC aborted on an internal
closure-symbol type inconsistency with zero checks. No product proof or callee
certificate qualified. That distinct compatibility failure lifts no Verus hold
and supplies no basis to patch a verifier or weaken the full-domain obligation.


## Later source-only hybrid assessment — 6 November 2026

The [query-split terminal record](../../tasks/s1.3-source-selection-readiness.md#fraction-query-split-terminal-assessment--6-november-2026)
adds a changed premise: fresh actual-body Kani Post/return-seed proofs exist for
construction,comparison and complement, while the actual addition caller still
timed out. Source-only inspection finds a plausible caller-only deductive route
because checked_shl resides in those callees and pinned vstd supplies relevant
Result/Option/control-flow/comparison specifications. No exact caller
compatibility or Kani-to-Verus contract equivalence has been executed/proved.

The old `--no-cheating` launch rejects external_body/assume_specification;
silently removing it or routing assumed contracts around it is not admitted.
A separately reviewed exact contract-import policy and machine-checked domain/
Post/type/frame correspondence are required before caller adoption. Kani seed
inclusion alone does not certify a new Verus predicate. Source review supports
only bridge-first preparation under a later finite card, with wrong-contract
controls and the actual caller/original obligation union preserved. The central
addition theorem cannot be imported. No new run, flag, installation,SDK patch or
adoption follows; the old unchanged-callgraph pilot remains terminal DEFER.

The existing Core task now owns the
[source-only bridge candidate](../../tasks/s1.3-source-selection-readiness.md#fraction-verus-bridge-source-preparation--6-november-2026).
It preserves six executable predicates and the nine product-source spans, with
reversible declaration annotations and separately frozen mathematical contracts.
Source checks establish extraction discipline only. Body-to-spec proofs, primitive
and dyadic correspondence, semantic controls and caller-import admission remain
open; the existing no-cheating profile is unchanged.

The subsequent [pure-bridge attempt](../../tasks/s1.3-source-selection-readiness.md#fraction-verus-bridge-terminal-assessment--6-november-2026)
is terminal DEFER after one healthy dispatch. Seven private ghost functions
were incorrectly marked `open`, so VIR stopped before SMT and no Z3 ran. This is
a preparation defect, not an arithmetic/backend incompatibility finding. The
separate source-only revision removes exactly seven modifiers, preserves private
types and all bodies, and passes fourteen source checks. The subsequent
[private-spec v2 card](../../tasks/s1.3-source-selection-readiness.md#private-spec-verus-bridge-v2-terminal-assessment--6-november-2026)
observed VIR compatibility and Z3 execution, then closed DEFER with five failed
equalities. Constructor Post's modular success depends on failed helpers; the
other three successes are Clone bodies. Zero jobs qualified, and controls/
restoration did not run. The eighteen-check diagnostic-consumer source repair
and reviewed bit-length lemma were separate preparation at that checkpoint;
no verifier adoption or full bridge followed.

The subsequent [dependency insertion preparation](../../tasks/s1.3-source-selection-readiness.md#bit-length-dependency-insertion-preparation--6-november-2026)
binds the lemma and two full-raw predicates, with exact ghost-block/signature
erasure. Its initial41 focused source/consumer checks were preparation only.
The subsequent
[bit-length dependency campaign](../../tasks/s1.3-source-selection-readiness.md#bit-length-dependency-terminal-assessment--6-november-2026)
completed all five jobs: the full-raw lemma and two predicate equalities,
three intended contract defects and restored health. All62 focused Python checks
pass;22 source files and final source/input/receipt/tool bookends are bound.
This is scoped mathematical dependency evidence, with global adoption false and
zero caller imports. Remaining arithmetic/native Ordering correspondence and the
complete bridge/addition gates stay open; earlier attempts remain closed.

The subsequent [constructor-Post campaign](../../tasks/s1.3-source-selection-readiness.md#constructor-post-terminal-assessment--6-november-2026)
completed all five jobs, re-verifying the lemma/two helpers with the full-raw Post
equality and detecting three isolated Post defects before restored health.
All93 focused Python checks pass, with24 source files and final custody/tool
bookends bound. This adds one predicate correspondence, not a new execution
theorem for `Fraction::new`. Existing generator functions and both older Rust
outputs remain exact; old proofs retain their frozen candidates. No certificate
is imported and global adoption remains false. The subsequent
[complement v2 qualification](../../tasks/s1.3-source-selection-readiness.md#complement-v2-terminal-qualification--6-november-2026)
passed seven jobs, establishing four predicate equalities and three arithmetic
lemmas with five contract defects/restoration. All143 selected Python checks pass;
30 source files/11 executed-Python rows and final guards are bound. The first
attempt's opaque-power preprocessing failure stays closed; no solver ran there.
Subsequent [rational-order qualification](../../tasks/s1.3-source-selection-readiness.md#rational-order-terminal-qualification--6-november-2026)
passed nineteen jobs with fifteen named functions: five full-raw predicate
equalities/seven total lemmas, seventeen intended defects and restored health.
All161 selected checks pass;33 sources/11 executed-Python records and final guards
bind the candidate. Source review confirms missing pinned checked-shift support;
the native Ordering equality/comparison-Post route, complete bridge,
caller/addition and Core/P1 remain open. Ordinary Kani independently completed
[RequestTiming admission](../../tasks/s1.1-exchange-association.md#request-timing-terminal-qualification--6-november-2026):
five full raw-pair/three-defect/restored jobs,158 Python checks and34-input/
nine-executed-Python guards. This actual-source constructor/getter result imports
no certificate and supplies no missing Verus model. The later [actual
append_depth proof](../../tasks/s1.3-source-selection-readiness.md#selection-append-depth-terminal-qualification--6-november-2026)
passed six full raw-state/four-defect/restored Kani jobs,671 checks and208 Python
checks with34 bound inputs/nine executed-Python records. This proves local state/
frame only; prefix/depth/topology/whole selection remain open. Subsequent
[actual lifecycle ordinal qualification](../../tasks/s1.3-source-selection-readiness.md#lifecycle-ordinal-terminal-qualification--6-november-2026)
passed five full raw-u64/three-defect/restored ordinary Kani jobs,263 checks and248
Python checks, with38 inputs/nine executed-Python records and complete23-enum
closure. Actual successor/exhaustion mathematics imports no certificate and
supplies no missing Verus model. Subsequent [actual source-counter
qualification](../../tasks/s1.3-source-selection-readiness.md#lifecycle-source-counter-terminal-qualification--6-november-2026)
passed five full raw2^80/three-defect/restored ordinary Kani jobs,145 properties
and291 Python checks, binding32 inputs/nine executed-Python records and actual
constants/From/min. The later [constructor source checkpoint](../../tasks/s1.3-source-selection-readiness.md#source-state-constructor-source-checkpoint--6-november-2026)
prepares real Garde/callback/constructor/Cargo source, authenticates14 selected
vendor packages and passes51 source checks plus independent review. No compiler/
proof or cross-verifier certificate runs; source-specific admission/target/
allocator/drop/unwinding and constructor theorem remain open. Existing Era model/
private-patch holds remain. Transactional lifecycle/global uniqueness and every fraction model/
bridge/caller hold remain open; no old Era/compiler campaign is restarted.
