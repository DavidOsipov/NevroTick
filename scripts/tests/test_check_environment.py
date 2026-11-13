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
import tomllib
import unittest
from unittest.mock import patch


SPEC = importlib.util.spec_from_file_location(
    "check_environment", Path(__file__).resolve().parents[1] / "check-environment.py"
)
ENV = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENV)


# Independent accepted policy: product constants do not supply this oracle.
EXPECTED_CARGO_CONFIGURATION = '''[build]
jobs=12
[env]
RUST_TEST_THREADS="12"
[registry]
global-min-publish-age="14 days"
[resolver]
incompatible-publish-age="deny"
[target.x86_64-pc-windows-msvc]
rustflags=["-C","target-cpu=x86-64-v3","-C","control-flow-guard=yes",
"-C","jump-tables=no","-C","link-arg=/GUARD:CF","-C","link-arg=/DYNAMICBASE",
"-C","link-arg=/HIGHENTROPYVA","-C","link-arg=/NXCOMPAT","-C","link-arg=/FIXED:NO",
"-C","link-arg=/LARGEADDRESSAWARE","-C","link-arg=/CETCOMPAT",
"-C","link-arg=/CETCOMPATSTRICT","-C","link-arg=/DEPENDENTLOADFLAG:0xA00",
"-C","link-arg=/WX"]
'''


