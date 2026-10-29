#!/usr/bin/env python3
"""Acquire the exact reviewed UI-M0 crate archives without executing Cargo/code.

Writes only a new external vendor directory. --configure additionally creates a
private local pointer for verify.py; existing outputs/pointers are not replaced.
"""

from __future__ import annotations

import argparse
import gzip
import hashlib
import io
import json
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import tarfile
import tempfile
import tomllib
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
MAX_ARCHIVE = 16 * 1024 * 1024
MAX_EXPANDED = 80 * 1024 * 1024
MAX_MEMBERS = 10000


def sha(data: bytes) -> str:
    """Return a content identity for already bounded bytes."""
    return hashlib.sha256(data).hexdigest()


def files_digest(files: dict[str, str]) -> str:
    """Use the same canonical file-map encoding as the product verifier."""
    return sha(json.dumps(files, sort_keys=True, separators=(",", ":")).encode())


def archive_files(data: bytes, package: dict) -> dict[str, bytes]:
    """Verify archive identity and return bounded regular files below its prefix."""
    if len(data) > MAX_ARCHIVE or sha(data) != package["checksum"]:
        raise ValueError("Crate archive identity or size does not match admission")
    prefix = f"{package['name']}-{package['version']}"
    files: dict[str, bytes] = {}
    total = 0
    with gzip.GzipFile(fileobj=io.BytesIO(data)) as stream:
        raw_tar = stream.read(MAX_EXPANDED + 1)
    if len(raw_tar) > MAX_EXPANDED:
        raise ValueError("Crate archive metadata expansion exceeds budget")
    with tarfile.open(fileobj=io.BytesIO(raw_tar), mode="r:") as archive:
        for count, member in enumerate(archive, 1):
            if count > MAX_MEMBERS:
                raise ValueError("Crate archive member count exceeds budget")
            path = PurePosixPath(member.name)
            if (path.is_absolute() or ".." in path.parts or "\\" in member.name
                    or not path.parts or path.parts[0] != prefix):
                raise ValueError("Crate archive path escapes its exact package")
            if member.isdir():
                continue
            if not member.isfile() or len(path.parts) < 2:
                raise ValueError("Crate archive contains a link or special member")
            relative = PurePosixPath(*path.parts[1:]).as_posix()
            if relative in files or relative == ".cargo-checksum.json":
                raise ValueError("Duplicate or reserved crate archive member")
            total += member.size
            if total > MAX_EXPANDED:
                raise ValueError("Crate archive expansion exceeds budget")
            stream = archive.extractfile(member)
            if stream is None:
                raise ValueError("Missing regular archive member data")
            with stream:
                contents = stream.read(member.size + 1)
            if len(contents) != member.size:
                raise ValueError("Crate archive member length mismatch")
            files[relative] = contents
    if not files or "Cargo.toml" not in files:
        raise ValueError("Crate archive has no manifest")
    manifest = tomllib.loads(files["Cargo.toml"].decode())["package"]
    if (manifest["name"], manifest["version"]) != (package["name"], package["version"]):
        raise ValueError("Crate manifest identity mismatch")
    if files_digest({name: sha(value) for name, value in files.items()}) != package["files_sha256"]:
        raise ValueError("Crate extracted source differs from reviewed file-map pin")
    return files


def acquire(vendor: Path, policy: dict) -> None:
    """Publish a complete new vendor directory after every archive validates."""
    if vendor.exists() or vendor.is_symlink() or vendor.is_relative_to(ROOT):
        raise ValueError("Vendor destination must be new and outside the checkout")
    vendor.parent.mkdir(parents=True, exist_ok=True)
    temporary = Path(tempfile.mkdtemp(prefix=".ui-m0-acquire-", dir=vendor.parent))
    try:
        for package in policy["packages"]:
            name, version = package["name"], package["version"]
            if not re.fullmatch(r"[A-Za-z0-9_-]+", name) or not re.fullmatch(r"\d+\.\d+\.\d+", version):
                raise ValueError("Invalid admitted package identity")
            url = f"https://static.crates.io/crates/{name}/{name}-{version}.crate"
            request = urllib.request.Request(url, headers={"User-Agent": "NevroTick-UI-M0-admission"})
            with urllib.request.urlopen(request, timeout=60) as response:
                data = response.read(MAX_ARCHIVE + 1)
            files = archive_files(data, package)
            directory = temporary / f"{name}-{version}"
            directory.mkdir()
            for relative, contents in files.items():
                destination = directory / relative
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_bytes(contents)
            checksums = {"package": package["checksum"], "files": {key: sha(value) for key, value in files.items()}}
            (directory / ".cargo-checksum.json").write_text(json.dumps(checksums, sort_keys=True))
            print(f"Verified {name} {version}", flush=True)
        # Reserve a new destination at publication time. rename(directory) alone
        # could replace an empty directory created concurrently by another owner.
        vendor.mkdir()
        for package in temporary.iterdir():
            package.rename(vendor / package.name)
    finally:
        if temporary.exists():
            shutil.rmtree(temporary)


def main() -> int:
    """Acquire admitted inputs and optionally configure a new local pointer."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--vendor-dir", type=Path, required=True)
    parser.add_argument("--configure", action="store_true")
    parser.add_argument("--scratch-root", type=Path)
    args = parser.parse_args()
    try:
        policy = json.loads((ROOT / "policy/ui-m0-dependencies.json").read_text())
        if policy["schema_version"] != 1 or sha((ROOT / "Cargo.lock").read_bytes()) != policy["lockfile_sha256"]:
            raise ValueError("Lockfile does not match the reviewed graph")
        packages = tomllib.loads((ROOT / "Cargo.lock").read_text())["package"]
        expected = {(p["name"], p["version"], p["checksum"]) for p in packages if "source" in p}
        actual = {(p["name"], p["version"], p["checksum"]) for p in policy["packages"]}
        if expected != actual or len(actual) != len(policy["packages"]):
            raise ValueError("Reviewed package inventory differs from Cargo.lock")
        pointer = Path.home() / ".local/share/nevrotick/ui-m0.json"
        if args.configure and (pointer.exists() or pointer.is_symlink() or args.scratch_root is None):
            raise ValueError("Configuration needs --scratch-root and an absent local pointer")
        vendor = args.vendor_dir.expanduser().resolve()
        acquire(vendor, policy)
        if args.configure:
            scratch = args.scratch_root.expanduser().resolve()
            if scratch.is_relative_to(ROOT):
                raise ValueError("Scratch must be external to the checkout")
            scratch.mkdir(parents=True, exist_ok=True)
            pointer.parent.mkdir(parents=True, exist_ok=True)
            with os.fdopen(os.open(pointer, os.O_CREAT | os.O_EXCL | os.O_WRONLY, 0o600), "w") as output:
                json.dump({"vendor_dir": str(vendor), "scratch_root": str(scratch),
                           "xwin_config": str(pointer.parent / "xwin.json")}, output, indent=2)
        print("Acquisition complete; no dependency build or application check executed.")
        return 0
    except (OSError, ValueError, KeyError, tarfile.TarError) as error:
        print(f"INCOMPLETE: {error}")
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
