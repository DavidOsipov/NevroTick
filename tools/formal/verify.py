#!/usr/bin/env python3
"""Run bounded source-bound arithmetic proofs through the admitted namespace.

Install nothing. Refuse source drift and stop at the first unqualified job.
Evidence is private and external to the checkout; no existing receipt is reused.
"""
from __future__ import annotations

import argparse
from collections import Counter, namedtuple
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import shutil
import stat
import sys
import time


ROOT = Path(__file__).resolve().parents[2]
RUSTFLAGS = "--edition=2024 --check-cfg=cfg(test)"
MAX_JOBS = 12
PARTITION_SELECTOR = "fraction-addition-partitions"
PARTITION_OUTPUT_LEAF = "formal-core-fraction-addition-partitions-20261106-v1"
TOOL_ADMISSION_PROFILES = ("full", "kani-proof")
SOLVER_BACKENDS = ("cadical", "z3")
KANI_BASELINE_FLAGS = (
    "--default-unwind", "1", "--output-format", "regular", "--solver", "cadical",
    "-Z", "unstable-options", "--run-sanity-checks", "--harness-timeout", "60s",
)
Z3_VERSION = "5.3.3"
Z3_EXECUTABLE_SHA256 = "fcd0ca765d1c0b4eac776f442ec0f10294b9a6416be91fad4e52fa19012c8d01"
Z3_INPUTS = ("tools/formal/solver_admission.py", "policy/formal-solvers.json",
             "tools/crypto-aes-siv-witness/fuzz/input_custody.py")
KANI_ENTRYPOINT = "kani/payload/kani-0.68.0/bin/kani-driver"
KANI_COMPILER = "kani/rustup/toolchains/nightly-2026-08-21-x86_64-unknown-linux-gnu"
REQUEST_TIMING_SELECTOR = "request-timing"
REQUEST_TIMING_OUTPUT_LEAF = "formal-request-timing-20261106-v1"
REQUEST_TIMING_PYTHON_NAMES = frozenset({
    "tools/formal/verify_request_timing.py", "tools/formal/python_source_binding.py",
    "tools/formal/verify.py", "tools/formal/additional_suites.py",
    "tools/formal/request_timing_projection.py", "scripts/check-analysis.py",
    "scripts/analysis-runtime.py", "scripts/analysis-controls.py", "scripts/check-environment.py",
})
SELECTION_DEPTH_SELECTOR = "selection-depth"
SELECTION_DEPTH_OUTPUT_LEAF = "formal-selection-depth-20261106-v1"
SELECTION_DEPTH_PYTHON_NAMES = frozenset({
    "tools/formal/verify_selection_depth.py", "tools/formal/python_source_binding.py",
    "tools/formal/verify.py", "tools/formal/additional_suites.py",
    "tools/formal/selection_depth_projection.py", "scripts/check-analysis.py",
    "scripts/analysis-runtime.py", "scripts/analysis-controls.py", "scripts/check-environment.py",
})
LIFECYCLE_ORDINAL_SELECTOR = "lifecycle-ordinal"
LIFECYCLE_ORDINAL_OUTPUT_LEAF = "formal-lifecycle-ordinal-20261106-v1"
LIFECYCLE_ORDINAL_PYTHON_NAMES = frozenset({
    "tools/formal/verify_lifecycle_ordinal.py", "tools/formal/python_source_binding.py",
    "tools/formal/verify.py", "tools/formal/additional_suites.py",
    "tools/formal/lifecycle_ordinal_projection.py", "scripts/check-analysis.py",
    "scripts/analysis-runtime.py", "scripts/analysis-controls.py", "scripts/check-environment.py",
})
LIFECYCLE_SOURCE_FIELDS_SELECTOR = "lifecycle-source-fields"
LIFECYCLE_SOURCE_FIELDS_OUTPUT_LEAF = "formal-lifecycle-source-fields-20261106-v1"
LIFECYCLE_SOURCE_FIELDS_PYTHON_NAMES = frozenset({
    "tools/formal/verify_lifecycle_source_fields.py", "tools/formal/python_source_binding.py",
    "tools/formal/verify.py", "tools/formal/additional_suites.py",
    "tools/formal/lifecycle_source_fields_projection.py", "scripts/check-analysis.py",
    "scripts/analysis-runtime.py", "scripts/analysis-controls.py", "scripts/check-environment.py",
})
SUPPORTER_LABELS_SELECTOR = "supporter-labels"
SUPPORTER_LABELS_OUTPUT_LEAF = "formal-supporter-labels-20261106-v1"
SUPPORTER_LABELS_PYTHON_NAMES = frozenset({
    "tools/formal/verify_supporter_labels.py", "tools/formal/python_source_binding.py",
    "tools/formal/verify.py", "tools/formal/additional_suites.py",
    "tools/formal/supporter_labels_projection.py", "scripts/check-analysis.py",
    "scripts/analysis-runtime.py", "scripts/analysis-controls.py", "scripts/check-environment.py",
})
_ClosedCohort = namedtuple("_ClosedCohort", (
    "selector", "wrapper", "output_leaf", "projection", "python_names",
    "consumer_test", "job_count", "minimum_first_seconds",
))
_CLOSED_COHORTS = (
    _ClosedCohort(REQUEST_TIMING_SELECTOR, "tools/formal/verify_request_timing.py",
                  REQUEST_TIMING_OUTPUT_LEAF, "tools/formal/request_timing_projection.py",
                  REQUEST_TIMING_PYTHON_NAMES, "tools/formal/test_verify_request_timing.py", 5, 1020),
    _ClosedCohort(SELECTION_DEPTH_SELECTOR, "tools/formal/verify_selection_depth.py",
                  SELECTION_DEPTH_OUTPUT_LEAF, "tools/formal/selection_depth_projection.py",
                  SELECTION_DEPTH_PYTHON_NAMES, "tools/formal/test_verify_selection_depth.py", 6, 1200),
    _ClosedCohort(LIFECYCLE_ORDINAL_SELECTOR, "tools/formal/verify_lifecycle_ordinal.py",
                  LIFECYCLE_ORDINAL_OUTPUT_LEAF, "tools/formal/lifecycle_ordinal_projection.py",
                  LIFECYCLE_ORDINAL_PYTHON_NAMES, "tools/formal/test_verify_lifecycle_ordinal.py", 5, 1020),
    _ClosedCohort(LIFECYCLE_SOURCE_FIELDS_SELECTOR, "tools/formal/verify_lifecycle_source_fields.py",
                  LIFECYCLE_SOURCE_FIELDS_OUTPUT_LEAF, "tools/formal/lifecycle_source_fields_projection.py",
                  LIFECYCLE_SOURCE_FIELDS_PYTHON_NAMES, "tools/formal/test_verify_lifecycle_source_fields.py", 5, 1020),
    _ClosedCohort(SUPPORTER_LABELS_SELECTOR, "tools/formal/verify_supporter_labels.py",
                  SUPPORTER_LABELS_OUTPUT_LEAF, "tools/formal/supporter_labels_projection.py",
                  SUPPORTER_LABELS_PYTHON_NAMES, "tools/formal/test_verify_supporter_labels.py", 5, 1020),
)
COHORT_SELECTORS = frozenset(item.selector for item in _CLOSED_COHORTS)
SOURCE_BINDING = globals().get("__source_binding__")
BOUND_SOURCE_ROOT = None
COHORT_CAMPAIGN = None
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
    if SOURCE_BINDING is not None:
        cohort = captured_cohort()
        path = Path(path)
        if not path.is_absolute() or path != path.resolve(strict=True):
            raise Incomplete("Captured Python input redirects or is not absolute")
        if path.is_relative_to(ROOT):
            relative = path.relative_to(ROOT).as_posix()
        elif BOUND_SOURCE_ROOT is not None and path.is_relative_to(BOUND_SOURCE_ROOT):
            relative = path.relative_to(BOUND_SOURCE_ROOT).as_posix()
        else:
            raise Incomplete("Captured Python input escapes the fixed source roots")
        if relative not in cohort.python_names:
            raise Incomplete("Undeclared captured Python input")
        return SOURCE_BINDING.load_module(name, ROOT / relative)
    spec = importlib.util.spec_from_file_location(name, path)
    if spec is None or spec.loader is None:
        raise Incomplete(f"Cannot load local module: {path}")
    loaded = importlib.util.module_from_spec(spec)
    sys.modules[name] = loaded
    spec.loader.exec_module(loaded)
    return loaded


