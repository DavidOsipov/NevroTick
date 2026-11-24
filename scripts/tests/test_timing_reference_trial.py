# SPDX-License-Identifier: GPL-2.0-only
"""Independent endpoint and bounded exhaustive controls for the W11-W01 trial."""

from dataclasses import replace
import importlib.util
from itertools import combinations_with_replacement, product
from pathlib import Path
import unittest


_PATH: Path = Path(__file__).resolve().parents[2] / "tools/windows-sandbox/timing_reference_trial.py"
_SPEC = importlib.util.spec_from_file_location("nevrotick_timing_reference_trial", _PATH)
assert _SPEC is not None and _SPEC.loader is not None
TRIAL = importlib.util.module_from_spec(_SPEC)
_SPEC.loader.exec_module(TRIAL)


class ReferenceTrialTests(unittest.TestCase):
    def setUp(self) -> None:
        event: TRIAL.TrialEvent = TRIAL.TrialEvent("e" * 64, TRIAL.ApplicationEvent.PRE_SEND)
        self.model: TRIAL.ModeledLeg = TRIAL.ModeledLeg(event, "a" * 64, TRIAL.Nanoseconds(9810, 10000))
        self.reference: TRIAL.HeldOutLeg = TRIAL.HeldOutLeg(
            event, "b" * 64, "c" * 64, TRIAL.Nanoseconds(10000, 10240),
            TRIAL.Nanoseconds(-20, -20), TRIAL.Nanoseconds(-200, -180),
        )

    def test_modeled_wall_reference_and_budget_boundary(self) -> None:
        # Independent local-wall range is [9780, 10040]. The wider distance
        # is from 9810 to 10040, even though the two ranges overlap.
        result: int = TRIAL.compare_held_out_leg(
            self.model, self.reference, declared_error_ns=230, leg_budget_ns=230,
        )
        self.assertEqual(result, 230)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "worst-case disagreement"):
            TRIAL.compare_held_out_leg(self.model, self.reference, declared_error_ns=229, leg_budget_ns=230)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "exceeds leg budget"):
            TRIAL.compare_held_out_leg(self.model, self.reference, declared_error_ns=230, leg_budget_ns=229)

    def test_receive_uses_positive_placement_and_original_wall_residual(self) -> None:
        event: TRIAL.TrialEvent = replace(self.model.event, event=TRIAL.ApplicationEvent.RECEIVE_RETURN)
        model: TRIAL.ModeledLeg = replace(self.model, event=event, local_wall=TRIAL.Nanoseconds(19800, 19802))
        reference: TRIAL.HeldOutLeg = replace(
            self.reference, event=event, wire_utc=TRIAL.Nanoseconds(20000, 20020),
            application_minus_wire=TRIAL.Nanoseconds(5, 8),
        )
        # [20000 + 5 - 200, 20020 + 8 - 180] = [19805, 19848].
        self.assertEqual(TRIAL.compare_held_out_leg(model, reference, declared_error_ns=48, leg_budget_ns=48), 48)

    def test_crossed_direction_or_exchange_is_refused_before_comparison(self) -> None:
        for event in (
            replace(self.reference.event, exchange_sha256="f" * 64),
            replace(self.reference.event, event=TRIAL.ApplicationEvent.RECEIVE_RETURN),
        ):
            with self.subTest(event=event), self.assertRaisesRegex(TRIAL.TrialRefused, "reference event differs"):
                TRIAL.compare_held_out_leg(
                    self.model, replace(self.reference, event=event), declared_error_ns=230, leg_budget_ns=230,
                )

    def test_original_capture_cannot_supply_either_independent_record(self) -> None:
        for reference in (
            replace(self.reference, wire_record_sha256=self.model.application_capture_sha256),
            replace(self.reference, residual_record_sha256=self.model.application_capture_sha256),
        ):
            with self.subTest(reference=reference), self.assertRaisesRegex(TRIAL.TrialRefused, "application capture reused"):
                TRIAL.compare_held_out_leg(self.model, reference, declared_error_ns=230, leg_budget_ns=230)

    def test_distinct_acquisitions_can_agree_exactly(self) -> None:
        wall: TRIAL.Nanoseconds = TRIAL.Nanoseconds(1000, 1000)
        zero: TRIAL.Nanoseconds = TRIAL.Nanoseconds(0, 0)
        model: TRIAL.ModeledLeg = replace(self.model, local_wall=wall)
        reference: TRIAL.HeldOutLeg = replace(
            self.reference, wire_utc=wall, application_minus_wire=zero, local_wall_minus_utc=zero,
        )
        self.assertEqual(TRIAL.compare_held_out_leg(model, reference, declared_error_ns=0, leg_budget_ns=0), 0)

    def test_reversed_invalid_and_outgrown_intervals_refuse(self) -> None:
        for earliest, latest in ((2, 1), (False, 0), (0, 1 << 63), (-(1 << 63) - 1, 0)):
            with self.subTest(earliest=earliest, latest=latest), self.assertRaisesRegex(
                TRIAL.TrialRefused, "invalid nanosecond interval",
            ):
                TRIAL.Nanoseconds(earliest, latest)
        endpoint: int = (1 << 63) - 1
        reference: TRIAL.HeldOutLeg = replace(
            self.reference, wire_utc=TRIAL.Nanoseconds(endpoint, endpoint),
            application_minus_wire=TRIAL.Nanoseconds(1000, 1000),
        )
        with self.assertRaisesRegex(TRIAL.TrialRefused, "invalid nanosecond interval"):
            TRIAL.compare_held_out_leg(self.model, reference, declared_error_ns=230, leg_budget_ns=230)

    def test_invalid_identities_and_limits_refuse(self) -> None:
        with self.assertRaisesRegex(TRIAL.TrialRefused, "invalid exchange identity"):
            TRIAL.TrialEvent("not-a-pin", TRIAL.ApplicationEvent.PRE_SEND)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "invalid application event"):
            TRIAL.TrialEvent("a" * 64, "pre-send")
        with self.assertRaisesRegex(TRIAL.TrialRefused, "invalid observation identity"):
            TRIAL.compare_held_out_leg(
                self.model, replace(self.reference, residual_record_sha256="Z" * 64),
                declared_error_ns=230, leg_budget_ns=230,
            )
        for invalid in (True, -1, 1 << 63):
            with self.subTest(limit=invalid), self.assertRaisesRegex(TRIAL.TrialRefused, "invalid error limit"):
                TRIAL.compare_held_out_leg(self.model, self.reference, declared_error_ns=invalid, leg_budget_ns=230)

    def test_complete_small_grid_matches_all_individual_disagreements(self) -> None:
        intervals: list[tuple[int, int]] = list(combinations_with_replacement((-1, 0, 1), 2))
        for case in product(intervals, repeat=4):
            candidate, wire, placement, residual = case
            # Exhaust the integer points before taking differences. This uses
            # neither the comparison's endpoint formula nor a range-sum helper.
            points = [range(first, last + 1) for first, last in case]
            expected: int = max(abs(m - u - p - r) for m, u, p, r in product(*points))
            model: TRIAL.ModeledLeg = replace(self.model, local_wall=TRIAL.Nanoseconds(*candidate))
            reference: TRIAL.HeldOutLeg = replace(
                self.reference, wire_utc=TRIAL.Nanoseconds(*wire),
                application_minus_wire=TRIAL.Nanoseconds(*placement),
                local_wall_minus_utc=TRIAL.Nanoseconds(*residual),
            )
            self.assertEqual(
                TRIAL.compare_held_out_leg(model, reference, declared_error_ns=expected, leg_budget_ns=expected),
                expected, case,
            )


if __name__ == "__main__":
    unittest.main()
