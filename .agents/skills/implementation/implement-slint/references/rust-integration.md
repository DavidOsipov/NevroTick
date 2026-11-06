# Slint Rust integration and ownership

**Source baseline:** Slint 1.18.1, commit
`372cf0ee5577c3dfec309a45e7b778ba4e81b734`, inspected 5 November 2026.
Treat these API findings as source evidence; qualify the actual admitted graph.

Apply [UI-M0's migration gates](../../../../../tasks/ui-m0.md#slint-migration-handoff--5-november-2026).
M0-SL-01/02 permit their named source preparation; implementation and native
execution remain held at their task gates. Keep the UI unelevated and fixture-only,
with D1/D2/D3 deferred. This reference introduces no runtime, dependency or effect.

## Contents

- [Build and version contract](#build-and-version-contract)
- [Generated Rust and unsafe policy](#generated-rust-and-unsafe-policy)
- [Controller and component ownership](#controller-and-component-ownership)
- [Dispatch and generations](#dispatch-and-generations)
- [Async runtimes and shutdown](#async-runtimes-and-shutdown)
- [Evidence at the applicable gate](#evidence-at-the-applicable-gate)

## Build and version contract

Prefer external `.slint` modules compiled through `slint-build`, then include the
generated module with `slint::include_modules!()`. Freeze the entry file, imported
components, build configuration and generated output identity with the candidate.
Keep build failures useful and fatal; never replace failed generation with stale
output or a stub UI. See the pinned [Rust integration API](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/lib.rs).

| Boundary | Required treatment |
| --- | --- |
| Runtime and compiler | Select the same exact admitted `slint` and `slint-build` version. Generated Rust requires a matching `VersionCheck_MAJOR_MINOR_PATCH`; a broad semver range is not evidence of a matched resolved graph. |
| Build and proc-macro dependencies | Inventory the host graph as well as the Windows runtime graph. Feature reduction on the runtime dependency does not establish the build-helper graph. |
| Rust toolchain | Check the actual selected manifests and repository compiler. The inspected upstream workspace declares Rust 1.92; do not infer that from older tutorial snippets. |
| Widget style | Select the admitted Fluent style through `CompilerConfiguration::with_style`; style, window backend and renderer are distinct settings. |
| Generated files | Use Cargo's `OUT_DIR` and supported include mechanism. Do not edit generated Rust to hide a compiler or lint problem. |

Sources: [version marker in the generator](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/compiler/generator/rust.rs),
[workspace manifest](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/Cargo.toml),
[build-helper manifest](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/build/Cargo.toml)
and [compiler configuration](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/build/lib.rs).

In `build.rs`, distinguish execution host from compilation target. `cfg!` and
`#[cfg]` there describe the host; read `CARGO_CFG_TARGET_OS` or `TARGET` at build
script execution for target decisions. Coordinate that decision with the target
gate on `include_modules!()` and the actual Cargo dependency declarations.
Cargo does not empty `OUT_DIR` on each rerun. Track imported source/assets and
configuration inputs, propagate compilation errors, and reject stale generated
output. Follow the [Cargo build-script contract](https://doc.rust-lang.org/cargo/reference/build-scripts.html)
and [build-script environment variables](https://doc.rust-lang.org/cargo/reference/environment-variables.html#environment-variables-cargo-sets-for-build-scripts).

Read examples against the selected release, not `master` or unqualified `latest`.
Even the pinned API prose contains older version literals. The inline `slint!`
macro also passes through Rust's tokenizer: string interpolation and some hex
colors valid in external Slint files cannot be written unchanged inside it.
Use the [documented macro limitations](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/macros/lib.rs)
when diagnosing a parser failure; do not add a runtime interpreter as a shortcut.

## Generated Rust and unsafe policy

Inventory generated module allowances, macro expansions, transitive derives and
handwritten native adapters separately. The inspected compiler uses `FieldOffsets`
and vtable machinery; the [field-offset derive](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/helper_crates/const-field-offset/macro/macro.rs)
emits unsafe operations and the [vtable macro](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/helper_crates/vtable/macro/macro.rs)
emits `allow(unsafe_code)` and unsafe implementations. External `.slint` files do
not eliminate this boundary. A safe public API is not proof of no generated unsafe.

Inspect the actual crate/module/workspace lint policy and resolve any generated
`allow` versus enclosing `forbid` conflict through M0-SL-02 and the current
Rust/security owners. Retain the approved handwritten-code policy. Never silently
weaken a `forbid` scope or workspace `deny`/lints, add a blanket `allow`, suppress
all adapter lints or patch generated code until checks turn green.
Identify any proposed isolation/allowance, exact covered bytes, invariants and
reviewer before adoption. Check actual expanded output with admitted tools only.

Preserve [function and module contracts](../../implement-rust-contracts/SKILL.md)
around generated interfaces. Slint-generated public structs, setters and callbacks
do not supply private domain construction, current authorization or sink safety.
Keep validated model owners outside the generated module and keep FFI in its
narrow reviewed adapter. Do not hand-edit compiler-owned types into security DTOs.

## Controller and component ownership

Keep checked preview state in Rust. Expose a bounded presentation model and typed
user-intent callbacks; see [language and model rules](language-and-models.md).
Document construction, mutation, error and release ownership for the window,
controller, models, timers, posted work and any native handles.

Create components and run Slint's event loop on the UI/main thread. Keep widget
callbacks short. Strong component handles behave like reference-counted owners;
capture `as_weak()` when a callback refers back to its component. A strong capture
can keep the callback and component alive indefinitely. Upgrade only where needed,
handle `None`, and release temporary borrows before invoking callbacks that may
re-enter the controller. Sources: [threading guidance](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/lib.rs)
and [weak-handle implementation](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/api.rs).

Keep `Rc<VecModel<T>>`, `ModelRc<T>` and component access on the UI thread. Pass
owned bounded data across threads, then construct/update the UI model there.
Do not add unsafe `Send`/`Sync` implementations to make a UI type cross threads.
Slint globals are shared within a component tree, not across separately created
top-level windows/tray components. Keep any shared state with its Rust owner and
project it explicitly; do not use a global as a process-wide authority store.
Sources: [ModelRc representation](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/model.rs)
and [global instance semantics](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/docs/astro/src/content/docs/reference/language/globals.mdx).

## Dispatch and generations

Use `Weak::upgrade_in_event_loop` or `invoke_from_event_loop` for an admitted
worker-to-UI handoff. `Weak::upgrade()` on the wrong thread returns `None`;
`unwrap()` can panic. Scheduling is fallible. A successful scheduling result is
not an application acknowledgement: `upgrade_in_event_loop` skips its closure if
the component no longer exists. Verify these distinctions against the
[pinned dispatch API](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/internal/core/api.rs).

Bound outstanding work before posting. A weak pointer prevents an ownership
cycle; it does not bound retained payloads or make a queue self-draining. Coalesce
replaceable presentation updates, cap batches/bytes and preserve nonreplaceable
outcome semantics. Do not enqueue one closure per unbounded log or measurement.

Bind delayed results to the editor/controller instance and revision that requested
them. Recheck that identity on the UI thread before publishing. Cancel, replacement
or closure must prevent an old result from overwriting a newer fixture state.
Specify overflow and shutdown behavior; count rejected/undelivered work without
retaining its raw payload. These local lifetime rules do not implement D2's future
production freshness, reconnect, policy or administrative-result machinery.

## Async runtimes and shutdown

Preserve the [engine-only runtime contract](../../../../../docs/windows-time-architecture.md#1861-engine-runtime-decision-and-alternatives).
`slint::spawn_local` polls a future in the Slint event loop; it does not create a
Tokio runtime. Upstream documents runtime-context/fairness constraints and rejects
using Tokio's current-thread scheduler on Slint's main thread. Its `async_compat`
example does not admit Tokio, that adapter or a second executor into NevroTick UI.
Do not introduce `#[tokio::main]`, blocking joins or nested `block_on` to bridge it.
Source: [spawn_local runtime compatibility](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/lib.rs).

Specify teardown for startup failure, normal close, cancellation and partial
construction. Stop producers/timers, prevent new posts, reconcile owned work and
release native resources at their real completion; dropping an awaiting future
does not prove that native work ended. Distinguish hiding from destroying:
`show()` retains an additional strong component reference while visible and
`hide()` releases that reference. Follow the [generated handle contract](https://github.com/slint-ui/slint/blob/372cf0ee5577c3dfec309a45e7b778ba4e81b734/api/rs/slint/docs.rs)
and the task's close/event-loop behavior. Never infer service shutdown from UI exit.

## Evidence at the applicable gate

Record source/graph/generation review in the existing M0-SL task chain. At the
authorized implementation gate, select real handler/model checks for invalid and
valid neighboring inputs, direct-call bypass, late delivery, cancelled edits and
resource ownership. At the authorized native gate, observe the real editor,
close/reopen and cleanup before process exit; a model test or successful build
does not establish those results. Keep D1/D2/D3 and inaccessible Windows checks
explicitly deferred/incomplete. Apply the [testing contract](../../../testing/CONTRACT.md)
without inventing a new product test gate for this reference edit.
