// Full raw-input theorem for the actual supporter_labels caller and reached sort.
// Every oracle census has sixteen fixed iterations, independent of raw count.
const _: () = assert!(usize::BITS == 64);

#[kani::proof]
#[kani::unwind(17)]
fn supporter_labels_full_domain() {
    let raw = [
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
        kani::any::<u64>(),
    ];
    let staged = [
        NonZeroU64::new(raw[0]),
        NonZeroU64::new(raw[1]),
        NonZeroU64::new(raw[2]),
        NonZeroU64::new(raw[3]),
        NonZeroU64::new(raw[4]),
        NonZeroU64::new(raw[5]),
        NonZeroU64::new(raw[6]),
        NonZeroU64::new(raw[7]),
        NonZeroU64::new(raw[8]),
        NonZeroU64::new(raw[9]),
        NonZeroU64::new(raw[10]),
        NonZeroU64::new(raw[11]),
        NonZeroU64::new(raw[12]),
        NonZeroU64::new(raw[13]),
        NonZeroU64::new(raw[14]),
        NonZeroU64::new(raw[15]),
    ];
    let count = kani::any::<usize>();
    let probe_label = kani::any::<u64>();
    let probe_index = kani::any::<usize>();
    let observed = supporter_labels(&staged, count);
    let mut expected_success = count >= 1 && count <= 16;
    let mut complete_frame = true;
    let mut descending = true;
    let mut all_maximum = true;
    for index in 0..16 {
        if index < count && raw[index] == 0 {
            expected_success = false;
        }
        complete_frame &= staged[index].map_or(0, NonZeroU64::get) == raw[index];
        all_maximum &= raw[index] == u64::MAX;
        if index < 15 {
            descending &= raw[index] > raw[index + 1];
        }
    }
    assert!(
        observed.is_ok() == expected_success,
        "supporter success iff prefix contract"
    );
    assert!(complete_frame, "supporter complete input frame");
    match observed {
        Ok(labels) => {
            // These bounds protect every operation even if admission were wrong.
            if probe_index < 15 && probe_index + 1 < count {
                assert!(
                    labels[probe_index] <= labels[probe_index + 1],
                    "supporter prefix ordered"
                );
            }
            let mut original_occurrences = 0_u8;
            let mut returned_occurrences = 0_u8;
            for index in 0..16 {
                if index < count {
                    if raw[index] == probe_label {
                        original_occurrences += 1;
                    }
                    if labels[index].get() == probe_label {
                        returned_occurrences += 1;
                    }
                }
            }
            assert!(
                original_occurrences == returned_occurrences,
                "supporter prefix multiplicity"
            );
            if probe_index < 16 && probe_index >= count {
                assert!(
                    labels[probe_index].get() == raw[0],
                    "supporter first-label tail"
                );
            }
        }
        Err(error) => {
            assert!(
                matches!(
                    error,
                    SelectionError::Invariant {
                        stage: SelectionInvariantStage::Support
                    }
                ),
                "supporter exact support error"
            );
        }
    }
    kani::cover!(
        count == 1 && observed.is_ok(),
        "supporter one label succeeds"
    );
    kani::cover!(
        count == 16 && observed.is_ok(),
        "supporter sixteen labels succeed"
    );
    kani::cover!(
        count == 0 && observed.is_err(),
        "supporter zero count rejects"
    );
    kani::cover!(
        count == 17 && observed.is_err(),
        "supporter seventeen count rejects"
    );
    kani::cover!(
        count == usize::MAX && observed.is_err(),
        "supporter maximum count rejects"
    );
    kani::cover!(
        count == 1 && raw[0] == 0 && observed.is_err(),
        "supporter missing first rejects"
    );
    kani::cover!(
        count == 3 && raw[0] != 0 && raw[1] != 0 && raw[2] == 0 && observed.is_err(),
        "supporter missing interior rejects"
    );
    kani::cover!(
        count == 2 && raw[0] != 0 && raw[0] == raw[1] && observed.is_ok(),
        "supporter duplicate prefix succeeds"
    );
    kani::cover!(
        count == 16 && descending && observed.is_ok(),
        "supporter descending sixteen succeeds"
    );
    kani::cover!(
        count == 16 && all_maximum && observed.is_ok(),
        "supporter maximum labels succeed"
    );
    kani::cover!(
        count == 1 && raw[15] == 0 && observed.is_ok(),
        "supporter irrelevant None tail succeeds"
    );
    kani::cover!(
        count == 1 && raw[15] != 0 && raw[15] != raw[0] && observed.is_ok(),
        "supporter irrelevant Some tail succeeds"
    );
}

#[kani::proof]
#[kani::unwind(17)]
fn supporter_labels_control_empty_sort() {
    let mut staged = [None; 16];
    staged[0] = NonZeroU64::new(2);
    staged[1] = NonZeroU64::new(1);
    let ordered = match supporter_labels(&staged, 2) {
        Ok(labels) => labels[0] <= labels[1],
        Err(_) => false,
    };
    assert!(ordered, "supporter prefix ordered");
}

#[kani::proof]
#[kani::unwind(17)]
fn supporter_labels_control_multiplicity() {
    let mut staged = [None; 16];
    staged[0] = NonZeroU64::new(1);
    staged[1] = NonZeroU64::new(2);
    let retained = match supporter_labels(&staged, 2) {
        Ok(labels) => {
            let mut occurrences = 0_u8;
            for index in 0..16 {
                if index < 2 && labels[index].get() == 1 {
                    occurrences += 1;
                }
            }
            occurrences == 1
        }
        Err(_) => false,
    };
    assert!(retained, "supporter prefix multiplicity");
}

#[kani::proof]
#[kani::unwind(17)]
fn supporter_labels_control_wrong_tail() {
    let mut staged = [None; 16];
    staged[0] = NonZeroU64::new(1);
    let retained = match supporter_labels(&staged, 1) {
        Ok(labels) => labels[1].get() == 1,
        Err(_) => false,
    };
    assert!(retained, "supporter first-label tail");
}
