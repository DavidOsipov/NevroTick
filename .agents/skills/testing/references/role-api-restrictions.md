# Enforce selected APIs by crate role

Use when changing pure-library boundaries or their Clippy enforcement. Architecture
[component ownership](../../../../docs/windows-time-architecture.md#3-component-architecture-and-authority)
and the current crate contract determine permitted effects. Inspect current
configs and runner controls before calling enforcement missing or qualified.
This procedure adopts no new API list or runner by itself.

## Derive the restriction from the owner

Pure calculations receive explicit time coordinates, validated configuration and
inputs. Selected bans can reject ambient clock/environment reads, file/socket
operations, process/thread creation and printing where that role forbids them.
Adapters retain the capabilities they actually own. Preserve inert values such
as `Duration` and `SocketAddr`; a type's namespace alone does not make it effectful.
Keep fixture-specific exceptions explicit and scoped; do not describe selected
I/O bans as a prohibition of every possible output path.

Use Clippy's `disallowed-methods`, `disallowed-types` and `disallowed-macros`
with qualified item paths and reasons explaining the owning adapter or explicit
input to use. Configuration supplies lists, not lint severity: verify effective
`clippy::disallowed_methods`, `clippy::disallowed_types` and
`clippy::disallowed_macros` denial on the pinned Clippy. An existing denied group
may already cover them; do not add redundant lint policy merely for appearance.
Workspace lint tables need member opt-in through `[lints] workspace = true`.

## Qualify discovery and the actual gate

- Inspect `CLIPPY_CONF_DIR`, `CARGO_MANIFEST_DIR` and working-directory discovery
  under the actual runner. Clippy searches upward from its selected starting
  directory and uses the first config; crate-local configuration does not merge
  with the root's settings. Preserve/admit common strict options in the selected
  file. Reject unadmitted environment redirection or shadow configuration files
  (`clippy.toml` or `.clippy.toml`); intentional admitted local configs remain valid.
- Integrate changed configuration with existing exact-policy admission and
  configured verification entry points. Check selected members, targets/features,
  fixture roles and error propagation; do not bypass the runner to obtain GREEN.
- In isolated compile-only fixtures, require explicit-input pure code to pass
  and each changed restriction family to fail with its intended Clippy diagnostic.
  Challenge imported aliases, function references, type-only uses and macros
  where relevant. Clippy resolves items; textual spelling checks are insufficient.
  Preserve a permitted adapter control using the same capability. Never execute
  an effectful control merely to prove whether it compiles.
- Challenge omitted/wrong config and suppressed lint paths as applicable to the
  changed gate. Narrow exceptions need a reason and the existing stale-expectation
  check. An unrelated compiler failure is not the expected rejection.

## State the coverage limit

Named bans are a useful regression guard, not a Rust effect system or sandbox.
Review wrappers, transitive dependencies, build scripts/proc macros, native code
and unselected feature/target paths independently. `no_std` narrows ambient APIs
but does not prove effect freedom through FFI or dependencies. Preserve ownership,
dependency and behavioral checks; do not add fake application crates for controls.

Keep concrete policies and execution receipts with the existing implementation
task. Report configured, exercised and uncovered restrictions separately under
the applicable CHECK-20/24/25 obligations.

Primary behavior: [Clippy configuration discovery](https://doc.rust-lang.org/clippy/configuration.html),
[disallowed-item settings](https://doc.rust-lang.org/clippy/lint_configuration.html),
[lint catalogue](https://clippy.rust-lang.org/stable/index.html#disallowed_methods),
[Cargo workspace lint inheritance](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-lints-table).
Recheck against the admitted tool version before changing policy.
