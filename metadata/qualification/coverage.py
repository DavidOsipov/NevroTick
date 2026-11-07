"""Measure the complete portable metadata source with subprocess identity custody.

Run with an isolated, explicitly selected coverage.py 7.17.2 environment. Reports
retain all eight source identities, zero-line markers, line and branch evidence,
actual test outcomes, copied-source admission and per-test line reachability.
"""

from __future__ import annotations

import argparse
import copy
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import time


VERSION = "7.17.2"
WHEEL_SHA256 = "36314997533266bde3d07765289674304d61bfe49165bb1b4fa9be12cb4ba0d1"
ROOT = Path(__file__).resolve().parents[2]
SOURCE_MODULES = frozenset(("metadata/__init__.py", "metadata/pipeline/__init__.py",
    "metadata/pipeline/__main__.py", "metadata/pipeline/contract.py", "metadata/pipeline/delivery.py",
    "metadata/pipeline/fields.py", "metadata/pipeline/generate.py", "metadata/pipeline/text_policy.py"))
MARKERS = frozenset(("metadata/__init__.py", "metadata/pipeline/__init__.py"))
CRITICAL_FILES = SOURCE_MODULES - MARKERS
SECURITY_MODULES = CRITICAL_FILES - {"metadata/pipeline/fields.py"}
INPUT_FILES = frozenset(("metadata/product.toml", "Cargo.toml", "crates/nevrotick-ui/Cargo.toml",
                         "crates/nevrotick-engine/Cargo.toml"))


def admit_manifest(manifest: dict) -> None:
    for key, expected in (("source_modules", SOURCE_MODULES), ("security_modules", SECURITY_MODULES),
                          ("critical_files", CRITICAL_FILES), ("input_files", INPUT_FILES)):
        if type(manifest.get(key)) is not list or len(manifest[key]) != len(expected) or set(manifest[key]) != expected:
            raise ValueError("manifest-required-scope-mismatch:" + key)
    for key, minimum in (("minimum_executable_line_percent", 90), ("minimum_adjusted_mutation_percent", 95)):
        if type(manifest.get(key)) is not int or not minimum <= manifest[key] <= 100:
            raise ValueError("manifest-required-threshold-mismatch:" + key)
    if type(manifest.get("nonexecuting_modules")) is not dict or set(manifest["nonexecuting_modules"]) != MARKERS:
        raise ValueError("manifest-marker-scope-mismatch")


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write_json(path: Path, data: object) -> None:
    path.write_text(json.dumps(data, indent=2, sort_keys=True) + "\n")


def enforce(report: dict, manifest: dict, hashes: dict[str, str]) -> list[str]:
    """Reject missing identity, empty critical observations or a per-file drop."""
    admit_manifest(manifest)
    errors: list[str] = []
    if set(report["modules"]) != set(manifest["source_modules"]):
        errors.append("source-inventory-mismatch")
    for module in manifest["source_modules"]:
        row = report["modules"].get(module)
        if row is None:
            continue
        if row["sha256"] != hashes[module]:
            errors.append("source-identity-mismatch:" + module)
        total = row["executable_lines"]
        covered = row["covered_lines"]
        if type(total) is not int or type(covered) is not int or not 0 <= covered <= total:
            errors.append("invalid-line-observation:" + module)
            continue
        if module in manifest["nonexecuting_modules"] and (total or covered):
            errors.append("nonexecuting-marker-has-denominator:" + module)
        if module in manifest["security_modules"] and (not total or covered * 100 < total * manifest["minimum_executable_line_percent"]):
            errors.append("line-coverage-below-minimum:" + module)
    if report["subprocess_count"] < 1:
        errors.append("missing-subprocess-evidence")
    if report.get("missing_subprocess_pids"):
        errors.append("missing-subprocess-observations")
    return errors


