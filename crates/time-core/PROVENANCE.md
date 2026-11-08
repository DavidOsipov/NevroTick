# time-core source and contract provenance

## S1.4 conditional frequency-prior adaptation — 8 November 2026

The reviewed Revision 2 contract in
`tasks/s1.3-source-selection-readiness.md#s14-frequency-prior-and-conditional-holdover-contract`
selects a coupled child of the existing exclusive reference/phase owner.
The source basis is Chrony 4.9, revision
`8df4f1263e206585e6c4e61352d042c2dd12916a`, `reference.c`:
`get_clock_estimates`840–873, its real `REF_SetReference` caller930–986,
and conditional dispersion growth around730–755. The retained complete file is
38,331 bytes with SHA256
`e9a9cb678413366c51d42e5e3dd13f8a4104a3006a538987f66bc3a2a4da803f`.
Its original Richard P. Curnow1997–2003, Miroslav Lichvar2009–2018/2020/2022
and Andy Fiddaman2024 notices and GPL-2.0-only grant/disclaimer accompany the
adapted numerical source. David Osipov owns the AI-assisted Rust modification.

The fixture keeps a same-chart mean instead of assuming the prior proposed
frequency was applied. Both means use positive-behind displacement per
monotonic second; the effective baseline remains a separate active-awake rate.
Normalized weights and the nonnegative algebraic skew mixture avoid unnecessary
absolute-skew/mean squaring. Both-zero weights refuse explicitly; no upstream
NaN/clamp behavior or new default is adopted. Statistical rate/phase SDs remain
original fit diagnostics, distinct from the conditional model and its separate
arithmetic enclosures. Added prediction growth is not a total UTC/root bound.

Actual source receipts, original observation age, explicit clock association,
known-total-effect anchor rebasing and sole continuation custody remain at the
coupled lifecycle owner. The current additive zero-rate chart effect is not
Chrony's multiplicative cooked/raw ppm conversion. No native oscillator mapping,
persistent rate acknowledgement, shipping limit, physical calibration or
service Tracking/Holdover authority follows. Source authoring and its exact
ordinary/contract/coverage/mutation qualification are separate stages; historical
finite-plan scores do not transfer to changed or new source. Formal work remains
post-V1 and all other execution holds remain.

## S1.4 finite phase-plan adaptation — 8 November 2026

The accepted contract in
`tasks/s1.3-source-selection-readiness.md#s14-finite-phase-plan-lifecycle-contract`
selects an owning, inert finite-plan mechanism beneath the existing reference and
settled-rate owners. The accounting order is source-derived from Chrony 4.9,
revision `8df4f1263e206585e6c4e61352d042c2dd12916a`: `REF_SetReference` in
`reference.c`934–987, `LCL_AccumulateFrequencyAndOffset` in `local.c`732–764,
and `update_slew` in `sys_generic.c`180–270. Known delivered transient correction
is accounted before selecting its replacement; effective baseline and transient
rates remain distinct. This is not a clean-room provenance claim.

The existing additive ledger now retains the complete immutable source files,
verified against their full Git blob identities before adaptation:

| File | SHA-256 | Original copyright notices |
| --- | --- | --- |
| `reference.c` | `e9a9cb678413366c51d42e5e3dd13f8a4104a3006a538987f66bc3a2a4da803f` | Richard P. Curnow1997–2003; Miroslav Lichvar2009–2018,2020,2022; Andy Fiddaman2024 |
| `local.c` | `af16a2e6f46716f5bd4e405bdeebaecacbf15ca62476e153323e3836e2c7d3ac` | Richard P. Curnow1997–2003; Miroslav Lichvar2011,2014–2015,2025 |
| `sys_generic.c` | `1556c00f4b646634879d311ea406681f661dd67e03367d6a73ff116ff01e048f` | Miroslav Lichvar2014–2015 |

Each file grants GPL-2.0-only. Its complete original notice/grant/disclaimer is
retained in the raw source, and the adapted owner preserves the selected
`sys_generic.c` notice. David Osipov owns the AI-assisted Rust adaptation. The
original numerical bound implementation is kept under the existing chart
roundoff owner; it is not an upstream numerical-accuracy or parity claim.

Deliberate deviations are positive-behind phase, explicit common additive rates
in displacement seconds per active-awake second, typed independent monotonic and
active clocks, a caller-declared known effective baseline, explicit finite fixture
horizon/caps, and carried binary64 debt with separate outward arithmetic bounds.
The implementation does not copy Chrony's Unix calls, constants, opposite step
sign, cooked/raw frequency denominator or seconds-squared correction parameter.
Full oscillator learning, adaptive horizon policy, physical forecast/accuracy,
native actuator mapping and durable broker authority remain parent obligations.
No shipping correction defaults or new source eligibility follow.

This is implementation/provenance preparation. Actual compiler, behavior,
external-contract, coverage and mutation results belong to the existing S1.3
task against their executed candidate. Formal work remains post-V1.

## N2a conditioned fitting authoring — 7 November 2026

The independently reviewed N2a contract in
`tasks/engine-transport-pilot.md#n2a-concrete-contract--conditioned-suffix-and-enclosed-affine-fit`
admits a bounded translation into `src/conditioned_fit.rs`, with new original
finite/subnormal interval primitives in `src/conditioned_fit/arithmetic.rs`.
The selected Chrony 4.9 revision remains
`8df4f1263e206585e6c4e61352d042c2dd12916a`. A fresh immutable GitHub read of
`regress.c` retained19004 bytes with SHA256
`bcd30c41f1a5c6c528adf199b6ef5215f62cc4c9078a82152a86ea90e46453f7`.
Selected references are `RGR_WeightedRegression`69–93 (inverse-divisor affine
coefficients and weighted SSE) and `RGR_FindBestRegression`252–269 (coefficient
context). Preserve the Richard P. Curnow1997–2003 and Miroslav Lichvar2011,
2016–2017 notices and exact GPL-2.0-only grant reproduced below in the derivative
fit owner. This is source-derived work, not a clean-room implementation.

Deliberate deviations are a suffix borrowing the actual immutable N1 chart and
associated divisors, full finite/subnormal arithmetic, one saved reciprocal per
row, centred response, five geometry-first ordered passes, exact-original-rational
enclosures and checked power-of-two lifting. The exact newest phase anchor stays
separate from the correction. The independent fixtures use exact rational facts;
upstream agreement supplies only supplemental evidence. No variance divided by
degrees of freedom, confidence/skew, runs, adaptive history, production source
authority, clock effect or native caller is adopted. The old fixture fitter and
its qualifications retain their historical scope.

The additive Chrony ledger was recorded before source translation. This is
authoring admission only: compiler, behavior, contract, coverage and mutation
results must be recorded against the final integrated candidate in the existing
task. Formal execution remains post-V1; no dependency or engine graph edge changes.

This crate is GPL-2.0-only source-derived work, adapted on 30 October 2026.
It supplies checked synthetic point/batch admission, response-independent
geometry, exact-constant handling and an ordered inverse-divisor coefficient
fit. The coordinator retained compiling behavioral RED for the absent operation
before authorizing this coefficient implementation. The temporary stub and
error variant have been removed. This authoring record supplies no independent
behavioral GREEN, upstream-parity, dependency, Windows or release qualification;
actual results belong to the coordinator's candidate-bound receipts.

The owner is David Osipov, with AI-assisted Rust adaptation. The private
candidate's `tasks/s0.1-chrony-ledger.json` contains the reviewed additive entry
before translation; historical source-review fields and receipts remain intact.
The S1.2 preparation's `public-api.md` SHA-256 is
`85ae325d40ec6c379d2367694e22157d7f6956b5cc9a82920fc930d51ab243dd`.
`numeric-reviewed-card.md` binds the separately reviewed synthetic envelope,
computed spread semantics and operation order. Independent tests own exact
facts and tolerances; this source never manufactures their expected output.

## Exact upstream scope

