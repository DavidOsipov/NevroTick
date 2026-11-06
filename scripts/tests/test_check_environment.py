"""Independent controls for bootstrap refusal and subprocess outcome handling."""

import contextlib
import importlib.util
import io
import json
import os
from pathlib import Path
import signal
import sys
import tempfile
import time
import unittest
from unittest.mock import patch


SPEC = importlib.util.spec_from_file_location(
    "check_environment", Path(__file__).resolve().parents[1] / "check-environment.py"
)
ENV = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENV)


class GuardTests(unittest.TestCase):
    def test_analysis_overrides_cannot_weaken_selected_tool_policy(self):
        for name in ("NEXTEST_RETRIES", "CARGO_MUTANTS_CAP_LINTS", "CARGO_LLVM_COV_IGNORE_RUN_FAIL",
                     "LLVM_COV", "LLVM_PROFDATA", "CARGO_INCREMENTAL"):
            with self.subTest(variable=name), patch.dict(os.environ, {name: ""}, clear=True):
                with self.assertRaisesRegex(ENV.Incomplete, name):
                    ENV.inspect(Path(tempfile.gettempdir()))

    def test_override_names_never_include_values(self):
        values = {"PATH": "ok", "RUSTFLAGS": "synthetic-private-value",
                  "CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER": "hidden-linker"}
        names = ENV.overridden_names(values)
        self.assertEqual(names, ["CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER", "RUSTFLAGS"])
        self.assertNotIn("synthetic-private-value", str(names))

    def test_even_empty_override_is_refused(self):
        with patch.dict(os.environ, {"RUSTFLAGS": ""}, clear=True):
            with self.assertRaisesRegex(ENV.Incomplete, "RUSTFLAGS"):
                ENV.inspect(Path(tempfile.gettempdir()))

    def test_linter_and_loader_overrides_are_refused_without_disclosing_values(self):
        names = ["RUSTFMT", "CLIPPY_ARGS", "CLIPPY_CONF_DIR", "LD_PRELOAD", "LD_LIBRARY_PATH"]
        values = dict.fromkeys(names, "synthetic-private-value")
        self.assertEqual(ENV.overridden_names(values), sorted(names))
        self.assertNotIn("synthetic-private-value", str(ENV.overridden_names(values)))

    def test_no_sandbox_never_falls_back_to_host_cargo(self):
        with patch.object(ENV.shutil, "which", return_value=None):
            with self.assertRaisesRegex(ENV.Incomplete, "fallback is forbidden"):
                ENV.sandbox_command(Path("/toolchain"), Path("/work"), ["cargo", "test"])

    def test_cleared_sandbox_receives_independent_build_and_test_budgets(self):
        with patch.object(ENV.shutil, "which", side_effect=lambda name: "/usr/bin/" + name):
            command = ENV.sandbox_command(Path("/toolchain"), Path("/work"), ["cargo", "test"])
        settings = {command[i + 1]: command[i + 2] for i, value in enumerate(command) if value == "--setenv"}
        self.assertEqual(settings["CARGO_BUILD_JOBS"], "12")
        self.assertEqual(settings["RUST_TEST_THREADS"], "12")
        self.assertIn("--clearenv", command)
        self.assertIn("--nproc=128", command)

    def test_changed_target_flags_are_refused_before_execution(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "rust-toolchain.toml").write_text(
                '[toolchain]\nchannel="1.99.1"\nprofile="minimal"\n'
                'components=["clippy","rustfmt","rust-analyzer","rust-src","llvm-tools-preview"]\n'
                'targets=["x86_64-pc-windows-msvc"]\n'
            )
            (root / ".cargo").mkdir()
            (root / ".cargo/config.toml").write_text(
                '[target.x86_64-pc-windows-msvc]\nrustflags=["--cap-lints=allow"]\n'
            )
            with patch.dict(os.environ, {}, clear=True):
                with self.assertRaisesRegex(ENV.Incomplete, "differs from the admitted"):
                    ENV.inspect(root)

    def test_empty_test_selection_is_not_passing_evidence(self):
        with self.assertRaisesRegex(ENV.Incomplete, "not an empty selection"):
            ENV.require_test_count("test result: ok. 0 passed; 0 failed; 0 ignored;", 2, "unit")
        ENV.require_test_count("test result: ok. 2 passed; 0 failed; 0 ignored;", 2, "unit")

    def test_cpu_floor_and_mitigations_are_admitted_together(self):
        flags = ["-C", "target-cpu=x86-64", "-C", "control-flow-guard=yes",
                 "-C", "link-arg=/GUARD:CF", "-C", "link-arg=/DYNAMICBASE",
                 "-C", "link-arg=/HIGHENTROPYVA", "-C", "link-arg=/NXCOMPAT",
                 "-C", "link-arg=/WX"]
        variants = [("healthy", flags, "Missing reviewed policy/tool-manifest.json")]
        variants += [(cpu, [value.replace("target-cpu=x86-64", "target-cpu=" + cpu)
                             for value in flags], "differs from the admitted")
                     for cpu in ("native", "x86-64-v2", "x86-64-v3")]
        variants.append(("missing CPU", flags[2:], "differs from the admitted"))
        for offset in range(2, len(flags), 2):
            variants.append(("missing " + flags[offset + 1],
                             flags[:offset] + flags[offset + 2:], "differs from the admitted"))
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "rust-toolchain.toml").write_text(
                '[toolchain]\nchannel="1.99.1"\nprofile="minimal"\n'
                'components=["clippy","rustfmt","rust-analyzer","rust-src","llvm-tools-preview"]\n'
                'targets=["x86_64-pc-windows-msvc"]\n')
            (root / ".cargo").mkdir()
            for name, candidate, diagnostic in variants:
                (root / ".cargo/config.toml").write_text(
                    '[build]\njobs=12\n[env]\nRUST_TEST_THREADS="12"\n'
                    '[target.x86_64-pc-windows-msvc]\nrustflags=' + json.dumps(candidate) + '\n')
                with self.subTest(case=name), patch.dict(os.environ, {}, clear=True):
                    with self.assertRaisesRegex(ENV.Incomplete, diagnostic):
                        ENV.inspect(root)


