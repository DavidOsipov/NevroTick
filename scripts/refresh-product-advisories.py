#!/usr/bin/env python3
"""Acquire a fresh public RustSec/index snapshot for a separate frozen check.

Runs only pinned cargo-deny/Cargo metadata acquisition. It builds no packages,
executes no proc macros and never silently refreshes during product verification.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import time

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("advisory_environment", ROOT / "scripts/check-environment.py")
ENV = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENV)


def inventory(directory: Path) -> dict[str, str]:
    """Record all regular snapshot bytes, rejecting linked/special paths."""
    result = {}
    for path in sorted(directory.rglob("*")):
        if path.is_symlink() or not (path.is_file() or path.is_dir()):
            raise ValueError("Unexpected linked or special acquisition output")
        if path.is_file():
            result[str(path.relative_to(directory))] = hashlib.sha256(path.read_bytes()).hexdigest()
    return result


def main() -> int:
    """Create one new snapshot and receipt without replacing prior evidence."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cargo-home", type=Path, required=True)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--deny", type=Path)
    parser.add_argument("--toolchain-dir", type=Path)
    args = parser.parse_args()
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
        if output.exists() or output.is_relative_to(ROOT) or home.is_relative_to(ROOT) or home.exists():
            raise ValueError("Snapshot and receipt must be new external destinations")
        lock = ENV.digest(ROOT / "Cargo.lock")
        home.mkdir(parents=True, mode=0o700)
        environment = {"PATH": f"{toolchain / 'bin'}:/usr/bin:/bin", "CARGO_HOME": str(home),
                       "RUSTC": str(toolchain / "bin/rustc"), "LANG": "C.UTF-8"}
        command = [str(deny), "--locked", "fetch", "db", "index"]
        result = ENV.run(command, cwd=ROOT, environment=environment, timeout=300)
        ENV.require_ok(result)
        acquired = time.time()
        if ENV.digest(ROOT / "Cargo.lock") != lock:
            raise ValueError("Lockfile changed during metadata acquisition")
        files = inventory(home)
        if not any(path.startswith("advisory-dbs/") for path in files) or not any(path.startswith("registry/index/") for path in files):
            raise ValueError("Acquisition omitted the RustSec database or registry index")
        output.parent.mkdir(parents=True, exist_ok=True)
        record = {"schema_version": 1, "acquired_unix": acquired, "cargo_home": str(home),
                  "files": files, "lockfile_sha256": lock, "command": command,
                  "exit_code": result["exit_code"], "output": result["output"],
                  "tool_sha256": ENV.digest(deny),
                  "scope": "Public metadata acquisition only; no application/advisory check executed"}
        with os.fdopen(os.open(output, os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600), "w") as stream:
            json.dump(record, stream, indent=2)
        print(f"Acquired and recorded {len(files)} metadata files; run the frozen dependency row next.")
        return 0
    except (OSError, ValueError, KeyError, StopIteration, ENV.Incomplete) as error:
        print(f"INCOMPLETE: metadata acquisition: {error}")
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
