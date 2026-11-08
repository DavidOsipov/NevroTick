#!/usr/bin/env python3
"""Qualify the inert service fixture through the existing isolated host runner.

Authoritative source and fresh raw records remain in this checkout. The private
persistent cache contains only derived sealed inputs and compiler outputs. This
entry point admits no Windows execution, service installation, network, formal
work, mutation campaign or alternative compiler flags. GPL-2.0-only.
"""
from __future__ import annotations

import sys

if sys.flags.optimize:
    raise SystemExit("Qualification refuses optimized Python admission.")

import argparse
import copy
import fcntl
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import resource
import shutil
import stat
import subprocess
import time
import tomllib

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[2]
SELF = "tools/windows-service-fixture/qualification.py"
TOOL = "tools/windows-service-fixture"
MANIFEST = TOOL + "/Cargo.toml"
RECORDS = ROOT / "artifacts/windows-services/recovery-20261107-v1"
CACHE = Path.home() / ".local/share/nevrotick/service-verification-cache-20261107-v1"
TARGETS = ("x86_64-unknown-linux-gnu", "x86_64-pc-windows-msvc")
FEATURES = ["Win32_Foundation", "Win32_Security", "Win32_System_Services", "Win32_System_Threading"]
OLD_FEATURES = ["Win32_Foundation", "Win32_System_Services"]
LOCK_SHA256 = "12f770f2eebb343c19b6c962bda9c367115daad4c88bfad1aebd734bad1e330f"
JOBS = 1
SECONDS = 180
DISK_FLOOR = 2 * 1024 ** 3
SPEC = importlib.util.spec_from_file_location("service_fixture_shared", ROOT / "scripts/portable-analysis.py")
C = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(C)
Incomplete = C.Incomplete


def require(condition, message):
    """Refuse a failed admission regardless of Python assertion settings."""
    if not condition:
        raise Incomplete(message)


def canonical(value):
    """Hash a complete JSON-compatible identity with stable field ordering."""
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def write_new(path, value):
    """Create a fresh bounded-run record, refusing replacement of prior evidence."""
    with path.open("x", encoding="utf-8") as stream:
        json.dump(value, stream, indent=2)
        stream.write("\n")


def index_identity():
    """Observe current staging bytes without taking locks or mutating Git."""
    result = subprocess.run(["git", "rev-parse", "--git-path", "index"], cwd=ROOT,
                            env={"PATH": "/usr/bin:/bin", "GIT_OPTIONAL_LOCKS": "0"},
                            capture_output=True, text=True, check=True, timeout=10)
    path = Path(result.stdout.strip())
    if not path.is_absolute():
        path = ROOT / path
    return {"path": str(path), "sha256": C.PRODUCT.sha256(path)}


def no_ambient_flags():
    """Reject ambient compiler/doc flag replacement before any tool execution."""
    require(not any(key in {"RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTDOCFLAGS", "CARGO_ENCODED_RUSTDOCFLAGS"}
                    or (key.startswith("CARGO_TARGET_") and key.endswith(("_RUSTFLAGS", "_RUSTDOCFLAGS")))
                    for key in os.environ), "Ambient compiler flag overrides are forbidden.")


def compiler_snapshot(stable):
    """Observe the complete current compiler, without an old machine's inventory pin."""
    entries = {}
    scopes = ("bin", "etc", "lib", "libexec", "share")

    def visit(path):
        """Record each real file or contained link; reject special and escaping entries."""
        details = path.lstat()
        relative = path.relative_to(stable).as_posix()
        require(len(entries) < 100000, "Compiler inventory exceeds its finite entry bound.")
        if stat.S_ISLNK(details.st_mode):
            resolved = path.resolve(strict=True)
            require(resolved.is_relative_to(stable)
                    and any(resolved.is_relative_to(stable / scope) for scope in scopes),
                    "Compiler symlink escapes selected payload scopes.")
            entries[relative] = {"kind": "symlink", "target": os.readlink(path)}
        elif stat.S_ISDIR(details.st_mode):
            for child in sorted(path.iterdir()):
                visit(child)
        else:
            require(stat.S_ISREG(details.st_mode), "Special compiler payload entry.")
            entries[relative] = {"kind": "file", "bytes": details.st_size,
                                 "sha256": C.PRODUCT.sha256(path)}

    for scope in scopes:
        require((stable / scope).is_dir(), "Incomplete installed compiler scope.")
        visit(stable / scope)
    return entries


