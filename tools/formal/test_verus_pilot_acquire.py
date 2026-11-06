"""Exercise bounded acquisition with local fixtures; never download or execute."""
from __future__ import annotations

import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import stat
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch
import warnings
import zipfile


HERE = Path(__file__).resolve().parent
SPEC = importlib.util.spec_from_file_location("verus_pilot_acquire_test", HERE / "verus_pilot_acquire.py")
ACQUIRE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ACQUIRE)

ELF_FIXTURE = b"\x7fELF\x02\x01" + b"\0" * 12 + b"\x3e\x00fixture-only"


def member(name, contents=b"fixture", *, kind=stat.S_IFREG):
    entry = zipfile.ZipInfo(name)
    entry.create_system = 3
    entry.external_attr = (kind | 0o600) << 16
    return entry, contents


def healthy_members():
    return [member("r/" + name, ELF_FIXTURE) for name in ("verus", "rust_verify", "z3")]


def zip_bytes(entries):
    output = io.BytesIO()
    with warnings.catch_warnings(), zipfile.ZipFile(output, "w") as archive:
        warnings.simplefilter("ignore", UserWarning)
        for entry, contents in entries:
            archive.writestr(entry, contents)
    return output.getvalue()


class LocalFixtures(unittest.TestCase):
    def setUp(self):
        import tempfile

        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-verus-acquire-controls-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.opener = Mock()
        self.opener.open.side_effect = AssertionError("Network is forbidden in acquisition controls")
        self.addCleanup(patch.stopall)
        patch.object(ACQUIRE.urllib.request, "build_opener", return_value=self.opener).start()

    def inspect(self, entries):
        with zipfile.ZipFile(io.BytesIO(zip_bytes(entries))) as archive:
            return ACQUIRE.members(archive)

    def store_parent(self):
        home = self.base / "home"
        parent = home / ".local/share/nevrotick"
        parent.mkdir(parents=True)
        return home, parent


