# NTPv5 benefits and adoption research

**Reviewed:** 21 November 2026. **Status:** proposed research; unscheduled.
**Coordinator:** [existing protocol task](../../tasks/s0.1-protocol-comparison.md).

## 1. Status, authority and inspected revisions

Recommendation: preserve NTPv4+NTS and the Chrony-derived Core; investigate useful
measurement improvements separately from a new wire protocol. F323 offers an
experimental route to remote frequency information without NTPv5. Neither
candidate is adopted, implemented or a new V1 acceptance gate.

[Architecture §§7–10](../windows-time-architecture.md#7-chrony-derived-numerical-and-protocol-core),
DEC-1, [owner decisions](../nevrotick-owner-decisions.md),
[leap policy](../plans/leap-second-policy.md) and the
[current continuation](../plans/development-plan.md#7-next-executable-task-for-a-development-agent)
govern. This review supplies evidence and proposed contracts, not competing policy.

| Surface | Inspected identity and evidence limit |
| --- | --- |
| NevroTick comparison | [78f8ae931c7139d37e0ff36ed18e41dabf8c72ff][N0], historical analysis baseline. |
| Documentation integration | [65b8e9a253fc6440beab61ebd53681940997380a][N1], 21 November: relevant wire/admission/quality owners unchanged; managed receipts and Engine service composition extended. Scoped source reconciliation, not repeated runtime qualification. |
| ntpd-rs | [917fa4e4fb05cd5ab7a2567aa49cc521977b881b][R0], 20 November; experimental main comparison, not replacement of the previously selected stable reference. |
| IETF | [draft-ietf-ntp-ntpv5-10][S1], 29 October; active Internet-Draft, not a published RFC. |
| Chrony | [4.9, 8df4f1263e206585e6c4e61352d042c2dd12916a][C0]; implementation reference, not an independent truth oracle. |

No new implementation, fuzz/mutation campaign, live peer query, Windows probe or
accuracy benchmark was executed for this documentation update. Earlier receipts
retain their original candidates and scope.

## 2. Protocol comparison and benefit priorities

Three separate questions govern adoption: **protocol** determines transmitted
information; **authentication** establishes its origin/integrity; **estimation**
determines what measurements justify. A newer protocol or valid signature does
not prove accurate UTC, source independence or correct Windows actuation.

This compact draft reading aid is not an implementation specification [S1]. The
[authors' IETF 127 update][S2] separately records removal of leap announcements,
replacement of Bloom filters by weighted reference-ID lists, and new source queries.

| Draft area | Current facts |
| --- | --- |
| Scope/header | Client/server only; no prescribed estimator; distinct cookies; explicit era/timescale. Flag bit 0 means **unsynchronized**; former leap bits reserved. |
| Metrics | Root fields use Q4.28, finite below 16 seconds with an infinity sentinel; timestamp64 remains Q32.32. |
| Framing | Extension length excludes alignment padding; total packet alignment remains four bytes. |
| Optional information | Monotonic/secondary receive timestamps, server information, reference timestamps, weighted reference IDs and time-source queries. |
| Polling | Poll denotes minimum interval; 127 requests cessation, superseding RATE/DENY encoding. |

Pinned ntpd-rs declares **draft -09**, retaining leap bits, `synchronized` bit
semantics, `LeapSmearedUtc` and Bloom-filter handling. Its server response path
sets UTC, era zero and no interleaving; the inspected client measurement path
does not consume era/timescale metadata. These bounded observations come from
[header/response code][R1], [reference IDs][R2] and [source handling][R3]. They
establish no production qualification. Copying that codec would not implement
-10. Packet layering and test organization remain useful comparisons; controller
adoption would reopen DEC-1.

The pinned [alpha changelog][R4] records substantial refactoring and temporarily
unavailable observability/configuration validation. [`ntp-proto`'s README][R5]
offers no public-API stability guarantee. Optional behavior remains partial in
the paths above; each selected feature needs its own evidence. The [packet
parser][R6] explicitly rejects missing or mismatched draft identification, so
configuration alone cannot make a -09 peer qualify a -10 implementation.

NevroTick's [response owner][N2] admits v4 protected replies and retains the exact
authenticated header. [Conditional quality][N3] derives advertised root metrics
from that header. No F323 or v5 semantic consumer was identified in those paths.
The newer [Engine worker][N4] composes bounded managed-loopback acquisition with
Stop/join ownership; this establishes neither production positive mapping nor
native time accuracy.

| Candidate | Engineering priority and reason |
| --- | --- |
| Remote frequency information | Highest experimental interest: test whether separating server phase corrections improves drift estimates and holdover. |
| High-resolution roots | Useful precision preservation; finer encoding cannot shrink physical capture/network uncertainty. |
| Interleaving | Evaluate after timestamp provenance; [RFC 9769][S3] already provides an NTPv4 route. Separate hardware capture from interleaving benefits. |
| Era/timescale handling | Correctness preparation; initially reject unsupported scales instead of silently mixing them. |
| Capability/reference-time diagnostics | Lower priority; explanations still require current source ownership and honest freshness. |
| Upstream ID/source lists | Possible correlation hints, especially for future serving; self-report is not independent provenance. |
| Network correction | Controlled-network research requiring supporting equipment and separate threat/evidence analysis. |

These priorities are this review's assessment, not a scheduling decision.

## 3. F323 over NTPv4 and NTS

Chrony's [4.9 manual][C1] labels F323 experimental, recommends known chronyd
4.2-or-later peers, and warns that unknown extensions can suppress replies from
other implementations. F323 is not standardized NTPv5. Confirm capability for
the actual delegated NTP endpoint, not merely its NTS-KE hostname.

The pinned [wire declaration][C2] defines a **28-byte extension**: four-byte
header plus this 24-byte body:

| Field | Representation |
| --- | --- |
| Magic | `0xF5BEDD9A`, 32 bits |
| Root delay and dispersion | Two unsigned Q4.28 values |
| Monotonic receive timestamp | Q32.32, 64 bits |
| Remote monotonic epoch | 32 bits |

[Chrony's exchange implementation][C3] adds F323 before authentication, derives
monotonic/wall receive-interval differences within matching epochs, and corrects
retained history before adding another sample through its [statistics owner][C7].
Missing usable metadata clears
auxiliary continuity. Its [NTS authenticator][C4] covers preceding packet bytes;
[response admission][C5] verifies authentication before accepting timing. This
supports the combined experiment by static inspection, not executed interoperation.

The useful idea requires estimator integration. Remote phase corrections can
otherwise resemble oscillator error; separate frequency information might reduce
that contamination while preserving genuine frequency changes. Merely decoding
the field supplies no benefit. It is not a jitter-free oscillator reference,
trusted UTC date, or replacement for local QPC/capture qualification.

Network correction is separate. Chrony 4.9 documents `10A` for NTP-over-PTP;
old `F324` is a configuration alias, not continued support for the old wire field.
Its different hardware/transport prerequisites [C1] remain outside this proposal.

## 4. Proposed source-owner integration contract

These obligations describe a possible future slice, not adopted APIs or present
capabilities.

| Existing owner | Proposed responsibility if selected |
| --- | --- |
| [ntp-wire framing](../../crates/ntp-wire/src/datagram.rs) | Bounded inert projection; no authentication/history authority. |
| [ntp-wire request writer](../../crates/ntp-wire/src/nts_request.rs) | Extend `prepare_nts_client_request` deliberately: its current v4-only UID/cookie/placeholder layout has no auxiliary-field input. Include F323 in field counts, associated-data length and packet budgets before sealing. |
| [Engine request](../../crates/nevrotick-engine/src/nts_ke_experiment/request.rs) | Bind requested capability to source/association and packet budget; insert before authentication. |
| [Engine response][N2] | Optional metadata from the verified authenticated prefix; exact header remains unchanged. |
| [Association ledger][N5] | Commit with the same one-use receipt, cookie transition and final guards. |
| [Managed receipt][N6], [measurement](../../crates/nevrotick-engine/src/measurement.rs), [quality][N3] | Carry original evidence to source-specific numerical interpretation, preserving elapsed bounds and identity. |
| [Core history](../../crates/time-core/src/regression/history.rs) and source lifecycle | Own bounded continuity and atomic history transformation. |

**Admission.** Enable through explicit experimental per-source configuration.
Validate one requested F323 before the first authenticator, exact
type/length/magic and numerical domain.
Malformed or duplicate opted-in authenticated fields reject that response without
partial metadata/cookie/observation publication. Unrequested fields remain inert.
Well-framed trailing unauthenticated fields, including apparent F323, retain
architecture §7.4's discard semantics; do not import Chrony's stricter
last-authenticator rule.

Authentication is insufficient: preserve endpoint/socket, issued source, UID,
association/configuration/clock generations, send completion and processing
deadline. Revalidate at serialized ledger commit. Metadata cannot refresh age,
become another vote, re-enable a stopped source or authorize clock effects.
Control-only KoD/NAK supplies no frequency observation.

**Custody.** Extend the original private receipt chain; never attach detached
scalars afterward. Retain T1/T4 capture brackets, authenticated T2/T3,
arrival/processing instants and guards through joined outcomes,
`ManagedProtectedReceipt`, series/address-round consumption and rejection.
Respect the existing **512-byte inline terminal ceiling** [N6]; enlargement needs
explicit layout review. Abnormal completion retains historical facts, not a new
successful sample.

**Numerical meaning.** Keep header and F323 roots as distinct tagged evidence.
**Never overwrite or reconstruct the authenticated header** so a v4 consumer
interprets Q4.28 bits. The quality owner selects one documented source-specific
representation, retains provenance, converts conservatively and preserves
local/network uncertainty. Never silently select the smaller conflicting value
or mix sources' roots. Derived representation spacing improves from approximately
15.259 microseconds (2^-16 seconds) to 3.725 nanoseconds (2^-28 seconds), **4096×
finer encoding**, without guaranteeing any physical accuracy improvement.
Chrony's [conversion implementation][C6] rounds positive encoded bounds upward
and decodes `0xffffffff` as a large saturation sentinel (approximately 65,536
seconds), not an ordinary value near 16 seconds. Infinity/unsupported range stays
explicit; conversion cannot wrap into an acceptable small bound. Freeze the consistency/disagreement
policy before effectful integration.

**Continuity.** Model absence as `Option<RemoteMonotonicEpoch>` or equivalent:
epoch zero is possible, not an absence marker. Remote epoch, source identity,
local clock generation and receipt identity are different domains. Compare only
successive accepted observations from the same eligible association/epoch.
Missing field, invalid monotonic timestamp, epoch/source/path replacement,
suspend or relevant mapping invalidation breaks auxiliary continuity. The next
usable observation establishes a baseline without splicing an old delta.
Rejected traffic cannot mutate a healthy series; accepted absence can clear
auxiliary state while preserving an otherwise admissible basic measurement.

The actual Core owner performs proposed remote-phase history correction
atomically, with independent sign/units/rounding expectations. Keep it separate
from DEC-11 transformations for acknowledged **local** clock effects. Remote
timestamps cannot acknowledge broker actions. Failure preserves valid prior
state or invokes existing explicit invalidation, never a partial rewrite.

**Bounds/fallback.** Charge framing, authentication and ignored-field work to
existing request/aggregate limits; include retained metadata and diagnostics.
Rejection refunds nothing and renews no deadline. A valid protected response
without F323 can use ordinary v4 metrics with auxiliary reset. If extension-bearing
requests receive no response, a bounded retry may omit F323 while retaining NTS,
backoff, endpoint policy and source accounting. No plain-NTP fallback, lower
quorum or weaker cookie rules follows.

## 5. Experimental NTPv5 compatibility contract

A later v5 experiment needs a version-specific codec and supported-draft identity,
not a version-bit edit to `RawHeader`. Draft -10 uses identification extension
`0xF5FF`, experimental NTS protocol `0x8001` and draft record `0x4001`.
Unknown/unrequested response flags are invalid; `NTP5DRFT` is the non-NTS draft
negotiation marker [S1].

[RFC 8915 §4.3][S4] binds exporter context to negotiated protocol ID. Bind
version/draft/AEAD/direction, keys, cookies and endpoint policy into one association;
test cross-protocol confusion. Permitted v4 fallback retains NTS and establishes
compatible association state, never reusing incompatible keys or cookies.

NevroTick's existing [v4 negotiation/exporter owner](../../crates/ntp-wire/src/nts_negotiation.rs)
privately fixes protocol zero in `NtsKeNtpv4Negotiation::exporter_context`.
A selected v5 experiment must introduce a deliberate version-specific negotiation
and exporter path; relabelling an existing v4 association cannot provide it.

Use independent layouts for length versus padded extent and version-specific
poll/control interpretation. Bound remote polling demands under existing source
policy. Control messages supply no time or successful-observation freshness.
Tests against -09 ntpd-rs must distinguish expected incompatibility from defects;
that peer alone cannot qualify -10.

Secondary timestamps need trusted conversion before joining unsmeared UTC.
Extra era bits establish no date correctness: supported true-date interval,
quorum and leap authority remain. Never infer permanent 37-second conversion or
remove v4 leap history because v5 changed its header.

Network corrections sit outside authentication and need bounded validation;
retain the uncorrected root-delay bound [S1]. Keep that optional feature disabled
until its hardware, threat and measurement case is selected. NTS does not prove
arbitrary path-delay correctness.

## 6. Independent verification and measurement

Before execution, existing protocol/Engine/Core owners select finite inputs,
time/case/retry budgets, candidate identity and observable exit. Reuse
CHECK-6/7/8/9/11/14/21/25/27. This review creates no execution permission or new
gate; applicable coverage, critical-file mutation and native requirements remain.

| Obligation | Independent positive and adversarial controls |
| --- | --- |
| Wire/crypto | Literal vectors; exact F323 versus truncation, magic/length mismatch, duplicates, unsolicited/trailing fields. Tampered prefix fails; inert suffix cannot change facts. |
| Custody/currentness | Receipt accepted once; stale UID/generation, deadline crossing, late Stop, wrong endpoint and abnormal join publish nothing new. Healthy current neighbor progresses. |
| Root conversion | Independent finite/infinity/rounding boundaries and distinct encodings; unchanged header/source survive admission and refusal. |
| Continuity | Same epoch, epoch zero, accepted absence, changed epoch, restart/path replacement; no cross-boundary delta or rejected-packet mutation. |
| Estimator | Independent UTC/oscillator trajectories with server phase actions and genuine frequency changes; stable and correctly transformed histories have equivalent physical conclusions. |
| Resource/control | Maximum fields/bytes, floods, absent F323, retry/cookie exhaustion retain charges, backoff, NTS and agreement. |
| v5 isolation | Draft mismatch, flag inversion, padding, era rollover, mixed scales, exporter vectors and downgrade attempts. |

Use property tests for bounded transformations/invalidation histories, negative
and direct-consumer controls for bypasses, bounded fuzzing for hostile framing
and transitions, and meaningful mutants for selected guards. Independent formulas
and physical trajectories are primary oracles; upstream programs are supplemental.
Shared inverse round-trips, copied tests and parser success do not establish
estimator correctness.

Compare identical controlled peers/capture paths using NTPv4+NTS baseline,
F323 alone, then interleaving as a separate factor. Predeclare tolerances;
measure UTC/frequency error, holdover drift, convergence, false confidence,
rejections and CPU/memory/network cost under loss, asymmetry, server corrections
and local power transitions. Distinguish simulation from supported Windows
capture/actuation evidence. Chrony's [hardware example][C8] combines several
optimizations; it predicts neither isolated F323 benefit nor NevroTick accuracy.

## 7. Scheduling, adoption and remaining uncertainty

The [roadmap](../../ROADMAP.md) remains **V1 → MATH-0 → TRUST-0 → MT-0 →
Khronos/Ananke → GEO-0 → Android**. F323/v5 remain unscheduled, not automatic
MATH-C01 additions. Current execution follows development §7: the newer service
composition checkpoint precedes the complete first portable W11-W08 logger
catalogue/routing stage with all five obligations, not an obsolete Core-only continuation.

Adoption needs a bounded assignment in the existing protocol task, current source
revalidation, frozen metadata/currentness contracts, independent review and
measured benefit. Record adoption, revision or rejection there. Material policy
or controller changes follow the existing owner-decision process. This review
selects no dependency/provider/default server, security relaxation, privilege or
packaging change.

Open questions: compatible endpoints; benefit amid Windows capture noise;
source-specific history integration; metadata consistency; maintenance across
draft revisions; and resource cost. Evidence is strongest for pinned source and
layout distinctions, weakest for product accuracy benefits without measurements.

## 8. Primary sources

- [N0: historical baseline][N0]; [N1: integration baseline][N1].
- [N2: protected response][N2]; [N3: quality][N3]; [N4: Engine worker][N4];
  [N5: ledger][N5]; [N6: managed receipt][N6].
- [R0: ntpd-rs commit][R0]; [R1: v5 header/server][R1];
  [R2: reference IDs][R2]; [R3: source consumer][R3];
  [R4: alpha changelog][R4]; [R5: crate API status][R5];
  [R6: draft-identification admission][R6].
- [S1: draft -10][S1]; [S2: authors' update][S2];
  [S3: RFC 9769][S3]; [S4: RFC 8915 key extraction][S4].
- [C0: Chrony commit][C0]; [C1: versioned manual][C1];
  [C2: wire declaration][C2]; [C3: exchange/history][C3];
  [C4: associated data][C4]; [C5: NTS admission][C5];
  [C6: Q4.28 conversion][C6]; [C7: history adjustment][C7];
  [C8: hardware example][C8].

[N0]: https://github.com/DavidOsipov/NevroTick/commit/78f8ae931c7139d37e0ff36ed18e41dabf8c72ff
[N1]: https://github.com/DavidOsipov/NevroTick/commit/65b8e9a253fc6440beab61ebd53681940997380a
[N2]: https://github.com/DavidOsipov/NevroTick/blob/65b8e9a253fc6440beab61ebd53681940997380a/crates/nevrotick-engine/src/nts_ke_experiment/response.rs
[N3]: https://github.com/DavidOsipov/NevroTick/blob/65b8e9a253fc6440beab61ebd53681940997380a/crates/nevrotick-engine/src/quality.rs
[N4]: https://github.com/DavidOsipov/NevroTick/blob/65b8e9a253fc6440beab61ebd53681940997380a/tools/windows-service-fixture/src/engine_worker.rs
[N5]: https://github.com/DavidOsipov/NevroTick/blob/65b8e9a253fc6440beab61ebd53681940997380a/crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs
[N6]: https://github.com/DavidOsipov/NevroTick/blob/65b8e9a253fc6440beab61ebd53681940997380a/crates/nevrotick-engine/src/nts_ke_experiment/managed_preview.rs
[R0]: https://github.com/pendulum-project/ntpd-rs/commit/917fa4e4fb05cd5ab7a2567aa49cc521977b881b
[R1]: https://github.com/pendulum-project/ntpd-rs/blob/917fa4e4fb05cd5ab7a2567aa49cc521977b881b/ntp-proto/src/packet/v5/mod.rs
[R2]: https://github.com/pendulum-project/ntpd-rs/blob/917fa4e4fb05cd5ab7a2567aa49cc521977b881b/ntp-proto/src/packet/v5/server_reference_id.rs
[R3]: https://github.com/pendulum-project/ntpd-rs/blob/917fa4e4fb05cd5ab7a2567aa49cc521977b881b/ntp-proto/src/source.rs
[R4]: https://github.com/pendulum-project/ntpd-rs/blob/917fa4e4fb05cd5ab7a2567aa49cc521977b881b/CHANGELOG.md
[R5]: https://github.com/pendulum-project/ntpd-rs/blob/917fa4e4fb05cd5ab7a2567aa49cc521977b881b/ntp-proto/README.md
[R6]: https://github.com/pendulum-project/ntpd-rs/blob/917fa4e4fb05cd5ab7a2567aa49cc521977b881b/ntp-proto/src/packet/mod.rs
[S1]: https://datatracker.ietf.org/doc/html/draft-ietf-ntp-ntpv5-10
[S2]: https://datatracker.ietf.org/meeting/127/materials/slides-127-ntp-ntpv5-update-00
[S3]: https://www.rfc-editor.org/rfc/rfc9769.html
[S4]: https://www.rfc-editor.org/rfc/rfc8915.html#section-4.3
[C0]: https://github.com/mlichvar/chrony/commit/8df4f1263e206585e6c4e61352d042c2dd12916a
[C1]: https://chrony-project.org/doc/4.9/chrony.conf.html
[C2]: https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/ntp.h
[C3]: https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/ntp_core.c
[C4]: https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/nts_ntp_auth.c
[C5]: https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/nts_ntp_client.c
[C6]: https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/util.c
[C7]: https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/sourcestats.c
[C8]: https://chrony-project.org/examples.html