def host_helpers():
    """Select usable public Debian/Ubuntu helpers; record current identity, not legacy pins."""
    selected = {}
    for name in ("bwrap", "prlimit", "cc", "ld", "python3"):
        found = shutil.which(name, path="/usr/bin:/bin")
        require(found is not None, "Required service host helper is missing: " + name)
        path = Path(found)
        resolved = path.resolve(strict=True)
        require(resolved.is_file() and resolved.is_relative_to(Path("/usr"))
                and os.access(resolved, os.X_OK), "Invalid public service helper: " + name)
        selected[name] = {"path": str(path), "resolved": str(resolved),
                          "sha256": C.PRODUCT.sha256(resolved)}
    return selected


def admit_service_tools():
    """Admit pinned upstream tools and observe this host, per the owner's portability amendment."""
    require(os.geteuid() != 0, "Service verification requires an ordinary user.")
    policy = json.loads((ROOT / "policy/tool-manifest.json").read_text())
    stable = Path.home() / (".rustup/toolchains/" + policy["rust"]["channel"]
                           + "-" + policy["rust"]["host"])
    require(stable.is_dir() and stable.resolve(strict=True) == stable,
            "Installed compiler must have a canonical real directory.")
    C.RUNTIME._narrow(stable)
    pins = policy["rust"]["linux_binary_sha256"]
    for name, digest in pins.items():
        C.PRODUCT.ENV.require_admitted_binary(stable / "bin" / name, digest, name)
    extensions = {}
    for name in ("cargo-mutants", "cargo-llvm-cov"):
        specification = next(item for item in policy["tools"] if item["name"] == name)
        path = Path.home() / ".cargo/bin" / name
        C.PRODUCT.ENV.require_admitted_binary(path, specification["binary_sha256"], name)
        extensions[name] = path
    snapshot = compiler_snapshot(stable)
    return {"stable": stable, "readonly_roots": [stable], "extensions": extensions,
            "helpers": host_helpers(), "snapshot": snapshot,
            "inventories": [{"name": "current-service-compiler", "entries": len(snapshot),
                             "sha256": canonical(snapshot)}],
            "manifest_sha256": C.PRODUCT.sha256(ROOT / "policy/tool-manifest.json"),
            "extension_sha256": {name: C.PRODUCT.sha256(path) for name, path in extensions.items()}}


def tool_identity(admitted):
    """Project the observed installation for exact same-candidate graph receipt checks."""
    return {"stable": str(admitted["stable"]), "inventories": admitted["inventories"],
            "helpers": admitted["helpers"], "manifest_sha256": admitted["manifest_sha256"],
            "extensions": {name: {"path": str(path), "sha256": admitted["extension_sha256"][name]}
                           for name, path in admitted["extensions"].items()},
            "admission": "owner-approved current host; upstream tool pins retained"}


def check_service_tools(admitted):
    """Detect changed helper paths or selected executables around every actual command."""
    require(host_helpers() == admitted["helpers"], "Service host helper changed during execution.")
    require(C.PRODUCT.sha256(ROOT / "policy/tool-manifest.json") == admitted["manifest_sha256"],
            "Upstream tool policy changed during execution.")
    policy = json.loads((ROOT / "policy/tool-manifest.json").read_text())
    for name, expected in policy["rust"]["linux_binary_sha256"].items():
        require(C.PRODUCT.sha256(admitted["stable"] / "bin" / name) == expected,
                "Compiler executable changed during execution.")
    for name, path in admitted["extensions"].items():
        require(C.PRODUCT.sha256(path) == admitted["extension_sha256"][name],
                "Service analysis executable changed during execution.")


