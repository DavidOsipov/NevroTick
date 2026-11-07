"""Observe actual regular-file consumption across a truthful concurrent growth."""

from __future__ import annotations

import importlib
import os
from pathlib import Path
import sys
import tempfile
import types
import unittest
from unittest import mock

from test_pipeline import REPOSITORY


FILE_LIMIT = 256 * 1024


class InputCustodyContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        sys.path.insert(0, str(REPOSITORY))
        try:
            cls.contract = importlib.import_module("metadata.pipeline.contract")
        finally:
            sys.path.remove(str(REPOSITORY))

    def observe_read(self, grow: bool) -> tuple[bytes | None, Exception | None, list[tuple[int, int]], list[os.stat_result], int]:
        """Mutate only our real file, after the receiving reader's first read."""
        with tempfile.TemporaryDirectory(prefix="nevrotick-read-budget-") as temporary:
            root = Path(temporary)
            path = root / "data"
            content = b"x" * FILE_LIMIT
            path.write_bytes(content)
            root_fd = os.open(root, os.O_RDONLY | os.O_DIRECTORY)
            original_read = os.read
            original_stat = os.fstat
            reads: list[tuple[int, int]] = []
            snapshots: list[os.stat_result] = []
            changed = False

            def truthful_stat(descriptor: int) -> os.stat_result:
                observed = original_stat(descriptor)
                snapshots.append(observed)
                return observed

            def observed_read(descriptor: int, requested: int) -> bytes:
                nonlocal changed
                actual = original_read(descriptor, requested)
                reads.append((requested, len(actual)))
                if grow and not changed:
                    # Initial fstat and this read are real. Extra data exists
                    # before all later reads; no metadata is forged or restored.
                    with path.open("ab") as writer:
                        writer.write(b"y" * (2 * FILE_LIMIT))
                    changed = True
                return actual

            native = types.SimpleNamespace(**{name: getattr(os, name) for name in dir(os)})
            native.read = observed_read
            native.fstat = truthful_stat
            value = None
            error = None
            try:
                with mock.patch.object(self.contract, "os", native):
                    try:
                        value = self.contract._read(root_fd, "data")
                    except self.contract.MetadataError as caught:
                        error = caught
                os.fstat(root_fd)  # The caller's qualified root remains owned.
            finally:
                os.close(root_fd)
            return value, error, reads, snapshots, path.stat().st_size

    def test_stable_exact_byte_limit_returns_the_complete_file(self) -> None:
        self.assertEqual(self.contract.MAX_FILE_BYTES, FILE_LIMIT)
        value, error, reads, snapshots, final_size = self.observe_read(False)
        self.assertIsNone(error)
        self.assertEqual(value, b"x" * FILE_LIMIT)
        self.assertEqual(sum(returned for _requested, returned in reads), FILE_LIMIT)
        self.assertEqual(snapshots[0].st_size, FILE_LIMIT)
        self.assertEqual(snapshots[-1].st_size, FILE_LIMIT)
        self.assertEqual(final_size, FILE_LIMIT)

    def test_growth_after_truthful_initial_stat_cannot_consume_over_budget(self) -> None:
        self.assertEqual(self.contract.MAX_FILE_BYTES, FILE_LIMIT)
        value, error, reads, snapshots, final_size = self.observe_read(True)
        self.assertIsNone(value)
        self.assertIsInstance(error, self.contract.MetadataError)
        self.assertIn("file changed during its bounded read", str(error))
        self.assertEqual(snapshots[0].st_size, FILE_LIMIT)
        self.assertEqual(snapshots[-1].st_size, 3 * FILE_LIMIT)
        self.assertEqual(final_size, 3 * FILE_LIMIT)
        # Count actual bytes returned by the native reader, not requested sizes
        # or final rejection. The extra byte is a permitted oversize sentinel.
        self.assertLessEqual(sum(returned for _requested, returned in reads), FILE_LIMIT + 1)


if __name__ == "__main__":
    unittest.main()
