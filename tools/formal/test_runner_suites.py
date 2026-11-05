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
        for selector in ("coordinate-exchange", "clock", "leap-calendar", "relative-range", "exchange-composition", "calendar-composition", "leap-ticks", "calendar-finite-laws", "leap-coverage"):
            with self.subTest(selector=selector):
                self.assertTrue(set(SUITES.input_names(selector)).issubset(VERIFY.input_names(selector)))
                self.assertNotIn("tools/formal/era_harness.rs", VERIFY.input_names(selector))

    def test_driver_runs_complete_fixed_schedules(self):
        for selector, count in (("coordinate-exchange", 10), ("clock", 10), ("leap-calendar", 7), ("relative-range", 5), ("exchange-composition", 10), ("calendar-composition", 11), ("leap-ticks", 5), ("calendar-finite-laws", 5), ("leap-coverage", 5)):
            with self.subTest(selector=selector):
                selected = VERIFY.selected_jobs(selector, 1)
                self.assertEqual(len(selected), count)
                self.assertEqual([job for _, job in selected], SUITES.jobs(selector))

    def test_new_healthy_results_need_their_own_complete_mathematical_contract(self):
        for selector in ("coordinate-exchange", "clock", "leap-calendar", "relative-range", "exchange-composition", "calendar-composition", "leap-ticks", "calendar-finite-laws", "leap-coverage"):
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
