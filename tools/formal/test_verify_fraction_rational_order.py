"""Rational consumer controls; synthetic reports execute no verifier or SDK."""
from __future__ import annotations

from copy import deepcopy
import importlib.util
import io
import json
import os
from pathlib import Path
import runpy
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch


SPEC = importlib.util.spec_from_file_location("fraction_rational_controller_under_test",
                                            Path(__file__).with_name("verify_fraction_verus_bridge.py"))
BRIDGE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BRIDGE)
RECIPE = BRIDGE._RATIONAL_ORDER_RECIPE
SOURCE = "/work/inputs/01-healthy/proof.rs"
# Independent protocol inventory; no producer metadata selects these names.
MODES = {"proof::lemma_bridge_bit_length_power": "proof",
         "proof::lemma_bridge_u128_shift_power": "proof",
         "proof::lemma_bridge_complement_reject_outside": "proof",
         "proof::lemma_bridge_u128_shifted_u64": "proof",
         "proof::lemma_bridge_denominator_factor": "proof",
         "proof::lemma_bridge_positive_factor_order": "proof",
         "proof::lemma_bridge_rational_gap_dominates": "proof",
         "proof::composition_constructor_admitted": "exec",
         "proof::composition_checked": "exec",
         "proof::composition_constructor_post": "exec",
         "proof::composition_rational_order": "exec",
         "proof::composition_complement_post": "exec",
         "proof::IntervalError::clone": "exec",
         "proof::FixtureEnclosureError::clone": "exec",
         "proof::Fraction::clone": "exec"}
SCHEDULE = (
    ("healthy", None), ("shift_wrong_theorem", "proof::lemma_bridge_u128_shift_power"),
    ("outside_wrong_theorem", "proof::lemma_bridge_complement_reject_outside"),
    ("complement_wrong_error", "proof::composition_complement_post"),
    ("complement_wrong_scale", "proof::composition_complement_post"),
    ("complement_wrong_conservation", "proof::composition_complement_post"),
    ("variable_shift_wrong_theorem", "proof::lemma_bridge_u128_shifted_u64"),
    ("factor_wrong_theorem", "proof::lemma_bridge_denominator_factor"),
    ("positive_order_wrong_theorem", "proof::lemma_bridge_positive_factor_order"),
    ("domination_wrong_theorem", "proof::lemma_bridge_rational_gap_dominates"),
    ("rational_invalid", "proof::composition_rational_order"),
    ("rational_left_zero", "proof::composition_rational_order"),
    ("rational_right_zero", "proof::composition_rational_order"),
    ("rational_equal_shifts", "proof::composition_rational_order"),
    ("rational_left_small_gap", "proof::composition_rational_order"),
    ("rational_right_small_gap", "proof::composition_rational_order"),
    ("rational_left_large_gap", "proof::composition_rational_order"),
    ("rational_right_large_gap", "proof::composition_rational_order"), ("restored", None))


def report(job):
    fault = job["target"] is not None
    return {"verus": deepcopy(BRIDGE.EXPECTED_IDENTITY),
            "verification-results": {"encountered-error": fault, "encountered-vir-error": False,
                "success": not fault, "verified": 14 if fault else 15, "errors": 1 if fault else 0,
                "is-verifying-entire-crate": True},
            "times-ms": {"smt": {"smt-run-module-times": [{"function-breakdown": [
                {"function": name, "mode:": mode, "success": name != job["target"]}
                for name, mode in MODES.items()]}]}}}


def diagnostic(job, primary=True, exit_line=None):
    span = job["spans"][job["target"]]
    first = span.get("clause_start", span.get("returns_line"))
    last = span.get("clause_end", first)
    return {"$message_type": "diagnostic", "level": "error", "message": "postcondition not satisfied",
            "spans": [{"file_name": SOURCE, "line_start": first, "line_end": last,
                       "is_primary": primary, "label": "failed this postcondition"},
                      {"file_name": SOURCE,
                       "line_start": span["return_start"] if exit_line is None else exit_line,
                       "line_end": span["return_end"] if exit_line is None else exit_line,
                       "is_primary": not primary, "label": "at the end of the function body"}]}


def native(value, job, errors=None):
    if errors is None:
        errors = [] if job["target"] is None else [diagnostic(job)]
    return {"failure": None, "exit_code": 0 if job["target"] is None else 1,
            "output": "\n".join(json.dumps(item) for item in [*errors, value])}


class RationalConsumerTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-rational-consumer-test-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.addCleanup(self.restore_permissions)
        self.generated = self.root / "baseline"
        self.details = BRIDGE.PROJECTION.prepare_rational_order(BRIDGE.ROOT, self.generated)
        self.healthy = (self.generated / "proof.rs").read_bytes()
        self.ghost = (BRIDGE.ROOT / "tools/formal/fraction_verus_bridge_spec.rs").read_bytes()

    def restore_permissions(self):
        for directory, directories, files in os.walk(self.root):
            os.chmod(directory, 0o700)
            for name in directories:
                if not (Path(directory) / name).is_symlink():
                    os.chmod(Path(directory) / name, 0o700)
            for name in files:
                if not (Path(directory) / name).is_symlink():
                    os.chmod(Path(directory) / name, 0o600)

    def job(self, index=0):
        job = {**BRIDGE._jobs(RECIPE)[index], "index": index + 1}
        data = BRIDGE.control_bytes(self.healthy, self.ghost, job, _recipe=RECIPE)
        job["spans"] = BRIDGE.function_spans(data, _recipe=RECIPE)
        return job

    def qualify(self, value, job):
        return BRIDGE.qualify(value, job, BRIDGE.EXPECTED_IDENTITY, SOURCE, _recipe=RECIPE)

    def reject(self, value, job):
        with self.assertRaises((RuntimeError, ValueError)):
            self.qualify(value, job)

    def test_nineteen_literal_jobs_require_fifteen_complete_mixed_mode_results(self):
        self.assertEqual(tuple((job["id"], job["target"]) for job in BRIDGE._jobs(RECIPE)), SCHEDULE)
        self.assertEqual(dict(RECIPE.modes), MODES)
        for index in range(19):
            job = self.job(index)
            with self.subTest(index=index):
                self.assertEqual(self.qualify(native(report(job), job), job)["functions"],
                                 {name: name != job["target"] for name in MODES})
        self.assertEqual(len(BRIDGE.FUNCTIONS), 9)

    def test_closed_recipe_cannot_be_forged_reconfigured_or_selected_by_legacy_globals(self):
        forged = BRIDGE._Recipe(*RECIPE)
        self.assertEqual(forged, RECIPE)
        with self.assertRaises(AttributeError):
            RECIPE.work = self.root
        with patch.object(BRIDGE.PILOT, "admit") as admit, patch.object(Path, "mkdir") as mkdir:
            for value in (forged, {}, "rational", object()):
                with self.subTest(value=type(value)), self.assertRaisesRegex(RuntimeError, "recipe"):
                    BRIDGE.run_campaign(_recipe=value)
            admit.assert_not_called()
            mkdir.assert_not_called()
        with patch.object(BRIDGE, "JOBS", ()), patch.object(BRIDGE, "FUNCTIONS", ()), \
             patch.object(BRIDGE, "_RATIONAL_ORDER_MODES", ()):
            job = self.job()
            self.assertTrue(self.qualify(native(report(job), job), job)["qualified"])
        copied = BRIDGE._jobs(RECIPE)
        copied[0]["target"] = "other"
        self.assertIsNone(BRIDGE._jobs(RECIPE)[0]["target"])

    def test_wrapper_captures_current_source_and_rejects_cli_without_native_dispatch(self):
        entry = BRIDGE.ROOT / "tools/formal/verify_fraction_rational_order.py"
        stub = SimpleNamespace(require=BRIDGE.require, run_rational_order_campaign=lambda: 0)
        with patch.dict("sys.modules", {"verify_fraction_verus_bridge": stub}), \
             patch("sys.argv", [str(entry), "--timeout=other"]):
            with self.assertRaisesRegex(RuntimeError, "no CLI"):
                runpy.run_path(str(entry), run_name="__main__")
        with patch.object(BRIDGE, "run_campaign", return_value=23) as run:
            self.assertEqual(BRIDGE.run_rational_order_campaign(), 23)
            run.assert_called_once_with(_recipe=RECIPE)
        with patch.object(BRIDGE, "run_rational_order_campaign") as dispatch, \
             patch.dict("sys.modules", {"verify_fraction_verus_bridge": BRIDGE}):
            runpy.run_path(str(entry), run_name="source_only_entrypoint")
            dispatch.assert_not_called()
        self.assertIn(b"_binding.load_module", entry.read_bytes())
        self.assertNotIn(b"_bridge = importlib.util.module_from_spec", entry.read_bytes())

    def test_thirty_three_source_closure_keeps_every_prior_owner_and_ten_projector_inputs(self):
        names = set(BRIDGE._source_names(RECIPE))
        prior = set(BRIDGE._source_names(BRIDGE._COMPLEMENT_POST_RECIPE))
        self.assertEqual(len(names), 33)
        self.assertEqual(names - prior, {"tools/formal/fraction_verus_rational_lemmas.rs",
                                       "tools/formal/verify_fraction_rational_order.py",
                                       "tools/formal/test_verify_fraction_rational_order.py"})
        self.assertEqual(len(BRIDGE.PROJECTION.rational_input_names()), 10)
        self.assertTrue(set(BRIDGE.PROJECTION.rational_input_names()) <= names)
        command = BRIDGE.PILOT.command(Path("/pinned/payload"), SOURCE, "/work/trace")
        for argument in ("--no-cheating", "--num-threads=1", "timeout=60000"):
            self.assertIn(argument, command)

    def test_every_control_is_reversible_changes_only_ghost_and_preserves_executable_erasure(self):
        prefix = (self.generated / "predicate_source.rs").read_bytes()
        fragment = prefix[:2333] + prefix[2684:3731]
        executable = BRIDGE.PROJECTION.annotated_rational_order(fragment)
        for index in range(1, 18):
            job = self.job(index)
            changed = BRIDGE.control_bytes(self.healthy, self.ghost, job, _recipe=RECIPE)
            with self.subTest(index=index):
                self.assertNotEqual(changed, self.healthy)
                self.assertEqual(changed.count(executable), 1)
                self.assertEqual(BRIDGE.PROJECTION.erase_rational_order_annotations(executable), fragment)
                self.assertEqual(changed.count((BRIDGE.ROOT / BRIDGE.PROJECTION.BIT_LENGTH_LEMMAS).read_bytes()), 1)
        self.assertEqual(BRIDGE.control_bytes(self.healthy, self.ghost, self.job(18), _recipe=RECIPE), self.healthy)
        for job in ({"id": "unknown", "target": None}, {**self.job(6), "target": "proof::composition_checked"}):
            with self.assertRaisesRegex(RuntimeError, "control identity"):
                BRIDGE.control_bytes(self.healthy, self.ghost, job, _recipe=RECIPE)

    def test_four_helper_faults_retain_all_genuine_contracts_as_conjuncts(self):
        for index, (_, _, theorem, extra, _) in enumerate(BRIDGE._RATIONAL_PROOF_CONTRACTS, 6):
            with self.subTest(index=index):
                changed = BRIDGE.control_bytes(self.healthy, self.ghost, self.job(index), _recipe=RECIPE)
                self.assertEqual(changed, self.healthy.replace(theorem, theorem + extra, 1))
                self.assertEqual(changed.replace(theorem + extra, theorem, 1), self.healthy)
                self.assertEqual(changed.count(self.ghost), 1)

    def test_eight_relation_controls_keep_original_body_and_disjoint_branch_guards(self):
        guards = dict(BRIDGE._RATIONAL_BRANCH_GUARDS)
        self.assertEqual(set(guards), {name for name, _ in SCHEDULE[10:18]})
        for index in range(10, 18):
            changed = BRIDGE.control_bytes(self.healthy, self.ghost, self.job(index), _recipe=RECIPE)
            self.assertIn(b"    if " + guards[SCHEDULE[index][0]] + b" {\n", changed)
            self.assertIn(b"let original = {", changed)
            self.assertIn(b"Ordering::Equal => Ordering::Less,", changed)
            self.assertIn(b"let left_scaled = (left.numerator as int) * bridge_denominator(right.shift);", changed)

    def test_helper_and_predicate_diagnostic_spans_bind_exact_contracts_and_bodies(self):
        spans = BRIDGE.function_spans(self.healthy, _recipe=RECIPE)
        self.assertEqual(set(spans), set(MODES) - {name for name in MODES if "::clone" in name})
        for _, declaration, theorem, _, tail in BRIDGE._RATIONAL_PROOF_CONTRACTS:
            name = "proof::" + declaration.split(b"fn ", 1)[1].split(b"(", 1)[0].decode()
            self.assertLess(spans[name]["clause_end"], spans[name]["return_start"])
            for old, new in ((theorem, theorem + b"    decreases 0,\n"),
                             (tail, tail.replace(b" {", b" && true {"))):
                with self.subTest(name=name), self.assertRaises(RuntimeError):
                    BRIDGE.function_spans(self.healthy.replace(old, new, 1), _recipe=RECIPE)

    def test_metadata_recomputed_and_every_claim_field_rejects_tampering(self):
        BRIDGE.validate_metadata(self.details, BRIDGE.ROOT, self.generated, _recipe=RECIPE)
        for field in self.details:
            changed = deepcopy(self.details)
            changed[field] = None
            (self.generated / "correspondence.json").write_text(json.dumps(changed))
            with self.subTest(field=field), self.assertRaises(RuntimeError):
                BRIDGE.validate_metadata(changed, BRIDGE.ROOT, self.generated, _recipe=RECIPE)
        for field, value in (("schema_version", True), ("proof_commands", False), ("prefix_bytes", 3731.0)):
            changed = deepcopy(self.details)
            changed[field] = value
            (self.generated / "correspondence.json").write_text(json.dumps(changed))
            with self.subTest(field=field), self.assertRaises(RuntimeError):
                BRIDGE.validate_metadata(changed, BRIDGE.ROOT, self.generated, _recipe=RECIPE)

    def test_metadata_nested_domains_offsets_inventory_and_unknown_claims_fail_closed(self):
        changes = []
        for field in ("rational_order", "shifted_u64", "denominator_factor", "positive_factor_order", "gap_dominates"):
            changed = deepcopy(self.details)
            changed["raw_domains"][field] = "requires checked constructor-produced values"
            changes.append(changed)
        for field in ("fragment_source_spans", "correspondence", "proof_hooks"):
            changed = deepcopy(self.details)
            if field == "proof_hooks":
                changed[field][0]["source_byte_offset"] = float(changed[field][0]["source_byte_offset"])
            else:
                key = "source_byte_offsets_half_open" if field == "fragment_source_spans" else "byte_offsets_half_open"
                changed[field][0][key][0] = False
            changes.append(changed)
        changed = deepcopy(self.details)
        changed["proved"] = True
        changes.append(changed)
        changed = deepcopy(self.details)
        changed["expected_function_modes"]["proof::lemma_bridge_u128_shifted_u64"] = "exec"
        changes.append(changed)
        for index, changed in enumerate(changes):
            (self.generated / "correspondence.json").write_text(json.dumps(changed))
            with self.subTest(index=index), self.assertRaises(RuntimeError):
                BRIDGE.validate_metadata(changed, BRIDGE.ROOT, self.generated, _recipe=RECIPE)

    def test_rehashed_generated_artifacts_still_cannot_replace_recomputed_sources(self):
        for name in ("proof.rs", "actual_source.rs", "predicate_source.rs"):
            artifact = self.generated / name
            original = artifact.read_bytes()
            changed = deepcopy(self.details)
            artifact.write_bytes(original + b"// tampered\n")
            changed["generated_sha256"][name] = BRIDGE.digest(artifact.read_bytes())
            (self.generated / "correspondence.json").write_text(json.dumps(changed))
            with self.subTest(name=name), self.assertRaisesRegex(RuntimeError, "generated"):
                BRIDGE.validate_metadata(changed, BRIDGE.ROOT, self.generated, _recipe=RECIPE)
            artifact.write_bytes(original)

    def test_reports_reject_missing_extra_duplicate_wrong_mode_and_wrong_success(self):
        for mutation in ("missing", "extra", "duplicate", "mode", "integer_status", "short_name", "other_failed"):
            job = self.job()
            value = report(job)
            rows = value["times-ms"]["smt"]["smt-run-module-times"][0]["function-breakdown"]
            if mutation == "missing":
                rows.pop()
            elif mutation == "extra":
                rows.append({"function": "proof::unexpected", "mode:": "proof", "success": True})
            elif mutation == "duplicate":
                rows.append(deepcopy(rows[0]))
            elif mutation == "mode":
                rows[3]["mode:"] = "exec"
            elif mutation == "integer_status":
                rows[0]["success"] = 1
            elif mutation == "short_name":
                rows[0]["function"] = "lemma_bridge_bit_length_power"
            else:
                rows[0]["success"] = False
            with self.subTest(mutation=mutation):
                self.reject(native(value, job), job)
        for index in (0, 1):
            job = self.job(index)
            for key, invalid in (("verified", True), ("verified", 13), ("errors", bool(index)),
                                 ("encountered-vir-error", True), ("is-verifying-entire-crate", False)):
                value = report(job)
                value["verification-results"][key] = invalid
                self.reject(native(value, job), job)

    def test_all_seventeen_faults_accept_only_postcondition_diagnostics_in_own_spans(self):
        for index in range(1, 18):
            job = self.job(index)
            for primary in (True, False):
                error = diagnostic(job, primary, job["spans"][job["target"]]["return_end"])
                self.assertTrue(self.qualify(native(report(job), job, [error]), job)["qualified"])
            for mutation in ("foreign", "outside", "no_clause", "no_body", "no_primary", "assertion", "termination"):
                error = diagnostic(job)
                if mutation == "foreign":
                    error["spans"][1]["file_name"] = "/work/other.rs"
                elif mutation == "outside":
                    error["spans"][1]["line_start"] = 1
                elif mutation == "no_clause":
                    error["spans"].pop(0)
                elif mutation == "no_body":
                    error["spans"].pop()
                elif mutation == "no_primary":
                    for span in error["spans"]:
                        span["is_primary"] = False
                else:
                    error["message"] = "assertion failed" if mutation == "assertion" else "decreases not satisfied"
                with self.subTest(index=index, mutation=mutation):
                    self.reject(native(report(job), job, [error]), job)

    def test_strict_output_identity_and_custody_remain_fail_closed(self):
        job = self.job()
        good = native(report(job), job)
        for output in ("", good["output"][:-2], good["output"] + "\n" + good["output"],
                       good["output"] + '\n{"unknown":true}', "unparsed\n" + good["output"]):
            self.reject({**good, "output": output}, job)
        for change in ({"failure": "timeout"}, {"exit_code": True}, {"exit_code": 1}):
            self.reject({**good, **change}, job)
        for field in BRIDGE.EXPECTED_IDENTITY:
            value = report(job)
            value["verus"][field] = "other"
            self.reject(native(value, job), job)

    def test_fixed_clock_and_only_new_cap_require_full_schedule_before_io_or_admission(self):
        self.assertEqual((RECIPE.started_unix_ns, RECIPE.started_monotonic_ns, RECIPE.deadline_monotonic_ns),
                         (1793937385474478571, 34053307946337, 37653307946337))
        self.assertEqual(BRIDGE._command_seconds(RECIPE), 90)
        for recipe in (None, BRIDGE._BIT_LENGTH_RECIPE, BRIDGE._CONSTRUCTOR_POST_RECIPE,
                       BRIDGE._COMPLEMENT_POST_RECIPE):
            self.assertEqual(BRIDGE._command_seconds(recipe), 180)
        with patch.object(BRIDGE, "remaining", return_value=1829.999), \
             patch.object(BRIDGE.PILOT, "admit") as admit, patch.object(Path, "mkdir") as mkdir:
            with self.assertRaisesRegex(RuntimeError, "nineteen"):
                BRIDGE.run_campaign(_recipe=RECIPE)
            admit.assert_not_called()
            mkdir.assert_not_called()
        with patch.object(BRIDGE, "remaining", return_value=1830):
            BRIDGE._rational_dispatch_capacity(RECIPE)
        with patch.object(Path, "read_text", return_value=RECIPE.boot_id), \
             patch.object(BRIDGE.time, "monotonic_ns", return_value=RECIPE.started_monotonic_ns + 10**9), \
             patch.object(BRIDGE.time, "time_ns", return_value=RECIPE.started_unix_ns + 10**9):
            self.assertEqual(BRIDGE.remaining(_recipe=RECIPE), 3599)
            with patch.object(BRIDGE.time, "monotonic_ns", return_value=RECIPE.deadline_monotonic_ns):
                with self.assertRaisesRegex(RuntimeError, "clock"):
                    BRIDGE.remaining(_recipe=RECIPE)

    def test_frozen_preparation_keeps_full_sources_and_exact_restoration(self):
        work = self.root / "preparation"
        work.mkdir(mode=0o700)
        with patch.object(BRIDGE, "_work", return_value=work):
            BRIDGE.freeze_sources(work, _recipe=RECIPE)
            planned, inputs = BRIDGE.prepare_inputs(work, _recipe=RECIPE)
        self.assertEqual(len(planned), 19)
        self.assertEqual((planned[0]["inputs"] / "proof.rs").read_bytes(),
                         (planned[-1]["inputs"] / "proof.rs").read_bytes())
        for job in planned:
            self.assertEqual((job["inputs"] / "predicate_source.rs").stat().st_size, 3731)
            self.assertEqual(set(job["spans"]), set(MODES) - {name for name in MODES if "::clone" in name})
        self.assertEqual(inputs["01-healthy/proof.rs"]["mode"], 0o400)

    def test_first_unexpected_native_result_retains_receipt_and_stops_with_ninety_second_caps(self):
        work, payload = self.root / "attempt", self.root / "payload"
        payload.mkdir()
        (payload / "version.json").write_text(json.dumps({"verus": BRIDGE.EXPECTED_IDENTITY}))
        stable = {"readonly_roots": [self.root / "sdk"], "admission_profile": "stable-core"}
        observed = ["/usr/bin/env", *(str(payload / name) for name in ("verus", "rust_verify", "z3"))]
        failure = {"failure": None, "exit_code": 1, "output": '{"$message_type":"diagnostic",'
                   '"level":"error","message":"unrelated compiler error","spans":[]}'}
        with patch.object(BRIDGE, "_work", return_value=work), \
             patch.object(BRIDGE, "remaining", return_value=2000), \
             patch.object(BRIDGE, "capacity", return_value=4096), \
             patch.object(BRIDGE.PILOT, "admit", return_value=(stable, payload)) as admit, \
             patch.object(BRIDGE.PILOT, "check_trace", return_value=observed), \
             patch.object(BRIDGE.CHECK.RUNTIME, "run", return_value=failure) as run, \
             patch("sys.stdout", new_callable=io.StringIO):
            self.assertEqual(BRIDGE.run_campaign(_recipe=RECIPE), 1)
        self.assertEqual(run.call_count, 1)
        self.assertGreaterEqual(admit.call_count, 2)
        summary = json.loads((work / "summary.json").read_text())
        self.assertEqual(summary["status"], "DEFER")
        self.assertEqual(summary["maximum_commands"], 19)
        self.assertEqual(len(summary["jobs"]), 1)
        self.assertTrue(summary["source_input_receipt_bookends_unchanged"])
        self.assertFalse((work / "jobs/02-shift_wrong_theorem").exists())
        self.assertEqual((run.call_args.kwargs["cpu_seconds"], run.call_args.kwargs["timeout"]), (90, 90))

    def test_synthetic_complete_loop_dispatches_nineteen_only_and_preserves_prior_counts(self):
        self.assertEqual(tuple(len(BRIDGE._jobs(recipe)) for recipe in
            (None, BRIDGE._BIT_LENGTH_RECIPE, BRIDGE._CONSTRUCTOR_POST_RECIPE, BRIDGE._COMPLEMENT_POST_RECIPE)),
            (5, 5, 5, 7))
        work = self.root / "simulated-complete"

        def synthetic(current_work, state, job, record, *, _recipe):
            self.assertIs(_recipe, RECIPE)
            self.assertEqual(current_work, work)
            record["qualified"] = True

        with patch.object(BRIDGE, "_work", return_value=work), \
             patch.object(BRIDGE, "remaining", return_value=2000), \
             patch.object(BRIDGE, "capacity", return_value=4096), \
             patch.object(BRIDGE, "execute_job", side_effect=synthetic) as execute, \
             patch.object(BRIDGE.PILOT, "admit") as admit, patch("sys.stdout", new_callable=io.StringIO):
            self.assertEqual(BRIDGE.run_campaign(_recipe=RECIPE), 0)
        self.assertEqual(execute.call_count, 19)
        admit.assert_not_called()
        self.assertEqual(json.loads((work / "summary.json").read_text())["status"],
                         "rational-order-dependency-checkpoint-passed")


if __name__ == "__main__":
    unittest.main()
