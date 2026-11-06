#!/usr/bin/env python3
"""Finite actual-source Kani function-contract pilot; install and patch nothing.

This separate consumer preserves the ordinary proof runner and flags. Callee
certificates require successful nonempty proofs of the same dependency closure;
checking-harness existence or an expected-negative result grants no certificate.
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
import re
import sys
import time

ROOT = Path(__file__).resolve().parents[2]


def module(name: str, path: Path):
    """Load a source-bound module with no package installation or search fallback."""
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError("Unavailable source module: " + str(path))
    loaded = importlib.util.module_from_spec(spec)
    sys.modules[name] = loaded
    spec.loader.exec_module(loaded)
    return loaded


BASE = module("fraction_contracts_base", ROOT / "tools/formal/verify.py")
Incomplete = BASE.Incomplete
STATUSES = BASE.STATUSES
one_record = BASE.one_record
digest = BASE.digest
POLICY_NAME = "tools/formal/fraction_contracts_policy.json"
CONSUMER = "fraction-contracts-pilot-v1"
JOB_IDS = ("mechanism_callee", "mechanism_caller", "mechanism_precondition",
           "mechanism_bad_callee", "constructor", "complement", "addition_healthy",
           "addition_reject_all", "addition_wrong_carry", "addition_wrong_residual",
           "addition_restored")


def policy_record(root: Path = ROOT) -> dict:
    """Require this one fixed experimental consumer and its unchanged resource cap."""
    policy = json.loads((root / POLICY_NAME).read_text())
    expected = {
        "schema_version": 1, "consumer": CONSUMER, "status": "candidate-only",
        "total_seconds": 3600, "maximum_native_proof_commands": 11,
        "solver": "cadical", "rustflags": BASE.RUSTFLAGS,
        "output_leaf": "formal-core-fraction-contracts-20261106-v1",
        "command_wall_seconds": 180, "command_cpu_seconds": 180,
        "address_space_bytes": 8 * 1024 ** 3, "process_limit": 128,
        "build_jobs": 2, "solver_workers": 1, "retries": 0,
        "flags": [*BASE.KANI_BASELINE_FLAGS, "-Z", "function-contracts", "-Z", "stubbing"],
    }
    if not isinstance(policy, dict) or any(type(policy.get(k)) is not type(v) or policy[k] != v for k, v in expected.items()):
        raise Incomplete("Contract-pilot policy differs from its exact closed profile")
    extra_keys = {"preparation_started_unix_ns", "preparation_started_monotonic_ns",
                  "preparation_boot_id", "source_sha256"}
    if set(policy) != set(expected) | extra_keys:
        raise Incomplete("Unknown or missing contract-pilot policy fields")
    source_hashes = {
        "crates/time-core/src/projection_enclosure.rs":
            "6b7b079f7d820e611bd13bb96361c243973d231a7f65a07fb6d4350a63282023",
        "crates/time-core/src/selection.rs":
            "da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e",
    }
    if policy.get("source_sha256") != source_hashes:
        raise Incomplete("Contract-pilot product source pins differ")
    for key in ("preparation_started_unix_ns", "preparation_started_monotonic_ns"):
        if type(policy.get(key)) is not int or policy[key] <= 0:
            raise Incomplete("Contract-pilot preparation start must be recorded")
    if policy.get("preparation_boot_id") != Path("/proc/sys/kernel/random/boot_id").read_text().strip():
        raise Incomplete("Pilot preparation belongs to a different host boot")
    return policy


def remaining_seconds(policy: dict) -> float:
    """Enforce the original preparation-to-decision deadline without a fresh clock."""
    elapsed = (time.monotonic_ns() - policy["preparation_started_monotonic_ns"]) / 1e9
    wall_elapsed = (time.time_ns() - policy["preparation_started_unix_ns"]) / 1e9
    remaining = policy["total_seconds"] - max(elapsed, wall_elapsed)
    if elapsed < 0 or wall_elapsed < 0 or not math.isfinite(remaining) or remaining <= 0:
        raise Incomplete("Contract-pilot total preparation/execution budget expired")
    return remaining


def normalized_clause(value: str) -> str:
    """Ignore only Rust stringify whitespace, not operators, names or literals."""
    return re.sub(r"\s+", "", value.strip('"'))


def validate_contract_properties(checks: list[dict], job: dict) -> None:
    """Require actual surviving postcondition/precondition assertions as applicable."""
    if job["case"] != "healthy":
        return
    for field in ("required_contract_assertions", "required_precondition_assertions"):
        for clause in job.get(field, []):
            found = [c for c in checks if c["category"] == "assertion"
                     and normalized_clause(clause) == normalized_clause(c["description"])]
            if not found or any(c["status"] != "Success" for c in found):
                raise Incomplete("Missing or unproved contract assertion: " + clause)


def validate_harness(report: dict, native: dict, job: dict, entrypoint: str | None) -> None:
    """Validate actual checking/replacement metadata without coercing ordinary Proof."""
    if not isinstance(report, dict) or not isinstance(native, dict):
        raise Incomplete("Native and Kani reports must be objects")
    if "failure" not in native or type(native.get("exit_code")) is not int:
        raise Incomplete("Native outcome metadata is incomplete")
    harnesses = report.get("harness_metadata")
    if not isinstance(harnesses, list) or len(harnesses) != 1 or not isinstance(harnesses[0], dict):
        raise Incomplete("Contract pilot requires exactly one selected harness")
    harness = harnesses[0]
    if (not isinstance(harness.get("attributes"), dict)
            or harness["attributes"].get("kind") != job["expected_kind"]):
        raise Incomplete("Wrong contract checking/replacement harness kind")
    contract = harness.get("contract")
    if not isinstance(contract, dict) or set(contract) != {"contracted_function_name", "recursion_tracker"}:
        raise Incomplete("Missing closed native contract metadata")
    symbol = contract["contracted_function_name"]
    checking = job["expected_kind"].startswith("ProofForContract {")
    if (contract["recursion_tracker"] is not None
            or (checking and (not isinstance(symbol, str) or not symbol.strip()))
            or (not checking and (job["expected_kind"] != "Proof" or symbol is not None))):
        raise Incomplete("Unexpected recursive or missing contract enforcement symbol")
    output = native.get("output")
    if not isinstance(output, str) or "Thread " in output:
        raise Incomplete("Missing single-worker native contract output")
    headers = [line for line in output.splitlines() if line.startswith("Checking harness ")]
    if headers != ["Checking harness " + job["harness"] + "..."]:
        raise Incomplete("Native output selected a different or multiple harnesses")
    if any(line.lstrip().startswith("- Stub:") for line in output.splitlines()):
        raise Incomplete("Unverified ordinary stubs are forbidden")
    actual = [line.split("- Verified stub: ", 1)[1].strip() for line in output.splitlines()
              if line.strip().startswith("- Verified stub: ")]
    expected = job["verified_stubs"]
    if len(actual) != len(expected) or len(set(actual)) != len(actual):
        raise Incomplete("Resolved verified-stub count differs from the closed job")
    expected_names = ["proof::fraction_contracts::" + target for target in expected]
    if sorted(actual) != sorted(expected_names):
        raise Incomplete("Resolved verified-stub identities differ from the fixed source layout")



def qualify_result(report: dict, native: dict, job: dict, entrypoint: str | None = None,
            *, expected_solver: str = "cadical", allowed_files: tuple[str, ...] = ()) -> dict:
    """Fail closed on empty, partial, mismatched or unintended Kani outcomes."""
    if expected_solver != "cadical":
        raise Incomplete("Contract pilot has one fixed solver")
    validate_harness(report, native, job, entrypoint)
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
            or tools["solvers"][0].get("name") != expected_solver):
        raise Incomplete("JSON tool/target/solver identity differs from the admitted profile")
    harnesses = report.get("harness_metadata")
    if (not isinstance(harnesses, list) or len(harnesses) != 1
            or not isinstance(harnesses[0], dict) or harnesses[0].get("pretty_name") != job["harness"]
            or harnesses[0].get("has_loop_contracts") is not False
            or not isinstance(harnesses[0].get("attributes"), dict)
            or harnesses[0].get("attributes", {}).get("kind") != job["expected_kind"]
            or harnesses[0].get("attributes", {}).get("should_panic") is not False
            or (entrypoint is not None and
                (not isinstance(harnesses[0].get("source"), dict)
                 or harnesses[0]["source"].get("file") != entrypoint))):
        raise Incomplete("Harness metadata differs from the selected ordinary proof")
    cbmc = one_record(report, "cbmc", job["harness"])
    if (not isinstance(cbmc.get("configuration"), dict)
            or cbmc["configuration"].get("solver") != expected_solver):
        raise Incomplete("CBMC did not use the selected solver")
    if not isinstance(cbmc.get("cbmc_stats"), dict):
        raise Incomplete("Missing finite solver timing")
    solver_seconds = cbmc["cbmc_stats"].get("runtime_solver_s")
    decision_seconds = cbmc["cbmc_stats"].get("runtime_decision_procedure_s")

    def finite_seconds(value):
        return (not isinstance(value, bool) and isinstance(value, (int, float))
                and math.isfinite(value) and value >= 0)

    if solver_seconds is None and expected_solver == "z3":
        if not finite_seconds(decision_seconds):
            raise Incomplete("Z3 requires finite decision-procedure timing when pure solver timing is absent")
        timing_field = "runtime_decision_procedure_s"
        timing_meaning = "reported CBMC decision-procedure time; not isolated solver runtime"
    else:
        if not finite_seconds(solver_seconds):
            raise Incomplete("Missing finite solver timing")
        timing_field = "runtime_solver_s"
        timing_meaning = "reported solver runtime"
    if expected_solver == "z3" and decision_seconds is not None and not finite_seconds(decision_seconds):
        raise Incomplete("Malformed Z3 decision-procedure timing")
    if job["case"] == "healthy":
        expected_assertions = set(job["expected_assertions"])
        expected_covers = set(job["expected_covers"])
        if (not expected_covers
                or (not expected_assertions and not job.get("required_contract_assertions"))):
            raise Incomplete("Contract pilot requires nonempty mathematical/contract assertions and covers")
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
                       or normalized_clause(intended) != normalized_clause(item["description"])
                       or (entrypoint is not None and
                           (not isinstance(item.get("location"), dict)
                            or item["location"].get("file") not in (allowed_files or (entrypoint,))))
                       for item in failures)):
            raise Incomplete("Defect did not fail exclusively at its intended assertion")
    validate_contract_properties(checks, job)
    return {"verification_summary": summary, "checks": len(checks), "status_counts": dict(counts),
            "failed_checks": failures, "covers": covers, "solver_seconds": solver_seconds,
            "decision_procedure_seconds": decision_seconds if expected_solver == "z3" else None,
            "timing_field": timing_field, "timing_meaning": timing_meaning,
            "reachability_and_sanity_checks": [item for item in checks
                if "reach" in item["category"] or "sanity" in item["category"]],
            "tools": tools, "metadata": metadata, "error_details": errors,
            "expected_solver_backend": expected_solver,
            "property_details": properties, "cbmc": cbmc, "harness_metadata": harnesses}

def input_names() -> list[str]:
    """Close over every ordinary helper and this candidate's specs/policy/tests."""
    return sorted(set(BASE.input_names("fraction-arithmetic")) | {
        "tools/formal/fraction_constructor_projection.py",
        "tools/formal/fraction_constructor_harness.rs",
        "tools/formal/test_fraction_constructor.py",
        "tools/formal/fraction_contracts_projection.py",
        "tools/formal/fraction_contracts_spec.rs",
        "tools/formal/test_fraction_contracts_projection.py",
        "tools/formal/verify_fraction_contracts.py",
        "tools/formal/test_verify_fraction_contracts.py", POLICY_NAME,
        "scripts/tests/test_check_analysis.py",
    })


