---
name: "implement-rust"
description: "Implement, refactor or review NevroTick Rust logic, stable API choices, ownership, typed errors, safe diagnostics, recovery, state, concurrency and Windows adapter boundaries."
---

# Implement Rust

Start with repository instructions/current architectural owner. Preserve requested
implementation/review/planning scope, dirty work and resolved decisions. Ordinary
code edits must not create Cargo workspaces, dependencies or service architecture
as side effects.

Before choosing language/library features, inspect the actual toolchain, member
MSRV/edition, target and admitted feature graph. For newer syntax, standard-library
replacements, pointer APIs or compiler compatibility, read
[modern stable Rust](references/modern-rust.md). Prefer safe APIs that simplify the
touched contract; preserve explicit rejection, rounding, guard lifetimes and
resource limits. Stable availability is not a reason for mechanical rewrites,
new unsafe code, a dependency/toolchain change or a claimed performance gain.
Keep the detailed release survey in the linked source register, not every task.

Read the shared [implementation contract](../CONTRACT.md). For every changed
function/module, identify preconditions, postconditions, failure and effect/state
guarantees and their enforcement. Use private checked types in signatures;
validate new raw values and transformed results before exposure. Mandatory garde
validation belongs in the shared fallible construction path, so direct callers
cannot forget a separate validation step. See
[contract enforcement](../implement-rust-contracts/references/contract-enforcement.md).

Locate producer, callers, mutation paths, effects and target/feature lane.
Distinguish pure core, wire admission, engine, broker, Windows adapter and UI.
With source present, initialize Serena before symbol/reference queries; use Probe
for Rust search/extraction and configured documentation tools for design context.
Empty graph before source exists is expected.

For a pure-crate boundary or lint-policy change, use
[role-specific API restrictions](../../testing/references/role-api-restrictions.md).
Keep ambient reads and effects in their designated adapters; named Clippy bans
supplement ownership review and do not prove transitive purity.

For ownership, async work, error/numerical boundaries, unsafe adapters,
dependency/API choices or their tests, apply
[Rust implementation anti-patterns](references/anti-patterns.md). Forbid
detached-copy updates presented as owner updates and borrow-checker workarounds
that obscure authority; preserve justified snapshots and bounded shared ownership.
Lint controls complement, never replace, real-owner behavioral checks.
For unresolved or newly proposed APIs, imports, features and dependencies, use
[API evidence and diagnostic triage](references/api-evidence.md) before changing
the graph or retrying generated code.

Choose smallest model that makes meaningful mistakes hard to express:

- Private newtypes for units/identity; enums for explicit states/outcomes;
  `Result` for expected failures; checked boundary conversion/arithmetic.
- Preserve exact timestamps/counters until deliberate conversion of small deltas.
  Reject non-finite estimator/FFI results and validate post-conversion bounds.
- Match ownership, borrowing and lifetimes to actual handle/state owners.
  No `Arc<Mutex<_>>`, cloning or dynamic dispatch merely to silence compiler errors.
  Locks must cover complete invariant-bearing transactions.
- Never erase operational failure into `None`, empty results or stale success.
  Redact secrets; retain structured context sufficient to diagnose rejection.