class ArchiveAdmissionTests(LocalFixtures):
    def test_healthy_namespace_retains_exact_members_and_declared_bytes(self):
        entries = healthy_members() + [member("r/nested/", b"", kind=stat.S_IFDIR),
                                       member("r/nested/NOTICE", b"retained notice")]
        actual, prefix, total = self.inspect(entries)
        self.assertEqual(prefix, "r")
        self.assertEqual([entry.filename for entry in actual], [entry.filename for entry, _ in entries])
        self.assertEqual(total, 3 * len(ELF_FIXTURE) + len(b"retained notice"))
        self.opener.open.assert_not_called()

    def test_unsafe_and_normalized_member_spellings_are_rejected(self):
        for name in ("/r/escape", "r/../escape", "r/./item", "r//item", "./r/item",
                     "r\\item", "r/C:item", "r/line\nbreak", "r/\x7f", "r/caf\u00e9",
                     "r/nested//"):
            kind = stat.S_IFDIR if name.endswith("/") else stat.S_IFREG
            with self.subTest(name=name), self.assertRaises(RuntimeError):
                self.inspect(healthy_members() + [member(name, b"", kind=kind)])

    def test_original_zip_name_cannot_be_silently_truncated_at_nul(self):
        entries = [member("r/verus!BAD", ELF_FIXTURE), *healthy_members()[1:]]
        encoded = zip_bytes(entries).replace(b"r/verus!BAD", b"r/verus\0BAD")
        with zipfile.ZipFile(io.BytesIO(encoded)) as archive:
            entry = archive.infolist()[0]
            self.assertEqual(entry.filename, "r/verus")
            self.assertEqual(entry.orig_filename, "r/verus\0BAD")
            with self.assertRaises(RuntimeError):
                ACQUIRE.members(archive)

    def test_special_types_and_directory_type_mismatches_are_rejected(self):
        cases = [("r/symlink", stat.S_IFLNK), ("r/fifo", stat.S_IFIFO),
                 ("r/socket", stat.S_IFSOCK), ("r/device", stat.S_IFCHR),
                 ("r/block", stat.S_IFBLK), ("r/file", stat.S_IFDIR),
                 ("r/directory/", stat.S_IFREG)]
        for name, kind in cases:
            with self.subTest(name=name, kind=kind), self.assertRaises(RuntimeError):
                self.inspect(healthy_members() + [member(name, b"", kind=kind)])

    def test_duplicates_aliases_and_file_parent_collisions_are_rejected(self):
        cases = [healthy_members() + [member("r/verus")],
                 healthy_members() + [member("r/item"), member("r/item/", b"", kind=stat.S_IFDIR)],
                 healthy_members() + [member("r/parent"), member("r/parent/child")],
                 healthy_members() + [member("r/parent/child"), member("r/parent")]]
        for index, entries in enumerate(cases):
            with self.subTest(case=index), self.assertRaises(RuntimeError):
                self.inspect(entries)

    def test_payload_requires_one_root_and_all_three_executables(self):
        for entries in ([], healthy_members()[:-1],
                        healthy_members() + [member("other/notice")],
                        healthy_members() + [member("top-level-file")]):
            with self.subTest(names=[e.filename for e, _ in entries]), self.assertRaises(RuntimeError):
                self.inspect(entries)

    def test_metadata_rejects_encryption_and_unselected_compression(self):
        for field, value in (("flag_bits", 1), ("compress_type", zipfile.ZIP_BZIP2)):
            entries = [entry for entry, _ in healthy_members()]
            setattr(entries[0], field, value)
            with self.subTest(field=field), self.assertRaises(RuntimeError):
                ACQUIRE.members(SimpleNamespace(infolist=lambda: entries))

    def test_declared_file_and_total_budget_boundaries_without_large_allocations(self):
        entries = [entry for entry, _ in healthy_members()]
        entries[0].file_size = 256 * 1024**2
        # This fixture has three files and one root directory. The contract
        # reserves sixteen further metadata slots, each with an 8192-byte bound.
        metadata_allowance = 20 * 8192
        remaining = ACQUIRE.LIMIT - ACQUIRE.ARCHIVE_BYTES - 16 * 1024**2 - metadata_allowance
        entries[1].file_size = remaining - 1 - entries[0].file_size
        entries[2].file_size = 0
        archive = SimpleNamespace(infolist=lambda: entries)
        self.assertEqual(ACQUIRE.members(archive)[2], remaining - 1)
        entries[1].file_size += 1
        with self.assertRaisesRegex(RuntimeError, "budget"):
            ACQUIRE.members(archive)
        entries[0].file_size += 1
        entries[1].file_size = 0
        with self.assertRaisesRegex(RuntimeError, "file size"):
            ACQUIRE.members(archive)

    def test_entry_budget_has_an_accepted_exact_neighbor(self):
        entries = [entry for entry, _ in healthy_members()]
        entries.extend(member(f"r/d{number}/", b"", kind=stat.S_IFDIR)[0]
                       for number in range(ACQUIRE.MAX_ENTRIES - len(entries)))
        archive = SimpleNamespace(infolist=lambda: entries)
        self.assertEqual(len(ACQUIRE.members(archive)[0]), ACQUIRE.MAX_ENTRIES)
        entries.append(member("r/one-more/", b"", kind=stat.S_IFDIR)[0])
        with self.assertRaisesRegex(RuntimeError, "entry budget"):
            ACQUIRE.members(archive)

    def test_late_invalid_member_blocks_every_extraction_read(self):
        home, parent = self.store_parent()
        encoded = zip_bytes(healthy_members() + [member("r/../escaped", b"must not appear")])

        def local_fetch(url, target, size, expected, folder, deadline):
            target.write_bytes(encoded)

        folder = parent / "candidate"
        with patch.object(ACQUIRE.Path, "home", return_value=home), \
             patch.object(ACQUIRE, "fetch", side_effect=local_fetch), \
             patch.object(zipfile.ZipFile, "open", side_effect=AssertionError("extraction reached")) as extract:
            with self.assertRaises(RuntimeError):
                ACQUIRE.acquire(folder)
        extract.assert_not_called()
        self.assertEqual(list((folder / "payload").iterdir()), [])
        self.assertFalse((folder / "escaped").exists())
        self.assertFalse((parent / "escaped").exists())

    def test_healthy_acquisition_extracts_fixture_bytes_and_records_no_execution(self):
        home, parent = self.store_parent()
        encoded = zip_bytes(healthy_members() + [member("r/nested/NOTICE", b"fixture notice")])
        license_bytes = {name: name.encode() for name in ACQUIRE.LICENSES}

        def local_fetch(url, target, size, expected, folder, deadline):
            target.write_bytes(encoded if target.name == "download.zip" else license_bytes[target.name])

        folder = parent / "candidate"
        with patch.object(ACQUIRE.Path, "home", return_value=home), \
             patch.object(ACQUIRE, "fetch", side_effect=local_fetch) as fetch, \
             patch.object(ACQUIRE.shutil, "disk_usage", return_value=SimpleNamespace(
                 free=ACQUIRE.RESERVE + ACQUIRE.LIMIT)), \
             patch("builtins.print"):
            ACQUIRE.acquire(folder)
        self.assertEqual(fetch.call_count, 1 + len(ACQUIRE.LICENSES))
        self.assertFalse((folder / "download.zip").exists())
        manifest = json.loads((folder / "manifest.json").read_text())
        self.assertEqual(manifest["status"], "acquired-not-executed-not-adopted")
        for name in ACQUIRE.EXECUTABLES:
            self.assertEqual((folder / "payload" / name).read_bytes(), ELF_FIXTURE)
            self.assertEqual(stat.S_IMODE((folder / "payload" / name).stat().st_mode), 0o500)
            self.assertEqual(manifest["payload"][name]["sha256"], hashlib.sha256(ELF_FIXTURE).hexdigest())
        self.assertEqual((folder / "payload/nested/NOTICE").read_bytes(), b"fixture notice")
        self.assertEqual(stat.S_IMODE(folder.stat().st_mode), 0o700)


