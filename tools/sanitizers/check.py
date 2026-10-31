#!/usr/bin/env python3
"""Admit isolated sanitizer payloads; optionally execute fixed detector controls.

No installation, host execution fallback, product build or qualification follows
from admission. No instrumented Rust std or application graph is qualified.
"""
from __future__ import annotations

import argparse
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import selectors
import signal
import stat
import subprocess
import sys
import time

sys.dont_write_bytecode = True
HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
PLATFORM = "x86_64-unknown-linux-gnu"
RUST_LINKER = "/usr/bin/x86_64-linux-gnu-gcc-13"
NATIVE_LINKER = "/usr/bin/x86_64-linux-gnu-ld.bfd"
GCC_COLLECT2 = "/usr/libexec/gcc/x86_64-linux-gnu/13/collect2"
LTO_PLUGIN = "/usr/libexec/gcc/x86_64-linux-gnu/13/liblto_plugin.so"
SYSROOTS = {"tsan": "thread", "msan": "memory"}
TOOLS = ("asan", "lsan", "tsan", "msan", "ubsan")
SOURCES = {name: f"fixtures/{name}.{'cpp' if name == 'ubsan' else 'rs'}" for name in TOOLS}
DIAGNOSTICS = {
    "asan": "ERROR: AddressSanitizer: heap-buffer-overflow",
    "lsan": "ERROR: LeakSanitizer: detected memory leaks",
    "tsan": "WARNING: ThreadSanitizer: data race",
    "msan": "WARNING: MemorySanitizer: use-of-uninitialized-value",
    "ubsan": "runtime error: signed integer overflow",
}
HEX = re.compile(r"[0-9a-f]{64}\Z")
SETUP_FAILURE = re.compile(
    r"failed to (?:mmap|map)|unable to map|can(?:not| not) mmap|unexpected memory mapping|"
    r"out of memory|runtime does not come first|error while loading shared libraries|"
    r"permission denied|no such file or directory|panicked at|fatal runtime error|"
    r"(?:Thread|Memory|Address)Sanitizer: (?:CHECK failed|DEADLYSIGNAL)|"
    r"LeakSanitizer has encountered a fatal error", re.I)
MAX_JSON = 2 * 1024 * 1024
MAX_OUTPUT = 2 * 1024 * 1024


class Incomplete(Exception):
    """Admission, custody or detector evidence is insufficient."""


def canonical(path):
    """Accept existing absolute paths without redirected ancestors."""
    path = Path(path)
    if not path.is_absolute() or path != path.resolve(strict=True):
        raise Incomplete("Expected a canonical absolute path without symlinks.")
    return path


def regular(path, *, private=False):
    """Reject special files and redirected paths; protect the private pointer."""
    path = canonical(path)
    details = path.lstat()
    if not stat.S_ISREG(details.st_mode):
        raise Incomplete("Expected a regular file.")
    if private and (details.st_uid != os.geteuid() or details.st_nlink != 1
                    or stat.S_IMODE(details.st_mode) != 0o600):
        raise Incomplete("Installation pointer must be owned, unlinked and mode 0600.")
    return details


def file_identity(details):
    """Reading may update atime; content/custody changes must still be rejected."""
    return (details.st_dev, details.st_ino, details.st_mode, details.st_nlink,
            details.st_uid, details.st_size, details.st_mtime_ns, details.st_ctime_ns)


def read_once(path, *, private=False, maximum=MAX_JSON):
    """Read bounded regular bytes without opening a substituted symlink/FIFO."""
    before = regular(path, private=private)
    descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(descriptor, "rb") as stream:
        opened = os.fstat(stream.fileno())
        if (opened.st_dev, opened.st_ino, opened.st_mode, opened.st_nlink) != (
                before.st_dev, before.st_ino, before.st_mode, before.st_nlink):
            raise Incomplete("Input changed while opening it.")
        data = stream.read(maximum + 1)
    after = regular(path, private=private)
    if file_identity(before) != file_identity(after) or len(data) > maximum:
        raise Incomplete("Input changed or exceeded its byte budget.")
    return data


