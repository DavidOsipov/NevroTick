# Rust development environment

Prepared on 28 October 2026 for the initial NevroTick development stage.
The repository pin is Rust **1.99.1**, the current stable release checked against
its official distribution manifest. Rustup is **1.29.2**. The same compiler and
components are installed in WSL and for the native Windows user.

There is no application Cargo workspace yet. This setup builds temporary,
dependency-free fixtures outside the checkout; it does not manufacture passing
application tests or select production crates, an MSRV, or a shipping profile.

## Installed toolchain and tools

[rust-toolchain.toml](rust-toolchain.toml) selects the compiler, minimal profile,
Clippy, rustfmt, rust-analyzer, rust-src, LLVM tools, and the
`x86_64-pc-windows-msvc` standard library. It governs this checkout without
changing other projects' default compiler. Edition 2024 has passed the bootstrap
fixture; actual package compatibility and MSRV remain explicit future decisions.

| Tool | Installed stable version | Initial scope |
| --- | --- | --- |
| Rust / Cargo | 1.99.1 | Host compilation and Windows MSVC target |
| rustup | 1.29.2 | Toolchain management |
| cargo-nextest | 0.9.150 | Optional test runner; doctests remain separate |
| cargo-deny | 0.20.2 | Dependency-policy tool when a real graph exists |
| cargo-audit | 0.22.2 | Optional complementary scan; shares RustSec data |
| cargo-mutants | 27.1.0 | Later critical-file mutation checks |
| cargo-llvm-cov | 0.9.1 | Later measured coverage with matching LLVM tools |
| xwin | 0.10.0 | Compact x64 Microsoft CRT/SDK acquisition for WSL cross-builds |

[policy/tool-manifest.json](policy/tool-manifest.json) records upstream archive
URLs/digests, installed binary identities, scopes and pending qualifications.
Tool availability and a version smoke do not qualify its eventual graph,
coverage, mutation or advisory gate. The strict configurations below have
isolated controls; real product graphs and behavioral denominators remain pending.
No application dependencies or signing authority were selected by this setup.

## Repeatable WSL setup and checks

Run from the checkout root with the existing Rustup and Python 3.11+:

```sh
python3 -B scripts/setup-unix.py --json /tmp/nevrotick-tool-install.json
python3 -B scripts/check-environment.py
python3 -B scripts/check-environment.py --smoke --json /tmp/nevrotick-environment.json
python3 -B scripts/check-strict.py --fixtures --json /tmp/nevrotick-strict.json
python3 -X dev -Werror -B -m unittest discover -s scripts/tests -v
```

The installer uses the reviewed pins rather than floating versions, verifies
prebuilt archive digests before execution, builds no third-party code, and
retains replaced executables outside the checkout. A partial failure records
completed tools and the backup directory. Updating the pins means checking
current stable upstream metadata and rerunning the affected admission controls.

The read-only checker rejects unexpected toolchain, rustflags, wrapper,
linker/profile and ancestor/home Cargo-configuration overrides. It prints names
rather than ambient secret values. A missing capability or guard failure is
incomplete, never a successful skip.

The smoke lane requires working bubblewrap and prlimit. It mounts only public OS
files, the pinned compiler and owned scratch; inherited environment, home,
checkout and privileged sockets are absent, and the network namespace is
isolated. Synthetic filesystem/environment/network canaries execute before
Cargo. There is no host-execution fallback.

The fixed fixture passes formatting, Clippy, **2 unit tests**, **1 doctest**, an
MSVC target compile check, deliberate unsafe-declaration rejection and a restored
healthy control. Jobs, output and wall time are bounded. Resource limits are
per-process/namespace, **not cumulative cgroup quotas**; this lane admits the
fixed trusted fixture, not arbitrary dependency build scripts.

## Strict development policy

Configured on 29 October 2026 using Context7 (Clippy and cargo-deny), pinned
tool help/schemas and primary upstream documentation. The policies use supported
stable rules. Broad Clippy `restriction`/`nursery` groups are deliberately excluded
because their rules can conflict or remain experimental; selected restrictions
follow the development-hardening guide.

