"""Independent admission/fail-closed controls; no installed analyzer is executed."""

import copy
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("check_analysis", ROOT / "scripts/check-analysis.py")
CHECK = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECK)


class AdmissionControls(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-analysis-tests-")
        self.addCleanup(self.temporary.cleanup)
        self.scratch = Path(self.temporary.name)
        self.checkout = self.scratch / "checkout"
        (self.checkout / "policy").mkdir(parents=True)
        self.base = self.scratch / "installation"
        (self.base / "common/bin").mkdir(parents=True)
        (self.base / "evidence").mkdir()
        self.home = self.scratch / "home"
        self.stable = self.home / f".rustup/toolchains/1.100.0-{CHECK.PLATFORM}"
        self.stable.mkdir(parents=True)
        self.pointer = self.scratch / "installation.json"
        self.pointer.write_text(json.dumps({"schema_version": 1, "root": str(self.base),
                                           "stable_toolchain": str(self.stable), "status": "installed",
                                           "platform": CHECK.PLATFORM}))
        self.pointer.chmod(0o600)
        self.policy = json.loads((ROOT / "policy/analysis-tools.json").read_text())
        self.policy["miri_sysroot"] = "common/miri-sysroot"
        self.policy["fuzz_target"] = CHECK.PLATFORM

    def write_policy(self):
        (self.checkout / "policy/analysis-tools.json").write_text(json.dumps(self.policy))

    def inventory(self, *, entry=None, scopes=None):
        target = self.base / "common/bin/analyzer"
        target.write_bytes(b"independent payload bytes")
        entries = {"bin/analyzer": entry or {"kind": "file", "bytes": target.stat().st_size,
                                            "sha256": hashlib.sha256(target.read_bytes()).hexdigest()}}
        value = {"schema_version": 1, "analysis_installation_root": str(self.base),
                 "tool_root": str(self.base / "common"), "scopes": scopes or ["bin"], "entries": entries}
        spec = {"name": "common", "path": "evidence/common.json", "root": "common", "scopes": scopes or ["bin"]}
        path = self.base / spec["path"]
        path.write_text(json.dumps(value))
        spec["sha256"] = hashlib.sha256(path.read_bytes()).hexdigest()
        return spec, value, path

    def check_inventory(self, specification):
        with patch.dict(CHECK.REQUIRED_SCOPES, {"common": {"bin"}}, clear=True):
            return CHECK.check_inventory(self.base, specification)

    def stable_fixture(self):
        """Create only tiny selected payloads; omitted inventories stay absent."""
        scopes = ["bin", "etc", "lib", "libexec", "share"]
        for scope in scopes:
            (self.stable / scope).mkdir(exist_ok=True)
        manifest = json.loads((ROOT / "policy/tool-manifest.json").read_text())
        for name in manifest["rust"]["linux_binary_sha256"]:
            payload = ("independent stable executable " + name).encode()
            target = self.stable / "bin" / name
            target.write_bytes(payload)
            target.chmod(0o500)
            manifest["rust"]["linux_binary_sha256"][name] = hashlib.sha256(payload).hexdigest()
        (self.stable / "lib/dependency.so").write_bytes(b"independent selected library")
        entries = {path.relative_to(self.stable).as_posix(): {
            "kind": "file", "bytes": path.stat().st_size,
            "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}
            for path in self.stable.rglob("*") if path.is_file()}
        inventory = {"schema_version": 1, "analysis_installation_root": str(self.base),
                     "tool_root": str(self.stable), "scopes": scopes, "entries": entries}
        specification = {"name": "stable", "path": "evidence/stable.json",
                         "root": "stable", "scopes": scopes}
        inventory_path = self.base / specification["path"]
        inventory_path.write_text(json.dumps(inventory))
        specification["sha256"] = hashlib.sha256(inventory_path.read_bytes()).hexdigest()
        self.policy["inventories"] = [specification if item["root"] == "stable" else item
                                      for item in self.policy["inventories"]]
        self.write_policy()
        (self.checkout / "policy/tool-manifest.json").write_text(json.dumps(manifest))
        return specification, manifest

    def admit_stable(self):
        """Only the external bootstrap helper observation is replaced."""
        with patch.object(Path, "home", return_value=self.home), \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}), \
             patch.object(CHECK.RUNTIME, "run") as run:
            try:
                return CHECK.admit(self.checkout, self.pointer, profile="stable-core")
            finally:
                run.assert_not_called()

    def kani_fixture(self):
        """Bind a complete tiny Kani/compiler closure, including its real link shape.

        Only fixture pins replace production pins; payload reads/hashes and all
        inventory/pointer checks remain real. Unselected inventories stay absent.
        """
        stable_specification, manifest = self.stable_fixture()
        (self.base / stable_specification["path"]).unlink()
        scopes = ["payload/kani-0.68.0",
                  "rustup/toolchains/nightly-2026-08-21-x86_64-unknown-linux-gnu"]
        kani = self.base / "kani"
        payload, compiler = (kani / scope for scope in scopes)
        files = {
            payload / "bin/kani-driver": b"independent fixture Kani driver",
            payload / "bin/kani-compiler": b"independent fixture Kani compiler",
            payload / "bin/cbmc": b"independent fixture CBMC and solver",
            payload / "lib/kani.rlib": b"independent fixture Kani verification library",
            compiler / "bin/rustc": b"independent fixture private rustc",
            compiler / "bin/rustdoc": b"independent fixture private rustdoc",
            compiler / "lib/librustc_driver.so": b"independent fixture compiler library",
            compiler / "lib/libLLVM.so": b"independent fixture LLVM library",
        }
        for path, contents in files.items():
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(contents)
            if path.parent.name == "bin":
                path.chmod(0o500)
        toolchain = payload / "toolchain"
        link = "../../" + scopes[1]
        toolchain.symlink_to(link, target_is_directory=True)
        entries = {path.relative_to(kani).as_posix(): {
            "kind": "file", "bytes": len(contents),
            "sha256": hashlib.sha256(contents).hexdigest()}
            for path, contents in files.items()}
        entries[toolchain.relative_to(kani).as_posix()] = {"kind": "symlink", "target": link}
        inventory = {"schema_version": 1, "analysis_installation_root": str(self.base),
                     "tool_root": str(kani), "scopes": scopes, "entries": entries}
        specification = {"name": "kani", "path": "evidence/kani-fixture.json",
                         "root": "kani", "scopes": scopes}
        inventory_path = self.base / specification["path"]
        inventory_path.write_text(json.dumps(inventory))
        specification["sha256"] = hashlib.sha256(inventory_path.read_bytes()).hexdigest()
        self.policy["inventories"] = [specification if item["root"] == "kani" else item
                                      for item in self.policy["inventories"]]
        self.kani_pin = ("0.68.0", "kani/payload/kani-0.68.0/bin/kani-driver",
                         hashlib.sha256(files[payload / "bin/kani-driver"]).hexdigest())
        self.policy["tools"]["kani"]["sha256"] = self.kani_pin[2]
        self.write_policy()
        return specification, manifest, files, payload, compiler

    def admit_kani(self):
        """Run real miniature admission; replace only fixture pin and host helpers."""
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}), \
             patch.object(CHECK.RUNTIME, "run") as run:
            try:
                return CHECK.admit_kani_proof(self.checkout, self.pointer)
            finally:
                run.assert_not_called()

    def test_kani_proof_admits_complete_selected_closure_and_exact_readonly_roots(self):
        specification, _, _, payload, compiler = self.kani_fixture()
        real_check = CHECK.check_inventory
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK, "check_inventory", wraps=real_check) as inventories, \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}) as helpers, \
             patch.object(CHECK.RUNTIME, "run") as run:
            admitted = CHECK.admit_kani_proof(self.checkout, self.pointer)
        inventories.assert_called_once_with(self.base, specification, self.stable)
        helpers.assert_called_once_with(self.checkout)
        run.assert_not_called()
        self.assertEqual(admitted["admission_profile"], "kani-proof")
        self.assertEqual(admitted["readonly_roots"], (payload, compiler))
        self.assertEqual([item["root"] for item in admitted["inventories"]], ["kani"])
        self.assertEqual(admitted["installation_pointer"], str(self.pointer))
        self.assertEqual(admitted["installation_pointer_sha256"],
                         hashlib.sha256(self.pointer.read_bytes()).hexdigest())
        for item in self.policy["inventories"]:
            if item["root"] != "kani":
                self.assertFalse((self.base / item["path"]).exists())

    def test_kani_proof_role_cannot_enter_generic_admission_or_cli_profile_choices(self):
        self.kani_fixture()
        self.assertEqual(CHECK.ADMISSION_PROFILES, ("full", "stable-core"))
        with patch.object(CHECK, "check_inventory") as inventories, \
             self.assertRaisesRegex(CHECK.Incomplete, "Unknown analysis admission profile"):
            CHECK.admit(self.checkout, self.pointer, profile="kani-proof")
        inventories.assert_not_called()

    def test_era_codegen_has_distinct_consumer_and_same_complete_kani_closure(self):
        specification, _, _, payload, compiler = self.kani_fixture()
        real_check = CHECK.check_inventory
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK, "check_inventory", wraps=real_check) as inventories, \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}) as helpers, \
             patch.object(CHECK.RUNTIME, "run") as run:
            admitted = CHECK.admit_era_constructor_codegen(self.checkout, self.pointer)
        inventories.assert_called_once_with(self.base, specification, self.stable)
        helpers.assert_called_once_with(self.checkout)
        run.assert_not_called()
        self.assertEqual(admitted["admission_consumer"], "era-constructor-codegen-v1")
        self.assertEqual(admitted["admission_profile"], "kani-proof")
        self.assertEqual(admitted["readonly_roots"], (payload, compiler))
        self.assertNotIn("admission_consumer", self.admit_kani())

    def test_era_codegen_tampering_is_rejected_and_cannot_select_generic_lane(self):
        _, _, files, payload, _ = self.kani_fixture()
        self.assertEqual(CHECK.ADMISSION_PROFILES, ("full", "stable-core"))
        with patch.object(CHECK, "_admit") as generic:
            with self.assertRaisesRegex(CHECK.Incomplete, "Unknown analysis admission profile"):
                CHECK.admit(self.checkout, self.pointer, profile="era-constructor-codegen-v1")
            generic.assert_not_called()
        target = payload / "bin/kani-driver"
        target.chmod(0o600)
        target.write_bytes(b"substituted codegen executable")
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}), \
             patch.object(CHECK.RUNTIME, "run") as run:
            with self.assertRaisesRegex(CHECK.Incomplete, "digest/size mismatch"):
                CHECK.admit_era_constructor_codegen(self.checkout, self.pointer)
        run.assert_not_called()
        target.write_bytes(files[target])
        target.chmod(0o500)
        self.admit_kani()

    def test_era_model_inspection_retains_complete_inventory_and_distinct_consumer(self):
        specification, _, _, payload, compiler = self.kani_fixture()
        real_check = CHECK.check_inventory
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK, "check_inventory", wraps=real_check) as inventories, \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}) as helpers, \
             patch.object(CHECK.RUNTIME, "run") as run:
            admitted = CHECK.admit_era_constructor_model_inspection(self.checkout, self.pointer)
        inventories.assert_called_once_with(self.base, specification, self.stable)
        helpers.assert_called_once_with(self.checkout)
        run.assert_not_called()
        self.assertEqual(admitted["admission_profile"], "kani-proof")
        self.assertEqual(admitted["admission_consumer"], "era-constructor-model-inspection-v1")
        self.assertEqual(admitted["readonly_roots"], (payload, compiler))

    def test_era_model_inspection_rejects_tamper_and_generic_selection(self):
        _, _, _, _, compiler = self.kani_fixture()
        with patch.object(CHECK, "_admit") as generic:
            with self.assertRaisesRegex(CHECK.Incomplete, "Unknown analysis admission profile"):
                CHECK.admit(self.checkout, self.pointer, profile="era-constructor-model-inspection-v1")
            generic.assert_not_called()
        library = compiler / "lib/librustc_driver.so"
        library.chmod(0o600)
        library.write_bytes(b"substituted transitive compiler library")
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}), \
             patch.object(CHECK.RUNTIME, "run") as run:
            with self.assertRaisesRegex(CHECK.Incomplete, "digest/size mismatch"):
                CHECK.admit_era_constructor_model_inspection(self.checkout, self.pointer)
        run.assert_not_called()

    def test_era_constructor_proof_has_fixed_role_and_complete_kani_roots(self):
        specification, _, _, payload, compiler = self.kani_fixture()
        real_check = CHECK.check_inventory
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK, "check_inventory", wraps=real_check) as inventories, \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}) as helpers, \
             patch.object(CHECK.RUNTIME, "run") as run:
            admitted = CHECK.admit_era_constructor_proof(self.checkout, self.pointer)
        inventories.assert_called_once_with(self.base, specification, self.stable)
        helpers.assert_called_once_with(self.checkout)
        run.assert_not_called()
        self.assertEqual(admitted["admission_consumer"], "era-constructor-proof-v1")
        self.assertEqual(admitted["admission_profile"], "kani-proof")
        self.assertEqual(admitted["readonly_roots"], (payload, compiler))

    def test_era_constructor_proof_cannot_bypass_helpers_or_generic_role_rejection(self):
        self.kani_fixture()
        with patch.object(CHECK, "_admit") as generic:
            with self.assertRaisesRegex(CHECK.Incomplete, "Unknown analysis admission profile"):
                CHECK.admit(self.checkout, self.pointer, profile="era-constructor-proof-v1")
            generic.assert_not_called()
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK.RUNTIME, "_helpers", side_effect=CHECK.Incomplete("proof helper denial")), \
             patch.object(CHECK.RUNTIME, "run") as run:
            with self.assertRaisesRegex(CHECK.Incomplete, "proof helper denial"):
                CHECK.admit_era_constructor_proof(self.checkout, self.pointer)
        run.assert_not_called()

    def test_fraction_query_split_has_distinct_consumer_and_original_complete_kani_closure(self):
        specification, _, _, payload, compiler = self.kani_fixture()
        original_profiles = copy.deepcopy(CHECK.PROFILES)
        real_check = CHECK.check_inventory
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK, "check_inventory", wraps=real_check) as inventories, \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}) as helpers, \
             patch.object(CHECK.RUNTIME, "run") as run:
            admitted = CHECK.admit_fraction_query_split_pilot(self.checkout, self.pointer)
        inventories.assert_called_once_with(self.base, specification, self.stable)
        helpers.assert_called_once_with(self.checkout)
        run.assert_not_called()
        self.assertEqual(admitted["admission_consumer"], "fraction-query-split-pilot-v1")
        self.assertEqual(admitted["admission_profile"], "kani-proof")
        self.assertEqual(admitted["readonly_roots"], (payload, compiler))
        self.assertEqual([item["root"] for item in admitted["inventories"]], ["kani"])
        self.assertEqual(admitted["policy"]["profiles"], original_profiles)
        self.assertEqual(CHECK.PROFILES, original_profiles)
        self.assertEqual(CHECK.ADMISSION_PROFILES, ("full", "stable-core"))
        self.assertNotIn("admission_consumer", self.admit_kani())

    def test_fraction_query_split_admission_is_closed_and_propagates_failure(self):
        with patch.object(CHECK, "_admit") as generic:
            with self.assertRaisesRegex(CHECK.Incomplete, "Unknown analysis admission profile"):
                CHECK.admit(self.checkout, self.pointer, profile="fraction-query-split-pilot-v1")
            for keyword, value in (("profile", "full"), ("solver", "z3"), ("timeout", 300),
                                   ("lane", "universal")):
                with self.subTest(keyword=keyword), self.assertRaises(TypeError):
                    CHECK.admit_fraction_query_split_pilot(self.checkout, self.pointer, **{keyword: value})
            generic.assert_not_called()
        with patch.object(CHECK, "_admit", side_effect=CHECK.Incomplete("query split admission denied")) as generic:
            with self.assertRaisesRegex(CHECK.Incomplete, "query split admission denied"):
                CHECK.admit_fraction_query_split_pilot(self.checkout, self.pointer)
        generic.assert_called_once_with(self.checkout, self.pointer, profile=CHECK.KANI_PROOF_PROFILE)

    def test_fraction_composition_has_distinct_consumer_and_original_complete_kani_closure(self):
        specification, _, _, payload, compiler = self.kani_fixture()
        original_profiles = copy.deepcopy(CHECK.PROFILES)
        real_check = CHECK.check_inventory
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK, "check_inventory", wraps=real_check) as inventories, \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}) as helpers, \
             patch.object(CHECK.RUNTIME, "run") as run:
            admitted = CHECK.admit_fraction_composition_pilot(self.checkout, self.pointer)
        inventories.assert_called_once_with(self.base, specification, self.stable)
        helpers.assert_called_once_with(self.checkout)
        run.assert_not_called()
        self.assertEqual(admitted["admission_consumer"], "fraction-composition-pilot-v1")
        self.assertEqual(admitted["admission_profile"], "kani-proof")
        self.assertEqual(admitted["readonly_roots"], (payload, compiler))
        self.assertEqual([item["root"] for item in admitted["inventories"]], ["kani"])
        self.assertEqual(admitted["policy"]["profiles"], original_profiles)
        self.assertEqual(CHECK.PROFILES, original_profiles)
        self.assertEqual(CHECK.ADMISSION_PROFILES, ("full", "stable-core"))
        self.assertNotIn("admission_consumer", self.admit_kani())

    def test_fraction_composition_admission_is_closed_and_propagates_failure(self):
        with patch.object(CHECK, "_admit") as generic:
            with self.assertRaisesRegex(CHECK.Incomplete, "Unknown analysis admission profile"):
                CHECK.admit(self.checkout, self.pointer, profile="fraction-composition-pilot-v1")
            for keyword, value in (("profile", "full"), ("solver", "z3"), ("timeout", 300)):
                with self.subTest(keyword=keyword), self.assertRaises(TypeError):
                    CHECK.admit_fraction_composition_pilot(self.checkout, self.pointer, **{keyword: value})
            generic.assert_not_called()
        with patch.object(CHECK, "_admit", side_effect=CHECK.Incomplete("composition admission denied")) as generic:
            with self.assertRaisesRegex(CHECK.Incomplete, "composition admission denied"):
                CHECK.admit_fraction_composition_pilot(self.checkout, self.pointer)
        generic.assert_called_once_with(self.checkout, self.pointer, profile=CHECK.KANI_PROOF_PROFILE)

    def test_fraction_addition_compute_has_distinct_consumer_and_unchanged_complete_kani_closure(self):
        specification, _, _, payload, compiler = self.kani_fixture()
        original_profiles = copy.deepcopy(CHECK.PROFILES)
        real_check = CHECK.check_inventory
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK, "check_inventory", wraps=real_check) as inventories, \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}) as helpers, \
             patch.object(CHECK.RUNTIME, "run") as run:
            admitted = CHECK.admit_fraction_addition_compute_pilot(self.checkout, self.pointer)
        inventories.assert_called_once_with(self.base, specification, self.stable)
        helpers.assert_called_once_with(self.checkout)
        run.assert_not_called()
        self.assertEqual(admitted["admission_consumer"], "fraction-addition-compute-pilot-v1")
        self.assertEqual(admitted["admission_profile"], "kani-proof")
        self.assertEqual(admitted["readonly_roots"], (payload, compiler))
        self.assertEqual([item["root"] for item in admitted["inventories"]], ["kani"])
        self.assertEqual(admitted["installation_pointer"], str(self.pointer))
        self.assertEqual(admitted["installation_pointer_sha256"],
                         hashlib.sha256(self.pointer.read_bytes()).hexdigest())
        self.assertEqual(admitted["policy"]["profiles"], original_profiles)
        self.assertEqual(CHECK.PROFILES, original_profiles)
        self.assertEqual(CHECK.ADMISSION_PROFILES, ("full", "stable-core"))
        self.assertNotIn("admission_consumer", self.admit_kani())
        for item in self.policy["inventories"]:
            if item["root"] != "kani":
                self.assertFalse((self.base / item["path"]).exists())

    def test_fraction_addition_compute_rejects_same_size_library_tampering_with_healthy_neighbors(self):
        _, _, files, payload, compiler = self.kani_fixture()
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}), \
             patch.object(CHECK.RUNTIME, "run") as run:
            self.assertEqual(CHECK.admit_fraction_addition_compute_pilot(self.checkout, self.pointer)
                             ["readonly_roots"], (payload, compiler))
            for target in (payload / "lib/kani.rlib", compiler / "lib/librustc_driver.so"):
                before = target.stat()
                target.write_bytes(b"X" * len(files[target]))
                os.utime(target, ns=(before.st_atime_ns, before.st_mtime_ns))
                self.assertEqual(target.stat().st_size, before.st_size)
                self.assertEqual(target.stat().st_mtime_ns, before.st_mtime_ns)
                with self.subTest(target=target), self.assertRaisesRegex(CHECK.Incomplete, "digest/size mismatch"):
                    CHECK.admit_fraction_addition_compute_pilot(self.checkout, self.pointer)
                target.write_bytes(files[target])
                self.assertEqual(CHECK.admit_fraction_addition_compute_pilot(self.checkout, self.pointer)
                                 ["admission_consumer"], "fraction-addition-compute-pilot-v1")
        run.assert_not_called()

    def test_fraction_addition_compute_rejects_generic_overrides_and_helper_failure(self):
        self.kani_fixture()
        with patch.object(CHECK, "_admit") as generic:
            with self.assertRaisesRegex(CHECK.Incomplete, "Unknown analysis admission profile"):
                CHECK.admit(self.checkout, self.pointer, profile="fraction-addition-compute-pilot-v1")
            for keyword, value in (("profile", "kani-proof"), ("solver", "z3"), ("timeout", 300)):
                with self.subTest(keyword=keyword), self.assertRaises(TypeError):
                    CHECK.admit_fraction_addition_compute_pilot(self.checkout, self.pointer, **{keyword: value})
            generic.assert_not_called()
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK.RUNTIME, "_helpers",
                          side_effect=CHECK.Incomplete("fraction addition compute helper denial")) as helpers, \
             patch.object(CHECK.RUNTIME, "run") as run:
            with self.assertRaisesRegex(CHECK.Incomplete, "fraction addition compute helper denial"):
                CHECK.admit_fraction_addition_compute_pilot(self.checkout, self.pointer)
        helpers.assert_called_once_with(self.checkout)
        run.assert_not_called()

    def test_fraction_contracts_pilot_has_distinct_consumer_and_complete_unchanged_kani_closure(self):
        specification, _, _, payload, compiler = self.kani_fixture()
        original_profiles = copy.deepcopy(CHECK.PROFILES)
        real_check = CHECK.check_inventory
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK, "check_inventory", wraps=real_check) as inventories, \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}) as helpers, \
             patch.object(CHECK.RUNTIME, "run") as run:
            admitted = CHECK.admit_fraction_contracts_pilot(self.checkout, self.pointer)
        inventories.assert_called_once_with(self.base, specification, self.stable)
        helpers.assert_called_once_with(self.checkout)
        run.assert_not_called()
        self.assertEqual(admitted["admission_consumer"], "fraction-contracts-pilot-v1")
        self.assertEqual(admitted["admission_profile"], "kani-proof")
        self.assertEqual(admitted["readonly_roots"], (payload, compiler))
        self.assertEqual([item["root"] for item in admitted["inventories"]], ["kani"])
        self.assertEqual(admitted["installation_pointer"], str(self.pointer))
        self.assertEqual(admitted["installation_pointer_sha256"],
                         hashlib.sha256(self.pointer.read_bytes()).hexdigest())
        self.assertEqual(admitted["policy"]["profiles"], original_profiles)
        self.assertEqual(CHECK.PROFILES, original_profiles)
        self.assertEqual(CHECK.ADMISSION_PROFILES, ("full", "stable-core"))
        self.assertNotIn("admission_consumer", self.admit_kani())
        for item in self.policy["inventories"]:
            if item["root"] != "kani":
                self.assertFalse((self.base / item["path"]).exists())

    def test_fraction_contracts_pilot_rejects_same_size_library_tampering_with_healthy_neighbors(self):
        _, _, files, payload, compiler = self.kani_fixture()
        targets = (payload / "lib/kani.rlib", compiler / "lib/librustc_driver.so")
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}), \
             patch.object(CHECK.RUNTIME, "run") as run:
            self.assertEqual(CHECK.admit_fraction_contracts_pilot(self.checkout, self.pointer)
                             ["readonly_roots"], (payload, compiler))
            for target in targets:
                before = target.stat()
                target.write_bytes(b"X" * len(files[target]))
                os.utime(target, ns=(before.st_atime_ns, before.st_mtime_ns))
                self.assertEqual(target.stat().st_size, before.st_size)
                self.assertEqual(target.stat().st_mtime_ns, before.st_mtime_ns)
                with self.subTest(target=target), self.assertRaisesRegex(CHECK.Incomplete, "digest/size mismatch"):
                    CHECK.admit_fraction_contracts_pilot(self.checkout, self.pointer)
                target.write_bytes(files[target])
                self.assertEqual(CHECK.admit_fraction_contracts_pilot(self.checkout, self.pointer)
                                 ["admission_consumer"], "fraction-contracts-pilot-v1")
        run.assert_not_called()

    def test_fraction_contracts_pilot_rejects_generic_selection_overrides_and_helper_failure(self):
        self.kani_fixture()
        with patch.object(CHECK, "_admit") as generic:
            with self.assertRaisesRegex(CHECK.Incomplete, "Unknown analysis admission profile"):
                CHECK.admit(self.checkout, self.pointer, profile="fraction-contracts-pilot-v1")
            with self.assertRaises(TypeError):
                CHECK.admit_fraction_contracts_pilot(self.checkout, self.pointer, profile="full")
            generic.assert_not_called()
        with patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             patch.object(CHECK.RUNTIME, "_helpers",
                          side_effect=CHECK.Incomplete("fraction contracts helper denial")) as helpers, \
             patch.object(CHECK.RUNTIME, "run") as run:
            with self.assertRaisesRegex(CHECK.Incomplete, "fraction contracts helper denial"):
                CHECK.admit_fraction_contracts_pilot(self.checkout, self.pointer)
        helpers.assert_called_once_with(self.checkout)
        run.assert_not_called()

    def test_kani_proof_hashes_each_tool_solver_and_transitive_compiler_library(self):
        _, _, files, _, _ = self.kani_fixture()
        self.admit_kani()
        for path, original in files.items():
            before = path.stat()
            path.chmod(0o600)
            path.write_bytes(bytes([original[0] ^ 1]) + original[1:])
            os.utime(path, ns=(before.st_atime_ns, before.st_mtime_ns))
            try:
                with self.subTest(path=path.relative_to(self.base)), \
                     self.assertRaisesRegex(CHECK.Incomplete, "digest/size mismatch"):
                    self.admit_kani()
            finally:
                path.write_bytes(original)
                path.chmod(before.st_mode & 0o777)
            self.admit_kani()

    def test_kani_proof_missing_extra_terminal_directory_and_fifo_are_rejected(self):
        _, _, files, payload, compiler = self.kani_fixture()
        target = compiler / "lib/librustc_driver.so"
        original = files[target]
        extra = payload / "lib/unlisted-library.so"
        self.admit_kani()
        for fault in ("missing", "extra", "terminal-directory", "fifo"):
            target.unlink()
            if fault == "extra":
                target.write_bytes(original)
                extra.write_bytes(b"unlisted payload")
            elif fault == "terminal-directory":
                target.mkdir()
            elif fault == "fifo":
                os.mkfifo(target)
            try:
                with self.subTest(fault=fault), self.assertRaises(CHECK.Incomplete):
                    self.admit_kani()
            finally:
                if target.is_dir():
                    target.rmdir()
                else:
                    target.unlink(missing_ok=True)
                target.write_bytes(original)
                extra.unlink(missing_ok=True)
            self.admit_kani()

    def test_kani_proof_rejects_library_and_toolchain_link_redirection(self):
        _, _, files, payload, compiler = self.kani_fixture()
        library = compiler / "lib/librustc_driver.so"
        peer = self.scratch / "same-content-compiler-library.so"
        peer.write_bytes(files[library])
        library.unlink()
        library.symlink_to(peer)
        with self.assertRaises(CHECK.Incomplete):
            self.admit_kani()
        library.unlink()
        library.write_bytes(files[library])
        self.admit_kani()
        toolchain = payload / "toolchain"
        expected = os.readlink(toolchain)
        toolchain.unlink()
        toolchain.symlink_to(self.scratch, target_is_directory=True)
        with self.assertRaises(CHECK.Incomplete):
            self.admit_kani()
        toolchain.unlink()
        toolchain.symlink_to(expected, target_is_directory=True)
        self.admit_kani()

    def test_kani_proof_scope_alias_with_identical_bytes_is_rejected(self):
        _, _, _, _, compiler = self.kani_fixture()
        self.admit_kani()
        outside = self.scratch / "same-content-private-toolchain"
        compiler.rename(outside)
        compiler.symlink_to(outside, target_is_directory=True)
        with self.assertRaises(CHECK.Incomplete):
            self.admit_kani()
        compiler.unlink()
        outside.rename(compiler)
        self.admit_kani()

    def test_kani_proof_keeps_all_inventory_descriptions_and_exact_selected_scopes(self):
        self.kani_fixture()
        healthy = copy.deepcopy(self.policy)
        mutations = [lambda p: p["inventories"].pop(),
                     lambda p: p["inventories"].append(copy.deepcopy(p["inventories"][0])),
                     lambda p: next(i for i in p["inventories"] if i["root"] == "kani").update(name="renamed"),
                     lambda p: next(i for i in p["inventories"] if i["root"] == "kani").update(scopes=["payload/kani-0.68.0"]),
                     lambda p: next(i for i in p["inventories"] if i["root"] == "kani")["scopes"].append("target"),
                     lambda p: next(i for i in p["inventories"] if i["root"] == "lockbud").update(sha256="invalid"),
                     lambda p: next(i for i in p["inventories"] if i["root"] == "common").update(path="../outside"),
                     lambda p: p["tools"]["kani"].update(sha256="0" * 64),
                     lambda p: p["tools"]["kani"].update(toolchain="unchecked-compiler"),
                     lambda p: p["profiles"]["kani_flags"].pop()]
        self.admit_kani()
        for mutate in mutations:
            self.policy = copy.deepcopy(healthy)
            mutate(self.policy)
            self.write_policy()
            with self.subTest(mutate=mutate), self.assertRaises(CHECK.Incomplete):
                self.admit_kani()
        self.policy = healthy
        self.write_policy()
        self.admit_kani()

    def test_kani_proof_keeps_inventory_digest_before_parsing_and_entrypoint_pin(self):
        specification, _, files, payload, _ = self.kani_fixture()
        inventory_path = self.base / specification["path"]
        original_inventory = inventory_path.read_bytes()
        inventory_path.write_bytes(b"not-json")
        with self.assertRaisesRegex(CHECK.Incomplete, "Inventory SHA256 mismatch"):
            self.admit_kani()
        inventory_path.write_bytes(original_inventory)
        self.admit_kani()

        # Even a coherently rebound payload inventory cannot change the separate
        # fixed Kani entrypoint pin without changing the admitted source policy.
        target = payload / "bin/kani-driver"
        target.chmod(0o700)
        target.write_bytes(b"substituted coherent inventory entrypoint")
        candidate = json.loads(original_inventory)
        candidate["entries"]["payload/kani-0.68.0/bin/kani-driver"] = {
            "kind": "file", "bytes": target.stat().st_size,
            "sha256": hashlib.sha256(target.read_bytes()).hexdigest()}
        inventory_path.write_text(json.dumps(candidate))
        specification["sha256"] = hashlib.sha256(inventory_path.read_bytes()).hexdigest()
        self.write_policy()
        with self.assertRaisesRegex(CHECK.Incomplete, "Entrypoint is not hash-admitted: kani"):
            self.admit_kani()
        target.write_bytes(files[target])
        target.chmod(0o500)
        inventory_path.write_bytes(original_inventory)
        specification["sha256"] = hashlib.sha256(original_inventory).hexdigest()
        self.write_policy()
        self.admit_kani()

    def test_kani_proof_keeps_shipping_executable_and_bootstrap_helper_checks(self):
        _, manifest, _, payload, _ = self.kani_fixture()
        self.admit_kani()
        for name in manifest["rust"]["linux_binary_sha256"]:
            target = self.stable / "bin" / name
            original = target.read_bytes()
            target.chmod(0o600)
            target.write_bytes(bytes([original[0] ^ 1]) + original[1:])
            try:
                with self.subTest(name=name), \
                     self.assertRaisesRegex(CHECK.Incomplete, "Stable compiler executable digest"):
                    self.admit_kani()
            finally:
                target.write_bytes(original)
                target.chmod(0o500)
        with patch.object(CHECK.RUNTIME, "_helpers", side_effect=CHECK.Incomplete("fixture helper denial")), \
             patch.object(Path, "home", return_value=self.home), \
             patch.dict(CHECK.EXPECTED, {"kani": self.kani_pin}), \
             self.assertRaisesRegex(CHECK.Incomplete, "fixture helper denial"):
            CHECK.admit_kani_proof(self.checkout, self.pointer)
        driver = payload / "bin/kani-driver"
        driver.chmod(0o400)
        with self.assertRaisesRegex(CHECK.Incomplete, "Nonexecutable tool entrypoint: kani"):
            self.admit_kani()
        driver.chmod(0o500)
        self.admit_kani()

    def test_kani_proof_rejects_pointer_drift_during_selected_admission(self):
        self.kani_fixture()
        self.admit_kani()
        original = self.pointer.read_bytes()
        real_check = CHECK.check_inventory

        def drift(base, specification, stable):
            result = real_check(base, specification, stable)
            candidate = json.loads(original)
            candidate["status"] = "installing"
            self.pointer.write_text(json.dumps(candidate))
            return result

        with patch.object(CHECK, "check_inventory", side_effect=drift), \
             self.assertRaisesRegex(CHECK.Incomplete, "pointer changed during kani-proof"):
            self.admit_kani()
        self.pointer.write_bytes(original)
        self.admit_kani()

    def test_kani_proof_namespace_exposes_selected_link_and_denies_omitted_canaries(self):
        """Actual Linux namespace; only admitted Python runs, never a fake analyzer.

        A broad mount exposes all canaries. The exact returned roots retain the
        Kani-to-compiler link while hiding unchecked settings and other stores.
        """
        _, _, files, payload, compiler = self.kani_fixture()
        omitted = [self.base / "common/bin/omitted-canary",
                   self.base / "lockbud/bin/omitted-canary",
                   self.stable / "bin/rustc",
                   self.base / "kani/rustup/settings.toml",
                   self.base / "kani/cargo-home/config.toml"]
        for path in omitted:
            if not path.exists():
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(b"unchecked namespace canary")
        admitted = self.admit_kani()
        work = self.scratch / "namespace-work"
        work.mkdir(mode=0o700)
        linked_library = payload / "toolchain/lib/librustc_driver.so"
        library = compiler / "lib/librustc_driver.so"
        original = files[library]
        script = ("from pathlib import Path\nimport json\n"
                  f"assert Path({str(linked_library)!r}).read_bytes() == {original!r}\n"
                  f"assert Path({str(linked_library)!r}).resolve() == Path({str(library)!r})\n"
                  f"visible = [p for p in {list(map(str, omitted))!r} if Path(p).exists()]\n"
                  f"try: Path({str(library)!r}).write_bytes(b'forbidden payload write')\n"
                  "except OSError: pass\n"
                  "else: raise SystemExit('selected payload was writable')\n"
                  "print(json.dumps(visible))\n")
        python = CHECK.RUNTIME._helpers(self.checkout)["python3"]
        broad = CHECK.RUNTIME.run(self.checkout, work, [python, "-c", script],
                                  readonly=(self.base, self.stable), timeout=10)
        self.assertEqual(json.loads(CHECK.RUNTIME.ENV.require_ok(broad)), list(map(str, omitted)))
        selected = CHECK.RUNTIME.run(self.checkout, work, [python, "-c", script],
                                     readonly=admitted["readonly_roots"], timeout=10)
        self.assertEqual(json.loads(CHECK.RUNTIME.ENV.require_ok(selected)), [])
        self.assertEqual(library.read_bytes(), original)

    def test_stable_core_admits_only_complete_selected_payload_and_pointer_identity(self):
        specification, _ = self.stable_fixture()
        real_check = CHECK.check_inventory
        with patch.object(Path, "home", return_value=self.home), \
             patch.object(CHECK, "check_inventory", wraps=real_check) as inventories, \
             patch.object(CHECK.RUNTIME, "_helpers", return_value={}) as helpers, \
             patch.object(CHECK.RUNTIME, "run") as run:
            admitted = CHECK.admit(self.checkout, self.pointer, profile="stable-core")
        inventories.assert_called_once_with(self.base, specification, self.stable)
        helpers.assert_called_once_with(self.checkout)
        run.assert_not_called()
        self.assertEqual(admitted["admission_profile"], "stable-core")
        self.assertEqual(admitted["readonly_roots"], tuple(self.stable / scope for scope in
                         ("bin", "etc", "lib", "libexec", "share")))
        self.assertEqual(admitted["installation_pointer"], str(self.pointer))
        self.assertEqual(admitted["installation_pointer_sha256"],
                         hashlib.sha256(self.pointer.read_bytes()).hexdigest())
        self.assertEqual([item["root"] for item in admitted["inventories"]], ["stable"])
        for item in self.policy["inventories"]:
            if item["root"] != "stable":
                self.assertFalse((self.base / item["path"]).exists())

    def test_default_and_explicit_full_still_admit_all_four_roots_and_entrypoints(self):
        self.stable_fixture()
        entrypoints = {}
        for item in self.policy["tools"].values():
            path = self.base / item["entrypoint"]
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"independent optional entrypoint")
            path.chmod(0o500)
            entrypoints[path] = item["sha256"]
        covered = {item["entrypoint"] for item in self.policy["tools"].values()}
        real_digest = CHECK.digest

        def inventory(base, specification, stable):
            self.assertEqual((base, stable), (self.base, self.stable))
            return {"root": specification["root"]}, covered

        def digest(path):
            return entrypoints[path] if path in entrypoints else real_digest(path)

        for options in ({}, {"profile": "full"}):
            with self.subTest(options=options), patch.object(Path, "home", return_value=self.home), \
                 patch.object(CHECK, "check_inventory", side_effect=inventory) as inventories, \
                 patch.object(CHECK, "digest", side_effect=digest), \
                 patch.object(CHECK.RUNTIME, "_helpers", return_value={}) as helpers:
                admitted = CHECK.admit(self.checkout, self.pointer, **options)
            self.assertEqual([call.args[1]["root"] for call in inventories.call_args_list],
                             [item["root"] for item in self.policy["inventories"]])
            self.assertEqual(inventories.call_count, 4)
            self.assertEqual(admitted["admission_profile"], "full")
            self.assertEqual(admitted["readonly_roots"], (self.base, self.stable))
            helpers.assert_called_once_with(self.checkout)

    def test_unknown_admission_profile_rejects_before_payload_admission(self):
        for profile in (None, True, "", "stable", "stable-core ", []):
            with self.subTest(profile=profile), patch.object(CHECK, "check_inventory") as inventories, \
                 self.assertRaises(CHECK.Incomplete):
                CHECK.admit(self.checkout, self.pointer, profile=profile)
            inventories.assert_not_called()

    def test_stable_core_keeps_full_description_schema_and_exact_selected_scopes(self):
        self.stable_fixture()
        healthy = copy.deepcopy(self.policy)
        mutations = [lambda p: p.update(inventories=[]),
                     lambda p: p["inventories"].pop(),
                     lambda p: p["inventories"].append(copy.deepcopy(p["inventories"][0])),
                     lambda p: next(item for item in p["inventories"] if item["root"] == "stable").update(name="renamed"),
                     lambda p: next(item for item in p["inventories"] if item["root"] == "stable").update(scopes=["bin", "lib"]),
                     lambda p: next(item for item in p["inventories"] if item["root"] == "stable")["scopes"].append("extra"),
                     lambda p: next(item for item in p["inventories"] if item["root"] == "common").update(path="../outside"),
                     lambda p: next(item for item in p["inventories"] if item["root"] == "common").update(sha256="invalid"),
                     lambda p: next(item for item in p["inventories"] if item["root"] == "common").update(scopes=[]),
                     lambda p: next(item for item in p["inventories"] if item["root"] == "common").update(scopes=["bin"]),
                     lambda p: next(item for item in p["inventories"] if item["root"] == "common")["scopes"].append("bin/nested")]
        for mutate in mutations:
            self.policy = copy.deepcopy(healthy)
            mutate(self.policy)
            self.write_policy()
            with self.subTest(mutate=mutate), self.assertRaises(CHECK.Incomplete):
                self.admit_stable()
        self.policy = healthy
        self.write_policy()
        self.assertEqual(self.admit_stable()["admission_profile"], "stable-core")

    def test_stable_core_actual_inventory_faults_reject_with_healthy_neighbors(self):
        self.stable_fixture()
        target = self.stable / "lib/dependency.so"
        original = target.read_bytes()
        extra = self.stable / "bin/undeclared-dependency"
        peer = self.scratch / "same-content-peer.so"
        peer.write_bytes(original)
        self.admit_stable()
        for fault in ("missing", "changed-same-length-restored-mtime", "symlink", "undeclared"):
            before = target.stat()
            if fault == "missing":
                target.unlink()
            elif fault == "changed-same-length-restored-mtime":
                target.write_bytes(bytes([original[0] ^ 1]) + original[1:])
                os.utime(target, ns=(before.st_atime_ns, before.st_mtime_ns))
            elif fault == "symlink":
                target.unlink()
                target.symlink_to(peer)
            else:
                extra.write_bytes(b"undeclared selected dependency")
            try:
                with self.subTest(fault=fault), self.assertRaises(CHECK.Incomplete):
                    self.admit_stable()
            finally:
                if target.is_symlink():
                    target.unlink()
                target.write_bytes(original)
                extra.unlink(missing_ok=True)
            self.admit_stable()

    def test_stable_core_scope_redirect_cannot_admit_same_content_payload(self):
        self.stable_fixture()
        self.admit_stable()
        scope = self.stable / "lib"
        outside = self.scratch / "same-content-library-scope"
        scope.rename(outside)
        scope.symlink_to(outside, target_is_directory=True)
        with self.assertRaises(CHECK.Incomplete):
            self.admit_stable()
        scope.unlink()
        outside.rename(scope)
        self.admit_stable()

    def test_stable_core_pointer_drift_during_selected_admission_is_rejected(self):
        self.stable_fixture()
        original = self.pointer.read_bytes()
        real_check = CHECK.check_inventory

        def drift(base, specification, stable):
            result = real_check(base, specification, stable)
            candidate = json.loads(original)
            candidate["status"] = "installing"
            self.pointer.write_text(json.dumps(candidate))
            return result

        self.admit_stable()
        with patch.object(CHECK, "check_inventory", side_effect=drift), self.assertRaises(CHECK.Incomplete):
            self.admit_stable()
        self.pointer.write_bytes(original)
        self.admit_stable()

    def test_stable_core_keeps_compiler_hash_and_bootstrap_helper_admission(self):
        _, manifest = self.stable_fixture()
        self.admit_stable()
        manifest_path = self.checkout / "policy/tool-manifest.json"
        original = manifest_path.read_bytes()
        manifest["rust"]["linux_binary_sha256"]["cargo"] = "0" * 64
        manifest_path.write_text(json.dumps(manifest))
        with self.assertRaisesRegex(CHECK.Incomplete, "compiler executable digest"):
            self.admit_stable()
        manifest_path.write_bytes(original)
        with patch.object(Path, "home", return_value=self.home), \
             patch.object(CHECK.RUNTIME, "_helpers", side_effect=CHECK.Incomplete("independent helper denial")), \
             self.assertRaisesRegex(CHECK.Incomplete, "independent helper denial"):
            CHECK.admit(self.checkout, self.pointer, profile="stable-core")
        self.admit_stable()

    def test_admitted_profiles_are_exact(self):
        CHECK.validate_policy(self.policy)
        for name in CHECK.PROFILES:
            for mutation in ([], self.policy["profiles"][name] + ["--unsafe"],
                             self.policy["profiles"][name][:-1], self.policy["profiles"][name] * 2):
                candidate = copy.deepcopy(self.policy)
                candidate["profiles"][name] = mutation
                with self.subTest(profile=name, mutation=mutation), self.assertRaises(CHECK.Incomplete):
                    CHECK.validate_policy(candidate)

    def test_tool_set_shipping_pin_codeql_and_private_compilers_cannot_drift(self):
        mutations = [lambda p: p.update(shipping_toolchain="nightly"),
                     lambda p: p.update(shipping_toolchain="1.99.1"),
                     lambda p: p.update(codeql="enabled"), lambda p: p["tools"].pop("loom"),
                     lambda p: p["tools"].update(extra={}),
                     lambda p: p["tools"]["lockbud"].update(toolchain="stable"),
                     lambda p: p["tools"]["miri"].update(sha256="0" * 64),
                     lambda p: p.update(miri_sysroot="/tmp/unchecked"),
                     lambda p: p.update(common_nightly_pin="nightly"),
                     lambda p: p.update(tools=[])]
        for mutate in mutations:
            candidate = copy.deepcopy(self.policy)
            mutate(candidate)
            with self.subTest(mutation=mutate), self.assertRaises(CHECK.Incomplete):
                CHECK.validate_policy(candidate)

    def test_duplicate_keys_and_nonfinite_json_are_rejected(self):
        for content in (b'{"entries":{"a":1,"a":2}}', b'{"profile":NaN}', b'\xff'):
            with self.subTest(content=content), self.assertRaises(CHECK.Incomplete):
                CHECK.decode(content)

    def test_traversal_and_noncanonical_paths_are_rejected(self):
        for path in (".", "../tool", "/tool", "a/../../tool", "a//tool", "a/./tool", "a\\tool", "a/", "", None):
            with self.subTest(path=path), self.assertRaises(CHECK.Incomplete):
                CHECK.relative(path)
        self.assertEqual(CHECK.relative("bin/analyzer"), "bin/analyzer")

    def test_private_pointer_healthy_control(self):
        with patch.object(Path, "home", return_value=self.home):
            self.assertEqual(CHECK.load_installation(self.pointer, self.checkout), (self.base, self.stable))

    def test_special_file_pointer_is_rejected_without_waiting_for_a_writer(self):
        fifo = self.scratch / "fifo-pointer"
        os.mkfifo(fifo, 0o600)
        with self.assertRaises(CHECK.Incomplete):
            CHECK.read_once(fifo, private=True)

    def test_complete_fixture_snapshot_covers_nonrust_inputs_and_added_configuration(self):
        work = self.scratch / "work"
        project = work / "controls/future-project"
        project.mkdir(parents=True)
        (project / "native-input.h").write_text("#define EXPECTED 1\n")
        original = CHECK.input_snapshot(work, [work / "controls"])
        (project / "native-input.h").write_text("#define EXPECTED 0\n")
        self.assertNotEqual(original, CHECK.input_snapshot(work, [work / "controls"]))
        (project / "native-input.h").write_text("#define EXPECTED 1\n")
        self.assertEqual(original, CHECK.input_snapshot(work, [work / "controls"]))
        (project / ".cargo").mkdir()
        (project / ".cargo/config.toml").write_text('[alias]\ncompile="unsafe"\n')
        self.assertNotEqual(original, CHECK.input_snapshot(work, [work / "controls"]))
        (work / ".cargo").mkdir()
        with self.assertRaises(CHECK.Incomplete):
            CHECK.input_snapshot(work, [work / "controls"])

    def test_mutable_store_root_symlink_is_rejected_before_any_traversal(self):
        work = self.scratch / "work"
        work.mkdir()
        outside = self.scratch / "outside"
        outside.mkdir()
        (outside / "private-canary").write_text("must never be enumerated or hashed")
        store = work / "vet-store"
        store.symlink_to(outside, target_is_directory=True)
        with patch.object(CHECK.os, "scandir") as enumerate_directory, \
             patch.object(CHECK, "digest") as hash_file, self.assertRaises(CHECK.Incomplete):
            CHECK.input_snapshot(work, [store], maximum_entries=4)
        enumerate_directory.assert_not_called()
        hash_file.assert_not_called()
        with patch.object(CHECK.os, "scandir") as enumerate_directory, \
             patch.object(CHECK, "digest") as hash_file, self.assertRaises(CHECK.Incomplete):
            CHECK.source_hashes(store)
        enumerate_directory.assert_not_called()
        hash_file.assert_not_called()
        with patch.object(CHECK.os, "scandir") as enumerate_directory, self.assertRaises(CHECK.Incomplete):
            CHECK.vet_store_snapshot(work, store)
        enumerate_directory.assert_not_called()
        store.unlink()
        store.mkdir()
        for name in ("config.toml", "audits.toml", "imports.lock", "unexpected"):
            (store / name).write_text("")
        with self.assertRaisesRegex(CHECK.Incomplete, "entry budget"):
            CHECK.input_snapshot(work, [store], maximum_entries=4)

    def test_bad_pointer_permissions_links_status_and_unpinned_stable_fail(self):
        original = self.pointer.read_text()
        for mode in (0o644, 0o400, 0o777):
            self.pointer.chmod(mode)
            with self.subTest(mode=mode), patch.object(Path, "home", return_value=self.home), self.assertRaises(CHECK.Incomplete):
                CHECK.load_installation(self.pointer, self.checkout)
        self.pointer.chmod(0o600)
        for key, value in (("status", "installing"), ("platform", "windows"),
                           ("root", str(self.base / ".." / "installation")), ("stable_toolchain", str(self.base))):
            candidate = json.loads(original)
            candidate[key] = value
            self.pointer.write_text(json.dumps(candidate))
            with self.subTest(key=key), patch.object(Path, "home", return_value=self.home), self.assertRaises(CHECK.Incomplete):
                CHECK.load_installation(self.pointer, self.checkout)
        self.pointer.write_text(original)
        alias = self.scratch / "pointer-alias"
        alias.symlink_to(self.pointer)
        with patch.object(Path, "home", return_value=self.home), self.assertRaises(OSError):
            CHECK.load_installation(alias, self.checkout)
        alias.unlink()
        alias.hardlink_to(self.pointer)
        with patch.object(Path, "home", return_value=self.home), self.assertRaises(CHECK.Incomplete):
            CHECK.load_installation(self.pointer, self.checkout)

    def test_previous_installed_compiler_cannot_satisfy_current_pointer(self):
        previous = self.home / f".rustup/toolchains/1.99.1-{CHECK.PLATFORM}"
        previous.mkdir()
        with patch.object(Path, "home", return_value=self.home):
            self.assertEqual(CHECK.load_installation(self.pointer, self.checkout),
                             (self.base, self.stable))
            candidate = json.loads(self.pointer.read_text())
            candidate["stable_toolchain"] = str(previous)
            self.pointer.write_text(json.dumps(candidate))
            with self.assertRaisesRegex(CHECK.Incomplete, "canonical private-pointer"):
                CHECK.load_installation(self.pointer, self.checkout)

    def test_complete_inventory_healthy_control_and_drift(self):
        specification, _, _ = self.inventory()
        report, covered = self.check_inventory(specification)
        self.assertEqual(report["entries"], 1)
        self.assertEqual(covered, {"common/bin/analyzer"})
        target = self.base / "common/bin/analyzer"
        target.write_bytes(b"substituted executable")
        with self.assertRaises(CHECK.Incomplete):
            self.check_inventory(specification)

    def test_inventory_hash_is_checked_before_json_parse(self):
        specification, _, path = self.inventory()
        path.write_text("unparseable substituted bytes")
        with patch.object(CHECK, "decode") as parse, self.assertRaisesRegex(CHECK.Incomplete, "SHA256 mismatch"):
            self.check_inventory(specification)
        parse.assert_not_called()

    def test_inventory_rejects_added_missing_and_special_entries(self):
        for case in ("extra", "missing", "fifo"):
            specification, _, _ = self.inventory()
            target = self.base / "common/bin/analyzer"
            if case == "extra":
                (target.parent / "injected.so").write_text("extra")
            elif case == "missing":
                target.unlink()
            else:
                target.unlink()
                os.mkfifo(target)
            with self.subTest(case=case), self.assertRaises((CHECK.Incomplete, OSError)):
                self.check_inventory(specification)
            for item in target.parent.iterdir():
                item.unlink()

    def test_inventory_terminal_name_cannot_admit_an_actual_directory(self):
        """A named terminal remains subject to actual kind checks in workers."""
        for kind in ("file", "symlink"):
            for populated in (False, True):
                specification, value, inventory_path = self.inventory()
                target = self.base / "common/bin/analyzer"
                target.unlink()
                target.mkdir()
                if populated:
                    (target / "hidden-payload").write_bytes(b"must not be admitted")
                if kind == "symlink":
                    value["entries"]["bin/analyzer"] = {"kind": "symlink", "target": "missing-target"}
                    inventory_path.write_text(json.dumps(value))
                    specification["sha256"] = hashlib.sha256(inventory_path.read_bytes()).hexdigest()
                with self.subTest(kind=kind, populated=populated), \
                     patch.object(CHECK, "digest") as payload_hash, self.assertRaises(CHECK.Incomplete):
                    self.check_inventory(specification)
                payload_hash.assert_not_called()
                shutil.rmtree(target)

    def test_inventory_unlisted_empty_directories_remain_harmless(self):
        """Inventory terminal discovery still allows unlisted empty directories."""
        specification, _, _ = self.inventory()
        (self.base / "common/bin/empty/nested").mkdir(parents=True)
        report, covered = self.check_inventory(specification)
        self.assertEqual(report["entries"], 1)
        self.assertEqual(covered, {"common/bin/analyzer"})

    def test_inventory_checks_bytes_when_payload_size_and_mtime_are_preserved(self):
        """A metadata lookalike cannot replace the exact admitted payload."""
        specification, _, _ = self.inventory()
        self.assertEqual(self.check_inventory(specification)[0]["entries"], 1)
        target = self.base / "common/bin/analyzer"
        before = target.stat()
        payload = target.read_bytes()
        target.write_bytes(bytes([payload[0] ^ 1]) + payload[1:])
        os.utime(target, ns=(before.st_atime_ns, before.st_mtime_ns))
        self.assertEqual(target.stat().st_size, before.st_size)
        self.assertEqual(target.stat().st_mtime_ns, before.st_mtime_ns)
        with self.assertRaises(CHECK.Incomplete):
            self.check_inventory(specification)

    def test_inventory_membership_rejections_precede_payload_hashing(self):
        """An extra or missing terminal prevents any payload hashing."""
        for case in ("extra-file", "extra-link", "missing"):
            specification, _, _ = self.inventory()
            target = self.base / "common/bin/analyzer"
            extra = target.parent / "extra"
            if case == "extra-file":
                extra.write_bytes(b"unlisted")
            elif case == "extra-link":
                extra.symlink_to(self.scratch / "outside-the-scope")
            else:
                target.unlink()
            with self.subTest(case=case), patch.object(CHECK, "digest") as payload_hash, \
                 self.assertRaises(CHECK.Incomplete):
                self.check_inventory(specification)
            payload_hash.assert_not_called()
            if extra.exists() or extra.is_symlink():
                extra.unlink()

    def test_inventory_terminal_cannot_hide_a_declared_descendant(self):
        """Conflicting terminal/descendant entries cannot form an exact inventory."""
        specification, value, inventory_path = self.inventory()
        target = self.base / "common/bin/analyzer"
        target.unlink()
        target.mkdir()
        child = target / "child"
        child.write_bytes(b"independent nested bytes")
        value["entries"]["bin/analyzer/child"] = {"kind": "file", "bytes": child.stat().st_size,
                                                 "sha256": hashlib.sha256(child.read_bytes()).hexdigest()}
        inventory_path.write_text(json.dumps(value))
        specification["sha256"] = hashlib.sha256(inventory_path.read_bytes()).hexdigest()
        with patch.object(CHECK, "digest") as payload_hash, self.assertRaises(CHECK.Incomplete):
            self.check_inventory(specification)
        payload_hash.assert_not_called()

    def test_inventory_cannot_omit_required_scope_or_add_traversal(self):
        specification, _, _ = self.inventory()
        for scopes in ([], ["bin", "bin"], ["bin", "bin/subdir"], ["../common/bin"], ["bin", "../escape"]):
            candidate = copy.deepcopy(specification)
            candidate["scopes"] = scopes
            with self.subTest(scopes=scopes), self.assertRaises(CHECK.Incomplete):
                self.check_inventory(candidate)

    def test_symlink_must_match_text_and_remain_in_selected_root_and_declared_scope(self):
        specification, value, path = self.inventory()
        link = self.base / "common/bin/link"
        link.symlink_to("analyzer")
        value["entries"]["bin/link"] = {"kind": "symlink", "target": "analyzer",
                                       "resolved_analysis_relative": "ignored audit metadata"}
        path.write_text(json.dumps(value))
        specification["sha256"] = CHECK.digest(path)
        self.assertEqual(self.check_inventory(specification)[0]["entries"], 2)
        link.unlink()
        link.symlink_to("./analyzer")
        with self.assertRaises(CHECK.Incomplete):
            self.check_inventory(specification)
        link.unlink()
        external = self.base / "kani/payload"
        external.parent.mkdir()
        external.write_bytes(b"admitted elsewhere, forbidden selected-root escape")
        link.symlink_to(external)
        value["entries"]["bin/link"]["target"] = str(external)
        path.write_text(json.dumps(value))
        specification["sha256"] = CHECK.digest(path)
        with self.assertRaises(CHECK.Incomplete):
            self.check_inventory(specification)

    def test_duplicate_inventory_entry_with_correct_outer_hash_is_rejected(self):
        specification, _, path = self.inventory()
        original = path.read_text()
        path.write_text(original.replace('"entries": {', '"entries": {"duplicate": {}, "duplicate": {},', 1))
        specification["sha256"] = CHECK.digest(path)
        with self.assertRaisesRegex(CHECK.Incomplete, "Duplicate JSON key"):
            self.check_inventory(specification)

    def test_unsafe_policy_or_absent_inventory_prevents_all_execution(self):
        with patch.object(CHECK.RUNTIME, "run") as run, patch.object(Path, "home", return_value=self.home):
            self.policy["profiles"]["miri_flags"] = ["-Zmiri-disable-isolation"]
            self.write_policy()
            with self.assertRaises(CHECK.Incomplete):
                CHECK.admit(self.checkout, self.pointer)
            self.policy["profiles"] = copy.deepcopy(CHECK.PROFILES)
            self.policy["inventories"] = []
            self.write_policy()
            with self.assertRaisesRegex(CHECK.Incomplete, "inventories"):
                CHECK.admit(self.checkout, self.pointer)
        run.assert_not_called()

    def test_fresh_external_private_receipt_cannot_overwrite_or_redirect(self):
        output = self.scratch / "receipt.json"
        with CHECK.receipt_file(output, self.checkout) as receipt:
            receipt.write("independent receipt")
        self.assertEqual(output.stat().st_mode & 0o777, 0o600)
        with self.assertRaises(FileExistsError):
            CHECK.receipt_file(output, self.checkout)
        alias = self.scratch / "alias.json"
        alias.symlink_to(output)
        with self.assertRaises(OSError):
            CHECK.receipt_file(alias, self.checkout)
        with self.assertRaises(CHECK.Incomplete):
            CHECK.receipt_file(self.checkout / "report.json", self.checkout)
        public = self.scratch / "public"
        public.mkdir(mode=0o755)
        with self.assertRaises(CHECK.Incomplete):
            CHECK.receipt_file(public / "report.json", self.checkout)

    def test_actual_nonzero_and_custody_failures_cannot_masquerade_as_detections(self):
        for result in ({"exit_code": 0, "failure": None, "output": "Undefined Behavior"},
                       {"exit_code": 1, "failure": "timeout", "output": "Undefined Behavior"},
                       {"exit_code": 1, "failure": None, "output": "compiler setup failed"}):
            with self.subTest(result=result), self.assertRaises(CHECK.Incomplete):
                CHECK.require_result(result, negative="Undefined Behavior")
        CHECK.require_result({"exit_code": 1, "failure": None, "output": "Undefined Behavior"}, negative="Undefined Behavior")

    def kani_report(self, defect=False):
        checks = [{"category": "assertion", "status": "Failure" if defect else "Success",
                   "description": "assertion failed: u32::from(ns) == wide"}]
        checks += [{"category": "cover", "status": "Satisfied"} for _ in range(3)]
        return {"verification_results": {"summary": {"status": "completed", "executed": 1,
                    "total_harnesses": 1, "successful": 0 if defect else 1, "failed": 1 if defect else 0},
                "results": [{"harness_id": "conversion_is_exact_or_rejected", "status": "Failure" if defect else "Success", "checks": checks}]}}

    def test_kani_requires_actual_harness_nonempty_checks_and_intended_assertion(self):
        for defect in (False, True):
            self.assertEqual(CHECK.kani_counts(self.kani_report(defect), defect)["harnesses"], 1)
            for mutate in (lambda p: p["summary"].update(executed=0),
                           lambda p: p.update(results=[]),
                           lambda p: p["results"][0].update(checks=[]),
                           lambda p: p["results"][0].update(harness_id="other")):
                report = self.kani_report(defect)
                mutate(report["verification_results"])
                with self.subTest(defect=defect), self.assertRaises(CHECK.Incomplete):
                    CHECK.kani_counts(report, defect)
        wrong = self.kani_report(True)
        wrong["verification_results"]["results"][0]["checks"][0]["description"] = "unrelated compiler crash"
        with self.assertRaises(CHECK.Incomplete):
            CHECK.kani_counts(wrong, True)

    def test_default_main_never_runs_fixture_or_accepts_previous_receipt(self):
        admitted = {"base": self.base, "policy_sha256": "a" * 64, "inventories": []}
        with patch.object(CHECK, "admit", return_value=admitted), \
             patch.object(CHECK, "fixtures") as fixtures, patch("sys.stdout", new_callable=io.StringIO) as stdout:
            self.assertEqual(CHECK.main([]), 0)
        fixtures.assert_not_called()
        report = json.loads(stdout.getvalue())
        self.assertEqual(report["mode"], "admission-only")
        self.assertEqual(report["product_qualification"], "pending")
        existing = self.scratch / "old.json"
        existing.write_text('{"status":"passed"}')
        with patch.object(CHECK, "admit") as admit, patch("sys.stdout", new_callable=io.StringIO):
            self.assertEqual(CHECK.main(["--json", str(existing)]), 1)
        admit.assert_not_called()
        self.assertEqual(existing.read_text(), '{"status":"passed"}')

    def test_failed_fixture_retains_raw_partial_receipt(self):
        admitted = {"base": self.base, "policy_sha256": "a" * 64, "inventories": []}
        output = self.scratch / "failed.json"

        def failed(admission, selected, progress):
            progress({"work": "/tmp/owned-control", "lanes": [{"exit_code": 1, "output": "setup failed", "failure": None}]})
            raise CHECK.Incomplete("independent setup failure")

        with patch.object(CHECK, "admit", return_value=admitted), patch.object(CHECK, "fixtures", side_effect=failed), \
             patch("sys.stdout", new_callable=io.StringIO):
            self.assertEqual(CHECK.main(["--fixtures", "--tool", "kani", "--json", str(output)]), 1)
        report = json.loads(output.read_text())
        self.assertEqual(report["status"], "incomplete")
        self.assertEqual(report["lanes"][0]["output"], "setup failed")

    def test_vet_requires_success_without_exemptions_and_actual_missing_loom_review(self):
        healthy = {"conclusion": "success", "vetted_fully": [], "vetted_partially": [], "vetted_with_exemptions": []}
        rejected = {"conclusion": "fail (vetting)", "failures": [{"name": "loom", "version": "0.7.2",
                                                                    "missing_criteria": ["safe-to-deploy"]}]}
        self.assertEqual(CHECK.vet_counts(healthy, False)["exemptions"], 0)
        self.assertEqual(CHECK.vet_counts(rejected, True)["missing_audits"], 1)
        for candidate, defect in (({}, False), (dict(healthy, vetted_with_exemptions=["fake"]), False),
                                  (dict(rejected, failures=[]), True),
                                  ({"conclusion": "fail (vetting)", "failures": [{"name": "other"}]}, True)):
            with self.subTest(candidate=candidate), self.assertRaises(CHECK.Incomplete):
                CHECK.vet_counts(candidate, defect)

    def test_vet_control_store_rejects_even_unused_audits_imports_or_exemptions(self):
        store = self.scratch / "vet-store"
        store.mkdir()
        defaults = {"config.toml": '[cargo-vet]\nversion="0.10"\n',
                    "audits.toml": "[audits]\n", "imports.lock": "# empty imports\n"}
        for name, text in defaults.items():
            (store / name).write_text(text)
        CHECK.empty_vet_store(store)
        for name, addition in (("config.toml", '\n[exemptions]\nunused=[]\n'),
                               ("config.toml", '\n[imports.unused]\nurl="https://example.invalid"\n'),
                               ("audits.toml", '\n[[audits.unused]]\nversion="1.0.0"\n'),
                               ("imports.lock", '\n[unused]\n')):
            (store / name).write_text(defaults[name] + addition)
            with self.subTest(name=name, addition=addition), self.assertRaises(CHECK.Incomplete):
                CHECK.empty_vet_store(store)
            (store / name).write_text(defaults[name])
        (store / "extra.toml").write_text("# forbidden additional policy\n")
        with self.assertRaises(CHECK.Incomplete):
            CHECK.empty_vet_store(store)

    def test_miri_all_four_seeds_require_counts_even_with_interleaved_summaries(self):
        interleaved = "".join(f"Trying seed: {seed}\n" for seed in range(4))
        interleaved += "running 1 test\n" * 4
        interleaved += "test result: " * 4 + "ok" * 4 + ". 1 passed; 0 failed; 0 ignored;" * 4
        self.assertEqual(CHECK.miri_count(interleaved), 4)
        for output in (interleaved.replace("Trying seed: 3", "Trying seed: 2"),
                       interleaved.replace("running 1 test", "running 0 tests", 1),
                       interleaved.replace(". 1 passed;", ". 0 passed;", 1),
                       interleaved + "panicked", ""):
            with self.subTest(output=output), self.assertRaises(CHECK.Incomplete):
                CHECK.miri_count(output)

    def test_fixture_launches_use_pinned_fixed_commands_and_independent_counts(self):
        (self.base / "common/cargo/registry/cache").mkdir(parents=True)
        (self.base / "common/cargo/registry/index").mkdir()
        (self.base / "common/cargo/registry/src").mkdir()
        source = CHECK.CONTROLS.create(self.base / "common/controls-source")
        for project in source.values():
            (project / "Cargo.lock").write_text("version = 4\n")
        (self.base / self.policy["common_nightly"]).mkdir(parents=True)
        admission = {"base": self.base, "root": self.checkout, "stable": self.stable, "policy": self.policy}
        calls = []

        def run(root, work, command, **options):
            calls.append((command, options))
            self.assertEqual(options["readonly"], [self.base, self.stable])
            self.assertEqual(set(path.name for path in options["sealed"][:4]), {"controls", "cache", "index", "src"})
            self.assertFalse(any(key.startswith("LD_") for key in options["environment"]))
            output, exit_code = "", 0
            if "test" in command:
                if any("healthy_owned_value" in item for item in command):
                    self.assertNotIn("RUSTC", options["environment"])
                    self.assertNotIn("RUSTDOC", options["environment"])
                    self.assertEqual(options["environment"]["MIRIFLAGS"], " ".join(CHECK.PROFILES["miri_flags"]))
                    output = "".join(f"Trying seed: {seed}\nrunning 1 test\n" for seed in range(4))
                    output += "test result: ok. 1 passed; 0 failed; 0 ignored;\n" * 4
                elif "seeded_use_after_free" in command:
                    output, exit_code = "Undefined Behavior: pointer has been freed", 1
                elif "tests::seeded_lost_update" in command:
                    output, exit_code = "both increments must survive", 101
                else:
                    output = "test result: ok. 1 passed; 0 failed; 0 ignored;"
            elif "fuzz" in command and "build" in command:
                self.assertEqual(command[command.index("--target") + 1], CHECK.PLATFORM)
                self.assertNotIn("--jobs", command)
                binary = work / Path(command[command.index("--target-dir") + 1]).relative_to("/work") / "x86_64-unknown-linux-gnu/release" / command[3]
                binary.parent.mkdir(parents=True, exist_ok=True)
                binary.write_text("emitted analyzer fixture")
            elif command[0].startswith("/work/target-fuzz-"):
                self.assertEqual(options["mode"], "sanitizer")
                for flag in CHECK.PROFILES["fuzzer_flags"]:
                    self.assertEqual(command.count(flag), 1)
                if command[0].endswith("/defect"):
                    output, exit_code = "signed byte decoding must preserve negative offsets", 77
                else:
                    output = "Done 256 runs in 0 second(s)"
            elif "vet" in command:
                self.assertEqual(options["environment"]["CARGO"], str(self.stable / "bin/cargo"))
                self.assertNotIn("check", command)
                if command[-1] == "init":
                    self.assertEqual(command[1:], ["vet", "init"])
                    supply = options["cwd"] / "supply-chain"
                    supply.mkdir()
                    (supply / "audits.toml").write_text("# empty owned fixture\n[audits]\n")
                    (supply / "config.toml").write_text('[cargo-vet]\nversion="0.10"\n')
                    (supply / "imports.lock").write_text("# empty imports\n")
                elif "unreviewed" in str(options["cwd"]):
                    self.assertIn("--store-path", command)
                    store = work / Path(command[command.index("--store-path") + 1]).relative_to("/work")
                    self.assertNotIn(store / "audits.toml", options["sealed"])
                    self.assertNotIn(store / "imports.lock", options["sealed"])
                    self.assertNotIn(store / "config.toml", options["sealed"])
                    self.assertIn(work / "controls", options["sealed"])
                    output = json.dumps({"conclusion": "fail (vetting)", "failures": [{"name": "loom", "version": "0.7.2",
                                                                                       "missing_criteria": ["safe-to-deploy"]}]})
                    exit_code = 255
                else:
                    self.assertIn("--store-path", command)
                    output = json.dumps({"conclusion": "success", "vetted_fully": [], "vetted_partially": [], "vetted_with_exemptions": []})
            elif "--harness" in command:
                defect = "kani-defect" in str(options["cwd"])
                exported = command[command.index("--export-json") + 1]
                (work / Path(exported).relative_to("/work")).write_text(json.dumps(self.kani_report(defect)))
                if defect:
                    output, exit_code = "VERIFICATION:- FAILED", 1
            elif "lockbud" in command:
                self.assertIn("--frozen", command)
                self.assertIn("--lib", command)
                if "lockbud-defect" in str(options["cwd"]):
                    output, exit_code = '"DoubleLock"', 101
                else:
                    target = work / Path(command[command.index("--target-dir") + 1]).relative_to("/work")
                    (target / "proof.rmeta").write_text("fresh analyzed library")
                    output = "Compiling nevrotick_lock_control"
            else:
                self.fail("Unexpected command: " + str(command))
            return {"command": command, "exit_code": exit_code, "output": output, "failure": None}

        state = {}
        try:
            with patch.object(CHECK.RUNTIME, "run", side_effect=run):
                result = CHECK.fixtures(admission, CHECK.TOOLS, state.update)
            self.assertEqual(len(result["lanes"]), 15)
            self.assertEqual(len(calls), 15)
            self.assertTrue(all(lane["sources_unchanged"] for lane in result["lanes"]))
            self.assertEqual([lane["observed_runs"] for lane in result["lanes"] if "observed_runs" in lane], [256])
            self.assertEqual([lane["observed_tests"] for lane in result["lanes"] if "observed_tests" in lane], [4, 1])
            stores = [lane["vet_store_lock_exception"] for lane in result["lanes"] if "vet_store_lock_exception" in lane]
            self.assertEqual(len(stores), 2)
            self.assertTrue(all(store["unchanged"] for store in stores))
        finally:
            if state.get("work"):
                shutil.rmtree(state["work"])

        def corrupt_store(root, work, command, **options):
            observed = run(root, work, command, **options)
            if "--store-path" in command:
                store = work / Path(command[command.index("--store-path") + 1]).relative_to("/work")
                (store / "config.toml").write_text('[exemptions]\nfabricated=[]\n# injected exemption\n')
            return observed

        corrupted = {}
        try:
            with patch.object(CHECK.RUNTIME, "run", side_effect=corrupt_store), self.assertRaises(CHECK.Incomplete):
                CHECK.fixtures(admission, ("vet",), corrupted.update)
            self.assertFalse(corrupted["lanes"][-1]["vet_store_lock_exception"]["unchanged"])
            self.assertIn("file size changed", corrupted["lanes"][-1]["vet_store_lock_exception"]["guard_error"])
            self.assertIn("--store-path", corrupted["lanes"][-1]["command"])
            self.assertEqual(json.loads(corrupted["lanes"][-1]["output"])["conclusion"], "success")
        finally:
            if corrupted.get("work"):
                shutil.rmtree(corrupted["work"])


if __name__ == "__main__":
    unittest.main()
