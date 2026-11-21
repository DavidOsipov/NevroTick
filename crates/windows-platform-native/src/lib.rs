// SPDX-License-Identifier: GPL-2.0-only
//! Native observation adapters linked into their owning Windows process.
//!
//! The portable platform crate retains its separate service lifecycle contract.

#![deny(unsafe_code)]
#![cfg_attr(
    not(windows),
    doc = r"
The subscription and local clock readers are available on Windows builds.

```compile_fail,E0432
use windows_platform_native::ResumeWatch;
```

```compile_fail,E0432
use windows_platform_native::read_clock_coordinates;
```
"
)]

#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod clock_coordinates;
#[cfg(any(windows, test))]
#[forbid(unsafe_code)]
mod resume_slot;
#[cfg(windows)]
mod windows_adjustment;
#[cfg(windows)]
mod windows_clock;
#[cfg(windows)]
mod windows_resume;

#[cfg(any(windows, test))]
pub use clock_coordinates::{ClockCoordinates, ClockReadError};
#[cfg(windows)]
pub use windows_adjustment::{ClockAdjustment, read_clock_adjustment};
#[cfg(windows)]
pub use windows_clock::read_clock_coordinates;
#[cfg(windows)]
pub use windows_resume::{ResumeRegistrationError, ResumeUnregistrationError, ResumeWatch};
