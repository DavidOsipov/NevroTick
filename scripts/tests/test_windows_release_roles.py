"""Independent source-only controls for the fixed Windows release export pair."""
from __future__ import annotations

import copy
import importlib.util
from pathlib import Path
import shutil
import tempfile
import tomllib
import unittest
from unittest.mock import patch


ROOT: Path = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("windows_release_strict", ROOT / "scripts/check-strict.py")
STRICT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(STRICT)
SOURCE_FILES: tuple[str, ...] = (
    "policy/verification-manifest.toml",
    "crates/nevrotick-ui/Cargo.toml", "crates/nevrotick-ui/src/main.rs",
    "crates/nevrotick-engine/Cargo.toml", "crates/nevrotick-engine/src/bin/nevrotick-observe.rs",
)


class WindowsReleaseRoleTests(unittest.TestCase):
    """Challenge the admission table and actual source checkout independently."""

    def setUp(self) -> None:
        temporary: tempfile.TemporaryDirectory = tempfile.TemporaryDirectory(prefix="nevrotick-release-roles-")
        self.addCleanup(temporary.cleanup)
        self.root: Path = Path(temporary.name)
        name: str
        for name in SOURCE_FILES:
            destination: Path = self.root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, destination)
        self.inventory: dict = tomllib.loads((self.root / SOURCE_FILES[0]).read_text(encoding="utf-8"))

    def refused(self, inventory: dict) -> None:
        with self.assertRaises(STRICT.ENV.Incomplete):
            STRICT.windows_release_role_bindings(self.root, inventory)

    def test_current_source_is_healthy_and_records_remain_immutable(self) -> None:
        bindings: dict[str, STRICT.WindowsReleaseRole] = STRICT.windows_release_role_bindings(self.root)
        self.assertEqual(set(bindings), {"ui", "observer"})
        self.assertEqual((bindings["ui"].artifact, bindings["ui"].package,
                          bindings["ui"].subsystem, bindings["ui"].source_state),
                         ("nevrotick-ui.exe", "nevrotick-ui", "gui", "fixture"))
        self.assertEqual((bindings["observer"].artifact, bindings["observer"].package,
                          bindings["observer"].subsystem, bindings["observer"].source_state),
                         ("nevrotick-observe.exe", "nevrotick-engine", "console", "experiment"))
        self.assertEqual(bindings["ui"].preentry_owner, "scripts/verify.py:compile_ui_entry")
        self.assertEqual(bindings["observer"].preentry_owner, "toolchain")
        self.assertEqual(bindings["observer"].required_features, ())
        self.assertEqual(bindings["ui"].runtime_policy, "pending-complete-role-qualification")
        with self.assertRaises(AttributeError):
            bindings["ui"].subsystem = "console"

    def test_missing_duplicate_and_unknown_roles_are_refused(self) -> None:
        changed: dict = copy.deepcopy(self.inventory)
        del changed["windows_release_roles"]
        self.refused(changed)
        changed = copy.deepcopy(self.inventory)
        changed["windows_release_roles"]["roles"].pop()
        self.refused(changed)
        replacement: str
        for replacement in ("ui", "broker"):
            with self.subTest(replacement=replacement):
                changed = copy.deepcopy(self.inventory)
                changed["windows_release_roles"]["roles"][1]["role"] = replacement
                self.refused(changed)

    def test_strict_configuration_uses_the_same_source_binding_admission(self) -> None:
        name: str
        for name in STRICT.CONFIG_FILES:
            destination: Path = self.root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, destination)
        snapshot: dict[str, bytes] = {name: (self.root / name).read_bytes() for name in STRICT.CONFIG_FILES}
        self.assertIn("policy/verification-manifest.toml", STRICT.read_configuration(self.root, snapshot))
        snapshot["policy/verification-manifest.toml"] = snapshot["policy/verification-manifest.toml"].replace(
            b'runtime_policy = "pending-complete-role-qualification"', b'runtime_policy = "enforced"', 1)
        with self.assertRaises(STRICT.ENV.Incomplete):
            STRICT.read_configuration(self.root, snapshot)

    def test_closed_section_and_exact_primitive_types_are_required(self) -> None:
        field: str
        value: object
        for field, value in (("schema_version", True), ("schema_version", 1.0),
                             ("schema_version", "1"), ("scope", False), ("roles", {})):
            with self.subTest(field=field, value=value):
                changed: dict = copy.deepcopy(self.inventory)
                changed["windows_release_roles"][field] = value
                self.refused(changed)
        changed = copy.deepcopy(self.inventory)
        changed["windows_release_roles"]["observed_enforced"] = True
        self.refused(changed)
        for field, value in (("role", True), ("artifact", 1), ("subsystem", False),
                             ("required_features", ""), ("required_features", [True])):
            with self.subTest(field=field, value=value):
                changed = copy.deepcopy(self.inventory)
                changed["windows_release_roles"]["roles"][0][field] = value
                self.refused(changed)

    def test_role_target_entry_and_qualification_substitution_are_refused(self) -> None:
        field: str
        value: object
        for field, value in (
                ("artifact", "nevrotick-observe.exe"), ("package", "nevrotick-engine"),
                ("cargo_manifest", "crates/nevrotick-engine/Cargo.toml"),
                ("entry_source", "crates/nevrotick-engine/src/bin/nevrotick-observe.rs"),
                ("entry_source", "../main.rs"), ("cargo_target", "declared-bin"),
                ("required_features", ["ke-association-prototype"]), ("subsystem", "console"),
                ("source_state", "production"), ("activation_owner", "SCM"),
                ("entry_scope", "complete-native-startup"), ("preentry_owner", "none"),
                ("runtime_policy", "enforced")):
            with self.subTest(field=field, value=value):
                changed: dict = copy.deepcopy(self.inventory)
                changed["windows_release_roles"]["roles"][0][field] = value
                self.refused(changed)
        changed = copy.deepcopy(self.inventory)
        changed["windows_release_roles"]["roles"][0]["observed"] = "passed"
        self.refused(changed)
        changed = copy.deepcopy(self.inventory)
        del changed["windows_release_roles"]["roles"][0]["runtime_policy"]
        self.refused(changed)

    def test_selected_cargo_target_changes_are_refused(self) -> None:
        manifest_path: Path = self.root / "crates/nevrotick-engine/Cargo.toml"
        original: str = manifest_path.read_text(encoding="utf-8")
        before: str
        after: str
        for before, after in (
                ('name = "nevrotick-engine"', 'name = "other-package"'),
                ('name = "nevrotick-observe"', 'name = "other-target"'),
                ('path = "src/bin/nevrotick-observe.rs"', 'path = "src/main.rs"'),
                ('path = "src/bin/nevrotick-observe.rs"',
                 'path = "src/bin/nevrotick-observe.rs"\nrequired-features = ["ke-association-prototype"]'),
                ('path = "src/bin/nevrotick-observe.rs"',
                 'path = "src/bin/nevrotick-observe.rs"\nrequired-features = false')):
            with self.subTest(change=after):
                self.assertEqual(original.count(before), 1)
                manifest_path.write_text(original.replace(before, after, 1), encoding="utf-8")
                self.refused(self.inventory)
        manifest_path.write_text(original, encoding="utf-8")
        self.assertEqual(set(STRICT.windows_release_role_bindings(self.root)), {"ui", "observer"})

    def test_default_main_cannot_be_disabled_or_replaced(self) -> None:
        manifest_path: Path = self.root / "crates/nevrotick-ui/Cargo.toml"
        original: str = manifest_path.read_text(encoding="utf-8")
        changed: str
        for changed in (
                original.replace('[package]', '[package]\nautobins = false', 1),
                original.replace('[package]', '[package]\nautobins = 1', 1),
                original + '\n[[bin]]\nname = "nevrotick-ui"\npath = "src/other.rs"\n',
                original + '\n[[bin]]\nname = "renamed-ui"\npath = "src/main.rs"\n'):
            with self.subTest(change=changed[-100:]):
                manifest_path.write_text(changed, encoding="utf-8")
                self.refused(self.inventory)

    def test_explicit_source_root_cannot_fall_back_to_global_checkout(self) -> None:
        with patch.object(STRICT, "ROOT", self.root / "absent-checkout"):
            self.assertEqual(set(STRICT.windows_release_role_bindings(self.root)), {"ui", "observer"})
        (self.root / "crates/nevrotick-ui/src/main.rs").unlink()
        with self.assertRaises(STRICT.ENV.Incomplete):
            STRICT.windows_release_role_bindings(self.root)

    def test_missing_redirected_and_oversized_sources_are_refused(self) -> None:
        name: str
        for name in SOURCE_FILES:
            with self.subTest(source=name):
                path: Path = self.root / name
                payload: bytes = path.read_bytes()
                path.unlink()
                with self.assertRaises(STRICT.ENV.Incomplete):
                    STRICT.windows_release_role_bindings(self.root)
                path.symlink_to(ROOT / name)
                with self.assertRaises(STRICT.ENV.Incomplete):
                    STRICT.windows_release_role_bindings(self.root)
                path.unlink()
                path.write_bytes(payload)
        path = self.root / "crates/nevrotick-ui/Cargo.toml"
        path.write_bytes(b"#" * (64 * 1024 + 1))
        with self.assertRaises(STRICT.ENV.Incomplete):
            STRICT.windows_release_role_bindings(self.root)

    def test_redirected_source_ancestor_is_refused(self) -> None:
        directory: Path = self.root / "crates/nevrotick-ui"
        retained: Path = self.root / "retained-ui"
        directory.rename(retained)
        directory.symlink_to(retained, target_is_directory=True)
        with self.assertRaises(STRICT.ENV.Incomplete):
            STRICT.windows_release_role_bindings(self.root)


