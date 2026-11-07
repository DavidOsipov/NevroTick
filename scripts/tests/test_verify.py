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
    "time-core": "crates/time-core", "nevrotick-engine": "crates/nevrotick-engine",
    "windows-platform": "crates/windows-platform",
}
EXPECTED_MEMBERS = 'members = ["crates/nevrotick-ui", "crates/ntp-wire", "crates/time-domain", "crates/ntp-exchange", "crates/time-core", "crates/nevrotick-engine", "crates/windows-platform"]'
EXCHANGE_LOCK = ('[[package]]\nname="ntp-exchange"\nversion="0.0.0"\n'
                 'dependencies=["ntp-wire","time-core","time-domain"]\n')


CORE_LOCK = ('[[package]]\nname="time-core"\nversion="0.0.0"\n'
             'dependencies=["garde","time-domain"]\n')


ENGINE_LOCK = ('[[package]]\nname="nevrotick-engine"\nversion="0.0.0"\n'
               'dependencies=["garde","getrandom","ntp-wire","time-domain","tokio"]\n')
# Independent fixture identities: no production helper supplies these expectations.
SELECTED_VERSIONS = {
    "castaway": "0.2.4", "cfg-if": "1.0.5", "compact_str": "0.9.1", "garde": "0.23.0",
    "garde_derive": "0.23.0", "getrandom": "0.4.3", "itoa": "1.0.18", "libc": "0.2.191",
    "mio": "1.2.4", "pin-project-lite": "0.2.17", "proc-macro2": "1.0.107", "quote": "1.0.47",
    "rustversion": "1.0.23", "ryu": "1.0.23", "smallvec": "1.16.4", "socket2": "0.6.5",
    "static_assertions": "1.1.0", "syn": "2.0.119", "tokio": "1.54.1", "tokio-macros": "2.7.1",
    "unicode-ident": "1.0.27", "windows-link": "0.2.1", "windows-sys": "0.61.2",
}
RESOLUTION_VERSIONS = {"r-efi": "6.0.0", "wasi": "0.11.1+wasi-snapshot-preview1"}
FIXTURE_BUILD_SCRIPTS = {"getrandom@0.4.3", "libc@0.2.191", "proc-macro2@1.0.107",
                         "quote@1.0.47", "rustversion@1.0.23"}


def dependency_fixture():
    """Declare inert registry/role facts; no Cargo or real package source is used."""
    selected, metadata = [], []
    for name, version in SELECTED_VERSIONS.items():
        grant = "BSL-1.0" if name == "ryu" else "MIT AND Unicode-3.0" if name == "unicode-ident" else "MIT"
        selected.append({"name": name, "version": version, "checksum": digest((name+"@"+version).encode()),
                         "license": grant, "selected_license": grant, "build_script": name+"@"+version in FIXTURE_BUILD_SCRIPTS,
                         "files_sha256": "f"*64, "upstream_revision": "a"*40})
    for name, version in RESOLUTION_VERSIONS.items():
        metadata.append({"name": name, "version": version, "checksum": digest((name+"@"+version).encode()),
                         "license": "MIT", "files_sha256": "e"*64, "role": "metadata-only"})
    rows = {}
    for target, excluded in (("x86_64-unknown-linux-gnu", {"windows-link", "windows-sys"}),
                             ("x86_64-pc-windows-msvc", {"libc"})):
        identities = sorted(name+"@"+version for name, version in SELECTED_VERSIONS.items() if name not in excluded)
        rows[target] = {"external_packages": identities, "features": {identity: [] for identity in identities},
                        "allowed_build_scripts": sorted(FIXTURE_BUILD_SCRIPTS & set(identities))}
    return {"schema_version": 2, "scope": "Independent inert verifier control", "lockfile_sha256": "0"*64,
            "manifests_sha256": {name: "c"*64 for name in
                ("Cargo.toml", *(directory+"/Cargo.toml" for directory in EXPECTED_PACKAGES.values()))},
            "packages": selected, "metadata_only_packages": metadata, "rows": rows}


def digest(data):
    return hashlib.sha256(data).hexdigest()


class TextContractControls(unittest.TestCase):
    def test_actual_coverage_result_requires_complete_nonempty_both_direction_success(self):
        healthy = {"status": "passed", "active_ids": 497, "used_ids": 497, "errors": []}
        self.assertEqual(VERIFY.validate_text_coverage_result(json.dumps(healthy)), healthy)
        for changed in ({**healthy, "status": "failed"}, {**healthy, "errors": ["missing"]},
                        {**healthy, "used_ids": 496}, {**healthy, "active_ids": 0, "used_ids": 0},
                        {**healthy, "active_ids": True, "used_ids": True},
                        {**healthy, "active_ids": 497.0}, {}, [], "passed"):
            with self.subTest(changed=changed), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.validate_text_coverage_result(json.dumps(changed))
        for output in ("", "not JSON", " " * 65537):
            with self.subTest(length=len(output)), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.validate_text_coverage_result(output)

    def test_text_contract_rows_use_frozen_source_and_executed_controls(self):
        expected = [
            ("ui-text-controls", ["python3", "-X", "dev", "-Werror", "-B", "-m", "unittest",
                                  "discover", "-s", "scripts/tests", "-p", "test_ui_text_coverage.py", "-v"]),
            ("ui-text-codegen-controls", ["python3", "-X", "dev", "-Werror", "-B", "-m", "unittest",
                                          "discover", "-s", "scripts/tests", "-p", "test_ui_text_codegen.py", "-v"]),
            ("ui-text-native-controls", ["python3", "-X", "dev", "-Werror", "-B", "-m", "unittest",
                                          "discover", "-s", "scripts/tests", "-p", "test_ui_text_native.py", "-v"]),
            ("ui-text-coverage", ["python3", "-X", "dev", "-Werror", "-B",
                                  "scripts/check-ui-text.py", "--root", "/work/fixture"]),
        ]
        for lane in ("portable", "windows", "dependencies"):
            self.assertEqual(VERIFY.text_contract_commands(lane, schema_version=3), expected)
            self.assertEqual(VERIFY.text_contract_commands(lane, schema_version=2), [])
        for schema in (True, 0, 4):
            with self.subTest(schema=schema), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.text_contract_commands("portable", schema_version=schema)
        with self.assertRaises(VERIFY.ENV.Incomplete):
            VERIFY.text_contract_commands("unselected", schema_version=3)

    def test_native_input_row_is_separate_frozen_and_all_schema3_lanes(self):
        expected = [("ui-native-input-controls", ["python3", "-X", "dev", "-Werror", "-B", "-m", "unittest",
                                                  "discover", "-s", "scripts/tests", "-p", "test_native_input_source.py", "-v"])]
        self.assertEqual(VERIFY.UI_NATIVE_INPUT_CHECKER_FILES,
                         ("scripts/native_input_source.py", "scripts/tests/test_native_input_source.py"))
        for lane in ("portable", "windows", "dependencies"):
            self.assertEqual(VERIFY.native_input_control_commands(lane, schema_version=3), expected)
            self.assertEqual(VERIFY.native_input_control_commands(lane, schema_version=2), [])
            self.assertNotIn("ui-native-input-controls", dict(VERIFY.text_contract_commands(lane, schema_version=3)))
        for schema in (True, 0, 4):
            with self.subTest(schema=schema), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.native_input_control_commands("portable", schema_version=schema)
        with self.assertRaises(VERIFY.ENV.Incomplete):
            VERIFY.native_input_control_commands("unselected", schema_version=3)

    def test_text_control_result_cannot_pass_with_zero_skipped_or_absent_tests(self):
        self.assertEqual(VERIFY.validate_text_control_result("Ran 29 tests in 0.1s\n\nOK\n"), 29)
        for output in ("", "Ran 0 tests in 0.1s\n\nOK\n", "Ran 29 tests in 0.1s\n\nOK (skipped=1)\n",
                       "Ran 29 tests in 0.1s\n\nOK (expected failures=1)\n", "Ran 29 tests in 0.1s\n\nFAILED\n"):
            with self.subTest(output=output), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.validate_text_control_result(output)


class ProductAdmissionTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="product-admission-control-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        for name in (*VERIFY.SOURCE_FILES, *VERIFY.STRICT.CONFIG_FILES, *VERIFY.CHECKER_FILES,
                     *VERIFY.UI_TEXT_CHECKER_FILES, *VERIFY.UI_NATIVE_INPUT_CHECKER_FILES):
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
            '[workspace.package]\nlicense="GPL-2.0-only"\n' +
            (ROOT / "policy/rust-workspace.toml").read_text())
        for package, directory in EXPECTED_PACKAGES.items():
            member = self.root / directory
            (member / "src").mkdir(parents=True, exist_ok=True)
            (member / "Cargo.toml").write_text(
                f'[package]\nname="{package}"\nversion="0.0.0"\n' +
                ('license="GPL-3.0-only"\n' if package == "nevrotick-ui" else 'license.workspace=true\n') +
                '[lints]\nworkspace=true\n')
            if package == "nevrotick-ui":
                with (member / "Cargo.toml").open("a") as stream:
                    stream.write('[dependencies]\ngarde={version="=0.23.0",default-features=false,features=["derive"]}\n'
                                 '[target.\'cfg(windows)\'.dependencies]\n'
                                 'windows-sys={version="=0.61.2",default-features=false,features=['
                                 '"Win32_Foundation","Win32_Graphics_Gdi","Win32_System_LibraryLoader",'
                                 '"Win32_UI_WindowsAndMessaging","Win32_UI_Controls"]}\n')
            if package in {"time-domain", "time-core", "windows-platform"}:
                with (member / "Cargo.toml").open("a") as stream:
                    stream.write('[dependencies]\ngarde={version="=0.23.0",default-features=false,features=["derive"]}\n')
                    if package == "time-core":
                        stream.write('time-domain={version="=0.0.0",path="../time-domain"}\n')
            if package == "ntp-exchange":
                with (member / "Cargo.toml").open("a") as stream:
                    stream.write('[dependencies]\nntp-wire={version="=0.0.0",path="../ntp-wire"}\n'
                                 'time-domain={version="=0.0.0",path="../time-domain"}\n'
                                 '[dev-dependencies]\ntime-core={version="=0.0.0",path="../time-core"}\n')
            if package == "nevrotick-engine":
                with (member / "Cargo.toml").open("a") as stream:
                    stream.write('[dependencies]\ngarde={version="=0.23.0",default-features=false,features=["derive"]}\n'
                                 'getrandom={version="=0.4.3",default-features=false}\n'
                                 'ntp-wire={version="=0.0.0",path="../ntp-wire"}\n'
                                 'time-domain={version="=0.0.0",path="../time-domain"}\n'
                                 'tokio={version="=1.54.1",default-features=false,features=["rt","net","time","sync","macros"]}\n'
                                 '[[bin]]\nname="nevrotick-observe"\npath="src/bin/nevrotick-observe.rs"\n')
                (member / "src/bin").mkdir()
                (member / "src/bin/nevrotick-observe.rs").write_text("fn main() {}\n")
                (member / "tests").mkdir()
                (member / "tests/runtime_boundary.rs").write_text("// Inert admission fixture; never compiled.\n")
            (member / "src/lib.rs").write_text('//! Test control; never compiled.\n')
        self.policy = dependency_fixture()
        records = self.policy["packages"] + self.policy["metadata_only_packages"]
        lock = ('version = 4\n' + CORE_LOCK + ENGINE_LOCK +
                '[[package]]\nname="nevrotick-ui"\nversion="0.0.0"\n'
                '[[package]]\nname="ntp-wire"\nversion="0.0.0"\n'
                '[[package]]\nname="time-domain"\nversion="0.0.0"\ndependencies=["garde"]\n'
                '[[package]]\nname="windows-platform"\nversion="0.0.0"\ndependencies=["garde"]\n' + EXCHANGE_LOCK)
        for entry in records:
            lock += ('[[package]]\nname="'+entry["name"]+'"\nversion="'+entry["version"]+'"\n'
                     'source="registry+https://github.com/rust-lang/crates.io-index"\nchecksum="'+entry["checksum"]+'"\n')
        (self.root / "Cargo.lock").write_text(lock)
        self.policy["lockfile_sha256"] = VERIFY.sha256(self.root / "Cargo.lock")
        self.policy["manifests_sha256"] = {name: VERIFY.sha256(self.root / name)
            for name in ("Cargo.toml", *(directory+"/Cargo.toml" for directory in EXPECTED_PACKAGES.values()))}
        self.write_policy()

    def write_policy(self):
        (self.root / "policy/ui-m0-dependencies.json").write_text(json.dumps(self.policy))

    def test_gui_test_debug_budget_is_scoped_and_retains_arithmetic_checks(self):
        policy = tomllib.loads((ROOT / "policy/rust-workspace.toml").read_text())["profile"]
        self.assertEqual(policy.get("test"), {"package": {"nevrotick-ui": {"debug": 0}}})
        self.assertTrue(policy["release"]["overflow-checks"])
        self.assertTrue(policy["verify-release"]["debug-assertions"])
        self.assertEqual(policy["release"]["debug"], "limited")
        self.assertNotIn("dev", policy)

    def test_build_rows_keep_twelve_jobs_except_bounded_test_linking(self):
        for lane in ("portable", "windows"):
            for label, command in VERIFY.commands(lane):
                if label == "format":
                    continue
                with self.subTest(lane=lane, row=label):
                    flag = "--build-jobs" if "nextest" in command else "--jobs"
                    expected = ("6" if lane == "portable" and label in (
                        "product-tests:nevrotick-ui", "product-tests:time-core", "optimized-tests:time-core",
                        "product-tests:ntp-wire", "product-tests:time-domain",
                        "optimized-tests:time-domain") else "12")
                    self.assertEqual(command[command.index(flag) + 1], expected)

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

    def test_reviewed_seven_member_graph_is_accepted(self):
        with patch.object(VERIFY.ENV, "run") as run:
            _, files = VERIFY.admit_product(self.root)
            run.assert_not_called()
        for directory in EXPECTED_PACKAGES.values():
            self.assertIn(directory + "/src/lib.rs", files)

    def test_product_admission_enforces_actual_cargo_configuration_before_execution(self):
        path = self.root / ".cargo/config.toml"
        healthy = path.read_text()
        self.assertEqual(healthy, (ROOT / ".cargo/config.toml").read_text())
        with patch.object(VERIFY.ENV.subprocess, "Popen") as launch:
            _, files = VERIFY.admit_product(self.root)
            self.assertEqual(files[".cargo/config.toml"], digest(healthy.encode()))
            launch.assert_not_called()
        changes = (
            ("generic CPU", "target-cpu=x86-64-v3", "target-cpu=x86-64"),
            ("missing strict CET", '    "-C", "link-arg=/CETCOMPATSTRICT",\n', ""),
            ("missing DLL search policy", '    "-C", "link-arg=/DEPENDENTLOADFLAG:0xA00",\n', ""),
            ("trailing CFG override", '    "-C", "link-arg=/WX",\n',
             '    "-C", "link-arg=/WX",\n    "-C", "control-flow-guard=no",\n'),
        )
        for name, before, after in changes:
            self.assertEqual(healthy.count(before), 1, "Counterexample must change one configuration input")
            path.write_text(healthy.replace(before, after))
            with self.subTest(case=name), patch.object(VERIFY.ENV.subprocess, "Popen") as launch:
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "differs from the admitted"):
                    VERIFY.admit_product(self.root)
                launch.assert_not_called()
        path.write_text(healthy)

    def test_first_party_license_roles_are_exact_even_with_refreshed_pins(self):
        paths = [self.root / "Cargo.toml", *(self.root / directory / "Cargo.toml"
                                            for directory in EXPECTED_PACKAGES.values())]
        for path in paths:
            healthy = path.read_text()
            if path == self.root / "Cargo.toml":
                changes = [('license="GPL-2.0-only"', 'license="GPL-3.0-only"'),
                           ('license="GPL-2.0-only"', 'license="GPL-2.0-or-later"'),
                           ('license="GPL-2.0-only"', 'license-file="LICENSE"'),
                           ('license="GPL-2.0-only"', 'license="GPL-2.0-only"\nlicense-file="LICENSE"')]
            elif path.parent.name == "nevrotick-ui":
                changes = [('license="GPL-3.0-only"', 'license.workspace=true'),
                           ('license="GPL-3.0-only"', 'license="GPL-2.0-only"'),
                           ('license="GPL-3.0-only"', 'license="GPL-3.0-or-later"'),
                           ('license="GPL-3.0-only"', 'license-file="LICENSE"'),
                           ('license="GPL-3.0-only"', 'license="GPL-3.0-only"\nlicense-file="LICENSE"')]
            else:
                changes = [('license.workspace=true', 'license="GPL-3.0-only"'),
                           ('license.workspace=true', 'license="GPL-2.0-only"'),
                           ('license.workspace=true', 'license.workspace=1'),
                           ('license.workspace=true', 'license.workspace=false'),
                           ('license.workspace=true', ''),
                           ('license.workspace=true', 'license.workspace=true\nlicense-file="LICENSE"')]
            for before, after in changes:
                with self.subTest(path=path.relative_to(self.root), declaration=after):
                    path.write_text(healthy.replace(before, after))
                    self.policy["manifests_sha256"][str(path.relative_to(self.root))] = VERIFY.sha256(path)
                    self.write_policy()
                    with patch.object(VERIFY.ENV, "run") as run:
                        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "license"):
                            VERIFY.admit_product(self.root)
                        run.assert_not_called()
            path.write_text(healthy)
            self.policy["manifests_sha256"][str(path.relative_to(self.root))] = VERIFY.sha256(path)
            self.write_policy()
        VERIFY.admit_product(self.root)

    def test_ui_baseline_dependencies_are_exact_even_with_refreshed_pins(self):
        path = self.root / "crates/nevrotick-ui/Cargo.toml"
        healthy = path.read_text()
        variants = [healthy.replace('version="=0.23.0"', 'version="=0.23.1"'),
                    healthy.replace('version="=0.61.2"', 'version="=0.61.1"'),
                    healthy.replace('default-features=false', 'default-features=true'),
                    healthy.replace('default-features=false', 'default-features=0'),
                    healthy.replace('default-features=false,', ''),
                    healthy.replace('features=["derive"]', 'features=["derive","serde"]'),
                    healthy.replace('"Win32_UI_Controls"', '"Win32_Networking_WinSock"'),
                    healthy.replace('garde={', 'validation={package="garde",'),
                    healthy.replace('windows-sys={', 'os={package="windows-sys",'),
                    healthy.replace("cfg(windows)", "cfg(unix)"),
                    healthy.replace('version="0.0.0"', 'version="0.0.1"')]
        variants.extend(healthy + extra for extra in (
            '\n[features]\nunreviewed=[]\n',
            '\n[dev-dependencies]\nextra="=1.0.0"\n',
            '\n[build-dependencies]\nslint-build="=1.18.1"\n',
            '\n[target.\'cfg(unix)\'.dependencies]\nextra="=1.0.0"\n'))
        variants.append(healthy.replace('[dependencies]\n', '[dependencies]\nslint="=1.18.1"\n'))
        for text in variants:
            with self.subTest(text=text):
                path.write_text(text)
                self.policy["manifests_sha256"]["crates/nevrotick-ui/Cargo.toml"] = VERIFY.sha256(path)
                self.write_policy()
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "UI.*reviewed"):
                    VERIFY.admit_product(self.root)
        path.write_text(healthy)
        self.policy["manifests_sha256"]["crates/nevrotick-ui/Cargo.toml"] = VERIFY.sha256(path)
        self.write_policy()
        VERIFY.admit_product(self.root)

    def test_schema3_admits_only_pinned_ui_build_and_exact_slint_profile(self):
        """Reach real product admission with inert source/lock controls, never Cargo."""
        ui = self.root / "crates/nevrotick-ui/Cargo.toml"
        ui.write_text(ui.read_text().replace("[dependencies]\n", '[dependencies]\n'
            'serde={version="=1.0.229",default-features=false,features=["std"]}\n'
            'serde_json={version="=1.0.154",default-features=false,features=["std"]}\n').replace("[target.'cfg(windows)'.dependencies]\n", "[target.'cfg(windows)'.dependencies]\n"
            'slint={version="=1.18.1",default-features=false,features=["std","backend-winit","renderer-software","accessibility","compat-1-18","unstable-winit-030"]}\n'
            'windows={version="=0.62.2",default-features=false,features=["ApplicationModel","Data_Xml_Dom","Foundation","UI_Notifications","Win32_System_Com","Win32_System_WinRT"]}\n').replace(
                '"Win32_UI_WindowsAndMessaging","Win32_UI_Controls"]',
                '"Win32_UI_WindowsAndMessaging","Win32_UI_Controls","Win32_UI_Shell"]') +
            '\n[target.\'cfg(not(windows))\'.dependencies]\nslint={version="=1.18.1",default-features=false,features=["std","renderer-software","compat-1-18"]}\n' +
            'i-slint-core={version="=1.18.1",default-features=false,features=["image-decoders","svg"]}\n' +
            '\n[build-dependencies]\nslint-build={version="=1.18.1",default-features=false,features=["compat-1-18"]}\n')
        build = ui.parent / "build.rs"
        build.write_text('// Inert admission control; never compiled.\nfn main() {}\n')
        self.policy["schema_version"] = 3
        self.policy["first_party_build_scripts"] = {"crates/nevrotick-ui/build.rs": VERIFY.sha256(build)}
        macro_names = {"garde_derive", "rustversion", "tokio-macros"}
        for record in self.policy["packages"]:
            record.update(proc_macro=record["name"] in macro_names, links=None)
        for name in ("slint", "slint-build"):
            record = {"name": name, "version": "1.18.1", "checksum": digest((name + "@1.18.1").encode()),
                      "license": "GPL-3.0-only", "selected_license": "GPL-3.0-only", "build_script": False,
                      "proc_macro": False, "links": None, "files_sha256": "f" * 64, "upstream_revision": None}
            self.policy["packages"].append(record)
            with (self.root / "Cargo.lock").open("a") as stream:
                stream.write('[[package]]\nname="' + name + '"\nversion="1.18.1"\n'
                             'source="registry+https://github.com/rust-lang/crates.io-index"\nchecksum="' + record["checksum"] + '"\n')
        self.policy["packages"].sort(key=lambda record: record["name"] + "@" + record["version"])
        for target, old in list(self.policy["rows"].items()):
            features = {key: {"host" if key.split("@")[0] in macro_names else "target": [values]}
                        for key, values in old["features"].items()}
            runtime = (["accessibility", "backend-winit", "compat-1-18", "renderer-software", "std", "unstable-winit-030"]
                       if target == "x86_64-pc-windows-msvc" else ["compat-1-18", "renderer-software", "std"])
            features.update({"slint@1.18.1": {"target": [runtime]},
                             "slint-build@1.18.1": {"host": [["compat-1-18"]]}})
            local_closures = {"nevrotick-ui": {"nevrotick-ui"}, "ntp-wire": {"ntp-wire"},
                              "time-domain": {"time-domain"}, "time-core": {"time-core", "time-domain"},
                              "ntp-exchange": {"ntp-exchange", "ntp-wire", "time-domain", "time-core"},
                              "nevrotick-engine": {"nevrotick-engine", "ntp-wire", "time-domain"},
                              "windows-platform": {"windows-platform"}}
            self.policy["rows"][target] = {
                "features": features,
                "local_features": {name + "@0.0.0": {"target": [[]]} for name in EXPECTED_PACKAGES},
                "metadata_features": {key: sorted({value for sets in roles.values() for values in sets for value in values})
                                      for key, roles in features.items()},
                "allowed_build_scripts": sorted(old["allowed_build_scripts"] + ["nevrotick-ui@0.0.0"]),
                "allowed_proc_macros": sorted(key for key in features if key.split("@")[0] in macro_names),
                "duplicate_exceptions": [],
                "package_scopes": {name: {"features": {key: roles for key, roles in features.items()
                                                       if name == "nevrotick-ui" or not key.startswith("slint")},
                                           "local_features": {member + "@0.0.0": {"target": [[]]} for member in local}}
                                   for name, local in local_closures.items()},
            }
        self.policy["lockfile_sha256"] = VERIFY.sha256(self.root / "Cargo.lock")
        self.policy["manifests_sha256"]["crates/nevrotick-ui/Cargo.toml"] = VERIFY.sha256(ui)
        # An unrelated platform package makes Cargo qualify this exact engine
        # edge. The pure engine's selected version remains 0.4.3.
        self.policy["metadata_only_packages"].insert(0, {
            "name": "getrandom", "version": "0.3.4", "checksum": "a" * 64,
            "license": "MIT", "files_sha256": "b" * 64, "role": "metadata-only"})
        lock = self.root / "Cargo.lock"
        unqualified = 'dependencies=["garde","getrandom","ntp-wire","time-domain","tokio"]'
        self.assertIn(unqualified, lock.read_text())
        lock.write_text(lock.read_text().replace(unqualified,
                                               'dependencies=["garde","getrandom 0.4.3","ntp-wire","time-domain","tokio"]') +
                        '\n[[package]]\nname="getrandom"\nversion="0.3.4"\n'
                        'source="registry+https://github.com/rust-lang/crates.io-index"\nchecksum="' + 'a' * 64 + '"\n')
        self.policy["lockfile_sha256"] = VERIFY.sha256(lock)
        self.write_policy()
        with patch.object(VERIFY.SLINT, "REVIEWED_POLICY_SHA256", VERIFY.SLINT.canonical_policy_sha256(self.policy)):
            _, inputs = VERIFY.admit_product(self.root)
            self.assertIn("crates/nevrotick-ui/build.rs", inputs)
            for checker in VERIFY.UI_NATIVE_INPUT_CHECKER_FILES:
                self.assertIn(checker, inputs)
                path = self.root / checker
                original = path.read_bytes()
                path.unlink()
                with self.subTest(missing_checker=checker), patch.object(VERIFY.ENV, "run") as run:
                    with self.assertRaises(VERIFY.ENV.Incomplete):
                        VERIFY.admit_product(self.root)
                    run.assert_not_called()
                path.write_bytes(original)
            VERIFY.admit_product(self.root)
            build.write_text('// Modified after admission.\nfn main() {}\n')
            with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "UI build script"):
                VERIFY.admit_product(self.root)
            build.write_text('// Inert admission control; never compiled.\nfn main() {}\n')
            pure_build = self.root / "crates/time-core/build.rs"
            pure_build.write_text('fn main() {}\n')
            with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unadmitted first-party build"):
                VERIFY.admit_product(self.root)
            pure_build.unlink()
        healthy = ui.read_text()
        for changed in (healthy.replace('features=["compat-1-18"]', 'features=["compat-1-18","renderer-software"]'),
                        healthy.replace('version="=1.18.1"', 'version="=1.18.2"'),
                        healthy.replace('default-features=false', 'default-features=true'),
                        healthy.replace('features=["std","renderer-software","compat-1-18"]',
                                        'features=["std","backend-winit","renderer-software","compat-1-18"]')):
            ui.write_text(changed)
            self.policy["manifests_sha256"]["crates/nevrotick-ui/Cargo.toml"] = VERIFY.sha256(ui)
            self.write_policy()
            with patch.object(VERIFY.SLINT, "REVIEWED_POLICY_SHA256", VERIFY.SLINT.canonical_policy_sha256(self.policy)):
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "UI.*reviewed"):
                    VERIFY.admit_product(self.root)
        # Challenge the receiving Windows Slint declaration alone; unrelated
        # Garde/build/native defaults are deliberately left valid.
        winit_slint = ('slint={version="=1.18.1",default-features=false,features=['
                       '"std","backend-winit","renderer-software","accessibility",'
                       '"compat-1-18","unstable-winit-030"]}')
        self.assertIn(winit_slint, healthy)
        winit_alternatives = (
            winit_slint.replace('default-features=false', 'default-features=true'),
            winit_slint.replace('default-features=false', 'default-features=0'),
            winit_slint.replace('default-features=false,', ''),
            winit_slint.replace(',"unstable-winit-030"', ''),
            winit_slint.replace('"unstable-winit-030"]', '"unstable-winit-030","unstable-winit-030"]'),
            winit_slint.replace('"unstable-winit-030"]', '"unstable-winit-030","system-testing"]'),
            winit_slint[:-1] + ',optional=true}',
            'renamed_' + winit_slint[:-1] + ',package="slint"}',
        )
        changed_winit = [healthy.replace(winit_slint, alternative) for alternative in winit_alternatives]
        changed_winit += [
            healthy.replace(winit_slint + '\n', '').replace('[dependencies]\n', '[dependencies]\n' + winit_slint + '\n', 1),
            healthy.replace('features=["std","renderer-software","compat-1-18"]',
                            'features=["std","renderer-software","compat-1-18","unstable-winit-030"]'),
            healthy + '\n[target.\'cfg(unix)\'.dependencies]\n' + winit_slint + '\n',
        ]
        for changed in changed_winit:
            with self.subTest(winit_manifest=changed), patch.object(VERIFY.ENV, "run") as run:
                self.assertNotEqual(changed, healthy)
                ui.write_text(changed)
                self.policy["manifests_sha256"]["crates/nevrotick-ui/Cargo.toml"] = VERIFY.sha256(ui)
                self.write_policy()
                with patch.object(VERIFY.SLINT, "REVIEWED_POLICY_SHA256", VERIFY.SLINT.canonical_policy_sha256(self.policy)):
                    with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "UI.*reviewed"):
                        VERIFY.admit_product(self.root)
                run.assert_not_called()
        ui.write_text(healthy)
        self.policy["manifests_sha256"]["crates/nevrotick-ui/Cargo.toml"] = VERIFY.sha256(ui)
        self.write_policy()
        with patch.object(VERIFY.SLINT, "REVIEWED_POLICY_SHA256", VERIFY.SLINT.canonical_policy_sha256(self.policy)):
            VERIFY.admit_product(self.root)
        # This is an inert strict-manifest control, not observed Cargo resolution.
        native = ('windows={version="=0.62.2",default-features=false,features=['
                  '"ApplicationModel","Data_Xml_Dom","Foundation","UI_Notifications",'
                  '"Win32_System_Com","Win32_System_WinRT"]}')
        self.assertIn(native, healthy)
        native_alternatives = (
            native.replace('version="=0.62.2"', 'version="=0.63.0"'),
            native.replace('default-features=false', 'default-features=true'),
            native.replace('default-features=false', 'default-features=0'),
            native.replace('features=[', 'features=["Networking",'),
            native.replace(',"Win32_System_WinRT"', ''),
            native.replace('features=["ApplicationModel"', 'features=["ApplicationModel","ApplicationModel"'),
            native.replace('features=["ApplicationModel","Data_Xml_Dom","Foundation","UI_Notifications","Win32_System_Com","Win32_System_WinRT"]', 'features=false'),
            native[:-1] + ',optional=true}',
            'renamed_' + native[:-1] + ',package="windows"}',
            '',
        )
        changed_native = [healthy.replace(native, item) for item in native_alternatives]
        changed_native += [
            healthy.replace(',"Win32_UI_Shell"', ''),
            healthy.replace(native + '\n', '').replace('[dependencies]\n', '[dependencies]\n' + native + '\n', 1),
            healthy.replace(native + '\n', '').replace("[target.'cfg(not(windows))'.dependencies]\n",
                "[target.'cfg(not(windows))'.dependencies]\n" + native + '\n'),
            healthy + '\n[target.\'cfg(unix)\'.dependencies]\n' + native + '\n',
        ]
        for changed in changed_native:
            with self.subTest(native_manifest=changed), patch.object(VERIFY.ENV, "run") as run:
                self.assertNotEqual(changed, healthy)
                ui.write_text(changed)
                self.policy["manifests_sha256"]["crates/nevrotick-ui/Cargo.toml"] = VERIFY.sha256(ui)
                self.write_policy()
                with patch.object(VERIFY.SLINT, "REVIEWED_POLICY_SHA256", VERIFY.SLINT.canonical_policy_sha256(self.policy)):
                    with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "UI.*reviewed"):
                        VERIFY.admit_product(self.root)
                run.assert_not_called()
        ui.write_text(healthy)
        self.policy["manifests_sha256"]["crates/nevrotick-ui/Cargo.toml"] = VERIFY.sha256(ui)
        core = self.root / "crates/time-core/Cargo.toml"
        core_healthy = core.read_text()
        core.write_text(core_healthy + '\n[target.\'cfg(windows)\'.dependencies]\n' + native + '\n')
        self.policy["manifests_sha256"]["crates/time-core/Cargo.toml"] = VERIFY.sha256(core)
        self.write_policy()
        with patch.object(VERIFY.SLINT, "REVIEWED_POLICY_SHA256", VERIFY.SLINT.canonical_policy_sha256(self.policy)), \
                patch.object(VERIFY.ENV, "run") as run:
            with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "time-core member requires"):
                VERIFY.admit_product(self.root)
            run.assert_not_called()
        core.write_text(core_healthy)
        self.policy["manifests_sha256"]["crates/time-core/Cargo.toml"] = VERIFY.sha256(core)
        self.write_policy()
        for name, version in (("serde", "1.0.229"), ("serde_json", "1.0.154")):
            exact = f'{name}={{version="={version}",default-features=false,features=["std"]}}'
            alternatives = (
                exact.replace('default-features=false', 'default-features=true'),
                exact.replace('default-features=false', 'default-features=0'),
                exact.replace('features=["std"]', 'features=[]'),
                exact.replace('features=["std"]', 'features=["std","derive"]'),
                exact.replace('features=["std"]', 'features=["std","raw_value"]'),
                exact.replace('features=["std"]', 'features=["std","unbounded_depth"]'),
                exact.replace('features=["std"]', 'features=["std","preserve_order"]'),
                exact.replace('features=["std"]', 'features=["std","arbitrary_precision"]'),
                exact.replace(f'version="={version}"', 'version="=1.0.1"'),
                exact[:-1] + ',optional=true}',
                'renamed_' + exact[:-1] + f',package="{name}"}}',
                '',
            )
            for alternative in alternatives:
                with self.subTest(serde_dependency=name, alternative=alternative):
                    changed = healthy.replace(exact, alternative)
                    self.assertNotEqual(changed, healthy)
                    ui.write_text(changed)
                    self.policy["manifests_sha256"]["crates/nevrotick-ui/Cargo.toml"] = VERIFY.sha256(ui)
                    self.write_policy()
                    with patch.object(VERIFY.SLINT, "REVIEWED_POLICY_SHA256", VERIFY.SLINT.canonical_policy_sha256(self.policy)):
                        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "UI.*reviewed"):
                            VERIFY.admit_product(self.root)
        ui.write_text(healthy)
        self.policy["manifests_sha256"]["crates/nevrotick-ui/Cargo.toml"] = VERIFY.sha256(ui)
        self.write_policy()
        with patch.object(VERIFY.SLINT, "REVIEWED_POLICY_SHA256", VERIFY.SLINT.canonical_policy_sha256(self.policy)):
            VERIFY.admit_product(self.root)

    def test_prior_five_members_cannot_hide_engine(self):
        path = self.root / "Cargo.toml"
        path.write_text(path.read_text().replace(', "crates/nevrotick-engine"', ''))
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "reviewed.*workspace members"):
            VERIFY.admit_product(self.root)

    def test_platform_admission_rejects_new_authority_and_missing_validation(self):
        """The pure shared owner cannot acquire runtime/FFI edges by manifest drift."""
        path = self.root / "crates/windows-platform/Cargo.toml"
        healthy = path.read_text()
        validation = 'garde={version="=0.23.0",default-features=false,features=["derive"]}\n'
        VERIFY.admit_product(self.root)
        alternatives = (
            healthy.replace(validation, ""),
            healthy.replace("default-features=false", "default-features=true"),
            healthy + 'tokio={version="=1.54.1",default-features=false,features=["rt"]}\n',
            healthy + '[target.\'cfg(windows)\'.dependencies]\nwindows-sys="=0.61.2"\n',
            healthy + '[features]\nservice=[]\n',
        )
        for candidate in alternatives:
            with self.subTest(candidate=candidate):
                path.write_text(candidate)
                self.policy["manifests_sha256"][str(path.relative_to(self.root))] = VERIFY.sha256(path)
                self.write_policy()
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "windows-platform member"):
                    VERIFY.admit_product(self.root)
        path.write_text(healthy)
        self.policy["manifests_sha256"][str(path.relative_to(self.root))] = VERIFY.sha256(path)
        self.write_policy()
        VERIFY.admit_product(self.root)

    def test_engine_garde_edge_is_exact_and_mandatory_with_healthy_control(self):
        """Independent direct-edge contract; no product or macro code is executed."""
        path = self.root / "crates/nevrotick-engine/Cargo.toml"
        healthy = path.read_text()
        declaration = 'garde={version="=0.23.0",default-features=false,features=["derive"]}\n'
        self.assertIn(declaration, healthy)
        VERIFY.admit_product(self.root)
        for replacement in ("", declaration.replace("=0.23.0", "=0.23.1"),
                            declaration.replace("default-features=false", "default-features=true"),
                            declaration.replace("default-features=false", "default-features=0"),
                            declaration.replace('features=["derive"]', 'features=[]'),
                            declaration.replace('features=["derive"]', 'features=["derive","serde"]'),
                            declaration.replace('garde={', 'validation={package="garde",')):
            with self.subTest(replacement=replacement):
                path.write_text(healthy.replace(declaration, replacement))
                self.policy["manifests_sha256"]["crates/nevrotick-engine/Cargo.toml"] = VERIFY.sha256(path)
                self.write_policy()
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "engine requires"):
                    VERIFY.admit_product(self.root)
        path.write_text(healthy)
        self.policy["manifests_sha256"]["crates/nevrotick-engine/Cargo.toml"] = VERIFY.sha256(path)
        self.write_policy()
        VERIFY.admit_product(self.root)

    def test_engine_manifest_effects_are_exact_even_with_refreshed_hash(self):
        path = self.root / "crates/nevrotick-engine/Cargo.toml"
        healthy = path.read_text()
        variants = [healthy.replace('version="=1.54.1"', 'version="=1.54.2"'),
                    healthy.replace('features=["rt","net","time","sync","macros"]', 'features=["full"]'),
                    healthy.replace('default-features=false', 'default-features=true'),
                    healthy.replace('default-features=false', 'default-features=0'),
                    healthy.replace('ntp-wire={version="=0.0.0",path="../ntp-wire"}', 'ntp-wire={path="../ntp-wire"}'),
                    healthy.replace('name="nevrotick-observe"', 'name="other"'),
                    healthy.replace('path="src/bin/nevrotick-observe.rs"', 'path="src/lib.rs"')]
        variants.extend(healthy + extra for extra in (
            '\n[dev-dependencies]\nextra="=1.0.0"\n', '\n[features]\nextra=[]\n',
            '\n[build-dependencies]\nextra="=1.0.0"\n',
            '\n[target.\'cfg(windows)\'.dependencies]\nextra="=1.0.0"\n'))
        variants.append(healthy.replace('[dependencies]\n', '[dependencies]\ntokio-macros="=2.7.1"\n'))
        for text in variants:
            with self.subTest(text=text):
                path.write_text(text)
                self.policy["manifests_sha256"]["crates/nevrotick-engine/Cargo.toml"] = VERIFY.sha256(path)
                self.write_policy()
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "engine requires"):
                    VERIFY.admit_product(self.root)

    def test_engine_time_domain_edge_has_exact_normal_role_and_locked_identity(self):
        path = self.root / "crates/nevrotick-engine/Cargo.toml"
        healthy = path.read_text()
        edge = 'time-domain={version="=0.0.0",path="../time-domain"}\n'
        self.assertIn(edge, healthy)
        variants = (
            healthy.replace(edge, ""),
            healthy.replace(edge, 'time-domain={path="../time-domain"}\n'),
            healthy.replace(edge, 'time-domain={version="=0.0.1",path="../time-domain"}\n'),
            healthy.replace(edge, 'time-domain={version="=0.0.0",path="../time-core"}\n'),
            healthy.replace(edge, 'domain={package="time-domain",version="=0.0.0",path="../time-domain"}\n'),
            healthy.replace(edge, "") + '\n[dev-dependencies]\n' + edge,
        )
        for text in variants:
            path.write_text(text)
            self.policy["manifests_sha256"]["crates/nevrotick-engine/Cargo.toml"] = VERIFY.sha256(path)
            self.write_policy()
            with self.subTest(manifest=text), self.assertRaisesRegex(VERIFY.ENV.Incomplete, "engine requires"):
                VERIFY.admit_product(self.root)
        path.write_text(healthy)
        self.policy["manifests_sha256"]["crates/nevrotick-engine/Cargo.toml"] = VERIFY.sha256(path)
        lock = self.root / "Cargo.lock"
        raw = lock.read_text()
        lock.write_text(raw.replace(ENGINE_LOCK, ENGINE_LOCK.replace(',"time-domain"', '')))
        self.policy["lockfile_sha256"] = VERIFY.sha256(lock)
        self.write_policy()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unexpected packages"):
            VERIFY.admit_product(self.root)
        lock.write_text(raw)
        self.policy["lockfile_sha256"] = VERIFY.sha256(lock)
        self.write_policy()
        VERIFY.admit_product(self.root)

    def test_ui_cannot_acquire_engine_runtime_even_with_updated_manifest_admission(self):
        path = self.root / "crates/nevrotick-ui/Cargo.toml"
        healthy = path.read_text()
        for declaration in ('tokio={version="=1.54.1"}',
                            'clock-runtime={package="tokio",version="=1.54.1"}',
                            'nevrotick-engine={version="=0.0.0",path="../nevrotick-engine"}'):
            path.write_text(healthy.replace('[dependencies]\n', '[dependencies]\n' + declaration + '\n'))
            self.policy["manifests_sha256"]["crates/nevrotick-ui/Cargo.toml"] = VERIFY.sha256(path)
            self.write_policy()
            with self.subTest(declaration=declaration), self.assertRaisesRegex(VERIFY.ENV.Incomplete, "only to the engine"):
                VERIFY.admit_product(self.root)

    def test_ui_cannot_inherit_a_workspace_runtime_alias_after_refreshing_all_pins(self):
        workspace = self.root / "Cargo.toml"
        workspace.write_text(workspace.read_text() + '\n[workspace.dependencies]\n'
                             'clock-runtime={package="tokio",version="=1.54.1",default-features=false,features=["rt","net","time","sync","macros"]}\n')
        member = self.root / "crates/nevrotick-ui/Cargo.toml"
        member.write_text(member.read_text().replace('[dependencies]\n', '[dependencies]\nclock-runtime={workspace=true}\n'))
        lock = self.root / "Cargo.lock"
        lock.write_text(lock.read_text().replace('name="nevrotick-ui"\nversion="0.0.0"',
                                                'name="nevrotick-ui"\nversion="0.0.0"\ndependencies=["tokio"]'))
        self.policy["manifests_sha256"]["Cargo.toml"] = VERIFY.sha256(workspace)
        self.policy["manifests_sha256"]["crates/nevrotick-ui/Cargo.toml"] = VERIFY.sha256(member)
        self.policy["lockfile_sha256"] = VERIFY.sha256(lock)
        self.write_policy()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "only to the engine"):
            VERIFY.admit_product(self.root)

    def test_engine_lock_edges_entrypoint_and_runtime_suite_are_mandatory(self):
        lock = self.root / "Cargo.lock"
        healthy = lock.read_text()
        changed = healthy.replace(ENGINE_LOCK, ENGINE_LOCK.replace('["garde","getrandom","ntp-wire","time-domain","tokio"]', '["tokio"]'))
        self.assertNotEqual(changed, healthy)
        lock.write_text(changed)
        self.policy["lockfile_sha256"] = VERIFY.sha256(lock)
        self.write_policy()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unexpected packages"):
            VERIFY.admit_product(self.root)
        lock.write_text(healthy)
        self.policy["lockfile_sha256"] = VERIFY.sha256(lock)
        self.write_policy()
        for name in ("src/bin/nevrotick-observe.rs", "tests/runtime_boundary.rs"):
            path = self.root / "crates/nevrotick-engine" / name
            raw = path.read_bytes()
            path.unlink()
            with self.subTest(name=name), self.assertRaisesRegex(VERIFY.ENV.Incomplete, "entrypoint or runtime-boundary"):
                VERIFY.admit_product(self.root)
            path.write_bytes(raw)

    def test_metadata_only_cannot_be_promoted_or_changed_with_updated_lock_hash(self):
        original = json.loads(json.dumps(self.policy))
        for change in ("promote", "role", "build", "macro"):
            self.policy = json.loads(json.dumps(original))
            if change == "promote":
                self.policy["packages"].append(self.policy["metadata_only_packages"].pop())
            elif change == "role":
                self.policy["metadata_only_packages"][0]["role"] = "runtime"
            elif change == "build":
                self.policy["metadata_only_packages"][0]["build_script"] = True
            else:
                next(p for p in self.policy["packages"] if p["name"] == "tokio-macros")["version"] = "2.7.2"
            self.write_policy()
            with self.subTest(change=change), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.admit_product(self.root)

    def test_dependency_json_is_bounded_strict_and_has_no_duplicate_fields(self):
        path = self.root / "policy/ui-m0-dependencies.json"
        healthy = path.read_text()
        for payload in ('{"schema_version":2,"schema_version":2}', '{"number":NaN}', '[]', ' ' * (512 * 1024 + 1)):
            path.write_text(payload)
            with self.subTest(prefix=payload[:40]), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.read_dependency_policy(path)
        path.write_text(healthy)
        self.assertEqual(VERIFY.read_dependency_policy(path), self.policy)

    def test_all_pure_role_clippy_configurations_are_frozen_product_inputs(self):
        _, files = VERIFY.admit_product(self.root)
        for package in ("time-domain", "ntp-exchange", "time-core"):
            name = f"crates/{package}/clippy.toml"
            with self.subTest(package=package):
                self.assertIn(name, files)
                self.assertEqual(files[name], digest((self.root / name).read_bytes()))

    def test_alternate_clippy_configuration_is_rejected_before_cargo(self):
        paths = (".clippy.toml", *(f"{directory}/.clippy.toml"
                                  for directory in EXPECTED_PACKAGES.values()))
        for name in paths:
            path = self.root / name
            with self.subTest(configuration=name), patch.object(VERIFY.ENV, "run") as run:
                path.write_text("check-private-items = false\n")
                with self.assertRaises(VERIFY.ENV.Incomplete):
                    VERIFY.admit_product(self.root)
                run.assert_not_called()
            path.unlink()
            VERIFY.admit_product(self.root)

    def test_nested_pure_clippy_configuration_is_rejected_before_cargo(self):
        for package in ("time-domain", "ntp-exchange", "time-core"):
            for filename in ("clippy.toml", ".clippy.toml"):
                path = self.root / f"crates/{package}/src/{filename}"
                with self.subTest(package=package, filename=filename), \
                        patch.object(VERIFY.ENV, "run") as run:
                    path.write_text("check-private-items = false\n")
                    with self.assertRaises(VERIFY.ENV.Incomplete):
                        VERIFY.admit_product(self.root)
                    run.assert_not_called()
                path.unlink()
                VERIFY.admit_product(self.root)

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
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "UI.*reviewed"):
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

    def test_metadata_only_source_is_required_without_becoming_selected(self):
        metadata = {"name": "wasi", "version": "0.11.1+wasi-snapshot-preview1",
                    "checksum": "3" * 64, "files_sha256": self.policy["packages"][0]["files_sha256"],
                    "role": "metadata-only"}
        self.policy["metadata_only_packages"] = [metadata]
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "missing or unexpected"):
            VERIFY.admit_vendor(self.root, self.policy)
        package = self.root / "wasi-0.11.1+wasi-snapshot-preview1"
        package.mkdir()
        (package / "lib.rs").write_bytes(self.payload.read_bytes())
        (package / ".cargo-checksum.json").write_text(json.dumps({"package": "3" * 64, "files": self.files}))
        self.assertEqual(len(VERIFY.admit_vendor(self.root, self.policy)), 4)
        self.assertEqual(len(self.policy["packages"]), 1)
        self.policy["packages"].append(metadata)
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "roles are duplicated"):
            VERIFY.admit_vendor(self.root, self.policy)

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
        self.policy = dependency_fixture()

    def test_only_exact_grants_change_and_held_rules_survive(self):
        result = tomllib.loads(VERIFY.product_deny(self.base, self.policy, "x86_64-unknown-linux-gnu"))
        original = tomllib.loads(self.base)
        self.assertEqual(result["bans"]["build"]["allow-build-scripts"],
                         ["getrandom@0.4.3", "libc@0.2.191", "proc-macro2@1.0.107", "quote@1.0.47", "rustversion@1.0.23"])
        self.assertEqual(set(result["licenses"]["allow"]), {"GPL-2.0-only", "MIT", "BSL-1.0", "Unicode-3.0"})
        self.assertEqual(result["licenses"]["exceptions"],
                         [{"crate": "nevrotick-ui@0.0.0", "allow": ["GPL-3.0-only"]}])
        result["licenses"]["allow"] = original["licenses"]["allow"]
        result["licenses"]["exceptions"] = original["licenses"]["exceptions"]
        result["bans"]["build"]["allow-build-scripts"] = original["bans"]["build"]["allow-build-scripts"]
        self.assertEqual(result, original)

    def test_gui_license_exception_is_exact_on_both_targets(self):
        for target in ("x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc"):
            with self.subTest(target=target):
                result = tomllib.loads(VERIFY.product_deny(self.base, self.policy, target))
                self.assertNotIn("GPL-3.0-only", result["licenses"]["allow"])
                self.assertEqual(result["licenses"]["exceptions"],
                                 [{"crate": "nevrotick-ui@0.0.0", "allow": ["GPL-3.0-only"]}])

    def test_gui_exception_does_not_admit_a_broadened_bootstrap(self):
        for replacement in (
            'exceptions = [{crate="nevrotick-ui@0.0.0",allow=["GPL-3.0-only"]}]',
            'exceptions = [{crate="time-core@0.0.0",allow=["GPL-3.0-only"]}]',
            'exceptions = [{crate="nevrotick-ui",allow=["GPL-3.0-only"]}]',
        ):
            with self.subTest(replacement=replacement):
                base = self.base.replace("exceptions = []", replacement)
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unexpectedly broadened"):
                    VERIFY.product_deny(base, self.policy, "x86_64-unknown-linux-gnu")

    def test_windows_build_allowance_excludes_linux_script_and_keeps_old_three(self):
        result = tomllib.loads(VERIFY.product_deny(self.base, self.policy, "x86_64-pc-windows-msvc"))
        self.assertEqual(result["bans"]["build"]["allow-build-scripts"],
                         ["getrandom@0.4.3", "proc-macro2@1.0.107", "quote@1.0.47", "rustversion@1.0.23"])
        self.policy["rows"]["x86_64-pc-windows-msvc"]["allowed_build_scripts"].insert(1, "libc@0.2.191")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "target package/build-script"):
            VERIFY.product_deny(self.base, self.policy, "x86_64-pc-windows-msvc")

    def test_frozen_metadata_is_compared_before_target_policy_scan(self):
        rows = VERIFY.commands("dependencies")
        self.assertEqual([label for label, _ in rows], [
            "dependency-metadata-x86_64-unknown-linux-gnu", "dependency-x86_64-unknown-linux-gnu",
            "dependency-metadata-x86_64-pc-windows-msvc", "dependency-x86_64-pc-windows-msvc"])
        for label, command in rows:
            self.assertIn("--frozen", command)
            if label.startswith("dependency-metadata-"):
                self.assertIn("--offline", command)
                self.assertIn("metadata", command)

    def test_actual_target_metadata_rejects_feature_source_and_role_drift(self):
        target = "x86_64-unknown-linux-gnu"
        expected = self.policy["rows"][target]["features"]
        packages = [{"id": name, "name": name, "version": "0.0.0", "source": None}
                    for name in EXPECTED_PACKAGES]
        packages += [{"id": identity, "name": identity.split("@")[0], "version": identity.split("@")[1],
                      "source": "registry+https://github.com/rust-lang/crates.io-index"} for identity in expected]
        healthy = {"packages": packages, "resolve": {"nodes": [{"id": p["id"], "features": []} for p in packages]}}
        self.assertEqual(VERIFY.validate_dependency_metadata(target, json.dumps(healthy), self.policy)["external_packages"],
                         sorted(expected))
        for variant in ("feature", "source", "extra", "duplicate", "missing", "local"):
            data = json.loads(json.dumps(healthy))
            if variant == "feature":
                next(n for n in data["resolve"]["nodes"] if n["id"] == "tokio@1.54.1")["features"] = ["full"]
            elif variant == "source":
                data["packages"][-1]["source"] = "git+https://example.invalid/unreviewed"
            elif variant == "extra":
                data["packages"].append({"id": "r-efi@6.0.0", "name": "r-efi", "version": "6.0.0", "source": "registry+https://github.com/rust-lang/crates.io-index"})
                data["resolve"]["nodes"].append({"id": "r-efi@6.0.0", "features": []})
            elif variant == "duplicate":
                data["resolve"]["nodes"].append(data["resolve"]["nodes"][-1])
            elif variant == "missing":
                data["packages"].pop()
            else:
                data["packages"][0]["name"] = "unreviewed-owner"
            with self.subTest(variant=variant), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.validate_dependency_metadata(target, json.dumps(data), self.policy)

    def test_new_build_script_or_version_requires_review(self):
        next(p for p in self.policy["packages"] if p["name"] == "proc-macro2")["version"] = "1.0.108"
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unexpected packages"):
            VERIFY.product_deny(self.base, self.policy, "x86_64-unknown-linux-gnu")

    def test_new_license_and_broadened_bootstrap_are_rejected(self):
        for grant in ("Apache-2.0", "GPL-3.0-only"):
            self.policy["packages"][0]["selected_license"] = grant
            with self.subTest(grant=grant), self.assertRaisesRegex(VERIFY.ENV.Incomplete, "reviewed policy change"):
                VERIFY.product_deny(self.base, self.policy, "x86_64-unknown-linux-gnu")
        self.policy["packages"][0]["selected_license"] = "MIT"
        for extra in ("MIT", "GPL-3.0-only"):
            base = self.base.replace('allow = ["GPL-2.0-only"]', 'allow = ["GPL-2.0-only", "' + extra + '"]')
            with self.subTest(extra=extra), self.assertRaisesRegex(VERIFY.ENV.Incomplete, "unexpectedly broadened"):
                VERIFY.product_deny(base, self.policy, "x86_64-unknown-linux-gnu")


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

    def test_engine_entrypoint_is_checked_and_built_on_both_targets(self):
        portable = dict(VERIFY.commands("portable"))
        for label in ("compiler", "clippy"):
            self.assertIn("--all-targets", portable[label])
        host = portable["host-engine-build"]
        self.assertEqual(host[host.index("--package") + 1], "nevrotick-engine")
        self.assertEqual(host[host.index("--bin") + 1], "nevrotick-observe")
        self.assertIn("build", host)
        windows = dict(VERIFY.commands("windows"))
        engine = windows["windows-engine-link"]
        self.assertEqual(engine[engine.index("--package") + 1], "nevrotick-engine")
        self.assertEqual(engine[engine.index("--bin") + 1], "nevrotick-observe")
        self.assertIn("--release", engine)
        self.assertIn("x86_64-pc-windows-msvc", engine)
        self.assertNotIn("run", engine)
        self.assertEqual(windows["windows-link"][windows["windows-link"].index("--package") + 1],
                         "nevrotick-ui")

    def test_engine_runtime_boundary_and_existing_release_guard_are_required(self):
        rows = dict(VERIFY.commands("portable"))
        for label in ("product-tests:nevrotick-engine", "optimized-tests:nevrotick-engine"):
            command = rows[label]
            self.assertEqual(command[command.index("--package") + 1], "nevrotick-engine")
            self.assertIn("--lib", command)
            self.assertIn("--tests", command)
            self.assertNotIn("--test", command)
            self.assertIn("--no-tests=fail", command)
        optimized = rows["optimized-tests:nevrotick-engine"]
        self.assertEqual(optimized[optimized.index("--cargo-profile") + 1], "release")
        self.junit.write_text('<testsuites><testsuite name="nevrotick-engine">'
                              '<testcase name="processing_guard_tests::processing_uses_fresh_deadline_and_current_control"/>'
                              '</testsuite><testsuite name="nevrotick-engine::runtime_boundary">'
                              '<testcase name="prepare_rejects_entered_runtime_before_native_startup"/>'
                              '</testsuite></testsuites>')
        self.assertEqual(VERIFY.validate_test_result("optimized-tests:nevrotick-engine", "", self.work)
                         ["executed"], 2)
        self.junit.write_text('<testsuites><testsuite name="nevrotick-engine">'
                              '<testcase name="processing_guard_tests::processing_uses_fresh_deadline_and_current_control"/>'
                              '</testsuite><testsuite name="nevrotick-engine::runtime_boundary">'
                              '<testcase name="prepare_outside_runtime_owns_and_releases_standard_sockets"/>'
                              '</testsuite></testsuites>')
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "discriminator"):
            VERIFY.validate_test_result("optimized-tests:nevrotick-engine", "", self.work)

    def test_engine_runtime_only_junit_cannot_hide_missing_processing_unit(self):
        """The three historical integration passes do not execute the library guard."""
        self.junit.write_text('<testsuites><testsuite name="nevrotick-engine::runtime_boundary">'
                              '<testcase name="prepare_outside_runtime_owns_and_releases_standard_sockets"/>'
                              '<testcase name="prepare_rejects_entered_runtime_before_native_startup"/>'
                              '<testcase name="execute_rejects_entered_runtime_without_nested_runtime_creation"/>'
                              '</testsuite></testsuites>')
        for lane in ("product-tests:nevrotick-engine", "optimized-tests:nevrotick-engine"):
            with self.subTest(lane=lane), self.assertRaisesRegex(VERIFY.ENV.Incomplete, "processing guard"):
                VERIFY.validate_test_result(lane, "", self.work)

    def test_engine_processing_unit_has_healthy_ordinary_and_optimized_neighbors(self):
        self.junit.write_text('<testsuites><testsuite name="nevrotick-engine">'
                              '<testcase name="processing_guard_tests::processing_uses_fresh_deadline_and_current_control"/>'
                              '</testsuite><testsuite name="nevrotick-engine::runtime_boundary">'
                              '<testcase name="prepare_rejects_entered_runtime_before_native_startup"/>'
                              '<testcase name="future_additional_cli_case"/></testsuite></testsuites>')
        for lane in ("product-tests:nevrotick-engine", "optimized-tests:nevrotick-engine"):
            with self.subTest(lane=lane):
                result = VERIFY.validate_test_result(lane, "", self.work)
                self.assertEqual(result["executed"], 3)
                self.assertIn("future_additional_cli_case", result["test_names"])

    def test_engine_processing_name_does_not_override_failed_or_wrong_owner_junit(self):
        for lane in ("product-tests:nevrotick-engine", "optimized-tests:nevrotick-engine"):
            for outcome in ("skipped", "failure", "error"):
                self.junit.write_text('<testsuites><testsuite name="nevrotick-engine">'
                                      '<testcase name="processing_guard_tests::processing_uses_fresh_deadline_and_current_control">'
                                      f'<{outcome}/></testcase>'
                                      '<testcase name="prepare_rejects_entered_runtime_before_native_startup"/>'
                                      '</testsuite></testsuites>')
                with self.subTest(lane=lane, outcome=outcome), self.assertRaisesRegex(VERIFY.ENV.Incomplete, "empty, skipped or failed"):
                    VERIFY.validate_test_result(lane, "", self.work)
            self.junit.write_text('<testsuites><testsuite name="time-core">'
                                  '<testcase name="processing_guard_tests::processing_uses_fresh_deadline_and_current_control"/>'
                                  '<testcase name="prepare_rejects_entered_runtime_before_native_startup"/>'
                                  '</testsuite></testsuites>')
            with self.subTest(lane=lane, outcome="wrong-owner"), self.assertRaisesRegex(VERIFY.ENV.Incomplete, "empty, skipped or failed"):
                VERIFY.validate_test_result(lane, "", self.work)

    def test_engine_doctests_preserve_nonempty_execution_requirement(self):
        command = dict(VERIFY.commands("portable"))["doctests:nevrotick-engine"]
        self.assertIn("--doc", command)
        self.assertEqual(VERIFY.validate_test_result("doctests:nevrotick-engine",
                         "test result: ok. 2 passed; 0 failed; 0 ignored;", self.work),
                         {"package": "nevrotick-engine", "executed": 2})
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "nonempty"):
            VERIFY.validate_test_result("doctests:nevrotick-engine",
                                        "test result: ok. 0 passed; 0 failed; 0 ignored;", self.work)

    def test_portable_output_retains_current_engine_before_build_directory_disposal(self):
        """Exercise the real producer with a declared simulated build, not Rust evidence."""
        project = self.work / "project"
        project.mkdir()
        names = ("Cargo.lock", "deny.toml", ".config/nextest.toml", *VERIFY.CHECKER_FILES)
        for name in names:
            destination = project / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, destination)
        identity = {name: VERIFY.sha256(project / name) for name in names}
        for directory in ("vendor", "toolchain", "scratch"):
            (self.work / directory).mkdir()
        output = self.work / "export"
        args = argparse.Namespace(lane="portable", vendor_dir=self.work / "vendor",
                                  toolchain_dir=self.work / "toolchain", scratch_root=self.work / "scratch",
                                  nextest=None, deny=None, output_dir=output)
        payload = b"\x7fELFdeclared-unit-build-fixture-never-executed"
        built = []

        def simulated_build(_toolchain, work, _source, _vendor, command, *_args):
            binary = work / "target/debug/nevrotick-observe"
            binary.parent.mkdir(parents=True)
            binary.write_bytes(payload)
            binary.chmod(0o700)
            built.append(binary)
            return command

        report = {}
        with patch.object(VERIFY, "admit_product", return_value=({"schema_version": 2}, identity)), \
                patch.object(VERIFY, "admit_vendor", return_value={}), \
                patch.object(VERIFY, "admit_tools", return_value={}), \
                patch.object(VERIFY, "commands", return_value=[("host-engine-build", ["simulated-owned-build"])]), \
                patch.object(VERIFY, "boundary_control"), \
                patch.object(VERIFY, "sandbox", side_effect=simulated_build), \
                patch.object(VERIFY.ENV, "run", return_value={"exit_code": 0, "failure": None, "output": ""}), \
                patch.object(VERIFY, "summarize_test_counts", return_value={}):
            VERIFY.execute(project, args, report)
        self.assertFalse(built[0].exists())
        self.assertTrue((output / "nevrotick-observe").is_file())
        self.assertEqual((output / "nevrotick-observe").read_bytes(), payload)
        self.assertEqual(report["engine_artifact"]["sha256"], digest(payload))

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
        (release / "nevrotick-observe.exe").write_bytes(b"engine-control")
        (release / "nevrotick_observe.pdb").write_bytes(b"engine-symbol-control")
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
        self.assertEqual(Path(report["engine"]["path"]).read_bytes(), b"engine-control")
        self.assertEqual(report["engine"]["symbols"]["sha256"], digest(b"engine-symbol-control"))
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

    def test_windows_engine_artifacts_are_admitted_before_any_ui_copy(self):
        release = self.work / "target/x86_64-pc-windows-msvc/release"
        release.mkdir(parents=True)
        for name in ("nevrotick-ui.exe", "nevrotick_ui.pdb", "nevrotick-observe.exe"):
            (release / name).write_bytes(b"independent-artifact-control")
        output = self.work / "export"
        output.mkdir()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "matching PDB"):
            VERIFY.export_windows_artifacts(self.work, output)
        self.assertEqual(list(output.iterdir()), [])
        (release / "nevrotick_observe.pdb").write_bytes(b"engine-symbol")
        (release / "nevrotick-observe.pdb").write_bytes(b"ambiguous-symbol")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "matching PDB"):
            VERIFY.export_windows_artifacts(self.work, output)
        self.assertEqual(list(output.iterdir()), [])

    def test_portable_row_keeps_doctests_separate_and_empty_tests_fatal(self):
        rows = dict(VERIFY.commands("portable"))
        self.assertIn("--workspace", rows["compiler"])
        for package in VERIFY.PACKAGES:
            self.assertIn("--no-tests=fail", rows["product-tests:" + package])
            self.assertIn("--lib", rows["product-tests:" + package])
            self.assertIn("--tests", rows["product-tests:" + package])
            self.assertIn("--doc", rows["doctests:" + package])
        self.assertEqual(set(rows), {"format", "compiler", "clippy", "rustdoc", "host-engine-build",
                                    "product-tests:nevrotick-ui", "product-tests:ntp-wire", "product-tests:time-domain",
                                    "product-tests:ntp-exchange", "product-tests:time-core",
                                    "doctests:nevrotick-ui", "doctests:ntp-wire", "doctests:time-domain",
                                    "doctests:ntp-exchange", "doctests:time-core", "optimized-tests:time-domain",
                                    "optimized-tests:ntp-exchange", "optimized-tests:time-core",
                                    "product-tests:nevrotick-engine", "doctests:nevrotick-engine",
                                    "product-tests:windows-platform", "doctests:windows-platform",
                                    "optimized-tests:nevrotick-engine"})

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
                   for package, count in (("nevrotick-ui", 3), ("ntp-wire", 2), ("time-domain", 4), ("ntp-exchange", 6), ("time-core", 8), ("nevrotick-engine", 3), ("windows-platform", 5))
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
        records.append({"lane": "optimized-tests:nevrotick-engine", "tests": {
            "package": "nevrotick-engine", "executed": 3,
            "test_names": ["prepare_rejects_entered_runtime_before_native_startup"]}})
        self.assertEqual(VERIFY.summarize_test_counts(records),
                         {"by_package": {"nevrotick-ui": {"product-tests": 3, "doctests": 3},
                                         "ntp-wire": {"product-tests": 2, "doctests": 2},
                                         "time-domain": {"product-tests": 4, "doctests": 4, "optimized-tests": 5},
                                         "ntp-exchange": {"product-tests": 6, "doctests": 6, "optimized-tests": 7},
                                         "time-core": {"product-tests": 8, "doctests": 8, "optimized-tests": 9},
                                         "nevrotick-engine": {"product-tests": 3, "doctests": 3, "optimized-tests": 3},
                                         "windows-platform": {"product-tests": 5, "doctests": 5}},
                          "tests_executed": 31, "doctests_executed": 31, "optimized_tests_executed": 24})
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


class HostEngineArtifactTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="host-engine-export-control-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.work = self.root / "work"
        self.output = self.root / "output"
        self.output.mkdir()
        self.binary = self.work / "target/debug/nevrotick-observe"
        self.binary.parent.mkdir(parents=True)
        self.payload = b"\x7fELFindependent-byte-fixture-never-executed"
        self.binary.write_bytes(self.payload)
        self.binary.chmod(0o700)

    def test_current_binary_bytes_identity_and_owner_execution_are_retained(self):
        artifact = VERIFY.export_host_engine_artifact(self.work, self.output)
        copied = self.output / "nevrotick-observe"
        self.assertEqual(copied.read_bytes(), self.payload)
        self.assertEqual(copied.stat().st_mode & 0o777, 0o700)
        self.assertEqual(artifact, {"path": str(copied), "sha256": digest(self.payload),
                                   "profile": "dev", "target": "x86_64-unknown-linux-gnu",
                                   "binary_name": "nevrotick-observe"})

    def test_missing_linked_or_nonexecutable_source_is_rejected_before_copy(self):
        self.binary.unlink()
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "regular nonlinked"):
            VERIFY.export_host_engine_artifact(self.work, self.output)
        other = self.root / "other"
        other.write_bytes(self.payload)
        other.chmod(0o700)
        self.binary.symlink_to(other)
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "regular nonlinked"):
            VERIFY.export_host_engine_artifact(self.work, self.output)
        self.binary.unlink()
        self.binary.write_bytes(self.payload)
        self.binary.chmod(0o600)
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "owner-executable"):
            VERIFY.export_host_engine_artifact(self.work, self.output)
        self.assertEqual(list(self.output.iterdir()), [])

    def test_existing_destination_is_unchanged(self):
        destination = self.output / "nevrotick-observe"
        destination.write_bytes(b"preserved-independent-artifact")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "existing destination"):
            VERIFY.export_host_engine_artifact(self.work, self.output)
        self.assertEqual(destination.read_bytes(), b"preserved-independent-artifact")

    def test_corrupted_copy_is_rejected_by_actual_byte_hash(self):
        def corrupt_copy(_origin, target):
            target.write(b"different-independent-bytes")

        with patch.object(VERIFY.shutil, "copyfileobj", side_effect=corrupt_copy):
            with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "copied bytes changed"):
                VERIFY.export_host_engine_artifact(self.work, self.output)
        self.assertEqual(self.binary.read_bytes(), self.payload)


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


