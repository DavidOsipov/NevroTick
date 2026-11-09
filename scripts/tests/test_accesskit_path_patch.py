"""Source-only second-local-patch controls; no Cargo or native execution."""
import copy
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
SPEC = importlib.util.spec_from_file_location("accesskit_fixture_profile", Path(__file__).with_name("test_slint_dependency_profile.py"))
BASE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BASE)
PROFILE, VERIFY = BASE.PROFILE, BASE.VERIFY
HOST, WINDOWS = BASE.HOST, BASE.WINDOWS
AK = "accesskit_windows@0.34.0"
AK_PATH = "third-party/accesskit_windows"
AK_LICENSE = "MIT AND BSD-3-Clause AND GPL-3.0-only"
EXPECTED_MEMBERS = ("nevrotick-ui", "ntp-wire", "time-domain", "ntp-exchange", "time-core", "nevrotick-engine", "windows-platform")


def accesskit_record():
    """Independent identity/grant pins; fake derivative hashes are inert shape controls."""
    return {"name": "accesskit_windows", "version": "0.34.0", "path": AK_PATH,
            "license": AK_LICENSE, "selected_license": AK_LICENSE,
            "build_script": False, "proc_macro": False, "links": None,
            "upstream_checksum": "106c2b961215864d1c2e703ee63269c25c4e80a577ffb2c1017b9c17dcdf83a1",
            "upstream_files_sha256": "7b5aa35d98dbf1c4ba0d311f591de9486000b84f97190462de4266542d0306e7",
            "upstream_revision": "c88605b96d04431f9c3c792464a0f2f253480e94",
            "files_sha256": "4" * 64,
            "modified_files_sha256": {name: "5" * 64 for name in
                ("Cargo.toml", "NEVROTICK-PATCH.md", "src/lib.rs", "src/node.rs", "src/range_state.rs", "src/subclass.rs", "src/text.rs", "src/value_input.rs", "tests/subclass_lifecycle_harness.rs")}}


def two_patch_policy():
    policy = BASE.add_patch(BASE.fixture_policy())
    policy["patched_packages"].append(accesskit_record())
    row = policy["rows"][WINDOWS]
    row["features"][AK] = {"target": [[]]}
    row["metadata_features"][AK] = []
    row["package_scopes"]["nevrotick-ui"]["features"][AK] = {"target": [[]]}
    return policy


def accesskit_tree(output):
    line = "|-- slint v1.18.1|" + ",".join(BASE.RUNTIME_FEATURES) + "\n"
    assert line in output
    return BASE.patch_tree(output).replace(line, line + "|   `-- accesskit_windows v0.34.0 (/work/fixture/third-party/accesskit_windows)|\n")


