//! Independent incremental NTS-KE framing facts for REQ-8/BND-2/CHECK-6/25.
//! Supplied chunks are not TLS records; finish observes no transport EOF/identity.
#![forbid(unsafe_code)]

// Proposed API contract, to be reconciled before Root applies this test patch:
// NtsKeAccumulator<'buf>::new(&'buf mut [u8], NtsKeFrameLimits)
//     -> Result<Self, NtsKeAccumulatorError>
// push(&mut self, &[u8]) -> Result<NtsKeProgress, NtsKeAccumulatorError>
// finish(self) -> Result<RawNtsKeMessage<'buf>, NtsKeAccumulatorError>
// NtsKeAccumulatorError::{StorageTooSmall, Frame(NtsKeFrameError)} and
// NtsKeProgress::{NeedMore, EndOfMessage}: Clone + Copy + Debug + Eq + PartialEq.
// Storage must cover the entire declared byte cap. Empty chunks report unchanged
// progress, never EOF. Every rejected chunk preserves all storage from before that
// call, poisons the owner, and latches its first error. No reset/Default/Debug.
// Actual-byte admission precedes parsing within each call; unseen future chunks
// cannot retroactively override an already latched framing error.
//
// Proposed public-API examples, NOT executed as integration-file comments.
// Root must place these on maintained API docs before claiming rustdoc evidence;
// the legal scoped-owner control is an actual test below.
//
// ```compile_fail,E0506
// use ntp_wire::{NtsKeAccumulator, NtsKeAccumulatorError, NtsKeFrameLimits};
// fn cannot_mutate() -> Result<(), NtsKeAccumulatorError> {
//     let mut storage = [0_u8; 4];
//     let cap = NtsKeFrameLimits::new(4, 1).map_err(NtsKeAccumulatorError::Frame)?;
//     let mut owner = NtsKeAccumulator::new(&mut storage, cap)?;
//     owner.push(&[0x80, 0, 0, 0])?;
//     let frame = owner.finish()?;
//     storage[0] = 0;
//     assert_eq!(frame.bytes()[0], 0x80);
//     Ok(())
// }
// ```
//
// ```compile_fail,E0515
// use ntp_wire::{NtsKeAccumulator, NtsKeAccumulatorError, NtsKeFrameLimits, RawNtsKeMessage};
// fn cannot_escape() -> Result<RawNtsKeMessage<'static>, NtsKeAccumulatorError> {
//     let mut storage = [0_u8; 4];
//     let cap = NtsKeFrameLimits::new(4, 1).map_err(NtsKeAccumulatorError::Frame)?;
//     let mut owner = NtsKeAccumulator::new(&mut storage, cap)?;
//     owner.push(&[0x80, 0, 0, 0])?;
//     owner.finish()
// }
// ```
//
// ```compile_fail,E0382
// use ntp_wire::{NtsKeAccumulator, NtsKeAccumulatorError, NtsKeFrameLimits};
// fn cannot_reuse() -> Result<(), NtsKeAccumulatorError> {
//     let mut storage = [0_u8; 4];
//     let cap = NtsKeFrameLimits::new(4, 1).map_err(NtsKeAccumulatorError::Frame)?;
//     let mut owner = NtsKeAccumulator::new(&mut storage, cap)?;
//     owner.push(&[0x80, 0, 0, 0])?;
//     let _frame = owner.finish()?;
//     owner.push(&[])?;
//     Ok(())
// }
// ```

use ntp_wire::{
    NtsKeAccumulator, NtsKeAccumulatorError, NtsKeFrameError, NtsKeFrameLimits,
    NtsKeNegotiationError, NtsKeProgress, negotiate_nts_ke_ntpv4, read_nts_ke_ntpv4_offer,
};

