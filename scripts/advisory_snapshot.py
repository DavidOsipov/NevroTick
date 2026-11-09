# SPDX-License-Identifier: GPL-2.0-only
"""Finite public-metadata custody shared by acquisition and frozen admission.

The caller provisions and owns the quota filesystem; this module never mounts,
escalates privileges, fetches metadata or changes a filesystem quota. Directory
and descriptor bookends detect observed mutation, not an adversarial transaction.
"""

from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import resource
import selectors
import signal
import stat
import subprocess
import time


ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location(
    "advisory_input_custody", ROOT / "tools/crypto-aes-siv-witness/fuzz/input_custody.py")
CUSTODY = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CUSTODY)

MAX_TOTAL_BYTES = 512 * 1024 * 1024
MAX_FILE_BYTES = 64 * 1024 * 1024
MAX_ENTRIES = 16_384
MAX_DEPTH = 16
MAX_PATH_BYTES = 1024
MAX_RECEIPT_BYTES = 8 * 1024 * 1024
MAX_OUTPUT_BYTES = 2 * 1024 * 1024
MAX_ADDRESS_SPACE = 1024 * 1024 * 1024
MAX_SECONDS = 300


def stamp(info: os.stat_result) -> tuple[int, ...]:
    """Bind inode type, ownership, links, size and mutation timestamps."""
    return tuple(getattr(info, name) for name in (
        "st_dev", "st_ino", "st_mode", "st_uid", "st_gid", "st_nlink",
        "st_size", "st_mtime_ns", "st_ctime_ns"))


def regular_root(directory: Path) -> os.stat_result:
    """Require an absolute canonical directory without following a final link."""
    if not directory.is_absolute() or directory.resolve(strict=True) != directory:
        raise ValueError("Metadata root must be a canonical directory")
    observed = directory.lstat()
    if not stat.S_ISDIR(observed.st_mode):
        raise ValueError("Metadata root must be a regular directory")
    return observed


def quota_root(directory: Path) -> dict[str, int]:
    """Admit a private empty dedicated filesystem by total bytes and inodes.

    Free space alone is insufficient: the writer must have a hard finite backing
    capacity even if it runs between controller observations. The caller keeps
    exclusive ownership of this mount until acquisition and export terminate.
    """
    before = regular_root(directory)
    if (before.st_uid != os.getuid() or stat.S_IMODE(before.st_mode) != 0o700
            or before.st_dev == directory.parent.stat().st_dev):
        raise ValueError("Acquisition root must be a private dedicated quota filesystem")
    capacity = os.statvfs(directory)
    byte_capacity = capacity.f_frsize * capacity.f_blocks
    if (not 0 < byte_capacity <= MAX_TOTAL_BYTES
            or not 0 < capacity.f_files <= MAX_ENTRIES):
        raise ValueError("Acquisition filesystem exceeds its byte or inode capacity")
    with os.scandir(directory) as children:
        if next(children, None) is not None:
            raise ValueError("Acquisition quota filesystem must be empty")
    if stamp(directory.lstat()) != stamp(before):
        raise ValueError("Acquisition root changed during quota admission")
    return {"bytes": byte_capacity, "inodes": capacity.f_files, "device": before.st_dev}


def scan(directory: Path) -> dict[str, os.stat_result]:
    """Reserve all namespace and regular-file bytes before reading any body."""
    root_before = regular_root(directory)
    flags = os.O_RDONLY | os.O_CLOEXEC | os.O_NOFOLLOW | os.O_DIRECTORY
    descriptor = os.open(directory, flags)
    entries, total = 0, 0
    result = {}

    def visit(folder: int, prefix: tuple[str, ...]) -> None:
        nonlocal entries, total
        with os.scandir(folder) as children:
            for child in children:
                entries += 1
                if entries > MAX_ENTRIES:
                    raise ValueError("Metadata snapshot exceeds its entry bound")
                parts = (*prefix, child.name)
                if len(parts) > MAX_DEPTH:
                    raise ValueError("Metadata snapshot exceeds its depth bound")
                relative = "/".join(parts)
                if len(relative.encode("utf-8")) > MAX_PATH_BYTES:
                    raise ValueError("Metadata snapshot exceeds its path byte bound")
                observed = child.stat(follow_symlinks=False)
                if observed.st_dev != root_before.st_dev:
                    raise ValueError("Metadata snapshot crosses its admitted filesystem")
                if stat.S_ISDIR(observed.st_mode):
                    nested = os.open(child.name, flags, dir_fd=folder)
                    try:
                        if stamp(os.fstat(nested)) != stamp(observed):
                            raise ValueError("Metadata directory changed before traversal")
                        visit(nested, parts)
                    finally:
                        os.close(nested)
                elif stat.S_ISREG(observed.st_mode):
                    total += observed.st_size
                    if not 0 <= observed.st_size <= MAX_FILE_BYTES or total > MAX_TOTAL_BYTES:
                        raise ValueError("Metadata snapshot exceeds its file or aggregate byte bound")
                    result[relative] = observed
                else:
                    raise ValueError("Unexpected linked or special acquisition output")

    try:
        if stamp(os.fstat(descriptor)) != stamp(root_before):
            raise ValueError("Metadata root changed before traversal")
        visit(descriptor, ())
        if stamp(os.fstat(descriptor)) != stamp(root_before):
            raise ValueError("Metadata root changed during traversal")
    finally:
        os.close(descriptor)
    if stamp(directory.lstat()) != stamp(root_before):
        raise ValueError("Metadata root changed during traversal")
    return result


