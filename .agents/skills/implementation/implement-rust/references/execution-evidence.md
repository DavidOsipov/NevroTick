# Rust execution evidence

Apply current policy/selected versions; copy no other project's command aliases,
MSRV, concurrency limits or required assets. Pinned PdfCraft Rust port's evidence
distinctions appear in [SOURCES](../../SOURCES.md): reference material, not NevroTick
implementation.

Before Cargo checks/compilation, inspect relevant local/build/proc-macro code and
command expansion. `--locked` prevents resolution changes but may fetch;
`--offline` restricts Cargo retrieval, not arbitrary build-script I/O. Neither is
sandboxing. Use admitted toolchain/cache policy. Missing prerequisites leave checks
unrun; silently install no components or global configuration changes.

Select small package/target/compiler/profile/defaults/explicit-feature matrix.
Workspace members need explicit metadata inheritance; root `rust-version` proves no
member MSRV declaration. Resolver fallback aids resolution, not promised-compiler
compilation. All-features may hide missing member declarations/enable conflicting
backends. Avoid exponential feature powersets/manifest-mutating helpers by default.

Capture selected test names, actual executions, ignored/filtered cases and first
failure. Lists are discovery; successful zero-test filters exercise no behavior.
Compile-fail specimens need intended diagnostic and compiling companion, not missing
imports/unrelated syntax. `no_run` compiles without executing. `cargo test --all-targets`
excludes doctests; preserve applicable separate doctest lane.

Cargo jobs, libtest threads and native-operation concurrency are separate budgets.
Separate writers need owned build/output/temp resources; runner serial groups grant
no exclusive Windows VM lease. Keep real clock/service tests in authorized isolated
Windows lane. Deadlines supervise owned descendants/report cleanup; timeouts are no
product assertions.

Admitted nextest runs processes per test; libtest shares test-binary processes.
Retain runner reproducing process-global defects. Files, ports/machine state may
collide under either. Optional coverage/mutation/compile-test instruments need
admitted setup/meaningful controls; installation executes no qualification. Inspect
LLVM-installing or temporarily manifest-rewriting helpers before calling them read-only.

Preserve source/dirty/config/lock/fixture identity, cwd, command, toolchain/runner,
target/features/profile, executed names, exit/signal, retries, timing/cleanup.
Separate source hypotheses, observed RED/GREEN, Windows runtime and release evidence.
Filter failure before intended guard is harness failure, not causal product RED.

## Handle review feedback

Before edits, verify each reported requirement/defect against actual owner, callers,
supported configurations and current architecture. Verify reviewer claims and
implementer rationales alike. Check existing behavior's purpose/dependent consumers.

For substantial feedback, mark each item accepted, rejected with technical evidence,
or unresolved with missing observation. Unverified claims are not rejections.
Resolve unclear dependencies before changing their behavior; continue independent
authorized work. Accepted fixes need focused causal checks/affected-consumer coverage.
Revise mistaken assessments when evidence changes. Local review grants no external
reply, apparently-unused-contract removal or architecture-change permission.

This intake method adapts Superpowers' pinned
[receiving-code-review workflow](https://github.com/obra/superpowers/blob/2ce8aa15d05bd57213e6f746f953aafb7ded7a8b/skills/receiving-code-review/SKILL.md);
[provenance](../../../PROVENANCE.md) and
[MIT notice](../../../provenance/licenses/superpowers/LICENSE) accompany it.