class AccessKitPathPatchControls(unittest.TestCase):
    def setUp(self):
        self.policy = two_patch_policy()
        pin = patch.object(PROFILE, "REVIEWED_POLICY_SHA256", PROFILE.canonical_policy_sha256(self.policy))
        pin.start()
        self.addCleanup(pin.stop)

    def test_second_patch_has_only_windows_gui_target_and_no_standalone_scope(self):
        PROFILE.validate_policy(self.policy)
        self.assertEqual(PROFILE.package_scopes(self.policy), (*BASE.MEMBERS, "spin_on"))
        self.assertNotIn(AK, self.policy["rows"][HOST]["features"])
        self.assertNotIn(AK, self.policy["rows"][HOST]["metadata_features"])
        self.assertNotIn("accesskit_windows", self.policy["rows"][WINDOWS]["package_scopes"])
        self.assertNotIn("checksum", self.policy["patched_packages"][1])

    def test_two_patch_cohort_is_closed_and_ordered(self):
        for mutate in (
            lambda p: p["patched_packages"].reverse(),
            lambda p: p["patched_packages"].pop(0),
            lambda p: p["patched_packages"].append(copy.deepcopy(p["patched_packages"][1])),
            lambda p: p["patched_packages"][1].update(name="another"),
            lambda p: p["patched_packages"][1].update(version="0.35.0"),
            lambda p: p["patched_packages"][1].update(path="crates/accesskit_windows"),
            lambda p: p["patched_packages"][1].update(upstream_checksum="0" * 64),
            lambda p: p["patched_packages"][1].update(selected_license="MIT"),
            lambda p: p["patched_packages"][1].update(license="GPL-3.0-only"),
            lambda p: p["patched_packages"][1].update(build_script=True),
            lambda p: p["patched_packages"][1].update(checksum="0" * 64),
            lambda p: p["patched_packages"][1]["modified_files_sha256"].pop("src/value_input.rs"),
            lambda p: p["patched_packages"][1]["modified_files_sha256"].pop("src/range_state.rs"),
            lambda p: p["patched_packages"][1]["modified_files_sha256"].pop("src/subclass.rs"),
            lambda p: p["patched_packages"][1]["modified_files_sha256"].pop("tests/subclass_lifecycle_harness.rs"),
        ):
            changed = copy.deepcopy(self.policy)
            mutate(changed)
            with self.subTest(changed=changed["patched_packages"]), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_policy_shape(changed)

    def test_no_linux_host_dev_feature_or_pure_owner_promotion(self):
        for mutate in (
            lambda p: p["rows"][HOST]["metadata_features"].update({AK: []}),
            lambda p: p["rows"][HOST]["features"].update({AK: {"target": [[]]}}),
            lambda p: p["rows"][WINDOWS]["features"][AK].update(host=[[]]),
            lambda p: p["rows"][WINDOWS]["features"][AK].update(target=[["dev"]]),
            lambda p: p["rows"][WINDOWS]["metadata_features"].update({AK: ["dev"]}),
            lambda p: p["rows"][WINDOWS]["package_scopes"]["nevrotick-ui"]["features"].pop(AK),
            lambda p: p["rows"][WINDOWS]["package_scopes"]["time-core"]["features"].update({AK: {"target": [[]]}}),
            lambda p: p["rows"][WINDOWS]["package_scopes"].update({"accesskit_windows": {"features": {AK: {"target": [[]]}}, "local_features": {}}}),
        ):
            changed = copy.deepcopy(self.policy)
            mutate(changed)
            with self.subTest(mutate=mutate), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_policy_shape(changed)
        PROFILE.validate_policy(self.policy)

    def test_registry_role_cannot_coexist_with_same_local_identity(self):
        record = {"name": "accesskit_windows", "version": "0.34.0", "checksum": "a" * 64,
                  "license": "MIT OR Apache-2.0", "files_sha256": "b" * 64, "role": "metadata-only"}
        changed = copy.deepcopy(self.policy)
        changed["metadata_only_packages"].insert(0, record)
        with self.assertRaises(PROFILE.Incomplete):
            PROFILE.validate_policy_shape(changed)
        changed = copy.deepcopy(self.policy)
        changed["packages"][0]["selected_license"] = "GPL-3.0-only"
        with self.assertRaises(PROFILE.Incomplete):
            PROFILE.validate_policy_shape(changed)

    def test_metadata_requires_local_exact_path_license_and_execution_role(self):
        for target in (HOST, WINDOWS):
            data = BASE.fixture_metadata(self.policy, target)
            PROFILE.validate_metadata(target, json.dumps(data), self.policy)
        data = BASE.fixture_metadata(self.policy, WINDOWS)
        for mutation in (
            {"source": PROFILE.REGISTRY}, {"manifest_path": "/work/fixture/crates/accesskit_windows/Cargo.toml"},
            {"license": "MIT OR Apache-2.0"}, {"targets": [{"kind": ["custom-build"]}]},
            {"targets": [{"kind": ["proc-macro"]}]}, {"links": "native-extra"},
        ):
            changed = copy.deepcopy(data)
            next(p for p in changed["packages"] if p["name"] == "accesskit_windows").update(mutation)
            with self.subTest(mutation=mutation), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_metadata(WINDOWS, json.dumps(changed), self.policy)
        PROFILE.validate_metadata(WINDOWS, json.dumps(data), self.policy)

    def test_tree_requires_real_fork_path_and_normal_target_edge(self):
        tree = accesskit_tree(BASE.fixture_tree(WINDOWS))
        PROFILE.validate_tree(WINDOWS, tree, self.policy)
        for changed in (
            tree.replace("/work/fixture/third-party/accesskit_windows", "/work/fixture/crates/accesskit_windows"),
            tree.replace(" (/work/fixture/third-party/accesskit_windows)", ""),
            tree.replace("|   `-- accesskit_windows", "|   [dev-dependencies]\n|   `-- accesskit_windows"),
            tree.replace("|   `-- accesskit_windows", "|   [build-dependencies]\n|   `-- accesskit_windows"),
        ):
            with self.subTest(changed=changed), self.assertRaises(PROFILE.Incomplete):
                PROFILE.validate_tree(WINDOWS, changed, self.policy)
        with self.assertRaises(PROFILE.Incomplete):
            PROFILE.validate_tree(WINDOWS, tree, self.policy, scope="accesskit_windows")
        PROFILE.validate_tree(WINDOWS, tree, self.policy)

    def test_selected_projection_preserves_second_local_source(self):
        trees = {name: BASE.patch_tree(output) for name, output in BASE.fixture_package_trees(WINDOWS).items()}
        trees["nevrotick-ui"] = accesskit_tree(BASE.fixture_package_trees(WINDOWS)["nevrotick-ui"])
        trees["spin_on"] = "spin_on v0.1.1 (/work/fixture/third-party/spin_on)|\n"
        projected = PROFILE.selected_metadata(WINDOWS, json.dumps(BASE.fixture_metadata(self.policy, WINDOWS)),
                    accesskit_tree(BASE.fixture_tree(WINDOWS)), self.policy, package_trees=trees)
        package = next(p for p in projected["packages"] if p["name"] == "accesskit_windows")
        self.assertIsNone(package["source"])
        self.assertEqual(package["manifest_path"], "/work/fixture/third-party/accesskit_windows/Cargo.toml")
        self.assertEqual(package["license"], AK_LICENSE)
        self.assertEqual(projected["workspace_members"], [
            "path+file:///work/fixture/crates/" + name + "#0.0.0" for name in EXPECTED_MEMBERS
        ])

    def test_commands_are_identical_to_spin_only_and_do_not_compile_accesskit_on_linux(self):
        targets = {"portable": (HOST,), "windows": (WINDOWS,), "dependencies": (HOST, WINDOWS)}
        for lane in ("portable", "windows", "dependencies"):
            single = VERIFY.commands(lane, schema_version=3, patched_packages=("spin_on",))
            double = VERIFY.commands(lane, schema_version=3, patched_packages=("spin_on", "accesskit_windows"))
            self.assertEqual(single, double)
            self.assertEqual([label for label, _ in double if label.startswith("dependency-tree-")], [
                "dependency-tree-" + (scope + "-" if scope else "") + target
                for target in targets[lane] for scope in ("", *EXPECTED_MEMBERS, "spin_on")
            ])
            self.assertFalse(any("accesskit_windows" in label or AK_PATH in " ".join(command) for label, command in double))

    def test_test_accounting_does_not_create_accesskit_package_evidence(self):
        records = []
        for package in (*BASE.MEMBERS, "spin_on"):
            for kind in ("product-tests", "doctests", *(("optimized-tests",) if package in VERIFY.OPTIMIZED_DISCRIMINATORS else ())):
                records.append({"lane": kind + ":" + package, "tests": {"package": package, "executed": 1}})
        result = VERIFY.summarize_test_counts(records, patched_packages=("spin_on", "accesskit_windows"))
        self.assertEqual(set(result["by_package"]), {*BASE.MEMBERS, "spin_on"})
        records.append({"lane": "product-tests:accesskit_windows", "tests": {"package": "accesskit_windows", "executed": 8}})
        with self.assertRaises(VERIFY.ENV.Incomplete):
            VERIFY.summarize_test_counts(records, patched_packages=("spin_on", "accesskit_windows"))

    def test_second_local_lock_has_only_exact_existing_normal_dependencies(self):
        locks = [{"name": "spin_on", "version": "0.1.1"}, {"name": "accesskit_windows", "version": "0.34.0",
                 "dependencies": ["accesskit", "accesskit_consumer", "hashbrown 0.16.1", "static_assertions", "windows", "windows-core"]}]
        VERIFY.validate_patched_lock(locks, self.policy["patched_packages"])
        for mutation in (
            {"source": PROFILE.REGISTRY}, {"checksum": "0" * 64}, {"version": "0.35.0"},
            {"dependencies": []}, {"dependencies": locks[1]["dependencies"] + ["winit"]},
        ):
            changed = copy.deepcopy(locks)
            changed[1].update(mutation)
            with self.subTest(mutation=mutation), self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.validate_patched_lock(changed, self.policy["patched_packages"])
        for changed in (locks[:1], locks + [locks[1]]):
            with self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.validate_patched_lock(changed, self.policy["patched_packages"])

    def test_second_source_preserves_exact_bytes_notices_and_manifest_edges(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            directory = root / AK_PATH
            shutil.copytree(ROOT / AK_PATH, directory)
            record = accesskit_record()

            def refresh():
                files = {p.relative_to(directory).as_posix(): VERIFY.sha256(p) for p in directory.rglob("*") if p.is_file()}
                record["files_sha256"] = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
                record["modified_files_sha256"] = {name: files[name] for name in record["modified_files_sha256"]}

            refresh()
            self.assertEqual(len(VERIFY.admit_patched_source(root, record)), 28)
            for name in ("LICENSE-MIT", "LICENSE-APACHE", "LICENSE.chromium", "LICENSE-GPL-3.0-only", "Cargo.toml.upstream", "Cargo.lock", "src/adapter.rs"):
                original = (directory / name).read_bytes()
                (directory / name).write_bytes(original + b"\nchanged\n")
                refresh()
                with self.subTest(name=name), self.assertRaises(VERIFY.ENV.Incomplete):
                    VERIFY.admit_patched_source(root, record)
                (directory / name).write_bytes(original)
            manifest = directory / "Cargo.toml"
            healthy = manifest.read_text()
            manifest.write_text(healthy.replace('version = "0.24.1"', 'version = "0.24.2"'))
            refresh()
            with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "manifest may only change"):
                VERIFY.admit_patched_source(root, record)
            manifest.write_text(healthy)
            refresh()
            VERIFY.admit_patched_source(root, record)
            for changed_features in (
                healthy.replace('    "Win32_System_Threading",\n', ''),
                healthy.replace('    "Win32_System_Threading",\n', '    "Win32_System_Threading",\n    "Win32_System_Registry",\n'),
            ):
                manifest.write_text(changed_features)
                refresh()
                with self.assertRaisesRegex(VERIFY.ENV.Incomplete, "reviewed owner-thread feature"):
                    VERIFY.admit_patched_source(root, record)
            manifest.write_text(healthy)
            refresh()
            VERIFY.admit_patched_source(root, record)
            (directory / "src/value_input.rs").write_text("// unreviewed replacement\n")
            with self.assertRaises(VERIFY.ENV.Incomplete):
                VERIFY.admit_patched_source(root, record)

    def test_composite_grants_stay_per_exact_windows_crate(self):
        base = (ROOT / "deny.toml").read_text()
        for target in (HOST, WINDOWS):
            denied = tomllib.loads(VERIFY.product_deny(base, self.policy, target))
            self.assertEqual(denied["licenses"]["allow"], ["GPL-2.0-only"])
            ak = [item for item in denied["licenses"]["exceptions"] if item["crate"] == AK]
            self.assertEqual(ak, [{"crate": AK, "allow": ["MIT", "BSD-3-Clause", "GPL-3.0-only"]}] if target == WINDOWS else [])
        changed = copy.deepcopy(self.policy)
        changed["patched_packages"][1]["modified_files_sha256"]["src/value_input.rs"] = "f" * 64
        with self.assertRaisesRegex(PROFILE.Incomplete, "code-owned pin"):
            PROFILE.validate_policy(changed)


