# Hifitime recommendations: critical disposition

**Reviewed:** 5 November 2026, Asia/Tbilisi. Source review and documentation
integration; no Hifitime execution, solver campaign or product repair.

The supplied “Hifitime → NevroTick: independently reviewed recommendations”
compares Hifitime `ede8b357640adb5449c03f87e6b5c4e6d52b5f06` with NevroTick
`8b8d0142ab1daae50509b6ec2cea800ada033449`. Intake SHA256:
`2edba9f654743141bd711250f8bc9c1d0d7506cf992909a7f479551fae2bef60`.
This disposition rechecks the important upstream bodies and the current working
tree based on `8a66af924c509e898a9bab9bb3282f11b24fba93`, including concurrent
uncommitted proof records. Those records supply attributed results; this review
does not independently certify or rerun their retained campaigns.

## Verdict and corrections

The recommendation to transfer verification lessons while retaining the existing
numerical design is supported. No missing production Core algorithm or NevroTick
arithmetic defect was established by this comparison. That bounded finding is
not a whole-Core audit or a claim that all existing mathematics is proved.

| Source observation or inference checked | Critical disposition |
| --- | --- |
| A range-only postcondition can certify an incorrect answer. | **Supported.** The pinned `Duration::signum` uses the century component's sign. A normalized positive one-nanosecond value has zero centuries; the postcondition admits its incorrect zero. This is a source-derived counterexample, without a Rust reproduction here. [Duration body](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/src/duration/mod.rs), [harness](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/src/duration/kani_verif.rs). |
| The named Epoch equality/order proof reaches the production traits. | **It does not; the report correctly identifies the gap.** `verify_epoch_eq_ord_consistent` compares extracted integer totals without constructing an Epoch. It proves no consistency property of the actual Epoch trait bodies. This is a harness-scope finding, not a demonstrated Epoch defect. The Duration harness does reach overloaded comparisons. [Epoch harness](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/src/epoch/kani_verif.rs), [traits](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/src/epoch/ops.rs), [Duration harness](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/src/duration/kani_verif.rs). |
| Pairwise matching establishes complete leap-data freshness. | **It does not; preserve NevroTick's stronger checks.** The upstream zipped comparator omits length equality, and the file parser skips comment metadata. Matching prefixes or appended rows can escape that comparator. Its mixed forward/reverse iterator also shares one position counter, allowing repeats/skips. These observations do not establish a bad live response or a reverse-only lookup failure. [Bundled provider](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/src/epoch/leap_seconds.rs), [file provider](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/src/epoch/leap_seconds_file.rs). |
| Similarly named rounding/calendar operations are interchangeable. | **Reject interchangeability.** Signed truncating remainder and the upstream exact-multiple `ceil` policy differ from NevroTick's explicit rounding contract. Civil-date decomposition is fixed-work, but the upstream Gregorian constructor traverses years. Neither supplies a replacement contract or performance result. [Duration](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/src/duration/mod.rs), [Gregorian conversion](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/src/epoch/gregorian.rs). |
| Independent anchors add information beyond round trips. | **Supported.** Selected upstream TCG/TCB tests combine SOFA-derived comparisons with reciprocity. The pinned J1900/J2000 constants encode TT while adjacent comments say TAI. This supports checking the actual epoch/unit/scale against independent facts; it supplies no NTP conversion theorem. [Tests](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/tests/epoch.rs), [constants](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/src/timescale/mod.rs). |
| The presence of proof CI establishes executed mathematical acceptance. | **Reject that inference.** The push/PR lane uses `--only-codegen`; the scheduled/manual lane selects proof groups. Configuration demonstrates neither current successful execution nor complete selected scope. Its positional manifest edit also targets the `rlib` line at this pin, rather than the separately declared `cdylib`. No resulting build failure was reproduced. [Build workflow](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/.github/workflows/kani-builder.yml), [scheduled workflow](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/.github/workflows/weekly-kani-checker.yaml), [manifest](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/Cargo.toml). |

