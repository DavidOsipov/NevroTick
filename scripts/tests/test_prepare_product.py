"""Independent archive/admission controls for UI-M0 acquisition; no network or Cargo."""
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("prepare_product", ROOT / "scripts/prepare-product.py")
PREPARE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PREPARE)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def file_map_digest(files):
    hashes = {name: sha(value) for name, value in files.items()}
    return sha(json.dumps(hashes, sort_keys=True, separators=(",", ":")).encode())


class ArchiveControls(unittest.TestCase):
    def setUp(self):
        self.files = {"Cargo.toml": b'[package]\nname="bounded-control"\nversion="1.0.0"\n',
                      "src/lib.rs": b"//! Synthetic admission control.\n"}
        self.package = {"name": "bounded-control", "version": "1.0.0", "files_sha256": file_map_digest(self.files)}

    def bundle(self, *, extra=(), replacements=None):
        stream = io.BytesIO()
        with tarfile.open(fileobj=stream, mode="w:gz") as archive:
            for name, contents in (replacements or self.files).items():
                item = tarfile.TarInfo("bounded-control-1.0.0/" + name)
                item.size = len(contents)
                archive.addfile(item, io.BytesIO(contents))
            for item, contents in extra:
                archive.addfile(item, io.BytesIO(contents) if contents is not None else None)
        data = stream.getvalue()
        self.package["checksum"] = sha(data)
        return data

    def regular(self, path, contents=b"unexpected"):
        item = tarfile.TarInfo(path)
        item.size = len(contents)
        return item, contents

    def test_admitted_regular_archive_has_exact_bytes(self):
        self.assertEqual(PREPARE.archive_files(self.bundle(), self.package), self.files)

    def test_wrong_archive_hash_is_rejected_before_tar_parser(self):
        data = self.bundle()
        with patch.object(PREPARE.tarfile, "open") as parser:
            with self.assertRaisesRegex(ValueError, "identity or size"):
                PREPARE.archive_files(data + b"substitution", self.package)
            parser.assert_not_called()

    def test_archive_limit_applies_before_parsing(self):
        data = self.bundle()
        with patch.object(PREPARE, "MAX_ARCHIVE", len(data) - 1):
            with self.assertRaisesRegex(ValueError, "identity or size"):
                PREPARE.archive_files(data, self.package)

    def test_traversal_absolute_wrong_prefix_and_backslash_are_rejected(self):
        for name in ("bounded-control-1.0.0/../../outside", "/absolute",
                     "another-package-1.0.0/src/lib.rs", "bounded-control-1.0.0/a\\b"):
            with self.subTest(name=name):
                data = self.bundle(extra=[self.regular(name)])
                with self.assertRaisesRegex(ValueError, "escapes its exact package"):
                    PREPARE.archive_files(data, self.package)

    def test_symlinks_hardlinks_and_devices_are_rejected(self):
        for kind in (tarfile.SYMTYPE, tarfile.LNKTYPE, tarfile.CHRTYPE):
            with self.subTest(kind=kind):
                item = tarfile.TarInfo("bounded-control-1.0.0/alias")
                item.type, item.linkname = kind, "src/lib.rs"
                data = self.bundle(extra=[(item, None)])
                with self.assertRaisesRegex(ValueError, "link or special"):
                    PREPARE.archive_files(data, self.package)

    def test_duplicate_and_reserved_checksum_records_are_rejected(self):
        for name in ("src/lib.rs", ".cargo-checksum.json"):
            with self.subTest(name=name):
                data = self.bundle(extra=[self.regular("bounded-control-1.0.0/" + name)])
                with self.assertRaisesRegex(ValueError, "Duplicate or reserved"):
                    PREPARE.archive_files(data, self.package)

    def test_changed_source_cannot_reuse_reviewed_file_map(self):
        files = dict(self.files, **{"src/lib.rs": b"// Substituted source.\n"})
        data = self.bundle(replacements=files)
        with self.assertRaisesRegex(ValueError, "reviewed file-map"):
            PREPARE.archive_files(data, self.package)

    def test_archive_manifest_cannot_claim_another_package(self):
        files = dict(self.files, **{"Cargo.toml": b'[package]\nname="wrong"\nversion="1.0.0"\n'})
        data = self.bundle(replacements=files)
        with self.assertRaisesRegex(ValueError, "manifest identity"):
            PREPARE.archive_files(data, self.package)

    def test_decompressed_metadata_and_padding_have_a_total_budget(self):
        data = self.bundle()
        with patch.object(PREPARE, "MAX_EXPANDED", 4096):
            with self.assertRaisesRegex(ValueError, "budget"):
                PREPARE.archive_files(data, self.package)

    def test_directory_members_count_toward_iteration_budget(self):
        directories = []
        for i in range(10):
            item = tarfile.TarInfo(f"bounded-control-1.0.0/directory-{i}")
            item.type = tarfile.DIRTYPE
            directories.append((item, None))
        data = self.bundle(extra=directories)
        with patch.object(PREPARE, "MAX_MEMBERS", 8, create=True):
            with self.assertRaisesRegex(ValueError, "budget"):
                PREPARE.archive_files(data, self.package)


