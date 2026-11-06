# Engine measurement mutation equivalence review

**6 November 2026.** Ordinary CHECK-25 mutation accounting for the
[first S2.1 measurement checkpoint](engine-transport-pilot.md#first-controlled-measurement-source-checkpoint--6-november-2026).
This record supplies no machine-checked product proof and resumes no formal work;
[OD-10's post-V1 deferral](../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026)
remains in force. Neither case receives kill credit. Retain actual native
survivors, raw denominators and separately adjusted results in campaign receipts.

Reviewed product identity: 168 inputs, canonical SHA256
`2d7e5cf796fc9f32dbfea780de1c6206dd7060e94fc03001185e384491d147e8`.
The source is [measurement.rs](../crates/nevrotick-engine/src/measurement.rs),
SHA256 `967610067ee0713002c7bec44dd54e2a03416787beb48d57db9b09e5c715a681`.
The engine policy binds each complete actual descriptor, candidate, source and
this record's bytes; changed inputs require renewed review.

## Boundary and independent controls

The boundary is the pure `coherence_residual` helper, including its own errors,
not merely the coordinates reachable through today's `measure` caller. Its
first guard now rejects either reversed individual monotonic bracket. The
following checked duration operations reject unavailable interval subtraction;
their successful results are converted exactly to signed nanoseconds. These
platform operations are not a physical ordering assertion: pinned Windows Rust
can return zero for a small negative interval within its QPC tolerance. A zero
`lower` is no greater than the nonnegative `upper`. A positive `lower` is the
elapsed interval from `base.mono_after` to `later.mono`; the separately ordered
brackets make the interval from `base.mono` to `later.mono_after` at least that
large. Thus successful local admission establishes `lower <= upper` on both
selected platforms. The checked wall subtraction runs before either comparison.

Before that guard, the direct helper accepted individually reversed brackets:
base endpoints 1,000/0 ns, later endpoints 2,000/2,000 ns and wall progress
2,000 ns gave a residual of 1,000 ns. The lower-equality mutant would give zero.
Thus caller ordering alone was insufficient grounds for exclusion. The actual
regression independently reverses each bracket, requires `CaptureOrder`, and
retains an ordered zero-residual neighbor. Its admitted RED observed
`[Ok(1000), Ok(1000)]` instead of two errors; after the guard all 47 ordinary
engine tests and 12 doctests passed, with compiler/Clippy and unchanged bookends.
The private regression is `coherence_helper_rejects_each_reversed_bracket`;
existing nonzero-bracket tests also check interval endpoints, inside/outside
neighbors and the exact model allowance.

Receipts remain under the existing S12e evidence root:
`s21-coherence-helper-red-v1/verification.json` (SHA256
`da934f7a5ada581af08eff13e591af18592e4854fc925e8839771be8e98917d4`)
and `s21-analysis-baseline-v2/verification.json` (SHA256
`24280bdada4d3d063703337389a144e920c2f602510724aa1d31950f64a4418f`).
The original incomplete full campaign remains `s21-three-owner-mutation-v1`;
its earlier source is not promoted to the repaired candidate.

## ENGINE-COHERENCE-LOWER-EQUALITY

Exact change: `wall_delta < lower` becomes `wall_delta <= lower` in
`coherence_residual` (current source line 526). The outcomes can differ only
when `wall_delta == lower`. Successful local admission establishes
`lower <= upper`, so the original takes the zero-residual branch. The mutant
computes checked `lower - lower`, exactly zero without overflow, then performs
the identical successful `u64` conversion. All other comparisons, errors and
results are unchanged. The earlier malformed-bracket counterexample is now
rejected by the same unchanged guard in both variants.

## ENGINE-COHERENCE-UPPER-EQUALITY

Exact change: `wall_delta > upper` becomes `wall_delta >= upper` in
`coherence_residual` (current source line 530). Where the first branch is taken,
both variants are identical. Otherwise only `wall_delta == upper` differs:
the original returns literal zero and the mutant computes checked
`upper - upper`, exactly zero without overflow. The same `u64` conversion and
all failure paths remain. This comparison change adds no I/O, allocation,
state mutation or lifecycle effect.

## Review and invalidation

The coordinator must obtain independent review of these exact source and record
bytes before adopting the two policy records. The scope excludes every other
survivor, constructor, macro, dependency and native Windows behavior. Reconcile
the bracket guard, duration/wall conversions, comparisons, result conversion and
their effects whenever their source or admitted input contract changes. The
policy and subsequent campaign, rather than this prose, establish actual
descriptor identity, observed viability and the final gate disposition.