/// Independent critical empty end-of-message record.
static EOM: [u8; 4] = [0x80, 0, 0, 0];
/// Independent NPN0, ordered AEAD15 and EOM, with three critical prefixes.
static REQUEST: [u8; 16] = [0x80, 1, 0, 2, 0, 0, 0x80, 4, 0, 2, 0, 15, 0x80, 0, 0, 0];
/// A complete EOM-looking body remains opaque until its enclosing record ends.
static OPAQUE: [u8; 12] = [0, 42, 0, 4, 0x80, 0, 0, 0, 0x80, 0, 0, 0];
/// Generic record bodies need not be word aligned.
static ODD: [u8; 11] = [0x12, 0x34, 0, 3, 0xaa, 0xbb, 0xcc, 0x80, 0, 0, 0];
/// Distinguish admitted copies from untouched caller storage.
const SENTINEL: u8 = 0xa5;

/// Supply explicit existing limits without selecting a parser default.
/// # Errors
/// Preserve an unexpected limits or accumulator-construction failure.
fn stream(
    storage: &mut [u8],
    bytes: usize,
    records: usize,
) -> Result<NtsKeAccumulator<'_>, String> {
    let limits = NtsKeFrameLimits::new(bytes, records).map_err(|e| format!("limits: {e:?}"))?;
    NtsKeAccumulator::new(storage, limits).map_err(|e| format!("new: {e:?}"))
}

/// Every split of finite literals preserves independent bytes, counts and spans.
/// # Errors
/// Preserve rejection of an independently complete valid literal.
/// # Panics
/// Require exact progress, original destination borrowing and untouched tail.
#[test]
fn ke_stream_all_small_literal_splits_preserve_original_storage() -> Result<(), String> {
    for (literal, count, kinds) in [
        (&EOM[..], 1, &[(0, true)][..]),
        (&REQUEST[..], 3, &[(1, true), (4, true), (0, true)][..]),
        (&OPAQUE[..], 2, &[(42, false), (0, true)][..]),
        (&ODD[..], 2, &[(0x1234, false), (0, true)][..]),
    ] {
        for cut in 0..=literal.len() {
            let mut storage = [SENTINEL; 64];
            let start = storage.as_ptr();
            let frame = {
                let mut owner = stream(&mut storage, literal.len(), count)?;
                let interim = if cut == literal.len() {
                    NtsKeProgress::EndOfMessage
                } else {
                    NtsKeProgress::NeedMore
                };
                assert_eq!(owner.push(&literal[..cut]), Ok(interim));
                assert_eq!(owner.push(&[]), Ok(interim));
                assert_eq!(owner.push(&literal[cut..]), Ok(NtsKeProgress::EndOfMessage));
                assert_eq!(owner.push(&[]), Ok(NtsKeProgress::EndOfMessage));
                owner.finish().map_err(|e| format!("finish: {e:?}"))?
            };
            assert_eq!(frame.bytes(), literal);
            assert!(core::ptr::eq(frame.bytes().as_ptr(), start));
            assert_eq!(frame.record_count(), count);
            assert_eq!(
                frame
                    .records()
                    .map(|r| (r.record_type(), r.critical()))
                    .collect::<Vec<_>>(),
                kinds
            );
            let mut rows = frame.records();
            for _ in 0..count {
                assert!(rows.next().is_some());
            }
            assert!(rows.next().is_none());
            assert!(rows.next().is_none());
            assert!(storage[literal.len()..].iter().all(|b| *b == SENTINEL));
        }
    }
    Ok(())
}

/// Single bytes and arbitrary empty chunks cannot manufacture an EOF or EOM.
/// # Errors
/// Preserve valid admission or existing semantic-consumer rejection.
/// # Panics
/// Require final-only completion, exact bodies and actual body-span borrowing.
#[test]
fn ke_stream_bytewise_empty_chunks_and_borrowed_records_are_explicit() -> Result<(), String> {
    let mut storage = [SENTINEL; 24];
    let aead_start = storage[10..].as_ptr();
    let mut owner = stream(&mut storage, 16, 3)?;
    for (index, byte) in REQUEST.iter().enumerate() {
        assert_eq!(owner.push(&[]), Ok(NtsKeProgress::NeedMore));
        assert_eq!(
            owner.push(core::slice::from_ref(byte)),
            Ok(if index == 15 {
                NtsKeProgress::EndOfMessage
            } else {
                NtsKeProgress::NeedMore
            })
        );
    }
    let frame = owner.finish().map_err(|e| format!("finish: {e:?}"))?;
    assert_eq!(
        frame
            .records()
            .map(|r| r.body().to_vec())
            .collect::<Vec<_>>(),
        [vec![0, 0], vec![0, 15], vec![]]
    );
    let offer = read_nts_ke_ntpv4_offer(&frame).map_err(|e| format!("offer: {e:?}"))?;
    assert_eq!(offer.request_bytes(), REQUEST);
    assert_eq!(offer.offered_aead_bytes(), [0, 15]);
    assert!(core::ptr::eq(
        offer.offered_aead_bytes().as_ptr(),
        aead_start
    ));
    assert_eq!(&storage[16..], &[SENTINEL; 8]);
    Ok(())
}

