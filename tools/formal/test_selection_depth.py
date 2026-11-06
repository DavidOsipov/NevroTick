"""Exercise exact append-depth preparation without executing Rust or a solver."""
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


PROJECTION = load("selection_depth_projection_test", HERE / "selection_depth_projection.py")
SUITES = load("selection_depth_registry_test", HERE / "additional_suites.py")


class SelectionDepthTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-selection-depth-test-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        for name in PROJECTION.SOURCE_SHA256:
            target = self.source / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / name).read_bytes())
        self.inputs = self.base / "inputs"
        self.harness = HERE / "selection_depth_harness.rs"

    def prepare(self, case="healthy", inputs=None):
        return PROJECTION.prepare_selection_depth(
            self.source, inputs or self.inputs, case, self.harness)

    def test_complete_selected_actual_types_and_body_are_exact(self):
        result = self.prepare()
        definitions = (self.inputs / "selection_depth_selected.rs").read_bytes()
        source = (ROOT / "crates/time-core/src/selection.rs").read_bytes()
        for name, span in PROJECTION.SPANS.items():
            selected = source[slice(*span["offsets"])]
            self.assertEqual(PROJECTION.digest(selected), span["sha256"], name)
            self.assertEqual(source.count(selected), 1, name)
            self.assertEqual(definitions.count(selected), 1, name)
        self.assertEqual(definitions.count(b"#[derive(Clone, Copy, Debug, Eq, PartialEq)]"), 2)
        self.assertIn(b"values: [usize; 63]", definitions)
        self.assertEqual(result["selected_original_bytes"], 1689)
        self.assertIsNone(result["mutation"])
        self.assertEqual(result["assumptions"], [])
        self.assertEqual(result["original_sources_sha256"], PROJECTION.SOURCE_SHA256)

    def test_correspondence_and_machine_domain_are_complete_and_exact(self):
        result = self.prepare()
        self.assertEqual(len(result["correspondence"]), 4)
        for span in result["correspondence"]:
            original = (ROOT / span["source"]).read_bytes()
            selected = original[slice(*span["byte_offsets_half_open"])]
            self.assertEqual(PROJECTION.digest(selected), span["sha256"])
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**result, "entrypoint": "proof.rs"})
        self.assertEqual(result["machine_widths"], {"usize": 64, "length_specification": 128})
        self.assertEqual(result["domain"], {"independent_array_values": 63,
                         "independent_scalar_values": 4, "raw_value_type": "usize",
                         "assumed_prefix_invariants": []})

    def test_harness_is_exact_inside_actual_type_module(self):
        self.prepare()
        proof = (self.inputs / "proof.rs").read_bytes()
        self.assertEqual(proof.count(self.harness.read_bytes()), 1)
        self.assertIn(b"mod selection_depth {\n", proof)
        self.assertIn(b'include!("selection_depth_selected.rs");', proof)
        self.assertNotIn(b"enum SelectionError", proof)
        self.assertNotIn(b"struct CellDepths", proof)
        self.assertNotIn(b"kani::assume", proof)

    def test_four_controls_change_only_named_actual_helper_bytes(self):
        healthy = self.prepare()
        original = (self.inputs / "selection_depth_selected.rs").read_bytes()
        original_proof = (self.inputs / "proof.rs").read_bytes()
        for case, control in PROJECTION.CONTROLS.items():
            with self.subTest(case=case):
                folder = self.base / case
                result = self.prepare(case, folder)
                before, after = control["before"].encode(), control["after"].encode()
                self.assertEqual(original.count(before), 1)
                self.assertEqual((folder / "selection_depth_selected.rs").read_bytes(),
                                 original.replace(before, after, 1))
                self.assertEqual((folder / "proof.rs").read_bytes(), original_proof)
                self.assertEqual(result["mutation"], control)
                self.assertNotEqual(result["compiled_definitions_sha256"],
                                    healthy["compiled_definitions_sha256"])
                source = (ROOT / "crates/time-core/src/selection.rs").read_bytes()
                first, last = PROJECTION.SPANS["implementation"]["offsets"]
                self.assertEqual(result["compiled_implementation_sha256"],
                                 PROJECTION.digest(source[first:last].replace(before, after, 1)))

    def test_restored_healthy_has_identical_complete_generated_tree(self):
        self.prepare()
        restored = self.base / "restored"
        self.prepare(inputs=restored)
        original_names = {path.name for path in self.inputs.iterdir()}
        self.assertEqual(original_names, {"proof.rs", "selection_depth_selected.rs", "correspondence.json"})
        self.assertEqual(original_names, {path.name for path in restored.iterdir()})
        for name in original_names:
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
        source = path.read_bytes().replace(b"cells.maximum.max(depth)", b"cells.maximum.min(depth)", 1)
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
            with self.subTest(case=case), self.assertRaisesRegex(ValueError, "Unknown selection-depth control"):
                self.prepare(case)
        self.assertFalse(self.inputs.exists())
        self.inputs.mkdir()
        for filename in ("proof.rs", "selection_depth_selected.rs", "correspondence.json"):
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

    def test_raw_domain_is_sixty_three_distinct_primitive_calls_plus_four_scalars(self):
        source = self.harness.read_text()
        healthy = source.split("fn selection_depth_full_domain() {", 1)[1].split("#[kani::proof]", 1)[0]
        array_literal = healthy.split("let values = [", 1)[1].split("];", 1)[0]
        self.assertEqual(array_literal.count("kani::any::<usize>()"), 63)
        self.assertEqual(healthy.count("kani::any::<usize>()"), 67)
        self.assertEqual(re.sub(r"\s+", "", array_literal), "kani::any::<usize>()," * 63)
        for name in ("len", "maximum", "depth", "probe"):
            self.assertIn("let " + name + " = kani::any::<usize>();", healthy)
        for forbidden in ("kani::assume", "from_fn", "kani::any::<[", "#[kani::unwind", "for ", "while "):
            self.assertNotIn(forbidden, healthy)
        self.assertIn("const _: () = assert!(usize::BITS == 64);", source)
        self.assertIn("(len as u128) + 1", source)
        self.assertIn("if depth > maximum { depth } else { maximum }", source)
        self.assertIn("if probe < 63 && probe != len", source)
        self.assertIn("if probe < 63", source)
        self.assertNotIn("checked_add", source)

    def test_independent_nine_assertions_ten_covers_and_five_harnesses(self):
        source = self.harness.read_text()
        harnesses = set(re.findall(r"#\[kani::proof\]\s+fn (\w+)\(", source))
        self.assertEqual(harnesses, {job["harness"].split("::")[-1] for job in PROJECTION.schedule()})
        assertions = PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS]
        covers = PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS]
        self.assertEqual(len(set(assertions)), 9)
        self.assertEqual(len(set(covers)), 10)
        self.assertEqual(source.count("kani::cover!("), 10)
        contract = source.split("#[kani::proof]", 1)[0]
        self.assertEqual(contract.count("assert_eq!("), 9)
        for label in assertions:
            self.assertEqual(contract.count('"' + label + '"'), 1)
        for label in covers:
            self.assertEqual(source.count('"' + label + '"'), 1)
        for case, control in PROJECTION.CONTROLS.items():
            witness = source.split("fn selection_depth_control_" + case + "() {", 1)[1].split("#[kani::proof]", 1)[0]
            self.assertEqual(witness.count("assert_eq!("), 1)
            self.assertIn('"' + control["intended_failure"] + '"', witness)
            self.assertIn("values: [7; 63]", witness)
        self.assertIn("probe == 0 && values[0] != depth", source)
        self.assertIn("probe == 62 && values[62] != depth", source)

    def test_registry_schedule_and_full_contract_are_independently_fixed(self):
        jobs = SUITES.jobs("selection-depth")
        self.assertEqual(jobs, PROJECTION.schedule())
        self.assertEqual([job["case"] for job in jobs],
                         ["healthy", "wrong_slot", "wrong_length", "wrong_maximum", "wrong_error", "healthy"])
        self.assertEqual([job["restored_healthy"] for job in jobs], [False] * 5 + [True])
        self.assertEqual(SUITES.expected_contract("selection_depth", PROJECTION.FULL_HARNESS),
                         (frozenset(PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS]),
                          frozenset(PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS])))
        for job in jobs[1:5]:
            with self.assertRaises(ValueError):
                SUITES.expected_contract(job["subject"], job["harness"])
        jobs[0]["case"] = "caller modification"
        self.assertEqual(SUITES.jobs("selection-depth")[0]["case"], "healthy")
        self.assertEqual(len(SUITES.jobs("request-timing")), 5)
        self.assertEqual(SUITES.MAX_JOBS, 12)

    def test_registry_freezes_complete_source_and_projection_closure(self):
        names = SUITES.input_names("selection-depth")
        self.assertEqual(len(names), len(set(names)))
        self.assertTrue(set(PROJECTION.SOURCE_SHA256).issubset(names))
        self.assertTrue({"tools/formal/selection_depth_projection.py", "tools/formal/selection_depth_harness.rs",
                         "tools/formal/test_selection_depth.py", "tools/formal/additional_suites.py",
                         "tools/formal/verify_selection_depth.py", "tools/formal/test_verify_selection_depth.py"}.issubset(names))
        self.assertNotIn("tools/formal/request_timing_projection.py", names)

    def test_registry_rejects_partial_overridden_reordered_jobs_before_loading(self):
        jobs = SUITES.jobs("selection-depth")
        invalid = [(jobs[0], 0), (jobs[0], True), (jobs[0], 2), (jobs[0], 7)]
        for field, value in (("case", "wrong_slot"), ("subject", "request_timing"),
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
            self.assertEqual(path, self.source / "tools/formal/selection_depth_projection.py")
            return PROJECTION
        harness_target = self.source / "tools/formal/selection_depth_harness.rs"
        harness_target.parent.mkdir(parents=True)
        harness_target.write_bytes(self.harness.read_bytes())
        job = SUITES.jobs("selection-depth")[0]
        with patch.object(SUITES.importlib.util, "spec_from_file_location",
                          side_effect=AssertionError("Unexpected direct registry loader")):
            result = SUITES.prepare(controlled_loader, self.source, self.inputs, job, 1)
        self.assertEqual(len(calls), 1)
        self.assertEqual(result["entrypoint"], self.inputs / "proof.rs")

    def test_preparation_clock_is_exact_nonresetting_and_uses_worse_elapsed(self):
        self.assertEqual(PROJECTION.PREPARATION_STARTED_UNIX_NS, 1793940925449538908)
        self.assertEqual(PROJECTION.PREPARATION_STARTED_MONOTONIC_NS, 37593282968081)
        self.assertEqual(PROJECTION.PREPARATION_DEADLINE_MONOTONIC_NS, 41193282968081)
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
