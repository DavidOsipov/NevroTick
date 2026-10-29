//! Fixture-only presentation and checked preview state for UI-M0.
//!
//! This crate performs no network, service, policy or clock operations.

pub mod model;

#[cfg(windows)]
pub mod native;