/// Partial chunks remain incomplete; only explicit finalization reports truncation.
/// # Errors
/// Preserve constructor rejection of the independently bounded fixtures.
/// # Panics
/// Require the specific header/body/missing-EOM causes, not inferred TLS EOF.
#[test]
fn ke_stream_finish_distinguishes_partial_headers_bodies_and_missing_eom() -> Result<(), String> {
    for (prefix, cause) in [
        (&[][..], NtsKeFrameError::MissingEndOfMessage),
        (&REQUEST[..1], NtsKeFrameError::TruncatedHeader),
        (&REQUEST[..3], NtsKeFrameError::TruncatedHeader),
        (&REQUEST[..5], NtsKeFrameError::TruncatedBody),
        (&REQUEST[..6], NtsKeFrameError::MissingEndOfMessage),
        (&REQUEST[..11], NtsKeFrameError::TruncatedBody),
        (&REQUEST[..12], NtsKeFrameError::MissingEndOfMessage),
        (&REQUEST[..15], NtsKeFrameError::TruncatedHeader),
    ] {
        let mut storage = [SENTINEL; 24];
        let mut owner = stream(&mut storage, 16, 3)?;
        assert_eq!(owner.push(prefix), Ok(NtsKeProgress::NeedMore));
        assert_eq!(owner.push(&[]), Ok(NtsKeProgress::NeedMore));
        assert_eq!(
            owner.finish().err(),
            Some(NtsKeAccumulatorError::Frame(cause))
        );
        assert_eq!(&storage[..prefix.len()], prefix);
        assert!(storage[prefix.len()..].iter().all(|b| *b == SENTINEL));
    }
    Ok(())
}

/// Chunk rejection is storage-atomic and terminal, including after an observed EOM.
/// # Errors
/// Preserve rejection of valid prefixes and fresh healthy EOM neighbors.
/// # Panics
/// Require first-error latching, no rejected bytes, and no revival by empty pushes.
#[test]
fn ke_stream_rejected_chunks_are_atomic_and_first_error_is_latched() -> Result<(), String> {
    for (prefix, bad, bytes, records, cause) in [
        (
            &REQUEST[..6],
            &[0; 11][..],
            16,
            3,
            NtsKeFrameError::ByteLimit,
        ),
        (
            &REQUEST[..12],
            &EOM[..],
            16,
            2,
            NtsKeFrameError::TooManyRecords,
        ),
        (
            &REQUEST[..12],
            &[0; 4][..],
            16,
            3,
            NtsKeFrameError::InvalidEndOfMessage,
        ),
        (
            &[][..],
            &[0x80, 0, 0, 1, 7][..],
            8,
            2,
            NtsKeFrameError::InvalidEndOfMessage,
        ),
        (
            &[][..],
            &[0x80, 0, 0, 0, 7][..],
            8,
            2,
            NtsKeFrameError::TrailingData,
        ),
        (&EOM[..], &[7][..], 8, 2, NtsKeFrameError::TrailingData),
        (&EOM[..], &[7][..], 4, 1, NtsKeFrameError::ByteLimit),
    ] {
        let mut storage = [SENTINEL; 64];
        let mut expected = storage;
        expected[..prefix.len()].copy_from_slice(prefix);
        let error = NtsKeAccumulatorError::Frame(cause);
        {
            let mut owner = stream(&mut storage, bytes, records)?;
            assert_eq!(
                owner.push(prefix),
                Ok(if prefix == EOM {
                    NtsKeProgress::EndOfMessage
                } else {
                    NtsKeProgress::NeedMore
                })
            );
            assert_eq!(owner.push(bad), Err(error));
            assert_eq!(owner.push(&[]), Err(error));
            assert_eq!(owner.push(&[0; 64]), Err(error));
            assert_eq!(owner.finish().err(), Some(error));
        }
        assert_eq!(storage, expected);
        let mut fresh = stream(&mut storage, 4, 1)?;
        assert_eq!(fresh.push(&EOM), Ok(NtsKeProgress::EndOfMessage));
        assert_eq!(
            fresh
                .finish()
                .map_err(|e| format!("neighbor: {e:?}"))?
                .bytes(),
            EOM
        );
    }
    Ok(())
}

