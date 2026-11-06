//! Fixture-only presentation and checked preview state for UI-M0.
//!
//! Time controls perform no network, service, policy or clock operations.
//! The fixed About repository action may request the user's default browser.

pub mod diagnostics_ui;
pub mod localization;
pub mod model;
pub mod notifications_ui;
pub mod policy;
pub mod preferences;
pub mod presentation;
pub mod recovery;
pub mod shell;
pub mod slint_host;
pub mod source_drafts;
pub mod source_inspection;

/// Bounded native notification-area observer, never service authority.
pub mod native_tray;
pub mod status_ui;
pub mod tray_ui;

/// Bounded authoritative raw editor drafts, separate from checked preview state.
pub mod ui_drafts;

/// Windows local-toast adapter; native qualification remains separate.
#[cfg(windows)]
pub mod native_notifications_windows;
