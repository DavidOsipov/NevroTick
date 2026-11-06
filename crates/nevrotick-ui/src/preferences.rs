//! Accepted, process-local GUI preferences without raw editor drafts or OS state.
//!
//! Custom intervals are whole minutes, never seconds or a scheduling algorithm.
//! Each raw admission is bounded before validation or parsing, and rejected
//! updates preserve all accepted preferences. The view owns drafts and uses
//! accepted getters to discard them on Cancel. No operation reads or writes
//! external state, manages an observer, publishes notifications or exits the UI.
#![forbid(unsafe_code)]

use std::{fmt, num::NonZeroU64};

use garde::Validate;

/// Maximum raw custom-minute bytes, including leading zeroes.
///
/// Twenty decimal digits cover every `u64` value while placing a finite work
/// bound on admission. This is a representation limit, not a scheduling policy.
pub const MAX_CUSTOM_MINUTES_BYTES: usize = 20;

/// A bounded preference rejection containing no rejected input or OS status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreferenceError {
    /// Custom minutes are absent, oversized, non-decimal, zero or outside `u64`.
    InvalidCustomMinutes,
    /// Opening at sign-in was requested while the tray preference was disabled.
    TrayRequired,
}

impl PreferenceError {
    /// Return fixed feedback without formatting or retaining rejected bytes.
    #[must_use]
    pub const fn text(self) -> &'static str {
        match self {
            Self::InvalidCustomMinutes => {
                "Enter whole minutes of at least 1, using digits only. For a very large value, try a smaller number."
            }
            Self::TrayRequired => "Enable the tray preference before opening at sign-in.",
        }
    }
}

impl fmt::Display for PreferenceError {
    /// Emit only code-owned feedback; no rejected input is available to format.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.text())
    }
}

impl std::error::Error for PreferenceError {}

/// Borrowed raw custom text with mandatory shared lexical validation.
#[derive(Validate)]
struct CustomMinutesInput<'a> {
    /// Raw decimal text, bounded before garde can allocate a one-field report.
    #[garde(custom(custom_minutes_rule))]
    value: &'a str,
}

/// Admit bounded nonempty ASCII decimal syntax without copying or normalizing.
///
/// # Errors
/// Returns a fixed lexical rejection; nonzero/range admission belongs to parsing
/// into `NonZeroU64` in `CustomMinutes::parse`.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde requires custom rules to borrow their context"
)]
fn custom_minutes_rule(value: &str, (): &()) -> garde::Result {
    if !value.is_empty()
        && value.len() <= MAX_CUSTOM_MINUTES_BYTES
        && value.bytes().all(|byte| byte.is_ascii_digit())
    {
        Ok(())
    } else {
        Err(garde::Error::new("invalid custom minutes"))
    }
}

/// An immutable nonzero whole-minute GUI preference, with no scheduling effect.
///
/// Raw construction is unavailable to external callers:
/// ```compile_fail,E0423
/// use nevrotick_ui::preferences::CustomMinutes;
/// let minutes = CustomMinutes(0);
/// ```
/// Checked construction admits leading zeroes before canonical formatting:
/// ```
/// use nevrotick_ui::preferences::CustomMinutes;
/// let minutes = CustomMinutes::parse("0007")?;
/// assert_eq!(minutes.get(), 7);
/// assert_eq!(minutes.to_string(), "7");
/// # Ok::<(), nevrotick_ui::preferences::PreferenceError>(())
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CustomMinutes(NonZeroU64);

impl CustomMinutes {
    /// Borrow and admit at most 20 ASCII decimal bytes into nonzero `u64` minutes.
    ///
    /// Input is bounded before validation/report allocation and parsing. No raw
    /// bytes are copied or retained. Successful formatting normalizes leading
    /// zeroes by formatting the accepted integer, never by editing raw input.
    ///
    /// # Errors
    /// Rejects empty, oversized, signed, spaced, fractional, exponent, non-ASCII,
    /// zero and overflowing input with fixed feedback and no external effect.
    pub fn parse(raw: &str) -> Result<Self, PreferenceError> {
        if raw.len() > MAX_CUSTOM_MINUTES_BYTES {
            return Err(PreferenceError::InvalidCustomMinutes);
        }
        let input = CustomMinutesInput { value: raw };
        input
            .validate()
            .map_err(|_| PreferenceError::InvalidCustomMinutes)?;
        raw.parse::<NonZeroU64>()
            .map(Self)
            .map_err(|_| PreferenceError::InvalidCustomMinutes)
    }

    /// Return accepted whole minutes without conversion or any scheduling effect.
    #[must_use]
    pub const fn get(self) -> u64 {
        self.0.get()
    }
}

impl Default for CustomMinutes {
    /// Supply the prototype's initial retained custom value of one whole minute.
    fn default() -> Self {
        Self(NonZeroU64::MIN)
    }
}

impl fmt::Display for CustomMinutes {
    /// Format the accepted integer; default formatting is canonical decimal text.
    ///
    /// The default format contains at most 20 digits and no leading zeroes.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, formatter)
    }
}

