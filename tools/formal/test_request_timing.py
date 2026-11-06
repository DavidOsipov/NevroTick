"""Exercise exact timing-proof preparation; never execute Rust or a solver."""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import re
import tempfile
import unittest
from unittest.mock import patch


HERE = Path(__file__).resolve().parent
ROOT = HERE.parent.parent


def load(name, path):
    """Load only the currently selected local Python source for these controls."""
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError("Local projection is unavailable")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


PROJECTION = load("request_timing_projection_test", HERE / "request_timing_projection.py")
SUITES = load("request_timing_registry_test", HERE / "additional_suites.py")


class RequestTimingTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-request-timing-test-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        for name in PROJECTION.SOURCE_SHA256:
            target = self.source / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / name).read_bytes())
        self.inputs = self.base / "inputs"
        self.harness = HERE / "request_timing_harness.rs"

    def prepare(self, case="healthy", inputs=None):
        return PROJECTION.prepare_request_timing(
            self.source, inputs or self.inputs, case, self.harness)

    def test_complete_actual_clock_and_each_selected_span_are_exact(self):
        result = self.prepare()
        definitions = (self.inputs / "request_timing_selected.rs").read_bytes()
        proof = (self.inputs / "proof.rs").read_bytes()
        self.assertEqual((self.inputs / "clock.rs").read_bytes(),
                         (ROOT / "crates/time-domain/src/clock.rs").read_bytes())
        for name, span in PROJECTION.SPANS.items():
            original = (ROOT / span["source"]).read_bytes()
            selected = original[slice(*span["offsets"])]
            self.assertEqual(PROJECTION.digest(selected), span["sha256"], name)
            self.assertEqual(original.count(selected), 1, name)
            destination = proof if name == "domain_error" else definitions
            self.assertEqual(destination.count(selected), 1, name)
        self.assertIsNone(result["mutation"])
        self.assertEqual(result["assumptions"], [])
        self.assertEqual(result["original_sources_sha256"], PROJECTION.SOURCE_SHA256)

    def test_correspondence_records_all_six_exact_source_selections(self):
        result = self.prepare()
        self.assertEqual(len(result["correspondence"]), 6)
        for span in result["correspondence"]:
            original = (ROOT / span["source"]).read_bytes()
            selected = original[slice(*span["byte_offsets_half_open"])]
            self.assertEqual(PROJECTION.digest(selected), span["sha256"])
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**result, "entrypoint": "proof.rs"})

    def test_harness_is_exact_and_inside_real_type_module(self):
        self.prepare()
        proof = (self.inputs / "proof.rs").read_bytes()
        self.assertEqual(proof.count(self.harness.read_bytes()), 1)
        self.assertIn(b'mod request_timing {\nuse crate::clock::MonotonicNs;\n', proof)
        self.assertIn(b'include!("request_timing_selected.rs");', proof)
        self.assertNotIn(b"pub struct MonotonicNs", proof)
        self.assertNotIn(b"kani::assume", proof)

    def test_three_controls_change_only_the_named_complete_impl(self):
        healthy = self.prepare()
        original = (self.inputs / "request_timing_selected.rs").read_bytes()
        original_clock = (self.inputs / "clock.rs").read_bytes()
        for case, control in PROJECTION.CONTROLS.items():
            with self.subTest(case=case):
                folder = self.base / case
                result = self.prepare(case, folder)
                before, after = control["before"].encode(), control["after"].encode()
                self.assertEqual(original.count(before), 1)
                self.assertEqual((folder / "request_timing_selected.rs").read_bytes(),
                                 original.replace(before, after, 1))
                self.assertEqual((folder / "clock.rs").read_bytes(), original_clock)
                self.assertEqual((folder / "proof.rs").read_bytes(),
                                 (self.inputs / "proof.rs").read_bytes())
                self.assertEqual(result["mutation"], control)
                self.assertNotEqual(result["compiled_definitions_sha256"],
                                    healthy["compiled_definitions_sha256"])

    def test_restored_healthy_is_byte_identical(self):
        self.prepare()
        restored = self.base / "restored"
        self.prepare(inputs=restored)
        for filename in ("proof.rs", "clock.rs", "request_timing_selected.rs", "correspondence.json"):
            self.assertEqual((self.inputs / filename).read_bytes(), (restored / filename).read_bytes())

    def test_every_complete_input_drift_fails_before_output_creation(self):
        for name in PROJECTION.SOURCE_SHA256:
            with self.subTest(name=name):
                path = self.source / name
                original = path.read_bytes()
                path.write_bytes(original + b"\n")
                with self.assertRaisesRegex(ValueError, "complete source changed"):
                    self.prepare()
                self.assertFalse(self.inputs.exists())
                path.write_bytes(original)

    def test_span_drift_is_rejected_even_when_whole_file_pin_is_rebound(self):
        span = PROJECTION.SPANS["implementation"]
        path = self.source / span["source"]
        source = path.read_bytes().replace(b"if lifetime == 0", b"if lifetime != 0", 1)
        path.write_bytes(source)
        with patch.dict(PROJECTION.SOURCE_SHA256, {span["source"]: PROJECTION.digest(source)}):
            with self.assertRaisesRegex(ValueError, "source span changed"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_ambiguous_anchor_is_rejected_even_with_rebound_complete_pin(self):
        span = PROJECTION.SPANS["implementation"]
        path = self.source / span["source"]
        source = path.read_bytes() + b"\n" + span["start"]
        path.write_bytes(source)
        with patch.dict(PROJECTION.SOURCE_SHA256, {span["source"]: PROJECTION.digest(source)}):
            with self.assertRaisesRegex(ValueError, "ambiguous"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_unknown_case_and_output_collision_fail_without_replacement(self):
        with self.assertRaisesRegex(ValueError, "Unknown request-timing control"):
            self.prepare("other")
        self.assertFalse(self.inputs.exists())
        self.inputs.mkdir()
        target = self.inputs / "proof.rs"
        target.write_bytes(b"owned original\n")
        with self.assertRaisesRegex(ValueError, "already exists"):
            self.prepare()
        self.assertEqual(target.read_bytes(), b"owned original\n")
        self.assertEqual(list(self.inputs.iterdir()), [target])

    def test_independent_full_domain_and_eight_cover_inventory(self):
        source = self.harness.read_text()
        self.assertEqual(source.count("kani::any::<u64>()"), 2)
        self.assertNotIn("kani::assume", source)
        self.assertNotIn("FIXTURE_MAX_LIFETIME_NS", source)
        self.assertNotIn("MonotonicNs {", source)
        self.assertIn("i128::from(deadline) - i128::from(send)", source)
        self.assertIn("1_000_000_000_i128", source)
        harnesses = set(re.findall(r"#\[kani::proof\]\s+fn (\w+)\(", source))
        self.assertEqual(harnesses, {job["harness"].split("::")[-1]
                                    for job in PROJECTION.schedule()})
        assertions = PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS]
        covers = PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS]
        self.assertEqual(len(assertions), 8)
        self.assertEqual(len(covers), 8)
        for label in (*assertions, *covers):
            self.assertIn('"' + label + '"', source)

    def test_registry_schedule_and_full_contract_are_independently_fixed(self):
        jobs = SUITES.jobs("request-timing")
        self.assertEqual(jobs, PROJECTION.schedule())
        self.assertEqual([job["case"] for job in jobs],
                         ["healthy", "zero_lifetime", "excess_lifetime", "retained_send", "healthy"])
        self.assertEqual([job["restored_healthy"] for job in jobs], [False] * 4 + [True])
        self.assertEqual(SUITES.expected_contract("request_timing", PROJECTION.FULL_HARNESS),
                         (frozenset(PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS]),
                          frozenset(PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS])))
        for job in jobs[1:4]:
            with self.assertRaises(ValueError):
                SUITES.expected_contract(job["subject"], job["harness"])
        jobs[0]["case"] = "caller modification"
        self.assertEqual(SUITES.jobs("request-timing")[0]["case"], "healthy")

    def test_registry_freezes_complete_source_and_projection_closure(self):
        names = SUITES.input_names("request-timing")
        self.assertEqual(len(names), len(set(names)))
        self.assertTrue(set(PROJECTION.SOURCE_SHA256).issubset(names))
        self.assertTrue({"tools/formal/request_timing_projection.py",
                         "tools/formal/request_timing_harness.rs",
                         "tools/formal/test_request_timing.py",
                         "tools/formal/additional_suites.py"}.issubset(names))

    def test_registry_rejects_partial_overridden_and_reordered_jobs_before_loading(self):
        jobs = SUITES.jobs("request-timing")
        invalid = [(jobs[0], 0), (jobs[0], True), (jobs[0], 2), (jobs[0], 6)]
        for field, value in (("case", "zero_lifetime"), ("subject", "clock"),
                             ("harness", "other"), ("restored_healthy", True),
                             ("intended_failure", "other"), ("extra", True)):
            invalid.append((dict(jobs[0], **{field: value}), 1))
        def forbidden_loader(name, path):
            self.fail("An altered job reached source loading")
        for job, index in invalid:
            with self.subTest(job=job, index=index), self.assertRaises(ValueError):
                SUITES.prepare(forbidden_loader, self.source, self.inputs, job, index)
        self.assertFalse(self.inputs.exists())

    def test_registry_uses_only_supplied_current_python_loader(self):
        calls = []
        def controlled_loader(name, path):
            calls.append((name, path))
            self.assertEqual(path, self.source / "tools/formal/request_timing_projection.py")
            return PROJECTION
        harness_target = self.source / "tools/formal/request_timing_harness.rs"
        harness_target.parent.mkdir(parents=True)
        harness_target.write_bytes(self.harness.read_bytes())
        job = SUITES.jobs("request-timing")[0]
        with patch.object(SUITES.importlib.util, "spec_from_file_location",
                          side_effect=AssertionError("Unexpected direct registry loader")):
            result = SUITES.prepare(controlled_loader, self.source, self.inputs, job, 1)
        self.assertEqual(len(calls), 1)
        self.assertEqual(result["entrypoint"], self.inputs / "proof.rs")

    def test_preparation_clock_is_exact_nonresetting_and_uses_worse_elapsed(self):
        self.assertEqual(PROJECTION.PREPARATION_STARTED_UNIX_NS, 1793939040629208775)
        self.assertEqual(PROJECTION.PREPARATION_STARTED_MONOTONIC_NS, 35708462632598)
        self.assertEqual(PROJECTION.PREPARATION_DEADLINE_MONOTONIC_NS, 39308462632598)
        self.assertEqual(PROJECTION.TOTAL_SECONDS, 3600)
        with patch.object(Path, "read_text", return_value=PROJECTION.PREPARATION_BOOT_ID + "\n"), \
             patch.object(PROJECTION.time, "monotonic_ns",
                          return_value=PROJECTION.PREPARATION_STARTED_MONOTONIC_NS + 700_000_000_000), \
             patch.object(PROJECTION.time, "time_ns",
                          return_value=PROJECTION.PREPARATION_STARTED_UNIX_NS + 800_000_000_000):
            self.assertEqual(PROJECTION.remaining_budget(), 2800)

    def test_wrong_boot_expiry_and_backward_clocks_are_rejected(self):
        cases = (("foreign boot", 1, 1), (PROJECTION.PREPARATION_BOOT_ID, 3600, 1),
                 (PROJECTION.PREPARATION_BOOT_ID, 1, 3600),
                 (PROJECTION.PREPARATION_BOOT_ID, -1, 1),
                 (PROJECTION.PREPARATION_BOOT_ID, 1, -1))
        for boot, mono_delta, wall_delta in cases:
            with self.subTest(boot=boot, mono=mono_delta, wall=wall_delta), \
                 patch.object(Path, "read_text", return_value=boot), \
                 patch.object(PROJECTION.time, "monotonic_ns", return_value=
                              PROJECTION.PREPARATION_STARTED_MONOTONIC_NS + mono_delta * 1_000_000_000), \
                 patch.object(PROJECTION.time, "time_ns", return_value=
                              PROJECTION.PREPARATION_STARTED_UNIX_NS + wall_delta * 1_000_000_000):
                with self.assertRaises(ValueError):
                    PROJECTION.remaining_budget()


if __name__ == "__main__":
    unittest.main()