def run_command(work, command, admitted, vendor, source, *, timeout=180,
                cpu_seconds=None, mutation=False, cwd=None):
    """Use the existing public-host sandbox with the service's unchanged resource custody."""
    require(isinstance(command, list) and command
            and all(isinstance(item, str) and item and "\0" not in item for item in command),
            "Service execution requires a nonempty literal argv.")
    require(type(mutation) is bool, "Mutation resource selection must be explicit.")
    cpu_seconds = timeout if cpu_seconds is None else cpu_seconds
    require(type(timeout) in (int, float) and 0 < timeout <= 3600,
            "Invalid finite service wall allowance.")
    require(type(cpu_seconds) is int and 0 < cpu_seconds <= 3600,
            "Invalid finite service CPU allowance.")
    C.RUNTIME._directory(work, "Service work")
    C.RUNTIME._owned_work(work, ROOT)
    C.RUNTIME._directory(source, "Service source")
    C.RUNTIME._directory(vendor, "Service vendor")
    require(source.is_relative_to(work), "Sealed service source is outside owned work.")
    config = work / "cargo-home/config.toml"
    C.RUNTIME._sealed_inputs(work, [source, config])
    cwd = source if cwd is None else cwd
    C.RUNTIME._directory(cwd, "Service command cwd")
    require(cwd.is_relative_to(source), "Service command cwd escapes the sealed source.")
    check_service_tools(admitted)
    for name in ("home", "tmp", "cargo-home", "target"):
        (work / name).mkdir(mode=0o700, exist_ok=True)
    replacements = {str(admitted["stable"] / "bin" / name): "/toolchain/bin/" + name
                    for name in ("cargo", "rustc", "rustdoc", "rustfmt", "cargo-fmt", "cargo-clippy")}
    replacements.update({str(path): "/tools/" + name for name, path in admitted["extensions"].items()})
    invocation = [replacements.get(item, item) for item in command]
    base = C.PRODUCT.ENV.sandbox_command(admitted["stable"], work, invocation)
    separator = base.index("--")
    args = []
    index = 0
    while index < separator:
        if base[index] == "--setenv":
            index += 3
        elif base[index] == "--chdir":
            index += 2
        else:
            args.append(base[index])
            index += 1
    args[0] = admitted["helpers"]["bwrap"]["path"]
    args.extend(["--ro-bind", str(source), "/work/source", "--ro-bind", str(vendor), "/vendor",
                 "--ro-bind", str(config), "/work/cargo-home/config.toml", "--dir", "/tools"])
    for name, path in admitted["extensions"].items():
        args.extend(["--ro-bind", str(path), "/tools/" + name])
    args.extend(["--chdir", str(Path("/work") / cwd.relative_to(work))])
    environment = {"PATH": "/tools:/toolchain/bin:/usr/bin:/bin", "LANG": "C.UTF-8",
                   "HOME": "/work/home", "TMPDIR": "/work/tmp", "TMP": "/work/tmp", "TEMP": "/work/tmp",
                   "CARGO_HOME": "/work/cargo-home", "CARGO_NET_OFFLINE": "true", "CARGO_INCREMENTAL": "0",
                   "CARGO_BUILD_JOBS": "1", "CARGO_TERM_COLOR": "never", "RUSTUP_AUTO_INSTALL": "0",
                   "RUSTC": "/toolchain/bin/rustc", "RUSTDOC": "/toolchain/bin/rustdoc",
                   "RUST_TEST_THREADS": "1"}
    if not mutation:
        environment["CARGO_TARGET_DIR"] = "/work/target"
    else:
        # Keep a semantic assertion visible if a later custody guard aborts the
        # same libtest process. An abort alone still supplies no mutation credit.
        environment["RUST_TEST_NOCAPTURE"] = "1"
    for key, value in environment.items():
        args.extend(["--setenv", key, value])
    args.extend(["--", admitted["helpers"]["prlimit"]["path"], "--as=8589934592",
                 "--cpu=" + str(cpu_seconds), "--nproc=" + ("512" if mutation else "128"),
                 "--nofile=4096" if mutation else "--nofile=256", "--fsize=268435456", "--", *invocation])
    result = C.PRODUCT.ENV.run(args, cwd=ROOT, timeout=timeout,
                               environment={"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"},
                               stdin=subprocess.DEVNULL)
    check_service_tools(admitted)
    return {**result, "namespace_argv": invocation}


