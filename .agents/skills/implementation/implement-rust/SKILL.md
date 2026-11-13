---
name: "implement-rust"
description: "Implement, refactor or review NevroTick Rust logic, stable API choices, ownership, typed errors, safe diagnostics, recovery, state, concurrency and Windows adapter boundaries."
---

# Implement Rust

Start with repository instructions/current architectural owner. Preserve requested
implementation/review/planning scope, dirty work and resolved decisions. Ordinary
code edits must not create Cargo workspaces, dependencies or service architecture
as side effects.

Before choosing language/library features, inspect actual toolchain, member
MSRV/edition, target and admitted feature graph. For newer syntax, standard-library
replacements, pointer APIs or compiler compatibility, read
[modern stable Rust](references/modern-rust.md). Prefer safe APIs simplifying the
touched contract; preserve explicit rejection, rounding, guard lifetimes and
resource limits. Stable availability justifies neither mechanical rewrites,
new unsafe code, a dependency/toolchain change nor performance claims.
Keep detailed release survey in linked source register, not every task.
Rust 1.100 cleanup, never types, trimming/construction, smart-pointer or
allocator choices: apply [explicit feature contracts](references/rust-1.100-contracts.md).
Name the prevented mistake and distinguish type enforcement, automatic cleanup,
lint rejection and clearer syntax; retain stronger existing domain guarantees.

For new or materially changed Rust code, select stable, admitted APIs, methods and
representations enforcing strongest applicable domain guarantees. Preserve legitimate
inputs, required failure behavior and ownership semantics. Prefer types, ownership
and checked operations over caller discipline; preserve stronger existing guarantees.
Necessary weaker alternatives need a concrete reason and enforcement location for
each missing guarantee. Use the simplest representation achieving this; added
restrictions or abstractions must prevent meaningful mistakes. API names or newness
alone establish no stronger contract.

Read shared [implementation contract](../CONTRACT.md). For each changed
function/module, identify preconditions, postconditions, failure and effect/state
guarantees and enforcement. Use private checked types in signatures;
validate new raw values and transformed results before exposure. Mandatory garde
validation belongs in shared fallible construction path, so direct callers
cannot forget a separate validation step. See
[contract enforcement](../implement-rust-contracts/references/contract-enforcement.md).
Cross-module handoffs: apply its typed-role, consuming-operation, production
visibility and lifecycle-outcome rules; bind related facts to the actual owner.

Locate producer, callers, mutation paths, effects and target/feature lane.
Distinguish pure core, wire admission, engine, broker, Windows adapter and UI.
With source present, initialize Serena before symbol/reference queries; use Probe
for Rust search/extraction and configured documentation tools for design context.
Empty graph before source exists is expected.

Pure-crate boundary or lint-policy changes: use
[role-specific API restrictions](../../testing/references/role-api-restrictions.md).
Keep ambient reads and effects in their designated adapters; named Clippy bans
supplement ownership review but do not prove transitive purity.

Ownership, async work, error/numerical boundaries, unsafe adapters,
dependency/API choices or their tests: apply
[Rust implementation anti-patterns](references/anti-patterns.md). Forbid
detached-copy updates presented as owner updates and borrow-checker workarounds
obscuring authority; preserve justified snapshots and bounded shared ownership.
Lint controls complement, never replace, real-owner behavioral checks.
Unresolved or newly proposed APIs, imports, features and dependencies: use
[API evidence and diagnostic triage](references/api-evidence.md) before changing
graph or retrying generated code.

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

