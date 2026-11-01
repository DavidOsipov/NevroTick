#!/usr/bin/env python3
"""Bounded host engine coverage and mutation; enumeration grants no qualification.

GPL-2.0-only. Reuse the admitted shared analysis namespace, tools, source custody
and native outcome accounting. No acquisition, new graph or profile exception.
"""
from __future__ import annotations

import argparse
import importlib.util
import json
import math
from pathlib import Path
import re
import sys
import time
import tomllib
from typing import NoReturn

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("engine_analysis_shared", ROOT / "scripts/portable-analysis.py")
COMMON = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COMMON)
PRODUCT = COMMON.PRODUCT
Incomplete = COMMON.Incomplete
POLICY = "policy/engine-analysis.json"
WRAPPER = "scripts/verify-engine-analysis.py"
TESTS = "scripts/tests/test_verify_engine_analysis.py"
SUBJECTS = {
    "library": "crates/nevrotick-engine/src/lib.rs",
    "cli": "crates/nevrotick-engine/src/bin/nevrotick-observe.rs",
}
SCOPE = COMMON.Scope("nevrotick-engine", tuple(SUBJECTS.items()), POLICY)
BOUND_INPUTS = (
    "Cargo.toml", "Cargo.lock", "crates/nevrotick-engine/Cargo.toml",
    *SUBJECTS.values(), "crates/nevrotick-engine/tests/runtime_boundary.rs",
    "crates/nevrotick-engine/tests/prepare_release.rs", "crates/nevrotick-engine/tests/prepare_entered.rs",
    "crates/nevrotick-engine/tests/execute_entered.rs",
    "crates/nevrotick-engine/tests/private/processing_guards.rs",
    "scripts/portable-analysis.py", "scripts/analysis-runtime.py", "scripts/check-analysis.py",
    "scripts/prepare-product.py", "policy/analysis-tools.json", "policy/verification-manifest.toml",
)
REQUIRED_TESTS = frozenset({
    "processing_guard_tests::processing_uses_fresh_deadline_and_current_control",
    "processing_guard_tests::terminal_io_causes_and_send_history_remain_distinct",
    "prepare_outside_runtime_owns_and_releases_standard_sockets",
    "prepare_rejects_entered_runtime_before_native_startup",
    "execute_rejects_entered_runtime_without_nested_runtime_creation",
    "cli_worker_binary_preserves_literal_transport_contracts",
    "cli_worker_binary_rejects_arguments_and_observes_sink_failures",
})
FIXED_POLICY = {
    "schema_version": 1, "package": "nevrotick-engine",
    "critical_files": list(SUBJECTS.values()),
    "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
    "maximum_mutants": 512, "maximum_listed_per_file": 4096, "maximum_listed_total": 8192,
    "coverage_wall_seconds": 300, "enumeration_wall_seconds": 240,
    "mutation_wall_seconds": 1200, "mutation_build_seconds": 90, "mutation_test_seconds": 60,
    "jobs": 12, "test_threads": 1, "coverage_test_threads": 1,
    "mutation_jobs": 10, "mutation_process_limit": 512, "mutation_message_format": "json",
    "tool_admission_profile": "full", "mutation_equivalences": [],
}


def invalid_number(value: str) -> NoReturn:
    """Reject nonfinite tokens at every depth of policy and native report input."""
    raise Incomplete("Engine JSON contains a nonfinite number.")


def finite_number(value: str) -> float:
    """Retain finite native timing values without accepting exponent overflow."""
    number = float(value)
    if not math.isfinite(number):
        invalid_number(value)
    return number


def read_json(path: Path, *, maximum: int = 524288) -> dict:
    """Read bounded regular, nonlinked JSON; reject duplicates and nonfinite values."""
    if path != path.resolve(strict=True):
        raise Incomplete("Engine JSON input cannot use a linked path.")
    data = COMMON.ANALYSIS.read_once(path, maximum=maximum)
    try:
        value = json.loads(data, object_pairs_hook=COMMON.ANALYSIS.unique_object,
                           parse_float=finite_number, parse_constant=invalid_number)
    except (ValueError, UnicodeError, RecursionError) as error:
        raise Incomplete("Invalid bounded engine JSON input.") from error
    if not isinstance(value, dict):
        raise Incomplete("Engine JSON input must be an object.")
    return value


