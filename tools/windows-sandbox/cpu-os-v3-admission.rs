// SPDX-License-Identifier: GPL-2.0-only
//! Laboratory CPU/OS admission diagnostic, not a product startup guard.
//! Compile only this std-only diagnostic for baseline x86-64; product stays v3.
//! Reads bounded CPU/OS facts, emits one fixed line (<=33 bytes), exits 0/1.
//! Stdout only: no arguments, filesystem path access, network, services or clock.

// Match the workspace's Rust/Clippy/rustdoc deny policy on 9 November 2026.
#![deny(
    warnings,
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    unused_unsafe,
    unused_must_use,
    let_underscore_drop,
    unused_macro_rules,
    unused_lifetimes,
    unfulfilled_lint_expectations,
    unknown_lints,
    renamed_and_removed_lints,
    improper_ctypes,
    improper_ctypes_definitions,
    unexpected_cfgs,
    missing_docs,
    unreachable_pub,
    unused_qualifications,
    clippy::all,
    clippy::pedantic,
    clippy::missing_docs_in_private_items,
    clippy::undocumented_unsafe_blocks,
    clippy::allow_attributes_without_reason,
    clippy::allow_attributes,
    clippy::multiple_unsafe_ops_per_block,
    clippy::unused_result_ok,
    clippy::string_slice,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::todo,
    clippy::unimplemented,
    clippy::unreachable,
    clippy::dbg_macro,
    clippy::let_underscore_must_use,
    clippy::as_conversions,
    clippy::mem_forget,
    clippy::exit,
    rustdoc::all,
    rustdoc::broken_intra_doc_links,
    rustdoc::private_intra_doc_links
)]

#[cfg(not(all(windows, target_arch = "x86_64")))]
compile_error!("This diagnostic is exclusively for an x64 Windows guest.");

use std::{
    arch::x86_64,
    io::{self, Write},
    process::ExitCode,
};

/// Inert CPU facts with no cross-field support invariant.
/// Every valid typed `Facts` value is permitted as predicate input.
#[derive(Clone, Copy)]
struct Facts {
    /// Maximum basic CPUID leaf, or a deliberately inadequate synthetic value.
    basic_max: u32,
    /// Maximum extended CPUID leaf, or an inadequate synthetic value.
    extended_max: u32,
    /// CPUID1 ECX feature bits; zero when the queried leaf is unavailable.
    leaf1_ecx: u32,
    /// CPUID7/subleaf0 EBX feature bits; zero when unavailable.
    leaf7_ebx: u32,
    /// Extended CPUID1 ECX feature bits; zero when unavailable.
    extended1_ecx: u32,
    /// XCR0 value, absent when its instruction cannot safely be executed.
    xcr0: Option<u64>,
}

/// CPUID1: SSE3, SSSE3, FMA, CX16, SSE4.1/4.2, MOVBE, POPCNT,
/// XSAVE, OSXSAVE, AVX, F16C. x64 Windows supplies the v1 baseline.
const LEAF1_REQUIRED: u32 = 0x3cd8_3201;
/// CPUID7/subleaf0: BMI1, AVX2, BMI2.
const LEAF7_REQUIRED: u32 = 0x0000_0128;
/// Extended CPUID1: LAHF/SAHF and LZCNT.
const EXTENDED1_REQUIRED: u32 = 0x0000_0021;
/// CPU XSAVE support plus OS CR4.OSXSAVE enablement required before XGETBV.
const XSAVE_AND_OSXSAVE: u32 = (1 << 26) | (1 << 27);

/// Every valid typed `Facts` value is accepted; complete v2/v3 and OS state pass.
/// Pure and constant work; never executes CPU instructions or changes input.
fn admitted(facts: &Facts) -> bool {
    facts.basic_max >= 7
        && facts.extended_max >= 0x8000_0001
        && facts.leaf1_ecx & LEAF1_REQUIRED == LEAF1_REQUIRED
        && facts.leaf7_ebx & LEAF7_REQUIRED == LEAF7_REQUIRED
        && facts.extended1_ecx & EXTENDED1_REQUIRED == EXTENDED1_REQUIRED
        && matches!(facts.xcr0, Some(value) if value & 7 == 7)
}

