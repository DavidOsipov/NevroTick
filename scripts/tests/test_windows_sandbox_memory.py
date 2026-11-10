"""Pure controls for English-profile HOST systeminfo memory observations."""
from __future__ import annotations

import importlib.util
from pathlib import Path
import unittest


_PATH = Path(__file__).resolve().parents[2] / "tools/windows-sandbox/host_memory.py"
_SPEC = importlib.util.spec_from_file_location("nevrotick_sandbox_host_memory", _PATH)
assert _SPEC is not None and _SPEC.loader is not None
MEMORY = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(MEMORY)


class FormatterRegressionTests(unittest.TestCase):
    """The missing first raw value is not reconstructed by these examples."""

    def test_actual_928_mb_neighbor_remains_an_observation_below_the_callers_floor(self) -> None:
        text = "Total Physical Memory:         28\u00a0357 MB\r\nAvailable Physical Memory:     928 MB\r\n"
        self.assertEqual(MEMORY.parse_available_physical_mb(text), 928)

    def test_observed_nbsp_formatter_counterexample_is_3400_mb(self) -> None:
        # Generated from the later observed separator, not the discarded first raw value.
        text = "Available Physical Memory:     3\u00a0400 MB\r\n"
        self.assertEqual(MEMORY.parse_available_physical_mb(text), 3400)


