# Local compatible pinning repair

Base: `spin_on` 0.1.1, official crates.io archive SHA-256
`076e103ed41b9864aa838287efe5f4e3a7a0362dd00671ae62a212e5e4612da2`.
The published `.cargo_vcs_info.json`, original `Cargo.toml.orig`, author names,
MIT and Apache-2.0 texts are preserved. NevroTick selects the MIT grant; this
directory is third-party code, separately from the original GUI's GPL3 grant.

NevroTick changes, 6 November 2026:

- Replace `pin_utils::pin_mut!(future)` with `core::pin::pin!(future)` and retain
  its mutable binding. The standard macro requires Rust 1.68; the workspace uses
  pinned Rust 1.99.1.
- Remove the effective manifest's `pin-utils` dependency. This eliminates the
  unsupported dependency reported by RUSTSEC-2026-0441; no advisory is ignored.
- Retain a standalone frozen lock and adjust `.gitignore` so that this reviewed
  lock is versionable; keep build outputs ignored.
- Add a public-API regression for a non-Unpin future, repeated Pending polls,
  stable pinned address, returned value and exactly one Drop. Original Ready
  and counting-future tests remain intact.

The public function, no_std boundary, polling behavior and original unsafe
no-op waker code are unchanged. This is a compiler/macro dependency repair; it
adds no executor to NevroTick's engine, broker, Core or service.

Source inspection is not a test pass. The modified exact source and dependency
graph must be admitted and its checks run before this fork executes. S0.5 owns
the current evidence. Remove the patch when an inspected compatible upstream
release removes `pin-utils`, then requalify the graph and the regression.
