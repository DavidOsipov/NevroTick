// SPDX-License-Identifier: GPL-2.0-only
//! One independent literal-field/AES-SIV builder shared by bounded generators.
//! It uses no product framing or sealing API and supplies no TLS/entropy evidence.

use aes_siv::{KeyInit, siv::Aes128Siv};

/// Bounded fixture construction failures, never cryptographic material.
pub(super) type FixtureResult<T = ()> = Result<T, &'static str>;

/// Build literal aligned fields independently of the product wire owner.
/// # Errors
/// Refuse an unrepresentable fixture length rather than truncate its prefix.
pub(super) fn field(kind: u16, body: &[u8]) -> FixtureResult<Vec<u8>> {
    let length = u16::try_from(body.len() + 4).map_err(|_| "fixture-field-length")?;
    let mut bytes = kind.to_be_bytes().to_vec();
    bytes.extend_from_slice(&length.to_be_bytes());
    bytes.extend_from_slice(body);
    Ok(bytes)
}

/// Use a declared public response key without changing the independent layout.
/// # Errors
/// Refuse unrepresentable lengths or a maintained encryption failure.
pub(super) fn packet_with_key_nonce(
    header: &[u8; 48],
    prefix: &[u8],
    plaintext: &[u8],
    suffix: &[u8],
    nonce: &[u8],
    s2c: &[u8; 32],
) -> FixtureResult<Vec<u8>> {
    let mut aad = header.to_vec();
    aad.extend_from_slice(prefix);
    let mut body = plaintext.to_vec();
    let mut cipher = Aes128Siv::new_from_slice(s2c).map_err(|_| "fixture-key")?;
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
pub(super) fn emit(name: &str, bytes: &[u8]) {
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
