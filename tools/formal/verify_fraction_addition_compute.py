#!/usr/bin/env python3
"""One closed longer-budget full-addition proof; ordinary defaults remain unchanged."""
from __future__ import annotations

PREPARATION_STARTED_UNIX_NS = 1793915211361402794
PREPARATION_STARTED_MONOTONIC_NS = 11879194829883
PREPARATION_BOOT_ID = 'd3fc5615-e867-45cd-b74f-4a7413afc5ab'

import argparse
import importlib.util
import json
import math
import os
from pathlib import Path
import sys
import time


ROOT = Path(__file__).resolve().parents[2]
CONSUMER = "fraction-addition-compute-pilot-v1"
FIXED_OUTPUT_LEAF = "formal-core-fraction-addition-z3-300-20261106-v1"
PREPARATION_DEADLINE_MONOTONIC_NS = 15479194829883
TOTAL_SECONDS = 3600
HARNESS_SECONDS = 300
COMMAND_SECONDS = 600
MODEL_SHA256 = {
    "proof.rs": "3ed88b432db17ada7145921ee9e64253c78c4009bccfcd5e7ae897b71cf38f2c",
    "fraction_arithmetic_selected.rs": "c433de9e562a7758436eaad3074dafad43ad3b4ebe16ecdc75d564ad0d4dc794",
    "correspondence.json": "413c34636d18cfe7284a790ccd6533708cabe57c586e0c0a5d34dbb0d11b8f9f",
}
MUTATIONS = {
    "always_error": (b"fn add_fractions(left: Fraction, right: Fraction) -> Result<FractionSum, FixtureEnclosureError> {",
                     b"fn add_fractions(left: Fraction, right: Fraction) -> Result<FractionSum, FixtureEnclosureError> {\n    return Err(FixtureEnclosureError::Invariant);"),
    "wrong_carry": (b"carry: comparison != Ordering::Less,", b"carry: comparison == Ordering::Greater,"),
    "wrong_residual": (b"residual: comparison != Ordering::Equal,", b"residual: comparison == Ordering::Greater,"),
}


def module(name: str, path: Path):
    """Load only a named repository/frozen source module."""
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError("Unavailable compute source module")
    loaded = importlib.util.module_from_spec(spec)
    sys.modules[name] = loaded
    spec.loader.exec_module(loaded)
    return loaded


BASE = module("fraction_compute_ordinary", ROOT / "tools/formal/verify.py")
Incomplete = BASE.Incomplete


def remaining_budget() -> float:
    """Keep the original preparation clock, including all admission/review work."""
    if Path("/proc/sys/kernel/random/boot_id").read_text().strip() != PREPARATION_BOOT_ID:
        raise Incomplete("Compute preparation belongs to a different host boot")
    monotonic_elapsed = (time.monotonic_ns() - PREPARATION_STARTED_MONOTONIC_NS) / 1e9
    wall_elapsed = (time.time_ns() - PREPARATION_STARTED_UNIX_NS) / 1e9
    remaining = TOTAL_SECONDS - max(monotonic_elapsed, wall_elapsed)
    if (monotonic_elapsed < 0 or wall_elapsed < 0
            or not math.isfinite(remaining) or remaining <= 0):
        raise Incomplete("Compute preparation/execution budget expired or clock moved backwards")
    return remaining


def compute_flags(check) -> list[str]:
    """Change only the dedicated harness duration after checking ordinary flags."""
    flags = BASE.solver_flags(check, "z3")
    position = flags.index("--harness-timeout") + 1
    if flags[position] != "60s":
        raise Incomplete("Compute pilot requires the unchanged ordinary 60-second baseline")
    flags[position] = "300s"
    return flags


def schedule(source: Path = ROOT) -> list[tuple[int, dict]]:
    """Select exactly the original complete addition group, preserving its indices."""
    jobs = BASE.additional_suites(source).jobs("fraction-arithmetic")
    expected = []
    for case, suffix, failure, restored in (
            ("healthy", "full_domain", None, False),
            ("always_error", "control_success", "addition required success", False),
            ("wrong_carry", "control_carry", "addition exact carry", False),
            ("wrong_residual", "control_residual", "addition exact residual", False),
            ("healthy", "full_domain", None, True)):
        expected.append({"subject": "fraction_addition", "case": case,
                         "harness": "fraction_arithmetic::fraction_addition_" + suffix,
                         "intended_failure": failure, "restored_healthy": restored})
    if len(jobs) != 10 or jobs[5:] != expected:
        raise Incomplete("Original five-job addition group changed")
    return list(zip(range(6, 11), expected))


