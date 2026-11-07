# N1b exact mutation dispositions

Ordinary source review for REQ-8/BND-1 and CHECK-7/8/25. This record supplies
source-bound mutation dispositions, not machine-checked formal product proofs.
Formal work remains post-V1. An equivalent receives no kill credit; complete
raw outcomes and interrupted attempts remain retained in the existing N1 task.

Integrated product:380 inputs, candidate SHA256
`7a7d1b7bc692ba2a5f07b3d8ad795d28d97c9cb0f5f1255d9077ff0bb0457274`.
The full typed descriptors are separately bound in
`policy/conditioned-window-analysis.json`. Relevant source hashes:

- `crates/time-core/src/conditioned_window.rs`:
  `77f53d02b7d4b321faea02dbddc94eb15ce0d4121bf1e43f51960cd80246cbb2`.
- `crates/time-core/src/window_variance.rs`:
  `caf7de1f54f431f530c76b95ce22dafe5c62655b1882250e572469c038779296`.
- Required zero-canonicalization callee,
  `crates/time-domain/src/rational_seconds.rs`:
  `2eeffceacf47251e7d917f90b59af12aec768e0a902465d32338ca75dae9a438`.

The earlier discovery candidate05adb is historical. The unchanged owner bytes
and descriptor digests were re-enumerated in the actual eight-worker campaign
on7a7d. Each disposition below concerns the exact observed successful-build,
successful-test survivor, never a name pattern or an unexecuted exclusion.

## N1-CEILING-EQUALITY

Exact mutant: `crates/time-core/src/conditioned_window.rs:853:18: replace < with <= in ceiling_exponent`.
Independent reviewer: `/root/n1_ceil_equivalence`; coordinator `/root`.
Verdict: `independently-justified-equivalent`.

The full helper domain is every `u128` magnitude and every `NonZeroU64`
denominator. Zero magnitude returns `Ok(0)` before the changed comparison.
Otherwise the selected quotient is in1..=u128::MAX, its integer logarithm is
in0..=127 and its checked conversion to `i32` succeeds. Unequal operands take
the same branches. Equality is the only changed case: quotient1 gives floor0;
the original computes checked addition of0 because divisibility and power-of-two
tests both hold, while the mutant negates integer zero. Both return identical
`Ok(0)`. The positive-branch checked sum is at most128, so neither variant adds
an error. Endpoint magnitudes and denominator endpoints are included. Skipped
operations are pure scalar built-ins, with no allocation, I/O, state, resource
or lifecycle effects; the bounded-work contract is unchanged.

## N1-EXPONENT-FIELD-ASSEMBLY

Exact mutant: `crates/time-core/src/conditioned_window.rs:979:44: replace | with ^ in exact_scale`.
Independent reviewer: `/root/n1_scale_equivalence`; coordinator `/root`.
Verdict: `independently-justified-equivalent`.

The full raw helper domain is `f64 × i32`, including rejected values. At the
changed expression, A=`value.to_bits() & !0x7ff0000000000000` has zeros in bits
52–62. Checked output exponent1..=2046, converted to `u64` and shifted by52,
produces B with bits only in52–62. Therefore A&B=0 and A|B=A^B. Sign bit63 and
fraction bits0–51 cannot overlap B. The fixed shift cannot overflow its type.

NaNs, infinities, signed subnormals and negative zero reject before the changed
expression. Every raw exponent takes the same checked-add/range paths; positive
zero bypasses the expression. Either sign of normal value has the same checked
output range and disjoint fields. The multiplication precedes the mutation and
is unchanged, so expected bits, final comparison, scalar/error bits and all
effects remain identical, including rejected underflow/overflow paths. No
assumption about the narrower current N1 caller scales is needed.

## N1-NORMAL-BOUND-ZERO-COMPARISON

Exact mutant: `crates/time-core/src/conditioned_window.rs:1018:40: replace > with >= in valid_bound`.
Independent reviewer: `/root/n1_ceil_equivalence`; coordinator `/root`.
Verdict: `independently-justified-equivalent`.

Every raw binary64 representation is covered. Positive zero passes the first
bit-zero branch without a comparison. Negative zero, subnormals, infinities and
every NaN representation fail `is_normal()` before either comparison. Normal
values are nonzero: positive normals satisfy both comparisons, negative normals
neither. The trailing mantissa-mask check consequently has the same reachability
and result. Return values, evaluation and absence of errors/effects/resources
are identical over the entire predicate domain.

## N1-ZERO-REFERENCE-SIGN

Exact mutant: `crates/time-core/src/conditioned_window.rs:1133:39: replace < with <= in prepare_point`.
Independent reviewer: `/root/n1_scale_equivalence`; coordinator `/root`.
Verdict: `independently-justified-equivalent`.

For every pair of checked rows, unequal exact half-tick references select the
same sign. Equality changes only the requested sign, with absolute difference
exactly zero. The actual `RoundedRationalSeconds::new` callee canonicalizes
either requested sign to Positive before rounding, publication or validation.
Its zero result has identical complete retained tuple, denominator, positive-zero
seconds and bound, exponent and exactness. This also covers distinct rows with
equal references and unrelated phases, contexts, keys or distances; equality is
not assumed to imply object identity or current population admission.

Phase conversion, checked negation, scaling and point publication receive the
same arguments afterward. Arbitrary raw scale pairs, including `i32::MIN`, and
arbitrary divisors therefore preserve rejection order and errors. The same
original-row borrow is retained, and validation/rejection allocation follows the
same paths. No changed effect or lifetime is introduced.

## N1-WINDOW-VARIANCE-THRESHOLD-EQUALITY

Exact mutant: `crates/time-core/src/window_variance.rs:373:26: replace > with >= in WindowVarianceDivisors<'a>::derive`.
Fresh independent reviewer: `/root/n1_window_equivalence`; coordinator `/root`.
Verdict: `independently-justified-equivalent`; no historical adoption inherited.

The full raw constructor domain includes every slice length, every binary64
precision/distance, duplicates/unordered inputs, positive subnormals and precision
above the minimum. All count/scalar rejections and prior arithmetic failures
remain identical. Reaching the comparison establishes finite distance/threshold
and explicitly checked positive finite spread. Only equality changes branches.
The original keeps initialized exact positive1. The mutant subtracts equal
finite operands, divides zero by positive finite spread, adds zero to1 and
squares1: all finite-result guards succeed and the final bits are exact positive1.
Either internal zero sign yields the same result. Equality uses the already
rounded threshold, so threshold absorption does not undermine this argument.

Other rows, all non-equality paths and the divisor guard are unchanged. The
extra fixed operations allocate nothing, publish no partial result, preserve
borrowed inputs, and perform no logging, I/O, clock/history or other state action.
Both satisfy bounded work/lifetime under the selected binary64 semantics;
instruction timing and external floating-status-register inspection are outside
this API contract. Fresh campaign diff SHA256 is
`4f3d15890b9e171becf612c75c3e94447b8d27b9cf37b55d0227b231a20fa49d`.

All dispositions require reconciliation after relevant source/domain, mask/type,
zero/subnormal/rounding semantics, callee behavior, guard/body, observable effect
or descriptor changes. Raw misses stay visible. These arguments qualify neither
physical accuracy, real provenance, fitting/history, Windows behavior nor release.