def manifest_admission(policy):
    """Admit only the fixed lab compositions and the two source-card feature additions."""
    manifest = tomllib.loads((ROOT / MANIFEST).read_text())
    fragment = tomllib.loads((ROOT / "policy/rust-workspace.toml").read_text())
    expected = {
        "package": {"name": "nevrotick-windows-service-fixture", "version": "0.0.0", "edition": "2024",
                    "rust-version": "1.99.1", "license": "GPL-2.0-only",
                    "repository": "https://github.com/DavidOsipov/NevroTick", "publish": False, "build": False,
                    "description": "Inert source-only S0.2 laboratory Windows service hosts"},
        "workspace": {"resolver": "3", "lints": fragment["workspace"]["lints"]},
        "dependencies": {"windows-platform": {"version": "=0.0.0", "path": "../../crates/windows-platform"}},
        "target": {"cfg(windows)": {"dependencies": {"windows-sys": {
            "version": "=0.61.2", "default-features": False, "features": FEATURES}}}},
        "bin": [{"name": "nevrotick-lab-s02-engine", "path": "src/bin/engine.rs"},
                {"name": "nevrotick-lab-s02-broker", "path": "src/bin/broker.rs"}],
        "lints": {"workspace": True},
        "profile": {key: fragment["profile"][key] for key in ("release", "verify-release")},
    }
    require(manifest == expected, "Standalone fixture manifest differs from its fixed source card.")
    lock_path = ROOT / TOOL / "Cargo.lock"
    require(C.PRODUCT.sha256(lock_path) == LOCK_SHA256, "Standalone registry lock changed.")
    lock = tomllib.loads(lock_path.read_text())
    expected_ids = set(policy["rows"][TARGETS[0]]["package_scopes"]["windows-platform"]["features"])
    expected_ids.update({"windows-link@0.2.1", "windows-sys@0.61.2"})
    records = {p["name"] + "@" + p["version"]: p for p in policy["packages"]}
    external = {}
    local = {}
    for package in lock["package"]:
        key = package["name"] + "@" + package["version"]
        selected = external if "source" in package else local
        require(key not in selected, "Duplicate standalone lock identity.")
        selected[key] = package
        if "source" in package:
            require(key in expected_ids and package["source"] == C.PRODUCT.SLINT.REGISTRY
                    and package["checksum"] == records[key]["checksum"], "Unadmitted standalone registry bytes.")
    require(set(external) == expected_ids
            and set(local) == {"windows-platform@0.0.0", "nevrotick-windows-service-fixture@0.0.0"},
            "Standalone lock closure differs from the fixed local consumers.")
    return {"before_features": OLD_FEATURES, "after_features": FEATURES,
            "added_features": sorted(set(FEATURES) - set(OLD_FEATURES)),
            "lock_sha256": LOCK_SHA256, "external_packages": sorted(expected_ids)}


def source_identity():
    """Freeze all admitted product bytes plus the complete standalone owner and runner inputs."""
    policy, identity = C.PRODUCT.admit_product(ROOT)
    identity.update({f"{TOOL}/{name}": digest for name, digest in C.PRODUCT.inventory(ROOT / TOOL).items()})
    for name in ("scripts/portable-analysis.py", "scripts/check-analysis.py", "scripts/analysis-runtime.py",
                 "scripts/analysis-controls.py", "scripts/prepare-product.py", "policy/analysis-tools.json",
                 "policy/rust-workspace.toml"):
        identity[name] = C.PRODUCT.sha256(ROOT / name)
    return policy, identity


def expected_contexts(policy, target):
    """Extend the already reviewed portable scope with the exact Windows binding closure."""
    row = policy["rows"][target]["package_scopes"]["windows-platform"]
    expected = copy.deepcopy({**row["features"], **row["local_features"]})
    expected["nevrotick-windows-service-fixture@0.0.0"] = {"target": [[]]}
    if target == TARGETS[1]:
        expected["windows-link@0.2.1"] = {"target": [[]]}
        expected["windows-sys@0.61.2"] = {"target": [["Win32", "Win32_Foundation", "Win32_Security",
                                                           "Win32_System", "Win32_System_Services",
                                                           "Win32_System_Threading"]]}
    return expected


