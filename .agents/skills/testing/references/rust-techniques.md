# Selecting Rust verification techniques

Current authority: repository manifest/pinned toolchain/chosen target/feature matrix. Use [configured tool execution](configured-tooling.md) for adopted commands and their scope; select optional techniques only for a concrete evidence gap. Reference installs no tools/creates no Cargo project.

## Cargo, compiler and discovery

Once a workspace exists, use the selected frozen format/compiler/Clippy, rustdoc and test lanes from [configured tool execution](configured-tooling.md). Select the required Windows target and features from the actual matrix; do not substitute `--all-features` when features are mutually exclusive or widen crypto/provider authority. Dependency resolution/acquisition is separate from frozen qualification.

`cargo test --workspace` normally includes enabled unit, integration and documentation tests. `--all-targets` expands to lib/bin/test/bench/example selection and does not include doctests; run a separate `--doc` invocation when that scope matters. Inspect `required-features`, `cfg`, harness settings and ignored tests. A name filter can select zero tests. `cargo check` does not link or execute the application; Clippy does not prove contextual authorization.

Use real compile-fail external consumer for private-field/unit/newtype/trait guarantees. Verify expected diagnostic cause/valid construction control; unrelated unresolved import must not pass. Same care for Rustdoc `compile_fail`; dedicated harness only if existing one cannot express requirement.

Inspect proc-macro/build-script/native prerequisites before Cargo on unfamiliar dependencies. Documentation-only change without product-behavior impact: formatting/pack checks suffice.

`--locked` prevents resolution changes but may fetch; `--offline` constrains Cargo retrieval, not arbitrary I/O performed by build scripts, proc macros or children. Cargo `-j` bounds build jobs, not libtest threads or application workers. Inspect and bound each resource separately. Optional nextest uses separate test processes, which may hide process-global interference seen under libtest; preserve the original-runner regression when relevant and run doctests separately. No dependency/toolchain installation follows from selecting a technique.

For feature/MSRV or public API claims, record an explicit package/target/compiler/profile/default/optional-feature matrix and a real baseline consumer. A workspace all-features run may unify features and hide a member's undeclared requirement. Workspace `rust-version` metadata alone does not prove member inheritance or actual MSRV compilation. Optional cargo-hack/cargo-semver-checks can aid a bounded comparison after their effects are reviewed; ordinary explicit Cargo selections and minimal real consumers are valid fallbacks.

## Time and asynchronous execution

Inject explicit UTC/monotonic/active-awake sources; advancing one must not implicitly advance others. Controllable transport/actuation ports around production core; preserve production algorithm. Drive completion with notifications/conditions/bounded deadline; no race-hiding sleeps.

If Tokio is selected, paused time requires its `time`/`test-util` capabilities and a current-thread runtime. It affects Tokio's clock, not `std::time::Instant` or Windows/QPC. Idle runtimes may auto-advance. `advance` jumps time and does not prove that all expired timers have been processed; await the actual completion condition. This runtime helper cannot simulate Windows sleep/resume or clock actuation by itself.

## Generated and concurrent cases