def inventory_bindings(policy: dict, *, required: bool = False) -> dict:
    """Admit either no review yet or complete exact nonempty per-file review pins."""
    bindings = policy.get("mutation_inventories")
    if not isinstance(bindings, dict):
        raise Incomplete("Engine mutation inventory bindings must be an object.")
    if not bindings and not required:
        return {}
    if set(bindings) != set(SUBJECTS):
        raise Incomplete("Campaign requires reviewed nonempty inventories for both engine files.")
    for binding in bindings.values():
        if (not isinstance(binding, dict) or set(binding) != {"count", "sha256"}
                or type(binding["count"]) is not int or not 0 < binding["count"] <= policy["maximum_mutants"]
                or not isinstance(binding["sha256"], str)
                or not re.fullmatch(r"[a-f0-9]{64}", binding["sha256"])):
            raise Incomplete("Engine inventory review requires exact positive counts and SHA-256 pins.")
    return bindings


def read_policy(root: Path) -> dict:
    """Bind the fixed engine scope to the actual whole product and protected helpers."""
    policy = read_json(root / POLICY)
    extras = {"candidate_sha256", "inputs_sha256", "mutation_inventories", "scope"}
    if (set(policy) != set(FIXED_POLICY) | extras
            or any(type(policy.get(key)) is not type(value) or policy.get(key) != value
                   for key, value in FIXED_POLICY.items())
            or not isinstance(policy["scope"], str) or not 0 < len(policy["scope"]) <= 2048):
        raise Incomplete("Unreviewed engine analysis scope, threshold, budget or profile.")
    _, identity = PRODUCT.admit_product(root)
    if (not isinstance(policy["candidate_sha256"], str)
            or policy["candidate_sha256"] != COMMON.canonical_sha256(identity)):
        raise Incomplete("Engine analysis policy differs from the complete current product candidate.")
    pins = policy["inputs_sha256"]
    if (not isinstance(pins, dict) or set(pins) != set(BOUND_INPUTS)
            or any(not isinstance(digest, str) or not re.fullmatch(r"[a-f0-9]{64}", digest)
                   or PRODUCT.sha256(root / name) != digest for name, digest in pins.items())):
        raise Incomplete("Engine analysis protected source/helper bindings changed.")
    manifest = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    for filename in SUBJECTS.values():
        if (manifest.get("critical_files", []).count(filename) != 1
                or sum(module.get("package") == SCOPE.package and filename in module.get("files", [])
                       for module in manifest.get("security_modules", [])) != 1):
            raise Incomplete("Engine file must have one matching critical/security owner.")
    inventory_bindings(policy)
    COMMON.admit_mutation_equivalences(root, SCOPE, policy)
    return policy


def readiness_proposal() -> dict:
    """Describe a finite single-worker measurement; never execute or admit it."""
    return {
        "kind": "proposal", "executed": False, "outer_workers": 1,
        "inner_cargo_jobs": 1, "test_threads": 1,
        "build_seconds": 90, "test_seconds": 60,
        "commands": [
            ["cargo", "test", "--no-run", "--verbose", "--package=nevrotick-engine@0.0.0", "--frozen"],
            ["cargo", "test", "--verbose", "--package=nevrotick-engine@0.0.0", "--frozen"],
        ],
        "required_observations": ["actual selected tests", "peak allocated worker tree bytes",
                                  "minimum normal-user available bytes", "process custody and actual exits"],
        "ten_worker_readiness": "not assessed; reserve ten measured workers plus stop margin before execution",
        "limits": "existing rlimits and sampled storage guards are not a hard aggregate RAM/disk quota",
    }


def listed_summary(inventories: dict) -> dict:
    """Hash every observed descriptor field except its optional rendered diff."""
    return {name: {"count": len(items), "sha256": COMMON.canonical_sha256({
        key: {field: value for field, value in item.items() if field != "diff"}
        for key, item in items.items()})} for name, items in inventories.items()}