def additional_suites(source: Path = ROOT):
    """Load the closed registry from the named, custody-checked source tree."""
    return module("formal_additional_registry", source / "tools/formal/additional_suites.py")


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
    """Write owned regular receipts without following or truncating aliases."""
    if not path.is_absolute() or path.parent != path.parent.resolve(strict=True):
        raise Incomplete("Artifact parent redirects")
    payload = json.dumps(serializable(value), indent=2, allow_nan=False) + "\n"
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_CLOEXEC | os.O_NOFOLLOW | os.O_NONBLOCK, 0o600)
    try:
        info = os.fstat(descriptor)
        if not stat.S_ISREG(info.st_mode) or info.st_uid != os.geteuid() or info.st_nlink != 1:
            raise Incomplete("Refusing to overwrite an unowned, special or aliased artifact")
        # Inspect the inode before truncation, including existing hardlink aliases.
        os.ftruncate(descriptor, 0)
        with os.fdopen(descriptor, "w", encoding="utf-8", closefd=False) as stream:
            stream.write(payload)
    finally:
        os.close(descriptor)


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


def solver_selection(solver_backend: str, solver_root=None) -> Path | None:
    """Reject unknown backends and noncanonical/path-splitting solver roots."""
    if type(solver_backend) is not str or solver_backend not in SOLVER_BACKENDS:
        raise Incomplete("Unknown proof solver backend")
    if solver_root is None:
        return None
    if solver_backend != "z3" or not isinstance(solver_root, (str, Path)):
        raise Incomplete("An explicit solver root is permitted only for Z3")
    path = Path(solver_root)
    if (not path.is_absolute() or str(path) != str(solver_root)
            or any(character == os.pathsep or ord(character) < 32 or 127 <= ord(character) <= 159
                   for character in str(path))):
        raise Incomplete("Z3 solver root must be a canonical directory safe for one PATH entry")
    try:
        if path != path.resolve(strict=True) or not path.is_dir():
            raise Incomplete("Z3 solver root must be a canonical existing directory")
    except (OSError, RuntimeError) as error:
        raise Incomplete("Z3 solver root is unavailable or redirected") from error
    return path


def solver_flags(check, solver_backend: str = "cadical") -> list[str]:
    """Copy the fixed baseline and replace only the selected solver's value."""
    solver_selection(solver_backend)
    flags = check.PROFILES.get("kani_flags")
    if not isinstance(flags, list) or flags != list(KANI_BASELINE_FLAGS):
        raise Incomplete("Proof command requires the unchanged fixed Kani flags")
    selected = list(flags)
    if solver_backend == "z3":
        selected[selected.index("--solver") + 1] = "z3"
    return selected


def input_names(subject: str, solver_backend: str = "cadical") -> list[str]:
    """Inventory the original crate, specifications, runners and pinned policies."""
    solver_selection(solver_backend)
    if SOURCE_BINDING is not None:
        captured_cohort(subject)
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
    if subject in ("coordinate-exchange", "clock", "leap-calendar", "relative-range", "exchange-composition", "calendar-composition", "leap-ticks", "calendar-finite-laws", "leap-coverage", "endpoint-finish", "fraction-constructor", "fraction-comparison", "fraction-arithmetic", "scaled-value-split", "scaled-value-representation", "fraction-addition-partitions", REQUEST_TIMING_SELECTOR, SELECTION_DEPTH_SELECTOR, LIFECYCLE_ORDINAL_SELECTOR, LIFECYCLE_SOURCE_FIELDS_SELECTOR, SUPPORTER_LABELS_SELECTOR):
        names.update(additional_suites().input_names(subject))
    cohort = cohort_metadata(subject)
    if cohort is not None:
        names.update(cohort.python_names)
        names.add(cohort.consumer_test)
    if subject in {"endpoint-finish", "fraction-constructor", "fraction-comparison", "fraction-arithmetic", "scaled-value-split", "scaled-value-representation", "fraction-addition-partitions"}:
        names.update(("crates/time-core/Cargo.toml",
                      "crates/time-core/src/projection_enclosure.rs",
                      "crates/time-core/src/selection.rs"))
    if solver_backend == "z3":
        names.update(Z3_INPUTS)
    if (ROOT / ".cargo/config.toml").exists():
        names.add(".cargo/config.toml")
    return sorted(names)


def freeze_inputs(check, source: Path, names: list[str]) -> dict:
    """Copy exact bounded regular-file inputs; preserve an original byte manifest."""
    global BOUND_SOURCE_ROOT
    if SOURCE_BINDING is not None:
        cohort = captured_cohort()
        expected = cohort_output() / "source"
        if names != input_names(cohort.selector):
            raise Incomplete("Captured cohort requires its complete fixed source inventory")
        if source != expected or BOUND_SOURCE_ROOT not in (None, source):
            raise Incomplete("Captured cohort requires its one fixed fresh source root")
        SOURCE_BINDING.verify(names)
        BOUND_SOURCE_ROOT = source
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
    if SOURCE_BINDING is not None:
        SOURCE_BINDING.verify(names, records)
    return records


def require_sources(check, source: Path, records: dict, subject: str,
                    solver_backend: str = "cadical") -> None:
    """Reject changed, missing or newly added original inputs and frozen bytes."""
    if input_names(subject, solver_backend) != sorted(records):
        raise Incomplete("Original source/configuration input inventory changed")
    if subject in COHORT_SELECTORS:
        require_cohort_binding(subject)
        if source != BOUND_SOURCE_ROOT:
            raise Incomplete("Captured cohort source root differs from its frozen root")
        SOURCE_BINDING.verify(records, records)
    for name, expected in records.items():
        original = ROOT / name
        if original != original.resolve(strict=True):
            raise Incomplete(f"Source input path changed: {name}")
        for path in (original, source / name):
            data = check.read_once(path)
            if {"sha256": digest(data), "bytes": len(data)} != expected:
                raise Incomplete(f"Original or frozen input drift: {name}")