Chrony 4.9 revision `8df4f1263e206585e6c4e61352d042c2dd12916a` is retained.
Coefficient/time-geometry references are `regress.c`:
`RGR_WeightedRegression` lines 69–87 and `RGR_FindBestRegression` lines 252–269.
The adaptation preserves inverse-divisor least-squares coefficients and centered
equations, with explicit scaling, checked arithmetic and result construction.
The exact-constant branch follows the reviewed prospective correction rather
than copying the upstream formula's constant rounding discrepancy. No other
upstream algorithm or original test was copied or executed.

| Retained file | SHA-256 | Role |
| --- | --- | --- |
| `regress.c` | `bcd30c41f1a5c6c528adf199b6ef5215f62cc4c9078a82152a86ea90e46453f7` | Selected inverse-divisor/time-center/coefficient equations only. |
| `regress.h` | `eb48f4cb99d5aee47169146449c0a3efdd8a110db79cb3710c0d09704d7d6fa8` | Signatures and minimum-count context; not an adopted API. |
| `sourcestats.c` | `9d7f6e04db284fcfde74c1a95a428d01d52d56548244610f75116fac1b0bddf6` | Actual caller, newest-reference and opposite stored-sign context only. |
| `sourcestats.h` | `c773b3aae44ea954a23a9b9e433d8ae6c8327ef1d7aaed9bb145c830967ec780` | Caller API context only. |

The inspected source unit test `test/unit/regress.c`, SHA-256
`1d2fbc3264e3be7d52c3ef7e0e7c6ff4d9a1c058227f35e2cd75c0f088edfe35`,
supplies supplemental scenario leads only; it was not copied or run.
Retained fix/alternative findings inform the existing task and ledger; no
upstream build/dependency closure or executable parity is claimed.

## Original notices and rights

The source file retains the original `regress.c` copyright and GPLv2 notice:

```
Copyright (C) Richard P. Curnow  1997-2003
Copyright (C) Miroslav Lichvar  2011, 2016-2017

This program is free software; you can redistribute it and/or modify
it under the terms of version 2 of the GNU General Public License as
published by the Free Software Foundation.

This program is distributed in the hope that it will be useful, but
WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
General Public License for more details.

You should have received a copy of the GNU General Public License along
with this program; if not, write to the Free Software Foundation, Inc.,
51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
```

The inspected headers credit Richard P. Curnow 1997–2002; caller-context
`sourcestats.c` additionally credits Miroslav Lichvar 2011–2014, 2016–2018, 2021.
Each inspected file grants GPL version 2 without an “or later” grant. Translation
is not clean-room work. `COPYING` is the retained complete GPLv2 text,
SHA-256 `ab15fd526bd8dd18a9e77ebc139656bf4d33e97fc7238cd11bf60e2b9b8666c6`.
Preserve file notices and corresponding-source/build obligations; this is no
blanket legal clearance for unrelated graphs or distribution artifacts.

## Deliberate adaptations and bounds

- Immutable checked synthetic 3–64-point inputs, newest relative reference zero,
  explicit nonzero fixture frame and positive-behind phase convention. Chrony's
  actual source-statistics consumer stores the opposite sign; no caller adapter,
  production eligible sample or native mapping is introduced.
- Mandatory three-field garde validation inside the only point constructor,
  shared typed predicates, finite/normal-or-zero field rules, and count/order/
  reference/span checks before copying/geometry publication.
- Two chronological x/divisor-only passes using `z=x/H`, `q=1/d`, centered time
  squares and the reviewed computed binary64 spread threshold. Geometry
  construction never evaluates phase-weighted products or response statistics.
- After checked geometry, exact numerical phase equality (including mixed signed
  zeros) returns the original first phase bits and positive zero rate, without
  response arithmetic. This preserves the reviewed `0.1` witness and admits
  a valid `MIN_POSITIVE` constant without unnecessary underflowing products.
- Nonconstant data alone evaluates chronological `mu_y=sum(q*y)/W` and
  `Szy=sum((q*(z-mu_z))*(y-mu_y))`, then `B=Szy/Szz`, `A=mu_y-B*mu_z`,
  and physical phase rate `B/H`. Private checked coefficient/result construction
  admits finite normal-or-zero outputs without reusing the point ±1-second
  envelope or applying a physical-policy clamp. The coefficient owner supplies
  no residual diagnostics; the separate S1.2b supplement below adds only its
  three named statistics. Confidence, skew, runs, pruning and history remain absent.
- Every evaluated binary64 primitive is separately assigned and guarded against
  unsupported nonfinite/subnormal output and nonzero-to-zero multiply/divide
  underflow. Typed errors retain Geometry/Response/Coefficients/Result ownership
  and the actual primitive operation, without raw inputs. No explicit FMA,
  reordered reduction or fast-math API.
- Input-array storage is fixed at 64 points, with checked immutable point reuse
  for unused slots. Logical owned input storage is below the reviewed 4 KiB cap;
  no allocation, RSS, whole-stack or native-resource measurement is claimed.
- Internal guard tests in `tests/private/unit_guards.rs` exercise raw scalar helpers
  without constructing invalid domain values. They are source-derived evidence,
  separate from independently authored coefficient and public admission facts.

Range/spread/underflow limits are reviewed synthetic contracts rather than
Chrony source-quality policy, shipped oscillator bounds or physical guarantees.
Compilation, independent numeric tests, optimized/doctest consumers and the
declared coverage/mutation/contract/resource lanes remain separate executed
evidence obligations for this exact authored candidate.

## S1.2b residual-statistics authoring

Controlled private authoring on 30 October 2026 adds
`src/regression/residuals.rs` and `tests/private/residual_guards.rs`, preserving
the qualified coefficient owner and its original private controls. The fixed
contract has SHA-256
`94007a68ca51c31ba0464706ae6efe894a3e67a3d3c52e437767d9016634b2e4`;
the independently checked facts have SHA-256
`3c554c316a63959e74741a561398d218580d86dab51a38460a6e377a80e0e111`.
The candidate's additive S1.2b Chrony ledger was retained before this authoring.

The selected source-derived sequence is the actual
`SST_DoNewRegression -> RGR_FindBestRegression` path's partial numerical scope:
`regress.c` lines 277-280 and 303-312 retain `y-a-b*x` grouping, chronological
`residual*residual/divisor` summation, division by `n-2`, and normalization
associated as `(scale*n)/W`. `RGR_WeightedRegression` lines 89-101 are only a
supplemental algebraic reference; its `scale*(n/W)` association is not adopted.
The pinned revision, source hashes, GPL-2.0-only grant and notices above remain
unchanged, and the new source file retains the `regress.c` copyright/GPL notice.
This is a bounded adaptation of selected statements, not the complete caller,
history, runs/start-index/pruning algorithm, square-root/confidence calculations
or any claim about production physical uncertainty or source quality.

The coordinator retained compiling behavioral RED in
`portable-red-markupfix.json` before this implementation was authored. The
temporary `ResidualError::NotImplemented` variant and associated branches,
raw-control reference and dead-code expectations have been removed. This is an
authoring record; the coordinator owns behavioral GREEN and final qualification.

The sole public producer now performs one fresh same-input parent fit, then uses
the exact returned physical rate and raw relative seconds in the chronological
`y-A`, `B*x`, difference, square, `square/d`, sum sequence. It starts SSE at
positive zero and separately evaluates `E/(n-2)`, `scale*n` and `product/W` in that
order. A private SSE helper accepts any checked line with an immutable checked
point view; it makes no same-input, same-frame or fitted-optimum assumption. The
public producer alone establishes fresh-line association and owns the complete
returned report. Checked count subtraction and `u8::try_from` followed by
`f64::from` preserve exact count/degree conversion without unchecked casts.

