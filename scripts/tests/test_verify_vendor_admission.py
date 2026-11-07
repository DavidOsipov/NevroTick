"""Vendor admission controls through the public verifier; no build is executed."""
from __future__ import annotations

import hashlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("vendor_verify_test", ROOT / "scripts/verify.py")
VERIFY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFY)


def digest(raw):
    return hashlib.sha256(raw).hexdigest()


class VendorAdmissionTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="vendor-admission-control-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.vendor = self.root / "vendor"
        self.vendor.mkdir()
        self.policy = {"packages": []}
        self.package, self.metadata = self.add_package("fixture")

    def add_package(self, name):
        package = self.vendor / (name + "-1.0.0")
        package.mkdir()
        raw = b"//! Independent inert vendor control.\n"
        (package / "lib.rs").write_bytes(raw)
        files = {"lib.rs": digest(raw)}
        archive = "2" * 64
        self.policy["packages"].append({
            "name": name, "version": "1.0.0", "checksum": archive,
            "files_sha256": digest(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()),
        })
        metadata = package / ".cargo-checksum.json"
        metadata.write_text(json.dumps({"package": archive, "files": files}), encoding="utf-8")
        return package, metadata

    def run_public(self, *, accepted=False):
        """Use actual main -> execute -> admit_vendor; stop at the next admission."""
        stdout, stderr = io.StringIO(), io.StringIO()
        with patch.object(VERIFY, "POINTER", self.root / "no-default-inputs"), \
                patch.object(VERIFY, "admit_product", return_value=(self.policy, {})), \
                patch.object(VERIFY, "admit_tools", side_effect=VERIFY.ENV.Incomplete("CONTROL: vendor accepted")) as tools, \
                patch("sys.stdout", stdout), patch("sys.stderr", stderr):
            result = VERIFY.main(["--vendor-dir", str(self.vendor), "--toolchain-dir", str(self.root)])
        self.assertEqual(result, 1)
        self.assertIn("INCOMPLETE:", stdout.getvalue())
        if accepted:
            self.assertEqual(tools.call_count, 1)
            self.assertEqual(stderr.getvalue().strip(), "CONTROL: vendor accepted")
        else:
            self.assertEqual(tools.call_count, 0, stderr.getvalue())
            self.assertNotIn("CONTROL:", stderr.getvalue())
            self.assertLess(len(stderr.getvalue()), 512)
        return stderr.getvalue()

    def test_ordinary_and_exact_raw_limit_metadata_are_accepted(self):
        self.run_public(accepted=True)
        raw = self.metadata.read_bytes()
        self.metadata.write_bytes(raw + b" " * (256 * 1024 - len(raw)))
        self.run_public(accepted=True)

    def test_one_byte_above_raw_limit_rejects_before_metadata_open(self):
        with patch.object(VERIFY, "vendor_file", wraps=VERIFY.vendor_file) as content, \
                patch.object(VERIFY.os, "open", wraps=VERIFY.os.open) as opened:
            self.run_public(accepted=True)
        self.assertGreater(content.call_count, 0)
        self.assertGreater(opened.call_count, 0)
        raw = self.metadata.read_bytes()
        self.metadata.write_bytes(raw + b" " * (256 * 1024 + 1 - len(raw)))
        with patch.object(VERIFY, "vendor_file", side_effect=AssertionError("Content preceded raw admission")), \
                patch.object(VERIFY.os, "open", side_effect=AssertionError("Open preceded raw admission")):
            self.assertIn("byte bound", self.run_public())

    def test_nonobject_and_wrong_field_shapes_are_controlled(self):
        ordinary = json.loads(self.metadata.read_bytes())
        records = [[], None, "object", 12, {}, {**ordinary, "extra": 1},
                   {**ordinary, "files": []}, {**ordinary, "files": {}},
                   {**ordinary, "files": {"lib.rs": 42}}, {**ordinary, "package": None},
                   {**ordinary, "package": "G" * 64}]
        for record in records:
            with self.subTest(record=record):
                self.metadata.write_text(json.dumps(record), encoding="utf-8")
                self.run_public()

    def test_duplicate_fields_and_excessive_depth_reject(self):
        ordinary = self.metadata.read_bytes()
        file_hash = digest((self.package / "lib.rs").read_bytes())
        records = [b'{"package":"' + b"2" * 64 + b'",' + ordinary[1:],
                   ('{"package":"' + "2" * 64 + '","files":{"lib.rs":"' + file_hash
                    + '","lib.rs":"' + file_hash + '"}}').encode(),
                   b'{"files":' + b"[" * 64 + b"0" + b"]" * 64 + b"}"]
        for raw in records:
            with self.subTest(raw=raw[:80]):
                self.metadata.write_bytes(raw)
                self.run_public()

    def test_reported_deep_nesting_and_large_unused_field_are_controlled(self):
        ordinary = self.metadata.read_bytes()
        nested = ordinary[:-1] + b',"unused":' + b"[" * 20_000 + b"0" + b"]" * 20_000 + b"}"
        extra = ordinary[:-1] + b',"unused":"' + b"x" * (16 * 1024) + b'"}'
        self.assertLess(len(nested), 40_512)
        original_decode = VERIFY.json.loads
        observed = []

        def guarded_decode(document, *args, **kwargs):
            if document == nested.decode("ascii"):
                raise AssertionError("Deep vendor metadata reached the JSON decoder")
            if document == ordinary.decode("ascii"):
                observed.append("healthy")
            elif document == extra.decode("ascii"):
                self.assertLess(len(document), 256 * 1024)
                observed.append("bounded extra field")
            return original_decode(document, *args, **kwargs)

        with patch.object(VERIFY.json, "loads", side_effect=guarded_decode):
            self.run_public(accepted=True)
            self.metadata.write_bytes(nested)
            self.assertIn("depth bound", self.run_public())
            self.metadata.write_bytes(extra)
            self.assertIn("unadmitted shape", self.run_public())
        self.assertEqual(observed, ["healthy", "bounded extra field"])

    def test_bad_utf8_nonfinite_and_number_tokens_are_controlled(self):
        for raw in (b"\xff", b'{"package":NaN,"files":{}}', b'{"package":1e999,"files":{}}',
                    b'{"package":' + b"1" * 5000 + b',"files":{}}'):
            with self.subTest(raw=raw[:80]):
                self.metadata.write_bytes(raw)
                self.run_public()

    def test_aggregate_metadata_preflight_rejects_before_any_hash(self):
        self.add_package("neighbor")
        total = sum(path.stat().st_size for path in self.vendor.glob("*/.cargo-checksum.json"))
        with patch.object(VERIFY, "MAX_VENDOR_METADATA_TOTAL", total, create=True), \
                patch.object(VERIFY, "vendor_file", wraps=VERIFY.vendor_file) as content, \
                patch.object(VERIFY.hashlib, "sha256", wraps=VERIFY.hashlib.sha256) as hashed:
            self.run_public(accepted=True)
        self.assertGreater(content.call_count, 0)
        self.assertGreater(hashed.call_count, 0)
        with patch.object(VERIFY, "MAX_VENDOR_METADATA_TOTAL", total - 1, create=True), \
                patch.object(VERIFY, "vendor_file", side_effect=AssertionError("Content preceded aggregate admission")), \
                patch.object(VERIFY.os, "open", side_effect=AssertionError("Open preceded aggregate admission")), \
                patch.object(VERIFY.hashlib, "sha256", side_effect=AssertionError("Hash preceded aggregate admission")):
            self.assertIn("aggregate", self.run_public())

    def test_traversal_file_and_aggregate_byte_limits_have_healthy_neighbors(self):
        total_bytes = sum(path.stat().st_size for path in self.package.iterdir())
        cases = [("MAX_VENDOR_ENTRIES", 3, 2), ("MAX_VENDOR_PACKAGE_ENTRIES", 2, 1),
                 ("MAX_VENDOR_FILE_BYTES", self.metadata.stat().st_size, self.metadata.stat().st_size - 1),
                 ("MAX_VENDOR_TOTAL_BYTES", total_bytes, total_bytes - 1)]
        for name, healthy, rejected in cases:
            with self.subTest(limit=name):
                with patch.object(VERIFY, name, healthy, create=True):
                    self.run_public(accepted=True)
                with patch.object(VERIFY, name, rejected, create=True):
                    self.run_public()

    def test_tree_depth_and_relative_path_limits_reject(self):
        (self.package / "nested").mkdir()
        (self.package / "nested" / "deeper").mkdir()
        with patch.object(VERIFY, "MAX_VENDOR_DEPTH", 1, create=True):
            self.assertIn("depth", self.run_public())
        with patch.object(VERIFY, "MAX_VENDOR_PATH_BYTES", 4, create=True):
            self.assertIn("path", self.run_public())

    def test_checksum_paths_and_map_count_are_checked_with_unicode_neighbor(self):
        original = json.loads(self.metadata.read_bytes())
        for relative in ("", "/absolute", "./lib.rs", "../lib.rs", "a//b", "a\\b", "a\x00b", "\ud800"):
            with self.subTest(relative=repr(relative)):
                record = {**original, "files": {relative: original["files"]["lib.rs"]}}
                self.metadata.write_text(json.dumps(record), encoding="utf-8")
                self.assertIn("unadmitted path", self.run_public())
        renamed = "é [sample].rs"
        (self.package / "lib.rs").rename(self.package / renamed)
        files = {renamed: original["files"]["lib.rs"]}
        self.metadata.write_text(json.dumps({**original, "files": files}), encoding="utf-8")
        self.policy["packages"][0]["files_sha256"] = digest(
            json.dumps(files, sort_keys=True, separators=(",", ":")).encode())
        self.run_public(accepted=True)
        with patch.object(VERIFY, "MAX_VENDOR_MAP_ENTRIES", 0):
            self.assertIn("unadmitted shape", self.run_public())

    def test_growth_after_fstat_is_rejected_within_the_reserved_read(self):
        original = VERIFY.os.fstat
        changed = False

        def grow_after_stat(descriptor):
            nonlocal changed
            observed = original(descriptor)
            if not changed:
                path = Path(VERIFY.os.readlink(f"/proc/self/fd/{descriptor}"))
                with path.open("ab") as stream:
                    stream.write(b" ")
                changed = True
            return observed

        with patch.object(VERIFY.os, "fstat", grow_after_stat):
            self.assertIn("reserved byte bound", self.run_public())
        self.assertTrue(changed)

    def test_end_of_admission_custody_rejects_later_source_drift(self):
        original = VERIFY.vendor_record

        def change_after_decode(raw):
            record = original(raw)
            (self.package / "lib.rs").write_bytes(b"//! Later changed source.\n")
            return record

        with patch.object(VERIFY, "vendor_record", change_after_decode):
            self.assertIn("changed during admission", self.run_public())

    def test_source_archive_and_reviewed_map_checks_still_reject(self):
        ordinary = self.metadata.read_bytes()
        (self.package / "lib.rs").write_bytes(b"//! Changed bytes.\n")
        self.assertIn("source bytes", self.run_public())
        record = json.loads(ordinary)
        record["files"]["lib.rs"] = digest((self.package / "lib.rs").read_bytes())
        self.metadata.write_text(json.dumps(record), encoding="utf-8")
        self.assertIn("reviewed archive extraction", self.run_public())
        record["package"] = "3" * 64
        self.metadata.write_text(json.dumps(record), encoding="utf-8")
        self.assertIn("archive identity", self.run_public())

    def test_symlinks_and_unexpected_packages_still_reject(self):
        (self.package / "alias").symlink_to("lib.rs")
        self.assertIn("Linked or special", self.run_public())
        (self.package / "alias").unlink()
        (self.vendor / "unexpected").mkdir()
        self.assertIn("unexpected packages", self.run_public())


if __name__ == "__main__":
    unittest.main()