/// Storage capacity is distinct from the explicit byte/count admission limits.
/// # Errors
/// Preserve valid constructor and complete-message neighbors.
/// # Panics
/// Require exact budget boundaries and an unchanged rejected constructor buffer.
#[test]
fn ke_stream_storage_byte_and_record_neighbors_remain_distinct() -> Result<(), String> {
    let mut short = [SENTINEL; 7];
    let cap = NtsKeFrameLimits::new(8, 2).map_err(|e| format!("limits: {e:?}"))?;
    assert_eq!(
        NtsKeAccumulator::new(&mut short, cap).err(),
        Some(NtsKeAccumulatorError::StorageTooSmall)
    );
    assert_eq!(short, [SENTINEL; 7]);
    let literal = [0, 42, 0, 0, 0x80, 0, 0, 0];
    for (bytes, records, expected) in [
        (7, 2, Some(NtsKeFrameError::ByteLimit)),
        (8, 1, Some(NtsKeFrameError::TooManyRecords)),
        (8, 2, None),
        (9, 2, None),
    ] {
        let mut storage = [SENTINEL; 16];
        let mut owner = stream(&mut storage, bytes, records)?;
        if let Some(cause) = expected {
            let error = NtsKeAccumulatorError::Frame(cause);
            assert_eq!(owner.push(&literal), Err(error));
            assert_eq!(owner.finish().err(), Some(error));
            assert_eq!(storage, [SENTINEL; 16]);
        } else {
            assert_eq!(owner.push(&literal), Ok(NtsKeProgress::EndOfMessage));
            let frame = owner.finish().map_err(|e| format!("finish: {e:?}"))?;
            assert_eq!(frame.bytes(), literal);
            assert_eq!(frame.record_count(), 2);
            assert_eq!(&storage[8..], &[SENTINEL; 8]);
        }
    }
    Ok(())
}

