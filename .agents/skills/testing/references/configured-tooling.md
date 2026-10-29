# Use the configured NevroTick tools

Use this reference when selecting implementation, dependency, static/security or
package checks. [TOOLING.md](../../../../TOOLING.md) owns current commands and
qualification limits; [tool pins](../../../../policy/tool-manifest.json), actual
manifests and the selected target/feature row determine what can run. Select the
affected checks below, rather than executing the whole catalog for every edit.

Run from the inspected checkout root. Repository scripts bind their own source
root, so execute the candidate's copy when verifying another checkout or staged
tree. Give receipts/SBOMs new owned external paths; preserve existing evidence.
Inspect acquisition and build-script/proc-macro effects before execution. Setup
scripts install tools; selecting a skill grants no new installation authority.

## Readiness and policy changes

| Need | Existing entry point | What the result establishes |
| --- | --- | --- |
| Admit the local Rust environment | `python3 -B scripts/check-environment.py` | Pinned binaries, effective selection, target flags and environment. This Linux/WSL checker admits specific helper digests; another host needs its own admission, not relaxed pins. |
| Check lint, runner and analyzer configuration | `python3 -B scripts/check-strict.py` | Configuration and tool admission, including without application source. |
| Check a real workspace's adoption | `python3 -B scripts/check-strict.py --workspace` | Member lint/profile inheritance only; no application compilation or tests. |
| Change compiler/lint/runner enforcement | `python3 -B scripts/check-strict.py --fixtures` | Actual healthy/rejection controls in isolated dependency-free workspaces; bootstrap scope only. Run affected `scripts/tests` controls too when changing their admission/runner behavior. |

Use `--json` with a fresh external receipt when the task needs retained command
evidence. `--smoke` on the environment checker is an isolated environment fixture,
not an application test. Before Cargo exists, these controls and skill-pack checks
are available; application, coverage, mutation and product-SBOM gates stay pending.
Do not scaffold Cargo or substitute a fixture graph to claim a product pass.

## Rust implementation and dependency rows

For an ordinary Rust change, select format, compiler/Clippy and focused regression
checks for affected packages and actual callers. Use the adopted workspace policy
from [rust-workspace.toml](../../../../policy/rust-workspace.toml); every member
inherits it, while pure-core roots additionally forbid unsafe code. Expectations
need reasons and must remain fulfilled; no blanket test or adapter exemptions.

The following are workspace forms. Narrow packages and add the actual `--target`,
profile, defaults and optional features before execution where required. Keep
Windows-only consumers in scope; host success does not cover them.

| Changed guarantee | Select the relevant commands |
| --- | --- |
| Source formatting | `cargo fmt --all -- --check` |
| Compilation/lint rules | `cargo check --workspace --all-targets --frozen` and `cargo clippy --workspace --all-targets --frozen -- -D warnings`; a selected Clippy lane can also supply the check evidence. Link/build separately when the claim needs emitted binaries or FFI linkage. |
| API/documentation | `cargo doc --workspace --no-deps --frozen`, plus affected doctests. Workspace rustdoc policy remains enabled. |
| Test execution | `cargo nextest run --workspace --frozen --user-config-file none --no-tests=fail` with the selected tests; run `cargo test --workspace --doc --frozen` separately for doctests. Keep required libtest cases where process-global behavior matters. |
| Changed dependency graph, features or supply-chain policy | `cargo deny --frozen check --deny warnings` for the selected graph/target/features. Optional complementary `cargo audit --no-fetch` uses its repository configuration and audits the lockfile; it does not supply Cargo feature-row graph evidence. |

Frozen analysis follows separately authorized acquisition/resolution. Verify
RustSec/index snapshot identity and a refresh receipt no older than 24 hours;
missing/stale metadata is incomplete. `--frozen` makes cargo-deny offline too.
Cargo-audit shares RustSec data and is not an independent advisory source. Retain
existing findings and provider/license holds. See
[Cargo operations](../../environment/dependency-updates/references/cargo-operations.md)
for scoped updates; do not use frozen flags on an intended resolution change.

