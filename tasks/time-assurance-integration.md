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

Roughtime key/profile support is required in the first catalogue schema. The
controlled V1 implementation and recorded development checks in §3 are present;
full production integration and qualification remain open. A usable Roughtime record
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

**Reconciled 28 November 2026 against `ef446b9f473afdf36d7ce0ea9bd2113b3011fd42`.**

**Recorded progress:** source/documentation research, the coordinated plan and
bounded controlled implementation. Section 3 records wire decoding, mandatory-root
keyed verification, three chained traversals, interval and whole-date admission,
retained signed conflict, invocation withdrawal and shared UDP/TCP retry handling,
with selected development checks on their stated candidates. The
[latest source-policy reconciliation](#352-rebind-roughtime-verification-after-the-authentication-bracket-change)
retains the current coverage and mutation gates. Historical results retain their
original source, graph, case and environment identities.
The research reviewed RFC 10049, RFC 9523, the original Ananke paper, existing NTS
recovery and broker contracts, and retained dated implementation/provider findings.
The plan's sources distinguish specification, inspected implementation and
unverified operational claims.

**Still open:** full production integration and applicable TA-C acceptance,
live catalogue/key/policy/clock-frame issuers, provider interoperability, remaining
current-candidate per-file metrics, native Windows verification/recovery,
performance and either roadmap exit. The controlled implementations and selected
checks do not qualify the complete interval → NTS → broker recovery path.
[The latest OPEN-8 continuation](#353-ta-w07-rebind-the-open-8-overlay-after-the-authentication-bracket-change)
still requires the cross-link, exact PE/PDB binding, cold Windows observations and
complete-interval receiving path. The earlier Sandbox readiness receipt belongs
to its own candidate and does not qualify this task's current binaries.

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
**Historical source baseline:** the observations in this subsection describe that
inspected candidate. The later implementation and development-check records in
§3, summarized in §2 above, govern current progress; the original absence of a
Roughtime client below is not a statement about the 28 November source tree.
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
| **TA-W04 — core/engine owner** | Implement the actual pure evidence-admission consumer and model V1 Roughtime/NTS branch selection, receipt consumption, epoch changes, ancestry, uncertainty and outcomes with independent fixture truth. Retain acquisition during leap ambiguity, all-silent healthy controls and fresh post-guard admission under S1.4. Add reserve-specific behavior after TA-W05 | Applicable TA-C01–TA-C07/10/12/13/16/18/21 controls reach the actual consumer; no duplicate credit, stale admission, calendar-vote learning or extra clock controller. **V1 P2 in progress:** §3 records controlled exchange, interval/date, conflict and withdrawal consumers with selected development checks; live production issuers, full NTS receiving integration and applicable acceptance remain open. **Reserve portions post-V1** |
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

**TA-W07 re-entry evidence.** The earlier `keprov1124a18deba5` readback
retains successful fixed-wall and ordinary KE with guest exit zero and store
restoration. Its `accepted-readback.json` SHA-256 is
`1ec72b7bf440814ac2aac1aa4732d298072bdfc3a73a085e44fbcf22f0f7c7e9`.
The [owning native task](engine-transport-pilot.md#carry-both-certificate-time-classes-through-completed-laboratory-ke)
records the actual ambient-UTC prerequisite and its 26 November CRL expiry.
The result remains a single-point, laboratory time-provider comparison.
Re-enter through R-05/CRYPTO-04 with a fresh admitted synthetic chain/CRL
and the applicable owned Windows guest and service context. The first
discriminator needs a chain valid throughout the candidate interval, a
validity boundary inside the interval, and stale or unavailable revocation
under the same strict policy and controlled wrong ambient time. The current
Windows wrapper and Engine classifier can collapse unavailable revocation
into a generic certificate refusal. Retain a separate typed result for that
case and observe no protected UDP for refusals before proposing the dependent
verifier choice. A separately managed bootstrap trust path still requires an
owner decision on trust, currentness and retrieval. This review supplies no
new guest execution or trust-policy admission.

The [CRYPTO-04 offline matrix](s0.1-crypto-feasibility.md#open-8-certificate-interval-and-revocation-fixtures)
now supplies the fresh, independently checked synthetic fixtures. OpenSSL
distinguishes a valid midpoint with a failing upper bound, missing CRL, and
CRL expiry at the ambient point. The [bound point image](s0.1-crypto-feasibility.md#open-8-windows-laboratory-point-image-binding)
cross-linked and pins the ten exact planned cases. The
[single-point collector](s0.1-crypto-feasibility.md#open-8-single-point-windows-collector-source)
passed PowerShell syntax parsing. Re-entry needs the behavioral controls,
admitted guest trust and UTC, and a native observation of the same policy
with candidate and ambient clocks distinguished.

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

### 3.20 TA-W06 bind retry admission to the selected transport

RFC-W01/W03, TA-C06/14/17/19 and CHECK-9/14/25/27. One controlled root and
retry state now accepts a checked numeric route or pair. Each route specifies
its transport. UDP and TCP can share a numeric endpoint. Repeating the exact
same transport and endpoint, specifying zero port or passing a noncanonical
loopback representation refuses construction. The original two-address fixture
constructor continues to authorize both transports on its selected addresses.

Direct UDP submission and TCP connect check the selected endpoint and transport
before consulting the retry floor or preparing native work. Round construction
checks both selected routes before it can hand the original retry state to the
worker. A known address with a different transport yields
`UnselectedTransport` and leaves the retry debt and native lane untouched. A
different address yields the original `ForeignPeer` refusal. The shared floor,
socket custody, signed frame chain and complete-date reset retain §3.19's owners.

Use actual loopback listeners to verify transport-specific refusal with no
native traffic and immediate healthy progress. The scheduled fallback control
uses the same numeric UDP and TCP port and checks the original floor and all
nine replies. A compile-fail consumer checks route construction privacy. Retain
the GNU/MSVC and shared DNS/TCP regression selections under one 240-second
execution and repair allowance. The route type consumes controlled selections.
Live catalogue issuer approval, exact provider evidence, native Windows
retirement and per-file qualification remain separate prerequisites.

The final 802-input candidate
`908364c1ed85bd0ba0679a8b35caf3ecae978c3e211ed27f4a18b8dd3dbbddfa`
passed all twelve selected lanes, six executed on this candidate and six
reused after exact affected-input comparison. The final GNU integration
roster contains 47 tests. Its scheduled fallback drove all three TCP visits
on the exact port used by the unanswered UDP peer and read back all nine
original signed replies. The negative control observed empty UDP and TCP
listeners after the mismatched requests and completed valid exchanges under
the same original authorities. Direct construction accepted canonical `::1`
and refused zero port, a non-loopback IPv4 address and IPv6 scope or flow
fields. GNU and MSVC Clippy for Roughtime and the combined KE/Roughtime MSVC
check passed. The unchanged 14 selected units, two shared worker custody controls,
97 Engine doctests, 12 wire tests, four wire doctests and default MSVC Clippy
match their passing first-run inputs. The Engine doctest roster includes the
new private-field construction control.

Fresh source review followed exact route identity through retry admission,
round selection, worker dispatch and guarded socket adoption. The first attempt
passed all twelve lanes with 46 Engine integration tests. The review identified
a direct constructor-boundary test gap. Only the integration fixture changed
between the attempts. The successor rechecked the affected targets and exact
47-case roster, then verified the unaffected lane identities and the candidate,
tool, vendor and graph bookends. Both attempts consumed 195.696 seconds
of the original 240-second allowance. Wrong-transport priority over a deferred
floor is covered by source review. Native Windows and production issuance,
restart continuity and per-file metrics remain open.

| Evidence under `~/.local/share/nevrotick/roughtime-transport-binding.BS5up5Rj/` | SHA-256 |
| --- | --- |
| `check.py` | `d4476e21455b7f1d5a2b1d08f38764f416c18ac745c60f5addbdbdd530273104` |
| `attempt.json` | `8387b8af56030f7814474be591303913caa1c6109c6fff62ea0e189c696cf33e` |
| `attempt.log` | `09197bf1636246db38faa4fae8cfda18ef8f78d0e782e54ea819ed0f47ca5e1a` |
| `check-boundary.py` | `00a3d5e5285749258907f7dfb1503ba945f19e06839b04a140a5c1d041abd771` |
| `final.json` | `126de255361e40ff5336b3366562928f94b0bf8b3dd26ca8f877a4b166370109` |
| `final.log` | `c9e1be513c67aa944240b8831ffe7fe91b9364d165f8c15187b7795f6c635cd7` |

### 3.21 TA-W06 select one retry-ready independent Roughtime round

RFC-M02/W01, TA-C01/06/14/17/19 and CHECK-7/8/9/14/21/25/27. The
controlled recovery constructor takes three through sixteen checked candidates
with their original exclusive retry states. It checks retry floors against one
clock sample and observes any worker settlement separately for each candidate.
It refuses duplicate UDP endpoints or roots across the roster and enumerates
every feasible trio. Each pair must have distinct endpoints,
keys, operators and upstream clock groups. A fallback-mode trio needs an
advertised TCP route and each selected alternate must preserve distinct endpoints
when it replaces that peer's UDP route. Sharing its own numeric UDP port remains
valid. The original retry floor is checked again at each native effect.

The finite enumeration visits at most 560 unordered trios. One rank across the
six orders of each feasible trio uses the admitted operating-system entropy
source. Accept complete buckets only, make at most eight draws, and refuse
entropy failure or an empty feasible set. The original absolute deadline covers
the search. The invocation rechecks it with the original control before issuance
and retains the selected order and exclusive bindings for both rounds. Refusal
emits no probe or successor and spends no invocation slot.

The integration fixture excludes a peer with observed retry debt and compares
the sole feasible trio with a greedy trap. It observes the selected order across
all nine real signed loopback replies and checks the original frames and quiet
unselected sockets. The fallback control checks the original UDP prefix and
retry floor, same-slot TCP substitution, and retained order through all nine
fresh signed replies. The deterministic rank controls cover the six orders of
the sole feasible trio, rank boundaries at the 16-peer cap, incomplete buckets,
entropy failure and bounded redraws.
The selector draws over eligible concrete peer schedules. Approved production
roster issuance and the shared live work budget retain their existing owners.

The Appendix B follow-up reaches NevroTick's actual keyed verifier with all
three published originals and all three proposed erratum 9238 replacements.
The originals must refuse the final-profile delegation signature. The proposal
must authenticate each original exchange, retain both nonce links and expose
contradictions on the first-to-third and second-to-third pairs. The proposal's
first two midpoints agree. The third is 86,400 seconds earlier.
The RFC Editor lists the erratum as Reported. Separate signed loopback cases
establish successful agreement and original-round conflict handling. [RFC Editor errata](https://errata.rfc-editor.org/search/?rfc_number=10049)

The isolated 818-input candidate
`e0430154f54afa623c8de88851a2487ab1737ab739bf8c97d2ffab33cb2fd217`
passed all twelve selected lanes. It ran 17 Roughtime units, the two
same-process worker-custody controls, 97 Engine doctests, 12 wire integration
tests and four wire doctests. MSVC selected/default Clippy and the combined
KE/Roughtime target check passed. The receipt preserves the GNU Clippy pass
and the exact 51-case integration roster, including both Appendix B controls
and both randomized loopback paths, from the preceding attempt. The final
source change removed one redundant path qualification in a unit test. Exact
comparison confirmed the applicable source, tools, target libraries, vendor and
graph inputs for those two reused lanes. The same input bookends matched on the
final run. The initial lints, unit compile refusal and repair consumed 199.226
seconds of the original 240-second allowance, including the passing run.

Fresh source review checked selection arithmetic, exclusive retry binding and
TCP route mapping. The review identified the additional rank-boundary and
randomized fallback controls. The independent byte check of Appendix B used
both signature contexts, verified the nonce links for the replacement, and
reported the exact contradictions. Its external `context-result.json` has
SHA-256 `63a640cd4aee1393c0e737eec3fc3b5447238f1103e49a9920b0ab7f0bada2ce`.

| Evidence under `~/.local/share/nevrotick/roughtime-random-selection.US9zzUHG/` | SHA-256 |
| --- | --- |
| `check.py` | `d295b2753bd4e75f2e21b61e98f908c29f9b28d8d030905d9f86dfb8b430a8fc` |
| `attempt.json` | `54063ec06f46dede85bfa0e876d06805382c0e779671406d627e0a1ff091a44f` |
| `check-final.py` | `fcd54ea240e8e987f1aa1b218c4254d9317e855c472dfcc1b5695c902da98efa` |
| `final.json` | `1441fda4fe391a783156909e334659e35dc368c02237c7cdd92cfc6b34d47a3f` |
| `final-expected-cases` | `4f4ba691bfe0652d6f7b790c4bd3f0927ed5d07d3a356cd11b8f5f6bfd574178` |
| `completed.json` | `7947220b57cb98f121b4c0afa434822f2d638590804893337c57af0e8355844c` |
| `check-confirmed.py` | `dce6b92dac354ba324aef6631d69a57bca933b051b7736e36491bb47cc10c42a` |
| `confirmed.json` | `587e6f83e552174f1cf2acb7cd2426b57ca3ee2750b6d6368eb0e194fb0ff9dd` |
| `confirmed.log` | `986d4a21f5c5a27307a87a5e47e0012da299194c21bb6955c75889bda7acb245` |

The next controlled source child reconstructs a proof from the original
`ConflictEvidence` for RFC-M01 using the retained roots, frames and linking
bytes. DATA-18 still owns authorized export, persistence and clearance. Live
catalogue provenance, native Windows retirement, restart continuity, release
trust and the per-file gates remain open under their existing owners.

### 3.22 TA-W04 reconstruct the retained signed conflict

RFC-M01, DATA-18, TA-C06/13/19/21 and CHECK-6/7/9/14/25. The existing
`ConflictEvidence` receiver rechecks the complete original prefix against each
expected endpoint and root in the original selected order. It uses the keyed
wire verifier on the exact request and response frames and verifies every later
link to the preceding complete response. The prefix contains 2–9 entries and
must end at the original witness. Live receipt acceptance and reconstruction
both use the strict signed bound comparison and choose the first earlier
violating position. The borrowed `ConflictProof` is available only from a
successful replay of that exact immutable evidence.

Replay is bounded, uses no I/O or current clock authority, and preserves the
original conflict on every refusal. It does not retry a probe, refresh a date
or open an export sink. DATA-18 owns authorized export, persistent state,
alerting, bounded remeasurement and clearance. Retain OPEN-DATA-05 and existing
release, per-file and native gates.

Check the actual recovery receiver after the original stop/expiry and a later
round. Use live signed two-entry and nine-entry prefixes with an independently
specified first witness. Use the intact reported erratum proposal with targeted
identity, shape, verification and nonce-link faults and a consistent prefix.
Check external construction refusal beside the legal borrowed proof. Select
isolated GNU and MSVC development checks in one 240-second execution and repair
allowance. Unchanged wire and default MSVC scopes stay with §3.21.

The isolated 820-input candidate
`0e4eefd756118efa85ad6599c30728d9cc95b50c9cc0c148381d7e6dad0d59d1`
passed all nine selected lanes. GNU ran 52 integration cases, 20 selected
Roughtime units, the two same-process worker controls and 98 Engine doctests.
The integration cases reconstruct from the original receiving path after
withdrawal, after timeout and after a later round. The two- and nine-receipt
controls require the earliest original witness, even when another earlier
receipt has a higher signed lower bound. The Reported erratum cases reject
missing and changed links, malformed frames, a changed signature, wrong
identity and false witness while retaining the valid original fixture.
The public construction refusal and legal borrowed consumer both passed.
MSVC Roughtime Clippy and the combined KE/Roughtime target check passed.

The first Clippy invocation stopped before execution because the verified
frame accessor tied the response borrow to the temporary verifier. The repair
preserves the wire response's independent byte lifetime in the existing
accessor. Both attempts used 121.215 seconds of the original 240-second window.
The final input bookends and all 820 live digests matched. Independent source
review traced the historical receiver, exact roster/witness binding, private
proof construction, bounded rejection and unchanged recovery state.

| Evidence under `~/.local/share/nevrotick/roughtime-conflict-proof.0Iyi0ruu/` | SHA-256 |
| --- | --- |
| `check.py` | `a26269a084ca3d94f3f5368937f5fd0d5160e6887ab8446d6520ae881515a4eb` |
| `expected-cases` | `aa334e70aa2d1eb12bdf7184bf70e9e5bb245cc7bbc93b82765d4447c9482e08` |
| `attempt.json` | `4ed54e4b41405095a9ba6b8025d2b7e22384b75f0c29c926275595a5bfd36b95` |
| `attempt.log` | `7f1ef60a9183eeafe214b132475b0c7c784136ad22c0d25243480a60b3714e1e` |
| `repaired.json` | `81b1083b1f2b8f98f97545900c336aaea09ffcf9ae6543516221a87b07577933` |
| `repaired.log` | `a58a62dca858f56c14f6eb715014f2f4c61f75178bfe5ccebf6c5c666efeda55` |

The next independent qualification action is a guarded GNU per-file coverage
lane for the Engine Roughtime owners. The current wire lane covers the decoder
file only. Use the existing portable analysis workspace, exact manifest file
selection, pinned stable tools, Roughtime graph/vendor and the Engine test
identities above. Recheck private scratch capacity before execution. Keep
the 90% per-file line gate and separate 95% critical-file mutation obligations.
DATA-18 and OPEN-8 still govern the authorized export and native same-handshake
NTS continuations.

### 3.23 TA-W06 independently check emitted padding and reserved TCP withdrawal

TA-C06/14/19 and CHECK-6/21/25/27. The review of `be318f7` identified two
test gaps. Compare both against the current candidate. The real UDP request
observer uses the published Appendix B request for framing, tag order, value
widths and version. It independently derives the expected server commitment
from the known root and checks the nonce width, request type and every padding
byte. A copy differing only in the last padding byte must fail. Exercise one
directed fault in a private frozen tree to show the actual sender regression
fails that assertion while the original encoder succeeds.

Pause the actual scheduled TCP worker immediately before its dispatch claim.
Withdraw separately through the round and invocation. Observe the control
result before polling or dropping the receiver, so receiver retirement cannot
mask a broken control latch. Observe no connector or request, retained lane
ownership, actual worker join, the exact original retry floor and a healthy
TCP successor. Keep the fixture seam test-only. Use one 240-second allowance
for the directed fault, isolated GNU/MSVC checkpoint and any needed repair.

The exact loopback case passed on the unmodified encoder and observed a
one-byte padding defect at the independent sender assertion. The private
specimen changed only the last emitted padding byte to `1`. The build reran
the Engine and executed the same selected case, which failed with the expected
profile message. This took 35.029 seconds, including retirement. The fault
was confined to the private specimen.

The isolated 820-input candidate
`45330ff66bed4e0898801cb790cd4df921752ba6a325b799e120f9a927f72dc1`
passed all nine selected lanes. The first combined invocation passed GNU
Clippy and all 52 integration cases, then stopped compiling the unit fixture
at an equality assertion on `Result<(), ExchangeError>`. The fixture now uses
the existing success check. The final run re-admitted the applicable inputs
for both passing GNU lanes and verified the exact one-line unit-only change.
It passed 22 Roughtime units, 98 Engine doctests, and both selected MSVC checks.
All four selected DNS/TCP custody cases also passed together in one process.

Both Stop controls produced the expected `Spent` result on the original
worker before receiver cleanup. The connector stayed unused, contenders saw
the occupied lane before join, and neither a connection nor a receipt
appeared. After actual join the same binding refused the predecessor of the
original retry floor, allowed the exact floor, and sent a real successor TCP
request. All 820 live digests matched the final bookends. The directed fault
and both combined attempts consumed 173.076 seconds of the 240-second budget.

The default and KE hosted profiles cover their configured scopes. The
Roughtime execution above comes from the explicitly selected local feature.
The Engine per-file coverage and mutation gates, native Windows execution,
and the Roughtime-to-NTS receiving integration remain open under §3.22 and
their existing downstream owners.

| Evidence under `~/.local/share/nevrotick/roughtime-audit-controls.hoQZmeky/` | SHA-256 |
| --- | --- |
| `check-padding.py` | `34703a688228a2500d77f11a2f895b476f0719b7a716cb3f5b0a7a7658274d0d` |
| `padding.json` | `a14241b86de0de53a3cf8143b30e477e9ca07e9c6d0753d3da04a065377d5fd4` |
| `padding.log` | `deaa7e2a2003d16312eeec8baae8e76670b678520a571b828453e1ff2daa9acb` |
| `sg01-pre-dispatch.patch` | `807c13c7d1ca95a7827d539db94855969528146085ee0d8ce4285e04730e8f91` |
| `check.py` | `0ba2e728c127668773792a42338ea2098b25530f6ede4092bc83e490d8f22f5c` |
| `attempt.json` | `8d0547980b3239e310b2caef1672395274e94e601aaece2f0295c4a98e1be25a` |
| `check-repaired.py` | `6af57d1dab9c63f5d843e7a573fc07e0dc1f154bf89cbcb4c72f257691778289` |
| `repaired.json` | `288ac96f5fcc9fb493fbff1db69183c78fee8f3ac8a687058d3e347d2cb95590` |
| `repaired.log` | `1e91f1026d8eb5cd4c35994a4d9b19d5238d204cf8707ba7ca422773538b622d` |

### 3.24 TA-W03 hostile wire and keyed-verifier coverage

TA-C06/14/17/19 and CHECK-6/25/26. External packet bytes, roots and persisted
provider descriptors require their normal admission checks. The added controls
exercise the public request, response, TCP header and selected-root entry
points. All generation and fuzz execution use local fixture bytes, with network
namespaces and task-owned writes. The catalogue collector has its own duplicate
member control in the [catalogue task](server-catalogue-and-statistics.md#88-hostile-catalogue-member-coverage).

The stable wire suite exercised 8,184 deterministic raw and length-correct
frames across sizes `0..=4097`. The response corpus covered 528 accepted cases
across all 33 Merkle path depths, two timestamp extremes, four radii and opaque
signed fields. Its 1,568 malformed neighbors reached the path, index or
delegation guards. A first fixture used a single trailing path byte and hit the
outer word-alignment check. Four trailing bytes keep the outer offsets aligned
and distinguish the intended inner path refusal.

The keyed Engine cases checked 1,518 frame-boundary cases and 198 single-byte
changes. They require the specific refusal at the actual keyed entry point and
check that independent healthy exchanges still retain exact frames and bounds.
The final 821-input candidate
`05e7a974366c932c3133bc9f026e55ce21d667af54c4b72c1f536bb05c326d95`
passed 54 Engine integration tests, 22 Roughtime unit tests, four same-process
worker cases, 98 Engine doctests and 14 wire tests. The isolated MSVC Roughtime
Clippy and combined KE/Roughtime compiler checks passed. The final source change
was confined to the wire test fixture. The shared runner rechecked source and
tool identities before crediting prior successful Engine GNU results. The two
attempts used 137.668 seconds.

The [separate libFuzzer target](../policy/roughtime-wire-fuzz-target.rs) receives
unchanged frames, with the same oracle as the stable wire test. The pinned GNU
nightly built distinct healthy and observation-control executables from the
admitted offline graph. Replaying the original valid request and response seeds
passed. Replaying the accepted request with the observation control produced the
intended assertion and exit 77. The healthy corpus contained those two original
frames and a response missing its last byte. LibFuzzer completed 256 callbacks
under the existing 4,096-byte, 2-second per-input and 30-second total flags, with
seed `688161900` and final `cov: 155`, `ft: 233`. The raw logs retain build, graph,
tool, fixture and executable digests. The frame above the cap is exercised by
the stable tests. The first attempt ended during inventory serialization, before
compilation. Both attempts took 387.695 seconds. Both binaries used the admitted
ASan build selection, but the assertion-induced signal qualifies only input
forwarding and failure propagation. It is not a representative ASan detector
control and supplies no ASan memory-fault qualification. The AES hold and
six-owner selector retain their existing scopes.

The configured per-file line rerun was refused before compiler startup. The
analysis runner rejected the reserved `/dev/shm` work path. An outer private
namespace provided new scratch, but its exact bind of the analysis tool root
was also refused as a mount root. The earlier decoder receipt retains its
original scope. Fresh per-file line and mutation gates, a
[maintained fuzz selection](#331-ta-w02030406-follow-up-fuzz-targets), native
Windows and production effect checks remain open.

| Evidence under `~/.local/share/nevrotick/roughtime-hostile-coverage.Hq03x6mA/` | SHA-256 |
| --- | --- |
| `check.py` | `c609c0fd1f90c78aac0d73c91e43b6897e2ff73ebfaa19a4275fbbe9b726cffd` |
| `attempt.json` | `335a7a5829d5fa373327849588dfea4ba791eb6d9f871c7803bf4ea42dfad293` |
| `check-repaired.py` | `3e39bd6ae1e9c10352b031fb245f371937505c50d2119e674423131f90a3647c` |
| `repaired.json` | `3aa39801b8ffec67d42c9f00e0933fe0a2dcf61f334daab125449da0914ed11a` |
| `check-fuzz.py` | `7aa47b7a7a81fe59d89157d4da2fbfb7415f00cf5a7fec30f817aee6bbf0bec0` |
| `fuzz.json` | `5bb237ad0560fe2e78815ddf5cd26d357d71ef64c0cf15c7b830a61bbc6fb2af` |
| `check-fuzz-repaired.py` | `e251f94bb1c40b6662204a81835a701ed968a357860ce89433b7cd84cf57f305` |
| `fuzz-repaired.json` | `cd611629cb147b17a3a1c5c3a6c360851aa25f4b1949e1d3b9e4754062091948` |
| `coverage.log` | `4cb0c2b40fbcaa0ffb60500fe27975326ae889ebddc55d90d0879ff6043b4aff` |
| `coverage-nested.log` | `a4e9c26fbe706a7086a282635c416f00327836bcac14b1baa27d0516caab4461` |

### 3.25 TA-W03/04/06 second-audit regression controls

TA-C04/06/14/19 and CHECK-6/21/25/27. The second review of `be318f7`
identified five specific test gaps. Current source inspection confirmed the
original deadline, transport, cryptographic, and withdrawal guards. Sections
3.23 and 3.24 already cover UDP emission and both pre-dispatch Stop paths.
This checkpoint exercises the remaining discriminators on the current
optional Engine Roughtime feature.

| Finding | Skills and exercised boundary | Independent expectation |
| --- | --- | --- |
| C-CB-001 | `implement-rust-contracts`, `qa-contract-tests`, and `qa-boundary-edge-cases` at bound date admission after nine real signed UDP replies | Entry two nanoseconds before cutoff and exactly two owner observations. Return one nanosecond before cutoff admits once and resets all three floors. Return at cutoff, a reversed sample, or exhaustion refuses without clearing the original floors. The second admission is spent in every case. Production samples its own monotonic clock at both checkpoints. |
| P-T01 | `qa-golden-fixture-parity` and `security-protocol-inputs` at `PendingRequest::verify` | Exact independently signed depth-two/index-two and depth-32/maximum-index vectors retain their original frames and bounds. Changing the later orientation or sibling order yields `RequestCommitment`. The complete response fits the padded request for each vector. |
| P-T02 and P-T03 | `qa-integration-tests` and `qa-adversarial-inputs` at real loopback receive and send | The UDP client processes the exact 4,096-byte frame, refuses 4,097 and 4,352 bytes with the same authenticated prefix, and spends each attempt. The capped reply is a client-limit specimen from a test signer. The TCP peer independently checks the emitted profile before signing, writes two frames together, and observes closure before publication of the exact first receipt. |
| R-F01 | `qa-state-concurrency-lifecycle` at both original Stop bindings and the actual TCP connector | The original refused dispatch retains its occupied lane through join, with zero connector calls or receipts and the exact seeded floor. The same gate then lets a live successor dispatch once. Retiring that real charged attempt advances the original sequence to 1.5 seconds. The earlier post-dispatch test retains its separate `RetryInFlight` observation while a binding is borrowable before join. |

`qa-environment-preflight`, `qa-regression-selection`, and `qa-independent-review`
used a frozen candidate and the existing isolated, offline recipe. The first
snapshot missed a manifest required by dependency admission. The two later
initial checks stopped at Clippy before running Rust tests. They identified a
slice assertion and private fixture documentation and ownership warnings.
After these corrections, the exact 826-input candidate
`c6ad5cb20b52fcb2b3ba53edff2b2c1f99d24768e53f47758df36ffd057e90fa`
passed GNU Clippy with `--tests`, all 58 selected Engine integration cases,
23 Roughtime units, the four-case same-process DNS/TCP cohort, 98 Engine
doctests and 14 wire cases. Format, selected MSVC-target Clippy and the combined
KE/Roughtime MSVC-target compiler check also passed. The final run took
121.806 seconds and all attempts used 181.650 seconds. The final source,
tool, vendor and graph bookends matched, and the live source map readback matched
all 826 digests. Independent Python/OpenSSL readback verified both signatures
for each deep proof.

Native Winsock outcomes and provider retirement retain their Windows owner.
The per-file Engine coverage and mutation gates and full Roughtime-to-NTS
receiving qualification remain open.

| Evidence under `~/.local/share/nevrotick/roughtime-second-audit.GwtgZ67w/` | SHA-256 |
| --- | --- |
| `attempt.json` | `1705d6c7631d9cfed4d5299f80985a1c4ce396ca254c86e9b64559187f012c6a` |
| `combined.json` | `ad2fdf2905af8b63ebf64cd24af28d2db9e444b278099b89aa640cae949fe2b2` |
| `repaired.json` | `4a05755da9355056e84836ac6ed6fe0cae1a4bf2e0be417632001095f2278fdc` |
| `check-final.py` | `05d2742baae6421b564222bc5e1b598162dd1a011261b9c7e078812df81bae4c` |
| `final.json` | `4b56341d2a35a8d8ff50017cd9e2ef7d2af872a4e5961c30cb9a92dc685cc9f7` |
| `final.log` | `bd8678b7f5cac453c2f0d1de074c490c86801c8bb91717e8773a3e142f7e3872` |
| `merkle-readback.log` | `cb301c4a5a012104dd69dc5c7753a1b492099c1a510e397bb432fca3e5fe9030` |

### 3.26 TA-W03/04/06 exact Engine Roughtime line coverage

CHECK-6/19/24/25/26. Select the ten registered Engine Roughtime owners under
the current `roughtime-verification` feature and GNU host target. The signed
`c97041d` product input map and §3.25 receipts identify the exact 23 unit and
58 integration cases. The coverage command selects their complete names and
reads back each original binary, including the declared filtered unit count.
An ignored, duplicated, substituted or displaced result cannot earn credit.

| Stage and skill | Requirement and action | Evidence or remaining condition |
| --- | --- | --- |
| Planning and preflight with `plan-code-change`, `qa-test-planning`, `qa-environment-preflight` | Bind the exact source and registered security files, stable coverage tool, feature, locked vendor and child output directory. | A private outer Linux scratch preflight reached the inner namespace with ordinary identity and read-only tool/vendor inputs. Preserve the runner's existing path, source and resource checks. |
| Implementation and regression with `qa-coverage-gap-analysis`, `qa-regression-selection`, `qa-contract-tests` | Require qualified names per binary and retain each selected file's positive denominator and 90% line threshold. Reject unsupported selector combinations and the wrong candidate. | The initial role is `--profile roughtime --lane coverage --tool-admission-profile stable-core`. The verified source map, actual tool output and focused reader controls own this checkpoint. |
| Independent review and closeout with `qa-independent-review`, `qa-context-checkpoint` | Compare all source, tool and vendor bookends and verify executed cases and exact coverage attribution. | Per-file mutation, controlled wire fuzzing, native Windows and same-handshake NTS integration have separate owners and evidence. |

Use one finite 480-second execution allowance across scoped preflight, commands
and required repair checks. The coverage child has its own 240-second wall cap.
The first focused reader run rejected its healthy fixture because its extra-row
check counted the libtest summary. The corrected check excludes only the summary
and still rejects an extra ignored row. Five focused reader and selector tests
and 67 existing shared wire-analysis controls passed. A separate executable-path
control refuses a mismatched GNU test binary even with the correct source label
and complete case list.

The first instrumented run executed all 81 selected tests. It retained the
raw report and closed its source, tool and vendor bookends. Nine files met the
90% gate. TCP covered 309 of 353 executable lines and the tool returned an
incomplete gate. The two added loopback cases exercise an invalid and a foreign
preconnected endpoint and the 64th idle read after observing the actual request.
The existing frame refusal case now also checks the decoder's typed bad-magic
result through the live TCP receiver. These controls exercise application
stream retirement through the peer. The idle-budget fixture requires the
initial request write to complete in one poll, as observed in both GNU runs.
Windows provider retirement remains a separate native observation.

The final product map `8b06f0b43e46aa52722c8a9f6a7b1290ad52f11b9555cd008d2549a40dbef5e2`
changes only the Engine integration test relative to the first 826-input map.
It executed 23 unit and 60 integration cases through both exact GNU binaries.
All 834 admitted source and tooling digests match the live files. The retained
vendor map contains 29,754 files, with digest
`e756aee1e8211d464b283eff3e27d0d097a8b4c9cbc0505a24e3e71a5f4f9edf`.
The configured gate passed for all ten files. Independent readback checked the
raw counts against the per-file results and exact roster.
The affected fixture also passed formatting and GNU/MSVC-target Clippy, plus
all three selected TCP tests in an ordinary GNU run. The focused run matched
the same 826-input product map and consumed 42.176 seconds. The two outer
coverage launches took approximately 92 and 90 seconds respectively, measured
from owned output creation to final receipt export. The scoped execution stayed
within the original 480-second allowance.

| Engine owner | Covered / executable lines | Percent |
| --- | ---: | ---: |
| Verifier | 99 / 101 | 98.02 |
| UDP exchange | 352 / 379 | 92.88 |
| Request | 8 / 8 | 100.00 |
| TCP | 318 / 353 | 90.08 |
| Retry | 210 / 214 | 98.13 |
| Round | 457 / 483 | 94.62 |
| Selection | 186 / 197 | 94.42 |
| Recovery | 326 / 341 | 95.60 |
| Conflict proof | 63 / 65 | 96.92 |
| Interval | 180 / 192 | 93.75 |

The source-bound GNU per-file line gate closes for this candidate. Keep
critical-file mutation qualification open and reconcile the actual per-file
descriptors and thresholds before admitting a campaign. Live trust and date
issuers, OPEN-8 verifier selection, native transport and complete same-handshake
Roughtime-to-NTS/broker qualification retain their existing exits.

| Evidence under `~/.local/share/nevrotick/roughtime-line-sandbox.XUq85F6u/` | SHA-256 |
| --- | --- |
| `launch.py` | `adf2e70b51dde6a526caca10c439c34246b8fb74fa1f0cee62cd1d12003d2010` |
| `controls.log` | `a17d3ad89044c4298d48c8e4b45e87bd9b9cb4ccee0a87ba3a44808fb7d2d545` |
| `controls-final.log` | `12256ca6d5306b991cfaed0cfed03ff6e33ee42ad15791affc778f5551b8ec08` |
| `shared-wire-controls.log` | `7e536dc2b0b7a436831eb50608521b14f2becebcd5fd96d6f9cd823233cb9b70` |
| `attempt/verification.json` | `d327ddeb3a27d940aa8d3ea845ea39cb300599c29c8e43aa04585228727e227f` |
| `attempt/coverage.json` | `8e00201cd6f2e0363a9fc6283b2bdf2a4c5411dcabd18a1003b8ddedee12aa51` |
| `final/verification.json` | `f11285485e60107ce99caa83be350fd85a16b6de6465417810a8c176c85c6ceb` |
| `final/coverage.json` | `463ca70f3e0a42c15cd9db7866a481068811d201cb85b333da5f9ae33199548b` |
| `check-tcp-lints.py` | `910906f5a80e5d5982c96b2bd14da5e43a0844a717cd5f13acc7ba51a02b1628` |
| `tcp-lints.json` | `846a6a609efb8e05d6f7cef7d593ed0ffc2bfaf012b6ae2e7c48f0a34e039cdc` |

### 3.27 TA-W03/04/06 Engine Roughtime mutation inventory

CHECK-6/19/24/25/26. Freeze the same ten critical files and product inputs as
the preceding line checkpoint. Admit `--profile roughtime --lane enumerate`
with stable-core, the exact `roughtime-verification` feature and one fixed
`--list --json` command per registered file. Retain only generated descriptors
and their scope. The 95% gate requires later viable outcomes and a passing
baseline. A listing supplies no viable denominator, equivalence or test result.

| Stage and skill | Requirement and action | Exit or remaining condition |
| --- | --- | --- |
| Planning and preflight with `plan-code-change`, `qa-mutation-testing`, `qa-environment-preflight` | Bind the preceding line policy, product map, pinned Cargo tool and stable GNU namespace. Use one shared 120-second listing deadline. | The exact tool can read the sealed source and emit bounded output. A separate 300-second execution allowance covers targeted controls, listing, affected line replay and repair. |
| Implementation and controls with `qa-contract-tests` and `qa-regression-selection` | Guard the exact selected owner, features and list-only command at the Session effect. Require ten unique, nonempty per-file inventories, at most 512 per file and 2,048 combined. | Exercise the accepted vector and refuse substituted commands, campaigns, alien files and malformed or foreign descriptors. Preserve the old line role and existing profiles. |
| Closeout with `qa-independent-review` and `qa-context-checkpoint` | Independently reconcile exported names and locations with each critical operation and identify generation gaps and qualified alternatives. | Retain raw inventory and per-file digest, explicit unsupported/ungenerated obligations and the exact next pre-campaign condition. |

The first listing completed in 37.84 seconds. Review identified two admission
gaps in the new route: the shared reader accepted partial descriptor shapes,
and the child timeout was set before rechecking source and tool custody. The
final route checks the observed descriptor schema and source spans, and
recomputes the remaining timeout after that custody check. It also refuses
terminal credit beyond the original listing deadline.

The final listing took 37.14 seconds and all ten commands exited successfully.
It produced the same **601 unique descriptors**, including identical raw
descriptor data, against product map
`8b06f0b43e46aa52722c8a9f6a7b1290ad52f11b9555cd008d2549a40dbef5e2`.
Independent readback matched all 835 bound source inputs, the per-file
descriptor digests, and the pinned tool and vendor bookends. Seven focused
inventory controls, five line-analysis controls and 67 shared wire-analysis
controls passed. Affected line evidence was replayed with the final runner in
94.30 seconds: the same 23 unit and 60 integration cases executed, and all ten
files returned the coverage counts in §3.26. Only the two runner files changed
in that replay's bound source map.

The independent source reconciliation distinguishes directly generated
operator or arm changes from whole-function replacements and operations the
listing does not target individually. The rightmost column records the controls
and gaps at this listing checkpoint. Sections 3.28–3.30 record the later
regressions, including both adjacent invalid roster sizes. A named test alone
supplies no mutation kill.

| Critical owner | Listed | Example scope and generation gap | Independent control or remaining evidence |
| --- | ---: | --- | --- |
| Verifier | 36 | Merkle index shifts are listed. The request constructor's selected-root commitment check and the strict signature-provider call have no individually targeted descriptor. | Every commitment byte is checked in the keyed verifier, and signed depth-two and depth-32 proofs exercise folding. Provider strictness sensitivity remains separate. |
| UDP exchange | 99 | Receive sentinel size and publication comparisons are listed. Actual send debit, socket release, and capture transfer lack individual descriptors. | Real UDP cap and excess cases, retained alias debt, and withdrawn publication controls. Winsock's generated oversize mapping needs native execution. |
| Request | 8 | Encoder body replacements and size arithmetic are listed. Literal prefix and individual copy and padding operations have no targeted descriptor. | The peer independently checks emitted UDP and TCP request fields with a one-byte padding refusal neighbor. |
| TCP | 66 | Deadline, frame and idle-counter expressions are listed. Worker-witness transfer, partial-write slice selection and stream release lack individual descriptors. | Both pre-dispatch withdrawals and the post-dispatch custody test, plus fragmented, coalesced, idle and truncated frames. See §3.30 for the later forced partial offers and retirement controls. Native provider retirement remains open. |
| Retry | 51 | Join/time ordering and backoff arithmetic are listed. Constructors and lease transitions lack operation-level targets. | Seeded debt in worker and final-date tests, rational backoff controls, and route validation. A duplicate-alias refusal through the legacy constructor remains open. |
| Round | 98 | Return-clock reversal, cutoff, and terminal comparisons are listed. Individual dual-witness assignments, guard reacquisition and retry reset calls are absent. | Separate round and recovery withdrawals and bounded clock observations inside date admission. Deterministic withdrawal between transport poll and final publication guard remains open. |
| Selection | 58 | Rank and rejection-sampling arithmetic are listed. Fixed permutations, roster caps and actual readiness calls lack individual targets. | All six trio orders, reachable rank across a 16-peer roster, bounded redraws and actual ready-trio reuse. An explicit over-cap roster control remains open. |
| Recovery | 73 | Budget, cutoff and conflict comparisons are listed. Exact silence variant, waiter arming, notification and evidence transfers lack individual targets. | Malformed UDP blocks fallback, silence restarts the chain, and Stop wakes the wait. Dropping the pending waiter without spending a round remains open. |
| Conflict proof | 26 | Shape, index and first-witness comparisons are listed. Exact replay calls and duplicate-link lookup lack individual targets. | Valid two- and nine-receipt prefixes and corrupted frame/link refusals. A correctly chained proof with repeated randomness remains open. |
| Interval | 86 | Capture order, age, width and scaling operators are listed. Policy construction, exact margin operations and calendar scope call lack individual targets. | Bracket, outward rounding, and scope refusal controls. Direct reversed-rate and zero-limit constructor refusals remain open. |

The 601 listed operations exceed the existing 512-descriptor campaign
reference by 89. Each per-file listing fits its 512 limit. The enumeration
policy rejects campaign execution and grants no mutation or native credit.
Before admitting a campaign, freeze the selected cases and passing baseline,
disposition the generation and target-specific gaps, and review bounded
per-file batches against an aggregate execution and resource budget. Keep the
95% viable-mutation threshold for each critical file and retain each observed
outcome against its exact denominator.

| Evidence under `~/.local/share/nevrotick/roughtime-inventory-sandbox.Rck16X27/` | SHA-256 |
| --- | --- |
| `launch.py` | `19e5bdbe55bf4545fae04c4328bc4a9407ffa1b02524cf7cfa6e3de2f382a2f0` |
| `inventory/verification.json` | `76f32d38038c0da391e86e6c780d8f56eb5d4682456867704e1dda1d290a708c` |
| `inventory-final/verification.json` | `b42011f31a2fe87c03586baf213dec09c7631caeb6a0ac009e45f67f0d64ff15` |
| `inventory-final/launch.json` | `8641d8d988d05b6f1db604cce914e1005881f9a2508e6ed28f35351a6e13efbb` |
| `final-engine_roughtime_inventory.log` | `f316a7d700c8cc3e37e9697d540801e8ec68609cef77c78adfc9373610e0d1be` |
| `final-engine_roughtime_line_analysis.log` | `12256ca6d5306b991cfaed0cfed03ff6e33ee42ad15791affc778f5551b8ec08` |
| `final-verify_wire_analysis.log` | `e9769180dee0b5d7ff71b0bfa62fcb2a30df8b71ee077c48109bde7412cf7a26` |
| `line-replay/verification.json` | `5a37bc12e5ac355733f2273564eebcfeac091e2016819ee176c64f7101fb527f` |
| `line-replay/coverage.json` | `d110f9e5a167ab8361fca2f9fdaa9733d36f4e9121c2d3a90ec9cc1765160961` |

### 3.28 TA-W03/04/06 controls for gaps in the generated inventory

CHECK-6/19/24/25/26. Use the precise gaps in §3.27 to select controls through
the existing policy, binding, selector, conflict verifier and recovery owners.
Keep the candidate test-only and retain the existing critical-file scope and
thresholds. The standalone proof verifier receives independently signed round
inputs. Scheduled fallback uses the existing controlled loopback, observes a
pending future, drops it without another poll, then checks the original
deadline and retry floor through a completed resumption.

| Stage and skill | Requirement and action | Checkpoint exit |
| --- | --- | --- |
| Planning and preflight with `qa-mutation-testing`, `qa-regression-selection`, `qa-environment-preflight` | Distinguish useful controls for uncovered behavior from generated mutant outcomes. Review the work and durable storage admission before any campaign. | Keep campaign admission separate. Allow 420 seconds total for selected checks and any causal repair here. |
| Test construction with `implement-rust-contracts`, `qa-contract-tests`, `qa-boundary-edge-cases`, `qa-state-concurrency-lifecycle` and `qa-golden-fixture-parity` | Check invalid limits independently, duplicate aliases through the legacy constructor, one extra eligible peer before entropy selection, exact duplicate-link cause with an otherwise authentic proof, and cancellation of a pending wait before retry or round consumption. | Exact healthy and refusing cases pass. Independent workers own the proof and waiter fixtures in disjoint source checkouts. |
| Candidate freeze and closeout with `qa-subagent-dispatch`, `qa-independent-review` and `qa-context-checkpoint` | Run both exact test binaries for ordinary GNU behavior and the 90% line gate. Re-list all ten owners after the test-source changes and compare every descriptor with §3.27. | Retain fresh source, tool and vendor bookends. Zero mutated builds or outcomes in this checkpoint. Native and 95% mutation qualification stay open. |

The 826-input product map is
`b90d3cef1aff95a34c5d4637c22c2c491ed6ed3808d79a3f4d542f815a80457b`.
Relative to §3.27, five Rust inputs changed, all in tests. The ordinary GNU
run admitted the same dependency graph and executed all 26 selected unit and
61 Roughtime integration cases without skips or failures. The selected
format, GNU Clippy, MSVC-target Clippy and isolation controls also passed.
This exact selected ordinary run took 101.16 seconds including startup,
admission and cleanup. A cargo-mutants baseline must still run under its own
admitted campaign command and account for its actual selected tests.

Seven inventory controls, five line-analysis controls and 67 shared wire
controls passed. The instrumented replay ran the same 87 cases in 95.19
seconds. All ten per-file thresholds passed. The changed line counts were
230/233 for Retry, 190/205 for Selection, 327/341 for Recovery and 181/192
for Interval. TCP remained the lowest result at 318/353 (90.08%). The ten
listing commands then passed in 39.11 seconds. All 601 descriptors retained
the same file, span, replacement, kind and per-file digest. Four Retry diffs
display updated test imports as surrounding context. Their actual edit lines
remain identical. The fresh receipt binds all 835 inputs and the same
29,754-file vendor map. The checks took approximately 237 seconds combined,
within the 420-second allowance.

An independent fresh-context reviewer matched all ten initial changed paths
to the frozen checkout and traced the five controls through their owners. The
17-peer case rejects before entropy is queried. The last repeated link remains
valid against the immediately preceding response, and each exchange separately
passes keyed verification. The dropped waiter retains the original floor and
slot and observes quiet sockets before the fresh nine-reply fallback. This
uses real monotonic time, so the initial one-second retry-floor observation
remains sensitive to host stalls. The test observes application effects and
does not measure a detached timer or provider resource with no such effects.

For the next campaign admission, the previous Engine work floor is 7 GiB at
startup and the durable supervisor reserves 5 GiB. The empty private tmpfs
probe observed 8,690,761,728 free bytes. The actual local durable ext4 evidence
path had 897,077,248 free bytes. The tmpfs probe establishes scratch capacity
only. It cannot satisfy the durable evidence reserve. Re-enter on independently
owned durable capacity and a reviewed private rule for one whole-file cohort.
The eight-descriptor request encoder is the smallest complete first cohort.
Keep all ten files in the full gate and retain one aggregate time/resource
budget when considering the subsequent cohorts. No public campaign opens.

| Retained evidence under `~/.local/share/nevrotick/roughtime-guard-controls.1ttwbzwv/` | SHA-256 |
| --- | --- |
| `baseline.py` | `fcbf4edb2d86110d4eb8a6674772f073a44df1cc0e9b26dab56f6f8af84924d5` |
| `ordinary.json` | `70e99954e684c80e15904d43741070456d285382b890560a7d6e6e9ae71f1ae0` |
| `engine_roughtime_inventory.log` | `8d8722d799e66103ee632bf031c5fc5d460aa3111c89882876b424469755a189` |
| `engine_roughtime_line_analysis.log` | `58b4d9eb44aad1a09eb5e592d39a7ddd513e13e4a1683a0188871d505d90f68b` |
| `verify_wire_analysis.log` | `320dab59311ff4236f08b715a89cae093a0df5fcc295220c483b49150252524b` |
| `coverage/verification.json` | `3288bfcae15ebdc73bd62577cfa703389cfa7d990461286c7c6ea6a20cd64089` |
| `coverage/coverage.json` | `4187fb275d59ef79be196ac7b37877dea722747da55189e4fa7180e0a13c030b` |
| `enumerate/verification.json` | `e2538e519d7dcc0d5aad692bd2db0ef2fab167a20a48e5fbe5c5f620f0150f08` |
| `resource-preflight.log` | `d494083c4af54d35e6adc688233f38fa70cb00a45ddc7a46b9661af32578398c` |

The `rgt_proof_control` and `rgt_waiter_control` workers returned only their
assigned test patches. Their original patches are retained as `proof.patch`
and `waiter.patch`. The independent `rgt_guard_exit_review` worker inspected
the frozen paths, complete reached controls and passing ordinary receipt.
Coverage and listing completed after the reviewer's readback. The coordinator
compared both against the exact frozen inputs, launch hashes and selected
outcomes. Both worker checkouts and the temporary freeze were retired.

### 3.29 TA-W06 public roster cardinality controls

CHECK-6/19/24/25/26 and §3.21. The follow-up audit of `c97041d` flagged both
adjacent invalid roster sizes. The §3.28 selector control covered 17 peers
internally. Extend it to two peers and require no entropy calls at either
size while retaining the 16-peer final-schedule control. Exercise the public
recovery constructor with each otherwise eligible roster. Require
`Configuration`, quiet peer sockets, and admission of a three-peer round
with the same original control after each refusal. A binding with a due
first failure remains eligible. Its next actual failure must still produce
the independently specified 1.5-second wait after both refusal sequences.

Apply `qa-boundary-edge-cases`, `qa-contract-tests`, `qa-regression-selection`
and the §3.28 admission and review route. The checkpoint uses a 420-second
total allowance for the exact selected ordinary and instrumented runs,
focused checker controls and complete listing. The source changes add only
test behavior and rebind the checked suite. Existing qualification holds
and thresholds apply.

The 826-input product map is
`22478879ca1d903d01c8e6498844bfa07cce8622687031d95328db994d22fb8a`.
Only the private selector test and the public integration test file changed
in the product map from §3.28. The exact ordinary GNU suite ran 26 unit and
62 integration tests in 98.00 seconds including admission and retirement.
The isolation and format controls and both GNU and MSVC-target Clippy lanes
passed. The fresh reviewer `roster_final_review` traced the guarded call and
independently recomputed the frozen product and roster bindings, then read the
passing ordinary receipt. The public test observes actual fixture sockets and
an active three-peer round after each refusal. The zero-draw oracle is in the
same selector called by the constructor. The retry progression observation
has the precision of the actual exchange's monotonic capture bracket.

The same 88 cases passed the instrumented GNU run in 94.63 seconds.
All ten 90% line gates passed. Selection moved from 190/205 to 192/206
executable lines. The other nine counts match §3.28. Seven inventory, five
line-analysis and 67 shared wire controls passed.

The complete inventory replay took 39.25 seconds and returned all 601 raw
descriptors byte-for-byte at the per-record level, including displayed diffs.
The coordinator matched all 835 input digests to the frozen tree and live
checkout and verified the line and inventory export bytes. The selected
checks took approximately 233 seconds combined. Mutation and Windows native
qualification remain separate and open.

| Evidence under `~/.local/share/nevrotick/roughtime-roster-controls.6L93PMB6/` | SHA-256 |
| --- | --- |
| `baseline.py` | `40100c09df71d81b7d10ac847fb5a68ae8280136b1807100cbf1cea77b74755a` |
| `ordinary.json` | `6a14aa5a8b871a1e9ccfaf7c2212bd28e24dc8cce8177ff4c5c58aba65079bd5` |
| `engine_roughtime_inventory.log` | `adec287eae8a3c3e056be251c8ffed89f4561520ea0ff9712dc8c6117c076b78` |
| `engine_roughtime_line_analysis.log` | `2c3ae7e86106e123c52d89dec82d9605aff42d9a9caac31cafdda07032128bbb` |
| `verify_wire_analysis.log` | `71bae17d8b65e41d6461f8ab55756f64ae935147e1742a5fc1703ac88f5ec375` |
| `coverage/verification.json` | `3b9bd03c81862300b5d949030bc04d384022a582746445dd0d0f87d11b7ff6ba` |
| `coverage/coverage.json` | `9c88e25115b2fca9cb7ba22a318eb6176f9fc2579762361f60b2ec647e51d299` |
| `enumerate/verification.json` | `6ffd9a6092cdcd91f44602dbf810725dc54ec445a2bad5940026680c8e0ad040` |

### 3.30 TA-W06 partial TCP request writes and retirement

CHECK-6/19/24/25/26. Exercise §3.27's remaining partial-request boundary in
the actual TCP `poll` path. Use a private per-connection fixture cap after
the same remaining-slice calculation used without the fixture. Observe each
native send's byte progress through the loopback peer, using the published
fixed request layout and the original prepared nonce. Verify that each
partial write keeps the attempt in `Preparing`, and the final write alone
opens `Awaiting`. Check original cutoff, first and last send capture, and
connection retirement before publication of the signed healthy response.
The withdrawal neighbor must observe prefix only, `Withdrawn(Stop)`, peer EOF
and a permanently spent original attempt. Its pending owner remains live
while the peer observes EOF.

Apply `implement-rust-contracts`, `qa-integration-tests`,
`qa-state-concurrency-lifecycle`, `qa-regression-selection` and the §3.29
admission/review path. Allow 420 seconds total for selected ordinary and
instrumented runs, checker tests, complete re-enumeration and any repair. The
fixture forces partial request progress by varying offered buffer extent at
the real socket. Native scheduling and platform short returns for a larger
offer remain separate observations, as does Winsock/provider retirement.
OPEN-8 and Windows trust/date authorization retain their existing owners.

The first frozen map
`42f95b454bcbc2761558506c603b3dc3dfd29324e81054f925ec1716a072430e`
passed the ordinary selection and checker controls. Fresh independent review
found that `Spent` on the pending exchange alone could hide reopening of its
shared authority. The coordinator interrupted the first coverage attempt at
29.08 seconds before a passing result. The final fixture separately checks
the shared `Spent` phase after publication and the latched `Withdrawn(Stop)`
phase after closure and the later spent poll. Independent repair readback
confirmed these assertions only observe the original authority, and the
production path matches the first freeze.

The final 827-input product map is
`49ee4580b42dbbcf1795953909932b27be2458690dfe0ac44c7fcede519a0bfe`.
The exact GNU suite executed 28 unit and 62 integration cases, including
both partial-write controls. Format, isolation, GNU Clippy and MSVC-target
Clippy passed. The ordinary run took 99.24 seconds including the admission
and retirement steps. The peer saw sixteen 63-byte writes followed by 28
bytes in the healthy case. The withdrawal case saw one 63-byte prefix
followed by EOF. Test capture inequalities concern the userspace write
brackets. Cutoff equality in this case does not qualify expiry during a
native write or a later provider cleanup.

The final instrumented GNU run took 94.72 seconds and executed the same
90 cases. All ten line gates passed. TCP covered 324 of 357 lines, compared
with 318 of 353 in §3.29. The other nine owner counts are unchanged. Seven
inventory, five line-analysis and 67 shared wire controls passed on the
final freeze.

The final listing took 39.45 seconds. The total remains 601 descriptors
over all ten files. Nine files reproduce the raw §3.29 descriptors exactly.
All 66 TCP entries match the prior owners, operations, replacements and source
spans after mapping moved lines. The test cap generated no additional target.
The coordinator verified all 836 listing input digests and both exported
artifact sets. Approximately 358 seconds were consumed across the two attempts
including the interruption, within the original 420-second allowance.

| Evidence under `~/.local/share/nevrotick/roughtime-partial-write.23fq7of7/` | SHA-256 |
| --- | --- |
| `baseline.py` | `dcd465c1a1fa1f07a3ccc544fd5cb0833ad16aac921ddb01d62106c15d487394` |
| `ordinary.json` | `560a84ac71cbbdde020b8b0e55620c5cf34ee3aadb10bb6300e52e5bf1a87c6d` |
| `coverage.log` | `0b79d47ead56c5f94122491bffc326a6559b04a665d41c9f8c41b6c748d65504` |
| `predecessor-fixture.rs` | `95abd2d8b7b1f1f0064dce698e505ec0d7c11266c9b2386b45ba21eaab316a24` |
| `final/ordinary.json` | `889c1f450d83e51e40d661fa4fa7d06db793243a43bd4aad9b00e9d7269230ed` |
| `final/engine_roughtime_inventory.log` | `9b6dd6edda6fef78b8fb23e1c56ffef41c768eaa0e87b18dcc7d359cf16fbc50` |
| `final/engine_roughtime_line_analysis.log` | `58b4d9eb44aad1a09eb5e592d39a7ddd513e13e4a1683a0188871d505d90f68b` |
| `final/verify_wire_analysis.log` | `3c44911f3041ce5aa77fd63038dace0df4a8f19ad32678c9d6e153f5a34604d5` |
| `final/coverage/verification.json` | `b69cd99fd0df1fbff5fa259750bad6f9baf1f40f22e77e317c5893c9aadace7b` |
| `final/coverage/coverage.json` | `14a1952190dddec996d9c36e1a0e2130bd3f08c348e9aeae955aa465ca647df9` |
| `final/enumerate/verification.json` | `874cfe3d2b7fdc5ed4838b7c5c941a53e770cc3809d8333d559414464d081486` |

### 3.31 TA-W02/03/04/06 follow-up fuzz targets

The sleep hold ended. One bounded priority-1 wire-only experiment completed for
the staged index tree `3e2b17d62d780119eea3677f0768f99d3ad195c0` and admitted
product source map
`17c69e3b30cf23b9f0a936dfe3f7cc68e0e0d92fae694af73de849c89df7c6ad`.
This task-only status update changes neither that admitted source map nor the
executed product bytes. Maintained selections for priorities 2–4 remain open. Keep the existing AES
hold and native Windows qualifications. Treat packet bytes, root selections,
structured evidence and test inputs as untrusted regardless of their origin.
Use local fixtures and preserve each real admission guard.

| Priority and owner | Target and seed inputs | Independent checks |
| --- | --- | --- |
| 1. TA-W02/03 wire admission | Maintain the existing `policy/roughtime-wire-fuzz-target.rs` experiment. Exercise request and response decoders and the TCP declared-length reader with separately attributed corpora. Keep accepted maps with unknown fields, official and corrected Appendix B specimens with provenance, and boundary neighbors for lengths, offsets and Merkle paths. | Admitted fields and original byte spans agree with an independent fixture reader. Legal cap-sized frames remain admissible, and oversized or structurally invalid frames refuse within the declared work bounds. Record accepted decode and header outcomes separately. |
| 2. TA-W03 keyed verification | Drive `RootKey::from_bytes`, `PendingRequest::new` and `PendingRequest::verify` with independently checked local signed vectors. Exercise both raw input variation and structured variations that can reach deeper verification. Retain healthy shallow and deep proofs and the published legacy-context refusal. | Accepted evidence retains the exact selected root, original framed exchange and independently specified signed bounds. Distinguish request identity, delegated key, response signature and Merkle inclusion outcomes. A structurally valid frame alone earns no keyed-verifier credit. |
| 3. TA-W04 conflict evidence | Guide bounded traces through the real round and conflict receiver. Seed independently signed multi-peer agreements and contradictions. Cover pair selection, response ancestry and repeated link randomness. Add a raw evidence target only once DATA-18 has an actual receiving decoder. | An independent transcript model computes the first strict causal contradiction and retained positions. An invalid prefix cannot become conflict proof, clear a conflict or admit a date. Preservation of a previously recorded proof stays observable after the round retires. |
| 4. TA-W06 selection and recovery | Guide bounded action sequences through public selected-roster and recovery owners with controlled fixtures. Include ready and deferred peers, distinct group constraints, the 2/3/16/17 roster neighbors, UDP silence, partial TCP progress, withdrawal, expiry and join. | Compare each observed transition with an independent state model. Record refusal, actual dispatch, retained retry debt, first conflict, admission and join as separate observations. Preserve one-use authority, no receipt after withdrawal, and no spent work being refunded. |

Start with `qa-structured-fuzzing` and the existing wire adapter. The current
wire analysis selector admits Roughtime coverage and enumeration. A maintained
fuzz selection requires a reviewed runner role, exact tool/graph/source
admission, and an accepted input plus an intended failure control through the
actual target. Bind the corpus provenance and digests. Set finite input,
execution, worker, memory and artifact limits before running. Record callback,
acceptance and reached-check counts separately. Use `qa-state-guided-fuzzing` for
the later trace targets only after qualifying the driver, independent state
model, fresh state reset and equal-budget comparison with the baseline. Retain
original and minimized witnesses privately with exact replay identity. Coverage,
mutation scores, actual Winsock/provider retirement and production Roughtime-to-NTS
qualification retain their own evidence.

The private result
`~/.local/share/nevrotick/roughtime-wire-fuzz-current-20261128/result.json`
(SHA-256
`db018d3b977215fd7fb0a8a47605ea772e4810a5531e9586f749d9714ef1b694`)
binds the admitted inputs, three initial seeds, tools, offline graph, distinct
healthy/control binaries and closing bookends. Fixed valid-request and
valid-response executions passed; their own output correctly says that these
were fixed replays, not fuzzing. The seeded assertion control exited 77 with its
expected marker, establishing accepted-input delivery and failure propagation,
not ASan detector operation. The healthy guided row completed 256 callbacks
with seed `2733597436`, final `cov: 139`, `ft: 180`, `corp: 7/4134b`, maximum
input length 1,036 bytes and final RSS 38 MiB. These are finite libFuzzer
feedback values, not line coverage or a coverage percentage. The complete
bounded role took 250.445 seconds and reported no unexpected target failure.

This result is a current bounded wire experiment, not the maintained fuzz
selection required above. The temporary workspace was removed after recording
the starting seeds and final counters, so the evolved seven-member corpus and
its member digests were not retained. The receipt also lacks an artifact
inventory and per-decoder accepted, refused and reached-check counts. Before the
next maintained run, the reviewed runner must copy a bounded final corpus and
any original and minimized failure artifacts to a private durable directory on
both success and failure, record their SHA-256 manifest and replay recipe, and
create durable files with mode `0600` under a `0700` owner directory. It must
also record the full source/tree identity and separate request, response and
header outcomes. Priorities 2–4, a representative ASan detector control,
native Windows behavior, mutation gates and production Roughtime-to-NTS remain
open; no universal malicious-input assurance follows from this experiment.

**Keyed-verifier exploratory run and RSS triage — 28 November 2026.** A
separate external, bounded semantic driver exercised the public
`RootKey::from_bytes`, `PendingRequest::new` and `PendingRequest::verify`
chain against the staged index tree
`acce28e78472e8e708e2bba755f0d4c1985b57b9` at HEAD
`172d03517123e6c4947fc94074b9b15a17dddc61`. It used independently checked
local signed shallow, depth-two and depth-32 vectors, plus bounded mutations
for nonce, response signature, Merkle commitment and truncation. All ten fixed
seed replays passed. A separate seeded assertion control exited 77 with its
expected marker. That establishes input forwarding for this temporary target,
not an ASan detector result. A fresh-child LSan-enabled run preserved
`ASAN_OPTIONS=detect_leaks=1` and the admitted 8,192-run, 64-byte, two-second,
512 MiB RSS and 256 MiB allocation limits. It completed with `cov: 1019`,
`ft: 2212`, `corp: 148/2172b`, fuzzer RSS 50 MiB and a child peak of
52,256 KiB, with no unexpected sanitizer diagnostic. The raw log is
`~/.local/share/nevrotick/roughtime-keyed-fuzz-run.Z3BVkx/guided-8192-lsan-fresh-child.log`
(SHA-256 `c9de04446379811a0ce2403a38bda4466ad61fe7190d240646aaa512a3adfa30`).

An earlier direct LSan launch stopped at 2,159 cases via libFuzzer's RSS guard
and emitted one 11-byte artifact. Its fresh-child LSan replay executed the
artifact once and exited zero at a 32,888 KiB child peak. Its log SHA-256 is
`7f9ad8c937befd43f5792d041a3b060976531d41615d0fc3c04b1d0a6c92bf6d`.
The retained triage control reproduced Linux `RUSAGE_SELF` high-water retention
across `exec` (1,287,988 KiB before and after `exec`) while a fresh child
measured 11,536 KiB. The source of the stop is therefore a temporary-runner
measurement artifact, not a demonstrated target failure. The original artifact,
replay and fresh-child log are retained in the private directory named above.
The classification is in `rss-triage.json` (SHA-256
`905cd2333d1ebca5e9f80222721acaef79480bd9272cadb4443b53422ce71732`).
Its `SHA256SUMS` binds 220 retained files, including both binaries, the fixed
seeds, the final on-disk corpus and the temporary driver inputs. The manifest
SHA-256 is `713a2efbe0ac488188ef7b492ed0cea8b43430cf02a967ba6cbaacb82c1a2ace`.
The private Linux copy matched the working files byte for byte.
The future reviewed keyed-verifier runner must launch each libFuzzer target in
a fresh child, retain the child-specific peak RSS and raw log, preserve ASan or
LSan settings and every admitted resource flag, and replay any RSS-limit
artifact in that same mode before classifying it. It must not weaken the
512 MiB RSS limit or disable leak detection. This exploratory driver is not a
maintained fuzz selection, raw-wire parser coverage, native transport evidence,
leak-freedom qualification or a claim that all hostile inputs are safe.

### 3.32 TA-W03 request-encoder cohort resource and input preflight

CHECK-6/19/24/25/26. The first private whole-file candidate remains the eight
generated descriptors for `request.rs`. Apply `qa-environment-preflight` and
`qa-mutation-testing` before building its guarded execution role. Keep the
seven-GiB work floor, separate five-GiB durable receipt floor, one mutation
worker and 95% viable-kill gate. The full ten-file scope remains required.

The current local ext4 and shared tmpfs paths were below their applicable
floors. The paired canary used a fresh namespace-private tmpfs for work and
a separately owned ext4 receipt image backed by the established C: lab
directory. After each wrote and read back a small fixed file, work had
8,690,757,632 bytes free and durable receipts had 9,927,118,848 bytes free.
Read-only remount preserved the exact receipt and refused a new file. The
5.440-second canary retired its image, mount, daemon and transient service.
These are storage technique observations for the successful path. The
campaign supervisor still needs failure and cancellation recovery. The
actual source and tool copy, resource lease and baseline need admission
under the private role.

The candidate bookend matches the working bytes and index blobs of all
837 distinct inputs across the retained ordinary, coverage and listing
receipts. The retained GNU ordinary result covers 28 unit and 62 integration
cases. The inventory reports 601 descriptors over ten owners. The eight
request descriptors retain the checked descriptor digest and a separate
binding over the rendered diffs. The first binder wrongly treated the
835- and 836-input maps as nested. The next readback distinguished the
existing descriptor digest from the separate diff binding. The final
receipt records both corrections and the exact verified source identities.

Re-entry requires a reviewed private singleton role that binds the actual
`roughtime-verification` GNU commands for discovery, baseline and campaign.
It must recheck its live work and receipt floors, run the ordinary baseline
under its actual test command and account for all eight generated identities.
Keep compiler-unviable, killed, survived, timed-out and incomplete results
separate. No mutation ran in this preflight and the public inventory policy
remains listing-only.

| Evidence under `~/.local/share/nevrotick/roughtime-request-readiness.hXG9ZOPp/` | SHA-256 |
| --- | --- |
| `paired-storage.py` | `6fcd17c19107010181dc1ee86ada0cc8fb198406fead72b698cec524f4d63e4b` |
| `paired-storage.json` | `95bd4773e664bdf97a1e05f94f8faa8639f625c4068c86efec94083a31bd0f37` |
| `paired-storage.unit.log` | `0b937a668602b0566b6bab0ffdfefd9e2e809a60607c35e30c6ca642d9862160` |
| `bind-request.py` | `835dc00bbea6f8aaf3c82f0216a16bed6b49d7d01f6167364d473e0b3ff227bd` |
| `request-cohort.json` | `2b660456ba9eeeab5f8f32b495de82ff01e025d354cd2d3a236b31472bd44ee7` |

### 3.33 TA-W03 private request-encoder command and result controls

CHECK-6/19/24/25/26. The private command capsule binds the cohort from §3.32
to the exact GNU target, Engine feature, request file, tool and all eight
descriptors and rendered diffs. It requires both Cargo test binaries and
the original 28 unit and 62 integration case names. Eight fixed command
checks passed, including changed target,
missing or repeated case, skipped baseline, foreign listing and diff drift.
The pinned 27.1.0 help confirmed the shared Cargo argument option. A
retained same-version GNU jobserver control supports one implicit Cargo
slot with zero extra tokens. Observe the combined Cargo phase vectors in
this cohort's own baseline before admitting them as runtime evidence.

The result reader consumes the same command capsule. Eleven fixed Python
controls passed, including foreign command, same-count wrong roster,
failed baseline, unviable mutant, caught mutant, survivor, timeout and
incomplete evidence. A counted native label still needs its causal log
and exact phase observation. The reader cannot award a mutation score.

The pinned Cargo 1.100.0 ran both phase vectors on a dependency-free
workspace with smaller test rosters. Artifacts for the two binaries shared
the expected package, source and GNU output paths across phases. The test
phase forwarded both case names, quoting the name containing `::` in its
verbose display. Each binary ran its intended case. The first probe was
stopped by an LLVM linker thread creation failure under the 24-task
canary cap. The follow-up used the existing 128-task runtime profile for
the fixture. Retain both observations. The fixture supplied Cargo output
shape evidence for the baseline reader.

Twenty baseline-reader controls passed. The reader binds both phase
banners, both matched Cargo compiler artifacts, their test invocations,
the exact named passing cases and the 283/0 filter counts. The retained
ordinary product output confirms the roster and filter counts. The direct
Cargo fixture confirms the artifact paths and argument decoding. The
combined cargo-mutants transcript with the product roster remains unobserved.

The configured Linux runtime ran two harmless probes in separate short-lived
user services. The first resolved the planned nested tool path, refused writes
to the tool and sealed source, kept the checkout out of the inner namespace
and read back the expected private worker output. The second observed a
detached child's held lease, issued the runtime's resource stop and observed
lease release before the outer supervisor exited. The service units and
cgroups were retired. These observations qualify the two probe routes under
their canary limits. They supply no Cargo, compiler or campaign result.

Keep the public inventory role listing-only. Re-entry needs a reviewed
private role in the existing execution gate, a supervisor that holds the
original work and durable receipt identities through abnormal stop, and
a fresh live reserve check. Before allowing the campaign, rehearse the
two exact Cargo vectors on the unmodified frozen product, observing both
complete rosters. The pinned mutation tool has no baseline-only switch.
Use its normal `--baseline run` in the later campaign and independently
bind the forwarded phases, native results and original logs. If the
campaign executes, account for all eight descriptors and review the cause
of each outcome before calculating the viable denominator. The ten-file
requirement and 95% per-file threshold remain. The bounded priority-1 wire
experiment is recorded in §3.31; its maintained corpus/artifact replay and
priorities 2–4 remain open.

| Private evidence under `~/.local/share/nevrotick/` | SHA-256 |
| --- | --- |
| `roughtime-request-command.H2yfonha/command_contract.py` | `1a760f59bc32f76b491a03a9daf164c7045144e518780b3b8da0833732149f44` |
| `roughtime-request-command.H2yfonha/command-contract.json` | `46ca89b7ce1cb3b7f4c099e818e3b9da7e0ac1acf073c8435b654b4962637510` |
| `roughtime-request-command.H2yfonha/test_command_contract.py` | `ac3c2b0652f71f42210b694e8a5a0ecae3a09aaa5cc5edd7bb74c5a0a9fcd9e6` |
| `roughtime-request-command.H2yfonha/python-controls.log` | `a9435520f84e2e342f148a210059c33fb6db7b5cd9cc5ce6061cacccf17504a1` |
| `roughtime-request-command.H2yfonha/mutants-help.stdout` | `cc1de762d004b8b74204bf940705c1142f5515b35841ba5c8df5b9863216914e` |
| `roughtime-request-command.H2yfonha/runtime-envelope.py` | `d3fd141d150546c4053e5b92e0ae07c5e6a73719cdf779da9b76726c9555b0e8` |
| `roughtime-request-command.H2yfonha/runtime-envelope.log` | `e8556bf6560bfc9b5277ee0a7570e13adb78a3df112b5321afba846f5b27fd32` |
| `roughtime-request-command.H2yfonha/runtime-stop.py` | `ed3492f8fefc6feaf145b21c0ca701dc1cdf829cce36fd3233093a4bf1993dd0` |
| `roughtime-request-command.H2yfonha/runtime-stop.log` | `140da5d0c9bf129001deaab1a699ca925d30a9e6f3e6e76f0165de210f704605` |
| `roughtime-request-command.H2yfonha/cargo-dialect-low-tasks.py` | `d3be7b2cdf9521e3a1f7877ad76b5dbfeb44c3173f28434116c1c93ad2e058bc` |
| `roughtime-request-command.H2yfonha/cargo-dialect-low-tasks.log` | `d3376fb133583db06f0005b553bf83f6a9ad5922600a35caa8c89d68226b48e8` |
| `roughtime-request-command.H2yfonha/cargo-dialect.py` | `c12a6786e966979210f19aaf312586f3ea7241e289e59539e04b832bbf1508d7` |
| `roughtime-request-command.H2yfonha/cargo-dialect.log` | `55785318e65526720e2c0eb98ba5b5d7817083309e25123b992a06f8de4fd2ec` |
| `roughtime-request-result.UjtBBn9E/request_result.py` | `01d079e8622935f41500cc5145a90146e0eb56002baae89a72239b22550ca31f` |
| `roughtime-request-result.UjtBBn9E/test_request_result.py` | `799ce292adcc6087de9206910942c1e533c67b934e5aad7b5754e20181874dfb` |
| `roughtime-request-result.UjtBBn9E/python-command-controls.log` | `6468dfafde9e4fbdd1d2a87306f1671664a5ad3571a9afd4ab16798febe98ffb` |
| `roughtime-request-baseline.sZ02mgQU/baseline_observer.py` | `e06b823b7d59df820e6fd90a8f1945b6b3184fe57b858a99cd3b81f95f61b6e1` |
| `roughtime-request-baseline.sZ02mgQU/test_baseline_observer.py` | `7da7549d12d07c9469ad87e771a2c66e7f12fb0f32c67bf4cfbb387159851549` |
| `roughtime-request-baseline.sZ02mgQU/python-controls.log` | `bc5c73769caae7bcf9dc7fe8176720cf2b56991d7a8974db14d06be877639235` |
| `roughtime-request-baseline.sZ02mgQU/reference-readback.json` | `75703119399291da878d213afb422cc1a98d8f90df23e36ff0b670bbc9f7e1a3` |

### 3.34 TA-W03 exact unmodified request-encoder rehearsal

CHECK-6/19/24/25/26. The two pinned direct Cargo commands passed on the
unmodified Engine Roughtime GNU candidate from §3.32. The selected Build
and Test phases exited zero and joined in 30.691 and 44.988 seconds.
The independent reader matched the same two compiler executables across
both phases and every named passing case in the original captures. The
unit binary passed 28 with 283 filtered. The integration binary passed
62 with zero filtered. Neither binary reported an ignored or failed
selected case. Eight fixed observer controls passed.

The first admission stopped before Cargo. The 827-file product copy lacked
four manifests that the configured cooldown checker reads from the full
resolution context. The successful retry verified each supplemental
manifest against the staged blob, working file and policy digest. It
passed frozen-tree admission before opening storage and retained the
unchanged 827-file product map. Source, toolchain, target libraries,
vendor and graph identities matched after both phases.

The owner observed separate tmpfs work and ext4 receipt mounts throughout
the 101.347-second inner run. The lowest sampled free space was
8,323,293,184 bytes for work and 9,926,926,336 bytes for receipts. The
lowest sampled host memory availability was 6,998,364,160 bytes. The
worker service used the 6 GiB memory, 512 MiB swap and 128-task limits,
with no change in its monitored memory or task events. The supervisor
read back the fsynced original reports after a read-only remount, copied
and rehashed them, and retired the worker and disposable image.

The unmodified direct rehearsal prerequisite is satisfied. The next
private effect needs source-reviewed one-shot admission under the existing
isolated runtime, a fresh listing of the exact eight descriptors and
rendered diffs, and live resource and receipt custody. The later
cargo-mutants run still needs its own baseline and causal observation of
each disposition before scoring. The public inventory remains listing-only.

| Private evidence under `~/.local/share/nevrotick/` | SHA-256 |
| --- | --- |
| `roughtime-request-rehearsal.pg6CmaqQ/reports/native.json` | `35e7c63c0ab1e0970fc5af87938b537f374bcaf84071363f552799d96f22252b` |
| `roughtime-request-rehearsal.pg6CmaqQ/supervisor.json` | `5d75167ecb4dbce32e85f25e8f430596b23db7098992ad02cc6c128e2daba446` |
| `roughtime-request-rehearsal.sPDrbImi/launch.py` | `bf26722b4ac4072ec9bb192bddce12b5f8c360f702aea6cdcc1831ea191e974b` |
| `roughtime-request-rehearsal.sPDrbImi/rehearsal.py` | `8cfe399f61084d2302c3e6e8e239e0e2f95b1b96f5bb0217e8da34d83d42bed7` |
| `roughtime-request-rehearsal.sPDrbImi/admit_rehearsal.py` | `fc90ac1a0dd355ab0d0f74e45b02ef348ee9edf6577febda813afce7a5d75df2` |
| `roughtime-request-rehearsal.sPDrbImi/frozen-inputs.json` | `327e97ae49f8361fd9f986035555b8d2fac24d268bb5498e951cebc4380ee6ec` |
| `roughtime-request-rehearsal.sPDrbImi/supervisor.json` | `58125901338d5d71dad952eadd083b3ff7aad5ca92863a4d211c9b1bb41815c0` |
| `roughtime-request-rehearsal.sPDrbImi/reports/native.json` | `7792fdc7ef9390a691eee26c8fc84c6a3204f83dc76c3794ca1009ed229aaafc` |
| `roughtime-request-rehearsal.sPDrbImi/reports/build.log` | `a762d9cc98e713ce32a914256ec3b0cca6a2b5265fd76a28e6ea9d6944207f73` |
| `roughtime-request-rehearsal.sPDrbImi/reports/test.log` | `d476cfe323d100cd707ba9f8f010eff13d3e928224e41a5f2335e7258ee59adb` |
| `roughtime-request-rehearsal.sPDrbImi/rehearsal-receipt.json` | `98ab13c7e8067ed496a72152ace0bcc6a1e3b68cd44c5183ecaae77753c26a46` |
| `roughtime-request-baseline.sZ02mgQU/direct_baseline.py` | `5fa9b1833e5bfcd2166f2217320eea445539844ac686059d3d1020989f70b1a9` |
| `roughtime-request-baseline.sZ02mgQU/direct-python-controls.log` | `5614057e33da8ce7d011ed5d703556292d8f73e0b155e9fc339c3554ee925105` |

### 3.35 TA-W03 request-encoder per-file mutation result

CHECK-6/19/24/25/26. The private GNU `roughtime-verification` campaign
met the 95% per-file mutation gate for `request.rs`. The selected
source identity is
`49ee4580b42dbbcf1795953909932b27be2458690dfe0ac44c7fcede519a0bfe`.
All eight descriptors and their rendered diffs matched the pinned
inventory before dispatch. The fresh cargo-mutants baseline used the
reviewed Build and Test vectors and passed all 90 named cases. The
unit binary passed 28 with 283 filtered. The integration binary passed
62 with zero filtered. Neither binary reported a selected failure or skip.

The original Cargo logs and diffs supplied a cause for every disposition.
Five variants compiled and were caught by selected tests. Three failed
at Build before any test could run. The viable score is **5/5 (100%)**,
with zero survivors, timeouts or equivalent exclusions.

| Encoder change | Count | Observed cause |
| --- | ---: | --- |
| Length addition replaced with multiplication | 1 caught | The UDP peer refused the emitted request profile. The TCP partial-write test observed a write of 1,071 bytes against its 1,036-byte expectation. |
| Server commitment or nonce boundary addition replaced with subtraction or multiplication | 4 caught | The selected TCP partial-write control reached the changed encoder and failed on its corresponding invalid slice bounds. |
| Length addition replaced with subtraction | 1 compiler-unviable | Rust reported `E0080` from constant subtraction overflow in the encoder. |
| Encoder body replaced with a constant array | 2 compiler-unviable | The checked lint policy denied the unused import, both arguments and four constants. |

The private gate spent its single dispatch before the campaign process
started. Eight fixed gate controls and eight fixed harvest controls passed.
The campaign exited zero in 174.075 seconds. Its original resource limits
held with no increase in memory or task events. The minimum sampled work
and durable receipt reserves were 8,225,984,512 and 9,927,098,368 bytes.
The supervisor read back and rehashed the receipts after a read-only
remount, then retired the worker and disposable image. Source, tools,
vendor and graph matched at the closing check. The campaign remained
inside the GNU fixture scope. The existing independent emitted-byte
oracle and signed receipt controls cover literal profile bytes and
padding that the generator did not change in this inventory.

The public mutation inventory remains listing-only. Nine Engine Roughtime
critical files still require their own reviewed denominators and per-file
scores. The bounded priority-1 wire experiment is recorded in §3.31; its
maintained corpus/artifact replay and priorities 2–4 remain open. Windows native,
production catalogue, DATA-18 and whole-interval NTS receiving qualification
remain open.

| Private evidence under `~/.local/share/nevrotick/` | SHA-256 |
| --- | --- |
| `roughtime-request-gate.YIKUdCHf/python-controls.log` | `ab5e7e1a7b7ce62dfb115c69b59bcce5a97eb9ff6f3718cfedd9ee8ddb1ad285` |
| `roughtime-request-harvest.T2KFxF5b/python-controls.log` | `9e6ecb8b676b5c0a1c1872a503ba2ca950fdf9449fec89aabb63b7de338de161` |
| `roughtime-request-campaign.fo5wNtcK/reviewed-inputs.json` | `0ec7a017ac76b9fafa33e3bd604cb1222051898b965f9203e0ab73953803b75c` |
| `roughtime-request-campaign.fo5wNtcK/reports/listing.stdout` | `7d3592c46a1d567901d56dbe27f89cae0109bf1ffa712cff0ecd5db5387aaac7` |
| `roughtime-request-campaign.fo5wNtcK/reports/dispatch.json` | `b80a7c31a7b61b75ac6109ff389a995bc4a1a9c4e46d3a09865c466be883ba06` |
| `roughtime-request-campaign.fo5wNtcK/reports/native.json` | `30b4106349f871950a170ab466dd48aaa27ef63728e381952c9b17febf4af86e` |
| `roughtime-request-campaign.fo5wNtcK/reports/campaign.stdout` | `10207302ff0525b063b0abf3a4713553674a161e6b4d5d99746c477604c1f47f` |
| `roughtime-request-campaign.fo5wNtcK/reports/mutants.out/outcomes.json` | `67419cc8ae5ebc93ceb9c5556136789b3bf84ce13837a4920fe31e9de8f85e5c` |
| `roughtime-request-campaign.fo5wNtcK/reports/mutants.out/log/baseline.log` | `9239fc51ce4af6b4be6c90ac7773ba18e85c94f9e98275aa375ba84f78ff5df9` |
| `roughtime-request-campaign.fo5wNtcK/supervisor.json` | `7afc7453cef6d431ecf8c404b702c4591a96fa8437ae441f3f12bba3398817e5` |
| `roughtime-request-campaign.fo5wNtcK/campaign-observation.json` | `3af327edf28483bba98aa1b7a130fefc25b3f53d3896eb9257ea07f7d0b4f451` |
| `roughtime-request-campaign.fo5wNtcK/verify_causes.py` | `dc66090c2983a7f8ab6b5232d3f5bd09175101bf30a5b886bc7130a99c259280` |
| `roughtime-request-campaign.fo5wNtcK/request-file-causal-review.json` | `df7d55ec90bc8e50b8850e03eb41776dd67fe3446691bc34bf98def743fb945e` |

### 3.36 TA-W04 conflict proof baseline and campaign boundary

CHECK-6/19/24/25/26. The next complete critical-file cohort is
`round/recovery/proof.rs`, with 26 descriptors at 19 distinct span starts.
The descriptor and rendered-diff readback binds the exact §3.35 source,
tool and vendor identities. The public inventory remains listing-only.

The private unmodified GNU rehearsal ran the two frozen Cargo commands
under `roughtime-verification`. The Build phase joined in 31.391 seconds.
The Test phase joined in 4.137 seconds. The independent reader matched
the actual executables to both compiler outputs and verified all eight
named passes. The unit binary passed four with 307 filtered. The integration
binary passed four with 58 filtered. Both reported zero failures or ignores.
The 90-case baseline and per-file line gates retain their earlier broader
scope on the same product source.

The focused unit tests exercise signed inputs, the exact first witness,
shape and identity failures, corrupted responses and missing, wrong or
repeated link values. Integration exercises public reconstruction of
two- and nine-receipt prefixes and readback after a timeout, a healthy
follow-up or a second conflict. These four public callers cover successful
reconstruction. Add a direct refusal through the public wrapper with
genuine signed receipts and a test-owned wrong claim to discriminate an
omitted `verify_prefix` call. That call has no individually generated
descriptor. An isolated malformed-request case and precise shape edges
remain additional coverage gaps. Strict causal comparison belongs to
`round.rs` and retains its own gate.

The observer passed 33 fixed synthetic controls, including a substituted
test with an unchanged count. The runtime guard refused an expired spawn
and a linked report in local controls. Both actual phases wrote and
fsynced their original intent before spawn. The isolated owner sampled
at least 8,318,840,832 bytes of free work storage and 9,926,934,528 bytes
of free durable storage. It finished in 61.417 seconds with no new memory
or task events. The supervisor rehashed all five reports on read-only
remount, observed cgroup retirement and retired the owned receipt image.

Use the focused roster for reviewing the next private single-worker role.
The 4.137-second test phase and the longest comparable warm build in
§3.35 give a rough 530-second whole-file estimate, including the observed
rehearsal overhead. This is a planning estimate. Keep the original
1,200-second aggregate effect limit, 90/60-second child limits, resource
reserves and one attempt. Adapt the private identity gate and artifact
reader to the exact 26 descriptors before dispatch. Scoring requires a
fresh cargo-mutants baseline and causal accounting of the original logs.
The proof file's 95% mutation gate remains open. Priority-3 conflict-evidence
fuzzing remains planned under §3.31; the bounded priority-1 wire experiment
does not cover it.

| Private evidence under `~/.local/share/nevrotick/` | SHA-256 |
| --- | --- |
| `roughtime-conflict-proof-readiness.gh12y_fb/proof-cohort.json` | `c5a5774b7f342abd164cd9daf0f1b20a8f2095dcadc779094c6af16317443892` |
| `roughtime-conflict-proof-readiness.gh12y_fb/proof-commands.json` | `b4488f1ca92c0e038043f770b96aa62ff1411e3c92691c3b7869268530a43b5f` |
| `roughtime-conflict-proof-readiness.gh12y_fb/launch_proof_rehearsal.py` | `6661c921f934f44b6bcd4ae08f912bb72c396d194228665ac1f9124d23fca0b0` |
| `roughtime-conflict-proof-readiness.gh12y_fb/proof_rehearsal.py` | `a38eb74765d6f606fb9d944a9dcbde1ed85ca250c1a6ebd3d6c47d78c91e1959` |
| `roughtime-conflict-proof-readiness.gh12y_fb/proof-rehearsal-controls.log` | `8cee40b1f63dcbffaa5a72ab88b596360b598e2f37920a467aef2ddff83cd0c9` |
| `roughtime-conflict-proof-baseline.bxYEoL8m/reader-receipt.json` | `66b7bcfd62a8a139abc85dd2b93bae28b73fd246c16324f7f8b5f5fe5d7d1416` |
| `roughtime-conflict-proof-readiness.gh12y_fb/reports/native.json` | `4e2ac73cd5f2c153f8637f006be2d7ab223c90f279cdc5b5b1237d981dc07056` |
| `roughtime-conflict-proof-readiness.gh12y_fb/supervisor.json` | `ac62fd0370ef54cc887be791e9e21c05ab943ebf508ab3e94eb0e58f69da212b` |
| `roughtime-conflict-proof-readiness.gh12y_fb/observe_proof_rehearsal.py` | `98f8c66135625645b961a1378f18b236c4414bd16b0326e22e9eaf1b76bd0a0a` |
| `roughtime-conflict-proof-readiness.gh12y_fb/proof-rehearsal-receipt.json` | `0f90eb48683d359ae7cc63448cc735365ee4952723027a8ce5909af972bea633` |

### 3.37 TA-W04 campaign preflight refusal after the host linker update

The first private conflict-proof campaign stopped in graph input admission
before fresh listing, Cargo or mutation. Ubuntu’s unattended Noble security
update installed Binutils `2.42-4ubuntu2.11` after the §3.36 rehearsal.
The previous Roughtime graph receipt binds the linker from `2.42-4ubuntu2.10`.
The refusal preserved the original reports and retired the process and
mount. The owned image remains with its refusal evidence. The attempt spent
preflight work only. It produced no mutation outcomes or score.

P0 DEV-01 verified the new linker against the Ubuntu-signed package index
and exact package archive. The [tooling record](../TOOLING.md#local-gnu-linker-pin-refresh--28-november-2026)
records the canonical pin and passing dependency-free controls.
The eight-case rehearsal in §3.36 retains its original toolchain identity.
Both affected dependency profiles passed all 30 lanes on an exact staged
projection. One of its 827 product inputs changed, `policy/tool-manifest.json`.
The refreshed product digest is
`bf7964d9c4ad856a4c503a1e141c90127c35d2a43fc6c0a3cc8707a9991a8fc8`.
The new graph receipts both bind that manifest and the authenticated linker.
The graph owner retired its worker and scratch before the closing check.

The line and inventory policies now bind the refreshed product digest.
The inventory binds the new exact line-policy bytes. Twelve policy controls
passed. Both thresholds, all ten critical files, the 90-case line roster and
the listing-only public role remain in force. The previous scores and baseline
retain their original tool identity. Renew the affected current baseline,
line and listing evidence before dispatching the private conflict-proof cohort.

The analysis-runtime regression suite also exposed two fixture defects.
The hand-built session lacked its Core scope. The host-access assertion
treated a directory created as a mount ancestor as evidence of host access.
The control now checks a real private host canary outside the mounted inputs.
Both repaired controls and the other 26 cases passed. The first run under
`/dev/shm` refused the fixture path before the affected children could start.

| Private evidence under `~/.local/share/nevrotick/` | SHA-256 |
| --- | --- |
| `roughtime-conflict-proof-campaign.sGJwJkGC/reports/native.json` | `3ec921d6cda15d657375eb0ef1b99c15266ca685e83da1f33096cba6fc2272e8` |
| `roughtime-conflict-proof-campaign.sGJwJkGC/supervisor.json` | `94d9e2104162885458b786623da124ba02a206d436fb6cb5eac3e4fed589be74` |
| `roughtime-conflict-proof-campaign.sGJwJkGC/campaign-observation.json` | `a46f31d91d503431be80ddaaa8bac747e6b1f5e172b051e15a00a7ba07cc7982` |
| `gnu-ld-refresh.iySMyPNg/graph-candidate.json` | `4ab1d080f64816604abc5f4264728a47758203f066354567aebcf9f414bf791f` |
| `gnu-ld-refresh.iySMyPNg/engine-roughtime-verification.json` | `9685284818fd35646b1c276c83b69e6c206de16cdecc66498f5910e7d3ffa9d0` |
| `gnu-ld-refresh.iySMyPNg/engine-ke-roughtime-verification.json` | `9e77a47df4399fc4448d8b86d165781599e52755c710416426993820f709c5cb` |
| `gnu-ld-refresh.iySMyPNg/graph-refresh.json` | `f1de8d2e0e1a3fffdb4da898c2d46019d7ebe2fc552b6c0232ffdb5bdb123533` |
| `gnu-ld-refresh.iySMyPNg/analysis-runtime-controls.log` | `28e13ab00d61e6e7ba54cdb6cd357a7e8e19a33ee6d2553e6d5a8d3005950d6d` |
| `gnu-ld-refresh.iySMyPNg/analysis-runtime-controls-current.log` | `594d530358c28ab95cc8c3058221ed36d27e2a44e60200d287200da1c0e633e1` |
| `gnu-ld-refresh.iySMyPNg/analysis-runtime-controls-repaired.log` | `ce0572212fe2846b26c9cd7d017d2b2a2b7ef5bfe96c5fe623bafc8647559e1e` |
| `gnu-ld-refresh.iySMyPNg/roughtime-policy-controls.log` | `a6f6d2bacc89430af974ad93fe311a56d19527c084705c56e40793b91fd13a70` |

### 3.38 TA-W03/04/06 renewed GNU metrics and proof cohort after linker refresh

CHECK-6/19/24/25/26. The configured GNU stable-core coverage and listing
lanes ran on the exact staged product from §3.37. The coverage run admitted
835 inputs. The listing admitted 836. Both retained the same vendor and
analysis tool identities as their predecessors. The changed inputs were
the linker manifest and the applicable candidate-bound policy files.

All 90 named instrumented tests passed. Every critical file cleared the
90% executable-line gate. TCP had the lowest result with 324 of 357 lines
(90.76%). Conflict proof reached 63 of 65 lines (96.92%), and the request
encoder reached all eight lines. These are GNU executable-line results.

The fresh listing reproduced all 601 descriptors and rendered diffs across
the same ten owners, including all 26 conflict-proof descriptors. The full
inventory still exceeds the 512-descriptor campaign reference. The public
launch executed only the ten listing commands and gives no mutation score.

The first outer launch refused a mistyped launcher hash before any tool
started. A later receipt reader looked for a standalone candidate field
that the configured results do not emit. The coverage run had already
completed. The corrected reader reused its original hashed export and
checked the complete input map and selected policy digest. Coverage ran
once. The two workers and private scratch were retired after their checks.

The refreshed product, line and listing inputs agree on the complete
837-file union. The successor proof cohort binds the staged blobs, present
working bytes and fresh listing. The four unit and four integration cases
and their exact direct Cargo vectors retain the same selection.
The successor uninstrumented rehearsal passed all eight cases with the new
linker and both refreshed graph receipts. It observed four unit passes with
307 filtered and four integration passes with 58 filtered, zero ignored
and zero failed. The Build and Test phases joined in 37.118 and 4.298
seconds. The observer matched both test executables across the original
and test phases. The owner completed in 59.845 seconds with at least
8,323,293,184 bytes of free work storage and 9,926,934,528 bytes of free
durable storage. It retained both phase intents and logs, observed actual
worker exit and read-only export, and retired the original image. The
reader rebinding passed eight identity controls and the inherited 33 fixed
transcript controls using synthetic successor inputs.

The private campaign preparation now pins the refreshed rehearsal and all
26 exact descriptors. Rebind the private execution gate and result reader
before dispatch. The proof campaign still needs its fresh cargo-mutants
baseline and per-file outcome accounting. Report survivors, timeouts and
compiler-unviable cases separately. The 95% mutation threshold and fuzzing
plan remain in force.

| Private evidence under `~/.local/share/nevrotick/gnu-ld-refresh.iySMyPNg/` | SHA-256 |
| --- | --- |
| `roughtime-metrics/coverage/verification.json` | `786ef8c98b7547b16217d4a3ac326afef2fe2112b40d811097a454ce0e81c135` |
| `roughtime-metrics/coverage/coverage.json` | `5730b9ac600ab066f392ea7b1d3918937f04fabc2fb18855ab1b34fb162ef55a` |
| `roughtime-metrics/enumerate/verification.json` | `2b3424188d892138ae76c5631efb602208a43ad92e466c6370898931a0c88726` |
| `roughtime-metrics/refresh.json` | `69d9e27ca6810d1baa5f83d7c3ed628586462c4bc3a86afbb5eb7c131aaa59f8` |
| `roughtime-metrics-launch.log` | `a82576c284c5cfbd8ebd3f3c05669016ece2ec653ef6a4c3c195eb468432fa89` |
| `roughtime-metrics-launch-current.log` | `6ae412bf6415c66984261f421be10e5abf1add732e3256d0674769a1a0dfe82f` |
| `proof-baseline-successor/binding.json` | `8be4087d35ff0f5a8289707767a40faa4048ea951438766c60b0425f868ce932` |
| `proof-baseline-successor/proof-cohort.json` | `206af774c2d3fdd7289718fee5ab5d44f5e05ab6573ddb961801412a1993cd19` |
| `proof-baseline-successor/proof-commands.json` | `ef3bc60f61843d6223b391484c7b615cbf0c896b88b9920f30e5ccf979551a3b` |
| `proof-baseline-successor/reports/native.json` | `3c0bde91982fb5099f8838a11bc4e5ae7b97d9d5e7682b3a0144eca1cb0fad1d` |
| `proof-baseline-successor/supervisor.json` | `b48fd53eb3ce03ca6651a0c8c2af7595dcd3617dd18a71b008316710cddc99af` |
| `proof-baseline-successor/proof-rehearsal-receipt.json` | `1e023c889e0df67fba375b33e69156065c38ecfd3194fa367889046d2ec993e5` |
| `proof-baseline-successor/campaign-preparation/proof-listed-descriptors.json` | `073bcdbc433bb88bc92c7f0decab1500dd1b92ddf2b98569a2af64c7f527421f` |
| `proof-baseline-successor/campaign-preparation/command-contract.json` | `b64f5d8798759923407221fd67ed3ddb375bcc82806dee132cbbdb95414f092d` |

### 3.39 TA-W04 conflict-proof per-file mutation result

CHECK-6/19/24/25/26. The exact GNU `roughtime-verification` proof cohort
met the 95% per-file mutation gate on product digest
`bf7964d9c4ad856a4c503a1e141c90127c35d2a43fc6c0a3cc8707a9991a8fc8`.
The private gate compared the fresh listing with all 26 descriptors and
rendered diffs and recorded its single dispatch before starting the campaign.
The fresh cargo-mutants baseline matched both original compiler artifacts,
command vectors and all eight named passes. It observed four unit passes
with 307 filtered and four integration passes with 58 filtered, zero failed
and zero ignored. It used the refreshed linker and both current graph receipts.

The raw outcome reader and causal readback agree on **22/22 viable
mutants caught (100%)**, four compiler-unviable, zero survivors, zero
timeouts and zero equivalent exclusions. Every caught mutation built and
failed a named selected test. The original logs identify the precise
assertion or source-local compiler diagnostic for all 26.

| Changed behavior | Count | Observed cause |
| --- | ---: | --- |
| Exact prefix size and claimed positions | 14 caught | Signed healthy, short, oversized and equal-position controls observed a wrong `Shape`, `Incomplete`, `Identity` or `Witness` outcome. |
| Original roster and identity | 5 caught | Roster division selected the wrong slot. Roster addition reached the invalid index in all eight selected tests. Identity predicate changes refused valid slots or accepted an isolated wrong root. |
| Link and first contradiction | 3 caught | Valid controls refused the original link or witness. The relaxed link predicate accepted both an altered link and correctly chained repeated randomness. |
| `Default` body replacements | 3 compiler-unviable | `E0277` at the replaced body reported no `Default` implementation for `ConflictEvidence`, `ConflictProof` or `ProofEntry`. |
| Whole verifier body replaced with `Ok(())` | 1 compiler-unviable | Denied unused imports, arguments and unread `ProofEntry` fields prevented the selected Build from completing. |

Eight gate controls, ten harvest controls and 15 result-reader controls
passed on the successor bindings. The isolated campaign completed in
525.396 seconds with one worker and the 90/60-second child caps. The minimum
sampled work and durable reserves were 8,223,350,784 and 9,927,065,600
bytes. The live owner accepted each memory and task counter check. The
supervisor rehashed all 65
retained files after read-only remount, observed the original worker and
cgroup retirement and removed the owned image. The closing source and tool
bookends matched the refreshed candidate.

The score covers the generated inventory for this GNU proof file. The
public reconstruction call still needs the direct refusal control from
§3.36. The nine other critical files need current per-file mutation evidence.
The request-encoder score in §3.35 remains bound to its earlier linker.
The public role remains listing-only. Priority-3 conflict-evidence fuzzing
remains planned under §3.31; the bounded priority-1 wire experiment does not
cover it. DATA-18 export and native Windows qualification remain open.

| Private evidence under `~/.local/share/nevrotick/` | SHA-256 |
| --- | --- |
| `roughtime-proof-gate-successor.UEBhxjZ7/python-controls.log` | `051c4325ce25295bfa8176cd8cb1ed88d899c34789a26679720eade87d9cc051` |
| `roughtime-conflict-proof-harvest-successor.gwZ61KlW/python-controls.log` | `7d47b70034bcbf0d93c56500f1989545ba591b46e49b78769099e7d7cf9485bb` |
| `roughtime-conflict-proof-result-successor.7oGePN8e/result-reader-receipt.json` | `803603e6b8b11dba3422748b32bb7147abafa758616110ddc13b1dd921432165` |
| `roughtime-conflict-proof-campaign-current.6ZRJdPZG/reviewed-inputs.json` | `6e5859b0a33685d6805a11880471caad649821c2742135d224a1a838a60a6e66` |
| `roughtime-conflict-proof-campaign-current.6ZRJdPZG/reports/dispatch.json` | `93b107c5aa05dc5ec0fdc155f6850fa728b8c4ef71634a9020f56d007ed9d8ad` |
| `roughtime-conflict-proof-campaign-current.6ZRJdPZG/reports/listing.stdout` | `3f8926999ec7e5af6eaeb79af44a2b993eb1fbb7aa2a1fddd7e4e3608670b40d` |
| `roughtime-conflict-proof-campaign-current.6ZRJdPZG/reports/native.json` | `615d56e2364dd8358a593e0e1dc34f8840c3a2bc73a6c34729ff05c1c41d95e7` |
| `roughtime-conflict-proof-campaign-current.6ZRJdPZG/reports/mutants.out/outcomes.json` | `908a168c9607d6d83bf45d92cfb4680c915f04a420fdcc3445fc40f0a7807ce9` |
| `roughtime-conflict-proof-campaign-current.6ZRJdPZG/reports/mutants.out/log/baseline.log` | `6a39c2e02536cf6611691ad14235f9e1391613fb15ecc77fbd66af48115cd97e` |
| `roughtime-conflict-proof-campaign-current.6ZRJdPZG/supervisor.json` | `a781b1ad70a541411c9e61c48000ccd1fdbb6eef45d4658ba7e6bc117d71a5a5` |
| `roughtime-conflict-proof-campaign-current.6ZRJdPZG/campaign-observation.json` | `e3d08f06b9a16f9873a49e39be3928b95fcac3d2d3abe220891892ac1329b254` |
| `roughtime-conflict-proof-campaign-current.6ZRJdPZG/proof-cause-readback.json` | `24491c25823b1a235981e85d1be4c6e0eb0a87f7d29d7c2f4fc3e157adcd27ba` |
| `roughtime-conflict-proof-campaign-current.6ZRJdPZG/proof-file-causal-review.json` | `563025a4ffb672572c47461836d118494473a325d7ba8c9a9d0d875ce66e2433` |

### 3.40 TA-W04 public conflict reconstruction refusal

CHECK-6/19/24/25/26. The existing proof regression now sends three real
loopback requests through `ControlledRecovery`, authenticates the signed
responses and retains all three original receipts. With radius three seconds,
the signed intervals are `(1017, 1023)`, `(1016, 1022)` and `(1006, 1012)`.
Both earlier lower bounds exceed the final upper bound. The public receiver
first reconstructs `(0, 2)` and returns a proof that borrows that same evidence.
Changing only the fixture's claimed earlier position from `0` to `1` then
requires `ConflictProofError::Witness` through the same public method. This
exercises the wrapper call identified in §3.36 and preserves the existing
test name and rosters.

The first isolated snapshot omitted supplemental cooldown manifests and
stopped before Cargo. With those exact files admitted, the focused test passed
and the original runner output retained its one selected pass. Clippy then
diagnosed two test style issues and repeated loading of the existing signer.
The three unit-test modules now share one signer under `cfg(test)`. The
separate integration test retains its own module. The final GNU product
`17c69e3b30cf23b9f0a936dfe3f7cc68e0e0d92fae694af73de849c89df7c6ad`
passed the exact test with 310 filtered, zero failed and zero ignored. GNU
Clippy for the selected library and tests and Rustfmt also passed. Independent
source review confirmed the first-pair expectation, authentic receipt custody
and test-only signer ownership.

The line and inventory policies bind that product on alternate index tree
`294e2f83dc824434ba081cf031e940cea7657e73`. Five line-reader controls
passed. All 90 named GNU tests passed under coverage instrumentation and all ten
files retained their line counts above the 90% gate. The new listing reproduced
all 601 descriptors. Two rendered diffs include the same appended test-only
module as context. A readback of the original output confirmed that the changed
lines and every other descriptor and diff are identical.

The §3.39 generated score retains its original candidate. Run later per-file
mutation cohorts against the updated exact test inputs. The public role remains
listing-only. Priority-3 conflict-evidence fuzzing remains planned under §3.31;
the bounded priority-1 wire experiment does not cover it. DATA-18 and
supported-Windows qualification remain open.

| Private development evidence under `~/.local/share/nevrotick/` | SHA-256 |
| --- | --- |
| `roughtime-public-witness.pyZpupfF/result.json` | `7d697aead96206d4c30143e777537140a799006fe444811acc2a43a0a9e812ec` |
| `roughtime-public-witness.TbB30qJY/result.json` | `faba9e4914802a06d526d8539fa8b2adf486a411949761c130b1ae840bb69e78` |
| `roughtime-public-witness.MzymMi66/result.json` | `9e02492fb3d84388a2ca75350d30955bf683280de49c6c1d67e6a68a8bc3217d` |
| `roughtime-public-witness.MzymMi66/test.log` | `1785f8e91d57ddde826e1d2c4676f575b2ec3310968aed308d1ddcd8e14823a4` |
| `roughtime-public-witness.MzymMi66/clippy.log` | `b5b1ab48c848810b7efca9ef9676fce9d1de71742607184659ff9851f338a9c4` |
| `roughtime-witness-metrics.nnDBEgg7/controls.log` | `e278d3c5eb76feb733c76182c825b2e5a88751542440797a86e517f25a0f0910` |
| `roughtime-witness-metrics.nnDBEgg7/coverage/verification.json` | `6a170b13e57c0f1f0e8e499109a9b9ffecac63a46176a0b51ce2f1bab10103f9` |
| `roughtime-witness-metrics.nnDBEgg7/enumerate/verification.json` | `5e7cfe41102242191aa1005508e24079641f14daddb524604d01814da4d20ea6` |
| `roughtime-witness-metrics.nnDBEgg7/verified.json` | `41665cf2cefd94048c6d732f93f6531bda3a1745d4f8d2d910eb16aac757bcd3` |

### 3.41 TA-W02/04/08 controlled DATA-18 disclosure boundary

RFC-M01, DATA-18/23, TA-C06/13/19/21, CHECK-6/14/17/21/24/25/27.
The selected source is index tree `c145c3eac5558e3c47d7dbdf186142275b45d722`
at `172d03517123e6c4947fc94074b9b15a17dddc61`.
`RecoveryRound::poll` moves the first signed contradiction into the original
`ControlledRecovery`. `ConflictEvidence::reconstruct` replays the exact
prefix and returns a borrowed proof. Current proof consumers are controlled
tests. DATA-18 still needs the receiver that authorizes handoff of those bytes.
[Recovery owner](../crates/nevrotick-engine/src/roughtime/round/recovery.rs)
and [proof receiver](../crates/nevrotick-engine/src/roughtime/round/recovery/proof.rs).

The controlled portable slice joins the proof receiver to a local in-memory
disclosure receiver. The original recovery invocation remains the sole holder
of its first conflict and original receipt prefix. The disclosure receiver
borrows that slot and reuses reconstruction before encoding. Maintain one
current action in the local disclosure owner. The action binds the exact
recovery and evidence slot to the controlled session and its fixed local
recipient. The current session and recipient are owned by the evidence slot;
they do not identify an OS user or a production IPC peer. A unique nonreused
generation identifies the action and its binding. The controlled caller
issues it by confirming that evidence.
A scan, a proof ID, or a previously accepted response cannot issue an action.
Replacement, dismissal and session retirement revoke both unused and in-flight
actions. The controlled fixture uses a finite monotonic action deadline.
Historical evidence remains eligible for separate disclosure while
the invocation still holds it and a current action exists. This works after
the acquisition cutoff without refreshing acquisition or date authority.

The receiver consumes the matching action into the only in-flight reservation
before reconstruction and encoding. At the serialized handoff, it rechecks that
exact reservation, its generation, recovery, evidence, session, recipient and
monotonic deadline under the same guard that commits the transfer. Replacement
with identical session and recipient still revokes the earlier generation.
Do not run callbacks or wait for I/O inside the
handoff guard. A bounded private buffer is transferred once to the controlled
local recipient. This stage uses a one-slot in-memory recipient that refuses
another transfer while full. At most one buffer per controlled evidence slot
is live across encoding and recipient custody. Check for a full slot before
allocating, and retain the reservation through cancelled encoding cleanup even
after revocation. Release
it on every exit and admit checked output size before allocation or growth.
Incomplete proof,
expired action, invalidated session, memory refusal or failed encoding yields
a bounded cause and no handoff. The receiver discards partial output; an
attempted disclosure never clears the retained conflict. The production UI
and ordinary-user file writer need their own current IPC, exact recipient and
publication evidence under DATA-23 and architecture §12.3. The ordinary
status and diagnostic paths receive only a redacted incident reference and
state. The encoded report includes original bytes which can carry private
context even when Base64-encoded. Keep the path user-initiated and local.
[ASVS applicability V14.1.1 and V14.2.3–7](../docs/asvs-5.0.0-applicability.md#v500-1423).

Bound the current controlled form to one borrowed prefix of two through nine
entries and one private output buffer of at most 65,536 bytes per action.
Each request is exactly 1,036 bytes, each response at most 4,096 bytes, each
root 32 bytes and each later link 32 bytes. Compact `responses` JSON with
padded Base64, the first `rand` omitted and the later `rand` fields present
takes at most 62,871 bytes. An independent standard-library size calculation
confirmed that envelope. Enforce the output cap before publishing. The bound
is for the controlled emitter and first-conflict prefix. General RFC imports,
larger rosters and aggregate installation retention have separate limits.

| Selected control | Independent observation required at the receiver |
| --- | --- |
| First signed contradiction, current explicit action | Read back the actual recipient's JSON independently. Require the exact original roots, request/response bytes and later links in original order, with first `rand` absent. Reconstruct the same first pair independently. Reusing the action yields no second handoff. |
| Current action after acquisition retirement | A retained historical proof reaches the same recipient while date admission stays refused. Earlier signed disagreement survives a later timeout or healthy round. |
| Missing or foreign action, replacement or dismissal with the same session and recipient, retired session, changed recipient, or expiry exactly at handoff | The recipient remains empty. Hold a deterministic control immediately before handoff to distinguish entry-time approval from current approval. A fresh action on the same proof succeeds. |
| Incomplete evidence, consistent evidence, or failure at the output cap | No report or partial bytes reach the recipient. A valid report at its exact required size proceeds. Compare the compact size ceiling separately using inert bounded data. Inspect retained conflict and subsequent date refusal before fixture teardown. |
| Full recipient or overlapping confirmation | A full slot refuses a second transfer without replacing its bytes. A second encoder cannot overlap the original reservation. After observed retirement, a fresh current action can transfer once. |
| Redacted summary and direct caller | Ordinary status, diagnostics and error results omit packet/key/link bytes. Test privacy at the audience-facing projection. Pair forbidden direct action construction with legal consumer use. |

Implementation entry requires the configured isolated GNU Roughtime lane,
read-only admitted source and graph, and current scratch capacity. Allow one
focused baseline and one causal repair within 240 seconds of product
verification work, with no concurrent build in that lane. Freeze the selected
case roster and tool inputs before execution. The later exact-candidate GNU
and MSVC checkpoints, applicable per-file coverage and mutation gates,
native UI/file lifecycle, OPEN-DATA-05 restart safety and clearance, and OPEN-8
whole-interval NTS admission remain with their existing owners. This design
checkpoint has no Rust execution credit.

**Fresh independent review.** The reviewer read the revised task at SHA-256
`2fefb883cda796d9b59c76a589b9bdecbcb190c610626cc9d2e1cf22d5020bf7`
against the cited index tree and completed a read-only review. Their initial
understanding was that the planned receiver borrows and reconstructs the first
conflict, then transfers one bounded RFC report after explicit confirmation;
disclosure does not refresh date authority or clear conflict. They described
the current recovery owner as retaining the original signed prefix, refusing
date admission during conflict, and providing historical reconstruction without
an existing disclosure publisher. The reviewer first identified missing
in-flight revocation and aggregate buffer limits. The revised contract and
controls address both. The reviewer independently reproduced the 62,871-byte
ceiling and found the bounded portable source stage ready after its existing
execution preflight. The receiver oracle must use fixture signer roots and
peer-observed bytes, and independently decode the actual recipient output and
check links and first witness. No Rust or native execution occurred in review.

**Source-stage preparation.** The coordinator copied the staged tree into a
private source workspace and checked the recovery, proof, test, task and Cargo
inputs against the original tree. The existing GNU recipe's isolation-only
preflight passed its boundary control and retired its scratch. Its receipt is
`~/.local/share/nevrotick/data18-preflight.88C6VJ3U/preflight.json`, SHA-256
`27ef102a555c81b25b4bb731beb418d3868761abb02a712a53868d6953f078c1`.
This observed credential, filesystem and loopback isolation only; it executed
no product case or graph admission. The selected source writer owns the private
workspace `/dev/shm/nevrotick-data18-source.Zs1myNRp` and its bounded output.
The main checkout and index remain under coordinator ownership. The completed
keyed-fuzz build caches were relocated to the private temporary directory
`/dev/shm/nevrotick-keyed-build-cache.MgrfU8ve` after checking for consumers;
the original logs, binaries, corpus and fixture inputs have a separate durable
copy listed in §3.31. Source, graph, tool and current scratch admission must be
checked again before executing the actual product candidate.

**First source candidate and focused GNU checks.** The isolated draft adds a
single evidence-bound disclosure owner and selected tests. Its frozen product
input map was
`aba04b4fe0fabbf49fb20cf0ff328bd666a5ea517b6d0feba9e0bffddcc7475d`.
The first focused invocation stopped at Clippy with 34 missing private-item and
error-contract documentation diagnostics before tests. The same repair also
corrected the new integration tests' error conversion. The repaired invocation
passed format, selected GNU Clippy, two selected library tests and two selected
integration tests. It rechecked source, graph, vendor and tool identities and
retired scratch. The first attempt took 30.047 seconds; the repaired receipt
charges 31 seconds and records 47.137 further seconds. The original 240-second
verification allowance and first failure remain in force.

A fresh read-only source reviewer inspected the five selected file hashes and
the raw test logs. They described a borrowed evidence-bound action, nonreused
generation, serialized final deadline and transfer, and a reservation retained
through buffer drop or transfer. The independent recipient oracle compares
decoded bytes with fixture signer roots and actual peer-observed frames, links
and the first causal witness. They found no demonstrated source defect in this
controlled path. The public historical receipt getters and controlled
confirmation do not establish production user authorization. UI/IPC identity,
actual recipient and file publication remain with their owners.

The same review found planned controls still incomplete: actual acquisition
cutoff expiry and subsequent date refusal after disclosure, unchanged full-slot
bytes, retirement during the precommit window, the new external doctest and an
audience-facing redaction projection. Allocation failure and an independently
constructed maximum-size encoded report are also unobserved. Complete the
reachable controls and identify absent actual consumers explicitly. The
verification budget remains shared across that continuation; do not credit this
focused receipt as the complete §3.41 exit. The reviewed candidate is preserved
under `~/.local/share/nevrotick/data18-source-draft.F4JvJug3/`; its five-file
manifest SHA-256 is
`960cd080f66010f7b768975a937ba8f867a7acf1ebe797cd0ac9048560eb0709`.
The initial generated manifest accidentally included itself and failed its
check; `SHA256SUMS.first-incomplete` retains that observation. The corrected
manifest verified all five files.

| Evidence under `~/.local/share/nevrotick/` | SHA-256 |
| --- | --- |
| `data18-focused-attempt.77TCTSzd/result.json` | `c96621ad56589d2b5e7f34315c8600dcc685dbba55f7b240b87e75db9e9a82c2` |
| `data18-focused-attempt.77TCTSzd/gnu-clippy.log` | `80602a6007e8110da135585672eec78a23c61a43aaa43d6978380c096140c655` |
| `data18-focused-repair.GFhYkWFR/result.json` | `00423f0cf52a335439e57d26be9e30801acb212184608e83bd22db46a8f8b2fb` |
| `data18-focused-repair.GFhYkWFR/gnu-lib-tests.log` | `b2ba8cfd276af74d4861130a58ec8117455fede62921cf9030a98d1c50e618ae` |
| `data18-focused-repair.GFhYkWFR/gnu-integration-tests.log` | `71e14eea088045db6b36d17c2021392aecb68bc601601be171bc75218ede96ea` |

**Expanded controls and critical-file registration.** The source test owner
added direct observations of the original recipient bytes after a full-slot
refusal, both date refusals after disclosure, session retirement immediately
before transfer, and the existing actual acquisition-cutoff expiry path. The
last case uses its pre-existing host-timed wait. A separate inert maximum-size
specimen checks the 62,871-byte envelope. The resulting four library and three
integration cases passed selected GNU Clippy and tests. A fresh reviewer read
the changed tests and raw logs and confirmed those four added observations.
They found no new source-proven defect. The independently measured maximum
does not exercise an actual maximum-size signed response, and allocation
failure and an audience-facing privacy projection remain unobserved.

The disclosure source is now individually registered as one security module
and one critical file. The existing 90% and 95% thresholds remain unchanged.
The ten-file Roughtime metrics role fails closed by source inspection until its
selection and policies are rebound to all eleven registered files. Independent
read-only review checked the registration and its scope. It did not execute the
metrics admission. The registered candidate's source map is
`1603f9e7e12c7fc8e5ef8d21be09fcb6ee14e5cf937b71e608824d9b27948d16`.
Its seven focused cases, Clippy, source/tool/graph bookends and scratch
retirement are in
`~/.local/share/nevrotick/data18-registered-controls.qzC4kYwz/result.json`
(SHA-256 `f11bafc2c45f64c974085981172738532ad5175ed62c48921d7719300397aaf8`).
The receipt records 178.587 seconds consumed against the original 240-second
work allowance, including the first refusal and all earlier focused runs. The
two new API doctests retain their own execution and diagnostic obligations.

**Doctest reader failure.** The selected isolated Cargo doctest command exited
zero. Its raw log contains 100 distinct passing cases in two rustdoc batches,
21 and 79, including the new legal-use and compile-fail examples. The private
reader expected one terminal summary and therefore stopped before its closing
source, tool and cgroup bookends. The isolated tested scratch was retired.
The outer receipt remains incomplete, and the raw result is an attributed
observation rather than a complete source-bound doctest checkpoint. Rustdoc's
normal output does not identify the negative case's actual compiler diagnostic;
the intended E0451 remains unverified. The later checked 828 candidate file
digests cannot reconstruct the original omitted closing snapshot.

The attempt used 22.742 seconds, with the cumulative receipt charging 201.742
of the original 240-second allowance. No budget is reset. Both failure receipt
and raw output remain at `~/.local/share/nevrotick/data18-doctests.esNqQZB6/`.
Their SHA-256 values are
`ddf372723f31077ec24296739b1c4331c414fc5bcc78ac2020d6896e7e1cbe68` for
`result.json` and
`2f80aad329acd55062e2cb605c55379a97eda407260f17b3f4900f311291962b` for
`gnu-doctests.log`. Correct the reader to reconcile both rustdoc batches and
challenge its controls before any permitted continuation. Retain the exact
diagnostic obligation separately and observe the remaining shared work bound.

**Doctest reader continuation.** The repaired reader reconciles each rustdoc
batch's announced count, individual case results and terminal summary. Its
inert controls accepted the original complete log and rejected 13 altered
batch, count, result, duplication and selected-case specimens. A new isolated
run then passed all 100 Engine doctests in the two 21- and 79-case batches,
including both new `DisclosureAction` examples. The complete source, graph,
tool, vendor and cgroup closing checks passed, and scratch retired. The receipt
charges 234.067 seconds against the original 240-second allowance. Ordinary
rustdoc output still does not expose the compile-fail case's compiler
diagnostic. The [external consumer check](#346-data-18-explicit-bindings-and-external-action-privacy)
records the exact E0451 observation for the successor candidate.

The repaired result is
`~/.local/share/nevrotick/data18-doctests-repaired.drNuiIie/result.json`
(SHA-256 `c890f695797622ec33c8ad5f14707451bbaaca5f67e9277dfeed5846c4ad41f6`).
Its raw `gnu-doctests.log` SHA-256 is
`b02def6c31c5edd799792c513960cdddbba3e465e0c694150b808d7c6ae0ad58`.

### 3.42 DATA-18 metrics-role reconciliation

CHECK-19/24/25. The disclosure owner adds an eleventh registered Roughtime
security and critical file. Its focused source result in §3.41 does not
requalify the earlier ten-file coverage or mutation inventory. The independent
static role selects each exact registered owner, the current product identity,
and the named per-binary test roster. It must refuse a missing, duplicated or
substituted owner and a malformed, reassigned or substituted roster. A healthy
role remains eligible for its separately admitted coverage or listing run.

`qa-regression-selection`, `qa-static-checks` and `qa-mutation-testing` own
this accounting change. Exercise the actual Python admission and result readers
with bounded synthetic controls and an independent expected roster identity;
bind the inventory role to the exact line-policy bytes. The expected 32 library
and 63 integration cases are derived from the prior selected roster and the
five additional source cases. The focused run observed those five cases and
the binary totals; a complete current-roster discovery remains pending. Python
reader checks have a separate 30-second watchdog and create no product-build
credit. The preceding source slice's 240-second allowance is not reset.

Coverage and mutation enumeration require their existing independent resource
preflight, exact candidate and source/tool/graph custody. The former retains the
90% per-file line gate; mutation retains the 95% per-critical-file requirement
and the public inventory role remains listing-only. The earlier 601 descriptors
and scores belong solely to their recorded candidate. No new listing, score,
native result or production DATA-18 qualification follows from role repair.

The shared scope and both policies now select eleven owners and bind the
registered source candidate. A code-owned canonical digest binds the planned
95 named cases independently of the mutable policy roster; the live reader
still requires those cases to execute. An independent reviewer checked that
the prior 90 names and ten owners remain, the five disclosure cases and owner
are added, the policy hashes match and no historical inventory is imported.
The reviewer identified a wrong-binary control whose changed counts caused an
earlier refusal; it was corrected to swap two cases while retaining 32/63 and
to require the assignment-specific error. No further source finding remained.

The final synthetic Python controls passed: six line-reader and seven inventory
tests, zero skipped or failed. The isolated Python 3.12 invocation used a
30-second watchdog, consumed 0.934 seconds, and matched all 202 recorded
Python/policy file hashes before and after; its temporary directories retired.
The retained receipt is
`~/.local/share/nevrotick/data18-metrics-python.uzxtwe2s/result.json`
(SHA-256 `66c729bc6370e53e3108a4d9f0f2168ced274dd6648054fcbb830b435888a3ab`).
This checks the reader's synthetic healthy and refusal paths only. Complete
95-case discovery, current coverage, current mutation listing and per-file
mutation campaigns are still unexecuted. The last preflight observed roughly
275 MiB free on the durable filesystem, 5.6 GiB on `/dev/shm`, and 4.1 GiB on
`/mnt/wsl`; those observations do not meet the existing campaign reserves.
The public coverage and listing roles separately require a private canonical
Linux output parent with at least 2 GiB free. They reject a direct `/mnt/wsl`
output, while their runtime reserves `/dev`. Historical namespace launchers
used a private `/tmp`; their tool hashes still match, but they do not themselves
provide an outer memory/task supervisor or durable fsynced receipt custody.
Fresh resource, source, graph and vendor admission remains necessary.
An independent preflight found that the historical launchers may lose their
private progress files on timeout, and the old storage canary lacks an owner
ledger and full-transaction cleanup for interrupted setup. Do not run either
unchanged as a failure-safe supervisor. Before a new campaign, qualify a
bounded owner that records its unique image, mount, daemon and cgroup before
effects, supervises the actual work namespace, preserves incomplete results,
and proves durable readback and retirement. The previously retained C:-backed
image and other owners' processes are not scratch. The Linux mount's access
mode alone does not establish the backing image's Windows ACL.

A private first-step ledger now records intended resource identities and
planned, created, incomplete and retired assertions. It performs no image,
mount, process, cgroup or cleanup operation and does not observe OS completion.
An independent source review identified and the author corrected strict
boolean identity handling, nonblocking lock admission, indeterminate post-write
failures, root-path admission, typed malformed JSON, caller-input snapshots,
parent fsync and a same-byte publication replacement. The coordinator reran
23 synthetic tests with exact source bookends; all passed. The private files
are under `~/.local/share/nevrotick/resource-ledger.pVX5dNk5/`: `ledger.py`
SHA-256 `16490088041f7b7b6feac1badb07eae269a6796c9d8eabff7bc2de4b173bd7a5`,
`test_ledger.py` SHA-256
`4b7aa2d54dcca75502e7a9b3fe38cedfa9da253cbd7687df390e6dcff894fe05`, and
`coordinator-result.json` SHA-256
`84dddf8824821d9d801ea024806a98c844bf3394c35ac0a6349c93eecfb2cb59`.
This prototype is not an admitted supervisor. External trusted recovery pins,
the planned-to-effect crash gap, real OS identity and retirement observations,
namespace monitoring, resource-stop controls and durable receipt recovery all
remain open. Device/inode readback does not prove universal object continuity
against inode reuse, same-inode writes or changes after a check.
The next inert supervisor slice is a separately durable owner-pin journal:
bind the exact run, ledger and lock identity, cumulative history, previous
pin and intended target/token before any corresponding effect. Exercise
healthy append/readback and stale, torn, forked, substituted and fsync-failure
controls using owned temporary files. A pin journal on the same filesystem
does not provide rollback resistance against its owner or administrator. An
effect completed before its dynamic identity and matching pin are durable is
indeterminate; it is not permission to delete a same-named resource.
Any real recovery owner must also bind the operating-system boot identity and,
for a service, its systemd invocation to fresh live observations; a run ID,
PID/start pair or mount ID by itself does not establish continuity across boots.

The separate inert pin-journal prototype now composes the original ledger's
checked state with append-only pin assertions. Twenty-two synthetic controls passed
on the independently reviewed bytes, with the ledger and journal sources
bookended. A deliberate competing-coordinator interleaving can still append an
extra unpinned assertion after a precheck; it leaves the caller indeterminate
and reopening refuses. Shared transaction serialization is required before any
real effect can rely on this design. The prototype does not issue effect or
cleanup authority and does not establish external recovery-pin custody or
anti-rollback. Its private sources and receipt are under
`~/.local/share/nevrotick/owner-pin-journal.IQi4yKMc/`: `pin_journal.py`
SHA-256 `d2bbcf90f08922abc2cb665a360aa01dbf01ce71d03965eeaab001bef02f19e7`,
`test_pin_journal.py` SHA-256
`6af4a08b59211dfa801cc3126b47a6a306e967876d85cb58c8e8048d4c1d5a15`, and
`coordinator-final-v2-result.json` SHA-256
`d45314773dbf9ae143e5d0fc27b6da6745537c372aa63ad6a19a08774deabd2a`.
The previous 18- and 20-control receipts remain intermediate candidates.

A later private candidate and its test receipt are retained under the
`transaction-pins-nonblock.KNvKYlkT` directory in the existing evidence store.
Its 54 synthetic controls passed; it remains unqualified for real operations.
The coordinator result SHA-256 is
`532a10679fa7dd9998f7932e4e106a685d7f6fe0ee4f54fa8d97c1de3bdbb501`.
Independent source review found no further issue within this inert scope.
The newer candidate holds the ledger guard through record and pin publication
for cooperating callers and captures its own revision while guarded. Direct
use of the ledger after unlock remains separate; neither prototype issues
authority for a real resource operation.
Current resource identity, crash recovery, effect admission, durable receipt
custody and coverage/listing execution remain separate open work.

### 3.43 Roughtime local audit repairs

CHECK-19/24/25. The local audit found that the eleven-owner line reader used
whole-file LLVM summaries containing inline fixture code. The affected tests
now reside in separate test-only source files. Test-only dispatch and write
fixtures also have separate implementations, while their production paths use
the same remaining request slice and original dispatch claim. The Roughtime
line-reader controls check the source separation and require the production
file itself to meet the 90% threshold even when a fixture file is fully covered.
The TCP fixture observes actual progress, including bounded no-progress polls,
before comparing peer bytes and testing withdrawal.

The disclosure controls now exercise cross-thread reservation and recipient
contention, lifetime boundaries, deterministic allocation refusal and a
62,871-byte report holding nine authenticated exact-cap response frames.
The response padding is an artificial client-boundary fixture. Independent
readback checks original frames, roots, nonce ancestry and the conflict.

The candidate source map is
`24c675d9af83d5b95a85ac2db5a5a7c90464cfff6e06af9dfa8e220c49501b1e`.
An isolated GNU development run passed Clippy and all 95 selected cases,
with 32 library and 63 integration cases. Its LLVM export contained exactly
the eleven selected source files, with no test functions mapped to them.
Each observed file exceeded 90%; the lowest was exchange at 312/339.
The isolated run is development evidence and does not replace the configured
coverage checkpoint, current mutation campaigns or native qualification.
Fifteen focused Python reader and inventory controls passed. The source
layout check is a targeted regression control, not a complete Rust parser.

The initial final-candidate Clippy attempt refused a complex fixture return
type. A named test type corrected it before the successful run. The retained
development outputs are under
`~/.local/share/nevrotick/local-audit-fixes.wua2puuf/`.
`inputs.json` has SHA-256
`2790366b01f22afe82bec2a479c4b92f060beebba9b15485febc34489bc5fb48`,
`instrumented.log` has
`e977919a8db08dc10f7da63c416942d8f5b40842fca6059f7a2ece1f925d9669`,
and `coverage.json` has
`d1851f2387005118fa1278bd3c3fa34149271145673ae86afbd3ee0bef5b7500`.
The original unmodified checkpoint and its consumed budget retain their
recorded identity. This repair does not refresh historical per-file scores.

### 3.44 TA-W07 distinguish unavailable certificate revocation status

OPEN-8/R-05/CRYPTO-04 and TA-C11/19/22. The bounded source slice starts
at `36849ebfc58ae0af1df5ee778b9d163b65acf492`. The ordinary laboratory
verifier's SSL-policy status passes through Rustls and the real NTS-KE
handshake error projector. Preserve an unavailable revocation check as a
closed refusal distinct from certificate dates, name mismatch and explicit
revocation. A successful SSL policy result still proceeds through the original
handshake. The strict cache-only policy, stage labels and refusal before
association publication govern the change.

Apply `implement-rust`, `implement-rust-contracts`,
`security-crypto-secrets`, `dependency-updates` for derivative byte
admission, and `qa-contract-tests` for the policy-result and Engine
projection controls. Select an isolated MSVC check only after its resource,
tool and vendor preflight. The two documented nonzero policy statuses must
refuse as unavailable revocation. Success, explicit revocation, expiry,
name mismatch and an unrelated status are independent neighbors.

The Windows policy result maps `CRYPT_E_NO_REVOCATION_CHECK` and
`CRYPT_E_REVOCATION_OFFLINE` to Rustls `UnknownRevocationStatus`. The Engine's
direct TLS error projector retains `RevocationUnavailable` at the original stage.
Normal error reporting still projects `handshake / invalid-data`, and the
existing mandatory handshake completion precedes any NTS-KE request, exporter
or association publication. Pinned Rustls queues the fatal `UnknownCA` alert
for this error. The previous opaque error queued `CertificateUnknown`.
Neither Windows status identifies whether a CRL was missing or stale.
[Windows SSL policy statuses](https://learn.microsoft.com/en-us/windows/win32/api/wincrypt/ns-wincrypt-cert_chain_policy_status)

The fresh independent reviewer examined both native refusal codes, the
existing handshake fallthrough and first-fault ordering, both test boundaries
and the new source and policy pins. The review completed without a blocking
finding on product input map
`7542897bbb9fbe52adc6ed59dc1514adb35ae97f84702b15e7f7ee203b7b07f5`.
The derivative's original upstream entries, licenses, manifests, dependency
graph policy rows and Cargo lock stay pinned. Both Roughtime metric policy
files bind the new complete source map. Historical results retain their
original candidates.

Regression selection used the real `nevrotick-engine` target and
`ke-association-prototype` feature. The isolated Rust 1.100.0 check admitted
the exact vendor and tool inputs, passed its sandbox boundary control, and
matched the frozen Cargo metadata and Engine dependency trees for GNU and
MSVC. MSVC Clippy checked the Engine library and tests with warnings denied.
GNU executed the exact public label compatibility test: one passed, none
failed or ignored, 652 filtered. Rustfmt and whitespace checks passed.
Twenty-four dependency-profile and fifteen Roughtime role-reader Python
controls passed. The two initial compiler-wrapper attempts stopped before
Cargo because the private wrapper passed patch records rather than the tuple
of package names. The corrected run completed within its bounded allowance.
Source, tools and vendor matched on readback and the owned scratch retired.

| Record under `~/.local/share/nevrotick/ta-w07-revocation.uebNzyUU/final/` | SHA-256 |
| --- | --- |
| `source.json` | `6c6be9a33bc8773683238e6a8c67a3804c8087f7966f1ac0f0068652226acd3f` |
| `msvc-clippy.json` | `8268ed287a66246b7da9bea474ad4d5240ce9b417155bd707ff7092c8b8bd2d4` |
| `gnu-cli-projection.json` | `8c628f70fa45bef44a0eb49f97b9da3377d8fe174a49bdc0619d3bd8c5791da8` |
| `bookends.json` | `cd0a2586435887635c6087be30f56f12ff62a700135e4fa3f8d906aadfe24107` |

The existing OPEN-8 point image and private collector currently expect opaque
refusals for the three CRL rows. The next native trial needs a new admitted
image and matching bounded transcript controls with the distinct expected
cause. Observe actual Windows policy and the same-handshake outcome under
independently cold admitted trust and candidate/ambient time settings.
Revalidate the existing CRL fixture's admitted UTC window before use.
No admitted volume met the existing cross-link's 6 GiB entry floor. The
Windows-only Engine and derivative behavioral tests remain unexecuted, and
the derivative test dependency graph retains its own admission gate.
The configured OPEN-8 link, native result, full-interval receiving and
production date-basis admission remain open.

### 3.45 TA-W07 prepare the typed OPEN-8 point observer

OPEN-8/R-05/CRYPTO-04 and TA-C11/19/22. The retained point image and
collectors in the [crypto feasibility task](s0.1-crypto-feasibility.md#open-8-windows-laboratory-point-image-binding)
remain the predecessor. Its three CRL cases required an opaque refusal.
The successor Rust fixture, binding producer, Python comparator and PowerShell
collector templates require the exact handshake-stage
`Certificate(RevocationUnavailable)` for those three cases. They still pair
each refusal with the same leaf, CRL or CRL absence, clock point and expected
peer exit. The five publication and two leaf-expiry expectations are unchanged.
The ambient-stale row retains the independent OpenSSL candidate-point success
and ambient-time CRL expiry. The Windows refusal remains an expectation for the
cold guest trial.

Planning, environment preflight, golden fixture parity, regression selection
and independent review used the retained predecessor pins. The private fixture
patch applies to the current checkout. Its source map adds only the test module
and its registration to the current 833 product inputs, producing 834 inputs
with canonical SHA-256
`8ac315f2a6657b7343788c2cb2ffffc9de50e652cb806281aefd99649f8d2fdf`.
The successor binding checks that complete map before issuing a point receipt.
Review found the historical image length in a guest gate, a missing successor
scope check before guest effects and a stale patch blob label. The repaired
source binds the new image's exact observed length within the 64 MiB PE reader
limit, checks version, scope and supported outcome before dispatch, and
identifies the correct Git blob. The reviewer rechecked all three corrections
and matched the final receipt's hashes. Collector and comparator pin
placeholders remain unfilled until a new image exists.

The isolated Linux comparator ran ten synthetic matched cases and 53
targeted refusals. The separate predecessor control rejected the successor
classification and accepted its own original classification. The successor
also refused opaque and explicit-revocation substitutions, wrong handshake
stage, wrong result counts, appended output, stale scope and invalid image
length. The existing renderer composed the pending PowerShell collector from
the pinned predecessor. The first harness attempt could not create a mount
under the read-only source tree. The retry reached the comparator controls
and stopped at renderer import without a sandbox home. The final run used a
separate read-only historical input mount, disposable home and scratch, matched
input hashes before and after, and retired its scratch.

| Record under `~/.local/share/nevrotick/open8-revocation-observer.dCxSw5QV/` | SHA-256 |
| --- | --- |
| `open8-fixture.patch` | `e1f260b4aa3b5fc19e730b4ea9575e1af11ac686df6ba8f555fe4d38b7046946` |
| `bind-points.py` | `bd418f44723b3276bcc7f0ed2b456950135692bc57c2d4c31d615040333f07d4` |
| `reviewed-observations.json` | `29ab7215c5e848b958c33fa3c5111bfbab45295ffbc521ba907f0fa21c743f30` |

Re-enter after the existing 6 GiB cross-link preflight passes on an admitted
volume. Bind the emitted PE/PDB, selector and exact executable length, apply
the same new binding hash to both collector inputs and the Python comparator,
then review the final rendered bytes and run the applicable PowerShell
controls. Native re-entry also needs the original cold guest and trust
prerequisites, a rechecked CRL validity window, image discovery and exclusive
guest shelf custody. The successor image, native execution and whole-interval
handoff remain open.

### 3.46 DATA-18 explicit bindings and external action privacy

The independent review of `36849eb` found missing explicit local types
(QA-S01) and requested the external `DisclosureAction` construction diagnostic
(QA-E01). Rust contract review and independent source readback checked the
source-visible typing rule against the disclosure encoder and its unit and
integration fixtures. The correction types 23 ordinary bindings and four
adjacent destructuring patterns using their existing types. The 833-input
product source map has canonical SHA-256
`853924785c086acecfb14ccef731f18c79c7dcfa646e10ae342af31300b5beab`.
Both metrics roles bind that candidate and retain the existing test rosters and
thresholds. Formatting, GNU Clippy for the Roughtime Engine library and
integration test, and 15 metrics reader tests passed.

The contract and regression checks compiled two external consumers against the
same sealed Engine library with pinned Rust 1.100.0, target
`x86_64-unknown-linux-gnu` and feature `roughtime-verification`. The legal
consumer compiled `confirm_disclosure` followed by `submit`. The struct
literal from the compile-fail example produced one error at `direct.rs:3`,
E0451, identifying both `evidence` and `generation` as private fields of
`DisclosureAction`. The existing diagnostic reader matched the error code,
location, field names and compiler summary. The negative produced no metadata.

The isolated compiler check took 35.722 seconds within its 160-second bound,
with two Cargo jobs and a 4 GiB scratch entry floor and 3 GiB stop floor. The
source, tool, vendor and standard-library identities matched before and after
execution. Scratch retired. The preceding doctest receipt retains its own
234.067-second charge against the original 240-second allowance.

| Record under `~/.local/share/nevrotick/disclosure-consumer-contract.adlgS0rp/` | SHA-256 |
| --- | --- |
| `bookends.json` | `98eb328657cda4068bb857e5f1be83cc187e5abaf7a60ddd9daf320231945afe` |
| `legal.json` | `61afaca3e501dec8167a887cf9fdf5c8d7dc43fb2e944a4de9d0db4f1f01115f` |
| `E0451.jsonl` | `0b6032b7d795b58623606ca33d0665c58434b80dc82607cffda588123046cf0e` |

**Audit closeout on the §3.51 successor.** Independent source readback found
all 23 ordinary bindings and four adjacent destructuring patterns explicitly
typed. The remaining inferred waiters have the anonymous async types permitted
by the source-visible rule. The successor's 833-input map hashes to
`6c06a4d3292d7166eba081e9b60150fe060393730cee3f3eca86c7ecc7d39db4`.
Fresh workspace formatting and GNU Roughtime Clippy passed. The paired external
consumers used the same newly built Engine library. The legal call compiled
and the forbidden literal produced exactly one E0451 at `direct.rs:3`, naming
both private fields. Its raw diagnostic was byte-identical to the earlier
observation. Input bookends passed and scratch retired after 58.380 seconds.

| Record under `~/.local/share/nevrotick/disclosure-consumer-successor.sOrwZ9/` | SHA-256 |
| --- | --- |
| `bookends.json` | `374ff4bc05bd4a9f7e224af342ca5732c745fd3e094a2c3a818d21ef7489a520` |
| `legal.json` | `521e03ca7fb23cd7a6a2a5c2f2f349cc226f4ee22ac90ff00fa3c98f25fdb1a9` |
| `E0451.jsonl` | `0b6032b7d795b58623606ca33d0665c58434b80dc82607cffda588123046cf0e` |

This compiler evidence covers external action construction. Current metrics,
Windows execution and the whole-interval receiving path remain open.

**Portable wire sanitizer code generation.** A bounded preparation attempt
compiled the dependency-free `ntp-wire` library separately for ordinary GNU
and the Rust 1.100.0 GNU ASan, MSan and TSan targets. The retained LLVM IR
readback found zero sanitizer calls in each of three named decoders on the
ordinary target. ASan found 4, 23 and 37 calls, MSan found 4, 33 and 62,
and TSan found 7, 32 and 64, respectively, in fixed-header, Roughtime request
and Roughtime response decoding. All four graph and compile commands passed.
The compiler, target payloads, source snapshot and vendor inputs were admitted
before the commands. The owned scratch readback subsequently matched all
25 wire source files and the four retained IR hashes.

The first preflight rejected an overly strict comparison of the ordinary
standard-library inventory. A second attempt compiled successfully but the
observer looked in the wrong Cargo output directory. A third observer included
closure symbols in its function match. The repaired run reached all four IR
observations but exhausted its original combined time budget during final
input bookends. The qualification is incomplete. The retained records under
`~/.local/share/nevrotick/wire-sanitizer-codegen-final.PCMtEd/` preserve
the output and exit 124. The standalone inputs and IR do not establish
detector operation or product execution on these targets.

### 3.47 TA-W07 rebind the OPEN-8 overlay after the compiler contract check

OPEN-8/R-05/CRYPTO-04 and TA-C11/19/22. Environment preflight and fixture
parity applied the exact two-file fixture patch to the current product
inputs. Independent readback reproduced the 834-input overlay SHA-256
`4351c04b6cf3de902c04324dcbe09a4f03c6f7a05142ebaf1dc98e83e2012b03`.
The product's three annotation files account for the full difference from
the previous overlay. All 833 product inputs still matched after the
documentation and skill fast-forward to `249dcee`.

The rebound image binder's source gate accepted the new map and reached the
deliberately absent staged fixture. It refused the previous map, one replaced
annotation-file hash and an omitted fixture at the source gate. A fenced
Linux run retained the same receipts before and after and retired scratch.
No image binding was issued. Evidence under
`~/.local/share/nevrotick/open8-disclosure-rebind.un6h10s9/` includes
`source-rebind.json` SHA-256
`72b3116fcf3f0ee9e11cb91cccd7d5601a6d00a5b409e42f93cab6f7a19d2096` and
`binding-controls.json` SHA-256
`2fdd1cb4ba29be5225389ed3b9685746e6b8f2fc5fe8ec65b1c833a362213254`.

The Linux work volumes did not meet the existing 6 GiB entry floor. The
[prior C: ext4 canary](engine-transport-pilot.md) supplies a backing-volume
mechanism. Before reusing it, check executable linker-wrapper custody in the
actual OPEN-8 sandbox, the 6 GiB entry and 4 GiB live floors, and separate
retention capacity for the emitted PE/PDB. The cross-link, image binding and
cold guest observations remain open. Apply [OD-22](../docs/nevrotick-owner-decisions.md#26-od-22--sanitizer-qualification-cadence)
to the coherent module checkpoints. The installed GNU sanitizer targets
supply preparation evidence. Product instrumentation, applicable detectors
and native Windows results remain separate task obligations.

### 3.48 TA-W07 verify the C-backed OPEN-8 build volume

Environment preflight and fixture parity exercised the existing product
sandbox with an owned C:-backed ext4 image. The first 30.020-second attempt
matched the 529-file SDK inventory and observed 9,927,102,464 free bytes on
the mounted work volume. The outer hard CPU limit prevented the configured
inner `prlimit` from starting the boundary control. The probe retired without
invoking Cargo. Its `capacity.json` under `open8-ext4-canary.VKslK8/` has
SHA-256 `a084b0f4e97e8b0efc11f2d82ec533110aa7b9cd170b2f94f6f550502b82e3fd`.

The 24.465-second repair retained the first charge and used the remaining
39-second wall allowance. The outer soft limits stayed at 45 CPU seconds
and 128 descriptors, with hard ceilings matching the inner 90/256 limits.
The configured isolation control passed. The pinned linker ran through the
read-only `/work/linker` bind, a write to it failed with `EROFS`, and the
sibling export was outside the sandbox's writable work mount. A same-device
hard link produced the exact expected bytes after read-only remount. The
work filesystem exceeded the 6 GiB entry and 4 GiB live floors. Inputs
matched before and after. The image, mount and service retired. The
`open8-ext4-retry.pG6NRA/capacity.json` SHA-256 is
`19fa79f3c723f19c91cb7085d8cf1cdf8441d17039d3c9068e46ddf68ab4aade`.

The C: parent exposes inherited read access to `CodexSandboxUsers`. The
separate successor build directory has a directly observed Windows ACL
granting the local owner access with inheritance disabled. This scopes
artifact retention to the owned child. The SDK, tool and overlay identities
still require their cross-link readback, and no guest execution follows
from this storage probe.

### 3.49 TA-W07 bounded cross-link on the admitted ext4 volume

The first cross-link setup stopped after 56.045 seconds when Git could not
write an object into the frozen checkout's FUSE-backed object database.
Moving the source and index to private tmpfs reached the exact 834-file
tree in another 6.859 seconds. That selected-file copy omitted the outer
native runner, which sits outside the product input map. Preserve both
failures. Their attempt receipts under `open8-crosslink-current.itdj8g/`
and `open8-crosslink-repair.zat4a0/` have SHA-256
`27f1d346016c66c91ee21124f463f617651493f2f7cf912f668a48bb4ca40c26` and
`e6b3bfa2555b89db1a41ae1cd51ba16774ff2d123e3bcbf640200954dd84de5e`.

The final freeze took the full tracked HEAD snapshot, replayed the current
product inputs and fixture patch, and checked the native runner import before
dispatch. The 834-file map matched §3.47 and the isolated Git tree was
`7c2d7b7d011e51dfc78ea424d94c7f9845c6f31d`. The boundary control, compiler
identity and Engine formatting passed. The fixed four-job MSVC
`ke-association-prototype` libtest build reached Cargo and was still in that
lane when the 191.351-second run expired. The outer timeout did not retain
the in-progress Cargo stream, so the last completed Cargo operation and the
dominant stall remain unclassified. No PE/PDB pair or image binding was
published. The unit, source tmpfs, mount and image all retired.

The three runs charged 254.255 seconds of the original 265-second allowance.
The terminal receipts under `open8-crosslink-frozen.vNMaJh/` are
`attempt.json` SHA-256 `7526372413d87a30dc4ebee8c2de83513625eb94938dce9ccd052a0c806cabd8`
and `unit.log` SHA-256 `8497945b7912cd3e1922b6608f6db6a38d8d249b91060f45284e5e5d6347b5bd`.
The current lane remains incomplete. Reentry requires a changed execution
input such as the previously successful Linux filesystem route with its
original reserve, plus a successor command that retains bounded Cargo
output through enforced timeout. Keep existing scratch and artifact
ownership when assessing capacity. The image binder, cold guest and
whole-interval receiving observations remain open.

### 3.50 TA-W07 preserve timeout evidence and check the private tmpfs route

The predecessor's outer timeout could end the build driver before its Cargo
receipt was written. The successor derives the exact pinned command supervisor
with one raw output sink after its existing byte-budget guard. It writes and
syncs each accepted chunk before continuing. Command arguments, capture limit,
timeout, resource guard and process-group retirement remain the existing
owner's. The separate `build-successor.py` binds those sinks to the selected
cross-link lanes. Healthy mixed-stream bytes, inner wall timeout, refused
overflow and changed-supervisor controls passed. An outer systemd timeout
stopped the real sandboxed inert child and its supervisor before terminal
receipt entry. The expected raw prefix survived and the unit retired.
No Cargo command ran in these controls. Evidence under
`~/.local/share/nevrotick/open8-stream-capture.jMV9BG/` includes
`controls.log` SHA-256 `3977ddcdbcdcd591e2844e0c012bde0614e56fce0c956c9d09834cecfb27067a`
and `outer.raw` SHA-256 `51a066525eb3d3de444b5fd507de659399ec2b2b7f4f5958eb9fdf9efcbd2499`.

The two shared Linux paths remained below the 6 GiB build entry floor.
The changed outer `bwrap` route mounted an 8 GiB private tmpfs and entered
the actual inner product sandbox. The first attempt hit a missing outer
`/tmp` mountpoint. The corrected topology passed the existing isolation check,
selected real linker invocation, fresh executable from writable scratch,
read-only source and linker controls, hidden sibling export and same-device
hard-link readback. It observed 8,589,934,592 free bytes at entry and
8,589,893,632 as the lowest live sample. Both attempts retired and consumed
0.454 seconds total. The retry `result.json` under
`~/.local/share/nevrotick/open8-private-tmpfs-retry.rS0dE5Jr/` has SHA-256
`9cb61d18188a318082bdae26cf01981e3ea6cdebbe1a5c7e3d85413c260c0fb4`.
The canary used small fixture inputs. Full-run physical memory fit, exact
graph and SDK readback, and a supervised build remain open. The 10.745 seconds
remaining in §3.49 do not admit another build.

### 3.51 TA-W07 carry the original handshake refusal to the receiving report

TA-C11/19/22 and CHECK-9/27. The existing Preview and protected-probe
failure reports now project the original typed handshake cause. The shared
owner selects the primary failure and admits only its `Handshake` stage.
The payload-free `TlsCause` and `CertificateCause` types retain diagnostics
with the verifier context discarded. The legacy stage/`invalid-data` labels
and independent cleanup observations retain their current interpretation.
The projection supplies historical data. Retry, date admission and protected
publication keep their existing guards.

Rust contract and regression checks followed actual joined worker reports.
They preserve expired and not-yet-valid certificate, local clock, wrong
name, unclassified and unavailable revocation status. The Preview checks
refuse a typed error at a later request stage and an untyped handshake I/O
failure as diagnostic inputs. The protected-probe control preserves the
handshake cause alongside independent TLS and observer cleanup, with no
published receipt. Healthy Preview and protected worker cases still passed.

The 833-input product map has SHA-256
`6c06a4d3292d7166eba081e9b60150fe060393730cee3f3eca86c7ecc7d39db4`.
The first exact-candidate run passed MSVC Clippy and the legacy label test.
An incorrect module path selected zero new GNU cases and triggered the
nonempty test guard. The repaired 49.051-second GNU run matched all
source/tool/vendor and dependency bookends, passed five exact tests, and
retired scratch. It retained the MSVC check from the first run on the same
map. The first `msvc-clippy.json`, under
`~/.local/share/nevrotick/ta-w07-projection.rnygmH/`, has SHA-256
`60b5c122a3fc7c8ff1536356a937c0e26245abdf28899e516af6e8233059e9d4`.
The final `bookends.json`, under `ta-w07-projection-selectors.DXzpbj/`, has
SHA-256 `2d3fa204345a894e55e1fffc09bef61c22e347151747a96cf68d91fc394aa279`.

The planned eleven-owner metrics roles bind this candidate and their existing
acceptance gates. All 15 reader tests passed. The exact two-file OPEN-8 patch
also yielded the successor 834-file overlay SHA-256
`c3c80a6b2ce7dd506ccda533634b540e13de9086ac816b5bfaa28c9ad44a3064`.
Its `source-rebind.json`, under `open8-typed-refusal-rebind.xyvdffjq/`, has
SHA-256 `80be1135306374838f98491cc2a3462b84b8e45a1f728af33319bd70f1ad565f`.
The current per-file qualification, cross-link, native verifier, and
same-connection whole-interval receiving path remain open.

### 3.52 Rebind Roughtime verification after the authentication bracket change

TA-W03/04/06 and CHECK-26/27. Both configured Roughtime lanes required the
§3.51 source identity after the Engine began retaining the original TLS
authentication bracket. The coverage and inventory policies now bind the
admitted 833-input product map
`18db3db3c4ac593d6830244275c536b7c4c98de30ee304aa0052a32716eced86`.
The inventory binds the updated line policy. The roster still has 32
library and 63 integration cases. The 90% line threshold, 95% per-file
mutation threshold and listing-only inventory admission remain in force.

Rust contract review made the component test's `AssociationCandidate<'_>`
type explicit before freezing the product map. On that map, the isolated
Rust 1.100.0 run passed formatting, Windows-target Engine library Clippy,
and four GNU Engine library cases with `ke-association-prototype`. The
cases covered the bracket window, separate associations, typed refusal
with cleanup, and the two receipts from one association. Source,
dependency, tool and vendor bookends matched.

The configured policy readers agreed with full product admission and
all 15 isolated Python reader tests passed. The first reader launch
stopped before discovery because the namespace had no account entry.
The retry used one inert account with its home inside the namespace.
Coverage and mutation inventory execution remain open for this map.

| Record under `~/.local/share/nevrotick/nov28-audit-rust.GMIVpIWj/` | SHA-256 |
| --- | --- |
| `roughtime-policy-readback.json` | `2de1a3c9320a69d45bc108aa0fd951522d72ab5472c341660a2ecb6fbf7af9f9` |
| `roughtime-policy-controls-retry.log` | `d5801f2d8ca4fdbe6d6c7b627358f87fa6fcfb1fc0bb77332c905fadb66b68e0` |
| `bookends.json` | `44975ca25470a536c9afbc3698f251762643093aa88937fdb2885cd371c517fc` |
| `msvc-clippy.json` | `adad99e9b6b7e5f54fab3cdfd42660fb66d0fe68d3999a94102ef85a5bcd785d` |
| `gnu-auth-3.json` | `c39752a1cd6470d3cd4a2963e3f026b13c2ffcacd3ea7b39e215e34348a12a6b` |

### 3.53 TA-W07 rebind the OPEN-8 overlay after the authentication bracket change

OPEN-8/R-05/CRYPTO-04 and TA-C11/19/22. The original two-file Windows
point fixture now applies to the §3.52 product map. Independent replay
produced the 834-input overlay SHA-256
`f56ca5e8b481cfd834381ee63ea0666bc2f3ed388495a13342905d7783718104`.
The patch keeps the original fixture bytes and its single registration
in the current transport. The §3.47 and §3.51 overlays retain their
historical identities.

The private binder now checks the new overlay digest. Confined Linux
controls refused the preceding map, one restored annotation hash and a
map missing the fixture. The current map reached the absent staged
fixture check. With the fixture present and the original PKI, collector
and peer digests verified, it reached the deliberately absent Engine image.
The source bookends matched and scratch retired. The current cross-link,
exact PE/PDB binding, cold Windows observations and complete-interval
receiving path remain open.

| Record under `~/.local/share/nevrotick/open8-auth-bracket-rebind.hr8f1bol/` | SHA-256 |
| --- | --- |
| `source-rebind.json` | `730b9531bfe9d5e740b8ff8aa9c5f32458ed1eadfff8d517de3fcdf6c58a93f0` |
| `binding-controls.json` | `aec895fbd010fb4c2af29a6f7897fddcb51547049eeef098be4b42ed1f7e82fe` |
| `expected-fixture-source.json` | `52770aa3cf630d5752340f9a90ab6825187f023de2e0a8863b2dc6d2737ed372` |
| `open8-fixture.patch` | `96b2c6eb87739da3aae3e251c6fa8f3203c9ba6362e158198155413530595875` |
| `bind-points.py` | `84651a168ed7b5293de5d7cd4831951dc484042eaa5e4f8c0a6212ac8ddd4a53` |

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

These refinements remain **open** under the existing cases and their canonical
CHECK mappings. The
[profile/fixture evidence](../docs/plans/time-assurance-integration.md#823-fixture-provenance-and-remaining-qualification)
records the Reported status and independent byte checks for RFC 10049 erratum
9238. Section 3.21 owns the focused verifier controls for those bytes. Retain the
legacy contexts as a rejection case and use independently checked v1 fixtures
for each positive behavior. Preserve fixture provenance and final RFC contexts.

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
qualify any runtime. At that original 3 November documentary checkpoint,
TA-W02–TA-W09 and all TA-C cases were unexecuted. The later implementation and
development-check records in §3 and the current assignment table govern their
present V1 or post-V1 status; the original receipt is not a current absence-of-work
claim.

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
