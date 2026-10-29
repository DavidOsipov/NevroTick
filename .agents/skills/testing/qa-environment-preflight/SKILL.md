---
name: "qa-environment-preflight"
description: "Establish tool, Cargo, target, Windows-machine and fixture readiness for selected NevroTick checks before substantial execution."
---

# Check verification readiness

Read [the testing contract](../CONTRACT.md).

Inspect current repository instructions/manifest/toolchain/lockfile/actual command expansion/targets/features/selected test identities. Verify absolute checkout path. Documentation-only tree has no Cargo gate; do not create one for green preflight.

Use [configured tool execution](../references/configured-tooling.md) for existing environment/policy checkers, tool pins and applicable Cargo/security/SBOM entry points. Run the selected readiness check or reuse matching current evidence before scheduling dependent gates. Keep local Linux/WSL admission distinct from hosted preparation and native Windows readiness.

Inspect selected tools' actual interfaces/installed versions/representative operation. Follow AGENTS.md: Serena session activation, SocratiCode indexing/watcher ownership, Probe, context-mode. Advertisement/existing index proves no current source/execution readiness. Use supported optional-tool fallback; missing required capability remains incomplete.

For Miri/cargo-fuzz/Loom/Kani/Lockbud/cargo-vet, use the guide's guarded analysis
admission and reuse only matching control evidence; hash admission alone executes
no tool. For CodeQL MCP work, read
[data-flow readiness](../references/codeql-data-flow.md): confirm this session's
exposed tools, separate CLI/query-pack admission and the actual Rust database's
source/target/features before accepting a data-flow result. A registered server
or hosted deferred gate establishes neither a connection nor an executed query.

For Application Verifier, GFlags/PageHeap or cdb, read
[native memory readiness](../references/windows-memory.md). Admit actual tools,
target/allocator coverage and healthy/faulty controls; inspect existing image
settings and a restoration owner before elevated setup. Configuration must not
elevate the ordinary-user target. No product executable means no active product
profile or product memory-safety result.

For a proposed optional tool, identify the concrete capability gap and record the exact version/source, integrity and license, executed hooks/scripts, data sent or indexed, egress, privileges, resource use, smoke check, removal and fallback. Respect configured watcher ownership/mode; do not import another project's watcher policy. Research may proceed within the task, while installation or configuration effects need existing authorization covering them; choosing a workflow grants none.

Rust runners: inspect build-script/proc-macro/native/SDK effects, cache/network requirements, child process controls. Native claims: Windows build/accounts/security profile/isolation/restoration owner. UI requires real interactive desktop/actual UIA/Narrator path; WSL shell insufficient.

Read [Rust techniques](../references/rust-techniques.md) for runner/feature details and [Windows evidence](../references/windows-evidence.md) for OS setup. Record preparation, a bounded readiness result and unverified dimensions. Do not install frameworks or alter trust/service/clock configuration just to activate the skill.
