"""Inspection provenance, frozen dispatch and file-custody controls; no native tool."""
from __future__ import annotations

import ast
import copy
import importlib.util
import io
import json
import os
from pathlib import Path
import shutil
import tempfile
from unittest.mock import Mock, patch
import unittest


HERE = Path(__file__).resolve().parent


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    value = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(value)
    return value


V = load("era_model_inspector_test", HERE / "inspect_era_model.py")
VENDOR = load("era_model_vendor_closure_test", HERE / "era_vendor_admission.py")


class InspectionTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-era-model-test-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.root, self.source, self.output = (self.base / name for name in ("checkout", "old-codegen", "inspection"))
        self.root.mkdir()
        self.output.mkdir(mode=0o700)
        for name in V.input_names():
            destination = self.root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(HERE.parent.parent / name, destination)
        self.pointer = self.base / "pointer.json"
        self.pointer.write_text('{"fixture":"private pointer"}')
        self.pointer.chmod(0o600)
        self.installation, self.stable = self.base / "installation", self.base / "stable"
        self.payload = self.installation / "kani/payload/kani-0.68.0"
        self.compiler = self.installation / V.FORMAL.KANI_COMPILER
        for directory in (self.payload / "bin", self.compiler / "bin", self.stable):
            directory.mkdir(parents=True)
        self.executable = self.payload / "bin/goto-instrument"
        self.executable.write_bytes(b"fixture inspector: never executed")
        self.inspector_hash = V.FORMAL.digest(self.executable.read_bytes())
        self.admitted = {"admission_profile": "kani-proof", "admission_consumer": V.CONSUMER,
            "base": self.installation, "stable": self.stable, "readonly_roots": (self.payload, self.compiler),
            "policy_sha256": "a" * 64, "inventories": [{"name": "kani", "sha256": "b" * 64, "entries": 5}],
            "installation_pointer": str(self.pointer), "installation_pointer_sha256": V.FORMAL.digest(self.pointer.read_bytes()),
            "policy": {"tools": {"kani": {"entrypoint": V.FORMAL.KANI_ENTRYPOINT, "toolchain": V.FORMAL.KANI_COMPILER}}}}
        self.consumer = V.CONSUMER
        self.admission_calls = 0
        self.post_action = self.final_action = self.native_action = self.copy_action = None
        self.output_bytes = b"/* inspection fixture, not compiled */\nvoid healthy(void) {}\n"
        self.exit_code, self.raise_native = 0, False
        self.check = self.frozen_consumer = None
        self.prepare_old_codegen()

    def put(self, name, value):
        path = self.source / name
        path.parent.mkdir(parents=True, exist_ok=True)
        data = (json.dumps(V.FORMAL.serializable(value), indent=2) + "\n").encode()
        path.write_bytes(data)
        return V.FORMAL.digest(data)

    def prepare_old_codegen(self):
        manifest = {}
        for name in V.CODEGEN.input_names(VENDOR):
            data = (self.root / name).read_bytes()
            if name == "scripts/check-analysis.py":
                tree = ast.parse(data)
                added = next(node for node in tree.body if isinstance(node, ast.FunctionDef)
                             and node.name == "admit_era_constructor_model_inspection")
                lines = data.splitlines(keepends=True)
                data = b"".join(lines[:added.lineno - 1] + lines[added.end_lineno:])
                self.old_helper_hash = V.FORMAL.digest(data)
                self.new_helper_hash = V.FORMAL.digest((self.root / name).read_bytes())
            path = self.source / "source-island/frozen" / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
            manifest[name] = {"bytes": len(data), "sha256": V.FORMAL.digest(data)}
        project = self.source / "source-island/project"
        (project / "src").mkdir(parents=True)
        generated = {"Cargo.toml": b'[package]\nname="fixture"\nversion="0.0.0"\n',
                     "src/proof.rs": b"// actual fixture source, never compiled\n"}
        for name, data in generated.items():
            (project / name).write_bytes(data)
        (project / "correspondence.json").write_text('{"fixture":"source context"}\n')
        self.model = self.source / "work/target/healthy.out"
        self.model.parent.mkdir(parents=True)
        self.model.write_bytes(b"verified fixture linked model; never executed")
        model_fact = {"bytes": self.model.stat().st_size, "sha256": V.FORMAL.digest(self.model.read_bytes())}
        metadata = {"proof_harnesses": [{"pretty_name": "proof::era_constructor_full_domain",
                                        "goto_file": "/work/target/healthy.symtab.out"}]}
        self.summary = {"status": "codegen-preflight-passed", "consumer": "era-constructor-codegen-v1",
            "all_retained_inputs_and_receipts_unchanged": True, "codegen_process_succeeded": True,
            "proof_qualified": False, "artifacts": {"target/healthy.out": model_fact}, "constructor_metadata": metadata}
        old = copy.deepcopy(self.admitted)
        old["admission_consumer"] = "era-constructor-codegen-v1"
        preparation = {"source_manifest": manifest, "preparation": {
            "generated_files_sha256": {name: V.FORMAL.digest(data) for name, data in generated.items()},
            "correspondence_sha256": V.FORMAL.digest((project / "correspondence.json").read_bytes())}}
        self.pins = {
            "summary.json": self.put("summary.json", self.summary),
            "work/receipts/preparation.json": self.put("work/receipts/preparation.json", preparation),
            "work/receipts/artifacts.json": self.put("work/receipts/artifacts.json", {
                "artifacts": self.summary["artifacts"], "constructor_metadata": metadata}),
            "work/final-admission.json": self.put("work/final-admission.json", {
                "consumer": "era-constructor-codegen-v1", "admitted": old}),
        }

    def admit(self, root):
        self.admission_calls += 1
        action = self.post_action if self.admission_calls == 2 else self.final_action if self.admission_calls == 3 else None
        if action:
            action()
        result = copy.deepcopy(self.admitted)
        result["admission_consumer"] = self.consumer
        return result

    def native(self, root, work, command, **options):
        receipt = json.loads((work / "receipts/execution.json").read_text())
        self.assertEqual(receipt["native_top_level_attempts"], 1)
        if self.raise_native:
            raise OSError("synthetic inspection native seam failure")
        self.check.RUNTIME._sealed_inputs(work, options["sealed"])
        (work / "era-constructor.c").write_bytes(self.output_bytes)
        if self.native_action:
            self.native_action()
        return {"exit_code": self.exit_code, "failure": None, "output": "mocked model inspection seam"}

    def execute(self):
        def local_loader(name, path):
            if path.name == "inspect_era_model.py":
                value = load(name, path)
                self.frozen_consumer = value
                value.SOURCE_ROOT, value.SOURCE_PINS = self.source, self.pins
                value.HELPER_OLD_SHA256, value.HELPER_NEW_SHA256 = self.old_helper_hash, self.new_helper_hash
                value.INSPECTOR_SHA256, value.INSPECTOR_BYTES = self.inspector_hash, self.executable.stat().st_size
                value.POINTER = self.pointer
                original_copy = value.copy_file
                def copied(source, target, expected, maximum):
                    original_copy(source, target, expected, maximum)
                    if self.copy_action:
                        self.copy_action(source, target)
                value.copy_file = copied
                mocked = patch.object(value, "load", side_effect=local_loader)
                mocked.start()
                self.addCleanup(mocked.stop)
                return value
            if path.name == "era_vendor_admission.py":
                return VENDOR
            if path.name == "check-analysis.py":
                value = load(name, path)
                if path != self.root / "scripts/check-analysis.py":
                    self.check = value
                    value.admit_era_constructor_model_inspection = Mock(side_effect=self.admit)
                    value.admit_kani_proof = Mock()
                    value.RUNTIME.run = Mock(side_effect=self.native)
                return value
            self.fail("Unexpected source load: " + str(path))

        with patch.object(V, "ROOT", self.root), patch.object(V.FORMAL, "ROOT", self.root), \
             patch.object(V, "load", side_effect=local_loader):
            return V.run(self.output)

    def test_healthy_fixed_candidate_exact_command_frozen_custody_and_no_proof_claim(self):
        result = self.execute()
        self.assertEqual(result["status"], "c-dump-generated", result)
        self.assertTrue(result["c_dump_generated"])
        self.assertFalse(result["loop_analysis_complete"])
        self.assertFalse(result["proof_qualified"])
        self.assertIsNone(result["unwind_bound"])
        self.assertEqual((result["native_top_level_attempts"], result["native_top_level_returned"]), (1, 1))
        self.assertEqual(self.admission_calls, 3)
        self.check.admit_kani_proof.assert_not_called()
        frozen = self.output / "inputs/frozen"
        call = self.check.RUNTIME.run.call_args
        self.assertEqual(call.args[0], frozen)
        self.assertEqual(call.args[2], [str(self.executable), "--dump-c", str(self.output / "inputs/model.out"), "/work/era-constructor.c"])
        self.assertEqual(call.kwargs["readonly"], (self.payload, self.compiler, self.output / "inputs"))
        self.assertEqual(call.kwargs["environment"], {})
        self.assertEqual((call.kwargs["timeout"], call.kwargs["cpu_seconds"], call.kwargs["address_space_bytes"],
                          call.kwargs["process_limit"]), (180, 180, 8 * 1024 ** 3, 128))
        self.assertEqual(Path(self.frozen_consumer.FORMAL.__file__), frozen / "tools/formal/verify.py")
        self.assertEqual(Path(self.check.RUNTIME.__file__), frozen / "scripts/analysis-runtime.py")
        self.assertEqual((self.output / "inputs/model.out").stat().st_nlink, 1)
        self.assertEqual((self.output / "inputs/model.out").read_bytes(), self.model.read_bytes())
        self.assertTrue(result["all_inputs_model_and_receipts_unchanged"])

    def test_forged_passed_receipt_cannot_replace_fixed_hash(self):
        self.summary["arbitrary_forgery"] = True
        self.put("summary.json", self.summary)
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("Pinned V3 provenance receipt changed", result["error"])
        self.check.RUNTIME.run.assert_not_called()

    def test_changed_model_is_rejected_before_native(self):
        self.model.write_bytes(b"different model bytes")
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.check.RUNTIME.run.assert_not_called()

    def test_copy_time_summary_drift_cannot_become_a_new_baseline(self):
        def drift(source, target):
            if source == self.source / "summary.json":
                source.write_bytes(source.read_bytes() + b"\n")
        self.copy_action = drift
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual(result["native_top_level_attempts"], 0)
        self.check.RUNTIME.run.assert_not_called()

    def test_copy_time_earlier_source_drift_cannot_become_a_new_baseline(self):
        def drift(source, target):
            if target.name == "old-check-analysis.py":
                earlier = self.source / "source-island/frozen/Cargo.toml"
                earlier.write_bytes(earlier.read_bytes() + b"\n# changed after validation\n")
        self.copy_action = drift
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual(result["native_top_level_attempts"], 0)
        self.check.RUNTIME.run.assert_not_called()

    def test_copy_time_earlier_generated_drift_cannot_become_a_new_baseline(self):
        def drift(source, target):
            if target.name == "project__correspondence.json":
                earlier = self.source / "source-island/project/Cargo.toml"
                earlier.write_bytes(earlier.read_bytes() + b"\n# changed after validation\n")
        self.copy_action = drift
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual(result["native_top_level_attempts"], 0)
        self.check.RUNTIME.run.assert_not_called()

    def test_hardlinked_original_model_is_rejected_without_overwriting_alias(self):
        alias = self.base / "model-alias"
        os.link(self.model, alias)
        before = alias.read_bytes()
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual(alias.read_bytes(), before)
        self.check.RUNTIME.run.assert_not_called()

    def test_wrong_consumer_never_launches_inspector(self):
        self.consumer = "era-constructor-codegen-v1"
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("different consumer", result["error"])
        self.check.RUNTIME.run.assert_not_called()

    def test_nonzero_native_exit_is_counted_without_dump_claim(self):
        self.exit_code = 1
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual((result["native_top_level_attempts"], result["native_top_level_returned"]), (1, 1))
        self.assertFalse(result["c_dump_generated"])
        self.assertEqual(self.admission_calls, 3)

    def test_native_exception_retains_durable_attempt_and_zero_returns(self):
        self.raise_native = True
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual((result["native_top_level_attempts"], result["native_top_level_returned"]), (1, 0))

    def test_invalid_utf8_output_is_not_inspection_material(self):
        self.output_bytes = b"bad UTF8: \xff"
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertFalse(result["c_dump_generated"])
        self.assertEqual(self.admission_calls, 3)

    def test_empty_output_is_rejected(self):
        self.output_bytes = b""
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertFalse(result["c_dump_generated"])

    def test_native_output_alias_preserves_private_outside_sentinel(self):
        sentinel = self.base / "outside-sentinel"
        sentinel.write_bytes(b"PRESERVE")
        def alias():
            output = self.output / "work/era-constructor.c"
            output.unlink()
            output.symlink_to(sentinel)
        self.native_action = alias
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual(sentinel.read_bytes(), b"PRESERVE")

    def test_post_admission_cannot_rebase_generated_dump(self):
        self.post_action = lambda: (self.output / "work/era-constructor.c").write_text("forged dump\n")
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("earlier proof input island or retained receipt changed", result["error"])

    def test_final_admission_cannot_rebase_prior_receipt(self):
        self.final_action = lambda: (self.output / "work/receipts/pre-admission.json").write_text("{}\n")
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertIn("retained receipt changed", result["final_error"])

    def test_copy_refuses_existing_alias_before_truncation(self):
        source = self.base / "source-model"
        source.write_bytes(b"known source")
        sentinel = self.base / "copy-sentinel"
        sentinel.write_bytes(b"PRESERVE")
        destination = self.base / "copy-target"
        destination.symlink_to(sentinel)
        with self.assertRaises(OSError):
            V.copy_file(source, destination, {"bytes": source.stat().st_size,
                        "sha256": V.FORMAL.digest(source.read_bytes())}, 1024)
        self.assertEqual(sentinel.read_bytes(), b"PRESERVE")

    def test_cli_has_no_source_or_extra_command_override(self):
        with patch.object(V.FORMAL, "fresh_output") as output, patch("sys.stderr", new=io.StringIO()), \
             self.assertRaises(SystemExit) as exited:
            V.main(["--output-dir", str(self.output), "--source-root", str(self.source)])
        self.assertEqual(exited.exception.code, 2)
        output.assert_not_called()

    def test_cli_cannot_create_output_inside_original_v3_evidence(self):
        with patch.object(V.FORMAL, "fresh_output") as output, patch("sys.stderr", new=io.StringIO()), \
             self.assertRaises(SystemExit) as exited:
            V.main(["--output-dir", str(V.SOURCE_ROOT / "new-child")])
        self.assertEqual(exited.exception.code, 2)
        output.assert_not_called()


if __name__ == "__main__":
    unittest.main()
