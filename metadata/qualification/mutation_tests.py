"""Controls for awarding mutation credit from reviewed startup failures."""

from __future__ import annotations

from copy import deepcopy
from hashlib import sha256
from importlib.util import module_from_spec, spec_from_file_location
from itertools import product
import json
from pathlib import Path
import sys
import tempfile
import unittest


QUALIFICATION = Path(__file__).resolve().parent
# Unittest discovery adds this directory to sys.path. Load the isolated driver
# with standard-library typing rather than the sibling typing qualification tool.
discovery_path = sys.path
try:
    sys.path = [entry for entry in discovery_path if Path(entry).resolve() != QUALIFICATION]
    from typing import Any
    from unittest.mock import patch

    spec = spec_from_file_location("metadata_mutation_qualification", QUALIFICATION / "mutation.py")
    assert spec is not None and spec.loader is not None
    mutation = module_from_spec(spec)
    spec.loader.exec_module(mutation)
finally:
    sys.path = discovery_path

ENTRYPOINT = "metadata/pipeline/__main__.py"
CLI_TEST = "test_cli_contracts.CliContractTests.test_script_and_import_entrypoint_roles"
STAGE_TEST = "test_pipeline.MetadataCliTests.test_source_stage_is_the_default"
SCRIPT_ROLE = "isolated metadata script"
IMPORT_ROLE = "metadata package import"
RAW_OUTCOME = "startup-review-required"
STARTUP_KILL = "killed-application-startup"


def startup_record(file: str = ENTRYPOINT, test: str = CLI_TEST,
                   role: str = SCRIPT_ROLE) -> dict[str, Any]:
    """Construct the original failure and a completed supplemental assertion."""
    return {
        "id": "startup-mutant",
        "file": file,
        "mutant_sha256": sha256(b"fixture mutant").hexdigest(),
        "outcome": RAW_OUTCOME,
        "import": {"exit": 1, "timeout": False, "elapsed": 0.1},
        "startup_cli_positive": {
            "timeout": False,
            "exit": 1,
            "report": {
                "discovered": 1,
                "discovered_ids": [test],
                "run": 1,
                "executed_ids": [test],
                "failures": [{"test": test, "traceback": f"AssertionError: {role}"}],
                "errors": [],
                "skipped": [],
                "expected_failures": [],
                "unexpected_successes": [],
                "successful": False,
            },
        },
    }