def enforcement_controls(manifest: dict, hashes: dict[str, str]) -> dict:
    rows = {name: {"sha256": value, "executable_lines": 0 if name in manifest["nonexecuting_modules"] else 100,
                   "covered_lines": 0 if name in manifest["nonexecuting_modules"] else 100}
            for name, value in hashes.items()}
    healthy = {"modules": rows, "subprocess_count": 1}
    cases = {"healthy": healthy}
    cases["coverage-drop"] = copy.deepcopy(healthy)
    cases["coverage-drop"]["modules"][manifest["security_modules"][0]]["covered_lines"] = 89
    cases["omitted-module"] = copy.deepcopy(healthy)
    del cases["omitted-module"]["modules"][manifest["source_modules"][0]]
    cases["wrong-hash"] = copy.deepcopy(healthy)
    cases["wrong-hash"]["modules"][manifest["source_modules"][0]]["sha256"] = "0" * 64
    cases["missing-subprocess"] = copy.deepcopy(healthy)
    cases["missing-subprocess"]["subprocess_count"] = 0
    observed = {name: enforce(report, manifest, hashes) for name, report in cases.items()}
    if observed["healthy"] or any(not observed[name] for name in cases if name != "healthy"):
        raise ValueError("coverage enforcement controls did not discriminate")
    for name, key, change in (("weakened-line-threshold", "minimum_executable_line_percent", 89),
                              ("omitted-manifest-module", "source_modules", manifest["source_modules"][:-1]),
                              ("omitted-security-owner", "security_modules", manifest["security_modules"][:-1]),
                              ("omitted-source-input", "input_files", manifest["input_files"][:-1])):
        faulty = copy.deepcopy(manifest)
        faulty[key] = change
        try:
            admit_manifest(faulty)
        except ValueError as error:
            observed[name] = [str(error)]
        else:
            raise ValueError("manifest control did not discriminate:" + name)
    return observed


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--python", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    parser.add_argument("--wheel", type=Path, help="Pinned cp312 Linux wheel; defaults to the venv parent's downloaded wheel")
    parser.add_argument("--controls-only", action="store_true")
    parser.add_argument("--missing-subprocess-control", action="store_true",
                        help="Controls-only negative: omit child startup instrumentation and require rejection")
    parser.add_argument("--legacy-file-tracer", action="store_true",
                        help="Retain the slower source-mapping tracer for independent comparison")
    args = parser.parse_args()
    if args.missing_subprocess_control and not args.controls_only:
        parser.error("--missing-subprocess-control requires --controls-only")
    output = args.output.absolute()
    started = time.monotonic()
    output.mkdir(parents=True, exist_ok=False)
    manifest = json.loads((ROOT / "metadata/qualification/manifest.json").read_text())
    admit_manifest(manifest)
    source_hashes = {name: digest(ROOT / name) for name in manifest["source_modules"]}
    candidate_files = (set(manifest["source_modules"]) | set(manifest["input_files"])
                       | {path.relative_to(ROOT).as_posix() for path in (ROOT / "metadata/tests").glob("*.py")}
                       | {path.relative_to(ROOT).as_posix() for path in (ROOT / "metadata/qualification").glob("coverage*") if path.is_file()}
                       | {"metadata/qualification/manifest.json", "docs/supporting-reviews/windows-service-metadata-catalogue.md"})
    candidate_hashes = {name: digest(ROOT / name) for name in sorted(candidate_files)}
    binding = {"root": str(ROOT), "source_hashes": source_hashes,
               "candidate_hashes": candidate_hashes,
               "audit": str(output / "audit")}
    (output / "audit").mkdir()
    write_json(output / "binding.json", binding)
    write_json(output / "enforcement-controls.json", enforcement_controls(manifest, source_hashes))
    # Import and hash every installed coverage package/data file, not only a
    # version string. Source re-admission is checked again after execution.
    wheels = list(args.python.absolute().parent.parent.parent.glob("coverage-7.17.2-cp312-*.whl"))
    wheel = args.wheel or (wheels[0] if len(wheels) == 1 else None)
    if wheel is None:
        raise ValueError("provide-pinned-coverage-wheel")
    tool = json.loads(subprocess.check_output([str(args.python), "-I", "-B", str(ROOT / "metadata/qualification/coverage_tool.py"), str(wheel)], text=True))
    if tool["version"] != VERSION:
        raise ValueError("wrong-coverage-version")
    tool["python_path"] = str(args.python.absolute())
    tool["python_sha256"] = digest(args.python.resolve())
    tool["files"][str(args.python.resolve())] = tool["python_sha256"]
    tool["wheel_sha256"] = WHEEL_SHA256
    write_json(output / "tool-identity.json", tool)
    site = subprocess.check_output([str(args.python), "-I", "-B", "-c", "import sysconfig; print(sysconfig.get_path('purelib'))"], text=True).strip()
    # This owned venv hook makes only the plugin importable under Python -I.
    hook = Path(site) / "00_nevrotick_metadata_coverage.pth"
    hook.write_text(str(ROOT / "metadata/qualification") + "\n")
    tool["files"][str(hook)] = digest(hook)
    write_json(output / "tool-identity.json", tool)
    health = json.loads(subprocess.check_output([str(args.python), "-I", "-B", str(ROOT / "metadata/qualification/coverage_tests.py"), "--controls"], text=True))
    write_json(output / "test-runner-controls.json", health)
    environment = os.environ.copy()
    environment["NEVROTICK_COVERAGE_BINDING"] = str(output / "binding.json")
    environment["NEVROTICK_COVERAGE_PROCESS_LEDGER"] = str(output / "processes.jsonl")
    config = output / ".coveragerc"
    patch = "" if args.missing_subprocess_control else "patch = subprocess\n"
    plugin = "coverage_plugin" if args.legacy_file_tracer else "coverage_identity"
    includes = [str(ROOT / name) if args.legacy_file_tracer else "*/" + name for name in manifest["source_modules"]]
    config.write_text("[run]\nbranch = true\nparallel = true\n" + patch + "plugins = " + plugin + "\ninclude =\n" + "".join("    " + name + "\n" for name in includes) + "data_file = " + str(output / ".coverage") + "\n[report]\nexclude_lines =\npartial_branches =\n")
    # -B prevents writes but still permits reading pre-existing copied .pyc.
    # A fresh prefix plus -B makes actual source bytes the compilation owner.
    cache = output / "empty-pycache"
    cache.mkdir()
    command = [str(args.python), "-I", "-B", "-X", "pycache_prefix=" + str(cache), "-m", "coverage", "run", "--rcfile=" + str(config)]
    if args.controls_only:
        probe = output / "subprocess_probe.py"
        probe.write_text("import subprocess,sys,shutil,pathlib\nfrom coverage_processes import install_process_ledger\ninstall_process_ledger()\nroot=pathlib.Path(" + repr(str(ROOT)) + ")\ncopy=pathlib.Path(" + repr(str(output / "copied")) + ")\nshutil.copytree(root/'metadata',copy/'metadata')\nfor entry in [root/'metadata/pipeline/__main__.py',copy/'metadata/pipeline/__main__.py']:\n subprocess.run([sys.executable,'-I','-B',str(entry),'validate','--root',str(root)],check=True)\ngenerator=copy/'metadata/pipeline/generate.py'\ngenerator.write_bytes(generator.read_bytes()+b'\\n# altered source identity control\\n')\nsubprocess.run([sys.executable,'-I','-B',str(copy/'metadata/pipeline/__main__.py'),'validate','--root',str(root)],check=True)\n")
        command.append(str(probe))
    else:
        command.extend([str(ROOT / "metadata/qualification/coverage_tests.py"), str(ROOT), str(output / "test-result.json")])
    write_json(output / "command.json", command)
    admitted = time.monotonic()
    with (output / "tests.log").open("w") as log:
        process = subprocess.Popen(command, env=environment, stdout=log, stderr=subprocess.STDOUT)
        returncode = process.wait()
    executed = time.monotonic()
    write_json(output / "execution.json", {"returncode": returncode, "pid": process.pid})
    # Keep each raw process file for independent reconciliation.
    if not args.legacy_file_tracer:
        subprocess.run([str(args.python), "-I", "-B", str(ROOT / "metadata/qualification/coverage_normalize.py"), str(output)], env=environment, check=True)
    commands = (["combine", "--keep"],) if args.legacy_file_tracer else ()
    for tail in commands:
        subprocess.run([str(args.python), "-I", "-B", "-m", "coverage", *tail[:1], "--rcfile=" + str(config), *tail[1:]], env=environment, check=True, cwd=ROOT)
    subprocess.run([str(args.python), "-I", "-B", str(ROOT / "metadata/qualification/coverage_report.py"), str(output), str(ROOT)], env=environment, check=True, cwd=ROOT)
    raw = json.loads((output / "coverage-raw.json").read_text())
    records = [record for path in (output / "audit").glob("*.json") for record in json.loads(path.read_text())]
    write_json(output / "source-execution-audit.json", records)
    pids = {row["pid"] for row in records if row["admitted"]}
    launched = [json.loads(line) for line in (output / "processes.jsonl").read_text().splitlines()]
    modules = {}
    contexts: dict[str, dict[str, list[int]]] = {}
    for name in manifest["source_modules"]:
        keys = [key for key in (name, str(ROOT / name)) if key in raw["files"]]
        if len(keys) != 1:
            raise ValueError("missing-or-duplicate-raw-module-observation:" + name)
        item = raw["files"][keys[0]]
        summary = item["summary"]
        modules[name] = {"sha256": source_hashes[name], "executable_lines": summary["num_statements"],
                         "covered_lines": summary["covered_lines"], "missing_lines": item["missing_lines"],
                         "branches": summary.get("num_branches", 0), "covered_branches": summary.get("covered_branches", 0),
                         "missing_branches": item.get("missing_branches", []),
                         "classification": "nonexecuting-marker" if name in manifest["nonexecuting_modules"] else "active-source"}
        for line, names in item["contexts"].items():
            for context in names:
                contexts.setdefault(context, {}).setdefault(name, []).append(int(line))
    report = {"schema_version": 1, "modules": modules, "subprocess_count": len(pids - {process.pid}),
              "process_count": len(pids), "altered_source_frames_excluded": sum(not row["admitted"] for row in records),
              "altered_source_count_unit": "per-process source-file identity observations; excluded before normalization",
              "expected_subprocess_count": len(launched), "missing_subprocess_pids": sorted({row["pid"] for row in launched} - pids),
              "returncode": returncode, "line_minimum_percent": manifest["minimum_executable_line_percent"]}
    report["timings_seconds"] = {"admission": admitted - started, "test_execution": executed - admitted,
                                 "analysis": time.monotonic() - executed}
    report["errors"] = enforce(report, manifest, source_hashes)
    if args.controls_only and not args.missing_subprocess_control:
        if any(not modules[name]["covered_lines"] for name in CRITICAL_FILES):
            report["errors"].append("healthy-subprocess-probe-missing-active-module-lines")
    if source_hashes != {name: digest(ROOT / name) for name in manifest["source_modules"]}:
        report["errors"].append("candidate-changed-during-execution")
    if candidate_hashes != {name: digest(ROOT / name) for name in candidate_hashes}:
        report["errors"].append("configuration-or-tests-changed-during-execution")
    if returncode:
        report["errors"].append("test-run-failed")
    if not args.controls_only:
        test_result = json.loads((output / "test-result.json").read_text())
        if (not test_result["successful"] or test_result["tests_run"] != test_result["discovered"]
                or any(test_result[name] for name in ("failures", "errors", "skipped", "expected_failures", "unexpected_successes"))):
            report["errors"].append("incomplete-test-outcomes")
    if any(digest(Path(name)) != expected for name, expected in tool["files"].items()):
        report["errors"].append("tool-changed-during-execution")
    write_json(output / "coverage-result.json", report)
    write_json(output / "test-line-contexts.json", contexts)
    print(json.dumps({"output": str(output), "errors": report["errors"], "subprocess_count": report["subprocess_count"], "altered_source_frames_excluded": report["altered_source_frames_excluded"]}))
    controls_passed = (args.controls_only and report["subprocess_count"] > 0 and not report["missing_subprocess_pids"]
                       and report["altered_source_frames_excluded"] > 0 and not returncode
                       and all(error.startswith("line-coverage-below-minimum:") for error in report["errors"]))
    return 0 if controls_passed or not report["errors"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