/// Requires execution as an x64 Windows diagnostic, not a v3 product image.
/// Returns actual bounded facts; unsupported leaves/state remain absent/zero.
/// CPUID is guaranteed on x64; XGETBV is guarded by CPU and OS enablement.
fn observe() -> Facts {
    let basic_max = x86_64::__cpuid_count(0, 0).eax;
    let extended_max = x86_64::__cpuid_count(0x8000_0000, 0).eax;
    let mut facts = Facts {
        basic_max,
        extended_max,
        leaf1_ecx: 0,
        leaf7_ebx: 0,
        extended1_ecx: 0,
        xcr0: None,
    };
    if basic_max >= 1 {
        facts.leaf1_ecx = x86_64::__cpuid_count(1, 0).ecx;
    }
    if basic_max >= 7 {
        facts.leaf7_ebx = x86_64::__cpuid_count(7, 0).ebx;
    }
    if extended_max >= 0x8000_0001 {
        facts.extended1_ecx = x86_64::__cpuid_count(0x8000_0001, 0).ecx;
    }
    if facts.leaf1_ecx & XSAVE_AND_OSXSAVE == XSAVE_AND_OSXSAVE {
        #[expect(
            unsafe_code,
            reason = "CPUID established XSAVE and OSXSAVE; read XCR0 only."
        )]
        // SAFETY: CPUID reports XSAVE support and CR4.OSXSAVE enablement;
        // XGETBV index0 is therefore available. No extended-state write occurs.
        let xcr0 = unsafe { x86_64::_xgetbv(0) };
        facts.xcr0 = Some(xcr0);
    }
    facts
}

/// Independent literal healthy facts and every missing required-bit neighbor.
/// Pure finite controls; false blocks machine observation/admission in main.
fn controls_pass() -> bool {
    let healthy = Facts {
        basic_max: 7,
        extended_max: 0x8000_0001,
        leaf1_ecx: 0x3cd8_3201,
        leaf7_ebx: 0x128,
        extended1_ecx: 0x21,
        xcr0: Some(7),
    };
    if !admitted(&healthy) {
        return false;
    }
    let higher_maxima = Facts {
        basic_max: 0x20,
        extended_max: 0x8000_0008,
        ..healthy
    };
    if !admitted(&higher_maxima) {
        return false;
    }
    for bit in [0, 9, 12, 13, 19, 20, 22, 23, 26, 27, 28, 29] {
        let mut missing = healthy;
        missing.leaf1_ecx &= !(1 << bit);
        if admitted(&missing) {
            return false;
        }
    }
    for bit in [3, 5, 8] {
        let mut missing = healthy;
        missing.leaf7_ebx &= !(1 << bit);
        if admitted(&missing) {
            return false;
        }
    }
    for bit in [0, 5] {
        let mut missing = healthy;
        missing.extended1_ecx &= !(1 << bit);
        if admitted(&missing) {
            return false;
        }
    }
    for xcr0 in [
        None,
        Some(0),
        Some(1),
        Some(2),
        Some(3),
        Some(4),
        Some(5),
        Some(6),
    ] {
        if admitted(&Facts { xcr0, ..healthy }) {
            return false;
        }
    }
    for basic_max in [0, 1, 6] {
        if admitted(&Facts {
            basic_max,
            ..healthy
        }) {
            return false;
        }
    }
    for extended_max in [0, 0x8000_0000] {
        if admitted(&Facts {
            extended_max,
            ..healthy
        }) {
            return false;
        }
    }
    admitted(&Facts {
        leaf1_ecx: u32::MAX,
        leaf7_ebx: u32::MAX,
        extended1_ecx: u32::MAX,
        xcr0: Some(u64::MAX),
        ..healthy
    })
}

/// Fixed bounded stdout only; observation/control/output failure returns 1.
/// Exit0 means this diagnostic observed admission, not product qualification.
fn main() -> ExitCode {
    let (message, accepted): (&[u8], bool) = if !controls_pass() {
        (b"cpu-os-v3=control-failed\n", false)
    } else if admitted(&observe()) {
        (b"cpu-os-v3=accepted\n", true)
    } else {
        (b"cpu-os-v3=refused\n", false)
    };
    let written = io::stdout().lock().write_all(message).is_ok();
    ExitCode::from(u8::from(!(accepted && written)))
}
