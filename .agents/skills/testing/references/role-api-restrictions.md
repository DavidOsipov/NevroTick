# Enforce selected APIs by crate role

Use when changing pure-library boundaries or their Clippy enforcement. Architecture
[component ownership](../../../../docs/windows-time-architecture.md#3-component-architecture-and-authority)
and current crate contract determine permitted effects. Inspect current configs
and runner controls before calling enforcement missing or qualified.
This procedure adopts no API list or runner.

## Derive the restriction from the owner

Pure calculations receive explicit time coordinates, validated configuration and
inputs. Selected bans can reject ambient clock/environment reads, file/socket
operations, process/thread creation and printing where that role forbids them.
Adapters retain owned capabilities. Preserve inert `Duration` and `SocketAddr`;
namespace alone does not imply effects. Keep fixture-specific exceptions explicit
and scoped; selected I/O bans do not prohibit every possible output path.

Review indirect ambient observations alongside explicit clock constructors.
`std::time::Instant::elapsed` and `std::time::SystemTime::elapsed` both read the
current clock despite arithmetic-looking names. Where pure roles forbid ambient
time reads, review these exact qualified paths with selected clock bans; guidance
alone establishes neither configuration nor execution. Arithmetic on two supplied coordinates,
including permitted duration differences, remains legal. Preserve inert path
manipulation; select actual filesystem reads, never the whole `std::path::Path`
namespace.

Use Clippy's `disallowed-methods`, `disallowed-types` and `disallowed-macros`
with qualified item paths and reasons naming the owning adapter or explicit input.
Configuration supplies lists, not lint severity: verify effective
`clippy::disallowed_methods`, `clippy::disallowed_types` and
`clippy::disallowed_macros` denial on the pinned Clippy. An existing denied group
may cover them; add no redundant lint policy for appearance.
Workspace lint tables need member opt-in through `[lints] workspace = true`.

## Qualify discovery and the actual gate

- Inspect `CLIPPY_CONF_DIR`, `CARGO_MANIFEST_DIR` and working-directory discovery
  under the actual runner. Clippy searches upward from its selected starting
  directory; first config wins, without merging crate-local and root settings.
  Preserve/admit common strict options in the selected file. Reject unadmitted
  environment redirection or shadow configuration files
  (`clippy.toml` or `.clippy.toml`); intentional admitted local configs remain valid.
- Integrate changed configuration with existing exact-policy admission and
  configured verification entry points. Check selected members, targets/features,
  fixture roles and error propagation; do not bypass the runner to obtain GREEN.
- In isolated compile-only fixtures, require explicit-input pure code to pass;
  each changed restriction family must fail with its intended Clippy diagnostic.
  Challenge imported aliases, function references, type-only uses and macros
  where relevant. Clippy resolves items; spelling checks are insufficient.
  Preserve permitted adapter control using the same capability. Never execute
  effectful controls merely to prove compilation.
- For an elapsed-method restriction, supply the timestamp as a fixture input;
  prohibited `now` construction can mask a missing elapsed restriction. Require
  that exact method's diagnostic, pure two-input arithmetic control and permitted
  adapter neighbor. Check both elapsed methods separately under selected config.
- Challenge omitted/wrong config and suppressed lint paths as applicable to the
  changed gate. Narrow exceptions need a reason and existing stale-expectation
  check. Unrelated compiler failures are not expected rejection.

## State the coverage limit

Named bans guard regressions; they are no Rust effect system or sandbox.
Review wrappers, transitive dependencies, build scripts/proc macros, native code
and unselected feature/target paths independently. `no_std` narrows ambient APIs
but does not prove effect freedom through FFI or dependencies. Preserve ownership,
dependency and behavioral checks; do not add fake application crates for controls.

Keep concrete policies and execution receipts in the existing implementation
task. Report configured, exercised and uncovered restrictions separately under
the applicable CHECK-20/24/25 obligations.

Primary behavior: [Clippy configuration discovery](https://doc.rust-lang.org/clippy/configuration.html),
[disallowed-item settings](https://doc.rust-lang.org/clippy/lint_configuration.html),
[lint catalogue](https://clippy.rust-lang.org/stable/index.html#disallowed_methods),
[Cargo workspace lint inheritance](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-lints-table),
[Instant elapsed](https://doc.rust-lang.org/std/time/struct.Instant.html#method.elapsed)
and [SystemTime elapsed](https://doc.rust-lang.org/std/time/struct.SystemTime.html#method.elapsed).
Recheck against the admitted tool version before changing policy.
