#!/usr/bin/env python3
"""Run bounded source-bound arithmetic proofs through the admitted namespace.

Install nothing. Refuse source drift and stop at the first unqualified job.
Evidence is private and external to the checkout; no existing receipt is reused.
"""
from __future__ import annotations

import argparse
from collections import Counter
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import stat
import sys
import time


ROOT = Path(__file__).resolve().parents[2]
RUSTFLAGS = "--edition=2024 --check-cfg=cfg(test)"
MAX_JOBS = 12
STATUSES = {"Success", "Satisfied", "Failure", "Unreachable", "Unsatisfiable"}
ROUNDING_COVERS = {
    "zero succeeds", "negative destination endpoint succeeds",
    "positive destination endpoint succeeds", "negative destination neighbor is rejected",
    "positive destination neighbor is rejected", "raw signed minimum is covered",
}
ERA_COVERS = {"unique point witness", "absent witness", "multiple-era witness"}
ROUNDING_ASSERTIONS = {
    "scale classification", "destination classification", "exact residual units",
    "exact signed reconstruction", "nearest integer", "even ties",
}
ERA_ASSERTIONS = {"absence classification", "ambiguity classification", "exact unique result"}


class Incomplete(RuntimeError):
    """A source, execution or verification obligation was not established."""


def module(name: str, path: Path):
    """Load one source-bound local Python module without installing anything."""
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise Incomplete(f"Cannot load local module: {path}")
    loaded = importlib.util.module_from_spec(spec)
    sys.modules[name] = loaded
    spec.loader.exec_module(loaded)
    return loaded


def digest(data: bytes) -> str:
    """Return the SHA256 of exact bytes."""
    return hashlib.sha256(data).hexdigest()


def serializable(value):
    """Preserve structured evidence while converting local Path values."""
    if isinstance(value, Path):
        return str(value)
    if isinstance(value, dict):
        return {str(key): serializable(item) for key, item in value.items()}
    if isinstance(value, (list, tuple)):
        return [serializable(item) for item in value]
    return value


def write_json(path: Path, value) -> None:
    """Create private structured evidence, retaining actual newlines."""
    path.write_text(json.dumps(serializable(value), indent=2, allow_nan=False) + "\n")


def fresh_output(value: str) -> Path:
    """Create exactly one new canonical, private directory outside the checkout."""
    path = Path(value)
    if (not path.is_absolute() or str(path) != value or path.name in ("", ".", "..")
            or path.parent != path.parent.resolve(strict=True)
            or path.is_relative_to(ROOT) or ROOT.is_relative_to(path)):
        raise Incomplete("Output must be a fresh canonical absolute directory outside the checkout")
    path.mkdir(mode=0o700)
    details = path.stat()
    if details.st_uid != os.geteuid() or stat.S_IMODE(details.st_mode) != 0o700:
        raise Incomplete("Output must be owned by this user with mode 0700")
    return path


def input_names(subject: str) -> list[str]:
    """Inventory the original crate, specifications, runners and pinned policies."""
    names = {
        "Cargo.toml", "Cargo.lock", "rust-toolchain.toml",
        "crates/time-domain/Cargo.toml", "policy/analysis-tools.json",
        "policy/tool-manifest.json", "scripts/check-analysis.py",
        "scripts/analysis-runtime.py", "scripts/analysis-controls.py",
        "scripts/check-environment.py", "tools/formal/verify.py",
        ".agents/skills/testing/references/formal-verification.md",
    }
    names.update(path.relative_to(ROOT).as_posix()
                 for path in (ROOT / "crates/time-domain/src").rglob("*.rs"))
    if subject in ("rounding", "all"):
        names.add("tools/formal/rounding_harness.rs")
    if subject in ("era", "all"):
        names.update(("tools/formal/era_harness.rs", "tools/formal/era_projection.py"))
    if (ROOT / ".cargo/config.toml").exists():
        names.add(".cargo/config.toml")
    return sorted(names)


def freeze_inputs(check, source: Path, names: list[str]) -> dict:
    """Copy exact bounded regular-file inputs; preserve an original byte manifest."""
    records = {}
    for name in names:
        original = ROOT / name
        if original != original.resolve(strict=True):
            raise Incomplete(f"Source input redirects through a symlink: {name}")
        data = check.read_once(original)
        target = source / name
        target.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
        target.write_bytes(data)
        records[name] = {"sha256": digest(data), "bytes": len(data)}
    return records


