"""Check metadata mutation selection without installing or running the mutator."""

from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest


ROOT = Path(__file__).resolve().parents[2]
DRIVER = ROOT / "metadata/qualification/mutation.py"
SPEC = importlib.util.spec_from_file_location("metadata_mutation", DRIVER)
assert SPEC is not None and SPEC.loader is not None
MUTATION = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MUTATION)

FILE = "metadata/pipeline/generate.py"
LATE = "test_sample.SampleTests.test_late"
EARLY = "test_sample.SampleTests.test_early"
SOURCES = {name: b"\n" * 20 for name in MUTATION.CRITICAL_FILES}


class MutationContextsTests(unittest.TestCase):
    def setUp(self) -> None:
        workspace = tempfile.TemporaryDirectory(prefix="metadata-mutation-context-")
        self.addCleanup(workspace.cleanup)
        directory = Path(workspace.name)
        self.path = directory / "contexts.json"
        (directory / "test-result.json").write_text(
            json.dumps({"test_durations": {EARLY: 1.0, LATE: 2.0}}), encoding="utf-8"
        )

    def load(self, raw: object) -> dict:
        self.path.write_text(json.dumps(raw), encoding="utf-8")
        return MUTATION.load_contexts(self.path, SOURCES)

    def select(self, contexts: dict, line: int, functions: list[dict] | None = None) -> tuple:
        mutation = {"file": FILE, "start": [line, 0], "end": [line, 1]}
        return MUTATION.context_selection(mutation, contexts, {FILE: functions or []})

    def test_current_report_selects_exact_line_then_containing_function(self) -> None:
        contexts = self.load({LATE: {FILE: [3]}, "": {FILE: [2, 3]}, EARLY: {FILE: [3, 4]}})
        self.assertEqual(self.select(contexts, 3), ([EARLY, LATE], "measured-mutated-line"))
        self.assertEqual(
            self.select(contexts, 5, [{"start": 4, "end": 6}]),
            ([EARLY], "measured-containing-function"),
        )

    def test_field_registry_dispatches_to_its_actual_consumers(self) -> None:
        fields = "metadata/pipeline/fields.py"
        store = {
            "file": fields, "start": [11, 0], "end": [11, 1], "definition": "retained_store_field",
        }
        self.assertEqual(MUTATION.context_selection(store, {}, {fields: []}), (
            ["test_field_coverage.ExpandedFieldTests.test_store_history_is_not_an_active_source_requirement"],
            "independent-store-history-delivery",
        ))
        duplicate = {"file": fields, "start": [5, 0], "end": [5, 1], "definition": ""}
        tests, _ = MUTATION.context_selection(duplicate, {}, {fields: []})
        self.assertIn("test_field_coverage.ExpandedModelTests.test_registry_rejects_duplicate_keys_at_import", tests)
        self.assertIn(
            "test_field_coverage.ExpandedModelTests.test_complete_field_register_has_reviewed_shape_values_and_catalogue_binding",
            tests,
        )

    def test_preflight_snapshot_is_the_observation_consumed(self) -> None:
        original = json.dumps({EARLY: {FILE: [3]}}).encode()
        self.path.write_text(json.dumps({LATE: {FILE: [3]}}), encoding="utf-8")
        contexts = MUTATION.load_contexts(self.path, SOURCES, report_bytes=original)
        self.assertEqual(self.select(contexts, 3), ([EARLY], "measured-mutated-line"))

    def test_legacy_file_map_remains_usable(self) -> None:
        for observation in ({"contexts": {"3": [LATE]}}, {"3": [LATE]}):
            with self.subTest(observation=observation):
                contexts = self.load({"files": {FILE: observation}})
                self.assertEqual(self.select(contexts, 3), ([LATE], "measured-mutated-line"))

    def test_unmeasured_and_global_only_lines_use_the_complete_suite(self) -> None:
        without_report = MUTATION.load_contexts(None, SOURCES)
        global_only = self.load({"": {FILE: [2]}})
        for contexts, line in ((without_report, 2), (global_only, 2), (global_only, 7)):
            with self.subTest(line=line):
                self.assertEqual(self.select(contexts, line),
                                 (None, "complete-suite-unmeasured-or-module-level-scope"))

    def test_malformed_supplied_reports_fail_instead_of_using_the_complete_suite(self) -> None:
        malformed = ({}, [], {LATE: []}, {LATE: {}}, {LATE: {"other.py": [3]}}, {LATE: {FILE: []}},
                     {LATE: {FILE: [True]}}, {LATE: {FILE: [0]}}, {LATE: {FILE: [21]}},
                     {LATE: {FILE: ["3"]}}, {"files": {}}, {"files": {FILE: {"no-line": [LATE]}}},
                     {"files": {FILE: {"3": [None]}}})
        for raw in malformed:
            with self.subTest(raw=raw), self.assertRaisesRegex(ValueError, "coverage context"):
                self.load(raw)

    def test_duplicate_context_key_is_rejected(self) -> None:
        self.path.write_text(f'{{"{LATE}": {{"{FILE}": [3]}}, "{LATE}": {{"{FILE}": [4]}}}}',
                             encoding="utf-8")
        with self.assertRaisesRegex(ValueError, "coverage context"):
            MUTATION.load_contexts(self.path, SOURCES)


if __name__ == "__main__":
    unittest.main()
