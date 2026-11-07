#!/usr/bin/env python3
"""Run the exact portable service producer and preserve its raw output in-repo.

Execution stays in the existing private persistent cache: Common's external-work
guard is unchanged. The archive records original paths; copied reports are never
rewritten. Full LLVM coverage targets and every profile are evidence. Only the
literal ordinary Cargo target caches below are omitted. No native/formal lane.
GPL-2.0-only.
"""
from __future__ import annotations

import sys

if sys.flags.optimize:
    raise SystemExit("Service analysis export refuses optimized Python.")

import argparse
import fcntl
import hashlib
import importlib.util
import json
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import stat
import subprocess
import time

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
RECORDS = ROOT / "artifacts/windows-services/recovery-20261107-v1"
CACHE = Path.home() / ".local/share/nevrotick/service-verification-cache-20261107-v1"
PRODUCER = ROOT / "scripts/verify-service-analysis.py"
LANES = ("enumerate", "coverage", "mutation", "all")
DISK_FLOOR = 2 * 1024 ** 3
MAX_FILE = 268435456  # Existing runtime's exact RLIMIT_FSIZE, including binaries.
MAX_ENTRIES = 200000
BOUND_FILES = ("scripts/run-service-analysis.py", "scripts/verify-service-analysis.py",
               "scripts/portable-analysis.py", "scripts/check-analysis.py",
               "scripts/analysis-runtime.py", "policy/service-analysis.json")
RULES = {
    "retain": ["work/target/llvm-cov-target/**", "**/*.profdata", "**/*.profraw",
               "all other regular producer files except the literal caches below"],
    "derived_only": ["work/target/** except retained LLVM target/profiles",
                     "work/tmp/cargo-mutants-<worker>.tmp/target/** except profiles"],
    "original_paths_rewritten": False,
}


def require(condition, message):
    """Refuse an incomplete observation without relying on Python assertions."""
    if not condition:
        raise ValueError(message)


def identity(item):
    """Capture attachment and byte-change metadata, allowing internal hardlinks."""
    return (item.st_dev, item.st_ino, item.st_mode, item.st_uid, item.st_gid,
            item.st_nlink, item.st_size, item.st_mtime_ns, item.st_ctime_ns)


def write_new(path, value):
    """Create a new raw record without replacing previous evidence."""
    with path.open("x", encoding="utf-8") as stream:
        json.dump(value, stream, indent=2, sort_keys=True)
        stream.write("\n")


def directory(path):
    """Require a canonical owned directory with no linked path component."""
    require(path.is_absolute() and path.resolve() == path, "Noncanonical directory: " + str(path))
    for parent in (path, *path.parents):
        require(not parent.is_symlink(), "Linked directory: " + str(parent))
    item = path.stat()
    require(stat.S_ISDIR(item.st_mode) and item.st_uid == os.geteuid(),
            "Directory is not owned: " + str(path))


def retained(relative):
    """Select evidence before applying the two literal regenerable-cache rules."""
    parts = PurePosixPath(relative).parts
    if relative.endswith((".profraw", ".profdata")):
        return True
    if parts[:3] == ("work", "target", "llvm-cov-target"):
        return True
    if parts[:2] == ("work", "target"):
        return False
    if (len(parts) >= 4 and parts[:2] == ("work", "tmp")
            and re.fullmatch(r"cargo-mutants-[A-Za-z0-9_.-]+\.tmp", parts[2])
            and parts[3] == "target"):
        return False
    return True


def inventory(origin):
    """Snapshot the complete bounded tree, refusing special/linked entries."""
    directory(origin)
    result = {}
    for parent, directories, files in os.walk(origin, followlinks=False):
        for name in sorted(directories + files):
            path = Path(parent) / name
            item = path.lstat()
            require(item.st_uid == os.geteuid() and
                    (stat.S_ISREG(item.st_mode) or stat.S_ISDIR(item.st_mode)),
                    "Unowned, linked or special producer entry: " + str(path))
            relative = path.relative_to(origin).as_posix()
            result[relative] = identity(item)
            require(len(result) <= MAX_ENTRIES, "Producer tree exceeds entry bound.")
    return result


