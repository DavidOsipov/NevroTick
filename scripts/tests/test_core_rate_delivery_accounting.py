"""Independent finite rate-delivery accounting controls; no Rust/native work."""
import hashlib
import importlib.util
import json
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "core_rate_delivery_accounting", ROOT / "scripts/core-rate-delivery-accounting.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

# Independent source-authoring census: seven public families and four numerical
# families plus one existing lifecycle consumer. The ordinary Rust total does
# not grow; neither parser nor reservation helper authors these source facts.
NAMES = (
    "dyadic_delivery_and_cancellation_use_active_time_only",
    "span_and_rate_admission_preserve_valid_neighbors",
    "noeffect_and_unknown_have_distinct_terminal_behavior",
    "cursor_carries_identity_and_nonoverlap_across_replacement_owners",
    "current_owner_and_chart_relationships_are_required_before_consumption",
    "admitted_failures_and_exhaustion_cannot_be_retried",
    "settled_translation_preserves_genuine_retained_histories_and_credit",
    "regression::chart::roundoff::tests::settled_rate_preserves_general_binary64_integration_error",
    "regression::chart::roundoff::tests::settled_rate_keeps_dyadic_and_signed_zero_points",
    "regression::chart::roundoff::tests::settled_rate_uses_exact_active_duration_and_integer_neighbors",
    "regression::chart::roundoff::tests::settled_rate_rejects_unsupported_rates_and_reachable_results",
    "combination::lifecycle::chart::guard_tests::chart_raw_pair_guards_preserve_complete_relationships",
)
DOCS = (
    "crates/time-core/src/reference_input/rate_delivery.rs - reference_input::rate_delivery::FixtureRateSpan (line 190) - compile fail",
    "crates/time-core/src/reference_input/rate_delivery.rs - reference_input::rate_delivery::FixtureRateSpan (line 197) - compile fail",
    "crates/time-core/src/reference_input/rate_delivery.rs - reference_input::rate_delivery::FixtureSettledRateInterval (line 252) - compile fail",
    "crates/time-core/src/reference_input/rate_delivery.rs - reference_input::rate_delivery::FixtureRateSettlementCursor (line 383) - compile fail",
    "crates/time-core/src/reference_input/rate_delivery.rs - reference_input::rate_delivery::FixtureRateSettlementCompletion (line 482) - compile fail",
    "crates/time-core/src/reference_input/rate_delivery.rs - reference_input::rate_delivery::FixtureReferenceInputState<'_>::receive_rate_settlement (line 649) - compile fail",
)
CASE_FIELDS = (
    "span_constructions", "interval_constructions", "cursor_constructions",
    "reference_owner_constructions", "receiver_attempts",
    "direct_numeric_effect_attempts", "direct_numeric_rate_support_checks",
    "genuine_history_fixture_envelopes",
)
CASE_VALUES = (
    (5, 5, 5, 5, 5, 0, 0, 0), (3, 6, 1, 1, 1, 0, 0, 0),
    (1, 1, 1, 3, 5, 0, 0, 0), (9, 9, 1, 4, 11, 0, 0, 0),
    (1, 4, 2, 1, 6, 0, 0, 0), (7, 7, 5, 8, 9, 0, 0, 0),
    (1, 1, 1, 2, 1, 0, 0, 1), (0, 0, 0, 0, 0, 2, 0, 0),
    (0, 0, 0, 0, 0, 8, 0, 0), (0, 0, 0, 0, 0, 6, 0, 0),
    (0, 0, 0, 0, 0, 4, 13, 0),
    (0, 0, 0, 0, 0, 1, 0, 0),
)


def outcomes(names=NAMES, status="ok"):
    """Author complete literal libtest rows independently of product constants."""
    return "".join("test " + name + " ... " + status + "\n" for name in names)


def recipes():
    """Independent complete source classifications, never operation telemetry."""
    return {name: dict(zip(CASE_FIELDS, values, strict=True))
            for name, values in zip(NAMES, CASE_VALUES, strict=True)}


