"""Inert correspondence and rejection tests; no compiler or proof execution."""
from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import sys
import tempfile
import tomllib
import unittest

ROOT = Path(__file__).resolve().parents[2]
MODULE = ROOT / "tools/formal/source_state_constructor_projection.py"
_spec = importlib.util.spec_from_file_location("tested_source_constructor", MODULE)
P = importlib.util.module_from_spec(_spec)
# Load this new module's exact buffer, bypassing cached bytecode.
exec(compile(MODULE.read_bytes(), str(MODULE), "exec", dont_inherit=True,
             optimize=sys.flags.optimize), P.__dict__)


class SourceStateConstructorPreparationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.temporary = tempfile.TemporaryDirectory(prefix="source-constructor-tests-")
        cls.base = Path(cls.temporary.name)
        cls.source = cls.base / "source"
        cls.source.mkdir(mode=0o700)
        for name in P.SOURCE_SHA256:
            destination = cls.source / name
            destination.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, destination)
        # Only owned temporary copies are mutated by adversarial vendor tests.
        originals = P._read_context(cls.source)
        cls.graph, cls.grants, cls.lock = P._graph_and_grants(originals)
        cls.vendor = cls.base / "vendor"
        cls.vendor.mkdir(mode=0o700)
        for record in cls.grants:
            name = record["name"] + "-" + record["version"]
            shutil.copytree(P.DEFAULT_VENDOR_ROOT / name, cls.vendor / name)
        cls.sequence = 0

    @classmethod
    def tearDownClass(cls):
        cls.temporary.cleanup()

    def output(self):
        type(self).sequence += 1
        return self.base / ("candidate-" + str(self.sequence))

    def prepare(self, case="healthy", output=None):
        output = output if output is not None else self.output()
        return output, P.prepare_source_state_constructor(
            self.source, output, case, ROOT / P.HARNESS, vendor_root=self.vendor)

    def test_exact_spans_and_real_type_closure(self):
        selected = P._selection(P._read_context(self.source))
        self.assertEqual(len(selected), 7)
        self.assertEqual(sum(map(len, selected.values())), 23483)
        self.assertEqual(sum(value.count(b"pub enum ") for name, value in selected.items()
                             if name.endswith("_errors")), 23)
        owner = selected["constructor"]
        for fragment in (b"#[derive(Validate)]", b"source_fields_rule(*value)",
                         b"RawSourceFields { counters }.validate()", b"validation.map_err",
                         b"NonZeroU64", b"impl fmt::Debug"):
            self.assertIn(fragment, owner)
        self.assertNotIn(b"time_domain", b"".join(selected.values()))

    def test_complete_source_and_policy_drift_rejected_before_output(self):
        for name in ("crates/time-core/src/combination/lifecycle.rs",
                     "crates/time-core/src/lib.rs", "crates/time-core/Cargo.toml",
                     "policy/ui-m0-dependencies.json", "rust-toolchain.toml"):
            with self.subTest(name=name):
                path = self.source / name
                original = path.read_bytes()
                output = self.output()
                try:
                    path.write_bytes(original + b"\n")
                    with self.assertRaisesRegex(ValueError, "Reviewed complete source changed"):
                        self.prepare(output=output)
                    self.assertFalse(output.exists())
                finally:
                    path.write_bytes(original)

    def test_independently_pinned_span_and_enum_drift(self):
        originals = P._read_context(self.source)
        name = "crates/time-core/src/selection.rs"
        originals[name] = originals[name].replace(b"pub enum ", b"pub struct ", 1)
        with self.assertRaisesRegex(ValueError, "span changed"):
            P._selection(originals)

    def test_exact_fifteen_package_root_and_grants(self):
        parsed = tomllib.loads(self.lock.decode())
        self.assertEqual(len(parsed["package"]), 15)
        root = next(x for x in parsed["package"] if x["name"] == P.PACKAGE_NAME)
        self.assertEqual(root, {"name": P.PACKAGE_NAME, "version": "0.0.0", "dependencies": ["garde"]})
        self.assertNotIn(b"nevrotick-era-constructor-proof", self.lock)
        self.assertEqual(len({(x["name"], x["version"]) for x in self.grants}), 14)
        for record in self.grants:
            self.assertRegex(record["files_sha256"], r"^[0-9a-f]{64}$")
            self.assertIsInstance(record["expected_features"], list)
            self.assertIn("build_path", record)
            self.assertIn("proc_macro", record)

    def test_healthy_restored_identical_every_generated_file_and_flags(self):
        healthy, first = self.prepare()
        restored, last = self.prepare()
        self.assertEqual(first, last)
        self.assertEqual(first["all_generated_files_sha256"], last["all_generated_files_sha256"])
        self.assertEqual(first["vendor_observation_before"], first["vendor_observation_after"])
        self.assertTrue(first["source_payload_authenticated_at_observation_time"])
        for flag in ("source_is_sealed", "execution_ready", "execution_admitted", "proof_qualified",
                     "target_features_observed", "compilation_observed", "controls_observed"):
            self.assertIs(first[flag], False)
        self.assertEqual(first["native_commands_executed"], 0)
        for name, expected in first["all_generated_files_sha256"].items():
            self.assertEqual((healthy / name).read_bytes(), (restored / name).read_bytes())
            self.assertEqual(hashlib.sha256((healthy / name).read_bytes()).hexdigest(), expected)
            self.assertEqual((healthy / name).stat().st_mode & 0o777, 0o600)
        self.assertEqual(healthy.stat().st_mode & 0o777, 0o700)
        self.assertEqual(len(first["recommended_schedule"]), 6)
        self.assertTrue(first["recommended_schedule"][-1]["restored_healthy"])
        self.assertEqual(first["observed_python_sources"], P.BINDING.snapshot())

    def test_all_four_mutations_are_actual_isolated_owner_changes(self):
        healthy, _ = self.prepare()
        for case, control in P.CONTROLS.items():
            with self.subTest(case=case):
                output, result = self.prepare(case)
                original, changed = (healthy / "src/owner.rs").read_bytes(), (output / "src/owner.rs").read_bytes()
                self.assertEqual(original.count(control["before"].encode()), 1)
                self.assertEqual(changed, original.replace(control["before"].encode(), control["after"].encode(), 1))
                self.assertEqual(result["mutation"], control)
                self.assertEqual((healthy / "src/harness.rs").read_bytes(), (output / "src/harness.rs").read_bytes())
                if case == "validation_bypass":
                    self.assertIn(b"fn source_fields_rule", changed)
                    self.assertIn(b"if source_fields_valid(value)", changed)
                    self.assertIn(b"if !source_fields_valid(counters)", changed)
                    self.assertIn(b"#[derive(Validate)]", changed)
                    self.assertNotIn(b"source_fields_rule(*value)", changed)

    def test_full_raw_harness_independent_oracle_and_focused_controls(self):
        harness = (ROOT / P.HARNESS).read_text()
        full = harness.split("#[kani::proof]", 2)[1]
        self.assertEqual(full.count("kani::any::<u64>()"), 2)
        self.assertEqual(full.count("kani::any::<u8>()"), 2)
        self.assertNotIn("kani::assume", harness)
        self.assertEqual(full.count("return;"), 1)
        self.assertIn("NonZeroU64::new(raw_label)", full)
        self.assertIn("maturity as u128 == ordinal as u128", full)
        self.assertIn("ordinal >= 8 && maturity == 8", full)
        self.assertIn("let result = FixtureCombinationSourceState::new", full)
        self.assertIn("if let Ok(state) = &result", full)
        for name in ("source_id", "reachability_updates", "penalty", "last_reachability_ordinal"):
            self.assertIn("state." + name + "()", full)
        for control in P.CONTROLS.values():
            self.assertIn(control["intended_failure"], harness)
        for body in harness.split("#[kani::proof]")[2:]:
            self.assertEqual(body.count("assert!("), 1)
            self.assertNotIn("source_state_constructor_full_domain()", body)

    def test_vendor_payload_and_self_consistent_map_tampering_rejected(self):
        record = self.grants[0]
        package = self.vendor / (record["name"] + "-" + record["version"])
        path = package / "Cargo.toml"
        metadata = package / ".cargo-checksum.json"
        source, original_map = path.read_bytes(), metadata.read_bytes()
        try:
            path.write_bytes(source + b"\n")
            with self.assertRaisesRegex(P.VENDOR.Incomplete, "Actual vendor bytes"):
                self.prepare()
            changed_map = json.loads(original_map)
            changed_map["files"]["Cargo.toml"] = P.digest(path.read_bytes())
            metadata.write_text(json.dumps(changed_map))
            with self.assertRaisesRegex(P.VENDOR.Incomplete, "file map differs"):
                self.prepare()
        finally:
            path.write_bytes(source)
            metadata.write_bytes(original_map)

    def test_grant_drift_rejected(self):
        originals = P._read_context(self.source)
        policy = json.loads(originals["policy/ui-m0-dependencies.json"])
        policy["packages"][0]["checksum"] = "0" * 64
        originals["policy/ui-m0-dependencies.json"] = json.dumps(policy).encode()
        with self.assertRaisesRegex(ValueError, "Policy differs"):
            P._graph_and_grants(originals)
        policy["packages"].append(policy["packages"][0])
        originals["policy/ui-m0-dependencies.json"] = json.dumps(policy).encode()
        with self.assertRaisesRegex(ValueError, "Duplicate"):
            P._graph_and_grants(originals)

    def test_unsafe_output_alias_and_overwrite_rejected(self):
        output, _ = self.prepare()
        with self.assertRaisesRegex(ValueError, "Fresh output"):
            self.prepare(output=output)
        alias = self.base / "alias"
        alias.symlink_to(self.base, target_is_directory=True)
        try:
            with self.assertRaisesRegex(ValueError, "Fresh output"):
                self.prepare(output=alias / "candidate")
        finally:
            alias.unlink()
        with self.assertRaisesRegex(ValueError, "Fresh output"):
            self.prepare(output=self.source / "new-output")

    def test_bootstrap_rejects_oversized_redirected_and_aliased_helper(self):
        fixture = self.base / "bootstrap-helper.py"
        fixture.write_bytes(b"VALUE = 1\n")
        self.assertEqual(P._binding_buffer(fixture), b"VALUE = 1\n")
        fixture.write_bytes(b" " * (2 * 1024**2 + 1))
        with self.assertRaises((ValueError, RuntimeError)):
            P._binding_buffer(fixture)
        fixture.write_bytes(b"VALUE = 1\n")
        alias = self.base / "bootstrap-helper-alias.py"
        alias.symlink_to(fixture)
        try:
            with self.assertRaises((ValueError, RuntimeError)):
                P._binding_buffer(alias)
        finally:
            alias.unlink()
        os.link(fixture, alias)
        try:
            with self.assertRaises((ValueError, RuntimeError)):
                P._binding_buffer(fixture)
        finally:
            alias.unlink()
            fixture.unlink()

    def test_stale_executing_module_and_binding_pin_rejected(self):
        # Check a stale helper buffer directly using the real binding's source check.
        real_path = ROOT / "tools/formal/era_constructor_projection.py"
        stale = compile(real_path.read_bytes() + b"\nSTALE_MARKER = True\n", str(real_path), "exec", dont_inherit=True)
        with self.assertRaisesRegex(P._binding_module.SourceBindingError, "differs from source"):
            P.BINDING._bind_code(real_path, stale)
        self.assertEqual(P._verify_python(), P.BINDING.snapshot())
        original = P.HELPER_SHA256["tools/formal/era_constructor_projection.py"]
        try:
            P.HELPER_SHA256["tools/formal/era_constructor_projection.py"] = "0" * 64
            with self.assertRaisesRegex(ValueError, "Observed helper differs from pin"):
                P._verify_python()
        finally:
            P.HELPER_SHA256["tools/formal/era_constructor_projection.py"] = original


if __name__ == "__main__":
    unittest.main()