The report's prepared-versus-completed proof inventory is historical. Current
[S1.1c](../../tasks/s1.1-time-domain.md#hifitime-recommendation-reconciliation--5-november-2026)
and [S1.4](../../tasks/s1.4-leap-policy.md#hifitime-recommendation-reconciliation--5-november-2026)
records now report qualified scalar, calendar-helper and coverage-prefix leaves,
with explicit assumptions and trusted correspondences. Reuse them while their
bound inputs remain valid or an explicit reviewed irrelevance argument covers a
change; do not requeue them from the older report. The proof-workflow document
changed here is itself a hashed runner input: the
[S1.1c custody note](../../tasks/s1.1-time-domain.md#hifitime-recommendation-reconciliation--5-november-2026)
requires reconciliation rather than silently relabelling historical receipts.
The actual
public `EraWindow::new`/Garde proof remains incomplete after a reproduced verifier
model defect. Its existing isolated repair checkpoint owns the continuation.
Ledger admission/traversal, final leap interpretation and remaining Core
mathematics retain their separate obligations.

## Adoption in existing owners

The [development plan §4.3](../plans/development-plan.md#43-apply-the-hifitime-verification-lessons)
routes these refinements under existing CHECK identities and the proof-first
order in §7. [OD-10](../nevrotick-owner-decisions.md#14-od-10--mandatory-mathematical-proofs)
already requires the full source-bound mathematical contract; these findings
clarify its application rather than create another proof framework.

| Recommendation | Disposition, owner and observable result |
| --- | --- |
| R1 — Semantic and reachability review | **Apply to pending proofs.** S1.1c, S1.4 and S1.2/S1.3 map the theorem to required answers/errors and the actual operation. Range/no-panic facts alone cannot establish the main result. Meaningful source defects must reach the intended assertion, followed by restored health. CHECK-7/8/25; correction CHECK-10 remains with its owner. |
| R2 — Constructor-to-consumer composition | **Existing required continuation.** S1.1c preserves the valid-window resolver theorem and separately closes raw public admission, retention/rejection and caller-established preconditions. Use the existing verifier-repair checkpoint; prepared/code-generated models supply no constructor proof. No numerical theorem authenticates an external era anchor. |
| R3 — Independent conversion answers | **Retain and refine at affected conversions.** S1.1c/S1.4 and later platform owners pair round trips with independently derived epoch/unit/scale anchors, exact signed boundaries and declared residual/error rules. Existing independent literals/oracles remain reusable. No Hifitime/SOFARS dependency is selected. |
| R4 — Real semantic operations and sequences | **Conditional on actual custom semantics.** Reach custom equality/order/hash/conversion/iterator bodies and exercise relevant operation sequences. Derived coordinate/clock comparisons and immutable slice scans require no speculative replacement API or duplicate campaign. |
| R5 — Proof automation accounting | **Apply at the next verification integration checkpoint.** The existing proof owner reconciles expected, selected and completed harnesses and retains codegen, inspection, solver, sensitivity and restored results separately. Preserve the admitted profiles, limits and full incomplete-result accounting; no copied upstream workflow or new CI gate is adopted. |
| Complete leap-data comparison | **Already implemented; retain.** S1.4/LP-C02 keeps full raw-byte/hash/length/record/metadata admission and full generated-output comparison in the [offline generator](../../tools/leap-data/generate.py). Its [process test](../../tools/leap-data/test_generate.py) already rejects an unexpected suffix and preserves the previous output. Add tests only for a demonstrated uncovered behavior. CHECK-15/23/25, with CHECK-26 provenance distinct. |
| Calendar month recognition | **Already implemented and now recorded qualified within its helper scope.** Retain the existing constant-work helper and independent seams. A similar upstream formula is no independent oracle. Ledger and interpretation obligations remain open under S1.4/LP-C01 and CHECK-7/25. |
| Integer display decomposition/narrow adapters | **Deferred to an actual consumer.** Freeze accepted range, sign, units, quantization/residual and output-width contract first. Decompose with integer quotient/remainder; avoid whole-duration floating conversion that erases neighbors. Future controls can include one source tick, signed ties, exact multiples, maximum range and 195 days minus one nanosecond. Preserve UI-M0 D1–D3 deferrals and native-effect gates. |

Keep exact Q32/Q33 values, checked destinations and explicit failures. No source
review justifies saturation at control boundaries, live leap retrieval,
last-known-offset extrapolation, astronomical/GNSS/UT1 scope or a new public
iterator. The upstream [manifest](https://github.com/nyx-space/hifitime/blob/ede8b357640adb5449c03f87e6b5c4e6d52b5f06/Cargo.toml)
identifies MPL-2.0 and an `lts` feature bringing `ut1`/network-related dependencies;
it is not a dependency-admission decision. No upstream code is copied by this
documentation integration; later source adaptation still needs exact derivation,
notices and dependency/licence qualification. No licence-compatibility conclusion
is made here.

## Reusable workflow changes and limits

The [proof reference](../../.agents/skills/testing/references/formal-verification.md)
now explicitly challenges codomain-only specifications, detached trait harnesses
and unaccounted proof selections. The
[numerical reference](../../.agents/skills/testing/references/numerical-protocol.md)
requires independent conversion anchors, and
[golden/parity review](../../.agents/skills/testing/qa-golden-fixture-parity/SKILL.md)
challenges complete-sequence comparisons with empty, prefix and appended cases.
These are focused refinements of existing obligations, without another skill or
an upstream-defect test campaign against unaffected NevroTick APIs.

Upstream source-derived examples warrant real public-API reproductions before
an upstream patch or defect report. This review neither establishes current
upstream CI success nor qualifies NevroTick's whole mathematical domain, physical
time authority, Windows integration or release. Runtime validation, behavioral,
coverage/mutation, native and package evidence retain their separate gates.
