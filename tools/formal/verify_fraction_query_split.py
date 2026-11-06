#!/usr/bin/env python3
"""One fixed query-separated pure-fraction card; install or patch nothing.

Both full-domain universal and all23 direct-source witness lanes are mandatory.
The original composition controller is never executed or reconfigured. Its
reviewed pure qualification/metadata helpers retain their original behavior.
"""
from __future__ import annotations

import sys
# This must precede every project import, including an ordinary import of us.
sys.dont_write_bytecode = True

import argparse
import importlib.util
import json
import math
import os
from pathlib import Path
import stat
import time

ROOT = Path(__file__).resolve().parents[2]
CONSUMER = "fraction-query-split-pilot-v1"
OUTPUT_LEAF = "formal-core-fraction-query-split-20261106-v1"
PREPARATION_STARTED_UNIX_NS = 1793921076355277638
PREPARATION_STARTED_MONOTONIC_NS = 17744188704676
PREPARATION_BOOT_ID = "d3fc5615-e867-45cd-b74f-4a7413afc5ab"
DEADLINE_MONOTONIC_NS = 21344188704676
TOTAL_SECONDS = 3600
COMMAND_SECONDS = 180
MAXIMUM_PROOF_COMMANDS = 20
CALLEES = ("constructor", "comparison", "complement")
JOB_IDS = ("mech_healthy", "mech_assert_then_assume", "constructor_bad_post",
           "constructor_wrong_seed", "constructor", "comparison_bad_post",
           "comparison_wrong_seed", "comparison", "complement_bad_post",
           "complement_wrong_seed", "complement", "comparison_bad_precondition",
           "complement_bad_precondition", "addition_universal_healthy", "addition_reject_all",
           "addition_wrong_carry", "addition_wrong_residual", "addition_universal_restored",
           "addition_witnesses_healthy", "addition_witnesses_restored")
SOURCE_NAMES = ("tools/formal/fraction_query_split_projection.py",
                "tools/formal/test_fraction_query_split.py",
                "tools/formal/verify_fraction_query_split.py",
                "tools/formal/test_verify_fraction_query_split.py")
PREFIX_SHA256 = "ff1d5e2118dacde0f5aaf25eec94d40ac2a9dbf3419bb039926515e0c05a3a3e"
FAULTS_SHA256 = "a2f091edd0ae2f3567392a1ce1b91f4cb3f2a2f868419baa7d3122d2f6402da7"
MODEL_PRECONDITIONS = ["composition comparison checked precondition",
                       "composition complement checked precondition"]


def module(name: str, path: Path):
    """Import one exact local module after globally suppressing bytecode writes."""
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError("Unavailable query-split source module")
    loaded = importlib.util.module_from_spec(spec)
    sys.modules[name] = loaded
    spec.loader.exec_module(loaded)
    return loaded


PURE = module("fraction_query_split_pure_helpers", ROOT / "tools/formal/verify_fraction_composition.py")
BASE = PURE.BASE
Incomplete = BASE.Incomplete
# Audited unchanged pure primitives. Never call PURE.main/execute_job/admission/
# remaining_budget/schedule, and never mutate the old module's globals.
qualify = PURE.qualify
validate_metadata = PURE.validate_metadata
stub_pairs = PURE.stub_pairs
token_path = PURE.token_path
compiler_receipt = PURE.compiler_receipt
binding = PURE.binding
validate_generated = PURE.validate_generated
admission_identity = PURE.admission_identity
GENERATED_NAMES = PURE.GENERATED_NAMES


def projection(source: Path = ROOT):
    """Load only the selected source island's new extraction/projection owner."""
    return module("fraction_query_split_projection_" + BASE.digest(str(source).encode())[:12],
                  source / SOURCE_NAMES[0])


def composition_flags(check) -> list[str]:
    """Preserve the exact ordinary CaDiCaL profile, adding only two admitted flags."""
    return [*BASE.solver_flags(check, "cadical"), "-Z", "stubbing", "--keep-temps"]


def input_names() -> list[str]:
    """Freeze the full ordinary/composition closure and all four new source files."""
    names = set(SOURCE_NAMES) | set(PURE.SOURCE_NAMES) | {
        "scripts/tests/test_check_analysis.py", "tools/formal/test_additional_suites.py"}
    for selector in ("fraction-constructor", "fraction-comparison", "fraction-arithmetic"):
        names.update(BASE.input_names(selector))
    names.update(projection().input_names())
    return sorted(names)


