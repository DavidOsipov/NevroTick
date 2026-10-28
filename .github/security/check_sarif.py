#!/usr/bin/env python3
"""Block CodeQL findings and incomplete reports; never print source or secrets.

Run only after a successful analyze step in the same job. This is a report gate,
not proof that the extractor understands every application trust boundary.
"""

from __future__ import annotations

import argparse
import json
import math
from pathlib import Path
import sys


class Rejected(ValueError):
    """Missing, incomplete, unreviewed or finding-bearing analysis."""


SENTINELS = {
    "python": "py/code-injection",
    "javascript-typescript": "js/code-injection",
    "actions": "actions/code-injection/critical",
    "rust": "rust/command-line-injection",
}
MAXIMUM_BYTES = 64 * 1024 * 1024


def read_report(path: Path, maximum_bytes: int = MAXIMUM_BYTES) -> dict:
    """Read one bounded regular SARIF JSON file without ambiguous JSON keys."""
    if path.is_symlink() or not path.is_file() or path.stat().st_size > maximum_bytes:
        raise Rejected("Report is absent, linked or oversized.")

    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise Rejected("Duplicate JSON key in report.")
            result[key] = value
        return result

    def invalid_constant(_value):
        raise Rejected("Non-finite JSON constant in report.")

    try:
        with path.open("rb") as stream:
            data = stream.read(maximum_bytes + 1)
        if len(data) > maximum_bytes:
            raise Rejected("Report exceeds the admitted byte budget.")
        report = json.loads(data, object_pairs_hook=unique,
                            parse_constant=invalid_constant)
    except (OSError, UnicodeError, json.JSONDecodeError, RecursionError) as error:
        raise Rejected("Report is unreadable or malformed.") from error
    if not isinstance(report, dict):
        raise Rejected("Report root must be an object.")
    return report


def evaluate(report: dict, language: str) -> dict:
    """Require successful nonempty analysis and reject every result/suppression."""
    if language not in SENTINELS or report.get("version") != "2.1.0":
        raise Rejected("Unadmitted language or SARIF version.")
    runs = report.get("runs")
    if not isinstance(runs, list) or not runs:
        raise Rejected("Analysis contains no runs.")
    total_rules = total_lines = total_files = 0
    for run in runs:
        if not isinstance(run, dict):
            raise Rejected("Malformed analysis run.")
        tool = run.get("tool", {})
        driver = tool.get("driver", {})
        if driver.get("name") not in {"CodeQL", "CodeQL command-line toolchain"}:
            raise Rejected("Unexpected analysis producer.")
        category = run.get("automationDetails", {}).get("id", "")
        if category != f"/language:{language}/":
            raise Rejected("Unexpected analysis category.")
        components = [driver, *tool.get("extensions", [])]
        rules = [rule for component in components for rule in component.get("rules", [])]
        by_id = {rule.get("id"): rule for rule in rules}
        if SENTINELS[language] not in by_id:
            raise Rejected("Required security query was not executed.")
        total_rules += len(rules)
        invocations = run.get("invocations")
        if not isinstance(invocations, list) or not invocations:
            raise Rejected("Invocation evidence is absent.")
        for invocation in invocations:
            if invocation.get("executionSuccessful") is not True:
                raise Rejected("Analyzer invocation was unsuccessful or incomplete.")
            for key in ("toolExecutionNotifications", "toolConfigurationNotifications"):
                for notice in invocation.get(key, []):
                    if notice.get("level", "warning") != "note":
                        raise Rejected("Analyzer reported an execution/configuration diagnostic.")
        results = run.get("results")
        if not isinstance(results, list):
            raise Rejected("Analysis result array is absent.")
        if results:
            # Includes notes, accepted suppressions and unchanged baseline alerts.
            raise Rejected(f"CodeQL reported {len(results)} finding(s); correction or independent disposition required.")
        lines = 0
        for metric in run.get("properties", {}).get("metricResults", []):
            rule = by_id.get(metric.get("ruleId"))
            if rule is None:
                ref = metric.get("rule", {})
                component_ref = ref.get("toolComponent", {})
                index = component_ref.get("index")
                component = (tool.get("extensions", [])[index]
                             if type(index) is int and 0 <= index < len(tool.get("extensions", []))
                             else driver)
                rule_index = ref.get("index")
                descriptors = component.get("rules", [])
                if type(rule_index) is int and 0 <= rule_index < len(descriptors):
                    rule = descriptors[rule_index]
            if rule and "lines-of-code" in rule.get("properties", {}).get("tags", []):
                value = metric.get("value")
                if type(value) not in (int, float) or not math.isfinite(value) or value <= 0:
                    raise Rejected("Analyzed lines of code must be positive.")
                lines += value
        files = [artifact for artifact in run.get("artifacts", [])
                 if "analysisTarget" in artifact.get("roles", [])]
        if any(not isinstance(artifact.get("location", {}).get("uri"), str)
               or not artifact["location"]["uri"].strip() for artifact in files):
            raise Rejected("Analyzed source-file evidence lacks a location.")
        if lines <= 0 or not files:
            raise Rejected("Positive analyzed-line and source-file evidence is required.")
        total_lines += lines
        total_files += len(files)
    return {"query_count": total_rules, "analyzed_lines": total_lines,
            "analyzed_files": total_files, "findings": 0}


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--language", choices=tuple(SENTINELS), required=True)
    parser.add_argument("--reports", type=Path, required=True)
    args = parser.parse_args(argv)
    try:
        reports = list(args.reports.glob("*.sarif"))
        if len(reports) != 1:
            raise Rejected("Expected exactly one language report.")
        counts = evaluate(read_report(reports[0]), args.language)
    except (Rejected, OSError, KeyError, TypeError, AttributeError, IndexError) as error:
        # Only policy-owned messages are printable; report content can hold secrets.
        message = str(error) if isinstance(error, Rejected) else "Malformed analysis structure."
        print(f"FAILED: {message}", file=sys.stderr)
        return 1
    print(json.dumps({"language": args.language, **counts}, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
