// SPDX-License-Identifier: GPL-2.0-only
//! Bounded public fixture generator; execution requires the admitted fixture lane.
//! Run as an integration test of the existing AES witness, using its current
//! aes_siv dependency. Output is public literal fixtures, never TLS evidence.

use aes_siv::{KeyInit, siv::Aes128Siv};

/// Bounded fixture construction failures, never cryptographic material.
type FixtureResult<T = ()> = Result<T, &'static str>;
/// Public response key; differs from the public C2S key in both halves.
const S2C: [u8; 32] = [
    0x40, 0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f,
    0x50, 0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x5b, 0x5c, 0x5d, 0x5e, 0x5f,
];
/// Public exact request UID, not random or supplied by an authenticated session.
const UID: [u8; 32] = [
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31,
];
/// Public fixed response nonce, carrying no production entropy guarantee.
const NONCE: [u8; 16] = [
    0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f,
];
/// Independent literal origin echo, separate from any real local T1 coordinate.
const ORIGIN: u64 = 0x1122_3344_5566_7788;
/// Independent literal receive coordinate, with no era or accuracy claim.
const RECEIVE: u64 = 0x2233_4455_6677_8899;
/// Independent literal transmit coordinate, distinct from origin and receive.
const TRANSMIT: u64 = 0x3344_5566_7788_99aa;
/// Exact opaque twelve-byte public replacement cookie.
const COOKIE: [u8; 12] = [
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab,
];

/// Build literal aligned fields independently of the product wire owner.
/// # Errors
/// Refuse an unrepresentable fixture length rather than truncate its prefix.
fn field(kind: u16, body: &[u8]) -> FixtureResult<Vec<u8>> {
    let length = u16::try_from(body.len() + 4).map_err(|_| "fixture-field-length")?;
    let mut bytes = kind.to_be_bytes().to_vec();
    bytes.extend_from_slice(&length.to_be_bytes());
    bytes.extend_from_slice(body);
    Ok(bytes)
}

/// Build a valid-tag packet independently of all product framing/sealing APIs.
/// # Errors
/// Preserve fixed-key encryption or bounded fixture length failures.
fn packet(
    header: &[u8; 48],
    prefix: &[u8],
    plaintext: &[u8],
    suffix: &[u8],
) -> FixtureResult<Vec<u8>> {
    packet_with_nonce(header, prefix, plaintext, suffix, &NONCE)
}

/// Preserve the independent layout while varying only the selected-AEAD nonce.
/// # Errors
/// Refuse unrepresentable lengths or a maintained encryption failure.
fn packet_with_nonce(
    header: &[u8; 48],
    prefix: &[u8],
    plaintext: &[u8],
    suffix: &[u8],
    nonce: &[u8],
) -> FixtureResult<Vec<u8>> {
    let mut aad = header.to_vec();
    aad.extend_from_slice(prefix);
    let mut body = plaintext.to_vec();
    let mut cipher = Aes128Siv::new_from_slice(&S2C).map_err(|_| "fixture-key")?;
    let tag = cipher
        .encrypt_inout_detached([aad.as_slice(), nonce], (&mut body[..]).into())
        .map_err(|_| "fixture-encryption")?;
    let ciphertext_length = u16::try_from(16 + body.len()).map_err(|_| "fixture-ciphertext")?;
    let padded_length = (usize::from(ciphertext_length) + 3) & !3;
    let padded_nonce = (nonce.len() + 3) & !3;
    let nonce_length = u16::try_from(nonce.len()).map_err(|_| "fixture-nonce")?;
    let authenticator_length =
        u16::try_from(8 + padded_nonce + padded_length).map_err(|_| "fixture-authenticator")?;
    let mut output = aad;
    output.extend_from_slice(&[4, 4]);
    output.extend_from_slice(&authenticator_length.to_be_bytes());
    output.extend_from_slice(&nonce_length.to_be_bytes());
    output.extend_from_slice(&ciphertext_length.to_be_bytes());
    output.extend_from_slice(nonce);
    output.resize(output.len() + padded_nonce - nonce.len(), 0);
    output.extend_from_slice(tag.as_slice());
    output.extend_from_slice(&body);
    output.resize(
        output.len() + padded_length - usize::from(ciphertext_length),
        0,
    );
    output.extend_from_slice(suffix);
    Ok(output)
}

/// Emit public literal Rust source only after actual maintained encryption.
fn emit(name: &str, bytes: &[u8]) {
    println!("/// Public independently generated valid-tag fixture: {name}.");
    println!("pub(in crate::nts_ke_experiment) const {name}: &[u8] = &[");
    for chunk in bytes.chunks(16) {
        print!("    ");
        for byte in chunk {
            print!("0x{byte:02x}, ");
        }
        println!();
    }
    println!("];\n");
}

