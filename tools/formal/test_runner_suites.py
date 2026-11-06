"""Additional suites must reach the real driver's inventory and result checks."""
import copy
from contextlib import redirect_stderr
from pathlib import Path
import importlib.util
import io
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch


def load(name, filename):
    spec = importlib.util.spec_from_file_location(name, Path(__file__).with_name(filename))
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


VERIFY = load("formal_extended_driver_test", "verify.py")
SUITES = load("formal_extended_registry_test", "additional_suites.py")
FIXTURE = load("formal_extended_json_fixture", "test_verify.py")


class RunnerSuiteTests(unittest.TestCase):
    def test_restoration_identity_covers_every_relative_generated_and_frozen_input(self):
        check = SimpleNamespace(read_once=lambda path: path.read_bytes())
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            first, restored = root / "first", root / "restored"
            for base in (first, restored):
                (base / "frozen").mkdir(parents=True)
                (base / "proof.rs").write_bytes(b"mod actual_source;\n")
                (base / "actual_source.rs").write_bytes(b"fn actual_body() {}\n")
                (base / "frozen/tool.py").write_bytes(b"# pinned preparation input\n")
            initial = VERIFY.generated_identity(check, first)
            self.assertEqual(initial, VERIFY.generated_identity(check, restored))
            for relative in ("proof.rs", "actual_source.rs", "frozen/tool.py"):
                path = restored / relative
                original = path.read_bytes()
                path.write_bytes(original + b"changed\n")
                self.assertNotEqual(initial, VERIFY.generated_identity(check, restored), relative)
                path.write_bytes(original)
            (restored / "extra-input.rs").write_bytes(b"// unexpected generated input\n")
            self.assertNotEqual(initial, VERIFY.generated_identity(check, restored))
            (restored / "extra-input.rs").unlink()
            self.assertEqual(initial, VERIFY.generated_identity(check, restored))
            (restored / "proof.rs").unlink()
            (restored / "proof.rs").symlink_to(first / "proof.rs")
            with self.assertRaises(VERIFY.Incomplete):
                VERIFY.generated_identity(check, restored)
            empty = root / "empty"
            empty.mkdir()
            with self.assertRaises(VERIFY.Incomplete):
                VERIFY.generated_identity(check, empty)

    def test_partition_budget_is_finite_original_and_inapplicable_to_old_subjects(self):
        for value in (0, -1, 3600.01, float("nan"), float("inf"), True, None, "100"):
            fake = SimpleNamespace(remaining_budget=lambda value=value: value)
            with self.subTest(value=value), patch.object(VERIFY, "partition_projection", return_value=fake):
                with self.assertRaises(VERIFY.Incomplete):
                    VERIFY.partition_remaining("fraction-addition-partitions")
        for value in (0.25, 180, 3600):
            fake = SimpleNamespace(remaining_budget=lambda value=value: value)
            with self.subTest(value=value), patch.object(VERIFY, "partition_projection", return_value=fake):
                self.assertEqual(VERIFY.partition_remaining("fraction-addition-partitions"), float(value))
        with patch.object(VERIFY, "partition_projection", side_effect=ValueError("expired original card")):
            with self.assertRaises(VERIFY.Incomplete):
                VERIFY.partition_remaining("fraction-addition-partitions")
        with patch.object(VERIFY, "partition_projection") as projection:
            self.assertIsNone(VERIFY.partition_remaining("all"))
            self.assertIsNone(VERIFY.partition_remaining("fraction-arithmetic"))
            projection.assert_not_called()

    def test_partition_preflight_requires_original_profile_solver_and_fixed_output(self):
        selector = "fraction-addition-partitions"
        output = str(Path.home() / ".local/share/nevrotick/formal-core-fraction-addition-partitions-20261106-v1")
        with patch.object(VERIFY, "partition_remaining", return_value=100) as budget:
            VERIFY.partition_preflight(selector, "kani-proof", "cadical", None, output)
            budget.assert_called_once_with(selector)
        for profile, solver, root, destination in (
                ("full", "cadical", None, output), ("kani-proof", "z3", None, output),
                ("kani-proof", "cadical", "/tmp", output),
                ("kani-proof", "cadical", None, output + "-retry"),
                ("kani-proof", "cadical", None, "/tmp/formal-core-fraction-addition-partitions-20261106-v1")):
            with self.subTest(profile=profile, solver=solver, root=root, output=destination):
                with patch.object(VERIFY, "partition_remaining") as budget, self.assertRaises(VERIFY.Incomplete):
                    VERIFY.partition_preflight(selector, profile, solver, root, destination)
                budget.assert_not_called()
        with patch.object(VERIFY, "partition_remaining") as budget:
            VERIFY.partition_preflight("clock", "full", "z3", "/tmp", "/tmp/original-output")
            budget.assert_not_called()

    def test_partition_cli_checks_restrictions_before_creating_any_output(self):
        class StopBeforeOutput(RuntimeError):
            pass
        output = str(Path.home() / ".local/share/nevrotick/formal-core-fraction-addition-partitions-20261106-v1")
        legal = ["--subject", "fraction-addition-partitions", "--output-dir", output,
                 "--tool-admission-profile", "kani-proof"]
        with patch.object(VERIFY, "partition_remaining", return_value=100), \
             patch.object(VERIFY.os, "umask"), \
             patch.object(VERIFY, "fresh_output", side_effect=StopBeforeOutput) as fresh:
            with self.assertRaises(StopBeforeOutput):
                VERIFY.main(legal)
            fresh.assert_called_once_with(output)
        attempts = [legal + ["--start-at", "2"], legal + ["--only-jobs", "1"],
                    legal + ["--solver-backend", "z3"],
                    legal[:-1] + ["full"], legal[:3] + [output + "-retry"] + legal[4:]]
        for argv in attempts:
            with self.subTest(argv=argv), patch.object(VERIFY, "partition_remaining", return_value=100), \
                 patch.object(VERIFY, "fresh_output") as fresh, redirect_stderr(io.StringIO()):
                with self.assertRaises(SystemExit) as stopped:
                    VERIFY.main(argv)
                self.assertEqual(stopped.exception.code, 2)
                fresh.assert_not_called()

    def test_partitions_have_one_closed_sixteen_job_schedule_and_actual_source_closure(self):
        selector = "fraction-addition-partitions"
        selected = VERIFY.selected_jobs(selector, 1)
        self.assertEqual([index for index, _ in selected], list(range(1, 17)))
        self.assertEqual([job for _, job in selected], SUITES.jobs(selector))
        self.assertEqual(VERIFY.MAX_JOBS, 12)
        self.assertEqual(SUITES.MAX_JOBS, 12)
        self.assertEqual([index for index, job in selected if job["restored_healthy"]], list(range(11, 17)))
        self.assertEqual(len(VERIFY.jobs("all")), 12)
        self.assertTrue(set(VERIFY.input_names("all")).isdisjoint({
            "tools/formal/fraction_addition_partitions_projection.py",
            "tools/formal/fraction_addition_partitions_harness.rs"}))
        names = set(VERIFY.input_names(selector))
        self.assertTrue(set(SUITES.input_names(selector)).issubset(names))
        self.assertTrue({"crates/time-core/Cargo.toml", "crates/time-core/src/projection_enclosure.rs",
                         "crates/time-core/src/selection.rs"}.issubset(names))
        self.assertNotIn("tools/formal/fraction_contracts_spec.rs", names)

    def test_partition_subject_refuses_suffix_and_explicit_subset_without_changing_old_selection(self):
        selector = "fraction-addition-partitions"
        for start, only in ((0, None), (2, None), (16, None), (17, None), (True, None),
                            (1, []), (1, [1]), (1, [16]), (1, list(range(1, 17)))):
            with self.subTest(start=start, only=only), self.assertRaises(VERIFY.Incomplete):
                VERIFY.selected_jobs(selector, start, only)
        self.assertEqual(len(VERIFY.selected_jobs(selector, 1)), 16)
        self.assertEqual([index for index, _ in VERIFY.selected_jobs("clock", 2)], list(range(2, 11)))
        self.assertEqual([index for index, _ in VERIFY.selected_jobs("clock", 1, [3, 1])], [1, 3])

    def test_sixteen_job_exception_cannot_broaden_any_existing_subject_or_accept_partial_partition_schedule(self):
        ordinary = VERIFY.jobs("clock")
        partitions = VERIFY.jobs("fraction-addition-partitions")
        for count in (0, 1, 12, 15, 17):
            fake = SimpleNamespace(jobs=lambda selector, count=count: (partitions * 2)[:count])
            with self.subTest(count=count), patch.object(VERIFY, "additional_suites", return_value=fake):
                with self.assertRaises(VERIFY.Incomplete):
                    VERIFY.jobs("fraction-addition-partitions")
        fake = SimpleNamespace(jobs=lambda selector: (ordinary * 2)[:13])
        with patch.object(VERIFY, "additional_suites", return_value=fake):
            with self.assertRaises(VERIFY.Incomplete):
                VERIFY.jobs("clock")
        self.assertEqual(len(VERIFY.jobs("fraction-addition-partitions")), 16)

    def test_partition_results_require_their_complete_reachable_assertion_and_cover_ledger(self):
        for job in SUITES.jobs("fraction-addition-partitions")[:7]:
            assertions, covers = SUITES.expected_contract(job["subject"], job["harness"])
            checks = [{"category": "assertion", "description": name, "status": "Success",
                       "location": {"file": FIXTURE.ENTRY}} for name in sorted(assertions)]
            checks += [{"category": "cover", "description": name, "status": "Satisfied"}
                       for name in sorted(covers)]
            native = {"exit_code": 0, "failure": None}
            original = FIXTURE.result(job, checks, True)
            with self.subTest(harness=job["harness"]):
                VERIFY.qualify(original, native, job, FIXTURE.ENTRY)
            for alteration in ("missing assertion", "unreachable assertion", "wrong cover", "unsatisfied cover"):
                report = copy.deepcopy(original)
                modified = report["verification_results"]["results"][0]["checks"]
                if alteration == "missing assertion":
                    del modified[0]
                elif alteration == "unreachable assertion":
                    modified[0]["status"] = "Unreachable"
                elif alteration == "wrong cover":
                    modified[-1]["description"] = "an unrelated reachable cover"
                else:
                    modified[-1]["status"] = "Unsatisfiable"
                FIXTURE.recount(report)
                with self.subTest(harness=job["harness"], alteration=alteration), self.assertRaises(VERIFY.Incomplete):
                    VERIFY.qualify(report, native, job, FIXTURE.ENTRY)

    def test_representation_completes_signed_conversion_before_bit_decoding(self):
        selected = VERIFY.selected_jobs("scaled-value-representation", 1)
        self.assertEqual([i for i, _ in selected], list(range(1, 11)))
        jobs = [job for _, job in selected]
        self.assertEqual([job["subject"] for job in jobs], ["signed_whole"] * 5 + ["from_bits"] * 5)
        self.assertEqual([job["case"] for job in jobs], [
            "healthy", "omit_negation", "narrow_magnitude", "use_fraction", "healthy",
            "healthy", "wrong_subnormal_scale", "lose_sign", "wrong_error", "healthy"])
        self.assertEqual([i for i, job in selected if job["restored_healthy"]], [5, 10])
        names = VERIFY.input_names("scaled-value-representation")
        self.assertTrue({"tools/formal/scaled_value_representation_harness.rs",
                         "tools/formal/scaled_value_representation_projection.py",
                         "tools/formal/test_scaled_value_representation.py",
                         "crates/time-core/src/projection_enclosure.rs",
                         "crates/time-core/src/selection.rs"}.issubset(names))
        self.assertNotIn("tools/formal/verus_compatibility.rs", names)

    def test_split_freezes_full_raw_source_and_three_semantic_controls(self):
        selected = VERIFY.selected_jobs("scaled-value-split", 1)
        self.assertEqual([i for i, _ in selected], [1, 2, 3, 4, 5])
        jobs = [job for _, job in selected]
        self.assertEqual([job["case"] for job in jobs], [
            "healthy", "reject_all", "zero_remainder", "wrong_error", "healthy"])
        self.assertEqual([job["restored_healthy"] for job in jobs], [False] * 4 + [True])
        self.assertTrue(all(job["subject"] == "scaled_value_split" for job in jobs))
        names = VERIFY.input_names("scaled-value-split")
        self.assertTrue({"tools/formal/scaled_value_split_harness.rs",
                         "tools/formal/scaled_value_split_projection.py",
                         "tools/formal/test_scaled_value_split.py",
                         "crates/time-core/src/projection_enclosure.rs",
                         "crates/time-core/src/selection.rs"}.issubset(names))
        self.assertNotIn("tools/formal/verus_compatibility.rs", names)

    def test_fraction_arithmetic_completes_complement_before_addition(self):
        selected = VERIFY.selected_jobs("fraction-arithmetic", 1)
        self.assertEqual([i for i, _ in selected], list(range(1, 11)))
        jobs = [job for _, job in selected]
        self.assertEqual([job["subject"] for job in jobs],
                         ["fraction_complement"] * 5 + ["fraction_addition"] * 5)
        self.assertEqual([job["case"] for job in jobs], [
            "healthy", "always_error", "wrong_numerator", "wrong_error", "healthy",
            "healthy", "always_error", "wrong_carry", "wrong_residual", "healthy"])
        self.assertEqual([i for i, job in selected if job["restored_healthy"]], [5, 10])
        names = VERIFY.input_names("fraction-arithmetic")
        self.assertTrue({"tools/formal/fraction_arithmetic_harness.rs",
                         "tools/formal/fraction_arithmetic_projection.py",
                         "tools/formal/test_fraction_arithmetic.py",
                         "crates/time-core/src/projection_enclosure.rs",
                         "crates/time-core/src/selection.rs"}.issubset(names))
        self.assertNotIn("tools/formal/era_constructor_harness.rs", names)

    def test_fraction_comparison_freezes_its_source_and_three_semantic_controls(self):
        selected = VERIFY.selected_jobs("fraction-comparison", 1)
        self.assertEqual([i for i, _ in selected], [1, 2, 3, 4, 5])
        jobs = [job for _, job in selected]
        self.assertEqual([job["case"] for job in jobs], [
            "healthy", "always_error", "reverse_unequal", "zero_alignment", "healthy"])
        self.assertEqual([job["restored_healthy"] for job in jobs], [False] * 4 + [True])
        self.assertTrue(all(job["subject"] == "fraction_comparison" for job in jobs))
        names = VERIFY.input_names("fraction-comparison")
        self.assertTrue({"tools/formal/fraction_comparison_harness.rs",
                         "tools/formal/fraction_comparison_projection.py",
                         "tools/formal/test_fraction_comparison.py",
                         "crates/time-core/Cargo.toml",
                         "crates/time-core/src/projection_enclosure.rs",
                         "crates/time-core/src/selection.rs"}.issubset(names))
        self.assertNotIn("tools/formal/era_constructor_harness.rs", names)

    def test_fraction_constructor_freezes_core_sources_and_restores_after_three_defects(self):
        selected = VERIFY.selected_jobs("fraction-constructor", 1)
        self.assertEqual([i for i, _ in selected], [1, 2, 3, 4, 5])
        jobs = [job for _, job in selected]
        self.assertEqual([job["case"] for job in jobs], [
            "healthy", "always_error", "wrong_numerator", "wrong_error", "healthy"])
        self.assertEqual([job["restored_healthy"] for job in jobs], [False] * 4 + [True])
        self.assertTrue(all(job["subject"] == "fraction_constructor" for job in jobs))
        names = VERIFY.input_names("fraction-constructor")
        self.assertTrue({"tools/formal/fraction_constructor_harness.rs",
                         "tools/formal/fraction_constructor_projection.py",
                         "tools/formal/test_fraction_constructor.py",
                         "crates/time-core/Cargo.toml",
                         "crates/time-core/src/projection_enclosure.rs",
                         "crates/time-core/src/selection.rs"}.issubset(names))
        self.assertNotIn("tools/formal/era_constructor_harness.rs", names)

    def test_endpoint_finish_freezes_core_sources_and_restores_after_three_defects(self):
        selected = VERIFY.selected_jobs("endpoint-finish", 1)
        self.assertEqual([i for i, _ in selected], [1, 2, 3, 4, 5])
        jobs = [job for _, job in selected]
        self.assertEqual([job["case"] for job in jobs], [
            "healthy", "reject_all", "omit_negative_floor", "range_as_invariant", "healthy"])
        self.assertEqual([job["restored_healthy"] for job in jobs], [False] * 4 + [True])
        self.assertTrue(all(job["subject"] == "endpoint_finish" for job in jobs))
        self.assertTrue(all(job["intended_failure"] for job in jobs[1:4]))
        names = VERIFY.input_names("endpoint-finish")
        self.assertTrue({"tools/formal/additional_suites.py",
                         "tools/formal/endpoint_finish_harness.rs",
                         "tools/formal/endpoint_finish_projection.py",
                         "tools/formal/test_endpoint_finish.py",
                         "crates/time-core/Cargo.toml",
                         "crates/time-core/src/projection_enclosure.rs",
                         "crates/time-core/src/selection.rs"}.issubset(names))
        self.assertNotIn("tools/formal/era_constructor_harness.rs", names)

    def test_leap_coverage_selection_freezes_the_leaf_and_restores_after_three_defects(self):
        selected = VERIFY.selected_jobs("leap-coverage", 1)
        self.assertEqual([i for i, _ in selected], [1, 2, 3, 4, 5])
        jobs = [job for _, job in selected]
        self.assertEqual([job["case"] for job in jobs], [
            "healthy", "include_exclusive_end", "exclude_continuous_start",
            "bridge_one_second_gap", "healthy"])
        self.assertEqual([job["restored_healthy"] for job in jobs], [False] * 4 + [True])
        self.assertTrue(all(job["subject"] == "leap_coverage" for job in jobs))
        self.assertTrue(all(job["intended_failure"] for job in jobs[1:4]))
        names = VERIFY.input_names("leap-coverage")
        self.assertTrue({"tools/formal/additional_suites.py",
                         "tools/formal/leap_coverage_harness.rs",
                         "tools/formal/leap_coverage_projection.py",
                         "tools/formal/test_leap_coverage.py",
                         "crates/time-domain/src/leap/interpretation.rs"}.issubset(names))
        self.assertNotIn("tools/formal/era_harness.rs", names)

    def test_driver_inventories_selected_proof_closure(self):
        for selector in ("coordinate-exchange", "clock", "leap-calendar", "relative-range", "exchange-composition", "calendar-composition", "leap-ticks", "calendar-finite-laws", "leap-coverage", "endpoint-finish", "fraction-constructor", "fraction-comparison", "fraction-arithmetic", "scaled-value-split", "scaled-value-representation"):
            with self.subTest(selector=selector):
                self.assertTrue(set(SUITES.input_names(selector)).issubset(VERIFY.input_names(selector)))
                self.assertNotIn("tools/formal/era_harness.rs", VERIFY.input_names(selector))

    def test_driver_runs_complete_fixed_schedules(self):
        for selector, count in (("coordinate-exchange", 10), ("clock", 10), ("leap-calendar", 7), ("relative-range", 5), ("exchange-composition", 10), ("calendar-composition", 11), ("leap-ticks", 5), ("calendar-finite-laws", 5), ("leap-coverage", 5), ("endpoint-finish", 5), ("fraction-constructor", 5), ("fraction-comparison", 5), ("fraction-arithmetic", 10), ("scaled-value-split", 5), ("scaled-value-representation", 10)):
            with self.subTest(selector=selector):
                selected = VERIFY.selected_jobs(selector, 1)
                self.assertEqual(len(selected), count)
                self.assertEqual([job for _, job in selected], SUITES.jobs(selector))

    def test_new_healthy_results_need_their_own_complete_mathematical_contract(self):
        for selector in ("coordinate-exchange", "clock", "leap-calendar", "relative-range", "exchange-composition", "calendar-composition", "leap-ticks", "calendar-finite-laws", "leap-coverage", "endpoint-finish", "fraction-constructor", "fraction-comparison", "fraction-arithmetic", "scaled-value-split", "scaled-value-representation"):
            for job in SUITES.jobs(selector):
                if job["case"] != "healthy" or job["restored_healthy"]:
                    continue
                with self.subTest(harness=job["harness"]):
                    assertions, covers = SUITES.expected_contract(job["subject"], job["harness"])
                    checks = [{"category": "assertion", "description": name, "status": "Success",
                               "location": {"file": FIXTURE.ENTRY}} for name in sorted(assertions)]
                    checks.extend({"category": "cover", "description": name, "status": "Satisfied"}
                                  for name in sorted(covers))
                    report = FIXTURE.result(job, checks, True)
                    native = {"exit_code": 0, "failure": None}
                    VERIFY.qualify(report, native, job, FIXTURE.ENTRY)
                    # Covers and internally consistent accounting cannot replace
                    # even one missing mathematical assertion.
                    report["verification_results"]["results"][0]["checks"].pop(0)
                    FIXTURE.recount(report)
                    with self.assertRaises(VERIFY.Incomplete):
                        VERIFY.qualify(report, native, job, FIXTURE.ENTRY)


if __name__ == "__main__":
    unittest.main()