When new fields or states need policy decisions, require review through exhaustive
field handling, explicit initialization and closed-enum
matches. Follow [structural change checks](../implement-rust-contracts/references/contract-enforcement.md#make-structural-changes-require-review)
without banning legitimate defaults or required external wildcard handling.
Include rejection tests' manual state equality; preserve required bitwise and
allocation-identity semantics when choosing exhaustive patterns or derives.
Prefer slice patterns, `first`, `split_first` or checked access over separate
length checks then indexing; handle absence under actual contract.
Confine initialization mutability to needed scope. Use named option enums
when boolean arguments obscure distinct choices; related invalid combinations
still need a constrained representation or checked construction.

Use established domain names for readable changed code; keep decisions, failures
and effects traceable. Extract helpers naming coherent operations or isolating
meaningful rules. Keep related matching, loops and invariant-bearing updates
together when extraction would obscure relationships. Nesting and function length
are review signals, not fixed limits; format and Clippy success do not establish
comprehensibility.

Floating-point estimation, exact-to-floating seams or numerical admission
thresholds: use [numerical kernels](references/numerical-kernels.md). Separate
independent mathematical results from rounded execution controls; freeze the
supported conditioning/range envelope and comparison semantics before coding.

Product mathematics: apply [OD-10’s first-release deferral](../../../../docs/nevrotick-owner-decisions.md#141-first-release-deferral--6-november-2026):
continue ordinary implementation and nonformal qualification; missing proofs do
not block V1. Keep unproved or invalidated obligations in existing task for
post-V1 work; retain exact historical proof scopes. Do not initiate formal
jobs, verifier extensions or pending proof admission/metadata repairs before V1.
Preserve runtime guards, checked arithmetic and independent tests. The
[proof workflow](../../testing/references/formal-verification.md) applies after V1.

Changes to error types/propagation, error-to-output mappings, retry/recovery,
fallible cleanup or panic boundaries: read and apply
[error handling and safe diagnostics](references/error-handling.md). Enforce the
shared `diagnostics`/`windows-platform` responsibilities; keep domain
failures and recovery at their owners. Project permitted typed fields before
formatting/buffering; verify the real failure, output and effect paths.

FFI, handles, panic policy, directional secret APIs, async work, cancellation
or lifecycle changes: use
[Rust and Windows boundaries](references/rust-windows.md). Pure core forbids unsafe;
Windows adapters need small reviewed unsafe contracts. UI/engine have no
clock-setting privilege.

Engine's Tokio runtime, blocking bridge, task/channel ownership, I/O,
deadline/scheduler changes or utilization review: read
[Tokio at the engine boundary](references/tokio.md). Select mechanisms for an
actual consumer; keep driver, progress and final eligibility within existing
native-custody and S2.2 contracts. Ordinary Rust work needs no Tokio-specific review.

Windows service registration, SCM callbacks/status, account isolation or
stop/restart changes: use [implement-windows-service](../implement-windows-service/SKILL.md)
and its relevant [service adapter details](references/windows-service-adapters.md).
That leaf owns service workflow, including management, diagnostics and
measured optimization; keep separate engine/broker recovery and access records.

For compiler/runner/feature changes or review-feedback intake, read
[execution evidence](references/execution-evidence.md) before selecting commands.

Translated or behavior-adapted chrony algorithms, or a deterministic simulator
qualifying their behavior: read
[chrony port and simulator recipe](references/chrony-port-simulator.md). Work one
algorithm-sized slice from exact upstream provenance through an independent
numerical contract and focused evidence. Preserve architecture section 18.3's
GPL-2.0-only, notices and corresponding-source/build obligations; translation
neither establishes clean-room provenance nor clears an unwritten dependency graph.

Route data admission or internal invariant contracts to
[implement-rust-contracts](../implement-rust-contracts/SKILL.md), compatibility
changes to [evolution](../evolve-data-contracts/SKILL.md), and controls/focus/rendered
status to [native UI](../implement-native-ui/SKILL.md). Reuse immutable typed
guarantees across internal calls; recheck facts invalidated by state/context changes.

Explicit performance task or measured cost/regression: use
[performance optimization](../optimize-rust-performance/SKILL.md): attribute the
bottleneck, compare bounded baseline/candidate runs; preserve correctness, security
and resource limits. Ordinary Rust edits need no benchmark campaign.

Select executable checks from
[configured-tool workflows](../../testing/references/configured-tooling.md).
For real Rust source, run applicable format/compiler/Clippy lanes and focused
regressions for changed owners and callers; API/documentation changes also need their
rustdoc/doctest lane. Admit actual target/features before execution;
host checks cannot replace Windows adapter linking or native observations.
Policy/runner changes use existing healthy/rejection controls, never a fabricated
application workspace. Preserve target mitigation flags and narrow expectations.

Concrete memory/ownership gap: select admitted Miri on reached portable
tests. Synchronization changes: select an instrumented bounded Loom model;
use experimental Lockbud only after the real source/features compile on its
separate admitted compiler. Kani proof work is deferred until after V1 under OD-10;
hostile parser/conversion inputs may suit cargo-fuzz. Use the guide's guarded
analysis lane then qualify the real harness; bootstrap controls alone test
no product path. Keep shipping Rust and native Windows observations separate.

Address, initialization, race or leak question: follow
[sanitizer selection](../../testing/references/sanitizers.md). Reuse existing Miri
and fuzz workflows; choose additional tools only for their admitted defect scope.

Reached Windows heap/handle/FFI or resource-lifetime changes: use
[native memory diagnostics](../../testing/references/windows-memory.md).
Select Application Verifier/PageHeap for native misuse and separate lifecycle
measurements for retention; retain the target's intended ordinary-user token
and restore only owned image settings after the run.

Uncertain cross-function input-to-effect or secret-to-output paths:
read [CodeQL data-flow analysis](../../testing/references/codeql-data-flow.md).
Confirm query paths against current callers and guards; a missing result
cannot establish an unreachable effect.

Verify changed guarantee/actual callers with smallest appropriate behavioral
and selected configured checks.
For bugs, show causal failure and healthy control before repair where practical.
Use [QA contract](../../testing/CONTRACT.md) for test integrity. Compiler acceptance
proves type obligations, not freshness, authorization, numerical truth or Win32
runtime behavior.

Report result, changed owner/callers, failure/effect decisions, executed checks
and target/runtime uncertainty. Reuse passing evidence while relevant state stays
unchanged; small edits need no blanket release-gate campaign.

Adapted David Osipov skill material: [licence and notices](THIRD_PARTY_NOTICES.md).