def mutation_prefix(tools: Path, subject: str, policy: dict) -> list[str]:
    """Use the unchanged exact shared campaign validator for the listing prefix."""
    filename = SCOPE.select(subject)[subject]
    prefix = [str(tools / "cargo-mutants"), "mutants", "--package", SCOPE.package,
              "--file", filename, "--test-workspace", "false", "--cap-lints", "false",
              "--jobs", str(policy["mutation_jobs"]), "--jobserver", "true",
              "--jobserver-tasks", str(policy["mutation_jobs"]), "--cargo-arg=--message-format=json"]
    campaign = [*prefix, "--baseline", "run", "--build-timeout", "90", "--timeout", "60",
                "--output", "/work/mutation/" + subject]
    COMMON.mutation_campaign_command(SCOPE, tools, subject, campaign,
                                    expected_jobs=policy["mutation_jobs"], expected_message_format="json")
    return prefix


def enumerate_only(session, subjects: dict, policy: dict, report: dict) -> None:
    """Retain complete bounded listings; launch no baseline or mutation campaign."""
    deadline = time.monotonic() + policy["enumeration_wall_seconds"]
    inventories = {}
    observations = report["mutation_enumeration"] = {}
    for name, filename in subjects.items():
        command = [*mutation_prefix(session.tools, name, policy), "--list", "--json"]
        raw = session.run("mutation-enumeration:" + name, command, COMMON.remaining_timeout(deadline, 120))
        observations[name] = {"raw_output": raw}
        items, warning = COMMON.parse_mutation_listing(raw, expected_jobs=policy["mutation_jobs"])
        observations[name]["warning"] = warning
        inventories[name] = COMMON.mutation_inventory(SCOPE, items, policy["maximum_listed_per_file"], filename)
    total = COMMON.combined_mutant_count(inventories, policy["maximum_listed_total"])
    report["listed_inventories"] = inventories
    report["listed_inventory_bindings"] = listed_summary(inventories)
    report["listed_total"] = total
    report["campaign_cap_exceeded"] = total > policy["maximum_mutants"]
    report["campaign_admission"] = False
    report["qualification_passed"] = False


def validate_test_execution(text: str, *, doctests: bool) -> dict:
    """Require actual successful libtest records, summaries and the required cases."""
    records = re.findall(r"^test ([^\r\n]+?) \.\.\. ok\r?$", text, re.MULTILINE)
    summaries = [line for line in text.splitlines() if line.startswith("test result:")]
    pattern = r"test result: ok\. (\d+) passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;.*"
    counts = [re.fullmatch(pattern, line) for line in summaries]
    if (not summaries or any(match is None for match in counts) or len(records) != len(set(records))
            or sum(int(match[1]) for match in counts if match is not None) != len(records)
            or not REQUIRED_TESTS <= set(records)):
        raise Incomplete("Engine execution omits required actual passing unit/integration records or complete summaries.")
    docs = [name for name in records if re.fullmatch(
        r"(?:[^\r\n]+/)?lib\.rs - (?:Config|ControlSchedule)::new \(line [1-9][0-9]*\)", name)]
    if doctests and (len(docs) != 2 or not all(any(" - " + owner + "::new " in name for name in docs)
                                            for owner in ("Config", "ControlSchedule"))):
        raise Incomplete("Engine mutation baseline omits its two actual construction doctests.")
    if not doctests and docs:
        raise Incomplete("Coverage selection unexpectedly includes doctests.")
    return {"tests_executed": len(records), "required_tests": sorted(REQUIRED_TESTS), "doctests": len(docs)}


def validate_baseline(work: Path, subject: str) -> dict:
    """Inspect the tool's actual source-bound baseline before accepting any score."""
    directory = work / "mutation" / subject / "mutants.out"
    outcomes = read_json(directory / "outcomes.json", maximum=64 * 1024 * 1024)
    baseline = [item for item in outcomes.get("outcomes", [])
                if isinstance(item, dict) and item.get("scenario") == "Baseline"]
    if len(baseline) != 1 or baseline[0].get("summary") != "Success":
        raise Incomplete("Engine mutation requires exactly one successful actual baseline.")
    relative = Path(baseline[0].get("log_path", ""))
    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
        raise Incomplete("Engine baseline log escapes its owned output.")
    path = directory / relative
    if path != path.resolve(strict=True):
        raise Incomplete("Engine baseline log cannot use a linked path.")
    data = COMMON.ANALYSIS.read_once(path, maximum=16 * 1024 * 1024)
    return {**validate_test_execution(data.decode("utf-8"), doctests=True), "log_sha256": PRODUCT.sha256(path)}


