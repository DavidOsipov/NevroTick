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

## Raw unsigned guard candidate revalidation — 6 November 2026

The rational-seconds raw-sign admission and direct raw-rule controls change the
complete374-input product map to
`b1852a81314a564c4923a7ed1be7d293da730ac2b7dd4152eca539f3a4474d35`.
The engine measurement source remains
`85089611c6e2316beeb37f4b060ebec88b1db8686974a81a39c690dbae4fd648`.
Independent reviewer `/root/rational_equivalence_review` compared the complete
adjacent coherence contract and function with `cb223c5`: the1,618 bytes are
identical, SHA256
`9032c1fe2ef9540906a015c3f7115f2addcd54f2f6b02250fb12c1e032e38865`.
Current lower/upper operators and documentation-inclusive span remain557:29,
561:26 and524:1–569:2 respectively.

The review covers the helper's full raw `SoftwareClock`/`ConvertedWall` domain,
not just successful `measure` callers. Both variants first reject either
reversed individual bracket, unavailable checked duration, failed narrowing or
wall-subtraction overflow identically. For successful durations, `lower` and
`upper` are nonnegative. If `lower=0`, it is no greater than `upper`. Otherwise,
the ordered endpoints make the interval from `base.mono` to `later.mono_after`
at least as long as the interval from `base.mono_after` to `later.mono`; their
monotone duration/nanosecond conversion preserves `lower<=upper`. A platform's
permitted small-negative-to-zero duration behavior cannot produce a negative
bound or reverse this ordering.

At `wall_delta=lower`, the lower-equality mutation computes checked `lower-lower`
instead of the original zero branch; `lower<=upper` excludes a conflicting
upper branch. The exact representable zero and final conversion agree. At
`wall_delta=upper`, after the identical first-branch decision, the upper-equality
mutation computes checked `upper-upper` instead of literal zero. All unequal
cases, errors and absence of allocation/I/O/state/lifecycle effects agree.
The bracket `>` to `>=` variants remain non-equivalent because zero-width
ordered brackets are legal. No other variant is excluded by this argument.

This independently revalidates only `ENGINE-COHERENCE-LOWER-EQUALITY` and
`ENGINE-COHERENCE-UPPER-EQUALITY` for the current source. It supplies no kill
credit or historical-score transfer. Fresh complete descriptor, source,
candidate, review-hash and native-outcome admission remains required before
the current campaign's gate can pass. No Cargo, formal or Windows job was run
for this revalidation.

## Refreshed graph revalidation — 6 November 2026

Reviewer `/root/rational_equivalence_review` independently revalidated only
`ENGINE-COHERENCE-LOWER-EQUALITY` and `ENGINE-COHERENCE-UPPER-EQUALITY` on the
refreshed 374-input product map
`cb2925bb1288a0d844ec7ce702ffa88a0155be4530dd851dda7e06d20fad25f3`.
Current files match the retained dependency-qualification map. Measurement
source remains
`85089611c6e2316beeb37f4b060ebec88b1db8686974a81a39c690dbae4fd648`;
the complete 1,618-byte coherence contract/body remains
`9032c1fe2ef9540906a015c3f7115f2addcd54f2f6b02250fb12c1e032e38865`.
This continues the named independent review rather than relabelling it as a new
fresh-context review.

The graph/Rustup refresh changes neither the helper, its raw
`SoftwareClock`/`ConvertedWall` domain nor the selected Rust 1.99.1 duration and
checked-integer operations. Tokio's updated macro/Syn edge does not execute
inside this pure helper. Reversed bracket, unavailable duration, narrowing and
wall-subtraction failures remain identical before the comparisons. On success,
ordered endpoints establish nonnegative `lower<=upper`, including Windows'
small-negative interval clamp to zero. At lower equality the changed branch
computes exact checked `lower-lower` rather than zero; at upper equality it
computes exact checked `upper-upper` rather than zero after the identical first
branch. All other results, errors and absence of allocation/I/O/state/lifecycle
effects agree. Zero-width brackets remain legal: their separate `>` to `>=`
guard variants are still non-equivalent.

The retained `rational-seconds-engine-enumeration-v2` contains 99 measurement
descriptors. Canonical complete descriptors excluding native `diff` are
`0b710d00684a4a9fe260c6c5d35063e5996a9359e39380fc72decdaa6b7d3a3a`
for557:29 (`<` to `<=`) and
`364f2471bb106936bdb7f5568530c76e127946296dfcf141121dfcb3a8d15970`
for561:26 (`>` to `>=`). The coordinator must match fresh descriptors and
current source/candidate/review hashes and actual native outcomes before
adopting either E entry. Retained enumeration is not current execution. These
arguments supply no kill credit, historical-score transfer or retroactive pass
for an incomplete campaign. No Cargo, native Windows, formal or solver job ran
for this revalidation; ordinary per-file qualification remains the next exit.

## ENGINE-NOMINAL-ZERO-SIGN

Independent reviewer `/root/rational_equivalence_review` inspected the new
native survivor over `nominal_seconds`' complete raw `i128` numerator and `u64`
denominator domain, including zero denominator, zero numerator and `i128::MIN`.
Verdict: **independently justified equivalent**, conditional on final candidate
and configured descriptor/source/review/outcome admission. The measurement source
is `85089611c6e2316beeb37f4b060ebec88b1db8686974a81a39c690dbae4fd648`;
the actual shared constructor source is
`2eeffceacf47251e7d917f90b59af12aec768e0a902465d32338ca75dae9a438`.
No caller restriction to today's nominal capture envelope is assumed.

Exact native descriptor: package `nevrotick-engine`, file
`crates/nevrotick-engine/src/measurement.rs`, genre `BinaryOperator`. Its exact name is
`crates/nevrotick-engine/src/measurement.rs:754:29: replace < with <= in nominal_seconds`;
replacement `<=`, operator span754:29–754:30. Function `nominal_seconds` has
return type `-> Result<RoundedRationalSeconds, MeasurementRefusal>` and
documentation-inclusive span741:1–762:2. The canonical complete descriptor
SHA256, excluding native `diff`, is
`21c6b249ac1075b0320ff846a839c688b9693e73ffbf3f886ef5ed6be14822d9`.
It matches both the actual `engine-measurement-coverage-mutation-v1` outcome and
the retained `rational-seconds-engine-enumeration-v2` descriptor exactly.

For every nonzero numerator, `<0` and `<=0` choose the same sign. Both variants
then execute the same nonzero-denominator check, `unsigned_abs`, shared
constructor and fixed-error mapping with identical arguments. This includes
`i128::MIN`, whose unsigned magnitude is `2^127`; no overflowing signed `abs`
is introduced. Thus all returned exact tuples, floating bits/bounds, validation
and checked-operation failures, allocations and other effects agree.

For numerator zero and denominator zero, the sign branches differ, but the same
`NonZeroU64::new(0)` refuses before either constructor call. Both return the same
fixed `MeasurementRefusal::Arithmetic`. The closed Copy sign enum has no Drop,
allocation, I/O or state effect, so this different temporary sign is unobservable.