def require_sources(check, source: Path, records: dict, subject: str) -> None:
    """Reject changed, missing or newly added original inputs and frozen bytes."""
    if input_names(subject) != sorted(records):
        raise Incomplete("Original source/configuration input inventory changed")
    for name, expected in records.items():
        original = ROOT / name
        if original != original.resolve(strict=True):
            raise Incomplete(f"Source input path changed: {name}")
        for path in (original, source / name):
            data = check.read_once(path)
            if {"sha256": digest(data), "bytes": len(data)} != expected:
                raise Incomplete(f"Original or frozen input drift: {name}")


def jobs(subject: str) -> list[dict]:
    """Return the complete fixed schedule, without retries or solver concurrency."""
    result = []

    def add(owner, case, harness, intended=None, restored=False):
        result.append({"subject": owner, "case": case, "harness": harness,
                       "intended_failure": intended, "restored_healthy": restored})

    if subject in ("rounding", "all"):
        add("rounding", "healthy", "phase_full_domain")
        add("rounding", "healthy", "round_trip_full_domain")
        add("rounding", "always_error", "control_zero", "required success")
        add("rounding", "signed_result", "control_negative_tie", "exact signed reconstruction")
        add("rounding", "wrong_error", "control_destination_overflow", "destination classification")
        add("rounding", "healthy", "phase_full_domain", restored=True)
        add("rounding", "healthy", "round_trip_full_domain", restored=True)
    if subject in ("era", "all"):
        add("era", "healthy", "era_full_domain")
        add("era", "always_error", "era_control_unique", "required unique success")
        add("era", "wrong_coordinate", "era_control_coordinate", "exact unique result")
        add("era", "wrong_error", "era_control_absent", "absence classification")
        add("era", "healthy", "era_full_domain", restored=True)
    if not result or len(result) > MAX_JOBS:
        raise Incomplete("Empty or excessive proof schedule")
    return result


def selected_jobs(subject: str, start_at: int) -> list[tuple[int, dict]]:
    """Select a nonempty schedule suffix without claiming omitted jobs ran.

    This is explicit partial execution, not automatic receipt reuse or campaign
    resumption. The coordinator must separately reconcile any earlier evidence.
    """
    schedule = jobs(subject)
    if type(start_at) is not int or not 1 <= start_at <= len(schedule):
        raise Incomplete("Start index must select a nonempty fixed-schedule suffix")
    return list(enumerate(schedule, 1))[start_at - 1:]


def prepare_rounding(source: Path, inputs: Path, case: str) -> dict:
    """Copy the actual rounding body and enum; alter only a documented defect."""
    subject_name = "crates/time-domain/src/rounding.rs"
    original = (source / subject_name).read_bytes()
    production = original
    mutation = None
    edits = {
        "always_error": (b"    let denominator = scale.denominator();",
                         b"    return Err(DomainError::ArithmeticOverflow);\n    let denominator = scale.denominator();"),
        "signed_result": (b"Ok((nanoseconds, residual))", b"Ok((-nanoseconds, residual))"),
        "wrong_error": (b".map_err(|_| DomainError::NanosecondsOutOfRange)?;",
                        b".map_err(|_| DomainError::ArithmeticOverflow)?;"),
    }
    if case != "healthy":
        if case not in edits:
            raise Incomplete("Unknown rounding defect")
        old, new = edits[case]
        if production.count(old) != 1:
            raise Incomplete("Rounding mutation point is not unique")
        production = production.replace(old, new, 1)
        mutation = {"before": old.decode(), "after": new.decode()}
    lib = (source / "crates/time-domain/src/lib.rs").read_bytes()
    first = b"/// Fixed redacted domain failures"
    last = b"impl std::fmt::Display for DomainError"
    if lib.count(first) != 1 or lib.count(last) != 1:
        raise Incomplete("Actual DomainError extraction is ambiguous")
    enum = lib[lib.index(first):lib.index(last)]
    if b"pub enum DomainError {" not in enum:
        raise Incomplete("Actual DomainError enum was not selected")
    harness = (source / "tools/formal/rounding_harness.rs").read_bytes()
    (inputs / "production_rounding.rs").write_bytes(production)
    (inputs / "proof.rs").write_bytes(b"#![allow(dead_code)]\n" + enum
        + b'\n#[path="production_rounding.rs"]\nmod rounding;\n' + harness.replace(b"//!", b"//"))
    return {"entrypoint": inputs / "proof.rs", "original_source_sha256": digest(original),
            "compiled_subject_sha256": digest(production), "domain_error_enum_sha256": digest(enum),
            "proof_spec_sha256": digest(harness), "mutation": mutation,
            "correspondence": [{"source": subject_name, "sha256": digest(original),
                                "selection": "complete byte-exact file before the named defect"}]}


