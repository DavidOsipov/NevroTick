// SPDX-License-Identifier: GPL-2.0-only

//! Inert, allocation-free framing of one complete NTS-KE byte buffer.
//!
//! RFC 8915 section 4 uses a four-byte prefix, a separate critical bit, a
//! fifteen-bit type and a body-only length. Bodies need not be word aligned.
//! This layer checks complete spans and a sole final critical empty EOM. Other
//! record types and bodies remain raw, including unknown critical records.
//! A later semantic receiver must reject unsupported critical types and validate
//! request/response shapes before accepting keys, cookies or an association.
//! No transport, TLS closure, EOF observation, authentication or effects occur.

/// Experimental framing byte bound, not a shipped response-policy default.
const MAX_MESSAGE_BYTES: usize = 131_072;
/// A four-byte minimum record gives the independent absolute work bound.
const MAX_RECORDS: usize = MAX_MESSAGE_BYTES / 4;

/// Closed, payload-free structural rejection; no cookie or input bytes escape.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NtsKeFrameError {
    /// A supplied byte/count ceiling is outside the experimental range.
    InvalidLimits,
    /// The complete supplied buffer exceeds its admitted byte ceiling.
    ByteLimit,
    /// Another record is present after the admitted count is exhausted.
    TooManyRecords,
    /// A complete-buffer boundary occurs inside a four-byte record prefix.
    TruncatedHeader,
    /// A complete prefix declares more body bytes than remain in this buffer.
    TruncatedBody,
    /// Complete ordinary records end without a final EOM record.
    MissingEndOfMessage,
    /// Type zero lacks its critical bit or has a nonempty body.
    InvalidEndOfMessage,
    /// Any bytes follow the first EOM, including a second EOM or partial prefix.
    TrailingData,
}

/// Explicit private framing ceilings; no default or mutable bypass is exposed.
///
/// Bytes range from 4 through 131,072 and records from 1 through 32,768. These
/// experiment ceilings permit the canonical 65,536-byte acceptance target; they
/// do not adopt TLS, transport, concurrency or final production budget policy.
/// Records include the final EOM. No alignment condition applies to either cap.
///
/// ```compile_fail,E0451
/// use ntp_wire::NtsKeFrameLimits;
/// let _ = NtsKeFrameLimits { max_message_bytes: 0, max_records: 0 };
/// ```
/// ```compile_fail,E0277
/// use ntp_wire::NtsKeFrameLimits;
/// let _: NtsKeFrameLimits = Default::default();
/// ```
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct NtsKeFrameLimits {
    /// Checked whole-message byte budget, including all prefixes and EOM.
    max_message_bytes: usize,
    /// Checked complete-record budget, including EOM.
    max_records: usize,
}

impl NtsKeFrameLimits {
    /// Admit explicit experiment bounds before any parser reads input bytes.
    ///
    /// # Errors
    /// Reject byte budgets outside `4..=131_072` or record budgets outside
    /// `1..=32_768`. Rejection creates no usable limits and has no effects.
    pub fn new(max_message_bytes: usize, max_records: usize) -> Result<Self, NtsKeFrameError> {
        if !(4..=MAX_MESSAGE_BYTES).contains(&max_message_bytes)
            || !(1..=MAX_RECORDS).contains(&max_records)
        {
            return Err(NtsKeFrameError::InvalidLimits);
        }
        Ok(Self {
            max_message_bytes,
            max_records,
        })
    }

    /// Check computed encoder dimensions against these immutable ceilings.
    ///
    /// This checks only budgets, not framing or semantic message validity.
    /// It reads no message bytes and changes no state.
    ///
    /// # Errors
    /// Reject a byte excess before a record-count excess, as in framing.
    pub(crate) fn check_message_budget(
        self,
        byte_count: usize,
        record_count: usize,
    ) -> Result<(), NtsKeFrameError> {
        if byte_count > self.max_message_bytes {
            return Err(NtsKeFrameError::ByteLimit);
        }
        if record_count > self.max_records {
            return Err(NtsKeFrameError::TooManyRecords);
        }
        Ok(())
    }
}

/// Closed accumulator admission failure; no payload or partial message escapes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NtsKeAccumulatorError {
    /// Caller storage cannot hold the entire declared byte ceiling.
    StorageTooSmall,
    /// The existing framing owner's bounded rejection, including truncation.
    Frame(
        /// Payload-free cause; a push failure remains latched through finish.
        NtsKeFrameError,
    ),
}

