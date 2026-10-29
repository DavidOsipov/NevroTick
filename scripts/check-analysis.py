#!/usr/bin/env python3
"""Admit pinned analysis payloads; optionally qualify independent tool controls.

No application is scaffolded, no package is installed, and no prior receipt is
accepted as execution evidence. Payload hashes and readonly namespaces protect
against accidental substitution, not a compromised host or concurrent owner.
"""

from __future__ import annotations

import argparse
from concurrent.futures import ThreadPoolExecutor
import hashlib
import importlib.util
import json
import os
from pathlib import Path, PurePosixPath
import re
import shutil
import stat
import sys
import tempfile
import time
import tomllib


ROOT = Path(__file__).resolve().parents[1]


def module(name, filename):
    spec = importlib.util.spec_from_file_location(name, Path(__file__).with_name(filename))
    loaded = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(loaded)
    return loaded


RUNTIME = module("analysis_runtime", "analysis-runtime.py")
CONTROLS = module("analysis_controls", "analysis-controls.py")
Incomplete = RUNTIME.Incomplete
TOOLS = ("miri", "fuzz", "loom", "kani", "lockbud", "vet")
PLATFORM = "x86_64-unknown-linux-gnu"
NIGHTLY = "nightly-2026-10-28"
EXPECTED = {
    "miri": (NIGHTLY, f"common/rustup/toolchains/{NIGHTLY}-{PLATFORM}/bin/cargo-miri",
             "51a4a3a2171925d17cdce768fbce7f15c26e2dd51b226e2ca68f4b8bc0c9162c"),
    "fuzz": ("0.13.2", "common/bin/cargo-fuzz", "f87d63fc80e4f897a32ead5df85bd67665afb6a63ed92bb0d2b0473e15295eba"),
    "loom": ("0.7.2", "common/cargo/registry/cache/index.crates.io-1949cf8c6b5b557f/loom-0.7.2.crate",
             "419e0dc8046cb947daa77eb95ae174acfbddb7673b4151f56d1eed8e93fbfaca"),
    "kani": ("0.68.0", "kani/payload/kani-0.68.0/bin/kani-driver", "a0f84272664e45a6b9f1ad7671c2e20aa4523676fb57f5310c18b0a359278a6d"),
    "lockbud": ("a1b5303cdbb1d4d0ad886359cc71418570e7c5fd", "lockbud/bin/cargo-lockbud",
                "ba6f0b27aff3921c6918d46af93700dbb83e540bacf3b8a5ba776d2fc91692c0"),
    "vet": ("0.10.3", "common/bin/cargo-vet", "a268514d8d4bb62cfe6e11a26686d92fdf2ff5c9e320414afa01cda05e2417cb"),
}
PROFILES = {
    "miri_flags": ["-Zmiri-strict-provenance", "-Zmiri-symbolic-alignment-check", "-Zmiri-many-seeds=0..4"],
    "fuzzer_flags": list(RUNTIME.SANITIZER_FLAGS),
    "vet_flags": ["--locked", "--frozen", "--no-all-features", "--no-minimize-exemptions",
                  "--no-registry-suggestions", "--output-format=json"],
    "kani_flags": ["--default-unwind", "1", "--output-format", "regular", "--solver", "cadical",
                   "-Z", "unstable-options", "--run-sanity-checks", "--harness-timeout", "60s"],
    "lockbud_flags": ["-k", "deadlock", "--exit-code-on-bug"],
}
REQUIRED_SCOPES = {
    "common": {"bin", f"rustup/toolchains/{NIGHTLY}-{PLATFORM}", "cargo/registry/cache",
               "cargo/registry/index", "cargo/registry/src", "controls-source", "miri-sysroot", "rustup/settings.toml"},
    "kani": {"payload/kani-0.68.0", "rustup/toolchains/nightly-2026-08-21-x86_64-unknown-linux-gnu"},
    "lockbud": {"bin", "rustup/toolchains/nightly-2026-02-07-x86_64-unknown-linux-gnu",
                "LICENSE.lockbud", "UPSTREAM_SOURCE.json"},
    "stable": {"bin", "lib"},
}
HEX = re.compile(r"[a-f0-9]{64}\Z")
MAX_JSON = 32 * 1024 * 1024


