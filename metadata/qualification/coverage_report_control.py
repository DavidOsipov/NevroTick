"""Exercise the coverage report CLI with the pinned coverage.py environment."""

from __future__ import annotations

import json
from pathlib import Path
import runpy
import subprocess
import sys
from tempfile import TemporaryDirectory
import unittest

from coverage import Coverage


class CoverageReportTests(unittest.TestCase):
    def test_file_contexts_are_retained_once(self) -> None:
        reporter = Path(__file__).with_name("coverage_report.py").resolve()
        with TemporaryDirectory() as directory:
            root = Path(directory)
            output = root / "output"
            output.mkdir()
            qualification = root / "metadata/qualification"
            qualification.mkdir(parents=True)
            name = "metadata/decision.py"
            marker = "metadata/__init__.py"
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
            config = output / ".coveragerc"
            config.write_text(f"[run]\nbranch = true\ndata_file = {output / '.coverage'}\n")
            measured = Coverage(config_file=str(config))
            measured.start()
            try:
                measured.switch_context("module-import")
                decisions = runpy.run_path(str(root / name))
                measured.switch_context("case-denied")
                self.assertEqual(decisions["Decision"]().choose(False), 0)
                measured.switch_context("case-allowed")
                self.assertEqual(decisions["Decision"]().choose(True), 1)
                self.assertTrue(decisions["is_allowed"](True))
            finally:
                measured.stop()
                measured.save()

            subprocess.run([sys.executable, "-I", "-B", str(reporter), str(output), str(root)],
                           cwd=root, check=True, timeout=15)
            report = json.loads((output / "coverage-raw.json").read_text())
            self.assertEqual(set(report["files"]), {name, marker})
            self.assertTrue(report["meta"]["show_contexts"])
            row = report["files"][name]
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


if __name__ == "__main__":
    unittest.main()
