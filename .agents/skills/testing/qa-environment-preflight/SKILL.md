---
name: "qa-environment-preflight"
description: "Establish tool, Cargo, target, Windows-machine and fixture readiness for selected NevroTick checks before substantial execution."
---

# Check verification readiness

Read [the testing contract](../CONTRACT.md).

Inspect current repository instructions, manifest/toolchain/lockfile, actual command expansion, targets/features and selected test identities. Verify the checkout by absolute path. A documentation-only tree has no Cargo gate to execute; do not create one to make preflight green.

Inspect selected tools' actual interfaces, installed versions and representative operation. Follow AGENTS.md for Serena session activation, SocratiCode indexing/watcher ownership, Probe and context-mode. A tool advertisement or existing index does not prove current source or execution readiness. Use a supported fallback for an unavailable optional tool; required missing capability stays incomplete.

For a proposed optional tool, identify the concrete capability gap and record the exact version/source, integrity and license, executed hooks/scripts, data sent or indexed, egress, privileges, resource use, smoke check, removal and fallback. Respect configured watcher ownership/mode; do not import another project's watcher policy. Research may proceed within the task, while installation or configuration effects need existing authorization covering them; choosing a workflow grants none.

For Rust runners, inspect build-script/proc-macro/native/SDK effects, cache/network requirements and child process controls. For native claims record Windows build, accounts, security profile, isolation and restoration owner. For UI checks require a real interactive desktop and actual UIA/Narrator path; a WSL shell is insufficient.

Read [Rust techniques](../references/rust-techniques.md) for runner/feature details and [Windows evidence](../references/windows-evidence.md) for OS setup. Record preparation, a bounded readiness result and unverified dimensions. Do not install frameworks or alter trust/service/clock configuration just to activate the skill.