class ProcessTests(unittest.TestCase):
    def test_invalid_output_budget_rejects_before_launch(self):
        for budget in (0, -1, True, None, "8", ENV.MAX_REQUESTED_OUTPUT + 1):
            with self.subTest(budget=budget), patch.object(ENV.subprocess, "Popen") as launch:
                with self.assertRaisesRegex(ENV.Incomplete, "output budget"):
                    ENV.run([sys.executable, "-c", "print('unreached')"],
                            cwd=Path.cwd(), max_output=budget)
                launch.assert_not_called()

    def test_explicit_output_budget_admits_exact_bound_and_rejects_next_byte(self):
        for size, failure in ((8, None), (9, "output budget exceeded")):
            with self.subTest(size=size):
                result = ENV.run([sys.executable, "-c", f"print('x'*{size}, end='')"],
                                 cwd=Path.cwd(), max_output=8)
                self.assertEqual(result["failure"], failure)
                if failure is None:
                    self.assertEqual(result["exit_code"], 0)
                    self.assertEqual(result["output"], "x" * 8)
        healthy = ENV.run([sys.executable, "-c", "print('ok')"], cwd=Path.cwd())
        self.assertEqual(healthy["exit_code"], 0)
        self.assertIsNone(healthy["failure"])
        self.assertEqual(healthy["output"], "ok\n")

    def test_larger_metadata_budget_is_per_call_and_default_remains_two_mib(self):
        self.assertEqual(ENV.MAX_OUTPUT, 2 * 1024 * 1024)
        size = ENV.MAX_OUTPUT + 1
        command = [sys.executable, "-c", f"print('x'*{size}, end='')"]
        admitted = ENV.run(command, cwd=Path.cwd(), max_output=3 * 1024 * 1024)
        self.assertEqual(admitted["exit_code"], 0)
        self.assertIsNone(admitted["failure"])
        self.assertEqual(len(admitted["output"]), size)
        rejected = ENV.run(command, cwd=Path.cwd())
        self.assertEqual(rejected["failure"], "output budget exceeded")
        self.assertLessEqual(len(rejected["output"]), ENV.MAX_OUTPUT)

    def test_failed_predecessors_preserve_same_parent_runner_and_pipe_cleanup(self):
        """Independent commands share a runner parent, not scenario state."""
        children = []
        launch = ENV.subprocess.Popen

        def observe_launch(*args, **kwargs):
            """Observe real owned children without performing runner cleanup."""
            child = launch(*args, **kwargs)
            children.append(child)
            return child

        def cleanup_owned_children():
            """Restore only this test's children after pre-teardown assertions."""
            for child in children:
                if child.poll() is None:
                    child.kill()
                    child.wait(timeout=5)
                child.stdout.close()

        self.addCleanup(cleanup_owned_children)
        expected = {"exit_code": 0, "failure": None, "output": "healthy\n"}

        def healthy_victim():
            """Check independent output and reached cleanup before fallback."""
            result = ENV.run([sys.executable, "-c", "print('healthy')"], cwd=Path.cwd())
            observed = {key: result[key] for key in expected}
            self.assertEqual(observed, expected)
            for child in children:
                self.assertIsNotNone(child.returncode, "runner did not reap an owned child")
                self.assertTrue(child.stdout.closed, "runner left an owned output pipe open")
            return observed

        predecessors = {
            "nonzero": ("raise SystemExit(7)", 30, None),
            "output": (f"import os; os.write(1, b'x'*{ENV.MAX_OUTPUT + 1})",
                       30, "output budget exceeded"),
            "timeout": ("import time; time.sleep(10)", 0.1, "wall timeout"),
        }
        with patch.object(ENV.subprocess, "Popen", side_effect=observe_launch):
            baseline = healthy_victim()
            for sequence in (("nonzero",), ("output", "nonzero"), ("timeout",)):
                with self.subTest(predecessors=sequence):
                    for name in sequence:
                        code, timeout, failure = predecessors[name]
                        result = ENV.run([sys.executable, "-c", code],
                                         cwd=Path.cwd(), timeout=timeout)
                        if failure is None:
                            self.assertEqual(result["exit_code"], 7)
                            self.assertIsNone(result["failure"])
                        else:
                            self.assertIn(failure, result["failure"] or "")
                        with self.assertRaises(ENV.Incomplete):
                            ENV.require_ok(result)
                    self.assertEqual(healthy_victim(), baseline)

    def test_completed_command_without_descendants_is_a_healthy_control(self):
        result = ENV.run([sys.executable, "-c", "print('healthy')"], cwd=Path.cwd())
        self.assertEqual(result["exit_code"], 0)
        self.assertIsNone(result["failure"])
        self.assertEqual(ENV.require_ok(result), "healthy")

    def test_child_nonzero_exit_is_preserved(self):
        result = ENV.run([sys.executable, "-c", "raise SystemExit(7)"], cwd=Path.cwd())
        self.assertEqual(result["exit_code"], 7)
        with self.assertRaises(ENV.Incomplete):
            ENV.require_ok(result)

    def test_wall_timeout_is_not_success(self):
        result = ENV.run([sys.executable, "-c", "import time; time.sleep(10)"],
                         cwd=Path.cwd(), timeout=0.1)
        self.assertIn("wall timeout", result["failure"])
        self.assertNotEqual(result["exit_code"], 0)

    def test_output_budget_is_not_success(self):
        result = ENV.run([sys.executable, "-c", "import os; os.write(1,b'x'*2097153)"],
                         cwd=Path.cwd())
        self.assertEqual(result["failure"], "output budget exceeded")
        self.assertLessEqual(len(result["output"]), ENV.MAX_OUTPUT)

    def test_closed_output_does_not_extend_the_command_wall_budget(self):
        started = time.monotonic()
        result = ENV.run(
            [sys.executable, "-c", "import os,time; os.close(1); os.close(2); time.sleep(20)"],
            cwd=Path.cwd(), timeout=0.1,
        )
        self.assertIn("wall timeout", result["failure"])
        self.assertLess(time.monotonic() - started, 1.5)

    def test_descendant_cannot_outlive_a_successful_leader(self):
        with tempfile.TemporaryDirectory() as directory:
            pid_path = Path(directory) / "descendant.pid"
            code = (
                "import pathlib,subprocess,sys; "
                "child=subprocess.Popen([sys.executable,'-c','import time; time.sleep(20)'], "
                "stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL); "
                "pathlib.Path(sys.argv[1]).write_text(str(child.pid))"
            )
            descendant = None
            try:
                result = ENV.run([sys.executable, "-c", code, str(pid_path)], cwd=Path.cwd())
                descendant = int(pid_path.read_text())
                self.assertEqual(result["exit_code"], 0)
                self.assertIn("descendant", result["failure"] or "")
                with self.assertRaises(ENV.Incomplete):
                    ENV.require_ok(result)
                deadline = time.monotonic() + 1
                while time.monotonic() < deadline:
                    status = Path(f"/proc/{descendant}/stat")
                    if not status.exists() or status.read_text().split()[2] in {"Z", "X"}:
                        break
                    time.sleep(0.01)
                else:
                    self.fail("Owned descendant remained running after the command returned")
            finally:
                if descendant is not None:
                    try:
                        os.kill(descendant, signal.SIGKILL)
                    except ProcessLookupError:
                        pass


