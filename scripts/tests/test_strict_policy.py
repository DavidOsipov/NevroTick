"""Counterexamples for strict-policy admission without executing Cargo.

Real tool behavior belongs to the sandboxed Cargo controls. These tests mutate
owned copies of the policy and parse actual member manifests; only Cargo's
metadata subprocess is replaced with a declared fixture response.
"""

import importlib.util
import json
from pathlib import Path
import re
import shutil
import tempfile
import tomllib
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("strict_policy", ROOT / "scripts/check-strict.py")
STRICT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(STRICT)


class PolicyAdmissionTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="nevrotick-policy-admission-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        for name in STRICT.CONFIG_FILES:
            destination = self.root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, destination)

    def replace(self, name, before, after):
        path = self.root / name
        source = path.read_text(encoding="utf-8")
        self.assertEqual(source.count(before), 1, "Mutation must reach exactly one intended input")
        path.write_text(source.replace(before, after, 1), encoding="utf-8")

    def assert_refused(self, diagnostic):
        with self.assertRaisesRegex(STRICT.ENV.Incomplete, diagnostic):
            STRICT.read_configuration(self.root)

    def test_admitted_repository_configuration_is_a_healthy_control(self):
        parsed = STRICT.read_configuration(self.root)
        self.assertEqual(set(parsed), set(STRICT.CONFIG_FILES))

    def test_removing_required_unwrap_rule_is_rejected(self):
        self.replace("policy/rust-workspace.toml", 'unwrap_used = "deny"\n', "")
        self.assert_refused("Required strict lint missing")

    def test_removing_private_contract_documentation_rule_is_rejected(self):
        self.replace("policy/rust-workspace.toml", 'missing_docs_in_private_items = "deny"\n', "")
        self.assert_refused("Required strict lint missing")

    def test_security_restrictions_cannot_be_removed(self):
        original = (self.root / "policy/rust-workspace.toml").read_bytes()
        for name in ("allow_attributes", "multiple_unsafe_ops_per_block", "unused_result_ok"):
            with self.subTest(lint=name):
                (self.root / "policy/rust-workspace.toml").write_bytes(original)
                self.replace("policy/rust-workspace.toml", f'{name} = "deny"\n', "")
                self.assert_refused("Required strict lint missing")

    def test_empty_clippy_table_cannot_become_an_empty_success(self):
        path = self.root / "policy/rust-workspace.toml"
        source, count = re.subn(
            r"(?ms)^\[workspace\.lints\.clippy\]\n.*?(?=^\[)",
            "[workspace.lints.clippy]\n",
            path.read_text(encoding="utf-8"),
        )
        self.assertEqual(count, 1)
        path.write_text(source, encoding="utf-8")
        self.assert_refused("Required strict lint missing")

    def test_missing_tool_tables_are_rejected(self):
        path = self.root / "policy/rust-workspace.toml"
        source = path.read_text(encoding="utf-8")
        profiles = source[source.index("[profile.release]"):]
        path.write_text("[workspace.lints]\n" + profiles, encoding="utf-8")
        self.assert_refused("workspace lint tool tables")

    def test_disabled_release_overflow_checks_are_rejected(self):
        self.replace("policy/rust-workspace.toml", "overflow-checks = true", "overflow-checks = false")
        self.assert_refused("profiles differ")

    def test_unreviewed_abort_profile_is_rejected(self):
        self.replace("policy/rust-workspace.toml", 'panic = "unwind"', 'panic = "abort"')
        self.assert_refused("profiles differ")

    def test_equal_group_priority_is_rejected(self):
        self.replace(
            "policy/rust-workspace.toml",
            'pedantic = { level = "deny", priority = -1 }',
            'pedantic = { level = "deny", priority = 0 }',
        )
        self.assert_refused("lint priority")

    def test_lowered_unsafe_level_is_rejected(self):
        self.replace("policy/rust-workspace.toml", 'unsafe_code = "deny"', 'unsafe_code = "allow"')
        self.assert_refused("non-deny workspace rule")

    def test_unknown_rustfmt_key_is_not_silently_ignored(self):
        with (self.root / "rustfmt.toml").open("a", encoding="utf-8") as output:
            output.write("\nunknown-formatting-policy = true\n")
        self.assert_refused("configuration keys or values: rustfmt")

    def test_disabling_explicit_abi_formatting_is_rejected(self):
        self.replace("rustfmt.toml", "force_explicit_abi = true", "force_explicit_abi = false")
        self.assert_refused("configuration keys or values: rustfmt")

    def test_unknown_clippy_key_is_rejected(self):
        with (self.root / "clippy.toml").open("a", encoding="utf-8") as output:
            output.write("\nunknown-clippy-policy = true\n")
        self.assert_refused("configuration keys or values: clippy")

    def test_blanket_test_unwrap_exemption_is_rejected(self):
        self.replace("clippy.toml", "allow-unwrap-in-tests = false", "allow-unwrap-in-tests = true")
        self.assert_refused("configuration keys or values: clippy")

    def test_configuration_parser_uses_frozen_bytes_instead_of_rereading(self):
        snapshot = {name: (self.root / name).read_bytes() for name in STRICT.CONFIG_FILES}
        expected = STRICT.read_configuration(self.root, snapshot)
        self.replace("clippy.toml", "allow-unwrap-in-tests = false", "allow-unwrap-in-tests = true")
        self.assertEqual(STRICT.read_configuration(self.root, snapshot), expected)
        self.assert_refused("configuration keys or values: clippy")


class WorkspaceAdoptionTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory(prefix="nevrotick-adoption-control-")
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        fragment = (ROOT / "policy/rust-workspace.toml").read_text(encoding="utf-8")
        self.policy = tomllib.loads(fragment)
        (self.root / "Cargo.toml").write_text(
            '[workspace]\nmembers = ["engine", "broker"]\nresolver = "3"\n' + fragment,
            encoding="utf-8",
        )
        self.packages = []
        for name in ("engine", "broker"):
            directory = self.root / name
            directory.mkdir()
            manifest = directory / "Cargo.toml"
            manifest.write_text(
                f'[package]\nname = "{name}"\nversion = "0.0.0"\n'
                '[lints]\nworkspace = true\n',
                encoding="utf-8",
            )
            self.packages.append({"id": name, "name": name, "manifest_path": str(manifest)})
        self.metadata = {"tools": {"cargo": {"path": "/admitted/cargo"}}}

    def result(self, packages=None, members=None):
        return {
            "command": ["/admitted/cargo", "metadata"],
            "exit_code": 0,
            "failure": None,
            "output": json.dumps({
                "workspace_members": ["engine", "broker"] if members is None else members,
                "packages": self.packages if packages is None else packages,
            }),
        }

    def test_every_declared_member_inherits_in_the_healthy_neighbor(self):
        with patch.object(STRICT.ENV, "run", return_value=self.result()) as run:
            result = STRICT.require_workspace_adoption(self.root, self.policy, self.metadata)
        self.assertEqual({entry["path"] for entry in result["members"]},
                         {"engine/Cargo.toml", "broker/Cargo.toml"})
        run.assert_called_once_with(
            ["/admitted/cargo", "metadata", "--no-deps", "--frozen", "--format-version", "1"],
            cwd=self.root,
        )

    def test_second_member_cannot_silently_omit_lint_inheritance(self):
        manifest = self.root / "broker/Cargo.toml"
        manifest.write_text('[package]\nname = "broker"\nversion = "0.0.0"\n', encoding="utf-8")
        with patch.object(STRICT.ENV, "run", return_value=self.result()):
            with self.assertRaisesRegex(STRICT.ENV.Incomplete, "inherit all workspace lints: broker"):
                STRICT.require_workspace_adoption(self.root, self.policy, self.metadata)

    def test_missing_application_workspace_is_incomplete_before_cargo(self):
        (self.root / "Cargo.toml").unlink()
        with patch.object(STRICT.ENV, "run") as run:
            with self.assertRaisesRegex(STRICT.ENV.Incomplete, "No application Cargo workspace"):
                STRICT.require_workspace_adoption(self.root, self.policy, self.metadata)
        run.assert_not_called()

    def test_empty_metadata_selection_is_incomplete(self):
        with patch.object(STRICT.ENV, "run", return_value=self.result(packages=[], members=[])):
            with self.assertRaisesRegex(STRICT.ENV.Incomplete, "selection is empty or incomplete"):
                STRICT.require_workspace_adoption(self.root, self.policy, self.metadata)

    def test_member_omitted_from_metadata_packages_is_incomplete(self):
        with patch.object(STRICT.ENV, "run", return_value=self.result(packages=self.packages[:1])):
            with self.assertRaisesRegex(STRICT.ENV.Incomplete, "selection is empty or incomplete"):
                STRICT.require_workspace_adoption(self.root, self.policy, self.metadata)


class ToolOutcomeTests(unittest.TestCase):
    @staticmethod
    def result(output, *, exit_code=0, failure=None):
        return {"command": ["admitted-tool"], "output": output,
                "exit_code": exit_code, "failure": failure}

    def test_clean_output_is_a_healthy_control(self):
        output = "Checking warning_controls v0.0.0\nFinished successfully\n"
        self.assertEqual(STRICT.require_warning_free(self.result(output)), output.strip())

    def test_successful_exit_with_native_diagnostic_is_rejected(self):
        for output in (
            "Warning: Unknown configuration option `unreviewed`\n",
            "warning[unknown_lints]: unknown lint `unreviewed`\n",
            "  warning: configuration setting was ignored\n",
        ):
            with self.subTest(output=output):
                with self.assertRaisesRegex(STRICT.ENV.Incomplete, "success with warnings"):
                    STRICT.require_warning_free(self.result(output))

    def test_nonzero_exit_is_rejected_without_requiring_a_warning(self):
        with self.assertRaisesRegex(STRICT.ENV.Incomplete, "Command failed"):
            STRICT.require_warning_free(self.result("fatal fixture rejection", exit_code=7))

    def test_timeout_is_rejected_even_when_leader_returned_zero(self):
        with self.assertRaisesRegex(STRICT.ENV.Incomplete, "wall timeout"):
            STRICT.require_warning_free(self.result("", failure="wall timeout (90s)"))


if __name__ == "__main__":
    unittest.main()