def unique_object(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise Incomplete(f"Duplicate JSON key: {key}")
        result[key] = value
    return result


def decode(data):
    try:
        return json.loads(data, object_pairs_hook=unique_object,
                          parse_constant=lambda value: (_ for _ in ()).throw(Incomplete("Nonfinite JSON value.")))
    except (ValueError, UnicodeError) as error:
        raise Incomplete("Invalid bounded UTF-8 JSON input.") from error


def read_once(path, *, maximum=MAX_JSON, private=False):
    """Read a single regular-file inode, without following the final symlink."""
    path = Path(path)
    descriptor = os.open(path, os.O_RDONLY | os.O_CLOEXEC | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        before = os.fstat(descriptor)
        if not stat.S_ISREG(before.st_mode) or before.st_size > maximum:
            raise Incomplete("Input must be a bounded regular file.")
        if private and (before.st_uid != os.geteuid() or stat.S_IMODE(before.st_mode) != 0o600
                        or before.st_nlink != 1):
            raise Incomplete("Installation pointer must be owned, mode0600, without hard links.")
        with os.fdopen(descriptor, "rb", closefd=False) as stream:
            data = stream.read(maximum + 1)
        after = os.fstat(descriptor)
        if len(data) > maximum or (before.st_size, before.st_mtime_ns, before.st_ctime_ns) != (
                after.st_size, after.st_mtime_ns, after.st_ctime_ns):
            raise Incomplete("Input changed during bounded read.")
        return data
    finally:
        os.close(descriptor)


def digest(path):
    checksum = hashlib.sha256()
    fd = os.open(path, os.O_RDONLY | os.O_CLOEXEC | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        before = os.fstat(fd)
        if not stat.S_ISREG(before.st_mode):
            raise Incomplete("Hash input is not a regular file.")
        with os.fdopen(fd, "rb", closefd=False) as stream:
            for block in iter(lambda: stream.read(1024 * 1024), b""):
                checksum.update(block)
        after = os.fstat(fd)
        if (before.st_size, before.st_mtime_ns, before.st_ctime_ns) != (
                after.st_size, after.st_mtime_ns, after.st_ctime_ns):
            raise Incomplete("Payload changed while hashing.")
        return checksum.hexdigest()
    finally:
        os.close(fd)


def relative(value):
    if (not isinstance(value, str) or not value or value == "." or "\\" in value or "\0" in value
            or str(PurePosixPath(value)) != value or PurePosixPath(value).is_absolute()
            or any(part in (".", "..") for part in PurePosixPath(value).parts)):
        raise Incomplete("Expected a canonical relative inventory path without traversal.")
    return value


def internal(root, value, *, link=False):
    path = root / relative(value)
    resolved = path.resolve(strict=True)
    if not resolved.is_relative_to(root) or (not link and resolved != path):
        raise Incomplete("Payload path escapes or redirects from its admitted root.")
    return path


def validate_policy(policy):
    if (not isinstance(policy, dict) or not isinstance(policy.get("tools"), dict)
            or policy.get("schema_version") != 1 or policy.get("platform") != PLATFORM
            or policy.get("shipping_toolchain") != "1.99.1" or policy.get("codeql") != "deferred"
            or policy.get("fuzz_target") != PLATFORM
            or policy.get("common_nightly_pin") != NIGHTLY
            or policy.get("common_nightly") != f"common/rustup/toolchains/{NIGHTLY}-{PLATFORM}"
            or policy.get("profiles") != PROFILES or set(policy.get("tools", {})) != set(TOOLS)):
        raise Incomplete("Analysis policy differs from the mandatory fixed security profiles/pins.")
    for name, (version, entrypoint, sha256) in EXPECTED.items():
        item = policy["tools"][name]
        if not isinstance(item, dict):
            raise Incomplete("Tool entry must be a JSON object.")
        if (item.get("version"), item.get("entrypoint"), item.get("sha256")) != (version, entrypoint, sha256):
            raise Incomplete(f"Unreviewed tool identity: {name}.")
    for name, expected in (("kani", "kani/rustup/toolchains/nightly-2026-08-21-x86_64-unknown-linux-gnu"),
                           ("lockbud", "lockbud/rustup/toolchains/nightly-2026-02-07-x86_64-unknown-linux-gnu")):
        if policy["tools"][name].get("toolchain") != expected:
            raise Incomplete(f"Unreviewed private toolchain: {name}.")
    if policy.get("miri_sysroot") != "common/miri-sysroot":
        raise Incomplete("Missing admitted prebuilt Miri sysroot.")


def load_installation(record, root):
    record = Path(record).expanduser()
    if not record.is_absolute() or record.parent != record.parent.resolve(strict=True):
        raise Incomplete("Installation pointer must have a canonical absolute parent.")
    item = decode(read_once(record, maximum=4096, private=True))
    if (not isinstance(item, dict) or item.get("schema_version") != 1 or item.get("platform") != PLATFORM
            or item.get("status") != "installed" or not isinstance(item.get("root"), str)):
        raise Incomplete("Installation pointer is incomplete or has an unknown schema/platform.")
    base = Path(item["root"])
    if not base.is_absolute() or base != base.resolve(strict=True) or not base.is_dir():
        raise Incomplete("Installation root must be a canonical absolute directory.")
    RUNTIME._narrow(base)
    if base.is_relative_to(root) or root.is_relative_to(base):
        raise Incomplete("Installed payload must be separate from the checkout.")
    stable = Path(item.get("stable_toolchain", ""))
    expected_stable = Path.home() / f".rustup/toolchains/1.99.1-{PLATFORM}"
    if (stable != expected_stable or stable != stable.resolve(strict=True) or not stable.is_dir()):
        raise Incomplete("Missing canonical private-pointer stable1.99.1 toolchain identity.")
    return base, stable


def check_inventory(base, specification, stable=None):
    scope_root = specification.get("root")
    if not isinstance(scope_root, str) or scope_root not in REQUIRED_SCOPES:
        raise Incomplete("Unknown inventory root.")
    folder = Path(stable) if scope_root == "stable" and stable else internal(base, scope_root)
    scopes = specification.get("scopes")
    if (not isinstance(scopes, list) or not scopes or not all(isinstance(item, str) for item in scopes)
            or len(scopes) != len(set(scopes))
            or not REQUIRED_SCOPES[scope_root].issubset(set(scopes))):
        raise Incomplete("Complete mandatory payload scopes are absent.")
    for scope in scopes:
        relative(scope)
    if any(a != b and PurePosixPath(a).is_relative_to(b) for a in scopes for b in scopes):
        raise Incomplete("Overlapping inventory scopes are forbidden.")
    expected_hash = specification.get("sha256", "")
    if not isinstance(expected_hash, str) or not HEX.fullmatch(expected_hash):
        raise Incomplete("Inventory requires a pinned SHA256.")
    inventory_path = relative(specification.get("path"))
    if not inventory_path.startswith("evidence/"):
        raise Incomplete("Inventory must be a pinned evidence file.")
    data = read_once(internal(base, inventory_path))
    if hashlib.sha256(data).hexdigest() != expected_hash:
        raise Incomplete("Inventory SHA256 mismatch; refusing to parse substituted bytes.")
    inventory = decode(data)
    if (not isinstance(inventory, dict) or inventory.get("schema_version") != 1 or inventory.get("scopes") != scopes
            or inventory.get("tool_root") != str(folder)
            or inventory.get("analysis_installation_root") != str(base)):
        raise Incomplete("Inventory root/scope identity mismatch.")
    entries = inventory.get("entries")
    if not isinstance(entries, dict) or not entries:
        raise Incomplete("Empty or missing payload inventory.")
    for path in entries:
        relative(path)
        if not any(PurePosixPath(path).is_relative_to(scope) for scope in scopes):
            raise Incomplete("Inventory entry is outside declared scopes.")
    found = set()
    files = []

    def inspect(path):
        key = path.relative_to(folder).as_posix()
        details = path.lstat()
        if stat.S_ISDIR(details.st_mode):
            with os.scandir(path) as children:
                for child in children:
                    inspect(Path(child.path))
            return
        found.add(key)
        expected = entries.get(key)
        if not isinstance(expected, dict):
            raise Incomplete(f"Unexpected payload entry: {scope_root}/{key}")
        if stat.S_ISLNK(details.st_mode):
            target = path.resolve(strict=True)
            if (expected.get("kind") != "symlink" or os.readlink(path) != expected.get("target")
                    or not target.is_relative_to(folder)
                    or not any(target.is_relative_to(folder / scope) for scope in scopes)
                    or (target.is_file() and target.relative_to(folder).as_posix() not in entries)
                    or (target.is_dir() and not any(PurePosixPath(key).is_relative_to(target.relative_to(folder))
                                                   for key in entries))):
                raise Incomplete("Inventory symlink identity/containment mismatch.")
        elif stat.S_ISREG(details.st_mode):
            if (expected.get("kind") != "file" or type(expected.get("bytes")) is not int
                    or details.st_size != expected["bytes"] or not isinstance(expected.get("sha256"), str)
                    or not HEX.fullmatch(expected["sha256"])):
                raise Incomplete(f"Payload digest/size mismatch: {scope_root}/{key}")
            files.append((path, expected["sha256"]))
        else:
            raise Incomplete("Special files are forbidden in payload scopes.")

    for scope in scopes:
        path = folder / scope
        # Parent directories must not redirect before reaching a listed scope.
        if path.parent.resolve(strict=True) != path.parent:
            raise Incomplete("Inventory scope parent redirects through a symlink.")
        inspect(path)
    if found != set(entries):
        raise Incomplete("Missing payload inventory entries.")

    def verify_file(item):
        path, expected = item
        if digest(path) != expected:
            raise Incomplete(f"Payload digest/size mismatch: {scope_root}/{path.relative_to(folder)}")

    # Directory traversal remains a single no-follow pass. Bounded parallel
    # streaming reads avoid serial Drvfs round trips without weakening hashes.
    with ThreadPoolExecutor(max_workers=4) as workers:
        for _ in workers.map(verify_file, files):
            pass
    return {"name": specification["name"], "sha256": expected_hash, "entries": len(found),
            "root": scope_root}, {f"{scope_root}/{key}" for key in found}


def admit(root=ROOT, installation_record=None):
    root = Path(root).resolve(strict=True)
    if os.name != "posix" or not sys.platform.startswith("linux") or os.geteuid() == 0:
        raise Incomplete("Analysis admission requires an ordinary Linux user.")
    data = read_once(root / "policy/analysis-tools.json")
    policy = decode(data)
    validate_policy(policy)
    pointer = installation_record or Path.home() / ".local/share/nevrotick/analysis-installation.json"
    base, stable = load_installation(pointer, root)
    inventories = policy.get("inventories")
    if (not isinstance(inventories, list) or not all(isinstance(item, dict) for item in inventories)
            or len(inventories) != len(REQUIRED_SCOPES)
            or {item.get("root") for item in inventories} != set(REQUIRED_SCOPES)
            or len({item.get("name") for item in inventories}) != len(inventories)):
        raise Incomplete("Complete common/Kani/Lockbud/stable payload inventories are required.")
    reports, covered = [], set()
    for item in inventories:
        report, paths = check_inventory(base, item, stable)
        reports.append(report)
        covered.update(paths)
    for name in TOOLS:
        item = policy["tools"][name]
        path = internal(base, item["entrypoint"])
        if item["entrypoint"] not in covered or digest(path) != item["sha256"]:
            raise Incomplete(f"Entrypoint is not hash-admitted: {name}.")
        if name != "loom" and not os.access(path, os.X_OK):
            raise Incomplete(f"Nonexecutable tool entrypoint: {name}.")
    stable_policy = decode(read_once(root / "policy/tool-manifest.json"))["rust"]
    if stable_policy["channel"] != "1.99.1":
        raise Incomplete("Shipping compiler policy changed.")
    for name, checksum in stable_policy["linux_binary_sha256"].items():
        if digest(internal(stable, "bin/" + relative(name))) != checksum:
            raise Incomplete("Stable compiler executable digest mismatch.")
    RUNTIME._helpers(root)
    return {"root": root, "base": base, "stable": stable, "policy": policy,
            "policy_sha256": hashlib.sha256(data).hexdigest(), "inventories": reports}


def source_hashes(path):
    snapshot = input_snapshot(path.parent, [path])
    result = {}
    for key, value in snapshot.items():
        relative_path = PurePosixPath(key).relative_to(path.name)
        if value["kind"] == "file" and (relative_path.suffix == ".rs" or relative_path.name in ("Cargo.toml", "Cargo.lock")):
            result[relative_path.as_posix()] = value["sha256"]
    return result


def input_snapshot(work, inputs, *, maximum_entries=100000):
    """Complete immutable input identity, including added files/configuration."""
    result = {}
    total_bytes = 0

    def inspect(item):
        nonlocal total_bytes
        # Inspect the root itself before enumeration: rglob on a replaced root
        # symlink would otherwise traverse outside work before noticing it.
        details = item.lstat()
        if len(result) >= maximum_entries:
            raise Incomplete("Control input snapshot exceeded its entry budget.")
        key = item.relative_to(work).as_posix()
        if stat.S_ISDIR(details.st_mode):
            result[key] = {"kind": "directory"}
            with os.scandir(item) as children:
                for child in children:
                    inspect(Path(child.path))
        elif stat.S_ISREG(details.st_mode):
            total_bytes += details.st_size
            if details.st_size > 256 * 1024 ** 2 or total_bytes > 2 * 1024 ** 3:
                raise Incomplete("Control input snapshot exceeded its byte budget.")
            result[key] = {"kind": "file", "bytes": details.st_size, "sha256": digest(item)}
        else:
            raise Incomplete("Copied fixture/dependency inputs must be regular files/directories; symlinks are forbidden.")

    for folder in inputs:
        if not folder.is_absolute() or not folder.is_relative_to(work) or folder.parent != folder.parent.resolve(strict=True):
            raise Incomplete("Control snapshot roots must be canonical descendants of owned work.")
        inspect(folder)
    for candidate in (work / ".cargo", work / "controls/.cargo", work / "cargo-home/config", work / "cargo-home/config.toml"):
        if candidate.exists() or candidate.is_symlink():
            raise Incomplete("Unexpected Cargo configuration was added to the owned control ancestry/home.")
    return result


def require_result(result, *, negative=None):
    if result.get("failure") is not None or type(result.get("exit_code")) is not int:
        raise Incomplete("Analysis execution lost wall/output/process custody.")
    output = result.get("output", "")
    if negative is None:
        if result["exit_code"] != 0:
            raise Incomplete("Healthy/setup command failed: " + output[-2000:])
    elif result["exit_code"] == 0 or negative not in output:
        raise Incomplete("The intended seeded defect was not rejected: " + output[-2000:])
    return output


def miri_count(output):
    """Observe all seeds despite concurrently interleaved libtest summaries."""
    seeds = re.findall(r"Trying seed: (\d+)", output)
    if (sorted(seeds) != ["0", "1", "2", "3"]
            or len(re.findall(r"running 1 test\b", output)) != 4
            or output.count("test result:") != 4
            or len(re.findall(r"\. 1 passed; 0 failed; 0 ignored;", output)) != 4
            or re.search(r"\bFAILED\b|\bpanicked\b|[1-9]\d* failed;", output)):
        raise Incomplete("Miri did not establish one actual passing test for each of four admitted seeds.")
    return 4


def kani_counts(report, defect):
    verification = report.get("verification_results", {})
    summary = verification.get("summary", {})
    results = verification.get("results", [])
    if (any(type(summary.get(key)) is not int for key in ("total_harnesses", "executed", "successful", "failed"))
            or summary.get("status") != "completed" or summary.get("total_harnesses") != 1
            or summary.get("executed") != 1 or len(results) != 1
            or results[0].get("harness_id") != "conversion_is_exact_or_rejected"):
        raise Incomplete("Kani must execute exactly the selected nonempty harness.")
    checks = results[0].get("checks", [])
    if (not isinstance(checks, list) or not checks or not all(isinstance(item, dict) for item in checks)
            or any(item.get("status") not in ("Success", "Satisfied", "Failure", "Unreachable", "Unsatisfiable")
                   for item in checks)):
        raise Incomplete("Kani returned no checks.")
    failures = [item for item in checks if item.get("status") == "Failure"]
    if defect:
        if (summary.get("failed") != 1 or summary.get("successful") != 0 or results[0].get("status") != "Failure"
                or not any(item.get("category") == "assertion" and "u32::from(ns) == wide" in
                           item.get("description", "") for item in failures)):
            raise Incomplete("Kani did not identify the intended conversion assertion.")
    elif (summary.get("successful") != 1 or summary.get("failed") != 0 or failures
          or results[0].get("status") != "Success"
          or any(item.get("status") not in ("Success", "Satisfied") for item in checks)
          or sum(item.get("category") == "cover" and item.get("status") == "Satisfied"
                 for item in checks) != 3):
        raise Incomplete("Kani healthy proof/three boundary cover checks were not established.")
    return {"harnesses": 1, "checks": len(checks), "failed_checks": len(failures)}


def vet_counts(report, defect):
    if not isinstance(report, dict):
        raise Incomplete("cargo-vet returned no structured check result.")
    if not defect:
        if (report.get("conclusion") != "success"
                or any(report.get(key) != [] for key in ("vetted_fully", "vetted_partially", "vetted_with_exemptions"))):
            raise Incomplete("Empty-graph vet control did not establish success without invented audits/exemptions.")
        return {"external_packages": 0, "exemptions": 0}
    failures = report.get("failures", [])
    if (report.get("conclusion") != "fail (vetting)" or not isinstance(failures, list)
            or not any(isinstance(item, dict) and item.get("name") == "loom" and item.get("version") == "0.7.2"
                       and "safe-to-deploy" in item.get("missing_criteria", []) for item in failures)):
        raise Incomplete("cargo-vet did not reject the actual unreviewed Loom dependency graph.")
    return {"missing_audits": len(failures), "loom_missing_safe_to_deploy": True}


def vet_store_entries(store, expected=None):
    """Reject root links/new entries/size drift before reading store contents."""
    if (not stat.S_ISDIR(store.lstat().st_mode)
            or store.parent != store.parent.resolve(strict=True)):
        raise Incomplete("Vet store must be a canonical directory without symlink redirection.")
    names = {"config.toml", "audits.toml", "imports.lock"}
    found = set()
    with os.scandir(store) as children:
        for child in children:
            if child.name not in names or child.name in found:
                raise Incomplete("Unexpected vet store entry; no implicit audits/exemptions are admitted.")
            details = child.stat(follow_symlinks=False)
            if not stat.S_ISREG(details.st_mode) or details.st_size > 4096:
                raise Incomplete("Vet store files must be bounded regular files without symlinks.")
            if expected is not None and details.st_size != expected[child.name]:
                raise Incomplete("Vet store file size changed.")
            found.add(child.name)
    if found != names:
        raise Incomplete("Missing initialized vet store file.")


def vet_store_snapshot(work, store, expected=None):
    sizes = {Path(key).name: value["bytes"] for key, value in (expected or {}).items() if value["kind"] == "file"}
    vet_store_entries(store, sizes if expected is not None else None)
    return input_snapshot(work, [store], maximum_entries=4)


def empty_vet_store(store):
    expected = {"config.toml": {"cargo-vet": {"version": "0.10"}},
                "audits.toml": {"audits": {}}, "imports.lock": {}}
    vet_store_entries(store)
    for name, value in expected.items():
        if tomllib.loads(read_once(store / name, maximum=4096).decode("utf-8")) != value:
            raise Incomplete("Vet control policy must contain no audits/imports/exemptions or extra configuration.")


KANI_SOURCE = '''fn nanos(ms: u16) -> Option<u16> { CONVERSION }
#[kani::proof]
fn conversion_is_exact_or_rejected() {
 let ms: u16 = kani::any();
 let wide = u32::from(ms) * 1_000;
 match nanos(ms) {
  Some(ns) => assert_eq!(u32::from(ns), wide),
  None => assert!(wide > u32::from(u16::MAX)),
 }
 kani::cover!(ms == 0);
 kani::cover!(ms == 65);
 kani::cover!(ms == 66);
}
'''
LOCK_SOURCE = '''use std::sync::Mutex;
pub fn locking_protocol() {
 let lock = Mutex::new(0_u8);
 let first = lock.lock().expect("first lock is healthy");
 BEFORE
 let second = lock.lock().expect("second lock must be admitted after release");
 drop(second);
 AFTER
}
'''


def fixtures(admitted, selected, progress=None):
    base, root, stable, policy = (admitted[key] for key in ("base", "root", "stable", "policy"))
    # tempfile creates a private Linux directory, never a Drvfs payload/work tree.
    work = Path(tempfile.mkdtemp(prefix="nevrotick-analysis-", dir="/tmp"))
    work.chmod(0o700)
    state = {"work": str(work), "lanes": [],
             "qualification": "independent tool controls only; product/native/release qualification pending"}
    if progress:
        progress(state)
    projects = CONTROLS.create(work / "controls")
    registry = work / "cargo-home/registry"
    registry.mkdir(parents=True)
    for folder in ("cache", "index", "src"):
        shutil.copytree(base / "common/cargo/registry" / folder, registry / folder)
    for name, project in projects.items():
        shutil.copyfile(internal(base, f"common/controls-source/{name}/Cargo.lock"), project / "Cargo.lock")
    for case in ("healthy", "defect"):
        project = work / "controls" / ("kani-" + case)
        project.mkdir()
        source = KANI_SOURCE.replace("CONVERSION", "ms.checked_mul(1_000)" if case == "healthy" else "Some(ms.wrapping_mul(1_000))")
        (project / "conversion.rs").write_text(source)
        projects["kani-" + case] = project
        project = work / "controls" / ("lockbud-" + case)
        (project / "src").mkdir(parents=True)
        crate = "nevrotick_lock_control"
        (project / "Cargo.toml").write_text(f'[package]\nname="{crate}"\nversion="0.0.0"\nedition="2021"\npublish=false\n[workspace]\n')
        (project / "Cargo.lock").write_text(f'version = 4\n[[package]]\nname = "{crate}"\nversion = "0.0.0"\n')
        (project / "src/lib.rs").write_text(LOCK_SOURCE.replace("BEFORE", "drop(first);" if case == "healthy" else "")
                                              .replace("AFTER", "" if case == "healthy" else "drop(first);"))
        projects["lockbud-" + case] = project
    sealed = [work / "controls", *(registry / folder for folder in ("cache", "index", "src"))]
    baseline = input_snapshot(work, sealed)
    state["input_baseline"] = baseline
    night = internal(base, policy["common_nightly"])
    common_bin = base / "common/bin"
    records = state["lanes"]

    def execute(lane, command, project, *, compiler=stable, env=None, negative=None,
                mode="ordinary", timeout=300, additional_sealed=(), mutable_store=None):
        if input_snapshot(work, sealed) != baseline:
            raise Incomplete("An immutable control/dependency input changed before execution.")
        before = source_hashes(project)
        store_before = vet_store_snapshot(work, mutable_store) if mutable_store else None
        environment = {"RUSTC": str(compiler / "bin/rustc"), "RUSTDOC": str(compiler / "bin/rustdoc")}
        if lane.startswith("miri-"):
            environment = {"MIRI": str(night / "bin/miri"), "MIRI_SYSROOT": str(base / policy["miri_sysroot"])}
        if compiler == night:
            environment.update(RUSTUP_HOME=str(base / "common/rustup"), RUSTUP_TOOLCHAIN=NIGHTLY)
        if lane.startswith("vet-"):
            environment["CARGO"] = str(stable / "bin/cargo")
        environment.update(env or {})
        paths = [compiler / "bin", common_bin]
        if lane.startswith("lockbud-"):
            paths.insert(0, base / "lockbud/bin")
        if lane.startswith("kani-"):
            paths = [base / "kani/payload/kani-0.68.0/bin", compiler / "bin"]
        result = RUNTIME.run(root, work, command, readonly=[base, stable], path_dirs=paths,
                             cwd=project, environment=environment, mode=mode, timeout=timeout,
                             cpu_seconds=min(timeout, 600), sealed=[*sealed, *additional_sealed])
        result.update(lane=lane, sources_before=before)
        records.append(result)
        if progress:
            progress(state)
        after = source_hashes(project)
        result.update(sources_after=after, sources_unchanged=before == after)
        if mutable_store:
            result["vet_store_lock_exception"] = {
                "reason": "cargo-vet 0.10.3 locks config.toml read-write and unconditionally rewrites all three store files on commit",
                "path": str(mutable_store), "before": store_before, "after": None, "unchanged": False,
                "limits": "fresh owned per-case clone only; canonical policy and dependency/source inputs stay readonly; all clone bytes/entries must remain unchanged",
            }
            try:
                store_after = vet_store_snapshot(work, mutable_store, store_before)
            except (Incomplete, OSError) as error:
                result["vet_store_lock_exception"]["guard_error"] = str(error)
                if progress:
                    progress(state)
                raise
            result["vet_store_lock_exception"].update(after=store_after, unchanged=store_before == store_after)
        result["all_inputs_unchanged"] = input_snapshot(work, sealed) == baseline
        if progress:
            progress(state)
        if (before != after or not result["all_inputs_unchanged"]
                or (mutable_store and not result["vet_store_lock_exception"]["unchanged"])):
            raise Incomplete("Analyzer changed Rust/Cargo/lock control inputs.")
        output = require_result(result, negative=negative)
        return result, output

    for name in selected:
        if name in ("loom", "miri"):
            if name == "loom":
                cases, compiler, binary = ("healthy_atomic_update", "seeded_lost_update"), stable, stable / "bin/cargo"
            else:
                cases, compiler, binary = ("healthy_owned_value", "seeded_use_after_free"), night, base / policy["tools"]["miri"]["entrypoint"]
            for index, case in enumerate(cases):
                selector = ("tests::" if name == "loom" else "") + case
                command = [str(binary), *(["miri"] if name == "miri" else []), "test", "--frozen",
                           "--manifest-path", f"/work/controls/{name}/Cargo.toml",
                           "--target-dir", f"/work/target-{name}-{index}", "--lib", selector,
                           "--", "--exact", "--test-threads=1"]
                result, output = execute(name + "-" + case, command, projects[name], compiler=compiler,
                                         env={"MIRIFLAGS": " ".join(PROFILES["miri_flags"])} if name == "miri" else {},
                                         negative=("Undefined Behavior" if name == "miri" else "both increments must survive") if index else None)
                if not index:
                    if name == "miri":
                        result["observed_tests"] = miri_count(output)
                    else:
                        RUNTIME.ENV.require_test_count(output, 1, name)
                        result["observed_tests"] = 1
                elif name == "miri" and not re.search(r"freed|deallocated|dangling", output, re.I):
                    raise Incomplete("Miri rejection did not identify the seeded freed pointer.")
        elif name == "fuzz":
            for case in ("healthy", "defect"):
                execute("fuzz-build-" + case, [str(common_bin / "cargo-fuzz"), "fuzz", "build", case,
                        "--fuzz-dir", "/work/controls/fuzz", "--sanitizer", "address", "--target", PLATFORM,
                        "--target-dir", "/work/target-fuzz-" + case], projects["fuzz"],
                        compiler=night, timeout=600)
                corpus = work / ("corpus-" + case)
                corpus.mkdir()
                for value in (0, 1, 127, 128, 255):
                    (corpus / str(value)).write_bytes(bytes([value]))
                target = work / ("target-fuzz-" + case) / "x86_64-unknown-linux-gnu/release" / case
                if not target.is_file() or target.is_symlink():
                    raise Incomplete("Actual emitted ASan fuzz target is missing.")
                result, output = execute("fuzz-" + case, ["/work/" + str(target.relative_to(work)),
                        "/work/" + corpus.name, "-artifact_prefix=/work/", *PROFILES["fuzzer_flags"]],
                        projects["fuzz"], compiler=night, mode="sanitizer", timeout=90,
                        negative="signed byte decoding must preserve negative offsets" if case == "defect" else None)
                if case == "healthy":
                    count = re.search(r"Done (\d+) runs", output)
                    if not count or int(count[1]) < 1:
                        raise Incomplete("Fuzzer performed zero observed executions.")
                    result["observed_runs"] = int(count[1])
        elif name == "vet":
            initial = work / "vet-init"
            shutil.copytree(projects["vet-healthy"], initial)
            execute("vet-init", [str(common_bin / "cargo-vet"), "vet", "init"], initial,
                    additional_sealed=[initial / "Cargo.toml", initial / "Cargo.lock", initial / "src"])
            empty_vet_store(initial / "supply-chain")
            for case in ("healthy", "unreviewed"):
                shutil.copytree(initial / "supply-chain", projects["vet-" + case] / "supply-chain")
            # One explicit admission of the freshly initialized empty policy;
            # subsequent checks receive readonly inputs and no mutable baseline.
            baseline = input_snapshot(work, sealed)
            state["vet_policy_baseline"] = baseline
            for case in ("healthy", "unreviewed"):
                store = work / ("vet-store-" + case)
                shutil.copytree(projects["vet-" + case] / "supply-chain", store)
                empty_vet_store(store)
                result, output = execute("vet-" + case, [str(common_bin / "cargo-vet"), "vet", "--manifest-path",
                        f"/work/controls/vet-{case}/Cargo.toml", "--store-path", "/work/" + store.name,
                        *PROFILES["vet_flags"]], projects["vet-" + case],
                        negative="missing" if case == "unreviewed" else None, mutable_store=store)
                parsed = decode(output.encode())
                result["structured_result"] = parsed
                result["observed"] = vet_counts(parsed, case == "unreviewed")
        elif name == "kani":
            compiler = base / policy["tools"]["kani"]["toolchain"]
            for case in ("healthy", "defect"):
                project = projects["kani-" + case]
                result_path = work / ("kani-" + case + "-results.json")
                command = [str(base / policy["tools"]["kani"]["entrypoint"]), f"/work/controls/{project.name}/conversion.rs",
                           "--harness", "conversion_is_exact_or_rejected", "--exact", *PROFILES["kani_flags"],
                           "--export-json", "/work/" + result_path.name, "--target-dir", f"/work/target-kani-{case}"]
                result, output = execute("kani-" + case, command, project, compiler=compiler,
                        env={"RUSTUP_HOME": str(base / "kani/rustup"), "RUSTUP_TOOLCHAIN": "nightly-2026-08-21"},
                        negative="VERIFICATION:- FAILED" if case == "defect" else None)
                result["observed"] = kani_counts(decode(read_once(result_path)), case == "defect")
        elif name == "lockbud":
            compiler = base / policy["tools"]["lockbud"]["toolchain"]
            for case in ("healthy", "defect"):
                project = projects["lockbud-" + case]
                actual = tomllib.loads((project / "Cargo.toml").read_text())["package"]["name"]
                if not re.fullmatch(r"[a-z][a-z0-9_]*", actual):
                    raise Incomplete("Invalid owned Lockbud crate selector.")
                target = work / ("lockbud-target-" + case)
                target.mkdir()
                command = [str(compiler / "bin/cargo"), "lockbud", "-k", "deadlock", "-l", actual,
                           "--exit-code-on-bug", "--", "--frozen", "--lib", "-p", actual,
                           "--target-dir", "/work/" + target.name]
                result, output = execute("lockbud-" + case, command, project, compiler=compiler,
                        env={"LOCKBUD_LOG": "info", "LOCKBUD_LOG_STYLE": "never"},
                        negative='"DoubleLock"' if case == "defect" else None)
                if case == "healthy":
                    artifacts = list(target.rglob("*.rmeta")) + list(target.rglob("*.rlib"))
                    if f"Compiling {actual}" not in output or not artifacts:
                        raise Incomplete("Fresh Lockbud compilation produced no observed library artifacts.")
                    result["observed_artifacts"] = len(artifacts)
    return state


def receipt_file(destination, root):
    path = Path(destination).expanduser()
    if not path.is_absolute() or path.parent != path.parent.resolve(strict=True):
        raise Incomplete("Receipt needs a canonical existing absolute parent.")
    if path.is_relative_to(root) or path.is_relative_to(Path("/mnt")):
        raise Incomplete("Receipt must be external to the checkout on the Linux filesystem.")
    parent = path.parent.stat()
    if parent.st_uid != os.geteuid() or stat.S_IMODE(parent.st_mode) & 0o077:
        raise Incomplete("Receipt parent must be an owned private Linux directory.")
    return os.fdopen(os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600), "w")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tool", choices=("all", *TOOLS), default="all")
    parser.add_argument("--fixtures", action="store_true", help="execute actual healthy and seeded-defect controls")
    parser.add_argument("--json", metavar="PATH", help="fresh external JSON receipt; existing receipts are never overwritten")
    arguments = parser.parse_args(argv)
    report = {"schema_version": 1, "status": "incomplete", "mode": "fixtures" if arguments.fixtures else "admission-only",
              "product_qualification": "pending", "codeql": "deferred", "started_unix": time.time()}
    receipt = None
    try:
        if arguments.fixtures and not arguments.json:
            folder = Path.home() / ".local/share/nevrotick/analysis-evidence"
            folder.mkdir(mode=0o700, parents=True, exist_ok=True)
            arguments.json = str(folder / f"controls-{time.time_ns()}-{os.getpid()}.json")
        if arguments.json:
            receipt = receipt_file(arguments.json, ROOT)
            report["receipt"] = arguments.json
        admitted = admit(ROOT)
        report.update(installation_root=str(admitted["base"]), policy_sha256=admitted["policy_sha256"],
                      inventories=admitted["inventories"])
        if arguments.fixtures:
            report.update(fixtures(admitted, TOOLS if arguments.tool == "all" else (arguments.tool,), report.update))
        report["status"] = "passed"
    except (Incomplete, OSError, ValueError, KeyError, TypeError, RuntimeError) as error:
        report["reason"] = str(error)
    finally:
        report["finished_unix"] = time.time()
        if receipt:
            with receipt:
                json.dump(report, receipt, indent=2)
                receipt.write("\n")
    print(json.dumps(report, indent=2))
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
