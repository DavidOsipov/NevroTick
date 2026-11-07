"""Discriminating controls for false-green qualification and unchecked source seams."""
from importlib.util import module_from_spec, spec_from_file_location
from pathlib import Path
import unittest

spec = spec_from_file_location("metadata_typing_qualification", Path(__file__).with_name("typing.py"))
assert spec is not None and spec.loader is not None
qualification = module_from_spec(spec)
spec.loader.exec_module(qualification)


class TypingGateTests(unittest.TestCase):
    def test_healthy_requires_exact_nonempty_checked_closure(self) -> None:
        self.assertTrue(qualification.qualifies({"exit_code": 0, "stdout": "Success: no issues found in 9 source files\n"}, "typing_consumer.py", None))
        for output in ("", "Success: no issues found in 0 source files", "Found 0 errors", "Success: no issues found in 8 source files"):
            with self.subTest(output=output):
                self.assertFalse(qualification.qualifies({"exit_code": 0, "stdout": output}, "typing_consumer.py", None))

    def test_fault_requires_intended_rule_and_actual_exit(self) -> None:
        intended = "typing_wrong_value.py:4: error: Incompatible types [typeddict-item]\n"
        self.assertTrue(qualification.qualifies({"exit_code": 1, "stdout": intended}, "typing_wrong_value.py", "typeddict-item"))
        for code, output in ((0, intended), (2, intended), (1, ""), (1, "unresolved import [import-not-found]"),
                             (1, intended + "metadata/pipeline/contract.py:2: error: unrelated [misc]")):
            with self.subTest(code=code, output=output):
                self.assertFalse(qualification.qualifies({"exit_code": code, "stdout": output}, "typing_wrong_value.py", "typeddict-item"))

    def test_any_cast_ignore_config_and_import_escapes_are_refused(self) -> None:
        for source in ("from typing import Any", "from typing import cast as unchecked", "import typing\nx = typing.Any", "x=1 # type: ignore", "# mypy: ignore-errors", "import third_party", "from ..pipeline import fields"):
            with self.subTest(source=source):
                with self.assertRaises(qualification.Incomplete):
                    qualification.inspect_source("consumer.py", source.encode())
        self.assertEqual(qualification.inspect_source("consumer.py", b"from pathlib import Path\nx: Path = Path('.')\n"), {"pathlib"})

    def test_tool_admission_rejects_changed_installed_bytes(self) -> None:
        checker = Path("/external/checker")
        policy = {"python_version": "3.12.3", "mypy_version": "2.4.1", "wheels": {"tool": "pin"}}
        files = {"bin/python": "known-hash"}
        admission = {"schema_version": 1, "checker_root": str(checker), "files": files,
                     "python_version": "3.12.3", "mypy_version": "2.4.1", "wheels": policy["wheels"],
                     "policy_sha256": "policy-hash"}
        qualification.admit_tool(admission, checker, files, policy, "policy-hash")
        with self.assertRaises(qualification.Incomplete):
            qualification.admit_tool(admission, checker, {"bin/python": "changed-hash"}, policy, "policy-hash")
        with self.assertRaises(qualification.Incomplete):
            qualification.admit_tool(admission, checker, files, policy, "changed-policy")

    def test_persistent_strict_configuration_matches_closed_gate(self) -> None:
        self.assertEqual((qualification.ROOT / qualification.CONFIG).read_bytes(), qualification.EXPECTED_CONFIG.encode())
        self.assertEqual(len(qualification.SOURCES), 8)
        for source in qualification.SOURCES:
            qualification.inspect_source(source, (qualification.ROOT / source).read_bytes())


if __name__ == "__main__":
    unittest.main(verbosity=2)