/// Stream completion remains inert framing; real request/response owners still decide.
/// # Errors
/// Preserve unexpected framing or semantic rejection of healthy literals.
/// # Panics
/// Require offered15/cookie bytes, typed peer error, and critical-extension rejection.
#[test]
fn ke_stream_complete_views_reach_existing_semantic_consumers() -> Result<(), String> {
    let mut request_storage = [SENTINEL; 64];
    let mut request = stream(&mut request_storage, 64, 8)?;
    assert_eq!(request.push(&REQUEST), Ok(NtsKeProgress::EndOfMessage));
    let raw = request.finish().map_err(|e| format!("request: {e:?}"))?;
    let offer = read_nts_ke_ntpv4_offer(&raw).map_err(|e| format!("offer: {e:?}"))?;
    let reply = [
        0x80, 1, 0, 2, 0, 0, 0, 4, 0, 2, 0, 15, 0, 5, 0, 1, 0xaa, 0x80, 0, 0, 0,
    ];
    let mut response_storage = [SENTINEL; 64];
    let cookie_start = response_storage[16..].as_ptr();
    let mut response = stream(&mut response_storage, 64, 8)?;
    assert_eq!(response.push(&reply[..9]), Ok(NtsKeProgress::NeedMore));
    assert_eq!(response.push(&reply[9..18]), Ok(NtsKeProgress::NeedMore));
    assert_eq!(response.push(&reply[18..]), Ok(NtsKeProgress::EndOfMessage));
    let pending = negotiate_nts_ke_ntpv4(
        &offer,
        response.finish().map_err(|e| format!("response: {e:?}"))?,
    )
    .map_err(|e| format!("negotiate: {e:?}"))?;
    assert_eq!(pending.aead_id(), 15);
    assert_eq!(pending.cookie_count(), 1);
    let cookie = pending.cookies().next().ok_or("missing literal cookie")?;
    assert_eq!(cookie, [0xaa]);
    assert!(core::ptr::eq(cookie.as_ptr(), cookie_start));
    let peer_error = [0x80, 2, 0, 2, 0, 1, 0x80, 0, 0, 0];
    let mut error_storage = [SENTINEL; 64];
    let mut response = stream(&mut error_storage, 64, 8)?;
    assert_eq!(response.push(&peer_error), Ok(NtsKeProgress::EndOfMessage));
    assert_eq!(
        negotiate_nts_ke_ntpv4(
            &offer,
            response
                .finish()
                .map_err(|e| format!("error frame: {e:?}"))?
        )
        .err(),
        Some(NtsKeNegotiationError::PeerError { code: 1 })
    );
    for critical in [0x7f, 0xff] {
        let mut bytes = vec![critical, 0xff, 0, 0];
        bytes.extend_from_slice(&REQUEST);
        let mut storage = [SENTINEL; 64];
        let mut owner = stream(&mut storage, 64, 8)?;
        assert_eq!(owner.push(&bytes), Ok(NtsKeProgress::EndOfMessage));
        let raw = owner
            .finish()
            .map_err(|e| format!("unknown frame: {e:?}"))?;
        if critical == 0xff {
            assert_eq!(
                read_nts_ke_ntpv4_offer(&raw).err(),
                Some(NtsKeNegotiationError::UnsupportedCriticalRecord)
            );
        } else {
            assert_eq!(
                read_nts_ke_ntpv4_offer(&raw)
                    .map_err(|e| format!("neighbor offer: {e:?}"))?
                    .offered_aead_bytes(),
                [0, 15]
            );
        }
    }
    Ok(())
}

/// Dropping a poisoned owner is not implicit reset, publication or buffer erasure.
/// # Errors
/// Preserve valid initial prefix and fresh-owner control admission.
/// # Panics
/// Require original error, original prefix and unchanged rejected/tail storage.
#[test]
fn ke_stream_failed_owner_drop_preserves_the_last_admitted_prefix() -> Result<(), String> {
    let mut storage = [SENTINEL; 16];
    let prefix = [0, 42, 0, 0];
    let error = NtsKeAccumulatorError::Frame(NtsKeFrameError::InvalidEndOfMessage);
    {
        let mut owner = stream(&mut storage, 8, 2)?;
        assert_eq!(owner.push(&prefix), Ok(NtsKeProgress::NeedMore));
        assert_eq!(owner.push(&[0, 0, 0, 0]), Err(error));
        assert_eq!(owner.push(&EOM), Err(error));
        assert_eq!(owner.push(&[]), Err(error));
        // The owner ends here without finish; only a fresh checked owner may reuse storage.
    }
    assert_eq!(&storage[..4], prefix);
    assert_eq!(&storage[4..], &[SENTINEL; 12]);
    let mut fresh = stream(&mut storage, 4, 1)?;
    assert_eq!(fresh.push(&EOM), Ok(NtsKeProgress::EndOfMessage));
    assert_eq!(
        fresh
            .finish()
            .map_err(|e| format!("fresh finish: {e:?}"))?
            .bytes(),
        EOM
    );
    Ok(())
}

/// Serialize independently declared opaque-record lengths, not a production oracle.
/// # Errors
/// Reject an unbounded fixture before allocation or checked-size overflow.
fn literal_records(lengths: &[u16]) -> Result<Vec<u8>, String> {
    let size = lengths
        .iter()
        .try_fold(4_usize, |sum, n| sum.checked_add(4 + usize::from(*n)))
        .ok_or("fixture size overflow")?;
    if size > 131_073 {
        return Err("fixture exceeds131073 bytes".into());
    }
    let mut bytes = Vec::with_capacity(size);
    for length in lengths {
        let octets = length.to_be_bytes();
        bytes.extend_from_slice(&[0, 42, octets[0], octets[1]]);
        bytes.resize(bytes.len() + usize::from(*length), 0x5a);
    }
    bytes.extend_from_slice(&EOM);
    Ok(bytes)
}

