# Rust development environment

**UI toolkit decision, 5 November 2026:**
[OD-13](docs/nevrotick-owner-decisions.md#17-od-13--slint-as-the-default-ui-toolkit)
selects Slint with the measured winit/software and Fluent-style starting
configuration. Current repository UI commands still check the Win32 reference
implementation; they establish no Slint qualification. The [UI-M0 task](tasks/ui-m0.md)
owns exact graph/licence admission, generated-code/lint-boundary review and the
migrated candidate's checks. No toolchain, Cargo graph or native execution
authority changes through this documentation amendment.

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

Those dated bootstrap and first-crate records are historical. The current
[workspace manifest](Cargo.toml) and [lockfile](Cargo.lock) cover six members;
the [implementation summary](docs/windows-time-architecture.md#1-current-state-capsule)
routes each member to its task evidence. Use the
[development plan's current task](docs/plans/development-plan.md#7-next-executable-task-for-a-development-agent)
for execution order and applicable holds. Later source changes require the
candidate-specific evidence described in those tasks, not automatic reuse of
the original bootstrap results.

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
python3 -B scripts/check-strict.py --pure-api-fixtures --json /tmp/nevrotick-pure-api.json
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

**Four-rule cohort, 3 November 2026:** the workspace and canonical fragment now
deny `let_underscore_drop`, `unused_macro_rules`, `unused_lifetimes` and Clippy
`string_slice`, inherited by all six members. Pinned-tool enforcement passed42
strict and6 pure-API fixture lanes, including a combined healthy control and four
intended rejections. Policy/admission Python controls passed37/38 tests. The
new139-input candidate passed22 portable rows:476 ordinary tests,127 doctests
and368 optimized tests. Packages, features, grants, lockfile, profiles, toolchain
and target flags remain unchanged; only the required root-manifest identity was
refreshed in dependency admission. `panic_in_result_fn` stays deferred. See the
[exact Core checkpoint](tasks/s1.2-regression-readiness.md#s156-four-lint-compiler-cohort--complete)
for before/after controls, identities and retained failures. These are static and
portable observations; Windows execution remains held.

| File | Enforced intent and current scope |
| --- | --- |
| [policy/rust-workspace.toml](policy/rust-workspace.toml) | Compiler warnings, unsafe boundaries, discarded results, unknown/stale lints/cfgs and missing public documentation fail. Clippy `all`/`pedantic` plus selected panic, conversion, unsafe, leak and exception rules fail. `missing_docs_in_private_items` also requires private-item documentation. Permanent local `allow` attributes, multiple unsafe operations per block and discarded `Result::ok()` values fail. Stable rustdoc rules fail. Optimized arithmetic remains checked; `verify-release` adds debug assertions. |
| [rustfmt.toml](rustfmt.toml), [clippy.toml](clippy.toml) | Explicit 2024 formatting and private API checks; no blanket test exemptions for unwrap/expect/panic/debug output. |
| [time-domain](crates/time-domain/clippy.toml), [ntp-exchange](crates/ntp-exchange/clippy.toml), [time-core](crates/time-core/clippy.toml) role policies | Preserve all five common Clippy settings and reject selected ambient clock/configuration, filesystem, socket, process/thread, printing and build-environment APIs. The existing `all = deny` activates these style lints. Root/adapter policy retains its separate scope. |
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

**Pure-role API enforcement, 31 October 2026:** the three package-local Clippy
policies participate in strict admission and product source snapshots. Clippy
uses the closest configuration rather than merging parent settings; the checker
therefore requires the complete common settings and exact role lists, and rejects
alternate or nested configuration files. Ambient `CLIPPY_CONF_DIR` remains
rejected. `--pure-api-fixtures` selects six dependency-free native Clippy
commands: a healthy workspace, identical capabilities accepted in an adapter,
and intended method/type/macro rejection in each pure role, including aliased
clock/environment/file functions and a clock function-reference bypass control.
`--fixtures` includes these controls alongside
the existing 37 lanes. The focused command admits only the tools it uses.

The observed baseline accepted ambient clocks in all three pure roles; the new
controls reject them. All 43 policy lanes and 107 focused Python tests passed.
Current `time-domain`/`ntp-exchange` library Clippy also passed. At adoption, full
workspace Clippy remained incomplete on the in-progress combination prerequisite's
unused square-root helper and documentation markup. Subsequent kernel work added
the real square-root consumers, corrected the markup and passed the full portable
row, including workspace Clippy: 284 ordinary tests, 56 doctests and 257 optimized
tests. The new numerical owner's per-file qualification remains pending.
Receipts and source scope are recorded in
[the S1.3 task](tasks/s1.3-source-selection-readiness.md#external-proposal-assessment-31-october-2026).
These named bans do not prove complete effect freedom through wrappers,
dependencies or native APIs. Fixture accounting retains its deliberate
`stderr().write_all` observer. No new analyzer or dependency was installed.

**Bounded Python verification typing, 31 October 2026:**
`scripts/check-python-typing.py` checks seven selected helpers: combination and
cohort accounting, chart accounting/contracts, and chart-combination
accounting/contracts, and settled-rate accounting, plus an independent consumer. The checker normalizes
their hyphenated filenames into private read-only input copies and checks
their actual exported functions against maintained Protocol interfaces.
It does not type-check the complete dynamic orchestration script. Runtime log,
JSON, exact-integer, range, identity and completeness checks remain required.

The local checker is Python 3.12.3 with mypy 2.4.1 and a closed six-wheel graph in
`policy/python-typing.json`. The isolated environment is under
`~/.local/share/nevrotick/python-typing-20261031/venv`; no system package or Cargo
dependency was added. The entry point requires an external admission receipt
binding every installed file, selected source byte and policy byte. It rejects
stale bindings, ambient checker configuration, unchecked imports/typing escapes
and writable source aliases; it installs or rewrites nothing.

For a change to a selected helper, run the admitted healthy typing check:

```sh
python3 -B scripts/check-python-typing.py \
  --checker-python /home/david_osipov/.local/share/nevrotick/python-typing-20261031/venv/bin/python \
  --admission /path/to/current-external-admission.json
```

For checker/configuration/interface changes, also select `--fixtures`: one
healthy consumer and nineteen intended static rejections cover wrong fields/value
types, reservation/observation confusion, partial/complete confusion, unhandled
statuses and an incompatible actual implementation. Lifecycle-specific controls
also reject reserved attempts as observations and partial records as complete.
The S158 extension includes chart-combination recipe and callback types,
reservation/outcome separation, incomplete-status handling and actual signature
compatibility for both new helpers. Its configured check passed seven files and
all fifteen negative controls on that earlier candidate. The settled-rate
extension selects eight checked files and adds four controls for the new helper's
field types, reservation/outcome separation, incomplete-status handling and
actual signature. Its current executed result belongs to the existing Core task;
configuration alone does not establish a typing pass.
Use `--json` with a fresh
external result path to retain the receipt. Refresh admission only after reviewing
the changed inputs; source hashes are not a substitute for those checks. A
complete counter transport establishes neither successful assertions nor product
qualification. Current execution evidence belongs to the existing S1.3 task.

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

[OD-10](docs/nevrotick-owner-decisions.md#14-od-10--mandatory-mathematical-proofs)
requires actual-source mathematical proofs under
[architecture §20.1.1](docs/windows-time-architecture.md#2011-mandatory-mathematical-proofs--5-november-2026).
Follow the [proof workflow](.agents/skills/testing/references/formal-verification.md);
the [rounding/era driver](tools/formal/README.md) is the first bounded campaign,
not a workspace-wide proof gate. Tool admission or its bootstrap controls alone
cannot close the per-algorithm obligations. Missing proofs remain open in the
existing source-owner task before dependent acceptance.

**Verus candidate, 5 November 2026:** the
[bounded Core addition pilot](tasks/s1.3-source-selection-readiness.md#verus-addition-pilot--5-november-2026)
was justified after Kani's recorded zero-check timeout. The private official
release `0.2026.11.02.8bf68ee` was acquired and admitted for bounded readiness
using existing Rust 1.99.1 and bundled Z3 4.16.0. **Candidate DEFER:** its final
compatibility command rejected the required `u64::checked_shl` call before SMT.
Exactly three preparation commands ran; no product proof ran or adoption followed.
The source guards and controls are in `tools/formal/verus_pilot*.py`; payload,
licences and raw receipts remain in the private directory named by the Core task.
The redundant archive was deleted. No global rustup/PATH/SDK or analysis-policy
change occurred. The frozen command used an offline namespace, read-only SDK,
one verifier thread, a real SMT timeout and `--no-cheating`. Do not restart the
exhausted trial or add an assumed specification/runtime rewrite. Reconsider only
after supported actual APIs and a separate finite checkpoint. Existing Kani
entrypoints and scoped evidence remain in force.

The separate [6 November Kani contract-mode pilot](tasks/s1.3-source-selection-readiness.md#addition-contract-mode-terminal-assessment--6-november-2026)
also ended **DEFER** at its first primitive checker: bundled CBMC aborted on a
closure-symbol type inconsistency, with zero checks, product proof commands or
callee certificates. The dedicated source consumer is unadopted; ordinary Kani
flags and the SDK are unchanged. Keep this candidate terminal and do not retry or
patch the tool under its closed card. Full-domain addition remains open.

**Latest Core proof checkpoint, 6 November 2026:** [complement v2](tasks/s1.3-source-selection-readiness.md#complement-v2-terminal-qualification--6-november-2026) passed all seven jobs: four full-raw predicate equalities/three arithmetic lemmas, five isolated contract failures and restored health. All143 selected checks pass;30 source files/11 executed Python sources and final source/input/receipt/tool guards bind the candidate. The captured-source `verify_fraction_complement_post.py` attempt is closed; do not rerun or delete its root. V1's opaque-power preprocessing failure remains historical. All earlier Rust pins are preserved, with no certificate imported. Next prepare rational-order alignment through pinned primitive cmp models; Ordering PartialEq/caller import, complete bridge/addition and Core/P1 remain open. No profile, SDK, CI or Windows execution hold changed.

| Tool | Installed pin | Qualification and intended use |
| --- | --- | --- |
| cargo-fuzz / libfuzzer-sys | 0.13.2 / 0.4.13 | One-off GNU-target ASan controls observed: 256 healthy runs and the intended signed-byte defect. Product targets still need accepted seeds, independent invariants and target/sanitizer qualification. |
| Miri | nightly-2026-10-28 | One-off controls observed across four seeds each: healthy execution and use-after-free rejection. Product core tests remain unqualified; this is not Win32/FFI execution. |
| Loom | 0.7.2 | Isolated library payload, not a Cargo executable. One-off healthy atomic-update control and lost-update counterexample observed. Product models still need instrumented primitives and bounded state space. |
| Kani | 0.68.0 | Source-bound rounding/valid-window era and named domain/helper proofs have scoped qualified checkpoints in the [current task](tasks/s1.1-time-domain.md). Public Garde-constructor proofs and remaining leap/Core obligations are open; tool controls alone establish no product proof. |
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

## Standalone sanitizer detector controls

The separate [sanitizer checker](tools/sanitizers/check.py) and
[payload policy](tools/sanitizers/policy.json) configure bounded GNU/Linux
detector controls. The four Rust profiles use an isolated
[`nightly-2026-10-30` bundle](https://static.rust-lang.org/dist/2026-10-30/channel-rust-nightly.toml),
the newest published nightly checked on 31 October, with LLVM **23.1.4**.
Native C++ UBSan uses Clang **22.1.8**. Rust linking follows an explicit
GCC13/BFD route. The private, mode-0600 pointer is
`~/.local/share/nevrotick/sanitizer-installation.json`.

The five selections are `asan`, `lsan`, `tsan`, `msan` and native C++ `ubsan`.
They are separate from `check-analysis.py`'s six existing analysis selections;
Miri continues to use that existing checker and sysroot.

```sh
python3 -B tools/sanitizers/check.py --tool asan
python3 -B tools/sanitizers/check.py --tool all --fixtures --output-dir /new/private/Linux/output
python3 -B tools/sanitizers/test_check.py -v
```

The first command admits hashes only. Fixture execution requires a new external
Linux directory and emits `verification.json`, retaining native commands, options,
outputs, artifact hashes and setup/detector failures. Select the useful tool;
`all` is not a required per-change campaign. No arbitrary flags, source programs,
inherited environment or unsandboxed fallback are accepted.

Rust ASan/LSan use standalone `no_std` fixtures. TSan/MSan additionally use
matching instrumented `core` and `compiler_builtins`, rebuilt separately from the
pinned Rust sources with Cargo `-Zbuild-std=core,compiler_builtins` and its complete
vendored dependency set. Their split `.rlib`/`.rmeta` files and corresponding
runtime archives are pinned in separate sysroots. These controls qualify neither
an instrumented Rust `std` nor an application/native-dependency graph. C++ UBSan
uses its own Clang runtime; no mixed Rust/C++ runtime compatibility is inferred.

The checker retains offline namespaces, read-only payloads/fixtures, fresh owned
outputs, bounded capture and owned-process cleanup. Only the fixed detector
executables receive the standalone shadow-address-space exception; CPU, wall,
process, file and output limits remain. This is separate from the existing
libFuzzer resource profile. Qualified detector evidence requires the actual
expected diagnosis and source site plus a successful healthy neighbor; a panic
or generic crash is insufficient.

The compiler refresh advances Rust to commit
`92b5b8c87c61b20adc9001b41baf587634a74734`. Both the previous October 28 and
current October 30 bundles use LLVM/compiler-rt revision
[`f738f3b0ed65c01edcb44a504a77edc12a82c319`](https://github.com/rust-lang/llvm-project/blob/f738f3b0ed65c01edcb44a504a77edc12a82c319/cmake/Modules/LLVMVersion.cmake).
All four ASan/LSan/TSan/MSan runtime archives are byte-identical across the
refresh; this supplies no newer sanitizer runtime fixes. The TSan/MSan
instrumented libraries were rebuilt with the new compiler and its sources.

Verified distribution archives, source/vendor checks, offline build receipts,
runtime comparisons and activation evidence are retained under
`~/.local/share/nevrotick/sanitizer-update-20261031-4g1800_z/`.
The isolated compiler and separate `sysroots/` are installed under
`/mnt/c/Users/perso/AppData/Local/NevroTick/sanitizer-update-20261031-4g1800_z/`.
The previous `sanitizer-setup-20261031-2cgkw00x/` retains its initial controls,
linker/ABI setup failures and old sysroots for rollback.
The shipping compiler/profile, product graph and original analysis installation
retain their independent admission. Windows AppVerifier/PageHeap and future
Android HWASan need their own native evidence; GNU controls cannot supply it.

**Refresh qualification, 31 October 2026:** `controls-v1/verification.json` in
the update evidence directory records all
five healthy controls returning 0 and all five seeded faults returning 86 with
the required detector diagnosis and fixture source site. Ten compilations
succeeded; all 203 payload records and runner/source bytes passed final custody.
The inventory includes the compiler's resolved native dependencies and ELF loader.
The checker has 13 passing Python controls for admission, classification and
supervision. No ABI-mismatch bypass was used. A byte-identical external checker
and fixture copy qualified the candidate before activation; actual repository
admission then passed. The policy and private pointer switch sequentially;
the mixed new-policy/old-pointer control rejected admission. Original copies
remain available for rollback. Existing Miri/fuzz campaigns keep their separately
pinned compiler and evidence chain.

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
complete package's release evidence and final checksums.** Before the protected publisher runs, the
release owner must reconcile this Rust inventory with native objects/libraries,
generated source, resources, build-tool provenance and the actual package payload;
validate the final CycloneDX/SPDX document; and bind the exact version, source
candidate and **final package/payload hashes**. For the selected Store MSIX route,
retain submitted and actual Store-delivered identities separately and reconcile
any changed bytes before claiming final CHECK-18/23/26 qualification. A Cargo inventory alone cannot
satisfy CHECK-26 or that complete-product requirement. Prepare the SBOM before
publication; do not generate a lockfile-only inventory after publishing a version.

The eventual source/evidence-release publisher prepares a draft release and
verifies its complete attachment set before publication. Store submission and
delivery use the [selected MSIX workflow](docs/plans/windows-msix-store.md#7-bounded-continuation-and-existing-acceptance-owners),
not `gh release upload`; that workflow separates local mechanics, David's
controlled submission and final Store-artifact qualification. Store package
redistribution is not presumed authorized by an SBOM workflow. If David later
chooses the retained MSI route, its existing signed-MSI upload procedure remains
separate. This is a handoff contract, not a new publication job or authorization
to upload now. The current Cargo graph does not supply a complete package or
release workflow; full payload reconciliation and publication remain pending.

The [release policy](docs/release-policy.md) owns the candidate approval/status
record and subsequent withdrawal or publishing-authority incident. Bind SBOM
and artifact evidence to that record while keeping support terms proposed until
approved. This tooling receipt supplies no operational recovery rehearsal or
installed-device remediation evidence.

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

The sixth member, [nevrotick-engine](crates/nevrotick-engine/Cargo.toml), owns the
[bounded engine transport pilot](tasks/engine-transport-pilot.md) and its
`nevrotick-observe` CLI. It requires two explicit loopback peers and an explicit
timeout. This experiment has no production profile, eligible samples,
authentication, clock effects or UI integration. Tokio remains engine-only.

The exact [lockfile](Cargo.lock), manifest/feature identities, selected grants
and source-file maps are recorded in
[the dependency policy](policy/ui-m0-dependencies.json). Its schema 2 distinguishes
twenty-three packages selected across the two targets from two metadata-only
resolution sources (`r-efi` and `wasi`). All twenty-five are needed for offline
lock resolution; those two gain no runtime, build-script or selected-license
admission. Frozen metadata checks require the exact twenty-one Linux and
twenty-two Windows packages and their feature sets before per-target cargo-deny.
All original sixteen package/grant/file-map records are retained. Garde still
uses only `derive`; the Windows binding feature union now also includes Tokio's
reviewed networking bindings. Runtime and native qualification remain separate.
On 29 October, `dependency-check-6.json` recorded successful frozen advisory,
license, source and build-script checks for Linux and Windows. The receipt lives
under `%LOCALAPPDATA%\NevroTick\ui-m0`; this proves that graph at its recorded
snapshot, not future advisory freshness or native UI behavior. Current application
results and unresolved observations belong in the task record.

The pure wire owners' additional evidence uses
[verify-wire-analysis.py](scripts/verify-wire-analysis.py) and
[its scoped policy](policy/ntp-wire-analysis.json), after the ordinary graph and
portable rows. Select `--lane contracts`, `coverage`, `mutation`, `fuzz` or `all`,
and `--subject header`, `datagram`, `nts_ke`, `nts_negotiation`, `nts_packet`,
`nts_plaintext` or `all` (default)
for coverage/mutation scope,
with `--output-dir /new/private/Linux/output` and, when selecting another copy,
`--vendor-dir /admitted/vendor`. The output path must be new. These are owned
isolated analyses with explicit source/tool/graph inventories, not host Cargo
fallbacks or installation commands. The repository's existing analysis installation
and matching payload inventories are required.

The coverage row reports each selected security file's executable-line denominator
from one actual package run. Mutation reconciles separate complete nonempty
per-file enumerations and treats build failures as unviable; the selected campaign
shares the fixed total mutant/time budgets. External consumer checks require a
compiling legal caller and exact privacy/borrow error causes. Contracts cover all
six registered owners; `--subject` does not narrow that lane. Candidate-specific
results belong to the [protocol task](tasks/s0.1-protocol-comparison.md#inert-nts-plaintext-framing).
The test-only fuzz target covers all six pure owners, including request-bound
negotiation, outer authenticator framing and inner plaintext framing. Contracts and fuzz exercise their complete scope;
`--subject` narrows coverage/mutation only. An independently written byte-state
model supplies request acceptance and fixed-offer response acceptance/projection
expectations. Rejection classes/precedence and refusal contexts without algorithm
15 remain direct-test evidence. The
`ntp-wire-ke-framing` sanitizer profile permits 131,073-byte inputs (one above the
parser ceiling) with fixed 256 runs, two-second input timeout, 512 MiB RSS,
256 MiB allocation and 30-second total limits. Its explicit caller/profile routing
rejects foreign, duplicate, missing and extra flags. Legacy/Core/history profiles
retain their separate limits. Four distinct retained executables provide the
healthy run and datagram, raw KE and negotiation count sensitivity controls; those
assertions do not themselves qualify an ASan detector.

The GNU-host harness separately admits its locked source/features and native
LLVM-exception notice and uses archive-verified vendor metadata. This test-only
graph is not production NTS/TLS or distribution admission. New profile/target
qualification passed for the frozen 108-input raw-framing candidate, with 256
healthy callbacks and both intended semantic-oracle controls. The
[bounded raw-framing checkpoint](tasks/s0.1-protocol-comparison.md#bounded-raw-framing-host-checkpoint)
records its exact receipts and separate Windows cross-build result; neither
establishes semantic/TLS integration or native Windows execution.
Historical successes, first failures and incomplete rows remain in
[S1.1a's candidate record](tasks/s1.1-header.md) and
[S1.1b's execution record](tasks/s1.1-framing.md); source/CLI presence is no pass.

The later [pending NTPv4 negotiation checkpoint](tasks/s0.1-protocol-comparison.md#pending-ntpv4-negotiation-implementation--1-november-2026)
records the frozen 110-input host gates and extended four-owner fuzz result:
256 healthy runs and all three intended count sensitivity failures, with four
distinct retained binaries. These checks admit pending negotiation data only;
TLS, provider support, complete endpoint policy and usable-association publication
remain separate qualifications.

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
and [time-core-analysis.json](policy/time-core-analysis.json). Its recorded
six-owner checkpoint covers regression, residuals, normalization, history, runs
and selection. The [S1.3 record](tasks/s1.3-source-selection-readiness.md) binds
501 exact generated variants and the three separately reviewed
[selection equivalences](tasks/s1.3-selection-equivalences.md); actual missed
outcomes are required before adjustment. The
[S1.2 concurrency record](tasks/s1.2-regression-readiness.md#user-requested-mutation-concurrency-trials-30-october-2026)
records the qualified ten-worker regression pilot and failed lower-limit trials.
The core selects ten outer workers, isolated mutant targets, one inner Cargo/test
slot and a fixed 512-task sandbox limit for admitted mutation campaigns.
Other invocations retain 128; this changes no shipping compiler flags, NTS job
limit or quality threshold. Core coverage explicitly uses
`-- --test-threads=1` to keep libtest progress from interrupting strict counter
records; compile jobs remain twelve. Other wrappers retain their default unless
they explicitly adopt that override. The 180 checkpoint profile/accounting controls
passed. The S1.3 record preserves the original failures, native 501-variant
execution and read-only reconciliation, serialized coverage and history-only
ASan-enabled semantic fuzz controls. The 4,096 healthy callbacks and intended
published-report assertion establish bounded input delivery and semantic failure
propagation. No matching address/lifetime detector control or real leak control
was located for this profile; ASan detection and leak freedom remain unqualified.
Its 31 October host-fixture checkpoint also records the configured
workspace and optimized checks. Follow the exact candidate and scope in those
receipts; neither tool admission nor these pure
fixtures establish production, native Windows or release qualification.

**Engine analysis entry point, 1 November 2026:**
[verify-engine-analysis.py](scripts/verify-engine-analysis.py) and
[engine-analysis.json](policy/engine-analysis.json) select the engine library and
CLI critical files through the existing **full** tool-admission profile. Use
`--lane enumerate`, `coverage` or `mutation`, `--subject library`, `cli` or `all`,
an explicit admitted `--vendor-dir` and a fresh external Linux `--output-dir`.
Enumeration retains complete descriptors and reports `enumerated`, with
qualification/campaign admission false. Mutation requires reviewed exact inventory
bindings; these are initially empty. The 90% executable-line and 95% per-file
mutation gates remain unchanged. The controller's 24 Python controls pass,
including a demonstrated snapshot-drift RED/GREEN; these are not actual engine
coverage or mutation results. The current [engine task](tasks/engine-transport-pilot.md#s29-engine-analysis-admission-and-cargo-test-layout)
records the isolated API-test layout, Cargo baseline and remaining resource/native
gates. A serial Cargo observation does not establish ten-worker readiness.

The next fixture projection owner and its tests are now source-authored in the
main checkout. That earlier checkpoint's product census is 86 inputs. Its exact 557-variant
inventory is admitted through a complete seven-file cohort plan; the whole union
remains subject to the cumulative reservation and receipt reconciliation. The
configured portable row, seven-file line coverage, all seven mutation cohorts,
consumer contracts and bounded history semantic fuzz passed within the declared
86-input host-fixture checkpoint in S1.3. The repaired merger separately bounds
native receipts to eight MiB while keeping admission cards/plans at 512 KiB.

**1 November lifecycle receipt refinement:** the expanded native report exceeded
that earlier limit. Native core receipts now have a 64 MiB UTF-8 byte cap and a
lossless compact final writer; admission cards and plans retain 512 KiB. Both
roles reject duplicate keys, truncation, symlinks and nonfinite values, including
finite-looking exponent overflow such as `1e999`. Five direct reader/writer
controls pass. A retained 16,393,749-byte failed-campaign report round-trips to
11,315,999 compact bytes with all values and its failed gate preserved; this
executes no native command and changes no raw receipt. The 64 MiB cap covers the
current admitted schema/cohorts and must be reassessed if their shape grows.

The ordinary and optimized portable `time-core` nextest rows compile with six
build jobs after a retained twelve-job linker thread-resource failure. Other
portable build rows keep twelve jobs, and test execution keeps twelve workers.
The focused verifier regression passes 75 controls. This changes no shipping
compiler flags, target baseline or toolchain. Full mutation campaigns retain ten
outer workers, one inner Cargo/test slot and their admitted 512-task envelope.

**Stable-core admission, 1 November:** the current time-core policy explicitly
selects `tool_admission_profile = "stable-core"`. Its native contracts, coverage
and mutation commands therefore require `--tool-admission-profile stable-core`.
Full admission remains the API/CLI default for other consumers. The current S151
inventory card explicitly pins `stable-core` and its listing command also requires
`--tool-admission-profile stable-core`. This exception admits only the canonical
one-owner residuals discovery role, with zero native test invocations and no
campaign. The card/profile, exact subjects and noncampaign fields must match
before a Session starts; the unchanged command guard permits only `--list --json`.
Stable compiler/vendor/source checks, separately sealed cargo-mutants bytes and
final readmission remain required; unused nightly/optional payloads are neither
mounted nor claimed verified by this profile. Metadata-only cohort union uses
its default flag and verifies native receipts' profile against the plan-bound
policy. This changes no compiler flags, tool versions or shared installation.

**Dedicated Kani admission, 5 November:**
`check-analysis.admit_kani_proof()` preserves the complete existing Kani payload
and private compiler inventory, shipping executable/bootstrap checks and pointer
bookends, while returning exactly two readonly Kani roots. Generic admission
choices remain `full`/`stable-core`; generic Cargo/analysis consumers cannot select
this role. The formal runner's explicit `kani-proof` option must use those roots
and the unchanged Kani flags and resource/source controls. Seventy-five focused
Python/runtime tests, including actual Linux namespace visibility and tampering
controls, passed. Native coordinate healthy/three-defect/restored commands and
final admission also passed, with the actual namespace mounts checked. Mean
selected admission was 9.36 seconds over five commands, versus 37.30 seconds for
the preceding full-admission coordinate proof; this is a sequential observation,
not a general benchmark. Exact receipts and remaining math are in the
[current proof checkpoint](tasks/s1.1-time-domain.md#exchange-solver-limit-and-admission-checkpoint--5-november-2026).
No whole-store fallback or shared installation change follows from this profile.

The separate `admit_era_constructor_codegen()` API reuses that complete inventory
and exact two-root mount closure for the closed `era-constructor-codegen-v1`
consumer. Its fixed Cargo-mode `--only-codegen` profile is preparation for the
actual public EraWindow/Garde constructor, not proof execution or generic Cargo
admission. It requires its separately authenticated selected vendor view,
immutable offline project/lock/configuration and ordinary runtime custody. The
proof API's flags and generic `full`/`stable-core` choices remain unchanged.
All 51 admission helper controls passed after this additive API; source/module
comparison confirmed every pre-existing admission definition unchanged. Track
actual compiler execution and generated-code inspection in the
[existing constructor checkpoint](tasks/s1.1-time-domain.md#remote-s21-handoff-and-public-constructor-preflight--5-november-2026).

`admit_era_constructor_model_inspection()` is another closed consumer over the
same complete Kani inventory and two readonly tool roots. Only its fixed bundled
`goto-instrument --dump-c` command, authenticated qualified model copy and
separate ordinary runtime custody are admitted; it grants no generic tool/flag,
Cargo or solver surface. All 53 admission helper controls passed. The
[model-inspection checkpoint](tasks/s1.1-time-domain.md#public-constructor-model-inspection--5-november-2026)
owns actual extraction and subsequent loop/unwind/proof evidence.

`admit_era_constructor_proof()` reuses the same complete inventory for the closed
five-job actual-constructor consumer. Its reviewed Cargo profile retains model
validation, safety checks and sanity, with a provisional unwind of 32. All 55
admission helper controls passed. The first actual healthy job stopped during
GOTO model validation, before any solver result, because padded union member
types disagree with their declared storage types. Do not rerun that candidate
unchanged or infer a constructor proof from codegen or the readable model.
The [constructor proof checkpoint](tasks/s1.1-time-domain.md#public-constructor-proof-checkpoint--5-november-2026)
owns the retained failure, two bounded diagnostic reserves and any separately
admitted verifier repair. The separate private candidate now has the matching
`rustc-dev` component and rebuilt compiler: all 46 binding tests and eight
bounded MEMBER controls qualified, including the intended false-assertion
failure and restored health. That historical 7,130-entry inventory changed only
the compiler. The current private 7,131-entry inventory also contains one exact
copy of its existing LLVM library in the target `rustlib` directory, required by
the nested Cargo/rustc loader. The installed library is 207,996,272 bytes,
SHA-256 `559bc1644a9b1124bf62bf11cb022c8c385cb53dc940893a8a87b44241f9a77d`;
the extension and original entries qualified separately. Constructor compilation
now reaches CBMC; its latest fixed 120-second attempt timed out without returning
checks. The public constructor proof remains incomplete; follow its existing
checkpoint's source-only hold. The follow-on target-refinement and raw-property-ID
driver patches are unbuilt and unadopted; no new driver is installed. The reusable
[compiler repair source kit](tools/formal/kani-padded-union/README.md) belongs in
Git; generated toolchains and campaign outputs remain external. Shared tool installation and shipping
Rust/LLVM policy are unchanged. Runtime admission checks the effective tools and frozen build
provenance without reopening unused compiler-build source/vendors.

The [later full-admission metadata optimization](tasks/s0.1-protocol-comparison.md#full-tool-admission-metadata-optimization--1-november-2026)
keeps those scopes and mounts intact. Inventory names schedule terminal checks;
the existing twelve-worker pool verifies actual types/sizes, symlink identity and
containment, and descriptor-read hashes. Exact membership still precedes hashing.
Two unprofiled sample pairs reduced mean full admission from 58.58 to 40.22 seconds
with identical complete results. The Linux stable inventory was slightly slower;
this is no claim about every profile or total development speed. All 152 selected
admission/caller/runtime controls passed. Historical execution receipts retain
their original checker identity.

The narrower profile hashes the complete pinned stable inventory, validates all
four inventory descriptions, and retains private installation-pointer, policy,
compiler-entrypoint and bootstrap-helper checks. Its five readonly stable scopes
replace the broad analysis-store mount; selected extension tools, vendor inputs,
source/configuration and per-command sealed-tree checks remain. Nightly compiler
or extra-tool-island requests are rejected. Sixteen new controls and 270 affected
regression tests pass, including actual private namespace observations that an
omitted executable is inaccessible and produces no effect. No compiler flag,
toolchain, Rust source, dependency or shipping policy changes in this refinement.

The paired current-candidate coverage sample executes the same 177 tests with
identical six-owner coverage and counter objects: full admission takes 127.94 s
overall/120.98 s in admission, versus 15.40 s overall/8.32 s in admission under
stable-core. This is one sequential profiled pair, not a general development-speed
measurement. Both profiles verify the selected inventory; unused stores are not
mounted in stable-core. Actual selected-profile contracts also pass all 27 native
commands. Source, scope, raw receipts and the comparison's limitations are in the
[current S1.3 checkpoint](tasks/s1.3-source-selection-readiness.md#stable-core-admission-and-current-campaign).
Its 163 controls and retained-native receiving-function check passed; historical
native reports keep their original tool identity and are not a fresh CLI campaign.
The new projection/cohort readers and existing wrapper
and portable controls passed 155 selected controls; 19 selector/runtime controls
also passed. First failures, causal repairs and incomplete portable attempts are
retained in the same S1.3 task. These control results qualify no product behavior.

The subsequent exact-enclosure bridge has a separately qualified 89-input
Linux host-fixture checkpoint in S1.3: portable checks, all eight line gates,
688 native mutation variants at ten workers with zero timeouts, actual CLI
union, ten legal/106 rejected consumers and 4096 bounded history callbacks with
the intended semantic-fault control. The 172 selected wrapper/accounting controls
passed. Four new exact equivalents have independent full-domain proofs; the
quarter-fraction assertion gap was causally repaired, preserving the first failed
campaign and its complete continuation reservation. E counts remain separate
from kills and unviable builds from tests.

That checkpoint used explicit supplemental full-closure checks for the omitted
nonmutation helper; they establish before/after custody under the frozen owner,
not an immutable helper snapshot. The subsequent persistent input-list repair
passed a three-lane boundary RED and 173-control GREEN; the stale aggregate-512
description is corrected too. Old native receipts retain their actual tool revision.
An independent input-copy pilot passed source admission. Subsequent projected-batch
snapshots executed eleven focused native cases while main's accounting advanced;
the main candidate then passed the configured portable row: 254 ordinary tests,
56 doctests and 227 optimized executions. Its genuine assertion RED, first harness
lint failures and independent recipe reconciliation remain in the same task.
The supplementary public validation-result probe also passed with a genuine
healthy control and an isolated callback failure. Multiple prepared source
copies require distinct Cargo target directories and actual compile-origin /
executed-binary observations: the first probe reused a pristine cached binary
and remains incomplete in the task record. The exact 92-input projected-batch
candidate now has a qualified Linux fixture checkpoint: all nine line gates,
717 native variants at ten workers (411 K, 7 M, 299 U, 0 T; E = 7 as a subset of M), nine
138-case baselines, actual CLI union, ten legal/120 rejected consumers and
bounded history semantic fuzz. Current wrapper/helper controls passed 193 cases;
the first diagnostic-expectation failure and explicit continuation remain retained.
History-only semantic
fuzz provides no address/lifetime-detector or leak qualification. Full S1.3/P1,
production, native Windows and release gates remain open.

The main checkout now also contains the bounded stateful
[principal/hysteresis fixture](tasks/s1.3-source-selection-readiness.md#next-principalhysteresis-fixture-31-october-2026).
Its actual constructor RED and first unchanged producer-to-principal GREEN are
retained separately from the 92-input checkpoint. The formatted 96-input candidate
has fourteen principal cases, including genuine private-receiver controls, and
passed the complete portable workspace row: 268 ordinary tests, 56 doctests and
241 optimized executions. Current 96-input coverage passed 147 core tests with
92.56% principal and 95.25% arithmetic line coverage; scoped contracts passed.
Both selected mutation cohorts and the actual CLI union passed all 327 generated
variants at ten workers, with one exact reviewed equivalent disposition. The
[checkpoint record](tasks/s1.3-source-selection-readiness.md#principal-receiver-checkpoint-31-october-2026)
retains both failed campaigns and the one exact reviewed floor equivalence.
Ordinary development uses the configured stable sandbox and the complete used
stable-toolchain inventory; unused optional analyzer stores need no full
admission for that row. The analysis wrapper now checks the unchanged 2 GiB
space floor before expensive admission and again before creating its output;
176 wrapper regression tests pass. Analysis honors six admitted Cargo build
jobs while retaining ten mutation workers with one nested job; other default
callers retain twelve. The existing runner owns the selected checkpoint. Preserve the old nine-owner qualification and its
scope; no current ten-owner qualification is implied.

The current principal campaign also has a qualified private RAM view of the
unchanged analysis payloads at their original logical paths. Complete admission
took 13.68/13.33 seconds through that view versus a retained 63.54-second original
store sample; these limited samples establish no whole-day speed estimate.
The [task record](tasks/s1.3-source-selection-readiness.md#execution-efficiency-correction-during-principal-development)
retains setup costs/failures, exact inventories and the successful corruption,
identity, nested-isolation and owned-write controls. Reuse the four read-only
child binds only inside the private outer namespace, with fresh `/dev` and an
explicit read-only backing-alias bind; keep all existing inner sandbox and
per-command admission checks. The tmpfs view is volatile, uses about 3.7 GB RAM,
and changes no installation pointer or shared configuration.

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
| `--lane portable` (default) | Format, all-target compiler/Clippy and library rustdoc for the six admitted members; explicit observe CLI build, separate per-package library/integration tests and nonempty doctests, plus configured optimized owner tests with actual discriminating cases. Missing, empty, skipped or wrong-package selections fail; package and aggregate counts are retained. |
| `--lane dependencies --advisory-receipt /path/to/current-receipt.json` | Frozen Linux and Windows metadata/feature checks followed by target-specific cargo-deny, using a separately acquired RustSec/index snapshot bound to the lockfile and at most 24 hours old. |
| `--lane windows --output-dir /path/to/new-build-output` | Windows-target compiler/Clippy and separate release links for the UI and observe CLI; exports both executables and matching PDBs to new external output, preserving existing UI receipt keys and adding the engine artifact record. Native execution remains separate. |

Portable wire and Core test builds use six Cargo build jobs to bound concurrent
LLVM test-binary links within the ordinary 128-task sandbox. Other build rows
retain their configured concurrency; test execution and mutation-worker limits
are separate. The [S1.3 task's S158 checkpoint](tasks/s1.3-source-selection-readiness.md#s158-integrated-authoring--portable-pass-qualification-next)
retains the observed wire-link failures, command controls and successful bounded
retry.

Rows run with a read-only source snapshot, cleared environment, isolated network
and owned output inside bubblewrap. The verifier admits pinned tools and source
bytes, checks sandbox boundaries, records effective checker/configuration hashes
and rejects input drift. Resource/time/output bounds remain distinct from hard
cumulative quotas. There is no host-execution fallback and no automatic download.
`--json /path/to/new-receipt.json` retains the selected row's output. The nextest
store is redirected only in the owned snapshot so reports stay in writable output;
test policy and the repository configuration remain unchanged.
For a later GNU runtime observation, give the portable lane a new `--output-dir`.
It retains the actual `nevrotick-observe` image before temporary build disposal,
with its exact hash/profile/target in `engine_artifact`; it verifies copied bytes
and owner execution permission. A compiler/test pass alone supplies no CLI journey
or native Windows observation.

For first setup, select new external destinations and run acquisition separately:

```sh
python3 -B scripts/prepare-product.py --vendor-dir /path/to/new-vendor --configure --scratch-root /path/to/build-scratch
python3 -B scripts/refresh-product-advisories.py --cargo-home /path/to/new-metadata --receipt /path/to/new-metadata-receipt.json
python3 -B scripts/verify.py --lane dependencies --advisory-receipt /path/to/new-metadata-receipt.json
python3 -B scripts/verify.py
```

`prepare-product.py` verifies bounded archives and their reviewed extracted-file
maps for the complete twenty-five-source resolution view without executing
package code. `--configure` requires an absent private
`~/.local/share/nevrotick/ui-m0.json` pointer; it never replaces an existing one.
The verifier reads `vendor_dir` and `scratch_root` from that pointer, while explicit
`--vendor-dir` and `--scratch-root` select another admitted location. Put large build
outputs on the selected Windows drive when appropriate. A small owned Linux copy
of vendor/metadata inputs can avoid DrvFS per-file overhead; preserve and verify
the same file maps. Refresh metadata into new locations when needed and pass the
new receipt explicitly. Portable/Windows rows do not silently refresh or rerun the
dependency row; complete that admission before executing newly admitted graph code.
An existing sixteen-source vendor pointer cannot admit this new graph. Select an
already admitted complete vendor explicitly with `--vendor-dir`; do not overwrite
shared host configuration or fetch during verification.

The root [deny.toml](deny.toml) deliberately retains the bootstrap GPL-only and
no-build-script baseline. The dependency row generates a configuration solely
inside its owned source snapshot: GPL-2.0-only, MIT, BSL-1.0 and Unicode-3.0 grants
for this frozen graph, and exact build-script allowances for
`proc-macro2@1.0.107`, `quote@1.0.47`, `rustversion@1.0.23` and
`getrandom@0.4.3` on both targets, plus `libc@0.2.191` on Linux only.
The `MIT AND Unicode-3.0` conjunction is preserved. All held-provider, advisory,
source and other ban rules remain.
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


## Fixture semantic-state guidance

The standalone `tools/state-guidance/` lane uses LibAFL/bolts 0.16.1 and the
admitted Rust 1.99.1 GNU toolchain. It adds no shipping workspace member or
product dependency. The pure versioned action driver calls the real
`ntp-exchange` association and checks a separately authored scalar model.
A reviewed wrapper instruments only the three selected pure product crates.

Both crates have defaults disabled in the
[engine manifest](tools/state-guidance/engine/Cargo.toml). Selected `libafl`
features are `derive` and `serdeany_autoreg`; selected `libafl_bolts` features
remain `std`, `derive` and `serdeany_autoreg`. Optional `libafl/std` was removed
after the original graph's audit reported unmaintained `bincode 2.0.1`
(`RUSTSEC-2025-0141`). The warning receipt remains historical; the change used
no upstream patch or ignored advisory. The current [lockfile](tools/state-guidance/engine/Cargo.lock)
contains 130 packages, with 101 in the selected GNU host graph. A fresh private
RustSec advisory-only audit reported zero vulnerabilities and zero warnings
for this exact lock. Yank status was outside that audit's scope; this is no
registry/index or full security/release qualification. The supervisor requires
the pinned passing exact-lock advisory receipt as part of admission.

```sh
python3 -B tools/state-guidance/run.py
python3 -B tools/state-guidance/run.py --controls --output-dir /absolute/existing-parent/new-controls
python3 -B tools/state-guidance/run.py --compare --output-dir /absolute/existing-parent/new-comparison
```

These are offline guarded commands. Fresh output must be outside the checkout
and installation; the parent must already exist. The private schema-1 pointer
`~/.local/share/nevrotick/state-guidance-installation.json` locates the installed
payload. The checkout's `tools/state-guidance/policy.json` pins its manifest, source,
recipe, native dependencies and qualifying controls. Changed bindings fail closed;
comparison requires controls for the exact candidate. Reading a skill does not
install, rebuild or repin tools. The installer retained exact lock/vendor/source
and compiler provenance in its private installation evidence directory.

The default corpus includes pending-request creation, completion, socket
rejection/recovery and stale-cancellation sequences. Its `arm` action creates a
pending request; it does not refer to ARM processors. The corpus-only follow-up
kept the x86-64 build fixed and requalified the changed seed recipe and controls.

The engine also provides an explicitly selected effects oracle and a lifecycle
mutation operator. Legacy calls keep baseline mutations and strict comparison.
The effects lane checks observable state/completion effects before continuing a
returned-error difference, and retains the first diagnostic input separately
from the coverage corpus. The lifecycle operator remains optional: the bounded
four-fault experiment improved cancellation sensitivity but regressed context
invalidation search. Its common training corpus excludes the default
stale-cancellation input, which already triggers one of the new private variants.
See the [fault-suite results](tasks/s1.1-deterministic-harness.md#bounded-fault-suite-and-mutation-follow-up-1-november-2026)
and [experimental admission procedure](.agents/skills/testing/qa-state-guided-fuzzing/references/backend-workflow.md#compare-fault-classes-oracle-objectives-and-mutations).

The completed bounded pilot, mixed results and later corpus experiment are recorded in the
[S1.1e follow-up](tasks/s1.1-deterministic-harness.md#semantic-feedback-follow-up-31-october-2026).
Use [state-guided fuzzing](.agents/skills/testing/qa-state-guided-fuzzing/SKILL.md)
for consumer/feedback selection and the corresponding skill reference for limits.
This lane qualifies fixture forwarding, oracle/feedback sensitivity and finite
search observations. It supplies no NTS, sanitizer, native Windows, performance
budget or release result. Existing cargo-fuzz, Miri, Loom, Kani and native
verification workflows retain their separate purposes.
