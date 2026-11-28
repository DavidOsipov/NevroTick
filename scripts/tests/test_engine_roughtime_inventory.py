"""Controls for the listing-only Engine Roughtime mutation boundary."""
from __future__ import annotations

import argparse
import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import time
import unittest
from unittest.mock import Mock

ROOT: Path = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "engine_roughtime_inventory_test", ROOT / "scripts/verify-engine-analysis.py")
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("Portable analysis module is unavailable.")
ENGINE_ANALYSIS = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENGINE_ANALYSIS)
COMMON = ENGINE_ANALYSIS.COMMON

TOOLS: Path = Path("/owned/tools")
PACKAGE: str = "nevrotick-engine"
ENGINE: str = "crates/nevrotick-engine/src"
FILES: tuple[tuple[str, str], ...] = (
    ("verifier", f"{ENGINE}/roughtime.rs"),
    ("exchange", f"{ENGINE}/roughtime/exchange.rs"),
    ("request", f"{ENGINE}/roughtime/request.rs"),
    ("tcp", f"{ENGINE}/roughtime/exchange/tcp.rs"),
    ("retry", f"{ENGINE}/roughtime/exchange/retry.rs"),
    ("round", f"{ENGINE}/roughtime/round.rs"),
    ("selection", f"{ENGINE}/roughtime/round/selection.rs"),
    ("recovery", f"{ENGINE}/roughtime/round/recovery.rs"),
    ("disclosure", f"{ENGINE}/roughtime/round/recovery/disclosure.rs"),
    ("proof", f"{ENGINE}/roughtime/round/recovery/proof.rs"),
    ("interval", f"{ENGINE}/roughtime/interval.rs"),
)
SCOPE: COMMON.Scope = COMMON.Scope(PACKAGE, FILES, COMMON.ENGINE_ROUGHTIME_INVENTORY_POLICY)
LABEL: str = "mutation-enumeration:verifier"


def listing_command(filename: str) -> list[str]:
    """Specify the expected argv independently of the analyzer builder."""
    return [
        "/owned/tools/cargo-mutants", "mutants", "--package", "nevrotick-engine",
        "--file", filename, "--test-workspace", "false", "--cap-lints", "false",
        "--jobs", "1", "--jobserver", "true", "--jobserver-tasks", "1",
        "--features", "roughtime-verification", "--list", "--json",
    ]


def descriptor(filename: str) -> dict[str, object]:
    """Build an independently specified shape for an in-function listing row."""
    return {
        "package": PACKAGE, "file": filename, "name": f"{filename}:4:3: fixture descriptor",
        "span": {"start": {"line": 4, "column": 3}, "end": {"line": 4, "column": 4}},
        "function": {"function_name": "fixture", "return_type": "",
                     "span": {"start": {"line": 3, "column": 1}, "end": {"line": 5, "column": 1}}},
        "genre": "UnaryOperator", "replacement": "", "diff": "fixture diff",
    }


