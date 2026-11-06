"""Adversarial composition admission controls; execute no compiler or solver."""
from __future__ import annotations

import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch


def load(name, filename):
    spec = importlib.util.spec_from_file_location(name, Path(__file__).with_name(filename))
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


VERIFY = load("composition_admission_tests", "verify_fraction_composition.py")
FIXTURE = load("composition_json_fixture", "test_verify.py")
CHECK = VERIFY.module("composition_receipt_reader", VERIFY.ROOT / "scripts/check-analysis.py")
ENTRY = "/work/jobs/14-addition_healthy/inputs/proof.rs"


def observed(job, entry=ENTRY):
    """Build independent schema records, preserving original registered obligations."""
    if job["case"] == "healthy":
        assertions, covers = VERIFY.BASE.additional_suites().expected_contract(job["subject"], job["harness"])
        labels = sorted(set(assertions) | set(job["required_refinement_assertions"]))
        checks = [{"category": "assertion", "description": '"' + name + '"',
                   "status": "Success", "location": {"file": entry}} for name in labels]
        checks += [{"category": "cover", "description": name, "status": "Satisfied"} for name in covers]
        success = True
    else:
        checks = [{"category": "assertion", "description": '"' + job["intended_failure"] + '"',
                   "status": "Failure", "location": {"file": entry}}]
        success = False
    report = FIXTURE.result(job, checks, success)
    exported = report["harness_metadata"][0]
    exported.update(source={"file": entry}, contract={"contracted_function_name": None, "recursion_tracker": None},
                    is_automatically_generated=False, is_bounded=False, is_ctor_based=False)
    attributes = {"kind": "Proof", "should_panic": False, "solver": None, "unwind_value": None,
                  "stubs": copy.deepcopy(job["expected_stubs"]), "verified_stubs": []}
    compiler_harness = {"pretty_name": job["harness"], "original_file": entry,
                        "crate_name": "proof", "attributes": attributes, "contract": None,
                        "has_loop_contracts": False, "is_automatically_generated": False,
                        "is_bounded": False, "is_ctor_based": False}
    compiler = {"crate_name": "proof", "proof_harnesses": [compiler_harness],
                "unsupported_features": [], "test_harnesses": [], "contracted_functions": []}
    output = "Checking harness " + job["harness"] + "...\n"
    output += "".join("  - Stub: " + pair["original"] + " -> " + pair["replacement"] + "\n" for pair in job["expected_stubs"])
    return report, compiler, {"exit_code": 0 if success else 1, "failure": None, "output": output}


