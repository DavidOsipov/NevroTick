"""Partial proof selection preserves obligations; no solver execution."""
from pathlib import Path
import importlib.util
import unittest


SPEC = importlib.util.spec_from_file_location("formal_schedule_test", Path(__file__).with_name("verify.py"))
VERIFY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFY)


class ScheduleTests(unittest.TestCase):
    def test_default_selection_runs_every_fixed_job(self):
        for subject, count in (("all", 12), ("rounding", 7), ("era", 5)):
            with self.subTest(subject=subject):
                selected = VERIFY.selected_jobs(subject, 1)
                self.assertEqual([i for i, _ in selected], list(range(1, count + 1)))
                self.assertEqual([job for _, job in selected], VERIFY.jobs(subject))

    def test_suffix_keeps_original_indices_and_restoration(self):
        selected = VERIFY.selected_jobs("all", 7)
        self.assertEqual([i for i, _ in selected], [7, 8, 9, 10, 11, 12])
        self.assertEqual([(job["subject"], job["case"]) for _, job in selected], [
            ("rounding", "healthy"), ("era", "healthy"), ("era", "always_error"),
            ("era", "wrong_coordinate"), ("era", "wrong_error"), ("era", "healthy")])
        self.assertTrue(selected[0][1]["restored_healthy"])
        self.assertTrue(selected[-1][1]["restored_healthy"])

    def test_empty_negative_bool_and_excessive_selection_fail_closed(self):
        for value in (0, -1, 13, True, False, 1.0, "7"):
            with self.subTest(value=value):
                with self.assertRaises(VERIFY.Incomplete):
                    VERIFY.selected_jobs("all", value)


if __name__ == "__main__":
    unittest.main()
