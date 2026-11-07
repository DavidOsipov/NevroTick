"""Run the real unittest suite and propagate each test identity into CLI children."""

from __future__ import annotations

import json
import io
import os
from pathlib import Path
import sys
import time
import unittest

import coverage

from coverage_processes import install_process_ledger


TEST_DURATIONS: dict[str, float] = {}


def complete_result(result: unittest.TestResult, discovered: int) -> bool:
    return (result.wasSuccessful() and discovered > 0 and result.testsRun == discovered
            and not result.skipped and not result.expectedFailures
            and not result.unexpectedSuccesses)


def controls() -> int:
    class Healthy(unittest.TestCase):
        def runTest(self) -> None:
            self.assertEqual(2 + 2, 4)

    @unittest.skip("owned harness control")
    class Skipped(Healthy):
        pass

    class ExpectedFailure(unittest.TestCase):
        @unittest.expectedFailure
        def runTest(self) -> None:
            self.fail("owned harness expected-failure control")

    observed = {}
    for name, case in (("healthy", Healthy), ("skipped", Skipped), ("expected-failure", ExpectedFailure)):
        result = unittest.TextTestRunner(stream=io.StringIO()).run(case())
        observed[name] = {"unittest_success": result.wasSuccessful(), "complete": complete_result(result, 1)}
    print(json.dumps(observed, sort_keys=True))
    return 0 if observed["healthy"]["complete"] and not observed["skipped"]["complete"] and not observed["expected-failure"]["complete"] else 1


class ContextResult(unittest.TextTestResult):
    def startTest(self, test: unittest.TestCase) -> None:
        name = test.id()
        os.environ["NEVROTICK_COVERAGE_CONTEXT"] = name
        current = coverage.Coverage.current()
        if current is None:
            raise RuntimeError("coverage instrumentation is not active")
        current.switch_context(name)
        self.started = time.monotonic()
        super().startTest(test)

    def stopTest(self, test: unittest.TestCase) -> None:
        TEST_DURATIONS[test.id()] = time.monotonic() - self.started
        super().stopTest(test)


def main() -> int:
    root = Path(sys.argv[1])
    install_process_ledger()
    suite = unittest.defaultTestLoader.discover(str(root / "metadata/tests"))
    discovered = suite.countTestCases()
    started = time.monotonic()
    result = unittest.TextTestRunner(verbosity=2, resultclass=ContextResult).run(suite)
    report = {"discovered": discovered, "tests_run": result.testsRun,
              "failures": len(result.failures), "errors": len(result.errors),
              "skipped": len(result.skipped), "expected_failures": len(result.expectedFailures),
              "unexpected_successes": len(result.unexpectedSuccesses),
              "successful": complete_result(result, discovered), "unittest_successful": result.wasSuccessful(),
              "elapsed_seconds": time.monotonic() - started,
              "test_durations": TEST_DURATIONS}
    Path(sys.argv[2]).write_text(json.dumps(report, indent=2) + "\n")
    return 0 if complete_result(result, discovered) else 1


if __name__ == "__main__":
    raise SystemExit(controls() if sys.argv[1:] == ["--controls"] else main())
