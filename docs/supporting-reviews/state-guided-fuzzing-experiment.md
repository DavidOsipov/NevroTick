# State-guided fuzzing: fault-suite and mutation experiment

**Date:** 1 November 2026, Asia/Tbilisi

**Disposition:** experiment complete; retain strict oracle and baseline mutations.

**Scope:** CHECK-19/25 qualification of an inert exchange-association fixture.

**Execution record:** [S1.1e experiment and receipts](../../tasks/s1.1-deterministic-harness.md#bounded-fault-suite-and-mutation-follow-up-1-november-2026).

## Decision and future use

Keep lifecycle mutation available for targeted cancellation testing. It greatly
reduced the calls needed to expose the selected stale-cancellation defect, but
made context-invalidation search slower. Keep the effects oracle explicitly
selected: it lets exploration continue past checked, state-equivalent error-order
differences while strict comparison remains the normal contract check.

The experiment is closed. Product development can continue under the
[existing execution plan](../plans/development-plan.md). Use this report when
choosing a future trace corpus, mutation strategy or oracle objective. Reopen
experimentation for a concrete uncovered state transition, changed input model or
observed search failure; the present results do not call for another backend or
an open-ended fuzzing campaign.

The [task record](../../tasks/s1.1-deterministic-harness.md) owns detailed execution
history and receipts. This report preserves the decision and reusable findings;
the [state-guided fuzzing skill](../../.agents/skills/testing/qa-state-guided-fuzzing/SKILL.md)
and its [backend procedure](../../.agents/skills/testing/qa-state-guided-fuzzing/references/backend-workflow.md)
own the reusable workflow. Product contracts remain in the canonical architecture.

## Question and experimental design

Would a lifecycle-aware mutation operator improve detection beyond the earlier
single-origin-guard experiment, and could an effects oracle distinguish harmful
acceptance/state changes from differences in rejection order?

The standalone [engine](../../tools/state-guidance/engine/src/main.rs) used
LibAFL/bolts 0.16.1 and Rust 1.99.1 on `x86_64-unknown-linux-gnu` under the existing
offline namespace/resource supervisor. LLVM PC guards instrumented only
`ntp-exchange`, `ntp-wire` and `time-domain`. The separately authored scalar model
and driver called the real public association API; model and engine code were
outside the coverage instrumenter. This was an x64 GNU/WSL analysis experiment,
with no CPU-policy migration or production network/clock effects.

| Dimension | Frozen choice |
| --- | --- |
| Target builds | One healthy build and four private builds, each containing one intentional defect; canonical product source unchanged. |
| Input | Versioned action trace; at most 32 actions and 65 bytes. |
| Training corpus | Three traces: create a pending request, complete a request, reject a wrong socket and then complete correctly. |
| Feedback | Semantic feedback plus real SUT edge coverage, held fixed across mutation strategies. |
| Baseline mutations | Existing bounded command mutation recipe. |
| Lifecycle mutations | 25% append a six-action succession sequence after a bounded random prefix; 75% use baseline mutations. No model queries during mutation. |
| Oracles | Strict and effects, selected separately. |
| RNG blocks | 20 paired seeds: `1, 7, 42, 2, 3, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19`. |
| Budget | 1,024 target callbacks per campaign, including three initial seeds; stop at the first selected oracle failure. Two workers. |
| Matrix | 320 defect campaigns plus 80 healthy campaigns: four defects × two mutations × two oracles × 20 seeds, plus healthy controls. |

The lifecycle suffix cancels current work, creates and completes one request,
creates its successor, attempts stale cancellation and receives the successor's
reply. Its selection probability is fixed; it is deliberately relevant to the
cancellation contract. A strong result on that defect is therefore evidence for
targeted search, not a general-purpose improvement.

Every training input passed every build under both oracles. The earlier default
corpus's stale-cancellation trace was excluded because it already exposes the new
cancellation defect at initialization. Training input seeds and RNG seeds are
different dimensions. The standard supervisor retained its four-input corpus;
the private comparison used the common three-input corpus above.

## Controls and oracle meanings

Each private defect compiled, failed a deterministic intended witness and passed
legal neighboring controls through the public target API. These are deliberately
seeded defects for measuring sensitivity, not newly discovered production bugs.

| Private change | Deterministic witness, hexadecimal | Required effects failure |
| --- | --- | --- |
| Omit origin-token rejection | `0100f002000100` | Forbidden acceptance. |
| Stale cancellation clears pending work | `0100f0010000f105000100` | Lifecycle-state corruption. |
| Accept a receipt exactly at its deadline | `0100000128` | Forbidden acceptance. |
| Advance context without clearing pending work | `0100f00800` | Lifecycle-state corruption. |

The **strict oracle** rejects every returned-category, state or completion-fact
disagreement. The **effects oracle** may continue a category difference only when
actual pending identity, next identity, full context and completion effects still
agree with the independent model. It retains the difference as a diagnostic.
Hidden pending token/timing fields have no public getter; completion facts and
later actions challenge them. Neither oracle claims immediate visibility into
every internal field.

An independent control, `0100f00201`, showed why the distinction matters: the
origin-defective build returned `ProcessingBeforeArrival` where the strict model
expected `OriginMismatch`, while preserving the pending request and producing no
completion. Strict comparison failed; effects comparison continued and recorded
the discrepancy. A one-call campaign verified that its bytes and diagnostics
were saved. Known witnesses were kept outside the training corpus.

## Results

All **400 campaigns** completed, executing **152,019 campaign callbacks**. All
**81,920 healthy callbacks passed**, with 441,004 completed actions. The 320 defect
campaigns produced **296 detections and 24 searches censored at the budget**.
Every detected witness reproduced on its exact defective build and passed on the
healthy build from fresh state.

Each row below compares baseline → lifecycle over 20 paired trials. Undetected
trials contribute the 1,024-call cap to the mean. The endpoint is first strict
disagreement or first effects failure, as named in the row.

| Defect | Oracle | Detections / 20 | Mean capped calls | Lifecycle earlier / later / tied |
| --- | --- | --- | --- | --- |
| Origin rejection omitted | strict | 20 → 20 | 230.80 → 131.35 | 10 / 10 / 0 |
| Origin rejection omitted | effects | 20 → 20 | 257.05 → 156.95 | 8 / 11 / 1 |
| Stale cancellation clears pending work | strict | 20 → 20 | 98.05 → 6.10 | 18 / 1 / 1 |
| Stale cancellation clears pending work | effects | 20 → 20 | 98.05 → 6.10 | 18 / 1 / 1 |
| Inclusive deadline acceptance | strict | 13 → 15 | 648.15 → 538.45 | 12 / 7 / 1 |
| Inclusive deadline acceptance | effects | 13 → 15 | 648.15 → 538.45 | 12 / 7 / 1 |
| Context advance retains pending work | strict | 20 → 20 | 34.10 → 39.55 | 8 / 12 / 0 |
| Context advance retains pending work | effects | 20 → 20 | 34.10 → 39.55 | 8 / 12 / 0 |

Cancellation shows the clearest improvement. Deadline detection improved in this
sample, but misses remained. Context search regressed. For origin rejection, the
improved means conceal mixed paired outcomes and worse medians: strict
96.5 → 114.5 calls, effects 96.5 → 132.0. These findings support targeted use of
the optional operator and retention of the baseline default.

The 40 strict origin detections included 35 forbidden `Time` completions and five
later timing rejections. Those five establish rejection-order differences, not
observed response acceptance. Across all effects campaigns, 68 failures were
`ForbiddenAcceptance` and 80 were `LifecycleStateCorruption`. Five campaigns
recorded 17 nonfatal category differences in total; each campaign's first
diagnostic witness was retained and replayed against both builds.

Controls and replays were separate from the campaign budget. Including them,
the study executed 152,711 target callbacks across 1,072 native commands. Its
approximately 74 seconds of supervised wall time excludes builds and standard
supervisor qualification; it is an execution receipt, not a product benchmark.

## Lessons to retain

1. **Qualify the oracle before measuring search.** A reachable deterministic
   witness distinguishes a missed defect from an oracle that cannot detect it.
   Build/setup failures establish no behavioral detection.
2. **Keep training inputs neutral across selected defects.** A useful old corpus
   can make a new comparison trivial by triggering the defect during startup.
3. **Separate rejection order from harmful effects.** Keep strict checking;
   identify an additional effects objective explicitly and retain both diagnoses.
4. **Save diagnostics independently of corpus admission.** A counted discrepancy
   can otherwise disappear when its input adds no coverage. Preserve the first
   diagnostic and first fatal input separately and replay identical bytes.
5. **Report each fault class, misses and paired outcomes.** A pooled score or
   improved mean can conceal slower typical detection and regressions elsewhere.
6. **Stop when the bounded question is answered.** Carry useful checks into the
   existing workflow and return to implementation; adding tools is not itself
   product progress.

The selected defects are not a representative mutation sample. The two oracle
lanes reuse seeds and target builds, so their matching results are not independent
replications. Different trace lengths and instrumentation layouts across builds
prevent equal-work or CPU claims. Successful-trace action totals exclude failing
prefixes. No actual missing/corrupt-completion defect variant was tested; classifier
seam tests provide narrower evidence. No statistical significance, complete
state-space coverage, sanitizer result, NTS security, native Windows behavior or
release qualification follows from this experiment.

## Evidence and reuse

The frozen candidate passed 18 harness tests, seven instrumented engine tests and
Clippy with warnings denied. The standard strict/baseline lane was then qualified
with another 6,144 healthy evaluations and six reproduced origin-fault witnesses.
These were checks performed for the experiment; this report creates no new runtime
qualification. Independent engine/recipe review and artifact reconciliation found
no remaining blockers. That reviewer authored the effects harness, so the review
does not constitute independent review of its own harness authorship.

Machine-local evidence is under `~/.local/share/nevrotick/` on the execution host:

| Record | Relative location | SHA-256 |
| --- | --- | --- |
| Study receipt | `state-guidance-fault-study-20261101-v1/verification.json` | `f410a41269444c7f76b162e99e1cd3423b878228a50c05975b01d3d84eb7cdc5` |
| Frozen study design | `state-guidance-fault-study-20261101-v1/design.json` | `30f1ee24e7673d8784eed3c1f06e94e51347d874fe607706d3cf279ddf0282ee` |
| Study recipe | `state-guidance-fault-study-20261101-v1/recipe.py` | `6e68f3825e4a9a94771634d592a20ca5cb0fdf71fb6f4d5e189f57c1d4b9d577` |
| Installation manifest | `state-guidance-20261031-rf4kkgu0/fault-suite-installation/installation-manifest.json` | `dbcb80b43f9fd781532c9e6fdd9d605645440c6d6f4f90e3aa006df7a6cbdb5f` |
| Default-lane comparison | `state-guidance-comparison-20261101-fault-suite/verification.json` | `2b263ccc0bce31d19242e77de668db6406109d1ffe8e3c40a70648a1900d2779` |

The study receipt binds its design, recipe, controls and per-campaign results.
The manifest binds source, executables and runtime dependencies. The execution
task retains earlier incomplete attempts and their corrections. These local
artifacts are not distributed by this Markdown report; another machine needs
those files or newly qualified evidence to reproduce the result.

Before reusing the installed lane, follow the
[admission procedure](../../TOOLING.md#fixture-semantic-state-guidance) and
[rebuild guidance](../../.agents/skills/testing/qa-state-guided-fuzzing/references/backend-workflow.md#rebuild-and-re-admit-changed-inputs).
Changed source, input domain, oracle, toolchain or dependencies require matching
admission and controls. The historical results remain useful for choosing an
approach, but do not qualify a changed candidate.