For numerator zero and any positive `u64` denominator, both have magnitude zero
and the same checked denominator. Original chooses Positive and mutant chooses
Negative. The actual `RoundedRationalSeconds::new` immediately replaces either
requested sign with Positive when magnitude is zero, **before** calling
`round_magnitude` or `publish`. Both therefore execute the same exact-zero
conversion and the same validation/publication path with identical inputs:
retained Positive sign, numerator zero, original denominator, positive-zero
seconds and positive-zero arithmetic bound. No validation report or allocation
path differs. The original requested sign is neither retained nor formatted.
There is no ambient read, callback, partial output or owner-state effect.

This is materially different from the historical unguarded publication OR/XOR
case at432:59: that mutation changed the bits presented to Garde and its report
allocation path for raw negative zero. That historical case remains **not E**.
Here canonicalization precedes every downstream mathematical/validation use of
the only differing temporary value. Any movement/removal of that canonicalization,
change to the sign type/effects, wrapper validation order or constructor/consumer
behavior requires renewed review of this argument.

The native v1 campaign's raw result remains99 variants:70 caught,26 unviable,
3 missed and0 timed out. Its raw `outcomes.json` SHA256 is
`94cb742c5cda5e53414bd95457786b04910fc3e88c802e7871ef80ea8a675f0f`;
this variant's raw diff SHA256 is
`cde36d47728a94c35f2d9691961f1c2c01c447fcd0367bd6df0a433d28e3728d`
and raw log SHA256 is
`107e03888c5202303670c911dba6b243ce57dd5e6a62b14c888ad0e243886b46`.
The wrapper receipt originally bound product map
`cb2925bb1288a0d844ec7ce702ffa88a0155be4530dd851dda7e06d20fad25f3`
and remains **incomplete** after bound-input drift. Final complete-candidate
identity awaits the coordinator's MSRV reconciliation; this source-bound
equivalence does not admit the drifting tree or supply a current gate pass.
Retain the missed outcome and show any later E adjustment separately, with no
kill credit. This review ran no Cargo, Windows, solver or formal job and changed
no source, controller, policy or execution hold.

## Commit-age fallback review — 7 November 2026

Independent reviewer `/root/rational_equivalence_review` reviewed the two
`commit_reply` counter-update survivors below under the existing ordinary
CHECK-7/8/11/14/25 ownership, timing and mutation contracts. Verdict:
**independently justified equivalent for the pinned Linux host configuration**,
subject to exact configured source/descriptor/review/outcome admission. This is
a source argument, not an executed formal proof or a killed-mutant result.
The three historical measurement E cases above retain their separate scopes.

The retained `relative-bridge-library-gates-v1/verification.json` binds the
376-input product candidate
`484784f792cce9d4ee58b7ba78dc7eb712ab728e3b013b3cb41a0b3e979f5019`.
Re-admitting its frozen `work/source` reproduces that exact product digest;
the report's larger 384-entry `source_sha256` also includes eight execution
helpers and is not the product-only map. Current and frozen
`crates/nevrotick-engine/src/lib.rs` both have SHA256
`6f3c6eb2f491de7f27a89f8fba84556e818874a9c875643b08081206f5dd4d46`.
The 2,066-byte adjacent contract and complete `commit_reply` body, from
`/// Commit checked packet facts` up to the next helper's documentation, hash to
`307ec2ede4f337700543029ce2664e57f5958338286f1130a8b1d1c600653290`.
Any later test/helper change has a new candidate that the coordinator must bind;
this review does not manufacture that identity or transfer a campaign pass.

### Full helper boundary, counterexample search and effects

The argument applies to every valid `LiveSlot`/immutable `StagedReply` state and
every representable valid `Instant` pair admitted at this helper, independently
of the present `process_reply` caller. No deadline, capture chronology or
caller-provided nonnegative-age precondition is assumed. Invalid Rust values,
data races and corrupted private standard-library representations are outside
the safe typed domain.

First, any different request/run identity, peer, local endpoint or profile, or
any Sending/Done phase, returns identically before either age check. For a
matching Pending state, the helper itself compares the immutable copied
`processing` value with `candidate.read.mono_after`. Every strictly smaller
value takes the unchanged earlier rejection update and return. This includes
a one-nanosecond predecessor and a physical clock anomaly whose stored value
actually moves backward. Both originals and mutants have the same earlier
counter effects, including any counter-bound failure in an outlying raw state;
neither proposed mutation changes that earlier update.

The unchanged explicit order guard is at 1330:5–1333:6; the checked-duration
`None` fallback is at 1334:5–1337:7, with its only changed operation at 1335.
Only stored pairs with `processing >= read.mono_after` reach the checked
subtraction. In the selected Rust 1.99.1 Linux implementation:

1. Public `Instant` and its Unix `Instant`/`Timespec` representations derive
   total `Ord` from the same stored seconds/nanoseconds used for subtraction.
2. Public `checked_duration_since` calls Unix `checked_sub_instant`, which
   returns `self.t.sub_timespec(&other.t).ok()`.
3. For `self >= other`, `sub_timespec` returns `Ok(Duration)` directly.
   Its full signed-seconds difference fits `u64`, including
   `i64::MAX - i64::MIN = u64::MAX`. If nanoseconds require a borrow, ordering
   establishes strictly greater seconds, so decrementing the later seconds
   cannot underflow and the result still fits. Normalized nanoseconds stay
   below one billion. Equal endpoints give `Some(Duration::ZERO)`.

Thus the `None` branch containing the two changed counter operations has no
execution in this complete valid host domain. Endpoint equality, adjacent
nanoseconds on either side, borrowing across a seconds boundary, negative
representable seconds and both signed-seconds extremes do not supply a
counterexample. These cases were challenged by source arithmetic; this review
did not execute new Rust cases or claim that all internal extrema are
constructible through today's public producer.

Consequently every executed instruction path, return, rejection counter,
`last_rejection`, send/read history, terminal transition and result is the same.
For accepted control/header paths the computed age is identical; the same
measurement update, `Box` allocation, `Arc` reference-count changes and retained
socket/run/request ownership occur. Borrowing, panic/allocation-failure paths,
drops and eventual resource cleanup are unchanged. In the unreachable branch,
decrement could otherwise underflow and multiply-by-one would otherwise omit
an increment: those operations are not arithmetic identities for this counter,
and their equivalence relies on the helper-local guard and pinned subtraction
implementation, not on bounded receive counts or missing assertions.

The public standard-library warning about monotonicity does not overturn this
argument: logical capture chronology can disagree with stored numeric order,
but the helper checks stored numeric order before subtracting those same
immutable values. There is no new OS clock read in Linux's subtraction.
Source/guard reordering, different `Instant` semantics, mutable operands or a
new path into the fallback requires renewed review. Retain the defensive
fallback and the explicit predecessor guard; neither is removed by this review.

