# SPDX-License-Identifier: GPL-3.0-only
"""Bounded file custody shared by GUI source readers, not their semantic oracles."""
import os
from pathlib import Path
import stat


def read_regular(path: Path, maximum: int) -> bytes:
    """Read retained regular-file bytes or reject replacement, growth and links.

    Nonblocking open lets descriptor checks reject a raced FIFO without waiting
    for another process. The caller owns directory scope and aggregate budgets.
    """
    if type(maximum) is not int or maximum < 0:
        raise ValueError('source byte bound')
    before = path.lstat()
    if not stat.S_ISREG(before.st_mode) or before.st_size > maximum:
        raise ValueError('regular source byte bound')
    flags = os.O_RDONLY | getattr(os, 'O_NOFOLLOW', 0) | getattr(os, 'O_NONBLOCK', 0)
    descriptor = os.open(path, flags)
    with os.fdopen(descriptor, 'rb') as stream:
        opened = os.fstat(stream.fileno())
        identity = lambda item: (item.st_dev, item.st_ino, item.st_size,
                                 item.st_mtime_ns, item.st_ctime_ns)
        if not stat.S_ISREG(opened.st_mode) or identity(opened) != identity(before):
            raise ValueError('source custody changed before reading')
        raw = stream.read(maximum + 1)
        after = os.fstat(stream.fileno())
    if len(raw) > maximum or len(raw) != opened.st_size or identity(opened) != identity(after):
        raise ValueError('source byte bound or custody changed during reading')
    return raw
