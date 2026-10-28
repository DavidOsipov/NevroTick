#!/usr/bin/env python3
"""Qualify strict Rust policy using isolated, dependency-free control workspaces.

The default checks configuration and tool admission only. --fixtures proves the
configured guards on fixed fixtures; it does not qualify application code.
--workspace checks adoption by real workspace members without executing builds.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib


SPEC = importlib.util.spec_from_file_location(
    "check_environment", Path(__file__).with_name("check-environment.py")
)
ENV = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENV)
ROOT = Path(__file__).resolve().parents[1]
CONFIG_FILES = (
    "policy/rust-workspace.toml", "rustfmt.toml", "clippy.toml", "deny.toml",
    ".config/nextest.toml", ".cargo/audit.toml", ".cargo/mutants.toml",
    "policy/verification-manifest.toml",
)

SOURCE = '''//! A dependency-free policy control for the toolchain.
/// Advance a tick without wrapping.
///
/// ```
/// assert_eq!(nevrotick_policy_control::next_tick(41), Some(42));
/// ```
#[must_use]
pub const fn next_tick(tick: u64) -> Option<u64> {
    tick.checked_add(1)
}

#[cfg(test)]
mod tests {
    use super::next_tick;

    /// Check the ordinary case.
    ///
    /// # Panics
    /// Panics if the observed tick differs from the expected value.
    #[test]
    fn ordinary_tick() {
        assert_eq!(next_tick(41), Some(42));
    }

    /// Check the overflow boundary.
    ///
    /// # Panics
    /// Panics if overflow is accepted.
    #[test]
    fn overflow_is_rejected() {
        assert_eq!(next_tick(u64::MAX), None);
    }
}
'''


def with_control(addition: str) -> str:
    return SOURCE.replace("#[cfg(test)]", addition + "\n#[cfg(test)]", 1)


def read_configuration(root: Path, snapshot: dict | None = None) -> dict:
    data = snapshot or {name: (root / name).read_bytes() for name in CONFIG_FILES}
    parsed = {name: tomllib.loads(data[name].decode("utf-8")) for name in CONFIG_FILES}
    policy = parsed["policy/rust-workspace.toml"]
    if set(policy) != {"workspace", "profile"} or set(policy["workspace"]) != {"lints"}:
        raise ENV.Incomplete("Rust workspace policy must contain only lints and profiles.")
    required = {
        "rust": {"warnings", "unsafe_code", "unsafe_op_in_unsafe_fn", "unused_unsafe",
                 "unused_must_use", "unfulfilled_lint_expectations", "unknown_lints",
                 "renamed_and_removed_lints", "improper_ctypes", "improper_ctypes_definitions",
                 "unexpected_cfgs", "missing_docs", "unreachable_pub", "unused_qualifications"},
        "clippy": {"all", "pedantic", "undocumented_unsafe_blocks", "allow_attributes_without_reason",
                   "allow_attributes", "multiple_unsafe_ops_per_block", "unused_result_ok",
                   "unwrap_used", "expect_used", "panic", "todo", "unimplemented", "unreachable",
                   "dbg_macro", "let_underscore_must_use", "as_conversions", "mem_forget", "exit"},
        "rustdoc": {"all", "broken_intra_doc_links", "private_intra_doc_links"},
    }
    rules_by_tool = policy["workspace"]["lints"]
    if set(rules_by_tool) != set(required):
        raise ENV.Incomplete("Missing or unadmitted workspace lint tool tables.")
    for tool, names in required.items():
        if not names.issubset(rules_by_tool[tool]):
            raise ENV.Incomplete(f"Required strict lint missing from {tool} policy.")
    for tool, rules in policy["workspace"]["lints"].items():
        for name, value in rules.items():
            level = value.get("level") if isinstance(value, dict) else value
            if level != "deny":
                raise ENV.Incomplete(f"Unexpected non-deny workspace rule: {tool}::{name}")
            expected_priority = -1 if name in {"warnings", "all", "pedantic"} else 0
            priority = value.get("priority", 0) if isinstance(value, dict) else 0
            if priority != expected_priority:
                raise ENV.Incomplete(f"Unexpected workspace lint priority: {tool}::{name}")
    profiles = {
        "release": {"overflow-checks": True, "debug-assertions": False, "panic": "unwind",
                    "incremental": False, "debug": "limited", "strip": "none"},
        "verify-release": {"inherits": "release", "debug-assertions": True},
    }
    if policy["profile"] != profiles:
        raise ENV.Incomplete("Workspace release/verification profiles differ from admitted policy.")
    inventory = parsed["policy/verification-manifest.toml"]
    if (inventory.get("schema_version") != 1
            or inventory.get("security_module_minimum_executable_line_percent") != 90
            or inventory.get("critical_file_minimum_mutation_percent") != 95):
        raise ENV.Incomplete("Verification policy must retain reviewed per-module/file thresholds.")
    require_nextest_configuration(parsed[".config/nextest.toml"])
    require_analysis_configuration(parsed)
    native_configs = {
        "rustfmt.toml": {"edition": "2024", "style_edition": "2024", "newline_style": "Unix",
                         "max_width": 100, "hard_tabs": False, "tab_spaces": 4,
                         "reorder_imports": True, "reorder_modules": True, "force_explicit_abi": True},
        "clippy.toml": {"check-private-items": True, "allow-unwrap-in-tests": False,
                        "allow-expect-in-tests": False, "allow-panic-in-tests": False,
                        "allow-dbg-in-tests": False},
    }
    for name, expected in native_configs.items():
        if parsed[name] != expected:
            raise ENV.Incomplete(f"Unadmitted configuration keys or values: {name}")
    return parsed


def require_analysis_configuration(parsed: dict) -> None:
    """Reject silent auditor/mutation policy changes, including unknown keys.

    These are bootstrap policy invariants, not a dependency-graph, advisory-age
    or mutation-score gate. Future reviewed exceptions must update this policy
    deliberately alongside the tool's native configuration and controls.
    """
    denied = parsed.get("deny.toml", {}).get("bans", {})
    denied = denied.get("deny") if isinstance(denied, dict) else None
    held = {"ring", "aws-lc-rs", "aws-lc-sys", "aws-lc-fips-sys"}
    if (not isinstance(denied, list) or len(denied) != len(held)
            or any(not isinstance(entry, dict) or set(entry) != {"crate", "reason"}
                   or not isinstance(entry["crate"], str)
                   or not isinstance(entry["reason"], str) or not entry["reason"].strip()
                   for entry in denied)
            or {entry["crate"] for entry in denied} != held):
        raise ENV.Incomplete("Analysis policy must retain every reasoned held-provider ban.")
    expected = {
        "deny.toml": {
            "graph": {"all-features": False, "no-default-features": False},
            "output": {"feature-depth": 2},
            "advisories": {
                "db-urls": ["https://github.com/RustSec/advisory-db"],
                "unmaintained": "all", "unsound": "all", "yanked": "deny",
                "disable-yank-checking": False, "maximum-db-staleness": "P1D",
                "unused-ignored-advisory": "deny", "ignore": [],
            },
            "licenses": {
                "allow": ["GPL-2.0-only"], "confidence-threshold": 1.0,
                "include-dev": True, "include-build": True,
                "unused-allowed-license": "deny", "unused-license-exception": "deny",
                "exceptions": [], "private": {"ignore": False},
            },
            "bans": {
                "multiple-versions": "deny", "multiple-versions-include-dev": True,
                "wildcards": "deny", "allow-wildcard-paths": False,
                "highlight": "all", "deny": denied, "skip": [], "skip-tree": [],
                "build": {"allow-build-scripts": [], "executables": "deny",
                          "interpreted": "deny", "enable-builtin-globs": True,
                          "include-dependencies": True, "include-workspace": True,
                          "include-archives": True},
            },
            "sources": {
                "unknown-registry": "deny", "unknown-git": "deny",
                "allow-registry": ["https://github.com/rust-lang/crates.io-index"],
                "allow-git": [], "required-git-spec": "rev",
                "unused-allowed-source": "allow", "unused-allowed-org": "deny",
            },
        },
        ".cargo/audit.toml": {
            "advisories": {"ignore": [], "informational_warnings": ["unmaintained", "unsound", "notice"]},
            "database": {"url": "https://github.com/RustSec/advisory-db.git", "fetch": False, "stale": False},
            "output": {"deny": ["warnings"], "format": "json", "quiet": False, "show_tree": True},
            "yanked": {"enabled": True, "update_index": False},
        },
        ".cargo/mutants.toml": {
            "additional_cargo_args": ["--frozen"], "all_features": False,
            "cap_lints": False, "copy_vcs": False, "copy_target": False,
            "gitignore": True, "test_tool": "cargo", "test_workspace": True,
            "timeout_multiplier": 3.0, "build_timeout_multiplier": 2.0,
            "minimum_test_timeout": 10.0, "output": "target",
        },
    }

    def require(actual, admitted, path):
        if type(actual) is not type(admitted):
            raise ENV.Incomplete(f"Unadmitted analysis policy type: {path}")
        if isinstance(admitted, dict):
            if set(actual) != set(admitted):
                raise ENV.Incomplete(f"Unknown or missing analysis configuration keys: {path}")
            for key, value in admitted.items():
                require(actual[key], value, f"{path}.{key}")
        elif actual != admitted:
            raise ENV.Incomplete(f"Unadmitted analysis policy value: {path}")

    for name, admitted in expected.items():
        require(parsed.get(name), admitted, name)


def require_nextest_configuration(config: dict) -> None:
    """Nextest only warns about unknown keys; make them and weakened budgets fatal."""
    expected = {
        "nextest-version": {"required": "0.9.150", "recommended": "0.9.150"},
        "profile": {"default": {
            "default-filter": "all()", "test-threads": 2, "retries": 0,
            "flaky-result": "fail", "fail-fast": False, "global-timeout": "10m",
            "slow-timeout": {"period": "30s", "terminate-after": 2,
                             "grace-period": "5s", "on-timeout": "fail"},
            "leak-timeout": {"period": "500ms", "result": "fail"},
            "status-level": "all", "final-status-level": "all",
            "failure-output": "immediate-final", "success-output": "never",
            "junit": {"path": "junit.xml", "report-name": "nevrotick",
                      "store-success-output": False, "store-failure-output": True,
                      "report-skipped": "all", "flaky-fail-status": "failure"},
        }},
    }
    if config != expected:
        raise ENV.Incomplete("Nextest configuration contains unknown keys or differs from admitted strict budgets.")


def require_warning_free(result: dict) -> str:
    output = ENV.require_ok(result)
    if re.search(r"(?im)^\s*warning(?:\[|:)", output):
        raise ENV.Incomplete("Tool returned success with warnings; strict policy requires clean configuration/output.")
    return output


def admit_extensions(root: Path) -> dict:
    """Hash executables before asking them to identify themselves."""
    policy = json.loads((root / "policy/tool-manifest.json").read_text())
    admitted = {}
    for entry in policy["tools"]:
        name = entry["name"]
        found = shutil.which(name)
        if not found:
            raise ENV.Incomplete(f"Missing admitted extension: {name}")
        path = Path(found).resolve()
        identity = ENV.require_admitted_binary(path, entry["binary_sha256"], name)
        output = ENV.require_ok(ENV.run([str(path), *entry["version_args"]], cwd=root))
        if not re.search(r"(?<![\d.])" + re.escape(entry["version"]) + r"(?![\d.])", output):
            raise ENV.Incomplete(f"Unexpected extension version: {name}")
        admitted[name] = {"path": str(path), "sha256": identity, "version": output}
    return admitted


def require_workspace_adoption(root: Path, policy: dict, metadata: dict) -> dict:
    manifest = root / "Cargo.toml"
    if not manifest.is_file():
        raise ENV.Incomplete("No application Cargo workspace exists; application checks remain pending.")
    workspace = tomllib.loads(manifest.read_text())
    if workspace.get("workspace", {}).get("lints") != policy["workspace"]["lints"]:
        raise ENV.Incomplete("Workspace lint tables differ from policy/rust-workspace.toml.")
    if workspace.get("profile") != policy["profile"]:
        raise ENV.Incomplete("Workspace profiles differ from policy/rust-workspace.toml.")
    result = ENV.run([metadata["tools"]["cargo"]["path"], "metadata", "--no-deps",
                      "--frozen", "--format-version", "1"], cwd=root)
    graph = json.loads(ENV.require_ok(result))
    members = set(graph["workspace_members"])
    packages = [package for package in graph["packages"] if package["id"] in members]
    if not packages or len(packages) != len(members):
        raise ENV.Incomplete("Workspace member selection is empty or incomplete.")
    manifests = []
    for package in packages:
        path = Path(package["manifest_path"])
        if not path.resolve().is_relative_to(root.resolve()):
            raise ENV.Incomplete("Workspace member is outside the admitted checkout.")
        parsed = tomllib.loads(path.read_text())
        if parsed.get("lints") != {"workspace": True}:
            raise ENV.Incomplete(f"Member must inherit all workspace lints: {package['name']}")
        manifests.append({"path": str(path.relative_to(root)), "sha256": ENV.digest(path)})
    return {"scope": "manifest policy adoption only; no application build/test execution",
            "members": manifests}


def fixture_command(metadata: dict, work: Path, command: list[str], extensions: dict) -> list[str]:
    toolchain = Path(metadata["tools"]["rustc"]["path"]).parent.parent
    args = ENV.sandbox_command(toolchain, work, command)
    offset = args.index("--")
    extra = ["--dir", "/tools"]
    for name, entry in extensions.items():
        extra.extend(["--ro-bind", entry["path"], f"/tools/{name}"])
    extra.extend(["--setenv", "PATH", "/tools:/toolchain/bin:/usr/bin:/bin"])
    return args[:offset] + extra + args[offset:]


def qualify_fixtures(root: Path, metadata: dict, extensions: dict, evidence: dict,
                     snapshot: dict) -> None:
    if shutil.disk_usage("/tmp").free < 512 * 1024 * 1024:
        raise ENV.Incomplete("Strict fixture requires 512 MiB free scratch space.")
    lanes = evidence.setdefault("lanes", [])
    with tempfile.TemporaryDirectory(dir="/tmp", prefix="nevrotick-strict-") as temporary:
        work = Path(temporary)
        fixture = work / "fixture"
        (fixture / "src").mkdir(parents=True)
        (work / "cargo-home").mkdir()
        (fixture / ".cargo").mkdir()
        (fixture / ".cargo/config.toml").write_bytes(snapshot[".cargo/config.toml"])
        for name in CONFIG_FILES:
            if name == "policy/rust-workspace.toml" or name == "policy/verification-manifest.toml":
                continue
            destination = fixture / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(snapshot[name])
        manifest = (
            '[package]\nname = "nevrotick-policy-control"\nversion = "0.0.0"\n'
            'edition = "2024"\nlicense = "GPL-2.0-only"\npublish = false\n'
            '[lints]\nworkspace = true\n[workspace]\nresolver = "3"\n'
        )
        (fixture / "Cargo.toml").write_text(manifest + snapshot["policy/rust-workspace.toml"].decode("utf-8"))
        (fixture / "Cargo.lock").write_text(
            'version = 4\n\n[[package]]\nname = "nevrotick-policy-control"\nversion = "0.0.0"\n'
        )
        source = fixture / "src/lib.rs"
        source.write_text(SOURCE)

        def execute(label: str, command: list[str], diagnostic: str | None = None) -> str:
            result = ENV.run(fixture_command(metadata, work, command, extensions), cwd=root, timeout=90)
            lanes.append({"lane": label, "expected": "rejected" if diagnostic else "passed", **result})
            if diagnostic is None:
                return require_warning_free(result)
            if result["failure"] or result["exit_code"] == 0 or diagnostic not in result["output"]:
                raise ENV.Incomplete(f"{label}: required rejection diagnostic absent: {diagnostic}")
            return result["output"]

        cargo = ["cargo", "check", "--workspace", "--all-targets", "--frozen", "--jobs", "1"]
        clippy = ["cargo", "clippy", "--workspace", "--all-targets", "--frozen", "--jobs", "1",
                  "--", "-D", "warnings"]
        docs = ["cargo", "doc", "--workspace", "--no-deps", "--frozen", "--jobs", "1"]
        execute("format-positive", ["cargo", "fmt", "--all", "--", "--check"])
        execute("compiler-positive", cargo)
        execute("clippy-positive", clippy)
        execute("rustdoc-positive", docs)
        tests = execute("unit-positive", ["cargo", "test", "--lib", "--frozen", "--jobs", "1"])
        ENV.require_test_count(tests, 2, "policy unit control")
        tests = execute("doctest-positive", ["cargo", "test", "--doc", "--frozen", "--jobs", "1"])
        ENV.require_test_count(tests, 1, "policy doctest control")
        execute("windows-compiler-positive", cargo + ["--target", ENV.WINDOWS_TARGET])
        for label, addition, diagnostic in (
            ("unsafe-negative", '\n/// Deliberately forbidden.\npub unsafe fn forbidden() {}\n',
             "declaration of an `unsafe` function"),
            ("missing-docs-negative", '\npub const UNDOCUMENTED: u8 = 0;\n', "missing documentation"),
            ("stale-expectation-negative", '\n#[expect(dead_code, reason = "deliberate control")]\n'
             '/// Public control.\npub const LIVE: u8 = 0;\n', "this lint expectation is unfulfilled"),
            ("unknown-cfg-negative", '\n#[cfg(nevrotick_unreviewed)]\nfn wrong_cfg() {}\n',
             "unexpected `cfg` condition name"),
            ("unknown-lint-negative", '\n#[deny(nevrotick_unknown_lint)]\n'
             '/// Deliberate lint control.\npub const UNKNOWN: u8 = 0;\n', "unknown lint"),
            ("ignored-result-negative", '\n/// Deliberate result control.\npub fn discard() { '
             'std::fs::read("synthetic-not-executed"); }\n', "unused `Result` that must be used"),
        ):
            source.write_text(with_control(addition))
            execute(label, cargo, diagnostic)
        for label, addition, diagnostic in (
            ("clippy-todo-negative", '\n/// Deliberate panic control.\npub fn prohibited() { todo!() }\n',
             "clippy::todo"),
            ("clippy-cast-negative", '\n/// Deliberate narrowing control.\n#[must_use]\n'
             'pub const fn narrow(value: u64) -> u8 { value as u8 }\n', "as_conversions"),
            ("clippy-ignored-result-negative", '\n/// Deliberate result control.\npub fn discard() { '
             'let _ = std::fs::read("synthetic-not-executed"); }\n', "let_underscore_must_use"),
            ("clippy-unwrap-negative", '\n/// Deliberate unwrap control.\n/// # Panics\n'
             '/// Panics when the value is absent.\n#[must_use]\n'
             'pub fn extract(value: Option<u8>) -> u8 { value.unwrap() }\n', "unwrap_used"),
            ("clippy-expect-negative", '\n/// Deliberate expect control.\n/// # Panics\n'
             '/// Panics when the value is absent.\n#[must_use]\n'
             'pub fn extract(value: Option<u8>) -> u8 { value.expect("synthetic") }\n', "expect_used"),
            ("clippy-reason-negative", '\n#[allow(dead_code)]\nfn unreasoned() {}\n',
             "allow_attributes_without_reason"),
            ("clippy-permanent-allow-negative", '\n#[allow(dead_code, reason = "isolated control")]\n'
             'fn permanently_suppressed() {}\n', "allow_attributes"),
            ("clippy-result-ok-negative", '\n/// Deliberate ignored error control.\n'
             'pub fn discard(value: Result<(), std::io::Error>) { value.ok(); }\n',
             "unused_result_ok"),
            ("clippy-test-unwrap-negative", '\n/// Deliberate test exemption control.\n/// # Panics\n'
             '/// Panics if the synthetic option is absent.\n#[test]\nfn test_scope() { '
             'let value = std::hint::black_box(Some(1_u8)); assert_eq!(value.unwrap(), 1); }\n',
             "unwrap_used"),
        ):
            source.write_text(with_control(addition))
            execute(label, clippy, diagnostic)
        unsafe_control = '''
/// Read one caller-owned byte.
///
/// # Safety
/// The pointer must be aligned, valid and initialized for a byte read.
#[expect(unsafe_code, reason = "isolated compiler control of a reviewed adapter exception")]
#[must_use]
pub unsafe fn read_control(pointer: *const u8) -> u8 {
    // SAFETY: The caller supplies the documented valid readable byte.
    unsafe { *pointer }
}
'''
        source.write_text(with_control(unsafe_control))
        execute("reasoned-unsafe-exception-positive", clippy)
        source.write_text(with_control(unsafe_control.replace(
            "unsafe { *pointer }",
            "unsafe { let first = *pointer; let second = *pointer; first.saturating_add(second) }",
        )))
        execute("multiple-unsafe-operations-negative", clippy, "multiple_unsafe_ops_per_block")
        source.write_text(with_control(unsafe_control.replace(
            "    // SAFETY: The caller supplies the documented valid readable byte.\n", ""
        )))
        execute("undocumented-unsafe-negative", clippy, "undocumented_unsafe_blocks")
        source.write_text(with_control('''
#[expect(dead_code, reason = "isolated control proves a fulfilled local expectation")]
fn deliberately_unreferenced() {}

/// Preserve an intentionally consumed optional result.
#[must_use]
pub fn optional_result(value: Result<u8, std::io::Error>) -> Option<u8> {
    value.ok()
}
'''))
        execute("fulfilled-expectation-and-used-result-positive", clippy)
        source.write_text(SOURCE.replace("Advance a tick without wrapping.", "See [`MissingControlType`]."))
        execute("rustdoc-link-negative", docs, "unresolved link")
        source.write_text(SOURCE.replace("tick.checked_add(1)", "tick.checked_add( 1 )"))
        execute("format-negative", ["cargo", "fmt", "--all", "--", "--check"], "Diff in")
        source.write_text(SOURCE)
        execute("restored-clippy-positive", clippy)
        execute("restored-rustdoc-positive", docs)

        # Exercise optimized profile semantics, rather than only inspecting TOML.
        binary = fixture / "src/main.rs"
        binary.write_text('''//! Profile control.
fn main() {
    let value = std::hint::black_box(u8::MAX - 1);
    let increment = std::hint::black_box(1_u8);
    assert_eq!(value + increment, u8::MAX);
    assert_eq!(u8::MAX.checked_add(increment), None);
    println!("RELEASE_BOUNDARY_OK");
}
''')
        output = execute("release-boundary-positive", ["cargo", "run", "--release", "--frozen", "--jobs", "1"])
        if "RELEASE_BOUNDARY_OK" not in output:
            raise ENV.Incomplete("Optimized arithmetic positive observer did not execute.")
        binary.write_text('''//! Profile control.
fn main() {
    let value = std::hint::black_box(u8::MAX);
    let increment = std::hint::black_box(1_u8);
    println!("{}", value + increment);
}
''')
        execute("release-overflow-negative", ["cargo", "run", "--release", "--frozen", "--jobs", "1"],
                "attempt to add with overflow")
        binary.write_text('''//! Profile control.
fn main() {
    assert!(cfg!(debug_assertions));
    println!("VERIFY_PROFILE_OK");
}
''')
        output = execute("verify-release-positive", ["cargo", "run", "--profile", "verify-release",
                         "--frozen", "--jobs", "1"])
        if "VERIFY_PROFILE_OK" not in output:
            raise ENV.Incomplete("Verification profile observer did not execute.")
        binary.unlink()
        output = execute("nextest-positive", ["cargo", "nextest", "run", "--workspace", "--frozen",
                         "--user-config-file", "none", "--no-tests=fail", "--color", "never"])
        if not re.search(r"2 tests run: 2 passed", output):
            raise ENV.Incomplete("Nextest positive control did not execute exactly two tests.")
        execute("nextest-empty-negative", ["cargo", "nextest", "run", "--workspace", "--frozen",
                "--user-config-file", "none", "--no-tests=fail", "-E", "test(does_not_exist)",
                "--color", "never"], "no tests")
    evidence["status"] = "passed"
    evidence["scope"] = "fixed std-only controls; no application, dependency graph or Windows runtime qualification"


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--fixtures", action="store_true", help="exercise strict policy in isolated temporary workspaces")
    parser.add_argument("--workspace", action="store_true", help="check real workspace policy inheritance (no builds)")
    parser.add_argument("--json", type=Path, help="write exact command results to an external evidence path")
    args = parser.parse_args(argv)
    report = {"scope": "NevroTick strict development policy", "status": "incomplete"}
    receipt = None
    try:
        if args.json:
            receipt = ENV.external_receipt(args.json, ROOT)
        inputs = (*CONFIG_FILES, ".cargo/config.toml", "rust-toolchain.toml",
                  "policy/tool-manifest.json", "scripts/check-strict.py", "scripts/check-environment.py")
        snapshot = {name: (ROOT / name).read_bytes() for name in inputs}
        config = read_configuration(ROOT, snapshot)
        report["metadata"] = ENV.inspect(ROOT)
        report["extensions"] = admit_extensions(ROOT)
        report["configuration_sha256"] = {
            name: hashlib.sha256(data).hexdigest() for name, data in snapshot.items()
        }
        if args.workspace:
            report["workspace"] = require_workspace_adoption(
                ROOT, config["policy/rust-workspace.toml"], report["metadata"]
            )
        if args.fixtures:
            report["fixtures"] = {"status": "incomplete", "lanes": []}
            qualify_fixtures(ROOT, report["metadata"], report["extensions"], report["fixtures"], snapshot)
        if any((ROOT / name).read_bytes() != data for name, data in snapshot.items()):
            raise ENV.Incomplete("Policy/tooling inputs changed during the check; results are stale.")
        report["status"] = "passed"
    except (ENV.Incomplete, OSError, ValueError, KeyError, subprocess.SubprocessError) as error:
        report["error"] = str(error)
    if receipt is not None:
        try:
            ENV.external_receipt(args.json, ROOT).write_text(json.dumps(report, indent=2) + "\n")
        except (ENV.Incomplete, OSError) as error:
            report["status"] = "incomplete"
            report["error"] = str(error)
    print(f"{report['status'].upper()}: {report['scope']}")
    if "error" in report:
        print(report["error"], file=sys.stderr)
    elif not args.workspace:
        print("Application workspace/critical-code gates remain pending; fixture results have bootstrap scope.")
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
