//! Strict UTF-8 first, then allocation preflight and duplicate-aware Serde seeds.
//! No `serde_json::Value`, derived positional structs, `size_hint` reserve or repair.
use super::{Budget, Error, JSON_BYTES, KEY_BYTES, MAX_DEPTH, MAX_MAP, Wire};
use serde::de::Error as _;
use serde::de::{self, DeserializeSeed, MapAccess, Visitor};
use std::fmt;

/// Maximum encoded JSON string-token bytes before Serde allocation.
const ENCODED_STRING: usize = 6_144;
/// Maximum lexical JSON tokens before Serde is invoked.
const TOKENS: usize = 8_192;

/// Only a lexical resource guard. Serde, not this scan, decides JSON grammar and
/// Unicode escape validity. Each Serde scratch/string allocation is capped by
/// the already checked retained encoded-token span.
///
/// # Errors
/// Rejects unselected lexical tokens, malformed bounded spans or excessive encoded work and nesting.
fn preflight(bytes: &[u8]) -> Result<(), Error> {
    let mut i = 0usize;
    let mut depth = 0usize;
    let mut tokens = 0usize;
    if bytes.iter().copied().find(|b| !b" \r\n\t".contains(b)) != Some(b'{') {
        return Err(Error::Json);
    }
    while i < bytes.len() {
        match bytes[i] {
            b'"' => {
                tokens += 1;
                i += 1;
                let start = i;
                let mut escaped = false;
                let mut closed = false;
                while i < bytes.len() {
                    let byte = bytes[i];
                    if !escaped && byte == b'"' {
                        closed = true;
                        break;
                    }
                    escaped = !escaped && byte == b'\\';
                    i += 1;
                    if i - start > ENCODED_STRING {
                        return Err(Error::Bounds);
                    }
                }
                if !closed {
                    return Err(Error::Json);
                }
                i += 1;
            }
            b'{' => {
                depth += 1;
                if depth > MAX_DEPTH {
                    return Err(Error::Bounds);
                }
                tokens += 1;
                i += 1;
            }
            b'}' => {
                depth = depth.checked_sub(1).ok_or(Error::Json)?;
                tokens += 1;
                i += 1;
            }
            b'0'..=b'9' => {
                tokens += 1;
                let start = i;
                let mut number = 0u32;
                while i < bytes.len() && bytes[i].is_ascii_digit() {
                    number = number
                        .checked_mul(10)
                        .and_then(|n| n.checked_add(u32::from(bytes[i] - b'0')))
                        .ok_or(Error::Bounds)?;
                    i += 1;
                    if i - start > 10 {
                        return Err(Error::Bounds);
                    }
                }
                if i - start > 1 && bytes[start] == b'0' {
                    return Err(Error::Json);
                }
                if i < bytes.len() && b".eE+-".contains(&bytes[i]) {
                    return Err(Error::Schema);
                }
            }
            b':' | b',' => {
                tokens += 1;
                i += 1;
            }
            b' ' | b'\n' | b'\r' | b'\t' => {
                i += 1;
            }
            // The selected schema has no arrays, booleans, null, signed/floating
            // numbers or alternate tokens. Serde still validates full syntax.
            _ => return Err(Error::Schema),
        }
        if tokens > TOKENS {
            return Err(Error::Bounds);
        }
    }
    if depth != 0 {
        return Err(Error::Json);
    }
    Ok(())
}
/// Duplicate-aware map-key/value string admission with a borrowed aggregate budget.
struct TextSeed<'a> {
    /// Single shared candidate allocation/work counters.
    budget: &'a mut Budget,
    /// Whether the narrower map-key length bound applies.
    key: bool,
}
impl<'de> DeserializeSeed<'de> for TextSeed<'_> {
    /// Decoded result carried by this bounded Serde seed.
    type Value = String;
    /// Decode only the selected bounded primitive/string shape with the shared budget.
    ///
    /// # Errors
    /// Propagates a fixed setup or admission failure while preparing the discriminating control.
    fn deserialize<D: de::Deserializer<'de>>(self, d: D) -> Result<String, D::Error> {
        d.deserialize_string(self)
    }
}
impl Visitor<'_> for TextSeed<'_> {
    /// Decoded result carried by this bounded Serde seed.
    type Value = String;
    /// Describe the fixed primitive requirement without catalog content.
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("bounded catalog text")
    }
    /// Check decoded borrowed text before creating its owned representation.
    ///
    /// # Errors
    /// Propagates a fixed setup or admission failure while preparing the discriminating control.
    fn visit_str<E: de::Error>(self, value: &str) -> Result<String, E> {
        self.budget.text(value, self.key).map_err(E::custom)?;
        Ok(value.to_owned())
    }
    /// Check the preflight-bounded decoded string before retaining it.
    ///
    /// # Errors
    /// Propagates a fixed setup or admission failure while preparing the discriminating control.
    fn visit_string<E: de::Error>(self, value: String) -> Result<String, E> {
        self.budget.text(&value, self.key).map_err(E::custom)?;
        Ok(value)
    }
}
/// Bounded recursive Serde wire seed; no generic unbounded Value is built.
struct Seed<'a> {
    /// Single shared candidate allocation/work counters.
    budget: &'a mut Budget,
    /// Current bounded value depth.
    depth: usize,
}
impl<'de> DeserializeSeed<'de> for Seed<'_> {
    /// Decoded result carried by this bounded Serde seed.
    type Value = Wire;
    /// Decode only the selected bounded primitive/string shape with the shared budget.
    ///
    /// # Errors
    /// Propagates a fixed setup or admission failure while preparing the discriminating control.
    fn deserialize<D: de::Deserializer<'de>>(self, d: D) -> Result<Wire, D::Error> {
        self.budget.node(self.depth).map_err(D::Error::custom)?;
        d.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for Seed<'_> {
    /// Decoded result carried by this bounded Serde seed.
    type Value = Wire;
    /// Describe the fixed primitive requirement without catalog content.
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("closed catalog primitive")
    }
    /// Narrow a JSON unsigned integer to the selected u32 representation.
    ///
    /// # Errors
    /// Propagates a fixed setup or admission failure while preparing the discriminating control.
    fn visit_u64<E: de::Error>(self, value: u64) -> Result<Wire, E> {
        Ok(Wire::Uint(
            u32::try_from(value).map_err(|_| E::custom("integer range"))?,
        ))
    }
    /// Check decoded borrowed text before creating its owned representation.
    ///
    /// # Errors
    /// Propagates a fixed setup or admission failure while preparing the discriminating control.
    fn visit_str<E: de::Error>(self, value: &str) -> Result<Wire, E> {
        self.budget.text(value, false).map_err(E::custom)?;
        Ok(Wire::Text(value.to_owned()))
    }
    /// Check the preflight-bounded decoded string before retaining it.
    ///
    /// # Errors
    /// Propagates a fixed setup or admission failure while preparing the discriminating control.
    fn visit_string<E: de::Error>(self, value: String) -> Result<Wire, E> {
        self.budget.text(&value, false).map_err(E::custom)?;
        Ok(Wire::Text(value))
    }
    /// Reject decoded duplicate keys and bound fields/nodes before map growth.
    ///
    /// # Errors
    /// Propagates a fixed setup or admission failure while preparing the discriminating control.
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<Wire, A::Error> {
        if self.depth >= MAX_DEPTH {
            self.budget.failure = Some(Error::Bounds);
            return Err(A::Error::custom("object depth"));
        }
        let mut fields: Vec<(String, Wire)> = Vec::new();
        while let Some(key) = map.next_key_seed(TextSeed {
            budget: &mut *self.budget,
            key: true,
        })? {
            self.budget.field().map_err(A::Error::custom)?;
            if fields.len() >= MAX_MAP || key.len() > KEY_BYTES {
                self.budget.failure = Some(Error::Bounds);
                return Err(A::Error::custom("map bound"));
            }
            for (old, _) in &fields {
                self.budget
                    .work(old.len().max(key.len()))
                    .map_err(A::Error::custom)?;
                if old == &key {
                    self.budget.failure = Some(Error::Duplicate);
                    return Err(A::Error::custom("duplicate decoded key"));
                }
            }
            let value = map.next_value_seed(Seed {
                budget: &mut *self.budget,
                depth: self.depth + 1,
            })?;
            fields.push((key, value));
        }
        Ok(Wire::Map(fields))
    }
}
/// Consume exactly one bounded JSON source without retaining decoder diagnostics.
///
/// # Errors
/// Rejects the complete retained candidate on representation, encoding, uniqueness or resource failure.
pub(super) fn read(bytes: &[u8]) -> Result<Wire, Error> {
    if bytes.len() > JSON_BYTES {
        return Err(Error::Size);
    }
    // This exact retained buffer goes to Serde. No detection/transcoding/lossy IO.
    std::str::from_utf8(bytes).map_err(|_| Error::Utf8)?;
    if bytes.starts_with(&[0xef, 0xbb, 0xbf]) {
        return Err(Error::Bom);
    }
    preflight(bytes)?;
    let mut budget = Budget::default();
    budget.work(bytes.len())?;
    let mut decoder = serde_json::Deserializer::from_slice(bytes);
    let value = Seed {
        budget: &mut budget,
        depth: 0,
    }
    .deserialize(&mut decoder)
    .map_err(|_| budget.failure.unwrap_or(Error::Json))?;
    decoder.end().map_err(|_| Error::Json)?;
    Ok(value)
}
