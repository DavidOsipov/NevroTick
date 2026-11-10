"""Offline controls for the explicit hosted xwin acquisition boundary."""
from __future__ import annotations

import contextlib
import gzip
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import sys
import tarfile
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock


def load_helper(filename: str):
    path = Path(__file__).resolve().parents[1] / filename
    name = "nevrotick_hosted_tests_" + filename.replace("-", "_").replace(".", "_")
    spec = importlib.util.spec_from_file_location(name, path)
    assert spec is not None and spec.loader is not None
    module = importlib.util.module_from_spec(spec)
    sys.modules[name] = module
    spec.loader.exec_module(module)
    return module


HOSTED = load_helper("xwin_hosted.py")
ENV = load_helper("check-environment.py")
XWIN = load_helper("setup-xwin.py")


class TemporaryControls(unittest.TestCase):
    def setUp(self) -> None:
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-hosted-xwin-", dir="/tmp")
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)


class InventoryControls(TemporaryControls):
    def setUp(self) -> None:
        super().setUp()
        self.cache = self.root / "cache"
        self.cache.mkdir()
        (self.cache / "a.cab").write_bytes(b"abc")
        (self.cache / "nested").mkdir()
        (self.cache / "nested/b.cab").write_bytes(b"defgh")

    def test_regular_payloads_have_exact_independent_bytes_and_hashes(self) -> None:
        result = HOSTED.inventory(self.cache, max_entries=3, max_bytes=8,
                                  max_file_bytes=5, max_depth=2)
        self.assertEqual(set(result), {"a.cab", "nested/b.cab"})
        for name, expected in [("a.cab", b"abc"), ("nested/b.cab", b"defgh")]:
            self.assertEqual(result[name]["bytes"], len(expected))
            self.assertEqual(result[name]["sha256"], hashlib.sha256(expected).hexdigest())

    def test_each_inventory_budget_has_an_exact_healthy_neighbor(self) -> None:
        good = {"max_entries": 3, "max_bytes": 8, "max_file_bytes": 5, "max_depth": 2}
        self.assertEqual(len(HOSTED.inventory(self.cache, **good)), 2)
        for field, limit in [("max_entries", 2), ("max_bytes", 7),
                             ("max_file_bytes", 4), ("max_depth", 1)]:
            with self.subTest(field=field), self.assertRaises(RuntimeError):
                HOSTED.inventory(self.cache, **(good | {field: limit}))

    def test_links_and_special_entries_cannot_become_verified_payloads(self) -> None:
        outside = self.root / "outside.cab"
        outside.write_bytes(b"valid-looking outside payload")
        path = self.cache / "untrusted"
        for kind in ["file-link", "directory-link", "hardlink", "fifo"]:
            with self.subTest(kind=kind):
                if kind == "file-link":
                    path.symlink_to(outside)
                elif kind == "directory-link":
                    path.symlink_to(self.root, target_is_directory=True)
                elif kind == "hardlink":
                    os.link(outside, path)
                else:
                    os.mkfifo(path)
                try:
                    with self.assertRaises(RuntimeError):
                        HOSTED.inventory(self.cache)
                    self.assertEqual(outside.read_bytes(), b"valid-looking outside payload")
                finally:
                    path.unlink()
        self.assertEqual(len(HOSTED.inventory(self.cache)), 2)

    def test_a_linked_inventory_root_is_not_a_second_authorized_view(self) -> None:
        alias = self.root / "cache-alias"
        alias.symlink_to(self.cache, target_is_directory=True)
        with self.assertRaises(RuntimeError):
            HOSTED.inventory(alias)


