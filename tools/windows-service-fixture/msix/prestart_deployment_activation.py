# SPDX-License-Identifier: GPL-2.0-only
"""Commit one rendered deployment entry to an admitted Sandbox input mapping."""

from hashlib import sha256
import os
from pathlib import Path

from prestart_deployment_owner import RenderedDeploymentOwner
from prestart_deployment_session import _remaining, _require_deadlines
from prestart_receipts import Member, require, require_run


_LEAF = "deployment-activation.bin"
_MAX_ENTRY = 2 * 1024 * 1024


def publish_deployment_activation(
    records_in: Path, entry: RenderedDeploymentOwner, deadline: float, retirement_deadline: float,
) -> None:
    """Publish exact bytes once, after the coordinator acquires both access roles.

    The caller owns this fresh, dedicated mapping and its protected ancestors.
    The guest independently binds receipt to its original run and work cutoff.
    A partial record has no commit marker. A publication or later activation
    failure leaves the mapping under the transaction owner's cleanup authority.
    The marker commits publication. Subsequent wait or launch expiry belongs to
    the supervising owner and cannot undo a visible commit.
    """
    _require_deadlines(deadline, retirement_deadline)
    require_run(entry.run)
    require(type(entry.contents) is bytes and 0 < len(entry.contents) <= _MAX_ENTRY,
            "invalid deployment activation extent")
    require(type(entry.pin) is Member and type(entry.pin.size) is int and type(entry.pin.sha256) is str and
            entry.pin == Member(len(entry.contents), sha256(entry.contents).hexdigest()),
            "deployment activation pin differs")
    require(records_in.is_absolute(), "deployment activation requires an absolute mapping")
    header: bytes = (
        f"nevrotick-prestart-activation-v1 {entry.run.id} {entry.pin.size} {entry.pin.sha256}\n"
    ).encode("ascii")
    _remaining(deadline)
    access: int = os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW
    mapping: int = os.open(records_in, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW)
    try:
        _remaining(deadline)
        try:
            os.stat(_LEAF + ".commit", dir_fd=mapping, follow_symlinks=False)
        except FileNotFoundError:
            pass
        else:
            require(False, "deployment activation already committed")
        with os.fdopen(os.open(_LEAF, access, 0o600, dir_fd=mapping), "wb") as stream:
            stream.write(header)
            stream.write(entry.contents)
            stream.flush()
            os.fsync(stream.fileno())
        _remaining(deadline)
        os.close(os.open(_LEAF + ".commit", access, 0o600, dir_fd=mapping))
    finally:
        os.close(mapping)
