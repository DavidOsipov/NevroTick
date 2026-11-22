# SPDX-License-Identifier: GPL-2.0-only
"""Exercise the real publication boundary in an owned temporary directory."""

import errno
from hashlib import sha256
import os
from pathlib import Path
import stat
import tempfile
import unittest
from unittest.mock import patch

import prepare_payload as producer


class PublicationTests(unittest.TestCase):
    def setUp(self):
        self.parent = Path(self.enterContext(tempfile.TemporaryDirectory()))
        self.output = self.parent / "result"
        self.payload = {"first": b"one", "Assets/second": b"two"}

    def assert_complete(self):
        actual = {path.relative_to(self.output).as_posix(): path.read_bytes()
                  for path in self.output.rglob("*") if path.is_file()}
        self.assertEqual(actual, self.payload)
        self.assertEqual(list(self.parent.iterdir()), [self.output])

    def test_write_failure_leaves_final_slot_absent_and_retry_works(self):
        write_payload = producer._write_payload

        def fail_after_previous_write(staging, name, content):
            self.assertFalse(self.output.exists())
            if name == "Assets/second":
                self.assertIn("first", os.listdir(staging))
                raise OSError(errno.EFBIG, "fixture write refusal")
            write_payload(staging, name, content)

        with patch.object(producer, "_write_payload", fail_after_previous_write):
            with self.assertRaises(OSError) as failure:
                producer.stage_payload(self.output, self.payload, None)
        self.assertEqual(failure.exception.errno, errno.EFBIG)
        self.assertEqual(list(self.parent.iterdir()), [])
        producer.stage_payload(self.output, self.payload, None)
        self.assert_complete()

    def test_late_empty_destination_keeps_original_inode(self):
        publish = producer.publish_new
        created = []

        def publish_beside_contender(parent, temporary, name):
            self.assertFalse(self.output.exists())
            os.mkdir(name, 0o700, dir_fd=parent)
            created.append(self.output.stat().st_ino)
            publish(parent, temporary, name)

        with patch.object(producer, "publish_new", publish_beside_contender):
            with self.assertRaises(FileExistsError):
                producer.stage_payload(self.output, self.payload, None)
        self.assertEqual(created, [self.output.stat().st_ino])
        self.assertEqual(list(self.output.iterdir()), [])
        self.assertEqual(list(self.parent.iterdir()), [self.output])

    def test_cleanup_failure_preserves_the_original_write_failure(self):
        with patch.object(producer, "_write_payload", side_effect=OSError(errno.EFBIG, "write")):
            with patch.object(producer, "_remove_staging", side_effect=OSError(errno.EIO, "cleanup")):
                with self.assertRaises(OSError) as failure:
                    producer.stage_payload(self.output, self.payload, None)
        self.assertEqual(failure.exception.errno, errno.EFBIG)
        self.assertEqual(len(failure.exception.__notes__), 1)
        self.assertIn("cleanup", failure.exception.__notes__[0])
        self.assertFalse(self.output.exists())

    def test_parent_flush_error_retains_complete_published_tree(self):
        flush = os.fsync
        parent = self.parent.stat()

        def fail_final_flush(fd):
            current = os.fstat(fd)
            if (current.st_dev, current.st_ino) == (parent.st_dev, parent.st_ino):
                self.assertTrue(self.output.is_dir())
                raise OSError(errno.EIO, "fixture parent flush refusal")
            flush(fd)

        with patch.object(os, "fsync", fail_final_flush):
            with self.assertRaises(producer.PublicationUncertainError):
                producer.stage_payload(self.output, self.payload, None)
        self.assert_complete()
        with self.assertRaises(FileExistsError):
            producer.stage_payload(self.output, {"different": b"new"}, None)
        self.assert_complete()

    def test_pack_group_flush_covers_nested_entries_before_rename(self):
        sdk = self.parent / "source-sdk"
        sdk.mkdir()
        (sdk / "tool.exe").write_bytes(b"fixture")
        pins = {"tool.exe": sha256(b"fixture").hexdigest()}
        flush = os.fsync

        def fail_complete_payload_group(fd):
            if stat.S_ISDIR(os.fstat(fd).st_mode) and set(os.listdir(fd)) == {"first", "Assets"}:
                self.assertFalse(self.output.exists())
                raise OSError(errno.EIO, "fixture pack payload directory flush refusal")
            flush(fd)

        with patch.object(producer, "SDK_SHA256", pins):
            with patch.object(os, "fsync", fail_complete_payload_group):
                with self.assertRaisesRegex(OSError, "pack payload directory flush refusal"):
                    producer.stage_payload(self.output, self.payload, sdk)
            self.assertEqual(list(self.parent.iterdir()), [sdk])
            producer.stage_payload(self.output, self.payload, sdk)
        self.assertEqual((self.output / "payload/Assets/second").read_bytes(), b"two")
        self.assertEqual((self.output / "sdk/tool.exe").read_bytes(), b"fixture")
