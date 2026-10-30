"""Independent namespace controls for the shared bounded analysis runtime."""

import importlib.util
import inspect
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("analysis_runtime", ROOT / "scripts/analysis-runtime.py")
RUNTIME = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(RUNTIME)


class RuntimeControls(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="nevrotick-runtime-control-")
        self.addCleanup(self.temporary.cleanup)
        self.scratch = Path(self.temporary.name)
        self.work = self.scratch / "work"
        self.source = self.scratch / "readonly"
        self.work.mkdir(mode=0o700)
        (self.source / "bin").mkdir(parents=True)

    def build(self, command=None, **options):
        return RUNTIME.sandbox_command(ROOT, self.work, command or ["/usr/bin/true"], **options)

    def test_required_namespace_and_inside_namespace_limits(self):
        argv = self.build(readonly=[self.source], path_dirs=[self.source / "bin"])
        for required in ("--unshare-all", "--die-with-parent", "--new-session", "--clearenv"):
            self.assertIn(required, argv)
        self.assertEqual(argv[argv.index("--cap-drop") + 1], "ALL")
        split = argv.index("--")
        self.assertEqual(argv[split + 1], "/usr/bin/prlimit")
        for limit in ("--as=8589934592", "--cpu=120", "--nproc=128", "--nofile=256", "--fsize=268435456"):
            self.assertIn(limit, argv[split + 2:])
        self.assertNotIn("--tmpfs", argv)
        self.assertIn(str(self.source), argv)
        binds = [argv[index + 1:index + 3] for index, item in enumerate(argv) if item == "--bind"]
        self.assertEqual(binds, [[str(self.work), "/work"], [str(self.work / "tmp"), "/tmp"]])

    def test_missing_or_substituted_helpers_fail_before_execution(self):
        with patch.object(RUNTIME.ENV, "require_admitted_binary", side_effect=RUNTIME.Incomplete("digest mismatch")):
            with self.assertRaisesRegex(RUNTIME.Incomplete, "digest mismatch"):
                self.build()
        policy = json.loads((ROOT / "policy/tool-manifest.json").read_text())
        policy["bootstrap_helpers"]["bwrap"]["path"] = "/does-not-exist/bwrap"
        with patch.object(RUNTIME.json, "loads", return_value=policy), \
             patch.object(RUNTIME.ENV, "run") as execute:
            with self.assertRaises((FileNotFoundError, RUNTIME.Incomplete)):
                RUNTIME.run(ROOT, self.work, ["/usr/bin/true"])
        execute.assert_not_called()

    def test_broad_overlap_and_symlink_mounts_are_rejected(self):
        for source in (Path("/"), Path("/etc"), Path("/home"), Path.home(), self.work, self.work.parent):
            with self.subTest(source=source), self.assertRaises(RUNTIME.Incomplete):
                self.build(readonly=[source])
        redirected = self.scratch / "redirected"
        redirected.symlink_to(self.source, target_is_directory=True)
        with self.assertRaises(RUNTIME.Incomplete):
            self.build(readonly=[redirected])
        with self.assertRaises(RUNTIME.Incomplete):
            RUNTIME.sandbox_command(ROOT, ROOT, ["/usr/bin/true"])

    def test_drive_and_actual_mount_roots_are_rejected_but_payload_paths_are_allowed(self):
        for root in (Path("/mnt/c"), Path("/mnt/d")):
            with self.subTest(root=root), self.assertRaises(RUNTIME.Incomplete):
                RUNTIME._narrow(root)
        RUNTIME._narrow(Path("/mnt/c/Users/owned/nevrotick-analysis"))
        with patch.object(Path, "is_mount", return_value=True):
            with self.assertRaises(RUNTIME.Incomplete):
                RUNTIME._narrow(self.source)

    def test_work_ownership_modes_links_and_path_admission(self):
        self.work.chmod(0o755)
        with self.assertRaises(RUNTIME.Incomplete):
            self.build()
        self.work.chmod(0o700)
        (self.work / "escape").symlink_to(self.source)
        with self.assertRaises(RUNTIME.Incomplete):
            self.build()
        (self.work / "escape").unlink()
        sentinel = self.source / "protected"
        sentinel.write_text("protected")
        (self.work / "hardlink").hardlink_to(sentinel)
        with self.assertRaises(RUNTIME.Incomplete):
            self.build()
        (self.work / "hardlink").unlink()
        with self.assertRaises(RUNTIME.Incomplete):
            self.build(path_dirs=[self.source / "bin"])

    def test_cargo_style_internal_hardlinks_and_file_symlinks_can_be_reused(self):
        deps = self.work / "target/debug/deps"
        deps.mkdir(parents=True)
        binary = deps / "fixture-hash"
        binary.write_text("compiled fixture")
        (deps.parent / "fixture").hardlink_to(binary)
        (deps.parent / "fixture-link").symlink_to("deps/fixture-hash")
        probe = (
            "from pathlib import Path\n"
            "binary=Path('/work/target/debug/fixture')\n"
            "assert binary.samefile('/work/target/debug/deps/fixture-hash')\n"
            "link=Path('/work/target/debug/fixture-link')\n"
            "assert link.read_text() == 'compiled fixture'\n"
            "link.write_text('reused fixture')\n"
            "assert binary.read_text() == 'reused fixture'\n"
            "print('REUSE_OBSERVED')\n"
        )
        result = RUNTIME.run(ROOT, self.work, ["/usr/bin/python3", "-c", probe], timeout=10)
        self.assertEqual(RUNTIME.ENV.require_ok(result), "REUSE_OBSERVED")
        self.assertEqual(binary.read_text(), "reused fixture")

    def test_external_hardlink_alias_is_rejected_even_with_multiple_internal_aliases(self):
        first = self.work / "first"
        first.write_text("must stay protected")
        (self.work / "second").hardlink_to(first)
        external = self.source / "external-alias"
        external.hardlink_to(first)
        with self.assertRaises(RUNTIME.Incomplete):
            self.build()
        self.assertEqual(external.read_text(), "must stay protected")

    def test_escaping_broken_and_directory_symlinks_are_rejected(self):
        target = self.work / "owned"
        target.write_text("owned")
        link = self.work / "link"
        for destination in (self.source, self.source / "missing", self.work / "missing", self.work):
            with self.subTest(destination=destination):
                link.symlink_to(destination)
                try:
                    with self.assertRaises(RUNTIME.Incomplete):
                        self.build()
                finally:
                    link.unlink()

    def test_sealed_inputs_accept_internal_directories_and_files_after_work_bind(self):
        controls = self.work / "controls"
        controls.mkdir()
        (controls / "lib.rs").write_text("canonical source")
        lockfile = self.work / "fixture.lock"
        lockfile.write_text("canonical lock")
        argv = self.build(sealed=[controls, lockfile])
        work_bind = argv.index("--bind")
        sealed_binds = [(index, argv[index + 1:index + 3]) for index, item in enumerate(argv)
                        if item == "--ro-bind" and argv[index + 1].startswith(str(self.work))]
        self.assertEqual([values for _, values in sealed_binds],
                         [[str(controls), "/work/controls"], [str(lockfile), "/work/fixture.lock"]])
        self.assertTrue(all(index > work_bind for index, _ in sealed_binds))

    def test_sealed_inputs_reject_outside_broad_duplicate_and_reserved_paths(self):
        controls = self.work / "controls"
        (controls / "nested").mkdir(parents=True)
        for sealed in ([self.source], [self.work], [controls, controls],
                       [controls, controls / "nested"]):
            with self.subTest(sealed=sealed), self.assertRaises(RUNTIME.Incomplete):
                self.build(sealed=sealed)
        for name in ("home", "tmp", "target"):
            reserved = self.work / name
            reserved.mkdir()
            with self.subTest(reserved=name), self.assertRaises(RUNTIME.Incomplete):
                self.build(sealed=[reserved])

    def test_sealed_inputs_reject_symlinks_and_writable_hardlink_aliases(self):
        controls = self.work / "controls"
        controls.mkdir()
        source = controls / "lib.rs"
        source.write_text("canonical source")
        alias = self.work / "writable-alias"
        alias.hardlink_to(source)
        with self.assertRaises(RUNTIME.Incomplete):
            self.build(sealed=[controls])
        alias.unlink()
        # Hardlinks wholly inside the sealed set have no writable alias.
        (controls / "same-source").hardlink_to(source)
        self.build(sealed=[controls])
        link = controls / "source-link"
        link.symlink_to("lib.rs")
        with self.assertRaises(RUNTIME.Incomplete):
            self.build(sealed=[controls])
        link.unlink()
        redirected = self.work / "redirected"
        redirected.symlink_to(controls, target_is_directory=True)
        with self.assertRaises(RUNTIME.Incomplete):
            self.build(sealed=[redirected])

    def test_actual_sealed_sources_reject_mutation_and_keep_outputs_writable(self):
        controls = self.work / "controls"
        (controls / ".cargo").mkdir(parents=True)
        source = controls / "lib.rs"
        source.write_text("canonical source")
        configuration = controls / ".cargo/config.toml"
        configuration.write_text("canonical configuration")
        lockfile = self.work / "fixture.lock"
        lockfile.write_text("canonical lock")
        probe = (
            "from pathlib import Path\n"
            "for name in ('/work/controls/lib.rs', '/work/controls/.cargo/config.toml', "
            "'/work/controls/added.rs', '/work/fixture.lock'):\n"
            " try: Path(name).write_text('mutated input')\n"
            " except OSError: pass\n"
            " else: raise SystemExit('sealed input remained writable: '+name)\n"
            "Path('/work/healthy-output').write_text('healthy')\n"
            "Path('/tmp/healthy-temp').write_text('temporary')\n"
            "print('SEALED_INPUTS_OBSERVED')\n"
        )
        result = RUNTIME.run(ROOT, self.work, ["/usr/bin/python3", "-c", probe],
                             sealed=[controls, lockfile], timeout=10)
        self.assertEqual(RUNTIME.ENV.require_ok(result), "SEALED_INPUTS_OBSERVED")
        self.assertEqual(source.read_text(), "canonical source")
        self.assertEqual(configuration.read_text(), "canonical configuration")
        self.assertEqual(lockfile.read_text(), "canonical lock")
        self.assertFalse((controls / "added.rs").exists())
        self.assertEqual((self.work / "healthy-output").read_text(), "healthy")
        self.assertEqual((self.work / "tmp/healthy-temp").read_text(), "temporary")

    def test_reserved_environment_cannot_weaken_offline_or_loader_controls(self):
        for key in ("CARGO_NET_OFFLINE", "HOME", "PATH", "TMPDIR", "LD_PRELOAD"):
            with self.subTest(variable=key), self.assertRaises(RUNTIME.Incomplete):
                self.build(environment={key: "untrusted"})

    def test_worker_cargo_target_is_absent_and_reserved_in_both_modes(self):
        def child_environment(argv):
            namespace = argv[:argv.index("--")]
            return [(namespace[index + 1], namespace[index + 2])
                    for index, item in enumerate(namespace) if item == "--setenv"]

        default = child_environment(self.build())
        self.assertEqual([value for key, value in default if key == "CARGO_TARGET_DIR"],
                         ["/work/target"])

        # On the old API, observe its actual default child environment so the
        # behavioral RED identifies the shared target, rather than a TypeError.
        has_mode = "cargo_target_mode" in inspect.signature(RUNTIME.sandbox_command).parameters
        worker_options = {"cargo_target_mode": "worker"} if has_mode else {}
        worker = child_environment(self.build(**worker_options))
        self.assertNotIn("CARGO_TARGET_DIR", [key for key, _ in worker])
        self.assertEqual(child_environment(self.build(cargo_target_mode="shared")), default)
        for key in ("PATH", "HOME", "TMPDIR", "CARGO_HOME", "CARGO_NET_OFFLINE",
                    "CARGO_INCREMENTAL", "CARGO_BUILD_JOBS", "CARGO_TERM_COLOR"):
            self.assertEqual([value for name, value in worker if name == key],
                             [value for name, value in default if name == key])

        for options in ({}, {"cargo_target_mode": "shared"}, {"cargo_target_mode": "worker"}):
            for key in ("CARGO_TARGET_DIR", "CARGO_BUILD_TARGET_DIR", "CARGO_BUILD_BUILD_DIR"):
                with self.subTest(options=options, variable=key), self.assertRaises(RUNTIME.Incomplete):
                    self.build(**options, environment={key: "/work/override"})
        with patch.object(RUNTIME.ENV, "run") as execute:
            for invalid in ("", "other", None, True):
                with self.subTest(invalid=invalid), self.assertRaises(RUNTIME.Incomplete):
                    RUNTIME.run(ROOT, self.work, ["/usr/bin/true"], cargo_target_mode=invalid)
            execute.assert_not_called()

    def test_build_parallelism_has_twelve_slots_and_explicit_serial_neighbor(self):
        for options, expected in (({}, "12"), ({"build_jobs": 1}, "1")):
            with self.subTest(options=options):
                command = self.build(**options)
                index = command.index("CARGO_BUILD_JOBS")
                self.assertEqual(command[index + 1], expected)
                self.assertIn("--nproc=128", command)
        for invalid in (True, 0, -1, 13, "12", 1.0):
            with self.subTest(invalid=invalid), self.assertRaises(RUNTIME.Incomplete):
                self.build(build_jobs=invalid)
        with self.assertRaises(RUNTIME.Incomplete):
            self.build(environment={"CARGO_BUILD_JOBS": "99"})

    def test_mutation_process_profile_changes_only_the_inside_namespace_task_limit(self):
        options = {"cargo_target_mode": "worker", "build_jobs": 1}
        default = self.build(**options)
        selected = self.build(**options, process_limit=512)
        self.assertEqual([item for item in default if item.startswith("--nproc=")], ["--nproc=128"])
        self.assertEqual([item for item in selected if item.startswith("--nproc=")], ["--nproc=512"])
        self.assertEqual([item for item in default if not item.startswith("--nproc=")],
                         [item for item in selected if not item.startswith("--nproc=")])
        self.assertIn("--unshare-all", selected)
        self.assertIn("--as=8589934592", selected)
        self.assertNotIn("CARGO_TARGET_DIR", selected)
        self.assertEqual(selected[selected.index("CARGO_BUILD_JOBS") + 1], "1")

    def test_process_limit_rejects_unknown_values_and_profile_borrowing_before_execution(self):
        worker = {"cargo_target_mode": "worker", "build_jobs": 1}
        invalid_values = (True, False, None, "512", 512.0, 0, -1, 127, 129, 256, 513, 1024)
        invalid_profiles = ({}, {"cargo_target_mode": "shared", "build_jobs": 1},
                            {"cargo_target_mode": "worker", "build_jobs": 12},
                            {"cargo_target_mode": "worker", "build_jobs": 1, "mode": "sanitizer"})
        with patch.object(RUNTIME.ENV, "run") as execute:
            for value in invalid_values:
                with self.subTest(value=value), self.assertRaises(RUNTIME.Incomplete):
                    RUNTIME.run(ROOT, self.work, ["/usr/bin/true"], **worker, process_limit=value)
            for options in invalid_profiles:
                with self.subTest(options=options), self.assertRaises(RUNTIME.Incomplete):
                    RUNTIME.run(ROOT, self.work, ["/usr/bin/true"], **options, process_limit=512)
            execute.assert_not_called()

    def test_actual_namespace_receives_admitted_jobs_instead_of_ambient_budget(self):
        probe = "import os; print(os.environ['CARGO_BUILD_JOBS'])"
        with patch.dict(os.environ, {"CARGO_BUILD_JOBS": "99"}):
            for jobs in (12, 1):
                with self.subTest(jobs=jobs):
                    result = RUNTIME.run(ROOT, self.work, ["/usr/bin/python3", "-c", probe],
                                         build_jobs=jobs, timeout=10)
                    self.assertEqual(RUNTIME.ENV.require_ok(result), str(jobs))

    def test_sanitizer_requires_fixed_limits_and_omits_address_space_cap(self):
        argv = self.build(["/work/fuzz-target", *RUNTIME.SANITIZER_FLAGS], mode="sanitizer")
        self.assertFalse(any(item.startswith("--as=") for item in argv))
        self.assertIn("--nproc=128", argv)
        for flags in ([], list(RUNTIME.SANITIZER_FLAGS[:-1]),
                      [*RUNTIME.SANITIZER_FLAGS, "-runs=0"], [*RUNTIME.SANITIZER_FLAGS, "-fork=8"],
                      [*RUNTIME.SANITIZER_FLAGS, "-error_exitcode=0"],
                      [*RUNTIME.SANITIZER_FLAGS, "-ignore_crashes=1"]):
            with self.assertRaises(RUNTIME.Incomplete):
                self.build(["/work/fuzz-target", *flags], mode="sanitizer")

    def test_run_passes_no_ambient_environment_and_preserves_native_exit(self):
        native = {"exit_code": 7, "output": "controlled", "failure": None}
        with patch.dict(os.environ, {"SYNTHETIC_SECRET": "never-copy"}), \
             patch.object(RUNTIME.ENV, "run", return_value=native) as execute:
            self.assertIs(RUNTIME.run(ROOT, self.work, ["/usr/bin/false"]), native)
        self.assertEqual(execute.call_args.kwargs["environment"], {"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"})
        self.assertEqual(execute.call_args.kwargs["stdin"], subprocess.DEVNULL)
        self.assertNotIn("never-copy", str(execute.call_args))

    def test_actual_namespace_has_healthy_output_and_rejects_host_access(self):
        protected = self.source / "protected"
        protected.write_text("unchanged")
        with socket.socket() as listener:
            listener.bind(("127.0.0.1", 0))
            listener.listen(1)
            port = listener.getsockname()[1]
            with socket.create_connection(("127.0.0.1", port), timeout=1):
                accepted, _ = listener.accept()
                accepted.close()
            probe = (
                "import os,pathlib,resource,socket\n"
                "assert 'SYNTHETIC_SECRET' not in os.environ\n"
                "assert os.environ['CARGO_NET_OFFLINE'] == 'true'\n"
                "assert resource.getrlimit(resource.RLIMIT_AS) == (8589934592,8589934592)\n"
                "assert resource.getrlimit(resource.RLIMIT_CPU) == (120,120)\n"
                "assert resource.getrlimit(resource.RLIMIT_NPROC) == (128,128)\n"
                "assert resource.getrlimit(resource.RLIMIT_NOFILE) == (256,256)\n"
                "assert resource.getrlimit(resource.RLIMIT_FSIZE) == (268435456,268435456)\n"
                f"assert not pathlib.Path({str(Path.home())!r}).exists()\n"
                f"protected=pathlib.Path({str(protected)!r})\n"
                "assert protected.read_text() == 'unchanged'\n"
                "try: protected.write_text('modified')\n"
                "except OSError: pass\n"
                "else: raise SystemExit('readonly boundary failed')\n"
                "s=socket.socket();s.settimeout(1)\n"
                f"try: s.connect(('127.0.0.1',{port}))\n"
                "except OSError: pass\n"
                "else: raise SystemExit('network boundary failed')\n"
                "finally: s.close()\n"
                "pathlib.Path('/work/result').write_text('healthy output')\n"
                "pathlib.Path('/tmp/temp-result').write_text('owned temporary output')\n"
                "print('BOUNDARIES_OBSERVED')\n"
            )
            with patch.dict(os.environ, {"SYNTHETIC_SECRET": "never-copy"}):
                result = RUNTIME.run(ROOT, self.work, ["/usr/bin/python3", "-c", probe],
                                     readonly=[self.source], timeout=10)
        self.assertEqual(RUNTIME.ENV.require_ok(result), "BOUNDARIES_OBSERVED")
        self.assertEqual(protected.read_text(), "unchanged")
        self.assertEqual((self.work / "result").read_text(), "healthy output")
        self.assertEqual((self.work / "tmp/temp-result").read_text(), "owned temporary output")

    def test_actual_namespace_preserves_nonzero_exit(self):
        result = RUNTIME.run(ROOT, self.work,
                             ["/usr/bin/python3", "-c", "raise SystemExit(7)"], timeout=10)
        self.assertEqual(result["exit_code"], 7)
        self.assertIsNone(result["failure"])

    def test_actual_namespace_cannot_read_ambient_stdin(self):
        # This outer process owns a real input pipe. A mock of Popen arguments
        # alone cannot establish whether the namespace's fd0 exposes its bytes.
        wrapper = (
            "import importlib.util,json,sys\n"
            "from pathlib import Path\n"
            "root,work=map(Path,sys.argv[1:])\n"
            "spec=importlib.util.spec_from_file_location('runtime',root/'scripts/analysis-runtime.py')\n"
            "runtime=importlib.util.module_from_spec(spec);spec.loader.exec_module(runtime)\n"
            "probe=\"import sys; data=sys.stdin.read(); print('STDIN_LEAK:'+data if data else 'STDIN_EOF'); print('HEALTHY_OUTPUT')\"\n"
            "result=runtime.run(root,work,['/usr/bin/python3','-c',probe],timeout=10)\n"
            "print(json.dumps(result))\n"
        )
        observed = subprocess.run(
            [sys.executable, "-B", "-c", wrapper, str(ROOT), str(self.work)],
            input="synthetic-stdin-canary", capture_output=True, text=True,
            timeout=20, check=True, env={"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"},
        )
        result = json.loads(observed.stdout)
        self.assertEqual(result["exit_code"], 0)
        self.assertIsNone(result["failure"])
        self.assertEqual(result["output"], "STDIN_EOF\nHEALTHY_OUTPUT\n")


if __name__ == "__main__":
    unittest.main()
