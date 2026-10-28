"""Acquisition and ownership controls without installing a toolchain."""

import hashlib
import importlib.util
import io
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


SPEC = importlib.util.spec_from_file_location("prepare_rust", Path(__file__).with_name("prepare_rust.py"))
PREPARE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PREPARE)


class RustAcquisitionControls(unittest.TestCase):
    def test_admitted_download_is_a_healthy_control(self):
        payload = b"synthetic admitted manager"
        with patch.object(PREPARE.urllib.request, "urlopen", return_value=io.BytesIO(payload)):
            self.assertEqual(PREPARE.download("https://static.rust-lang.org/fixture", hashlib.sha256(payload).hexdigest()), payload)

    def test_substituted_download_and_wrong_origin_fail(self):
        with patch.object(PREPARE.urllib.request, "urlopen", return_value=io.BytesIO(b"substitution")):
            with self.assertRaises(ValueError):
                PREPARE.download("https://static.rust-lang.org/fixture", "0" * 64)
        with patch.object(PREPARE.urllib.request, "urlopen") as network:
            with self.assertRaises(ValueError):
                PREPARE.download("https://unreviewed.invalid/fixture", "0" * 64)
            network.assert_not_called()

    def test_only_a_new_private_external_area_is_owned(self):
        with tempfile.TemporaryDirectory() as folder:
            parent = Path(folder)
            self.assertEqual(PREPARE.private_area(parent), parent / "nevrotick-rust")
            with self.assertRaises(FileExistsError):
                PREPARE.private_area(parent)
        with self.assertRaises(ValueError):
            PREPARE.private_area(PREPARE.ROOT)

    def test_preexisting_symlink_cannot_redirect_rust_installation(self):
        with tempfile.TemporaryDirectory() as folder:
            parent = Path(folder)
            (parent / "nevrotick-rust").symlink_to(PREPARE.ROOT)
            with self.assertRaises(FileExistsError):
                PREPARE.private_area(parent)


if __name__ == "__main__":
    unittest.main()