### Pinned primary source and platform limit

Primary source was read from the actually selected
`/home/david_osipov/.rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/`:

| Source | Relevant lines | SHA256 |
| --- | --- | --- |
| `std/src/time.rs` | 153–156, 340–342 | `dbf93afc64e22548a08fbcfa68290a63b657933eb006ee6670270f6eddc0f9c3` |
| `std/src/sys/time/unix.rs` | 49–52, 77–79 | `2250ca181d117737d32734c50156a7a9e74852251a17bf70b2a002b8d6fbac43` |
| `std/src/sys/pal/unix/time.rs` | 27–31, 112–145 | `6e1caa7b5e1c9a26b0b8cd02d8fd67c0fc57e4de11c89877d01c2b69eb32031a` |
| `std/src/sys/time/windows.rs` | 13–18, 49–58 | `3e344b796c406f5446dc0d31bb7737302723f21b4c606ed2b30794731e9b53de` |
| `core/src/time.rs` | 79–85, 726–741 | `12efe985ec205c32a8813a64c889464f5d7692d3a5fe97ab3e12088d9964b526` |

The pinned Windows source also derives order from its stored `Duration` and,
for an ordered pair, takes `Duration::checked_sub`, which succeeds. Its
small-negative epsilon clamp explains why the explicit predecessor guard is
necessary; the helper rejects those pairs before the clamp. This is supporting
Windows **source evidence only**. The actual reviewed mutant executions and
E accounting are scoped to `x86_64-unknown-linux-gnu`; no native Windows
qualification, clock accuracy, lifetime campaign or execution-hold release is
claimed.

## ENGINE-COMMIT-DURATION-FALLBACK-SUB

Exact native name:
`crates/nevrotick-engine/src/lib.rs:1335:23: replace += with -= in commit_reply`.
Package `nevrotick-engine`, file `crates/nevrotick-engine/src/lib.rs`, genre
`BinaryOperator`, replacement `-=`, operator span1335:23–1335:25. Function
`commit_reply` has native `return_type` `""` (implicit unit) and
documentation-inclusive span1315:1–1364:2. The complete typed descriptor,
excluding rendered `diff`, hashes to
`7367d98456163b7ac5194b8423d289121a3b30bc1e60910bc67f0f3dfb294c39`.
It matches the actual v1 listing and `MissedMutant` outcome exactly.

The full helper argument above applies: the changed `slot.rejected -= 1`
exists only in the unreachable checked-duration `None` branch. No execution
reaches its potentially different counter result or underflow behavior.
Actual Build and Test phases both report `Success`; the raw log ends
`*** result: Success` and contains 101 passing tests across 68 ordinary records
and 33 doctests, zero failed/ignored records and zero observed resource-error
markers. Raw diff SHA256 is
`788558275d28d7cd5192d70ab9df35d514638f8733bf2bc5dbd2385afb60ce80`;
raw log SHA256 is
`dfe9276d20199e875861d4250dc13c390d13b6205a901bccdd89d8c0019989e9`.
These observations establish the actual survivor and healthy execution, not
the equivalence argument by themselves.

## ENGINE-COMMIT-DURATION-FALLBACK-MUL

Exact native name:
`crates/nevrotick-engine/src/lib.rs:1335:23: replace += with *= in commit_reply`.
All descriptor fields and spans are the same as the preceding case except
name and replacement `*=`. The complete typed descriptor, excluding rendered
`diff`, hashes to
`640fe4b14f5d217b3662ca08f74bcfee0ad41eff9d9a613434fee7e0f15178d0`.
It matches the actual v1 listing and `MissedMutant` outcome exactly.

The full helper argument above applies: the changed `slot.rejected *= 1`
exists only in the unreachable checked-duration `None` branch. Multiplication
by one is not the original increment; absence of any execution of this branch
is the reason their helper behavior agrees. Actual Build and Test phases both
report `Success`; the raw log ends `*** result: Success` and contains 101
passing tests across 68 ordinary records and 33 doctests, zero failed/ignored
records and zero observed resource-error markers. Raw diff SHA256 is
`d0e9117c3c6a09ce8515693dfcc051f27aaa8074b4925342cb9a11835ba0c887`;
raw log SHA256 is
`7a1d6210e3eaf59a744ffdcdcae7758ae33c2d50f450989d5373b545a9c0fd5a`.

### Retained outcomes and adoption boundary

Both raw logs/diffs remain under
`~/.local/share/nevrotick/dependency-refresh-20261106-v1/relative-bridge-library-gates-v1/work/mutation/library/mutants.out/`.
The exact raw `outcomes.json` SHA256 is
`b85ba86ab45e947c51cd508409c514770a6b832ef0ac5a178e2ec2d498c47597`;
the wrapper `verification.json` SHA256 is
`2be4368a1b950814ab85a70363e72a4465f39cc9d4fcb3bb429ecc6ddd728c19`.
The unmodified baseline also completed 101 passing tests without failed/ignored
records; its raw log SHA256 is
`80eaf7c9da03391912cd2786f752db0210e7571f7d52a040d9f4472764ad9a18`.
The two mutant logs and baseline were independently checked for disk-full,
spawn/execution/permission, allocation-failure, panic, FAILED and TIMEOUT
markers; all selected marker counts were zero. This bounded scan does not
characterize the campaign's separate timeout causes.

The v1 raw campaign remains 211 variants: 92 caught, 114 unviable, 2 missed and
3 timed out. Its mutation command exited 3 with no runner-level `failure`,
and the wrapper remains **incomplete**, `qualification_passed=false`.
Keep that raw outcome unchanged. Neither E receives kill credit; the three
timeouts remain unresolved by this review. Fresh final candidate/source/review
bindings, exact descriptor matching, separately displayed E adjustment and
the remaining ordinary gate results are still required. No product, policy or
controller was changed for this review. No Cargo, native, solver or formal job
was run; post-V1 formal deferral and all other execution holds remain in force.

## N3 current series fallback pair — 7 November 2026

This is a new source-bound review for the explicit `series` profile, not an
inheritance of either historical engine E cohort above. Independent reviewer
`n2_arithmetic_independent`, with a separate challenge by `series_role_review`,
reconciled the actual current descriptors and the installed primary sources.
The source-only verdict is **independently justified equivalent for the pinned
Linux host configuration**, over the full valid helper domain below. No formal
compiler/solver job, Windows execution or new mutant execution was performed by
this review. The coordinator separately observed the terminal native outcomes.

The reviewed 388-input product candidate is
`5cc85952bffb47333fcfda3cfd89a64b061ed20f12b2b42eb1283b00b9e6244e`.
Current and frozen enumeration `crates/nevrotick-engine/src/lib.rs` have SHA256
`10e047bf2953becf82b9b6ce714dfcaf06a292f98084702c762eefc757423821`.
Its complete adjacent contract and `commit_reply` body, lines1324–1373 inclusive,
are 2,066 bytes with SHA256
`307ec2ede4f337700543029ce2664e57f5958338286f1130a8b1d1c600653290`.
The current series owner is
`bcbd2fec35a0e498e7ef05e9470d188f8a1ef225b5d32c1bd2ae26843c7bd515`.
Changes to other test inputs require a newly bound complete candidate and actual
outcomes; these recorded identities are not a future-candidate admission.

