"""Independent SBOM expectations through the real offline generator boundary.

Fixtures contain only locally owned path packages. Real admitted Cargo and
cargo-cyclonedx run in the normal cleared-environment network sandbox; only the
environment-inspection result is reused from the real repository for temp roots.
"""

import copy
import importlib.util
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("sbom_controls", ROOT / "scripts/generate-sbom.py")
SBOM = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SBOM)


def valid_bom():
    return {
        "bomFormat": "CycloneDX", "specVersion": "1.5", "version": 1,
        "metadata": {
            "component": {"type": "application", "name": "root-control", "version": "1.0.0", "bom-ref": "root"},
            "properties": [{"name": "cdx:rustc:sbom:target:triple", "value": "x86_64-pc-windows-msvc"}],
        },
        "components": [{"type": "library", "name": "normal-control", "version": "1.0.0", "bom-ref": "normal"}],
        "dependencies": [{"ref": "root", "dependsOn": ["normal"]}, {"ref": "normal", "dependsOn": []}],
    }


class BomStructureControls(unittest.TestCase):
    def test_identified_connected_graph_is_a_healthy_control(self):
        SBOM.validate_bom(valid_bom())

    def test_dangling_dependency_and_missing_root_edges_are_rejected(self):
        for edges in ([{"ref": "root", "dependsOn": ["missing"]}],
                      [{"ref": "normal", "dependsOn": []}], []):
            with self.subTest(edges=edges):
                bom = valid_bom()
                bom["dependencies"] = edges
                with self.assertRaises(SBOM.ENV.Incomplete):
                    SBOM.validate_bom(bom)

    def test_duplicate_component_identity_is_rejected(self):
        bom = valid_bom()
        bom["components"].append(copy.deepcopy(bom["components"][0]))
        with self.assertRaises(SBOM.ENV.Incomplete):
            SBOM.validate_bom(bom)

    def test_malformed_or_unidentified_bom_is_rejected(self):
        for mutation in ({"specVersion": "1.3"}, {"bomFormat": "other"},
                         {"version": True}, {"metadata": {}}, {"version": 0}):
            with self.subTest(mutation=mutation):
                bom = valid_bom()
                bom.update(mutation)
                with self.assertRaises(SBOM.ENV.Incomplete):
                    SBOM.validate_bom(bom)

    def test_absent_product_source_does_not_create_output_or_admit_tools(self):
        with tempfile.TemporaryDirectory(prefix="sbom-no-source-") as temporary:
            area = Path(temporary)
            root, output = area / "checkout", area / "output"
            root.mkdir()
            with patch.object(SBOM.ENV, "inspect") as inspect:
                with self.assertRaisesRegex(SBOM.ENV.Incomplete, "pending"):
                    SBOM.generate(root, output, [], False)
                inspect.assert_not_called()
            self.assertFalse(output.exists())


class ReportAdmissionControls(unittest.TestCase):
    def test_exact_byte_budget_has_a_healthy_neighbor_and_rejects_one_more_byte(self):
        with tempfile.TemporaryDirectory(prefix="sbom-report-budget-", dir="/tmp") as temporary:
            report = Path(temporary) / "component.cdx.json"
            report.write_bytes(b"1234")
            with patch.object(SBOM, "MAX_REPORT_BYTES", 4):
                self.assertEqual(SBOM.read_report(report, 4), b"1234")
                with self.assertRaises(SBOM.ENV.Incomplete):
                    SBOM.read_report(report, 3)
                report.write_bytes(b"12345")
                with self.assertRaises(SBOM.ENV.Incomplete):
                    SBOM.read_report(report, 10)

    def test_symlink_and_directory_cannot_be_consumed_as_reports(self):
        with tempfile.TemporaryDirectory(prefix="sbom-report-kind-", dir="/tmp") as temporary:
            root = Path(temporary)
            report, alias = root / "component.cdx.json", root / "linked.cdx.json"
            report.write_bytes(b"{}")
            alias.symlink_to(report)
            for path in (alias, root):
                with self.subTest(path=path.name), self.assertRaises(SBOM.ENV.Incomplete):
                    SBOM.read_report(path, 1024)


