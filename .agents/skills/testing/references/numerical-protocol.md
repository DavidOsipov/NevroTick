# Independent numerical, protocol and contract truth

Read canonical [architecture](../../../../docs/windows-time-architecture.md), sections 6–10 and 20: actual constants/invariants. CHECK IDs route to planned evidence; no existing test-name claim.

## Numerical truth and control trajectories

Build expectations from a known UTC trajectory, oscillator error, delay/asymmetry, loss, suspend and source faults. Production code must not compute its own expected results. The architecture's analytic example uses millisecond-relative `T1=0, T2=70, T3=80, T4=110`: expected offset **+20 ms** and round-trip network delay **100 ms**. Positive offset means Windows is behind and needs to advance. Verify this sign through conversion and actual Windows actuation separately.

Compare selection/principal/supporter/rejected membership/reasons, common-instant uncertainty, poll changes, age, full correction trajectory. Authenticated sources can give wrong time; asymmetric delay biases estimates. Cover one/two falsetickers, two-source disagreement, duplicate logical identities, configured minima; no silent threshold reduction (CHECK-7/8).

Keep exact NTP timestamp differences and Windows conversions in checked integer/fixed-point arithmetic before deliberately using floating point for small deltas. Declare physical error envelopes and numerical tolerances before execution. Test NaN/infinity, cancellation, saturation/overflow boundaries, quantization and signed rounding independently. Differential comparison with pinned chrony is useful but cannot be the only oracle for a ported algorithm. Record upstream revision/port-ledger identity.

For conversions, pair reciprocity with independent known-answer anchors for the
actual epoch, unit, scale and coverage interval. Shared inverse mistakes can
round-trip. A related upstream formula is supplemental evidence, not an
independent oracle for its translation. At affected rounding/narrowing boundaries,
include both signs, zero, exact multiples, tie parities and destination neighbors
as applicable; preserve the declared residual rather than silently saturating.

