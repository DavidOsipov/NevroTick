// Proof-only total relations and primitive seeds; never compiled into the product.
// No constructor, filtering, assume, custom Arbitrary or foreign trait is used here.

fn composition_constructor_admitted(numerator: u64, shift: u32) -> bool {
    let bits = 64 - numerator.leading_zeros();
    shift <= 1041 && bits <= 53 && (numerator == 0 || bits <= shift)
}

fn composition_checked(value: Fraction) -> bool {
    let bits = 64 - value.numerator.leading_zeros();
    value.shift <= 1041
        && bits <= 53
        && if value.numerator == 0 {
            value.shift == 0
        } else {
            value.shift >= 1 && bits <= value.shift
        }
}

fn composition_constructor_post(
    numerator: u64,
    shift: u32,
    result: &Result<Fraction, FixtureEnclosureError>,
) -> bool {
    let admitted = composition_constructor_admitted(numerator, shift);
    match result {
        Ok(value) => {
            admitted
                && composition_checked(*value)
                && value.numerator == numerator
                && value.shift == if numerator == 0 { 0 } else { shift }
        }
        Err(FixtureEnclosureError::Invariant) => !admitted,
        Err(_) => false,
    }
}

// Total even on arbitrary raw inputs: checkedness is tested before this oracle.
// Its guarded cross scaling differs from the executable leading-power algorithm.
fn composition_rational_order(left: Fraction, right: Fraction) -> Ordering {
    if !composition_checked(left) || !composition_checked(right) {
        return Ordering::Equal;
    }
    if left.numerator == 0 {
        return if right.numerator == 0 {
            Ordering::Equal
        } else {
            Ordering::Less
        };
    }
    if right.numerator == 0 {
        return Ordering::Greater;
    }
    if left.shift == right.shift {
        return left.numerator.cmp(&right.numerator);
    }
    if left.shift < right.shift {
        let gap = right.shift - left.shift;
        if gap < 64 {
            (u128::from(left.numerator) << gap).cmp(&u128::from(right.numerator))
        } else {
            Ordering::Greater
        }
    } else {
        let gap = left.shift - right.shift;
        if gap < 64 {
            u128::from(left.numerator).cmp(&(u128::from(right.numerator) << gap))
        } else {
            Ordering::Less
        }
    }
}

fn composition_comparison_post(
    left: Fraction,
    right: Fraction,
    result: &Result<Ordering, FixtureEnclosureError>,
) -> bool {
    composition_checked(left)
        && composition_checked(right)
        && match result {
            Ok(order) => *order == composition_rational_order(left, right),
            Err(_) => false,
        }
}

fn composition_complement_post(
    input: Fraction,
    result: &Result<Fraction, FixtureEnclosureError>,
) -> bool {
    if !composition_checked(input) {
        return false;
    }
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
                    && composition_checked(*value)
                    && value.shift == input.shift
                    && u128::from(value.numerator) + u128::from(input.numerator) == denominator
                    && 1 <= value.numerator
                    && value.numerator <= 4_503_599_627_370_496
            }
            None => false,
        },
        Err(FixtureEnclosureError::Invariant) => !required_success,
        Err(_) => false,
    }
}

// Six flattened alternatives: raw Ok fields plus all five error leaves.
#[derive(Clone, Copy)]
struct CompositionFractionSeed {
    tag: u8,
    numerator: u64,
    shift: u32,
}

fn composition_decode_fraction(
    seed: CompositionFractionSeed,
) -> Result<Fraction, FixtureEnclosureError> {
    match seed.tag {
        0 => Ok(Fraction {
            numerator: seed.numerator,
            shift: seed.shift,
        }),
        1 => Err(FixtureEnclosureError::Unrepresentable),
        2 => Err(FixtureEnclosureError::Invariant),
        3 => Err(FixtureEnclosureError::Interval {
            source: IntervalError::ZeroLabel,
        }),
        4 => Err(FixtureEnclosureError::Interval {
            source: IntervalError::ReversedBounds,
        }),
        _ => Err(FixtureEnclosureError::Interval {
            source: IntervalError::ValidationInvariant,
        }),
    }
}

