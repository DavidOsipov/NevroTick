#!/usr/bin/env python3
"""Acquire a fresh public RustSec/index snapshot for a separate frozen check.

Runs only pinned cargo-deny/Cargo metadata acquisition. It builds no packages,
executes no proc macros and never silently refreshes during product verification.
"""

from __future__ import annotations

import argparse
import fcntl
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("advisory_environment", ROOT / "scripts/check-environment.py")
ENV = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENV)
SNAPSHOT_SPEC = importlib.util.spec_from_file_location("advisory_snapshot", ROOT / "scripts/advisory_snapshot.py")
SNAPSHOT = importlib.util.module_from_spec(SNAPSHOT_SPEC)
SNAPSHOT_SPEC.loader.exec_module(SNAPSHOT)


def inventory(directory: Path) -> dict[str, str]:
    """Record a bounded regular snapshot through the shared custody owner."""
    return SNAPSHOT.inventory(directory)


def main() -> int:
    """Create one new snapshot and receipt without replacing prior evidence."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cargo-home", type=Path, required=True)
    parser.add_argument("--acquisition-root", type=Path, required=True,
                        help="Existing private empty dedicated filesystem, at most 512 MiB and 16,384 inodes")
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--deny", type=Path)
    parser.add_argument("--toolchain-dir", type=Path)
    args = parser.parse_args()
    quota_descriptor = None
    created_home = created_receipt = None
    accepted = False
    try:
        policy = json.loads((ROOT / "policy/tool-manifest.json").read_text())
        deny = (args.deny or Path(shutil.which("cargo-deny") or "/missing-cargo-deny")).resolve(strict=True)
        deny_policy = next(item for item in policy["tools"] if item["name"] == "cargo-deny")
        ENV.require_admitted_binary(deny, deny_policy["binary_sha256"], "cargo-deny")
        if args.toolchain_dir:
            toolchain = args.toolchain_dir.resolve(strict=True)
        else:
            manager = Path(shutil.which("rustup") or "/missing-rustup").resolve(strict=True)
            ENV.require_admitted_binary(manager, policy["rust"]["rustup_linux_sha256"], "rustup")
            toolchain = Path(ENV.require_ok(ENV.run(
                [str(manager), "which", "--toolchain", policy["rust"]["channel"], "rustc"], cwd=ROOT))).parent.parent
        for name in ("rustc", "cargo"):
            ENV.require_admitted_binary(toolchain / "bin" / name, policy["rust"]["linux_binary_sha256"][name], name)
        output = args.receipt.expanduser().resolve()
        home = args.cargo_home.expanduser().resolve()
        quota = args.acquisition_root.expanduser().absolute()
        if output.exists() or output.is_relative_to(ROOT) or home.is_relative_to(ROOT) or home.exists():
            raise ValueError("Snapshot and receipt must be new external destinations")
        if (quota.is_relative_to(ROOT) or home.is_relative_to(quota) or output.is_relative_to(quota)
                or output.is_relative_to(home) or home.is_relative_to(output)):
            raise ValueError("Quota work, exported snapshot and receipt must have separate external custody")
        SNAPSHOT.regular_root(quota)
        quota_descriptor = os.open(quota, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC)
        fcntl.flock(quota_descriptor, fcntl.LOCK_EX | fcntl.LOCK_NB)
        capacity = SNAPSHOT.quota_root(quota)
        if SNAPSHOT.stamp(os.fstat(quota_descriptor)) != SNAPSHOT.stamp(quota.lstat()):
            raise ValueError("Acquisition root changed before work admission")
        lock = ENV.digest(ROOT / "Cargo.lock")
        # cargo-deny's `fetch index` invokes `cargo fetch`, including source
        # downloads. Resolve only registry metadata, then fetch RustSec alone.
        planned = (
            ("registry-index", [str(toolchain / "bin/cargo"), "update", "--workspace", "--locked"]),
            ("advisory-database", [str(deny), "--locked", "fetch", "db"]),
        )
        commands = []
        with tempfile.TemporaryDirectory(prefix="advisory-", dir=quota) as folder:
            work = Path(folder)
            transient, temporary = work / "cargo-home", work / "tmp"
            transient.mkdir(mode=0o700)
            temporary.mkdir(mode=0o700)
            environment = {"PATH": f"{toolchain / 'bin'}:/usr/bin:/bin", "CARGO_HOME": str(transient),
                           "RUSTC": str(toolchain / "bin/rustc"), "LANG": "C.UTF-8", "TMPDIR": str(temporary)}
            for stage, command in planned:
                if ENV.digest(ROOT / "Cargo.lock") != lock:
                    raise ValueError("Lockfile changed before metadata acquisition stage")
                started = time.time()
                result = SNAPSHOT.run(command, cwd=ROOT, environment=environment)
                commands.append({**result, "stage": stage, "started_unix": started,
                                 "finished_unix": time.time()})
                ENV.require_ok(result)
                if ENV.digest(ROOT / "Cargo.lock") != lock:
                    raise ValueError("Lockfile changed during metadata acquisition")
                files = inventory(transient)
                if any(path.startswith(("registry/src/", "registry/cache/")) for path in files):
                    raise ValueError("Metadata acquisition produced a crate source or archive payload")
                if not any(path.startswith("registry/index/") for path in files):
                    raise ValueError("Acquisition omitted the registry index")
                if stage == "advisory-database" and not any(path.startswith("advisory-dbs/") for path in files):
                    raise ValueError("Acquisition omitted the RustSec database")
            home.mkdir(parents=True, mode=0o700)
            created_home = home.lstat()
            SNAPSHOT.export(transient, home, files)
        output.parent.mkdir(parents=True, exist_ok=True)
        record = {"schema_version": 1, "acquired_unix": commands[0]["started_unix"],
                  "completed_unix": commands[-1]["finished_unix"], "commands": commands,
                  "cargo_home": str(home),
                  "files": files, "lockfile_sha256": lock, "command": command,
                  "exit_code": result["exit_code"], "output": result["output"],
                  "tool_sha256": ENV.digest(deny),
                  "cargo_sha256": ENV.digest(toolchain / "bin/cargo"),
                  "acquisition_filesystem": capacity,
                  "scope": "Public metadata acquisition only; no application/advisory check executed"}
        payload = json.dumps(record, indent=2).encode("utf-8")
        if len(payload) > SNAPSHOT.MAX_RECEIPT_BYTES:
            raise ValueError("Metadata receipt exceeds its byte bound")
        descriptor = os.open(output, os.O_CREAT | os.O_EXCL | os.O_WRONLY | os.O_NOFOLLOW, 0o600)
        created_receipt = os.fstat(descriptor)
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(payload)
        if ENV.digest(ROOT / "Cargo.lock") != lock:
            raise ValueError("Lockfile changed during metadata export")
        accepted = True
        print(f"Acquired and recorded {len(files)} metadata files; run the frozen dependency row next.")
        return 0
    except (OSError, ValueError, RuntimeError, KeyError, StopIteration, subprocess.SubprocessError, ENV.Incomplete) as error:
        # Context-manager cleanup can fail after a fetch/validation refusal.
        # Preserve that first cause instead of replacing it with cleanup alone.
        if error.__context__ is not None:
            print(f"INCOMPLETE: earlier acquisition failure: {str(error.__context__)[:3000]}")
        print(f"INCOMPLETE: metadata acquisition: {error}")
        return 1
    finally:
        try:
            if not accepted:
                if created_receipt is not None and output.exists():
                    current = output.lstat()
                    if (current.st_dev, current.st_ino) == (created_receipt.st_dev, created_receipt.st_ino):
                        output.unlink()
                if created_home is not None and home.exists():
                    current = home.lstat()
                    if (current.st_dev, current.st_ino) == (created_home.st_dev, created_home.st_ino):
                        shutil.rmtree(home)
        finally:
            if quota_descriptor is not None:
                os.close(quota_descriptor)


if __name__ == "__main__":
    raise SystemExit(main())
