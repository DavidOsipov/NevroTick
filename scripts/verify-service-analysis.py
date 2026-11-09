#!/usr/bin/env python3
"""Measure three frozen portable service owners through the existing analysis custodian.

GPL-2.0-only. No acquisition, host Cargo fallback, native Windows execution,
formal work, error substitutions or mutation exclusions are admitted here.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path, PurePosixPath
import re
import sys
import time
import tomllib

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("service_analysis_shared", ROOT / "scripts/portable-analysis.py")
COMMON = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COMMON)
Incomplete = COMMON.Incomplete
POLICY = "policy/service-analysis.json"
WRAPPER = "scripts/verify-service-analysis.py"
CONTROLS = "scripts/tests/test_verify_service_analysis.py"
SOURCE = "crates/windows-platform/src/service.rs"
TEST_SOURCE = "crates/windows-platform/tests/service.rs"
SCM_SOURCE = "crates/windows-platform/src/scm.rs"
SCM_TEST_SOURCE = "crates/windows-platform/tests/scm.rs"
PRIVILEGE_SOURCE = "crates/windows-platform/src/privilege_observation.rs"
PRIVILEGE_TEST_SOURCE = "crates/windows-platform/tests/privilege_observation.rs"
PRIVILEGE_CONTRACT_SOURCE = "crates/windows-platform/tests/privilege_observation_contracts.rs"
SUBJECTS = {"lifecycle": SOURCE, "scm": SCM_SOURCE, "privilege": PRIVILEGE_SOURCE}
ATTRIBUTED_SOURCES = (SOURCE, TEST_SOURCE, SCM_SOURCE, SCM_TEST_SOURCE,
                      PRIVILEGE_SOURCE, PRIVILEGE_TEST_SOURCE, PRIVILEGE_CONTRACT_SOURCE)
COMPILER_TARGETS = {"windows_platform", "service", "scm", "privilege_observation", "privilege_observation_contracts"}
SCOPE = COMMON.Scope("windows-platform", tuple(SUBJECTS.items()), POLICY)
BOUND_INPUTS = (
    "Cargo.toml", "Cargo.lock", "crates/windows-platform/Cargo.toml",
    "crates/windows-platform/src/lib.rs", *ATTRIBUTED_SOURCES, WRAPPER, CONTROLS,
    "scripts/portable-analysis.py", "scripts/tests/test_portable_analysis.py",
    "scripts/analysis-runtime.py", "scripts/check-analysis.py", "scripts/analysis-controls.py",
    "scripts/prepare-product.py", "scripts/verify.py", "scripts/slint_dependency_profile.py",
    "policy/verification-manifest.toml", "policy/analysis-tools.json", "policy/tool-manifest.json",
    "policy/ui-m0-dependencies.json", ".cargo/config.toml", ".cargo/mutants.toml",
)
FIXED_POLICY = {
    "schema_version": 5, "package": "windows-platform", "subjects": SUBJECTS,
    "critical_files": list(SUBJECTS.values()), "tool_admission_profile": "stable-core",
    "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
    "coverage_wall_seconds": 300, "enumeration_wall_seconds": 120,
    "mutation_wall_seconds": 1200, "mutation_build_seconds": 90,
    "mutation_test_seconds": 60, "jobs": 6, "test_threads": 1,
    "coverage_test_threads": 1, "mutation_jobs": 4, "mutation_process_limit": 512,
    "mutation_message_format": "json", "maximum_mutants": 512,
    "mutation_equivalences": [],
}
HEX = re.compile(r"[a-f0-9]{64}\Z")
REMAINING_QUALIFICATION = [
    "Independent source/function/guard reconciliation against the complete generated descriptor inventory, including omitted functions.",
    "Independent semantic and Garde-derived validation controls for behavior outside generated descriptors.",
]


def validate_policy(policy: dict, *, require_binding: bool = True, campaign: bool = False) -> dict:
    """Reject arbitrary subjects, weakened gates, exceptions and incomplete freezes."""
    if (type(policy) is not dict or set(policy) != set(FIXED_POLICY) | {"binding", "mutation_inventories"}
            or any(type(policy.get(k)) is not type(v) or policy[k] != v for k, v in FIXED_POLICY.items())):
        raise Incomplete("Service analysis requires its exact source-only scope, gates and budgets.")
    binding = policy["binding"]
    if binding is None:
        if require_binding:
            raise Incomplete("Service analysis is not bound to a frozen product candidate.")
    elif (type(binding) is not dict
          or set(binding) != {"source_file_count", "candidate_sha256", "inputs_sha256", "ordinary_tests", "doctests"}
          or type(binding["source_file_count"]) is not int or binding["source_file_count"] <= 0
          or type(binding["candidate_sha256"]) is not str or HEX.fullmatch(binding["candidate_sha256"]) is None
          or type(binding["inputs_sha256"]) is not dict or set(binding["inputs_sha256"]) != set(BOUND_INPUTS)
          or any(type(value) is not str or HEX.fullmatch(value) is None for value in binding["inputs_sha256"].values())):
        raise Incomplete("Service candidate/source binding is malformed or incomplete.")
    if binding is not None:
        for kind in ("ordinary_tests", "doctests"):
            names = binding[kind]
            if (type(names) is not list or not 0 < len(names) <= 512
                    or any(type(name) is not str or not 0 < len(name) <= 4096 or "\n" in name or "\r" in name for name in names)
                    or sorted(set(names)) != names):
                raise Incomplete("Service test inventory must be sorted, unique and nonempty.")
        if set(binding["ordinary_tests"]) & set(binding["doctests"]):
            raise Incomplete("Service ordinary and doctest identities overlap.")
    inventories = policy["mutation_inventories"]
    if type(inventories) is not dict or (inventories and set(inventories) != set(SUBJECTS)):
        raise Incomplete("Service descriptor review must cover all three exact portable subjects.")
    if inventories:
        for record in inventories.values():
            if (type(record) is not dict or set(record) != {"count", "sha256"}
                    or type(record["count"]) is not int or not 0 < record["count"] <= 512
                    or type(record["sha256"]) is not str or HEX.fullmatch(record["sha256"]) is None):
                raise Incomplete("Service reviewed descriptors require per-file nonempty counts and exact hashes.")
        if sum(record["count"] for record in inventories.values()) > 512:
            raise Incomplete("Service reviewed descriptors exceed the combined 512-variant budget.")
    if campaign and (binding is None or not inventories):
        raise Incomplete("Service mutation requires reviewed and bound actual descriptors first.")
    return policy


def read_policy(root: Path, *, campaign: bool) -> dict:
    """Bind the actual full product and protected helper bytes before any Session."""
    if any(name in {"RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTDOCFLAGS", "CARGO_ENCODED_RUSTDOCFLAGS"}
           or (name.startswith("CARGO_TARGET_") and name.endswith(("_RUSTFLAGS", "_RUSTDOCFLAGS")))
           for name in os.environ):
        raise Incomplete("Service analysis forbids ambient compiler flag overrides.")
    policy = validate_policy(COMMON.read_analysis_json(root / POLICY), campaign=campaign)
    binding = policy["binding"]
    _, identity = COMMON.PRODUCT.admit_product(root)
    if (len(identity) != binding["source_file_count"]
            or COMMON.canonical_sha256(identity) != binding["candidate_sha256"]
            or any(COMMON.PRODUCT.sha256(root / name) != digest for name, digest in binding["inputs_sha256"].items())):
        raise Incomplete("Service candidate, source or analysis helpers changed after freezing.")
    manifest = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    for source in SUBJECTS.values():
        if (manifest.get("critical_files", []).count(source) != 1
                or sum(module.get("package") == SCOPE.package and source in module.get("files", [])
                       for module in manifest.get("security_modules", [])) != 1):
            raise Incomplete("Service analysis requires all three exact critical/security owner registrations.")
    return policy


def validate_execution(text: str, policy: dict, *, doctests: bool) -> dict:
    """Require every named test and zero skipped, filtered, failed or duplicate cases."""
    expected = policy["binding"]["ordinary_tests"] + (policy["binding"]["doctests"] if doctests else [])
    names = []
    for line in text.splitlines():
        if line.startswith("test ") and not line.startswith("test result:"):
            match = re.fullmatch(r"test ([^\r\n]+?) \.\.\. ok", line)
            if match is None:
                raise Incomplete("Service native completion lacks its named successful test.")
            names.append(match[1])
    summaries = [line for line in text.splitlines() if line.startswith("test result:")]
    matches = [re.fullmatch(r"test result: ok\. (\d+) passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;.*", line)
               for line in summaries]
    if (not summaries or any(match is None for match in matches)
            or sorted(names) != sorted(expected) or len(set(names)) != len(names)
            or sum(int(match[1]) for match in matches if match is not None) != len(names)):
        raise Incomplete("Service execution omits its complete bound test/doctest inventory.")
    return {"tests_executed": len(names), "test_names": sorted(names), "doctests_included": doctests}


def owned_log(directory: Path, value: str) -> bytes:
    """Read only a bounded relative log under the campaign's owned output."""
    relative = COMMON.analysis_output_path(value)
    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
        raise Incomplete("Service mutation log escapes its owned output.")
    return COMMON.read_analysis_bytes(directory / relative, maximum=16 * 1024 * 1024)


