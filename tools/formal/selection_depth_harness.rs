// Full raw-state contract for the exact private append-depth helper.
// Prefix validity, interval depth truth and selection topology remain caller work.

const _: () = assert!(usize::BITS == 64);

/// Check the complete transition without assuming a valid initialized prefix.
fn append_depth_contract(
    values: [usize; 63],
    len: usize,
    maximum: usize,
    depth: usize,
    probe: usize,
) -> Result<(), SelectionError> {
    let mut cells = CellDepths {
        values,
        len,
        maximum,
    };
    let observed = append_depth(&mut cells, depth);
    if len < 63 {
        assert_eq!(observed, Ok(()), "append required success");
        assert_eq!(cells.values[len], depth, "append exact selected slot");
        assert_eq!(
            cells.len as u128,
            (len as u128) + 1,
            "append exact mathematical length"
        );
        let expected_maximum = if depth > maximum { depth } else { maximum };
        assert_eq!(
            cells.maximum, expected_maximum,
            "append independent maximum"
        );
        if probe < 63 && probe != len {
            assert_eq!(
                cells.values[probe], values[probe],
                "append success probe unchanged"
            );
        }
    } else {
        assert_eq!(
            observed,
            Err(SelectionError::Invariant {
                stage: SelectionInvariantStage::Slot,
            }),
            "append exact slot error"
        );
        assert_eq!(cells.len, len, "append error length unchanged");
        assert_eq!(cells.maximum, maximum, "append error maximum unchanged");
        if probe < 63 {
            assert_eq!(
                cells.values[probe], values[probe],
                "append error probe unchanged"
            );
        }
    }
    observed
}

#[kani::proof]
fn selection_depth_full_domain() {
    // Each primitive call is independent; no array generator needs unwinding.
    let values = [
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
        kani::any::<usize>(),
    ];
    let len = kani::any::<usize>();
    let maximum = kani::any::<usize>();
    let depth = kani::any::<usize>();
    let probe = kani::any::<usize>();
    let observed = append_depth_contract(values, len, maximum, depth, probe);
    let slot_error = Err(SelectionError::Invariant {
        stage: SelectionInvariantStage::Slot,
    });
    kani::cover!(len == 0 && observed.is_ok(), "append first slot succeeds");
    kani::cover!(len == 62 && observed.is_ok(), "append final slot succeeds");
    kani::cover!(
        len == 0 && depth == usize::MAX && maximum == 0 && observed.is_ok(),
        "append maximum grows to raw maximum"
    );
    kani::cover!(
        len == 0 && depth == 0 && maximum == usize::MAX && observed.is_ok(),
        "append maximum retains raw maximum"
    );
    kani::cover!(
        len == 0 && maximum == depth && observed.is_ok(),
        "append equal maximum succeeds"
    );
    kani::cover!(
        len == 63 && observed == slot_error,
        "append first unavailable slot rejects"
    );
    kani::cover!(
        len == usize::MAX && observed == slot_error,
        "append raw maximum length rejects"
    );
    kani::cover!(
        len == 1 && probe == 0 && values[0] != depth && observed.is_ok(),
        "append nonselected earlier slot retained"
    );
    kani::cover!(
        len == 1 && probe == 62 && values[62] != depth && observed.is_ok(),
        "append nonselected later slot retained"
    );
    kani::cover!(
        len == 63 && probe == 0 && values[0] != depth && observed == slot_error,
        "append rejection retains distinct probe value"
    );
}

#[kani::proof]
fn selection_depth_control_wrong_slot() {
    let mut cells = CellDepths {
        values: [7; 63],
        len: usize::MAX,
        maximum: 0,
    };
    let _observed = append_depth(&mut cells, 3);
    assert_eq!(cells.values[0], 7, "append error probe unchanged");
}

#[kani::proof]
fn selection_depth_control_wrong_length() {
    let mut cells = CellDepths {
        values: [7; 63],
        len: 0,
        maximum: 0,
    };
    let _observed = append_depth(&mut cells, 3);
    assert_eq!(
        cells.len as u128, 1_u128,
        "append exact mathematical length"
    );
}

#[kani::proof]
fn selection_depth_control_wrong_maximum() {
    let mut cells = CellDepths {
        values: [7; 63],
        len: 0,
        maximum: 2,
    };
    let _observed = append_depth(&mut cells, 3);
    assert_eq!(cells.maximum, 3, "append independent maximum");
}

#[kani::proof]
fn selection_depth_control_wrong_error() {
    let mut cells = CellDepths {
        values: [7; 63],
        len: 63,
        maximum: 2,
    };
    assert_eq!(
        append_depth(&mut cells, 3),
        Err(SelectionError::Invariant {
            stage: SelectionInvariantStage::Slot,
        }),
        "append exact slot error"
    );
}
