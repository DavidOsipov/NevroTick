// SPDX-License-Identifier: GPL-3.0-only
//! Bounded native keyboard and IME ingress before Slint retains event text.
//!
//! Keyboard chunks keep all toolkit key meanings under an original UTF-8 byte
//! cap. IME text is printable ASCII, including empty/spaces; both optional byte
//! cursor endpoints must fit that original text, without requiring their order.
//! This boundary grants no field/effect authority and cannot undo earlier
//! OS/Winit allocations. The Windows hook borrows events without retaining them.
#![forbid(unsafe_code)]

/// Window-wide byte ceiling, equal to the largest maintained raw editor limit.
const MAX_NATIVE_INPUT_BYTES: usize = 1024;

/// Bound original keyboard bytes without changing Unicode/control/key semantics.
fn keyboard_chunk_allowed(text: &str) -> bool {
    text.len() <= MAX_NATIVE_INPUT_BYTES
}

/// Admit printable ASCII IME bytes and two inclusive, possibly reversed offsets.
/// ASCII plus the byte cap also makes every accepted offset an i32-safe boundary.
fn ime_preedit_allowed(text: &str, cursor: Option<(usize, usize)>) -> bool {
    ime_commit_allowed(text)
        && cursor.is_none_or(|(start, end)| start <= text.len() && end <= text.len())
}

/// Bound before scanning the original IME representation; never normalize it.
fn ime_commit_allowed(text: &str) -> bool {
    keyboard_chunk_allowed(text) && text.bytes().all(|byte| (b' '..=b'~').contains(&byte))
}

/// Install the sole stateless filter after explicit Winit selection and before
/// showing the window or installing native observers. The public setter returns
/// no receipt and is a no-op on other backends; `create_native` selects Winit first.
#[cfg(windows)]
pub(super) fn install(window: &slint::Window) {
    use slint::winit_030::WinitWindowAccessor;
    window.on_winit_window_event(filter_window_event);
}

/// Borrow one native event; rejected text reaches no downstream Slint handler.
/// Accepted/non-text lifecycle, focus, Close and IME cancel events propagate.
#[cfg(windows)]
fn filter_window_event(
    _window: &slint::Window,
    event: &slint::winit_030::winit::event::WindowEvent,
) -> slint::winit_030::EventResult {
    use slint::winit_030::{
        EventResult,
        winit::event::{Ime, WindowEvent},
    };
    let allowed = match event {
        WindowEvent::KeyboardInput { event, .. } => keyboard_event_allowed(event),
        WindowEvent::Ime(Ime::Preedit(text, cursor)) => ime_preedit_allowed(text, *cursor),
        WindowEvent::Ime(Ime::Commit(text)) => ime_commit_allowed(text),
        _ => true,
    };
    if allowed {
        EventResult::Propagate
    } else {
        EventResult::PreventDefault
    }
}

/// Bound the borrowed text/logical Character before requesting the public
/// modifier-supplement value, then immediately bound that derived Character.
/// Winit returns that Key by clone; this guard cannot undo its prior allocation.
#[cfg(windows)]
fn keyboard_event_allowed(event: &slint::winit_030::winit::event::KeyEvent) -> bool {
    use slint::winit_030::winit::{
        keyboard::Key, platform::modifier_supplement::KeyEventExtModifierSupplement,
    };
    if event
        .text
        .as_ref()
        .is_some_and(|text| !keyboard_chunk_allowed(text.as_str()))
    {
        return false;
    }
    if let Key::Character(text) = &event.logical_key
        && !keyboard_chunk_allowed(text.as_str())
    {
        return false;
    }
    !matches!(event.key_without_modifiers(), Key::Character(text) if !keyboard_chunk_allowed(text.as_str()))
}

#[cfg(test)]
mod tests {
    //! Literal behavioral fixtures; expected decisions do not call policy helpers.

    use super::{ime_commit_allowed, ime_preedit_allowed, keyboard_chunk_allowed};

    /// Catch a missing or off-by-one keyboard byte cap.
    ///
    /// # Panics
    /// Fails if the exact boundary is rejected or the next byte is forwarded.
    #[test]
    fn keyboard_accepts_1024_original_bytes_and_rejects_1025() {
        assert!(keyboard_chunk_allowed(""));
        assert!(keyboard_chunk_allowed(&"k".repeat(1024)));
        assert!(!keyboard_chunk_allowed(&"k".repeat(1025)));
    }

    /// Catch scalar-count limits or an accidental keyboard alphabet restriction.
    ///
    /// # Panics
    /// Fails if original UTF-8 bytes are miscounted or bounded Unicode keys are denied.
    #[test]
    fn keyboard_limits_utf8_bytes_without_rejecting_bounded_unicode() {
        assert!(keyboard_chunk_allowed(&"é".repeat(512)));
        assert!(!keyboard_chunk_allowed(&"é".repeat(513)));
        assert!(keyboard_chunk_allowed(&"🙂".repeat(256)));
        assert!(!keyboard_chunk_allowed(&"🙂".repeat(257)));
    }

    /// Catch a blanket keyboard ASCII/control filter that breaks toolkit key meanings.
    ///
    /// # Panics
    /// Fails if a bounded control/private-key encoding is filtered as IME text.
    #[test]
    fn keyboard_preserves_control_and_private_key_chunks() {
        for key in [
            "\0\u{1}\u{8}\t\n\r\u{1b}\u{7f}",
            "\u{e000}",
            "\u{f700}\u{f8ff}",
            " ",
        ] {
            assert!(keyboard_chunk_allowed(key));
        }
    }

