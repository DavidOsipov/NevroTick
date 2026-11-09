# Laboratory KE compile contracts

These seven explicit compiler fixtures belong to OPEN-8/R-05 and
CHECK-8/9/17/25/26/27 under the existing S2.2 laboratory producer task. They are
nested below `tests/` so Cargo does not automatically discover them as integration
test targets. An admitted compiler runner must select them explicitly.

Every fixture imports the actual `../../src/bin/nts_ke/mod.rs` by source path.
Its child modules retain their original paths and visibility. No copied types,
replacement implementation, visibility shim or provider substitute is used. This
checks an external caller of the frozen module sources in the CLI's crate-root
placement; it does not introduce or qualify a public library API.

Select the actual admitted Windows target and exact dependency graph. Each
fixture retains `nts_ke::entry(std::env::args_os())`, keeping the real producer
call graph reachable for checking. **Compile/check only; never execute these
fixtures.** They are not runnable test scenarios. A forced Windows configuration
on a host build is not a substitute for the selected target and dependencies.

First require `nts-ke-legal.rs` to compile successfully on the same candidate,
compiler, target and graph as the negative fixtures. Then require each negative
to fail for the named actual item's privacy rule:

| Fixture | Required privacy meaning | Possible additional diagnostics |
| --- | --- | --- |
| `nts-ke-config-new.rs` | `E0603`: `Config` is private | `E0624`: `new` is private |
| `nts-ke-config-literal.rs` | `E0603`: `Config` is private | `E0451`: its raw fields are private |
| `nts-ke-config-mutate.rs` | `E0603`: `Config` is private | `E0616`: `peer` is private |
| `nts-ke-control-new.rs` | `E0603`: `Control` is private | `E0624`: `new` is private |
| `nts-ke-control-mutate.rs` | `E0603`: `Control` is private | `E0616`: `expiry` is private |
| `nts-ke-candidate-access.rs` | `E0603`: `transport` is private | Private `InternalCandidate` or `aead` diagnostics |

Match the actual item, source span and diagnostic meaning; a nonzero compiler
exit is insufficient. Missing imports/dependencies, an absent Windows-only
entrypoint, syntax failures or unrelated lint errors do not establish privacy.
Retain the diagnostics actually emitted. The outer privacy barrier may prevent
independent field/method diagnostics; do not claim those additional restrictions
from `E0603` alone.

No legitimate external caller can obtain these private configuration, control or
candidate values. These fixtures therefore establish blocked external access,
not publicly reachable move/borrow or consume-once behavior. Existing internal
deadline, cancellation, repeated-commit and concurrent-winner tests own the
control semantics; candidate binding and `finish(self)` require their own source
and semantic evidence. Never widen production visibility to manufacture a
consumer for those claims.

The source files alone provide no execution evidence. The
[existing S2.2 task](../../../../tasks/s0.1-crypto-feasibility.md#primary-tls-error-repair-and-external-consumers)
records the exact successful seven-consumer compiler checkpoint and actual
diagnostics. This supplies no certificate/TLS/exporter success, Windows
cleanup/resource qualification or production association authority.
Native execution, signing and post-V1 proof holds remain unchanged.
