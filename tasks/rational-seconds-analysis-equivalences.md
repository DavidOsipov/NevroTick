# Rational-seconds mutation equivalence review

**6 November 2026.** Ordinary CHECK-25 mutation accounting for the
[S1.2 exact-rational-seconds child](s1.2-regression-readiness.md#exact-rational-seconds-normalization--6-november-2026),
under its REQ-8/BND-1 and CHECK-6/7/8/11/14/25/27 parent obligations. This is
manual source-bound reasoning, not an executed machine-checked product proof.
[OD-10's post-V1 deferral](../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026)
and all existing execution holds remain in force.

Reviewed source: [rational_seconds.rs](../crates/time-domain/src/rational_seconds.rs),
SHA256 `2eeffceacf47251e7d917f90b59af12aec768e0a902465d32338ca75dae9a438`.
The coordinator's complete374-input product map is
`b1852a81314a564c4923a7ed1be7d293da730ac2b7dd4152eca539f3a4474d35`.
Reviewer `/root/rational_equivalence_review` independently inspected the complete
helpers and their invariant owners, private raw-boundary controls and historical
native outcomes. The coordinator must match fresh complete descriptors and
actual native outcomes to this source/candidate and bind these exact review
bytes before adopting an E disposition. A line summary alone is insufficient.

None of these five proposed E dispositions receives kill credit. Fresh baseline,
coverage, mutation, unviable/timeout accounting and the per-file gate remain
independently required. No historical score transfers to this candidate.

## Historical outcome and non-equivalent controls

The first native campaign, on earlier source
`ad9a9b50d8f720dfdc3e2c1f7aa4e0d40279c79285609f35295de31135035d17`
and product map
`abfb2bd3549e2d98b13ce9008a6c57faf09a5fcc5b04577e678ff67b39a10c5e`,
retains104 variants:69 caught,28 unviable,7 missed and0 timed out. Its raw
`outcomes.json` SHA256 is
`78e6d62941d208ac073a22d1fe44281eb0c57d88c4f58702f57edd0e0d1fc832`.
It remains incomplete; this review does not revise its raw native outcome.
Receipts and raw diffs remain under the existing external
`s12e-composition-20261030T082648Z/rational-seconds-coverage-mutation-v1` root.

The two `roundoff_rule` conjunction changes are **not equivalent** on that
helper's full raw `f64` domain. Replacing its first `&&` with `||` admits the
normal fractional predecessor of the minimum bound, bits
`0x389fffffffffffff`; replacing its range conjunction with `||` admits the exact
out-of-range power `2^-118`, bits `0x3890000000000000`. Later publication
association checks cannot discharge this raw-rule contract. The direct private
regression now challenges these values, valid zero and endpoint powers, an
in-range fractional neighbor and the power above the upper endpoint. Retain
the actual isolated RED receipts with their earlier candidates; current native
kills must be observed separately.

The old `publish:432:59` OR/XOR variant also receives **no equivalence credit**.
For raw `seconds=-0.0`, requested Negative, magnitude1, denominator1, exact=true,
original_exponent0 and bound+0, old OR retained negative zero and caused a Garde
rejection/report allocation. XOR yielded positive zero, passed the scalar check
and reached a later rejection without that report. Equal final fixed errors did
not establish equal allocation paths. The current source instead enforces the
raw unsigned-magnitude sign before either expression; the current case below
depends on this actual local guard, not on today's `round_magnitude` caller.

## Shared normalization domain argument

The mutated boundary is `normalized_ratio(N,D)` itself: every `u128` magnitude
and every `NonZeroU64` denominator, including zero magnitude. No narrower public
caller precondition is assumed. Zero is rejected by the unchanged
`original_binade` call before any mutated expression. For `N>0`, exact
`r=N/D` obeys `2^-64 < r < 2^128`, hence its binade exponent `E` satisfies
`-64<=E<=127` and `2^E<=r<2^(E+1)`.

The actual `original_binade` establishes that binade from integer bit lengths
and an exact comparison. For bit widths `n` and `d`, `c=n-d` lies in
`[-63,127]`. If `c>=0`, aligning `D` by `c` gives exactly `n<=128` significant
bits; otherwise aligning `N` by `-c` gives `d<=64` bits. The alignment therefore
does not lose high bits. Comparing the aligned values selects `E=c` or `c-1`.
Conversions and checked operations occur identically before each mutation.

`normalized_ratio` sets `s=52-E`. If `s>=0`, its integer pair is
`X=N*2^s`, `Y=D`, with `X< D*2^53 <2^117`. If `s<0`, its pair is
`X=N`, `Y=D*2^(-s)`. In both cases `2^52<=X/Y<2^53`, so integer division gives
`2^52<=q<2^53` and `0<=remainder<Y`. Also `Y<2^76`: in the first case it is
at most `2^64-1`; in the second case `q>=2^52` implies
`Y<=X/2^52<2^76`. All scaling counts fit the checked operation, all scaled
integers fit `u128`, and `q` fits `u64`. This is a statement about the whole
raw typed input domain, not fabricated `NormalizedRatio` fields.

Thus the three guard predicates at the changed boundary are all false:
`A = q outside [2^52,2^53)`, `B = remainder>=Y`, `C = Y>=2^76`.
Their range/comparison evaluations are total scalar operations with no
allocation, I/O, state change or user callback. Any earlier fixed failure is
identical; the later returned fields are identical.

Independent endpoint/scaling challenges used exact Python rational comparisons
for3,048 nonzero `(N,D)` pairs around binary powers, full unsigned endpoints and
selected denominator endpoints. These are sampled arithmetic cross-checks of
the reasoning, not exhaustive execution, a Rust campaign or a formal proof.

## RATIONAL-NORMALIZED-LOWER-RANGE

Exact change at309:17: the lower range expression `1_u64 << 52` becomes
`1_u64 >> 52`, or zero. Historical full descriptor SHA256, excluding native
`diff`, is
`2f7b93720d46d4b50aebe6eaf5f7f14cf0da14b05fee892c28528d7d9817b4c5`.

The changed guard range is `[0,2^53)` instead of `[2^52,2^53)`. The actual
quotient is always in the smaller range by the shared whole-domain argument.
Both `contains` checks therefore return true, and the surrounding negation is
false. Zero input and all earlier failures occur before this expression in both
variants. No return, fixed error, allocation or other effect changes. Choosing
raw quotients below `2^52` cannot refute this result: this helper accepts raw
`N,D`, computes its own quotient and does not accept a caller-supplied quotient.

## RATIONAL-NORMALIZED-GUARD-FIRST

Exact change at310:9: the first `||` becomes `&&`. Historical complete descriptor
SHA256, excluding native `diff`, is
`421b228242d79b4dcf343b6ae1ba8efae1c71ae21a4a103280c2b0d718a9d797`.

Accounting for Rust operator precedence, `A || B || C` becomes
`(A && B) || C`. All three predicates are false over this helper's raw typed
domain by the shared argument. Both guards are false. The changed
short-circuit evaluation skips only total, effect-free integer comparisons;
it introduces no failure, allocation or observable state/lifecycle change.
Every returned field and earlier error is unchanged.

## RATIONAL-NORMALIZED-GUARD-SECOND

Exact change at311:9: the second `||` becomes `&&`. Historical complete descriptor
SHA256, excluding native `diff`, is
`adfe12ed6c1f50e1acd9df5080677fb2e93c82b4e6ad113e0f7836d263c1973a`.

The guard becomes `A || (B && C)`. The shared whole-domain argument establishes
`A=B=C=false`, so rejection and all output fields are unchanged. The omitted
comparison is total and effect-free. Error propagation before this guard is
identical, including zero-magnitude rejection. This argument does not assert
that arbitrary externally supplied significands or remainders are valid inputs
to another helper.

## RATIONAL-SIGNIFICAND-DISJOINT-BITS

Exact change at389:75 in `round_magnitude`: the `|` joining the exponent word and
fraction becomes `^`. Historical complete descriptor SHA256, excluding native
`diff`, is
`caf805553b3b75b2578cdf07ee82b5b4772b943e6f0b72713f53d938bb520464`.

This helper accepts all `u128` magnitudes and `NonZeroU64` denominators. Zero
returns the same exact positive-zero record before the changed expression.
For nonzero input, its actual normalized constructor gives
`2^52<=q<2^53` and a proper remainder. Incrementing by at most one yields
`q'<=2^53`. The explicit carry replaces `q'=2^53` with significand `2^52` and
increments only the output exponent. Otherwise the significand remains in
`[2^52,2^53)`. Subtracting `2^52` therefore gives `0<=fraction<2^52`.

The actual `binary_power` returns an exponent-only normal word: its low52
fraction bits and sign bit are zero. Its checked exponent/bias construction and
any failure happen identically before either operator. Thus its word has no set
bit in common with `fraction`, making OR and XOR exactly the same `u64` word.
`from_bits`, exactness, original exponent, arithmetic bound and every failure
remain identical. This is a bit-vector identity under enforced local arithmetic,
not an assumption that real-number algebra preserves floating rounding.

## RATIONAL-PUBLICATION-DISJOINT-SIGN

Current exact change at437:59 in `publish`: the `|` applying `sign_bit` becomes
`^`. The old432:59 descriptor is not this descriptor. Fresh enumeration must
bind the complete current function span, replacement and native identity.

The reviewed boundary includes every `RationalSign`, `u128`, `NonZeroU64`, and
every raw `MagnitudeRounding` field combination: arbitrary `f64` bit patterns
for seconds/bound, any `i32` exponent and either exactness flag. It does not
assume the private caller supplied a valid magnitude or associated metadata.
The unchanged local check at426 rejects every seconds word with sign bit1,
including negative zero, negative infinity and negative NaNs, before constructing
the candidate or calling Garde. Both variants take the same allocation-free
fixed-error path for all these inputs.

For every remaining word `W`, its bit63 is zero. The closed enum supplies either
`S=0` or `S=1<<63`, so `W&S=0` and `W|S=W^S` exactly. This includes positive
zero, subnormals, normals, positive infinity and every positive NaN payload;
subsequent admission may reject them, but it receives identical bits. Candidate
metadata and both Garde validations are identical. Therefore any bounded report
allocation and destruction, scalar or association error, checked expected-bound
failure, returned value and subsequent effect path are identical. No distinct
allocation trace is hidden behind an equal final error.

The direct raw-owner regression
`raw_unsigned_magnitude_is_admitted_before_sign_application` distinguishes
requested Negative with raw negative unit/negative zero from a legal raw positive
unit and canonical zero. Its isolated guard-omission RED belongs to its own
retained source receipt; it is not a kill of this OR/XOR variant.

## Review scope and invalidation

The source-bound verdict is **independently justified equivalent**, conditional
on fresh exact descriptor/source/candidate/outcome reconciliation, for precisely
the five IDs above. Any unmatched survivor remains unresolved. Preserve raw
generated/viable/caught/missed/unviable/timeout totals and show adjusted E
accounting separately; equivalence does not turn a missed variant into a kill.

Re-review changes to integer types, bit-width/binade selection, checked scaling,
division/remainder, normalization guards, carry/fraction construction,
`binary_power`, raw-sign admission order, enum variants, Garde rules or error and
allocation behavior. Different targets/features/generated validation require
their configured reconciliation. No result here establishes physical clock
accuracy, provenance, production eligibility, regression readiness, native
Windows behavior, full S1.2/P1 or release qualification.
