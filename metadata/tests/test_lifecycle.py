"""Actual source publication/check transitions, with owned deterministic faults."""

from __future__ import annotations

import errno
import importlib
import json
import os
from pathlib import Path
import shutil
import stat
import sys
import unittest
from unittest import mock

from test_pipeline import EXPECTED_FILES, REJECTED_SENTINEL, REPOSITORY, SourceFixture, snapshot


class LifecycleFixture(SourceFixture):
    @classmethod
    def setUpClass(cls) -> None:
        sys.path.insert(0, str(REPOSITORY))
        try:
            cls.contract = importlib.import_module("metadata.pipeline.contract")
            cls.generator = importlib.import_module("metadata.pipeline.generate")
        finally:
            sys.path.remove(str(REPOSITORY))

    def copy_tool(self, driver: str) -> None:
        """Copy the actual tool and instrument only its owned process driver."""
        (self.root / "metadata/__init__.py").write_bytes((REPOSITORY / "metadata/__init__.py").read_bytes())
        shutil.copytree(REPOSITORY / "metadata/pipeline", self.root / "metadata/pipeline")
        self.entrypoint = self.root / "metadata/pipeline/__main__.py"
        text = self.entrypoint.read_text(encoding="utf-8")
        marker = '\nif __name__ == "__main__":'
        self.assertEqual(text.count(marker), 1)
        prefix = ("\nimport importlib as _qa_importlib\n"
                  "_qa_generator = _qa_importlib.import_module('metadata.pipeline.generate')\n"
                  "_qa_trigger = Path(__file__).absolute().parents[2] / 'qa-trigger'\n")
        self.entrypoint.write_text(text.replace(marker, prefix + driver + marker), encoding="utf-8")

    def trigger(self) -> None:
        (self.root / "qa-trigger").write_text("owned deterministic test trigger", encoding="ascii")

    def assert_no_staging(self) -> None:
        self.assertEqual(list(self.base.glob(".nevrotick-metadata-*")), [])


class CodeIdentityLifecycleTests(LifecycleFixture):
    def test_code_change_during_generation_refuses_publication(self) -> None:
        self.copy_tool("""
_qa_write = _qa_generator._write_payload
_qa_changed = False
def _qa_write_then_change(directory, relative, data):
    global _qa_changed
    _qa_write(directory, relative, data)
    if _qa_trigger.exists() and not _qa_changed:
        helper = Path(__file__).parent / '__init__.py'
        helper.write_bytes(helper.read_bytes() + b'\\n# owned code change during generation\\n')
        _qa_changed = True
_qa_generator._write_payload = _qa_write_then_change
""")
        healthy = self.base / "healthy"
        self.require_success(self.cli("generate", output=healthy))
        self.require_success(self.cli("check", output=healthy))
        before = snapshot(healthy)
        self.trigger()
        self.require_rejection(self.cli("generate", output=self.output))
        self.assertIn(b"owned code change during generation", (self.root / "metadata/pipeline/__init__.py").read_bytes())
        self.assertFalse(self.output.exists())
        self.assertEqual(snapshot(healthy), before)
        self.assert_no_staging()

    def test_code_change_during_check_is_rejected_without_repair(self) -> None:
        self.copy_tool("""
_qa_read = _qa_generator._read_regular
_qa_changed = False
def _qa_read_then_change(directory, name, maximum):
    global _qa_changed
    data = _qa_read(directory, name, maximum)
    if _qa_trigger.exists() and name == 'metadata.json' and not _qa_changed:
        helper = Path(__file__).parent / '__init__.py'
        helper.write_bytes(helper.read_bytes() + b'\\n# owned code change during check\\n')
        _qa_changed = True
    return data
_qa_generator._read_regular = _qa_read_then_change
""")
        self.generate()
        self.require_success(self.cli("check", output=self.output))
        before = snapshot(self.output)
        self.trigger()
        self.require_rejection(self.cli("check", output=self.output))
        self.assertIn(b"owned code change during check", (self.root / "metadata/pipeline/__init__.py").read_bytes())
        self.assertEqual(snapshot(self.output), before)

    def test_replaced_output_root_is_rejected_without_touching_either_tree(self) -> None:
        self.generate()
        self.generator.check(self.root, self.output)
        original = snapshot(self.output)
        moved = self.base / "held-original"
        read = self.generator._read_regular
        changed = False
        replacement = None

        def replace_after_read(directory: int, name: str, maximum: int) -> bytes:
            nonlocal changed, replacement
            data = read(directory, name, maximum)
            if name == "metadata.json" and not changed:
                self.output.rename(moved)
                shutil.copytree(moved, self.output)
                (self.output / "metadata.json").write_bytes(b'{"wrong": "' + REJECTED_SENTINEL.encode() + b'"}')
                replacement = snapshot(self.output)
                changed = True
            return data

        with mock.patch.object(self.generator, "_read_regular", side_effect=replace_after_read):
            with self.assertRaises(self.contract.MetadataError):
                self.generator.check(self.root, self.output)
        self.assertTrue(changed)
        self.assertEqual(snapshot(moved), original)
        self.assertEqual(snapshot(self.output), replacement)
        self.assert_no_staging()

    def test_source_change_during_check_is_rejected_without_output_writes(self) -> None:
        self.generate()
        before = snapshot(self.output)
        read = self.generator._read_regular
        changed = False

        def change_after_read(directory: int, name: str, maximum: int) -> bytes:
            nonlocal changed
            data = read(directory, name, maximum)
            if name == "metadata.json" and not changed:
                self.record.write_bytes(self.record.read_bytes() + b"\n# source changed during check\n")
                changed = True
            return data

        with mock.patch.object(self.generator, "_read_regular", side_effect=change_after_read):
            with self.assertRaises(self.contract.MetadataError):
                self.generator.check(self.root, self.output)
        self.assertTrue(changed)
        self.assertEqual(snapshot(self.output), before)