class PublicationControls(unittest.TestCase):
    bundle = ArchiveControls.bundle

    def setUp(self):
        ArchiveControls.setUp(self)
        temporary = tempfile.TemporaryDirectory(prefix="product-acquisition-control-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.vendor = self.root / "vendor"

    def test_verified_new_vendor_contains_only_reviewed_files(self):
        data = self.bundle()
        with patch.object(PREPARE.urllib.request, "urlopen", return_value=io.BytesIO(data)) as request:
            PREPARE.acquire(self.vendor, {"packages": [self.package]})
        url = request.call_args.args[0].full_url
        self.assertEqual(url, "https://static.crates.io/crates/bounded-control/bounded-control-1.0.0.crate")
        package = self.vendor / "bounded-control-1.0.0"
        self.assertEqual((package / "src/lib.rs").read_bytes(), self.files["src/lib.rs"])
        record = json.loads((package / ".cargo-checksum.json").read_text())
        self.assertEqual(record["package"], self.package["checksum"])
        self.assertEqual(record["files"], {key: sha(value) for key, value in self.files.items()})
        self.assertFalse(list(self.root.glob(".ui-m0-acquire-*")))

    def test_existing_output_is_preserved_without_network(self):
        self.vendor.mkdir()
        sentinel = self.vendor / "owned-by-user"
        sentinel.write_text("preserve")
        with patch.object(PREPARE.urllib.request, "urlopen") as network:
            with self.assertRaisesRegex(ValueError, "must be new"):
                PREPARE.acquire(self.vendor, {"packages": []})
            network.assert_not_called()
        self.assertEqual(sentinel.read_text(), "preserve")

    def test_failed_archive_publishes_no_partial_vendor(self):
        data = self.bundle() + b"substitution"
        with patch.object(PREPARE.urllib.request, "urlopen", return_value=io.BytesIO(data)):
            with self.assertRaisesRegex(ValueError, "identity or size"):
                PREPARE.acquire(self.vendor, {"packages": [self.package]})
        self.assertFalse(self.vendor.exists())
        self.assertFalse(list(self.root.glob(".ui-m0-acquire-*")))

    def test_concurrent_empty_destination_is_not_replaced(self):
        data = self.bundle()
        def concurrent_destination(*args, **kwargs):
            self.vendor.mkdir()
            return io.BytesIO(data)
        with patch.object(PREPARE.urllib.request, "urlopen", side_effect=concurrent_destination):
            with self.assertRaises((ValueError, FileExistsError, OSError)):
                PREPARE.acquire(self.vendor, {"packages": [self.package]})
        self.assertEqual(list(self.vendor.iterdir()), [])


if __name__ == "__main__":
    unittest.main()