fn composition_encode_fraction(
    result: Result<Fraction, FixtureEnclosureError>,
) -> CompositionFractionSeed {
    match result {
        Ok(value) => CompositionFractionSeed {
            tag: 0,
            numerator: value.numerator,
            shift: value.shift,
        },
        Err(FixtureEnclosureError::Unrepresentable) => CompositionFractionSeed {
            tag: 1,
            numerator: 0,
            shift: 0,
        },
        Err(FixtureEnclosureError::Invariant) => CompositionFractionSeed {
            tag: 2,
            numerator: 0,
            shift: 0,
        },
        Err(FixtureEnclosureError::Interval {
            source: IntervalError::ZeroLabel,
        }) => CompositionFractionSeed {
            tag: 3,
            numerator: 0,
            shift: 0,
        },
        Err(FixtureEnclosureError::Interval {
            source: IntervalError::ReversedBounds,
        }) => CompositionFractionSeed {
            tag: 4,
            numerator: 0,
            shift: 0,
        },
        Err(FixtureEnclosureError::Interval {
            source: IntervalError::ValidationInvariant,
        }) => CompositionFractionSeed {
            tag: 5,
            numerator: 0,
            shift: 0,
        },
    }
}

// Eight flattened alternatives retain the native Ordering and every error leaf.
fn composition_decode_order(tag: u8) -> Result<Ordering, FixtureEnclosureError> {
    match tag {
        0 => Ok(Ordering::Less),
        1 => Ok(Ordering::Equal),
        2 => Ok(Ordering::Greater),
        3 => Err(FixtureEnclosureError::Unrepresentable),
        4 => Err(FixtureEnclosureError::Invariant),
        5 => Err(FixtureEnclosureError::Interval {
            source: IntervalError::ZeroLabel,
        }),
        6 => Err(FixtureEnclosureError::Interval {
            source: IntervalError::ReversedBounds,
        }),
        _ => Err(FixtureEnclosureError::Interval {
            source: IntervalError::ValidationInvariant,
        }),
    }
}

fn composition_encode_order(result: Result<Ordering, FixtureEnclosureError>) -> u8 {
    match result {
        Ok(Ordering::Less) => 0,
        Ok(Ordering::Equal) => 1,
        Ok(Ordering::Greater) => 2,
        Err(FixtureEnclosureError::Unrepresentable) => 3,
        Err(FixtureEnclosureError::Invariant) => 4,
        Err(FixtureEnclosureError::Interval {
            source: IntervalError::ZeroLabel,
        }) => 5,
        Err(FixtureEnclosureError::Interval {
            source: IntervalError::ReversedBounds,
        }) => 6,
        Err(FixtureEnclosureError::Interval {
            source: IntervalError::ValidationInvariant,
        }) => 7,
    }
}

fn composition_error_equal(left: FixtureEnclosureError, right: FixtureEnclosureError) -> bool {
    match (left, right) {
        (FixtureEnclosureError::Unrepresentable, FixtureEnclosureError::Unrepresentable)
        | (FixtureEnclosureError::Invariant, FixtureEnclosureError::Invariant) => true,
        (
            FixtureEnclosureError::Interval { source: left },
            FixtureEnclosureError::Interval { source: right },
        ) => match (left, right) {
            (IntervalError::ZeroLabel, IntervalError::ZeroLabel)
            | (IntervalError::ReversedBounds, IntervalError::ReversedBounds)
            | (IntervalError::ValidationInvariant, IntervalError::ValidationInvariant) => true,
            _ => false,
        },
        _ => false,
    }
}

fn composition_fraction_result_equal(
    left: Result<Fraction, FixtureEnclosureError>,
    right: Result<Fraction, FixtureEnclosureError>,
) -> bool {
    match (left, right) {
        (Ok(left), Ok(right)) => left.numerator == right.numerator && left.shift == right.shift,
        (Err(left), Err(right)) => composition_error_equal(left, right),
        _ => false,
    }
}

fn composition_order_result_equal(
    left: Result<Ordering, FixtureEnclosureError>,
    right: Result<Ordering, FixtureEnclosureError>,
) -> bool {
    match (left, right) {
        (Ok(left), Ok(right)) => match (left, right) {
            (Ordering::Less, Ordering::Less)
            | (Ordering::Equal, Ordering::Equal)
            | (Ordering::Greater, Ordering::Greater) => true,
            _ => false,
        },
        (Err(left), Err(right)) => composition_error_equal(left, right),
        _ => false,
    }
}
