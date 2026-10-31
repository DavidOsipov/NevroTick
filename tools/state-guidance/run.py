#!/usr/bin/env python3
"""Admit and run the bounded fixture-only state-guidance comparison.

Acquisition/build is an explicit installation operation. This entry point is
offline: immutable admitted executable, current source bindings, private work,
existing namespace/resource guards and before/after custody for every command.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import sys

ROOT = Path(__file__).resolve().parents[2]
HERE = Path(__file__).resolve().parent
POINTER = Path.home() / ".local/share/nevrotick/state-guidance-installation.json"
SPEC = importlib.util.spec_from_file_location("state_guidance_runtime", ROOT / "scripts/analysis-runtime.py")
RUNTIME = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RUNTIME)
Incomplete = RUNTIME.Incomplete
MAX_JSON = 2 * 1024 * 1024


def digest(path: Path) -> str:
    """Hash current regular-file bytes, without following an unadmitted alias."""
    if path.is_symlink() or not path.is_file():
        raise Incomplete("Admitted input must be a regular non-symlink file.")
    result = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            result.update(chunk)
    return result.hexdigest()


def closed_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    """Preserve duplicate-key rejection before dictionary normalization."""
    result = {}
    for key, value in pairs:
        if key in result:
            raise Incomplete("Duplicate JSON key.")
        result[key] = value
    return result


def read_object(path: Path) -> dict[str, object]:
    """Bound one complete JSON object; annotations do not validate input bytes."""
    if path.is_symlink() or not path.is_file() or path.stat().st_size > MAX_JSON:
        raise Incomplete("JSON input is missing, redirected or oversized.")
    with path.open(encoding="utf-8") as stream:
        value = json.load(stream, object_pairs_hook=closed_object)
    if type(value) is not dict:
        raise Incomplete("Expected a JSON object.")
    return value


def relative_path(value: object) -> Path:
    """Only bounded checkout/installation descendants can be bound by a manifest."""
    if type(value) is not str or not 0 < len(value) <= 512:
        raise Incomplete("Invalid relative input path.")
    path = Path(value)
    if path.is_absolute() or ".." in path.parts or not path.parts:
        raise Incomplete("Input path escapes its owner.")
    return path


def check_bindings(base: Path, values: object) -> None:
    """Require every selected path to remain canonical and byte-identical."""
    if type(values) is not dict or not 0 < len(values) <= 4096:
        raise Incomplete("Invalid input-binding inventory.")
    for name, expected in values.items():
        path = base / relative_path(name)
        if (type(expected) is not str or re.fullmatch(r"[0-9a-f]{64}", expected) is None
                or path.resolve(strict=True) != path or digest(path) != expected):
            raise Incomplete("Source or payload differs from its admitted binding.")


def admit(variant: str | None = None) -> tuple[Path, Path, dict[str, object]]:
    """Admit current recipe, installation, executable and selected dependencies."""
    policy = read_object(HERE / "policy.json")
    if type(policy.get("schema_version")) is not int or policy["schema_version"] != 1:
        raise Incomplete("Unknown policy schema.")
    pointer = read_object(POINTER)
    details = POINTER.stat()
    if (set(pointer) != {"schema_version", "root"} or type(pointer["schema_version"]) is not int
            or pointer["schema_version"] != 1
            or type(pointer["root"]) is not str or details.st_uid != os.geteuid()
            or details.st_nlink != 1 or details.st_mode & 0o077):
        raise Incomplete("Installation pointer is not a private owned schema-1 record.")
    base = Path(pointer["root"])
    if not base.is_absolute() or base.resolve(strict=True) != base or base.is_relative_to(ROOT):
        raise Incomplete("Installation must be canonical and outside the checkout.")
    manifest_path = base / "installation-manifest.json"
    if digest(manifest_path) != policy.get("installation_manifest_sha256"):
        raise Incomplete("Installation manifest differs from the admitted policy.")
    manifest = read_object(manifest_path)
    if manifest.get("backend_version") != "0.16.1":
        raise Incomplete("Unexpected state-guidance backend version.")
    check_bindings(ROOT, manifest.get("source_bindings"))
    check_bindings(HERE, policy.get("recipe_bindings"))
    check_bindings(ROOT, policy.get("supervisor_bindings"))
    payload = manifest
    if variant is not None:
        if variant != "origin_guard_omitted" or type(manifest.get("faults")) is not dict:
            raise Incomplete("Unknown or unqualified intentional fault.")
        payload = manifest["faults"].get(variant)
        if type(payload) is not dict:
            raise Incomplete("Intentional-fault payload is absent.")
    engine = base / relative_path(payload.get("engine_relative"))
    if engine.resolve(strict=True) != engine or digest(engine) != payload.get("engine_sha256"):
        raise Incomplete("Engine bytes differ from the qualified installation.")
    payloads = manifest.get("runtime_payloads")
    if type(payloads) is not dict or not payloads:
        raise Incomplete("Runtime dependency closure is absent.")
    for name, expected in payloads.items():
        path = Path(name)
        if not path.is_absolute() or path.resolve(strict=True) != path or digest(path) != expected:
            raise Incomplete("A runtime dependency changed.")
    require_advisory(base, policy)
    return base, engine, policy


def fresh_work(value: str) -> Path:
    """Create one private external output owner; preserve every existing path."""
    path = Path(value)
    if (not path.is_absolute() or path.exists() or path.is_symlink()
            or path.parent.resolve(strict=True) != path.parent
            or path.is_relative_to(ROOT)):
        raise Incomplete("Output must be a fresh canonical path outside the checkout.")
    if shutil.disk_usage(path.parent).free < 512 * 1024 ** 2:
        raise Incomplete("State-guidance needs 512 MiB of free output capacity.")
    path.mkdir(mode=0o700)
    return path


def execute(work: Path, arguments: list[str], name: str, *,
            variant: str | None = None) -> tuple[dict[str, object], dict[str, object]]:
    """Execute only the admitted engine under the existing guarded runtime."""
    base, engine, _ = admit(variant)
    result = RUNTIME.run(ROOT, work, [str(engine), *arguments], readonly=[base],
                         timeout=60, cpu_seconds=60, address_space_bytes=2 * 1024 ** 3,
                         build_jobs=1)
    (work / f"{name}-native.json").write_text(json.dumps(result, indent=2) + "\n")
    admit(variant)  # A changed source/tool/runtime payload invalidates this command.
    if result["failure"] or result["exit_code"] not in (0, 86):
        raise Incomplete("Engine did not complete the intended action; retain native receipt.")
    report = json.loads(result["output"], object_pairs_hook=closed_object)
    if (type(report) is not dict or type(report.get("schema_version")) is not int
            or report["schema_version"] != 1):
        raise Incomplete("Engine report is not a supported object.")
    return result, report


def write_seeds(work: Path) -> None:
    """Use declared raw seeds; never derive training inputs from target outputs."""
    seeds = read_object(HERE / "seeds.json")
    if seeds.get("schema_version") != 1 or type(seeds.get("seeds")) is not dict:
        raise Incomplete("Invalid seed registry.")
    target = work / "seeds"
    target.mkdir(mode=0o700)
    if not 0 < len(seeds["seeds"]) <= 16:
        raise Incomplete("Seed count exceeds the declared pilot domain.")
    for name, raw in seeds["seeds"].items():
        if (type(name) is not str or re.fullmatch(r"[a-z][a-z0-9_-]{0,47}", name) is None
                or type(raw) is not str or re.fullmatch(r"[0-9a-f]{6,130}", raw) is None):
            raise Incomplete("Seed identity/hex bytes are invalid.")
        data = bytes.fromhex(raw)
        if len(data) % 2 != 1 or data[0] != 1 or any(data[i] >= 16 for i in range(1, len(data), 2)):
            raise Incomplete("Seed is outside the admitted version-1 trace domain.")
        (target / f"{name}.trace").write_bytes(data)


def validate_campaign(report: dict[str, object], mode: str, seed: int) -> None:
    """Transport completion cannot substitute for exact observed execution facts."""
    if report.get("status") != "complete" or report.get("mode") != mode:
        raise Incomplete("Comparison arm is incomplete or has the wrong mode.")
    for name, expected in (("seed", seed), ("executions_requested", 1024),
                           ("executions_completed", 1024), ("oracle_failures", 0)):
        if type(report.get(name)) is not int or report[name] != expected:
            raise Incomplete("Comparison arm has missing or inconsistent execution facts.")
    for name in ("observed_coverage_edges", "observed_semantic_features",
                 "retained_semantic_features"):
        values = report.get(name)
        limit = 65536 if name == "observed_coverage_edges" else 512
        if (type(values) is not list or not values or len(values) > limit
                or any(type(value) is not int or not 0 <= value < limit for value in values)
                or len(set(values)) != len(values)):
            raise Incomplete("Comparison arm has missing or malformed reached observations.")
    if not set(report["retained_semantic_features"]).issubset(report["observed_semantic_features"]):
        raise Incomplete("Retained semantic evidence was never observed.")
    for name, low, high in (("registered_guards", 1, 65535), ("corpus_count", 1, 1024),
                            ("saved_queue_files", 1, 1024), ("completed_trace_steps", 1, 32768),
                            ("malformed_inputs", 0, 0), ("completions", 0, 32768)):
        value = report.get(name)
        if type(value) is not int or not low <= value <= high:
            raise Incomplete("Comparison arm has invalid bounded resource/observation facts.")
    if report["saved_queue_files"] != report["corpus_count"]:
        raise Incomplete("Saved corpus and corpus accounting differ.")


def replay(work: Path, name: str, data: bytes, *, variant: str | None = None) -> tuple[dict, dict]:
    """Replay concrete bounded bytes from fresh target state in the same binary."""
    traces = work / "traces"
    traces.mkdir(mode=0o700, exist_ok=True)
    path = traces / f"{name}.trace"
    if path.exists():
        raise Incomplete("A replay must not overwrite an earlier input.")
    path.write_bytes(data)
    return execute(work, ["replay", f"/work/traces/{name}.trace"], name, variant=variant)


def controls(work: Path) -> dict[str, object]:
    """Check reached outcomes, reset, real coverage and one actual guard fault."""
    cases = {
        "arm": ("0100f0", 1, 0, 1),
        "complete": ("0100f00100", 2, 1, None),
        "wrong_origin_then_complete": ("0100f002000100", 3, 1, None),
        "replay": ("0100f001000c00", 3, 1, None),
        "context": ("0100f008000100", 3, 0, None),
        "stale_cancel": ("0100f0040000f005000100", 5, 1, None),
        "expiry": ("0100f0060007000100", 4, 0, None),
    }
    observations = {}
    for name, (raw, steps, completions, pending) in cases.items():
        native, report = replay(work, name, bytes.fromhex(raw))
        if (native["exit_code"] != 0 or report.get("status") != "ok"
                or report.get("steps") != steps or report.get("completions") != completions
                or report.get("final_pending") != pending or not report.get("coverage_edges")
                or not report.get("semantic_features")):
            raise Incomplete("A healthy control failed its independently specified facts.")
        observations[name] = report
    _, repeat = replay(work, "complete_repeat", bytes.fromhex(cases["complete"][0]))
    if repeat != observations["complete"]:
        raise Incomplete("Identical trace observations are not reproducible.")
    _, left = replay(work, "configuration_even", bytes([1, 8, 0]))
    _, right = replay(work, "configuration_odd", bytes([1, 8, 1]))
    if (left.get("status") != "ok" or right.get("status") != "ok"
            or left.get("coverage_edges") != right.get("coverage_edges")
            or left.get("semantic_features") == right.get("semantic_features")):
        raise Incomplete("State control did not distinguish real semantic states sharing coverage.")
    probe_seeds = work / "feedback-seeds"
    probe_seeds.mkdir(mode=0o700)
    for name, data in (("a-even", bytes([1, 8, 0])), ("b-odd", bytes([1, 8, 1])),
                       ("c-even-repeat", bytes([1, 8, 0]))):
        (probe_seeds / f"{name}.trace").write_bytes(data)
    feedback_reports = []
    for mode, expected_count in (("coverage", 1), ("semantic", 2)):
        native, report = execute(work, ["campaign", mode, "7", "3", "/work/feedback-seeds",
                                       f"/work/feedback-{mode}"], f"feedback-{mode}")
        if (native["exit_code"] != 0 or report.get("status") != "complete"
                or report.get("executions_completed") != 3
                or report.get("corpus_count") != expected_count
                or report.get("oracle_failures") != 0):
            raise Incomplete("The actual feedback channel did not control corpus admission as specified.")
        feedback_reports.append(report)
    _, malformed = replay(work, "unsupported_version", bytes([0, 0, 0]))
    if (malformed.get("status") != "invalid_input" or malformed.get("steps") != 0
            or malformed.get("coverage_edges")):
        raise Incomplete("Malformed admission performed or claimed target work.")
    native, faulty = replay(work, "origin_guard_fault", bytes.fromhex(cases["wrong_origin_then_complete"][0]),
                            variant="origin_guard_omitted")
    diagnosis = str(faulty.get("failure", ""))
    if (native["exit_code"] != 86 or faulty.get("status") != "oracle_failure"
            or "OracleMismatch" not in diagnosis
            or re.search(r"expected:\s*OriginMismatch\b", diagnosis) is None
            or re.search(r"actual:\s*Time\b", diagnosis) is None):
        raise Incomplete("The deliberate origin-guard fault was not detected by its intended oracle.")
    return {"schema_version": 1, "status": "complete", "healthy_controls": observations,
            "reset_replay": repeat, "same_coverage_state_pair": [left, right],
            "actual_feedback_admission": feedback_reports,
            "malformed_control": malformed, "guard_fault_control": faulty,
            "scope": "Fixture controls; no NTS, sanitizer or native Windows qualification."}


def require_controls(base: Path, policy: dict[str, object]) -> None:
    """A comparison requires completed controls for this exact recipe/artifact."""
    path = base / relative_path(policy.get("controls_relative"))
    if digest(path) != policy.get("controls_sha256"):
        raise Incomplete("Qualified controls are absent or changed.")
    report = read_object(path)
    if (report.get("status") != "complete"
            or report.get("installation_manifest_sha256") != policy.get("installation_manifest_sha256")
            or report.get("recipe_bindings") != policy.get("recipe_bindings")):
        raise Incomplete("Controls belong to another candidate or recipe.")


def require_advisory(base: Path, policy: dict[str, object]) -> None:
    """Preserve the strict advisory result for the exact installed lockfile."""
    path = base / relative_path(policy.get("advisory_relative"))
    if digest(path) != policy.get("advisory_sha256"):
        raise Incomplete("The candidate advisory receipt is missing or changed.")
    report = read_object(path)
    if (report.get("status") != "passed" or type(report.get("native_exit")) is not int
            or report["native_exit"] != 0 or report.get("supervisor_failure") is not None
            or type(report.get("security_vulnerability_count")) is not int
            or report["security_vulnerability_count"] != 0 or report.get("warnings") != []
            or report.get("ignored_advisory_ids") != []
            or report.get("lockfile_sha256") != digest(HERE / "engine/Cargo.lock")):
        raise Incomplete("The installed graph has no passing exact-lock advisory result.")


def fault_search(work: Path) -> list[dict[str, object]]:
    """Measure first detection or censoring for one declared private guard fault."""
    results = []
    for seed in (1, 7, 42):
        for mode in ("coverage", "semantic"):
            name = f"fault-{mode}-{seed}"
            native, report = execute(work, ["campaign", mode, str(seed), "1024", "/work/seeds",
                                           f"/work/{name}"], name, variant="origin_guard_omitted")
            count = report.get("executions_completed")
            if (report.get("mode") != mode or type(report.get("seed")) is not int
                    or report["seed"] != seed or type(count) is not int or not 1 <= count <= 1024):
                raise Incomplete("Fault-search execution identity/count is invalid.")
            if native["exit_code"] == 0:
                validate_campaign(report, mode, seed)
                results.append({"mode": mode, "seed": seed, "outcome": "censored_at_budget",
                                "executions_to_detection_or_limit": count, "report": report})
                continue
            diagnosis = str(report.get("first_failure", ""))
            if (report.get("status") != "oracle_failure" or report.get("oracle_failures") != 1
                    or "OracleMismatch" not in diagnosis
                    or re.search(r"expected:\s*OriginMismatch\b", diagnosis) is None
                    or re.search(r"actual:\s*(Time|Control)\b", diagnosis) is None):
                raise Incomplete("Fault search failed outside the intended oracle context.")
            witness = (work / name / "first-failure.trace").read_bytes()
            if not 3 <= len(witness) <= 65 or witness != (work / name / "current-input.trace").read_bytes():
                raise Incomplete("Failure witness does not match the input saved before execution.")
            bad_native, bad = replay(work, f"{name}-replay", witness, variant="origin_guard_omitted")
            good_native, good = replay(work, f"{name}-healthy", witness)
            if (bad_native["exit_code"] != 86 or bad.get("status") != "oracle_failure"
                    or good_native["exit_code"] != 0 or good.get("status") != "ok"):
                raise Incomplete("Discovered witness did not reproduce beside its healthy control.")
            results.append({"mode": mode, "seed": seed, "outcome": "detected",
                            "executions_to_detection_or_limit": count, "witness_sha256": hashlib.sha256(witness).hexdigest(),
                            "report": report, "fault_replay": bad, "healthy_replay": good})
    return results


def compare(work: Path) -> dict[str, object]:
    """Compare equal actual-execution budgets; preserve every trial and outcome."""
    write_seeds(work)
    reports = []
    for seed in (1, 7, 42):
        for mode in ("coverage", "semantic"):
            name = f"{mode}-{seed}"
            native, report = execute(work, ["campaign", mode, str(seed), "1024", "/work/seeds",
                                           f"/work/{name}"], name)
            reports.append(report)
            validate_campaign(report, mode, seed)
            if native["exit_code"] != 0:
                raise Incomplete("A comparison arm is incomplete or reports an oracle failure.")
    faults = fault_search(work)
    return {"schema_version": 1, "status": "complete", "scope": "fixture-only three-seed pilot",
            "executions_per_arm": 1024, "reports": reports,
            "fault_search_maximum_executions_per_arm": 1024, "fault_search": faults,
            "limits": "No general effectiveness, sanitizer, native Windows or release claim."}


def main() -> int:
    """Default to read-only admission; execution requires a fresh output owner."""
    parser = argparse.ArgumentParser(description=__doc__)
    selection = parser.add_mutually_exclusive_group()
    selection.add_argument("--compare", action="store_true")
    selection.add_argument("--controls", action="store_true")
    parser.add_argument("--output-dir")
    args = parser.parse_args()
    try:
        base, engine, policy = admit()
        if not args.compare and not args.controls:
            if args.output_dir:
                raise Incomplete("Output is only used for an explicitly selected comparison.")
            print(json.dumps({"status": "admitted", "engine": str(engine)}))
            return 0
        if not args.output_dir:
            raise Incomplete("Comparison requires --output-dir.")
        candidate_output = Path(args.output_dir)
        if candidate_output.is_relative_to(base) or base.is_relative_to(candidate_output):
            raise Incomplete("Output must not overlap the installed readonly inputs.")
        if args.compare:
            require_controls(base, policy)
        work = fresh_work(args.output_dir)
        report = controls(work) if args.controls else compare(work)
        _, _, final_policy = admit()
        report["installation_manifest_sha256"] = final_policy["installation_manifest_sha256"]
        report["recipe_bindings"] = final_policy["recipe_bindings"]
        (work / "verification.json").write_text(json.dumps(report, indent=2) + "\n")
        print(json.dumps({"status": "complete", "receipt": str(work / "verification.json")}))
        return 0
    except (Incomplete, OSError, ValueError, KeyError, TypeError) as error:
        print(f"State-guidance incomplete: {error}", file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
