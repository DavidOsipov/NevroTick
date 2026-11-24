# SPDX-License-Identifier: GPL-2.0-only
"""Independent endpoint and bounded exhaustive controls for the W11-W01 trial."""

from dataclasses import replace
from hashlib import sha256
import importlib.util
from itertools import combinations_with_replacement, product
import json
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


class RetainedCaptureTests(unittest.TestCase):
    def setUp(self) -> None:
        self.rows: list[bytes] = [
            b"scope=managed-protected-successor-timing leg=first point=pre-send client_port=49000 "
            b"peer_port=45000 qpc_before=-12 filetime_100ns=116444736000000010 qpc_after=-10 frequency_hz=10000000",
            b"scope=managed-protected-successor-timing leg=first point=send-return client_port=49000 "
            b"peer_port=45000 qpc_before=-10 filetime_100ns=116444736000000011 qpc_after=-8 frequency_hz=10000000",
            b"scope=managed-protected-successor-timing leg=first point=receive-return client_port=49000 "
            b"peer_port=45000 qpc_before=10 filetime_100ns=116444736000000012 qpc_after=12 frequency_hz=10000000",
            b"scope=managed-protected-successor-timing leg=successor point=pre-send client_port=49001 "
            b"peer_port=45000 qpc_before=20 filetime_100ns=116444736000000013 qpc_after=22 frequency_hz=10000000",
            b"scope=managed-protected-successor-timing leg=successor point=send-return client_port=49001 "
            b"peer_port=45000 qpc_before=30 filetime_100ns=116444736000000014 qpc_after=32 frequency_hz=10000000",
            b"scope=managed-protected-successor-timing leg=successor point=receive-return client_port=49001 "
            b"peer_port=45000 qpc_before=40 filetime_100ns=116444736000000015 qpc_after=42 frequency_hz=10000000",
        ]
        self.stream: bytes = b"\r\n".join(self.rows) + b"\r\n"

    def receipt(self, stream: bytes, run_id: str = "ketiming_000000000001", *, frequency: int = 10_000_000) -> bytes:
        document: dict[str, object] = {
            "scope": "native-authenticated-ordinary-timing",
            "scenario": "ordinary-timing",
            "status": "passed-native-managed-successor-facade",
            "run_id": run_id,
            "empty_mappings": True,
            "retirement": "exact-one-peer-timing-restoration",
            "observed": {"relative_timing": {"frequency_hz": frequency}},
            "retained_output": {"healthy-ke-managed-successor-facade-client-stdout.txt": {
                "bytes": len(stream), "sha256": sha256(stream).hexdigest(),
            }},
        }
        return json.dumps(document).encode("ascii")

    def test_exact_triplets_preserve_coordinates_and_project_both_comparisons(self) -> None:
        receipt: bytes = self.receipt(self.stream)
        first: TRIAL.RetainedExchange
        successor: TRIAL.RetainedExchange
        first, successor = TRIAL.read_retained_captures(receipt, sha256(receipt).hexdigest(), self.stream)
        self.assertEqual((first.client_port, successor.client_port, first.peer_port), (49000, 49001, 45000))
        self.assertEqual((first.pre_send.qpc_before, first.send_return.qpc_before, first.receive_return.qpc_after),
                         (-12, -10, 12))
        self.assertEqual(first.pre_send.filetime_100ns, 116444736000000010)
        self.assertEqual(successor.receive_return.filetime_100ns, 116444736000000015)
        self.assertEqual(first.frequency_hz, 10_000_000)
        self.assertNotEqual(first.exchange_sha256, successor.exchange_sha256)
        direction: TRIAL.ApplicationEvent
        raw: bytes
        wall: int
        for direction, raw, wall in (
            (TRIAL.ApplicationEvent.PRE_SEND, self.rows[0], 1000),
            (TRIAL.ApplicationEvent.RECEIVE_RETURN, self.rows[2], 1700),
        ):
            with self.subTest(direction=direction):
                point: TRIAL.Nanoseconds = TRIAL.Nanoseconds(wall, wall)
                model: TRIAL.ModeledLeg = first.modeled_leg(direction, point)
                self.assertEqual(model.application_capture_sha256, sha256(raw + b"\r\n").hexdigest())
                reference: TRIAL.HeldOutLeg = TRIAL.HeldOutLeg(
                    TRIAL.TrialEvent(first.exchange_sha256, direction), "b" * 64, "c" * 64,
                    TRIAL.Nanoseconds(wall + 10, wall + 10), TRIAL.Nanoseconds(0, 0), TRIAL.Nanoseconds(-10, -10),
                )
                self.assertEqual(TRIAL.compare_held_out_leg(model, reference, declared_error_ns=0, leg_budget_ns=0), 0)

    def test_crossed_receipt_and_changed_transcript_refuse_at_separate_boundaries(self) -> None:
        receipt: bytes = self.receipt(self.stream)
        pin: str = sha256(receipt).hexdigest()
        other_receipt: bytes = self.receipt(self.stream, "ketiming_000000000002")
        with self.assertRaisesRegex(TRIAL.TrialRefused, "receipt digest differs"):
            TRIAL.read_retained_captures(other_receipt, pin, self.stream)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "stream differs from receipt"):
            TRIAL.read_retained_captures(receipt, pin, self.stream.replace(b"qpc_after=-10", b"qpc_after=-11"))
        first: TRIAL.RetainedExchange
        other_first: TRIAL.RetainedExchange
        first, _ = TRIAL.read_retained_captures(receipt, pin, self.stream)
        other_first, _ = TRIAL.read_retained_captures(other_receipt, sha256(other_receipt).hexdigest(), self.stream)
        self.assertEqual(first.pre_send, other_first.pre_send)
        direction: TRIAL.ApplicationEvent = TRIAL.ApplicationEvent.PRE_SEND
        zero: TRIAL.Nanoseconds = TRIAL.Nanoseconds(0, 0)
        reference: TRIAL.HeldOutLeg = TRIAL.HeldOutLeg(
            TRIAL.TrialEvent(other_first.exchange_sha256, direction), "b" * 64, "c" * 64, zero, zero, zero,
        )
        with self.assertRaisesRegex(TRIAL.TrialRefused, "reference event differs"):
            TRIAL.compare_held_out_leg(first.modeled_leg(direction, zero), reference, declared_error_ns=0, leg_budget_ns=0)

    def test_bound_but_invalid_rows_and_receipt_refuse(self) -> None:
        changed: bytes
        reason: str
        for changed, reason in (
            (self.stream[:-1], "exactly six terminated"),
            (self.stream.replace(b"qpc_before=-12", b"qpc_before=-0"), "invalid capture row"),
            (self.stream.replace(b"qpc_before=-12", b"qpc_before=-9223372036854775809"), "row bounds"),
            (self.stream.replace(b"filetime_100ns=116444736000000010", b"filetime_100ns=18446744073709551616"),
             "row bounds"),
            (self.stream.replace(b"qpc_before=-10", b"qpc_before=-11"), "bracket order"),
            (self.stream.replace(b"frequency_hz=10000000", b"frequency_hz=10000001", 1), "frequency differs"),
            (self.stream.replace(b"client_port=49001", b"client_port=49000"), "exchange sockets"),
        ):
            with self.subTest(reason=reason), self.assertRaisesRegex(TRIAL.TrialRefused, reason):
                receipt: bytes = self.receipt(changed)
                TRIAL.read_retained_captures(receipt, sha256(receipt).hexdigest(), changed)
        incomplete: bytes = self.receipt(self.stream).replace(b"passed-native-managed-successor-facade", b"incomplete")
        with self.assertRaisesRegex(TRIAL.TrialRefused, "receipt is not a completed timing run"):
            TRIAL.read_retained_captures(incomplete, sha256(incomplete).hexdigest(), self.stream)

    def test_rejects_ambiguous_receipt_fields_and_oversized_inputs(self) -> None:
        receipt: bytes = self.receipt(self.stream)
        duplicate: bytes = rb'{"\u0073cope": "crossed", ' + receipt[1:]
        with self.assertRaisesRegex(TRIAL.TrialRefused, "duplicate timing receipt field"):
            TRIAL.read_retained_captures(duplicate, sha256(duplicate).hexdigest(), self.stream)
        oversized_receipt: bytes = receipt + b" " * 65_536
        with self.assertRaisesRegex(TRIAL.TrialRefused, "receipt exceeds input bounds"):
            TRIAL.read_retained_captures(oversized_receipt, sha256(oversized_receipt).hexdigest(), self.stream)
        oversized_stream: bytes = self.stream + b" " * 4096
        oversized_receipt = self.receipt(oversized_stream)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "stream exceeds input bounds"):
            TRIAL.read_retained_captures(oversized_receipt, sha256(oversized_receipt).hexdigest(), oversized_stream)

    def test_native_frequency_and_filetime_keep_their_distinct_integer_ranges(self) -> None:
        limit: int = 9_223_372_036_854_775_807
        stream: bytes = self.stream.replace(b"frequency_hz=10000000", b"frequency_hz=9223372036854775807")
        stream = stream.replace(b"filetime_100ns=116444736000000010", b"filetime_100ns=18446744073709551615")
        receipt: bytes = self.receipt(stream, frequency=limit)
        first: TRIAL.RetainedExchange
        first, _ = TRIAL.read_retained_captures(receipt, sha256(receipt).hexdigest(), stream)
        self.assertEqual(first.frequency_hz, limit)
        self.assertEqual(first.pre_send.filetime_100ns, 18_446_744_073_709_551_615)
        stream = stream.replace(b"frequency_hz=9223372036854775807", b"frequency_hz=9223372036854775808")
        receipt = self.receipt(stream, frequency=limit + 1)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "invalid timing receipt frequency"):
            TRIAL.read_retained_captures(receipt, sha256(receipt).hexdigest(), stream)


