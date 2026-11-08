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
