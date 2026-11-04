"""Admission and failure-propagation controls; real mypy fixtures are a separate lane."""
import importlib.util
import ast
import contextlib
import io
import json
import os
from pathlib import Path
import shutil
import sys
import tempfile
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("check_python_typing", ROOT / "scripts/check-python-typing.py")
TYPING = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(TYPING)


def result(output="Success: no issues found in 8 source files\n", exit_code=0, failure=None):
    return {"command": ["synthetic-checker"], "exit_code": exit_code, "failure": failure, "output": output}


class AdmissionTests(unittest.TestCase):
    def test_seven_selected_helpers_have_exact_eighteen_input_source_closure(self):
        expected = {
            "scripts/core-combination-accounting.py": "core_combination_accounting.py",
            "scripts/core-analysis-cohorts.py": "core_analysis_cohorts.py",
            "scripts/core-chart-accounting.py": "core_chart_accounting.py",
            "scripts/core-chart-contracts.py": "core_chart_contracts.py",
            "scripts/core-chart-combination-accounting.py": "core_chart_combination_accounting.py",
            "scripts/core-chart-combination-contracts.py": "core_chart_combination_contracts.py",
            "scripts/core-rate-delivery-accounting.py": "core_rate_delivery_accounting.py",
        }
        self.assertEqual(TYPING.HELPERS, expected)
        self.assertEqual(len(TYPING.SOURCE_PATHS), 18)
        self.assertEqual(len(set(TYPING.SOURCE_PATHS)), 18)
        sources, hashes = TYPING.source_snapshot(ROOT)
        self.assertEqual(set(sources), set(TYPING.SOURCE_PATHS))
        self.assertEqual(set(hashes), set(TYPING.SOURCE_PATHS))

    def test_bounded_read_allocates_from_actual_file_size_not_the_large_allowance(self):
        requested = []
        original_fdopen = os.fdopen

        class RecordingStream:
            def __init__(self, descriptor, mode):
                self.stream = original_fdopen(descriptor, mode)

            def __enter__(self):
                self.stream.__enter__()
                return self

            def __exit__(self, *args):
                return self.stream.__exit__(*args)

            def fileno(self):
                return self.stream.fileno()

            def read(self, size):
                requested.append(size)
                return self.stream.read(size)

        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "tiny"
            path.write_bytes(b"thirteen-byte")
            with patch.object(TYPING.os, "fdopen", side_effect=RecordingStream):
                self.assertEqual(TYPING.read_regular(path, TYPING.MAX_TOOL_BYTES), b"thirteen-byte")
        self.assertEqual(requested, [14])

    def test_size_changes_during_read_are_rejected_with_unchanged_file_healthy(self):
        original_fdopen = os.fdopen
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "changing"
            for replacement in (b"grown beyond original", b"short", b"same-size-new"):
                path.write_bytes(b"thirteen-byte")

                class ChangingStream:
                    def __init__(self, descriptor, mode):
                        self.stream = original_fdopen(descriptor, mode)

                    def __enter__(self):
                        self.stream.__enter__()
                        return self

                    def __exit__(self, *args):
                        return self.stream.__exit__(*args)

                    def fileno(self):
                        return self.stream.fileno()

                    def read(self, size):
                        path.write_bytes(replacement)
                        return self.stream.read(size)

                with self.subTest(replacement=replacement), patch.object(TYPING.os, "fdopen", side_effect=ChangingStream):
                    with self.assertRaisesRegex(TYPING.Incomplete, "changed during"):
                        TYPING.read_regular(path, TYPING.MAX_TOOL_BYTES)
                path.write_bytes(b"thirteen-byte")
                self.assertEqual(TYPING.read_regular(path, TYPING.MAX_TOOL_BYTES), b"thirteen-byte")

    def test_complete_inventory_rejects_added_removed_changed_and_linked_files(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "first").write_text("healthy")
            admitted = TYPING.inventory_tree(root)
            (root / "extra").write_text("extra")
            self.assertNotEqual(TYPING.inventory_tree(root), admitted)
            (root / "extra").unlink()
            (root / "first").write_text("changed")
            self.assertNotEqual(TYPING.inventory_tree(root), admitted)
            (root / "first").unlink()
            with self.assertRaisesRegex(TYPING.Incomplete, "empty"):
                TYPING.inventory_tree(root)
            (root / "alias").symlink_to(ROOT)
            with self.assertRaisesRegex(TYPING.Incomplete, "link or special"):
                TYPING.inventory_tree(root)

    def test_final_file_symlink_and_hardlink_are_not_custodied_bytes(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            first = directory / "first"
            first.write_text("value")
            alias = directory / "alias"
            alias.symlink_to(first)
            with self.assertRaises(TYPING.Incomplete):
                TYPING.read_regular(alias, 32)
            alias.unlink()
            alias.hardlink_to(first)
            with self.assertRaisesRegex(TYPING.Incomplete, "unique custody"):
                TYPING.read_regular(first, 32)

    def test_duplicate_json_keys_are_rejected(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "receipt.json"
            path.write_text('{"schema_version": 1, "schema_version": 1}')
            with self.assertRaisesRegex(TYPING.Incomplete, "Duplicate JSON key"):
                TYPING.read_json(path)

    def test_exact_source_closure_rejects_missing_extra_and_noncanonical_paths(self):
        valid = {"owner.py": "a" * 64}
        self.assertEqual(TYPING.require_hash_map(valid, expected={"owner.py"}), valid)
        for invalid in ({}, {"other.py": "a" * 64}, {"../owner.py": "a" * 64},
                        {"./owner.py": "a" * 64}, {"/owner.py": "a" * 64},
                        {"owner.py": "pending"}):
            with self.subTest(invalid=invalid), self.assertRaises(TYPING.Incomplete):
                TYPING.require_hash_map(invalid, expected={"owner.py"})

    def test_policy_refuses_plugin_missing_import_and_strictness_overrides(self):
        original = json.loads((ROOT / TYPING.POLICY_PATH).read_text())
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            (root / "policy").mkdir()
            for key, value in (("plugins", "arbitrary_plugin"), ("ignore_missing_imports", True),
                               ("strict", False), ("follow_imports", "skip")):
                policy = json.loads(json.dumps(original))
                policy["mypy_options"][key] = value
                (root / TYPING.POLICY_PATH).write_text(json.dumps(policy))
                with self.subTest(key=key), self.assertRaisesRegex(TYPING.Incomplete, "closed strict"):
                    TYPING.inspect_policy(root)

    def test_checked_source_refuses_any_cast_ignore_and_import_expansion(self):
        sources, _ = TYPING.source_snapshot(ROOT)
        for owner in TYPING.HELPERS:
            for extra in ("\nfrom typing import Any\n", "\nfrom typing import cast\n",
                          "\nvalue = 1  # type: ignore\n", "\n# mypy: ignore-errors\n",
                          "\nimport arbitrary_module\n", "\nimport typing\nvalue = typing.cast(int, 1)\n"):
                changed = {**sources, owner: sources[owner] + extra.encode()}
                with self.subTest(owner=owner, extra=extra), self.assertRaises(TYPING.Incomplete):
                    TYPING.inspect_sources(changed)
        self.assertIn("core_combination_accounting", TYPING.inspect_sources(sources))

    def test_normalized_copy_is_byte_identical_and_only_fault_copy_changes(self):
        sources, _ = TYPING.source_snapshot(ROOT)
        before = dict(sources)
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            healthy = root / "healthy"
            faulty = root / "faulty"
            TYPING.prepare_source(healthy, sources, None)
            TYPING.prepare_source(faulty, sources, TYPING.CASES[-1])
            for original, normalized in TYPING.HELPERS.items():
                self.assertEqual((healthy / normalized).read_bytes(), sources[original])
            self.assertIn(b"def full_package_reservation(required: str)",
                          (faulty / "core_combination_accounting.py").read_bytes())
        self.assertEqual(sources, before)

    def test_new_generated_consumer_binds_actual_modules_without_changing_old_fixture(self):
        sources, _ = TYPING.source_snapshot(ROOT)
        original = sources[f"{TYPING.FIXTURE_DIR}/typed_consumer.py"]
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary) / "healthy"
            TYPING.prepare_source(directory, sources, None)
            actual = (directory / "typed_consumer.py").read_bytes()
        self.assertEqual(actual, original + TYPING.CHART_COMBINATION_CONSUMER + TYPING.RATE_DELIVERY_CONSUMER)
        self.assertEqual(sources[f"{TYPING.FIXTURE_DIR}/typed_consumer.py"], original)
        tree = ast.parse(actual.decode())
        bindings = {node.target.id: ast.unparse(node.annotation) for node in tree.body
                    if isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name)}
        self.assertEqual(bindings["chart_combination_accounting"], "ChartCombinationAccountingInterface")
        self.assertEqual(bindings["chart_combination_legal"], "ChartCombinationContractsInterface")
        self.assertEqual(bindings["rate_delivery_accounting"], "RateDeliveryAccountingInterface")
        classes = {node.name: node for node in tree.body if isinstance(node, ast.ClassDef)}
        self.assertIn("ChartCombinationAccountingInterface", classes)
        self.assertIn("ChartCombinationContractsInterface", classes)
        rate_interface = classes["RateDeliveryAccountingInterface"]
        self.assertEqual({node.name for node in rate_interface.body if isinstance(node, ast.FunctionDef)},
                         {"case_recipes", "validate_case_recipes", "package_overlay", "validate_overlay",
                          "admit_sources", "profile_observation"})
        actual_names = TYPING.inspect_sources(sources)
        self.assertIn("core_chart_combination_accounting", actual_names)
        self.assertIn("core_chart_combination_contracts", actual_names)
        self.assertIn("core_rate_delivery_accounting", actual_names)

    def test_old_nine_and_six_chart_controls_are_preserved_with_four_rate_controls(self):
        old = (
            ("wrong_field", "wrong_field.py", "typeddict-item", "unknown_counter"),
            ("wrong_value", "wrong_value.py", "typeddict-item", "terms"),
            ("reservation_as_observation", "reservation_as_observation.py", "return-value", "ReservedAttempts"),
            ("partial_as_complete", "partial_as_complete.py", "arg-type", "PartialObservation"),
            ("lifecycle_reservation_as_observation", "lifecycle_reservation_as_observation.py", "return-value", "ReservedLifecycleAttempts"),
            ("lifecycle_partial_as_complete", "lifecycle_partial_as_complete.py", "arg-type", "LifecyclePartialObservation"),
            ("unhandled_status", "unhandled_status.py", "arg-type", "InterruptedObservation"),
            ("unhandled_match", "unhandled_match.py", "exhaustive-match", "interrupted"),
            ("incompatible_implementation", "typed_consumer.py", "assignment", "CombinationInterface"))
        names = {case[0] for case in old}
        self.assertEqual(tuple(case for case in TYPING.CASES if case[0] in names), old)
        self.assertEqual(len(TYPING.CASES), 19)
        self.assertEqual(len({case[0] for case in TYPING.CASES}), 19)
        self.assertEqual(len(TYPING.GENERATED_FIXTURES), 7)
        self.assertEqual(len(TYPING.IMPLEMENTATION_MUTATIONS), 4)
        rate_cases = (
            ("rate_delivery_wrong_field", "rate_delivery_wrong_field.py", "typeddict-item", "rate_source_only_pending"),
            ("rate_delivery_reservation_as_outcome", "rate_delivery_reservation_as_outcome.py", "return-value", "Observation"),
            ("rate_delivery_unhandled_status", "rate_delivery_unhandled_status.py", "arg-type", "incomplete"),
            ("incompatible_rate_delivery_accounting", "typed_consumer.py", "assignment", "RateDeliveryAccountingInterface"),
        )
        self.assertEqual(tuple(case for case in TYPING.CASES if "rate_delivery" in case[0]), rate_cases)

    def test_each_new_fixture_has_its_exact_frozen_copy_and_only_one_changed_owner(self):
        sources, _ = TYPING.source_snapshot(ROOT)
        before = dict(sources)
        selected = [case for case in TYPING.CASES
                    if "chart_combination" in case[0] or "rate_delivery" in case[0]]
        with tempfile.TemporaryDirectory() as temporary:
            for case in selected:
                directory = Path(temporary) / case[0]
                TYPING.prepare_source(directory, sources, case)
                if case[1] in TYPING.GENERATED_FIXTURES:
                    self.assertEqual((directory / case[1]).read_bytes(), TYPING.GENERATED_FIXTURES[case[1]])
                for original, normalized in TYPING.HELPERS.items():
                    expected = sources[original]
                    if case[0] in TYPING.IMPLEMENTATION_MUTATIONS:
                        owner, start, replacement = TYPING.IMPLEMENTATION_MUTATIONS[case[0]]
                        if normalized == owner:
                            self.assertEqual(expected.count(start), 1)
                            expected = expected.replace(start, replacement)
                    self.assertEqual((directory / normalized).read_bytes(), expected)
        self.assertEqual(sources, before)

    def test_signature_mutations_reject_missing_and_duplicate_exact_owners(self):
        sources, _ = TYPING.source_snapshot(ROOT)
        with tempfile.TemporaryDirectory() as temporary:
            for name, (normalized, start, _) in TYPING.IMPLEMENTATION_MUTATIONS.items():
                original = next(path for path, copy in TYPING.HELPERS.items() if copy == normalized)
                case = next(case for case in TYPING.CASES if case[0] == name)
                for mode in ("missing", "duplicate"):
                    raw = sources[original]
                    altered = raw.replace(start, b"def changed_signature(") if mode == "missing" else raw + b"\n" + start
                    with self.subTest(name=name, mode=mode), self.assertRaisesRegex(TYPING.Incomplete, "one exact owner"):
                        TYPING.prepare_source(Path(temporary) / (name + mode), {**sources, original: altered}, case)

    def test_generated_controls_receive_the_same_escape_and_import_admission(self):
        sources, _ = TYPING.source_snapshot(ROOT)
        for extra in (b"\nfrom typing import Any\n", b"\nvalue = 1 # type: ignore\n", b"\nimport arbitrary_module\n"):
            with patch.object(TYPING, "CHART_COMBINATION_CONSUMER", TYPING.CHART_COMBINATION_CONSUMER + extra):
                with self.assertRaises(TYPING.Incomplete):
                    TYPING.inspect_sources(sources)
            with patch.object(TYPING, "RATE_DELIVERY_CONSUMER", TYPING.RATE_DELIVERY_CONSUMER + extra):
                with self.assertRaises(TYPING.Incomplete):
                    TYPING.inspect_sources(sources)
            name, raw = next(iter(TYPING.GENERATED_FIXTURES.items()))
            with patch.dict(TYPING.GENERATED_FIXTURES, {name: raw + extra}):
                with self.assertRaises(TYPING.Incomplete):
                    TYPING.inspect_sources(sources)


