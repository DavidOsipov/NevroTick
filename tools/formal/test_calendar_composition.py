"""Calendar factor/oracle preparation controls; execute no Rust or solver."""
from __future__ import annotations

from datetime import date, timedelta
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent
SPEC = importlib.util.spec_from_file_location("calendar_composition_test", HERE / "calendar_composition_projection.py")
PROJECTION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PROJECTION)


def start(year):
    return 365 * year + year // 4 - year // 100 + year // 400


def leap(year):
    return year % 4 == 0 and (year % 100 != 0 or year % 400 == 0)


def represent(day):
    low, high = 0, 400
    for _ in range(9):
        middle = low + (high - low) // 2
        if start(middle) <= day:
            low = middle
        else:
            high = middle
    if not (high == low + 1 and start(low) <= day < start(high)):
        raise AssertionError("Forward representation failed")
    return low, day - start(low)


def prefix(month, is_leap):
    lengths = (31, 30, 31, 30, 31, 31, 30, 31, 30, 31, 31, 29 if is_leap else 28)
    return sum(lengths[:month])


class CalendarPreparationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-calendar-composition-")
        self.addCleanup(self.temporary.cleanup)
        self.folder = Path(self.temporary.name)
        self.source = self.folder / "source"
        self.target = self.source / PROJECTION.SOURCE
        self.target.parent.mkdir(parents=True)
        self.original = (ROOT / PROJECTION.SOURCE).read_bytes()
        self.target.write_bytes(self.original)
        for name in PROJECTION.CONTEXT_SHA256:
            destination = self.source / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes((ROOT / name).read_bytes())
        self.harness = HERE / "calendar_composition_harness.rs"
        self.inputs = self.folder / "inputs"

    def prepare(self, case="healthy", inputs=None):
        return PROJECTION.prepare_calendar_composition(
            self.source, self.inputs if inputs is None else inputs, case, self.harness)

    def test_exact_factors_reconstruct_whole_source_and_preserve_original_early_return(self):
        details = self.prepare()
        actual = b"".join(self.original.splitlines(keepends=True)[82:98])
        generated = (self.inputs / "calendar_selected.rs").read_bytes()
        self.assertTrue(generated.startswith(actual))
        header = b"fn is_month_start(seconds: u64) -> bool {\n"
        prefix_record, suffix_record = details["correspondence"][1:]
        first, last = prefix_record["function_byte_offsets_half_open"]
        prefix_bytes = actual[first:last]
        first, last = suffix_record["function_byte_offsets_half_open"]
        suffix_bytes = actual[first:last]
        self.assertEqual(header + prefix_bytes + suffix_bytes + b"}\n", actual)
        self.assertIn(b"return false;", prefix_bytes)
        self.assertNotIn(b"seconds", suffix_bytes)
        self.assertEqual(hashlib.sha256(prefix_bytes).hexdigest(), prefix_record["sha256"])
        self.assertEqual(hashlib.sha256(suffix_bytes).hexdigest(), suffix_record["sha256"])
        self.assertIn(prefix_bytes + b"    *output = Some(day_in_cycle);", generated)
        self.assertIn(b"fn proof_suffix(day_in_cycle: i128) -> bool {\n" + suffix_bytes, generated)
        self.assertEqual(details["assumptions"], [])
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**details, "entrypoint": "proof.rs"})

    def test_all_eleven_jobs_share_exact_original_factor_and_oracle_identities(self):
        baseline = None
        for index, job in enumerate(PROJECTION.schedule(), 1):
            with self.subTest(index=index, case=job["case"]):
                inputs = self.folder / f"job-{index}"
                details = self.prepare(job["case"], inputs)
                identity = (details["composite_contract"], details["correspondence"], details["harness_sha256"])
                if baseline is None:
                    baseline = identity
                self.assertEqual(identity, baseline)
                self.assertEqual(details["mutation"] is None, job["case"] == "healthy")
                self.assertEqual(self.target.read_bytes(), self.original)

    def test_each_fault_changes_only_one_original_helper_expression(self):
        original_function = b"".join(self.original.splitlines(keepends=True)[82:98])
        for case, (before, after, _, _) in PROJECTION.CONTROLS.items():
            inputs = self.folder / case
            details = self.prepare(case, inputs)
            generated = (inputs / "calendar_selected.rs").read_bytes()
            expected = original_function.replace(before.encode(), after.encode(), 1)
            self.assertEqual(original_function.count(before.encode()), 1)
            self.assertTrue(generated.startswith(expected))
            compiled_prefix, compiled_suffix = PROJECTION.reviewed_factors(expected, case)
            self.assertEqual(generated[len(expected):], PROJECTION.factor_wrappers(compiled_prefix, compiled_suffix))
            binding = details["structural_binding"]
            self.assertEqual(binding["status"], "checked-reviewed-byte-family")
            self.assertFalse(binding["machine_H_equals_factor_theorem"])
            self.assertTrue(binding["compiled_fragments_reconstruct_function"])
            self.assertEqual(binding["compiled_prefix_sha256"], hashlib.sha256(compiled_prefix).hexdigest())
            self.assertEqual(binding["compiled_suffix_sha256"], hashlib.sha256(compiled_suffix).hexdigest())
            self.assertEqual(details["compiled_function_sha256"], hashlib.sha256(expected).hexdigest())

    def test_source_drift_unknown_case_and_existing_inputs_fail_closed(self):
        self.target.write_bytes(self.original + b"\n// changed outside selected helper\n")
        with self.assertRaisesRegex(ValueError, "source changed"):
            self.prepare()
        self.assertFalse(self.inputs.exists())
        self.target.write_bytes(self.original)
        with self.assertRaisesRegex(ValueError, "Unknown calendar-composition"):
            self.prepare("unreviewed")
        self.assertFalse(self.inputs.exists())
        self.inputs.mkdir()
        saved = self.inputs / "proof.rs"
        saved.write_bytes(b"retained evidence")
        with self.assertRaisesRegex(ValueError, "overwrite"):
            self.prepare()
        self.assertEqual(saved.read_bytes(), b"retained evidence")

    def test_schedule_preserves_all_four_universal_leaves_faults_and_restorations(self):
        jobs = PROJECTION.schedule()
        names = ["calendar_source_full_domain", "calendar_representation_full_domain",
                 "calendar_suffix_full_domain", "calendar_gregorian_laws"]
        self.assertEqual(len(jobs), 11)
        self.assertEqual([j["harness"] for j in jobs[:4]], names)
        self.assertEqual([j["harness"] for j in jobs[-4:]], names)
        self.assertTrue(all(not j["restored_healthy"] for j in jobs[:4]))
        self.assertTrue(all(j["restored_healthy"] for j in jobs[-4:]))
        self.assertEqual([j["case"] for j in jobs[4:7]],
                         ["reject_month_start", "omit_alignment", "omit_century_adjustment"])
        text = self.harness.read_text()
        self.assertEqual(set(re.findall(r"#\[kani::proof\]\s+fn (\w+)\(", text)), {j["harness"] for j in jobs})
        self.assertNotIn("kani::assume", text)
        self.assertIn("let seconds = kani::any::<u64>();", text)
        self.assertIn("let day = kani::any::<u32>();", text)
        self.assertIn("let cycles = kani::any::<u64>();", text)
        self.assertIn("let mut represented = None;", text)
        for name, assertions in PROJECTION.REQUIRED_ASSERTIONS.items():
            self.assertTrue(assertions)
            self.assertEqual(len(PROJECTION.REQUIRED_COVERS[name]), 4)
            for value in assertions + PROJECTION.REQUIRED_COVERS[name]:
                self.assertIn(f'"{value}"', text)

    def test_closed_shape_rejects_unreviewed_returns_effects_bindings_and_control_mismatch(self):
        function = b"".join(self.original.splitlines(keepends=True)[82:98])
        for altered in (function + b"\n", function.replace(b"let march_days", b"let mut march_days", 1),
                        function.replace(b"    let year_in_cycle", b"    external_effect();\n    let year_in_cycle", 1),
                        function.replace(b"matches!", b"unreviewed_matches!", 1),
                        function.replace(b"return false;", b"return true;", 1)):
            with self.subTest(altered=altered[:80]), self.assertRaises(ValueError):
                PROJECTION.reviewed_factors(altered, "healthy")
        for case, (before, after, _, _) in PROJECTION.CONTROLS.items():
            mutant = function.replace(before.encode(), after.encode(), 1)
            with self.assertRaises(ValueError):
                PROJECTION.reviewed_factors(mutant, "healthy")
            PROJECTION.reviewed_factors(mutant, case)

    def test_context_drift_fails_before_inputs_and_prefix_node_has_no_symbolic_suffix(self):
        name = next(iter(PROJECTION.CONTEXT_SHA256))
        path = self.source / name
        path.write_bytes(path.read_bytes() + b"\n// changed context\n")
        with self.assertRaisesRegex(ValueError, "source context changed"):
            self.prepare()
        self.assertFalse(self.inputs.exists())
        text = self.harness.read_text()
        begin = text.index("fn calendar_source_full_domain()")
        end = text.index("#[kani::proof]", begin)
        node = text[begin:end]
        self.assertNotIn("calendar::proof_original(seconds)", node)
        self.assertNotIn("calendar::proof_suffix(", node)
        self.assertNotIn("source exact recomposition", text)
        self.assertEqual(len(PROJECTION.REQUIRED_ASSERTIONS["calendar_source_full_domain"]), 6)


