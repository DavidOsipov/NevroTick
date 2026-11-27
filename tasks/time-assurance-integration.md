# Roughtime and Khronos/Ananke coordination task

**Date:** 3 November 2026, Asia/Tbilisi<br>
**Retained leap-policy integration baseline:** `DavidOsipov/NevroTick@7550f83292f959a6d0acbb2d396e964a6bcf4b01`; 4 November 2026, NV-TA-001 revision 1.2. Earlier amendment and receipt identities are retained below.<br>
**Current design owner:** [NV-TA-001, revision 1.9](../docs/plans/time-assurance-integration.md), including the 27 November V1 scope, post-V1 clock-permissions, implementation-source and RFC 10049 amendments below.<br>
**Mode/status:** RFC semantic baseline revised to 1.9. Controlled wire, keyed verification, three traversals, interval, whole date, conflict retention, invocation withdrawal, shared UDP/TCP retry binding and complete-chain UDP retry reset through controlled recovery passed selected development checks. The full engineering freeze and production qualification remain open. Khronos/Ananke remains post-V1.<br>
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
| **TA-W03 — Roughtime/dependency owner** | Evaluate §2.1's source recommendation; admit the exact wire/crypto implementation and graph under CHECK-26, then implement bounded inert decoding and mandatory-root-key evidence construction after TA-W02. Keep protocol-profile fixtures independent of the implementation | The mandatory-root verifier and wire decoder passed the selected GNU behavior, consumer contracts and cross-target compiler checks in §3.1. Wire and Engine per-file coverage and mutation are separate gates in §3.3. **V1 P2, in progress** |
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

### 3.1 V1 Roughtime admission contract — 27 November 2026

TA-W02, CHECK-6/7/8/9/14/25/26/27. This contract prepares TA-W03's first
controlled wire and mandatory-key adapter and defines what later round and
verifier owners may consume. It applies OD-02 and architecture §§8.1/8.5 to
NV-TA-001 §§4–7. Source base `acbb85bda8b467605f12e66819cd6b36147fd0fb`.
The existing task and plan were clean at entry. The physical-reference trial
and Engine receiving metrics retain their separate prerequisites.

**Actual entry points.** The offline catalogue builder produces provenance,
key material and a profile claim with `supported_profile: None`. For example,
an integer declaration of `1` or a string `IETF-Roughtime` supplies no runtime
profile or root authority. Its future admitted consumer is TA-W06.
The Engine lab's `transport.rs::connection` receives `LocalWallTime` and
classifies the resulting certificate basis as local or fixture wall time.
`transport_fault.rs::classify_tls` can label expiry and not-yet-valid errors.
A first verifier error can mask further failures. The label alone cannot
authorize date recovery. No actual Engine Roughtime round, catalogue-root
admission, full-interval verifier or broker handoff exists in this source.
The UI recovery view is a preview.

