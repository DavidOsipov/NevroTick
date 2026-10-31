#!/usr/bin/env python3
"""Admit and isolate a pinned checker for two fixture-verification Python owners.

This entry point installs nothing. The external receipt freezes every regular
file in an owned checker environment, the exact wheel pins, policy and source
closure. Public read-only OS libraries are the existing bootstrap trust boundary;
the venv interpreter and all installed checker packages are inventoried. Static
results do not replace runtime JSON/log validation or native qualification.
"""
from __future__ import annotations

import argparse
import ast
from concurrent.futures import ThreadPoolExecutor
from email.parser import Parser
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import selectors
import shutil
import signal
import stat
import subprocess
import tempfile
import time


POLICY_PATH = "policy/python-typing.json"
FIXTURE_DIR = "scripts/tests/fixtures/python_typing"
HELPERS = {
    "scripts/core-combination-accounting.py": "core_combination_accounting.py",
    "scripts/core-analysis-cohorts.py": "core_analysis_cohorts.py",
}
FIXTURE_NAMES = (
    "typed_consumer.py", "wrong_field.py", "wrong_value.py",
    "reservation_as_observation.py", "partial_as_complete.py",
    "unhandled_status.py", "unhandled_match.py",
)
SOURCE_PATHS = (
    "scripts/check-python-typing.py", "scripts/tests/test_python_typing.py",
    *HELPERS, *(f"{FIXTURE_DIR}/{name}" for name in FIXTURE_NAMES),
)
OPTIONS = {
    "python_version": "3.12", "platform": "linux", "strict": True,
    "disallow_any_expr": True, "disallow_any_explicit": True,
    "disallow_any_decorated": True, "disallow_any_unimported": True,
    "warn_unused_configs": True, "enable_error_code": "exhaustive-match",
    "no_site_packages": True, "incremental": False, "cache_dir": "/dev/null",
    "mypy_path": "/input", "follow_imports": "normal",
    "show_error_codes": True, "show_column_numbers": True,
    "pretty": False, "color_output": False, "error_summary": True,
}
MAX_JSON = 8 * 1024 * 1024
MAX_SOURCE = 2 * 1024 * 1024
MAX_FILES = 20000
MAX_TOOL_BYTES = 512 * 1024 * 1024
MAX_OUTPUT = 1024 * 1024
RECEIPT_KEYS = frozenset({
    "schema_version", "checker_root", "python_version", "mypy_version",
    "wheels", "files", "sources", "policy_sha256",
})
BOOTSTRAP = "import sys; sys.path.insert(0, '/checker/lib/python3.12/site-packages'); "
CHECKER_ENTRY = BOOTSTRAP + "from mypy.__main__ import console_entry; console_entry()"
VERSION_ENTRY = BOOTSTRAP + (
    "import json, mypy.version; "
    "print(json.dumps({'python': '.'.join(map(str, sys.version_info[:3])), "
    "'mypy': mypy.version.__version__}, sort_keys=True))"
)
# Each negative case must produce its intended error in its selected source.
# Extra cascading diagnostics are allowed only in that same deliberately bad file.
CASES = (
    ("wrong_field", "wrong_field.py", "typeddict-item", "unknown_counter"),
    ("wrong_value", "wrong_value.py", "typeddict-item", "terms"),
    ("reservation_as_observation", "reservation_as_observation.py", "return-value", "ReservedAttempts"),
    ("partial_as_complete", "partial_as_complete.py", "arg-type", "PartialObservation"),
    ("unhandled_status", "unhandled_status.py", "arg-type", "InterruptedObservation"),
    ("unhandled_match", "unhandled_match.py", "exhaustive-match", "interrupted"),
    ("incompatible_implementation", "typed_consumer.py", "assignment", "CombinationInterface"),
)


