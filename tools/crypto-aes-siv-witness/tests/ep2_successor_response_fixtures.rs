// SPDX-License-Identifier: GPL-2.0-only
//! Distinct public successor fixtures; execution requires exact isolated admission.
//! These literal keys and packets establish no TLS/exporter/native/entropy provenance.

#[path = "common/ep2_response_builder.rs"]
mod builder;

use builder::{FixtureResult, emit, field, packet_with_key_nonce};

/// Legacy public response key for the valid-old-key companion.
const LEGACY_S2C: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
];

/// Generate the distinct successor, count-refusal and valid-old-key material.
/// Existing generator inputs and emitted constants remain unchanged.
/// # Errors
/// Preserve fixed-key encryption or bounded fixture length failures.
/// # Panics
/// Fail if the independently declared healthy packet length changes.
#[test]
fn generate_ep2_successor_response_fixtures() -> FixtureResult {
    // These literal public values are not exporter or entropy evidence.
    let s2c = [
        0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce,
        0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, 0xdb, 0xdc, 0xdd,
        0xde, 0xdf,
    ];
    let uid = [
        0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28, 0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e,
        0x2f, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d,
        0x3e, 0x3f,
    ];
    let nonce = [
        0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76, 0x77, 0x78, 0x79, 0x7a, 0x7b, 0x7c, 0x7d, 0x7e,
        0x7f,
    ];
    let cookie = [
        0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb,
    ];
    let mut header = [0_u8; 48];
    header[0] = 0x24;
    header[1] = 2;
    header[24..32].copy_from_slice(&0x8899_aabb_ccdd_eeff_u64.to_be_bytes());
    header[32..40].copy_from_slice(&0x4455_6677_8899_aabb_u64.to_be_bytes());
    header[40..48].copy_from_slice(&0x5566_7788_99aa_bbcc_u64.to_be_bytes());
    let uid = field(0x0104, &uid)?;
    let cookie = field(0x0204, &cookie)?;
    let cases = [
        (
            "SUCCESSOR_HEALTHY",
            packet_with_key_nonce(&header, &uid, &cookie, &[], &nonce, &s2c)?,
        ),
        (
            "SUCCESSOR_OLD_S2C",
            packet_with_key_nonce(&header, &uid, &cookie, &[], &nonce, &LEGACY_S2C)?,
        ),
        (
            "SUCCESSOR_ZERO_COOKIES",
            packet_with_key_nonce(&header, &uid, &[], &[], &nonce, &s2c)?,
        ),
    ];
    assert_eq!(cases[0].1.len(), 140);
    assert_eq!(cases[1].1.len(), 140);
    assert_eq!(cases[2].1.len(), 124);
    println!("// SPDX-License-Identifier: GPL-2.0-only");
    println!(
        "//! Public successor component fixtures generated through the existing maintained AES-SIV API."
    );
    println!(
        "//! No TLS/exporter/native/entropy or production authentication provenance is claimed."
    );
    for (name, bytes) in cases {
        emit(name, &bytes);
    }
    Ok(())
}
