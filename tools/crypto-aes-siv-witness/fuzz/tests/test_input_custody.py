# SPDX-License-Identifier: GPL-2.0-only
"""Behavioral controls for the metadata input reader; no network or native work."""

import ast
import errno
import hashlib
import importlib.util
import os
from pathlib import Path
import stat
from types import SimpleNamespace
import sys
import tempfile
import unittest
from unittest import mock

# Use the sibling under test, not an installed module or a process-wide os patch.
sys.dont_write_bytecode = True
MODULE_PATH = Path(__file__).resolve().parents[1] / "input_custody.py"
SPEC = importlib.util.spec_from_file_location("s139_test_input_custody", MODULE_PATH)
if SPEC is None or SPEC.loader is None:
    raise RuntimeError("missing input custody module loader")
CUSTODY = importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name] = CUSTODY
SPEC.loader.exec_module(CUSTODY)

CHUNK = 64 * 1024


class ReadRequestExceeded(Exception):
    """Reject an oversized syscall before allocating its requested buffer."""


class TrackedOS:
    """Per-reader syscall proxy; optional hook changes only an owned test file."""

    def __init__(self, hook=None):
        self.requests = []
        self.opened = []
        self.closed = []
        self.hook = hook

    def __getattr__(self, name):
        return getattr(os, name)

    def open(self, *args, **kwargs):
        fd = os.open(*args, **kwargs)
        self.opened.append(fd)
        return fd

    def read(self, fd, count):
        self.requests.append(count)
        if count > CHUNK:
            raise ReadRequestExceeded("read request exceeds 64 KiB")
        result = os.read(fd, count)
        if self.hook is not None:
            hook, self.hook = self.hook, None
            hook()
        return result

    def close(self, fd):
        os.close(fd)
        self.closed.append(fd)


class ReaderTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix="custody-test-")
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name).resolve()
        self.proxy = TrackedOS()
        self.reader = CUSTODY.Reader(SimpleNamespace(os=self.proxy))

    def file(self, name, data):
        path = self.root / name
        path.write_bytes(data)
        return path

    def assert_descriptors_closed(self):
        self.assertEqual(self.proxy.opened, self.proxy.closed)
        for fd in self.proxy.opened:
            with self.assertRaises(OSError) as error:
                os.fstat(fd)
            self.assertEqual(error.exception.errno, errno.EBADF)

    def test_empty_small_multichunk_and_streaming_digest(self):
        for index, data in enumerate((b"", b"small\x00input", bytes(range(256)) * 513)):
            with self.subTest(size=len(data)):
                path = self.file(f"input-{index}", data)
                self.assertEqual(self.reader.read_file(path), data)
                self.assertEqual(self.reader.digest(path), hashlib.sha256(data).hexdigest())
        self.assertTrue(self.proxy.requests)
        self.assertLessEqual(max(self.proxy.requests), CHUNK)
        self.assert_descriptors_closed()

    def test_same_files_checks_current_bytes_each_time(self):
        first = self.file("first", b"alpha")
        second = self.file("second", b"beta")
        expected = {first: hashlib.sha256(b"alpha").hexdigest(),
                    second: hashlib.sha256(b"beta").hexdigest()}
        self.assertTrue(self.reader.same_files(expected))
        second.write_bytes(b"zeta")
        self.assertFalse(self.reader.same_files(expected))
        # A mismatch does not excuse skipping a later uninspectable input.
        with self.assertRaises(OSError):
            self.reader.same_files({first: "0" * 64, self.root / "absent": "0" * 64})
        self.assert_descriptors_closed()

    def test_explicit_oversize_symlink_and_directory_rejected(self):
        path = self.file("ordinary", b"x" * 33)
        empty = self.file("empty", b"")
        self.assertEqual(self.reader.read_file(empty, maximum=0), b"")
        self.assertEqual(self.reader.read_file(path, maximum=33), b"x" * 33)
        link = self.root / "link"
        link.symlink_to(path)
        directory_link = self.root / "directory-link"
        directory_link.symlink_to(self.root, target_is_directory=True)
        for label, action in (
            ("oversize", lambda: self.reader.read_file(path, maximum=32)),
            ("negative-maximum", lambda: self.reader.read_file(path, maximum=-1)),
            ("boolean-maximum", lambda: self.reader.read_file(path, maximum=True)),
            ("symlink", lambda: self.reader.read_file(link)),
            ("ancestor-symlink", lambda: self.reader.read_file(directory_link / path.name)),
            ("directory", lambda: self.reader.read_file(self.root)),
        ):
            with self.subTest(case=label):
                with self.assertRaises((RuntimeError, OSError)):
                    action()
                self.assert_descriptors_closed()

    def test_replaced_truncated_and_grown_files_are_rejected(self):
        original = bytes(range(256)) * 257
        for label in ("replace", "truncate", "grow"):
            with self.subTest(case=label):
                path = self.file(f"changing-{label}", original)
                replacement = self.file(f"replacement-{label}", b"other")

                def mutate():
                    if label == "replace":
                        os.replace(replacement, path)
                    elif label == "truncate":
                        os.truncate(path, 1)
                    else:
                        with path.open("ab") as stream:
                            stream.write(b"growth")

                self.proxy = TrackedOS(hook=mutate)
                self.reader = CUSTODY.Reader(SimpleNamespace(os=self.proxy))
                with self.assertRaises((RuntimeError, OSError)):
                    if label == "truncate":
                        self.reader.digest(path)
                    else:
                        self.reader.read_file(path)
                self.assertIsNone(self.proxy.hook, "mutation was not reached")
                self.assertTrue(self.proxy.opened, "failure preceded descriptor acquisition")
                self.assert_descriptors_closed()

    def test_read_exception_closes_descriptor(self):
        class FailingOS(TrackedOS):
            def read(self, fd, count):
                self.requests.append(count)
                raise OSError(errno.EIO, "owned read failure")

        path = self.file("read-failure", b"data")
        self.proxy = FailingOS()
        self.reader = CUSTODY.Reader(SimpleNamespace(os=self.proxy))
        with self.assertRaises(OSError) as error:
            self.reader.read_file(path)
        self.assertEqual(error.exception.errno, errno.EIO)
        self.assertTrue(self.proxy.opened)
        self.assert_descriptors_closed()


