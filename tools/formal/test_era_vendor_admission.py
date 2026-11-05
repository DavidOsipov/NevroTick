"""Selected vendor byte/custody controls; copy only tiny temporary fixtures."""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import stat
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
SPEC = importlib.util.spec_from_file_location("era_vendor_admission_test", HERE / "era_vendor_admission.py")
VENDOR = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VENDOR)
LEGACY_SPEC = importlib.util.spec_from_file_location("era_vendor_semantics_test", ROOT / "scripts/verify.py")
LEGACY = importlib.util.module_from_spec(LEGACY_SPEC)
LEGACY_SPEC.loader.exec_module(LEGACY)


def digest(data):
    return hashlib.sha256(data).hexdigest()


class ContractTests(unittest.TestCase):
    def test_actual_reviewed_constructor_closure_has_fourteen_granted_packages(self):
        contract = VENDOR._contract(ROOT)
        packages = contract["packages"]
        self.assertEqual(len(packages), 14)
        self.assertEqual({item["name"]: item["version"] for item in packages}, VENDOR.PROJECTION.GRAPH_VERSIONS)
        self.assertEqual(next(item["expected_features"] for item in packages if item["name"] == "garde"), ["derive"])
        self.assertEqual({item["name"] for item in packages if item["build_script"]},
                         {"proc-macro2", "quote", "rustversion"})
        self.assertEqual({item["name"] for item in packages if item["proc_macro"]}, {"garde_derive", "rustversion"})
        self.assertEqual(set(contract["inputs_sha256"]), set(VENDOR.input_names()))


class VendorAdmissionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="era-vendor-admission-controls-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.root = self.base / "frozen-repo"
        self.root.mkdir(mode=0o700)
        self.source = self.base / "source-cache"
        self.source.mkdir(mode=0o700)
        self.work = self.base / "writable-work"
        self.work.mkdir(mode=0o700)
        self.destination = self.base / "selected-bundle"
        # Tests substitute an explicitly reviewed miniature extraction map.
        # Production APIs expose no alternate policy or unpinned graph option.
        self.contract = copy.deepcopy(VENDOR._contract(ROOT))
        for record in self.contract["packages"]:
            package = self.source / (record["name"] + "-" + record["version"])
            package.mkdir(mode=0o700)
            manifest = (f'[package]\nname = {json.dumps(record["name"])}\n'
                        f'version = {json.dumps(record["version"])}\n'
                        f'license = {json.dumps(record["license"])}\n')
            if record["build_path"]:
                manifest += 'build = ' + json.dumps(record["build_path"]) + '\n'
            if record["proc_macro"]:
                manifest += '\n[lib]\nproc-macro = true\n'
            payloads = {"Cargo.toml": manifest.encode(), "src/lib.rs": b"//! Fixture bytes, never compiled.\n",
                        "LICENSE": b"Fixture notice retained byte for byte.\n"}
            if record["build_path"]:
                payloads[record["build_path"]] = b"fn main() {} // Fixture, never executed.\n"
            files = {}
            for relative, data in payloads.items():
                path = package / relative
                path.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
                path.write_bytes(data)
                path.chmod(0o600)
                files[relative] = digest(data)
            record["files_sha256"] = digest(json.dumps(files, sort_keys=True, separators=(",", ":")).encode())
            (package / ".cargo-checksum.json").write_text(json.dumps({"files": files, "package": record["checksum"]}))
        self.contract_patch = patch.object(VENDOR, "_contract", return_value=self.contract)
        self.contract_patch.start()
        self.addCleanup(self.contract_patch.stop)
        self.first_name = self.contract["packages"][0]["name"] + "-" + self.contract["packages"][0]["version"]
        self.package = self.source / self.first_name

    def prepare(self):
        return VENDOR.prepare(self.root, self.destination, self.source, work_root=self.work)

    def admit(self):
        return VENDOR.admit(self.root, self.destination, work_root=self.work)

    def test_healthy_selected_view_is_complete_sealed_and_stably_readmitted(self):
        result = self.prepare()
        self.assertEqual(result["readonly_root"], self.destination)
        self.assertEqual(result["vendor_root"], self.destination / "vendor")
        self.assertEqual({path.name for path in self.destination.iterdir()},
                         {"vendor", "cargo-config.toml", "manifest.json"})
        self.assertEqual(len(list(result["vendor_root"].iterdir())), 14)
        self.assertEqual(result["selected_file_count"], sum(len(item["files"]) for item in result["selected_packages"].values()))
        self.assertTrue(result["work_scope_checked"])
        self.assertEqual(result["native_commands_executed"], 0)
        self.assertFalse(result["shipping_graph_qualified"])
        self.assertEqual(VENDOR.identity(result), VENDOR.identity(self.admit()))
        for path in (self.destination, *self.destination.rglob("*")):
            self.assertEqual(stat.S_IMODE(path.stat().st_mode), 0o500 if path.is_dir() else 0o400)
        for name, package in result["selected_packages"].items():
            for relative, expected in package["files"].items():
                copied = self.destination / "vendor" / name / relative
                self.assertEqual(digest(copied.read_bytes()), expected)
                self.assertEqual(copied.read_bytes(), (self.source / name / relative).read_bytes())
        manifest = json.loads(result["manifest_path"].read_text())
        self.assertIn("allocation-success", manifest["allocation_model"])

    def test_authentication_matches_existing_admit_vendor_semantic_control(self):
        result = self.prepare()
        policy = {"packages": self.contract["packages"]}
        existing = LEGACY.admit_vendor(result["vendor_root"], policy)
        self.assertEqual(existing, {relative.removeprefix("vendor/"): value
                                   for relative, value in result["inventory"].items() if relative.startswith("vendor/")})

    def test_unrelated_source_siblings_are_neither_trusted_copied_nor_modified(self):
        unrelated = self.source / "unrelated-cache"
        unrelated.mkdir()
        sentinel = unrelated / "sentinel"
        sentinel.write_bytes(b"PRESERVE UNSELECTED")
        (unrelated / "alias").symlink_to(self.package / "src/lib.rs")
        os.mkfifo(unrelated / "not-read-fifo")
        result = self.prepare()
        self.assertNotIn("unrelated-cache", {path.name for path in result["vendor_root"].iterdir()})
        self.assertEqual(sentinel.read_bytes(), b"PRESERVE UNSELECTED")
        self.assertTrue((unrelated / "alias").is_symlink())

    def test_self_consistent_rewritten_source_and_checksum_map_fail_policy_pin(self):
        payload = self.package / "src/lib.rs"
        payload.write_bytes(b"//! Substitute bytes with self-consistent attacker metadata.\n")
        path = self.package / ".cargo-checksum.json"
        checksum = json.loads(path.read_text())
        checksum["files"]["src/lib.rs"] = digest(payload.read_bytes())
        path.write_text(json.dumps(checksum))
        with self.assertRaisesRegex(VENDOR.Incomplete, "reviewed archive extraction"):
            self.prepare()
        self.assertFalse(self.destination.exists())

    def test_same_size_same_mtime_substitution_is_hashed_again(self):
        path = self.package / "src/lib.rs"
        previous = path.stat()
        data = path.read_bytes()
        path.write_bytes(b"X" + data[1:])
        os.utime(path, ns=(previous.st_atime_ns, previous.st_mtime_ns))
        with self.assertRaisesRegex(VENDOR.Incomplete, "Actual vendor bytes"):
            self.prepare()
        self.assertFalse(self.destination.exists())

    def test_archive_identity_mismatch_rejected_before_destination_creation(self):
        path = self.package / ".cargo-checksum.json"
        checksum = json.loads(path.read_text())
        checksum["package"] = "0" * 64
        path.write_text(json.dumps(checksum))
        with self.assertRaisesRegex(VENDOR.Incomplete, "archive/checksum"):
            self.prepare()
        self.assertFalse(self.destination.exists())

    def test_missing_extra_file_and_extra_empty_directory_are_rejected(self):
        path = self.package / "LICENSE"
        data = path.read_bytes()
        path.unlink()
        with self.assertRaisesRegex(VENDOR.Incomplete, "missing or unexpected"):
            self.prepare()
        path.write_bytes(data)
        extra = self.package / "extra.rs"
        extra.write_bytes(b"unreviewed")
        with self.assertRaisesRegex(VENDOR.Incomplete, "missing or unexpected"):
            self.prepare()
        extra.unlink()
        extra.mkdir()
        with self.assertRaisesRegex(VENDOR.Incomplete, "missing or unexpected"):
            self.prepare()
        self.assertFalse(self.destination.exists())

    def test_payload_symlink_hardlink_and_fifo_are_rejected_without_following(self):
        path = self.package / "src/lib.rs"
        saved = path.read_bytes()
        sentinel = self.base / "outside"
        sentinel.write_bytes(saved)
        for kind in ("symlink", "hardlink", "fifo"):
            with self.subTest(kind=kind):
                path.unlink()
                if kind == "symlink":
                    path.symlink_to(sentinel)
                elif kind == "hardlink":
                    os.link(sentinel, path)
                else:
                    os.mkfifo(path)
                with self.assertRaises(VENDOR.Incomplete):
                    self.prepare()
                self.assertEqual(sentinel.read_bytes(), saved)
                path.unlink()
                path.write_bytes(saved)
        self.assertFalse(self.destination.exists())

    def test_unsafe_file_mode_or_owner_rejected(self):
        path = self.package / "src/lib.rs"
        path.chmod(0o666)
        with self.assertRaisesRegex(VENDOR.Incomplete, "unsafe mode"):
            self.prepare()
        path.chmod(0o600)
        with patch.object(VENDOR, "_uid", return_value=os.geteuid() + 1):
            with self.assertRaisesRegex(VENDOR.Incomplete, "owner/mode"):
                self.prepare()
        self.assertFalse(self.destination.exists())

    def test_checksum_duplicate_keys_path_escape_and_oversized_reads_fail_closed(self):
        path = self.package / ".cargo-checksum.json"
        saved = path.read_bytes()
        path.write_bytes(b'{"files":{},"files":{},"package":"a"}')
        with self.assertRaisesRegex(VENDOR.Incomplete, "Duplicate"):
            self.prepare()
        checksum = json.loads(saved)
        checksum["files"]["../outside.rs"] = "a" * 64
        path.write_text(json.dumps(checksum))
        with self.assertRaisesRegex(VENDOR.Incomplete, "escapes or aliases"):
            self.prepare()
        path.write_bytes(saved)
        with patch.object(VENDOR, "MAX_FILE_BYTES", 8):
            with self.assertRaisesRegex(VENDOR.Incomplete, "excessive size"):
                self.prepare()
        self.assertFalse(self.destination.exists())

    def test_aggregate_byte_budget_rejected_before_copying(self):
        with patch.object(VENDOR, "MAX_TOTAL_BYTES", 1):
            with self.assertRaisesRegex(VENDOR.Incomplete, "aggregate budget"):
                self.prepare()
        self.assertFalse(self.destination.exists())

    def test_existing_destination_alias_and_work_overlap_preserve_sentinels(self):
        self.destination.mkdir(mode=0o700)
        sentinel = self.destination / "preserved"
        sentinel.write_bytes(b"PRESERVE")
        with self.assertRaisesRegex(VENDOR.Incomplete, "new absolute leaf"):
            self.prepare()
        self.assertEqual(sentinel.read_bytes(), b"PRESERVE")
        sentinel.unlink()
        self.destination.rmdir()
        self.destination.symlink_to(self.source, target_is_directory=True)
        with self.assertRaisesRegex(VENDOR.Incomplete, "new absolute leaf"):
            self.prepare()
        self.destination.unlink()
        with self.assertRaisesRegex(VENDOR.Incomplete, "separate"):
            VENDOR.prepare(self.root, self.work / "bad-view", self.source, work_root=self.work)
        self.assertFalse((self.work / "bad-view").exists())
        alias = self.base / "source-alias"
        alias.symlink_to(self.source, target_is_directory=True)
        with self.assertRaisesRegex(VENDOR.Incomplete, "canonical"):
            VENDOR.prepare(self.root, self.destination, alias, work_root=self.work)
        self.assertFalse(self.destination.exists())

    def test_selected_source_drift_during_copy_rejects_partial_private_bundle(self):
        original = VENDOR.CUSTODY.read_file
        changed = []
        def drift(path, **options):
            data = original(path, **options)
            path = Path(path)
            if self.destination.exists() and path == self.package / "src/lib.rs" and not changed:
                path.write_bytes(b"X" + data[1:])
                changed.append(True)
            return data
        with patch.object(VENDOR.CUSTODY, "read_file", side_effect=drift):
            with self.assertRaisesRegex(VENDOR.Incomplete, "source changed during materialization"):
                self.prepare()
        self.assertTrue(changed)
        self.assertFalse((self.destination / "manifest.json").exists())

    def test_output_tamper_extra_package_and_config_redirection_fail_fresh_admission(self):
        self.prepare()
        payload = self.destination / "vendor" / self.first_name / "src/lib.rs"
        data = payload.read_bytes()
        payload.chmod(0o600)
        payload.write_bytes(b"X" + data[1:])
        payload.chmod(0o400)
        with self.assertRaisesRegex(VENDOR.Incomplete, "Actual vendor bytes"):
            self.admit()
        payload.chmod(0o600)
        payload.write_bytes(data)
        payload.chmod(0o400)
        vendor = self.destination / "vendor"
        vendor.chmod(0o700)
        extra = vendor / "unexpected"
        extra.mkdir(mode=0o500)
        vendor.chmod(0o500)
        with self.assertRaisesRegex(VENDOR.Incomplete, "unexpected packages"):
            self.admit()
        vendor.chmod(0o700)
        extra.rmdir()
        vendor.chmod(0o500)
        config = self.destination / "cargo-config.toml"
        config.chmod(0o600)
        config.write_bytes(b'[net]\noffline=false\n')
        config.chmod(0o400)
        with self.assertRaisesRegex(VENDOR.Incomplete, "only the selected offline"):
            self.admit()

    def test_manifest_tamper_and_writable_or_foreign_ancestor_rejected(self):
        self.prepare()
        manifest = self.destination / "manifest.json"
        data = json.loads(manifest.read_text())
        data["shipping_graph_qualified"] = True
        manifest.chmod(0o600)
        manifest.write_text(json.dumps(data))
        manifest.chmod(0o400)
        with self.assertRaisesRegex(VENDOR.Incomplete, "bundle manifest"):
            self.admit()
        self.base.chmod(0o777)
        with self.assertRaisesRegex(VENDOR.Incomplete, "ancestor"):
            self.admit()
        self.base.chmod(0o700)


if __name__ == "__main__":
    unittest.main()
