# Selected upstream specialists

These four reviewed specialists supplement 52 project-owned adaptations. Select for specific diagnostic/automation questions; NevroTick's architecture, implementation/security owners and QA contract still govern product decisions, effects and qualification.

| Skill | Added capability | Reviewed source and license |
| --- | --- | --- |
| [rust-compiler-errors](rust-compiler-errors/SKILL.md) | Diagnose compiler errors by ownership, trait, inference and dyn-compatibility mechanism; recheck the original failing command | [po4yka/rust-skills](https://github.com/po4yka/rust-skills/tree/5d5f4706d6bf1d03366a1fb463aa1732de2dbbd2/skills/rust-compiler-errors), [BSD-3-Clause](rust-compiler-errors/LICENSE) |
| [rust-borrow-semantics](rust-borrow-semantics/SKILL.md) | Expression-specific temporary/drop scopes, two-phase borrowing and edition-sensitive probes | [po4yka/rust-skills](https://github.com/po4yka/rust-skills/tree/5d5f4706d6bf1d03366a1fb463aa1732de2dbbd2/skills/rust-borrow-semantics), [BSD-3-Clause](rust-borrow-semantics/LICENSE) |
| [rust-send-sync](rust-send-sync/SKILL.md) | Auto-trait and task/thread-boundary diagnosis, lock payload bounds and deliberate PhantomData | [po4yka/rust-skills](https://github.com/po4yka/rust-skills/tree/5d5f4706d6bf1d03366a1fb463aa1732de2dbbd2/skills/rust-send-sync), [BSD-3-Clause](rust-send-sync/LICENSE) |
| [winapp-ui-automation](winapp-ui-automation/SKILL.md) | Concrete native UIA CLI inspection, pattern invocation, JSON output and observed UI actions | [microsoft/winappCli](https://github.com/microsoft/winappCli/tree/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/plugins/winapp/skills/winapp-ui-automation), [MIT](winapp-ui-automation/LICENSE) |

## Import and local changes

Trusted skill-installer copied only selected directories from full commit pins above. Root licenses retained separately. No imported scripts, hooks, plugin/MCP configuration, dependencies or runtime executables bundled. Qualify Rust compiler/target/edition behavior and Windows CLI in actual future task; copying skills neither installs nor validates them.

Each leaf's `LOCAL-CHANGES.md` records project scope, metadata normalization, pinned optional references and technical corrections: cloning/lifetime-bound overclaims, generic Cell/marker bounds and Windows UIPI/delivery claims. Reviewed original source digests and current installed-file hashes: [UPSTREAM.json](../UPSTREAM.json). Hashes differ where documented local changes apply. Pinned local copy; no automatic update channel.

UIA skill requires qualified Windows environment with matching `winapp ui` interface when used. Installation here changes no native Win32 application choice, packaging, services or privileges. Rust examples describe upstream compiler versions; they select no NevroTick MSRV/toolchain. Existing dependency and authorization workflows retain control.

## Research decisions

Discovery used public skills directory and primary GitHub repositories; inspected candidate bodies and dependency closures, not popularity alone. Other candidates omitted for concrete reasons:

- General Rust routers and generic QA skills duplicate existing owners. Some ownership/unsafe collections lacked complete license notices or gave incorrect memory-safety remedies.
- Examined async-internals skill needs broader failure, cancellation, supervision and runtime-selection corrections; not imported.
- Linux/macOS profiling, host tuning and direct-installation packs add little to existing native Windows performance workflow.
- Trail of Bits' zeroize audit may later help actual NTS key handling; currently needs large executable/tool/role closure and buildable implementation. Plugin configuration not imported.
- Examined dimensional-analysis workflow imposes incompatible annotations/financial conventions. .NET/WinUI API-discovery workflows do not establish Rust/Win32 ABI or SCM correctness.

Bounded search found no suitable dedicated NTP/NTS or MSI/service-specific skill. Existing protocol, IPC, installer and clock owners remain appropriate guidance. Selection record; no exhaustive claim about online skills.
