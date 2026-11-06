# Engine measurement mutation equivalence review

**6 November 2026.** Ordinary CHECK-25 mutation accounting for the
[first S2.1 measurement checkpoint](engine-transport-pilot.md#first-controlled-measurement-source-checkpoint--6-november-2026).
This record supplies no machine-checked product proof and resumes no formal work;
[OD-10's post-V1 deferral](../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026)
remains in force. Neither case receives kill credit. Retain actual native
survivors, raw denominators and separately adjusted results in campaign receipts.

Historical reviewed product identity: 168 inputs, canonical SHA256
`2d7e5cf796fc9f32dbfea780de1c6206dd7060e94fc03001185e384491d147e8`.
The historical source is [measurement.rs](../crates/nevrotick-engine/src/measurement.rs),
SHA256 `967610067ee0713002c7bec44dd54e2a03416787beb48d57db9b09e5c715a681`.
The engine policy binds each complete actual descriptor, candidate, source and
this record's bytes; changed inputs require renewed review.

## Exact nominal candidate revalidation

The exact nominal representation child has370 product inputs, canonical SHA256
`b0c6e05fce67260e1b03ef392ba11b78a44f0508fb26aa27a3f17bb3541e9732`.
Current measurement source SHA256 is
`ad78facc81705a16adb47a752eea14cb28e6ba14c1281dc20ce72941020710b8`.
The complete `coherence_residual` body and its adjacent contract are byte-identical
to the historical source. New nominal construction runs after its unchanged
calls; the helper's full raw-input/error/effect domain has not narrowed.

Fresh enumeration locates the exact lower equality case at555:29 and upper case
at559:26, function span522:1–567:2. The engine policy binds the full typed
current descriptors, not these line summaries. The separate comparisons rejecting
reversed individual brackets are not equivalent; their equality changes remain
ordinary variants requiring actual killing. This revalidation applies only to the
two wall-delta equality cases below and grants no kill credit. New native outcomes
and the complete338-variant campaign remain independently required.

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


## Seconds conversion candidate revalidation — 6 November 2026

The S1.2 exact-rational-seconds child changes the measurement source and adds
checked nominal seconds adapters. It does not change the coherence operation.
On the complete374-input product map
`abfb2bd3549e2d98b13ce9008a6c57faf09a5fcc5b04577e678ff67b39a10c5e`,
the measurement source hashes to
`85089611c6e2316beeb37f4b060ebec88b1db8686974a81a39c690dbae4fd648`.
The complete adjacent contract plus function remains byte-identical to
`cb223c5`:1,618bytes, SHA256
`9032c1fe2ef9540906a015c3f7115f2addcd54f2f6b02250fb12c1e032e38865`.
This selection starts at the function's documentation and includes its closing
brace; differently delimited historical selections retain their own hashes.

Fresh-context reviewer `/root/normalization_bounded_plan` completed a read-only
revalidation against the actual source, argument, policy and existing review
record. Bracket/duration/narrowing/wall-subtraction failures precede either
comparison and remain identical. Successful bracket admission establishes
`lower <= upper`. At lower equality, the changed branch computes checked
`lower-lower` instead of literal zero; at upper equality it computes checked
`upper-upper`, with identical earlier branch behavior. Both differences are
representable zero; final unsigned conversion and absence of effects agree.
The coordinator independently inspected the same operation. This is manual
mutation-equivalence reasoning, not a machine-checked product proof.

The two existing IDs remain the only permitted exclusions. Current lower/upper
operators are557:29 and561:26; documentation-inclusive function span is
524:1–569:2 (`fn` begins532:1). Fresh enumeration must confirm the complete
descriptors before campaign admission. The bracket-order comparisons at538:18
and538:50 are **not equivalent** under `>` to `>=`: zero-width ordered brackets
are a valid neighboring control. Earlier dated line references remain historical.

Retain actual native outcomes and raw denominators. An exact E disposition
supplies no kill credit; descriptor/source/candidate/review disagreement fails
closed, and every unresolved survivor or timeout keeps qualification incomplete.
No historical score is transferred. Ordinary qualification for this candidate
remains pending until the new terminal receipts. No formal or Windows job ran.
