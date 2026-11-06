//! Canonical closed CBOR: unsigned u32, UTF-8 text and definite text-keyed maps.
//! Reject all tags, arrays, bytes, booleans/null, floats, negatives and extensions.
use super::{Budget, CBOR_BYTES, Error, KEY_BYTES, MAX_MAP, STRING_BYTES, Wire};

/// Bounded CBOR cursor over one retained immutable byte slice.
struct Reader<'a> {
    /// Retained input bytes; no decoder path reopening occurs.
    bytes: &'a [u8],
    /// Current checked input offset.
    at: usize,
    /// Single shared candidate allocation/work counters.
    budget: Budget,
}
impl<'a> Reader<'a> {
    /// Borrow a checked input span and charge consumed work.
    ///
    /// # Errors
    /// Returns a fixed rejection when the retained span, required field or charged work is unavailable.
    fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let end = self.at.checked_add(n).ok_or(Error::Bounds)?;
        let value = self.bytes.get(self.at..end).ok_or(Error::Cbor)?;
        self.at = end;
        self.budget.work(n)?;
        Ok(value)
    }
    /// Read or write the shortest selected CBOR argument encoding; reject alternatives.
    ///
    /// # Errors
    /// Rejects noncanonical, unsupported or overflowing headers and bounded output failures.
    fn head(&mut self) -> Result<(u8, u64), Error> {
        let byte = self.take(1)?[0];
        let major = byte >> 5;
        let additional = byte & 31;
        let value = match additional {
            0..=23 => u64::from(additional),
            24..=27 => {
                let n = 1usize << usize::from(additional - 24);
                let mut value = 0u64;
                for b in self.take(n)? {
                    value = value
                        .checked_mul(256)
                        .and_then(|v| v.checked_add(u64::from(*b)))
                        .ok_or(Error::Bounds)?;
                }
                let minimum = match n {
                    1 => 24,
                    2 => 256,
                    4 => 65_536,
                    8 => 4_294_967_296,
                    _ => return Err(Error::Cbor),
                };
                if value < minimum {
                    return Err(Error::Cbor);
                }
                value
            }
            _ => return Err(Error::Cbor),
        };
        Ok((major, value))
    }
    /// Check declared text length, strict UTF-8 and repertoire before allocating a `String`.
    ///
    /// # Errors
    /// Rejects excessive length, truncated bytes, invalid UTF-8, forbidden characters or aggregate exhaustion.
    fn text_after_head(&mut self, length: u64, key: bool) -> Result<String, Error> {
        let maximum = if key { KEY_BYTES } else { STRING_BYTES };
        if length > u64::try_from(maximum).map_err(|_| Error::Bounds)? {
            return Err(Error::Bounds);
        }
        let n = usize::try_from(length).map_err(|_| Error::Bounds)?;
        let raw = self.take(n)?;
        let text = std::str::from_utf8(raw).map_err(|_| Error::Utf8)?;
        self.budget.text(text, key)?;
        Ok(text.to_owned())
    }
    /// Read one bounded CBOR value and canonical text-key map without size-hint allocation.
    ///
    /// # Errors
    /// Rejects unsupported, noncanonical, malformed or over-budget values before retaining a result.
    fn value(&mut self, depth: usize) -> Result<Wire, Error> {
        self.budget.node(depth)?;
        let (major, value) = self.head()?;
        match major {
            0 => Ok(Wire::Uint(u32::try_from(value).map_err(|_| Error::Bounds)?)),
            3 => Ok(Wire::Text(self.text_after_head(value, false)?)),
            5 => {
                if depth >= super::MAX_DEPTH {
                    return Err(Error::Bounds);
                }
                if value > u64::try_from(MAX_MAP).map_err(|_| Error::Bounds)? {
                    return Err(Error::Bounds);
                }
                let count = usize::try_from(value).map_err(|_| Error::Bounds)?;
                let mut fields = Vec::new();
                let mut previous: Option<&'a [u8]> = None;
                // Never reserve a peer-provided size hint or capacity.
                for _ in 0..count {
                    self.budget.field()?;
                    let start = self.at;
                    let (kind, length) = self.head()?;
                    if kind != 3 {
                        return Err(Error::Cbor);
                    }
                    let key = self.text_after_head(length, true)?;
                    let encoded = self.bytes.get(start..self.at).ok_or(Error::Cbor)?;
                    if let Some(old) = previous {
                        if old == encoded {
                            return Err(Error::Duplicate);
                        }
                        if old > encoded {
                            return Err(Error::Cbor);
                        }
                    }
                    previous = Some(encoded);
                    let child = self.value(depth + 1)?;
                    fields.push((key, child));
                }
                Ok(Wire::Map(fields))
            }
            _ => Err(Error::Cbor),
        }
    }
}
/// Consume exactly one bounded canonical CBOR resource without retaining decoder diagnostics.
///
/// # Errors
/// Rejects the complete retained candidate on representation, encoding, uniqueness or resource failure.
pub(super) fn read(bytes: &[u8]) -> Result<Wire, Error> {
    if bytes.len() > CBOR_BYTES {
        return Err(Error::Size);
    }
    let mut reader = Reader {
        bytes,
        at: 0,
        budget: Budget::default(),
    };
    let value = reader.value(0)?;
    if reader.at != bytes.len() {
        return Err(Error::Cbor);
    }
    Ok(value)
}