class SlintCaptureControls(unittest.TestCase):
    """Capture opt-in and binary image custody controls; no renderer executes."""

    def test_capture_uses_one_fixed_dev_ui_debug_override_without_test_mode(self):
        expected = ["cargo", "--config", "profile.dev.package.nevrotick-ui.debug=0", "rustc",
                    "--frozen", "--offline", "--jobs", "6", "--package", "nevrotick-ui",
                    "--example", "capture", "--", "-C", "strip=debuginfo"]
        self.assertEqual(VERIFY.slint_capture_build_command(), expected)
        self.assertNotIn("--profile", expected)
        self.assertNotIn("--test", expected)

    def test_capture_debug_override_never_leaks_into_ordinary_build_rows(self):
        for lane in ("portable", "windows", "dependencies"):
            for schema, patches in ((2, ()), (3, ("spin_on", "accesskit_windows"))):
                rows = VERIFY.commands(lane, schema_version=schema, patched_packages=patches)
                for label, command in rows:
                    with self.subTest(lane=lane, schema=schema, label=label):
                        self.assertFalse(any("profile.dev.package.nevrotick-ui.debug" in arg for arg in command))
                        self.assertNotIn("strip=debuginfo", command)
                        self.assertNotIn("debuginfo=0", command)

    def test_capture_requires_explicit_portable_sdk_and_external_output(self):
        args = argparse.Namespace(lane="portable",linux_sdk_dir=Path("/owned/sdk"),
                                  output_dir=Path("/owned/new-output"),capture_slint=True)
        self.assertTrue(VERIFY.require_slint_capture(args,{"schema_version":3}))
        for name,value in (("lane","windows"),("linux_sdk_dir",None),("output_dir",None),
                           ("capture_slint",1)):
            before = getattr(args,name)
            setattr(args,name,value)
            with self.subTest(field=name),self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.require_slint_capture(args,{"schema_version":3})
            setattr(args,name,before)
        with self.assertRaises(VERIFY.ENV.Incomplete):
            VERIFY.require_slint_capture(args,{"schema_version":2})
        args.capture_slint = False
        self.assertFalse(VERIFY.require_slint_capture(args,{"schema_version":2}))

    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.source = self.work / "slint-captures"
        self.source.mkdir()
        self.output = self.work / "output"
        self.output.mkdir()
        # Independent expected matrix, not supplied by the production helper.
        self.expected = {f"{mode}-{page}-{width}.ppm":width
            for width in (640,1080)
            for mode,pages in (("simple",("status","settings","about")),
                              ("power",("status","sources","statistics","diagnostics","settings","about")))
            for page in pages}
        self.expected.update({"source-invalid-1080.ppm":1080,"interval-invalid-1080.ppm":1080,
                              "interval-invalid-640.ppm":640,"source-add-ntp-1080.ppm":1080,
                              "source-review-1080.ppm":1080,"source-empty-640.ppm":640,
                              "source-empty-1080.ppm":1080})
        self.expected = {name:(width,780) for name,width in self.expected.items()}
        self.expected.update({"recovery-640.ppm":(640,780), "recovery-1080.ppm":(1080,780),
            "recovery-invalid-640.ppm":(640,780), "recovery-leave-640x600.ppm":(640,600),
            "recovery-invalid-640x600.ppm":(640,600),
            "interval-invalid-640x600.ppm":(640,600),
            "notifications-simple-640x600.ppm":(640,600),
            "personal-simple-640x600.ppm":(640,600),
            "quit-warning-top-640x600.ppm":(640,600), "quit-warning-bottom-640x600.ppm":(640,600),
            "quit-warning-long-top-640x600.ppm":(640,600), "quit-warning-long-bottom-640x600.ppm":(640,600),
            "service-simple-640.ppm":(640,780), "service-power-640x600.ppm":(640,600),
            "diagnostic-report-640.ppm":(640,780), "diagnostic-report-1080.ppm":(1080,780),
            "diagnostic-collection-640x600.ppm":(640,600), "diagnostic-correction-640x600.ppm":(640,600),
            "policy-ntp-review-640x600.ppm":(640,600), "policy-ntp-review-1080x780.ppm":(1080,780),
            "policy-observe-review-640x600.ppm":(640,600), "policy-observe-review-1080x780.ppm":(1080,780),
            "sources-query-640x600.ppm":(640,600), "sources-no-match-640x600.ppm":(640,600),
            "source-details-640x600.ppm":(640,600)})
        for name,(width,height) in self.expected.items():
            with (self.source/name).open("wb") as stream:
                stream.write(f"P6\n{width} {height}\n255\n".encode("ascii"))
                stream.truncate(len(f"P6\n{width} {height}\n255\n")+width*height*3)

    def test_exact_selected_frame_set_exports_with_bound_dimensions_and_hashes(self):
        report = VERIFY.export_slint_captures(self.work,self.output)
        self.assertEqual(set(report["files"]),set(self.expected))
        for name,record in report["files"].items():
            self.assertEqual((record["width"],record["height"]),self.expected[name])
            self.assertEqual(record["sha256"],VERIFY.sha256(self.output/"slint-captures"/name))

    def test_extra_truncated_or_linked_frame_rejects_before_export(self):
        extra = self.source/"unexpected.ppm"
        extra.write_bytes(b"Unlisted data")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete,"inventory"):
            VERIFY.export_slint_captures(self.work,self.output)
        extra.unlink()
        target = self.source/"simple-status-640.ppm"
        before = target.read_bytes()
        target.write_bytes(before[:-1])
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete,"truncated"):
            VERIFY.export_slint_captures(self.work,self.output)
        target.unlink()
        target.symlink_to(self.source/"simple-about-640.ppm")
        with self.assertRaisesRegex(VERIFY.ENV.Incomplete,"linked"):
            VERIFY.export_slint_captures(self.work,self.output)
        self.assertFalse((self.output/"slint-captures").exists())