The actual `n3-series-enumerate-v2/verification.json` SHA256 is
`69587fde48e56808f922e42e615146bfd9af7d0fbe2ba84a77b1785c1b87680f`.
Its 214-descriptor library inventory has canonical SHA256
`b11ba4426343ee85bc7fd2db73f65bcd5d9f948d261c2fe0c5e5bfcf0e10375b`;
the exact captured native library listing bytes hash to
`908b7655df2f2d8f1c4ce3bbbc24ed8404860267af3d84a376e3a827f4b65f59`.
Enumeration selected package `nevrotick-engine`, the library file, default
features and the pinned Linux host toolchain. It establishes no execution score.

### Complete helper domain and effects

The argument covers every valid `LiveSlot` state, immutable `StagedReply` and
representable valid `Instant` pair admitted by this helper. It does not narrow
the helper to today's `process_reply` or `publish_series_reply` callers, assume
clock monotonicity, impose capture/bracket chronology, require a successful
measurement, or rely on a bounded normal rejection count. Invalid Rust values,
data races and corrupted private standard-library representations are excluded
by the safe typed domain, not by an invented caller precondition.

Mismatched request/run identities, endpoints or profile, and any non-Pending
phase, return unchanged before either time check. For a matching Pending state,
the helper-local raw predecessor guard at1339–1341 rejects every stored
`processing < candidate.read.mono_after` pair using the unchanged increment.
This includes a one-nanosecond predecessor and stored observations of a clock
anomaly. Arbitrary private `u16` counter values do not refute equivalence:
overflow/panic/wrapping behavior at this earlier increment is unchanged.

The remaining inputs satisfy `processing >= candidate.read.mono_after` for the
same immutable operands passed to `checked_duration_since`. Public `Instant`,
Unix `Instant` and `Timespec` compare the same stored seconds/nanoseconds used
by subtraction. Unix `checked_sub_instant` calls `sub_timespec(...).ok()`.
Its ordered branch always returns `Ok(Duration)`: the full nonnegative `i64`
seconds difference fits `u64`, including `i64::MAX - i64::MIN = u64::MAX`.
When nanoseconds require a borrow, strict seconds ordering makes subtracting
one safe; resulting nanoseconds remain below one billion. `Duration::new`
therefore needs no normalization or overflowing seconds addition. Equality
gives `Some(Duration::ZERO)`. Signed-second extrema, negative seconds, adjacent
nanoseconds and second-boundary borrowing supply no counterexample.

There is no intervening await, operand mutation or additional OS clock read in
this comparison/subtraction chain. Thus the `None` fallback containing line1344
has no execution in this entire valid pinned-host domain. Subtraction by one
and multiplication by one are not equivalent to incrementing a counter on a
reachable branch; the unchanged local guard and actual subtraction semantics
are essential. No missing assertion or ordinary test pass establishes this
conclusion on its own.

All reachable returns, counters, retained rejection causes, send/read history,
terminal transitions and measurement age are identical. The same accepted
control/header paths perform the same `Box` allocation and `Arc` operations;
allocation failure, panic, borrowing, drops, socket/run/request custody and
eventual cleanup are unchanged. Source/guard reordering, mutable operands, a
new path into the fallback or different standard-library/target semantics
requires renewed review. Retain both defensive checks in product source.

### Current pinned primary sources

Primary files were reread from
`/home/david_osipov/.rustup/toolchains/1.99.1-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/`.
The installation manifest identifies Rust/std/rust-src as
`1.99.1 (58cf8f9c3 2026-10-13)`; its SHA256 is
`48419307eb32e389a25ac360b6bccd4d196f3c4f35ce9d67a223319eff017c6c`.
The enumeration receipt selects this toolchain root and records unchanged
before/after full tool admission. This is a source/provenance argument, not a
machine-checked proof of compiler or installed binary correctness.

| Primary source | Relevant lines | SHA256 |
| --- | --- | --- |
| `std/src/time.rs` |153–156,340–342| `dbf93afc64e22548a08fbcfa68290a63b657933eb006ee6670270f6eddc0f9c3` |
| `std/src/sys/time/mod.rs` |20–29| `449afdd86955252a302d889032a6cb2cbb04dd786f8cb81d9a9059ad933e7e4c` |
| `std/src/sys/time/unix.rs` |49–52,77–79| `2250ca181d117737d32734c50156a7a9e74852251a17bf70b2a002b8d6fbac43` |
| `std/src/sys/pal/unix/time.rs` |27–31,112–145| `6e1caa7b5e1c9a26b0b8cd02d8fd67c0fc57e4de11c89877d01c2b69eb32031a` |
| `core/src/time.rs` |79–85,194–205| `12efe985ec205c32a8813a64c889464f5d7692d3a5fe97ab3e12088d9964b526` |
| `core/src/num/niche_types.rs` |57–78,96–98| `1a74080c73dd7687b08e667d5611d4068a1b7556311f0b58e44edadb17d0a160` |

## ENGINE-SERIES-COMMIT-DURATION-FALLBACK-SUB

Exact native name:
`crates/nevrotick-engine/src/lib.rs:1344:23: replace += with -= in commit_reply`.
Package `nevrotick-engine`, file `crates/nevrotick-engine/src/lib.rs`, genre
`BinaryOperator`, replacement `-=`, operator span1344:23–1344:25. The function
has implicit-unit native `return_type` `""` and span1324:1–1373:2. Its complete
typed descriptor, excluding rendered `diff`, has canonical SHA256
`fb8ea69cd621b8843440c420b136eac359ec2409e0436f291f13cc0c5742377d`.
The current full-domain argument above applies only to this exact fallback
replacement. The actual terminal native outcome is `MissedMutant`, with both
Build and Test phases `Success`. Its diff SHA256 is
`5eed1f4d86a1fa2bb690eb67eb15bf5803928a4f61cefa77e2e7dee85378d745`;
log SHA256 is
`fdec6f408a4ddc86f259efa7079b1368486daf5ed6960b960ae71f3d00f4a6f0`.

## ENGINE-SERIES-COMMIT-DURATION-FALLBACK-MUL

Exact native name:
`crates/nevrotick-engine/src/lib.rs:1344:23: replace += with *= in commit_reply`.
All typed descriptor fields and spans match the preceding current case except
name and replacement `*=`. Its complete typed descriptor canonical SHA256 is
`a2d94adbb592864a95c4e6e1bd7a999afbab8fd089b1fdc05a33ea471408ffac`.
The current full-domain argument above applies only to this exact fallback
replacement. The actual terminal native outcome is `MissedMutant`, with both
Build and Test phases `Success`. Its diff SHA256 is
`a7251da8475618b40d764c95fd23a0ec52e5730205b9a30f515337196f2383cf`;
log SHA256 is
`bdfc6a1e51de1b7a4ca2258493a5d809c11c906c02064d199d530ed956344673`.