def input_names() -> list[str]:
    """Freeze all original ordinary/Z3 inputs and this consumer's complete delta."""
    return sorted(set(BASE.input_names("fraction-arithmetic", "z3")) | {
        "tools/formal/verify_fraction_addition_compute.py",
        "tools/formal/test_verify_fraction_addition_compute.py",
        "scripts/tests/test_check_analysis.py",
    })


def output_preflight(value: str) -> Path:
    """Allow one fresh output, with no alternate directory or relaunch suffix."""
    expected = Path.home() / ".local/share/nevrotick" / FIXED_OUTPUT_LEAF
    if value != str(expected) or expected.exists() or expected.is_symlink():
        raise Incomplete("Compute card requires its one fixed fresh output directory")
    remaining_budget()
    return expected


def require_sources(check, source: Path, records: dict) -> None:
    """Check the complete superset rather than the ordinary runner's smaller list."""
    if sorted(records) != input_names():
        raise Incomplete("Compute input closure changed")
    for name, expected in records.items():
        for path in (ROOT / name, source / name):
            if path != path.resolve(strict=True):
                raise Incomplete("Compute source path redirects: " + name)
            data = check.read_once(path)
            if {"sha256": BASE.digest(data), "bytes": len(data)} != expected:
                raise Incomplete("Original/frozen compute input drift: " + name)


def baseline_inputs(check, work: Path) -> Path:
    """Authenticate the previous exact full-model Z3 attempt, without reusing a proof."""
    old = (Path.home() / ".local/share/nevrotick/formal-core-fraction-addition-z3-20261106-v1/jobs"
           / "06-fraction_addition-healthy-fraction_arithmetic::fraction_addition_full_domain/inputs")
    data = {}
    for name, expected in MODEL_SHA256.items():
        path = old / name
        if path != path.resolve(strict=True):
            raise Incomplete("Retained baseline model redirects")
        value = check.read_once(path)
        if BASE.digest(value) != expected:
            raise Incomplete("Retained full-model baseline changed: " + name)
        data[name] = value
    target = work / "baseline-inputs"
    target.mkdir(mode=0o700)
    for name, value in data.items():
        (target / name).write_bytes(value)
    return target


def require_model(check, inputs: Path, job: dict, baseline: Path) -> dict:
    """Bind actual Rust to the old full model and the three exact original mutants."""
    original = {}
    for name, expected in MODEL_SHA256.items():
        original[name] = check.read_once(baseline / name)
        if BASE.digest(original[name]) != expected:
            raise Incomplete("Anchored full-model baseline changed")
    if check.read_once(inputs / "proof.rs") != original["proof.rs"]:
        raise Incomplete("Full raw-domain proof/oracle bytes changed")
    selected = original["fraction_arithmetic_selected.rs"]
    if job["case"] != "healthy":
        before, after = MUTATIONS[job["case"]]
        if selected.count(before) != 1:
            raise Incomplete("Original source mutation point is not unique")
        selected = selected.replace(before, after, 1)
    if check.read_once(inputs / "fraction_arithmetic_selected.rs") != selected:
        raise Incomplete("Actual Rust is not the exact original healthy/mutant body")
    if job["case"] == "healthy" and check.read_once(inputs / "correspondence.json") != original["correspondence.json"]:
        raise Incomplete("Healthy full-model correspondence changed")
    return {"baseline_model_sha256": dict(MODEL_SHA256),
            "proof_and_actual_body_match_original_model": True,
            "selected_body_sha256": BASE.digest(selected)}