/// Framing progress only; neither state observes EOF or authenticates a peer.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NtsKeProgress {
    /// More bytes are required; an empty push is not a transport EOF.
    NeedMore,
    /// A complete critical empty EOM ends the accepted prefix, not TLS closure.
    EndOfMessage,
}

/// Copyable staged cursor over already admitted bytes, without payload storage.
#[derive(Clone, Copy)]
struct AccumulatorCursor {
    /// One prefix, retained unchanged until its declared body is complete.
    header: [u8; 4],
    /// Zero through three prefix bytes, or four while consuming its body.
    header_used: usize,
    /// Remaining declared body bytes; bounded by the two-byte wire length.
    body_left: usize,
    /// Started records, including an incomplete current prefix/body and EOM.
    records: usize,
    /// A complete valid EOM was consumed; subsequent nonempty input must reject.
    ended: bool,
}

impl AccumulatorCursor {
    /// Construct the empty cursor without reading or changing caller storage.
    fn new() -> Self {
        Self {
            header: [0; 4],
            header_used: 0,
            body_left: 0,
            records: 0,
            ended: false,
        }
    }

    /// Return current progress without treating an empty chunk as EOF.
    fn progress(&self) -> NtsKeProgress {
        if self.ended {
            NtsKeProgress::EndOfMessage
        } else {
            NtsKeProgress::NeedMore
        }
    }

    /// Complete one fully present prefix/body, then permit the next prefix.
    ///
    /// The staged caller has proved four header bytes and zero remaining body.
    /// # Errors
    /// Reject type zero unless critical with an empty declared body. Waiting for
    /// the whole body preserves the existing complete framer's split precedence.
    fn complete_record(&mut self) -> Result<(), NtsKeFrameError> {
        let kind = u16::from_be_bytes([self.header[0] & 0x7f, self.header[1]]);
        if kind == 0 {
            if self.header[0] & 0x80 == 0 || self.header[2] != 0 || self.header[3] != 0 {
                return Err(NtsKeFrameError::InvalidEndOfMessage);
            }
            self.ended = true;
        }
        self.header_used = 0;
        Ok(())
    }

    /// Advance a private cursor copy once through a byte-admitted chunk.
    ///
    /// Each byte is consumed once; each prefix length is decoded once. A record
    /// starts only below the validated record ceiling. Errors affect this copy
    /// only; the owner discards it and latches the cause before copying bytes.
    /// # Errors
    /// Preserve record-budget, EOM-shape and trailing-data framing failures.
    fn consume(
        &mut self,
        chunk: &[u8],
        max_records: usize,
    ) -> Result<NtsKeProgress, NtsKeFrameError> {
        for &byte in chunk {
            if self.ended {
                return Err(NtsKeFrameError::TrailingData);
            }
            if self.body_left != 0 {
                self.body_left -= 1;
                if self.body_left == 0 {
                    self.complete_record()?;
                }
                continue;
            }
            if self.header_used == 0 {
                if self.records == max_records {
                    return Err(NtsKeFrameError::TooManyRecords);
                }
                self.records += 1;
            }
            self.header[self.header_used] = byte;
            self.header_used += 1;
            if self.header_used == 4 {
                self.body_left = usize::from(u16::from_be_bytes([self.header[2], self.header[3]]));
                if self.body_left == 0 {
                    self.complete_record()?;
                }
            }
        }
        Ok(self.progress())
    }
}

/// Incremental, inert framing in exclusively borrowed caller-owned storage.
///
/// Only `new` constructs this owner; it has no reset, default, payload debug or
/// mutable-storage route. Accepted chunks append exactly their original bytes.
/// A rejected chunk changes no storage byte or accepted cursor, then latches the
/// first error; later pushes and finish return that error. Previously accepted
/// bytes remain in storage, but no partial public message is returned.
///
/// Fixed cursor storage and work are O(bytes + records + calls), with no heap,
/// ambient I/O/clock, TLS-record assumption, EOF observation or external effect.
/// `EndOfMessage` proves neither transport closure, semantic negotiation, provider
/// support, peer identity, authentication nor a usable association.
///
/// Checked construction cannot be bypassed with a default or raw-state mutation:
/// ```compile_fail,E0277
/// use ntp_wire::NtsKeAccumulator;
/// let _: NtsKeAccumulator<'static> = Default::default();
/// ```
/// ```compile_fail,E0616
/// use ntp_wire::NtsKeAccumulator;
/// fn bypass(mut value: NtsKeAccumulator<'_>) { value.used = 0; }
/// ```
/// Payload-bearing state has no debug formatter:
/// ```compile_fail,E0277
/// use ntp_wire::NtsKeAccumulator;
/// fn needs_debug<T: core::fmt::Debug>() {}
/// needs_debug::<NtsKeAccumulator<'static>>();
/// ```
pub struct NtsKeAccumulator<'buf> {
    /// Exclusively borrowed storage, at least the declared byte ceiling long.
    storage: &'buf mut [u8],
    /// Existing checked byte and record ceilings, never widened by a chunk.
    limits: NtsKeFrameLimits,
    /// Accepted prefix length, always at or below the declared byte ceiling.
    used: usize,
    /// State describing exactly the accepted original prefix.
    cursor: AccumulatorCursor,
    /// First push rejection; no retry or later chunk may clear it.
    failure: Option<NtsKeFrameError>,
}

