"""Private-file controls for codegen orchestration; execute no native tool."""
from __future__ import annotations

import copy
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch


HERE = Path(__file__).resolve().parent


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


V = load("era_codegen_controller_test", HERE / "verify_era_codegen.py")
REAL = load("era_codegen_custody_test", HERE.parent.parent / "scripts/check-analysis.py")
PROJECTION = load("era_codegen_source_graph_test", HERE / "era_constructor_projection.py")
POLICY = json.loads((HERE.parent.parent / "policy/ui-m0-dependencies.json").read_text())
VENDOR = load("era_codegen_vendor_closure_test", HERE / "era_vendor_admission.py")


class CodegenTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-era-codegen-test-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.root = self.base / "checkout"
        self.root.mkdir()
        self.names = V.input_names(VENDOR)
        for name in self.names:
            destination = self.root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(HERE.parent.parent / name, destination)
        (self.root / "fixture.rs").write_bytes(b"independent frozen fixture source")
        self.output = self.base / "evidence"
        self.output.mkdir(mode=0o700)
        self.pointer = self.base / "pointer.json"
        self.pointer.write_text('{"fixture":"installation pointer"}')
        self.pointer.chmod(0o600)
        self.installation = self.base / "installation"
        self.payload = self.installation / "kani/payload/kani-0.68.0"
        self.compiler = self.installation / V.FORMAL.KANI_COMPILER
        self.stable = self.base / "stable"
        for folder in (self.payload / "bin", self.compiler / "bin", self.stable):
            folder.mkdir(parents=True)
        for path in (self.payload / "bin/kani-driver", self.payload / "bin/goto-cc",
                     self.compiler / "bin/cargo", self.compiler / "bin/rustc", self.compiler / "bin/rustdoc"):
            path.write_bytes(b"tiny mock executable: never run")
        self.cc, self.ld = self.base / "cc", self.base / "ld"
        self.cc.write_bytes(b"mock pinned cc: never run")
        self.ld.write_bytes(b"mock pinned ld: never run")
        self.graph = [{"name": name, "version": version,
            "expected_features": POLICY["rows"]["x86_64-unknown-linux-gnu"]["features"][name + "@" + version]}
            for name, version in PROJECTION.GRAPH_VERSIONS.items()]
        self.tool_calls = self.vendor_calls = 0
        self.native_action = self.post_tool_action = self.final_tool_action = self.final_vendor_action = None
        self.exit_code, self.failure, self.raise_native = 0, None, False
        self.fingerprint_layout = "cargo-1.100"
        self.consumer = V.CONSUMER
        self.admitted = {"admission_profile": "kani-proof", "admission_consumer": self.consumer,
            "base": self.installation, "stable": self.stable,
            "readonly_roots": (self.payload, self.compiler), "policy_sha256": "a" * 64,
            "inventories": [{"name": "kani", "sha256": "b" * 64, "entries": 5}],
            "installation_pointer": str(self.pointer),
            "installation_pointer_sha256": V.FORMAL.digest(self.pointer.read_bytes()),
            "policy": {"tools": {"kani": {"entrypoint": V.FORMAL.KANI_ENTRYPOINT,
                                               "toolchain": V.FORMAL.KANI_COMPILER}}}}
        self.check = self.consumer_module = self.bootstrap_check = None
        self.loaded_paths = []
        self.vendor = SimpleNamespace(input_names=VENDOR.input_names, prepare=self.prepare_vendor,
                                      admit=self.admit_vendor, identity=lambda value: value)
        self.projection = SimpleNamespace(prepare_era_constructor=self.prepare_project)

    def admit_tools(self, frozen):
        self.tool_calls += 1
        if self.tool_calls == 2 and self.post_tool_action:
            self.post_tool_action()
        if self.tool_calls == 3 and self.final_tool_action:
            self.final_tool_action()
        value = copy.deepcopy(self.admitted)
        value["admission_consumer"] = self.consumer
        return value

    def prepare_vendor(self, root, destination, vendor_root=None, *, work_root=None):
        destination.mkdir(mode=0o700)
        (destination / "vendor").mkdir()
        (destination / "cargo-config.toml").write_text('[net]\noffline = true\n')
        (destination / "manifest.json").write_text('{"fixture":"admitted selected graph"}\n')

    def admit_vendor(self, root, bundle, *, work_root=None):
        self.vendor_calls += 1
        if self.vendor_calls == 4 and self.final_vendor_action:
            self.final_vendor_action()
        return {"readonly_root": bundle, "vendor_root": bundle / "vendor",
                "config_path": bundle / "cargo-config.toml", "manifest_path": bundle / "manifest.json",
                "inventory": REAL.input_snapshot(self.output, [bundle])}

    def prepare_project(self, root, project, case, harness_path, *, vendor_root=None):
        self.assertEqual(case, "healthy")
        project.mkdir()
        (project / ".cargo").mkdir()
        (project / "src").mkdir()
        (project / "Cargo.toml").write_text('[package]\nname="fixture"\nversion="0.0.0"\nedition="2024"\n')
        (project / "Cargo.lock").write_text("version = 4\n")
        (project / "src/proof.rs").write_text("// mocked source preparation; never compiled\n")
        (project / ".cargo/config.toml").write_bytes((vendor_root.parent / "cargo-config.toml").read_bytes())
        return {"entrypoint": project / "Cargo.toml", "registry_graph": self.graph,
                "preparation_only": True, "native_commands_executed": 0}

    def native(self, root, work, command, **options):
        self.persisted_native_receipt = json.loads((work / "receipts/execution.json").read_text())
        self.assertEqual(self.persisted_native_receipt["native_top_level_attempts"], 1)
        if self.raise_native:
            raise OSError("synthetic native seam failed")
        REAL.RUNTIME._sealed_inputs(work, options["sealed"])
        project = options["cwd"]
        self.models = work / "target/arbitrary-cargo-layout/pkg-hash/out"
        self.models.mkdir(parents=True)
        harnesses = []
        for index, name in enumerate(sorted(V.HARNESSES)):
            stem = self.models / ("harness_" + str(index))
            Path(str(stem) + ".symtab.out").write_bytes(b"unexecuted model fixture")
            Path(str(stem) + ".out").write_bytes(b"unexecuted linked model fixture")
            for suffix in (".type_map.json", ".pretty_name_map.json"):
                Path(str(stem) + suffix).write_text('{"fixture":"readable map"}\n')
            harnesses.append({"crate_name": V.CRATE, "pretty_name": V.CRATE + "::proof::" + name,
                "original_file": str(project / "src/proof.rs"),
                "goto_file": "/work/" + Path(str(stem) + ".symtab.out").relative_to(work).as_posix()})
        self.metadata = self.models / "fixture.kani-metadata.json"
        self.metadata.write_text(json.dumps({"crate_name": V.CRATE, "proof_harnesses": harnesses,
                                            "test_harnesses": [], "unsupported_features": []}))
        for package in self.graph:
            if self.fingerprint_layout == "cargo-1.100":
                host = package["name"] in {"garde_derive", "proc-macro2", "quote", "rustversion", "syn", "unicode-ident"}
                prefix = "target/kani/debug/build" if host else "target/kani/x86_64-unknown-linux-gnu/debug/build"
                folder = work / prefix / package["name"] / "0123456789abcdef/fingerprint"
            else:
                folder = work / "target/another-layout/.fingerprint" / (package["name"] + "-0123456789abcdef")
            folder.mkdir(parents=True)
            (folder / ("lib-" + package["name"].replace("-", "_") + ".json")).write_text(
                json.dumps({"features": json.dumps(package["expected_features"])}))
        if self.native_action:
            self.native_action()
        return {"exit_code": self.exit_code, "failure": self.failure, "output": "mocked native codegen seam"}

    def execute(self):
        def local_loader(name, path):
            self.loaded_paths.append(path)
            if path.name == "verify_era_codegen.py":
                consumer = load(name, path)
                self.consumer_module = consumer
                mocked_loader = patch.object(consumer, "load", side_effect=local_loader)
                mocked_loader.start()
                self.addCleanup(mocked_loader.stop)
                return consumer
            if path.name == "check-analysis.py":
                check = load(name, path)
                if path == self.root / "scripts/check-analysis.py":
                    self.bootstrap_check = check
                else:
                    self.check = check
                    check.admit_era_constructor_codegen = Mock(side_effect=self.admit_tools)
                    check.admit_kani_proof = Mock()
                    check.RUNTIME.run = Mock(side_effect=self.native)
                    check.RUNTIME._helpers = Mock(return_value={"cc": str(self.cc), "ld": str(self.ld)})
                return check
            if path.name == "era_vendor_admission.py":
                return self.vendor
            if path.name == "era_constructor_projection.py":
                return self.projection
            self.fail("Unexpected code load: " + str(path))

        with patch.object(V, "ROOT", self.root), patch.object(V.FORMAL, "ROOT", self.root), \
             patch.object(V, "POINTER", self.pointer), patch.object(V, "load", side_effect=local_loader):
            # Each real frozen consumer imports its own constant; override only
            # the fixture's installation pointer, never its source/helper roots.
            original_loader = local_loader
            def pointer_loader(name, path):
                value = original_loader(name, path)
                if path.name == "verify_era_codegen.py":
                    value.POINTER = self.pointer
                    value.MAX_ARTIFACT = V.MAX_ARTIFACT
                return value
            V.load.side_effect = pointer_loader
            return V.run(self.output)

    def test_healthy_single_command_has_exact_profile_mounts_custody_and_counts(self):
        result = self.execute()
        self.assertEqual(result["status"], "codegen-preflight-passed", result)
        self.assertFalse(result["proof_qualified"])
        self.assertIsNone(result["unwind_bound"])
        self.assertFalse(result["standalone_proof_flags_applied"])
        self.assertEqual((result["native_top_level_attempts"], result["native_top_level_returned"]), (1, 1))
        self.assertEqual(self.tool_calls, 3)
        self.assertEqual(self.vendor_calls, 4)
        frozen = self.output / "source-island/frozen"
        self.assertEqual(self.check.RUNTIME.run.call_args.args[0], frozen)
        self.assertIn(self.root / "scripts/check-analysis.py", self.loaded_paths)
        self.assertIn(frozen / "scripts/check-analysis.py", self.loaded_paths)
        self.assertIn(frozen / "tools/formal/verify_era_codegen.py", self.loaded_paths)
        self.assertEqual(Path(self.consumer_module.FORMAL.__file__), frozen / "tools/formal/verify.py")
        self.assertEqual(Path(self.consumer_module.run_prepared.__code__.co_filename),
                         frozen / "tools/formal/verify_era_codegen.py")
        self.assertEqual(Path(self.check.__file__), frozen / "scripts/check-analysis.py")
        self.assertEqual(Path(self.check.RUNTIME.__file__), frozen / "scripts/analysis-runtime.py")
        self.assertEqual(Path(self.check.read_once.__code__.co_filename), frozen / "scripts/check-analysis.py")
        self.assertEqual(result["original_root"], self.root)
        self.assertEqual(result["frozen_root"], frozen)
        self.check.admit_kani_proof.assert_not_called()
        command = self.check.RUNTIME.run.call_args.args[2]
        options = self.check.RUNTIME.run.call_args.kwargs
        self.assertEqual(command, [str(self.payload / "bin/kani-driver"), "kani", "--manifest-path",
            str(self.output / "source-island/project/Cargo.toml"), *V.FLAGS, "--target-dir", "/work/target"])
        self.assertEqual(options["readonly"], (self.payload, self.compiler,
            self.output / "source-island", self.output / "vendor-island"))
        self.assertEqual(options["path_dirs"], [self.payload / "bin", self.compiler / "bin"])
        self.assertEqual(options["sealed"], [self.output / "work/receipts", self.output / "work/cargo-home/config.toml"])
        self.assertEqual((options["timeout"], options["cpu_seconds"], options["address_space_bytes"],
                          options["process_limit"], options["build_jobs"]), (180, 180, 8 * 1024 ** 3, 128, 2))
        self.assertNotIn("RUSTFLAGS", options["environment"])
        self.assertNotIn("CARGO_ENCODED_RUSTFLAGS", options["environment"])
        self.assertTrue(result["all_retained_inputs_and_receipts_unchanged"])
        self.assertEqual(len(result["observed_package_features"]), 14)
        self.assertFalse(result["readable_loop_ir_available"])
        self.assertFalse(result["loop_allocator_inspection_complete"])
        self.assertTrue(any(name.endswith(".pretty_name_map.json") for name in result["artifacts"]))
        self.assertGreaterEqual(result["execute_seconds"], 0.0)
        for name in ("pre-admission.json", "post-admission.json"):
            receipt = json.loads((self.output / "work/receipts" / name).read_text())
            self.assertGreaterEqual(receipt["admission_seconds"], 0.0)

    def test_observed_cargo_1_100_host_and_target_fingerprints_are_accepted(self):
        self.fingerprint_layout = "cargo-1.100"
        result = self.execute()
        self.assertEqual(result["status"], "codegen-preflight-passed", result)
        self.assertEqual(len(result["observed_package_features"]), 14)
        self.assertTrue(any("/kani/debug/build/" in item["path"]
                            for rows in result["observed_package_features"].values() for item in rows))
        self.assertTrue(any("/kani/x86_64-unknown-linux-gnu/debug/build/" in item["path"]
                            for rows in result["observed_package_features"].values() for item in rows))

    def test_legacy_fingerprint_layout_is_not_a_current_profile(self):
        self.fingerprint_layout = "legacy"
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("do not observe all fourteen packages", result["error"])

    def test_fingerprint_ancestry_package_hash_and_library_name_are_closed(self):
        result = self.execute()
        self.assertEqual(result["status"], "codegen-preflight-passed", result)
        work = self.output / "work"
        target = work / "target"
        original = next(target.rglob("lib-garde.json"))
        candidates = (
            "kani/aarch64-unknown-linux-gnu/debug/build/garde/0123456789abcdef/fingerprint/lib-garde.json",
            "kani/x86_64-unknown-linux-gnu/release/build/garde/0123456789abcdef/fingerprint/lib-garde.json",
            "kani/debug/build/garde/not-a-hash/fingerprint/lib-garde.json",
            "kani/debug/build/garde/0123456789abcdef/extra/fingerprint/lib-garde.json",
            "kani/debug/build/unselected/0123456789abcdef/fingerprint/lib-unselected.json",
            "kani/debug/build/garde/0123456789abcdef/fingerprint/lib-forged.json",
        )
        for name in candidates:
            destination = target / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            original.rename(destination)
            try:
                with self.subTest(name=name), self.assertRaises(V.Incomplete):
                    V.observe_artifacts(REAL, work, self.output / "source-island/project", self.graph)
            finally:
                destination.rename(original)

    def test_nonzero_exit_is_counted_and_never_proof_or_codegen_pass(self):
        self.exit_code = 1
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual((result["native_top_level_attempts"], result["native_top_level_returned"]), (1, 1))
        self.assertIn("did not complete successfully", result["error"])
        self.assertFalse(result["proof_qualified"])

    def test_native_seam_exception_preserves_attempt_and_zero_return_count(self):
        self.raise_native = True
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual((result["native_top_level_attempts"], result["native_top_level_returned"]), (1, 0))
        self.assertIn("synthetic native seam failed", result["error"])

    def test_wrong_consumer_never_launches_native(self):
        self.consumer = "kani-proof"
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("different consumer", result["error"])
        self.check.RUNTIME.run.assert_not_called()

    def test_wrong_actual_features_reject_after_real_mocked_command(self):
        def tamper():
            path = next((self.output / "work/target").rglob("lib-garde.json"))
            path.write_text('{"features":"[]"}')
        self.native_action = tamper
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("Observed Cargo features differ: garde", result["error"])
        self.check.RUNTIME.run.assert_called_once()

    def test_missing_model_cannot_be_hidden_by_exit_zero(self):
        self.native_action = lambda: next(self.models.glob("*.symtab.out")).unlink()
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("model or linked output is missing", result["error"])

    def test_oversized_artifact_is_incomplete_not_silently_omitted(self):
        with patch.object(V, "MAX_ARTIFACT", 8):
            result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("empty or oversized", result["error"])

    def test_foreign_harness_source_is_rejected(self):
        def tamper():
            value = json.loads(self.metadata.read_text())
            value["proof_harnesses"][0]["original_file"] = "/outside/proof.rs"
            self.metadata.write_text(json.dumps(value))
        self.native_action = tamper
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("harness source identity differs", result["error"])

    def test_exact_cargo_relative_harness_source_is_accepted(self):
        def change_to_relative():
            value = json.loads(self.metadata.read_text())
            for harness in value["proof_harnesses"]:
                harness["original_file"] = "src/proof.rs"
            self.metadata.write_text(json.dumps(value))
        self.native_action = change_to_relative
        result = self.execute()
        self.assertEqual(result["status"], "codegen-preflight-passed", result)

    def test_relative_source_spelling_and_escapes_are_rejected(self):
        result = self.execute()
        self.assertEqual(result["status"], "codegen-preflight-passed", result)
        original = self.metadata.read_bytes()
        for source in ("proof.rs", "./src/proof.rs", "../project/src/proof.rs", "src/../src/proof.rs", "/outside/proof.rs"):
            value = json.loads(original)
            value["proof_harnesses"][0]["original_file"] = source
            self.metadata.write_text(json.dumps(value))
            with self.subTest(source=source), self.assertRaises(V.Incomplete):
                V.observe_artifacts(REAL, self.output / "work", self.output / "source-island/project", self.graph)
        value = json.loads(original)
        value["proof_harnesses"][0]["original_file"] = "src/proof.rs"
        self.metadata.write_text(json.dumps(value))
        proof = self.output / "source-island/project/src/proof.rs"
        proof.unlink()
        sentinel = self.base / "other-proof.rs"
        sentinel.write_text("// unrelated private fixture\n")
        proof.symlink_to(sentinel)
        with self.assertRaisesRegex(V.Incomplete, "source path redirects"):
            V.observe_artifacts(REAL, self.output / "work", self.output / "source-island/project", self.graph)

    def test_original_root_drift_is_not_checked_twice_against_frozen_root(self):
        self.native_action = lambda: (self.root / "crates/time-domain/src/era.rs").write_text("// changed original\n")
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("Original or frozen codegen source changed", result["error"])
        self.check.RUNTIME.run.assert_called_once()

    def test_unselected_cargo_home_config_is_rejected(self):
        self.native_action = lambda: (self.output / "work/cargo-home/config").write_text("[build]\n")
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("Unselected Cargo ancestry/home", result["error"])

    def test_host_linker_config_is_pinned_generic_and_sealed(self):
        result = self.execute()
        self.assertEqual(result["status"], "codegen-preflight-passed", result)
        self.assertEqual(result["host_linker"]["content"], '[host]\nlinker = ' + json.dumps(str(self.cc)) + '\n')
        self.assertNotIn("rustflags", result["host_linker"]["content"])

    def test_receipt_alias_does_not_overwrite_private_outside_sentinel(self):
        sentinel = self.base / "outside-sentinel"
        sentinel.write_bytes(b"PRESERVE")
        def tamper():
            path = self.output / "work/receipts/execution.json"
            path.unlink()
            path.symlink_to(sentinel)
        self.native_action = tamper
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual(sentinel.read_bytes(), b"PRESERVE")

    def test_final_tool_admission_cannot_rebase_prior_receipt(self):
        self.final_tool_action = lambda: (self.output / "work/receipts/preparation.json").write_text("{}\n")
        result = self.execute()
        self.assertEqual(self.tool_calls, 3)
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("Retained codegen receipts changed", result["final_error"])
        self.assertNotIn("all_retained_inputs_and_receipts_unchanged", result)

    def test_post_tool_admission_cannot_rebase_preparation_receipt(self):
        self.post_tool_action = lambda: (self.output / "work/receipts/preparation.json").write_text("{}\n")
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("earlier proof input island or retained receipt changed", result["error"])

    def test_post_tool_admission_cannot_rebase_emitted_model(self):
        self.post_tool_action = lambda: next(self.models.glob("*.symtab.out")).write_bytes(b"changed after codegen")
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("earlier proof input island or retained receipt changed", result["error"])

    def test_final_vendor_admission_cannot_change_final_tool_receipt(self):
        self.final_vendor_action = lambda: (self.output / "work/final-admission.json").write_text("{}\n")
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("Final tool receipt changed", result["final_error"])

    def test_closed_cli_rejects_extra_native_flags_before_output(self):
        with patch.object(V.FORMAL, "fresh_output") as output, patch("sys.stderr", new=io.StringIO()), \
             self.assertRaises(SystemExit) as exited:
            V.main(["--output-dir", str(self.output), "--solver", "z3"])
        self.assertEqual(exited.exception.code, 2)
        output.assert_not_called()


if __name__ == "__main__":
    unittest.main()