class ClosedCardTests(unittest.TestCase):
    def test_all_source_inputs_are_real_repository_files_and_include_consumer_delta(self):
        names = VERIFY.input_names()
        self.assertTrue(set(VERIFY.SOURCE_NAMES).issubset(names))
        self.assertIn("scripts/tests/test_check_analysis.py", names)
        self.assertNotIn("proof.rs", names)
        for name in names:
            self.assertTrue((VERIFY.ROOT / name).is_file(), name)
            self.assertEqual(VERIFY.ROOT / name, (VERIFY.ROOT / name).resolve(strict=True))

    def test_exact_eighteen_jobs_callee_no_stubs_and_original_addition_contract(self):
        jobs = VERIFY.schedule()
        self.assertEqual(tuple(j["id"] for _, j in jobs), VERIFY.JOB_IDS)
        for index in (5, 8, 11):
            self.assertEqual(jobs[index - 1][1]["expected_stubs"], [])
            self.assertEqual(len(jobs[index - 1][1]["required_refinement_assertions"]), 2)
        assertions, covers = VERIFY.BASE.additional_suites().expected_contract("fraction_addition", jobs[13][1]["harness"])
        self.assertEqual(len(assertions), 11)
        self.assertEqual(len(covers), 23)
        for change in ("missing", "reordered", "extended", "callee_stubs", "dependencies", "old_domain", "original_fault"):
            changed = copy.deepcopy([j for _, j in jobs])
            if change == "missing": changed.pop()
            elif change == "reordered": changed[0], changed[1] = changed[1], changed[0]
            elif change == "extended": changed.append(copy.deepcopy(changed[-1]))
            elif change == "callee_stubs": changed[4]["expected_stubs"] = changed[13]["expected_stubs"]
            elif change == "dependencies": changed[13]["dependencies"].pop()
            elif change == "old_domain": changed[4]["harness"] = "narrowed_constructor"
            else: changed[15]["case"] = "different_fault"
            with self.subTest(change=change), patch.object(VERIFY, "projection", return_value=SimpleNamespace(schedule=lambda: changed)):
                with self.assertRaises(VERIFY.Incomplete): VERIFY.schedule()

    def test_flags_preserve_ordinary_defaults_and_only_add_stubbing_and_retention(self):
        baseline = list(VERIFY.BASE.KANI_BASELINE_FLAGS)
        check = SimpleNamespace(PROFILES={"kani_flags": baseline})
        self.assertEqual(VERIFY.composition_flags(check), baseline + ["-Z", "stubbing", "--keep-temps"])
        self.assertEqual(check.PROFILES["kani_flags"], baseline)
        for modified in (baseline + ["-Z", "function-contracts"], baseline + ["--keep-temps"], baseline[:-1] + ["300s"]):
            check.PROFILES["kani_flags"] = modified
            with self.assertRaises(VERIFY.Incomplete): VERIFY.composition_flags(check)

    def test_deadline_boot_and_wall_bookends_cannot_be_reset(self):
        start, wall = VERIFY.PREPARATION_STARTED_MONOTONIC_NS, VERIFY.PREPARATION_STARTED_UNIX_NS
        with patch.object(Path, "read_text", return_value=VERIFY.PREPARATION_BOOT_ID), \
             patch.object(VERIFY.time, "monotonic_ns", return_value=start + 10**9), \
             patch.object(VERIFY.time, "time_ns", return_value=wall + 2 * 10**9):
            self.assertEqual(VERIFY.remaining_budget(), 3598)
        for boot, mono, unix in (("another boot", start, wall),
                                 (VERIFY.PREPARATION_BOOT_ID, start - 1, wall),
                                 (VERIFY.PREPARATION_BOOT_ID, start, wall - 1),
                                 (VERIFY.PREPARATION_BOOT_ID, VERIFY.DEADLINE_MONOTONIC_NS, wall),
                                 (VERIFY.PREPARATION_BOOT_ID, start, wall + 3600 * 10**9)):
            with self.subTest(boot=boot, mono=mono, unix=unix), patch.object(Path, "read_text", return_value=boot), \
                 patch.object(VERIFY.time, "monotonic_ns", return_value=mono), \
                 patch.object(VERIFY.time, "time_ns", return_value=unix):
                with self.assertRaises(VERIFY.Incomplete): VERIFY.remaining_budget()

    def test_output_and_cli_deny_relaunch_partial_jobs_and_resource_overrides_before_creation(self):
        destination = str(Path.home() / ".local/share/nevrotick" / VERIFY.OUTPUT_LEAF)
        with patch.object(VERIFY, "remaining_budget", return_value=100), patch.object(Path, "exists", return_value=False), patch.object(Path, "is_symlink", return_value=False):
            self.assertEqual(VERIFY.output_preflight(destination), Path(destination))
            for changed in (destination + "-retry", "/tmp/" + VERIFY.OUTPUT_LEAF):
                with self.assertRaises(VERIFY.Incomplete): VERIFY.output_preflight(changed)
        with patch.object(VERIFY, "remaining_budget", return_value=100), patch.object(Path, "exists", return_value=True):
            with self.assertRaises(VERIFY.Incomplete): VERIFY.output_preflight(destination)
        for override in (["--only-jobs", "14"], ["--start-at", "14"], ["--solver", "z3"], ["--timeout", "300"]):
            with patch.object(VERIFY.BASE, "fresh_output") as fresh, patch("sys.stderr"):
                with self.assertRaises(SystemExit): VERIFY.main(["--output-dir", destination, *override])
                fresh.assert_not_called()