impl<'buf> NtsKeAccumulator<'buf> {
    /// Borrow storage for one message under explicit existing framing limits.
    ///
    /// No storage is read or written at construction. Chunks may split any prefix
    /// or body and may coalesce records; transport/TLS boundaries are irrelevant.
    /// # Errors
    /// Reject storage shorter than the declared byte cap, without changing it.
    ///
    /// ```
    /// use ntp_wire::{NtsKeAccumulator, NtsKeFrameLimits, NtsKeProgress};
    /// let mut storage = [0xa5; 8];
    /// let limits = NtsKeFrameLimits::new(8, 2).unwrap();
    /// let mut accumulator = NtsKeAccumulator::new(&mut storage, limits).unwrap();
    /// assert_eq!(accumulator.push(&[]).unwrap(), NtsKeProgress::NeedMore);
    /// assert_eq!(accumulator.push(&[0x80, 0]).unwrap(), NtsKeProgress::NeedMore);
    /// assert_eq!(accumulator.push(&[0, 0]).unwrap(), NtsKeProgress::EndOfMessage);
    /// let message = accumulator.finish().unwrap();
    /// assert_eq!(message.bytes(), [0x80, 0, 0, 0]);
    /// assert_eq!(message.record_count(), 1);
    /// ```
    /// Caller mutation cannot bypass the owner's exclusive storage borrow:
    /// ```compile_fail,E0506
    /// use ntp_wire::{NtsKeAccumulator, NtsKeFrameLimits};
    /// let mut storage = [0; 4];
    /// let limits = NtsKeFrameLimits::new(4, 1).unwrap();
    /// let mut accumulator = NtsKeAccumulator::new(&mut storage, limits).unwrap();
    /// storage[0] = 0xff;
    /// let _ = accumulator.push(&[0x80, 0, 0, 0]);
    /// ```
    pub fn new(
        storage: &'buf mut [u8],
        limits: NtsKeFrameLimits,
    ) -> Result<Self, NtsKeAccumulatorError> {
        if storage.len() < limits.max_message_bytes {
            return Err(NtsKeAccumulatorError::StorageTooSmall);
        }
        Ok(Self {
            storage,
            limits,
            used: 0,
            cursor: AccumulatorCursor::new(),
            failure: None,
        })
    }

    /// Append one entire admitted chunk, or change no storage and latch failure.
    ///
    /// A copied cursor validates the whole chunk before the first write. Empty
    /// chunks report current progress and never imply EOF. Nonempty bytes after
    /// EOM reject even in later calls. No accepted prefix is reparsed here.
    /// # Errors
    /// Return an already latched error first. Otherwise whole-chunk byte excess
    /// precedes parsing, then the first observable framing error is latched.
    /// Error priority can depend on chunk boundaries: a later oversized chunk
    /// cannot replace a failure already observed in an earlier call.
    pub fn push(&mut self, chunk: &[u8]) -> Result<NtsKeProgress, NtsKeAccumulatorError> {
        if let Some(error) = self.failure {
            return Err(NtsKeAccumulatorError::Frame(error));
        }
        if chunk.len() > self.limits.max_message_bytes - self.used {
            return Err(self.latch(NtsKeFrameError::ByteLimit));
        }
        let mut staged = self.cursor;
        let progress = match staged.consume(chunk, self.limits.max_records) {
            Ok(progress) => progress,
            Err(error) => return Err(self.latch(error)),
        };
        let end = self.used + chunk.len();
        self.storage[self.used..end].copy_from_slice(chunk);
        self.used = end;
        self.cursor = staged;
        Ok(progress)
    }