All three output scalars unavoidably invoke the one shared garde predicate
inside private construction. It rejects NonFinite then Subnormal then Negative,
admits outputs above one and preserves either signed-zero bit pattern. The raw
primitive wrapper uses one exhaustive step-to-operation mapping and preserves
actual shared Response-stage errors in distinct residual context. Bounded error
formatting includes the exact step/field/issue and the retained typed fit cause,
including for manually constructed public variants, without raw floating values
or library reports. Only the documentation of the pre-existing Response variant
is extended; its shape and fit behavior stay unchanged. Work is bounded by one
fit plus a 3–64-point pass with fixed scalar scratch, no input mutation, I/O,
native work or clock effects. Scalar validation owns only a bounded one-field
temporary report; fixed result fields remain below 512 logical bytes. These
source bounds are not allocation measurements or resource qualification.

Five private raw-control tests cover scalar/garde admission, all nine typed
steps, unsupported operands/results including masking cases, legal neighbors
and total bounded public error formatting/source projection. They invoke zero
point constructors, zero direct fits and zero statistics calls, each emitting
one complete zero-work accounting record. This edit removes only the obsolete
temporary error variant from those controls and preserves their format-length
assertions. No test execution, compilation or passing claim follows from this
authoring handoff. The coordinator owns all format,
compiler, behavioral, contract, analysis, fuzz, cross-target and final-candidate
qualification evidence; no Windows, release or dependency graph is qualified here.

## S1.2c exact-observation normalization RED scaffold

Controlled private source-only authoring on 30 October 2026 adds
`src/regression/normalization.rs` and `tests/private/normalization_guards.rs`.
The existing coefficient and residual operation bodies remain unchanged. Their
owner registers the private child and re-exports the additive API; only the
existing Geometry/Response stage documentation includes the separate time/phase
conversion context. No enum variant, old numerical acceptance or public raw
kernel helper is added to the parent.

The new contract has SHA-256
`d21c51fcb115be0d3ab346da7053db6801aa0a6750931dee7b781819ddd25d30`.
Its independently owned exact/rounded facts v2 have SHA-256
`2c165c30ea2d6c1ea650d6c8bf6921d3c007870de43c9dce89cad34e2abef839`.
Those facts remain independent of this source. The coordinator's
`public-api-contract.md` and `authoring-activation.json` define the additive
Rust shape and source-only gate; the candidate's additive Chrony ledger was
appended before this authoring. Neither this record nor the scaffold qualifies
the local time-domain edge, the normalization algorithm or an execution profile.

The selected prospective reference is `convert_to_intervals` in the retained
`sourcestats.c` lines 353–368 at the unchanged Chrony revision and SHA-256 above.
It supplies only the newest-reference negative-interval geometry. The new module
preserves that file's exact Richard P. Curnow 1997–2003 and Miroslav Lichvar
2011–2014, 2016–2018, 2021 copyright/GPLv2 notice. Its grant remains GPL-2.0-only;
the existing complete `COPYING` and corresponding-source/build obligations stay
unchanged. No ring storage, runs, historical policy or upstream executable/test
is copied or executed. Checked monotonic nanosecond subtraction, the narrow
positive-behind Q33 mapping and mandatory construction are deliberate fixture
contracts, rather than adopted Chrony caller policy or native continuity claims.

Context construction really retains three immutable nonzero asserted tags.
Observation construction really invokes garde on exactly the raw integer phase
and divisor fields, then projects phase-before-divisor typed rejection using the
one narrow integer predicate and unchanged shared divisor predicate. Typed time
and broad phase values stay under their existing owners outside that candidate;
no skipped field, optional validation, raw Q33 constructor or mutable escape is
introduced. Debug exposes only permitted context/count/reference-presence
metadata; errors retain useful bounded closed context and actual typed causes,
including total formatting/source projection for forged public combinations.

Preparation honestly checks count above 64 before reading an observation, then
returns temporary `NormalizationError::NotImplemented` for every other count.
It creates no insufficient/prepared payload, duration, transformed point, batch,
fit or statistic. The private integer helper enforces its own inclusive `2^53`
bound, then likewise returns `NotImplemented` for supported inputs. The separate
raw arithmetic adapter has one exhaustive component-to-stage and step-to-operation
owner. Narrow non-test dead-code expectations identify these two private RED
helpers and must disappear when GREEN uses them. The fixed public outcome shape
retains the actual owned checked input; its narrowly justified
`large_enum_variant` expectation requires actual coordinator Clippy confirmation,
with unfulfilled expectations still denied. No allocator/RSS claim follows.

All payload fields remain private and immutable, without a mixed-input factory,
default or deserialization. The prepared point getter borrows only
`input.batch().values.get(index)`, never unused fixed-capacity slots, reconstructed
points or mutable state. Its declared payloads become observable only after the
future same-input producer establishes their relationships; this RED source
publishes neither a fabricated empty outcome nor a partial prepared input.

Six private raw guard scopes cover phase/divisor/garde rules, primitive ownership,
unsupported intermediates beside signed-zero neighbors, the raw unsigned bound
and total public errors. They construct no public/context/monotonic/UTC value and
make zero observation, normalization, getter, point, fit, statistics or domain
arithmetic requests. Each emits one complete nine-field all-zero
`CORE_NORMALIZATION_PROFILE_COUNTS` record before assertions, including unwind
paths; raw-scope multiplicity remains a separate source-audited count. The file
contains no public numerical oracle or fabricated internal-call telemetry.

No formatter, compiler, Python, mathematical program, Rust/Cargo test, analyzer,
fuzzer or candidate helper was executed by this author. Root owns actual format,
admission, compiling behavioral RED, GREEN and candidate-bound qualification.
This scaffold provides no completed numerical, dependency, native Windows,
resource, full S1.2/P1 or release result.

## S1.2c numeric producer after compiling RED

The preceding scaffold section records the preserved RED stage. After root's
`portable-red-lintfix.json` observed 46 passing and ten causally failing core
scopes, the same selected Chrony geometry was implemented in the existing private
normalization owner. No additional upstream function, history or source-quality
policy was adopted. The original notices and additive source ledger are retained.

The producer now admits every asserted context before exact order, then checks
all actual monotonic durations and the maximum window before floating conversion.
Insufficient counts publish only their admitted context/count/reference. Sufficient
counts also enforce the integer minimum span. Separate guarded high/low-word
reconstruction preserves exact integers; only time's division by one billion is
inexact, while Q33 phase conversion remains exact. Both zero outputs are positive.
Exactly one real point per row and one fresh checked parent input precede paired
publication; no fit, statistic, history mutation, native read or external effect
occurs in preparation. The temporary unimplemented variant and two dead-code
expectations are removed. The public held-point view remains unchanged.

Root retained the independent public test and literal bytes unchanged; the only
private-test change removes the obsolete unimplemented-error formatting case.
Compiler, behavioral GREEN, external consumers, analysis, fuzz and final scope
qualification remain separately evidenced in the coordinator's candidate receipts.
Authored numeric source alone establishes none of those results.

## S1.2d pretranslation source selection

Before any history Rust authoring, root appended the fifth local adaptation in
`tasks/s0.1-chrony-ledger.json`, preserving all18 source entries and the four
previous adaptation records. Chrony4.9 revision
`8df4f1263e206585e6c4e61352d042c2dd12916a`, retained `sourcestats.c` SHA-256
`9d7f6e04db284fcfde74c1a95a428d01d52d56548244610f75116fac1b0bddf6`,
is unchanged. Selected context is `SST_AccumulateSample`289–329: capacity294–298,
chronological admission300–305, time314 and count328. `prune_register`270–285,
ring308–310, phase sign flip312–316 and peer/root quality317–326 are excluded.
The selected history uses existing checked normalization instead of translating
`convert_to_intervals`353–368 again. No upstream executable is built or run.

Unlike upstream eviction-before-order and healthy-history reset, the fixture
transaction rejects context/order first, stages at most64 observations and
commits replacement only after same-input normalization/statistics success.
Incoming phase is already positive-behind and is never negated again. There is
no extra runs history, pruning/aging/source-quality or skew/default policy.
These intentional fixture contracts establish no production history policy.

