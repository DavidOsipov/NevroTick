//! Bounded fixture presentation and atomic in-memory source-preview edits.
//!
//! Field limits belong to UI-M0 only. No value here grants network, policy or
//! clock authority. Construction validates before allocating accepted strings.
#![forbid(unsafe_code)]

use std::{fmt, net::IpAddr};

use garde::Validate;

/// Maximum accepted label bytes; labels use printable ASCII in this prototype.
pub const MAX_LABEL_BYTES: usize = 64;
/// Maximum accepted DNS/IP identity bytes, without a URI or transport suffix.
pub const MAX_HOST_BYTES: usize = 253;
/// Maximum raw port bytes; only unsigned decimal digits are admitted.
pub const MAX_PORT_BYTES: usize = 5;
/// Persistent context for every UI page and editor.
pub const SAMPLE_LABEL: &str = "Prototype — sample data";

/// A bounded, redacted preview error that contains no rejected input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ModelError {
    /// Label is absent, too long, non-ASCII, contains controls or edge spaces.
    InvalidLabel,
    /// Host is not a bounded ASCII DNS name or IP literal.
    InvalidHost,
    /// Port is not an unsigned decimal number from 1 through 65535.
    InvalidPort,
    /// Requested row is outside the fixed fixture inventory.
    InvalidSelection,
}

impl fmt::Display for ModelError {
    /// Emit only a fixed product message; rejected bytes are never formatted.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::InvalidLabel => {
                "Use a label of 1–64 printable ASCII characters, without edge spaces."
            }
            Self::InvalidHost => {
                "Use an ASCII DNS name or IP address, without a URL or port suffix."
            }
            Self::InvalidPort => "Use a decimal port from 1 to 65535, without spaces or a sign.",
            Self::InvalidSelection => "Select one of the sample sources.",
        })
    }
}

impl std::error::Error for ModelError {}

/// Borrowed raw fields; every field has one mandatory garde rule owner.
#[derive(Validate)]
struct PreviewInput<'a> {
    /// Display name, bounded before validation/report allocation.
    #[garde(custom(label_rule))]
    label: &'a str,
    /// ASCII identity, used only as sample text.
    #[garde(custom(host_rule))]
    host: &'a str,
    /// Raw unsigned decimal port text.
    #[garde(custom(port_rule))]
    port: &'a str,
}

/// Check the prototype label rule without allocating or normalizing input.
///
/// # Errors
/// Rejects absent, oversized, non-printable labels and edge spaces.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde requires custom rules to borrow their context"
)]
fn label_rule(value: &str, (): &()) -> garde::Result {
    if !value.is_empty()
        && value.len() <= MAX_LABEL_BYTES
        && value.bytes().all(|byte| (b' '..=b'~').contains(&byte))
        && !value.starts_with(' ')
        && !value.ends_with(' ')
    {
        Ok(())
    } else {
        Err(garde::Error::new("invalid label"))
    }
}

/// Check bounded ASCII DNS/IP syntax; this function performs no DNS resolution.
///
/// # Errors
/// Rejects oversized identities, non-ASCII text and invalid DNS/IP syntax.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde requires custom rules to borrow their context"
)]
fn host_rule(value: &str, (): &()) -> garde::Result {
    let valid = !value.is_empty()
        && value.len() <= MAX_HOST_BYTES
        && value.is_ascii()
        && (value.parse::<IpAddr>().is_ok()
            || value.split('.').all(|label| {
                !label.is_empty()
                    && label.len() <= 63
                    && !label.starts_with('-')
                    && !label.ends_with('-')
                    && label
                        .bytes()
                        .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
            }));
    if valid {
        Ok(())
    } else {
        Err(garde::Error::new("invalid host"))
    }
}

/// Admit a raw port's lexical form and nonzero u16 range without allocation.
///
/// # Errors
/// Rejects absent/non-decimal ports and values outside the nonzero u16 range.
#[expect(
    clippy::trivially_copy_pass_by_ref,
    reason = "garde requires custom rules to borrow their context"
)]
fn port_rule(value: &str, (): &()) -> garde::Result {
    if !value.is_empty()
        && value.len() <= MAX_PORT_BYTES
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && value.parse::<u16>().is_ok_and(|port| port != 0)
    {
        Ok(())
    } else {
        Err(garde::Error::new("invalid port"))
    }
}

