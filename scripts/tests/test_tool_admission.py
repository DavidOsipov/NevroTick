"""Compiler identity controls independent of rustup's version selection.

For release-mismatch OR commit-mismatch, the healthy identity is the common
accepting neighbor of the release-only and commit-only rejections. Requested
channel and LLVM stay valid in both pairs. These are logical condition-influence
controls, not instrumented MC/DC coverage measurements.
"""
import importlib.util
import hashlib
import json
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

    def test_current_distribution_identity_and_previous_release_rejection(self):
        """The released candidate is distinct from the synthetic legacy controls."""
        policy = json.loads((Path(__file__).resolve().parents[2]
                             / "policy/tool-manifest.json").read_text())
        identity = ("release: 1.100.0\n"
                    "commit-hash: 0ef1bc386059596dada8fb65b50425aab2ea637a\n"
                    "LLVM version: 23.1.5\n")
        ENV.require_admitted_compiler("1.100.0", identity, policy)
        for wrong in (identity.replace("release: 1.100.0", "release: 1.99.1"),
                      identity.replace("0ef1bc386059596dada8fb65b50425aab2ea637a",
                                       "58cf8f9c35b355199727627d07ae34a4e2846d67"),
                      identity.replace("23.1.5", "23.1.1")):
            with self.subTest(identity=wrong), self.assertRaises(ENV.Incomplete):
                ENV.require_admitted_compiler("1.100.0", wrong, policy)
        with self.assertRaisesRegex(ENV.Incomplete, "manifest disagree"):
            ENV.require_admitted_compiler("1.99.1", identity, policy)

    def test_another_installed_pin_does_not_change_admission(self):
        with self.assertRaisesRegex(ENV.Incomplete, "manifest disagree"):
            ENV.require_admitted_compiler("1.98.1", self.IDENTITY, self.POLICY)

    def test_same_release_with_another_compiler_commit_is_rejected(self):
        with self.assertRaisesRegex(ENV.Incomplete, "release/commit"):
            ENV.require_admitted_compiler("1.99.1", self.IDENTITY.replace("admitted-commit", "other-commit"), self.POLICY)

    def test_llvm_substitution_is_rejected(self):
        with self.assertRaisesRegex(ENV.Incomplete, "LLVM differs"):
            ENV.require_admitted_compiler("1.99.1", self.IDENTITY.replace("23.1.1", "22.1.0"), self.POLICY)

    def test_other_effective_release_with_matching_commit_is_rejected(self):
        identity = "release: 1.99.2\ncommit-hash: admitted-commit\nLLVM version: 23.1.1\n"
        with self.assertRaisesRegex(ENV.Incomplete, "Effective compiler release/commit differs"):
            ENV.require_admitted_compiler("1.99.1", identity, self.POLICY)

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