class PublicationFaultTests(LifecycleFixture):
    def test_file_flush_failure_is_precommit_and_cleans_staging(self) -> None:
        fsync = self.generator.os.fsync
        reached = False

        def fail_file(descriptor: int) -> None:
            nonlocal reached
            if stat.S_ISREG(os.fstat(descriptor).st_mode):
                reached = True
                raise OSError(errno.EIO, "owned file flush fault")
            fsync(descriptor)

        with mock.patch.object(self.generator.os, "fsync", side_effect=fail_file):
            with self.assertRaises(OSError):
                self.generator.generate(self.root, self.output)
        self.assertTrue(reached)
        self.assertFalse(self.output.exists())
        self.assert_no_staging()
        self.generate()

    def test_child_directory_flush_failure_is_precommit_and_cleans_staging(self) -> None:
        fsync = self.generator.os.fsync
        reached = False

        def fail_child_directory(descriptor: int) -> None:
            nonlocal reached
            info = os.fstat(descriptor)
            if stat.S_ISDIR(info.st_mode):
                for staging in self.base.glob(".nevrotick-metadata-*"):
                    child = staging / "authoring"
                    if child.exists():
                        current = child.stat()
                        if (info.st_dev, info.st_ino) == (current.st_dev, current.st_ino):
                            reached = True
                            raise OSError(errno.EIO, "owned child-directory flush fault")
            fsync(descriptor)

        with mock.patch.object(self.generator.os, "fsync", side_effect=fail_child_directory):
            with self.assertRaises(OSError):
                self.generator.generate(self.root, self.output)
        self.assertTrue(reached, "A containing-directory flush must actually be reached")
        self.assertFalse(self.output.exists())
        self.assert_no_staging()
        self.generate()

    def test_final_staging_flush_failure_is_precommit_and_cleans_staging(self) -> None:
        fsync = self.generator.os.fsync
        reached = False

        def fail_complete_staging(descriptor: int) -> None:
            nonlocal reached
            info = os.fstat(descriptor)
            if stat.S_ISDIR(info.st_mode):
                for staging in self.base.glob(".nevrotick-metadata-*"):
                    current = staging.stat()
                    if ((info.st_dev, info.st_ino) == (current.st_dev, current.st_ino)
                            and set(snapshot(staging)) == EXPECTED_FILES):
                        reached = True
                        raise OSError(errno.EIO, "owned complete-staging flush fault")
            fsync(descriptor)

        with mock.patch.object(self.generator.os, "fsync", side_effect=fail_complete_staging):
            with self.assertRaises(OSError):
                self.generator.generate(self.root, self.output)
        self.assertTrue(reached)
        self.assertFalse(self.output.exists())
        self.assert_no_staging()
        self.generate()

    def test_rename_failure_is_precommit_and_prior_candidate_survives(self) -> None:
        prior = self.base / "prior"
        self.require_success(self.cli("generate", output=prior))
        before = snapshot(prior)
        with mock.patch.object(self.generator.os, "rename", side_effect=OSError(errno.EIO, "owned rename fault")) as rename:
            with self.assertRaises(OSError):
                self.generator.generate(self.root, self.output)
        self.assertEqual(rename.call_count, 1)
        self.assertFalse(self.output.exists())
        self.assertEqual(snapshot(prior), before)
        self.assert_no_staging()
        self.generate()

    def test_zero_progress_write_is_rejected_and_cleans_staging(self) -> None:
        with mock.patch.object(self.generator.os, "write", return_value=0) as write:
            with self.assertRaises(self.contract.MetadataError):
                self.generator.generate(self.root, self.output)
        self.assertEqual(write.call_count, 1)
        self.assertFalse(self.output.exists())
        self.assert_no_staging()
        self.generate()

    def test_parent_flush_failure_preserves_published_tree_and_reports_uncertainty(self) -> None:
        fsync = self.generator.os.fsync
        parent_info = self.base.stat()
        reached = False

        def fail_parent(descriptor: int) -> None:
            nonlocal reached
            info = os.fstat(descriptor)
            if (info.st_dev, info.st_ino) == (parent_info.st_dev, parent_info.st_ino):
                reached = True
                raise OSError(errno.EIO, "owned parent flush fault")
            fsync(descriptor)

        error = None
        with mock.patch.object(self.generator.os, "fsync", side_effect=fail_parent):
            try:
                self.generator.generate(self.root, self.output)
            except Exception as caught:
                error = caught
        self.assertTrue(reached)
        self.assertEqual(set(snapshot(self.output)), EXPECTED_FILES)
        self.assert_no_staging()
        before = snapshot(self.output)
        self.assertIsInstance(error, self.contract.MetadataError)
        self.assertEqual(type(error).__name__, "PublicationUncertainError")
        self.assertEqual(str(error), "metadata-publication-durability-unconfirmed")
        # Reconciliation checks visible bytes; it cannot prove a lost flush acknowledgement.
        self.require_success(self.cli("check", output=self.output))
        self.require_rejection(self.cli("generate", output=self.output))
        self.assertEqual(snapshot(self.output), before)

    def test_cli_publication_uncertainty_has_distinct_redacted_outcome(self) -> None:
        self.copy_tool("""
_qa_fsync = _qa_generator.os.fsync
def _qa_fsync_parent(descriptor):
    if _qa_trigger.exists() and '--output' in sys.argv:
        output = Path(sys.argv[sys.argv.index('--output') + 1])
        parent = output.parent.stat()
        observed = _qa_generator.os.fstat(descriptor)
        if (observed.st_dev, observed.st_ino) == (parent.st_dev, parent.st_ino):
            raise OSError(5, 'PRIVATE-REJECTED-CONTENT-934781')
    return _qa_fsync(descriptor)
_qa_generator.os.fsync = _qa_fsync_parent
""")
        healthy = self.base / "healthy"
        self.require_success(self.cli("generate", output=healthy))
        self.require_success(self.cli("check", output=healthy))
        self.trigger()
        result = self.cli("generate", output=self.output)
        self.assertTrue(self.output.exists(), "The injected failure follows successful rename")
        self.assertEqual(result.returncode, 3)
        self.assertEqual(result.stderr, "")
        self.assertEqual(json.loads(result.stdout), {
            "schema_version": 1, "status": "publication-uncertain", "durability": "unconfirmed",
            "action": "check-existing-output-before-retry",
        })
        self.assertNotIn(str(self.base), result.stdout)
        self.assertNotIn(REJECTED_SENTINEL, result.stdout)
        before = snapshot(self.output)
        self.require_success(self.cli("check", output=self.output))
        self.require_rejection(self.cli("generate", output=self.output))
        self.assertEqual(snapshot(self.output), before)
        self.assert_no_staging()

    def test_owned_descriptors_are_released_after_precommit_failure(self) -> None:
        descriptors = Path("/proc/self/fd")
        if not descriptors.is_dir():
            self.skipTest("Linux descriptor observation is unavailable on this POSIX host")
        before = len(list(descriptors.iterdir()))
        for _ in range(3):
            with mock.patch.object(self.generator.os, "write", return_value=0):
                with self.assertRaises(self.contract.MetadataError):
                    self.generator.generate(self.root, self.output)
        self.assertEqual(len(list(descriptors.iterdir())), before)
        self.assertFalse(self.output.exists())
        self.assert_no_staging()


if __name__ == "__main__":
    unittest.main()
