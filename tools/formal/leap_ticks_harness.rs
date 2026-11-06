//! Full-u64 whole-second/Q32 bridge through the actual conversion and coordinate.
//!
//! No constructor precondition is assumed. Metadata/Garde/ledger admission and
//! real date provenance remain separate obligations; upper raw values are not
//! silently interpreted as valid leap metadata.

/// Independent scalar cutoff supplied by signed-i64 whole-second semantics.
const MAX_ADMITTED_WHOLE_SECOND: u64 = 9_223_372_036_854_775_807;

/// Observe exact conversion and classify the actual public coordinate constructor.
fn leap_ticks_contract(seconds: u64) -> (i128, bool) {
    let observed = leap_ticks::proof_second_ticks(seconds);
    // The specification concatenates 32 zero fraction bits, rather than copying
    // the subject's signed multiplication. All 64 input bits remain meaningful.
    let encoded = u128::from(seconds) << 32;
    assert!(observed >= 0, "whole seconds remain nonnegative");
    assert!(encoded <= i128::MAX as u128, "full u64 Q32 fits signed i128");
    assert_eq!(observed as u128, encoded, "exact unsigned Q32 encoding");
    assert_eq!(observed as u128 & 0xffff_ffff, 0, "zero fractional bits");
    assert_eq!(observed as u128 >> 32, u128::from(seconds), "full second bits retained");

    let coordinate = coordinate::UtcCoordinateQ32::new(observed);
    let admitted = coordinate.is_ok();
    assert_eq!(
        admitted,
        seconds <= MAX_ADMITTED_WHOLE_SECOND,
        "coordinate admission iff scalar envelope"
    );
    match coordinate {
        Ok(value) => assert_eq!(value.ticks(), observed, "exact coordinate ticks retained"),
        Err(error) => assert_eq!(
            error,
            DomainError::CoordinateOutOfRange,
            "outside coordinate error classification"
        ),
    }
    (observed, admitted)
}

#[kani::proof]
fn leap_ticks_full_domain() {
    let seconds = kani::any::<u64>();
    let (ticks, admitted) = leap_ticks_contract(seconds);
    kani::cover!(seconds == 0 && ticks == 0 && admitted, "zero whole second is covered");
    kani::cover!(
        seconds == 1 && ticks == 4_294_967_296 && admitted,
        "one whole second retains Q32 units"
    );
    kani::cover!(seconds == 2_272_060_800 && admitted, "leap baseline scalar is covered");
    kani::cover!(
        seconds == 4_294_967_296 && ticks == 18_446_744_073_709_551_616 && admitted,
        "post-u32 second bits are covered"
    );
    kani::cover!(
        seconds == MAX_ADMITTED_WHOLE_SECOND && admitted,
        "signed-i64 whole-second endpoint is covered"
    );
    kani::cover!(
        seconds == 9_223_372_036_854_775_808 && !admitted,
        "first outside whole-second neighbor is covered"
    );
    kani::cover!(seconds == u64::MAX && !admitted, "raw u64 maximum remains represented");
}

/// Returning zero loses an ordinary, valid nonzero second.
#[kani::proof]
fn leap_ticks_control_nonzero() {
    assert_eq!(leap_ticks::proof_second_ticks(1), 4_294_967_296, "nonzero second retained");
}

/// A half-scale fault manufactures fractional bits from a whole-second input.
#[kani::proof]
fn leap_ticks_control_scale() {
    assert_eq!(
        leap_ticks::proof_second_ticks(1) as u128 & 0xffff_ffff,
        0,
        "whole-second fractional alignment"
    );
}

/// Truncation at the wire width loses a valid unfolded second, not just a raw outlier.
#[kani::proof]
fn leap_ticks_control_high_bits() {
    assert_eq!(
        leap_ticks::proof_second_ticks(4_294_967_296) as u128 >> 32,
        4_294_967_296,
        "high second bits retained"
    );
}