def digest(path):
    """Hash a stable regular payload without loading it into memory."""
    before = regular(path)
    value = hashlib.sha256()
    descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(descriptor, "rb") as stream:
        opened = os.fstat(stream.fileno())
        if (opened.st_dev, opened.st_ino) != (before.st_dev, before.st_ino):
            raise Incomplete("Payload changed while opening it.")
        for part in iter(lambda: stream.read(1024 * 1024), b""):
            value.update(part)
    if file_identity(before) != file_identity(regular(path)):
        raise Incomplete("Payload changed while hashing it.")
    return value.hexdigest()


def decode(data):
    """Decode bounded strict JSON, rejecting duplicates and nonfinite values."""
    def pairs(items):
        result = {}
        for key, value in items:
            if key in result:
                raise Incomplete("Duplicate JSON field.")
            result[key] = value
        return result

    def constant(_value):
        raise Incomplete("Nonfinite JSON value.")

    return json.loads(data, object_pairs_hook=pairs, parse_constant=constant)


def relative(value):
    """Accept only normalized, nonempty relative POSIX paths."""
    if (not isinstance(value, str) or not value or "\\" in value or "\0" in value
            or PurePosixPath(value).is_absolute() or str(PurePosixPath(value)) != value
            or any(part in (".", "..") for part in PurePosixPath(value).parts)):
        raise Incomplete("Invalid relative policy path.")
    return value


def checksum(value):
    if not isinstance(value, str) or not HEX.fullmatch(value):
        raise Incomplete("Expected a lowercase SHA256.")
    return value


def validate_policy(policy):
    """Keep the tool, source, helper and configuration surface fixed."""
    fields = {"schema_version", "platform", "payloads", "rust_compiler", "native_compiler",
              "symbolizer", "tools", "inputs", "helpers", "sysroots"}
    if (not isinstance(policy, dict) or set(policy) != fields
            or type(policy["schema_version"]) is not int or policy["schema_version"] != 1
            or policy["platform"] != PLATFORM or policy["tools"] != SOURCES
            or policy["sysroots"] != SYSROOTS
            or not isinstance(policy["inputs"], dict) or set(policy["inputs"]) != set(SOURCES.values())):
        raise Incomplete("Unknown sanitizer policy schema, platform or tool/source set.")
    for value in policy["inputs"].values():
        checksum(value)
    payloads = policy["payloads"]
    if not isinstance(payloads, list) or not 1 <= len(payloads) <= 20000:
        raise Incomplete("Missing or excessive payload inventory.")
    seen = set()
    for item in payloads:
        if (not isinstance(item, dict) or set(item) != {"root", "path", "sha256"}
                or item["root"] not in ("rust", "clang", "sysroots")):
            raise Incomplete("Unknown payload schema/root.")
        key = (item["root"], relative(item["path"]))
        if key in seen:
            raise Incomplete("Duplicate payload path.")
        seen.add(key)
        checksum(item["sha256"])
    for field, root in (("rust_compiler", "rust"), ("native_compiler", "clang"), ("symbolizer", "clang")):
        if (root, relative(policy[field])) not in seen:
            raise Incomplete("Compiler/symbolizer lacks a pinned payload.")
    for tool, folder in SYSROOTS.items():
        prefix = f"{folder}/lib/rustlib/{PLATFORM}/lib/"
        paths = {path for root, path in seen if root == "sysroots"}
        for library in ("core", "compiler_builtins"):
            pattern = re.escape(prefix) + "lib" + library + r"-[0-9a-f]+\.rlib"
            libraries = [path for path in paths if re.fullmatch(pattern, path)]
            if len(libraries) != 1:
                raise Incomplete("Sanitizer sysroot requires one pinned core/compiler_builtins rlib.")
            metadata = libraries[0].removesuffix(".rlib") + ".rmeta"
            metadata_pattern = re.escape(prefix) + "lib" + library + r"-[0-9a-f]+\.rmeta"
            if {path for path in paths if re.fullmatch(metadata_pattern, path)} != {metadata}:
                raise Incomplete("Sanitizer sysroot requires exactly the matching pinned rmeta.")
        if prefix + f"librustc-nightly_rt.{tool}.a" not in paths:
            raise Incomplete("Sanitizer sysroot lacks its matching pinned runtime.")
    helpers = policy["helpers"]
    if not isinstance(helpers, dict) or set(helpers) != {"bwrap", "prlimit"}:
        raise Incomplete("Unknown helper set.")
    for name, item in helpers.items():
        if (not isinstance(item, dict) or set(item) != {"path", "sha256"}
                or item["path"] != "/usr/bin/" + name):
            raise Incomplete("Unexpected sandbox helper path/schema.")
        checksum(item["sha256"])