### N3 raw outcomes and strict opt-in boundary

Current raw artifacts remain beneath
`~/.local/share/nevrotick/dependency-refresh-20261106-v1/n3-series-mutation-v1/work/mutation/library/mutants.out/`.
The exact `outcomes.json` SHA256 is
`0ca157d75a4d05e762fae493e97df121f63184d93f55cc7f8e46156aba5a1fc1`;
wrapper `verification.json` SHA256 is
`9ae512fe9e1ce17dd8e40af0ac3d5082d72c702e74278bc04da5f9b48c1c1201`.
Both exact missed descriptors match the current enumeration. Each selected
mutant log and the successful baseline contains 131 unique passing identities
(89 ordinary and42 doctests), matching successful summary counts with no
failed, ignored, measured or filtered credit, and ends `*** result: Success`.
The baseline log SHA256 is
`bca976d4c02975260f37c8028bd05e74e77549589e50307873b852f8fc2aa741`.
This bounded reconciliation covers these two mutants and baseline, not a full
audit of the other native logs or the causes of their timeouts.

The unchanged raw campaign is214 variants:96 caught,113 unviable,2 missed and
3 timed out. Native exit is3, the wrapper is incomplete and no score is
qualified. Neither E is a kill. These two reviews do not resolve or exclude any
timeout, compiler rejection, foreign mutant or historical descriptor.

Only the explicit new `series` profile may opt into this complete two-ID cohort;
empty E remains allowed. Its separate validator pins the exact current library
source and complete native descriptor hashes, with these exact maintained
review sections. Unchanged shared admission must additionally match the actual
current product candidate, source/proof bytes, complete enumeration and native
`MissedMutant` outcomes. Raw and adjusted accounting stay separate; native
errors, timeouts, surviving non-E mutants and the90%/95% gates remain enforced.
Final test-candidate rebinding and fresh actual campaign reconciliation remain
coordinator work. The historical engine validator, E IDs and scopes are not
broadened. No product/P1, native Windows, formal or release claim follows.

## Unavailable-admission accessor pair — 7 November 2026

This ordinary mutation-equivalence review covers two actual native misses from
`n3-series-qualification-v2`, candidate
`d7cc1ae8d6224166ab43d61b4ead291791c376dcb05c3bda82e6244f78881272`.
The142-variant outcomes SHA256 is
`792fae02f4e7476eeaf6eda63ac7c647206f9f92a54c61edbb588a6002bb4319`.
Both exact variants build and pass their native tests; neither log contains an
incomplete marker. The whole batch remains incomplete for other missing controls
and unclassified observer failures. These two equivalences supply no kill credit.

The reviewed series source SHA256 is
`bcbd2fec35a0e498e7ef05e9470d188f8a1ef225b5d32c1bd2ae26843c7bd515`.
Independent reviewer `temporal_control_review` challenged the complete local
domain, construction paths and possible effects. `StatisticalAdmission` has only
`Unavailable`. The private `SeriesReport::admission` field is initialized to that
variant by its sole constructor, `SeriesReceiver::finish`; there is no decoding
path or mutation API. More strongly, the exhaustive getter match yields zero for
every valid value of the field, regardless of other report fields. The enum is
not `Copy`, but matching its field without a binding moves nothing; it has no
destructor. Neither getter allocates, performs I/O, mutates state, checks a clock,
fails or changes ownership. Returning literal zero therefore preserves every
admitted result, error/effect and lifecycle behavior, not merely today's callers.
Nonzero returns are non-equivalent and must remain discriminated by tests.

This is source reasoning for ordinary mutation accounting, with no formal tool,
solver or product-proof claim. Reconcile exact accessor bodies, enum, visibility,
constructors and complete candidate before admission on a later source. A new
admission variant, changed semantics or additional construction/mutation path
invalidates this review; a future nonzero sample count cannot inherit it.

## ENGINE-SERIES-UNAVAILABLE-ADMITTED-ZERO

The actual descriptor replaces
`SeriesReport::admitted_sample_count -> usize` with `0` at
`crates/nevrotick-engine/src/series.rs:464:9`, function span461:5–467:6,
replacement span464:9–466:10. Canonical descriptor SHA256:
`7da9d471e1fa869efa3b8ee17021b7a8c669c48e1c90a2b6c56db19acbe35545`.
The complete adjacent contract and getter, lines461–467 inclusive, hash to
`085a65547db12fec4d598648abda798f77c93df8fcedccfe06331223d1c3db65`.
Its full-domain argument is the unavailable-only construction and effect argument
above. Preserve the raw native `MissedMutant`; reviewed E is not a kill.

## ENGINE-SERIES-UNAVAILABLE-ELIGIBLE-ZERO

The actual descriptor replaces
`SeriesReport::eligible_sample_count -> usize` with `0` at
`crates/nevrotick-engine/src/series.rs:471:9`, function span468:5–474:6,
replacement span471:9–473:10. Canonical descriptor SHA256:
`a321a5a66afa4e62d840f1f040c6acbca0d152d84e89996a67c6ccf422e40861`.
The complete adjacent contract and getter, lines468–474 inclusive, hash to
`221ff1f7ed59ffa644153cd8ef614cbdc1a6c9478aabd8004d3e6033234dc66f`.
The same unavailable-only argument covers this independently declared getter.
Preserve the raw native `MissedMutant`; reviewed E is not a kill. Current opt-in
admission must pin both exact accessor IDs plus the exact library fallback pair,
their individual source/descriptor/review identities and the complete candidate.
Empty E remains permitted; partial/foreign cohorts, unclassified failures,
timeouts and unresolved survivors remain rejected by the existing gates.

### Writable-owner extraction reconciliation

After test-only compiler/Clippy corrections, the next407-input candidate is
`25654289cd4f7ac0931be077e4b443c975ce761f256a16a692d35afc1fc038e7`.
Series source is now
`84bb369585ca0f53a5d54f945295b5aeca41b6c2d4d91c9d3a745dab291e5d3f`.
Its sole application change extracts the writable-event arm into one synchronous
owner; independent source review confirms identical guard/watch/deadline/send
ordering. Both unavailable-accessor bodies, exact descriptors, unavailable-only
enum, private construction and effect argument above remain unchanged. The
library source and complete commit-duration helper also remain byte-identical.
This permits rebinding the four source arguments to that candidate; fresh
enumeration and actual native misses are still mandatory before any E accounting.
No historical outcome or product proof transfers to the new candidate.

### Quiet-owner test-only reconciliation

