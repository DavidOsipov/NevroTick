# NevroTick skills

These repository skills include 52 adaptations of David Osipov's website
workflows and four selected upstream specialists for the Windows time client
described in [the documentation guide](../../docs/README.md).
Select the smallest set matching the task. Skill names and descriptions support
discovery; full instructions and conditional references load when needed. This
index is navigation, not an instruction to load every skill.

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
product decisions. In particular, use bounded Serde decoding into strict wire
records, fallible private domain construction, and current authorization and
effect checks. A native Win32 UI, unprivileged network engine, privileged clock
broker, administrative helper and complete per-machine MSI have distinct owners.
Prototype crate choices still require dependency qualification. Retained TUF
research does not adopt an updater or define a second release. Future-version
work must name the architecture decisions it changes.

The source skills' evidence discipline carries over: preserve existing work,
trace producers through state to consumers, use independent behavioral oracles,
and distinguish compiler, simulated, native Windows and packaged-product proof.
The repository currently has documentation and tooling setup; these skills do
not establish an application, dependency graph, Windows test run or release.

## Adaptation and research

[ADAPTATION.json](ADAPTATION.json) maps every original website skill to its target
and records each original entrypoint's digest. Group research records explain
the Rust and Windows decisions, with dated primary sources:

- [Implementation sources](implementation/SOURCES.md)
- [Environment sources](environment/SOURCES.md)
- [Planning sources](planning/SOURCES.md)
- [Security sources](security/SOURCES.md)
- [Testing sources](testing/SOURCES.md)
- [Attribution and the PdfCraft contribution](PROVENANCE.md)
- [Pinned upstream imports, licenses and local corrections](upstream/README.md)

Canonical files live once under this tree. Keep normal implicit discovery; no
global skills or tool installations are needed. Use explicit invocation such as
`$implement-rust-contracts` when task selection needs to be precise.
The installed Codex was verified to discover and enable all 56 grouped skills
without aliases, including the four upstream specialists.

## Maintaining the pack

Run this standard-library-only package check from the repository root:

```sh
python3 .agents/skills/scripts/check_skills.py
python3 -B .agents/skills/scripts/test_check_skills.py -v
```

It checks this pack's supported YAML subset, unique names, UI prompts, relative
Markdown links and source-to-target coverage. Entrypoint metadata requires
`name` and `description` as JSON-quoted YAML strings, with an optional quoted
`license` identifier. UI metadata uses an
`interface` mapping with quoted `display_name`, `short_description` and
`default_prompt`. Extend the checker deliberately if authoring needs change.
To compare with the original checkout when available, pass
`--source-root /path/to/personal-website-templated-1/.agents`.

The CLI regression suite checks exact skill invocations, punctuation and multiple
mentions, and rejects names with attached suffixes or a different invocation.
[UPSTREAM.json](UPSTREAM.json) separately accounts for imported leaves, exact
commits, retained licenses and their complete installed-file hashes. The checker
rejects unregistered leaves, mutable pins, license-metadata mismatches, missing
license files and file-inventory drift. It does not legally clear a license or
authenticate an unfetched remote source solely from a recorded hash.

Discovery checks and realistic independent use exercises are separate evidence.
Package checks do not test Rust application behavior. Select Cargo and Windows
checks for actual implementation changes after a Cargo workspace exists.