def selected_span(value: str) -> bool:
    """Recognize only selected source/test paths in the owned mutation namespace."""
    if type(value) is not str or ".." in PurePosixPath(value).parts:
        return False
    return value in ATTRIBUTED_SOURCES or (value.startswith("/work/")
        and any(value.endswith("/" + selected) for selected in ATTRIBUTED_SOURCES))


def validate_phases(outcome: dict) -> None:
    """Keep the shared phase/status contract and require the fixed JSON diagnostic row."""
    COMMON.mutation_phases(SCOPE, outcome)
    if any("--message-format=json" not in phase["argv"] for phase in outcome["phase_results"]):
        raise Incomplete("Service mutation phases must use the fixed JSON diagnostic format.")


def validate_causes(directory: Path, outcomes: dict, policy: dict, *, subject: str = "lifecycle") -> dict:
    """Separate owned behavioral failures from actual compiler-unviable variants."""
    if subject not in SUBJECTS:
        raise Incomplete("Service mutation causes require one exact portable subject.")
    expected_file = SUBJECTS[subject]
    rows = outcomes.get("outcomes")
    if type(rows) is not list or not 1 <= len(rows) <= 513 or any(type(row) is not dict for row in rows):
        raise Incomplete("Service mutation causes lack their bounded outcome records.")
    checked = {}
    for row in rows:
        validate_phases(row)
        summary = row.get("summary")
        if summary not in {"CaughtMutant", "Unviable"}:
            continue
        scenario = row.get("scenario")
        mutant = scenario.get("Mutant") if type(scenario) is dict else None
        if (type(mutant) is not dict or mutant.get("package") != SCOPE.package or mutant.get("file") != expected_file
                or type(mutant.get("name")) is not str or not 0 < len(mutant["name"]) <= 4096
                or mutant["name"] in checked):
            raise Incomplete("Service failure cause has a duplicate or out-of-scope identity.")
        data = owned_log(directory, row.get("log_path", ""))
        text = data.decode("utf-8")
        if re.search(r"HARNESS_INCOMPLETE|Cannot allocate memory|memory allocation of [^\n]*failed|"
                     r"No space left on device|No such file or directory|Permission denied|"
                     r"failed to (?:run custom build command|execute process)|could not execute process|"
                     r"internal compiler error", text):
            raise Incomplete("Service failure contains an infrastructure or incomplete-harness cause.")
        diagnostics = []
        for line in text.splitlines():
            if line.startswith('{"reason":') and '"compiler-message"' in line:
                try:
                    diagnostic = json.loads(line)
                except (ValueError, RecursionError) as error:
                    raise Incomplete("Service compiler diagnostic is malformed.") from error
                if (type(diagnostic) is not dict or type(diagnostic.get("message")) is not dict
                        or type(diagnostic.get("target")) is not dict
                        or type(diagnostic["message"].get("spans")) is not list
                        or any(type(span) is not dict for span in diagnostic["message"]["spans"])):
                    raise Incomplete("Service compiler diagnostic lacks its typed source record.")
                diagnostics.append(diagnostic)
        errors = [item for item in diagnostics if item.get("message", {}).get("level") == "error"]
        if summary == "Unviable":
            causes = []
            for item in errors:
                message = item["message"]
                primary = [span for span in message["spans"] if span.get("is_primary") is True]
                if (item["target"].get("name") not in COMPILER_TARGETS
                        or any(not selected_span(span.get("file_name")) for span in primary)):
                    raise Incomplete("Service unviable output includes a foreign compiler error.")
                if primary:
                    causes.append(message)
                elif (message["spans"] or type(message.get("message")) is not str
                      or re.fullmatch(r"aborting due to \d+ previous errors?(?:; \d+ warnings? emitted)?", message["message"]) is None):
                    raise Incomplete("Service unviable output includes an unattributed compiler error.")
            for line in text.splitlines():
                if line.startswith("error:") and re.fullmatch(
                        r"error: could not compile `windows-platform`(?: \([^\n)]+\))? due to \d+ previous errors?(?:; \d+ warnings? emitted)?", line) is None:
                    raise Incomplete("Service unviable output includes an unclassified native error.")
            if not causes:
                raise Incomplete("Service unviable variant lacks its actual selected compiler diagnostic.")
            checked[mutant["name"]] = {"kind": "compiler-unviable", "log_sha256": hashlib.sha256(data).hexdigest(),
                                      "diagnostics": len(causes), "kill_credit": 0}
            continue
        if errors or re.search(r"error\[E\d+\]|could not compile|HARNESS_INCOMPLETE", text):
            raise Incomplete("Service caught variant contains compiler or incomplete-harness failure.")
        failures = []
        for match in re.finditer(r"^---- ([^\r\n]+) stdout ----\n(.*?)(?=^---- |^failures:|\Z)", text, re.MULTILINE | re.DOTALL):
            name, body = match.groups()
            if name not in policy["binding"]["ordinary_tests"] or f"test {name} ... FAILED" not in text:
                continue
            assertion = re.search(r"panicked at ([^\n]+?):(\d+):\d+:\nassertion[^\n]*failed", body)
            if assertion is not None and selected_span(assertion[1]):
                failures.append({"test": name, "kind": "assertion", "source": assertion[1], "line": int(assertion[2])})
            elif (re.search(r"^Error: (?:InvalidTransition|ReportingFailed|OutstandingWork|DeadlineElapsed|DeadlineOverflow|TimeReversed|BudgetError|WorkLimit|CheckpointOverflow|FailureOutcomeRequired|Report\(|Bounds|Length|DuplicatePrivilege|DuplicateTargetIds|UnsupportedAttributes|RemovedEntry|EnginePrivilegeError)\b", body, re.MULTILINE)
                  or re.search(
                      r"^Error: (?:Forbidden\(EnginePrivilegeError \{ privilege: (?:SystemTime|Debug|Impersonate), "
                      r"state: (?:PresentDisabled|PresentEnabled) \}\)|MissingRequired\((?:ChangeNotify|SystemTime)\)"
                      r"|UnexpectedPrivilege|BrokerClockEnabled)$", body, re.MULTILINE)):
                failures.append({"test": name, "kind": "typed-positive-path-refusal"})
        if not failures:
            raise Incomplete("Service caught variant lacks an actual selected behavioral test cause.")
        checked[mutant["name"]] = {"kind": "behavioral", "log_sha256": hashlib.sha256(data).hexdigest(), "causes": failures}
    return {"failures": checked, "compiler_or_harness_kill_credit": 0}


