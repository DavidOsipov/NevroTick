#!/usr/bin/env python3
"""One closed ordinary-stub composition card; install or patch nothing.

The source-bound pure-call inclusion rule is an explicit trusted boundary.
Ordinary stubs supply mechanics, not certification. Exported Kani JSON omits
stub pairs: retain the compiler metadata with --keep-temps and authenticate
its token pairs alongside native stdout and the sealed generated source.
"""
from __future__ import annotations

import argparse
import importlib.util
import json
import math
import os
from pathlib import Path
import re
import sys
import time


ROOT = Path(__file__).resolve().parents[2]
CONSUMER = "fraction-composition-pilot-v1"
OUTPUT_LEAF = "formal-core-fraction-composition-20261106-v1"
PREPARATION_STARTED_UNIX_NS = 1793918875353739445
PREPARATION_STARTED_MONOTONIC_NS = 15543187166373
PREPARATION_BOOT_ID = "d3fc5615-e867-45cd-b74f-4a7413afc5ab"
DEADLINE_MONOTONIC_NS = 19143187166373
TOTAL_SECONDS = 3600
COMMAND_SECONDS = 180
CALLEES = ("constructor", "comparison", "complement")
JOB_IDS = ("mech_healthy", "mech_assert_then_assume", "constructor_bad_post",
           "constructor_wrong_seed", "constructor", "comparison_bad_post",
           "comparison_wrong_seed", "comparison", "complement_bad_post",
           "complement_wrong_seed", "complement", "comparison_bad_precondition",
           "complement_bad_precondition", "addition_healthy", "addition_reject_all",
           "addition_wrong_carry", "addition_wrong_residual", "addition_restored")
SOURCE_NAMES = ("tools/formal/fraction_composition_projection.py",
                "tools/formal/fraction_composition_spec.rs",
                "tools/formal/fraction_composition_harness.rs",
                "tools/formal/test_fraction_composition.py",
                "tools/formal/verify_fraction_composition.py",
                "tools/formal/test_verify_fraction_composition.py")
GENERATED_NAMES = ("proof.rs", "fraction_composition_selected.rs",
                   "fraction_composition_spec.rs", "correspondence.json")


def module(name: str, path: Path):
    """Load only a named current/frozen local file, never a search-path fallback."""
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise RuntimeError("Unavailable composition source module")
    loaded = importlib.util.module_from_spec(spec)
    sys.modules[name] = loaded
    spec.loader.exec_module(loaded)
    return loaded


BASE = module("fraction_composition_ordinary", ROOT / "tools/formal/verify.py")
Incomplete = BASE.Incomplete


def projection(source: Path = ROOT):
    """Load the selected projection from the same source island being checked."""
    return module("fraction_composition_projection_" + BASE.digest(str(source).encode())[:12],
                  source / SOURCE_NAMES[0])


def remaining_budget() -> float:
    """Keep the original boot/wall/monotonic preparation clock without a reset."""
    if Path("/proc/sys/kernel/random/boot_id").read_text().strip() != PREPARATION_BOOT_ID:
        raise Incomplete("Composition preparation belongs to a different boot")
    elapsed = (time.monotonic_ns() - PREPARATION_STARTED_MONOTONIC_NS) / 1e9
    wall = (time.time_ns() - PREPARATION_STARTED_UNIX_NS) / 1e9
    remaining = TOTAL_SECONDS - max(elapsed, wall)
    if (elapsed < 0 or wall < 0 or not math.isfinite(remaining) or remaining <= 0
            or time.monotonic_ns() >= DEADLINE_MONOTONIC_NS):
        raise Incomplete("Composition preparation/execution budget expired or clock moved backwards")
    return remaining


def composition_flags(check) -> list[str]:
    """Add only reviewed ordinary-stub mechanics and metadata retention flags."""
    return [*BASE.solver_flags(check, "cadical"), "-Z", "stubbing", "--keep-temps"]


def output_preflight(value: str) -> Path:
    """Allow one fixed fresh output, with no alternate destination or relaunch."""
    expected = Path.home() / ".local/share/nevrotick" / OUTPUT_LEAF
    if value != str(expected) or expected.exists() or expected.is_symlink():
        raise Incomplete("Composition requires its one fixed fresh output directory")
    remaining_budget()
    return expected