The future history source must preserve the original Richard P. Curnow1997–2003
and Miroslav Lichvar2011–2014,2016–2018,2021 GPLv2 notice. GPL-2.0-only and
complete COPYING SHA-256
`ab15fd526bd8dd18a9e77ebc139656bf4d33e97fc7238cd11bf60e2b9b8666c6`
remain unchanged, including corresponding-source/build obligations.

The frozen v2 contract SHA-256 is
`53ce5e686cbb88175bd7138302f35c5d02939316615134451ba74c3fee847ea0`;
independent transition facts SHA-256 is
`ec6a1b96806c6219bd2b74c37b737fde7cbfddafe6cf2f07b12cb14d1d9c92d1`.
This record precedes translation. It is source-only activation, with no graph,
compiler, numerical GREEN, sanitizer, native Windows or release qualification.

## S1.2d atomic transaction after compiling RED

The earlier pretranslation selection remains unchanged. Root's retained
`portable-red-lintfix.json` observed compiler/Clippy/rustdoc success,118 passing
ancestor ordinary tests, and73 core scopes:58pass/15fail/zero skipped. All15 unique
new-history failures reached the independent first-result assertion with the
temporary missing-append error. Earlier fixture-visibility and Clippy-style
failures remain recorded as prebehavior failures, never numerical RED.

After that actual compiling RED, the private history owner implements exact
context/order admission, one staged64-cell prefix, one existing normalization
and, only for sufficient input, one fresh same-input residual-statistics call.
At full capacity it stages only the old suffix1..63 plus incoming row. Report
context/reference come from that actual prepared value and statistics from its
actual input; no mixed-input factory or duplicate solver is introduced. The
sole complete state assignment follows all fallible work. A rejected append
preserves every old row, bit, count, exact reference and report.

Temporary NotImplemented is removed. Payload-free StateInvariant belongs only
to checked private count/slice/last/length relationships. These defensive cases
are not reachable malicious typed-observation claims. Root extends only the
existing zero-product manual-error scope with the new public leaf's bounded
formatting/source contract. Original transition assertions and all exact
frozen oracle literals remain unchanged after compiling RED.

The per-append source bound remains64 actual-incoming initialization assignments
plus at most63 retained logical Copy assignments, one normalizer, one statistics
call and zero direct fits or domain/scalar constructors. Live old64-row storage
and staged64 cells overlap the normalizer's duration/point/input scratch and
bounded old/new summaries; no allocation/RSS/physical-copy or native-resource
measurement follows. No new private raw helper, test file, dependency, doctest,
aging/pruning/uncertainty/default or clock authority is selected.

This section records source authoring after RED. Behavioral GREEN, actual
consumer checks, four-file analysis, source faults, history ASan fuzz, cross-build
and final qualification remain separately evidenced and pending.

### Source-derived fixture runs selection — proposed, not executed

The proposed source is Chrony 4.9 revision `8df4f1263e206585e6c4e61352d042c2dd12916a`, immutable [regress.c](https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/regress.c), SHA-256 `bcd30c41f1a5c6c528adf199b6ef5215f62cc4c9078a82152a86ea90e46453f7`. Scopes proposed to be retained are the static table `critical_runs`, lines 153–164, indices 3–64 inclusive; `n_runs_from_residuals`, lines 175–192; and `RGR_FindBestRegression` bounded suffix/decision-window, stop and terminal recount behavior, lines 242–296, with coefficient assignments and same-suffix statistical context at lines 298–316. Line 164 also contains unselected entries above 64. `regress.h:65–70` supplies the pinned ratio-two/minimum-three context. `sourcestats.c:521–562,564–598` identifies the upstream caller, source-quality preparation and successful regression/pruning, not code to be adopted by this fixture operation. Context-only headers, caller and tests are not automatically imported; revisit their file-specific notices if scope changes. The source inventory and original notices are recorded in `tasks/s1.2-regression-readiness.md:42–76` and the existing Chrony ledger.

Preserve `regress.c`'s original copyright notices for Richard P. Curnow, 1997–2003, and Miroslav Lichvar, 2011, 2016–2017, its complete GPL version 2 grant and warranty/distribution notice, and the crate's complete GPL-2.0-only `COPYING` (existing recorded SHA-256 `ab15fd526bd8dd18a9e77ebc139656bf4d33e97fc7238cd11bf60e2b9b8666c6`). Any new source-derived Rust file must retain that complete original file-specific notice and mark its actual modification attribution. The existing full notice appears in `PROVENANCE.md` under “Original notices and rights”; it does not remove the obligation to retain notice alongside derived source. Translation is derivative work, not a clean-room reimplementation or a new GPL grant. Record selected source, deviations and future test evidence in the existing Chrony ledger; retain corresponding-source/build obligations. No unrelated graph or distribution legal clearance is asserted.

The proposed scope is one pure, immutable fixture operation over 3–64 checked synthetic points and an explicit fixture minimum between three and the input count. It uses the existing positive-behind phase convention and inverse divisors; its frame and newest x=0 are asserted fixture coordinates and provide no absolute monotonic timestamp or source identity. For `m=0`, it excludes samples before the original input but tests residuals for an extended ratio-two window that may still include points removed from the selected fit. It keeps Chrony's strict greater-than comparison and fit-only returned run count, including transitions involving signed zero; at three fit points the runs condition necessarily passes for a successful residual computation. The proposed checked arithmetic, typed error/atomicity guarantees, explicit stop metadata and bounded private report differ from the original C assumptions and API. Exact candidate operation order and any further deviation remain to be recorded from actual authorship. Retaining the table's indexed entries does not create a physical uncertainty/confidence promise.

The proposed operation reports a fixture suffix and same-fit residual statistics; it does not prune or mutate `FixtureHistory`, derive source quality/divisors/asymmetry, produce `ValidatedSample` or `SourceEstimate`, establish frequency skew, perform source selection, age a sample, apply or request a clock correction, or port the complete Chrony caller. It introduces no additional retained sample store. Production trust, retention, actual correction coordinates, aging and policy remain with their existing owners.

Proposed independent facts and checks are in `next-runs-contract-revision.md` and `next-runs-contract-and-facts.md`: weighted versus reversed-divisor behavior, strict equality versus greater-than, extended-window versus suffix-only selection, valid post-drop pass, larger-minimum fallback, returned suffix run recount, zero signs and a window with a nonzero start. `next-runs-revision-review.md` records a source/manual-arithmetic review of the revised facts, including an exact-real perturbation bound; the provenance review records the contract SHA-256 `0850be16c69207b3f6c783b29549cc6df4a0d8e97ff3a820d3baaebb9e95fa12` and review-receipt SHA-256 `002bb6a98fd8db0212a3716b6b1e7bb54ff9a2b9a32f084be0250e0d577b1dd8`. These are receipt-reported identities, not a new checksum execution. The review is not a binary64 bound or execution; recheck identities after any change. The previously suggested `1e-12` tolerance remains withdrawn. Preserve predeclared justifiable numeric tolerances, real entrypoint tests and valid/error neighbors, exact candidate/graph identity, nonzero selected/executed counts, bounded resource accounting, and applicable critical-file mutation/coverage and configured checks. Pinned upstream differential comparisons are supplemental and no original upstream test is claimed to have been copied or executed by this draft.

This supplement records no behavioral RED/GREEN, compiler, fuzz, mutation, cross-target, native Windows, product, full P1 or release evidence. Attach actual results and exact source identity in the existing S1.2 task chain when obtained; do not promote planned evidence to a pass. OD-01 authorizes the accepted source-derived reuse direction with provenance and independent verification; OD-04 allows routine bounded engineering without a fresh owner choice. A material unresolved product/security/rights decision still follows the existing owner-decision process.

## Integration checklist for the eventual owner