class SlintSdkExecutionControls(unittest.TestCase):
    """SDK lane/custody integration controls; no subprocess or library executes."""

    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.project = self.root / "project"
        self.project.mkdir()
        names = ("Cargo.lock", "deny.toml", ".config/nextest.toml", *VERIFY.CHECKER_FILES,
                 *VERIFY.UI_TEXT_CHECKER_FILES, *VERIFY.UI_NATIVE_INPUT_CHECKER_FILES)
        for name in names:
            path = self.project / name
            path.parent.mkdir(parents=True,exist_ok=True)
            if (ROOT / name).is_file():
                shutil.copyfile(ROOT / name,path)
            else:
                path.write_text("Inert mocked text checker; never executed")
        (self.project / VERIFY.SDK.POLICY).parent.mkdir(exist_ok=True)
        (self.project / VERIFY.SDK.POLICY).write_text("Inert mocked SDK policy; never parsed")
        self.identity = {name:VERIFY.sha256(self.project / name) for name in names}
        for name in ("vendor","toolchain","scratch","sdk"):
            (self.root / name).mkdir()
        self.args = argparse.Namespace(lane="portable",vendor_dir=self.root/"vendor",
            toolchain_dir=self.root/"toolchain",scratch_root=self.root/"scratch",
            nextest=None,deny=None,output_dir=None,linux_sdk_dir=self.root/"sdk")

    def invoke(self,policy):
        with patch.object(VERIFY,"admit_product",return_value=(policy,dict(self.identity))), \
                patch.object(VERIFY,"admit_vendor",return_value={}), \
                patch.object(VERIFY,"admit_tools",return_value={}), \
                patch.object(VERIFY,"commands",return_value=[]), \
                patch.object(VERIFY,"text_contract_commands",return_value=[]), \
                patch.object(VERIFY,"native_input_control_commands",return_value=[]), \
                patch.object(VERIFY,"boundary_control"), \
                patch.object(VERIFY,"summarize_test_counts",return_value={}), \
                patch.object(VERIFY.ENV,"run") as subprocess:
            try:
                VERIFY.execute(self.project,self.args,{})
            finally:
                subprocess.assert_not_called()

    def test_sdk_wrong_lane_or_legacy_profile_rejects_before_native_input(self):
        for lane,schema in (("windows",3),("dependencies",3),("portable",2)):
            self.args.lane = lane
            with self.subTest(lane=lane,schema=schema),patch.object(VERIFY.SDK,"admit") as native:
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete,"only to portable Slint"):
                    self.invoke({"schema_version":schema})
                native.assert_not_called()

    def test_sdk_drift_rejects_and_omitted_sdk_preserves_default_execution(self):
        with patch.object(VERIFY.SDK,"admit",side_effect=[{"fixture":"before"},{"fixture":"after"}]) as native:
            with self.assertRaisesRegex(VERIFY.ENV.Incomplete,"SDK inputs changed"):
                self.invoke({"schema_version":3})
            self.assertEqual(native.call_count,2)
        self.args.linux_sdk_dir = None
        with patch.object(VERIFY.SDK,"admit") as native:
            self.invoke({"schema_version":2})
            native.assert_not_called()