Where a new field or state needs a policy decision, make its addition require
review through exhaustive field handling, explicit initialization and closed-enum
matches. Follow [structural change checks](../implement-rust-contracts/references/contract-enforcement.md#make-structural-changes-require-review)
without banning legitimate defaults or required external wildcard handling.
Prefer slice patterns, `first`, `split_first` or checked access over separate
length checks followed by indexing; handle absence under the actual contract.
Confine initialization mutability to its needed scope. Use named option enums
when boolean arguments obscure distinct choices; related invalid combinations
still need a constrained representation or checked construction.

Make changed code readable in the domain's terms: use established names and keep
decisions, failures and effects traceable. Extract a helper when it names a
coherent operation or isolates a meaningful rule. Keep related matching, loops
and invariant-bearing updates together when extraction would obscure their
relationship. Treat nesting and function length as review signals, not fixed
limits; format and Clippy success do not establish comprehensibility.

For floating-point estimation, exact-to-floating seams or numerical admission
thresholds, use [numerical kernels](references/numerical-kernels.md). Separate
independent mathematical results from rounded execution controls; freeze the
supported conditioning/range envelope and comparison semantics before coding.

For product mathematics, apply [OD-10’s first-release deferral](../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026):
continue ordinary implementation and nonformal qualification; missing proofs do
not block V1. Keep unproved or invalidated obligations in the existing task for
post-V1 work and retain exact historical proof scopes. Do not initiate formal
jobs, verifier extensions or pending proof admission/metadata repairs before V1.
Preserve runtime guards, checked arithmetic and independent tests. The
[proof workflow](../../testing/references/formal-verification.md) applies after V1.

When changing error types/propagation, error-to-output mappings, retry/recovery,
fallible cleanup or panic boundaries, read and apply
[error handling and safe diagnostics](references/error-handling.md). Enforce the
shared `diagnostics`/`windows-platform` responsibilities while keeping domain
failures and recovery at their owners. Project permitted typed fields before
formatting/buffering; verify the real failure, output and effect paths.

For FFI, handles, panic policy, directional secret APIs, async work, cancellation
or lifecycle changes, use
[Rust and Windows boundaries](references/rust-windows.md). Pure core forbids unsafe;
Windows adapters need small reviewed unsafe contracts. UI/engine have no
clock-setting privilege.

For the engine's Tokio runtime, blocking bridge, task/channel ownership, I/O,
deadline/scheduler changes or utilization review, read
[Tokio at the engine boundary](references/tokio.md). Select mechanisms for an
actual consumer; keep driver, progress and final eligibility within the existing
native-custody and S2.2 contracts. Ordinary Rust work needs no Tokio-specific review.

For Windows service registration, SCM callbacks/status, account isolation or
stop/restart changes, use [implement-windows-service](../implement-windows-service/SKILL.md)
and its relevant [service adapter details](references/windows-service-adapters.md).
That leaf owns the service workflow, including management, diagnostics and
measured optimization; keep separate engine/broker recovery and access records.

For compiler/runner/feature changes or review-feedback intake, read
[execution evidence](references/execution-evidence.md) before selecting commands.

For translated or behavior-adapted chrony algorithms, or a deterministic simulator
used to qualify their behavior, read the
[chrony port and simulator recipe](references/chrony-port-simulator.md). Work one
algorithm-sized slice from exact upstream provenance through an independent
numerical contract and focused evidence. Preserve architecture section 18.3's
GPL-2.0-only, notices and corresponding-source/build obligations; translation does
not establish clean-room provenance or clear an unwritten dependency graph.

Route data admission or internal invariant contracts to
[implement-rust-contracts](../implement-rust-contracts/SKILL.md), compatibility
changes to [evolution](../evolve-data-contracts/SKILL.md), and controls/focus/rendered
status to [native UI](../implement-native-ui/SKILL.md). Reuse immutable typed
guarantees across internal calls; recheck facts invalidated by state/context changes.

For an explicit performance task or measured cost/regression, use
[performance optimization](../optimize-rust-performance/SKILL.md): attribute the
bottleneck, compare bounded baseline/candidate runs and preserve correctness,
security and resource limits. Ordinary Rust edits need no benchmark campaign.

Select executable checks from the
[configured-tool workflows](../../testing/references/configured-tooling.md).
For real Rust source, run applicable format/compiler/Clippy lanes and focused
regressions for the changed owner and callers; API/documentation changes also
need their rustdoc/doctest lane. Admit actual target/features before execution;
host checks cannot replace Windows adapter linking or native observations.
Policy/runner changes use existing healthy/rejection controls, not a fabricated
application workspace. Preserve target mitigation flags and narrow expectations.

For a concrete memory/ownership gap, select admitted Miri on reached portable
tests. For synchronization changes, select an instrumented bounded Loom model;
use experimental Lockbud only after the real source/features compile on its
separate admitted compiler. Kani proof work is deferred until after V1 under OD-10;
hostile parser/conversion inputs may suit cargo-fuzz. Use the guide's guarded
analysis lane and then qualify the real harness; bootstrap controls alone test
no product path. Keep shipping Rust and native Windows observations separate.

For an address, initialization, race or leak question, follow
[sanitizer selection](../../testing/references/sanitizers.md). Reuse existing Miri
and fuzz workflows; choose additional tools only for their admitted defect scope.

For reached Windows heap/handle/FFI or resource-lifetime changes, use
[native memory diagnostics](../../testing/references/windows-memory.md).
Select Application Verifier/PageHeap for native misuse and separate lifecycle
measurements for retention; retain the target's intended ordinary-user token
and restore only owned image settings after the run.

When locating uncertain cross-function input-to-effect or secret-to-output paths,
read [CodeQL data-flow analysis](../../testing/references/codeql-data-flow.md).
Confirm query paths against current callers and current guards; a missing result
cannot establish an unreachable effect.

Verify changed guarantee/actual callers with smallest appropriate behavioral
check and the selected configured checks.
For bugs, show causal failure and healthy control before repair where practical.
Use [QA contract](../../testing/CONTRACT.md) for test integrity. Compiler acceptance
proves type obligations, not freshness, authorization, numerical truth or Win32
runtime behavior.

Report result, changed owner/callers, failure/effect decisions, executed checks
and target/runtime uncertainty. Reuse passing evidence while relevant state stays
unchanged; small edits need no blanket release-gate campaign.

Adapted David Osipov skill material: [licence and notices](THIRD_PARTY_NOTICES.md).
