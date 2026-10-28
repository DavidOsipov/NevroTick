---
name: "qa-static-checks"
description: "Select and validate NevroTick compiler, format, Clippy, source-policy and feature/MSRV checks with meaningful admission controls."
---

# Verify Rust static enforcement

Read [the testing contract](../CONTRACT.md).

Inspect current manifests, pinned toolchain, lint configuration, cfg/feature/target selection and actual CI entry points. Confirm relevant files and generated/native sources are admitted. A configured rule or installed analyzer is not executed enforcement.

Use current [TOOLING.md](../../../../TOOLING.md) baseline once a Cargo project exists. Add only required supported matrix entries: member/default/no-default/optional features, Windows target and promised compiler. Do not blindly enable all features or entire Clippy restriction/nursery groups.

For development-policy changes, read the [development-hardening guide](../../../../docs/windows-time-development-hardening.md). It is a proposed enforcement and qualification specification: qualify the selected tool/version/target, policy inheritance, exceptions and actual violation controls before claiming an adopted or passing gate. Reading the proposal does not install its tools or require every advanced analyzer for every change.

Challenge changed enforcement with an isolated known violation and valid companion through the actual command, checking nonzero propagation. Type/visibility/trait contracts need the intended compile diagnostic, not an unresolved import. Preserve narrow reviewed exceptions and stale-expectation detection on the selected compiler.

Read [Rust techniques](../references/rust-techniques.md). Static success cannot prove received bytes, current authorization, runtime cleanup or Windows effects. Documentation/skill edits need their applicable pack checks, not a fabricated product build.