class Incomplete(ValueError):
    """Admission, isolation or an intended enforcement observation is incomplete."""

    evidence: dict | None = None


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def read_regular(path: Path, maximum: int) -> bytes:
    """Read bounded nonlinked bytes without following a final symlink."""
    try:
        descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW)
        with os.fdopen(descriptor, "rb") as stream:
            metadata = os.fstat(stream.fileno())
            if not stat.S_ISREG(metadata.st_mode) or metadata.st_nlink != 1 or metadata.st_size > maximum:
                raise Incomplete(f"Not a bounded regular file with unique custody: {path}")
            # Allocate for the admitted actual size, with one growth sentinel.
            # Using the entire allowance allocates 512 MiB even for a tiny file
            # under Python's development allocator; the byte budget is a ceiling.
            value = stream.read(metadata.st_size + 1)
            current = os.fstat(stream.fileno())
            identity = lambda item: (item.st_dev, item.st_ino, item.st_size, item.st_mtime_ns,
                                     item.st_ctime_ns, item.st_nlink)
            if len(value) != metadata.st_size or identity(current) != identity(metadata):
                raise Incomplete(f"File changed during bounded read: {path}")
            return value
    except OSError as error:
        raise Incomplete(f"Cannot read admitted regular file: {path}: {error.strerror}") from error


def closed_pairs(pairs: list[tuple[str, object]]) -> dict[str, object]:
    result = {}
    for key, value in pairs:
        if key in result:
            raise Incomplete(f"Duplicate JSON key: {key}")
        result[key] = value
    return result


def read_json(path: Path) -> tuple[dict, bytes]:
    raw = read_regular(path, MAX_JSON)
    try:
        value = json.loads(raw, object_pairs_hook=closed_pairs)
    except (UnicodeDecodeError, json.JSONDecodeError) as error:
        raise Incomplete(f"Invalid JSON: {path}") from error
    if type(value) is not dict:
        raise Incomplete("A JSON object is required.")
    return value, raw


def absolute_external(path: Path, root: Path) -> Path:
    """Require an existing external path with no symlink aliases in its spelling."""
    if not path.is_absolute() or path != path.resolve(strict=True):
        raise Incomplete("External input must use an absolute, unaliased existing path.")
    if path == root or root in path.parents:
        raise Incomplete("Checker environment and receipt must be outside the checkout.")
    return path


def source_snapshot(root: Path) -> tuple[dict[str, bytes], dict[str, str]]:
    sources = {name: read_regular(root / name, MAX_SOURCE) for name in SOURCE_PATHS}
    return sources, {name: digest(raw) for name, raw in sources.items()}


def inventory_tree(root: Path) -> dict[str, str]:
    """Hash the COMPLETE checker tree, including metadata; no symlink shortcuts."""
    selected = []
    total = 0
    for directory, directories, files in os.walk(root, followlinks=False):
        for name in [*directories, *files]:
            path = Path(directory) / name
            metadata = path.lstat()
            if stat.S_ISDIR(metadata.st_mode):
                continue
            if not stat.S_ISREG(metadata.st_mode) or metadata.st_nlink != 1:
                raise Incomplete(f"Checker tree contains a link or special file: {path}")
            total += metadata.st_size
            selected.append(path)
            if len(selected) > MAX_FILES or total > MAX_TOOL_BYTES:
                raise Incomplete("Checker inventory exceeds the admitted count/byte budget.")
    if not selected:
        raise Incomplete("Checker inventory is empty.")
    selected.sort()
    with ThreadPoolExecutor(max_workers=12) as executor:
        hashes = list(executor.map(lambda path: digest(read_regular(path, MAX_TOOL_BYTES)), selected))
    return {path.relative_to(root).as_posix(): value for path, value in zip(selected, hashes)}


def require_hash_map(value: object, *, expected: set[str] | None = None) -> dict[str, str]:
    if type(value) is not dict or not value or len(value) > MAX_FILES:
        raise Incomplete("Invalid or empty admitted hash inventory.")
    for name, sha256 in value.items():
        if (type(name) is not str or not name or len(name) > 512
                or PurePosixPath(name).is_absolute() or ".." in PurePosixPath(name).parts
                or str(PurePosixPath(name)) != name
                or type(sha256) is not str or re.fullmatch(r"[a-f0-9]{64}", sha256) is None):
            raise Incomplete("Inventory requires canonical relative paths and SHA-256 values.")
    if expected is not None and set(value) != expected:
        raise Incomplete("Admitted source closure differs from the exact selected files.")
    return value