When translating integer logic into a model or proof, reconcile
[specification and machine semantics](formal-verification.md#relate-integer-specifications-to-machine-operations),
especially negative division/remainder, casts and required-success overflow paths.
Product mathematics follows [OD-10’s first-release deferral](../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026):
formal proof work resumes after V1 and missing proofs are no first-release gate.
Continue independent numerical/behavioral oracles and runtime checks; retain
exact historical proof scopes and post-V1 obligations in the existing tasks.
The [proof workflow](formal-verification.md) applies on resumption, including
pending proof admission/metadata repairs.

For a numerical kernel or threshold, use the
[numerical procedure](../../implementation/implement-rust/references/numerical-kernels.md).
Retain separate exact mathematical expectations and independently modeled rounded
controls; freeze actual input bits and operation semantics where admission depends
on them. A computed cutoff can accept a value whose exact-real counterpart is
below the boundary. Cover conditioning/range limits and required algebraic
identities, with output tolerances/units declared before execution. Reach the
actual constructor/consumer: nonfinite-input rejection proves no arithmetic
overflow, and finite output proves no accuracy. Test only failures reachable
under that interface's admitted inputs; record why any proposed failure is
excluded. Constant, subnormal, signed-zero and reduction-order rules come from
the specific API contract, not a universal test recipe.

Maintain independent UTC/monotonic/active-awake clocks. Wall-clock step cannot extend monotonic authorization deadline; suspend time is not delivered active slew. Configuration/association/clock generations invalidate late responses/proposals. Model actual/lost acknowledgements/ambiguous effects; retries must not duplicate step/reset cumulative budgets (CHECK-10–12).

For a convergence claim, drive the independent oscillator/actuator model with
actual controller outputs and achieved effects. Prerecorded decreasing offsets
test response to those inputs; they do not establish feedback convergence. Use
the existing [physical-effect recipe](../../implementation/implement-rust/references/chrony-port-simulator.md)
and distinguish requested, quantized, partially delivered, refused and unknown
effects. Select no new plant, law or tuning constant from a donor test.

For source/controller integration, include these independent paired trajectories:

- Stable coordinates versus correctly remapped history for the same physical
  trace after an achieved correction; refusal, lost acknowledgement and unknown
  external steps have distinct effects. A proposed correction cannot change history.
- One good observation followed by silence and repeated selection: age and modeled
  uncertainty grow, quality expires under the frozen policy, and no new vote appears.
- Agreeing phase estimates with positive individual uncertainty versus disagreeing
  estimates: trace admitted uncertainty terms through the actual selection,
  combination and publication consumers. Zero disagreement cannot erase required
  within-source or other uncertainty; assume no source-independence benefit.
- Stable drift/noise versus a real drift change; a weak new fit must not destroy a
  sound prior, and temporary phase correction must end without contaminating drift.
- One anomaly versus fresh sustained agreement under the existing recovery bounds;
  elapsed time or repeated old samples cannot independently authorize correction.
- A permanent delay increase still inside the declared operating envelope plus
  transient spikes, paired with a path beyond hard limits. Observe bounded
  relearning/reacquisition or honest unavailability without loosening those limits.

Use the [port recipe](../../implementation/implement-rust/references/chrony-port-simulator.md)
for exact upstream callers. These scenarios add no constants or unconditional
accuracy/convergence guarantee; record per-event outputs and physical assumptions.

## Leap knowledge and event boundaries

When the slice reaches leap or time-scale policy, use
[architecture §10.5](../../../../docs/windows-time-architecture.md#105-leap-seconds-and-time-scales)
and [NV-LP-001](../../../../docs/plans/leap-second-policy.md). Keep independent
history/coverage truth, provider scale, Windows behavior and observation age distinct.
Pair these cases under S1.4/P2 and CHECK-6/7/8/11/14/25/26:

- All unsmeared sources send LI=0 with a locally known event, with no event and
  with unknown future coverage. Silence grants no calendar authority; unknown
  coverage alone permits same-segment tracking outside possible-event guards.
  Include non-June/December month-ends; covered continuous UTC needs no recurring
  legacy guard.
- Expired or absent expiry metadata preserves valid history without inventing
  unlimited future coverage; malformed replacement leaves the admitted dataset
  intact. Wide file dates do not wrap like packet seconds; the effective
  continuous-UTC boundary causes no clock impulse or universal 37-second mapping.
- One contradictory observation leaves healthy peers eligible. Authentication
  does not prove scale; smeared/unknown-scale inputs cannot join unsmeared UTC.
- Full T1–T4 spans crossing a guard, delayed completions, stale generations and
  repeated LI cannot admit corrections or extend guard authority. Reacquisition
  requires fresh post-guard exchanges and the existing independent quorum.

Separately qualify CHECK-10/11/12/27 on Windows: the kernel alone executes explicit
leaps, and recovery corrects measured residual only. Test OS-handled, missing and
unexpected events, both signs and approximately zero residual without a duplicate
second. Simulated success proves no native presentation, cancellation or accuracy.

## NTP and NTS boundaries

Use independent exact byte/crypto vectors/controlled peer fixtures. Exercise short/truncated frames, length overflow, invalid extension alignment, mode/version, origin/UID/association mismatch, replay, duplicate replies, NAK storms, cookie/key handling (CHECK-6). Assert invalid packets cause no sample/cookie/clock effect; admit valid neighbor exactly once.

NTS recovery tests cover dates far in both directions, ordinary/provisional verifier entry, identity mismatch, service-account trust stores, certificate validity/revocation uncertainty, offline roots and bounded retries. Bootstrap-only plain NTP evidence never becomes permission for plain-clock actuation or silent NTS downgrade. Include expired compromised-key scenarios and the residual risk the architecture records (CHECK-9/27). A browser/Linux TLS handshake cannot qualify the selected Windows verifier.

Routine tests must not depend on public NTP/NTS availability. Live interoperability: distinct authorized evidence layer, named endpoints, no uncontrolled privileged actuation. Simulator/loopback fixture claims no Internet accuracy.

For S2.1 port policy, observe concurrent associations, closure/replacement, delayed
replies and bounded handles under architecture §7.3. Keep the real Windows socket
check separate from pure exchange fixtures (CHECK-6/8/27).

Pair an admitted correlated NTS KoD/NAK with missing/mismatched UID, stale/replayed
control packets and a healthy protected reply. Observe only the permitted bounded
control/recovery effects: no sample, cookie acceptance, observation/authentication
freshness refresh or plain fallback. Apply §7.4's control exception explicitly; a blanket AEAD prerequisite
would reject this legitimate path (CHECK-6/9/27).
Observe the actual next permitted send after RATE, including policy limit cases;
exercise continuous-poll and recovery callers as reached. A correct one-shot
helper does not qualify another caller's control branch.

Qualify every split point of representative valid KE messages and coalesced
records, required/unknown record cases and valid nonce/padding/cookie alternatives.
Assert no partial association publication; a failed replacement retains only
still-valid old state. Exhaust cookies, cancel/fail KE and restore incomplete state
while observing the actual sender: normal NTS mode must never emit plain fallback.
Vary the wire token with physical T1–T4 fixed; arithmetic must remain unchanged.

Exercise many protected exchanges through the actual association owner, including
loss, fewer/excess replacements and exhaustion. Bound both cookie count and
aggregate available/reserved/staged bytes; derive placeholders from the admitted
unused-inventory deficit and packet budget. Per-datagram bounds cannot cap
accumulated state. Freeze zero-replacement completion/retry and cookie reuse/privacy
policy before asserting those outcomes; the existing S2.2 decisions remain open.
Rejected replies publish no replacement state, and a useful valid neighbor still
progresses within the admitted budgets (CHECK-6/9/21/27).

Flap connectivity and issue repeated refresh/NAK events while KE succeeds but
protected UDP fails. Check per-operation earliest times, aggregate/per-source
budgets, coalescing and obsolete-generation suppression. Retry reset must belong
to successful protected use of the same KE transaction. Mix invalid traffic with
valid neighbors and inspect bounded counters, redacted examples/summary costs and
continued valid progress within the predeclared load envelope (CHECK-6/9/17/21/27).
For persistence, use the architecture's
[restoration matrix](../../../../docs/windows-time-architecture.md#1211-restoration-validity)
with both warm-restart and discontinuity/corruption controls (CHECK-9/11/15/25).

## Strict JSON and domain constructors

Follow CHECK-25 actual bounded raw bytes → map-only wire DTO → fallible private domain value → current guard → serialized effect → acknowledgement path. No `serde_json::Value` normalization before duplicate-key rejection tests. Schema success/private types prove no current permission/truth/freshness.

Required categories include nested unknown/duplicate/escaped-equivalent keys; positional arrays substituted for objects; missing versus null versus invalid supplied fields; supported and future versions; truncation/UTF-8; a trailing second document/scalar/garbage; exact permitted whitespace; and every builder/migration/deserialization bypass.

Test fixed-width integer spellings/ranges independently: neighbors around 2^53, type endpoints, overflow, fractions/exponents/strings and `-0` under the selected wire contract. Cross-field policy limits are narrower than primitive decoding. Test finite computed values after arithmetic/FFI, not merely legal JSON syntax. Complete-consumption checks must survive path/error adapters.

Bound pre-allocation reads, decoding-time collections, depth, escaped strings, numeric tokens, queue/work, redacted diagnostics. Post-parse length check alone proves no allocation limit. Preserve architecture limits; invent no serde_json depth setter. Include legitimate maximum-size controls; measure allocation/work where claim requires.

Malformed, valid-shaped unauthorized, stale, replayed and overbudget requests must have no prohibited clock/policy/secret/network/installer effect. Failed policy updates preserve the previous revision; invalid mandatory startup authority blocks effects. Error output uses bounded allowlisted field paths/codes, without raw values, attacker-chosen paths, keys or cookies. Pure mocks cover admission only; real Windows enforcement stays in CHECK-2/3/10.