def cohort_metadata(subject: str):
    """Return fixed inventory metadata only; this grants no execution role."""
    return next((item for item in _CLOSED_COHORTS if item.selector == subject), None)


def job_limit(subject: str) -> int:
    """Preserve ordinary bounds and each captured cohort's literal schedule."""
    cohort = cohort_metadata(subject)
    if cohort is not None:
        return cohort.job_count
    return 16 if subject == PARTITION_SELECTOR else MAX_JOBS


def request_timing_output() -> Path:
    """Return the original timing root, independent of any captured role."""
    return Path.home() / ".local/share/nevrotick" / REQUEST_TIMING_OUTPUT_LEAF


def selection_depth_output() -> Path:
    """Return the selection card's sole fresh root without caller overrides."""
    return Path.home() / ".local/share/nevrotick" / SELECTION_DEPTH_OUTPUT_LEAF


def lifecycle_ordinal_output() -> Path:
    """Return the ordinal card's sole fresh root without caller overrides."""
    return Path.home() / ".local/share/nevrotick" / LIFECYCLE_ORDINAL_OUTPUT_LEAF


def lifecycle_source_fields_output() -> Path:
    """Return the source-counter card's sole fresh root without caller overrides."""
    return Path.home() / ".local/share/nevrotick" / LIFECYCLE_SOURCE_FIELDS_OUTPUT_LEAF


def supporter_labels_output() -> Path:
    """Return the supporter-label card's sole fresh root without overrides."""
    return Path.home() / ".local/share/nevrotick" / SUPPORTER_LABELS_OUTPUT_LEAF


def captured_cohort(subject: str | None = None):
    """Resolve immutable wrapper identity; source bookends verify current bytes."""
    if (SOURCE_BINDING is None or SOURCE_BINDING is not globals().get("__source_binding__")
            or SOURCE_BINDING.root != ROOT):
        raise Incomplete("Closed cohort requires its captured fixed wrapper")
    snapshot = SOURCE_BINDING.snapshot()
    candidates = [item for item in _CLOSED_COHORTS if item.wrapper in snapshot]
    if (len(candidates) != 1
            or not {"tools/formal/python_source_binding.py", "tools/formal/verify.py"} <= snapshot.keys()):
        raise Incomplete("Closed cohort requires exactly one captured fixed wrapper")
    cohort = candidates[0]
    if subject is not None and subject != cohort.selector:
        raise Incomplete("Captured fixed wrapper cannot select another cohort subject")
    if not snapshot.keys() <= cohort.python_names:
        raise Incomplete("Captured Python source is outside the fixed wrapper inventory")
    return cohort


def require_cohort_binding(subject: str | None = None):
    """Verify all executed source bytes at the selected cohort's bookends."""
    cohort = captured_cohort(subject)
    SOURCE_BINDING.verify(cohort.python_names)
    return cohort


def cohort_output() -> Path:
    """Return only the root of the role established by captured execution."""
    cohort = captured_cohort()
    return Path.home() / ".local/share/nevrotick" / cohort.output_leaf


def cohort_resources(work: Path | None = None) -> dict:
    """Bound retained owned files, excluding only exact disposable job targets."""
    cohort = captured_cohort()
    location = cohort_output().parent if work is None else work
    if shutil.disk_usage(location).free < 3 * 1024**3:
        raise Incomplete("Closed cohort requires three GiB free disk reserve")
    total = 0
    if work is not None:
        if work != cohort_output() or work != work.resolve(strict=True):
            raise Incomplete("Closed cohort resource root redirects or differs")
        targets = {work / "jobs" / f"{index:02d}-{job['subject']}-{job['case']}-{job['harness']}" / "target"
                   for index, job in enumerate(jobs(cohort.selector), 1)}
        def unreadable(error):
            raise Incomplete("Closed cohort retained tree is unreadable") from error

        entries = 0
        for directory, subdirs, files in os.walk(work, followlinks=False, onerror=unreadable):
            current = Path(directory)
            for name in [*subdirs, *files]:
                entries += 1
                if entries > 100000:
                    raise Incomplete("Closed cohort retained tree exceeds its finite entry bound")
                item = current / name
                info = item.lstat()
                if (info.st_uid != os.geteuid() or item != item.resolve(strict=True)
                        or not (stat.S_ISDIR(info.st_mode) or stat.S_ISREG(info.st_mode))
                        or stat.S_ISREG(info.st_mode) and info.st_nlink != 1):
                    raise Incomplete("Closed cohort retained input aliases or is not owned/regular")
                if item in targets:
                    if not stat.S_ISDIR(info.st_mode):
                        raise Incomplete("Disposable target is not an owned directory")
                    subdirs.remove(name)
                elif stat.S_ISREG(info.st_mode):
                    total += info.st_size
                    if total > 128 * 1024**2:
                        raise Incomplete("Closed cohort retained source/receipts exceed 128 MiB")
    return {"retained_bytes": total, "minimum_free_bytes": 3 * 1024**3,
            "maximum_retained_bytes": 128 * 1024**2}


def cleanup_cohort_targets(check, work: Path) -> list[str]:
    """Remove only exact role-owned targets after terminal native custody checks."""
    cohort = captured_cohort()
    if (work != cohort_output() or work != work.resolve(strict=True)
            or not work.is_dir()):
        raise Incomplete("Refusing cleanup outside the exact canonical captured-cohort work root")
    check.RUNTIME._owned_work(work, ROOT)
    if not shutil.rmtree.avoids_symlink_attacks:
        raise Incomplete("Safe descriptor-based target cleanup is unavailable")
    targets = []
    for index, job in enumerate(jobs(cohort.selector), 1):
        path = work / "jobs" / f"{index:02d}-{job['subject']}-{job['case']}-{job['harness']}" / "target"
        if path.exists() or path.is_symlink():
            if (path != path.resolve(strict=True) or not path.is_dir()
                    or path.lstat().st_uid != os.geteuid()):
                raise Incomplete("Refusing aliased or unowned disposable target cleanup")
            for directory, _, _ in os.walk(path, followlinks=False):
                if Path(directory).is_mount():
                    raise Incomplete("Refusing mounted disposable target cleanup")
            targets.append(path)
    removed = []
    for path in targets:
        shutil.rmtree(path)
        removed.append(path.relative_to(work).as_posix())
    check.RUNTIME._owned_work(work, ROOT)
    return removed


def partition_projection(source: Path = ROOT):
    """Load the reviewed ordinary partition spec without installing or executing tools."""
    return module("formal_partition_budget", source / "tools/formal/fraction_addition_partitions_projection.py")


