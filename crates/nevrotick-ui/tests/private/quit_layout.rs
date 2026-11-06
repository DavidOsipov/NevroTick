//! Bounded plain-text renderer fixture, not a product setting or catalog grant.
//!
//! Both strings are printable ASCII within the current catalog's byte/scalar
//! bounds. Direct presentation setters do not admit a catalog, approve meaning,
//! change private quit ownership or submit a service/clock operation.
#![forbid(unsafe_code)]

use nevrotick_ui::localization::{STRING_BYTES, STRING_SCALARS};
use nevrotick_ui::slint_host::AppWindow;

/// Exercise overflow with known bounded text after the real warning is opened.
///
/// The actual maintained English warning is preserved as a separate healthy
/// fixture. These repeated sentences are layout stress, not reviewed release copy.
///
/// # Panics
/// Fails if a source change makes either technical fixture non-ASCII or oversized.
pub(crate) fn expand(window: &AppWindow) {
    let headline = "Unconfirmed background service state. ".repeat(13);
    let consequence = "This preview cannot confirm the service state. ".repeat(10);
    for text in [&headline, &consequence] {
        assert!(text.is_ascii());
        assert!(text.len() <= STRING_BYTES && text.len() <= STRING_SCALARS);
    }
    window.set_fixture_quit_headline(headline.into());
    window.set_fixture_quit_consequence(consequence.into());
}
