# Window variance mutation dispositions

Ordinary source review for S1.2/CHECK-7/8/25. This is not a machine-checked
product proof; formal work remains post-V1. No mutant receives kill credit from
this record. Raw outcomes and each failed campaign remain retained.

## WINDOW-VARIANCE-THRESHOLD-EQUALITY

Source: `crates/time-core/src/window_variance.rs`, SHA256
`aa88951916a471266f143c8c8dfff6acfb2d6378483335be888f6e90b624922f`.
Integrated product candidate: 370 inputs, canonical SHA256
`904a1bb068bb54ef585f39db0dca73a999bdf7b8d2caaf6679c99b6708eaac8d`.
The full typed descriptor is bound separately in
`policy/window-variance-analysis.json`: line373,column26, binary operator `>`
replaced with `>=` in `WindowVarianceDivisors<'a>::derive`, function span
308:5–388:6, expression span373:26–373:27.

The admitted domain is the function's complete enforced raw-input domain, not
merely inputs reached by a current caller. Count, precision, original distances
and all preceding arithmetic are checked identically. Rejections for zeros,
negative/nonfinite values and intermediate overflow are unchanged. Reaching the
comparison establishes finite strictly positive distance, computed threshold and
spread, including positive subnormals and precision greater than the minimum.

Only equality with the already rounded binary64 threshold changes branches.
The original retains initialized exact positive1. The mutant subtracts equal
finite operands to obtain zero, divides zero by positive finite spread to obtain
zero, adds zero to1 to obtain exact positive1, then squares it to obtain exact
positive1. Either internal zero sign gives the same final bits. Each finite check
and the divisor lower-bound check succeeds. This also covers threshold absorption
at large magnitudes; no assumption about an exact real-valued sum is needed.
All non-equality branches and subsequent rows are identical. The extra fixed
arithmetic/helper calls allocate nothing, perform no I/O/logging/clock/history
operation and mutate only the same unpublished local slot. Both paths preserve
borrowed input and the declared bounded-work contract. Timing and external
inspection of floating-point status registers are outside this API's contract.

Independent reviewer: `/root/quality_weight_contract_design`; coordinator
reconciles the unchanged source and exact integrated candidate. Verdict:
`independently-justified-equivalent`. Review must be repeated if admission,
zero/subnormal treatment, helper semantics, branch bodies or effects change.

The first campaign on the earlier 170-input candidate generated60 variants:
34 caught,25 unviable,1 missed,0 timeouts; raw score34/35. Its parser error keeps
that receipt incomplete. The exact diff SHA256 is
`4f3d15890b9e171becf612c75c3e94447b8d27b9cf37b55d0227b231a20fa49d`;
raw outcomes SHA256
`67cabffc30f3f3cac2ead34d11f94b2c957eb3e07a7be7166bf7e76b536ff00b`.
An exact reviewed equivalence removes one survivor from the viable denominator,
not from the retained raw results: adjusted34/34 only after current-candidate
policy admission and a complete campaign. It supplies no native, production
sample-eligibility, estimator or release qualification.