def partition_remaining(subject: str, source: Path = ROOT) -> float | None:
    """Check the selected finite card's once-recorded source-to-decision clock."""
    if SOURCE_BINDING is not None:
        captured_cohort(subject)
    if subject in COHORT_SELECTORS:
        cohort = require_cohort_binding(subject)
        projection = module("formal_" + subject.replace("-", "_") + "_budget", source / cohort.projection)
        remaining = projection.remaining_budget()
        if (isinstance(remaining, bool) or not isinstance(remaining, (int, float))
                or not math.isfinite(remaining) or not 0 < remaining <= 3600):
            raise Incomplete("Captured cohort preparation/execution budget expired or malformed")
        return float(remaining)
    if subject != PARTITION_SELECTOR:
        return None
    try:
        remaining = partition_projection(source).remaining_budget()
    except (OSError, ValueError) as error:
        raise Incomplete("Partition preparation/execution budget unavailable: " + str(error)) from error
    if (isinstance(remaining, bool) or not isinstance(remaining, (int, float))
            or not math.isfinite(remaining) or not 0 < remaining <= 3600):
        raise Incomplete("Partition preparation/execution budget expired or malformed")
    return float(remaining)


def partition_preflight(subject: str, profile: str, solver_backend: str,
                        solver_root, output_dir: str) -> None:
    """Constrain captured cohorts without changing ordinary lane options."""
    if SOURCE_BINDING is not None:
        captured_cohort(subject)
    if subject in COHORT_SELECTORS:
        cohort = require_cohort_binding(subject)
        if (profile != "kani-proof" or solver_backend != "cadical" or solver_root is not None
                or output_dir != str(cohort_output())):
            raise Incomplete("Captured cohort requires the fixed output and Kani-proof/Cadical role")
        if partition_remaining(subject) < cohort.minimum_first_seconds:
            raise Incomplete(f"{subject} requires {cohort.minimum_first_seconds} seconds before first admission")
        cohort_resources()
        return
    if subject != PARTITION_SELECTOR:
        return
    if profile != "kani-proof" or solver_backend != "cadical" or solver_root is not None:
        raise Incomplete("Partition card requires the original Kani-proof/Cadical profile")
    expected = Path.home() / ".local/share/nevrotick" / PARTITION_OUTPUT_LEAF
    if output_dir != str(expected):
        raise Incomplete("Partition card requires its one fixed fresh output directory")
    partition_remaining(subject)


def generated_identity(check, inputs: Path) -> dict:
    """Compare complete generated/frozen input bytes, not differing job-root paths."""
    result = {}
    for path in sorted(inputs.rglob("*")):
        if path.is_dir() and not path.is_symlink():
            continue
        if path != path.resolve(strict=True) or not path.is_file():
            raise Incomplete("Generated partition input redirects or is not a regular file")
        data = check.read_once(path)
        result[path.relative_to(inputs).as_posix()] = {"sha256": digest(data), "bytes": len(data)}
    if not result:
        raise Incomplete("No generated proof input bytes")
    return result


def jobs(subject: str) -> list[dict]:
    """Return the complete fixed schedule, without retries or solver concurrency."""
    if SOURCE_BINDING is not None:
        captured_cohort(subject)
    if subject in ("coordinate-exchange", "clock", "leap-calendar", "relative-range", "exchange-composition", "calendar-composition", "leap-ticks", "calendar-finite-laws", "leap-coverage", "endpoint-finish", "fraction-constructor", "fraction-comparison", "fraction-arithmetic", "scaled-value-split", "scaled-value-representation", "fraction-addition-partitions", REQUEST_TIMING_SELECTOR, SELECTION_DEPTH_SELECTOR, LIFECYCLE_ORDINAL_SELECTOR, LIFECYCLE_SOURCE_FIELDS_SELECTOR, SUPPORTER_LABELS_SELECTOR):
        result = additional_suites().jobs(subject)
        limit = job_limit(subject)
        if not 1 <= len(result) <= limit or (subject in COHORT_SELECTORS | {PARTITION_SELECTOR} and len(result) != limit):
            raise Incomplete("Empty or excessive additional proof schedule")
        return result
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


def selected_jobs(subject: str, start_at: int, only_jobs: list[int] | None = None) -> list[tuple[int, dict]]:
    """Select fixed indices or a nonempty suffix without qualifying omitted jobs.

    This is explicit partial execution, not automatic receipt reuse or campaign
    resumption. The coordinator must separately reconcile any earlier evidence.
    """
    if subject in COHORT_SELECTORS and (start_at != 1 or only_jobs is not None):
        raise Incomplete("Captured cohort requires its complete fixed selection")
    if subject == PARTITION_SELECTOR and (start_at != 1 or only_jobs is not None):
        raise Incomplete("Partition theorem requires the full fixed sixteen-job selection")
    schedule = jobs(subject)
    if type(start_at) is not int or not 1 <= start_at <= len(schedule):
        raise Incomplete("Start index must select a nonempty fixed-schedule suffix")
    if only_jobs is not None:
        if start_at != 1:
            raise Incomplete("Explicit job indices cannot be combined with a schedule suffix")
        if (type(only_jobs) is not list or not only_jobs
                or any(type(index) is not int or not 1 <= index <= len(schedule) for index in only_jobs)
                or len(set(only_jobs)) != len(only_jobs)):
            raise Incomplete("Explicit jobs must be nonempty unique fixed-schedule indices")
        return [(index, schedule[index - 1]) for index in sorted(only_jobs)]
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


def qualify(report: dict, native: dict, job: dict, entrypoint: str | None = None,
            *, expected_solver: str = "cadical") -> dict:
    """Fail closed on empty, partial, mismatched or unintended Kani outcomes."""
    solver_selection(expected_solver)
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
            or harnesses[0].get("attributes", {}).get("kind") != "Proof"
            or harnesses[0].get("attributes", {}).get("should_panic") is not False
            or (entrypoint is not None and
                (not isinstance(harnesses[0].get("source"), dict)
                 or harnesses[0]["source"].get("file") != entrypoint))):
        raise Incomplete("Harness metadata differs from the selected ordinary proof")
    source_unwind = None
    if (job["subject"] == "supporter_labels"
            or isinstance(job["harness"], str) and job["harness"].startswith("supporter_labels::")):
        fixed_jobs = additional_suites().jobs(SUPPORTER_LABELS_SELECTOR)
        if not any((job["subject"], job["harness"]) == (item["subject"], item["harness"])
                   for item in fixed_jobs):
            raise Incomplete("Source unwind profile requires the fixed supporter-label subject and harness")
        attributes = harnesses[0]["attributes"]
        unwind_value = attributes.get("unwind_value")
        if type(unwind_value) is not int or unwind_value != 17:
            raise Incomplete("Supporter-label proof requires the exact source unwind integer 17")
        for field in ("stubs", "verified_stubs"):
            value = attributes.get(field, [])
            if type(value) is not list or value:
                raise Incomplete("Supporter-label proof permits no stubs or verified stubs")
        source_unwind = 17
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
        if job["subject"] == "rounding":
            expected_covers, expected_assertions = ROUNDING_COVERS, ROUNDING_ASSERTIONS
        elif job["subject"] == "era":
            expected_covers, expected_assertions = ERA_COVERS, ERA_ASSERTIONS
        else:
            expected_assertions, expected_covers = additional_suites().expected_contract(
                job["subject"], job["harness"])
            if not expected_assertions or not expected_covers:
                raise Incomplete("Additional proofs require nonempty fixed mathematical obligations")
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
    qualified = {"verification_summary": summary, "checks": len(checks), "status_counts": dict(counts),
            "failed_checks": failures, "covers": covers, "solver_seconds": solver_seconds,
            "decision_procedure_seconds": decision_seconds if expected_solver == "z3" else None,
            "timing_field": timing_field, "timing_meaning": timing_meaning,
            "reachability_and_sanity_checks": [item for item in checks
                if "reach" in item["category"] or "sanity" in item["category"]],
            "tools": tools, "metadata": metadata, "error_details": errors,
            "expected_solver_backend": expected_solver,
            "property_details": properties, "cbmc": cbmc, "harness_metadata": harnesses}
    if source_unwind is not None:
        qualified.update(expected_source_unwind=17, effective_source_unwind=source_unwind)
    return qualified


