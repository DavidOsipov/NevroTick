"""Behavioral controls for product admission and result accounting; no Cargo execution."""
import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import tomllib
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("product_verify", ROOT / "scripts/verify.py")
VERIFY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFY)


def digest(data):
    return hashlib.sha256(data).hexdigest()


class ProductAdmissionTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="product-admission-control-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        for name in (*VERIFY.SOURCE_FILES, *VERIFY.STRICT.CONFIG_FILES, *VERIFY.CHECKER_FILES):
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            if (ROOT / name).is_file():
                shutil.copyfile(ROOT / name, path)
            else:
                path.write_text("")
        (self.root / "policy").mkdir(exist_ok=True)
        for name in ("rust-workspace.toml", "tool-manifest.json"):
            shutil.copyfile(ROOT / "policy" / name, self.root / "policy" / name)
        (self.root / "Cargo.toml").write_text(
            '[workspace]\nmembers = ["crates/nevrotick-ui"]\n' +
            (ROOT / "policy/rust-workspace.toml").read_text())
        member = self.root / VERIFY.MEMBER
        (member / "src").mkdir(parents=True)
        (member / "Cargo.toml").write_text(
            '[package]\nname="nevrotick-ui"\nversion="0.0.0"\n[lints]\nworkspace=true\n')
        (member / "src/lib.rs").write_text('//! Test control; never compiled.\n')
        self.archive_hash = "1" * 64
        (self.root / "Cargo.lock").write_text(
            'version = 4\n[[package]]\nname="nevrotick-ui"\nversion="0.0.0"\n'
            '[[package]]\nname="fixture-dependency"\nversion="1.0.0"\n'
            'source="registry+https://github.com/rust-lang/crates.io-index"\n'
            f'checksum="{self.archive_hash}"\n')
        self.policy = {"schema_version": 1,
                       "lockfile_sha256": VERIFY.sha256(self.root / "Cargo.lock"),
                       "manifests_sha256": {name: VERIFY.sha256(self.root / name)
                                            for name in ("Cargo.toml", VERIFY.MEMBER + "/Cargo.toml")},
                       "packages": [{"name": "fixture-dependency", "version": "1.0.0",
                                     "checksum": self.archive_hash}]}
        self.write_policy()

    def write_policy(self):
        (self.root / "policy/ui-m0-dependencies.json").write_text(json.dumps(self.policy))

    def test_actual_single_member_and_locked_graph_are_accepted(self):
        _, files = VERIFY.admit_product(self.root)
        self.assertIn("crates/nevrotick-ui/src/lib.rs", files)
        self.assertIn("Cargo.lock", files)

    def test_missing_manifest_is_incomplete_without_running_a_tool(self):
        (self.root / "Cargo.toml").unlink()
        with patch.object(VERIFY.ENV, "run") as run:
            with self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.admit_product(self.root)
            run.assert_not_called()

    def test_unreviewed_member_cannot_hide_behind_green_ui_tests(self):
        path = self.root / "Cargo.toml"
        path.write_text(path.read_text().replace('members = ["crates/nevrotick-ui"]',
                                                'members = ["crates/nevrotick-ui", "other"]'))
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "single reviewed"):
            VERIFY.admit_product(self.root)

    def test_member_must_inherit_policy(self):
        path = self.root / VERIFY.MEMBER / "Cargo.toml"
        path.write_text(path.read_text().replace("workspace=true", "workspace=false"))
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "inherited lints"):
            VERIFY.admit_product(self.root)

    def test_modified_lock_is_rejected_before_cargo(self):
        with (self.root / "Cargo.lock").open("a") as stream:
            stream.write("\n# changed candidate\n")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "lockfile differs"):
            VERIFY.admit_product(self.root)

    def test_feature_change_cannot_reuse_an_unchanged_lock_admission(self):
        path = self.root / VERIFY.MEMBER / "Cargo.toml"
        path.write_text(path.read_text() + '\n[features]\nunreviewed = []\n')
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "manifest/features differ"):
            VERIFY.admit_product(self.root)

    def test_new_local_package_requires_explicit_admission(self):
        with (self.root / "Cargo.lock").open("a") as stream:
            stream.write('\n[[package]]\nname="unexpected"\nversion="0.1.0"\n')
        self.policy["lockfile_sha256"] = VERIFY.sha256(self.root / "Cargo.lock")
        self.write_policy()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unexpected packages"):
            VERIFY.admit_product(self.root)

    def test_build_script_and_nested_cargo_config_are_rejected(self):
        member = self.root / VERIFY.MEMBER
        (member / "build.rs").write_text("fn main() {}")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "build script"):
            VERIFY.admit_product(self.root)
        (member / "build.rs").unlink()
        (member / ".cargo").mkdir()
        (member / ".cargo/config.toml").write_text("[build]\n")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "nested manifest or Cargo"):
            VERIFY.admit_product(self.root)

    def test_source_symlink_is_not_followed(self):
        (self.root / VERIFY.MEMBER / "src/alias.rs").symlink_to("lib.rs")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "Linked or special"):
            VERIFY.admit_product(self.root)


class VendorIntegrityTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="product-vendor-control-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.package = self.root / "fixture-1.0.0"
        self.package.mkdir()
        self.payload = self.package / "lib.rs"
        self.payload.write_text("//! Healthy admitted vendor bytes.\n")
        self.files = {"lib.rs": VERIFY.sha256(self.payload)}
        self.archive_hash = "2" * 64
        self.policy = {"packages": [{"name": "fixture", "version": "1.0.0", "checksum": self.archive_hash,
                                    "files_sha256": digest(json.dumps(self.files, sort_keys=True,
                                                                       separators=(",", ":")).encode())}]}
        self.record = self.package / ".cargo-checksum.json"
        self.record.write_text(json.dumps({"package": self.archive_hash, "files": self.files}))

    def test_verified_source_inventory_is_accepted(self):
        self.assertEqual(len(VERIFY.admit_vendor(self.root, self.policy)), 2)

    def test_modified_source_is_rejected(self):
        self.payload.write_text("//! Tampered source.\n")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "source bytes"):
            VERIFY.admit_vendor(self.root, self.policy)

    def test_mutating_source_and_its_self_reported_checksum_still_fails(self):
        self.payload.write_text("//! Tampered source.\n")
        self.record.write_text(json.dumps({"package": self.archive_hash,
                                          "files": {"lib.rs": VERIFY.sha256(self.payload)}}))
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "reviewed archive extraction"):
            VERIFY.admit_vendor(self.root, self.policy)

    def test_extra_package_and_symlink_are_rejected(self):
        extra = self.root / "unexpected"
        extra.mkdir()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unexpected packages"):
            VERIFY.admit_vendor(self.root, self.policy)
        extra.rmdir()
        (self.package / "alias").symlink_to("lib.rs")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "Linked or special"):
            VERIFY.admit_vendor(self.root, self.policy)


class DependencyPolicyTests(unittest.TestCase):
    def setUp(self):
        self.base = (ROOT / "deny.toml").read_text()
        self.policy = {"packages": [
            {"name": "proc-macro2", "version": "1.0.107", "selected_license": "MIT", "build_script": True},
            {"name": "quote", "version": "1.0.47", "selected_license": "MIT", "build_script": True},
            {"name": "rustversion", "version": "1.0.23", "selected_license": "MIT", "build_script": True},
            {"name": "bounded-helper", "version": "1.0.0", "selected_license": "BSL-1.0"},
            {"name": "unicode-control", "version": "1.0.0", "selected_license": "MIT AND Unicode-3.0"},
        ]}

    def test_only_exact_grants_change_and_held_rules_survive(self):
        result = tomllib.loads(VERIFY.product_deny(self.base, self.policy))
        original = tomllib.loads(self.base)
        self.assertEqual(result["bans"]["build"]["allow-build-scripts"],
                         ["proc-macro2@1.0.107", "quote@1.0.47", "rustversion@1.0.23"])
        self.assertEqual(set(result["licenses"]["allow"]), {"GPL-2.0-only", "MIT", "BSL-1.0", "Unicode-3.0"})
        result["licenses"]["allow"] = original["licenses"]["allow"]
        result["bans"]["build"]["allow-build-scripts"] = original["bans"]["build"]["allow-build-scripts"]
        self.assertEqual(result, original)

    def test_new_build_script_or_version_requires_review(self):
        self.policy["packages"][0]["version"] = "1.0.108"
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "reviewed policy change"):
            VERIFY.product_deny(self.base, self.policy)

    def test_new_license_and_broadened_bootstrap_are_rejected(self):
        self.policy["packages"][0]["selected_license"] = "Apache-2.0"
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "reviewed policy change"):
            VERIFY.product_deny(self.base, self.policy)
        self.policy["packages"][0]["selected_license"] = "MIT"
        base = self.base.replace('allow = ["GPL-2.0-only"]', 'allow = ["GPL-2.0-only", "MIT"]')
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unexpectedly broadened"):
            VERIFY.product_deny(base, self.policy)


class ExecutionAccountingTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="product-results-control-")
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.junit = self.work / "target/nextest/default/junit.xml"
        self.junit.parent.mkdir(parents=True)

    def test_passing_nonempty_product_selection_is_counted(self):
        self.junit.write_text('<testsuites><testsuite><testcase name="valid_neighbor"/></testsuite></testsuites>')
        result = VERIFY.validate_test_result("product-tests", "", self.work)
        self.assertEqual(result, {"executed": 1, "test_names": ["valid_neighbor"]})

    def test_empty_skipped_failed_and_missing_product_reports_are_incomplete(self):
        for content in ("<testsuites/>", '<testsuites><testcase><skipped/></testcase></testsuites>',
                        '<testsuites><testcase><failure/></testcase></testsuites>'):
            with self.subTest(content=content):
                self.junit.write_text(content)
                with self.assertRaises(VERIFY.ENV.Incomplete):
                    VERIFY.validate_test_result("product-tests", "", self.work)
        self.junit.unlink()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "JUnit accounting"):
            VERIFY.validate_test_result("product-tests", "", self.work)

    def test_doctests_require_execution_and_no_ignored_selection(self):
        self.assertEqual(VERIFY.validate_test_result("doctests", "test result: ok. 2 passed; 0 failed; 0 ignored;", self.work),
                         {"executed": 2})
        for output in ("test result: ok. 0 passed; 0 failed; 0 ignored;",
                       "test result: ok. 1 passed; 0 failed; 1 ignored;", "compiled successfully"):
            with self.subTest(output=output), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.validate_test_result("doctests", output, self.work)

    def test_windows_row_links_without_attempting_native_execution(self):
        rows = dict(VERIFY.commands("windows"))
        self.assertIn("build", rows["windows-link"])
        self.assertIn("x86_64-pc-windows-msvc", rows["windows-link"])
        self.assertNotIn("run", rows["windows-link"])
        self.assertIn("--bin", rows["windows-link"])

    def test_portable_row_keeps_doctests_separate_and_empty_tests_fatal(self):
        rows = dict(VERIFY.commands("portable"))
        self.assertIn("--no-tests=fail", rows["product-tests"])
        self.assertIn("--tests", rows["product-tests"])
        self.assertIn("--doc", rows["doctests"])
        self.assertEqual(set(rows), {"format", "compiler", "clippy", "rustdoc", "product-tests", "doctests"})

    def test_nextest_store_is_writable_without_changing_test_policy(self):
        base = (ROOT / ".config/nextest.toml").read_text()
        result = tomllib.loads(VERIFY.product_nextest(base))
        self.assertEqual(result.pop("store"), {"dir": "/work/target/nextest"})
        self.assertEqual(result, tomllib.loads(base))

    def test_unreviewed_nextest_store_override_is_rejected(self):
        base = (ROOT / ".config/nextest.toml").read_text() + '\n[store]\ndir="/other"\n'
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "Unexpected nextest store"):
            VERIFY.product_nextest(base)

    def test_unknown_row_is_not_a_successful_empty_selection(self):
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "Unknown"):
            VERIFY.commands("not-admitted")

    def test_missing_sandbox_capability_has_no_host_fallback(self):
        with patch.object(VERIFY.ENV.shutil, "which", return_value=None):
            with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "host execution fallback is forbidden"):
                VERIFY.sandbox(Path("/compiler"), self.work, self.work, self.work, ["cargo", "test"], None)


class AdvisoryAdmissionTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="product-advisory-control-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.home = self.root / "cargo"
        for name in ("advisory-dbs/record", "registry/index/record"):
            path = self.home / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("synthetic snapshot bytes")
        self.receipt = self.root / "receipt.json"
        self.lock = "1" * 64
        self.record = {"acquired_unix": 100000, "cargo_home": str(self.home), "files": VERIFY.inventory(self.home),
                       "lockfile_sha256": self.lock}
        self.receipt.write_text(json.dumps(self.record))

    def test_current_complete_snapshot_is_admitted(self):
        self.assertEqual(VERIFY.admit_advisories(self.receipt, 100010, self.lock)[0], self.home)

    def test_stale_future_and_missing_receipts_fail(self):
        for now in (99999, 186401):
            with self.subTest(now=now), self.assertRaisesRegex(VERIFY.ENV.Incomplete, "24 hours"):
                VERIFY.admit_advisories(self.receipt, now, self.lock)
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "current acquisition receipt"):
            VERIFY.admit_advisories(None, 100000, self.lock)

    def test_mutated_snapshot_is_rejected(self):
        (self.home / "advisory-dbs/record").write_text("changed")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "snapshot bytes"):
            VERIFY.admit_advisories(self.receipt, 100010, self.lock)

    def test_snapshot_cannot_supply_cargo_execution_configuration(self):
        (self.home / "config.toml").write_text('[build]\nrustc-wrapper="/untrusted"\n')
        self.record["files"] = VERIFY.inventory(self.home)
        self.receipt.write_text(json.dumps(self.record))
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "snapshot bytes"):
            VERIFY.admit_advisories(self.receipt, 100010, self.lock)

    def test_old_graph_receipt_cannot_admit_a_different_lockfile(self):
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "different product lockfile"):
            VERIFY.admit_advisories(self.receipt, 100010, "2" * 64)


class WindowsLibraryAdmissionTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="product-library-control-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.splat = self.root / "splat"
        self.work = self.root / "work"
        self.work.mkdir()
        self.toolchain = self.root / "toolchain"
        self.policy = json.loads((ROOT / "policy/tool-manifest.json").read_text())
        linker = self.toolchain / "lib/rustlib" / self.policy["rust"]["host"] / "bin/rust-lld"
        linker.parent.mkdir(parents=True)
        linker.write_text("synthetic linker; never executed")
        self.policy["cross_toolchain"]["linker"]["binary_sha256"] = VERIFY.sha256(linker)
        (self.root / "policy").mkdir()
        (self.root / "policy/tool-manifest.json").write_text(json.dumps(self.policy))
        suffixes = ("crt/lib/x86_64", "sdk/lib/um/x86_64", "sdk/lib/ucrt/x86_64")
        for suffix, name in zip(suffixes, ("msvcrt.lib", "kernel32.lib", "ucrt.lib")):
            path = self.splat / suffix / name
            path.parent.mkdir(parents=True)
            path.write_text("synthetic library " + name)
        self.extra_library = self.splat / "sdk/lib/um/x86_64/user32.lib"
        self.extra_library.write_text("independently pinned additional library")
        self.config = self.root / "xwin.json"
        self.config.write_text(json.dumps({"root": str(self.splat),
                                          "library_paths": [str(self.splat / p) for p in suffixes],
                                          "microsoft_inputs": self.policy["cross_toolchain"]["microsoft_inputs"]}))
        self.receipt = self.root / "libraries.json"
        self.receipt.write_text(json.dumps({"files": VERIFY.library_inventory(self.splat),
                                          "acquisition_receipt_sha256": VERIFY.sha256(self.config)}))

    def test_complete_library_tree_and_linker_are_admitted(self):
        directory, files = VERIFY.windows_libraries(self.root, self.toolchain, self.config, self.work, self.receipt)
        self.assertEqual(directory, self.splat)
        self.assertEqual(len(files), 4)
        self.assertTrue((self.work / "linker").is_file())

    def test_unpinned_windows_inputs_fail_before_linker_creation(self):
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "complete library inventory"):
            VERIFY.windows_libraries(self.root, self.toolchain, self.config, self.work, None)
        self.assertFalse((self.work / "linker").exists())

    def test_mutation_outside_three_sample_libraries_is_rejected(self):
        self.extra_library.write_text("substituted user32 library")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "library bytes differ"):
            VERIFY.windows_libraries(self.root, self.toolchain, self.config, self.work, self.receipt)
        self.assertFalse((self.work / "linker").exists())

    def test_inventory_cannot_refer_to_another_acquisition_receipt(self):
        record = json.loads(self.receipt.read_text())
        record["acquisition_receipt_sha256"] = "0" * 64
        self.receipt.write_text(json.dumps(record))
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "different acquisition receipt"):
            VERIFY.windows_libraries(self.root, self.toolchain, self.config, self.work, self.receipt)


if __name__ == "__main__":
    unittest.main()
