"""Independent controls for bootstrap refusal and subprocess outcome handling."""

import importlib.util
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch


SPEC = importlib.util.spec_from_file_location(
    "check_environment", Path(__file__).resolve().parents[1] / "check-environment.py"
)
ENV = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENV)


class GuardTests(unittest.TestCase):
    def test_override_names_never_include_values(self):
        values = {"PATH": "ok", "RUSTFLAGS": "synthetic-private-value",
                  "CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER": "hidden-linker"}
        names = ENV.overridden_names(values)
        self.assertEqual(names, ["CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER", "RUSTFLAGS"])
        self.assertNotIn("synthetic-private-value", str(names))

    def test_even_empty_override_is_refused(self):
        with patch.dict(os.environ, {"RUSTFLAGS": ""}, clear=True):
            with self.assertRaisesRegex(ENV.Incomplete, "RUSTFLAGS"):
                ENV.inspect(Path(tempfile.gettempdir()))

    def test_no_sandbox_never_falls_back_to_host_cargo(self):
        with patch.object(ENV.shutil, "which", return_value=None):
            with self.assertRaisesRegex(ENV.Incomplete, "fallback is forbidden"):
                ENV.sandbox_command(Path("/toolchain"), Path("/work"), ["cargo", "test"])

    def test_changed_target_flags_are_refused_before_execution(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "rust-toolchain.toml").write_text(
                '[toolchain]\nchannel="1.99.1"\nprofile="minimal"\n'
                'components=["clippy","rustfmt","rust-analyzer","rust-src","llvm-tools-preview"]\n'
                'targets=["x86_64-pc-windows-msvc"]\n'
            )
            (root / ".cargo").mkdir()
            (root / ".cargo/config.toml").write_text(
                '[target.x86_64-pc-windows-msvc]\nrustflags=["--cap-lints=allow"]\n'
            )
            with patch.dict(os.environ, {}, clear=True):
                with self.assertRaisesRegex(ENV.Incomplete, "differs from the admitted"):
                    ENV.inspect(root)

    def test_empty_test_selection_is_not_passing_evidence(self):
        with self.assertRaisesRegex(ENV.Incomplete, "not an empty selection"):
            ENV.require_test_count("test result: ok. 0 passed; 0 failed; 0 ignored;", 2, "unit")
        ENV.require_test_count("test result: ok. 2 passed; 0 failed; 0 ignored;", 2, "unit")


class ProcessTests(unittest.TestCase):
    def test_child_nonzero_exit_is_preserved(self):
        result = ENV.run([sys.executable, "-c", "raise SystemExit(7)"], cwd=Path.cwd())
        self.assertEqual(result["exit_code"], 7)
        with self.assertRaises(ENV.Incomplete):
            ENV.require_ok(result)

    def test_wall_timeout_is_not_success(self):
        result = ENV.run([sys.executable, "-c", "import time; time.sleep(10)"],
                         cwd=Path.cwd(), timeout=0.1)
        self.assertIn("wall timeout", result["failure"])
        self.assertNotEqual(result["exit_code"], 0)

    def test_output_budget_is_not_success(self):
        result = ENV.run([sys.executable, "-c", "import os; os.write(1,b'x'*2097153)"],
                         cwd=Path.cwd())
        self.assertEqual(result["failure"], "output budget exceeded")
        self.assertLessEqual(len(result["output"]), ENV.MAX_OUTPUT)


if __name__ == "__main__":
    unittest.main()