# These two historical controls are selected only when explicitly requested.
# A missing or altered requested source is a test failure, not a skip.
if "NEVROTICK_LEGACY_READER" in os.environ:
    LEGACY_SHA256 = "0d048451f21fbc3240bf69176c16247454371d9d43313336d801018e24888610"

    class GuardedStream:
        def __init__(self, stream, requests):
            self.stream = stream
            self.requests = requests

        def __enter__(self):
            self.stream.__enter__()
            return self

        def __exit__(self, *args):
            return self.stream.__exit__(*args)

        def read(self, count):
            self.requests.append(count)
            if count < 0 or count > CHUNK:
                raise ReadRequestExceeded("legacy read exceeds 64 KiB")
            return self.stream.read(count)

    class LegacyOS(TrackedOS):
        def fdopen(self, *args, **kwargs):
            return GuardedStream(os.fdopen(*args, **kwargs), self.requests)

    class LegacyControls(ReaderTests):
        # Inherit only fixture helpers, not the four current-reader test cases.
        test_empty_small_multichunk_and_streaming_digest = None
        test_same_files_checks_current_bytes_each_time = None
        test_explicit_oversize_symlink_and_directory_rejected = None
        test_replaced_truncated_and_grown_files_are_rejected = None
        test_read_exception_closes_descriptor = None

        def setUp(self):
            super().setUp()
            self.proxy = LegacyOS()
            path = Path(os.environ["NEVROTICK_LEGACY_READER"])
            self.assertTrue(path.is_absolute())
            self.assertEqual(path.resolve(strict=True), path)
            info = path.lstat()
            self.assertTrue(stat.S_ISREG(info.st_mode) and info.st_size <= 1024 * 1024)
            raw = CUSTODY.read_file(path, maximum=1024 * 1024)
            self.assertEqual(hashlib.sha256(raw).hexdigest(), LEGACY_SHA256)
            tree = ast.parse(raw, filename=str(path))
            functions = [node for node in tree.body
                         if isinstance(node, ast.FunctionDef) and node.name == "read_file"]
            self.assertEqual(len(functions), 1)
            namespace = {"Path": Path, "os": self.proxy, "stat": stat}
            code = compile(ast.Module(body=functions, type_ignores=[]), str(path), "exec")
            exec(code, namespace)  # The pinned function alone; no module import or global os patch.
            self.legacy_read = namespace["read_file"]

        def test_legacy_default_request_rejected_before_large_allocation(self):
            path = self.file("legacy-default", b"public small input")
            with self.assertRaises(ReadRequestExceeded):
                self.legacy_read(path)
            self.assertEqual(self.proxy.requests, [512 * 1024 * 1024 + 1])
            self.assert_descriptors_closed()

        def test_legacy_explicit_65535_limit_healthy(self):
            data = b"public small input"
            path = self.file("legacy-healthy", data)
            self.assertEqual(self.legacy_read(path, maximum=65535), data)
            self.assertEqual(self.proxy.requests, [65536])
            self.assert_descriptors_closed()

    class LegacyLoaderTests(unittest.TestCase):
        def test_changed_legacy_source_rejected_before_parsing(self):
            raw = CUSTODY.read_file(Path(os.environ["NEVROTICK_LEGACY_READER"]),
                                    maximum=1024 * 1024)
            self.assertEqual(hashlib.sha256(raw).hexdigest(), LEGACY_SHA256)
            with tempfile.TemporaryDirectory(prefix="legacy-loader-test-") as folder:
                source = Path(folder).resolve() / "legacy.py"
                source.write_bytes(raw)
                original_lstat = Path.lstat
                changed = False

                def grow_after_check(path, *args, **kwargs):
                    nonlocal changed
                    info = original_lstat(path, *args, **kwargs)
                    if path == source and not changed:
                        source.write_bytes(b"x" * (1024 * 1024 + 1))
                        changed = True
                    return info

                case = LegacyControls("test_legacy_explicit_65535_limit_healthy")
                try:
                    with mock.patch.dict(os.environ, {"NEVROTICK_LEGACY_READER": str(source)}), \
                            mock.patch.object(Path, "lstat", grow_after_check), \
                            mock.patch.object(ast, "parse") as parse:
                        with self.assertRaisesRegex(RuntimeError, "bounded regular file"):
                            case.setUp()
                        self.assertTrue(changed, "growth did not reach the loader admission seam")
                        parse.assert_not_called()
                finally:
                    case.doCleanups()
