"""Source-only query-split custody/control tests; run no compiler or solver."""
from __future__ import annotations

import sys
sys.dont_write_bytecode = True

import ast
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch


def load(name, filename):
    spec = importlib.util.spec_from_file_location(name, Path(__file__).with_name(filename))
    result = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(result)
    return result


VERIFY = load("query_split_admission_tests", "verify_fraction_query_split.py")
FIXTURE = load("query_split_json_fixture", "test_verify.py")
CHECK = VERIFY.module("query_split_receipt_reader", VERIFY.ROOT / "scripts/check-analysis.py")
ENTRY = "/work/jobs/14-addition_universal_healthy/inputs/proof.rs"


def writable_cleanup(path):
    """Restore only this test's temporary tree so normal cleanup can remove it."""
    if not path.exists():
        return
    path.chmod(0o700)
    for root, dirs, files in os.walk(path):
        for name in dirs:
            item = Path(root) / name
            if not item.is_symlink():
                item.chmod(0o700)
        for name in files:
            item = Path(root) / name
            if not item.is_symlink():
                item.chmod(0o600)


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
    def test_exact_twenty_jobs_both_lanes_and_empty_witness_mappings(self):
        jobs = VERIFY.schedule()
        self.assertEqual(len(jobs), 20)
        self.assertEqual(tuple(job["id"] for _, job in jobs), VERIFY.JOB_IDS)
        for index in (5, 8, 11):
            self.assertEqual(jobs[index - 1][1]["expected_stubs"], [])
            self.assertEqual(len(jobs[index - 1][1]["required_refinement_assertions"]), 2)
        for position, count in ((13, 1), (17, 1), (18, 23), (19, 23)):
            job = jobs[position][1]
            assertions, covers = VERIFY.BASE.additional_suites().expected_contract(job["subject"], job["harness"])
            self.assertEqual(len(assertions), 11)
            self.assertEqual(len(covers), count)
        for change in ("missing", "reordered", "extended", "callee_stub", "dependencies", "domain",
                       "original_fault", "witness_stub", "witness_assumptions", "restore", "wrong_lane"):
            changed = copy.deepcopy([job for _, job in jobs])
            if change == "missing": changed.pop()
            elif change == "reordered": changed[0], changed[1] = changed[1], changed[0]
            elif change == "extended": changed.append(copy.deepcopy(changed[-1]))
            elif change == "callee_stub": changed[4]["expected_stubs"] = changed[13]["expected_stubs"]
            elif change == "dependencies": changed[13]["dependencies"].pop()
            elif change == "domain": changed[13]["harness"] = "narrowed_domain"
            elif change == "original_fault": changed[15]["case"] = "different_fault"
            elif change == "witness_stub":
                for index in (18, 19): changed[index]["expected_stubs"] = changed[13]["expected_stubs"]
            elif change == "witness_assumptions":
                for index in (18, 19): changed[index]["extra_assertions"] = ["model assumption"]
            elif change == "restore": changed[19]["restored_healthy"] = False
            else:
                for index in (18, 19): changed[index]["subject"] = "fraction_addition_universal"
            with self.subTest(change=change), patch.object(VERIFY, "projection", return_value=SimpleNamespace(schedule=lambda: changed)):
                with self.assertRaises(VERIFY.Incomplete): VERIFY.schedule()

    def test_native_request_history_denies_early_repeated_skipped_and_twenty_first(self):
        for index in range(1, 21):
            VERIFY.require_dispatch_order(index, list(range(1, index)))
        for index, history in ((14, []), (1, [1]), (2, [1, 1]), (3, [1]),
                               (21, list(range(1, 21))), (True, []), (2, [True])):
            with self.subTest(index=index, history=history), self.assertRaises(VERIFY.Incomplete):
                VERIFY.require_dispatch_order(index, history)

    def test_old_controller_clock_entrypoints_and_globals_are_never_called(self):
        tree = ast.parse(Path(VERIFY.__file__).read_text())
        forbidden = {"main", "execute_job", "admission", "remaining_budget", "schedule"}
        for node in ast.walk(tree):
            if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute):
                if isinstance(node.func.value, ast.Name) and node.func.value.id == "PURE":
                    self.assertNotIn(node.func.attr, forbidden)
            if isinstance(node, (ast.Assign, ast.AugAssign, ast.AnnAssign)):
                targets = node.targets if isinstance(node, ast.Assign) else [node.target]
                for target in targets:
                    if isinstance(target, ast.Attribute) and isinstance(target.value, ast.Name):
                        self.assertNotEqual(target.value.id, "PURE")
        self.assertEqual(VERIFY.PREPARATION_STARTED_UNIX_NS, 1793921076355277638)
        self.assertEqual(VERIFY.PREPARATION_STARTED_MONOTONIC_NS, 17744188704676)
        self.assertEqual(VERIFY.DEADLINE_MONOTONIC_NS, 21344188704676)
        self.assertNotEqual(VERIFY.PREPARATION_STARTED_MONOTONIC_NS, VERIFY.PURE.PREPARATION_STARTED_MONOTONIC_NS)

    def test_all_source_inputs_are_real_repository_files_and_include_consumer_delta(self):
        names = VERIFY.input_names()
        self.assertTrue(set(VERIFY.SOURCE_NAMES).issubset(names))
        self.assertIn("scripts/tests/test_check_analysis.py", names)
        self.assertNotIn("proof.rs", names)
        for name in names:
            self.assertTrue((VERIFY.ROOT / name).is_file(), name)
            self.assertEqual(VERIFY.ROOT / name, (VERIFY.ROOT / name).resolve(strict=True))

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
        for index in (5, 8, 11, 14, 19, 20):
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

    def test_witness_selected_compiler_and_stdout_mappings_must_both_be_empty(self):
        job = self.jobs[18][1]
        self.qualify(observed(job), job)
        for channel in ("compiler", "stdout", "verified"):
            values = observed(job)
            if channel == "compiler":
                values[1]["proof_harnesses"][0]["attributes"]["stubs"] = self.job["expected_stubs"]
            elif channel == "stdout":
                values[2]["output"] += "  - Stub: A::f -> A::model\n"
            else:
                values[1]["proof_harnesses"][0]["attributes"]["verified_stubs"] = ["hidden"]
            with self.subTest(channel=channel), self.assertRaises(VERIFY.Incomplete):
                self.qualify(values, job)

    def test_every_original_math_label_is_required_in_both_lanes(self):
        for index in (13, 18):
            job = self.jobs[index][1]
            labels, _ = VERIFY.BASE.additional_suites().expected_contract(job["subject"], job["harness"])
            for label in labels:
                values = observed(job)
                checks = values[0]["verification_results"]["results"][0]["checks"]
                checks[:] = [item for item in checks if item["description"].strip('"') != label]
                FIXTURE.recount(values[0])
                with self.subTest(lane=job["id"], label=label), self.assertRaises(VERIFY.Incomplete):
                    self.qualify(values, job)


class PermissionCustodyTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-query-permissions-")
        self.addCleanup(self.temporary.cleanup)
        self.work = Path(self.temporary.name)
        self.addCleanup(writable_cleanup, self.work)
        self.inputs = self.work / "inputs"
        (self.inputs / "nested").mkdir(parents=True)
        (self.inputs / "nested/value.py").write_text("VALUE = 1\n")

    def test_hardened_inputs_are_readable_exactly_owned_and_runtime_compatible(self):
        before = VERIFY.harden_inputs(self.work, self.inputs)
        self.assertEqual((self.inputs / "nested/value.py").read_text(), "VALUE = 1\n")
        self.assertTrue(all(item["mode"] == (0o500 if item["kind"] == "directory" else 0o400)
                            and item["uid"] == os.geteuid() for item in before.values()))
        CHECK.RUNTIME._owned_work(self.work, VERIFY.ROOT)
        self.assertEqual(before, VERIFY.permission_map(self.work, [self.inputs]))
        self.assertEqual(self.work.stat().st_mode & 0o777, 0o700)

    def test_chmod_only_drift_is_rejected_while_global_content_snapshot_stays_equal(self):
        VERIFY.harden_inputs(self.work, self.inputs)
        retained = []
        VERIFY.retain_artifacts(CHECK, self.work, retained, [self.inputs], permission_roots=[self.inputs])
        baseline = CHECK.input_snapshot(self.work, [self.inputs])
        target = self.inputs / "nested/value.py"
        for path, mode in ((target, 0o600), (target.parent, 0o700), (self.inputs, 0o700)):
            original = path.stat().st_mode & 0o777
            path.chmod(mode)
            self.assertEqual(baseline, CHECK.input_snapshot(self.work, [self.inputs]))
            with self.subTest(path=path), self.assertRaises(VERIFY.Incomplete):
                VERIFY.audit_retained(CHECK, self.work, retained)
            path.chmod(original)
        VERIFY.audit_retained(CHECK, self.work, retained)

    def test_extra_file_directory_owner_symlink_and_hardlink_are_rejected(self):
        VERIFY.harden_inputs(self.work, self.inputs)
        retained = []
        VERIFY.retain_artifacts(CHECK, self.work, retained, [self.inputs], permission_roots=[self.inputs])
        for kind in ("file", "directory", "symlink", "hardlink"):
            self.inputs.chmod(0o700)
            extra = self.inputs / "extra"
            if kind == "file":
                extra.write_bytes(b"extra")
                extra.chmod(0o400)
            elif kind == "directory": extra.mkdir(mode=0o500)
            elif kind == "symlink": extra.symlink_to(self.inputs / "nested/value.py")
            else: os.link(self.inputs / "nested/value.py", extra)
            self.inputs.chmod(0o500)
            with self.subTest(kind=kind), self.assertRaises(VERIFY.Incomplete):
                VERIFY.audit_retained(CHECK, self.work, retained)
            self.inputs.chmod(0o700)
            if kind == "directory": extra.rmdir()
            else: extra.unlink()
            self.inputs.chmod(0o500)
        with patch.object(VERIFY.os, "geteuid", return_value=os.geteuid() + 1):
            with self.assertRaises(VERIFY.Incomplete): VERIFY.permission_map(self.work, [self.inputs])
        alias = self.work / "redirected"
        alias.symlink_to(self.inputs, target_is_directory=True)
        with self.assertRaises(VERIFY.Incomplete): VERIFY.permission_map(self.work, [alias])
        VERIFY.audit_retained(CHECK, self.work, retained)

    def test_hardening_rejects_unsafe_existing_entries_before_chmod(self):
        target = self.work / "outside"
        target.write_text("untouched")
        target.chmod(0o600)
        (self.inputs / "redirect").symlink_to(target)
        with self.assertRaises(VERIFY.Incomplete): VERIFY.harden_inputs(self.work, self.inputs)
        self.assertEqual(target.stat().st_mode & 0o777, 0o600)

    def test_ordinary_python_import_cannot_add_bytecode_to_complete_frozen_source(self):
        source = self.work / "source"
        records = VERIFY.BASE.freeze_inputs(CHECK, source, VERIFY.input_names())
        permissions = VERIFY.harden_inputs(self.work, source)
        before = CHECK.input_snapshot(self.work, [source])
        environment = dict(os.environ)
        environment.pop("PYTHONDONTWRITEBYTECODE", None)
        environment.pop("PYTHONPYCACHEPREFIX", None)
        program = ("import sys; assert not sys.dont_write_bytecode; "
                   "sys.path.insert(0, sys.argv[1]); import verify_fraction_query_split; "
                   "assert sys.dont_write_bytecode")
        child = subprocess.run([sys.executable, "-c", program, str(source / "tools/formal")],
                               env=environment, text=True, capture_output=True, timeout=15, check=False)
        self.assertEqual(child.returncode, 0, child.stderr)
        self.assertFalse(list(source.rglob("__pycache__")))
        self.assertFalse(list(source.rglob("*.pyc")))
        self.assertEqual(before, CHECK.input_snapshot(self.work, [source]))
        self.assertEqual(permissions, VERIFY.permission_map(self.work, [source]))
        VERIFY.require_sources(CHECK, source, records)

    def test_identical_ordinary_import_creates_cache_only_in_writable_sibling(self):
        frozen, writable = self.work / "frozen", self.work / "writable"
        for folder in (frozen, writable):
            folder.mkdir(mode=0o700)
            (folder / "accidental_observer.py").write_text("ANSWER = 42\n")
            (folder / "accidental_observer.py").chmod(0o600)
        permissions = VERIFY.harden_inputs(self.work, frozen)
        before = CHECK.input_snapshot(self.work, [frozen])
        program = ("import sys; assert not sys.dont_write_bytecode; "
                   "sys.path.insert(0,sys.argv[1]); import accidental_observer; "
                   "assert accidental_observer.ANSWER == 42; assert not sys.dont_write_bytecode")
        environment = dict(os.environ)
        environment.pop("PYTHONDONTWRITEBYTECODE", None)
        environment.pop("PYTHONPYCACHEPREFIX", None)
        for folder in (frozen, writable):
            child = subprocess.run([sys.executable, "-I", "-c", program, str(folder)],
                                   env=environment, capture_output=True, text=True, timeout=15, check=False)
            self.assertEqual(child.returncode, 0, child.stderr)
        self.assertFalse(list(frozen.rglob("__pycache__")))
        self.assertTrue(list(writable.rglob("__pycache__")))
        self.assertTrue(list(writable.rglob("*.pyc")))
        self.assertEqual(before, CHECK.input_snapshot(self.work, [frozen]))
        self.assertEqual(permissions, VERIFY.permission_map(self.work, [frozen]))


class CertificateAndDispatchTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-query-certificates-")
        self.addCleanup(self.temporary.cleanup)
        self.work = Path(self.temporary.name)
        self.addCleanup(writable_cleanup, self.work)
        self.source = VERIFY.ROOT
        self.jobs = VERIFY.schedule()
        self.bindings = {key: hashlib.sha256(key.encode()).hexdigest() for key in VERIFY.CALLEES}
        self.admitted_record = {"consumer": VERIFY.CONSUMER, "seconds": 0.01,
            "readonly_roots": ["/admitted/kani", "/admitted/compiler"],
            "installation_pointer_sha256": "pointer", "kani_executable_sha256": "kani",
            "compiler_rustc_sha256": "rustc", "compiler_rustdoc_sha256": "rustdoc", "profile": {"flags": "fixed"},
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
                data = ("synthetic source-only controller control " + name).encode()
                (inputs / name).write_bytes(data)
                generated[name] = hashlib.sha256(data).hexdigest()
            permissions = VERIFY.harden_inputs(self.work, inputs)
            before = CHECK.input_snapshot(self.work, [inputs])
            report, compiler, native = observed(job, "/work/" + (inputs / "proof.rs").relative_to(self.work).as_posix())
            prepared = {"callee_bindings": self.bindings, "input_files_sha256": generated,
                        "generated_input_files": list(VERIFY.GENERATED_NAMES)}
            record = {"index": index, "job": job, "consumer": VERIFY.CONSUMER,
                      "expected_outcome_qualified": True, "input_unchanged": True,
                      "input_before": before, "input_after": before,
                      "permissions_before": permissions, "permissions_after": permissions,
                      "permissions_unchanged": True, "native": native,
                      "tool_identity": self.identity, "preparation": prepared}
            for name, value in (("input-before.json", before), ("input-after.json", before),
                                ("permissions-before.json", permissions), ("permissions-after.json", permissions),
                                ("admission.json", self.admitted_record), ("kani.json", report),
                                ("compiler-metadata.json", compiler), ("execution.json", record)):
                VERIFY.BASE.write_json(folder / name, value)
            self.certificates[key] = VERIFY.issue_certificate(CHECK, self.work, self.source, key, self.identity)
        self.job = self.jobs[13][1]
        self.prepared = {"callee_bindings": self.bindings}

    def folder(self, key="constructor"):
        index, job = next((i, j) for i, j in self.jobs if j["id"] == key)
        return VERIFY.job_folder(self.work, index, job)

    def require(self, certificates=None, job=None):
        VERIFY.require_callees(CHECK, self.work, self.source, job or self.job, self.prepared,
                               self.certificates if certificates is None else certificates, self.identity)

    def test_fresh_raw_healthy_callees_required_before_universal_and_every_source_fault(self):
        self.require()
        for _, job in self.jobs[13:18]:
            self.require(job=job)
            with self.subTest(job=job["id"]), self.assertRaises(VERIFY.Incomplete):
                self.require(certificates={}, job=job)
        for key in ("constructor_bad_post", "comparison_wrong_seed", "addition_universal_healthy", "mech_healthy"):
            with self.subTest(key=key), self.assertRaises(VERIFY.Incomplete):
                VERIFY.issue_certificate(CHECK, self.work, self.source, key, self.identity)

    def test_missing_extra_negative_old_consumer_and_stale_certificate_are_rejected(self):
        for change in ("missing", "extra", "negative", "consumer", "tool", "body", "schema", "summary", "snapshot", "permission"):
            certificates = copy.deepcopy(self.certificates)
            if change == "missing": del certificates["comparison"]
            elif change == "extra": certificates["negative_control"] = copy.deepcopy(certificates["constructor"])
            elif change == "negative": certificates["constructor"]["key"] = "constructor_bad_post"
            elif change == "consumer": certificates["constructor"]["consumer"] = VERIFY.PURE.CONSUMER
            elif change == "tool": certificates["constructor"]["tool_identity"] = {"different": "tool"}
            elif change == "body": certificates["constructor"]["binding"] = "0" * 64
            elif change == "schema": certificates["constructor"]["schema_version"] = True
            elif change == "summary": certificates["constructor"]["qualified"] = True
            elif change == "snapshot": certificates["constructor"]["receipt_snapshot"] = {}
            else: certificates["constructor"]["input_permissions"] = {}
            with self.subTest(change=change), self.assertRaises(VERIFY.Incomplete): self.require(certificates)

    def test_raw_post_seed_and_original_cover_corruption_reject_even_rebased_summary(self):
        for key in VERIFY.CALLEES:
            folder = self.folder(key)
            path = folder / "kani.json"
            original = path.read_bytes()
            job = next(job for _, job in self.jobs if job["id"] == key)
            baseline = CHECK.decode(original)
            checks = baseline["verification_results"]["results"][0]["checks"]
            labels = list(job["required_refinement_assertions"])
            labels += [next(item["description"] for item in checks if item["category"] == "cover")]
            for label in labels:
                changed = copy.deepcopy(baseline)
                values = changed["verification_results"]["results"][0]["checks"]
                values[:] = [item for item in values if item["description"].strip('"') != label.strip('"')]
                FIXTURE.recount(changed)
                VERIFY.BASE.write_json(path, changed)
                certificates = copy.deepcopy(self.certificates)
                certificates[key]["receipt_snapshot"] = CHECK.input_snapshot(self.work, VERIFY.receipt_roots(folder))
                with self.subTest(key=key, label=label), self.assertRaises(VERIFY.Incomplete): self.require(certificates)
                with self.assertRaises(VERIFY.Incomplete): VERIFY.issue_certificate(CHECK, self.work, self.source, key, self.identity)
                path.write_bytes(original)

    def test_permission_only_callee_drift_invalidates_certificate_without_content_change(self):
        folder = self.folder()
        target = folder / "inputs/proof.rs"
        before = CHECK.input_snapshot(self.work, [folder / "inputs"])
        target.chmod(0o600)
        self.assertEqual(before, CHECK.input_snapshot(self.work, [folder / "inputs"]))
        with self.assertRaises(VERIFY.Incomplete): self.require()
        with self.assertRaises(VERIFY.Incomplete): VERIFY.issue_certificate(CHECK, self.work, self.source, "constructor", self.identity)
        target.chmod(0o400)
        self.require()

    def test_missing_permission_bookend_cannot_issue_certificate(self):
        (self.folder() / "permissions-after.json").unlink()
        with self.assertRaises((VERIFY.Incomplete, OSError)):
            VERIFY.issue_certificate(CHECK, self.work, self.source, "constructor", self.identity)


class CompletePipelineTests(unittest.TestCase):
    """Drive actual preparation/custody with synthetic native-result records only."""

    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-query-full-pipeline-")
        self.addCleanup(self.temporary.cleanup)
        self.work = Path(self.temporary.name)
        self.addCleanup(writable_cleanup, self.work)
        self.source = self.work / "source"
        self.records = VERIFY.BASE.freeze_inputs(CHECK, self.source, VERIFY.input_names())
        VERIFY.harden_inputs(self.work, self.source)
        self.retained = []
        VERIFY.retain_artifacts(CHECK, self.work, self.retained, [self.source], permission_roots=[self.source])
        self.jobs = VERIFY.schedule(self.source)
        self.certificates, self.anchor, self.history = {}, {}, []
        base = self.work / "admitted-tools"
        compiler = base / VERIFY.BASE.KANI_COMPILER
        self.readonly = (base / "kani/payload/kani-0.68.0", compiler)
        stable = base / "stable"
        for path in (*self.readonly, stable): path.mkdir(parents=True, exist_ok=True)
        admitted = {"admission_profile": "kani-proof", "base": base, "stable": stable,
                    "readonly_roots": list(self.readonly), "inventories": ["synthetic test identity"],
                    "policy_sha256": "synthetic policy", "policy": {"tools": {"kani": {
                        "entrypoint": VERIFY.BASE.KANI_ENTRYPOINT, "toolchain": VERIFY.BASE.KANI_COMPILER}}}}
        self.admitted_record = {"consumer": VERIFY.CONSUMER, "seconds": 0.01,
            "readonly_roots": self.readonly, "installation_pointer_sha256": "pointer",
            "kani_executable_sha256": "kani", "compiler_rustc_sha256": "rustc",
            "compiler_rustdoc_sha256": "rustdoc", "profile": {"flags": "fixed"}, "admitted": admitted}
        def admit(_check, output):
            VERIFY.BASE.write_json(output, self.admitted_record)
            return admitted, self.admitted_record
        self.admit = admit
        self.run = Mock(side_effect=self.synthetic_native)
        self.check = SimpleNamespace(input_snapshot=CHECK.input_snapshot, read_once=CHECK.read_once,
            decode=CHECK.decode, PROFILES={"kani_flags": list(VERIFY.BASE.KANI_BASELINE_FLAGS)},
            RUNTIME=SimpleNamespace(run=self.run, _owned_work=CHECK.RUNTIME._owned_work))

    def synthetic_native(self, _root, work, command, **options):
        entry = work / command[1].removeprefix("/work/")
        folder = entry.parent.parent
        index = int(folder.name.split("-", 1)[0])
        job = self.jobs[index - 1][1]
        report, compiler, native = observed(job, command[1])
        VERIFY.permission_map(work, [entry.parent])
        self.assertEqual(command[2:5], ["--harness", job["harness"], "--exact"])
        self.assertEqual(command[5:5 + len(VERIFY.BASE.KANI_BASELINE_FLAGS)], list(VERIFY.BASE.KANI_BASELINE_FLAGS))
        self.assertEqual(options["readonly"], self.readonly)
        self.assertEqual(options["timeout"], 100)
        self.assertEqual(options["cpu_seconds"], 180)
        self.assertEqual(options["build_jobs"], 2)
        self.assertEqual(options["address_space_bytes"], 8 * 1024**3)
        self.assertEqual(options["process_limit"], 128)
        self.assertIn(self.source, options["sealed"])
        for key in self.certificates:
            callee_index, callee = next((i, j) for i, j in self.jobs if j["id"] == key)
            expected = VERIFY.receipt_roots(VERIFY.job_folder(work, callee_index, callee))
            self.assertTrue(set(expected).issubset(options["sealed"]))
        (folder / "target").mkdir()
        VERIFY.BASE.write_json(folder / "target/proof.kani-metadata.json", compiler)
        VERIFY.BASE.write_json(folder / "kani.json", report)
        return native

    def execute(self, index):
        job = self.jobs[index - 1][1]
        return VERIFY.execute_job(self.check, self.work, self.source, self.records, job, index,
                                  self.retained, self.certificates, self.anchor, self.history)

    def test_complete_twenty_job_pipeline_checks_both_restorations_permissions_and_resources(self):
        with patch.object(VERIFY, "remaining_budget", return_value=100), \
             patch.object(VERIFY, "admission", side_effect=self.admit):
            for index, job in self.jobs:
                record = self.execute(index)
                self.assertTrue(record["expected_outcome_qualified"], (index, record.get("qualification_error")))
                self.assertTrue(record["permissions_unchanged"])
                if job["id"] in VERIFY.CALLEES:
                    self.certificates[job["id"]] = VERIFY.issue_certificate(
                        self.check, self.work, self.source, job["id"], self.anchor)
                if index in {18, 20}:
                    self.assertTrue(record["restored_generated_input_matches_healthy"])
                    self.assertEqual(record["restored_healthy_index"], 14 if index == 18 else 19)
                if index in {2, 11, 13, 18, 20}:
                    VERIFY.phase_bookend(self.check, self.work, self.source, self.records,
                                         self.retained, self.anchor, index)
        self.assertEqual(self.run.call_count, 20)
        self.assertEqual(self.history, list(range(1, 21)))
        VERIFY.audit_retained(self.check, self.work, self.retained)
        self.assertFalse(list(self.source.rglob("__pycache__")))

    def test_early_witness_and_universal_are_refused_before_native_request(self):
        for index in (14, 19, 20):
            with self.subTest(index=index), self.assertRaises(VERIFY.Incomplete): self.execute(index)
        self.run.assert_not_called()
        self.assertEqual(self.history, [])

    def test_universal_with_missing_certificates_cannot_dispatch_even_with_correct_order(self):
        self.history[:] = list(range(1, 14))
        with patch.object(VERIFY, "remaining_budget", return_value=100), \
             patch.object(VERIFY, "admission", side_effect=self.admit):
            record = self.execute(14)
        self.assertFalse(record["expected_outcome_qualified"])
        self.assertIn("certificate", record["qualification_error"])
        self.run.assert_not_called()
        self.assertEqual(self.history, list(range(1, 14)))

    def test_permission_drift_during_admission_stops_before_native_request(self):
        def drift(check, output):
            result = self.admit(check, output)
            (output.parent / "inputs/proof.rs").chmod(0o600)
            return result
        with patch.object(VERIFY, "remaining_budget", return_value=100), \
             patch.object(VERIFY, "admission", side_effect=drift):
            record = self.execute(1)
        self.assertFalse(record["expected_outcome_qualified"])
        self.assertIn("permission", record["qualification_error"].lower())
        self.run.assert_not_called()
        self.assertEqual(self.history, [])

    def test_frozen_source_permission_drift_during_admission_stops_before_native_request(self):
        def drift(check, output):
            result = self.admit(check, output)
            (self.source / VERIFY.SOURCE_NAMES[0]).chmod(0o600)
            return result
        with patch.object(VERIFY, "remaining_budget", return_value=100), \
             patch.object(VERIFY, "admission", side_effect=drift):
            record = self.execute(1)
        self.assertFalse(record["expected_outcome_qualified"])
        self.assertIn("permission", record["qualification_error"].lower())
        self.run.assert_not_called()
        self.assertEqual(self.history, [])

    def test_permission_drift_during_native_command_is_retained_as_incomplete(self):
        def drift(root, work, command, **options):
            result = self.synthetic_native(root, work, command, **options)
            (work / command[1].removeprefix("/work/")).chmod(0o600)
            return result
        self.run.side_effect = drift
        with patch.object(VERIFY, "remaining_budget", return_value=100), \
             patch.object(VERIFY, "admission", side_effect=self.admit):
            record = self.execute(1)
        self.assertFalse(record["expected_outcome_qualified"])
        self.assertIn("permission", record["qualification_error"].lower())
        self.run.assert_called_once()
        self.assertEqual(self.history, [1])

    def test_frozen_source_permission_drift_during_native_command_is_incomplete(self):
        def drift(root, work, command, **options):
            result = self.synthetic_native(root, work, command, **options)
            (self.source / VERIFY.SOURCE_NAMES[0]).chmod(0o600)
            return result
        self.run.side_effect = drift
        with patch.object(VERIFY, "remaining_budget", return_value=100), \
             patch.object(VERIFY, "admission", side_effect=self.admit):
            record = self.execute(1)
        self.assertFalse(record["expected_outcome_qualified"])
        self.assertIn("permission", record["qualification_error"].lower())
        self.run.assert_called_once()
        self.assertEqual(self.history, [1])


class FirstIncompleteStopTests(unittest.TestCase):
    def test_main_stops_at_first_incomplete_job_without_retry_or_later_witness_dispatch(self):
        with tempfile.TemporaryDirectory(prefix="nevrotick-query-main-stop-") as temporary:
            parent = Path(temporary)
            self.addCleanup(writable_cleanup, parent)
            admitted_record = {"consumer": VERIFY.CONSUMER, "seconds": 0,
                "readonly_roots": ["/synthetic/kani", "/synthetic/compiler"],
                "installation_pointer_sha256": "pointer", "kani_executable_sha256": "kani",
                "compiler_rustc_sha256": "rustc", "compiler_rustdoc_sha256": "rustdoc", "profile": {},
                "admitted": {"base": "/synthetic", "stable": "/synthetic/stable",
                             "policy_sha256": "policy", "inventories": []}}
            def admission(_check, output):
                VERIFY.BASE.write_json(output, admitted_record)
                return {}, admitted_record
            old_mask = os.umask(0o077)
            os.umask(old_mask)
            try:
                for first_incomplete in (2, 14, 19):
                    destination = parent / f"stop-{first_incomplete}"
                    requested = []
                    def execute(_check, work, _source, _records, job, index,
                                _retained, _certificates, _anchor, history):
                        requested.append((index, job["id"]))
                        history.append(index)
                        folder = VERIFY.job_folder(work, index, job)
                        folder.mkdir(parents=True)
                        return {"index": index, "job": job, "output": str(folder),
                            "expected_outcome_qualified": index != first_incomplete,
                            "native": {"exit_code": None if index == first_incomplete else 0},
                            "qualification_error": "synthetic timeout with zero checks" if index == first_incomplete else None}
                    with self.subTest(first_incomplete=first_incomplete), \
                         patch.object(VERIFY, "output_preflight", return_value=destination), \
                         patch.object(VERIFY, "remaining_budget", return_value=100), \
                         patch.object(VERIFY, "execute_job", side_effect=execute) as dispatch, \
                         patch.object(VERIFY, "issue_certificate", return_value={"synthetic": True}), \
                         patch.object(VERIFY, "phase_bookend"), \
                         patch.object(VERIFY, "admission", side_effect=admission), patch("builtins.print"):
                        self.assertEqual(VERIFY.main(["--output-dir", str(destination)]), 1)
                        self.assertEqual(dispatch.call_count, first_incomplete)
                    summary = CHECK.decode((destination / "summary.json").read_bytes())
                    self.assertEqual(summary["status"], "incomplete")
                    self.assertEqual(summary["decision"], "DEFER")
                    self.assertEqual(summary["native_proof_request_order"], list(range(1, first_incomplete + 1)))
                    self.assertEqual([index for index, _ in requested], list(range(1, first_incomplete + 1)))
                    self.assertNotIn("addition_witnesses_restored", [name for _, name in requested])
            finally:
                os.umask(old_mask)
                writable_cleanup(parent)


if __name__ == "__main__":
    unittest.main()
