"""Bounded formal-result admission controls; execute no compiler or solver."""
from __future__ import annotations

from collections import Counter
import copy
import importlib.util
from pathlib import Path
import unittest


SPEC = importlib.util.spec_from_file_location("formal_verify_test", Path(__file__).with_name("verify.py"))
VERIFY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(VERIFY)
ENTRY = "/work/inputs/proof.rs"


def healthy(subject="rounding"):
    """Literal independent obligations from the two mathematical specifications."""
    if subject == "rounding":
        assertions = ["scale classification", "destination classification", "exact residual units",
                      "exact signed reconstruction", "nearest integer", "even ties"]
        covers = ["zero succeeds", "negative destination endpoint succeeds",
                  "positive destination endpoint succeeds", "negative destination neighbor is rejected",
                  "positive destination neighbor is rejected", "raw signed minimum is covered"]
        harness = "phase_full_domain"
    else:
        assertions = ["absence classification", "ambiguity classification", "exact unique result"]
        covers = ["unique point witness", "absent witness", "multiple-era witness"]
        harness = "era_full_domain"
    checks = [{"category": "assertion", "description": f'"{name}"', "status": "Success",
               "location": {"file": ENTRY}} for name in assertions]
    checks.append({"category": "assertion", "description": '"required success"',
                   "status": "Unreachable", "location": {"file": ENTRY}})
    checks.extend({"category": "cover", "description": name, "status": "Satisfied"} for name in covers)
    job = {"subject": subject, "case": "healthy", "harness": harness, "intended_failure": None}
    return job, result(job, checks, True), {"exit_code": 0, "failure": None}


def result(job, checks, success):
    """Model the selected Kani JSON schema, preserving accounting independently."""
    report = {
        "verification_results": {
            "summary": {"status": "completed", "total_harnesses": 1, "executed": 1,
                        "successful": int(success), "failed": int(not success)},
            "results": [{"harness_id": job["harness"], "status": "Success" if success else "Failure",
                         "checks": checks}],
        },
        "error_details": [{"harness_id": job["harness"], "has_errors": not success,
                           "error_type": "assertion_failure", "exit_status": "properties_failed"}],
        "property_details": [{"harness_id": job["harness"], "property_details": {}}],
        "metadata": {"kani_version": "0.68.0", "target": "x86_64-unknown-linux-gnu"},
        "tools": {"kani": "0.68.0", "solvers": [{"name": "cadical"}]},
        "harness_metadata": [{"pretty_name": job["harness"], "has_loop_contracts": False,
                              "attributes": {"kind": "Proof", "should_panic": False},
                              "source": {"file": ENTRY}}],
        "cbmc": [{"harness_id": job["harness"], "configuration": {"solver": "cadical"},
                  "cbmc_stats": {"runtime_solver_s": 0.1}}],
    }
    recount(report)
    return report


def recount(report):
    """Keep negative fixtures internally consistent so accounting is no shortcut."""
    checks = report["verification_results"]["results"][0]["checks"]
    counts = Counter(item["status"] for item in checks)
    report["property_details"][0]["property_details"] = {
        "total_properties": len(checks), "passed": counts["Success"], "failed": counts["Failure"],
        "unreachable": counts["Unreachable"], "satisfied": counts["Satisfied"],
        "unsatisfiable": counts["Unsatisfiable"], "undetermined": 0, "solver_error": 0,
    }


def defect():
    """One intended failed mathematical assertion with an ordinary nonzero exit."""
    job = {"subject": "rounding", "case": "always_error", "harness": "control_zero",
           "intended_failure": "required success"}
    checks = [{"category": "assertion", "description": '"required success"',
               "status": "Failure", "location": {"file": ENTRY}}]
    return job, result(job, checks, False), {"exit_code": 1, "failure": None}


class QualificationTests(unittest.TestCase):
    """Challenge actual result admission while retaining healthy neighbors."""

    def qualify(self, values):
        job, report, native = values
        return VERIFY.qualify(report, native, job, ENTRY)

    def test_healthy_both_subjects_preserve_unreachable_error_branches(self):
        for subject in ("rounding", "era"):
            with self.subTest(subject=subject):
                self.assertGreater(self.qualify(healthy(subject))["checks"], 0)

    def test_intended_defect_failure_is_admitted(self):
        self.assertEqual(len(self.qualify(defect())["failed_checks"]), 1)

    def test_cover_only_result_is_incomplete(self):
        for subject in ("rounding", "era"):
            with self.subTest(subject=subject):
                values = healthy(subject)
                checks = values[1]["verification_results"]["results"][0]["checks"]
                checks[:] = [item for item in checks if item["category"] == "cover"]
                recount(values[1])
                with self.assertRaises(VERIFY.Incomplete):
                    self.qualify(values)

    def test_each_promised_assertion_must_be_observed_successfully(self):
        for subject in ("rounding", "era"):
            original = healthy(subject)
            for index, item in enumerate(original[1]["verification_results"]["results"][0]["checks"]):
                if item["category"] != "assertion" or item["status"] != "Success":
                    continue
                for alteration in ("missing", "unreachable", "satisfied", "wrong_file"):
                    with self.subTest(subject=subject, assertion=item["description"], alteration=alteration):
                        values = copy.deepcopy(original)
                        checks = values[1]["verification_results"]["results"][0]["checks"]
                        if alteration == "missing":
                            del checks[index]
                        elif alteration == "wrong_file":
                            checks[index]["location"]["file"] = "/work/other.rs"
                        else:
                            checks[index]["status"] = alteration.title()
                        recount(values[1])
                        with self.assertRaises(VERIFY.Incomplete):
                            self.qualify(values)

    def test_signal_termination_is_not_a_defect_control(self):
        for code in (-9, -15):
            with self.subTest(code=code):
                values = defect()
                values[2]["exit_code"] = code
                with self.assertRaises(VERIFY.Incomplete):
                    self.qualify(values)

    def test_unsatisfiable_noncover_obligation_is_incomplete(self):
        for factory in (healthy, defect):
            values = factory()
            values[1]["verification_results"]["results"][0]["checks"].append(
                {"category": "sanity", "description": "unresolved sanity", "status": "Unsatisfiable"})
            recount(values[1])
            with self.subTest(factory=factory.__name__), self.assertRaises(VERIFY.Incomplete):
                self.qualify(values)

    def test_malformed_nested_json_is_classified_incomplete(self):
        for path in (("tools", "solvers", 0), ("harness_metadata", 0, "attributes"),
                     ("harness_metadata", 0, "source"), ("cbmc", 0, "configuration"),
                     ("cbmc", 0, "cbmc_stats")):
            for invalid in (None, [], "malformed", 1):
                values = healthy()
                parent = values[1]
                for key in path[:-1]:
                    parent = parent[key]
                parent[path[-1]] = invalid
                with self.subTest(path=path, invalid=invalid), self.assertRaises(VERIFY.Incomplete):
                    self.qualify(values)

    def test_unrelated_failure_remains_incomplete(self):
        values = defect()
        values[1]["verification_results"]["results"][0]["checks"][0]["description"] = "unrelated assertion"
        with self.assertRaises(VERIFY.Incomplete):
            self.qualify(values)

    def test_native_timeout_remains_incomplete(self):
        values = defect()
        values[2]["failure"] = "wall timeout (180s)"
        with self.assertRaises(VERIFY.Incomplete):
            self.qualify(values)


if __name__ == "__main__":
    unittest.main()
