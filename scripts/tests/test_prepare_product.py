"""Independent archive/admission controls for product acquisition; no network or Cargo."""
import contextlib
import copy
import gzip
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


def admission_archive(name, version):
    """Create independent bounded source bytes for an exact identity, without Cargo."""
    files = {"Cargo.toml": f'[package]\nname="{name}"\nversion="{version}"\n'.encode(),
             "src/lib.rs": b"//! Inert mocked-acquisition control, never compiled.\n"}
    stream = io.BytesIO()
    with tarfile.open(fileobj=stream, mode="w:gz") as archive:
        for relative, contents in files.items():
            member = tarfile.TarInfo(f"{name}-{version}/{relative}")
            member.size = len(contents)
            archive.addfile(member, io.BytesIO(contents))
    data = stream.getvalue()
    return {"name": name, "version": version, "checksum": sha(data),
            "files_sha256": file_map_digest(files)}, data, files


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

    def test_reviewed_expansion_requires_exact_name_version_and_both_pins(self):
        data = self.bundle()
        identity = tuple(self.package[key] for key in ("name", "version", "checksum", "files_sha256"))
        limits = (len(gzip.decompress(data)), sum(map(len, self.files.values())))
        with patch.object(PREPARE, "MAX_EXPANDED", 1024), \
                patch.dict(PREPARE.REVIEWED_EXPANSION_LIMITS, {identity: limits}, clear=True):
            self.assertEqual(PREPARE.archive_files(data, self.package), self.files)
            for key, value in (("name", "other"), ("version", "1.0.1"),
                               ("checksum", "0" * 64), ("files_sha256", "0" * 64)):
                with self.subTest(changed=key):
                    changed = dict(self.package, **{key: value})
                    with self.assertRaisesRegex(ValueError, "identity or size|metadata expansion"):
                        PREPARE.archive_files(data, changed)

    def test_reviewed_expansion_preserves_tar_payload_and_member_ceilings(self):
        data = self.bundle()
        identity = tuple(self.package[key] for key in ("name", "version", "checksum", "files_sha256"))
        tar_size, payload_size = len(gzip.decompress(data)), sum(map(len, self.files.values()))
        for limits, members, error in (
                ((tar_size - 1, payload_size), PREPARE.MAX_MEMBERS, "metadata expansion"),
                ((tar_size, payload_size - 1), PREPARE.MAX_MEMBERS, "archive expansion"),
                ((tar_size, payload_size), 1, "member count")):
            with self.subTest(limits=limits, members=members), \
                    patch.object(PREPARE, "MAX_EXPANDED", 1024), \
                    patch.object(PREPARE, "MAX_MEMBERS", members), \
                    patch.dict(PREPARE.REVIEWED_EXPANSION_LIMITS, {identity: limits}, clear=True):
                with self.assertRaisesRegex(ValueError, error):
                    PREPARE.archive_files(data, self.package)

    def test_large_windows_allowance_matches_only_the_reviewed_identities(self):
        policy = json.loads((ROOT / "policy/ui-m0-dependencies.json").read_text())
        package = next(item for item in policy["packages"]
                       if item["name"] == "windows" and item["version"] == "0.62.2")
        identity = tuple(package[key] for key in ("name", "version", "checksum", "files_sha256"))
        self.assertEqual(PREPARE.REVIEWED_EXPANSION_LIMITS,
                         {identity: (114_755_072, 114_212_919),
                          ("windows", "0.61.3",
                           "9babd3a767a4c1aef6900409f85f5d53ce2544ccdfaa86dad48c91782c6d6893",
                           "47ccf625f7c4d2bcb7c8721cd038491ffd48469f5beb430ec39195e7dad40601"):
                              (114_900_480, 114_341_344)})

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
        # These archive/copy controls isolate already-admitted inputs. The real
        # age boundary has separate entry-point controls below.
        age = patch.object(PREPARE.VERIFY, "require_root_cooldown_admission", return_value={"fixture": True})
        age.start()
        self.addCleanup(age.stop)

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


