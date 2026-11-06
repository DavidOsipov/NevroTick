// Copyright NevroTick contributors
// SPDX-License-Identifier: Apache-2.0 OR MIT
//! Source-only controls for guarded per-callsite refinements. Not executed.

trait Value {
    fn get(&self) -> u64;
}

struct First;
struct Second;

impl Value for First {
    fn get(&self) -> u64 {
        17
    }
}

impl Value for Second {
    fn get(&self) -> u64 {
        29
    }
}

// Separate monomorphic sites share one trait/slot target set. Refining the first
// site to Second is an intentional same-trait omission, not an unknown target.
fn first_value() -> u64 {
    let object: &dyn Value = &First;
    object.get()
}

fn second_value() -> u64 {
    let object: &dyn Value = &Second;
    object.get()
}

#[kani::proof]
fn guarded_singleton_healthy() {
    let branch: bool = kani::any();
    if branch {
        assert_eq!(first_value(), 17, "guarded first semantic value");
    } else {
        assert_eq!(second_value(), 29, "guarded second semantic value");
    }
    kani::cover!(branch, "guarded first reachable");
    kani::cover!(!branch, "guarded second reachable");
}

#[kani::proof]
fn guarded_singleton_defect() {
    assert_eq!(first_value(), 18, "guarded target semantic defect");
}