class ActualCargoSbomControls(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.admitted_environment = SBOM.ENV.inspect(ROOT)
        cls.toolchain = Path(cls.admitted_environment["tools"]["rustc"]["path"]).parent.parent

    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="nevrotick-sbom-control-", dir="/tmp")
        self.addCleanup(temporary.cleanup)
        self.area = Path(temporary.name)
        self.work = self.area / "work"
        self.root = self.work / "fixture"
        (self.root / "src").mkdir(parents=True)
        (self.work / "cargo-home").mkdir()
        for name in ("policy/sbom.toml", "policy/tool-manifest.json", ".cargo/config.toml", "rust-toolchain.toml"):
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, path)
        subprocess.run(["git", "init", "--quiet", str(self.root)], check=True, capture_output=True, timeout=10)
        for name in ("normal", "build", "dev", "optional", "windows-only", "unix-only"):
            self.package(name)
        self.manifest = '''[package]
name = "independent-sbom-root"
version = "0.1.0"
edition = "2024"
license = "GPL-2.0-only"
[workspace]
resolver = "3"
exclude = ["deps/normal", "deps/build", "deps/dev", "deps/optional", "deps/windows-only", "deps/unix-only", "deps/switch", "deps/branch-windows", "deps/branch-unix"]
[dependencies]
control-normal = { path = "deps/normal", version = "=0.1.0" }
control-optional = { path = "deps/optional", version = "=0.1.0", optional = true }
[build-dependencies]
control-build = { path = "deps/build", version = "=0.1.0" }
[dev-dependencies]
control-dev = { path = "deps/dev", version = "=0.1.0" }
[target.'cfg(windows)'.dependencies]
control-windows-only = { path = "deps/windows-only", version = "=0.1.0" }
[target.'cfg(unix)'.dependencies]
control-unix-only = { path = "deps/unix-only", version = "=0.1.0" }
[features]
extra = ["dep:control-optional"]
'''
        (self.root / "Cargo.toml").write_text(self.manifest, encoding="utf-8")
        (self.root / "src/main.rs").write_text("fn main() {}\n", encoding="utf-8")
        (self.root / "build.rs").write_text(
            'fn main() { std::fs::write("BUILD_SCRIPT_EXECUTED", "unexpected").unwrap(); }\n',
            encoding="utf-8",
        )
        self.lock()

    def package(self, name, extra=""):
        path = self.root / "deps" / name
        (path / "src").mkdir(parents=True)
        (path / "Cargo.toml").write_text(
            f'[package]\nname = "control-{name}"\nversion = "0.1.0"\nedition = "2024"\n'
            f'license = "GPL-2.0-only"\n{extra}', encoding="utf-8",
        )
        (path / "src/lib.rs").write_text("pub fn control() {}\n", encoding="utf-8")

    def lock(self):
        command = SBOM.ENV.sandbox_command(self.toolchain, self.work, ["cargo", "generate-lockfile", "--offline"])
        SBOM.ENV.require_ok(SBOM.ENV.run(command, cwd=ROOT, timeout=30))

    def generate(self, output_name, features=()):
        output = self.area / output_name
        with patch.object(SBOM.ENV, "inspect", return_value=self.admitted_environment):
            SBOM.generate(self.root, output, list(features), False)
        boms = [json.loads(path.read_text()) for path in output.rglob("*.cdx.json")]
        self.assertEqual(len(boms), 1, "Only the independently selected root is a workspace member")
        self.assertEqual(boms[0]["metadata"]["component"]["name"], "independent-sbom-root")
        return boms[0]

    def add_second_workspace_member(self):
        member = self.root / "member-two"
        (member / "src").mkdir(parents=True)
        (member / "Cargo.toml").write_text(
            '[package]\nname = "independent-second-root"\nversion = "0.1.0"\n'
            'edition = "2024"\nlicense = "GPL-2.0-only"\n', encoding="utf-8",
        )
        (member / "src/lib.rs").write_text("pub fn second_control() {}\n", encoding="utf-8")
        (self.root / "Cargo.toml").write_text(self.manifest.replace(
            '[workspace]\n', '[workspace]\nmembers = ["member-two"]\n',
        ), encoding="utf-8")
        self.lock()

    def test_every_actual_workspace_member_has_exactly_one_inventory(self):
        self.add_second_workspace_member()
        output = self.area / "two-member-control"
        with patch.object(SBOM.ENV, "inspect", return_value=self.admitted_environment):
            SBOM.generate(self.root, output, [], False)
        boms = [json.loads(path.read_text()) for path in output.rglob("*.cdx.json")]
        self.assertEqual(len(boms), 2)
        self.assertEqual(
            {bom["metadata"]["component"]["name"] for bom in boms},
            {"independent-sbom-root", "independent-second-root"},
        )
        self.assertFalse((self.root / "BUILD_SCRIPT_EXECUTED").exists())

    def test_missing_duplicate_and_dependency_root_reports_are_withheld(self):
        self.add_second_workspace_member()
        native_run = SBOM.ENV.run
        for mutation in ("omitted", "duplicated", "dependency-root"):
            output = self.area / ("rejected-" + mutation)
            reached_generator = []

            def corrupt_emission(command, **kwargs):
                result = native_run(command, **kwargs)
                if ("--" in command and "/tools/cargo-cyclonedx" in command[command.index("--") + 1:]
                        and result["exit_code"] == 0):
                    reached_generator.append(True)
                    fixture = Path(command[command.index("--bind") + 1]) / "fixture"
                    reports = {json.loads(path.read_text())["metadata"]["component"]["name"]: path
                               for path in fixture.rglob("*.cdx.json")}
                    self.assertEqual(set(reports), {"independent-sbom-root", "independent-second-root"})
                    report = reports["independent-second-root"]
                    if mutation == "omitted":
                        report.unlink()
                    elif mutation == "duplicated":
                        (fixture / "duplicated-root.cdx.json").write_bytes(report.read_bytes())
                    else:
                        # A dependency is a real package in the graph, but it
                        # cannot replace the intended workspace member's BOM.
                        report.write_text(json.dumps({
                            "bomFormat": "CycloneDX", "specVersion": "1.5", "version": 1,
                            "metadata": {"component": {
                                "type": "library", "name": "control-normal",
                                "version": "0.1.0", "bom-ref": "replacement",
                            }},
                            "dependencies": [{"ref": "replacement", "dependsOn": []}],
                        }), encoding="utf-8")
                return result

            with self.subTest(mutation=mutation), \
                 patch.object(SBOM.ENV, "inspect", return_value=self.admitted_environment), \
                 patch.object(SBOM.ENV, "run", side_effect=corrupt_emission):
                with self.assertRaisesRegex(SBOM.ENV.Incomplete, "workspace member"):
                    SBOM.generate(self.root, output, [], False)
            self.assertEqual(reached_generator, [True], "Fault must reach actual successful generator emission")
            self.assertFalse(output.exists(), "Partial, duplicate or wrong-root inventory must never become output")
            self.assertFalse((self.root / "BUILD_SCRIPT_EXECUTED").exists())

    def test_actual_windows_graph_and_optional_feature_rows(self):
        before = SBOM.source_snapshot(self.root)
        expected = {"control-normal": "required", "control-build": "excluded", "control-windows-only": "required"}
        for name, features in (("optional-off", []), ("optional-on", ["extra"])):
            with self.subTest(features=features):
                bom = self.generate(name, features)
                wanted = expected | ({"control-optional": "required"} if features else {})
                self.assertEqual({component["name"]: component["scope"] for component in bom["components"]}, wanted)
                self.assertIn({"name": "cdx:rustc:sbom:target:triple", "value": "x86_64-pc-windows-msvc"},
                              bom["metadata"]["properties"])
                self.assertFalse((self.root / "BUILD_SCRIPT_EXECUTED").exists())
                self.assertEqual(SBOM.source_snapshot(self.root), before)

    def test_target_conditional_feature_leakage_is_rejected_without_output(self):
        self.package("branch-windows")
        self.package("branch-unix")
        self.package("switch", '''[dependencies]
control-branch-windows = { path = "../branch-windows", optional = true }
control-branch-unix = { path = "../branch-unix", optional = true }
[features]
windows = ["dep:control-branch-windows"]
unix = ["dep:control-branch-unix"]
''')
        manifest = self.manifest.replace(
            "[target.'cfg(windows)'.dependencies]\n",
            "[target.'cfg(windows)'.dependencies]\n"
            'control-switch = { path = "deps/switch", default-features = false, features = ["windows"] }\n',
        ).replace(
            "[target.'cfg(unix)'.dependencies]\n",
            "[target.'cfg(unix)'.dependencies]\n"
            'control-switch = { path = "deps/switch", default-features = false, features = ["unix"] }\n',
        )
        (self.root / "Cargo.toml").write_text(manifest, encoding="utf-8")
        self.lock()
        output = self.area / "rejected-leak"
        with patch.object(SBOM.ENV, "inspect", return_value=self.admitted_environment):
            with self.assertRaisesRegex(SBOM.ENV.Incomplete, "Cargo tree"):
                SBOM.generate(self.root, output, [], False)
        self.assertFalse(output.exists(), "Known false target inventory must never be published as successful output")
        self.assertFalse((self.root / "BUILD_SCRIPT_EXECUTED").exists())


if __name__ == "__main__":
    unittest.main()