class AcquisitionIdentityControls(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="product-identity-control-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.vendor = self.root / "vendor"
        age = patch.object(PREPARE.VERIFY, "require_root_cooldown_admission", return_value={"fixture": True})
        age.start()
        self.addCleanup(age.stop)

    def test_large_source_inventory_is_finite_and_prechecked(self):
        template, _, _ = admission_archive("bounded-control", "1.0.0")
        packages = [dict(template, name=f"package-{index}") for index in range(537)]
        self.assertEqual(len(PREPARE.acquisition_records({"packages": packages})), 537)
        oversized = [dict(template, name=f"package-{index}") for index in range(769)]
        with patch.object(PREPARE.urllib.request, "urlopen") as network:
            with self.assertRaisesRegex(ValueError, "bound"):
                PREPARE.acquire(self.vendor, {"packages": oversized})
            network.assert_not_called()

    def test_aggregate_archive_and_source_budgets_leave_no_partial_vendor(self):
        first, data, files = admission_archive("first", "1.0.0")
        second, other_data, _ = admission_archive("second", "1.0.0")
        for budget_name, budget in (("MAX_TOTAL_ARCHIVE", len(data)),
                                    ("MAX_TOTAL_EXPANDED", sum(map(len, files.values())))):
            vendor = self.root / budget_name
            with self.subTest(budget=budget_name), \
                    patch.object(PREPARE, budget_name, budget, create=True), \
                    patch.object(PREPARE.urllib.request, "urlopen",
                                 side_effect=[io.BytesIO(data), io.BytesIO(other_data)]):
                with self.assertRaisesRegex(ValueError, "Aggregate"):
                    PREPARE.acquire(vendor, {"packages": [first, second]})
                self.assertFalse(vendor.exists())
                self.assertFalse(list(self.root.glob(".ui-m0-acquire-*")))

    def test_metadata_build_version_is_acquired_without_a_selected_grant(self):
        package, data, files = admission_archive("wasi", "0.11.1+wasi-snapshot-preview1")
        package["role"] = "metadata-only"
        policy = {"packages": [], "metadata_only_packages": [package]}
        before = copy.deepcopy(policy)
        with patch.object(PREPARE.urllib.request, "urlopen", return_value=io.BytesIO(data)) as network:
            PREPARE.acquire(self.vendor, policy)
        self.assertEqual(network.call_count, 1)
        self.assertEqual(network.call_args.args[0].full_url,
                         "https://static.crates.io/crates/wasi/wasi-0.11.1+wasi-snapshot-preview1.crate")
        directory = self.vendor / "wasi-0.11.1+wasi-snapshot-preview1"
        self.assertEqual((directory / "src/lib.rs").read_bytes(), files["src/lib.rs"])
        checksum = json.loads((directory / ".cargo-checksum.json").read_text())
        self.assertEqual(checksum, {"package": package["checksum"],
                                   "files": {name: sha(value) for name, value in files.items()}})
        self.assertEqual(policy, before)
        self.assertNotIn("selected_license", policy["metadata_only_packages"][0])
        self.assertNotIn("build_script", policy["metadata_only_packages"][0])

    def test_unsafe_or_non_semver_versions_fail_before_any_download(self):
        package, _, _ = admission_archive("bounded-control", "1.0.0")
        versions = ("1.0.0+", "1.0.0+a..b", "01.0.0", "1.0.0+../../outside",
                    "1.0.0+a/b", "1.0.0+a\\b", "1.0.0+a%2Fb", "1.0.0+a?b",
                    "1.0.0+a#b", "1.0.0+a:b", "1.0.0+white space", "\u0661.0.0", 1)
        for version in versions:
            with self.subTest(version=version):
                rejected = dict(package, version=version)
                with patch.object(PREPARE.urllib.request, "urlopen") as network:
                    with self.assertRaisesRegex(ValueError, "package identity"):
                        PREPARE.acquire(self.vendor, {"packages": [rejected]})
                    network.assert_not_called()
                self.assertFalse(self.vendor.exists())
                self.assertFalse(list(self.root.glob(".ui-m0-acquire-*")))

    def test_late_unsafe_metadata_identity_prevents_earlier_download(self):
        package, data, _ = admission_archive("bounded-control", "1.0.0")
        unsafe = dict(package, name="../outside", role="metadata-only")
        with patch.object(PREPARE.urllib.request, "urlopen", return_value=io.BytesIO(data)) as network:
            with self.assertRaisesRegex(ValueError, "package identity"):
                PREPARE.acquire(self.vendor, {"packages": [package],
                                             "metadata_only_packages": [unsafe]})
            network.assert_not_called()
        self.assertFalse(self.vendor.exists())


class CompletePolicyAcquisitionControls(unittest.TestCase):
    """Legacy source/archive CLI controls; graph-age admission is isolated here."""

    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="product-policy-acquisition-control-")
        self.addCleanup(temporary.cleanup)
        self.directory = Path(temporary.name)
        self.checkout = self.directory / "checkout"
        (self.checkout / "policy").mkdir(parents=True)
        self.vendor = self.directory / "vendor"
        age = patch.object(PREPARE.VERIFY, "require_root_cooldown_admission", return_value={"fixture": True})
        age.start()
        self.addCleanup(age.stop)
        self.policy = json.loads((ROOT / "scripts/tests/fixtures/ui-m0-schema2.json").read_text())
        self.assertEqual(self.policy["schema_version"], 2)
        # Inert shape pin for the adopted seventh member, not workspace admission.
        self.policy["manifests_sha256"]["crates/windows-platform/Cargo.toml"] = "c" * 64
        self.responses = {}
        self.sources = {}
        for package in self.policy["packages"] + self.policy["metadata_only_packages"]:
            pins, data, files = admission_archive(package["name"], package["version"])
            package.update(checksum=pins["checksum"], files_sha256=pins["files_sha256"])
            self.responses[f"https://static.crates.io/crates/{package['name']}/{package['name']}-{package['version']}.crate"] = data
            self.sources[f"{package['name']}-{package['version']}"] = files
        self.lock_packages = copy.deepcopy(self.policy["packages"] + self.policy["metadata_only_packages"])
        self.write_inputs()

    def write_inputs(self):
        lock = "version = 4\n"
        for package in self.lock_packages:
            lock += ("\n[[package]]\n"
                     f"name = {json.dumps(package['name'])}\n"
                     f"version = {json.dumps(package['version'])}\n"
                     'source = "registry+https://github.com/rust-lang/crates.io-index"\n'
                     f"checksum = {json.dumps(package['checksum'])}\n")
        (self.checkout / "Cargo.lock").write_text(lock)
        self.policy["lockfile_sha256"] = sha(lock.encode())
        (self.checkout / "policy/ui-m0-dependencies.json").write_text(json.dumps(self.policy))

    def invoke_main(self):
        def response(request, **_kwargs):
            return io.BytesIO(self.responses[request.full_url])
        output = io.StringIO()
        with patch.object(PREPARE, "ROOT", self.checkout), \
                patch("sys.argv", ["prepare-product.py", "--vendor-dir", str(self.vendor)]), \
                patch.object(PREPARE.urllib.request, "urlopen", side_effect=response) as network, \
                contextlib.redirect_stdout(output):
            code = PREPARE.main()
        return code, network, output.getvalue()

    def test_main_acquires_all_23_selected_and_two_resolution_only_sources(self):
        code, network, output = self.invoke_main()
        self.assertEqual(code, 0, output)
        self.assertEqual(network.call_count, 25)
        self.assertEqual({call.args[0].full_url for call in network.call_args_list}, set(self.responses))
        self.assertEqual({path.name for path in self.vendor.iterdir()}, set(self.sources))
        for name, files in self.sources.items():
            self.assertEqual((self.vendor / name / "src/lib.rs").read_bytes(), files["src/lib.rs"])
            checksum = json.loads((self.vendor / name / ".cargo-checksum.json").read_text())
            self.assertEqual(checksum["files"], {relative: sha(data) for relative, data in files.items()})
        self.assertEqual(len(self.policy["packages"]), 23)
        self.assertEqual({(p["name"], p["version"], p["role"]) for p in self.policy["metadata_only_packages"]},
                         {("r-efi", "6.0.0", "metadata-only"),
                          ("wasi", "0.11.1+wasi-snapshot-preview1", "metadata-only")})

    def test_missing_metadata_lock_record_rejects_before_download(self):
        self.lock_packages = [p for p in self.lock_packages if p["name"] != "wasi"]
        self.write_inputs()
        code, network, output = self.invoke_main()
        self.assertEqual(code, 1)
        self.assertIn("package inventory differs from Cargo.lock", output)
        network.assert_not_called()
        self.assertFalse(self.vendor.exists())

    def test_metadata_role_promotion_rejects_before_download(self):
        self.policy["metadata_only_packages"][0]["selected_license"] = "MIT"
        self.write_inputs()
        code, network, output = self.invoke_main()
        self.assertEqual(code, 1)
        self.assertIn("role promotion", output)
        network.assert_not_called()
        self.assertFalse(self.vendor.exists())

    def test_unexpected_metadata_identity_rejects_before_download(self):
        self.policy["metadata_only_packages"][0]["name"] = "unreviewed-resolution-node"
        self.write_inputs()
        code, network, output = self.invoke_main()
        self.assertEqual(code, 1)
        self.assertIn("missing or unexpected packages", output)
        network.assert_not_called()
        self.assertFalse(self.vendor.exists())


