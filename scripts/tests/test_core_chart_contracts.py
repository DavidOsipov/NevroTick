"""Pure controls for chart external-fixture inventory and output accounting."""

import hashlib
import importlib.util
import json
import pathlib
import re
import unittest
from fractions import Fraction

ROOT = pathlib.Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("core_chart_contracts", ROOT / "scripts/core-chart-contracts.py")
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
RATE_SPEC = importlib.util.spec_from_file_location("core_rate_delivery_accounting", ROOT / "scripts/core-rate-delivery-accounting.py")
assert RATE_SPEC is not None and RATE_SPEC.loader is not None
RATE = importlib.util.module_from_spec(RATE_SPEC)
RATE_SPEC.loader.exec_module(RATE)


class ChartContractControls(unittest.TestCase):
    """Check literal fixtures without claiming compiler or native execution."""

    @staticmethod
    def healthy():
        return (
            "CORE_CHART_CONTRACT_PROFILE labels=5 contexts=2 monotonic=4 "
            "coordinates=1 exchanges=1 observations=3 histories=1 appends=3 "
            "frames=1 domains=1 charts=1 prepares=4 terms=1 tracked_projections=1 "
            "enclosures=1 getters=22 batches=1 minima=1 selections=1 batch_getters=22 "
            "raw_enclosures=1 raw_batches=1 principal_parameters=1 principal_strata=1 "
            "principal_seeds=1 principal_advances=1 chart_principal_advances=1 principal_getters=27\n"
            "legal-chart-contract\n"
        )

    def test_fixed_legal_recipe_and_literally_bound_source(self):
        self.assertEqual(MODULE.legal_expected(), {
            "labels": 5, "contexts": 2, "monotonic": 4, "coordinates": 1,
            "exchanges": 1, "observations": 3, "histories": 1, "appends": 3,
            "frames": 1, "domains": 1, "charts": 1, "prepares": 4,
            "terms": 1, "tracked_projections": 1, "enclosures": 1, "getters": 22,
            "batches": 1, "minima": 1, "selections": 1, "batch_getters": 22,
            "raw_enclosures": 1, "raw_batches": 1, "principal_parameters": 1,
            "principal_strata": 1, "principal_seeds": 1, "principal_advances": 1,
            "chart_principal_advances": 1, "principal_getters": 27,
        })
        self.assertEqual(sum(MODULE.legal_expected().values()), 111)
        self.assertEqual(MODULE.contract_counts(self.healthy()), MODULE.legal_expected())
        source = MODULE.legal_contract_source()
        self.assertEqual(hashlib.sha256(source.encode()).hexdigest(),
                         "707d00d4716fb8651975b2cbe12bf49917d2854cea97f1e2aba6b24a79dabb02")
        self.assertEqual(len(re.findall(r"call!\(c,15,", source)), 22)
        self.assertIn("enclose_fixture_chart_projection(&p)", source)
        self.assertIn("FixtureChartBatch::new(&chart,t4,&entries)", source)
        self.assertIn("FixtureMinimum::new(1)", source)
        self.assertIn("batch.select(minimum)", source)
        self.assertEqual(len(re.findall(r"call!\(c,19,", source)), 22)
        self.assertIn("FixtureChartOutcome::Refused", source)
        self.assertIn("FixtureChartOutcome::Unknown", source)
        self.assertIn("FixtureChartError::ContextMismatch", source)

    def test_exact_lf_crlf_and_both_record_orders(self):
        a, b = self.healthy().splitlines(keepends=True)
        for text in (a + b, b + a, (a + b).replace("\n", "\r\n"),
                     (b + a).replace("\n", "\r\n")):
            with self.subTest(text=text):
                self.assertEqual(MODULE.contract_counts(text), MODULE.legal_expected())

    def test_incomplete_duplicate_malformed_and_unrelated_records_reject(self):
        text = self.healthy()
        marker, sentinel = text.splitlines(keepends=True)
        bad = (
            "", marker, sentinel, text.rstrip("\n"), marker + marker,
            text + marker, text + "unrelated\n", text.replace("appends=3", "appends=2"),
            text.replace("getters=22", "getters=022"), text.replace("getters=22", "getters=true"),
            text.replace(" charts=1", ""), text.replace("getters=22", "getters=22 injected=1"),
            text.replace("labels=5 contexts=2", "contexts=2 labels=5"),
            text.replace("CORE_CHART_CONTRACT_PROFILE", "CORE_ADAPTIVE_CONTRACT_PROFILE"),
            text.replace("legal-chart-contract", "legal-core-contract"), "x" * 4097,
        )
        for item in bad:
            with self.subTest(item=item):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.contract_counts(item)
        for item in (None, 1, b"invalid", [text], True):
            with self.subTest(item=item):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.contract_counts(item)

    def test_closed_inventory_has_one_legal_and_twenty_four_distinct_negatives(self):
        cases = MODULE.contract_examples()
        expected = {
            "legal_chart_tracking_and_enclosure", "chart_frames_private_update",
            "chart_domain_private_update", "chart_chart_private_update", "chart_view_private_update",
            "chart_projection_private_update", "chart_interval_private_update", "chart_chart_default",
            "chart_projection_default", "chart_live_history_borrow", "chart_descriptor_lifetime",
            "chart_to_legacy_projection", "chart_to_legacy_interval", "chart_interval_in_old_batch",
            "chart_batch_private_update", "chart_selection_private_update",
            "chart_selection_batch_lifetime", "chart_batch_descriptor_lifetime",
            "chart_batch_entries_lifetime", "chart_interval_into_legacy", "chart_batch_live_slice_borrow",
            "chart_principal_private_update", "chart_principal_batch_lifetime",
            "chart_principal_into_legacy_consumer", "chart_principal_legacy_uncertainty",
        }
        self.assertEqual(set(cases), expected)
        self.assertEqual(sum(item["code"] is None for item in cases.values()), 1)
        self.assertEqual(sum(item["code"] is not None for item in cases.values()), 24)
        self.assertEqual(cases[MODULE.LEGAL_NAME]["chart_calls"], MODULE.legal_expected())
        self.assertEqual(cases[MODULE.LEGAL_NAME]["source"], MODULE.legal_contract_source())
        self.assertEqual(len({item["source"] for item in cases.values()}), 25)

    def test_negative_descriptors_bind_error_kind_context_line_and_neighbor(self):
        expected_codes = {
            "chart_frames_private_update": "E0451", "chart_domain_private_update": "E0451",
            "chart_chart_private_update": "E0451", "chart_view_private_update": "E0451",
            "chart_projection_private_update": "E0451", "chart_interval_private_update": "E0451",
            "chart_chart_default": "E0277", "chart_projection_default": "E0277",
            "chart_live_history_borrow": "E0502", "chart_descriptor_lifetime": "E0515",
            "chart_to_legacy_projection": "E0308", "chart_to_legacy_interval": "E0308",
            "chart_interval_in_old_batch": "E0308",
            "chart_batch_private_update": "E0451", "chart_selection_private_update": "E0451",
            "chart_selection_batch_lifetime": "E0515", "chart_batch_descriptor_lifetime": "E0515",
            "chart_batch_entries_lifetime": "E0515", "chart_interval_into_legacy": "E0277",
            "chart_batch_live_slice_borrow": "E0502",
        }
        for name, code in expected_codes.items():
            with self.subTest(name=name):
                case = MODULE.contract_examples()[name]
                self.assertEqual(set(case), {"source", "code", "tokens", "line", "external_legal_neighbor"})
                self.assertEqual(case["code"], code)
                self.assertTrue(case["tokens"] and all(type(token) is str and token for token in case["tokens"]))
                new_cases = {"chart_batch_private_update", "chart_selection_private_update",
                             "chart_selection_batch_lifetime", "chart_batch_descriptor_lifetime",
                             "chart_batch_entries_lifetime", "chart_interval_into_legacy", "chart_batch_live_slice_borrow"}
                neighbor = ({"file": "crates/time-core/tests/private/chart_batch_cases.rs",
                             "test": "chart_batch_cases::chart_batch_selects_genuine_majority_at_common_time"}
                            if name in new_cases else {
                                "file": "crates/time-core/tests/chart.rs",
                                "test": "weighted_tracking_uses_one_common_chart_reference_and_genuine_owner"})
                self.assertEqual(case["external_legal_neighbor"], neighbor)
                lines = case["source"].splitlines()
                self.assertTrue(type(case["line"]) is int and 1 <= case["line"] <= len(lines))
                primary = lines[case["line"] - 1]
                self.assertTrue(primary.startswith(" ") and primary.strip())
                if name == "chart_live_history_borrow":
                    self.assertIn("history.try_append(row)", primary)
                elif name == "chart_descriptor_lifetime":
                    self.assertIn("Known(&chart)", primary)
                elif name == "chart_interval_in_old_batch":
                    self.assertIn("FixtureProjectedBatch::new", primary)


    def test_predecessor_thirteen_negatives_retain_exact_meanings_and_sources(self):
        cases = MODULE.contract_examples()
        old_names = {"chart_frames_private_update", "chart_domain_private_update", "chart_chart_private_update",
                     "chart_view_private_update", "chart_projection_private_update", "chart_interval_private_update",
                     "chart_chart_default", "chart_projection_default", "chart_live_history_borrow",
                     "chart_descriptor_lifetime", "chart_to_legacy_projection", "chart_to_legacy_interval",
                     "chart_interval_in_old_batch"}
        original = {name: cases[name] for name in old_names}
        self.assertEqual(hashlib.sha256(json.dumps(original, sort_keys=True, separators=(",", ":")).encode()).hexdigest(),
                         "129b52c99dfb66754db82cf121efae3642a2df5b762804febf6bdab68ea8113f")

    def test_new_batch_record_fields_are_closed_and_cannot_be_omitted(self):
        raw = self.healthy()
        for field in ("batches", "minima", "selections", "batch_getters"):
            value = 22 if field == "batch_getters" else 1
            for bad in (raw.replace(f" {field}={value}", ""),
                        raw.replace(f" {field}={value}", f" {field}=0"),
                        raw.replace(f" {field}={value}", f" {field}={value} injected=1")):
                with self.subTest(field=field, bad=bad):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.contract_counts(bad)

    def test_new_lifetime_and_mutation_primary_lines_name_the_actual_boundary(self):
        expected = {"chart_selection_batch_lifetime": "batch.select(minimum)",
                    "chart_batch_descriptor_lifetime": "FixtureChartBatch::new(&chart",
                    "chart_batch_entries_lifetime": "FixtureChartBatch::new(chart,at,&entries)",
                    "chart_interval_into_legacy": "value.into()",
                    "chart_batch_live_slice_borrow": "values.reverse()"}
        for name, expression in expected.items():
            with self.subTest(name=name):
                case = MODULE.contract_examples()[name]
                self.assertIn(expression, case["source"].splitlines()[case["line"] - 1])
                self.assertTrue(case["source"].endswith("fn main(){}\n"))
        mutable = MODULE.contract_examples()["chart_batch_live_slice_borrow"]["source"]
        self.assertIn("let selected=batch.select(minimum)", mutable)
        self.assertLess(mutable.index("values.reverse()"), mutable.index("selected.selection()"))

    def test_descriptor_lifetime_probe_owns_its_non_copy_chart(self):
        case = MODULE.contract_examples()["chart_batch_descriptor_lifetime"]
        source = case["source"]
        # The descriptor is already checked, but intentionally local to bypass.
        # Copying it from &FixtureAffineChart adds an unrelated E0507 before the
        # intended returned-borrow diagnostic can qualify this restriction.
        signature = next(line for line in source.splitlines() if line.startswith("fn bypass"))
        self.assertIn("original:FixtureAffineChart", signature)
        self.assertNotIn("original:&", signature)
        self.assertIn(" let chart=original;\n", source)
        self.assertNotIn("*original", source)
        self.assertEqual(case["code"], "E0515")
        self.assertEqual(case["tokens"], ["chart", "cannot return value referencing"])
        self.assertEqual(source.splitlines()[case["line"] - 1],
                         " FixtureChartBatch::new(&chart,at,values).unwrap()")

    def test_s157_receiver_reuses_actual_raw_projection_and_observes_provenance(self):
        source = MODULE.legal_contract_source()
        for expression in (
            "enclose_fixture_projection(raw_projection)",
            "FixtureProjectedBatch::new(b,g,t4,&raw_entries)",
            "FixturePrincipalState::seed(&[a],parameters,minimum,&raw_batch,&strata)",
            "state.advance(&raw_batch,&strata)", "state.advance_chart(&chart,&batch,&strata)",
            "decision.chart_selection()", "decision.batch()", "decision.entries()",
            "decision.principal_entry()", "principal_projection.chart()",
            "principal_projection.source_projection()", "source_state.pending()",
            "FixturePrincipalOutcomeKind::RetainedWithoutNewEstimate",
        ):
            with self.subTest(expression=expression):
                self.assertIn(expression, source)
        self.assertEqual(len(re.findall(r"call!\(c,27,", source)), 27)
        for slot in range(20, 27):
            self.assertEqual(len(re.findall(rf"call!\(c,{slot},", source)), 1)
        # A legal receiver must not buy another history/append/projection setup.
        self.assertEqual(source.count("FixtureAdaptiveHistory::new("), 1)
        self.assertEqual(source.count("history.try_append(row)"), 1)
        self.assertEqual(source.count("view.project_tracking("), 1)

    def test_s157_negative_causes_bind_the_actual_receiver_and_legal_neighbor(self):
        expected = {
            "chart_principal_private_update": ("E0451", ["FixtureChartPrincipalDecision", "private"],
                                               "FixtureChartPrincipalDecision{..value}"),
            "chart_principal_batch_lifetime": ("E0515", ["batch", "cannot return value referencing"],
                                               "state.advance_chart(chart,&batch,strata)"),
            "chart_principal_into_legacy_consumer": ("E0308", ["FixturePrincipalDecision", "FixtureChartPrincipalDecision"],
                                                     "combine_fixture_supporters(value,&[])"),
            "chart_principal_legacy_uncertainty": ("E0308", ["FixtureProjectedInterval", "FixtureChartInterval"],
                                                   "FixtureSupporterUncertainty::new(entry,0.0,0.0,0.0)"),
        }
        cases = MODULE.contract_examples()
        for name, (code, tokens, expression) in expected.items():
            with self.subTest(name=name):
                case = cases[name]
                self.assertEqual(set(case), {"source", "code", "tokens", "line", "external_legal_neighbor"})
                self.assertEqual(case["code"], code)
                self.assertEqual(case["tokens"], tokens)
                self.assertIn(expression, case["source"].splitlines()[case["line"] - 1])
                self.assertEqual(case["external_legal_neighbor"], {
                    "file": "crates/time-core/tests/private/chart_principal_cases.rs",
                    "test": "chart_principal_cases::retained_owner_preserves_credit_and_actual_chart_provenance",
                })
                self.assertTrue(case["source"].endswith("fn main(){}\n"))

    def test_s157_record_fields_require_exact_complete_counts(self):
        raw = self.healthy()
        for field, value in (("raw_enclosures", 1), ("raw_batches", 1),
                             ("principal_parameters", 1), ("principal_strata", 1),
                             ("principal_seeds", 1), ("principal_advances", 1),
                             ("chart_principal_advances", 1), ("principal_getters", 27)):
            for replacement in ("", f" {field}=0", f" {field}=True", f" {field}={value} injected=1"):
                with self.subTest(field=field, replacement=replacement):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.contract_counts(raw.replace(f" {field}={value}", replacement))

    def test_s155_twenty_negative_descriptors_retain_exact_sources_and_metadata(self):
        historical = {name: item for name, item in MODULE.contract_examples().items()
                      if item["code"] is not None and not name.startswith("chart_principal_")}
        self.assertEqual(len(historical), 20)
        self.assertEqual(hashlib.sha256(json.dumps(historical, sort_keys=True, separators=(",", ":")).encode()).hexdigest(),
                         "a65c99363c803165d5b31f9fe92bdb8448f9f28008791ae6ebbc03da83ace7f0")

    def test_six_garde_carriers_are_a_gap_map_not_mutation_credit(self):
        self.assertEqual(set(MODULE.GARDE_CARRIERS), {
            "FramesCandidate", "DomainCandidate", "AffineCandidate", "HistoryCandidate",
            "ProjectionCandidate", "TrackingPhaseCandidate",
        })
        self.assertTrue(all(type(value) is str and value for value in MODULE.GARDE_CARRIERS.values()))
        self.assertNotIn("killed", MODULE.contract_counts(self.healthy()))
        self.assertNotIn("native_exit", MODULE.contract_counts(self.healthy()))