def admission(check, output: Path, source: Path, work: Path) -> tuple[dict, dict]:
    """Admit the dedicated role and exact three immutable runtime roots."""
    remaining_budget()
    started = time.monotonic()
    pointer = Path.home() / ".local/share/nevrotick/analysis-installation.json"
    before = check.read_once(pointer, maximum=4096, private=True)
    flags = compute_flags(check)
    admitted = check.admit_fraction_addition_compute_pilot(ROOT)
    if admitted.get("admission_consumer") != CONSUMER:
        raise Incomplete("Wrong compute admission consumer")
    readonly = BASE.proof_readonly_roots(admitted, "kani-proof")
    solver = BASE.admit_z3_solver(admitted, source, work=work)
    readonly += (solver["readonly_root"],)
    if len(set(readonly)) != 3 or check.read_once(pointer, maximum=4096, private=True) != before:
        raise Incomplete("Compute roots or installation pointer changed during admission")
    kani = admitted["policy"]["tools"]["kani"]
    compiler = admitted["base"] / kani["toolchain"]
    record = {"seconds": time.monotonic() - started, "consumer": CONSUMER,
              "admitted": admitted, "readonly_roots": readonly,
              "solver_backend": "z3", "solver_admission": solver,
              "installation_pointer_sha256": BASE.digest(before),
              "kani_executable_sha256": check.digest(admitted["base"] / kani["entrypoint"]),
              "compiler_rustc_sha256": check.digest(compiler / "bin/rustc"),
              "compiler_rustdoc_sha256": check.digest(compiler / "bin/rustdoc"),
              "profile": {"kani_flags": flags, "rustflags": BASE.RUSTFLAGS,
                          "wall_seconds": COMMAND_SECONDS, "cpu_seconds": COMMAND_SECONDS,
                          "address_space_bytes": 8 * 1024**3, "process_limit": 128,
                          "build_jobs": 2, "solver_workers": 1}}
    remaining_budget()
    BASE.write_json(output, record)
    return admitted, record


def admission_identity(record: dict) -> dict:
    """Bind every command/final bookend to one unchanged pointer and tool closure."""
    admitted = record["admitted"]
    return BASE.serializable({
        **{key: record[key] for key in ("consumer", "installation_pointer_sha256", "profile",
             "kani_executable_sha256", "compiler_rustc_sha256", "compiler_rustdoc_sha256", "readonly_roots")},
        **{key: admitted[key] for key in ("base", "stable", "policy_sha256", "inventories")},
        "solver": BASE.solver_identity(record["solver_admission"]),
    })


def require_admission_identity(record: dict, anchor: dict) -> None:
    identity = admission_identity(record)
    if not anchor:
        anchor.update(identity)
    elif identity != anchor:
        raise Incomplete("Compute campaign tool/pointer/solver identity changed")


