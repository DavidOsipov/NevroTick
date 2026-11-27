"""Controls for the Engine Roughtime runner's coverage evidence boundary."""
from __future__ import annotations

import argparse
from contextlib import redirect_stderr, redirect_stdout
import importlib.util
import io
import json
from pathlib import Path
import shutil
import tempfile
import unittest
from unittest.mock import patch

ROOT: Path = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    "engine_roughtime_line_test", ROOT / "scripts/verify-engine-analysis.py")
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("Engine analysis wrapper is unavailable.")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

LIBRARY: str = "src/lib.rs"
INTEGRATION: str = "tests/roughtime_verification.rs"
ROSTER: dict[str, list[str]] = {
    LIBRARY: [f"roughtime::fixture::unit_{index:02}" for index in range(23)],
    INTEGRATION: [f"fixture::integration_{index:02}" for index in range(60)],
}
LIB_HEADER: str = (
    "     Running unittests src/lib.rs "
    "(/work/target/llvm-cov-target/x86_64-unknown-linux-gnu/debug/build/nevrotick-engine/"
    "0123456789abcdef/out/nevrotick_engine-0123456789abcdef)")
INTEGRATION_HEADER: str = (
    "     Running tests/roughtime_verification.rs "
    "(/work/target/llvm-cov-target/x86_64-unknown-linux-gnu/debug/build/nevrotick-engine/"
    "fedcba9876543210/out/roughtime_verification-fedcba9876543210)")


def libtest_block(header: str, names: list[str], filtered: int) -> str:
    """Construct a literal-shaped transcript without invoking Cargo or libtest."""
    count: int = len(names)
    cases: str = "".join(f"test {name} ... ok\n" for name in reversed(names))
    return (
        f"{header}\n\nrunning {count} tests\n{cases}\n"
        f"test result: ok. {count} passed; 0 failed; 0 ignored; 0 measured; "
        f"{filtered} filtered out; finished in 0.12s\n"
    )


class RoughtimeLineExecutionTests(unittest.TestCase):
    """Check identity and terminal evidence through the actual result reader."""

    def test_reconciles_both_selected_binaries(self) -> None:
        text: str = self.healthy()
        result: dict[str, list[str]] = MODULE.roughtime_line_execution(text, ROSTER)
        self.assertEqual({key: sorted(names) for key, names in result.items()}, ROSTER)

    def test_refuses_identity_and_status_changes(self) -> None:
        text: str = self.healthy()
        unit: str = ROSTER[LIBRARY][0]
        neighbor: str = ROSTER[LIBRARY][1]
        integration: str = ROSTER[INTEGRATION][0]
        cases: dict[str, str] = {
            "same-count substitution": text.replace(unit, "roughtime::fixture::alien", 1),
            "same-count duplicate": text.replace(unit, neighbor, 1),
            "wrong binary": text.replace(unit, "swap_marker", 1)
            .replace(integration, unit, 1).replace("swap_marker", integration, 1),
            "duplicate binary": text.replace(INTEGRATION_HEADER, LIB_HEADER, 1),
            "wrong executable in correct header": text.replace(
                "/out/nevrotick_engine-", "/out/roughtime_verification-", 1),
            "ignored case": text.replace(f"test {unit} ... ok", f"test {unit} ... ignored", 1)
            .replace("23 passed; 0 failed; 0 ignored", "22 passed; 0 failed; 1 ignored", 1),
            "undeclared ignored case": text.replace(
                f"test {unit} ... ok", f"test {unit} ... ok\ntest extra ... ignored", 1),
            "misstated filtered count": text.replace("283 filtered out", "282 filtered out", 1),
        }
        for label, damaged in cases.items():
            with self.subTest(label=label), self.assertRaises(MODULE.Incomplete):
                MODULE.roughtime_line_execution(damaged, ROSTER)

    @staticmethod
    def healthy() -> str:
        return (libtest_block(LIB_HEADER, ROSTER[LIBRARY], 283) + "\n"
                + libtest_block(INTEGRATION_HEADER, ROSTER[INTEGRATION], 0))