def inspect_policy(root: Path) -> tuple[dict, bytes]:
    policy, raw = read_json(root / POLICY_PATH)
    if (set(policy) != {"schema_version", "python_version", "mypy_version", "wheels", "mypy_options"}
            or type(policy["schema_version"]) is not int or policy["schema_version"] != 1
            or policy["python_version"] != "3.12.3" or policy["mypy_version"] != "2.4.1"
            or policy["mypy_options"] != OPTIONS):
        raise Incomplete("Python typing policy differs from the closed strict configuration.")
    wheels = policy["wheels"]
    if type(wheels) is not dict or set(wheels) != {
            "mypy", "typing-extensions", "mypy-extensions", "pathspec", "librt", "ast-serialize"}:
        raise Incomplete("The six-package checker wheel closure is required.")
    for name, wheel in wheels.items():
        if (type(wheel) is not dict or set(wheel) != {"version", "sha256"}
                or type(wheel["version"]) is not str or not wheel["version"]
                or type(wheel["sha256"]) is not str
                or re.fullmatch(r"[0-9a-f]{64}", wheel["sha256"]) is None):
            raise Incomplete(f"Invalid pinned wheel record: {name}")
    if wheels["mypy"]["version"] != policy["mypy_version"]:
        raise Incomplete("Checker version and wheel version disagree.")
    return policy, raw


def inspect_environment(checker_root: Path, files: dict[str, str], policy: dict) -> None:
    """Bind interpreter, package metadata and exact file set before any execution."""
    if inventory_tree(checker_root) != files:
        raise Incomplete("Checker COMPLETE installed-file inventory differs from admission.")
    cfg = read_regular(checker_root / "pyvenv.cfg", 16384).decode("utf-8")
    settings = dict(line.partition(" = ")[::2] for line in cfg.splitlines() if " = " in line)
    if settings.get("include-system-site-packages") != "false" or settings.get("version") != policy["python_version"]:
        raise Incomplete("Checker requires the exact isolated Python venv configuration.")
    site = checker_root / "lib/python3.12/site-packages"
    actual = {}
    for metadata in sorted(site.glob("*.dist-info/METADATA")):
        fields = Parser().parsestr(read_regular(metadata, MAX_SOURCE).decode("utf-8"))
        name = re.sub(r"[-_.]+", "-", fields.get("Name", "")).lower()
        if name in actual or not name:
            raise Incomplete("Duplicate or unnamed installed distribution.")
        actual[name] = fields.get("Version")
    if actual != {name: record["version"] for name, record in policy["wheels"].items()}:
        raise Incomplete("Installed distribution versions differ from the exact wheel closure.")
    if "bin/python" not in files or "pyvenv.cfg" not in files:
        raise Incomplete("Copied interpreter and venv configuration must be inventoried.")


def inspect_sources(sources: dict[str, bytes]) -> list[str]:
    """Reject unchecked escape hatches/import expansion in the checked closure."""
    allowed = {"__future__", "typing", "re", "hashlib", "json", "pathlib",
               "core_combination_accounting", "core_analysis_cohorts"}
    imports = set()
    checked = [*HELPERS, *(f"{FIXTURE_DIR}/{name}" for name in FIXTURE_NAMES)]
    for name in checked:
        text = sources[name].decode("utf-8")
        if re.search(r"#\s*type:\s*ignore\b|#\s*mypy:", text):
            raise Incomplete(f"Unchecked per-source checker override: {name}")
        tree = ast.parse(text, filename=name)
        for node in ast.walk(tree):
            if isinstance(node, ast.Import):
                names = [entry.name for entry in node.names]
            elif isinstance(node, ast.ImportFrom):
                if node.level:
                    raise Incomplete(f"Relative import outside the admitted closure: {name}")
                names = [node.module]
                if node.module == "typing" and any(entry.name in {"Any", "cast", "no_type_check"} for entry in node.names):
                    raise Incomplete(f"Unchecked typing escape hatch: {name}")
            else:
                names = []
            if any(imported not in allowed for imported in names):
                raise Incomplete(f"Import outside the selected standard-library/source closure: {name}")
            imports.update(names)
            if isinstance(node, ast.Attribute) and node.attr in {"Any", "cast", "no_type_check"}:
                raise Incomplete(f"Unchecked typing escape hatch: {name}")
    return sorted(imports)