class StartupReviewTests(unittest.TestCase):
    def setUp(self) -> None:
        self.directory = Path(self.enterContext(tempfile.TemporaryDirectory(prefix="nevrotick-startup-review-")))
        self.worker = self.directory / "worker"
        self.prior_path = self.directory / "outcomes" / "startup-mutant" / "outcome.json"
        self.prior_path.parent.mkdir(parents=True)
        self.review_path = self.directory / "review.json"

    def apply_review(self, observed: dict[str, Any], prior: dict[str, Any]) -> list[dict[str, Any]]:
        source = b"reviewed fixture source\n"
        evidence = json.dumps(prior).encode()
        self.prior_path.write_bytes(evidence)
        self.review_path.write_text(json.dumps({
            "independent_review": "fixture reviewer",
            "evidence": str(self.directory),
            "source_sha256": sha256(source).hexdigest(),
            "startup_semantic_kills": [{
                "id": prior["id"],
                "mutant_sha256": prior["mutant_sha256"],
                "outcome_sha256": sha256(evidence).hexdigest(),
                "raw_outcome": RAW_OUTCOME,
                "argument": "The selected startup role failed its actual assertion.",
            }],
        }), encoding="utf-8")
        return mutation.apply_reviews([observed], {observed["file"]: source}, [self.review_path])

    def test_matching_startup_role_and_source_stage_receive_credit(self) -> None:
        for file, test, role in (
            (ENTRYPOINT, CLI_TEST, SCRIPT_ROLE),
            (ENTRYPOINT, CLI_TEST, IMPORT_ROLE),
            ("metadata/pipeline/fields.py", STAGE_TEST, "source stage"),
        ):
            with self.subTest(file=file, role=role):
                prior = startup_record(file, test, role)
                observed = deepcopy(prior)
                probe = observed.pop("startup_cli_positive")
                with patch.object(mutation, "execute_tests", return_value=probe) as execute:
                    mutation.record_startup_probe(observed, Path(sys.executable), self.worker,
                                                  self.directory, 2.0)
                execute.assert_called_once_with(
                    Path(sys.executable), self.worker, [test], self.directory / "startup-cli-positive", 2.0
                )
                self.assertEqual(observed["startup_cli_positive"], probe)
                self.assertEqual(observed["startup_probe_classification"], "killed-assertion")
                accepted = self.apply_review(observed, prior)
                self.assertEqual([(entry["id"], entry["decision"]) for entry in accepted],
                                 [(prior["id"], STARTUP_KILL)])
                self.assertEqual(observed["outcome"], RAW_OUTCOME)
                self.assertEqual(observed["reviewed_outcome"], STARTUP_KILL)

    def test_timeout_and_non_assertion_exits_receive_no_credit(self) -> None:
        changes = ({"timeout": True}, {"exit": 0}, {"exit": -9}, {"exit": True})
        for target, change in product(("reviewed", "replayed"), changes):
            with self.subTest(target=target, change=change):
                prior = startup_record()
                observed = deepcopy(prior)
                record = prior if target == "reviewed" else observed
                record["startup_cli_positive"].update(change)
                with self.assertRaisesRegex(ValueError, "needs its actual CLI positive assertion"):
                    self.apply_review(observed, prior)
                self.assertNotIn("reviewed_outcome", observed)

    def test_wrong_test_and_conflicting_probe_results_receive_no_credit(self) -> None:
        failure = {"test": CLI_TEST, "traceback": f"AssertionError: {SCRIPT_ROLE}"}
        changes = (
            {"run": 2},
            {"executed_ids": [STAGE_TEST]},
            {"failures": [{"test": STAGE_TEST, "traceback": failure["traceback"]}]},
            {"failures": [failure, failure]},
            {"errors": [{"test": CLI_TEST, "traceback": "OSError: unrelated cleanup"}]},
            {"skipped": [{"test": CLI_TEST, "reason": "supplemental assertion did not run"}]},
        )
        for target, change in product(("reviewed", "replayed"), changes):
            with self.subTest(target=target, change=change):
                prior = startup_record()
                observed = deepcopy(prior)
                record = prior if target == "reviewed" else observed
                record["startup_cli_positive"]["report"].update(change)
                with self.assertRaisesRegex(ValueError, "needs its actual CLI positive assertion"):
                    self.apply_review(observed, prior)
                self.assertNotIn("reviewed_outcome", observed)

    def test_different_or_ambiguous_startup_roles_receive_no_credit(self) -> None:
        both_roles = f"{SCRIPT_ROLE} and {IMPORT_ROLE}"
        for reviewed_role, replayed_role in (
            (SCRIPT_ROLE, IMPORT_ROLE),
            (SCRIPT_ROLE, both_roles),
            (both_roles, both_roles),
            ("unspecified startup", "unspecified startup"),
        ):
            with self.subTest(reviewed=reviewed_role, replayed=replayed_role):
                observed = startup_record(role=replayed_role)
                with self.assertRaisesRegex(ValueError, "startup failure roles differ"):
                    self.apply_review(observed, startup_record(role=reviewed_role))
                self.assertNotIn("reviewed_outcome", observed)

    def test_supplemental_execution_error_retains_original_failure(self) -> None:
        prior = startup_record()
        for error in (OSError("worker unavailable"), ValueError("probe report unreadable")):
            with self.subTest(error=type(error).__name__):
                observed = deepcopy(prior)
                del observed["startup_cli_positive"]
                original = deepcopy(observed)
                with patch.object(mutation, "execute_tests", side_effect=error):
                    mutation.record_startup_probe(observed, Path(sys.executable), self.worker,
                                                  self.directory, 2.0)
                self.assertEqual(observed, original | {"startup_probe_failure": repr(error)})
                with self.assertRaisesRegex(ValueError, "needs its actual CLI positive assertion"):
                    self.apply_review(observed, prior)
                self.assertNotIn("reviewed_outcome", observed)
