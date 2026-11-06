#!/usr/bin/env python3
"""Acquire one pinned Verus pilot payload; never execute downloaded bytes.

This is a finite, private candidate, not an installer or shipping-tool policy.
The existing Core task owns authorization, budgets and the adoption decision.
Only a newly created sibling of its existing evidence directories is writable.
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path, PurePosixPath
import shutil
import stat
import sys
import time
import urllib.parse
import urllib.request
import zipfile


VERSION = "0.2026.11.02.8bf68ee"
COMMIT = "8bf68ee741df833ea5d0fe6a2255126fdea87e47"
ARCHIVE_BYTES = 36_022_359
ARCHIVE_SHA256 = "f3a0119292df8855e4435da8130bee1873b0e6a60c306accf81a08ea687bba3c"
ARCHIVE_URL = (f"https://github.com/verus-lang/verus/releases/download/release/{VERSION}/"
               f"verus-{VERSION}-x86-linux.zip")
LIMIT = 512 * 1024**2
RESERVE = 3 * 1024**3
MAX_ENTRIES = 4096
MAX_DIRECTORIES = 4096
EXECUTABLES = frozenset(("verus", "rust_verify", "z3"))
LICENSES = {
    "LICENSE.verus": (
        f"https://raw.githubusercontent.com/verus-lang/verus/{COMMIT}/LICENSE", 1079,
        "ab78fc657425500bc64f617e36bb6819c6b0d039c573642f603dd594b12fe0d3"),
    "LICENSE.z3": (
        "https://raw.githubusercontent.com/Z3Prover/z3/z3-4.16.0/LICENSE.txt", 1096,
        "e617cad2ab9347e3129c2b171e87909332174e17961c5c3412d0799469111337"),
}
HOSTS = frozenset(("github.com", "release-assets.githubusercontent.com", "raw.githubusercontent.com"))
_custody_spec = importlib.util.spec_from_file_location(
    "verus_pilot_custody", Path(__file__).resolve().parents[1] / "crypto-aes-siv-witness/fuzz/input_custody.py")
CUSTODY = importlib.util.module_from_spec(_custody_spec)
_custody_spec.loader.exec_module(CUSTODY)


def require(condition, message):
    """Reject an unproved acquisition precondition without running a payload."""
    if not condition:
        raise RuntimeError(message)


def allowed_url(url):
    """Permit only HTTPS official acquisition endpoints, without URL credentials."""
    parts = urllib.parse.urlsplit(url)
    require(parts.scheme == "https" and parts.hostname in HOSTS
            and parts.port in (None, 443) and parts.username is None
            and parts.password is None and not parts.fragment, "Unapproved acquisition URL")


class Redirects(urllib.request.HTTPRedirectHandler):
    """Check every redirect before following it; do not retain signed asset URLs."""

    def redirect_request(self, request, fp, code, message, headers, newurl):
        allowed_url(newurl)
        return super().redirect_request(request, fp, code, message, headers, newurl)


def capacity(folder, deadline, *, pending=0):
    """Enforce the single-attempt wall clock, total bytes and free-space reserve."""
    require(time.monotonic() < deadline, "Ten-minute bootstrap deadline exhausted")
    total = 0
    for directory, subdirs, files in os.walk(folder, followlinks=False):
        for name in subdirs + files:
            item = Path(directory) / name
            info = item.lstat()
            require(info.st_uid == os.geteuid() and not stat.S_ISLNK(info.st_mode),
                    "Acquisition entry is unowned or redirected")
            require(stat.S_ISREG(info.st_mode) or stat.S_ISDIR(info.st_mode),
                    "Acquisition contains a special entry")
            if stat.S_ISREG(info.st_mode):
                require(info.st_nlink == 1, "Acquisition file has an alias")
            total += info.st_blocks * 512
    require(total + pending <= LIMIT, "512-MiB total acquisition footprint exhausted")
    require(shutil.disk_usage(folder).free >= RESERVE + pending, "Three-GiB reserve exhausted")
    return total


def fetch(url, target, size, expected, folder, deadline):
    """One bounded HTTPS fetch, no proxy environment, retry, hooks or subprocess."""
    allowed_url(url)
    capacity(folder, deadline, pending=size + 4096)
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}), Redirects())
    request = urllib.request.Request(url, headers={"User-Agent": "NevroTick-bounded-Verus-pilot"})
    checksum = hashlib.sha256()
    count = 0
    with opener.open(request, timeout=20) as response, target.open("xb") as output:
        os.chmod(target, 0o600)
        require(response.status == 200, "Acquisition HTTP status is not 200")
        while True:
            require(time.monotonic() < deadline, "Bootstrap deadline exhausted during fetch")
            chunk = response.read(min(1024 * 1024, size + 1 - count))
            if not chunk:
                break
            count += len(chunk)
            require(count <= size, "Acquisition exceeded pinned size")
            capacity(folder, deadline, pending=len(chunk) + 4096)
            output.write(chunk)
            checksum.update(chunk)
    require(count == size and checksum.hexdigest() == expected, "Acquisition bytes/digest mismatch")
    capacity(folder, deadline)


def members(archive):
    """Validate the entire ZIP namespace/types/budget before creating any member."""
    entries = archive.infolist()
    require(0 < len(entries) <= MAX_ENTRIES, "ZIP entry budget exhausted")
    names, files, roots, directories = set(), set(), set(), set()
    total = 0
    for entry in entries:
        raw = entry.filename
        require(raw == entry.orig_filename and raw and len(raw) <= 512
                and "\\" not in raw and ":" not in raw
                and all(32 <= ord(c) < 127 for c in raw), "Unsafe ZIP member spelling")
        path = PurePosixPath(raw)
        require(not path.is_absolute() and all(p not in ("", ".", "..") for p in raw.rstrip("/").split("/"))
                and raw == path.as_posix() + ("/" if entry.is_dir() else ""),
                "ZIP member traversal or normalization")
        key = path.as_posix()
        require(key not in names, "Duplicate ZIP member")
        names.add(key)
        roots.add(path.parts[0])
        require(len(path.parts) <= 16, "ZIP directory depth exhausted")
        directories.update(parent.as_posix() for parent in path.parents if parent != PurePosixPath("."))
        require(not entry.flag_bits & 1 and entry.compress_type in (zipfile.ZIP_STORED, zipfile.ZIP_DEFLATED),
                "Unsupported encrypted/compressed ZIP member")
        kind = stat.S_IFMT(entry.external_attr >> 16)
        require(kind in ((0, stat.S_IFDIR) if entry.is_dir() else (0, stat.S_IFREG)),
                "ZIP link or special file is forbidden")
        require(0 <= entry.file_size <= 256 * 1024**2, "ZIP file size exhausted")
        if not entry.is_dir():
            require(len(path.parts) >= 2, "ZIP payload lacks a single parent directory")
            files.add(key)
            total += entry.file_size
        else:
            directories.add(key)
            require(entry.file_size == 0, "ZIP directory contains data")
    require(len(roots) == 1, "ZIP must contain exactly one payload root")
    require(len(directories) <= MAX_DIRECTORIES, "ZIP directory count exhausted")
    # Account for block rounding and directory metadata before creating entries.
    overhead = (len(files) + len(directories) + 16) * 8192
    require(total + ARCHIVE_BYTES + overhead + 16 * 1024**2 < LIMIT,
            "ZIP extraction exceeds total pilot budget")
    require(not any(parent.as_posix() in files for name in names
                    for parent in PurePosixPath(name).parents), "ZIP file/directory namespace collision")
    prefix = next(iter(roots))
    require({prefix + "/" + name for name in EXECUTABLES} <= files, "ZIP lacks the selected executable closure")
    return entries, prefix, total


def inventory(folder):
    """Hash every extracted regular file and record closed directory membership."""
    result = {}
    before = sorted(folder.rglob("*"))
    require(len(before) <= MAX_ENTRIES + len(LICENSES), "Payload entry count exhausted")
    total = 0
    for path in before:
        info = path.lstat()
        require(info.st_uid == os.geteuid() and (stat.S_ISREG(info.st_mode) or stat.S_ISDIR(info.st_mode)),
                "Unowned or special extracted entry")
        row = {"mode": oct(stat.S_IMODE(info.st_mode)), "kind": "directory"}
        if stat.S_ISREG(info.st_mode):
            require(info.st_nlink == 1, "Extracted file has an alias")
            total += info.st_size
            require(total <= LIMIT, "Payload inventory byte budget exhausted")
            checksum = CUSTODY.digest(path, maximum=min(info.st_size, 256 * 1024**2))
            require(CUSTODY._identity(path.lstat()) == CUSTODY._identity(info), "Payload changed during inventory")
            row.update(kind="file", bytes=info.st_size, sha256=checksum)
        result[path.relative_to(folder).as_posix()] = row
    require(before == sorted(folder.rglob("*")), "Payload namespace changed during inventory")
    return result


def acquire(folder):
    """Create one fresh private store and retain failures; repeat attempts are forbidden."""
    require(os.name == "posix" and os.geteuid() != 0, "Use an ordinary Linux user")
    parent = Path.home() / ".local/share/nevrotick"
    require(parent == parent.resolve(strict=True) and parent.is_dir(), "Store parent redirects")
    require(folder.is_absolute() and folder.parent == parent and not folder.exists(), "Use a new direct private store")
    folder.mkdir(mode=0o700)
    started = time.monotonic()
    started_unix = time.time()
    deadline = started + 600
    # Reserve the complete maximum footprint before the single network attempt.
    # Other concurrent disk writers remain outside this task's control; each
    # following write rechecks its bounded allocation immediately beforehand.
    capacity(folder, deadline, pending=LIMIT)
    archive_path = folder / "download.zip"
    fetch(ARCHIVE_URL, archive_path, ARCHIVE_BYTES, ARCHIVE_SHA256, folder, deadline)
    payload = folder / "payload"
    payload.mkdir(mode=0o700)
    with zipfile.ZipFile(archive_path) as archive:
        entries, prefix, total = members(archive)
        capacity(folder, deadline, pending=total + (len(entries) + MAX_DIRECTORIES) * 8192)
        for entry in entries:
            capacity(folder, deadline, pending=16 * 8192)
            relative = PurePosixPath(entry.filename).relative_to(prefix)
            if relative == PurePosixPath("."):
                continue
            target = payload / str(relative)
            if entry.is_dir():
                target.mkdir(mode=0o700, parents=True, exist_ok=True)
                continue
            target.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
            with archive.open(entry) as source, target.open("xb") as output:
                copied = 0
                while block := source.read(1024 * 1024):
                    copied += len(block)
                    require(copied <= entry.file_size, "ZIP expanded beyond declared size")
                    capacity(folder, deadline, pending=len(block) + 4096)
                    output.write(block)
                require(copied == entry.file_size, "ZIP truncated member")
            os.chmod(target, 0o500 if str(relative) in EXECUTABLES else 0o400)
            capacity(folder, deadline)
    for name in EXECUTABLES:
        # Header inspection need not buffer a complete executable.
        descriptor = os.open(payload / name, os.O_RDONLY | os.O_NOFOLLOW | os.O_CLOEXEC)
        try:
            header = os.read(descriptor, 20)
        finally:
            os.close(descriptor)
        require(header[:6] == b"\x7fELF\x02\x01" and header[18:20] == b"\x3e\x00", "Expected x86-64 little-endian ELF")
    for name, (url, size, checksum) in LICENSES.items():
        fetch(url, payload / name, size, checksum, folder, deadline)
        os.chmod(payload / name, 0o400)
    records = inventory(payload)
    manifest = {"status": "acquired-not-executed-not-adopted", "version": VERSION, "commit": COMMIT,
                "bootstrap_started_unix_seconds": started_unix,
                "acquisition_seconds": time.monotonic() - started,
                "archive": {"url": ARCHIVE_URL, "bytes": ARCHIVE_BYTES, "sha256": ARCHIVE_SHA256},
                "extracted_bytes": total, "payload": records,
                "licenses": {name: {"url": row[0], "bytes": row[1], "sha256": row[2]}
                             for name, row in LICENSES.items()}}
    data = (json.dumps(manifest, indent=2, sort_keys=True) + "\n").encode()
    path = folder / "manifest.json"
    path.write_bytes(data)
    os.chmod(path, 0o400)
    # Every member is authenticated by the archive digest and the retained closed
    # inventory. The redundant downloaded archive has no future use in this pilot.
    archive_path.unlink()
    footprint = capacity(folder, deadline)
    print(json.dumps({"store": str(folder), "manifest_sha256": hashlib.sha256(data).hexdigest(),
                      "payload_entries": len(records), "extracted_bytes": total,
                      "disk_bytes": footprint, "executed": False}))


if __name__ == "__main__":
    require(len(sys.argv) == 2, "Pass exactly one fresh private store path")
    acquire(Path(sys.argv[1]))