def require_sources(check, source: Path, records: dict) -> None:
    """Reject missing, changed or additional original/frozen proof input files."""
    if sorted(records) != input_names():
        raise Incomplete("Contract-pilot input closure changed")
    for name, expected in records.items():
        for path in (ROOT / name, source / name):
            if path != path.resolve(strict=True):
                raise Incomplete("Source path redirects: " + name)
            data = check.read_once(path)
            if {"sha256": digest(data), "bytes": len(data)} != expected:
                raise Incomplete("Original/frozen contract input drift: " + name)


def admission(check, output: Path, policy: dict) -> tuple[dict, dict]:
    """Admit only the dedicated contract consumer, with exact original tool roots."""
    remaining_seconds(policy)
    pointer = Path.home() / ".local/share/nevrotick/analysis-installation.json"
    before = check.read_once(pointer, maximum=4096, private=True)
    started = time.monotonic()
    admitted = check.admit_fraction_contracts_pilot(ROOT)
    if admitted.get("admission_consumer") != CONSUMER:
        raise Incomplete("Wrong dedicated contract consumer")
    readonly = BASE.proof_readonly_roots(admitted, "kani-proof")
    if check.read_once(pointer, maximum=4096, private=True) != before:
        raise Incomplete("Installation pointer changed during contract admission")
    kani = admitted["policy"]["tools"]["kani"]
    compiler = admitted["base"] / kani["toolchain"]
    record = {
        "admitted": admitted, "seconds": time.monotonic() - started,
        "readonly_roots": readonly, "consumer": CONSUMER,
        "installation_pointer_sha256": digest(before),
        "kani_executable_sha256": check.digest(admitted["base"] / kani["entrypoint"]),
        "compiler_rustc_sha256": check.digest(compiler / "bin/rustc"),
        "compiler_rustdoc_sha256": check.digest(compiler / "bin/rustdoc"),
        "profile": policy,
    }
    BASE.write_json(output, record)
    return admitted, record


