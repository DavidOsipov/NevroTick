"""Additional suites must reach the real driver's inventory and result checks."""
from pathlib import Path
import importlib.util
import unittest


def load(name, filename):
    spec = importlib.util.spec_from_file_location(name, Path(__file__).with_name(filename))
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


VERIFY = load("formal_extended_driver_test", "verify.py")
SUITES = load("formal_extended_registry_test", "additional_suites.py")
FIXTURE = load("formal_extended_json_fixture", "test_verify.py")


class RunnerSuiteTests(unittest.TestCase):
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
        for selector in ("coordinate-exchange", "clock", "leap-calendar", "relative-range", "exchange-composition", "calendar-composition", "leap-ticks", "calendar-finite-laws", "leap-coverage", "endpoint-finish", "fraction-constructor", "fraction-comparison", "fraction-arithmetic"):
            with self.subTest(selector=selector):
                self.assertTrue(set(SUITES.input_names(selector)).issubset(VERIFY.input_names(selector)))
                self.assertNotIn("tools/formal/era_harness.rs", VERIFY.input_names(selector))

    def test_driver_runs_complete_fixed_schedules(self):
        for selector, count in (("coordinate-exchange", 10), ("clock", 10), ("leap-calendar", 7), ("relative-range", 5), ("exchange-composition", 10), ("calendar-composition", 11), ("leap-ticks", 5), ("calendar-finite-laws", 5), ("leap-coverage", 5), ("endpoint-finish", 5), ("fraction-constructor", 5), ("fraction-comparison", 5), ("fraction-arithmetic", 10)):
            with self.subTest(selector=selector):
                selected = VERIFY.selected_jobs(selector, 1)
                self.assertEqual(len(selected), count)
                self.assertEqual([job for _, job in selected], SUITES.jobs(selector))

    def test_new_healthy_results_need_their_own_complete_mathematical_contract(self):
        for selector in ("coordinate-exchange", "clock", "leap-calendar", "relative-range", "exchange-composition", "calendar-composition", "leap-ticks", "calendar-finite-laws", "leap-coverage", "endpoint-finish", "fraction-constructor", "fraction-comparison", "fraction-arithmetic"):
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