Record actual selected/executed tests, ignored/skipped cases and native exits.
Zero tests, a successful admission command, or `cargo check` without linking
cannot satisfy a claim needing execution or delivered Windows behavior. Keep
mitigation flags in `.cargo/config.toml`; convenience `RUSTFLAGS` can replace them.

## Additional Rust analysis

[The analysis policy](../../../../policy/analysis-tools.json) and
[TOOLING's analysis status](../../../../TOOLING.md#additional-rust-analysis-tools)
own the exact separate tool/compiler pins and observed fixture qualifications.
Use `python3 -B scripts/check-analysis.py --tool <name>` for hash admission only;
it executes no version/smoke commands. Add `--fixtures` for that lane's bounded
healthy/defect controls. `--tool all`
selects the six lanes, not a required per-edit campaign. These commands do not
test an application or create a product workspace.

| Changed guarantee | Select only the useful lane |
| --- | --- |
| Portable Rust undefined behavior — Miri | `--tool miri --fixtures`, then qualify real selected core tests on the admitted Miri toolchain. Native Win32/FFI execution remains separate. |
| Parser/conversion input resilience — cargo-fuzz | `--tool fuzz --fixtures`, then a bounded production target with accepted seeds, independent invariants and a sanitizer/target control. The admitted build profile explicitly selects `--target x86_64-unknown-linux-gnu`; the musl prebuilt cargo-fuzz otherwise defaults to musl. |
| Instrumented synchronization model — Loom | `--tool loom --fixtures`, then a small model whose actual synchronization uses Loom primitives; record branch/permutation/preemption bounds. Loom is a library payload, not a cargo-installed executable. |
| Small bounded property proof — Kani | `--tool kani --fixtures`, then a real harness with reachable healthy controls, explicit assumptions/unwind bounds and full proof-result accounting. |
| Static concurrency defect screening — Lockbud | `--tool lockbud --fixtures`, then admit compatible source/feature rows on its older experimental analysis compiler. Shipping Rust 1.99.1 is unchanged; no automatic compiler downgrade follows. |
| Dependency review obligations — cargo-vet | `--tool vet --fixtures`, then a real audited graph with `--locked --frozen --no-all-features --no-minimize-exemptions --no-registry-suggestions`. Missing audits block; do not invent reviews/imports/exemptions. |

The guarded entry point verifies the private Linux installation pointer and
payload inventories pinned by repository policy. Full inventories and payloads
reside on Drvfs; mode `0777` there is not privacy evidence. Payloads are mounted
read-only during analysis. The pointer
and execution receipts use Linux `0600`; actual work uses owned Linux `0700`
directories. Check current fixture replay status in TOOLING; successful tool
controls do not establish an application lane.
Cargo-vet needs a writable store even for locked checks; use an owned isolated
copy and check its complete inventory and bytes before and after execution.
The analysis runtime clears inherited environment, isolates network/processes
and bounds command/resource/output custody. ASan mode needs its large virtual
reservation, so it omits the ordinary address-space cap while retaining fixed
finite libFuzzer runs/length/time/RSS/allocation limits. These are not a hard
cgroup RAM or total disk quota. No floating-nightly install, global PATH change,
default-compiler change or long campaign follows from loading a skill.

## Native Windows memory diagnostics

For a Windows heap, handle, FFI or resource-lifecycle gap, read
[the native diagnostic workflow](windows-memory.md) and
[TOOLING's current native status](../../../../TOOLING.md#native-windows-memory-diagnostics).
Admit Application Verifier, GFlags/PageHeap and cdb separately from Linux/WSL
analyzers. Use only the owned image's documented checks; retain elevated setup,
ordinary-user target execution and verified restoration as separate steps.
Never overwrite another owner's image configuration or enable system-wide flags.

Healthy/faulty tool controls establish the reached mechanism, not application
memory safety. PageHeap is no general leak detector; measured lifecycle/resource
retention and uninstrumented performance evidence remain separate. Before a
product executable exists, no active product profile or product qualification
follows from installation. The lane does not expand UI-M0 scope.

## Workflow, SAST and secret checks

For workflow/scanner/gate changes, use
`python3 -X dev -Werror -B -m unittest discover -s .github/security -v` and
`python3 -B .github/security/check_workflows.py`. The latter exercises controls
and enforces pinned actionlint, ShellCheck and pedantic offline zizmor. Its
default invocation downloads verified binaries into temporary storage; use
`--tools-dir` for available digest-admitted copies when avoiding acquisition.
Offline zizmor does not cover online metadata audits.

For secret review, use
`python3 -B .github/security/check_secrets.py --binary /absolute/path/to/gitleaks`
with the pinned executable. It challenges detection and scans retained history
and the working tree with redaction. `--controls-only` checks the scanner, not the
candidate. Review a staged candidate's actual materialized tree separately; a
working-tree scan in the original checkout does not establish staged-tree safety.

Hosted CodeQL scanning is explicitly deferred in the
[security workflow](../../../../.github/workflows/security.yml). All prepared
CodeQL lanes skip acquisition/extraction/analysis; the final gate independently
requires the deferral marker, skipped output-free CodeQL jobs and successful
mandatory workflow/secret checks. A passing gate supplies no CodeQL SAST result.
Helper tests and `check_sarif.py` alone prove no extraction. Keep unavailable
checks explicit; do not push code or change hosted protections merely to obtain
a run. Hosted preparation scripts are a separate runner lane from local tools.

## Local CodeQL development MCP

For a specific Rust source-to-sink question, select
[qa-data-flow-analysis](../qa-data-flow-analysis/SKILL.md) and the
[CodeQL execution reference](codeql-data-flow.md). The local MCP payloads are
installed separately from hosted scanning. [TOOLING.md](../../../../TOOLING.md#local-codeql-development-mcp)
owns current qualification status; installation and configuration alone establish
neither a live session connection nor an executed product query.

[The local policy](../../../../policy/codeql-mcp.json) owns pinned payloads,
allowed tools and limits. Use [the launcher](../../../../scripts/codeql-mcp.py):
`python3 -B scripts/codeql-mcp.py --check` admits installation identity without
connecting. Follow the reference for current MCP schema discovery, owned database
creation, primary Rust query APIs, healthy/faulty controls and actual path results.
Rust extraction can execute build scripts/proc macros even in `build-mode: none`;
retain the offline restricted launcher and inspect build inputs first.

Before Cargo source exists, only isolated tool/query fixtures can run; do not
create a product workspace for a scan. Tie application findings to nonempty
extracted source, target/features and query/model identity. Missing coverage,
empty results and AST/CFG diagrams cannot establish absence of a flow. Native
Windows, process-boundary and complete-package evidence remain separate.

## Coverage, mutation and release inventory

Installed cargo-llvm-cov and cargo-mutants have admitted identities/configuration;
their application gates still need real inventories in
[verification-manifest.toml](../../../../policy/verification-manifest.toml).
For declared coverage work, use matching LLVM tools, frozen selected rows,
`--failure-mode any --json --output-path` and `--fail-under-file-lines 90`, then
reconcile file/line evidence with each security module. Mutation needs an actual
baseline, nonempty critical-file selection and explicit jobs/test/build timeouts.
Keep ordinary lint capping disabled; compilation failures are unviable mutants,
not caught behavioral faults. Preserve the 95% per-critical-file obligation.

For an actual Rust release inventory, use
`python3 -B scripts/generate-sbom.py --output-dir /new/external/output`, passing
the build's `--no-default-features` and repeated `--features` only as applicable.
The wrapper currently fixes `x86_64-pc-windows-msvc` through
[sbom.toml](../../../../policy/sbom.toml); another target needs policy/wrapper
admission, not an invented `--target` argument. It admits cargo-cyclonedx and
freezes source/graph/configuration, requiring real manifests, lockfile and cached
dependencies. Reconcile its Rust
inventory with native/generated/MSI contents and final artifacts; generation
alone proves neither complete-product inventory nor permission to publish.
