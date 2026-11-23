# SPDX-License-Identifier: GPL-2.0-only
"""Exercise the real publication boundary in an owned temporary directory."""

from contextlib import contextmanager, redirect_stderr, redirect_stdout
import errno
from hashlib import sha256
import io
import json
import os
from pathlib import Path
import stat
import sys
import tempfile
import unittest
from unittest.mock import patch

import prepare_payload as producer
from metadata.tests.test_pipeline import SourceFixture


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


class EventResourceTests(SourceFixture):
    """Use the real generation, CLI arguments, receipt checks and staging boundary."""

    def setUp(self):
        super().setUp()
        self.resource_name = "nevrotick-test-events.dll"
        self.provider_guid = "407b0d7f-0220-4ca1-9e6b-d0a6c0d33380"
        self.event_source = self.record.with_name("events.toml")
        self.event_source.write_text(self.original_record.replace("[fields]", (
            '[diagnostics.event_provider]\n'
            'name = "NevroTick-TestService"\n'
            f'guid = "{self.provider_guid}"\n'
            f'package_resource_file = "{self.resource_name}"\n\n'
            '[fields]'
        ), 1), encoding="utf-8")
        self.require_success(self.cli("generate", "--source", str(self.event_source), output=self.output))
        self.build = self.base / "resource-build"
        self.build.mkdir()
        self.receipt = self.build / "receipt.json"
        self.image = b"inert synthetic image bytes"
        self.image_path = self.build / self.resource_name
        self.image_path.write_bytes(self.image)
        self.readback = {"provider_guid": self.provider_guid, "message_ids": [0x40000001]}
        self.receipt_record = {
            "schema_version": 1,
            "status": "linked-and-read-back",
            "target": "x86_64-pc-windows-msvc",
            "manifest_sha256": sha256((self.output / "authoring/service-events.man").read_bytes()).hexdigest(),
            "generation_receipt_sha256": sha256((self.output / "receipt.json").read_bytes()).hexdigest(),
            "file": self.resource_name,
            "bytes": len(self.image),
            "sha256": sha256(self.image).hexdigest(),
            "readback": self.readback,
        }
        self.write_receipt()
        self.inert = self.base / "inert.exe"
        self.inert.write_bytes(b"inert application")
        self.pack = self.base / "pack"
        self.role_images = {name + ".exe": name.encode() for name in producer.BINS}

    def write_receipt(self, **changes):
        self.receipt.write_bytes(json.dumps({**self.receipt_record, **changes}).encode())

    def event_options(self, receipt_hash=None):
        return ["--event-resource-build", str(self.build),
                "--event-resource-receipt-sha256", receipt_hash or sha256(self.receipt.read_bytes()).hexdigest(),
                "--event-resource-generation", str(self.output),
                "--event-resource-source-root", str(self.root),
                "--event-resource-source", str(self.event_source)]

    def run_pack(self, options, *, exit_code=None, output=None, synthetic_readback=True):
        args = [str(producer.HERE / "prepare_payload.py"),
                "--receipt", str(self.base / "unused.json"), "--receipt-sha256", "0" * 64,
                "--inert", str(self.inert), "--inert-sha256", sha256(self.inert.read_bytes()).hexdigest(),
                "--output", str(output or self.pack), *options]
        stdout, stderr = io.StringIO(), io.StringIO()
        with (
            patch.object(sys, "argv", args),
            patch.object(producer, "role_images", return_value=dict(self.role_images)),
            redirect_stdout(stdout),
            redirect_stderr(stderr),
            self._resource_readers(synthetic_readback),
        ):
            if exit_code is None:
                producer.main()
            else:
                with self.assertRaises(SystemExit) as outcome:
                    producer.main()
                self.assertEqual(outcome.exception.code, exit_code, stderr.getvalue())
                self.assertEqual(stdout.getvalue(), "")
        return stdout.getvalue(), stderr.getvalue()

    @contextmanager
    def _resource_readers(self, synthetic_readback):
        """Substitute only successful parsing of the synthetic image when selected."""
        if not synthetic_readback:
            yield
            return
        leaves = {(11, 1, 1033): b"synthetic message"}

        def read_image(image):
            self.assertEqual(image, self.image)
            return leaves

        def verify(provider, observed_leaves):
            self.assertEqual(provider.guid, self.provider_guid)
            self.assertEqual(observed_leaves, leaves)
            return self.readback

        with (patch.object(producer, "read_resource_dll", read_image),
              patch.object(producer, "verify_event_resources", verify)):
            yield

    def test_explicit_resource_enters_only_the_payload_inventory(self):
        default = self.base / "default"
        default_report, _ = self.run_pack([], output=default)
        self.assertNotIn(self.resource_name, json.loads(default_report))
        self.assertEqual(set(json.loads(default_report)), {
            *self.role_images, "AppxManifest.xml", "Inert.exe", "Assets/PackageLogo.png",
            "Assets/Square44x44Logo.png", "Assets/Square150x150Logo.png",
        })

        sdk = self.base / "sdk-source"
        sdk.mkdir()
        (sdk / "test-tool.exe").write_bytes(b"inert sdk tool")
        with patch.object(producer, "SDK_SHA256", {"test-tool.exe": sha256(b"inert sdk tool").hexdigest()}):
            selected_report, _ = self.run_pack([*self.event_options(), "--sdk-source", str(sdk)])
        self.assertEqual((self.pack / "payload" / self.resource_name).read_bytes(), self.image)
        expected = {**json.loads(default_report), self.resource_name: sha256(self.image).hexdigest()}
        published = json.loads((self.pack / "input-set.json").read_bytes())
        self.assertEqual(published["payload"], expected)
        self.assertEqual(json.loads(selected_report)["payload"], expected)
        self.assertEqual({path.relative_to(self.pack / "payload").as_posix()
                          for path in (self.pack / "payload").rglob("*") if path.is_file()}, expected.keys())

    def test_incomplete_event_selection_refuses_before_publication(self):
        options = self.event_options()
        for start in range(0, 8, 2):
            with self.subTest(missing=options[start]):
                _, stderr = self.run_pack(options[:start] + options[start + 2:], exit_code=2)
                self.assertIn("require", stderr)
        self.run_pack(options[-2:], exit_code=2)
        self.assertFalse(self.pack.exists())

    def test_resealed_receipt_cannot_replace_link_and_readback_facts(self):
        for changes, reason in (
            ({"status": "compiled"}, "receipt profile"),
            ({"schema_version": True}, "receipt profile"),
            ({"target": "aarch64-pc-windows-msvc"}, "receipt profile"),
            ({"manifest_sha256": "0" * 64}, "generation"),
            ({"generation_receipt_sha256": "0" * 64}, "generation"),
            ({"file": "other.dll"}, "receipt profile"),
            ({"bytes": float(len(self.image))}, "DLL size or digest"),
            ({"sha256": "0" * 64}, "DLL size or digest"),
            ({"readback": {**self.readback, "message_ids": [float(0x40000001)]}}, "readback"),
        ):
            with self.subTest(changes=changes):
                self.write_receipt(**changes)
                _, stderr = self.run_pack(self.event_options(), exit_code=1)
                self.assertIn(reason, stderr)
                self.assertFalse(self.pack.exists())

    def test_readback_and_input_custody_fail_without_touching_output(self):
        self.pack.mkdir()
        (self.pack / "prior").write_bytes(b"previous contents")
        options = self.event_options()
        _, stderr = self.run_pack(options, synthetic_readback=False, exit_code=1)
        self.assertIn("Event resource:", stderr)

        _, stderr = self.run_pack(self.event_options("0" * 64), exit_code=1)
        self.assertIn("receipt SHA-256", stderr)

        extra = self.build / "private.txt"
        extra.write_bytes(b"exclude from package")
        _, stderr = self.run_pack(options, exit_code=1)
        self.assertIn("build inventory", stderr)
        extra.unlink()

        moved = self.base / "moved.dll"
        self.image_path.rename(moved)
        self.image_path.symlink_to(moved)
        self.run_pack(options, exit_code=1)
        self.image_path.unlink()
        os.link(moved, self.image_path)
        _, stderr = self.run_pack(options, exit_code=1)
        self.assertIn("output-file-custody", stderr)
        self.assertEqual(list(self.pack.iterdir()), [self.pack / "prior"])
        self.assertEqual((self.pack / "prior").read_bytes(), b"previous contents")

    def test_changed_generation_and_ambiguous_receipt_are_refused(self):
        options = self.event_options()
        manifest = self.output / "authoring/service-events.man"
        original = manifest.read_bytes()
        manifest.write_bytes(original + b"\n")
        _, stderr = self.run_pack(options, exit_code=1)
        self.assertIn("output-content-mismatch", stderr)
        manifest.write_bytes(original)

        receipt = self.receipt.read_bytes()
        self.receipt.write_bytes(b'{"file":"other.dll",' + receipt[1:])
        _, stderr = self.run_pack(self.event_options(), exit_code=1)
        self.assertIn("delivery-duplicate-json-key", stderr)
        self.assertFalse(self.pack.exists())

    def test_publication_writes_the_checked_snapshot_after_input_changes(self):
        stage_payload = producer.stage_payload

        def change_source_after_readback(output, payload, sdk_source):
            self.image_path.write_bytes(b"different on disk")
            return stage_payload(output, payload, sdk_source)

        with patch.object(producer, "stage_payload", change_source_after_readback):
            report, _ = self.run_pack(self.event_options())
        self.assertEqual(json.loads(report)[self.resource_name], sha256(self.image).hexdigest())
        self.assertEqual((self.pack / self.resource_name).read_bytes(), self.image)
        self.assertEqual(self.image_path.read_bytes(), b"different on disk")
