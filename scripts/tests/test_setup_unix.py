"""Installer boundary controls: synthetic archives, mocked network and commands."""
from __future__ import annotations

import contextlib
import importlib.util
import io
import json
from pathlib import Path
import stat
import subprocess
import sys
import tarfile
import tempfile
import unittest
from unittest import mock


SPEC = importlib.util.spec_from_file_location(
    "nevrotick_setup_unix", Path(__file__).resolve().parents[1] / "setup-unix.py"
)
assert SPEC is not None and SPEC.loader is not None
SETUP = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SETUP)


def archive_bytes(entries: list[tuple[str, bytes | None, bytes]]) -> bytes:
    """Build small files/link headers without touching the installation paths."""
    buffer = io.BytesIO()
    with tarfile.open(fileobj=buffer, mode="w:gz") as archive:
        for name, payload, member_type in entries:
            member = tarfile.TarInfo(name)
            member.type = member_type
            if member_type in (tarfile.SYMTYPE, tarfile.LNKTYPE):
                member.linkname = "unreviewed-target"
            member.size = len(payload) if payload is not None else 0
            archive.addfile(member, io.BytesIO(payload) if payload is not None else None)
    return buffer.getvalue()


class FetchControls(unittest.TestCase):
    def test_correct_download_digest_has_a_healthy_control(self) -> None:
        payload = b"reviewed archive bytes"
        response = mock.MagicMock()
        response.__enter__.return_value.read.return_value = payload
        with mock.patch.object(SETUP.urllib.request, "urlopen", return_value=response):
            self.assertEqual(
                SETUP.fetch("https://github.com/owner/repo/tool.tar.gz", SETUP.sha256(payload)),
                payload,
            )

    def test_wrong_download_digest_is_rejected(self) -> None:
        response = mock.MagicMock()
        response.__enter__.return_value.read.return_value = b"unexpected bytes"
        with mock.patch.object(SETUP.urllib.request, "urlopen", return_value=response):
            with self.assertRaises(ValueError):
                SETUP.fetch("https://github.com/owner/repo/tool.tar.gz", "0" * 64)

    def test_unadmitted_origin_never_reaches_network(self) -> None:
        with mock.patch.object(SETUP.urllib.request, "urlopen") as network:
            for url in ["file:///tmp/tool", "http://github.com/tool", "https://github.com.evil/tool"]:
                with self.subTest(url=url), self.assertRaises(ValueError):
                    SETUP.fetch(url, "0" * 64)
            network.assert_not_called()


class ManagerAdmissionControls(unittest.TestCase):
    def check_manager(self, *, admitted: bool) -> tuple[bool, mock.Mock]:
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            project, home = root / "project", root / "home"
            (project / "policy").mkdir(parents=True)
            manager, marker = root / "rustup", root / "manager-executed"
            manager.write_text(
                f"#!{sys.executable}\nfrom pathlib import Path\n"
                f"Path({str(marker)!r}).write_text('executed')\n"
                "print('rustup 1.29.2 synthetic-control')\n"
            )
            manager.chmod(0o755)
            (project / "policy/tool-manifest.json").write_text(json.dumps({
                "rust": {"channel": "1.99.1", "rustup_version": "1.29.2",
                         "rustup_linux_sha256": SETUP.sha256(manager.read_bytes()) if admitted else "0" * 64,
                         "manifest_url": "https://static.rust-lang.org/dist/channel-rust-1.99.1.toml",
                         "manifest_sha256": SETUP.sha256(b"manifest")},
                "tools": [],
            }))
            (project / "rust-toolchain.toml").write_text('[toolchain]\nchannel="1.99.1"\n')
            with mock.patch.object(SETUP, "ROOT", project), \
                 mock.patch.object(Path, "home", return_value=home), \
                 mock.patch.object(SETUP.platform, "system", return_value="Linux"), \
                 mock.patch.object(SETUP.platform, "machine", return_value="x86_64"), \
                 mock.patch.object(SETUP.os, "geteuid", return_value=1000), \
                 mock.patch.dict(SETUP.os.environ, {}, clear=True), \
                 mock.patch.object(SETUP.shutil, "which", return_value=str(manager)), \
                 mock.patch.object(sys, "argv", ["setup-unix.py", "--tools-only"]), \
                 mock.patch.object(SETUP, "fetch", return_value=b"manifest") as fetch, \
                 contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
                if admitted:
                    SETUP.main()
                else:
                    with self.assertRaises(SystemExit):
                        SETUP.main()
            return marker.exists(), fetch

    def test_version_spoofing_manager_is_rejected_before_execution_or_download(self) -> None:
        ran, fetch = self.check_manager(admitted=False)
        self.assertFalse(ran)
        fetch.assert_not_called()

    def test_hash_admitted_manager_has_a_healthy_execution_control(self) -> None:
        ran, fetch = self.check_manager(admitted=True)
        self.assertTrue(ran)
        fetch.assert_called_once()