class Admission:
    """Recheck declared payloads, policy, pointer and fixture bytes per command."""

    def __init__(self, policy_path, installation, *, root=ROOT):
        self.root = canonical(root)
        self.policy_path = canonical(policy_path)
        self.installation = Path(installation)
        self.policy_bytes = read_once(self.policy_path)
        self.pointer_bytes = read_once(self.installation, private=True, maximum=16384)
        self.policy = decode(self.policy_bytes)
        validate_policy(self.policy)
        pointer = decode(self.pointer_bytes)
        if (not isinstance(pointer, dict) or set(pointer) != {"schema_version", "roots"}
                or type(pointer["schema_version"]) is not int or pointer["schema_version"] != 1
                or not isinstance(pointer["roots"], dict) or set(pointer["roots"]) != {"rust", "clang", "sysroots"}
                or not all(isinstance(value, str) for value in pointer["roots"].values())):
            raise Incomplete("Unknown installation pointer schema.")
        self.roots = {key: canonical(value) for key, value in pointer["roots"].items()}
        for name, path in self.roots.items():
            if (not path.is_dir() or (len(path.parts) < 4 and not (name == "clang" and path == Path("/usr"))) or path == self.root
                    or path.is_relative_to(self.root) or self.root.is_relative_to(path)):
                raise Incomplete("Tool roots must be narrow external directories.")
        if len(set(self.roots.values())) != len(self.roots) or any(a != b and (a.is_relative_to(b) or b.is_relative_to(a))
               for a in self.roots.values() for b in self.roots.values()):
            raise Incomplete("Tool roots must be separate.")
        self.hashes = {}

        def pin(path, expected):
            if path in self.hashes and self.hashes[path] != expected:
                raise Incomplete("Conflicting payload/helper/fixture identity.")
            self.hashes[path] = expected

        for item in self.policy["payloads"]:
            pin(self.roots[item["root"]] / item["path"], item["sha256"])
        for name, expected in self.policy["inputs"].items():
            pin(self.policy_path.parent / name, expected)
        for item in self.policy["helpers"].values():
            pin(Path(item["path"]), item["sha256"])
        pin(Path(__file__).resolve(strict=True), digest(Path(__file__).resolve(strict=True)))
        self.rust_linker = Path(RUST_LINKER)
        if any(Path(path) not in self.hashes for path in (RUST_LINKER, NATIVE_LINKER, GCC_COLLECT2, LTO_PLUGIN)):
            raise Incomplete("Fixed GNU compiler/linker/collect2/plugin lacks a pinned payload.")
        self.check()
        for name, tool_root in (("rust_compiler", "rust"), ("native_compiler", "clang"), ("symbolizer", "clang")):
            if not os.access(self.roots[tool_root] / self.policy[name], os.X_OK):
                raise Incomplete("Compiler/symbolizer is not executable.")
        for item in self.policy["helpers"].values():
            if not os.access(item["path"], os.X_OK):
                raise Incomplete("Sandbox helper is not executable.")

    def check(self):
        if (read_once(self.policy_path) != self.policy_bytes
                or read_once(self.installation, private=True, maximum=16384) != self.pointer_bytes):
            raise Incomplete("Policy or installation pointer changed.")
        def verify(item):
            path, expected = item
            if digest(path) != expected:
                raise Incomplete(f"Pinned input/payload changed: {path}")
        with ThreadPoolExecutor(max_workers=12) as workers:
            for _ in workers.map(verify, self.hashes.items()):
                pass

    def receipt(self):
        return {"policy_sha256": hashlib.sha256(self.policy_bytes).hexdigest(),
                "installation_sha256": hashlib.sha256(self.pointer_bytes).hexdigest(),
                "roots": {key: str(value) for key, value in self.roots.items()},
                "pinned_sha256": {str(path): value for path, value in self.hashes.items()}}