class WindowsLabServiceRoleTests(unittest.TestCase):
    """Check standalone source attribution without granting runtime credit."""

    def setUp(self) -> None:
        temporary: tempfile.TemporaryDirectory = tempfile.TemporaryDirectory(prefix="nevrotick-lab-roles-")
        self.addCleanup(temporary.cleanup)
        self.root: Path = Path(temporary.name)
        name: str
        for name in ("policy/verification-manifest.toml", "tools/windows-service-fixture/Cargo.toml",
                     "tools/windows-service-fixture/src/bin/engine.rs",
                     "tools/windows-service-fixture/src/bin/broker.rs"):
            destination: Path = self.root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, destination)
        self.inventory: dict = tomllib.loads((self.root / "policy/verification-manifest.toml").read_text())

    def refused(self, inventory: dict) -> None:
        with self.assertRaises(STRICT.ENV.Incomplete):
            STRICT.windows_lab_service_role_bindings(self.root, inventory)

    def test_current_targets_bind_distinct_immutable_fixture_roles(self) -> None:
        bindings: dict[str, STRICT.WindowsReleaseRole] = STRICT.windows_lab_service_role_bindings(self.root)
        self.assertEqual(set(bindings), {"lab-engine", "lab-broker"})
        role: str
        basename: str
        entry: str
        for role, basename, entry in (
                ("lab-engine", "nevrotick-lab-s02-engine.exe", "engine.rs"),
                ("lab-broker", "nevrotick-lab-s02-broker.exe", "broker.rs")):
            self.assertEqual(bindings[role].artifact, basename)
            self.assertEqual(bindings[role].entry_source, "tools/windows-service-fixture/src/bin/" + entry)
            self.assertEqual(bindings[role].package, "nevrotick-windows-service-fixture")
            self.assertEqual(bindings[role].required_features, ())
            self.assertEqual((bindings[role].subsystem, bindings[role].source_state), ("console", "fixture"))
            self.assertEqual(bindings[role].activation_owner, "SCM-laboratory-controller")
            self.assertEqual(bindings[role].runtime_policy, "pending-complete-role-qualification")
            with self.assertRaises(AttributeError):
                bindings[role].role = "ui"

    def test_missing_duplicate_and_foreign_rosters_are_refused(self) -> None:
        changed: dict = copy.deepcopy(self.inventory)
        del changed["windows_lab_service_roles"]
        self.refused(changed)
        changed = copy.deepcopy(self.inventory)
        changed["windows_lab_service_roles"]["roles"].pop()
        self.refused(changed)
        replacement: str
        for replacement in ("lab-engine", "ui", "observer"):
            changed = copy.deepcopy(self.inventory)
            changed["windows_lab_service_roles"]["roles"][1]["role"] = replacement
            self.refused(changed)
        changed = copy.deepcopy(self.inventory)
        changed["windows_lab_service_roles"] = changed["windows_release_roles"]
        self.refused(changed)

    def test_schema_and_runtime_claims_are_closed(self) -> None:
        field: str
        value: object
        for field, value in (("schema_version", True), ("schema_version", 1.0), ("schema_version", 2),
                             ("scope", "workspace-release-export-pair-source-binding"),
                             ("roles", {}), ("observed_enforced", True)):
            changed: dict = copy.deepcopy(self.inventory)
            changed["windows_lab_service_roles"][field] = value
            self.refused(changed)
        for field, value in (("runtime_policy", "enforced"), ("source_state", "production"),
                             ("activation_owner", "entrypoint"), ("entry_scope", "complete-native-startup"),
                             ("preentry_owner", "toolchain"), ("role", True), ("required_features", [True]),
                             ("artifact", "nevrotick-lab-s02-broker.exe"), ("subsystem", "gui"),
                             ("entry_source", "tools/windows-service-fixture/src/bin/broker.rs"),
                             ("package", "nevrotick-engine"), ("observed", "passed")):
            changed = copy.deepcopy(self.inventory)
            changed["windows_lab_service_roles"]["roles"][0][field] = value
            self.refused(changed)

    def test_actual_package_entry_and_feature_drift_is_refused(self) -> None:
        manifest: Path = self.root / "tools/windows-service-fixture/Cargo.toml"
        original: str = manifest.read_text()
        altered: str
        for altered in (
                original.replace('name = "nevrotick-windows-service-fixture"', 'name = "other"', 1),
                original.replace('path = "src/bin/engine.rs"', 'path = "src/bin/broker.rs"', 1),
                original.replace('name = "nevrotick-lab-s02-engine"', 'name = "another-engine"', 1),
                original.replace('path = "src/bin/engine.rs"', 'path = "src/bin/engine.rs"\nrequired-features = ["changed"]', 1),
                original + '\n[[bin]]\nname = "nevrotick-lab-s02-engine"\npath = "src/bin/engine.rs"\n'):
            self.assertNotEqual(altered, original)
            try:
                manifest.write_text(altered)
                self.refused(self.inventory)
            finally:
                manifest.write_text(original)
        self.assertEqual(set(STRICT.windows_lab_service_role_bindings(self.root)), {"lab-engine", "lab-broker"})

    def test_missing_oversized_and_redirected_sources_are_refused(self) -> None:
        entry: Path = self.root / "tools/windows-service-fixture/src/bin/engine.rs"
        original: bytes = entry.read_bytes()
        try:
            entry.unlink()
            self.refused(self.inventory)
            entry.write_bytes(b"x" * (512 * 1024 + 1))
            self.refused(self.inventory)
            entry.unlink()
            retained: Path = self.root / "retained-engine.rs"
            retained.write_bytes(original)
            entry.symlink_to(retained)
            self.refused(self.inventory)
        finally:
            entry.unlink(missing_ok=True)
            entry.write_bytes(original)
        self.assertEqual(STRICT.windows_lab_service_role_bindings(self.root)["lab-engine"].artifact,
                         "nevrotick-lab-s02-engine.exe")

    def test_redirected_source_ancestor_is_refused(self) -> None:
        directory: Path = self.root / "tools/windows-service-fixture"
        retained: Path = self.root / "retained-service"
        directory.rename(retained)
        directory.symlink_to(retained, target_is_directory=True)
        self.refused(self.inventory)


if __name__ == "__main__":
    unittest.main()