def proof_readonly_roots(admitted: dict, profile: str) -> tuple[Path, Path]:
    """Require the selected role's exact canonical read-only payload roots."""
    if type(profile) is not str or profile not in TOOL_ADMISSION_PROFILES:
        raise Incomplete("Unknown proof tool-admission profile")
    if not isinstance(admitted, dict) or admitted.get("admission_profile") != profile:
        raise Incomplete("Proof tool admission returned a different profile")
    base, stable = admitted.get("base"), admitted.get("stable")
    if not isinstance(base, Path) or not isinstance(stable, Path):
        raise Incomplete("Proof admission requires canonical installation paths")
    if profile == "kani-proof":
        kani = admitted.get("policy", {}).get("tools", {}).get("kani", {})
        if (not isinstance(kani, dict) or kani.get("entrypoint") != KANI_ENTRYPOINT
                or kani.get("toolchain") != KANI_COMPILER):
            raise Incomplete("Kani proof admission must retain fixed compiler and entrypoint paths")
        expected = (base / "kani/payload/kani-0.68.0", base / KANI_COMPILER)
    else:
        expected = (base, stable)
    roots = admitted.get("readonly_roots")
    if (not isinstance(roots, (tuple, list)) or len(roots) != 2
            or any(not isinstance(path, Path) for path in roots)
            or tuple(roots) != expected or len(set(roots)) != 2):
        raise Incomplete("Proof admission returned different read-only payload roots")
    for path in roots:
        if not path.is_absolute() or path != path.resolve(strict=True) or not path.is_dir():
            raise Incomplete("Proof read-only payload roots must be canonical existing directories")
    return tuple(roots)


def checked_z3_admission(value: dict, admitted: dict, source: Path, *, solver_root=None,
                          work: Path | None = None) -> dict:
    """Accept only the policy-pinned solver's one canonical, disjoint closure."""
    requested = solver_selection("z3", solver_root)
    if (not isinstance(value, dict) or value.get("name") != "z3" or value.get("version") != Z3_VERSION
            or value.get("executable_sha256") != Z3_EXECUTABLE_SHA256):
        raise Incomplete("Z3 admission differs from the fixed solver identity")
    root = value.get("readonly_root")
    if not isinstance(root, Path):
        raise Incomplete("Z3 admission requires one canonical read-only root")
    root = solver_selection("z3", root)
    if requested is not None and root != requested:
        raise Incomplete("Z3 admission returned a different explicitly selected root")
    executable, path_dir = value.get("executable"), value.get("path_dir")
    if (not isinstance(executable, Path) or not isinstance(path_dir, Path)
            or executable != root / "bin/z3" or path_dir != root / "bin"
            or executable != executable.resolve(strict=True) or not executable.is_file()
            or path_dir != path_dir.resolve(strict=True) or not path_dir.is_dir()
            or not os.access(executable, os.X_OK)):
        raise Incomplete("Z3 executable and PATH must be exactly the admitted root/bin/z3")
    forbidden = (ROOT, source, admitted["base"], admitted["stable"])
    if work is not None:
        forbidden += (work,)
    if any(root.is_relative_to(path) or path.is_relative_to(root) for path in forbidden):
        raise Incomplete("Z3 closure must be separate from checkout, work and Kani/stable stores")
    policy_path = source / "policy/formal-solvers.json"
    if value.get("policy_path") != policy_path:
        raise Incomplete("Z3 admission must use the selected frozen solver policy")
    for field in ("policy_sha256", "manifest_sha256", "executable_sha256"):
        item = value.get(field)
        if not isinstance(item, str) or len(item) != 64 or any(character not in "0123456789abcdef" for character in item):
            raise Incomplete("Z3 admission requires complete policy/manifest/executable hashes")
    if not isinstance(value.get("inventory"), dict) or not value["inventory"]:
        raise Incomplete("Z3 admission requires its complete nonempty file inventory")
    if (not isinstance(value.get("closure"), dict)
            or value["closure"].get("host_runtime") != "existing-public-host-trusted-base-not-rehashed"
            or value["closure"].get("per_command_library_hashes_checked") is not False):
        raise Incomplete("Z3 admission must retain the existing public host-runtime trust boundary")
    for before, after in (("store_identity_before", "store_identity_after"),
                          ("ancestor_identity_before", "ancestor_identity_after")):
        if (not isinstance(value.get(before), dict) or not value[before]
                or value.get(after) != value[before]):
            raise Incomplete("Z3 admission requires unchanged complete store and ancestor bookends")
    return value


def admit_z3_solver(admitted: dict, source: Path, *, solver_root=None,
                    work: Path | None = None) -> dict:
    """Load the frozen policy helper; it hashes files but never executes Z3."""
    helper = module("formal_z3_admission", source / "tools/formal/solver_admission.py")
    value = helper.admit(source, solver_root=solver_root)
    return checked_z3_admission(value, admitted, source, solver_root=solver_root, work=work)


def solver_identity(value: dict) -> dict:
    """Select stable payload/policy facts for Z3's pre/post-execution bookend."""
    return {field: value[field] for field in (
        "name", "version", "readonly_root", "executable", "path_dir", "policy_path",
        "policy_sha256", "manifest_sha256", "executable_sha256", "inventory", "closure",
        "store_identity_before", "ancestor_identity_before")}


