"""Exercise exact ordinal source preparation without a compiler or solver."""
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
    """Load only the selected current local source for these Python controls."""
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError("Local projection is unavailable")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


PROJECTION = load("lifecycle_ordinal_projection_test", HERE / "lifecycle_ordinal_projection.py")
SUITES = load("lifecycle_ordinal_registry_test", HERE / "additional_suites.py")


class LifecycleOrdinalTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-lifecycle-ordinal-test-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        for name in PROJECTION.SOURCE_SHA256:
            target = self.source / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / name).read_bytes())
        self.inputs = self.base / "inputs"
        self.harness = HERE / "lifecycle_ordinal_harness.rs"

    def prepare(self, case="healthy", inputs=None):
        return PROJECTION.prepare_lifecycle_ordinal(
            self.source, inputs or self.inputs, case, self.harness)

    def test_all_six_exact_original_spans_and_twenty_three_enum_types(self):
        result = self.prepare()
        all_definitions = ((self.inputs / "lifecycle_ordinal_selected.rs").read_bytes()
                           + (self.inputs / "lifecycle_ordinal_owner.rs").read_bytes())
        expected = [
            ("combination/lifecycle.rs", [6261, 9644]),
            ("combination/lifecycle.rs", [22383, 22780]),
            ("principal.rs", [2988, 7799]), ("combination.rs", [4658, 10807]),
            ("regression.rs", [4900, 9375]), ("selection.rs", [2840, 3497]),
        ]
        self.assertEqual([(item["source"].removeprefix("crates/time-core/src/"),
                           item["byte_offsets_half_open"]) for item in result["correspondence"]], expected)
        for item in result["correspondence"]:
            raw = (ROOT / item["source"]).read_bytes()
            original = raw[slice(*item["byte_offsets_half_open"])]
            self.assertEqual(PROJECTION.digest(original), item["sha256"])
            self.assertEqual(all_definitions.count(original), 1)
        expected_types = {
            "FixtureCombinationParameterField", "FixtureCombinationParameterIssue",
            "FixtureCombinationLifecycleStep", "FixtureCombinationLifecycleError",
            "FixturePrincipalParameterField", "FixturePrincipalScalarIssue", "FixturePrincipalStep",
            "FixturePrincipalContextField", "FixturePrincipalError",
            "FixtureCombinationScalarField", "FixtureCombinationScalarIssue", "FixtureCombinationContextField",
            "FixtureCombinationDenominatorField", "FixtureCombinationStep", "FixtureCombinationError",
            "PointField", "PointIssue", "ArithmeticStage", "ArithmeticOperation", "ArithmeticIssue", "FitError",
            "SelectionInvariantStage", "SelectionError",
        }
        enum_names = re.findall(rb"pub enum (\w+)", all_definitions)
        self.assertEqual(len(enum_names), 23)
        self.assertEqual({name.decode() for name in enum_names}, expected_types)
        self.assertEqual(result["selected_original_bytes"], 19872)
        self.assertIsNone(result["mutation"])
        self.assertEqual(result["assumptions"], [])
        self.assertNotIn(b"impl ", all_definitions)
        self.assertIn(b"#[derive(PartialEq)]\npub enum FixtureCombinationLifecycleError", all_definitions)

    def test_actual_namespace_resolution_and_standard_nonzero_construction(self):
        self.prepare()
        definitions = (self.inputs / "lifecycle_ordinal_selected.rs").read_text()
        proof = (self.inputs / "proof.rs").read_text()
        owner = (self.inputs / "lifecycle_ordinal_owner.rs").read_text()
        for framing in ("mod regression {", "pub use regression::FitError;", "mod selection {",
                        "pub use selection::SelectionError;", "mod principal {\nuse crate::{FitError, SelectionError};",
                        "pub use principal::FixturePrincipalError;", "mod combination {\nuse crate::FitError;"):
            self.assertIn(framing, definitions)
        self.assertIn("mod lifecycle_ordinal {\nuse std::num::NonZeroU64;", proof)
        self.assertIn("use crate::{FitError, FixturePrincipalError};", proof)
        self.assertIn("use crate::combination::FixtureCombinationError;", proof)
        self.assertIn('.checked_add(1)\n        .and_then(NonZeroU64::new)', owner)
        self.assertEqual(proof.count(self.harness.read_text()), 1)
        self.assertNotIn("enum ", proof)
        self.assertNotIn("struct NonZero", definitions + owner + proof)
        self.assertNotIn("new_unchecked", definitions + owner + proof)

    def test_correspondence_hashes_machine_domain_and_all_output_files(self):
        result = self.prepare()
        self.assertEqual(json.loads((self.inputs / "correspondence.json").read_text()),
                         {**result, "entrypoint": "proof.rs"})
        for name, expected in result["compiled_files_sha256"].items():
            self.assertEqual(PROJECTION.digest((self.inputs / name).read_bytes()), expected)
        definitions = (self.inputs / "lifecycle_ordinal_selected.rs").read_bytes()
        owner = (self.inputs / "lifecycle_ordinal_owner.rs").read_bytes()
        self.assertEqual(result["compiled_definitions_sha256"], PROJECTION.digest(definitions + owner))
        self.assertEqual(result["machine_widths"], {"u64": 64, "successor_specification": 128})
        self.assertEqual(result["domain"], {"independent_scalar_values": 1, "raw_value_type": "u64",
                         "assumed_input_invariants": [], "actual_enum_declarations": 23})

    def test_three_controls_change_only_one_original_helper_point(self):
        healthy = self.prepare()
        owner = (self.inputs / "lifecycle_ordinal_owner.rs").read_bytes()
        for case, control in PROJECTION.CONTROLS.items():
            with self.subTest(case=case):
                folder = self.base / case
                result = self.prepare(case, folder)
                before, after = control["before"].encode(), control["after"].encode()
                self.assertEqual(owner.count(before), 1)
                self.assertEqual((folder / "lifecycle_ordinal_owner.rs").read_bytes(),
                                 owner.replace(before, after, 1))
                for name in ("proof.rs", "lifecycle_ordinal_selected.rs"):
                    self.assertEqual((folder / name).read_bytes(), (self.inputs / name).read_bytes())
                self.assertEqual(result["mutation"], control)
                self.assertNotEqual(result["compiled_implementation_sha256"],
                                    healthy["compiled_implementation_sha256"])
        self.assertEqual(PROJECTION.CONTROLS["always_error"]["after"], ".and_then(|_| None)")

    def test_exact_restoration_reproduces_the_complete_generated_tree(self):
        self.prepare()
        restored = self.base / "restored"
        self.prepare(inputs=restored)
        names = {path.name for path in self.inputs.iterdir()}
        self.assertEqual(names, {"proof.rs", "lifecycle_ordinal_selected.rs",
                                 "lifecycle_ordinal_owner.rs", "correspondence.json"})
        self.assertEqual(names, {path.name for path in restored.iterdir()})
        for name in names:
            self.assertEqual((self.inputs / name).read_bytes(), (restored / name).read_bytes())

    def test_complete_source_drift_rejected_before_any_output(self):
        for name in PROJECTION.SOURCE_SHA256:
            with self.subTest(name=name):
                path = self.source / name
                original = path.read_bytes()
                path.write_bytes(original + b"\n")
                with self.assertRaisesRegex(ValueError, "complete source changed"):
                    self.prepare()
                self.assertFalse(self.inputs.exists())
                path.write_bytes(original)

    def test_span_drift_and_ambiguous_anchor_fail_with_rebound_complete_pin(self):
        for name, span in PROJECTION.SPANS.items():
            with self.subTest(name=name):
                path = self.source / span["source"]
                original = path.read_bytes()
                first, last = span["offsets"]
                changed_at = first + len(span["start"]) + 2
                for source, failure in ((original[:changed_at] + b" " + original[changed_at + 1:], "source span changed"),
                                        (original + b"\n" + span["start"], "ambiguous")):
                    path.write_bytes(source)
                    with patch.dict(PROJECTION.SOURCE_SHA256, {span["source"]: PROJECTION.digest(source)}):
                        with self.assertRaisesRegex(ValueError, failure):
                            self.prepare()
                    self.assertFalse(self.inputs.exists())
                path.write_bytes(original)

    def test_case_and_all_output_collisions_fail_without_replacement(self):
        for case in ("other", None, True, 1):
            with self.subTest(case=case), self.assertRaisesRegex(ValueError, "Unknown lifecycle-ordinal control"):
                self.prepare(case)
        self.inputs.mkdir()
        for name in ("proof.rs", "lifecycle_ordinal_selected.rs", "lifecycle_ordinal_owner.rs", "correspondence.json"):
            target = self.inputs / name
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

    def test_healthy_domain_and_widened_oracle_have_no_filters_or_helpers(self):
        source = self.harness.read_text()
        healthy = source.split("fn lifecycle_ordinal_full_domain() {", 1)[1].split("#[kani::proof]", 1)[0]
        self.assertEqual(healthy.count("kani::any::<u64>()"), 1)
        self.assertIn("let mathematical_successor = (previous as u128) + 1;", healthy)
        self.assertIn("let observed = checked_next(previous);", healthy)
        self.assertIn("if let Ok(value) = &observed", healthy)
        for forbidden in ("kani::assume", "#[kani::unwind", "checked_add", "NonZeroU64::new", "for ", "while ",
                          ".unwrap(", ".expect(", "assert_eq!(observed", "impl "):
            self.assertNotIn(forbidden, source)

    def test_six_healthy_assertions_five_covers_and_dedicated_single_assertion_controls(self):
        source = self.harness.read_text()
        healthy = source.split("fn lifecycle_ordinal_full_domain() {", 1)[1].split("#[kani::proof]", 1)[0]
        self.assertEqual(len(re.findall(r"\bassert(?:_eq)?!", healthy)), 6)
        self.assertEqual(source.count("kani::cover!("), 5)
        for label in PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS]:
            self.assertEqual(healthy.count('"' + label + '"'), 1)
        for label in PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS]:
            self.assertEqual(source.count('"' + label + '"'), 1)
        harnesses = set(re.findall(r"#\[kani::proof\]\s+fn (\w+)\(", source))
        self.assertEqual(harnesses, {job["harness"].split("::")[-1] for job in PROJECTION.schedule()})
        for case, control in PROJECTION.CONTROLS.items():
            block = source.split("fn lifecycle_ordinal_control_" + case + "() {", 1)[1].split("#[kani::proof]", 1)[0]
            self.assertEqual(block.count("assert!("), 1)
            self.assertIn('"' + control["intended_failure"] + '"', block)
            self.assertIn("checked_next(" + ("u64::MAX" if case == "wrong_error" else "0") + ")", block)

    def test_registry_schedule_and_contract_independently_match_five_fixed_jobs(self):
        jobs = SUITES.jobs("lifecycle-ordinal")
        self.assertEqual(jobs, PROJECTION.schedule())
        self.assertEqual([job["case"] for job in jobs],
                         ["healthy", "always_error", "wrong_increment", "wrong_error", "healthy"])
        self.assertEqual([job["restored_healthy"] for job in jobs], [False] * 4 + [True])
        self.assertEqual(SUITES.expected_contract("lifecycle_ordinal", PROJECTION.FULL_HARNESS),
                         (frozenset(PROJECTION.REQUIRED_ASSERTIONS[PROJECTION.FULL_HARNESS]),
                          frozenset(PROJECTION.REQUIRED_COVERS[PROJECTION.FULL_HARNESS])))
        for job in jobs[1:4]:
            with self.assertRaises(ValueError):
                SUITES.expected_contract(job["subject"], job["harness"])
        jobs[0]["case"] = "caller modification"
        self.assertEqual(SUITES.jobs("lifecycle-ordinal")[0]["case"], "healthy")
        self.assertEqual(len(SUITES.jobs("request-timing")), 5)
        self.assertEqual(len(SUITES.jobs("selection-depth")), 6)
        self.assertEqual(SUITES.MAX_JOBS, 12)

    def test_registry_freezes_all_current_sources_and_rejects_altered_jobs_before_load(self):
        names = SUITES.input_names("lifecycle-ordinal")
        self.assertEqual(len(names), len(set(names)))
        self.assertTrue(set(PROJECTION.SOURCE_SHA256).issubset(names))
        self.assertTrue({"tools/formal/lifecycle_ordinal_projection.py", "tools/formal/lifecycle_ordinal_harness.rs",
                         "tools/formal/test_lifecycle_ordinal.py", "tools/formal/additional_suites.py",
                         "tools/formal/verify_lifecycle_ordinal.py", "tools/formal/test_verify_lifecycle_ordinal.py"}.issubset(names))
        job = SUITES.jobs("lifecycle-ordinal")[0]
        invalid = [(job, 0), (job, True), (job, 2), (job, 6)]
        invalid += [(dict(job, **{field: value}), 1) for field, value in
                    (("case", "always_error"), ("subject", "request_timing"), ("harness", "other"),
                     ("restored_healthy", True), ("intended_failure", "other"), ("extra", True))]
        def forbidden_loader(name, path):
            self.fail("An altered job reached source loading")
        for altered, index in invalid:
            with self.subTest(job=altered, index=index), self.assertRaises(ValueError):
                SUITES.prepare(forbidden_loader, self.source, self.inputs, altered, index)

    def test_registry_uses_only_current_supplied_python_loader(self):
        calls = []
        def controlled_loader(name, path):
            calls.append((name, path))
            self.assertEqual(path, self.source / "tools/formal/lifecycle_ordinal_projection.py")
            return PROJECTION
        harness_target = self.source / "tools/formal/lifecycle_ordinal_harness.rs"
        harness_target.parent.mkdir(parents=True)
        harness_target.write_bytes(self.harness.read_bytes())
        with patch.object(SUITES.importlib.util, "spec_from_file_location",
                          side_effect=AssertionError("Unexpected registry loader")):
            result = SUITES.prepare(controlled_loader, self.source, self.inputs,
                                    SUITES.jobs("lifecycle-ordinal")[0], 1)
        self.assertEqual(len(calls), 1)
        self.assertEqual(result["entrypoint"], self.inputs / "proof.rs")

    def test_clock_is_fixed_nonresetting_and_worse_elapsed_controls_expiry(self):
        self.assertEqual(PROJECTION.PREPARATION_STARTED_UNIX_NS, 1793943385621323997)
        self.assertEqual(PROJECTION.PREPARATION_STARTED_MONOTONIC_NS, 40053454753050)
        self.assertEqual(PROJECTION.PREPARATION_DEADLINE_MONOTONIC_NS, 43653454753050)
        self.assertEqual(PROJECTION.TOTAL_SECONDS, 3600)
        for boot, mono_delta, wall_delta, expected in (
                (PROJECTION.PREPARATION_BOOT_ID, 700, 800, 2800),
                ("foreign boot", 1, 1, None), (PROJECTION.PREPARATION_BOOT_ID, 3600, 1, None),
                (PROJECTION.PREPARATION_BOOT_ID, 1, 3600, None),
                (PROJECTION.PREPARATION_BOOT_ID, -1, 1, None),
                (PROJECTION.PREPARATION_BOOT_ID, 1, -1, None)):
            with self.subTest(boot=boot, mono=mono_delta, wall=wall_delta), \
                 patch.object(Path, "read_text", return_value=boot), \
                 patch.object(PROJECTION.time, "monotonic_ns", return_value=
                              PROJECTION.PREPARATION_STARTED_MONOTONIC_NS + mono_delta * 1_000_000_000), \
                 patch.object(PROJECTION.time, "time_ns", return_value=
                              PROJECTION.PREPARATION_STARTED_UNIX_NS + wall_delta * 1_000_000_000):
                if expected is None:
                    with self.assertRaises(ValueError):
                        PROJECTION.remaining_budget()
                else:
                    self.assertEqual(PROJECTION.remaining_budget(), expected)


if __name__ == "__main__":
    unittest.main()