def input_names() -> list[str]:
    """Freeze the ordinary callee/caller closures and this complete consumer delta."""
    names = set(SOURCE_NAMES) | {"scripts/tests/test_check_analysis.py",
                                "tools/formal/test_additional_suites.py"}
    for selector in ("fraction-constructor", "fraction-comparison", "fraction-arithmetic"):
        names.update(BASE.input_names(selector))
    names.update(projection().input_names())
    return sorted(names)


def require_sources(check, source: Path, records: dict) -> None:
    """Reject changed original/frozen bytes and added or missing source inputs."""
    if sorted(records) != input_names():
        raise Incomplete("Composition input closure changed")
    for name, expected in records.items():
        for path in (ROOT / name, source / name):
            if path != path.resolve(strict=True):
                raise Incomplete("Composition source path redirects")
            data = check.read_once(path)
            if {"sha256": BASE.digest(data), "bytes": len(data)} != expected:
                raise Incomplete("Original/frozen composition source drift: " + name)


def token_path(value: str) -> str:
    """Normalize separator whitespace only; reject split identifiers and aliases."""
    identifier = r"[A-Za-z_][A-Za-z_0-9]*"
    if not isinstance(value, str) or not re.fullmatch(
            rf"[ \t]*{identifier}(?:[ \t]*::[ \t]*{identifier})*[ \t]*", value):
        raise Incomplete("Stub paths must use the closed ASCII identifier/path grammar")
    return "::".join(part.strip(" \t") for part in value.split("::"))


def stub_pairs(value) -> tuple[tuple[str, str], ...]:
    """Require exact typed pair objects, unique originals and no duplicate pairs."""
    if not isinstance(value, (list, tuple)):
        raise Incomplete("Stub pairs must be an array")
    pairs = []
    for item in value:
        if not isinstance(item, dict) or set(item) != {"original", "replacement"}:
            raise Incomplete("Stub pair has missing or extra fields")
        pairs.append((token_path(item["original"]), token_path(item["replacement"])))
    if len(set(pairs)) != len(pairs) or len({a for a, _ in pairs}) != len(pairs):
        raise Incomplete("Duplicate or conflicting stub pairs")
    return tuple(sorted(pairs))


def schedule(source: Path = ROOT) -> list[tuple[int, dict]]:
    """Require the one eighteen-command order and unchanged original healthy/fault jobs."""
    jobs = projection(source).schedule()
    if (not isinstance(jobs, list) or len(jobs) != len(JOB_IDS)
            or any(not isinstance(j, dict) for j in jobs)
            or tuple(j.get("id") for j in jobs) != JOB_IDS):
        raise Incomplete("Composition schedule differs from the fixed eighteen jobs")
    for index, job in enumerate(jobs, 1):
        role = "mechanism" if index <= 2 else "callee" if index <= 11 else "precondition" if index <= 13 else "addition"
        if job.get("role") != role or type(job.get("restored_healthy")) is not bool:
            raise Incomplete("Composition job role or restoration marker changed")
        expected_dependencies = list(CALLEES) if role in {"precondition", "addition"} else []
        if job.get("dependencies") != expected_dependencies:
            raise Incomplete("Composition dependencies differ from the closed graph")
        pairs = stub_pairs(job.get("expected_stubs"))
        if role == "callee" and pairs:
            raise Incomplete("Actual-body callee refinements cannot use stubs")
        labels = job.get("required_refinement_assertions")
        if (not isinstance(labels, (list, tuple)) or any(not isinstance(x, str) or not x for x in labels)
                or len(set(labels)) != len(labels)):
            raise Incomplete("Malformed refinement assertion inventory")
        if job["id"] in CALLEES and (job.get("case") != "healthy" or len(labels) < 2):
            raise Incomplete("Callee certification requires Post and actual-return seed obligations")
    fields = ("subject", "case", "harness", "intended_failure", "restored_healthy")
    for position, selector in ((5, "fraction-constructor"), (8, "fraction-comparison"), (11, "fraction-arithmetic")):
        original = BASE.additional_suites(source).jobs(selector)[0]
        if any(jobs[position - 1].get(k) != original[k] for k in fields):
            raise Incomplete("Original callee healthy contract or complete domain changed")
    original_addition = BASE.additional_suites(source).jobs("fraction-arithmetic")[5:]
    if len(original_addition) != 5 or any(
            any(actual.get(k) != expected[k] for k in fields)
            for actual, expected in zip(jobs[13:], original_addition, strict=True)):
        raise Incomplete("Original addition healthy/three-defect/restored group changed")
    return list(enumerate(jobs, 1))


