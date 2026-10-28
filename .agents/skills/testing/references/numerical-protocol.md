# Independent numerical, protocol and contract truth

Read the canonical [architecture](../../../../docs/windows-time-architecture.md), sections 6–10 and 20, for actual constants and invariants. CHECK IDs below route to planned product evidence, not existing test names.

## Numerical truth and control trajectories

Build expectations from a known UTC trajectory, oscillator error, delay/asymmetry, loss, suspend and source faults. Production code must not compute its own expected results. The architecture's analytic example uses millisecond-relative `T1=0, T2=70, T3=80, T4=110`: expected offset **+20 ms** and round-trip network delay **100 ms**. Positive offset means Windows is behind and needs to advance. Verify this sign through conversion and actual Windows actuation separately.

Compare selection/principal/supporter/rejected membership and reasons, uncertainty at a common instant, poll changes, age and the full correction trajectory. An authenticated source can still provide wrong time; asymmetric network delay still biases estimates. Cover one/two falsetickers, two-source disagreement, duplicate logical identities and configured minima without silent threshold reduction (CHECK-7/8).

Keep exact NTP timestamp differences and Windows conversions in checked integer/fixed-point arithmetic before deliberately using floating point for small deltas. Declare physical error envelopes and numerical tolerances before execution. Test NaN/infinity, cancellation, saturation/overflow boundaries, quantization and signed rounding independently. Differential comparison with pinned chrony is useful but cannot be the only oracle for a ported algorithm. Record upstream revision/port-ledger identity.

Maintain independent UTC, monotonic and active-awake clocks. A wall-clock step does not extend a monotonic authorization deadline; suspend time is not delivered active slew. Configuration, association and clock generations invalidate late responses and proposals. Model actual acknowledgements, lost acknowledgements and ambiguous effects; retry must not duplicate a step or reset cumulative budgets (CHECK-10–12).

## NTP and NTS boundaries

Use independent exact byte/crypto vectors and controlled peer fixtures. Exercise short/truncated frames, length overflow, invalid extension alignment, mode/version, origin/UID/association mismatch, replay, duplicate replies, NAK storms and cookie/key handling (CHECK-6). Assert invalid packets have no sample, cookie or clock effect and a valid neighbor is admitted exactly once.

NTS recovery tests cover dates far in both directions, ordinary/provisional verifier entry, identity mismatch, service-account trust stores, certificate validity/revocation uncertainty, offline roots and bounded retries. Bootstrap-only plain NTP evidence never becomes permission for plain-clock actuation or silent NTS downgrade. Include expired compromised-key scenarios and the residual risk the architecture records (CHECK-9/27). A browser/Linux TLS handshake cannot qualify the selected Windows verifier.

No routine test should depend on public NTP/NTS availability. A live interoperability run is a distinct, authorized evidence layer with named endpoints and no uncontrolled privileged actuation. A simulator/loopback fixture does not claim Internet accuracy.

## Strict JSON and domain constructors

Follow CHECK-25 through the actual bounded raw bytes → map-only wire DTO → fallible private domain value → current guard → serialized effect → acknowledgement path. Do not normalize into `serde_json::Value` before testing duplicate-key rejection. Schema success and private types cannot prove current permission, truth or freshness.

Required categories include nested unknown/duplicate/escaped-equivalent keys; positional arrays substituted for objects; missing versus null versus invalid supplied fields; supported and future versions; truncation/UTF-8; a trailing second document/scalar/garbage; exact permitted whitespace; and every builder/migration/deserialization bypass.

Test fixed-width integer spellings/ranges independently: neighbors around 2^53, type endpoints, overflow, fractions/exponents/strings and `-0` under the selected wire contract. Cross-field policy limits are narrower than primitive decoding. Test finite computed values after arithmetic/FFI, not merely legal JSON syntax. Complete-consumption checks must survive path/error adapters.

Bound reads before allocation, collections during decoding, depth, escaped strings, numeric tokens, queue/work and redacted diagnostics. A post-parse length check alone does not establish an allocation limit. Preserve the architecture's limits; do not invent a serde_json depth setter. Include legitimate maximum-size controls and measure allocation/work where the claim requires it.

Malformed, valid-shaped unauthorized, stale, replayed and overbudget requests must have no prohibited clock/policy/secret/network/installer effect. Failed policy updates preserve the previous revision; invalid mandatory startup authority blocks effects. Error output uses bounded allowlisted field paths/codes, without raw values, attacker-chosen paths, keys or cookies. Pure mocks cover admission only; real Windows enforcement stays in CHECK-2/3/10.