def local_targets(package):
    """Refuse added native/host executables, tests, examples or build targets in either local owner."""
    if package["name"] == "nevrotick-windows-service-fixture":
        owner = TOOL
        expected = {
            "nevrotick_windows_service_fixture": (["lib"], ["lib"], "src/lib.rs"),
            "nevrotick-lab-s02-engine": (["bin"], ["bin"], "src/bin/engine.rs"),
            "nevrotick-lab-s02-broker": (["bin"], ["bin"], "src/bin/broker.rs"),
            "entrypoints": (["test"], ["bin"], "tests/entrypoints.rs"),
        }
    else:
        require(package["name"] == "windows-platform", "Unexpected local target owner.")
        owner = "crates/windows-platform"
        expected = {"windows_platform": (["lib"], ["lib"], "src/lib.rs")}
        expected.update({name: (["test"], ["bin"], f"tests/{name}.rs")
                         for name in ("service", "scm", "privilege_observation", "privilege_observation_contracts")})
    expected = {name: (kind, crate_types, f"/work/source/{owner}/{relative}")
                for name, (kind, crate_types, relative) in expected.items()}
    actual = {}
    for target in package["targets"]:
        require(target["name"] not in actual and not target.get("required-features"), "Duplicate or gated local target.")
        actual[target["name"]] = (target["kind"], target["crate_types"], target["src_path"])
    require(actual == expected, "Local metadata target set differs from the two fixed compositions and declared tests.")


def graph_row(policy, target, metadata_output, tree_output):
    """Require exact registry provenance, targets and normal/build feature contexts before compilation."""
    data = C.ANALYSIS.decode(metadata_output.encode("utf-8"))
    nodes = {node["id"]: node for node in data["resolve"]["nodes"]}
    packages = {package["id"]: package for package in data["packages"]}
    require(len(packages) == len(data["packages"]) and len(nodes) == len(data["resolve"]["nodes"])
            and set(packages) == set(nodes), "Malformed metadata package/node closure.")
    records = {p["name"] + "@" + p["version"]: p for p in policy["packages"]}
    expected = expected_contexts(policy, target)
    seen_metadata = set()
    for key, package in packages.items():
        identity = package["name"] + "@" + package["version"]
        require(identity in expected and identity not in seen_metadata, "Unexpected metadata package.")
        seen_metadata.add(identity)
        if package["source"] is None:
            owner = TOOL if package["name"] == "nevrotick-windows-service-fixture" else "crates/windows-platform"
            require(package["manifest_path"] == f"/work/source/{owner}/Cargo.toml"
                    and package["license"] == "GPL-2.0-only"
                    and not any("custom-build" in item["kind"] for item in package["targets"]),
                    "Unadmitted local metadata source, licence or build script.")
            local_targets(package)
        else:
            record = records[identity]
            require(package["source"] == C.PRODUCT.SLINT.REGISTRY and package["license"] == record["license"]
                    and package.get("links") == record["links"]
                    and any("custom-build" in item["kind"] for item in package["targets"]) == record["build_script"]
                    and any("proc-macro" in item["kind"] for item in package["targets"]) == record["proc_macro"],
                    "Registry execution metadata differs from admitted provenance.")
        union = sorted({feature for sets in expected[identity].values() for features in sets for feature in features})
        require(nodes[key]["features"] == union, "Metadata features differ from the fixed scope.")
    require(seen_metadata == set(expected), "Metadata omits a selected package.")
    contexts, stack, sections = {}, {}, {}
    roots = 0
    for line in tree_output.splitlines():
        marker = re.fullmatch(r"((?:\|   |    )*)\[build-dependencies\]", line)
        if marker:
            depth = len(marker[1]) // 4
            require(depth in stack, "Orphan build-dependency marker.")
            sections[depth] = True
            continue
        match = re.fullmatch(r"((?:(?:\|   |    )*(?:\|-- |`-- ))?)([a-zA-Z0-9_-]+) v([^ |]+)([^|]*)\|([^\r\n]*)", line)
        require(match is not None, "Malformed complete Cargo feature tree.")
        prefix, name, version, annotation, text = match.groups()
        depth, identity = len(prefix) // 4, name + "@" + version
        require(identity in expected and depth <= 128, "Unexpected tree identity or depth.")
        local = identity.endswith("@0.0.0")
        macro = not local and records[identity]["proc_macro"]
        if local:
            owner = TOOL if name == "nevrotick-windows-service-fixture" else "crates/windows-platform"
            require(annotation == f" (/work/source/{owner})", "Unexpected local tree source.")
        else:
            require(annotation == (" (proc-macro)" if macro else ""), "Unexpected registry tree annotation.")
        features = text.split(",") if text else []
        require(features == sorted(set(features)), "Tree features are not a canonical set.")
        if depth == 0:
            roots += 1
            require(identity == "nevrotick-windows-service-fixture@0.0.0", "Unexpected tree root.")
            role = "target"
        else:
            require(depth - 1 in stack, "Tree has an orphan dependency.")
            role = "host" if macro or sections.get(depth - 1) or stack[depth - 1] == "host" else "target"
        for old_depth in list(stack):
            if old_depth >= depth:
                del stack[old_depth]
                sections.pop(old_depth, None)
        stack[depth] = role
        observed = contexts.setdefault(identity, {}).setdefault(role, [])
        if features not in observed:
            observed.append(features)
            observed.sort()
    require(roots == 1 and contexts == expected, "Actual normal/build feature contexts differ from admission.")
    return {"target": target, "contexts": contexts, "external_packages": sorted(set(contexts) & set(records))}