class NativeInputExecutionControls(unittest.TestCase):
    """Reach actual pre-Cargo dispatch with an explicitly simulated runner."""

    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="native-input-row-control-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.project = self.root / "project"
        self.project.mkdir()
        names = ("Cargo.lock", "deny.toml", ".config/nextest.toml", *VERIFY.CHECKER_FILES,
                 *VERIFY.UI_TEXT_CHECKER_FILES, *VERIFY.UI_NATIVE_INPUT_CHECKER_FILES)
        for name in names:
            path = self.project / name
            path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, path)
        self.identity = {name: VERIFY.sha256(self.project / name) for name in names}
        for name in ("vendor", "toolchain", "scratch"):
            (self.root / name).mkdir()
        self.args = argparse.Namespace(lane="portable", vendor_dir=self.root / "vendor",
            toolchain_dir=self.root / "toolchain", scratch_root=self.root / "scratch",
            nextest=None, deny=None, output_dir=None)

    def invoke(self, output, report, *, accepted):
        simulated = []
        def runner(command, **_kwargs):
            simulated.append(command)
            return {"exit_code": 0, "failure": None,
                    "output": output if command[0] == "python3" else ""}
        with patch.object(VERIFY, "admit_product", return_value=({"schema_version": 3}, dict(self.identity))), \
                patch.object(VERIFY, "admit_vendor", return_value={}), \
                patch.object(VERIFY, "admit_tools", return_value={}), \
                patch.object(VERIFY, "commands", return_value=[("format", ["SIMULATED-CARGO-NEVER-EXECUTED"])]) as cargo, \
                patch.object(VERIFY, "text_contract_commands", return_value=[]), \
                patch.object(VERIFY, "boundary_control"), \
                patch.object(VERIFY, "summarize_test_counts", return_value={}), \
                patch.object(VERIFY, "sandbox", side_effect=lambda *_args: _args[4]), \
                patch.object(VERIFY.ENV, "run", side_effect=runner):
            if accepted:
                VERIFY.execute(self.project, self.args, report)
                self.assertEqual(len(simulated), 2)
                self.assertEqual(simulated[-1], ["SIMULATED-CARGO-NEVER-EXECUTED"])
                cargo.assert_called_once()
            else:
                with self.assertRaises(VERIFY.ENV.Incomplete):
                    VERIFY.execute(self.project, self.args, report)
                self.assertEqual(len(simulated), 1)
                cargo.assert_not_called()
        self.assertEqual(simulated[0], VERIFY.native_input_control_commands("portable", schema_version=3)[0][1])

    def test_empty_zero_skipped_or_failed_native_input_controls_stop_before_cargo(self):
        for output in ("", "Ran 0 tests in 0.1s\n\nOK\n",
                       "Ran 1 test in 0.1s\n\nOK (skipped=1)\n",
                       "Ran 1 test in 0.1s\n\nFAILED\n"):
            report = {}
            with self.subTest(output=output):
                self.invoke(output, report, accepted=False)
                self.assertEqual([row["lane"] for row in report["lanes"]], ["ui-native-input-controls"])
                self.assertNotIn("executed_controls", report["lanes"][0])
                self.assertNotEqual(report.get("status"), "passed")

    def test_nonempty_native_input_result_is_recorded_before_simulated_cargo(self):
        report = {}
        self.invoke("Ran 2 tests in 0.1s\n\nOK\n", report, accepted=True)
        self.assertEqual(report["lanes"][0]["executed_controls"], 2)
        self.assertEqual(report["status"], "passed")
        for name in VERIFY.UI_NATIVE_INPUT_CHECKER_FILES:
            self.assertEqual(report["checker_sha256"][name], self.identity[name])


if __name__ == "__main__":
    unittest.main()