class ReceiptTests(unittest.TestCase):
    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory(prefix="nevrotick-receipt-control-")
        self.addCleanup(self.scratch.cleanup)
        self.directory = Path(self.scratch.name)
        self.root = self.directory / "checkout"
        (self.root / "scripts").mkdir(parents=True)
        self.sentinel = self.root / "policy.toml"
        self.sentinel.write_text("protected sentinel\n")

    def invoke(self, receipt, inspect_effect=None):
        with patch.object(ENV, "__file__", str(self.root / "scripts/check-environment.py")), \
             patch.object(ENV, "inspect", return_value={"scope": "synthetic metadata"},
                          side_effect=inspect_effect) as inspect, \
             contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
            result = ENV.main(["--json", str(receipt)])
        return result, inspect

    def test_checkout_receipt_is_rejected_before_checks_without_overwriting_input(self):
        status, inspect = self.invoke(self.sentinel)
        self.assertEqual(status, 1)
        inspect.assert_not_called()
        self.assertEqual(self.sentinel.read_text(), "protected sentinel\n")

    def test_external_symlink_to_checkout_is_rejected_without_overwriting_input(self):
        receipt = self.directory / "external.json"
        receipt.symlink_to(self.sentinel)
        status, inspect = self.invoke(receipt)
        self.assertEqual(status, 1)
        inspect.assert_not_called()
        self.assertEqual(self.sentinel.read_text(), "protected sentinel\n")

    def test_external_hardlink_to_input_is_rejected_without_overwriting_input(self):
        receipt = self.directory / "external.json"
        receipt.hardlink_to(self.sentinel)
        status, inspect = self.invoke(receipt)
        self.assertEqual(status, 1)
        inspect.assert_not_called()
        self.assertEqual(self.sentinel.read_text(), "protected sentinel\n")

    def test_receipt_redirected_during_checks_is_revalidated_before_write(self):
        receipt = self.directory / "external.json"

        def redirect(_root):
            receipt.symlink_to(self.sentinel)
            return {"scope": "synthetic metadata"}

        status, inspect = self.invoke(receipt, redirect)
        self.assertEqual(status, 1)
        inspect.assert_called_once_with(self.root)
        self.assertEqual(self.sentinel.read_text(), "protected sentinel\n")

    def test_valid_external_receipt_has_a_neighboring_success_control(self):
        receipt = self.directory / "external.json"
        status, inspect = self.invoke(receipt)
        self.assertEqual(status, 0)
        inspect.assert_called_once_with(self.root)
        self.assertEqual(json.loads(receipt.read_text())["status"], "passed")


if __name__ == "__main__":
    unittest.main()