def one_record(report: dict, field: str, harness: str) -> dict:
    """Require exactly the selected harness in each independent JSON section."""
    items = report.get(field)
    if (not isinstance(items, list) or len(items) != 1 or not isinstance(items[0], dict)
            or items[0].get("harness_id") != harness):
        raise Incomplete(f"Missing, multiple or mismatched harness in {field}")
    return items[0]


def qualify(report: dict, native: dict, job: dict, entrypoint: str | None = None) -> dict:
    """Fail closed on empty, partial, mismatched or unintended Kani outcomes."""
    if not isinstance(report, dict) or not isinstance(native, dict):
        raise Incomplete("Native and Kani results must be objects")
    if (native.get("failure") is not None or type(native.get("exit_code")) is not int
            or native["exit_code"] < 0):
        raise Incomplete("Native execution failed or has no real exit status")
    verification = report.get("verification_results", {})
    if not isinstance(verification, dict):
        raise Incomplete("Missing verification results")
    summary = verification.get("summary", {})
    results = verification.get("results", [])
    if (not isinstance(summary, dict) or not isinstance(results, list)
            or any(type(summary.get(key)) is not int for key in
                   ("total_harnesses", "executed", "successful", "failed"))
            or summary.get("status") != "completed"
            or summary["total_harnesses"] != 1 or summary["executed"] != 1
            or len(results) != 1 or not isinstance(results[0], dict)
            or results[0].get("harness_id") != job["harness"]):
        raise Incomplete("Kani did not complete exactly the selected nonempty harness")
    checks = results[0].get("checks")
    if (not isinstance(checks, list) or not checks
            or any(not isinstance(item, dict) or item.get("status") not in STATUSES
                   or not isinstance(item.get("category"), str)
                   or not isinstance(item.get("description"), str) for item in checks)):
        raise Incomplete("Kani checks are missing or have an unknown outcome")
    if any(item["category"] != "cover" and item["status"] not in
           {"Success", "Failure", "Unreachable"} for item in checks):
        raise Incomplete("Non-cover obligations must have ordinary proof outcomes")
    failures = [item for item in checks if item["status"] == "Failure"]
    covers = [item for item in checks if item["category"] == "cover"]
    counts = Counter(item["status"] for item in checks)
    errors = one_record(report, "error_details", job["harness"])
    properties = one_record(report, "property_details", job["harness"]).get("property_details", {})
    if not isinstance(properties, dict):
        raise Incomplete("Missing property accounting")
    expected_counts = {"total_properties": len(checks), "passed": counts["Success"],
                       "failed": counts["Failure"], "unreachable": counts["Unreachable"],
                       "satisfied": counts["Satisfied"], "unsatisfiable": counts["Unsatisfiable"],
                       "undetermined": 0, "solver_error": 0}
    if any(type(properties.get(key)) is not int or properties[key] != value
           for key, value in expected_counts.items()):
        raise Incomplete("Property summary disagrees with checks or contains unresolved work")
    metadata = report.get("metadata", {})
    tools = report.get("tools", {})
    if (not isinstance(metadata, dict) or metadata.get("kani_version") != "0.68.0"
            or metadata.get("target") != "x86_64-unknown-linux-gnu"
            or not isinstance(tools, dict) or tools.get("kani") != "0.68.0"
            or not isinstance(tools.get("solvers"), list) or len(tools["solvers"]) != 1
            or not isinstance(tools["solvers"][0], dict)
            or tools["solvers"][0].get("name") != "cadical"):
        raise Incomplete("JSON tool/target/solver identity differs from the admitted profile")
    harnesses = report.get("harness_metadata")
    if (not isinstance(harnesses, list) or len(harnesses) != 1
            or not isinstance(harnesses[0], dict) or harnesses[0].get("pretty_name") != job["harness"]
            or harnesses[0].get("has_loop_contracts") is not False
            or not isinstance(harnesses[0].get("attributes"), dict)
            or harnesses[0].get("attributes", {}).get("kind") != "Proof"
            or harnesses[0].get("attributes", {}).get("should_panic") is not False
            or (entrypoint is not None and
                (not isinstance(harnesses[0].get("source"), dict)
                 or harnesses[0]["source"].get("file") != entrypoint))):
        raise Incomplete("Harness metadata differs from the selected ordinary proof")
    cbmc = one_record(report, "cbmc", job["harness"])
    if (not isinstance(cbmc.get("configuration"), dict)
            or cbmc["configuration"].get("solver") != "cadical"):
        raise Incomplete("CBMC did not use the selected solver")
    if not isinstance(cbmc.get("cbmc_stats"), dict):
        raise Incomplete("Missing finite solver timing")
    solver_seconds = cbmc["cbmc_stats"].get("runtime_solver_s")
    if (isinstance(solver_seconds, bool) or not isinstance(solver_seconds, (int, float))
            or not math.isfinite(solver_seconds) or solver_seconds < 0):
        raise Incomplete("Missing finite solver timing")
    if job["case"] == "healthy":
        expected_covers = ROUNDING_COVERS if job["subject"] == "rounding" else ERA_COVERS
        expected_assertions = ROUNDING_ASSERTIONS if job["subject"] == "rounding" else ERA_ASSERTIONS
        successful_assertions = {
            item["description"].strip('"') for item in checks
            if item["category"] == "assertion" and item["status"] == "Success"
            and (entrypoint is None or isinstance(item.get("location"), dict)
                 and item["location"].get("file") == entrypoint)
        }
        if (native["exit_code"] != 0 or results[0].get("status") != "Success"
                or summary["successful"] != 1 or summary["failed"] != 0 or failures
                or errors.get("has_errors") is not False
                or len(covers) != len(expected_covers)
                or {item["description"].strip('"') for item in covers} != expected_covers
                or any(item["status"] != "Satisfied" for item in covers)
                or not expected_assertions.issubset(successful_assertions)
                or any(item["status"] == "Unsatisfiable" for item in checks)):
            raise Incomplete("Healthy proof, required covers or sanity obligations failed")
    else:
        intended = job["intended_failure"]
        if (native["exit_code"] == 0 or results[0].get("status") != "Failure"
                or summary["successful"] != 0 or summary["failed"] != 1 or not failures or covers
                or errors.get("has_errors") is not True
                or errors.get("error_type") != "assertion_failure"
                or errors.get("exit_status") != "properties_failed"
                or any(item["category"] != "assertion"
                       or item["description"].strip('"') != intended
                       or (entrypoint is not None and
                           (not isinstance(item.get("location"), dict)
                            or item["location"].get("file") != entrypoint))
                       for item in failures)):
            raise Incomplete("Defect did not fail exclusively at its intended assertion")
    return {"verification_summary": summary, "checks": len(checks), "status_counts": dict(counts),
            "failed_checks": failures, "covers": covers, "solver_seconds": solver_seconds,
            "reachability_and_sanity_checks": [item for item in checks
                if "reach" in item["category"] or "sanity" in item["category"]],
            "tools": tools, "metadata": metadata, "error_details": errors,
            "property_details": properties, "cbmc": cbmc, "harness_metadata": harnesses}


