# Selected upstream specialists

These four reviewed instructional skills supplement the 52 project-owned
adaptations. Select one for its specific diagnostic or automation question;
NevroTick's architecture, implementation/security owners and QA contract continue
to govern product decisions, effects and qualification.

| Skill | Added capability | Reviewed source and license |
| --- | --- | --- |
| [rust-compiler-errors](rust-compiler-errors/SKILL.md) | Diagnose compiler errors by ownership, trait, inference and dyn-compatibility mechanism; recheck the original failing command | [po4yka/rust-skills](https://github.com/po4yka/rust-skills/tree/5d5f4706d6bf1d03366a1fb463aa1732de2dbbd2/skills/rust-compiler-errors), [BSD-3-Clause](rust-compiler-errors/LICENSE) |
| [rust-borrow-semantics](rust-borrow-semantics/SKILL.md) | Expression-specific temporary/drop scopes, two-phase borrowing and edition-sensitive probes | [po4yka/rust-skills](https://github.com/po4yka/rust-skills/tree/5d5f4706d6bf1d03366a1fb463aa1732de2dbbd2/skills/rust-borrow-semantics), [BSD-3-Clause](rust-borrow-semantics/LICENSE) |
| [rust-send-sync](rust-send-sync/SKILL.md) | Auto-trait and task/thread-boundary diagnosis, lock payload bounds and deliberate PhantomData | [po4yka/rust-skills](https://github.com/po4yka/rust-skills/tree/5d5f4706d6bf1d03366a1fb463aa1732de2dbbd2/skills/rust-send-sync), [BSD-3-Clause](rust-send-sync/LICENSE) |
| [winapp-ui-automation](winapp-ui-automation/SKILL.md) | Concrete native UIA CLI inspection, pattern invocation, JSON output and observed UI actions | [microsoft/winappCli](https://github.com/microsoft/winappCli/tree/31fc2ac52b9340846d9b5baa0b9be634ca77a8d1/plugins/winapp/skills/winapp-ui-automation), [MIT](winapp-ui-automation/LICENSE) |

## Import and local changes

The trusted skill-installer copied only the selected directories from the full
commit pins above. Root licenses were retained separately. No imported scripts,
hooks, plugin/MCP configuration, dependencies or runtime executables are bundled.
Rust compiler/target/edition behavior and the Windows CLI need qualification in
the actual future task; copying a skill does not install or validate them.

Each leaf's `LOCAL-CHANGES.md` records project scoping, metadata normalization,
pinned optional references and technical corrections. These include cloning and
lifetime-bound overclaims, generic Cell/marker bounds and Windows UIPI/delivery
claims. The reviewed original source digests and current installed-file hashes
are in [UPSTREAM.json](../UPSTREAM.json). Source and installed hashes differ where
the documented local changes apply. This is a pinned local copy, not an automatic
update channel.

The UIA skill requires a qualified Windows environment with the matching
`winapp ui` interface when it is actually used. Its installation here does not
change the native Win32 application choice, packaging, services or privileges.
Rust examples describe upstream compiler versions; they do not select NevroTick's
MSRV or toolchain. Existing dependency and authorization workflows retain control.

## Research decisions

Discovery used the public skills directory and primary GitHub repositories;
candidate bodies and dependency closures were inspected rather than selecting
by popularity alone. Other candidates were omitted for concrete reasons:

- General Rust routers and generic QA skills substantially duplicate existing
  owners. Some ownership/unsafe collections also lacked complete license notices
  or gave incorrect memory-safety remedies.
- The examined async-internals skill needs broader corrections to failure,
  cancellation, supervision and runtime-selection advice; it was not imported.
- Profiling packs centered on Linux/macOS, host tuning and direct installations
  add little to the existing native Windows performance workflow.
- Trail of Bits' zeroize audit is a possible later specialist for actual NTS key
  handling, but currently brings a large executable/tool/role closure and needs
  a buildable implementation. Its plugin configuration was not imported.
- The examined dimensional-analysis workflow imposes annotations and financial
  conventions that do not match this project. .NET/WinUI API-discovery workflows
  do not establish Rust/Win32 ABI or SCM correctness.

No suitable dedicated NTP/NTS or MSI/service-specific skill emerged from the
bounded search. The existing protocol, IPC, installer and clock owners remain the
appropriate guidance. This is a selection record, not an exhaustive claim about
all skills available online.
