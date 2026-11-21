"""Portable boundary controls for the maintained Windows event-resource link."""

from __future__ import annotations

import hashlib
import importlib
import json
import os
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import unittest

REPOSITORY = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(REPOSITORY))

from metadata.tests.test_pipeline import SourceFixture


def resource_record(kind: bytes, payload: bytes, *, language: int = 1033) -> bytes:
    """Construct independent MS RC records with intentionally inert payloads."""
    fields = kind + struct.pack("<HHIHHII", 0xFFFF, 1, 0, 0x30, language, 0, 0)
    return (struct.pack("<II", len(payload), len(fields) + 8) + fields
            + payload + b"\0" * (-len(payload) % 4))


class EventResourceTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        sys.path.insert(0, str(REPOSITORY))
        try:
            cls.builder = importlib.import_module("metadata.build_event_resource")
            cls.parser = importlib.import_module("metadata.event_resource_pe")
        finally:
            sys.path.remove(str(REPOSITORY))

    def test_compiler_receipt_is_bound_to_the_artifact_and_manifest(self) -> None:
        manifest = b"test-only manifest bytes"
        digest = hashlib.sha256(manifest).hexdigest()
        command = {"pid": 7, "exit_code": 0, "reaped": True, "timed_out": False,
                   "kill_attempted": False, "error": None}
        receipt = {
            "status": "compiled", "failure": None, "copied_inputs_rechecked": True,
            "scratch_cleanup": "removed",
            "input_sha256": {**self.builder.SDK_TOOLS, "service-events.man": digest},
            "commands": {
                "mc": {**command, "image": "mc.exe", "arguments": "-um service-events.man"},
                "rc": {**command, "image": "rc.exe",
                       "arguments": "/x /fo service-events.res service-events.rc"},
            },
            "artifact_count": 5,
            "artifacts": [{"name": name, "bytes": 1, "sha256": digest} for name in (
                "service-events.res", "MSG00001.bin", "service-eventsTEMP.BIN",
                "service-events.h", "service-events.rc")],
        }

        def verify(record: dict, *, selected: bytes = manifest) -> dict:
            raw = json.dumps(record).encode("utf-8")
            return self.builder._compiler_artifacts(raw, hashlib.sha256(raw).hexdigest(), selected)

        self.assertEqual(verify(receipt)["service-events.res"], (1, digest))
        with self.assertRaisesRegex(self.builder.MetadataError, "event-compiler-receipt-digest"):
            self.builder._compiler_artifacts(json.dumps(receipt).encode("utf-8"), "0" * 64, manifest)
        with self.assertRaisesRegex(self.builder.MetadataError, "event-compiler-receipt-inputs"):
            verify(receipt, selected=manifest + b" changed")
        with self.assertRaisesRegex(self.builder.MetadataError, "event-compiler-receipt-artifacts"):
            verify({**receipt, "artifacts": [*receipt["artifacts"][:-1], receipt["artifacts"][0]]})
        for changed in ({"exit_code": False}, {"timed_out": True}, {"reaped": False},
                        {"kill_attempted": True}, {"arguments": "/x /fo other.res other.rc"}):
            with self.subTest(changed=changed):
                commands = receipt["commands"]
                with self.assertRaisesRegex(self.builder.MetadataError, "event-compiler-receipt-commands"):
                    verify({**receipt, "commands": {**commands, "rc": {**commands["rc"], **changed}}})

    def test_resource_object_refuses_missing_foreign_and_trailing_records(self) -> None:
        marker = struct.pack("<IIHHHHIIII", 0, 32, 0xFFFF, 0, 0xFFFF, 0, 0, 0, 0, 0)
        message = resource_record(b"\xff\xff\x0b\0", b"MSG")
        template = resource_record("WEVT_TEMPLATE\0".encode("utf-16-le"), b"WEVT")
        valid = marker + message + template
        self.assertEqual(self.parser.read_resource_object(valid), {
            (11, 1, 1033): b"MSG", ("WEVT_TEMPLATE", 1, 1033): b"WEVT",
        })
        foreign_language = resource_record(b"\xff\xff\x0b\0", b"MSG", language=1031)
        for candidate in (marker + message, valid + message, valid + b"\0", valid[:-1],
                          marker + foreign_language + template, marker + template + message):
            with self.subTest(candidate=candidate):
                with self.assertRaises(self.builder.MetadataError):
                    self.parser.read_resource_object(candidate)

    def test_final_publication_preserves_a_late_empty_destination(self) -> None:
        with tempfile.TemporaryDirectory(prefix="nevrotick-event-publish-") as directory:
            root = Path(directory)
            staging, output = root / "staging", root / "output"
            staging.mkdir()
            (staging / "receipt.json").write_bytes(b"fixture")
            parent = self.builder._open_directory(root, private=True)
            try:
                self.builder._require_absent(parent, output.name)
                output.mkdir()
                original = output.stat()
                with self.assertRaisesRegex(self.builder.MetadataError, "event-output-already-exists"):
                    self.builder._publish_new(parent, staging.name, output.name)
                self.assertEqual(output.stat().st_ino, original.st_ino)
                self.assertEqual(list(output.iterdir()), [])
                self.assertEqual((staging / "receipt.json").read_bytes(), b"fixture")
                output.rmdir()
                self.builder._publish_new(parent, staging.name, output.name)
                self.assertFalse(staging.exists())
                self.assertEqual((output / "receipt.json").read_bytes(), b"fixture")
            finally:
                os.close(parent)


class EventProviderlessBuildTests(SourceFixture):
    def test_default_source_refuses_without_linking_or_creating_output(self) -> None:
        self.generate()
        target: Path = self.base / "resource-dll"
        result = subprocess.run([
            sys.executable, "-I", "-B", str(REPOSITORY / "metadata/build_event_resource.py"),
            "--root", str(self.root), "--generation", str(self.output),
            "--resource-object", str(self.base / "unused.res"),
            "--compiler-receipt", str(self.base / "unused.json"),
            "--compiler-receipt-sha256", "0" * 64, "--output", str(target),
        ], capture_output=True, text=True, check=False, timeout=15)
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertEqual(result.stdout, "")
        self.assertEqual(result.stderr.strip(), "Event resource error: event-provider-unselected")
        self.assertFalse(target.exists())