class HeldOutExchangeTests(unittest.TestCase):
    def setUp(self) -> None:
        self.fixture: RetainedCaptureTests = RetainedCaptureTests()
        self.fixture.setUp()
        coordinates: tuple[tuple[int, int], ...] = ((100, 108), (109, 118), (190, 199),
                                                   (201, 209), (210, 218), (240, 249))
        rows: list[bytes] = []
        row: bytes
        before: int
        after: int
        for row, (before, after) in zip(self.fixture.rows, coordinates, strict=True):
            fields: list[bytes] = row.split(b" ")
            fields[5] = f"qpc_before={before}".encode("ascii")
            fields[7] = f"qpc_after={after}".encode("ascii")
            rows.append(b" ".join(fields))
        self.stream: bytes = b"\r\n".join(rows) + b"\r\n"
        self.receipt: bytes = self.fixture.receipt(self.stream)
        self.pin: str = sha256(self.receipt).hexdigest()
        self.first: TRIAL.RetainedExchange
        self.successor: TRIAL.RetainedExchange
        self.first, self.successor = TRIAL.read_retained_captures(self.receipt, self.pin, self.stream)
        self.window: TRIAL.QpcValidityWindow = TRIAL.QpcValidityWindow(self.pin, 100, 200)
        self.span: TRIAL.PairedExchangeSpan = TRIAL.PairedExchangeSpan(self.pin, self.first.exchange_sha256, 101, 198)
        pre_event: TRIAL.TrialEvent = TRIAL.TrialEvent(self.first.exchange_sha256, TRIAL.ApplicationEvent.PRE_SEND)
        receive_event: TRIAL.TrialEvent = TRIAL.TrialEvent(self.first.exchange_sha256, TRIAL.ApplicationEvent.RECEIVE_RETURN)
        self.pre: TRIAL.HeldOutComparison = TRIAL.HeldOutComparison(
            TRIAL.Nanoseconds(9810, 10000),
            TRIAL.HeldOutLeg(pre_event, "b" * 64, "c" * 64, TRIAL.Nanoseconds(10000, 10240),
                             TRIAL.Nanoseconds(-20, -20), TRIAL.Nanoseconds(-200, -180)),
            230, 230,
        )
        self.receive: TRIAL.HeldOutComparison = TRIAL.HeldOutComparison(
            TRIAL.Nanoseconds(19800, 19802),
            TRIAL.HeldOutLeg(receive_event, "b" * 64, "c" * 64, TRIAL.Nanoseconds(20000, 20020),
                             TRIAL.Nanoseconds(5, 8), TRIAL.Nanoseconds(-200, -180)),
            48, 48,
        )

    def compare(self, window: TRIAL.QpcValidityWindow, span: TRIAL.PairedExchangeSpan,
                *, pre: TRIAL.HeldOutComparison | None = None,
                receive: TRIAL.HeldOutComparison | None = None) -> tuple[int, int]:
        return TRIAL.compare_held_out_exchange(
            self.receipt, self.pin, self.stream, window, span,
            pre_send=self.pre if pre is None else pre, receive_return=self.receive if receive is None else receive,
        )

    def test_exact_half_open_endpoints_and_both_numbers(self) -> None:
        self.assertEqual((self.first.pre_send.qpc_before, self.first.receive_return.qpc_after), (100, 199))
        # Independent local-wall ranges [9780, 10040] and [19805, 19848]
        # are compared with [9810, 10000] and [19800, 19802].
        self.assertEqual(self.compare(self.window, self.span), (230, 48))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "application bracket crosses"):
            self.compare(replace(self.window, start_tick=101), self.span)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "application bracket crosses"):
            self.compare(replace(self.window, end_tick_exclusive=199), self.span)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "paired span crosses"):
            self.compare(self.window, replace(self.span, earliest_tick=99))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "paired span crosses"):
            self.compare(self.window, replace(self.span, latest_tick=200))

    def test_complete_small_grid_matches_explicit_point_membership(self) -> None:
        original_ticks: set[int] = set(range(100, 119)) | set(range(190, 200))
        cases = product((99, 100, 101), (199, 200, 201), (99, 100, 101), (198, 199, 200))
        for start, end, earliest, latest in cases:
            window: TRIAL.QpcValidityWindow = replace(self.window, start_tick=start, end_tick_exclusive=end)
            span: TRIAL.PairedExchangeSpan = replace(self.span, earliest_tick=earliest, latest_tick=latest)
            covered: set[int] = set(range(start, end))
            required: set[int] = original_ticks | set(range(earliest, latest + 1))
            if required <= covered:
                self.assertEqual(self.compare(window, span), (230, 48))
            else:
                with self.assertRaisesRegex(TRIAL.TrialRefused, "crosses validity window"):
                    self.compare(window, span)

    def test_chart_identity_and_exchange_selection_are_independent(self) -> None:
        other_pin: str = sha256(self.fixture.receipt(self.stream, "ketiming_000000000002")).hexdigest()
        with self.assertRaisesRegex(TRIAL.TrialRefused, "validity chart differs"):
            self.compare(replace(self.window, receipt_sha256=other_pin), self.span)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "validity chart differs"):
            self.compare(self.window, replace(self.span, receipt_sha256=other_pin))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "span exchange differs"):
            self.compare(self.window, replace(self.span, exchange_sha256="f" * 64))
        wide: TRIAL.QpcValidityWindow = replace(self.window, end_tick_exclusive=260)
        span: TRIAL.PairedExchangeSpan = replace(self.span, exchange_sha256=self.successor.exchange_sha256,
                                                earliest_tick=200, latest_tick=249)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "reference event differs"):
            self.compare(wide, span)
        pre: TRIAL.HeldOutComparison = replace(self.pre, reference=replace(
            self.pre.reference, event=replace(self.pre.reference.event, exchange_sha256=self.successor.exchange_sha256),
            wire_record_sha256="d" * 64, residual_record_sha256="e" * 64,
        ))
        receive: TRIAL.HeldOutComparison = replace(self.receive, reference=replace(
            self.receive.reference, event=replace(self.receive.reference.event, exchange_sha256=self.successor.exchange_sha256),
            wire_record_sha256="d" * 64, residual_record_sha256="e" * 64,
        ))
        self.assertEqual(self.compare(wide, span, pre=pre, receive=receive), (230, 48))

    def test_no_partial_result_when_the_second_leg_is_wrong(self) -> None:
        with self.assertRaisesRegex(TRIAL.TrialRefused, "worst-case disagreement"):
            self.compare(self.window, self.span, receive=replace(self.receive, declared_error_ns=47))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "reference event differs"):
            self.compare(self.window, self.span, receive=self.pre)

    def test_app_artifacts_cannot_be_the_independent_reference(self) -> None:
        pin: str
        field: str
        for pin in (self.pin, sha256(self.stream).hexdigest(), self.first.send_return.row_sha256,
                    self.first.pre_send.row_sha256, self.successor.receive_return.row_sha256):
            for field in ("wire_record_sha256", "residual_record_sha256"):
                with self.subTest(pin=pin, field=field), self.assertRaisesRegex(TRIAL.TrialRefused, "reused as independent"):
                    comparison: TRIAL.HeldOutComparison = replace(self.receive, reference=replace(
                        self.receive.reference, **{field: pin},
                    ))
                    self.compare(self.window, self.span, receive=comparison)

    def test_complete_comparison_rechecks_original_bytes(self) -> None:
        changed: bytes = self.stream.replace(b"qpc_before=100", b"qpc_before=99", 1)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "stream differs from receipt"):
            TRIAL.compare_held_out_exchange(self.receipt, self.pin, changed, self.window, self.span,
                                           pre_send=self.pre, receive_return=self.receive)

    def test_qpc_values_keep_signed_range_and_distinct_endpoint_contracts(self) -> None:
        with self.assertRaisesRegex(TRIAL.TrialRefused, "invalid QPC validity window"):
            replace(self.window, end_tick_exclusive=100)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "invalid QPC validity window"):
            replace(self.window, end_tick_exclusive=1 << 63)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "invalid paired QPC span"):
            replace(self.span, earliest_tick=False)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "invalid paired QPC span"):
            replace(self.span, earliest_tick=-(1 << 63) - 1)
        singleton: TRIAL.PairedExchangeSpan = replace(self.span, earliest_tick=100, latest_tick=100)
        self.assertEqual((singleton.earliest_tick, singleton.latest_tick), (100, 100))