    /// Catch a too-narrow IME alphabet while preserving exact printable punctuation.
    ///
    /// # Panics
    /// Fails if printable ASCII text cannot reach preedit or commit.
    #[test]
    fn ime_accepts_printable_ascii_without_normalizing_spaces() {
        let text = "  !\"#$%&'()*+,-./0123456789:;<=>?@ABCDEFGHIJKLMNOPQRSTUVWXYZ[\\]^_`abcdefghijklmnopqrstuvwxyz{|}~  ";
        assert!(ime_preedit_allowed(text, None));
        assert!(ime_commit_allowed(text));
    }

    /// Catch an empty/space ban that prevents composition cancellation or spaces.
    ///
    /// # Panics
    /// Fails if a valid empty cancellation or ASCII-space composition is denied.
    #[test]
    fn ime_empty_and_space_cancel_events_remain_valid() {
        assert!(ime_preedit_allowed("", None));
        assert!(ime_preedit_allowed("", Some((0, 0))));
        assert!(ime_commit_allowed(""));
        assert!(ime_preedit_allowed("   ", Some((3, 0))));
        assert!(ime_commit_allowed("   "));
    }

    /// Catch an off-by-one cap or rejection of inclusive cursor endpoints.
    ///
    /// # Panics
    /// Fails if exact1024-byte IME text or its legal boundary cursor is denied.
    #[test]
    fn ime_accepts_exact_1024_bytes_and_cursor_boundaries() {
        let text = "a".repeat(1024);
        assert!(ime_preedit_allowed(&text, None));
        assert!(ime_preedit_allowed(&text, Some((0, 1024))));
        assert!(ime_preedit_allowed(&text, Some((1024, 0))));
        assert!(ime_preedit_allowed(&text, Some((1024, 1024))));
        assert!(ime_commit_allowed(&text));
        assert!(ime_preedit_allowed(&" ".repeat(1024), Some((1024, 0))));
    }

    /// Catch missing preedit/commit size guards independently of cursor validity.
    ///
    /// # Panics
    /// Fails if either IME event forwards the next byte beyond the cap.
    #[test]
    fn ime_rejects_1025_original_bytes_for_preedit_and_commit() {
        let text = "a".repeat(1025);
        assert!(!ime_preedit_allowed(&text, None));
        assert!(!ime_preedit_allowed(&text, Some((0, 0))));
        assert!(!ime_commit_allowed(&text));
    }

    /// Catch NUL, C0 control or DEL admission in a short otherwise-printable string.
    ///
    /// # Panics
    /// Fails if either IME event admits a control regardless of its small byte length.
    #[test]
    fn ime_rejects_nul_every_c0_control_and_del() {
        for byte in (0_u8..=31).chain(std::iter::once(127)) {
            let text = format!("a{}b", char::from(byte));
            assert!(!ime_preedit_allowed(&text, None));
            assert!(!ime_commit_allowed(&text));
        }
    }

    /// Catch non-ASCII, emoji, bidi, zero-width, combining or private-use admission.
    ///
    /// # Panics
    /// Fails if either IME event accepts a non-ASCII sequence below its byte cap.
    #[test]
    fn ime_rejects_non_ascii_emoji_and_invisible_unicode() {
        for payload in [
            "é",
            "я",
            "中",
            "e\u{301}",
            "\u{a0}",
            "\u{200b}",
            "\u{202e}",
            "🙂",
            "👩‍💻",
            "\u{e000}",
            "\u{10ffff}",
        ] {
            let text = format!("a{payload}b");
            assert!(!ime_preedit_allowed(&text, None));
            assert!(!ime_commit_allowed(&text));
        }
    }

    /// Catch omitted, endpoint-only or ordered-selection cursor assumptions.
    ///
    /// # Panics
    /// Fails if absent, inclusive or reversed valid byte cursor pairs are denied.
    #[test]
    fn preedit_accepts_none_and_both_inclusive_reversed_byte_offsets() {
        assert!(ime_preedit_allowed("abcd", None));
        for cursor in [(0, 0), (0, 4), (4, 4), (4, 0), (3, 1)] {
            assert!(ime_preedit_allowed("abcd", Some(cursor)));
        }
    }

    /// Catch validation of only one cursor endpoint or truncation of `usize::MAX`.
    ///
    /// # Panics
    /// Fails if either original cursor exceeds the text byte length.
    #[test]
    fn preedit_rejects_either_out_of_range_cursor_and_usize_max() {
        for cursor in [
            (0, 5),
            (5, 0),
            (5, 5),
            (usize::MAX, 0),
            (0, usize::MAX),
            (usize::MAX, usize::MAX),
        ] {
            assert!(!ime_preedit_allowed("abcd", Some(cursor)));
        }
    }

    /// Catch an early empty-text success that skips cursor validation.
    ///
    /// # Panics
    /// Fails if an empty cancellation carries a nonzero or unrepresentable cursor.
    #[test]
    fn empty_preedit_still_rejects_nonzero_cursor_offsets() {
        for cursor in [(0, 1), (1, 0), (1, 1), (usize::MAX, 0), (0, usize::MAX)] {
            assert!(!ime_preedit_allowed("", Some(cursor)));
        }
    }
}
