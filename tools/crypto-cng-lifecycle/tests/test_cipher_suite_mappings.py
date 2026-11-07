"""Source-only descriptor checks; these execute neither Rust nor Windows CNG.

RFC 5289 section 3.2 fixes the TLS 1.2 suite identities, AES key widths and
hashes; RFC 8446 appendix B.4 supplies the TLS 1.3 neighboring controls.
Run with: python3 -B -m unittest discover -s tools/crypto-cng-lifecycle/tests -v
"""

from pathlib import Path
import re
import unittest


SOURCE = Path(__file__).resolve().parents[1] / "vendor/rustls-cng-crypto/src"
ECDSA256 = "TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384"


def descriptor(source: str, name: str) -> str:
    """Read exactly one named static descriptor, rejecting unrecognized syntax."""
    matches = re.findall(
        rf"^pub static {re.escape(name)}: SupportedCipherSuite\s*=\s*"
        rf"SupportedCipherSuite::Tls(?:12|13)\(&Tls(?:12|13)CipherSuite \{{"
        rf"(.*?)^    \}}\);",
        source,
        re.MULTILINE | re.DOTALL,
    )
    if len(matches) != 1:
        raise AssertionError(f"expected one descriptor for {name}, got {len(matches)}")
    return matches[0]


class CipherSuiteMappingTests(unittest.TestCase):
    """Trace advertised suite -> bound algorithm -> declared key byte length."""

    def setUp(self) -> None:
        self.tls12 = (SOURCE / "tls12.rs").read_text(encoding="utf-8")
        self.tls13 = (SOURCE / "tls13.rs").read_text(encoding="utf-8")
        self.aead = (SOURCE / "aead.rs").read_text(encoding="utf-8")

    def check_suite(
        self, source: str, name: str, key_bytes: int, hash_name: str, aead: str
    ) -> None:
        body = descriptor(source, name)
        self.assertEqual(re.findall(r"suite: CipherSuite::(\w+),", body), [name])
        self.assertEqual(re.findall(r"hash_provider: &(\w+),", body), [hash_name])
        bindings = re.findall(r"aead_alg: &aead::(\w+),", body)
        self.assertEqual(len(bindings), 1, name)
        algorithms = re.findall(
            rf"pub\(crate\) const {re.escape(bindings[0])}: Algorithm = Algorithm \{{"
            rf"([^}}]+)\}};",
            aead,
        )
        self.assertEqual(len(algorithms), 1, bindings[0])
        self.assertEqual(
            re.findall(r"key_size: (\d+),", algorithms[0]),
            [str(key_bytes)],
            name,
        )
        self.assertEqual(re.findall(r"is_aes: (\w+),", algorithms[0]), ["true"])

    def test_tls12_aes_suite_identities_key_sizes_and_hashes(self) -> None:
        for name, key_bytes, hash_name in (
            (ECDSA256, 32, "SHA384"),
            ("TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256", 16, "SHA256"),
            ("TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384", 32, "SHA384"),
            ("TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256", 16, "SHA256"),
        ):
            with self.subTest(suite=name):
                self.check_suite(self.tls12, name, key_bytes, hash_name, self.aead)

    def test_tls13_aes_neighbors(self) -> None:
        for name, key_bytes, hash_name in (
            ("TLS13_AES_256_GCM_SHA384", 32, "SHA384"),
            ("TLS13_AES_128_GCM_SHA256", 16, "SHA256"),
        ):
            with self.subTest(suite=name):
                self.check_suite(self.tls13, name, key_bytes, hash_name, self.aead)

    def test_rejects_wrong_aead_binding_mutant_and_preserves_neighbor(self) -> None:
        body = descriptor(self.tls12, ECDSA256)
        wrong = re.sub(r"aead_alg: &aead::\w+,", "aead_alg: &aead::AES_128_GCM,", body)
        mutated = self.tls12.replace(body, wrong, 1)
        with self.assertRaises(AssertionError):
            self.check_suite(mutated, ECDSA256, 32, "SHA384", self.aead)
        self.check_suite(
            mutated, "TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384", 32, "SHA384", self.aead
        )

    def test_rejects_wrong_algorithm_key_size_mutant(self) -> None:
        mutated = self.aead.replace("key_size: 32,", "key_size: 16,", 1)
        with self.assertRaises(AssertionError):
            self.check_suite(self.tls12, ECDSA256, 32, "SHA384", mutated)


if __name__ == "__main__":
    unittest.main()