    /// Consume this owner and frame the original accepted prefix exactly once.
    ///
    /// Only this boundary calls the existing complete framer, at most once. Its
    /// returned message borrows the original storage; finish observes no EOF or
    /// TLS closure. A saved push error prevents any partial result or reparse.
    /// # Errors
    /// Preserve the first push error, or the existing complete framer's truncation,
    /// missing-EOM and other framing causes for the accumulated original prefix.
    ///
    /// A finished message cannot escape its caller-owned storage:
    /// ```compile_fail,E0515
    /// use ntp_wire::{NtsKeAccumulator, NtsKeAccumulatorError, NtsKeFrameLimits, RawNtsKeMessage};
    /// fn escape() -> Result<RawNtsKeMessage<'static>, NtsKeAccumulatorError> {
    ///     let mut storage = [0; 4];
    ///     let limits = NtsKeFrameLimits::new(4, 1).unwrap();
    ///     let mut accumulator = NtsKeAccumulator::new(&mut storage, limits)?;
    ///     accumulator.push(&[0x80, 0, 0, 0])?;
    ///     accumulator.finish()
    /// }
    /// ```
    /// Finalization consumes the owner; another push requires a new owner:
    /// ```compile_fail,E0382
    /// use ntp_wire::{NtsKeAccumulator, NtsKeFrameLimits};
    /// let mut storage = [0; 4];
    /// let limits = NtsKeFrameLimits::new(4, 1).unwrap();
    /// let mut accumulator = NtsKeAccumulator::new(&mut storage, limits).unwrap();
    /// accumulator.push(&[0x80, 0, 0, 0]).unwrap();
    /// let _message = accumulator.finish().unwrap();
    /// let _ = accumulator.push(&[]);
    /// ```
    pub fn finish(self) -> Result<RawNtsKeMessage<'buf>, NtsKeAccumulatorError> {
        if let Some(error) = self.failure {
            return Err(NtsKeAccumulatorError::Frame(error));
        }
        frame_nts_ke_message(&self.storage[..self.used], self.limits)
            .map_err(NtsKeAccumulatorError::Frame)
    }

    /// Preserve the first closed framing cause without changing storage/cursor.
    fn latch(&mut self, error: NtsKeFrameError) -> NtsKeAccumulatorError {
        NtsKeAccumulatorError::Frame(*self.failure.get_or_insert(error))
    }
}

/// A completely framed immutable buffer, not a semantically valid KE response.
///
/// Only [`frame_nts_ke_message`] constructs this borrowed view after checking the
/// entire buffer. EOM-only input is structurally framed but does not thereby
/// satisfy request, successful-response or Error-response rules. No payload
/// `Debug`, mutable access or association-publication method is supplied.
///
/// The iterator and a record may outlive the message view while its input lives:
///
/// ```
/// use ntp_wire::{NtsKeFrameError, NtsKeFrameLimits, frame_nts_ke_message};
/// # fn main() -> Result<(), NtsKeFrameError> {
/// let bytes = [0x80, 0x00, 0x00, 0x00];
/// let mut records = {
///     let message = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(4, 1)?)?;
///     message.records()
/// };
/// let record = records.next().ok_or(NtsKeFrameError::MissingEndOfMessage)?;
/// assert_eq!(record.record_type(), 0);
/// assert!(record.critical());
/// assert!(record.body().is_empty());
/// assert!(records.next().is_none());
/// # Ok(())
/// # }
/// ```
///
/// External callers cannot construct or change the admitted metadata:
///
/// ```compile_fail,E0451
/// use ntp_wire::RawNtsKeMessage;
/// let _ = RawNtsKeMessage { bytes: &[], count: 0 };
/// ```
///
/// ```compile_fail,E0616
/// use ntp_wire::{NtsKeFrameError, NtsKeFrameLimits, frame_nts_ke_message};
/// # fn main() -> Result<(), NtsKeFrameError> {
/// let bytes = [0x80, 0x00, 0x00, 0x00];
/// let mut message = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(4, 1)?)?;
/// message.count = usize::MAX;
/// # Ok(())
/// # }
/// ```
///
/// ```compile_fail,E0277
/// use ntp_wire::RawNtsKeMessage;
/// let _: RawNtsKeMessage<'static> = Default::default();
/// ```
///
/// A view cannot outlive its original input or permit mutation while still used:
///
/// ```compile_fail,E0597
/// use ntp_wire::{NtsKeFrameError, NtsKeFrameLimits, frame_nts_ke_message};
/// # fn main() -> Result<(), NtsKeFrameError> {
/// let message;
/// {
///     let bytes = [0x80, 0x00, 0x00, 0x00];
///     message = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(4, 1)?)?;
/// }
/// assert_eq!(message.record_count(), 1);
/// # Ok(())
/// # }
/// ```
///
/// ```compile_fail,E0506
/// use ntp_wire::{NtsKeFrameError, NtsKeFrameLimits, frame_nts_ke_message};
/// # fn main() -> Result<(), NtsKeFrameError> {
/// let mut bytes = [0x80, 0x00, 0x00, 0x00];
/// let message = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(4, 1)?)?;
/// bytes[0] = 0x00;
/// assert_eq!(message.bytes()[0], 0x80);
/// # Ok(())
/// # }
/// ```
///
/// This raw view deliberately supplies no payload-bearing debug route:
///
/// ```compile_fail,E0277
/// use ntp_wire::RawNtsKeMessage;
/// fn requires_debug<T: core::fmt::Debug>() {}
/// requires_debug::<RawNtsKeMessage<'static>>();
/// ```
pub struct RawNtsKeMessage<'a> {
    /// Original complete buffer, ending at its sole critical empty EOM.
    bytes: &'a [u8],
    /// Exact admitted record count, including EOM.
    count: usize,
}