def fresh_output(value, *, root=ROOT):
    """Create one fresh private Linux output without replacing existing work."""
    if not isinstance(value, str) or os.path.normpath(value) != value:
        raise Incomplete("Output must be a canonical absolute path.")
    path = Path(value)
    parent = canonical(path.parent)
    if (not path.is_absolute() or path.exists() or path.is_symlink()
            or path.is_relative_to(root) or root.is_relative_to(path)
            or path.is_relative_to(Path("/mnt"))):
        raise Incomplete("Use a fresh owned external Linux output directory outside /mnt.")
    path.mkdir(mode=0o700)
    if path.stat().st_uid != os.geteuid() or stat.S_IMODE(path.stat().st_mode) != 0o700:
        raise Incomplete("Output directory is not private.")
    return path


def control_environment(tool, symbolizer):
    """Explicit detector options; no inherited loader or sanitizer settings."""
    common = "halt_on_error=1:abort_on_error=0:exitcode=86:symbolize=1"
    options = {
        "asan": {"ASAN_OPTIONS": "detect_leaks=1:" + common},
        "lsan": {"LSAN_OPTIONS": "exitcode=86:report_objects=1"},
        "tsan": {"TSAN_OPTIONS": common + ":external_symbolizer_path=" + str(symbolizer)},
        "msan": {"MSAN_OPTIONS": common + ":external_symbolizer_path=" + str(symbolizer)},
        "ubsan": {"UBSAN_OPTIONS": common + ":print_stacktrace=1:external_symbolizer_path=" + str(symbolizer)},
    }
    return {"ASAN_SYMBOLIZER_PATH": str(symbolizer), **options[tool]}


def compile_command(admitted, tool, defect):
    """Return the only supported literal build vectors, outside product Cargo."""
    policy, roots = admitted.policy, admitted.roots
    output = f"/work/bin/{tool}-{'defect' if defect else 'healthy'}"
    source = "/work/inputs/" + Path(SOURCES[tool]).name
    if tool == "ubsan":
        return [str(roots["clang"] / policy["native_compiler"]), "--driver-mode=g++",
                "--target=" + PLATFORM, "-fsanitize=undefined", "-fno-sanitize-recover=all",
                "-O1", "-g", "-fno-omit-frame-pointer", "-fPIE", "-pie",
                *(["-DSEEDED_DEFECT=1"] if defect else []), source, "-o", output]
    kind = {"asan": "address", "lsan": "leak", "tsan": "thread", "msan": "memory"}[tool]
    return [str(roots["rust"] / policy["rust_compiler"]), source, "--edition=2024",
            "--crate-name", "sanitizer_" + tool, "--target", PLATFORM, "-Zsanitizer=" + kind,
            *(["--sysroot", str(roots["sysroots"] / policy["sysroots"][tool])] if tool in SYSROOTS else []),
            "-Copt-level=1", "-Cdebuginfo=2", "-Cpanic=abort", "-Coverflow-checks=yes",
            "-Cforce-frame-pointers=yes", "-Crelocation-model=pic", "-Clink-arg=-pie",
            "-l", "dylib=gcc_s", "-l", "dylib=m",
            "-Clinker=" + str(admitted.rust_linker), "-Clink-arg=-fuse-ld=bfd",
            *(["--cfg", "seeded_defect"] if defect else []), "-o", output]


def sandbox_command(admitted, work, command, *, runtime=False, environment=None):
    """Retain offline namespaces and fixed limits; only runtime omits RLIMIT_AS."""
    helpers = admitted.policy["helpers"]
    environment = environment or {}
    trusted = {"PATH": str((admitted.roots["clang"] / admitted.policy["symbolizer"]).parent) + ":/usr/bin:/bin",
               "LANG": "C.UTF-8", "HOME": "/work/home", "TMPDIR": "/work/tmp",
               "TMP": "/work/tmp", "TEMP": "/work/tmp", "RUSTUP_AUTO_INSTALL": "0"}
    if set(trusted) & set(environment):
        raise Incomplete("Reserved sandbox environment.")
    trusted.update(environment)
    args = [helpers["bwrap"]["path"], "--unshare-all", "--die-with-parent", "--new-session",
            "--cap-drop", "ALL", "--clearenv", "--bind", str(work), "/work",
            "--bind", str(work / "tmp"), "/tmp"]
    for name in ("/usr", "/bin", "/lib", "/lib64", "/etc/ld.so.cache"):
        if Path(name).exists():
            args.extend(["--ro-bind", name, name])
    for path in admitted.roots.values():
        args.extend(["--ro-bind", str(path), str(path)])
    args.extend(["--ro-bind", str(work / "inputs"), "/work/inputs",
                 "--proc", "/proc", "--dev", "/dev", "--chdir", "/work"])
    if runtime:
        args.extend(["--ro-bind", str(work / "bin"), "/work/bin"])
    for key, value in trusted.items():
        args.extend(["--setenv", key, value])
    limits = ["--cpu=20:20" if runtime else "--cpu=60:60", "--fsize=33554432:33554432",
              "--nproc=128:128", "--nofile=256:256", "--core=0:0"]
    if not runtime:
        limits.append("--as=8589934592:8589934592")
    return args + ["--", helpers["prlimit"]["path"], *limits, "--", *command], trusted


