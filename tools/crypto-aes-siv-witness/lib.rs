#![forbid(unsafe_code)]
//! Standalone AES-SIV observations and private staged-decryption prototype.
//!
//! RFC 5297 Appendix A.1 supplies the independent, public test values below:
//! <https://www.rfc-editor.org/rfc/rfc5297.html#appendix-A.1>.
//! They are fixture data, not keys or plaintext from an application. The tested
//! `RustCrypto` crates are retained separately under their MIT OR Apache-2.0 terms;
//! this witness copies no algorithm implementation. Exact sources, dependencies,
//! features, licenses and build evidence remain separately subject to admission.
//! An observed error buffer is not by itself an API-violation or disclosure claim.

/// Private staged-decryption experiment; no main-product crypto adoption.
pub mod staged;

/// Fixed-shape laboratory request sealing; no association or send authority.
pub mod sealing;

#[cfg(test)]
mod tests {
    use aes_siv::{KeyInit, Tag, aead::inout::InOutBuf, siv::Aes128Siv};

    // Independent RFC 5297 A.1 values, rendered as fixed public octets.
    const KEY: [u8; 32] = [
        0xff, 0xfe, 0xfd, 0xfc, 0xfb, 0xfa, 0xf9, 0xf8, 0xf7, 0xf6, 0xf5, 0xf4, 0xf3, 0xf2, 0xf1,
        0xf0, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd,
        0xfe, 0xff,
    ];
    const AAD: [u8; 24] = [
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27,
    ];
    const PLAIN: [u8; 14] = [
        0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee,
    ];
    const TAG_BYTES: [u8; 16] = [
        0x85, 0x63, 0x2d, 0x07, 0xc6, 0xe8, 0xf3, 0x7f, 0x95, 0x0a, 0xcd, 0x32, 0x0a, 0x2e, 0xcc,
        0x93,
    ];
    const CIPHER: [u8; 14] = [
        0x40, 0xc0, 0x2b, 0x96, 0x90, 0xc4, 0xdc, 0x04, 0xda, 0xef, 0x7f, 0x6a, 0xfe, 0x5c,
    ];
    const SENTINEL: [u8; 14] = [0xa5; 14];

    // Fixed labels only: failure formatting never prints buffers, keys or errors.
    type WitnessResult<T> = Result<T, &'static str>;

    fn need(value: bool, failure: &'static str) -> WitnessResult<()> {
        if value { Ok(()) } else { Err(failure) }
    }

    fn new_cipher() -> WitnessResult<Aes128Siv> {
        Aes128Siv::new_from_slice(&KEY).map_err(|_| "fixture-key-shape")
    }

    fn tag(bytes: &[u8; 16]) -> WitnessResult<Tag> {
        Tag::try_from(&bytes[..]).map_err(|_| "fixture-tag-shape")
    }

    fn in_place(headers: &[&[u8]], input: &[u8; 14], tag: &Tag) -> WitnessResult<(bool, [u8; 14])> {
        let mut body = *input;
        let mut cipher = new_cipher()?;
        let ok = cipher
            .decrypt_inout_detached(headers, (&mut body[..]).into(), tag)
            .is_ok();
        Ok((ok, body))
    }

    fn disjoint(headers: &[&[u8]], input: &[u8; 14], tag: &Tag) -> WitnessResult<(bool, [u8; 14])> {
        let mut body = SENTINEL;
        let mut cipher = new_cipher()?;
        let spans = InOutBuf::new(&input[..], &mut body[..]).map_err(|_| "fixture-span-shape")?;
        let ok = cipher.decrypt_inout_detached(headers, spans, tag).is_ok();
        Ok((ok, body))
    }

    fn fixed_observation(label: &'static str, ok: bool, output: &[u8; 14]) {
        // Only public fixed labels and boolean classifications leave this process.
        println!(
            "case={label} authenticated={ok} equals_plain={} equals_cipher={} equals_sentinel={}",
            *output == PLAIN,
            *output == CIPHER,
            *output == SENTINEL,
        );
    }

    /// Independent vector: actual detached encryption and both real decrypt forms.
    #[test]
    fn rfc5297_a1_known_answer() -> WitnessResult<()> {
        let headers: [&[u8]; 1] = [&AAD];
        let mut ciphertext = PLAIN;
        let mut cipher = new_cipher()?;
        let produced = cipher
            .encrypt_inout_detached(headers, (&mut ciphertext[..]).into())
            .map_err(|_| "kat-encrypt-rejected")?;
        need(produced.as_slice() == &TAG_BYTES[..], "kat-tag-mismatch")?;
        need(ciphertext == CIPHER, "kat-ciphertext-mismatch")?;
        let known_tag = tag(&TAG_BYTES)?;
        let (ok_in, mut result_in) = in_place(&headers, &CIPHER, &known_tag)?;
        let (ok_out, mut result_out) = disjoint(&headers, &CIPHER, &known_tag)?;
        need(ok_in && result_in == PLAIN, "kat-in-place-decrypt")?;
        need(ok_out && result_out == PLAIN, "kat-disjoint-decrypt")?;
        println!("case=rfc5297-a1 status=pass evidence=independent-known-answer");
        ciphertext.fill(0);
        result_in.fill(0);
        result_out.fill(0);
        Ok(())
    }