def job_folder(work: Path, index: int, job: dict) -> Path:
    """Derive paths only from the fixed index and exact closed job identity."""
    return work / "jobs" / f"{index:02d}-{job['id']}"


def schedule(source: Path = ROOT) -> list[tuple[int, dict]]:
    """Admit exactly the fresh thirteen-job prefix and both complete new lanes."""
    jobs = projection(source).schedule()
    if (not isinstance(jobs, list) or len(jobs) != MAXIMUM_PROOF_COMMANDS
            or any(not isinstance(job, dict) for job in jobs)
            or tuple(job.get("id") for job in jobs) != JOB_IDS):
        raise Incomplete("Query-split schedule differs from the fixed twenty jobs")
    for values, expected in ((jobs[:13], PREFIX_SHA256), (jobs[14:17], FAULTS_SHA256)):
        actual = BASE.digest(json.dumps(values, sort_keys=True, separators=(",", ":")).encode())
        if actual != expected:
            raise Incomplete("Original prefix or three source-fault job contracts changed")
    for healthy, restored, subject, role in (
            (13, 17, "fraction_addition_universal", "addition"),
            (18, 19, "fraction_addition_witnesses", "witness")):
        first, last = jobs[healthy], jobs[restored]
        if ({key: value for key, value in first.items() if key not in {"id", "restored_healthy"}}
                != {key: value for key, value in last.items() if key not in {"id", "restored_healthy"}}
                or first.get("restored_healthy") is not False
                or last.get("restored_healthy") is not True):
            raise Incomplete("Healthy/restored lane contracts differ")
        if (first.get("subject") != subject or first.get("harness") != "fraction_arithmetic::" + subject + "_full_domain"
                or first.get("case") != "healthy" or first.get("intended_failure") is not None
                or first.get("role") != role or first.get("expected_kind") != "Proof"):
            raise Incomplete("Query-split lane identity or complete-domain contract changed")
        pairs = stub_pairs(first.get("expected_stubs"))
        if stub_pairs(first.get("stub_pairs")) != pairs:
            raise Incomplete("Lane mapping representations disagree")
        if role == "witness":
            if (pairs or first.get("dependencies") != [] or first.get("extra_assertions") != []
                    or first.get("required_refinement_assertions") != []):
                raise Incomplete("Direct-source witnesses forbid stubs and model premises")
        elif (first.get("dependencies") != list(CALLEES)
              or pairs != stub_pairs(jobs[14]["expected_stubs"])
              or first.get("required_refinement_assertions") != MODEL_PRECONDITIONS
              or first.get("extra_assertions") != MODEL_PRECONDITIONS):
            raise Incomplete("Universal lane requires all three exact certified models")
    return list(enumerate(jobs, 1))


def _tree_entries(work: Path, roots: list[Path]) -> list[tuple[Path, os.stat_result]]:
    """Enumerate owned canonical regular trees without following links or aliases."""
    result, seen = [], set()

    def inspect(path):
        details = path.lstat()
        if (path != path.resolve(strict=True) or details.st_uid != os.geteuid()
                or not (stat.S_ISREG(details.st_mode) or stat.S_ISDIR(details.st_mode))
                or (stat.S_ISREG(details.st_mode) and details.st_nlink != 1)
                or (details.st_dev, details.st_ino) in seen):
            raise Incomplete("Frozen input owner/type/link/canonical-path or alias violation")
        seen.add((details.st_dev, details.st_ino))
        result.append((path, details))
        if len(result) > 100000:
            raise Incomplete("Frozen permission inventory exceeded its entry budget")
        if stat.S_ISDIR(details.st_mode):
            with os.scandir(path) as entries:
                for entry in sorted(entries, key=lambda item: item.name):
                    inspect(Path(entry.path))

    work_info = work.lstat()
    if (not work.is_absolute() or work != work.resolve(strict=True)
            or not stat.S_ISDIR(work_info.st_mode) or work_info.st_uid != os.geteuid()
            or stat.S_IMODE(work_info.st_mode) != 0o700):
        raise Incomplete("Permission inventory needs canonical owned work")
    for root in roots:
        if (not root.is_absolute() or root == work or not root.is_relative_to(work)
                or root.parent != root.parent.resolve(strict=True)):
            raise Incomplete("Permission roots must be canonical descendants of work")
        inspect(root)
    return result