class RoughtimeInventoryTests(unittest.TestCase):
    """Exercise the common admission, command, and result boundaries."""

    def setUp(self) -> None:
        self.policy: dict[str, object] = {
            **COMMON.ENGINE_ROUGHTIME_INVENTORY_FACTS,
            "candidate_sha256": "a" * 64,
            "line_policy_sha256": "b" * 64,
        }
        self.arguments: dict[str, str] = {
            "profile": "roughtime", "lane": "enumerate", "subject": "all",
            "tool_admission_profile": "stable-core",
        }

    def test_cli_admission_requires_exact_listing_role(self) -> None:
        args: argparse.Namespace = argparse.Namespace(**self.arguments)
        COMMON.require_roughtime_inventory_role(SCOPE, self.policy)
        self.assertEqual(COMMON.admission_profile(args, SCOPE, self.policy), "stable-core")
        for key, value in (
            ("lane", "mutation"), ("lane", "coverage"), ("lane", "all"),
            ("profile", "legacy"), ("subject", "verifier"), ("tool_admission_profile", "full"),
        ):
            changed: argparse.Namespace = argparse.Namespace(**{**self.arguments, key: value})
            with self.subTest(key=key, value=value), self.assertRaises(COMMON.Incomplete):
                COMMON.admission_profile(changed, SCOPE, self.policy)

    def test_role_keeps_eleven_owners_and_campaign_closed(self) -> None:
        wrong_scopes: tuple[COMMON.Scope, ...] = (
            SCOPE._replace(subjects=FILES[:-1]),
            SCOPE._replace(policy=COMMON.ENGINE_ROUGHTIME_LINE_POLICY),
            SCOPE._replace(package="ntp-wire"),
        )
        for scope in wrong_scopes:
            with self.subTest(scope=scope), self.assertRaisesRegex(COMMON.Incomplete, "exact stable listing-only role"):
                COMMON.require_roughtime_inventory_role(scope, self.policy)
        for key, value in (
            ("campaign_admission", True), ("campaign_admission", 0),
            ("mutation_minimum_viable_killed_percent", 94),
            ("feature", "roughtime-verification,nts-ke-association"),
            ("candidate_sha256", "a" * 63), ("line_policy_sha256", "g" * 64),
            ("mutation_inventories", {}),
        ):
            with self.subTest(key=key, value=value), self.assertRaisesRegex(
                    COMMON.Incomplete, "exact stable listing-only role"):
                COMMON.require_roughtime_inventory_role(SCOPE, {**self.policy, key: value})

    def test_inventory_requires_the_exact_line_policy_bytes(self) -> None:
        with tempfile.TemporaryDirectory() as directory:
            root: Path = Path(directory)
            (root / "policy").mkdir()
            for filename in (SCOPE.policy, COMMON.ENGINE_ROUGHTIME_LINE_POLICY,
                             "policy/verification-manifest.toml"):
                shutil.copyfile(ROOT / filename, root / filename)
            inventory: dict[str, object] = ENGINE_ANALYSIS.read_roughtime_inventory_policy(root)
            line_policy: Path = root / COMMON.ENGINE_ROUGHTIME_LINE_POLICY
            self.assertEqual(inventory["candidate_sha256"], json.loads(line_policy.read_bytes())["candidate_sha256"])
            line_policy.write_bytes(line_policy.read_bytes() + b"\n")
            with self.assertRaisesRegex(COMMON.Incomplete, "different candidates or policies"):
                ENGINE_ANALYSIS.read_roughtime_inventory_policy(root)

            line_policy.write_bytes((ROOT / COMMON.ENGINE_ROUGHTIME_LINE_POLICY).read_bytes())
            policy_file: Path = root / SCOPE.policy
            policy: dict[str, object] = json.loads(policy_file.read_bytes())
            policy["candidate_sha256"] = "f" * 64
            policy_file.write_text(json.dumps(policy))
            with self.assertRaisesRegex(COMMON.Incomplete, "different candidates or policies"):
                ENGINE_ANALYSIS.read_roughtime_inventory_policy(root)

    def test_exact_listing_arguments_at_shared_effect_boundary(self) -> None:
        for subject, filename in FILES:
            with self.subTest(subject=subject):
                COMMON.require_roughtime_listing_command(
                    SCOPE, self.policy, TOOLS, f"mutation-enumeration:{subject}", listing_command(filename))
        valid: list[str] = listing_command(FILES[0][1])
        cases: dict[str, tuple[str, list[str]]] = {
            "list option removed": (LABEL, valid[:-2] + ["--json"]),
            "wider feature list": (LABEL, valid[:-3] + ["roughtime-verification,nts-ke-association"] + valid[-2:]),
            "wrong owner": (LABEL, listing_command(FILES[1][1])),
            "unknown subject": ("mutation-enumeration:library", valid),
            "campaign label": ("mutation-campaign:verifier", valid),
            "additional argument": (LABEL, valid + ["--all-features"]),
            "alternate executable": (LABEL, ["/elsewhere/cargo-mutants", *valid[1:]]),
        }
        for case, (label, command) in cases.items():
            with self.subTest(case=case), self.assertRaisesRegex(COMMON.Incomplete, "exact inventory command"):
                COMMON.require_roughtime_listing_command(SCOPE, self.policy, TOOLS, label, command)
        with self.assertRaisesRegex(COMMON.Incomplete, "grants no mutation campaign"):
            COMMON.mutation_campaign_command(SCOPE, TOOLS, "verifier", valid, policy=self.policy)

    def test_session_rejects_list_to_campaign_substitution_before_effect(self) -> None:
        session: COMMON.Session = object.__new__(COMMON.Session)
        session.scope = SCOPE
        session.policy = self.policy
        session.tools = TOOLS
        session.check = Mock(side_effect=RuntimeError("custody reached in synthetic control"))
        valid: list[str] = listing_command(FILES[0][1])
        with self.assertRaisesRegex(RuntimeError, "custody reached"):
            session.run(LABEL, valid, 120)
        session.check.assert_called_once_with()
        session.check.reset_mock()

        with self.assertRaisesRegex(COMMON.Incomplete, "exact inventory command"):
            session.run(LABEL, valid[:-2] + ["--json"], 120)
        with self.assertRaisesRegex(COMMON.Incomplete, "alternate command modes and budgets"):
            session.run(LABEL, valid, 120, mutation_subject="verifier")
        with self.assertRaisesRegex(COMMON.Incomplete, "alternate command modes and budgets"):
            session.run(LABEL, valid, 121)
        session.check.assert_not_called()
        session.listing_deadline = time.monotonic() - 1.0
        session.check.side_effect = None
        with self.assertRaisesRegex(COMMON.Incomplete, "wall-time budget is exhausted"):
            session.run(LABEL, valid, 120)
        session.check.assert_called_once_with()

    def test_enumeration_lists_each_exact_owner_and_retains_observations(self) -> None:
        calls: list[tuple[str, list[str]]] = []
        report: dict[str, object] = {}
        descriptors: dict[str, dict[str, object]] = {subject: descriptor(filename) for subject, filename in FILES}
        descriptors["request"]["function"] = None

        def run(label: str, command: list[str], timeout: int) -> str:
            self.assertTrue(0 < timeout <= 120)
            calls.append((label, command))
            return json.dumps([descriptors[label.removeprefix("mutation-enumeration:")]])

        result: COMMON.MutationEnumeration = COMMON.enumerate_mutation_inventories(
            SCOPE, dict(FILES), self.policy, TOOLS, run, report,
            deadline=time.monotonic() + 120, maximum_per_file=512, maximum_total=2048)
        self.assertEqual(calls, [
            (f"mutation-enumeration:{subject}", listing_command(filename)) for subject, filename in FILES
        ])
        self.assertEqual(result.inventories, {
            subject: {descriptor["name"]: descriptor} for subject, descriptor in descriptors.items()
        })
        self.assertEqual(report, {
            "mutation_enumeration": {
                subject: {"raw_output": json.dumps([descriptor]), "warning": None}
                for subject, descriptor in descriptors.items()
            },
            "mutation_enumerated_total": 11,
            "mutation_campaign_budget_exceeded": False,
        })

    def test_rejects_foreign_and_duplicate_descriptors_through_listing(self) -> None:
        subject, filename = FILES[0]
        valid: dict[str, object] = descriptor(filename)
        damaged: list[list[dict[str, object]]] = [
            [], [valid, valid], [{**valid, "file": FILES[1][1]}],
            [{"package": PACKAGE, "file": filename, "name": valid["name"]}],
            [{**valid, "name": f"{filename}:6:3: wrong location"}],
            [{**valid, "span": {"start": {"line": 4, "column": 3}, "end": {"line": 6, "column": 1}}}],
            [{**valid, "function": "unexpected function record"}],
        ]
        for items in damaged:
            with self.subTest(items=items), self.assertRaises(COMMON.Incomplete):
                COMMON.enumerate_mutation_inventories(
                    SCOPE, {subject: filename}, self.policy, TOOLS,
                    lambda _label, _command, _timeout: json.dumps(items), {},
                    deadline=time.monotonic() + 120, maximum_per_file=512, maximum_total=2048)


if __name__ == "__main__":
    unittest.main()