The later407-input candidate is
`e71aab7bae772e0190db522cc5a2e4d15ed4751ffb04425a08420de0e2fe4654`.
Only the private legacy processing test gains a quiet predeadline owner control;
both critical production files, all four exact descriptors, constructors and
complete source arguments above remain byte-identical. Current ordinary tests,
both coverage owners and fresh library214 execution remain required. The143
series outcomes retain their actual `25654289...` candidate and original proof
bytes; only an explicit independent unaffected-state argument may join them to
the current candidate. This note creates no fresh series execution or kill credit
and does not accept the prior fixture-only library catch.

## Current N3 final-summary source re-admission — 8 November 2026

This dated addendum supplies current source review for the same four N3 IDs;
all earlier candidates, native outcomes and source arguments above retain their
historical scopes. It admits no historical outcome as a fresh execution and
supplies no kill or score. This is ordinary mutation accounting, with no formal
compiler/solver, Windows-native or library mutation execution by these reviewers.

The current421-input product candidate is
`e8f6ead34f41429033a8d6c0ebd823bc693dae8caa645d53a8eafedf20e8727d`.
Its final ordinary receipt is
`~/.local/share/nevrotick/series-final-checkpoint-20261108-v1/green-ordinary-v3/verification.json`,
SHA256 `887732785dca641646e1c72adf1dcaf6af9054cd3566b33487d9e4ee1fee58b3`:
113 ordinary,113 optimized and50 doctests, with unchanged source/helper/recipe/
vendor/tool bookends. Those passes do not establish equivalence themselves.

The actual current enumeration is
`~/.local/share/nevrotick/series-final-checkpoint-20261108-v1/enumerate-v1/verification.json`,
SHA256 `a819df52c17245b7f9597c705e7865f6f0bd032bd1316fa3ecdbcbe1914eabc8`.
It contains214 library descriptors, canonical inventory SHA256
`0f1ca0520c68e80b752245af0271bea934f14db9bffd4159a171bf8c09dabc21`,
and145 series descriptors, canonical inventory SHA256
`e5cf359d7cb73c0298e9f007c6e2a6b2921674bebdfc2ced18fdadf1485e6b4c`.
All359 are actual native listings, below the unchanged512 cap. The complete typed
descriptor identities below exclude only the optional rendered `diff`.

### Current fallback domain and exact two operators

Independent reviewer `/root/series_checkpoint_design_review` checked the current
library source, actual native descriptors and pinned standard-library sources.
The library file SHA256 is
`f14ab14174eab86abdd14b8b70a567e88deeda4062520ed13323e7c4a8d11622`.
`commit_reply` has native function span1330:1–1379:2 including its adjacent
contract; those complete lines hash to
`307ec2ede4f337700543029ce2664e57f5958338286f1130a8b1d1c600653290`.
Its function definition, lines1333–1379 without the terminal newline, has SHA256
`39b380c49078491dbb2fb8337b6c10474733cf831be02ebd1c5b12269333bc04`
and is unchanged from the `89f035e` helper. Q1 exports changed whole-file bytes
and moved descriptor coordinates; they did not establish new library execution.
The six primary-source hashes in the earlier pinned-source table were checked
and remain identical for the admitted Linux Rust1.99.1 configuration.

The full valid helper-domain argument remains necessary: mismatched identities,
endpoints, profile or phase return before the time checks. The local guard rejects
every `processing < candidate.read.mono_after` using the unchanged, reachable
increment at1346. For the remaining immutable operands, the pinned Linux
comparison and subtraction return `Some(Duration)`, including equal instants,
signed-second extrema and nanosecond borrowing. No caller chronology or normal
counter-range assumption is added. The fallback at1350 is therefore unreachable
over the complete valid helper domain. These two operators do not target the
reachable1346 increment.

| Existing N3 ID | Actual current operator/name | Complete descriptor SHA256 |
| --- | --- | --- |
| `ENGINE-SERIES-COMMIT-DURATION-FALLBACK-SUB` | `crates/nevrotick-engine/src/lib.rs:1350:23: replace += with -= in commit_reply` | `2ab92195ec79b1c71bae647479ea1f5a82b5e6e0a0bfde2cbc3da0bd9067af1d` |
| `ENGINE-SERIES-COMMIT-DURATION-FALLBACK-MUL` | `crates/nevrotick-engine/src/lib.rs:1350:23: replace += with *= in commit_reply` | `5d7961f5476b58579bee819ba1a895a6da90fc0cfa4a10ddcb389bf82ce982f9` |

Both are `BinaryOperator` descriptors with operator span1350:23–1350:25,
function `commit_reply`, implicit-unit return type and the function span above.
Every reachable return, counter, retained cause, history, terminal/measurement
result, allocation, error/panic, borrow/drop and resource custody remains
unchanged. Changed guards, helper operands/body, platform, toolchain or
standard-library semantics invalidate this source re-admission.

### Current unavailable-only getter domain

Independent reviewer `/root/series_checkpoint_tests` checked the current series
source, all report constructors/cleanup/execution paths and the actual145-entry
native map. Series source SHA256 is
`174ab3b4538356c5134cb850baac19640baf27c1e14ad3c12f953948549a43f7`.
`StatisticalAdmission` still has the sole `Unavailable` variant. The private
field remains unavailable through every construction and lifecycle path; more
strongly, the exhaustive getters return zero for every valid report regardless
of its other fields. Literal zero changes no effect, failure or ownership.
Neither changed final-summary timing nor a returned control result supplies
sample admission. Nonzero replacements remain non-equivalent.

| Existing N3 ID | Actual current replacement | Function span | Replacement span | Complete descriptor SHA256 |
| --- | --- | --- | --- | --- |
| `ENGINE-SERIES-UNAVAILABLE-ADMITTED-ZERO` | `crates/nevrotick-engine/src/series.rs:469:9: replace SeriesReport::admitted_sample_count -> usize with 0` |466:5–472:6|469:9–471:10| `f4b3fae4f21b063861301bae063b9115737894d7916b6583f896cf63ca5ec531` |
| `ENGINE-SERIES-UNAVAILABLE-ELIGIBLE-ZERO` | `crates/nevrotick-engine/src/series.rs:476:9: replace SeriesReport::eligible_sample_count -> usize with 0` |473:5–479:6|476:9–478:10| `cc59ea98db653c91db4af372b1f9a3db26c081098e00eb62588d1fb33f097298` |

Both have package `nevrotick-engine`, genre `FnValue`, return type `-> usize`
and replacement `0`. A new enum variant, constructor/mutation path or changed
getter/domain/effects requires renewed review and source/descriptor binding.

### Current admission and execution limit

The series profile retains strict empty-or-complete-four opt-in. Each current
entry binds this whole candidate, its exact whole-file source, the complete
actual descriptor and the updated dossier bytes. No partial cohort, historical
descriptor, reachable increment, timeout or unclassified failure is accepted.
An E exclusion still requires the exact actual native `MissedMutant` under the
existing accounting gate and receives no kill credit. This source-only review
does not predict any native result or mutation score. The selected continuation
executes affected `series` mutation only; it supplies no current library mutation
result or pass. Coverage and all other qualification obligations remain separate.