impl<'a> RawNtsKeMessage<'a> {
    /// Borrow the original admitted spans; no body bytes are copied or normalized.
    #[must_use]
    pub const fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Return the exact complete-record count, including the final EOM.
    #[must_use]
    pub const fn record_count(&self) -> usize {
        self.count
    }

    /// Start a fresh bounded traversal of unchanged original record bytes.
    #[must_use]
    pub const fn records(&self) -> NtsKeRecords<'a> {
        NtsKeRecords {
            remaining: self.bytes,
        }
    }
}

/// One immutable complete record; type recognition and body semantics are absent.
///
/// Private construction proves at least four bytes and the exact body span.
/// A critical bit is preserved, not interpreted as proof the record is supported.
/// No payload-bearing `Debug` implementation or mutation factory exists.
///
/// ```compile_fail,E0451
/// use ntp_wire::RawNtsKeRecord;
/// let _ = RawNtsKeRecord { bytes: &[] };
/// ```
///
/// ```compile_fail,E0597
/// use ntp_wire::{NtsKeFrameError, NtsKeFrameLimits, frame_nts_ke_message};
/// # fn main() -> Result<(), NtsKeFrameError> {
/// let record;
/// {
///     let bytes = [0x80, 0x00, 0x00, 0x00];
///     let message = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(4, 1)?)?;
///     record = message.records().next().ok_or(NtsKeFrameError::MissingEndOfMessage)?;
/// }
/// assert_eq!(record.record_type(), 0);
/// # Ok(())
/// # }
/// ```
///
/// ```compile_fail,E0277
/// use ntp_wire::RawNtsKeRecord;
/// fn requires_debug<T: core::fmt::Debug>() {}
/// requires_debug::<RawNtsKeRecord<'static>>();
/// ```
pub struct RawNtsKeRecord<'a> {
    /// Four-byte prefix and exactly its declared body bytes.
    bytes: &'a [u8],
}

impl<'a> RawNtsKeRecord<'a> {
    /// Project the independent most-significant critical bit without effects.
    #[must_use]
    pub const fn critical(&self) -> bool {
        self.bytes[0] & 0x80 != 0
    }

    /// Project exactly the fifteen network-order type bits, including unknowns.
    #[must_use]
    pub const fn record_type(&self) -> u16 {
        u16::from_be_bytes([self.bytes[0] & 0x7f, self.bytes[1]])
    }

    /// Borrow this complete original record, including its four-byte prefix.
    #[must_use]
    pub const fn bytes(&self) -> &'a [u8] {
        self.bytes
    }

    /// Borrow the exact declared body, including zero and odd lengths.
    #[must_use]
    pub fn body(&self) -> &'a [u8] {
        &self.bytes[4..]
    }
}

