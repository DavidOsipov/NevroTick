//! Descriptor-only regressions for a later admitted provider test lane.
//!
//! These tests invoke the real Rustls descriptor methods without native crypto.
//! They have not been executed: the owned derivative intentionally omits the
//! upstream dev graph required by its other internal tests. Windows/provider
//! admission and the `tls12` feature are prerequisites; do not restore that graph
//! or bypass execution holds merely to run these tests.

use super::*;
use rustls::crypto::hash::HashAlgorithm;

#[test]
fn ecdsa_aes256_descriptor_uses_32_byte_keys() {
    let SupportedCipherSuite::Tls12(suite) = TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384 else {
        panic!("expected a TLS 1.2 descriptor");
    };
    assert_eq!(
        suite.common.suite,
        CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384
    );
    assert_eq!(
        suite.common.hash_provider.algorithm(),
        HashAlgorithm::SHA384
    );
    let shape = suite.aead_alg.key_block_shape();
    assert_eq!(shape.enc_key_len, 32);
    assert_eq!(shape.fixed_iv_len, 4);
    assert_eq!(shape.explicit_nonce_len, 8);
}

#[test]
fn tls12_aes_neighbor_key_sizes() {
    for (descriptor, key_bytes) in [
        (TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256, 16),
        (TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256, 16),
        (TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384, 32),
    ] {
        let SupportedCipherSuite::Tls12(suite) = descriptor else {
            panic!("expected a TLS 1.2 descriptor");
        };
        let shape = suite.aead_alg.key_block_shape();
        assert_eq!(shape.enc_key_len, key_bytes, "{:?}", suite.common.suite);
        assert_eq!(shape.fixed_iv_len, 4);
        assert_eq!(shape.explicit_nonce_len, 8);
    }
}

#[test]
fn tls13_aes_neighbor_key_sizes() {
    for (descriptor, key_bytes) in [
        (crate::tls13::TLS13_AES_128_GCM_SHA256, 16),
        (crate::tls13::TLS13_AES_256_GCM_SHA384, 32),
    ] {
        let SupportedCipherSuite::Tls13(suite) = descriptor else {
            panic!("expected a TLS 1.3 descriptor");
        };
        assert_eq!(
            suite.aead_alg.key_len(),
            key_bytes,
            "{:?}",
            suite.common.suite
        );
    }
}
