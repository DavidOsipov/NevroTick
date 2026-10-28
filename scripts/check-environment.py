#!/usr/bin/env python3
"""Check bootstrap tools; optionally run an isolated, dependency-free Rust fixture.

This is DEV-01 preparation, not a product/Windows-native/release qualification.
The smoke lane never falls back from bubblewrap to executing Cargo on the host.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import selectors
import shutil
import signal
import socket
import subprocess
import sys
import tempfile
import time
import tomllib


WINDOWS_TARGET = "x86_64-pc-windows-msvc"
MSVC_FLAGS = [
    "-C", "control-flow-guard=yes", "-C", "link-arg=/GUARD:CF",
    "-C", "link-arg=/DYNAMICBASE", "-C", "link-arg=/HIGHENTROPYVA",
    "-C", "link-arg=/NXCOMPAT", "-C", "link-arg=/WX",
]
REQUIRED_COMPONENTS = {"clippy", "rustfmt", "rust-analyzer", "rust-src", "llvm-tools-preview"}
FORBIDDEN_ENV = {
    "RUSTUP_TOOLCHAIN", "RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "RUSTDOCFLAGS",
    "CARGO_ENCODED_RUSTDOCFLAGS", "RUSTC_BOOTSTRAP", "RUSTC", "RUSTDOC",
    "RUSTC_WRAPPER", "RUSTC_WORKSPACE_WRAPPER",
    "CARGO_MAKEFLAGS", "CC", "CXX", "AR", "LD", "CFLAGS", "CXXFLAGS", "LDFLAGS",
    "RUSTUP_DIST_SERVER", "RUSTUP_UPDATE_ROOT",
    "RUSTFMT", "CLIPPY_ARGS", "CLIPPY_CONF_DIR", "LD_PRELOAD", "LD_LIBRARY_PATH", "LD_AUDIT",
    "CARGO_INCREMENTAL", "LLVM_COV", "LLVM_PROFDATA",
}
FORBIDDEN_PREFIXES = (
    "CARGO_TARGET_", "CARGO_PROFILE_", "CARGO_BUILD_", "CARGO_ALIAS_", "CC_", "CXX_",
    "NEXTEST_", "CARGO_MUTANTS_", "CARGO_LLVM_COV_",
)
MAX_OUTPUT = 2 * 1024 * 1024


class Incomplete(RuntimeError):
    """A necessary input/control is absent or fails; never a successful skip."""


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def external_receipt(path: Path, root: Path) -> Path:
    """Resolve output links and keep evidence from overwriting checkout inputs."""
    destination = path.expanduser().resolve()
    if destination.is_relative_to(root.resolve()):
        raise Incomplete("Evidence receipts must resolve outside the checkout.")
    if destination.exists():
        if not destination.is_file() or destination.stat().st_nlink != 1:
            raise Incomplete("Evidence receipt must be a regular file without hard links.")
    return destination


def overridden_names(environment: dict[str, str]) -> list[str]:
    """Only return variable names; ambient values can contain credentials."""
    return sorted(name for name in environment if name in FORBIDDEN_ENV or name.startswith(FORBIDDEN_PREFIXES))


def run(command: list[str], *, cwd: Path, timeout: float = 30, environment=None) -> dict:
    """Bound wall time/output and reject surviving children of the owned group."""
    process = subprocess.Popen(
        command, cwd=cwd, env=environment, stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT, start_new_session=True,
    )
    output = bytearray()
    failure = None
    try:
        with selectors.DefaultSelector() as selector:
            selector.register(process.stdout, selectors.EVENT_READ)
            deadline = time.monotonic() + timeout
            while selector.get_map() or process.poll() is None:
                if time.monotonic() >= deadline:
                    failure = f"wall timeout ({timeout}s)"
                    break
                for key, _ in selector.select(min(0.25, max(0, deadline - time.monotonic()))):
                    chunk = os.read(key.fileobj.fileno(), 65536)
                    if not chunk:
                        selector.unregister(key.fileobj)
                    elif len(output) + len(chunk) > MAX_OUTPUT:
                        failure = "output budget exceeded"
                        break
                    else:
                        output.extend(chunk)
                if failure:
                    break
        if failure:
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
        process.wait(timeout=5)
        if failure is None:
            try:
                os.killpg(process.pid, 0)
            except ProcessLookupError:
                pass
            else:
                failure = "descendant processes outlived command"
    finally:
        # A reaped leader does not establish completion of its process group.
        # Close custody on success and failure, including descendants that have
        # detached their standard streams. Escaped sessions remain outside this
        # helper's guarantee; the fixed smoke fixture also has a PID namespace.
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        if process.poll() is None:
            process.wait(timeout=5)
        process.stdout.close()
    return {"command": command, "exit_code": process.returncode,
            "output": output.decode("utf-8", errors="replace"), "failure": failure}


def require_ok(result: dict) -> str:
    if result["failure"] or result["exit_code"] != 0:
        raise Incomplete(f"Command failed: {result['command'][0]}: "
                         f"{result['failure'] or result['exit_code']}\n{result['output'][-3000:]}")
    return result["output"].strip()


def require_test_count(output: str, expected: int, lane: str) -> None:
    if not re.search(rf"test result: ok\. {expected} passed; 0 failed; 0 ignored;", output):
        raise Incomplete(f"{lane}: expected {expected} actual passing tests, not an empty selection.")


def require_admitted_compiler(channel: str, compiler_info: str, policy: dict) -> None:
    admitted = policy["rust"]
    fields = dict(line.split(": ", 1) for line in compiler_info.splitlines() if ": " in line)
    if channel != admitted["channel"]:
        raise Incomplete("Toolchain file and reviewed tool manifest disagree.")
    if fields.get("release") != channel or fields.get("commit-hash") != admitted["compiler_commit"]:
        raise Incomplete("Effective compiler release/commit differs from the reviewed tool manifest.")
    if fields.get("LLVM version") != admitted["llvm_version"]:
        raise Incomplete("Effective LLVM differs from the reviewed tool manifest.")


def require_admitted_binary(path: Path, expected: str, label: str) -> str:
    actual = digest(path)
    if actual != expected:
        raise Incomplete(f"{label} executable digest differs from the reviewed tool manifest.")
    return actual


def inspect(root: Path) -> dict:
    if os.name != "posix" or not sys.platform.startswith("linux"):
        raise Incomplete("This checker owns the Linux/WSL lane; use setup-windows.ps1 for native Windows.")
    if os.geteuid() == 0:
        raise Incomplete("Run development checks as an ordinary user, not root.")
    names = overridden_names(dict(os.environ))
    if names:
        raise Incomplete("Unreviewed build overrides are set (names only): " + ", ".join(names))
    toolchain_file = root / "rust-toolchain.toml"
    if not toolchain_file.is_file():
        raise Incomplete("Missing repository rust-toolchain.toml; no floating toolchain is admitted.")
    toolchain = tomllib.loads(toolchain_file.read_text())["toolchain"]
    channel = toolchain.get("channel", "")
    if not re.fullmatch(r"\d+\.\d+\.\d+", channel):
        raise Incomplete("The stable toolchain must have an exact version pin.")
    if toolchain.get("profile") != "minimal" or "path" in toolchain:
        raise Incomplete("Expected the admitted minimal, versioned toolchain.")
    if not REQUIRED_COMPONENTS.issubset(toolchain.get("components", [])):
        raise Incomplete("The toolchain file omits a required development component.")
    if WINDOWS_TARGET not in toolchain.get("targets", []):
        raise Incomplete("The toolchain file omits the x64 Windows MSVC standard library.")

    config = root / ".cargo/config.toml"
    if not config.is_file():
        raise Incomplete("Missing repository target-specific Cargo configuration.")
    parsed = tomllib.loads(config.read_text())
    if parsed != {"target": {WINDOWS_TARGET: {"rustflags": MSVC_FLAGS}}}:
        raise Incomplete("Cargo configuration differs from the admitted target-only Windows flags.")
    policy_file = root / "policy/tool-manifest.json"
    if not policy_file.is_file():
        raise Incomplete("Missing reviewed policy/tool-manifest.json.")
    policy = json.loads(policy_file.read_text())
    if channel != policy["rust"]["channel"]:
        raise Incomplete("Toolchain file and reviewed tool manifest disagree.")
    ambient = []
    cargo_home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo")).expanduser()
    for directory in [*root.parents, cargo_home.parent]:
        for name in ("config", "config.toml"):
            path = directory / ".cargo" / name
            if path.is_file() and path.resolve() != config.resolve():
                ambient.append({"path": str(path), "sha256": digest(path)})
    for path in (root / ".cargo/config", cargo_home / "config", cargo_home / "config.toml"):
        if path.is_file() and path.resolve() != config.resolve():
            item = {"path": str(path), "sha256": digest(path)}
            if item not in ambient:
                ambient.append(item)
    if ambient:
        raise Incomplete("Unreviewed ancestor/home/legacy Cargo configuration: " + json.dumps(ambient))

    rustup = shutil.which("rustup")
    if not rustup:
        raise Incomplete("rustup is missing; this checker installs nothing.")
    require_admitted_binary(Path(rustup).resolve(), policy["rust"]["rustup_linux_sha256"], "rustup")
    installed = require_ok(run([rustup, "toolchain", "list"], cwd=root))
    if not any(line.startswith(channel + "-") for line in installed.splitlines()):
        raise Incomplete(f"Rust {channel} is not installed; bootstrap it before this check.")
    active = require_ok(run([rustup, "show", "active-toolchain"], cwd=root))
    if not active.startswith(channel + "-") or str(toolchain_file) not in active:
        raise Incomplete("Effective toolchain is not selected by this repository file: " + active)
    compiler = Path(require_ok(run([rustup, "which", "--toolchain", channel, "rustc"], cwd=root)))
    toolchain_dir = compiler.parent.parent
    identities = {}
    for executable, args in [("rustc", ["-vV"]), ("cargo", ["-V"]),
                             ("rustfmt", ["--version"]), ("clippy-driver", ["--version"]),
                             ("cargo-clippy", ["--version"]), ("cargo-fmt", ["--version"]),
                             ("rustdoc", ["--version"]),
                             ("rust-analyzer", ["--version"])]:
        path = toolchain_dir / "bin" / executable
        if not path.is_file():
            raise Incomplete(f"Missing toolchain executable: {executable}")
        identity = require_admitted_binary(path, policy["rust"]["linux_binary_sha256"][executable], executable)
        identities[executable] = {"path": str(path), "sha256": identity,
                                  "version": require_ok(run([str(path), *args], cwd=root))}
    compiler_info = identities["rustc"]["version"]
    if f"release: {channel}\n" not in compiler_info + "\n":
        raise Incomplete("Installed compiler does not match the pin.")
    require_admitted_compiler(channel, compiler_info, policy)
    manager_version = require_ok(run([rustup, "--version"], cwd=root))
    if not re.match(r"rustup " + re.escape(policy["rust"]["rustup_version"]) + r"\s", manager_version):
        raise Incomplete("Effective rustup differs from the reviewed tool manifest.")
    target_list = require_ok(run([rustup, "target", "list", "--toolchain", channel, "--installed"], cwd=root))
    components = require_ok(run([rustup, "component", "list", "--toolchain", channel, "--installed"], cwd=root))
    for component in REQUIRED_COMPONENTS:
        stem = "llvm-tools" if component == "llvm-tools-preview" else component
        if not any(line == stem or line.startswith(stem + "-") for line in components.splitlines()):
            raise Incomplete("Required component is not installed: " + component)
    if WINDOWS_TARGET not in target_list.splitlines():
        raise Incomplete("Windows MSVC target is not installed.")
    helpers = {}
    for name in ("bwrap", "prlimit", "cc", "ld", "python3"):
        found = shutil.which(name)
        if found:
            path = Path(found).resolve()
            identity = require_admitted_binary(path, policy["bootstrap_helpers"][name]["sha256"], name)
            helpers[name] = {"path": str(path), "sha256": identity,
                             "version": require_ok(run([str(path), "--version"], cwd=root)).splitlines()[0]}
        else:
            helpers[name] = {"status": "missing"}
    return {"toolchain": channel, "effective_selection": active, "tools": identities,
            "host_helpers": helpers,
            "rustup": manager_version,
            "components": components.splitlines(), "targets": target_list.splitlines(),
            "configuration_sha256": {"rust-toolchain.toml": digest(toolchain_file),
                                      ".cargo/config.toml": digest(config),
                                      "policy/tool-manifest.json": digest(policy_file)},
            "product_manifest_present": (root / "Cargo.toml").is_file()}


FIXTURE_SOURCE = '''#![forbid(unsafe_code)]
/// Advance a tick without wrapping.
///
/// ```
/// assert_eq!(nevrotick_env_smoke::next_tick(41), Some(42));
/// ```
#[must_use]
pub const fn next_tick(tick: u64) -> Option<u64> {
    tick.checked_add(1)
}

#[cfg(test)]
mod tests {
    use super::next_tick;

    #[test]
    fn ordinary_tick() {
        assert_eq!(next_tick(41), Some(42));
    }

    #[test]
    fn overflow_is_rejected() {
        assert_eq!(next_tick(u64::MAX), None);
    }
}
'''


def sandbox_command(toolchain_dir: Path, work: Path, command: list[str]) -> list[str]:
    bwrap, prlimit = shutil.which("bwrap"), shutil.which("prlimit")
    if not bwrap or not prlimit:
        raise Incomplete("--smoke requires bubblewrap and prlimit; host execution fallback is forbidden.")
    args = [bwrap, "--unshare-all", "--die-with-parent",
            "--new-session", "--cap-drop", "ALL", "--clearenv"]
    # Debian/Ubuntu's cc symlink resolves through public alternatives metadata.
    for path in ("/usr", "/bin", "/lib", "/lib64", "/etc/alternatives"):
        if Path(path).exists():
            args.extend(["--ro-bind", path, path])
    if Path("/etc/ld.so.cache").is_file():
        args.extend(["--ro-bind", "/etc/ld.so.cache", "/etc/ld.so.cache"])
    args.extend(["--ro-bind", str(toolchain_dir), "/toolchain", "--bind", str(work), "/work",
                 "--proc", "/proc", "--dev", "/dev", "--tmpfs", "/tmp",
                 "--chdir", "/work/fixture"])
    for key, value in {"PATH": "/toolchain/bin:/usr/bin:/bin", "LANG": "C.UTF-8",
                       "TMPDIR": "/tmp", "CARGO_HOME": "/work/cargo-home",
                       "CARGO_TARGET_DIR": "/work/target", "CARGO_INCREMENTAL": "0"}.items():
        args.extend(["--setenv", key, value])
    # Apply NPROC inside the new user namespace. Applying it before bwrap would
    # charge unrelated host processes/threads owned by this development user.
    return args + ["--", "/usr/bin/prlimit", "--as=4294967296", "--cpu=90", "--nproc=128",
                   "--nofile=256", "--fsize=268435456", "--", *command]


def smoke(root: Path, metadata: dict, evidence: dict) -> dict:
    if shutil.disk_usage(tempfile.gettempdir()).free < 512 * 1024 * 1024:
        raise Incomplete("Insufficient scratch space for the bounded fixture (512 MiB required).")
    toolchain_dir = Path(metadata["tools"]["rustc"]["path"]).parent.parent
    lanes = evidence.setdefault("lanes", [])
    with tempfile.TemporaryDirectory(prefix="nevrotick-env-") as temp:
        scratch = Path(temp)
        work = scratch / "work"
        fixture = work / "fixture"
        (fixture / "src").mkdir(parents=True)
        (fixture / ".cargo").mkdir()
        (work / "cargo-home").mkdir()
        (fixture / ".cargo/config.toml").write_bytes((root / ".cargo/config.toml").read_bytes())
        (fixture / "Cargo.toml").write_text('[package]\nname = "nevrotick-env-smoke"\nversion = "0.0.0"\nedition = "2024"\npublish = false\n', encoding="utf-8")
        (fixture / "Cargo.lock").write_text('version = 4\n\n[[package]]\nname = "nevrotick-env-smoke"\nversion = "0.0.0"\n', encoding="utf-8")
        source = fixture / "src/lib.rs"
        source.write_text(FIXTURE_SOURCE, encoding="utf-8")
        canary = scratch / "host-canary"
        canary.write_text("synthetic boundary control; not a credential", encoding="utf-8")
        with socket.socket() as listener:
            listener.bind(("127.0.0.1", 0))
            listener.listen(2)
            listener.settimeout(2)
            port = listener.getsockname()[1]
            with socket.create_connection(("127.0.0.1", port), timeout=2):
                accepted, _ = listener.accept()
                accepted.close()
            probe = (
                "import os,pathlib,socket; "
                f"assert not pathlib.Path({str(canary)!r}).exists(); "
                "assert 'NEVROTICK_SYNTHETIC_CREDENTIAL' not in os.environ; "
                "assert 'HOME' not in os.environ; "
                "assert not pathlib.Path('/run/docker.sock').exists(); "
                "s=socket.socket();s.settimeout(1)\n"
                f"try: s.connect(('127.0.0.1',{port}))\n"
                "except OSError: print('BOUNDARY_CONTROLS_OK')\n"
                "else: raise SystemExit('network boundary failed')\n"
                "finally: s.close()\n"
            )
            environment = dict(os.environ, NEVROTICK_SYNTHETIC_CREDENTIAL="synthetic-control")
            result = run(sandbox_command(toolchain_dir, work, ["python3", "-c", probe]),
                         cwd=root, environment=environment)
            lanes.append({"lane": "boundary-controls", **result})
            if "BOUNDARY_CONTROLS_OK" not in require_ok(result):
                raise Incomplete("Sandbox control did not execute its observer.")
        commands = [
            ("format", ["cargo", "fmt", "--", "--check"], None),
            ("clippy", ["cargo", "clippy", "--offline", "--locked", "--all-targets", "--jobs", "1",
                         "--", "-D", "warnings", "-D", "clippy::all", "-D", "clippy::pedantic"], None),
            ("unit-tests", ["cargo", "test", "--offline", "--locked", "--lib", "--jobs", "1",
                             "--", "--test-threads=1"], 2),
            ("doctests", ["cargo", "test", "--offline", "--locked", "--doc", "--jobs", "1"], 1),
            ("windows-target-check", ["cargo", "check", "--offline", "--locked", "--all-targets",
                                       "--target", WINDOWS_TARGET, "--jobs", "1"], None),
        ]
        for label, command, expected_tests in commands:
            result = run(sandbox_command(toolchain_dir, work, command), cwd=root, timeout=90)
            lanes.append({"lane": label, **result})
            output = require_ok(result)
            if expected_tests is not None:
                require_test_count(output, expected_tests, label)
        source.write_text(FIXTURE_SOURCE + "\npub unsafe fn prohibited_control() {}\n", encoding="utf-8")
        result = run(sandbox_command(toolchain_dir, work, ["cargo", "check", "--offline", "--locked", "--lib"]), cwd=root)
        lanes.append({"lane": "unsafe-negative-control", **result})
        if result["failure"] or result["exit_code"] == 0 or "declaration of an `unsafe` function" not in result["output"]:
            raise Incomplete("The deliberate unsafe declaration did not fail for the required compiler guard.")
        source.write_text(FIXTURE_SOURCE, encoding="utf-8")
        result = run(sandbox_command(toolchain_dir, work, ["cargo", "check", "--offline", "--locked", "--lib"]), cwd=root)
        lanes.append({"lane": "restored-positive-control", **result})
        require_ok(result)
    evidence.update({"status": "passed", "scope": "std-only fixture; 2 unit tests, 1 doctest, MSVC compile check",
            "native_windows_link_or_execution": "not tested", "product_behavior": "not tested",
            "isolation": {"network_namespace": "isolated", "inherited_environment": "cleared",
                          "host_credentials_and_checkout": "not mounted", "writable_paths": "task scratch, private tmp/dev/proc",
                          "limits": "90s wall/command; per-process 4GiB address space, 90s CPU, 256 open files, 256MiB/file; namespace UID process limit 128; 2MiB output",
                          "limit_caveat": "Per-process rlimits are not a cumulative cgroup quota; this lane admits only the fixed trusted fixture."},
            "lanes": lanes})
    return evidence


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--smoke", action="store_true", help="run the mandatory isolated std-only fixture")
    parser.add_argument("--json", type=Path, help="write evidence including command outputs to this explicit path")
    args = parser.parse_args(argv)
    root = Path(__file__).resolve().parents[1]
    report = {"scope": "NevroTick DEV-01 bootstrap only", "repository": str(root), "status": "incomplete"}
    receipt = None
    try:
        if args.json:
            receipt = external_receipt(args.json, root)
        report["metadata"] = inspect(root)
        if args.smoke:
            report["smoke"] = {"status": "incomplete", "lanes": []}
            smoke(root, report["metadata"], report["smoke"])
        report["status"] = "passed"
    except (Incomplete, OSError, ValueError, KeyError, subprocess.SubprocessError, tomllib.TOMLDecodeError) as error:
        report["error"] = str(error)
    if receipt is not None:
        try:
            external_receipt(args.json, root).write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
        except (Incomplete, OSError, ValueError) as error:
            report["status"] = "incomplete"
            report["error"] = str(error)
    print(f"{report['status'].upper()}: {report['scope']}")
    if "error" in report:
        print(report["error"], file=sys.stderr)
    elif args.smoke:
        print("Isolated fixture: 2 unit tests, 1 doctest; Windows compile check only. No native/product qualification.")
    else:
        print("Tool metadata/configuration only; use --smoke for isolated execution controls.")
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
