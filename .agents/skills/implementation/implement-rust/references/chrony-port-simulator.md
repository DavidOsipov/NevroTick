# Chrony port and deterministic simulator recipe

Use for translated/adapted chrony algorithms or their qualifying simulator.
Work one bounded algorithm, owned state and immediate consumers at a time; never
translate whole C program/recreate Unix integration. Reuse adopted runner/pure
simulation seam. This selects no harness library, dependencies, Cargo workspace
or native actions.

The canonical owners are architecture [section 7](../../../../../docs/windows-time-architecture.md#7-chrony-derived-numerical-and-protocol-core)
for the port map/pipeline, [section 10](../../../../../docs/windows-time-architecture.md#10-windows-clock-backend)
for correction accounting and ownership,
[section 18.3](../../../../../docs/windows-time-architecture.md#183-licensing-and-provenance)
for provenance, [section 20](../../../../../docs/windows-time-architecture.md#20-verification-strategy-and-acceptance-checks)
for independent truth and CHECK-6/7/8/19/20, and
[P1](../../../../../docs/windows-time-architecture.md#p1---deliver-the-deterministic-core-and-evidence-harness)
for the pure-core exit. They remain plans until actual implementation evidence
exists. Keep the current GPL/TLS-provider/CNG admission holds in
[section 18.7](../../../../../docs/windows-time-architecture.md#187-tls-nts-and-windows-trust-decisions);
portable numerical success does not clear them.

## Slice inputs and reviewable outputs

Before implementation, establish artifacts in existing task evidence location;
names describe content, not new repository layout.

| Input | Output to retain |
| --- | --- |
| Named behavior, architectural rule, current callers and candidate identity | Slice statement: entry point, owned state, output/effect, excluded behavior, target lane and relevant CHECK IDs |
| Exact upstream source and fixture origins | Port ledger: version **and** commit, file/function/line range, source URL/content identity, original notices and file-specific grant, copied/adapted material, local owner and modification provenance |
| Units, sign, time coordinates, policy and resource limits | Pure domain contract: valid inputs, invalid-input behavior, output/state invariants, predeclared tolerances, deliberate deviations and consumer consequences |
| Independent trajectory/vector facts and event schedule | Reproducible scenario: literal inputs, derivation, seed if used, clock model, event ordering, bounded workload and expected per-event observations |
| Candidate and runner/configuration identities | Evidence: first discriminating failure, healthy control, minimal repair, executed cases/results, raw trace/diff and pass/fail/blocked scope |

Record deviations, including omitted chrony options, with rationale, changed
expectations and affected callers. Missing contract/source or unresolved slice rights
blocks implementation claims; retain separate useful read-only analysis. A ledger is
not legal clearance. Watch upstream fixes affecting retained behavior; record last
checked revision/local owner. Never silently advance pin.

## Extract the algorithm and its contract

Architecture's chrony **4.9 reference** pins commit
`8df4f1263e206585e6c4e61352d042c2dd12916a`, never moving branches. These primary
files/functions were inspected; recheck exact selected body, dependencies/notices
when choosing another slice/pin.

| Pinned primary source | Candidate boundary and translation trap |
| --- | --- |
| [regress.c](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/regress.c), `RGR_WeightedRegression` | Record whether a parameter is a variance divisor or precision. This function divides by `w`; larger values reduce influence. Preserve the estimator's sample-count and denominator preconditions. |
| [sourcestats.c](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/sourcestats.c), `SST_DoNewRegression`, `SST_GetSelectionData`, `SST_SlewSamples` | Bound history, retain aging/pruning and uncertainty semantics, and account for coordinate changes caused by applied corrections. |
| [sources.c](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/sources.c), `SRC_SelectSource`, `combine_sources` | Separate eligibility/majority, principal hysteresis and supporter compatibility. Offset weighting uses inverse root distance; frequency weighting uses inverse squared frequency standard deviation. |
| [reference.c](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/reference.c), `REF_SetReference` | Separate oscillator learning, phase error and abstract correction decisions; preserve relevant elapsed-time and uncertainty semantics. |
| [local.c](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/local.c), `LCL_AccumulateFrequencyAndOffset` | Map requested versus driver-returned frequency and sample-coordinate notifications into explicit domain events. |
| [sys_generic.c](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/sys_generic.c), `update_slew`, `apply_step_offset` | Learn from achieved-correction accounting, not Unix calls/constants. Its step-offset convention must be mapped explicitly to NevroTick's positive-behind convention. |

The inspected file headers grant GPL version 2 without an “or later” grant.
Preserve the actual source notices in the implementation/provenance artifacts;
neither language translation nor a COPYING boilerplate example changes the
file-specific grant. Broader dependency and distribution conclusions remain with
the architectural license decision.

Define the pure contract before porting:

- Keep raw NTP/Windows timestamps, counters and their checked differences exact
  in integer/fixed-point form before converting small deltas for estimation.
  State epoch/era, units, offset direction and frequency convention at every seam.
  Product positive offset means the local Windows clock is behind and should
  advance; upstream internal coordinates need explicit mappings.
- Make the core consume validated values, explicit time readings and backend
  outcome events, and return estimates/rejections/proposals plus owned state.
  Keep network, real timers, persistence and raw Windows APIs outside `time-core`.
  A pure proposal carries no privilege to apply it.
- Declare history/event/resource bounds and the adopted thresholds from their
  current policy owner. Specify insufficient/degenerate samples, duplicate time
  readings, stale generations, zero denominators, NaN/infinity, overflow,
  quantization and signed rounding. Reject invalid results before state/effect
  admission; do not mask them with a plausible estimate or invented default.
- Separate numerical tolerance from a physical error envelope. Choose each from
  the scenario and contract before seeing candidate output. Exercise just below,
  exactly at and just above each relevant adopted boundary in representable units.

## Build independent facts before the port

Use [numerical truth](../../../testing/references/numerical-protocol.md) and existing
[golden](../../../testing/qa-golden-fixture-parity/SKILL.md),
[property](../../../testing/qa-property-based-testing/SKILL.md) or
[model](../../../testing/qa-model-based-testing/SKILL.md) owner for selected method.
No parallel framework or production-generated expected values.

Useful small analytic fixtures, selected only when relevant to the slice:

- Millisecond-relative `T1=0, T2=70, T3=80, T4=110` represents true server offset
  `+20 ms`, symmetric `50 ms` paths and `10 ms` processing: estimated offset
  `+20 ms`, network round-trip delay `100 ms`. Change only paths to `20/80 ms`:
  `T2=40, T3=50` gives estimated offset `-10 ms`, delay `100 ms`, bias `-30 ms`
  relative to the same truth. Authentication cannot remove this physical bias.
- For a regression boundary using variance divisors, independently solve the
  weighted normal equations for `x=(-1,0,1) s`, `y=(-1,0,3) ms`,
  `w=(1,1,4)`: intercept at `x=0` is `4/9 ms`, slope `5/3 ms/s`. Contrast an
  exact straight line and equal-weight control. This checks weight direction;
  history selection, skew and uncertainty require their own expectations.
- Give sources explicit offset intervals and distinct logical identities. Derive
  majority/absence of majority on paper, then add an endpoint alias that must not
  gain a vote. Compare the configured minimum independently of majority. Include
  an eligible agreeing set as the healthy control.

Regression/history slices cover known slope/intercept, asymmetric jitter,
sample aging/pruning and acknowledged-correction compensation. Selection slices
compare estimates aged to **same** instant, eligibility, principal/supporter/rejected
sets/reasons, uncertainty and nearly-equal-peer hysteresis. Use one/two falsetickers
with explicit populations and two-source disagreement; do not assume majority survives.
Without majority, allow no new disputed phase correction; never silently reduce
configured minimum or fall back to single-source profile.

For discipline/holdover assert full trajectory: finite phase budget separate from
stable oscillator compensation, uncertainty growth/honest freshness, reacquisition
after discontinuity and accepted leap/time-scale ownership. Compatible supporters
need not be independent error samples. Never average away all uncertainty or infer
tracking from reachability.

## Deterministic event simulator seam

Use a portable explicit interface, with no real sleeps, sockets or clock writes.
Keep scenario truth independent of production estimates and retain these separate
coordinates:

| Coordinate | Scenario semantics |
| --- | --- |
| True UTC trajectory | Independently defined reference truth, oscillator/source faults and network paths; never computed by the production controller |
| Observed local UTC | Evolves from oscillator error and modeled actual actuation; wall-clock steps change this mapping, not authorization time |
| Monotonic elapsed time | Orders events and expires deadlines through sleep; UTC steps cannot extend authorization |
| Active-awake elapsed time | Advances only while awake; sleep duration is not evidence of delivered transient slew |

Explicit events cover packet send/delivery/loss/reorder, timer expiry,
configuration/association and clock-generation changes, suspend/resume,
external/self-induced steps, proposal submission, backend application/acknowledgement.
Declare deterministic ties, seeds, event count/horizon/queue bounds and fresh resets
for standalone literal-failure replay. Generate exchanges from declared path/oscillator
model, never implementation estimator.

Maintain an independent physical-effect ledger and the controller's knowledge of
those effects. Include rejected-with-no-effect, applied-with-quantization,
lost/duplicate/delayed acknowledgement and ambiguous outcome cases. A lost
acknowledgement does not prove no effect; a request is not an applied correction.
Reconcile late outcomes by operation identity without granting stale proposals new
authority. Duplicates/retries must not repeat a step, spend the same correction
twice or reopen cumulative/count budgets on reconnect, sign change or same-boot
restart. Preserve uncertainty when application cannot be established.

Before replacing/cancelling a plan, account for actual delivered correction in
the correct awake/time domain, including rate representation and actuation
latency. Transform historical samples consistently or retain stable coordinates;
the controller must not learn its own phase correction as oscillator drift.
Suspend/resume invalidates crossing exchanges/proposals through generation
changes, remaps clocks and retains only the adopted bounded drift prior. A
successor/ambiguous clock owner blocks writes and global resets; an old journal
or matching numeric rate is not current ownership. Model that decision without
claiming the simulation proves Windows ownership detection.

## Complete one slice with bounded evidence

1. Freeze the slice contract, upstream ledger, scenario inputs, expectations,
   candidate and selected runner configuration. Verify that the relevant cases
   are discovered and a failing case reaches the intended algorithm/effect seam.
   Setup/compile/zero-case failure is not behavioral RED.
2. Before repair, exercise a discriminating failure and a healthy neighboring
   case; for a new algorithm, establish the missing behavior before or alongside
   minimal implementation. Retain exact observations and the competing wrong
   behavior ruled out, such as inverted weights, mixed-time comparison or
   request-as-delivery accounting.
3. Port/repair only the causal owner and necessary immediate consumers. Run the
   focused failing, healthy and relevant boundary/recovery cases against the
   resulting candidate. Preserve original observations; do not rewrite expected
   facts from a new snapshot to obtain GREEN.
4. Add original-versus-port differential evidence when useful and available:
   pin upstream executable/source/build/flags, configuration and adapter/scenario
   mapping. Compare per-event selection, reasons, polling, uncertainty and the
   correction trajectory, with exact protocol facts and predeclared numerical
   tolerances. Explain deviations rather than normalizing them away. Agreement
   with chrony complements analytic/model facts; the same copied bug can agree.
5. Retain scenario/trace/ledger identities, commands, reached case counts, first
   failure and final results, unresolved deviations and pass/fail/blocked status.
   For CHECK-19, prove the actual runner/gate propagates a relevant intentional
   failure and accepts its valid control when that gate is in scope. CHECK-20's
   source/license/build inventory is separate from numerical pass results.

For clknetsim follow architecture section 20.1: pin revision; prove exercised
I/O/timer interception or compare captured chrony trajectories through explicit
simulation interface. Linux interception is no Windows service harness. Never
install/build imported source merely for documentation/planning completion.

Report exact qualified scope. Pure algorithm/simulator slices may pass while
differential evidence, prerequisites/native Windows checks remain blocked/unassessed.
P1's CHECK-6/7/8/19/20 and pure admission subset need separate evidence; one slice
cannot complete phase. Native sign, quantization, sleep/Modern Standby, ownership
and actual clock trajectory remain separate P0/P4 Windows obligations.
