// SPDX-License-Identifier: GPL-2.0-only
//! Narrow LLVM sanitizer-coverage ABI adapter for the test-only single worker.
//!
//! The reviewed build wrapper instruments only the three real SUT crates. The
//! engine, model, standard library and LibAFL are not instrumented. LLVM supplies
//! the guard-array pointers; no user input can supply a pointer or choose an ID.

use std::sync::atomic::{AtomicBool, AtomicU8, AtomicU32, Ordering};

pub const MAP_CAPACITY: usize = 65_536;
static NEXT: AtomicU32 = AtomicU32::new(1);
static OVERFLOW: AtomicBool = AtomicBool::new(false);
static ACTIVE: AtomicBool = AtomicBool::new(false);
static EDGES: [AtomicU8; MAP_CAPACITY] = [const { AtomicU8::new(0) }; MAP_CAPACITY];

/// Initialize one compiler-owned guard array once. LLVM guarantees both pointers
/// delimit the same writable array, and constructor execution precedes campaigns.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __sanitizer_cov_trace_pc_guard_init(start: *mut u32, stop: *mut u32) {
    if start.is_null() || stop.is_null() || start >= stop {
        return;
    }
    // SAFETY: LLVM's callback ABI supplies a nonempty writable guard array.
    if unsafe { *start } != 0 {
        return;
    }
    let mut guard = start;
    while guard < stop {
        let id = NEXT.fetch_add(1, Ordering::Relaxed);
        let assigned = if (id as usize) < MAP_CAPACITY {
            id
        } else {
            OVERFLOW.store(true, Ordering::Relaxed);
            0
        };
        // SAFETY: guard remains inside the compiler-provided array, and add(1)
        // reaches at most its one-past endpoint. The array is initialized once.
        unsafe {
            *guard = assigned;
            guard = guard.add(1);
        }
    }
}

/// Record an actual reached instrumented edge. Calls are non-reentrant because
/// this runtime is deliberately outside the wrapper's instrumented crate list.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __sanitizer_cov_trace_pc_guard(guard: *mut u32) {
    if guard.is_null() || !ACTIVE.load(Ordering::Relaxed) {
        return;
    }
    // SAFETY: LLVM passes a valid guard initialized by the callback above.
    let id = unsafe { *guard } as usize;
    if id != 0 && id < MAP_CAPACITY {
        EDGES[id].store(1, Ordering::Relaxed);
    } else if id >= MAP_CAPACITY {
        OVERFLOW.store(true, Ordering::Relaxed);
    }
}

/// Begin a fresh observation. This callback must not overlap another trace;
/// fuzzer feedback history is stored separately and is intentionally retained.
pub fn begin() {
    ACTIVE.store(false, Ordering::Relaxed);
    for edge in &EDGES {
        edge.store(0, Ordering::Relaxed);
    }
    ACTIVE.store(true, Ordering::Relaxed);
}

/// End observation before reading it. Every normal driver outcome calls this.
pub fn end() {
    ACTIVE.store(false, Ordering::Relaxed);
}

/// Copy one complete bitmap into the observer; no LLVM map pointer escapes.
pub fn copy_into(destination: &mut [u8]) -> Result<(), &'static str> {
    if ACTIVE.load(Ordering::Relaxed) || destination.len() != MAP_CAPACITY {
        return Err("coverage observation has the wrong lifetime or shape");
    }
    if OVERFLOW.load(Ordering::Relaxed) {
        return Err("LLVM guard capacity exceeded; coverage is incomplete");
    }
    if NEXT.load(Ordering::Relaxed) <= 1 {
        return Err("no real SUT LLVM guards were registered");
    }
    for (output, edge) in destination.iter_mut().zip(&EDGES) {
        *output = edge.load(Ordering::Relaxed);
    }
    Ok(())
}

/// The exact installed binary's guard count, not a source-level coverage score.
pub fn registered_guards() -> usize {
    NEXT.load(Ordering::Relaxed).saturating_sub(1) as usize
}