class IsolationTests(unittest.TestCase):
    def test_missing_sandbox_is_incomplete_without_host_fallback(self):
        with patch.object(TYPING.shutil, "which", return_value=None):
            with self.assertRaisesRegex(TYPING.Incomplete, "fallback is forbidden"):
                TYPING.sandbox_command(Path("/checker"), ROOT, Path("/work"), Path("/input"), ["checker"])

    def test_sandbox_preserves_readonly_source_environment_network_and_resource_limits(self):
        with patch.object(TYPING.shutil, "which", side_effect=lambda name, **_: "/usr/bin/" + name):
            command = TYPING.sandbox_command(Path("/tool"), ROOT, Path("/owned/work"),
                                            Path("/owned/input"), ["/checker/bin/python", "-I", "-B", "-S"])
        self.assertIn("--unshare-all", command)
        self.assertIn("--clearenv", command)
        self.assertIn("--cap-drop", command)
        self.assertIn("--as=4294967296", command)
        self.assertIn("--cpu=90", command)
        self.assertIn("--nproc=128", command)
        self.assertIn("--nofile=256", command)
        readonly = [(command[index + 1], command[index + 2])
                    for index, item in enumerate(command) if item == "--ro-bind"]
        self.assertIn(("/tool", "/checker"), readonly)
        self.assertIn((str(ROOT), "/src"), readonly)
        self.assertIn(("/owned/input", "/input"), readonly)
        writable = [(command[index + 1], command[index + 2])
                    for index, item in enumerate(command) if item == "--bind"]
        self.assertEqual(writable, [("/owned/work", "/work")])
        settings = {command[index + 1] for index, item in enumerate(command) if item == "--setenv"}
        self.assertEqual(settings, {"PATH", "LANG", "TMPDIR"})

    def test_checker_bootstrap_disables_site_hooks_and_checker_config_is_fixed(self):
        config = TYPING.config_bytes().decode()
        self.assertIn("strict = true\n", config)
        self.assertIn("no_site_packages = true\n", config)
        self.assertIn("enable_error_code = exhaustive-match\n", config)
        self.assertIn("disallow_any_expr = true\n", config)
        self.assertNotIn("plugins", config)
        self.assertNotIn("ignore_missing_imports", config)
        self.assertIn("/checker/lib/python3.12/site-packages", TYPING.CHECKER_ENTRY)
        self.assertNotIn("site.addsitedir", TYPING.CHECKER_ENTRY)


