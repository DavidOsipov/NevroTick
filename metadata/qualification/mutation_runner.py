"""Run selected real metadata tests in a disposable source tree.

This runner changes unittest reporting and ordering only. Production imports,
handler calls and the tests' isolated command-line subprocesses remain intact.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import sys
import time
import unittest


class RecordingResult(unittest.TextTestResult):
    """Retain the actual test IDs and durations for selection admission."""

    def __init__(self, *arguments, **keywords):
        super().__init__(*arguments, **keywords)
        self.executed_ids = []
        self.durations = {}

    def startTest(self, test):
        super().startTest(test)
        self.executed_ids.append(test.id())
        self.started = time.monotonic()

    def stopTest(self, test):
        self.durations[test.id()] = time.monotonic() - self.started
        super().stopTest(test)


def test_ids(suite):
    result = []
    for item in suite:
        if isinstance(item, unittest.TestSuite):
            result.extend(test_ids(item))
        else:
            result.append(item.id())
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--selection", type=Path)
    parser.add_argument("--report", type=Path, required=True)
    args = parser.parse_args()
    root = args.root.resolve(strict=True)
    sys.path.insert(0, str(root))
    sys.path.insert(0, str(root / "metadata/tests"))
    loader = unittest.TestLoader()
    if args.selection:
        names = json.loads(args.selection.read_text(encoding="utf-8"))
        if (not names or any(type(name) is not str or not name for name in names)
                or len(names) != len(set(names))):
            raise ValueError("A nonempty test selection is required")
        suite = loader.loadTestsFromNames(names)
    else:
        suite = loader.discover(str(root / "metadata/tests"))
    discovered = suite.countTestCases()
    discovered_ids = test_ids(suite)
    if args.selection and discovered != len(names):
        raise ValueError("Each selected test ID must resolve to exactly one test")
    started = time.monotonic()
    result = unittest.TextTestRunner(verbosity=2, failfast=True, resultclass=RecordingResult).run(suite)
    report = {
        "discovered": discovered,
        "discovered_ids": discovered_ids,
        "executed_ids": result.executed_ids,
        "test_durations": result.durations,
        "run": result.testsRun,
        "failures": [{"test": test.id(), "traceback": trace} for test, trace in result.failures],
        "errors": [{"test": test.id(), "traceback": trace} for test, trace in result.errors],
        "skipped": [{"test": test.id(), "reason": reason} for test, reason in result.skipped],
        "expected_failures": [{"test": test.id(), "traceback": trace}
                              for test, trace in result.expectedFailures],
        "unexpected_successes": [test.id() for test in result.unexpectedSuccesses],
        "elapsed": time.monotonic() - started,
        "successful": (result.wasSuccessful() and discovered > 0
                       and result.testsRun == discovered and not result.skipped
                       and not result.expectedFailures and not result.unexpectedSuccesses),
    }
    args.report.write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
    return 0 if report["successful"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