class UrlAndFetchTests(LocalFixtures):
    def test_only_official_https_endpoints_without_credentials_are_accepted(self):
        for url in (ACQUIRE.ARCHIVE_URL, "https://github.com:443/verus-lang/verus",
                    "https://release-assets.githubusercontent.com/asset?signature=fixture",
                    "https://raw.githubusercontent.com/verus-lang/verus/commit/LICENSE"):
            ACQUIRE.allowed_url(url)
        for url in ("http://github.com/asset", "https://github.com.evil.invalid/asset",
                    "https://evil.invalid/github.com", "https://user@github.com/asset",
                    "https://user:password@github.com/asset", "https://github.com:444/asset",
                    "https://github.com/asset#fragment", "file:///tmp/asset", "//github.com/asset"):
            with self.subTest(url=url), self.assertRaises(RuntimeError):
                ACQUIRE.allowed_url(url)
        self.opener.open.assert_not_called()

    def test_redirect_validation_precedes_following_the_location(self):
        request = ACQUIRE.urllib.request.Request(ACQUIRE.ARCHIVE_URL)
        with patch.object(ACQUIRE.urllib.request.HTTPRedirectHandler, "redirect_request", return_value="accepted") as follow:
            with self.assertRaises(RuntimeError):
                ACQUIRE.Redirects().redirect_request(request, None, 302, "Found", {}, "http://github.com/asset")
            follow.assert_not_called()
            self.assertEqual(ACQUIRE.Redirects().redirect_request(
                request, None, 302, "Found", {}, "https://release-assets.githubusercontent.com/asset"), "accepted")
            follow.assert_called_once()

    def fetch_fixture(self, data, *, size=None, checksum=None, status=200):
        response = io.BytesIO(data)
        response.status = status
        self.opener.open.side_effect = None
        self.opener.open.return_value = response
        target = self.base / "download"
        with patch.object(ACQUIRE, "capacity"), patch.object(ACQUIRE.time, "monotonic", return_value=0):
            ACQUIRE.fetch(ACQUIRE.ARCHIVE_URL, target, len(data) if size is None else size,
                          hashlib.sha256(data).hexdigest() if checksum is None else checksum,
                          self.base, 1)
        return target

    def test_fetch_retains_exact_bytes_and_ignores_ambient_proxy_configuration(self):
        with patch.dict(os.environ, {"HTTPS_PROXY": "http://untrusted.invalid:1234"}):
            target = self.fetch_fixture(b"authenticated fixture")
        self.assertEqual(target.read_bytes(), b"authenticated fixture")
        self.assertEqual(stat.S_IMODE(target.stat().st_mode), 0o600)
        handlers = ACQUIRE.urllib.request.build_opener.call_args.args
        self.assertEqual(handlers[0].proxies, {})
        self.assertIsInstance(handlers[1], ACQUIRE.Redirects)
        self.assertEqual(self.opener.open.call_args.kwargs["timeout"], 20)

    def test_fetch_rejects_short_oversized_wrong_digest_and_wrong_status(self):
        cases = [(b"short", {"size": 6}), (b"oversized", {"size": 3}),
                 (b"wrong digest", {"checksum": "0" * 64}), (b"bad status", {"status": 206})]
        for data, options in cases:
            target = self.base / "download"
            with self.subTest(options=options), self.assertRaises(RuntimeError):
                self.fetch_fixture(data, **options)
            if target.exists():
                target.unlink()

    def test_unapproved_fetch_url_is_rejected_before_any_network_attempt(self):
        with self.assertRaises(RuntimeError):
            ACQUIRE.fetch("https://evil.invalid/asset", self.base / "download", 1, "0" * 64,
                          self.base, 10)
        self.opener.open.assert_not_called()
        self.assertFalse((self.base / "download").exists())

    def test_fetch_reserves_download_before_opening_network_or_output(self):
        target = self.base / "download"
        with patch.object(ACQUIRE, "capacity", side_effect=RuntimeError("reservation rejected")) as capacity:
            with self.assertRaisesRegex(RuntimeError, "reservation rejected"):
                ACQUIRE.fetch(ACQUIRE.ARCHIVE_URL, target, 7, "0" * 64, self.base, 10)
        capacity.assert_called_once_with(self.base, 10, pending=7 + 4096)
        self.opener.open.assert_not_called()
        self.assertFalse(target.exists())

    def test_fetch_rechecks_next_chunk_before_writing_its_bytes(self):
        data = b"fixture chunk"
        response = io.BytesIO(data)
        response.status = 200
        self.opener.open.side_effect = None
        self.opener.open.return_value = response
        target = self.base / "download"
        with patch.object(ACQUIRE, "capacity", side_effect=[None, RuntimeError("write rejected")]) as capacity, \
             patch.object(ACQUIRE.time, "monotonic", return_value=0):
            with self.assertRaisesRegex(RuntimeError, "write rejected"):
                ACQUIRE.fetch(ACQUIRE.ARCHIVE_URL, target, len(data), hashlib.sha256(data).hexdigest(),
                              self.base, 1)
        self.assertEqual(capacity.call_count, 2)
        self.assertEqual(capacity.call_args.kwargs, {"pending": len(data) + 4096})
        self.assertEqual(target.read_bytes(), b"")


