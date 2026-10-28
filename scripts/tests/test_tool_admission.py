"""Compiler identity controls independent of rustup's version selection."""
import importlib.util
import hashlib
from pathlib import Path
import tempfile
import unittest

SPEC = importlib.util.spec_from_file_location(
    "tool_admission", Path(__file__).resolve().parents[1] / "check-environment.py"
)
ENV = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENV)


class CompilerAdmissionTests(unittest.TestCase):
    POLICY = {"rust": {"channel": "1.99.1", "compiler_commit": "admitted-commit", "llvm_version": "23.1.1"}}
    IDENTITY = "release: 1.99.1\ncommit-hash: admitted-commit\nLLVM version: 23.1.1\n"

    def test_healthy_compiler_identity_is_admitted(self):
        ENV.require_admitted_compiler("1.99.1", self.IDENTITY, self.POLICY)

    def test_another_installed_pin_does_not_change_admission(self):
        with self.assertRaisesRegex(ENV.Incomplete, "manifest disagree"):
            ENV.require_admitted_compiler("1.98.1", self.IDENTITY, self.POLICY)

    def test_same_release_with_another_compiler_commit_is_rejected(self):
        with self.assertRaisesRegex(ENV.Incomplete, "release/commit"):
            ENV.require_admitted_compiler("1.99.1", self.IDENTITY.replace("admitted-commit", "other-commit"), self.POLICY)

    def test_llvm_substitution_is_rejected(self):
        with self.assertRaisesRegex(ENV.Incomplete, "LLVM differs"):
            ENV.require_admitted_compiler("1.99.1", self.IDENTITY.replace("23.1.1", "22.1.0"), self.POLICY)

    def test_distribution_source_override_is_refused(self):
        self.assertEqual(ENV.overridden_names({"RUSTUP_DIST_SERVER": "synthetic"}), ["RUSTUP_DIST_SERVER"])

    def test_healthy_helper_bytes_are_admitted(self):
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder) / "helper"
            path.write_bytes(b"admitted synthetic helper")
            expected = hashlib.sha256(path.read_bytes()).hexdigest()
            self.assertEqual(ENV.require_admitted_binary(path, expected, "helper"), expected)

    def test_substituted_helper_is_rejected_before_execution(self):
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder) / "helper"
            expected = hashlib.sha256(b"admitted synthetic helper").hexdigest()
            path.write_bytes(b"substituted helper")
            with self.assertRaisesRegex(ENV.Incomplete, "executable digest differs"):
                ENV.require_admitted_binary(path, expected, "bwrap")


if __name__ == "__main__":
    unittest.main()