def validate_metadata(report: dict, compiler: dict, native: dict, job: dict, entrypoint: str) -> None:
    """Cross-check export, compiler token metadata, source selection and native stdout."""
    expected = stub_pairs(job["expected_stubs"])
    harnesses = report.get("harness_metadata") if isinstance(report, dict) else None
    if not isinstance(harnesses, list) or len(harnesses) != 1 or not isinstance(harnesses[0], dict):
        raise Incomplete("Export must contain exactly one selected harness")
    exported = harnesses[0]
    if (exported.get("pretty_name") != job["harness"]
            or exported.get("contract") != {"contracted_function_name": None, "recursion_tracker": None}
            or not isinstance(exported.get("attributes"), dict)
            or exported["attributes"] != {"kind": "Proof", "should_panic": False}
            or any(exported.get(k) is not False for k in ("has_loop_contracts", "is_automatically_generated", "is_bounded", "is_ctor_based"))):
        raise Incomplete("Exported harness must be an ordinary unbounded proof without contracts")
    if (not isinstance(compiler, dict) or compiler.get("crate_name") != "proof"
            or compiler.get("unsupported_features") != [] or compiler.get("test_harnesses") != []
            or compiler.get("contracted_functions") != []):
        raise Incomplete("Compiler metadata has unsupported features, tests or contracts")
    harnesses = compiler.get("proof_harnesses")
    if not isinstance(harnesses, list) or any(not isinstance(h, dict) for h in harnesses):
        raise Incomplete("Compiler proof harness metadata must be an array of objects")
    matches = [h for h in harnesses if h.get("pretty_name") == job["harness"]]
    if len(matches) != 1:
        raise Incomplete("Compiler metadata must contain exactly one selected proof harness")
    selected = matches[0]
    attributes = selected.get("attributes")
    if (selected.get("pretty_name") != job["harness"] or selected.get("original_file") != entrypoint
            or selected.get("crate_name") != "proof" or selected.get("contract") is not None
            or any(selected.get(k) is not False for k in ("has_loop_contracts", "is_automatically_generated", "is_bounded", "is_ctor_based"))
            or not isinstance(attributes, dict)
            or set(attributes) != {"kind", "should_panic", "solver", "unwind_value", "stubs", "verified_stubs"}
            or attributes.get("kind") != "Proof"
            or attributes.get("should_panic") is not False or attributes.get("verified_stubs") != []
            or attributes.get("solver") is not None or attributes.get("unwind_value") is not None
            or stub_pairs(attributes.get("stubs")) != expected):
        raise Incomplete("Compiler harness, ordinary attributes or exact stub token pairs changed")
    if (not isinstance(native, dict) or "failure" not in native or native["failure"] is not None
            or type(native.get("exit_code")) is not int or native["exit_code"] < 0
            or not isinstance(native.get("output"), str)):
        raise Incomplete("Missing real native completion/output")
    output = native["output"]
    if "Thread " in output or [line for line in output.splitlines() if line.startswith("Checking harness ")] != ["Checking harness " + job["harness"] + "..."]:
        raise Incomplete("Native output selected a different or multiple harnesses/workers")
    actual = []
    for line in output.splitlines():
        line = line.strip()
        if line.startswith("- Verified stub:"):
            raise Incomplete("Verified/function-contract stubs are forbidden")
        if line.startswith("- Stub:"):
            pair = line.removeprefix("- Stub:").strip().split(" -> ")
            if len(pair) != 2:
                raise Incomplete("Malformed native stub pair")
            actual.append({"original": pair[0], "replacement": pair[1]})
    if stub_pairs(actual) != expected:
        raise Incomplete("Native stub token pairs differ from the closed source mapping")


def qualify(report: dict, compiler: dict, native: dict, job: dict, entrypoint: str) -> dict:
    """Keep ordinary qualification and require every extra refinement fact to survive."""
    validate_metadata(report, compiler, native, job, entrypoint)
    result = BASE.qualify(report, native, job, entrypoint)
    checks = report["verification_results"]["results"][0]["checks"]
    for label in job["required_refinement_assertions"]:
        matches = [x for x in checks if x["category"] == "assertion" and x["description"].strip('"') == label]
        if not matches or any(x["status"] != "Success" or x.get("location", {}).get("file") != entrypoint for x in matches):
            raise Incomplete("Missing, unreachable or unproved refinement assertion: " + label)
    return result


