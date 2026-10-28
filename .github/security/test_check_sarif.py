"""Independent accepting/rejecting controls for the blocking CodeQL report gate."""

import importlib.util
import json
from pathlib import Path
import tempfile
import unittest


PATH = Path(__file__).with_name("check_sarif.py")


class SarifPolicyTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        spec = importlib.util.spec_from_file_location("sarif_policy", PATH)
        cls.policy = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(cls.policy)

    def healthy(self):
        return {
            "version": "2.1.0",
            "runs": [{
                "automationDetails": {"id": "/language:python/"},
                "tool": {"driver": {"name": "CodeQL", "rules": []}, "extensions": [{
                    "rules": [
                        {"id": "py/code-injection"},
                        {"id": "py/summary/lines-of-code", "properties": {"tags": ["lines-of-code"]}},
                    ]
                }]},
                "invocations": [{"executionSuccessful": True}],
                "properties": {"metricResults": [{
                    "rule": {"index": 1, "toolComponent": {"index": 0}}, "value": 123
                }]},
                "artifacts": [{"location": {"uri": "scripts/check-environment.py"},
                               "roles": ["analysisTarget"]}],
                "results": [],
            }],
        }

    def check(self, report):
        return self.policy.evaluate(report, "python")

    def test_real_codeql_metric_rule_reference_shape_passes(self):
        self.assertEqual(self.check(self.healthy())["analyzed_lines"], 123)

    def test_flat_driver_rules_and_metric_rule_id_are_supported(self):
        report = self.healthy()
        run = report["runs"][0]
        run["tool"]["driver"]["rules"] = run["tool"].pop("extensions")[0]["rules"]
        run["properties"]["metricResults"][0] = {"ruleId": "py/summary/lines-of-code", "value": 123}
        self.assertEqual(self.check(report)["query_count"], 2)

    def test_all_findings_block_including_notes_and_suppressions(self):
        for result in ({"ruleId": "py/code-injection", "level": "error"},
                       {"ruleId": "py/code-injection", "level": "note"},
                       {"ruleId": "py/code-injection", "suppressions": [{"status": "accepted"}]},
                       {"ruleId": "py/code-injection", "baselineState": "unchanged"}):
            with self.subTest(result=result):
                report = self.healthy()
                report["runs"][0]["results"] = [result]
                with self.assertRaises(self.policy.Rejected):
                    self.check(report)

    def test_missing_empty_failed_and_wrong_language_reports_block(self):
        for edit in (lambda r: r.update(runs=[]),
                     lambda r: r["runs"][0].pop("results"),
                     lambda r: r["runs"][0].update(invocations=[]),
                     lambda r: r["runs"][0]["invocations"][0].update(executionSuccessful=False),
                     lambda r: r["runs"][0]["invocations"][0].update(executionSuccessful=1),
                     lambda r: r["runs"][0].update(properties={}),
                     lambda r: r["runs"][0].update(artifacts=[]),
                     lambda r: r["runs"][0]["automationDetails"].update(id="/language:javascript/"),
                     lambda r: r["runs"][0]["tool"]["extensions"][0].update(rules=[]),
                     lambda r: r["runs"][0]["tool"]["driver"].update(name="unreviewed")):
            report = self.healthy()
            edit(report)
            with self.assertRaises(self.policy.Rejected):
                self.check(report)

    def test_partial_analysis_notifications_block(self):
        for key in ("toolExecutionNotifications", "toolConfigurationNotifications"):
            for level in ("warning", "error"):
                report = self.healthy()
                report["runs"][0]["invocations"][0][key] = [{"level": level}]
                with self.assertRaises(self.policy.Rejected):
                    self.check(report)

    def test_analyzed_source_files_need_nonempty_locations(self):
        for artifact in ({"roles": ["analysisTarget"]},
                         {"roles": ["analysisTarget"], "location": {"uri": " "}}):
            report = self.healthy()
            report["runs"][0]["artifacts"] = [artifact]
            with self.assertRaises(self.policy.Rejected):
                self.check(report)

    def test_zero_negative_boolean_and_invalid_line_counts_block(self):
        for count in (0, -1, True, "123", None, float("inf"), float("nan")):
            report = self.healthy()
            report["runs"][0]["properties"]["metricResults"][0]["value"] = count
            with self.assertRaises(self.policy.Rejected):
                self.check(report)

    def test_malformed_duplicate_oversized_and_symlink_input_block(self):
        with tempfile.TemporaryDirectory() as folder:
            path = Path(folder) / "python.sarif"
            for data in ('{"version":"2.1.0","version":"2.1.0"}', '{"runs":NaN}', '[]'):
                path.write_text(data)
                with self.assertRaises(self.policy.Rejected):
                    self.policy.read_report(path)
            path.write_text(json.dumps(self.healthy()))
            alias = Path(folder) / "alias.sarif"
            alias.symlink_to(path)
            with self.assertRaises(self.policy.Rejected):
                self.policy.read_report(alias)
            with self.assertRaises(self.policy.Rejected):
                self.policy.read_report(path, maximum_bytes=1)


if __name__ == "__main__":
    unittest.main()