def admission(check, output: Path, *, profile: str = "full", solver_backend: str = "cadical",
              solver_root=None, source: Path = ROOT, work: Path | None = None) -> tuple[dict, dict]:
    """Perform explicit-role payload admission and retain exact pointer/tool identity."""
    if type(profile) is not str or profile not in TOOL_ADMISSION_PROFILES:
        raise Incomplete("Unknown proof tool-admission profile")
    requested_solver = solver_selection(solver_backend, solver_root)
    flags = solver_flags(check, solver_backend)
    pointer = Path.home() / ".local/share/nevrotick/analysis-installation.json"
    started = time.monotonic()
    before = check.read_once(pointer, maximum=4096, private=True)
    admitted = check.admit(ROOT) if profile == "full" else check.admit_kani_proof(ROOT)
    if check.read_once(pointer, maximum=4096, private=True) != before:
        raise Incomplete(f"Installation pointer changed during {profile} admission")
    readonly = proof_readonly_roots(admitted, profile)
    solver = None
    if solver_backend == "z3":
        solver = admit_z3_solver(admitted, source, solver_root=requested_solver, work=work)
        readonly += (solver["readonly_root"],)
        if check.read_once(pointer, maximum=4096, private=True) != before:
            raise Incomplete("Installation pointer changed during Z3 payload admission")
    kani = admitted["policy"]["tools"]["kani"]
    compiler = admitted["base"] / kani["toolchain"]
    record = {"seconds": time.monotonic() - started, "admitted": admitted,
              "expected_admission_profile": profile, "readonly_roots": readonly,
              "solver_backend": solver_backend, "solver_admission": solver,
              "requested_solver_root": requested_solver,
              "host_runtime_trust": "existing admitted Python/stdlib and public OS runtime remain unchanged",
              "installation_pointer": str(pointer), "installation_pointer_sha256": digest(before),
              "installation_pointer_record": check.decode(before),
              "kani_executable_sha256": check.digest(admitted["base"] / kani["entrypoint"]),
              "compiler_rustc_sha256": check.digest(compiler / "bin/rustc"),
              "compiler_rustdoc_sha256": check.digest(compiler / "bin/rustdoc"),
              "profile": {"kani_flags": flags, "rustflags": RUSTFLAGS,
                          "wall_seconds": 180, "cpu_seconds": 180,
                          "address_space_bytes": 8 * 1024 ** 3, "process_limit": 128,
                          "build_jobs": 2, "solver_workers": 1}}
    write_json(output, record)
    return admitted, record


def audit_retained(check, work: Path, retained: list[dict]) -> None:
    """Recheck all anchored proof islands and required receipts, never targets."""
    for item in retained:
        if check.input_snapshot(work, item["roots"]) != item["snapshot"]:
            raise Incomplete("An earlier proof input island or retained receipt changed")


def retain_artifacts(check, work: Path, retained: list[dict], roots: list[Path]) -> None:
    """Anchor newly completed coordinator-owned facts without rebasing old ones."""
    audit_retained(check, work, retained)
    retained.append({"roots": list(roots), "snapshot": check.input_snapshot(work, roots)})


def job_receipts(folder: Path, solver_backend: str, *, required: bool) -> list[Path]:
    """Select required JSON evidence while excluding disposable compiler targets."""
    names = ["input-before.json", "admission.json", "input-after.json", "kani.json", "execution.json"]
    if solver_backend == "z3":
        names.append("solver-after-admission.json")
    paths = [folder / name for name in names]
    if required and any(not path.exists() or path.is_symlink() for path in paths):
        raise Incomplete("A qualified job is missing a required regular JSON receipt")
    return paths if required else [path for path in paths if path.exists() or path.is_symlink()]