class MetadataAndProofTests(unittest.TestCase):
    def setUp(self):
        self.jobs = VERIFY.schedule()
        self.job = self.jobs[13][1]

    def qualify(self, values, job=None):
        return VERIFY.qualify(*values, job or self.job, ENTRY)

    def test_healthy_callees_addition_and_mechanism_plus_all_negative_controls_qualify(self):
        for _, job in self.jobs:
            with self.subTest(job=job["id"]):
                self.assertGreater(self.qualify(observed(job), job)["checks"], 0)

    def test_token_parser_accepts_separator_spacing_but_rejects_split_names_aliases_and_extra_syntax(self):
        self.assertEqual(VERIFY.token_path(" crate :: fraction_arithmetic :: Fraction :: new "), "crate::fraction_arithmetic::Fraction::new")
        for value in ("Fr action::new", "Fraction::n ew", "Fraction/*x*/::new", "Fraction::<u8>::new", "::Fraction::new", "Fraction::new\n", "Fraction: :new", None, 1):
            with self.subTest(value=value), self.assertRaises(VERIFY.Incomplete): VERIFY.token_path(value)
        for pairs in ([{"original": "A::f", "replacement": "B::f"}] * 2,
                      [{"original": "A::f", "replacement": "B::f"}, {"original": "A::f", "replacement": "C::f"}],
                      [{"original": "A::f", "replacement": "B::f", "ignored": True}], "A -> B"):
            with self.assertRaises(VERIFY.Incomplete): VERIFY.stub_pairs(pairs)

    def test_compiler_and_stdout_pairs_both_must_match_exact_count_and_targets(self):
        for change in ("compiler_missing", "compiler_extra", "compiler_duplicate", "wrong_target", "wrong_replacement", "split_identifier", "stdout_missing", "stdout_extra", "verified_stub", "wrong_header", "thread_header"):
            values = observed(self.job)
            report, compiler, native = values
            pairs = compiler["proof_harnesses"][0]["attributes"]["stubs"]
            if change == "compiler_missing": pairs.pop()
            elif change == "compiler_extra": pairs.append({"original": "Other::f", "replacement": "Other::model"})
            elif change == "compiler_duplicate": pairs.append(copy.deepcopy(pairs[0]))
            elif change == "wrong_target": pairs[0]["original"] = "Alias::new"
            elif change == "wrong_replacement": pairs[0]["replacement"] += "_other"
            elif change == "split_identifier": pairs[0]["original"] = pairs[0]["original"].replace("Fraction", "Fr action")
            elif change == "stdout_missing": native["output"] = native["output"].splitlines()[0] + "\n"
            elif change == "stdout_extra": native["output"] += "  - Stub: Other::f -> Other::model\n"
            elif change == "verified_stub": native["output"] += "  - Verified stub: A::f\n"
            elif change == "wrong_header": native["output"] = native["output"].replace(self.job["harness"], "other_harness")
            else: native["output"] = "Thread 0: " + native["output"]
            with self.subTest(change=change), self.assertRaises(VERIFY.Incomplete): self.qualify(values)
        values = observed(self.job)
        for pair in values[1]["proof_harnesses"][0]["attributes"]["stubs"]:
            for key in pair: pair[key] = pair[key].replace("::", " :: ")
        self.qualify(values)

    def test_unselected_compiler_harness_is_harmless_but_duplicate_selected_or_wrong_source_fails(self):
        values = observed(self.job)
        extra = copy.deepcopy(values[1]["proof_harnesses"][0])
        extra["pretty_name"] = "fraction_arithmetic::unselected_original_control"
        values[1]["proof_harnesses"].append(extra)
        self.qualify(values)
        for change in ("duplicate", "wrong_file", "wrong_kind", "missing_pair_array", "hidden_contract", "bounded", "unsupported"):
            values = observed(self.job)
            selected = values[1]["proof_harnesses"][0]
            if change == "duplicate": values[1]["proof_harnesses"].append(copy.deepcopy(selected))
            elif change == "wrong_file": selected["original_file"] = "/work/other.rs"
            elif change == "wrong_kind": selected["attributes"]["kind"] = "ProofForContract"
            elif change == "missing_pair_array": del selected["attributes"]["stubs"]
            elif change == "hidden_contract": selected["contract"] = {"contracted_function_name": "hidden"}
            elif change == "bounded": selected["is_bounded"] = True
            else: values[1]["unsupported_features"] = [{"feature": "unsupported selected call"}]
            with self.subTest(change=change), self.assertRaises(VERIFY.Incomplete): self.qualify(values)

    def test_export_cannot_invent_stub_metadata_or_hide_nonordinary_execution(self):
        for change in ("fake_stub_export", "contract", "multiple", "bounded", "missing_failure", "timeout", "missing_output"):
            values = observed(self.job)
            exported = values[0]["harness_metadata"][0]
            if change == "fake_stub_export": exported["attributes"]["stubs"] = self.job["expected_stubs"]
            elif change == "contract": exported["contract"]["contracted_function_name"] = "hidden"
            elif change == "multiple": values[0]["harness_metadata"].append(copy.deepcopy(exported))
            elif change == "bounded": exported["is_bounded"] = True
            elif change == "missing_failure": del values[2]["failure"]
            elif change == "timeout": values[2]["failure"] = "wall timeout"
            else: del values[2]["output"]
            with self.subTest(change=change), self.assertRaises(VERIFY.Incomplete): self.qualify(values)

    def test_every_post_seed_and_original_label_must_survive_successfully_with_all_witnesses(self):
        for index in (5, 8, 11, 14):
            job = self.jobs[index - 1][1]
            baseline = observed(job)
            checks = baseline[0]["verification_results"]["results"][0]["checks"]
            targets = [i for i, check in enumerate(checks) if check["category"] == "cover" or check["description"].strip('"') in job["required_refinement_assertions"]]
            for position in targets:
                for change in ("missing", "unreachable", "wrong_source"):
                    values = copy.deepcopy(baseline)
                    changed = values[0]["verification_results"]["results"][0]["checks"]
                    if change == "missing": del changed[position]
                    elif change == "unreachable": changed[position]["status"] = "Unreachable"
                    else:
                        if changed[position]["category"] == "cover": continue
                        changed[position]["location"]["file"] = "/work/other.rs"
                    FIXTURE.recount(values[0])
                    with self.subTest(job=job["id"], position=position, change=change), self.assertRaises(VERIFY.Incomplete): self.qualify(values, job)

    def test_precondition_control_green_or_unrelated_failure_is_not_accepted(self):
        for index in (2, 12, 13):
            job = self.jobs[index - 1][1]
            for change in ("green", "unrelated", "cover", "wrong_location"):
                values = observed(job)
                check = values[0]["verification_results"]["results"][0]["checks"][0]
                if change == "green": values[2]["exit_code"] = 0
                elif change == "unrelated": check["description"] = '"unrelated panic"'
                elif change == "cover": check["category"] = "cover"
                else: check["location"]["file"] = "/work/other.rs"
                FIXTURE.recount(values[0])
                with self.subTest(job=job["id"], change=change), self.assertRaises(VERIFY.Incomplete): self.qualify(values, job)


class CertificateAndCustodyTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-composition-gates-")
        self.addCleanup(self.temporary.cleanup)
        self.work = Path(self.temporary.name)
        self.source = VERIFY.ROOT
        self.jobs = VERIFY.schedule()
        self.bindings = {key: hashlib.sha256(key.encode()).hexdigest() for key in VERIFY.CALLEES}
        self.admitted_record = {"consumer": VERIFY.CONSUMER, "seconds": 0.01, "readonly_roots": ["/admitted/kani", "/admitted/compiler"],
            "installation_pointer_sha256": "pointer", "kani_executable_sha256": "kani", "compiler_rustc_sha256": "rustc",
            "compiler_rustdoc_sha256": "rustdoc", "profile": {"flags": "fixed"},
            "admitted": {"base": "/admitted", "stable": "/stable", "policy_sha256": "policy", "inventories": ["complete"]}}
        self.identity = VERIFY.admission_identity(self.admitted_record)
        self.certificates = {}
        for key in VERIFY.CALLEES:
            index, job = next((i, j) for i, j in self.jobs if j["id"] == key)
            folder = VERIFY.job_folder(self.work, index, job)
            inputs = folder / "inputs"
            inputs.mkdir(parents=True)
            generated = {}
            for name in VERIFY.GENERATED_NAMES:
                content = ("synthetic controller-test receipt input " + name).encode()
                (inputs / name).write_bytes(content)
                generated[name] = hashlib.sha256(content).hexdigest()
            before = CHECK.input_snapshot(self.work, [inputs])
            report, compiler, native = observed(job, "/work/" + (inputs / "proof.rs").relative_to(self.work).as_posix())
            prepared = {"callee_bindings": self.bindings, "input_files_sha256": generated,
                        "generated_input_files": list(VERIFY.GENERATED_NAMES)}
            record = {"index": index, "job": job, "consumer": VERIFY.CONSUMER,
                      "expected_outcome_qualified": True, "input_unchanged": True,
                      "input_before": before, "input_after": before, "native": native,
                      "tool_identity": self.identity, "preparation": prepared}
            for name, value in (("input-before.json", before), ("input-after.json", before),
                                ("admission.json", self.admitted_record), ("kani.json", report),
                                ("compiler-metadata.json", compiler), ("execution.json", record)):
                VERIFY.BASE.write_json(folder / name, value)
            self.certificates[key] = VERIFY.issue_certificate(CHECK, self.work, self.source, key, self.identity)
        self.job = self.jobs[13][1]
        self.prepared = {"callee_bindings": self.bindings}

    def require(self, certificates=None):
        return VERIFY.require_callees(CHECK, self.work, self.source, self.job, self.prepared,
                                      self.certificates if certificates is None else certificates, self.identity)

    def folder(self, key="constructor"):
        index, job = next((i, j) for i, j in self.jobs if j["id"] == key)
        return VERIFY.job_folder(self.work, index, job)

    def test_three_current_raw_qualified_callees_are_accepted_controls_cannot_issue(self):
        self.require()
        for key in ("constructor_bad_post", "comparison_wrong_seed", "addition_healthy", "mech_healthy"):
            with self.subTest(key=key), self.assertRaises(VERIFY.Incomplete):
                VERIFY.issue_certificate(CHECK, self.work, self.source, key, self.identity)

    def test_caller_refuses_missing_extra_negative_stale_or_wrong_schema_certificates(self):
        for change in ("missing", "extra", "negative", "tool", "body", "schema", "summary_flag", "snapshot"):
            certificates = copy.deepcopy(self.certificates)
            if change == "missing": del certificates["comparison"]
            elif change == "extra": certificates["negative_control"] = copy.deepcopy(certificates["constructor"])
            elif change == "negative": certificates["constructor"]["key"] = "constructor_bad_post"
            elif change == "tool": certificates["constructor"]["tool_identity"] = {"different": "tool"}
            elif change == "body": certificates["constructor"]["binding"] = "0" * 64
            elif change == "schema": certificates["constructor"]["schema_version"] = True
            elif change == "summary_flag": certificates["constructor"]["qualified"] = True
            else: certificates["constructor"]["receipt_snapshot"] = {}
            with self.subTest(change=change), self.assertRaises(VERIFY.Incomplete): self.require(certificates)
        self.require()

    def test_actual_receipt_mutation_invalidates_certificate_without_rebasing(self):
        for relative in ("inputs/proof.rs", "inputs/fraction_composition_spec.rs", "kani.json", "compiler-metadata.json", "execution.json", "admission.json"):
            path = self.folder() / relative
            content = path.read_bytes()
            path.write_bytes(content + b"\nchanged")
            with self.subTest(relative=relative), self.assertRaises(VERIFY.Incomplete): self.require()
            path.write_bytes(content)
        self.require()

    def test_forged_success_summary_and_rebased_snapshot_cannot_hide_raw_missing_post_or_seed(self):
        for key in VERIFY.CALLEES:
            folder = self.folder(key)
            path = folder / "kani.json"
            original = path.read_bytes()
            job = next(j for _, j in self.jobs if j["id"] == key)
            for label in job["required_refinement_assertions"]:
                report = json.loads(original)
                checks = report["verification_results"]["results"][0]["checks"]
                checks[:] = [item for item in checks if item["description"].strip('"') != label]
                FIXTURE.recount(report)
                VERIFY.BASE.write_json(path, report)
                certificates = copy.deepcopy(self.certificates)
                certificates[key]["receipt_snapshot"] = CHECK.input_snapshot(self.work, VERIFY.receipt_roots(folder))
                with self.subTest(key=key, label=label), self.assertRaises(VERIFY.Incomplete): self.require(certificates)
                with self.assertRaises(VERIFY.Incomplete): VERIFY.issue_certificate(CHECK, self.work, self.source, key, self.identity)
            path.write_bytes(original)
        self.require()

    def test_record_summaries_cannot_relabel_negative_empty_or_mismatched_native_facts(self):
        path = self.folder() / "execution.json"
        original = path.read_bytes()
        for change in ("negative_job", "wrong_identity", "failed_native", "timeout", "no_failure_field", "wrong_bookend", "malformed_manifest"):
            record = json.loads(original)
            if change == "negative_job": record["job"]["case"] = "bad_post"
            elif change == "wrong_identity": record["tool_identity"] = {"different": "tool"}
            elif change == "failed_native": record["native"]["exit_code"] = 1
            elif change == "timeout": record["native"]["failure"] = "timeout"
            elif change == "no_failure_field": del record["native"]["failure"]
            elif change == "wrong_bookend": record["input_after"] = {}
            else: record["preparation"]["input_files_sha256"].pop("correspondence.json")
            VERIFY.BASE.write_json(path, record)
            certificates = copy.deepcopy(self.certificates)
            certificates["constructor"]["receipt_snapshot"] = CHECK.input_snapshot(self.work, VERIFY.receipt_roots(self.folder()))
            with self.subTest(change=change), self.assertRaises(VERIFY.Incomplete): self.require(certificates)
            path.write_bytes(original)
        self.require()

    def test_complete_generated_restoration_covers_specs_correspondence_and_frozen_tools(self):
        original, restored = self.work / "original", self.work / "restored"
        for base in (original, restored):
            (base / "frozen").mkdir(parents=True)
            for name in (*VERIFY.GENERATED_NAMES, "frozen/controller.py"):
                (base / name).write_bytes(("identical " + name).encode())
        expected = VERIFY.BASE.generated_identity(CHECK, original)
        self.assertEqual(expected, VERIFY.BASE.generated_identity(CHECK, restored))
        for relative in (*VERIFY.GENERATED_NAMES, "frozen/controller.py"):
            path = restored / relative
            content = path.read_bytes()
            path.write_bytes(content + b"changed")
            self.assertNotEqual(expected, VERIFY.BASE.generated_identity(CHECK, restored), relative)
            path.write_bytes(content)
        (restored / "extra.rs").write_bytes(b"unexpected")
        self.assertNotEqual(expected, VERIFY.BASE.generated_identity(CHECK, restored))

    def test_compiler_metadata_is_copied_byte_exactly_and_missing_redirected_or_reused_receipt_is_rejected(self):
        folder = self.work / "metadata-receipt"
        (folder / "target").mkdir(parents=True)
        original = folder / "target/proof.kani-metadata.json"
        with self.assertRaises(FileNotFoundError): VERIFY.compiler_receipt(CHECK, folder)
        data = b'{ "proof_harnesses": [], "original_bytes": true }\n'
        original.write_bytes(data)
        self.assertEqual(VERIFY.compiler_receipt(CHECK, folder)["original_bytes"], True)
        self.assertEqual((folder / "compiler-metadata.json").read_bytes(), data)
        with self.assertRaises(VERIFY.Incomplete): VERIFY.compiler_receipt(CHECK, folder)
        redirected = self.work / "redirected-metadata"
        (redirected / "target").mkdir(parents=True)
        (redirected / "target/proof.kani-metadata.json").symlink_to(original)
        with self.assertRaises(VERIFY.Incomplete): VERIFY.compiler_receipt(CHECK, redirected)

    def test_healthy_mocked_dispatch_preserves_fixed_argv_resources_and_seals_all_callee_receipts(self):
        source = self.work / "source"
        source.mkdir()
        retained = []
        for key in VERIFY.CALLEES:
            VERIFY.BASE.retain_artifacts(CHECK, self.work, retained, VERIFY.receipt_roots(self.folder(key)))
        tool_base = self.work / "tools"
        compiler_path = tool_base / VERIFY.BASE.KANI_COMPILER
        readonly = (tool_base / "kani/payload/kani-0.68.0", compiler_path)
        for path in readonly: path.mkdir(parents=True)
        admitted = {"base": tool_base, "policy": {"tools": {"kani": {
            "entrypoint": VERIFY.BASE.KANI_ENTRYPOINT, "toolchain": VERIFY.BASE.KANI_COMPILER}}}}
        folder = VERIFY.job_folder(self.work, 14, self.job)
        entry = "/work/" + (folder / "inputs/proof.rs").relative_to(self.work).as_posix()
        report, compiler, native = observed(self.job, entry)
        def run(_root, _work, _command, **_options):
            (folder / "target").mkdir()
            VERIFY.BASE.write_json(folder / "target/proof.kani-metadata.json", compiler)
            VERIFY.BASE.write_json(folder / "kani.json", report)
            return native
        runtime = SimpleNamespace(run=Mock(side_effect=run), _owned_work=Mock())
        check = SimpleNamespace(input_snapshot=CHECK.input_snapshot, read_once=CHECK.read_once, decode=CHECK.decode,
            PROFILES={"kani_flags": list(VERIFY.BASE.KANI_BASELINE_FLAGS)}, RUNTIME=runtime)
        actual_projection = VERIFY.projection()
        def prepare(_source, inputs, job):
            result = actual_projection.prepare(VERIFY.ROOT, inputs, job)
            result["callee_bindings"] = self.bindings
            return result
        def admit(_check, output):
            VERIFY.BASE.write_json(output, self.admitted_record)
            return admitted, self.admitted_record
        with patch.object(VERIFY, "schedule", return_value=self.jobs), \
             patch.object(VERIFY, "projection", return_value=SimpleNamespace(prepare=prepare)), \
             patch.object(VERIFY, "require_sources"), \
             patch.object(VERIFY, "remaining_budget", return_value=100), \
             patch.object(VERIFY, "admission", side_effect=admit), \
             patch.object(VERIFY.BASE, "proof_readonly_roots", return_value=readonly):
            result = VERIFY.execute_job(check, self.work, source, {}, self.job, 14, retained,
                                        self.certificates, dict(self.identity))
        self.assertTrue(result["expected_outcome_qualified"], result.get("qualification_error"))
        runtime.run.assert_called_once()
        args, options = runtime.run.call_args
        self.assertEqual(args[2][0], str(tool_base / VERIFY.BASE.KANI_ENTRYPOINT))
        self.assertEqual(args[2][5:5 + len(VERIFY.BASE.KANI_BASELINE_FLAGS)], list(VERIFY.BASE.KANI_BASELINE_FLAGS))
        self.assertIn("--keep-temps", args[2])
        self.assertNotIn("function-contracts", args[2])
        self.assertEqual(options["readonly"], readonly)
        for key, value in {"timeout": 100, "cpu_seconds": 180, "build_jobs": 2,
                           "address_space_bytes": 8 * 1024**3, "process_limit": 128}.items():
            self.assertEqual(options[key], value)
        self.assertTrue(set(VERIFY.receipt_roots(self.folder())).issubset(options["sealed"]))
        self.assertEqual((folder / "compiler-metadata.json").read_bytes(), (folder / "target/proof.kani-metadata.json").read_bytes())

    def test_early_caller_dispatch_is_refused_before_runtime(self):
        work = self.work / "early"
        work.mkdir()
        source = work / "source"
        source.mkdir()
        run = Mock()
        check = SimpleNamespace(input_snapshot=CHECK.input_snapshot, read_once=CHECK.read_once, decode=CHECK.decode,
                                RUNTIME=SimpleNamespace(run=run, _owned_work=Mock()))
        identity = self.identity
        def prepare(_source, inputs, job):
            generated = {}
            for name in VERIFY.GENERATED_NAMES:
                data = ("synthetic early-dispatch control " + name).encode()
                (inputs / name).write_bytes(data)
                generated[name] = hashlib.sha256(data).hexdigest()
            return {"entrypoint": inputs / "proof.rs", "expected_stubs": job["expected_stubs"],
                    "callee_bindings": self.bindings, "input_files_sha256": generated,
                    "generated_input_files": list(VERIFY.GENERATED_NAMES)}
        actual_schedule = self.jobs
        projection = SimpleNamespace(prepare=prepare)
        with patch.object(VERIFY, "schedule", return_value=actual_schedule), \
             patch.object(VERIFY, "projection", return_value=projection), \
             patch.object(VERIFY, "require_sources"), \
             patch.object(VERIFY, "remaining_budget", return_value=100), \
             patch.object(VERIFY, "admission", return_value=({}, self.admitted_record)):
            result = VERIFY.execute_job(check, work, source, {}, self.job, 14, [], {}, dict(identity))
        self.assertFalse(result["expected_outcome_qualified"])
        self.assertIn("certificate", result["qualification_error"])
        run.assert_not_called()


if __name__ == "__main__":
    unittest.main()