def harden_inputs(work: Path, root: Path) -> dict:
    """Make a newly completed input tree read-only before its first custody seal."""
    entries = _tree_entries(work, [root])
    for path, details in reversed(entries):
        path.chmod(0o500 if stat.S_ISDIR(details.st_mode) else 0o400)
    return permission_map(work, [root])


def permission_map(work: Path, roots: list[Path]) -> dict:
    """Require exact owner0400/0500 modes and retain identity/link metadata."""
    result = {}
    for path, details in _tree_entries(work, roots):
        directory = stat.S_ISDIR(details.st_mode)
        mode = stat.S_IMODE(details.st_mode)
        if mode != (0o500 if directory else 0o400):
            raise Incomplete("Frozen input permission drift: " + str(path))
        result[path.relative_to(work).as_posix()] = {
            "uid": details.st_uid, "mode": mode,
            "kind": "directory" if directory else "file", "nlink": details.st_nlink,
            "device": details.st_dev, "inode": details.st_ino}
    return result


def audit_retained(check, work: Path, retained: list[dict]) -> None:
    """Validate content and exact permissions for every earlier sealed input tree."""
    for item in retained:
        roots = item.get("permission_roots", [])
        if roots and permission_map(work, roots) != item.get("permissions"):
            raise Incomplete("An earlier frozen input permission/identity map changed")
    BASE.audit_retained(check, work, retained)


def retain_artifacts(check, work: Path, retained: list[dict], roots: list[Path],
                     *, permission_roots: list[Path] | None = None) -> None:
    """Anchor immutable bytes plus local permission facts without rebasing history."""
    audit_retained(check, work, retained)
    permissions = permission_map(work, permission_roots or [])
    BASE.retain_artifacts(check, work, retained, roots)
    retained[-1].update(permission_roots=list(permission_roots or []), permissions=permissions)


def require_sources(check, source: Path, records: dict) -> None:
    """Bind every original/frozen byte and reject extra or writable frozen entries."""
    permission_map(source.parent, [source])
    if sorted(records) != input_names():
        raise Incomplete("Query-split source closure changed")
    frozen_names = set()
    for path in source.rglob("*"):
        if path.is_file():
            frozen_names.add(path.relative_to(source).as_posix())
    if frozen_names != set(records):
        raise Incomplete("Unexpected or missing frozen source input")
    for name, expected in records.items():
        for path in (ROOT / name, source / name):
            if path != path.resolve(strict=True):
                raise Incomplete("Original/frozen source redirects")
            data = check.read_once(path)
            if {"sha256": BASE.digest(data), "bytes": len(data)} != expected:
                raise Incomplete("Original/frozen query-split source drift: " + name)


def receipt_roots(folder: Path) -> list[Path]:
    """Retain raw compiler/results and both exact input permission bookends."""
    roots = PURE.receipt_roots(folder)
    for name in ("permissions-before.json", "permissions-after.json"):
        path = folder / name
        if not path.is_file() or path.is_symlink():
            raise Incomplete("Qualified proof is missing a permission receipt")
        roots.append(path)
    return roots


def restored_identity(check, work: Path, inputs: Path) -> dict:
    """Compare complete bytes and relative permission structure across lane roots."""
    permissions = permission_map(work, [inputs])
    relative = {}
    for name, facts in permissions.items():
        path = (work / name).relative_to(inputs).as_posix()
        relative[path] = {key: value for key, value in facts.items() if key not in {"device", "inode"}}
    return {"contents": BASE.generated_identity(check, inputs), "permissions": relative}


def require_dispatch_order(index: int, command_history: list[int]) -> None:
    """Bound actual native requests to the exact fresh twenty-job order, no retry."""
    if (type(index) is not int or not 1 <= index <= MAXIMUM_PROOF_COMMANDS
            or command_history != list(range(1, index))
            or any(type(value) is not int for value in command_history)):
        raise Incomplete("Native request is early, repeated or outside the fixed twenty-command card")