def job_folder(work: Path, index: int, job: dict) -> Path:
    """Derive receipts from the fixed schedule; never trust receipt-supplied paths."""
    return work / "jobs" / f"{index:02d}-{job['id']}"


def compiler_receipt(check, folder: Path) -> dict:
    """Read the exact standalone compiler metadata and retain its original bytes."""
    original = folder / "target/proof.kani-metadata.json"
    if original != original.resolve(strict=True):
        raise Incomplete("Compiler metadata path redirects")
    data = check.read_once(original)
    destination = folder / "compiler-metadata.json"
    if destination.exists() or destination.is_symlink():
        raise Incomplete("Compiler metadata receipt already exists")
    descriptor = os.open(destination, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_CLOEXEC, 0o600)
    with os.fdopen(descriptor, "wb") as stream:
        stream.write(data)
    return check.decode(data)


def receipt_roots(folder: Path) -> list[Path]:
    """Include compiler JSON alongside all ordinary raw-result/custody receipts."""
    return [folder / "inputs", *BASE.job_receipts(folder, "cadical", required=True), folder / "compiler-metadata.json"]


def binding(prepared: dict, key: str) -> str:
    """Require all three complete source/spec/codec/mapping closures, not a bare name."""
    values = prepared.get("callee_bindings")
    if not isinstance(values, dict) or set(values) != set(CALLEES) or any(
            not isinstance(v, str) or not re.fullmatch(r"[0-9a-f]{64}", v) for v in values.values()):
        raise Incomplete("Missing or malformed complete callee binding closure")
    return values[key]


def validate_generated(check, inputs: Path, prepared: dict) -> None:
    """Bind the exact four generated leaves, including correspondence, to prepared bytes."""
    if (prepared.get("generated_input_files") != list(GENERATED_NAMES)
            or not isinstance(prepared.get("input_files_sha256"), dict)
            or set(prepared["input_files_sha256"]) != set(GENERATED_NAMES)):
        raise Incomplete("Generated composition input inventory differs from the closed four leaves")
    current = BASE.generated_identity(check, inputs)
    if {name for name in current if not name.startswith("frozen/")} != set(GENERATED_NAMES):
        raise Incomplete("Unexpected or missing generated composition inputs")
    for name in GENERATED_NAMES:
        if current[name]["sha256"] != prepared["input_files_sha256"][name]:
            raise Incomplete("Generated composition manifest differs from actual bytes: " + name)


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
            "receipt_snapshot": check.input_snapshot(work, roots)}


def require_callees(check, work: Path, source: Path, job: dict, prepared: dict,
                    certificates: dict, identity: dict) -> None:
    """Refuse incomplete, negative, stale or forged summarized evidence before dispatch."""
    if job["role"] not in {"precondition", "addition"}:
        return
    if job["dependencies"] != list(CALLEES) or set(certificates) != set(CALLEES):
        raise Incomplete("Caller requires exactly all three fresh callee certificates")
    for key in CALLEES:
        certificate = certificates[key]
        if (not isinstance(certificate, dict) or set(certificate) != {"schema_version", "consumer", "key", "binding", "tool_identity", "receipt_snapshot"}
                or type(certificate.get("schema_version")) is not int or certificate["schema_version"] != 1
                or certificate["consumer"] != CONSUMER or certificate["key"] != key
                or certificate["tool_identity"] != identity or certificate["binding"] != binding(prepared, key)):
            raise Incomplete("Caller callee certificate/schema/closure changed: " + key)
        index, callee_job = next((i, j) for i, j in schedule(source) if j["id"] == key)
        roots = receipt_roots(job_folder(work, index, callee_job))
        if certificate["receipt_snapshot"] != check.input_snapshot(work, roots):
            raise Incomplete("Callee raw receipts or proof inputs changed: " + key)
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
    admitted = check.admit_fraction_composition_pilot(ROOT)
    if admitted.get("admission_consumer") != CONSUMER:
        raise Incomplete("Wrong composition admission consumer")
    readonly = BASE.proof_readonly_roots(admitted, "kani-proof")
    if check.read_once(pointer, maximum=4096, private=True) != before:
        raise Incomplete("Installation pointer changed during composition admission")
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