def config_bytes() -> bytes:
    def value(item: object) -> str:
        return str(item).lower() if type(item) is bool else str(item)
    return ("[mypy]\n" + "".join(f"{key} = {value(item)}\n" for key, item in OPTIONS.items())).encode()


def sandbox_command(checker_root: Path, root: Path, work: Path, source: Path,
                    command: list[str]) -> list[str]:
    bwrap = shutil.which("bwrap", path="/usr/bin:/bin")
    prlimit = shutil.which("prlimit", path="/usr/bin:/bin")
    if not bwrap or not prlimit:
        raise Incomplete("Python typing requires bubblewrap/prlimit; host execution fallback is forbidden.")
    args = [bwrap, "--unshare-all", "--die-with-parent", "--new-session", "--cap-drop", "ALL", "--clearenv"]
    for path in ("/usr", "/bin", "/lib", "/lib64", "/etc/ld.so.cache"):
        if Path(path).exists():
            args.extend(["--ro-bind", path, path])
    args.extend(["--ro-bind", str(checker_root), "/checker", "--ro-bind", str(root), "/src",
                 "--bind", str(work), "/work", "--ro-bind", str(source), "/input",
                 "--proc", "/proc", "--dev", "/dev", "--tmpfs", "/tmp", "--chdir", "/work",
                 "--setenv", "PATH", "/checker/bin:/usr/bin:/bin",
                 "--setenv", "LANG", "C.UTF-8", "--setenv", "TMPDIR", "/tmp"])
    return args + ["--", "/usr/bin/prlimit", "--as=4294967296", "--cpu=90", "--nproc=128",
                   "--nofile=256", "--fsize=16777216", "--", *command]


def run(command: list[str], *, cwd: Path, timeout: float = 120) -> dict:
    """Bound the isolated process group, output and wall time on every outcome."""
    process = subprocess.Popen(command, cwd=cwd, env={}, stdin=subprocess.DEVNULL,
                               stdout=subprocess.PIPE, stderr=subprocess.STDOUT, start_new_session=True)
    output = bytearray()
    failure = None
    try:
        with selectors.DefaultSelector() as selector:
            selector.register(process.stdout, selectors.EVENT_READ)
            deadline = time.monotonic() + timeout
            while selector.get_map() or process.poll() is None:
                if time.monotonic() >= deadline:
                    failure = "wall timeout"
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
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        if process.poll() is None:
            process.wait(timeout=5)
        process.stdout.close()
    return {"command": command, "exit_code": process.returncode,
            "failure": failure, "output": output.decode("utf-8", errors="replace")}


def require_result(result: dict, case: tuple[str, str, str, str] | None = None) -> None:
    """Setup/import failures never count as the intended static rejection."""
    if result["failure"]:
        raise Incomplete(f"Checker execution incomplete: {result['failure']}")
    errors = re.findall(r"^/input/([^:]+):\d+:\d+: error: (.+)  \[([^\]]+)\]$", result["output"], re.MULTILINE)
    if case is None:
        if result["exit_code"] != 0 or errors or "Success: no issues found in 3 source files" not in result["output"]:
            raise Incomplete(f"Healthy typed implementation/consumer failed:\n{result['output'][-6000:]}")
    else:
        _, filename, code, fragment = case
        if (result["exit_code"] != 1 or not errors
                or not any(file == filename and diagnostic == code and fragment in message
                           for file, message, diagnostic in errors)
                or any(file != filename or diagnostic in {"import", "import-not-found", "import-untyped", "syntax"}
                       for file, _, diagnostic in errors)):
            raise Incomplete(f"Missing intended {case[0]} diagnostic ({code}):\n{result['output'][-6000:]}")


