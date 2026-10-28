# Rust execution evidence

Apply current project policy and selected tool versions; never copy another
project's command aliases, MSRV, concurrency limits or required assets. The pinned
PdfCraft Rust port supplied useful evidence distinctions, recorded in
[SOURCES](../../SOURCES.md). It is reference material, not NevroTick implementation.

Before Cargo compilation/checks, inspect the relevant local/build/proc-macro code
and command expansion. `--locked` prevents resolution changes but may fetch;
`--offline` restricts Cargo retrieval, not arbitrary build-script I/O. Neither flag
is a sandbox. Use the admitted toolchain and cache policy; missing prerequisites
leave the check unrun, without silently installing components or changing global
configuration.

Select a small package/target/compiler/profile/defaults/explicit-feature matrix.
Workspace metadata needs explicit member inheritance; a root `rust-version` does
not prove every member declares that MSRV. Resolver fallback helps resolution,
not compilation on the promised compiler. All-features unification may conceal
missing member declarations and may enable incompatible backends. Do not default
to an exponential feature powerset or manifest-mutating helper.

Capture the exact selected test names and actual executions, ignored/filtered
cases and first failure. Listing tests is discovery; a successful zero-test filter
does not exercise behavior. Compile-fail specimens need the intended diagnostic
and a compiling companion, not a missing import or unrelated syntax error.
`no_run` examples compile without executing. `cargo test --all-targets` excludes
doctests; preserve a separate doctest lane when applicable.

Cargo build jobs, libtest threads and native operation concurrency are different
budgets. Separate writers need owned build/output/temp resources; a test-runner
serial group does not grant an exclusive Windows VM lease. Keep real clock/service
tests in the project-authorized isolated Windows lane. Deadlines must supervise
owned descendants and report cleanup; a timeout is not a product assertion.

If nextest is admitted, its process-per-test model differs from libtest's shared
test-binary process; retain the runner that reproduced any process-global defect.
External files, ports and machine state can collide under either model. Optional
coverage/mutation/compile-test instruments require their own admitted setup and
meaningful controls; installing one does not execute qualification. Inspect tools
that offer LLVM component installation or temporarily rewrite manifests before
calling them read-only.

Preserve source/dirty/config/lock/fixture identity, cwd, command, toolchain/runner,
target/features/profile, executed names, exit/signal, retries, timing and cleanup.
Keep source-derived hypotheses, observed RED/GREEN, runtime Windows behavior and
release evidence distinct. A filter failing before the intended guard is a harness
failure, not causal product RED.

## Handle review feedback

Before editing, check each reported requirement or defect against the actual
owner, callers, supported configurations and current architecture. Treat both
the reviewer's claim and the implementer's rationale as claims to verify. Check
why the existing behavior exists and which consumers rely on it.

For substantial feedback, record each item as accepted, rejected with technical
evidence, or unresolved with the missing observation. An unverified claim is not
a rejection. Resolve unclear dependencies before changing their behavior while
continuing independent authorized work. Implement accepted corrections with a
focused causal check and affected-consumer coverage; revise a mistaken assessment
when new evidence changes it. A local feedback review grants no permission to
post external replies, remove apparently unused contracts or change architecture.

This intake method adapts Superpowers' pinned
[receiving-code-review workflow](https://github.com/obra/superpowers/blob/2ce8aa15d05bd57213e6f746f953aafb7ded7a8b/skills/receiving-code-review/SKILL.md);
[provenance](../../../PROVENANCE.md) and
[MIT notice](../../../provenance/licenses/superpowers/LICENSE) accompany it.