def admission(check, output: Path) -> tuple[dict, dict]:
    """Perform full payload admission and retain pointer, compiler and pin identity."""
    pointer = Path.home() / ".local/share/nevrotick/analysis-installation.json"
    started = time.monotonic()
    before = check.read_once(pointer, maximum=4096, private=True)
    admitted = check.admit(ROOT)
    if check.read_once(pointer, maximum=4096, private=True) != before:
        raise Incomplete("Installation pointer changed during full admission")
    if admitted.get("admission_profile") != "full":
        raise Incomplete("Full installation admission is required before every proof")
    kani = admitted["policy"]["tools"]["kani"]
    compiler = admitted["base"] / kani["toolchain"]
    record = {"seconds": time.monotonic() - started, "admitted": admitted,
              "installation_pointer": str(pointer), "installation_pointer_sha256": digest(before),
              "installation_pointer_record": check.decode(before),
              "kani_executable_sha256": check.digest(admitted["base"] / kani["entrypoint"]),
              "compiler_rustc_sha256": check.digest(compiler / "bin/rustc"),
              "compiler_rustdoc_sha256": check.digest(compiler / "bin/rustdoc"),
              "profile": {"kani_flags": check.PROFILES["kani_flags"], "rustflags": RUSTFLAGS,
                          "wall_seconds": 180, "cpu_seconds": 180,
                          "address_space_bytes": 8 * 1024 ** 3, "process_limit": 128,
                          "build_jobs": 2, "solver_workers": 1}}
    write_json(output, record)
    return admitted, record