def execute(args, report: dict) -> None:
    """Execute only the fixed selected lane, with full shared custody bookends."""
    if args.lane not in {"enumerate", "coverage", "mutation"}:
        raise Incomplete("Unknown engine analysis lane.")
    policy = read_policy(ROOT)
    subjects = SCOPE.select(args.subject)
    if args.lane == "mutation":
        bindings = inventory_bindings(policy, required=True)
        if sum(bindings[name]["count"] for name in subjects) > policy["maximum_mutants"]:
            raise Incomplete("Selected complete mutation inventory exceeds the shared campaign cap.")
    session_args = argparse.Namespace(**vars(args))
    if args.lane == "enumerate":
        session_args.lane = "mutation"  # Select only the admitted extension, not campaign execution.
    report.update(subjects=subjects, candidate_sha256=policy["candidate_sha256"],
                  qualification_passed=False, readiness_proposal=readiness_proposal())
    session = COMMON.Session(ROOT, session_args, report, SCOPE, policy, (WRAPPER, TESTS, *BOUND_INPUTS))

    def run(label, command, timeout, **options):
        result = session.run(label, command, timeout, **options)
        if label == "coverage":
            report["coverage_execution"] = validate_test_execution(result, doctests=False)
        elif label.startswith("mutation-campaign:"):
            subject = label.removeprefix("mutation-campaign:")
            report.setdefault("baseline_execution", {})[subject] = validate_baseline(session.work, subject)
        return result

    try:
        if COMMON.canonical_sha256(session.product_identity) != policy["candidate_sha256"]:
            raise Incomplete("Captured engine Session differs from the policy's product candidate.")
        if any(PRODUCT.sha256(session.source / name) != policy["inputs_sha256"][name]
               for name in BOUND_INPUTS):
            raise Incomplete("Captured engine Session protected input pins changed.")
        if COMMON.canonical_sha256(read_json(session.source / POLICY)) != COMMON.canonical_sha256(policy):
            raise Incomplete("Captured engine Session policy differs from the admitted policy.")
        if args.lane == "enumerate":
            enumerate_only(session, subjects, policy, report)
        else:
            COMMON.run_analysis_lanes(SCOPE, subjects, policy, session.work, session.tools, run, report, args.lane)
    finally:
        session.finish()
    if args.lane != "enumerate":
        report["qualification_passed"] = True


def main(argv=None) -> int:
    """Keep listing completion distinct from selected host coverage/mutation gates."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", choices=("enumerate", "coverage", "mutation"), required=True)
    parser.add_argument("--subject", choices=(*SUBJECTS, "all"), default="all")
    parser.add_argument("--vendor-dir", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    report = {"status": "incomplete", "scope": "Bounded host engine " + args.lane,
              "selected_lane": args.lane, "qualification_passed": False,
              "native_windows_qualification": "not performed"}
    try:
        if args.vendor_dir is None:
            args.vendor_dir = Path(read_json(PRODUCT.POINTER)["vendor_dir"])
        execute(args, report)
        report["status"] = "enumerated" if args.lane == "enumerate" else "passed"
    except (Incomplete, PRODUCT.ENV.Incomplete, OSError, ValueError, KeyError, TypeError, StopIteration) as error:
        report["qualification_passed"] = False
        report["error"] = str(error)
    if report.get("owned_output") == str(args.output_dir):
        (args.output_dir / "verification.json").write_text(json.dumps(report, indent=2, allow_nan=False) + "\n")
    print(report["status"].upper() + ": " + report["scope"], flush=True)
    if report.get("error"):
        print(report["error"], file=sys.stderr)
    return 0 if report["status"] in {"enumerated", "passed"} else 1


if __name__ == "__main__":
    raise SystemExit(main())
