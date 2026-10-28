# Independent numerical, protocol and contract truth

Read canonical [architecture](../../../../docs/windows-time-architecture.md), sections 6–10 and 20: actual constants/invariants. CHECK IDs route to planned evidence; no existing test-name claim.

## Numerical truth and control trajectories

Build expectations from a known UTC trajectory, oscillator error, delay/asymmetry, loss, suspend and source faults. Production code must not compute its own expected results. The architecture's analytic example uses millisecond-relative `T1=0, T2=70, T3=80, T4=110`: expected offset **+20 ms** and round-trip network delay **100 ms**. Positive offset means Windows is behind and needs to advance. Verify this sign through conversion and actual Windows actuation separately.

Compare selection/principal/supporter/rejected membership/reasons, common-instant uncertainty, poll changes, age, full correction trajectory. Authenticated sources can give wrong time; asymmetric delay biases estimates. Cover one/two falsetickers, two-source disagreement, duplicate logical identities, configured minima; no silent threshold reduction (CHECK-7/8).

Keep exact NTP timestamp differences and Windows conversions in checked integer/fixed-point arithmetic before deliberately using floating point for small deltas. Declare physical error envelopes and numerical tolerances before execution. Test NaN/infinity, cancellation, saturation/overflow boundaries, quantization and signed rounding independently. Differential comparison with pinned chrony is useful but cannot be the only oracle for a ported algorithm. Record upstream revision/port-ledger identity.

Maintain independent UTC/monotonic/active-awake clocks. Wall-clock step cannot extend monotonic authorization deadline; suspend time is not delivered active slew. Configuration/association/clock generations invalidate late responses/proposals. Model actual/lost acknowledgements/ambiguous effects; retries must not duplicate step/reset cumulative budgets (CHECK-10–12).

## NTP and NTS boundaries

Use independent exact byte/crypto vectors/controlled peer fixtures. Exercise short/truncated frames, length overflow, invalid extension alignment, mode/version, origin/UID/association mismatch, replay, duplicate replies, NAK storms, cookie/key handling (CHECK-6). Assert invalid packets cause no sample/cookie/clock effect; admit valid neighbor exactly once.

NTS recovery tests cover dates far in both directions, ordinary/provisional verifier entry, identity mismatch, service-account trust stores, certificate validity/revocation uncertainty, offline roots and bounded retries. Bootstrap-only plain NTP evidence never becomes permission for plain-clock actuation or silent NTS downgrade. Include expired compromised-key scenarios and the residual risk the architecture records (CHECK-9/27). A browser/Linux TLS handshake cannot qualify the selected Windows verifier.

Routine tests must not depend on public NTP/NTS availability. Live interoperability: distinct authorized evidence layer, named endpoints, no uncontrolled privileged actuation. Simulator/loopback fixture claims no Internet accuracy.

## Strict JSON and domain constructors

Follow CHECK-25 actual bounded raw bytes → map-only wire DTO → fallible private domain value → current guard → serialized effect → acknowledgement path. No `serde_json::Value` normalization before duplicate-key rejection tests. Schema success/private types prove no current permission/truth/freshness.

Required categories include nested unknown/duplicate/escaped-equivalent keys; positional arrays substituted for objects; missing versus null versus invalid supplied fields; supported and future versions; truncation/UTF-8; a trailing second document/scalar/garbage; exact permitted whitespace; and every builder/migration/deserialization bypass.

Test fixed-width integer spellings/ranges independently: neighbors around 2^53, type endpoints, overflow, fractions/exponents/strings and `-0` under the selected wire contract. Cross-field policy limits are narrower than primitive decoding. Test finite computed values after arithmetic/FFI, not merely legal JSON syntax. Complete-consumption checks must survive path/error adapters.

Bound pre-allocation reads, decoding-time collections, depth, escaped strings, numeric tokens, queue/work, redacted diagnostics. Post-parse length check alone proves no allocation limit. Preserve architecture limits; invent no serde_json depth setter. Include legitimate maximum-size controls; measure allocation/work where claim requires.

Malformed, valid-shaped unauthorized, stale, replayed and overbudget requests must have no prohibited clock/policy/secret/network/installer effect. Failed policy updates preserve the previous revision; invalid mandatory startup authority blocks effects. Error output uses bounded allowlisted field paths/codes, without raw values, attacker-chosen paths, keys or cookies. Pure mocks cover admission only; real Windows enforcement stays in CHECK-2/3/10.