## ENGINE-QUALITY-INTERVAL-ZERO-SIGN — 8 November 2026

This Q1-specific source adjudication is separate from every N3 exclusion above.
Independent reviewer `/root/series_checkpoint_design_review` inspected the actual
native descriptor/diff, the complete helper domain, its current callers and the
rational-result constructor/publication/getter chain. The review performed no
build, mutation execution or formal compiler/solver work and supplies no kill
credit. It does not close Q1's incomplete campaign or admit an analysis policy.

### Exact source and historical native observation

The descriptor is
`crates/nevrotick-engine/src/quality.rs:524:29: replace < with <= in interval_seconds`.
Its package is `nevrotick-engine`, file
`crates/nevrotick-engine/src/quality.rs`, genre `BinaryOperator`, replacement
`<=`, and operator span 524:29–524:30. Function `interval_seconds` has native
return type `-> Result<RoundedRationalSeconds, QualityRefusal>` and
documentation-inclusive span 514:1–531:2. Its complete typed descriptor,
excluding only rendered `diff`, has canonical SHA256
`ab870f10eb8cde026ababd980eb91338a2026e753395d4cd178bd50b8b6393ec`.
Both the actual 167-entry Q1 enumeration and historical native outcome contain
this exact descriptor. The replacement changes only the sign chosen for a zero
signed numerator; it does not change the denominator check or magnitude.

| Current source owner | SHA256 |
| --- | --- |
| Complete `crates/nevrotick-engine/src/quality.rs` | `f3abb4b00c769b3a4fb655cb923df7626c4209cb792594a55f065449051e2135` |
| Helper definition, lines 517–531 without its terminal newline | `13743a3e5f2145727748b0e79cb531aee3befd1963d90be164f3a1aa211b4183` |
| Complete descriptor function span, lines 514–531 including documentation and newlines | `82352ecaaf7a8d2f1a08c94ce1acd0513e9aad683c58e24f91b1fcf7ea499445` |
| Complete `crates/time-domain/src/rational_seconds.rs` | `2eeffceacf47251e7d917f90b59af12aec768e0a902465d32338ca75dae9a438` |
| `RoundedRationalSeconds::new`, lines 492–504 including newlines | `0f709d6501f7697c5e31aeee3350add36b0b6c7cf5a62fce15a20a31983341e5` |

These two complete production-source hashes match the current checkout and
the historical Q1 frozen source. The current 421-input product identity is
`e8f6ead34f41429033a8d6c0ebd823bc693dae8caa645d53a8eafedf20e8727d`;
this source review supplies no Q1 qualification for that candidate.
Historical native evidence remains under
`~/.local/share/nevrotick/dependency-refresh-20261106-v1/q1-quality-qualification-v1/work/mutation/quality/mutants.out/`.
This exact variant is `MissedMutant`, with Build and Test both `Success`.
Its raw diff SHA256 is
`30543ec1f8b2b782512731bd6148deb4be7e9fe8a80d97eac1a6030443584745`;
its raw log SHA256 is
`0114833b926b3f2e6790760e6e9cebb9d836d279798d74769812b965a1901ee0`.
The campaign belongs to the 416-input candidate
`8c05b4eedd9c3f231e4d809d9cc3ef028a8a9d1d21672ed6ebf045a525a96c29`.
Its terminal wrapper receipt is **incomplete**, SHA256
`6eb9ed7bcd335fe850219465b611bd28b21a1858fa8c582e1b2161aaa7215a93`:
the caught-cause gate refused missing selected Q1 assertion evidence. Neither
this source argument nor a successful survivor log turns that receipt into a
qualified campaign. Changed current tests and the larger integrated candidate
must retain their separate admission/execution identities.

### Complete typed domain, counterexamples and effects

The helper's own boundary admits every `i128` numerator, every `u64`
denominator and every valid `QualityInterval` variant. No assumption that a
denominator is already positive, a numerator is nonzero, or values came from
today's `exact_intervals` caller narrows this argument. The following cases
cover the complete domain:

1. If the denominator is zero, both versions return the same
   `QualityRefusal::ExactArithmetic(interval)` at `NonZeroU64::new` before
   evaluating the mutated comparison. This includes zero and both signed
   numerator extrema.
2. For a positive denominator and a nonzero numerator, `<0` and `<=0` select
   the same sign. Both pass the same `numerator.unsigned_abs()` and denominator
   to the same constructor. This includes `i128::MIN`, whose unsigned magnitude
   is 2^127 without signed negation/overflow, and `i128::MAX`, whose magnitude
   is 2^127−1. Every later success or failure path therefore agrees.
3. For a positive denominator and numerator zero, the original requests
   `RationalSign::Positive` and the mutant requests `Negative`, both with
   magnitude 0. `RoundedRationalSeconds::new` immediately replaces either sign
   with `Positive` when magnitude is zero, **before** `round_magnitude`, shared
   output validation or `publish`. The zero rounding branch supplies positive
   floating zero, zero error bound, exponent 0 and exact=true. Publication then
   receives identical inputs and checks the same canonical-zero association.
   Both results retain sign `Positive`, magnitude 0, the original positive
   denominator, `seconds().to_bits()==0` and
   `absolute_roundoff_seconds().to_bits()==0`.

The discriminating boundary is numerator 0. Adjacent −1/+1, both signed extrema,
and denominators 0/1/`u64::MAX` do not provide a counterexample for the reasons
above. This is an argument over the exact typed operations, not an inference
from absence of a failing test or from real-number algebra applied to floating
arithmetic. The existing raw-helper test includes zero and both extrema; it
does not itself establish equivalence or a new execution result.

`exact_intervals` derives local/server/raw-RTT tuples from the same borrowed
successful `TransportFacts` measurement and calls this helper three times.
`compute_quality`, `evaluate_conditional_quality` and the three public interval
getters consume/retain the same canonical results. They cannot observe the
discarded requested zero sign. All five rational-result fields, typed refusals,
representation bounds, original facts/assumption association and acquisition
age therefore remain unchanged. This conclusion also covers direct helper
calls outside those current producers.

The changed comparison and sign enum have no callbacks, allocation, I/O, time
read or mutation effects. After zero canonicalization, both executions invoke
the identical conversion/validation/publication path with identical inputs;
any allocation or failure behavior belongs to that unchanged path. No new
allocation, cleanup, ownership, rounding or physical-uncertainty claim is made.
The advertised-distance getter replacement with 1.0 is a different, non-equivalent
survivor and is not covered by this exclusion.

### Invalidation and admission boundary

Changed helper inputs/body, denominator/refusal precedence, sign or magnitude
representation, constructor zero canonicalization/order, rational conversion,
publication, getters or observable effects require renewed review. In
particular, validating/publishing the requested sign before zero
canonicalization, retaining negative zero, or exposing the discarded sign would
invalidate this argument. Bind the exact source/converter/descriptor and the
whole selected candidate before any Q1 policy admission; do not import an N3
cohort or a historical score. An admitted E remains an exclusion, never a kill,
and current qualification must preserve the actual native outcome and complete
caught-cause/function/guard evidence under the existing 90%/95% gates.


