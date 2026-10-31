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
PURE_CLIPPY_FILES = (
    "crates/time-domain/clippy.toml",
    "crates/ntp-exchange/clippy.toml",
    "crates/time-core/clippy.toml",
)

COMMON_CLIPPY = {
    "check-private-items": True, "allow-unwrap-in-tests": False,
    "allow-expect-in-tests": False, "allow-panic-in-tests": False,
    "allow-dbg-in-tests": False,
}
PURE_API_REASON = "Pure libraries receive explicit inputs; delegate ambient state and effects to the owning adapter."
PURE_METHOD_PATHS = (
    "std::time::SystemTime::now",
    "std::time::Instant::now",
    "std::env::var",
    "std::env::var_os",
    "std::env::vars",
    "std::env::vars_os",
    "std::env::args",
    "std::env::args_os",
    "std::env::current_dir",
    "std::env::current_exe",
    "std::env::temp_dir",
    "std::env::set_current_dir",
    "std::fs::read",
    "std::fs::read_to_string",
    "std::fs::write",
    "std::fs::read_dir",
    "std::fs::create_dir",
    "std::fs::create_dir_all",
    "std::fs::remove_dir",
    "std::fs::remove_dir_all",
    "std::fs::remove_file",
    "std::fs::rename",
    "std::fs::copy",
    "std::fs::hard_link",
    "std::fs::metadata",
    "std::fs::symlink_metadata",
    "std::fs::canonicalize",
    "std::fs::set_permissions",
    "std::fs::read_link",
    "std::fs::exists",
    "std::fs::File::open",
    "std::fs::File::create",
    "std::fs::File::create_new",
    "std::fs::OpenOptions::new",
    "std::fs::DirBuilder::new",
    "std::net::TcpStream::connect",
    "std::net::TcpStream::connect_timeout",
    "std::net::TcpListener::bind",
    "std::net::UdpSocket::bind",
    "std::process::Command::new",
    "std::process::Command::spawn",
    "std::process::Command::output",
    "std::process::Command::status",
    "std::thread::spawn",
    "std::thread::scope",
    "std::thread::sleep",
    "std::thread::park",
    "std::thread::park_timeout",
    "std::thread::yield_now",
    "std::thread::current",
    "std::thread::available_parallelism",
)
PURE_TYPE_PATHS = (
    "std::fs::File",
    "std::fs::OpenOptions",
    "std::fs::DirBuilder",
    "std::fs::ReadDir",
    "std::fs::DirEntry",
    "std::net::TcpStream",
    "std::net::TcpListener",
    "std::net::UdpSocket",
    "std::process::Command",
    "std::process::Child",
    "std::thread::Builder",
    "std::thread::JoinHandle",
    "std::thread::ScopedJoinHandle",
)
PURE_MACRO_PATHS = (
    "std::print",
    "std::println",
    "std::eprint",
    "std::eprintln",
    "std::env",
    "std::option_env",
)

