"""Exercise exact supporter-label preparation without executing Rust or a solver."""
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


PROJECTION = load("supporter_labels_projection_test", HERE / "supporter_labels_projection.py")
SUITES = load("supporter_labels_registry_test", HERE / "additional_suites.py")


class SupporterLabelsTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-supporter-labels-test-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        for name in PROJECTION.SOURCE_SHA256:
            target = self.source / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / name).read_bytes())
        self.inputs = self.base / "inputs"
        self.harness = HERE / "supporter_labels_harness.rs"

    def prepare(self, case="healthy", inputs=None):
        return PROJECTION.prepare_supporter_labels(
            self.source, inputs or self.inputs, case, self.harness)

    def test_complete_five_actual_spans_and_standard_nonzero_are_exact(self):
        result = self.prepare()
        definitions = (self.inputs / "supporter_labels_selected.rs").read_bytes()
        owner = (self.inputs / "supporter_labels_owner.rs").read_bytes()
        source = (ROOT / "crates/time-core/src/selection.rs").read_bytes()
        for name, span in PROJECTION.SPANS.items():
            selected = source[slice(*span["offsets"])]
            self.assertEqual(PROJECTION.digest(selected), span["sha256"], name)
            self.assertEqual((definitions + owner).count(selected), 1, name)
        self.assertIn(b"use std::num::NonZeroU64;", definitions)
        self.assertIn(b"fn sort_prefix<T: Copy + Ord>", definitions)
        self.assertEqual(result["selected_original_bytes"], 2958)
        self.assertIsNone(result["mutation"])
        self.assertEqual(result["assumptions"], [])
        self.assertEqual(result["original_sources_sha256"], PROJECTION.SOURCE_SHA256)
        self.assertEqual(result["effective_harness_unwind"], 17)

    def test_correspondence_domain_and_every_generated_hash_are_exact(self):
        result = self.prepare()
        self.assertEqual(len(result["correspondence"]), 5)
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**result, "entrypoint": "proof.rs"})
        self.assertEqual(result["domain"]["primitive_any_calls"], 19)
        self.assertEqual(result["domain"]["assumed_input_invariants"], [])
        for name, expected in result["compiled_files_sha256"].items():
            self.assertEqual(PROJECTION.digest((self.inputs / name).read_bytes()), expected)

    def test_harness_is_exact_inside_actual_type_module(self):
        self.prepare()
        proof = (self.inputs / "proof.rs").read_bytes()
        self.assertEqual(proof.count(self.harness.read_bytes()), 1)
        self.assertIn(b'mod supporter_labels {', proof)
        self.assertIn(b'include!("supporter_labels_selected.rs");', proof)
        self.assertIn(b'include!("supporter_labels_owner.rs");', proof)
        self.assertNotIn(b"enum SelectionError", proof)
        self.assertNotIn(b"kani::assume", proof)

    def test_three_controls_change_only_unique_actual_caller_bytes(self):
        healthy = self.prepare()
        original = (self.inputs / "supporter_labels_owner.rs").read_bytes()
        for case, control in PROJECTION.CONTROLS.items():
            with self.subTest(case=case):
                folder = self.base / case
                result = self.prepare(case, folder)
                before, after = control["before"].encode(), control["after"].encode()
                self.assertEqual(original.count(before), 1)
                self.assertEqual((folder / "supporter_labels_owner.rs").read_bytes(),
                                 original.replace(before, after, 1))
                for name in ("proof.rs", "supporter_labels_selected.rs"):
                    self.assertEqual((folder / name).read_bytes(), (self.inputs / name).read_bytes())
                self.assertEqual(result["mutation"], control)
                self.assertNotEqual(result["compiled_implementation_sha256"],
                                    healthy["compiled_implementation_sha256"])

    def test_restored_healthy_has_identical_all_four_generated_files(self):
        self.prepare()
        restored = self.base / "restored"
        self.prepare(inputs=restored)
        names = {"proof.rs", "supporter_labels_selected.rs", "supporter_labels_owner.rs", "correspondence.json"}
        self.assertEqual(names, {path.name for path in self.inputs.iterdir()})
        self.assertEqual(names, {path.name for path in restored.iterdir()})
        for name in names:
            self.assertEqual((self.inputs / name).read_bytes(), (restored / name).read_bytes())

    def test_each_complete_source_drift_fails_before_output_creation(self):
        for name in PROJECTION.SOURCE_SHA256:
            with self.subTest(name=name):
                path = self.source / name
                original = path.read_bytes()
                path.write_bytes(original + b"\n")
                with self.assertRaisesRegex(ValueError, "complete source changed"):
                    self.prepare()
                self.assertFalse(self.inputs.exists())
                path.write_bytes(original)

    def test_span_drift_rejected_even_with_rebound_complete_file_pin(self):
        path = self.source / "crates/time-core/src/selection.rs"
        source = path.read_bytes().replace(b"sort_prefix(prefix)?;", b"sort_prefix(prefix)?; ", 1)
        path.write_bytes(source)
        with patch.dict(PROJECTION.SOURCE_SHA256, {"crates/time-core/src/selection.rs": PROJECTION.digest(source)}):
            with self.assertRaisesRegex(ValueError, "source span changed"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_ambiguous_anchor_rejected_even_with_rebound_complete_file_pin(self):
        path = self.source / "crates/time-core/src/selection.rs"
        source = path.read_bytes() + b"\n" + PROJECTION.SPANS["implementation"]["start"]
        path.write_bytes(source)
        with patch.dict(PROJECTION.SOURCE_SHA256, {"crates/time-core/src/selection.rs": PROJECTION.digest(source)}):
            with self.assertRaisesRegex(ValueError, "ambiguous"):
                self.prepare()
        self.assertFalse(self.inputs.exists())

    def test_unknown_case_and_all_output_collisions_fail_without_replacement(self):
        for case in ("other", None, True, 1):
            with self.subTest(case=case), self.assertRaisesRegex(ValueError, "Unknown supporter-labels control"):
                self.prepare(case)
        self.assertFalse(self.inputs.exists())
        self.inputs.mkdir()
        for filename in ("proof.rs", "supporter_labels_selected.rs", "supporter_labels_owner.rs", "correspondence.json"):
            target = self.inputs / filename
            target.write_bytes(b"owned original\n")
            with self.assertRaisesRegex(ValueError, "already exists"):
                self.prepare()
            self.assertEqual(target.read_bytes(), b"owned original\n")
            target.unlink()
        target = self.inputs / "proof.rs"
        target.symlink_to("nonexistent-original")
        with self.assertRaisesRegex(ValueError, "already exists"):
            self.prepare()
        self.assertTrue(target.is_symlink())

    def test_raw_domain_is_nineteen_independent_calls_and_sixteen_nonzero_maps(self):
        source = self.harness.read_text()
        healthy = source.split("fn supporter_labels_full_domain() {", 1)[1].split("#[kani::proof]", 1)[0]
        raw = healthy.split("let raw = [", 1)[1].split("];", 1)[0]
        self.assertEqual(re.sub(r"\s+", "", raw), "kani::any::<u64>()," * 16)
        self.assertEqual(healthy.count("kani::any::<"), 19)
        for index in range(16):
            self.assertEqual(healthy.count(f"NonZeroU64::new(raw[{index}])"), 1)
        for forbidden in ("kani::assume", "from_fn", "kani::any::<[", "0..count", "1..count", "while ", "filter("):
            self.assertNotIn(forbidden, source)
        self.assertEqual(source.count("#[kani::proof]"), 4)
        self.assertEqual(source.count("#[kani::unwind(17)]"), 4)
        self.assertEqual(re.findall(r"for index in ([^ ]+) ", source), ["0..16"] * 3)
        self.assertIn("if probe_index < 15 && probe_index + 1 < count", source)
        self.assertIn("if probe_index < 16 && probe_index >= count", source)
        self.assertIn('assert!(complete_frame, "supporter complete input frame")', source)
        self.assertIn("const _: () = assert!(usize::BITS == 64);", source)
        self.assertNotIn("assert_eq!", source)
        self.assertNotIn("supporter target width", source)
        self.assertEqual(len(PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS]), 6)

    def test_literal_contract_matches_healthy_assertions_and_covers(self):
        source = self.harness.read_text().split("fn supporter_labels_full_domain() {", 1)[1].split("#[kani::proof]", 1)[0]
        labels = re.findall(r'"(supporter [^"\n]+)"', source)
        expected = (*PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS],
                    *PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS])
        self.assertCountEqual(labels, expected)
        self.assertEqual(len(labels), len(set(labels)))
        for control in PROJECTION.CONTROLS.values():
            body = self.harness.read_text().split("fn " + control["harness"].split("::")[-1] + "() {", 1)[1]
            body = body.split("#[kani::proof]", 1)[0]
            self.assertEqual(re.findall(r'"(supporter [^"\n]+)"', body), [control["intended_failure"]])

    def test_registry_schedule_and_healthy_contract_are_exact(self):
        jobs = SUITES.jobs("supporter-labels")
        self.assertEqual(jobs, PROJECTION.schedule())
        self.assertEqual(len(jobs), 5)
        self.assertEqual([job["case"] for job in jobs], ["healthy", "empty_sort", "multiplicity", "wrong_tail", "healthy"])
        self.assertEqual(SUITES.expected_contract("supporter_labels", PROJECTION.FULL_HARNESS),
                         (frozenset(PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS]),
                          frozenset(PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS])))
        for job in jobs[1:4]:
            with self.assertRaises(ValueError):
                SUITES.expected_contract(job["subject"], job["harness"])
        jobs[0]["case"] = "caller modification"
        self.assertEqual(SUITES.jobs("supporter-labels")[0]["case"], "healthy")
        self.assertEqual(SUITES.MAX_JOBS, 12)

    def test_registry_freezes_complete_source_and_projection_closure(self):
        names = SUITES.input_names("supporter-labels")
        self.assertEqual(len(names), len(set(names)))
        self.assertTrue(set(PROJECTION.SOURCE_SHA256).issubset(names))
        self.assertTrue({"tools/formal/supporter_labels_projection.py", "tools/formal/supporter_labels_harness.rs",
                         "tools/formal/test_supporter_labels.py", "tools/formal/additional_suites.py",
                         "tools/formal/verify_supporter_labels.py", "tools/formal/test_verify_supporter_labels.py"}.issubset(names))
        self.assertNotIn("tools/formal/request_timing_projection.py", names)

    def test_registry_rejects_partial_overridden_reordered_jobs_before_loading(self):
        jobs = SUITES.jobs("supporter-labels")
        invalid = [(jobs[0], 0), (jobs[0], True), (jobs[0], 2), (jobs[0], 7)]
        for field, value in (("case", "empty_sort"), ("subject", "request_timing"),
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
            self.assertEqual(path, self.source / "tools/formal/supporter_labels_projection.py")
            return PROJECTION
        harness_target = self.source / "tools/formal/supporter_labels_harness.rs"
        harness_target.parent.mkdir(parents=True)
        harness_target.write_bytes(self.harness.read_bytes())
        job = SUITES.jobs("supporter-labels")[0]
        with patch.object(SUITES.importlib.util, "spec_from_file_location",
                          side_effect=AssertionError("Unexpected direct registry loader")):
            result = SUITES.prepare(controlled_loader, self.source, self.inputs, job, 1)
        self.assertEqual(len(calls), 1)
        self.assertEqual(result["entrypoint"], self.inputs / "proof.rs")

    def test_preparation_clock_is_exact_nonresetting_and_uses_worse_elapsed(self):
        self.assertEqual(PROJECTION.PREPARATION_STARTED_UNIX_NS, 1793949009810488193)
        self.assertEqual(PROJECTION.PREPARATION_STARTED_MONOTONIC_NS, 45677643916875)
        self.assertEqual(PROJECTION.PREPARATION_DEADLINE_MONOTONIC_NS, 49277643916875)
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
