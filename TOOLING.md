# Rust development environment

Prepared on 28 October 2026 for the initial NevroTick development stage.
The repository pin is Rust **1.99.1**, the current stable release checked against
its official distribution manifest. Rustup is **1.29.2**. The same compiler and
components are installed in WSL and for the native Windows user.

The initial setup used temporary, dependency-free fixtures outside the checkout.
On 29 October, the separate UI-M0 implementation added a minimal real Cargo
workspace, one `nevrotick-ui` crate and a locked dependency graph. Use
[Starting implementation](#starting-implementation) for its shared verification
command and [the task record](tasks/ui-m0.md) for current results. Historical
bootstrap controls below do not establish application behavior or a shipping profile.
The subsequent [S1.1a task](tasks/s1.1-header.md) adds the dependency-free
`ntp-wire` header/framing owner and explicit package verification. Its source,
behavior and analysis evidence remain separate from UI and native qualification.
The subsequent [S1.1c task](tasks/s1.1-time-domain.md) adds the checked
`time-domain` arithmetic owner and admits three-member verification without
expanding the existing sixteen-package external dependency cohort.
The additional analysis tools below were installed separately on 29 October;
their compiler experiments do not change the repository's shipping Rust pin.

## Installed toolchain and tools

[rust-toolchain.toml](rust-toolchain.toml) selects the compiler, minimal profile,
Clippy, rustfmt, rust-analyzer, rust-src, LLVM tools, and the
`x86_64-pc-windows-msvc` standard library. It governs this checkout without
changing other projects' default compiler. UI-M0 declares edition 2024 and
`rust-version = "1.99"`; its verification lane uses the pinned 1.99.1 compiler.
This does not establish a separately tested production MSRV.

| Tool | Installed stable version | Initial scope |
| --- | --- | --- |
| Rust / Cargo | 1.99.1 | Host compilation and Windows MSVC target |
| rustup | 1.29.2 | Toolchain management |
| cargo-nextest | 0.9.150 | Optional test runner; doctests remain separate |
| cargo-deny | 0.20.2 | Frozen UI-M0 dependency rows and separate bootstrap controls |
| cargo-audit | 0.22.2 | Optional complementary scan; shares RustSec data |
| cargo-mutants | 27.1.0 | Later critical-file mutation checks |
| cargo-llvm-cov | 0.9.1 | Later measured coverage with matching LLVM tools |
| xwin | 0.10.0 | Compact x64 Microsoft CRT/SDK acquisition for WSL cross-builds |
| cargo-cyclonedx | 0.5.9 | Release-tool lane; CycloneDX 1.5 JSON Rust dependency inventories |

[policy/tool-manifest.json](policy/tool-manifest.json) records upstream archive
URLs/digests, installed binary identities, scopes and pending qualifications.
Tool availability and a version smoke do not qualify its eventual graph,
coverage, mutation or advisory gate. The strict configurations below have
isolated controls. The first UI-M0 graph has separate admission evidence;
production graphs and security coverage/mutation denominators remain pending.
The original tool setup selected no application dependencies or signing authority.

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

## Development CPU budget

David requested **12 parallel workers** on 30 October 2026. The repository
[Cargo configuration](.cargo/config.toml) sets twelve build jobs and twelve
libtest threads; [nextest](.config/nextest.toml) separately admits twelve test
workers. The guarded compiler, Clippy, rustdoc, test and coverage commands carry
the same budget into cleared sandbox environments. Independent inventory hashing
uses twelve workers. actionlint and Gitleaks receive `GOMAXPROCS=12`, which budgets
Go execution rather than their external subprocesses. Tools without a supported
parallel scheduler remain unchanged.

Mutation campaigns use twelve outer workers, twelve shared jobserver tokens and
**one Cargo build job and one libtest thread per worker**. Both enumeration and
campaign commands explicitly enable the pinned tool's jobserver. Do not multiply
both layers to twelve. Sequential
fixture edits, shared output ownership, native Windows operations, deterministic
resource observations, fuzz campaign ownership and Loom's modeled thread count
keep their existing ordering/bounds. Release codegen units, numerical semantics,
security checks and memory/CPU/wall-time limits are unchanged. These are worker
budgets per operation, not CPU affinity, an aggregate machine quota or a promise
of twelve busy cores; admitted dependency graphs may expose less parallel work.
Run one owner per shared build/output directory.

The local CodeQL profile, CLI configuration, extractor and JVM processor hint
also request twelve threads. Reconnect the CodeQL MCP after this change so a new
process receives the updated environment. Hosted CodeQL remains deferred. Its
existing RAM estimates and request/CPU limits still apply independently.
See the [Cargo jobs documentation](https://doc.rust-lang.org/cargo/reference/config.html#buildjobs),
[nextest scheduling](https://nexte.st/docs/configuration/threads-required/),
[CodeQL thread option](https://docs.github.com/en/code-security/reference/code-scanning/codeql/codeql-cli-manual/query-run)
and [Go scheduler setting](https://pkg.go.dev/runtime#hdr-Environment_Variables).

Validation on 30 October: the old two-worker nextest configuration failed a
twelve-party test barrier; nextest and libtest both reached it with the new
budget. The integrated portable row passed formatting, compilation, Clippy and
rustdoc, 118 ordinary tests, 51 doctests and 91 optimized test executions.
All 311 selected tooling controls, five workflow-acquisition controls and the
strict-policy fixtures passed. Evidence is retained under
`~/.local/share/nevrotick/cpu12-20261030-axla64fh/`. The Windows-mounted vendor path
stalled before execution; that owned attempt was interrupted, then the same
sixteen admitted archives/source maps were verified from existing cached bytes
into Linux storage for the successful row. The default local storage pointer was
not changed. Native Windows, fresh CodeQL execution and full coverage/mutation/
fuzz campaigns were not rerun; no acceleration measurement is claimed.

That validation also exposed inherited stale time-domain equivalence and fuzz
policy references. Independent review checked the unchanged six exact source/
proof bindings, identical sixteen-package registry records and unchanged
standalone fuzz graph before refreshing their candidate/input references.
`equivalence-rebind.json` in the evidence directory records the carry-forward;
historical campaign outcomes remain historical and were not rewritten.

## Strict development policy

Configured on 29 October 2026 using Context7 (Clippy and cargo-deny), pinned
tool help/schemas and primary upstream documentation. The policies use supported
stable rules. Broad Clippy `restriction`/`nursery` groups are deliberately excluded
because their rules can conflict or remain experimental; selected restrictions
follow the development-hardening guide.

| File | Enforced intent and current scope |
| --- | --- |
| [policy/rust-workspace.toml](policy/rust-workspace.toml) | Compiler warnings, unsafe boundaries, discarded results, unknown/stale lints/cfgs and missing public documentation fail. Clippy `all`/`pedantic` plus selected panic, conversion, unsafe, leak and exception rules fail. `missing_docs_in_private_items` also requires private-item documentation. Permanent local `allow` attributes, multiple unsafe operations per block and discarded `Result::ok()` values fail. Stable rustdoc rules fail. Optimized arithmetic remains checked; `verify-release` adds debug assertions. |
| [rustfmt.toml](rustfmt.toml), [clippy.toml](clippy.toml) | Explicit 2024 formatting and private API checks; no blanket test exemptions for unwrap/expect/panic/debug output. |
| [deny.toml](deny.toml) | Retained strict bootstrap policy, including held crypto providers; its GPL-2.0-only/no-build-script baseline remains unchanged. The UI-M0 verifier adds only the reviewed graph's grants to an owned snapshot; direct `cargo deny` against this baseline does not admit the product graph. |
| [.config/nextest.toml](.config/nextest.toml) | Twelve test workers, zero retries, fatal timeouts/leaks, bounded total duration and JUnit accounting including skips. The trusted checker rejects unknown settings because nextest itself only warns. |
| [.cargo/audit.toml](.cargo/audit.toml) | No ignored advisory IDs or severity cutoff; warnings fail. Acquisition is separate from analysis and requires an admitted database/index snapshot. |
| [.cargo/mutants.toml](.cargo/mutants.toml) | Frozen Cargo inputs, bounded timeout multipliers, no copied VCS/target, lint capping disabled. Enumeration/configuration controls are qualified; a mutation campaign is still pending. |
| [policy/verification-manifest.toml](policy/verification-manifest.toml) | Retains 90% executable-line coverage per security module and 95% mutation per critical file. Records UI-M0 preview admission and the inert header/framing owners, six critical files and Linux/Windows rows for both packages. Candidate results belong to their task records; UI-M0 follows its restricted acceptance scope. |

The Rust Cargo fragment is tested in owned temporary workspaces and is now
adopted by the UI-M0 workspace. Keep `[lints] workspace = true` in **every**
first-party member, including any future build-script/proc-macro crates. Pure
domain/protocol/numerical crates additionally use `#![forbid(unsafe_code)]`.
Windows adapters use narrowly scoped, reasoned exceptions with documented safety
contracts. Stale expectations fail. Release `panic = "unwind"` is a qualification
baseline; each component's shipping fault policy still needs its documented
decision and FFI checks.

**Contract adoption, 29 October:** [DEC-18 and section 6](docs/windows-time-architecture.md#6-domain-types-and-state-authority)
now require shared garde validation inside private checked construction, plus
explicit input/output, failure/effect and lifecycle contracts for every function
and module. [The agent enforcement guide](.agents/skills/implementation/implement-rust-contracts/references/contract-enforcement.md)
defines rule ownership, bypass restrictions and required behavioral evidence.
Garde 0.23.0 with defaults disabled and `derive` only is now part of the admitted
UI-M0 graph. Public/private documentation lints enforce presence, not the
correctness of those contracts. The following receipts concern the earlier
independent bootstrap controls, not the UI-M0 application checks.
The private-helper control passed with the old policy, failed under the new rule,
and retained a passing documented neighbor. All 37 strict-policy fixture lanes
and 24 policy tests passed; receipts are under
`~/.local/share/nevrotick/garde-enforcement-evidence/`.

`scripts/check-strict.py` admits hashes before executing tools, validates policy
invariants, freezes fixture configuration bytes and rejects stale results after
concurrent policy changes. `--fixtures` checks healthy cases and intended failures
for formatting, compiler/Clippy/rustdoc rules, unsafe exceptions, arithmetic
profiles and nextest. The unittest command also exercises actual cargo-deny,
audit, nextest and mutation enumeration with local fixtures in the cleared-env,
network-isolated sandbox. Neither command builds application code.

`python3 -B scripts/check-strict.py --workspace` checks **manifest adoption only**;
it can now inspect the actual UI-M0 workspace. It does not prove the intended
package/target inventory, feature matrix, native behavior or release readiness.
Extend the reviewed security/critical-file inventory as production owners arrive
and apply the corresponding gates. CLI evidence paths must resolve outside
the checkout; they cannot overwrite checked policy files.

Dependency controls cover local license/ban/build-script graphs and synthetic
source metadata. The audit control detects a synthetic vulnerable package in a
local committed database and accepts its patched neighbor. These controls do
not establish real registry/Git acquisition, yank checks or freshness. A successful
scan and `stale = false` are insufficient freshness evidence: require a verified
RustSec/index refresh receipt no older than 24 hours. Keep network refresh separate
from frozen, credential-free analysis. `cargo-audit` shares RustSec data with
`cargo-deny`; it is complementary evidence, not an independent advisory source.

For applicable coverage/mutation work, use the recorded package/feature/target
rows and native host where required. Coverage has no native configuration
file: the selected invocation must pass `--frozen --failure-mode any --json
--output-path <external-report> --fail-under-file-lines 90` to `cargo llvm-cov`.
Reconcile its files and executable-line denominators with the reviewed security
modules; the CLI file threshold alone does not implement module accounting.
Mutation campaigns also require explicit `--baseline run`, `--jobs`, `--timeout`
and `--build-timeout` budgets and complete per-file outcome accounting. Strict-lint
build failures are unviable mutants, not caught behavioral mutations. Missing
reports, empty selections, tool failures and incomplete feature rows remain
incomplete. These coverage/mutation application gates cannot be qualified yet.

## Repository security automation

[The security workflow](.github/workflows/security.yml) enforces workflow analysis
and redacted secret scanning. **Hosted CodeQL is explicitly deferred**; its prepared
jobs do not download, extract or analyze anything. The workflow runs on pull
requests, merge-queue candidates, main pushes, a daily schedule and manual
dispatch. Each analysis uses an ephemeral GitHub-hosted runner, read-only
repository access and checkout without persisted credentials. No job receives
signing/deployment credentials, OIDC, a self-hosted runner, a release cache or
code-scanning write permission. The always-running final gate requires inventory,
workflow-policy and secret-scan success for this candidate. It independently
requires the explicit CodeQL deferral marker and skipped, output-free CodeQL
jobs; deferral is never reported as successful SAST.

| Tool / configuration | Strict behavior |
| --- | --- |
| [CodeQL configuration](.github/codeql-config.yml) and [pins](.github/security/tool-pins.json) | Prepared, deferred lanes for Actions, Python, JavaScript and Rust. Retained settings pin the bundle/actions and select `security-extended` plus `security-and-quality` without query exclusions. Source presence does not override deferral. Re-enabling requires a reviewed workflow and independent gate-policy change. |
| [Rust preparation](.github/security/prepare_rust.py) and [admission](.github/security/check_rust_policy.py) | The new product workflow uses the private pinned-toolchain preparation; the CodeQL admission/extraction path remains inactive. Product admission and isolated execution use `scripts/verify.py`. Rust extraction can execute build scripts/proc macros even with `build-mode: none`; retained CodeQL preparation is not an ordinary local setup command. |
| [SARIF gate](.github/security/check_sarif.py) | Retained report-policy controls reject all findings, including suppressed/baseline findings, plus missing/malformed/oversized reports, failed invocations and empty source/line evidence. Its helper tests do not supply a CodeQL execution or application SAST result. |
| [Workflow checker](.github/security/check_workflows.py) and [pins](.github/security/workflow-tool-pins.json) | actionlint 1.7.12, ShellCheck 0.11.0 with optional checks, and zizmor 1.30.1 in pedantic offline mode. All reported severities fail; ignores and user configuration are disabled. Acquisition verifies archive and executable SHA-256 and copies only a bounded regular archive member. |
| [Gitleaks checker](.github/security/check_secrets.py) and [configuration](.gitleaks.toml) | Pinned executable, redaction, default detection rules, full retained-history and working-tree scans. No finding baseline or inline suppression bypass. One independently reviewed exact ASVS prose line in two documentation locations is the only project exception; token-shaped test values in those same documents remain blocking. |

Run the local configuration/control checks with:

```sh
python3 -X dev -Werror -B -m unittest discover -s .github/security -v
python3 -B .github/security/check_workflows.py
```

The workflow checker downloads verified binaries into owned temporary storage;
it does not install shared tools. For offline rechecks use `--tools-dir` with
the already acquired binaries; every reused executable is hashed again. Its
actual healthy and violation controls exercise workflow syntax, ShellCheck and
template injection before checking repository workflows. Secret-scan acquisition
and its exact invocation live in the workflow; the checker accepts only the
pinned executable digest and challenges synthetic-secret detection first.

The new [Rust workflow](.github/workflows/rust.yml) uses the same shared verifier:
it provisions bubblewrap and loads Ubuntu's packaged `bwrap-userns-restrict`
AppArmor profile on its temporary Linux runner, then exercises namespace setup
before acquisition. This preserves the runner's global AppArmor restrictions and
the verifier's isolated network and dropped capabilities. It acquires pinned tools
and reviewed vendor bytes, refreshes public advisory/index
metadata separately, checks both frozen dependency target rows, then runs the
portable product row. Windows linking and native UI observations remain separate.
The [hosted Rust run](https://github.com/DavidOsipov/NevroTick/actions/runs/40861569865)
and [security run](https://github.com/DavidOsipov/NevroTick/actions/runs/40861569914)
passed for signed commit `905c0e296f7e93fa3031f1e15e23752ea67d86e4` on 29 October.
Only after that observation was the actual Rust check added to main's ruleset.

GitHub ruleset reads on 29 October confirmed the alignment with CodeQL deferral.
[Main protection (26069359)](https://github.com/DavidOsipov/NevroTick/rules/26069359)
still protects all branches against deletion and force pushes and requires linear
history and signed commits; its deferred CodeQL requirement was removed.
[Main required checks (26110199)](https://github.com/DavidOsipov/NevroTick/rules/26110199)
requires `Security gate` and `UI-M0 portable model` on `refs/heads/main`, both bound
to GitHub Actions integration 15368 with strict status checks. Creation is exempt,
and both rulesets retain the same existing bypass actors. The update was read back
and verified. These settings do not prevent an authorized bypass; the direct
publication used the existing owner bypass while each new commit awaited checks.

Local analyzer/control results do not establish application CodeQL extraction,
complete Rust/Windows cfg coverage or hosted secret push protection.
Offline zizmor skips online metadata audits, and actionlint's
optional Python helper is disabled; deferred CodeQL supplies no Python analysis. A native
PowerShell-specific analyzer and native Windows/package mitigation observations
remain pending. Do not treat configuration alone as those qualifications.

## Local CodeQL development MCP

The [CodeQL development MCP server](https://github.com/advanced-security/codeql-development-mcp-server/tree/b0fae2809de0a5ff4fb984677354b7015b6ad57f)
**2.27.3**, CodeQL CLI **2.27.3**, Rust library **0.2.24** and Node **24.21.0**
are installed separately from the product. [The local policy](policy/codeql-mcp.json)
pins their identities and complete payload inventories. The downloaded CLI/server
retain their GitHub CodeQL Terms and notices. The Rust pack includes its exact
transitive libraries; runtime pack downloads are disabled. The local installation
does not enable hosted CodeQL or change its CI deferral.

[Project Codex configuration](.codex/config.toml) registers `codeql_dev` through
[the guarded launcher](scripts/codeql-mcp.py). Restart Codex to load it into a new
session, then inspect the exposed tool schemas. No global Codex configuration,
shared PATH or default Rust toolchain is changed. The host installation/work
paths live in `~/.local/share/nevrotick/codeql-installation.json`. Inside the
server, the private Linux work directory is `/work`; use `/work/queries` for
queries, `/work/results` for outputs and `/work/packs/rust-tools` as the explicit
builtin query-pack path. Large immutable payloads live on the recorded Windows
volume; their permissions are not a POSIX privacy boundary. Full file inventories
are checked before each launch, and payloads, compiler, checkout, configuration
and query libraries are mounted read-only inside the namespace.

The stdio-only server has no network access or host home/credentials. Its
[protocol supervisor](scripts/codeql-mcp-supervisor.py) independently enforces
20 allowed query, result, source and LSP operations. Database extraction uses the
separate launcher command below; MCP database creation, pack installation, custom
execution scripts, annotation/audit mutation and baseline acceptance are excluded.
All `additionalArgs` overrides and `learn: true` are refused. Compilation warnings
fail. Output paths must be under `/work/results`; startup admission rejects
existing output symlink/hardlink aliases. CLI positional inputs cannot smuggle
options, and per-call resource settings cannot exceed the admitted profile.
Environment flags alone are insufficient: upstream currently registers annotation
tools even with `ENABLE_ANNOTATION_TOOLS=false`; the allowlist supplies enforcement.

Protocol input is limited before upstream schema validation: 1 MiB/frame,
32 nesting levels, 256 entries/collection and 4096 nodes/request. Per connection,
output is bounded to 64 MiB protocol plus 1 MiB stderr. Each request has a
300-second wall deadline; startup allows 600 seconds. Timeout/disconnect kills
the private PID namespace. Per-process limits include 600 CPU seconds, 8 GiB
address space, 256 MiB/file and bounded file descriptors/processes. Twelve threads
and a 1024 MB RAM estimate are requested, but CodeQL raises some estimates to
2048 MB and uses its own JVM heap settings. These are **not an aggregate RAM
quota**. Exhausted budgets require investigation/reconnection, not removal of
limits or a passing-result claim. Query/test inputs remain writable where the
upstream tools place compilation and test artifacts. Startup alias inspection
does not freeze them against native code executed later: retain authored input/
expectation hashes outside the runtime workspace and compare before/after each
analysis. A changed query, fixture or baseline invalidates its evidence.

The registry precheck returned [grade F and unverified identity](https://policylayer.com/tools/codeql-development)
(last scan 24 October), recommending restrictions on `audit_clear_repo` and
`annotation_delete`; both are excluded. Independent dependency inspection found
bundled Zod **3.25.76** affected by the high-severity
[GHSA-mrhx-c7xc-m3w5](https://github.com/advisories/GHSA-mrhx-c7xc-m3w5) resource
exhaustion advisory. It remains **unfixed upstream in this installed bundle**.
Bounded input and isolated execution reduce exposure; they do not make the
installation vulnerability-free. Reassess the pinned bundle when upstream ships
a compatible fix; do not silently replace its Zod major version.

```sh
python3 -B scripts/codeql-mcp.py --check
# Only after a real Cargo manifest/lockfile and its inputs have been admitted:
python3 -B scripts/codeql-mcp.py --create-database /absolute/source --name reviewed-slice
```

Extraction accepts this checkout or owned external fixtures, creates a new
database, and seals source inputs. Rust extraction can run build scripts and
proc macros even with `--build-mode=none`. Missing offline dependencies or
unsupported target/features remain incomplete. UI-M0 now has a real workspace;
the historical fixture receipt below does not qualify its CodeQL extraction or
queries. Do not create extra workspaces for a tool check. Use
[the data-flow skill](.agents/skills/testing/qa-data-flow-analysis/SKILL.md) and
[its query/evidence reference](.agents/skills/testing/references/codeql-data-flow.md)
for explicit source/sink models, positive/negative controls, database freshness
and native/FFI coverage limits.

Qualification on 29 October used an owned std-only Rust fixture outside this
checkout: five expected functions were extracted; the interprocedural taint
query found one expected unsafe path through `relay` and excluded the constant
clean control. Actual MCP initialization, the exact 20-tool inventory, query
compilation/execution and BQRS decoding passed. The decoded results matched the
independent CLI run; SARIF contained the expected expanded call path. Five
forbidden-call controls passed, and query/fixture hashes remained unchanged.
The private receipts are `~/.local/share/nevrotick/codeql-install-evidence/`
(`mcp-qualification.json`, transcript and helper checks), with CLI extraction
receipts under the recorded work directory's `evidence/`. Forty launcher/
supervisor tests, 25 skill-checker tests and the 57-skill pack check passed.
The extractor's toolchain-detection warning remains recorded; these controls
qualify the local fixture workflow, not product coverage or native Windows.
The server's advertised `query_compile.database` parameter is incompatible with
this CLI: compile with `query` only and provide the database to execution.

The 29 October Rust refresh checked official stable tool releases and the planned
crate candidates in architecture sections 18.5–18.8. Installed Rust 1.99.1 and the
existing Cargo analysis-tool pins remain current; the selected runtime versions
also remain current. The [architecture dependency refresh](docs/windows-time-architecture.md#186-runtime-dependency-register)
updates held AWS-LC/Schannel references and preserves their provider/license/API
gates. That maintenance pass preceded the first application lockfile; the later
UI-M0 graph was admitted separately and does not lift the held provider decisions.

## Additional Rust analysis tools

[The analysis policy](policy/analysis-tools.json) and the private installation
record `~/.local/share/nevrotick/analysis-installation.json` identify the separate
analysis payloads. Use [the guarded checker](scripts/check-analysis.py), which
admits their bytes before execution; do not substitute floating nightly installs
or change the global PATH/default toolchain.

| Tool | Installed pin | Qualification and intended use |
| --- | --- | --- |
| cargo-fuzz / libfuzzer-sys | 0.13.2 / 0.4.13 | One-off GNU-target ASan controls observed: 256 healthy runs and the intended signed-byte defect. Product targets still need accepted seeds, independent invariants and target/sanitizer qualification. |
| Miri | nightly-2026-10-28 | One-off controls observed across four seeds each: healthy execution and use-after-free rejection. Product core tests remain unqualified; this is not Win32/FFI execution. |
| Loom | 0.7.2 | Isolated library payload, not a Cargo executable. One-off healthy atomic-update control and lost-update counterexample observed. Product models still need instrumented primitives and bounded state space. |
| Kani | 0.68.0 | One-off healthy/defect controls observed. Product proofs still need explicit harnesses, assumptions, unwind bounds and a non-vacuous reachable property. |
| Lockbud | a1b5303cdbb1d4d0ad886359cc71418570e7c5fd | One-off healthy/DoubleLock controls observed using experimental nightly-2026-02-07 / Rust 1.95. This older analysis compiler does not change shipping Rust 1.99.1; admit each real source/feature row before relying on diagnostics. |
| cargo-vet | 0.10.3 | One-off controls observed: std-only healthy exit 0; unreviewed Loom graph exit 255 with missing reviews and no audits/exemptions. Application audit-policy acceptance and real dependency reviews remain pending. |

The reusable guarded checker also exercised the healthy and defect controls:
an all-tool replay completed the first five lanes, then a separate cargo-vet
replay passed after correcting its store-lock setup. Their private receipts
retain the earlier setup failure as well as the successful observations. These
bounded controls do not establish application qualification.

Run only the selected tool's hash admission or bounded controls:

```sh
python3 -B scripts/check-analysis.py --tool miri
python3 -B scripts/check-analysis.py --tool fuzz --fixtures
python3 -B scripts/check-analysis.py --tool loom --fixtures
python3 -B scripts/check-analysis.py --tool kani --fixtures
python3 -B scripts/check-analysis.py --tool lockbud --fixtures
python3 -B scripts/check-analysis.py --tool vet --fixtures
```

`--tool all` selects all six installed analysis lanes. Without `--fixtures` the
checker performs hash admission only, with no version/smoke command execution.
Fixture success
qualifies only that declared control; no product Cargo workspace is created.
Select relevant tools when their guarantee changes rather than running every
analyzer for every edit. Prototype dependencies remain outside this task.

The fuzz build profile explicitly selects `--target x86_64-unknown-linux-gnu`.
The admitted prebuilt cargo-fuzz executable is built for musl and otherwise
defaults to its musl target; do not infer the intended sanitizer target from the
host or executable name. The Miri lane uses its exact admitted prebuilt sysroot.

Large payloads and complete per-file inventories under the installation's
`evidence/` directory live on an explicitly recorded `/mnt/c/...` volume because
WSL root storage is constrained. Repository analysis policy pins the inventory
SHA-256 values; the checker verifies those inventories and payload identities.
Drvfs mode `0777` does not establish filesystem privacy. Only the private
installation pointer and execution receipts reside on Linux with `0600`
permissions; actual analysis work uses owned Linux directories with `0700`
permissions. Admitted source and dependency inputs are mounted read-only with
the public OS paths.
[The runtime](scripts/analysis-runtime.py) uses a cleared environment, separate
network/PID namespaces, no credentials, private work/temp paths and bounded
wall/CPU/process/file/output limits; it has no host-execution fallback. Ordinary
analysis has a virtual-address-space cap. ASan/libFuzzer mode deliberately omits
that cap for ASan's large virtual reservation, retaining fixed finite runs,
length, timeout, RSS and allocation flags. These are not a hard aggregate RAM or
cgroup quota. Per-file limits also do not establish a total disk quota.

Cargo-vet uses the selected frozen graph with `--locked --frozen
--no-all-features --no-minimize-exemptions --no-registry-suggestions`. Missing
audits remain blocking; procurement or a real review is a separate trust
decision. Never add fake audits/imports/exemptions or widen features to silence
the tool. Even a locked check opens and rewrites its store, so the fixture lane
uses a fresh writable store copy for each case and rejects changes to its full
file inventory or bytes. For real graphs, work in an owned isolated copy and
check the store before and after execution. Before adopting any application
lane, its owner must qualify the real
harness, source/lockfile, target/features, reachable healthy/defect controls and
reported limitations; native Windows and complete-package evidence remain
separate obligations.

## Release SBOM tooling

`cargo-cyclonedx` **0.5.9** is installed in the ordinary user's Cargo bin directory
in Linux/WSL. Its separate `release_tools` entry in
[the tool manifest](policy/tool-manifest.json) pins the official prebuilt archive,
executable SHA-256, version command and Apache-2.0 tool license. The retained tool
license grants no new runtime dependency permission. Existing compiler/analyzer
binaries are unchanged; ordinary lint checks do not require the release tool.

Reinstall only this tool, without changing the toolchain or other analyzers:

```sh
python3 -B scripts/setup-unix.py --sbom-only --json /tmp/nevrotick-sbom-install.json
```

[The SBOM policy](policy/sbom.toml) selects CycloneDX **1.5 JSON**, the supported
Windows x64 target, all transitive dependencies, retained build dependencies and
strict license-expression parsing. [The generator wrapper](scripts/generate-sbom.py)
uses the actual build's feature selection. It runs frozen Cargo metadata and the
generator in an offline, credential-free, resource-bounded sandbox, with an
immutable lockfile and a private source snapshot. Its Cargo cache mounts are
read-only and exclude credentials. Reports and input/output hash receipts go to
a **new external directory**, leaving user manifests, lockfiles and source intact.

For an admitted build/feature row and the generator's required offline cache:

```sh
python3 -B scripts/generate-sbom.py --output-dir /tmp/nevrotick-rust-sbom
```

Pass `--no-default-features` and repeat `--features <actual-feature>` when those
match the build being inventoried. No universal all-features selection is used.
The wrapper checks nonempty component identity and dependency-reference closure;
these structural checks are not full CycloneDX JSON-schema validation. It also
compares each crate's component identities with an independently selected frozen
`cargo tree` for the same target and feature row. The actual controls retain
normal/build dependencies, exclude dev/other-target dependencies and exercise
optional features. Build dependencies are inventoried with the generator's
`excluded` scope, distinct from distributed runtime components.

Every actual workspace member must emit exactly one inventory with its expected
Cargo identity. Missing, duplicate or foreign roots fail. Reports are bounded to
16 MiB each, 64 MiB in aggregate and 128 files; the source snapshot is limited to
256 MiB. Oversized/linked reports are rejected before retaining their contents.

Upstream [issue 871](https://github.com/CycloneDX/cyclonedx-rust-cargo/issues/871)
is reproduced by a target-conditional feature fixture: Cargo metadata and the
generator can over-include another target's dependency. The wrapper rejects that
mismatch and emits no report. It never prunes the generated inventory to conceal
the problem. Real RED/healthy/GREEN controls cover this guard. Other project
feature rows still require qualification against their actual graph.

**Every published version must attach its prepared SBOM assets alongside the
complete MSI and final checksums.** Before the protected publisher runs, the
release owner must reconcile this Rust inventory with native objects/libraries,
generated source, resources, build-tool provenance and the actual MSI payload;
validate the final CycloneDX/SPDX document; and bind the exact version, source
candidate and **final signed MSI/payload hashes**. A Cargo inventory alone cannot
satisfy CHECK-26 or that complete-product requirement. Prepare the SBOM before
publication; do not generate a lockfile-only inventory after publishing a version.

The eventual release publisher uploads the verified attachment set into a draft
release, verifies that every required asset is attached, then publishes the draft.
The attachment step uses,
for example `gh release upload "$tag" "$signed_msi" "$complete_sbom" "$checksums"`
without `--clobber`. This is a handoff contract, not a new publication job or
authorization to upload now. The UI-M0 Cargo graph exists, but no complete MSI or
release workflow exists, so complete-package reconciliation and publication
remain pending.

## Native Windows memory diagnostics

The **Windows SDK 10.0.28000.2957** Application Verifier and Debugging Tools
components are installed on the development Windows host. The signed Microsoft
installer selected only `OptionId.AvrfExternal` and
`OptionId.WindowsDesktopDebuggers`; installation returned 0 without a restart.
The latter supplies **GFlags/PageHeap and CDB**. These are development tools,
separate from the LLVM+xwin application build route and runtime dependencies.

[Development hardening section 9.5](docs/windows-time-development-hardening.md#95-fuzzing-and-native-memory-diagnostics)
defines when to select these checks and separate retention measurements. Use
[the shared agent workflow](.agents/skills/testing/references/windows-memory.md)
for candidate ownership, evidence and restoration; the commands below configure
owned manual runs, not an automatic application CI gate.

[The native memory policy](policy/windows-memory.json) pins installer provenance,
the x64 executables and selected supporting DLLs by hash, version and Microsoft
signer. The component identifiers were checked in the signed installer's embedded
manifest. Microsoft's SDK terms apply; this setup grants no redistribution rights.
For acquisition, installation and read-only admission in native Windows PowerShell:

```powershell
# Ordinary shell: download and verify the installer without executing it.
.\scripts\setup-windows-memory.ps1 -DownloadOnly
# Elevated shell: install only the two selected components; never auto-restart.
.\scripts\setup-windows-memory.ps1
# Ordinary shell: configure CDB for the account that will run the tests.
.\scripts\setup-windows-memory.ps1 -ConfigureOnly
# Ordinary shell: verify installed files, versions and Microsoft signatures.
.\scripts\windows-memory.ps1 -Action Preflight
```

Do not change persistent execution policy to run these scripts. An administrator
account with Restricted defaults can use `powershell -NoProfile -ExecutionPolicy
RemoteSigned -File ...` for only that reviewed process. If RemoteSigned
rejects the WSL UNC checkout, use a reviewed, byte-identical local Windows copy of
the scripts and policy. An admission failure is not permission to relax pins.
The setup retains its installer/log in a new access-restricted Windows directory;
it changes neither PATH nor active per-image diagnostics. Keep logs on failure.
[The debugger startup configuration](policy/windows-debugger.xml) disables the
extension gallery, local/user gallery repositories, automatic NuGet updates,
credential-provider installation, network symbol paths and automatic settings
saving. It uses the installed engine's Version 2 schema and is installed at
`%LOCALAPPDATA%\DBG\config.xml` for the ordinary-user test account. Fresh-process
observation established that this SDK ignores the executable-directory location
described in older documentation. Preflight admits the current user's file by
hash. Elevated Enable/Disable configures only the owned image and never launches
CDB, so it does not require the administrator account's debugger preferences.
An existing different configuration is preserved for explicit reconciliation;
only an exact previously admitted task-owned version can be atomically replaced,
with its old bytes retained in a backup.
These settings are not a network sandbox for arbitrary debuggee/extension code.

For a reviewed test executable, the guarded profile enables **Application Verifier
Basics plus full PageHeap**. Configuration requires elevation and persists until
removed. The helper admits only local AMD64 images with a unique
`NevroTick-<12–64 lowercase hex>.exe` basename. It rejects existing settings or
running processes with that name and keeps recovery receipts under protected
`C:\ProgramData\NevroTickWindowsMemory` custody. Use exclusive per-image
configuration ownership; this is a cooperative development guard, not protection
against another administrator changing the machine concurrently.

```powershell
# Elevated configuration shell; the helper never launches the candidate.
.\scripts\windows-memory.ps1 -Action Enable -ImagePath 'C:\owned-fixture\NevroTick-0123456789abcdef.exe'
# Run the candidate under the admitted CDB in a separate ordinary-user shell.
# Always stop the owned process and restore the profile, including after failure.
.\scripts\windows-memory.ps1 -Action Disable -ImagePath 'C:\owned-fixture\NevroTick-0123456789abcdef.exe'
```

The paths above are examples, not shipped application binaries. Do not rename an
application silently if its filename affects behavior. Keep the policy unchanged
until cleanup; settings changed outside the recorded transaction need explicit
reconciliation. Disable remains possible if the candidate file was removed.
Keep debugger logs, actual process exit/stop evidence, `!avrf` provider state,
image hashes and the final absence of per-image settings. Empty XML output or
CDB exit 0 alone does not establish success. The helper configures profiles;
it does not launch or supervise CDB/the candidate. The workload owner must bound
execution duration, memory/output and process cleanup; the helper's configuration
command timeout and post-collection output check are not live workload limits.
Do not debug untrusted code with an elevated token.

Use a fresh owned output directory and reviewed debugger command file. The
qualified CDB options are `-noshell -nosqm -noinh -netsyms:no -sins -y
<owned-local-symbol-directory> -G -cf <command-file> -logo <fresh-log>` followed
by the executable and its arguments. This build's help misspells the accepted
`-netsyms:no` switch; the singular form and a separate `no` argument fail before
the fixture. Capture access violations, verifier breakpoints and invalid-handle
exceptions explicitly, rather than letting unattended debugger input end the run.
Windows component symbols may remain unavailable with network symbols disabled;
retain that stack-resolution limit. Run unattended elevated configuration with
hidden windows and redirected output so console selection cannot pause cleanup.

Full PageHeap's default guard targets overruns; an independent backwards profile
would be needed for underrun coverage and is not enabled by this helper. The
GFlags `/leaks` option disables full PageHeap, so do not combine it with this
profile. AppVerifier's Leak layer reports at module unload; `!avrf -leak` lists
outstanding resources, which may still be legitimately owned. Neither facility
replaces repeated lifecycle tests, allocation/handle accounting and the planned
resource soak. Custom allocators can escape Windows heap instrumentation.
Measure ordinary resource budgets separately from instrumentation overhead.

Read the [agent workflow](.agents/skills/testing/references/windows-memory.md)
before selecting native tests. The UI-M0 workspace exists; its reached native
paths need their own observations and are not qualified by these tool controls.
UI-M0 remains fixture-only. Native control
evidence is retained outside the checkout under
`%LOCALAPPDATA%\NevroTick\windows-memory-20261029`. The bootstrap controls use a
fixed dependency-free Rust executable, native Windows build 10.0.26300 and an
ordinary-user debugger; installation/configuration uses the separate elevated
account. They do not qualify release Windows versions, application code, all
allocators, the Leak layer or sustained resource retention.

The final native replay (`round-7/classification.json`) passed: the healthy
heap/event lifecycle completed, the deliberate one-byte heap overrun stopped
with `0xc0000005`, and the double-close control stopped with `0xc0000008`.
All three ran unelevated with full PageHeap `0x3` and verifier flags `0x81643027`.
Fresh CDB startup and settings output confirmed the disabled extension/download
preferences. No control timed out; both IFEO registry views were absent after
cleanup. CDB returned 0 even for caught faults, so classification used the
actual exception/stack and standalone control markers, not that exit code.
The configuration helper's **36 native controls** also passed, including real
ACL/exclusive-lock behavior and rejection of unexpected configuration changes.
Earlier setup/harness failures and their corrections remain in the same external
evidence directory; they were not counted as successful native controls.

For removal, disable and verify every owned image while the tools are still
installed, then use Windows SDK maintenance to remove the selected diagnostic
components. Remove the current user's debugger config only when its bytes match
the owned policy; preserve other settings and the retained receipts. Portable
Rust checks remain available but cannot replace the removed native observations.

Primary references: [SDK downloads](https://learn.microsoft.com/en-us/windows/apps/windows-sdk/downloads),
[Application Verifier](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/application-verifier),
[GFlags commands](https://learn.microsoft.com/en-us/windows-hardware/drivers/debugger/gflags-commands),
[AppVerifier FAQ](https://learn.microsoft.com/en-us/windows-hardware/drivers/devtest/application-verifier-faqs).

## Windows development route

The selected route is **LLVM + xwin in WSL**, retaining the documented MSVC ABI.
Visual Studio workloads remain absent. The original cross-build preparation did
not install a system SDK; the separate diagnostic components described below
were installed on 29 October 2026 without changing the build route.
The linker is Rust's bundled rust-lld, matching LLVM **23.1.1**. Microsoft CRT/SDK
library acquisition remains subject to Microsoft's terms independently of
xwin's own licence. The compact route's exact inputs and checks are recorded in
the tool manifest.

The selected stable catalog supplies SDK selector **10.0.28000** and CRT selector
**14.51** (CRT payload **14.51.36254**). Selectors and exact catalog/payload
identities are separate; the manifest and setup receipt retain both.

Acquire the reviewed libraries into an explicit cache on the Windows drive when
that setup is needed:

```sh
python3 -B scripts/setup-xwin.py --cache-dir '/mnt/c/Users/<Windows-user>/AppData/Local/NevroTick/xwin-cache' --accept-license
```

The setup prints the library location and retains acquisition evidence outside
the checkout. The wrapper adds only the three admitted library paths and
preserves the compiler's link arguments, including the target hardening flags.
It does not change the global Cargo configuration or default target. For UI-M0,
use the shared `windows` row below, which additionally admits the complete library
inventory and builds inside the isolated product environment.

Setup prunes verified payload copies and matching unpack intermediates by
default; `--keep-cache` retains them for offline reacquisition. The installed
SDK/CRT headers and libraries occupy about **636 MiB**; the complete cache,
including provenance and the qualification fixtures, about **717 MiB**. The
cache is on the Windows drive. This cross-build acquisition used no Visual
Studio or system SDK installer; native diagnostic installation is separate.

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

## Application optimization settings

The workspace's release candidate uses `opt-level = 3`, `lto = "thin"` and
`codegen-units = 1`; `verify-release` inherits those settings and enables debug
assertions. Overflow checks, unwind qualification policy, limited debug symbols,
no stripping and Windows CFG/ASLR/DEP flags remain required. Development builds
retain Cargo's incremental defaults. These settings favor runtime optimization
opportunities at higher compile/link cost; no speedup has been measured.

The currently admitted Windows configuration still uses `target-cpu=x86-64`.
[OD-07](docs/nevrotick-owner-decisions.md#11-od-07--x64-baseline-and-runtime-optimization)
now selects an intended v3 minimum and required runtime-selected optimization.
The [execution plan](docs/plans/development-plan.md#81-x64-baseline-and-runtime-optimization-workstream)
owns migration of `.cargo/config.toml`, strict checker/control expectations,
affected input bindings, skills guidance and qualified Windows artifacts.
This documentation change implements no CPU flag or dispatch function; do not
copy v3 into the current configuration without that coordinated admission.
Preserve all mitigations and use exact-feature functions for optional AVX-512
work, rather than global v4 or `native` compilation. AVX2 is already part of the
intended v3 baseline. This does not
rebuild the supplied standard library for another CPU. The policy checker still
rejects unreviewed CPU/profile overrides, including ambient PGO flags that would
replace the complete mitigation vector. See [hardening §6.7](docs/windows-time-development-hardening.md#67-v3-baseline-and-runtime-cpu-dispatch)
for compatibility, numerical and measured-benefit gates.

The guarded `scripts/verify.py --lane windows` row retains compiler/Clippy checks
and links the UI with `--release`, exporting the executable and its matching PDB
with profile and digest identity. The direct Windows allocation observer mirrors
the optimized profiles. These paths require fresh receipts after this change;
older debug/native receipts do not qualify the new release artifact.

See [hardening section 6.5](docs/windows-time-development-hardening.md#65-windows-x64-optimization-policy-and-evidence-limits)
and the [build optimization task](tasks/build-optimization.md) for compiler-selfbuild
issue scope, conditional native PGO, BOLT/PE limits, checks and remaining evidence.
Use the admitted commands below rather than ambient `RUSTFLAGS` or a host-build
fallback. A configured profile, cross-link or fixture pass establishes no native
performance, complete mitigation, installation or release result.

The admitted Rust 1.99.1 compiler embeds LLVM 23.1.1. Its optimized pass pipeline
and rustc-managed MSVC linker defaults remain selected; no extra raw LLVM flags,
forced inlining/unrolling/SIMD widths or relaxed floating-point assumptions are
adopted. [Hardening section 6.6](docs/windows-time-development-hardening.md#66-llvm-settings-for-this-workload)
records the source-backed choices. Recheck the actual bundled backend after a
toolchain update instead of installing an unrelated LLVM or copying Clang options
into `RUSTFLAGS`. Diagnostics and comparison profiles require their own admitted
scope; a compiler accepting an option establishes no performance benefit.

## Starting implementation

Start with [docs/README.md](docs/README.md) and the selected task. UI work uses
architecture §16.5 and [the UI-M0 task record](tasks/ui-m0.md); pure wire work
uses [S1.1a](tasks/s1.1-header.md) and [S1.1b](tasks/s1.1-framing.md);
checked numerical/domain work uses [S1.1c](tasks/s1.1-time-domain.md).
The minimal [workspace](Cargo.toml)
contains [nevrotick-ui](crates/nevrotick-ui/Cargo.toml): a portable,
checked preview model and a Windows-only native adapter. Its fixture data and
edits are in memory only. There is no live network/DNS, production IPC,
persistence, service, elevation or clock effect. D1 display/theme, D2 production
state and D3 full accessibility remain deferred at the canonical re-entry gates.
The second member, [ntp-wire](crates/ntp-wire/Cargo.toml), owns inert exactly
48-byte fixed-header representation and the bounded no-legacy-MAC framing
implementation. It has no external dependencies. Candidate-specific framing
results belong to S1.1b. The third member,
[time-domain](crates/time-domain/Cargo.toml), owns explicit checked coordinates,
numeric era windows/resolution, exact offset/signed RTT and checked rounding,
plus distinct elapsed-clock units. Its sole external dependency is the existing
minimal garde declaration. Numeric arithmetic grants no accepted-exchange,
source/authentication, date-truth or correction authority; those owners and NTS
remain future work. Its qualified bounded candidate and evidence limits belong
to S1.1c. The fourth member,
[ntp-exchange](crates/ntp-exchange/Cargo.toml), joins the two pure owners for the
[S1.1d single-slot fixture task](tasks/s1.1-exchange-association.md). Its local-only
graph and extended runner have separate admission; checked inputs and rejecting
transition stubs are the initial test baseline. Candidate-specific behavioral
results belong to that task.

The exact [lockfile](Cargo.lock), manifest/feature identities, sixteen external
packages, selected license grants and source-file maps are recorded in
[the UI-M0 dependency policy](policy/ui-m0-dependencies.json). Garde uses only
`derive`; the five selected `windows-sys` feature groups are Windows-only.
On 29 October, `dependency-check-6.json` recorded successful frozen advisory,
license, source and build-script checks for Linux and Windows. The receipt lives
under `%LOCALAPPDATA%\NevroTick\ui-m0`; this proves that graph at its recorded
snapshot, not future advisory freshness or native UI behavior. Current application
results and unresolved observations belong in the task record.

The pure wire owners' additional evidence uses
[verify-wire-analysis.py](scripts/verify-wire-analysis.py) and
[its scoped policy](policy/ntp-wire-analysis.json), after the ordinary graph and
portable rows. Select `--lane contracts`, `coverage`, `mutation`, `fuzz` or `all`,
and `--subject header`, `datagram` or `all` (default) for coverage/mutation scope,
with `--output-dir /new/private/Linux/output` and, when selecting another copy,
`--vendor-dir /admitted/vendor`. The output path must be new. These are owned
isolated analyses with explicit source/tool/graph inventories, not host Cargo
fallbacks or installation commands. The repository's existing analysis installation
and matching payload inventories are required.

The coverage row reports each selected security file's executable-line denominator
from one actual package run. Mutation reconciles separate complete nonempty
per-file enumerations and treats build failures as unviable; the selected campaign
shares the fixed total mutant/time budgets. External consumer checks require a
compiling legal caller and exact privacy/borrow error causes. Contracts and fuzz
always cover both wire owners; `--subject` never narrows those lanes. The fixed
GNU-host fuzz harness separately admits
its locked source/features and native LLVM-exception notice, uses archive-verified
vendor metadata and retains the 256-run sanitizer profile plus seeded fault
control. This test-only graph is not production NTS/TLS or distribution admission.
Current successes, first failures and incomplete rows remain in
[S1.1a's candidate record](tasks/s1.1-header.md) and
[S1.1b's execution record](tasks/s1.1-framing.md); the CLI's existence is no pass.

The domain owner's equivalent explicit-scope entry point is
[verify-time-domain-analysis.py](scripts/verify-time-domain-analysis.py), with
[its policy](policy/time-domain-analysis.json). It accepts the same five lanes,
and `--subject coordinate`, `era`, `exchange`, `clock`, `rounding` or `all`.
Contracts and fuzz cover the complete domain scope; selected subjects narrow
only coverage/mutation. Its standalone test-only fuzz graph independently admits
garde's exact closure and libFuzzer, without adding shipping dependencies.
Use a new private Linux output and the admitted vendor; actual results belong to
S1.1c. Tooling admission alone establishes no numerical or Windows behavior.

The pure fixture core uses [verify-core-analysis.py](scripts/verify-core-analysis.py)
and [time-core-analysis.json](policy/time-core-analysis.json). Its existing
five-owner admission covers regression, residuals, normalization, history and
runs; the new selection owner requires a revised integrated admission before a
six-owner claim. The [S1.2 concurrency record](tasks/s1.2-regression-readiness.md#user-requested-mutation-concurrency-trials-30-october-2026)
records the qualified ten-worker regression pilot and failed lower-limit trials.
The core selects ten outer workers, isolated mutant targets, one inner Cargo/test
slot and a fixed 512-task sandbox limit for admitted mutation campaigns.
Other invocations retain 128; this changes no shipping compiler flags, NTS job
limit or quality threshold. The 131 persistent profile/accounting controls passed;
the expanded candidate's actual analysis results remain pending. A healthy pilot
is no full pass.

The separate [time-domain allocation observer](scripts/verify-time-domain-windows-allocation.py)
has two explicit lanes. Build with `--lane build --profile release`, the admitted
`--vendor-dir`, `--toolchain-dir`, `--xwin-config`, `--xwin-inventory` and a new
private Linux `--output-dir`. This retains the guarded MSVC library build,
observer format/Clippy/link results, immutable emitted dependencies and exact
artifact/source/runtime inputs. Then select `--lane native --build-receipt
/path/to/build/verification.json --native-dir /mnt/c/Users/USER/AppData/Local/NevroTick/NEW
--output-dir /new/private/Linux/native-receipt`. Both destinations must be new;
the native lane rejects stale build/source/helper bindings.

This test-only Windows workload checks ordinary supervisor/child tokens before
releasing its exact startup gate, and records 25 samples covering 22,282 era-window
constructor calls with allocation controls. It observes requested payload and
retention before process teardown, separately requiring stream/process release
and an empty owned job. It does not alter AppVerifier/PageHeap/registry settings
or establish native heap overhead, RSS, universal allocation counts, absence of
leaks, clock behavior or release qualification. Actual failures and completed
measurements belong to [S1.1c](tasks/s1.1-time-domain.md).

Use [scripts/verify.py](scripts/verify.py) from the checkout root. With the external
inputs configured, the default command is the portable row:

```sh
python3 -B scripts/verify.py
```

| Row | Work and evidence limit |
| --- | --- |
| `--lane portable` (default) | Format, compiler, Clippy and rustdoc for the admitted workspace members; separate per-package library/integration tests and doctests, plus the configured optimized owner tests with their release-only discriminators. Missing, empty, skipped or wrong-package selections fail; package and aggregate counts are retained. |
| `--lane dependencies --advisory-receipt /path/to/current-receipt.json` | Frozen Linux and Windows graph checks using a separately acquired RustSec/index snapshot, bound to the lockfile and at most 24 hours old. |
| `--lane windows --output-dir /path/to/new-build-output` | Windows-target compiler/Clippy and release binary linking; copies the executable, matching PDB and receipt to a new external directory. Native execution remains a separate observation. |

Rows run with a read-only source snapshot, cleared environment, isolated network
and owned output inside bubblewrap. The verifier admits pinned tools and source
bytes, checks sandbox boundaries, records effective checker/configuration hashes
and rejects input drift. Resource/time/output bounds remain distinct from hard
cumulative quotas. There is no host-execution fallback and no automatic download.
`--json /path/to/new-receipt.json` retains the selected row's output. The nextest
store is redirected only in the owned snapshot so reports stay in writable output;
test policy and the repository configuration remain unchanged.

For first setup, select new external destinations and run acquisition separately:

```sh
python3 -B scripts/prepare-product.py --vendor-dir /path/to/new-vendor --configure --scratch-root /path/to/build-scratch
python3 -B scripts/refresh-product-advisories.py --cargo-home /path/to/new-metadata --receipt /path/to/new-metadata-receipt.json
python3 -B scripts/verify.py --lane dependencies --advisory-receipt /path/to/new-metadata-receipt.json
python3 -B scripts/verify.py
```

`prepare-product.py` verifies bounded archives and their reviewed extracted-file
maps without executing package code. `--configure` requires an absent private
`~/.local/share/nevrotick/ui-m0.json` pointer; it never replaces an existing one.
The verifier reads `vendor_dir` and `scratch_root` from that pointer, while explicit
`--vendor-dir` and `--scratch-root` select another admitted location. Put large build
outputs on the selected Windows drive when appropriate. A small owned Linux copy
of vendor/metadata inputs can avoid DrvFS per-file overhead; preserve and verify
the same file maps. Refresh metadata into new locations when needed and pass the
new receipt explicitly. Portable/Windows rows do not silently refresh or rerun the
dependency row; complete that admission before executing newly admitted graph code.

The root [deny.toml](deny.toml) deliberately retains the bootstrap GPL-only and
no-build-script baseline. The dependency row generates a configuration solely
inside its owned source snapshot: GPL-2.0-only, MIT, BSL-1.0 and Unicode-3.0 grants
for this frozen graph, and exactly `proc-macro2@1.0.107`, `quote@1.0.47` and
`rustversion@1.0.23` build-script allowances. The `MIT AND Unicode-3.0` conjunction
is preserved. All held-provider, advisory, source and other ban rules remain.
These license identifiers are scoped by the immutable graph, not package-specific
license exceptions. Running direct `cargo deny` with the root baseline will reject
this product graph; use the shared dependency row.

The Windows row additionally needs `--xwin-config` and `--xwin-inventory`, or the
corresponding pointer values. Its full selected-library inventory is bound to the
acquisition receipt and checked before/after linking. The current installed-input
inventory does not reproduce archive-to-installed parity: the earlier verified
acquisition pruned its payload copies. Target CFG/ASLR/DEP flags are retained from
[.cargo/config.toml](.cargo/config.toml); inspect the emitted image before claiming
effective protections.

The configured host's reviewed inventory is
`%LOCALAPPDATA%\NevroTick\ui-m0\xwin-installed-inventory.json`, selected by its
private pointer. It covers all 529 files in the admitted x64 CRT, SDK UM and SDK
UCRT library directories. Neither `setup-xwin.py` nor `prepare-product.py`
generates this inventory. A fresh host must separately review its acquisition
receipt and installed library bytes, then record that inventory before the Windows
row can run; copying an unrelated host's inventory or merely hashing unreviewed
installed inputs does not establish their provenance. Portable verification does
not require the Windows inventory.

For native observations, use the ordinary Windows account with exclusive desktop
interaction. Copy [ui-m0-windows.ps1](scripts/ui-m0-windows.ps1) to a local Windows
path and verify its SHA-256 matches the repository file; also match the executable
to the cross-build receipt. This avoids a RemoteSigned rejection of a WSL UNC
script without changing execution policy. With those local paths:

```powershell
powershell -NoProfile -File 'C:\owned\ui-m0-windows.ps1' -ImagePath 'C:\owned\nevrotick-ui.exe' -OutputDirectory 'C:\owned\new-ui-m0-observations'
```

`ImagePath` must identify the owned local executable; `OutputDirectory` must be
new. The helper observes controls and window lifecycle and records bounded resource
samples. Its existence or a cross-build is not a passed native run, accessibility
qualification, sustained leak/soak result, P0 completion or release evidence.
Keep TLS/provider, production-license, service/clock laboratory and signing holds.

## Code-navigation tools

Serena's project configuration selects Rust; use initial_instructions and the
returned Serena session before activation. SocratiCode uses the absolute checkout,
current indexing status and the registered document contexts. Probe supplies
Rust search/extraction without a persistent index. Context-mode processes large
output; Context7 and primary upstream sources resolve API questions. Root
[AGENTS.md](AGENTS.md) describes when to choose each tool. Existing connection
history is not a fresh runtime qualification. UI-M0 now supplies Rust source for
navigation; confirm current indexing before relying on its application graph.