def prepare_source(directory: Path, sources: dict[str, bytes], case: tuple | None) -> None:
    directory.mkdir()
    for original, normalized in HELPERS.items():
        raw = sources[original]
        if case and case[0] == "incompatible_implementation" and normalized == "core_combination_accounting.py":
            before = b"def full_package_reservation() -> CombinationReservation:"
            after = b"def full_package_reservation(required: str) -> CombinationReservation:"
            if raw.count(before) != 1:
                raise Incomplete("Actual implementation signature mutation no longer has one exact owner.")
            raw = raw.replace(before, after)
        (directory / normalized).write_bytes(raw)
    (directory / "typed_consumer.py").write_bytes(sources[f"{FIXTURE_DIR}/typed_consumer.py"])
    if case and case[1] != "typed_consumer.py":
        (directory / case[1]).write_bytes(sources[f"{FIXTURE_DIR}/{case[1]}"])


def execute(root: Path, checker_python: Path, admission: Path, fixtures: bool) -> dict:
    """Read/admit first; run only fixed commands against owned scratch copies."""
    started = time.monotonic()
    admission = absolute_external(admission, root)
    receipt, receipt_raw = read_json(admission)
    if set(receipt) != RECEIPT_KEYS or type(receipt["schema_version"]) is not int or receipt["schema_version"] != 1:
        raise Incomplete("Typing admission is not the closed schema_version 1 receipt.")
    if type(receipt["checker_root"]) is not str:
        raise Incomplete("checker_root must be an absolute path.")
    checker_root = absolute_external(Path(receipt["checker_root"]), root)
    checker_python = absolute_external(checker_python, root)
    if not checker_root.is_dir() or checker_python != checker_root / "bin/python":
        raise Incomplete("--checker-python must be the inventoried copied checker_root/bin/python.")
    policy, policy_raw = inspect_policy(root)
    if (receipt["python_version"] != policy["python_version"] or receipt["mypy_version"] != policy["mypy_version"]
            or receipt["wheels"] != policy["wheels"] or receipt["policy_sha256"] != digest(policy_raw)):
        raise Incomplete("Typing receipt and current pinned policy disagree.")
    sources, source_hashes = source_snapshot(root)
    if require_hash_map(receipt["sources"], expected=set(SOURCE_PATHS)) != source_hashes:
        raise Incomplete("Current source bytes differ from typing admission.")
    files = require_hash_map(receipt["files"])
    inspect_environment(checker_root, files, policy)
    checked_imports = inspect_sources(sources)
    admitted_seconds = time.monotonic() - started
    evidence = {"schema_version": 1, "status": "incomplete", "scope": "two fixture helpers and independent typed consumer",
                "admission_sha256": digest(receipt_raw), "policy_sha256": digest(policy_raw),
                "sources": source_hashes, "python_version": policy["python_version"],
                "mypy_version": policy["mypy_version"], "wheels": policy["wheels"],
                "installed_files": len(files), "generated_config_sha256": digest(config_bytes()),
                "checked_files": [*HELPERS, f"{FIXTURE_DIR}/typed_consumer.py"],
                "checked_imports": checked_imports,
                "fixture_controls_requested": fixtures, "fixture_controls_executed": [],
                "normalization": HELPERS, "exceptions": [], "lanes": [],
                "timings_seconds": {"admission": round(admitted_seconds, 3)}}
    def bookend(work: Path, selected: Path, selected_files: dict[str, str]) -> None:
        if (source_snapshot(root)[1] != source_hashes or read_regular(root / POLICY_PATH, MAX_JSON) != policy_raw
                or read_regular(admission, MAX_JSON) != receipt_raw
                or read_regular(work / "mypy.ini", MAX_JSON) != config_bytes()
                or inventory_tree(selected) != selected_files):
            raise Incomplete("Typing source/config/admission/scratch input changed during the command.")
        inspect_environment(checker_root, files, policy)

    try:
        with tempfile.TemporaryDirectory(prefix="nevrotick-python-typing-") as temporary:
            scratch = Path(temporary)
            work = scratch / "work"
            work.mkdir()
            (work / "mypy.ini").write_bytes(config_bytes())
            # Source siblings have no writable /work alias inside the sandbox.
            healthy = scratch / "healthy"
            prepare_source(healthy, sources, None)
            healthy_files = inventory_tree(healthy)
            version = run(sandbox_command(checker_root, root, work, healthy,
                          ["/checker/bin/python", "-I", "-B", "-S", "-c", VERSION_ENTRY]), cwd=root, timeout=30)
            evidence["version"] = version
            bookend(work, healthy, healthy_files)
            if version["failure"] or version["exit_code"] != 0:
                raise Incomplete(f"Inventoried checker failed isolated version admission:\n{version['output'][-2000:]}")
            if json.loads(version["output"]) != {"python": policy["python_version"], "mypy": policy["mypy_version"]}:
                raise Incomplete("Executed checker/interpreter versions differ from exact admission.")
            for case in [None, *(CASES if fixtures else ())]:
                selected = healthy
                if case:
                    selected = scratch / case[0]
                    prepare_source(selected, sources, case)
                selected_inventory = inventory_tree(selected)
                selected_files = ["/input/core_combination_accounting.py", "/input/core_analysis_cohorts.py", "/input/typed_consumer.py"]
                if case and case[1] != "typed_consumer.py":
                    selected_files.append("/input/" + case[1])
                command = ["/checker/bin/python", "-I", "-B", "-S", "-c", CHECKER_ENTRY,
                           "--config-file", "/work/mypy.ini", *selected_files]
                lane_started = time.monotonic()
                result = run(sandbox_command(checker_root, root, work, selected, command), cwd=root)
                result["name"] = case[0] if case else "healthy"
                result["seconds"] = round(time.monotonic() - lane_started, 3)
                evidence["lanes"].append(result)
                bookend(work, selected, selected_inventory)
                require_result(result, case)
                if case:
                    evidence["fixture_controls_executed"].append(case[0])
    except (Incomplete, OSError, UnicodeError, json.JSONDecodeError, SyntaxError) as error:
        failure = Incomplete(str(error))
        failure.evidence = evidence
        raise failure from error
    evidence["status"] = "passed-static-fixture-typing" if fixtures else "passed-static-typing"
    evidence["timings_seconds"]["total"] = round(time.monotonic() - started, 3)
    return evidence