PURE_CLIPPY = {
    **COMMON_CLIPPY,
    **{key: [{"path": path, "reason": PURE_API_REASON} for path in paths]
       for key, paths in (("disallowed-methods", PURE_METHOD_PATHS),
                          ("disallowed-types", PURE_TYPE_PATHS),
                          ("disallowed-macros", PURE_MACRO_PATHS))},
}
CONFIG_FILES = (
    "policy/rust-workspace.toml", "rustfmt.toml", "clippy.toml", "deny.toml",
    ".config/nextest.toml", ".cargo/audit.toml", ".cargo/mutants.toml",
    "policy/verification-manifest.toml", *PURE_CLIPPY_FILES,
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

/// Behavioral controls for the checked successor boundary.
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
    marker = "/// Behavioral controls for the checked successor boundary."
    return SOURCE.replace(marker, addition + "\n" + marker, 1)


def read_configuration(root: Path, snapshot: dict | None = None) -> dict:
    require_clippy_locations(root)
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
        "clippy": {"all", "pedantic", "missing_docs_in_private_items",
                   "undocumented_unsafe_blocks", "allow_attributes_without_reason",
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
        "release": {"opt-level": 3, "lto": "thin", "codegen-units": 1,
                    "overflow-checks": True, "debug-assertions": False, "panic": "unwind",
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
        "clippy.toml": COMMON_CLIPPY,
        **dict.fromkeys(PURE_CLIPPY_FILES, PURE_CLIPPY),
    }
    for name, expected in native_configs.items():
        if parsed[name] != expected:
            raise ENV.Incomplete(f"Unadmitted configuration keys or values: {name}")
    return parsed


def require_clippy_locations(root: Path) -> None:
    """Reject missing, linked or shadowing Clippy policy before invoking Cargo.

    Only the root adapter policy and the three pure-role policies are admitted.
    Nested/alternate files must not change Clippy's nearest-file discovery.
    """
    admitted = {"clippy.toml", *PURE_CLIPPY_FILES}
    for name in admitted:
        path = root / name
        if path.is_symlink() or not path.is_file():
            raise ENV.Incomplete(f"Missing or linked admitted Clippy configuration: {name}")
    candidates = [root / ".clippy.toml"]
    crates = root / "crates"
    if crates.is_dir():
        candidates.extend(path for path in crates.rglob("*clippy.toml")
                          if path.name in {"clippy.toml", ".clippy.toml"})
    for path in candidates:
        if (path.exists() or path.is_symlink()) and str(path.relative_to(root)) not in admitted:
            raise ENV.Incomplete(f"Unadmitted shadow Clippy configuration: {path.relative_to(root)}")


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
            "default-filter": "all()", "test-threads": ENV.CPU_JOBS, "retries": 0,
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


PURE_API_HEALTHY = '''//! A pure-role explicit-input policy control.
/// Advance only the supplied coordinate without ambient state or effects.
#[must_use]
pub const fn explicit_tick(tick: u64) -> Option<u64> {
    tick.checked_add(1)
}
'''

PURE_API_VIOLATIONS = '''
/// Deliberately read the ambient monotonic clock through an alias.
#[must_use]
pub fn ambient_clock() -> std::time::Instant {
    use std::time::Instant as Clock;
    Clock::now()
}
/// Deliberately read the ambient UTC clock.
#[must_use]
pub fn ambient_utc() -> std::time::SystemTime {
    std::time::SystemTime::now()
}
/// Deliberately read ambient configuration through an alias.
/// # Errors
/// Returns the standard error when the synthetic variable is absent.
pub fn ambient_configuration() -> Result<String, std::env::VarError> {
    use std::env::var as read_environment;
    read_environment("NEVROTICK_PURE_CONTROL")
}
/// Deliberately read a file through an alias; this function is never executed.
/// # Errors
/// Returns the underlying file error.
pub fn read_external_file() -> std::io::Result<String> {
    use std::fs::read_to_string as read_file;
    read_file("synthetic-not-executed")
}
/// Deliberately name a socket effect; this function is never executed.
/// # Errors
/// Returns the underlying bind error.
pub fn bind_external_socket() -> std::io::Result<std::net::TcpListener> {
    std::net::TcpListener::bind("127.0.0.1:0")
}
/// Deliberately construct an external-work capability without executing it.
#[must_use]
pub fn external_command() -> std::process::Command {
    std::process::Command::new("synthetic-not-executed")
}
/// Deliberately name a thread effect; this function is never executed.
#[must_use]
pub fn external_thread() -> std::thread::JoinHandle<()> {
    std::thread::spawn(|| ())
}
/// A type-only effect capability, without calling its constructor.
pub type ExternalFile = std::fs::File;
/// Deliberately print from a calculation; this function is never executed.
pub fn implicit_output() {
    println!("synthetic-not-executed");
}
/// Deliberately capture ambient build configuration.
#[must_use]
pub const fn implicit_build_configuration() -> Option<&'static str> {
    option_env!("NEVROTICK_PURE_CONTROL")
}
'''


def qualify_pure_api_fixtures(root: Path, metadata: dict, evidence: dict,
                             snapshot: dict) -> None:
    """Exercise actual workspace discovery, alias bans and adapter permissions.

    These dependency-free functions are compiled, never executed. A healthy
    workspace and the identical adapter-role source distinguish policy rejection
    from compiler/setup failures. Every pure role must reject all three lints.
    """
    if shutil.disk_usage("/tmp").free < 512 * 1024 * 1024:
        raise ENV.Incomplete("Pure API fixtures require 512 MiB free scratch space.")
    lanes = evidence.setdefault("lanes", [])
    with tempfile.TemporaryDirectory(dir="/tmp", prefix="nevrotick-pure-api-") as temporary:
        work = Path(temporary)
        fixture = work / "fixture"
        fixture.mkdir()
        (work / "cargo-home").mkdir()
        (fixture / ".cargo").mkdir()
        (fixture / ".cargo/config.toml").write_bytes(snapshot[".cargo/config.toml"])
        (fixture / "clippy.toml").write_bytes(snapshot["clippy.toml"])
        roles = [Path(name).parent.name for name in PURE_CLIPPY_FILES]
        members = [*roles, "adapter-control"]
        (fixture / "Cargo.toml").write_text(
            '[workspace]\nmembers = ' + json.dumps([f"crates/{name}" for name in members])
            + '\nresolver = "3"\n' + snapshot["policy/rust-workspace.toml"].decode("utf-8"))
        lock = "version = 4\n"
        for name in members:
            member = fixture / "crates" / name
            (member / "src").mkdir(parents=True)
            (member / "Cargo.toml").write_text(
                f'[package]\nname="{name}"\nversion="0.0.0"\nedition="2024"\n'
                '[lints]\nworkspace=true\n')
            (member / "src/lib.rs").write_text(PURE_API_HEALTHY)
            lock += f'\n[[package]]\nname="{name}"\nversion="0.0.0"\n'
        (fixture / "Cargo.lock").write_text(lock)
        for name in PURE_CLIPPY_FILES:
            (fixture / name).write_bytes(snapshot[name])
        toolchain = Path(metadata["tools"]["rustc"]["path"]).parent.parent

        def execute(label: str, selection: list[str], rejected: bool = False,
                    required: tuple[str, ...] | None = None) -> None:
            command = ["cargo", "clippy", *selection, "--all-targets", "--frozen",
                       "--jobs", str(ENV.CPU_JOBS), "--", "-D", "warnings"]
            result = ENV.run(ENV.sandbox_command(toolchain, work, command), cwd=root, timeout=90)
            lanes.append({"lane": label, "expected": "rejected" if rejected else "passed", **result})
            if not rejected:
                require_warning_free(result)
                return
            if required is None:
                required = ("disallowed_methods", "disallowed_types", "disallowed_macros", PURE_API_REASON,
                            "std::time::Instant::now", "std::time::SystemTime::now", "std::env::var",
                            "std::fs::read_to_string", "std::net::TcpListener::bind",
                            "std::process::Command::new", "std::thread::spawn")
            if (result["failure"] or result["exit_code"] == 0
                    or any(marker not in result["output"] for marker in required)):
                raise ENV.Incomplete(f"{label}: intended pure API rejection diagnostics absent.")

        execute("pure-explicit-input-workspace-positive", ["--workspace"])
        adapter = fixture / "crates/adapter-control/src/lib.rs"
        adapter.write_text(PURE_API_HEALTHY + PURE_API_VIOLATIONS)
        execute("adapter-identical-capabilities-positive", ["--package", "adapter-control"])
        for name in roles:
            source = fixture / "crates" / name / "src/lib.rs"
            source.write_text(PURE_API_HEALTHY + PURE_API_VIOLATIONS)
            execute(f"{name}-ambient-capabilities-negative", ["--package", name], rejected=True)
            source.write_text(PURE_API_HEALTHY)
        source = fixture / "crates" / roles[0] / "src/lib.rs"
        source.write_text(PURE_API_HEALTHY + '''
/// Deliberately retain an ambient clock function without calling it.
#[must_use]
pub const fn ambient_clock_reader() -> fn() -> std::time::Instant {
    std::time::Instant::now
}
''')
        execute("pure-ambient-clock-function-reference-negative", ["--package", roles[0]],
                rejected=True, required=("disallowed_methods", PURE_API_REASON, "std::time::Instant::now"))
    evidence["status"] = "passed"
    evidence["scope"] = "selected direct API enforcement; no proof of effect-free wrappers/dependencies"


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

        cargo = ["cargo", "check", "--workspace", "--all-targets", "--frozen", "--jobs", "12"]
        clippy = ["cargo", "clippy", "--workspace", "--all-targets", "--frozen", "--jobs", "12",
                  "--", "-D", "warnings"]
        docs = ["cargo", "doc", "--workspace", "--no-deps", "--frozen", "--jobs", "12"]
        execute("format-positive", ["cargo", "fmt", "--all", "--", "--check"])
        execute("compiler-positive", cargo)
        execute("clippy-positive", clippy)
        # This proves documentation presence on a private helper. Its prose and
        # semantic pre/postconditions still need independent contract review.
        private_control = '''
/// Return the checked successor through the private contract boundary.
#[must_use]
pub const fn private_contract_entry(value: u64) -> Option<u64> {
    checked_successor(value)
}

/// Return the successor or `None` on overflow, without modifying state.
const fn checked_successor(value: u64) -> Option<u64> {
    value.checked_add(1)
}
'''
        source.write_text(with_control(private_control))
        execute("private-contract-docs-positive", clippy)
        source.write_text(with_control(private_control.replace(
            "/// Return the successor or `None` on overflow, without modifying state.\n", ""
        )))
        execute("private-contract-docs-negative", clippy, "missing_docs_in_private_items")
        source.write_text(SOURCE)
        execute("rustdoc-positive", docs)
        tests = execute("unit-positive", ["cargo", "test", "--lib", "--frozen", "--jobs", "12"])
        ENV.require_test_count(tests, 2, "policy unit control")
        tests = execute("doctest-positive", ["cargo", "test", "--doc", "--frozen", "--jobs", "12"])
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
/// Deliberately unused helper for the fulfilled-expectation control.
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
        output = execute("release-boundary-positive", ["cargo", "run", "--release", "--frozen", "--jobs", "12"])
        if "RELEASE_BOUNDARY_OK" not in output:
            raise ENV.Incomplete("Optimized arithmetic positive observer did not execute.")
        binary.write_text('''//! Profile control.
fn main() {
    let value = std::hint::black_box(u8::MAX);
    let increment = std::hint::black_box(1_u8);
    println!("{}", value + increment);
}
''')
        execute("release-overflow-negative", ["cargo", "run", "--release", "--frozen", "--jobs", "12"],
                "attempt to add with overflow")
        binary.write_text('''//! Profile control.
fn main() {
    assert!(cfg!(debug_assertions));
    println!("VERIFY_PROFILE_OK");
}
''')
        output = execute("verify-release-positive", ["cargo", "run", "--profile", "verify-release",
                         "--frozen", "--jobs", "12"])
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
    parser.add_argument("--pure-api-fixtures", action="store_true",
                        help="exercise only pure-role API restrictions and adapter controls")
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
        report["extensions"] = admit_extensions(ROOT) if args.fixtures or not args.pure_api_fixtures else {}
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
        if args.fixtures or args.pure_api_fixtures:
            report["pure_api_fixtures"] = {"status": "incomplete", "lanes": []}
            qualify_pure_api_fixtures(ROOT, report["metadata"], report["pure_api_fixtures"], snapshot)
        require_clippy_locations(ROOT)
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