/// Fresh immutable traversal over a completely checked record chain.
///
/// Construction is private. Every step consumes at least four bytes and retains
/// a checked suffix. No failed parse is converted into iterator exhaustion.
/// No payload-bearing `Debug` implementation or mutable construction is exposed.
///
/// ```compile_fail,E0451
/// use ntp_wire::NtsKeRecords;
/// let _ = NtsKeRecords { remaining: &[] };
/// ```
///
/// ```compile_fail,E0597
/// use ntp_wire::{NtsKeFrameError, NtsKeFrameLimits, frame_nts_ke_message};
/// # fn main() -> Result<(), NtsKeFrameError> {
/// let mut records;
/// {
///     let bytes = [0x80, 0x00, 0x00, 0x00];
///     let message = frame_nts_ke_message(&bytes, NtsKeFrameLimits::new(4, 1)?)?;
///     records = message.records();
/// }
/// assert!(records.next().is_some());
/// # Ok(())
/// # }
/// ```
///
/// ```compile_fail,E0277
/// use ntp_wire::NtsKeRecords;
/// fn requires_debug<T: core::fmt::Debug>() {}
/// requires_debug::<NtsKeRecords<'static>>();
/// ```
pub struct NtsKeRecords<'a> {
    /// Empty or a whole admitted suffix of the original record chain.
    remaining: &'a [u8],
}

impl<'a> Iterator for NtsKeRecords<'a> {
    type Item = RawNtsKeRecord<'a>;

    /// Project one proven span; an exhausted iterator remains exhausted.
    ///
    /// The complete-message constructor has proved the indexed prefix, addition
    /// and split fit this exact immutable suffix; no input validation is optional.
    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining.is_empty() {
            return None;
        }
        let length = 4 + usize::from(u16::from_be_bytes([self.remaining[2], self.remaining[3]]));
        let (bytes, remaining) = self.remaining.split_at(length);
        self.remaining = remaining;
        Some(RawNtsKeRecord { bytes })
    }
}

/// Split one complete generic record without reading or copying its body.
///
/// Prefix admission precedes length reads. The second checked split proves the
/// whole declared body exists. Returned slices borrow unchanged input; no state,
/// diagnostics or allocation occurs, and no partial public message is published.
///
/// # Errors
/// Distinguish a partial four-byte prefix from a missing declared body suffix.
fn split_record(input: &[u8]) -> Result<(&[u8], &[u8]), NtsKeFrameError> {
    let (prefix, body_and_tail) = input
        .split_at_checked(4)
        .ok_or(NtsKeFrameError::TruncatedHeader)?;
    let body_length = usize::from(u16::from_be_bytes([prefix[2], prefix[3]]));
    let (_, tail) = body_and_tail
        .split_at_checked(body_length)
        .ok_or(NtsKeFrameError::TruncatedBody)?;
    let consumed = input.len() - tail.len();
    Ok((&input[..consumed], tail))
}

/// Frame exactly one complete NTS-KE buffer under explicit experiment limits.
///
/// Whole-buffer byte admission precedes reads. Each iteration checks the record
/// budget before another prefix and consumes at least four bytes. Success checks
/// every span and exactly one final critical empty EOM before returning a view.
/// Output metadata is constant-size and borrowed; no heap, I/O, clock, TLS,
/// cookie/key acceptance or association changes occur. Unknown record types and
/// non-EOM bodies remain opaque for a later semantic receiver.
///
/// This is a complete-buffer API. Its truncation errors describe supplied bytes,
/// not an observed TCP/TLS EOF. A future stream owner must implement reassembly,
/// aggregate input/work budgets and TLS closure without treating TLS records as
/// KE message boundaries. Such integration is not qualified by this function.
///
/// # Errors
/// Byte limit precedes record count, partial prefix/body, EOM shape and trailing
/// bytes. A complete chain without EOM returns `MissingEndOfMessage`. Errors are
/// redacted and return no partially framed message or permitted external effect.
pub fn frame_nts_ke_message(
    input: &[u8],
    limits: NtsKeFrameLimits,
) -> Result<RawNtsKeMessage<'_>, NtsKeFrameError> {
    if input.len() > limits.max_message_bytes {
        return Err(NtsKeFrameError::ByteLimit);
    }
    let mut remaining = input;
    let mut count = 0_usize;
    while !remaining.is_empty() {
        if count == limits.max_records {
            return Err(NtsKeFrameError::TooManyRecords);
        }
        let (bytes, tail) = split_record(remaining)?;
        count += 1;
        let record = RawNtsKeRecord { bytes };
        if record.record_type() == 0 {
            if !record.critical() || !record.body().is_empty() {
                return Err(NtsKeFrameError::InvalidEndOfMessage);
            }
            if !tail.is_empty() {
                return Err(NtsKeFrameError::TrailingData);
            }
            return Ok(RawNtsKeMessage {
                bytes: input,
                count,
            });
        }
        remaining = tail;
    }
    Err(NtsKeFrameError::MissingEndOfMessage)
}
