#!/usr/bin/env python3
"""Bounded admission/classifier tests; no compiler or sanitizer is executed."""
import contextlib
import importlib.util
import io
import json
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("sanitizer_check", Path(__file__).with_name("check.py"))
CHECK = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(CHECK)


class Controls(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix="sanitizer-unit-")
        self.base = Path(self.temp.name)
        self.repository = self.base / "repository"
        self.repository.mkdir()
        self.sources = self.repository / "tools/sanitizers"
        (self.sources / "fixtures").mkdir(parents=True)
        self.roots = {name: self.base / name for name in ("rust", "clang", "sysroots")}
        payloads = []
        for name, relative in (("rust", "bin/rustc"), ("clang", "bin/clang-22"), ("clang", "bin/llvm-symbolizer"),
                               ("clang", "bin/x86_64-linux-gnu-gcc-13"), ("clang", "bin/x86_64-linux-gnu-ld.bfd"),
                               ("clang", "libexec/gcc/x86_64-linux-gnu/13/collect2"),
                               ("clang", "libexec/gcc/x86_64-linux-gnu/13/liblto_plugin.so"),
                               *(("sysroots", f"{folder}/lib/rustlib/{CHECK.PLATFORM}/lib/{name}")
                                 for tool, folder in CHECK.SYSROOTS.items()
                                 for name in ("libcore-123abc.rlib", "libcore-123abc.rmeta",
                                              "libcompiler_builtins-123abc.rlib", "libcompiler_builtins-123abc.rmeta",
                                              f"librustc-nightly_rt.{tool}.a"))):
            path = self.roots[name] / relative
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(b"unit payload " + relative.encode())
            path.chmod(0o500)
            payloads.append({"root": name, "path": relative, "sha256": CHECK.digest(path)})
        inputs = {}
        for name in CHECK.SOURCES.values():
            path = self.sources / name
            path.write_text("// unit fixture\n")
            inputs[name] = CHECK.digest(path)
        self.policy = {"schema_version": 1, "platform": CHECK.PLATFORM,
                       "payloads": payloads, "rust_compiler": "bin/rustc",
                       "native_compiler": "bin/clang-22", "symbolizer": "bin/llvm-symbolizer",
                       "sysroots": dict(CHECK.SYSROOTS),
                       "tools": dict(CHECK.SOURCES), "inputs": inputs,
                       "helpers": {name: {"path": "/usr/bin/" + name, "sha256": "0" * 64}
                                   for name in ("bwrap", "prlimit")}}
        self.policy_path = self.sources / "policy.json"
        self.policy_path.write_text(json.dumps(self.policy))
        self.pointer = self.base / "installation.json"
        self.pointer.write_text(json.dumps({"schema_version": 1, "roots": {key: str(value) for key, value in self.roots.items()}}))
        self.pointer.chmod(0o600)
        self.real_digest = CHECK.digest

    def tearDown(self):
        self.temp.cleanup()

    def admission(self):
        def digest(path):
            return "0" * 64 if str(path) in ("/usr/bin/bwrap", "/usr/bin/prlimit") else self.real_digest(path)
        with patch.object(CHECK, "digest", side_effect=digest), patch.object(CHECK.os, "access", return_value=True), \
                patch.object(CHECK, "RUST_LINKER", str(self.roots["clang"] / "bin/x86_64-linux-gnu-gcc-13")), \
                patch.object(CHECK, "NATIVE_LINKER", str(self.roots["clang"] / "bin/x86_64-linux-gnu-ld.bfd")), \
                patch.object(CHECK, "GCC_COLLECT2", str(self.roots["clang"] / "libexec/gcc/x86_64-linux-gnu/13/collect2")), \
                patch.object(CHECK, "LTO_PLUGIN", str(self.roots["clang"] / "libexec/gcc/x86_64-linux-gnu/13/liblto_plugin.so")):
            return CHECK.Admission(self.policy_path, self.pointer, root=self.repository)

    def result(self, tool, *, defect=True):
        return {"exit_code": 86 if defect else 0, "failure": None,
                "output": CHECK.DIAGNOSTICS[tool] + "\n/work/inputs/" + Path(CHECK.SOURCES[tool]).name + ":17:5\n"
                if defect else "SANITIZER_HEALTHY\n"}

    def test_all_mechanisms_require_discriminating_fault_and_healthy_controls(self):
        for tool in CHECK.TOOLS:
            with self.subTest(tool=tool):
                CHECK.classify(tool, self.result(tool), defect=True)
                CHECK.classify(tool, self.result(tool, defect=False), defect=False)
                for changes in ({"exit_code": 0}, {"failure": "timeout"},
                                {"output": "unrelated process exit\n"},
                                {"output": CHECK.DIAGNOSTICS[tool] + "\nother.rs:17:5"},
                                {"output": CHECK.DIAGNOSTICS[tool] + "\nwrong-" + Path(CHECK.SOURCES[tool]).name + ":17:5"},
                                {"output": CHECK.DIAGNOSTICS[tool] + "\n" + Path(CHECK.SOURCES[tool]).name},
                                {"output": self.result(tool)["output"] + "\nFATAL: unexpected memory mapping"}):
                    bad = {**self.result(tool), **changes}
                    with self.assertRaises(CHECK.Incomplete):
                        CHECK.classify(tool, bad, defect=True)
                with self.assertRaises(CHECK.Incomplete):
                    CHECK.classify(tool, {**self.result(tool, defect=False), "output": "no marker"}, defect=False)

    def test_wrong_detector_and_panic_are_not_asan_evidence(self):
        for output in ("WARNING: ThreadSanitizer: data race\nasan.rs:17:5",
                       "thread 'main' panicked at asan.rs:17:5\n", "libFuzzer: deadly signal\nasan.rs:17:5"):
            with self.assertRaises(CHECK.Incomplete):
                CHECK.classify("asan", {**self.result("asan"), "output": output}, defect=True)

    def test_policy_is_fixed_and_rejects_unknown_tools_flags_paths_and_types(self):
        CHECK.validate_policy(self.policy)
        for field, value in (("schema_version", True), ("platform", "other"),
                             ("tools", {**CHECK.SOURCES, "other": "other.rs"}),
                             ("sysroots", {"tsan": "memory", "msan": "thread"}),
                             ("rust_compiler", "../rustc"), ("native_compiler", "/bin/clang")):
            changed = {**self.policy, field: value}
            with self.subTest(field=field), self.assertRaises(CHECK.Incomplete):
                CHECK.validate_policy(changed)
        with self.assertRaises(CHECK.Incomplete):
            CHECK.validate_policy({**self.policy, "flags": []})
        for value in ("../source.rs", "./source.rs", "a//b", "a/../b", "/source.rs", "a\\b", "", False):
            with self.subTest(path=value), self.assertRaises(CHECK.Incomplete):
                CHECK.relative(value)

    def test_sysroot_requires_unique_core_builtins_and_correct_runtime_for_each_tool(self):
        for tool, folder in CHECK.SYSROOTS.items():
            prefix = f"{folder}/lib/rustlib/{CHECK.PLATFORM}/lib/"
            for missing in ("libcore-123abc.rlib", "libcore-123abc.rmeta",
                            "libcompiler_builtins-123abc.rlib", "libcompiler_builtins-123abc.rmeta",
                            f"librustc-nightly_rt.{tool}.a"):
                policy = {**self.policy, "payloads": [entry for entry in self.policy["payloads"]
                                                       if entry["path"] != prefix + missing]}
                with self.subTest(tool=tool, missing=missing), self.assertRaises(CHECK.Incomplete):
                    CHECK.validate_policy(policy)
            policy = {**self.policy, "payloads": [*self.policy["payloads"],
                       {"root": "sysroots", "path": prefix + "libcore-456def.rlib", "sha256": "0" * 64}]}
            with self.assertRaises(CHECK.Incomplete):
                CHECK.validate_policy(policy)
            for library in ("core", "compiler_builtins"):
                policy = {**self.policy, "payloads": [
                    {**entry, "path": prefix + f"lib{library}-456def.rmeta"}
                    if entry["path"] == prefix + f"lib{library}-123abc.rmeta" else entry
                    for entry in self.policy["payloads"]]}
                with self.subTest(tool=tool, mismatched_metadata=library), self.assertRaises(CHECK.Incomplete):
                    CHECK.validate_policy(policy)

    def test_pointer_requires_separate_sysroots_and_gcc_helper_pins(self):
        pointer = json.loads(self.pointer.read_text())
        pointer["roots"].pop("sysroots")
        self.pointer.write_text(json.dumps(pointer))
        with self.assertRaises(CHECK.Incomplete):
            self.admission()
        self.pointer.write_text(json.dumps({"schema_version": 1, "roots": {key: str(value) for key, value in self.roots.items()}}))
        for missing in ("collect2", "liblto_plugin.so"):
            policy = {**self.policy, "payloads": [entry for entry in self.policy["payloads"] if Path(entry["path"]).name != missing]}
            self.policy_path.write_text(json.dumps(policy))
            with self.subTest(missing=missing), self.assertRaises(CHECK.Incomplete):
                self.admission()

    def test_strict_json_rejects_duplicates_and_nonfinite_values(self):
        for source in (b'{"a":1,"a":2}', b'{"a":NaN}', b'{"a":Infinity}'):
            with self.assertRaises(CHECK.Incomplete):
                CHECK.decode(source)

    def test_pointer_permissions_hardlinks_symlinks_and_schema_fail_closed(self):
        self.admission()
        self.pointer.chmod(0o644)
        with self.assertRaises(CHECK.Incomplete):
            self.admission()
        self.pointer.chmod(0o600)
        alias = self.base / "pointer-alias"
        os.link(self.pointer, alias)
        with self.assertRaises(CHECK.Incomplete):
            self.admission()
        alias.unlink()
        alias.symlink_to(self.pointer)
        with self.assertRaises(CHECK.Incomplete):
            CHECK.read_once(alias, private=True)
        self.pointer.write_text('{"schema_version":true,"roots":{}}')
        with self.assertRaises(CHECK.Incomplete):
            self.admission()

    def test_payload_and_fixture_hash_swaps_fail_before_execution(self):
        self.admission()
        payload = self.roots["rust"] / "bin/rustc"
        payload.chmod(0o700)
        payload.write_bytes(b"substituted")
        with patch.object(CHECK, "run_command") as run, self.assertRaises(CHECK.Incomplete):
            self.admission()
        run.assert_not_called()

    def test_changed_fixture_is_rejected_by_readmission(self):
        admitted = self.admission()
        (self.sources / CHECK.SOURCES["asan"]).write_text("changed fixture")
        with self.assertRaises(CHECK.Incomplete):
            admitted.check()

    def test_new_output_rejects_existing_repo_mnt_relative_and_symlink_paths(self):
        output = self.base / "fresh"
        CHECK.fresh_output(str(output), root=self.repository)
        self.assertEqual(output.stat().st_mode & 0o777, 0o700)
        for value in (str(output), str(self.repository / "new"), "/mnt/new", "relative"):
            with self.subTest(path=value), self.assertRaises((CHECK.Incomplete, OSError)):
                CHECK.fresh_output(value, root=self.repository)
        link = self.base / "link"
        link.symlink_to(self.base, target_is_directory=True)
        with self.assertRaises(CHECK.Incomplete):
            CHECK.fresh_output(str(link / "new"), root=self.repository)

    def test_compile_and_namespace_profiles_preserve_explicit_target_and_limits(self):
        admitted = self.admission()
        work = self.base / "work"
        work.mkdir()
        for tool in CHECK.TOOLS:
            command = CHECK.compile_command(admitted, tool, True)
            self.assertIn(CHECK.PLATFORM, " ".join(command))
            self.assertIn("SEEDED_DEFECT", " ".join(command).upper())
            self.assertFalse(any("unsafe-allow-abi-mismatch" in value for value in command))
            if tool in CHECK.SYSROOTS:
                self.assertEqual(command[command.index("--sysroot") + 1],
                                 str(admitted.roots["sysroots"] / CHECK.SYSROOTS[tool]))
            else:
                self.assertNotIn("--sysroot", command)
            if tool != "ubsan":
                for required in ("--edition=2024", "-Cpanic=abort", "dylib=gcc_s", "dylib=m"):
                    self.assertIn(required, command)
                self.assertIn("-Clinker=" + str(admitted.rust_linker), command)
                self.assertEqual([value for value in command if value.startswith("-Clink-arg=")][-1],
                                 "-Clink-arg=-fuse-ld=bfd")
            build, _ = CHECK.sandbox_command(admitted, work, command)
            self.assertIn("--as=8589934592:8589934592", build)
            runtime, environment = CHECK.sandbox_command(admitted, work, ["/work/bin/" + tool + "-healthy"], runtime=True,
                                                        environment=CHECK.control_environment(tool, admitted.roots["clang"] / "bin/llvm-symbolizer"))
            self.assertFalse(any(value.startswith("--as=") for value in runtime))
            for value in ("--unshare-all", "--die-with-parent", "--new-session", "--clearenv", "--core=0:0", "--cpu=20:20"):
                self.assertIn(value, runtime)
            self.assertEqual(environment["HOME"], "/work/home")
        with self.assertRaises(CHECK.Incomplete):
            CHECK.sandbox_command(admitted, work, ["anything"], environment={"PATH": "ambient"})

    def test_admission_only_cli_never_runs_controls_and_rejects_unknown_flags(self):
        admitted = self.admission()
        with patch.object(CHECK, "Admission", return_value=admitted), patch.object(admitted, "check"), \
                patch.object(CHECK.os, "geteuid", return_value=1000), patch.object(CHECK, "fixtures") as fixtures, \
                contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(CHECK.main([]), 0)
        fixtures.assert_not_called()
        for argv in (["--tool", "other"], ["--flags", "-O0"], ["--fixtures"], ["--output-dir", "/tmp/unused"]):
            with contextlib.redirect_stderr(io.StringIO()), self.assertRaises(SystemExit):
                CHECK.main(argv)

    def test_timeout_and_output_budget_kill_only_owned_group_and_preserve_failure(self):
        for cause in ("timeout", "output-limit"):
            with self.subTest(cause=cause), patch.object(CHECK.subprocess, "Popen") as popen, \
                    patch.object(CHECK.selectors, "DefaultSelector") as selectors, \
                    patch.object(CHECK.os, "killpg") as killpg, \
                    patch.object(CHECK.os, "read", return_value=b"a" * 64), \
                    patch.object(CHECK.time, "monotonic", side_effect=[0, 31 if cause == "timeout" else 0.1, 32]):
                process = popen.return_value
                process.pid = 987654
                process.wait.return_value = -9
                selector = selectors.return_value.__enter__.return_value
                selector.get_map.return_value = {1: 1}
                key = unittest.mock.Mock()
                selector.select.return_value = [(key, 1)]
                result = CHECK.run_command(["owned-sandbox"], timeout=30, output_limit=8)
                self.assertEqual(result["failure"], cause)
                self.assertEqual(result["exit_code"], -9)
                self.assertLessEqual(len(result["output"]), 8)
                killpg.assert_called_once_with(987654, CHECK.signal.SIGKILL)
                self.assertTrue(popen.call_args.kwargs["start_new_session"])


if __name__ == "__main__":
    unittest.main()
