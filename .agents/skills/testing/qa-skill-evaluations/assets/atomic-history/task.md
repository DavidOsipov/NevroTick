# Repair bounded history admission

This is a synthetic Rust exercise, independent of the product workspace. Modify
only your supplied `history.rs` and your own tests in this directory. Keep its
public API. No dependencies, network, filesystem effects in the implementation,
unsafe code or changes elsewhere are needed.

`History` starts empty and retains the newest three accepted `i32` samples, in
arrival order. Accepted values are every integer in the inclusive range
`[-100, 100]`. `push` returns `Err(OutOfRange)` for every other value and preserves
the entire previous observable history. A successful push appends exactly once;
when full, it removes exactly the oldest sample. `samples()` exposes a shared
read-only slice. These limits belong only to this exercise.

Inspect the candidate, repair any contract violation and add meaningful regression
tests with healthy controls. Keep the repair proportional. Demonstrate the
original behavioral failure before repair where practical; unrelated compilation
failures are not behavioral evidence. Report actual commands, counts/results and
remaining limits. Use the admitted host Rust compiler; no Cargo setup is needed.