def execute_job(check, work: Path, source: Path, records: dict, subject: str,
                job: dict, index: int) -> dict:
    """Prepare, fully admit, run once, preserve observations, and qualify one job."""
    started = time.monotonic()
    folder = work / "jobs" / f"{index:02d}-{job['subject']}-{job['case']}-{job['harness']}"
    inputs = folder / "inputs"
    inputs.mkdir(mode=0o700, parents=True)
    record = {"job": job, "expected_outcome_qualified": False, "output": str(folder)}
    try:
        require_sources(check, source, records, subject)
        # Preserve a complete runner/spec/policy/source copy for this individual command.
        for name in records:
            target = inputs / "frozen" / name
            target.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
            target.write_bytes((source / name).read_bytes())
        if job["subject"] == "rounding":
            prepared = prepare_rounding(source, inputs, job["case"])
        else:
            projection = module(f"formal_era_projection_{index}", source / "tools/formal/era_projection.py")
            prepared = projection.prepare_era(source, inputs, job["case"],
                                               source / "tools/formal/era_harness.rs")
        entry = prepared["entrypoint"]
        if (not isinstance(entry, Path) or entry != inputs / "proof.rs"
                or not entry.is_file() or entry != entry.resolve(strict=True)):
            raise Incomplete("Projection must return the canonical inputs/proof.rs")
        record.update(preparation=prepared, prepare_seconds=time.monotonic() - started,
                      source_original_manifest=records)
        before = check.input_snapshot(work, [inputs])
        record["input_before"] = before
        write_json(folder / "input-before.json", before)
        admitted, admitted_record = admission(check, folder / "admission.json")
        record["admission_seconds"] = admitted_record["seconds"]
        require_sources(check, source, records, subject)
        if check.input_snapshot(work, [inputs]) != before:
            raise Incomplete("Prepared inputs changed during full admission")
        base, stable = admitted["base"], admitted["stable"]
        kani = admitted["policy"]["tools"]["kani"]
        compiler = base / kani["toolchain"]
        namespace_entry = "/work/" + entry.relative_to(work).as_posix()
        command = [str(base / kani["entrypoint"]), namespace_entry,
                   "--harness", job["harness"], "--exact", *check.PROFILES["kani_flags"],
                   "--export-json", "/work/" + (folder / "kani.json").relative_to(work).as_posix(),
                   "--target-dir", "/work/" + (folder / "target").relative_to(work).as_posix()]
        environment = {"RUSTC": str(compiler / "bin/rustc"),
                       "RUSTDOC": str(compiler / "bin/rustdoc"),
                       "RUSTUP_HOME": str(base / "kani/rustup"),
                       "RUSTUP_TOOLCHAIN": "nightly-2026-08-21", "RUSTFLAGS": RUSTFLAGS}
        record.update(command=command, explicit_environment=environment)
        write_json(folder / "execution.json", record)
        execute_start = time.monotonic()
        native = check.RUNTIME.run(ROOT, work, command, readonly=[base, stable],
            path_dirs=[base / "kani/payload/kani-0.68.0/bin", compiler / "bin"], cwd=inputs,
            environment=environment, timeout=180, cpu_seconds=180, build_jobs=2,
            address_space_bytes=8 * 1024 ** 3, process_limit=128, sealed=[inputs])
        record.update(native=native, execute_seconds=time.monotonic() - execute_start)
        after = check.input_snapshot(work, [inputs])
        record.update(input_after=after, input_unchanged=before == after)
        write_json(folder / "input-after.json", after)
        require_sources(check, source, records, subject)
        if before != after:
            raise Incomplete("Complete proof input tree changed during execution")
        report = check.decode(check.read_once(folder / "kani.json"))
        observed = qualify(report, native, job, namespace_entry)
        record.update(observed)
        record["execute_other_seconds"] = record["execute_seconds"] - observed["solver_seconds"]
        record["expected_outcome_qualified"] = True
    except Exception as error:
        record["qualification_error"] = f"{type(error).__name__}: {error}"
    finally:
        record["job_total_seconds"] = time.monotonic() - started
        write_json(folder / "execution.json", record)
    return record


