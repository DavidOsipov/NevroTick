# Roughtime and Khronos/Ananke coordination task

**Date:** 3 November 2026, Asia/Tbilisi<br>
**Retained leap-policy integration baseline:** `DavidOsipov/NevroTick@7550f83292f959a6d0acbb2d396e964a6bcf4b01`; 4 November 2026, NV-TA-001 revision 1.2. Earlier amendment and receipt identities are retained below.<br>
**Current design owner:** [NV-TA-001, revision 1.6](../docs/plans/time-assurance-integration.md), including the 27 November V1 scope amendment below.<br>
**Mode/status:** V1 Roughtime implementation/qualification work defined; protocol/runtime implementation and qualification have not run. Khronos/Ananke remains post-V1.<br>
**Coordinator:** existing numerical/protocol P2/P7 owner, with independent security and Windows reviewers; David owns material policy choices.

**Current V1 scope, 27 November 2026:**
[OD-02](../docs/nevrotick-owner-decisions.md#v1-protocol-and-hardware-scope--27-november-2026)
selects NTPv4/NTS and Roughtime cold-date bootstrap, without PPS/GNSS/refclock
product inputs. RGT-0 implementation and RGT-C01 qualification belong to existing
P2/P7 owners before V1. This supersedes the earlier later-only RGT scheduling;
CHR-0 still requires RGT-C01 and stays post-V1. No missing laboratory hardware
becomes a recurring prerequisite for independent protocol work. Actual
provider/key/interval/Windows-verifier gates remain open.

**Post-V1 scheduling amendment, 14 November 2026:** [OD-16 §20.1](../docs/nevrotick-owner-decisions.md#201-mathematics-experiments-first--13-november-2026) preserves [MATH-0/MATH-C01](../ROADMAP.md#math-0--post-v1-mathematical-estimation-experiments) first after actual V1 release. [OD-20](../docs/nevrotick-owner-decisions.md#24-od-20--supplementary-certificate-distrust-after-v1) and NV-TA-001 revision 1.5 placed [TRUST-0/TRUST-C01](../docs/plans/post-v1-certificate-distrust.md) second, then [MT-0/MT-C01](../docs/plans/post-v1-multithreading.md), before this task's KHR work; that post-V1 order remains. MATH-C01 requires evaluation/disposition, not production adoption. Earlier baseline and review receipts below retain their historical scope.

**Pre-V1 case preparation, 13 November 2026:** [OD-18](../docs/nevrotick-owner-decisions.md#22-od-18--literature-derived-qualification-before-v1)
and [LIT-0](../ROADMAP.md#lit-0--literature-derived-qualification-before-v1)
require the inexpensive literature-derived cases to be specified/reviewed before
V1 and exercised against actual shared V1 consumers where they exist. The
[research handoff](../docs/supporting-reviews/chrony-ntpsec-noncore-research.md)
refines the existing TA-C IDs below. Case preparation alone does not implement a
TCA or qualify a runtime. Under the 27 November scope, applicable RGT cases must
also reach actual V1 consumers before RGT-C01; reserve-only KHR cases stay
post-V1. An absent consumer remains an implementation prerequisite; a mock
protocol or a written case supplies no TA-C pass.

**Catalogue linkage, 4 November 2026:** the
[binary catalogue refinement](../docs/plans/server-catalogue-binary-format.md)
and [catalogue task](server-catalogue-and-statistics.md) carry the first-schema
public-key requirement and future publisher tooling. The 3 November protocol
research and review receipts below retain their original candidate scope.

## 1. Request, scope and retained authority

Record the Roughtime research and design how it cooperates with Khronos/Ananke.
Reuse one evidence chain for [KHR-0](../ROADMAP.md#khr-0--evaluate-independent-time-cross-checking)
and [RGT-0](../ROADMAP.md#rgt-0--roughtime-bootstrap-for-a-badly-wrong-clock), keeping
their prerequisites, implementation decisions and exits distinct. Ananke informs
pool admission; it is not a fourth measurement protocol. Existing engine policy
and the broker retain correction ownership.

**Later owner direction, 3 November 2026:** Roughtime is preferred for cold
initial date bootstrap; Khronos/Ananke with an NTP population is a conditional
reserve when Roughtime cannot supply usable evidence for NTS. The reserve
supplies a bounded provisional date, followed by the same full NTS and broker
gates. Continuous watchdog operation is a deferred alternative. The future
reserve is automatic within its qualified envelope; until then the current
strict-default/explicit-helper baseline remains. Healthy retained NTS does not
gain an unnecessary Roughtime dependency.

**Leap-policy integration, 4 November 2026:** apply [NV-LP-001](../docs/plans/leap-second-policy.md)
and [architecture §10.5](../docs/windows-time-architecture.md#105-leap-seconds-and-time-scales)
through the [S1.4 task](s1.4-leap-policy.md) before dependent TA integration.
The reviewed compiled dataset, effective UTC rule and full date/era hypotheses
belong to that policy; bootstrap results do not replace them. Preserve useful
all-LI=0 unsmeared sources, bounded possible-month-end guards and independent
acquisition while a sample lacks discipline authority. A demonstrably conflicting
LI observation is excluded without discarding healthy peers or blocking fresh
consistent re-entry. Native explicit events remain kernel-owned; no synthetic
±1-second request or extra correction budget is introduced. Existing phase order,
execution holds and historical review receipts remain unchanged.

RGT-0 is required V1 work at its existing P2/P7 entry, without changing the next
Core task or selecting providers/dependencies, public licensing or direct clock
correction from Roughtime/Khronos. V1 released → MATH-C01 disposition → TRUST-C01
→ MT-C01 → KHR-C01 → GEO-C01 → Android remains; CHR-0 requires RGT-C01 but stays
post-V1. Roughtime-only qualification does not depend on adopting Khronos.
V1 exposes the absent reserve. Without a separately enabled and admissible
plain-helper route, failed cold recovery stays blocked or independently usable
cached/strict NTS continues. [Architecture §8.1](../docs/windows-time-architecture.md#81-recovery-order)
retains the existing explicit helper policy; Roughtime failure neither enables
helpers nor clears signed conflict. Claiming the later combined automatic-reserve profile
requires its reserve implementation and integration evidence.

Roughtime key/profile support is required in the first catalogue schema even
while this task's V1 runtime work remains unimplemented. A usable Roughtime record
needs its admitted root-key algorithm/bytes, exact profile, endpoint scope and
provenance; an optional-key diagnostic path cannot create authenticated evidence.
The ordinary NTP/NTS catalogue and publisher collector can be prepared before
RGT-0. Their binary preference changes neither the strict JSON policy/IPC/journal
contracts nor this task's roadmap order, roster qualification or acceptance exits.

Read AGENTS.md, the documentation guide, the architecture, NV-TA-001 and the
current [development next-task record](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
before execution. The scoped owned-Sandbox permission and separately admitted
guest work do not release the blocked S134 artifact, general host Windows
execution, host security/service/clock changes, the separate AES fuzz hold or the
storage experiment pause. Reconcile current gates at the dependent checkpoint.

## 2. Current evidence and reconciliation

**Completed:** source/documentation research and this coordinated plan.
Reviewed RFC 10049, RFC 9523, the original Ananke paper, existing NTS recovery and
broker contracts, and retained dated Roughtime implementation/provider findings.
The plan's sources distinguish specification, inspected implementation and
unverified operational claims.

**Not established:** product implementation, exercised TA-C cases, provider
interoperability, native Windows verification/recovery, performance or either
roadmap exit. The earlier Sandbox readiness receipt belongs to its own candidate
and does not qualify this task's planned binaries.

| Predecessor obligation | Disposition in this task |
| --- | --- |
| KHR-0 evaluation and its four outcomes | Retained; linked to one explicit task and joint evidence model |
| RGT-0 interval → NTS → broker recovery | Retained; moved into V1 P2/P7 by the 27 November scope, with no preliminary global clock write adopted |
| NTS quorum, source minimum and 16-source/resource limits | Retained; auxiliary mechanisms receive no extra vote or separate unbounded budget |
| OPEN-8 and saved-floor supersession | Retained as dependent gates; external agreement does not close them |
| Runtime comparison and feedback | Added proposed dependency/epoch contracts and combined hostile/valid cases |
| Current development and execution order | Existing P2/P7 owners now include V1 RGT-C01; current next-task and execution holds remain. Documentation completion is no runtime qualification |
| Earlier later-only RGT and immediate RGT-0 → CHR-0 timing | Superseded: RGT-C01 precedes V1; CHR-0 retains that prerequisite and remains post-V1 |
| Revision 1.0 periodic watchdog proposal and blanket no-automatic-reserve wording | Superseded for the future selected profile by David's later Roughtime-first / conditional-reserve direction; continuous monitoring remains deferred |
| Existing opt-in plain-bootstrap baseline | Retained until the scoped future automatic reserve is specified and qualified; no silent relaxation of NTS, quorum or clock authority |
| Earlier small provider shortlist | Enriched by a pinned mixed-version census: 14 hostnames / about 10 nominal groups; only two explicit RFC-v1 operator advertisements and a third maintainer-documented candidate, with no live checks |

## 3. Work assignments and finite exits

TA-W identifiers are local assignments. Each implementation owner freezes the
exact source/graph, selected cases, finite work budget and environment before its
run. Later unresolved choices do not block this documentation handoff.

| ID / owner | Work and required input | Observable exit / current status |
| --- | --- | --- |
| **TA-W01 — coordinator/source reviewer** | Reconcile the request, primary sources, existing KHR/RGT stages and affected consumers | This plan/task and canonical routing exist; source findings are separated from qualification. **Source/design work complete; review receipt below owns handoff evidence** |
| **TA-W02 — protocol/security owner** | Freeze V1 Roughtime/NTS triggers and refusal behavior, evidence fields, dependency groups, era/frame/interval admission, generations and conflict clearance; distinguish date-specific NTS failures and retain compromised-key controls. Integrate NV-LP-001's dataset/rule/scale-segment, full-date-hypothesis and guard fields without a circular bootstrap prerequisite. Freeze reserve-only triggers/envelope and its expired-NTS-key/common-path case at the later KHR entry | Reviewed contract and independent expected-result table for each selected slice; exact owners, assurance limits and forbidden effects. **V1 P2 contract pending; reserve-only work post-V1** |
| **TA-W03 — Roughtime/dependency owner** | Select exact wire/client/crypto graph under CHECK-26; implement bounded inert decoding and mandatory-root-key evidence construction after TA-W02 | Valid response accepted; missing/wrong root, version/unit errors, malformed/replayed input and invalid intervals cannot create authenticated evidence. **V1 P2, not implemented** |
| **TA-W04 — core/engine owner** | Model V1 Roughtime/NTS branch selection, receipt consumption, epoch changes, ancestry, uncertainty and outcomes with independent fixture truth; retain acquisition during leap ambiguity, all-silent healthy controls and fresh post-guard admission under S1.4. Add reserve-specific behavior after TA-W05 | Applicable TA-C01–TA-C07/10/12/13/16/18/21 controls reach the policy consumer; no duplicate credit, stale admission, calendar-vote learning or extra clock controller. **V1 P2 work pending; reserve portions post-V1** |
| **TA-W05 — KHR numerical/security owner** | After actual V1 release, MATH-C01 disposition, TRUST-C01 and MT-C01, evaluate cold-start reserve candidates, Ananke-inspired admission, initial roster/refresh and bounded sampling against the unchanged baseline; do not import periodic-watchdog security bounds | KHR-0's recovery/error/false-acceptance/latency/cost evidence plus `ADOPT_LIMITED`, `DEFER` or `REJECT`; adoption needs qualified bounded implementation. **Future KHR phase** |
| **TA-W06 — RGT transport/trust owner** | Reconcile the dated census and linked first-schema key inventory by wire version/operator/key/upstream; consume the catalogue owner's required key/profile bindings, admit update/rollback rules, and qualify advertised UDP/TCP alternatives and finite interoperability work; controlled counterpart first | Exact compatibility and minimum-independent-source success/failure evidence, including TA-C17; terms and observed availability scope retained. Catalogue entries and decoded public-key lengths are not live passes. **V1 P2/P7 evidence pending** |
| **TA-W07 — Windows/NTS owner** | With admitted artifacts and owned disposable environment, exercise the V1 Roughtime date-basis route under OPEN-8/R-02/R-05, exact error categories and fresh isolated verification across hypotheses; exercise the reserve route only after its post-V1 admission | Valid supported-date recovery and explicit failure for bad identity/revocation/era; applicable TA-C11/19/20/22, no unauthorized effect, observed cleanup/restoration. **V1 P2/P7 gate open; existing native admission/holds remain** |
| **TA-W08 — engine/performance/security owner** | Exercise V1 Roughtime failure, resource, cancellation, conflict and broker boundaries, including healthy cached/strict NTS and the absent reserve. Add forced-reserve cases only for a qualified post-V1 profile | Applicable TA-C08/11/12/14–TA-C22 plus integrated cases; aggregate limits and actual effects observed. **V1 P2/P7 evidence pending; reserve-only work post-V1** |
| **TA-W09 — coordinator/independent reviewer** | Reconcile candidate-specific evidence and remaining policy/operational gaps at each exit | RGT-C01 before V1; separate post-V1 KHR-C01 disposition and CHR-0 dependency. **Both exits open; no combined automatic pass** |

TA-W03/04 do not require a full production service or public network access.
TA-W02/04/07/08 have separately scoped V1 and reserve portions; incomplete
post-V1 reserve work cannot block the Roughtime-only exit. TA-W05 compares the
qualified baseline at its post-V1 entry; simulation supplies no implementation
or Windows qualification for either route.
TA-W07 requires the whole NTS/Windows path, not a successful Roughtime packet
exchange. Split an oversized assignment into bounded slices inside this task.

## 4. Independent acceptance cases

All cases below are **planned, not run**. Pair each hostile/failure input with the
stated valid neighbor. Use independent fixture truth, not another translation of
the production algorithm. The existing [CHECK catalog](../docs/windows-time-architecture.md#202-acceptance-catalog)
and [testing contract](../.agents/skills/testing/CONTRACT.md) retain their full
obligations, including applicable coverage/mutation/native evidence. Exercise the
Roughtime/NTS and absent-reserve portions before RGT-C01/V1. Reserve-only cases
and qualified-reserve controls remain post-V1; retain them as deferred, not passed.

| ID / owner / layer | Controlled case and valid neighbor | Expected result and forbidden effect | Canonical mapping |
| --- | --- | --- | --- |
| **TA-C01 — core/security; portable** | One operator appears as NTS, Roughtime and watchdog; distinct servers also relay one issuer/time assertion. Control uses independently evidenced groups and time bases | No triple independence credit or duplicate vote. Preserve genuinely separate observations while counting the common issuer/time premise once; control preserves distinct support | CHECK-7/8 |
| **TA-C02 — protocol/security; portable** | NTS was bootstrapped using Roughtime; control has an independently admitted time basis | Retain bootstrap ancestry; successful later validation does not erase it | CHECK-9/14 |
| **TA-C03 — numerical; deterministic model** | Pool filtered/retried until it matches the initial estimate; control uses the frozen independent sampling policy | Expose changed selection/assumptions; no imported security bound or manufactured corroboration | CHECK-7/8 |
| **TA-C04 — protocol/engine; portable** | Response arrives after key removal, policy/pool generation change or deadline; renewal changes issuer/key, endpoint or route. Current-generation admitted renewal is the control | Stale result cannot affect new state; revalidate changed identity/routing instead of inheriting authority. Valid neighbor admitted once | CHECK-6/8/27 |
| **TA-C05 — core/platform; model then native as applicable** | Clock step, uncertain acknowledgement, suspend, reboot or VM restore during a round; stable-frame control | Invalidate or qualified rebase; no mixed-frame comparison or age renewal | CHECK-10/11/12 |
| **TA-C06 — crypto/protocol; portable** | Root key omitted/wrong, old compromised delegation, replay or mismatched request; a newly signed response carries historical authentic time evidence. Current valid keyed and sufficiently fresh response is the control | Diagnostic self-consistency cannot become root-authenticated evidence; packet freshness cannot renew historical authority evidence. Evaluate the old-key threat under the declared multi-source policy | CHECK-6/9/26 |
| **TA-C07 — numerical/TLS; portable plus actual verifier** | Very broad interval or certificate boundary inside it; biased samples all remain inside a valid coarse interval. Control has an independently justified sufficiently precise interval inside the allowed range | No midpoint-only assurance or false precision. Preserve interval width and ordinary sample-quality checks; valid bounded case proceeds without inventing accuracy from signature acceptance | CHECK-7/9 |
| **TA-C08 — acquisition; controlled network** | Honest sources selectively lost, including a selected honest endpoint whose response misses the deadline; available but insufficient groups; healthy full set | Distinguish selected, responding and admitted support. V1 refuses insufficient Roughtime support. Without a separately enabled, admissible helper route, cold bootstrap stays blocked or independently usable cached/strict NTS continues; existing explicit helpers retain their full policy and a qualified post-V1 reserve may produce a labelled provisional date. No minimum reduction, ordinary plain-NTP discipline or retry-until-agreement. Healthy acquisition succeeds | CHECK-6/8/27 |
| **TA-C09 — domain/Windows; model then disposable guest** | Local date wrong by about ±100 years, true date inside/outside supported range and NTP-era boundaries; valid supported case | Independent era interpretation; explicit rejection of ambiguity/out-of-range arithmetic; no correction from rough evidence alone | CHECK-9/10 |
| **TA-C10 — storage/recovery; portable then applicable native owner** | Intact but wrong future floor, restored old signed catalogue, or ordered historical credentials after first boot/state loss/restart/offline recovery; consistent current-state control | Relative ordering cannot establish present freshness. Only approved supersession/refusal path; no erasure of provenance, mandatory state or consumed budgets | CHECK-9/15 |
| **TA-C11 — Windows/NTS; disposable guest** | Roughtime succeeds but wrong-name, revoked, unavailable-revocation or untrusted NTS fails; date-only failure and valid full-path controls | OPEN-8 policy governs; distinguish failures, preserve required verification and try other eligible identities. No arbitrary-date search to bypass non-date errors; observe cleanup/recovery | CHECK-9/27 |
| **TA-C12 — engine; controlled network/model** | Qualified NTS with a false late plain-reserve alarm or Roughtime outage; healthy route control | Ordinary NTS continues under its policy; reserve stays inactive absent a recovery trigger, with no unauthenticated replacement, unlimited veto or invented agreement | CHECK-7/8/14 |
| **TA-C13 — policy; portable** | Fresh authenticated disagreement, followed by timeout or unavailable/expired authority evidence; control supplies sufficient new resolving evidence | Distinguish contradiction, stale evidence and network failure. No generic mechanism-majority winner or silent downgrade; timeout does not clear conflict; reviewed clearance rule works | CHECK-7/9/14 |
| **TA-C14 — engine/performance; controlled network/native where needed** | Repeated conflicts, slow verification, cancellation, failed renewal and expanded rounds; normal bounded round | One aggregate work budget; actual job retains permit; no primary-acquisition starvation, unbounded public-server burst or silent route fallback | CHECK-21/27 |
| **TA-C15 — broker/platform; model then disposable guest** | Concurrent mechanism proposals and lost acknowledgement; one valid authorized correction | One serialized authority/effect, actual consumption and no double step or budget refill | CHECK-10/12 |
| **TA-C16 — integration/UI contract; portable then selected product layer** | Khronos absent in V1; compare strict/no-helper recovery with a separately enabled, policy-valid helper route. Post-V1 disabled/deferred/rejected, untriggered during healthy NTS or required but unavailable; qualified triggered reserve as a post-V1 control | RGT/NTS remains independently testable; disclose route and missing reserve, retain bootstrap ancestry and UI deferrals. V1 can recover without the reserve through Roughtime or separately enabled admissible helpers, each retaining its provenance and all NTS/broker gates. With no admitted bootstrap route, cold recovery stays blocked while independently usable cached/strict NTS remains available. An untriggered reserve is not a reserve pass | CHECK-9/14 |
| **TA-C17 — catalogue/protocol; portable then controlled peers** | Four hosts under one operator, duplicate PQ mode, draft-only peers, or two valid operators where three are required; three admitted independent RFC peers as control | Count compatible admitted groups; refuse insufficient Roughtime support. This refusal does not enable plain helpers or the post-V1 reserve; a separately enabled admissible helper route retains its own policy. Reject incompatible wire data; test qualified TCP after UDP failure without inventing an extra operator | CHECK-6/8/9/26 |
| **TA-C18 — policy/network; deterministic model then controlled peers** | Suppress Roughtime traffic but leave NTS reachable. Compare strict/no-helper mode with two explicitly enabled, admissible agreeing helpers and no signed conflict; healthy Roughtime is the preferred-route control | Without an admitted helper route, V1 exposes blocked cold bootstrap or retains independently usable cached/strict NTS. The policy-enabled healthy helper neighbor can proceed through every architecture §8.1 NTS/broker gate; signed conflict cannot be erased by helpers. Only a separately qualified post-V1 profile activates the Khronos/Ananke reserve. No weakened NTS checks, source minimum, clock privilege or ordinary-protocol downgrade; normal Roughtime route succeeds | CHECK-6/9/14/27 |
| **TA-C19 — protocol/policy; portable plus actual verifier** | One invalid/greased Roughtime reply, certificate-time incompatibility, wrong name, explicit revocation and network timeout; valid responses and genuinely date-dependent recovery as controls | Distinct cause/branch behavior; invalid packets cannot select arbitrary time or prove authenticated misbehavior. No NTS connection-error catch-all that declares Roughtime wrong | CHECK-6/9/14 |
| **TA-C20 — trust/security; controlled credentials and peers** | Common-path forged Pool date plus a compromised expired test NTS key; valid current authority and saved-bound controls | Candidate-date/certificate self-consistency is not independent proof. Enforce configured saved bounds, NTS agreement and revocation; record unsupported assurance when those constraints cannot exclude the scenario, not a fabricated safe recovery | CHECK-9/14/15 |
| **TA-C21 — policy; portable** | Signed Roughtime contradiction followed by reserve agreement, timeout or a route change; sufficient independent resolving evidence as control | Conflict and provenance survive; only the frozen clearance policy admits final recovery. No majority across mechanism labels | CHECK-7/9/14 |
| **TA-C22 — TLS/session lifecycle; controlled verifier then native owner** | Change candidate date while resumable state or old verification remains; fresh isolated full-handshake control | No stale certificate-time assurance or cross-hypothesis session reuse; fresh identity/verification/protected-observation gates pass independently, within aggregate limits | CHECK-9/12/27 |

**Leap-policy refinements to existing cases — planned, not executed:** TA-W02/04
reuse TA-C04/05 for dataset/rule/mapping changes, crossing or delayed replies and
the minimum fresh post-guard acquisition instant. TA-C07/09 cover a host date
wrong by a century, all admitted era hypotheses and intervals spanning a known
event or the **20 May 2027** effective rule. A rule-date crossing is a healthy
control when all hypotheses yield the same admissible event-free interpretation;
actual interpretation ambiguity withholds only dependent clock authority while
bounded independent bootstrap/NTS acquisition continues. Rejecting all acquisition
cannot pass these cases.

TA-C10 preserves the distinction between a valid release with unknown future
coverage, expired historical input metadata, and corrupted package/mandatory
state. The first may continue ordinary tracking outside guards; none resets
protected floors or consumed correction authority. TA-C12 supplies all-LI=0
unsmeared peers, a false announcement plus healthy peers, fresh source re-entry,
and every possible month-end under unknown leap-capable coverage. A nonzero-LI
quorum is never required; a faulty peer cannot extend the guard or reduce normal
agreement. TA-C15 and TA-W07 retain kernel-only explicit events, actual measured
residual recovery and no synthetic extra second when Windows already handled
the event. Reuse CHECK-6/7/8/9/10/11/12/14/15/25 at their existing phase scopes;
portable refinements do not qualify native event behavior or execute RGT-C01.

## 5. Exit and claim discipline

**KHR-C01:** TA-W05/09 retain the actual evaluated candidate and disposition under
the roadmap. `ADOPT_LIMITED` requires the bounded implementation and its relevant
cases qualified before Android. `DEFER`/`REJECT` retain reasons and re-entry
triggers. A Roughtime paper or successful bootstrap does not complete this exit.
The next roadmap experiment is [GEO-0](../ROADMAP.md#geo-0--evaluate-geography-assisted-source-ordering),
owned by the [existing catalogue task](server-catalogue-and-statistics.md#deferred-geoip-experiment--after-khronos).
Its disposition precedes Android without requiring GeoIP implementation or
altering the qualified Khronos/Ananke population and sampling distribution.

**RGT-C01 — required before V1:** TA-W02/03/04/06/07 and applicable TA-W08/09 evidence demonstrate
qualified interval → NTS → broker recovery with refusal cases and actual Windows
limits. If no reserve is adopted, TA-C16 establishes that disclosed configuration;
full KHR implementation is not required for Roughtime-only qualification. Claiming
the selected combined profile requires its reserve implementation and TA-C17–22
alongside the applicable dependency/frame/resource cases. CHR-0 retains RGT-C01
as a prerequisite and remains post-V1; this exit does not immediately start it.

Record candidate hashes, exact cases/environment, budgets, valid/failure controls,
actual results, cleanup and remaining gaps here. Do not convert documentary
review, a successful build, another candidate's Sandbox readiness or a zero-test
exit into a product pass.

## 6. Documentation review and handoff

The original 3 November change was limited to English documentation and task
routing, with source consistency, link/anchor, diff, independent-review and
remote-content checks. Its receipts below establish only the readiness of those
research/plan candidates. They do not review the later scheduling amendments or
qualify any runtime. TA-W02–TA-W09 and all TA-C cases remain unexecuted, at their
current V1 or post-V1 placement.

### 6.1 Completed independent review — 3 November 2026

**Historical scope:** revision 1.0, before the later preferred/reserve refinement.
Its original candidate hashes and attributed paragraphs remain unchanged; this
receipt does not review revision 1.1 or the additional TA-C17–TA-C22 cases.

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

### 6.2 Revision 1.1 independent review — later on 3 November 2026

**Reviewer:** `roughtime_reserve_fresh_review`, separate agent with
`fork_turns: "none"`; no author-conversation inheritance or private-KB retrieval.
**Status:** completed after an initial context-limit launch failure and retry.
The actual final result was received before this receipt. A supplementary census
child failed to launch; the reviewer performed the cited source verification
directly and claimed no evidence from that child.

**Stage:** documentary research/design handoff. **Reviewed identity before
receipt insertion:** real project baseline
`9c6b5ef39b134c844a850deb9126823d668babe7`, eight-file candidate; the local synthetic
snapshot was inspection-only, not project history. The final source-compatibility
addition was included in the completed review:

- Binary diff SHA-256: `6b5ba5f9f281270cc096fcd247ac4d8f046fabbed1bd82b3ae7a59e67f7cfa8d`.
- NV-TA-001 plan SHA-256: `870d25d208c6218071297c011e8441cb73862b9ec353be79c8be7fb66ccc2bb0`.
- Coordination task SHA-256: `d7f7e40b586baa3aa67b990e3534f6dca879d8c0138f0b53489f790dacd657d7`.

**Reviewer comprehension, reproduced verbatim:**

> The revision selects a future recovery sequence in which Roughtime provides the preferred initial date basis, NTS supplies the authenticated observations, and Khronos/Ananke supplies a conditional provisional-date reserve when the preferred route cannot produce usable evidence. It replaces the earlier periodic-watchdog proposal for this profile. Both bootstrap routes retain the full NTS verification, fresh agreement, provenance and broker requirements; neither auxiliary mechanism gains ordinary NTS votes or direct clock-writing authority.

> The supplied current records describe portable protocol/numerical fixtures, inert NTS framing, a bounded observation-only engine pilot and a fixture-only native UI. Production NTS recovery, the complete broker/service path and the new preferred/reserve mechanism remain planned. The current next work is S157 principal/chart qualification, followed by chart-aware combination. Scoped owned-Sandbox work remains distinct from the blocked S134 artifact, general host Windows execution, host security/service/clock changes, the separate fuzz hold and the storage-experiment pause. These are documentary current-state findings, not new observations of product binaries.

**Reviewer verdict and findings, reproduced verbatim:**

> READY for the documentary research/design handoff.
>
> Review status: completed, fresh-context, read-only independent review. No private knowledge-base retrieval, edits, time-server probes, product builds, tests or Windows execution were performed.
>
> Material findings: none. No corrective change is required for this handoff stage. The unresolved implementation choices remain assigned to future work instead of being represented as completed decisions or qualification.

**Inputs actually inspected, as reported by the completed reviewer:** root
instructions, documentation guide, independent-review/shared-handoff skills and
testing contract; complete revision 1.1 plan/task and eight-file diff; surrounding
KHR/RGT, OD-02/03/04, R-02/04/05, architecture ownership/source/recovery/TLS/CHECK
sections, DEC-7/OPEN-6/OPEN-8 and development-plan current-task/handoff sections.
Primary-source reads covered relevant RFC 10049/9523/8915 passages, Ananke,
Microsoft's chain API, pinned Roughenough README/manifest/validator, pinned Tanner
catalogue, operator pages, pinned Cloudflare records, teax page-source and Pool
vendor guidance. Alastyr's English URL could not be fetched; its
[Turkish operator page](https://ntp.alastyr.com/roughtime.html) supplied the
advertised draft/transport facts. No live service health was established.

The reviewer independently recomputed the catalogue arithmetic, confirmed the
RFC-v1/TCP advertisements and optional Roughenough compatibility mode, and found
the priority/default supersession, trigger distinctions, retained signed conflict,
consumer routing and acceptance coverage coherent for this stage. No corrective
finding or new risk acceptance resulted.

**Reviewer next action, reproduced verbatim:**

> Append this completed v1.1 documentary review to the existing coordination task, preserving the v1.0 receipt and identifying the hashes above as the reviewed candidate before receipt insertion. This verdict grants no implementation, native experiment, release, or permission to push the synthetic local repository.

**Author checks:** all five added/changed relative links and anchors resolved;
nine TA-W assignments and 22 planned TA-C cases remain distinct from execution;
`git diff --check` passed. This receipt changes no reviewed design rule and adds
no relative link. Publication uses the real base tree, expected-head lease and
exact remote-content verification. TA-W02–TA-W09, all TA-C cases, source
admission/interoperability, exact reserve/conflict policy, dependency admission,
native Windows recovery and both roadmap exits remain future work.

### 6.3 V1 scheduling repair — 27 November 2026

Documentation-only scope: `plan-code-change` and `plan-architecture` reconcile
OD-02's current scope with ROADMAP, NV-TA-001 and this task, retaining the existing
engine/broker and trust contracts. RGT-C01 now belongs to P2/P7 before V1;
reserve-only work and CHR-0 remain post-V1. Trusted `git diff --check` passed on
the documentation candidate based on `45e2fcaafe25faae02c41d5ddc4a46ebdcf48a92`;
the coordinator owns combined-candidate canonical-link and consistency verification.
No product execution, new dependency, hardware trial or TA-C pass is part of
this repair; historical review receipts retain their original identities.