class CapacityTests(LocalFixtures):
    def measured_capacity(self, *, blocks=None, free=None, now=0, mode=stat.S_IFREG,
                          links=1, owner=None, pending=0):
        info = SimpleNamespace(st_uid=os.geteuid() if owner is None else owner,
                               st_mode=mode | 0o600, st_nlink=links,
                               st_blocks=ACQUIRE.LIMIT // 512 if blocks is None else blocks)
        with patch.object(ACQUIRE.os, "walk", return_value=[(str(self.base), [], ["file"])]), \
             patch.object(ACQUIRE.Path, "lstat", return_value=info), \
             patch.object(ACQUIRE.shutil, "disk_usage", return_value=SimpleNamespace(
                 free=ACQUIRE.RESERVE if free is None else free)), \
             patch.object(ACQUIRE.time, "monotonic", return_value=now):
            return ACQUIRE.capacity(self.base, 1, pending=pending)

    def test_exact_512_mib_and_three_gib_reserve_are_accepted(self):
        self.assertEqual(self.measured_capacity(), 512 * 1024**2)

    def test_next_allocated_block_low_reserve_and_expired_deadline_are_rejected(self):
        for options in ({"blocks": ACQUIRE.LIMIT // 512 + 1}, {"free": ACQUIRE.RESERVE - 1}, {"now": 1}):
            with self.subTest(options=options), self.assertRaises(RuntimeError):
                self.measured_capacity(**options)

    def test_pending_write_checks_exact_footprint_neighbor(self):
        blocks = ACQUIRE.LIMIT // 512 - 1
        self.assertEqual(self.measured_capacity(blocks=blocks, free=ACQUIRE.RESERVE + 1024, pending=512),
                         ACQUIRE.LIMIT - 512)
        with self.assertRaisesRegex(RuntimeError, "512-MiB"):
            self.measured_capacity(blocks=blocks, free=ACQUIRE.RESERVE + 1024, pending=513)

    def test_pending_write_checks_exact_free_reserve_neighbor(self):
        self.assertEqual(self.measured_capacity(blocks=0, free=ACQUIRE.RESERVE + 512, pending=512), 0)
        with self.assertRaisesRegex(RuntimeError, "Three-GiB"):
            self.measured_capacity(blocks=0, free=ACQUIRE.RESERVE + 511, pending=512)

    def test_redirected_special_unowned_and_aliased_entries_are_rejected(self):
        for options in ({"mode": stat.S_IFLNK}, {"mode": stat.S_IFIFO}, {"links": 2},
                        {"owner": os.geteuid() + 1}):
            with self.subTest(options=options), self.assertRaises(RuntimeError):
                self.measured_capacity(**options)


if __name__ == "__main__":
    unittest.main()
