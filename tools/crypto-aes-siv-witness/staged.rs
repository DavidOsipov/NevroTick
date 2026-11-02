// SPDX-License-Identifier: GPL-2.0-only
//! Standalone algorithm-15 decryption into private, scoped scratch storage.
//!
//! This prototype uses a supplied key, not a qualified NTS association. Successful
//! AEAD and inner framing grant no UID, endpoint, replay, cookie, sample or clock
//! authority. The caller retains responsibility for the key and any copies made
//! by its consumer. Product adoption remains held by architecture section 18.7.3.

use aes_siv::{KeyInit, Tag, siv::Aes128Siv};
use ntp_wire::{NtsAuthenticatorParts, NtsPlaintextError, NtsPlaintextLimits, RawNtsPlaintext};
use zeroize::Zeroize;

/// Fixed failures containing no key, plaintext, nonce, tag or packet material.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StagedDecryptError {
    /// The declared ciphertext does not contain its complete 16-byte prefix tag.
    ShortTag,
    /// The encrypted body exceeds the fixed scratch extent.
    ///
    /// Current outer framing caps the whole packet at 4096 bytes, so its checked
    /// envelope cannot reach this defensive limit with the current public API.
    Capacity,
    /// The supplied key, associated data, nonce, tag or ciphertext did not verify.
    Authentication,
    /// Verified bytes were not a complete bounded inner-field sequence.
    Plaintext(NtsPlaintextError),
}

/// Exclusive scratch custody, established before any fallible adapter operation.
///
/// The original whole array stays borrowed until this guard drops. Only a checked
/// prefix is exposed internally; every exit wipes the entire original array.
struct ScratchWipe<'a> {
    /// Caller storage under exclusive custody, including its unused suffix.
    bytes: &'a mut [u8; 4096],
}

impl ScratchWipe<'_> {
    /// Borrow exactly the requested prefix without exposing the remaining bytes.
    ///
    /// # Errors
    /// Returns the redacted capacity error before copying when the span cannot
    /// fit. The guard continues to own and wipe the entire array on this error.
    fn prefix(&mut self, length: usize) -> Result<&mut [u8], StagedDecryptError> {
        self.bytes
            .get_mut(..length)
            .ok_or(StagedDecryptError::Capacity)
    }
}

impl Drop for ScratchWipe<'_> {
    /// Zeroize the entire borrowed array on return or ordinary Rust unwinding.
    ///
    /// This does not promise cleanup after abort/process termination, erasure of
    /// caller-owned keys, library internals or copies made by the consumer.
    fn drop(&mut self) {
        self.bytes.zeroize();
    }
}

/// Authenticate, frame, and synchronously consume one private plaintext candidate.
///
/// `parts` must come from the actual bounded outer-envelope reader. `key` is the
/// exact 32-byte experimental algorithm-15 key. The ciphertext consists of a
/// 16-byte prefix SIV tag followed by its encrypted body. Exactly two ordered
/// associated-data components are supplied: the original contiguous outer AAD,
/// then the exact declared nonce. No algorithm-specific nonce restriction or
/// association/key-provenance claim is added here.
///
/// An exclusive guard is created before all fallible checks. Only the encrypted
/// body is copied into scratch, and decryption uses that same buffer in place.
/// The inner framer and consumer run only after successful authentication. The
/// consumer receives one completely framed immutable sequence; its return type
/// is independent of the temporary plaintext lifetime. Its owned return value
/// is passed through without interpretation, including any consumer-defined error.
///
/// All 4096 scratch bytes are zeroized before returning either success or error,
/// including bytes beyond the used prefix. Ordinary consumer unwinding also
/// drops the guard. No allocation, I/O, logging, network or Windows operation is
/// performed by this adapter; arbitrary consumer effects remain the caller's
/// responsibility. No claim covers abort/process death or all prior secret copies.
///
/// A legal consumer returns metadata without retaining a stage borrow:
///
/// ```
/// use nevrotick_s115_aes_siv_fixture::staged::{with_decrypted_frame, StagedDecryptError};
/// use ntp_wire::{NtsAuthenticatorParts, NtsPlaintextLimits};
/// pub fn field_count(parts: &NtsAuthenticatorParts<'_>, key: &[u8; 32],
///     scratch: &mut [u8; 4096], limits: NtsPlaintextLimits)
///     -> Result<usize, StagedDecryptError>
/// {
///     with_decrypted_frame(parts, key, scratch, limits, |frame| frame.field_count())
/// }
/// ```
///
/// A consumer cannot return a borrow into the temporary plaintext:
///
/// ```compile_fail
/// use nevrotick_s115_aes_siv_fixture::staged::{with_decrypted_frame, StagedDecryptError};
/// use ntp_wire::{NtsAuthenticatorParts, NtsPlaintextLimits};
/// pub fn escape(parts: &NtsAuthenticatorParts<'_>, key: &[u8; 32],
///     scratch: &mut [u8; 4096], limits: NtsPlaintextLimits)
///     -> Result<&'static [u8], StagedDecryptError>
/// {
///     with_decrypted_frame(parts, key, scratch, limits, |frame| {
///         frame.fields().next().map_or(&[][..], |field| field.body())
///     })
/// }
/// ```
///
/// The consumer cannot alias the exclusively borrowed caller storage:
///
/// ```compile_fail,E0502
/// use nevrotick_s115_aes_siv_fixture::staged::with_decrypted_frame;
/// use ntp_wire::{NtsAuthenticatorParts, NtsPlaintextLimits};
/// pub fn alias(parts: &NtsAuthenticatorParts<'_>, key: &[u8; 32],
///     scratch: &mut [u8; 4096], limits: NtsPlaintextLimits)
/// {
///     let _ = with_decrypted_frame(parts, key, scratch, limits, |_| scratch[0]);
/// }
/// ```
///
/// # Errors
/// Returns a short-tag or capacity error before decryption, an authentication
/// error before framing/consumption, or the redacted inner-framing error before
/// consumption. Every such result wipes the full scratch array.
pub fn with_decrypted_frame<R, F>(
    parts: &NtsAuthenticatorParts<'_>,
    key: &[u8; 32],
    scratch: &mut [u8; 4096],
    limits: NtsPlaintextLimits,
    consume: F,
) -> Result<R, StagedDecryptError>
where
    F: for<'p> FnOnce(RawNtsPlaintext<'p>) -> R,
{
    let mut staging = ScratchWipe { bytes: scratch };
    let (tag, ciphertext) = parts
        .ciphertext()
        .split_first_chunk::<16>()
        .ok_or(StagedDecryptError::ShortTag)?;
    let body = staging.prefix(ciphertext.len())?;
    body.copy_from_slice(ciphertext);
    let mut cipher = Aes128Siv::new(key.into());
    let tag: &Tag = tag.into();
    cipher
        .decrypt_inout_detached(
            [parts.associated_data(), parts.nonce()],
            (&mut *body).into(),
            tag,
        )
        .map_err(|_| StagedDecryptError::Authentication)?;
    let frame =
        ntp_wire::frame_nts_plaintext(body, limits).map_err(StagedDecryptError::Plaintext)?;
    Ok(consume(frame))
}
