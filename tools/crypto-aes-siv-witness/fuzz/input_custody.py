# SPDX-License-Identifier: GPL-2.0-only
"""Bounded, uncached reads for the standalone staged-AES qualification inputs.

The historical S44 helper remains untouched. Reader delegates its constants and
other attributes, but performs these three input operations here. Canonical-path
and descriptor/path metadata bookends detect observed redirection or mutation;
they do not establish an adversarial filesystem transaction. No file is written.
"""

import hashlib
import os
from pathlib import Path
import stat


DEFAULT_MAXIMUM = 512 * 1024 ** 2
CHUNK_SIZE = 64 * 1024


def _identity(info):
    """Keep S44's identity fields plus owner, group and link-count changes."""
    return tuple(getattr(info, name) for name in (
        "st_dev", "st_ino", "st_mode", "st_uid", "st_gid", "st_nlink",
        "st_size", "st_mtime_ns", "st_ctime_ns",
    ))


def _checked_path(path, maximum, io):
    """Admit one canonical regular file, rejecting resolved path redirection.

    The explicit ceiling can narrow the fixed 512-MiB maximum, including to
    zero. Reject invalid ceilings before opening any descriptor. Return the
    original lstat result for exact-size reads and later identity comparisons.
    """
    if type(maximum) is not int or not 0 <= maximum <= DEFAULT_MAXIMUM:
        raise RuntimeError("Invalid input byte ceiling")
    if not path.is_absolute() or path.resolve(strict=True) != path:
        raise RuntimeError("Noncanonical input: " + str(path))
    before = io.lstat(path)
    if not stat.S_ISREG(before.st_mode) or not 0 <= before.st_size <= maximum:
        raise RuntimeError("Input is not a bounded regular file: " + str(path))
    return before


def _consume(path, maximum, update, io):
    """Deliver exactly the original file extent in requests of at most 64 KiB.

    A no-follow, nonblocking descriptor must match the admitted path identity.
    Require every original byte, then an actual EOF read; recheck the held
    descriptor and canonical path afterwards. A short read, growth, replacement
    or metadata drift raises before a caller can return a value. Always close
    the descriptor, including when a read or consumer raises.
    """
    path = Path(path)
    before = _checked_path(path, maximum, io)
    identity = _identity(before)
    descriptor = io.open(path, os.O_RDONLY | os.O_CLOEXEC | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        if _identity(io.fstat(descriptor)) != identity:
            raise RuntimeError("Input changed before read: " + str(path))
        remaining = before.st_size
        while remaining:
            requested = min(CHUNK_SIZE, remaining)
            chunk = io.read(descriptor, requested)
            if not isinstance(chunk, bytes) or not 0 < len(chunk) <= requested:
                raise RuntimeError("Input ended or changed during read: " + str(path))
            update(chunk)
            remaining -= len(chunk)
        if io.read(descriptor, 1) != b"":
            raise RuntimeError("Input grew during read: " + str(path))
        if _identity(io.fstat(descriptor)) != identity:
            raise RuntimeError("Input descriptor changed during read: " + str(path))
        after = _checked_path(path, maximum, io)
        if _identity(after) != identity:
            raise RuntimeError("Input path changed during read: " + str(path))
    finally:
        io.close(descriptor)


def _read_file(path, maximum, io):
    """Collect only the admitted extent, without a ceiling-sized read request."""
    chunks = []
    _consume(path, maximum, chunks.append, io)
    return b"".join(chunks)


def _digest(path, maximum, io):
    """Stream every admitted byte through SHA256 without retaining file data."""
    digest = hashlib.sha256()
    _consume(path, maximum, digest.update, io)
    return digest.hexdigest()


def _same_files(expected, io):
    """Compare full fresh SHA256 hashes for every supplied path; cache nothing.

    A content mismatch returns False. Missing, redirected, oversized or changing
    inputs retain the read failure instead of being treated as a stable hash.
    """
    matches = True
    for path, value in expected.items():
        matches = (_digest(path, DEFAULT_MAXIMUM, io) == value) and matches
    return matches


def read_file(path, maximum=DEFAULT_MAXIMUM):
    """Return exact bytes of an unchanged canonical file within the fixed cap.

    Reads are at most 64 KiB. Returning bytes still requires memory proportional
    to the admitted file size; callers needing only identity should use digest.
    """
    return _read_file(path, maximum, os)


def digest(path, maximum=DEFAULT_MAXIMUM):
    """Return a streaming full-file SHA256 after descriptor/path bookends."""
    return _digest(path, maximum, os)


def same_files(expected):
    """Check every path against its complete current SHA256, with no cache."""
    return _same_files(expected, os)


class Reader:
    """Wrap a historical module without modifying its globals or other APIs.

    All other attributes are delegated unchanged. Only calls through this
    object's read_file/digest/same_files use these implementations; a historical
    delegated function keeps its original module globals. The delegate's os
    object supplies filesystem operations, permitting isolated test proxies
    without changing the process-global os module or the historical module.
    """

    def __init__(self, delegate):
        """Retain one delegate and its filesystem operations, without I/O."""
        self._delegate = delegate
        self._io = getattr(delegate, "os", os)

    def __getattr__(self, name):
        """Forward an otherwise absent attribute to the original object."""
        return getattr(self._delegate, name)

    def read_file(self, path, maximum=DEFAULT_MAXIMUM):
        """Read through this instance's bounded, checked filesystem route."""
        return _read_file(path, maximum, self._io)

    def digest(self, path, maximum=DEFAULT_MAXIMUM):
        """Stream a complete fresh SHA256 through this instance's route."""
        return _digest(path, maximum, self._io)

    def same_files(self, expected):
        """Compare every complete fresh hash through this instance's route."""
        return _same_files(expected, self._io)