class PinnedReferenceExchangeTests(unittest.TestCase):
    def setUp(self) -> None:
        self.app: HeldOutExchangeTests = HeldOutExchangeTests()
        self.app.setUp()
        self.tx: dict[str, object] = {
            "event": "pre-send", "wire_direction": "tx", "wire_utc_ns": [10000, 10240],
            "application_minus_wire_ns": [-20, -20], "local_wall_minus_utc_ns": [-200, -180],
        }
        self.rx: dict[str, object] = {
            "event": "receive-return", "wire_direction": "rx", "wire_utc_ns": [20000, 20020],
            "application_minus_wire_ns": [5, 8], "local_wall_minus_utc_ns": [-200, -180],
        }
        self.pre: TRIAL.ComparisonBounds = TRIAL.ComparisonBounds(TRIAL.Nanoseconds(9810, 10000), 230, 230)
        self.receive: TRIAL.ComparisonBounds = TRIAL.ComparisonBounds(TRIAL.Nanoseconds(19800, 19802), 48, 48)
        self.wire: bytes = b"separately acquired wire events"
        self.residual: bytes = b"independent local wall residuals"
        self.first: str = sha256(f"{self.app.pin}:first".encode("ascii")).hexdigest()
        self.record: dict[str, object] = {
            "schema_version": 1, "receipt_sha256": self.app.pin, "exchange_sha256": self.first,
            "client_port": 49000, "peer_port": 45000, "frequency_hz": 10000000,
            "span_qpc_ticks": [101, 198], "wire_record_sha256": sha256(self.wire).hexdigest(),
            "residual_record_sha256": sha256(self.residual).hexdigest(),
            "pre_send": self.tx, "receive_return": self.rx,
        }

    def compare(
        self, pairing: bytes, *, pairing_pin: str | None = None, wire: bytes | None = None,
        residual: bytes | None = None, exchange: str | None = None, window: TRIAL.QpcValidityWindow | None = None,
        receive: TRIAL.ComparisonBounds | None = None, stream: bytes | None = None,
    ) -> tuple[int, int]:
        return TRIAL.compare_pinned_reference_exchange(
            self.app.receipt, self.app.pin, self.app.stream if stream is None else stream,
            self.app.window if window is None else window, exchange_sha256=self.first if exchange is None else exchange,
            pairing=pairing, pairing_sha256=sha256(pairing).hexdigest() if pairing_pin is None else pairing_pin,
            wire_record=self.wire if wire is None else wire, residual_record=self.residual if residual is None else residual,
            pre_send=self.pre, receive_return=self.receive if receive is None else receive,
        )

    def encoded(self, **changes: object) -> bytes:
        return json.dumps({**self.record, **changes}).encode("utf-8")

    def test_both_events_compare_against_independent_limits(self) -> None:
        pairing: bytes = self.encoded()
        self.assertEqual(self.compare(pairing), (230, 48))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "worst-case disagreement"):
            self.compare(pairing, receive=replace(self.receive, declared_error_ns=47))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "declared error exceeds leg budget"):
            self.compare(pairing, receive=replace(self.receive, leg_budget_ns=47))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "stream differs from receipt"):
            self.compare(pairing, stream=self.app.stream.replace(b"qpc_before=100", b"qpc_before=99", 1))

    def test_pairing_and_original_artifact_pins_are_checked(self) -> None:
        pairing: bytes = self.encoded()
        pin: str = sha256(pairing).hexdigest()
        with self.assertRaisesRegex(TRIAL.TrialRefused, "pairing digest differs"):
            self.compare(pairing + b" ", pairing_pin=pin)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "wire record digest differs"):
            self.compare(pairing, wire=self.residual)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "residual record digest differs"):
            self.compare(pairing, residual=self.residual + b" ")
        with self.assertRaisesRegex(TRIAL.TrialRefused, "residual record digest differs"):
            self.compare(self.encoded(residual_record_sha256="Z" * 64))

    def test_pairing_cannot_choose_a_different_exchange_inside_the_same_chart(self) -> None:
        wide: TRIAL.QpcValidityWindow = replace(self.app.window, end_tick_exclusive=260)
        successor: str = sha256(f"{self.app.pin}:successor".encode("ascii")).hexdigest()
        pairing: bytes = self.encoded(exchange_sha256=successor, client_port=49001, span_qpc_ticks=[200, 249])
        self.assertEqual(self.compare(pairing, exchange=successor, window=wide), (230, 48))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "pairing receipt or exchange differs"):
            self.compare(pairing, window=wide)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "pairing receipt or exchange differs"):
            self.compare(self.encoded(receipt_sha256="a" * 64))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "validity chart differs"):
            self.compare(self.encoded(), window=replace(wide, receipt_sha256="b" * 64))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "paired span exchange differs"):
            self.compare(self.encoded(exchange_sha256="c" * 64), exchange="c" * 64)

    def test_pairing_roles_socket_and_clock_chart_match_original(self) -> None:
        with self.assertRaisesRegex(TRIAL.TrialRefused, "reference leg role differs"):
            self.compare(self.encoded(pre_send=self.rx, receive_return=self.tx))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "reference leg role differs"):
            self.compare(self.encoded(receive_return={**self.rx, "wire_direction": "tx"}))
        field: str
        value: object
        for field, value in (("client_port", 49001), ("peer_port", "45000"), ("frequency_hz", True),
                             ("frequency_hz", 10_000_001)):
            with self.subTest(field=field, value=value), self.assertRaisesRegex(
                TRIAL.TrialRefused, "ports or QPC frequency differ",
            ):
                self.compare(self.encoded(**{field: value}))

    def test_reference_cannot_be_the_application_stream_or_sibling_row(self) -> None:
        row: bytes = self.app.stream.splitlines(keepends=True)[-1]
        with self.assertRaisesRegex(TRIAL.TrialRefused, "application capture reused"):
            self.compare(self.encoded(wire_record_sha256=sha256(row).hexdigest()), wire=row)
        with self.assertRaisesRegex(TRIAL.TrialRefused, "application capture reused"):
            self.compare(self.encoded(residual_record_sha256=sha256(self.app.stream).hexdigest()),
                         residual=self.app.stream)

    def test_original_brackets_and_supplied_span_fit_half_open_window(self) -> None:
        pairing: bytes = self.encoded(span_qpc_ticks=[100, 199])
        self.assertEqual(self.compare(pairing), (230, 48))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "paired span crosses"):
            self.compare(self.encoded(span_qpc_ticks=[100, 200]))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "application bracket crosses"):
            self.compare(pairing, window=replace(self.app.window, start_tick=101))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "invalid paired QPC span"):
            self.compare(self.encoded(span_qpc_ticks=[101, 100]))

    def test_incomplete_extra_or_duplicate_pairing_fields_are_refused(self) -> None:
        missing: dict[str, object] = dict(self.record)
        del missing["receive_return"]
        pairings: tuple[tuple[bytes, str], ...] = (
            (json.dumps(missing).encode("utf-8"), "pairing fields or version"),
            (self.encoded(schema_version=True), "pairing fields or version"),
            (self.encoded(leg_budget_ns=1000), "pairing fields or version"),
            (self.encoded(receive_return={"event": "receive-return"}), "leg fields"),
            (self.encoded(receive_return={**self.rx, "local_wall_ns": [0, 0]}), "leg fields"),
            (rb'{"\u0073chema_version": 1, ' + self.encoded()[1:], "duplicate reference pairing field"),
            (self.encoded().replace(b'"event": "pre-send"', rb'"event": "pre-send", "\u0065vent": "pre-send"'),
             "duplicate reference pairing field"),
        )
        pairing: bytes
        reason: str
        for pairing, reason in pairings:
            with self.subTest(reason=reason), self.assertRaisesRegex(TRIAL.TrialRefused, reason):
                self.compare(pairing)

    def test_pairing_numbers_are_exact_and_input_sizes_bounded(self) -> None:
        pairing: bytes = self.encoded()
        self.assertEqual(self.compare(pairing.ljust(8192, b" ")), (230, 48))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "pairing exceeds input bounds"):
            self.compare(pairing.ljust(8193, b" "))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "invalid reference pairing JSON"):
            self.compare(pairing + b"\xff")
        wire: bytes = b"w" * 65_536
        self.assertEqual(self.compare(self.encoded(wire_record_sha256=sha256(wire).hexdigest()), wire=wire), (230, 48))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "wire record exceeds input bounds"):
            self.compare(pairing, wire=wire + b"w")
        with self.assertRaisesRegex(TRIAL.TrialRefused, "residual record exceeds input bounds"):
            self.compare(pairing, residual=b"")
        token: bytes
        for token in (b"-0", b"1.01", b"1e2", b"NaN", b"12345678901234567890"):
            with self.subTest(token=token), self.assertRaisesRegex(TRIAL.TrialRefused, "invalid reference pairing integer"):
                self.compare(pairing.replace(b"[101, 198]", b"[" + token + b", 198]"))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "invalid reference pairing coordinates"):
            self.compare(self.encoded(span_qpc_ticks=[True, 198]))
        with self.assertRaisesRegex(TRIAL.TrialRefused, "invalid nanosecond interval"):
            self.compare(self.encoded(receive_return={**self.rx, "wire_utc_ns": [0, 9_223_372_036_854_775_808]}))


if __name__ == "__main__":
    unittest.main()