def validate_baseline(work: Path, policy: dict, *, subject: str = "lifecycle") -> dict:
    """Check complete actual baseline execution and per-variant causes before scoring."""
    if subject not in SUBJECTS:
        raise Incomplete("Service baseline requires one exact portable subject.")
    directory = work / "mutation" / subject / "mutants.out"
    outcomes = COMMON.read_analysis_json(directory / "outcomes.json")
    rows = outcomes.get("outcomes")
    if type(rows) is not list or any(type(row) is not dict for row in rows):
        raise Incomplete("Service baseline lacks actual outcome records.")
    baselines = [row for row in rows if row.get("scenario") == "Baseline"]
    if len(baselines) != 1 or baselines[0].get("summary") != "Success":
        raise Incomplete("Service mutation requires one successful actual baseline.")
    validate_phases(baselines[0])
    data = owned_log(directory, baselines[0].get("log_path", ""))
    return {**validate_execution(data.decode("utf-8"), policy, doctests=True),
            "log_sha256": hashlib.sha256(data).hexdigest(), "causes": validate_causes(directory, outcomes, policy, subject=subject)}


def execute(args, report: dict) -> None:
    """Run only the fixed lane through unchanged source/tool/namespace custody."""
    if args.lane not in {"enumerate", "coverage", "mutation", "all"}:
        raise Incomplete("Unknown service analysis lane.")
    policy = read_policy(ROOT, campaign=args.lane in {"mutation", "all"})
    session_args = argparse.Namespace(**vars(args))
    if args.lane == "enumerate":
        session_args.lane = "mutation"
    session = COMMON.Session(ROOT, session_args, report, SCOPE, policy, BOUND_INPUTS)
    report.update(subjects=SUBJECTS, candidate_sha256=policy["binding"]["candidate_sha256"],
                  qualification_passed=False, metrics_passed=False, campaign_admission=False,
                  remaining_independent_qualification=list(REMAINING_QUALIFICATION),
                  generation_gap_controls="Function/guard and Garde-derived coverage require separate source/inventory reconciliation.")

    def run(label, command, timeout, **options):
        """Retain exact observed execution and real failure causes before scoring."""
        result = session.run(label, command, timeout, **options)
        if label == "coverage":
            report["coverage_execution"] = validate_execution(result, policy, doctests=False)
        elif label.startswith("mutation-campaign:"):
            subject = label.removeprefix("mutation-campaign:")
            report.setdefault("baseline_execution", {})[subject] = validate_baseline(session.work, policy, subject=subject)
        return result

    try:
        binding = policy["binding"]
        if (len(session.product_identity) != binding["source_file_count"]
                or COMMON.canonical_sha256(session.product_identity) != binding["candidate_sha256"]
                or any(COMMON.PRODUCT.sha256(session.source / name) != digest for name, digest in binding["inputs_sha256"].items())
                or COMMON.read_analysis_json(session.source / POLICY) != policy):
            raise Incomplete("Captured service candidate or protected helpers differ from the frozen policy.")
        if args.lane == "enumerate":
            listing = COMMON.enumerate_mutation_inventories(SCOPE, SUBJECTS, policy, session.tools, run, report,
                deadline=time.monotonic() + policy["enumeration_wall_seconds"], maximum_per_file=512, maximum_total=512)
            if set(listing.inventories) != set(SUBJECTS):
                raise Incomplete("Service enumeration omitted an exact portable subject.")
            report["listed_inventories"] = listing.inventories
            report["listed_inventory_bindings"] = {
                name: {"count": len(items), "sha256": COMMON.canonical_sha256({
                    key: {field: value for field, value in item.items() if field != "diff"} for key, item in items.items()})}
                for name, items in listing.inventories.items()}
        else:
            report["campaign_admission"] = args.lane in {"mutation", "all"}
            COMMON.run_analysis_lanes(SCOPE, SUBJECTS, policy, session.work, session.tools, run, report, args.lane)
    finally:
        session.finish()
    if args.lane != "enumerate":
        report["metrics_passed"] = True


def main(argv=None) -> int:
    """Report incomplete preparation honestly and retain owned execution receipts."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", choices=("enumerate", "coverage", "mutation", "all"), required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--vendor-dir", type=Path)
    args = parser.parse_args(argv)
    args.tool_admission_profile = "stable-core"
    report = {"status": "incomplete", "scope": "portable service lifecycle " + args.lane,
              "qualification_passed": False, "metrics_passed": False,
              "remaining_independent_qualification": list(REMAINING_QUALIFICATION)}
    try:
        if args.vendor_dir is None:
            args.vendor_dir = Path(json.loads(COMMON.PRODUCT.POINTER.read_text())["vendor_dir"])
        execute(args, report)
        report["status"] = "passed"
    except (Incomplete, COMMON.PRODUCT.ENV.Incomplete, OSError, ValueError, KeyError, TypeError, StopIteration) as error:
        report["qualification_passed"] = False
        report["metrics_passed"] = False
        report["error"] = str(error)
    if report.get("owned_output") == str(args.output_dir):
        (args.output_dir / "verification.json").write_text(json.dumps(report, indent=2) + "\n")
    print(report["status"].upper() + ": " + report["scope"], flush=True)
    if report.get("error"):
        print(report["error"], file=sys.stderr)
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
