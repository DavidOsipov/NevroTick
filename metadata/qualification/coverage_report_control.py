"""Exercise the coverage report CLI with the pinned coverage.py environment."""

from __future__ import annotations

from collections.abc import Callable
import json
from pathlib import Path
import runpy
import subprocess
import sys
from tempfile import TemporaryDirectory
from typing import Any
import unittest
from unittest.mock import patch

from coverage import Coverage
from coverage import CoverageData


REPORTER: Path = Path(__file__).with_name("coverage_report.py").resolve()


class CoverageReportTests(unittest.TestCase):
    def test_file_contexts_are_retained_once(self) -> None:
        directory: str
        with TemporaryDirectory() as directory:
            root: Path = Path(directory)
            output: Path = root / "output"
            output.mkdir()
            qualification: Path = root / "metadata/qualification"
            qualification.mkdir(parents=True)
            name: str = "metadata/decision.py"
            marker: str = "metadata/__init__.py"
            (qualification / "manifest.json").write_text(json.dumps({"source_modules": [name, marker]}))
            (root / marker).write_text('"""A non-executing package marker."""\n')
            (root / name).write_text("""\
class Decision:
    def choose(self, allowed):
        if allowed:
            return 1
        return 0

def is_allowed(allowed):
    if allowed:
        return True
    return False
""")
            config: Path = output / ".coveragerc"
            config.write_text(f"[run]\nbranch = true\ndata_file = {output / '.coverage'}\n")
            measured: Coverage = Coverage(config_file=str(config))
            measured.start()
            try:
                measured.switch_context("module-import")
                decisions: dict[str, Any] = runpy.run_path(str(root / name))
                measured.switch_context("case-denied")
                self.assertEqual(decisions["Decision"]().choose(False), 0)
                measured.switch_context("case-allowed")
                self.assertEqual(decisions["Decision"]().choose(True), 1)
                self.assertTrue(decisions["is_allowed"](True))
            finally:
                measured.stop()
                measured.save()

            subprocess.run([sys.executable, "-I", "-B", str(REPORTER), str(output), str(root)],
                           cwd=root, check=True, timeout=15)
            report: dict[str, Any] = json.loads((output / "coverage-raw.json").read_text())
            self.assertEqual(set(report["files"]), {name, marker})
            self.assertTrue(report["meta"]["show_contexts"])
            row: dict[str, Any] = report["files"][name]
            self.assertEqual(row["summary"]["num_statements"], 9)
            self.assertEqual(row["summary"]["covered_lines"], 8)
            self.assertEqual(row["missing_lines"], [10])
            self.assertEqual(row["missing_branches"], [[8, 10]])
            self.assertEqual({line: sorted(contexts) for line, contexts in row["contexts"].items()}, {
                "1": ["module-import"], "2": ["module-import"], "3": ["case-allowed", "case-denied"],
                "4": ["case-allowed"], "5": ["case-denied"], "7": ["module-import"],
                "8": ["case-allowed"], "9": ["case-allowed"],
            })
            self.assertEqual(report["files"][marker]["contexts"], {})
            self.assertIn("Decision.choose", row["functions"])
            self.assertIn("Decision", row["classes"])
            for kind in ("functions", "classes"):
                for region in row[kind].values():
                    self.assertNotIn("contexts", region)


class CoverageReportRefusalTests(unittest.TestCase):
    def setUp(self) -> None:
        self.root: Path = Path(self.enterContext(TemporaryDirectory()))
        self.output: Path = self.root / "output"
        self.output.mkdir()
        self.name: str = "metadata/decision.py"
        self.marker: str = "metadata/__init__.py"
        self.absolute: str = str(self.root / self.name)
        qualification: Path = self.root / "metadata/qualification"
        qualification.mkdir(parents=True)
        (qualification / "manifest.json").write_text(
            json.dumps({"source_modules": [self.name, self.marker]}), encoding="utf-8"
        )
        (self.root / self.name).write_text("decision = 1\n", encoding="utf-8")
        (self.root / self.marker).write_text('"""Package marker."""\n', encoding="utf-8")
        self.data_file: Path = self.output / ".coverage"
        (self.output / ".coveragerc").write_text(
            f"[run]\ndata_file = {self.data_file}\n", encoding="utf-8"
        )
        self.published: Path = self.output / "coverage-raw.json"
        self.previous: bytes = b'{"meta": {"previous": true}, "files": {}}\n'
        self.published.write_bytes(self.previous)
        self.enterContext(patch.object(sys, "argv", [str(REPORTER), str(self.output), str(self.root)]))
        self.report: Callable[[], int] = runpy.run_path(str(REPORTER))["main"]

    def _measure(self, *filenames: str) -> None:
        data: CoverageData = CoverageData(basename=str(self.data_file))
        data.add_lines({name: {1} for name in filenames})
        data.write()

    def _assert_refused(self, reason: str) -> None:
        entries: set[Path] = set(self.output.iterdir())
        with self.assertRaises(ValueError) as caught:
            self.report()
        self.assertEqual(str(caught.exception), reason)
        self.assertEqual(self.published.read_bytes(), self.previous)
        self.assertEqual(set(self.output.iterdir()), entries)

    def _assert_report_keys_refused(self, keys: tuple[str, ...], reason: str) -> None:
        self._measure(self.absolute)

        def emit_report(*, morfs: list[str], outfile: str, show_contexts: bool) -> None:
            """Substitute only the report emitted by Coverage.py."""
            Path(outfile).write_text(
                json.dumps({"files": dict.fromkeys(keys, {}), "meta": {}}), encoding="utf-8"
            )

        with patch.object(Coverage, "json_report", side_effect=emit_report):
            self._assert_refused(reason)

    def test_raw_inventory_count_is_exact(self) -> None:
        with self.subTest("missing module"):
            self._assert_report_keys_refused((self.name,), "raw-report-inventory-mismatch")
        with self.subTest("extra module"):
            self._assert_report_keys_refused(
                (self.name, self.marker, "metadata/extra.py"), "raw-report-inventory-mismatch"
            )

    def test_raw_inventory_rejects_missing_and_duplicate_module_at_same_count(self) -> None:
        reason: str = "missing-or-duplicate-raw-module-observation:" + self.name
        with self.subTest("same count, different module"):
            self._assert_report_keys_refused((self.marker, "metadata/extra.py"), reason)
        with self.subTest("same count, duplicate aliases"):
            self._assert_report_keys_refused((self.name, self.absolute), reason)

    def test_conflicting_measured_aliases_are_rejected(self) -> None:
        self._measure(self.name, self.absolute)
        self._assert_refused("ambiguous-measured-module-observation:" + self.name)

    def test_unexpected_analysis_filename_is_rejected(self) -> None:
        unrelated: Path = self.root / "metadata/unselected.py"
        unrelated.write_text("unselected = 1\n", encoding="utf-8")
        # With neither selected alias measured, only the analysis filename
        # check can reject the substituted result.
        self._measure(str(unrelated))
        with patch.object(Coverage, "analysis2", return_value=(str(unrelated), [1], [], [], "")):
            self._assert_refused("ambiguous-measured-module-observation:" + self.name)


if __name__ == "__main__":
    unittest.main()
