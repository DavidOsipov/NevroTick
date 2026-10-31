# Design and verify numerical kernels

Use for changed floating-point estimation, exact-to-floating conversion or a
numerical admission threshold. Read architecture
[sections 6–7](../../../../../docs/windows-time-architecture.md#6-domain-types-and-state-authority)
and [20.1](../../../../../docs/windows-time-architecture.md#201-independent-numerical-truth).
For source-derived work also use [the Chrony recipe](chrony-port-simulator.md).
Keep concrete limits and accepted semantics in the current task/API contract;
these examples adopt no production threshold, dependency or new runner.

## Freeze the numerical question before choosing arithmetic

| Decision | Record at the owning API or existing task |
| --- | --- |
| Representation | Exact input integers/rationals versus the actual admitted floating bit patterns; units, sign, reference instant and frame/generation; deliberate conversion and its rounding/residual |
| Supported envelope | Count, magnitudes, span, weight range, conditioning/geometry and bounded scratch/work; check reciprocal and transformed ranges as well as raw fields |
| Required result | Mathematical quantity and units; coefficient/statistic/uncertainty distinctions; result range need not equal input range |
| Execution semantics | Precision, operation/reduction order where material, fused versus separate operations, signed-zero/subnormal policy and selected target/profile |
| Comparison | Exact identities if required; otherwise justified absolute/relative or ULP criteria, including near zero; keep numerical tolerance separate from physical error/uncertainty |
| Failure | Reachable singular, unsupported, nonfinite, overflow/underflow and conversion failures; unchanged state/output on rejection, with a legal neighbor |

Derive bounds from the real domain and consumers. A finite answer alone proves
neither accuracy nor good conditioning. Compare credible formulations, such as
centered/scaled least squares versus raw normal equations, against cancellation,
range and the required output, not just agreement on a convenient example. Keep
exact timestamp differences before converting relative deltas. A tolerance fitted
after observing candidate results is not an independent acceptance criterion.

## Keep mathematical truth and rounded execution separate

1. **Mathematical oracle:** derive the intended result with exact rational
   arithmetic or a justified higher-precision independent formulation. State
   whether decimal `0.1` means rational `1/10` or the exact stored binary64 value;
   converting a rounded float to a rational does not recover its decimal intent.
   Avoid importing production helpers or copying the same reduction as the sole
   oracle. Preserve units and derivation with each expected result.
2. **Execution control:** when admission or exact behavior depends on rounding,
   use an independently authored model of the specified operations. Retain actual
   input bits, order, rounding and the first discriminating intermediate value.
   Ordinary host-language sums, vector libraries or compiler flags may implement
   different reductions; qualify the model with known controls before use. Exact
   arithmetic rounded only once at the end cannot model rounding after each step.

These answer different questions. Agreement with a rounded-operation model can
confirm execution yet expose a violation of the mathematical/API contract. A
different exact formulation can establish an expected coefficient without
establishing the rounded admission decision. Tie actual Rust observations to the
admitted compiler, target and profile; Python or another model proves no Rust
execution. Rust's [binary64 and `mul_add` documentation](https://doc.rust-lang.org/std/primitive.f64.html#method.mul_add)
distinguishes fused from separate arithmetic. Follow the existing
[floating-point optimization policy](../../optimize-rust-performance/references/measurement.md);
this procedure adds no blanket FMA prohibition or compiler flag.

### Worked independent coefficient facts

For `a+b*x` minimizing `sum((y-a-b*x)^2/d)`, let `q=1/d`. With
`x=(-1,0,1)` seconds, `y=(-1,0,3)` milliseconds and divisors `d=(1,1,4)`, exact
sums are `W=9/4`, `Sx=-3/4`, `Sy=-1/4`, `Sxx=5/4`, `Sxy=7/4`.
The normal-equation determinant `W*Sxx-Sx*Sx` is `9/4`, giving
`a=(Sy*Sxx-Sx*Sxy)/det=4/9 ms` and
`b=(W*Sxy-Sx*Sy)/det=5/3 ms/s`.

Healthy/control variants have independent meanings:

- Equal divisors give `a=2/3 ms`, `b=2 ms/s`; using the divisors as precision
  multipliers instead gives the distinct wrong line `16/21 ms`, `15/7 ms/s`.
- Shifting to `x'=x-1` gives intercept `19/9 ms` and unchanged slope.
- Multiplying every divisor by one positive common factor preserves the exact
  fit. Constant `y=(2,2,2) ms` has exact intercept `2 ms` and zero slope.

These exact facts can check a different centered floating implementation. They
do not prescribe its operation order, prove a tolerance over arbitrary inputs,
or qualify history, residual variance, skew, sample trust or source selection.

### Worked rounded controls

For binary64 round-to-nearest-ties-even after each operation, take
`a=2^53`, `b=1`, `c=-2^53`. Exact arithmetic gives `1`; `(a+b)+c` gives `0`,
while `a+(b+c)` gives `1`. All inputs/results are finite. A model that returns
the same answer for both has not established these specified execution semantics.

A computed threshold can also differ from its exact-real counterpart. Let
`x=0x1.fffffffffffffp-2` (the binary64 predecessor of `0.5`) and evaluate
`g=(x+1)-1` with separately rounded operations. The exact value is
`9007199254740991/18014398509481984 < 1/2`, while the calculated `g` is `0.5`.
Thus `g >= 0.5` accepts under a *computed-value* contract; it does not establish
the exact-real inequality. Freeze which rule the real API intends. Derive its
actual representable cutoff neighbors; do not substitute these illustrative
numbers for the algorithm's own geometry or policy.

Constant-series checks can reveal a subtler difference. For chronological
`x=(-2,-1,0)`, `z=x/2`, binary64 phases all `0.1`, and divisors `(1,16,16)`, a
left-to-right centered fit with `q=1/d`, `W=sum(q)`,
`mu_z=sum(q*z)/W`, `mu_y=sum(q*y)/W`,
`Szz=sum((q*(z-mu_z))*(z-mu_z))`,
`Szy=sum((q*(z-mu_z))*(y-mu_y))`, and `rate=(Szy/Szz)/2`
produces `mu_y=0x1.999999999999bp-4` and
`rate=0x1.5555555555555p-108` when every primitive operation is rounded
separately. The input phase is `0x1.999999999999ap-4`; the exact line is constant.
Finiteness and absence of subnormal intermediates do not catch this difference.
If the API promises the original constant and exact zero rate, this violates it;
if it promises bounded error, assess against that declared bound. A legitimate
exact-constant branch must follow valid geometry/admission, not disguise failed
arithmetic. Do not impose exact-constant results on unrelated algorithms.

## Turn the contract into discriminating implementation checks

- Check applicable identities: constant/linear trajectories, sign reversal,
  reference translation and common weight scaling. Distinguish mathematical
  invariance from bitwise invariance; rounding and the admitted domain can limit
  a transformation. Preserve one useful accepted control for each rejection family.
- Inventory reachable arithmetic failures after admission. Raw infinity rejection
  does not exercise overflow of finite operands. If proven input bounds exclude a
  failure, retain that argument; do not invent an impossible public-path test or
  weaken admission merely to reach it. Challenge independently exposed helpers
  against their own admitted inputs.
- Distinguish zero from a valid cancellation, exact zero covariance, subnormal
  input/result and nonzero multiplication/division rounded to zero. Accept or
  reject each according to the adopted kernel contract; no universal epsilon,
  silent clamp or fallback estimate. Do not use input bounds as output bounds
  without deriving that implication.
- Before implementation/repair, freeze independent expected facts and applicable
  tolerances. Reach the real constructor and numerical consumer; show invalid
  and valid cases, preserved inputs/state and checked outputs. Keep numerical
  accuracy evidence separate from constructor privacy, fuzz no-panic observations
  and tool admission. Reuse [numerical QA](../../../testing/references/numerical-protocol.md)
  and the existing test/analysis lanes rather than creating a parallel framework.

## Optional whole-domain rounding proof

Consider Gappa only for a named consumer-relevant rounding-error bound over a
declared nonempty domain that existing fixtures or bounded Kani checks do not
adequately establish. Begin with one small kernel and compare the maintenance
cost with the current verification route. No arithmetic-only change requires
this tool, a new proof stack or an installation.

Map the model to current Rust conversions, exact constant values, units, operation
order, separate/fused operations and intermediate admission guards. A formula
simplified over the reals can erase rounding and observable rejection behavior.
Distinguish error relative to admitted binary inputs from conversion error
relative to exact source integers/rationals, upstream estimator error and physical
clock uncertainty. An enclosure of already computed floating values establishes
no bound on omitted earlier arithmetic or physical error.

Use the appropriate format and minimum exponent when modeling subnormals;
unbounded-exponent precision alone does not model gradual underflow. Separately
discharge finite exponent overflow, NaN/infinity and any signed-zero bit contract:
Gappa's real-valued model does not by itself establish these Rust behaviors.
Preserve actual nonzero-to-zero/subnormal rejection rules. Check domain
satisfiability, denominators, assumptions and manually supplied rewrite equalities;
contradictory assumptions or unproved hints establish no useful source guarantee.
Treat `-Munconstrained` as exploration with outstanding hypotheses.

Keep model/source hashes, tool/options, domain, theorem, assumptions and results
in the existing task evidence. Distinguish successful Gappa analysis from an
independently checked certificate: its default `-Bnull` emits no proof script;
`-Bcoq` generates one that still needs the compatible checker and libraries.
Admit any selected tool/checker chain before execution. Retain discriminating
Rust fixtures and healthy neighbors. Failure to prove a bound is inconclusive,
not a Rust counterexample; proof of the model alone does not qualify source
correspondence, native arithmetic or clock accuracy.

Primary semantics: [Gappa arithmetic](https://gappa.gitlabpages.inria.fr/gappa/arithmetic.html),
[formalization and hints](https://gappa.gitlabpages.inria.fr/gappa/formalizing.html),
[invocation and proof backends](https://gappa.gitlabpages.inria.fr/gappa/invoking.html).

Report the supported envelope, comparison semantics, reached consumer and
unresolved range/conditioning/target gaps. A finite fixture family supplies
evidence for that family; it is no whole-domain error bound or physical accuracy
claim.
