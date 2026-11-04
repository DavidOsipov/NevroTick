# NevroTick — leap knowledge, silent sources and Windows ownership

**Document:** NV-LP-001, revision 1.0, 4 November 2026<br>
**Status:** selected engineering design; implementation and native qualification pending<br>
**Authority:** [architecture §10.5](../windows-time-architecture.md#105-leap-seconds-and-time-scales), OPEN-7; [OD-09](../nevrotick-owner-decisions.md#13-od-09--leap-policy-and-silent-servers) records the delegated research/documentation request<br>
**Execution:** [S1.4 leap-policy task](../../tasks/s1.4-leap-policy.md), existing S1.4/P2/S0.4/P4 gates

## 1. Selected outcome and scope

NevroTick v1 will carry a reviewed, immutable leap-history dataset and a separate,
effective-dated UTC policy in its authenticated release. Correct unsmeared UTC
servers remain usable when **every server sends LI=0**. Received leap indicators
are claims to check; they neither create an event nor replace local authority.

The **Windows kernel is the sole executor of explicit leap events**. NevroTick
does not schedule an additional OS leap, change global leap configuration or
manufacture a ±1-second correction. It protects ambiguous exchanges and clock
effects around a possible event, then corrects the actually measured residual
using fresh ordinary agreement and the existing broker limits. If Windows event
knowledge is not established, exact UTC during the event is not guaranteed;
visible uncertainty and qualified subsequent convergence are the v1 contract.

Future-calendar expiry alone does not stop ordinary synchronization. Outside
ambiguous boundaries, fresh UTC observations may continue to discipline the
clock with a persistent **future leap schedule unknown** qualifier. An unknown
TAI−UTC conversion is a different fact from unusable NTP UTC observations.

This selects the design David requested on 4 November; it does not assert that
he separately approved a new clock-authority envelope or that the product has
implemented it. Retain NTS/authentication policy, source independence and minima,
supported date/era checks, correction budgets, durable outcomes, sole ordinary
clock ownership, package trust and current execution holds. There is no new
runtime network updater, privileged parser, service, dependency or trust root.

### Current source and deliberate changes

The inspected application baseline is
[`7550f83292f959a6d0acbb2d396e964a6bcf4b01`](https://github.com/DavidOsipov/NevroTick/tree/7550f83292f959a6d0acbb2d396e964a6bcf4b01).
`ntp-wire` retains inert LI bits; `ntp-exchange` rejects LI=3 for ordinary time
and retains LI=0/1/2 in fixture observations. Its stratum-zero path remains
control-only. The engine loopback pilot also rejects LI=3 but is explicitly
unauthenticated and ineligible. `time-domain` unfolds eras against an explicit
window. None of those paths implements a leap calendar, leap-aware native
mapping or a production leap owner. Existing fixture results prove none of them.

This amendment replaces the former open-ended choice in §10.5 with the contract
above. It retains positive/negative modeling and the one-owner requirement,
adds independently sourced leap knowledge and ambiguity handling, and explicitly
does not adopt Chrony/NTPsec's dynamic LI-vote event authority. OPEN-7 remains
open for native mapping, event-window limits and the finite-slew failure envelope.
The current Core checkpoint and applicable selection prerequisites remain ahead
of dependent leap-policy implementation.

## 2. Verified evidence and what it establishes

| Evidence inspected on 4 November 2026 | Consequence for this design |
| --- | --- |
| RFC 5905 with verified erratum 4504: LI=0 is no warning; LI=1/2 concerns the current UTC day; LI=3 is unsynchronized. [L1] | Silence is not a complete calendar; NTS authenticates a claim, not its truth. |
| IERS Bulletin C 72, dated 6 July 2026: no leap at the end of December 2026; UTC−TAI is −37 s from 1 January 2017 until further notice. The inspected IANA copy of `leap-seconds.list` has `#@ 4023129600` (28 June 2027). [L2] | Preserve the exact historical and announcement scope. A file-maintenance date is not an indefinite no-event rule. |
| CGPM Resolution 5 (2026) makes continuous UTC effective on **20 May 2027**, with maximum \|UT1−UTC\| of 3,600 s. [L3] | Carry this separately sourced, effective-dated rule. It causes no clock impulse, no epoch reset and no 37-second correction. Do not generalize it to all historical dates or irrevocable eternity. |
| RFC 9636 separates leap-table expiration from past corrections and addresses non-expiring tables after discontinuation; its format is **TZif**. IANA 2026f updates `zic` accordingly. [L4] | Do not misapply a TZif rule as a mandatory change to every text `leap-seconds.list` parser. |
| Chrony consults a local leap database when the selected LI aggregate is normal; its documentation explicitly covers sources without announcements. Its separate selectable-source majority can still announce an event. [L5] | Reuse the independent local-knowledge idea. Do not copy a source-majority override of the reviewed continuous-UTC rule. |
| NTPsec preserves an expired table; dynamic appends are prohibited before its expiry and cannot rewrite previous history. [L6] | Separate historical validity, future authority and source learning. Expiry has semantic effects beyond log noise in that implementation. |
| ntpd-rs v1.9.0 explicitly tests all-LI=0 as `NoWarning` and conflicting votes as no consensus; the inspected controller has no local-table fallback. [L7] | Consensus alone does not solve missing announcements. Do not transplant unrelated fixed-offset shortcuts. |
| NIST documents repeated-last-second and repeated-midnight realizations; NTP packets do not describe the realization. [L8] | Guard the full exchange span and mapping, not just the one instant named by a local calendar. |

The Habr article motivated this review. Its reported older daemon versions were
checked against pinned upstream code, not rerun here. Current upstream inspection
also used GitHub MCP. Windows research used Microsoft Learn MCP search/fetch/code
search, supplemented by official Microsoft articles and the Microsoft W32Time
validation guide. These are source findings, not native experiments. No official
future IERS text-file formatting policy was established by this work.

## 3. Knowledge and distribution contract

The following are conceptual domain states, not claims that these Rust types
already exist. Keep them separate even if a shared module holds the data.

| Owner / value | Required meaning |
| --- | --- |
| `time-domain`: historical ledger | Ordered effective UTC transitions, signed changes and baseline TAI−UTC; exact supported interval, authoritative provenance, revision and digest. The 1972 baseline is distinct from subsequent ±1-second events. Pre-1972 conversion is outside this ledger. |
| `time-domain`: future coverage | Explicit `Covered` intervals, `Unknown` outside them, and a separately sourced `ContinuousFrom` rule. Imported finite expiry, absent expiry and malformed input are different states. A missing field cannot construct unlimited authority. |
| `time-core`: current leap context | Policy revision, complete admitted true-date/era interval, compatible scale segment, possible event identity, guard phase and mapping generation. Construction requires the current context; a once-valid value cannot certify future freshness. |
| Protocol/engine: observations | Raw LI, authenticated identity/provenance, declared scale, full possible exchange span, observation age and clock/association/policy generations. A source claim is never a privileged command. |
| Windows adapter/broker: observed native state | Process presentation mode, supported conversion and adjustment behavior, ownership and actual outcomes. OS event knowledge describes OS behavior; it does not override authoritative UTC truth. |

### Release ingestion and update

Use one normalized, versioned dataset shared by the pure domain/policy consumers
and delivered in the **same authenticated complete release** as the application.
Prefer generated immutable data compiled from reviewed, repository-pinned inputs;
there is no arbitrary runtime leap-file path or hot download in v1. The selected
package/build provenance binds the exact dataset revision and digest. This does
not invent a second signature authority or assume that an IERS text checksum is
a signature. `#h` is an unkeyed consistency checksum.

The offline importer must bound bytes, records, integer/date ranges and work;
reject malformed, duplicate/conflicting and out-of-order records; distinguish a
baseline from signed changes; and use wide non-wrapping seconds for file dates.
A file timestamp is not the wrapping 32-bit seconds field of an NTP packet.
Freeze these limits in the first pure implementation slice before accepting its
parser. Reproducible generation and independent date/offset fixtures must agree.
No build retrieves mutable network data implicitly.

Absent or expired metadata does not erase valid historical entries. Do not invent
expiry in 2100, silently map absence to 1900/1970, or accept an unsupported
`infinity` spelling. Non-expiring future behavior is derived from the explicit
reviewed rule, within the supported product date range. Calendar review dates
remain maintenance information; they do not create time-based synchronization
shutdowns after continuous UTC applies.

A malformed replacement leaves the previous admitted release/data intact. A
rollback must follow existing authorized package/version transitions and cannot
reset protected floors, correction budgets or consumed authority. Historical
corrections require explicit reviewed supersession with retained provenance;
network LI cannot perform them. A corrupt authenticated-package payload is an
integrity failure under existing package policy, not permission to run altered
code with an `Unknown` calendar. An otherwise valid supported bundle may honestly
contain unknown future coverage.

Whole-release withdrawal and corrective delivery follow the
[release policy](../release-policy.md#6-mandatory-state-compatibility-and-owned-recovery).
Its operational procedures preserve this dataset's provenance and the separate
mandatory-state owner; they supply no runtime calendar updater or native pass.

`ContinuousFrom` establishes absence of subsequent events; it does not fill an
earlier coverage gap. If an intervening offset is unknown, current TAI−UTC remains
unknown until independently established, even inside the continuous regime.
Ordinary event-free UTC tracking remains a separate, potentially usable result.

### Date and bootstrap

Evaluate rules against the **entire independently supported true-date interval**,
including admitted era hypotheses. A host clock wrong by a century must not make
the program select a convenient regime, declare metadata fresh or assume TAI−UTC
is always 37 s. The amount by which Windows is wrong is distinct from the
supported true-date interval.

If all admitted hypotheses imply the same event-free segment and admissible
target, an interval straddling the continuous-UTC rule's date is not itself a
reason to block. If a hypothesis changes the interpretation or crosses a possible
event, withhold that sample's clock authority. Keep provisional bootstrap
acquisition, NTS transport and authenticated release admission available; require
no circular proof from an already discipline-eligible sample. Independent date
evidence, fresh measurements and monotonic passage can resolve ambiguity. The
existing bootstrap policy still grants no direct leap vote or clock-write right.

## 4. Source and LI admission

Admit **unsmeared UTC** only to this v1 population. Source-scale provenance is
configuration/operator evidence. `Smeared` and `Unknown` are separate ineligible
states; neither NTS nor LI=0 nor agreement with another server proves the scale.
Unknown announcement behavior alone does not mean unknown time scale. A provider
may be explicitly unsmeared while never announcing LI. [L8, L9]

| Input / current knowledge | Selected behavior |
| --- | --- |
| Stratum 0, any LI | Existing bounded control admission; no ordinary time or leap authority. |
| LI=3, any calendar | Reject ordinary time observation as unsynchronized. A calendar cannot repair it. |
| LI=0, covered event or no event | Ordinary candidate under all existing checks. Silence neither cancels an event nor proves no event. |
| LI=1/2 matching covered event/day | Corroborating claim; ordinary candidate outside the guard. No additional event action. |
| LI=1/2 contradicting covered event/day or continuous UTC | Reject that observation from discipline and diagnose the source conflict. Other healthy identities retain their votes. No global reset, policy rewrite or leap scheduling. |
| LI=0/1/2 with unknown future coverage | Retain the claim and uncertainty. Same-segment UTC tracking may continue outside the calendar-derived guard; no dynamic LI-majority event learning in v1. Impossible day/claim combinations still fail the selected rule. |
| All otherwise eligible sources send LI=0 | Normal acquisition/tracking, including independent local event protection. Never require a nonzero-LI quorum. |
| All remaining sources conflict or lack ordinary agreement | Existing bounded holdover/unsynchronized outcome; no reduced minimum, old-predictor vote or single-peer fallback. |

The conservative v1 choice is to exclude a demonstrably contradictory observation,
rather than use its timestamp while ignoring its LI. This is not a permanent
identity ban: fresh consistent observations can re-enter through ordinary
admission. In unknown future coverage, disagreement with an absent schedule is
not a demonstrated contradiction. A single faulty peer must not discard healthy
peers, move the protected interval or renew it through repeated announcements.

LI is retained through staged admission and diagnostics. It is never normalized
out of authenticated bytes before verification. Release policy, source-scale
provenance and current mapping generation must accompany derived eligibility.

## 5. Event guard, holdover and reacquisition

Derive event windows from local knowledge, not from majority LI. For a known
event, protect its boundary. If future coverage is unknown in the leap-capable
regime, conservatively protect **every possible UTC month-end**, not only June and
December. This monthly rule is our conservative policy; RFC 7164's related RTP
guidance is supporting precedent, not an NTP implementation mandate. [L10]
Within covered continuous UTC there is no recurring legacy guard.

The guard must cover both repeated-last-second and repeated-midnight realizations,
the Windows presentation behavior, date/mapping/capture uncertainty, maximum
exchange lifetime and bounded actuation latency. Freeze finite bounds from the
admitted profile in S1.4 and qualify their native assumptions in S0.4/P4. Do not
hardcode an unexplained one-second timeout. An unbounded date/mapping uncertainty
cannot be hidden by selecting a small guard; it blocks the dependent clock write.

1. **Before the guard:** stop admitting new phase/rate proposals and reconcile or
   cancel the existing transient plan through the normal durable outcome rules.
   Retain only the owned, qualified frequency base and bounded drift prior. A
   cancellation request or elapsed timer is not proof that cancellation happened.
2. **Enter once per boundary identity:** advance the relevant mapping generation,
   invalidate crossing exchanges/proposals and mark the minimum post-guard
   acquisition instant. Repeated LI cannot repeat this transition or extend its
   budget. Actual unexpected discontinuities still use the ordinary generation
   invalidation path.
3. **During the guard:** publish Holdover and growing uncertainty; accept no new
   clock corrections or ordinary offset fit across the ambiguous interval.
   Network acquisition and diagnostics may continue with inert/provisional data.
   Do not learn an apparent leap as oscillator drift or a successfully applied
   broker effect.
4. **After the guard:** obtain fresh exchanges initiated after the guard whose
   entire possible T1–T4 span lies in one compatible event-free segment. Final
   admission checks span, age and generations even for delayed completions.
   Obtain the existing fresh independent quorum; timeout alone restores no vote.
5. **Reconcile:** correct only the measured residual through existing bounded
   slew/step authority. If Windows already handled the event, an approximately
   zero residual causes no synthetic extra second. If it did not, the measured
   error is corrected normally; budgets/step limits may require visible continued
   degradation or existing approval rather than a leap-specific exemption.

Use monotonic elapsed/deadline clocks, with the existing sleep/active-awake
distinction. Sleep, restore, restart or failed cancellation can invalidate the
window/mapping; they never replenish authority. Once ordinary admission succeeds,
unknown future coverage alone allows **Tracking with a qualifier**, not perpetual
Holdover. No finite outage or accuracy number is promised until oscillator,
uncertainty, network, cancellation and recovery limits have been qualified.

## 6. Windows contract and limits

Modern Windows has native leap support; the old Windows Time KB is not a complete
Windows 11 specification. Microsoft describes kernel knowledge arriving through
NTP, Windows Update and Hyper-V integration. Stopping W32Time does not establish
that this knowledge is absent or that native handling is disabled. Preserve the
existing transactional W32Time takeover and VM/foreign-owner checks. [L11]

The selected backend leaves global leap settings and OS history unchanged. It
does not call guessed NT event setters, edit leap registry data, start W32Time
temporarily or parse `w32tm` output as a production protocol. The documented
`w32tm /leapseconds /getstatus /verbose` command is useful **operator/lab evidence**;
its stable machine schema and behavior with W32Time disabled were not established.
`NtQuerySystemInformation(SystemLeapSecondInformation)` exposes Enabled/reserved
flags, not a supported exact event-list interface. [L11, L14]

For v1, qualify the **default compatibility presentation** in the engine and
broker: the process does not opt into `:60`; the adapter records/verifies its own
presentation setting with the documented process APIs. Unexpected or unsupported
presentation cannot silently reuse a mapping. During a positive native event,
this mode stretches the displayed 59th second over two seconds; the guard excludes
that interval. This is process presentation, not a global disable switch. Imported
libraries and future changes to opt-in behavior require the same qualification.
Use the current flag definition rather than the old article's zeroed-flags
opt-in example. [L12]

Keep Windows FILETIME ticks, civil UTC, NTP coordinates and monotonic durations
as distinct types. Windows conversions account for OS-known leap data; raw
FILETIME is neither universal TAI nor an always-correct fixed-offset NTP mapping.
Use and qualify documented Windows conversions at the adapter, including preserved
sub-millisecond precision outside guards, range checks, process mode and mapping
invalidation. Do not compensate by adding/subtracting a universal 37 s. [L13]

Ordinary adjustment remains the existing precise-adjustment/step backend.
Query and reconcile **mode and value**, actual trajectories and durable outcomes;
kernel event execution is not a broker acknowledgement. A wrong or unexpected
native event is an external discontinuity, followed by mapping invalidation and
fresh acquisition. The documented interface set does not establish prevention of
every such event. QPC supplies elapsed measurements, not UTC truth. [L15]

**Native gate:** OPEN-7 is narrowed to supported-build mapping/presentation,
event knowledge/behavior, finite guard/cancellation/recovery bounds and ownership
evidence. Exact event-time UTC requires separate proof of appropriate OS event
knowledge and behavior. If absent, v1 reports that limit and uses the guard plus
residual-recovery contract; it does not install an undocumented second owner.

These prohibitions govern the product/runtime. A separately admitted disposable
Windows fixture may arrange synthetic OS events and enabled/disabled test images
under its bounded S0.4 experiment card, documented lab mechanisms and restoration
checks. That fixture is not shipped, and configuring a test condition supplies
no proof that the ordinary product observed or handled it correctly.

## 7. Implementation owners and observable exits

| Stage | Work and exit | Existing owners/checks |
| --- | --- | --- |
| Pure knowledge contract | Add bounded private history/coverage/rule constructors and reproducible pinned-data generation in the `time-domain` boundary. Freeze intervals and limits. Independent history, absent/expired metadata, malformed replacement and 2027 no-impulse fixtures discriminate failures from healthy controls. | S1.4 after S1.3 prerequisites; CHECK-7/25/26 |
| Pure policy and source integration | Add the decision table, guard transitions and provenance/generation contract in `time-core`; carry raw LI and full exchange span through `ntp-exchange` and the production engine admission path. All-LI=0 must succeed; no stale fit, downgraded quorum or announcement storm may authorize effects. | S1.4/P2; CHECK-6/7/8/11/14/25 |
| Windows adapter and broker | Implement the selected process-mode/mapping boundary and guard cancellation/reacquisition using supported APIs. Observe OS-handled/missing/unexpected event scenarios, both signs, actual residual and no duplicate action. | S0.4/P4 and W11-W02; CHECK-10/11/12/27 |
| Package, storage and presentation | Bind dataset provenance to the authenticated package; retain protected correction continuity through update/rollback; project coverage/conflict/guard/native-knowledge status to UI/export. | P3/P5/P6; CHECK-14/15/17/18/23/24 |

No stage creates a blanket dependency on completing all platform research before
the next pure task. No existing portable pass closes the Windows gate. The task
record owns the case matrix, implementation readiness and review evidence.

## 8. Sources and inspection pins

- **L1:** [RFC 5905 §7.3](https://www.rfc-editor.org/rfc/rfc5905.html#section-7.3) and [verified erratum 4504](https://www.rfc-editor.org/errata/eid4504); [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html) for authenticated NTP transport and its security scope.
- **L2:** [IERS Bulletin C 72](https://hpiers.obspm.fr/iers/bul/bulc/bulletinc.72), [IANA 2026f leap-seconds.list](https://data.iana.org/time-zones/tzdb-2026f/leap-seconds.list). Preserve raw source identity when constructing the future bundle.
- **L3:** [CGPM Resolution 5 (2026)](https://www.bipm.org/en/cgpm-2026/resolution-5), particularly the effective date and continuity decision.
- **L4:** [RFC 9636 §3.2](https://www.rfc-editor.org/rfc/rfc9636.html#section-3.2), [IANA tzdb 2026f release](https://www.iana.org/time-zones/releases/2026f).
- **L5:** Chrony `7fab5413c58eccc91d81d940ec75654fd23e1d36`: [documentation](https://github.com/mlichvar/chrony/blob/7fab5413c58eccc91d81d940ec75654fd23e1d36/doc/chrony.conf.adoc), [reference](https://github.com/mlichvar/chrony/blob/7fab5413c58eccc91d81d940ec75654fd23e1d36/reference.c), [sources](https://github.com/mlichvar/chrony/blob/7fab5413c58eccc91d81d940ec75654fd23e1d36/sources.c), [leap database](https://github.com/mlichvar/chrony/blob/7fab5413c58eccc91d81d940ec75654fd23e1d36/leapdb.c), [simulation 113](https://github.com/mlichvar/chrony/blob/7fab5413c58eccc91d81d940ec75654fd23e1d36/test/simulation/113-leapsecond). Comparison does not replace the project's adopted Chrony pin or admit more source code.
- **L6:** NTPsec `bea210dba1e05188051872ebb002d9aedc9f7f5f`: [leap database](https://github.com/ntpsec/ntpsec/blob/bea210dba1e05188051872ebb002d9aedc9f7f5f/ntpd/ntp_leapsec.c), [protocol](https://github.com/ntpsec/ntpsec/blob/bea210dba1e05188051872ebb002d9aedc9f7f5f/ntpd/ntp_proto.c), [tests](https://github.com/ntpsec/ntpsec/blob/bea210dba1e05188051872ebb002d9aedc9f7f5f/tests/ntpd/leapsec.c).
- **L7:** ntpd-rs v1.9.0, `46ec9bb4d5b6cb24f814f5543d85b9138afb4cba`: [combiner and tests](https://github.com/pendulum-project/ntpd-rs/blob/46ec9bb4d5b6cb24f814f5543d85b9138afb4cba/ntp-proto/src/algorithm/kalman/combiner.rs), [controller](https://github.com/pendulum-project/ntpd-rs/blob/46ec9bb4d5b6cb24f814f5543d85b9138afb4cba/ntp-proto/src/algorithm/kalman/mod.rs). The separately inspected main `18a5bd6a87a65033a41c36dc8b4b737c98b5523f` [conversion FIXMEs](https://github.com/pendulum-project/ntpd-rs/blob/18a5bd6a87a65033a41c36dc8b4b737c98b5523f/ntp-proto/src/time_types.rs) are not a general historical conversion contract.
- **L8:** [NIST Internet Time Service, leap seconds](https://www.nist.gov/pml/time-and-frequency-division/time-distribution/internet-time-service-its).
- **L9:** [RFC 8633 §3.7](https://www.rfc-editor.org/rfc/rfc8633.html#section-3.7), compatible leap-smear populations.
- **L10:** [RFC 7164 §5.1](https://www.rfc-editor.org/rfc/rfc7164.html#section-5.1), month-end possibilities and RTP timestamp ambiguity guidance.
- **L11:** Microsoft [Leap Seconds for the IT Pro](https://techcommunity.microsoft.com/blog/networkingblog/leap-seconds-for-the-it-pro-what-you-need-to-know/339811); compare the older [Windows Time KB](https://learn.microsoft.com/en-us/troubleshoot/windows-server/active-directory/time-service-treats-leap-second). The official [developer validation guide](https://github.com/microsoft/W32Time/blob/master/Leap%20Seconds/Validation%20Guide%20-%20Dev%20-%20Leap%20Second.docx) illustrates status output; no experiment was executed.
- **L12:** Microsoft [PROCESS_LEAP_SECOND_INFO](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/ns-processthreadsapi-process_leap_second_info), [SetProcessInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-setprocessinformation), [GetProcessInformation](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-getprocessinformation).
- **L13:** Microsoft [Leap Seconds for the AppDev](https://techcommunity.microsoft.com/blog/networkingblog/leap-seconds-for-the-appdev-what-you-should-know/339813); [file_clock](https://learn.microsoft.com/en-us/cpp/standard-library/file-clock-class?view=msvc-170) documents a different historical scope from a complete TAI ledger. This is not adoption of a C++ runtime or a timeless FILETIME formula.
- **L14:** Microsoft [NtQuerySystemInformation](https://learn.microsoft.com/en-us/windows/win32/api/winternl/nf-winternl-ntquerysysteminformation), including its internal-interface warning.
- **L15:** Microsoft [high-resolution timestamps](https://learn.microsoft.com/en-us/windows/win32/sysinfo/acquiring-high-resolution-time-stamps), [GetSystemTimeAdjustmentPrecise](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-getsystemtimeadjustmentprecise), [SetSystemTimeAdjustmentPrecise](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtimeadjustmentprecise); the existing architecture owns mode, privilege, recovery and no-finite-lease limits.