/// Produce the complete bounded semantic-control fixture inventory.
/// # Errors
/// Fail generation rather than inventing tags or emitting a partially valid row.
#[test]
fn generate_ep2_response_fixtures() -> FixtureResult {
    let mut header = [0_u8; 48];
    header[0] = 0x24;
    header[1] = 2;
    header[24..32].copy_from_slice(&ORIGIN.to_be_bytes());
    header[32..40].copy_from_slice(&RECEIVE.to_be_bytes());
    header[40..48].copy_from_slice(&TRANSMIT.to_be_bytes());
    let uid = field(0x0104, &UID)?;
    let cookie = field(0x0204, &COOKIE)?;
    let unknown_outer = field(0x1234, &[0x55; 12])?;
    let unknown_inner = field(0x1234, &[])?;
    let suffix = field(0x0204, &[0x99; 24])?;
    let mut cases = vec![
        ("HEALTHY", packet(&header, &uid, &cookie, &[])?),
        (
            "NONCE_ZERO",
            packet_with_nonce(&header, &uid, &cookie, &[], &[])?,
        ),
        (
            "NONCE_ONE",
            packet_with_nonce(&header, &uid, &cookie, &[], &[0x10])?,
        ),
        (
            "NONCE_TWELVE",
            packet_with_nonce(&header, &uid, &cookie, &[], &NONCE[..12])?,
        ),
        ("ZERO_COOKIES", packet(&header, &uid, &[], &[])?),
        (
            "EMPTY_COOKIE",
            packet(&header, &uid, &field(0x0204, &[])?, &[])?,
        ),
        (
            "TWO_COOKIES",
            packet(&header, &uid, &cookie.repeat(2), &[])?,
        ),
        (
            "EIGHT_COOKIES",
            packet(&header, &uid, &cookie.repeat(8), &[])?,
        ),
        (
            "NINE_COOKIES",
            packet(&header, &uid, &cookie.repeat(9), &[])?,
        ),
        ("MISSING_UID", packet(&header, &[], &cookie, &[])?),
        (
            "SHORT_UID",
            packet(&header, &field(0x0104, &[0x66; 12])?, &cookie, &[])?,
        ),
        (
            "DUPLICATE_UID",
            packet(&header, &uid.repeat(2), &cookie, &[])?,
        ),
        (
            "CLEAR_COOKIE",
            packet(
                &header,
                &[uid.as_slice(), cookie.as_slice()].concat(),
                &cookie,
                &[],
            )?,
        ),
        (
            "CLEAR_PLACEHOLDER",
            packet(
                &header,
                &[uid.as_slice(), field(0x0304, &[0; 12])?.as_slice()].concat(),
                &cookie,
                &[],
            )?,
        ),
        (
            "INNER_UID",
            packet(
                &header,
                &uid,
                &[cookie.as_slice(), uid.as_slice()].concat(),
                &[],
            )?,
        ),
        (
            "INNER_AUTH",
            packet(
                &header,
                &uid,
                &[cookie.as_slice(), field(0x0404, &[])?.as_slice()].concat(),
                &[],
            )?,
        ),
        (
            "INNER_PLACEHOLDER",
            packet(
                &header,
                &uid,
                &[cookie.as_slice(), field(0x0304, &[0; 12])?.as_slice()].concat(),
                &[],
            )?,
        ),
        (
            "UNKNOWN_FIELDS_AND_SUFFIX",
            packet(
                &header,
                &[uid.as_slice(), unknown_outer.as_slice()].concat(),
                &[cookie.as_slice(), unknown_inner.as_slice()].concat(),
                &suffix,
            )?,
        ),
        (
            "SUFFIX_DUPLICATE_UID_AUTH",
            packet(
                &header,
                &uid,
                &cookie,
                &[uid.as_slice(), field(0x0404, &[0xff; 24])?.as_slice()].concat(),
            )?,
        ),
        (
            "MALFORMED_INNER",
            packet(&header, &uid, &[2, 4, 0, 5, 0], &[])?,
        ),
        (
            "MAX_INNER_FIELDS",
            packet(
                &header,
                &uid,
                &[cookie.as_slice(), unknown_inner.repeat(63).as_slice()].concat(),
                &[],
            )?,
        ),
        (
            "TOO_MANY_INNER",
            packet(
                &header,
                &uid,
                &[cookie.as_slice(), unknown_inner.repeat(64).as_slice()].concat(),
                &[],
            )?,
        ),
        (
            "MAX_OUTER_FIELDS",
            packet(
                &header,
                &[uid.as_slice(), unknown_outer.repeat(62).as_slice()].concat(),
                &cookie,
                &[],
            )?,
        ),
        (
            "TOO_MANY_OUTER",
            packet(
                &header,
                &[uid.as_slice(), unknown_outer.repeat(63).as_slice()].concat(),
                &cookie,
                &[],
            )?,
        ),
        (
            "MAX_PACKET_BYTES",
            packet(
                &header,
                &[uid.as_slice(), field(0x1234, &[0; 3952])?.as_slice()].concat(),
                &cookie,
                &[],
            )?,
        ),
        (
            "TOO_MANY_PACKET_BYTES",
            packet(
                &header,
                &[uid.as_slice(), field(0x1234, &[0; 3956])?.as_slice()].concat(),
                &cookie,
                &[],
            )?,
        ),
    ];
    let mut changed_uid = UID;
    changed_uid[0] ^= 1;
    cases.push((
        "WRONG_UID",
        packet(&header, &field(0x0104, &changed_uid)?, &cookie, &[])?,
    ));
    let mut changed_header = header;
    changed_header[24..32].copy_from_slice(&(ORIGIN ^ 1).to_be_bytes());
    cases.push(("WRONG_ORIGIN", packet(&changed_header, &uid, &cookie, &[])?));
    for (name, offset, value) in [
        ("WRONG_VERSION", 0, 0x1c),
        ("WRONG_MODE", 0, 0x23),
        ("UNSYNCHRONIZED", 0, 0xe4),
        ("STRATUM_ZERO", 1, 0),
        ("STRATUM_SIXTEEN", 1, 16),
    ] {
        let mut changed = header;
        changed[offset] = value;
        cases.push((name, packet(&changed, &uid, &cookie, &[])?));
    }
    println!("// SPDX-License-Identifier: GPL-2.0-only");
    println!(
        "//! Public component fixtures generated through the existing maintained AES-SIV API."
    );
    println!(
        "//! No TLS/exporter/native/entropy or production authentication provenance is claimed."
    );
    for (name, bytes) in cases {
        emit(name, &bytes);
    }
    Ok(())
}