def execute_job(check, work: Path, source: Path, records: dict, job: dict, index: int,
                retained: list, baseline: Path, *, anchor: dict | None = None) -> dict:
    """Own the larger-budget native boundary without changing the ordinary runner."""
    if (index, job) not in schedule(source):
        raise Incomplete("Compute command is outside its fixed five-job group")
    if anchor is None:
        anchor = {}
    started = time.monotonic()
    folder = work / "jobs" / f"{index:02d}-{job['subject']}-{job['case']}-{job['harness']}"
    inputs = folder / "inputs"
    inputs.mkdir(mode=0o700, parents=True)
    record = {"job": job, "index": index, "output": str(folder),
              "expected_outcome_qualified": False, "consumer": CONSUMER}
    try:
        remaining_budget()
        BASE.audit_retained(check, work, retained)
        require_sources(check, source, records)
        for name in records:
            target = inputs / "frozen" / name
            target.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
            target.write_bytes((source / name).read_bytes())
        prepared = BASE.additional_suites(source).prepare(BASE.module, source, inputs, job, index)
        entry = prepared["entrypoint"]
        if entry != inputs / "proof.rs" or entry != entry.resolve(strict=True) or not entry.is_file():
            raise Incomplete("Compute projection returned a different entrypoint")
        record.update(preparation=prepared, source_original_manifest=records,
                      model_parity=require_model(check, inputs, job, baseline))
        if job["restored_healthy"]:
            healthy_job = schedule(source)[0][1]
            healthy = work / "jobs" / f"06-{healthy_job['subject']}-{healthy_job['case']}-{healthy_job['harness']}" / "inputs"
            if BASE.generated_identity(check, inputs) != BASE.generated_identity(check, healthy):
                raise Incomplete("Restored complete generated/frozen input tree differs from healthy")
            record["restored_generated_input_matches_healthy"] = True
        before = check.input_snapshot(work, [inputs])
        BASE.write_json(folder / "input-before.json", before)
        record["input_before"] = before
        admitted, admitted_record = admission(check, folder / "admission.json", source, work)
        require_admission_identity(admitted_record, anchor)
        record["admission_seconds"] = admitted_record["seconds"]
        require_sources(check, source, records)
        if check.input_snapshot(work, [inputs]) != before:
            raise Incomplete("Prepared inputs changed during compute admission")
        base = admitted["base"]
        kani = admitted["policy"]["tools"]["kani"]
        compiler = base / kani["toolchain"]
        solver = BASE.checked_z3_admission(admitted_record["solver_admission"], admitted, source, work=work)
        readonly = (*BASE.proof_readonly_roots(admitted, "kani-proof"), solver["readonly_root"])
        if tuple(admitted_record["readonly_roots"]) != readonly:
            raise Incomplete("Compute runtime requires exactly the admitted three roots")
        namespace_entry = "/work/" + entry.relative_to(work).as_posix()
        command = [str(base / kani["entrypoint"]), namespace_entry,
                   "--harness", job["harness"], "--exact", *compute_flags(check),
                   "--export-json", "/work/" + (folder / "kani.json").relative_to(work).as_posix(),
                   "--target-dir", "/work/" + (folder / "target").relative_to(work).as_posix()]
        environment = {"RUSTC": str(compiler / "bin/rustc"), "RUSTDOC": str(compiler / "bin/rustdoc"),
                       "RUSTUP_HOME": str(base / "kani/rustup"), "RUSTUP_TOOLCHAIN": "nightly-2026-08-21",
                       "RUSTFLAGS": BASE.RUSTFLAGS}
        paths = [solver["path_dir"], base / "kani/payload/kani-0.68.0/bin", compiler / "bin"]
        record.update(command=command, explicit_environment=environment,
                      runtime_readonly_roots=readonly, runtime_path_dirs=paths)
        BASE.audit_retained(check, work, retained)
        pre_receipts = [folder / name for name in ("input-before.json", "admission.json", "execution.json")]
        BASE.write_json(folder / "execution.json", record)
        current_roots = [inputs, *pre_receipts]
        if (work / "summary.json").exists():
            current_roots.append(work / "summary.json")
        sealed = [path for item in retained for path in item["roots"]] + current_roots
        record["runtime_sealed_roots"] = sealed
        BASE.write_json(folder / "execution.json", record)
        current = check.input_snapshot(work, current_roots)
        budget = remaining_budget()
        execute_started = time.monotonic()
        native = check.RUNTIME.run(ROOT, work, command, readonly=readonly,
            path_dirs=paths, cwd=inputs, environment=environment,
            timeout=min(COMMAND_SECONDS, budget), cpu_seconds=COMMAND_SECONDS, build_jobs=2,
            address_space_bytes=8 * 1024**3, process_limit=128, sealed=sealed)
        record.update(native=native, execute_seconds=time.monotonic() - execute_started,
                      runtime_effective_wall_seconds=min(COMMAND_SECONDS, budget))
        check.RUNTIME._owned_work(work, ROOT)
        BASE.audit_retained(check, work, retained)
        if check.input_snapshot(work, current_roots) != current:
            raise Incomplete("Sealed command inputs or receipts changed during execution")
        after = check.input_snapshot(work, [inputs])
        record.update(input_after=after, input_unchanged=before == after)
        BASE.write_json(folder / "input-after.json", after)
        require_sources(check, source, records)
        if before != after:
            raise Incomplete("Complete compute input tree changed")
        solver_after = BASE.admit_z3_solver(admitted, source, work=work)
        BASE.write_json(folder / "solver-after-admission.json", solver_after)
        if BASE.solver_identity(solver_after) != BASE.solver_identity(solver):
            raise Incomplete("Z3 solver changed during compute execution")
        pointer = Path.home() / ".local/share/nevrotick/analysis-installation.json"
        if BASE.digest(check.read_once(pointer, maximum=4096, private=True)) != admitted_record["installation_pointer_sha256"]:
            raise Incomplete("Installation pointer changed during compute execution")
        report = check.decode(check.read_once(folder / "kani.json"))
        record.update(BASE.qualify(report, native, job, namespace_entry, expected_solver="z3"))
        require_sources(check, source, records)
        if check.input_snapshot(work, [inputs]) != before:
            raise Incomplete("Compute inputs changed during solver readmission or result qualification")
        remaining_budget()
        record["expected_outcome_qualified"] = True
    except Exception as error:
        record["qualification_error"] = f"{type(error).__name__}: {error}"
    finally:
        record["job_total_seconds"] = time.monotonic() - started
        try:
            BASE.write_json(folder / "execution.json", record)
            receipts = BASE.job_receipts(folder, "z3", required=record["expected_outcome_qualified"])
            BASE.retain_artifacts(check, work, retained, [inputs, *receipts])
        except Exception as error:
            record.update(expected_outcome_qualified=False, persistence_error=f"{type(error).__name__}: {error}")
            record.setdefault("qualification_error", record["persistence_error"])
            try:
                BASE.write_json(folder / "execution.json", record)
            except Exception as write_error:
                record["persistence_write_error"] = str(write_error)
    return record