def main(argv=None) -> int:
    """Run a finite campaign and perform full final admission even after failure."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", required=True,
                        help="fresh canonical absolute private directory outside this checkout")
    parser.add_argument("--subject", choices=("rounding", "era", "all"), default="all")
    parser.add_argument("--start-at", type=int, default=1,
                        help="execute only this fixed-schedule suffix; omitted jobs are not qualified")
    args = parser.parse_args(argv)
    os.umask(0o077)
    work = fresh_output(args.output_dir)
    source = work / "source"
    summary = {"status": "incomplete", "subject": args.subject, "jobs": [],
               "maximum_jobs": MAX_JOBS, "root": str(ROOT), "output": str(work),
               "started_unix_ns": time.time_ns(), "retries": 0}
    started = time.monotonic()
    check = None
    records = None
    try:
        check = module("formal_analysis_admission", ROOT / "scripts/check-analysis.py")
        records = freeze_inputs(check, source, input_names(args.subject))
        require_sources(check, source, records, args.subject)
        write_json(work / "source-manifest.json", {"source_original_manifest": records,
                   "runner_sha256": records["tools/formal/verify.py"]["sha256"]})
        selection = selected_jobs(args.subject, args.start_at)
        summary.update(planned_jobs=[job for _, job in selection],
                       selection_start_at=args.start_at,
                       complete_schedule=args.start_at == 1,
                       fixed_schedule_jobs=len(jobs(args.subject)),
                       scope="selected proof commands only; omitted jobs require separate evidence")
        print(json.dumps({"status": "prepared", "jobs": len(selection),
                          "complete_schedule": summary["complete_schedule"], "output": str(work)}), flush=True)
        for index, job in selection:
            result = execute_job(check, work, source, records, args.subject, job, index)
            compact = {"index": index, "subject": job["subject"], "case": job["case"],
                       "harness": job["harness"], "output": result["output"],
                       "qualified": result["expected_outcome_qualified"],
                       "exit_code": result.get("native", {}).get("exit_code"),
                       "checks": result.get("checks"), "admission_seconds": result.get("admission_seconds"),
                       "execute_seconds": result.get("execute_seconds"),
                       "solver_seconds": result.get("solver_seconds"),
                       "qualification_error": result.get("qualification_error")}
            summary["jobs"].append(compact)
            write_json(work / "summary.json", summary)
            print(json.dumps(compact), flush=True)
            if not result["expected_outcome_qualified"]:
                raise Incomplete(f"Stopped at first unqualified job: {index} {job['harness']}")
        summary["status"] = "passed"
    except Exception as error:
        summary["status"] = "incomplete"
        summary["error"] = f"{type(error).__name__}: {error}"
    finally:
        if check is not None:
            try:
                _, final = admission(check, work / "final-admission.json")
                summary["final_admission_seconds"] = final["seconds"]
                if records is None:
                    raise Incomplete("No complete original source manifest was frozen")
                require_sources(check, source, records, args.subject)
                summary["original_and_frozen_inputs_unchanged"] = True
            except Exception as error:
                summary["status"] = "incomplete"
                summary["final_error"] = f"{type(error).__name__}: {error}"
        summary["total_seconds"] = time.monotonic() - started
        write_json(work / "summary.json", summary)
    print(json.dumps({"status": summary["status"], "completed_jobs": len(summary["jobs"]),
                      "summary": str(work / "summary.json"),
                      "error": summary.get("error"), "final_error": summary.get("final_error")}), flush=True)
    return 0 if summary["status"] == "passed" else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, Incomplete) as error:
        print(f"Formal verification incomplete: {error}", file=sys.stderr)
        raise SystemExit(1)