class RateDeliveryAccountingTests(unittest.TestCase):
    def test_existing_asymmetric_consumer_adds_one_effect_without_new_producers(self):
        name = ('combination::lifecycle::chart::guard_tests::'
                'chart_raw_pair_guards_preserve_complete_relationships')
        self.assertEqual(MODULE.case_recipes()[name], {
            'span_constructions': 0, 'interval_constructions': 0,
            'cursor_constructions': 0, 'reference_owner_constructions': 0,
            'receiver_attempts': 0, 'direct_numeric_effect_attempts': 1,
            'direct_numeric_rate_support_checks': 0, 'genuine_history_fixture_envelopes': 0})
        vector = MODULE.package_overlay()
        self.assertEqual(vector['rate.direct_numeric_effect_attempts'], 21)
        self.assertEqual(vector['rate.total_settled_effect_attempts_upper'], 51)
        self.assertEqual(vector['rate.total_rate_support_requests_upper'], 130)
        partial = MODULE.profile_observation('test ' + name + ' ... ok\n')
        self.assertEqual(partial['rate_test_statuses'][name], 'ok')
        self.assertTrue(partial['rate_source_only_pending'])

    def test_frozen_overlay_matches_independent_census(self):
        vector = MODULE.package_overlay(include_doctests=True)
        self.assertEqual(len(vector), 55)
        self.assertTrue(all(key.startswith("rate.") for key in vector))
        digest = hashlib.sha256(json.dumps(
            vector, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        self.assertEqual(digest, "1d3d0f3c40ae37dac50bb333b7810dc0a97ccd7fa35bbef5789576afd8979db6")
        self.assertEqual(vector["rate.receiver_attempts"], 38)
        self.assertEqual(vector["rate.affine_candidate_admissions_upper"], 97)
        self.assertEqual(vector["rate.reference_owner_constructions"], 24)
        self.assertEqual(vector["rate.fixture_direct_charts"], 16)
        self.assertEqual(vector["rate.fixture_direct_frames"], 65)
        self.assertEqual(vector["rate.settlement_garde_validations_upper"], 38)
        self.assertEqual(vector["rate.settlement_garde_callbacks_upper"], 38)
        self.assertEqual(vector["rate.cursor_state_context_ordinal_mapping_checks_upper"], 76)
        self.assertEqual(vector["rate.reference_identity_checks_upper"], 76)
        self.assertEqual(vector["rate.settlement_relationship_passes_upper"], 76)
        self.assertEqual(vector["rate.affine_candidate_garde_callbacks_upper"], 388)
        self.assertEqual(vector["rate.affine_candidate_typed_scalar_checks_upper"], 388)
        self.assertEqual(vector["rate.ordinal_consumptions_upper"], 38)
        self.assertEqual(vector["rate.fixture_observer_readout_requests_upper"], 6768)
        self.assertEqual(vector["rate.known_effect_attempts_upper"], 30)
        self.assertEqual(vector["rate.known_relationship_passes_upper"], 60)
        self.assertEqual(vector["rate.total_settled_effect_attempts_upper"], 51)
        self.assertEqual(vector["rate.total_new_outward_bound_requests_upper"], 1440)
        self.assertNotIn("charged_fit_work_upper", vector)
        self.assertNotIn("point_admissions_upper", vector)

    def test_docs_add_only_six_compilations_and_no_runtime_credit(self):
        ordinary = MODULE.package_overlay()
        docs = MODULE.package_overlay(include_doctests=True)
        changed = {key: (ordinary[key], value) for key, value in docs.items()
                   if ordinary[key] != value}
        self.assertEqual(changed, {"rate.new_compile_fail_doc_compilations_upper": (0, 6)})
        self.assertEqual(docs["rate.new_doctest_executed_examples"], 0)
        self.assertEqual(MODULE.validate_overlay(docs, include_doctests=True), docs)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_overlay(ordinary, include_doctests=True)

    def test_overlay_rejects_every_discount_and_bad_integer(self):
        original = MODULE.package_overlay(include_doctests=True)
        for key, amount in original.items():
            if amount:
                discounted = dict(original, **{key: amount - 1})
                with self.subTest(discounted=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_overlay(discounted, include_doctests=True)
            for bad in (True, False, -1, 1 << 63, 1.0, "1", None):
                changed = dict(original, **{key: bad})
                with self.subTest(field=key, bad=bad), self.assertRaises(MODULE.Incomplete):
                    MODULE.validate_overlay(changed, include_doctests=True)
        self.assertEqual(MODULE.validate_overlay(original, include_doctests=True), original)

    def test_overlay_closed_shape_and_defensive_copy(self):
        original = MODULE.package_overlay()
        missing = dict(original)
        missing.pop("rate.receiver_attempts")
        unknown = dict(original, **{"achieved_phase.receiver_attempts": 36})
        for bad in (None, [], missing, unknown):
            with self.subTest(bad=type(bad)), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_overlay(bad)
        checked = MODULE.validate_overlay(original)
        checked["rate.receiver_attempts"] = 0
        self.assertEqual(MODULE.package_overlay()["rate.receiver_attempts"], 38)

    def test_twelve_source_recipes_have_independent_finite_totals(self):
        expected = recipes()
        self.assertEqual(MODULE.case_recipes(), expected)
        self.assertEqual(tuple(MODULE.CURRENT_ORDINARY_TESTS), NAMES)
        self.assertEqual(tuple(sum(row[field] for row in expected.values())
                               for field in CASE_FIELDS), (27, 33, 16, 24, 38, 21, 13, 1))
        self.assertEqual(MODULE.validate_case_recipes(expected), expected)
        expected[NAMES[0]]["receiver_attempts"] = 0
        self.assertEqual(MODULE.case_recipes()[NAMES[0]]["receiver_attempts"], 5)

    def test_source_recipes_reject_renamed_missing_and_undercharged_cases(self):
        for name in NAMES:
            bad = recipes()
            bad["rate_delivery::unknown"] = bad.pop(name)
            with self.subTest(renamed=name), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_case_recipes(bad)
            bad = recipes()
            del bad[name]
            with self.subTest(missing=name), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_case_recipes(bad)
        for field in CASE_FIELDS:
            bad = recipes()
            bad[NAMES[0]][field] = True
            with self.subTest(boolean=field), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_case_recipes(bad)
        bad = recipes()
        bad[NAMES[6]]["genuine_history_fixture_envelopes"] = 0
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_case_recipes(bad)

    def test_all_eight_source_pins_match_current_frozen_bytes(self):
        pins = MODULE.SOURCE_SHA256
        self.assertEqual(len(pins), 8)
        actual = {name: hashlib.sha256((ROOT / name).read_bytes()).hexdigest() for name in pins}
        self.assertEqual(MODULE.admit_sources(actual), actual)
        accepted = MODULE.admit_sources(actual)
        accepted.clear()
        self.assertEqual(len(MODULE.SOURCE_SHA256), 8)

    def test_source_admission_rejects_drift_missing_and_unknown_dependencies(self):
        pins = dict(MODULE.SOURCE_SHA256)
        for path in pins:
            changed = dict(pins)
            changed[path] = "0" * 64
            missing = dict(pins)
            del missing[path]
            for bad in (changed, missing):
                with self.subTest(path=path), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_sources(bad)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_sources(dict(pins, unknown="0" * 64))

    def test_complete_literal_outcomes_do_not_infer_inner_counts(self):
        for text in (outcomes(), outcomes(reversed(NAMES)), outcomes().replace("\n", "\r\n")):
            observed = MODULE.profile_observation(text, require_complete=True)
            self.assertFalse(observed["rate_source_only_pending"])
            self.assertEqual(observed["rate_source_test_observation"], "complete")
            self.assertEqual(observed["rate_test_statuses"], dict.fromkeys(NAMES, "ok"))
            self.assertIsNone(observed["observed_operation_counts"])
            self.assertEqual(observed["rate_recorded_source_work_upper"], MODULE.package_overlay())

    def test_missing_failed_ignored_and_partial_never_refund_source_work(self):
        missing = MODULE.profile_observation("")
        self.assertEqual(missing["rate_test_statuses"], dict.fromkeys(NAMES, "missing"))
        self.assertIsNone(missing["rate_recorded_source_work_upper"])
        for text in (outcomes(NAMES[:1]), outcomes(NAMES[:1], "FAILED"),
                     outcomes(NAMES[:1], "ignored"), outcomes(NAMES[:-1])):
            observed = MODULE.profile_observation(text)
            self.assertTrue(observed["rate_source_only_pending"])
            self.assertIsNone(observed["observed_operation_counts"])
            self.assertEqual(observed["rate_recorded_source_work_upper"], MODULE.package_overlay())
            with self.subTest(text=text[:90]), self.assertRaises(MODULE.Incomplete):
                MODULE.profile_observation(text, require_complete=True)

    def test_each_failed_or_ignored_axis_blocks_complete_observation(self):
        for name in NAMES:
            for status in ("FAILED", "ignored"):
                text = outcomes().replace("test " + name + " ... ok\n",
                                          "test " + name + " ... " + status + "\n")
                observed = MODULE.profile_observation(text)
                self.assertEqual(observed["rate_test_failed"], [name])
                self.assertEqual(observed["rate_test_statuses"][name], status)
                with self.subTest(name=name, status=status), self.assertRaises(MODULE.Incomplete):
                    MODULE.profile_observation(text, require_complete=True)

    def test_duplicate_unknown_malformed_and_unterminated_outcomes_reject(self):
        good = outcomes(NAMES[:1])
        variants = (
            good + good, good.rstrip("\n"), good.replace(" ... ", " .. "),
            good.replace(" ... ok", " ... passed"), good.replace(" ... ok", " ... ok "),
            outcomes(("rate_delivery::unknown",)),
            outcomes(("regression::chart::roundoff::tests::settled_rate_unknown",)),
            outcomes(("combination::lifecycle::chart::guard_tests::chart_raw_pair_guards_unknown",)),
        )
        for text in variants:
            with self.subTest(text=text[:100]), self.assertRaises(MODULE.Incomplete):
                MODULE.profile_observation(text)
        self.assertEqual(MODULE.profile_observation(good)["rate_test_statuses"][NAMES[0]], "ok")

    def test_historical_rows_json_and_replay_prose_are_not_rate_observations(self):
        text = (
            "test reference_input_readiness::achieved_phase_dyadics_change_coordinates_and_preserve_rate ... ok\n"
            "test ordinary_unrelated_package_case ... ok\n"
            + json.dumps({"output": outcomes()}) + "\n"
            + "replayed " + outcomes(NAMES[:1])
        )
        observed = MODULE.profile_observation(text)
        self.assertTrue(observed["rate_source_only_pending"])
        self.assertEqual(observed["rate_test_missing"], list(NAMES))
        self.assertIsNone(observed["rate_recorded_source_work_upper"])

    def test_six_exact_compile_fail_outcomes_have_no_runtime_credit(self):
        self.assertEqual(tuple(MODULE.DOCTESTS), DOCS)
        observed = MODULE.profile_observation(outcomes() + outcomes(DOCS),
                                              require_complete=True, require_doctests=True)
        self.assertEqual(observed["rate_literal_doctests"], list(DOCS))
        self.assertFalse(observed["rate_doctest_pending"])
        self.assertFalse(observed["rate_source_only_pending"])
        self.assertEqual(observed["rate_recorded_source_work_upper"],
                         MODULE.package_overlay(include_doctests=True))
        self.assertIsNone(observed["observed_operation_counts"])

    def test_partial_failed_ignored_docs_reserve_all_six_and_stay_pending(self):
        for text in (outcomes(DOCS[:1]), outcomes(DOCS[:1], "FAILED"),
                     outcomes(DOCS[:1], "ignored"), outcomes(DOCS[:-1])):
            observed = MODULE.profile_observation(outcomes() + text)
            self.assertTrue(observed["rate_doctest_pending"])
            self.assertEqual(observed["rate_recorded_source_work_upper"],
                             MODULE.package_overlay(include_doctests=True))
            with self.subTest(text=text[:100]), self.assertRaises(MODULE.Incomplete):
                MODULE.profile_observation(outcomes() + text, require_doctests=True)

    def test_doc_identity_drift_duplicates_and_unterminated_rows_reject(self):
        good = outcomes(DOCS[:1])
        variants = (good + good, good[:-1], good.replace("line 190", "line 191"),
                    good.replace("compile fail", "compile_fail"),
                    good.replace("FixtureRateSpan", "Unknown"),
                    good.replace("rate_delivery.rs", "other.rs"))
        for text in variants:
            with self.subTest(text=text[:100]), self.assertRaises(MODULE.Incomplete):
                MODULE.profile_observation(text)
        self.assertEqual(MODULE.profile_observation(good)["rate_literal_doctests"], [DOCS[0]])

    def test_modes_and_output_have_exact_bounded_types(self):
        for bad in (0, 1, None, "true"):
            for call in (lambda: MODULE.package_overlay(include_doctests=bad),
                         lambda: MODULE.profile_observation("", require_complete=bad),
                         lambda: MODULE.profile_observation("", require_doctests=bad)):
                with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                    call()
        for bad in (None, [], b"", "x" * (16 * 1024 * 1024 + 1)):
            with self.subTest(kind=type(bad)), self.assertRaises(MODULE.Incomplete):
                MODULE.profile_observation(bad)


if __name__ == "__main__":
    unittest.main()