- Before translation, insert a reviewed pre-translation proposal naming `crates/time-core/src/regression/runs.rs` into the existing ledger/provenance; reconcile selected source, contract and receipt identities first. If the actual source-to-file destination changes, update and review it before adapting that material. Set author, actual modification attribution, candidate identity and executed results only when they exist; do not fabricate them.
- Add one object to `local_adaptations`, preserving schema version, all five prior entries, the original root status and `files[].code_imported` historical source-review snapshot, and exact staged/unstaged custody; add a dated, candid supplement to the existing provenance. Those historical root fields are not current overall implementation claims. Reconcile the earlier stage descriptions as historical if current facts would otherwise appear contradictory. The evidence directory is a local artifact location, not proof of a run or a public reproducibility guarantee.
- Retain the full original notice in every new source-derived file, preserve licence text and verify any intentionally changed licence/provenance inventory and relevant local links with the configured project checks. Attribute the actual modification; do not change copyright history.
- Keep authoring status, independent fact review, test execution, upstream differential evidence, target/feature graph, native, product and release qualification distinct. No additional owner approval is created by this draft.

## 30 October 2026: temporary runs API scaffold

After the pre-translation record above, a private candidate added the proposed
`regression/runs.rs` surface with the complete original notice and explicit
temporary `Unimplemented` response. The scaffold contains no runs table or
selection algorithm. At this point neither compilation nor behavioral testing
has been performed for it; the independent source-only review is retained in
the local evidence directory. The temporary stage and documentation must be
removed when the implementation is authored.

The subsequent pinned, isolated public-consumer run compiled and executed nine
test scopes: all nine failed against the temporary unimplemented selector, with
zero ignored or filtered tests. Seven reported its explicit error; two expected
later typed failure stages. Five invalid-minimum control calls completed before
the later failures. This is behavioral RED for the scaffold, not a valid-path
GREEN, numerical qualification or proof of individual residual signs. The
original output and resource reservation are retained in `runs-red-result.json`
and `runs-red-reservation.json` in the local evidence directory.

Following that RED, the private candidate replaced the temporary response with
a proposed source-derived runs selector and factored shared residual primitives
for the terminal same-fit report. The public pre-existing statistics path is
intended to retain its original operation and error order. Source review,
compilation, behavioral GREEN, numerical qualification and the remaining
analysis are pending for these newly authored bytes; this entry records
authorship rather than test results.

## 30 October 2026: S1.3 restricted interval fixture, pretranslation record

This new record is confined to the private S1.3 snapshot. The older entries
above remain historical records of their own source and candidate stages. No
S1.3 selection algorithm, scaffold, or source-derived Rust has been authored.
The source/contract/construction designs have independent source reviews; this
integrated ledger/provenance record requires a separate independent review
**before any algorithm translation**. It asserts no compiler, behavioral test,
upstream parity, graph admission, upstream defect, product, P1 or release result.

The proposed exact Rust owner is `crates/time-core/src/selection.rs`, with only
module/re-export wiring proposed in `src/lib.rs`. If selected material is to be
adapted into another owner, amend and review the source mapping before copying.
Chrony 4.9 `sources.c` at revision
`8df4f1263e206585e6c4e61352d042c2dd12916a` is retained in the local evidence
directory, 60,639 bytes, SHA-256
`019021c49f6ccc545fc78513494136cc4edd015e757b9dee4cd811b6d9b8759d`, Git blob
`3c6de32cf546e5df49f45a8076da6049bddad15f`:
<https://github.com/mlichvar/chrony/blob/8df4f1263e206585e6c4e61352d042c2dd12916a/sources.c>.
The reviewed selection is `Sort_Element` tag/index context at 159–168 and
`compare_sort_elements` 698–715; plus the `SRC_SelectSource` endpoint/depth and
strict-majority concepts at 1157–1171, 1204–1205, 1234–1272 and 1279–1280.
Its 1307–1350 envelope-containment membership and survivor-minimum path is
comparison context, not the fixture's common-point algorithm. Other source
eligibility, trust, preferences, principal, weighting, reference updates and OS
effects remain context only. An actual adaptation must be checked against the
final source-to-destination map, retain the **complete original notice** below
in the derived source, mark actual modifications and retain GPL-2.0-only COPYING
and corresponding-source/build obligations. This is not legal clearance or a
clean-room claim.

Complete original `sources.c` notice, lines 1–21:

```text
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2011-2016, 2018, 2020-2025
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 *
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 *
 **********************************************************************
```

The opening comment continues with source description in the retained file;
the copyright, grant, disclaimer and distribution notice above is not a
replacement for retaining the file-specific full notice in any future owner.
Retained source hashes and Git blobs below are from the coordinator's
`s13-selected-source/acquisition-context.json` and independent context review;
they were not recomputed by this integration.

| Context-only file at the same Chrony revision | Bytes | SHA-256 | Git blob SHA-1 | Source-specific notice |
| --- | ---: | --- | --- | --- |
| `sources.h` | 5,537 | `c883426f16d0097e2ec816a30f07141decf8f0779b38ed25bff121e221bdd260` | `89cf0add27e25f6b2d77cf0e5021d1eee05e726f` | Richard P. Curnow 1997–2002; Miroslav Lichvar 2014; GPL version 2. |
| `ntp_core.c` | 115,496 | `ca206410bddc03c56996bba2bddbca2ab01029f9c5b619d4a55025f37d8c4ac0` | `86231f1e4ab84138c95d0e5c39639b4cd6218e79` | Curnow 1997–2003; Lichvar 2009–2024; GPL version 2. |
| `test/unit/sources.c` (retained as `test-unit-sources.c`) | 10,476 | `2215cac8524dee6b3380ef1bb828cd031d35e90570ff42656f5bc70612f3176f` | `c7216bb2d77cb287b3d8dbb40056db38a1dd2b9a` | Lichvar 2016, 2018, 2022; GPL version 2. |

Each contextual source has its own complete retained notice and is not selected
for translation. The reviewed NTP path is one caller, not a complete caller
inventory. The unit source's three phases are source evidence only; no original
unit test was executed or copied. `sourcestats.c` is producer context and any
future adaptation requires separate exact pin, scope and notice review.

The independently reviewed alternative is `pendulum-project/ntpd-rs`, revision
`46ec9bb4d5b6cb24f814f5543d85b9138afb4cba`, file
`ntp-proto/src/algorithm/kalman/select.rs`, 10,797 bytes, SHA-256
`98d3b48df0742199ac1c00f323504e0d9340c738f640356f937716f4b530eb23`, Git blob
`0b4c03c73f207208102de036f7d7ecd1a9fec8fb`. Its retained COPYRIGHT states
Apache-2.0 or MIT at the recipient's option except where otherwise noted, and
names Trifecta Tech Foundation, Tweede Golf and contributors, 2022–2024.
Retained context-only rights files at that revision are:

| File | Bytes | SHA-256 | Git blob SHA-1 |
| --- | ---: | --- | --- |
| `COPYRIGHT` | 356 | `0a05e14063c52ccf456a6212c7b816439f51f0ca3c9253916bd07007c3bd21a5` | `75babb45dd0f405e590f5f6cc1fa0976fe6ed90f` |
| `LICENSE-MIT` | 1,104 | `8313742d155656954302b7b1aed06f0c61b3fff685d80dbd0f9a80af7f677276` | `62dbd13a8a79066bdd4eebac105cb167db58c7b1` |
| `LICENSE-APACHE` | 9,723 | `62c7a1e35f56406896d7aa7ca52d0cc0d272ac022b5d2796e7d6905db8a3636a` | `1b5ec8b78e237b5c3b3d812a7c0a6589d0f7161d` |

No ntpd-rs code or dependency is adopted. Any future reuse needs its own exact
licence/notice, compatibility and graph review; this comparison grants none.
The conditional source observations about ties and envelopes neither demonstrate
an upstream bug nor constitute executed differential evidence.