def tool_identity(record: dict) -> str:
    """Bind callee certificates to transitive admitted bytes, roots and fixed flags."""
    a = record["admitted"]
    closure = {key: a[key] for key in ("policy_sha256", "inventories", "readonly_roots")}
    closure.update({key: record[key] for key in (
        "installation_pointer_sha256", "kani_executable_sha256", "compiler_rustc_sha256",
        "compiler_rustdoc_sha256", "consumer", "profile")})
    return digest(json.dumps(BASE.serializable(closure), sort_keys=True).encode())


def successful_native(native) -> bool:
    """A real complete successful outcome has an integer exit and explicit custody result."""
    return (isinstance(native, dict) and type(native.get("exit_code")) is int
            and native["exit_code"] == 0 and "failure" in native and native["failure"] is None)


def require_callees(job: dict, prepared: dict, certificates: dict, identity: str,
                    check, work: Path) -> None:
    """Refuse stale/failed/incomplete checking evidence before any abstracted caller."""
    for dependency in job["dependencies"]:
        certificate = certificates.get(dependency)
        if not isinstance(certificate, dict) or certificate.get("qualified") is not True:
            raise Incomplete("Missing qualified callee certificate: " + dependency)
        if (certificate.get("tool_identity") != identity
                or certificate.get("dependency_binding") != prepared["callee_bindings"].get(dependency)):
            raise Incomplete("Callee body/spec/type/generator/tool closure changed: " + dependency)
        roots = [Path(p) for p in certificate["receipt_roots"]]
        if check.input_snapshot(work, roots) != certificate["receipt_snapshot"]:
            raise Incomplete("Successful callee receipts changed: " + dependency)
        # In-memory qualification alone never substitutes for retained real results.
        execution = check.decode(check.read_once(Path(certificate["execution_receipt"])))
        native = execution.get("native", {})
        if (execution.get("expected_outcome_qualified") is not True
                or execution.get("job", {}).get("id") != dependency
                or execution.get("job", {}).get("case") != "healthy"
                or not execution["job"].get("expected_kind", "").startswith("ProofForContract {")
                or not successful_native(native)
                or type(execution.get("checks")) is not int or execution["checks"] <= 0
                or execution.get("input_unchanged") is not True):
            raise Incomplete("Retained callee result is incomplete or failed: " + dependency)


