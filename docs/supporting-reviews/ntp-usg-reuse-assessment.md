# ntp_usg reuse assessment

**Reviewed and integrated: 11 November 2026.** This is a source comparison and
acceptance-design handoff, not a reproduced vulnerability report, benchmark run
or native Windows qualification.

| Evidence | Immutable revision |
| --- | --- |
| NevroTick comparison baseline | [`b8158df6e55c3bf3a58e7fbaac76ff34514a5019`](https://github.com/DavidOsipov/NevroTick/commit/b8158df6e55c3bf3a58e7fbaac76ff34514a5019) |
| NevroTick documentation integration base | [`2a3e59111ce402bca01e702b4b5080bbf5b2c35e`](https://github.com/DavidOsipov/NevroTick/commit/2a3e59111ce402bca01e702b4b5080bbf5b2c35e); intervening Rust/API-evidence guidance and strict-policy changes preserved |
| 192d-Wing/ntp_usg comparison | [`bfab924b9a7f550d46eae53c53d9b77fb046e815`](https://github.com/192d-Wing/ntp_usg/commit/bfab924b9a7f550d46eae53c53d9b77fb046e815); workspace version 5.0.0, not a claim about a published release |

## Reuse disposition

Use ntp_usg as a secondary source of test scenarios, integration comparisons and
benchmark structure. Retain OD-01's coherent Chrony-derived estimator/controller
and the existing ntpd-rs/NTPsec comparisons. The inspected material does not
establish an algorithm, accuracy or performance improvement over NevroTick's
selected pipeline. No crate, source/test body, controller constant, crypto
provider, runtime or platform is adopted by this review.

NevroTick already requires bounded NTS state, conservative uncertainty, current
observation identity and acknowledged correction accounting. Its
[combination owner](https://github.com/DavidOsipov/NevroTick/blob/b8158df6e55c3bf3a58e7fbaac76ff34514a5019/crates/time-core/src/combination.rs#L1045-L1167)
already retains within-source phase variance and between-source disagreement,
with distinct phase/rate/tracking weights. The additions below exercise those
properties at future consumers; they do not identify a missing combination
formula or reopen completed pure-kernel qualification. The actual product
integration and native evidence remain with their existing tasks.

## Useful patterns and their limits

| Pattern in the pinned donor | Useful adaptation for NevroTick |
| --- | --- |
| [`valid_nts_request_roundtrips_through_pipeline`](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-server/src/server_common/nts_pipeline.rs#L222-L289) composes a cookie, protected request, server handler, protected reply, replacement-cookie decoding and a wrong-direction-key negative. | Exercise the real S2.2 formatter/AEAD/association/receiver together after their prerequisites exist. Its keys are generated directly in the test: this pattern does not test TLS or NTS-KE. Shared request/reply helpers can share a defect, so retain independent vectors and separately qualified interoperability. |
| [`selection_benchmark.rs`](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/benches/selection_benchmark.rs) measures selection, clustering, combination and the full pipeline. | Measure actual NevroTick stages and useful integrated work under the admitted bounds, including normal, disagreement, stale and rejection paths. Record allocation count/bytes and elapsed time where relevant. No donor timing, population or benchmark framework becomes a product requirement. |
| [CI feature rows](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/.github/workflows/ci.yml) exercise explicit default, Tokio, NTS and clock configurations on Windows. | Preserve the exact package/features/target/profile behind each claim. Check the source path and executed assertions, including ignored tests and accepted-error outcomes. Reuse NevroTick's selected matrix; donor ARM, smol, NTPv5 and other features add no support commitment. |

The donor has implemented NTS and Windows adapters and a successful
[CI run for the inspected commit](https://github.com/192d-Wing/ntp_usg/actions/runs/37150392873).
It must not be described as lacking Windows tests. That workflow's Windows rows
enable `clock` but do not enable `discipline`; some
[clock tests](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/src/clock.rs#L515-L569)
tolerate API failure or are ignored. This is narrower evidence than achieved
clock corrections, service lifecycle or convergence. No donor tests were run
as part of this assessment.

## Counterexamples to preserve as independent cases

These observations concern the exact revision and paths linked below. They are
source-derived behavior or policy differences, with no measured exploitability,
resource exhaustion or physical clock-error claim.

### Poll direction and control-message correlation

The continuous client's [RATE branch](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/src/client.rs#L338-L354)
calls [`decrease_poll`](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/src/client_common.rs#L271-L298).
Above its floor, reducing the exponent shortens the interval: 64 seconds becomes
32 seconds. NevroTick's existing RATE contract requires the opposite direction.
Test actual earliest sends before/at/after the revised guard, including the floor,
cap and repeated controls; inspecting a field named "backoff" is insufficient.

The same client's [NTS receive path](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/src/client.rs#L504-L525)
can return `check_kod` before protected-response validation and the later origin
check. Its [header parser](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/src/request.rs#L196-L250)
checks source IP and header properties but has no outstanding UID input on that
branch. The [one-shot NTS helper](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/src/nts.rs#L316-L398)
has a separate origin/UID check; its coverage cannot qualify the continuous caller.

[RFC 8915 §5.7](https://www.rfc-editor.org/rfc/rfc8915.html#section-5.7) permits
unprotected KoD responses with the required request correlation. Missing AEAD is
therefore not itself the defect. Pair a legitimate correlated control with wrong
UID/origin/endpoint/socket, stale-generation, expired and replayed controls at
the actual scheduler. Only the admitted control changes bounded recovery/polling;
none supplies a time sample, accepted cookie or authenticated freshness. This
does not establish an unrestricted off-path attack against the donor.

### Cookie inventory across successful exchanges

The donor [request builder](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-proto/src/nts_common.rs#L187-L209)
defines seven placeholders and [adds all seven each time](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-proto/src/nts_common.rs#L294-L298).
The [client consumes one and appends replacements](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/src/client.rs#L467-L524),
while the [server can return one plus the placeholder count](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-server/src/nts_server_common.rs#L394-L404).
With eight replacements per successful exchange and no intervening reset/rekey,
the retained inventory increases by seven each time; the inspected append path
has no aggregate cap. A 2,048-byte receive buffer cannot bound that accumulation.
This is a conditional source-derived growth argument, not a measured OOM result.

RFC 8915 §5.7 recommends replenishing unused inventory to eight with **SHOULD**
strength. Freeze NevroTick's admitted inventory target and derive placeholders
from the current deficit and packet budget. Test many exchanges with varying
cookie sizes, fewer/excess replacements, loss, rejected replies and exhaustion;
measure both count and aggregate retained/staged/reserved bytes. The existing
local zero-replacement acceptance/completion/retry and reuse/privacy decisions
remain pending. Cover their eventual admitted outcomes without selecting them
through a fixture. Failed re-establishment retains only independently valid old
state and never permits ordinary plain fallback.

### Agreement, uncertainty and real control feedback

The donor's [system peer caller](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/src/client_common.rs#L565-L640)
filters stratum/root distance, then permits a single eligible peer after majority
selection fails. This is a policy difference, not proof that all single-source
clients are defective. NevroTick retains configured minimum agreement and no new
disputed correction after majority loss.

Trace observation identity, age and each uncertainty contribution through actual
projection, selection, combination and publication. Equal phase estimates with
positive individual uncertainty must not become zero total uncertainty merely
because their disagreement is zero. Retain separate path/server claims,
within-source error and between-source disagreement; presume no independence
benefit from source count. Repeated evaluation creates no fresh observation.

The donor [`test_ten_samples_converge`](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/tests/discipline_integration.rs#L111)
feeds decaying offsets and checks state/jitter; that alone does not demonstrate a
controller causing convergence. Its [`ClockAdjuster::tick`](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/src/clock_adjust.rs#L40-L60)
debits modeled residual before the [caller attempts the OS operation](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/src/client.rs#L230-L242).
The [Windows adapter](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/src/clock.rs#L301-L356)
converts its argument into a persistent rate using a 30-second horizon.
[Microsoft's API contract](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtimeadjustment)
defines periodic adjustment and the separate return to system-managed adjustment.
Do not equate a per-tick phase amount, requested rate and achieved correction, or
infer a universal numerical error factor from this source mismatch.

At the existing S1.4 integration, drive an independently specified oscillator/
actuator model with actual controller outputs. Quantization, partial delivery,
refusal and unknown outcomes must feed back truthfully; only actual known effects
remap history or settle debt. Pair isolated anomalies with fresh persistent
agreement. Keep native behavior with P4 and preserve the selected Chrony law.

## Rights and dependency boundary

The donor [workspace manifest](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/Cargo.toml#L20)
declares `Apache-2.0 OR MIT`, and its
[MIT notice](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/LICENSE-MIT)
exists. Relevant files, including
[selection](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/src/selection.rs#L1-L2),
carry `SPDX-License-Identifier: Apache-2.0`. Resolve the exact file-level grant
and provenance before copying code or tests; package metadata alone does not
settle that discrepancy. Do not declare every file MIT-cleared, exclusively
Apache-licensed or public domain from these observations. Apache's
[compatibility guidance](https://apache.org/licenses/GPL-compatibility.html)
distinguishes GPLv3 from GPLv2; a file available only under Apache-2.0 cannot be
assumed compatible with NevroTick's GPL-2.0-only Core/engine. The GUI's separate
GPL-3.0-only grant does not change Core provenance.

The donor [client features](https://github.com/192d-Wing/ntp_usg/blob/bfab924b9a7f550d46eae53c53d9b77fb046e815/crates/ntp_usg-client/Cargo.toml#L18-L51)
make `nts` and `nts-smol` include `pq-nts`, enabling rustls `aws-lc-rs` and
`prefer-post-quantum`; the workspace also declares `ring`. Any future reuse must
inspect the exact unified graph, selected provider and emitted/native artifacts.
This establishes no measured binary overhead or claim that every enabled
provider is linked. Existing NevroTick provider/licence decisions remain in force.

## Integration into existing work

| Existing owner | Added observation and activation point |
| --- | --- |
| [S0.1 protocol comparison](../../tasks/s0.1-protocol-comparison.md#ntp_usg-comparison-handoff--11-november-2026) | Preserve pinned source/caller evidence and RATE direction at the actual next-send guard. REQ-8/BND-1 for UDP; BND-2 for KE/association context; CHECK-6/8/9/17/27. |
| [S2.2 paired acceptance](../../tasks/engine-transport-pilot.md#paired-acceptance-evidence--planned-not-run) | Protected composition and repeated cookie/control sequences at the real association owner after provider/contract admission. AC-04; CHECK-6/8/9/17/21/25/26/27, plus CHECK-7 at the numerical consumer. |
| [Core integration refinement](../../tasks/s1.3-source-selection-readiness.md#ntp_usg-consumer-integration-refinement--11-november-2026) | Preserve identity/age/uncertainty to the future consumer; exercise independent controller-driven trajectories at the existing S1.4 parent. CHECK-7/8/10/11/14/25/27; AC-02/19. |
| [P4 clock integration](../../tasks/development-backlog.md#p4-clock-integration) | Qualify achieved effects, persistent-rate cancellation and post-step continuity at S4.1/S4.3/S4.5 on an authorized native fixture. Existing CHECK-10/11/12/15/21/25/27 owners retain their scopes. |
| [Existing performance qualification](../plans/development-plan.md#8-performance-limits-and-qualification-environments) | Select a small stage/pipeline cohort only for a concrete optimization question. Use actual admitted populations and refusal neighbors, exact hardening/features/target and independent numerical expectations. CHECK-7/21/25/26/27 as reached. |

This records recommendations in the existing work chain. It closes no CHECK,
changes no current product priority, admits no new dependency or numerical law,
and lifts no native/Sandbox, fuzz, storage or UI hold. Formal work remains
post-V1. The two reusable procedure refinements live in existing
[numerical/protocol QA](../../.agents/skills/testing/references/numerical-protocol.md)
and [Cargo evidence](../../.agents/skills/environment/dependency-updates/references/cargo-operations.md);
their group source records retain provenance without importing donor code.