def execute_job(check, work: Path, source: Path, records: dict, subject: str,
                job: dict, index: int, *, tool_admission_profile: str = "full",
                solver_backend: str = "cadical", solver_root=None, retained: list[dict] | None = None) -> dict:
    """Prepare and qualify one command while preserving the campaign's old facts.

    main supplies its cumulative in-memory custody list. A standalone call owns
    only its frozen source and current job; it establishes no cross-job campaign.
    """
    if SOURCE_BINDING is not None:
        captured_cohort(subject)
    if subject in COHORT_SELECTORS:
        cohort = require_cohort_binding(subject)
        campaign = COHORT_CAMPAIGN
        if (type(campaign) is not dict or campaign.get("cohort") is not cohort
                or campaign.get("work") != work or work != cohort_output()
                or campaign.get("source") != source or source != BOUND_SOURCE_ROOT
                or campaign.get("retained") is not retained
                or campaign.get("next_index") != index or type(index) is not int or not 1 <= index <= cohort.job_count
                or job != jobs(subject)[index - 1]
                or tool_admission_profile != "kani-proof" or solver_backend != "cadical"
                or solver_root is not None):
            raise Incomplete("Captured cohort permits only its complete fixed campaign without retries")
        campaign["next_index"] += 1
    started = time.monotonic()
    folder = work / "jobs" / f"{index:02d}-{job['subject']}-{job['case']}-{job['harness']}"
    inputs = folder / "inputs"
    inputs.mkdir(mode=0o700, parents=True)
    record = {"job": job, "expected_outcome_qualified": False, "output": str(folder),
              "expected_admission_profile": tool_admission_profile, "expected_solver_backend": solver_backend}
    try:
        if subject == PARTITION_SELECTOR and (
                tool_admission_profile != "kani-proof" or solver_backend != "cadical" or solver_root is not None):
            raise Incomplete("Partition commands require the fixed Kani-proof/Cadical role")
        if retained is None:
            retained = []
            retain_artifacts(check, work, retained, [source])
        audit_retained(check, work, retained)
        partition_remaining(subject, source)
        requested_solver = solver_selection(solver_backend, solver_root)
        flags = solver_flags(check, solver_backend)
        require_sources(check, source, records, subject, solver_backend)
        # Preserve a complete runner/spec/policy/source copy for this individual command.
        for name in records:
            target = inputs / "frozen" / name
            target.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
            target.write_bytes((source / name).read_bytes())
        if job["subject"] == "rounding":
            prepared = prepare_rounding(source, inputs, job["case"])
        elif job["subject"] == "era":
            projection = module(f"formal_era_projection_{index}", source / "tools/formal/era_projection.py")
            prepared = projection.prepare_era(source, inputs, job["case"],
                                               source / "tools/formal/era_harness.rs")
        else:
            prepared = additional_suites(source).prepare(module, source, inputs, job, index)
        entry = prepared["entrypoint"]
        if (not isinstance(entry, Path) or entry != inputs / "proof.rs"
                or not entry.is_file() or entry != entry.resolve(strict=True)):
            raise Incomplete("Projection must return the canonical inputs/proof.rs")
        record.update(preparation=prepared, prepare_seconds=time.monotonic() - started,
                      source_original_manifest=records)
        if subject in COHORT_SELECTORS | {PARTITION_SELECTOR} and job["restored_healthy"]:
            original_index = 1 if subject in COHORT_SELECTORS else index - 9
            if (subject == PARTITION_SELECTOR and not 2 <= original_index <= 7
                    or subject in COHORT_SELECTORS and index != cohort.job_count):
                raise Incomplete("Unexpected restored partition index")
            original = work / "jobs" / f"{original_index:02d}-{job['subject']}-{job['case']}-{job['harness']}" / "inputs"
            if generated_identity(check, inputs) != generated_identity(check, original):
                raise Incomplete("Restored partition input bytes differ from its qualified healthy leaf")
            record.update(restored_generated_input_matches_healthy=True,
                          restored_healthy_inputs=original)
        before = check.input_snapshot(work, [inputs])
        record["input_before"] = before
        write_json(folder / "input-before.json", before)
        solver_options = {} if solver_backend == "cadical" else {
            "solver_backend": solver_backend, "solver_root": requested_solver, "source": source, "work": work}
        if subject in COHORT_SELECTORS:
            require_sources(check, source, records, subject, solver_backend)
            cohort_resources(work)
            if index == 1 and partition_remaining(subject, source) < cohort.minimum_first_seconds:
                raise Incomplete(f"{subject} requires {cohort.minimum_first_seconds} seconds before first admission")
        admitted, admitted_record = admission(check, folder / "admission.json", profile=tool_admission_profile,
                                              **solver_options)
        record["admission_seconds"] = admitted_record["seconds"]
        partition_remaining(subject, source)
        require_sources(check, source, records, subject, solver_backend)
        if check.input_snapshot(work, [inputs]) != before:
            raise Incomplete("Prepared inputs changed during tool admission")
        base = admitted["base"]
        kani = admitted["policy"]["tools"]["kani"]
        compiler = base / kani["toolchain"]
        namespace_entry = "/work/" + entry.relative_to(work).as_posix()
        command = [str(base / kani["entrypoint"]), namespace_entry,
                   "--harness", job["harness"], "--exact", *flags,
                   "--export-json", "/work/" + (folder / "kani.json").relative_to(work).as_posix(),
                   "--target-dir", "/work/" + (folder / "target").relative_to(work).as_posix()]
        environment = {"RUSTC": str(compiler / "bin/rustc"),
                       "RUSTDOC": str(compiler / "bin/rustdoc"),
                       "RUSTUP_HOME": str(base / "kani/rustup"),
                       "RUSTUP_TOOLCHAIN": "nightly-2026-08-21", "RUSTFLAGS": RUSTFLAGS}
        record.update(command=command, explicit_environment=environment)
        readonly = admitted["readonly_roots"]
        paths = [base / "kani/payload/kani-0.68.0/bin", compiler / "bin"]
        if solver_backend == "z3":
            solver = checked_z3_admission(admitted_record.get("solver_admission"), admitted, source,
                                           solver_root=requested_solver, work=work)
            readonly = (*tuple(readonly), solver["readonly_root"])
            if tuple(admitted_record.get("readonly_roots", ())) != readonly:
                raise Incomplete("Z3 runtime mounts differ from the exact admitted three roots")
            paths.insert(0, solver["path_dir"])
        record.update(runtime_readonly_roots=readonly, runtime_path_dirs=paths)
        # Preserve the old current-input fields; cumulative custody is separate.
        audit_retained(check, work, retained)
        pre_receipts = [folder / name for name in ("input-before.json", "admission.json", "execution.json")]
        write_json(folder / "execution.json", record)
        if any(not path.exists() or path.is_symlink() for path in pre_receipts):
            raise Incomplete("A proof command is missing a required pre-execution receipt")
        current_roots = [inputs, *pre_receipts]
        summary_path = work / "summary.json"
        if summary_path.exists() or summary_path.is_symlink():
            current_roots.append(summary_path)
        sealed = [path for item in retained for path in item["roots"]] + current_roots
        record["runtime_sealed_roots"] = sealed
        budget_before_snapshot = partition_remaining(subject, source)
        if budget_before_snapshot is not None:
            record["preparation_budget_remaining_before_snapshot_seconds"] = budget_before_snapshot
            record["runtime_wall_seconds_upper_bound"] = min(180, budget_before_snapshot)
        write_json(folder / "execution.json", record)
        # execution.json just gained the actual seal list through the owner.
        current_snapshot = check.input_snapshot(work, current_roots)
        budget = partition_remaining(subject, source)
        command_timeout = 180 if budget is None else min(180, budget)
        if budget is not None:
            record["preparation_budget_remaining_seconds"] = budget
            record["runtime_effective_wall_seconds"] = command_timeout
        execute_start = time.monotonic()
        native = check.RUNTIME.run(ROOT, work, command, readonly=readonly,
            path_dirs=paths, cwd=inputs,
            environment=environment, timeout=command_timeout, cpu_seconds=180, build_jobs=2,
            address_space_bytes=8 * 1024 ** 3, process_limit=128, sealed=sealed)
        record.update(native=native, execute_seconds=time.monotonic() - execute_start)
        # Close native filesystem authority before ANY coordinator artifact write.
        check.RUNTIME._owned_work(work, ROOT)
        audit_retained(check, work, retained)
        if check.input_snapshot(work, current_roots) != current_snapshot:
            raise Incomplete("Current inputs, command receipts or checkpoint changed during execution")
        after = check.input_snapshot(work, [inputs])
        record.update(input_after=after, input_unchanged=before == after)
        write_json(folder / "input-after.json", after)
        require_sources(check, source, records, subject, solver_backend)
        if before != after:
            raise Incomplete("Complete proof input tree changed during execution")
        if solver_backend == "z3":
            solver_after = admit_z3_solver(admitted, source, solver_root=requested_solver, work=work)
            write_json(folder / "solver-after-admission.json", solver_after)
            record["solver_after_admission"] = solver_after
            if solver_identity(solver_after) != solver_identity(solver):
                raise Incomplete("Z3 solver policy or payload identity changed during execution")
            pointer = Path.home() / ".local/share/nevrotick/analysis-installation.json"
            pointer_after = digest(check.read_once(pointer, maximum=4096, private=True))
            record["installation_pointer_after_sha256"] = pointer_after
            if pointer_after != admitted_record.get("installation_pointer_sha256"):
                raise Incomplete("Installation pointer changed during Z3 proof execution")
        report = check.decode(check.read_once(folder / "kani.json"))
        observed = qualify(report, native, job, namespace_entry, expected_solver=solver_backend)
        require_sources(check, source, records, subject, solver_backend)
        record.update(observed)
        record["execute_other_seconds"] = (record["execute_seconds"] - observed["solver_seconds"]
                                           if observed["solver_seconds"] is not None else None)
        partition_remaining(subject, source)
        record["expected_outcome_qualified"] = True
    except Exception as error:
        record["qualification_error"] = f"{type(error).__name__}: {error}"
    finally:
        record["job_total_seconds"] = time.monotonic() - started
        try:
            write_json(folder / "execution.json", record)
            if retained is not None:
                receipts = job_receipts(folder, solver_backend, required=record["expected_outcome_qualified"])
                retain_artifacts(check, work, retained, [inputs, *receipts])
        except Exception as error:
            record.update(expected_outcome_qualified=False,
                          persistence_error=f"{type(error).__name__}: {error}")
            record.setdefault("qualification_error", "Custody persistence failed: " + record["persistence_error"])
            # A refused receipt remains unqualified. Return the in-memory
            # failure so main still accounts for a command that actually ran.
            try:
                write_json(folder / "execution.json", record)
            except Exception as write_error:
                record["persistence_write_error"] = f"{type(write_error).__name__}: {write_error}"
    return record


