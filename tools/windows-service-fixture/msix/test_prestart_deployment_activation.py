# SPDX-License-Identifier: GPL-2.0-only

from dataclasses import replace
from hashlib import sha256
from pathlib import Path
from tempfile import TemporaryDirectory
from time import monotonic
import unittest
from unittest.mock import patch
from uuid import UUID

from prestart_deployment_activation import publish_deployment_activation
from prestart_deployment_owner import RenderedDeploymentOwner
from prestart_receipts import Member, ReceiptRefused, Run


class DeploymentActivationTests(unittest.TestCase):
    def setUp(self) -> None:
        directory = TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.mapping = Path(directory.name)
        self.frame = self.mapping / "deployment-activation.bin"
        self.commit = self.mapping / "deployment-activation.bin.commit"
        run = Run(
            UUID("716046d4-77af-4664-9ef7-b218ca943b08"),
            "NevroTick.S06.ScmFixture_1.0.0.0_x64__0123456789abc",
            "1" * 64, Member(31, "2" * 64), Member(32, "3" * 64),
            "4" * 64, "5" * 64, object(), object(), object(),
        )
        contents = b"# independent activation payload\n"
        self.entry = RenderedDeploymentOwner(run, contents, Member(len(contents), sha256(contents).hexdigest()))
        self.work = monotonic() + 3
        self.retirement = self.work + 1

    def test_commit_contains_the_original_header_and_entry_and_cannot_be_replaced(self) -> None:
        publish_deployment_activation(self.mapping, self.entry, self.work, self.retirement)
        expected = (
            f"nevrotick-prestart-activation-v1 716046d4-77af-4664-9ef7-b218ca943b08 "
            f"33 {self.entry.pin.sha256}\n# independent activation payload\n"
        ).encode("ascii")
        self.assertEqual(self.frame.read_bytes(), expected)
        self.assertEqual(self.commit.read_bytes(), b"")
        with self.assertRaisesRegex(ReceiptRefused, "already committed"):
            publish_deployment_activation(self.mapping, self.entry, self.work, self.retirement)
        self.assertEqual(self.frame.read_bytes(), expected)

    def test_refuses_wrong_pin_and_expired_input_without_writing(self) -> None:
        wrong = replace(self.entry, pin=Member(self.entry.pin.size, "f" * 64))
        with self.assertRaisesRegex(ReceiptRefused, "pin differs"):
            publish_deployment_activation(self.mapping, wrong, self.work, self.retirement)
        with self.assertRaises(TimeoutError):
            publish_deployment_activation(self.mapping, self.entry, 1, 2)
        self.assertEqual(list(self.mapping.iterdir()), [])

    def test_boolean_extent_does_not_alias_a_canonical_one_byte_extent(self) -> None:
        digest = sha256(b"x").hexdigest()
        entry = replace(self.entry, contents=b"x", pin=Member(True, digest))
        with self.assertRaisesRegex(ReceiptRefused, "pin differs"):
            publish_deployment_activation(self.mapping, entry, self.work, self.retirement)
        self.assertFalse(self.frame.exists())
        publish_deployment_activation(self.mapping, replace(entry, pin=Member(1, digest)), self.work, self.retirement)
        self.assertIn(f" 1 {digest}\nx".encode("ascii"), self.frame.read_bytes())

    def test_failed_flush_cannot_publish_the_commit(self) -> None:
        with patch("prestart_deployment_activation.os.fsync", side_effect=OSError("controlled flush refusal")):
            with self.assertRaisesRegex(OSError, "controlled flush refusal"):
                publish_deployment_activation(self.mapping, self.entry, self.work, self.retirement)
        self.assertTrue(self.frame.exists())
        self.assertFalse(self.commit.exists())

    def test_occupied_partial_record_cannot_be_reused(self) -> None:
        self.frame.write_bytes(b"incomplete")
        with self.assertRaises(FileExistsError):
            publish_deployment_activation(self.mapping, self.entry, self.work, self.retirement)
        self.assertEqual(self.frame.read_bytes(), b"incomplete")
        self.assertFalse(self.commit.exists())

    def test_cutoff_before_commit_leaves_only_uncommitted_bytes(self) -> None:
        def remaining(_deadline: float) -> None:
            if self.frame.exists():
                raise TimeoutError("expiry after data write")

        with patch("prestart_deployment_activation._remaining", side_effect=remaining):
            with self.assertRaisesRegex(TimeoutError, "expiry after data write"):
                publish_deployment_activation(self.mapping, self.entry, self.work, self.retirement)
        self.assertTrue(self.frame.exists())
        self.assertFalse(self.commit.exists())

    def test_commit_ends_publication_even_if_the_clock_then_expires(self) -> None:
        def remaining(_deadline: float) -> None:
            if self.commit.exists():
                raise TimeoutError("expiry after commit")

        with patch("prestart_deployment_activation._remaining", side_effect=remaining):
            publish_deployment_activation(self.mapping, self.entry, self.work, self.retirement)
        self.assertTrue(self.frame.exists())
        self.assertTrue(self.commit.exists())
