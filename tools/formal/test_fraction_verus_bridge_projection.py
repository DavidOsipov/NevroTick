"""Source correspondence controls only; these tests run no Rust verifier."""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import re
import sys
import tempfile
import unittest


sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
LOADER = importlib.util.spec_from_file_location("fraction_verus_bridge_test", HERE / "fraction_verus_bridge_projection.py")
PROJECTION = importlib.util.module_from_spec(LOADER)
LOADER.loader.exec_module(PROJECTION)


class FractionVerusBridgePreparationTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-verus-bridge-source-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.source = self.base / "source"
        for name in PROJECTION.input_names():
            target = self.source / name
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / name).read_bytes())
        self.output = self.base / "candidate"
        self.prefix = (self.source / "tools/formal/fraction_composition_spec.rs").read_bytes().split(
            PROJECTION.PREFIX_MARKER, 1)[0]

    def prepare(self):
        return PROJECTION.prepare(self.source, self.output)

    def assert_rejected_without_output(self, pattern):
        with self.assertRaisesRegex(ValueError, pattern):
            self.prepare()
        self.assertFalse(self.output.exists())

    def test_finite_input_inventory_and_explicit_unproved_status(self):
        self.assertEqual(len(PROJECTION.input_names()), 7)
        self.assertEqual(len(set(PROJECTION.input_names())), 7)
        result = self.prepare()
        self.assertEqual(result["status"], "unproved_source_candidate")
        self.assertEqual(result["proof_commands"], 0)
        self.assertNotIn("adoption", result)
        self.assertNotIn("entrypoint", result)
        self.assertEqual({p.name for p in self.output.iterdir()},
                         {"bridge.rs", "actual_source.rs", "correspondence.json"})
        self.assertEqual(json.loads((self.output / "correspondence.json").read_text()), result)

    def test_all_nine_product_spans_and_source_maps_are_exact(self):
        result = self.prepare()
        actual = (self.output / "actual_source.rs").read_bytes()
        bridge = (self.output / "bridge.rs").read_bytes()
        self.assertEqual(tuple(row["item"] for row in result["correspondence"]), PROJECTION.SPAN_NAMES)
        for row in result["correspondence"]:
            first, last = row["byte_offsets_half_open"]
            selected = (self.source / row["source"]).read_bytes()[first:last]
            self.assertEqual(PROJECTION.digest(selected), row["sha256"])
            self.assertEqual(actual.count(selected), 1, row["item"])
            self.assertEqual(selected in bridge, row["bridge_type"], row["item"])
        self.assertIn(b"impl Fraction {", actual)
        self.assertNotIn(b"verus!", actual)
        self.assertNotIn(b"composition_model", actual)

    def test_only_reviewed_annotations_insert_and_inverse_erases_whole_prefix(self):
        candidate = PROJECTION.annotated(self.prefix)
        self.assertEqual(candidate.count(b"#[verifier::allow_in_spec]"), 6)
        self.assertEqual(candidate.count(b"\n    returns "), 6)
        self.assertEqual(PROJECTION.erase_annotations(candidate), self.prefix)
        self.assertEqual(PROJECTION.digest(self.prefix), PROJECTION.PREFIX_SHA256)
        for declaration, relation in PROJECTION.DECLARATIONS:
            self.assertEqual(candidate.count(PROJECTION._annotation(declaration, relation)), 1)
        self.prepare()
        self.assertIn(candidate, (self.output / "bridge.rs").read_bytes())

    def test_unrestricted_raw_fields_results_and_rejection_guards_survive(self):
        self.prepare()
        bridge = (self.output / "bridge.rs").read_bytes()
        self.assertNotRegex(bridge, rb"\brequires\b")
        self.assertIn(b"result: &Result<Fraction, FixtureEnclosureError>", bridge)
        self.assertIn(b"result: &Result<Ordering, FixtureEnclosureError>", bridge)
        self.assertIn(b"if !composition_checked(left) || !composition_checked(right)", bridge)
        self.assertIn(b"if !composition_checked(input) {\n        return false;", bridge)
        self.assertIn(b"Err(FixtureEnclosureError::Invariant) => !admitted", bridge)
        self.assertIn(b"Err(FixtureEnclosureError::Invariant) => !required_success", bridge)
        # Raw malformed Ok fields and all error values remain inputs: no seed
        # decoder, constructor invocation, assumption or caller-domain filter.
        self.assertNotIn(b"CompositionFractionSeed", bridge)
        self.assertNotIn(b"composition_decode", bridge)
        self.assertNotIn(b"Fraction::new(", bridge)

    def test_exact_native_ordering_error_types_and_no_local_assumptions(self):
        self.prepare()
        bridge = (self.output / "bridge.rs").read_bytes()
        for leaf in (b"Unrepresentable", b"Invariant", b"ZeroLabel", b"ReversedBounds", b"ValidationInvariant"):
            self.assertIn(leaf, bridge)
        self.assertIn(b"use std::cmp::Ordering;", bridge)
        self.assertNotRegex(bridge, rb"(?:struct|enum) Ordering")
        self.assertIn(b"// No constructor, filtering, assume, custom Arbitrary", bridge)
        local_code = re.sub(rb"(?m)//[^\n]*", b"", bridge)
        self.assertNotRegex(local_code, rb"\b(?:assume|admit|external_body|external_fn_specification|assume_specification|axiom)\b")
        self.assertNotIn(b"#[kani::", bridge)

    def test_product_drift_is_rejected_before_directory_creation(self):
        for name in ("projection_enclosure.rs", "selection.rs"):
            with self.subTest(source=name):
                path = self.source / "crates/time-core/src" / name
                original = path.read_bytes()
                path.write_bytes(original + b"\n// changed product\n")
                self.assert_rejected_without_output("complete product source changed")
                path.write_bytes(original)

    def test_old_support_is_read_inertly_and_unreviewed_code_is_rejected(self):
        path = self.source / "tools/formal/fraction_arithmetic_projection.py"
        path.write_bytes(path.read_bytes() + b"\nraise RuntimeError('must never execute')\n")
        self.assert_rejected_without_output("arithmetic projection changed")

    def test_predicate_body_raw_result_and_domain_changes_are_rejected(self):
        path = self.source / "tools/formal/fraction_composition_spec.rs"
        original = path.read_bytes()
        for before, after in (
            (b"Err(_) => false,", b"Err(_) => true,"),
            (b"if !composition_checked(input) {", b"if false {"),
            (b"return Ordering::Equal;", b"return Ordering::Greater;"),
            (b"shift <= 1041", b"shift <= 53"),
        ):
            with self.subTest(mutation=after):
                self.assertIn(before, original)
                path.write_bytes(original.replace(before, after, 1))
                self.assert_rejected_without_output("composition specification changed")
        path.write_bytes(original)

    def test_ghost_specification_drift_cannot_rebind_returns(self):
        path = self.source / "tools/formal/fraction_verus_bridge_spec.rs"
        path.write_bytes(path.read_bytes() + b"\n// different ghost source\n")
        self.assert_rejected_without_output("ghost specification changed")

    def test_private_spec_visibility_preserves_every_failed_candidate_body_byte(self):
        path = self.source / "tools/formal/fraction_verus_bridge_spec.rs"
        current = path.read_bytes()
        self.assertEqual(re.findall(rb"(?m)^spec fn (bridge_[a-z_]+)\(", current), [
            b"bridge_denominator", b"bridge_constructor_admitted", b"bridge_checked",
            b"bridge_constructor_post", b"bridge_rational_order", b"bridge_comparison_post",
            b"bridge_complement_post",
        ])
        # The actual pinned compiler rejected these seven private `open` functions.
        # Reconstruct its exact input to prove this repair changes no math body.
        failed = re.sub(rb"(?m)^spec fn ", b"open spec fn ", current)
        self.assertEqual(PROJECTION.digest(failed),
                         "2607c7b3f33058bfda89e189f61c4564a15cef403dbf6a877cb6363029598f1a")
        path.write_bytes(failed)
        self.assert_rejected_without_output("ghost specification changed")

    def test_source_file_directory_and_root_symlinks_are_rejected(self):
        path = self.source / "tools/formal/fraction_composition_spec.rs"
        real = self.base / "real-spec.rs"
        real.write_bytes(path.read_bytes())
        path.unlink()
        path.symlink_to(real)
        self.assert_rejected_without_output("path redirects")
        path.unlink()
        path.write_bytes(real.read_bytes())
        alias = self.base / "source-alias"
        alias.symlink_to(self.source, target_is_directory=True)
        with self.assertRaisesRegex(ValueError, "path redirects"):
            PROJECTION.prepare(alias, self.output)
        formal = self.source / "tools/formal"
        moved = self.base / "formal"
        formal.rename(moved)
        formal.symlink_to(moved, target_is_directory=True)
        self.assert_rejected_without_output("path redirects")

    def test_output_collision_symlink_and_source_containment_preserve_custody(self):
        self.output.mkdir()
        sentinel = self.output / "existing"
        sentinel.write_bytes(b"retain me")
        with self.assertRaisesRegex(ValueError, "fresh output leaf"):
            self.prepare()
        self.assertEqual(sentinel.read_bytes(), b"retain me")
        self.assertEqual([p.name for p in self.output.iterdir()], ["existing"])
        sentinel.unlink()
        self.output.rmdir()
        self.output.symlink_to(self.base / "absent-output")
        with self.assertRaisesRegex(ValueError, "path redirects"):
            self.prepare()
        self.output.unlink()
        with self.assertRaisesRegex(ValueError, "outside its source tree"):
            PROJECTION.prepare(self.source, self.source / "candidate")
        self.assertFalse((self.source / "candidate").exists())
        alias = self.base / ".." / self.base.name / "source" / "candidate"
        with self.assertRaisesRegex(ValueError, "not canonical"):
            PROJECTION.prepare(self.source, alias)
        self.assertFalse((self.source / "candidate").exists())

    def test_inverse_erasure_rejects_body_signature_and_contract_tampering(self):
        candidate = PROJECTION.annotated(self.prefix)
        for before, after in (
            (b"Err(_) => false,", b"Err(_) => true,"),
            (b"returns bridge_checked(value)", b"returns true"),
            (b"value: Fraction", b"value: &Fraction"),
            (b"    let bits = 64 - numerator.leading_zeros();", b"    let bits = 0;"),
        ):
            with self.subTest(mutation=after):
                with self.assertRaises(ValueError):
                    PROJECTION.erase_annotations(candidate.replace(before, after, 1))
        with self.assertRaisesRegex(ValueError, "prefix changed"):
            PROJECTION.annotated(self.prefix.replace(b"Err(_) => false,", b"Err(_) => true,", 1))

    def test_deterministic_outputs_freeze_every_source_and_no_execution_surface(self):
        first = self.prepare()
        second_output = self.base / "second-candidate"
        second = PROJECTION.prepare(self.source, second_output)
        self.assertEqual(first, second)
        for path in self.output.iterdir():
            self.assertEqual(path.read_bytes(), (second_output / path.name).read_bytes())
        self.assertEqual(set(first["source_files_sha256"]), set(PROJECTION.input_names()))
        for name, expected in first["generated_sha256"].items():
            self.assertEqual(PROJECTION.digest((self.output / name).read_bytes()), expected)
        script = (HERE / "fraction_verus_bridge_projection.py").read_text()
        self.assertNotRegex(script, r"(?:^|\n)(?:from|import) (?:subprocess|os|importlib)\b")
        self.assertNotIn("if __name__", script)
        self.assertNotIn("--no-cheating", script)
        self.assertNotIn("ADOPT_LIMITED", script)
        self.assertNotIn("PENDING_", script)


