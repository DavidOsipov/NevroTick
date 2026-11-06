// Proof-only independent contracts and unrestricted local return generators.
// Product bodies and their runtime validation remain in selected source spans.

/// All raw factory inputs admitted by the mathematical representation contract.
fn constructor_admitted(numerator: u64, shift: u32) -> bool {
    let bits = 64 - numerator.leading_zeros();
    shift <= 1041 && bits <= 53 && (numerator == 0 || bits <= shift)
}

/// Canonical checked objects, including the unique zero representation.
fn checked_fraction(value: Fraction) -> bool {
    let bits = 64 - value.numerator.leading_zeros();
    value.shift <= 1041
        && bits <= 53
        && if value.numerator == 0 {
            value.shift == 0
        } else {
            value.shift >= 1 && bits <= value.shift
        }
}

/// Complete success, exact representation, invariant and error classification.
fn constructor_post(
    numerator: u64,
    shift: u32,
    result: &Result<Fraction, FixtureEnclosureError>,
) -> bool {
    let admitted = constructor_admitted(numerator, shift);
    match result {
        Ok(value) => {
            admitted
                && checked_fraction(*value)
                && value.numerator == numerator
                && value.shift == if numerator == 0 { 0 } else { shift }
        }
        Err(FixtureEnclosureError::Invariant) => !admitted,
        Err(_) => false,
    }
}

/// Independent half threshold; every operation is total even on raw fields.
fn complement_post(
    input: Fraction,
    result: &Result<Fraction, FixtureEnclosureError>,
) -> bool {
    let denominator = if 1 <= input.shift && input.shift <= 53 {
        Some(1_u128 << input.shift)
    } else {
        None
    };
    let required_success = match denominator {
        Some(value) => 2 * u128::from(input.numerator) >= value,
        None => false,
    };
    match result {
        Ok(value) => match denominator {
            Some(denominator) => {
                required_success
                    && checked_fraction(*value)
                    && value.shift == input.shift
                    && u128::from(value.numerator) + u128::from(input.numerator)
                        == denominator
                    && 1 <= value.numerator
                    && value.numerator <= 4_503_599_627_370_496
            }
            None => false,
        },
        Err(FixtureEnclosureError::Invariant) => !required_success,
        Err(_) => false,
    }
}

// These generators deliberately enumerate raw fields and every error variant.
// They never call constructors, use assumptions or filter invalid objects.
impl kani::Arbitrary for Fraction {
    fn any() -> Self {
        Self {
            numerator: kani::any::<u64>(),
            shift: kani::any::<u32>(),
        }
    }
}

impl kani::Arbitrary for IntervalError {
    fn any() -> Self {
        match kani::any::<u8>() {
            0 => Self::ZeroLabel,
            1 => Self::ReversedBounds,
            _ => Self::ValidationInvariant,
        }
    }
}

impl kani::Arbitrary for FixtureEnclosureError {
    fn any() -> Self {
        match kani::any::<u8>() {
            0 => Self::Unrepresentable,
            1 => Self::Invariant,
            _ => Self::Interval {
                source: kani::any::<IntervalError>(),
            },
        }
    }
}
