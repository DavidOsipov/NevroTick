"""Independently read back a complete frozen coverage collection without rerunning it.

This read-only recovery stage preserves the original collector's failed result.
It admits the frozen source, tests, tools, raw process data and complete outcomes,
then rebuilds attribution in a fresh output and applies the unchanged line gates.
"""

from __future__ import annotations

import argparse
import configparser
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import platform
import re
import stat
import subprocess
import sys
import time
import zipfile

import coverage
from coverage import Coverage, CoverageData


MAX_JSON_BYTES = 128 * 1024 * 1024


def sha(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        while block := stream.read(1024 * 1024):
            digest.update(block)
    return digest.hexdigest()


def unique_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result: dict[str, object] = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("duplicate-json-key")
        result[key] = value
    return result


def invalid_constant(value: str) -> object:
    raise ValueError("non-finite-json-number")


def read_json(path: Path) -> object:
    descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
    with os.fdopen(descriptor, "rb") as stream:
        info = os.fstat(stream.fileno())
        if not stat.S_ISREG(info.st_mode) or info.st_size > MAX_JSON_BYTES:
            raise ValueError("json-file-type-or-budget")
        raw = stream.read(MAX_JSON_BYTES + 1)
    if len(raw) > MAX_JSON_BYTES:
        raise ValueError("json-byte-budget")
    value = json.loads(raw, object_pairs_hook=unique_object, parse_constant=invalid_constant)
    pending = [value]
    count = 0
    while pending:
        current = pending.pop()
        count += 1
        if count > 1_000_000:
            raise ValueError("json-node-budget")
        if type(current) is dict:
            pending.extend(current.values())
        elif type(current) is list:
            pending.extend(current)
    return value


def write_json(path: Path, value: object) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True, allow_nan=False) + "\n")


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def checked_hashes(root: Path, expected: dict[str, str]) -> dict[str, str]:
    require(type(expected) is dict and bool(expected), "empty-file-identity-closure")
    actual = {}
    for name, digest in expected.items():
        relative = Path(name)
        require(type(name) is str and not relative.is_absolute() and ".." not in relative.parts,
                "invalid-relative-identity-path")
        path = root / relative
        require(not path.is_symlink() and path.is_file(), "identity-file-not-regular:" + name)
        actual[name] = sha(path)
        require(actual[name] == digest, "identity-file-changed:" + name)
    return actual


def module_rows(raw: dict, root: Path, manifest: dict, hashes: dict[str, str]) -> dict:
    files = raw["files"]
    require(type(files) is dict and len(files) == len(manifest["source_modules"]), "raw-report-inventory-mismatch")
    require(raw["meta"]["branch_coverage"] is True, "raw-report-has-no-branch-observations")
    result = {}
    for name in manifest["source_modules"]:
        keys = [key for key in (name, str(root / name)) if key in files]
        require(len(keys) == 1, "missing-or-duplicate-raw-module:" + name)
        observed = files[keys[0]]
        summary = observed["summary"]
        require(all(type(summary[key]) is int for key in ("num_branches", "covered_branches"))
                and 0 <= summary["covered_branches"] <= summary["num_branches"], "invalid-branch-observation:" + name)
        result[name] = {"sha256": hashes[name], "executable_lines": summary["num_statements"],
                        "covered_lines": summary["covered_lines"], "missing_lines": observed["missing_lines"],
                        "branches": summary["num_branches"], "covered_branches": summary["covered_branches"],
                        "missing_branch_lines": observed.get("missing_branch_lines", []),
                        "branch_destination_projection": "Not projected; per-line total/taken branch counts are retained.",
                        "classification": "nonexecuting-marker" if name in manifest["nonexecuting_modules"] else "active-source"}
    return result