class GuardTests(unittest.TestCase):
    def test_exact_fourteen_day_deny_configuration_is_accepted(self):
        ENV.require_cargo_configuration(tomllib.loads(EXPECTED_CARGO_CONFIGURATION))

    def test_cooldown_configuration_missing_weakened_or_extended_is_rejected(self):
        healthy = tomllib.loads(EXPECTED_CARGO_CONFIGURATION)
        variants = []
        for table in ("registry", "resolver"):
            changed = json.loads(json.dumps(healthy))
            del changed[table]
            variants.append(("missing " + table, changed))
        for age in ("0", "0 days", "13 days", "30 days", 14):
            changed = json.loads(json.dumps(healthy))
            changed["registry"]["global-min-publish-age"] = age
            variants.append(("age " + str(age), changed))
        for action in ("allow", "warn", True):
            changed = json.loads(json.dumps(healthy))
            changed["resolver"]["incompatible-publish-age"] = action
            variants.append(("resolver " + str(action), changed))
        changed = json.loads(json.dumps(healthy))
        changed["registries"] = {"crates-io": {"min-publish-age": "0 days"}}
        variants.append(("per-registry weakening", changed))
        changed = json.loads(json.dumps(healthy))
        changed["registries"] = {"private": {"min-publish-age": "14 days"}}
        variants.append(("additional same-age registry", changed))
        for name, candidate in variants:
            with self.subTest(case=name), patch.object(ENV.subprocess, "Popen") as launch:
                with self.assertRaisesRegex(ENV.Incomplete, "differs from the admitted"):
                    ENV.require_cargo_configuration(candidate)
                launch.assert_not_called()

    def test_age_overrides_are_rejected_before_execution_without_values(self):
        names = ("CARGO_REGISTRY_GLOBAL_MIN_PUBLISH_AGE",
                 "CARGO_REGISTRIES_CRATES_IO_MIN_PUBLISH_AGE",
                 "CARGO_REGISTRIES_PRIVATE_ARCHIVE_MIN_PUBLISH_AGE",
                 "CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE")
        for name in names:
            for value in ("", "14 days", "deny", "synthetic-private-value"):
                with (self.subTest(name=name, value=value),
                      patch.dict(os.environ, {name: value}, clear=True),
                      patch.object(ENV.subprocess, "Popen") as launch):
                    with self.assertRaisesRegex(ENV.Incomplete, name) as failure:
                        ENV.inspect(Path(tempfile.gettempdir()))
                    self.assertNotIn("synthetic-private-value", str(failure.exception))
                    launch.assert_not_called()
        values = {name: "synthetic-private-value" for name in names}
        self.assertEqual(ENV.overridden_names(values), sorted(names))

    def test_unrelated_registry_credentials_and_similar_names_remain_allowed(self):
        names = ("CARGO_REGISTRY_TOKEN", "CARGO_REGISTRIES_PRIVATE_TOKEN",
                 "CARGO_REGISTRIES_PRIVATE_INDEX", "CARGO_REGISTRIES_PRIVATE_CREDENTIAL_PROVIDER",
                 "CARGO_REGISTRIES_PRIVATE_MIN_PUBLISH_AGE_EXTRA",
                 "CARGO_REGISTRIES_MIN_PUBLISH_AGE")
        self.assertEqual(ENV.overridden_names(dict.fromkeys(names, "synthetic-secret")), [])

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
                EXPECTED_CARGO_CONFIGURATION.replace('"target-cpu=x86-64-v3"', '"--cap-lints=allow"')
            )
            with patch.dict(os.environ, {}, clear=True):
                with self.assertRaisesRegex(ENV.Incomplete, "differs from the admitted"):
                    ENV.inspect(root)

    def test_empty_test_selection_is_not_passing_evidence(self):
        with self.assertRaisesRegex(ENV.Incomplete, "not an empty selection"):
            ENV.require_test_count("test result: ok. 0 passed; 0 failed; 0 ignored;", 2, "unit")
        ENV.require_test_count("test result: ok. 2 passed; 0 failed; 0 ignored;", 2, "unit")

    def test_cpu_floor_and_mitigations_are_admitted_together(self):
        # This expected policy is independent of ENV.MSVC_FLAGS. A valid vector
        # must reach the next deliberate gate; invalid vectors must stop earlier.
        flags = tomllib.loads(EXPECTED_CARGO_CONFIGURATION)["target"]["x86_64-pc-windows-msvc"]["rustflags"]
        refused = "differs from the admitted"
        variants = [("healthy v3 profile", flags, "Missing reviewed policy/tool-manifest.json")]
        variants += [(cpu, [value.replace("target-cpu=x86-64-v3", "target-cpu=" + cpu)
                             for value in flags], "differs from the admitted")
                     for cpu in ("native", "x86-64", "x86-64-v2", "x86-64-v4")]
        for offset in range(0, len(flags), 2):
            variants.append(("missing " + flags[offset + 1],
                             flags[:offset] + flags[offset + 2:], refused))
        for name, before, after in (
            ("CFG metadata only", "control-flow-guard=yes", "control-flow-guard=nochecks"),
            ("jump tables enabled", "jump-tables=no", "jump-tables=yes"),
            ("CET disabled", "link-arg=/CETCOMPAT", "link-arg=/CETCOMPAT:NO"),
            ("CET strict disabled", "link-arg=/CETCOMPATSTRICT", "link-arg=/CETCOMPATSTRICT:NO"),
            ("CET relaxed", "link-arg=/CETCOMPATSTRICT", "link-arg=/CETIPVALIDATIONRELAXED"),
            ("CET dynamic in-process allowance", "link-arg=/CETCOMPATSTRICT",
             "link-arg=/CETDYNAMICAPISINPROC"),
            ("default DLL search", "link-arg=/DEPENDENTLOADFLAG:0xA00",
             "link-arg=/DEPENDENTLOADFLAG:0"),
            ("additional DLL search directories", "link-arg=/DEPENDENTLOADFLAG:0xA00",
             "link-arg=/DEPENDENTLOADFLAG:0x1000"),
        ):
            self.assertEqual(flags.count(before), 1, "Counterexample must change one policy input")
            variants.append((name, [after if value == before else value for value in flags], refused))
        for name, extra in (
            ("trailing CPU override", "target-cpu=native"),
            ("v3 feature disabled", "target-feature=-avx2"),
            ("higher ISA added", "target-feature=+avx512f"),
            ("trailing CFG disable", "control-flow-guard=no"),
            ("duplicate option", "jump-tables=no"),
        ):
            variants.append((name, flags + ["-C", extra], refused))
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
                    '[registry]\nglobal-min-publish-age="14 days"\n'
                    '[resolver]\nincompatible-publish-age="deny"\n'
                    '[target.x86_64-pc-windows-msvc]\nrustflags=' + json.dumps(candidate) + '\n')
                with (self.subTest(case=name), patch.dict(os.environ, {}, clear=True),
                      patch.object(ENV.subprocess, "Popen") as launch):
                    with self.assertRaisesRegex(ENV.Incomplete, diagnostic):
                        ENV.inspect(root)
                    launch.assert_not_called()


class CargoContextTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="cargo-context-control-")
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.root = self.base / "checkout"
        self.config = self.root / ".cargo/config.toml"
        self.config.parent.mkdir(parents=True)
        self.config.write_text(EXPECTED_CARGO_CONFIGURATION)
        self.home = self.base / "controlled-cargo-home"
        self.home.mkdir()
        self.environment = {"CARGO_HOME": str(self.home)}

    def test_structural_context_returns_validated_bytes_and_canonical_cwd_without_tools(self):
        expected = {"cwd": str(self.root), "config_sha256": ENV.hashlib.sha256(self.config.read_bytes()).hexdigest()}
        with (patch.object(ENV.subprocess, "Popen") as launch,
              patch.object(ENV.shutil, "which") as find,
              patch.object(ENV.os, "geteuid", return_value=0)):
            self.assertEqual(ENV.require_cargo_context(self.root / ".", self.environment), expected)
            launch.assert_not_called()
            find.assert_not_called()

    def test_explicit_cleared_environment_is_distinct_from_ambient_environment(self):
        with patch.dict(os.environ, {"CARGO_REGISTRY_GLOBAL_MIN_PUBLISH_AGE": "private-value"}, clear=True):
            self.assertEqual(ENV.require_cargo_context(self.root, {})["cwd"], str(self.root))
            with self.assertRaisesRegex(ENV.Incomplete, "CARGO_REGISTRY_GLOBAL_MIN_PUBLISH_AGE") as error:
                ENV.require_cargo_context(self.root)
            self.assertNotIn("private-value", str(error.exception))

    def test_ancestor_home_and_legacy_configuration_cannot_shadow_selected_policy(self):
        paths = (self.base / ".cargo/config", self.base / ".cargo/config.toml",
                 self.root / ".cargo/config", self.home / "config", self.home / "config.toml")
        for path in paths:
            with self.subTest(path=path):
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text('token="synthetic-private-value"\n')
                try:
                    with patch.object(ENV.subprocess, "Popen") as launch:
                        with self.assertRaisesRegex(ENV.Incomplete, "ancestor/home/legacy") as error:
                            ENV.require_cargo_context(self.root, self.environment)
                        self.assertNotIn("synthetic-private-value", str(error.exception))
                        launch.assert_not_called()
                finally:
                    path.unlink()
        self.assertEqual(ENV.require_cargo_context(self.root, self.environment)["cwd"], str(self.root))

    def test_bookends_detect_changed_valid_config_and_reject_weakened_policy(self):
        before = ENV.require_cargo_context(self.root, self.environment)
        self.config.write_text(EXPECTED_CARGO_CONFIGURATION + "\n# deliberate input change\n")
        after = ENV.require_cargo_context(self.root, self.environment)
        self.assertEqual(before["cwd"], after["cwd"])
        self.assertNotEqual(before["config_sha256"], after["config_sha256"])
        self.config.write_text(EXPECTED_CARGO_CONFIGURATION.replace('"14 days"', '"0 days"'))
        with self.assertRaisesRegex(ENV.Incomplete, "differs from the admitted"):
            ENV.require_cargo_context(self.root, self.environment)

    def test_supplied_cargo_home_must_be_absolute_canonical_without_tilde_expansion(self):
        alias = self.base / "redirected-parent"
        alias.symlink_to(self.home, target_is_directory=True)
        for home in ("", "relative", "~/.cargo", str(alias / "new-home"),
                     str(self.home) + "/.", str(self.home) + "/", str(self.home / ".." / "new-home")):
            with self.subTest(home=home), patch.object(ENV.subprocess, "Popen") as launch:
                with self.assertRaisesRegex(ENV.Incomplete, "CARGO_HOME.*absolute canonical") as error:
                    ENV.require_cargo_context(self.root, {"CARGO_HOME": home})
                if home:
                    self.assertNotIn(home, str(error.exception))
                launch.assert_not_called()
        not_created = self.base / "not-created-home"
        self.assertFalse(not_created.exists())
        self.assertEqual(ENV.require_cargo_context(self.root, {"CARGO_HOME": str(not_created)})["cwd"], str(self.root))

    def test_intended_home_and_account_fallback_are_checked_instead_of_ambient_home(self):
        intended_home = self.base / "intended-home"
        hidden = intended_home / ".cargo/config.toml"
        hidden.parent.mkdir(parents=True)
        hidden.write_text('token="synthetic-private-value"\n')
        with self.assertRaisesRegex(ENV.Incomplete, "ancestor/home/legacy"):
            ENV.require_cargo_context(self.root, {"HOME": str(intended_home)})
        with (patch.dict(os.environ, {"HOME": str(self.home)}, clear=True),
              patch("pwd.getpwuid", return_value=type("Account", (), {"pw_dir": str(intended_home)})())):
            with self.assertRaisesRegex(ENV.Incomplete, "ancestor/home/legacy"):
                ENV.require_cargo_context(self.root, {})

    def test_redirected_or_oversized_config_is_refused_before_tools(self):
        original = self.config.read_bytes()
        target = self.base / "other-config.toml"
        target.write_bytes(original)
        self.config.unlink()
        self.config.symlink_to(target)
        with self.assertRaisesRegex(ENV.Incomplete, "canonical|redirected"):
            ENV.require_cargo_context(self.root, self.environment)
        self.config.unlink()
        self.config.write_bytes(original + b"#" * (64 * 1024))
        with self.assertRaisesRegex(ENV.Incomplete, "bound"):
            ENV.require_cargo_context(self.root, self.environment)

    def test_config_change_during_read_is_not_reported_as_admitted(self):
        read = ENV.os.read
        changed = False
        def changing_read(descriptor, size):
            nonlocal changed
            raw = read(descriptor, size)
            if not changed:
                changed = True
                self.config.write_text(EXPECTED_CARGO_CONFIGURATION.replace('"14 days"', '"0 days"'))
            return raw
        with patch.object(ENV.os, "read", side_effect=changing_read):
            with self.assertRaisesRegex(ENV.Incomplete, "changed during"):
                ENV.require_cargo_context(self.root, self.environment)


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
