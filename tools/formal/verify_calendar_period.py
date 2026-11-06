#!/usr/bin/env python3
"""Qualify only the source-bound auxiliary calendar period through direct SMT.

This separate candidate consumer installs nothing and imports no old receipt.
It supplies neither the product prefix/suffix proofs nor finite Gregorian laws.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import stat
import sys
import time


ROOT = Path(__file__).resolve().parents[2]
sys.dont_write_bytecode = True
MODEL_TERMS = frozenset(("cycles", "year", "cycle_year", "forward_input",
                         "forward_result", "cycle_result", "cycle_independent_result", "rhs_result"))
CASES = (("endpoint_min", False, "sat"), ("endpoint_max", False, "sat"),
         ("healthy", False, "unsat"), ("wrong_year_coefficient", False, "sat"),
         ("omit_century_subtraction", False, "sat"), ("wrong_period", False, "sat"),
         ("healthy", True, "unsat"))
INPUTS = (
    "tools/formal/calendar_composition_harness.rs", "tools/formal/calendar_period_smt.py",
    "tools/formal/verify_calendar_period.py", "tools/formal/solver_admission.py",
    "tools/crypto-aes-siv-witness/fuzz/input_custody.py", "policy/formal-solvers.json",
    "scripts/check-analysis.py", "scripts/analysis-runtime.py", "scripts/analysis-controls.py",
    "scripts/check-environment.py", "policy/analysis-tools.json", "policy/tool-manifest.json",
    ".agents/skills/testing/references/formal-verification.md",
)


class Incomplete(RuntimeError):
    """No completed source-bound theorem follows from this candidate result."""


def module(name: str, path: Path):
    """Load only a selected source file, including its frozen relative imports."""
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise Incomplete("Cannot load frozen proof consumer")
    loaded = importlib.util.module_from_spec(spec)
    sys.modules[name] = loaded
    spec.loader.exec_module(loaded)
    return loaded


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def serial(value):
    if isinstance(value, Path):
        return str(value)
    if isinstance(value, dict):
        return {str(key): serial(item) for key, item in value.items()}
    if isinstance(value, (tuple, list)):
        return [serial(item) for item in value]
    return value


def write(path: Path, value) -> None:
    """Write owned regular receipts without following or truncating aliases."""
    if not path.is_absolute() or path.parent != path.parent.resolve(strict=True):
        raise Incomplete("Artifact parent redirects")
    payload = json.dumps(serial(value), indent=2, allow_nan=False) + "\n"
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_CLOEXEC | os.O_NOFOLLOW | os.O_NONBLOCK, 0o600)
    try:
        info = os.fstat(descriptor)
        if not stat.S_ISREG(info.st_mode) or info.st_uid != os.geteuid() or info.st_nlink != 1:
            raise Incomplete("Refusing to overwrite an unowned, special or aliased artifact")
        # Inspect before truncation: an external hardlink must preserve its data.
        os.ftruncate(descriptor, 0)
        with os.fdopen(descriptor, "w", encoding="utf-8", closefd=False) as stream:
            stream.write(payload)
    finally:
        os.close(descriptor)


def parse_output(output: str, expected: str) -> dict[str, int] | None:
    """Accept one exact SMT status and, for SAT only, one bounded value list."""
    if (type(output) is not str or len(output) > 8192 or expected not in {"sat", "unsat"}
            or any(ord(char) < 32 and char not in "\r\n\t" for char in output)
            or any(ord(char) >= 127 for char in output)):
        raise Incomplete("Malformed bounded solver protocol")
    tokens = re.findall(r"\(|\)|[^\s()]+", output)
    if expected == "unsat":
        if tokens != ["unsat"]:
            raise Incomplete("Expected exactly one UNSAT result")
        return None
    if not tokens or tokens.pop(0) != "sat" or len(tokens) > 96:
        raise Incomplete("Expected one SAT result and one complete value list")
    position = 0

    def parse(depth=0):
        nonlocal position
        if depth > 3 or position >= len(tokens):
            raise Incomplete("Incomplete or deeply nested solver value list")
        token = tokens[position]
        position += 1
        if token == "(":
            items = []
            while position < len(tokens) and tokens[position] != ")":
                items.append(parse(depth + 1))
            if position >= len(tokens):
                raise Incomplete("Unclosed solver value list")
            position += 1
            return items
        if token == ")":
            raise Incomplete("Unexpected solver closing delimiter")
        return token

    values = parse()
    if position != len(tokens) or type(values) is not list or len(values) != len(MODEL_TERMS):
        raise Incomplete("Missing or extra solver values/results")
    model = {}
    for pair in values:
        if (type(pair) is not list or len(pair) != 2 or type(pair[0]) is not str
                or pair[0] not in MODEL_TERMS or pair[0] in model or type(pair[1]) is not str
                or len(pair[1]) > 128 or re.fullmatch(r"0|[1-9][0-9]*", pair[1]) is None):
            raise Incomplete("Solver values must be unique named nonnegative integers")
        model[pair[0]] = int(pair[1])
    if set(model) != MODEL_TERMS:
        raise Incomplete("Solver model does not cover every required term")
    return model


def qualify(native: dict, case: str, expected: str, spec) -> dict:
    """Require native completion, an exact answer and independently checked SAT values."""
    if not any(case == chosen and expected == status for chosen, _, status in CASES):
        raise Incomplete("Unknown fixed result contract")
    if (type(native) is not dict or not {"exit_code", "failure", "output"}.issubset(native)
            or type(native.get("exit_code")) is not int
            or native["exit_code"] != 0 or native.get("failure") is not None):
        raise Incomplete("Solver did not complete normally")
    model = parse_output(native.get("output"), expected)
    witness = None if model is None else spec.check_model(case, model)
    if model is not None and (type(witness) is not dict
            or witness.get("machine_bounds_hold") is not True
            or witness.get("period_holds") is not case.startswith("endpoint_")):
        raise Incomplete("SAT result lacks its independently checked intended witness")
    return {"expected_status": expected, "model": model, "checked_witness": witness,
            "qualified": True}


def identity(admitted: dict) -> dict:
    """Compare complete payload/custody identities without admission timing noise."""
    return {key: admitted[key] for key in (
        "name", "version", "readonly_root", "executable", "path_dir", "sha256",
        "policy_path", "policy_sha256", "manifest_sha256", "inventory", "closure",
        "store_identity_before", "store_identity_after", "ancestor_identity_before", "ancestor_identity_after")}


def require_sources(check, actual: Path, frozen: Path, records: dict) -> None:
    if set(records) != set(INPUTS):
        raise Incomplete("Frozen source closure differs from the fixed consumer")
    for name, expected in records.items():
        for base in (actual, frozen):
            path = base / name
            if path != path.resolve(strict=True):
                raise Incomplete("Proof source path redirects")
            data = check.read_once(path, maximum=4 * 1024 ** 2)
            if {"sha256": digest(data), "bytes": len(data)} != expected:
                raise Incomplete("Original or frozen proof source changed: " + name)


def require_generated(check, inputs: Path, prepared: dict) -> None:
    """Bind the sealed executable SMT bytes to the actual generator output."""
    for name, field in (("proof.smt2", "script_sha256"), ("correspondence.json", "metadata_sha256")):
        if digest(check.read_once(inputs / name, maximum=4 * 1024 ** 2)) != prepared[field]:
            raise Incomplete("Generated proof input differs from its source-bound preparation")


def audit_retained(check, work: Path, generated: list, completed: list) -> None:
    """Preserve every generated island and completed receipt through closeout."""
    for item in generated:
        require_generated(check, item["inputs"], item["preparation"])
        if check.input_snapshot(work, [item["inputs"]]) != item["snapshot"]:
            raise Incomplete("An earlier or current generated input island changed")
    for item in completed:
        if check.input_snapshot(work, [item["folder"]]) != item["snapshot"]:
            raise Incomplete("A completed proof or retained receipt changed")


def checked_solver(admission, frozen: Path, actual: Path, output: Path, requested=None) -> dict:
    admitted = admission.admit(frozen, requested)
    store = admitted["readonly_root"]
    if any(store.is_relative_to(path) or path.is_relative_to(store)
           for path in (actual, output, frozen)):
        raise Incomplete("Solver, checkout and evidence scopes must be disjoint")
    if (admitted["name"] != "z3" or admitted["version"] != "5.3.3"
            or admitted["executable"] != store / "bin/z3"
            or admitted["path_dir"] != store / "bin"):
        raise Incomplete("Unexpected direct-SMT executable identity")
    return admitted


def run_frozen(actual: Path, output: Path, frozen: Path, records: dict, requested=None) -> int:
    """Execute frozen code/policies with only the single admitted solver mounted."""
    admission = module("period_frozen_solver_admission", frozen / "tools/formal/solver_admission.py")
    check = admission.CHECK
    spec = module("period_frozen_specification", frozen / "tools/formal/calendar_period_smt.py")
    if set(spec.input_names()) != {"tools/formal/calendar_composition_harness.rs", "tools/formal/calendar_period_smt.py"}:
        raise Incomplete("Unexpected auxiliary expression-source closure")
    work = output / "work"
    work.mkdir(mode=0o700)
    ancestors = admission._ancestors(work)
    summary = {"status": "incomplete", "scope": "auxiliary Gregorian period and machine bounds only",
               "source_manifest": records, "jobs": [], "retries": 0, "planned_commands": len(CASES),
               "trusted_frontend": "closed reviewed Rust-expression-to-Int correspondence",
               "host_runtime": "existing-public-host-trusted-base-not-rehashed"}
    started = time.monotonic()
    generated, completed = [], []
    try:
        for index, (case, restored, expected) in enumerate(CASES, 1):
            require_sources(check, actual, frozen, records)
            folder = work / "jobs" / f"{index:02d}-{case}"
            inputs = folder / "inputs"
            folder.mkdir(mode=0o700, parents=True)
            record = {"index": index, "case": case, "restored_healthy": restored,
                      "expected_status": expected, "qualified": False, "output": folder}
            try:
                prepared = spec.prepare(frozen, inputs, case, restored_healthy=restored)
                expected_terms = MODEL_TERMS if expected == "sat" else frozenset()
                if (prepared["entrypoint"] != inputs / "proof.smt2" or prepared["expected_status"] != expected
                        or type(prepared["required_model_terms"]) is not list
                        or len(prepared["required_model_terms"]) != len(expected_terms)
                        or set(prepared["required_model_terms"]) != expected_terms):
                    raise Incomplete("SMT preparation differs from its fixed command contract")
                record["preparation"] = prepared
                require_generated(check, inputs, prepared)
                generated.append({"inputs": inputs, "preparation": prepared,
                                  "snapshot": check.input_snapshot(work, [inputs])})
                audit_retained(check, work, generated, completed)
                audited = [item["folder"] for item in completed] + [inputs]
                before = check.input_snapshot(work, audited)
                # This direct solver returns stdout only. All job inputs and
                # receipts, including the current one, can stay read-only.
                sealed = [work / "jobs"]
                admitted = checked_solver(admission, frozen, actual, output, requested)
                record["solver_admission"] = admitted
                require_sources(check, actual, frozen, records)
                audit_retained(check, work, generated, completed)
                if check.input_snapshot(work, audited) != before or admission._ancestors(work) != ancestors:
                    raise Incomplete("Input or evidence authority changed before execution")
                command = [str(admitted["executable"]), "-smt2",
                           "/work/" + prepared["entrypoint"].relative_to(work).as_posix()]
                record.update(command=command, runtime_readonly_roots=[admitted["readonly_root"]],
                              runtime_sealed_roots=sealed,
                              profile={"wall_seconds": 180, "cpu_seconds": 180, "solver_timeout_ms": 60_000,
                                       "address_space_bytes": 8 * 1024 ** 3, "process_limit": 128, "solver_workers": 1})
                write(folder / "execution.json", record)
                tick = time.monotonic()
                native = check.RUNTIME.run(frozen, work, command, readonly=[admitted["readonly_root"]],
                    path_dirs=[admitted["path_dir"]], sealed=sealed, cwd=inputs,
                    timeout=180, cpu_seconds=180, build_jobs=1,
                    address_space_bytes=8 * 1024 ** 3, process_limit=128)
                record.update(native=native, execute_seconds=time.monotonic() - tick)
                after = check.input_snapshot(work, audited)
                admitted_after = checked_solver(admission, frozen, actual, output, requested)
                record.update(input_before=before, input_after=after, solver_after_admission=admitted_after)
                if before != after or identity(admitted) != identity(admitted_after):
                    raise Incomplete("Solver or complete sealed inputs changed during execution")
                require_sources(check, actual, frozen, records)
                audit_retained(check, work, generated, completed)
                if admission._ancestors(work) != ancestors:
                    raise Incomplete("Evidence authority changed during execution")
                record.update(qualify(native, case, expected, spec))
            except Exception as error:
                record["error"] = f"{type(error).__name__}: {error}"
                raise
            finally:
                summary["jobs"].append(record)
                try:
                    write(folder / "execution.json", record)
                except Exception as error:
                    record.update(qualified=False, persistence_error=f"{type(error).__name__}: {error}")
                    raise
                if record["qualified"]:
                    completed.append({"folder": folder, "snapshot": check.input_snapshot(work, [folder])})
                print(json.dumps({"index": index, "case": case, "qualified": record["qualified"]}), flush=True)
            if not record["qualified"]:
                raise Incomplete("Stopped at first unqualified SMT command")
        summary["status"] = "passed"
    except Exception as error:
        summary["error"] = f"{type(error).__name__}: {error}"
    finally:
        try:
            require_sources(check, actual, frozen, records)
            summary["final_solver_admission"] = checked_solver(admission, frozen, actual, output, requested)
            audit_retained(check, work, generated, completed)
            if admission._ancestors(work) != ancestors:
                raise Incomplete("Evidence authority changed at closeout")
            summary["original_and_frozen_inputs_unchanged"] = True
            summary["all_generated_inputs_and_completed_receipts_unchanged"] = True
        except Exception as error:
            summary.update(status="incomplete", final_error=f"{type(error).__name__}: {error}")
        summary["total_seconds"] = time.monotonic() - started
        write(output / "summary.json", summary)
    print(json.dumps({"status": summary["status"], "commands": len(summary["jobs"]),
                      "summary": str(output / "summary.json")}), flush=True)
    return 0 if summary["status"] == "passed" else 1


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", required=True)
    parser.add_argument("--solver-root")
    args = parser.parse_args(argv)
    output = None
    try:
        value = Path(args.output_dir)
        if (not value.is_absolute() or value.exists() or value.is_symlink()
                or value.parent != value.parent.resolve(strict=True) or value.name in {".", ".."}):
            raise Incomplete("Evidence must be a new canonical absolute leaf under an existing parent")
        actual = ROOT.resolve(strict=True)
        if value.is_relative_to(actual) or actual.is_relative_to(value):
            raise Incomplete("Evidence must be outside the checkout")
        live_admission = module("period_parent_solver_preflight", actual / "tools/formal/solver_admission.py")
        live_solver = live_admission.admit(actual, args.solver_root)
        store = live_solver["readonly_root"]
        if value.is_relative_to(store) or store.is_relative_to(value):
            raise Incomplete("Evidence and solver store must be separate")
        previous_umask = os.umask(0o077)
        try:
            value.mkdir(mode=0o700)
            output = value
            live_admission._ancestors(output)
            frozen = output / "source"
            frozen.mkdir(mode=0o700)
            records = {}
            for name in INPUTS:
                path = actual / name
                if path != path.resolve(strict=True):
                    raise Incomplete("Source closure redirects")
                data = live_admission.CHECK.read_once(path, maximum=4 * 1024 ** 2)
                destination = frozen / name
                destination.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
                destination.write_bytes(data)
                destination.chmod(0o400)
                records[name] = {"sha256": digest(data), "bytes": len(data)}
            write(output / "source-manifest.json", records)
            consumer = module("period_frozen_consumer", frozen / "tools/formal/verify_calendar_period.py")
            return consumer.run_frozen(actual, output, frozen, records, args.solver_root)
        finally:
            os.umask(previous_umask)
    except Exception as error:
        message = f"{type(error).__name__}: {error}"
        if output is not None:
            write(output / "startup-incomplete.json", {"status": "incomplete", "error": message})
        print(message, file=sys.stderr)
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