class CompositionContractControls(unittest.TestCase):
    """Keep the new focused composition registry independent of old consumers."""

    @staticmethod
    def healthy():
        return (
            "CORE_CHART_COMPOSITION_CONTRACT_PROFILE labels=10 monotonic=3 "
            "frames=5 domains=1 charts=5 compositions=3 getters=28\n"
            "legal-chart-composition-contract\n"
        )

    def test_closed_focused_registry_has_one_legal_and_six_bound_probes(self):
        cases = MODULE.composition_contract_examples()
        self.assertEqual(set(cases), {
            "legal_chart_composition_and_retained_bounds",
            "chart_composition_phase_bound_private_update",
            "chart_composition_rate_bound_private_update",
            "chart_composition_phase_bound_private_write",
            "chart_composition_rate_bound_private_write",
            "chart_composition_phase_bound_owned_getter",
            "chart_composition_rate_bound_owned_getter",
        })
        self.assertEqual(sum(case["code"] is None for case in cases.values()), 1)
        self.assertEqual(sum(case["code"] is not None for case in cases.values()), 6)
        self.assertEqual(len({case["source"] for case in cases.values()}), 7)
        legal = cases["legal_chart_composition_and_retained_bounds"]
        self.assertEqual(legal["source"], MODULE.composition_legal_source())
        self.assertEqual(legal["chart_composition_calls"], MODULE.composition_legal_expected())
        self.assertEqual(legal["calls"], {
            "direct_fit_calls": 0, "stats_calls": 0, "point_admissions": 0,
        })

    def test_independent_direct_census_and_complete_record(self):
        self.assertEqual(MODULE.composition_legal_expected(), {
            "labels": 10, "monotonic": 3, "frames": 5, "domains": 1,
            "charts": 5, "compositions": 3, "getters": 28,
        })
        self.assertEqual(sum(MODULE.composition_legal_expected().values()), 55)
        self.assertEqual(MODULE.composition_contract_counts(self.healthy()),
                         MODULE.composition_legal_expected())

    def test_legal_source_charges_every_direct_call_and_preserves_operands(self):
        source = MODULE.composition_legal_source()
        self.assertEqual(hashlib.sha256(source.encode()).hexdigest(),
                         "395218b250d35c76f54238f7cb6efbc8172b7da4a38aa5b49d985fd719e7e5fd")
        for slot, count in enumerate((10, 3, 5, 1, 5, 3, 28)):
            with self.subTest(slot=slot):
                self.assertEqual(len(re.findall(rf"call!\(c,{slot},", source)), count)
        for expression in (
            "first.compose(&unit)", "negative.compose(&zero)", "left.compose(&right)",
            "left_before", "right_before", "(1.0..8.0).contains(&phase_error)",
            "(1.0..8.0).contains(&rate_error)", "combined.frames()),ae",
            "assert!(phase_error>=phase_inherited+2.0)",
            "assert!(rate_error>=rate_inherited+2.0)",
            "combined.domain()),domain", "first.phase_at_reference_seconds()),big",
            "first.rate_seconds_per_second()),big", "assert_eq!(c.0,EXPECTED)",
        ):
            with self.subTest(expression=expression):
                self.assertIn(expression, source)
        self.assertEqual(source.count("FixtureAffineChart::new("), 5)
        self.assertEqual(source.count(".compose("), 3)
        for forbidden in ("FixtureAdaptiveHistory", "try_append", "project_tracking",
                          "FixtureChartBatch", "advance_chart", "FixtureReferenceInputState"):
            self.assertNotIn(forbidden, source)

    def test_stored_input_lost_unit_expectation_has_an_independent_exact_reference(self):
        # Exact stored binary64 inputs sum to1; two separately rounded operands
        # both lose the unit. This checks fixture truth without running Rust.
        big = 9_007_199_254_740_992.0
        exact = sum((Fraction.from_float(value) for value in (big, 1.0, -big, 0.0)),
                    Fraction(0))
        point = (big + 1.0) + (-big + 0.0)
        self.assertEqual(exact, Fraction(1))
        self.assertEqual(Fraction.from_float(point), Fraction(0))
        self.assertEqual(abs(exact - Fraction.from_float(point)), Fraction(1))
        unit_roundoff = Fraction(1, 2**53)
        self.assertEqual(unit_roundoff * (abs(Fraction.from_float(big))
                                         + abs(Fraction.from_float(-big))), Fraction(2))

    def test_composition_parser_accepts_only_complete_lf_crlf_records_in_either_order(self):
        a, b = self.healthy().splitlines(keepends=True)
        for output in (a + b, b + a, (a + b).replace("\n", "\r\n"),
                       (b + a).replace("\n", "\r\n")):
            with self.subTest(output=output):
                self.assertEqual(MODULE.composition_contract_counts(output), {
                    "labels": 10, "monotonic": 3, "frames": 5, "domains": 1,
                    "charts": 5, "compositions": 3, "getters": 28,
                })

    def test_composition_parser_rejects_missing_changed_duplicate_and_foreign_rows(self):
        healthy = self.healthy()
        marker, sentinel = healthy.splitlines(keepends=True)
        bad = ["", marker, sentinel, healthy.rstrip("\n"), marker + marker,
               sentinel + sentinel, healthy + marker, healthy + "unrelated\n",
               healthy.replace("labels=10 monotonic=3", "monotonic=3 labels=10"),
               healthy.replace("CORE_CHART_COMPOSITION", "CORE_CHART"),
               healthy.replace("legal-chart-composition-contract", "legal-chart-contract"),
               ChartContractControls.healthy(), "x" * 4097]
        for name, value in (("labels", 10), ("monotonic", 3), ("frames", 5),
                            ("domains", 1), ("charts", 5), ("compositions", 3),
                            ("getters", 28)):
            for replacement in ("", f" {name}=0", f" {name}=0{value}",
                                f" {name}=True", f" {name}={value} injected=1"):
                bad.append(healthy.replace(f" {name}={value}", replacement))
        for output in [*bad, None, True, 1, b"invalid", [healthy]]:
            with self.subTest(output=output):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.composition_contract_counts(output)

    def test_six_negatives_bind_both_coefficient_fields_and_owned_getter_types(self):
        cases = MODULE.composition_contract_examples()
        for name, field, getter in (
            ("phase", "phase_error", "phase_coefficient_roundoff_seconds"),
            ("rate", "rate_error", "rate_coefficient_roundoff_seconds_per_second"),
        ):
            expected = {
                "private_update": ("E0451", ["FixtureAffineChart", "private"],
                                   f" let _=FixtureAffineChart{{{field}:0.0,..value}};"),
                "private_write": ("E0616", [field, "private"], f" value.{field}=0.0;"),
                "owned_getter": ("E0308", ["&mut f64", "f64"],
                                 f" let _: &mut f64=value.{getter}();"),
            }
            for suffix, (code, tokens, primary) in expected.items():
                with self.subTest(name=name, suffix=suffix):
                    case = cases[f"chart_composition_{name}_bound_{suffix}"]
                    self.assertEqual(set(case), {
                        "source", "code", "tokens", "line", "external_legal_neighbor",
                    })
                    self.assertEqual(case["code"], code)
                    self.assertEqual(case["tokens"], tokens)
                    self.assertEqual(case["source"].splitlines()[case["line"] - 1], primary)
                    self.assertEqual(case["external_legal_neighbor"], {
                        "file": "crates/time-core/tests/private/chart_composition.rs",
                        "test": "chart_composition::composition_retains_both_operand_errors_through_signed_elapsed",
                    })
                    self.assertTrue(case["source"].endswith("fn main(){}\n"))
                    self.assertNotIn("::new(", case["source"])
                    self.assertNotIn(".compose(", case["source"])

    def test_entire_historical_registry_and_legal_source_remain_byte_identical(self):
        historical = json.dumps(MODULE.contract_examples(), sort_keys=True,
                                separators=(",", ":")).encode()
        self.assertEqual(hashlib.sha256(historical).hexdigest(),
                         "62ec0f43ffa3dd6ebdc2de170c370a3d34fae5c9b71f1e63e922d1a694436e09")
        self.assertEqual(hashlib.sha256(MODULE.legal_contract_source().encode()).hexdigest(),
                         "707d00d4716fb8651975b2cbe12bf49917d2854cea97f1e2aba6b24a79dabb02")

    def test_historical_three_source_owners_and_current_rate_pins_keep_two_garde_gaps(self):
        self.assertEqual(MODULE.COMPOSITION_SOURCE_SHA256, {
            "crates/time-core/src/regression/chart.rs":
                "8c1eaae20f287b96f0859b79a4522cf3176ead59f5f46a35c5cfa797aded2221",
            "crates/time-core/src/regression/chart/roundoff.rs":
                "1aa537417ef06238d85e7614124e399a301f1e8ecd8e6fdf74b6506e58e395bc",
            "crates/time-core/src/chart_batch.rs":
                "b25ac73756f94bd2be9692130d8dee201bd8c2b0a9c0adf63c7eba929740c79c",
        })
        current_rate = {name: hashlib.sha256((ROOT / name).read_bytes()).hexdigest()
                        for name in RATE.SOURCE_SHA256}
        self.assertEqual(RATE.admit_sources(current_rate), current_rate)
        for name, digest in MODULE.COMPOSITION_SOURCE_SHA256.items():
            with self.subTest(name=name):
                self.assertEqual(hashlib.sha256((ROOT / name).read_bytes()).hexdigest(),
                                 RATE.SOURCE_SHA256.get(name, digest))
        self.assertEqual(set(MODULE.COMPOSITION_GARDE_CARRIERS), {
            "CompositionCandidate", "AffineCandidate",
        })
        self.assertIn("both coefficient bounds", MODULE.COMPOSITION_GARDE_CARRIERS["AffineCandidate"])
        self.assertNotIn("killed", MODULE.composition_legal_expected())
        self.assertNotIn("native_exit", MODULE.composition_legal_expected())


if __name__ == "__main__":
    unittest.main()
