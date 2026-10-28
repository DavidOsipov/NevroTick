"""Challenge workflow tool acquisition without executing unadmitted payloads."""

import hashlib
import importlib.util
import io
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("workflow_checks", Path(__file__).with_name("check_workflows.py"))
CHECK = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECK)


class AcquisitionControls(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="workflow-admission-control-")
        self.addCleanup(self.temporary.cleanup)
        self.directory = Path(self.temporary.name)
        self.payload = b"fixture payload, never executable"
        self.entry = {"name": "actionlint", "member": "release/actionlint",
                      "url": "https://github.com/example/tool/releases/download/v1/tool.tar.gz",
                      "binary_sha256": hashlib.sha256(self.payload).hexdigest()}

    def archive(self, *, symlink=False, duplicate=False):
        output = io.BytesIO()
        with tarfile.open(fileobj=output, mode="w:gz") as archive:
            info = tarfile.TarInfo(self.entry["member"])
            if symlink:
                info.type = tarfile.SYMTYPE
                info.linkname = "../../outside"
                archive.addfile(info)
            else:
                info.size = len(self.payload)
                archive.addfile(info, io.BytesIO(self.payload))
                if duplicate:
                    archive.addfile(info, io.BytesIO(self.payload))
        data = output.getvalue()
        self.entry["archive_sha256"] = hashlib.sha256(data).hexdigest()
        return data

    def acquire(self, data):
        with patch.object(CHECK.urllib.request, "urlopen", return_value=io.BytesIO(data)):
            return CHECK.acquire(self.entry, self.directory)

    def test_expected_regular_binary_is_the_only_written_payload(self):
        binary = self.acquire(self.archive())
        self.assertEqual(binary.read_bytes(), self.payload)
        self.assertEqual(CHECK.admit(self.entry, self.directory), binary)
        self.assertFalse((self.directory / "release").exists())

    def test_corrupt_archive_is_rejected_before_archive_parsing(self):
        data = self.archive() + b"unexpected mutation"
        with patch.object(CHECK.tarfile, "open") as parser:
            with self.assertRaisesRegex(CHECK.ENV.Incomplete, "archive digest"):
                self.acquire(data)
            parser.assert_not_called()

    def test_symlink_and_duplicate_members_are_rejected(self):
        for options in ({"symlink": True}, {"duplicate": True}):
            with self.subTest(options=options):
                with self.assertRaisesRegex(CHECK.ENV.Incomplete, "regular archive member"):
                    self.acquire(self.archive(**options))
                self.assertFalse((self.directory / "actionlint").exists())

    def test_wrong_member_payload_is_rejected_before_executable_write(self):
        data = self.archive()
        self.entry["binary_sha256"] = "0" * 64
        with self.assertRaisesRegex(CHECK.ENV.Incomplete, "binary digest"):
            self.acquire(data)
        self.assertFalse((self.directory / "actionlint").exists())

    def test_reused_tool_is_admitted_by_content(self):
        self.acquire(self.archive())
        (self.directory / "actionlint").write_bytes(b"replaced executable")
        with self.assertRaisesRegex(CHECK.ENV.Incomplete, "executable digest"):
            CHECK.admit(self.entry, self.directory)


if __name__ == "__main__":
    unittest.main()
