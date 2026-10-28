"""Offline xwin acquisition, installation and linker-guard controls."""
from __future__ import annotations

import contextlib
import hashlib
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


def load_script(name: str):
    path = Path(__file__).resolve().parents[1] / name
    spec = importlib.util.spec_from_file_location(name.replace("-", "_"), path)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


XWIN = load_script("setup-xwin.py")
LINKER = load_script("xwin-linker.py")


def sha(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


class TemporaryControls(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-xwin-test-", dir="/tmp")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)


class DownloadControls(TemporaryControls):
    def test_declared_filename_does_not_replace_the_reviewed_content_pin(self) -> None:
        body = b"reviewed raw Microsoft metadata"
        declared = "1" * 64
        self.assertNotEqual(declared, sha(body))
        destination = self.root / ("pkg_manifest_" + declared + ".vsman")
        destination.write_bytes(body)
        with mock.patch.object(XWIN.urllib.request, "urlopen") as network:
            self.assertEqual(XWIN.download("https://example.invalid/catalog", destination, sha(body)), body)
            network.assert_not_called()
        destination.write_bytes(b"corrupted cache")
        with mock.patch.object(XWIN.urllib.request, "urlopen", return_value=io.BytesIO(body)) as network:
            self.assertEqual(XWIN.download("https://example.invalid/catalog", destination, sha(body)), body)
            network.assert_called_once()
        self.assertEqual(destination.read_bytes(), body)

    def test_wrong_content_digest_cannot_overwrite_the_previous_cached_file(self) -> None:
        destination = self.root / "catalog.vsman"
        destination.write_bytes(b"previous cache")
        with mock.patch.object(XWIN.urllib.request, "urlopen", return_value=io.BytesIO(b"wrong body")):
            with self.assertRaises(RuntimeError):
                XWIN.download("https://example.invalid/catalog", destination, sha(b"reviewed body"))
        self.assertEqual(destination.read_bytes(), b"previous cache")


class InstallationControls(TemporaryControls):
    def setUp(self) -> None:
        super().setUp()
        self.bin_dir = self.root / "bin"
        self.bin_dir.mkdir()
        self.executable = self.bin_dir / "xwin"
        self.backup = self.root / "backup"
        self.payload = b"synthetic xwin executable; no real execution"
        self.previous = b"previous trusted xwin"
        self.executable.write_bytes(self.previous)
        self.executable.chmod(0o750)

    def install(self, version="xwin 0.10.0\n"):
        with mock.patch.object(XWIN.subprocess, "check_output", return_value=version):
            return XWIN.install_xwin(self.payload, self.executable, "0.10.0", self.backup)

    def test_new_binary_is_version_checked_before_replacing_the_previous_one(self) -> None:
        def check(command, **kwargs):
            self.assertEqual(self.executable.read_bytes(), self.previous)
            self.assertNotEqual(Path(command[0]), self.executable)
            self.assertEqual(Path(command[0]).read_bytes(), self.payload)
            self.assertEqual(command[1:], ["--version"])
            return "xwin 0.10.0\n"

        with mock.patch.object(XWIN.subprocess, "check_output", side_effect=check):
            self.assertEqual(XWIN.install_xwin(self.payload, self.executable, "0.10.0", self.backup),
                             "xwin 0.10.0")
        self.assertEqual(self.executable.read_bytes(), self.payload)
        self.assertEqual(stat.S_IMODE(self.executable.stat().st_mode), 0o755)
        self.assertEqual([p.read_bytes() for p in self.backup.iterdir()], [self.previous])
        self.assertEqual([p.name for p in self.bin_dir.iterdir()], ["xwin"])

    def test_wrong_version_preserves_previous_bytes_permissions_and_no_temporary_file(self) -> None:
        with self.assertRaises(RuntimeError):
            self.install(version="xwin 0.9.0\n")
        self.assertEqual(self.executable.read_bytes(), self.previous)
        self.assertEqual(stat.S_IMODE(self.executable.stat().st_mode), 0o750)
        self.assertEqual([p.name for p in self.bin_dir.iterdir()], ["xwin"])

    def test_failed_version_command_leaves_the_previous_executable_untouched(self) -> None:
        with mock.patch.object(XWIN.subprocess, "check_output", side_effect=
                               subprocess.CalledProcessError(7, "synthetic xwin --version")):
            with self.assertRaises(subprocess.CalledProcessError):
                XWIN.install_xwin(self.payload, self.executable, "0.10.0", self.backup)
        self.assertEqual(self.executable.read_bytes(), self.previous)
        self.assertEqual([p.name for p in self.bin_dir.iterdir()], ["xwin"])

    def test_stale_predictable_temporary_symlink_cannot_modify_its_referent(self) -> None:
        victim = self.root / "unrelated-file"
        victim.write_bytes(b"unrelated user content")
        victim.chmod(0o640)
        stale = self.bin_dir / ".xwin-nevrotick.tmp"
        stale.symlink_to(victim)
        self.install()
        self.assertEqual(victim.read_bytes(), b"unrelated user content")
        self.assertEqual(stat.S_IMODE(victim.stat().st_mode), 0o640)
        self.assertTrue(stale.is_symlink())
        self.assertEqual(self.executable.read_bytes(), self.payload)

    def test_target_symlink_is_rejected_without_executing_or_replacing_it(self) -> None:
        referent = self.root / "other-executable"
        referent.write_bytes(self.previous)
        self.executable.unlink()
        self.executable.symlink_to(referent)
        with mock.patch.object(XWIN.subprocess, "check_output") as command:
            with self.assertRaises(RuntimeError):
                XWIN.install_xwin(self.payload, self.executable, "0.10.0", self.backup)
            command.assert_not_called()
        self.assertTrue(self.executable.is_symlink())
        self.assertEqual(referent.read_bytes(), self.previous)


class CatalogControls(TemporaryControls):
    def setUp(self) -> None:
        super().setUp()
        self.dl = self.root / "dl"
        self.dl.mkdir()
        self.cab = b"reviewed synthetic Microsoft CAB"
        self.packages = {"packages": [{"payloads": [{"sha256": sha(self.cab)}]}]}
        self.metadata = json.dumps(self.packages).encode("utf-8")
        self.manifest = self.dl / ("pkg_manifest_" + "1" * 64 + ".vsman")
        self.manifest.write_bytes(self.metadata)
        (self.dl / "sdk.cab").write_bytes(self.cab)

    def test_raw_catalog_pin_and_catalog_listed_cab_have_a_healthy_control(self) -> None:
        rows = XWIN.verify_catalog_payloads(self.root, self.packages, sha(self.metadata))
        self.assertEqual({row["sha256"] for row in rows}, {sha(self.metadata), sha(self.cab)})
        self.assertEqual(sum(row["bytes"] for row in rows), len(self.metadata) + len(self.cab))

    def test_changed_cab_or_unlisted_payload_cannot_be_reported_as_verified(self) -> None:
        (self.dl / "sdk.cab").write_bytes(b"unlisted CAB bytes")
        with self.assertRaises(RuntimeError):
            XWIN.verify_catalog_payloads(self.root, self.packages, sha(self.metadata))

    def test_raw_metadata_digest_is_required_even_when_declared_filename_is_retained(self) -> None:
        self.manifest.write_bytes(b"changed metadata")
        with self.assertRaises(RuntimeError):
            XWIN.verify_catalog_payloads(self.root, self.packages, sha(self.metadata))


class CacheCleanupControls(TemporaryControls):
    def setUp(self) -> None:
        super().setUp()
        self.dl = self.root / "dl"
        self.dl.mkdir()
        self.payloads = [self.dl / name for name in ["sdk.cab", "crt.cab"]]
        self.package = self.dl / "reviewed.vsman"
        for path in [*self.payloads, self.package]:
            path.write_bytes(path.name.encode())
        self.downloaded = [{"path": str(p), "sha256": XWIN.digest(p), "bytes": p.stat().st_size}
                           for p in [*self.payloads, self.package]]
        self.generated = self.root / "unpack/sdk.cab"
        self.generated.mkdir(parents=True)
        (self.generated / "contents").write_bytes(b"generated unpack data")
        self.retained = [self.dl / "unknown-file", self.root / "unpack/unknown/contents",
                         self.root / "provenance/LICENSE", self.root / "splat/sdk/lib/retained.lib"]
        for path in self.retained:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"retain this content")

    def test_only_listed_payloads_and_matching_unpack_tree_are_removed(self) -> None:
        result = XWIN.prune_generated_cache(self.root, self.downloaded, self.package)
        self.assertEqual(set(result["removed_paths"]), {str(p) for p in [*self.payloads, self.generated]})
        self.assertTrue(all(not path.exists() for path in [*self.payloads, self.generated]))
        self.assertEqual(self.package.read_bytes(), b"reviewed.vsman")
        self.assertTrue(all(path.read_bytes() == b"retain this content" for path in self.retained))

    def test_changed_payload_aborts_before_any_deletion(self) -> None:
        self.payloads[1].write_bytes(b"changed after verification")
        with self.assertRaises(RuntimeError):
            XWIN.prune_generated_cache(self.root, self.downloaded, self.package)
        self.assertTrue(all(path.exists() for path in [*self.payloads, self.generated, self.package]))
        self.assertTrue(all(path.read_bytes() == b"retain this content" for path in self.retained))

    def test_symlink_payload_is_rejected_before_any_deletion(self) -> None:
        referent = self.root / "unrelated-file"
        referent.write_bytes(b"crt.cab")
        self.payloads[1].unlink()
        self.payloads[1].symlink_to(referent)
        with self.assertRaises(RuntimeError):
            XWIN.prune_generated_cache(self.root, self.downloaded, self.package)
        self.assertTrue(self.payloads[1].is_symlink())
        self.assertEqual(referent.read_bytes(), b"crt.cab")
        self.assertTrue(all(path.exists() for path in [self.payloads[0], self.generated, self.package]))
        self.assertTrue(all(path.read_bytes() == b"retain this content" for path in self.retained))


class MetadataComplete(Exception):
    """Stop the isolated main-entrypoint fixture before terms/SDK acquisition."""


class ManifestDeclarationControls(TemporaryControls):
    def setUp(self) -> None:
        super().setUp()
        self.project = self.root / "project"
        (self.project / "policy").mkdir(parents=True)
        self.channel_url = "https://example.invalid/reviewed.chman"
        self.package_url = "https://example.invalid/reviewed.vsman"
        self.terms_url = "https://example.invalid/terms"
        self.declared = "a" * 64
        self.raw_metadata = b'{"packages": []}'
        self.channel = json.dumps({
            "info": {"productMilestoneIsPreRelease": False},
            "channelItems": [{"type": "Manifest", "payloads": [
                {"sha256": self.declared, "url": self.package_url},
            ]}],
        }).encode("utf-8")
        buffer = io.BytesIO()
        with tarfile.open(fileobj=buffer, mode="w:gz") as archive:
            payload = b"synthetic xwin; never executed"
            entry = tarfile.TarInfo("xwin")
            entry.size = len(payload)
            archive.addfile(entry, io.BytesIO(payload))
        self.archive = buffer.getvalue()
        self.archive_url = "https://example.invalid/xwin.tar.gz"
        self.policy = {"cross_toolchain": {
            "xwin": {"version": "0.10.0", "url": self.archive_url, "sha256": sha(self.archive)},
            "microsoft_inputs": {
                "architecture": "x86_64", "variant": "desktop", "manifest_version": "18",
                "sdk_version": "10.0.28000", "crt_version": "14.51",
                "channel_manifest_url": self.channel_url,
                "channel_manifest_sha256": sha(self.channel),
                "package_manifest_declared_sha256": self.declared,
                "package_manifest_sha256": sha(self.raw_metadata), "terms_url": self.terms_url,
            },
        }}

    def run_setup_to_metadata(self) -> None:
        (self.project / "policy/tool-manifest.json").write_text(json.dumps(self.policy))
        self.calls = []
        original_match = XWIN.re.match

        def classify_mount(pattern, value, *args, **kwargs):
            if pattern == r"^/mnt/[a-z](?:/|$)":
                return original_match(pattern, "/mnt/c/disposable-fixture")
            return original_match(pattern, value, *args, **kwargs)

        def acquire(url, destination, expected=None):
            self.calls.append((url, destination, expected))
            if url == self.terms_url:
                raise MetadataComplete()
            return {self.archive_url: self.archive, self.channel_url: self.channel,
                    self.package_url: self.raw_metadata}[url]

        with mock.patch.object(XWIN, "ROOT", self.project), \
             mock.patch.object(XWIN, "LOCATION", self.root / "locator.json"), \
             mock.patch.object(Path, "home", return_value=self.root / "home"), \
             mock.patch.object(sys, "argv", ["setup-xwin.py", "--accept-license",
                                             "--cache-dir", str(self.root / "cache")]), \
             mock.patch.object(XWIN.re, "match", side_effect=classify_mount), \
             mock.patch.object(XWIN, "download", side_effect=acquire), \
             mock.patch.object(XWIN, "install_xwin", return_value="xwin 0.10.0"), \
             contextlib.redirect_stdout(io.StringIO()):
            # Only the Windows-mount location classification is stubbed: all
            # paths and writes belong to this disposable /tmp fixture.
            XWIN.main()

    def test_reviewed_declared_and_raw_pins_are_used_separately(self) -> None:
        self.assertNotEqual(self.declared, sha(self.raw_metadata))
        with self.assertRaises(MetadataComplete):
            self.run_setup_to_metadata()
        package_call = next(call for call in self.calls if call[0] == self.package_url)
        self.assertEqual(package_call[2], sha(self.raw_metadata))
        self.assertEqual(package_call[1].name, "pkg_manifest_" + self.declared + ".vsman")
        channel_call = next(call for call in self.calls if call[0] == self.channel_url)
        self.assertEqual(channel_call[2], sha(self.channel))

    def test_changed_declaration_is_rejected_before_package_acquisition(self) -> None:
        self.policy["cross_toolchain"]["microsoft_inputs"]["package_manifest_declared_sha256"] = "f" * 64
        with self.assertRaises(RuntimeError):
            self.run_setup_to_metadata()
        self.assertNotIn(self.package_url, [call[0] for call in self.calls])
        self.assertNotIn(self.terms_url, [call[0] for call in self.calls])


class LinkerControls(TemporaryControls):
    def setUp(self) -> None:
        super().setUp()
        self.home, self.project = self.root / "home", self.root / "project"
        self.rustup = self.home / ".cargo/bin/rustup"
        self.rustup.parent.mkdir(parents=True)
        self.rustup.write_bytes(b"synthetic rustup")
        self.host = "x86_64-unknown-linux-gnu"
        self.sysroot = self.root / "toolchain"
        self.linker = self.sysroot / "lib/rustlib" / self.host / "bin/rust-lld"
        self.linker.parent.mkdir(parents=True)
        self.linker.write_bytes(b"synthetic rust-lld")
        self.splat = self.root / "splat with spaces"
        self.libraries = [self.splat / path for path in
                          ["crt/lib/x86_64", "sdk/lib/um/x86_64", "sdk/lib/ucrt/x86_64"]]
        for path in self.libraries:
            path.mkdir(parents=True)
        self.inputs = {
            "architecture": "x86_64", "variant": "desktop", "sdk_version": "10.0.28000",
            "crt_version": "14.51", "channel_manifest_sha256": "a" * 64,
            "package_manifest_sha256": "b" * 64, "package_manifest_declared_sha256": "c" * 64,
        }
        policy = {
            "rust": {"channel": "1.99.1", "host": self.host,
                     "rustup_linux_sha256": sha(self.rustup.read_bytes())},
            "cross_toolchain": {"microsoft_inputs": self.inputs,
                                "linker": {"binary_sha256": sha(self.linker.read_bytes())}},
        }
        (self.project / "policy").mkdir(parents=True)
        (self.project / "policy/tool-manifest.json").write_text(json.dumps(policy))
        self.config_path = self.home / ".local/share/nevrotick/xwin.json"
        self.config_path.parent.mkdir(parents=True)
        self.config = {"microsoft_inputs": self.inputs, "root": str(self.splat),
                       "library_paths": [str(p) for p in self.libraries]}
        self.config_path.write_text(json.dumps(self.config))

    def run_linker(self, args=None, environment=None) -> None:
        def observe_exec(*args):
            self.exec_environment = dict(LINKER.os.environ)

        with mock.patch.object(LINKER, "ROOT", self.project), \
             mock.patch.object(Path, "home", return_value=self.home), \
             mock.patch.dict(LINKER.os.environ, environment or {}, clear=True), \
             mock.patch.object(sys, "argv", ["xwin-linker.py", *(args or [])]), \
             mock.patch.object(LINKER.subprocess, "check_output", return_value=str(self.sysroot)) as rustup, \
             mock.patch.object(LINKER.os, "execv", side_effect=observe_exec) as execute:
            self.rustup_command, self.execute = rustup, execute
            LINKER.main()

    def test_pinned_linker_preserves_exact_link_arguments_and_library_paths(self) -> None:
        args = ["/OUT:example with spaces.exe", "@response with spaces.rsp", "/GUARD:CF", "/WX"]
        self.run_linker(args)
        self.assertEqual(self.rustup_command.call_args.args[0],
                         [str(self.rustup), "run", "1.99.1-" + self.host, "rustc", "--print", "sysroot"])
        executable, argv = self.execute.call_args.args
        self.assertEqual(executable, self.linker)
        self.assertEqual(argv, [str(self.linker), "-flavor", "link",
                                *("/libpath:" + str(p) for p in self.libraries), *args])

    def test_jobserver_flags_are_stripped_only_from_the_metadata_child(self) -> None:
        environment = {"CARGO_MAKEFLAGS": "--jobserver-auth=9,10", "MAKEFLAGS": "--jobserver-auth=9,10",
                       "ORDINARY_CONTROL": "retained"}
        args = ["/OUT:with spaces.exe", "@response.rsp"]
        self.run_linker(args, environment)
        helper_environment = self.rustup_command.call_args.kwargs["env"]
        self.assertNotIn("CARGO_MAKEFLAGS", helper_environment)
        self.assertNotIn("MAKEFLAGS", helper_environment)
        self.assertEqual(helper_environment["ORDINARY_CONTROL"], "retained")
        self.assertEqual(self.exec_environment, environment)
        self.assertEqual(self.execute.call_args.args[1][-len(args):], args)

    def test_rustup_digest_drift_is_rejected_before_any_execution(self) -> None:
        self.rustup.write_bytes(b"changed rustup")
        with self.assertRaises(RuntimeError):
            self.run_linker()
        self.rustup_command.assert_not_called()
        self.execute.assert_not_called()

    def test_linker_digest_drift_is_rejected_before_exec(self) -> None:
        self.linker.write_bytes(b"changed linker")
        with self.assertRaises(RuntimeError):
            self.run_linker()
        self.execute.assert_not_called()

    def test_setup_policy_drift_is_rejected_before_rustup_or_linker(self) -> None:
        self.config["microsoft_inputs"] = {**self.inputs, "sdk_version": "unadmitted-sdk"}
        self.config_path.write_text(json.dumps(self.config))
        with self.assertRaises(RuntimeError):
            self.run_linker()
        self.rustup_command.assert_not_called()
        self.execute.assert_not_called()

    def test_missing_library_directory_is_rejected_before_linker_execution(self) -> None:
        self.libraries[0].rmdir()
        with self.assertRaises(RuntimeError):
            self.run_linker()
        self.execute.assert_not_called()


if __name__ == "__main__":
    unittest.main()
