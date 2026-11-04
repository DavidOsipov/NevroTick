# Server catalogue and advisory statistics: continuation task

**4 November 2026 — discussion-stage handoff; implementation and measurements not run.** This task belongs to the [existing draft and §10 comparison card](../docs/plans/server-catalogue-and-statistics-draft.md#10-minimum-record-profile-and-bounded-comparison-card). It does not create a competing product queue or replace protected-storage/time-assurance tasks.

## 1. Scope and current disposition

Prepared: six minimum logical record families, separate catalogue/Q/H comparisons, reproducible input recipes, a literal independent-oracle seed and a bounded first pilot. No format, dependency, provider, trust policy, retention default or automatic server-exclusion rule is adopted.

The selected-document baseline is commit `d6bb2bdcd2145950586407698490b61d9e12ced9`, actual tree `3635c70a72cd32ca0a7f66cc545719c1aae46ece`. Its 28 locally used documentation files were matched to current Git blobs; this was not a full source checkout or target-code audit. Draft §§1–9 retain their original bytes and historical review scope. The fresh continuation review is recorded below when completed.

[Development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent) retains S157/S1.3 Core priority. The protected-storage pause and Windows/fuzz holds retain their stated scope; they do not create a blanket prohibition on separately admitted portable work. No target/native experiment is executed by this documentation change. Ordinary NTP/NTS quality data is in scope independently of RGT-0; conditional Roughtime metadata does not alter RGT-0 → CHR-0.

## 2. Ownership and authority

| Boundary | Owner and acceptance basis |
| --- | --- |
| Catalogue/schema, current admission and policy | Existing source/configuration owner; DATA-01–09 and OPEN-CAT-01/02/03. No roster or signature decision here. |
| Attempt facts, compatible aggregation and explanation | Engine/core contracts; DATA-10–20, OPEN-DATA-01/04. Freshness, operator dependence and actual correction effects remain explicit. |
| Advisory Q/H, publication, cleanup and migration | Storage owner; OPEN-DATA-02/03/06 and OPEN-ST-01/02. Existing mandatory stores and protected secrets retain independent authority. |
| Independent oracle and finite execution | QA/testing owner under [testing CONTRACT](../.agents/skills/testing/CONTRACT.md); §10.3 literal/analytical expectations, paired valid/refusal controls and §10.4 budget. |
| Conditional conflict/pin/bootstrap policy | Existing [TA-W02/06/08 owners](time-assurance-integration.md) and OPEN-PIN-01; no future safety record is silently adopted. |
| Native protected-storage implications | Existing [ST-W01 onward](windows-protected-storage.md). Reuse their frozen contracts where applicable; this task neither duplicates nor completes them. |

UI receives bounded projections; it does not open the database. Quality history cannot authenticate a server, authorize a correction, clear mandatory state or renew full NTS authentication age.

## 3. Next bounded preparation

The next development coordinator first reconciles this handoff with its actual checkout and active task. Prepare only the Q0-versus-Q-J no-raw comparison: freeze the concrete schema/presence rules and queries, independent literal/generated oracle, exact fixture hashes, two adapter acknowledgement/recovery contracts and configured runner. Bind commit plus dirty bytes, relevant dependency/features/tool versions, host/filesystem, effective settings and output limits before execution. Missing admission/tool/harness prerequisites are recorded as missing; no package acquisition or product implementation is implied.

The proposed pilot uses 200 ticks × 16 sources: **3,200 scheduled slots, 3,136 delivered outcomes, 2,816 successes**, plus the separate six-slot ledger. Use two 1,600-slot commits; verify update/read/restart, same-identity recovery, conflicting-content refusal and explicit unavailable raw replay. Total run budget is **15 minutes**, one worker, two adapter trials capped at 60 seconds each, 512-MiB process and 256-MiB output ceilings, no automatic reruns. Setup inside the run, controls, recovery and cleanup count toward the total; prerequisite development is reported separately. Stop on mismatch, bounds or non-equivalent durability.

Exit evidence: immutable fixture/oracle/configuration/adapter identities; actual selected/executed counts and outcomes; retained failures; logical and physical whole-family footprint; candidate/harness/combined memory; CPU/wall/I/O and acknowledgement costs; recovery correctness and limitations. The comparison must agree with independent expectations, not merely round-trip. No pass, compression ratio or winner exists yet.

Only a named remaining question justifies a larger profile or H/catalogue lane. The optional 150-minute envelope is a future ceiling, not required work or a gate for this draft. Consequential unresolved product choices use NV-DEC-001; this handoff invents no additional approval step for routine work within accepted contracts.

## 4. Actual evidence and remaining gaps

Two separate source/design helpers supplied the minimum record profile and comparison card; the coordinator reconciled them into §10. A third agent with fresh context reviews only the frozen integrated candidate and canonical requirements; helper proposals and author discussion are excluded from its inputs. A failed agent-start attempt is not a completed review.

A read-only availability check in this task's selected Linux x86_64 Python 3.12.14 interpreter reported SQLite 3.53.1 and `zstandard` 0.25.0; `pyarrow`, `cbor2` and `psutil` were absent there. No workload ran and no package was installed. This says nothing about other interpreters, Rust/Windows bindings, an admitted product graph or global tool availability.

Remaining: concrete fixture/schema/adapter artifacts, actual configured-runner admission and execution, resource measurements, broader cases justified by a result, and any later native/package/security qualification. No CHECK, TA-W or ST-W gate is closed. The existing documentation index already routes to the draft; §10 routes here.

## 5. Review and reconciliation record

The completed fresh review, exact candidate identities and coordinator disposition are appended here before publication. Its scope is readiness to publish this discussion-stage handoff, not adoption of the proposals or qualification of runtime behavior.

### 5.1 Coordinator disposition — 4 November 2026

Accepted the completed independent review's **READY** verdict for publication of the discussion-stage handoff. It raised no material NEXT-Rxx finding; no corrective change or additional review round was needed. The addendum remains exactly at its reviewed hash. The task's first 6,512 bytes remain exactly at its reviewed hash; only this review receipt is appended. The raw report below is preserved verbatim, including its two comprehension paragraphs, inspected scope and limits.

- Reviewer: separate `storage_continuation_review` agent, prepared with fresh context and neutral requirements; completed candidate review after preparation. The failed initial start and the preparation-only response are not passes.
- Raw report SHA-256: `93ab5966c456857eb1642c3b57390b0d8da2bb655c44a67d7df25c6977ee55c9`.
- Coordinator checks: compared the 28 selected baseline document blobs; only the planned draft changed. Preserved all 130,063 predecessor bytes. Checked all nine new local links/anchors and the written count/budget arithmetic. The complete remote tree contained no `docs/SHA256SUMS.txt`; no bundle-integrity claim follows.
- Public change allowlist: the appended draft and this task only. The existing documentation index remains unchanged and continues routing to the draft.
- Source-only completion: no benchmark, product code, dependency acquisition, live-server or Windows operation. Review and static arithmetic do not establish performance, security enforcement or qualification.
- Next action remains §3: prepare the concrete no-raw Q comparison inputs and adapters at the appropriate development checkpoint. The review does not adopt OPEN decisions or change active priorities/holds.

### 5.2 Verbatim independent review

<details>
<summary>Completed review, frozen identities and reasoning</summary>

# Independent continuation review — 4 November 2026

The addendum turns the existing inventory into a smaller description of the records needed to explain source observations and maintain advisory summaries. It then proposes a staged comparison: first compare a SQLite summary store with a complete JSON snapshot that answers the same questions, and investigate detailed history or catalogue encodings only when a specific remaining question warrants them. The synthetic inputs, expected counts, failure cases and finite run limits make that next preparation concrete. They do not select a product format, retention default, cryptographic construction or automated source-exclusion policy.

The inspected authorities keep source observations, statistical interpretation, current security policy and clock effects under different owners. S157/S1.3 remains the current Core development priority. Ordinary NTP/NTS catalogue and quality work can proceed independently of the later Roughtime stage; TA-W02 still owns the unresolved reserve/conflict contract. Mandatory policy, floors, correction accounting and lifecycle recovery remain separate from disposable history, as do the NTS/DNS secret stores and their root, data, privacy and authentication-age rules. Request cancellation cannot end actual native-resource custody. These are documentary contracts and recorded task dispositions, not results of a source or runtime audit in this review.

## Identity, context and completion

- **Reviewed stage:** publication of the discussion-stage continuation handoff, comprising the new draft §10 and its linked task.
- **Selected-document baseline:** DavidOsipov/NevroTick commit d6bb2bdcd2145950586407698490b61d9e12ced9; tree 3635c70a72cd32ca0a7f66cc545719c1aae46ece. The local material is a selected documentation snapshot, not a full checkout.
- **Context:** independent reviewer preparation used the neutral continuation packet and published requirements. The candidate review received the frozen integrated files and identities. No helper proposal, author discussion or private KB was consulted. Previously inspected unchanged requirements were reused.
- **Completion:** completed the requested candidate review. The earlier failed preparation attempt is not counted as a review, and the successful preparation-only response was not a candidate verdict.

| Frozen candidate | Bytes | Independently checked SHA-256 |
| --- | ---: | --- |
| docs/plans/server-catalogue-and-statistics-draft.md | 150150 | fee88fc2aef84dc6054ca5c11a2063990d9321a74257599db30c8720c2024633 |
| tasks/server-catalogue-and-statistics.md | 6512 | ac5ffe97c1897848562bedbc27fb6e195fc37fb68b650d569a18f849b54fb2fc |

Both file hashes matched the supplied frozen identities. The draft's first 130063 bytes independently reproduce baseline Git blob 3618297b71cb1dd5caf15aa4cbbd9081b8519dcb; the addition is 20087 bytes. This verifies preservation of the predecessor bytes without transferring its historical reviews to §10.

## Actual inspected scope

- Entire new §10.1–§10.6 and the complete continuation task; the published predecessor §§1–8 were inspected during preparation. The raw historical §9 review appendix was not used as a verdict on the addition.
- AGENTS.md; the documentation guide's plan/acceptance map, relevant baseline, authority/reading guide and integrity guidance; testing CONTRACT; planning and independent-review guidance; the review/handoff procedure; data-contract evolution and transition guidance; relevant implementation-contract sections.
- Owner decisions OD-02, OD-03 and OD-05; architecture §6, §7.4, §8.1, §10.2.1, §§12.1–12.1.2, §12.3, §§17–17.1 and §19.4.
- Development §7, §8's resource/environment contract, and relevant §8.4 scope, imported ST-W assignments and handoff requirements.
- Time-assurance plan §§3–7 and its assignment table, including TA-W02/06/08; protected-storage plan §§1–4 and §§7–8, task scope/ST-W01 and recorded coordinator dispositions.
- Primary documentation for the two material format/durability statements: SQLite's synchronous settings and Parquet's internal page compression.

Performed file-identity checks and small, exact arithmetic checks of the written recipe. No generated workload, database adapter, benchmark, target program, provider test, public-server probe or native experiment ran. No package was installed. Only this scratch review report was written. The task's interpreter-availability receipt was not rerun and supplies no product-admission evidence. The coordinator separately reported its 28-blob comparison and new-link checks; this review does not claim to have repeated them.

## Material findings

**No material NEXT-Rxx findings are raised for this publication stage.** No corrective change is required before completing the expected review bookkeeping.

### Data meaning and minimum coverage

The six families provide a coherent path from catalogue definitions and immutable context through controlled attempts, observations, compatible aggregates, assessments and attributable publication. They refine the existing DATA inventory rather than claiming to replace all 24 groups. The unchanged allocation continues to own active estimator/prior state, mandatory stores, secrets, status, publisher evidence and their different lifetimes.

The addition preserves zero versus absence, stage-specific populations, distinct metric counts, squared units for M2, the public phase sign, source/reference dependence, unknown independence and qualified time/generation meaning. Historical labels and verification summaries cannot construct live authentication or restore clock authority. Missing context is visible instead of being replaced with current definitions. Production widths and absolute date representation remain explicitly open; the next task requires a concrete schema before execution.

The existing 16 active-source and 64 regression-sample-per-source caps are retained. Retired context cardinality receives a separate bound requirement. Ordinary source-quality work does not become an RGT-0 prerequisite, and no cross-restart quarantine rule is silently adopted. Protected-storage exact-envelope identity, root/data/privacy generations, secret separation and independent authentication ages retain their existing owners.

### Deterministic recipe and independent expectations

The 50-tick cycle contains one gap, two timeouts, two verification rejections, one cancellation and 44 successes per source. Static arithmetic independently confirms:

| Profile | Scheduled slots | Delivered outcomes | Successful measurements | Received RTT values |
| --- | ---: | ---: | ---: | ---: |
| First pilot | 3200 | 3136 | 2816 | 2944 |
| Ordinary or context churn | 151200 | 148176 | 133056 | 139104 |
| Burst | 1382400 | 1354752 | 1216512 | 1271808 |

The separate six-slot ledger has the stated six slots, five outcomes, two successes, three RTT values and two offset values. Its supplied RTT values and offsets agree with the written formulas, including the zero offset at slot 248. The moment example has M2 = 800 ns² and sample variance = 400 ns². The large-integer control can distinguish loss through floating conversion. The churn count is also consistent: 9450/50 = 189 labels, yielding 3024 source/context pairs.

The burst generator fixes unsigned width, seed, update order and per-step truncation. The chronology rules explicitly prevent using hidden fixture elapsed time to manufacture freshness after the monotonic reset. Exact schema, presence encoding, ordering, numeric tolerance, partition expectations and fixture/oracle digests remain named preparation work. That is an honest boundary for this handoff; neither adapter agreement nor a shared codec round trip is accepted as the independent oracle.

### Fair comparison, failure semantics and resources

Q0 and Q-J have the same requested summary capabilities and both report raw replay unavailable. Q-J must pay for complete decode/rewrite/publication. Later detail comparisons freeze a common Q implementation and common detail semantics; compressed BLOB and JSONL lanes share input bytes, and H-B receives no separate free database allowance.

The candidate requires equivalent acknowledgement/recovery obligations before ranking and distinguishes committed, absent and uncertain unacknowledged outcomes. It retains batch identities through the pilot, refuses conflicting content under the same identity, and makes reconciliation outside a future receipt window explicit. External publication and Q commit remain separate. Optional loss cannot reset mandatory-state sentinels, and logical request retirement does not discharge real native custody.

The proposed SQLite settings are labelled experimental and must be read back. The distinction between FULL and NORMAL durability in WAL is supported by the [SQLite synchronous documentation](https://www.sqlite.org/pragma.html#pragma_synchronous); that documentation does not qualify the future adapters or filesystem. The internal-page compression description agrees with the [Parquet specification](https://parquet.apache.org/docs/file-format/data-pages/compression/). No implementation/provider graph is adopted by either statement.

Resource accounting includes complete file families, WAL/journals, staging, orphans, simultaneous migration copies, allocated bytes and candidate/harness memory separately and together. First-open reads are not called cold-cache measurements. Process interruption and injected failures are not promoted to power-loss, Windows or package evidence. The pilot cannot produce a statistically supported winner; the later 10% screen is expressly not a significance test.

### Budget arithmetic and current ownership

The first pilot's two 1600-slot batches cover its 3200 scheduled slots. Its two 60-second adapter caps fit within the 15-minute total, which also includes in-run preparation, controls, recovery and cleanup. The separate prerequisite-preparation accounting does not become benchmark time or a completed result.

The optional larger envelope is arithmetically consistent: 54 × 90 seconds + 18 × 10 seconds + (72 + 24) × 10 seconds = 6000 seconds, or 100 minutes. The 150-minute campaign ceiling therefore leaves at most 50 minutes for the other included work. These are ceilings, not a requirement to run every lane. The illustrative payload calculation is also correct: 148176 × 192 = 28449792 bytes = 27.1318359375 MiB, before the explicitly excluded overheads.

The task keeps current Core routing and existing TA-W/ST-W ownership intact, preserves the actual scopes of the storage/Windows/fuzz holds, and claims no execution or qualification gate completed. The review-receipt placeholder is expected completion bookkeeping. Concrete schemas, adapters, admitted runner inputs, full oracle artifacts and measurements remain prerequisites to a later run, not missing results that block publishing this proposal.

## Verdict and permitted next action

**READY — scoped to publication of this discussion-stage handoff at the frozen identities above.** This verdict adopts none of the proposed formats, numerical settings, dependencies or security policies and establishes no runtime or release qualification.

**One permitted next action:** complete the task's review receipt with this attributed result and the coordinator's disposition, then publish the documentary handoff within the existing authorization. Recheck only any subsequently changed material scope; preserve this review under its original candidate identities.

</details>

## 6. Independent proposal audit — 4 November 2026

### 6.1 Coordinator reconciliation and next preparation

David requested additional subagents with minimal context to audit the proposals themselves. Three new agents started without the conversation history, author/helper discussions or prior proposal verdicts. Their review packet contained exact excerpts of draft §§1–8/10 and task §§1–4, plus applicable canonical requirements. Draft §9 and this task's prior review section were excluded. Each agent froze its report before the coordinator read and reconciled the findings; they did not read one another's reports.

**Reviewed source:** commit `a80543024f0edb59a9fa9f90a372fef052f9e9de`, actual tree `6c61e24607fc76b7946589edf0b1f634cb8078bc`. The coordinator matched 35 selected source documents to that tree. The proposal extract's SHA-256 was `73f8ff0964ffd21bf8c0cb53d9a08c9c8e675af21a833e32a191d03bb5874742`; the task extract's was `3c74a64f16d32a75a42bc610f0264a2bf2c49790dd2376f3a15061c3e22795df`. This was a selected-document audit, not a code checkout or implementation audit. Initial agent-start failures supplied no reviews; all three subsequently completed their assigned independent reviews.

**Reconciled disposition:** continue the already identified bounded semantic/schema/adapter preparation. EXP-A01 is a confirmed limitation of the later capacity recipe and must be addressed before drawing capacity conclusions. The remaining medium items specify missing execution or decision evidence; EXP-A03 is an optional later refinement. None establishes an implemented vulnerability or mathematical defect. The earlier publication-readiness verdict remains historical evidence under its original identities and does not qualify these future experiments.

| ID | Classification and scope | Coordinator disposition / smallest next action |
| --- | --- | --- |
| DATA-A01 | Medium; initial fixture preparation | Accept. Freeze explicit stage truth, including before-send/after-send cancellation and unknown outcomes, with independently counted denominators. Terminal category totals alone are insufficient. Engine acquisition and QA own this. |
| DATA-A02 | Medium; initial semantic preparation | Accept. Freeze compatibility per metric, including a qualified RTT merge beside a phase-across-clock-step refusal and incompatible uncertainty meanings. Preserve provenance without automatically splitting every metric on every generation. Engine/core and QA own the card. |
| DATA-A03 | Medium; evidence before claiming Q is sufficient | Accept. Name the actual product/support questions, windows, populations and unavailable answers. Adapter equivalence establishes preservation of chosen information; it cannot establish that this information answers an untested temporal question. This does not select raw history or reopen deferred chart features. |
| EXP-A01 | Medium; confirmed later capacity-recipe limitation | Accept as open corrective work. Normal profiles remain below 1.5 million slots and never add a third unknown-age run; a seven-day population plus age jumps does not fill 180 days of summaries. Replace a relevant later trial with small cap/cap+1/third-run controls; use an independently seeded near-full summary state for retained-state cost. QA/storage own this before capacity interpretation. |
| EXP-A02 | Medium; initial measurement preparation | Accept. Freeze memory metric/process scope, simultaneous combined peak, logical versus storage-layer I/O, measurement phases, checkpoint mode, bounded wait and observable completion. Unsupported attribution remains unavailable. QA/storage own this. |
| EXP-A03 | Low; optional later cost interpretation | Retain for a later background-I/O/loss-window question. Declare publication cadence and retained-state size; a two-commit feasibility pilot does not predict arbitrary rewrite frequency. No cadence or loss tolerance is adopted. |
| SEC-A01 | Medium; initial recovery-contract preparation | Accept. Interrupt before graceful final drain and derive recovered state from actual persisted artifacts, including required WAL state. The independent expected ledger must not reconstruct the candidate. Bind Q-J aggregate/coverage/receipt content to the selected snapshot generation. Storage/QA own the paired recovery controls. |
| SEC-A02 | Medium; actual temporary-output/accounting boundary | Accept with scope. Declare actual temporary locations and observation of live peak allocations for the selected adapter; include spills and failed cleanup when those paths are exercised. Native creation-time rights remain with the platform owner. A synthetic no-spill run qualifies neither spill cost nor Windows access control. |
| SEC-A03 | Medium; conditional compressed-lane preparation | Accept at the compression trigger only. Freeze decoder mode and input/output/window/workspace/frame/work limits for the selected binding; pair excessive-window/output cases with valid neighbors. This does not block Q0/Q-J preparation. |

The coordinator checked the reported scenarios against the frozen proposal and confirmed the external SQLite WAL/checkpoint/temporary-file, Linux I/O-counter and Zstd streaming-window distinctions in primary documentation. The raw reports retain source links. The finite pilot remains 15 minutes with no automatic reruns; later corrective controls must replace relevant cases within the recorded budget, or be a separately justified bounded slice. No full-matrix expansion is requested.

**One next action in this workstream:** complete task §3's concrete preparation card with the stage ledger, per-metric compatibility and answerable-query contract, actual recovery artifacts/cuts, and measurement/checkpoint observer definitions. Keep EXP-A01/03 and SEC-A03 attached to their later triggers. This audit does not itself execute a workload, freeze a product default, adopt a dependency or change Core priority or the scoped execution holds.

This publication appends the coordinator disposition and the three verbatim reports to the existing task. The proposal and earlier task bytes remain unchanged. Findings remain open until the named preparation/correction supplies its evidence; recording a finding is not its resolution. No additional review round or target test was needed merely to preserve these completed reports.

### 6.2 Data and statistical meaning — verbatim independent report

Raw report SHA-256: `62921a6cc2f8c52c90897f82fc725e68feae7da818b5e42c8c5efb9e1a5d2dbd`.

<details>
<summary>Completed independent audit: Data and statistical meaning</summary>

The proposal separates a release catalogue, transient observations, advisory aggregates, optional detail and existing authoritative stores. Section 10 supplies six logical record families and a small comparison of SQLite summaries against JSON snapshots. Its useful idea is to preserve identities, context, denominators and missingness while avoiding routine persistence of every sample. Neither a storage format nor an automatic reputation policy has been adopted.

The supplied architecture keeps protocol admission, current source policy, pure estimation and acknowledged clock effects under separate owners. Historical success cannot recreate authentication, fresh agreement or correction authority. Clock/reference changes can invalidate comparisons; operator dependence and local network conditions constrain attribution. These are documented contracts, not evidence of implemented storage. Current Core priority and execution holds remain; this review concerns the proposed data semantics and comparison prerequisites.

## Scope, source and completion

**Completed independent document review.** Candidate: `a80543024f0edb59a9fa9f90a372fef052f9e9de`, tree `6c61e24607fc76b7946589edf0b1f634cb8078bc`, as identified by the supplied manifests. This is a selected-document packet; `git rev-parse` confirmed no local checkout. I read the audit/agent instructions, repository review/testing guidance, addendum, continuation task, DATA-03/04/08–12/14–20, draft §§1–2/4/5.1–5.4/6–7, relevant owner decisions, architecture §6's type/authority table, §§8.4–9/10.1, §12.1's separation/restoration rules, and feature F-09/§10. No excluded proposal/task verdict, author discussion or other audit report was consulted.

**Disposition: proceed with bounded semantic/schema preparation.** The comparison awaits the concrete contract already required by task §3. I found no confirmed mathematical or authority-separation defect. The three items below are unfilled execution or decision prerequisites, not reasons to settle every open product policy before continuing.

## Findings

### DATA-A01 — Terminal categories do not yet determine the fixture’s stage denominators

**Medium; execution prerequisite. Confidence: high**, based on the explicit difference between the minimum record and the supplied recipe.

**Evidence:** §10.1 requires “Distinguish scheduled/admitted/sent/received/validated/authenticated/eligible”; §10.3 specifies “r=5 cancellation.” DATA-10 separately requires each denominator to have a documented population. The recipe fixes outcome and metric counts, but does not fix the stage reached by cancellation, the stage of verification rejection, or whether every successful measurement is eligible/assessed.

**Counterexample:** cancel one admitted request before sending and another after sending. Both have the prescribed cancellation category and absent numeric values, but only the second belongs in a sent-request denominator. A telemetry gap may also leave the stage unknown; it cannot silently become an unsent request or a timeout. As a valid neighboring case, two requests cancelled before sending contribute two cancellation outcomes and zero sent requests.

**Consequence:** adapters can agree on the published totals and still disagree on availability or admissible-yield percentages. QA cannot independently derive stage truth from these terminal names. This is a fixture-contract gap, not evidence that the proposed record family lacks stage fields.

**Smallest correction / owner:** engine acquisition and QA owners freeze a literal stage/presence table for each fixture category, including one before-send cancellation, one after-send cancellation and one unknown outcome. Define whether selection coverage counts distinct observations or assessment episodes; repeated assessment must not create another observation.

**Independent check:** hand-derive each stage denominator and metric count for that tiny ledger, retaining the valid cancellation neighbor. An unknown suffix must produce incomplete coverage or an unavailable rate, never an invented exact denominator.

### DATA-A02 — Metric compatibility needs a concrete rule before footprint or merging results are meaningful

**Medium; execution prerequisite. Confidence: high** that the rule is still open; the failure scenarios are conditional, not observed defects.

**Evidence:** §10.1 says “Incompatible contexts split populations unless a qualified compatibility rule permits a merge”; OPEN-DATA-04 leaves aggregate segmentation open. The first pilot’s 200 ticks span only 12,736 seconds between first and last observation, so its ordinary recipe never changes its daily context label or catalogue revision. Section 10.3 names wrong-generation/incompatible-frame controls but has not supplied their literal meaning.

**Counterexample:** a confirmed local clock step changes Windows-relative offsets while the source remains correct. Combining those phase populations fabricates source instability. Conversely, splitting every metric on every applied-effect/catalogue revision can turn compact summaries into many tiny populations, even when a qualified monotonic RTT measurement and path remain unchanged. A valid neighbor is merging two disjoint RTT populations whose measurement definition, path and monotonic continuity are demonstrably compatible.

Uncertainty needs the same precision: a deterministic radius and a statistical standard deviation are both measured in nanoseconds, but averaging them together does not preserve meaning. DATA-11 recognizes this distinction; the concrete compatibility key must enforce it, including confidence level/assumptions where applicable.

**Smallest correction / owner:** engine/core owners define a short per-metric compatibility table: required context components, permitted transformations, uncertainty interpretation and unavailable cases. Retain all provenance without requiring every provenance change to split every metric.

**Independent check:** pair a phase-across-step refusal with the qualified RTT merge; pair different uncertainty meanings with a same-meaning merge. QA derives the expected partitions separately. Add these literal controls to the existing small pilot rather than increasing corpus size.

### DATA-A03 — Summary equivalence cannot establish that summaries answer the intended product question

**Medium; decision-evidence prerequisite. Confidence: high**, from an information-loss counterexample and the comparison’s stated queries.

**Evidence:** Q0’s §10.2 question is “Can dictionaries, aggregates, gaps and sparse events answer the required questions?” The §10.4 pilot reads latest source summaries and coverage. OPEN-DATA-02 explicitly leaves the actual user/support questions open; §5.3 acknowledges loss of sample order and later metrics. The proposal correctly limits the pilot to feasibility evidence.

**Counterexample:** within one bucket, latency sequences `[10,10,100,100]` and `[100,100,10,10]` have identical count 4, mean 55, M2 8,100 and extrema, with identical coverage times. Unless a retained event distinguishes them, Q cannot answer whether latency recently deteriorated or recovered. Either adapter can be perfectly correct. A valid neighboring question—“What was this bucket’s mean latency?”—is answerable exactly. This is not a demand to retain raw history for an unrequested trend feature.

**Consequence:** the pilot can compare persistence mechanisms for a chosen information set; it cannot by itself justify that information set or a source-reputation decision. Similarly, observed per-path success rates cannot establish provider fault or remove adaptive-selection bias. DATA-16/17 already state these limits; the synthetic equal-polling corpus supplies no contrary evidence.

**Smallest correction / owner:** the product/support and engine owners name a small set of actual questions, with windows, populations and permitted unavailable answers, before declaring Q sufficient. Keep format feasibility and product sufficiency as separate conclusions. Long-term charts remain deferred in feature §10.

**Independent check:** use the two equal-summary sequences to demonstrate the information boundary. A required order-sensitive question should trigger a specific additional statistic/event or justified detail retention; a bucket-summary question should pass without H.

## Limits and next action

The six-slot values, first-pilot counts and three-value moment example are arithmetically consistent by manual inspection. No workload, code, provider, native behavior or dependency was exercised. Source/path grouping, self-influenced references, unknown operator relationships and selected-only bias are already acknowledged; I do not relabel those honest open choices as defects. Observation count also does not establish an independent effective sample size or a clock-error bound.

**One bounded next action:** complete task §3’s semantic card for the two Q adapters: the stage ledger, per-metric compatibility rules and a few independently answerable product queries, with explicit unavailable cases. Freeze that card and its tiny counterexamples before preparing the configured run. This review adopts no format, retention policy or automated exclusion rule.

</details>

### 6.3 Experiment design and resources — verbatim independent report

Raw report SHA-256: `2cfbc5d6ca41368ac1ab1c3e897d94c5fed4f377e03ee42359f5028e55b299df`.

<details>
<summary>Completed independent audit: Experiment design and resources</summary>

The addendum separates immutable catalogue encoding, mutable advisory summaries (Q), and optional detailed history (H). Its first proposed experiment compares SQLite Q0 with a versioned JSON snapshot Q-J using two commits and no raw history. Larger workloads and four detail representations are conditional follow-ups. No format, retention default, dependency or performance result is selected; the small pilot is intended to establish feasibility and accounting, not a winner.

The supplied requirements keep advisory quality writes with the engine, expose bounded projections to the UI, and preserve separately owned mandatory clock/security state. The 16-source and 64-regression-sample limits do not bound historical contexts. Architecture §17 resource targets remain unmeasured product targets; a 512-MiB experimental process allowance does not qualify them. Current Core priority and the scoped storage, Windows and fuzz holds remain in force.

## Scope, source and completion

**Completed independent document review, 4 November 2026.** Reviewed source: `a80543024f0edb59a9fa9f90a372fef052f9e9de`, tree `6c61e24607fc76b7946589edf0b1f634cb8078bc`. The supplied proposal/task extract SHA-256 values match their manifest. This directory is a selected-document packet, not a Git checkout.

I read AUDIT/AGENTS, the documentation routing, repository independent-review/testing guidance, addendum §§10.1–10.6, next-task §§1–4, predecessor DATA-19/20 and §§5.2–5.4/7, owner-decision summary, and architecture §§17/17.1/20.1 and relevant CHECK entries. Context consisted of the assignment and supplied packet; no author/helper discussion, prior storage verdict or other agent report was used. Official SQLite and Linux documentation supplied the external facts below. No target code, workload, installation or native experiment ran.

**Disposition:** proceed with the named Q0/Q-J preparation. Execution awaits the already identified frozen schema/oracle, equivalent adapter recovery contracts and configured harness. Revise the later capacity recipe before using it to support capacity conclusions.

## Findings

### EXP-A01 — Later profiles do not reach the retention and capacity boundaries

**Medium; confirmed experiment-design limitation, future capacity scope. High confidence:** direct comparison of the specified fixture sizes and limits.

**Evidence:** §10.4 proposes “180 days daily,” a “1.5-million-slot ceiling,” and unknown-age detail retaining “at most two runs.” Section 10.3 supplies seven days of ordinary/churn activity and one day of burst activity; the largest corpus contains 1,382,400 slots. Churn has one restart, hence only two runs overall. Advancing fixture time by 8/181 days changes age, not the amount of populated historical state.

**Counterexample:** an implementation that never enforces the slot ceiling or evicts a third unknown-age run passes every normal profile because neither boundary is crossed. A JSON snapshot that works cheaply with seven populated daily periods may require substantially more rewrite memory/I/O at a full 180-day working set. An age jump that expires a small store does not reveal that cost. **Valid neighbor:** these recipes can legitimately check ordinary ingestion and known-age expiry; those narrower results remain useful.

**Consequence:** the advertised future capacity question can receive reassuring but non-discriminating evidence, particularly for snapshot rewrites, dictionary retention and physical slack. This does not invalidate the first smoke pilot.

**Smallest correction:** replace one relevant later trial with a small fixture whose declared count/run caps are actually crossed, and, only for retained-state cost, independently seed a near-full 180-day summary state and stream across expiry. Measure before and after the boundary; avoid generating 180 days of raw events merely to populate summaries.

**Owner/check:** QA plus storage, DATA-20/OPEN-DATA-03. Independently enumerate survivors at cap, cap+1 and third-run insertion; verify the accepted neighbor, explicit loss/refusal, dictionary reclamation and whole-family peak under the same remaining campaign budget.

### EXP-A02 — Measurement and checkpoint semantics need a concrete freeze

**Medium; execution prerequisite, not a demonstrated implementation defect. High confidence:** the current card names outputs but leaves these operational definitions open.

**Evidence:** §10.5 requests “bytes read/written,” candidate/harness memory “separately and combined,” and “explicit checkpoints every eight batches plus final drain.” Neither a memory/counter definition nor checkpoint mode/completion condition is specified. Next-task §3 correctly requires a configured runner; these details belong in that freeze.

**Counterexamples:** Linux `read_bytes` measures storage-layer reads, while `rchar` includes reads satisfied from page cache. A repeated scan can therefore report zero physical reads despite substantial decoding work. These are different useful measurements, not interchangeable “I/O” values ([kernel documentation](https://docs.kernel.org/filesystems/proc.html#proc-pid-io-display-the-io-accounting-fields)). Likewise, SQLite PASSIVE checkpointing need not finish with an old reader present; RESTART and TRUNCATE have different file-reuse/truncation behavior. Merely issuing a checkpoint is not proof of the intended final drain ([SQLite checkpoint contract](https://www.sqlite.org/pragma.html#pragma_wal_checkpoint)).

**Valid neighbor:** a fully accounted warm read or deliberately retained WAL is valid when its state and metric are declared. The defect would be interpreting it as zero query work or a common fully drained endpoint.

**Consequence/correction:** freeze the exact memory metric and process scope of the 512-MiB ceiling, observed simultaneous combined peak, I/O counters and cache labels, measurement phases, checkpoint mode/busy deadline and returned completion values. Keep total-process measurements authoritative where candidate/harness allocation attribution is unavailable; subtracting unrelated peaks cannot establish it. Include maintenance/recovery in elapsed and I/O accounting, as already required.

**Owner/check:** QA measurement owner with storage. Within the admitted harness controls, distinguish a warm read from no read. If readers can remain open, distinguish a blocked checkpoint from the successful released-reader neighbor; otherwise freeze that exclusion. Report unsupported attribution rather than invented precision. No additional product policy decision is required.

### EXP-A03 — Publication cadence should bound later performance interpretations

**Low; optional refinement before a format-cost decision. High confidence in the arithmetic; workload relevance remains an owner question.**

**Evidence:** §10.4 uses “two acknowledged batches of 1,600 scheduled slots,” then 4,096-slot batches. At 16 sources and 64-second ticks, the pilot batch represents 100 ticks, about 107 minutes of scheduled activity; a later ordinary batch represents about 4.55 hours. These are synthetic activity spans, not proposed benchmark wall times.

**Scenario:** very few full-snapshot publications can make Q-J inexpensive even though a future shorter loss window would require many more rewrites. Conversely, imposing frequent publication without a product need can unfairly penalize the simpler representation. **Valid neighbor:** two commits are adequate for the named initial update/restart/accounting question; the proposal correctly declines to name a winner.

**Smallest correction:** preserve that scope. If a later question is background I/O or acceptable loss, declare the tested acknowledgement cadence and retained-state size, and select one bounded contrasting cadence instead of automatically running the full matrix. Keep loss tolerance OPEN-ST-02 until its owner decides it.

**Owner/check:** storage/product owner with QA; analytically count publications and acknowledged prefixes, then compare equivalent recovery obligations and total rewrite/flush work for each declared cadence.

## What is already defensible; limitations

The six-slot ledger, separate metric counts, exact integer/rational expectations and duplicate/overlap refusals give QA a sound independent-oracle starting point. Separating Q-only from H comparisons and requiring one common Q implementation for H avoids a basic capability mismatch. The later invocation arithmetic is correct: 81 + 3 + 16 = 100 minutes inside a 150-minute total cap. No extra repetitions or matrix expansion are justified by this review.

Concrete adapter behavior and measured effects remain unknown. In particular, a process restart cannot establish power-loss durability; the proposal already states this limitation and prohibits a durability-equivalent ranking without matching demonstrated semantics. Synthetic compression results and Python/Arrow costs remain limited to their fixture/runtime. This review establishes no CHECK-21, native, security or product-budget pass.

## One bounded next action

Have the existing QA/storage owners complete the current task’s Q0/Q-J preparation card with EXP-A02’s measurement/checkpoint definitions and freeze its literal/generated oracle and two-commit recovery contract; retain EXP-A01/03 against only the later questions they affect. Do not expand or execute the campaign on this review’s authority.

</details>

### 6.4 Security and recovery — verbatim independent report

Raw report SHA-256: `2b1fcf1d180647fdb9ef77a1ca7cf97243b32aeef4c1d5e7d8ddf362e58c4e2f`.

<details>
<summary>Completed independent audit: Security and recovery</summary>

# Independent security and recovery audit

The proposal separates an immutable release catalogue (C), mutable advisory quality summaries (Q), and optional detailed history (H). Its six minimum record families preserve identities, context, coverage and interpretation without requiring six physical tables. The immediate continuation is to prepare a small SQLite-versus-JSON-snapshot comparison with the same Q behavior; detailed archives, catalogue encodings, encryption and pins remain conditional. Neither a format nor a security policy is adopted by this comparison.

The canonical system assigns network acquisition to an engine without clock privilege and keeps policy, trusted bounds, correction accounting and lifecycle recovery with their existing authorities. Complete hardened MSIX is the selected preparation target, with native enforcement and removal recovery still requiring qualification. NTS/DNS secrets have a separate protected-storage contract. Advisory records cannot construct authenticated samples, restore pending exchanges, refund correction authority, clear mandatory restrictions or renew full NTS authentication age. This packet contains design documents, not an implementation of these stores.

## Scope, source, context and completion

**Completed:** independent source/design review, 4 November 2026. Source identity is `DavidOsipov/NevroTick` commit `a80543024f0edb59a9fa9f90a372fef052f9e9de`, tree `6c61e24607fc76b7946589edf0b1f634cb8078bc`, as identified by the supplied manifests. The proposal/task extract SHA-256 values match `extract-manifest.json`; this directory is not a Git checkout.

Inspected: `AUDIT.md`, `AGENTS.md`, documentation guide, repository independent-review/security/testing guidance; the full §10 addendum and task §§1–4; proposal §§1–2, DATA-01–10/18–24 and §§5–7; architecture §§6, 7.4, 8.1, 10.2.1, 12.1–12.1.2 and acceptance catalogue; protected-storage §§4–5/7; selected MSIX integrity/lifecycle contracts and current OD-05. No earlier proposal verdict, author/helper discussion, another agent's report or private KB was consulted. Primary SQLite, Microsoft and Zstd documentation was checked. No target code, workload, provider, Windows or network experiment ran; no dependencies were acquired.

**Disposition:** proceed with the already named bounded preparation. No confirmed design defect was established in this lens. Execution awaits the concrete adapter/harness contracts already required by task §3. The three items below make those prerequisites discriminating; they are not findings that the proposal has implemented an insecure behavior.

## Findings and execution prerequisites

### SEC-A01 — Recovery must be observed before final drain and reconstructed from durable Q state

**Classification/severity:** underspecified execution prerequisite; Medium consequence if omitted. **Confidence:** high in the counterexample, supported by SQLite's WAL contract; no adapter exists in the reviewed packet.

**Evidence:** §10.4 requires “retaining their exact commit receipts throughout the pilot”; §10.5 schedules “checkpoints every eight batches plus final drain” and says every acknowledged batch survives the declared restart model. Task §3 correctly requires acknowledgement/recovery contracts, but their concrete artifacts remain absent.

**Counterexample:** in the two-batch pilot, acknowledged changes can reside in WAL before any scheduled checkpoint. A harness that copies only `quality.sqlite`, validates only its retained receipts, or closes/drains SQLite before every restart can miss loss at the actual crash boundary. For Q-J, separately publishing aggregates and deduplication receipts can similarly leave either double credit or a receipt for missing data. SQLite documents that separating a database from its WAL can lose committed transactions or corrupt the database: [WAL §4](https://www.sqlite.org/wal.html#the_wal_file).

**Valid neighbor:** after an interruption before drain, restart from the actual store family; the acknowledged first batch remains exactly once, and an unacknowledged second batch is absent or reconciled once. Same-ID/different-content replay fails without changing the healthy state.

**Smallest correction/check:** freeze the restart model, authority-bearing receipt location and cut points in the existing preparation. Keep the independent expected ledger outside the candidate, but derive the candidate's recovered state solely from its persisted artifacts. Put Q-J aggregates/coverage/receipts in one selected snapshot generation; qualify selection/publication explicitly. Observe a lost acknowledgement and one interrupted recovery. **Owner:** storage and QA; DATA-19/20, OPEN-ST-02, CHECK-15/25. This is an execution prerequisite, not a demand to solve Windows power loss during the portable pilot.

### SEC-A02 — File-backed temporaries need concrete location and peak-accounting coverage

**Classification/severity:** underspecified adapter/measurement prerequisite; Medium. **Confidence:** high that directory-only accounting is insufficient; actual placement and access remain unknown.

**Evidence:** §10.5 selects “file-backed temporaries” and requires complete-family footprint; §5.6 requires protection of temporary files “from creation onward.” These are sound obligations, but the proposed settings alone do not establish them.

**Counterexample:** a sort, materialization or migration spills outside the Q directory. Measuring only that directory, or only files remaining after cleanup, undercounts peak allocation and can incorrectly report compliance with the output ceiling. With real local-source metadata, an inadequately protected temporary directory also breaks confidentiality despite a correct ACL on the main database. SQLite's documented Windows search can use the directory returned by `GetTempPath`, and temporary-file behavior is implementation-dependent: [temporary-file locations, §5](https://www.sqlite.org/tempfiles.html).

**Valid neighbor:** a forced spill remains in an admitted private location, is included while live, and disappears through attributable cleanup; a non-spilling query produces the same logical answer.

**Smallest correction/check:** before pilot execution, bind the actual SQLite build/VFS and temporary-file placement to an observer that includes transient peak allocation, failed cleanup and migration copies. State whether the ceiling covers all such output. Before native use, verify actual creation-time rights under the final service identity, including another service sharing its account. Do not infer those rights from a PRAGMA or from the synthetic pilot. **Owner:** storage/QA, then platform security; DATA-20/23, OPEN-ST-01, CHECK-15/17/21. No new encryption decision is required to close this prerequisite.

### SEC-A03 — The decoded-batch limit needs a separate decoder-workspace rule

**Classification/severity:** prerequisite for a later compressed lane, not the first Q0/Q-J pilot; Medium. **Confidence:** high for streaming Zstd's distinction; the selected API/build is still unknown.

**Evidence:** §10.5 says “cap decoded batches at 8 MiB”; §§5.4–5.5 separately require bounded codec workspace and work even for authenticated content. Those broader requirements must become executable limits rather than only measured costs.

**Counterexample:** a bounded frame advertises a history window larger than the admitted workspace while producing less than 8 MiB of output. Checking output length alone does not impose that workspace bound. Upstream explicitly provides `ZSTD_d_windowLogMax` for streaming allocation control and distinguishes single-pass decoding: [Zstd API](https://github.com/meta/zstd/blob/dev/lib/zstd.h).

**Valid neighbor:** the same useful payload with an admitted window and framing decodes successfully within the independent memory budget.

**Smallest correction/check:** when a compressed lane is justified, freeze decoder mode, compressed/output/window/workspace limits and concatenated-frame policy for the actual binding. Pair excessive-window and excessive-output inputs with valid neighbors; observe refusal before oversized allocation and retain exact library identity. An authentic catalogue or BLOB receives no exemption. **Owner:** storage/parser and QA; DATA-20, OPEN-ST-01, CHECK-25/27. The existing proposal already states the governing invariant; this item supplies a counterexample the expansion-only control would miss.

## Boundaries that survived review

Sections 5.5–5.7 correctly distinguish binary encoding, release authentication, currentness and truth. An independently trusted executable digest is a plausible release-only binding; a writable sibling checksum is not. The same captured bytes must reach admission. Explicit MSIX integrity matches [Microsoft's documentation](https://learn.microsoft.com/en-us/windows/msix/desktop/tamper-protection), while whole-release rollback remains OPEN-CAT-03. SQLite integrity checks/encryption are not claimed to authenticate observations or defeat rollback. Reset/migration preserve mandatory owners, and conditional NTS pins retain PKIX, intended KE identity, same-path matching and current-generation checks. No additional signature, encryption or pinning requirement follows from this review.

## Limits and one next action

This review establishes no implementation vulnerability, native ACL result, decoder qualification, durable-write guarantee or benchmark winner. Loss tolerance, production budgets, catalogue freshness and pin policy remain their named owner decisions.

**Next action:** the existing storage/QA coordinator should freeze the Q0/Q-J adapter and observer contract with SEC-A01/02's paired controls in task §3; keep SEC-A03 attached to the later compressed-lane trigger. That is preparation within the current task, not authorization to run an experiment or adopt a format.

</details>
