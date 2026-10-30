"""Independent fixed sanitizer-profile admission controls; execute no payload."""
from __future__ import annotations

import importlib.util
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("core_runtime_control", ROOT / "scripts/analysis-runtime.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
LEGACY = ("-runs=256", "-max_len=4096", "-timeout=2", "-rss_limit_mb=512",
          "-malloc_limit_mb=256", "-max_total_time=30")
CORE = ("-runs=4096", "-max_len=1537", "-timeout=2", "-rss_limit_mb=512",
        "-malloc_limit_mb=256", "-max_total_time=15")
HISTORY = ("-runs=4096", "-max_len=1561", "-timeout=2", "-rss_limit_mb=512",
           "-malloc_limit_mb=256", "-max_total_time=15")


class CoreRuntimeTests(unittest.TestCase):
    """A selected new profile must neither weaken nor silently replace legacy."""

    def test_legacy_default_preserved_and_core_explicit(self):
        self.assertEqual(MODULE.SANITIZER_FLAGS, LEGACY)
        self.assertEqual(MODULE.CORE_SANITIZER_FLAGS, CORE)
        MODULE._sanitizer(["/work/fuzzer", *LEGACY])
        MODULE._sanitizer(["/work/fuzzer", *LEGACY, "-seed=123"])
        MODULE._sanitizer(["/work/fuzzer", *CORE], "time-core-s12a")
        for flags, profile in [(CORE, "legacy"), (LEGACY, "time-core-s12a")]:
            with self.subTest(profile=profile), self.assertRaises(MODULE.Incomplete):
                MODULE._sanitizer(["/work/fuzzer", *flags], profile)

    def test_unknown_profile_and_changed_missing_duplicate_limits_fail(self):
        for profile in ["", "core", "time-core-s12a-extra", None, True]:
            with self.subTest(profile=profile), self.assertRaises(MODULE.Incomplete):
                MODULE._sanitizer(["/work/fuzzer", *CORE], profile)
        for unknown in ["-seed=123", "-print_final_stats=1", "-artifact_prefix=/tmp/escape/"]:
            with self.subTest(unknown=unknown), self.assertRaises(MODULE.Incomplete):
                MODULE._sanitizer(["/work/fuzzer", *CORE, unknown], "time-core-s12a")
        for index in range(len(CORE)):
            variants = [list(CORE[:index] + CORE[index + 1:]), list(CORE) + [CORE[index]],
                        [*CORE[:index], CORE[index].split("=")[0] + "=999999", *CORE[index + 1:]]]
            for flags in variants:
                with self.subTest(flags=flags), self.assertRaises(MODULE.Incomplete):
                    MODULE._sanitizer(["/work/fuzzer", *flags], "time-core-s12a")

    def test_failure_handling_and_worker_overrides_remain_forbidden(self):
        for flag in ["-fork=1", "-jobs=2", "-workers=2", "-merge=1", "-minimize_crash=1",
                     "-ignore_crashes=1", "-handle_segv=0", "-error_exitcode=0", "-timeout_exitcode=0"]:
            for flags, profile in [(LEGACY, "legacy"), (CORE, "time-core-s12a"),
                                   (HISTORY, "time-core-history-s12d")]:
                with self.subTest(flag=flag, profile=profile), self.assertRaises(MODULE.Incomplete):
                    MODULE._sanitizer(["/work/fuzzer", *flags, flag], profile)

    def test_namespace_and_ordinary_limits_remain_separate(self):
        with tempfile.TemporaryDirectory() as temp:
            root, work = Path(temp) / "root", Path(temp) / "work"
            root.mkdir(mode=0o700); work.mkdir(mode=0o700)
            helpers = {"bwrap": "/reviewed/bwrap", "prlimit": "/reviewed/prlimit"}
            owner = work.stat().st_uid
            self.assertNotEqual(owner, 0, "Namespace controls require an ordinary-user test environment.")
            with patch.object(MODULE.os, "geteuid", return_value=owner), patch.object(MODULE, "_helpers", return_value=helpers):
                ordinary = MODULE.sandbox_command(root, work, ["/work/control"])
                core = MODULE.sandbox_command(root, work, ["/work/fuzzer", *CORE], mode="sanitizer",
                                              sanitizer_profile="time-core-s12a")
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.sandbox_command(root, work, ["/work/control"], sanitizer_profile="time-core-s12a")
            self.assertTrue(any(arg.startswith("--as=") for arg in ordinary))
            self.assertFalse(any(arg.startswith("--as=") for arg in core))
            for command in [ordinary, core]:
                for required in ["--unshare-all", "--clearenv", "--die-with-parent", "--cap-drop", "--nproc=128"]:
                    self.assertIn(required, command)
                self.assertIn("CARGO_NET_OFFLINE", command)

    def test_history_profile_has_exact_independent_limits_and_rejects_other_profiles(self):
        self.assertEqual(MODULE.HISTORY_SANITIZER_PROFILE, "time-core-history-s12d")
        self.assertEqual(MODULE.HISTORY_SANITIZER_FLAGS, HISTORY)
        MODULE._sanitizer(["/work/fuzzer", *HISTORY], "time-core-history-s12d")
        MODULE._sanitizer(["/work/fuzzer", "-artifact_prefix=/work/fuzz-artifacts/", *HISTORY],
                          "time-core-history-s12d")
        for flags, profile in [(HISTORY, "legacy"), (HISTORY, "time-core-s12a"),
                               (LEGACY, "time-core-history-s12d"), (CORE, "time-core-history-s12d")]:
            with self.subTest(profile=profile, flags=flags), self.assertRaises(MODULE.Incomplete):
                MODULE._sanitizer(["/work/fuzzer", *flags], profile)
        for profile in ["time-core-history", "time-core-history-s12d-extra"]:
            with self.subTest(profile=profile), self.assertRaises(MODULE.Incomplete):
                MODULE._sanitizer(["/work/fuzzer", *HISTORY], profile)
        for unknown in ["-seed=123", "-print_final_stats=1", "-artifact_prefix=/tmp/escape/"]:
            with self.subTest(unknown=unknown), self.assertRaises(MODULE.Incomplete):
                MODULE._sanitizer(["/work/fuzzer", *HISTORY, unknown], "time-core-history-s12d")
        for index in range(len(HISTORY)):
            variants = [list(HISTORY[:index] + HISTORY[index + 1:]), list(HISTORY) + [HISTORY[index]],
                        [*HISTORY[:index], HISTORY[index].split("=")[0] + "=999999", *HISTORY[index + 1:]]]
            for flags in variants:
                with self.subTest(flags=flags), self.assertRaises(MODULE.Incomplete):
                    MODULE._sanitizer(["/work/fuzzer", *flags], "time-core-history-s12d")

    def test_history_namespace_preserves_build_jobs_and_rejects_ordinary_mode(self):
        with tempfile.TemporaryDirectory() as temp:
            root, work = Path(temp) / "root", Path(temp) / "work"
            root.mkdir(mode=0o700); work.mkdir(mode=0o700)
            helpers = {"bwrap": "/reviewed/bwrap", "prlimit": "/reviewed/prlimit"}
            owner = work.stat().st_uid
            self.assertNotEqual(owner, 0, "Namespace controls require an ordinary-user test environment.")
            with patch.object(MODULE.os, "geteuid", return_value=owner), patch.object(MODULE, "_helpers", return_value=helpers):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.sandbox_command(root, work, ["/work/control"],
                                           sanitizer_profile="time-core-history-s12d")
                for options, expected_jobs in [({}, "12"), ({"build_jobs": 1}, "1")]:
                    with self.subTest(build_jobs=expected_jobs):
                        command = MODULE.sandbox_command(root, work, ["/work/fuzzer", *HISTORY],
                            mode="sanitizer", sanitizer_profile="time-core-history-s12d", **options)
                        self.assertFalse(any(arg.startswith("--as=") for arg in command))
                        self.assertEqual(command[command.index("CARGO_BUILD_JOBS") + 1], expected_jobs)
                        for required in ["--unshare-all", "--clearenv", "--die-with-parent", "--cap-drop", "--nproc=128"]:
                            self.assertIn(required, command)
                        self.assertIn("CARGO_NET_OFFLINE", command)


if __name__ == "__main__":
    unittest.main()