def graph_commands(cargo):
    """Return the four fixed graph observations for the selected admitted compiler."""
    commands = []
    for target in TARGETS:
        commands.extend([[cargo, "metadata", "--manifest-path", "/work/source/" + MANIFEST,
                          "--frozen", "--offline", "--format-version", "1", "--filter-platform", target],
                         [cargo, "tree", "--manifest-path", "/work/source/" + MANIFEST,
                          "--frozen", "--offline", "--target", target, "--edges", "normal,build",
                          "--charset", "ascii", "--format", "{p}|{f}", "--no-dedupe"]])
    return commands


def receive_graph(graph, binding, policy, cargo):
    """Recheck received raw observations as well as their exact current context before compilation."""
    require(graph.get("status") == "passed" and graph.get("lane") == "graph"
            and graph.get("bookends_passed") is True and graph.get("index_bytes_unchanged") is True
            and not graph.get("custody_error") and graph.get("graph_binding") == binding,
            "Graph receipt does not admit these exact source/configuration/tool/vendor bytes.")
    commands = graph.get("commands")
    require(type(commands) is list and len(commands) == 4, "Graph receipt requires four actual command results.")
    for observed, expected in zip(commands, graph_commands(cargo), strict=True):
        require(type(observed) is dict and observed.get("argv") == expected
                and type(observed.get("exit_code")) is int and observed["exit_code"] == 0
                and "failure" in observed and observed["failure"] is None
                and type(observed.get("output")) is str,
                "Graph command identity or successful bounded execution is missing.")
    rows = [graph_row(policy, target, commands[i * 2]["output"], commands[i * 2 + 1]["output"])
            for i, target in enumerate(TARGETS)]
    require(graph.get("graph_rows") == rows, "Graph receipt rows differ from the revalidated raw observations.")


def lane_commands(lane, cargo):
    """Select literal finite commands; caller input cannot inject flags or executables."""
    tool = ["--manifest-path", "/work/source/" + MANIFEST]
    frozen = ["--frozen", "--offline"]
    tests = ["--", "--test-threads=1"]
    commands = {
        "platform-test": [[cargo, "test", "-p", "windows-platform", *frozen, "--all-targets", *tests]],
        "platform-optimized": [[cargo, "test", "-p", "windows-platform", *frozen, "--release", "--all-targets", *tests]],
        "fixture-test": [[cargo, "test", *tool, *frozen, "--all-targets", *tests]],
        "fixture-admission-control": [[cargo, "test", *tool, *frozen, "--all-targets",
                                       "real_host_engine_requires_absence_while_broker_observation_is_descriptive", *tests]],
        "fixture-role-control": [[cargo, "test", *tool, *frozen, "--all-targets",
                                  "both_fixed_roles_start_and_stop_after_a_real_notification", *tests]],
        "fixture-optimized": [[cargo, "test", *tool, *frozen, "--release", "--all-targets", *tests]],
        "platform-clippy": [[cargo, "clippy", "-p", "windows-platform", *frozen, "--all-targets", "--", "-D", "warnings"]],
        "fixture-clippy": [[cargo, "clippy", *tool, *frozen, "--all-targets", "--", "-D", "warnings"]],
        "windows-clippy": [[cargo, "clippy", *tool, *frozen, "--all-targets", "--target", TARGETS[1], "--", "-D", "warnings"]],
        "platform-doc": [[cargo, "test", "-p", "windows-platform", *frozen, "--doc", *tests]],
        "fixture-doc": [[cargo, "test", *tool, *frozen, "--doc", *tests]],
        "fixture-rustdoc": [[cargo, "doc", *tool, *frozen, "--no-deps", "--document-private-items"]],
        "fixture-fmt": [[cargo, "fmt", *tool, "--all", "--", "--check"]],
        "platform-fmt": [[cargo, "fmt", "--package", "windows-platform", "--", "--check"]],
    }
    require(lane in commands, "Unknown source verification lane.")
    return commands[lane]