class CooldownPublicationControls(unittest.TestCase):
    """Exercise the actual graph boundary, without granting fixture age approval."""

    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="cooldown-publication-")
        self.addCleanup(temporary.cleanup)
        self.directory = Path(temporary.name)
        self.vendor = self.directory / "vendor"
        self.policy = PREPARE.VERIFY.read_dependency_policy(ROOT / "policy/ui-m0-dependencies.json")

    def checkout(self):
        root = self.directory / "checkout"
        names = {*self.policy["manifests_sha256"], "Cargo.lock", ".cargo/config.toml",
                 "policy/tool-manifest.json", "policy/ui-m0-dependencies.json"}
        for name in names:
            target = root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / name).read_bytes())
        return root

    def test_direct_archive_call_cannot_bypass_missing_age_decision(self):
        policy = copy.deepcopy(self.policy)
        policy.pop("cooldown_admission")
        with patch.object(PREPARE.urllib.request, "urlopen") as network:
            with self.assertRaises(PREPARE.VERIFY.ENV.Incomplete):
                PREPARE.acquire(self.vendor, policy)
            network.assert_not_called()
        self.assertFalse(self.vendor.exists())

    def test_accepted_baseline_reaches_transport_without_retroactive_age_claim(self):
        with patch.object(PREPARE.urllib.request, "urlopen", side_effect=OSError("inert transport sentinel")) as network:
            with self.assertRaisesRegex(OSError, "inert transport sentinel"):
                PREPARE.acquire(self.vendor, self.policy)
            network.assert_called_once()
        self.assertEqual(self.policy["cooldown_admission"]["status"], "accepted-baseline")
        self.assertFalse(self.vendor.exists())
        self.assertFalse(list(self.directory.glob(".ui-m0-acquire-*")))

    def test_manually_changed_lock_cannot_publish_as_accepted_baseline(self):
        root = self.checkout()
        path = root / "Cargo.lock"
        path.write_bytes(path.read_bytes() + b"\n# imported candidate, not the approved lock\n")
        with patch.object(PREPARE, "ROOT", root), patch.object(PREPARE.urllib.request, "urlopen") as network:
            with self.assertRaises(PREPARE.VERIFY.ENV.Incomplete):
                PREPARE.acquire(self.vendor, self.policy)
            network.assert_not_called()
        self.assertFalse(self.vendor.exists())

    def test_configuration_drift_after_download_cannot_publish_vendor(self):
        root = self.checkout()
        package, data, _files = admission_archive("bounded-control", "1.0.0")
        def response(*_args, **_kwargs):
            path = root / ".cargo/config.toml"
            path.write_text(path.read_text().replace('"14 days"', '"0"'))
            return io.BytesIO(data)
        with (patch.object(PREPARE, "ROOT", root),
              patch.object(PREPARE, "acquisition_records", return_value=[package]),
              patch.object(PREPARE.urllib.request, "urlopen", side_effect=response) as network):
            with self.assertRaises(PREPARE.VERIFY.ENV.Incomplete):
                PREPARE.acquire(self.vendor, self.policy)
            network.assert_called_once()
        self.assertFalse(self.vendor.exists())
        self.assertFalse(list(self.directory.glob(".ui-m0-acquire-*")))


if __name__ == "__main__":
    unittest.main()
