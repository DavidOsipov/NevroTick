# Roughtime and Khronos/Ananke coordination task

**Date:** 3 November 2026, Asia/Tbilisi<br>
**Source baseline:** `DavidOsipov/NevroTick@d868da58ee98f0d66f0b41cc74627a5c47a6cf8d`<br>
**Design owner:** [NV-TA-001](../docs/plans/time-assurance-integration.md)<br>
**Mode/status:** documentation and future work definition; protocol/runtime implementation and qualification have not run.<br>
**Coordinator:** future numerical/protocol owner, with independent security and Windows reviewers; David owns material policy choices.

## 1. Request, scope and retained authority

Record the Roughtime research and design how it cooperates with Khronos/Ananke.
Reuse one evidence chain for [KHR-0](../ROADMAP.md#khr-0--evaluate-independent-time-cross-checking)
and [RGT-0](../ROADMAP.md#rgt-0--roughtime-bootstrap-for-a-badly-wrong-clock), keeping
their prerequisites, implementation decisions and exits distinct. Ananke informs
pool admission; it is not a fourth measurement protocol. Existing engine policy
and the broker retain correction ownership.

This task does not activate future implementation, change the next Core task,
select providers/dependencies, change public licensing or permit direct clock
correction from Roughtime/Khronos. Windows P0–P7 → KHR-C01 → Android and the later
RGT-0 → CHR-0 sequence remain. Optional joint-monitor evidence must not make
RGT-C01 depend on adopting Khronos.

Read AGENTS.md, the documentation guide, the architecture, NV-TA-001 and the
current [development next-task record](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
before execution. The scoped owned-Sandbox permission and separately admitted
guest work do not release the blocked S134 artifact, general host Windows
execution, host security/service/clock changes, the separate AES fuzz hold or the
storage experiment pause. Reconcile current gates at the dependent checkpoint.

## 2. Current evidence and reconciliation

**Completed:** source/documentation research and this coordinated future plan.
Reviewed RFC 10049, RFC 9523, the original Ananke paper, existing NTS recovery and
broker contracts, and retained dated Roughtime implementation/provider findings.
The plan's sources distinguish specification, inspected implementation and
unverified operational claims.

**Not established:** product implementation, exercised TA-C cases, provider
interoperability, native Windows verification/recovery, performance or either
roadmap exit. The earlier Sandbox readiness receipt belongs to its own candidate
and does not qualify this task's future binaries.

| Predecessor obligation | Disposition in this task |
| --- | --- |
| KHR-0 evaluation and its four outcomes | Retained; linked to one explicit task and joint evidence model |
| RGT-0 interval → NTS → broker recovery | Retained; no preliminary global clock write adopted |
| NTS quorum, source minimum and 16-source/resource limits | Retained; auxiliary mechanisms receive no extra vote or separate unbounded budget |
| OPEN-8 and saved-floor supersession | Retained as dependent gates; external agreement does not close them |
| Runtime comparison and feedback | Added proposed dependency/epoch contracts and combined hostile/valid cases |
| Current development and execution order | Retained; documentation completion is not permission to run future work |

## 3. Work assignments and finite exits

TA-W identifiers are local assignments. Each implementation owner freezes the
exact source/graph, selected cases, finite work budget and environment before its
run. Later unresolved choices do not block this documentation handoff.

| ID / owner | Work and required input | Observable exit / current status |
| --- | --- | --- |
| **TA-W01 — coordinator/source reviewer** | Reconcile the request, primary sources, existing KHR/RGT stages and affected consumers | This plan/task and canonical routing exist; source findings are separated from qualification. **Source/design work complete; review receipt below owns handoff evidence** |
| **TA-W02 — protocol/security owner** | Freeze evidence fields, dependency groups, common frame, interval admission, generations and conflict-clearance rules; prepare concrete cards for unresolved NV-TA-001 §9 choices | Reviewed contract and independent expected-result table for the selected slice; exact owners and forbidden effects. **Future, not executed** |
| **TA-W03 — Roughtime/dependency owner** | Select exact wire/client/crypto graph under CHECK-26; implement bounded inert decoding and mandatory-root-key evidence construction after TA-W02 | Valid response accepted; missing/wrong root, version/unit errors, malformed/replayed input and invalid intervals cannot create authenticated evidence. **Future** |
| **TA-W04 — core/engine owner** | Model joint receipt consumption, epoch changes, ancestry, uncertainty and advisory outcomes with independent fixture truth | TA-C01–TA-C07/10/13/16 controls reach the policy consumer; no duplicate credit, stale admission or extra clock controller. **Future** |
| **TA-W05 — KHR numerical/security owner** | After the Windows-plan handoff, compare qualified population/sampling candidates, Ananke-inspired admission and bounded expanded checks against the unchanged baseline | KHR-0's detection, false-alarm, latency and cost evidence plus `ADOPT_LIMITED`, `DEFER` or `REJECT`; adoption needs qualified bounded implementation. **Future KHR phase** |
| **TA-W06 — RGT transport/trust owner** | Admit candidate key/endpoint catalogues, update/rollback rules and finite transport/interoperability work; controlled counterpart first, public queries only within admitted limits | Exact protocol/operator/key compatibility, fresh interval and refusal evidence; terms and availability limits retained. **Future RGT phase** |
| **TA-W07 — Windows/NTS owner** | With admitted artifacts and owned disposable environment, exercise the actual verifier and NTS handoff for wrong dates under OPEN-8 and R-02/R-05 | Valid supported-date recovery and explicit failure for bad identity/revocation/era; no unauthorized effect, observed cleanup/restoration. **Conditional; no execution authorized by this task alone** |
| **TA-W08 — engine/performance/security owner** | Exercise the adopted joint profile's resource, cancellation, conflict and broker-effect boundaries; include disabled/deferred watchdog variant | TA-C08/11/12/14/15 plus relevant integrated cases; aggregate limits and actual effects observed. **Future; environment follows the selected claims** |
| **TA-W09 — coordinator/independent reviewer** | Reconcile candidate-specific evidence and remaining policy/operational gaps | Separate KHR-C01 and RGT-C01 dispositions; preserve CHR-0 ordering and user-visible claims. **Future; no combined automatic pass** |

TA-W03/04 do not require a full production service or public network access.
TA-W05 can compare an existing qualified acquisition baseline and a simulated
future Roughtime variant without making RGT-C01 an Android prerequisite.
TA-W07 requires the whole NTS/Windows path, not a successful Roughtime packet
exchange. Split an oversized assignment into bounded slices inside this task.

## 4. Independent acceptance cases

All cases below are **planned, not run**. Pair each hostile/failure input with the
stated valid neighbor. Use independent fixture truth, not another translation of
the production algorithm. The existing [CHECK catalog](../docs/windows-time-architecture.md#202-acceptance-catalog)
and [testing contract](../.agents/skills/testing/CONTRACT.md) retain their full
obligations, including applicable coverage/mutation/native evidence.

| ID / owner / layer | Controlled case and valid neighbor | Expected result and forbidden effect | Canonical mapping |
| --- | --- | --- | --- |
| **TA-C01 — core/security; portable** | One operator appears as NTS, Roughtime and watchdog; control uses independently evidenced groups | No triple independence credit or duplicate vote; control preserves distinct support | CHECK-7/8 |
| **TA-C02 — protocol/security; portable** | NTS was bootstrapped using Roughtime; control has an independently admitted time basis | Retain bootstrap ancestry; successful later validation does not erase it | CHECK-9/14 |
| **TA-C03 — numerical; deterministic model** | Pool filtered/retried until it matches the initial estimate; control uses the frozen independent sampling policy | Expose changed selection/assumptions; no imported security bound or manufactured corroboration | CHECK-7/8 |
| **TA-C04 — protocol/engine; portable** | Response arrives after key removal, policy/pool generation change or deadline; current-generation control | Stale result cannot affect new state; valid neighbor admitted once | CHECK-6/8/27 |
| **TA-C05 — core/platform; model then native as applicable** | Clock step, uncertain acknowledgement, suspend, reboot or VM restore during a round; stable-frame control | Invalidate or qualified rebase; no mixed-frame comparison or age renewal | CHECK-10/11/12 |
| **TA-C06 — crypto/protocol; portable** | Root key omitted/wrong, old compromised delegation, replay or mismatched request; current valid keyed response | Diagnostic self-consistency cannot become root-authenticated evidence; old-key threat evaluated under the declared multi-source policy | CHECK-6/9/26 |
| **TA-C07 — numerical/TLS; portable plus actual verifier** | Very broad interval or certificate boundary inside it; precise interval inside the allowed range | No midpoint-only assurance or false precision; valid bounded case proceeds | CHECK-7/9 |
| **TA-C08 — acquisition; controlled network** | Honest sources selectively lost; available but insufficient groups; healthy full set | No silent minimum reduction, plain fallback or retry-until-agreement; healthy acquisition succeeds | CHECK-6/8/27 |
| **TA-C09 — domain/Windows; model then disposable guest** | Local date wrong by about ±100 years, true date inside/outside supported range and NTP-era boundaries; valid supported case | Independent era interpretation; explicit rejection of ambiguity/out-of-range arithmetic; no correction from rough evidence alone | CHECK-9/10 |
| **TA-C10 — storage/recovery; portable then applicable native owner** | Intact but wrong future floor or restored old signed catalogue; consistent current-state control | Only approved supersession/refusal path; no erasure of provenance, mandatory state or consumed budgets | CHECK-9/15 |
| **TA-C11 — Windows/NTS; disposable guest** | Roughtime succeeds but wrong-name, revoked, unavailable-revocation or untrusted NTS fails; valid full-path control | OPEN-8 policy governs; auxiliary agreement cannot bypass required verification; observe cleanup and good recovery | CHECK-9/27 |
| **TA-C12 — engine; controlled network/model** | Qualified NTS with false plain-watchdog alarm or optional Roughtime outage; agreeing-monitor control | Correct diagnostic distinctions; no unauthenticated replacement or unlimited veto, no invented agreement | CHECK-7/8/14 |
| **TA-C13 — policy; portable** | Fresh authenticated disagreement, followed by timeout; control supplies sufficient new resolving evidence | No generic mechanism-majority winner; timeout does not clear conflict; reviewed clearance rule works | CHECK-7/9/14 |
| **TA-C14 — engine/performance; controlled network/native where needed** | Repeated conflicts, slow verification, cancellation and expanded rounds; normal bounded round | One aggregate work budget; actual job retains permit; no primary-acquisition starvation or unbounded public-server burst | CHECK-21/27 |
| **TA-C15 — broker/platform; model then disposable guest** | Concurrent mechanism proposals and lost acknowledgement; one valid authorized correction | One serialized authority/effect, actual consumption and no double step or budget refill | CHECK-10/12 |
| **TA-C16 — integration/UI contract; portable then selected product layer** | Khronos disabled/deferred/rejected, optional checks stale and ordinary NTS healthy; explicitly enabled qualified case | RGT/NTS remains independently testable; disabled/unknown is not passing and current UI deferrals stay respected | CHECK-9/14 |

## 5. Exit and claim discipline

**KHR-C01:** TA-W05/09 retain the actual evaluated candidate and disposition under
the roadmap. `ADOPT_LIMITED` requires the bounded implementation and its relevant
cases qualified before Android. `DEFER`/`REJECT` retain reasons and re-entry
triggers. A Roughtime paper or successful bootstrap does not complete this exit.

**RGT-C01:** TA-W02/03/04/06/07 and applicable TA-W08/09 evidence demonstrate
qualified interval → NTS → broker recovery with refusal cases and actual Windows
limits. If no watchdog is adopted, TA-C16 establishes that configuration; full
KHR implementation is not required. If an adopted joint profile is claimed,
qualify its additional dependency/frame/resource cases. CHR-0 remains next after
this Roughtime stage.

Record candidate hashes, exact cases/environment, budgets, valid/failure controls,
actual results, cleanup and remaining gaps here. Do not convert documentary
review, a successful build, another candidate's Sandbox readiness or a zero-test
exit into a product pass.

## 6. Documentation review and handoff

The current change is limited to English documentation and task routing. Its
verification scope is source consistency, relative links/anchors, diff hygiene,
independent design review and remote commit/content verification. No product
build, time-server query, Windows execution or system-clock modification is part
of this documentation task.

The following receipt establishes only readiness of this research/plan handoff;
TA-W02–TA-W09 and all TA-C cases remain future work.

### 6.1 Completed independent review — 3 November 2026

**Reviewer:** `time_team_fresh_review`, a separate agent started with
`fork_turns: "none"`; read-only review, no inherited author conversation or private
knowledge-base retrieval. **Status:** completed. The actual final result was
received before this receipt was added. **Stage:** documentary research/design
handoff, not implementation or product qualification.

**Reviewed identity:** actual project baseline
`d868da58ee98f0d66f0b41cc74627a5c47a6cf8d`, eight changed files. The local snapshot
used a synthetic baseline only for inspection and was not project history.
The candidate hashes below identify the content before this review receipt:

- Binary diff SHA-256: `604d77e11d17c041cfeee6d8bbef44c6b8fabe5d0f6d0337fa15c95a81ee1384`.
- NV-TA-001 plan SHA-256: `f56228323f3633c202c1ce10bf98b2e0e32f453fadc03195e7dddea0e7ba0708`.
- Coordination task SHA-256: `c9f136ad67a35c9a46adb3842f311be146f9922cefe8044da0cfd4ea78d688f5`.

**Reviewer comprehension, reproduced verbatim:**

> The change records a future cooperation design in which Roughtime supplies an authenticated coarse date interval for NTS bootstrap, NTS remains the primary source of ordinary time estimates, Khronos is evaluated as an additional watchdog, and Ananke informs watchdog-pool admission. It proposes one evidence-admission policy and one broker-controlled clock-write path, with explicit provenance, comparison epochs, uncertainty, conflict handling and aggregate budgets. It preserves separate KHR-C01 and RGT-C01 exits and does not adopt a provider, dependency, new quorum, preliminary clock step or correction-inhibiting watchdog.

> The supplied current records describe implemented portable protocol/numerical fixtures, inert NTS framing kernels, a bounded observation-only engine pilot and a fixture-only native UI; production NTS recovery, the complete service/broker path and these additional mechanisms remain planned. Current product work is the S1.3 principal/combination qualification chain. UI-M0 D1–D3 remain deferred. Scoped owned-Sandbox management and separately admitted guest work are permitted under the later disposition, while the blocked artifact, general host Windows execution, host security/service/clock changes, separate fuzz hold and storage-experiment pause remain outside that permission. These current-state statements come from the supplied documentation, not a new inspection or execution of product binaries.

**Reviewer verdict and findings, reproduced verbatim:**

> Verdict: READY for the documentary handoff
>
> Review status: completed, fresh-context, read-only independent review. No files were changed, no product tests were run, and no time servers were queried.
>
> Material findings: none. No correction is required to make this research/design handoff ready. The unresolved choices are identified as future dependent gates rather than silently approved behavior.

**Inputs actually inspected, as reported by the completed reviewer:**

- Root `AGENTS.md`, documentation-guide authority/current-amendment sections,
  `plan-independent-review`, `review-and-handoff` and the testing contract;
  the complete new plan/task and complete eight-file diff.
- Relevant surrounding KHR-0/RGT-0, OD-02/OD-04, R-02/R-05, architecture
  roles/domain/recovery/CHECK/OPEN-8 sections and development-plan current-task
  and later-handoff sections. The diff supplied predecessor text for changes.
- RFC 10049 bootstrap, sequence and security sections; RFC 9523 design,
  sampling, threat and correction-normalization sections; RFC 8915 bootstrap,
  delay and stripping sections; the original Ananke paper; Microsoft
  `CertGetCertificateChain`; the pinned Roughenough README, manifest, validator
  and protocol-version sources; SIDN and Tanner Ryan service pages; Cloudflare
  usage/beta documentation and the pinned README. Exact primary-source URLs and
  implementation pins are retained in the plan's source inventory.

The reviewer found the authority, independence/uncertainty, epoch/recovery,
resource/lifecycle, Windows-trust and separate-exit boundaries coherent for this
stage. Its provider/source inspection established no live availability, Windows
compatibility, admitted dependency graph or NevroTick runtime result. No finding
resolution or policy supersession was necessary.

**Reviewer next action, reproduced verbatim:**

> Complete the documentary handoff by recording this review in the existing coordination task and finishing the author’s candidate/publication checks under the existing authorization. TA-W02 remains the next workstream contract-freeze step when its phase is selected. This verdict grants no product implementation, native experiment, package release or permission to push the synthetic local repository.

**Author documentary checks:** all 34 newly introduced relative links/anchors
resolved against the pinned repository tree/candidate; `git diff --check` passed.
The review receipt adds no new relative links or changes to the reviewed design.
Publication uses the real remote base tree and expected-head lease, with exact
remote content verification. These checks are not execution of TA-C01–TA-C16.
