# Roughtime and Khronos/Ananke coordination task

**Date:** 3 November 2026, Asia/Tbilisi<br>
**Retained leap-policy integration baseline:** `DavidOsipov/NevroTick@7550f83292f959a6d0acbb2d396e964a6bcf4b01`; 4 November 2026, NV-TA-001 revision 1.2. Earlier amendment and receipt identities are retained below.<br>
**Current design owner:** [NV-TA-001, revision 1.9](../docs/plans/time-assurance-integration.md), including the 27 November V1 scope, post-V1 clock-permissions, implementation-source and RFC 10049 amendments below.<br>
**Mode/status:** RFC semantic baseline documented; V1 Roughtime engineering freeze, implementation and qualification remain open. Protocol/runtime implementation and qualification have not run. Khronos/Ananke remains post-V1.<br>
**Coordinator:** existing numerical/protocol P2/P7 owner, with independent security and Windows reviewers; David owns material policy choices.

**Current V1 scope, 27 November 2026:**
[OD-02](../docs/nevrotick-owner-decisions.md#v1-protocol-and-hardware-scope--27-november-2026)
selects NTPv4/NTS and Roughtime cold-date bootstrap, without PPS/GNSS/refclock
product inputs. RGT-0 implementation and RGT-C01 qualification belong to existing
P2/P7 owners before V1. This supersedes the earlier later-only RGT scheduling;
CHR-0 still requires RGT-C01 and stays post-V1. No missing laboratory hardware
becomes a recurring prerequisite for independent protocol work. Actual
provider/key/interval/Windows-verifier gates remain open.

**RFC 10049 audit integration, 27 November 2026:**
[NV-TA-001 §8.4](../docs/plans/time-assurance-integration.md#84-rfc-10049-client-contract-and-acceptance-mapping--27-november-2026)
owns the consolidated client contract and dispositions for RFC-W01–W06 and
RFC-M01–M04; RFC-I01 is RFC-W01 and RFC-I02 is RFC-M01, not additional findings.
Section 4.2 below maps independent expected results to existing TA-C owners.
This completes the documentary semantic baseline, not TA-W02's engineering
exit: typed constructors, remaining product constants, aggregation/fault policy
and exact independent fixtures still require their bounded freeze and review.
Combined revision 1.9 documentary review completed in §6.5. Current product order and every
dependency, native, resource and release gate retain their scope.

**Post-V1 scheduling amendment, 14 November 2026:** [OD-16 §20.1](../docs/nevrotick-owner-decisions.md#201-mathematics-experiments-first--13-november-2026) preserves [MATH-0/MATH-C01](../ROADMAP.md#math-0--post-v1-mathematical-estimation-experiments) first after actual V1 release. [OD-20](../docs/nevrotick-owner-decisions.md#24-od-20--supplementary-certificate-distrust-after-v1) and NV-TA-001 revision 1.5 placed [TRUST-0/TRUST-C01](../docs/plans/post-v1-certificate-distrust.md) second, then [MT-0/MT-C01](../docs/plans/post-v1-multithreading.md), before this task's KHR work; the later 27 November amendment below inserts CLK-0 between MT-0 and Khronos. MATH-C01 requires evaluation/disposition, not production adoption. Earlier baseline and review receipts below retain their historical scope.

**Post-V1 clock-permissions amendment, 27 November 2026:** [OD-21](../docs/nevrotick-owner-decisions.md#25-od-21--restricted-local-clock-changing-permissions) and [NV-CLK-001](../docs/plans/windows-clock-permissions.md) insert CLK-0 after MT-C01 and immediately before KHR-0. CLK-C01 records assessed implementation/qualification and an explicit evidence-supported disposition; research or blocked work is not an implemented pass or silent stage skip. Existing W11-W05/W11-W10 retain permission/lifecycle/package ownership; this task retains the separate RGT-C01 and KHR-C01 protocol evidence.

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
→ MT-C01 → CLK-C01 → KHR-C01 → GEO-C01 → Android applies under OD-21; CHR-0 requires RGT-C01 but stays
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

### 2.1 Source review and implementation basis — 27 November 2026

**Inspected project source:** `3e3536d808789dabb1b6577b069140ca13ad6e31`.
The [implementation-source review](../docs/plans/time-assurance-integration.md#82-implementation-source-review--27-november-2026)
records the supplied `roughtime 0.2.1` package and pinned Roughenough comparison.
The engineering recommendation is RFC 10049/v1 with pinned Roughenough protocol
and verification code as a reference, NevroTick-owned bounded Engine acquisition
and pure evidence admission, and a separately admitted maintained Ed25519/SHA-512
backend. It does not adopt either complete upstream client or its dependency
graph. `roughtime 0.2.1` is not the implementation base: its legacy-profile
behavior, aggregation and lifecycle limits need independent controls rather than
inheritance. The plan retains exact source evidence and the distinction between
reference-code reuse and dependency adoption.

| Existing surface | Actual evidence and remaining work |
| --- | --- |
| [Workspace](../Cargo.toml) and [Engine graph](../crates/nevrotick-engine/Cargo.toml) | No Roughtime runtime client is present. Keep the engine-only current-thread Tokio arrangement; no core, broker or UI runtime expansion follows from this recommendation. Exact features, transitive/native graph, source/archive parity, registry cooldown and licences remain CHECK-26 admission work. |
| [Catalogue importer](../tools/server-catalogue/build_catalogue.py) and [UI recovery preview](../crates/nevrotick-ui/src/recovery.rs) | The importer preserves candidate keys/profile claims with unknown independence and no supported runtime profile; Ed25519 lengths alone establish no trust. The UI contains explanatory text and plain-helper previews. Neither supplies a runtime roster, signature evidence or bootstrap implementation. |
| [Era-domain owner](../crates/time-domain/src/era.rs) and [pure Core](../crates/time-core/src/lib.rs) | Checked coordinates, numeric windows and era resolution are reusable. They supply no trusted date provenance or Roughtime policy by themselves; TA-W04 must reach the actual pure receiving consumer, not only a disconnected model. |
| [KE time provider](../crates/nevrotick-engine/src/nts_ke_experiment/transport.rs), [strict profile](../crates/nevrotick-engine/src/nts_ke_experiment/profile.rs) and [protected receipts](../crates/nevrotick-engine/src/nts_ke_experiment/managed_preview.rs) | The existing private connection/profile/receipt chain retains laboratory certificate-time classification. A production Roughtime basis does not exist. Preserve original checked evidence through the same configured NTS connection, completed full handshake and protected receipt; a timestamp callback or caller-set authentication label is insufficient. |
| [Windows verifier derivative](../tools/crypto-cng-lifecycle/vendor/rustls-platform-verifier/src/verification/windows.rs) | Actual source is laboratory-only: separate cache-only chain/revocation controls, chain-excluding-root checks, no ignored policy errors, checked FILETIME conversion and a normal-date ceiling. The older upstream candidate's soft-failure/overflow observations do not describe this derivative's current code. Neither profile is a qualified wrong-date bootstrap solution; OPEN-8 remains open. |
| [TLS CNG verification](../tools/crypto-cng-lifecycle/vendor/rustls-cng-crypto/src/verify.rs) | Ed25519 is not enabled in the inspected provider. Existing TLS/CNG or private SHA-512 code does not admit a Roughtime crypto backend. Select and qualify its exact maintained implementation and graph separately; the AWS-LC/ring holds in architecture §18.7.1 still apply. |

These are source observations, not new native results or runtime qualification.
The [current Engine receiving owner](../crates/nevrotick-engine/src/nts_ke_experiment/receiving.rs)
still has no positive production mapping admission. Roughtime does not supply
that missing physical/quality evidence. Reuse the current typed
[TLS failure projection](../crates/nevrotick-engine/src/nts_ke_experiment/transport_fault.rs)
when specifying routing, preserving date errors and unavailable local time
separately from identity, revocation, network and unclassified failures. A failure
class alone grants no retry or alternate-date authority.

## 3. Work assignments and finite exits

TA-W identifiers are local assignments. Each implementation owner freezes the
exact source/graph, selected cases, finite work budget and environment before its
run. Later unresolved choices do not block this documentation handoff.

| ID / owner | Work and required input | Observable exit / current status |
| --- | --- | --- |
| **TA-W01 — coordinator/source reviewer** | Reconcile the request, primary sources, existing KHR/RGT stages and affected consumers | Source/design findings, RFC semantic baseline and canonical routing are recorded. **Revision 1.9 independent reviews completed: ready for documentary handoff only, §6.5. The revision 1.8 receipt remains in §6.4; historical receipts keep their original scopes** |
| **TA-W02 — protocol/security owner** | Use NV-TA-001 §8.4's RFC semantic baseline and §4.2's independent expectations; freeze §3.1's remaining typed construction, packet/work/date/rate limits, aggregation/fault assumptions, exact fixtures, retry-state lifecycle and evidence retention/clearance. Preserve mandatory roots, independent groups, selected sequential schedule, original time frames, generations and refusal. Distinguish date-specific NTS failures and retain compromised-key controls. Integrate NV-LP-001 without a circular bootstrap prerequisite. Freeze reserve-only envelope and expired-NTS-key/common-path threats at the later KHR entry | Reviewed executable contract and independent expected-result table for each selected slice; exact owners, assurance limits and forbidden effects. **RFC semantic mapping complete at documentary scope; V1 P2 engineering freeze pending; reserve-only work post-V1** |
| **TA-W03 — Roughtime/dependency owner** | Evaluate §2.1's source recommendation; admit the exact wire/crypto implementation and graph under CHECK-26, then implement bounded inert decoding and mandatory-root-key evidence construction after TA-W02. Keep protocol-profile fixtures independent of the implementation | Valid response accepted; missing/wrong root, version/context/unit errors, malformed/replayed input and invalid intervals cannot create authenticated evidence. **V1 P2, not implemented; no graph admitted by this review** |
| **TA-W04 — core/engine owner** | Implement the actual pure evidence-admission consumer and model V1 Roughtime/NTS branch selection, receipt consumption, epoch changes, ancestry, uncertainty and outcomes with independent fixture truth. Retain acquisition during leap ambiguity, all-silent healthy controls and fresh post-guard admission under S1.4. Add reserve-specific behavior after TA-W05 | Applicable TA-C01–TA-C07/10/12/13/16/18/21 controls reach the actual consumer; no duplicate credit, stale admission, calendar-vote learning or extra clock controller. **V1 P2 work pending; reserve portions post-V1** |
| **TA-W05 — KHR numerical/security owner** | After actual V1 release, MATH-C01 disposition, TRUST-C01, MT-C01 and CLK-C01, evaluate cold-start reserve candidates, Ananke-inspired admission, initial roster/refresh and bounded sampling against the unchanged baseline; do not import periodic-watchdog security bounds | KHR-0's recovery/error/false-acceptance/latency/cost evidence plus `ADOPT_LIMITED`, `DEFER` or `REJECT`; adoption needs qualified bounded implementation. **Future KHR phase** |
| **TA-W06 — RGT transport/trust owner** | Reconcile the dated census and linked first-schema key inventory by wire version/operator/key/upstream; consume required key/profile bindings, admit update/rollback rules, and qualify advertised UDP/TCP alternatives and finite interoperability work; controlled counterpart first. Observe NV-TA-001 §8.4's acquisition backoff and authenticated reset at actual sends/connect attempts under TA-C14/19 | Exact compatibility, transport/retry and minimum-independent-source success/failure evidence, including TA-C17; terms and observed availability scope retained. Catalogue entries and decoded public-key lengths are not live passes. **V1 P2/P7 evidence pending** |
| **TA-W07 — Windows/NTS owner** | With admitted artifacts and owned disposable environment, exercise the V1 Roughtime date-basis route under OPEN-8/R-02/R-05, exact error categories and fresh isolated verification across hypotheses; exercise the reserve route only after its post-V1 admission | Valid supported-date recovery and explicit failure for bad identity/revocation/era; applicable TA-C11/19/20/22, no unauthorized effect, observed cleanup/restoration. **V1 P2/P7 gate open; existing native admission/holds remain** |
| **TA-W08 — engine/performance/security owner** | Exercise V1 Roughtime failure, acquisition backoff/reset, resource, cancellation, conflict and broker boundaries, including DATA-18 proof/export custody, bounded alert/remeasurement, healthy cached/strict NTS and the absent reserve. Add forced-reserve cases only for a qualified post-V1 profile | Applicable TA-C08/11/12/14–TA-C22 plus integrated cases; aggregate limits, preserved conflict and actual effects observed. **V1 P2/P7 evidence pending; reserve-only work post-V1** |
| **TA-W09 — coordinator/independent reviewer** | Reconcile candidate-specific evidence and remaining policy/operational gaps at each exit | RGT-C01 before V1; separate post-V1 KHR-C01 disposition and CHR-0 dependency. **Both exits open; no combined automatic pass** |

TA-W03/04 do not require a full production service or public network access.
TA-W02/04/07/08 have separately scoped V1 and reserve portions; incomplete
post-V1 reserve work cannot block the Roughtime-only exit. TA-W05 compares the
qualified baseline at its post-V1 entry; simulation supplies no implementation
or Windows qualification for either route.
TA-W07 requires the whole NTS/Windows path, not a successful Roughtime packet
exchange. Split an oversized assignment into bounded slices inside this task.

### 3.1 First portable Roughtime slice — 27 November 2026

This is the next **Roughtime-specific** source slice, not a replacement for
[development §7's current product child](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent).
Reuse TA-W02/03/04 and the existing TA-C/CHECK owners; create no parallel campaign.
Before implementation, record the selected source/graph, actual receiving
consumer, independent fixture provenance, healthy/refused exit, finite work and
verification budget. Later Windows, roster or reserve questions block their
dependent integration, not every portable contract calculation.

1. **TA-W02 — finish one executable contract.** Apply
   [NV-TA-001 §8.4](../docs/plans/time-assurance-integration.md#84-rfc-10049-client-contract-and-acceptance-mapping--27-november-2026)
   and the independent expectations in §4.2. The documentary v1 profile uses
   `S || S || S`: three total traversals of the same randomly selected set of at
   least three admitted operators in one fixed serial order, with a continuous
   32-byte nonce chain. This is the conservative literal interpretation of RFC
   §8.2, not an authoritative erratum. Preserve the selected signature/schema,
   causal, interval, retry and proof rules rather than reopening them implicitly.
   Freeze enforcing private types/constructors, remaining packet/work/date/rate
   limits, capture-error bounds, aggregation/fault model, retry-state lifetime,
   DATA-18 retention/clearance and exact independently checked fixture bytes.
   Bind all controls to the actual producer/consumer and retain the documented
   erratum's fixture limitations. No unsupported clock assumption or convenient
   midpoint may replace the full interval and certificate-admission contract.
2. **TA-W03 — bounded codec and verification.** After exact manifest/feature/
   transitive/native/crypto/licence and cooldown admission under CHECK-26,
   implement the inert parser and mandatory-root-key verification adapter.
   Separate parsed data, authenticated assertions and policy-eligible evidence.
   Bound packet fields, bytes, allocation and cryptographic work before expensive
   operations. An optional-root diagnostic path, public raw-field constructor or
   caller-set `authenticated` flag cannot issue production evidence.
3. **TA-W04 — actual pure receiving consumer.** Admit current evidence into an
   immutable bounded bootstrap-date result or a closed refusal. Preserve source/
   key/catalogue/policy identity, original monotonic observation/evaluation
   instants, evidence age, boot/clock/mapping generations, uncertainty, conflict
   and ancestry. Independent truth must exercise a useful accepted result and
   bad-root/profile/replay/duplicate-operator/broad-interval/stale-generation/
   signed-conflict neighbors at this consumer. No normal NTS vote or broker
   clock-effect authority is returned. Controlled fixture evidence remains
   explicitly separate from a production cryptographic issuer.
4. **Subsequent Engine/NTS integration.** Own bounded acquisition under the
   existing current-thread runtime, original stage/total deadlines and one
   aggregate resource budget. Apply the selected one-second, factor-1.5,
   24-hour-cap acquisition retry profile; only a fully validated, current,
   request- and admitted-root-bound response resets it. This is the stronger
   product reset predicate. A new round, TCP establishment or invalid traffic
   cannot reset backoff, and no retry may exceed the remaining finite budget.
   Cancellation requests do not release permits or
   retained input until actual work completes or its qualified recovery owner
   takes custody. Fresh final guards precede evidence publication and consumption.
   Keep the private date basis with the same NTS connection, full handshake and
   protected receipt; revalidate at candidate time without cross-hypothesis
   resumption. The Windows path remains separately subject to OPEN-8 and TA-W07.

Recording the RFC semantics does not close TA-W02. Its engineering exit still
requires the reviewed enforcing contract, selected limits and independent
fixture/expected-result identities before dependent implementation.

The portable exit is a checked useful interval and observable typed refusals at
the actual adapter/pure consumer, with original provenance and no clock effects.
It requires no public peers, reference-clock hardware or Khronos implementation.
It does not close RGT-C01: controlled transport, roster/operator interoperability,
Windows trust and complete bootstrap-to-NTS/broker qualification retain their
existing owners and valid/refused controls.

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

### 4.1 Source-review refinements — 27 November 2026

These refinements remain **planned, not executed**, under the existing cases and
their canonical CHECK mappings. The
[profile/fixture evidence](../docs/plans/time-assurance-integration.md#823-fixture-provenance-and-remaining-qualification)
records the dated report and limits of the available status evidence for
RFC 10049 erratum 9238; confirm the selected interpretation at the profile freeze.
The original Appendix B legacy contexts
are not a positive strict-v1 oracle. Use independently checked corrected or
generated v1 fixtures, preserve their provenance, and never weaken v1 signature
contexts to make a historical example pass.

| Existing cases | Additional independent observation |
| --- | --- |
| TA-C06/17/19 — profile and authentication | Accept a correct, independently checked v1 fixture with the admitted root; reject legacy/different-version signature contexts, wrong root, delegation/request mismatch and replay. A second signer that duplicates the parser/verifier's own wire convention is not independent interoperability evidence. Retain exact raw bytes and expected units/contexts. |
| TA-C01/04/05/08/17 — measurement procedure and freshness | Exercise the frozen RFC §8.2 sequential chain and required repetition, sufficient independent groups and healthy elapsed-time bounds. Reject duplicate operators masquerading as distinct support, missing required repetitions, reordered/foreign chain state, expired evidence and changed key/policy/clock generations. Arrival or reevaluation cannot renew an original measurement's age. |
| TA-C07/13/21 — uncertainty and disagreement | Retain the counterexample with centers `0, 0, 10, 10` seconds and radius `1` second each: an even-count median of `5` with the maximum input radius describes `[4, 6]`, disjoint from every input interval. That construction cannot authorize bootstrap or clear signed conflict. Pair it with independently supported compatible intervals admitted by the reviewed full-interval rule; check a certificate validity boundary inside the interval. |
| TA-C04/14/22 — deadlines, cancellation and resource custody | Delay decoding/verification or delivery across the original deadline, key/policy replacement, Stop and candidate-date changes. Final acceptance refuses stale results; cancellation retains the permit and inputs until actual completion/recovery. A healthy bounded operation completes and releases custody once. Repeated individually valid requests must remain within the aggregate source/socket/byte/signature/job budget without starving primary acquisition. |
| TA-C02/11/16/19/22 — NTS handoff | Preserve the private original Roughtime basis through the actual configured connection, accepted full handshake and protected receipt. Test unsupported local clock readings separately from certificate date, name, trust, revocation and transport failures. Healthy cached/strict NTS avoids unnecessary bootstrap; absent reserve remains visible. Current laboratory provenance and a successful callback do not establish production bootstrap or OPEN-8 compliance. |

### 4.2 RFC 10049 acceptance refinements — 27 November 2026

These are independent expected results, **planned, not executed**. The sole
client-contract owner is
[NV-TA-001 §8.4](../docs/plans/time-assurance-integration.md#84-rfc-10049-client-contract-and-acceptance-mapping--27-november-2026);
the rows below refine existing TA-C cases and their CHECK mappings, without
creating a second specification or campaign. RFC-W01 absorbs RFC-I01; RFC-M01
absorbs RFC-I02. TA-W02 fixes the independent fixture identities and remaining
engineering limits before the responsible TA-W03/04/06/08 consumer runs them.
Codec or signature acceptance alone never supplies date eligibility, correct
UTC, an ordinary NTS vote or clock-write authority.

| Canonical finding / existing owners and cases | Healthy independent control | Discriminating expected result |
| --- | --- | --- |
| **RFC-W01 — acquisition retry/reset**; TA-W02/06/08, TA-C14/19 | Observe actual UDP sends and TCP connection attempts after successive failures: minimum waits are `1`, `1.5`, `2.25`, `3.375` seconds, with the minimum saturating at `86400` seconds, subject to earlier finite-budget exhaustion. A fully validated current request/root-bound response permits the selected reset; independent healthy acquisition still progresses. | Fixed-spacing failures fail the contract. Malformed packets, invalid-signature/profile grease, stale or wrong-root/request replies, TCP establishment, cancellation/re-entry and round/route recreation cannot reset or bypass the peer/root retry record shared across endpoint aliases and UDP/TCP. Exhaustion stops dependent work; it cannot reduce the minimum, replenish budgets or starve primary acquisition. A helper's delay calculation without observed send/connect behavior is incomplete evidence. |
| **RFC-W02 — bounded framing/maps and extension handling**; TA-W02/03/06, TA-C06/17/19 | Independently encoded smallest one-pair map; nondecreasing equal offsets for allowed empty values; valid empty PATH; well-formed unknown tags, including a correctly signed unknown SREP value and an opaque final value whose length is not a multiple of four. Healthy TCP frames survive split/coalesced delivery. | Reject zero count, truncated/overflowing headers, duplicate or numerically unsorted tags, descending/unaligned/out-of-message offsets and malformed nested extents before allocation/crypto. Wrong magic/length, trailing UDP bytes or combining UDP datagrams cannot create a packet. Unknown tags are ignored semantically; changing signed unknown bytes without resigning fails authentication. No unconditional four-byte total-length rule or TCP trailing-frame rejection may exclude the healthy controls. |
| **RFC-W03 — generated requests and response schema**; TA-W02/03/06, TA-C06/17/19 | Check emitted v1 bytes independently: `VER=[1]`, 32-byte NONC, request TYPE=0, the §8.4 admitted-root SRV selector and zero ZZZZ; a default-size 1024-byte UDP message has a 1036-byte framed packet. A controlled multi-key peer uses the requested admitted root. A correct response has the required nested VER and selected-version membership in VERS. | Missing/wrong-width required fields, wrong TYPE, empty/33-entry/duplicate/descending request versions or mixed-profile contents cannot yield accepted evidence. An unknown SRV produces no controlled-server response; wrong SRV binding cannot yield evidence attributed to the intended root. Missing/moved response fields, invalid VERS bounds/order/membership and unsupported or unoffered response versions are refused under the selected product profile. A deliberately specified smaller-message exception is distinct from malformed encoding and may receive no server response; it is no universal RFC-invalid-packet rule. |
| **RFC-W04 — original signature inputs and scope**; TA-W02/03, TA-C06/19 | Independently checked final-v1 plain-Ed25519 signatures over the exact original DELE and SREP values with their respective complete NUL-terminated prefixes both verify under the admitted root and delegated key. | Missing NUL, wrong case, legacy/swapped prefix, wrong root/delegated key, a modified signed value or either invalid signature prevents authenticated evidence. Re-serialization or signing only recognized fields cannot pass original-byte vectors. Unsigned outer fields acquire no SREP-signature authority. Keep Appendix B's reported legacy-context defect separate from the normative positive fixture. |
| **RFC-W05 — full-request Merkle binding**; TA-W02/03, TA-C06/17/19 | Empty PATH, INDX=0 and ROOT equal to the independently computed full framed-request leaf passes. Correct left/right cases and a 32-sibling proof pass within a sufficiently large controlled request/packet budget. | The same empty proof with INDX=1 fails even when ROOT still matches. Nonzero remainder bits, 33 siblings, non-32-byte sibling extents, wrong child order/root or nonce-only/message-only/re-encoded request hashing fail. Changing the original header, length, versions, extension or padding with the same nonce invalidates the old proof. Cover the depth-32 boundary without a word-width shift. |
| **RFC-W06 — time encoding and delegation**; TA-W02/03/04, TA-C06/07/09 | At decoding, MIDP=1 means Unix epoch plus one second. `MINT=MIDP=MAXT` passes the inclusive delegation predicate with nonzero RADI; legitimate RADI=1 and RADI=2 controls remain valid even when their intervals extend outside delegation endpoints. A separately supported date/interval passes the pure eligibility consumer. | Wrong epoch/units/width, zero RADI, MIDP outside either endpoint, wrapped/saturated arithmetic or unsupported date conversion cannot create eligible evidence. A wrong host clock cannot decide delegation validity. Strict endpoint or universal RADI≥3 rejection fails the healthy controls. Wire/delegation validity and whole-interval product eligibility must have separate results. |
| **RFC-M02 — complete sequence and continuous chain**; TA-W02/03/04/06, TA-C01/06/08/17/19 | For an independently fixed random selection `S` of `N≥3` separately operated admitted servers, observe exactly the selected `S \|\| S \|\| S` schedule: `3N` successful serial request/response links, at least nine. A random first 32-byte nonce and distinct subsequent 32-byte linking random values preserve one chain across traversal boundaries. Retransmission retains exact request bytes and first-send time; only its first admitted reply advances the chain. | Two total traversals, a missing/reshuffled traversal, reset chain, wrong/partial previous-response hash input or aliases counted as separate operators cannot complete the measurement. Timeout, invalid response or RNG failure cannot advance a fabricated link or relabel a partial round complete. A restarted round uses a fresh first nonce and full schedule while preserving conflict, backoff and aggregate budgets under §8.4. The three-traversal total is the documented interpretation, not a claimed RFC erratum. |
| **RFC-M03 — raw all-pairs causality**; TA-W02/04, TA-C06/07/13/21 | The raw signed pair `(MIDP,RADI)=(12,1),(10,1)` passes equality (`11≤11`); the chronologically increasing, disjoint pair `(10,1),(20,1)` also passes the causal predicate. Date-admission/aggregation remains separate. | In receive order `(20,1),(15,10),(10,1)`, adjacent checks pass but pair 1/3 fails (`19>11`). The complete measurement must report authenticated causal contradiction. Checking only neighbors, monotonic centers or interval overlap fails this witness; adding local delay/aging to raw proof bounds cannot hide it. The proof establishes at least one wrong assertion, not which operator lied. |
| **RFC-M04 — processing, receipt and evaluation intervals**; TA-W02/04, TA-C04/05/07/09/14/22 | With zero drift/capture error and zero RTT, `[999,1001]` advances after 60 seconds to `[1059,1061]`. For §8.4's illustrative sufficient enclosure, `m_s=0`, `m_r=2`, `m_e=62`, raw `[1000,1002]` and zero drift/capture error yield `[1060,1064]`; the two seconds of asymmetric delay remain represented. A selected more conservative model may widen these bounds while containing all admissible true-time cases. | A fixed old center, omitted delay, inward rounding or late verification that renews age fails. Missing qualified rate/capture/scale/continuity assumptions or stale generations cannot yield current eligible evidence. A certificate starting at 1060 cannot cover all of `[1059,1061]`; a valid window covering the complete admitted interval passes this date predicate while every other NTS check remains required. RFC §6's notation hazard is not an independently verified erratum. |
| **RFC-M01 — reconstructible proof and local/reporting disposition**; TA-W02/04/08 with DATA-18, TA-C06/13/19/21 | An independently generated valid contradictory chain reconstructs from ordered expected long-term root, exact framed request, exact framed response and each later 32-byte `rand`. Authorized export from the dedicated bounded evidence owner preserves the RFC response sequence with base64 fields; the owner surfaces a local alert and schedules only bounded remeasurement. A consistent healthy chain produces no causal-malfeasance report; ordinary logs remain redacted summaries. | Missing `rand`/packet material, changed header/order/root or altered link yields explicit incomplete/invalid proof, without clearing conflict. Invalid signatures, protocol grease, timeout or broader NTS/Roughtime disagreement alone cannot create a malfeasance submission. Preserve no-default-upload; an explicitly selected future reporting route must satisfy §8.4's conditional endpoint/HTTPS POST/media-type and separate reporting-backoff contract. Storage/export failure or resource refusal retains the safety outcome, not a false complete proof or automatic global ban. |

[DATA-18](../docs/plans/server-catalogue-and-statistics-draft.md#data-18--sparse-evidence-and-unresolved-security-conflicts)
remains the evidence owner with TA-W02/OPEN-DATA-05 for bounded lifetime,
conditional cross-restart safety state, missing-state behavior and authorized
clearance. The RFC tuple refines that existing ownership; it creates neither a
second conflict store nor permanent retention. Local proof generation/export,
alert and another measurement are the selected feasible behavior; their actual
consumers and budgets remain unimplemented. The conditional reporting SHOULD is
deliberately left without automatic external submission under the existing
no-default-upload policy. A later explicitly selected sender needs its own
qualified disclosure/endpoint policy and exponential report retry state; it
cannot use acquisition success or an invalid packet as report authority.

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

### 6.4 Implementation-source amendment — 27 November 2026

Documentation-only scope based on project
`3e3536d808789dabb1b6577b069140ca13ad6e31`: record the
[revision 1.8 source recommendation](../docs/plans/time-assurance-integration.md#82-implementation-source-review--27-november-2026),
the actual catalogue/UI/KE/Core boundaries and the first portable TA-W02/03/04
contract, with focused refinements to existing TA-C cases. The source review
qualifies no dependency, provider, public service, Windows bootstrap or runtime
exit. Historical receipts above are unchanged.

**Completed independent review:** `/root/roughtime_docs_review`, launched with
fresh conversation context (`fork_turns: "none"`), reviewed the seven-file
candidate after its final-ready notification. The substantive candidate tree was
`76c75382c3ce423d4d2467520072043e01b113e8`, based on the project commit above.
The reviewer read the applicable repository instructions, independent-review
skill and testing contract, checked canonical routing and consequential primary
sources, and returned **ready for documentation handoff only**, with no open
material findings. **RGT-DOC-001**, a low-severity intermediate ambiguity about
CNG Ed25519 support, was corrected and independently verified before that freeze.

The raw result is `roughtime-docs-independent-review.md`, SHA-256
`6c038d3f747e8db353a2639b42b856c0083b76301b137dab393daa1abb1fadfe`;
retain it verbatim, including both comprehension paragraphs, with this change's
PR record. Coordinator and reviewer checks passed whitespace, all **29 added
local-link occurrences / 28 unique source-target pairs**, and the seven-file
Markdown-only scope. The working bytes matched the frozen tree. The publication
record binds the subsequent receipt-only amendment to its final tree; no earlier
review is relabeled as a revision 1.8 pass. No product build/test, dependency
resolution, public-peer probe or native qualification was performed for this
documentation handoff.

### 6.5 RFC 10049 audit-integration checkpoint — 27 November 2026

**Documentary candidate base:**
`2583f8b80c106bad410466bafd5cf4416ca60eab`. The completed RFC audit's consolidated
findings and supplied RFC 10049 text inform NV-TA-001 revision 1.9 and this task's
existing assignments/acceptance map. Historical receipts in §§6.1–6.4 retain
their original contents and candidates. No previous review is relabeled as a
revision 1.9 review.

| Stage | Skill / requirement or trigger | Concrete action and status | Remaining owner / next condition |
| --- | --- | --- | --- |
| Scope and decision reconciliation | `plan-code-change`, `plan-architecture`; existing RGT-0/P2 and bounded documentary amendment | Reconciled RFC-W01–W06/M01–M04, deduplicated I01/I02 and integrated ten existing plan/task/architecture/catalogue/handoff files. Reused Engine, DATA-18, NTS and broker owners; retained V1/post-V1 order and holds. Two disjoint editing workers and two fresh read-only reviewers completed their assigned slices | Current development §7 product child is unchanged; dependent engineering remains with the existing TA-W owners |
| Acceptance design | `qa-test-planning`, testing contract and numerical/protocol truth; independent expected results before implementation | Added §4.2's healthy/refused controls under existing TA-C/CHECK owners; completed documentary RFC semantic mapping only | TA-W02 freezes enforcing constructors, remaining constants/aggregation and independent fixture identities; TA-W03/04/06/08 later execute their consumers |
| Documentary verification and commit preparation | `plan-independent-review`, `qa-pre-commit`; combined candidate, canonical consistency and historical-evidence custody | Two independent reviews completed with no material correction; scope and raw-result identities below. Combined added-link/anchor and changed-table checks passed; nine TA-W, 22 primary TA-C and ten canonical RFC refinements remain. Historical §§6.1–6.4 are byte-for-byte preserved. Git whitespace/scope and candidate-content checks cover the intended Markdown-only change; all TA-C results remain planned/unexecuted | The publication commit binds the final tree and review receipts; implementation, interoperability, coverage/mutation and native gates remain open under their existing owners |

**Independent reviews: completed, ready for documentary publication.** Both
reviewers were spawned with conversation inheritance disabled and began their
raw results with two own-words comprehension paragraphs. The coordinator read
both complete results, checked their source reasoning against this candidate,
and retained the reports verbatim, including those paragraphs and file-identity
tables, in the commit message introducing this checkpoint. No reviewer was given
the other's result. Neither review found a material correction within its stated
scope; this is not an implementation-conformance verdict.

| Reviewer / scope | Reviewed identity and retained raw-result SHA-256 |
| --- | --- |
| `/root/rfc_contract_review`; RFC packet/crypto, retry, sequence, interval and proof contract plus eight companion files; task acceptance file excluded | Base `2583f8b80c106bad410466bafd5cf4416ca60eab`; plan SHA-256 `c13622db6822af46bbd782faa9172d0c006073570bee858591d370e1fa8848bf`. Raw report `rfc-contract-adjustment-review.md`: `547105512036bfa22f9f28766392622c1094e57b61b6cfeadced8f09e7d08989` |
| `/root/rfc_integration_adjustment_review`; all ten changed files, actual §4.2 cases, owner decisions, DATA-18, reporting, holds and completion claims | Same base and plan; task SHA-256 `446e69b545e0e19c01606d20d1c22e96c27ea0e5a7b57b404194e311c3dcc027`. Raw report `rfc-integration-adjustment-review.md`: `c16c822684cb412227a6d31cba11a87efbe0fd8949f977d9594fae0939def446` |

After those frozen reviews, only this task's review-status references and §6.5
receipt were updated. The acceptance rows and all other nine candidate files
remain unchanged. The final publication tree binds those receipt-only edits;
the earlier reviewed hashes are not relabeled as final hashes. The coordinator's
bounded Python standard-library link/anchor and table checks, `git diff --check`
and staged-tree inspection supply documentary evidence only. An initial TA-C
count-check locator omitted the table's bold markup; correcting that locator
required no document change. No top-level `docs/SHA256SUMS.txt` exists.

No product build/test, dependency resolution, formal job, public-peer probe or
native execution was performed for this amendment. TA-W02's engineering exit,
RGT-C01, OPEN-8, CHECK-26/27 and the existing source/Windows/resource/roster gates
remain open at their current scopes.
