"""Complement-Post consumer controls; no SDK, admission or solver executes."""
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


SPEC = importlib.util.spec_from_file_location("fraction_complement_post_controller_under_test",
                                            Path(__file__).with_name("verify_fraction_verus_bridge.py"))
BRIDGE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BRIDGE)
RECIPE = BRIDGE._COMPLEMENT_POST_RECIPE
SOURCE = "/work/inputs/01-healthy/proof.rs"
# Independent expected protocol inventory, not copied from consumer metadata.
MODES = {"proof::lemma_bridge_bit_length_power": "proof",
         "proof::lemma_bridge_u128_shift_power": "proof",
         "proof::lemma_bridge_complement_reject_outside": "proof",
         "proof::composition_constructor_admitted": "exec",
         "proof::composition_checked": "exec",
         "proof::composition_constructor_post": "exec",
         "proof::composition_complement_post": "exec",
         "proof::IntervalError::clone": "exec",
         "proof::FixtureEnclosureError::clone": "exec",
         "proof::Fraction::clone": "exec"}


def report(job):
    fault = job["target"] is not None
    return {"verus": deepcopy(BRIDGE.EXPECTED_IDENTITY),
            "verification-results": {"encountered-error": fault, "encountered-vir-error": False,
                "success": not fault, "verified": 9 if fault else 10, "errors": 1 if fault else 0,
                "is-verifying-entire-crate": True},
            "times-ms": {"smt": {"smt-run-module-times": [{"function-breakdown": [
                {"function": name, "mode:": mode, "success": name != job["target"]}
                for name, mode in MODES.items()]}]}}}


def diagnostic(job, *, primary_clause=True, exit_line=None):
    spans = job["spans"][job["target"]]
    first = spans.get("clause_start", spans.get("returns_line"))
    last = spans.get("clause_end", first)
    return {"$message_type": "diagnostic", "level": "error", "message": "postcondition not satisfied",
            "spans": [{"file_name": SOURCE, "line_start": first, "line_end": last,
                       "is_primary": primary_clause, "label": "failed this postcondition"},
                      {"file_name": SOURCE,
                       "line_start": spans["return_start"] if exit_line is None else exit_line,
                       "line_end": spans["return_end"] if exit_line is None else exit_line,
                       "is_primary": not primary_clause, "label": "at the end of the function body"}]}


def native(value, job, errors=None):
    if errors is None:
        errors = [] if job["target"] is None else [diagnostic(job)]
    return {"failure": None, "exit_code": 0 if job["target"] is None else 1,
            "output": "\n".join(json.dumps(item) for item in [*errors, value])}


class ComplementPostConsumerTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-complement-post-consumer-test-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.addCleanup(self.restore_permissions)
        self.generated = self.root / "baseline"
        self.details = BRIDGE.PROJECTION.prepare_complement_post(BRIDGE.ROOT, self.generated)
        self.healthy = (self.generated / "proof.rs").read_bytes()
        self.ghost = (BRIDGE.ROOT / "tools/formal/fraction_verus_bridge_spec.rs").read_bytes()

    def restore_permissions(self):
        for directory, subdirs, files in os.walk(self.root):
            os.chmod(directory, 0o700)
            for name in subdirs:
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

    def rejected(self, value, job):
        with self.assertRaises((RuntimeError, ValueError)):
            self.qualify(value, job)

    def test_all_seven_jobs_require_same_ten_exact_mixed_mode_results(self):
        self.assertEqual([(job["id"], job["target"]) for job in BRIDGE._jobs(RECIPE)], [
            ("healthy", None), ("shift_wrong_theorem", "proof::lemma_bridge_u128_shift_power"),
            ("outside_wrong_theorem", "proof::lemma_bridge_complement_reject_outside"),
            ("complement_wrong_error", "proof::composition_complement_post"),
            ("complement_wrong_scale", "proof::composition_complement_post"),
            ("complement_wrong_conservation", "proof::composition_complement_post"), ("restored", None)])
        for index in range(7):
            with self.subTest(index=index):
                job = self.job(index)
                qualified = self.qualify(native(report(job), job), job)
                self.assertEqual(qualified["functions"], {name: name != job["target"] for name in MODES})
        self.assertEqual(len(BRIDGE.FUNCTIONS), 9)
        self.assertEqual(len(BRIDGE.PROJECTION.input_names()), 7)

    def test_recipe_and_nested_inventories_are_immutable_and_legacy_globals_do_not_select_jobs(self):
        with self.assertRaises(AttributeError):
            RECIPE.work = self.root
        self.assertTrue(all(type(item) is tuple for item in (*RECIPE.modes, *RECIPE.jobs)))
        with patch.object(BRIDGE, "JOBS", ()), patch.object(BRIDGE, "FUNCTIONS", ()), \
             patch.object(BRIDGE, "_COMPLEMENT_POST_MODES", ()):
            self.assertEqual(len(BRIDGE._jobs(RECIPE)), 7)
            job = self.job()
            self.assertTrue(self.qualify(native(report(job), job), job)["qualified"])
        changed = BRIDGE._jobs(RECIPE)
        changed[0]["target"] = "forged"
        self.assertIsNone(BRIDGE._jobs(RECIPE)[0]["target"])

    def test_forged_recipe_rejected_before_io_admission_or_work_creation(self):
        forged = BRIDGE._Recipe(*RECIPE)
        self.assertEqual(forged, RECIPE)
        self.assertIsNot(forged, RECIPE)
        with patch.object(BRIDGE.PILOT, "admit") as admit, patch.object(Path, "mkdir") as mkdir:
            for value in (forged, {}, "complement-post", object()):
                with self.subTest(value=type(value)), self.assertRaisesRegex(RuntimeError, "recipe"):
                    BRIDGE.run_campaign(_recipe=value)
            admit.assert_not_called()
            mkdir.assert_not_called()

    def test_fixed_entrypoint_rejects_cli_and_dispatches_only_dedicated_function(self):
        entry = BRIDGE.ROOT / "tools/formal/verify_fraction_complement_post.py"
        stub = SimpleNamespace(require=BRIDGE.require, run_complement_post_campaign=lambda: 0)
        with patch.dict("sys.modules", {"verify_fraction_verus_bridge": stub}), \
             patch("sys.argv", [str(entry), "--profile=other"]):
            with self.assertRaisesRegex(RuntimeError, "no CLI"):
                runpy.run_path(str(entry), run_name="__main__")
        with patch.object(BRIDGE, "run_campaign", return_value=23) as run:
            self.assertEqual(BRIDGE.run_complement_post_campaign(), 23)
            run.assert_called_once_with(_recipe=RECIPE)
        with patch.object(BRIDGE, "run_complement_post_campaign") as dispatch, \
             patch.dict("sys.modules", {"verify_fraction_verus_bridge": BRIDGE}):
            runpy.run_path(str(entry), run_name="source_only_entrypoint")
            dispatch.assert_not_called()

    def test_closed_source_inventory_includes_four_consumers_tests_and_nine_projection_inputs(self):
        names = set(BRIDGE._source_names(RECIPE))
        self.assertEqual(len(names), 30)
        self.assertTrue(set(BRIDGE.PROJECTION.complement_input_names()) <= names)
        self.assertEqual(len(BRIDGE.PROJECTION.complement_input_names()), 9)
        self.assertTrue({"tools/formal/verify_fraction_verus_bridge.py",
                         "tools/formal/test_verify_fraction_verus_bridge.py",
                         "tools/formal/verify_fraction_bit_length.py",
                         "tools/formal/test_verify_fraction_bit_length.py",
                         "tools/formal/verify_fraction_constructor_post.py",
                         "tools/formal/test_verify_fraction_constructor_post.py",
                         "tools/formal/fraction_verus_complement_lemmas.rs",
                         "tools/formal/verify_fraction_complement_post.py",
                         "tools/formal/test_verify_fraction_complement_post.py"} <= names)
        command = BRIDGE.PILOT.command(Path("/pinned/payload"), SOURCE, "/work/trace")
        self.assertIn("--no-cheating", command)
        self.assertIn("timeout=60000", command)
        self.assertIn("--num-threads=1", command)

    def test_five_controls_change_only_named_lemma_or_ghost_segments_and_restore_all_bytes(self):
        fragments = {
            3: (b"Err(FixtureEnclosureError::Invariant) => !required_success,",
                b"Err(FixtureEnclosureError::Invariant) => true,"),
            4: (b"&& value.shift == input.shift", b"&& value.shift >= input.shift"),
            5: (b"&& (value.numerator as int) + (input.numerator as int) == denominator",
                b"&& (value.numerator as int) + (input.numerator as int) <= denominator"),
        }
        prefix = (self.generated / "predicate_source.rs").read_bytes()
        fragment = prefix[:1202] + prefix[2684:3731]
        executable = BRIDGE.PROJECTION.annotated_complement_post(fragment)
        segment = self.ghost[self.ghost.index(b"spec fn bridge_complement_post("):]
        for index, (old, new) in fragments.items():
            job = self.job(index)
            changed = BRIDGE.control_bytes(self.healthy, self.ghost, job, _recipe=RECIPE)
            self.assertNotEqual(changed, self.healthy)
            faulty_segment = segment.replace(old, new, 1)
            self.assertEqual(changed, self.healthy.replace(segment, faulty_segment, 1))
            self.assertEqual(changed.replace(faulty_segment, segment, 1), self.healthy)
            self.assertEqual(changed.count(executable), 1)
            self.assertEqual(BRIDGE.PROJECTION.erase_complement_post_annotations(executable), fragment)
            self.assertEqual(changed.count((BRIDGE.ROOT / BRIDGE.PROJECTION.BIT_LENGTH_LEMMAS).read_bytes()), 1)
        self.assertEqual(BRIDGE.control_bytes(self.healthy, self.ghost, self.job(6), _recipe=RECIPE), self.healthy)
        with self.assertRaisesRegex(RuntimeError, "control identity"):
            BRIDGE.control_bytes(self.healthy, self.ghost, {"id": "other", "target": None}, _recipe=RECIPE)
        for index in range(1, 6):
            job = self.job(index)
            with self.subTest(index=index), self.assertRaisesRegex(RuntimeError, "control identity"):
                BRIDGE.control_bytes(self.healthy, self.ghost,
                    {**job, "target": "proof::composition_checked"}, _recipe=RECIPE)

    def test_strengthened_lemma_controls_preserve_their_original_guarded_facts(self):
        theorem_pairs = (
            (1, b"    ensures\n        (1 <= s && s <= 53)\n"
                b"            ==> ((1_u128 << s) as int == pow(2, s as nat)),\n",
                b"    ensures\n        ((1 <= s && s <= 53)\n"
                b"            ==> ((1_u128 << s) as int == pow(2, s as nat)))\n"
                b"            && s != 1,\n"),
            (2, b"    ensures\n"
                b"        (bridge_checked(Fraction { numerator: n, shift: s }) && (s == 0 || s > 53))\n"
                b"            ==> (2 * (n as int) < bridge_denominator(s)),\n",
                b"    ensures\n"
                b"        ((bridge_checked(Fraction { numerator: n, shift: s }) && (s == 0 || s > 53))\n"
                b"            ==> (2 * (n as int) < bridge_denominator(s)))\n"
                b"            && (n != 0 || s != 0),\n"),
        )
        for index, old, strengthened in theorem_pairs:
            changed = BRIDGE.control_bytes(self.healthy, self.ghost, self.job(index), _recipe=RECIPE)
            self.assertEqual(changed, self.healthy.replace(old, strengthened, 1))
            self.assertEqual(changed.replace(strengthened, old, 1), self.healthy)
            self.assertEqual(changed.count(self.ghost), 1)

    def test_new_proof_headers_bind_literal_ensures_and_own_guarded_body(self):
        spans = BRIDGE.function_spans(self.healthy, _recipe=RECIPE)
        outside = spans["proof::lemma_bridge_complement_reject_outside"]
        lines = self.healthy.splitlines()
        self.assertIn(b"Fraction { numerator: n, shift: s }", lines[outside["clause_start"] - 1])
        self.assertIn(b"if bridge_checked(Fraction", lines[outside["return_start"] - 1])
        self.assertGreater(outside["return_start"], outside["clause_end"])
        for old, new in ((BRIDGE._SHIFT_THEOREM, BRIDGE._SHIFT_THEOREM.replace(b"<= 53", b"<= 54")),
                         (BRIDGE._OUTSIDE_THEOREM, BRIDGE._OUTSIDE_THEOREM + b"    decreases s,\n"),
                         (b"    if 1 <= s && s <= 53 {\n", b"    if 1 <= s && s <= 52 {\n")):
            with self.subTest(new=new), self.assertRaises(RuntimeError):
                BRIDGE.function_spans(self.healthy.replace(old, new, 1), _recipe=RECIPE)

    def test_metadata_recomputed_from_sources_including_lengths_domains_modes_and_hooks(self):
        BRIDGE.validate_metadata(self.details, BRIDGE.ROOT, self.generated, _recipe=RECIPE)
        mutations = {"schema_version": True, "proof_commands": False, "status": "proved",
            "scope": "narrow", "source_files_sha256": {}, "original_sources_sha256": {},
            "correspondence": [], "prefix_sha256": "bad", "prefix_bytes": 3730,
            "fragment_source_spans": [], "fragment_sha256": "bad", "complement_lemma_sha256": "bad",
            "annotated_fragment_sha256": "bad", "ghost_sha256": "bad", "lemma_sha256": "bad",
            "predicate_relations": [], "full_bridge_predicate_relations": [],
            "expected_function_modes": {**MODES, "proof::lemma_bridge_bit_length_power": "exec"},
            "raw_domains": {"lemma": "n > 0"}, "proof_hooks": [], "upstream_dependencies": {},
            "remaining": []}
        metadata = self.generated / "correspondence.json"
        for field, value in mutations.items():
            changed = deepcopy(self.details)
            changed[field] = value
            metadata.write_text(json.dumps(changed))
            with self.subTest(field=field), self.assertRaises(RuntimeError):
                BRIDGE.validate_metadata(changed, BRIDGE.ROOT, self.generated, _recipe=RECIPE)
        metadata.write_text(json.dumps(self.details))
        BRIDGE.validate_metadata(self.details, BRIDGE.ROOT, self.generated, _recipe=RECIPE)

    def test_metadata_rejects_missing_extra_claims_and_boolean_numeric_aliases(self):
        for mutation in ("missing_remaining", "unknown_claim", "bridge_integer", "offset_float", "hook_float",
                         "fragment_bool", "fragment_float", "fragment_extra", "fragment_order", "prefix_float"):
            changed = deepcopy(self.details)
            if mutation == "missing_remaining":
                del changed["remaining"]
            elif mutation == "unknown_claim":
                changed["proved"] = True
            elif mutation == "bridge_integer":
                changed["correspondence"][0]["bridge_type"] = int(changed["correspondence"][0]["bridge_type"])
            elif mutation == "offset_float":
                changed["correspondence"][0]["byte_offsets_half_open"][0] = float(
                    changed["correspondence"][0]["byte_offsets_half_open"][0])
            elif mutation == "hook_float":
                changed["proof_hooks"][0]["source_byte_offset"] = float(
                    changed["proof_hooks"][0]["source_byte_offset"])
            elif mutation == "fragment_bool":
                changed["fragment_source_spans"][0]["source_byte_offsets_half_open"][0] = False
            elif mutation == "fragment_float":
                changed["fragment_source_spans"][1]["source_byte_offsets_half_open"][1] = 3731.0
            elif mutation == "fragment_extra":
                changed["fragment_source_spans"][0]["proved"] = True
            elif mutation == "fragment_order":
                changed["fragment_source_spans"].reverse()
            else:
                changed["prefix_bytes"] = 3731.0
            (self.generated / "correspondence.json").write_text(json.dumps(changed))
            with self.subTest(mutation=mutation), self.assertRaises(RuntimeError):
                BRIDGE.validate_metadata(changed, BRIDGE.ROOT, self.generated, _recipe=RECIPE)

    def test_raw_complement_domain_and_four_relations_cannot_be_narrowed_or_relabelled(self):
        expected = ("every raw Fraction { numerator:u64, shift:u32 } and independently chosen "
                    "&Result<Fraction, FixtureEnclosureError>; raw Ok fields and every error variant; no precondition")
        self.assertEqual(self.details["raw_domains"]["complement_post"], expected)
        self.assertEqual(len(self.details["predicate_relations"]), 4)
        self.assertEqual(self.details["predicate_relations"][-1], "bridge_complement_post(input, result)")
        for mutation in ("constructor_result", "checked_ok_only", "missing_domain", "extra_domain", "three_relations",
                         "shift_requires", "outside_requires"):
            changed = deepcopy(self.details)
            if mutation == "constructor_result":
                changed["raw_domains"]["complement_post"] = "only Fraction::new-produced results"
            elif mutation == "checked_ok_only":
                changed["raw_domains"]["complement_post"] = "only Ok(checked Fraction); excludes errors"
            elif mutation == "missing_domain":
                del changed["raw_domains"]["complement_post"]
            elif mutation == "extra_domain":
                changed["raw_domains"]["precondition"] = "checked input"
            elif mutation == "shift_requires":
                changed["raw_domains"]["shift_power"] = "requires 1 <= s <= 53"
            elif mutation == "outside_requires":
                changed["raw_domains"]["reject_outside"] = "requires checked Fraction"
            else:
                changed["predicate_relations"].pop()
            (self.generated / "correspondence.json").write_text(json.dumps(changed))
            with self.subTest(mutation=mutation), self.assertRaises(RuntimeError):
                BRIDGE.validate_metadata(changed, BRIDGE.ROOT, self.generated, _recipe=RECIPE)

    def test_generated_artifacts_cannot_be_rebound_by_matching_tampered_metadata(self):
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
        (self.generated / "correspondence.json").write_text(json.dumps(self.details))

    def test_function_mode_inventory_rejects_missing_extra_duplicate_and_wrong_status(self):
        job = self.job()
        for mutation in ("missing", "extra", "duplicate", "lemma_exec", "shift_exec", "outside_exec",
                         "predicate_proof", "constructor_proof", "complement_proof", "clone_short",
                         "mode_key", "status_int"):
            value = report(job)
            rows = value["times-ms"]["smt"]["smt-run-module-times"][0]["function-breakdown"]
            if mutation == "missing":
                rows.pop()
            elif mutation == "extra":
                rows.append({"function": "proof::other", "mode:": "exec", "success": True})
            elif mutation == "duplicate":
                rows.append(deepcopy(rows[0]))
            elif mutation == "lemma_exec":
                rows[0]["mode:"] = "exec"
            elif mutation == "shift_exec":
                rows[1]["mode:"] = "exec"
            elif mutation == "outside_exec":
                rows[2]["mode:"] = "exec"
            elif mutation == "predicate_proof":
                rows[3]["mode:"] = "proof"
            elif mutation == "constructor_proof":
                rows[5]["mode:"] = "proof"
            elif mutation == "complement_proof":
                rows[6]["mode:"] = "proof"
            elif mutation == "clone_short":
                rows[-1]["function"] = "clone"
            elif mutation == "mode_key":
                rows[0]["mode"] = rows[0].pop("mode:")
            else:
                rows[0]["success"] = 1
            with self.subTest(mutation=mutation):
                self.rejected(native(value, job), job)

    def test_query_flags_minima_and_counts_are_typed_and_independent_of_diagnostic_count(self):
        for index in (0, 1):
            job = self.job(index)
            for field, value in (("verified", True), ("verified", 8 if index else 9),
                                 ("errors", bool(index)), ("is-verifying-entire-crate", False),
                                 ("encountered-vir-error", True)):
                changed = report(job)
                changed["verification-results"][field] = value
                with self.subTest(index=index, field=field):
                    self.rejected(native(changed, job), job)
        job = self.job()
        changed = report(job)
        changed["verification-results"]["verified"] = 12
        self.assertTrue(self.qualify(native(changed, job), job)["qualified"])

    def test_all_five_faults_accept_both_diagnostic_orientations_and_body_end(self):
        for index in range(1, 6):
            job = self.job(index)
            spans = job["spans"][job["target"]]
            last_clause = spans.get("clause_end", spans.get("returns_line"))
            self.assertLess(last_clause, spans["return_start"])
            self.assertLess(spans["return_start"], spans["return_end"])
            for primary in (True, False):
                for line in (None, spans["return_end"]):
                    with self.subTest(index=index, primary=primary, line=line):
                        self.assertTrue(self.qualify(native(report(job), job,
                            [diagnostic(job, primary_clause=primary, exit_line=line)]), job)["qualified"])

    def test_proof_diagnostics_reject_foreign_malformed_unlinked_or_outside_spans(self):
        job = self.job(1)
        for mutation in ("foreign", "keyword", "label", "no_body", "no_clause", "no_primary",
                         "outside", "malformed", "assertion", "termination", "whole_function"):
            error = diagnostic(job)
            if mutation == "foreign":
                error["spans"][1]["file_name"] = "/work/other.rs"
            elif mutation == "keyword":
                error["spans"][0]["line_start"] -= 1
            elif mutation == "label":
                error["spans"][0]["label"] = "other"
            elif mutation == "no_body":
                error["spans"].pop()
            elif mutation == "no_clause":
                error["spans"].pop(0)
            elif mutation == "no_primary":
                for span in error["spans"]:
                    span["is_primary"] = False
            elif mutation == "outside":
                error["spans"].append({**error["spans"][0], "line_start": 1, "line_end": 1})
            elif mutation == "malformed":
                error["spans"].append({"is_primary": False})
            elif mutation == "whole_function":
                error["spans"][1]["line_start"] = job["spans"][job["target"]]["clause_start"] - 2
            else:
                error["message"] = "assertion failed" if mutation == "assertion" else "decreases not satisfied"
            with self.subTest(mutation=mutation):
                self.rejected(native(report(job), job, [error]), job)

    def test_multiple_distinct_same_obligation_errors_require_exact_abort_accounting(self):
        job = self.job(1)
        whole = diagnostic(job)
        exit_error = diagnostic(job, exit_line=job["spans"][job["target"]]["return_end"])
        abort = {"$message_type": "diagnostic", "level": "error", "message": "aborting due to 2 previous errors",
                 "spans": []}
        self.assertTrue(self.qualify(native(report(job), job, [whole, exit_error, abort]), job)["qualified"])
        dependency_job = {**job, "target": "proof::composition_checked"}
        for errors in ([whole, deepcopy(whole), abort], [whole, exit_error, {**abort, "message":
                       "aborting due to 1 previous error"}], [whole, exit_error, abort, abort],
                       [whole, diagnostic(dependency_job), abort]):
            self.rejected(native(report(job), job, errors), job)
        changed = report(job)
        changed["times-ms"]["smt"]["smt-run-module-times"][0]["function-breakdown"][0]["success"] = False
        self.rejected(native(changed, job), job)

    def test_strict_output_identity_custody_and_unknown_objects_remain_fail_closed(self):
        job = self.job()
        good = native(report(job), job)
        for output in ("", good["output"][:-2], good["output"] + "\n" + good["output"],
                       good["output"].replace('"verified": 10', '"verified": 10, "verified": 10'),
                       good["output"] + '\n{"unknown": true}', "unparsed\n" + good["output"]):
            self.rejected({**good, "output": output}, job)
        for change in ({"failure": "timeout"}, {"exit_code": True}, {"exit_code": 1}):
            self.rejected({**good, **change}, job)
        for field in BRIDGE.EXPECTED_IDENTITY:
            value = report(job)
            value["verus"][field] = "other"
            self.rejected(native(value, job), job)

    def test_distinct_original_clock_rejects_expiry_rollback_and_boot_change(self):
        self.assertEqual(RECIPE.started_unix_ns, 1793935038999026301)
        self.assertEqual(RECIPE.started_monotonic_ns, 31706832453750)
        self.assertEqual(RECIPE.deadline_monotonic_ns, 35306832453750)
        self.assertNotEqual(RECIPE.work, BRIDGE.WORK)
        with patch.object(Path, "read_text", return_value=RECIPE.boot_id), \
             patch.object(BRIDGE.time, "monotonic_ns", return_value=RECIPE.started_monotonic_ns + 10**9), \
             patch.object(BRIDGE.time, "time_ns", return_value=RECIPE.started_unix_ns + 10**9):
            self.assertEqual(BRIDGE.remaining(_recipe=RECIPE), 3599)
            with patch.object(BRIDGE.time, "monotonic_ns", return_value=RECIPE.deadline_monotonic_ns):
                with self.assertRaisesRegex(RuntimeError, "clock"):
                    BRIDGE.remaining(_recipe=RECIPE)
            with patch.object(BRIDGE.time, "time_ns", return_value=RECIPE.started_unix_ns - 1):
                with self.assertRaisesRegex(RuntimeError, "clock"):
                    BRIDGE.remaining(_recipe=RECIPE)
            with patch.object(Path, "read_text", return_value="other"):
                with self.assertRaisesRegex(RuntimeError, "boot"):
                    BRIDGE.remaining(_recipe=RECIPE)

    def test_new_recipe_preserves_frozen_permissions_hardlink_path_and_capacity_guards(self):
        island = self.root / "island"
        island.mkdir()
        value = island / "data"
        value.write_bytes(b"copy")
        with patch.object(BRIDGE, "_work", return_value=self.root):
            BRIDGE.seal(island, _recipe=RECIPE)
        before = BRIDGE.tree(island)
        self.assertEqual(before["data"]["mode"], 0o400)
        os.chmod(value, 0o600)
        with self.assertRaisesRegex(RuntimeError, "permission"):
            BRIDGE.tree(island)
        alias = self.root / "alias"
        os.link(value, alias)
        with self.assertRaisesRegex(RuntimeError, "hardlink"):
            BRIDGE.identity(value)
        alias.unlink()
        redirect = self.root / "redirect"
        redirect.symlink_to(island, target_is_directory=True)
        with patch.object(BRIDGE, "_work", return_value=self.root), self.assertRaisesRegex(RuntimeError, "redirect"):
            BRIDGE.write(redirect / "bad.json", {}, _recipe=RECIPE)
        redirect.unlink()
        with patch.object(BRIDGE, "remaining", return_value=100), patch.object(BRIDGE.shutil, "disk_usage") as usage:
            usage.return_value.free = BRIDGE.RESERVE_BYTES - 1
            with self.assertRaisesRegex(RuntimeError, "reserve"):
                BRIDGE.capacity(self.root, _recipe=RECIPE)
            usage.return_value.free = BRIDGE.RESERVE_BYTES
            with patch.object(BRIDGE, "MAXIMUM_BYTES", 0), self.assertRaisesRegex(RuntimeError, "retained"):
                BRIDGE.capacity(self.root, _recipe=RECIPE)

    def test_frozen_preparation_retains_all_original_artifacts_and_byte_identical_restoration(self):
        work = self.root / "preparation"
        work.mkdir(mode=0o700)
        with patch.object(BRIDGE, "_work", return_value=work):
            BRIDGE.freeze_sources(work, _recipe=RECIPE)
            planned, inputs = BRIDGE.prepare_inputs(work, _recipe=RECIPE)
        self.assertEqual(len(planned), 7)
        self.assertEqual((planned[0]["inputs"] / "proof.rs").read_bytes(),
                         (planned[-1]["inputs"] / "proof.rs").read_bytes())
        for job in planned:
            self.assertEqual((job["inputs"] / "predicate_source.rs").stat().st_size, 3731)
            self.assertEqual((job["inputs"] / "actual_source.rs").read_bytes(),
                             (self.generated / "actual_source.rs").read_bytes())
            self.assertEqual(set(job["spans"]), set(MODES) - {name for name in MODES if "::clone" in name})
        self.assertEqual(inputs["01-healthy/proof.rs"]["mode"], 0o400)

    def test_first_failed_native_job_retains_receipt_and_never_dispatches_later_jobs(self):
        work = self.root / "attempt"
        payload = self.root / "payload"
        payload.mkdir()
        (payload / "version.json").write_text(json.dumps({"verus": BRIDGE.EXPECTED_IDENTITY}))
        stable = {"readonly_roots": [self.root / "sdk"], "admission_profile": "stable-core"}
        observed = ["/usr/bin/env", *(str(payload / name) for name in ("verus", "rust_verify", "z3"))]
        failure = {"failure": None, "exit_code": 1, "output": '{"$message_type":"diagnostic",'
                   '"level":"error","message":"unrelated compiler error","spans":[]}'}
        with patch.object(BRIDGE, "_work", return_value=work), \
             patch.object(BRIDGE, "remaining", return_value=100), \
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
        self.assertEqual(summary["maximum_commands"], 7)
        self.assertEqual(summary["started_unix_ns"], 1793935038999026301)
        self.assertEqual(len(summary["jobs"]), 1)
        self.assertEqual(summary["jobs"][0]["native"], failure)
        self.assertTrue(summary["source_input_receipt_bookends_unchanged"])
        self.assertEqual((work / "jobs/01-healthy/execution.json").stat().st_mode & 0o777, 0o400)
        self.assertFalse((work / "jobs/02-shift_wrong_theorem").exists())
        options = run.call_args.kwargs
        self.assertEqual((options["cpu_seconds"], options["address_space_bytes"], options["build_jobs"],
                          options["process_limit"]), (180, 8 * 1024**3, 1, 128))
        self.assertEqual(options["timeout"], 100)

    def test_only_new_recipe_has_seven_jobs_and_shared_loop_completes_all_seven(self):
        self.assertEqual(len(BRIDGE._jobs(None)), 5)
        self.assertEqual(len(BRIDGE._jobs(BRIDGE._BIT_LENGTH_RECIPE)), 5)
        self.assertEqual(len(BRIDGE._jobs(BRIDGE._CONSTRUCTOR_POST_RECIPE)), 5)
        work = self.root / "simulated-complete"

        def qualify_without_native(current_work, state, job, record, *, _recipe):
            self.assertIs(_recipe, RECIPE)
            self.assertEqual(current_work, work)
            record["qualified"] = True

        with patch.object(BRIDGE, "_work", return_value=work), \
             patch.object(BRIDGE, "remaining", return_value=100), \
             patch.object(BRIDGE, "capacity", return_value=4096), \
             patch.object(BRIDGE, "execute_job", side_effect=qualify_without_native) as execute, \
             patch.object(BRIDGE.PILOT, "admit") as admit, patch("sys.stdout", new_callable=io.StringIO):
            self.assertEqual(BRIDGE.run_campaign(_recipe=RECIPE), 0)
        self.assertEqual(execute.call_count, 7)
        admit.assert_not_called()
        summary = json.loads((work / "summary.json").read_text())
        self.assertEqual(summary["status"], "complement-post-dependency-checkpoint-passed")
        self.assertEqual(summary["maximum_commands"], 7)
        self.assertEqual(len(summary["jobs"]), 7)

    def test_reused_fixed_root_rejects_before_native_or_admission(self):
        with patch.object(BRIDGE, "_work", return_value=self.root), \
             patch.object(BRIDGE, "remaining", return_value=100), patch.object(BRIDGE.PILOT, "admit") as admit:
            with self.assertRaisesRegex(RuntimeError, "fresh fixed"):
                BRIDGE.run_campaign(_recipe=RECIPE)
            admit.assert_not_called()



if __name__ == "__main__":
    unittest.main()
