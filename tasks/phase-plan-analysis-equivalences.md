# Finite phase-plan generated-mutation equivalences

The [existing S1.4 task](s1.3-source-selection-readiness.md#s14-finite-phase-plan-lifecycle-contract)
owns execution and qualification. This bounded artifact supplies exact reviewed
source arguments to the existing mutation reader, whose proof-artifact admission
requires a document no larger than 64 KiB. It is not a formal proof or another
development plan. Every admitted exclusion stays a raw miss and receives no kill
credit. Complete current inventories, native outcomes and per-file gates remain
required.

## PHASE-PROFILE-NORMAL-POSITIVE

**Reviewed 8 November 2026:** `/root/series_checkpoint_design_review` and `/root`.
REQ-8/10/11/14/15; CHECK-7/8/10/11/14/25; AC-19.

Exact source:
`crates/time-core/src/reference_input/rate_delivery/phase_plan.rs`, SHA256
`7a332363cc72c569a0a99fa5adbba4b5695bbb1367ea60d08449ed122d5bee13`.
Exact mutant: line 176, column 42, replace `>` with `>=` in `profile_valid`.
Its normalized seven-field descriptor SHA256 is
`131b5b15383236d6dcc596b9750cd491c2ea4933decf5a4f2db38b4c819b5f9e`.
The policy separately binds its complete native descriptor and product candidate.

The expression is `v.is_normal() && *v > 0.0`. The changed comparison executes
only for normal binary64 values. Every such value is finite and nonzero:
positive normals satisfy both comparisons and negative normals satisfy neither.
Both signed zeros, all subnormals, both infinities and every NaN encoding fail
the unchanged left operand and skip the comparison. This partitions the entire
raw `f64` domain; it does not assume a previously admitted profile.

The earlier horizon predicates, array iteration order, `all` short-circuiting
and scalar reads are unchanged. The predicate has no effects. Thus every caller
receives the same Boolean, including the actual constructor's shared predicate
and mandatory Garde path, with identical rejection/state behavior. Equality at
zero cannot distinguish the mutation because normality has already excluded it.

Independent review matched the exact source span, descriptor and rendered diff
against the full 277-entry lifecycle inventory. The interrupted ten-worker
campaign observed the mutant's complete 537-test success log. That observation
does not complete the campaign or establish any other mutation result. Review
receipt `profile-normal-positive-equivalence-review-v1.json` in the external
`phase-plan-20261108-v1` evidence directory has SHA256
`9bb42379280a5c540dc2b849e62cf6203c5c862da5578e05648a6e3e27ecbd97`.

This argument applies only to this exact descriptor on the bound source. It
authorizes no other positivity change, omitted validation or ungenerated fault.

The earlier observed source hash was
`8d3936d3ef79ac302b4e87a2b3a2df91e126aa79488bcbf258fcc18052adaf89`.
The current file preserves all 66,440 original bytes and appends only a private
`cfg(test)` child declaration. The predicate and every descriptor span are
unchanged; the current campaign must still enumerate and bind them exactly.

## PHASE-NUMERIC-ADDITION-ABS-SIGN

**Reviewed 8 November 2026:** `/root/inert_nts_formatter_plan_review` and `/root`.
The same S1.4 requirements and qualification limits above apply.

Exact source `crates/time-core/src/regression/chart/roundoff/phase_plan.rs`,
SHA256 `2d2e5ecf16c6aed1131f63420bae97f7037074b4511f2f9744be9fcdc31cddc9`.
Exact mutant: line 431, column 13, delete unary `-` in `PhaseDebt::settled`.
Normalized descriptor SHA256:
`0e3501f19639eca712ca0a6bbe423178d91415996edc3963b7f7f0bcadbb9a3a`.

This changes only the right operand passed to the error-bound helper, not the
stored subtraction point. `addition_error` forwards that operand unchanged to
`coefficient_addition_error` in
`crates/time-core/src/regression/chart/roundoff.rs`, SHA256
`c8a05da2b622468426063a3addb17e2c27d8ba9856efee9c53ff7f6d7201166f`.
That helper's first use is `right.abs()`; no other use observes its sign.
For every admitted finite total, `abs(-total)` and `abs(total)` have identical
bits, including both signed-zero cases. Earlier scalar, duration and point
checks are unchanged. All inherited errors, outward operations, failures and
their order therefore receive identical operands and produce identical results.
The separately rounded debt recurrence is untouched. The reader also pins the
parent helper hash; a later helper change cannot inherit this exclusion.

## PHASE-NUMERIC-POSITIVE-CLAMP-EQUALITY

**Reviewed 8 November 2026:** `/root/inert_nts_formatter_plan_review` and `/root`.
Exact numerical source SHA256 is the one above. Exact mutant: line 493,
column 37, replace `>` with `>=` in `PhaseDebt::proposal`.
Normalized descriptor SHA256:
`157595293ff60b8a3a3c036a78382a8b4a2102405edadcb6f4eccb36161aa229`.

The admitted transient cap is strictly positive, finite and normal; the earlier
checked division has already admitted its quotient. The branches differ only
when quotient equals that cap. At equality, both the cap branch and the original
fallthrough return the same nonzero normal binary64 bits. The negative-cap
comparison cannot select a different value for a positive quotient. All other
quotients take the same value-producing branch. Subsequent aggregate arithmetic,
bounds and mandatory output admission receive identical values. Both paths have
constant bounded work and no state or external effects.

## PHASE-NUMERIC-NEGATIVE-CLAMP-EQUALITY

**Reviewed 8 November 2026:** `/root/inert_nts_formatter_plan_review` and `/root`.
Exact numerical source SHA256 is the one above. Exact mutant: line 495,
column 28, replace `<` with `<=` in `PhaseDebt::proposal`.
Normalized descriptor SHA256:
`e74479b761cec558fdce8ce470058f36d48ad71afa8ba27d75d1fca6f3a10d1f`.

The same admitted quotient and positive normal cap constraints apply. The
branches differ only when quotient equals the exact negation of the cap. That
value is nonzero and normal, so equality implies identical stored bits; signed
zero and NaN cannot distinguish the two returned values. The positive-cap guard
is false for this negative quotient. Every subsequent value, bound, failure and
effect is consequently unchanged.

The three numerical arguments are recorded in external
`numeric-equivalence-review-v1.json`, SHA256
`7e64f14ce4557ebe576ae806be1686c16eb6c2bb71a7379829abddcf03077a8f`.
Its reviewer parsed each actual miss log and the native baseline: all contain
537 successful identities on the captured 0991c85f candidate. This is observation
of the original misses, not kill credit, a formal proof or transfer of the
incomplete campaign's status. The policy pins every complete descriptor and
current product; no other numerical mutation is admitted by these arguments.

## FREQUENCY-PROFILE-NORMAL-POSITIVE

**Current-source review, 8 November 2026:** `/root/series_checkpoint_design_review`,
`/root/series_checkpoint_verification` and `/root`. Requirements remain
REQ-8/10/11/14/15 and the pure CHECK-7/8/10/14/25 scope in the
[frequency-prior task](s1.3-source-selection-readiness.md#frequency-diagnosticprovenance-qualification-checkpoint--8-november-2026).
This is an ordinary source-equivalence argument, not a formal-verifier result.

The current 436-input product candidate is
`3c8ae8f191cba09742259957d491bb362f8301a39c7506c1a029e02be4ae7e90`.
Exact current source is
`crates/time-core/src/reference_input/rate_delivery/phase_plan.rs`, SHA256
`a9770f085170b8c9ae52a5957761f99173ee3ff02294df56cb1e8e0c8b888b48`.
Only the line178, column42 replacement of `>` with `>=` in `profile_valid`
is admitted. Its normalized complete seven-field descriptor SHA256 is
`87ebc63219626344ce56ac51b015c23fb2d7e2cf8ae69695757b6ce33ba11d04`;
the rendered diff SHA256 is
`dfeaa49099fa3e0942a626a76fb9dfef5b83940f0ff3eb25644576c9e8a6da49`.

The current predicate retains `v.is_normal() && *v > 0.0` for each of its three
caps. Both zeros, every subnormal, infinities and all NaN encodings fail
`is_normal()` and short-circuit before either comparison. Every other binary64
value is a finite nonzero normal: positive values satisfy both comparisons and
negative values satisfy neither. Thus the replacement preserves the Boolean for
the entire raw `f64` domain, including invalid/private inputs. The unchanged
horizon predicates, iteration order and short-circuit behavior preserve the
complete predicate result.

Both mandatory Garde admission and the constructor's repeated explicit check
consume that same result. Their returned values, rejection reports, allocation
paths, state and effects remain identical. The enabled frequency constructor
adds a caller without changing this predicate or the argument. This is a fresh
review against the current source, expanded callers and actual 295-descriptor
phase inventory; the historical176:42 source binding is not inherited.

For this phase owner, the policy may name only this exact case. All 295 variants
must still
execute. An exclusion requires the actual matching `MissedMutant` result with
successful build and tests; a caught, unviable, timed-out, absent or changed case
cannot receive this exclusion. A matching result remains a raw miss and receives
no behavioral kill credit. No current miss or completed campaign is asserted by
this source preparation. The three older numerical equivalences above are
outside the current four-owner inventory and supply no exclusions for it.

## FREQUENCY-GROWTH-NORMAL-POSITIVE

**Current-source review, 8 November 2026:** `/root/inert_nts_formatter_plan_review`,
`/root/series_checkpoint_verification` and `/root`. The requirements and current
436-input candidate above apply. This is an ordinary source argument, not a
formal-verifier result or a mutation kill.

Exact source `crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs`,
SHA256 `a0a6af42cba3f012545d05ad5949f8934c28bd4bd65e503f695cc7fdf150c23a`.
The original 42,083-byte production prefix remains unchanged; only the private
test-module declaration follows it. Exact variant: line180, column19, `>` to
`>=` in `profile_valid`. Complete normalized descriptor SHA256:
`b5a0a6e706ec4a2cbe7f87283276804d97af40ccc03e1b1c1f8b4dbb1fe95355`.
Rendered diff SHA256:
`8252d2903b1bef3a9f56801a7b871da96aef90721a6adca8187e90657915eae0`.

The unchanged immediately preceding `growth.is_normal()` excludes both zeros,
all subnormals, infinities and every NaN encoding. Reached positive normals
satisfy both comparisons; reached negative normals satisfy neither. Earlier
skew/clock-error predicates and subsequent age predicates retain their original
order and short-circuit behavior. This covers the entire raw profile tuple,
including invalid values, rather than assuming a previously admitted profile.
Both mandatory Garde and the repeated explicit predicate receive the same
Boolean, preserving returned parameters, rejection/allocation paths and effects.
The unmodified original-source variant actually survived the earlier state
campaign; that incomplete result supplies no current-source passing score.

## FREQUENCY-ANCHOR-BOUND-ABS-SIGN

**Current-source review, 8 November 2026:** `/root/inert_nts_formatter_plan_review`,
`/root/series_checkpoint_verification` and `/root`. The same requirements,
candidate and score restrictions apply.

Exact numerical source
`crates/time-core/src/regression/chart/roundoff/frequency_prior.rs`, SHA256
`c29c5461fc90820de0aef29503174a64b70dc18aca131b8592b247e123ad247b`.
Exact variant: line625, column64, delete unary `-` in `FrequencyAnchor::rebase`.
Normalized descriptor SHA256:
`22517d32dfe7600ed511c0e3e94937445c1c7079b9cbc6bc277e494927283f1c`.
Rendered diff SHA256:
`3acdd5b45d9fc361e6e7249efbdff3aeb98f2db2376c9de38e34bb9d7ca598fb`.
The required parent is `crates/time-core/src/regression/chart/roundoff.rs`, SHA256
`4bea2d3fbd596ec5c28b7febd56d21efe42b19c7de2fdbd84f804d5451ff476f`.
Its pin is mandatory; the older phase proof's parent hash cannot be inherited.

Only the right operand of `coefficient_addition_error` changes. The separately
rounded point subtraction, raw effect admission and zero-effect branch do not.
The parent's first use of this operand is `right.abs()`, and no operation observes
its sign beforehand. For every finite admitted effect, including both signed
zeros and extreme magnitudes, `abs(-effect.phase)` and `abs(effect.phase)` have
identical bits. All subsequent bound operands, operation order, overflow/refusal
causes, output admission, anchor bits and caller-owned effects are identical.
Invalid raw effects fail through the unchanged earlier admission path. This
covers the complete method domain and its errors, not only current examples.

## FREQUENCY-INTERVAL-WHOLE-ZERO-PRODUCT

**Current-source review, 8 November 2026:** `/root/series_checkpoint_design_review`,
`/root/series_checkpoint_verification` and `/root`. The same requirements,
candidate and score restrictions apply.

Exact numerical source SHA256 is the one above. Exact variant: line1196,
column27, outer `||` to `&&` in `Interval::multiply`.
Normalized descriptor SHA256:
`31b9588c3950a1a8c2069d4bc14ebf95ed991885d9ffaf8b2bf9658716181872`.
Rendered diff SHA256:
`b7fae67cb70757cc5896a3737def9bf39cd9071334535bfb5d95c9781cba7199`.

The admitted helper domain contains finite ordered closed intervals, including
signed-zero endpoints and bound-only subnormals. Partition it into three cases.
With both whole intervals zero, both versions return the same canonical `ZERO`.
With neither whole interval zero, both execute the identical original loop.
With exactly one whole interval zero, only the mutated version enters the loop;
each of its four endpoint pairs has a zero operand. The unchanged inner `||`
therefore selects the literal positive-zero pair every time, bypassing products
and outward rounding. Starting from finite extrema, min/max reduction yields
the same canonical positive-zero endpoints as `ZERO`. No error, allocation,
counter, source state or other effect differs. The additional four iterations
remain within the existing maximum-four-pair fixed-work contract; exact timing
equality is neither required nor claimed. The distinct inner-operator variant is
not equivalent and has its own zero-endpoint regression control.

These three cases and the current phase-normality case are the entire permitted
current set. Every exact variant still executes and must actually finish as
`MissedMutant` with its complete passing current roster before exclusion. Caught,
unviable, absent, changed or timed-out variants cannot receive this treatment.
All exclusions remain raw misses and receive zero kill credit. The earlier
incomplete campaigns and completed proofs retain only their original scopes.