def validate_preparation(job: dict, prepared: dict) -> None:
    """Keep selected harness/mapping premises explicit, including empty witnesses."""
    if (not isinstance(prepared, dict) or prepared.get("job") != job
            or any(prepared.get(key) != job[key] for key in ("subject", "case", "harness"))
            or stub_pairs(prepared.get("expected_stubs")) != stub_pairs(job["expected_stubs"])
            or stub_pairs(prepared.get("stub_pairs")) != stub_pairs(job["expected_stubs"])
            or prepared.get("required_refinement_assertions") != job["required_refinement_assertions"]):
        raise Incomplete("Prepared selected job, mapping or refinement contract changed")
    if job["role"] == "witness" and prepared.get("assumptions") != []:
        raise Incomplete("Direct-source witnesses cannot carry selected assumptions")


def remaining_budget() -> float:
    """Keep the original boot/wall/monotonic preparation clock without a reset."""
    if Path("/proc/sys/kernel/random/boot_id").read_text().strip() != PREPARATION_BOOT_ID:
        raise Incomplete("Query-split preparation belongs to a different boot")
    elapsed = (time.monotonic_ns() - PREPARATION_STARTED_MONOTONIC_NS) / 1e9
    wall = (time.time_ns() - PREPARATION_STARTED_UNIX_NS) / 1e9
    remaining = TOTAL_SECONDS - max(elapsed, wall)
    if (elapsed < 0 or wall < 0 or not math.isfinite(remaining) or remaining <= 0
            or time.monotonic_ns() >= DEADLINE_MONOTONIC_NS):
        raise Incomplete("Query-split preparation/execution budget expired or clock moved backwards")
    return remaining


def output_preflight(value: str) -> Path:
    """Allow one fixed fresh output, with no alternate destination or relaunch."""
    expected = Path.home() / ".local/share/nevrotick" / OUTPUT_LEAF
    if value != str(expected) or expected.exists() or expected.is_symlink():
        raise Incomplete("Query-split requires its one fixed fresh output directory")
    remaining_budget()
    return expected


def requalify_callee(check, work: Path, source: Path, key: str) -> tuple[dict, dict]:
    """Derive the exact healthy job and re-evaluate retained raw proof evidence."""
    if key not in CALLEES:
        raise Incomplete("Unknown callee certificate owner")
    index, job = next((i, j) for i, j in schedule(source) if j["id"] == key)
    folder = job_folder(work, index, job)
    record = check.decode(check.read_once(folder / "execution.json"))
    if (not isinstance(record, dict) or record.get("job") != job or record.get("index") != index
            or record.get("consumer") != CONSUMER or record.get("expected_outcome_qualified") is not True
            or record.get("input_unchanged") is not True):
        raise Incomplete("Retained callee execution differs from its exact healthy job")
    before = check.decode(check.read_once(folder / "input-before.json"))
    after = check.decode(check.read_once(folder / "input-after.json"))
    if before != after or before != check.input_snapshot(work, [folder / "inputs"]) or record.get("input_before") != before or record.get("input_after") != after:
        raise Incomplete("Retained callee input bookends do not match current inputs")
    permissions_before = check.decode(check.read_once(folder / "permissions-before.json"))
    permissions_after = check.decode(check.read_once(folder / "permissions-after.json"))
    if (permissions_before != permissions_after
            or permissions_before != permission_map(work, [folder / "inputs"])
            or record.get("permissions_before") != permissions_before
            or record.get("permissions_after") != permissions_after
            or record.get("permissions_unchanged") is not True):
        raise Incomplete("Retained callee permission bookends do not match current inputs")
    validate_generated(check, folder / "inputs", record.get("preparation", {}))
    admitted = check.decode(check.read_once(folder / "admission.json"))
    if admission_identity(admitted) != record.get("tool_identity"):
        raise Incomplete("Retained callee admission does not bind its actual tool identity")
    report = check.decode(check.read_once(folder / "kani.json"))
    metadata = check.decode(check.read_once(folder / "compiler-metadata.json"))
    entrypoint = "/work/" + (folder / "inputs/proof.rs").relative_to(work).as_posix()
    observed = qualify(report, metadata, record.get("native"), job, entrypoint)
    binding(record.get("preparation", {}), key)
    return record, observed


