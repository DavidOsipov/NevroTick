// Complete actual RequestTiming admission over two independent raw u64 readings.
// The signed-i128 difference and literal ceiling are independent of product code.
// Supplied clock-origin/generation truth and production policy remain separate.

/// Check every outcome through real clock construction and timing admission.
fn request_timing_contract(send: u64, deadline: u64) -> Result<RequestTiming, TimingError> {
    let send_reading = MonotonicNs::new(send);
    let deadline_reading = MonotonicNs::new(deadline);
    assert_eq!(send_reading.nanoseconds(), send, "timing typed send exact");
    assert_eq!(
        deadline_reading.nanoseconds(),
        deadline,
        "timing typed deadline exact"
    );
    let difference = i128::from(deadline) - i128::from(send);
    let required_success = difference > 0 && difference <= 1_000_000_000_i128;
    let observed = RequestTiming::new(send_reading, deadline_reading);
    assert_eq!(
        observed.is_ok(),
        required_success,
        "timing success iff contract"
    );
    assert_eq!(
        matches!(observed, Err(TimingError::NonPositiveLifetime)),
        difference <= 0,
        "timing nonpositive classification"
    );
    assert_eq!(
        matches!(observed, Err(TimingError::LifetimeExceeded)),
        difference > 1_000_000_000_i128,
        "timing excess classification"
    );
    if let Ok(value) = observed {
        assert_eq!(
            value.send().nanoseconds(),
            send,
            "timing exact send retained"
        );
        assert_eq!(
            value.deadline().nanoseconds(),
            deadline,
            "timing exact deadline retained"
        );
        assert_eq!(
            i128::from(value.deadline().nanoseconds()) - i128::from(value.send().nanoseconds()),
            difference,
            "timing exact returned lifetime"
        );
    }
    observed
}

#[kani::proof]
fn request_timing_full_domain() {
    let send = kani::any::<u64>();
    let deadline = kani::any::<u64>();
    let observed = request_timing_contract(send, deadline);
    kani::cover!(
        send == 0 && deadline == 1 && observed.is_ok(),
        "timing one tick succeeds"
    );
    kani::cover!(
        send == 0 && deadline == 0 && observed == Err(TimingError::NonPositiveLifetime),
        "timing equal endpoints reject"
    );
    kani::cover!(
        send == 1 && deadline == 0 && observed == Err(TimingError::NonPositiveLifetime),
        "timing reversed neighbors reject"
    );
    kani::cover!(
        send == 0 && deadline == 1_000_000_000 && observed.is_ok(),
        "timing exact ceiling succeeds"
    );
    kani::cover!(
        send == 0 && deadline == 1_000_000_001 && observed == Err(TimingError::LifetimeExceeded),
        "timing first excess rejects"
    );
    kani::cover!(
        send == u64::MAX - 1 && deadline == u64::MAX && observed.is_ok(),
        "timing near maximum succeeds"
    );
    kani::cover!(
        send == 0 && deadline == u64::MAX && observed == Err(TimingError::LifetimeExceeded),
        "timing full positive span rejects"
    );
    kani::cover!(
        send == u64::MAX && deadline == 0 && observed == Err(TimingError::NonPositiveLifetime),
        "timing reversed raw extremes reject"
    );
}

#[kani::proof]
fn request_timing_control_zero_lifetime() {
    assert_eq!(
        RequestTiming::new(MonotonicNs::new(5), MonotonicNs::new(5)),
        Err(TimingError::NonPositiveLifetime),
        "timing nonpositive classification"
    );
}

#[kani::proof]
fn request_timing_control_excess_lifetime() {
    assert_eq!(
        RequestTiming::new(MonotonicNs::new(0), MonotonicNs::new(1_000_000_001)),
        Err(TimingError::LifetimeExceeded),
        "timing excess classification"
    );
}

#[kani::proof]
fn request_timing_control_retained_send() {
    assert_eq!(
        RequestTiming::new(MonotonicNs::new(5), MonotonicNs::new(6))
            .map(|value| value.send().nanoseconds()),
        Ok(5),
        "timing exact send retained"
    );
}
