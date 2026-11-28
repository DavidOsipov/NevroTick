#!/usr/bin/env python3
"""Qualify strict Rust policy using isolated, dependency-free control workspaces.

The default checks configuration and tool admission only. --fixtures proves the
configured guards on fixed fixtures; it does not qualify application code.
--workspace checks adoption by real workspace members without executing builds.
--preview-contract checks the selected UI preview admission source without Cargo.
--fixtures with both explicit tool paths uses the hosted product tool admission.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import stat
import subprocess
import sys
import tempfile
import tomllib
from typing import NamedTuple


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
    "std::time::Instant::elapsed",
    "std::time::SystemTime::elapsed",
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
# Source inputs remain separate from configuration parsed as TOML.
WINDOWS_RELEASE_ROLE_SOURCES: dict[str, int] = {
    "crates/nevrotick-ui/Cargo.toml": 64 * 1024,
    "crates/nevrotick-ui/src/main.rs": 512 * 1024,
    "crates/nevrotick-engine/Cargo.toml": 64 * 1024,
    "crates/nevrotick-engine/src/bin/nevrotick-observe.rs": 512 * 1024,
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


def clone_policy_controls() -> tuple[str, tuple[tuple[str, str, str], ...]]:
    """Return healthy ownership code and independently selected clone rejections.

    These fixture strings do not detect product defects. The configured Clippy
    runner must compile the healthy neighbor and reject each negative with the
    named diagnostic before reporting a qualified control.
    """
    healthy = '''
/// Copy an ordinary scalar without an unnecessary trait call.
#[must_use]
pub const fn copy_tick(value: u64) -> u64 {
    value
}

/// Produce two owned values when the contract requires both outputs.
#[must_use]
pub fn two_owned<T: Clone>(value: T) -> (T, T) {
    (value.clone(), value)
}
'''
    original = "pub const fn copy_tick(value: u64) -> u64 {\n    value\n}"
    if healthy.count(original) != 1:
        raise ENV.Incomplete("Clone control must select exactly one scalar body.")
    negatives = tuple(
        (label, healthy.replace(original,
         "pub fn copy_tick(value: u64) -> u64 {\n    " + expression + "\n}", 1),
         "clippy::clone_on_copy")
        for label, expression in (
            ("clone-on-copy-method-negative", "value.clone()"),
            ("clone-on-copy-ufcs-negative", "Clone::clone(&value)"),
        )
    )
    return healthy, negatives



def future_policy_controls() -> tuple[str, tuple[tuple[str, str, str], ...]]:
    """Distinguish deliberate unpolled cancellation from implicit future loss.

    This std-only compiler fixture establishes no task settlement or effect
    completion. In particular, dropping a spawned task handle has other semantics.
    """
    healthy = '''
/// Transfer the future to its next owner rather than silently discard it.
#[must_use]
pub fn forward_future<F: Future>(future: F) -> F {
    future
}

/// Deliberately cancel an unpolled future whose cleanup contract permits Drop.
pub fn cancel_unpolled_future<F: Future>(future: F) {
    drop(future);
}
'''
    original = "drop(future);"
    if healthy.count(original) != 1:
        raise ENV.Incomplete("Future control must select exactly one disposal.")
    negative = healthy.replace(original, "let _ = future;", 1)
    return healthy, (("future-implicit-discard-negative", negative,
                      "clippy::let_underscore_future"),)


class WindowsReleaseRole(NamedTuple):
    """Checked source binding for one existing export, not runtime qualification."""

    role: str
    artifact: str
    package: str
    cargo_manifest: str
    entry_source: str
    cargo_target: str
    required_features: tuple[str, ...]
    subsystem: str
    source_state: str
    activation_owner: str
    entry_scope: str
    preentry_owner: str
    runtime_policy: str


def _windows_role_source_bytes(root: Path, name: str, *, maximum: int) -> bytes:
    """Read bounded regular source without following symbolic links.

    Reject redirects and changed file identities. This is admission of the
    selected source paths, not complete source-map or module-closure custody.
    """
    parts: tuple[str, ...] = tuple(name.split("/"))
    directory: int = -1
    descriptor: int = -1
    try:
        if (not root.is_absolute() or root.resolve(strict=True) != root
                or not parts or any(part in {"", ".", ".."} for part in parts)):
            raise ENV.Incomplete("Windows release role source requires canonical checkout paths.")
        directory = os.open(root, os.O_RDONLY | os.O_DIRECTORY | os.O_CLOEXEC | os.O_NOFOLLOW)
        part: str
        for part in parts[:-1]:
            next_directory: int = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_CLOEXEC
                                          | os.O_NOFOLLOW, dir_fd=directory)
            os.close(directory)
            directory = next_directory
        descriptor = os.open(parts[-1], os.O_RDONLY | os.O_CLOEXEC | os.O_NOFOLLOW
                             | os.O_NONBLOCK, dir_fd=directory)
        before: os.stat_result = os.fstat(descriptor)
        if not stat.S_ISREG(before.st_mode) or not 0 <= before.st_size <= maximum:
            raise ENV.Incomplete("Windows release role source exceeds its regular-file byte bound.")
        payload: bytes = b""
        while len(payload) <= maximum:
            chunk: bytes = os.read(descriptor, maximum + 1 - len(payload))
            if not chunk:
                break
            payload += chunk
        after: os.stat_result = os.fstat(descriptor)
        named: os.stat_result = os.stat(parts[-1], dir_fd=directory, follow_symlinks=False)
        identity: tuple[int, ...] = (before.st_dev, before.st_ino, before.st_mode,
                                    before.st_size, before.st_mtime_ns, before.st_ctime_ns)
        if (len(payload) != before.st_size or any(
                (item.st_dev, item.st_ino, item.st_mode, item.st_size,
                 item.st_mtime_ns, item.st_ctime_ns) != identity for item in (after, named))):
            raise ENV.Incomplete("Windows release role source changed during admission.")
        return payload
    except OSError as error:
        raise ENV.Incomplete("Windows release role source is missing, redirected or unreadable.") from error
    finally:
        if descriptor >= 0:
            os.close(descriptor)
        if directory >= 0:
            os.close(directory)


def windows_release_role_bindings(root: Path, inventory: dict | None = None, *,
                                  source_snapshot: dict[str, bytes] | None = None
                                  ) -> dict[str, WindowsReleaseRole]:
    """Match only the workspace UI/Observe exports to immutable source bindings."""
    if source_snapshot is not None and (type(source_snapshot) is not dict
                                        or set(source_snapshot) != set(WINDOWS_RELEASE_ROLE_SOURCES)):
        raise ENV.Incomplete("Workspace role source snapshot differs from the fixed source roster.")
    expected: dict[str, WindowsReleaseRole] = {
        "ui": WindowsReleaseRole(
            "ui", "nevrotick-ui.exe", "nevrotick-ui", "crates/nevrotick-ui/Cargo.toml",
            "crates/nevrotick-ui/src/main.rs", "package-default-main", (), "gui", "fixture",
            "entrypoint", "rust-entry-source-only", "scripts/verify.py:compile_ui_entry",
            "pending-complete-role-qualification"),
        "observer": WindowsReleaseRole(
            "observer", "nevrotick-observe.exe", "nevrotick-engine", "crates/nevrotick-engine/Cargo.toml",
            "crates/nevrotick-engine/src/bin/nevrotick-observe.rs", "declared-bin", (), "console", "experiment",
            "entrypoint", "rust-entry-source-only", "toolchain", "pending-complete-role-qualification"),
    }
    return _windows_role_bindings(root, inventory, section_name="windows_release_roles",
                                  scope="workspace-release-export-pair-source-binding", expected=expected,
                                  source_snapshot=source_snapshot)


def windows_lab_service_role_bindings(root: Path, inventory: dict | None = None
                                      ) -> dict[str, WindowsReleaseRole]:
    """Match only the two inert service targets; grant no native/production authority."""
    expected: dict[str, WindowsReleaseRole] = {
        "lab-engine": WindowsReleaseRole(
            "lab-engine", "nevrotick-lab-s02-engine.exe", "nevrotick-windows-service-fixture",
            "tools/windows-service-fixture/Cargo.toml", "tools/windows-service-fixture/src/bin/engine.rs",
            "declared-bin", (), "console", "fixture", "SCM-laboratory-controller",
            "rust-entry-source-only", "scripts/cross-link-service-fixture.py:run",
            "pending-complete-role-qualification"),
        "lab-broker": WindowsReleaseRole(
            "lab-broker", "nevrotick-lab-s02-broker.exe", "nevrotick-windows-service-fixture",
            "tools/windows-service-fixture/Cargo.toml", "tools/windows-service-fixture/src/bin/broker.rs",
            "declared-bin", (), "console", "fixture", "SCM-laboratory-controller",
            "rust-entry-source-only", "scripts/cross-link-service-fixture.py:run",
            "pending-complete-role-qualification"),
    }
    return _windows_role_bindings(root, inventory, section_name="windows_lab_service_roles",
                                  scope="standalone-laboratory-service-source-binding", expected=expected)


def _windows_role_bindings(root: Path, inventory: dict | None, *, section_name: str,
                           scope: str, expected: dict[str, WindowsReleaseRole],
                           source_snapshot: dict[str, bytes] | None = None
                           ) -> dict[str, WindowsReleaseRole]:
    """Validate one code-owned closed roster against bounded exact Cargo/entry sources.

    Immutable returned rows attest source attribution only. Neither current
    process policy nor final module closure is established by this reader.
    """
    def role_source(name: str, maximum: int) -> bytes:
        """Consume captured immutable bytes, or perform a bounded live admission."""
        if source_snapshot is None:
            return _windows_role_source_bytes(root, name, maximum=maximum)
        payload: object = source_snapshot.get(name)
        if type(payload) is not bytes or len(payload) > maximum:
            raise ENV.Incomplete("Role source snapshot omits bounded immutable input bytes.")
        return payload

    payload: bytes = _windows_role_source_bytes(
        root, "policy/verification-manifest.toml", maximum=512 * 1024)
    if inventory is None:
        try:
            inventory = tomllib.loads(payload.decode("utf-8"))
        except (UnicodeError, ValueError) as error:
            raise ENV.Incomplete("Windows release role inventory is invalid UTF-8 TOML.") from error
    if type(inventory) is not dict:
        raise ENV.Incomplete("Windows release role inventory must be a table.")
    section: object = inventory.get(section_name)
    if (type(section) is not dict or set(section) != {"schema_version", "scope", "roles"}
            or type(section.get("schema_version")) is not int or section["schema_version"] != 1
            or type(section.get("scope")) is not str
            or section["scope"] != scope):
        raise ENV.Incomplete("Windows release role section differs from the bounded source-binding schema.")
    rows: object = section["roles"]
    if type(rows) is not list or len(rows) != len(expected):
        raise ENV.Incomplete("Windows role roster differs from its code-owned section.")
    checked: dict[str, WindowsReleaseRole] = {}
    row: object
    for row in rows:
        if type(row) is not dict or set(row) != set(WindowsReleaseRole._fields):
            raise ENV.Incomplete("Windows release role fields are missing or unknown.")
        field: str
        value: object
        for field, value in row.items():
            if field == "required_features":
                if (type(value) is not list or len(value) > 256
                        or any(type(feature) is not str or not 0 < len(feature) <= 128 for feature in value)):
                    raise ENV.Incomplete("Windows release role required features must be a string array.")
            elif type(value) is not str or not 0 < len(value) <= 256:
                raise ENV.Incomplete("Windows release role fields require exact string types.")
        role: str = row["role"]
        if role not in expected or role in checked:
            raise ENV.Incomplete("Windows release role is unknown or duplicated.")
        binding: WindowsReleaseRole = expected[role]
        wanted: dict[str, object] = dict(binding._asdict())
        wanted["required_features"] = list(binding.required_features)
        if row != wanted:
            raise ENV.Incomplete("Windows release role binding or pending qualification differs from its export owner.")
        try:
            manifest: dict = tomllib.loads(role_source(
                binding.cargo_manifest, 64 * 1024).decode("utf-8"))
        except (UnicodeError, ValueError) as error:
            raise ENV.Incomplete("Windows release role Cargo source is invalid UTF-8 TOML.") from error
        package: object = manifest.get("package")
        binaries: object = manifest.get("bin", [])
        if (type(package) is not dict or type(package.get("name")) is not str
                or package["name"] != binding.package or type(binaries) is not list
                or len(binaries) > 256 or any(type(binary) is not dict for binary in binaries)):
            raise ENV.Incomplete("Windows release role Cargo package or binary targets differ.")
        local_entry: str = str(Path(binding.entry_source).relative_to(Path(binding.cargo_manifest).parent))
        selected: list[dict] = [binary for binary in binaries
                                if binary.get("name") == binding.artifact.removesuffix(".exe")]
        if binding.cargo_target == "package-default-main":
            if (type(package.get("autobins", True)) is not bool or not package.get("autobins", True)
                    or selected or any(binary.get("path") == local_entry for binary in binaries)):
                raise ENV.Incomplete("Windows release role default main was disabled or substituted.")
        else:
            if (len(selected) != 1 or type(selected[0].get("path")) is not str
                    or selected[0]["path"] != local_entry
                    or type(selected[0].get("required-features", [])) is not list
                    or selected[0].get("required-features", []) != list(binding.required_features)):
                raise ENV.Incomplete("Windows release role declared entry or required features differ.")
        role_source(binding.entry_source, 512 * 1024)
        checked[role] = binding
    if set(checked) != set(expected):
        raise ENV.Incomplete("Windows role roster differs from its code-owned section.")
    return checked


def read_configuration(root: Path, snapshot: dict | None = None, *,
                       role_snapshot: dict[str, bytes] | None = None) -> dict:
    require_clippy_locations(root)
    data = snapshot or {name: (root / name).read_bytes() for name in CONFIG_FILES}
    parsed = {name: tomllib.loads(data[name].decode("utf-8")) for name in CONFIG_FILES}
    policy = parsed["policy/rust-workspace.toml"]
    if set(policy) != {"workspace", "profile"} or set(policy["workspace"]) != {"lints"}:
        raise ENV.Incomplete("Rust workspace policy must contain only lints and profiles.")
    required = {
        "rust": {"warnings", "unsafe_code", "unsafe_op_in_unsafe_fn", "unused_unsafe",
                 "unused_must_use", "let_underscore_drop", "unused_macro_rules", "unused_lifetimes",
                 "unfulfilled_lint_expectations", "unknown_lints",
                 "renamed_and_removed_lints", "improper_ctypes", "improper_ctypes_definitions",
                 "unexpected_cfgs", "missing_docs", "unreachable_pub", "unused_qualifications"},
        "clippy": {"all", "pedantic", "missing_docs_in_private_items",
                   "undocumented_unsafe_blocks", "allow_attributes_without_reason",
                   "allow_attributes", "multiple_unsafe_ops_per_block", "unused_result_ok", "string_slice",
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
        "test": {"package": {"nevrotick-ui": {"debug": 0}}},
    }
    if (policy["profile"] != profiles
            or type(policy["profile"]["test"]["package"]["nevrotick-ui"]["debug"]) is not int):
        raise ENV.Incomplete("Workspace release/verification profiles differ from admitted policy.")
    inventory = parsed["policy/verification-manifest.toml"]
    if (inventory.get("schema_version") != 1
            or inventory.get("security_module_minimum_executable_line_percent") != 90
            or inventory.get("critical_file_minimum_mutation_percent") != 95):
        raise ENV.Incomplete("Verification policy must retain reviewed per-module/file thresholds.")
    windows_release_role_bindings(root, inventory, source_snapshot=role_snapshot)
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


def fixture_workspace_policy(snapshot: dict, adapter: str) -> str:
    """Scope the admitted test-debug budget to an existing synthetic adapter.

    Both fixed fixture graphs omit Slint. Preserve the validated policy's lint
    and profile values while naming their actual adapter for its package-only
    debug budget. This does not modify or relax the product policy.
    """
    if adapter not in {"nevrotick-policy-control", "adapter-control"}:
        raise ENV.Incomplete("Unknown synthetic fixture adapter.")
    fragment = snapshot["policy/rust-workspace.toml"].decode("utf-8")
    header = "[profile.test.package.nevrotick-ui]"
    if fragment.count(header) != 1:
        raise ENV.Incomplete("Expected exactly one admitted UI test-debug profile.")
    return fragment.replace(header, f"[profile.test.package.{adapter}]", 1)


PURE_API_HEALTHY = '''//! A pure-role explicit-input policy control.
/// Advance only the supplied coordinate without ambient state or effects.
#[must_use]
pub const fn explicit_tick(tick: u64) -> Option<u64> {
    tick.checked_add(1)
}
/// Compute a monotonic span from two supplied coordinates.
#[must_use]
pub fn explicit_monotonic_span(
    end: std::time::Instant,
    start: std::time::Instant,
) -> Option<std::time::Duration> {
    end.checked_duration_since(start)
}
/// Compute a UTC span from two supplied coordinates.
/// # Errors
/// Returns the supplied-coordinate ordering error when `end` precedes `start`.
pub fn explicit_utc_span(
    end: std::time::SystemTime,
    start: std::time::SystemTime,
) -> Result<std::time::Duration, std::time::SystemTimeError> {
    end.duration_since(start)
}
'''

PURE_ELAPSED_CONTROLS = (
    ("instant", "std::time::Instant::elapsed", '''
/// Deliberately read elapsed time from a supplied monotonic timestamp.
#[must_use]
pub fn ambient_monotonic_elapsed(since: std::time::Instant) -> std::time::Duration {
    use std::time::Instant as Clock;
    Clock::elapsed(&since)
}
'''),
    ("system-time", "std::time::SystemTime::elapsed", '''
/// Deliberately read elapsed time from a supplied UTC timestamp.
/// # Errors
/// Returns the standard error when the supplied coordinate is later than now.
pub fn ambient_utc_elapsed(
    since: std::time::SystemTime,
) -> Result<std::time::Duration, std::time::SystemTimeError> {
    since.elapsed()
}
'''),
)

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
            + '\nresolver = "3"\n' + fixture_workspace_policy(snapshot, "adapter-control"))
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
        adapter.write_text(PURE_API_HEALTHY + PURE_API_VIOLATIONS
                           + "".join(control for _, _, control in PURE_ELAPSED_CONTROLS))
        execute("adapter-identical-capabilities-positive", ["--package", "adapter-control"])
        for name in roles:
            source = fixture / "crates" / name / "src/lib.rs"
            source.write_text(PURE_API_HEALTHY + PURE_API_VIOLATIONS)
            execute(f"{name}-ambient-capabilities-negative", ["--package", name], rejected=True)
            for label, method, control in PURE_ELAPSED_CONTROLS:
                source.write_text(PURE_API_HEALTHY + control)
                execute(f"{name}-{label}-elapsed-negative", ["--package", name],
                        rejected=True, required=("disallowed_methods", PURE_API_REASON, method))
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
        (fixture / "Cargo.toml").write_text(manifest + fixture_workspace_policy(snapshot, "nevrotick-policy-control"))
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
        # Existing clippy::all supplies clone_on_copy; these are new controls,
        # not a newly enabled rule or proof that all accidental clones are found.
        clone_healthy, clone_negatives = clone_policy_controls()
        source.write_text(with_control(clone_healthy))
        execute("clone-policy-positive", clippy)
        for label, negative, diagnostic in clone_negatives:
            source.write_text(with_control(negative))
            execute(label, clippy, diagnostic)
        source.write_text(SOURCE)
        # all already supplies let_underscore_future. A deliberate Drop is
        # cancellation, not execution; the forwarding neighbor retains ownership.
        future_healthy, future_negatives = future_policy_controls()
        source.write_text(with_control(future_healthy))
        execute("future-policy-positive", clippy)
        for label, negative, diagnostic in future_negatives:
            source.write_text(with_control(negative))
            execute(label, clippy, diagnostic)
        source.write_text(SOURCE)
        # One genuine neighbor exercises all four selected lint alternatives.
        # Each negative below changes only its own construct in this same source.
        # In particular, use an owned String rather than a mutex: the existing
        # let_underscore_lock rule must not masquerade as let_underscore_drop.
        four_lint_control = '''
/// Hold the owned value until the end of this scope.
pub fn retain_guard() {
    let _guard = String::from("guard");
}

/// Select a fixed byte through either of two deliberately exercised rules.
macro_rules! lint_choice {
    (first) => { 1_u8 };
    (second) => { 2_u8 };
}

/// Exercise the first macro rule.
pub const LINT_FIRST: u8 = lint_choice!(first);
/// Exercise the second macro rule.
pub const LINT_SECOND: u8 = lint_choice!(second);

/// Return the first borrow; the second input makes the named lifetime necessary.
#[must_use]
pub fn borrow_first<'a>(first: &'a str, _second: &str) -> &'a str {
    first
}

/// Return the byte-one tail only when the input has that UTF-8 boundary.
#[must_use]
pub fn checked_tail(value: &str) -> Option<&str> {
    value.get(1..)
}
'''
        source.write_text(with_control(four_lint_control))
        execute("four-lint-cohort-positive", clippy)
        for label, before, after, command, diagnostic in (
            ("let-underscore-drop-negative", 'let _guard = String::from("guard");',
             'let _ = String::from("guard");', cargo,
             "non-binding let on a type that has a destructor"),
            ("unused-macro-rules-negative", "lint_choice!(second)", "2_u8", cargo,
             "of macro `lint_choice` is never used"),
            ("unused-lifetimes-negative", "borrow_first<'a>", "borrow_first<'a, 'unused>", cargo,
             "lifetime parameter `'unused` never used"),
            ("clippy-string-slice-negative",
             "/// Return the byte-one tail only when the input has that UTF-8 boundary.\n"
             "#[must_use]\npub fn checked_tail(value: &str) -> Option<&str> {\n"
             "    value.get(1..)\n}",
             "/// Deliberately slice the synthetic input at byte one.\n"
             "/// # Panics\n"
             "/// Panics when byte one is outside the input or inside a UTF-8 character.\n"
             "#[must_use]\npub fn checked_tail(value: &str) -> Option<&str> {\n"
             "    Some(&value[1..])\n}", clippy,
             "indexing into a string may panic"),
        ):
            if four_lint_control.count(before) != 1:
                raise ENV.Incomplete(f"{label}: intended source control is absent or ambiguous.")
            source.write_text(with_control(four_lint_control.replace(before, after, 1)))
            execute(label, command, diagnostic)
        source.write_text(SOURCE)
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
    parser.add_argument("--preview-contract", action="store_true",
                        help="check the selected UI preview admission and consumer seams")
    parser.add_argument("--fixture-toolchain-dir", type=Path,
                        help="use an explicitly admitted Linux runner toolchain for fixtures")
    parser.add_argument("--fixture-nextest", type=Path,
                        help="use the pinned nextest executable for explicit runner fixtures")
    args = parser.parse_args(argv)
    explicit_fixtures = args.fixture_toolchain_dir is not None or args.fixture_nextest is not None
    if explicit_fixtures and (not args.fixture_toolchain_dir or not args.fixture_nextest
                              or not args.fixtures or args.workspace or args.pure_api_fixtures
                              or args.preview_contract):
        parser.error("Both explicit fixture tool paths require --fixtures alone.")
    if args.preview_contract:
        if args.fixtures or args.workspace or args.pure_api_fixtures or args.json:
            parser.error("--preview-contract selects a separate source-only JSON report")
        try:
            from preview_contract import report as preview_report

            report = preview_report(ROOT)
        except (ImportError, OSError, ValueError) as error:
            report = {"scope": "UI preview source policy", "status": "incomplete", "error": str(error)}
        print(json.dumps(report, indent=2))
        return int(report["status"] != "passed")
    report = {"scope": "NevroTick strict development policy", "status": "incomplete"}
    receipt = None
    try:
        if args.json:
            receipt = ENV.external_receipt(args.json, ROOT)
        inputs = (*CONFIG_FILES, ".cargo/config.toml", "rust-toolchain.toml",
                  "policy/tool-manifest.json", "scripts/check-strict.py", "scripts/check-environment.py")
        if explicit_fixtures:
            inputs += ("scripts/verify.py",)
        snapshot = {name: (ROOT / name).read_bytes() for name in inputs}
        role_snapshot: dict[str, bytes] = {
            name: _windows_role_source_bytes(ROOT, name, maximum=maximum)
            for name, maximum in WINDOWS_RELEASE_ROLE_SOURCES.items()
        }
        config = read_configuration(ROOT, snapshot, role_snapshot=role_snapshot)
        report["role_source_sha256"] = {
            name: hashlib.sha256(data).hexdigest() for name, data in role_snapshot.items()
        }
        if explicit_fixtures:
            import verify as product

            context = ENV.require_cargo_context(ROOT)
            toolchain = args.fixture_toolchain_dir.resolve(strict=True)
            nextest = args.fixture_nextest.resolve(strict=True)

            def runner_tools() -> dict:
                try:
                    return product.admit_tools(ROOT, toolchain, nextest, "portable")
                except product.ENV.Incomplete as error:
                    raise ENV.Incomplete(str(error)) from error

            identities = runner_tools()
            report["runner_tool_sha256"] = identities
            report["metadata"] = {"tools": {"rustc": {"path": str(toolchain / "bin/rustc")}}}
            report["extensions"] = {"cargo-nextest": {"path": str(nextest), "sha256": identities["cargo-nextest"]}}
            report["runner_smoke"] = {"status": "incomplete", "lanes": []}
            ENV.smoke(ROOT, report["metadata"], report["runner_smoke"])
        else:
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
        if explicit_fixtures and (runner_tools() != identities
                                  or ENV.require_cargo_context(ROOT) != context):
            raise ENV.Incomplete("Explicit runner tooling or Cargo context changed during fixture execution.")
        require_clippy_locations(ROOT)
        if any((ROOT / name).read_bytes() != data for name, data in snapshot.items()):
            raise ENV.Incomplete("Policy/tooling inputs changed during the check; results are stale.")
        if any(_windows_role_source_bytes(ROOT, name, maximum=WINDOWS_RELEASE_ROLE_SOURCES[name]) != data
               for name, data in role_snapshot.items()):
            raise ENV.Incomplete("Workspace role source inputs changed during the check; results are stale.")
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