class ArchiveControls(unittest.TestCase):
    prefix = "xwin-0.10.0-x86_64-unknown-linux-musl/"
    expected = {"xwin": b"public executable fixture", "LICENSE-APACHE": b"Apache notice",
                "LICENSE-MIT": b"MIT notice", "README.md": b"public readme"}

    def archive(self, *, extra=(), omit=()) -> bytes:
        output = io.BytesIO()
        with tarfile.open(fileobj=output, mode="w:gz") as bundle:
            root = tarfile.TarInfo(self.prefix)
            root.type = tarfile.DIRTYPE
            bundle.addfile(root)
            for name, body in self.expected.items():
                if name not in omit:
                    item = tarfile.TarInfo(self.prefix + name)
                    item.size = len(body)
                    bundle.addfile(item, io.BytesIO(body))
            for item, body in extra:
                bundle.addfile(item, io.BytesIO(body) if body is not None else None)
        return output.getvalue()

    def test_exact_pinned_bundle_members_have_a_healthy_neighbor(self) -> None:
        self.assertEqual(HOSTED.tar_members(self.archive(), "0.10.0"), self.expected)
        with self.assertRaises(RuntimeError):
            HOSTED.tar_members(self.archive(), "0.10.1")

    def test_unknown_duplicate_traversal_and_missing_grants_refuse(self) -> None:
        for name in [self.prefix + "extra", self.prefix + "xwin", self.prefix + "../xwin",
                     "/absolute/xwin", "other-prefix/xwin"]:
            with self.subTest(name=name):
                item = tarfile.TarInfo(name)
                item.size = 1
                with self.assertRaises(RuntimeError):
                    HOSTED.tar_members(self.archive(extra=[(item, b"x")]), "0.10.0")
        for name in self.expected:
            with self.subTest(missing=name), self.assertRaises(RuntimeError):
                HOSTED.tar_members(self.archive(omit=[name]), "0.10.0")

    def test_links_and_special_archive_members_cannot_supply_the_executable(self) -> None:
        for kind in [tarfile.SYMTYPE, tarfile.LNKTYPE, tarfile.FIFOTYPE, tarfile.DIRTYPE]:
            with self.subTest(kind=kind):
                item = tarfile.TarInfo(self.prefix + "xwin")
                item.type = kind
                item.linkname = "outside"
                with self.assertRaises(RuntimeError):
                    HOSTED.tar_members(self.archive(extra=[(item, None)], omit=["xwin"]), "0.10.0")

    def test_declared_expansion_is_refused_before_extracting_a_large_body(self) -> None:
        item = tarfile.TarInfo(self.prefix + "xwin")
        item.size = 64 * 1024 ** 2 + 1
        # The tiny stream declares a huge member. The size guard must decide
        # before extraction; no huge allocation or decompression is necessary.
        blob = gzip.compress(item.tobuf() + b"\0" * 1024)
        with mock.patch.object(tarfile.TarFile, "extractfile",
                               side_effect=AssertionError("oversized member was consumed")):
            with self.assertRaisesRegex(RuntimeError, "[Ee]xpand|[Bb]udget|[Ll]imit|[Ss]ize"):
                HOSTED.tar_members(blob, "0.10.0")


