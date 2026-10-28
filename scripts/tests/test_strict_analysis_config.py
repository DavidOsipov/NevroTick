"""Independent weakening/typo counterexamples for analysis configuration admission."""

import copy
import importlib.util
from pathlib import Path
import unittest


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("strict_analysis_policy", ROOT / "scripts/check-strict.py")
STRICT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(STRICT)


class AnalysisPolicyAdmissionTests(unittest.TestCase):
    def setUp(self):
        self.snapshot = {name: (ROOT / name).read_bytes() for name in STRICT.CONFIG_FILES}
        self.config = STRICT.read_configuration(ROOT, self.snapshot)

    def reject_value(self, file, keys, value):
        candidate = copy.deepcopy(self.config)
        owner = candidate[file]
        for key in keys[:-1]:
            owner = owner[key]
        owner[keys[-1]] = value
        with self.assertRaises(STRICT.ENV.Incomplete):
            STRICT.require_analysis_configuration(candidate)

    def test_current_native_configurations_are_a_healthy_control(self):
        STRICT.require_analysis_configuration(self.config)

    def test_read_configuration_rejects_each_observed_weakening(self):
        for file, before, after in (
            (".cargo/audit.toml", "stale = false", "stale = true"),
            (".cargo/audit.toml", "fetch = false", "fetch = true"),
            (".cargo/mutants.toml", "cap_lints = false", "cap_lints = true"),
            (".cargo/mutants.toml", "all_features = false", "all_features = true"),
            ("deny.toml", 'unsound = "all"', 'unsound = "none"'),
        ):
            with self.subTest(file=file, change=after):
                snapshot = dict(self.snapshot)
                self.assertEqual(snapshot[file].count(before.encode()), 1)
                snapshot[file] = snapshot[file].replace(before.encode(), after.encode(), 1)
                with self.assertRaises(STRICT.ENV.Incomplete):
                    STRICT.read_configuration(ROOT, snapshot)

    def test_advisory_ignore_cannot_be_introduced_in_either_auditor(self):
        for file in ("deny.toml", ".cargo/audit.toml"):
            with self.subTest(file=file):
                self.reject_value(file, ["advisories", "ignore"], ["RUSTSEC-2026-9999"])

    def test_unreviewed_license_exception_is_rejected(self):
        self.reject_value("deny.toml", ["licenses", "exceptions"], [{"crate": "held", "allow": ["Apache-2.0"]}])

    def test_private_build_and_dev_license_exclusions_are_rejected(self):
        for keys, value in ((["licenses", "private", "ignore"], True),
                            (["licenses", "include-build"], False),
                            (["licenses", "include-dev"], False)):
            with self.subTest(keys=keys):
                self.reject_value("deny.toml", keys, value)

    def test_removing_held_provider_ban_is_rejected(self):
        self.reject_value("deny.toml", ["bans", "deny"], self.config["deny.toml"]["bans"]["deny"][1:])

    def test_wrapper_bypass_and_missing_ban_reason_are_rejected(self):
        for change in ({"wrappers": ["anything"]}, {"reason": " "}):
            with self.subTest(change=change):
                denied = copy.deepcopy(self.config["deny.toml"]["bans"]["deny"])
                denied[0].update(change)
                self.reject_value("deny.toml", ["bans", "deny"], denied)

    def test_unreviewed_source_and_build_script_permissions_are_rejected(self):
        self.reject_value("deny.toml", ["sources", "allow-git"], ["https://unreviewed.invalid/repo"])
        self.reject_value("deny.toml", ["bans", "build", "allow-build-scripts"], ["unreviewed"])

    def test_yank_and_warning_bypass_are_rejected(self):
        self.reject_value(".cargo/audit.toml", ["yanked", "enabled"], False)
        self.reject_value(".cargo/audit.toml", ["output", "deny"], [])
        self.reject_value("deny.toml", ["advisories", "disable-yank-checking"], True)

    def test_universal_mutation_features_and_lint_caps_are_rejected(self):
        self.reject_value(".cargo/mutants.toml", ["additional_cargo_args"], ["--frozen", "--all-features"])
        self.reject_value(".cargo/mutants.toml", ["additional_cargo_args"], ["--frozen", "--", "--cap-lints=allow"])

    def test_mutation_scope_exclusion_is_rejected(self):
        self.reject_value(".cargo/mutants.toml", ["exclude_globs"], ["src/security.rs"])

    def test_unknown_leaf_and_empty_section_are_rejected_for_every_tool(self):
        for file in ("deny.toml", ".cargo/audit.toml", ".cargo/mutants.toml"):
            for value in (True, {}):
                with self.subTest(file=file, value=value):
                    self.reject_value(file, ["unreviewed_policy"], value)

    def test_numeric_boolean_substitution_is_not_equal_policy(self):
        self.reject_value(".cargo/audit.toml", ["database", "fetch"], 0)
        self.reject_value(".cargo/mutants.toml", ["cap_lints"], 0)


if __name__ == "__main__":
    unittest.main()
