#!/usr/bin/env python3
"""Observe hard byte and inode exhaustion on an owned, tiny quota filesystem."""
from __future__ import annotations

import argparse
import errno
import os
from pathlib import Path
import re
import stat
import sys


def check(root: Path) -> None:
    """Write at most 2 MiB/32 files after admitting an empty 1-MiB/16-inode mount.

    The caller owns mounting and unmounting. Failures preserve their original
    exception; cleanup removes only files created by this control. This is a
    kernel quota control, not SDK acquisition or aggregate memory qualification.
    """
    if sys.platform != "linux" or os.getuid() == 0:
        raise ValueError("Quota control requires the ordinary Linux runner user")
    if not root.is_absolute() or root.resolve(strict=True) != root:
        raise ValueError("Quota root must be canonical")
    info = root.lstat()
    descriptor = os.open(root, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        observed = os.fstat(descriptor)
        if (observed.st_dev, observed.st_ino) != (info.st_dev, info.st_ino):
            raise ValueError("Quota root changed before admission")
        _check_open(root, descriptor, observed)
        after = root.lstat()
        if (after.st_dev, after.st_ino) != (info.st_dev, info.st_ino):
            raise ValueError("Quota root changed during control")
    finally:
        os.close(descriptor)
    print("XWIN_QUOTA_CONTROL_OK byte_ENOSPC inode_ENOSPC")


def _check_open(root: Path, directory: int, info: os.stat_result) -> None:
    """Keep every control-file creation and removal relative to the owned FD."""
    capacity = os.fstatvfs(directory)
    if (not stat.S_ISDIR(info.st_mode) or info.st_uid != os.getuid()
            or stat.S_IMODE(info.st_mode) != 0o700
            or info.st_dev == root.parent.stat().st_dev
            or not 0 < capacity.f_frsize * capacity.f_blocks <= 1024 ** 2
            or not 0 < capacity.f_files <= 16):
        raise ValueError("Control requires a private, tiny dedicated quota filesystem")
    with os.scandir(directory) as entries:
        if next(entries, None) is not None:
            raise ValueError("Quota control root must be empty")
    created = []
    try:
        extent = "byte-control"
        descriptor = os.open(extent, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=directory)
        created.append(extent)
        try:
            for _ in range(512):
                try:
                    if os.write(descriptor, bytes(4096)) != 4096:
                        raise RuntimeError("Unexpected partial quota-control write")
                except OSError as error:
                    if error.errno != errno.ENOSPC:
                        raise
                    break
            else:
                raise RuntimeError("Byte quota did not refuse within the finite control")
        finally:
            os.close(descriptor)
        os.unlink(extent, dir_fd=directory)
        created.remove(extent)
        for number in range(32):
            path = f"inode-{number}"
            try:
                descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=directory)
            except OSError as error:
                if error.errno != errno.ENOSPC:
                    raise
                break
            created.append(path)
            os.close(descriptor)
        else:
            raise RuntimeError("Inode quota did not refuse within the finite control")
    finally:
        active_failure = sys.exc_info()[0] is not None
        cleanup_error = None
        for path in reversed(created):
            try:
                os.unlink(path, dir_fd=directory)
            except OSError as error:
                cleanup_error = error
        if cleanup_error is not None and not active_failure:
            raise cleanup_error


def assert_unmounted(root: Path) -> None:
    """Refuse recursive cleanup when actual mount state is present or unknown."""
    if not root.is_absolute() or root.resolve(strict=True) != root:
        raise ValueError("Cleanup root must be canonical")
    before = root.lstat()
    if (not stat.S_ISDIR(before.st_mode) or before.st_uid != os.getuid()
            or stat.S_IMODE(before.st_mode) != 0o700):
        raise ValueError("Cleanup root custody changed")
    with open("/proc/self/mountinfo", "rb") as stream:
        data = stream.read(4 * 1024 ** 2 + 1)
    if not data or len(data) > 4 * 1024 ** 2:
        raise ValueError("Unknown oversized mount state; retain owned directory")
    for line in data.decode().splitlines():
        fields, filesystem = line.split(" - ", 1)
        fields, filesystem = fields.split(), filesystem.split()
        if len(fields) < 6 or len(filesystem) < 3:
            raise ValueError("Unknown mount state; retain owned directory")
        if (not re.fullmatch(r"[0-9]+", fields[0]) or int(fields[0]) == 0
                or not re.fullmatch(r"[0-9]+", fields[1])
                or not re.fullmatch(r"[0-9]+:[0-9]+", fields[2])
                or not fields[3].startswith("/") or not fields[4].startswith("/")):
            raise ValueError("Malformed mount identity; retain owned directory")
        mountpoint = Path(re.sub(r"\\([0-7]{3})", lambda match: chr(int(match[1], 8)), fields[4]))
        if mountpoint.is_relative_to(root):
            raise ValueError("Owned subtree remains mounted; retain it")
    after = root.lstat()
    if (before.st_dev, before.st_ino) != (after.st_dev, after.st_ino):
        raise ValueError("Cleanup root changed during mount observation")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, required=True)
    parser.add_argument("--assert-unmounted", action="store_true")
    args = parser.parse_args()
    if args.assert_unmounted:
        assert_unmounted(args.root)
    else:
        check(args.root)
