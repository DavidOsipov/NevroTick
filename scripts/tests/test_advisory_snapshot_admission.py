"""Consumer controls for finite, exact, fresh public advisory snapshots."""
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("advisory_consumer_verify", ROOT / "scripts/verify.py")
VERIFY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFY)


class BoundedAdvisoryAdmission(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="advisory-consumer-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.home = self.root / "metadata"
        self.names = ("advisory-dbs/record", "registry/index/record")
        data = b"independent public metadata control"
        for name in self.names:
            path = self.home / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        self.files = dict.fromkeys(self.names, hashlib.sha256(data).hexdigest())
        self.lock = "1" * 64
        self.record = {"schema_version": 1, "acquired_unix": 100000,
                       "cargo_home": str(self.home), "files": self.files,
                       "lockfile_sha256": self.lock}
        self.receipt = self.root / "receipt.json"
        self.receipt.write_text(json.dumps(self.record))

    def admit(self):
        return VERIFY.admit_advisories(self.receipt, 100010, self.lock)

    def test_healthy_exact_snapshot_remains_compatible(self):
        self.assertEqual(self.admit(), (self.home, self.files))

    def test_raw_receipt_limit_precedes_json_and_tree_admission(self):
        with self.receipt.open("wb") as stream:
            stream.truncate(VERIFY.ADVISORY.MAX_RECEIPT_BYTES + 1)
        with patch.object(VERIFY.ADVISORY, "inventory") as inventory:
            with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "byte bound"):
                self.admit()
            inventory.assert_not_called()

    def test_forged_file_map_limits_and_paths_fail_before_tree_io(self):
        cases = (
            ({"../outside": "1" * 64}, None),
            ({"/outside": "1" * 64}, None),
            ({"advisory-dbs/record": "G" * 64}, None),
            ({"first": "1" * 64, "second": "2" * 64}, 1),
        )
        for files, maximum in cases:
            with self.subTest(files=files):
                self.record["files"] = files
                self.receipt.write_text(json.dumps(self.record))
                with (patch.object(VERIFY.ADVISORY, "MAX_ENTRIES", maximum or 16384),
                      patch.object(VERIFY.ADVISORY, "inventory") as inventory):
                    with self.assertRaises(VERIFY.ENV.Incomplete):
                        self.admit()
                    inventory.assert_not_called()

    def test_duplicate_receipt_fields_are_refused(self):
        self.receipt.write_text('{"files":{},"files":{}}')
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "Duplicate"):
            self.admit()

    def test_complete_map_cannot_authorize_oversized_body(self):
        with (self.home / self.names[0]).open("wb") as stream:
            stream.truncate(VERIFY.ADVISORY.MAX_FILE_BYTES + 1)
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "byte bound"):
            self.admit()

    def test_linked_metadata_root_is_not_normalized_into_admission(self):
        link = self.root / "alias"
        link.symlink_to(self.home, target_is_directory=True)
        self.record["cargo_home"] = str(link)
        self.receipt.write_text(json.dumps(self.record))
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "canonical directory"):
            self.admit()

    def test_receipt_bookend_uses_the_same_raw_ceiling(self):
        self.assertEqual(VERIFY.ADVISORY.receipt_digest(self.receipt),
                         hashlib.sha256(self.receipt.read_bytes()).hexdigest())
        with self.receipt.open("wb") as stream:
            stream.truncate(VERIFY.ADVISORY.MAX_RECEIPT_BYTES + 1)
        with self.assertRaisesRegex(ValueError, "byte bound"):
            VERIFY.ADVISORY.receipt_digest(self.receipt)


if __name__ == "__main__":
    unittest.main()
