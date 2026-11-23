"""Exercise the actual strict source-report entry point on preview-owner copies."""

from contextlib import redirect_stdout
import importlib.util
import io
import json
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("preview_strict", ROOT / "scripts/check-strict.py")
STRICT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(STRICT)
SOURCE = "crates/nevrotick-ui/src/model.rs"
DRAFTS = "crates/nevrotick-ui/src/source_drafts.rs"
FIXTURE = "crates/nevrotick-ui/src/presentation.rs"
SLINT = "crates/nevrotick-ui/src/slint_host/source_workflow.rs"


class PreviewContractTests(unittest.TestCase):
    def setUp(self):
        self.enterContext(patch.object(sys, "path", [str(ROOT / "scripts"), *sys.path]))
        temporary = tempfile.TemporaryDirectory(prefix="preview-contract-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.source = self.root / SOURCE
        self.source.parent.mkdir(parents=True)
        self.original = (ROOT / SOURCE).read_text(encoding="utf-8")
        for path in (DRAFTS, FIXTURE, SLINT):
            target = self.root / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes((ROOT / path).read_bytes())

    def run_report(self, text):
        self.source.write_text(text, encoding="utf-8")
        output = io.StringIO()
        with (patch.object(STRICT, "ROOT", self.root),
              patch.object(STRICT.ENV, "inspect", side_effect=AssertionError("source report ran tool admission")),
              redirect_stdout(output)):
            code = STRICT.main(["--preview-contract"])
        return code, json.loads(output.getvalue())

    def test_actual_model_and_comment_string_decoys_pass(self):
        for text in (self.original, self.original.replace(
            "#[derive(Validate)]", '/* nested /* garde(allow_unvalidated) */ comment */\n'
            "#[derive(Validate)]", 1)):
            with self.subTest(decoys=text != self.original):
                code, report = self.run_report(text)
                self.assertEqual(code, 0)
                self.assertEqual(report["findings"], [])
                self.assertEqual(report["status"], "passed")
        text = self.original.replace('"Prototype — sample data"',
                                     '"garde(allow_unvalidated) struct PreviewInput letter r"')
        code, report = self.run_report(text)
        self.assertEqual(code, 1)
        self.assertEqual([row["rule"] for row in report["findings"]], ["CR-03"])

    def test_seeded_field_policy_changes_fail_with_intended_rules(self):
        for replacement in ("", "#[garde(skip)]", '#[garde(rename("label"))]',
                            "#[garde(ascii)]", "#[cfg_attr(test, garde(custom(label_rule)))]"):
            with self.subTest(replacement=replacement):
                code, report = self.run_report(
                    self.original.replace("#[garde(custom(label_rule))]", replacement, 1))
                self.assertEqual(code, 1)
                self.assertEqual(report["status"], "findings")
                self.assertEqual([row["rule"] for row in report["findings"]],
                                 ["preview-input-field-rules", "CR-03"])
                self.assertEqual(report["findings"][0]["path"], SOURCE)

    def test_forbidden_escape_has_its_own_finding(self):
        code, report = self.run_report(self.original.replace(
            "#[derive(Validate)]", "#[cfg_attr(test, garde(allow_unvalidated))]\n#[derive(Validate)]", 1))
        self.assertEqual(code, 1)
        self.assertEqual([row["rule"] for row in report["findings"]],
                         ["preview-no-allow-unvalidated", "preview-input-admission", "CR-03"])

    def test_scoped_visibility_and_conditional_derive_fail(self):
        for before, after, rule in (
            ("label: &'a str,", "pub(crate) label: &'a str,", "preview-input-field-rules"),
            ("#[derive(Validate)]", "#[cfg(any(test, windows))]\n#[derive(Validate)]", "preview-input-admission"),
        ):
            with self.subTest(after=after):
                code, report = self.run_report(self.original.replace(before, after, 1))
                self.assertEqual(code, 1)
                self.assertEqual([row["rule"] for row in report["findings"]], [rule, "CR-03"])

    def test_missing_owner_and_unsupported_syntax_are_incomplete(self):
        for text in (self.original.replace("struct PreviewInput", "struct Unselected", 1),
                     self.original + '\nconst DECOY: &str = r#"unsupported"#;\n'):
            code, report = self.run_report(text)
            self.assertEqual(code, 1)
            self.assertEqual(report["status"], "incomplete")

    def test_nested_declaration_does_not_stand_in_for_module_owner(self):
        changed = self.original.replace("#[derive(Validate)]",
                                        "mod alternate {\n#[derive(Validate)]", 1)
        changed = changed.replace("port: &'a str,\n}", "port: &'a str,\n}\n}", 1)
        code, report = self.run_report(changed)
        self.assertEqual(code, 1)
        self.assertEqual([row["rule"] for row in report["findings"]], ["preview-input-admission", "CR-03"])

    def test_actual_constructor_omission_and_bound_movement(self):
        start = self.original.index("        let input = PreviewInput")
        before_port = self.original.index("        let port = port.parse", start)
        no_validation = self.original[:start] + self.original[before_port:]
        first_bound = self.original.index("        if label.len()")
        moved_bounds = (self.original[:first_bound] + self.original[start:before_port]
                        + self.original[first_bound:start] + self.original[before_port:])
        for text in (no_validation, moved_bounds):
            with self.subTest(omission=text == no_validation):
                code, report = self.run_report(text)
                self.assertEqual(code, 1)
                self.assertEqual([(f["rule"], f["occurrence"]) for f in report["findings"]],
                                 [("CR-03", "SourcePreview::parse/body")])

    def test_live_write_before_admission_is_reported(self):
        checked = "        let replacement = SourcePreview::parse(label, host, port)?;"
        changed = self.original.replace(checked, "        self.selected = 1;\n" + checked)
        code, report = self.run_report(changed)
        self.assertEqual(code, 1)
        self.assertEqual([(f["rule"], f["occurrence"]) for f in report["findings"]],
                         [("CR-03", "PreviewModel::update_selected/body")])

    def test_scoped_output_field_and_new_unchecked_setter_are_reported(self):
        for changed in (
            self.original.replace("    port: u16,", "    pub(crate) port: u16,", 1),
            self.original.replace("impl SourcePreview {", "impl SourcePreview {\n"
                                  "    pub fn set_label(&mut self, raw: String) { self.label = raw; }", 1),
            self.original + "\nimpl From<String> for SourcePreview {\n"
                            "    fn from(label: String) -> Self { Self { label, host: String::new(), port: 0 } }\n}\n",
        ):
            with self.subTest(changed=changed):
                code, report = self.run_report(changed)
                self.assertEqual(code, 1)
                self.assertEqual([(f["rule"], f["occurrence"]) for f in report["findings"]],
                                 [("CR-03", "model admission surface")])

    def test_each_consumer_seam_is_registered_and_checks_its_body(self):
        for path, old, new, symbol in (
            (FIXTURE, ".update_selected(label, host, port)", ".update_selected(host, label, port)",
             "UiSession::update_preview"),
            (DRAFTS, "SourcePreview::parse(raw.label, host, raw.port)", 'SourcePreview::parse("Fixture", host, raw.port)',
             "DraftSource::parse"),
            (DRAFTS, "        if raw.len() > MAX_SOURCES {\n"
                     "            return Err(SourceDraftError::Capacity);\n"
                     "        }\n", "", "SourceDrafts::from_baseline"),
            (DRAFTS, "let candidate = DraftSource::parse(id, raw)?;",
             'let candidate = DraftSource::parse(id, raw)?; self.changes.clear();', "SourceDrafts::add"),
            (DRAFTS, "let candidate = DraftSource::parse(id, raw)?;",
             'let candidate = DraftSource::parse(id, raw)?; self.changes.clear();', "SourceDrafts::edit"),
            (DRAFTS, "fn replace_change(", "pub fn replace_change(", "SourceDrafts::replace_change"),
            (SLINT, "window.on_update_requested(", "window.on_cancel_requested(",
             "install_source_editor_callbacks"),
            (SLINT, "owner.editor = EditorTarget::Closed;", "owner.selected = None;",
             "update_preview"),
        ):
            with self.subTest(symbol=symbol):
                file = self.root / path
                original = (ROOT / path).read_text(encoding="utf-8")
                # The add/edit and Slint write snippets recur in the source.
                # Restrict seeding to the named production body.
                start = original.index(f"fn {symbol.split('::')[-1]}(")
                updated = original[:start] + original[start:].replace(old, new, 1)
                self.assertNotEqual(updated, original)
                file.write_text(updated, encoding="utf-8")
                code, report = self.run_report(self.original)
                file.write_text(original, encoding="utf-8")
                self.assertEqual((code, [(f["rule"], f["occurrence"]) for f in report["findings"]]),
                                 (1, [("CR-01", symbol)]))
                if symbol == "SourceDrafts::from_baseline":
                    self.assertEqual(report["findings"][0]["review_with"],
                                     ["DraftSource::parse", "SourceDrafts::fixture", "RecoveryPreview::parse"])

    def test_interface_change_and_supported_adjacent_exception(self):
        path = self.root / FIXTURE
        original = path.read_text(encoding="utf-8")
        anchor = "    /// Borrow raw callback fields"
        changed = original.replace(anchor, "    /// Forward raw callback fields", 1)
        path.write_text(changed, encoding="utf-8")
        code, report = self.run_report(self.original)
        self.assertEqual(code, 1)
        finding, = report["findings"]
        self.assertEqual(finding["rule"], "CR-01")
        self.assertEqual(list(finding["changes"]), ["interface"])
        marker = (
            f'    // preview-contract-expect CR-01 UiSession::update_preview {finding["context"]} {finding["binding"]}'
            " | rationale=Editorial verb change keeps immutable forwarding"
            " | owner=UiSession::update_preview | consumer=PreviewModel::update_selected"
            " | evidence=fixture forwarding source inspection"
            " | recheck=when forwarding or model admission changes\n"
        )
        new_anchor = "    /// Forward raw callback fields"
        for index, (variant, expected) in enumerate((
            (changed.replace(new_anchor, marker + new_anchor), 0),
            (changed.replace(new_anchor, marker + "\n" + new_anchor), 1),
            (changed.replace(new_anchor, marker.replace("Editorial verb change keeps immutable forwarding", "safe")
                             + new_anchor), 1),
            (changed.replace(new_anchor, marker.replace("CR-01", "CR-03") + new_anchor), 1),
            (changed.replace(new_anchor, marker.replace("owner=UiSession::update_preview", "owner=ui dev")
                             + new_anchor), 1),
            (changed.replace(new_anchor, marker.replace("owner=UiSession::update_preview",
                                                        "owner=SourceDrafts::edit") + new_anchor), 1),
            (changed.replace(new_anchor, marker.replace("consumer=PreviewModel::update_selected",
                                                        "consumer=SourceDrafts::edit") + new_anchor), 1),
            (changed.replace(new_anchor, marker + marker + new_anchor), 1),
            (original.replace(anchor, marker + anchor), 1),
        )):
            with self.subTest(variant=index):
                path.write_text(variant, encoding="utf-8")
                status, report = self.run_report(self.original)
                self.assertEqual(status, expected)
                if expected == 0:
                    self.assertIn("exception", report["findings"][0])
                else:
                    self.assertIn("invalid-expectation", [row["rule"] for row in report["findings"]])
        path.write_text(changed.replace(new_anchor, marker + new_anchor), encoding="utf-8")
        changed_model = self.original.replace("Validate all raw fields", "Check all raw fields", 1)
        code, report = self.run_report(changed_model)
        self.assertEqual(code, 1)
        self.assertIn("invalid-expectation", [row["rule"] for row in report["findings"]])

        # A complete metadata record does not certify its claim. The recorded
        # semantic review would reject this specimen: host and label are swapped.
        wrong_fields = changed.replace(".update_selected(label, host, port)",
                                       ".update_selected(host, label, port)")
        path.write_text(wrong_fields, encoding="utf-8")
        code, report = self.run_report(self.original)
        self.assertEqual(code, 1)
        self.assertEqual(list(report["findings"][0]["changes"]), ["interface", "body"])
        false_marker = marker.replace(finding["binding"], report["findings"][0]["binding"])
        path.write_text(wrong_fields.replace(new_anchor, false_marker + new_anchor), encoding="utf-8")
        code, report = self.run_report(self.original)
        self.assertEqual(code, 0)
        self.assertIn("exception", report["findings"][0])

    def test_model_body_and_interface_expectations_are_independently_attached(self):
        old_doc = "    /// Atomically replace only the selected preview after mandatory validation."
        new_doc = old_doc.replace("replace", "update")
        changed = self.original.replace(old_doc, new_doc).replace(
            "let replacement = SourcePreview::parse", "let checked = SourcePreview::parse"
        ).replace("self.sources[self.selected] = replacement", "self.sources[self.selected] = checked")
        code, report = self.run_report(changed)
        self.assertEqual(code, 1)
        body, interface = report["findings"]
        self.assertEqual((body["rule"], body["occurrence"]),
                         ("CR-03", "PreviewModel::update_selected/body"))
        self.assertEqual((interface["rule"], interface["occurrence"]),
                         ("CR-01", "PreviewModel::update_selected"))
        self.assertEqual(body.get("owner"), "PreviewModel::update_selected")
        self.assertEqual(interface.get("owner"), body["owner"])
        self.assertIn("UiSession::update_preview", body["review_with"])
        details = (
            " | rationale=Local name and wording preserve checked replacement"
            " | owner=PreviewModel::update_selected | consumer=UiSession::update_preview"
            " | evidence=actual checked construction and commit inspection"
            " | recheck=when construction or commit moves\n"
        )
        body_marker = (
            f'        // preview-contract-expect CR-03 {body["occurrence"]} {body["context"]} {body["binding"]}'
            + details
        )
        interface_marker = (
            f'    // preview-contract-expect CR-01 {interface["occurrence"]} {interface["context"]} {interface["binding"]}'
            + details
        )
        anchor = "        let checked = SourcePreview::parse"
        both = changed.replace(new_doc, interface_marker + new_doc).replace(anchor, body_marker + anchor)
        code, report = self.run_report(both)
        self.assertEqual(code, 0)
        self.assertEqual([row["rule"] for row in report["findings"]], ["CR-03", "CR-01"])
        self.assertTrue(all(row["exception"]["owner"] == body["owner"] for row in report["findings"]))
        for condition, variant in (
            ("gap", both.replace(body_marker, "\n" + body_marker, 1)),
            ("duplicate", both.replace(body_marker, body_marker + body_marker, 1)),
            ("unrelated consumer", both.replace(body_marker, body_marker.replace(
                "consumer=UiSession::update_preview", "consumer=SourceDrafts::edit"), 1)),
            ("wrong placement", both.replace(body_marker, "").replace(new_doc, body_marker + new_doc)),
            ("stale digest", both.replace("= checked;", "= checked.clone();")),
        ):
            with self.subTest(condition=condition):
                code, report = self.run_report(variant)
                self.assertEqual(code, 1)
                self.assertIn("invalid-expectation", [row["rule"] for row in report["findings"]])

    def test_constructor_body_expectation_retains_raw_finding_and_hard_policy(self):
        anchor = "        // Bound work before garde"
        changed = self.original.replace("let input = PreviewInput", "let candidate = PreviewInput").replace(
            "input.validate()", "candidate.validate()")
        code, report = self.run_report(changed)
        finding, = report["findings"]
        self.assertEqual((code, finding["rule"], finding["occurrence"]),
                         (1, "CR-03", "SourcePreview::parse/body"))
        self.assertIn("binding", finding)
        details = (
            " | rationale=Binding name preserves same fields and validation"
            " | owner=SourcePreview::parse | consumer=DraftSource::parse"
            " | evidence=actual shared preview construction inspection"
            " | recheck=when validation or forwarding changes\n"
        )
        marker = (
            f'        // preview-contract-expect CR-03 {finding["occurrence"]} {finding["context"]} {finding["binding"]}'
            + details
        )
        closed = changed.replace(anchor, marker + anchor)
        code, report = self.run_report(closed)
        self.assertEqual(code, 0)
        self.assertIn("exception", report["findings"][0])
        bad = closed.replace("#[derive(Validate)]", "#[garde(allow_unvalidated)]\n#[derive(Validate)]")
        code, report = self.run_report(bad)
        self.assertEqual(code, 1)
        hard, = [row for row in report["findings"] if row["rule"] == "preview-no-allow-unvalidated"]
        self.assertNotIn("binding", hard)
        self.assertNotIn("exception", hard)
        surface, = [row for row in report["findings"] if row["occurrence"] == "model admission surface"]
        self.assertNotIn("binding", surface)

    def test_free_function_expectation_uses_its_module_qualified_owner(self):
        path = self.root / SLINT
        old_doc = "/// Admit every raw source field using the Rust-owned Add/Edit target only."
        new_doc = old_doc.replace("Admit", "Check")
        changed = path.read_text(encoding="utf-8").replace(old_doc, new_doc)
        path.write_text(changed, encoding="utf-8")
        code, report = self.run_report(self.original)
        finding, = report["findings"]
        self.assertEqual((code, finding["occurrence"], finding.get("owner")),
                         (1, "update_preview", "source_workflow::update_preview"))
        marker = (
            f'// preview-contract-expect CR-01 {finding["occurrence"]} {finding["context"]} {finding["binding"]}'
            " | rationale=Editorial verb change keeps checked Add/Edit routing"
            " | owner=source_workflow::update_preview | consumer=SourceDrafts::edit"
            " | evidence=fixture checked Add/Edit source inspection"
            " | recheck=when Add/Edit admission or routing changes\n"
        )
        path.write_text(changed.replace(new_doc, marker + new_doc), encoding="utf-8")
        code, report = self.run_report(self.original)
        self.assertEqual(code, 0)
        self.assertIn("exception", report["findings"][0])

    def test_consumer_impl_cfg_change_is_reported(self):
        path = self.root / FIXTURE
        path.write_text(path.read_text().replace("impl UiSession {", "#[cfg(test)]\nimpl UiSession {"),
                        encoding="utf-8")
        code, report = self.run_report(self.original)
        self.assertEqual(code, 1)
        self.assertEqual([f["occurrence"] for f in report["findings"]], ["UiSession::update_preview"])

    def test_inner_outcome_suppression_cannot_hide_outside_a_selected_method(self):
        carrier = self.root / FIXTURE
        original = carrier.read_text(encoding="utf-8")
        for attribute, expected in (
            ("#![allow(unused_must_use, reason = \"synthetic control\")]", "unused_must_use"),
            ("#![allow(clippy::default, reason = \"synthetic control\")]", "clippy::default"),
            ("#![allow(clippy::suspicious, reason = \"synthetic control\")]", "clippy::suspicious"),
            ("#![allow(let_underscore, reason = \"synthetic control\")]", "let_underscore"),
            ("#![cfg_attr(test, cfg_attr(unix, allow(clippy::default)))]", "clippy::default"),
            ("#![cfg_attr(test, cfg_attr(unix, allow(clippy::suspicious)))]", "clippy::suspicious"),
            ("#![cfg_attr(test, cfg_attr(unix, allow(let_underscore)))]", "let_underscore"),
            ("#![allow(renamed_and_removed_lints, clippy_all)]", "clippy_all"),
            ("#![allow(renamed_and_removed_lints, clippy_suspicious)]", "clippy_suspicious"),
            ("#![allow(renamed_and_removed_lints, clippy_restriction)]", "clippy_restriction"),
            ("#![cfg_attr(test, cfg_attr(unix, expect(\n"
             "    r#unused, clippy /* nested /* comment */ */ :: unused_result_ok,\n"
             '    reason = "synthetic control, with punctuation"\n)))]', "clippy::unused_result_ok, unused"),
            ("#![warn(warnings)]", "warnings"),
            ("#![cfg_attr(test, warn(clippy::all, clippy::let_underscore_future))]",
             "clippy::all, clippy::let_underscore_future"),
            ('#![doc = "#![allow(unused_must_use)]"]', None),
            ("/* #![allow(unused_must_use)] */", None),
            ("#![allow(unused_imports)]", None),
            ('#![allow(unused_imports, reason = "context,unused_must_use, continuation")]', None),
            ("#![cfg_attr(any(), inert(allow(unused_must_use)))]", None),
        ):
            with self.subTest(attribute=attribute):
                carrier.write_text(attribute + "\n" + original, encoding="utf-8")
                code, report = self.run_report(self.original)
                self.assertEqual(code, int(expected is not None))
                self.assertEqual([(row["rule"], row["occurrence"]) for row in report["findings"]],
                                 [("preview-inner-outcome-suppression", expected)] if expected else [])

    def test_scanner_distinguishes_typed_skip_from_forbidden_option(self):
        # Use the real Core declaration as a lexical specimen outside the
        # selected model surface. Its owning Core code remains untouched.
        core = (ROOT / "crates/time-core/src/conditioned_window.rs").read_text(encoding="utf-8")
        start = core.index("struct RowCandidate {")
        end = core.index("\n}", start) + len("\n}")
        row = core[start:end]
        typed_skip = "#[garde(skip)]\n    observation_key: NonZeroU64,"
        self.assertIn(typed_skip, row)
        carrier = self.root / DRAFTS
        original = carrier.read_text(encoding="utf-8")
        for variant, expected in (
            (row, []),
            (row.replace(typed_skip, typed_skip.replace("skip", "allow_unvalidated"), 1),
             ["preview-no-allow-unvalidated"]),
        ):
            carrier.write_text(original + "\n\n" + variant + "\n", encoding="utf-8")
            code, report = self.run_report(self.original)
            self.assertEqual((code, [f["rule"] for f in report["findings"]]),
                             (int(bool(expected)), expected))

    def test_missing_report_rule_and_consumer_are_incomplete(self):
        from preview_contract import SEAMS
        with patch("preview_contract.SEAMS", SEAMS[:-1]):
            code, report = self.run_report(self.original)
            self.assertEqual((code, report["status"]), (1, "incomplete"))
        (self.root / FIXTURE).unlink()
        code, report = self.run_report(self.original)
        self.assertEqual((code, report["status"]), (1, "incomplete"))
        with patch.dict(sys.modules, {"preview_contract": None}):
            code, report = self.run_report(self.original)
            self.assertEqual((code, report["status"]), (1, "incomplete"))


if __name__ == "__main__":
    unittest.main()
