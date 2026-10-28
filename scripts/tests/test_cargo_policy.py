"""Real pinned-tool controls on isolated, local-only Cargo fixtures.

No product workspace, package acquisition, advisory refresh, clock change or
service experiment is performed. Missing tools/isolation fail, never skip.
"""

import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import tomllib
import unittest
import xml.etree.ElementTree as ET


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("cargo_policy_environment", ROOT / "scripts/check-environment.py")
ENV = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENV)


class CargoPolicyControls(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        policy = json.loads((ROOT / "policy/tool-manifest.json").read_text())
        rustup = Path(shutil.which("rustup") or "/missing-rustup").resolve()
        ENV.require_admitted_binary(rustup, policy["rust"]["rustup_linux_sha256"], "rustup")
        rustc = Path(ENV.require_ok(ENV.run(
            [str(rustup), "which", "--toolchain", policy["rust"]["channel"], "rustc"], cwd=ROOT
        )))
        cls.toolchain = rustc.parent.parent
        for name in ("rustc", "cargo"):
            ENV.require_admitted_binary(cls.toolchain / "bin" / name,
                                       policy["rust"]["linux_binary_sha256"][name], name)
        for name in ("bwrap", "prlimit"):
            path = Path(shutil.which(name) or ("/missing-" + name)).resolve()
            ENV.require_admitted_binary(path, policy["bootstrap_helpers"][name]["sha256"], name)
        cls.tools = {}
        for name in ("cargo-deny", "cargo-nextest", "cargo-audit", "cargo-mutants"):
            entry = next(item for item in policy["tools"] if item["name"] == name)
            path = Path(shutil.which(name) or ("/missing-" + name)).resolve()
            ENV.require_admitted_binary(path, entry["binary_sha256"], name)
            cls.tools[name] = path

    def setUp(self):
        self.scratch = tempfile.TemporaryDirectory(prefix="nevrotick-cargo-policy-")
        self.addCleanup(self.scratch.cleanup)
        self.work = Path(self.scratch.name) / "work"
        self.fixture = self.work / "fixture"
        (self.work / "cargo-home").mkdir(parents=True)
        self.package(self.fixture, "policy-control")
        (self.fixture / ".cargo").mkdir()
        shutil.copyfile(ROOT / "deny.toml", self.work / "deny.toml")
        shutil.copyfile(ROOT / ".config/nextest.toml", self.work / "nextest.toml")
        for name in ("audit.toml", "mutants.toml"):
            shutil.copyfile(ROOT / ".cargo" / name, self.fixture / ".cargo" / name)

    @staticmethod
    def package(path, name, *, version="0.0.0", license_expression="GPL-2.0-only", extra=""):
        (path / "src").mkdir(parents=True, exist_ok=True)
        license_line = f'license = "{license_expression}"\n' if license_expression else ""
        (path / "Cargo.toml").write_text(
            f'[package]\nname = "{name}"\nversion = "{version}"\nedition = "2024"\n'
            f'publish = false\n{license_line}{extra}', encoding="utf-8"
        )
        (path / "src/lib.rs").write_text(
            "pub fn permitted_tick(tick: u64) -> Option<u64> { tick.checked_add(1) }\n"
            "#[test] fn legitimate_neighbor() { assert_eq!(permitted_tick(41), Some(42)); }\n",
            encoding="utf-8",
        )

    def command(self, command):
        args = ENV.sandbox_command(self.toolchain, self.work, command)
        boundary = args.index("--")
        mounts = ["--dir", "/tools"]
        for name, path in self.tools.items():
            mounts += ["--ro-bind", str(path), "/tools/" + name]
        args[boundary:boundary] = mounts
        result = ENV.run(args, cwd=ROOT, timeout=90)
        self.assertIsNone(result["failure"], result["output"])
        return result

    def lock(self):
        result = self.command(["cargo", "generate-lockfile", "--offline"])
        self.assertEqual(result["exit_code"], 0, result["output"])

    def deny(self, *checks):
        self.lock()
        return self.command([
            "/tools/cargo-deny", "--manifest-path", "/work/fixture/Cargo.toml",
            "--config", "/work/deny.toml", "--frozen", "check", "-D", "warnings",
            *(checks or ("bans", "licenses", "sources")),
        ])

    def reject(self, result, diagnostic):
        self.assertNotEqual(result["exit_code"], 0, result["output"])
        self.assertIn(diagnostic, result["output"])

    def nextest(self, *extra):
        self.lock()
        return self.command([
            "/tools/cargo-nextest", "nextest", "run", "--frozen", "--build-jobs", "1",
            "--config-file", "/work/nextest.toml", "--user-config-file", "none",
            "--no-tests=fail", *extra,
        ])

    def test_allowed_local_graph_passes_all_offline_policy_checks(self):
        dep = self.fixture / "permitted"
        self.package(dep, "permitted-dependency")
        with (self.fixture / "Cargo.toml").open("a") as manifest:
            manifest.write('\n[dependencies]\npermitted-dependency = { path = "permitted", version = "=0.0.0" }\n')
        result = self.deny()
        self.assertEqual(result["exit_code"], 0, result["output"])

    def test_unapproved_license_is_rejected(self):
        self.package(self.fixture, "policy-control", license_expression="Apache-2.0")
        self.reject(self.deny("licenses"), "license")

    def test_unlicensed_package_is_rejected(self):
        self.package(self.fixture, "policy-control", license_expression=None)
        self.reject(self.deny("licenses"), "unlicensed")

    def test_spdx_choice_passes_but_combined_obligations_are_rejected(self):
        self.package(self.fixture, "policy-control", license_expression="GPL-2.0-only OR Apache-2.0")
        result = self.deny("licenses")
        self.assertEqual(result["exit_code"], 0, result["output"])
        self.package(self.fixture, "policy-control", license_expression="GPL-2.0-only AND Apache-2.0")
        self.reject(self.deny("licenses"), "license")

    def test_dev_dependency_license_is_checked(self):
        self.package(self.fixture / "dev-only", "unreviewed-dev", license_expression="Apache-2.0")
        with (self.fixture / "Cargo.toml").open("a") as manifest:
            manifest.write('\n[dev-dependencies]\nunreviewed-dev = { path = "dev-only", version = "=0.0.0" }\n')
        self.reject(self.deny("licenses"), "unreviewed-dev")

    def test_build_dependency_license_is_checked(self):
        self.package(self.fixture / "build-only", "unreviewed-build", license_expression="Apache-2.0")
        with (self.fixture / "Cargo.toml").open("a") as manifest:
            manifest.write('\n[build-dependencies]\nunreviewed-build = { path = "build-only", version = "=0.0.0" }\n')
        self.reject(self.deny("licenses"), "unreviewed-build")

    def test_transitively_enabled_held_provider_is_rejected(self):
        self.package(self.fixture / "provider", "ring")
        self.package(self.fixture / "transitive", "transitive-control", extra=(
            '\n[dependencies]\nring = { path = "../provider", version = "=0.0.0", optional = true }\n'
            '\n[features]\nrestore-provider = ["dep:ring"]\n'
        ))
        with (self.fixture / "Cargo.toml").open("a") as manifest:
            manifest.write('\n[dependencies]\ntransitive-control = { path = "transitive", version = "=0.0.0" }\n')
        result = self.deny("bans")
        self.assertEqual(result["exit_code"], 0, result["output"])
        manifest_path = self.fixture / "Cargo.toml"
        manifest_path.write_text(manifest_path.read_text().replace(
            'path = "transitive", version = "=0.0.0"',
            'path = "transitive", version = "=0.0.0", features = ["restore-provider"]',
        ), encoding="utf-8")
        self.reject(self.deny("bans"), "banned")

    def test_wildcard_path_version_is_rejected(self):
        self.package(self.fixture / "permitted", "permitted-dependency")
        with (self.fixture / "Cargo.toml").open("a") as manifest:
            manifest.write('\n[dependencies]\npermitted-dependency = { path = "permitted", version = "*" }\n')
        self.reject(self.deny("bans"), "wildcard")

    def test_duplicate_versions_are_rejected(self):
        for number in (1, 2):
            self.package(self.fixture / f"dep{number}", "duplicate-control", version=f"{number}.0.0")
        with (self.fixture / "Cargo.toml").open("a") as manifest:
            manifest.write('\n[dependencies]\na = { package = "duplicate-control", path = "dep1", version = "=1.0.0" }\nb = { package = "duplicate-control", path = "dep2", version = "=2.0.0" }\n')
        self.reject(self.deny("bans"), "error[duplicate]")

    def test_unreviewed_build_script_is_rejected_without_execution(self):
        (self.fixture / "build.rs").write_text(
            'fn main() { std::fs::write("SHOULD-NOT-EXECUTE", "fixture").unwrap(); }\n', encoding="utf-8"
        )
        self.reject(self.deny("bans"), "build script")
        self.assertFalse((self.fixture / "SHOULD-NOT-EXECUTE").exists())

    def test_unknown_deny_configuration_key_fails(self):
        with (self.work / "deny.toml").open("a") as config:
            config.write('\nunreviewed-policy-key = true\n')
        self.reject(self.deny("bans"), "unreviewed-policy-key")

    def test_source_checker_rejects_unknown_origins_in_synthetic_metadata(self):
        # Exercise the actual source checker without fetching a registry/repo.
        # This is report-input qualification, not source/artifact provenance.
        self.package(self.fixture / "permitted", "permitted-dependency")
        with (self.fixture / "Cargo.toml").open("a") as manifest:
            manifest.write('\n[dependencies]\npermitted-dependency = { path = "permitted", version = "=0.0.0" }\n')
        self.lock()
        result = self.command(["cargo", "metadata", "--format-version", "1", "--frozen"])
        self.assertEqual(result["exit_code"], 0, result["output"])
        metadata = json.loads(result["output"])
        package = next(item for item in metadata["packages"] if item["name"] == "permitted-dependency")
        cases = [
            ("registry+https://github.com/rust-lang/crates.io-index", True),
            ("registry+https://unreviewed.invalid/index", False),
            ("git+https://unreviewed.invalid/crate?rev=" + "0" * 40 + "#" + "0" * 40, False),
        ]
        for origin, allowed in cases:
            with self.subTest(origin=origin):
                package["source"] = origin
                (self.work / "metadata.json").write_text(json.dumps(metadata), encoding="utf-8")
                result = self.command([
                    "/tools/cargo-deny", "--metadata-path", "/work/metadata.json",
                    "--manifest-path", "/work/fixture/Cargo.toml", "--config", "/work/deny.toml",
                    "--frozen", "check", "-D", "warnings", "sources",
                ])
                if allowed:
                    self.assertEqual(result["exit_code"], 0, result["output"])
                else:
                    self.reject(result, "error[source-not-allowed]")

    def test_nextest_valid_test_runs_and_produces_complete_junit(self):
        with (self.fixture / "src/lib.rs").open("a") as source:
            source.write('#[test]\n#[ignore = "synthetic skipped-report control"]\nfn reported_skip() {}\n')
        result = self.nextest()
        self.assertEqual(result["exit_code"], 0, result["output"])
        report = ET.parse(self.fixture / "target/nextest/default/junit.xml").getroot()
        cases = report.findall(".//testcase")
        self.assertEqual(len(cases), 2)
        self.assertEqual(sum(case.find("skipped") is not None for case in cases), 1)
        self.assertEqual(sum(case.find("failure") is not None for case in cases), 0)

    def test_nextest_failure_is_rejected(self):
        with (self.fixture / "src/lib.rs").open("a") as source:
            source.write('#[test] fn deliberate_failure() { assert_eq!(1, 2); }\n')
        result = self.nextest()
        self.assertEqual(result["exit_code"], 100, result["output"])
        self.assertIn("deliberate_failure", result["output"])
        report = ET.parse(self.fixture / "target/nextest/default/junit.xml").getroot()
        self.assertEqual(len(report.findall(".//failure")), 1)

    def test_nextest_empty_selection_is_rejected(self):
        result = self.nextest("-E", "none()")
        self.assertEqual(result["exit_code"], 4, result["output"])
        self.assertIn("no tests", result["output"].lower())

    def test_nextest_timeout_is_an_error_with_owned_short_budget(self):
        path = self.work / "nextest.toml"
        path.write_text(path.read_text().replace(
            'period = "30s", terminate-after = 2, grace-period = "5s"',
            'period = "100ms", terminate-after = 1, grace-period = "100ms"',
        ), encoding="utf-8")
        with (self.fixture / "src/lib.rs").open("a") as source:
            source.write('#[test] fn deliberate_timeout() { std::thread::sleep(std::time::Duration::from_secs(5)); }\n')
        result = self.nextest()
        self.assertEqual(result["exit_code"], 100, result["output"])
        self.assertIn("TIMEOUT", result["output"])

    def test_nextest_leaked_subprocess_is_an_error(self):
        with (self.fixture / "src/lib.rs").open("a") as source:
            source.write(
                '#[test] fn deliberate_leak() { let _child = std::process::Command::new("/bin/sleep")'
                '.arg("2").spawn().unwrap(); }\n'
            )
        result = self.nextest()
        self.assertEqual(result["exit_code"], 100, result["output"])
        self.assertIn("LEAK", result["output"])

    def test_nextest_unknown_key_warning_is_rejected_by_trusted_policy(self):
        config_path = self.work / "nextest.toml"
        config_path.write_text(config_path.read_text().replace(
            "[profile.default]", "[profile.default]\nunreviewed_policy_key = true", 1
        ), encoding="utf-8")
        # The runner alone accepts this typo: the trusted policy must close it.
        result = self.nextest()
        self.assertEqual(result["exit_code"], 0, result["output"])
        self.assertIn("unreviewed_policy_key", result["output"])
        spec = importlib.util.spec_from_file_location("strict_nextest_policy", ROOT / "scripts/check-strict.py")
        strict = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(strict)
        with self.assertRaises(strict.ENV.Incomplete):
            strict.require_nextest_configuration(tomllib.loads(config_path.read_text()))
        strict.require_nextest_configuration(tomllib.loads((ROOT / ".config/nextest.toml").read_text()))

    def test_mutants_config_lists_nonempty_fixture_and_rejects_unknown_key(self):
        self.lock()
        command = ["/tools/cargo-mutants", "mutants", "--list", "--json"]
        result = self.command(command)
        self.assertEqual(result["exit_code"], 0, result["output"])
        self.assertGreater(len(json.loads(result["output"])), 0)
        with (self.fixture / ".cargo/mutants.toml").open("a") as config:
            config.write("\nunreviewed_policy_key = true\n")
        self.reject(self.command(command), "unreviewed_policy_key")

    def test_audit_config_rejects_synthetic_vulnerability_and_unknown_key(self):
        # A committed, synthetic local DB qualifies parsing/report/exit behavior.
        # --no-yanked excludes registry acquisition in this bounded fixture only.
        # This proves no real DB provenance, refresh age, index/yank or product result.
        database = self.work / "advisory-db"
        advisory = database / "crates/vulnerable-control/RUSTSEC-2026-9999.md"
        advisory.parent.mkdir(parents=True)
        advisory.write_text(
            '```toml\n[advisory]\nid = "RUSTSEC-2026-9999"\npackage = "vulnerable-control"\n'
            'date = "2026-10-29"\nurl = "https://example.invalid/synthetic-fixture"\n'
            '[versions]\npatched = [">=1.0.1"]\n```\n\n# Synthetic advisory control\n\n'
            'Trusted local fixture, not an upstream advisory.\n', encoding="utf-8"
        )
        for args in (
            ["git", "-C", "/work/advisory-db", "init"],
            ["git", "-C", "/work/advisory-db", "add", "."],
            ["git", "-C", "/work/advisory-db", "-c", "user.name=Synthetic fixture",
             "-c", "user.email=fixture@example.invalid", "-c", "commit.gpgsign=false",
             "commit", "-m", "synthetic fixture"],
        ):
            result = self.command(args)
            self.assertEqual(result["exit_code"], 0, result["output"])
        command = ["/tools/cargo-audit", "audit", "--db", "/work/advisory-db", "--no-fetch", "--no-yanked"]
        for version, vulnerable in (("1.0.1", False), ("1.0.0", True)):
            with self.subTest(version=version):
                (self.fixture / "Cargo.lock").write_text(
                    'version = 4\n\n[[package]]\nname = "vulnerable-control"\n'
                    f'version = "{version}"\nsource = "registry+https://github.com/rust-lang/crates.io-index"\n'
                    'checksum = "' + "0" * 64 + '"\n', encoding="utf-8"
                )
                result = self.command(command)
                self.assertEqual(result["exit_code"], int(vulnerable), result["output"])
                report = json.loads(result["output"])
                self.assertEqual(report["vulnerabilities"]["found"], vulnerable)
                self.assertEqual(report["database"]["advisory-count"], 1)
                self.assertEqual(report["lockfile"]["dependency-count"], 1)
        with (self.fixture / ".cargo/audit.toml").open("a") as config:
            config.write("\nunreviewed_policy_key = true\n")
        self.reject(self.command(command), "unreviewed_policy_key")


if __name__ == "__main__":
    unittest.main()