class GregorianModelControls(unittest.TestCase):
    def test_complete_cycle_representation_and_month_membership_against_civil_dates(self):
        epoch = date(2000, 3, 1)
        for day in range(146_097):
            year, ordinal = represent(day)
            length = 365 + int(leap(year + 1))
            self.assertLess(year, 400)
            self.assertLess(ordinal, length)
            self.assertEqual(start(year) + ordinal, day)
            expected = (epoch + timedelta(days=day)).day == 1
            observed = ordinal in {prefix(month, leap(year + 1)) for month in range(12)}
            self.assertEqual(observed, expected, (day, year, ordinal))

    def test_epoch_century_and_final_cycle_boundaries(self):
        self.assertEqual(start(400), 146_097)
        self.assertEqual(start(1899) + prefix(10, False), 693_901)
        self.assertEqual(693_901 % 146_097, start(299) + prefix(10, False))
        self.assertEqual(represent(0), (0, 0))
        self.assertEqual(represent(146_096), (399, 365))
        self.assertEqual(represent(36_524), (100, 0))
        self.assertEqual(represent(146_068), (399, 337))
        self.assertFalse(leap(100))
        self.assertTrue(leap(400))
        maximum = 2 ** 64 - 1
        for year in (0, 3, 99, 399):
            self.assertEqual(start(400 * maximum + year), 146_097 * maximum + start(year))


if __name__ == "__main__":
    unittest.main()
