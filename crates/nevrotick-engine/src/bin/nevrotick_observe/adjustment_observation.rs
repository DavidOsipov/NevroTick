// SPDX-License-Identifier: GPL-2.0-only
//! Bookends for one requested read-only adjustment observation.

use std::io::{self, Write};
use std::time::Instant;

/// Raw native values, with no conversion to elapsed or UTC time.
#[derive(Clone, Copy, Eq, PartialEq)]
pub(super) struct AdjustmentState {
    /// Original adjusted clock update word.
    pub(super) adjustment: u64,
    /// Original clock update word.
    pub(super) increment: u64,
    /// Whether Windows reported periodic programmed adjustment disabled.
    pub(super) periodic_adjustment_disabled: bool,
}

/// Bounds one native read or its original failure in the same monotonic domain.
struct BracketedRead {
    /// Monotonic reading immediately before the adapter call.
    start: Instant,
    /// Monotonic reading immediately after the adapter returned.
    end: Instant,
    /// Actual raw values or the original native error.
    state: Result<AdjustmentState, u32>,
}

impl BracketedRead {
    /// Sample the bracket around exactly one bounded read.
    fn capture(
        read: &mut impl FnMut() -> Result<AdjustmentState, u32>,
        now: &mut impl FnMut() -> Instant,
    ) -> Self {
        let start: Instant = now();
        let state: Result<AdjustmentState, u32> = read();
        let end: Instant = now();
        Self { start, end, state }
    }

    /// Emit one bounded record. Offsets exist only for an ordered pair of brackets.
    ///
    /// # Errors
    /// Preserve the first sink refusal.
    fn write(
        &self,
        output: &mut impl Write,
        point: &'static str,
        offsets: Option<[u128; 2]>,
    ) -> io::Result<()> {
        write!(output, "event=clock_adjustment point={point}")?;
        if let Some([start, end]) = offsets {
            write!(output, " bracket_start_ns={start} bracket_end_ns={end}")?;
        }
        match self.state {
            Ok(state) => writeln!(
                output,
                " status=observed adjustment={} increment={} periodic_disabled={}",
                state.adjustment,
                state.increment,
                u8::from(state.periodic_adjustment_disabled)
            ),
            Err(error) => writeln!(output, " status=unavailable error={error}"),
        }
    }
}

/// Historical adjustment readings associated with one bounded operation.
#[must_use = "retain the two read observations with the original operation outcome"]
pub(super) struct AdjustmentObservation {
    /// Read completed before dispatching the operation.
    before: BracketedRead,
    /// Read started after the operation returned.
    after: BracketedRead,
}

impl AdjustmentObservation {
    /// Convert to invocation-relative offsets only when the original reads are ordered.
    fn ordered_offsets(&self) -> Option<[u128; 3]> {
        let origin: Instant = self.before.start;
        if origin > self.before.end
            || self.before.end > self.after.start
            || self.after.start > self.after.end
        {
            return None;
        }
        Some([
            self.before.end.checked_duration_since(origin)?.as_nanos(),
            self.after.start.checked_duration_since(origin)?.as_nanos(),
            self.after.end.checked_duration_since(origin)?.as_nanos(),
        ])
    }

    /// Emit two original readings and one bounded summary before completion.
    /// Equal readings carry no mapping or continuity guarantee.
    ///
    /// # Errors
    /// Return an output error before the caller can report normal completion.
    pub(super) fn write(&self, output: &mut impl Write) -> io::Result<()> {
        let (ordering, state): (&'static str, &'static str) =
            if let Some([before_end, after_start, after_end]) = self.ordered_offsets() {
                self.before.write(output, "before", Some([0, before_end]))?;
                self.after
                    .write(output, "after", Some([after_start, after_end]))?;
                let state: &'static str = match (self.before.state, self.after.state) {
                    (Ok(before), Ok(after)) if before == after => "equal",
                    (Ok(_), Ok(_)) => "changed",
                    _ => "unavailable",
                };
                ("ordered", state)
            } else {
                self.before.write(output, "before", None)?;
                self.after.write(output, "after", None)?;
                ("unavailable", "unavailable")
            };
        writeln!(
            output,
            "event=clock_adjustment_summary state={state} ordering={ordering} mapping=unavailable"
        )
    }
}

/// Read before and after the one operation, preserving the operation's original result.
/// Read refusal cannot replace that result or make the operation eligible.
pub(super) fn observe<T>(
    read: impl FnMut() -> Result<AdjustmentState, u32>,
    operation: impl FnOnce() -> T,
) -> (T, AdjustmentObservation) {
    observe_with_clock(read, operation, Instant::now)
}

/// Consume time observations at each actual read boundary.
fn observe_with_clock<T>(
    mut read: impl FnMut() -> Result<AdjustmentState, u32>,
    operation: impl FnOnce() -> T,
    mut now: impl FnMut() -> Instant,
) -> (T, AdjustmentObservation) {
    let before: BracketedRead = BracketedRead::capture(&mut read, &mut now);
    let result: T = operation();
    let after: BracketedRead = BracketedRead::capture(&mut read, &mut now);
    (result, AdjustmentObservation { before, after })
}

#[cfg(test)]
#[path = "../../../tests/private/adjustment_observation_clock.rs"]
mod clock_tests;
