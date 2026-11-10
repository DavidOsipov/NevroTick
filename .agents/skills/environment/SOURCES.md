# Environment workflow sources

## Dependency workflow

Reviewed 28 October 2026. Source skill:
`/home/david_osipov/personal-website-templated-1/.agents/skills/environment/dependency-updates/SKILL.md`.
Retains scoped discovery/cohort/review/qualification and dirty-work custody.
Cargo metadata, lockfile/source/features/MSRV/targets and build scripts replace
npm/ncu, peer graphs and node_modules assumptions. Source read, never modified;
npm comparison helper not copied.

[canonical architecture](../../../docs/windows-time-architecture.md), sections
18.2–18.10, controls licensing, Serde/TLS/native decisions and admission. Skill
research neither installs, pins nor qualifies versions/optional tools.

| Primary source inspected | Applied finding |
| --- | --- |
| [Cargo update](https://doc.rust-lang.org/cargo/commands/cargo-update.html) | Package-scoped updates are conservative but can change required transitives; --precise selects a version/revision; inspect the actual lock delta and don't default to blanket update |
| [Cargo tree](https://doc.rust-lang.org/cargo/commands/cargo-tree.html) | Feature/reverse graphs explain feature activation and duplicates; graph display is not a guarantee of exact compiled/shipped bytes |
| [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/features.html) | Additive feature unification means disabling one direct default does not prove the resolved feature is disabled |
| [Rust 2024 resolver](https://doc.rust-lang.org/stable/edition-guide/rust-2024/cargo-resolver.html) | Edition 2024 implies resolver 3 for a package; a virtual workspace needs an explicit resolver decision; fallback does not prove MSRV compilation |
| [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) | A shared lockfile and explicit member metadata inheritance have different meanings; inspect each supported member's actual declaration |
| [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) | Build scripts execute during compilation and may compile native components; preflight build/proc-macro behavior, not only runtime packages |
| [Cargo build](https://doc.rust-lang.org/cargo/commands/cargo-build.html) | --locked preserves resolution; --offline restricts Cargo retrieval. Neither confines arbitrary build-script behavior |

Also read the user's pinned [PdfCraft vendored-maintenance skill](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-vendored-dependency-maintenance/SKILL.md)
and [Rust workflow](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-rust-implementation/references/rust-workflow.md)
through the GitHub connector. Preserve each actual local patch's upstream identity,
diff/hash, regression and retirement condition; excluded vendored crates need
explicit test accounting. Published-base audits do not attest to local changes.
The full David Osipov MIT notice is retained in the dependency leaf.

Adaptation added, upgraded, downloaded/built no dependencies. Prospective commands
need selected-toolchain checks. Advisory freshness, license compatibility and
Windows runtime remain real-graph admission/qualification tasks.

### Package identity clarification — 8 November 2026

The first-adoption name/origin check makes existing architecture sections 18.2
and 18.5 explicit in Cargo discovery. It changes no dependency decision or
execution permission.

| Primary source inspected | Applied finding |
| --- | --- |
| [Cargo manifests and lockfiles](https://doc.rust-lang.org/cargo/guide/cargo-toml-vs-cargo-lock.html) | Exact resolution preserves the selected dependency; intended-upstream identity remains a separate admission question. |
| [Spracklen et al., package-hallucination research, v3](https://arxiv.org/abs/2406.10279v3) | Original Python/JavaScript research motivates checking generated package names against the intended upstream; it establishes no Rust or current-model prevalence. |

### Publish-age watchlist — 9 November 2026

The [Cargo reference](dependency-updates/references/cargo-operations.md#future-publish-age-watchlist--9-november-2026)
retains a future evaluation trigger; it selects no delay, toolchain update or
configuration. Registry-age policy cannot replace source/dependency admission.

| Primary source inspected | Applied finding |
| --- | --- |
| [Beta Cargo configuration](https://doc.rust-lang.org/beta/cargo/reference/config.html#registryglobal-min-publish-age) and [stable configuration](https://doc.rust-lang.org/cargo/reference/config.html) | Beta documents the settings as respected from 1.100+; stable 1.99.1 lacks them. Lockfile exemptions, overrides and absent registry publication-time support limit coverage; recheck released behavior before any later adoption. |
| [RFC 3923](https://rust-lang.github.io/rfcs/3923-cargo-min-publish-age.html) | Motivation and proposal history; the shipped configuration/resolver contract must control a future experiment. No delay or security-benefit measurement is established here. |

### Cargo intermediate-layout watch — 9 November 2026

The [Cargo operations reference](dependency-updates/references/cargo-operations.md#future-intermediate-layout-compatibility--9-november-2026)
adds a future upgrade check for actual verifier consumers, separate from the
existing publication-age watch. No toolchain or runtime configuration changed.

| Primary source inspected | Applied finding |
| --- | --- |
| [Beta Cargo 1.100 changelog](https://doc.rust-lang.org/beta/cargo/CHANGELOG.html#cargo-1100-2026-11-12) | Upcoming intermediate-layout change, dated 12 November; recheck released behavior at admission. |
| [Beta build cache](https://doc.rust-lang.org/beta/cargo/reference/build-cache.html) | Intermediate layout is internal; distinguish it from final target-directory exports. |
| [Cargo external-tool messages](https://doc.rust-lang.org/cargo/reference/external-tools.html#artifact-messages) | Discover emitted artifacts/output directories from messages; retain independent identity, containment, inventory and byte checks. |

## Windows Sandbox management

The original [manage-windows-sandbox](manage-windows-sandbox/SKILL.md) workflow was
researched on 3 November 2026 through Microsoft Learn MCP and pinned Microsoft
`winappCli` documentation, implementation and tests. It is locally authored;
no upstream skill or executable controller is imported.

Its six directly linked references keep the detailed source registers beside
the relevant operations, configuration, WSL resource, optional winapp, NevroTick
evidence and troubleshooting guidance. The released winapp baseline is v0.7.1 at
`e3cd0b94f3e7b776ac51ba0564f4908094e979b5`; a separately identified development
revision supplies version-drift evidence, not installed-release guarantees.
The [existing Windows task](../../../tasks/windows11-integration.md#416-windows-sandbox-management-skill--3-november-2026)
owns the documentation delivery and checks. Source review does not install or
qualify Sandbox, adopt the proposed 3072 MB profile, lift execution holds or
establish native application behavior.

The 10 November SAC troubleshooting amendment checks Microsoft's
[Sandbox maintainer workaround](https://github.com/microsoft/Windows-Sandbox/issues/85#issuecomment-3025447682),
its [reported counterexample](https://github.com/microsoft/Windows-Sandbox/issues/85#issuecomment-3025799619),
[SAC test-mode documentation](https://learn.microsoft.com/en-us/windows/apps/develop/smart-app-control/test-your-app-with-smart-app-control),
[component-block support explanation](https://support.microsoft.com/en-us/windows/security/threat-malware-protection/smart-app-control-has-blocked-part-of-this-app),
[Sandbox restart/disposal semantics](https://learn.microsoft.com/en-us/windows/security/application-security/application-isolation/windows-sandbox/)
and [Code Integrity event meanings](https://learn.microsoft.com/en-us/windows/security/application-security/application-control/app-control-for-business/operations/event-id-explanations).
The procedure stays guest-only and distinguishes configured Off, effective
execution, owner-reported success and SAC-enabled product qualification. It
imports no general recovery/BitLocker/host-security procedure. The current case
belongs to W11 §4.19; skill maintenance itself executes no Windows commands.

The guest process-only PowerShell amendment uses Microsoft's Windows PowerShell
5.1 [execution-policy scope/precedence](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_execution_policies?view=powershell-5.1)
and [CLI policy parameter](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_powershell_exe?view=powershell-5.1).
It preserves Group Policy refusals and source admission; no host/persistent
setting, `Bypass` or automatic unblocking is adopted. W11 §4.19 and the existing
crypto task retain the initial `Restricted` setup stop and later observed
process-scoped `RemoteSigned` result separately from native product gates.