| File | Enforced intent and current scope |
| --- | --- |
| [policy/rust-workspace.toml](policy/rust-workspace.toml) | Compiler warnings, unsafe boundaries, discarded results, unknown/stale lints/cfgs and missing public documentation fail. Clippy `all`/`pedantic` plus selected panic, conversion, unsafe, leak and exception rules fail. Stable rustdoc rules fail. Optimized arithmetic remains checked; `verify-release` adds debug assertions. |
| [rustfmt.toml](rustfmt.toml), [clippy.toml](clippy.toml) | Explicit 2024 formatting and private API checks; no blanket test exemptions for unwrap/expect/panic/debug output. |
| [deny.toml](deny.toml) | Strict advisory, license, source, duplicate/wildcard and build-script policy, including held crypto providers. Only the project's GPL-2.0-only license is initially allowed; third-party licenses require exact-graph review. |
| [.config/nextest.toml](.config/nextest.toml) | Two test workers, zero retries, fatal timeouts/leaks, bounded total duration and JUnit accounting including skips. The trusted checker rejects unknown settings because nextest itself only warns. |
| [.cargo/audit.toml](.cargo/audit.toml) | No ignored advisory IDs or severity cutoff; warnings fail. Acquisition is separate from analysis and requires an admitted database/index snapshot. |
| [.cargo/mutants.toml](.cargo/mutants.toml) | Frozen Cargo inputs, bounded timeout multipliers, no copied VCS/target, lint capping disabled. Enumeration/configuration controls are qualified; a mutation campaign is still pending. |
| [policy/verification-manifest.toml](policy/verification-manifest.toml) | Retains 90% executable-line coverage per security module and 95% mutation per critical file. Inventories are explicitly awaiting product code; empty inventories cannot establish application qualification. |

The Rust Cargo fragment is tested in owned temporary workspaces. Adopt its lint
and profile tables into the first real workspace and put `[lints] workspace = true`
in **every** first-party member, including build-script/proc-macro crates. Pure
domain/protocol/numerical crates additionally use `#![forbid(unsafe_code)]`.
Windows adapters use narrowly scoped, reasoned exceptions with documented safety
contracts. Stale expectations fail. Release `panic = "unwind"` is a qualification
baseline; each component's shipping fault policy still needs its documented
decision and FFI checks.

`scripts/check-strict.py` admits hashes before executing tools, validates policy
invariants, freezes fixture configuration bytes and rejects stale results after
concurrent policy changes. `--fixtures` checks healthy cases and intended failures
for formatting, compiler/Clippy/rustdoc rules, unsafe exceptions, arithmetic
profiles and nextest. The unittest command also exercises actual cargo-deny,
audit, nextest and mutation enumeration with local fixtures in the cleared-env,
network-isolated sandbox. Neither command builds application code.

`python3 -B scripts/check-strict.py --workspace` checks **manifest adoption only**;
it currently fails because no application workspace exists. It does not prove
the intended package/target inventory, feature matrix, native behavior or release
readiness. The reviewed verification inventory must be populated from real owners
before application gates are introduced. CLI evidence paths must resolve outside
the checkout; they cannot overwrite checked policy files.

Dependency controls cover local license/ban/build-script graphs and synthetic
source metadata. The audit control detects a synthetic vulnerable package in a
local committed database and accepts its patched neighbor. These controls do
not establish real registry/Git acquisition, yank checks or freshness. A successful
scan and `stale = false` are insufficient freshness evidence: require a verified
RustSec/index refresh receipt no older than 24 hours. Keep network refresh separate
from frozen, credential-free analysis. `cargo-audit` shares RustSec data with
`cargo-deny`; it is complementary evidence, not an independent advisory source.

Once real code and the critical inventory exist, use the admitted package/feature/
target rows and native host where required. Coverage has no native configuration
file: the selected invocation must pass `--frozen --failure-mode any --json
--output-path <external-report> --fail-under-file-lines 90` to `cargo llvm-cov`.
Reconcile its files and executable-line denominators with the reviewed security
modules; the CLI file threshold alone does not implement module accounting.
Mutation campaigns also require explicit `--baseline run`, `--jobs`, `--timeout`
and `--build-timeout` budgets and complete per-file outcome accounting. Strict-lint
build failures are unviable mutants, not caught behavioral mutations. Missing
reports, empty selections, tool failures and incomplete feature rows remain
incomplete. These coverage/mutation application gates cannot be qualified yet.

## Windows development route

The selected route is **LLVM + xwin in WSL**, retaining the documented MSVC ABI.
Visual Studio workloads and a system-wide Windows SDK were not installed.
The linker is Rust's bundled rust-lld, matching LLVM **23.1.1**. Microsoft CRT/SDK
library acquisition remains subject to Microsoft's terms independently of
xwin's own licence. The compact route's exact inputs and checks are recorded in
the tool manifest.

The selected stable catalog supplies SDK selector **10.0.28000** and CRT selector
**14.51** (CRT payload **14.51.36254**). Selectors and exact catalog/payload
identities are separate; the manifest and setup receipt retain both.