**Inert wire and keyed response.** TA-W03 targets RFC 10049 version 1.
An encoded request has `VER = [1]`, little-endian `TYPE = 0`, a 32-byte nonce
and `SRV` equal to the first 32 bytes of SHA-512 over `0xff` and the admitted
root-key bytes. The controlled UDP request fixture pads the message to 1,024
bytes using zero `ZZZZ` bytes, before the 12-byte framing overhead. Require
the selected response version to match and appear in `VERS`. Check the
entire raw list for 1–32 complete, ascending `u32` entries. Check the input
byte cap before copying or decoding. The `ROUGHTIM` header and little-endian
message length must cover the exact input for the single-frame entry.
Check tag table extent, ordered distinct tags, aligned nondecreasing offsets,
and every range before slicing. An empty `PATH` at index zero and a
structurally valid unknown tag remain legal. Admit only the fixed semantic
nesting needed for `SREP`, `CERT` and `DELE`. Require response `TYPE` equal
to the little-endian `u32` value `1`, exactly 32-byte nonce and root, at most
32 path nodes of 32 bytes each, and fixed key, signature, index, timestamp
and radius types. The representation
retains the exact signed and request bytes. A successful parse creates only
inert fields. [RFC 10049 §§4–5](https://www.rfc-editor.org/rfc/rfc10049.html#section-4)

The verifier requires the original request and an admitted long-term
Ed25519 key for that exact source, endpoint, profile and key generation.
Check the echoed nonce, delegation signature, inclusive delegation coverage
of `MIDP`, the response signature and Merkle inclusion of the complete framed
request. Use the two RFC v1 signature contexts with their terminating zero
bytes and the distinct leaf/node SHA-512 truncation inputs. Unused index bits
must be zero. Decoded public-key bytes, a self-consistent response and a
well-formed signature provide no substitute for the admitted root. The pinned
Roughenough validator accepts an optional root, leaves `VERS` membership
separate and does not compare the echoed nonce. Its wire reader copies before
checking framing, accepts suffix bytes at its single-frame entry, parses at
most eight raw versions and accepts zero radius. Its client and crypto
siblings require AWS-LC even with default features disabled, which conflicts
with the current provider bans. The wire-only crate avoids AWS-LC and needs
separate graph admission for `pastey`. The existing Engine graph supplies
fallible `getrandom` but no selected Ed25519 and SHA-512 pair for this use.
Use the upstream code as a comparison during TA-W03. A dependency choice
and its exact Windows/GNU graph remain CHECK-26 work.
[RFC 10049 §§5.2–5.4](https://www.rfc-editor.org/rfc/rfc10049.html#section-5.2)
[Pinned upstream validator](https://github.com/int08h/roughenough/blob/04ca59bf321b46b650574aa9e67900f1a9c216a5/crates/roughenough-client/src/validation.rs#L132-L215)
[Provider policy](../deny.toml)

**Round and interval.** Engine owns the original request, capture brackets,
endpoint, cutoff and work charge. Before admitting a result, verify current
round, key/catalogue, policy, and clock/frame identity, then consume the
original request once. Reject truncation, limit exhaustion, replay and results
past the exclusive cutoff without renewing any age or issuing another send.
Preserve selected, responding and admitted identities and evidenced operator
and shared-clock dependencies. RFC §8.2 applies to the complete ordered
measurement sequence. Select at least three eligible operational servers at
random from distinct operators. Preserve the repeated passes in the same
order and derive each chained nonce from the entire previous response frame
and fresh random bytes, including across passes. Check every ordered pair's
signed bounds and enforce the admitted maximum delay for each exchange.
Live bootstrap additionally requires compatible
independent support and useful uncertainty. Three replies from one operator,
three keys for a shared clock, and repeated passes through a group cannot
provide three independent supporting groups. Missing links or authenticated
contradictions in the selected sequence remain visible. A later timeout or
helper response cannot clear a recorded signed conflict.
[RFC 10049 §8](https://www.rfc-editor.org/rfc/rfc10049.html#section-8)

Use `MIDP` as Unix-epoch seconds and `RADI` as nonzero whole seconds. Derive
the signed bounds with checked arithmetic. When transporting them to a
qualified later evaluation instant, enclose every feasible processing instant
between the original send/receive captures; the unobserved one-way delay
cannot be assumed symmetric. For the zero-rate-error fixture only, a send
at 10, receive at 14, and evaluation at 16 yield elapsed bounds of 2 to 6
seconds. `MIDP = 1,000`, `RADI = 2` therefore permits a conservative enclosing
interval `[1,000, 1,008]` at evaluation. Keep rate, capture and frame
uncertainty in the real bounds. Reject underflow/overflow or invalid context;
do not saturate, refresh the original measurement or narrow to one date.
Date/era, saved floor and leap-rule owners apply their complete-hypothesis
checks to the bounded interval. The wall clock being repaired supplies neither
freshness nor an era preference.

**Recovery use.** Healthy retained/strict NTS prevents a new Roughtime round.
Cold activation requires an unreliable local date, a scoped date-blocking
condition under the adopted recovery policy and a current bounded opportunity.
Keep name, trust, revocation, protocol, transport and agreement failures
distinct. A usable Roughtime interval may condition a fresh isolated NTS
verification attempt. Preserving that ancestry is mandatory even if NTS
subsequently succeeds. A certificate validity boundary inside the admissible
interval refuses that date basis. OPEN-8 still owns the real Windows
trust/distrust, revocation, retrieval and full-handshake checks. Broker
authority requires the existing fresh protected observations, agreement and
current effect guards; Roughtime adds no clock write, NTS vote or extra
correction budget. Signed conflict blocks dependent recovery. An unavailable
or insufficient result retains the existing explicitly enabled helper policy
and discloses the absent V1 reserve.
[RFC 8915 §8.5](https://www.rfc-editor.org/rfc/rfc8915.html#section-8.5)

**First-check controls and handoff.** TA-W03 uses one independently encoded
valid v1 response with a mandatory fixture root, plus fixed wrong-root,
replayed-request, echoed-nonce-only mutation, inconsistent `VER`/`VERS`, draft
signature-context and nonzero-unused-index controls through the actual
checked entry. Include signed zero-radius and malformed `VERS` suffix
refusals, with a valid nine-entry list. Pair truncated, appended-tail,
over-cap and duplicate-tag inputs with valid empty-path and unknown-tag cases.
Preserve a separate decoder arithmetic fixture and check its absolute interval
endpoints independently. TA-W04 then reaches the round/policy consumer for
TA-C01/02/04–07/12/13/16/19, including live-generation refusal, one useful
current handoff, timeout after signed conflict, and a genuinely date-only
activation neighbor. No test uses a public peer or changes a clock. Byte and
signature verification cannot qualify a roster, current trust or a Windows
verifier. TA-W03 binds the exact wire/client/crypto candidate with finite
packet and verification costs before implementation. TA-W04/06/07 own exact
round, interval width, trust/floor, verifier and conflict-clearance policy
before enabling their dependent steps. Those choices are outside the inert
wire exit.

**Independent fixture readback.** In original RFC 10049 Appendix B, all six
signatures failed under the specified `Roughtime` contexts. The same bytes
verified under the older `RoughTime` contexts. The first refusal is retained.
The replacement proposed in erratum 9238 passed all six signatures under the
specified contexts and failed under the older ones. Python independently
read each original framed request and response and verified the two nonce
links, delegation span, zero-depth request commitment and numeric bounds for
each sequence. Under their matching signature contexts, published pairs
1→2 and 1→3 contradict causal order. The proposed replacement instead
contradicts on pairs 1→3 and 2→3. A separate pinned upstream v1 pair passed
both signature checks and refused the wrong-root and old-context controls.
The erratum's status grants no runtime profile. Test the proposed replacement
as individually valid signed replies with a contradictory sequence. Use the
original Appendix B bytes as an exact-context refusal. Retain the specified
RFC v1 context and no dual-context retry. The private readback used bounded
Python 3.12.3 and OpenSSL 3.0.13, with no project code or network peer call
during verification. `context-result.json` under
`~/.local/share/nevrotick/roughtime-appendix-b.xiPKQtMV/` has SHA-256
`63a640cd4aee1393c0e737eec3fc3b5447238f1103e49a9920b0ab7f0bada2ce`.
[Published Appendix B](https://www.rfc-editor.org/rfc/rfc10049.html#appendix-B)
[IETF erratum report 9238](https://mailarchive.ietf.org/arch/msg/ntp/6Flv19F-d52veYryjOxM_iy9MJ8/)

| Stage and skills | Action and status |
| --- | --- |
| Scope, architecture and trust | `plan-code-change`, `plan-architecture`, `plan-threat-model`, `implement-rust-contracts`, `security-protocol-inputs`. Traced catalogue → keyed response → round → isolated verifier → existing clock owner and checked primary wire/version and NTS certificate sources. Source contract drafted. |
| Acceptance and review | `qa-test-planning`, `plan-independent-review`. Fresh reviewer completed the review and confirmed the framing, keyed-verification and round corrections. Offline primary-vector readback passed. Ready for TA-W03 inert wire candidate selection. Rust, graph, fuzz, coverage, mutation and native obligations follow their actual consumers. |

The independent review and recheck are retained in the same private directory
as `independent-review.md`, SHA-256
`1e8a4b11345ec28e8984b5a645c363853ecd82de13b964f21245d2a7221266ce`.

**TA-W03 provider source preflight.** The active Engine lock SHA-256 is
`d0cbe60cb93954940854e14cbf0b27f6d4edececcb06b79dd09377fbd5378515`.
Microsoft Learn MCP confirmed SHA-512 among the documented CNG algorithms.
The two inspected CNG tables identify ECDSA but give no Ed25519 provider
contract. The existing Rustls CNG provider omits Ed25519 and keeps its hash
implementation private. These findings concern the inspected API and graph.
[CNG identifiers](https://learn.microsoft.com/windows/win32/seccng/cng-algorithm-identifiers)
[CNG providers](https://learn.microsoft.com/windows/win32/seccertenroll/cng-cryptographic-algorithm-providers)

For CHECK-26 evaluation, the checksummed `ed25519-dalek 3.0.0` and
`curve25519-dalek 5.0.0` archives declare BSD-3-Clause and publish the same
source revision `07bef73ff85998a206cd2cea7f2605c801d0d1c9`. Dalek's normal
SHA-512 dependency allows `sha2 0.11`. The checksummed `sha2 0.11.1` archive
declares MIT OR Apache-2.0 at `138dc5a05b93f8e49cda177fc6cfd7cc43673e5d`.
Those three versions cleared the 14-day age check on 27 November. The
newer `sha2 0.11.2`, published 25 November, does not. The curve archive has
a build script that invokes Rust version detection and selects the backend
from target and environment configuration, with an x86-64 derive dependency.
Disabling top-level features leaves that host execution. `dependency-updates`
therefore routes the candidate to a full graph delta, exact licence grants,
build and macro source admission, advisory review and actual GNU/MSVC
x86-64-v3 controls. The preflight installed and compiled no crate and changed
no lock or dependency policy. No authenticated provider is admitted.
[Dalek package metadata](https://crates.io/api/v1/crates/ed25519-dalek/3.0.0)
[Curve package metadata](https://crates.io/api/v1/crates/curve25519-dalek/5.0.0)
[SHA-2 package metadata](https://crates.io/api/v1/crates/sha2/0.11.1)

**CHECK-26 guarded provider resolution — 27 November 2026.** The private
Engine manifest adds only the default-off `roughtime-verification` feature,
with exact `ed25519-dalek 3.0.0` and `sha2 0.11.1` pins and both crates'
defaults disabled. The [repaired accepted baseline](build-optimization.md#clear-the-url-advisory-under-the-approved-cooldown-exception)
retains the original URL decision as history. The product metadata-only
resolver ran Cargo 1.100.0 `update --workspace` under ordinary 14-day deny.
The actual cutoff was 13 November 2026 at 13:34:20 UTC. All seven added
registry records had earlier publication times and matching checksums.
No exception was applied.

| Added lock record | Version |
| --- | --- |
| `curve25519-dalek` | 5.0.0 |
| `curve25519-dalek-derive` | 0.1.1 |
| `ed25519` | 3.0.0 |
| `ed25519-dalek` | 3.0.0 |
| `fiat-crypto` | 0.3.0 |
| `sha2` | 0.11.1 |
| `signature` | 3.0.0 |

Readback compared all 603 predecessor and 610 proposal records. No record
was removed. The retained Engine record adds exactly the two direct
dependencies. All other predecessor records and their edges match. The
configured quota admitted 532,738,048 bytes and 16,000 inodes on a fresh
private unprivileged ext4 FUSE mount. Its first service stopped before
resolution because it used the wrong working directory. The corrected
service passed in 79.794 seconds, with 601 MiB peak memory and zero swap.
The mount, image and service retired. The tracked Engine manifest,
`Cargo.lock` and dependency policy remain unchanged.

| Evidence under `~/.local/share/nevrotick/roughtime-provider-resolution.YkcpgESo/` | SHA-256 |
| --- | --- |
| `manifest.patch` | `5c5ca5db65509437e098799b4b39963340935d7d9dfaf641569a7df4ac363d1d` |
| `unit.log` | `2ed6bc9e94aaa0131387d9d642d1dd93e19cc1515371733c0e9ef44d193ad72e` |
| `resolve-unit.log` | `4ac0fae7b02f9c02ba9a4e2a5dee62d5f3ed827ecb42f39488d6cf7a0436a7c0` |
| `resolution/cooldown-resolution.json` | `9ab873b04740136d032391884573f5355584162bffe71456fa384254fb1b9209` |
| `resolution/Cargo.lock` | `7c8b1bd0d043f7866f6c944dcdec97a187dcdb23c1b90a0dc055ccf672a9abb6` |

The next CHECK-26 gate is independent review of the additional archive
sources, licences and advisories, followed by actual GNU and MSVC feature
and target projections. Bind the x86 derive macro and Curve build script
to the actual rows. The conditional `fiat-crypto` lock edge requires a
target projection before attributing any build or runtime role. The
candidate API review found plain Ed25519 `verify_strict` with defaults
disabled. Preserve the exact RFC context bytes, canonical key encoding,
weak-key and signature rejection, and absence of `legacy_compatibility`
in the actual graphs. Provider approval, compilation and the keyed
constructor remain open.

**Provider source review checkpoint.** Use the seven exact registry records
above and the retained lock and cooldown receipt. Rehash the three retained
archives and acquire only the four missing archives into a private review
directory. Bound each acquisition to 4 MiB and 15 seconds. The archive
readback has a 120-second execution budget including acquisition, bounded
decompression and recording the manifest, source and licence bytes. Keep
execution limited to the download and readback tools. Independently check exact
advisory ranges and trace the admitted graph projection. Inspect the
Curve build script, the derive macro and the conditional Fiat backend before
any compile. This checkpoint leaves the tracked graph and product feature
policy unchanged.

**Feature discovery.** Reuse the existing calendar graph-discovery approach
with the configured product metadata and tree commands. Use one private
source copy with the proposed lock, a verified copy of the admitted
baseline vendor and the seven exact review archives. Select default, EP1,
Roughtime and the combined EP1/Roughtime rows under both GNU and MSVC. Apply
the new feature only to workspace and Engine selections. Keep the
feature-off and existing EP1 rows as controls, and preserve raw package
scopes and tool, source and vendor bookends. Use one serial confined service
with a 200-second total execution window and the configured offline
invocations. This is projection evidence for policy review. Compiler,
product-policy, cargo-deny and native checkpoints remain separate.

**Executed source and feature discovery.** Three retained and four newly
acquired archives matched the proposed lock's registry checksums. The
bounded archive reader verified 237 regular files and 6,709,336 expanded
bytes. Both Dalek libraries supply BSD-3-Clause grants. The derive crate's
manifest uses `MIT/Apache-2.0` and carries both licence texts. The Ed25519,
signature and SHA-2 archives also provide MIT alternatives. Fiat carries
MIT, Apache-2.0 and BSD-1-Clause alternatives and remains a lock-only
entry. Retain the exact notices in any admitted vendor or build inventory.

The source readback examined Curve's build-time Rust version check and
backend selection, and the host macro's target-feature wrappers. Curve
and SHA-512 have CPU-dispatch branches in the reviewed source under
ordinary configuration. Metadata alone supplies no instruction or
Windows execution evidence. Keep both the generated unsafe calls and the
actual CPU admission in the next compiler and runtime review.

The independent advisory readback bound the seven archive identities to the
proposal and RustSec revision `c5279d0b0c9e8b70df666eb6d1edbabcfd1176bd`.
Three advisories attach to the selected package names, and each proposed
version satisfies the documented patched range. The other four names
have no record in the retrieved revision. The corresponding exact
versions were non-yanked in the retained resolver index. Fresh
full-lock cargo-deny checks remain at product-policy admission.
[Curve advisory](https://rustsec.org/advisories/RUSTSEC-2024-0344.html)
[Ed25519 advisory](https://rustsec.org/advisories/RUSTSEC-2022-0093.html)
[SHA-2 advisory](https://rustsec.org/advisories/RUSTSEC-2021-0100.html)

The first discovery service passed its isolation control and stopped
before Cargo because its operation position assertion addressed a
configuration argument. The corrected service passed 88 frozen offline
metadata and tree commands plus the isolation control in 37.304 seconds,
with no package compilation. Its 2,428-file source and 29,754-file vendor
bookends and tool identities matched. A separate readback inventoried and
checked every raw row. It verified 32 byte-identical unrelated package
scopes across the two profile pairings and both targets.

GNU and MSVC Roughtime and combined rows select Curve with the `digest`
feature and its custom build script. The derive proc macro is a host
dependency with empty features, reached through a normal Curve edge.
The separate `rustc_version` build edge runs on the host. Ed25519,
Ed25519-Dalek, SHA-2 and signature are target libraries with empty
features. Fiat appears in the lock and in none of the selected trees or
filtered metadata. Neither feature-off control selects the seven new
identities. The combined row adds `mac` and `zeroize` to the observed
Roughtime-use context of `digest`; Windows combined adds
`const-generics` to the existing `subtle` context. The actual
combined profile needs its own exact policy rows.

| Evidence | SHA-256 |
| --- | --- |
| `roughtime-provider-source.YyUaCphj/readback.json` | `f00896eaddf21d7a385a99a190d6a330584050ad15b5412c67caa4f6256913c4` |
| `rgt-rustsec-delta.1HBjKD47/receipt.json` | `9c4e02c02c6d7d3a43f34c689950226c6bb63f89d1fedf5c85e84755841825f3` |
| `roughtime-provider-projections.UkzSjBh4/receipt.json` | `7ca53333e95fa91b3ec282f4effc85996b6998208c902f3ba79041b5abc8cbbe` |
| `roughtime-provider-projections.UkzSjBh4/projection-readback.json` | `d176612586f9b7151dd405573297ba8367afde89151cb7c2477c3a0393b45286` |
| `roughtime-provider-projections.UkzSjBh4/unit.log` | `7452fc3044196b532a3089286fc5adbca06cac559dfab24a8468ebc28612d9f1` |

Paths in the table are relative to `~/.local/share/nevrotick/`. The
tracked manifest and dependency policy still retain the accepted
baseline. Add the closed Roughtime and combined profile handling to the
existing graph owner before admitting any provider build. The current
policy accepts only default and EP1, and its existing negative controls
remain part of that change.

**Lock-bound advisory acquisition.** Use the configured metadata refresh
owner on the private proposed lock and exact Cargo configuration. Limit
the acquisition to the pinned Cargo and cargo-deny tools, public
registry-index and RustSec metadata, and a separate quota of at most
512 MiB and 16,384 inodes. Keep the confined execution and any causal
repair within 120 seconds, with a ten-second retirement allowance.
Export a fresh snapshot and receipt
for the configured graph check. The existing tracked policy and archive
cache are outside the writable boundary.

**Acquisition and readback passed.** The initial ext4 FUSE quota allowed
the locked registry-index stage, then Git could not create a temporary
pack file during the RustSec stage. It retired in 19.428 seconds with no
snapshot. The replacement used a private user and mount namespace to
create a bounded 512 MiB, 16,384-inode tmpfs. The first preparatory check
refused access to PID 1's namespace link before mounting. The corrected
check compared the host and current namespace identities through the
launch record. It proved unprivileged temporary-pack creation, then ran
the same pinned acquisition with UID 1000, zero effective and ambient
capabilities, a read-only candidate and no-new-privileges. The service
passed in 9.318 seconds. Both configured acquisition commands exited
zero, the quota retired and the lock was unchanged.

The exported snapshot contains 2,174 registry-index and RustSec metadata
files. The existing `verify.py` snapshot consumer rehashed and admitted
the complete proposal-lock snapshot, and refused the same receipt for
the predecessor lock. Its first readback stopped at import without a
private HOME; the corrected confined readback passed. This establishes
acquisition and current-lock custody. Configured selected-graph
cargo-deny results remain open.

| Evidence | SHA-256 |
| --- | --- |
| `roughtime-provider-advisory.fGPEegpc/unit.log` | `29793587c63c4abca90db035208365d826c2cd93c311af97f292cafacca87c34` |
| `roughtime-provider-advisory-tmpfs.hAx6V9fz/quota.json` | `fa74bab304e8534b1b6b133c3d24cebf7c04f1f6653ba24693666a4fe45158eb` |
| `roughtime-provider-advisory-tmpfs.hAx6V9fz/export/receipt.json` | `fb2090432b69b7958ba700fead3356fd65bd8693d1dc0e9507a6b08ef82a0930` |
| `roughtime-provider-advisory-tmpfs.hAx6V9fz/admission.log` | `d1a5101aecbcca4f9d6c39f6fc6967f31f02c061f984e52e2533e0dc91dd3d98` |

Paths are relative to `~/.local/share/nevrotick/`. Reuse is subject to
the configured 24-hour freshness and exact input checks.

**Closed graph policy checkpoint.** Extend the existing graph owner and
configured dependency runner for the Engine Roughtime and combined
KE/Roughtime profiles. Freeze the generated candidate using the
observed rows, reviewed archive maps and exact cooldown receipt. Keep
the original default and EP1 command and graph behavior; add the
Roughtime feature only to the workspace and Engine metadata and tree
selectors. The dependency checkpoint adds no compiler or artifact
commands. Exercise healthy selector and policy projections, combined
feature unification, and refusal of unrelated scopes, broad features,
metadata-only promotion or a wrong build/host role. Use one 60-second
focused Python window and a 360-second aggregate window for the four
configured dependency profiles, including one causal correction and
retirement. Retain complete fresh metadata, target-tree, advisory, ban,
licence and source results before accepting the lock and policy. Use the
existing separate provider compilation and keyed behavior checkpoint
after graph admission.

**First configured policy readback.** The new profiles now select exact
dependency rows and refuse portable and Windows build entry. The graph
owner accepted all eight retained GNU/MSVC projections and the focused
selector, policy and cooldown suites passed. The first configured service
stopped after its boundary control because non-test rows were forwarded to
the test-accounting hook. The corrected verifier accepts those rows without
test credit and rejects product-test accounting for the dependency-only
profiles.

The repaired service passed the GNU Roughtime metadata and all selected
tree rows. Cargo-deny passed advisories, licences and sources, then
refused `curve25519-dalek@5.0.0/tests/build_tests.sh` under the interpreted
file rule. The exact retained candidate receipt SHA-256 is
`90e57f269e27f1821a3466041524eaa96bb095568ec8d11acc8c976bfe846587`.
The service used 71.004 seconds after the first 11.038-second attempt.
Independent archive review found this single upstream manual backend
test harness, SHA-256
`7adcf715a27bb24482a9828be3a44f93a944756f7b80042d032a34fb10565f51`.
The selected manifest, build script, derive macro and product workflow
have no invocation of it. The resulting candidate records this exact
presence-only scanner allowance in the existing archive-file policy.
General executable and interpreted denials remain in force. The
upstream script was never run.

**Configured graph checkpoint passed.** All four profiles passed their
30 configured rows on the same private candidate. GNU and MSVC each
returned passing advisories, bans, licences and sources in every profile.
The four runners used disjoint writable scratch and a shared bounded
service, with read-only candidate, vendor and advisory inputs. The final
service took 105.609 seconds and peaked at 922.6 MiB. The three services
consumed 187.651 seconds of the 360-second window. Candidate bookends
matched and the service retired.

| Exact report under `~/.local/share/nevrotick/roughtime-policy-admission.FV5zB1g0/` | SHA-256 |
| --- | --- |
| `default.json` | `59e17b35436d662e2a09bbd8ba9f066c4bfefd0075c1041c4e408357572f648a` |
| `engine-ke-association.json` | `0d8c9ff36cc58f36f942d5933238d01faac33f4f4368dd842cd9584322a87c86` |
| `engine-roughtime-verification.json` | `9047effafa336e55206643dee74a628d0bf882c53c7f7e9a94c520102ac7e1ca` |
| `engine-ke-roughtime-verification.json` | `73950f364b9156af7fe9f9e014a8579e6fce1ff5265b2d9fa5e61d10035ced80` |
| `tracked-admission.log` | `df0e9fb5391b2c426c52cd4f985122a59c595c434f01054f5cd02551cc614968` |

Integrated that exact Engine manifest, lock and policy. Canonical policy
SHA-256 is `c60e4cc7a22a665adfb102a28378b902dede9c92ad1396f5ccd6bf24d1665e2d`.
The accepted cooldown checkpoint binds the proposal lock, registry
inventory and retained URL exception history. The historical URL
control now reconstructs its exact registry independently of the live
lock. The separate tracked-source admission rehashed the graph inputs
and passed 99 Python controls after registration and fixture updates.
The tests distinguish the graph's current ordinary resolution from
the historical exception. The profiles still admit only the configured
dependency lane. Actual compiler and macro execution, keyed response
controls, native work and mutation evidence remain open.

**Mandatory-root verifier checkpoint.** The wire owner decodes the exact
original framed v1 request, including required `SRV`, without allocation.
The Engine's default-off verifier requires the caller-selected key,
matches the original server commitment and echoed nonce, and uses strict
plain Ed25519 with the two zero-terminated contexts. It checks both key
encodings, uses the decoder's original delegation and response slices and
time bounds, and authenticates the complete original frame through the
ordered Merkle path. Bound work to two signature checks, one leaf and at
most 32 node hashes per response. The consuming call returns original frame
and root custody. Source admission, deadlines, nonce generation, network
transport, replay across reconstructed requests and current round policy
remain with TA-W04/06.

Use the existing isolated product snapshot, reviewed vendor and Rust
1.100.0 in one 240-second development window including one causal repair
and retirement. Run selected GNU wire and signed-fixture tests, the
external consume-once doctest, and GNU/MSVC Clippy on the reached Engine
test target. Check the combined KE and Roughtime target if the same window
admits it. The current configured dependency profiles retain their
dependency-only contract. No public peer, clock change or native Windows
run is needed for this checkpoint. The affected wire line-coverage rerun,
per-file mutation and final integrated product qualification retain their
own gates.

**Execution and disposition.** The first 791-input candidate
`ae1e5cca6023688f4ece6c0d032f364221027defcaa567d57ce92ed2e98e972b`
passed isolation and formatting, then GNU Clippy reached rustc error E0015.
The pinned provider exposes `VerifyingKey::as_bytes` as a non-const method.
The one permitted repair removes `const` from the borrowed key accessor.
The repaired 791-input candidate
`08f76a68bc3eb3ce203236ecacf31b5c59b028d1758410b8cbd48bf8258c1f71`
passed isolation and formatting. Clippy stopped with 19 diagnostics, including
missing `# Errors` and `# Panics` sections in the tests. Both services retired
their verifier scratch. Their combined
service runtime was 42.612 seconds. The repair allowance is spent and the
checkpoint remains incomplete. GNU wire and signed behavior, doctests,
MSVC Clippy and the combined-target check did not reach execution.

The independent fixture readback used OpenSSL and SHA-512 to verify the
19 exact signed-vector files and 22 signature observations, including
the old-context refusals. It supplies fixture evidence. The actual
Rust consumer and metrics retain their own checks. Address the test
contract documentation before a later admitted candidate checkpoint.

| Retained receipt | SHA-256 |
| --- | --- |
| `roughtime-keyed-check.f4sPh8ci/attempt-a.json` | `55bd8dec35bdb0255e968d2e27833de668cf740125ec98663c131c7db00161e0` |
| `roughtime-keyed-check.f4sPh8ci/attempt-b.json` | `e832068c0c918c468d94f9279886d8ea6fbb195edd82bfcf362f735220a022c4` |
| `roughtime-tests.Rn7BCKXc/verified-archive.txt` | `df1ba341643b18450a729805a3a4c4a7edf0b5d724b2b6c2d8213fb71cd7eac6` |

**Signed-fixture contract continuation.** The nine fallible test entry points
document their propagated errors and their assertions. The canonical encoding
test documents its assertions. This postimage changes no test body, wire
implementation, fixtures or dependency graph. Re-admit it through the same
isolated selection and verifier, with at most 190 seconds including retirement
within the unspent original wall allocation. Preserve both previous receipts.
Require all ten named signed cases,
the wire integration cases, affected doctests, both selected Clippy targets and
the combined-feature compile check.

GNU Clippy passed. The signed suite passed nine tests and revealed a fixture
defect in the tenth. Byte 164 is the start of `TYPE`, already 1. The Merkle
index starts at byte 472. The first attempted direction change left the packet
identical and the verifier correctly accepted it. Preserve this 31.306-second
failed run (`contract.json`, SHA-256
`52bda6a362babc150a6717e2f5dbb11ac5d6a9ef06bf84f4a0d1222ee0d674bb`).
Correct the two index offsets and label the three refusal oracles. Allow that
different fixture postimage at most 160 seconds including retirement. The
earlier services and this run together used 73.918 seconds of the original
240-second wall allocation. The first corrected candidate stopped in
formatting before any Cargo build or test. Preserve its 11.036 seconds and
`merkle.json` SHA-256
`04fc828dc8c3683ecb0bf3a2f338826696c3929d4be380514d3048d51506f371`.
Apply the pinned Rust formatter and precheck that file before refreezing.
Use at most 145 more seconds including retirement, within the original
allocation. Keep the existing per-command limits.

The formatted correction passed all nine selected lanes on the 791-input map
`3535bcf65900fe83427c847f88e72549b06f5a818b685b0e9ea4e2aea44f377c`.
GNU ran all ten signed Engine cases, 12 wire integration cases, 92 Engine
doctests including two Roughtime external refusals, and four selected wire
doctests. Both GNU and MSVC selected Clippy commands passed. MSVC checked the
combined KE and Roughtime features with the signed integration target.
The receipt matches the admitted graph, source, vendor and compiler at both
ends. The service and scratch retired after 61.147 seconds, for 146.101 seconds
total across the recorded keyed attempts.
`roughtime-keyed-check.f4sPh8ci/formatted.json` SHA-256 is
`05641f9398b83c5a3c3c6ffb85cbac6072d22175a433205c456c354c3b73b42e`.
The later inventory registration changes only `policy/verification-manifest.toml`
among these bound inputs. The source and compile results retain the exact
preceding map. Native Windows execution and full product qualification remain open.

The bounded inert decoder uses fixed nesting and a borrowed parser with no
upstream wire dependency. Its exact-frame and field controls use independent
fixed bytes. An Internet trial retains its separate admission.

### 3.2 TA-W03 inert response decoder

CHECK-6/25. Use a distinct `ntp-wire::roughtime` module. The existing pure
wire crate can enforce the borrowed `no_std` boundary without adding a
workspace member or a dependency. The Engine remains the owner of request
custody, cryptographic verification and live admission. The inert decoder
uses a 4,096-byte complete-frame cap, fixed nesting and bounded lists. This
cap fits the full RFC-sized path and version list plus bounded opaque
fields. The later transport may impose smaller request-bound limits before
buffering. The decoder retains the original complete response and both
exact signature inputs. It checks the wire interval and delegation span.
The future keyed caller checks the actual signatures, request commitment
and current authority.

`implement-rust`, `implement-rust-contracts` and `security-protocol-inputs`
own the source. `qa-contract-tests`, `qa-boundary-edge-cases` and
`qa-regression-selection` exercise the real public decoder through
independent framed bytes and an external checked-type consumer.
`qa-structured-fuzzing` retains the applicable parser checkpoint with
exact-frame and authenticated seeds kept separate. Keep the existing
coverage and per-file mutation gates. Use up to 240 seconds total for
confined focused GNU and MSVC checks, including one causal repair, with no
network or native actor. Capture the first refusal and command identity.
The configured broad workspace rows retain their own admission and exit.

**Development checkpoint.** The fixed-nesting, borrowed decoder is in
`crates/ntp-wire/src/roughtime.rs`. It preserves original response and
signature inputs, checks the full version list, and refuses invalid path
indices, numerical bounds and delegation spans. Its output has private
fields. One security-module entry and one critical-file entry identify the
decoder for later qualification. The workspace and `Cargo.lock` are
unchanged.

The confined package check used the stock verifier's product, vendor, tool
and network/filesystem boundary admission on a frozen Git archive with
five reviewed overlays. It retained the 4 GiB reserves for both the frozen
source and scratch. The successful snapshot had source-map SHA-256
`f95c46c58b5a24af8e77f83ca3caa47d49f52c76b6e51468df5b4b2f56e6fe6f`
and matched 29,510 vendor files. GNU formatting, all-target Clippy, 128
ordinary cases, 88 doctests and Rustdoc passed. The ordinary suite included
all nine Roughtime cases. The doctests included both new external privacy
controls. The MSVC all-target Clippy lane passed compilation. The independent
readback matched exact source bytes, the names of the executed Roughtime
cases and scratch retirement. Native execution, authenticated provider
admission and coverage, mutation and fuzz qualification remain open.

The first selection stopped at eleven production lint diagnostics. The
second reached two fixture lints, and the third found a missing type on an
empty-slice comparison. All stopped before libtest. These separate fixture
corrections exceeded the planned repair count. Retain those incomplete
selections as well as the final pass. All four services consumed 49.245
seconds of the original 240-second wall allowance and retired. The fresh
review led to discriminating privacy, cap precedence, numeric ordering and
opaque final-value controls. The fixed bounded public table geometry rejects
a fractional version word before the semantic version checks.

| Receipt under `~/.local/share/nevrotick/roughtime-wire.AlJcv96R/` | SHA-256 |
| --- | --- |
| `attempt-a.json` | `27bd93e6b0cdf52054b41a488306582c57a84b4628446717e771a492cad3c5ab` |
| `attempt-b.json` | `1fa67c1a56ef3d2e91cd6aa46802ed6435b6bb4a6649e1a97f51eea95696d160` |
| `attempt-c.json` | `115a4e804fa00496699b37c5433c02c8ca2ae68af34ea603c1cad25b2030b141` |
| `attempt-d.json` | `7ded8a4d34c56cb6746e9b233780829366ef976d3d5b75e7a5be9d7a2054d764` |

Next, admit the new subject to the existing wire-analysis checkpoint and
bind the exact CHECK-26 provider candidate for keyed verification. The
current Roughtime registration leaves the existing 90% and 95% gates open.
The proposed RFC erratum vectors still need to reach that keyed constructor.

### 3.3 TA-W03 decoder coverage and inventory admission

CHECK-6/25. The existing wire runner will select a separate Roughtime-only
coverage and enumeration role. Preserve the historical seven-owner policy,
its contract claim and the scoped fuzz hold. The new role admits the stable
tool payload and selected extensions, freezes the complete product and vendor
inputs, and uses the shared per-file line and mutation-descriptor readers.
The enumeration role stops before any baseline or mutant execution. A
separate campaign requires independent review of the generated inventory and
guards it cannot exercise. Keep the existing 90% line and 95% viable mutation
requirements. Neither scope selects authenticated verification.

`qa-coverage-gap-analysis`, `qa-mutation-testing` and
`qa-environment-preflight` own selection and resource admission.
`qa-contract-tests` and `qa-regression-selection` verify that the new role
cannot borrow an old subject, the fuzz route or campaign authorization.
Use the existing runner and one private frozen checkout, two serial selected
analysis commands, and a total four-minute window including input admission
and retirement. Validate the actual test run and emitted source file or raw
descriptor roster. Retain partial output on refusal. Existing fuzz, native
and provider gates retain their separate exits.

**Executed checkpoint.** `verify-wire-analysis.py --subject roughtime`
accepts only the exact `coverage` and `enumerate` selections. The existing
seven-owner scope and fuzz bytes retain their original identities. The
new stable-core path requires an external private Linux output with a 2 GiB
entry reserve. The earlier 4 GiB development-wrapper floors belong to that
separate execution path. Both selected commands used the same frozen
778-input analysis map, SHA-256
`4c5dba4c85f2e0b9712ea4a75ab586a205782319d27805f888b789353f9f18fc`.
The final confined verifier suite passed all 67 controls. It included
exact file attribution, the 90% boundary and wrong-file/duplicate refusals,
the listing-only command and refusal of a foreign descriptor.

All nine Roughtime tests passed under GNU instrumentation. The exact decoder
file reached **184/186 executable lines (98.92%)**, above the 90% gate.
The uncovered lines are the two short frame-prefix returns at lines 175 and
178. The report had 203/230 covered regions and no branch denominator.
Enumeration retained 112 unique descriptors from the exact decoder. The
normalized inventory without rendered diffs hashes to
`7d692f15f1b5e858a8400258646339c6ab87ff24db3b45610d0ccb413e3ac73e`.
The supervised service exited zero in 39.866 seconds, with zero swap and
no surviving PID. The owned frozen copies and analysis results are retained.
This checkpoint supplies no mutant execution or 95% score.

| Evidence under `~/.local/share/nevrotick/roughtime-metrics.c3SLHHod/` | SHA-256 |
| --- | --- |
| `controls-final.log` | `3301833415fdf48a9ace7411a59f8a28f7f98dbfa2d92fb4ef9281e0a2d0088a` |
| `coverage/verification.json` | `2dd5ae9d6177185b8032d24f050248b955bc2caa40e190312c10f71b28d0f988` |
| `coverage/work/coverage.json` | `ddf37903241529f47969d5eb784aba600df5a887d2f9920426f8ad753c56e56b` |
| `enumerate/verification.json` | `6b37c884b42c0704df5b014c91b6cc2daa60a70721c26eb415d6a4b9b1ce496a` |
| `unit.log` | `3ec3692994874963ec6c2680385ca6d2791d9c4f35cfe2e758c4b2f1257e9e15` |

**Refined boundary controls.** The fixture now refuses incomplete magic and
length prefixes and accepts a legal zero-width final opaque field. The last
offset in the accepted packet equals the values length. All nine GNU tests
passed, and the exact decoder reached **186/186 executable lines (100%)**.
The two analysis commands used the same 778-input map,
`2f391942708f6c4e4876598addc6a7511fd71877446a4e740a1d154cb18343f9`.
The Roughtime test file was the only change from the first analysis map.
All 112 descriptor records match the first enumeration. The second unit
exited zero after 46.603 seconds, bringing the two service runs to 86.469
seconds. The unit is inactive with no surviving PID. Mutation viability
and disposition remain open, including the 95% gate.

| Evidence under `~/.local/share/nevrotick/roughtime-metrics-refined.4AZiTT0g/` | SHA-256 |
| --- | --- |
| `coverage/verification.json` | `027e7db608a3ab450ffad08de8b71b0b54eb3e8b527f81ebe99c72e9c4d40f83` |
| `coverage/work/coverage.json` | `ab087e680f9946b5daa92f82290dbd1fd372dfa526d86f64a48081f70dca96f3` |
| `enumerate/verification.json` | `9c8a2eb9214617affd8df0c4df1d4753b07a562e43992e75753c7d4faf603fc8` |
| `unit.log` | `909b990c4825df76bb3456d409790b690783a35892fe708e672d0a17fd169377` |

**Request and response candidate recheck.** The inert wire owner now covers
both framed directions. Register the feature-gated Engine signature and
commitment verifier as a separate security module and critical file. Use the
existing wire role for the 12 named tests and fresh descriptor listing against
one frozen current candidate. Preserve both earlier analysis receipts and
charge up to 140 seconds including retirement against the original four-minute
allocation, after 86.469 seconds already used. Preserve all existing internal
command floors and time limits. The wire role cannot supply Engine metrics.
The Engine owner needs its own exact feature-specific analysis selection and
the signed integration consumer. Both files retain independently reviewed
viability, generated and ungenerated behavior, and per-file mutation exits.

The first placement entered admission, then refused the reserved `/dev`
output namespace before any analysis subprocess. It consumed 12.102 seconds.
The same frozen source with private ext4 output passed both selected commands
in 48.403 seconds. Both services retired. The four historical and current
analysis services together consumed 146.974 seconds of the original
240-second allocation.

Readback matched all 799 inputs to the current worktree in both receipts.
Their canonical source-map SHA-256 is
`69ffd33c78b56be1d5d8747509b0c98af04137ef7c0d4ecd6397b2c03716a6a6`.
All 12 named wire tests passed under GNU instrumentation. The exact wire
file reached **210/210 executable lines** and 239/263 regions. The tool
emitted no branch denominator. The enumerator retained 123 unique descriptors
for this file, including 11 across the request constructor and three request
accessors. Its descriptor-map SHA-256 without rendered diffs is
`2dc4ac1323d9653ac22071e1cf8470cd8559fa8cb568c5e1f9b8f11442206551`.
Determine viability from actual bounded execution and review what the
generator cannot vary. Retain the independent outer-table geometry witness
for the fractional `VERS` case from §3.2. Mutation
execution and the Engine's separate analysis role remain pending.

| Evidence under `~/.local/share/nevrotick/roughtime-wire-request-metrics.kMrdtLHr/` | SHA-256 |
| --- | --- |
| `reserved-path-incomplete.json` | `f0154a891c45983ac2b25210153fced305193a51cd980d0e71058cf97c943f4e` |
| `coverage/verification.json` | `abfbf09c232e45c92f2c93ba5f74fe30d4919073e87ea5730228a2325f4904fe` |
| `coverage/work/coverage.json` | `84d212918dc550b07df59c0840397728bc0ca89ba2a8befc3434549ebceaca7c` |
| `enumerate/verification.json` | `1702278a8211ca144d3256a11f79265b3c17289a9d0b89b6fd5539913e502e75` |
| `ext4-unit.log` | `47075a4a3310a3f738abb08923760df883cfa9553ca09957de0f07cd13b67cd3` |

### 3.4 TA-W04 original controlled exchange

CHECK-6/7/9/21/25/27. Connect actual Engine UDP submission to receipt
publication through one nonrenewable authority under the existing default-off
feature. Bind one numeric loopback destination and selected root. Allocate the
nonce at submission, encode the fixed 1,036-byte request, and retain the
original successful send bracket and cutoff. Charge each nonblocking read.
Consume the first datagram or terminal native error before bounded signature
work. Retire the socket before verification. At receipt publication acquire
the same authority, recheck its withdrawal and read a fresh monotonic instant.
The receipt retains the original frames, root, peer, signed open interval and
monotonic brackets for the future round consumer. It supplies no independent
source vote, candidate date or clock effect.

Use the Rust contract and protocol-input skills at the send/receive/commit
boundary. The independent peer reads the actual transmitted bytes and builds a
reply using the retained signed fixture and its public delegated test key.
Require one correct publication and replay refusal on a fresh attempt, all six
post-verification withdrawals, no datagram after pre-send Stop, and refusal of
an otherwise valid signed reply from a different port. Use the selected
Roughtime verifier recipe with a 220-second total window, including one causal
repair and retirement. Preserve its per-command CPU, isolation and tool/graph
checks and record the actual GNU integration/doctest cohort and MSVC outcomes.
Native Windows behavior, exact cutoff controls, per-file metrics, multiple
ordered chained exchanges and live catalogue/key/policy/clock-frame issuers
remain separate. The eventual owner must route actual withdrawal to the same
authority. The loopback interface itself does not issue those production facts.

The first frozen run passed isolation and formatting, then stopped on five
GNU Clippy diagnostics before executing any cases. Its service took 20.634
seconds. Independent review also found that an empty native read could report
pending after consuming the final allowance or crossing the cutoff. Settle
that read immediately. Preserve a valid answer on the final permitted call.
The corrective candidate includes the corresponding actual loopback control
and the compiler corrections. Preserve both candidates and use the remainder
of the existing development window for one isolated rerun.

That rerun passed the library compile and exposed two remaining Clippy
diagnostics in the test peer before any case ran. It took 20.765 seconds.
Apply only the peer's binding-name and fixed-width chunk lint corrections and
use the same remaining window for the final selected readback. Across all
invocations preserve the original 220-second ceiling and stop on any further
refusal.

The final 793-input candidate
`c82844a1ca9af54c2fe798cb9f6ac146c84bcd9b71f8a9c8affccf0cbf2e2491`
passed all nine selected lanes. GNU executed 14 Engine integration cases,
including all four new loopback cases, 93 Engine doctests, 12 wire integration
cases and four wire doctests. The new staged-receipt doctest refused a second
publication through the moved value. GNU and MSVC Clippy passed, along with
the combined KE and Roughtime MSVC check. Readback matched every bound file in
the working tree and verified graph, compiler, vendor and tool bookends.
The service took 51.526 seconds. All three services and their verifier scratch
retired, using 92.925 seconds total.

The controlled endpoint reuses the laboratory ceiling of 30 seconds and
64 native read attempts, including empty reads. The final read control
confirmed an empty socket returns exhaustion immediately while a selected
valid datagram on that same read can still transfer. Withdrawal delivery
can report lock contention and therefore requires explicit owner settlement.
The exact-key, catalogue, round, policy and clock-frame change issuers and
multiple ordered exchanges with independent support remain open. Advance the
round consumer from this single receipt. Keep the separate native and
per-file qualification gates, and the no-public-peer scope of these results.

| Evidence under `~/.local/share/nevrotick/roughtime-original-exchange.4yihopfy/` | SHA-256 |
| --- | --- |
| `attempt.json` | `4e5e17bd21ac8646947d8419226e786e37ccc4a9f727f9f27aff367ba44542b1` |
| `repair.json` | `20fb2f8ddd6b54336925cb6a1e8fe3de428aa13cd577ceb80d5b7d5eb8b697ff` |
| `final.json` | `5ac3db75356ceb9fa106dfca3667dd1756dbcbc0282c3a260069a8471028ae1c` |

### 3.5 TA-W04 ordered controlled round

This two-pass control predates revision 1.9. Its six receipts retain their
original regression scope. The selected V1 measurement requires three
traversals and at least nine accepted receipts under
[NV-TA-001 §8.4.2](../docs/plans/time-assurance-integration.md#842-acquisition-retries-and-the-complete-measurement).

CHECK-6/7/8/9/14/21/25/27. One fixed controlled roster contains three
distinct loopback endpoints, long-term keys, declared operators and declared
clock groups. Its original control can start the round once. Visit the same
order twice. Submit and consume at most one native probe at a time under the
original overall cutoff. The successor nonce hashes the complete prior
response frame followed by 32 fresh random bytes. Retain the actual complete
requests, responses and chain inputs. Check every earlier lower signed bound
against the current upper bound, including equality, and retain any
contradictory pair as an absorbing terminal result. Use the original
send-before to receive-after elapsed time for the declared per-peer limit.
The loopback roster declares its groups and ordering. Live catalogue
provenance, random selection from eligible candidates, interval transport,
usable independent support, and cross-round recovery clearance remain with
their existing owners. [RFC 10049 §8.2](https://www.rfc-editor.org/rfc/rfc10049.html#section-8.2)

The independent loopback fixtures sign the actual observed bytes using
three distinct test roots and delegated keys. Require all six sends and
receipts in order with complete-frame chaining across the pass transition,
an accepted causal equality, a nonadjacent signed conflict retained after
Stop, duplicate operator and clock-group refusal, and no successor after
withdrawal. Exercise the previous one-shot and wire controls in the same
frozen candidate. Use the existing isolated verifier and one 220-second
development allowance including one causal repair and retirement. Keep all
per-command limits. Native Windows and per-file metrics remain separate.

The first frozen candidate passed isolation and formatting, then Clippy
refused the six-receipt inline array under the existing stack-size limit.
No test executed in those 22.068 seconds. Reserve storage fallibly for all
six receipts before the first send, with no append allocation during the
round. Add actual signed late-reply and expired-successor controls while
preparing the one corrective candidate. Preserve the original failure and
use the remaining development allocation with unchanged per-command limits.

The bounded-storage revision compiled and reached one Clippy diagnostic in
the test peer before execution. Its 22.889 seconds bring the two launches to
44.957 seconds. Compare the exact progress enum in that test and use the
remaining original window for the selected readback, with no further
behavioral repair in this checkpoint.

The final 794-input candidate
`6d84f6af270dd83454abf0037fa7349397c4bf7fe13824785c5949c5feae52ba`
passed all nine selected lanes. GNU executed 19 Engine integration cases,
including all five ordered-round cases, 93 Engine doctests, 12 wire
integration cases and four wire doctests. GNU and MSVC Clippy and the combined
KE and Roughtime MSVC compiler check passed. The source, vendor, compiler and
graph matched at both ends. The final service took 63.937 seconds. All three
services and scratch retired in 108.894 seconds total.

The controlled run preserved the two-pass order and verified every successor
nonce from actual retained frames and chain randomness. The nonadjacent
conflict test preserved both signed witnesses after Stop. A late signed reply
retained its receipt without granting a successor. The expired-round case
confirmed the next peer received no request. Round withdrawal is serialized
with local preparation, polling and publication. Contended delivery still
requires explicit settlement by the caller. The round supplies no current
date or TLS verifier authority. Exact live source admissions, common-instant
interval transport, independent agreement and cross-round conflict handling
are still open.

| Evidence under `~/.local/share/nevrotick/roughtime-ordered-round.1BNvx5ck/` | SHA-256 |
| --- | --- |
| `attempt.json` | `b24e4985b0f5c60896a5f9cbcd4df85d02b393a6db264063412d2b8d99e55b88` |
| `repair.json` | `0713e12d170b4a4a417b77261758a19bd3454316bf74782e799f23209d08009c` |
| `final.json` | `9aafb237e3419d47bc1c7d626ba527755d50efe8589898720d7135d86e676cd5` |

### 3.6 Portable Roughtime evidence admission and NTS handoff

Revision 1.8 records the implementation boundaries below.
[Development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
and §§3.1–3.5 record the controlled implementation and current qualification
state.
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

### 3.7 TA-W04 controlled interval admission

CHECK-7/8/9/14/21/25/27. Evaluate only the original complete measured
round. The receiver retains signed conflict before checking current
withdrawal. Under the original control guard, use a single monotonic
evaluation sample before the original exclusive cutoff. Transport each
signed Unix interval over the outer send-before to receive-after capture
span with explicitly declared positive rate endpoints and UTC projection
margin. Refuse reversed captures, stale original age, excessive individual
width and coordinates outside the wire date envelope.

Keep scaled endpoints exact through both per-group and cross-group
intersection. Require positive-width overlap and count each of the three
declared groups once across the two passes. Round the returned Unix
nanosecond endpoints outward only for presentation. Retain the six exact
original observations, group declarations, evaluated policy and original
cutoff. This conditional result precedes admitted live catalogue, clock
frame, UTC scale, date/era and full NTS ownership. It grants no clock effect.

The numerical oracle uses independent integer literals for capture order,
age, width, fractional rounding, exact endpoint contact and wire-epoch
overflow. The receiving cases use the actual six-exchange sequence to test
one compatible interval with original receipt custody, independent width
refusal before narrowing, within-group and cross-group disagreement, and
incomplete, stale and clock-frame-withdrawn evidence. Retain the earlier
signed-conflict witness at the evaluation method after Stop. Execute the
existing isolated Roughtime cohort and six exact arithmetic cases in one
240-second window including a causal repair and retirement. The current
native, production and per-file qualification gates remain.

The first frozen candidate passed its selected executions in 66.201 seconds.
Independent review found that the pinned Windows Rust implementation can
return zero for a slightly reversed checked `Instant` subtraction. Add an
explicit receive-to-evaluation ordering check before subtraction, the same
direct ordering across successive exchanges, and an adjacent reversal with
an equality control. Also verify a nonzero projection margin with exact
known endpoints. Re-admit this postimage within the remaining original
allowance. The original execution retains its own source binding.

The final 796-input candidate
`49f23ca6156b6dbb40b9ea1ab56c35bac0331106f65a3127d60739106ff7da11`
passed all ten lanes. GNU executed 23 Engine integration cases, six exact
interval arithmetic cases, 93 Engine doctests, 12 wire integration cases and
four wire doctests. GNU and MSVC Clippy passed, along with the combined KE
and Roughtime MSVC compiler check. Both tested input maps were re-admitted
from `2583f8b`. The final map matches the working tree. Both services and
scratch retired after 131.349 seconds total.

The controlled consumer checks age at its one sampled evaluation instant.
Its result carries that instant and the original cutoff and receipts.
A later use requires a fresh applicable guard and cannot renew the original
age. Live catalogue and frame admission, supported date/era and scale checks,
NTS connection and receipt ancestry, native Windows execution, and per-file
qualification remain open.

| Evidence under `~/.local/share/nevrotick/roughtime-common-interval.3RqcU49D/` | SHA-256 |
| --- | --- |
| `attempt.json` | `03527528669f579a02456cff285305f74b57bd2237791ea74e29daad4b830a82` |
| `final.json` | `97c1f84653faeff9320fd04faaea2d773c2cc2931d844ebc48bdcba0fea58d0d` |

### 3.8 TA-W04 controlled whole-date interpretation

For TA-C07/09 and CHECK-7/9/25, pass the completed controlled agreement
through the existing `time-domain` date-scope and leap-interpretation owners.
Convert its exact open Unix endpoints outward to an inclusive NTP-epoch Q32
window before checking containment. Preserve the original agreement, explicit
scope and complete calendar answer with its dataset identity. Keep the original
round's current guard and cutoff across the transformation. The numerical result
is conditional on the controlled source declarations and the supplied calendar.
Leave unknown or changing leap information visible to the next consumer.

Use independent Q32 fixtures for the era boundary, fractional rounding and
numeric envelope, then real signed loopback receipts for accepted full scope,
whole-interval rejection with an in-scope center, and current withdrawal.
Use the existing isolated Engine Roughtime recipe, exact graph and cohort with
the added selectors. One 240-second execution and repair allowance covers the
first attempt and any causal repair. Native verifier policy, live issuers,
cross-round conflict, TLS ancestry and per-file qualification retain their
existing prerequisites.

The 796-input candidate
`fd117e5f9e8912fc1028e699d3fae32d89d851440462ffd630807de2173d6bff`
passed all ten lanes in 71.969 seconds. GNU ran 26 signed-verification and
Engine integration cases, nine interval and date arithmetic cases, 94 Engine
doctests, 12 wire integration cases and four wire doctests. GNU and MSVC Clippy
and the combined KE/Roughtime MSVC compiler check passed. The additional
signed-loopback cases refuse either out-of-scope endpoint, preserve all six
receipts and calendar identity, distinguish known and unknown leap knowledge,
retain the 2017 transition, and refuse a withdrawn clock frame. A fresh source
review confirmed the unchanged post-transform cutoff guard and independently
derived the era rollover and last representable Q32 tick. The candidate map
matched the live inputs, tool and vendor bookends passed, and the service and
private build scratch retired.

`evaluate_date` has a controlled integration caller. Production catalogue,
key and frame admission must still establish actual scale, current authority
and allowed scope before NTS use. The currently selected Windows verifier takes
one date, so a later NTS handoff still needs whole-interval certificate validity
and original connection, handshake and protected-receipt ancestry. This
checkpoint supplies no native trust, coverage or mutation credit.

| Evidence under `~/.local/share/nevrotick/roughtime-whole-date.frzkb36o/` | SHA-256 |
| --- | --- |
| `check.py` | `c47b0d7edc2cd29cc45dcd3132cbfbbb43eb706777c5b041545a514dc5e1405e` |
| `attempt.json` | `af025289bbb66406b656230d313b8a65c8cfa596d9026150221cd1fab363b550` |
| `attempt.log` | `75f7b2307cc8a96dce0e626c8228274d6b5952f3b1e47703acbc3b8b80428b19` |

### 3.9 TA-W04 retain signed conflict across controlled rounds

TA-C04/13/14/21 and CHECK-7/9/14/21/25/27. One controlled recovery
invocation owns the ordered-round consumer and can spend an initial round and
one caller-selected follow-up under the same original exclusive deadline.
Keep the first authenticated causal contradiction synchronously at receipt
settlement. Move the original bounded transcript, with intervening frames and
chain inputs, and retain the original roster and limits without extending
their authority. Timeout, Stop, withdrawal or a later complete agreement
cannot replace that first contradiction or produce a recovery date while it
stands. The low-level round remains available for its controlled diagnostics.
The new invocation selects no production retry, clearance or fallback policy.

Exercise real signed loopback receipt settlement and subsequent no-reply,
distinct conflict and otherwise-healthy date cases. Check complete byte custody,
original roster, bounded follow-up work and a clean date control. Use the
existing isolated Roughtime cohort plus the new selectors under a shared
240-second development and repair allowance. TA-W02 conflict-clearance policy,
live issuers, NTS ancestry, per-file and native gates remain open.

The 797-input candidate
`caa8501ff3226b1e05174d333e8a5866d01f48050f67a4cd8752161eb6dca94f`
passed all ten lanes in 75.480 seconds. GNU ran 29 Engine Roughtime integration
cases, nine interval/date arithmetic cases, 95 Engine doctests, 12 wire
integration cases and four wire doctests. GNU and MSVC Clippy and the combined
KE/Roughtime MSVC compile check passed. The compiling healthy consumers and
the overlapping-borrow compile-fail case reached the actual round admission.
The three new loopback cases observed the submitted follow-up expire at the
original invocation limit, checked the first conflict's unchanged frames and
roster after a distinct later contradiction, and compared a refused complete
follow-up date with the healthy clean invocation. The independent source review
traced original receipt publication into the synchronous historical transfer
and the same-slot date check. Source, vendor, tool and graph bookends passed.

The retained cutoff is the effective deadline of the conflicting round.
The historical witness remains available until the invocation settles, even
after that deadline. Fresh library
constructors remain available for independent diagnostic runs. A live recovery
owner must still bind catalogue, key, policy, clock-frame and scale issuers and
implement a qualified conflict-resolution policy. The selected cohort supplies
no native, per-file or production NTS evidence.

| Evidence under `~/.local/share/nevrotick/roughtime-conflict-recovery.lc7oxfz7/` | SHA-256 |
| --- | --- |
| `check.py` | `253a5dc79741f7345a70ce0bd7ae492e928941b1c140d155aa1e24c4e6dd4e01` |
| `attempt.json` | `786f96afbdf02a86bef7fac03bd263ff466b22f0894b6d49ea6b6213a5e1404f` |
| `attempt.log` | `1c1a7cd237bc1586a140e9cead04ae1d45a6dd0b62326b76ea72d1e3ab5ebdc6` |

### 3.10 TA-W04 guard the controlled recovery invocation

TA-C04/05/13 and CHECK-9/11/14/25/27. Give the controlled scheduler one
exact, one-use recovery authority above its two round controls. An acknowledged
catalogue, key, policy or clock-frame change, or Stop, retires remaining sends,
receipt publication and date evaluation in the original invocation. Hold the
invocation guard before the round guard at admission and each effect, and
reacquire both before authenticated receipt publication. Retain historical
signed conflict independently of current permission. Fix the interval policy,
date scope and calendar for the invocation, so later evaluation cannot swap
inputs for the same evidence.

Use the prior signed loopback cohort and two focused controls at the real
round consumer. Observe withdrawal after transmission with the signed reply
ready, and clock-frame retirement after one useful date. Check successor
refusal with an unused round control, single issuance and healthy neighbors.
Run the isolated recipe with one 240-second shared execution and repair
allowance. Installed catalogue approval, supplier/key rotation, Windows
frame issuance, scale, per-file metrics and native NTS qualification keep
their existing owners.

The 797-input candidate
`927c363e4c8389cb08ed3f5ea69509fc20d70f7c4037a05a34e636728f8be787`
passed all ten lanes in 80.682 seconds. GNU ran 31 Engine Roughtime
integration cases, nine interval/date arithmetic cases, 95 Engine doctests,
12 wire integration cases and four wire doctests. GNU and MSVC Clippy and
the combined KE/Roughtime MSVC compile check passed. The new loopback
controls observed queued-reply and successor refusal after acknowledged Stop,
date re-evaluation refusal after clock-frame retirement, and single issuance
of the original recovery control. Each retained its healthy neighbor.
Source, vendor, tool and graph bookends passed.

Independent review traced the parent-to-round guard order and both poll guard
acquisitions, including staged-result retirement after a refused reacquisition.
Delivery between the read and publication guards remains source-reviewed at
the new parent layer. The queued-response test withdraws before `poll`.
The controlled authority consumes a scheduler-delivered change. Production
issuers must still detect and deliver changes from the installed catalogue,
root-key and current-policy owners and the qualified Windows clock frame.
The controlled date remains historical and needs current admission at the
next consumer.

| Evidence under `~/.local/share/nevrotick/roughtime-current-recovery.z4swsjxt/` | SHA-256 |
| --- | --- |
| `check.py` | `526053b9bf4c8f70e8f549804af84f4346f87575f37cc497bb05bb7c569e3083` |
| `attempt.json` | `cb0736eb9015d61ac9cde94b32aee2c08da90b93a67caf9c5f45d9a5c8b07809` |
| `attempt.log` | `e6c5393dd6e115cbca91b28d00b46e70c790f6ce41b254318bb33e00ed7d341b` |

### 3.11 TA-W06 controlled TCP counterpart

TA-C17 and CHECK-6/9/21/25/27. Exercise the RFC 10049 v1 stream transport
over one fresh, exclusively transferred loopback connection. Match its actual
peer before request preparation. Each poll performs at most one nonblocking
read or write. Keep the original cutoff, one request and one response, and
bound successful calls by the wire sizes and stalled calls by the shared
allowance. Read the fixed prefix before bounding the declared body. Treat EOF
before completion as terminal. Retire the connection before verifying and
preserve the exact transmitted frames, TCP provenance and capture brackets
through shared request verification and guarded publication.

Exercise the complete signed fragmented response, header-only oversize refusal,
mid-body EOF and withdrawal of a staged response with actual loopback sockets.
Include the prior UDP, recovery, arithmetic and wire tests in the frozen recipe.
One 240-second shared development allowance includes causal repair and
retirement. Connection establishment belongs to the controlled fixture, which
supplies a fresh stream with default disabled linger and no retained clone.
Provider admission, native client transport, scheduled UDP-to-TCP fallback and
RFC backoff, interoperable peers and per-file qualification remain open.
[RFC 10049 §5](https://www.rfc-editor.org/rfc/rfc10049.html#section-5)

The first frozen run stopped at the wire helper's missing Clippy Errors
section after 19.364 seconds. No behavior ran. Independent review also found
that a failed authority-lock acquisition could leave the owned connection
open. The repair retires that connection without changing the unavailable
authority and adds a loopback control that holds the real lock. The first
repair stopped at three compiler-lint findings for documentation, an elidable
lifetime and a request metadata argument passed by value. No behavior ran in
that attempt. Both failures remain attached to their original candidates.

The final 798-input candidate
`91a05cf1c73320194474dc60268c218d41644f20f9119bd2571bb26c36699500`
passed all ten lanes in 73.556 seconds. GNU ran 34 Engine integration cases,
nine numerical cases and the TCP guard-contention unit control, 95 Engine
doctests, 12 wire integration cases and four wire doctests. GNU and MSVC
Clippy and the combined KE/Roughtime MSVC compile check passed. The TCP
loopback tests observed signed fragmented progress, exact request and response
custody, header-only oversize refusal, incomplete EOF, withdrawal before
publication, and peer-side stream retirement. The prior 22.634-second repair
and first run remain part of the original 240-second allowance. Tool, source,
vendor and graph bookends passed. These receipts apply to the preconnected
TCP exchange and prior two-pass controller. The three-traversal scheduler,
authenticated backoff, native connect and actual provider interoperability
retain their separate gates.

| Evidence under `~/.local/share/nevrotick/roughtime-controlled-tcp.enncajpy/` | SHA-256 |
| --- | --- |
| `attempt.json` | `b9a79d7e3273621863c9fd2f25c5d57c1eea46f6ceead06b17054cd7f5e7c23e` |
| `repair.json` | `5d0b4250b6894121546169c23b69696d6d5e91404246c117b3fa93691cb0476f` |
| `final.json` | `94b2634827e4c387723a104154b2c2a20859c46aeebd5cc83f4844c7e73c3d48` |

### 3.12 RFC-M02 controlled three-traversal measurement

TA-C01/06/08/17/19 and CHECK-7/8/9/21/25/27. Apply revision 1.9's
three-traversal interpretation to one fixed controlled roster of three
operators. Run `S || S || S` under the original deadline, with one live
socket and nine original send opportunities. Reserve all nine receipt slots
before a send. At 64 read calls per opportunity the per-round maximum is 576.
The invocation retains its two-round ceiling and shared deadline, with room
for one historical conflict transcript and one active or settled follow-up.

Advance the chain only after accepting the original signed response. Each
successor uses the entire predecessor frame and fresh OS bytes. Refuse a
repeated linking value in the original chain before sending, including a
nonadjacent repeat. Use all three visits in each group's interval and keep
three declared groups. Six or eight receipts cannot supply a complete date.
Continue comparing earlier signed bounds through the third traversal.

Use actual signed loopback neighbors for complete order, both chain boundaries,
incomplete evaluation and the decisive third traversal. Exercise the duplicate
randomness check with a private literal control and independent hash answer.
Run the isolated existing Roughtime recipe plus the exact changed selectors
within one 240-second execution and repair allowance. Random selection from an
admitted catalogue, replacement and retransmission scheduling, per-binding
authenticated backoff, live generations, and native and per-file qualification
keep their existing owners. The historical two-pass receipts retain their
original source and acceptance scope.

The 798-input candidate
`27811291e80592306255329440bc9371da1ba3146a4b03f6b8ff792dfeb9570d`
passed all ten lanes in 69.164 seconds. GNU ran 35 Engine integration tests,
11 selected unit tests, 95 Engine doctests, 12 wire integration tests and four
wire doctests. GNU and MSVC Clippy and combined KE/Roughtime MSVC checking
passed. The independent loopback order observed all nine real exchanges and
eight distinct links, including both traversal boundaries, refused interval
and date evaluation at six and eight receipts, and observed no tenth send.
Independent cases retained a signed contradiction against the first receipt
at position seven and required a third-visit interval refusal despite six
compatible earlier receipts. The private nonce control rejected an earlier
nonadjacent linking value and accepted the fresh known-answer hash. Source,
tool, vendor and dependency graph bookends passed.

RFC-W01's [controlled UDP binding](#313-rfc-w01-controlled-udp-retry-binding)
is the first scheduling consumer. The production binding owner has to outlive
individual round and invocation handles, charge the actual send or connect,
and reset only on admitted bound response under current authority.
The controlled TCP exchange starts after connection establishment. Production
catalogue admission, scheduled fallback, a durable retry/reset lifecycle and
coverage, mutation and native behavior retain their existing prerequisites.

| Evidence under `~/.local/share/nevrotick/roughtime-three-traversals.2tiiqjml/` | SHA-256 |
| --- | --- |
| `check.py` | `450410927908cc8fedabea9bc1c3edd4e1749525e48e63b2ea98b1f4ce75f81e` |
| `attempt.json` | `20146be6a7021cd2317bac49c6c1d972e4512117195695cc929ecccd09b958c2` |
| `attempt.log` | `ef3c6a4f2ff3479b615a7fa06f7a1ac7c5c90a4ec8955aea7a1260ac65cb43ee` |

### 3.13 RFC-W01 controlled UDP retry binding

TA-C06/14/17/19 and CHECK-6/9/21/25/27. Bind one explicit controlled
logical peer and exact root to two checked loopback aliases. The scheduler
owns the binding outside each one-shot authority. One exclusive proposal
can be charged at the actual UDP send boundary under the original cutoff.
Retain its lease through socket retirement and staged receipt custody.
Refusing a floor before send charges no attempt. A submitted failure,
withdrawal, expiry or discard of the original opportunity advances the
binding's floor from terminal observation. Refuse unrepresentable or
reversed clock state. Use the exact rational 3/2 profile rounded upward at
nanosecond granularity and capped at 24 hours. Apply zero additional jitter.

The single-exchange path resets only when the actual original signed response
publishes under its current authority. Here the exchange itself owns the final
admission. A future measurement consumer must also exclude a discarded chain
before publishing a reset. Qualify actual alias send refusal and a separate
healthy binding, malformed/noncurrent nonreset, and a current signed reset.
Use the existing isolated recipe under one 240-second execution and repair
allowance. TCP connection scheduling, round-owned deferred reset, live
catalogue issuance, storage and restart continuity, and per-file/native
qualification retain their existing prerequisites.

The first frozen candidate reached GNU Clippy and stopped after 19.450
seconds because the floor check used two identical success arms. No
behavioral case ran. Combine the arms without changing the early-floor
comparison and reuse the remaining original allowance for the repaired
candidate. The charged state is explicit before the native call, so losing
the lease without retirement leaves that binding blocked.

The final 799-input candidate
`c185edfe467bcce46aea4ae0937a84e1d310b58d1df9259a13515ec6eafb1756`
passed all ten lanes in 71.922 seconds. GNU ran 36 Engine integration tests,
13 selected units, 96 Engine doctests, 12 wire integration tests and four
wire doctests. GNU and MSVC Clippy and the combined KE/Roughtime MSVC
check passed. The loopback control observed no early alias send under the
one- and 1.5-second floors, verified an independent binding submitted work,
and required the next failure to return to the one-second floor after
publishing the current signed receipt. A verified but withdrawn receipt
advanced the original failure floor. The compiler refused overlapping
binding borrows. Unit controls checked growth through the prescribed initial
values, upward rounding and cap, and the immediate neighbors of the due
instant. Source review covered socket-before-lease retirement, matching root,
exact publication guard and the active-state latch. Source, tool, vendor and
dependency graph bookends passed.

The loopback caller explicitly retains the exact controlled binding. The next
[TCP-connect slice](#314-rfc-w01-controlled-tcp-connect-and-shared-retry-floor)
uses it at the native connection boundary. A production owner still has to
issue and retain the binding across round, transport and process lifetimes
and apply the reset rule at complete chain admission under RFC-W01 and TA-W02/06/08.

| Evidence under `~/.local/share/nevrotick/roughtime-udp-retry.g_6e1ew_/` | SHA-256 |
| --- | --- |
| `check.py` | `4b1ae3c9e673104eff9996de4835d3d9874c8b1adb8d975f7bdf5446f18ca449` |
| `attempt.json` | `911609d83a6834371388b85bbf8ec56e8c1192ad07db1f7fd12156a05757a91d` |
| `final.json` | `b8ed7fb4b05c2a1ed0227719131215d75956ef6beb5cba1dbb3931c225490c10` |
| `final.log` | `68f85237a75d013f334f026f89b3f6faa25cf81c0a44ef1a4f704f695a73cd75` |

### 3.14 RFC-W01 controlled TCP connect and shared retry floor

TA-C06/14/17/19 and CHECK-6/9/21/25/27. Use the same bounded peer/root
binding and original deadline at an actual loopback TCP connect. Check the
selected alias and charge at native entry while holding original authority.
Pass only the remaining time to the synchronous connector. This entry is
for a blocking controlled caller. An undelivered concurrent withdrawal still
requires settlement by its caller. Carry the exclusive retry lease through
connector return, stream retirement and receipt publication. Connection
success alone leaves the lease charged. Current signed publication resets
the single-exchange floor. A failed connect or discarded result advances it.

Observe a real listener and datagram peer before and after the shared floor,
including failure on one transport, early refusal on the other, and a current
signed publication. Exercise the supplied-stream and UDP cohorts unchanged.
Run the existing isolated recipe within one 240-second shared execution and
repair allowance. Automatic UDP-to-TCP fallback, the native blocking-worker
handoff, complete-chain reset, catalogue and restart continuity, and native
and per-file qualification retain their existing gates.

The 799-input candidate
`dc680df32e9548096a25664b26c80c52f7bff87a37b54a15d939622d9d3d5569`
passed all ten lanes in 76.522 seconds. GNU ran 38 Engine integration tests,
13 selected units, 96 Engine doctests, 12 wire integration tests and four
wire doctests. GNU and MSVC Clippy and the combined KE/Roughtime MSVC
check passed. The loopback controls observed the shared retry floor in both
directions, no listener accept after acknowledged pre-connect withdrawal,
native connection refusal advancing the UDP alias floor, a signed but
withdrawn TCP result retaining the failure sequence, and a current signed TCP
result resetting the next UDP failure to one second. Source, tool, vendor
and dependency graph bookends passed.

The Rust contract review traced the selected alias and lease through native
connect and current receipt publication. The lifecycle review checked socket
retirement before lease release, including the standard library's connection
error path and the checked deadline after setup. Regression selection retained
the earlier UDP, supplied-stream, round and wire consumers. In-flight
withdrawal ordering is source-reviewed at the controlled connector. The Windows
worker handoff and runtime remain open. Its native owner must observe bounded
socket/handle retirement on connect refusal, a late result and Stop before
fixture teardown, using the existing Windows resource procedure.

| Evidence under `~/.local/share/nevrotick/roughtime-cross-transport-retry.rpr4elks/` | SHA-256 |
| --- | --- |
| `check.py` | `dea0443a9d2ffc87d4d699c7ce413d418463b1e0262b841926bfe5f9703471d7` |
| `attempt.json` | `3993487c37417f99bd26263c26b1bd85f63c102729ac126dbca2d85d52ab5f6c` |
| `attempt.log` | `9c036c89217b23fdc79016a83bc9a5a7a6bacf360f0830fe6ef54f090a454393` |

### 3.15 RFC-W01 controlled complete-chain retry reset

TA-C06/07/14/19/21 and CHECK-6/9/21/25/27. Give one controlled ordered
round exclusive access to its three preselected retry bindings. Charge each
original UDP send at native entry. The accepted signed leg returns its state
to the same slot with prior debt intact, so repeat visits may proceed under
the original floor. A failed or rejected charged attempt retires with debt.
Use the exact nine receipts, original guard, whole date scope and explicit
calendar for a separate final admission. Only one successful final admission
clears the three retained states. An abandoned prefix, conflict, withdrawal
or rejected date leaves prior debt in force. Existing single-exchange callers
retain their final-response reset contract.

Exercise real seeded retry debt, signed provisional replies, discarded
measurements and one admitted date, then observe the next floor. Use the
isolated existing recipe within one 240-second shared execution and repair
allowance. Live issuer identity and persistence, fallback and recovery
scheduler integration, and per-file/native qualification remain open.

The 799-input candidate
`51254c54ba3641cf589d0e095e94754ff7203679a13862acadc52df964e01a9c`
passed all ten lanes in 87.737 seconds. GNU ran 39 Engine integration tests,
13 selected units, 96 Engine doctests, 12 wire integration tests and four
wire doctests. GNU and MSVC Clippy and the combined KE/Roughtime MSVC
check passed. The real loopback cohort seeded each binding with one failure,
ran six and nine signed legs, and observed every selected alias again. A
discarded prefix and a date-scope refusal preserved the 1.5-second wait on
the next failure. The scope-refused round also refused a second admission
with wider bounds. The admitted nine-receipt date restored the next wait to
one second for all three bindings. Source, tool, vendor and dependency graph
bookends passed.

The Rust contract review traced each original root and its borrowed state
through the native send, tentative signed result and one-use date admission.
The lifecycle review accounted for socket retirement before lease settlement
and the final time and authority checks before any reset. Retry debt after an
in-flight conflict or withdrawal and moves of the round are source-reviewed.
Regression selection retained the earlier one-exchange UDP/TCP, invocation,
date, chain and wire cases. The [bound recovery child](#316-rfc-w01-bound-recovery-and-retained-conflict)
carries the same states into a controlled follow-up. Production route and
restart continuity remain with their existing owners.

| Evidence under `~/.local/share/nevrotick/roughtime-chain-retry.8jsg0ttk/` | SHA-256 |
| --- | --- |
| `check.py` | `12367e296daaa37945ef6a6601fa2115c1056f5ae5e4dd64b1a0499f02bdc056` |
| `attempt.json` | `2c131d1f2d08e36d869baa556e32a92575a6ca50b79d5555b9fb4948cc4984e4` |
| `attempt.log` | `c8f46513cfdcb2951b8f3ce7ab6840985255bfdf19e822750c2caecbb02d69e8` |

### 3.16 RFC-W01 bound recovery and retained conflict

TA-C06/13/14/21 and CHECK-6/9/14/21/25/27. Bind one controlled recovery
invocation to the exact selected retry peers and fixed date policy. Reborrow
those same states for the initial and optional follow-up round. Both rounds
share the original invocation cutoff and charged slot limit. Keep the
original round and recovery guards through sends, publication and date
admission. A caller cannot choose a different unbound roster for a bound
invocation. Retain the original signed conflict independently of the bound
retry states. Refuse final date admission before any reset when that conflict
still stands, including after a complete, otherwise healthy follow-up.

Exercise actual initial debt, signed contradiction and follow-up with the
same three controlled root/alias bindings. Compare their next floors with a
clean admitted-date invocation. Reuse the existing isolated recipe within
one 240-second shared execution and repair allowance. Production scheduling,
automatic fallback, conflict clearance, live issuers, restart continuity
and native and per-file qualification remain open.

The 799-input candidate
`f16b09d158ae17dedeff6eff54397418a0554a0a10a9b679df792871c3c5f401`
passed all ten lanes in 87.210 seconds. GNU ran 40 Engine integration tests,
13 selected units, 96 Engine doctests, 12 wire integration tests and four
wire doctests. GNU and MSVC Clippy and the combined KE/Roughtime MSVC
check passed. The paired loopback control completed all nine signed replies
in both follow-ups. A clean date reset each binding to a one-second wait on
its next failure. A retained contradiction barred date admission and left
those waits at 1.5, 1.5 and 2.25 seconds. The contradicting response had
charged the third binding once more. The test also verified that a raw
roster refusal did not spend the original control, and that a successful
date admission could not repeat. Source, tool, vendor and dependency graph
bookends passed.

Contract review traced the same selected root, endpoint and retry borrow
through both rounds. Lifecycle review checked the shared cutoff and guard
order, expiry and withdrawal, and transfer of the first signed conflict
before the date consumer. The new test directly exercises binding continuity,
debt and conflict priority. The earlier withdrawal, deadline and transcript
controls remain in the selected regression cohort. The bound connector is
still selected by its controlled caller. Transport fallback and its restart
or outstanding-position handling need their own admitted scheduling path.
Use a distinct unanswered-UDP disposition from the original send and receive
owner. The generic `Expired` outcome also covers unpolled and already-consumed
responses.

A canceled alias formatter fetch left a partial toolchain during test
preparation. Its owned alias was retired after checking for consumers.
The frozen verifier used the exact pinned 1.100.0 toolchain.

| Evidence under `~/.local/share/nevrotick/roughtime-bound-recovery.htnrwv64/` | SHA-256 |
| --- | --- |
| `check.py` | `859b98e884945b07cb69772c829dc668f38430812f18d960aa0f57f81b9c93d0` |
| `attempt.json` | `ea5ce26eaccc2a659a5cca73fb0d45327056bc6fecaf6433e449da7cc044a848` |
| `attempt.log` | `20e4875a8618e9f22f71b472bd54329bdf332c82377ac95ecd58aa197a17eef3` |

### 3.17 RFC-W01 controlled UDP-to-TCP fallback and fresh chain

TA-C06/13/14/21 and CHECK-6/9/14/21/25/27. The controlled caller selects a
finite UDP response wait and optionally binds an advertised TCP alias to each
original root and retry state. After a successful UDP send, an empty native
read started at or after that wait and completed before the round cutoff yields
`Unanswered`. The round first rechecks its original recovery and round
authority, retires the attempt and preserves its signed prefix as history.
Malformed, foreign and late packets, a receive-budget refusal and generic
expiry cannot issue that disposition.

The second slot reuses the same invocation deadline, selected roster and
retry states. The unanswered peer needs an advertised TCP route and its
original retry floor must be due. If the floor meets or exceeds the effective
cutoff, admission refuses. An early request can return the existing floor
without spending a round slot or opening a connection. A fresh nine-leg
round starts with an independent nonce and uses TCP on the unanswered peer's
three visits. The other peers retain UDP. Subsequent requests chain the exact
preceding response across transport changes. Only a complete, current date
admission can clear the shared debt. Repeated visits still count toward the
original three supporting groups.

The Rust contract and protocol-input reviews followed send completion, the
native read and retry lease through the second round and date consumer.
An independent lifecycle review identified that anchoring the response wait
before `send_to` could spend it before the request was transmitted. Anchoring
it at successful completion closed that path. The reviewed route remains
controlled loopback, with a synchronous bounded connect and no live scheduler
or preemptive native-worker Stop guarantee.

The bounded variant review traced `response_wait` and `response_cutoff` across
the Roughtime exchange owners. The UDP writer assigns the only response
sub-deadline. TCP connect consumes time from the original round cutoff.

Regression selection retains the existing isolated GNU and MSVC checks within
one 240-second execution and repair allowance. The healthy loopback control
checks the historical prefix, absence of early TCP, the short-cutoff refusal,
each fresh frame and transport, connection retirement and complete date
admission. The malformed UDP neighbor checks that invalid traffic cannot
authorize a TCP connection.

The 799-input candidate
`18799fa53ca028a8324342617592ffb77c1ebe927e425562c1058b3447adc7ba`
passed all ten lanes in 97.333 seconds. GNU ran 42 Engine integration tests,
13 selected units, 96 Engine doctests, 12 wire integration tests and four wire
doctests. GNU and MSVC Clippy, the combined KE/Roughtime MSVC check and source,
tool, vendor and graph bookends passed. Both fallback controls were present in
the executed roster. The healthy test observed all three TCP stream retirements
and read back the original UDP frames separately. The malformed response yielded
its original framing refusal, followed by `FallbackUnavailable` and an empty
TCP listener.

The first two attempts stopped at GNU Clippy before behavior ran. The first
reported fixture helper lifetime, length and documentation diagnostics. The
repair extracted the follow-up driver and left its TCP address out of scope.
The final attempt passed after that helper read the listener's actual address.
The three attempts consumed 144.034 seconds of the original 240-second budget.
Production scheduling, live issuers, restart continuity, native Windows and
per-file qualification retain their existing owners and prerequisites.

| Evidence under `~/.local/share/nevrotick/roughtime-fallback.xZYJ5pMU/` | SHA-256 |
| --- | --- |
| `check.py` | `f2865d1b3df935b233d7564ea7c04b249698a7aabb8b482bbdbf0db84f5ac347` |
| `attempt.json` | `3467c14a411142c542cb337c9300274cf8cc43dfa2b28d676e367ea26bec71fc` |
| `attempt.log` | `36657d1fc0888b610d30f30113d525995b2c092f38e02ba3378f717414c3853d` |
| `final.json` | `ebfa0d4064e497d9f9f43b2a7fe8d2410ee54863a9a35331674f4e814f493584` |
| `final.log` | `223c2cc97afaa7e9c6760c901792021660b4b4132355afd7ab2a605882d3a4f6` |
| `confirmed.json` | `567d3647bef5f19a027883ee94d73ded7f58496083ec3c163f318f9d6894bd02` |
| `confirmed.log` | `20a183352d7bba394c65ee81cbe7410a3872efb944743871ef51f4c02278bbaf` |

### 3.18 RFC-W01 schedule controlled fallback at the original retry floor

TA-C14/21 and CHECK-9/14/21/25/27. The existing Engine Tokio runtime may wait
for a fallback after the controlled UDP owner has recorded silence. The first
poll fixes one absolute follow-up cutoff, clamped to the invocation's original
deadline. A floor at or after that cutoff refuses immediately. While waiting,
the scheduler borrows the original invocation and releases its control lock.
It reserves neither a round slot nor a native socket.

The wait subscribes before checking the original control. The first recorded
withdrawal wakes it after the control lock is released. Timer or withdrawal
wake returns to the same checked round-admission path with the original
absolute cutoff. A late wake cannot renew that cutoff. A dropped pending wait
leaves the original debt and remaining slot available to their owner. This
checkpoint used the controlled blocking connector. The [scheduled worker](#319-ta-w06-scheduled-tcp-connect-with-shared-worker-custody)
extends that path with owned native custody.

The Rust contract and lifecycle controls follow the original silence, retry
floor and authority through suspension to the admitting consumer. Observe a
pending retry and an empty TCP listener, eligible follow-up admission, prompt
Stop while pending, and refusal on a timer observed after its cutoff. Reuse the
isolated GNU/MSVC recipe within one 240-second total execution and repair
allowance. The Engine and TA-W02/S3.3 mandatory-state owners must still bind
restart continuity to qualified boot and clock-domain evidence and an admitted
writer. The process-local retry schedule carries neither durable deadline nor
new clock authority.

For the transport handoff, retain the original retry lease and staged request
with the round. A bounded worker can receive the selected numeric route,
absolute cutoff and original dispatch authority. Account for its capacity
until actual join and settle the returned stream under current round and
invocation guards. Observe late result closure and independent Stop progress.
Thread custody can follow the existing resolver pattern within the Engine's
aggregate cap. Pinned Tokio 1.54.1 and Mio 1.2.4 source show that Windows socket
drop requests AFD cancellation and the reactor retains state until it drains
the completion. That route would also need an observable retirement boundary.

The 799-input candidate
`2452c287084bc66ee9018756ed02881eb23d557d87e86a6f4beb925d14b8f656`
passed all ten lanes in 101.748 seconds. GNU ran 45 Engine integration tests,
13 selected units, 96 Engine doctests, 12 wire integration tests and four wire
doctests. GNU and MSVC Clippy, the combined KE/Roughtime MSVC check and source,
tool, vendor and dependency graph bookends passed. The three scheduler controls
observed an empty TCP listener before explicit dispatch, nine signed follow-up
replies after the original floor, prompt Stop with unchanged retry debt, and
late-wake refusal with the slot still available. The independent source review
traced enrollment before the phase check, notification after lock release and
fresh guarded admission after either kind of wake. Direct withdrawal of the
child round is checked at admission. Invocation withdrawal also wakes the wait.
Drop of a pending waiter is source-reviewed. Native dispatch, platform
retirement, cross-restart admission and per-file qualification remain open.

| Evidence under `~/.local/share/nevrotick/roughtime-fallback-schedule.E6qyI2mN/` | SHA-256 |
| --- | --- |
| `check.py` | `48fd7cd1f6d59d5277dc352aa5492bef37ecafd685a40633d1c7a406cb523b7a` |
| `attempt.json` | `d4a98aa60af71145514a7d509f024e465a53128ed56ca237601968cc3785f14d` |
| `attempt.log` | `ab65f9e303ad47bc577447a83cd23d44b3440af6b2dfffd09a4e6c8e8407217e` |

### 3.19 TA-W06 scheduled TCP connect with shared worker custody

TA-C14/21 and CHECK-9/14/21/25/27. Scheduled fallback uses the shared
DNS/TCP lane and keeps one outstanding native job. The separate KE owner
retains its slot under the existing two-job aggregate cap. Admission either
reserves the current lane immediately or reports `ConnectBusy`. The worker
receives the exact numeric destination and absolute cutoff. It claims one
dispatch and resamples the remaining time at native entry. The round keeps
the prepared request and retry binding while the worker owns its stream.

The original invocation and round controls can revoke a reserved dispatch.
A connect that already won dispatch keeps its custody until settlement after
acknowledged Stop. Collection observes actual worker exit and joins before transferring a
stream. Original round, invocation, attempt and deadline checks cover
adoption. Abandonment closes the original result receiver before the lane can
reap its worker and submit a successor.
The exact binding refuses more work on either alias while its abandoned job
remains pending. A never-dispatched job preserves the prior floor. A charged
discard advances the next floor from observed settlement. A current joined
stream moves into the original request's retry lease and signed receipt path.

Regression selection covers the existing blocking fixture, scheduled real
loopback TCP, a connection held through Stop while the Engine progresses, an
independent contender during custody, EOF before fixture teardown and a valid
successor. The DNS custody case checks the same lane across result types. The
existing isolated recipe includes the default and selected MSVC feature paths
and one 240-second shared execution and repair allowance. Installed Windows
retirement behavior, live catalogue and restart admission and the per-file
gates keep their existing prerequisites.

The 801-input candidate
`f2434eec543ae25ba43b3cb0a5751232910cddb5de9305c545e246c48974e67d`
passed the twelve selected lanes. GNU executed 45 Roughtime Engine integration
tests, 14 selected units, the DNS/TCP custody pair in one two-thread libtest
process, 96 Engine doctests, 12 wire integration tests and four wire doctests.
The scheduled test dispatched all three TCP legs from the Engine runtime and
verified all nine signed replies. The Stop test held a real connected worker,
observed the shared lane and original retry state still occupied, then saw EOF
without request bytes and a healthy successor. The DNS test separately observed
TCP refusal while the retired resolver still occupied the lane. Default and
Roughtime MSVC Clippy and the combined KE/Roughtime compiler check passed.

Independent source review found an old relative timeout could survive a pause
before dispatch. The connector now resamples after the claim and refuses an
elapsed cutoff before calling the native adapter. The review also found that the
two unit fixtures could contend when run together. They now serialize fixture
ownership while the individual controls exercise actual production contention.

The first attempt stopped at GNU Clippy for a Boolean expression. The repaired
source passed its GNU behavior and custody checks. Receipt readback initially
expected the DNS case without its `series::` prefix. The completion run
revalidated the original command output, exact test identities and source, tool,
vendor, target and graph bindings, then ran the remaining lanes and bookends.
The attempts used 138.480 of the original 240 seconds. Windows cross-target
checks provide compiler evidence. Native worker exit under Windows and
production scheduling, persistence and per-file qualification remain open.

| Evidence under `~/.local/share/nevrotick/roughtime-connect-worker.nvhsbEkf/` | SHA-256 |
| --- | --- |
| `check.py` | `2458c1b7fd5a8c1787ab41f90590c9cb38e086a929a510ca1decd55408680600` |
| `attempt.json` | `148719e8217e03313e64ced4c66c3f6b2aa05af3c67c7b9e399f1bbf42496d3d` |
| `attempt.log` | `87d96f334419160badb0c6c9b17c91a9b301433cc514b1c38c3e98621cb50e7a` |
| `check-final.py` | `5fe9e2e22ee91c5254b31a1157b5c98c348dd54aef48c5e2e583c4cd7d67cb03` |
| `final.json` | `e3d54d54114a561abf406aa96db8d437e3bd3b14ed80fc1653b840c752663f18` |
| `final.log` | `b75af9a2e8d006b178e758deab950f9f2a3fd97861c38d25c772b42c95e5806f` |
| `check-completion.py` | `bd5dd57523efdf55c2d2828bf892195f2fe11435e44d56b8b6c4200ea8b910fc` |
| `confirmed.json` | `470f8f91be81f326adc26a92a059f57c5180ec5226c61b56ba6b29810a12b8d2` |
| `confirmed.log` | `90f546ce4042d47ae5944810cf05b4b271a7b1ca5bfa89cfa4a21de260979db8` |

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