/// Wide byte limits use a few selected fragmentation points, not quadratic splits.
/// # Errors
/// Preserve an unexpected valid wide-message or limits rejection.
/// # Panics
/// Require65535/65536/65537 and128KiB endpoints, whole bytes and unchanged tail.
#[test]
fn ke_stream_wide_acceptance_and_actual_byte_excess_are_separate() -> Result<(), String> {
    for (size, lengths) in [
        (65_535, &[65_527_u16][..]),
        (65_536, &[65_528][..]),
        (65_537, &[65_529][..]),
        (131_071, &[65_535, 65_524][..]),
        (131_072, &[65_535, 65_525][..]),
    ] {
        let bytes = literal_records(lengths)?;
        assert_eq!(bytes.len(), size);
        let mut storage = vec![SENTINEL; 131_080];
        let mut owner = stream(&mut storage, 131_072, 3)?;
        for chunk in [
            &bytes[..3],
            &bytes[3..4],
            &bytes[4..size - 3],
            &bytes[size - 3..size - 1],
        ] {
            assert_eq!(owner.push(chunk), Ok(NtsKeProgress::NeedMore));
        }
        assert_eq!(
            owner.push(&bytes[size - 1..]),
            Ok(NtsKeProgress::EndOfMessage)
        );
        let frame = owner.finish().map_err(|e| format!("wide finish: {e:?}"))?;
        assert_eq!(frame.bytes(), bytes);
        assert_eq!(frame.record_count(), lengths.len() + 1);
        assert!(storage[size..].iter().all(|b| *b == SENTINEL));
    }
    let bytes = literal_records(&[65_535, 65_526])?;
    assert_eq!(bytes.len(), 131_073);
    let mut storage = vec![SENTINEL; 131_080];
    let mut owner = stream(&mut storage, 131_072, 3)?;
    assert_eq!(owner.push(&bytes[..131_072]), Ok(NtsKeProgress::NeedMore));
    let error = NtsKeAccumulatorError::Frame(NtsKeFrameError::ByteLimit);
    assert_eq!(owner.push(&bytes[131_072..]), Err(error));
    assert_eq!(owner.finish().err(), Some(error));
    assert_eq!(&storage[..131_072], &bytes[..131_072]);
    assert!(storage[131_072..].iter().all(|b| *b == SENTINEL));
    Ok(())
}

/// Record count includes EOM and reaches the existing independent absolute limit.
/// # Errors
/// Preserve valid maximal record/message neighbors.
/// # Panics
/// Require exact32768-record acceptance and independent one-less count rejection.
#[test]
fn ke_stream_existing_absolute_record_limit_includes_eom() -> Result<(), String> {
    let bytes = literal_records(&vec![0; 32_767])?;
    assert_eq!(bytes.len(), 131_072);
    let mut storage = vec![SENTINEL; 131_080];
    let mut owner = stream(&mut storage, 131_072, 32_767)?;
    let error = NtsKeAccumulatorError::Frame(NtsKeFrameError::TooManyRecords);
    assert_eq!(owner.push(&bytes), Err(error));
    assert_eq!(owner.finish().err(), Some(error));
    assert!(storage.iter().all(|b| *b == SENTINEL));
    let mut owner = stream(&mut storage, 131_072, 32_768)?;
    assert_eq!(owner.push(&bytes), Ok(NtsKeProgress::EndOfMessage));
    let frame = owner
        .finish()
        .map_err(|e| format!("record neighbor: {e:?}"))?;
    assert_eq!(frame.bytes(), bytes);
    assert_eq!(frame.record_count(), 32_768);
    assert_eq!(&storage[131_072..], &[SENTINEL; 8]);
    Ok(())
}
