#!/usr/bin/python3
"""Apply PC-guard coverage only to the admitted portable product crates.

The isolated build supplies a read-only source snapshot/toolchain and an owned
``/work`` log. This wrapper rejects compiler substitution and hidden coverage
flags; it never enables instrumentation for the engine or independent oracle.
"""

from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path
import stat
import sys


ROOT = Path(__file__).resolve().parents[2]
SUBJECTS = frozenset({"ntp_exchange", "ntp_wire", "time_domain"})
# The admitted GNU toolchain uses bundled LLD. Bound each linker's threads so
# twelve Cargo jobs remain inside the runner's unchanged 128-task allowance.
BASE_FLAGS = ("-Clink-arg=-Wl,--threads=1",)
FLAGS = (
    "-Cpasses=sancov-module",
    "-Cllvm-args=-sanitizer-coverage-level=3",
    "-Cllvm-args=-sanitizer-coverage-trace-pc-guard",
    "-Ccodegen-units=1",
)


def digest(path: Path) -> str:
    """Hash current regular compiler bytes without metadata-based caching."""
    value = hashlib.sha256()
    with path.open("rb") as source:
        for block in iter(lambda: source.read(1024 * 1024), b""):
            value.update(block)
    return value.hexdigest()


def record(crate: str | None, arguments: list[str], instrumented: bool) -> None:
    """Append one bounded, redacted build observation to owned namespace work."""
    path = Path(os.environ["STATE_GUIDANCE_WRAPPER_LOG"])
    if (not path.is_absolute() or not path.is_relative_to("/work")
            or path.parent != path.parent.resolve(strict=True)):
        raise ValueError("Wrapper log must be in canonical owned namespace work.")
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_APPEND | os.O_NOFOLLOW, 0o600)
    with os.fdopen(descriptor, "a", encoding="utf-8") as output:
        details = os.fstat(output.fileno())
        if (not stat.S_ISREG(details.st_mode) or details.st_uid != os.geteuid()
                or details.st_nlink != 1 or stat.S_IMODE(details.st_mode) != 0o600):
            raise ValueError("Wrapper log is not a private owned regular file.")
        output.write(json.dumps({"crate": crate, "instrumented": instrumented,
                                 "injected_flags": [*BASE_FLAGS, *(FLAGS if instrumented else ())],
                                 "argument_sha256": hashlib.sha256(
                                     json.dumps(arguments).encode()).hexdigest()}) + "\n")


def main() -> None:
    """Validate compiler/argument custody, record scope and replace this process."""
    if len(sys.argv) < 2:
        raise ValueError("Expected Cargo's explicit compiler path.")
    compiler, arguments = Path(sys.argv[1]), sys.argv[2:]
    policy = json.loads((ROOT / "policy/tool-manifest.json").read_text())["rust"]
    expected_parent = f"{policy['channel']}-{policy['host']}"
    if (not compiler.is_absolute() or compiler != compiler.resolve(strict=True)
            or compiler.name != "rustc" or compiler.parent.name != "bin"
            or compiler.parent.parent.name != expected_parent
            or digest(compiler) != policy["linux_binary_sha256"]["rustc"]):
        raise ValueError("Compiler differs from the admitted Rust payload.")
    for name in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTC_BOOTSTRAP"):
        if os.environ.get(name):
            raise ValueError("Ambient compiler flags cannot alter this build recipe.")
    if any(item.startswith("@") or "sanitizer-coverage" in item
           or "passes=sancov" in item for item in arguments):
        raise ValueError("Hidden or pre-existing coverage arguments are forbidden.")
    names = []
    for index, argument in enumerate(arguments):
        if argument == "--crate-name":
            if index + 1 == len(arguments):
                raise ValueError("Missing crate name.")
            names.append(arguments[index + 1])
        elif argument.startswith("--crate-name="):
            names.append(argument.partition("=")[2])
    if len(names) > 1 or any(not name or not name.isidentifier() for name in names):
        raise ValueError("Ambiguous or invalid crate-name arguments.")
    crate = names[0] if names else None
    instrumented = crate in SUBJECTS
    record(crate, arguments, instrumented)
    os.execv(compiler, [str(compiler), *arguments, *BASE_FLAGS,
                       *(FLAGS if instrumented else ())])


if __name__ == "__main__":
    try:
        main()
    except (OSError, ValueError, KeyError) as error:
        print(f"state-guidance compiler admission failed: {error}", file=sys.stderr)
        raise SystemExit(2) from error
