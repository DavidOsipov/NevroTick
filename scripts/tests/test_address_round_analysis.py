"""Synthetic admission/accounting controls only; no compiler or product execution.

These invented fixture identities are never policy bindings or qualification.
"""
from __future__ import annotations

import copy
import hashlib
import importlib.util
from pathlib import Path
import sys
import tempfile
from types import FrameType, SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("address_analysis_controls", ROOT / "scripts/verify-engine-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def fixture() -> dict:
    """Construct synthetic shape controls, never an actual admission receipt."""
    ordinary = sorted([prefix + "synthetic_test" for prefix in MODULE.ADDRESS_TEST_SOURCES]
                      + ["synthetic_test_" + str(index) for index in range(176)])
    path = MODULE.ADDRESS_SUBJECTS["address_round"] + " - series::address_round::"
    docs = sorted([path + "AddressRoundConfig::new (line 1)",
                   *[path + "AddressRoundConfig (line " + str(index) + ") - compile fail" for index in (2, 3, 4)],
                   path + "PreparedAddressRound (line 5) - compile fail"]
                  + ["synthetic_doc_" + str(index) for index in range(64)])
    inventory = {"ordinary": ordinary, "doctests": docs,
                 "sha256": {"ordinary": MODULE.COMMON.canonical_sha256(ordinary),
                            "doctests": MODULE.COMMON.canonical_sha256(docs),
                            "baseline": MODULE.COMMON.canonical_sha256(sorted(ordinary + docs))}}
    binding = {"candidate_sha256": "1" * 64, "source_file_count": 1,
               "inputs_sha256": dict.fromkeys(MODULE.ADDRESS_BOUND_INPUTS, "2" * 64),
               "ordinary_evidence": None, "test_inventory": inventory}
    return {**copy.deepcopy(MODULE.ADDRESS_FIXED_POLICY), "binding": binding, "mutation_inventories": {}}


def execution(names: list[str]) -> str:
    """Literal complete healthy libtest block; no real tests are represented."""
    return (f"running {len(names)} tests\n" + "".join(f"test {name} ... ok\n" for name in names)
            + f"test result: ok. {len(names)} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")


def failed_execution(sections: list[tuple[str, str]]) -> str:
    """Construct complete synthetic libtest records with independently chosen causes."""
    names = [name for name, _body in sections]
    return (f"running {len(names)} tests\n"
            + "".join(f"test {name} ... FAILED\n" for name in names)
            + "\nfailures:\n\n"
            + "".join(f"---- {name} stdout ----\n{body}\n\n" for name, body in sections)
            + "failures:\n\n" + "".join(f"    {name}\n" for name in names)
            + f"\ntest result: FAILED. 0 passed; {len(names)} failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")


def r47_fixture() -> tuple[str, list[tuple[str, str]], dict]:
    """Invent only parser-shape data; these names supply no real source qualification."""
    name = "crates/nevrotick-engine/src/address_round.rs:394:8: delete ! in guarded_writable"
    prefix = "series::address_round::tests::"
    writable = prefix + "final_writable_guard_rechecks_controls_after_actual_socket_readiness"
    receive = prefix + "real_receive_budget_boundary_distinguishes_valid_sixty_fourth_from_all_rejected"
    source = "crates/nevrotick-engine/tests/private/address_round.rs"
    marker = "quiet writable neighbor never sent its actual query"
    site = {"source": source, "line": 10, "column": 5, "owner": writable.removeprefix(prefix),
            "marker": marker, "exact": True}
    bodies = [(writable, f"thread '{writable}' (7) panicked at {source}:10:5:\n{marker}"),
              (receive, 'Error: Custom { kind: TimedOut, error: "receive-boundary fixture exceeded its actual original cutoff" }')]
    return name, bodies, {writable: [site], receive: [{**site, "line": 20}]}


def admit_caught(text: str, name: str, sites: dict) -> dict:
    """Use the actual owned-file decoder; temporary synthetic logs are not evidence."""
    outcomes = {"outcomes": [{"summary": "CaughtMutant", "scenario": {"Mutant": {"name": name}},
                              "log_path": "caught.log"}]}
    with tempfile.TemporaryDirectory() as raw_directory:
        directory = Path(raw_directory)
        (directory / "caught.log").write_text(text)
        return MODULE.address_caught_causes(directory, outcomes, sites)


class AddressAnalysisControls(unittest.TestCase):
    """Challenge exact scope, binding, denominators and failure cause admission."""

    def test_fresh_wrapper_module_imports_are_bound(self) -> None:
        observed: set[str] = set()

        def observe_module(frame: FrameType, event: str, _argument: object) -> None:
            if event != "call" or frame.f_code.co_name != "<module>":
                return
            path = Path(frame.f_code.co_filename)
            if path.is_absolute() and path.is_relative_to(ROOT):
                observed.add(path.relative_to(ROOT).as_posix())

        previous_profile = sys.getprofile()
        try:
            sys.setprofile(observe_module)
            spec = importlib.util.spec_from_file_location(
                "address_analysis_import_custody_control", ROOT / "scripts/verify-engine-analysis.py")
            fresh = importlib.util.module_from_spec(spec)
            spec.loader.exec_module(fresh)
        finally:
            sys.setprofile(previous_profile)
        self.assertIs(sys.getprofile(), previous_profile)
        self.assertIn("scripts/verify-engine-analysis.py", observed)
        self.assertFalse(observed - set(fresh.ADDRESS_BOUND_INPUTS),
                         "Unbound repository module imports: " + repr(sorted(observed - set(fresh.ADDRESS_BOUND_INPUTS))))

    def test_unbound_policy_refuses_before_execution(self) -> None:
        policy = fixture()
        policy["binding"] = None
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_address_policy(policy)

    def test_two_owner_shape_has_no_inherited_exclusions(self) -> None:
        policy = fixture()
        self.assertEqual(MODULE.validate_address_policy(policy), policy["binding"])
        policy["mutation_equivalences"] = [{"synthetic": True}]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_address_policy(policy)

    def test_thresholds_jobs_and_exact_integer_bounds_cannot_weaken(self) -> None:
        for key, value in (("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
                           ("jobs", True), ("mutation_jobs", 2), ("minimum_free_bytes", 0)):
            policy = fixture()
            policy[key] = value
            with self.assertRaises(MODULE.Incomplete):
                MODULE.validate_address_policy(policy)

    def test_campaign_requires_both_nonempty_reviewed_inventories(self) -> None:
        policy = fixture()
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_address_policy(policy, campaign=True)
        policy["mutation_inventories"] = {name: {"count": 1, "sha256": "3" * 64} for name in MODULE.ADDRESS_SUBJECTS}
        MODULE.validate_address_policy(policy, campaign=True)
        del policy["mutation_inventories"]["address_progress"]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.validate_address_policy(policy, campaign=True)

    def test_missing_duplicate_and_changed_native_identities_refuse(self) -> None:
        binding = fixture()["binding"]
        ordinary = binding["test_inventory"]["ordinary"]
        MODULE.address_execution(execution(ordinary), binding, doctests=False)
        for names in (ordinary[:-1], ordinary + [ordinary[0]], ["wrong_test", *ordinary[1:]]):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.address_execution(execution(names), binding, doctests=False)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.address_execution(execution(ordinary).replace("0 filtered out", "1 filtered out"), binding, doctests=False)

    def test_positive_baseline_requires_all_ordinary_and_doctest_records(self) -> None:
        binding = fixture()["binding"]
        inventory = binding["test_inventory"]
        MODULE.address_execution(execution(inventory["ordinary"]) + execution(inventory["doctests"]), binding, doctests=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.address_execution(execution(inventory["ordinary"]), binding, doctests=True)

    def test_success_blocks_require_matching_running_counts_and_complete_endings(self) -> None:
        healthy = execution(["synthetic_healthy"])
        self.assertEqual(MODULE.read_successful_address_records(healthy + execution([])), ["synthetic_healthy"])
        for changed in (healthy.replace("running 1 tests", "running 2 tests"),
                        healthy + "running 1 test\n", healthy + "running 1 test",
                        healthy + "test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n",
                        "test synthetic_orphan ... ok\n" + healthy,
                        healthy.replace("1 passed;", "0 passed;"), healthy + "ok\n"):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.read_successful_address_records(changed)

    def test_storage_inventory_includes_shared_and_mutation_worker_targets(self) -> None:
        with tempfile.TemporaryDirectory() as raw_directory:
            directory = Path(raw_directory)
            shared = directory / "work/target/debug/shared"
            worker = directory / "work/tmp/mutant-0/target/debug/worker"
            for path in (shared, worker):
                path.parent.mkdir(parents=True)
                path.write_bytes(b"synthetic allocation control" * 128)
            usage = MODULE.address_storage_usage(directory)
            self.assertGreaterEqual(usage["allocated_bytes"], sum(path.stat().st_blocks * 512 for path in (shared, worker)))
            self.assertGreaterEqual(usage["entries"], 9)

    def test_aggregate_cap_is_checked_at_each_before_and_after_boundary(self) -> None:
        policy = fixture()
        cap = policy["owned_output_maximum_allocated_bytes"]
        with tempfile.TemporaryDirectory() as raw_directory:
            session = SimpleNamespace(output=Path(raw_directory))
            with patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=6 * 1024**3)):
                for phase in ("before", "after"):
                    with patch.object(MODULE, "address_storage_usage", return_value={"allocated_bytes": cap - 1, "entries": 1}):
                        MODULE.address_storage_checkpoint(session, policy, {}, "synthetic-command", phase)
                    for allocated in (cap, cap + 1):
                        with patch.object(MODULE, "address_storage_usage", return_value={"allocated_bytes": allocated, "entries": 1}):
                            with self.assertRaises(MODULE.Incomplete):
                                MODULE.address_storage_checkpoint(session, policy, {}, "synthetic-command", phase)

    def test_caught_log_requires_exact_semantic_assertion_and_no_native_error(self) -> None:
        prefix, filename = next(iter(MODULE.ADDRESS_TEST_SOURCES.items()))
        test = prefix + "synthetic_test"
        site = {"source": filename, "line": 10, "column": 5, "owner": "synthetic_test",
                "marker": "assertion failed:", "exact": False}
        text = (f"running 1 test\ntest {test} ... FAILED\n\nfailures:\n\n---- {test} stdout ----\n"
                f"thread '{test}' (7) panicked at {filename}:10:5:\nassertion failed: result_matches\n"
                f"\nfailures:\n\n    {test}\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n")
        outcomes = {"outcomes": [{"summary": "CaughtMutant", "scenario": {"Mutant": {"name": "synthetic-mutant"}}, "log_path": "caught.log"}]}
        with tempfile.TemporaryDirectory() as raw_directory:
            directory = Path(raw_directory)
            path = directory / "caught.log"
            path.write_text(text)
            result = MODULE.address_caught_causes(directory, outcomes, {test: [site]})
            self.assertEqual(set(result["checked_caught_logs"]), {"synthetic-mutant"})
            for changed in (text.replace(":10:5:", ":11:5:"), text + "Error: synthetic fixture failure\n",
                            text + "ADDRESS_ROUND_ATTEMPT_IO position=0\n",
                            text + "ADDRESS_ROUND_SOCKET_RETAINED kind=AddrInUse\n", text + "process aborted\n",
                            text + "ADDRESS_ROUND_RECEIVE_INCOMPLETE status=Header received=63\n",
                            text.replace("assertion failed: result_matches", "deliberate expected control"), text[:-40]):
                path.write_text(changed)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.address_caught_causes(directory, outcomes, {test: [site]})

    def test_shared_scoring_keeps_nonempty_exact_per_file_denominators(self) -> None:
        report = {"data": [{"files": [{"filename": "/work/source/" + MODULE.ADDRESS_SUBJECTS["address_progress"],
                                        "summary": {"lines": {"count": 10, "covered": 9}}}]}]}
        MODULE.COMMON.coverage_counts(MODULE.ADDRESS_SCOPE, report, "/work/source/" + MODULE.ADDRESS_SUBJECTS["address_progress"])
        for lines in ({"count": 0, "covered": 0}, {"count": 10, "covered": 8}):
            report["data"][0]["files"][0]["summary"]["lines"] = lines
            with self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.coverage_counts(MODULE.ADDRESS_SCOPE, report, "/work/source/" + MODULE.ADDRESS_SUBJECTS["address_progress"])

    def test_r47_exact_receive_cutoff_has_zero_credit_beside_the_required_semantic_witness(self) -> None:
        name, sections, sites = r47_fixture()
        normal = admit_caught(failed_execution(sections[:1]), name, sites)
        self.assertEqual(normal["checked_caught_logs"][name]["causes"],
                         [{"test": sections[0][0], **sites[sections[0][0]][0]}])
        result = admit_caught(failed_execution(sections), name, sites)
        entry = result["checked_caught_logs"][name]
        section = f"---- {sections[1][0]} stdout ----\n{sections[1][1]}\n\n"
        self.assertEqual(entry["inconclusive_sections"], [{
            "test": sections[1][0], "section_sha256": hashlib.sha256(section.encode()).hexdigest(),
            "reason": "R47 receive cutoff is inconclusive; separate writable assertion supplies the cause",
            "kill_credit": 0}])
        self.assertEqual(entry["causes"], normal["checked_caught_logs"][name]["causes"])
        self.assertEqual(result["compiler_or_harness_kill_credit"], 0)

    def test_r47_adjudication_rejects_mutant_fixture_name_or_body_drift(self) -> None:
        name, sections, sites = r47_fixture()
        changed_name = sections[1][0] + "_other"
        changed_sites = {**sites, changed_name: sites[sections[1][0]]}
        variants = [(name.replace(":394:8:", ":395:8:"), sections, sites),
                    ("synthetic-other-mutant", sections, sites),
                    (name, [sections[0], (changed_name, sections[1][1])], changed_sites)]
        for body in (sections[1][1].replace("actual original cutoff", "another cutoff"),
                     sections[1][1] + "\nError: extra fixture error",
                     sections[1][1] + "\n" + sections[0][1],
                     "ADDRESS_ROUND_CLEANUP position=0\n" + sections[1][1]):
            variants.append((name, [sections[0], (sections[1][0], body)], sites))
        for mutant, records, admitted_sites in variants:
            with self.subTest(mutant=mutant, records=records):
                with self.assertRaises(MODULE.Incomplete):
                    admit_caught(failed_execution(records), mutant, admitted_sites)

    def test_r47_adjudication_requires_the_separate_exact_source_owned_writable_assertion(self) -> None:
        name, sections, sites = r47_fixture()
        other = "series::address_round::tests::synthetic_other_semantic_test"
        other_body = sections[0][1].replace(sections[0][0], other)
        variants = [(sections[1:], sites),
                    ([(other, other_body), sections[1]], {**sites, other: sites[sections[0][0]]}),
                    ([(sections[0][0], sections[0][1].replace(":10:5:", ":11:5:")), sections[1]], sites),
                    ([(sections[0][0], sections[0][1].replace("quiet writable neighbor", "different semantic fact")), sections[1]], sites),
                    ([(sections[0][0], sections[0][1].replace("quiet writable neighbor", "different semantic fact")), sections[1]],
                     {**sites, sections[0][0]: [{**sites[sections[0][0]][0],
                                               "marker": sites[sections[0][0]][0]["marker"].replace("quiet writable neighbor", "different semantic fact")}]}),
                    (sections, {**sites, sections[0][0]: [{**sites[sections[0][0]][0], "owner": "synthetic_other_owner"}]}),
                    (sections, {**sites, sections[0][0]: [{**sites[sections[0][0]][0], "source": "foreign.rs"}]})]
        for records, admitted_sites in variants:
            with self.subTest(records=records, sites=admitted_sites):
                with self.assertRaises(MODULE.Incomplete):
                    admit_caught(failed_execution(records), name, admitted_sites)

    def test_r47_adjudication_preserves_global_same_section_and_resource_failure_refusal(self) -> None:
        name, sections, sites = r47_fixture()
        text = failed_execution(sections)
        variants = [text + "Error: unknown global failure\n", "Error: unknown prelude failure\n" + text,
                    text + "ADDRESS_ROUND_CLEANUP position=0\n", text + "ADDRESS_ROUND_PEER kind=Other\n",
                    text + "source custody failed\nError: changed source\n", text + "process aborted\n",
                    text + "resource temporarily unavailable\n", text + "error[E0308]: mismatched types\n"]
        for marker in ("Error: same-section native failure", "ADDRESS_ROUND_WRITABLE_IO kind=Other",
                       "process aborted", "out of memory"):
            variants.append(failed_execution([(sections[0][0], sections[0][1] + "\n" + marker), sections[1]]))
        for marker in ("process aborted", "failed to spawn"):
            variants.append(failed_execution([sections[0], (sections[1][0], sections[1][1] + "\n" + marker)]))
        for changed in variants:
            with self.subTest(text=changed):
                with self.assertRaises(MODULE.Incomplete):
                    admit_caught(changed, name, sites)

    def test_r47_adjudication_rejects_duplicate_truncated_or_inconsistent_native_records(self) -> None:
        name, sections, sites = r47_fixture()
        text = failed_execution(sections)
        variants = [text[:-40], failed_execution([sections[0], sections[1], sections[1]]),
                    text.replace("running 2 tests", "running 3 tests"),
                    text.replace("2 failed;", "1 failed;"), text.replace("0 filtered out", "1 filtered out"),
                    text.replace(f"    {sections[1][0]}\n", "", 1)]
        for changed in variants:
            with self.subTest(text=changed):
                with self.assertRaises(MODULE.Incomplete):
                    admit_caught(changed, name, sites)


if __name__ == "__main__":
    unittest.main()
