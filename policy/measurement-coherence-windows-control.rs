// SPDX-License-Identifier: GPL-2.0-only
//! Source-linked subtraction surrogate for the private coherence-residual helper.
//!
//! This standalone fixture is not a Cargo member, product clock provider or
//! native Windows observation. The admitted external contract runner replaces
//! the single source marker below with the complete current `coherence_residual`
//! function from `crates/nevrotick-engine/src/measurement.rs`, without rewriting
//! its body. The runner binds the template, inserted source, compiler, commands,
//! executable, output and actual exits before and after execution.
//!
//! Only the clock primitive is substituted: the test-local representation gives
//! exact ordering and Windows' small-negative-to-zero checked subtraction. The
//! chosen one-nanosecond epsilon is a fixture parameter, not an observed host QPC
//! frequency or a product allowance. Pinned Rust 1.99.1 implements that tolerance
//! in `library/std/src/sys/time/windows.rs` at compiler commit
//! `58cf8f9c35b355199727627d07ae34a4e2846d67`; its epsilon comes from
//! `library/std/src/sys/pal/windows/time.rs`. A host/native clock is never read.
//!
//! Healthy neighbors and primitive calibration precede the discriminating final
//! crossed-bracket assertion. In a separate isolated source variant, remove only
//! the newly added cross-bracket guard from the inserted actual helper. That
//! variant must fail the final assertion, while the repaired source succeeds.
//! The assertion is an independent expected rejection, not an injected panic.
//! Neither result is native Windows RED/GREEN or a product mutation score.
#![deny(warnings, unsafe_code)]
#![deny(clippy::all, clippy::pedantic)]

use std::time::Duration;

/// Fixed surrogate parameter for a Windows backend with a positive epsilon.
const WINDOWS_EPSILON_NS: u64 = 1;

/// Exact synthetic coordinate; ordering never applies subtraction tolerance.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ClockInstant {
    /// Synthetic nanoseconds, independent of wall time and every host clock.
    nanoseconds: u64,
}

impl ClockInstant {
    /// Reproduce the pinned Windows small-negative checked-subtraction behavior.
    /// A negative difference beyond the fixed epsilon remains unavailable.
    fn checked_duration_since(self, earlier: Self) -> Option<Duration> {
        if let Some(backward) = earlier.nanoseconds.checked_sub(self.nanoseconds)
            && backward > 0
            && backward <= WINDOWS_EPSILON_NS
        {
            Some(Duration::ZERO)
        } else {
            self.nanoseconds
                .checked_sub(earlier.nanoseconds)
                .map(Duration::from_nanos)
        }
    }
}

/// Only the capture fields consumed by the exact inserted production helper.
#[derive(Clone, Copy)]
struct SoftwareClock {
    /// Selected lower software capture coordinate.
    mono: ClockInstant,
    /// Selected upper software capture coordinate.
    mono_after: ClockInstant,
}

/// Only the converted-wall field consumed by the exact inserted helper.
#[derive(Clone, Copy)]
struct ConvertedWall {
    /// Independently supplied exact synthetic wall nanoseconds.
    nanoseconds: i128,
}

/// The actual helper's complete refusal vocabulary; no native errors are modeled.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum MeasurementRefusal {
    /// An individual or cross-bracket ordering rule failed.
    CaptureOrder,
    /// Checked conversion or gap arithmetic cannot represent the result.
    Arithmetic,
}

/// Preserve the production duration-narrowing context for the inserted helper.
/// # Errors
/// Return Arithmetic when the unsigned nanoseconds cannot fit the signed result.
fn signed_nanoseconds(value: Duration) -> Result<i128, MeasurementRefusal> {
    i128::try_from(value.as_nanos()).map_err(|_| MeasurementRefusal::Arithmetic)
}

// NEVROTICK_COHERENCE_RESIDUAL_SOURCE

/// Construct a literal raw bracket; malformed values deliberately remain raw.
const fn capture(lower: u64, upper: u64) -> SoftwareClock {
    SoftwareClock {
        mono: ClockInstant { nanoseconds: lower },
        mono_after: ClockInstant { nanoseconds: upper },
    }
}

/// Construct an independent synthetic wall value, without conversion or I/O.
const fn wall(nanoseconds: i128) -> ConvertedWall {
    ConvertedWall { nanoseconds }
}

/// Run finite known-answer controls, then require the discriminating rejection.
/// # Panics
/// Fail when the surrogate primitive, a healthy neighbor or required rejection
/// differs from its independently specified result.
fn main() {
    let earlier = ClockInstant { nanoseconds: 10 };
    let later = ClockInstant { nanoseconds: 11 };
    let before_epsilon = ClockInstant { nanoseconds: 9 };
    assert!(earlier < later);
    assert_eq!(
        later.checked_duration_since(earlier),
        Some(Duration::from_nanos(1))
    );
    assert_eq!(earlier.checked_duration_since(later), Some(Duration::ZERO));
    assert_eq!(before_epsilon.checked_duration_since(later), None);

    // Touching equality is legal; the wall delta lies inside [0, 2] ns.
    assert_eq!(
        coherence_residual(capture(0, 1), capture(1, 2), wall(0), wall(1)),
        Ok(0)
    );
    // Separated brackets yield [2, 4] ns; zero and one-nanosecond gaps differ.
    assert_eq!(
        coherence_residual(capture(0, 1), capture(3, 4), wall(0), wall(3)),
        Ok(0)
    );
    assert_eq!(
        coherence_residual(capture(0, 1), capture(3, 4), wall(0), wall(5)),
        Ok(1)
    );
    assert_eq!(
        coherence_residual(capture(1, 0), capture(2, 3), wall(0), wall(2)),
        Err(MeasurementRefusal::CaptureOrder)
    );
    assert_eq!(
        coherence_residual(capture(0, 1), capture(3, 2), wall(0), wall(2)),
        Err(MeasurementRefusal::CaptureOrder)
    );
    // A larger negative interval is refused even by the tolerant primitive.
    assert_eq!(
        coherence_residual(capture(0, 2), capture(0, 3), wall(0), wall(0)),
        Err(MeasurementRefusal::CaptureOrder)
    );

    // Own brackets are valid; cross-order is wrong by exactly the chosen epsilon.
    // Without the new explicit guard, the inserted helper instead returns Ok(0).
    assert_eq!(
        coherence_residual(capture(0, 1), capture(0, 1), wall(0), wall(0)),
        Err(MeasurementRefusal::CaptureOrder),
        "WINDOWS_EPSILON_CROSS_GUARD_REQUIRED"
    );
    println!("measurement-coherence-windows-surrogate:healthy");
}
