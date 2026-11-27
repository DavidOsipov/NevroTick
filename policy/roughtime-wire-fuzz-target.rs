// SPDX-License-Identifier: GPL-2.0-only
//! Offline libFuzzer target for the production inert Roughtime wire decoders.
#![no_main]

use libfuzzer_sys::fuzz_target;

// The admitted launcher places the shared test oracle beside this target.
mod hostile;

fuzz_target!(|input: &[u8]| {
    hostile::check_untrusted_frame(input);
    // A separate build exercises the observer on an exact, valid request seed.
    #[cfg(feature = "seeded-defect")]
    if ntp_wire::roughtime::decode_request(input).is_ok() {
        assert!(input.is_empty(), "accepted Roughtime seed reached the fuzz target");
    }
});