def issue_certificate(check, work: Path, record: dict) -> dict:
    """Only successful full checking jobs can issue a custody-bound callee fact."""
    job = record["job"]
    if (job["id"] not in {"mechanism_callee", "constructor", "complement"}
            or job["case"] != "healthy" or record.get("expected_outcome_qualified") is not True
            or not job["expected_kind"].startswith("ProofForContract {")
            or not successful_native(record.get("native"))
            or record.get("input_unchanged") is not True
            or type(record.get("checks")) is not int or record["checks"] <= 0):
        raise Incomplete("Only qualified healthy callee checking proofs issue certificates")
    folder = Path(record["output"])
    roots = [folder / "inputs", *BASE.job_receipts(folder, "cadical", required=True)]
    return {"qualified": True, "job_id": job["id"],
            "dependency_binding": record["preparation"]["callee_bindings"][job["id"]],
            "tool_identity": record["tool_identity"],
            "execution_receipt": str(folder / "execution.json"),
            "receipt_roots": [str(p) for p in roots],
            "receipt_snapshot": check.input_snapshot(work, roots)}


def execute_job(check, work: Path, source: Path, records: dict, projection,
                policy: dict, job: dict, index: int, retained: list, certificates: dict) -> dict:
    """Run one fixed command, sealing every prior proof and required receipt."""
    started = time.monotonic()
    folder = work / "jobs" / f"{index:02d}-{job['id']}"
    inputs = folder / "inputs"
    inputs.mkdir(mode=0o700, parents=True)
    record = {"job": job, "output": str(folder), "expected_outcome_qualified": False}
    try:
        remaining_seconds(policy)
        BASE.audit_retained(check, work, retained)
        require_sources(check, source, records)
        for name in records:
            target = inputs / "frozen" / name
            target.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
            target.write_bytes((source / name).read_bytes())
        prepared = projection.prepare(source, inputs, job)
        entry = prepared["entrypoint"]
        if entry != inputs / "proof.rs" or entry != entry.resolve(strict=True) or not entry.is_file():
            raise Incomplete("Contract projection must return canonical inputs/proof.rs")
        record.update(preparation=prepared, source_original_manifest=records,
                      prepare_seconds=time.monotonic() - started)
        before = check.input_snapshot(work, [inputs])
        BASE.write_json(folder / "input-before.json", before)
        record["input_before"] = before
        admitted, admission_record = admission(check, folder / "admission.json", policy)
        record.update(admission_seconds=admission_record["seconds"],
                      tool_identity=tool_identity(admission_record))
        require_sources(check, source, records)
        if check.input_snapshot(work, [inputs]) != before:
            raise Incomplete("Prepared contract inputs changed during admission")
        require_callees(job, prepared, certificates, record["tool_identity"], check, work)
        base = admitted["base"]
        kani = admitted["policy"]["tools"]["kani"]
        compiler = base / kani["toolchain"]
        namespace_entry = "/work/" + entry.relative_to(work).as_posix()
        command = [str(base / kani["entrypoint"]), namespace_entry,
                   "--harness", job["harness"], "--exact", *policy["flags"],
                   "--export-json", "/work/" + (folder / "kani.json").relative_to(work).as_posix(),
                   "--target-dir", "/work/" + (folder / "target").relative_to(work).as_posix()]
        environment = {"RUSTC": str(compiler / "bin/rustc"),
                       "RUSTDOC": str(compiler / "bin/rustdoc"),
                       "RUSTUP_HOME": str(base / "kani/rustup"),
                       "RUSTUP_TOOLCHAIN": "nightly-2026-08-21", "RUSTFLAGS": policy["rustflags"]}
        paths = [base / "kani/payload/kani-0.68.0/bin", compiler / "bin"]
        record.update(command=command, explicit_environment=environment,
                      runtime_readonly_roots=admitted["readonly_roots"], runtime_path_dirs=paths,
                      dependencies=job["dependencies"])
        BASE.audit_retained(check, work, retained)
        # Recheck callee receipts after command assembly, before the real native effect.
        require_callees(job, prepared, certificates, record["tool_identity"], check, work)
        pre_receipts = [folder / n for n in ("input-before.json", "admission.json", "execution.json")]
        BASE.write_json(folder / "execution.json", record)
        current_roots = [inputs, *pre_receipts]
        if (work / "summary.json").exists():
            current_roots.append(work / "summary.json")
        sealed = [p for item in retained for p in item["roots"]] + current_roots
        record["runtime_sealed_roots"] = sealed
        BASE.write_json(folder / "execution.json", record)
        current = check.input_snapshot(work, current_roots)
        remaining = remaining_seconds(policy)
        native_started = time.monotonic()
        native = check.RUNTIME.run(ROOT, work, command,
            readonly=admitted["readonly_roots"], path_dirs=paths, cwd=inputs,
            environment=environment, timeout=min(policy["command_wall_seconds"], remaining),
            cpu_seconds=policy["command_cpu_seconds"], build_jobs=policy["build_jobs"],
            address_space_bytes=policy["address_space_bytes"], process_limit=policy["process_limit"],
            sealed=sealed)
        record.update(native=native, execute_seconds=time.monotonic() - native_started)
        check.RUNTIME._owned_work(work, ROOT)
        BASE.audit_retained(check, work, retained)
        if check.input_snapshot(work, current_roots) != current:
            raise Incomplete("Sealed contract inputs or prior receipts changed during execution")
        after = check.input_snapshot(work, [inputs])
        record.update(input_after=after, input_unchanged=before == after)
        BASE.write_json(folder / "input-after.json", after)
        require_sources(check, source, records)
        if before != after:
            raise Incomplete("Contract input tree changed during execution")
        report = check.decode(check.read_once(folder / "kani.json"))
        allowed_files = tuple("/work/" + p.relative_to(work).as_posix() for p in inputs.glob("*.rs"))
        record.update(qualify_result(report, native, job, namespace_entry, allowed_files=allowed_files))
        remaining_seconds(policy)
        record["expected_outcome_qualified"] = True
    except Exception as error:
        record["qualification_error"] = f"{type(error).__name__}: {error}"
    finally:
        record["job_total_seconds"] = time.monotonic() - started
        try:
            BASE.write_json(folder / "execution.json", record)
            receipts = BASE.job_receipts(folder, "cadical", required=record["expected_outcome_qualified"])
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
    """Run the single eleven-command candidate; no suffix/retry/extra-flag API exists."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", required=True)
    args = parser.parse_args(argv)
    policy = policy_record()
    remaining_seconds(policy)
    output = Path.home() / ".local/share/nevrotick" / policy["output_leaf"]
    if args.output_dir != str(output):
        raise Incomplete("This one-launch pilot requires its fixed fresh output directory")
    os.umask(0o077)
    work = BASE.fresh_output(args.output_dir)
    source = work / "source"
    check = module("fraction_contracts_admission", ROOT / "scripts/check-analysis.py")
    summary = {"status": "incomplete", "decision": "DEFER", "consumer": CONSUMER,
               "jobs": [], "certificates": {}, "maximum_jobs": 11, "retries": 0,
               "preparation_started_unix_ns": policy["preparation_started_unix_ns"],
               "scope": "candidate-only actual-source function-contract composition; no Core/P1 closure"}
    retained = []
    records = None
    started = time.monotonic()
    try:
        records = BASE.freeze_inputs(check, source, input_names())
        require_sources(check, source, records)
        BASE.write_json(work / "source-manifest.json", {"source_original_manifest": records})
        BASE.retain_artifacts(check, work, retained, [source, work / "source-manifest.json"])
        projection = module("fraction_contracts_frozen_projection", source / "tools/formal/fraction_contracts_projection.py")
        schedule = projection.jobs()
        if (not isinstance(schedule, list) or len(schedule) != 11
                or tuple(j.get("id") for j in schedule) != JOB_IDS):
            raise Incomplete("Contract pilot requires its full fixed eleven-command schedule")
        summary.update(planned_jobs=schedule, complete_schedule=True)
        BASE.write_json(work / "summary.json", summary)
        print(json.dumps({"status": "prepared", "jobs": 11, "output": str(work)}), flush=True)
        certificates = {}
        for index, job in enumerate(schedule, 1):
            remaining_seconds(policy)
            require_sources(check, source, records)
            record = execute_job(check, work, source, records, projection, policy, job, index, retained, certificates)
            compact = {"index": index, "id": job["id"], "qualified": record["expected_outcome_qualified"],
                       "output": record["output"], "exit_code": record.get("native", {}).get("exit_code"),
                       "checks": record.get("checks"), "status_counts": record.get("status_counts"),
                       "admission_seconds": record.get("admission_seconds"),
                       "execute_seconds": record.get("execute_seconds"),
                       "qualification_error": record.get("qualification_error")}
            summary["jobs"].append(compact)
            BASE.write_json(work / "summary.json", summary)
            print(json.dumps(compact), flush=True)
            if not record["expected_outcome_qualified"]:
                raise Incomplete("Stopped at first unqualified contract-pilot job: " + job["id"])
            if job["id"] in {"mechanism_callee", "constructor", "complement"}:
                certificate = issue_certificate(check, work, record)
                certificates[job["id"]] = certificate
                path = Path(record["output"]) / "certificate.json"
                BASE.write_json(path, certificate)
                BASE.retain_artifacts(check, work, retained, [path])
                summary["certificates"][job["id"]] = certificate
        remaining_seconds(policy)
        summary.update(status="passed", decision="AWAIT_INDEPENDENT_REVIEW")
    except Exception as error:
        summary.update(status="incomplete", decision="DEFER", error=f"{type(error).__name__}: {error}")
    finally:
        try:
            _, final = admission(check, work / "final-admission.json", policy)
            summary["final_admission_seconds"] = final["seconds"]
            if records is None:
                raise Incomplete("No complete source input closure")
            require_sources(check, source, records)
            BASE.retain_artifacts(check, work, retained, [work / "final-admission.json"])
            remaining_seconds(policy)
            summary["original_and_frozen_inputs_unchanged"] = True
            if summary["status"] == "passed":
                summary["all_proof_inputs_and_required_receipts_unchanged"] = True
            else:
                summary["retained_partial_inputs_and_receipts_unchanged"] = True
        except Exception as error:
            summary.update(status="incomplete", decision="DEFER", final_error=f"{type(error).__name__}: {error}")
        summary["campaign_seconds"] = time.monotonic() - started
        summary["preparation_to_terminal_seconds"] = (
            time.monotonic_ns() - policy["preparation_started_monotonic_ns"]) / 1e9
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