The proposed fixture uses one synthetic real axis, exact i64 integer endpoints
of **closed real intervals**, and at most 16 distinct nonzero caller-asserted
logical labels; these are neither authenticated source identities nor clock
coordinates. Strict majority and the independently configured 1..16 minimum
are distinct metadata and gates. Only one global-maximum connected component
can produce a left-end witness and the complete sorted set of labels covering
it; multiple maxima passing the other gates return a typed fixture ambiguity
without a chosen result. This deliberately differs from Chrony's tied-maxima
envelope and survivor minimum, and does not settle product tie, minimum,
independence, eligibility, fallback, principal or clock policy.

The separately reviewed construction design proposes unavoidable garde-backed
private fallible raw-label, **whole endpoint-pair** (`lo <= hi`) and minimum
validation, bounded batch capacity/first-duplicate guards, immutable values and
checked output ownership. It does not use no-op pair checks or
`allow_unvalidated`. The actual feature graph, derive syntax/compilation,
privacy, before-read receiver seam, error order and real bypass/valid-neighbor
tests remain to be established. Garde's bounded reports may allocate; no
allocation-free claim follows. Record actual authorship, modification notices,
source identity and executed evidence only after they exist. Keep the existing
S1.3 task and Chrony ledger as the single record for this bounded slice.

### Main-checkout relocation, 30 October 2026

The preceding S1.3 pretranslation section retains its historical private-stage
statements. The reviewed inert `selection.rs` scaffold, its external test draft
and module wiring have now been installed in the main checkout at David's
request, with the append above and seventh ledger entry. All six preceding
adaptations and their provenance were preserved. No private runs, build-policy
or verification source was copied over the newly qualified main candidate.

The scaffold explicitly returns unimplemented errors, constructs no domain
values and contains no Chrony algorithm translation. The main tests now include
additional independently derived input-order, maximum-component, endpoint
capacity, sparse-supporter and extreme-value facts; they have not been executed.
`tasks/s1.3-source-selection-readiness.md` owns current candidate and test evidence.
Earlier authoring hashes continue to identify their historical bytes. Actual
constructor/algorithm authorship, notices, graph admission and executed results
must be recorded as subsequent stages, without relabeling this move as a pass.

### Local constructor implementation, 30 October 2026

David Osipov with AI assistance authored the checked interval and minimum
constructors in the main `selection.rs` owner after the initial real constructor
RED. Three private one-field garde candidates enforce a nonzero asserted label,
the actual whole endpoint-pair order and an independent 1..=16 minimum. The label
rule precedes bounds; its successful value is checked into `NonZeroU64`. The
sole-rule failures become bounded typed errors, with raw reports dropped. This
does not independently detect an erroneous validator/predicate disagreement.

The obsolete unimplemented constructor and unused mismatch variants were removed
from the unreleased fixture API. Batch admission and selection remain explicit
stubs; no Chrony selection algorithm is translated by this local construction
work. The complete retained source mapping and original notices above still
govern any later adaptation. Constructor GREEN, bypass checks and complete
selection qualification remain pending; the task records actual evidence.

### Closed-real fixture adaptation authorship, 30 October 2026

The preceding constructor-stage paragraph describes its historical candidate.
The current main owner now also stages at most sixteen checked intervals before
publishing a batch and evaluates the declared closed-real fixture. David Osipov
with AI assistance adapted the pinned endpoint-ordering, depth and strict-majority
concepts mapped above. The derived file retains the full original `sources.c`
copyright, GPL version-2 grant, disclaimer and distribution notice, and adds the
actual dated modification attribution and explicit fixture deviations.

The source-reviewed evaluator candidate is `selection.rs` SHA-256
`1d9b5bdf0207a403a16f5d9a7d9d77e966f0daf48b25ba1b2108fcb8a57bd0e8`.
It stores at most 32 distinct endpoint keys and 63 alternating point/open-span
cells, then checks a sole maximum component's complete supporters and exact
intersection. Local garde constructors and the capacity/duplicate receiver are
separate local code. This is no principal, source-eligibility, trust, freshness,
weighting, network or correction implementation, and does not claim Chrony's
multi-peak envelope as a common-point intersection. Source review found no
demonstrated algorithm defect; runtime, privacy, coverage/mutation, full graph
and native qualification remain owned by the existing task.

### Principal and hysteresis pretranslation mapping, 31 October 2026

The preceding selection mapping remains unchanged. The next separate owner is
`src/principal.rs`, adapting Chrony 4.9 `sources.c` at revision
`8df4f1263e206585e6c4e61352d042c2dd12916a`: line 191 and lines 1377–1442,
1447–1490. The retained source is 60,639 bytes, SHA-256
`019021c49f6ccc545fc78513494136cc4edd015e757b9dee4cd811b6d9b8759d`,
recomputed against the current retained bytes before this mapping. These ranges
own the strict score threshold, minimum survivor stratum, asymmetric NTP
reselection penalty, per-source update scoring, stable first-maximum tie,
replacement, pending-update gate, score reset and credit consumption.

Source context at 980–983, 365–375, 848–860, 1197–1202, 1279–1305 and
1344–1348 informs update/reset/negative-result lifetime. The already retained
`ntp_core.c` lines 1876–1877 show the sample then selection update call; its
current SHA-256 is
`ca206410bddc03c56996bba2bddbca2ab01029f9c5b619d4a55025f37d8c4ac0`.
These context files/ranges supply no additional copied implementation.

The complete original Curnow/Lichvar copyright, GPL version-2 grant, disclaimer
and distribution notice recorded above must remain in the new derived file,
with actual dated modification attribution. `COPYING`, corresponding source
and build obligations remain; translation does not establish clean-room work
or license clearance for an unqualified product graph.

The fixed ordered roster has at most sixteen asserted fixture identities.
The actual existing common-witness selector owns agreement; the new state owns
scores, observation watermarks, pending credit and cached source distance/stratum.
Explicit fixture reselection/stratum parameters have no shipped defaults. Seed
captures initial unconsumed observations without selecting; later transitions
derive at most one new-source event from original latest-observation timestamps.
An absent registered principal uses owned cached numerical context during scoring
before forced replacement. This preserves score-before-replacement order, but
uses its last projection; Chrony refreshes registered-source selection data at
line 1030. Absent-source aging parity is not claimed. Boolean pending credit replaces upstream counters;
consuming it yields only an inert principal-ready decision, not a reference or
clock effect. Configuration/source-lifetime/mapping changes require new state.
Unsupported arithmetic and invalid transitions preserve the complete old state.

The reviewed contract is `next-s13-selection-slice.md`, SHA-256
`9c87c7dff9d632fd01e1d59b8b0691048f416b3368d9ad13058b41725afe7b52`,
and independently derived sequences are `principal-independent-facts.md` in
the existing S1.3 evidence directory. Tests are locally authored, not imported
upstream tests. This paragraph records a proposed source-to-owner map before
algorithm translation. Independent review of these exact new mapping bytes,
mandatory garde construction, actual source/graph admission, behavioral tests
and per-file analysis remain pending. No production freshness, source identity,
eligibility, combination, P1, Windows or release result follows.

### Principal fixture adaptation authorship, 31 October 2026

The preceding mapping describes the pretranslation stage. David Osipov with
AI assistance subsequently authored `src/principal.rs` after the real seed
constructor RED. The derived file retains the complete original notice and
dated modification attribution. It uses the existing checked arithmetic owner
through crate-internal visibility and the explicit Principal diagnostic stage;
the arithmetic body and existing stage meanings are unchanged.

The mapped scoring/update mechanism now consumes the real projected-batch
selector and original projection metadata. Whole-roster and contextual garde
admission precede one staged transition, checked same-batch decision and commit.
The explicit fixed-roster/event/seed/credit/cached-absence adaptations above
remain; compatible weighting, production source eligibility and reference/clock
effects are not implemented. The independently authored initial producer-to-
consumer sequence passed its declared Linux fixture scope after the initial
seed RED. The existing S1.3 task retains exact candidate hashes, native exits,
actual counters, independent source review and remaining expanded verification.
No upstream differential, complete S1.3/P1 or product/release pass follows.

