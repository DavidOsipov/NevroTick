#!/usr/bin/env python3
"""Bound one reviewed HTTPS acquisition before accepting its response body.

Callers own the exact URL, approved redirect origins, digest and byte limit.
No input response can expand that authority. Downloads are streamed to a fresh
temporary file and published only after bounded completion and digest checking.
"""
from __future__ import annotations

import argparse
import hashlib
import os
from pathlib import Path
import re
import ssl
import stat
import tempfile
import time
import urllib.error
import urllib.parse
import urllib.request

CHUNK_BYTES = 64 * 1024
MAXIMUM_BYTES = 4 * 1024 ** 3
MAXIMUM_REDIRECTS = 5


def https_origin(url: str) -> str:
    """Return a strict HTTPS origin; reject ambiguous authority before I/O."""
    if (not isinstance(url, str) or not 1 <= len(url) <= 8192
            or any(ord(char) <= 32 or ord(char) >= 127 for char in url) or "\\" in url):
        raise RuntimeError("Download URL is outside the admitted representation.")
    try:
        parsed = urllib.parse.urlsplit(url)
        port = parsed.port
    except ValueError as error:
        raise RuntimeError("Download URL has an invalid authority.") from error
    if (parsed.scheme != "https" or not parsed.hostname or parsed.username is not None
            or parsed.password is not None or port not in (None, 443) or parsed.fragment
            or not re.fullmatch(r"[A-Za-z0-9.-]+", parsed.hostname)):
        raise RuntimeError("Download requires HTTPS without credentials, fragment or alternate port.")
    return "https://" + parsed.hostname.lower()


class NoAutomaticRedirect(urllib.request.HTTPRedirectHandler):
    """Require explicit admission before urllib can send a redirected request."""

    def redirect_request(self, request, response, code, message, headers, newurl):
        return None


def _open(request: urllib.request.Request, timeout: float):
    context = ssl.create_default_context()
    context.minimum_version = ssl.TLSVersion.TLSv1_2
    return urllib.request.build_opener(
        NoAutomaticRedirect(), urllib.request.HTTPSHandler(context=context)
    ).open(request, timeout=timeout)


def _chunks(stream, maximum_bytes: int):
    """Read at most the limit plus one sentinel byte, independent of headers."""
    remaining = maximum_bytes
    read = getattr(stream, "read1", stream.read)
    while True:
        block = read(min(CHUNK_BYTES, remaining + 1))
        if not isinstance(block, bytes):
            raise RuntimeError("Download stream returned a non-byte body.")
        if len(block) > remaining:
            raise RuntimeError("Download body exceeds its admitted byte limit.")
        if not block:
            return
        remaining -= len(block)
        yield block


def _read_cached(path: Path, maximum_bytes: int, *, collect: bool = False):
    """Admit a bounded regular cache file without following its final symlink."""
    descriptor = os.open(path, os.O_RDONLY | os.O_NONBLOCK | getattr(os, "O_NOFOLLOW", 0))
    with os.fdopen(descriptor, "rb") as stream:
        before = os.fstat(stream.fileno())
        if not stat.S_ISREG(before.st_mode) or before.st_size > maximum_bytes:
            raise RuntimeError("Cached download is not an admitted bounded regular file.")
        digest = hashlib.sha256()
        body = bytearray() if collect else None
        size = 0
        for block in _chunks(stream, maximum_bytes):
            digest.update(block)
            size += len(block)
            if body is not None:
                body.extend(block)
        after = os.fstat(stream.fileno())
        current = path.lstat()
        fields = ("st_dev", "st_ino", "st_size", "st_mtime_ns", "st_ctime_ns")
        if (any(getattr(before, name) != getattr(after, name) for name in fields)
                or any(getattr(after, name) != getattr(current, name) for name in fields)
                or size != before.st_size):
            raise RuntimeError("Cached download changed during bounded admission.")
        return digest.hexdigest(), size, bytes(body) if body is not None else None


def bounded_file_bytes(path: Path, maximum_bytes: int) -> bytes:
    """Read only an already-bounded small artifact; never use for large CI files."""
    return _read_cached(path, maximum_bytes, collect=True)[2]