class DiagnosticTests(unittest.TestCase):
    def test_cli_accepts_both_declared_success_modes_and_preserves_incomplete_failure(self):
        command = ["check-python-typing.py", "--checker-python", "/external/checker/bin/python",
                   "--admission", "/external/admission.json"]
        for status, expected in (("passed-static-typing", 0), ("passed-static-fixture-typing", 0),
                                 ("incomplete", 1)):
            with self.subTest(status=status), patch.object(sys, "argv", command), \
                    patch.object(TYPING, "execute", return_value={"status": status}), \
                    contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(TYPING.main(), expected)

    def test_healthy_control_requires_real_nonempty_selection(self):
        TYPING.require_result(result())
        for candidate in (result("Success: no issues found in 0 source files\n"),
                          result("Success: no issues found in 3 source files\n"),
                          result("Success: no issues found in 4 source files\n"),
                          result("Success: no issues found in 5 source files\n"),
                          result("Success: no issues found in 6 source files\n"),
                          result("Success: no issues found in 7 source files\n"),
                          result("Success: no issues found in 9 source files\n"),
                          result(exit_code=7), result(failure="wall timeout")):
            with self.subTest(candidate=candidate), self.assertRaises(TYPING.Incomplete):
                TYPING.require_result(candidate)

    def test_intended_negative_diagnostic_is_accepted(self):
        case = TYPING.CASES[1]
        TYPING.require_result(result('/input/wrong_value.py:5:5: error: Value of "terms" is invalid  [typeddict-item]\n', 1), case)

    def test_setup_import_success_wrong_error_and_wrong_owner_cannot_count_as_rejection(self):
        case = TYPING.CASES[1]
        outputs = [
            '/input/wrong_value.py:1:1: error: Cannot find implementation "terms"  [import-not-found]\n',
            '/input/wrong_value.py:5:5: error: "terms" has wrong type  [arg-type]\n',
            '/input/other.py:5:5: error: "terms" has wrong type  [typeddict-item]\n',
            '/input/wrong_value.py:5:5: error: "terms" has wrong type  [typeddict-item]\n'
            '/input/core_analysis_cohorts.py:1:1: error: unrelated  [misc]\n',
        ]
        for output in outputs:
            with self.subTest(output=output), self.assertRaises(TYPING.Incomplete):
                TYPING.require_result(result(output, 1), case)
        with self.assertRaises(TYPING.Incomplete):
            TYPING.require_result(result(), case)

    def test_every_new_control_requires_its_intended_owner_type_and_diagnostic(self):
        for case in TYPING.CASES:
            if "chart_combination" not in case[0] and "rate_delivery" not in case[0]:
                continue
            _, filename, code, fragment = case
            intended = f'/input/{filename}:5:1: error: intended {fragment} diagnostic  [{code}]\n'
            TYPING.require_result(result(intended, 1), case)
            for output in (intended.replace(f"[{code}]", "[import-not-found]"),
                           intended.replace(fragment, "unrelated"),
                           intended.replace(filename, "other.py")):
                with self.subTest(case=case[0], output=output), self.assertRaises(TYPING.Incomplete):
                    TYPING.require_result(result(output, 1), case)
            with self.assertRaises(TYPING.Incomplete):
                TYPING.require_result(result(intended, 0), case)