class FractionBitLengthPreparationTests(unittest.TestCase):
    def setUp(self):
        FractionVerusBridgePreparationTests.setUp(self)
        path = self.source / "tools/formal/fraction_verus_bridge_lemmas.rs"
        path.write_bytes((ROOT / "tools/formal/fraction_verus_bridge_lemmas.rs").read_bytes())
        self.fragment = self.prefix.split(b"fn composition_constructor_post(", 1)[0]

    def prepare(self):
        return PROJECTION.prepare_bit_length(self.source, self.output)

    def assert_rejected_without_output(self, pattern):
        with self.assertRaisesRegex(ValueError, pattern):
            self.prepare()
        self.assertFalse(self.output.exists())

    def test_dependency_inventory_unproved_scope_and_exact_mixed_modes(self):
        self.assertEqual(len(PROJECTION.input_names()), 7)
        self.assertEqual(PROJECTION.dependency_input_names(),
                         (*PROJECTION.input_names(), "tools/formal/fraction_verus_bridge_lemmas.rs"))
        result = self.prepare()
        self.assertEqual(len(result["source_files_sha256"]), 8)
        self.assertEqual(set(result["source_files_sha256"]), set(PROJECTION.dependency_input_names()))
        self.assertEqual(result["status"], "unproved_source_candidate")
        self.assertEqual(result["proof_commands"], 0)
        self.assertEqual(result["expected_function_modes"], {
            "proof::lemma_bridge_bit_length_power": "proof",
            "proof::composition_constructor_admitted": "exec",
            "proof::composition_checked": "exec",
            "proof::IntervalError::clone": "exec",
            "proof::FixtureEnclosureError::clone": "exec",
            "proof::Fraction::clone": "exec",
        })
        self.assertEqual(result["raw_domains"], {
            "lemma": "every n:u64 and s:u32; no precondition",
            "constructor_admitted": "every numerator:u64 and shift:u32; no precondition",
            "checked": "every raw Fraction { numerator:u64, shift:u32 }; no precondition",
        })
        self.assertEqual(len(result["predicate_relations"]), 2)
        self.assertEqual(len(result["full_bridge_predicate_relations"]), 6)
        self.assertIn("all six full-domain bridge equalities and their controls", result["remaining"])
        self.assertIn("u32 leading_zeros result and subtraction-to-int inference", result["remaining"])
        self.assertNotIn("adoption", result)
        self.assertNotIn("entrypoint", result)

    def test_two_exact_pre_subtraction_hooks_and_complete_correspondence_inputs(self):
        result = self.prepare()
        bridge = (self.output / "proof.rs").read_bytes()
        actual = (self.output / "actual_source.rs").read_bytes()
        candidate = PROJECTION.annotated_bit_length(self.fragment)
        self.assertEqual(len(self.fragment), 661)
        self.assertEqual(PROJECTION.digest(self.fragment),
                         "f8f90170153ec3bb7a49b0a3bdcd8bf770bb511d3f59429dadd427d2b67cc947")
        self.assertEqual(candidate.count(b"    proof {\n"), 2)
        self.assertEqual(candidate.count(b"#[verifier::allow_in_spec]"), 2)
        self.assertEqual(candidate.count(b"\n    returns "), 2)
        self.assertEqual(PROJECTION.erase_bit_length_annotations(candidate), self.fragment)
        self.assertIn(candidate, bridge)
        self.assertEqual((self.output / "predicate_source.rs").read_bytes(), self.prefix)
        self.assertEqual(result["prefix_bytes"], 3731)
        self.assertEqual(result["prefix_sha256"], PROJECTION.PREFIX_SHA256)
        self.assertEqual(result["fragment_source_byte_offsets_half_open"], [0, 661])
        for index, (name, anchor, block) in enumerate(PROJECTION.BIT_LENGTH_HOOKS):
            self.assertEqual(candidate.count(block + anchor), 1)
            self.assertEqual(result["proof_hooks"][index], {
                "function": "proof::" + name,
                "source_byte_offset": (245, 414)[index],
                "insertion": "before unchanged leading_zeros subtraction",
                "anchor_sha256": PROJECTION.digest(anchor),
                "proof_block_sha256": PROJECTION.digest(block),
            })
        self.assertEqual(tuple(row["item"] for row in result["correspondence"]), PROJECTION.SPAN_NAMES)
        for row in result["correspondence"]:
            first, last = row["byte_offsets_half_open"]
            selected = (self.source / row["source"]).read_bytes()[first:last]
            self.assertEqual(actual.count(selected), 1, row["item"])
            self.assertEqual(selected in bridge, row["bridge_type"], row["item"])
        for path in ("fraction_verus_bridge_spec.rs", "fraction_verus_bridge_lemmas.rs"):
            self.assertEqual(bridge.count((self.source / "tools/formal" / path).read_bytes()), 1)
        self.assertNotIn(b"fn composition_constructor_post(", bridge)
        self.assertNotIn(b"fn add_fractions(", bridge)
        self.assertNotRegex(re.sub(rb"(?m)//[^\n]*", b"", bridge),
                            rb"\b(?:requires|assume|admit|external_body|external_fn_specification|assume_specification|axiom)\b")

    def test_exact_erasure_rejects_hook_body_signature_and_contract_tampering(self):
        candidate = PROJECTION.annotated_bit_length(self.fragment)
        for before, after in (
            (b"value: Fraction", b"value: &Fraction"),
            (b"returns bridge_checked(value)", b"returns true"),
            (b"let bits = 64 - numerator.leading_zeros();", b"let bits = 0;"),
            (b"numerator.leading_zeros();", b"numerator.leading_zeros() as u64;"),
            (b"bits <= 53", b"bits < 53"),
            (b"axiom_u64_leading_zeros(numerator);", b""),
            (b"lemma_bridge_bit_length_power(numerator, 53);", b"lemma_bridge_bit_length_power(numerator, 52);"),
            (b"lemma_bridge_bit_length_power(value.numerator, value.shift);",
             b"lemma_bridge_bit_length_power(value.numerator, 53);"),
        ):
            with self.subTest(mutation=after):
                with self.assertRaises(ValueError):
                    PROJECTION.erase_bit_length_annotations(candidate.replace(before, after, 1))
        for _, anchor, block in PROJECTION.BIT_LENGTH_HOOKS:
            for altered in (candidate.replace(block, b"", 1),
                            candidate.replace(block, block + block, 1),
                            candidate.replace(block + anchor, anchor + block, 1)):
                with self.assertRaises(ValueError):
                    PROJECTION.erase_bit_length_annotations(altered)
        with self.assertRaisesRegex(ValueError, "two-predicate slice changed"):
            PROJECTION.annotated_bit_length(self.fragment + b"\n")

    def test_lemma_content_drift_is_rejected_before_any_output(self):
        path = self.source / "tools/formal/fraction_verus_bridge_lemmas.rs"
        original = path.read_bytes()
        for altered in (original + b"\n", original.replace(b"decreases n,", b"decreases 0,", 1),
                        original.replace(b"< pow(2, s as nat)", b"<= pow(2, s as nat)", 1)):
            path.write_bytes(altered)
            self.assert_rejected_without_output("Pinned bit-length ghost lemma changed")
        path.write_bytes(original)

    def test_new_path_rejects_redirected_lemma_and_source_containment(self):
        path = self.source / "tools/formal/fraction_verus_bridge_lemmas.rs"
        original = path.read_bytes()
        target = self.base / "lemma.rs"
        target.write_bytes(original)
        path.unlink()
        path.symlink_to(target)
        self.assert_rejected_without_output("path redirects")
        path.unlink()
        path.write_bytes(original)
        with self.assertRaisesRegex(ValueError, "outside its source tree"):
            PROJECTION.prepare_bit_length(self.source, self.source / "candidate")
        self.assertFalse((self.source / "candidate").exists())
        alias = self.base / "source-alias"
        alias.symlink_to(self.source, target_is_directory=True)
        with self.assertRaisesRegex(ValueError, "path redirects"):
            PROJECTION.prepare_bit_length(alias, self.output)
        self.assertFalse(self.output.exists())

    def test_new_path_collision_and_output_symlink_preserve_existing_custody(self):
        self.output.mkdir()
        sentinel = self.output / "existing"
        sentinel.write_bytes(b"retain me")
        with self.assertRaisesRegex(ValueError, "fresh output leaf"):
            self.prepare()
        self.assertEqual(sentinel.read_bytes(), b"retain me")
        self.assertEqual([path.name for path in self.output.iterdir()], ["existing"])
        sentinel.unlink()
        self.output.rmdir()
        self.output.symlink_to(self.base / "missing")
        with self.assertRaisesRegex(ValueError, "path redirects"):
            self.prepare()

    def test_original_path_rust_bytes_remain_identical_and_do_not_require_lemma(self):
        (self.source / "tools/formal/fraction_verus_bridge_lemmas.rs").unlink()
        old = PROJECTION.prepare(self.source, self.output)
        bridge = (self.output / "bridge.rs").read_bytes()
        self.assertEqual(len(bridge), 9628)
        self.assertEqual(PROJECTION.digest(bridge),
                         "bcb1ca3d22a7e15b3948d65da34d1ecf6f34547153374591717948d4a279a4b5")
        self.assertEqual(len(old["source_files_sha256"]), 7)
        self.assertEqual(len(old["predicate_relations"]), 6)
        self.assertNotIn(b"lemma_bridge_bit_length_power", bridge)
        self.assertNotIn("expected_function_modes", old)

    def test_new_path_retains_original_product_and_specification_drift_rejection(self):
        for name, before, after, pattern in (
            ("crates/time-core/src/projection_enclosure.rs", b"fn new(", b"fn changed(", "complete product source changed"),
            ("crates/time-core/src/selection.rs", b"enum IntervalError", b"enum ChangedError", "complete product source changed"),
            ("tools/formal/fraction_composition_spec.rs", b"bits <= 53", b"bits <= 52", "composition specification changed"),
            ("tools/formal/fraction_verus_bridge_spec.rs", b"shift <= 1041", b"shift <= 1040", "ghost specification changed"),
        ):
            path = self.source / name
            original = path.read_bytes()
            self.assertIn(before, original)
            path.write_bytes(original.replace(before, after, 1))
            self.assert_rejected_without_output(pattern)
            path.write_bytes(original)

    def test_new_path_is_deterministic_with_exact_payload_and_no_execution_surface(self):
        first = self.prepare()
        other = self.base / "second-bit-length"
        second = PROJECTION.prepare_bit_length(self.source, other)
        self.assertEqual(first, second)
        self.assertEqual({path.name for path in self.output.iterdir()},
                         {"proof.rs", "actual_source.rs", "predicate_source.rs", "correspondence.json"})
        self.assertEqual(json.loads((self.output / "correspondence.json").read_text()), first)
        for name, expected in first["generated_sha256"].items():
            self.assertEqual(PROJECTION.digest((self.output / name).read_bytes()), expected)
            self.assertEqual((self.output / name).read_bytes(), (other / name).read_bytes())
        self.assertEqual(first["lemma_sha256"],
                         "b2fddd6592ff98ccd8fc450821d71f462966c8ce70e4322044f2df9d8acafce0")
        script = (HERE / "fraction_verus_bridge_projection.py").read_text()
        self.assertNotRegex(script, r"(?:^|\n)(?:from|import) (?:subprocess|os|importlib)\b")
        self.assertNotIn("if __name__", script)
        self.assertNotIn("--no-cheating", script)