class LayoutControls(TemporaryControls):
    def setUp(self) -> None:
        super().setUp()
        self.runner_temp = self.root / "runner-temp"
        self.runner_temp.mkdir(mode=0o700)
        self.profile = self.runner_temp / "profile"
        self.profile.mkdir(mode=0o700)
        self.payload = self.profile / "payload"
        self.payload.mkdir(mode=0o700)
        self.receipt = self.payload / "receipt.json"
        self.environ = {"GITHUB_ACTIONS": "true", "RUNNER_ENVIRONMENT": "github-hosted",
                        "RUNNER_OS": "Linux", "RUNNER_TEMP": str(self.runner_temp)}
        self.real_stat = os.stat
        self.profile_device = self.profile.stat().st_dev
        self.payload_device = self.profile_device + 1
        self.quota = SimpleNamespace(f_frsize=4096, f_blocks=524288, f_files=65536)

    def mountinfo(self, *, options="rw,nosuid,nodev,noexec", extra="", root="/") -> str:
        major, minor = os.major(self.payload_device), os.minor(self.payload_device)
        return (f"901 1 {major}:{minor} {root} {self.payload} {options} - tmpfs tmpfs rw,size=2097152k,nr_inodes=65536\n"
                + extra)

    @contextlib.contextmanager
    def platform_facts(self, *, mountinfo=None, quota=None, uid=None, platform="linux", module=HOSTED):
        def observed_stat(path, *args, **kwargs):
            result = self.real_stat(path, *args, **kwargs)
            if Path(path) == self.payload:
                values = list(result)
                values[2] = self.payload_device
                return os.stat_result(values)
            return result

        with mock.patch.object(module, "read_mountinfo", return_value=self.mountinfo() if mountinfo is None else mountinfo), \
             mock.patch.object(module.os, "statvfs", return_value=self.quota if quota is None else quota), \
             mock.patch.object(module.os, "stat", side_effect=observed_stat), \
             mock.patch.object(module.os, "geteuid", return_value=os.getuid() if uid is None else uid), \
             mock.patch.object(module.sys, "platform", platform):
            yield

    def test_healthy_precreated_quota_profile_admits_without_creating_cache_or_receipt(self) -> None:
        before = sorted(p.relative_to(self.profile).as_posix() for p in self.profile.rglob("*"))
        with self.platform_facts():
            result = HOSTED.admit_layout(self.profile, self.receipt, self.environ)
        self.assertEqual(result.profile, self.profile)
        self.assertEqual(result.cache, self.payload / "cache")
        self.assertEqual(result.locator, self.payload / "xwin.json")
        self.assertEqual(result.receipt, self.receipt)
        self.assertNotIn(self.payload, result.bootstrap.parents)
        self.assertEqual(sorted(p.relative_to(self.profile).as_posix() for p in self.profile.rglob("*")), before)
        self.assertFalse(result.cache.exists())
        self.assertFalse(self.receipt.exists())

    def test_hosted_markers_ordinary_uid_and_linux_are_mandatory(self) -> None:
        for key in ["GITHUB_ACTIONS", "RUNNER_ENVIRONMENT", "RUNNER_OS", "RUNNER_TEMP"]:
            for value in [None, "untrusted"]:
                env = dict(self.environ)
                if value is None:
                    del env[key]
                else:
                    env[key] = value
                with self.subTest(key=key, value=value), self.platform_facts(), self.assertRaises(RuntimeError):
                    HOSTED.admit_layout(self.profile, self.receipt, env)
        for facts in [{"uid": 0}, {"platform": "win32"}]:
            with self.subTest(facts=facts), self.platform_facts(**facts), self.assertRaises(RuntimeError):
                HOSTED.admit_layout(self.profile, self.receipt, self.environ)

    def test_payload_requires_its_own_exact_quota_mount_and_protective_options(self) -> None:
        cases = ["", self.mountinfo(options="rw,nodev,noexec"), self.mountinfo(options="rw,nosuid,noexec"),
                 self.mountinfo(options="rw,nosuid,nodev"), self.mountinfo(options="ro,nosuid,nodev,noexec"),
                 self.mountinfo(root="/alias"), self.mountinfo().replace("- tmpfs ", "- ext4 ")]
        major, minor = os.major(self.payload_device), os.minor(self.payload_device)
        cases.append(self.mountinfo(extra=f"902 1 {major}:{minor} / {self.runner_temp / 'alias'} rw - tmpfs tmpfs rw\n"))
        cases.append(self.mountinfo(extra=f"902 901 0:99 / {self.payload / 'nested'} rw - tmpfs tmpfs rw\n"))
        cases.append(self.mountinfo().replace(f" {major}:{minor} ", " 0:99999 ", 1))
        for info in cases:
            with self.subTest(info=info), self.platform_facts(mountinfo=info), self.assertRaises(RuntimeError):
                HOSTED.admit_layout(self.profile, self.receipt, self.environ)
        for quota in [SimpleNamespace(f_frsize=4096, f_blocks=524287, f_files=65536),
                      SimpleNamespace(f_frsize=4096, f_blocks=524289, f_files=65536),
                      SimpleNamespace(f_frsize=4096, f_blocks=524288, f_files=65535),
                      SimpleNamespace(f_frsize=4096, f_blocks=524288, f_files=65537)]:
            with self.subTest(quota=quota), self.platform_facts(quota=quota), self.assertRaises(RuntimeError):
                HOSTED.admit_layout(self.profile, self.receipt, self.environ)
        self.payload_device = self.profile_device
        with self.platform_facts(), self.assertRaises(RuntimeError):
            HOSTED.admit_layout(self.profile, self.receipt, self.environ)

    def test_shared_permissions_old_cache_and_old_receipt_refuse(self) -> None:
        for directory in [self.profile, self.payload]:
            directory.chmod(0o750)
            try:
                with self.subTest(directory=directory), self.platform_facts(), self.assertRaises(RuntimeError):
                    HOSTED.admit_layout(self.profile, self.receipt, self.environ)
            finally:
                directory.chmod(0o700)
        cache = self.payload / "cache"
        cache.mkdir()
        try:
            with self.platform_facts(), self.assertRaises(RuntimeError):
                HOSTED.admit_layout(self.profile, self.receipt, self.environ)
        finally:
            cache.rmdir()
        self.receipt.write_bytes(b"prior receipt")
        with self.platform_facts(), self.assertRaises(RuntimeError):
            HOSTED.admit_layout(self.profile, self.receipt, self.environ)
        self.assertEqual(self.receipt.read_bytes(), b"prior receipt")

    def test_preexisting_payload_is_not_a_fresh_acquisition_profile(self) -> None:
        with self.platform_facts():
            self.assertEqual(HOSTED.admit_layout(self.profile, self.receipt, self.environ).receipt, self.receipt)
        retained = self.payload / "prior-input"
        retained.write_bytes(b"unrelated payload must not be adopted")
        with self.platform_facts(), self.assertRaises(RuntimeError):
            HOSTED.admit_layout(self.profile, self.receipt, self.environ)
        self.assertEqual(retained.read_bytes(), b"unrelated payload must not be adopted")

    def test_worker_rechecks_only_fresh_empty_home_and_temporary_directories(self) -> None:
        for name in ["home", "tmp"]:
            (self.payload / name).mkdir(mode=0o700)
        with self.platform_facts():
            self.assertEqual(HOSTED.admit_layout(self.profile, self.receipt, self.environ, worker=True).receipt, self.receipt)
        retained = self.payload / "tmp/prior-input"
        retained.write_bytes(b"prior temporary payload")
        with self.platform_facts(), self.assertRaises(RuntimeError):
            HOSTED.admit_layout(self.profile, self.receipt, self.environ, worker=True)
        self.assertEqual(retained.read_bytes(), b"prior temporary payload")

    def test_receipt_cannot_become_a_member_of_the_frozen_cache_inventory(self) -> None:
        with self.platform_facts():
            self.assertEqual(HOSTED.admit_layout(self.profile, self.receipt, self.environ).receipt, self.receipt)
        inside_cache = self.payload / "cache/receipt.json"
        with self.platform_facts(), self.assertRaises(RuntimeError):
            HOSTED.admit_layout(self.profile, inside_cache, self.environ)
        self.assertFalse((self.payload / "cache").exists())

    def test_profile_alias_and_receipt_outside_payload_refuse(self) -> None:
        alias = self.runner_temp / "alias"
        alias.symlink_to(self.profile, target_is_directory=True)
        with self.platform_facts(), self.assertRaises(RuntimeError):
            HOSTED.admit_layout(alias, self.receipt, self.environ)
        for receipt in [self.profile / "receipt.json", self.root / "outside.json", self.payload / "../receipt.json"]:
            with self.subTest(receipt=receipt), self.platform_facts(), self.assertRaises(RuntimeError):
                HOSTED.admit_layout(self.profile, receipt, self.environ)

    def test_exclusive_owner_refuses_overlap_and_can_be_reacquired_after_release(self) -> None:
        with self.platform_facts(), mock.patch.dict(os.environ, self.environ, clear=True):
            layout = HOSTED.admit_layout(self.profile, self.receipt, self.environ)
            with HOSTED.ownership(layout):
                with self.assertRaises(RuntimeError):
                    with HOSTED.ownership(layout):
                        self.fail("second owner entered the same hosted profile")
            with HOSTED.ownership(layout):
                self.assertFalse(self.receipt.exists())
                self.assertFalse(layout.cache.exists())

    def test_owner_lock_alias_cannot_modify_an_outside_file(self) -> None:
        outside = self.root / "outside.lock"
        outside.write_bytes(b"unrelated lock bytes")
        with self.platform_facts(), mock.patch.dict(os.environ, self.environ, clear=True):
            layout = HOSTED.admit_layout(self.profile, self.receipt, self.environ)
            (self.profile / ".owner.lock").symlink_to(outside)
            with self.assertRaises(RuntimeError):
                with HOSTED.ownership(layout):
                    self.fail("linked lock admitted an owner")
        self.assertEqual(outside.read_bytes(), b"unrelated lock bytes")

    def test_healthy_frontend_admits_before_dispatch_and_leaves_home_untouched(self) -> None:
        home = self.root / "home"
        executable = home / ".cargo/bin/xwin"
        locator = home / ".local/share/nevrotick/xwin.json"
        for path, body in [(executable, b"unrelated home executable"), (locator, b"unrelated home locator")]:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(body)
        before = {p.relative_to(home): p.read_bytes() for p in home.rglob("*") if p.is_file()}
        with self.platform_facts(module=XWIN.HOSTED), \
             mock.patch.dict(os.environ, self.environ, clear=True), \
             mock.patch.object(Path, "home", return_value=home), \
             mock.patch.object(XWIN, "LOCATION", locator), \
             mock.patch.object(sys, "argv", ["setup-xwin.py", "--hosted-runner", "--accept-license",
                                              "--cache-dir", str(self.profile), "--receipt", str(self.receipt)]), \
             mock.patch.object(XWIN.HOSTED, "dispatch") as dispatch, \
             mock.patch.object(XWIN, "download") as download, \
             mock.patch.object(XWIN, "install_xwin") as install, \
             mock.patch.object(XWIN.subprocess, "check_output") as version, \
             mock.patch.object(XWIN.subprocess, "run") as unsupervised:
            XWIN.main()
            dispatch.assert_called_once()
            self.assertEqual(dispatch.call_args.args[0], XWIN.ROOT)
            self.assertEqual(dispatch.call_args.args[1].profile, self.profile)
            self.assertEqual(dispatch.call_args.args[1].receipt, self.receipt)
            self.assertEqual(dispatch.call_args.kwargs, {"keep_cache": False})
            download.assert_not_called()
            install.assert_not_called()
            version.assert_not_called()
            unsupervised.assert_not_called()
        self.assertEqual({p.relative_to(home): p.read_bytes() for p in home.rglob("*") if p.is_file()}, before)
        self.assertFalse((self.payload / "cache").exists())
        self.assertFalse(self.receipt.exists())

    def test_frontend_refuses_bad_markers_missing_receipt_and_old_cache_before_effects(self) -> None:
        for case in ["marker", "receipt", "cache"]:
            env = dict(self.environ)
            args = ["setup-xwin.py", "--hosted-runner", "--accept-license", "--cache-dir", str(self.profile)]
            if case != "receipt":
                args += ["--receipt", str(self.receipt)]
            if case == "marker":
                env["GITHUB_ACTIONS"] = "false"
            if case == "cache":
                (self.payload / "cache").mkdir()
                (self.payload / "cache/prior").write_bytes(b"prior cache bytes")
            with self.subTest(case=case), self.platform_facts(module=XWIN.HOSTED), \
                 mock.patch.dict(os.environ, env, clear=True), mock.patch.object(sys, "argv", args), \
                 mock.patch.object(XWIN.HOSTED, "dispatch") as dispatch, \
                 mock.patch.object(XWIN, "download") as download, \
                 mock.patch.object(XWIN, "install_xwin") as install, \
                 mock.patch.object(XWIN.subprocess, "check_output") as version, \
                 contextlib.redirect_stderr(io.StringIO()):
                with self.assertRaises((RuntimeError, SystemExit)):
                    XWIN.main()
                dispatch.assert_not_called()
                download.assert_not_called()
                install.assert_not_called()
                version.assert_not_called()
                self.assertFalse(self.receipt.exists())
            if case == "cache":
                prior = self.payload / "cache/prior"
                self.assertEqual(prior.read_bytes(), b"prior cache bytes")
                prior.unlink()
                prior.parent.rmdir()


