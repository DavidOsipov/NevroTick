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

    def test_explicit_coordinate_jobs_keep_fixed_indices_and_restore_healthy(self):
        selected = VERIFY.selected_jobs("coordinate-exchange", 1, [9, 3, 1, 5, 4])
        self.assertEqual([index for index, _ in selected], [1, 3, 4, 5, 9])
        self.assertEqual([job["case"] for _, job in selected],
                         ["healthy", "always_error", "wrong_coordinate", "wrong_error", "healthy"])
        self.assertTrue(all(job["subject"] == "coordinate" for _, job in selected))
        self.assertTrue(selected[-1][1]["restored_healthy"])

    def test_explicit_selection_rejects_empty_duplicate_invalid_and_bool_indices(self):
        for indices in ([], [1, 1], [0], [-1], [13], [True], [False], [1.0], ["1"], "1", 1):
            with self.subTest(indices=indices), self.assertRaises(VERIFY.Incomplete):
                VERIFY.selected_jobs("all", 1, indices)

    def test_explicit_selection_cannot_be_combined_with_a_suffix(self):
        with self.assertRaises(VERIFY.Incomplete):
            VERIFY.selected_jobs("all", 2, [3, 4])

    def test_explicit_full_selection_retains_every_obligation(self):
        selected = VERIFY.selected_jobs("all", 1, list(range(12, 0, -1)))
        self.assertEqual([index for index, _ in selected], list(range(1, 13)))
        self.assertEqual([job for _, job in selected], VERIFY.jobs("all"))


if __name__ == "__main__":
    unittest.main()