### Combination kernel pretranslation mapping, 31 October 2026

This additive mapping retains the earlier context-only records. The proposed
derived owner is `src/combination.rs`; `src/lib.rs` may receive module/re-export
wiring only. No kernel translation has yet occurred. The shared unary arithmetic
helper and retained fitted-rate field are independently authored infrastructure;
they do not add another Chrony translation destination.

The source is Chrony 4.9 revision
`8df4f1263e206585e6c4e61352d042c2dd12916a`, `sources.c`, 60,639 bytes,
SHA-256 `019021c49f6ccc545fc78513494136cc4edd015e757b9dee4cd811b6d9b8759d`.
The [ledger](../../tasks/s0.1-chrony-ledger.json) selects `combine_sources`
882–886 for singleton/initialization and 925–926, 932–950, 955 for distinct
weights, principal-centered disagreement, complete phase/rate/uncertainty outputs
and count. Preserve the original Curnow/Lichvar copyright, complete GPL version2
grant/disclaimer/distribution notice and actual dated modification attribution
in the derived file, together with existing COPYING/corresponding-source duties.

The [existing S1.3 task](../../tasks/s1.3-source-selection-readiness.md#kernel-contract-for-translation)
freezes the proposed API, primitive order, independent facts and deviations.
Inputs are genuine common-time projection/principal results plus explicitly
checked fixture phase SD, rate SD and tracking skew. Positive-behind phase and
seconds/second rate remain explicit; raw tracking-producer sign conventions and
additional time extrapolation are not copied. The pure bounded kernel includes
all actual witness supporters; it establishes no compatibility policy or physical
uncertainty calibration. Fixed storage, mandatory Garde admission, original-entry
association, deterministic reductions and typed checked arithmetic replace the
unbounded C storage/assertion assumptions. Singleton passthrough is not fallback
after disagreement or invalid arithmetic.

Compatibility/distant penalties/status at 903–920, tracking production at
894–897, extrapolation at 922–924, logging and reference effects remain context
only. A real transactional compatibility/re-entry/principal-change/replay owner
is still required for the complete combiner; the mathematical kernel cannot
discharge it. Independent review of these actual mapping/contract/provenance
bytes is required before translation. No implementation, differential, full
S1.3/P1, Windows or product/release qualification follows from this mapping.

### Combination kernel adaptation authorship, 31 October 2026

The preceding section records the pretranslation checkpoint. Its independent
review is retained in the ledger and existing S1.3 task. After the genuine
two-source numerical RED and healthy unavailable control, David Osipov with AI
assistance authored the mapped mathematical body in `src/combination.rs`.
The file retains the complete upstream notice and dated modification attribution.
It consumes the existing checked arithmetic owner and crate-internal square root;
neither helper is another translated Chrony source destination.

The implementation preserves the distinct reciprocal phase/rate weights,
original-principal disagreement, three checked square-root uncertainty outputs,
exact singleton passthrough, actual witnessed-supporter associations and bounded
ascending-label reduction. These source changes are under verification, not
qualified by this authorship record. Mandatory raw uncertainty admission and
original-entry/context guards remain separate from physical confidence or
production eligibility. The full compatibility/penalty/re-entry/replay lifecycle
and external reference effects remain required later work as recorded above.

## S1.2 retained-prefix runs extension, 2 November 2026

David Osipov with AI assistance is extending the existing `regression/runs.rs`
owner to admit an explicit decision-only prefix. The frozen contract is
`tasks/s1.2-regression-readiness.md`, section "S146 retained-prefix selector
contract". The independently derived two-round facts are retained in external
`s145-auxiliary-history-facts-v1.json`, SHA-256
`cf76352c6c57f4a0ffa2864277e2f93c28f11c9a34833040712aca1fe9b3bea6`.

The unchanged Chrony 4.9 revision above supplies `RGR_FindBestRegression`,
`regress.c` lines 201–207 and 271–288: distinguish the fitted population from
earlier decision rows and begin candidate fitting after that retained prefix.
The existing checked fit, residual window and terminal-statistics implementation
is reused. `sourcestats.c` lines 270–283 explain the later caller's need to retain
pruned rows; that caller is not activated by this selector extension.

The combined input remains at most 64 rows, including auxiliary rows. This
deliberately retains less decision history than upstream's possible 128-row
combined arrays, preserving the canonical product cap. The existing zero-prefix
API, positive-behind phase/rate convention, checked numerical envelope, complete
regress.c notices, GPL-2.0-only grant and COPYING remain unchanged. Source
activation, feature RED and subsequent checks establish distinct evidence;
this notice establishes no caller, production or release qualification.

## S1.2 staged adaptive-history caller, 2 November 2026

The subsequent explicit authoring scope is `regression/adaptive_history.rs`,
owned by David Osipov with AI assistance. It adapts retained-versus-fitted row
ownership and success-dependent pruning from the same pinned `sourcestats.c`,
lines 270–329 and 556–598, through the checked normalizer and retained-prefix
selector. Original source copyright is Richard P. Curnow 1997–2003 and Miroslav
Lichvar 2011–2014, 2016–2018, 2021; retain the complete GPLv2 grant/disclaimer and
existing COPYING. The source SHA-256 remains
`9d7f6e04db284fcfde74c1a95a428d01d52d56548244610f75116fac1b0bddf6`.

The S147 contract in the existing S1.2 task freezes combined-64 retention,
transactional rejection, explicit configured/effective minima, pre-/post-prune
indices and current-prefix/report association. Mandatory fixture admission replaces
the upstream destructive invalid-time reset. Divisor derivation, asymmetry,
confidence/skew clamps, production source quality, correction feedback, restoration
and Windows effects remain outside this activation. The independent rational
oracle remains separate from any executed Rust observations; source authoring
does not establish those remaining obligations.

## S151 same-fit standard errors — pretranslation, 3 November 2026

Before authoring, the existing S1.2 task freezes an explicit fallible coefficient-
standard-error method over the genuine residual report. Contract SHA-256 is
`319842addf97cfe5baa4afae84c0cccad94490ff844f90180bce8bb19dc52ebd`; independent exact facts SHA-256 is
`ec68884fe760ad2ec9b39204bde679fb32c788b97bb2202d1fcdd7a7ed5381f6`. The retained source remains Chrony 4.9 commit
`8df4f1263e206585e6c4e61352d042c2dd12916a`, regress.c SHA-256
`bcd30c41f1a5c6c528adf199b6ef5215f62cc4c9078a82152a86ea90e46453f7`.
Selected RGR_FindBestRegression statements are lines 309-311: rate standard
error, mean/rate product and phase standard error. Earlier same-fit residual
scale and geometry are already separately owned. Copy the actual producing
input's private normalized Geometry into the residual report; the new method
derives V and u through the frozen checked operations, then retains the selected
upstream standard-error grouping. Normalized geometry conversion, checked
nonfinite/subnormal/underflow rejection, atomic typed output and explicit-only
calculation are deliberate adaptations; no Chrony bit parity is claimed.

Retain the complete regress.c notices for Richard P. Curnow 1997-2003 and
Miroslav Lichvar 2011, 2016-2017, GPL version 2 grant/disclaimer, existing
GPL-2.0-only COPYING and corresponding-source/build obligations. Modification
attribution is David Osipov with AI assistance, 3 November 2026. No additional
source file or dependency is imported, and no upstream executable is run.

The result reports phase seconds at the original zero-reference instant and
rate seconds/second, not confidence bounds, calibrated tracking skew, physical
uncertainty, sample eligibility or correction authority. Existing fitting and
append behavior stays under its previous contract. Independent weighted/common-
scale, selected post-prune, zero-residual and public tiny-output boundary facts
precede implementation. This is authoring/provenance admission only; behavioral
RED/GREEN and exact-candidate qualification remain pending.

## S152 same-owner fitted tracking — pretranslation, 3 November 2026

The S1.3 task freezes genuine adaptive-history → raw-anchor projection with opaque
same-owner fitted tracking → actual-entry combination/lifecycle. Contract SHA-256
`bb98b2a0a14025aa4c275febe1281b1b3781107385dfc2788e0cce2536dc1e31`; independent facts SHA-256
`32c6e2c8e1e43e25288830a35cfa959c98397c4467f626e16c7dfdd2470ebb91` precede source authoring.
Selected Chrony revision remains `8df4f1263e206585e6c4e61352d042c2dd12916a`.
`sourcestats.c` SHA `9d7f6e04db284fcfde74c1a95a428d01d52d56548244610f75116fac1b0bddf6`
contributes SST_GetTrackingData lines709–714; `sources.c` SHA
`019021c49f6ccc545fc78513494136cc4edd015e757b9dee4cd811b6d9b8759d` contributes
acquisition894–897, fit-reference age922–924, weights925–926, reductions932–950.
Tracking root-delay/dispersion lines715–718 are excluded. Reuse existing checked
normalization/arithmetic/standard errors and actual-entry admission. Private
same-owner association, explicit new APIs, bounded Garde construction and failure
atomicity are deliberate adaptations; legacy selection/combination stays intact.
No C executable, differential bit parity, physical confidence, native Windows or
release qualification is implied. David Osipov with AI assistance, 3 November 2026.

Retain both complete original notices alongside the adapted source, existing
GPL-2.0-only COPYING and corresponding-source/build obligations. These are the
original file headers, not blanket clearance for another dependency/package graph.

### sourcestats.c original notice

```text
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2011-2014, 2016-2018, 2021
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 * 
 **********************************************************************

  =======================================================================

  This file contains the routines that do the statistical
  analysis on the samples obtained from the sources,
  to determined frequencies and error bounds. */
```

### sources.c original notice

```text
/*
  chronyd/chronyc - Programs for keeping computer clocks accurate.

 **********************************************************************
 * Copyright (C) Richard P. Curnow  1997-2003
 * Copyright (C) Miroslav Lichvar  2011-2016, 2018, 2020-2025
 * 
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of version 2 of the GNU General Public License as
 * published by the Free Software Foundation.
 * 
 * This program is distributed in the hope that it will be useful, but
 * WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
 * General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA.
 * 
 **********************************************************************

  =======================================================================

  The routines in this file manage the complete pool of sources that
  we might be synchronizing to.  This includes NTP sources and others
  (e.g. local reference clocks, eyeball + wristwatch etc).

  */
```

## S154a mathematical affine chart view — 3 November 2026

The existing S1.2 task freezes a borrowed, inert mathematical chart view before
translation. Selected Chrony revision remains
`8df4f1263e206585e6c4e61352d042c2dd12916a`. Rechecked retained files:

- `sourcestats.c` SHA256
  `9d7f6e04db284fcfde74c1a95a428d01d52d56548244610f75116fac1b0bddf6`,
  SST_SlewSamples728–760: both retained decision rows and fitted coefficients must
  agree about a coordinate change.
- `sources.c` SHA256
  `019021c49f6ccc545fc78513494136cc4edd015e757b9dee4cd811b6d9b8759d`,
  slew_sources1542–1558: known changes versus unknown-step reset and dependent
  selection. Global fanout/reset/selection migration is **not implemented** here.
- `util.c` SHA256
  `c0e80e237ceb9705fb42407ab9dd4dffbb29c711e48ccca30f925d716d0cd9dd`,
  UTI_AdjustTimespec618–625, inspected for the coordinate convention only.
  No util.c text or wall-time adjustment algorithm is copied.

This is a deliberate partial adaptation, not SST_SlewSamples bit parity:
MonotonicNs instants stay unchanged, positive-behind phase maps as
phase'=phase−(D+F*(t−common_reference)), and rate'=rate−F. Chrony's remapped wall
times and `(old_frequency-dfreq)/(1-dfreq)` denominator are inapplicable to this
unchanged independent variable and are not imported. Original genuine Q33 rows,
same-report statistics and exact observation identities remain immutable. A
distinct chart view/projection carries target labels and separately bounded new
floating arithmetic. It does not update raw histories or install actual clock
correction state. Refused/Unknown are fixture request outcomes, not broker proof.

Independent rational fixtures and the binary64 u+eta bound were frozen in the
S154a task before authoring. The new outward-bound arithmetic and chart-specific
enclosure adapter are David Osipov with AI assistance, 3 November 2026; they reuse
the existing checked primitives and exact Q33 enclosure owner. Retain the complete
sourcestats.c/sources.c notices already reproduced above, the notices with the
adapted owner, existing GPL-2.0-only COPYING and corresponding-source/build
obligations. No new dependency or upstream executable is adopted. Authoring and
mathematical review establish no Rust/native execution, achieved effect, physical
UTC uncertainty, stateful generation migration or product qualification.

### S155 chart-bound interval batch and selection

`src/chart_batch.rs` is an additive original Rust adapter by David Osipov with AI
assistance, 3 November 2026. It follows the repository's existing projected-batch
admission pattern and delegates interval uniqueness and majority/minimum semantics
to the existing qualified owners; it copies no additional upstream algorithm.
Complete chart descriptor values/bits and exact evaluation time are checked while
each original chart interval and the selection-to-batch borrow remain intact.
The existing Chrony provenance, notices, GPL-2.0-only licence and corresponding
source/build obligations remain. No dependency, actual clock correction, history
migration, production eligibility or physical-uncertainty claim is introduced.
The S154c evidence records its historical candidate; S155 needs its own checks.

## Source-quality window variance divisors — 6 November 2026

The separately reviewed S1.2 child selects the complete window-dependent divisor
derivation for `window_variance.rs`. This is source-derived work, modified by
David Osipov with AI assistance on 6 November 2026, under GPL-2.0-only.
It is not a clean-room implementation or a physical uncertainty guarantee.
The existing Chrony ledger records the selected scope before translation.

At revision `8df4f1263e206585e6c4e61352d042c2dd12916a`, the selected sources are:

| Source | SHA256 | Adapted scope |
| --- | --- | --- |
| `sourcestats.c` | `9d7f6e04db284fcfde74c1a95a428d01d52d56548244610f75116fac1b0bddf6` | Constant at 496 and complete distance/median/precision-to-divisor calculation, 528–550. |
| `regress.c` | `bcd30c41f1a5c6c528adf199b6ef5215f62cc4c9078a82152a86ea90e46453f7` | Odd/even median semantics and primitive order, 415–442; selection machinery is replaced by bounded scratch sorting. |
| `util.h` | `845633dc8bf74f2e94456d389123ee495228af90940500ab8e35eaaa5e1f3093` | MAX/MIN/CLAMP/SQUARE semantics, 275–284; precision wins when clamp bounds invert. |

The new Rust source must preserve the original `sourcestats.c` notice, including
Richard P. Curnow 1997–2003 and Miroslav Lichvar 2011–2014, 2016–2018, 2021,
and its full GPLv2 grant/disclaimer. The median and utility notice obligations
remain those retained above. `COPYING` continues to supply the complete GPLv2 text.

The numerical contract accepts strictly positive finite values, including
subnormals, and returns a borrowed original-order snapshot with inline divisors.
It checks every source-order arithmetic intermediate and returns typed errors
on nonfinite results, even where an upstream clamp might conceal overflow.
No synthetic divisor cap, raw input clipping, normal-reciprocal requirement,
ambient clock/precision lookup or physical confidence is inherited.

This entry records source scope and authoring obligations, not completed Rust,
upstream parity, dependency, native or release qualification. The existing S1.2
task owns actual RED/GREEN and subsequent per-file coverage/mutation evidence.
Genuine observation/history admission remains held on its separate production
contract. Formal work remains post-V1.