## Current Session/coherence equivalences — 8 November 2026

This addendum re-adjudicates five actual misses on the current compiled candidate
`6d4cc1560b7974129d75462cdb7b46411f97784bf8442ac64b88896468a5acb4`.
It does not inherit an earlier outcome or disposition. Source-level equivalence
reasoning here is not a machine-checked product proof and discharges no OD-10
post-V1 proof obligation. Each exclusion receives zero kill credit; retain raw
counts, all descriptors, native exits and the original incomplete receipts.

Current source bindings:

- `crates/nevrotick-engine/src/series.rs`:
  `3444eeb5e950c77ac738bbb268ef43e2dd68b6984adac1454e2d8754b174da7c`.
- `crates/nevrotick-engine/src/measurement.rs`:
  `0b97439bbb6b6e83db74b046a7140b0030e3c97b7aa69b67423f60c454889ff7`.
- The shared constructor dependency, `crates/time-domain/src/rational_seconds.rs`:
  `2eeffceacf47251e7d917f90b59af12aec768e0a902465d32338ca75dae9a438`.

The two Series misses are actual `MissedMutant` rows in v2 outcomes
`d4a05096669ab5c5b1b7cd293ce84e94fa66d777919ba5824e15b2cd25d8b479`.
The three Measurement misses are actual rows in outcomes
`af7176966956174447b686862e370cbaefdb3906936e98fd58bd3ddaff77276d`.
Each built and executed the complete 207-identity baseline successfully; no
ignored, filtered, compiler-unviable, timed-out or ungenerated case is excluded.
The legitimate doctest line shifts caused by replacing function bodies remain
separately checked against each raw diff. Descriptor hashes below identify the
complete native mutant object WITHOUT its separately retained rendered diff.

Independent reviewers: `series_checkpoint_verification/enumerated_series_review`
for both Series cases; `series_checkpoint_design_review` for Measurement;
`series_checkpoint_verification` and the coordinator challenged all five against
current declarations, constructors, raw outcomes and dependent source. This is
an independently justified equivalence verdict only for the exact bindings.

## ENGINE-SESSION-SERIES-UNAVAILABLE-ADMITTED-ZERO

Actual descriptor: `series.rs:476:9`, replace
`SeriesReport::admitted_sample_count -> usize` with `0`.
Native descriptor SHA256:
`8b3ff21dbdb4481bd500c4a6afc55933af64e5f954bc7e41824f453aef84e8af`.

The full valid typed domain, not just a successful caller, has one possible
admission state. `StatisticalAdmission` is private and has exactly `Unavailable`;
`SeriesReport.admission` is private. The sole production report literal in
`SeriesReceiver::finish` sets `Unavailable`. Cleanup changes end/failure fields,
and the execution/session owners move or borrow that same report without
mutating admission. There is no public construction, mutation, Default or From
route for another state. The exhaustive borrowed const getter therefore returns
`usize` zero for every valid report, regardless of all other fields or caller
chronology. Literal zero has the same result, ownership, allocation, failure,
panic and effect behavior. Adding an admission variant or changing the field,
constructor, cleanup/session mutation or getter invalidates this argument.

## ENGINE-SESSION-SERIES-UNAVAILABLE-ELIGIBLE-ZERO

Actual descriptor: `series.rs:483:9`, replace
`SeriesReport::eligible_sample_count -> usize` with `0`.
Native descriptor SHA256:
`1bb74259a1ad856d2c4d85150a1fb75787ec932cc116910bcf3bf1c8ce90f2ef`.

The same complete private typed domain and sole report constructor established
above apply. This separate exhaustive borrowed const getter also maps its sole
`Unavailable` state to zero, without allocation, failure, panic or effects.
Replacing it by zero changes neither any valid return nor state/ownership.
The same enum, field, constructor, mutation and getter changes invalidate it.

## ENGINE-SESSION-COHERENCE-LOWER-EQUALITY

Actual descriptor: `measurement.rs:558:29`, replace `<` with `<=` in
`coherence_residual`. Native descriptor SHA256:
`bd833b6be02ec7ea0e551202bc0ba3982d6a587c5ae694d6d504030a95065e0d`.

The helper itself checks both own brackets and their cross order before any
subtraction. In every reached successful comparison path, its original raw
monotonic endpoints satisfy base <= base_after <= later <= later_after. Exact
checked duration/nanosecond conversions consequently give 0 <= lower <= upper.
All order, checked-duration, duration-to-i128 and wall-difference errors occur
before the changed comparison and are identical. The operators differ only when
wall_delta == lower. The original lower branch is false; the upper branch is
also false because lower <= upper, so the gap is literal zero. The replacement
computes checked lower - wall_delta = 0; subtracting equal representable i128
values cannot overflow. Both then perform the same successful conversion to
u64 zero. No state, allocation, I/O, callback or error changes. Any relevant
ordering, duration/conversion, subtraction or publication change invalidates it.

## ENGINE-SESSION-COHERENCE-UPPER-EQUALITY

Actual descriptor: `measurement.rs:562:26`, replace `>` with `>=` in
`coherence_residual`. Native descriptor SHA256:
`da36761fb7f15b660253819ecfd4d29a868bf422d9a54ce49f845331bcd33dcc`.

The helper-enforced full-domain ordering and unchanged earlier errors above
apply. The operators differ only when wall_delta == upper. The lower branch is
false because lower <= upper. The original returns literal zero; the replacement
computes checked wall_delta - upper = 0 and performs the same u64 conversion.
The return, errors, allocation and effects are identical. The same guard,
conversion, subtraction and publication changes invalidate this argument.

## ENGINE-SESSION-NOMINAL-ZERO-SIGN

Actual descriptor: `measurement.rs:799:29`, replace `<` with `<=` in
`nominal_seconds`. Native descriptor SHA256:
`85e4e17d40dbe6e7a5ed1e147a2caed88565d9e49f8255ce33249134ce321a3e`.

Consider the complete raw i128 numerator by u64 denominator domain. Nonzero
numerators choose the same sign in both versions, including i128::MIN, whose
unsigned_abs is representable as 2^127. Only numerator zero changes the temporary
Copy sign. Denominator zero returns the same Arithmetic refusal before calling
the converter. Otherwise magnitude is zero and `RoundedRationalSeconds::new`
canonicalizes its sign to Positive BEFORE calling the same round_magnitude and
publication/validation paths. All resulting fields, including the stored sign,
positive floating-zero bits, exact ratio and rounding bound, are identical.
Errors, allocation and effects are unchanged; the temporary Copy sign has no
Drop behavior and is neither logged nor exposed. This depends on the exact
shared constructor above, not a wrapper assumption or floating equality of
+0.0 and -0.0. Changing canonicalization, call/publication order, conversion,
validation, sign effects or this helper invalidates the argument.