Acquire the reviewed libraries into an explicit cache on the Windows drive,
then use the narrow linker override when a real Cargo workspace exists:

```sh
python3 -B scripts/setup-xwin.py --cache-dir '/mnt/c/Users/<Windows-user>/AppData/Local/NevroTick/xwin-cache' --accept-license
cargo --config "target.x86_64-pc-windows-msvc.linker=\"${PWD}/scripts/xwin-linker.py\"" build --target x86_64-pc-windows-msvc --locked
```

The setup prints the library location and retains acquisition evidence outside
the checkout. The wrapper adds only the three admitted library paths and
preserves the compiler's link arguments, including the target hardening flags.
It does not change the global Cargo configuration or default target.

Setup prunes verified payload copies and matching unpack intermediates by
default; `--keep-cache` retains them for offline reacquisition. The installed
SDK/CRT headers and libraries occupy about **636 MiB**; the complete cache,
including provenance and the qualification fixtures, about **717 MiB**. The
cache is on the Windows drive. No Visual Studio or system SDK installer ran.

The fixed std-only fixture has passed both direct Rust linking and the actual
Cargo `--config` entry point inside an isolated, credential-free namespace. The
Cargo-built PE is AMD64/PE32+, contains the expected ASLR/DEP/CFG header flags
and instrumented CFG function table, and ran on Windows without elevation,
printing `NevroTick MSVC smoke: 42`. Its SHA-256 matched before and after native
execution. These are emitted-image and execution observations, not a claim that
all Windows runtime mitigation policies or application behavior are qualified.

Microsoft's channel metadata declares an incorrect package-catalog checksum,
also documented in [xwin's pinned source](https://github.com/Jake-Shadle/xwin/blob/0.10.0/src/manifest.rs#L377-L385).
The reviewed policy records the declaration **and** the repeatedly observed raw
catalog digest. Setup requires both pins to match and verifies individual
Microsoft payloads against that exact catalog. This is a content pin acquired
from the authenticated upstream source, not a claim that the incorrect
publisher-declared catalog checksum verified. Any changed bytes require review.

Native Windows Rust can be prepared without modifying persistent PATH:

```powershell
powershell -NoProfile -File .\scripts\setup-windows.ps1
```

PowerShell RemoteSigned can reject unsigned scripts on the WSL UNC share.
Use a reviewed Windows-local copy whose SHA-256 matches the repository file;
no machine execution-policy change or automatic elevation is needed. The Rust
executables reside in the current user's `.cargo\bin`; use explicit paths or
add that directory to the current shell only. The setup has been replayed on
Windows 11 Pro x64, build 26300.9711.

Cross-linking and running a small Windows fixture qualify only that declared
lane. They do not establish service privileges, IPC, clock behavior, native UIA/
Narrator, MSI lifecycle, production crypto compatibility or release readiness.
Those experiments still need the architecture's owned disposable Windows lab.

## Starting implementation

Use [docs/README.md](docs/README.md), the canonical architecture, and hardening
DEV-01 onward. The next real experiment owns its Cargo workspace, exact lockfile,
member lint inheritance, feature rows, independent acceptance oracle and selected
profiles. The strict lint/profile fragment above must be adopted there; this
environment setup does not silently impose a shipping panic policy.

[.cargo/config.toml](.cargo/config.toml) records the target-specific Windows CFG,
ASLR, DEP and linker-warning intent, without forcing a Windows default target.
Do not replace those flags with global RUSTFLAGS. The cross-linker is admitted
through a narrow, explicit command/configuration; substituted linker output must
be inspected rather than inferred from flags.

Once a real workspace exists, run its selected checks, for example:

```sh
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --frozen -- -D warnings
cargo doc --workspace --no-deps --frozen
cargo nextest run --workspace --frozen --user-config-file none --no-tests=fail
cargo test --workspace --doc --frozen
cargo deny --frozen check --deny warnings
```

Use explicit feature/target selections; do not add a universal all-features gate.
Commit Cargo.lock with the real graph. Preserve the documented TLS/provider,
licence and signing holds; installing toolchains resolves none of them.

## Code-navigation tools

Serena's project configuration selects Rust; use initial_instructions and the
returned Serena session before activation. SocratiCode uses the absolute checkout,
current indexing status and the registered document contexts. Probe supplies
Rust search/extraction without a persistent index. Context-mode processes large
output; Context7 and primary upstream sources resolve API questions. Root
[AGENTS.md](AGENTS.md) describes when to choose each tool. Existing connection
history is not a fresh runtime qualification; no application graph is expected
before source exists.
