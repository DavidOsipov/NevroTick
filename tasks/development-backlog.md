# Deferred development parents

**Status:** grouped future parents are DEFERRED. The existing
[S0.5 Slint continuation](s0.5-ui-comparison.md#active-goal--finish-the-complete-slint-gui)
now owns the implemented v0.4 fixture GUI and its separate remaining
backend/native/text-approval and integration gates. Earlier source-only queue
states are historical. This single grouped record
fills parent-task gaps in [the task map](README.md); it does not replace the
[development plan](../docs/plans/development-plan.md), existing active children
or canonical architecture. Unless explicitly marked active, rows below are future
cards; none is completed evidence.
Role owners are responsibilities, not staffing assignments.

Activate only the next bounded child when its named prerequisites and current
authorization hold. In that existing chain, freeze candidate/source/graph,
literal independent expected facts, valid/rejected controls, environment,
finite execution limits and cleanup before implementation. Record actual exit
observations and gaps through the [testing contract](../.agents/skills/testing/CONTRACT.md).
Do not pre-create one task file per stage or start a second plan.

The current [Core continuation and holds](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
govern throughout: blocked S134 stager/artifact; general host Windows execution
and host elevation/security/service/clock changes; separate AES fuzz no-retry
hold; optional-storage experiment pause. [Owned-Sandbox permission](windows11-integration.md#419-owner-ready-sandbox-scope--3-november-2026)
is scoped management/readiness and separately admitted guest work, not a release
of those holds. UI-M0 remains fixture-only with D1/D2/D3 deferred.

P6 S6.1–S6.4 already has an owner: [W11-W10 with W11-W05](windows11-integration.md)
and the [selected MSIX plan](../docs/plans/windows-msix-store.md). Keep mitigation,
provisioning/recovery, one-product lifecycle and submission/delivered provenance
there. Complete CHECK-20/26, SBOM and non-Cargo payload reconciliation belong to
that exit; MSI is an alternative for a later David decision, not a prerequisite.

[Enhanced source-to-Store proof](../docs/plans/release-provenance.md) is optional
under [OD-05 §7.5](../docs/nevrotick-owner-decisions.md#75-optional-enhanced-build-provenance--4-november-2026).
Attestations, independent rebuilds and a public proof bundle add no P6/P7
activation or release-blocking requirement. Required custody, SBOM/licence,
package integrity and installed-product checks above keep their existing scope.

## Post-V1 formal-proof backlog — 6 November 2026

[OD-10 §14.1](../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026)
defers **all formal-proof work until after the first released software version
(V1)**. No missing theorem, verifier/tool extension, formal source preparation,
metadata-consumer repair or formal campaign is a first-version implementation,
phase or release blocker. Existing proof inventories remain in the domain,
exchange, Core and leap task records, with successful subscopes and all failed or
incomplete attempts preserved. Older next-proof directions below are historical;
resume through normal bounded planning after V1. The owner's report of more than
a day without a newly identified product mathematical defect explains the
priority change; it supplies no general correctness claim.

**Ready product continuation:** freeze the existing
[S2.1 observation-to-Core admission contract](engine-transport-pilot.md#next-source-checkpoint-observation-to-core-admission),
following the [completed pure-leap checkpoint](s1.4-leap-policy.md#pure-leap-final-qualification--6-november-2026)
and [three-owner measurement qualification](engine-transport-pilot.md#three-owner-measurement-qualification--6-november-2026).
That recorded three-owner candidate passed 47 ordinary tests, 12 doctests and
complete per-file coverage/mutation gates, with two exactly reviewed equivalent
survivors and no kill credit for them. Those receipts do not qualify later source
or dependency graphs. The [refreshed graph's numerical critical-owner gates](s0.1-source-readiness.md#refreshed-graph-and-current-evidence)
remain pending; reconcile them before the reviewed reference/anchoring bridge,
in the order owned by [development §7](../docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent).
Freeze reference time, genuine date/scale provenance,
uncertainty/divisor and actual receiver semantics before production history
insertion. Authentication and eligibility remain absent; estimator admission,
production scheduling and native Windows retain their separate contracts and
holds. Runtime validation,
checked arithmetic, independent tests/property/boundary cases, the existing
90%/95% thresholds, security/native/package/release gates and all execution holds
remain required. No general parent below is activated by this note.

### Product guarantees still open

**Owner-requested debt, 6 November 2026; review baseline:**
`882f99f227daa00bde7d9d00a1659ca0b21fa55b`.
[OD-10 §14.2](../docs/nevrotick-owner-decisions.md#142-proof-purpose-and-completion--6-november-2026)
requires useful product-operation guarantees and honest completion accounting.
All rows below are **OPEN / DEFERRED until after V1**. `FP-D01`–`FP-D10` are local
backlog identifiers, not new architecture CHECKs or first-release gates. Existing
tasks own the detailed contracts, exact candidates and historical receipts; this
section consolidates their missing composition without opening a second plan.

The review found substantive scoped proofs, not a demonstrated vacuous proof
system. Historical successful runs below are recorded evidence, not fresh solver
reproduction. Ordinary qualification remains valuable and separately scoped.
The proposed failure examples are discriminators for future proofs, **not claims
of current product bugs or executed mutants**. Several already have ordinary
regression tests; reuse those expectations instead of adding duplicate tests.
A changed-source hash rejection detects custody drift, not the semantic defect.

#### FP-D01 — Raw timestamp admission through numerical measurement

**Owner:** domain/exchange; [S1.1 domain](s1.1-time-domain.md) and
[RequestTiming evidence](s1.1-exchange-association.md#request-timing-terminal-qualification--6-november-2026),
CHECK-6/7/25. Sources: `time-domain/src/{coordinate,era,exchange,rounding,clock}.rs`
under `crates/`.

Retain the scoped coordinate, clock, relative-range, exchange/scaling, rounding,
valid-window era and RequestTiming proofs. Complete the real raw-input → public
constructor/Garde → consumer chain, including `EraWindow::new`, exact retained
fields, caller admission and all required success/rejection classes. Closure
requires the actual admitted path to produce the specified offset/RTT and exact
rounding/error result over its full domain. Controls should distinguish a
reject-all constructor, changed retained endpoint, negative era boundary error
and swapped timestamp roles. Physical capture/date/authentication provenance
remains with acquisition/native owners; this numerical guarantee cannot supply it.

**Concurrent integration:** commit `a43916a5190b6f559174a6801eaf8d5295179d83`
adds the [S2.1 experimental measurement consumer](engine-transport-pilot.md#first-controlled-measurement-source-checkpoint--6-november-2026)
in `crates/nevrotick-engine/src/measurement.rs`. Its declared clock/date model,
capture/conversion residuals and actual domain-to-result composition are newly
affected post-V1 obligations under that engine owner. The original review baseline
predates this addition; existing domain proofs do not qualify the new consumer.

The [exact nominal representation child](engine-transport-pilot.md#exact-nominal-representation-activation)
adds source-bound decimal-remainder composition to this same guarantee. Retain
the exact corrected phase/raw-RTT scales, original capture association and actual
producer/consumer path as post-V1 proof obligations. Ordinary implementation and
nonformal checks do not extend the historical domain proofs.

The [exact rational seconds child](s1.2-regression-readiness.md#exact-rational-seconds-normalization--6-november-2026)
adds signed-magnitude/nonzero-denominator conversion, once-rounded binary64
seconds and a separately bounded arithmetic representation error. Prove its
full-domain source correspondence, tie/carry/sign/zero handling, error-bound
association and actual engine seconds-unit adapters after V1. Later exact-anchor
and half-nanosecond-reference composition remains part of the production seam;
ordinary finite examples and conversion bounds supply no physical accuracy.

#### FP-D02 — Regression and diagnostics from the same admitted population

**Owner:** numerical Core; [S1.2 fit contract](s1.2-regression-readiness.md#frozen-proposal-for-synthetic-numerical-behavior)
and [same-fit standard errors](s1.2-regression-readiness.md#s151-same-fit-standard-errors--frozen-authoring-contract),
CHECK-7/25. Sources: `time-core/src/regression.rs`,
`regression/{normalization,residuals}.rs` under `crates/`.

Retain independent rational facts and ordinary S1.2/S151 qualification. Prove
the actual coordinate normalization, geometry admission, inverse-divisor
weighted fit, same-fit residual SSE, `n - 2` scale and standard errors together.
Specify ordered binary64 operations, constant/tiny-series behavior, supported
cancellation, explicit unsupported underflow and justified numerical bounds.
Closure requires the prescribed results/errors for every admitted population,
without unrelated fit/geometry or partial diagnostics. Controls include reversed
weight semantics, `n - 1` scaling, wrong rate sign and statistics from a different
fit. Fixture tolerances alone do not establish a universal error bound.

The selected [source-quality window divisor child](s1.2-regression-readiness.md#source-quality-window-divisors--6-november-2026)
adds the actual per-window derivation, original-row correspondence, positive-finite
input domain including subnormals, and checked source-order overflow outcomes to
this same post-V1 owner. Its computational divisors establish no physical
calibration or eligibility. This debt entry schedules no proof preparation or job.

#### FP-D03 — History updates and one coherent terminal fit

**Owner:** numerical Core/history; [S1.2 adaptive-history contract](s1.2-regression-readiness.md#s147-staged-adaptive-history-caller-contract--2-november-2026),
CHECK-7/14/25. Sources: `crates/time-core/src/regression/{history,adaptive_history,runs,normalization,residuals}.rs`.

Retain fixed-history, runs, retained-prefix and multi-round ordinary evidence.
Complete context/time admission, bounded eviction, auxiliary versus fitted
populations, runs selection, pruning indices and terminal statistics as one
update. Closure requires correct retained evidence and associated report on
success; every rejection preserves prior rows/counts/report/reference. Prove
the declared capacity and fit-attempt bounds. Controls include dropping
decision-only prefix rows, admitting auxiliary rows to fitted geometry,
duplicate-time admission and partial mutation before a numerical failure.

#### FP-D04 — Projection through conservative outward enclosure

**Owner:** numerical Core; [S1.2 anchor bridge](s1.2-regression-readiness.md#s150-fitted-anchor-bridge-contract-and-behavioral-red),
[chart view](s1.2-regression-readiness.md#s154a-frozen-mathematical-view-contract)
and [S1.3 enclosure inventory](s1.3-source-selection-readiness.md#remaining-enclosure-proof-inventory),
CHECK-7/8/10/25. Sources: `crates/time-core/src/regression/{projection,chart,chart/roundoff}.rs`,
`crates/time-core/src/{projection_enclosure,chart_enclosure}.rs`.

Retain ordinary projection/chart qualification and exact fraction constructor,
comparison/complement, split, representation and endpoint-finish leaves. Complete
full-domain fraction addition, actual operand/Garde admission, `EndpointParts::new`
and `enclose_scalars`, then connect them to real projections and inherited chart
roundoff. Closure requires exact time/source/chart association and endpoints
`floor((c - r) * 2^33)` / `ceil((c + r) * 2^33)` for the exact values represented
by the admitted center/radius bits, with exact representability/error behavior.
Require the corresponding rounding classifications; mere containment by an
arbitrarily wide interval is insufficient. Separately preserve justified inherited
arithmetic bounds and conservative chart-view containment; statistical/physical uncertainty remains
distinct from arithmetic roundoff.

The completed addition partition prefix does not close the carry branch:
`1/2 + 1/2` must produce carry without a residual. Other controls should
distinguish wrong negative residual signs, inward endpoint rounding, using a
fitted intercept as the raw anchor and discarding inherited error. Completed
callees or unchanged endpoint finishing cannot close a defective producer.

#### FP-D05 — Selection, supporters and principal retention

**Owner:** selection/Core; [S1.3 expected facts](s1.3-source-selection-readiness.md#independent-expected-facts)
and [S157 checkpoint](s1.3-source-selection-readiness.md#s157-selected-three-owner-linux-fixture-checkpoint--complete),
CHECK-8/14/25. Sources: `crates/time-core/src/{selection,projected_batch,chart_batch,principal,principal/chart}.rs`.

Retain ordinary batch/selection/principal qualification and the exact
`append_depth` lemma. The supporter-label attempt is incomplete. Prove the actual
interval-to-cell depths, maximum components, strict majority, minimum agreement,
witness intersection and complete supporter identity/order, then principal and
hysteresis transitions on that evidence. Closure requires the independently
specified result for the whole admitted batch and preserved state on rejection.
Controls include lost singleton endpoints, merged separated components,
foreign same-label entries and principal retention after invalidating evidence.
With minimum agreement set to two, a four-interval population whose maximum
depth is two must remain `NoMajority`; correct slot append arithmetic alone says
nothing about that decision.

#### FP-D06 — Combination from the exact admitted contributors

**Owner:** numerical Core/combination; [S1.3 kernel contract](s1.3-source-selection-readiness.md#kernel-contract-for-translation),
[S157 arithmetic](s1.3-source-selection-readiness.md#s157-combination-arithmetic--reviewed-next-child-contract)
and [S158 checkpoint](s1.3-source-selection-readiness.md#s158-final-qualification--nine-selected-owners),
CHECK-7/8/14/25. Sources: `crates/time-core/src/combination.rs`,
`combination/{admission,chart}.rs` and `combination/chart/{reduction,roundoff,receiving}.rs`.

Retain independent kernel facts and ordinary S158 qualification. Complete witness
admission, original-entry identity, deterministic reduction, singleton behavior,
weighted phase/rate/dispersion and companion bounds. Closure requires the
prescribed numerical operations and a proved bound on the difference between
each rounded result and its independent exact rational reference from the actual
stored input bits; a rounded mean need not equal that rational exactly.
Rejected denominators/intermediates preserve the
transaction. Controls include altered `1/s_i^2` weights, replacing original
principal phase with the combined mean in dispersion, filtering before metadata
validation, treating a filtered larger witness as a true singleton and clamping
a denominator lower bound upward.

#### FP-D07 — Transactional lifecycle and evidence credit

**Owner:** Core lifecycle; [S1.3 lifecycle implementation](s1.3-source-selection-readiness.md#full-fixture-combination-lifecycle-implementation-31-october-2026)
and [V5 checkpoint](s1.3-source-selection-readiness.md#v5-lifecycle-checkpoint--1-november-2026),
CHECK-8/14/25. Sources: `crates/time-core/src/combination/lifecycle.rs`,
`combination/lifecycle/chart.rs` and `principal.rs`.

Retain ordinary lifecycle/receiving evidence and the exact checked-successor and
source-counter-predicate proofs. The prepared full source-state constructor has
no completed constructor proof. Complete actual construction, event admission,
maturity/penalty updates, contributor preparation, round emission and state
commit. Closure requires correct evidence association, no replay-derived credit
or wrapping identifiers, correct reset/currentness behavior and complete rollback
on rejection. Controls include duplicate ordinals, credit for projection refresh,
penalty mutation before failed combination, premature publication and omission
of the emitted-round state update. Proving `checked_next` alone cannot establish
uniqueness of successive results from its mutable caller.

#### FP-D08 — Reference invalidation and settled active-time feedback

**Owner:** Core reference/chart; [S1.4a readiness](s1.3-source-selection-readiness.md#s14a-reference-input-readiness--admitted-implementation-contract),
[S1.4b composition](s1.3-source-selection-readiness.md#s14b-same-reference-chart-composition--authoring-contract)
and [settled-rate checkpoint](s1.3-source-selection-readiness.md#settled-rate-final-qualification--four-selected-owners),
CHECK-7/8/10/14/25. Sources: `crates/time-core/src/reference_input{,/rate_delivery}.rs`,
`regression/chart.rs` and `regression/chart/roundoff.rs`.

Retain ordinary four-owner settled-rate/reference/chart qualification and clock
arithmetic leaves. Complete exact-instance reference admission, monotone
readiness, chart composition with inherited bounds, supplied active-duration
contributions, cursor overlap/ordinal semantics and failure handling. Closure
requires correct quantitative contributions, no double credit and no restoration
of Ready by consumed/unknown evidence. Rejected admission preserves its owners;
post-admission failures must follow their specified invalidation contract.
Controls include integrating sleep time, replaying a settled interval, dropping
composition error and restoring Ready after arithmetic failure. Future controller
and holdover equations, physical delivery and broker/durable/native settlement
retain their separate P1/P4 owners and require new obligations when implemented.

#### FP-D09 — Whole-interval leap interpretation and atomic knowledge replacement

**Owner:** domain/leap; [S1.4 pure checkpoint](s1.4-leap-policy.md#pure-leap-final-qualification--6-november-2026),
LP-C01–04 and their existing integration owners, CHECK-7/25 plus their scoped
protocol/native CHECKs. Sources: `crates/time-domain/src/leap.rs` and
`leap/{validation,interpretation,bundled,generated}.rs`.

Retain qualified ordinary pure-leap/generator evidence and scoped Gregorian,
scalar-conversion and coverage-prefix proofs. Complete real ledger/scope/metadata
construction, generated-data correspondence, calendar factoring correspondence,
interval completeness, event traversal and final offset/event classification.
Closure requires correct known/unknown/varying outcomes for every admitted
interval, exact boundary semantics, bounded traversal and atomic invalid
replacement. Controls include extending expired knowledge, miscounting boundary
events, claiming a constant offset across an interior transition and partially
publishing a replacement. Silent-source acquisition, guard/reacquisition and
Windows mapping remain separately owned; no pure proof establishes them.

#### FP-D10 — Repeatable proof execution and trustworthy result consumption

**Owner:** proof tooling/QA within the selected domain/Core task; this is an
enabling debt, **not an additional product theorem**. Follow the
[proof workflow](../.agents/skills/testing/references/formal-verification.md)
and retain the [supporter terminal record](s1.3-source-selection-readiness.md#supporter-terminal-record-and-first-release-deferral--6-november-2026).

After V1, separate fresh, bounded candidate invocations from immutable historical
campaigns. Existing fixed boot/time/output bindings are one-off evidence, not a
current-candidate regression gate. Preserve source/tool/configuration custody
without resetting expired attempts or overwriting receipts. Repair the known
supporter consumer's dependence on `attributes.unwind_value` using actual pinned
exporter output/schema; its synthetic fixture currently supplies a field absent
from that export. Do not weaken required evidence or invent substitute metadata.

Closure requires a genuine emitted fixture accepted by the consumer, relevant
malformed/partial/empty fixtures rejected, and a fresh candidate's healthy →
intended semantic defect → restored healthy run with exact scope accounting.
Count attempted, completed and qualified jobs separately; helper lemmas, controls,
covers and Python tests never become counts of proved product algorithms. Claim
automatic regression protection only when that specific gate is configured and
actually runs. No new generic framework or CI campaign is required merely to
record this debt; a tooling repair must unblock the selected product guarantee.

### Completion and resumption of proof debt

After the first released V1, activate **one bounded next child** in its existing
owner task. The first candidate should address a named missing product relation,
such as full addition through enclosure or selection through its actual outcome,
using unchanged qualified premises where applicable. This is a choice to justify
from then-current source/risk/feasibility, not an automatic restart of old pilots.

Before work, record the independent complete contract, admitted domain, exact
source and dependency chain, meaningful fault controls, remaining trust boundary
and finite total cost/stop criterion. Required success, errors, machine semantics,
state effects and work bounds belong to the guarantee. Constructor/callee
assumptions need evidence for the same bytes and an explicit composition link.
Healthy/mutant/restored results must reach the intended operations; source guard,
compile or setup failures are not semantic kills. If the hard downstream theorem
remains open, retain useful completed premises and leave the parent row open.

Report closure by the exact product guarantee and candidate, with its supporting
lemmas and limitations. Preserve failed/partial attempts and ordinary coverage;
timeouts, infrastructure checks and larger counts cannot discharge a row.
This documentation/instruction update executes no formal jobs and supplies no
new product qualification.

### Policy-change validation — 6 November 2026

This is documentation/skill evidence, not closure of FP-D01–10. The unchanged
package checker passed for all 65 skills; local-link and whitespace checks passed.
A bounded owner-mapping review corrected three draft ambiguities: exact scalar
floor/ceil versus mere containment, the minimum in the half-population example,
and floating-point error bounds versus exact rational equality. That reviewer
also supplied the initial debt inventory, so it is not an independent inventory
audit.

Two fresh-context agents received the changed testing skills and raw hypothetical
post-V1 fixtures, without the coordinator's expected verdict or debt inventory.
The queue fixture contained a successful arithmetic-helper proof, an omitted
caller state update and an exhausted whole-operation pilot; the agent identified
the concrete capacity/count failure, retained the helper's narrow result and
stopped the proposed budget reset/unrelated proof continuation. The neighboring
utility fixture contained one complete public-operation proof, passing consumer
mocks incompatible with a genuine exporter schema and an expired wrapper; the
agent accepted the useful small theorem, rejected the integration/automation and
inflated-count claims, and proposed a bounded schema-grounded correction.
Each exercise had one initial context-allocation setup failure and one completed
retry. Both reviews were textual; no product code or proof job ran. These two
cases support the instruction change in those scenarios, not a measured
old/new improvement or a guarantee of future agent behavior.

## P0 platform and policy

**Owner:** platform/security, with IPC, clock, UI and architecture/policy owners
as below. **Status:** deferred parents; existing S0.1 and W11-W05/W11-W10 S0.6
records remain authoritative. **Exit:** [plan §6/P0](../docs/plans/development-plan.md#p0--admit-sources-and-prove-platform-assumptions)
and [architecture §21](../docs/windows-time-architecture.md#21-delivery-phases-and-readiness-gates).

S0.2/S0.3 consume the [management route policy](../docs/windows-time-architecture.md#421-service-management-admission-and-administrative-recovery)
and W11-W05/W11-W10 source refinement: exact SCM rights, current actor plus
installed-component admission for protected IPC, deliberate elevated recovery
and same-principal/wrong-application controls. Mechanism selection/native qualification remain open dependencies of
existing deferred cards.

| Future card / role owner | Prerequisites and next bounded action | Independent observation required for exit |
| --- | --- | --- |
| S0.2 service tokens / platform/security | Intended account topology, admitted wrapper/graph, disposable Windows fixture and current native gates; prepare one two-service token/lifecycle experiment. Clock-writing subcase also needs the canonical temporary experiment card. | Intended broker operation succeeds within policy while engine/UI clock and broker-injection access fail; real callback/stop ownership remains bounded. CHECK-1/27 subset. |
| S0.3 mutual IPC / IPC/security | Relevant S0.2 identity evidence and §5.4 endpoint/caller-lifetime contract; specify one real status method plus one protected method and their allowed/denied principals. | Correct caller/server pair succeeds; fake server, wrong/revoked role, old session and restart/PID-reuse cases cannot gain effects. CHECK-2 subset; retain the canonical local-RPC alternative if pipe proof fails. |
| S0.4 backend semantics / clock/platform | Owned guest, controlled reference, current owner, independent recovery and predeclared operation/phase/rate/duration/attempt bounds; apply W11-W02/leap child and begin read-only/denied calls. | Measured sign/units/quantization and delivered/cancelled effects match independent time; kill/restart/power and ambiguous outcomes do not imply a finite-expiry guarantee. CHECK-10/11/12. Unset write bounds mean no write. |
| S0.5 Slint replacement and full UI feasibility / UI/accessibility + dependency/licence | Accepted [Slint default, OD-13](../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit) and the [current GUI handoff](slint-gui-handoff.md). Consume the implemented v0.4 fixture GUI, compatible original GUI GPL3 scope and exact branch evidence; reconcile the merged candidate and remaining backend/native/text/hardening gaps before dependent work. Full window/source/settings/elevation/tray feasibility retains its own scope and D1/D3 triggers. No new toolkit selection or Win32 fallback. | Named keyboard/Narrator/UIA journeys work at required contrast/scales; missing focus/labels or unusable scaling fails feasibility. CHECK-22; historical Win32/private-pilot or portable fixture success alone cannot close it. |
| S0.7 decision freeze / architecture/policy/security | Relevant P0 measurements, accepted owner decisions and remaining OPEN issues; prepare the next dependent decision card rather than guessing production limits. | ASVS/native-profile, step/date/holdover, supported-build, service trust/revocation/egress and native-work limits have explicit decisions or blockers. Unsupported alternatives remain blocked; CHECK-4/9/10/16/27 as applicable. |

**S0.4/S0.7 clock-limit preparation:** apply the [existing W11 matrix](windows11-integration.md#431-clock-correction-limits-preparation--5-november-2026)
to field definitions, bounded evidence and the next dependent OD-04 decision.
Native capability, model assumptions and acceptable policy tolerances remain
separate; source review adopts no production values or execution permission.

**S0.4/S0.7 platform preparation:** [W11 §4.32](windows11-integration.md#432-two-family-windows-qualification-matrix--5-november-2026)
records OD-14's two intended OS families and first engineering fixtures. Platform,
build and W11-W05/10 owners resolve admission/delivery and actual fixture inputs;
P4/P6/P7 consume row-specific timing, protection, lifecycle and resource evidence.
OPEN-2 and all execution holds remain open/intact; §7 supplies current product priority and formal work is post-V1.

### Windows component reuse at existing stage entry

**Source findings integrated 5 November 2026; dependent implementation remains
deferred.** Apply [plan §4.4](../docs/plans/development-plan.md#44-apply-windows-component-reuse)
and [architecture §18.6.2](../docs/windows-time-architecture.md#1862-windows-component-reuse-and-binding-coverage)
inside these existing parents. This is a shared entry checklist, not another
workstream or an instruction to evaluate every Windows crate now.

| Existing parent / owner | Trigger and next action | Required evidence or retained disposition |
| --- | --- | --- |
| S0.2/S0.3, S2.1, S3.1–S3.4, S4.1; platform/network/IPC/clock | Before the next dependent native adapter, map its operation to a selected published binding/wrapper, version/features/SDK symbols and remaining product contract in W11 §4.29. | API coverage and compatible ownership/error/callback behavior; subsequent native controls under existing holds. Retain Mullvad candidate and Microsoft plural service hold. |
| S2.3 and reached protected-storage consumers; ST-W01/ST-W04 | Before VBS-dependent implementation, complete storage §6.7's binding/SDK coverage card and reconcile the missing flags. | Compatible generated/source-pinned API plus separate actual-key/OS evidence; REQUIRE semantics and policy-controlled fallback preserved. Native storage experiments remain paused. |
| Existing settings/state adapter; platform/UI | A concrete registry operation justifies comparing `windows-registry`. | Exact rights/DACL, bounded values and failure recovery preserved; no unconditional dependency or move of authoritative state into the registry. |
| S0.5/S5.1–S5.4; UI | OD-13 selects Slint; the current S0.5/GUI handoff owns the implemented fixture GUI and remaining qualification, with the old M0-SL queue retained historically. A concrete issue preventing selected delivery and the bounded revisit procedure precede any alternative. | Preserve equivalent fixture, resource/lifecycle/error, accessibility, package/runtime and mitigation evidence. Retain the existing Reactor/Slint measurements; no general toolkit reselection or assumption that `windows-window` can own Slint's event loop. |
| S5.3; UI/diagnostics | Export needs a native picker: recheck working published `windows-pickers` or use the admitted OS dialog path. | Cancellation and error remain distinct, output stays bounded/redacted and written by the ordinary UI; no service arbitrary-path write. |

Retain [UI handoff](ui-m0.md#windows-component-reuse-handoff--5-november-2026),
[toolkit comparison trigger](s0.5-ui-comparison.md#conditional-windows-toolkit-candidates--5-november-2026),
[W11 routing](windows11-integration.md#429-windows-component-reuse-routing--5-november-2026)
and [storage coverage](windows-protected-storage.md#67-windows-binding-coverage-and-reuse--5-november-2026)
in their existing task chains. No optional crate becomes a current-Core,
UI-M0 or global release prerequisite; existing CHECK/ST-T evidence still governs.

### Deferred production research contracts

S0.7's architecture/security coordinator owns these research-to-contract handoffs
with the named production consumers. They remain open until their existing owner
decision and candidate-specific checks; they do not delay the current inert Core
checkpoint. [R-02/R-03/R-04](../docs/nevrotick-research-handoff.md) and accepted
OD-02/03 retain policy authority.

| Existing research / consumer | Deliverable before the dependent behavior | Independent exit and retained constraints |
| --- | --- | --- |
| R-02/R-04, OPEN-5/6; Core/acquisition policy before production voting | Freeze routine versus exceptional denominators, operator/dependence groups, timeout versus statistical rejection, observation age and uncertainty growth under architecture §7.2. | Compare four fresh groups, two unavailable groups, duplicate identities, a 2–2 split and one fresh source plus old predictors. Missing peers or predictions cannot manufacture fresh independent votes, lower the configured minimum or select a silent single-peer fallback. CHECK-7/8/14. |
| R-02, OPEN-6; security/persistence/P2 before persisted-floor recovery | Specify independently evidenced, narrowly authorized supersession of an intact but wrong future floor, including refusal and dependent model/association invalidation. | Valid recovery beside forged/stale/unauthorized supersession preserves old evidence/provenance, consumed correction budgets and installer/controller ownership; deleting a file or resetting optional state grants no fresh authority. CHECK-9/15; future TA-C10 does not replace this ordinary P2 gate. |
| R-03, OPEN-5/9; product/security/Core before S4.2 plain-profile correction | Propose plain-NTP versus authenticated recovery envelopes with cumulative absolute movement, count, restart and holdover semantics. | Malicious common-path/repeated-shift cases are discriminated from legitimate offline/RTC drift; report security and availability consequences for the owner decision. Twelve hours, 1,000 seconds and upstream step thresholds are comparison candidates, not adopted constants. CHECK-8/10/12/15; preserve OD-02's separate authenticated-recovery goal. |

## P1 Core completion

**Owner:** numerical Core/domain; tooling/dependency/QA maintains verification.
**Status:** deferred general parent; current S1.1–S1.3 and pure S1.4 consumers
remain in [their existing task chain](s1.3-source-selection-readiness.md).
**Exit:** [plan §6/P1](../docs/plans/development-plan.md#p1--pure-protocols-and-independently-checked-algorithms),
CHECK-6/7/8/19/20 and pure CHECK-25; this proves no Windows effect.

**Retained proof history; all further work post-V1, 6 November 2026:** the
[query-split card remains terminal DEFER/incomplete](s1.3-source-selection-readiness.md#fraction-query-split-terminal-assessment--6-november-2026).
Fourteen of twenty maximum commands ran; jobs 1–13 qualified 1,415 raw checks
(1,315 Success, 69 covers, nine intended failures, 22 Unreachable). Job 14's
universal proof timed out at **60,018 ms** with zero checks. Jobs 15–20 never
ran, leaving direct witnesses and both restorations unqualified. Final source,
permission, receipt and tool custody passed across 49 source inputs, fourteen
input trees and three fresh certificates, without bytecode drift. The 67 focused
code checks passed; the old eighteen-command kit and ordinary defaults remain
unchanged. The earlier prefix is partial evidence, with no full addition or
Core/P1 completion.

The existing task's
[source-only bridge preparation](s1.3-source-selection-readiness.md#fraction-verus-bridge-source-preparation--6-november-2026)
retains the six executable predicates with reversible declaration annotations
and explicit mathematical contracts. Its
[first pure-bridge card](s1.3-source-selection-readiness.md#fraction-verus-bridge-terminal-assessment--6-november-2026)
is terminal DEFER after one healthy dispatch: seven private-`open` annotation
errors, zero SMT/proof results, no controls/restoration. The exact seven-modifier
source-only correction has fourteen passing source checks. The subsequent
[private-spec v2 card](s1.3-source-selection-readiness.md#private-spec-verus-bridge-v2-terminal-assessment--6-november-2026)
reached Z3 but remains terminal DEFER: five equalities failed, zero jobs qualified,
and controls/restoration did not run. Constructor Post's modular success depends
on failed helper equalities. The subsequent
[bit-length dependency campaign](s1.3-source-selection-readiness.md#bit-length-dependency-terminal-assessment--6-november-2026)
completed five jobs: full-raw lemma/two body-to-spec equalities, three intended
contract defects and restored health. Its62 focused Python checks pass, with22
source files and final custody/tool bookends bound. Reuse the unchanged scoped
proofs for remaining full-bridge arithmetic/representation work. The subsequent
[constructor-Post campaign](s1.3-source-selection-readiness.md#constructor-post-terminal-assessment--6-november-2026)
completed five jobs: full-raw Post/lemma/helpers, three intended Post defects and
restored health, with93 focused Python checks and24 bound source files. The later
[complement v2 campaign](s1.3-source-selection-readiness.md#complement-v2-terminal-qualification--6-november-2026)
has now passed seven jobs: four full-raw predicate equalities/three arithmetic
lemmas, five intended failures and restoration. All143 selected checks pass;
30 source files/11 executed Python sources and final custody/tool guards bind
the candidate. V1's preprocessing failure remains closed and historical.
The later [rational-order qualification](s1.3-source-selection-readiness.md#rational-order-terminal-qualification--6-november-2026)
passed nineteen jobs: five full-raw predicate equalities/seven total lemmas,
seventeen intended failures and restoration. All161 selected checks pass;
33 sources/11 executed-Python records and final guards bind the candidate.
The source gate confirms missing pinned checked-shift support; native Ordering
equality/comparison Post and caller import stay held. Independent
[RequestTiming qualification](s1.1-exchange-association.md#request-timing-terminal-qualification--6-november-2026)
passed five full raw-pair/three-defect/restored jobs,158 Python checks and final
34-input/nine-executed-Python guards. The later [actual append-depth proof](s1.3-source-selection-readiness.md#selection-append-depth-terminal-qualification--6-november-2026)
passed six full raw-state/four-defect/restored jobs,671 checks and208 Python
checks;34 inputs/nine executed-Python records and final guards bind it.
The subsequent [actual lifecycle ordinal proof](s1.3-source-selection-readiness.md#lifecycle-ordinal-terminal-qualification--6-november-2026)
passed five full raw-u64/three-defect/restored jobs,263 checks and248 Python
checks;38 inputs/nine executed-Python records and complete23-enum closure bind it.
The subsequent [source-counter invariant proof](s1.3-source-selection-readiness.md#lifecycle-source-counter-terminal-qualification--6-november-2026)
passed five full raw2^80/three-defect/restored jobs,145 properties and291 Python
checks;32 inputs/nine executed-Python records bind it. The [complete constructor
source checkpoint](s1.3-source-selection-readiness.md#source-state-constructor-source-checkpoint--6-november-2026)
now prepares genuine Garde/derive/Report/constructor source and six Cargo cases,
authenticates14 selected vendor packages and passes51 source checks plus independent
review. No compiler/proof ran; source-specific admission/target/allocator/drop/
unwinding and full constructor proof remain open, without a retry of held or
exhausted compiler campaigns. Transactional lifecycle, prefix/depth/topology and
whole selection remain open. Continue independent [supporter-label source preparation](s1.3-source-selection-readiness.md#next-independent-core-source-assignment--supporter-labels)
over its complete raw array/count and actual sort-callee closure; loop/oracle/
unwinding/card review precedes execution. The [constructor return trigger](s1.3-source-selection-readiness.md#constructor-execution-gap-and-return-trigger)
retains the known model/admission holds. No unsupported comparison retry follows. The complete six-equality bridge and its
remaining controls stay open before relating the actual Verus caller to separately
Kani-proved callees. The existing Verus `--no-cheating` gate rejects
`external_body`/`assume_specification`; no flag change, adoption or new run is
authorized. Smaller encoding and SAT timings identify no completed property or
counterexample. All eleven mathematical assertions, all23 concrete witnesses,
both complete restorations and existing owner-decision/tool-adoption rules
remain required. No automatic retry or budget extension follows; all execution
holds and the wider P0–P7 objective remain in force.

**Earlier manual composition checkpoint:** the
[manual composition card remains terminal DEFER](s1.3-source-selection-readiness.md#manual-composition-terminal-assessment--6-november-2026)
after fourteen of eighteen maximum commands. Jobs 1–13 completed 1,415 raw
checks, including three actual-callee Post/actual-return seed proofs and nine
compiled controls. Job 14's full-domain addition proof timed out at **60,016 ms**
with zero checks; jobs 15–18 did not run. An external audit import separately
added four `.pyc` files and `__pycache__` to the frozen source island. All 45
bound inputs in the checkout and frozen copy, fourteen job input trees and
three certificate receipt seals remained unchanged, but the global custody
failure is preserved without cleanup or retry. The earlier narrow premises are
partial evidence, with no full adoption, automatic certificate reuse or Core/P1
completion.

The earlier
[full-addition compute checkpoint ended DEFER](s1.3-source-selection-readiness.md#full-addition-compute-budget-terminal-assessment--6-november-2026)
in the existing Core task under OD-10 and CHECK-7/8/25. The first healthy
full-domain Z3 proof timed out at **300,011 ms**, exporting no checks; no mutant
or restored job ran. The exact original model, source and custody stayed intact.
Addition and dependent enclosure/Core acceptance remain open.

The [ordinary partition pilot](s1.3-source-selection-readiness.md#ordinary-addition-partition-terminal-assessment--6-november-2026)
retains four bounded healthy results—the union and cases 0–2—with **631 accounted
checks and 22 covers**. Healthy case 3 timed out at **60,018 ms** with an empty
checks list; later leaves, all three semantic defects and all restored proofs
did not run. Its aggregate remains partial DEFER. The separate
[ordinary Z3 comparison](s1.3-source-selection-readiness.md#existing-z3-comparison-terminal-assessment--6-november-2026)
also remains terminal after its 60,012 ms first-healthy timeout with no checks.

The [contract-mode pilot](s1.3-source-selection-readiness.md#addition-contract-mode-terminal-assessment--6-november-2026)
also remains terminal DEFER. Its first primitive checker exited Kani 1 / CBMC 134 on an internal
`symbol_expr_type_matches_symbol_table` invariant failure in `apply_closure`,
returning an empty checks list. One command ran; zero callee certificates and
zero product proof jobs followed. No retry, source/tool change or mode adoption
follows that failed card.

Retain the earlier **60,025 ms monolithic timeout with zero checks** and the
[Verus addition pilot](s1.3-source-selection-readiness.md#verus-addition-pilot--5-november-2026).
That trial's three-command readiness budget is exhausted: the final command
rejected required `u64::checked_shl` during VIR translation before SMT execution.
It remains **DEFER**, with zero product-proof commands. Official support for the
unchanged API and a new reviewed finite checkpoint are its return trigger.
Held patches and all execution holds remain in force.

| Card / role owner | Prerequisites and next bounded action | Independent observation required for exit |
| --- | --- | --- |
| S1.3 Verus addition pilot / numerical Core; tooling/QA | **Post-V1; readiness attempt terminal DEFER.** Required `u64::checked_shl` is unsupported in the pinned candidate. Official support for the unchanged API and a newly reviewed finite checkpoint are the return trigger; no retry or local assumed specification follows. | Verify actual `Fraction::new`, `compare`, `complement` and `add_fractions` bodies with exact executable correspondence and source/tool custody. Require three compiled semantic defects to fail their intended obligations and a restored healthy pass. No product `external_body`, assumed central theorem, alternate executable behavior or model-only proof closes this exit. |
| S1.3 independent split proof / numerical Core; tooling/QA | **Qualified raw leaf only.** The [split terminal checkpoint](s1.3-source-selection-readiness.md#split-terminal-qualification--5-november-2026) completed all five expected job outcomes with the unchanged admitted Kani route. | Full raw significand/scale admission, exact decomposition, fraction canonicalization/invariants and error classification are proved, including three compiled semantic defects and restored healthy proof. This does not qualify the complete producer, addition or Core. |
| S1.3 raw scaled-value representation proofs / numerical Core; tooling/QA | **Qualified exact raw leaves only.** The [representation terminal checkpoint](s1.3-source-selection-readiness.md#representation-terminal-qualification--6-november-2026) completed all ten expected outcomes: `signed_whole` covers every bool sign, u64 whole and arbitrary u64/u32 fraction field pair; `from_bits` covers every raw u64 bit pattern. | Exact signed magnitude/fraction independence, raw bit admission, sign/Q33 representation, invariant/error guarantees, each leaf's three semantic defects and restored health are qualified. This closes no Garde/caller admission, addition, `EndpointParts` construction, complete enclosure or Core/P1 acceptance. |
| S1.3 full-domain fraction addition / numerical Core; tooling/QA | **Post-V1; query-split card terminal DEFER.** The [terminal assessment](s1.3-source-selection-readiness.md#fraction-query-split-terminal-assessment--6-november-2026) records the qualified thirteen-job prefix, job 14's zero-check universal timeout, unrun jobs 15–20 and passing final custody. Direct witnesses and restorations remain unqualified. After V1 only: source-only encoding/property diagnosis and bounded Verus/Kani bridge scout; `--no-cheating` rejects `external_body`/`assume_specification`. An exact bridge is required; no flag change, adoption, new run, automatic retry or budget extension follows. Prior trials and all holds retain their status. | Retain the existing full checked-pair carry/residual and required-success contract, unchanged actual-source full domain, all eleven mathematical assertions, all 23 witnesses, three semantic defects and both complete restored lanes. Earlier bounded results, partial regions, narrowed domains and assumed product theorems do not close addition or Core. |
| S1.4 discipline/holdover / Core/domain | Applicable S1.3 selection/combination and current reference, achieved-effect and active-time delivered-rate contracts; finish the current child first, then freeze one abstract controller/holdover transition. | Independent trajectories give bounded step/slew/holdover proposals; stale generations, duplicate/unknown effects and unacknowledged requests cannot create delivered correction or fresh observations. CHECK-7/8/10/11/25, AC-19. Current S1.4a–c and rate-delivery receipts stay in S1.3. |
| S1.4 leap child / domain/Core/protocol | **Selected pure checkpoint complete:** [current qualification](s1.4-leap-policy.md#pure-leap-final-qualification--6-november-2026) reconciles four new mutation cohorts, qualified portable/contracts/coverage reuse, its admission-code delta and current offline generator checks. Full acquisition, guard/reacquisition, integration/native scopes and post-V1 proofs remain open. | Pure history/coverage/rule cases and atomic replacement pass within the selected scope. Full silent-source acquisition and LP-C04 stay in protocol integration; native mapping stays S0.4/P4. Existing LP-C cases and CHECK-6/7/8/11/14/25/26 retain their owning slices. |
| S1.5 verification closure / tooling/dependency/QA | Existing configured application gates and selected critical owners; maintain required checks now, close remaining phase scope at its actual checkpoint. Deferred QF-02 recipe maintenance stays acknowledged in the active Core task. | Healthy code/inputs pass and intended lint/SAST/discovery/admission faults fail actual gates; retained provenance, applicable per-file coverage/mutation and exact receipt scope support closure. CHECK-19/20/25/26. No new implementation prerequisite or duplicate campaign. |

Reuse unchanged Kani proofs within their recorded scope; the held Kani extensions
remain held. Raw split, `from_bits` and `signed_whole` are qualified within their
recorded domains. Retain the existing remaining proof inventory: addition,
Garde/caller admission, endpoint/enclosure construction and complete producer/Core
acceptance remain open. The pilot adds
no global CI gate or phase blocker and lifts no execution hold; missing required
mathematical evidence remains post-V1 debt under OD-10 §14.1, without blocking
V1 implementation, phase acceptance or release.

## P2 acquisition

**Owner:** engine/protocol/security with platform/dependency reviewers.
**Status:** deferred production parent. Reuse [engine](engine-transport-pilot.md),
[protocol](s0.1-protocol-comparison.md) and [crypto](s0.1-crypto-feasibility.md)
work at their qualified seams; fixtures, isolated kernels and host receipts do
not inherit production admission. **Exit:** [plan §6/P2](../docs/plans/development-plan.md#p2--secure-bounded-ntpnts-communication).

Under [OD-11](../docs/nevrotick-owner-decisions.md#15-od-11--socks5-scope-and-deferral),
P2/P5 retain Direct interface/family policy, upstream-failure recovery and path
reuse. Proxy support and its specific checks/UI are deferred to SOCKS-0 below;
they are not a prerequisite to first Windows delivery. Unsupported proxy state
must still fail explicitly instead of becoming Direct.

| Future card / role owner | Prerequisites and next bounded action | Independent observation required for exit |
| --- | --- | --- |
| S2.1 UDP/DNS observation / engine/platform | [Three-owner measurement qualification](engine-transport-pilot.md#three-owner-measurement-qualification--6-november-2026) passes on its recorded host candidate. Next freeze the existing observation-to-Core admission contract. W11-W01/native, production scheduling/KoD, leap/scale and statistical admission retain separate contracts. | Actual controlled IPv4/IPv6 exchanges yield independently checked, explicitly ineligible nominal measurements or diagnosed refusal. Production wrong-endpoint/correlation/generation and forged/stale KoD controls remain required at their reached owners; no threshold or retry bypass. CHECK-6/8/21/25/27. |
| [S2.2 ordinary composition](engine-transport-pilot.md#ordinary-full-nts-composition--s22-preparation-5-november-2026) / engine/protocol/security | Source card prepared; implementation deferred. Exact TLS/provider/verifier/AEAD graph, ordinary Windows profile, finite runtime/lifetime contracts and local zero-cookie disposition precede dependent integration. | Fresh KE publishes one complete association; protected UDP completes once and eligible evidence reaches the actual Core consumer beside authentic numerical refusal. Paired path/packet/resource/trust controls retain independent vectors/interoperability. CHECK-6/7/8/9/17/20/21/25/26/27, AC-04. |
| S2.3 reuse/recovery / engine/security/storage | Qualified ordinary NTS first; OPEN-8/relevant recovery decisions and any optional-state construction/provider/pause gates; freeze one compatible-restart or provisional-recovery child. | Compatible reuse succeeds beside unavailable/stale/substituted state; strict authentication remains, no restored pending request, plain hint vote or global clock effect. Two-helper failure creates no silent one-helper path. CHECK-8/9/15/25/26, AC-02/09/10; applicable ST-T cases. |
| S2.4 adversarial closure / protocol/security/QA | Reached S2.1–S2.3 entrypoints and frozen retry/endpoint/native-job contracts; choose the remaining discriminating failure case. | Replay/endpoint changes, storms, stuck native work and cancellation preserve bounded real-job custody, atomic state and redacted output while valid neighboring acquisition works. CHECK-6/8/9/17/26/27; complete the linked checklist. |

Before cached/resumed association support, apply the existing
[R-05 authentication-lifetime card](s0.1-crypto-feasibility.md#deferred-nts-authentication-lifetime--r-05).
Before persisted-floor recovery or production voting, close the corresponding
S0.7 research contract above. These are dependent-behavior gates, not a new wait
for unrelated research before ordinary full NTS acquisition.

## P3 service boundary

**Owner:** platform/IPC/security, with Core and diagnostics owners.
**Status:** deferred full-services parent; apply W11-W04/05/08 and relevant
ST-W03–ST-W07 before dependent adapters. Optional-cache pause does not pause the
separate mandatory policy/authority/floor state owner. **Exit:** [plan §6/P3](../docs/plans/development-plan.md#p3--windows-services-secure-ipc-and-protected-state).

S3.1/S3.2 inherit the [management admission/recovery contract](../docs/windows-time-architecture.md#421-service-management-admission-and-administrative-recovery)
and hardening §9.9/§9.11 controls from S0.2/S0.3. Service DACL/package signature alone cannot close application admission; normal-client denial and
deliberately elevated recovery must both work. Full services remain deferred.

| Future card / role owner | Prerequisites and next bounded action | Independent observation required for exit |
| --- | --- | --- |
| S3.1 final services / platform/security | Relevant S0.2/S0.7 evidence, admitted narrow SCM adapter and native fixture; freeze one service's startup/status/stop child. | Intended service becomes actually ready; denied token actions and failed/delayed callbacks cannot report false readiness or escape shutdown ownership. CHECK-1/27, AC-01/13. |
| S3.2 authenticated handlers / IPC/security | S0.3 proof, final endpoints/tokens and strict framing/domain/current-authorization contracts; expand one permitted/denied method pair. | Valid current caller succeeds; malformed/oversized/stale/unauthorized/fake-endpoint/partial-I/O cases cause no protected effect; native buffers/permits survive actual completion. CHECK-2/3/25/27. |
| S3.3 mandatory policy/state / platform/security | Frozen revisions/migration/durable-outcome and directory-authority contracts; select one atomic mandatory-state transition and recovery case. Adopted optional storage remains a separately gated child under its existing task. | Valid current policy commits; corrupt/old/unauthorized records, disk-full/interruption and path substitution preserve safe state and charged authority. Optional reset cannot erase mandatory state. CHECK-15/25/27, AC-07; relevant W11/ST-W refinements. |
| S3.4 simulated integration/status / Core/platform/diagnostics | Actual secured services/handlers plus qualified pure proposals; integrate one journey against a simulated clock backend. | Valid proposal/status is coherent; stale generations, unavailable dependencies and malformed snapshots cannot claim Tracking, applied effects or success. Required redacted diagnostics remain bounded. CHECK-14/17/25/27, AC-05/16; actual clock effects stay P4. |

### Shared adapters, errors and diagnostics

**Owner:** platform/IPC/diagnostics with protocol/security review; executable
compositions retain their own sinks. **Status:** deferred interface handoff,
applied before the first dependent production adapter, handler or event emitter.
P0 prepares finite limits/sink policy; P2 uses it at reached protocol boundaries;
S3.1–S3.4 integrate it and S5.3 consumes redacted output. Current Core stays separate.

Use [architecture §3.4](../docs/windows-time-architecture.md#34-shared-windows-adapters-and-diagnostics),
[§6.5](../docs/windows-time-architecture.md#65-errors-diagnostics-and-proof-obligations),
[§12.3](../docs/windows-time-architecture.md#123-logging-and-support) and
[hardening §9.8](../docs/windows-time-development-hardening.md#98-diagnostics-and-shared-adapter-verification).
Freeze narrow role-specific `windows-platform`, portable `ipc-contract` and
platform-neutral diagnostics interfaces; no generic privileged Windows manager
or separate logging service is selected. Concentrate shared product-error mapping
and sanitization behind one facade in one or two shared modules; domain-specific
errors remain with their enforcing owners and retain useful internal causes.
Do not scaffold crates solely to satisfy this task.

Before exposing errors, unify bounded parser/Serde/Garde failures into stable
product codes and allowlisted paths. Unknown field names stay generic; freeze
finite count/byte limits and bound work before constructing a report. A valid
useful error is the neighbor for malformed input, secret sentinels, control
characters and raw-library-error leakage at actual IPC/UI/events/exports.

One decision owner emits one useful event. Required events reach their required
sink despite optional WARN/ERROR/OFF filters. Per-process authorized DEBUG
sessions expire locally; helper/UI exit, restart, lost acknowledgement and a
full/unavailable sink preserve bounded resources and authority. Test the actual
feature/sink closure and privacy, not just a formatting helper. Diagnostics
neither publish authoritative UI state nor acknowledge clock effects. Existing
CHECK-1/2/3/14/15/17/21/25/26/27 apply at their reached boundaries.

## P4 clock integration

**Owner:** clock/platform/security with independent reference/QA owners.
**Status:** deferred; relevant S0.2–S0.4/S0.7 and P3 evidence, native admission,
owned disposable fixture/reference/recovery and explicit finite experiment card
precede real effects. Apply W11-W02, leap child and relevant state/adapter
refinements. **Exit:** [plan §6/P4](../docs/plans/development-plan.md#p4--windows-clock-integration-ownership-and-recovery).

Use [W11 §4.31](windows11-integration.md#431-clock-correction-limits-preparation--5-november-2026)
for the ordered producer/owner/consumer handoff and paired controls. Account actual
combined movement and late durable outcomes even when Core rejects publication;
qualify holdover assumptions and conditional recovery exposure. OPEN-5/7/13 and
all current holds remain; no parallel clock task or numerical envelope is added.

| Future card / role owner | Prerequisites and next bounded action | Independent observation required for exit |
| --- | --- | --- |
| S4.1 backend/accounting / clock/Core | Measured sign/units/quantization and actual-outcome contracts; freeze one acknowledged operation and lost-acknowledgement case. | Independent time matches delivered effect; unknown/duplicate outcomes cannot trigger a blind repeat or mischarge requested effects as achieved. CHECK-10/15/25/27, AC-19. |
| S4.2 limits/budgets / broker/security | Explicit ordinary/initial/exceptional/date/era limits and durable boot-budget decisions; select one within-limit proposal and forbidden neighbor. | Current approved bounded effect succeeds; oversize/stale/replayed approval and reconnect/sign-change/crash cannot renew authority or budgets. CHECK-3/10/12/15, AC-08/11. |
| S4.3 power/generations / platform/clock | Qualified active-awake/elapsed clock mapping, cancellation and leap guard contracts; choose one suspend/resume or mapping-change journey. | Fresh post-transition quorum restores useful tracking; late/crossing work is rejected, sleep is not delivered slew and no synthetic leap correction occurs. CHECK-10/11/27, AC-12. |
| S4.4 ownership/recovery / platform/security/lifecycle | Final W32Time/controller ownership, takeover/restore and outcome-journal contracts; select one current-owner recovery plus successor/ambiguous-owner control. | Owned cleanup/restoration succeeds; GPO/competing controller/unknown successor blocks blind resets, crash loops retain consumed budgets. CHECK-5/10/12/13/15, AC-08/13/14. |
| S4.5 reference soak/faults / QA/performance/platform | Integrated bounded controller, declared convergence/recovery envelope and supported native fixture; run the next prescribed reference/soak child once admitted. | Independent trajectories and cleanup/resource observations meet the declared profile; outages/crashes cannot preserve false freshness or hide surviving native work. CHECK-10/11/12/21/27, AC-19/20. |

## P5 production UI

**Owner:** UI/accessibility with service/security/lifecycle owners.
**Status:** live P5 remains deferred. [OD-13](../docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit)
selects Slint; the [S0.5 active goal](s0.5-ui-comparison.md#active-goal--finish-the-complete-slint-gui)
and [GUI handoff](slint-gui-handoff.md) own the delivered v0.4 fixture implementation,
original GUI GPL-3.0-only grant and exact branch evidence. The earlier UI-M0
M0-SL-01–05 queue is historical planning input, superseded where source stages
have been completed. Merged-candidate reconciliation, actual backend/native
qualification, independent critical-text approval and the planned P3/P5/P6
hardening handoff remain open. Retain main's Slint/iced pilots and reaffirmation;
no fresh toolkit-choice comparison or transfer of Win32 evidence is required.
Restore D2 freshness/reconnect/generation semantics before any live read-only
status, then revision/authorization/confirmed-failed-unknown results before
protected commands. Restore D1/D3 before general-user preview/release or an
earlier corresponding qualification claim. **Exit:** [plan §6/P5](../docs/plans/development-plan.md#p5--truthful-accessible-production-ui).

| Future card / role owner | Prerequisites and next bounded action | Independent observation required for exit |
| --- | --- | --- |
| S5.1 real status/sources / UI/service | Admitted Slint adapter/checked bridge from the UI-M0 chain, actual secured current snapshots, relevant P2/P3 evidence and D2 status re-entry; wire one read-only journey using canonical design references. | Valid real status matches authentication/agreement/ages/principal/supporters; unavailable/stale/malformed/old-generation evidence cannot look current. CHECK-14/25, AC-02/03/05; applicable ST-T01/14. |
| S5.2 settings/helper/review / UI/security/platform | Admitted Slint adapter/checked handler bridge, actual receiver/helper/policy revisions and D2 command re-entry, plus applicable P4 authority; expand one current operation and rejection/cancellation pair. | Authorized current request succeeds; UAC cancellation, stale edit, denied/lost/unknown result and replayed approval cannot appear successful. CHECK-2/3/10/15/25, AC-06/07/11. |
| S5.3 diagnostics/tray / UI/diagnostics | Qualified per-process output/privacy and observer lifecycle; freeze/admit any actual Slint tray feature/native graph expansion, then select one export or optional-tray journey. The pilot omitted tray; normal Close must still leave no UI by default. | Useful redacted export and enabled observer work; sink failure/stale status cannot mislead, disabled tray/default close retains no UI, services survive UI exit. CHECK-12/14/17/21/23, AC-01/16/18. |
| S5.4 full usability/resources / accessibility/QA | Reached production journeys, D1/D3 restored, admitted accessibility-enabled Slint graph/Windows bridge and supported display profile; qualify one complete critical journey before expanding the matrix. The pilot's omitted feature and Win32 receipts are no accessibility pass. | Actual keyboard/Narrator/UIA, contrast/scaling/long-text and native resource observations pass; lost focus/inaccessible action/retained resources fail qualification. CHECK-21/22/27, AC-17/18/20. |

### Production UI contract handoff

For externalized translations, follow [OD-15's catalog design](../docs/plans/ui-localization.md).
S0.5 records the implemented source/interface profile and exact branch source-control evidence; merged-candidate and semantic approval remain separate. S5.1/S5.2 project stable codes and typed
arguments into localized status/settings/errors without changing authorization;
S5.3 uses the same catalog for tray and local notifications; S5.4 owns adopted
locale, placeholder/plural, long-text/RTL and actual accessibility observations.
Language is a per-user preference available in both UI modes; English is the
initial adopted/default/fallback language. Separate source/binary files alone
adopt neither additional locales nor arbitrary user-imported packs. This adds
no current native UI-M0 or Core prerequisite.

The [6 November 2026 threat amendment](../docs/plans/ui-localization.md#hostile-catalogs-security-amendment--6-november-2026-asiatbilisi)
treats every raw JSON/CBOR input as malicious until bounded admission, including
English and generated/signed files. Independent code-owned contracts/approval,
isolated conversion and exact file/output custody are required. Critical content
uses reviewed whole-group English fallback; ordinary valid omissions may fall
back per message. S0.5 records finite limits and independent attack/healthy
source controls for the strict producer/loaders. Preserve those exact-candidate
results without inferring native, fuzz, semantic approval or combined-tree success.

Before each relevant S5 child, read the
current [design v0.4](../docs/nevrotick-design-v0.4/README.md) and
[latest HTML audit](../docs/nevrotick-design-v0.4/review/tray/REPORT-RU.md).
Use the retained v0.2
[UI specification](../docs/nevrotick-design-v0.2/docs/02-ui-spec.md),
[source editor contract](../docs/nevrotick-design-v0.2/docs/03-server-configuration.md),
[state/acceptance matrix](../docs/nevrotick-design-v0.2/docs/04-states-and-acceptance.md)
and [feature correspondence](../docs/nevrotick-design-v0.2/docs/06-features-review.md).
Canonical architecture and later owner decisions govern semantics; archived
source copies, MSI wording and rendered scenes are references. Do not execute
the archived React tooling until its recorded dependency findings are resolved.

- S5.1/D2: retain separate source-reply, eligible-estimate, acknowledged-correction
  and UI-snapshot ages. Unknown health differs from confirmed service stop.
  Certificate-time basis/bootstrap history survives later NTS success; calendar
  uncertainty and per-store protection/readiness remain separate published facts.
- S5.1/D2 refresh: coalesce repeated Sync now requests, preserve existing evidence
  and report actual checking/waiting/limited/blocked/unavailable outcomes. A check
  completion does not update last-applied correction; UI close cancels no
  service-owned measurement or correction.
- S5.2/D2: preserve active versus draft policy and personal preferences. Apply
  revalidates at the receiver against the current revision/authority; UAC cancel
  and conflict preserve draft/focus and active policy. New-generation sources
  become Waiting; accepted configuration does not remove Blocked/ownership limits.
  Lost acknowledgement after admission requires result reconciliation before retry.
- Source editing: NTS-KE TCP and negotiated NTP UDP endpoints stay distinct.
  Host/IDN/IP/port validation agrees at UI and receiver; permitted LAN sources work
  beside forbidden post-DNS/delegation targets. Profile changes never silently
  rewrite protocols, manufacture independent votes or reduce minimum agreement.
- S5.3/S5.4: qualify Copy summary preview and explicit export/redaction, source
  detail/recovery history and expired one-use approvals; tray/sign-in choices are
  per-user, off by default, and do not alter services. Restore D1/D3 at their
  existing triggers, including field-associated errors and actual focus/UIA.
  Use the [AVIF manifest](../docs/NevroTick-AVIF-kit-v1/manifest.json) and preserve
  proprietary brand and third-party notices separately from the code licence.

These are later production observations under existing CHECK/AC cases; they add
no UI-M0 implementation or qualification requirement.

## P7 final qualification

**Owner:** independent verifier with security/platform/dependency/release owners;
David remains exact-candidate release approver. **Status:** deferred. Applicable
P0–P6 results, supported environment, selected complete MSIX and actual
Store-delivered bytes/trust precede release closure. Next freeze the candidate
and reconcile earlier evidence invalidation before choosing the first remaining
black-box observation. **Exit:** [plan §6/P7](../docs/plans/development-plan.md#p7--final-integrated-acceptance).

Independent valid journeys must match admitted inputs, actual status/effects,
persisted state and installed payload. Malformed/unauthorized/stale/tampered
neighbors must fail without protected effects while valid controls remain usable.
Reconcile CHECK-1–27, applicable AC-01–22, applicable ST-T cases, critical-file
gates, native accessibility/authority/lifecycle and baseline/soak evidence.
AC-21/22 retain their [OD-11 scope split](../docs/windows-time-features.md#8-acceptance-scenarios-and-evidence):
current direct-path/shared cases remain required; proxy-specific cases keep
their later activation criteria. Missing
required evidence stays incomplete; only David's explicit applicable decisions
can accept exceptions. Fresh review and release approval bind the exact artifact.
CHECK-4 uses the [pinned ASVS worksheet](../docs/asvs-5.0.0-applicability.md):
reconcile its 345 rows and individual outstanding questions against actual
surfaces/evidence, including owner-reviewed N/A/native interpretations and
OD-04/OPEN-9 decisions. This restates the existing gate, not a new compliance claim.

## Integrated slices

These are joins of the same parents, using [feature plan §9](../docs/windows-time-features.md#9-delivery-slices-for-agentic-development);
they introduce no second execution chain or independent readiness claim.
The development coordinator owns activation, one integrated candidate and the
combined exit for each slice; participating role owners retain their existing
boundaries. In particular, S-2 closes only on the joined acquisition, secured
status and real read-only UI observations, not on separate child reports.

| Slice | Parent tasks and observable combined exit |
| --- | --- |
| S-0 foundations | P0 cards, including existing S0.1/S0.6 owners: finite platform/authority/UI/package evidence and explicit dependent decisions/blockers. UI-M0 alone does not close S-0. |
| S-1 checked model | Existing P1 children plus deferred Core completion: independent admission, trajectories, selection/principal/supporters and applicable critical-file gates. |
| S-2 observe-only client | P2 acquisition + P3 secured service status + early P5/S5.1 with real D2 re-entry. Actual admitted NTS/explicit NTP, denied controls and ordinary-user real read-only UI; no global clock writes. Headless pilot or fixture window alone is insufficient. |
| S-3 unattended sync | P3/P4 plus existing installation owner: real bounded clock trajectory before login/with UI closed, actual ownership, stop/restart/recovery and handover. |
| S-4 recovery/admin | P2/P4/P5 under applicable OPEN-5/6/8 decisions: AC-07–14 valid/failure pairs, current one-use approval and no downgrade/hint-derived clock authority. |
| S-5 finished experience | P5 plus existing P6 owner: usable real screens/tray/export, keyboard/Narrator/scaling, resource and interrupted-maintenance observations. |
| S-6 exact release | Existing P6 owner + P7: complete hardened MSIX preparation, David's Store outcome, actual delivered/installed trust and full applicable acceptance before public release. |

Later [roadmap handoffs](../docs/plans/development-plan.md#86-handoff-after-the-windows-plan)
are Windows P0–P7 → KHR-C01 disposition → GEO-C01 disposition → Android, with later RGT-0 → CHR-0
and the [shared time-assurance task](time-assurance-integration.md) retaining
their distinct exits. Future DNS retains its own DoQ-only/DNS-C owners; do not
invent ordering between DNS and Android or add either as a current Core gate.


## Future roadmap parents

**Status:** DEFERRED, outside current Windows P0–P7 execution. These parent cards
retain the [roadmap](../ROADMAP.md) and focused-plan owners; they authorize no
device/host changes, dependency adoption, public probes or clock effects. Expand
only the next admitted child when its future stage is reached. Do not invent an
ordering between DNS and Android. KHR/RGT stay in the existing time-assurance task.

| Future parent / role owner | Entry and first bounded child | Existing exit / controls |
| --- | --- | --- |
| GEO-0 / catalogue/release/engine/QA/security | After KHR-C01, before Android. The [existing catalogue task](server-catalogue-and-statistics.md#deferred-geoip-experiment--after-khronos) freezes a bounded comparison of ordinary acquisition with optional geography ordering, reusing C/R/Q ownership. | GEO-C01 records actual evidence/gaps and `ADOPT_LIMITED`, `DEFER` or `REJECT`; adoption needs measured benefit within unchanged constraints. No changed reserve sampler, trust vote, mandatory client-IP lookup, parallel database or dependency on deferred Roughtime/SOCKS5 runtime. Any GeoIP implementation is separately scoped. |
| AND-0–AND-6 / Android/platform/protocol/ADB/QA | Windows P0–P7, then KHR-C01 disposition and qualified Khronos implementation if adopted, then GEO-C01 disposition before Android. GeoIP implementation is not required. Start AND-0 read-only device/build discovery, then an ordinary-UID loopback fixture under explicit device/restore bounds. | AND-C01–06 retain device/APK, local responder/NTS, reversible ADB setup/restoration, lifecycle and signed-preview evidence. No generic Android clock-write privilege is assumed. AND-6/AND-C07 server scope remains separately conditional. |
| CHR-0 / numerical/protocol/provenance/rights | Immediately after RGT-C01 in the existing RGT-0 → CHR-0 sequence; freeze the source/material ledger, consumer boundary and independent contract before replacement work. | CHR-C01 requires independently checked behavior, complete material/dependency provenance and reviewed rights. Current Chrony GPL obligations remain until an actual qualified replacement; public licence and Microsoft permission decisions stay with David. |
| SOCKS-0 / engine/protocol/Windows/security/QA/UI | Beyond first Windows P0–P7 delivery, when a real use case and maintenance capacity justify it; placement among future work remains open without reordering KHR/Android or RGT/CHR. Reuse the [existing engine task](engine-transport-pilot.md#socks5-feasibility-and-deferral--5-november-2026) to freeze the manual loopback/no-auth/local-DNS subset and qualified relay/provider envelope. | SOCKS-C01 and applicable PATH/AC/CHECK cases: actual CONNECT plus UDP, strict no-escape/recovery, endpoint identity, service lifetime, DNS/verifier routing, timing and bounded resource evidence. Remote/auth/proxy-DNS extensions are separate later admissions, not first-subset requirements. |
| DNS-0–DNS-6 / DNS/platform/security/resources | At its future phase, start [DNS-0 contract freeze](../docs/plans/system-dns-cache-design.md#12-staged-delivery-and-next-action), then the plan's baseline → DNSSEC → native integration → shadow demand → bounded refresh sequence; DNS-6 remains conditional. | DNS-C01–18/DOQ-C01, DoQ-only upstream and leaf/intermediate pinning; qualify actual application routing rather than configuration readback. Apply [source-derived ownership/publication/retained-memory/async controls](../docs/plans/dns-upstream-lessons.md) at DNS-1/2 and demand/forget/privacy/persistence controls at DNS-4/5. W11/ST refinements and optional-storage pause retain their scope. |
