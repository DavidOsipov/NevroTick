"""Real descriptor-read and isolated mount-observation controls for hosted xwin."""
from __future__ import annotations

import importlib.util
import io
import os
from pathlib import Path
import stat
import sys
import tempfile
import unittest
from unittest import mock


def load_checker(filename: str):
    path = Path(__file__).resolve().parent / filename
    name = "nevrotick_readback_tests_" + filename.replace(".", "_")
    spec = importlib.util.spec_from_file_location(name, path)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


READBACK = load_checker("check_xwin_hosted_receipt.py")
QUOTA = load_checker("check_xwin_quota.py")


class PrivateDirectoryControls(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-xwin-readback-", dir="/tmp")
        self.addCleanup(self.temporary.cleanup)
        self.workspace = Path(self.temporary.name)
        self.root = self.workspace / "owned"
        self.root.mkdir(mode=0o700)


class DescriptorReadControls(PrivateDirectoryControls):
    def setUp(self) -> None:
        super().setUp()
        self.directory = os.open(self.root, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC)
        self.addCleanup(os.close, self.directory)
        (self.root / "nested").mkdir(mode=0o700)
        self.body = b"public readback bytes"
        (self.root / "nested/value").write_bytes(self.body)

    def test_actual_bytes_at_exact_limit_pass_and_one_smaller_limit_refuses(self) -> None:
        self.assertEqual(READBACK.read_at(self.directory, "nested/value", len(self.body)), self.body)
        with self.assertRaises(RuntimeError):
            READBACK.read_at(self.directory, "nested/value", len(self.body) - 1)
        self.assertEqual(READBACK.read_at(self.directory, "nested/value", len(self.body)), self.body)

    def test_forbidden_components_refuse_before_the_body_reader_and_keep_caller_fd(self) -> None:
        before = os.fstat(self.directory)
        for relative in ["", ".", "..", "../outside", "/absolute", "nested/../value",
                         "./nested/value", "nested//value", "nested/value/"]:
            with self.subTest(relative=relative), \
                 mock.patch.object(READBACK.os, "read") as consume:
                with self.assertRaises(ValueError):
                    READBACK.read_at(self.directory, relative, 64)
                consume.assert_not_called()
                after = os.fstat(self.directory)
                self.assertEqual((after.st_dev, after.st_ino), (before.st_dev, before.st_ino))

    def test_linked_parent_cannot_read_an_existing_external_body(self) -> None:
        outside = self.workspace / "outside"
        outside.mkdir(mode=0o700)
        (outside / "value").write_bytes(b"outside secret sentinel")
        (self.root / "linked").symlink_to(outside, target_is_directory=True)
        with mock.patch.object(READBACK.os, "read") as consume:
            with self.assertRaises((ValueError, OSError)):
                READBACK.read_at(self.directory, "linked/value", 64)
            consume.assert_not_called()
        self.assertEqual((outside / "value").read_bytes(), b"outside secret sentinel")

    def test_link_hardlink_and_fifo_leaves_refuse_before_content_consumption(self) -> None:
        outside = self.workspace / "outside-value"
        outside.write_bytes(b"outside secret sentinel")
        selected = self.root / "selected"
        for kind in ["symlink", "hardlink", "fifo"]:
            with self.subTest(kind=kind):
                if kind == "symlink":
                    selected.symlink_to(outside)
                elif kind == "hardlink":
                    os.link(outside, selected)
                else:
                    os.mkfifo(selected)
                try:
                    with mock.patch.object(READBACK.os, "read") as consume:
                        with self.assertRaises((ValueError, OSError)):
                            READBACK.read_at(self.directory, "selected", 64)
                        consume.assert_not_called()
                    self.assertEqual(outside.read_bytes(), b"outside secret sentinel")
                finally:
                    selected.unlink()
        self.assertEqual(READBACK.read_at(self.directory, "nested/value", len(self.body)), self.body)

    def test_leaf_replaced_by_a_hardlink_at_actual_open_is_refused(self) -> None:
        selected = self.root / "raw"
        selected.write_bytes(b"ordinary admitted bytes")
        outside = self.workspace / "outside-raw"
        outside_body = b"outside secret sentinel"
        outside.write_bytes(outside_body)
        actual_open = os.open
        injected = False

        def replace_at_open(path, *args, **kwargs):
            nonlocal injected
            if not injected and Path(os.fsdecode(path)).name == "raw":
                selected.unlink()
                os.link(outside, selected)
                injected = True
            return actual_open(path, *args, **kwargs)

        with mock.patch.object(READBACK.os, "open", side_effect=replace_at_open):
            try:
                with self.assertRaises(ValueError):
                    READBACK.read_at(self.directory, "raw", 64)
            finally:
                self.assertTrue(injected, "the replacement must reach the actual leaf open")
                self.assertEqual(outside.read_bytes(), outside_body)

    def test_consumer_requires_the_existing_private_lock_and_actual_exclusive_ownership(self) -> None:
        # Exercise only the real producer/consumer lock mechanism. This plain
        # directory and constructed layout claim no hosted mount admission.
        def require_refusal():
            unexpected = None
            try:
                with self.assertRaises((ValueError, OSError)):
                    unexpected = READBACK.lock_owner(self.directory)
            finally:
                if unexpected is not None:
                    os.close(unexpected)

        payload = self.root / "payload"
        layout = READBACK.HOSTED.HostedLayout(self.root, payload, self.root / "bootstrap",
                                             payload / "cache", payload / "receipt.json", payload / "xwin.json")
        lock = self.root / ".owner.lock"
        with self.subTest(case="producer owns lock"):
            with READBACK.HOSTED.ownership(layout):
                require_refusal()
            owned = READBACK.lock_owner(self.directory)
            try:
                info = os.fstat(owned)
                self.assertTrue(stat.S_ISREG(info.st_mode))
                self.assertEqual((info.st_uid, stat.S_IMODE(info.st_mode), info.st_nlink), (os.getuid(), 0o600, 1))
                with self.assertRaises(RuntimeError):
                    with READBACK.HOSTED.ownership(layout):
                        self.fail("producer entered while the consumer owned its lock")
            finally:
                os.close(owned)
            with READBACK.HOSTED.ownership(layout):
                pass
        lock.unlink()

        outside = self.workspace / "outside-owner-lock"
        outside_body = b"unrelated private lock sentinel"
        descriptor = os.open(outside, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_CLOEXEC, 0o600)
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(outside_body)
        for case in ["missing", "symlink", "hardlink", "fifo", "wrong-mode", "oversized"]:
            with self.subTest(case=case):
                if case == "symlink":
                    lock.symlink_to(outside)
                elif case == "hardlink":
                    os.link(outside, lock)
                elif case == "fifo":
                    os.mkfifo(lock)
                elif case in {"wrong-mode", "oversized"}:
                    mode = 0o644 if case == "wrong-mode" else 0o600
                    descriptor = os.open(lock, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_CLOEXEC, mode)
                    with os.fdopen(descriptor, "wb") as stream:
                        stream.write(b"x" * (4097 if case == "oversized" else 1))
                try:
                    require_refusal()
                    if case == "missing":
                        self.assertFalse(lock.exists(), "consumer must never create the producer's lock")
                    self.assertEqual(outside.read_bytes(), outside_body)
                finally:
                    if lock.exists() or lock.is_symlink():
                        lock.unlink()

    def test_renamed_root_descriptor_keeps_original_bytes_after_path_replacement(self) -> None:
        held = os.fstat(self.directory)
        moved = self.workspace / "moved"
        self.root.rename(moved)
        self.root.mkdir(mode=0o700)
        (self.root / "nested").mkdir(mode=0o700)
        replacement = self.root / "nested/value"
        replacement.write_bytes(b"replacement bytes")

        self.assertEqual(READBACK.read_at(self.directory, "nested/value", len(self.body)), self.body)
        self.assertEqual((os.fstat(self.directory).st_dev, os.fstat(self.directory).st_ino),
                         (held.st_dev, held.st_ino))
        self.assertEqual((moved / "nested/value").read_bytes(), self.body)
        self.assertEqual(replacement.read_bytes(), b"replacement bytes")


class CleanupMountControls(PrivateDirectoryControls):
    @staticmethod
    def mount_line(path: Path) -> bytes:
        escaped = str(path).replace("\\", r"\134").replace(" ", r"\040").replace("\t", r"\011")
        return f"9 1 0:7 / {escaped} rw,nodev,nosuid,noexec - tmpfs tmpfs rw\n".encode()

    def observe(self, data: bytes, *, root=None) -> None:
        stream = io.BytesIO(data)
        with mock.patch("builtins.open", return_value=stream) as opened:
            QUOTA.assert_unmounted(self.root if root is None else root)
        opened.assert_called_once_with("/proc/self/mountinfo", "rb")

    def test_healthy_unmounted_directory_remains_untouched(self) -> None:
        sentinel = self.root / "keep"
        sentinel.write_bytes(b"unrelated retained evidence")
        self.observe(self.mount_line(Path("/")))
        self.assertEqual(sentinel.read_bytes(), b"unrelated retained evidence")
        self.assertEqual(self.root.stat().st_mode & 0o777, 0o700)

    def test_exact_or_nested_mount_prevents_cleanup(self) -> None:
        for mountpoint in [self.root, self.root / "payload", self.root / "payload/nested"]:
            with self.subTest(mountpoint=mountpoint), self.assertRaises(ValueError):
                self.observe(self.mount_line(mountpoint))
        self.assertTrue(self.root.is_dir())

    def test_escaped_space_mountpoint_is_decoded_before_subtree_admission(self) -> None:
        spaced = self.workspace / "owned with space"
        spaced.mkdir(mode=0o700)
        for mountpoint in [spaced, spaced / "payload"]:
            with self.subTest(mountpoint=mountpoint), self.assertRaises(ValueError):
                self.observe(self.mount_line(mountpoint), root=spaced)
        self.assertTrue(spaced.is_dir())

    def test_sibling_with_the_same_text_prefix_is_not_a_nested_mount(self) -> None:
        sibling = self.root.with_name(self.root.name + "-sibling")
        self.observe(self.mount_line(sibling))
        self.assertTrue(self.root.is_dir())

    def test_empty_malformed_and_oversized_mount_state_fail_closed(self) -> None:
        cases = [b"", b"not a mount record\n", b"1 0 0:1 / / rw - tmpfs\n", b"\xff\n",
                 b"not-an-id parent invalid-device / / rw - tmpfs tmpfs rw\n",
                 b"1 0 0:1 / relative-path rw - tmpfs tmpfs rw\n",
                 b"x" * (4 * 1024 ** 2 + 1)]
        for data in cases:
            with self.subTest(bytes=len(data), prefix=data[:80]), self.assertRaises(ValueError):
                self.observe(data)
        self.assertTrue(self.root.is_dir())


if __name__ == "__main__":
    unittest.main()