def download_to_file(url: str, destination: Path, expected: str | None, *,
                     maximum_bytes: int, allowed_origins=(), timeout: int = 120) -> dict:
    """Acquire one bounded artifact, preserving the previous file on failure.

    A caller-supplied origin set can only admit HTTPS origins, never wildcard
    authorities. With no set, redirects are confined to the initial URL's origin.
    Content-Length is an early refusal only; actual reads enforce the byte bound.
    Elapsed checks and finite socket reads are separate from an outer hard wall
    limit: a blocked read can overrun the elapsed budget by its socket timeout.
    """
    if type(maximum_bytes) is not int or not 0 < maximum_bytes <= MAXIMUM_BYTES:
        raise RuntimeError("Download byte limit must be a positive finite admitted value.")
    if type(timeout) is not int or not 0 < timeout <= 300:
        raise RuntimeError("Download timeout is outside the finite admitted range.")
    if expected is not None and (not isinstance(expected, str)
                                 or re.fullmatch(r"[a-fA-F0-9]{64}", expected) is None):
        raise RuntimeError("Download digest must be SHA256.")
    initial_origin = https_origin(url)
    allowed = set()
    for origin in allowed_origins:
        canonical = https_origin(origin)
        if urllib.parse.urlsplit(origin).path not in ("", "/") or urllib.parse.urlsplit(origin).query:
            raise RuntimeError("Redirect admission requires origins, not URLs or patterns.")
        allowed.add(canonical)
    if not allowed:
        allowed.add(initial_origin)
    if len(allowed) > 8 or initial_origin not in allowed:
        raise RuntimeError("Initial download origin is not admitted.")
    destination = Path(destination)
    if destination.is_symlink() or (destination.exists() and not destination.is_file()):
        raise RuntimeError("Download destination must be a regular file or absent.")
    if destination.exists():
        digest, size, _ = _read_cached(destination, maximum_bytes)
        if expected is not None and digest == expected.lower():
            return {"bytes": size, "sha256": digest, "cached": True}
    destination.parent.mkdir(parents=True, exist_ok=True)
    descriptor, temporary_name = tempfile.mkstemp(prefix=".nevrotick-download-", dir=destination.parent)
    temporary = Path(temporary_name)
    started = time.monotonic()
    current_url = url
    try:
        with os.fdopen(descriptor, "wb") as output:
            for redirects in range(MAXIMUM_REDIRECTS + 1):
                if https_origin(current_url) not in allowed:
                    raise RuntimeError("Redirect target origin is not admitted.")
                remaining = timeout - (time.monotonic() - started)
                if remaining <= 0:
                    raise RuntimeError("Download exceeded its elapsed-time limit.")
                request = urllib.request.Request(current_url, headers={"Accept-Encoding": "identity"})
                try:
                    response = _open(request, remaining)
                except urllib.error.HTTPError as error:
                    with error:
                        if error.code not in (301, 302, 303, 307, 308):
                            raise RuntimeError("Download returned an unsuccessful HTTP status.") from error
                        locations = error.headers.get_all("Location", [])
                        if len(locations) != 1 or redirects == MAXIMUM_REDIRECTS:
                            raise RuntimeError("Download redirect is absent, ambiguous or exceeds its budget.")
                        target = urllib.parse.urljoin(current_url, locations[0])
                        if https_origin(target) not in allowed:
                            raise RuntimeError("Redirect target origin is not admitted.")
                        current_url = target
                    continue
                with response:
                    if response.status != 200:
                        raise RuntimeError("Download returned an unsuccessful HTTP status.")
                    lengths = response.headers.get_all("Content-Length", [])
                    if len(lengths) > 1 or (lengths and not re.fullmatch(r"[0-9]{1,20}", lengths[0])):
                        raise RuntimeError("Download has an ambiguous or malformed content length.")
                    if lengths and int(lengths[0]) > maximum_bytes:
                        raise RuntimeError("Download body exceeds its admitted byte limit.")
                    if response.headers.get("Content-Encoding", "identity").lower() != "identity":
                        raise RuntimeError("Encoded download bodies are not admitted.")
                    digest = hashlib.sha256()
                    size = 0
                    for block in _chunks(response, maximum_bytes):
                        if time.monotonic() - started >= timeout:
                            raise RuntimeError("Download exceeded its elapsed-time limit.")
                        output.write(block)
                        digest.update(block)
                        size += len(block)
                    if time.monotonic() - started >= timeout:
                        raise RuntimeError("Download exceeded its elapsed-time limit.")
                    if lengths and size != int(lengths[0]):
                        raise RuntimeError("Download body does not match its declared length.")
                    if expected is not None and digest.hexdigest() != expected.lower():
                        raise RuntimeError("Download SHA256 mismatch.")
                    output.flush()
                    os.fsync(output.fileno())
                    break
            else:
                raise RuntimeError("Download exhausted its redirect budget.")
        if destination.is_symlink():
            raise RuntimeError("Download destination became a symlink; replacement refused.")
        os.replace(temporary, destination)
        return {"bytes": size, "sha256": digest.hexdigest(), "cached": False}
    finally:
        temporary.unlink(missing_ok=True)


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--url", required=True)
    parser.add_argument("--destination", type=Path, required=True)
    parser.add_argument("--sha256", required=True)
    parser.add_argument("--maximum-bytes", type=int, required=True)
    parser.add_argument("--allowed-origin", action="append", default=[])
    parser.add_argument("--timeout", type=int, default=120)
    args = parser.parse_args(argv)
    try:
        result = download_to_file(args.url, args.destination, args.sha256,
                                  maximum_bytes=args.maximum_bytes,
                                  allowed_origins=args.allowed_origin, timeout=args.timeout)
    except (OSError, RuntimeError, urllib.error.URLError):
        parser.exit(1, "Bounded HTTPS acquisition failed; no replacement was published.\n")
    print(f"Verified bounded download: {result['bytes']} bytes; SHA256 {result['sha256']}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