def admission_identity(record: dict) -> dict:
    """Bind all commands and bookends to one tool/pointer/profile identity."""
    admitted = record["admitted"]
    return BASE.serializable({**{key: record[key] for key in (
        "consumer", "readonly_roots", "installation_pointer_sha256", "kani_executable_sha256",
        "compiler_rustc_sha256", "compiler_rustdoc_sha256", "profile")},
        **{key: admitted[key] for key in ("base", "stable", "policy_sha256", "inventories")}})


def require_admission_identity(record: dict, anchor: dict) -> None:
    identity = admission_identity(record)
    if not anchor:
        anchor.update(identity)
    elif anchor != identity:
        raise Incomplete("Composition campaign tool/pointer/profile identity changed")


def phase_bookend(check, work: Path, source: Path, records: dict, retained: list,
                  anchor: dict, index: int) -> None:
    """Re-admit source/tool/receipt custody at each completed pre-caller phase."""
    if index not in {2, 11, 13}:
        raise Incomplete("Unknown composition phase bookend")
    path = work / f"phase-{index:02d}-admission.json"
    _, record = admission(check, path)
    require_admission_identity(record, anchor)
    require_sources(check, source, records)
    BASE.retain_artifacts(check, work, retained, [path])
    remaining_budget()


def execute_job(check, work: Path, source: Path, records: dict, job: dict, index: int,
                retained: list, certificates: dict, anchor: dict) -> dict:
    """Run one fixed proof, sealing prior raw evidence and refusing early callers."""
    if (index, job) not in schedule(source):
        raise Incomplete("Composition command is outside the fixed eighteen jobs")
    folder = job_folder(work, index, job)
    inputs = folder / "inputs"
    inputs.mkdir(mode=0o700, parents=True)
    started = time.monotonic()
    record = {"index": index, "job": job, "output": str(folder), "consumer": CONSUMER,
              "expected_outcome_qualified": False}
    try:
        remaining_budget()
        BASE.audit_retained(check, work, retained)
        require_sources(check, source, records)
        for name in records:
            target = inputs / "frozen" / name
            target.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
            target.write_bytes((source / name).read_bytes())
        prepared = projection(source).prepare(source, inputs, job)
        entry = prepared["entrypoint"]
        if entry != inputs / "proof.rs" or entry != entry.resolve(strict=True) or not entry.is_file():
            raise Incomplete("Composition projection must return canonical inputs/proof.rs")
        if stub_pairs(prepared.get("expected_stubs")) != stub_pairs(job["expected_stubs"]):
            raise Incomplete("Prepared mapping differs from the fixed job")
        validate_generated(check, inputs, prepared)
        for key in CALLEES:
            binding(prepared, key)
        record.update(preparation=prepared, source_original_manifest=records)
        if index == 18:
            original = job_folder(work, 14, schedule(source)[13][1]) / "inputs"
            if BASE.generated_identity(check, inputs) != BASE.generated_identity(check, original):
                raise Incomplete("Restored complete generated/frozen inputs differ from job fourteen")
            record["restored_generated_input_matches_healthy"] = True
        before = check.input_snapshot(work, [inputs])
        record["input_before"] = before
        BASE.write_json(folder / "input-before.json", before)
        admitted, admitted_record = admission(check, folder / "admission.json")
        require_admission_identity(admitted_record, anchor)
        record.update(admission_seconds=admitted_record["seconds"], tool_identity=dict(anchor))
        require_sources(check, source, records)
        if check.input_snapshot(work, [inputs]) != before:
            raise Incomplete("Prepared inputs changed during composition admission")
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
        BASE.audit_retained(check, work, retained)
        require_callees(check, work, source, job, prepared, certificates, anchor)
        current_roots = [inputs, *(folder / n for n in ("input-before.json", "admission.json", "execution.json"))]
        if (work / "summary.json").exists():
            current_roots.append(work / "summary.json")
        sealed = [path for item in retained for path in item["roots"]] + current_roots
        record["runtime_sealed_roots"] = sealed
        BASE.write_json(folder / "execution.json", record)
        current = check.input_snapshot(work, current_roots)
        budget = remaining_budget()
        native_started = time.monotonic()
        native = check.RUNTIME.run(ROOT, work, command, readonly=readonly, path_dirs=paths,
            cwd=inputs, environment=environment, timeout=min(COMMAND_SECONDS, budget),
            cpu_seconds=COMMAND_SECONDS, build_jobs=2, address_space_bytes=8 * 1024**3,
            process_limit=128, sealed=sealed)
        record.update(native=native, execute_seconds=time.monotonic() - native_started,
                      runtime_effective_wall_seconds=min(COMMAND_SECONDS, budget))
        check.RUNTIME._owned_work(work, ROOT)
        BASE.audit_retained(check, work, retained)
        if check.input_snapshot(work, current_roots) != current:
            raise Incomplete("Sealed composition inputs or receipts changed during execution")
        metadata = compiler_receipt(check, folder)
        after = check.input_snapshot(work, [inputs])
        record.update(input_after=after, input_unchanged=before == after)
        BASE.write_json(folder / "input-after.json", after)
        require_sources(check, source, records)
        if before != after:
            raise Incomplete("Complete composition input tree changed")
        report = check.decode(check.read_once(folder / "kani.json"))
        record.update(qualify(report, metadata, native, job, namespace_entry))
        require_callees(check, work, source, job, prepared, certificates, anchor)
        require_sources(check, source, records)
        if check.input_snapshot(work, [inputs]) != before:
            raise Incomplete("Composition inputs changed during qualification")
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
            BASE.retain_artifacts(check, work, retained, roots)
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
    check = module("fraction_composition_admission", ROOT / "scripts/check-analysis.py")
    summary = {"status": "incomplete", "decision": "DEFER", "consumer": CONSUMER,
               "jobs": [], "certificates": {}, "maximum_proof_commands": 18, "retries": 0,
               "preparation_started_unix_ns": PREPARATION_STARTED_UNIX_NS,
               "scope": "pure fraction composition only; wider Core/P1/Windows/package gates remain open"}
    retained, anchor, certificates = [], {}, {}
    records = None
    started = time.monotonic()
    try:
        records = BASE.freeze_inputs(check, source, input_names())
        require_sources(check, source, records)
        BASE.write_json(work / "source-manifest.json", {"source_original_manifest": records})
        BASE.retain_artifacts(check, work, retained, [source, work / "source-manifest.json"])
        jobs = schedule(source)
        summary["planned_jobs"] = [j for _, j in jobs]
        BASE.write_json(work / "summary.json", summary)
        for index, job in jobs:
            remaining_budget()
            record = execute_job(check, work, source, records, job, index, retained, certificates, anchor)
            compact = {key: record.get(key) for key in ("index", "output", "checks", "status_counts", "admission_seconds", "execute_seconds", "qualification_error")}
            compact.update(id=job["id"], qualified=record["expected_outcome_qualified"], exit_code=record.get("native", {}).get("exit_code"))
            summary["jobs"].append(compact)
            BASE.write_json(work / "summary.json", summary)
            print(json.dumps(compact), flush=True)
            if not record["expected_outcome_qualified"]:
                raise Incomplete("Stopped at first unqualified composition job: " + job["id"])
            if job["id"] in CALLEES:
                certificate = issue_certificate(check, work, source, job["id"], anchor)
                certificates[job["id"]] = certificate
                path = Path(record["output"]) / "certificate.json"
                BASE.write_json(path, certificate)
                BASE.retain_artifacts(check, work, retained, [path])
                summary["certificates"][job["id"]] = certificate
            if index in {2, 11, 13}:
                phase_bookend(check, work, source, records, retained, anchor, index)
        remaining_budget()
        summary.update(status="passed", decision="AWAIT_INDEPENDENT_REVIEW")
    except Exception as error:
        summary.update(status="incomplete", decision="DEFER", error=f"{type(error).__name__}: {error}")
    finally:
        try:
            _, final = admission(check, work / "final-admission.json")
            require_admission_identity(final, anchor)
            if records is None:
                raise Incomplete("No complete composition source closure")
            require_sources(check, source, records)
            BASE.retain_artifacts(check, work, retained, [work / "final-admission.json"])
            remaining_budget()
            summary.update(final_admission_seconds=final["seconds"], original_and_frozen_inputs_unchanged=True)
            summary["all_proof_inputs_and_required_receipts_unchanged" if summary["status"] == "passed" else "retained_partial_inputs_and_receipts_unchanged"] = True
        except Exception as error:
            summary.update(status="incomplete", decision="DEFER", final_error=f"{type(error).__name__}: {error}")
        summary["campaign_seconds"] = time.monotonic() - started
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
