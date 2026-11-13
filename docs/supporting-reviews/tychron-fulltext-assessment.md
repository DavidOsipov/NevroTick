# Tychron: full-text assessment and NevroTick experiment handoff

**Review date:** 13 November 2026.

**Source baseline:** NevroTick main
[`a25696b8bf67a53ebbcf40c4c0ba43c613159ff1`](https://github.com/DavidOsipov/NevroTick/tree/a25696b8bf67a53ebbcf40c4c0ba43c613159ff1),
tree `9056cbc1f289fb43d81e279c8b7c240b1d1e878c`.

**Paper:** Yilun Liu, Weiping Zhu, Chao Ma and Chuanhe Huang,
[*Improving NTP Time Synchronization Accuracy for NTS Enhancement*][P],
MobiQuitous 2025, LNICST 683 (2026), pp. 314–334. The publisher records first
online publication on 1 May 2026. References below give printed pages and the
corresponding pages of the complete 21-page chapter: printed p. 314 is PDF p. 1.

## 1. Decision and evidence boundary

Retain Tychron as a useful candidate for the existing post-V1 estimator comparison.
The complete chapter supplies more than an abstract-level lead: a concrete noise
weighting formula, an offset/frequency model, source-management ideas, several
network conditions, and enabled/disabled Kalman comparisons. It does not establish
a ready replacement for NevroTick's Chrony-derived processing or a measured
advantage over NevroTick.

The strongest finding is mathematical: the quantity called delay asymmetry is
exactly twice the absolute raw NTP offset. It can be useful when the clock is
already independently known to be close to correct time, but cannot separately
identify clock error and physical path asymmetry. Several numerical claims also
need clarification, especially authenticated-source counts and traffic-rate
normalization. These limitations do not establish that the plotted improvements
are fictitious or that every proposed mechanism is ineffective.

This review distinguishes four evidence classes:

- **Paper-reported:** claims and plotted results attributed to the chapter.
- **Independently derived:** algebra and arithmetic using its stated equations or
  labeled counts; these are analytical checks, not client reproduction.
- **Current-source facts:** inspected repository contracts, source, and recorded
  candidate-specific results at the baseline above.
- **Proposed tests:** future cases under existing owners; no pass is implied.

The assessment inspected the full text and rendered equations/figures. It did
not run Tychron, Chrony or NTPsec, scan the Internet, probe public servers, or
alter a clock. No production algorithm, dependency, roster, polling constant,
admission rule or cryptographic downgrade is adopted.

## 2. Useful contributions and established components

Tychron combines a public-server survey and quality score with authenticated
multi-source sampling, per-source filtering, selection/clustering, weighted
combining, a two-state Kalman filter, and anomaly-driven peer replacement.
Figure 1 labels server choice as weighted random selection; Section 3.2 says
each trusted server has an independent filter before aggregation ([P],
pp. 315, 319–322/PDF 2, 6–9).

The five-stage pipeline itself is established NTP practice. The chapter explicitly
acknowledges that fact on p. 320/PDF 7. [RFC 5905][NTP], Sections 10 and
11.2.1–11.2.3, already describe sample filtering, selection, clustering and
combining weighted by inverse root synchronization distance. This establishes
inherited components, not an absence of useful integration work.

The evaluated [Chrony 4.5 manual][CHR45] already documents delay-based sample
weighting/rejection, automatic estimation of network-jitter asymmetry, and
combining offsets and frequencies from several sources. Its jitter-asymmetry
correction does not promise identification of arbitrary constant path bias.
Comparison must therefore preserve the actual baseline mechanisms rather than
reduce Chrony to unweighted regression or single-source averaging.

The worthwhile research questions concern the particular adaptive-noise rule,
its interaction with source replacement and polling, and whether the combined
design improves accuracy, recovery and cost under controlled assumptions. The
survey also usefully distinguishes successful key establishment from usable
time delivery, although its reported populations need correction below.

## 3. What the equations establish

### 3.1 The asymmetry metric is the raw offset in another form

The paper defines

$$
\theta_{\mathrm{raw}}=
\frac{(T_2-T_1)+(T_3-T_4)}{2}
$$

in Eq. (2), then

$$
\mathrm{Asym}=|(T_2-T_1)-(T_4-T_3)|
=2|\theta_{\mathrm{raw}}|
$$

in Eq. (11) ([P], pp. 320–321/PDF 7–8). The equality is exact for the same
timestamps. Equation (12) is consequently

$$
R=R_0+4\lambda\theta_{\mathrm{raw}}^2.
$$

If the filter observation is the raw exchange offset, this directly makes its
measurement variance a function of that observation's magnitude. Separate
preprocessing would need specification but would not change the raw identity.

For physical interpretation, let `b` be server-minus-client clock offset during
the exchange, and `d_f`, `d_r` the forward and reverse delays. Neglecting clock
change during the exchange gives

$$
\theta_{\mathrm{raw}}=b+\frac{d_f-d_r}{2},\qquad
\mathrm{Asym}=|2b+d_f-d_r|.
$$

Here is an independently constructed two-case counterexample. All values are
milliseconds, with one millisecond of server processing:

| Physical situation | `b` | `d_f` | `d_r` | `(T1,T2,T3,T4)` | Raw offset / `Asym` |
| --- | ---: | ---: | ---: | --- | --- |
| Equal clocks and symmetric path | 0 | 15 | 15 | `(0,15,16,31)` | `0 / 0` |
| Offset clock and asymmetric path | 5 | 10 | 20 | `(0,15,16,31)` | `0 / 0` |

Identical complete observations admit different clock truths. Conversely, a
5 ms clock offset on a symmetric path produces a 10 ms metric despite zero
physical delay asymmetry. This demonstrates ambiguity, not an executed attack
against Tychron: its additional history and multi-peer checks might prevent a
particular trajectory from reaching such a state.

The favorable interpretation is also precise. Writing `a = d_f - d_r`,

$$
\bigl||a+2b|-|a|\bigr|\leq2|b|.
$$

If independent evidence bounds `|b|` by `epsilon`, the metric approximates
physical asymmetry magnitude within `2 epsilon`. A stable, already synchronized
client with unaffected peers could therefore use it to suppress transient
disturbances. The metric itself supplies neither that prior bound nor an
independent measurement of one-way delay.

### 3.2 Observable state does not resolve physical ambiguity

The state is `[theta,rho]`, with transition `A = [[1,dt],[0,1]]` and observation
`H = [1,0]` ([P], pp. 320–321/PDF 7–8, Eqs. (3)–(5)). Its observability matrix is

$$
\mathcal O=\begin{bmatrix}1&0\\1&\Delta t\end{bmatrix}.
$$

Its determinant is `dt`: the stated two-state model is observable for nonzero
elapsed time. Calling that model unobservable would be incorrect.

The different issue is identification of physical offset separately from delay
bias. Add a constant state `a = (d_f-d_r)/2` and observe `y = b+a+v`. For state
`[b,rho,a]`, successive observation rows are `[1,0,1]`, `[1,dt,1]`,
`[1,2dt,1]`, with rank two. Increasing `b` and decreasing `a` equally leaves all
observations unchanged. A linear delay-bias trend can likewise resemble clock
skew. The two-state filter can estimate a useful composite bias without proving
that its estimate is true clock offset.

Increasing `R` discounts a sample; it does not estimate and subtract a signed,
persistent delay bias. The zero-mean Gaussian noise model in Eq. (5) does not
automatically cover adversarial bias. Multiple sources can add information under
explicit fault/path assumptions, but endpoint count alone cannot remove a
shared directional disturbance.

### 3.3 Gain, units and implementation detail

The rendered PDF contains the transposes in Eqs. (7)–(8) and parentheses in
Eq. (9). Their disappearance in extracted text is not a mathematical defect.
With predicted covariance `P^- = [[p,c],[c,q]]`, Eq. (8) gives

$$
K=\frac{1}{p+R}\begin{bmatrix}p\\c\end{bmatrix},\qquad
\frac{\partial K_\theta}{\partial R}=-\frac{p}{(p+R)^2}\leq0.
$$

At fixed predicted covariance, increasing `R` suppresses gain. This matches
Section 4.3's explanation and the conclusion's suppression trigger ([P],
pp. 327, 332/PDF 14, 19). Section 3.2 instead describes gain amplification
correlated with rising asymmetry as a trigger (pp. 321–322/PDF 8–9). Covariance
also changes over time, so simultaneous increasing gain and increasing asymmetry
is possible. The problem is that the chapter does not distinguish these cases
or define one reproducible detector.

Missing details include the gain component, threshold/window/hysteresis,
innovation test, reselection cooldown, `Q`, `R0`, `lambda`, initialization,
sample-window size and exact aggregation weights. The equations are dimensionally
consistent if `rho` means fractional frequency error, `R0` is time squared and
Eq. (12)'s `lambda` is dimensionless and nonnegative. Gain components have
different units, so any combined magnitude needs normalization. Eq. (1)'s score
also needs an explicit unit or scale for synchronization distance in its
exponent ([P], p. 319/PDF 6).

Actual elapsed time matters after missing observations or reselection. Known
clock steps, slews and frequency changes must be reflected in the model or a
stable clock coordinate system. The chapter does not specify that integration.
These are specification gaps, not demonstrated defects in unavailable code.

## 4. Reported performance and its limits

Section 4 names Chrony 4.5 and NTPsec 1.2.3, and Section 4.5 states that absolute
error is measured against an in-house Stratum-1 reference, `S_U` ([P],
pp. 323, 328/PDF 10, 15). These are useful reporting details. The following are
the paper's cumulative quantities, not independently reproduced measurements:

| Scenario | Tychron | Chrony | NTPsec | Paper location |
| --- | ---: | ---: | ---: | --- |
| Ideal LAN | 42.8 ms | 182.8 ms | 199.3 ms | §4.5, p. 328/PDF 15; Figs. 15–16, pp. 327–328/PDF 14–15 |
| Degraded LAN | Below 80 ms | Roughly four times Tychron | Roughly four times Tychron | §4.5, p. 328/PDF 15; Figs. 17–18, pp. 328–329/PDF 15–16 |
| Real Internet | 152.9 ms | Above 660 ms | Above 720 ms | §4.5, p. 329/PDF 16; Figs. 19–20, pp. 329–330/PDF 16–17 |
| Delay attacks | 1,071.2 ms | Above 11,000 ms | Above 11,000 ms | §4.7, pp. 329–330/PDF 16–17; Figs. 23–24, p. 331/PDF 18 |
| Attacks, Tychron without Kalman | Described as four times full Tychron | — | — | §4.7, p. 330/PDF 17; Fig. 25, p. 331/PDF 18 |

Arithmetic on the ideal-LAN totals gives reductions of about 76.6% and 78.5%.
Those percentages describe the reported aggregate only. If cumulative error is
`sum |e_k|`, mean absolute error requires the observation count; if it is an
integral, its units should include elapsed time. No precise accumulation formula,
sample count or sampling cadence is supplied. A time axis reaching 7,200 seconds
does not establish 7,200 samples. Thus the totals cannot independently yield
microsecond mean error or a universal accuracy guarantee.

In particular, 11 seconds of accumulated error does not mean a clock was 11
seconds wrong. The attack prose reports baseline drift around −5 ms and Tychron
peak error below 0.4 ms (p. 329/PDF 16). Smoothness, instantaneous error,
accumulated error and final offset are different measurements.

### 4.1 What the comparisons support

The illustrated traces favor Tychron in benign, degraded and injected-delay
conditions. The same-client Kalman enabled/disabled comparisons are useful
preliminary component evidence ([P], Fig. 11, p. 325/PDF 12; Figs. 24–25,
p. 331/PDF 18). They are stronger evidence for a module contribution than an
end-to-end comparison alone.

However, the chapter omits exact client configurations, source counts, polling
and startup policies, timestamp method, oscillator/platform characteristics,
reference calibration/independence, repeated runs, seeds and raw series. The
ten adversarial servers are not placed against a stated total/selected peer
population or healthy replacement supply. Delay amplitudes, slopes, schedules
and path sharing are insufficiently specified (pp. 327–330/PDF 14–17).

The two Tychron instances in Section 4.3 share a host. They could be independent
read-only estimators; two controllers adjusting one clock would need isolation.
The chapter does not resolve that distinction. Offset curves alone do not
establish independently measured operating-system clock correction. No gain or
innovation trace isolates the claimed detection mechanism. Missing reporting
does not prove the controls were absent, but limits causal attribution and
reproduction.

### 4.2 Three discrepancies that change interpretation

**Duration.** The real-Internet prose describes six hours, while Figs. 19–20
display approximately 7,200 seconds, or two hours ([P], pp. 329–330/PDF 16–17).
A two-hour excerpt is a possible explanation, but is unstated. Cumulative totals
need their actual measurement interval.

**Attack smoothness.** Section 4.6 says more than 90% of Tychron updates are below
0.02 ms. The benign CDFs support that qualitative claim, but Fig. 22(b)'s attack
CDF does not: its 90th percentile is visually around 0.09 ms, not 0.02 ms
(pp. 329–330/PDF 16–17). This is a visual estimate, not a raw quantile. Tychron's
curve still shows a relative advantage in that run. A smooth biased clock can
remain inaccurate, and update intervals affect this metric.

**Traffic-rate labeling.** The stated twelve-hour average is 6.63 requests/minute,
consistent with roughly 4,770 cumulative Tychron requests. Against approximately
11/minute, that is about 39.7% fewer requests. Yet Fig. 14's late cumulative
slope implies roughly 3–4/minute, while its right-hand curve labeled requests
per minute sits around 15–20 ([P], pp. 327–328/PDF 14–15). A five-minute counting
window mislabeled as per-minute could explain the discrepancy; that remains a
hypothesis. Baseline whole-run averages and post-startup slopes also need separate
reporting. Aggregate client request counts do not establish per-server fairness,
bytes, key-establishment cost or server CPU load.

## 5. Server population and score interpretation

Figure 7's labeled counts give the following population ([P], p. 324/PDF 11):

| Response category | Count |
| --- | ---: |
| Plain and authenticated NTP | 285 |
| Plain NTP only | 70 |
| Authenticated NTP only | 3 |
| Neither | 286 |
| Total discovered NTS-KE endpoints | 644 |

Therefore **358 responded in either mode, but only 288 responded in authenticated
mode**. Section 4.2 analyzes 358 functional sources under authenticated
source-quality wording (p. 326/PDF 13); Fig. 8's stratum counts also sum to 358
(p. 324/PDF 11).
Those denominators are not interchangeable if Fig. 7 is correct.

The claimed pool above 600 can charitably mean the discovery inventory, not
successful authenticated time providers ([P], pp. 314, 319/PDF 1, 6). A
certificate-validation/authenticated-response cross-tab is absent; even all 288
cannot be assumed to satisfy both conditions. Missing scan dates, vantage points,
timeouts and raw endpoint evidence further prevent treating this as a current
global census.

Equation (1)'s score and Fig. 10's score distribution do not independently
validate source quality. A spread in a constructed score does not establish
prediction of later true timing accuracy, calibrated trust, or independence.
For NevroTick, discovery, identity/authentication admission, usable time,
independent groups and advisory preference need separate denominators and rules.

## 6. Standards and artifact availability

The statement that NTS-KE operates over TLS 1.2/1.3 is incorrect ([P],
p. 318/PDF 5): [RFC 8915 Section 3][NTS] prohibits negotiation below TLS 1.3.
This is a prose-level correction, not evidence of a Tychron client vulnerability.
Section 8.6 also recognizes asymmetric delay attacks and conditional mitigation
through multiple sources/paths; cryptographic integrity alone does not remove
delivery-delay uncertainty.

No author-linked public implementation, dataset/roster, runnable experiment
package or complete standalone algorithm specification was verified in the
bounded review of the chapter, publisher page, exact-title/DOI/Tychron searches,
author/institution checks and GitHub name search. This is not proof that none
exists. The chapter supplies no located artifact link, although its conclusion
calls the platform reproducible (p. 332/PDF 19).

A faithful reproduction needs original code or an adequate specification,
configuration, parameters, measurement definitions and replay inputs. A future
paper-inspired comparison can proceed without those artifacts by declaring its
own complete choices under MATH's existing limits. It must not be labeled a
reproduction of Tychron or credited with the paper's performance. Any code reuse
separately needs verified provenance and licensing.

## 7. Current NevroTick fit

At the stated source baseline, bounded regression/history, selection, principal
hysteresis and supporter combination already exist in `time-core` within their
documented fixture scopes. [Architecture Sections 7.2–7.2.1][ARCH] require
interval-majority selection, configured minimum agreement, uncertainty retention,
observation identity, common-instant aging and acknowledged-correction accounting.
An added sliding filter or offset cluster cannot silently replace those contracts.

The protected numerical receiver also now exists. The actual public facade still
supplies no date/coherence model, preserving numerical unavailability rather than
inventing production eligibility. The [integrated LIT-0/receiver checkpoint][LIT]
records 409 ordinary engine tests, including both imported numerical regressions,
and the unchanged 38-method catalogue suite on its specific candidate. These are
repository-recorded executions, not reruns by this assessment. Earlier statements
that those imported Rust regressions are unexecuted are stale; remaining
qualification and positive production suppliers remain open.

There is no qualified Tychron-style adaptive Kalman/score/reselection consumer
to test merely by adding another toy implementation. Existing measurement,
identity, source-agreement and catalogue-forgery controls already cover relevant
shared invariants. [OD-18][OD18] calls for genuinely missing regressions at actual
consumers and early specification of future cases, not manufactured test passes.

## 8. Handoff to existing experiments and owners

Use [MATH-0][MATH] and the [existing S1.3 task][S13], with [S1.2 regression][S12],
[engine acquisition][ENGINE] and [catalogue][CAT] owners. These refinements add
no parallel plan or acceptance suite. Canonical CHECK-6/7/8/9/14/25/27 and the
reached owner's remaining contracts retain authority.

### 8.1 MATH-W01: freeze a comparison that can answer the question

Identify the released NevroTick estimator and the roadmap's exact Chrony reference;
the paper's Chrony 4.5 result is a separate historical comparison. Freeze units,
signs, seeds, sample counts, duration, initialization, warm-up, parameter-selection
method, finite resources, success/refutation criteria and stop conditions.

Replay identical observations for estimator-only comparisons. For feedback
convergence, use the same independent oscillator/actuator/network scenarios and
seeds; observations may differ causally because controllers change the clock.
The controller must not generate its own expected truth. Hold roster and
polling constant while attributing estimator benefit, then separately test
query-budget tradeoffs. Compare ordinary filtering, fixed-noise Kalman,
adaptive noise and reselection sufficiently separately to identify their effects.

Report signed/absolute offset, frequency error, tails, convergence/reacquisition,
holdover and uncertainty containment alongside width. Define accumulated metrics
with exact denominators. Separate startup, steady state, per-peer and aggregate
traffic, KE/rekey, churn and computation. A smaller residual or smoother curve is
not the success criterion by itself.

### 8.2 MATH-W04: physical ambiguity and uncertainty

Include the two identical-observation worlds above, genuine offset on symmetric
paths, unequal delivery with equal clocks, constant directional delay, shared
delay across authenticated peers, slow ramps and benign route changes. Each has
independent clock/delay truth and a healthy neighboring control. Large raw phase
must not automatically become proof of server misconduct or independent
asymmetry evidence.

Keep numerical rounding enclosure, model covariance and physical UTC uncertainty
separate. Declare server/capture/rate-mapping assumptions and challenge coverage,
including common operator/upstream dependence. More endpoints must not create
fictitious independent confidence. A candidate may honestly refuse an ambiguous
case; it must not report unwarranted accuracy.

### 8.3 MATH-W05: detector behavior, corrections and recovery

Specify `Q/R/P`, actual elapsed `dt`, gain components, innovation normalization,
thresholds, persistence, hysteresis, reset/re-entry and covariance initialization
before execution. Observe gain and innovation alongside independent error. Pair
malicious delays with legitimate clock corrections and route changes that produce
similar residuals; measure false alarms, missed/late detection and switching cost.

Exercise irregular/missing observations, holdover, suspend/discontinuity and
recovery after source replacement. Account for acknowledged achieved corrections,
including partial delivery, refusal and unknown outcome, using the existing
coordinate contract. Requested corrections cannot rewrite history. Reprojection,
old samples and reselection cannot refresh observation age, mint votes or
replenish correction authority. Include recovery within allowed bounds as well
as honest unavailability outside them.

### 8.4 Source selection, catalogue and MATH-C01

At actual future consumers, retain these cases under catalogue DATA-01/03/04/10/
12/16/17 and the existing selection/acquisition owners:

| Case | Independent expected result |
| --- | --- |
| Excellent score but failed identity, authentication or time-scale admission | Source stays ineligible; a valid slower source remains usable under existing agreement rules. |
| Several endpoints sharing one known operator/upstream | Measurements remain distinct; they gain no unsupported independent voting credit. |
| Successful KE followed by failed protected time delivery | Stage counters remain distinct; no time observation or freshness is fabricated. |
| Failure, cooldown and later healthy recovery | Bounded reprobe permits justified re-entry without storms, permanent starvation or relaxed admission. |
| Source replacement during disagreement | Configured minima, majority, generations and observation identity remain intact. |

Prepare cases before V1 where OD-18 calls for it; execute actual reached V1
behavior under its existing gates. New estimator experiments remain after actual
V1 release. At **MATH-C01**, record `ADOPT_LIMITED`, `DEFER` or `REJECT` with
evidence, limits and a return trigger. Missing original artifacts can limit
reproduction without blocking an independently specified comparison. Production
adoption is not required to advance the sequence:

**actual V1 release → MATH-C01 → MT-C01 → KHR-C01 → GEO-C01 → Android.**

Existing formal/native/fuzz/storage/UI holds and the separate RGT-0 → CHR-0
direction remain. This documentation reports no new product test pass, measured
NevroTick improvement, live scanner, roster approval or clock authority.

## 9. Public primary references and maintained owners

[P]: https://doi.org/10.1007/978-3-032-22500-9_18
[NTP]: https://www.rfc-editor.org/rfc/rfc5905.html
[NTS]: https://www.rfc-editor.org/rfc/rfc8915.html
[CHR45]: https://chrony-project.org/doc/4.5/chrony.conf.html
[ARCH]: ../windows-time-architecture.md#72-sample-to-clock-pipeline
[LIT]: ../../tasks/engine-transport-pilot.md#integrated-lit-0-and-receiver-checkpoint--13-november-2026
[OD18]: ../nevrotick-owner-decisions.md#22-od-18--literature-derived-qualification-before-v1
[MATH]: ../../ROADMAP.md#math-0--post-v1-mathematical-estimation-experiments
[S13]: ../../tasks/s1.3-source-selection-readiness.md
[S12]: ../../tasks/s1.2-regression-readiness.md
[ENGINE]: ../../tasks/engine-transport-pilot.md
[CAT]: ../../tasks/server-catalogue-and-statistics.md

The paper locations above identify the reviewed claims. RFC 5905 supplies the
established NTP mechanisms; RFC 8915 supplies the NTS security/version boundary;
the versioned Chrony manual supplies the evaluated baseline's documented
capabilities. Repository links route implementation and evidence to their
existing owners. Current contracts and candidate-specific receipts govern any
later continuation; this supporting review does not replace them.
