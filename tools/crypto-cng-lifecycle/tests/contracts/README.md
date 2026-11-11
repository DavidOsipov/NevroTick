# Engine-owned KE compile contracts

These twelve explicit compiler fixtures belong to OPEN-8/R-05 and
CHECK-8/9/17/25/26/27 under the existing S2.2/EP1 task. They are nested below
`tests/` so Cargo does not automatically discover them as integration test
targets. An admitted compiler runner must select them explicitly.

**11 November EP1 source preparation:** the maintained producer moved into
Engine's private `nts_ke_experiment` module. These fixtures now consume the
actual `nevrotick-engine` library dependency, not the removed tool module or a
copied source tree. The tool's manifest selects `ke-association-prototype`;
the Engine workspace default does not. No copied types, visibility shim,
replacement implementation or provider substitute is used.

Select the actual admitted Windows target and exact feature graph. Every fixture
keeps `nevrotick_engine::experimental_nts_ke_probe(std::env::args_os())` reachable
for type-checking. **Compile/check only; never execute these fixtures.** They are
not runnable test scenarios. A forced Windows configuration on a host build is
not a substitute for the selected target and dependencies. On a non-Windows
ordinary library build the private module is absent; that missing-module refusal
cannot establish its Windows privacy contract.

First require `nts-ke-legal.rs` to compile successfully on the same candidate,
compiler, target and graph as the negative fixtures. Then require each negative
to fail with `E0603` naming the actual private `nts_ke_experiment` module at the
external library access. Each fixture retains its distinct attempted bypass:

| Fixture | Attempted bypass | Possible additional diagnostics |
| --- | --- | --- |
| `nts-ke-config-new.rs` | Construct the private configuration | Private `Config` or `new` |
| `nts-ke-config-literal.rs` | Construct configuration from raw fields | Private `Config` or its raw fields |
| `nts-ke-config-mutate.rs` | Mutate the checked peer | Private `Config` or `peer` |
| `nts-ke-control-new.rs` | Construct the private publication control | Private `Control` or `new` |
| `nts-ke-control-mutate.rs` | Renew the original expiry | Private `Control` or `expiry` |
| `nts-ke-candidate-access.rs` | Mutate the actual transport candidate | Private `transport`, `InternalCandidate` or `aead` |
| `nts-ke-directional-keys-access.rs` | Expose the actual C2S and S2C fields | Private `request`, `DirectionalKeys`, `c2s_key` or `s2c_key` |
| `nts-ke-handoff-access.rs` | Rebind a prepared request's original control | Private `handoff`, `PreparedRequest` or `control` |
| `nts-ke-ledger-ticket-forge.rs` | Forge the actual same-owner ticket | Private `association`, `ledger`, `Ticket`, `control` or `serial` |
| `nts-ke-ledger-mutate.rs` | Reset original budgets or release occupied custody | Private `LedgerState`, `attempts`, `frames`, `bytes` or `active` |
| `nts-ke-response-forge.rs` | Construct staged output without actual AEAD admission | Private `response`, `StagedReply` or its fields |

Match the actual item, source span and diagnostic meaning; a nonzero compiler
exit is insufficient. Missing imports/dependencies, an absent Windows-only
entrypoint, syntax failures or unrelated lint errors do not establish privacy.
Retain the diagnostics actually emitted. The outer privacy barrier may prevent
independent field/method diagnostics; do not claim those additional restrictions
from `E0603` alone.

The three EP2 additions and expanded S2C attempt are source specimens awaiting a
fresh admitted compiler run. The historical nine-fixture result below does not
qualify them. Keep the same real legal bridge control and exact outer-module
`E0603`; infer no non-Clone, lifetime, return-state or deeper-field pass from it.

No legitimate external caller can obtain these private configuration, control or
candidate values. These fixtures therefore establish blocked external access,
not publicly reachable move/borrow or consume-once behavior. Existing internal
deadline, cancellation, repeated-commit and concurrent-winner tests own the
control semantics; candidate binding and `finish(self)` require their own source
and semantic evidence. Never widen production visibility to manufacture a
consumer for those claims.

The [current EP1 task](../../../../tasks/engine-transport-pilot.md#ep1-graph-admission-and-first-compile--11-november-2026)
records admitted graph and fresh Windows-target compiler results for the
migrated candidate: one legal caller succeeds and all eight negatives fail with
the intended outer-module `E0603`. No fixture was executed. These results prove
neither deeper field/method restrictions nor native behavior. The
[existing S2.2 task](../../../../tasks/s0.1-crypto-feasibility.md#primary-tls-error-repair-and-external-consumers)
records the historical seven-consumer compiler checkpoint and actual diagnostics;
the later handoff checkpoint records the expanded nine-consumer scope. Those
results apply to their original source placement and are not a pass for this
library migration. The current EP1 task owns the new candidate and remaining
required checks. This supplies no certificate/TLS/
exporter success, Windows cleanup/resource qualification or production
association authority.
Native execution, signing and post-V1 proof holds remain unchanged.
