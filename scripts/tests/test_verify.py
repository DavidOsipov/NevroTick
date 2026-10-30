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
EXPECTED_PACKAGES = {
    "nevrotick-ui": "crates/nevrotick-ui", "ntp-wire": "crates/ntp-wire",
    "time-domain": "crates/time-domain", "ntp-exchange": "crates/ntp-exchange",
    "time-core": "crates/time-core",
}
EXPECTED_MEMBERS = 'members = ["crates/nevrotick-ui", "crates/ntp-wire", "crates/time-domain", "crates/ntp-exchange", "crates/time-core"]'
EXCHANGE_LOCK = ('[[package]]\nname="ntp-exchange"\nversion="0.0.0"\n'
                 'dependencies=["ntp-wire","time-core","time-domain"]\n')


CORE_LOCK = ('[[package]]\nname="time-core"\nversion="0.0.0"\n'
             'dependencies=["garde","time-domain"]\n')


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
            '[workspace]\n' + EXPECTED_MEMBERS + '\n' +
            (ROOT / "policy/rust-workspace.toml").read_text())
        for package, directory in EXPECTED_PACKAGES.items():
            member = self.root / directory
            (member / "src").mkdir(parents=True)
            (member / "Cargo.toml").write_text(
                f'[package]\nname="{package}"\nversion="0.0.0"\n[lints]\nworkspace=true\n')
            if package in {"time-domain", "time-core"}:
                with (member / "Cargo.toml").open("a") as stream:
                    stream.write('[dependencies]\ngarde={version="=0.23.0",default-features=false,features=["derive"]}\n')
                    if package == "time-core":
                        stream.write('time-domain={version="=0.0.0",path="../time-domain"}\n')
            if package == "ntp-exchange":
                with (member / "Cargo.toml").open("a") as stream:
                    stream.write('[dependencies]\nntp-wire={version="=0.0.0",path="../ntp-wire"}\n'
                                 'time-domain={version="=0.0.0",path="../time-domain"}\n'
                                 '[dev-dependencies]\ntime-core={version="=0.0.0",path="../time-core"}\n')
            (member / "src/lib.rs").write_text('//! Test control; never compiled.\n')
        self.archive_hash = "1" * 64
        (self.root / "Cargo.lock").write_text(
            'version = 4\n' + CORE_LOCK + '[[package]]\nname="nevrotick-ui"\nversion="0.0.0"\n'
            '[[package]]\nname="ntp-wire"\nversion="0.0.0"\n'
            '[[package]]\nname="time-domain"\nversion="0.0.0"\ndependencies=["garde"]\n' + EXCHANGE_LOCK +
            '[[package]]\nname="garde"\nversion="0.23.0"\n'
            'source="registry+https://github.com/rust-lang/crates.io-index"\n'
            'checksum="' + "2" * 64 + '"\n'
            '[[package]]\nname="fixture-dependency"\nversion="1.0.0"\n'
            'source="registry+https://github.com/rust-lang/crates.io-index"\n'
            f'checksum="{self.archive_hash}"\n')
        self.policy = {"schema_version": 1,
                       "lockfile_sha256": VERIFY.sha256(self.root / "Cargo.lock"),
                       "manifests_sha256": {name: VERIFY.sha256(self.root / name)
                                            for name in ("Cargo.toml", "crates/nevrotick-ui/Cargo.toml",
                                                         "crates/ntp-wire/Cargo.toml", "crates/time-domain/Cargo.toml",
                                                         "crates/ntp-exchange/Cargo.toml", "crates/time-core/Cargo.toml")},
                       "packages": [{"name": "fixture-dependency", "version": "1.0.0",
                                     "checksum": self.archive_hash},
                                    {"name": "garde", "version": "0.23.0", "checksum": "2" * 64}]}
        self.write_policy()

    def write_policy(self):
        (self.root / "policy/ui-m0-dependencies.json").write_text(json.dumps(self.policy))

    def test_build_and_lint_rows_use_the_twelve_job_budget(self):
        for lane in ("portable", "windows"):
            for label, command in VERIFY.commands(lane):
                if label == "format":
                    continue
                with self.subTest(lane=lane, row=label):
                    flag = "--build-jobs" if "nextest" in command else "--jobs"
                    self.assertEqual(command[command.index(flag) + 1], "12")

    def test_prior_single_member_graph_no_longer_hides_missing_protocol_member(self):
        path = self.root / "Cargo.toml"
        path.write_text(path.read_text().replace(
            EXPECTED_MEMBERS,
            'members = ["crates/nevrotick-ui"]'))
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "reviewed.*workspace members"):
            VERIFY.admit_product(self.root)

    def test_prior_two_member_graph_cannot_hide_the_domain(self):
        path = self.root / "Cargo.toml"
        path.write_text(path.read_text().replace(
            EXPECTED_MEMBERS,
            'members = ["crates/nevrotick-ui", "crates/ntp-wire"]'))
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "reviewed.*workspace members"):
            VERIFY.admit_product(self.root)

    def test_reviewed_time_domain_graph_is_accepted(self):
        _, files = VERIFY.admit_product(self.root)
        self.assertIn("crates/time-domain/src/lib.rs", files)
        self.assertIn("crates/ntp-wire/src/lib.rs", files)
        self.assertIn("crates/nevrotick-ui/src/lib.rs", files)

    def test_reviewed_five_member_graph_is_accepted(self):
        with patch.object(VERIFY.ENV, "run") as run:
            _, files = VERIFY.admit_product(self.root)
            run.assert_not_called()
        for directory in EXPECTED_PACKAGES.values():
            self.assertIn(directory + "/src/lib.rs", files)

    def test_prior_four_member_graph_cannot_hide_the_kernel(self):
        path = self.root / "Cargo.toml"
        path.write_text(path.read_text().replace(EXPECTED_MEMBERS,
            'members = ["crates/nevrotick-ui", "crates/ntp-wire", "crates/time-domain", "crates/ntp-exchange"]'))
        with patch.object(VERIFY.ENV, "run") as run:
            with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "reviewed.*workspace members"):
                VERIFY.admit_product(self.root)
            run.assert_not_called()

    def test_kernel_manifest_faults_reject_even_with_updated_hash(self):
        path = self.root / "crates/time-core/Cargo.toml"
        healthy = path.read_text()
        changes = [healthy.replace('default-features=false', 'default-features=true'),
                   healthy.replace('features=["derive"]', 'features=["full"]'),
                   healthy + '\n[dev-dependencies]\nextra="=1.0.0"\n',
                   healthy.replace('workspace=true', 'workspace=false')]
        for text in changes:
            path.write_text(text)
            self.policy["manifests_sha256"]["crates/time-core/Cargo.toml"] = VERIFY.sha256(path)
            self.write_policy()
            with self.subTest(text=text), patch.object(VERIFY.ENV, "run") as run:
                with self.assertRaises(VERIFY.ENV.Incomplete): VERIFY.admit_product(self.root)
                run.assert_not_called()

    def test_kernel_local_lock_record_is_required_and_exact(self):
        path = self.root / "Cargo.lock"
        healthy = path.read_text()
        for text in [healthy.replace(CORE_LOCK, ''),
                     healthy.replace(CORE_LOCK, CORE_LOCK.replace('["garde","time-domain"]', '[]'))]:
            path.write_text(text)
            self.policy["lockfile_sha256"] = VERIFY.sha256(path)
            self.write_policy()
            with self.subTest(text=text), patch.object(VERIFY.ENV, "run") as run:
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "lock graph"):
                    VERIFY.admit_product(self.root)
                run.assert_not_called()

    def test_prior_three_member_graph_cannot_hide_the_exchange(self):
        path = self.root / "Cargo.toml"
        path.write_text(path.read_text().replace(EXPECTED_MEMBERS,
            'members = ["crates/nevrotick-ui", "crates/ntp-wire", "crates/time-domain"]'))
        lock = self.root / "Cargo.lock"
        lock.write_text(lock.read_text().replace(EXCHANGE_LOCK, ""))
        self.policy["lockfile_sha256"] = VERIFY.sha256(lock)
        self.policy["manifests_sha256"]["Cargo.toml"] = VERIFY.sha256(path)
        del self.policy["manifests_sha256"]["crates/ntp-exchange/Cargo.toml"]
        self.write_policy()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "reviewed.*workspace members"):
            VERIFY.admit_product(self.root)

    def test_missing_manifest_is_incomplete_without_running_a_tool(self):
        (self.root / "Cargo.toml").unlink()
        with patch.object(VERIFY.ENV, "run") as run:
            with self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.admit_product(self.root)
            run.assert_not_called()

    def test_unreviewed_member_cannot_hide_behind_green_ui_tests(self):
        path = self.root / "Cargo.toml"
        path.write_text(path.read_text().replace(
            EXPECTED_MEMBERS, EXPECTED_MEMBERS[:-1] + ', "other"]'))
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "reviewed.*workspace members"):
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

    def test_protocol_features_and_dependencies_are_not_admitted_even_with_new_hash(self):
        path = self.root / "crates/ntp-wire/Cargo.toml"
        original = path.read_text()
        for extra in ('\n[features]\nunreviewed=[]\n', '\n[dependencies]\nfixture-dependency="=1.0.0"\n',
                      '\n[lib]\nproc-macro=true\n', '\n[target.\'cfg(windows)\'.dependencies]\nother="1"\n'):
            with self.subTest(extra=extra):
                path.write_text(original + extra)
                self.policy["manifests_sha256"]["crates/ntp-wire/Cargo.toml"] = VERIFY.sha256(path)
                self.write_policy()
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "may not add dependencies"):
                    VERIFY.admit_product(self.root)

    def test_protocol_member_inherits_build_and_source_checks(self):
        protocol = self.root / "crates/ntp-wire"
        for name, content, message in (("build.rs", "fn main() {}", "build script"),
                                       (".cargo/config.toml", "[build]\n", "nested manifest or Cargo")):
            with self.subTest(name=name):
                path = protocol / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(content)
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, message):
                    VERIFY.admit_product(self.root)
                path.unlink()
                if name.startswith(".cargo/"):
                    path.parent.rmdir()
        (protocol / "src/alias.rs").symlink_to("lib.rs")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "Linked or special"):
            VERIFY.admit_product(self.root)

    def test_protocol_lock_dependencies_are_rejected_even_with_new_lock_hash(self):
        path = self.root / "Cargo.lock"
        path.write_text(path.read_text().replace('name="ntp-wire"\nversion="0.0.0"',
                                                'name="ntp-wire"\nversion="0.0.0"\ndependencies=["fixture-dependency"]'))
        self.policy["lockfile_sha256"] = VERIFY.sha256(path)
        self.write_policy()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unexpected packages"):
            VERIFY.admit_product(self.root)

    def test_protocol_lint_policy_and_library_presence_are_mandatory(self):
        path = self.root / "crates/ntp-wire/Cargo.toml"
        original = path.read_text()
        path.write_text(original.replace("workspace=true", "workspace=false"))
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "inherited lints"):
            VERIFY.admit_product(self.root)
        path.write_text(original)
        (self.root / "crates/ntp-wire/src/lib.rs").unlink()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "required product library"):
            VERIFY.admit_product(self.root)

    def test_domain_dependency_profile_is_exact_even_with_new_manifest_hash(self):
        path = self.root / "crates/time-domain/Cargo.toml"
        original = path.read_text()
        declaration = 'garde={version="=0.23.0",default-features=false,features=["derive"]}'
        variants = [original.replace(declaration, value) for value in (
            'garde="=0.23.0"',
            'garde={version="=0.23.1",default-features=false,features=["derive"]}',
            'garde={version="=0.23.0",features=["derive"]}',
            'garde={version="=0.23.0",default-features=true,features=["derive"]}',
            'garde={version="=0.23.0",default-features=0,features=["derive"]}',
            'garde={version="=0.23.0",default-features=false,features=["full"]}',
            'garde={version="=0.23.0",default-features=false,features=["derive","regex"]}',
            'garde={workspace=true}')]
        variants.extend(original + extra for extra in (
            '\nserde="1"\n', '\nntp-wire={path="../ntp-wire"}\n', '\n[features]\nother=[]\n',
            '\n[dev-dependencies]\nserde="1"\n', '\n[build-dependencies]\nserde="1"\n',
            '\n[target.\'cfg(windows)\'.dependencies]\nserde="1"\n'))
        VERIFY.admit_product(self.root)
        for candidate in variants:
            with self.subTest(candidate=candidate):
                path.write_text(candidate)
                self.policy["manifests_sha256"]["crates/time-domain/Cargo.toml"] = VERIFY.sha256(path)
                self.write_policy()
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "requires only exact garde"):
                    VERIFY.admit_product(self.root)

    def test_domain_lock_edge_is_exact_even_with_new_lock_hash(self):
        path = self.root / "Cargo.lock"
        original = path.read_text()
        for edge in ('dependencies=[]', 'dependencies=["garde","fixture-dependency"]',
                     'dependencies=["ntp-wire"]'):
            with self.subTest(edge=edge):
                path.write_text(original.replace('dependencies=["garde"]', edge))
                self.policy["lockfile_sha256"] = VERIFY.sha256(path)
                self.write_policy()
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unexpected packages"):
                    VERIFY.admit_product(self.root)

    def test_domain_cannot_resolve_a_different_garde_version(self):
        path = self.root / "Cargo.lock"
        path.write_text(path.read_text().replace('name="garde"\nversion="0.23.0"',
                                                'name="garde"\nversion="0.23.1"'))
        self.policy["lockfile_sha256"] = VERIFY.sha256(path)
        next(p for p in self.policy["packages"] if p["name"] == "garde")["version"] = "0.23.1"
        self.write_policy()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unexpected packages"):
            VERIFY.admit_product(self.root)

    def test_domain_member_inherits_build_source_and_lint_guards(self):
        domain = self.root / "crates/time-domain"
        build = domain / "build.rs"
        build.write_text("fn main() {}")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "build script"):
            VERIFY.admit_product(self.root)
        build.unlink()
        config = domain / ".cargo/config.toml"
        config.parent.mkdir(); config.write_text("[build]\n")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "nested manifest or Cargo"):
            VERIFY.admit_product(self.root)
        config.unlink(); config.parent.rmdir()
        alias = domain / "src/alias.rs"; alias.symlink_to("lib.rs")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "Linked or special"):
            VERIFY.admit_product(self.root)
        alias.unlink()
        manifest = domain / "Cargo.toml"
        manifest.write_text(manifest.read_text().replace("workspace=true", "workspace=false"))
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "inherited lints"):
            VERIFY.admit_product(self.root)

    def test_exchange_dependency_profile_is_exact_even_with_new_manifest_hash(self):
        path = self.root / "crates/ntp-exchange/Cargo.toml"
        original = path.read_text()
        wire = 'ntp-wire={version="=0.0.0",path="../ntp-wire"}'
        core = 'time-core={version="=0.0.0",path="../time-core"}'
        dev_table = '[dev-dependencies]\n' + core + '\n'
        variants = [original.replace(wire, value) for value in (
            'ntp-wire="=0.0.0"',
            'ntp-wire={version="=0.0.1",path="../ntp-wire"}',
            'ntp-wire={version="=0.0.0",path="../time-domain"}',
            'ntp-wire={version="=0.0.0",path="/outside/ntp-wire"}',
            'ntp-wire={version="=0.0.0",path="../ntp-wire",optional=true}',
            'ntp-wire={version="=0.0.0",path="../ntp-wire",features=["other"]}',
            'ntp-wire={workspace=true}',
            '')]
        variants.extend((original.replace('path="../time-domain"', 'path="../ntp-wire"'),
                         original.replace('name="ntp-exchange"\nversion="0.0.0"',
                                          'name="ntp-exchange"\nversion="0.1.0"'),
                         original.replace(dev_table, ''),
                         original.replace(dev_table, core + '\n'),
                         original.replace('[dev-dependencies]\n', core + '\n[dev-dependencies]\n'),
                         original.replace('[dev-dependencies]', '[build-dependencies]'),
                         original.replace('[dev-dependencies]', "[target.'cfg(windows)'.dev-dependencies]")))
        variants.extend(original.replace(core, value) for value in (
            'time-core="=0.0.0"',
            'time-core={path="../time-core"}',
            'time-core={version="=0.0.1",path="../time-core"}',
            'time-core={version="=0.0.0",path="../time-domain"}',
            'time-core={version="=0.0.0",path="/outside/time-core"}',
            'time-core={version="=0.0.0",path="../time-core",package="time-domain"}',
            'time-core={version="=0.0.0",path="../time-core",optional=true}',
            'time-core={version="=0.0.0",path="../time-core",default-features=false}',
            'time-core={version="=0.0.0",path="../time-core",features=[]}',
            'time-core={workspace=true}',
            ''))
        variants.extend(original + extra for extra in (
            '\ngarde="=0.23.0"\n', '\n[features]\nother=[]\n',
            '\nserde="1"\n', '\n[build-dependencies]\nserde="1"\n',
            '\n[target.\'cfg(windows)\'.dependencies]\nserde="1"\n', '\n[lib]\nproc-macro=true\n'))
        VERIFY.admit_product(self.root)
        for candidate in variants:
            with self.subTest(candidate=candidate):
                path.write_text(candidate)
                self.policy["manifests_sha256"]["crates/ntp-exchange/Cargo.toml"] = VERIFY.sha256(path)
                self.write_policy()
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "requires only exact local"):
                    VERIFY.admit_product(self.root)

    def test_exchange_lock_edges_are_exact_even_with_new_lock_hash(self):
        path = self.root / "Cargo.lock"
        original = path.read_text()
        for edges in ('', 'dependencies=[]\n', 'dependencies=["ntp-wire"]\n',
                      'dependencies=["time-domain"]\n',
                      'dependencies=["ntp-wire","time-domain"]\n',
                      'dependencies=["ntp-wire","time-core"]\n',
                      'dependencies=["ntp-wire","time-core","time-domain","garde"]\n'):
            with self.subTest(edges=edges):
                path.write_text(original.replace(EXCHANGE_LOCK,
                    '[[package]]\nname="ntp-exchange"\nversion="0.0.0"\n' + edges))
                self.policy["lockfile_sha256"] = VERIFY.sha256(path)
                self.write_policy()
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unexpected packages"):
                    VERIFY.admit_product(self.root)

    def test_exchange_member_preserves_build_source_lint_and_presence_guards(self):
        member = self.root / "crates/ntp-exchange"
        VERIFY.admit_product(self.root)
        for name, text, message in (("build.rs", "fn main() {}", "build script"),
                                    (".cargo/config.toml", "[build]\n", "nested manifest or Cargo"),
                                    ("nested/Cargo.toml", "[package]\n", "nested manifest or Cargo")):
            with self.subTest(name=name):
                path = member / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(text)
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, message):
                    VERIFY.admit_product(self.root)
                path.unlink()
                if "/" in name:
                    path.parent.rmdir()
        alias = member / "src/alias.rs"; alias.symlink_to("lib.rs")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "Linked or special"):
            VERIFY.admit_product(self.root)
        alias.unlink()
        manifest = member / "Cargo.toml"; original = manifest.read_text()
        manifest.write_text(original.replace("workspace=true", "workspace=false"))
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "inherited lints"):
            VERIFY.admit_product(self.root)
        manifest.write_text(original)
        (member / "src/lib.rs").unlink()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "required product library"):
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
        self.junit.write_text('<testsuites><testsuite name="nevrotick-ui::preview"><testcase name="valid_neighbor"/></testsuite></testsuites>')
        result = VERIFY.validate_test_result("product-tests:nevrotick-ui", "", self.work)
        self.assertEqual(result, {"package": "nevrotick-ui", "executed": 1, "test_names": ["valid_neighbor"]})

    def test_protocol_test_row_cannot_reuse_only_ui_green(self):
        self.junit.write_text('<testsuites><testsuite name="ntp-wire::header"><testcase name="valid_neighbor"/></testsuite></testsuites>')
        result = VERIFY.validate_test_result("product-tests:ntp-wire", "", self.work)
        self.assertEqual(result, {"package": "ntp-wire", "executed": 1, "test_names": ["valid_neighbor"]})

    def test_optimized_domain_result_requires_actual_discriminating_case(self):
        self.junit.write_text('<testsuites><testsuite name="time-domain::coordinate">'
                              '<testcase name="release_build_preserves_checked_admission"/>'
                              '</testsuite></testsuites>')
        result = VERIFY.validate_test_result("optimized-tests:time-domain", "", self.work)
        self.assertEqual(result, {"package": "time-domain", "executed": 1,
                                  "test_names": ["release_build_preserves_checked_admission"]})
        self.junit.write_text('<testsuites><testsuite name="time-domain::coordinate">'
                              '<testcase name="debug_neighbor"/></testsuite></testsuites>')
        with self.assertRaises(VERIFY.ENV.Incomplete):
            VERIFY.validate_test_result("optimized-tests:time-domain", "", self.work)

    def test_exchange_ordinary_and_optimized_results_require_their_own_observations(self):
        self.junit.write_text('<testsuites><testsuite name="ntp-exchange::association">'
                              '<testcase name="valid_neighbor"/></testsuite></testsuites>')
        result = VERIFY.validate_test_result("product-tests:ntp-exchange", "", self.work)
        self.assertEqual(result, {"package": "ntp-exchange", "executed": 1, "test_names": ["valid_neighbor"]})
        self.junit.write_text('<testsuites><testsuite name="ntp-exchange::association">'
                              '<testcase name="release_build_preserves_exchange_admission"/>'
                              '</testsuite></testsuites>')
        result = VERIFY.validate_test_result("optimized-tests:ntp-exchange", "", self.work)
        self.assertEqual(result["test_names"], ["release_build_preserves_exchange_admission"])
        for package, name in (("ntp-exchange", "valid_neighbor"),
                              ("ntp-exchange", "release_build_preserves_checked_admission"),
                              ("time-domain", "release_build_preserves_exchange_admission")):
            with self.subTest(package=package, name=name):
                self.junit.write_text(f'<testsuites><testsuite name="{package}"><testcase name="{name}"/>'
                                      '</testsuite></testsuites>')
                with self.assertRaises(VERIFY.ENV.Incomplete):
                    VERIFY.validate_test_result("optimized-tests:ntp-exchange", "", self.work)
        self.junit.write_text('<testsuites><testsuite name="time-domain">'
                              '<testcase name="release_build_preserves_exchange_admission"/>'
                              '</testsuite></testsuites>')
        with self.assertRaises(VERIFY.ENV.Incomplete):
            VERIFY.validate_test_result("optimized-tests:time-domain", "", self.work)

    def test_exchange_empty_skipped_failed_and_stale_reports_remain_incomplete(self):
        for xml in ('<testsuites/>', '<testsuites><testsuite name="ntp-exchange"/></testsuites>',
                    '<testsuites><testsuite name="ntp-exchange"><testcase><skipped/></testcase></testsuite></testsuites>',
                    '<testsuites><testsuite name="ntp-exchange"><testcase><failure/></testcase></testsuite></testsuites>'):
            with self.subTest(xml=xml):
                self.junit.write_text(xml)
                with self.assertRaises(VERIFY.ENV.Incomplete):
                    VERIFY.validate_test_result("product-tests:ntp-exchange", "", self.work)
        self.junit.write_text('<testsuites><testsuite name="ntp-exchange">'
                              '<testcase name="release_build_preserves_exchange_admission"/>'
                              '</testsuite></testsuites>')
        VERIFY.clear_test_report("optimized-tests:ntp-exchange", self.work)
        self.assertFalse(self.junit.exists())
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "JUnit accounting"):
            VERIFY.validate_test_result("optimized-tests:ntp-exchange", "", self.work)

    def test_exchange_doctests_are_separate_nonempty_and_have_no_ignored_cases(self):
        self.assertEqual(VERIFY.validate_test_result("doctests:ntp-exchange",
                         "test result: ok. 2 passed; 0 failed; 0 ignored;", self.work),
                         {"package": "ntp-exchange", "executed": 2})
        for text in ("", "test result: ok. 0 passed; 0 failed; 0 ignored;",
                     "test result: ok. 2 passed; 0 failed; 1 ignored;"):
            with self.subTest(text=text), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.validate_test_result("doctests:ntp-exchange", text, self.work)

    def test_prior_three_package_results_cannot_hide_the_exchange(self):
        records = [{"lane": kind + ":" + package, "tests": {"package": package, "executed": 2}}
                   for package in ("nevrotick-ui", "ntp-wire", "time-domain")
                   for kind in ("product-tests", "doctests")]
        records.append({"lane": "optimized-tests:time-domain", "tests": {
            "package": "time-domain", "executed": 1,
            "test_names": ["release_build_preserves_checked_admission"]}})
        with self.assertRaises(VERIFY.ENV.Incomplete):
            VERIFY.summarize_test_counts(records)

    def test_kernel_commands_and_release_discriminator_are_required(self):
        rows = dict(VERIFY.commands("portable"))
        for lane in ("product-tests", "doctests", "optimized-tests"):
            command = rows[lane + ":time-core"]
            self.assertEqual(command[command.index("--package") + 1], "time-core")
        command = rows["optimized-tests:time-core"]
        self.assertEqual(command[command.index("--cargo-profile") + 1], "release")
        self.assertIn("--no-tests=fail", command)
        self.junit.write_text('<testsuites><testsuite name="time-core">'
                              '<testcase name="ordinary_neighbor"/></testsuite></testsuites>')
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "discriminator"):
            VERIFY.validate_test_result("optimized-tests:time-core", "", self.work)
        self.junit.write_text('<testsuites><testsuite name="time-core">'
                              '<testcase name="release_build_preserves_regression_admission"/>'
                              '</testsuite></testsuites>')
        self.assertEqual(VERIFY.validate_test_result("optimized-tests:time-core", "", self.work)["executed"], 1)

    def test_exchange_commands_require_release_and_ordinary_execution(self):
        rows = dict(VERIFY.commands("portable"))
        self.assertIn("product-tests:ntp-exchange", rows)
        self.assertIn("doctests:ntp-exchange", rows)
        command = rows["optimized-tests:ntp-exchange"]
        self.assertEqual(command[command.index("--cargo-profile") + 1], "release")
        self.assertEqual(command[command.index("--package") + 1], "ntp-exchange")
        self.assertIn("--no-tests=fail", command)

    def test_prior_two_package_results_cannot_hide_missing_numerical_owner(self):
        records = [{"lane": kind + ":" + package, "tests": {"package": package, "executed": 2}}
                   for package in ("nevrotick-ui", "ntp-wire")
                   for kind in ("product-tests", "doctests")]
        with self.assertRaises(VERIFY.ENV.Incomplete):
            VERIFY.summarize_test_counts(records)

    def test_domain_commands_require_release_and_ordinary_execution(self):
        rows = dict(VERIFY.commands("portable"))
        self.assertIn("product-tests:time-domain", rows)
        self.assertIn("doctests:time-domain", rows)
        command = rows["optimized-tests:time-domain"]
        self.assertEqual(command[command.index("--cargo-profile") + 1], "release")
        self.assertIn("--no-tests=fail", command)

    def test_protocol_package_commands_are_explicit_and_nonempty(self):
        rows = dict(VERIFY.commands("portable"))
        for package in ("nevrotick-ui", "ntp-wire", "time-domain", "ntp-exchange", "time-core"):
            with self.subTest(package=package):
                self.assertIn("--no-tests=fail", rows["product-tests:" + package])
                command = rows["product-tests:" + package]
                self.assertEqual(command[command.index("--package") + 1], package)
                self.assertIn("--doc", rows["doctests:" + package])

    def test_empty_skipped_failed_and_missing_product_reports_are_incomplete(self):
        for content in ("<testsuites/>",
                        '<testsuites><testsuite name="ntp-wire"><testcase><skipped/></testcase></testsuite></testsuites>',
                        '<testsuites><testsuite name="ntp-wire"><testcase><failure/></testcase></testsuite></testsuites>',
                        '<testsuites><testsuite name="nevrotick-ui"><testcase/></testsuite></testsuites>',
                        '<testsuites><testcase/></testsuites>'):
            with self.subTest(content=content):
                self.junit.write_text(content)
                with self.assertRaises(VERIFY.ENV.Incomplete):
                    VERIFY.validate_test_result("product-tests:ntp-wire", "", self.work)
        self.junit.unlink()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "JUnit accounting"):
                VERIFY.validate_test_result("product-tests:ntp-wire", "", self.work)

    def test_doctests_require_execution_and_no_ignored_selection(self):
        self.assertEqual(VERIFY.validate_test_result("doctests:ntp-wire", "test result: ok. 2 passed; 0 failed; 0 ignored;", self.work),
                         {"package": "ntp-wire", "executed": 2})
        for output in ("test result: ok. 0 passed; 0 failed; 0 ignored;",
                       "test result: ok. 1 passed; 0 failed; 1 ignored;", "compiled successfully"):
            with self.subTest(output=output), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.validate_test_result("doctests:ntp-wire", output, self.work)

    def test_windows_row_links_without_attempting_native_execution(self):
        rows = dict(VERIFY.commands("windows"))
        self.assertIn("build", rows["windows-link"])
        self.assertIn("--release", rows["windows-link"])
        self.assertIn("x86_64-pc-windows-msvc", rows["windows-link"])
        self.assertNotIn("run", rows["windows-link"])
        self.assertIn("--bin", rows["windows-link"])

    def test_windows_export_selects_release_and_preserves_matching_symbols(self):
        release = self.work / "target/x86_64-pc-windows-msvc/release"
        debug = self.work / "target/x86_64-pc-windows-msvc/debug"
        release.mkdir(parents=True)
        debug.mkdir(parents=True)
        (release / "nevrotick-ui.exe").write_bytes(b"release-control")
        (release / "nevrotick_ui.pdb").write_bytes(b"release-symbol-control")
        (debug / "nevrotick-ui.exe").write_bytes(b"wrong-debug-control")
        output = self.work / "export"
        output.mkdir()
        report = VERIFY.export_windows_artifacts(self.work, output)
        self.assertEqual(report["profile"], "release")
        self.assertEqual(report["target"], "x86_64-pc-windows-msvc")
        self.assertEqual((output / "nevrotick-ui.exe").read_bytes(), b"release-control")
        self.assertEqual(report["sha256"], digest(b"release-control"))
        self.assertEqual(report["symbols"]["sha256"], digest(b"release-symbol-control"))
        self.assertEqual(Path(report["symbols"]["path"]).read_bytes(), b"release-symbol-control")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "existing destinations"):
            VERIFY.export_windows_artifacts(self.work, output)

    def test_windows_export_refuses_missing_or_ambiguous_symbols_before_copying(self):
        release = self.work / "target/x86_64-pc-windows-msvc/release"
        release.mkdir(parents=True)
        (release / "nevrotick-ui.exe").write_bytes(b"release-control")
        output = self.work / "export"
        output.mkdir()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "matching PDB"):
            VERIFY.export_windows_artifacts(self.work, output)
        self.assertEqual(list(output.iterdir()), [])
        for name in ("nevrotick-ui.pdb", "nevrotick_ui.pdb"):
            (release / name).write_bytes(b"ambiguous-symbol-control")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "matching PDB"):
            VERIFY.export_windows_artifacts(self.work, output)
        self.assertEqual(list(output.iterdir()), [])

    def test_portable_row_keeps_doctests_separate_and_empty_tests_fatal(self):
        rows = dict(VERIFY.commands("portable"))
        self.assertIn("--workspace", rows["compiler"])
        for package in VERIFY.PACKAGES:
            self.assertIn("--no-tests=fail", rows["product-tests:" + package])
            self.assertIn("--tests", rows["product-tests:" + package])
            self.assertIn("--doc", rows["doctests:" + package])
        self.assertEqual(set(rows), {"format", "compiler", "clippy", "rustdoc",
                                    "product-tests:nevrotick-ui", "product-tests:ntp-wire", "product-tests:time-domain",
                                    "product-tests:ntp-exchange", "product-tests:time-core",
                                    "doctests:nevrotick-ui", "doctests:ntp-wire", "doctests:time-domain",
                                    "doctests:ntp-exchange", "doctests:time-core", "optimized-tests:time-domain",
                                    "optimized-tests:ntp-exchange", "optimized-tests:time-core"})

    def test_ui_report_is_removed_before_protocol_test_command(self):
        self.junit.write_text('<testsuites><testsuite name="nevrotick-ui"><testcase/></testsuite></testsuites>')
        VERIFY.clear_test_report("product-tests:ntp-wire", self.work)
        self.assertFalse(self.junit.exists())
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "JUnit accounting"):
            VERIFY.validate_test_result("product-tests:ntp-wire", "", self.work)

    def test_ordinary_report_cannot_stand_in_for_optimized_execution(self):
        self.junit.write_text('<testsuites><testsuite name="time-domain"><testcase name="debug_neighbor"/>'
                              '</testsuite></testsuites>')
        VERIFY.clear_test_report("optimized-tests:time-domain", self.work)
        self.assertFalse(self.junit.exists())
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "JUnit accounting"):
            VERIFY.validate_test_result("optimized-tests:time-domain", "", self.work)

    def test_optimized_discriminator_from_a_different_package_is_rejected(self):
        self.junit.write_text('<testsuites><testsuite name="ntp-wire">'
                              '<testcase name="release_build_preserves_checked_admission"/>'
                              '</testsuite></testsuites>')
        with self.assertRaises(VERIFY.ENV.Incomplete):
            VERIFY.validate_test_result("optimized-tests:time-domain", "", self.work)
        with self.assertRaises(VERIFY.ENV.Incomplete):
            VERIFY.validate_test_result("optimized-tests:ntp-wire", "", self.work)

    def test_linked_test_report_is_rejected_without_removing_its_target(self):
        target = self.work / "other-report"
        target.write_text("retained control")
        self.junit.symlink_to(target)
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "linked product test report"):
            VERIFY.clear_test_report("product-tests:ntp-wire", self.work)
        self.assertEqual(target.read_text(), "retained control")

    def test_package_and_aggregate_accounting_require_both_test_layers(self):
        records = [{"lane": kind + ":" + package, "tests": {"package": package, "executed": count}}
                   for package, count in (("nevrotick-ui", 3), ("ntp-wire", 2), ("time-domain", 4), ("ntp-exchange", 6), ("time-core", 8))
                   for kind in ("product-tests", "doctests")]
        records.append({"lane": "optimized-tests:time-domain", "tests": {
            "package": "time-domain", "executed": 5,
            "test_names": ["release_build_preserves_checked_admission"]}})
        records.append({"lane": "optimized-tests:ntp-exchange", "tests": {
            "package": "ntp-exchange", "executed": 7,
            "test_names": ["release_build_preserves_exchange_admission"]}})
        records.append({"lane": "optimized-tests:time-core", "tests": {
            "package": "time-core", "executed": 9,
            "test_names": ["release_build_preserves_regression_admission"]}})
        self.assertEqual(VERIFY.summarize_test_counts(records),
                         {"by_package": {"nevrotick-ui": {"product-tests": 3, "doctests": 3},
                                         "ntp-wire": {"product-tests": 2, "doctests": 2},
                                         "time-domain": {"product-tests": 4, "doctests": 4, "optimized-tests": 5},
                                         "ntp-exchange": {"product-tests": 6, "doctests": 6, "optimized-tests": 7},
                                         "time-core": {"product-tests": 8, "doctests": 8, "optimized-tests": 9}},
                          "tests_executed": 23, "doctests_executed": 23, "optimized_tests_executed": 21})
        for incomplete in (records[:-1], records[:-2] + records[-1:], records[:2], records + records[:1],
                           [{"lane": "product-tests:ntp-wire", "tests": {"package": "nevrotick-ui", "executed": 99}}]):
            with self.subTest(records=incomplete), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.summarize_test_counts(incomplete)

    def test_unreviewed_package_cannot_supply_test_accounting(self):
        for lane in ("product-tests:other", "doctests:other"):
            with self.subTest(lane=lane), self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unreviewed package"):
                VERIFY.validate_test_result(lane, "test result: ok. 1 passed; 0 failed; 0 ignored;", self.work)

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