class RoughtimeLineAdmissionTests(unittest.TestCase):
    """Check policy, manifest and CLI role isolation without opening a Session."""

    def test_reads_current_policy_and_challenges_threshold_and_manifest(self) -> None:
        scope_path: Path = Path("policy/engine-roughtime-line-analysis.json")
        manifest_path: Path = Path("policy/verification-manifest.toml")
        with tempfile.TemporaryDirectory() as folder:
            root: Path = Path(folder)
            (root / "policy").mkdir()
            policy_file: Path = root / scope_path
            manifest_file: Path = root / manifest_path
            shutil.copyfile(ROOT / scope_path, policy_file)
            shutil.copyfile(ROOT / manifest_path, manifest_file)
            policy, roster = MODULE.read_roughtime_line_policy(root)
            self.assertEqual({name: len(cases) for name, cases in roster.items()},
                             {LIBRARY: 23, INTEGRATION: 60})
            self.assertEqual(policy["line_minimum_percent"], 90)
            self.assertEqual(len(policy["critical_files"]), 10)

            policy_bytes: bytes = policy_file.read_bytes()
            policy_file.write_text(json.dumps({**policy, "line_minimum_percent": 89}))
            with self.assertRaisesRegex(MODULE.Incomplete, "differs from its fixed role"):
                MODULE.read_roughtime_line_policy(root)
            policy_file.write_bytes(policy_bytes)

            manifest: str = manifest_file.read_text()
            threshold: str = "security_module_minimum_executable_line_percent = 90"
            self.assertEqual(manifest.count(threshold), 1)
            manifest_file.write_text(manifest.replace(threshold, threshold[:-2] + "89", 1))
            with self.assertRaisesRegex(MODULE.Incomplete, "registered security line threshold"):
                MODULE.read_roughtime_line_policy(root)
            request_owner: str = '"crates/nevrotick-engine/src/roughtime/request.rs"'
            self.assertEqual(manifest.count(request_owner), 2)
            manifest_file.write_text(manifest.replace(
                request_owner, '"crates/ntp-wire/src/roughtime.rs"', 1))
            with self.assertRaisesRegex(MODULE.Incomplete, "registered security owner"):
                MODULE.read_roughtime_line_policy(root)

    def test_stable_session_role_rejects_wider_or_borrowed_scope(self) -> None:
        args: argparse.Namespace = argparse.Namespace(
            profile="roughtime", subject="all", lane="coverage", tool_admission_profile="stable-core")
        scope = MODULE.ROUGHTIME_LINE_SCOPE
        policy: dict[str, str] = {
            "admission_status": "engine-roughtime-line-coverage-only",
            "tool_admission_profile": "stable-core",
        }
        self.assertEqual(MODULE.COMMON.admission_profile(args, scope, policy), "stable-core")
        for key, value in (("profile", "legacy"), ("subject", "library"), ("lane", "mutation")):
            widened: argparse.Namespace = argparse.Namespace(**{**vars(args), key: value})
            with self.subTest(key=key), self.assertRaisesRegex(MODULE.Incomplete, "exact stable coverage-only role"):
                MODULE.COMMON.admission_profile(widened, scope, policy)
        wrong_scope = scope._replace(policy="policy/engine-analysis.json")
        with self.assertRaisesRegex(MODULE.Incomplete, "exact stable coverage-only role"):
            MODULE.COMMON.admission_profile(args, wrong_scope, policy)

    def test_cli_refuses_widening_before_dispatch(self) -> None:
        base: list[str] = ["--profile", "roughtime", "--output-dir", "/unused/analysis",
                           "--vendor-dir", "/unused/vendor"]
        for options in (["--lane", "mutation", "--tool-admission-profile", "stable-core"],
                        ["--lane", "coverage", "--subject", "library", "--tool-admission-profile", "stable-core"],
                        ["--lane", "coverage"]):
            stderr: io.StringIO = io.StringIO()
            with self.subTest(options=options), patch.object(MODULE, "execute") as execute, redirect_stderr(stderr):
                with self.assertRaises(SystemExit) as refused:
                    MODULE.main([*base, *options])
                self.assertEqual(refused.exception.code, 2)
                self.assertIn("only stable-core coverage", stderr.getvalue())
                execute.assert_not_called()

        with patch.object(MODULE, "execute") as execute, redirect_stdout(io.StringIO()):
            result: int = MODULE.main([*base, "--lane", "coverage", "--tool-admission-profile", "stable-core"])
        self.assertEqual(result, 0)
        execute.assert_called_once()
        admitted: argparse.Namespace = execute.call_args.args[0]
        self.assertEqual((admitted.profile, admitted.subject, admitted.lane, admitted.tool_admission_profile),
                         ("roughtime", "all", "coverage", "stable-core"))


if __name__ == "__main__":
    unittest.main()