def issue_certificate(check, work: Path, source: Path, key: str, identity: dict) -> dict:
    """Only freshly requalified actual-body Post/seed proofs issue a certificate."""
    record, _ = requalify_callee(check, work, source, key)
    if record.get("tool_identity") != identity:
        raise Incomplete("Callee tool identity differs from the campaign")
    folder = job_folder(work, record["index"], record["job"])
    roots = receipt_roots(folder)
    return {"schema_version": 1, "consumer": CONSUMER, "key": key,
            "binding": binding(record["preparation"], key), "tool_identity": identity,
            "receipt_snapshot": check.input_snapshot(work, roots),
            "input_permissions": permission_map(work, [folder / "inputs"])}


def require_callees(check, work: Path, source: Path, job: dict, prepared: dict,
                    certificates: dict, identity: dict) -> None:
    """Refuse incomplete, negative, stale or forged summarized evidence before dispatch."""
    if job["role"] not in {"precondition", "addition"}:
        return
    if job["dependencies"] != list(CALLEES) or set(certificates) != set(CALLEES):
        raise Incomplete("Caller requires exactly all three fresh callee certificates")
    for key in CALLEES:
        certificate = certificates[key]
        if (not isinstance(certificate, dict) or set(certificate) != {"schema_version", "consumer", "key", "binding", "tool_identity", "receipt_snapshot", "input_permissions"}
                or type(certificate.get("schema_version")) is not int or certificate["schema_version"] != 1
                or certificate["consumer"] != CONSUMER or certificate["key"] != key
                or certificate["tool_identity"] != identity or certificate["binding"] != binding(prepared, key)):
            raise Incomplete("Caller callee certificate/schema/closure changed: " + key)
        index, callee_job = next((i, j) for i, j in schedule(source) if j["id"] == key)
        roots = receipt_roots(job_folder(work, index, callee_job))
        if certificate["receipt_snapshot"] != check.input_snapshot(work, roots):
            raise Incomplete("Callee raw receipts or proof inputs changed: " + key)
        if certificate["input_permissions"] != permission_map(work, [job_folder(work, index, callee_job) / "inputs"]):
            raise Incomplete("Callee input permissions or identity changed: " + key)
        record, _ = requalify_callee(check, work, source, key)
        if record.get("tool_identity") != identity or binding(record["preparation"], key) != certificate["binding"]:
            raise Incomplete("Requalified callee does not bind this exact caller closure")