/// A saved GUI interval choice, carrying custom values only after admission.
///
/// Presets retain their distinct UI identity even when a custom value has the
/// same number of minutes. This type grants no engine or clock authority.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum CheckInterval {
    /// Initial automatic preference; no algorithm is implemented here.
    #[default]
    Automatic,
    /// Suggested interval of two whole minutes.
    TwoMinutes,
    /// Suggested interval of five whole minutes.
    FiveMinutes,
    /// Suggested interval of fifteen whole minutes.
    FifteenMinutes,
    /// Explicit custom whole minutes accepted by `CustomMinutes`.
    Custom(CustomMinutes),
}

/// Accepted GUI preferences, independent of OS availability and observer state.
///
/// Tray and sign-in preferences start disabled; disabling tray clears sign-in.
/// Notifications start enabled and changing them affects no incident, history,
/// observer or OS state. Raw interval drafts remain view-owned. There is no Quit
/// or reset operation: exiting an observer must not reset accepted preferences.
/// Direct field mutation is unavailable to external callers:
/// ```compile_fail,E0616
/// use nevrotick_ui::preferences::UiPreferences;
/// let mut preferences = UiPreferences::default();
/// preferences.sign_in_enabled = true;
/// ```
/// Legal sign-in changes use the current tray preference:
/// ```
/// use nevrotick_ui::preferences::UiPreferences;
/// let mut preferences = UiPreferences::default();
/// preferences.set_tray_enabled(true);
/// preferences.set_sign_in_enabled(true)?;
/// assert!(preferences.sign_in_enabled());
/// # Ok::<(), nevrotick_ui::preferences::PreferenceError>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiPreferences {
    /// Accepted interval choice, independent of any in-progress view draft.
    interval: CheckInterval,
    /// Last accepted custom value, retained across automatic/preset choices.
    last_custom_minutes: CustomMinutes,
    /// User preference only, not notification permission or delivery evidence.
    notifications_enabled: bool,
    /// User preference only, not evidence that a tray observer exists.
    tray_enabled: bool,
    /// User preference only; true requires `tray_enabled` in this instance.
    sign_in_enabled: bool,
}

impl UiPreferences {
    /// Return the accepted interval for presentation or discarding view drafts.
    #[must_use]
    pub const fn interval(&self) -> CheckInterval {
        self.interval
    }

    /// Return the last accepted custom minutes for presentation or Cancel.
    ///
    /// Reading this value neither selects Custom nor mutates accepted state.
    #[must_use]
    pub const fn last_custom_minutes(&self) -> CustomMinutes {
        self.last_custom_minutes
    }

    /// Accept an already checked interval choice without external effects.
    ///
    /// Custom updates both the interval and retained custom value; Automatic and
    /// presets retain the last custom value. The typed input admits no invalid
    /// custom value, so this operation cannot fail and changes no other field.
    pub fn set_interval(&mut self, interval: CheckInterval) {
        if let CheckInterval::Custom(minutes) = interval {
            self.last_custom_minutes = minutes;
        }
        self.interval = interval;
    }

    /// Admit borrowed raw minutes, then atomically select and retain Custom.
    ///
    /// Success returns the accepted value for canonical view formatting and
    /// changes only the accepted interval and retained custom minutes.
    ///
    /// # Errors
    /// Invalid or oversized input preserves every preference. The view retains
    /// its raw draft while showing the fixed error; rejected bytes are not kept.
    pub fn set_custom_minutes(&mut self, raw: &str) -> Result<CustomMinutes, PreferenceError> {
        let minutes = CustomMinutes::parse(raw)?;
        self.set_interval(CheckInterval::Custom(minutes));
        Ok(minutes)
    }

    /// Return the user's notification preference without observing OS state.
    #[must_use]
    pub const fn notifications_enabled(&self) -> bool {
        self.notifications_enabled
    }

    /// Change only the notification preference, without delivery or state effects.
    pub fn set_notifications_enabled(&mut self, enabled: bool) {
        self.notifications_enabled = enabled;
    }

    /// Return the accepted tray preference without checking observer availability.
    #[must_use]
    pub const fn tray_enabled(&self) -> bool {
        self.tray_enabled
    }

    /// Change the tray preference and clear sign-in when disabling tray.
    ///
    /// Enabling tray preserves sign-in's current value. No observer or startup
    /// registration is created or removed; all other preferences are preserved.
    pub fn set_tray_enabled(&mut self, enabled: bool) {
        self.tray_enabled = enabled;
        if !enabled {
            self.sign_in_enabled = false;
        }
    }

    /// Return the accepted sign-in preference without reading startup settings.
    #[must_use]
    pub const fn sign_in_enabled(&self) -> bool {
        self.sign_in_enabled
    }

    /// Change only sign-in's preference after checking the current tray choice.
    ///
    /// Disabling sign-in is always permitted and changes no tray, notification
    /// or interval preference. No OS startup registration is changed.
    ///
    /// # Errors
    /// Enabling sign-in while tray is disabled returns `TrayRequired`, preserving
    /// every preference without an external effect.
    pub fn set_sign_in_enabled(&mut self, enabled: bool) -> Result<(), PreferenceError> {
        if enabled && !self.tray_enabled {
            return Err(PreferenceError::TrayRequired);
        }
        self.sign_in_enabled = enabled;
        Ok(())
    }
}

impl Default for UiPreferences {
    /// Construct the prototype defaults without reading or writing external state.
    fn default() -> Self {
        Self {
            interval: CheckInterval::Automatic,
            last_custom_minutes: CustomMinutes::default(),
            notifications_enabled: true,
            tray_enabled: false,
            sign_in_enabled: false,
        }
    }
}
