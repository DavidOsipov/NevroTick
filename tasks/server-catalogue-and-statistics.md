# Server catalogue and advisory statistics: continuation task

**4 November 2026 — discussion-stage handoff; implementation and measurements not run.** This task belongs to the [existing draft and §10 comparison card](../docs/plans/server-catalogue-and-statistics-draft.md#10-minimum-record-profile-and-bounded-comparison-card), the [storage tables](../docs/plans/server-catalogue-storage-tables.md) and the later [binary catalogue refinement](../docs/plans/server-catalogue-binary-format.md). It does not create a competing product queue or replace protected-storage/time-assurance tasks.

## 1. Scope and current disposition

Prepared: six minimum logical record families, separate catalogue/Q/H comparisons, reproducible input recipes, a literal independent-oracle seed and a bounded first pilot. The later owner direction prefers binary catalogue shipment and requires Roughtime key/profile support in the first schema; §7 records that refinement and future publisher tooling. No final encoding, dependency, provider roster, trust policy, retention default or automatic server-exclusion rule is adopted.

The original continuation baseline was commit `d6bb2bdcd2145950586407698490b61d9e12ced9`, actual tree `3635c70a72cd32ca0a7f66cc545719c1aae46ece`. Its 28 locally used documentation files were matched to Git blobs; this was not a full source checkout or target-code audit. That checkpoint preserved draft §§1–9; the later binary refinement updates current catalogue passages while preserving the historical raw reports and their candidate scope. The completed continuation review is retained below and does not review the later refinement.

[Development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent) retains S157/S1.3 Core priority. The protected-storage pause and Windows/fuzz holds retain their stated scope; they do not create a blanket prohibition on separately admitted portable work. No target/native experiment is executed by this documentation change. Ordinary NTP/NTS catalogue and quality data are in scope independently of RGT-0; requiring Roughtime schema fields now does not alter RGT-0 → CHR-0.

## 2. Ownership and authority

| Boundary | Owner and acceptance basis |
| --- | --- |
| Catalogue/schema, current admission and policy | Existing source/configuration owner; DATA-01–09 and OPEN-CAT-01/02/03. No roster or signature decision here. |
| Publisher collection, semantic review and deterministic build | Existing catalogue/release owner; R, DATA-01/24 and the binary refinement. Future Python tooling under `tools/` emits candidates or approved build artifacts; collection supplies no approval, signing or publication authority. |
| Attempt facts, compatible aggregation and explanation | Engine/core contracts; DATA-10–20, OPEN-DATA-01/04. Freshness, operator dependence and actual correction effects remain explicit. |
| Advisory Q/H, publication, cleanup and migration | Storage owner; OPEN-DATA-02/03/06 and OPEN-ST-01/02. Existing mandatory stores and protected secrets retain independent authority. |
| Independent oracle and finite execution | QA/testing owner under [testing CONTRACT](../.agents/skills/testing/CONTRACT.md); §10.3 literal/analytical expectations, paired valid/refusal controls and §10.4 budget. |
| Conditional conflict/pin/bootstrap policy | Existing [TA-W02/06/08 owners](time-assurance-integration.md) and OPEN-PIN-01; no future safety record is silently adopted. |
| Native protected-storage implications | Existing [ST-W01 onward](windows-protected-storage.md). Reuse their frozen contracts where applicable; this task neither duplicates nor completes them. |

UI receives bounded projections; it does not open the database. Quality history cannot authenticate a server, authorize a correction, clear mandatory state or renew full NTS authentication age.

## 3. Next bounded preparation

This section retains the Q0/Q-J statistical-store preparation. The source-only
catalogue/schema continuation is recorded separately in §7 within this same task;
it neither executes nor silently renames the pilot below.

The next development coordinator first reconciles this handoff with its actual checkout and active task. Prepare only the Q0-versus-Q-J no-raw comparison: freeze the concrete schema/presence rules and queries, independent literal/generated oracle, exact fixture hashes, two adapter acknowledgement/recovery contracts and configured runner. Bind commit plus dirty bytes, relevant dependency/features/tool versions, host/filesystem, effective settings and output limits before execution. Missing admission/tool/harness prerequisites are recorded as missing; no package acquisition or product implementation is implied.

The proposed pilot uses 200 ticks × 16 sources: **3,200 scheduled slots, 3,136 delivered outcomes, 2,816 successes**, plus the separate six-slot ledger. Use two 1,600-slot commits; verify update/read/restart, same-identity recovery, conflicting-content refusal and explicit unavailable raw replay. Total run budget is **15 minutes**, one worker, two adapter trials capped at 60 seconds each, 512-MiB process and 256-MiB output ceilings, no automatic reruns. Setup inside the run, controls, recovery and cleanup count toward the total; prerequisite development is reported separately. Stop on mismatch, bounds or non-equivalent durability.

Exit evidence: immutable fixture/oracle/configuration/adapter identities; actual selected/executed counts and outcomes; retained failures; logical and physical whole-family footprint; candidate/harness/combined memory; CPU/wall/I/O and acknowledgement costs; recovery correctness and limitations. The comparison must agree with independent expectations, not merely round-trip. No pass, compression ratio or winner exists yet.

Only a named remaining question justifies a larger profile or H/catalogue lane. The optional 150-minute envelope is a future ceiling, not required work or a gate for this draft. Consequential unresolved product choices use NV-DEC-001; this handoff invents no additional approval step for routine work within accepted contracts.

## 4. Actual evidence and remaining gaps

At the original continuation checkpoint, two source/design helpers supplied the minimum record profile and comparison card; the coordinator reconciled them into §10. A third agent with fresh context reviewed the frozen integrated candidate and canonical requirements, excluding helper proposals and author discussion. Its receipt and the later proposal audit remain below under their original identities; they do not qualify the current binary refinement. A failed agent-start attempt is not a completed review.

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


## 7. Binary catalogue and future collector refinement — 4 November 2026

**Documentation baseline:** `7550f83292f959a6d0acbb2d396e964a6bcf4b01`;
selected-file snapshot, not a local source checkout. The
[binary catalogue design](../docs/plans/server-catalogue-binary-format.md)
records the owner's request for a small, secure native Windows utility with a
release catalogue for hundreds of NTP/pool, NTS and Roughtime records. Binary
shipment is preferred over JSON; constrained CBOR and generated immutable tables
remain candidates. Roughtime public-key algorithm/bytes, exact profile,
source/endpoint scope and provenance are required in the first schema. Runtime
Roughtime implementation, provider admission and RGT-C01 remain future work.

The future collector is planned at `tools/server-catalogue/build_catalogue.py`,
reusing the existing lowercase `tools/` directory. It is publisher-side R work:
bounded source acquisition/parsing, normalized candidates, provenance and
semantic diffs, then deterministic construction from reviewed approved inputs.
Collection is separate from approval, signing and publication. Neither Python nor
raw source caches become installed client requirements. No tool file, dependency,
probe, benchmark or automatic updater is implemented by this handoff.

**Leap-policy integration, 4 November 2026:** [NV-LP-001](../docs/plans/leap-second-policy.md)
and [S1.4](s1.4-leap-policy.md) own the separate immutable compiled public leap
dataset, its release-bound revision/digest and effective-rule provenance; this
does not choose C's final binary format. DATA-02/03/09 retain source-scale
provenance separately from announcement behavior: all-LI=0 unsmeared peers remain
ordinary candidates, while contradictory observations follow NV-LP-001 exclusion
and fresh re-entry. Unknown future coverage does not globally stop tracking or
independent bootstrap acquisition. The future publisher collector preserves
claims and evidence; it grants no runtime fetch, probe, leap-schedule or clock
authority. Carry these cases into the existing schema/TA/S1.4 owners below; the
completed review receipts in §7.1–7.3 retain their original scope and contents.

| Preparation / existing owner | Required documentary exit and retained gap |
| --- | --- |
| Catalogue/schema and QA — DATA-01–09; OPEN-CAT-01 | Freeze stable-ID/reference/normalization rules, presence, exact profile/key bindings, aggregate limits, generated-data alternative and old/new reader behavior. Distinguish recognized inert Roughtime metadata from enabled/qualified runtime; explicitly identify any reviewed compatible projection. Freeze concrete accepted/refused fixtures before implementation; no final encoding or parser graph is selected now. |
| Source/default and time assurance — OPEN-CAT-02/03; R-04; OD-03; TA-W06 | Retain key publication/profile evidence, unknowns, retirement/overlap, operator dependence and Pool-use constraints. Freeze catalogue versus reviewed preset, configured and active membership, the preset's C/policy binding, and whether automatic replacement is in scope. If selected, replacement needs an eligible population, trigger, stable reselection, finite traversal and continuing provider budgets. No default or traversal permission is adopted. The 13-key documentary table is narrower than the dated endpoint census; neither is a shipped roster or live interoperability pass. |
| Release and lifecycle — OPEN-CAT-01/03; existing W11/ST-W owners | Preserve exact-artifact trust, optional separate-signature decision, unreliable-clock freshness/rollback and protected package/state replacement. No catalogue repair may reset mandatory policy or conflict state. Actual native/package observations remain open. |
| Publisher collection/build — R; DATA-01/24 | Specify bounded adapters, source-failure/conflict handling, exact approved normalized-snapshot identity, deterministic outputs and semantic review. Changed parsing/normalization/schema inputs invalidate approval; build does not silently refetch or substitute records after review. Later implementation follows the existing tools area and the admitted tool/fixture scope. |
| Statistics/storage — Q/H; existing DATA/EXP/SEC findings | Keep Q0/Q-J preparation, query/compatibility truth, recovery and measurement definitions unchanged in scope. The earlier audit findings remain open at their named triggers; binary C preference does not select a Q/H encoding or execute a pilot. |

**Documentation outcome:** the requested five-role review and integration are
complete for this discussion stage. The dispositions, retained cases and original
reports below preserve their distinct reviewed candidates. The final check
verified owner routing, canonical OPEN/CHECK/TA/ST identities, report preservation,
links and documentary key/source data. It establishes no format winner, measured
resource result or implementation qualification.

**Next continuation:** prepare the relevant schema/fixture, preset/policy and
compatibility contracts under the owners above before dependent catalogue/tool
implementation. OMI-R01's routing repair is complete; its product choice remains
open under OPEN-CAT-02/R-04/OD-03. OMI-R02–08 refine already-owned future controls,
not additional documentation-publication blockers. Core priority, the separately
scoped Sandbox disposition, blocked artifacts/runners and the protected-storage
pause retain their current scope.

### 7.1 Completed documentation review and dispositions

The initial proposal was frozen at SHA-256
`b8977ea8c3e2fa1f90511f59f613511983d03a615888a361bbd9c1d1d15adced`.
Role 3's corrected proposal was
`77e906705fced8667bd6bcc7f0a70b4c17352e4a1bd78f22438adb1eacf2c4a4`;
role 5 inspected that complete document, the actual four reports and the changed
canonical passages before the remaining integration. The final review records
the later identities. These are selected documentation candidates, not full
implementation qualification.

The coordinator records initial context-length launch failures for roles 1–5;
those attempts produced no usable review and count as **not run**. The retained
reports are the completed resumed assignments. Roles 1 and 3 disclose bounded
cross-document coordination; role 5 deliberately consumes all preceding results.
The read-only mechanical link/table helper supports role 5 and is not a sixth
review role. Neither the number of reviewers nor agreement among them multiplies
executed evidence.

| Role / completed scope | Original report SHA-256 | Disposition for this milestone |
| --- | --- | --- |
| 1 — documentation consistency | `36270ccd6922b098fc3187a93ac3379afd03936e2f458bbc0d0f76924a50c31d` | Seven existing documents updated; current requirements and historical evidence separated. Final integration checked the changed passages. |
| 2 — fresh format and location comparison | `4b4014e977e86ac9878466e6b248b954a743c264d0cda42707089ead6a36cb81` | Its original REVISE applies to the frozen initial proposal. R2-F01/02 are corrected below; conditional representation/location countercases remain. |
| 3 — schema review and corrections | `44ae69c5a9d8c51fad28c2125251131fa3929d5def78286c50f9505572d6ad1f` | Corrected the logical schema, reference/profile/key boundaries, producer/reader transition obligations and canonical routing. Implementation freezes remain open. |
| 4 — omitted scenarios and unresolved decisions | `19f247c2e7d6bdba6bf4ff36d13587d3b6e788fc19514cf9ad3ce3236b775477` | OMI-R01 routing corrected; OMI-R02–08 retained with existing owners and triggers in §7.2. |
| 5 — final integration and documentation-publication review | `fe5583016f8e8b8377d52aeedd465782c5d7075746cbbadf0524b0ca8f38d0b8` | Accepted for the requested documentation stage after the recorded corrections/checks. Remote commit/ref verification is a separate publication receipt, not a product release result. |

| Stable findings / requirement | Verified disposition in the integrated documentation | Retained owner or invalidation trigger |
| --- | --- | --- |
| R1-01; R2-R01/02 | Binary C preference is consistent; CBOR is conditional on a justified separate resource, generated immutable tables remain a leading release-bound alternative, and encoding/placement/runtime access are separate choices. | OPEN-CAT-01; no measured winner. Reopen if later requirements, corpus or complete-product costs change. |
| R1-02; R3-F06 | DATA-05 and the first schema require complete Roughtime key/profile material. Recognized inert metadata and a reviewed compatible export are distinct from runtime capability; ordinary NTP/NTS preparation remains independent of RGT-0. | DATA-05; OPEN-CAT-01/02; TA-W02/03/06. Exact early-reader/export behavior remains to be frozen. |
| R1-03; R2-F01; R3-F07 | OPEN-CAT-01 retains representation/signature/trust-root strategy, 02 roster/profile/cold resolution, and 03 freshness/withdrawal/rollback. Existing IDs were not reassigned. | Reopen if those routing meanings drift; no signature or roster decision is closed here. |
| R1-04; R2-F02; R3-F01 | Required country/region-or-unknown, provenance/scope and time-scale/leap/smear compatibility are explicit in the refined schema. | DATA-03/05; all candidate encodings must retain equal semantics. |
| R3-F02/03/04 | Stable source/endpoint/key/binding identities, typed variants, reference integrity, normalization, exact scoped key authorization and distinct lifecycle states are explicit; no implicit cross-product or profile inference. | DATA-01–09; OPEN-CAT-01–03. Concrete encodings, bounds, transitions and actual verifier admission remain future. |
| R1-05; R3-F05 | The existing task/development chain owns future `tools/server-catalogue/build_catalogue.py`; exact reviewed snapshots, bounded adapters, conflicts/partial results and safe deterministic generation are specified. | R/DATA-01/24. Changed normalized input/schema/adapter invalidates prior data approval; no collector is implemented. |
| R1-06 | Historical draft/task reports and the dated census remain under their original identities. The 13-key inventory is a narrower documentary check. | Old reports do not review later candidates; future key/profile changes require renewed evidence. |
| OMI-R01 | Binary proposal §§4.3/8, canonical OPEN-CAT-02, storage terminology, this task and development §8.7 now distinguish catalogue/preset/configured/active membership and route automatic-replacement scope. | OPEN-CAT-02/R-04/OD-03; product choice and preset location remain open, without adding a Core prerequisite. |
| OMI-R02–08 | Concrete C/E, withdrawal, cold-start, usable-group, resource, history and recovery scenarios are retained in §7.2; the underlying controls already have owners. | Apply at the stated contract/integration triggers; these are proposed future cases, not executed passes or new publication blockers. |
| R2-R03; R5-F02 | The 32-MiB goal is precisely engine/broker steady-state private bytes. Future comparison includes complete package/image/installed costs, mapped residency, startup and peak overlap, with separate Q/H write/recovery accounting. | Architecture §17 and OPEN-CAT-01; resource targets and measurements remain unqualified. |

### 7.2 Retained scenarios under existing owners

These preserve role 4's discriminating cases without introducing another
acceptance catalogue. The full original reasoning remains in its raw report.
Concrete fixtures, supported outcomes and execution admission are future work.

| Finding / existing owner and trigger | Required paired scenario and next action |
| --- | --- |
| **OMI-R01** — OPEN-CAT-02; R-04; OD-03; DATA-03/04/06/17/20. Before preset/schema or source-policy/acquisition freeze. | Reorder an identical C and add valid unselected entries: the same approved preset and ordinary traffic must remain unchanged. Then lose one selected operator: perform only a specifically authorized replacement within continuing budgets or report reduced/insufficient availability. Retain a healthy, compatible independently evidenced preset as the positive control. Name whether preset membership is in C or authorized policy, and whether replacement exists; select no quorum, default or automatic traversal here. |
| **OMI-R02** — DATA-06; OPEN-CAT-01/03; configuration/broker and lifecycle owners. Before P3/S3.3 integration and P6 migration. | Freeze C1/E1, C2/E1, C1/E2 and C2/E2, including interrupted migration and stale compare-and-swap. C2 removes or replaces an E1 target while local disable/pin/withdrawal and an E1-based edit survive. Name the composed-view owner and publication point. A compatible addition preserves unchanged policy; missing targets cannot silently drop/remap restrictions, lower minima or create permissive defaults. |
| **OMI-R03** — OPEN-CAT-03; release/trust; TA-W02/06/07; package owner. Before key use/transition and later release qualification. | Trace publisher withdrawal through reviewed corrective release to installed adoption. Distinguish first install without a protected floor, intact local withdrawal, old package with current E, and whole-machine rollback; include update unavailable under a wrong clock. Pair old-but-permitted with known-withdrawn. Unknown newer withdrawals cannot be discovered from an old signature alone. Freeze supported reaction/staleness and recovery claims without expiry invention, TLS relaxation, independent updating or erasing known restrictions on reset. Ordinary NTS retains its applicable policy. |
| **OMI-R04** — OPEN-CAT-02; TA-W02/06/07; resolver/network owner; CHECK-9/18/23/27. Paper/fixture freeze before bootstrap/native work. | Trace installed-artifact admission/startup → permitted resolution/destination → Roughtime → isolated NTS identity/revocation validation → fresh protected observations/agreement → broker gate, with no useful time/DNS/NTS cache and a badly wrong clock. Pair an admitted cold-resolution path with date-dependent/unavailable resolution; reject inadmissible private/stale-hint destinations. Include unavailable UDP and a qualified advertised TCP alternative where supported. Keys alone do not qualify this complete path or relax DNS policy. |
| **OMI-R05** — OPEN-CAT-02; R-04/OD-03; TA-W02/06; TA-C01/02/17. Before a usable/shipped preset claim. | Produce an admission matrix after operator/dependence, profile, root, compatible time policy, use permission, transport/address-family and evidence filters. Count remaining usable independent groups for each role, not rows/hosts/keys. Pair sufficient groups with one fewer represented by extra aliases/roots/transports; cross-protocol overlap and bootstrap ancestry cannot add votes. The 13 documentary keys and two explicit RFC-v1 advertisements do not close this gate or authorize weaker minima. |
| **OMI-R06** — DATA-20; OPEN-CAT-01/OPEN-DATA-03; engine/resource owner; TA-W08; CHECK-21/27. Before admitted loader/acquisition integration. | Bind bytes/work, reference fan-out, simultaneous views and actual job custody to named limits. Pair a maximum valid catalogue and completed job with capacity-plus-one, a retained old-generation job after waiter cancellation, and traversal exhaustion. Preserve provider retry/use obligations across authorized replacements; refusal means local insufficiency, not server dishonesty. Apply only to the selected lifecycle; no hot-reload implementation is required by this case. |
| **OMI-R07** — DATA-18/19; OPEN-DATA-02/03/04/05/06; storage and TA-W02. Before Q/H migration/reset. | Keep an old observation across root rotation, operator merge/split and endpoint retirement, then prune its old dictionary or clear Q/H. A retained historical definition still explains the observation; missing context becomes interpretation unavailable, never today's label, another operator's score or current key authority. Separately preserve whatever cross-restart safety minimum TA-W02 actually adopts. This does not adopt permanent quarantine or full raw-response retention. |
| **OMI-R08** — W11-W05/W11-W10; mandatory-state owner; applicable ST-W03/06; CHECK-13/15/23. Before package/lifecycle qualification. | Add catalogue/composed-policy admission failure after replacement while a prior time-ownership/restoration obligation remains. Pair successful update/removal with interruption and Reset. Optional C/Q/H repair cannot delete the sole mandatory recovery record, remove the last invocable recovery owner or let predecessor cleanup act underneath its successor. The known independent-uninstall/Reset ordering gap stays with its existing owner; this documentary milestone does not resolve it. |

### 7.3 Original review packets

The following fenced Markdown packets preserve each original report verbatim,
including its comprehension paragraphs, finding IDs, scope limitations and
historical verdict. Packet-relative references such as `work/...`,
`review-input/...` and `source-manifest.json` identify the selected review packet;
they are literal evidence text, not live repository navigation. Use the canonical
links and dispositions above for current continuation. Original report hashes
appear in §7.1. Final publication identities are established separately after the
GitHub ref update to avoid a commit/self-hash cycle.

<details>
<summary>Role 1 — documentation consistency: original report</summary>

````markdown
# Role 1 — catalogue documentation consistency

The proposed addition gives the release catalogue a more concrete direction without implementing it. It should hold hundreds of source definitions while the engine activates only its bounded working set. Binary delivery is the owner's preference; constrained CBOR and generated immutable tables remain alternatives to evaluate. Each Roughtime record needs its actual public root-key bytes, algorithm, exact protocol profile, endpoint scope and provenance in the first schema. The planned Python collector belongs to publisher preparation under the existing `tools/` directory: it prepares candidates and reproducible reviewed inputs, while source approval, signing and release remain separately owned.

The current project documents keep the pure Core continuation ahead of catalogue implementation and retain the scoped execution/admission boundaries. Fixed IPC, machine policy and broker journals still use their selected strict JSON contracts. Catalogue C, advisory quality/history Q/H, mandatory state and secrets E, and publisher work R have different authority and failure rules. The future Roughtime route supplies a provisional date for full NTS verification and the existing broker gate; placing keys in a schema does not complete that route, approve a roster or alter RGT-0 → CHR-0. Existing Q0/Q-J preparation and its unresolved audit findings remain a separate bounded question.

## Identity, authorization and completion

- **Role:** delegated documentation-consistency reviewer/editor; completed the assigned corrections and static checks. This is not the final integrated review or publication approval.
- **Source baseline:** `DavidOsipov/NevroTick@7550f83292f959a6d0acbb2d396e964a6bcf4b01`, recursive tree `d060c21586745f0e5c341a421230a14c78c6e515` as recorded in `source-manifest.json`.
- **Environment:** selected-file snapshot at `/workspace/scratch/2fc5e474c1c0/nevrotick-catalogue-20261104/work`, not a Git checkout. `git rev-parse --show-toplevel` confirmed the absence of a checkout; `git diff --no-index` was used only for file comparison. No HEAD/index/commit claim is made.
- **Writable scope used:** the seven existing documentation/task files listed below, plus this review report outside the maintained documentation tree. I did not edit the new binary-format document.
- **Context limits:** I used the assigned task, repository guidance, current canonical passages and the new draft. I did not retrieve the private KB or parallel reviewers' reports. Direct coordination with the new-document owner was limited to cross-file contradictions and writable ownership; this report claims no independent fresh-context final review.
- **Execution scope:** no product code, collector, benchmark, protocol probe, package/dependency installation, native operation or remote mutation.

## Inspected scope

Read `AGENTS.md`; the documentation guide's navigation/current amendments; the planning guide, `plan-code-change`, its review/handoff reference, `evolve-data-contracts`, its implementation contract and transition reference, and the testing contract. The full recursive manifest contains 1,072 paths. It confirms the existing lowercase `tools/` area, no nested AGENTS applying to these documentation paths, and no `docs/SHA256SUMS.txt`. Enumerating paths is not a full repository source audit.

Substantive document inspection covered:

| Surface | Relevant read scope |
| --- | --- |
| New binary-format draft | Initial full draft; subsequently rechecked its OPEN-CAT mapping and required geography/profile rows after the assigned owner changed them. The final publisher must review the integrated final bytes. |
| Catalogue/statistics draft | Purpose and meaning rules; DATA-01–08 and DATA-22–24; allocation/security/open-decision/prerequisite passages in §§4–8; full §10 comparison card. Other DATA groups were read through the allocation/minimum-record summaries where needed, not independently re-audited in full. |
| Storage tables | Current C/Q/H/E/R definitions, Tables A–C, format shortlist and accompanying constraints. Existing capacity calculations and their source claims were not re-benchmarked. |
| Catalogue task | Current §§1–4 and §6.1 coordinator dispositions, to preserve the named DATA/EXP/SEC gaps and existing pilot. Historical raw reports were not used as new review opinions and were preserved bytewise. |
| Time-assurance plan/task | Plan §§1–3 and 7–10; task §§1–6 introduction, with particular attention to TA-W03/06, TA-C06/10/17, roadmap order and execution scope. Historical review sections were preserved bytewise. |
| Development plan | Current §7 Core/admission checkpoint and source-work routing; §8.6 later roadmap handoff. Added only a catalogue routing row and §8.7 within existing owners. |
| Canonical technical/owner context | OD-03 §5; architecture strict-input/JSON contracts in §§6.1–6.2, state separation/restoration/optional-storage introduction in §12.1, and resource/native-work contracts in §17; protected-storage plan §§1–2; MSIX plan §3 and the content-integrity requirements in §3.1. |

The initial archived new-draft input is available in `review-input/server-catalogue-binary-format.md`, SHA-256 `b8977ea8c3e2fa1f90511f59f613511983d03a615888a361bbd9c1d1d15adced`. This report's fixes are against the working snapshot; the other owner continued refining the new draft concurrently.

## Stable findings and disposition

### R1-01 — Medium — current catalogue descriptions retained the superseded shipment choice

**Evidence:** draft §1.1/§5.1 and storage Table A still presented JSON and CBOR as equally open shipped-catalogue candidates. The format table described JSON as a small-catalogue baseline without the owner's new binary-delivery qualification. The older comparison card did not include the generated-table alternative's cost boundary.

**Consequence:** a development agent could reasonably continue preparing JSON for delivery or treat a serialization microcomparison as the full installed-cost decision.

**Correction:** current C passages now prefer binary shipment, distinguish constrained CBOR and generated immutable tables from a final selection, and retain JSON as an optional publisher/reference representation. CBOR's recommendation is conditional on choosing a separate resource; generated tables remain a leading release-bound alternative. The C-J/C-C reference pair and the distinct Q0/Q-J pilot remain named; no new trial count or budget is introduced. The existing authenticity/loader wording now distinguishes exact captured resource bytes from generated-data construction and current-policy checks. A separate signature and its algorithm remain open.

**Disposition:** corrected and statically verified in the assigned files. Invalidated by a later owner change to the delivery direction or by edits that again conflate C with Q/IPC/policy/journals.

### R1-02 — Medium — first-schema Roughtime material was described as conditional on runtime adoption

**Evidence:** DATA-05's class, draft §10.1 and the catalogue task's current scope called Roughtime fields conditional, without separating schema support from future protocol execution.

**Consequence:** an initial catalogue/schema could omit the public-key/profile variant and leave a later incompatible expansion or an unusable keyless record.

**Correction:** DATA-05, the minimum C record, storage Table A and both task handoffs now require the algorithm, exact public-key bytes, exact profile, endpoint scope and provenance for every Roughtime record from the first schema. The future synthetic C recipe must freeze corresponding public test-key bindings. Recognized inert metadata is explicitly separate from enabled/qualified runtime, with the new document owning any reviewed compatible projection. Runtime verification, key-policy qualification and roster admission remain future. TA-W06 consumes those bindings under its existing acceptance scope.

**Disposition:** corrected and statically verified. No real key was added to a production artifact and no protocol case was run.

### R1-03 — Medium — new draft reused OPEN-CAT-02 for a different unresolved decision

**Evidence:** canonical draft §6 assigns format/schema/parser limits and release digest/signature/trust-root strategy to **OPEN-CAT-01**; roster/permissions/Pool/Roughtime compatibility and cold resolution to **OPEN-CAT-02**; freshness/withdrawal/rollback to **OPEN-CAT-03**. The initial new draft assigned a separate signature and release trust to OPEN-CAT-02.

**Correction owner:** the new-document owner, notified with the canonical mapping. I retained the original IDs and meanings in the existing documents and the appended task preparation table.

**Disposition:** verified corrected in the new-document owner's frozen §3/§7/§8 text: independent signature uses OPEN-CAT-01; release trust/freshness uses OPEN-CAT-01/03; roster/profile admission uses OPEN-CAT-02 with relevant lifecycle owners. The checked new-draft SHA-256 was `77e906705fced8667bd6bcc7f0a70b4c17352e4a1bd78f22438adb1eacf2c4a4`. Final integrated-byte verification remains the publisher's responsibility; subsequent edits require rechecking affected conclusions.

### R1-04 — Medium — new logical schema did not clearly retain DATA-03's required geography field

**Evidence:** DATA-03 requires country/region or an explicit unknown value with evidence/applicability. The initial new schema mentioned only optional region on operator metadata, which could conflate domicile with endpoint location and omit the required source/endpoint coverage.

**Correction owner:** the new-document owner, notified of the unchanged canonical requirement. Existing draft/Table A requirements were retained.

**Disposition:** verified corrected in the observed new-draft geography/time-behavior row at the same hash above. It requires country/region-or-unknown at the appropriate source/endpoint scope and separates domicile, anycast scope and IP-derived estimates. No location inference or live geolocation was performed.

### R1-05 — Medium — catalogue/tool refinement lacked a preparation handoff within the existing ownership chain

**Evidence:** the index routed only to the older draft/tables; the current catalogue task's next step covered Q0/Q-J; development §8.6 routed Roughtime only through the later roadmap stage. The new first-schema and publisher-tool requirements had no corresponding existing-task preparation entry.

**Consequence:** the catalogue requirement could be overlooked, incorrectly held until RGT-0, or used to replace the separate statistics pilot and its open findings.

**Correction:** linked the new draft in the guide, added one source-only row in development §7 and §8.7 routing, and appended catalogue task §7. The planned path is explicitly future `tools/server-catalogue/build_catalogue.py`, reusing the actual lowercase area. DATA-24, the R table and the task require the exact approved normalized snapshot and relevant adapter/schema/generator identities; changed inputs invalidate that approval. Source/default, schema, release, storage and time-assurance owners remain distinct. Current Core work, Q0/Q-J, the later RGT sequence and scoped holds remain controlling.

**Disposition:** corrected and verified by link/anchor checks and comparison of the unchanged current Core/admission block and Q pilot bounds. No script or runtime was created.

### R1-06 — Low — historical identity and census scopes needed explicit separation after current-text revision

**Evidence:** current addendum/task wording said all earlier draft sections still retained their original bytes. That assertion would become false when the newly authorized current passages changed. The new public-key table has 13 Ed25519 rows from one pinned list, while the existing dated census has 14 hostnames/about 10 groups after adding another documentary endpoint; those are different read scopes.

**Correction:** made the preserved-byte statement explicitly historical, retained every raw report and its original identity, and added a narrow cross-reference distinguishing the 13-key documentary check from the broader dated endpoint census. The latter's counts, endpoint rows and compatibility conclusions were not rewritten as new verification.

**Disposition:** corrected and verified. The whole historical draft §9, catalogue task historical report content, and time-assurance review sections remain unchanged. Older review verdicts do not approve this candidate.

## Edited files and checked identities

| File under `work/` | SHA-256 after Role 1 edits |
| --- | --- |
| `docs/README.md` | `d7aa49923c93f1d94dcd4a16552829b7aacfd0a335a632ff14029f8d559a1dda` |
| `docs/plans/server-catalogue-and-statistics-draft.md` | `b649256510fe944c6ed1429d73b564c9ad748a85e02c357728ea7f165838ee79` |
| `docs/plans/server-catalogue-storage-tables.md` | `6e3b7817b3467972c5385f6db7dadc728f732250db1ac383bd6a086c859af8fe` |
| `docs/plans/development-plan.md` | `12a3dfa58308327e8c3a75aaabe97e41bf5b7c6b9b579f9f5ba091f0941b7ecc` |
| `docs/plans/time-assurance-integration.md` | `bae863263e7728e75bb105b5fe047ad6015e0f571ac1b36a0f3762e108b21e2a` |
| `tasks/time-assurance-integration.md` | `42bb684ad95c04cd81d97fe9f597ae063ae163d89668be6da10b6e776d231a0d` |
| `tasks/server-catalogue-and-statistics.md` | `714e42fe35a29bc536bf07bb0acb7063b92c5c81d1d6c4da99d20beb2b5b9358` |

Checks actually completed:

- Compared each assigned file against its baseline before editing; all seven were unchanged at entry.
- Checked all **17 newly introduced relative-link targets**, with **3 new anchor targets** resolved against actual selected-file headings. No missing path, missing anchor or unavailable new anchor body remained.
- Ran `git diff --no-index --check` for each of the seven changed files. No whitespace diagnostics remained. The expected no-index difference exit status was distinguished from a check error. An initial combined-tree check flagged the new document's original Markdown hard-break spaces; its owner corrected those, and the final combined-tree check also produced no diagnostics. I did not edit that file.
- Verified UTF-8 decoding and absence of replacement characters for all seven files.
- Verified the complete historical draft §9 byte block, all six historical `<details>` blocks across the draft/catalogue task, and the time-assurance review suffix were unchanged. Catalogue task §§5–6 differ only by the following section separator outside their retained content.
- Compared development §7 up to the source-work table, draft §10.4 onward, time-assurance TA-C acceptance table, and the substantive dated census body with baseline: **unchanged**. Current Core/admission scope, original Q pilot/run limits, TA-C cases and dated census content therefore did not change in this edit.
- Searched the current edited sections, excluding protected historical reports, for the specific stale shipment/conditional-key phrases addressed above; none remained.

These are document/static checks. They establish no binary-format size, startup performance, algorithm/parser enforcement, cryptographic authentication, provider availability, Windows protection or release result. The source manifest has no documentation checksum manifest, so no bundle-integrity result is claimed.

## Remaining gaps and handoff

The final publisher should consume this report with the separately owned format/storage, schema and omission reviews, reconcile the final new-document bytes, and record the integrated identity/dispositions in the existing catalogue task. This role is complete for its bounded scope; the cross-document fixes are ready for that integrated review.

Retained product gaps include the final C representation and complete installed/runtime measurements; concrete logical-to-wire schema, caps, normalization, required-feature and reader/rollback rules; approved provider/key/profile roster and rotation evidence; release trust and bad-clock freshness; and the actual future collector's bounded acquisition/deterministic build contracts. Existing DATA/EXP/SEC findings, Q/H defaults and protected-storage/native/package gates remain open at their owners. No new owner decision, acceptance pass, code implementation or remote publication is claimed.
````

</details>

<details>
<summary>Role 2 — fresh formats and storage: original report</summary>

````markdown
The proposal would supply NevroTick with a small, immutable binary catalogue of reviewed NTP, NTS and Roughtime definitions, including Roughtime public verification keys from the first schema. It favors constrained CBOR when the catalogue is a separate resource, keeps generated Rust data as a serious release-bound alternative, and leaves Protobuf conditional. A future Python publisher tool would collect untrusted source material, preserve provenance, expose conflicts for review and produce a deterministic artifact. Collection, source admission, signing and client operation remain different steps.

The inspected baseline describes an existing portable/prototype implementation and a planned Windows service, trust and storage architecture; it does not establish a qualified production catalogue or persistence system. The selected delivery direction is one complete hardened MSIX. Current strict NTS/default-policy and JSON IPC, machine-policy and broker-journal contracts remain authoritative. C holds public release definitions; Q and optional H describe observations; E denotes several existing owners with different mandatory, optional and secret-state rules. Future Roughtime bootstrap still feeds full NTS verification and the existing clock authority. Hundreds of catalogue records do not imply hundreds of active associations.

# Independent review 2 — formats and storage locations

**Reviewer:** fresh format/storage reviewer, role 2  
**Date:** 4 November 2026  
**Candidate:** NV-CAT-BIN-001 discussion draft 0.1, frozen `review-input/server-catalogue-binary-format.md`  
**Candidate SHA-256:** `b8977ea8c3e2fa1f90511f59f613511983d03a615888a361bbd9c1d1d15adced`  
**Baseline:** `7550f83292f959a6d0acbb2d396e964a6bcf4b01`  
**Requested stage:** documentation readiness and independent comparison; no implementation, benchmark, native Windows work or public time-server probes.

## 1. Verdict and evidence boundary

**REVISE for the documentation handoff.** Two bounded inconsistencies need correction: the supplement repurposes existing OPEN-CAT identifiers, and its logical catalogue table weakens inherited metadata requirements. These are correctable document defects, not demonstrated runtime vulnerabilities. Neither requires another product decision merely to preserve the current authority.

The representation shortlist and C/Q/H/E separation are otherwise defensible. There is no evidence for declaring CBOR, generated tables or Protobuf the smallest or fastest deployed implementation. The draft generally states that limit correctly. Its missing production caps, concrete codecs, qualified Windows paths and runtime measurements are openly assigned future work; they are not additional blockers for completing this documentation stage.

**Permitted next action:** reconcile R2-F01 and R2-F02 in the existing documents and task, retain the conditional recommendations below, then continue the already requested documentation review. This verdict starts no experiment and closes no implementation, native or release gate.

### Inputs actually inspected

I read the frozen candidate in full, `base/AGENTS.md`, the documentation guide, the independent-review/handoff guidance, the testing contract, and the implementation/data-contract/transition guidance. Substantive baseline checks used:

- Catalogue/statistics draft §§1–7 and §10: DATA-01–DATA-24, existing OPEN identifiers, allocation, authenticity, recovery and comparison requirements.
- Storage tables §§1–7: C/Q/H/E meanings, candidate formats and whole-family accounting.
- Architecture §§12.1–12.1.2 and §17: state ownership, restoration, optional-secret boundaries and resource contracts.
- MSIX plan §§3–3.1 and §5: package identity/content trust and update/Reset/removal limits.
- Time-assurance plan §§3, 5, 7 and 10, plus the relevant TA-W task assignments.
- The relevant owner-decision rows for OD-03 and amended OD-05.

The supplied manifest identifies an immutable selected-file snapshot, not a Git checkout. I verified the candidate SHA-256 and Git blob identities of 17 inspected baseline/guidance files against that manifest; all matched. This verifies the inspected files, not the entire repository or implementation. I did not open other agents’ reports or the private KB, and did not use earlier review conclusions as evidence. The only write is this report. No dependency was installed and no benchmark, protocol exchange, Windows operation or code test was run.

The independent expected result is preservation of DATA-02–09’s catalogue meaning, DATA-15–24’s authority/lifetime separation, existing OPEN-CAT/OPEN-DATA/OPEN-ST ownership, TA-W02/06’s evidence and catalogue boundaries, and architecture §17’s bounded-work accounting. The catalogue/build owner owns representation preparation; storage owners own Q/H and paths; package and time-assurance owners retain trust, freshness and native qualification.

## 2. Stable findings

### R2-F01 — Medium — OPEN-CAT identifiers have acquired different meanings

**Location:** candidate §3, line 53; §7, line 167; and §8’s release-trust and Roughtime rows, lines 179–180.

**Evidence:** line 53 assigns a separate project signature to OPEN-CAT-02. The authoritative catalogue/statistics draft §6 instead defines:

| Existing ID | Existing meaning |
| --- | --- |
| OPEN-CAT-01 | Catalogue format/schema, parser limits, release digest/signature strategy and trust-root lifecycle |
| OPEN-CAT-02 | Initial provider roster, permissions, Pool vendor allocation, Roughtime wire/key compatibility and cold-bootstrap resolution |
| OPEN-CAT-03 | Revision floors, stale/offline behavior, key withdrawal and rollback under an unreliable clock |

The supplement’s release-trust row points to OPEN-CAT-02/03, while its Roughtime-admission row names OPEN-CAT-03 without OPEN-CAT-02. The same identifiers therefore no longer reliably locate the same decision. The generic OPEN-CAT reference in §5 is not itself a defect.

**Consequence:** a later implementer could close a signature question under the roster identifier, or overlook the actual roster/permission prerequisite. This defeats the supplement’s own promise that the existing draft owns these decisions. It is a routing/authority defect even while all mechanisms remain unselected.

**Smallest correction and owner:** the documentation coordinator should use OPEN-CAT-01 for format/authentication construction, OPEN-CAT-02 for roster/permission/profile/bootstrap-resolution admission, and OPEN-CAT-03 for freshness/withdrawal/rollback. Use combinations only where the named work spans them. Preserve the canonical definitions; do not renumber the existing inventory.

**Stage effect:** correct before calling this a reconciled documentation handoff. No new signature decision or implementation is required to fix the references.

### R2-F02 — Medium — The refined C table does not preserve the inherited minimum metadata

**Location:** candidate §4, especially the operator/dependence row at line 63 and the endpoint/protocol rows at lines 64–67.

**Evidence:** the new table says “optional region.” DATA-03 explicitly requires a country/region field with an unknown state; finer location enrichment is optional. DATA-03 also requires declared time behavior, including UTC/leap or smear policy and compatibility provenance. DATA-05 carries the corresponding Roughtime time-policy meaning. The minimum C profile in catalogue/statistics §10.1 explicitly includes time-scale/leap policy and country/region with method or unknown.

The supplement does not explicitly carry a time-policy binding. Its protocol-profile definition distinguishes wire variants such as RFC-v1 and drafts, which is not by itself a statement of leap/smear compatibility.

**Consequence:** the logical refinement can be read as reducing the information that each format must represent. A smaller benchmark corpus would then compare unequal capabilities. If that reduced mapping reached implementation, unknown geography could be silently treated as omitted enrichment, and selection would lack an explicit source for time-policy compatibility. No such runtime implementation or mis-selection was observed.

**Counterevidence considered:** §1 says the original DATA inventory remains authoritative. That prevents this supplement from formally overriding it, but leaves an internal inconsistency for the promised logical-to-wire freeze.

**Smallest correction and owner:** the catalogue/schema owner should change the location entry to “country/region with explicit unknown and provenance/applicability; finer location optional,” and add an explicit time-scale/leap/smear-policy reference or clearly map it to an existing profile record. Preserve unknown values and current admission rules; do not manufacture geographic or clock-policy evidence. All candidate encodings must carry the same required semantics.

**Stage effect:** correct the logical table before calling the schema refinement reconciled. Exact numeric encodings and a production CDDL schema remain later work.

## 3. Critical comparison of the catalogue choices

### The comparison needs three separate choices

Format, placement and in-memory representation are different decisions:

| Choice | Alternatives relevant here | Why it matters |
| --- | --- | --- |
| Logical encoding | Constrained CBOR, a declared Protobuf profile, generated typed records | Determines accepted meanings, extension rules and structural admission |
| Delivered placement | Separate immutable package resource, embedded encoded bytes, generated image data | Determines release binding, installed layout, update granularity and access path |
| Runtime representation | Retained immutable bytes plus bounded indexes, materialized compact records, generated arrays/pools | Determines allocation, lifetime, lookup and memory costs |

For example, embedding a CBOR blob in an executable does not remove CBOR decoding. Generating typed arrays can remove that decoding but introduces generator and image-layout obligations. A separate resource can be read once into owned immutable bytes without repeatedly traversing the file path. “Binary” alone settles none of these costs.

**R2-R01 — Nonblocking recommendation:** make these axes explicit in the later comparison card. No new candidate family or benchmark campaign is needed.

### Generated immutable data is the strongest countercase for the current release model

**Engineering judgment:** generated immutable data deserves equal standing with CBOR for the current release-bound, single-client workload. A future Python builder does not by itself require a runtime interchange format. It can emit reviewed data literals or compact arrays alongside a semantic manifest.

Rust permits constant-expression static initialization, and immutable statics without interior mutability may be placed in read-only memory. This supports the mechanism; it does not establish the resulting PE layout or memory use. [Rust static-item reference](https://doc.rust-lang.org/reference/items/static-items.html)

The credible version of this alternative is more than a large list of pointer-rich structs. Compare bounded numeric identifiers, immutable arrays and shared string/key pools with checked indices. This can reduce repeated strings and pointer/relocation overhead without introducing an unsafe packed-struct or raw-memory reinterpretation format. The size benefit is a hypothesis to measure.

Its strongest advantages are no general runtime deserializer for C, no per-record owned allocation when consumers can borrow the data, and release/code/schema coupling that makes mixed artifact generations easier to avoid. Its contrary costs include larger or less favorable image layout, rebuilds for every catalogue change, permanent image residency where pages remain used, and a generator that turns untrusted strings into source code. Literal escaping, range checks, ordering, provenance and a reviewable semantic diff are therefore security-relevant builder obligations.

Generated data must preserve private validated-domain guarantees. A successful Rust compilation does not establish operator permission, destination policy, root provenance or current trust eligibility. The draft’s distinction between immutable pure invariants and current consumer checks is appropriate.

### CBOR is a reasonable separate-resource candidate, with an implementation-dependent cost

CBOR supports integer keys and byte strings, while CDDL can describe the accepted shape. Neither gives the actual Rust consumer strict validation automatically. CBOR’s specification specifically warns that some generic decoders can lose duplicate-key information; a later domain check cannot recover evidence already discarded. The draft’s constrained types, definite lengths, duplicate rejection and aggregate caps are justified. [RFC 8949 §§3–5](https://www.rfc-editor.org/rfc/rfc8949.html) · [CDDL, RFC 8610](https://www.rfc-editor.org/rfc/rfc8610.html)

The strongest CBOR implementation to compare is bounded, type-directed admission into a compact representation, potentially retaining borrowed strings/key bytes in one owned immutable backing buffer. Do not assume the only CBOR implementation builds a general value tree and then hundreds of heap-owned objects. Conversely, do not claim a zero-copy design unless its actual decoder, lifetime model and index validation achieve it.

CBOR’s strongest case is a plainly inspectable independent resource with a documented schema, a compact binary payload and a clean publisher/consumer boundary. Its costs include newly linked decoder code if not already present, structural and semantic validation, captured-input/output overlap, compatibility maintenance and package-resource identity handling.

**R2-R02 — Nonblocking recommendation:** retain the wording “recommended separate resource” as conditional. If a separate artifact has no required operational advantage, the evidence does not justify preferring it over compact generated data. This is a recommendation to compare well, not a claim that generated tables win.

### Protobuf is viable, but its advertised strengths must match the workload

Protobuf can offer established schemas, generated accessors and Python/Rust interoperability. The current design has one shipped Rust consumer and a publisher-side tool; independently evolving clients or network RPC are not established requirements. Adding their machinery solely for hundreds of static rows is unjustified.

Its precise countercases matter. Standard decoding permits repeated singular fields with last-value or message-merge behavior. Explicit presence is available; it should be chosen where absent and zero differ. A consumer requiring duplicate or unknown-critical-field rejection must enforce it before ordinary decoding loses that distinction. This is a concrete admission cost, not evidence that the format is unusable. [Protobuf encoding](https://protobuf.dev/programming-guides/encoding/) · [Field presence](https://protobuf.dev/programming-guides/field_presence/)

The draft is correct that noncanonical serialization does not prevent authenticating an exact file. Deterministic regeneration across tool versions is a separate requirement from verifying fixed delivered bytes. [Protobuf serialization and canonicalization](https://protobuf.dev/programming-guides/serialization-not-canonical/)

Do not compare “Protobuf” as one universal runtime footprint. For example, Prost generates Rust types and omits runtime reflection; its ordinary string/byte fields still have allocation implications. That upstream example is not a dependency recommendation: the precise runtime, feature, generator and licence graph must satisfy NevroTick’s existing admission policy. [Prost upstream documentation](https://github.com/tokio-rs/prost)

**Disposition:** keep Protobuf conditional. A real interoperability need, already-admitted implementation, or materially better complete-product result could justify it. No such result is present in this review.

## 4. C/Q/H/E allocation and physical-location countercases

| Class | Assessment | Strongest contrary case and required decision |
| --- | --- | --- |
| **C** | One immutable package-owned definition set, with one engine admission owner, is appropriate. Separate resource and image data both remain credible. | Do not create a writable per-user catalogue merely to simplify path discovery. Package identity, approved deployment location and exact-content binding matter more than a familiar directory prefix. Hold each consumer to one generation through replacement. |
| **Q** | Engine-owned machine storage is appropriate for shared advisory summaries. Retaining SQLite versus a bounded snapshot is justified. | If the retained state is small and changes infrequently, a snapshot may avoid database code, connections and maintenance. If dictionaries, sparse events and checkpoints need frequent coordinated changes or selective reads, rewriting the entire snapshot may cost more than SQLite. Freeze operations and loss tolerance before choosing. |
| **H** | Optional bounded detail belongs outside live estimator state. The draft correctly avoids making it mandatory. | A required replay, support or forensic question may justify detail that aggregates cannot recover. If that question is absent, the cheapest H is no detailed archive. A runtime Parquet/codec stack is not justified solely by the possibility of future analysis. |
| **E** | Separate existing owners must be retained. This is an ownership category, not one database, encoding or protection level. | Machine policy and consumed correction authority cannot share disposable recovery. A drift prior and optional NTS snapshot have different, sometimes discardable, lifetimes. Do not make every E item mandatory merely because some E items are mandatory. |
| **R / U** | Publisher evidence stays outside clients; the UI receives bounded projections and writes deliberate user exports under its normal identity. | Release reproducibility must still have enough retained publisher input and provenance. Minimizing client bytes must not destroy the evidence needed to explain a key change. |

### Q: simplicity must include failure and write costs

A snapshot avoids database machinery only if the required operations remain simple. Its full read/decode/rewrite, staging, flush, replacement and recovery costs belong in the comparison. SQLite already supplies transactional mechanisms, but their guarantees depend on the declared filesystem/VFS behavior; they are not cost-free or automatically stronger than a carefully bounded snapshot for every workload. [SQLite application-file use](https://www.sqlite.org/whentouse.html) · [SQLite atomic commit](https://www.sqlite.org/atomiccommit.html)

Neither one writer nor a small main file makes a WAL family bounded. Long-lived readers can prevent complete checkpoints and allow WAL growth. The original draft’s complete-family limits, reader bounds and maintenance reservations should remain controlling. The new catalogue preference does not justify changing the separate Q0/Q-J comparison or introducing CBOR into already-selected JSON policy/journal boundaries. [SQLite WAL](https://www.sqlite.org/wal.html)

“Protected engine-owned storage” should continue to mean the actual admitted service identity and file/directory access boundary unless a separate encryption construction is selected. It does not imply that all advisory summaries need VBS roots. Encrypting Q would introduce key availability, migration and recovery costs without converting history into authoritative evidence.

### Windows integration should follow lifetimes, not API availability

The distinction between immutable package content and writable machine state is sound. Microsoft documents package content enforcement as explicit opt-in through the manifest and describes remediation when tampering is detected. That is supporting source evidence, not an observation that the intended service/package path has passed on Windows. [MSIX package-integrity enforcement](https://learn.microsoft.com/en-us/windows/msix/desktop/tamper-protection)

Windows-managed per-service directories are a legitimate Q/H alternative when their disposable lifetime is intended. Microsoft states that the persistent-state directory limits write access to LocalSystem and the service SID, and that service-state directories are deleted when the service is uninstalled. This supports considering them for optional state while rejecting them as the sole home for unfinished mandatory restoration. It does not establish all read rights, child-file rights or a qualifying upgrade/reset sequence. [GetServiceDirectory](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/nf-winsvc-getservicedirectory)

The draft correctly leaves final paths, ACLs, package Reset and removal to W11/ST-W owners. A format choice should not be used to preselect those native mechanisms.

### Public roots, private secrets and safety evidence remain different

Roughtime verification roots are public catalogue/policy material. Their integrity and authorized replacement matter; they are not NTS traffic secrets and do not require secret-key storage merely because the field is named “key.” The public source catalogue must also remain distinct from protected storage’s root/key catalogue, which tracks storage authority.

Q/H deletion must not erase an adopted safety restriction. At the same time, the exact cross-restart conflict minimum and clearance rule are still TA-W02/OPEN-DATA-05 work. The final text should retain that conditional status, not imply that this supplement has already adopted a new permanent quarantine or proof archive.

A separate signature for C may become useful for an independent distribution/authority model. Under today’s release-bound model, it also adds a second trust-root, rotation and recovery lifecycle. It should not be selected by the word “binary,” and it cannot solve stale signed releases or a wrong-clock freshness cycle. The frozen draft appropriately keeps these choices open, subject to R2-F01’s corrected routing.

## 5. Total resource costs that can change the recommendation

The draft’s 300 × 512-byte example is correct arithmetic: 153,600 bytes, or 150 KiB, of hypothetical payload. It is not a maximum or representative measurement. The architecture’s 32 MiB target is specifically combined engine/broker **steady-state private bytes**, with peaks separately reported; it is not an allowance for the catalogue alone or a total physical-memory claim.

**R2-R03 — Nonblocking recommendation:** carry the following accounting into the existing future comparison card, without executing it in this milestone:

| Dimension | Necessary accounting |
| --- | --- |
| Delivered and installed bytes | Incremental codec/runtime code, generated data, executable alignment, package metadata, actual compressed MSIX and installed allocation; include simultaneous old/new update and staging needs where relevant |
| Startup CPU and latency | Artifact access/authentication, structural/domain admission, indexes, copies and allocations; compare equivalent accepted data and policies |
| Steady operation | Catalogue lookups and active-state cost; no recurring full decode, periodic scrape or per-poll persistence introduced by a large passive catalogue |
| Memory | Private bytes, relevant resident/mapped image pages, allocator overhead, backing buffers, indexes, temporary decode overlap and peak native work |
| Q/H write cost | Actual write/flush cadence, rewrite or changed-page amplification, checkpoints, compression, cleanup, export and migration headroom |
| Security and maintenance | Parser/generator graph, strict-admission checks, hostile-input review, schema migration, dependency/licence updates, signature/key lifecycle and recovery ownership |

Windows working-set measurements concern resident physical pages; private-byte accounting alone does not describe the residency of shared or mapped catalogue data. Therefore a static/mapped design must not “win RAM” merely by moving bytes outside the private allocation metric. Avoid double-counting shared pages across processes, but do not treat them as free. [Windows working-set documentation](https://learn.microsoft.com/en-us/windows/win32/memory/working-set)

The complete-package comparison should also preserve actual MSIX delivery behavior. Windows uses file/block metadata for integrity and differential delivery; changing an executable and changing an isolated resource can have different update costs. No fixed update-size advantage is asserted here. [Microsoft MSIX overview](https://learn.microsoft.com/en-us/windows/msix/overview)

Start without an additional catalogue compression dependency, as proposed. Compression should be justified by net package/disk benefit after code, buffers, expansion limits and CPU costs, not by raw payload size alone. Public key bytes, timestamps, repeated labels and provenance have different distributions; synthetic repeated text cannot establish a production ratio.

Finally, catalogue cardinality is not the whole resource dimension. Historical source revisions, retired dictionaries, network contexts, stale generations, concurrent old/new artifacts and unfinished native jobs require independent caps. Allocate mutable protocol state only for admitted active work, retain only historical definitions needed by surviving Q/H records, and count Roughtime/reserve work in the same aggregate resource ownership. The frozen document substantially preserves these principles.

## 6. Stage disposition

| Stage | Result |
| --- | --- |
| Frozen source/document inspection | Completed within the stated scope; candidate and 17 baseline/guidance file identities matched |
| Format and location comparison | Completed as source-based reasoning, with credible countercases retained |
| Documentation handoff | **Revise** R2-F01 and R2-F02; R2-R01–03 are nonblocking refinements |
| Production format or storage selection | Unselected; no measured winner |
| Consumer, generator and persistence implementation | Not reviewed or run |
| Resource caps, Windows enforcement and package lifecycle | Future qualification under existing owners |
| Release approval | Not requested or established |

No additional schema engine, background service, encryption default, independent catalogue updater, public-server campaign or replacement storage owner is warranted by this document. The next useful result is a coherent corrected document and owner mapping; the later implementation must establish its own candidate-specific behavior and costs.

````

</details>

<details>
<summary>Role 3 — schema review and corrections: original report</summary>

````markdown
# Role 3 — catalogue schema review and permitted corrections

The proposal prepares a binary catalogue for hundreds of NTP/pool, NTS and Roughtime records while keeping the set of active network associations separately bounded. It compares a constrained CBOR resource with generated immutable Rust tables, includes public Roughtime root keys and profile material in the logical schema now, and assigns future collection and deterministic building to Python tooling under `tools/`. The catalogue is release-bound configuration: collection, curation, artifact admission and current network use have separate responsibilities. This stage produces documentation, not a final wire schema, a shipped provider roster or an implementation.

The inspected current authorities retain the strict JSON contracts for IPC/policy/journals, current NTS and source-default policy, one privileged clock broker, and separate mandatory versus advisory storage. DATA-01–09 already require layered identities, time-scale and location meaning, root/profile binding, provenance and generation-aware consumption. Roughtime-first bootstrap is a later qualified extension; putting its records in the schema does not execute RGT-0 or make it a prerequisite for independent ordinary NTP/NTS work. Historical quality scores and saved public keys cannot construct fresh authenticated evidence, source independence or clock authority. Current Core priority and the named execution holds remain controlling.

## Identity, scope and completion

**Completed:** bounded schema review, authorized documentary corrections and static reinspection. **Disposition:** ready for coordinator reconciliation and the final documentation reviewer. Dependent implementation is not ready merely because this review is complete; the open freezes below still apply.

| Item | Identity or scope |
| --- | --- |
| Maintained writable surface | `work/docs/plans/server-catalogue-binary-format.md` only |
| Review artifact | `review-3-schema.md`, outside the maintained documentation bundle |
| Supplied repository baseline | `7550f83292f959a6d0acbb2d396e964a6bcf4b01`; selected-document packet, not an independently checked full repository |
| Initial target SHA-256 | `b8977ea8c3e2fa1f90511f59f613511983d03a615888a361bbd9c1d1d15adced`; matched the frozen `review-input/server-catalogue-binary-format.md` |
| Corrected target SHA-256 | `77e906705fced8667bd6bcc7f0a70b4c17352e4a1bd78f22438adb1eacf2c4a4` |
| Supplied Roughtime source extraction SHA-256 | `e1cc8a4acf2df434f0aaa3ea644e28899732bb516bff97092fb141b7435e41d9` |
| Supplied key-check evidence SHA-256 | `80cb6b7b7518af5079f13db16112f5d9bacd232df030dd23b1bc34ede904a530` |
| Inspection environment | Local Linux selected snapshot; `git rev-parse --show-toplevel` returned “not a git repository” |
| Excluded execution | No scripts/product code, dependency changes, benchmarks, live time-server exchanges, Windows probes or GitHub writes |

The full target was read before repair and reread after the substantive edits. Canonical reads covered the documentation guide's authority/routing, catalogue draft §§1–2 and DATA-01–09, its open-decision definitions and qualification expectations, the catalogue task §§1–4, architecture §6, §7.4 and §12.1, OD-03/04, and relevant time-assurance admission/inventory and TA-W/TA-C ownership. Applicable repository guidance included `evolve-data-contracts`, its transition procedure, `implement-rust-contracts`, contract enforcement, the implementation/security/testing contracts, crypto/trust guidance, and the independent-review/handoff procedure.

Context limitation: no private memory or current peer-review file was opened. Broad reads of canonical task/draft files incidentally exposed some embedded historical review receipts; those receipts were excluded as finding oracles. Root/role-1 coordination messages identified canonical-ID/field concerns and the required pre-RGT distinction, which were checked against the primary contracts. This is therefore a bounded delegated schema review with disclosed coordination and incidental historical exposure, not a claim of an entirely blind review.

## Stable findings and applied corrections

All entries below describe design/documentation deficiencies or ambiguities in the frozen initial draft. They establish no implemented vulnerability. “Corrected” means the revised document was inspected against the stated source contract; it does not mean future runtime controls were executed.

| ID / severity | Initial location, evidence and consequence | Applied correction and verification basis |
| --- | --- | --- |
| **R3-F01 — Medium** | §4 only mentioned optional region in operator metadata and did not carry the canonical time-scale/leap/smear compatibility contract. DATA-03 requires country/region-or-unknown at the appropriate scope; DATA-03/05 require time behavior. An implementation using only the short schema table could lose required explanatory data or assume compatible UTC behavior. | §4.1 now carries explicit country/region-or-unknown, provenance/scope, and time-scale/leap/smear policy compatibility. Operator domicile, endpoint location, anycast and IP-derived estimates remain distinct. Checked against DATA-03/05; no location was invented. |
| **R3-F02 — Medium** | §§4–5 called IDs stable but did not define reference integrity, local namespace collisions, material changes, normalization collisions or the relation between a root key and its authorized uses. A single key/state row plus endpoint/profile arrays could be read as authorizing their Cartesian product or allowing replacement bytes under an old ID. | §§4.1–4.2 and §5.1 now separate source, endpoint, key and binding identity; preserve key-material identity; require exact scoped joins, complete reference checks, namespace rules, declared endpoint transitions and revision-bound history. Key rotation may preserve a logical source but cannot silently transfer context or replace old key bytes. Independent controls are retained as future work in §6.3. |
| **R3-F03 — Medium** | §4's generic endpoint/profile table left mixed protocol variants, effective port/default meaning and host/address equivalence implicit. §3's generic deterministic-CBOR requirement did not explicitly cover encoded-key type equivalence, duplicate loss, absent/null or unselected types. Schema shape alone would not establish one safe interpretation. | §§3 and 4 now require closed typed service variants, distinct DNS/IPv4/IPv6 forms, absolute DNS semantics, a frozen IDNA/normalization contract, explicit transport/port and typed field/presence rules. Duplicate members are rejected before map normalization, references are checked before publication, and consumers use checked values. These are semantic obligations; exact binary labels, widths and caps remain unselected. |
| **R3-F04 — Medium** | §5 identified keys and profiles but conflated publisher candidate approval with runtime key lifecycle, and left exact key/profile/endpoint scope and successor activation too implicit. A familiar name, equal key length, an observed second key or an ambiguous version label must not authorize a new verifier path. | §5.1 separates curation from active/next/overlap/retired/revoked binding state; identifies SRV versus catalogue ID/fingerprint; binds a recognized profile to its reviewed wire/time contract; requires reviewed scoped overlap and explicit withdrawal. Bad-clock activation, compromise/rollback recovery and actual verifier qualification remain named owner work. The RFC facts were checked directly; algorithm-specific runtime checks are not claimed completed. |
| **R3-F05 — Medium** | §6 allowed documented number/string version adaptation without stating the meaning of the RFC field or the Python coercion trap. Approval also needed a stronger binding to the normalized candidate so a later fetch/build could not silently change reviewed material. Same-endpoint conflicts needed to distinguish unreviewed disagreement from explicitly approved overlap. | §§6.1–6.3 now distinguish source-schema identity, advertised protocol claim and admitted data profile; reject coercible wrong types and duplicate members; retain field provenance; bind predecessor, review, exact snapshot and output identity; and require safe typed generation. Missing/partial source data cannot impersonate retirement or a complete empty catalogue. Parser drift and changed-after-review cases are explicit future controls. |
| **R3-F06 — Medium** | The initial §§4/7 requirement to carry only admitted profiles and reject unsupported profiles could conflate a recognized stored record with an implemented acquisition capability. Reserving Roughtime keys now could accidentally create a current RGT dependency, or encourage silent client-side omission. Version/rollback handling was described without a concrete producer/reader/data matrix. | §§4.3/7 now separate required schema semantics, recognized inert Roughtime data and enabled/qualified runtime use. An early reader can retain understood inactive records; a reader lacking their shape needs an explicitly reviewed compatible export with its own identity. Unknown semantics are refused. The version/process/rollback/history matrix retains current-policy guards, bounds overlapping generations and prevents default-based migration. Exact pre-RGT export behavior remains OPEN-CAT-01/02. |
| **R3-F07 — Medium** | §3 mapped a possible independent signature to OPEN-CAT-02; §8 similarly routed release trust to 02/03. The canonical register assigns format/signature/trust-root strategy to OPEN-CAT-01, roster/profile/cold resolution to 02 and freshness/withdrawal/rollback to 03. Reusing the IDs with different meanings would misroute implementation work. | §§3, 5, 7 and 8 now retain the canonical meanings, including trust under 01/03 and roster/profile work under 02/03 with the relevant TA-W owners. Checked directly against the existing catalogue draft §6. |

Nonblocking corrections also preserve encoding, installed placement and runtime access as separate decisions. Generated tables and CBOR remain leading alternatives, with the CBOR recommendation conditional on a justified separate resource. Typed code generation must safely encode literals and cannot splice upstream strings into executable syntax; no unsafe packed-struct/native-layout interpretation is selected. Combined labels such as `[C7, C12]` were replaced by separately resolvable references, and inherited Markdown hard-break trailing spaces were removed.

## Source checks and limits

Primary sources inspected directly for the material representation/protocol facts:

- [RFC 10049](https://www.rfc-editor.org/rfc/rfc10049.html), especially §§4.1.4, 5.1.4, 5.2.5 and 8.3: profile/time/key identity and the common server-list contract. This confirms the distinctions the schema must retain; it does not qualify an endpoint or implementation.
- [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html#section-4.1.7), especially the negotiated server/port records: catalogue KE identity and actual negotiated NTP destination remain separate.
- [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html#section-4.2): deterministic encoding requires a declared profile; CBOR syntax alone does not provide product admission.
- [RFC 8610](https://www.rfc-editor.org/rfc/rfc8610.html): CDDL describes structure and does not replace the actual bounded consumer contract.

The supplied `roughtime-source-records.json` and `roughtime-key-verification.json` were read and the 13 displayed public-key records retained unchanged. This review did not independently refetch the ecosystem blob, rerun the Base64 checks, repeat operator-page checks, verify signatures or query any time server. The initial draft's key-length/source receipt remains attributable to its original evidence. Its roster remains a documentary candidate inventory; neither profile ambiguity nor operator independence was resolved by this schema repair.

Static verification on the corrected document:

- The initial target and frozen review-input SHA-256 values matched before editing.
- Full original/revised reads and targeted canonical comparisons preserved the owner direction, current Core priority, DATA/open-ID meaning and future-work status.
- `git diff --no-index --check` emitted no whitespace diagnostics; its exit 1 reflects that the two files differ. The static diff reports one maintained file changed, 100 insertions and 47 deletions.
- A search for malformed combined `[C…, C…]` labels returned no matches. No relative-link destinations were added by this role; the existing target files were present in the supplied packet.
- No `docs/SHA256SUMS.txt` was present in the selected snapshot. No whole-bundle integrity, full repository audit, behavioral GREEN or CHECK/TA/ST completion is claimed.

## Remaining choices and cross-file handoff

The following are intentionally unresolved before dependent implementation: exact CBOR/generated-table selection and dependency graph; final schema and integer labels/widths/caps; ID allocation/collision and endpoint-transition representation; precise DNS/IDNA and port admission; exact pre-RGT reader/export compatibility; released providers/use permission and enabled profile set; release integrity/signature coverage and key custody; root rotation/activation/withdrawal and unreliable-clock rollback policy; and historical dictionary/retention migrations. These are already assigned through OPEN-CAT-01–03, DATA-01–09 and TA-W02/03/06/08, with Q/H and Windows lifecycle work retaining their existing owners. No new arbitrary cap or default roster was selected.

Role 1/root should reconcile the existing catalogue task, C/R storage-table rows and development routing with the new document, retaining one task chain. In particular, DATA-05's future runtime condition must coexist with complete schema material now; recognized inert metadata must be distinct from RGT activation; schema/normalization/reference and exact reviewed-import snapshot freezes must precede the relevant implementation; and the separate Q0/Q-J preparation must not be renamed or executed by this catalogue amendment. Those files were not edited by this role.

**Next permitted action:** the coordinator and final reviewer inspect the frozen corrected candidate and reconcile the existing documentation/task references. Python implementation, format measurements, live probes and Windows/package qualification remain later separately scoped work.
````

</details>

<details>
<summary>Role 4 — omissions and unresolved decisions: original report</summary>

````markdown
# Role 4 — omitted scenarios and unresolved product decisions

The proposal prepares a compact catalogue for hundreds of NTP/pool, NTS and Roughtime definitions, with Roughtime root keys and precise protocol bindings present in the first schema. It separates publisher collection and review from deterministic construction and release delivery. Constrained CBOR and generated immutable tables remain alternatives; the future Python tool belongs under `tools/` and does not become a client runtime. The proposal also keeps the large catalogue separate from the small set of active associations. It defines a documentation and preparation direction, not an implemented loader, collector, provider roster, measured format winner or independent updater.

The current system described by the canonical documents has an unprivileged acquisition engine, a separate broker that owns clock effects and mandatory policy, a normal-user UI, and a lifecycle helper. Ordinary NTS, deliberately selected plain profiles and future provisional-date bootstrap have distinct authority. C contains release definitions; Q/H hold advisory history; E retains separate mandatory policy, recovery and secret-state owners. The current Core checkpoint remains the development priority. RGT-0, exact catalogue freshness and key transitions, protected-storage qualification, and complete MSIX lifecycle evidence remain future or separately gated work. Neither a catalogue entry nor a historical success can create current authenticated evidence or independent source credit.

## Review identity, inspected scope and limits

**Completed:** independent, read-only omissions review on 4 November 2026. The only file written by this reviewer is this report.

| Input | Identity and scope |
| --- | --- |
| Selected repository snapshot | `DavidOsipov/NevroTick`, baseline `7550f83292f959a6d0acbb2d396e964a6bcf4b01`, tree `d060c21586745f0e5c341a421230a14c78c6e515`, as recorded in [source-manifest.json](source-manifest.json). This is a selected-file snapshot, not a Git checkout or full implementation audit. |
| Frozen initial proposal | [review-input/server-catalogue-binary-format.md](review-input/server-catalogue-binary-format.md), SHA-256 `b8977ea8c3e2fa1f90511f59f613511983d03a615888a361bbd9c1d1d15adced`. |
| Revised proposal used for final disposition | [work/docs/plans/server-catalogue-binary-format.md](work/docs/plans/server-catalogue-binary-format.md), SHA-256 `77e906705fced8667bd6bcc7f0a70b4c17352e4a1bd78f22438adb1eacf2c4a4`. The working document changed during preparation; the relevant schema, pipeline, lifecycle and open-question sections were re-read at this final identity. Earlier observations are not silently attributed to it. |

Inspected the applicable [AGENTS guide](work/AGENTS.md), documentation routing, repository independent-review and review/handoff guidance, testing/security contracts, crypto-trust guidance, and data-contract/transition guidance. The substantive source set was the DATA inventory and OPEN register, catalogue task and storage tables; time-assurance plan and TA assignments/cases; owner decisions OD-03/04 and relevant R-02/R-04/R-05 research; architecture §§6, 12, 17 and the MSIX gate; selected MSIX and protected-storage lifecycle sections; the wrong-clock activation matrix; and development §§7, 8.4 and 8.7. Existing historical findings in the canonical catalogue task were used for deduplication, not as proof of the current proposal.

No author/private memory, private KB, or other **current** role-review report was consulted. No product code, dependency, benchmark, native/protocol probe, public time-server query or remote mutation was performed. The primary evidence here is the supplied project contracts. Operator publications and protocol claims in the proposal were not independently revalidated in this role. Absence claims below concern the inspected documents and sections, not all 1,072 paths in the manifest.

## Documentation-stage verdict

**REVISE — one small routing amendment, with the existing preparation continuing.** OMI-R01 is a newly exposed product-scope question caused by expanding a small preset into a catalogue of hundreds. Record it explicitly under the existing source/default owner. It does not require choosing the final default, introducing a new policy owner or asking for an immediate product decision.

OMI-R02–OMI-R08 are **already-owned gaps with more discriminating scenarios**, not seven newly discovered defects. They belong at their existing contract-freeze or integration stages. The proposal already acknowledges most of their underlying obligations. No demonstrated implementation vulnerability, native failure or performance conclusion follows from this review.

**One permitted next action:** during the current documentation integration, add the catalogue-to-active-roster question to proposal §8 and task §7, and retain the relevant paired scenarios below with their existing owners. No fresh broad audit, benchmark, collector implementation or live-server campaign is needed to complete that amendment. [Current preparation](work/tasks/server-catalogue-and-statistics.md#7-binary-catalogue-and-future-collector-refinement--4-november-2026); [development priority and execution scope](work/docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent).

## New omission

### OMI-R01 — Define what the large catalogue is allowed to activate

**Classification:** new product-scope/routing omission; Medium consequence before source-policy or scheduler implementation. It is not a claim that the current design already probes every catalogue entry.

**Observation.** The proposal says catalogue cardinality does not enlarge the active set, and that listing a service grants no live authority. Those are correct constraints. It does not identify whether the extra entries form a user-selectable directory, one or more reviewed presets, an automatically traversed replacement pool, or only inputs for a particular future bootstrap branch. The pre-existing R-04 work selects a small qualified provider preset; F-03 selects a principal and supporters from admitted source estimates. Neither operation by itself defines promotion of an unused catalogue entry into the configured acquisition set. [Proposal §§2 and 4.3](work/docs/plans/server-catalogue-binary-format.md#2-resource-model-and-physical-allocation); [R-04](work/docs/nevrotick-research-handoff.md#6-r-04--default-servers-and-the-supplied-nts-list); [F-03](work/docs/windows-time-features.md#f-03--select-a-principal-source-and-compatible-supporters).

**Why the distinction matters.** A developer can obey a sixteen-active-source cap while sequentially scanning hundreds of entries after each failure. Another can take the first four entries in deterministic artifact order, accidentally giving sorting a source-selection meaning. A third can treat every admitted service as permission to replace an administrator's selected source. These are plausible divergent implementations of the missing relationship, not observed behavior. They differ in traffic, provider load, cold-start latency, privacy, stability and default-source diversity. The architecture already forbids accidentally turning list order into security priority. [Configuration semantics](work/docs/windows-time-architecture.md#122-configuration-semantics); [source use and permissions](work/docs/plans/server-catalogue-and-statistics-draft.md#data-01--curation-inputs-admission-research-and-service-use-permission).

**Smallest correction.** Add one explicit open preparation item: identify the owner and representation of reviewed preset membership, distinguish catalogue membership from configured/active membership, and state whether automatic replacement is in scope. If replacement is selected later, freeze its eligible population, trigger, stable selection/reselection rule, finite traversal and provider-budget continuity. Do not import random-sampling guarantees from the separate Khronos reserve. A reasonable first-release recommendation is a reviewed small preset with other entries inert until separately selected; that is a recommendation, not an adopted default.

**Owner and stage.** Existing source/default and acquisition owners: OPEN-CAT-02, R-04, OD-03, DATA-03/04/06/17/20; P0/P2 source admission and the shipped-preset gate. If preset membership belongs in C, its meaning must be declared before that schema is frozen. If it belongs in separately authorized policy, name that binding instead of adding a second copy. Core's current numerical selection work need not wait. [OD-03](work/docs/nevrotick-owner-decisions.md#5-od-03--the-pool-preference-and-an-nts-preset); [current task ownership](work/tasks/server-catalogue-and-statistics.md#2-ownership-and-authority).

**Discriminating case.** Reorder an otherwise identical catalogue and add many valid but unselected entries. The same approved preset and ordinary traffic remain unchanged. Pair this with loss of one selected operator: the declared policy either uses a specifically authorized replacement within its budget or reports reduced/insufficient availability; it does not infer permission from position, proximity, low latency, catalogue membership or the need to fill four slots. Use a healthy preset with compatible, independently evidenced operators as the positive control. No numeric quorum or automatic replacement policy is selected by this case.

## Already-owned gaps: retain these concrete cases

The IDs below keep each scenario stable for integration. They refine existing assignments rather than creating competing OPEN, TA, ST or CHECK identities.

### OMI-R02 — Compose a new C with retained E before calling the effective policy usable

**Already owned by:** DATA-06 and OPEN-CAT-01/03, configuration/broker and lifecycle owners; contract freeze before P3/S3.3 policy integration and P6 package migration. DATA-06 explicitly leaves catalogue-refresh/local-override precedence open. The revised proposal also requires a missing-target reconciliation rule. [DATA-06](work/docs/plans/server-catalogue-and-statistics-draft.md#data-06--authorized-sourcesecurity-policy-and-local-overrides); [policy overlay and stable identities](work/docs/plans/server-catalogue-binary-format.md#41-records-and-their-authority); [transition guidance](work/.agents/skills/implementation/evolve-data-contracts/references/transitions.md).

**Scenario to add to that freeze.** C2 is well formed but removes or replaces an endpoint referenced by intact E1. E1 also contains a local disable, a pin restriction if adopted, or a key withdrawal. A source-policy edit is concurrently pending under E1's revision. Package replacement and broker policy publication are not one transaction merely because both have generation numbers. A valid new catalogue can therefore coexist with a policy that has missing or incompatible references.

**Smallest next action/control.** Add C1/E1, C2/E1, C1/E2 and C2/E2 to the required compatibility table, including interrupted migration and stale compare-and-swap. Name the owner of the composed effective view and its publication point. A compatible catalogue addition must preserve unchanged authorized policy. Removal must yield the frozen explicit result, not remap a restriction to a similarly named source, drop it, lower agreement minima or create permissive defaults. The precise permissible recovery remains an existing decision; this review does not require a new transactional store. The proposal's reader/data table is a useful start, but does not itself settle this two-owner composition. [Proposal §7](work/docs/plans/server-catalogue-binary-format.md#7-compatibility-failure-and-release-lifecycle); [architecture §12.2](work/docs/windows-time-architecture.md#122-configuration-semantics).

### OMI-R03 — Give release-bound withdrawals an end-to-end operational outcome

**Already owned by:** OPEN-CAT-03, release/trust owners, TA-W02/06/07 and the selected package owner. This is a pre-use freshness/key-transition contract, followed by release and native qualification; it is not a reason to add an independent updater. [OPEN register](work/docs/plans/server-catalogue-and-statistics-draft.md#6-open-decisions-and-owner-questions); [proposal key lifecycle](work/docs/plans/server-catalogue-binary-format.md#51-root-keys-and-protocol-profiles); [time-assurance key maintenance](work/docs/plans/time-assurance-integration.md#71-admission-and-independence).

**Scenario.** A public root is withdrawn after an installed release, while the client is offline or cannot complete its package trust/update path because its date is badly wrong. Distinguish a first installation with no protected revision knowledge, an existing installation with an intact withdrawal/floor, a restored old package with current E, and a whole-machine rollback. These cases do not have the same evidence. A locally known withdrawal cannot be ignored; an unknown newer withdrawal cannot be detected merely by verifying an old package. A wall-time expiry based on the clock being repaired supplies no independent answer.

**Smallest next action/control.** Prepare a short lifecycle table from publisher decision → reviewed corrective release → installed client adoption, with planned overlap, emergency withdrawal, update unavailable, and authorized recovery outcomes. State any supported staleness/reaction claim and what evidence supports it, or leave that claim unavailable. Include “old but still permitted” beside “known withdrawn”; failed Roughtime use must preserve the independently applicable ordinary-NTS policy. Do not promise unconditional freshness or rollback resistance, invent an expiry constant, or make reinstall/reset erase known restrictions. [Bad-clock release matrix](work/docs/windows-time-signing-guide.md#51-wrong-clock-and-offline-activation-matrix); [MSIX update/recovery distinctions](work/docs/plans/windows-msix-store.md#distinguishing-lifecycle-cases).

### OMI-R04 — Test the complete cold-start dependency path, not only possession of keys

**Already owned by:** OPEN-CAT-02, TA-W02/06/07, the resolver/network owner and CHECK-9/18/23/27. The time-assurance plan explicitly calls for resolving the endpoint/DNS bootstrap cycle; the signing guide separately covers the ability to start the code needed for recovery. [Time-assurance §7.1](work/docs/plans/time-assurance-integration.md#71-admission-and-independence); [Windows verification gate](work/docs/plans/time-assurance-integration.md#9-windows-gate-and-explicit-decision-alternatives); [activation matrix](work/docs/windows-time-signing-guide.md#51-wrong-clock-and-offline-activation-matrix).

**Scenario.** On a cold installation there is no eligible NTS cache, trusted time floor or usable DNS cache; the clock is grossly wrong, and the catalogue contains names and valid roots. Name resolution or the platform trust path needed to launch/update the product may itself require unavailable time-dependent infrastructure. Having an embedded Roughtime root solves only one link. A private-address result, stale optional address hint, or blocked address family is not a permitted destination merely because bootstrap needs it.

**Smallest next action/control.** Put one complete dependency row in the existing source freeze: installed artifact admission/startup → permitted resolution/address admission → Roughtime → isolated NTS identity/revocation validation → fresh protected observations/agreement → broker gate. Use an admitted cold-resolution path as the healthy neighbor and a date-dependent/unavailable path as the refusal case. Include unavailable UDP with a qualified advertised TCP alternative where the selected profile supports it. Keep bootstrap hints, actual network capability and current time evidence separate. This is a paper/fixture contract first; it grants no DNS-policy relaxation or native probing authority.

### OMI-R05 — Count usable groups after all profile and policy filters

**Already owned by:** OPEN-CAT-02, OD-03/R-04 and TA-W02/06, with TA-C01/02/17. The proposal's thirteen documentary key rows are not thirteen usable independent RFC-v1 sources; its inventory identifies only two rows with explicit RFC-v1 advertisement and leaves other profiles unresolved. That is a statement about the document, not a fresh census or a new quorum choice. [Documentary key inventory](work/docs/plans/server-catalogue-binary-format.md#52-initial-public-key-evidence-inventory--4-november-2026); [TA acceptance cases](work/tasks/time-assurance-integration.md#4-independent-acceptance-cases).

**Smallest next action/control.** Before claiming a usable preset, produce a compact admission matrix by logical operator/dependence group, protocol/profile, authorized key, compatible time policy, permitted use, transport/address-family eligibility and unresolved evidence. Report the remaining usable groups for each selected role after exclusions. Do not merely report host, key or row totals. Pair the required number of compatible independent groups with one fewer group represented by extra aliases, overlapping roots or transports. Cross-protocol overlap and bootstrap ancestry must not manufacture additional support. If no suitable default set can be qualified, the feature plan already classifies that as an onboarding/release gap; it does not authorize weaker minima. [Feature default gate](work/docs/windows-time-features.md#5-default-settings-and-authority); [dependence accounting](work/docs/plans/time-assurance-integration.md#51-prevent-manufactured-corroboration).

### OMI-R06 — Preserve resource and retry obligations when candidates or generations change

**Already owned by:** DATA-20, OPEN-CAT-01/OPEN-DATA-03, engine/resource owners and TA-W08; CHECK-21/27. The revised proposal accounts for encoded/decoded overlap and concurrent old/new backing storage. Time assurance already requires one aggregate budget and actual native completion. [DATA-20](work/docs/plans/server-catalogue-and-statistics-draft.md#data-20--retention-policy-resource-accounting-and-collection-health); [aggregate budget](work/docs/plans/time-assurance-integration.md#72-one-aggregate-budget); [native custody](work/docs/windows-time-architecture.md#171-bounded-async-and-native-work).

**Scenario.** A large, semantically valid catalogue contains expensive endpoints; an old-generation verification job remains live while its waiter is cancelled, and a replacement or retry wants a new slot. Sequential traversal can obey the active-count limit while consuming excessive cumulative DNS/KE/signature work. Source replacement can also lose a provider backoff/use restriction if those obligations are accidentally tied only to an ephemeral index or generation. This is distinct from malformed-input rejection.

**Smallest next action/control.** At the admitted loader/acquisition freeze, bind total bytes/work, reference fan-out, simultaneously live views and actual job ownership to named limits; declare behavior when an old view/job cannot retire. Preserve applicable provider restrictions through authorized source transitions. Pair a maximum valid catalogue and a released-job neighbor with capacity-plus-one, retained-job and finite traversal-exhaustion cases. Resource refusal is local insufficiency, not a server dishonesty score. A release-only catalogue requires no hot-reload implementation to satisfy this case; apply it only to the lifecycle actually selected.

### OMI-R07 — Keep retained labels and safety obligations useful under bounded history loss

**Already owned by:** DATA-18/19, OPEN-DATA-02/03/04/05/06, storage and TA-W02. Stable identity, revision-bound dictionaries and explicit unavailable interpretation are already present in the revised proposal. [History manifests](work/docs/plans/server-catalogue-and-statistics-draft.md#data-19--historyexport-manifests-dictionaries-migrations-and-gaps); [conflict ownership](work/docs/plans/server-catalogue-and-statistics-draft.md#data-18--sparse-evidence-and-unresolved-security-conflicts); [proposal §7](work/docs/plans/server-catalogue-binary-format.md#7-compatibility-failure-and-release-lifecycle).

**Smallest next action/control.** Use two catalogue generations and one old observation: rotate a root, split/merge an operator relationship, retire an endpoint, then prune the old dictionary or clear Q/H. A healthy retained definition must still explain the old measurement. If the permitted descriptive context is gone, report interpretation unavailable; never substitute current labels, move the score to a different operator/key, or restore old keys as current authority. Separately test that any safety minimum actually adopted by TA-W02 survives optional-history loss. Do not pre-empt that owner by mandating permanent quarantine or full raw-response retention. This belongs before Q/H migration/reset integration, not in the current format discussion.

### OMI-R08 — Catalogue failure must not obscure the existing removal/recovery gap

**Already owned by:** W11-W05/W11-W10, the separate mandatory-state owner, and applicable ST-W03/06; CHECK-13/15/23. The MSIX plan already records unresolved supported ordering for independent Settings uninstall, Reset and last-user/all-users removal. It would be incorrect to report that as a newly discovered catalogue vulnerability. [MSIX removal boundary](work/docs/plans/windows-msix-store.md#concrete-removal-and-recovery-boundary); [protected-storage ownership](work/docs/plans/windows-protected-storage.md#2-owners-data-classes-and-restoration).

**Smallest next action/control.** Carry one catalogue-specific fault into the existing lifecycle matrix: after package replacement, the new catalogue or composed source policy cannot be admitted while a prior time-ownership/restoration obligation remains. Compare normal successful update/removal with interruption and Reset. Verify that optional C/Q/H repair does not delete the sole mandatory recovery record, disable the last invocable recovery owner or let old-package cleanup act underneath its successor. The native evidence remains at the existing package/takeover gates; publishing this discussion does not depend on resolving that broader platform problem.

## End-to-end trace and non-findings

| Flow boundary | Current documentary coverage | Remaining routing from this review |
| --- | --- | --- |
| Import → normalized candidate | Bounded source adapters, provenance, strict types/duplicates, conflicts and partial-fetch handling. [§6.1–6.2](work/docs/plans/server-catalogue-binary-format.md#61-source-adapters) | Exact adapters/caps remain their existing future freeze; no separate omissions finding. |
| Candidate → review → artifact | Exact predecessor/candidate approval, changed-input refusal, deterministic generation, semantic diff and artifact identity. [§6.2](work/docs/plans/server-catalogue-binary-format.md#62-bounded-pipeline-and-publication-boundary) | Preserve valid-source and changed-after-review controls. These obligations are now explicit; do not report them as still missing. |
| Artifact → admitted client records | Exact release trust, bounded complete admission, immutable view and distinct recognized metadata/runtime capability. [§§3–4](work/docs/plans/server-catalogue-binary-format.md#3-representation-candidates) | Representation/provider measurements remain OPEN-CAT-01. No evidence supports declaring CBOR or generated tables the measured winner. |
| Admitted records → configured/active work | Current role/policy/destination/key/budget checks; default-four/maximum-sixteen remain separate from catalogue size. [§4.3](work/docs/plans/server-catalogue-binary-format.md#43-source-records-do-not-construct-live-authority) | **OMI-R01** names the missing product relationship. OMI-R05/06 refine existing diversity and resource gates. |
| Active results → history | Historical context, identities, missingness and generation binding; Q/H cannot construct live authority. [DATA-09–19](work/docs/plans/server-catalogue-and-statistics-draft.md#data-09--event-identity-time-bases-and-applicability-generations) | OMI-R07; retain earlier DATA/EXP/SEC preparation findings without rerunning or relabelling their pilot. |
| Release change → installed C/E → running consumers | Explicit old/new reader combinations, coherent views, old-result refusal, unchanged mandatory authority. [§7](work/docs/plans/server-catalogue-binary-format.md#7-compatibility-failure-and-release-lifecycle) | OMI-R02/03, including update unavailable and known withdrawal. |
| Repair/reset/remove | Existing separation of optional data, mandatory state and supported recovery owner. [MSIX §5](work/docs/plans/windows-msix-store.md#5-recovery-data-and-update-semantics) | OMI-R08; native qualification remains unresolved under its original owners. |

The initial proposal's imprecise OPEN-CAT routing is corrected in the reviewed working version: OPEN-CAT-01 owns representation/trust-root strategy, OPEN-CAT-02 roster/profile/cold resolution, and OPEN-CAT-03 freshness/withdrawal/rollback. The working document also explicitly separates inert recognized Roughtime metadata from runtime qualification, binds root authorization to exact source/endpoint/profile relationships, and supplies a reader/version matrix. Those are observed documentary improvements at the recorded hash, not executed enforcement.

The strongest conclusion supported by this review is narrow: the design can proceed as a lightweight release-bound catalogue without adopting another database, compression dependency, updater, daemon or Python runtime on Windows. Its largest remaining practical uncertainty is whether the intended usable presets and recovery routes can meet their existing trust, diversity, permission and lifecycle contracts. The requested binary encoding does not resolve or worsen those questions by itself. The next documentation change should make the activation relationship explicit and preserve the named owners; implementation and qualification remain at their declared stages.
````

</details>

<details>
<summary>Role 5 — final integration and publication review: original report</summary>

````markdown
# Role 5 — final documentation integration and publication review

The change defines a release-bound binary catalogue for hundreds of NTP/pool, NTS and Roughtime source records while keeping actual acquisition bounded. It requires complete Roughtime public-key, profile and endpoint bindings in the first schema, compares constrained CBOR with generated immutable data, and assigns future collection and deterministic construction to publisher-side Python tooling. The collector, codec, shipped roster and resource results are not implemented by these documents. The review packet supplies the original design, four completed reviews, their documentary corrections and an existing task/development chain to reconcile.

The current authorities retain strict JSON IPC, machine-policy and journal contracts; separate acquisition and clock-broker authority; reviewed source/default policy; mandatory state versus advisory Q/H; and the selected complete-MSIX preparation direction. Current Core work remains first. Ordinary NTP/NTS catalogue preparation can precede the later RGT-0 implementation, while recognized Roughtime metadata grants no verifier capability or fresh authenticated evidence. Catalogue, preset, configured and active membership require separate meanings. The scoped Sandbox disposition, blocked artifacts/runners, storage pause, Windows/package gates and earlier Q0/Q-J findings retain their existing owners.

## Identity, inspected scope and actual completion

**Completed:** the fifth role's final documentary review, integration and bounded static/source checks on 4 November 2026. **Verdict:** accept the integrated documentation for the explicitly authorized repository-publication stage, subject to the actual remote ref/tree/content checks described below. This is no product release, format selection, implementation qualification or native pass.

The first launch failed with a context-length error before a usable review. It is **not run**, not a clean review. This report belongs to the resumed assignment. The role intentionally read all four current reports and the integrated candidate; it is a final synthesis with independent evidence checks, not a blind fresh-opinion vote. A read-only helper checked links and tables on the pre-integration candidate; those counts were rechecked after the substantive edits. It is not a sixth requested review role. No private KB or Memory work was performed.

The supplied baseline is `DavidOsipov/NevroTick@7550f83292f959a6d0acbb2d396e964a6bcf4b01`, actual tree `d060c21586745f0e5c341a421230a14c78c6e515`. `git rev-parse --show-toplevel` independently returned “not a git repository”; this is a selected-file snapshot, not a local checkout. All 38 available baseline files were independently hashed as Git blobs and matched the supplied nontruncated tree inventory. That inventory has 1,072 blob paths; path enumeration is not a whole-repository source audit. It identifies no nested AGENTS applicable to the eight changed paths and no `docs/SHA256SUMS.txt`.

Inspected the actual reports `review-1-consistency.md`, `review-2-formats-storage.md`, `review-3-schema.md` and `review-4-omissions.md` in full and verified their SHA-256 identities against the coordinator record. Read the complete 50,748-byte corrected proposal at `77e906705fced8667bd6bcc7f0a70b4c17352e4a1bd78f22438adb1eacf2c4a4`, the relevant final changes and every changed hunk in the seven existing files. Applicable guidance was AGENTS, documentation routing, planning/review-handoff, data-contract/transition, implementation and testing/independent-review contracts. Canonical checks included DATA-01–09 and OPEN-CAT-01–03; R-04/OD-03; architecture source/default, strict-data and resource contracts; the current development checkpoint; catalogue/Q0/Q-J preparation; and time-assurance ownership, acceptance and dated census. Historical reviews were checked for preservation, not treated as current approval.

The authorized write set remains exactly eight Markdown files. No application code, future collector, dependency, benchmark, live time-server exchange, Windows operation, signing key or access-control change was introduced. The final publisher uses GitHub Git-data operations because no checkout is present; it must not invent local history or claim that local hooks ran.

## Final findings, corrections and retained choices

### OMI-R01 — inherited routing finding, documentary correction verified

The role-4 omission remained in the inspected candidate: a bounded active set did not define which catalogue entries could become a reviewed preset or configured acquisition sources, or whether automatic replacement was part of the product. This is a source-policy question, not evidence of an implemented catalogue sweep.

The corrected proposal §§4.3/8, canonical OPEN-CAT-02 row, storage terminology, task §7 and development §§7/8.7 now route that relationship to OPEN-CAT-02, R-04 and OD-03. The source/default owner must identify the preset's C/policy representation and stable-ID binding, then decide whether replacement is in scope. A later selected replacement policy needs its eligible population, trigger, stable reselection, finite traversal and continuing provider budgets. No default, quorum, traversal permission or new Core prerequisite was chosen. The task retains the reorder/add-unselected-records control and selected-operator-loss case.

**Disposition:** the documentary omission is resolved; the product decision remains open before its dependent schema or acquisition implementation. Reopen the documentary finding if later text again treats catalogue position/membership or an empty active slot as authority.

### R5-F02 — low-severity precision correction to resource accounting

The corrected proposal still abbreviated architecture §17's 32-MiB goal as combined-service memory. The normative metric is combined engine/broker **steady-state private bytes**, with peaks separately reported. A mapped/generated representation must not appear cheaper merely by moving memory outside that metric.

Proposal §2 now uses the exact metric and distinguishes mapped/image residency. Section 8's future comparison includes encoding, placement and runtime access; incremental code/data, complete compressed and installed package costs, old/new staging, startup/copy/allocation work, steady lookups, private bytes, relevant residency and peak overlap. Q/H write/checkpoint/rewrite and migration costs retain their separate workload accounting. No target or measured number was changed. This incorporates R2-R01–03's useful refinements without expanding the existing pilot.

**Disposition:** corrected and checked against architecture §17; still no performance winner or executed resource result.

### Preceding corrections verified against the integrated text

| Stable identities | Final verification and scope |
| --- | --- |
| R1-01; R2-R01/02 | Binary shipment is the direction; generated immutable data and constrained CBOR remain leading alternatives, with the CBOR recommendation conditional on a separate resource. Strict JSON IPC/policy/journals and Q0/Q-J remain separate. |
| R1-02; R3-F06 | The first schema carries required Roughtime root/profile/binding data, while recognized inert metadata, explicitly reviewed export compatibility and enabled/qualified runtime are distinct. RGT-0 is no new ordinary-NTP/NTS prerequisite. |
| R1-03; R2-F01; R3-F07 | Canonical OPEN-CAT meanings are retained: 01 representation/signature/trust-root strategy; 02 roster/profile/cold resolution; 03 freshness/withdrawal/rollback. |
| R1-04; R2-F02; R3-F01 | The refined logical schema explicitly retains country/region-or-unknown, appropriate scope/provenance and time-scale/leap/smear compatibility. |
| R3-F02/03/04 | Closed variants, stable layered identities, normalization/reference integrity, exact key/source/endpoint/profile joins and separate curation/key-transition states address the documented ambiguities. Actual encoding, bounds and verifier controls remain future freezes. |
| R1-05; R3-F05 | Future tooling remains at lowercase `tools/server-catalogue/build_catalogue.py`, with bounded acquisition, conflict/partial-result handling, exact reviewed input identity, deterministic safe generation and separate release authority. |
| R1-06 | Original reports, the older candidate scopes, dated census and Q-pilot evidence remain distinguishable. Current statements no longer claim every older current-text section is byte-identical after this amendment. |
| OMI-R02–08 | Task §7.2 retains the concrete C/E composition, withdrawal/update-unavailable, cold-start dependency, usable-group, actual-resource-custody, historical-interpretation and mandatory-recovery cases at their existing owners/triggers. These refine existing gaps; none is a newly invented current-publication blocker. |

The substantive findings were checked against the actual revised text, not closed by reviewer agreement. There remains no final schema/width/cap/normalization freeze, production reader or generator, approved provider/default roster, signature/freshness/rollback policy, measured representation winner, qualified package lifecycle, Store acceptance or protocol availability claim.

## Independent source checks

The immutable ecosystem blob was fetched directly through GitHub, stored losslessly and rehashed to `cd9eb5043fd7ff77845333ffe97b8318303ff94e` (5,617 bytes; SHA-256 `f4602b67ddfb3b8c550a9f1bb1bb8fd1ab5c88c29ffb4716ad989aa26a444a70`). Its 13 Ed25519 records exactly match the supplied extraction and all 13 endpoint/key pairs in the final proposal. Strict Base64 decoding and re-encoding verified 32-byte canonical strings for each. The unsupported ML-DSA record remains excluded. Source: [pinned ecosystem](https://github.com/tannerryan/roughtime/blob/09ffa30abfb8c9ea22b81858edf393f6c5a7ddef/ecosystem.json). This checks public source bytes and transcription, not mathematical key validity, authorization, signatures or server operation.

Directly inspected [RFC 10049](https://www.rfc-editor.org/rfc/rfc10049.html), including §§4.1, 5.1.4, 5.2.5 and 8.3. Its experimental v1 identity, long-term-key/SRV distinction, timestamp/radius units and common-list version meaning support the corrected distinctions. Directly inspected [RFC 8949](https://www.rfc-editor.org/rfc/rfc8949.html) and [Protobuf's canonicalization documentation](https://protobuf.dev/programming-guides/serialization-not-canonical/); these supply representation constraints, not a measured product ranking or an adopted decoder.

Reopened [TimeNL](https://nts.time.nl/), [Tanner Ryan](https://time.txryan.com/), [roughtime.se](https://roughtime.se/), and Cloudflare's [usage](https://developers.cloudflare.com/time-services/roughtime/usage/) and [deprecation](https://developers.cloudflare.com/time-services/roughtime/deprecation/) pages. The checked operator publications support the two explicit RFC-v1 endpoint/key advertisements, roughtime.se's draft-19 claim, Cloudflare's current port/key and beta caveat, and the separate 2024-06-30 deprecated tuple. These were webpage reads only; no DNS/NTP/NTS/Roughtime service exchange occurred. Other rows remain pinned-community-list evidence in this check, and no operator-independence inference was promoted into qualification.

## Static checks and exact candidate identities

Checks on the substantively integrated candidate, including original reports 1–4 but before appending this report and inserting its hash:

- Exactly the eight intended Markdown paths differ from the baseline; no other work-tree file is changed relative to that selected snapshot.
- All **37 affected inline relative-link occurrences** on added/replaced lines resolve, including their anchors against actual local target headings. No missing or uninspected new anchor remains. Fenced historical packet links are deliberately literal.
- The new proposal's **9 tables and 81 data rows** have consistent column counts. All **14 C reference labels** resolve, and no combined `[C7, C12]` label remains.
- All changed files decode as UTF-8 without replacement characters. Original reports 1–4 are embedded byte-for-byte with their recorded hashes.
- All six pre-existing historical details blocks across the draft/task are unchanged. The draft's complete §9, Q-pilot §10.4 onward, current Core section-7 prefix, TA acceptance cases/review suffix and substantive dated census body are unchanged.
- Before raw-packet insertion, `git diff --no-index --check` produced no diagnostics (exit 1 for differences). After insertion it returns exit 3 solely for **five original role-2 header lines with Markdown hard-break spaces**. Their original bytes are intentionally retained; there is no new prose whitespace defect. No zero-diagnostic claim is made for that full raw-inclusive diff.

| Integrated path | Bytes | SHA-256 |
| --- | ---: | --- |
| `docs/plans/server-catalogue-binary-format.md` | 52,958 | `7eb73c3fcec40ba85e82dbda7d984ef75a959fd3976f8801dd87227fb8aa5aa3` |
| `docs/README.md` | 47,260 | `d7aa49923c93f1d94dcd4a16552829b7aacfd0a335a632ff14029f8d559a1dda` |
| `docs/plans/server-catalogue-and-statistics-draft.md` | 154,867 | `d1d355cf1d9602474bcb494b600ea27f6f712889e55583822ee3f276f13a5f32` |
| `docs/plans/server-catalogue-storage-tables.md` | 31,142 | `f3370624f4a1c7af7f718b1a4d857bec4a7ef09dce0576d954cffc95b432dfdb` |
| `docs/plans/development-plan.md` | 140,507 | `f4bfba76873541cba40af99d36914e642852d56907d96dee7695534b6f6f110b` |
| `docs/plans/time-assurance-integration.md` | 35,947 | `bae863263e7728e75bb105b5fe047ad6015e0f571ac1b36a0f3762e108b21e2a` |
| `tasks/time-assurance-integration.md` | 29,944 | `42bb684ad95c04cd81d97fe9f597ae063ae163d89668be6da10b6e776d231a0d` |
| `tasks/server-catalogue-and-statistics.md` — before this report/hash insertion | 157,755 | `5fc6ae69cfd3f2cbdd16f54210f6211de44aecd48f87c7940ff8d1be38ffdd7f` |

The task necessarily changes when this exact report and its hash are appended. Its final bytes and all eight remote blob identities belong in the publication receipt. This avoids a self-hash/commit cycle; it does not attribute the preceding task hash to the published file. The final assembly must preserve this substantive task prefix except for replacing the explicit role-5 hash placeholder, preserve all five raw reports exactly, and add no new active navigation from their fenced contents.

## Publication boundary and next work

The repository update itself is explicitly authorized. At report freeze, the fresh remote main ref still matches the baseline; the publisher will re-read it immediately before constructing the commit. Build from the actual fresh commit's tree, compare all proposed paths against baseline, reconcile any concurrent changes and preserve every unrelated tree entry/mode. Verify the new tree differs only at the eight intended paths and each blob matches the final local bytes. Create a single-parent commit, then update `main` with `expected_sha` and `force:false`. A lease rejection requires a new read/reconciliation, never a forced overwrite.

Publication is **not yet claimed by this pre-publication review report**. The requested receipt must subsequently record the actual parent/new commit/tree, final path/blob/SHA-256/byte map, ref/parent/content verification and any limitation. Re-fetch the commit, ref and final blobs before reporting success. Automatic-review rejection, if any, must be reported accurately; no approval workaround is authorized.

After this documentation milestone, the existing catalogue task owns the next relevant contract/fixture preparation. OPEN-CAT-01–03, R-04/OD-03, TA-W and W11/ST-W owners retain their named choices and evidence. Current Core continuation and the separate Q0/Q-J preparation stay as recorded. No extra broad review campaign, benchmark, collector implementation or live probe is needed to publish these corrected discussion documents.
````

</details>