def copy_regular(source, destination, expected):
    """Stream one bounded regular file through pinned nonlinked ancestors.

    Hardlinked Cargo executables become independent archive files; link count and
    inode custody remain recorded. All leaf and directory attachments are checked
    again after copying. The original producer output remains untouched.
    """
    descriptors, bindings = [], []
    flags = os.O_RDONLY | os.O_CLOEXEC | os.O_NOFOLLOW | os.O_NONBLOCK
    try:
        descriptors.append(os.open("/", flags | os.O_DIRECTORY))
        for component in source.parts[1:-1]:
            parent = descriptors[-1]
            descriptor = os.open(component, flags | os.O_DIRECTORY, dir_fd=parent)
            descriptors.append(descriptor)
            bindings.append((parent, component, os.fstat(descriptor)))
        parent = descriptors[-1]
        descriptor = os.open(source.name, flags, dir_fd=parent)
        descriptors.append(descriptor)
        before = os.fstat(descriptor)
        require(identity(before) == expected and stat.S_ISREG(before.st_mode)
                and before.st_size <= MAX_FILE, "Producer file changed or exceeds byte bound: " + str(source))
        destination.parent.mkdir(parents=True, exist_ok=True)
        digest, size = hashlib.sha256(), 0
        with destination.open("xb") as stream:
            while chunk := os.read(descriptor, 1024 * 1024):
                size += len(chunk)
                require(size <= MAX_FILE, "Producer file grew past byte bound.")
                digest.update(chunk)
                stream.write(chunk)
        after = os.fstat(descriptor)
        require(size == before.st_size and identity(after) == expected
                and identity(os.stat(source.name, dir_fd=parent, follow_symlinks=False)) == expected,
                "Producer file changed during export: " + str(source))
        for parent_fd, name, opened in bindings:
            attached = os.stat(name, dir_fd=parent_fd, follow_symlinks=False)
            require(stat.S_ISDIR(attached.st_mode) and
                    (attached.st_dev, attached.st_ino) == (opened.st_dev, opened.st_ino),
                    "Producer directory changed during export.")
        with destination.open("rb") as stream:
            copied_hash = hashlib.file_digest(stream, "sha256").hexdigest()
        require(copied_hash == digest.hexdigest(), "Archive bytes differ from producer bytes.")
        return {"source": str(source), "archive": str(destination), "bytes": size,
                "sha256": copied_hash, "source_stat": list(expected)}
    finally:
        for descriptor in reversed(descriptors):
            os.close(descriptor)


def export_tree(origin, archive, record):
    """Retain every selected file and a complete selection/byte hash ledger."""
    before = inventory(origin)
    selected = {name: item for name, item in before.items()
                if stat.S_ISREG(item[2]) and retained(name)}
    required_bytes = sum(item[6] for item in selected.values())
    # Persist a complete planned ledger first; retain completed rows on any failure.
    write_new(record / "export-plan.json", {"rules": RULES, "origin": str(origin),
              "archive": str(archive), "inventory": {k: list(v) for k, v in before.items()},
              "selected": sorted(selected), "selected_bytes": required_bytes})
    require(shutil.disk_usage(RECORDS).free >= DISK_FLOOR + required_bytes,
            "Insufficient disk to export evidence while retaining the 2 GiB floor.")
    archive.mkdir(mode=0o700)
    for name, item in sorted(before.items()):
        if stat.S_ISDIR(item[2]) and retained(name):
            (archive / name).mkdir(parents=True, exist_ok=True)
    copied = {}
    with (record / "export-progress.jsonl").open("x", encoding="utf-8") as progress:
        for name, expected in sorted(selected.items()):
            require(shutil.disk_usage(RECORDS).free >= DISK_FLOOR + expected[6],
                    "Disk headroom fell below the evidence export floor.")
            row = copy_regular(origin / name, archive / name, expected)
            copied[name] = row
            progress.write(json.dumps({"relative": name, **row}, sort_keys=True) + "\n")
            progress.flush()
    require(inventory(origin) == before, "Producer tree changed during complete export.")
    archived = inventory(archive)
    require({k for k, v in archived.items() if stat.S_ISREG(v[2])} == set(selected),
            "Archive inventory differs from selected producer files.")
    write_new(record / "export-map.json", {"complete": True, "rules": RULES,
              "files": copied, "excluded_files": sorted(k for k, v in before.items()
              if stat.S_ISREG(v[2]) and k not in selected), "bytes": required_bytes})
    return copied


def index_identity():
    """Observe staging without changing it; each new invocation owns its epoch."""
    result = subprocess.run(["git", "rev-parse", "--git-path", "index"], cwd=ROOT,
                            env={"PATH": "/usr/bin:/bin", "GIT_OPTIONAL_LOCKS": "0"},
                            capture_output=True, text=True, check=True, timeout=10)
    path = Path(result.stdout.strip())
    if not path.is_absolute():
        path = ROOT / path
    return {"path": str(path), "sha256": hashlib.sha256(path.read_bytes()).hexdigest()}


def bindings(common):
    """Use the existing bounded custody reader for frontend/producer inputs."""
    return {name: hashlib.sha256(common.read_analysis_bytes(ROOT / name, maximum=16 * 1024 * 1024)).hexdigest()
            for name in BOUND_FILES}


