---
name: "qa-structured-fuzzing"
description: "Run or design a bounded NevroTick fuzz target for structured IPC/configuration, NTP/NTS parsing or numeric conversion with useful seeds and explicit oracles."
---

# Fuzz actual parsers and conversions

Read [the testing contract](../CONTRACT.md).

Select one target/invariant: raw framed JSON admission, DTO-to-domain conversion, migration, numeric conversion, safe error projection, NTP wire, NTS/crypto. Separate targets; earlier parser rejection proves no later-guard coverage.

Start accepted seeds/independent expected facts/named malformed boundaries. Preserve duplicate keys/raw malformed bytes through actual production decoder. Coverage-guided/structure-aware generation requires qualified runner; deterministic byte/transition cases remain useful without it.

Cap total cases/time, bytes/depth, memory, workers, output retention, minimization. Distinguish ordinary rejection/crash/hang/launch/wait error/cleanup failure. Caught panic/zero wrapper exit cannot hide failure. Preserve original/minimized exact bytes, corpus/build/configuration hashes, toolchain, reproduction command.

Read [Rust techniques](../references/rust-techniques.md), including the conflicting upstream Windows support statements. Qualify the installed nightly/sanitizer/target with known controls. `tmin` reduces an individual failure; `cmin` reduces a coverage corpus. Neither creates Windows protected-effect evidence or permission for an overnight campaign.
