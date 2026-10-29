"""Challenge metadata acquisition custody without network, Cargo or executable fixtures."""
import contextlib
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("refresh_product", ROOT / "scripts/refresh-product-advisories.py")
REFRESH = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(REFRESH)


class MetadataAcquisitionControls(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="product-refresh-control-")
        self.addCleanup(temporary.cleanup)
        self.base = Path(temporary.name)
        self.root = self.base / "checkout"
        (self.root / "policy").mkdir(parents=True)
        (self.root / "Cargo.lock").write_text("synthetic admitted lock bytes\n")
        self.toolchain = self.base / "toolchain"
        (self.toolchain / "bin").mkdir(parents=True)
        self.deny = self.base / "cargo-deny"
        self.deny.write_text("synthetic deny bytes; never executable")
        compiler_hashes = {}
        for name in ("rustc", "cargo"):
            path = self.toolchain / "bin" / name
            path.write_text("synthetic pinned " + name)
            compiler_hashes[name] = hashlib.sha256(path.read_bytes()).hexdigest()
        policy = {"tools": [{"name": "cargo-deny", "binary_sha256": hashlib.sha256(self.deny.read_bytes()).hexdigest()}],
                  "rust": {"linux_binary_sha256": compiler_hashes}}
        (self.root / "policy/tool-manifest.json").write_text(json.dumps(policy))
        self.home = self.base / "metadata"
        self.receipt = self.base / "receipt.json"
        self.arguments = ["refresh-product-advisories.py", "--cargo-home", str(self.home),
                          "--receipt", str(self.receipt), "--deny", str(self.deny),
                          "--toolchain-dir", str(self.toolchain)]

    def acquisition(self, command, **kwargs):
        for name in ("advisory-dbs/reviewed-db/advisory.toml", "registry/index/cache"):
            path = self.home / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("controlled public metadata")
        self.assertEqual(set(kwargs["environment"]), {"PATH", "CARGO_HOME", "RUSTC", "LANG"})
        self.assertEqual(command, [str(self.deny), "--locked", "fetch", "db", "index"])
        return {"command": command, "exit_code": 0, "failure": None, "output": "synthetic acquisition observation"}

    def invoke(self, effect=None):
        with (patch.object(REFRESH, "ROOT", self.root), patch("sys.argv", self.arguments),
              patch.object(REFRESH.ENV, "run", side_effect=effect or self.acquisition) as run,
              contextlib.redirect_stdout(io.StringIO()) as output):
            status = REFRESH.main()
        return status, run, output.getvalue()

    def test_complete_acquisition_records_lock_tools_files_and_freshness(self):
        status, run, _ = self.invoke()
        self.assertEqual(status, 0)
        run.assert_called_once()
        record = json.loads(self.receipt.read_text())
        self.assertEqual(record["lockfile_sha256"], hashlib.sha256((self.root / "Cargo.lock").read_bytes()).hexdigest())
        self.assertEqual(record["tool_sha256"], hashlib.sha256(self.deny.read_bytes()).hexdigest())
        self.assertEqual(record["files"], REFRESH.inventory(self.home))
        self.assertGreater(record["acquired_unix"], 0)
        self.assertEqual(self.receipt.stat().st_mode & 0o777, 0o600)

    def test_substituted_tool_is_rejected_before_process_execution(self):
        self.deny.write_text("substituted executable")
        status, run, output = self.invoke()
        self.assertEqual(status, 1)
        self.assertIn("executable digest", output)
        run.assert_not_called()
        self.assertFalse(self.receipt.exists())

    def test_existing_snapshot_and_receipt_are_preserved(self):
        self.home.mkdir()
        sentinel = self.home / "existing"
        sentinel.write_text("preserve")
        self.receipt.write_text("preserve receipt")
        status, run, _ = self.invoke()
        self.assertEqual(status, 1)
        run.assert_not_called()
        self.assertEqual(sentinel.read_text(), "preserve")
        self.assertEqual(self.receipt.read_text(), "preserve receipt")

    def test_failed_acquisition_cannot_emit_success_receipt(self):
        def failure(command, **kwargs):
            return {"command": command, "exit_code": 17, "failure": None, "output": "controlled network failure"}
        status, run, output = self.invoke(failure)
        self.assertEqual(status, 1)
        self.assertIn("controlled network failure", output)
        run.assert_called_once()
        self.assertFalse(self.receipt.exists())

    def test_zero_exit_with_missing_required_snapshot_is_incomplete(self):
        def empty(command, **kwargs):
            return {"command": command, "exit_code": 0, "failure": None, "output": "no metadata"}
        status, _, output = self.invoke(empty)
        self.assertEqual(status, 1)
        self.assertIn("omitted", output)
        self.assertFalse(self.receipt.exists())

    def test_lock_drift_invalidates_successful_acquisition(self):
        def changed(command, **kwargs):
            result = self.acquisition(command, **kwargs)
            (self.root / "Cargo.lock").write_text("changed graph")
            return result
        status, _, output = self.invoke(changed)
        self.assertEqual(status, 1)
        self.assertIn("Lockfile changed", output)
        self.assertFalse(self.receipt.exists())

    def test_linked_metadata_is_not_followed_into_receipt(self):
        def linked(command, **kwargs):
            result = self.acquisition(command, **kwargs)
            (self.home / "unexpected").symlink_to(self.root / "Cargo.lock")
            return result
        status, _, output = self.invoke(linked)
        self.assertEqual(status, 1)
        self.assertIn("linked or special", output)
        self.assertFalse(self.receipt.exists())

    def test_ambient_credentials_do_not_reach_acquisition(self):
        with patch.dict(os.environ, {"NEVROTICK_SYNTHETIC_CREDENTIAL": "never-forward"}):
            status, run, _ = self.invoke()
        self.assertEqual(status, 0)
        self.assertNotIn("NEVROTICK_SYNTHETIC_CREDENTIAL", run.call_args.kwargs["environment"])


if __name__ == "__main__":
    unittest.main()