def write_result(destination: Path, root: Path, evidence: dict) -> None:
    """Create one fresh external result; never replace user files or checkout bytes."""
    if not destination.is_absolute() or destination.parent != destination.parent.resolve(strict=True):
        raise Incomplete("Result requires an absolute path below an existing unaliased directory.")
    if destination == root or root in destination.parents:
        raise Incomplete("Result must be external to the checkout.")
    descriptor = os.open(destination, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
    with os.fdopen(descriptor, "w", encoding="utf-8") as stream:
        json.dump(evidence, stream, indent=2, sort_keys=True)
        stream.write("\n")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checker-python", type=Path, required=True)
    parser.add_argument("--admission", type=Path, required=True)
    parser.add_argument("--fixtures", action="store_true")
    parser.add_argument("--json", type=Path)
    args = parser.parse_args()
    root = Path(__file__).resolve().parents[1]
    try:
        evidence = execute(root, args.checker_python, args.admission, args.fixtures)
    except (Incomplete, OSError, UnicodeError, json.JSONDecodeError, SyntaxError) as error:
        evidence = getattr(error, "evidence", None) or {"schema_version": 1, "status": "incomplete"}
        evidence["error"] = str(error)
    if args.json:
        try:
            write_result(args.json, root, evidence)
        except (Incomplete, OSError) as error:
            print(f"Python typing incomplete: {error}")
            return 1
    print(json.dumps(evidence, indent=2, sort_keys=True))
    return 0 if evidence["status"] in {"passed-static-typing", "passed-static-fixture-typing"} else 1


if __name__ == "__main__":
    raise SystemExit(main())