def inventory(directory: Path) -> dict[str, str]:
    """Hash an admitted finite stable snapshot using the existing custody reader."""
    observed = scan(directory)
    result = {}
    try:
        for relative, before in sorted(observed.items()):
            path = directory / relative
            if stamp(path.lstat()) != stamp(before):
                raise ValueError("Metadata snapshot changed after bounded preflight")
            result[relative] = CUSTODY.digest(path, maximum=before.st_size)
    except RuntimeError as error:
        raise ValueError("Metadata input custody changed during hashing") from error
    after = scan(directory)
    if ({name: stamp(value) for name, value in observed.items()}
            != {name: stamp(value) for name, value in after.items()}):
        raise ValueError("Metadata snapshot changed during hashing")
    return result


def read_receipt(path: Path) -> dict:
    """Bound raw receipt bytes before strict JSON decoding and file-map admission."""
    def unique(pairs):
        result = {}
        for name, value in pairs:
            if name in result:
                raise ValueError("Duplicate metadata receipt field")
            result[name] = value
        return result

    try:
        payload = CUSTODY.read_file(path, maximum=MAX_RECEIPT_BYTES)
        result = json.loads(payload, object_pairs_hook=unique,
                            parse_constant=lambda _: (_ for _ in ()).throw(
                                ValueError("Nonfinite metadata receipt number")))
    except (RuntimeError, RecursionError) as error:
        raise ValueError("Metadata receipt exceeds its byte bound or input custody") from error
    if type(result) is not dict or type(result.get("files")) is not dict:
        raise ValueError("Metadata receipt is missing its finite file map")
    files = result["files"]
    if not 0 < len(files) <= MAX_ENTRIES:
        raise ValueError("Metadata receipt exceeds its file-map entry bound")
    for name, value in files.items():
        if (type(name) is not str or not name or name.startswith("/") or "\\" in name
                or any(part in ("", ".", "..") for part in name.split("/"))
                or len(name.split("/")) > MAX_DEPTH or len(name.encode()) > MAX_PATH_BYTES
                or type(value) is not str or len(value) != 64
                or any(character not in "0123456789abcdef" for character in value)):
            raise ValueError("Metadata receipt has an invalid bounded path or digest")
    return result


def receipt_digest(path: Path) -> str:
    """Hash only a bounded unchanged receipt extent, including at bookends."""
    try:
        return CUSTODY.digest(path, maximum=MAX_RECEIPT_BYTES)
    except RuntimeError as error:
        raise ValueError("Metadata receipt exceeds its byte bound or input custody") from error


def export(source: Path, destination: Path, expected: dict[str, str]) -> None:
    """Copy only reserved stable bytes into a new caller-owned final directory.

    The caller owns rollback of that new directory. A failed or changed input
    cannot authorize an unbounded copy, and a receipt is published only after
    this export and both complete inventory comparisons finish.
    """
    observed = scan(source)
    if set(observed) != set(expected):
        raise ValueError("Metadata export source namespace changed")
    for relative, before in sorted(observed.items()):
        try:
            data = CUSTODY.read_file(source / relative, maximum=before.st_size)
        except RuntimeError as error:
            raise ValueError("Metadata export input custody changed") from error
        if hashlib.sha256(data).hexdigest() != expected[relative]:
            raise ValueError("Metadata export source bytes changed")
        output = destination / relative
        output.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
        descriptor = os.open(output, os.O_CREAT | os.O_EXCL | os.O_WRONLY | os.O_NOFOLLOW, 0o600)
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(data)
    if inventory(source) != expected or inventory(destination) != expected:
        raise ValueError("Metadata snapshot changed during bounded export")


def run(command: list[str], *, cwd: Path, environment: dict[str, str]) -> dict:
    """Bound an owned fetch group, output and per-process resources; always reap.

    The separately admitted quota filesystem bounds aggregate transient writes.
    Process-group custody covers the pinned fetch's non-detached descendants;
    escaped sessions are outside this helper's guarantee.
    """
    def limits():
        resource.setrlimit(resource.RLIMIT_FSIZE, (MAX_FILE_BYTES, MAX_FILE_BYTES))
        resource.setrlimit(resource.RLIMIT_AS, (MAX_ADDRESS_SPACE, MAX_ADDRESS_SPACE))
        resource.setrlimit(resource.RLIMIT_NOFILE, (128, 128))
        resource.setrlimit(resource.RLIMIT_CPU, (MAX_SECONDS, MAX_SECONDS))

    process = subprocess.Popen(command, cwd=cwd, env=environment, stdout=subprocess.PIPE,
                               stderr=subprocess.STDOUT, start_new_session=True, preexec_fn=limits)
    output, failure = bytearray(), None
    try:
        with selectors.DefaultSelector() as selector:
            selector.register(process.stdout, selectors.EVENT_READ)
            deadline = time.monotonic() + MAX_SECONDS
            while selector.get_map() or process.poll() is None:
                remaining = deadline - time.monotonic()
                if remaining <= 0:
                    failure = "wall timeout"
                    break
                for key, _ in selector.select(min(0.1, remaining)):
                    chunk = os.read(key.fileobj.fileno(), 65536)
                    if not chunk:
                        selector.unregister(key.fileobj)
                    elif len(output) + len(chunk) > MAX_OUTPUT_BYTES:
                        failure = "output budget exceeded"
                        break
                    else:
                        output.extend(chunk)
                if failure:
                    break
        if failure is None:
            process.wait(timeout=5)
            try:
                os.killpg(process.pid, 0)
            except ProcessLookupError:
                pass
            else:
                failure = "descendant processes outlived command"
    finally:
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        process.wait(timeout=5)
        process.stdout.close()
    return {"command": command, "exit_code": process.returncode,
            "output": output.decode("utf-8", errors="replace"), "failure": failure}
