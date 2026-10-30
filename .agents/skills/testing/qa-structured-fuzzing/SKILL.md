---
name: "qa-structured-fuzzing"
description: "Run or design a bounded NevroTick fuzz target for structured IPC/configuration, NTP/NTS parsing or numeric conversion with useful seeds and explicit oracles."
---

# Fuzz actual parsers and conversions

Read [the testing contract](../CONTRACT.md).

Select one target/invariant: raw framed JSON admission, DTO-to-domain conversion, migration, numeric conversion, safe error projection, NTP wire, NTS/crypto. Separate targets; earlier parser rejection proves no later-guard coverage.

When reusing Chrony scenarios/corpora, read the
[upstream tools reference](../references/chrony-upstream-tools.md): chrony-fuzz's
AFL++ patches simplify protocol checks and bypass SIV/TLS for reachability. Keep
the admitted Rust engine and actual production guards; qualify seed rights,
outer-record versus payload framing, target forwarding, state reset and finite
resources. Patched acceptance supplies no independent security oracle.

Start accepted seeds/independent expected facts/named malformed boundaries. Preserve duplicate keys/raw malformed bytes through actual production decoder. Coverage-guided/structure-aware generation requires qualified runner; deterministic byte/transition cases remain useful without it.

For a new/changed adapter, use [harness qualification](../references/harness-qualification.md) to check actual forwarding with a finite accepted seed and an intended fault. An adapter that changes the selected bytes or rejects everything can hide the boundary despite a successful runner.

Cap total cases/time, bytes/depth, memory, workers, output retention, minimization. Distinguish ordinary rejection/crash/hang/launch/wait error/cleanup failure. Caught panic/zero wrapper exit cannot hide failure. Preserve original/minimized exact bytes, corpus/build/configuration hashes, toolchain, reproduction command.

Read [Rust techniques](../references/rust-techniques.md), including the conflicting upstream Windows support statements. Qualify the installed nightly/sanitizer/target with known controls. `tmin` reduces an individual failure; `cmin` reduces a coverage corpus. Neither creates Windows protected-effect evidence or permission for an overnight campaign.

Use `python3 -B scripts/check-analysis.py --tool fuzz --fixtures` through
[configured analysis](../references/configured-tooling.md#additional-rust-analysis)
for admitted cargo-fuzz/libFuzzer identities and bounded runner controls.
It does not create or qualify a product target. Keep analysis offline with owned
private work and immutable admitted inputs; do not install a floating nightly.
Preserve the explicit `--target x86_64-unknown-linux-gnu` build selection: the
admitted musl prebuilt cargo-fuzz otherwise defaults to musl.
ASan's virtual reservation requires the runner's sanitizer mode without the
ordinary address-space cap. Retain fixed finite runs/length/time/RSS/allocation
limits; they do not establish a hard cgroup RAM or total disk quota. Challenge
actual target execution with both an accepted seed and an intended fault.
