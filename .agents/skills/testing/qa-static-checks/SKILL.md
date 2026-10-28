---
name: "qa-static-checks"
description: "Select and validate NevroTick compiler, format, Clippy, source-policy and feature/MSRV checks with meaningful admission controls."
---

# Verify Rust static enforcement

Read [the testing contract](../CONTRACT.md).

Inspect current manifests/pinned toolchain/lint configuration/cfg/feature/target selection/actual CI entry points. Confirm relevant files/generated/native sources admitted. Configured rule/installed analyzer proves no executed enforcement.

Use current [TOOLING.md](../../../../TOOLING.md) baseline once a Cargo project exists. Add only required supported matrix entries: member/default/no-default/optional features, Windows target and promised compiler. Do not blindly enable all features or entire Clippy restriction/nursery groups.

Development-policy changes: read [development-hardening guide](../../../../docs/windows-time-development-hardening.md). Proposed enforcement/qualification specification: qualify selected tool/version/target, policy inheritance, exceptions, actual violation controls before adopted/passing gate claims. Reading installs no tools, mandates no every-analyzer-per-change campaign.

Challenge changed enforcement: isolated known violation/valid companion through actual command; check nonzero propagation. Type/visibility/trait contracts need intended compile diagnostic; unresolved import insufficient. Preserve narrow reviewed exceptions/stale-expectation detection on selected compiler.

Read [Rust techniques](../references/rust-techniques.md). Static success cannot prove received bytes, current authorization, runtime cleanup or Windows effects. Documentation/skill edits need their applicable pack checks, not a fabricated product build.