RANGE_RECEIVERS = ("Compare", "CompareEndpoints", "MoveEndpointByRange")
RANGE_CHECKED_FIRST = (
    "let other: &PlatformRange = required_param(&other)?.cast_object_ref::<PlatformRange>()?;"
)


def require_checked_range_receivers(source):
    """Pin the first admission statement at the three known COM range receivers.

    This is a narrow source regression guard, not a Rust parser or behavioral
    proof of COM provenance, pointer validity, marshaling or native execution.
    """
    for name in RANGE_RECEIVERS:
        marker = "    fn " + name + "("
        if source.count(marker) != 1:
            raise ValueError("Range receiver missing or ambiguous")
        start = source.index(marker)
        end = source.find("\n    fn ", start + len(marker))
        receiver = source[start:end if end != -1 else len(source)]
        body = receiver.partition("{")[2].lstrip()
        if (not body.startswith(RANGE_CHECKED_FIRST)
                or receiver.count(".cast_object_ref::<PlatformRange>()?") != 1
                or ".as_impl(" in receiver or ".as_impl_ptr(" in receiver):
            raise ValueError("Range receiver lacks checked first admission")


class RangeProvenanceSourceControls(unittest.TestCase):
    """Executable source controls only; these do not run the Windows adapter."""

    def test_actual_receivers_have_fallible_first_admission(self):
        source = (ROOT / AK_PATH / "src/text.rs").read_text(encoding="utf-8", errors="strict")
        require_checked_range_receivers(source)
        self.assertEqual(source.count(RANGE_CHECKED_FIRST), 3)

    def test_known_unsafe_panic_late_and_missing_regressions_fail(self):
        source = (ROOT / AK_PATH / "src/text.rs").read_text(encoding="utf-8", errors="strict")
        require_checked_range_receivers(source)
        for name in RANGE_RECEIVERS:
            start = source.index("    fn " + name + "(")
            point = source.index(RANGE_CHECKED_FIRST, start)
            for replacement in (
                "let other = unsafe { required_param(&other)?.as_impl() };",
                "let other: &PlatformRange = required_param(&other)?.cast_object_ref::<PlatformRange>().unwrap();",
                "let _premature = self.state.read().unwrap();\n        " + RANGE_CHECKED_FIRST,
                "let other: &PlatformRange = required_param(&other)?.cast_object_ref::<AnotherRange>()?;",
            ):
                changed = source[:point] + replacement + source[point + len(RANGE_CHECKED_FIRST):]
                with self.subTest(receiver=name, replacement=replacement), self.assertRaises(ValueError):
                    require_checked_range_receivers(changed)
            changed = source[:start] + source[start:].replace("    fn " + name + "(", "    fn Removed" + name + "(", 1)
            with self.subTest(receiver=name, replacement="missing"), self.assertRaises(ValueError):
                require_checked_range_receivers(changed)
        original = source.replace(RANGE_CHECKED_FIRST, "let other = unsafe { required_param(&other)?.as_impl() };")
        with self.subTest(replacement="all three original unsafe casts"), self.assertRaises(ValueError):
            require_checked_range_receivers(original)


if __name__ == "__main__":
    unittest.main()
