"""Closed additional-suite controls; prepare inputs without Rust or solver work."""
from __future__ import annotations

import copy
import importlib.util
from pathlib import Path
import re
import tempfile
import unittest


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError("Cannot load a local preparation module")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


SUITES = load("additional_suites_test", HERE / "additional_suites.py")


class AdditionalSuiteTests(unittest.TestCase):
    def test_each_selected_proof_closure_includes_registry_and_only_selected_code(self):
        for selector, stem in (("coordinate-exchange", "coordinate_exchange"), ("clock", "clock"),
                               ("leap-calendar", "leap_calendar"), ("relative-range", "relative_range"), ("exchange-composition", "exchange_composition"), ("calendar-composition", "calendar_composition"), ("leap-ticks", "leap_ticks"), ("calendar-finite-laws", "calendar_finite_laws"), ("leap-coverage", "leap_coverage")):
            with self.subTest(selector=selector):
                expected = {
                    "tools/formal/additional_suites.py", f"tools/formal/{stem}_projection.py",
                    f"tools/formal/{stem}_harness.rs"}
                if selector == "calendar-finite-laws":
                    expected.update(("tools/formal/calendar_composition_harness.rs",
                                     "tools/formal/calendar_composition_projection.py"))
                if selector == "leap-coverage":
                    expected.add("tools/formal/test_leap_coverage.py")
                self.assertEqual(set(SUITES.input_names(selector)), expected)
                self.assertTrue(all((ROOT / name).is_file() for name in SUITES.input_names(selector)))

    def test_unknown_selectors_fail_closed(self):
        for selector in ("all", "era", "coordinate", "leap_calendar", "", None, True, []):
            with self.subTest(selector=selector):
                with self.assertRaises(ValueError):
                    SUITES.input_names(selector)
                with self.assertRaises(ValueError):
                    SUITES.jobs(selector)

    def test_fixed_counts_budgets_and_restoration(self):
        for selector, count, subjects in (("coordinate-exchange", 10, ["coordinate", "exchange"]),
                                         ("clock", 10, ["clock", "clock"]),
                                         ("leap-calendar", 7, ["leap_calendar", "leap_calendar"]),
                                         ("relative-range", 5, ["relative_range"]),
                                         ("exchange-composition", 10, ["exchange_composition"] * 3),
                                         ("calendar-composition", 11, ["calendar_composition"] * 4),
                                         ("leap-ticks", 5, ["leap_ticks"]),
                                         ("calendar-finite-laws", 5, ["calendar_finite_laws"]),
                                         ("leap-coverage", 5, ["leap_coverage"])):
            with self.subTest(selector=selector):
                jobs = SUITES.jobs(selector)
                self.assertEqual(len(jobs), count)
                self.assertLessEqual(len(jobs), 12)
                self.assertEqual([j["subject"] for j in jobs[:len(subjects)]], subjects)
                self.assertEqual([j["subject"] for j in jobs[-len(subjects):]], subjects)
                self.assertTrue(all(j["case"] == "healthy" and not j["restored_healthy"] for j in jobs[:len(subjects)]))
                self.assertTrue(all(j["case"] == "healthy" and j["restored_healthy"] for j in jobs[-len(subjects):]))
                self.assertTrue(all(j["intended_failure"] and j["case"] != "healthy" for j in jobs[len(subjects):-len(subjects)]))
                jobs[0]["harness"] = "changed by caller"
                self.assertNotEqual(SUITES.jobs(selector)[0]["harness"], "changed by caller")

    def test_schedules_match_actual_projection_control_contracts(self):
        clock = load("additional_clock_metadata_test", HERE / "clock_projection.py")
        calendar = load("additional_calendar_metadata_test", HERE / "leap_calendar_projection.py")
        combined = load("additional_coordinate_metadata_test", HERE / "coordinate_exchange_projection.py")
        self.assertEqual(SUITES.jobs("clock"), clock.recommended_schedule())
        self.assertEqual(SUITES.jobs("leap-calendar"), calendar.schedule())
        relative = load("additional_relative_metadata_test", HERE / "relative_range_projection.py")
        self.assertEqual(SUITES.jobs("relative-range"), relative.schedule())
        composition = load("additional_composition_metadata_test", HERE / "exchange_composition_projection.py")
        self.assertEqual(SUITES.jobs("exchange-composition"), composition.schedule())
        calendar_composition = load("additional_calendar_composition_metadata_test", HERE / "calendar_composition_projection.py")
        self.assertEqual(SUITES.jobs("calendar-composition"), calendar_composition.schedule())
        leap_ticks = load("additional_leap_ticks_metadata_test", HERE / "leap_ticks_projection.py")
        self.assertEqual(SUITES.jobs("leap-ticks"), leap_ticks.schedule())
        finite_laws = load("additional_finite_laws_metadata_test", HERE / "calendar_finite_laws_projection.py")
        self.assertEqual(SUITES.jobs("calendar-finite-laws"), finite_laws.schedule())
        coverage = load("additional_leap_coverage_metadata_test", HERE / "leap_coverage_projection.py")
        self.assertEqual(SUITES.jobs("leap-coverage"), coverage.schedule())
        for job in SUITES.jobs("coordinate-exchange"):
            if job["case"] == "healthy":
                continue
            control = combined.CONTROLS[job["subject"]][job["case"]]
            self.assertEqual((job["harness"], job["intended_failure"]),
                             (control["harness"], control["intended_failure"]))

    def test_healthy_obligations_are_fixed_nonempty_and_immutable(self):
        expected_sizes = {
            ("coordinate", "coordinate_full_domain"): (2, 7),
            ("exchange", "exchange_full_domain"): (6, 6),
            ("clock", "monotonic_full_domain"): (6, 6),
            ("clock", "active_awake_full_domain"): (6, 6),
            ("leap_calendar", "leap_calendar_full_domain"): (1, 6),
            ("leap_calendar", "leap_calendar_oracle_laws"): (7, 4),
            ("relative_range", "relative_full_domain"): (3, 7),
            ("exchange_composition", "exchange_phase_full_domain"): (3, 4),
            ("exchange_composition", "exchange_rtt_full_domain"): (3, 4),
            ("exchange_composition", "exchange_scaling_full_domain"): (1, 4),
            ("calendar_composition", "calendar_source_full_domain"): (6, 4),
            ("calendar_composition", "calendar_representation_full_domain"): (3, 4),
            ("calendar_composition", "calendar_suffix_full_domain"): (2, 4),
            ("calendar_composition", "calendar_gregorian_laws"): (8, 4),
            ("leap_ticks", "leap_ticks_full_domain"): (8, 7),
            ("calendar_finite_laws", "calendar_finite_laws_full_domain"): (7, 4),
            ("leap_coverage", "leap_coverage_full_domain"): (9, 13),
        }
        for (subject, harness), sizes in expected_sizes.items():
            with self.subTest(subject=subject, harness=harness):
                assertions, covers = SUITES.expected_contract(subject, harness)
                self.assertEqual((len(assertions), len(covers)), sizes)
                self.assertIsInstance(assertions, frozenset)
                self.assertIsInstance(covers, frozenset)
        for subject, harness in (("clock", "coordinate_full_domain"), ("coordinate", "coordinate_control_zero"),
                                 ("unknown", "monotonic_full_domain"), ("clock", ""), (None, None)):
            with self.subTest(subject=subject, harness=harness), self.assertRaises(ValueError):
                SUITES.expected_contract(subject, harness)

    def test_cargo_constructor_contract_is_fixed_without_a_standalone_selector(self):
        projection = load("additional_era_constructor_contract", HERE / "era_constructor_projection.py")
        name = "proof::era_constructor_full_domain"
        self.assertEqual(SUITES.expected_contract("era_constructor", name),
                         (frozenset(projection.REQUIRED_ASSERTIONS[name]), frozenset(projection.REQUIRED_COVERS[name])))
        self.assertEqual(tuple(map(len, SUITES.expected_contract("era_constructor", name))), (5, 9))
        for job in projection.schedule()[1:4]:
            with self.subTest(harness=job["harness"]), self.assertRaises(ValueError):
                SUITES.expected_contract("era_constructor", job["harness"])
        for selector in ("era-constructor", "era_constructor"):
            with self.subTest(selector=selector), self.assertRaises(ValueError):
                SUITES.jobs(selector)
            with self.subTest(selector=selector), self.assertRaises(ValueError):
                SUITES.input_names(selector)

    def test_expected_names_match_actual_harnesses_and_projection_metadata(self):
        for selector in ("coordinate-exchange", "clock", "leap-calendar", "relative-range", "exchange-composition", "calendar-composition", "leap-ticks", "calendar-finite-laws", "leap-coverage"):
            harness_file = next(name for name in SUITES.input_names(selector) if name.endswith("_harness.rs"))
            text = (ROOT / harness_file).read_text()
            actual_names = set(re.findall(r"#\[kani::proof\]\s+fn (\w+)\(", text))
            self.assertEqual({j["harness"] for j in SUITES.jobs(selector)}, actual_names)
            for job in SUITES.jobs(selector):
                if job["case"] == "healthy":
                    assertions, covers = SUITES.expected_contract(job["subject"], job["harness"])
                    for name in assertions | covers:
                        self.assertIn(f'"{name}"', text)
                else:
                    self.assertIn(f'"{job["intended_failure"]}"', text)
        clock = load("additional_clock_contract_test", HERE / "clock_projection.py")
        for name, contract in clock.harness_contracts().items():
            if "successful_assertions" in contract:
                self.assertEqual(SUITES.expected_contract("clock", name),
                                 (frozenset(contract["successful_assertions"]), frozenset(contract["covers"])))
        calendar = load("additional_calendar_contract_test", HERE / "leap_calendar_projection.py")
        for name, assertions in calendar.REQUIRED_ASSERTIONS.items():
            self.assertEqual(SUITES.expected_contract("leap_calendar", name),
                             (frozenset(assertions), frozenset(calendar.REQUIRED_COVERS[name])))
        calendar_composition = load("additional_calendar_composition_contract_test", HERE / "calendar_composition_projection.py")
        for name, assertions in calendar_composition.REQUIRED_ASSERTIONS.items():
            self.assertEqual(SUITES.expected_contract("calendar_composition", name),
                             (frozenset(assertions), frozenset(calendar_composition.REQUIRED_COVERS[name])))
        leap_ticks = load("additional_leap_ticks_contract_test", HERE / "leap_ticks_projection.py")
        for name, assertions in leap_ticks.REQUIRED_ASSERTIONS.items():
            self.assertEqual(SUITES.expected_contract("leap_ticks", name),
                             (frozenset(assertions), frozenset(leap_ticks.REQUIRED_COVERS[name])))
        finite_laws = load("additional_finite_laws_contract_test", HERE / "calendar_finite_laws_projection.py")
        for name, assertions in finite_laws.REQUIRED_ASSERTIONS.items():
            self.assertEqual(SUITES.expected_contract("calendar_finite_laws", name),
                             (frozenset(assertions), frozenset(finite_laws.REQUIRED_COVERS[name])))

    def source_fixture(self, source, selector):
        names = [*SUITES.input_names(selector), "crates/time-domain/src/lib.rs",
                     "crates/time-domain/src/coordinate.rs", "crates/time-domain/src/exchange.rs",
                     "crates/time-domain/src/rounding.rs", "crates/time-domain/src/clock.rs",
                     "crates/time-domain/src/leap.rs",
                     "crates/time-domain/src/leap/validation.rs"]
        if selector == "leap-coverage":
            names.extend(("crates/time-domain/src/era.rs",
                          "crates/time-domain/src/leap/interpretation.rs"))
        for name in names:
            destination = source / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes((ROOT / name).read_bytes())

    def test_all_fixed_jobs_prepare_using_only_the_frozen_projection_paths(self):
        for selector in ("coordinate-exchange", "clock", "leap-calendar", "relative-range", "exchange-composition", "calendar-composition", "leap-ticks", "calendar-finite-laws", "leap-coverage"):
            with self.subTest(selector=selector), tempfile.TemporaryDirectory() as directory:
                folder = Path(directory)
                source = folder / "source"
                self.source_fixture(source, selector)
                loaded = []

                def frozen_loader(name, path):
                    loaded.append(path)
                    self.assertTrue(path.is_relative_to(source))
                    return load(name, path)

                for index, job in enumerate(SUITES.jobs(selector), 1):
                    inputs = folder / f"inputs-{index}"
                    metadata = SUITES.prepare(frozen_loader, source, inputs, job, index)
                    self.assertEqual(metadata["entrypoint"], inputs / "proof.rs")
                    self.assertTrue((inputs / "proof.rs").is_file())
                    self.assertEqual(metadata["mutation"] is None, job["case"] == "healthy")
                self.assertEqual(len(loaded), len(SUITES.jobs(selector)))

    def test_modified_jobs_and_positions_are_rejected_before_loading_or_writing(self):
        healthy = SUITES.jobs("clock")[0]
        altered = []
        for field, value in (("harness", "monotonic_control_zero"), ("intended_failure", ""),
                             ("restored_healthy", 0), ("case", "monotonic_always_error"),
                             ("subject", "unknown")):
            job = copy.deepcopy(healthy)
            job[field] = value
            altered.append((job, 1))
        altered.extend([(healthy, 0), (healthy, True), (healthy, 2), (healthy, 13), ({}, 1)])
        with tempfile.TemporaryDirectory() as directory:
            folder = Path(directory)

            def forbidden_loader(name, path):
                self.fail("Invalid job reached projection loading")

            for job, index in altered:
                with self.subTest(job=job, index=index), self.assertRaises(ValueError):
                    SUITES.prepare(forbidden_loader, folder / "source", folder / "inputs", job, index)
            self.assertFalse((folder / "inputs").exists())


if __name__ == "__main__":
    unittest.main()
