"""Calendar projection/independent-oracle preparation controls; run no solver."""
from __future__ import annotations

from datetime import date, timedelta
import importlib.util
from pathlib import Path
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
SPEC = importlib.util.spec_from_file_location("calendar_projection_test", HERE / "leap_calendar_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


def year_start(years: int) -> int:
    """January-forward candidate oracle; compared below to civil-date facts."""
    return 365 * years + years // 4 - years // 100 + years // 400


def month_start_oracle(seconds: int) -> tuple[bool, int]:
    """Preparation model only; this Python function proves no Rust execution."""
    if seconds % 86400:
        return False, 0
    day = (seconds // 86400 + 693595) % 146097
    low, high = 0, 400
    for _ in range(9):
        middle = low + (high - low) // 2
        if year_start(middle) <= day:
            low = middle
        else:
            high = middle
    if not (high == low + 1 and year_start(low) <= day < year_start(high)):
        raise AssertionError("Year search did not bracket its day")
    civil_year = low + 1
    leap = civil_year % 4 == 0 and (civil_year % 100 != 0 or civil_year % 400 == 0)
    prefixes = (0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334)
    starts = (value + int(leap and month >= 2) for month, value in enumerate(prefixes))
    return day - year_start(low) in starts, civil_year


class ProjectionTests(unittest.TestCase):
    """Challenge frozen actual-source preparation without trusting file names."""

    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-calendar-preparation-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        source = self.source / PROJECTION.SOURCE
        source.parent.mkdir(parents=True)
        self.original = (ROOT / PROJECTION.SOURCE).read_bytes()
        source.write_bytes(self.original)
        self.harness = HERE / "leap_calendar_harness.rs"

    def prepare(self, case="healthy"):
        return PROJECTION.prepare_leap_calendar(
            self.source, self.base / "inputs", case, self.harness)

    def test_healthy_preserves_actual_private_body_and_scope(self):
        result = self.prepare()
        generated = (self.base / "inputs/calendar_selected.rs").read_bytes()
        actual = b"".join(self.original.splitlines(keepends=True)[82:98])
        self.assertTrue(generated.startswith(actual))
        self.assertEqual(result["correspondence"][0]["sha256"], PROJECTION.digest(actual))
        self.assertEqual(result["assumptions"], [])
        self.assertIsNone(result["mutation"])
        self.assertIn("valid_ledger", result["omitted"])
        self.assertEqual(result["entrypoint"], self.base / "inputs/proof.rs")
        self.assertNotIn("garde::", generated.decode())

    def test_each_defect_changes_exactly_its_single_reviewed_expression(self):
        actual = b"".join(self.original.splitlines(keepends=True)[82:98]).decode()
        for case, (before, after) in PROJECTION.MUTATIONS.items():
            with self.subTest(case=case):
                inputs = self.base / case
                result = PROJECTION.prepare_leap_calendar(self.source, inputs, case, self.harness)
                generated = (inputs / "calendar_selected.rs").read_text()
                self.assertEqual(actual.count(before), 1)
                self.assertTrue(generated.startswith(actual.replace(before, after, 1)))
                self.assertEqual(result["mutation"], {"before": before, "after": after})

    def test_full_file_drift_outside_selected_helper_is_rejected(self):
        (self.source / PROJECTION.SOURCE).write_bytes(b"// changed owner\n" + self.original)
        with self.assertRaisesRegex(ValueError, "source differs"):
            self.prepare()
        self.assertFalse((self.base / "inputs").exists())

    def test_actual_body_drift_is_rejected(self):
        (self.source / PROJECTION.SOURCE).write_bytes(self.original.replace(b"693_901", b"693_900"))
        with self.assertRaisesRegex(ValueError, "source differs"):
            self.prepare()

    def test_unknown_case_creates_no_inputs(self):
        with self.assertRaisesRegex(ValueError, "Unknown"):
            self.prepare("skip_centuries")
        self.assertFalse((self.base / "inputs").exists())

    def test_existing_proof_input_is_preserved(self):
        inputs = self.base / "inputs"
        inputs.mkdir()
        proof = inputs / "proof.rs"
        proof.write_bytes(b"existing evidence")
        with self.assertRaisesRegex(ValueError, "overwrite"):
            self.prepare()
        self.assertEqual(proof.read_bytes(), b"existing evidence")

    def test_schedule_keeps_all_three_defects_and_restores_both_theorems(self):
        jobs = PROJECTION.schedule()
        self.assertEqual(len(jobs), 7)
        self.assertEqual({job["case"] for job in jobs if job["intended_failure"]},
                         set(PROJECTION.MUTATIONS))
        self.assertEqual([job["harness"] for job in jobs if job["restored_healthy"]],
                         ["leap_calendar_full_domain", "leap_calendar_oracle_laws"])
        self.assertEqual(len({job["intended_failure"] for job in jobs if job["intended_failure"]}), 3)
        self.assertEqual(sum(len(items) for items in PROJECTION.REQUIRED_COVERS.values()), 10)


class OraclePreparationTests(unittest.TestCase):
    """Independent civil-date checks of the proposed oracle, not a product proof."""

    def test_complete_gregorian_cycle_and_nine_step_search(self):
        epoch = date(1900, 1, 1)
        count = 0
        for days in range(146097):
            civil = epoch + timedelta(days=days)
            observed, cycle_year = month_start_oracle(days * 86400)
            self.assertEqual(observed, civil.day == 1, civil.isoformat())
            self.assertEqual(cycle_year, (civil.year - 1) % 400 + 1)
            count += int(observed)
        self.assertEqual(count, 4800)

    def test_literal_epoch_century_and_raw_word_neighbors(self):
        for seconds, expected in ((0, True), (1, False), (2_274_739_200, True),
                                  (2_277_158_400, False), (6_316_531_200, True),
                                  (12_622_780_802_272_060_800, True),
                                  (2**64 - 1, False)):
            with self.subTest(seconds=seconds):
                self.assertEqual(month_start_oracle(seconds)[0], expected)
        self.assertEqual(year_start(1899), 693595)
        self.assertEqual(year_start(400), 146097)


if __name__ == "__main__":
    unittest.main()