def main(argv=None) -> int:
    """Run the single fresh five-command card, stopping at its first failure."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", required=True)
    args = parser.parse_args(argv)
    output_preflight(args.output_dir)
    os.umask(0o077)
    work = BASE.fresh_output(args.output_dir)
    source = work / "source"
    check = module("fraction_compute_admission", ROOT / "scripts/check-analysis.py")
    summary = {"status": "incomplete", "decision": "DEFER", "consumer": CONSUMER,
               "jobs": [], "maximum_proof_commands": 5, "retries": 0,
               "preparation_started_unix_ns": PREPARATION_STARTED_UNIX_NS,
               "scope": "complete addition group6-10 only; complement, enclosure and Core remain separate"}
    retained, anchor = [], {}
    records = None
    started = time.monotonic()
    try:
        records = BASE.freeze_inputs(check, source, input_names())
        require_sources(check, source, records)
        BASE.write_json(work / "source-manifest.json", {"source_original_manifest": records})
        baseline = baseline_inputs(check, work)
        BASE.retain_artifacts(check, work, retained, [source, work / "source-manifest.json", baseline])
        jobs = schedule(source)
        summary["planned_jobs"] = [job for _, job in jobs]
        BASE.write_json(work / "summary.json", summary)
        for index, job in jobs:
            remaining_budget()
            record = execute_job(check, work, source, records, job, index, retained, baseline, anchor=anchor)
            compact = {"index": index, "case": job["case"], "qualified": record["expected_outcome_qualified"],
                       "output": record["output"], "exit_code": record.get("native", {}).get("exit_code"),
                       "checks": record.get("checks"), "status_counts": record.get("status_counts"),
                       "admission_seconds": record.get("admission_seconds"),
                       "execute_seconds": record.get("execute_seconds"),
                       "qualification_error": record.get("qualification_error")}
            summary["jobs"].append(compact)
            BASE.write_json(work / "summary.json", summary)
            print(json.dumps(compact), flush=True)
            if not record["expected_outcome_qualified"]:
                raise Incomplete("Stopped at first unqualified compute job: " + str(index))
        remaining_budget()
        summary.update(status="passed", decision="AWAIT_INDEPENDENT_REVIEW")
    except Exception as error:
        summary.update(status="incomplete", decision="DEFER", error=f"{type(error).__name__}: {error}")
    finally:
        try:
            _, final = admission(check, work / "final-admission.json", source, work)
            require_admission_identity(final, anchor)
            if records is None:
                raise Incomplete("No complete compute source closure")
            require_sources(check, source, records)
            BASE.retain_artifacts(check, work, retained, [work / "final-admission.json"])
            remaining_budget()
            summary.update(final_admission_seconds=final["seconds"], original_and_frozen_inputs_unchanged=True)
            if summary["status"] == "passed":
                summary.update(all_proof_inputs_and_required_receipts_unchanged=True,
                               complete_addition_group=True)
            else:
                summary["retained_partial_inputs_and_receipts_unchanged"] = True
        except Exception as error:
            summary.update(status="incomplete", decision="DEFER", final_error=f"{type(error).__name__}: {error}")
        summary["campaign_seconds"] = time.monotonic() - started
        summary["preparation_to_terminal_seconds"] = (time.monotonic_ns() - PREPARATION_STARTED_MONOTONIC_NS) / 1e9
        BASE.write_json(work / "summary.json", summary)
    print(json.dumps({"status": summary["status"], "completed_jobs": len(summary["jobs"]),
                      "summary": str(work / "summary.json"), "error": summary.get("error"),
                      "final_error": summary.get("final_error")}), flush=True)
    return 0 if summary["status"] == "passed" else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (Incomplete, OSError, ValueError) as error:
        print(json.dumps({"status": "incomplete", "error": f"{type(error).__name__}: {error}"}), flush=True)
        raise SystemExit(1)