class IntegrationCustodyTests(unittest.TestCase):
    """Simulated process outcomes test the wrapper, not actual checker enforcement."""

    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.directory = Path(self.temporary.name)
        self.root = self.directory / "repo"
        self.root.mkdir()
        for name in (*TYPING.SOURCE_PATHS, TYPING.POLICY_PATH):
            destination = self.root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, destination)
        self.policy, _ = TYPING.inspect_policy(self.root)
        self.checker = self.directory / "checker"
        (self.checker / "bin").mkdir(parents=True)
        (self.checker / "bin/python").write_text("synthetic interpreter; never executed")
        (self.checker / "pyvenv.cfg").write_text(
            "include-system-site-packages = false\nversion = 3.12.3\n")
        site = self.checker / "lib/python3.12/site-packages"
        for name, wheel in self.policy["wheels"].items():
            metadata = site / (name.replace("-", "_") + ".dist-info/METADATA")
            metadata.parent.mkdir(parents=True)
            metadata.write_text(f"Name: {name}\nVersion: {wheel['version']}\n")
        self.receipt = self.directory / "admission.json"
        self.receipt.write_text(json.dumps({
            "schema_version": 1, "checker_root": str(self.checker),
            "python_version": self.policy["python_version"], "mypy_version": self.policy["mypy_version"],
            "wheels": self.policy["wheels"], "files": TYPING.inventory_tree(self.checker),
            "sources": TYPING.source_snapshot(self.root)[1],
            "policy_sha256": TYPING.digest((self.root / TYPING.POLICY_PATH).read_bytes()),
        }))

    def execute(self):
        return TYPING.execute(self.root, self.checker / "bin/python", self.receipt, False)

    def test_healthy_command_selects_all_seven_actual_helpers_and_the_consumer(self):
        version = result(json.dumps({"python": "3.12.3", "mypy": "2.4.1"}))
        with patch.object(TYPING, "run", side_effect=[version, result()]) as invoked:
            evidence = self.execute()
        expected = {
            "/input/core_combination_accounting.py", "/input/core_analysis_cohorts.py",
            "/input/core_chart_accounting.py", "/input/core_chart_contracts.py",
            "/input/core_chart_combination_accounting.py", "/input/core_chart_combination_contracts.py",
            "/input/core_rate_delivery_accounting.py",
            "/input/typed_consumer.py",
        }
        command = invoked.call_args_list[1].args[0]
        self.assertEqual({value for value in command if value.startswith("/input/")}, expected)
        self.assertEqual(len(evidence["sources"]), 18)
        self.assertEqual(len(evidence["checked_files"]), 8)
        self.assertEqual(evidence["scope"], "seven fixture helpers and independent typed consumer")
        sources, _ = TYPING.source_snapshot(self.root)
        expected_generated = {"typed_consumer.py": TYPING.digest(sources[f"{TYPING.FIXTURE_DIR}/typed_consumer.py"] + TYPING.CHART_COMBINATION_CONSUMER + TYPING.RATE_DELIVERY_CONSUMER),
                              **{name: TYPING.digest(raw) for name, raw in TYPING.GENERATED_FIXTURES.items()}}
        self.assertEqual(evidence["generated_source_sha256"], expected_generated)

    def test_stale_source_extra_tool_and_receipt_keys_refuse_before_execution(self):
        source_paths = {"source": "scripts/core-analysis-cohorts.py", "chart_accounting": "scripts/core-chart-accounting.py",
                        "chart_contracts": "scripts/core-chart-contracts.py",
                        "chart_combination_accounting": "scripts/core-chart-combination-accounting.py",
                        "chart_combination_contracts": "scripts/core-chart-combination-contracts.py",
                        "rate_delivery_accounting": "scripts/core-rate-delivery-accounting.py"}
        for mode in (*source_paths, "tool", "key"):
            with self.subTest(mode=mode), patch.object(TYPING, "run") as invoked:
                if mode in source_paths:
                    target = self.root / source_paths[mode]
                    original = target.read_bytes()
                    target.write_bytes(original + b"\n# drift\n")
                elif mode == "tool":
                    target = self.checker / "extra.py"
                    target.write_text("extra")
                else:
                    original_receipt = self.receipt.read_bytes()
                    value = json.loads(original_receipt)
                    value["arbitrary"] = True
                    self.receipt.write_text(json.dumps(value))
                with self.assertRaises(TYPING.Incomplete):
                    self.execute()
                invoked.assert_not_called()
                if mode in source_paths:
                    target.write_bytes(original)
                elif mode == "tool":
                    target.unlink()
                else:
                    self.receipt.write_bytes(original_receipt)

    def test_historical_four_helper_admission_cannot_admit_current_seven(self):
        receipt = json.loads(self.receipt.read_text())
        for path in ("scripts/core-chart-combination-accounting.py", "scripts/core-chart-combination-contracts.py",
                     "scripts/core-rate-delivery-accounting.py"):
            receipt["sources"].pop(path)
        self.receipt.write_text(json.dumps(receipt))
        with patch.object(TYPING, "run") as invoked:
            with self.assertRaises(TYPING.Incomplete):
                self.execute()
        invoked.assert_not_called()

    def test_pre_rate_six_helper_admission_cannot_omit_the_current_rate_owner(self):
        receipt = json.loads(self.receipt.read_text())
        receipt["sources"].pop("scripts/core-rate-delivery-accounting.py", None)
        self.receipt.write_text(json.dumps(receipt))
        version = result(json.dumps({"python": "3.12.3", "mypy": "2.4.1"}))
        with patch.object(TYPING, "run", side_effect=[version, result()]) as invoked:
            with self.assertRaises(TYPING.Incomplete):
                self.execute()
        invoked.assert_not_called()

    def test_missing_rate_source_refuses_before_any_checker_process(self):
        (self.root / "scripts/core-rate-delivery-accounting.py").unlink()
        with patch.object(TYPING, "run") as invoked:
            with self.assertRaisesRegex(TYPING.Incomplete, "Cannot read admitted regular file: .*core-rate-delivery-accounting"):
                self.execute()
        invoked.assert_not_called()

    def test_native_nonzero_exit_and_output_are_preserved_in_failed_evidence(self):
        version = result(json.dumps({"python": "3.12.3", "mypy": "2.4.1"}))
        failed = result("synthetic native checker failure", 7)
        with patch.object(TYPING, "run", side_effect=[version, failed]):
            with self.assertRaises(TYPING.Incomplete) as captured:
                self.execute()
        self.assertEqual(captured.exception.evidence["status"], "incomplete")
        self.assertEqual(captured.exception.evidence["lanes"][0]["exit_code"], 7)
        self.assertEqual(captured.exception.evidence["lanes"][0]["output"], "synthetic native checker failure")

    def test_tool_changed_during_process_is_rejected_by_bookend(self):
        def changed(*_, **__):
            (self.checker / "bin/python").write_text("changed during command")
            return result(json.dumps({"python": "3.12.3", "mypy": "2.4.1"}))
        with patch.object(TYPING, "run", side_effect=changed):
            with self.assertRaisesRegex(TYPING.Incomplete, "inventory differs"):
                self.execute()

    def test_healthy_and_fixture_modes_report_their_actual_control_scope(self):
        version = result(json.dumps({"python": "3.12.3", "mypy": "2.4.1"}))
        for fixtures in (False, True):
            outcomes = [version, result()]
            if fixtures:
                outcomes.extend(result(
                    f"/input/{filename}:5:1: error: synthetic {fragment} rejection  [{code}]\n", 1)
                    for _, filename, code, fragment in TYPING.CASES)
            with self.subTest(fixtures=fixtures), patch.object(TYPING, "run", side_effect=outcomes):
                evidence = TYPING.execute(self.root, self.checker / "bin/python", self.receipt, fixtures)
                expected_status = "passed-static-fixture-typing" if fixtures else "passed-static-typing"
                self.assertEqual(evidence["status"], expected_status)
                self.assertEqual(evidence.get("fixture_controls_requested"), fixtures)
                self.assertEqual(evidence.get("fixture_controls_executed"),
                                 [case[0] for case in TYPING.CASES] if fixtures else [])

    def test_rejected_fixture_diagnostic_does_not_count_as_executed_control(self):
        version = result(json.dumps({"python": "3.12.3", "mypy": "2.4.1"}))
        _, filename, code, fragment = TYPING.CASES[0]
        accepted = result(f"/input/{filename}:5:1: error: synthetic {fragment} rejection  [{code}]\n", 1)
        rejected = result("/input/wrong_value.py:5:1: error: unrelated rejection  [arg-type]\n", 1)
        with patch.object(TYPING, "run", side_effect=[version, result(), accepted, rejected]):
            with self.assertRaises(TYPING.Incomplete) as captured:
                TYPING.execute(self.root, self.checker / "bin/python", self.receipt, True)
        evidence = captured.exception.evidence
        self.assertEqual(evidence["status"], "incomplete")
        self.assertEqual(evidence.get("fixture_controls_requested"), True)
        self.assertEqual(evidence.get("fixture_controls_executed"), ["wrong_field"])

    def test_external_result_requires_fresh_path_and_never_replaces_existing_bytes(self):
        destination = self.directory / "result.json"
        TYPING.write_result(destination, self.root, {"status": "synthetic"})
        before = destination.read_bytes()
        with self.assertRaises(FileExistsError):
            TYPING.write_result(destination, self.root, {"status": "overwrite"})
        self.assertEqual(destination.read_bytes(), before)
        with self.assertRaises(TYPING.Incomplete):
            TYPING.write_result(self.root / "result.json", self.root, {})


class ProcessTests(unittest.TestCase):
    def test_runner_preserves_real_child_exit_status(self):
        outcome = TYPING.run([sys.executable, "-I", "-B", "-c", "raise SystemExit(7)"], cwd=ROOT)
        self.assertEqual(outcome["exit_code"], 7)
        self.assertIsNone(outcome["failure"])

    def test_runner_output_limit_kills_owned_group(self):
        outcome = TYPING.run([sys.executable, "-I", "-B", "-c", "import os; os.write(1, b'x' * 2097153)"], cwd=ROOT)
        self.assertEqual(outcome["failure"], "output budget exceeded")
        self.assertLessEqual(len(outcome["output"]), TYPING.MAX_OUTPUT)


if __name__ == "__main__":
    unittest.main()