    /// Changed AAD: record the source-derived asymmetry after actual rejection.
    #[test]
    fn changed_aad_failure_output() -> WitnessResult<()> {
        let mut changed = AAD;
        changed[0] ^= 1;
        let headers: [&[u8]; 1] = [&changed];
        let known_tag = tag(&TAG_BYTES)?;
        let (ok_in, mut result_in) = in_place(&headers, &CIPHER, &known_tag)?;
        let (ok_out, mut result_out) = disjoint(&headers, &CIPHER, &known_tag)?;
        fixed_observation("changed-aad-in-place", ok_in, &result_in);
        fixed_observation("changed-aad-disjoint", ok_out, &result_out);
        need(!ok_in && !ok_out, "changed-aad-not-rejected")?;
        need(
            result_in == CIPHER,
            "changed-aad-in-place-prediction-changed",
        )?;
        need(
            result_out == PLAIN,
            "changed-aad-disjoint-prediction-changed",
        )?;
        result_in.fill(0);
        result_out.fill(0);
        changed.fill(0);
        Ok(())
    }

    /// A changed tag's masked CTR bit leaves the RFC candidate bytes unchanged.
    #[test]
    fn masked_tag_bit_failure_output() -> WitnessResult<()> {
        let headers: [&[u8]; 1] = [&AAD];
        let mut changed_tag = TAG_BYTES;
        changed_tag[8] ^= 0x80;
        let changed = tag(&changed_tag)?;
        let (ok_in, mut result_in) = in_place(&headers, &CIPHER, &changed)?;
        let (ok_out, mut result_out) = disjoint(&headers, &CIPHER, &changed)?;
        fixed_observation("masked-tag-in-place", ok_in, &result_in);
        fixed_observation("masked-tag-disjoint", ok_out, &result_out);
        need(!ok_in && !ok_out, "masked-tag-not-rejected")?;
        need(
            result_in == CIPHER,
            "masked-tag-in-place-prediction-changed",
        )?;
        need(
            result_out == PLAIN,
            "masked-tag-disjoint-prediction-changed",
        )?;
        result_in.fill(0);
        result_out.fill(0);
        changed_tag.fill(0);
        Ok(())
    }

    /// A non-masked tag mutation does not assume any specific candidate bytes.
    #[test]
    fn unmasked_tag_bit_failure_output() -> WitnessResult<()> {
        let headers: [&[u8]; 1] = [&AAD];
        let mut changed_tag = TAG_BYTES;
        changed_tag[0] ^= 1;
        let changed = tag(&changed_tag)?;
        let (ok_in, mut result_in) = in_place(&headers, &CIPHER, &changed)?;
        let (ok_out, mut result_out) = disjoint(&headers, &CIPHER, &changed)?;
        fixed_observation("unmasked-tag-in-place", ok_in, &result_in);
        fixed_observation("unmasked-tag-disjoint", ok_out, &result_out);
        need(!ok_in && !ok_out, "unmasked-tag-not-rejected")?;
        need(
            result_in == CIPHER,
            "unmasked-tag-in-place-prediction-changed",
        )?;
        result_in.fill(0);
        result_out.fill(0);
        changed_tag.fill(0);
        Ok(())
    }

    /// 126 headers: independently observed mismatch and a same-code healthy neighbor.
    #[test]
    fn header_limit_126() -> WitnessResult<()> {
        let headers: [&[u8]; 126] = [&AAD; 126];
        let known_tag = tag(&TAG_BYTES)?;
        let (bad_in, mut mismatch_in) = in_place(&headers, &CIPHER, &known_tag)?;
        let (bad_out, mut mismatch_out) = disjoint(&headers, &CIPHER, &known_tag)?;
        fixed_observation("headers-126-mismatch-in-place", bad_in, &mismatch_in);
        fixed_observation("headers-126-mismatch-disjoint", bad_out, &mismatch_out);
        need(!bad_in && !bad_out, "headers-126-mismatch-not-rejected")?;
        need(
            mismatch_in == CIPHER && mismatch_out == PLAIN,
            "headers-126-mismatch-prediction-changed",
        )?;

        // This is a self-consistency neighbor, not an independent KAT for 126 headers.
        let mut generated = PLAIN;
        let mut cipher = new_cipher()?;
        let generated_tag = cipher
            .encrypt_inout_detached(headers, (&mut generated[..]).into())
            .map_err(|_| "headers-126-valid-encryption-rejected")?;
        let (ok_in, mut valid_in) = in_place(&headers, &generated, &generated_tag)?;
        let (ok_out, mut valid_out) = disjoint(&headers, &generated, &generated_tag)?;
        need(
            ok_in && ok_out && valid_in == PLAIN && valid_out == PLAIN,
            "headers-126-self-consistency",
        )?;
        println!("case=headers-126-valid status=pass evidence=self-consistency");
        mismatch_in.fill(0);
        mismatch_out.fill(0);
        generated.fill(0);
        valid_in.fill(0);
        valid_out.fill(0);
        Ok(())
    }

    /// 127 headers: early S2V rejection after the unchanged RFC-tag CTR transform.
    #[test]
    fn header_limit_127_early_rejection() -> WitnessResult<()> {
        let headers: [&[u8]; 127] = [&AAD; 127];
        let known_tag = tag(&TAG_BYTES)?;
        let (ok_in, mut result_in) = in_place(&headers, &CIPHER, &known_tag)?;
        let (ok_out, mut result_out) = disjoint(&headers, &CIPHER, &known_tag)?;
        fixed_observation("headers-127-in-place", ok_in, &result_in);
        fixed_observation("headers-127-disjoint", ok_out, &result_out);
        need(!ok_in && !ok_out, "headers-127-not-rejected")?;
        need(
            result_in == PLAIN && result_out == PLAIN,
            "headers-127-output-prediction-changed",
        )?;
        result_in.fill(0);
        result_out.fill(0);
        Ok(())
    }
}