def main(argv=None) -> int:
    """Run finite selected commands and repeat their admission role at closeout."""
    global COHORT_CAMPAIGN
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", required=True,
                        help="fresh canonical absolute private directory outside this checkout")
    parser.add_argument("--subject", choices=("rounding", "era", "all", "coordinate-exchange",
                                             "clock", "leap-calendar", "relative-range", "exchange-composition", "calendar-composition", "leap-ticks", "calendar-finite-laws", "leap-coverage", "endpoint-finish", "fraction-constructor", "fraction-comparison", "fraction-arithmetic", "scaled-value-split", "scaled-value-representation", "fraction-addition-partitions", REQUEST_TIMING_SELECTOR, SELECTION_DEPTH_SELECTOR, LIFECYCLE_ORDINAL_SELECTOR, LIFECYCLE_SOURCE_FIELDS_SELECTOR, SUPPORTER_LABELS_SELECTOR), default="all",
                        help="fixed suite; all retains the original twelve-job rounding/era schedule")
    parser.add_argument("--start-at", type=int, default=1,
                        help="execute only this fixed-schedule suffix; omitted jobs are not qualified")
    parser.add_argument("--only-jobs", type=int, nargs="+",
                        help="execute sorted unique fixed job indices; omitted jobs are not qualified")
    parser.add_argument("--tool-admission-profile", choices=TOOL_ADMISSION_PROFILES, default="full",
                        help="explicit proof payload closure; full preserves the original default")
    parser.add_argument("--solver-backend", choices=SOLVER_BACKENDS, default="cadical",
                        help="closed proof solver selection; cadical preserves the original default")
    parser.add_argument("--solver-root", help="canonical admitted Z3 directory; valid only with --solver-backend z3")
    args = parser.parse_args(argv)
    try:
        if SOURCE_BINDING is not None:
            require_cohort_binding(args.subject)
        # Reject invalid selection before creating evidence or admitting tools;
        # recompute under the frozen source bookends before actual execution.
        selected_jobs(args.subject, args.start_at, args.only_jobs)
        requested_solver = solver_selection(args.solver_backend, args.solver_root)
        partition_preflight(args.subject, args.tool_admission_profile, args.solver_backend,
                            args.solver_root, args.output_dir)
    except (Incomplete, ValueError) as error:
        parser.error(str(error))
    os.umask(0o077)
    work = fresh_output(args.output_dir)
    source = work / "source"
    summary = {"status": "incomplete", "subject": args.subject, "jobs": [],
               "maximum_jobs": job_limit(args.subject), "root": str(ROOT), "output": str(work),
               "tool_admission_profile": args.tool_admission_profile,
               "solver_backend": args.solver_backend, "requested_solver_root": requested_solver,
               "started_unix_ns": time.time_ns(), "retries": 0}
    started = time.monotonic()
    check = None
    records = None
    retained = []
    try:
        check = module("formal_analysis_admission", ROOT / "scripts/check-analysis.py")
        records = freeze_inputs(check, source, input_names(args.subject, args.solver_backend))
        require_sources(check, source, records, args.subject, args.solver_backend)
        write_json(work / "source-manifest.json", {"source_original_manifest": records,
                   "runner_sha256": records["tools/formal/verify.py"]["sha256"]})
        if args.subject in COHORT_SELECTORS:
            summary["executed_python_sources"] = SOURCE_BINDING.snapshot()
            COHORT_CAMPAIGN = {"cohort": require_cohort_binding(args.subject),
                               "work": work, "source": source,
                               "retained": retained, "next_index": 1}
        retain_artifacts(check, work, retained, [source, work / "source-manifest.json"])
        selection = selected_jobs(args.subject, args.start_at, args.only_jobs)
        fixed_count = len(jobs(args.subject))
        summary.update(planned_jobs=[job for _, job in selection],
                       selection_start_at=args.start_at,
                       selection_only_jobs=sorted(args.only_jobs) if args.only_jobs is not None else None,
                       complete_schedule=[index for index, _ in selection] == list(range(1, fixed_count + 1)),
                       fixed_schedule_jobs=fixed_count,
                       scope="selected proof commands only; omitted jobs require separate evidence")
        print(json.dumps({"status": "prepared", "jobs": len(selection),
                          "complete_schedule": summary["complete_schedule"], "output": str(work)}), flush=True)
        write_json(work / "summary.json", summary)
        for index, job in selection:
            solver_options = {} if args.solver_backend == "cadical" else {
                "solver_backend": args.solver_backend, "solver_root": requested_solver}
            result = execute_job(check, work, source, records, args.subject, job, index,
                                 tool_admission_profile=args.tool_admission_profile, retained=retained, **solver_options)
            compact = {"index": index, "subject": job["subject"], "case": job["case"],
                       "harness": job["harness"], "output": result["output"],
                       "qualified": result["expected_outcome_qualified"],
                       "exit_code": result.get("native", {}).get("exit_code"),
                       "checks": result.get("checks"), "admission_seconds": result.get("admission_seconds"),
                       "execute_seconds": result.get("execute_seconds"),
                       "solver_seconds": result.get("solver_seconds"),
                       "decision_procedure_seconds": result.get("decision_procedure_seconds"),
                       "timing_field": result.get("timing_field"), "timing_meaning": result.get("timing_meaning"),
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
                partition_remaining(args.subject, source)
                if args.subject in COHORT_SELECTORS:
                    if records is None:
                        raise Incomplete("No complete captured-cohort source manifest was frozen")
                    require_sources(check, source, records, args.subject, args.solver_backend)
                    cohort_resources(work)
                solver_options = {} if args.solver_backend == "cadical" else {
                    "solver_backend": args.solver_backend, "solver_root": requested_solver,
                    "source": source, "work": work}
                _, final = admission(check, work / "final-admission.json", profile=args.tool_admission_profile,
                                     **solver_options)
                summary["final_admission_seconds"] = final["seconds"]
                if records is None:
                    raise Incomplete("No complete original source manifest was frozen")
                require_sources(check, source, records, args.subject, args.solver_backend)
                # retain_artifacts audits every older anchored fact after final
                # admission, then anchors the new owner-written final receipt.
                retain_artifacts(check, work, retained, [work / "final-admission.json"])
                partition_remaining(args.subject, source)
                summary["original_and_frozen_inputs_unchanged"] = True
                if summary["status"] == "passed":
                    summary["all_proof_inputs_and_required_receipts_unchanged"] = True
            except Exception as error:
                summary["status"] = "incomplete"
                summary["final_error"] = f"{type(error).__name__}: {error}"
        if args.subject in COHORT_SELECTORS:
            try:
                if check is None:
                    raise Incomplete("No runtime custody owner available for target cleanup")
                summary["removed_disposable_targets"] = cleanup_cohort_targets(check, work)
                summary["retained_resources"] = cohort_resources(work)
                summary["executed_python_sources"] = SOURCE_BINDING.snapshot()
            except Exception as error:
                summary["status"] = "incomplete"
                summary["cleanup_error"] = f"{type(error).__name__}: {error}"
        summary["total_seconds"] = time.monotonic() - started
        write_json(work / "summary.json", summary)
    print(json.dumps({"status": summary["status"], "completed_jobs": len(summary["jobs"]),
                      "complete_schedule": summary.get("complete_schedule", False),
                      "summary": str(work / "summary.json"),
                      "error": summary.get("error"), "final_error": summary.get("final_error")}), flush=True)
    return 0 if summary["status"] == "passed" else 1


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, Incomplete) as error:
        print(f"Formal verification incomplete: {error}", file=sys.stderr)
        raise SystemExit(1)
