"""Admit the selected installed coverage payload against its pinned wheel bytes."""

from __future__ import annotations

import hashlib
import json
from pathlib import Path
import sys
import platform
import zipfile

import coverage


def main() -> int:
    wheel = Path(sys.argv[1])
    expected = "36314997533266bde3d07765289674304d61bfe49165bb1b4fa9be12cb4ba0d1"
    if hashlib.sha256(wheel.read_bytes()).hexdigest() != expected:
        raise ValueError("coverage-wheel-hash-mismatch")
    package = Path(coverage.__file__).parent
    if platform.python_implementation() != "CPython" or sys.version_info[:2] != (3, 12):
        raise ValueError("coverage-runtime-mismatch-requires-cpython-3.12")
    with zipfile.ZipFile(wheel) as archive:
        entries = {name: archive.read(name) for name in archive.namelist()
                   if name.startswith("coverage/") and not name.endswith("/")}
    installed = {"coverage/" + path.relative_to(package).as_posix(): path.read_bytes()
                 for path in package.rglob("*") if path.is_file() and "__pycache__" not in path.parts}
    if entries != installed:
        raise ValueError("coverage-installed-payload-mismatch")
    files = {str(path): hashlib.sha256(path.read_bytes()).hexdigest()
             for path in package.rglob("*") if path.is_file()}
    for path in package.parent.glob("*.pth"):
        files[str(path)] = hashlib.sha256(path.read_bytes()).hexdigest()
    print(json.dumps({"version": coverage.__version__, "files": files,
                      "python_version": sys.version, "python_implementation": platform.python_implementation(),
                      "wheel": str(wheel), "wheel_sha256": expected,
                      "verified_wheel_entries": len(entries)}))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
