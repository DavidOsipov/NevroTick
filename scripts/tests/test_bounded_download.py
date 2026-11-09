"""Offline byte, authority, atomicity and real CLI controls for acquisition."""
from __future__ import annotations

import contextlib
from email.message import Message
import hashlib
import importlib.util
import io
import os
from pathlib import Path
import ssl
import tempfile
import unittest
from unittest import mock
import urllib.error

SPEC = importlib.util.spec_from_file_location(
    "bounded_acquisition_controls", Path(__file__).resolve().parents[1] / "bounded_download.py"
)
assert SPEC is not None and SPEC.loader is not None
DOWNLOAD = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(DOWNLOAD)


def digest(body):
    return hashlib.sha256(body).hexdigest()


class Response(io.BytesIO):
    status = 200

    def __init__(self, body, headers=()):
        super().__init__(body)
        self.headers = Message()
        for key, value in headers:
            self.headers.add_header(key, value)
        self.requests = []

    def read1(self, size):
        self.requests.append(size)
        if not 0 < size <= DOWNLOAD.CHUNK_BYTES:
            raise AssertionError("network read exceeded bounded chunk")
        return self.read(size)


class BoundedDownloadControls(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-download-control-")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.target = self.root / "artifact.bin"
        self.url = "https://example.invalid/reviewed.bin"

    def acquire(self, body, *, maximum=8, headers=(), expected=None):
        response = Response(body, headers)
        with mock.patch.object(DOWNLOAD, "_open", return_value=response):
            result = DOWNLOAD.download_to_file(self.url, self.target,
                                                digest(body) if expected is None else expected,
                                                maximum_bytes=maximum)
        return result, response

    def assert_previous_preserved(self):
        self.assertEqual(self.target.read_bytes(), b"old")
        self.assertEqual({p.name for p in self.root.iterdir()}, {self.target.name})

    def test_exact_limit_and_limit_minus_one_publish_exact_bytes(self):
        for body in (b"1234567", b"12345678"):
            with self.subTest(bytes=len(body)):
                self.target.unlink(missing_ok=True)
                result, response = self.acquire(body)
                self.assertEqual(self.target.read_bytes(), body)
                self.assertEqual(result, {"bytes": len(body), "sha256": digest(body), "cached": False})
                self.assertTrue(all(0 < size <= 9 for size in response.requests))
                self.assertEqual({p.name for p in self.root.iterdir()}, {self.target.name})

    def test_unknown_length_limit_plus_one_is_rejected_without_publication(self):
        with self.assertRaisesRegex(RuntimeError, "byte limit"):
            self.acquire(b"123456789")
        self.assertEqual(list(self.root.iterdir()), [])

    def test_false_small_length_does_not_bypass_actual_byte_limit(self):
        self.target.write_bytes(b"old")
        with self.assertRaisesRegex(RuntimeError, "byte limit"):
            self.acquire(b"123456789", headers=(("Content-Length", "1"),))
        self.assert_previous_preserved()

    def test_declared_oversize_is_rejected_before_body_reads(self):
        response = Response(b"x", (("Content-Length", "9"),))
        with mock.patch.object(DOWNLOAD, "_open", return_value=response):
            with self.assertRaisesRegex(RuntimeError, "byte limit"):
                DOWNLOAD.download_to_file(self.url, self.target, digest(b"x"), maximum_bytes=8)
        self.assertEqual(response.requests, [])
        self.assertEqual(list(self.root.iterdir()), [])

    def test_inconsistent_or_ambiguous_headers_preserve_previous_file(self):
        self.target.write_bytes(b"old")
        cases = [(("Content-Length", "8"),), (("Content-Length", "-1"),),
                 (("Content-Length", "1"), ("Content-Length", "1")),
                 (("Content-Encoding", "gzip"),)]
        for headers in cases:
            with self.subTest(headers=headers), self.assertRaises(RuntimeError):
                self.acquire(b"a", headers=headers)
            self.assert_previous_preserved()

    def test_digest_refusal_preserves_existing_file_and_removes_temporary(self):
        self.target.write_bytes(b"old")
        with self.assertRaisesRegex(RuntimeError, "SHA256"):
            self.acquire(b"new", expected=digest(b"expected"))
        self.assert_previous_preserved()

    def test_read_failure_preserves_existing_file_and_removes_temporary(self):
        self.target.write_bytes(b"old")
        response = Response(b"ignored")
        with mock.patch.object(response, "read1", side_effect=OSError("synthetic read failure")), \
             mock.patch.object(DOWNLOAD, "_open", return_value=response):
            with self.assertRaises(OSError):
                DOWNLOAD.download_to_file(self.url, self.target, digest(b"expected"), maximum_bytes=8)
        self.assert_previous_preserved()

    def test_cached_artifact_is_bounded_and_hashed_without_network(self):
        self.target.write_bytes(b"cached")
        with mock.patch.object(DOWNLOAD, "_open") as network:
            result = DOWNLOAD.download_to_file(self.url, self.target, digest(b"cached"), maximum_bytes=8)
            network.assert_not_called()
        self.assertTrue(result["cached"])
        self.assertEqual(DOWNLOAD.bounded_file_bytes(self.target, 8), b"cached")
        self.target.write_bytes(b"oversized")
        with mock.patch.object(DOWNLOAD, "_open") as network, self.assertRaises(RuntimeError):
            DOWNLOAD.download_to_file(self.url, self.target, digest(b"oversized"), maximum_bytes=8)
        network.assert_not_called()

    def test_symlink_destination_does_not_modify_its_referent(self):
        victim = self.root / "victim"
        victim.write_bytes(b"old")
        self.target.symlink_to(victim)
        with mock.patch.object(DOWNLOAD, "_open") as network, self.assertRaises(RuntimeError):
            DOWNLOAD.download_to_file(self.url, self.target, digest(b"new"), maximum_bytes=8)
        network.assert_not_called()
        self.assertEqual(victim.read_bytes(), b"old")

    def test_nonregular_cached_body_is_refused_without_blocking(self):
        fifo = self.root / "fifo"
        os.mkfifo(fifo)
        with self.assertRaises(RuntimeError):
            DOWNLOAD.bounded_file_bytes(fifo, 8)

    def test_invalid_authorities_and_origins_make_no_request(self):
        cases = ["http://example.invalid/body", "https://user@example.invalid/body",
                 "https://example.invalid:444/body", "https://example.invalid/body#fragment",
                 "https://example.invalid\\@other.invalid/body", "https://example.invalid/\nbody",
                 "file:///tmp/input", "https://[::1]/body"]
        for url in cases:
            with self.subTest(url=url), mock.patch.object(DOWNLOAD, "_open") as network:
                with self.assertRaises(RuntimeError):
                    DOWNLOAD.download_to_file(url, self.target, digest(b"new"), maximum_bytes=8)
                network.assert_not_called()
        with mock.patch.object(DOWNLOAD, "_open") as network, self.assertRaises(RuntimeError):
            DOWNLOAD.download_to_file(self.url, self.target, digest(b"new"), maximum_bytes=8,
                                      allowed_origins=("https://other.invalid",))
        network.assert_not_called()

    def redirect(self, target, *, code=302):
        headers = Message()
        headers.add_header("Location", target)
        return urllib.error.HTTPError(self.url, code, "redirect", headers, io.BytesIO())

    def test_unadmitted_redirect_is_refused_before_second_outbound_request(self):
        for target in ("https://127.0.0.1/private.docx", "https://evil.invalid/payload",
                       "http://example.invalid/body", "https://example.invalid:8443/body"):
            with self.subTest(target=target), \
                 mock.patch.object(DOWNLOAD, "_open", side_effect=self.redirect(target)) as network:
                with self.assertRaises(RuntimeError):
                    DOWNLOAD.download_to_file(self.url, self.target, digest(b"new"), maximum_bytes=8)
                self.assertEqual(network.call_count, 1)
                self.assertEqual(list(self.root.iterdir()), [])

    def test_relative_redirect_and_explicit_cross_origin_have_healthy_controls(self):
        for target, allowed in (("/same-origin", ()),
                                ("https://assets.invalid/file", ("https://example.invalid", "https://assets.invalid"))):
            self.target.unlink(missing_ok=True)
            with self.subTest(target=target), mock.patch.object(
                    DOWNLOAD, "_open", side_effect=[self.redirect(target), Response(b"new")]) as network:
                DOWNLOAD.download_to_file(self.url, self.target, digest(b"new"), maximum_bytes=8,
                                          allowed_origins=allowed)
                self.assertEqual(network.call_count, 2)
                self.assertEqual(self.target.read_bytes(), b"new")
                expected = "https://example.invalid/same-origin" if target.startswith("/") else target
                self.assertEqual(network.call_args.args[0].full_url, expected)

    def test_redirect_loop_has_finite_request_budget_and_cleanup(self):
        with mock.patch.object(DOWNLOAD, "_open", side_effect=lambda *_: (_ for _ in ()).throw(
                self.redirect("/loop"))) as network:
            with self.assertRaisesRegex(RuntimeError, "redirect"):
                DOWNLOAD.download_to_file(self.url, self.target, digest(b"new"), maximum_bytes=8)
            self.assertEqual(network.call_count, DOWNLOAD.MAXIMUM_REDIRECTS + 1)
        self.assertEqual(list(self.root.iterdir()), [])

    def test_elapsed_limit_is_checked_even_after_delayed_eof(self):
        response = Response(b"")
        with mock.patch.object(DOWNLOAD, "_open", return_value=response), \
             mock.patch.object(DOWNLOAD.time, "monotonic", side_effect=[0, 0, 121]):
            with self.assertRaisesRegex(RuntimeError, "elapsed-time"):
                DOWNLOAD.download_to_file(self.url, self.target, digest(b""), maximum_bytes=8)
        self.assertEqual(list(self.root.iterdir()), [])

    def test_tls_minimum_and_manual_redirect_handler_are_applied(self):
        opener = mock.Mock()
        with mock.patch.object(DOWNLOAD.urllib.request, "build_opener", return_value=opener) as build:
            DOWNLOAD._open(DOWNLOAD.urllib.request.Request(self.url), 12)
        manual, https = build.call_args.args
        self.assertIsInstance(manual, DOWNLOAD.NoAutomaticRedirect)
        self.assertIsNone(manual.redirect_request(None, None, 302, "", {}, "https://evil.invalid"))
        self.assertGreaterEqual(https._context.minimum_version, ssl.TLSVersion.TLSv1_2)
        self.assertEqual(opener.open.call_args.kwargs["timeout"], 12)

    def test_real_cli_writes_verified_artifact_and_refuses_oversize_without_publication(self):
        args = ["--url", self.url, "--destination", str(self.target), "--sha256", digest(b"new"),
                "--maximum-bytes", "8", "--allowed-origin", "https://example.invalid"]
        with mock.patch.object(DOWNLOAD, "_open", return_value=Response(b"new")), \
             contextlib.redirect_stdout(io.StringIO()) as stdout:
            self.assertEqual(DOWNLOAD.main(args), 0)
        self.assertEqual(self.target.read_bytes(), b"new")
        self.assertIn("3 bytes", stdout.getvalue())
        self.target.unlink()
        with mock.patch.object(DOWNLOAD, "_open", return_value=Response(b"oversized")), \
             contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit) as failed:
            DOWNLOAD.main(args)
        self.assertEqual(failed.exception.code, 1)
        self.assertEqual(list(self.root.iterdir()), [])


if __name__ == "__main__":
    unittest.main()