def main(argv=None):
    """Invoke only the unchanged literal producer and archive successful or failed runs."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", required=True, choices=LANES)
    parser.add_argument("--label", required=True)
    parser.add_argument("--vendor-dir", required=True, type=Path)
    args = parser.parse_args(argv)
    require(re.fullmatch(r"[a-z0-9][a-z0-9-]{0,63}", args.label), "Invalid fresh record label.")
    require(not any(key in {"RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTDOCFLAGS", "CARGO_ENCODED_RUSTDOCFLAGS"}
                    or (key.startswith("CARGO_TARGET_") and key.endswith(("_RUSTFLAGS", "_RUSTDOCFLAGS")))
                    for key in os.environ), "Ambient compiler flag overrides are forbidden.")
    require(args.vendor_dir.is_absolute() and args.vendor_dir.resolve() == args.vendor_dir,
            "Select an explicit canonical vendor directory.")
    for path in (RECORDS, CACHE):
        path.mkdir(parents=True, exist_ok=True)
        directory(path)
    require(shutil.disk_usage(CACHE).free >= DISK_FLOOR, "Less than 2 GiB execution disk headroom.")
    writer = os.open(CACHE / "writer.lock", os.O_CREAT | os.O_RDWR | os.O_CLOEXEC | os.O_NOFOLLOW, 0o600)
    try:
        item = os.fstat(writer)
        require(stat.S_ISREG(item.st_mode) and item.st_uid == os.geteuid() and item.st_nlink == 1,
                "Writer lock is not an owned regular file.")
        fcntl.flock(writer, fcntl.LOCK_EX | fcntl.LOCK_NB)
        record, output = RECORDS / ("analysis-" + args.label), CACHE / ("analysis-" + args.label)
        require(not record.exists() and not output.exists(), "Label already owns records or execution output.")
        record.mkdir(mode=0o700)
        receipt = {"status": "incomplete", "qualification_passed": False, "lane": args.lane,
                   "record": str(record), "execution_output": str(output), "rules": RULES,
                   "started_ns": time.time_ns(), "native_execution": "NOTRUN", "formal_execution": "NOTRUN"}
        command = [sys.executable, "-B", str(PRODUCER), "--lane", args.lane,
                   "--output-dir", str(output), "--vendor-dir", str(args.vendor_dir)]
        receipt["command"] = command
        write_new(record / "invocation.json", receipt)
        common, before, epoch, result = None, None, None, None
        errors = []
        try:
            spec = importlib.util.spec_from_file_location("service_export_common", ROOT / "scripts/portable-analysis.py")
            common = importlib.util.module_from_spec(spec)
            sys.modules[spec.name] = common
            spec.loader.exec_module(common)
            before, epoch = bindings(common), index_identity()
            receipt.update(input_hashes_before=before, index_before=epoch)
            write_new(record / "admission.json", {"input_hashes": before, "index": epoch})
            with (record / "producer.stdout").open("xb") as stdout, (record / "producer.stderr").open("xb") as stderr:
                result = subprocess.run(command, cwd=ROOT, stdout=stdout, stderr=stderr, check=False)
            receipt["producer_exit_code"] = result.returncode
        except Exception as error:
            errors.append(type(error).__name__ + ": " + str(error))
        # Export partial/failed producer output even when frontend bookends fail.
        try:
            if before is not None:
                receipt["input_hashes_after"] = bindings(common)
                receipt["index_after"] = index_identity()
                receipt["frontend_bookends_passed"] = (receipt["input_hashes_after"] == before
                                                         and receipt["index_after"] == epoch)
                require(receipt["frontend_bookends_passed"], "Frontend input or index custody changed during producer job.")
        except Exception as error:
            errors.append(type(error).__name__ + ": " + str(error))
        try:
            if output.exists():
                exported = export_tree(output, record / "raw", record)
                receipt["export_complete"] = True
                receipt["exported_files"] = len(exported)
                report = common.read_analysis_json(record / "raw/verification.json")
                receipt["producer_status"] = report.get("status")
                receipt["producer_metrics_passed"] = report.get("metrics_passed")
                receipt["export_map_sha256"] = hashlib.sha256((record / "export-map.json").read_bytes()).hexdigest()
                if (not errors and result is not None and result.returncode == 0
                        and report.get("status") == "passed" and receipt.get("frontend_bookends_passed")):
                    receipt["status"] = "passed"
        except Exception as error:
            errors.append(type(error).__name__ + ": " + str(error))
        receipt["errors"] = errors
        receipt["record_hashes"] = {path.name: hashlib.sha256(path.read_bytes()).hexdigest()
                                    for path in record.iterdir() if path.is_file()}
        receipt["finished_ns"] = time.time_ns()
        write_new(record / "verification.json", receipt)
        print(receipt["status"].upper() + ": " + str(record))
        return 0 if receipt["status"] == "passed" else 1
    finally:
        os.close(writer)


if __name__ == "__main__":
    raise SystemExit(main())