class InstallationControls(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-installer-test-", dir="/tmp")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.bin_dir, self.cache, self.backup = [self.root / name for name in ("bin", "cache", "backup")]
        for path in (self.bin_dir, self.cache, self.backup):
            path.mkdir()
        self.target = self.bin_dir / "cargo-deny"
        self.old = b"previous trusted executable"
        self.payload = b"synthetic executable; never actually run"
        self.data = archive_bytes([("release/cargo-deny", self.payload, tarfile.REGTYPE)])
        self.tool = {
            "name": "cargo-deny",
            "version": "0.20.2",
            "version_args": ["--version"],
            "url": "https://github.com/owner/repo/tool.tar.gz",
            "sha256": SETUP.sha256(self.data),
        }

    def install(self, data: bytes | None = None, *, side_effect: BaseException | None = None,
                version: str = "cargo-deny 0.20.2\n") -> tuple[dict, mock.Mock]:
        if data is None:
            data = self.data
        tool = {**self.tool, "sha256": SETUP.sha256(data)}
        with mock.patch.object(SETUP, "fetch", return_value=data), \
             mock.patch.object(SETUP.subprocess, "run") as command, \
             contextlib.redirect_stdout(io.StringIO()):
            self.last_command = command
            if side_effect is not None:
                command.side_effect = side_effect
            else:
                command.return_value = subprocess.CompletedProcess([], 0, version, "")
            result = SETUP.install_tool(tool, self.bin_dir, self.cache, self.backup)
        return result, command

    def preserve_existing(self) -> None:
        self.target.write_bytes(self.old)
        self.target.chmod(0o750)

    def assert_previous_restored(self) -> None:
        self.assertEqual(self.target.read_bytes(), self.old)
        self.assertEqual(stat.S_IMODE(self.target.stat().st_mode), 0o750)
        self.assertEqual((self.backup / self.target.name).read_bytes(), self.old)
        self.assertEqual([p.name for p in self.bin_dir.iterdir()], [self.target.name])

    def test_reviewed_regular_binary_installs_and_records_old_and_new_identity(self) -> None:
        self.preserve_existing()
        result, command = self.install()
        self.assertEqual(self.target.read_bytes(), self.payload)
        self.assertEqual((self.backup / self.target.name).read_bytes(), self.old)
        self.assertEqual(result["archive_sha256"], self.tool["sha256"])
        self.assertEqual(result["binary_sha256"], SETUP.sha256(self.payload))
        self.assertEqual(result["previous_binary"], str(self.backup / self.target.name))
        self.assertEqual(command.call_args.args[0], [str(self.target), "--version"])
        self.assertTrue(command.call_args.kwargs["check"])
        self.assertEqual(stat.S_IMODE(self.target.stat().st_mode), 0o755)

    def test_corrupt_cached_archive_cannot_replace_or_execute_existing_tool(self) -> None:
        self.preserve_existing()
        (self.cache / (self.tool["sha256"] + ".tar.gz")).write_bytes(b"corrupted cache")
        with mock.patch.object(SETUP, "fetch") as network, \
             mock.patch.object(SETUP.subprocess, "run") as command, \
             contextlib.redirect_stdout(io.StringIO()):
            with self.assertRaises(ValueError):
                SETUP.install_tool(self.tool, self.bin_dir, self.cache, self.backup)
            network.assert_not_called()
            command.assert_not_called()
        self.assertEqual(self.target.read_bytes(), self.old)

    def test_oversized_cache_is_rejected_without_reading_or_executing(self) -> None:
        self.preserve_existing()
        archive = self.cache / (self.tool["sha256"] + ".tar.gz")
        # A sparse fixture declares the forbidden size without allocating its bytes.
        with archive.open("wb") as stream:
            stream.truncate(160 * 1024 * 1024 + 1)
        with mock.patch.object(Path, "read_bytes", side_effect=AssertionError("Unexpected read")) as read, \
             mock.patch.object(SETUP, "fetch") as network, \
             mock.patch.object(SETUP.subprocess, "run") as command, \
             contextlib.redirect_stdout(io.StringIO()):
            with self.assertRaises(ValueError):
                SETUP.install_tool(self.tool, self.bin_dir, self.cache, self.backup)
            read.assert_not_called()
            network.assert_not_called()
            command.assert_not_called()
        self.assertEqual(self.target.read_bytes(), self.old)

    def test_valid_cache_has_a_neighboring_success_without_network(self) -> None:
        (self.cache / (self.tool["sha256"] + ".tar.gz")).write_bytes(self.data)
        with mock.patch.object(SETUP, "fetch") as network, \
             mock.patch.object(SETUP.subprocess, "run", return_value=
                               subprocess.CompletedProcess([], 0, "cargo-deny 0.20.2\n", "")), \
             contextlib.redirect_stdout(io.StringIO()):
            result = SETUP.install_tool(self.tool, self.bin_dir, self.cache, self.backup)
            network.assert_not_called()
        self.assertEqual(self.target.read_bytes(), self.payload)
        self.assertEqual(result["archive_sha256"], self.tool["sha256"])

    def test_unregistered_executable_name_is_rejected_before_acquisition(self) -> None:
        tool = {**self.tool, "name": "unregistered-shell"}
        with mock.patch.object(SETUP, "fetch") as network, \
             mock.patch.object(SETUP.subprocess, "run") as command:
            with self.assertRaises(ValueError):
                SETUP.install_tool(tool, self.bin_dir, self.cache, self.backup)
            network.assert_not_called()
            command.assert_not_called()
        self.assertEqual(list(self.bin_dir.iterdir()), [])

    def test_wrong_download_bytes_cannot_reach_tar_or_execution(self) -> None:
        self.preserve_existing()
        with mock.patch.object(SETUP, "fetch", return_value=b"wrong bytes"), \
             mock.patch.object(SETUP.tarfile, "open") as archive, \
             mock.patch.object(SETUP.subprocess, "run") as command, \
             contextlib.redirect_stdout(io.StringIO()):
            with self.assertRaises(ValueError):
                SETUP.install_tool(self.tool, self.bin_dir, self.cache, self.backup)
            archive.assert_not_called()
            command.assert_not_called()
        self.assertEqual(self.target.read_bytes(), self.old)

    def test_no_or_multiple_regular_candidates_are_rejected(self) -> None:
        self.preserve_existing()
        cases = {
            "absent": [("other-tool", self.payload, tarfile.REGTYPE)],
            "ambiguous": [("a/cargo-deny", self.payload, tarfile.REGTYPE),
                          ("b/cargo-deny", self.payload, tarfile.REGTYPE)],
            "symlink": [("cargo-deny", None, tarfile.SYMTYPE)],
            "hardlink": [("cargo-deny", None, tarfile.LNKTYPE)],
            "directory": [("cargo-deny", None, tarfile.DIRTYPE)],
        }
        for name, entries in cases.items():
            with self.subTest(name=name):
                with self.assertRaises(ValueError):
                    self.install(archive_bytes(entries))
                self.last_command.assert_not_called()
                self.assertEqual(self.target.read_bytes(), self.old)

    def test_symlink_destination_and_its_referent_are_preserved(self) -> None:
        referent = self.root / "unrelated-file"
        referent.write_bytes(self.old)
        self.target.symlink_to(referent)
        with self.assertRaises(ValueError):
            self.install()
        self.assertTrue(self.target.is_symlink())
        self.assertEqual(referent.read_bytes(), self.old)

    def test_failed_version_command_restores_existing_bytes_and_permissions(self) -> None:
        for error in [subprocess.CalledProcessError(1, "synthetic --version"),
                      subprocess.TimeoutExpired("synthetic --version", 30)]:
            with self.subTest(error=type(error).__name__):
                self.preserve_existing()
                with self.assertRaises(type(error)):
                    self.install(side_effect=error)
                self.assert_previous_restored()

    def test_wrong_or_missing_version_restores_existing_tool(self) -> None:
        for version in ["cargo-deny 0.19.0\n", ""]:
            with self.subTest(version=version):
                self.preserve_existing()
                with self.assertRaises(Exception):
                    self.install(version=version)
                self.assert_previous_restored()

    def test_failed_first_install_removes_unqualified_binary(self) -> None:
        with self.assertRaises(ValueError):
            self.install(version="cargo-deny 0.19.0\n")
        self.assertFalse(self.target.exists())
        self.assertEqual(list(self.bin_dir.iterdir()), [])

    def test_second_tool_failure_records_partial_outcome_and_preserved_backups(self) -> None:
        project, home, cargo_home = [self.root / name for name in ("project", "home", "cargo-home")]
        (project / "policy").mkdir(parents=True)
        first = {**self.tool, "url": "https://github.com/owner/repo/deny.tar.gz"}
        second_data = archive_bytes([("cargo-nextest", b"second synthetic tool", tarfile.REGTYPE)])
        second = {**self.tool, "name": "cargo-nextest", "version": "0.9.150",
                  "url": "https://github.com/owner/repo/nextest.tar.gz",
                  "sha256": SETUP.sha256(second_data)}
        rust_manifest_url = "https://static.rust-lang.org/dist/channel-rust-1.99.1.toml"
        manager = self.root / "rustup"
        manager.write_bytes(b"admitted synthetic Rustup")
        (project / "policy/tool-manifest.json").write_text(json.dumps({
            "rust": {"channel": "1.99.1", "rustup_version": "1.29.2", "manifest_url": rust_manifest_url,
                     "rustup_linux_sha256": SETUP.sha256(manager.read_bytes()),
                     "manifest_sha256": SETUP.sha256(b"manifest")},
            "tools": [first, second],
        }))
        (project / "rust-toolchain.toml").write_text(
            '[toolchain]\nchannel="1.99.1"\nprofile="minimal"\n'
            'components=["clippy","rustfmt"]\ntargets=["x86_64-pc-windows-msvc"]\n'
        )
        tool_bin = cargo_home / "bin"
        tool_bin.mkdir(parents=True)
        first_target, second_target = tool_bin / first["name"], tool_bin / second["name"]
        first_target.write_bytes(b"old first tool")
        second_target.write_bytes(b"old second tool")
        receipt_path = self.root / "requested-receipt.json"
        checkpoints = []

        def acquire(url: str, expected: str) -> bytes:
            if url == rust_manifest_url:
                return b"manifest"
            checkpoints.append(json.loads(receipt_path.read_text()))
            return {first["url"]: self.data, second["url"]: second_data}[url]

        def version_command(command: list[str], **kwargs) -> subprocess.CompletedProcess:
            if Path(command[0]).name == "rustup":
                return subprocess.CompletedProcess(command, 0, "rustup 1.29.2 (synthetic-manager)\n", "")
            if Path(command[0]).name == first["name"]:
                return subprocess.CompletedProcess(command, 0, "cargo-deny 0.20.2\n", "")
            raise subprocess.CalledProcessError(7, command)

        output = io.StringIO()
        with mock.patch.object(SETUP, "ROOT", project), \
             mock.patch.object(Path, "home", return_value=home), \
             mock.patch.object(SETUP.platform, "system", return_value="Linux"), \
             mock.patch.object(SETUP.platform, "machine", return_value="x86_64"), \
             mock.patch.object(SETUP.os, "geteuid", return_value=1000), \
             mock.patch.dict(SETUP.os.environ, {"CARGO_HOME": str(cargo_home)}, clear=True), \
             mock.patch.object(SETUP.shutil, "which", return_value=str(manager)), \
             mock.patch.object(sys, "argv", ["setup-unix.py", "--tools-only", "--json", str(receipt_path)]), \
             mock.patch.object(SETUP, "fetch", side_effect=acquire), \
             mock.patch.object(SETUP.subprocess, "run", side_effect=version_command), \
             contextlib.redirect_stdout(output):
            with self.assertRaises(subprocess.CalledProcessError):
                SETUP.main()

        receipt = json.loads(receipt_path.read_text())
        self.assertNotEqual(receipt["status"], "complete")
        self.assertEqual(receipt["failed_tool"], second["name"])
        self.assertEqual([tool["name"] for tool in receipt["tools"]], [first["name"]])
        self.assertEqual(first_target.read_bytes(), self.payload)
        self.assertEqual(second_target.read_bytes(), b"old second tool")
        backup = Path(receipt["backup_directory"])
        self.assertEqual((backup / first["name"]).read_bytes(), b"old first tool")
        self.assertEqual((backup / second["name"]).read_bytes(), b"old second tool")
        self.assertEqual(checkpoints[0]["tools"], [])
        self.assertEqual([tool["name"] for tool in checkpoints[1]["tools"]], [first["name"]])
        self.assertLess(output.getvalue().index("Backups:"), output.getvalue().index("Verifying"))


if __name__ == "__main__":
    unittest.main()