- **Property testing:** proptest is a candidate for composable generators/shrinking. State the invariant and independent oracle first. Generate valid values by construction; test malformed wire representations separately. Bound rejection/shrink work and retain concrete minimized inputs as well as seeds. Persisted seeds can generate different inputs after strategy changes; replay the actual counterexample as a named regression. Keep regression files available for review; do not stage them automatically.
- **Concurrency models:** Loom can explore small Rust synchronization models after the code's relevant primitives are instrumented. Ordinary `std`/OS synchronization outside that model is not automatically covered. Bound state space and record the model/configuration. Its documented C11-model omissions can yield missed executions or false alarms; a pass is not proof of Windows scheduling, FFI or all interleavings.
- **Undefined behavior:** Miri can interpret selected portable Rust tests and expose invalid memory/use/aliasing on reached paths. Qualify the admitted nightly and healthy/defect controls through the [analysis entry point](configured-tooling.md#additional-rust-analysis); preserve its admitted sysroot, strict provenance, alignment checks and seed selection. Do not disable validation or isolation to clear a failure. Unsupported native calls or unexecuted paths remain gaps.
- **Bounded proofs:** Kani needs an explicit property harness, reachable healthy behavior, justified assumptions and finite loop/unwind bounds. Check reachability of the intended property as well as its assertion; assumptions that remove all meaningful inputs prove nothing useful. Incomplete/failed proof obligations, vacuous assumptions and missing harnesses cannot become a pass. Lockbud provides separate experimental static concurrency screening; qualify compatibility with its pinned older compiler before analyzing real source. Neither replaces an independent state model or native Windows tests.
- **State models:** use an independent reducer/transition oracle for controller state, generations, permits, cumulative authority, recovery and status publication. Compare effects and state after every action, including invalid actions. Model checking and property testing complement controlled real-thread and real-OS tests.
- **Structured fuzzing:** qualify the exact cargo-fuzz/nightly/LLVM/sanitizer setup with a seeded valid target and a known fault control. Upstream sources currently disagree about Windows support: the README says Unix only, while the Rust Fuzz Book documents MSVC/AddressSanitizer setup. Prefer a qualified portable-core runner; qualify any native Windows fuzz runner independently. Record this source discrepancy instead of claiming universal support or impossibility.

Miri, cargo-fuzz, Loom, Kani and Lockbud now have separate installed payloads;
use the guarded [analysis selection](configured-tooling.md#additional-rust-analysis)
and current [qualification status](../../../../TOOLING.md#additional-rust-analysis-tools).
Installed identity and bounded bootstrap controls do not qualify a product
harness, Windows target or the exact dependency/features under test. Preserve
shipping Rust 1.99.1; select analysis toolchains explicitly without global changes.

For address, initialization, race or leak detection, use
[sanitizer selection and controls](sanitizers.md). It complements these existing
tools and distinguishes actual detector diagnostics from oracle assertions,
native C/C++ instrumentation and platform-specific evidence.

For Lockbud, retain admitted `--exit-code-on-bug`, exact crate selection and fresh
build outputs so the selected compiler actually analyzes the intended code.
Unknown `LOCKBUD_FLAGS`, cached/skipped compilation or a zero-diagnostic wrapper
can invalidate apparent success. Check controls and native exit propagation;
do not lower product MSRV or ignore its declared Rust version to fit this
experimental lane. Loom is an isolated library payload: importing it into a
model requires actual instrumented synchronization and an admitted dependency
graph, not merely the installed payload's existence.

For generated cases, reset model and owned resources for every case/shrink/replay; a failed reset invalidates subsequent reductions. Budget persisted replays, rejected draws, sequence length, shrinking, bytes and cleanup as well as nominal case count. Fuzzer `tmin` reduces one failure input while `cmin` reduces a coverage corpus; preserve original inputs and verify the minimized case independently. Neither corpus size nor a wrapper's zero exit proves semantic coverage or successful child execution.

## Source-to-sink and dependency review

For a cross-function data-flow or taint question, use
[CodeQL data-flow analysis](codeql-data-flow.md) to define sources, transformations,
guards and sinks against an admitted Rust database. Inspect current MCP schemas
and CLI/query-pack coverage before choosing an operation. Confirm reported paths
in current source and with independent behavioral controls; source graph,
query compilation and zero findings establish different, narrower facts.
Retain extraction, macro, native FFI and process-boundary gaps. Hosted CodeQL
deferral does not itself run a local query or authorize changing hosted gates.

For a real dependency-review obligation, select cargo-vet through
[configured analysis](configured-tooling.md#additional-rust-analysis) and
[dependency updates](../../environment/dependency-updates/SKILL.md). Audit
criteria, exact dependency graph and review provenance must be meaningful;
advisory scanners and cargo-vet answer different questions. Missing audits remain
blocking. Preserve canonical audit data, use an owned writable store copy and
verify the complete entry/byte inventory before and after locked checks.

## Coverage and mutation

Use selected coverage/mutation tool native data/effective target filters. cargo-llvm-cov/cargo-mutants have admitted installed identities, while real inventories, instrumentation and application campaign qualification remain required; use the [configured coverage/mutation selection](configured-tooling.md#coverage-mutation-and-release-inventory). Coverage measures reached code, not assertion correctness. Publish uninstrumented native/FFI paths/platform exclusions; missing Windows instrumentation cannot make critical path noncritical.

For mutation, establish an unmodified passing baseline; isolate the candidate and bound workers/time. For the architecture's 95% per-file gate, identify generated, viable, killed, survived, timed-out, unviable and excluded mutants separately. Compile failures are unviable, not killed. Diagnose timeouts; justify equivalence/exclusions independently. A targeted trial cannot claim an unrun full-manifest score. Select focused campaigns for a local change; full campaigns follow the declared qualification scope.

Primary-source research and limitations are retained in [SOURCES.md](../SOURCES.md).
