---
name: "qa-structured-fuzzing"
description: "Run or design a bounded NevroTick fuzz target for structured IPC/configuration, NTP/NTS parsing or numeric conversion with useful seeds and explicit oracles."
---

# Fuzz actual parsers and conversions

Read [the testing contract](../CONTRACT.md).

Select one target and invariant: raw framed JSON admission, DTO-to-domain conversion, migration, numeric conversion, safe error projection, NTP wire or NTS/crypto. Keep these distinct so rejection at an earlier parser does not masquerade as coverage of a later guard.

Start with accepted seeds plus independent expected facts and named malformed boundaries. Preserve duplicate keys/raw malformed bytes through the actual production decoder. Use coverage-guided or structure-aware generation only when the chosen runner is qualified; deterministic byte/transition cases remain useful when unavailable.

Cap total cases/time, bytes/depth, memory, workers, output retention and minimization. Distinguish ordinary rejection, crash, hang, launch/wait error and cleanup failure. A caught panic or zero wrapper exit cannot hide an actual failure. Preserve original and minimized exact bytes, corpus/build/configuration hashes, toolchain and reproduction command.

Read [Rust techniques](../references/rust-techniques.md), including the conflicting upstream Windows support statements. Qualify the installed nightly/sanitizer/target with known controls. `tmin` reduces an individual failure; `cmin` reduces a coverage corpus. Neither creates Windows protected-effect evidence or permission for an overnight campaign.
