#!/usr/bin/env python3
"""Controls for the service-local portable host admission and sandbox adapter."""
from __future__ import annotations

import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("fixture_qualification_controls", Path(__file__).with_name("qualification.py"))
Q = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(Q)


class HostAdmissionTests(unittest.TestCase):
    """Machine relocation must work while changed inputs and unsafe mounts fail closed."""

    def test_current_helpers_do_not_need_historical_ubuntu_checksums(self):
        """Current public executables are recorded without comparing an old host pin."""
        with patch.object(Q.C.PRODUCT, "sha256", return_value="a" * 64):
            helpers = Q.host_helpers()
        self.assertEqual(set(helpers), {"bwrap", "prlimit", "cc", "ld", "python3"})
        self.assertTrue(all(item["sha256"] == "a" * 64 for item in helpers.values()))
        self.assertTrue(all(Path(item["resolved"]).is_relative_to("/usr") for item in helpers.values()))

    def test_missing_helper_is_not_a_host_execution_fallback(self):
        """A missing namespace helper is an incomplete environment, never direct execution."""
        with patch.object(Q.shutil, "which", return_value=None):
            with self.assertRaisesRegex(Q.Incomplete, "helper is missing"):
                Q.host_helpers()

    def test_compiler_snapshot_records_contained_links_and_rejects_escape(self):
        """Observe the whole selected payload and reject redirected library custody."""
        with tempfile.TemporaryDirectory(prefix="service-tool-control-") as directory:
            stable = Path(directory)
            for name in ("bin", "etc", "lib", "libexec", "share"):
                (stable / name).mkdir()
            (stable / "bin/compiler").write_bytes(b"compiler-control")
            (stable / "lib/owned").write_bytes(b"library-control")
            (stable / "lib/alias").symlink_to("owned")
            found = Q.compiler_snapshot(stable)
            self.assertEqual(set(found), {"bin/compiler", "lib/owned", "lib/alias"})
            self.assertEqual(found["lib/alias"], {"kind": "symlink", "target": "owned"})
            (stable / "lib/escape").symlink_to("/usr/bin/python3")
            with self.assertRaisesRegex(Q.Incomplete, "escapes"):
                Q.compiler_snapshot(stable)

    def test_command_argv_isolated_inputs_and_resource_profiles(self):
        """Keep the existing namespace/custody protections with actual host helpers."""
        for mutation in (False, True):
            with self.subTest(mutation=mutation), tempfile.TemporaryDirectory(prefix="service-runtime-control-") as directory:
                parent = Path(directory)
                work = parent / "work"
                work.mkdir(mode=0o700)
                source = work / "source"
                source.mkdir()
                vendor = parent / "vendor"
                vendor.mkdir()
                stable = parent / "compiler"
                stable.mkdir()
                config = work / "cargo-home/config.toml"
                config.parent.mkdir()
                config.write_text("source-control\n")
                admitted = {"stable": stable, "extensions": {}, "helpers": Q.host_helpers()}
                with patch.object(Q, "check_service_tools") as check, patch.object(
                    Q.C.PRODUCT.ENV, "run", return_value={"exit_code": 0, "output": "controlled", "failure": None}
                ) as execute:
                    result = Q.run_command(work, [str(stable / "bin/cargo"), "--version"], admitted,
                                           vendor, source, timeout=180, mutation=mutation)
                self.assertEqual(check.call_count, 2)
                self.assertEqual(result["namespace_argv"], ["/toolchain/bin/cargo", "--version"])
                args = execute.call_args.args[0]
                self.assertIn("--unshare-all", args)
                self.assertIn("--clearenv", args)
                self.assertIn("--cap-drop", args)
                self.assertIn("--as=8589934592", args)
                self.assertIn("--cpu=180", args)
                self.assertIn("--nproc=512" if mutation else "--nproc=128", args)
                self.assertIn("--nofile=4096" if mutation else "--nofile=256", args)
                self.assertIn("--fsize=268435456", args)
                self.assertIn(["--ro-bind", str(source), "/work/source"], [args[i:i+3] for i in range(len(args)-2)])
                self.assertIn(["--ro-bind", str(vendor), "/vendor"], [args[i:i+3] for i in range(len(args)-2)])
                settings = {args[i+1]: args[i+2] for i, value in enumerate(args[:-2]) if value == "--setenv"}
                self.assertEqual(settings["CARGO_NET_OFFLINE"], "true")
                self.assertEqual(settings["CARGO_BUILD_JOBS"], "1")
                self.assertEqual(settings["RUST_TEST_THREADS"], "1")
                self.assertEqual(settings.get("CARGO_TARGET_DIR"), None if mutation else "/work/target")
                self.assertEqual(settings.get("RUST_TEST_NOCAPTURE"), "1" if mutation else None)
                self.assertEqual(execute.call_args.kwargs["environment"], {"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"})

    def test_bad_command_and_budget_are_rejected_before_execution(self):
        """Reject empty/ambiguous input before touching a process or owned directory."""
        for command, options in (([], {}), (["tool\0name"], {}), (["tool"], {"timeout": float("inf")}),
                                 (["tool"], {"timeout": True}), (["tool"], {"mutation": "yes"})):
            with self.subTest(command=command, options=options), patch.object(Q.C.PRODUCT.ENV, "run") as execute:
                with self.assertRaises(Q.Incomplete):
                    Q.run_command(Path("/not-used"), command, {}, Path("/not-used"), Path("/not-used"), **options)
                execute.assert_not_called()


if __name__ == "__main__":
    unittest.main()