/// Check total encoded-output length before copying any bytes.
///
/// # Errors
/// Rejects overflow or an output length above the fixed CBOR byte limit before appending.
fn put(out: &mut Vec<u8>, bytes: &[u8]) -> Result<(), Error> {
    if out
        .len()
        .checked_add(bytes.len())
        .is_none_or(|n| n > CBOR_BYTES)
    {
        return Err(Error::Bounds);
    }
    out.extend_from_slice(bytes);
    Ok(())
}
/// Read or write the shortest selected CBOR argument encoding; reject alternatives.
///
/// # Errors
/// Rejects noncanonical, unsupported or overflowing headers and bounded output failures.
fn head(out: &mut Vec<u8>, major: u8, value: usize) -> Result<(), Error> {
    let small = u8::try_from(value);
    if value <= 23 {
        put(out, &[(major << 5) | small.map_err(|_| Error::Bounds)?])
    } else if let Ok(value) = small {
        put(out, &[(major << 5) | 0x18, value])
    } else if let Ok(value) = u16::try_from(value) {
        put(out, &[(major << 5) | 0x19])?;
        put(out, &value.to_be_bytes())
    } else if let Ok(value) = u32::try_from(value) {
        put(out, &[(major << 5) | 0x1a])?;
        put(out, &value.to_be_bytes())
    } else {
        Err(Error::Bounds)
    }
}
/// Write one canonical UTF-8 text value.
///
/// # Errors
/// Rejects the wrong wire variant, invalid text or bounded output failure for this receiving operation.
fn text(out: &mut Vec<u8>, value: &str) -> Result<(), Error> {
    head(out, 3, value.len())?;
    put(out, value.as_bytes())
}
/// Encode only bounded selected primitives and canonical text-key map order.
///
/// # Errors
/// Rejects excessive nesting, oversized or duplicate maps, and output capacity exhaustion.
fn emit(out: &mut Vec<u8>, wire: &Wire, depth: usize) -> Result<(), Error> {
    if depth > super::MAX_DEPTH {
        return Err(Error::Bounds);
    }
    match wire {
        Wire::Uint(value) => head(out, 0, usize::try_from(*value).map_err(|_| Error::Bounds)?),
        Wire::Text(value) => text(out, value),
        Wire::Map(fields) => {
            if depth >= super::MAX_DEPTH {
                return Err(Error::Bounds);
            }
            if fields.len() > MAX_MAP {
                return Err(Error::Bounds);
            }
            let mut ordered: Vec<_> = fields.iter().collect();
            // All keys are text, so this equals bytewise canonical encoded-key order.
            ordered.sort_by(|a, b| {
                a.0.len()
                    .cmp(&b.0.len())
                    .then_with(|| a.0.as_bytes().cmp(b.0.as_bytes()))
            });
            if ordered.windows(2).any(|pair| pair[0].0 == pair[1].0) {
                return Err(Error::Duplicate);
            }
            head(out, 5, ordered.len())?;
            for (key, value) in ordered {
                text(out, key)?;
                emit(out, value, depth + 1)?;
            }
            Ok(())
        }
    }
}
/// Create one bounded deterministic CBOR buffer from an admitted wire projection.
///
/// # Errors
/// Rejects an inadmissible wire shape or fixed canonical-output bound.
pub(super) fn encode(wire: &Wire) -> Result<Vec<u8>, Error> {
    let mut bytes = Vec::new();
    emit(&mut bytes, wire, 0)?;
    Ok(bytes)
}