def run_command(command, *, timeout, output_limit=MAX_OUTPUT):
    """Bound capture and wall time; kill only the newly owned process group."""
    started = time.monotonic()
    result = {"command": command, "exit_code": None, "failure": None, "output": ""}
    process = None
    captured = bytearray()
    try:
        process = subprocess.Popen(command, stdin=subprocess.DEVNULL, stdout=subprocess.PIPE,
                                   stderr=subprocess.STDOUT, start_new_session=True,
                                   env={"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"})
        with selectors.DefaultSelector() as selector:
            selector.register(process.stdout, selectors.EVENT_READ)
            while selector.get_map():
                remaining = timeout - (time.monotonic() - started)
                if remaining <= 0:
                    result["failure"] = "timeout"
                    break
                for key, _events in selector.select(min(remaining, 0.2)):
                    part = os.read(key.fileobj.fileno(), 65536)
                    if not part:
                        selector.unregister(key.fileobj)
                    else:
                        available = output_limit - len(captured)
                        captured.extend(part[:max(0, available)])
                        if len(part) > available:
                            result["failure"] = "output-limit"
                            break
                if result["failure"]:
                    break
        if not result["failure"]:
            remaining = timeout - (time.monotonic() - started)
            try:
                process.wait(timeout=max(0.001, remaining))
            except subprocess.TimeoutExpired:
                result["failure"] = "timeout"
    except OSError as error:
        result["failure"] = "spawn-error"
        result["error"] = str(error)
    finally:
        if process is not None:
            if result["failure"]:
                try:
                    os.killpg(process.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            try:
                result["exit_code"] = process.wait(timeout=5)
            except subprocess.TimeoutExpired:
                result["failure"] = "cleanup-incomplete"
            if process.stdout:
                process.stdout.close()
        result["output"] = captured.decode("utf-8", errors="replace")
        result["elapsed_seconds"] = time.monotonic() - started
    return result


def classify(tool, result, *, defect):
    """Require actual mechanism/site evidence; crashes and setup errors fail."""
    output = result["output"]
    if result.get("failure") or result.get("exit_code") is None or SETUP_FAILURE.search(output):
        raise Incomplete("Control failed before valid detector evidence.")
    if not defect:
        if (result["exit_code"] != 0 or not re.search(r"(?m)^SANITIZER_HEALTHY\s*$", output)
                or re.search(r"(?:ERROR|WARNING): \w*Sanitizer:|runtime error:", output)):
            raise Incomplete("Healthy control did not succeed cleanly with its marker.")
    elif (result["exit_code"] == 0 or DIAGNOSTICS[tool] not in output
          or not re.search(r"(?<![\w.-])" + re.escape(Path(SOURCES[tool]).name) + r":\d+(?::\d+)?", output)):
        raise Incomplete("Defect control lacks its exact detector diagnosis/source site.")


def fixtures(admitted, output, selected, report):
    """Run independent fixed pairs; retain the first failure and all raw results."""
    work = output / "work"
    work.mkdir(mode=0o700)
    for name in ("inputs", "bin", "tmp", "home"):
        (work / name).mkdir(mode=0o700)
    for source, expected in admitted.policy["inputs"].items():
        data = read_once(admitted.policy_path.parent / source, maximum=256 * 1024)
        if hashlib.sha256(data).hexdigest() != expected:
            raise Incomplete("Fixture changed before copying.")
        destination = work / "inputs" / Path(source).name
        destination.write_bytes(data)
        destination.chmod(0o400)
    copied = {path: digest(path) for path in (work / "inputs").iterdir()}
    artifacts = {}

    def check():
        admitted.check()
        if (set((work / "inputs").iterdir()) != set(copied)
                or any(digest(path) != value for path, value in copied.items())):
            raise Incomplete("Copied fixture inputs changed.")
        if any(digest(path) != value for path, value in artifacts.items()):
            raise Incomplete("Emitted control artifact changed.")

    def save():
        (output / "verification.json").write_text(json.dumps(report, indent=2) + "\n")

    for tool in selected:
        row = {"status": "incomplete", "commands": []}
        report["tools"][tool] = row
        try:
            symbolizer = admitted.roots["clang"] / admitted.policy["symbolizer"]
            for defect in (False, True):
                case = "defect" if defect else "healthy"
                for phase in ("compile", "run"):
                    check()
                    command = compile_command(admitted, tool, defect) if phase == "compile" else [f"/work/bin/{tool}-{case}"]
                    environment = control_environment(tool, symbolizer) if phase == "run" else {}
                    argv, effective = sandbox_command(admitted, work, command, runtime=phase == "run", environment=environment)
                    result = run_command(argv, timeout=30 if phase == "run" else 90)
                    result.update(phase=phase, case=case, effective_environment=effective)
                    row["commands"].append(result)
                    save()
                    check()
                    if phase == "compile":
                        if result["failure"] or result["exit_code"] != 0:
                            raise Incomplete("Control compilation failed.")
                        artifact = work / "bin" / f"{tool}-{case}"
                        regular(artifact)
                        if not os.access(artifact, os.X_OK):
                            raise Incomplete("Compiler emitted no executable control.")
                        row[case + "_artifact_sha256"] = digest(artifact)
                        artifacts[artifact] = row[case + "_artifact_sha256"]
                    else:
                        classify(tool, result, defect=defect)
            row["status"] = "qualified"
        except (Incomplete, OSError, ValueError, TypeError, KeyError) as error:
            row["reason"] = str(error)
            report.setdefault("first_failure", {"tool": tool, "reason": str(error)})
        save()
    check()


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tool", choices=(*TOOLS, "all"), default="all")
    parser.add_argument("--fixtures", action="store_true")
    parser.add_argument("--output-dir")
    parser.add_argument("--installation", default=str(Path.home() / ".local/share/nevrotick/sanitizer-installation.json"))
    args = parser.parse_args(argv)
    if args.fixtures != bool(args.output_dir):
        parser.error("--fixtures requires --output-dir; output is accepted only for fixtures")
    report = {"schema_version": 1, "status": "incomplete", "tools": {},
              "mode": "fixtures" if args.fixtures else "admission-only",
              "qualification": "isolated detector controls only; no product/native Windows/release qualification",
              "limitations": ["Instrumented core/compiler_builtins for standalone no_std controls only; no std/application graph or libc coverage.",
                              "Pinned payload list declares admitted coverage; no mixed-language runtime qualification.",
                              "Runtime shadow address space has no RLIMIT_AS; no hard aggregate RAM/disk quota."],
              "started_unix": time.time()}
    output = None
    try:
        if os.name != "posix" or not sys.platform.startswith("linux") or os.geteuid() == 0:
            raise Incomplete("Run controls as an ordinary Linux user.")
        admitted = Admission(HERE / "policy.json", args.installation)
        report.update(admitted.receipt())
        if args.fixtures:
            output = fresh_output(args.output_dir)
            report["output_dir"] = str(output)
            fixtures(admitted, output, TOOLS if args.tool == "all" else (args.tool,), report)
        admitted.check()
        if not args.fixtures or all(item["status"] == "qualified" for item in report["tools"].values()):
            report["status"] = "passed"
    except (Incomplete, OSError, ValueError, TypeError, KeyError) as error:
        report["reason"] = str(error)
        report.setdefault("first_failure", {"reason": str(error)})
    report["finished_unix"] = time.time()
    if output:
        (output / "verification.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps(report, indent=2))
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