class PrivateEntryControls(TemporaryControls):
    def test_public_worker_selector_refuses_before_dispatch_or_download(self) -> None:
        frontend = load_helper("setup-xwin.py")
        with mock.patch.object(sys, "argv", ["setup-xwin.py", "--hosted-worker", "--accept-license",
                                            "--cache-dir", str(self.root)]), \
             contextlib.redirect_stderr(io.StringIO()), \
             mock.patch.object(frontend.HOSTED, "dispatch") as dispatch, \
             mock.patch.object(frontend, "download") as download:
            with self.assertRaises(SystemExit) as refused:
                frontend.main()
            self.assertEqual(refused.exception.code, 2)
            dispatch.assert_not_called()
            download.assert_not_called()

    def test_private_python_entry_refuses_uncontained_execution_before_effects(self) -> None:
        frontend = load_helper("setup-xwin.py")
        with mock.patch.object(frontend.HOSTED, "dispatch") as dispatch, \
             mock.patch.object(frontend, "download") as download:
            with self.assertRaisesRegex(RuntimeError, "Worker"):
                frontend.main(_hosted_worker=True)
            dispatch.assert_not_called()
            download.assert_not_called()


class NamespaceSurfaceControls(TemporaryControls):
    def test_actual_worker_and_native_namespaces_keep_other_filesystems_read_only(self) -> None:
        profile = self.root / "profile"
        payload, bootstrap = profile / "payload", profile / "bootstrap"
        for directory in [profile, payload, bootstrap, payload / "tmp", payload / "home"]:
            directory.mkdir(mode=0o700)
        layout = SimpleNamespace(profile=profile, payload=payload, bootstrap=bootstrap)
        source_root = Path(__file__).resolve().parents[2]
        for worker in [False, True]:
            with self.subTest(worker=worker):
                environment = {"PATH": "/usr/bin:/bin", "HOME": str(payload / "home"),
                               "LANG": "C.UTF-8", "PAYLOAD": str(payload), "BOOTSTRAP": str(bootstrap),
                               "WORKER": str(int(worker)), "SOURCE_ROOT": str(source_root)}
                code = (
                    "import errno,os,pathlib\n"
                    "assert os.getuid()==os.geteuid()!=0\n"
                    "assert 'NEVROTICK_SYNTHETIC_CREDENTIAL' not in os.environ\n"
                    "if os.environ['WORKER']=='1':\n"
                    " import importlib.util,sys\n"
                    " spec=importlib.util.spec_from_file_location('actual_worker_boundary',pathlib.Path(os.environ['SOURCE_ROOT'])/'scripts/xwin_hosted.py')\n"
                    " m=importlib.util.module_from_spec(spec);sys.modules[spec.name]=m;spec.loader.exec_module(m)\n"
                    " m.require_worker_boundary()\n"
                    "p=pathlib.Path(os.environ['PAYLOAD']);b=pathlib.Path(os.environ['BOOTSTRAP'])\n"
                    "(p/'surface-control').write_bytes(b'owned');print('PAYLOAD_WRITE_OK',flush=True)\n"
                    "if os.environ['WORKER']=='1':(b/'surface-control').write_bytes(b'owned')\n"
                    "targets=[pathlib.Path('/outside-owned-quota'),pathlib.Path('/dev/outside-owned-quota')]\n"
                    "if os.environ['WORKER']=='0':targets.append(b/'forbidden-native-write')\n"
                    "for target in targets:\n"
                    " try:target.write_bytes(b'forbidden')\n"
                    " except OSError as error:assert error.errno in (errno.EROFS,errno.EACCES)\n"
                    " else:raise AssertionError('write escaped owned payload: '+str(target))\n"
                    "print('NAMESPACE_WRITE_SURFACE_OK')\n"
                )
                command = HOSTED.namespace(source_root, layout, worker=worker, environment=environment)
                result = ENV.run([*command, "/usr/bin/python3", "-I", "-B", "-c", code],
                                 cwd=payload, timeout=10, environment={"PATH": "/usr/bin:/bin",
                                     "NEVROTICK_SYNTHETIC_CREDENTIAL": "public-control-canary"})
                self.assertIsNone(result["failure"])
                self.assertEqual(result["exit_code"], 0, result["output"])
                self.assertIn("NAMESPACE_WRITE_SURFACE_OK", result["output"])



