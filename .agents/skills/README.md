# NevroTick skills

52 David Osipov website workflow adaptations, four selected upstream specialists
for Windows time client in [the documentation guide](../../docs/README.md).
Select smallest task-matching set. Names/descriptions support discovery; load
full instructions/conditional references when needed. Index navigates; never
requires loading every skill.

| Task | Start with |
| --- | --- |
| Implement Rust ownership, errors, concurrency or Windows adapters | [implement-rust](implementation/implement-rust/SKILL.md) |
| Decode IPC/configuration or enforce domain invariants | [implement-rust-contracts](implementation/implement-rust-contracts/SKILL.md) |
| Change wire, persisted or domain compatibility | [evolve-data-contracts](implementation/evolve-data-contracts/SKILL.md) |
| Implement native controls, interactions or status | [implement-native-ui](implementation/implement-native-ui/SKILL.md) |
| Plan a change, architecture, review or threat model | [Planning guide](planning/README.md) |
| Change privilege, IPC, NTS, logging or installation security | [Security guide](security/README.md) |
| Choose or run testing and qualification | [Testing guide](testing/README.md) |
| Assess or change Cargo dependencies | [dependency-updates](environment/dependency-updates/SKILL.md) |
| Diagnose a specific compiler, borrow/drop or Send/Sync mechanism; use qualified Windows UIA commands | [Selected upstream specialists](upstream/README.md) |

The [canonical architecture](../../docs/windows-time-architecture.md) controls
product decisions. Use bounded Serde decoding into strict wire records, fallible
private domain construction, current authorization/effect checks. Native Win32
UI, unprivileged network engine, privileged clock broker, administrative helper,
complete per-machine MSI have distinct owners. Prototype crates require
dependency qualification. Retained TUF research adopts no updater or second
release. Future-version work must name changed architecture decisions.

Preserve source skills' evidence discipline: existing work, producer → state →
consumer tracing, independent behavioral oracles; distinguish compiler, simulated,
native Windows, packaged-product proof. Repository currently has documentation/
tooling; skills establish no application, dependency graph, Windows test run or release.

## Adaptation and research

[ADAPTATION.json](ADAPTATION.json) maps every original website skill to target and
original entrypoint digest. Group research explains Rust/Windows decisions with
dated primary sources:

- [Implementation sources](implementation/SOURCES.md)
- [Environment sources](environment/SOURCES.md)
- [Planning sources](planning/SOURCES.md)
- [Security sources](security/SOURCES.md)
- [Testing sources](testing/SOURCES.md)
- [Attribution and the PdfCraft contribution](PROVENANCE.md)
- [Pinned upstream imports, licenses and local corrections](upstream/README.md)

Canonical files live once here. Keep normal implicit discovery; no global skill/
tool installation needed. Explicit invocation, e.g. `$implement-rust-contracts`,
for precise selection. Installed Codex was verified to discover/enable all 56
grouped skills without aliases, including four upstream specialists.

## Maintaining the pack

Run standard-library-only package check from repository root:

```sh
python3 .agents/skills/scripts/check_skills.py
python3 -B .agents/skills/scripts/test_check_skills.py -v
```

Checks supported YAML subset, unique names, UI prompts, relative Markdown links,
source-to-target coverage. Entrypoints require JSON-quoted YAML `name` and
`description`, optional quoted `license` identifier. UI metadata: `interface`
mapping with quoted `display_name`, `short_description`, `default_prompt`. Extend
checker deliberately for changed authoring needs. Compare available original
checkout via `--source-root /path/to/personal-website-templated-1/.agents`.

CLI regressions check exact skill invocations, punctuation, multiple mentions;
reject attached name suffixes/different invocation. [UPSTREAM.json](UPSTREAM.json)
accounts separately for imported leaves, exact commits, retained licenses,
complete installed-file hashes. Checker rejects unregistered leaves, mutable pins,
license-metadata mismatches, missing license files, inventory drift. Recorded
hash alone neither legally clears license nor authenticates unfetched remote source.

Discovery checks/realistic independent use exercises provide separate evidence.
Package checks do not test Rust behavior. Select Cargo/Windows checks for actual
implementation changes after Cargo workspace exists.
