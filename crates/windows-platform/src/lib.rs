// SPDX-License-Identifier: GPL-2.0-only
//! Narrow shared mechanics for `NevroTick`'s separate Windows process roles.
//!
//! The initial service owner is portable source preparation. It performs no
//! Windows calls, installs no service and grants no networking or clock authority.
//! Native hosting, identity admission and package provisioning remain separate.
//!
//! A host-only example supplies an explicit test reporter and laboratory limits:
//! ```
//! use std::{convert::Infallible, num::NonZeroU32, time::Instant};
//! use windows_platform::service::{PendingBudget, ServiceLifecycle, ServiceRole,
//!     ServiceStatus, StatusReporter, StopOutcome};
//! struct TestReporter;
//! impl StatusReporter for TestReporter {
//!     type Error = Infallible;
//!     fn report(&mut self, _: ServiceStatus) -> Result<(), Infallible> { Ok(()) }
//! }
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! let limits = PendingBudget::new(10, 100)?;
//! let cap = NonZeroU32::MIN;
//! let mut service = ServiceLifecycle::new(ServiceRole::Engine, TestReporter,
//!     limits, limits, cap);
//! let now = Instant::now();
//! service.start(now)?;
//! service.ready(now)?;
//! let work = service.begin_work(now)?;
//! service.request_stop(now)?;
//! work.complete(); // The adapter calls this only after actual native completion.
//! service.finish_stop(now, StopOutcome::Clean)?;
//! # Ok(()) }
//! ```
//!
//! Pending budgets cannot bypass mandatory checked construction:
//! ```compile_fail,E0451
//! use windows_platform::service::PendingBudget;
//! let invalid = PendingBudget { wait_hint_ms: 0, timeout_ms: 0 };
//! ```
//!
//! Completion permits cannot be duplicated:
//! ```compile_fail,E0599
//! use windows_platform::service::WorkPermit;
//! fn duplicate(permit: WorkPermit) { let copied = permit.clone(); }
//! ```
#![forbid(unsafe_code)]

pub mod privilege_observation;
pub mod scm;
pub mod service;
