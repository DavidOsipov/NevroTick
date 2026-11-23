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


class PreviewContractTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="preview-contract-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.source = self.root / SOURCE
        self.source.parent.mkdir(parents=True)
        self.original = (ROOT / SOURCE).read_text(encoding="utf-8")

    def run_report(self, text):
        self.source.write_text(text, encoding="utf-8")
        output = io.StringIO()
        with (patch.object(STRICT, "ROOT", self.root),
              patch.object(sys, "path", [str(ROOT / "scripts"), *sys.path]),
              patch.object(STRICT.ENV, "inspect", side_effect=AssertionError("source report ran tool admission")),
              redirect_stdout(output)):
            code = STRICT.main(["--preview-contract"])
        return code, json.loads(output.getvalue())

    def test_actual_model_and_comment_string_decoys_pass(self):
        for text in (self.original, self.original.replace(
            "#[derive(Validate)]", '/* nested /* garde(allow_unvalidated) */ comment */\n'
            "#[derive(Validate)]", 1).replace(
                '"Prototype — sample data"', '"garde(allow_unvalidated) struct PreviewInput letter r"')):
            with self.subTest(decoys=text != self.original):
                code, report = self.run_report(text)
                self.assertEqual(code, 0)
                self.assertEqual(report["findings"], [])
                self.assertEqual(report["status"], "passed")

    def test_seeded_field_policy_changes_fail_with_intended_rules(self):
        for replacement in ("", "#[garde(skip)]", '#[garde(rename("label"))]',
                            "#[garde(ascii)]", "#[cfg_attr(test, garde(custom(label_rule)))]"):
            with self.subTest(replacement=replacement):
                code, report = self.run_report(
                    self.original.replace("#[garde(custom(label_rule))]", replacement, 1))
                self.assertEqual(code, 1)
                self.assertEqual(report["status"], "findings")
                self.assertEqual([row["rule"] for row in report["findings"]], ["preview-input-field-rules"])
                self.assertEqual(report["findings"][0]["path"], SOURCE)

    def test_forbidden_escape_has_its_own_finding(self):
        code, report = self.run_report(self.original.replace(
            "#[derive(Validate)]", "#[cfg_attr(test, garde(allow_unvalidated))]\n#[derive(Validate)]", 1))
        self.assertEqual(code, 1)
        self.assertEqual([row["rule"] for row in report["findings"]],
                         ["preview-no-allow-unvalidated", "preview-input-admission"])

    def test_scoped_visibility_and_conditional_derive_fail(self):
        for before, after, rule in (
            ("label: &'a str,", "pub(crate) label: &'a str,", "preview-input-field-rules"),
            ("#[derive(Validate)]", "#[cfg(any(test, windows))]\n#[derive(Validate)]", "preview-input-admission"),
        ):
            with self.subTest(after=after):
                code, report = self.run_report(self.original.replace(before, after, 1))
                self.assertEqual(code, 1)
                self.assertEqual([row["rule"] for row in report["findings"]], [rule])

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
        self.assertEqual([row["rule"] for row in report["findings"]], ["preview-input-admission"])


if __name__ == "__main__":
    unittest.main()