class StrictObservationTests(unittest.TestCase):
    def test_known_grouping_styles_have_the_same_integer_meaning(self) -> None:
        for separator in (",", " ", "\u00a0", "\u202f"):
            for number, expected in [(f"3{separator}400", 3400),
                                     (f"28{separator}357", 28357),
                                     (f"1{separator}234{separator}567", 1234567)]:
                with self.subTest(number=number):
                    self.assertEqual(MEMORY.parse_available_physical_mb(
                        f"Available Physical Memory:         {number} MB\r\n"), expected)

    def test_plain_values_preserve_zero_and_floor_neighbors_without_admitting_a_job(self) -> None:
        for value in (0, 1, 928, 2047, 2048, 2049, 3400):
            with self.subTest(value=value):
                self.assertEqual(MEMORY.parse_available_physical_mb(
                    f"Available Physical Memory:\t{value} MB\n"), value)

    def test_uint64_range_has_exact_accepted_and_refused_neighbors(self) -> None:
        self.assertEqual(MEMORY.parse_available_physical_mb(
            "Available Physical Memory: 18446744073709551615 MB"), 18446744073709551615)
        for number in ("18446744073709551616", "184467440737095516150", "9" * 100):
            with self.subTest(number=number), self.assertRaises(ValueError):
                MEMORY.parse_available_physical_mb(f"Available Physical Memory: {number} MB")

    def test_malformed_or_mixed_grouping_never_becomes_a_larger_count(self) -> None:
        numbers = ("3,40", "3,4000", "34,00", ",3400", "3400,", "1,,024", "1,234,56",
                   "12,3456,789", "1  024", "1,234 567", "1\u00a0234\u202f567", "1,234\u00a0567",
                   "00", "03400", "0,928", "000,928", "1\t024", "1_024", "1.024")
        for number in numbers:
            with self.subTest(number=number), self.assertRaises(ValueError):
                MEMORY.parse_available_physical_mb(f"Available Physical Memory: {number} MB")

    def test_signed_fractional_nonfinite_and_non_ascii_digits_refuse(self) -> None:
        for number in ("-928", "+928", "2048.0", "2048/1", "2e3", "NaN", "Infinity",
                       "\u0662\u0660\u0664\u0668", "\uff12\uff10\uff14\uff18", "2\u066048"):
            with self.subTest(number=number), self.assertRaises(ValueError):
                MEMORY.parse_available_physical_mb(f"Available Physical Memory: {number} MB")

    def test_wrong_unit_payload_and_noncanonical_field_spelling_refuse(self) -> None:
        lines = ("Available Physical Memory: 2048 MB trailing", "Available Physical Memory: 2048 MB\t",
                 "Available Physical Memory: 2048 MiB", "Available Physical Memory: 2048 mb",
                 "Available Physical Memory: 2048 GB", "Available Physical Memory: 2048MB",
                 "Available Physical Memory:2048 MB", "Available Physical Memory : 2048 MB",
                 " Available Physical Memory: 2048 MB", "available physical memory: 2048 MB")
        for line in lines:
            with self.subTest(line=line), self.assertRaises(ValueError):
                MEMORY.parse_available_physical_mb(line)

    def test_exact_or_malformed_duplicate_field_is_ambiguous(self) -> None:
        good = "Available Physical Memory: 2048 MB\n"
        for second in (good, "Available Physical Memory: 928 MB\n",
                       "Available Physical Memory: invalid MB\n",
                       "\tAvailable Physical Memory : 9999 MB\n"):
            with self.subTest(second=second), self.assertRaises(ValueError):
                MEMORY.parse_available_physical_mb(good + second)

    def test_other_memory_fields_do_not_supply_this_observation(self) -> None:
        for output in ("", "Total Physical Memory: 28357 MB\nVirtual Memory: Available: 33843 MB\n",
                       "MemAvailable: 3481600 kB\n", "Mem: 28357 24957 3400\n",
                       "Доступная физическая память: 3400 МБ\n"):
            with self.subTest(output=output), self.assertRaises(ValueError):
                MEMORY.parse_available_physical_mb(output)

    def test_field_is_not_an_embedded_substring(self) -> None:
        with self.assertRaises(ValueError):
            MEMORY.parse_available_physical_mb("OS Name: Available Physical Memory: 9999 MB\n")

    def test_actual_crlf_and_normalized_lf_are_supported(self) -> None:
        for ending in ("\r\n", "\n", ""):
            with self.subTest(ending=ending):
                self.assertEqual(MEMORY.parse_available_physical_mb(
                    "Available Physical Memory: 2,048 MB" + ending), 2048)

    def test_bare_cr_and_alternate_or_hidden_line_controls_refuse(self) -> None:
        field = "Available Physical Memory: 2048 MB"
        for control in ("\r", "\x00", "\v", "\f", "\x7f", "\x85", "\u2028", "\u2029"):
            with self.subTest(control=repr(control)), self.assertRaises(ValueError):
                MEMORY.parse_available_physical_mb(field + control + "Available Physical Memory: 9999 MB")
        with self.assertRaises(ValueError):
            MEMORY.parse_available_physical_mb(field + "\r")

    def test_output_byte_budget_accepts_exact_boundary_and_refuses_next_byte(self) -> None:
        field = "Available Physical Memory: 2048 MB\n"
        remaining = 65536 - len(field)
        output = field + "x\n" * (remaining // 2) + "x" * (remaining % 2)
        self.assertEqual(len(output.encode("utf-8")), 65536)
        self.assertEqual(MEMORY.parse_available_physical_mb(output), 2048)
        with self.assertRaises(ValueError):
            MEMORY.parse_available_physical_mb(output + "x")

    def test_output_budget_counts_utf8_bytes_after_decoding(self) -> None:
        with self.assertRaises(ValueError):
            MEMORY.parse_available_physical_mb("Available Physical Memory: 2048 MB\n" + "é\n" * 22000)

    def test_line_byte_budget_has_exact_ascii_and_unicode_neighbors(self) -> None:
        field = "\nAvailable Physical Memory: 2048 MB\n"
        for line in ("x" * 1024, "é" * 512):
            with self.subTest(kind=line[:1]):
                self.assertEqual(MEMORY.parse_available_physical_mb(line + field), 2048)
                with self.assertRaises(ValueError):
                    MEMORY.parse_available_physical_mb(line + "x" + field)

    def test_invalid_decoded_unicode_and_wrong_runtime_types_refuse(self) -> None:
        with self.assertRaises(ValueError):
            MEMORY.parse_available_physical_mb("Available Physical Memory: 2048 MB\n\ud800")
        for value in (b"Available Physical Memory: 2048 MB", None, 2048, True, []):
            with self.subTest(value=repr(value)), self.assertRaises(TypeError):
                MEMORY.parse_available_physical_mb(value)

    def test_rejection_does_not_echo_supplied_systeminfo_payload(self) -> None:
        with self.assertRaises(ValueError) as refused:
            MEMORY.parse_available_physical_mb("Available Physical Memory: SECRET-HOST-TEXT MB")
        self.assertNotIn("SECRET-HOST-TEXT", str(refused.exception))


if __name__ == "__main__":
    unittest.main()