def compact_report(measured: Coverage, data: CoverageData, root: Path, manifest: dict) -> dict:
    """Use documented APIs without format-3 per-function context duplication."""
    files = {}
    for name in manifest["source_modules"]:
        filename, statements, excluded, missing, _formatted = measured.analysis2(str(root / name))
        require(not excluded, "unexpected-excluded-source-lines:" + name)
        branches = measured.branch_stats(str(root / name))
        total = sum(pair[0] for pair in branches.values())
        taken = sum(pair[1] for pair in branches.values())
        files[name] = {"summary": {"num_statements": len(statements), "covered_lines": len(statements) - len(missing),
                                    "num_branches": total, "covered_branches": taken},
                       "executed_lines": sorted(set(statements) - set(missing)), "missing_lines": missing,
                       "contexts": {str(line): sorted(contexts) for line, contexts in data.contexts_by_lineno(filename).items()},
                       "branch_stats": {str(line): {"total": pair[0], "taken": pair[1]} for line, pair in branches.items()},
                       "missing_branch_lines": sorted(line for line, pair in branches.items() if pair[0] != pair[1])}
    return {"schema_version": 1, "meta": {"branch_coverage": data.has_arcs(), "version": coverage.__version__,
            "api": "Coverage.analysis2, Coverage.branch_stats, CoverageData.contexts_by_lineno"}, "files": files}


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--measurement", type=Path, required=True)
    parser.add_argument("--snapshot-manifest", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--expected-tests", type=int, default=231)
    args = parser.parse_args()
    started = time.monotonic()
    measurement = args.measurement.resolve()
    frozen = read_json(args.snapshot_manifest)
    root = Path(frozen["root"]).resolve()
    output = args.output.absolute()
    require(not output.is_relative_to(measurement) and not output.is_relative_to(root), "derived-output-must-be-separate")
    output.mkdir(parents=True, exist_ok=False)
    require(root.stat().st_uid == os.getuid() and stat.S_IMODE(root.stat().st_mode) & 0o077 == 0,
            "snapshot-is-not-private-owned-directory")
    require(not any(path.is_symlink() for path in root.rglob("*")), "snapshot-contains-symlink")
    snapshot_files = {path.relative_to(root).as_posix() for path in root.rglob("*") if path.is_file()}
    require(snapshot_files == set(frozen["files"]), "snapshot-file-inventory-changed")
    snapshot_hashes = checked_hashes(root, frozen["files"])
    binding = read_json(measurement / "binding.json")
    require(Path(binding["root"]).resolve() == root, "measurement-snapshot-root-mismatch")
    checked_hashes(root, binding["candidate_hashes"])
    checked_hashes(root, binding["source_hashes"])
    helper = Path(__file__).with_name("coverage.py")
    analysis_sources = {str(path): sha(path) for path in (Path(__file__).resolve(), helper.resolve())}
    spec = importlib.util.spec_from_file_location("nevrotick_coverage_policy", helper)
    policy = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(policy)
    manifest = read_json(root / "metadata/qualification/manifest.json")
    policy.admit_manifest(manifest)
    require(set(binding["source_hashes"]) == set(manifest["source_modules"]), "binding-source-inventory-mismatch")
    require(set(manifest["input_files"]) <= set(binding["candidate_hashes"]), "binding-input-inventory-mismatch")
    require({name for name in snapshot_files if name.startswith("metadata/tests/test_") and name.endswith(".py")}
            <= set(binding["candidate_hashes"]), "binding-test-inventory-mismatch")
    tool = read_json(measurement / "tool-identity.json")
    require(tool["version"] == "7.17.2" and tool["python_implementation"] == "CPython"
            and tool["python_version"].startswith("3.12."), "selected-tool-runtime-mismatch")
    require(coverage.__version__ == tool["version"] and sys.version == tool["python_version"]
            and platform.python_implementation() == "CPython", "actual-analysis-runtime-mismatch")
    require(str(Path(sys.executable).absolute()) == tool["python_path"], "analyzer-must-use-admitted-python")
    require(all(not Path(name).is_symlink() and sha(Path(name)) == digest for name, digest in tool["files"].items()),
            "collector-tool-file-identity-changed")
    require(sha(Path(tool["python_path"]).resolve()) == tool["python_sha256"], "collector-python-identity-changed")
    require(sha(Path(tool["wheel"])) == tool["wheel_sha256"] == policy.WHEEL_SHA256, "collector-wheel-identity-changed")
    package = Path(coverage.__file__).parent
    with zipfile.ZipFile(tool["wheel"]) as archive:
        released = {name: archive.read(name) for name in archive.namelist() if name.startswith("coverage/") and not name.endswith("/")}
        installed = {"coverage/" + path.relative_to(package).as_posix(): path.read_bytes()
                     for path in package.rglob("*") if path.is_file() and "__pycache__" not in path.parts}
        require(released == installed, "actual-coverage-payload-differs-from-pinned-wheel")
        require((package.parent / "a1_coverage.pth").read_bytes() == archive.read("a1_coverage.pth"),
                "coverage-startup-hook-differs-from-pinned-wheel")
    config = configparser.ConfigParser(interpolation=None)
    config.read(measurement / ".coveragerc")
    require(config.getboolean("run", "branch") and config.getboolean("run", "parallel")
            and config.get("run", "patch").strip() == "subprocess"
            and config.get("run", "plugins").strip() == "coverage_identity", "collector-configuration-mismatch")
    require(set(config.get("run", "include").split()) == {"*/" + name for name in manifest["source_modules"]},
            "collector-include-inventory-mismatch")
    require(config.get("report", "exclude_lines").strip() == "" and config.get("report", "partial_branches").strip() == "",
            "collector-report-exclusions-mismatch")
    command = read_json(measurement / "command.json")
    expected_command = [tool["python_path"], "-I", "-B", "-X", "pycache_prefix=" + str(measurement / "empty-pycache"),
                        "-m", "coverage", "run", "--rcfile=" + str(measurement / ".coveragerc"),
                        str(root / "metadata/qualification/coverage_tests.py"), str(root), str(measurement / "test-result.json")]
    require(command == expected_command, "collector-command-mismatch")
    require(not list((measurement / "empty-pycache").rglob("*")), "collector-bytecode-prefix-not-empty")
    outcomes = read_json(measurement / "test-result.json")
    require(type(args.expected_tests) is int and args.expected_tests > 0, "invalid-expected-test-count")
    require(outcomes["successful"] is True and all(type(outcomes[key]) is int and outcomes[key] == args.expected_tests
            for key in ("discovered", "tests_run")), "incomplete-test-discovery-or-execution")
    require(all(type(outcomes[key]) is int and outcomes[key] == 0 for key in
                ("failures", "errors", "skipped", "expected_failures", "unexpected_successes")), "incomplete-test-outcomes")
    require(len(outcomes["test_durations"]) == args.expected_tests, "test-identity-inventory-mismatch")
    execution = read_json(measurement / "execution.json")
    require(type(execution["returncode"]) is int and execution["returncode"] == 0, "collector-execution-failed")
    old_result = read_json(measurement / "coverage-result.json")
    expected_reader_errors = {"line-coverage-below-minimum:" + name for name in manifest["security_modules"]}
    require(set(old_result["errors"]) == expected_reader_errors and old_result["returncode"] == 0
            and all(row["covered_lines"] == 0 for row in old_result["modules"].values()),
            "original-failure-is-not-the-demonstrated-absolute-path-reader-fault")
    ledger_path = measurement / "processes.jsonl"
    require(ledger_path.stat().st_size <= MAX_JSON_BYTES, "process-ledger-byte-budget")
    ledger = [json.loads(line, object_pairs_hook=unique_object, parse_constant=invalid_constant)
              for line in ledger_path.read_text().splitlines()]
    child_pids = {row["pid"] for row in ledger}
    require(len(child_pids) == len(ledger) and all(type(pid) is int and pid > 0 for pid in child_pids), "invalid-child-pid-inventory")
    require(all(row["context"] in outcomes["test_durations"] and "-I" in row["command"] and "-B" in row["command"]
                and "pycache_prefix=" + str(measurement / "empty-pycache") in row["command"] for row in ledger),
            "child-command-or-test-context-mismatch")
    original_files = [path for path in measurement.rglob("*") if path.is_file()]
    original_hashes = {str(path.relative_to(measurement)): sha(path) for path in original_files}
    aggregate = CoverageData(basename=str(output / ".coverage"))
    raw_pids = set()
    admitted_pids = set()
    excluded = []
    raw_ids = []
    for raw_path in sorted(measurement.glob(".coverage.*")):
        match = re.search(r"\.pid([0-9]+)\.", raw_path.name)
        require(match is not None and not raw_path.is_symlink(), "invalid-raw-process-data-path")
        pid = int(match[1])
        require(pid not in raw_pids, "duplicate-raw-process-pid")
        raw_pids.add(pid)
        audit_path = measurement / "audit" / (str(pid) + ".json")
        audit = read_json(audit_path)
        mapped = {}
        for row in audit:
            require(row["pid"] == pid and row["module"] in binding["source_hashes"]
                    and row["path"].endswith("/" + row["module"]), "invalid-executed-source-identity")
            admitted = row["sha256"] == binding["source_hashes"][row["module"]]
            require(type(row["admitted"]) is bool and row["admitted"] is admitted, "invalid-source-admission-decision")
            if admitted:
                mapped[row["path"]] = str(root / row["module"])
                admitted_pids.add(pid)
            else:
                excluded.append(row)
        data = CoverageData(basename=str(raw_path))
        data.read()
        require(not data.measured_files() or data.has_arcs(), "raw-process-branch-data-missing")
        for context in sorted(data.measured_contexts()):
            require(context == "" or context in outcomes["test_durations"], "raw-data-has-unknown-test-context")
            data.set_query_context(context)
            aggregate.set_context(context)
            for filename in data.measured_files():
                if filename in mapped:
                    arcs = data.arcs(filename)
                    if arcs:
                        aggregate.add_arcs({mapped[filename]: arcs})
        raw_ids.append({"pid": pid, "data_file": raw_path.name, "sha256": sha(raw_path), "audit_sha256": sha(audit_path)})
    require(raw_pids == child_pids | {execution["pid"]}, "raw-process-inventory-mismatch")
    require(child_pids <= admitted_pids, "missing-subprocess-source-observation")
    aggregate.write()
    measured = Coverage(config_file=False, data_file=str(output / ".coverage"))
    measured.set_option("report:exclude_lines", [])
    measured.set_option("report:partial_branches", [])
    measured.load()
    raw_report = compact_report(measured, aggregate, root, manifest)
    write_json(output / "coverage-raw.json", raw_report)
    modules = module_rows(raw_report, root, manifest, binding["source_hashes"])
    result = {"schema_version": 1, "modules": modules, "subprocess_count": len(child_pids),
              "expected_subprocess_count": len(child_pids), "missing_subprocess_pids": [],
              "test_count": args.expected_tests, "altered_source_observations_excluded": len(excluded),
              "original_gate_errors": old_result["errors"],
              "measurement": str(measurement), "snapshot": str(root), "analysis_seconds": time.monotonic() - started}
    result["errors"] = policy.enforce(result, manifest, binding["source_hashes"])
    controls = policy.enforcement_controls(manifest, binding["source_hashes"])
    relative = copy.deepcopy(raw_report)
    relative["files"] = {name: raw_report["files"].get(name, raw_report["files"].get(str(root / name)))
                         for name in manifest["source_modules"]}
    absolute = copy.deepcopy(relative)
    absolute["files"] = {str(root / name): value for name, value in relative["files"].items()}
    require(module_rows(relative, root, manifest, binding["source_hashes"]) == modules
            == module_rows(absolute, root, manifest, binding["source_hashes"]), "relative-absolute-control-mismatch")
    controls["actual-relative-and-absolute-module-report"] = []
    omitted = copy.deepcopy(relative)
    del omitted["files"][manifest["source_modules"][0]]
    try:
        module_rows(omitted, root, manifest, binding["source_hashes"])
    except ValueError as error:
        controls["actual-omitted-raw-module"] = [str(error)]
    else:
        raise ValueError("omitted-raw-module-control-not-rejected")
    require(snapshot_hashes == checked_hashes(root, frozen["files"]), "snapshot-changed-during-readback")
    require(original_hashes == {str(path.relative_to(measurement)): sha(path) for path in measurement.rglob("*") if path.is_file()},
            "original-evidence-changed-during-readback")
    require(all(sha(Path(name)) == digest for name, digest in analysis_sources.items()), "analyzer-changed-during-readback")
    require(all(sha(Path(name)) == digest for name, digest in tool["files"].items()), "tool-changed-during-readback")
    write_json(output / "controls.json", controls)
    write_json(output / "raw-process-identities.json", raw_ids)
    write_json(output / "excluded-source-observations.json", excluded)
    write_json(output / "source-and-evidence-custody.json", {"snapshot_manifest_sha256": sha(args.snapshot_manifest),
               "snapshot_files": snapshot_hashes, "original_measurement_files": original_hashes,
               "analysis_sources": analysis_sources, "tool_identity_sha256": sha(measurement / "tool-identity.json"),
               "claim": "Fresh read-only analysis; original failed reader result and raw collection preserved unchanged."})
    write_json(output / "coverage-result.json", result)
    print(json.dumps({"output": str(output), "errors": result["errors"], "test_count": args.expected_tests,
                      "subprocess_count": len(child_pids)}))
    return 0 if not result["errors"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