class SupervisorAdapterControls(TemporaryControls):
    def run_child(self, code: str, *, timeout=1.0, max_output=1024):
        return HOSTED.run_checked([sys.executable, "-c", code], cwd=self.root,
                                  environment={"PATH": os.defpath, "HOME": str(self.root)},
                                  timeout=timeout, max_output=max_output, runner=ENV.run)

    def test_actual_benign_child_has_a_healthy_version_output(self) -> None:
        result = self.run_child("print('xwin 0.10.0')")
        self.assertEqual(result.strip(), "xwin 0.10.0")

    def test_actual_nonzero_preserves_an_escaped_diagnostic_tail(self) -> None:
        payload = "\n::error::synthetic annotation\x1b[31m\r\n\x00HOSTED_CHILD_DIAGNOSTIC_MARKER"
        code = "import os; os.write(1," + repr(payload.encode()) + "); raise SystemExit(7)"
        with self.assertRaises(RuntimeError) as raised:
            self.run_child(code)
        message = str(raised.exception)
        self.assertIn("HOSTED_CHILD_DIAGNOSTIC_MARKER", message)
        record = json.loads(message)
        self.assertEqual(record["exit_code"], 7)
        self.assertIsNone(record["failure"])
        self.assertEqual(record["output_tail"], payload)
        self.assertLessEqual(len(message), 4096)
        self.assertTrue(message.isascii())
        self.assertFalse(any(ord(character) < 32 or ord(character) == 127 for character in message))

    def test_actual_long_nonzero_diagnostic_stays_bounded_and_keeps_its_tail(self) -> None:
        payload = "discarded-prefix" + "x" * 7000 + "\x00" * 1000 + "HOSTED_CHILD_FINAL_MARKER"
        code = "import os; os.write(1," + repr(payload.encode()) + "); raise SystemExit(9)"
        with self.assertRaises(RuntimeError) as raised:
            self.run_child(code, max_output=16_384)
        message = str(raised.exception)
        self.assertIn("HOSTED_CHILD_FINAL_MARKER", message)
        record = json.loads(message)
        self.assertEqual(record["exit_code"], 9)
        self.assertIsNone(record["failure"])
        self.assertTrue(record["output_tail"].endswith("HOSTED_CHILD_FINAL_MARKER"))
        self.assertNotIn("discarded-prefix", record["output_tail"])
        self.assertLessEqual(len(message), 4096)
        self.assertTrue(message.isascii())
        self.assertFalse(any(ord(character) < 32 or ord(character) == 127 for character in message))

    def test_actual_nonzero_and_timeout_remain_failures(self) -> None:
        for code, timeout, expected in [("raise SystemExit(7)", 1.0, "7"),
                                        ("import time; time.sleep(10)", 0.1, "timeout")]:
            with self.subTest(code=code), self.assertRaisesRegex(RuntimeError, expected):
                self.run_child(code, timeout=timeout)
        self.assertEqual(self.run_child("print('healthy')").strip(), "healthy")

    def test_actual_exact_output_bound_passes_and_next_byte_refuses(self) -> None:
        self.assertEqual(self.run_child("import os; os.write(1,b'x'*8)", max_output=8), "x" * 8)
        with self.assertRaisesRegex(RuntimeError, "output budget"):
            self.run_child("import os; os.write(1,b'x'*9)", max_output=8)
        self.assertEqual(self.run_child("print('healthy')").strip(), "healthy")


if __name__ == "__main__":
    unittest.main()