/// Checked immutable source-preview fields, without live connection state.
///
/// Callers cannot construct or mutate raw fields:
/// ```compile_fail,E0451
/// use nevrotick_ui::model::SourcePreview;
/// let source = SourcePreview { label: String::new(), host: String::new(), port: 0 };
/// ```
/// A legal checked construction remains available:
/// ```
/// use nevrotick_ui::model::SourcePreview;
/// let source = SourcePreview::parse("Sample", "clock.example.invalid", "4460")?;
/// assert_eq!(source.port(), 4460);
/// # Ok::<(), nevrotick_ui::model::ModelError>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourcePreview {
    /// Owned checked display label; no mutable access is exposed.
    label: String,
    /// Owned checked ASCII identity; no resolution is performed.
    host: String,
    /// Checked nonzero NTS-KE preview port, distinct from an NTP endpoint.
    port: u16,
}

impl SourcePreview {
    /// Validate all raw fields through garde, then allocate the accepted value.
    ///
    /// # Errors
    /// Rejects oversized or invalid labels, hosts and ports without any effect.
    pub fn parse(label: &str, host: &str, port: &str) -> Result<Self, ModelError> {
        // Bound work before garde can allocate its bounded three-field report.
        if label.len() > MAX_LABEL_BYTES {
            return Err(ModelError::InvalidLabel);
        }
        if host.len() > MAX_HOST_BYTES {
            return Err(ModelError::InvalidHost);
        }
        if port.len() > MAX_PORT_BYTES {
            return Err(ModelError::InvalidPort);
        }
        let input = PreviewInput { label, host, port };
        input.validate().map_err(|report| {
            // Only schema-owned field names are inspected; reports/rejected
            // values are never sent to the UI or diagnostics.
            let has_field = |field: &str| report.iter().any(|(path, _)| path.to_string() == field);
            if has_field("label") {
                ModelError::InvalidLabel
            } else if has_field("host") {
                ModelError::InvalidHost
            } else {
                ModelError::InvalidPort
            }
        })?;
        let port = port.parse::<u16>().map_err(|_| ModelError::InvalidPort)?;
        Ok(Self {
            label: label.to_owned(),
            host: host.to_owned(),
            port,
        })
    }

    /// Borrow the checked label without exposing a mutation path.
    #[must_use]
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Borrow the checked sample identity without resolving it.
    #[must_use]
    pub fn host(&self) -> &str {
        &self.host
    }

    /// Return the nonzero NTS-KE preview port; no endpoint is contacted.
    #[must_use]
    pub const fn port(&self) -> u16 {
        self.port
    }
}

/// Three deterministic source previews and a checked current selection.
///
/// Recreating the model resets every value; there is no persistence or clock.
/// Raw inputs cannot replace the checked inventory:
/// ```compile_fail,E0616
/// let mut model = nevrotick_ui::model::PreviewModel::fixture()?;
/// model.selected = usize::MAX;
/// # Ok::<(), nevrotick_ui::model::ModelError>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewModel {
    /// Fixed nonempty inventory bounds aggregate retained preview data.
    sources: [SourcePreview; 3],
    /// Maintained strictly below `sources.len()` by `select`.
    selected: usize,
}

impl PreviewModel {
    /// Construct the sample inventory through the same checked path as edits.
    ///
    /// # Errors
    /// Returns a validation error if a maintained fixture violates the contract.
    pub fn fixture() -> Result<Self, ModelError> {
        Ok(Self {
            sources: [
                SourcePreview::parse("Sample North", "north.example.invalid", "4460")?,
                SourcePreview::parse("Sample South", "south.example.invalid", "4460")?,
                SourcePreview::parse("Sample West", "west.example.invalid", "4460")?,
            ],
            selected: 0,
        })
    }

    /// Borrow the fixed checked inventory without exposing mutable values.
    #[must_use]
    pub fn sources(&self) -> &[SourcePreview] {
        &self.sources
    }

    /// Return the current index, which is always inside the fixed inventory.
    #[must_use]
    pub const fn selected_index(&self) -> usize {
        self.selected
    }

    /// Borrow the selected source; construction and select preserve the index.
    #[must_use]
    pub fn selected(&self) -> &SourcePreview {
        &self.sources[self.selected]
    }

    /// Select an existing source, preserving the previous selection on error.
    ///
    /// # Errors
    /// Returns `InvalidSelection` for an index outside the fixed inventory.
    pub fn select(&mut self, index: usize) -> Result<(), ModelError> {
        if index >= self.sources.len() {
            return Err(ModelError::InvalidSelection);
        }
        self.selected = index;
        Ok(())
    }

    /// Atomically replace only the selected preview after mandatory validation.
    ///
    /// Unsubmitted UI-control text remains outside this state; Cancel needs only
    /// to redisplay `selected()`. This method has no effects beyond this instance.
    ///
    /// # Errors
    /// Invalid raw fields leave every source and the selection unchanged.
    pub fn update_selected(
        &mut self,
        label: &str,
        host: &str,
        port: &str,
    ) -> Result<(), ModelError> {
        let replacement = SourcePreview::parse(label, host, port)?;
        self.sources[self.selected] = replacement;
        Ok(())
    }
}