def run(args):
    """Freeze one current epoch, retain all raw outcomes, and publish success only after bookends."""
    no_ambient_flags()
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    require(re.fullmatch(r"[a-z0-9][a-z0-9-]{0,63}", args.label) is not None, "Invalid fresh record label.")
    require(RECORDS.is_dir() and not RECORDS.is_symlink(), "Main-repository record directory is missing.")
    destination = RECORDS / (args.label + ".json")
    require(not destination.exists(), "A prior raw record must never be replaced.")
    require(shutil.disk_usage(ROOT).free >= DISK_FLOOR, "Less than 2GiB free before admission.")
    CACHE.mkdir(mode=0o700, parents=True, exist_ok=True)
    require(CACHE.resolve() == CACHE and CACHE.stat().st_uid == os.geteuid()
            and CACHE.stat().st_mode & 0o077 == 0, "Derived cache must be canonical, owned and private.")
    writer = os.open(CACHE / "writer.lock", os.O_CREAT | os.O_RDWR | os.O_CLOEXEC | os.O_NOFOLLOW, 0o600)
    try:
        fcntl.flock(writer, fcntl.LOCK_EX | fcntl.LOCK_NB)
    except BlockingIOError:
        os.close(writer)
        raise Incomplete("Another service qualification writer is active.") from None
    work = CACHE / args.label
    work.mkdir(mode=0o700)
    record = {"schema_version": 1, "scope": "source-only inert service fixture", "lane": args.lane,
              "status": "incomplete", "started_unix": time.time(), "commands": [],
              "bookends_passed": False, "index_bytes_unchanged": False, "work": str(work),
              "limits": {"jobs": JOBS, "test_threads": 1, "wall_seconds": SECONDS,
                         "cpu_seconds": SECONDS, "processes": 128, "core_bytes": 0,
                         "disk_floor_bytes": DISK_FLOOR}, "native_execution": False}
    graph_receipt = None
    try:
        policy, identity = source_identity()
        record["source_sha256"] = identity
        record["candidate_sha256"] = canonical(identity)
        record["recipe_sha256"] = identity[SELF]
        record["index_before"] = index_identity()
        record["feature_delta"] = manifest_admission(policy)
        admitted = admit_service_tools()
        # Selection supplies no trust: the complete chosen directory must still
        # match every current reviewed package byte and checksum map below.
        vendor = (args.vendor_dir or Path(json.loads(C.PRODUCT.POINTER.read_text())["vendor_dir"])).resolve(strict=True)
        vendor_identity = C.PRODUCT.admit_vendor(vendor, policy)
        record["vendor"] = {"path": str(vendor), "inventory_sha256": canonical(vendor_identity), "files": len(vendor_identity)}
        record["tools"] = tool_identity(admitted)
        source = work / "source"
        source.mkdir(mode=0o700)
        for name in identity:
            target = source / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        require(C.PRODUCT.inventory(source) == identity, "Derived source copy is incomplete.")
        (work / "cargo-home").mkdir(mode=0o700)
        config = work / "cargo-home/config.toml"
        config.write_text('[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory="/vendor"\n')
        record["config_sha256"] = C.PRODUCT.sha256(config)
        cargo = "/toolchain/bin/cargo"
        graph_binding = {key: record[key] for key in ("candidate_sha256", "recipe_sha256", "config_sha256", "feature_delta", "vendor", "tools")}
        if args.lane not in {"ready", "graph"}:
            require(args.graph_receipt is not None, "Compilation requires an explicit current graph receipt.")
            graph_receipt = args.graph_receipt.resolve(strict=True)
            require(graph_receipt.parent == RECORDS and not args.graph_receipt.is_symlink(), "Graph receipt must be a main-repository regular record.")
            graph = C.read_analysis_json(graph_receipt)
            receive_graph(graph, graph_binding, policy, cargo)
            record["received_graph_sha256"] = C.PRODUCT.sha256(graph_receipt)
            record["received_graph_path"] = str(graph_receipt)
        record["graph_binding"] = graph_binding

        def check():
            """Recheck the full current candidate, sealed copies/config and this job's staging epoch."""
            require(source_identity()[1] == identity, "Root source or complete inventory changed during the job.")
            require(C.PRODUCT.inventory(source) == identity, "Sealed derived source changed during the job.")
            require(C.PRODUCT.sha256(config) == record["config_sha256"], "Sealed Cargo configuration changed.")
            require(index_identity() == record["index_before"], "Staging bytes changed during this job.")
            if graph_receipt:
                require(C.PRODUCT.sha256(graph_receipt) == record["received_graph_sha256"], "Received graph record changed during the job.")

        if args.lane == "ready":
            commands = [["/toolchain/bin/rustc", "--version", "--verbose"]]
        elif args.lane == "graph":
            commands = graph_commands(cargo)
        else:
            commands = lane_commands(args.lane, cargo)
        for command in commands:
            check()
            require(shutil.disk_usage(ROOT).free >= DISK_FLOOR, "Less than 2GiB free before execution.")
            result = run_command(work, command, admitted, vendor, source,
                                 timeout=SECONDS, cpu_seconds=SECONDS)
            record["commands"].append({"argv": command, **result})
            check()
            require(result.get("failure") is None and type(result.get("exit_code")) is int,
                    "Execution lost wall/output/process custody.")
            if result["exit_code"] != 0:
                break
        succeeded = len(record["commands"]) == len(commands) and all(
            command["exit_code"] == 0 for command in record["commands"])
        if args.lane == "graph" and succeeded:
            record["graph_rows"] = [graph_row(policy, target, record["commands"][i * 2]["output"],
                                              record["commands"][i * 2 + 1]["output"])
                                    for i, target in enumerate(TARGETS)]
        check()
        require(C.PRODUCT.admit_vendor(vendor, policy) == vendor_identity, "Vendor content changed during the job.")
        after = admit_service_tools()
        require(tool_identity(after) == tool_identity(admitted), "Tool payload changed during the job.")
        check()
        record["index_after"] = index_identity()
        record["index_bytes_unchanged"] = True
        record["bookends_passed"] = True
        # A finite observed test/formatter/compiler failure remains a failed
        # source lane with healthy custody, rather than a fabricated harness
        # failure. Only successful complete graph commands admit compilation.
        record["status"] = "passed" if succeeded else "failed"
    except Exception as error:
        record["custody_error"] = str(error)
        raise
    finally:
        record["finished_unix"] = time.time()
        try:
            write_new(destination, record)
            print(json.dumps({"record": str(destination), "status": record["status"], "commands": len(record["commands"])}), flush=True)
        finally:
            os.close(writer)
    return record["status"]


def main():
    """Parse only declared source lanes and fresh record labels; return nonzero on admission failure."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("lane", choices=("ready", "graph", "platform-test", "platform-optimized", "fixture-test",
                        "fixture-admission-control", "fixture-role-control", "fixture-optimized", "platform-clippy", "fixture-clippy", "windows-clippy", "platform-doc",
                        "fixture-doc", "fixture-rustdoc", "fixture-fmt", "platform-fmt"))
    parser.add_argument("--label", required=True)
    parser.add_argument("--graph-receipt", type=Path)
    parser.add_argument("--vendor-dir", type=Path)
    args = parser.parse_args()
    try:
        status = run(args)
    except (Incomplete, C.PRODUCT.ENV.Incomplete, OSError, ValueError, subprocess.SubprocessError) as error:
        print(str(error), file=sys.stderr)
        return 1
    return 0 if status == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
