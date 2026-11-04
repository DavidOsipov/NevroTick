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
