"""Closed constructor campaign controls; mock native execution, never a Rust proof."""
from __future__ import annotations

import copy
import importlib.util
import io
import json
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


V = load("era_constructor_campaign_test", HERE / "verify_era_constructor.py")
VENDOR = load("era_constructor_vendor_test", HERE / "era_vendor_admission.py")
PROJECTION = load("era_constructor_projection_test", HERE / "era_constructor_projection.py")
RESULTS = load("era_constructor_result_fixture", HERE / "test_verify.py")
POLICY = json.loads((HERE.parent.parent / "policy/ui-m0-dependencies.json").read_text())


class ConstructorCampaignTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-era-constructor-proof-test-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.root, self.output = self.base / "checkout", self.base / "campaign"
        self.root.mkdir()
        self.output.mkdir(mode=0o700)
        for name in V.input_names():
            destination = self.root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(HERE.parent.parent / name, destination)
        self.pointer = self.base / "pointer.json"
        self.pointer.write_text('{"fixture":"private tool pointer"}')
        self.pointer.chmod(0o600)
        self.installation, self.stable = self.base / "installation", self.base / "stable"
        self.payload = self.installation / "kani/payload/kani-0.68.0"
        self.compiler = self.installation / V.FORMAL.KANI_COMPILER
        for path in (self.payload / "bin", self.compiler / "bin", self.stable):
            path.mkdir(parents=True)
        for name in ("kani-driver", "goto-cc", "goto-instrument", "cbmc"):
            (self.payload / "bin" / name).write_bytes(b"fixture tool: never executed")
        for name in ("cargo", "rustc", "rustdoc"):
            (self.compiler / "bin" / name).write_bytes(b"fixture compiler: never executed")
        self.cc, self.ld = self.base / "cc", self.base / "ld"
        self.cc.write_bytes(b"fixture cc: never executed")
        self.ld.write_bytes(b"fixture ld: never executed")
        self.admitted = {"admission_profile": "kani-proof", "admission_consumer": V.CONSUMER,
            "base": self.installation, "stable": self.stable, "readonly_roots": (self.payload, self.compiler),
            "policy_sha256": "a" * 64, "inventories": [{"name": "kani", "sha256": "b" * 64, "entries": 7}],
            "installation_pointer": str(self.pointer), "installation_pointer_sha256": V.FORMAL.digest(self.pointer.read_bytes()),
            "policy": {"tools": {"kani": {"entrypoint": V.FORMAL.KANI_ENTRYPOINT, "toolchain": V.FORMAL.KANI_COMPILER}}}}
        self.graph = [{"name": name, "version": version, "expected_features":
            POLICY["rows"]["x86_64-unknown-linux-gnu"]["features"][name + "@" + version]}
            for name, version in PROJECTION.GRAPH_VERSIONS.items()]
        self.schedule = PROJECTION.schedule()
        self.consumer = V.CONSUMER
        self.check = self.frozen_consumer = None
        self.native_calls = self.tool_calls = 0
        self.native_action = self.tool_action = None
        self.bad_export_index = self.fail_unwind_index = self.override_index = None
        self.raise_native = False
        self.vendor = SimpleNamespace(input_names=VENDOR.input_names, prepare=self.prepare_vendor,
            admit=self.admit_vendor, identity=lambda value: value)
        self.projection = SimpleNamespace(schedule=lambda: copy.deepcopy(self.schedule),
            prepare_era_constructor=self.prepare_project)

    def admit(self, root):
        self.tool_calls += 1
        if self.tool_action:
            self.tool_action(self.tool_calls)
        value = copy.deepcopy(self.admitted)
        value["admission_consumer"] = self.consumer
        return value

    def prepare_vendor(self, root, bundle, vendor_root=None, *, work_root=None):
        bundle.mkdir()
        (bundle / "vendor").mkdir()
        (bundle / "cargo-config.toml").write_text('[net]\noffline = true\n')
        (bundle / "manifest.json").write_text('{"fixture":"fourteen admitted sources"}\n')

    def admit_vendor(self, root, bundle, *, work_root=None):
        return {"readonly_root": bundle, "vendor_root": bundle / "vendor", "config_path": bundle / "cargo-config.toml",
                "manifest_path": bundle / "manifest.json", "inventory": self.check.input_snapshot(self.output, [bundle])}

    def prepare_project(self, root, project, case, harness_path, *, vendor_root=None):
        project.mkdir(parents=True)
        (project / "src").mkdir()
        (project / ".cargo").mkdir()
        data = {"Cargo.toml": b'[package]\nname="fixture"\nversion="0.0.0"\nedition="2024"\n',
                "Cargo.lock": b"version = 4\n", "src/proof.rs": b"// mocked proof input; never compiled\n",
                "src/era.rs": ("// fixed constructor source case: " + case + "\n").encode(),
                ".cargo/config.toml": (vendor_root.parent / "cargo-config.toml").read_bytes()}
        for name, value in data.items():
            (project / name).write_bytes(value)
        correspondence = b'{"fixture":"source identity, not a mathematical proof"}\n'
        (project / "correspondence.json").write_bytes(correspondence)
        return {"entrypoint": project / "Cargo.toml", "registry_graph": self.graph,
                "generated_files_sha256": {name: V.FORMAL.digest(value) for name, value in data.items()},
                "correspondence_sha256": V.FORMAL.digest(correspondence),
                "compiled_era_sha256": V.FORMAL.digest(data["src/era.rs"])}

    def report(self, job, project, target):
        healthy = job["case"] == "healthy"
        checks = [{"category": "assertion", "description": name, "status": "Success",
                   "location": {"file": "src/proof.rs"}} for name in
                  PROJECTION.REQUIRED_ASSERTIONS[job["harness"]]] if healthy else [
            {"category": "assertion", "description": job["intended_failure"], "status": "Failure",
             "location": {"file": "src/proof.rs"}}]
        if healthy:
            checks.extend({"category": "cover", "description": name, "status": "Satisfied"}
                          for name in PROJECTION.REQUIRED_COVERS[job["harness"]])
        checks.extend({"category": name, "description": name + " obligation", "status": "Success"}
                      for name in ("unwinding", "overflow", "memory-safety", "sanity"))
        value = RESULTS.result(job, checks, healthy)
        h = value["harness_metadata"][0]
        h.update(crate_name=V.CRATE, is_automatically_generated=False, is_bounded=False, is_ctor_based=False)
        h["source"]["file"] = "src/proof.rs"
        value["project"] = {"workspace_root": str(project), "crate_name": [V.CRATE],
                            "output_dir": "/work/" + target.relative_to(self.output / "work").as_posix() + "/kani/out"}
        return value

    def native(self, root, work, command, **options):
        self.native_calls += 1
        index = self.native_calls
        job, project = self.schedule[index - 1], options["cwd"]
        self.assertEqual(json.loads((work / f"jobs/{index:02d}/execution.json").read_text())["native_top_level_attempts"], 1)
        if self.raise_native:
            raise OSError("synthetic constructor launch failure")
        self.check.RUNTIME._sealed_inputs(work, options["sealed"])
        target = work / f"targets/{index:02d}"
        modeldir = target / "kani/x86_64-unknown-linux-gnu/debug/build/nevrotick-era-constructor-proof/0123456789abcdef/out"
        modeldir.mkdir(parents=True)
        stem = modeldir / "selected"
        for suffix in (".symtab.out", ".out"):
            Path(str(stem) + suffix).write_bytes(b"mocked compiler model: never executed")
        for suffix in (".type_map.json", ".pretty_name_map.json"):
            Path(str(stem) + suffix).write_text('{"fixture":"inspection map"}\n')
        h = {"pretty_name": job["harness"], "crate_name": V.CRATE, "original_file": "src/proof.rs",
             "goto_file": "/work/" + Path(str(stem) + ".symtab.out").relative_to(work).as_posix(),
             "attributes": {"kind": "Proof", "should_panic": False, "solver": None,
                            "unwind_value": None, "stubs": [], "verified_stubs": []},
             "contract": None, "has_loop_contracts": False, "is_automatically_generated": False,
             "is_bounded": False, "is_ctor_based": False}
        if index == self.override_index:
            h["attributes"]["unwind_value"] = 1
        metadata = {"crate_name": V.CRATE, "proof_harnesses": [h], "unsupported_features": [],
                    "test_harnesses": [], "contracted_functions": [], "autoharness_md": None}
        (modeldir / "selected.kani-metadata.json").write_text(json.dumps(metadata))
        for package in self.graph:
            host = package["name"] in {"garde_derive", "proc-macro2", "quote", "rustversion", "syn", "unicode-ident"}
            prefix = "kani/debug/build" if host else "kani/x86_64-unknown-linux-gnu/debug/build"
            folder = target / prefix / package["name"] / "0123456789abcdef/fingerprint"
            folder.mkdir(parents=True)
            (folder / ("lib-" + package["name"].replace("-", "_") + ".json")).write_text(
                json.dumps({"features": json.dumps(package["expected_features"])}))
        report = self.report(job, project, target)
        if index == self.fail_unwind_index:
            report["verification_results"]["results"][0]["checks"].append(
                {"category": "unwinding", "description": "actual loop unwinding incomplete", "status": "Failure"})
            report["verification_results"]["results"][0]["status"] = "Failure"
            report["verification_results"]["summary"].update(successful=0, failed=1)
            report["error_details"][0]["has_errors"] = True
            RESULTS.recount(report)
        export = work / f"jobs/{index:02d}/evidence/kani.json"
        export.write_text("{}\n" if index == self.bad_export_index else json.dumps(report))
        if self.native_action:
            self.native_action(index)
        exit_code = int(job["case"] != "healthy" or index == self.fail_unwind_index)
        return {"exit_code": exit_code, "failure": None, "output": "mocked native constructor proof seam"}

    def execute(self):
        def local_loader(name, path):
            if path.name == "verify_era_constructor.py":
                value = load(name, path)
                self.frozen_consumer = value
                value.POINTER = self.pointer
                mocked = patch.object(value, "load", side_effect=local_loader)
                mocked.start()
                self.addCleanup(mocked.stop)
                return value
            if path.name == "era_vendor_admission.py":
                return self.vendor
            if path.name == "era_constructor_projection.py":
                return self.projection
            if path.name == "check-analysis.py":
                value = load(name, path)
                if path != self.root / "scripts/check-analysis.py":
                    self.check = value
                    value.admit_era_constructor_proof = Mock(side_effect=self.admit)
                    value.admit_kani_proof = Mock()
                    value.RUNTIME.run = Mock(side_effect=self.native)
                    value.RUNTIME._helpers = Mock(return_value={"cc": str(self.cc), "ld": str(self.ld)})
                return value
            self.fail("Unexpected controller load: " + str(path))

        with patch.object(V, "ROOT", self.root), patch.object(V.FORMAL, "ROOT", self.root), \
             patch.object(V, "load", side_effect=local_loader):
            return V.run(self.output)

    def test_five_fixed_commands_qualify_healthy_defects_restoration_without_reuse(self):
        result = self.execute()
        self.assertEqual(result["status"], "passed", result.get("error"))
        self.assertTrue(result["proof_qualified"])
        self.assertEqual((result["native_top_level_attempts"], result["native_top_level_returned"]), (5, 5))
        self.assertEqual(self.tool_calls, 11)
        self.assertFalse(result["compiler_cache_reused"])
        self.assertTrue(all(job["expected_outcome_qualified"] for job in result["jobs"]))
        self.assertTrue(result["all_required_evidence_unchanged"])
        self.check.admit_kani_proof.assert_not_called()
        calls = self.check.RUNTIME.run.call_args_list
        self.assertEqual(len({call.args[2][-1] for call in calls}), 5)
        for index, call in enumerate(calls, 1):
            argv = call.args[2]
            self.assertEqual(call.args[0], self.output / "source-island/frozen")
            self.assertEqual(argv[1], "kani")
            self.assertEqual(argv[argv.index("--default-unwind") + 1], "32")
            self.assertEqual(argv[argv.index("--solver") + 1], "cadical")
            self.assertIn("--run-sanity-checks", argv)
            self.assertIn("--exact", argv)
            self.assertNotIn("--only-codegen", argv)
            self.assertNotIn("RUSTFLAGS", call.kwargs["environment"])
            self.assertEqual((call.kwargs["timeout"], call.kwargs["cpu_seconds"], call.kwargs["process_limit"]), (180, 180, 128))
            if index > 1:
                self.assertIn(self.output / f"work/jobs/{index-1:02d}", call.kwargs["sealed"])

    def test_first_healthy_unwinding_failure_stops_without_retry_or_weakened_checks(self):
        self.fail_unwind_index = 1
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertFalse(result["proof_qualified"])
        self.assertEqual(self.native_calls, 1)
        self.assertEqual(result["native_top_level_attempts"], 1)
        self.assertIn("Healthy proof", result["jobs"][0]["qualification_error"])

    def test_control_must_fail_only_at_its_intended_assertion(self):
        self.fail_unwind_index = 2
        result = self.execute()
        self.assertEqual(self.native_calls, 2)
        self.assertFalse(result["jobs"][-1]["expected_outcome_qualified"])
        self.assertIn("exclusively at its intended assertion", result["jobs"][-1]["qualification_error"])

    def test_raw_compiler_unwind_override_is_rejected(self):
        self.override_index = 2
        result = self.execute()
        self.assertEqual(self.native_calls, 2)
        self.assertIn("attributes/source override", result["jobs"][-1]["qualification_error"])

    def test_schema_failure_retains_raw_export_and_whole_target_without_rerun(self):
        self.bad_export_index = 1
        result = self.execute()
        self.assertEqual(self.native_calls, 1)
        self.assertFalse(result["proof_qualified"])
        raw = json.loads((self.output / "work/jobs/01/raw-custody.json").read_text())
        key = "work/jobs/01/evidence/kani.json"
        self.assertEqual(raw["snapshot"][key]["sha256"], V.FORMAL.digest(b"{}\n"))
        self.assertTrue(any(name.endswith(".kani-metadata.json") for name in raw["snapshot"]))
        self.assertTrue(any(name.endswith(".symtab.out") for name in raw["snapshot"]))
        self.assertNotIn("final_error", result)

    def test_post_admission_cannot_rebase_raw_failed_schema_data(self):
        self.bad_export_index = 1
        def change(count):
            if count == 2:
                (self.output / "work/jobs/01/evidence/kani.json").write_text('{"changed":true}')
        self.tool_action = change
        result = self.execute()
        self.assertEqual(self.native_calls, 1)
        self.assertIn("Raw constructor output changed", result["jobs"][0]["qualification_error"])
        self.assertIn("final_error", result)

    def test_later_command_cannot_change_prior_qualified_export(self):
        def change(index):
            if index == 2:
                (self.output / "work/jobs/01/evidence/kani.json").write_text("{}\n")
        self.native_action = change
        result = self.execute()
        self.assertEqual(self.native_calls, 2)
        self.assertFalse(result["proof_qualified"])
        self.assertIn("retained receipt changed", result["jobs"][-1]["qualification_error"])

    def test_pre_admission_drift_stops_before_the_next_native_command(self):
        def change(count):
            if count == 3:
                (self.output / "work/jobs/01/evidence/kani.json").write_text("{}\n")
        self.tool_action = change
        result = self.execute()
        self.assertEqual(self.native_calls, 1)
        self.assertEqual(result["native_top_level_attempts"], 1)
        self.assertEqual(result["jobs"][-1]["native_top_level_attempts"], 0)
        self.assertIn("retained receipt changed", result["jobs"][-1]["qualification_error"])

    def test_wrong_role_never_reaches_native(self):
        self.consumer = "era-constructor-codegen-v1"
        result = self.execute()
        self.assertEqual(self.native_calls, 0)
        self.assertFalse(result["proof_qualified"])

    def test_earlier_job_cannot_seed_a_future_cold_target(self):
        def seed(index):
            if index == 1:
                target = self.output / "work/targets/02"
                target.mkdir(parents=True)
                (target / "prior-command-cache").write_text("unadmitted previous command output\n")
        self.native_action = seed
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual(self.native_calls, 1)
        self.assertEqual(result["native_top_level_attempts"], 1)
        self.assertIn("target must be absent", result["jobs"][-1]["qualification_error"])

    def test_admission_cannot_seed_the_current_cold_target(self):
        def seed(count):
            if count == 1:
                target = self.output / "work/targets/01"
                target.mkdir(parents=True)
                (target / "admission-cache").write_text("unadmitted admission output\n")
        self.tool_action = seed
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual(self.native_calls, 0)
        self.assertEqual(result["native_top_level_attempts"], 0)
        self.assertIn("target must be absent", result["jobs"][-1]["qualification_error"])

    def test_admission_cannot_hide_a_cold_target_as_a_dangling_symlink(self):
        def seed(count):
            if count == 1:
                target = self.output / "work/targets/01"
                target.parent.mkdir(parents=True)
                target.symlink_to(self.base / "absent-target", target_is_directory=True)
                self.assertFalse(target.exists())
                self.assertTrue(target.is_symlink())
        self.tool_action = seed
        result = self.execute()
        self.assertEqual(result["status"], "incomplete")
        self.assertEqual(self.native_calls, 0)
        self.assertEqual(result["native_top_level_attempts"], 0)
        self.assertIn("target must be absent", result["jobs"][-1]["qualification_error"])

    def test_native_seam_exception_keeps_durable_attempt_and_zero_returns(self):
        self.raise_native = True
        result = self.execute()
        self.assertEqual((result["native_top_level_attempts"], result["native_top_level_returned"]), (1, 0))
        self.assertEqual(len(result["jobs"]), 1)

    def test_source_closure_includes_frozen_contract_registry(self):
        self.assertIn("tools/formal/additional_suites.py", V.input_names())
        self.assertIn("tools/formal/test_additional_suites.py", V.input_names())

    def test_cli_cannot_override_unwind_solver_or_job_selection(self):
        for option in (("--unwind", "1"), ("--solver", "z3"), ("--only-jobs", "1")):
            with self.subTest(option=option), patch.object(V.FORMAL, "fresh_output") as output, \
                 patch("sys.stderr", new=io.StringIO()), self.assertRaises(SystemExit) as exited:
                V.main(["--output-dir", str(self.output), *option])
            self.assertEqual(exited.exception.code, 2)
            output.assert_not_called()


if __name__ == "__main__":
    unittest.main()