class FractionConstructorPostPreparationTests(unittest.TestCase):
    def setUp(self):
        FractionBitLengthPreparationTests.setUp(self)
        self.fragment = self.prefix.split(b"// Total even on arbitrary raw inputs", 1)[0]

    def prepare(self):
        return PROJECTION.prepare_constructor_post(self.source, self.output)

    def assert_rejected_without_output(self, pattern):
        with self.assertRaisesRegex(ValueError, pattern):
            self.prepare()
        self.assertFalse(self.output.exists())

    def test_exact_metadata_schema_scope_raw_domains_and_seven_modes(self):
        result = self.prepare()
        self.assertEqual(set(result), {
            "schema_version", "status", "proof_commands", "scope", "source_files_sha256",
            "original_sources_sha256", "correspondence", "prefix_sha256", "prefix_bytes",
            "fragment_source_byte_offsets_half_open", "fragment_sha256", "annotated_fragment_sha256",
            "ghost_sha256", "lemma_sha256", "predicate_relations", "full_bridge_predicate_relations",
            "expected_function_modes", "proof_hooks", "raw_domains", "upstream_dependencies",
            "remaining", "generated_sha256",
        })
        self.assertEqual(result["schema_version"], 1)
        self.assertEqual(result["status"], "unproved_source_candidate")
        self.assertEqual(result["proof_commands"], 0)
        self.assertEqual(result["scope"],
                         "bit-length lemma and three unrestricted predicate equalities; source preparation only")
        self.assertEqual(result["expected_function_modes"], {
            "proof::lemma_bridge_bit_length_power": "proof",
            "proof::composition_constructor_admitted": "exec",
            "proof::composition_checked": "exec",
            "proof::composition_constructor_post": "exec",
            "proof::IntervalError::clone": "exec",
            "proof::FixtureEnclosureError::clone": "exec",
            "proof::Fraction::clone": "exec",
        })
        self.assertEqual(result["raw_domains"], {
            "lemma": "every n:u64 and s:u32; no precondition",
            "constructor_admitted": "every numerator:u64 and shift:u32; no precondition",
            "checked": "every raw Fraction { numerator:u64, shift:u32 }; no precondition",
            "constructor_post": "every numerator:u64, shift:u32 and independently chosen &Result<Fraction, FixtureEnclosureError>; raw Ok fields and every error variant; no precondition",
        })
        self.assertEqual(result["predicate_relations"], [
            "bridge_constructor_admitted(numerator, shift)", "bridge_checked(value)",
            "bridge_constructor_post(numerator, shift, result)",
        ])
        self.assertEqual(result["remaining"], [
            "exact Verus compatibility and all lemma assertions/termination",
            "u32 leading_zeros result and subtraction-to-int inference",
            "three completed full-raw predicate equalities and wrong-contract/restoration controls",
            "all six full-domain bridge equalities and their controls",
            "native Ordering equality correspondence, rational alignment and complement",
            "separate caller-import profile admission and original addition union",
        ])
        self.assertEqual(result["source_files_sha256"], {
            name: PROJECTION.digest((self.source / name).read_bytes())
            for name in PROJECTION.dependency_input_names()
        })
        self.assertEqual(len(result["source_files_sha256"]), 8)

    def test_exact_fragment_hooks_full_prefix_native_types_and_product_closure(self):
        result = self.prepare()
        proof = (self.output / "proof.rs").read_bytes()
        actual = (self.output / "actual_source.rs").read_bytes()
        candidate = PROJECTION.annotated_constructor_post(self.fragment)
        self.assertEqual(PROJECTION.CONSTRUCTOR_POST_MARKER, b"// Total even on arbitrary raw inputs")
        self.assertEqual(len(self.fragment), 1202)
        self.assertEqual(PROJECTION.digest(self.fragment),
                         "5b8758b1faaa516db3b849dc24d4822519dbbf0dfa55834c91605d8f4bb15217")
        self.assertEqual(PROJECTION.CONSTRUCTOR_POST_SLICE_SHA256, PROJECTION.digest(self.fragment))
        self.assertEqual(result["fragment_source_byte_offsets_half_open"], [0, 1202])
        self.assertEqual(result["fragment_sha256"], PROJECTION.digest(self.fragment))
        self.assertEqual(result["annotated_fragment_sha256"], PROJECTION.digest(candidate))
        self.assertEqual(candidate.count(b"    proof {\n"), 2)
        self.assertEqual(candidate.count(b"#[verifier::allow_in_spec]"), 3)
        self.assertEqual(candidate.count(b"\n    returns "), 3)
        self.assertEqual(PROJECTION.erase_constructor_post_annotations(candidate), self.fragment)
        self.assertIn(candidate, proof)
        for declaration, relation in PROJECTION.DECLARATIONS[:3]:
            self.assertEqual(candidate.count(PROJECTION._annotation(declaration, relation)), 1)
        self.assertEqual((self.output / "predicate_source.rs").read_bytes(), self.prefix)
        self.assertEqual(result["prefix_bytes"], 3731)
        self.assertEqual(result["prefix_sha256"], PROJECTION.PREFIX_SHA256)
        for index, (name, anchor, block) in enumerate(PROJECTION.BIT_LENGTH_HOOKS):
            self.assertEqual(candidate.count(block + anchor), 1)
            self.assertEqual(result["proof_hooks"][index], {
                "function": "proof::" + name, "source_byte_offset": (245, 414)[index],
                "insertion": "before unchanged leading_zeros subtraction",
                "anchor_sha256": PROJECTION.digest(anchor), "proof_block_sha256": PROJECTION.digest(block),
            })
        self.assertEqual(tuple(row["item"] for row in result["correspondence"]), PROJECTION.SPAN_NAMES)
        for row in result["correspondence"]:
            first, last = row["byte_offsets_half_open"]
            selected = (self.source / row["source"]).read_bytes()[first:last]
            self.assertEqual(PROJECTION.digest(selected), row["sha256"])
            self.assertEqual(actual.count(selected), 1, row["item"])
            self.assertEqual(selected in proof, row["bridge_type"], row["item"])
        for leaf in ("fraction_verus_bridge_spec.rs", "fraction_verus_bridge_lemmas.rs"):
            self.assertEqual(proof.count((self.source / "tools/formal" / leaf).read_bytes()), 1)
        self.assertNotIn(b"fn composition_rational_order(", proof)
        self.assertNotIn(b"fn add_fractions(", proof)

    def test_raw_independent_results_errors_and_native_derives_have_no_new_premise(self):
        self.prepare()
        proof = (self.output / "proof.rs").read_bytes()
        self.assertIn(b"// UNPROVED constructor-Post dependency source; full bridge acceptance remains open.\n", proof)
        self.assertIn(b"result: &Result<Fraction, FixtureEnclosureError>", proof)
        self.assertIn(b"Err(FixtureEnclosureError::Invariant) => !admitted,", proof)
        self.assertIn(b"Err(_) => false,", proof)
        self.assertIn(b"&& value.numerator == numerator", proof)
        self.assertIn(b"&& value.shift == if numerator == 0 { 0 } else { shift }", proof)
        for leaf in (b"Unrepresentable", b"Invariant", b"ZeroLabel", b"ReversedBounds", b"ValidationInvariant"):
            self.assertIn(leaf, proof)
        self.assertEqual(re.findall(
            rb"(?m)^#\[derive\(([^)]*)\)\]\n(?:pub )?(?:enum|struct) (IntervalError|FixtureEnclosureError|Fraction)\b",
            proof), [
                (b"Clone, Copy, Debug, Eq, PartialEq", b"IntervalError"),
                (b"Clone, Copy, Eq, PartialEq", b"FixtureEnclosureError"),
                (b"Clone, Copy", b"Fraction"),
            ])
        self.assertIn(b"use std::cmp::Ordering;", proof)
        self.assertNotRegex(proof, rb"(?:struct|enum) Ordering")
        self.assertNotRegex(re.sub(rb"(?m)//[^\n]*", b"", proof),
                            rb"\b(?:requires|assume|admit|external_body|external_fn_specification|assume_specification|axiom)\b")
        for bypass in (b"CompositionFractionSeed", b"composition_decode", b"Fraction::new(", b"#[kani::"):
            self.assertNotIn(bypass, proof)

    def test_inverse_rejects_post_body_signature_contract_and_hook_tampering(self):
        candidate = PROJECTION.annotated_constructor_post(self.fragment)
        for before, after in (
            (b"result: &Result<Fraction, FixtureEnclosureError>", b"result: &Result<Fraction, ()>"),
            (b"returns bridge_constructor_post(numerator, shift, result)", b"returns true"),
            (b"&& value.numerator == numerator", b"&& value.numerator <= numerator"),
            (b"if numerator == 0 { 0 } else { shift }", b"shift"),
            (b"Err(FixtureEnclosureError::Invariant) => !admitted,", b"Err(FixtureEnclosureError::Invariant) => true,"),
            (b"let bits = 64 - numerator.leading_zeros();", b"let bits = 0;"),
            (b"axiom_u64_leading_zeros(numerator);", b""),
            (b"lemma_bridge_bit_length_power(value.numerator, value.shift);",
             b"lemma_bridge_bit_length_power(value.numerator, 53);"),
        ):
            with self.subTest(mutation=after):
                self.assertIn(before, candidate)
                with self.assertRaises(ValueError):
                    PROJECTION.erase_constructor_post_annotations(candidate.replace(before, after, 1))
        for _, anchor, block in PROJECTION.BIT_LENGTH_HOOKS:
            for altered in (candidate.replace(block, b"", 1), candidate.replace(block, block + block, 1),
                            candidate.replace(block + anchor, anchor + block, 1)):
                with self.assertRaises(ValueError):
                    PROJECTION.erase_constructor_post_annotations(altered)

    def test_annotation_rejects_changed_truncated_or_extended_source_fragment(self):
        for altered in (self.fragment + b"\n", self.fragment[:-1],
                        self.fragment.replace(b"&& value.numerator == numerator", b"&& true", 1),
                        self.prefix):
            with self.assertRaisesRegex(ValueError, "three-predicate slice changed"):
                PROJECTION.annotated_constructor_post(altered)

    def test_source_product_prefix_ghost_lemma_and_projector_drift_create_no_output(self):
        for name, pattern in (
            ("crates/time-core/src/projection_enclosure.rs", "complete product source changed"),
            ("crates/time-core/src/selection.rs", "complete product source changed"),
            ("tools/formal/fraction_arithmetic_projection.py", "arithmetic projection changed"),
            ("tools/formal/fraction_composition_spec.rs", "composition specification changed"),
            ("tools/formal/fraction_verus_bridge_spec.rs", "ghost specification changed"),
            ("tools/formal/fraction_verus_bridge_lemmas.rs", "bit-length ghost lemma changed"),
            ("tools/formal/fraction_verus_bridge_projection.py", "differs from this executing source"),
        ):
            with self.subTest(source=name):
                path = self.source / name
                original = path.read_bytes()
                path.write_bytes(original + b"\n// unreviewed source\n")
                self.assert_rejected_without_output(pattern)
                path.write_bytes(original)

    def test_redirected_lemma_root_and_source_containment_are_rejected(self):
        path = self.source / "tools/formal/fraction_verus_bridge_lemmas.rs"
        original = path.read_bytes()
        target = self.base / "lemma.rs"
        target.write_bytes(original)
        path.unlink()
        path.symlink_to(target)
        self.assert_rejected_without_output("path redirects")
        path.unlink()
        path.write_bytes(original)
        alias = self.base / "source-alias"
        alias.symlink_to(self.source, target_is_directory=True)
        with self.assertRaisesRegex(ValueError, "path redirects"):
            PROJECTION.prepare_constructor_post(alias, self.output)
        with self.assertRaisesRegex(ValueError, "outside its source tree"):
            PROJECTION.prepare_constructor_post(self.source, self.source / "candidate")
        self.assertFalse((self.source / "candidate").exists())
        self.assertFalse(self.output.exists())

    def test_collision_and_output_symlink_preserve_existing_custody(self):
        self.output.mkdir()
        sentinel = self.output / "existing"
        sentinel.write_bytes(b"retain me")
        with self.assertRaisesRegex(ValueError, "fresh output leaf"):
            self.prepare()
        self.assertEqual(sentinel.read_bytes(), b"retain me")
        self.assertEqual([path.name for path in self.output.iterdir()], ["existing"])
        sentinel.unlink()
        self.output.rmdir()
        self.output.symlink_to(self.base / "missing")
        with self.assertRaisesRegex(ValueError, "path redirects"):
            self.prepare()
        self.assertFalse((self.base / "missing").exists())

    def test_original_and_bit_length_generated_rust_remain_exactly_pinned(self):
        self.prepare()
        actual = (self.output / "actual_source.rs").read_bytes()
        old_output, bit_output = self.base / "old", self.base / "bit-length"
        old = PROJECTION.prepare(self.source, old_output)
        bit = PROJECTION.prepare_bit_length(self.source, bit_output)
        self.assertEqual(PROJECTION.digest((old_output / "bridge.rs").read_bytes()),
                         "bcb1ca3d22a7e15b3948d65da34d1ecf6f34547153374591717948d4a279a4b5")
        self.assertEqual(PROJECTION.digest((bit_output / "proof.rs").read_bytes()),
                         "08c78bf2017c86bf62b7374db52a98b89b5dfacf7636970f0fa92adfa2206176")
        self.assertEqual((old_output / "actual_source.rs").read_bytes(), actual)
        self.assertEqual((bit_output / "actual_source.rs").read_bytes(), actual)
        self.assertEqual(len(old["source_files_sha256"]), 7)
        self.assertEqual(len(bit["source_files_sha256"]), 8)
        self.assertEqual(len(old["predicate_relations"]), 6)
        self.assertEqual(len(bit["predicate_relations"]), 2)

    def test_deterministic_closed_payload_and_no_execution_surface(self):
        first = self.prepare()
        other = self.base / "second-constructor-post"
        second = PROJECTION.prepare_constructor_post(self.source, other)
        self.assertEqual(first, second)
        self.assertEqual({path.name for path in self.output.iterdir()},
                         {"proof.rs", "actual_source.rs", "predicate_source.rs", "correspondence.json"})
        self.assertEqual(json.loads((self.output / "correspondence.json").read_text()), first)
        for name, expected in first["generated_sha256"].items():
            self.assertEqual(PROJECTION.digest((self.output / name).read_bytes()), expected)
            self.assertEqual((self.output / name).read_bytes(), (other / name).read_bytes())
        self.assertEqual(first["ghost_sha256"],
                         "fa9dc1f45f9e6fda44abab22d3a8cc4a61db4910c014a934585b4312e107bfa5")
        self.assertEqual(first["lemma_sha256"],
                         "b2fddd6592ff98ccd8fc450821d71f462966c8ce70e4322044f2df9d8acafce0")
        script = (HERE / "fraction_verus_bridge_projection.py").read_text()
        self.assertNotRegex(script, r"(?:^|\n)(?:from|import) (?:subprocess|os|importlib)\b")
        self.assertNotIn("if __name__", script)
        self.assertNotIn("--no-cheating", script)


if __name__ == "__main__":
    unittest.main()