def admission(check, output: Path) -> tuple[dict, dict]:
    """Admit the unchanged two-root Kani closure under this distinct consumer."""
    remaining_budget()
    pointer = Path.home() / ".local/share/nevrotick/analysis-installation.json"
    before = check.read_once(pointer, maximum=4096, private=True)
    started = time.monotonic()
    flags = composition_flags(check)
    admitted = check.admit_fraction_query_split_pilot(ROOT)
    if admitted.get("admission_consumer") != CONSUMER:
        raise Incomplete("Wrong query-split admission consumer")
    readonly = BASE.proof_readonly_roots(admitted, "kani-proof")
    if check.read_once(pointer, maximum=4096, private=True) != before:
        raise Incomplete("Installation pointer changed during query-split admission")
    kani = admitted["policy"]["tools"]["kani"]
    compiler = admitted["base"] / kani["toolchain"]
    record = {"consumer": CONSUMER, "seconds": time.monotonic() - started,
              "admitted": admitted, "readonly_roots": readonly,
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


def require_admission_identity(record: dict, anchor: dict) -> None:
    identity = admission_identity(record)
    if not anchor:
        anchor.update(identity)
    elif anchor != identity:
        raise Incomplete("Query-split campaign tool/pointer/profile identity changed")


def phase_bookend(check, work: Path, source: Path, records: dict, retained: list,
                  anchor: dict, index: int) -> None:
    """Re-admit source/tool/receipt custody at each completed pre-caller phase."""
    if index not in {2, 11, 13, 18, 20}:
        raise Incomplete("Unknown query-split phase bookend")
    audit_retained(check, work, retained)
    require_sources(check, source, records)
    path = work / f"phase-{index:02d}-admission.json"
    _, record = admission(check, path)
    require_admission_identity(record, anchor)
    require_sources(check, source, records)
    retain_artifacts(check, work, retained, [path])
    remaining_budget()


def execute_job(check, work: Path, source: Path, records: dict, job: dict, index: int,
                retained: list, certificates: dict, anchor: dict,
                command_history: list[int]) -> dict:
    """Run one fixed proof, sealing prior raw evidence and refusing early callers."""
    if (index, job) not in schedule(source):
        raise Incomplete("Query-split command is outside the fixed twenty jobs")
    require_dispatch_order(index, command_history)
    folder = job_folder(work, index, job)
    inputs = folder / "inputs"
    inputs.mkdir(mode=0o700, parents=True)
    started = time.monotonic()
    record = {"index": index, "job": job, "output": str(folder), "consumer": CONSUMER,
              "expected_outcome_qualified": False}
    try:
        remaining_budget()
        audit_retained(check, work, retained)
        require_sources(check, source, records)
        for name in records:
            target = inputs / "frozen" / name
            target.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
            target.write_bytes((source / name).read_bytes())
        prepared = projection(source).prepare(source, inputs, job)
        validate_preparation(job, prepared)
        entry = prepared["entrypoint"]
        if entry != inputs / "proof.rs" or entry != entry.resolve(strict=True) or not entry.is_file():
            raise Incomplete("Query-split projection must return canonical inputs/proof.rs")
        if stub_pairs(prepared.get("expected_stubs")) != stub_pairs(job["expected_stubs"]):
            raise Incomplete("Prepared mapping differs from the fixed job")
        permissions_before = harden_inputs(work, inputs)
        validate_generated(check, inputs, prepared)
        for key in CALLEES:
            binding(prepared, key)
        record.update(preparation=prepared, source_original_manifest=records)
        if index in {18, 20}:
            original_index = 14 if index == 18 else 19
            original = job_folder(work, original_index, schedule(source)[original_index - 1][1]) / "inputs"
            if restored_identity(check, work, inputs) != restored_identity(check, work, original):
                raise Incomplete("Restored complete generated/frozen lane inputs differ from healthy")
            record["restored_generated_input_matches_healthy"] = True
            record["restored_healthy_index"] = original_index
        before = check.input_snapshot(work, [inputs])
        record.update(input_before=before, permissions_before=permissions_before)
        BASE.write_json(folder / "input-before.json", before)
        BASE.write_json(folder / "permissions-before.json", permissions_before)
        audit_retained(check, work, retained)
        require_sources(check, source, records)
        admitted, admitted_record = admission(check, folder / "admission.json")
        require_admission_identity(admitted_record, anchor)
        record.update(admission_seconds=admitted_record["seconds"], tool_identity=dict(anchor))
        require_sources(check, source, records)
        if check.input_snapshot(work, [inputs]) != before:
            raise Incomplete("Prepared inputs changed during query-split admission")
        if permission_map(work, [inputs]) != permissions_before:
            raise Incomplete("Prepared input permissions changed during query-split admission")
        audit_retained(check, work, retained)
        require_callees(check, work, source, job, prepared, certificates, anchor)
        base = admitted["base"]
        kani = admitted["policy"]["tools"]["kani"]
        compiler = base / kani["toolchain"]
        namespace_entry = "/work/" + entry.relative_to(work).as_posix()
        command = [str(base / kani["entrypoint"]), namespace_entry, "--harness", job["harness"],
                   "--exact", *composition_flags(check), "--export-json",
                   "/work/" + (folder / "kani.json").relative_to(work).as_posix(),
                   "--target-dir", "/work/" + (folder / "target").relative_to(work).as_posix()]
        environment = {"RUSTC": str(compiler / "bin/rustc"), "RUSTDOC": str(compiler / "bin/rustdoc"),
                       "RUSTUP_HOME": str(base / "kani/rustup"), "RUSTUP_TOOLCHAIN": "nightly-2026-08-21",
                       "RUSTFLAGS": BASE.RUSTFLAGS}
        readonly = BASE.proof_readonly_roots(admitted, "kani-proof")
        paths = [base / "kani/payload/kani-0.68.0/bin", compiler / "bin"]
        record.update(command=command, explicit_environment=environment,
                      runtime_readonly_roots=readonly, runtime_path_dirs=paths)
        audit_retained(check, work, retained)
        require_sources(check, source, records)
        if permission_map(work, [inputs]) != permissions_before:
            raise Incomplete("Current input permission drift before dispatch")
        require_callees(check, work, source, job, prepared, certificates, anchor)
        current_roots = [inputs, *(folder / n for n in ("input-before.json", "permissions-before.json", "admission.json", "execution.json"))]
        if (work / "summary.json").exists():
            current_roots.append(work / "summary.json")
        sealed = [path for item in retained for path in item["roots"]] + current_roots
        record["runtime_sealed_roots"] = sealed
        BASE.write_json(folder / "execution.json", record)
        current = check.input_snapshot(work, current_roots)
        budget = remaining_budget()
        native_started = time.monotonic()
        require_dispatch_order(index, command_history)
        command_history.append(index)
        native = check.RUNTIME.run(ROOT, work, command, readonly=readonly, path_dirs=paths,
            cwd=inputs, environment=environment, timeout=min(COMMAND_SECONDS, budget),
            cpu_seconds=COMMAND_SECONDS, build_jobs=2, address_space_bytes=8 * 1024**3,
            process_limit=128, sealed=sealed)
        record.update(native=native, execute_seconds=time.monotonic() - native_started,
                      runtime_effective_wall_seconds=min(COMMAND_SECONDS, budget))
        check.RUNTIME._owned_work(work, ROOT)
        audit_retained(check, work, retained)
        require_sources(check, source, records)
        permissions_after = permission_map(work, [inputs])
        if permissions_before != permissions_after:
            raise Incomplete("Complete input permission map changed during execution")
        if check.input_snapshot(work, current_roots) != current:
            raise Incomplete("Sealed query-split inputs or receipts changed during execution")
        metadata = compiler_receipt(check, folder)
        after = check.input_snapshot(work, [inputs])
        record.update(input_after=after, input_unchanged=before == after,
                      permissions_after=permissions_after, permissions_unchanged=True)
        BASE.write_json(folder / "input-after.json", after)
        BASE.write_json(folder / "permissions-after.json", permissions_after)
        require_sources(check, source, records)
        if before != after:
            raise Incomplete("Complete query-split input tree changed")
        report = check.decode(check.read_once(folder / "kani.json"))
        record.update(qualify(report, metadata, native, job, namespace_entry))
        require_callees(check, work, source, job, prepared, certificates, anchor)
        require_sources(check, source, records)
        if check.input_snapshot(work, [inputs]) != before:
            raise Incomplete("Query-split inputs changed during qualification")
        if permission_map(work, [inputs]) != permissions_before:
            raise Incomplete("Query-split input permissions changed during qualification")
        remaining_budget()
        record["expected_outcome_qualified"] = True
    except Exception as error:
        record["qualification_error"] = f"{type(error).__name__}: {error}"
    finally:
        record["job_total_seconds"] = time.monotonic() - started
        try:
            BASE.write_json(folder / "execution.json", record)
            roots = receipt_roots(folder) if record["expected_outcome_qualified"] else [inputs, *BASE.job_receipts(folder, "cadical", required=False)]
            if not record["expected_outcome_qualified"] and (folder / "compiler-metadata.json").exists():
                roots.append(folder / "compiler-metadata.json")
            if not record["expected_outcome_qualified"]:
                roots.extend(folder / name for name in ("permissions-before.json", "permissions-after.json")
                             if (folder / name).exists())
            retain_artifacts(check, work, retained, roots, permission_roots=[inputs])
        except Exception as error:
            record.update(expected_outcome_qualified=False, persistence_error=f"{type(error).__name__}: {error}")
            record.setdefault("qualification_error", record["persistence_error"])
            BASE.write_json(folder / "execution.json", record)
    return record


def main(argv=None) -> int:
    """Run the fixed fresh card once; first incomplete job ends its campaign."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", required=True)
    args = parser.parse_args(argv)
    output_preflight(args.output_dir)
    os.umask(0o077)
    work = BASE.fresh_output(args.output_dir)
    source = work / "source"
    check = module("fraction_query_split_admission", ROOT / "scripts/check-analysis.py")
    summary = {"status": "incomplete", "decision": "DEFER", "consumer": CONSUMER,
               "jobs": [], "certificates": {}, "maximum_proof_commands": MAXIMUM_PROOF_COMMANDS, "retries": 0,
               "preparation_started_unix_ns": PREPARATION_STARTED_UNIX_NS,
               "preparation_started_monotonic_ns": PREPARATION_STARTED_MONOTONIC_NS,
               "preparation_boot_id": PREPARATION_BOOT_ID,
               "deadline_monotonic_ns": DEADLINE_MONOTONIC_NS,
               "total_preparation_execution_seconds": TOTAL_SECONDS,
               "scope": "both pure fraction-addition lanes only; wider Core/P1/Windows/package gates remain open"}
    retained, anchor, certificates = [], {}, {}
    command_history = []
    records = None
    started = time.monotonic()
    try:
        remaining_budget()
        records = BASE.freeze_inputs(check, source, input_names())
        source_permissions = harden_inputs(work, source)
        require_sources(check, source, records)
        BASE.write_json(work / "source-manifest.json", {
            "source_original_manifest": records, "source_permissions": source_permissions})
        retain_artifacts(check, work, retained, [source, work / "source-manifest.json"],
                         permission_roots=[source])
        jobs = schedule(source)
        summary["planned_jobs"] = [j for _, j in jobs]
        BASE.write_json(work / "summary.json", summary)
        for index, job in jobs:
            remaining_budget()
            record = execute_job(check, work, source, records, job, index, retained,
                                 certificates, anchor, command_history)
            compact = {key: record.get(key) for key in ("index", "output", "checks", "status_counts", "admission_seconds", "execute_seconds", "qualification_error")}
            compact.update(id=job["id"], qualified=record["expected_outcome_qualified"], exit_code=record.get("native", {}).get("exit_code"))
            summary["jobs"].append(compact)
            BASE.write_json(work / "summary.json", summary)
            print(json.dumps(compact), flush=True)
            if not record["expected_outcome_qualified"]:
                raise Incomplete("Stopped at first unqualified query-split job: " + job["id"])
            if job["id"] in CALLEES:
                certificate = issue_certificate(check, work, source, job["id"], anchor)
                certificates[job["id"]] = certificate
                path = Path(record["output"]) / "certificate.json"
                BASE.write_json(path, certificate)
                retain_artifacts(check, work, retained, [path])
                summary["certificates"][job["id"]] = certificate
            if index in {2, 11, 13, 18, 20}:
                phase_bookend(check, work, source, records, retained, anchor, index)
        remaining_budget()
        if command_history != list(range(1, MAXIMUM_PROOF_COMMANDS + 1)):
            raise Incomplete("Both complete lanes and the exact twenty native requests are required")
        summary.update(status="passed", decision="AWAIT_INDEPENDENT_REVIEW")
    except Exception as error:
        summary.update(status="incomplete", decision="DEFER", error=f"{type(error).__name__}: {error}")
    finally:
        try:
            audit_retained(check, work, retained)
            if records is None:
                raise Incomplete("No complete query-split source closure")
            require_sources(check, source, records)
            _, final = admission(check, work / "final-admission.json")
            require_admission_identity(final, anchor)
            require_sources(check, source, records)
            retain_artifacts(check, work, retained, [work / "final-admission.json"])
            remaining_budget()
            summary.update(final_admission_seconds=final["seconds"], original_and_frozen_inputs_unchanged=True)
            summary["all_proof_inputs_and_required_receipts_unchanged" if summary["status"] == "passed" else "retained_partial_inputs_and_receipts_unchanged"] = True
        except Exception as error:
            summary.update(status="incomplete", decision="DEFER", final_error=f"{type(error).__name__}: {error}")
        summary["campaign_seconds"] = time.monotonic() - started
        summary["native_proof_requests"] = len(command_history)
        summary["native_proof_request_order"] = command_history
        summary["preparation_to_terminal_seconds"] = (time.monotonic_ns() - PREPARATION_STARTED_MONOTONIC_NS) / 1e9
        BASE.write_json(work / "summary.json", summary)
    print(json.dumps({"status": summary["status"], "completed_jobs": len(summary["jobs"]), "summary": str(work / "summary.json"), "error": summary.get("error"), "final_error": summary.get("final_error")}), flush=True)
    return 0 if summary["status"] == "passed" else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (Incomplete, OSError, ValueError) as error:
        print(json.dumps({"status": "incomplete", "error": f"{type(error).__name__}: {error}"}), flush=True)
        raise SystemExit(1)
