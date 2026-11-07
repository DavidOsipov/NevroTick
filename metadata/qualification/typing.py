"""Qualify strict typing of the complete portable metadata source/import closure.

The existing pinned checker store is only read. Every check runs in a private
networkless read-only-source sandbox with bounded resources and fresh output.
Static interfaces complement runtime validation; they do not validate raw input.
"""
from __future__ import annotations

import argparse
import ast
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tempfile
import time

ROOT = Path(__file__).resolve().parents[2]
SOURCES = (
    "metadata/__init__.py", "metadata/pipeline/__init__.py",
    "metadata/pipeline/__main__.py", "metadata/pipeline/contract.py",
    "metadata/pipeline/fields.py", "metadata/pipeline/text_policy.py",
    "metadata/pipeline/delivery.py", "metadata/pipeline/generate.py",
)
FIXTURES = {
    "typing_consumer.py": None,
    "typing_wrong_field.py": "typeddict-item",
    "typing_wrong_value.py": "typeddict-item",
    "typing_wrong_interface.py": "assignment",
    "typing_unhandled_outcome.py": "exhaustive-match",
    "typing_frozen_model.py": "misc",
}
CONFIG = "metadata/qualification/typing_mypy.ini"
EXPECTED_CONFIG = '[mypy]\npython_version = 3.12\nplatform = linux\nstrict = True\ndisallow_any_expr = True\ndisallow_any_explicit = True\ndisallow_any_decorated = True\ndisallow_any_unimported = True\nwarn_unused_configs = True\nenable_error_code = exhaustive-match\nno_site_packages = True\nincremental = False\ncache_dir = /dev/null\nmypy_path = /input\nfollow_imports = normal\nshow_error_codes = True\nshow_column_numbers = True\npretty = False\ncolor_output = False\nerror_summary = True\n'
ALLOWED_IMPORTS = {
    "__future__", "argparse", "bisect", "collections", "collections.abc", "csv", "dataclasses",
    "hashlib", "io", "ipaddress", "json", "os", "pathlib", "re", "secrets", "shutil",
    "stat", "sys", "tomllib", "types", "typing", "unicodedata", "urllib.parse",
    "xml.etree.ElementTree", "metadata.pipeline.contract", "metadata.pipeline.generate",
    "fields", "text_policy", "contract", "delivery",
}


class Incomplete(ValueError):
    """The selected gate did not complete; no pass may be inferred."""


def digest(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def inventory(base: Path) -> dict[str, str]:
    """Bind every installed file, including executable interpreter and bytecode."""
    result = {}
    for path in sorted(base.rglob("*")):
        if path.is_symlink():
            raise Incomplete("checker store contains an unadmitted symlink")
        if path.is_file():
            result[str(path.relative_to(base))] = digest(path.read_bytes())
    if "bin/python" not in result or "pyvenv.cfg" not in result:
        raise Incomplete("checker store is not the selected isolated venv")
    return result


def inspect_source(name: str, raw: bytes) -> set[str]:
    """Disallow Any/cast/ignore/config/import escapes in every checked input."""
    source = raw.decode("utf-8")
    if re.search(r"#\s*(?:type:\s*ignore|mypy:)", source):
        raise Incomplete("per-file typing override: " + name)
    imports = set()
    for node in ast.walk(ast.parse(source, filename=name)):
        if isinstance(node, ast.Import):
            names = [item.name for item in node.names]
        elif isinstance(node, ast.ImportFrom):
            names = [node.module or ""]
            if node.level not in (0, 1) or (node.level and not name.startswith("metadata/pipeline/")):
                raise Incomplete("import outside selected closure: " + name)
            if node.module == "typing" and any(item.name in {"Any", "cast", "no_type_check"} for item in node.names):
                raise Incomplete("unchecked typing escape: " + name)
        else:
            names = []
        if set(names) - ALLOWED_IMPORTS:
            raise Incomplete("import outside selected closure: " + name)
        imports.update(names)
        if isinstance(node, ast.Attribute) and node.attr in {"Any", "cast", "no_type_check"}:
            raise Incomplete("unchecked typing escape: " + name)
    return imports


def sandbox(checker: Path, inputs: Path, work: Path, command: list[str]) -> list[str]:
    """Admit only the pinned checker and copied source; no host fallback."""
    bwrap = shutil.which("bwrap", path="/usr/bin:/bin")
    prlimit = shutil.which("prlimit", path="/usr/bin:/bin")
    if not bwrap or not prlimit:
        raise Incomplete("bubblewrap and prlimit are required")
    result = [bwrap, "--unshare-all", "--die-with-parent", "--new-session", "--cap-drop", "ALL", "--clearenv"]
    for path in ("/usr", "/bin", "/lib", "/lib64", "/etc/ld.so.cache"):
        if Path(path).exists():
            result.extend(("--ro-bind", path, path))
    result.extend(("--ro-bind", str(checker), "/checker", "--ro-bind", str(inputs), "/input",
                   "--bind", str(work), "/work", "--proc", "/proc", "--dev", "/dev",
                   "--tmpfs", "/tmp", "--chdir", "/input", "--setenv", "LANG", "C.UTF-8",
                   "--", "/usr/bin/prlimit", "--as=4294967296", "--cpu=90", "--nproc=128",
                   "--nofile=256", "--fsize=16777216", "--", *command))
    return result


def run(command: list[str], output: Path) -> dict[str, object]:
    """Retain real exit/stdout; setup failures and timeouts cannot count as rejection."""
    started = time.monotonic()
    with output.open("xb") as stream:
        process = subprocess.Popen(command, stdin=subprocess.DEVNULL, stdout=stream,
                                   stderr=subprocess.STDOUT, env={}, start_new_session=True)
        try:
            exit_code = process.wait(timeout=120)
        except subprocess.TimeoutExpired as error:
            os.killpg(process.pid, 9)
            process.wait()
            raise Incomplete("typing subprocess timed out") from error
    if output.stat().st_size > 1024 * 1024:
        raise Incomplete("typing subprocess exceeded output budget")
    return {"command": command, "exit_code": exit_code,
            "stdout": output.read_text(encoding="utf-8"), "elapsed_seconds": time.monotonic() - started}


def qualifies(result: dict[str, object], fixture: str, error_code: str | None) -> bool:
    """Require the exact healthy admission or an intended diagnostic at its consumer."""
    text = str(result["stdout"])
    if error_code is None:
        return result["exit_code"] == 0 and text.strip() == "Success: no issues found in 9 source files"
    intended = any(fixture + ":" in line and "[" + error_code + "]" in line
                   for line in text.splitlines())
    foreign_failure = any("error:" in line and fixture + ":" not in line for line in text.splitlines())
    return result["exit_code"] == 1 and intended and not foreign_failure


def admit_tool(record: dict[str, object], checker: Path, tools: dict[str, str],
               policy: dict[str, object], policy_sha256: str) -> None:
    """Reuse prior installed-tool custody, independently binding current source below."""
    if (type(record.get("schema_version")) is not int or record["schema_version"] != 1
            or record.get("checker_root") != str(checker) or record.get("files") != tools
            or record.get("python_version") != policy["python_version"]
            or record.get("mypy_version") != policy["mypy_version"]
            or record.get("wheels") != policy["wheels"]
            or record.get("policy_sha256") != policy_sha256):
        raise Incomplete("installed checker differs from prior tool admission")


def execute(checker_python: Path, output: Path, tool_admission: Path | None = None) -> int:
    """Check production and legal consumer, then demand five intended rejections."""
    if not checker_python.is_absolute() or checker_python != checker_python.resolve(strict=True):
        raise Incomplete("checker Python must be an absolute unaliased regular path")
    checker = checker_python.parent.parent
    if checker == ROOT or ROOT in checker.parents:
        raise Incomplete("checker store must be external")
    if not output.is_absolute() or ROOT == output or ROOT in output.parents:
        raise Incomplete("evidence output must be external and absolute")
    output.mkdir(mode=0o700)
    policy_raw = (ROOT / "policy/python-typing.json").read_bytes()
    policy: dict[str, object] = json.loads(policy_raw)
    if policy["python_version"] != "3.12.3" or policy["mypy_version"] != "2.4.1":
        raise Incomplete("selected checker policy drift")
    tools = inventory(checker)
    selected_admission = tool_admission or checker.parent / "admission-v7.json"
    if (not selected_admission.is_absolute() or selected_admission != selected_admission.resolve(strict=True)
            or ROOT == selected_admission or ROOT in selected_admission.parents):
        raise Incomplete("tool admission must be an external unaliased file")
    admission_raw = selected_admission.read_bytes()
    admission: dict[str, object] = json.loads(admission_raw)
    admit_tool(admission, checker, tools, policy, digest(policy_raw))
    actual_sources = {str(path.relative_to(ROOT)) for path in (ROOT / "metadata/pipeline").glob("*.py")} | {"metadata/__init__.py"}
    if actual_sources != set(SOURCES):
        raise Incomplete("production source closure changed without typing review")
    names = (*SOURCES, "metadata/qualification/typing.py", "metadata/qualification/typing_tests.py",
             "metadata/qualification/typing_requirements.lock", CONFIG, *("metadata/qualification/" + name for name in FIXTURES))
    raw = {name: (ROOT / name).read_bytes() for name in names}
    if raw[CONFIG] != EXPECTED_CONFIG.encode("utf-8"):
        raise Incomplete("closed strict typing configuration differs")
    source_identity = {name: digest(value) for name, value in raw.items()}
    imports = set()
    for name in (*SOURCES, *("metadata/qualification/" + name for name in FIXTURES)):
        imports.update(inspect_source(name, raw[name]))
    cases = []
    passed = True
    with tempfile.TemporaryDirectory(prefix="metadata-typing-") as temporary:
        base = Path(temporary)
        inputs, work = base / "inputs", base / "work"
        inputs.mkdir(); work.mkdir()
        for name, value in raw.items():
            destination = inputs / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_bytes(value)
        version_code = (
            "import importlib.metadata,json,sys,mypy.version; "
            "print(json.dumps({'python':sys.version.split()[0],'mypy':mypy.version.__version__,"
            "'packages':{d.metadata['Name'].lower().replace('_','-'):d.version for d in importlib.metadata.distributions()}},sort_keys=True))"
        )
        version = run(sandbox(checker, inputs, work, ["/checker/bin/python", "-I", "-B", "-c", version_code]), output / "version.txt")
        if version["exit_code"] != 0:
            raise Incomplete("checker version control failed")
        versions: dict[str, object] = json.loads(str(version["stdout"]))
        wheels = policy["wheels"]
        if type(wheels) is not dict:
            raise Incomplete("checker wheel policy malformed")
        expected_packages = {name: item["version"] for name, item in wheels.items()}
        expected_lock = ("# Existing policy/python-typing.json closure; no automatic installation.\n" +
                         "".join(name + "==" + item["version"] + " --hash=sha256:" + item["sha256"] + "\n"
                                 for name, item in wheels.items()))
        if raw["metadata/qualification/typing_requirements.lock"] != expected_lock.encode("utf-8"):
            raise Incomplete("typing lock differs from admitted policy wheel graph")
        if (versions["python"] != policy["python_version"] or versions["mypy"] != policy["mypy_version"]
                or versions["packages"] != expected_packages):
            raise Incomplete("checker runtime/package identity differs from pinned policy")
        prefix = ["/checker/bin/python", "-I", "-B", "-m", "mypy", "--config-file", "/input/" + CONFIG]
        for fixture, error_code in FIXTURES.items():
            command = prefix + list(SOURCES) + ["metadata/qualification/typing_consumer.py"]
            if error_code:
                command.append("metadata/qualification/" + fixture)
            result = run(sandbox(checker, inputs, work, command), output / (fixture + ".txt"))
            valid = qualifies(result, fixture, error_code)
            result.update({"fixture": fixture, "expected_error_code": error_code, "qualified": valid})
            cases.append(result)
            passed = passed and valid
    if selected_admission.read_bytes() != admission_raw:
        raise Incomplete("tool admission changed during qualification")
    if (ROOT / "policy/python-typing.json").read_bytes() != policy_raw:
        raise Incomplete("checker policy changed during qualification")
    if inventory(checker) != tools or any((ROOT / name).read_bytes() != value for name, value in raw.items()):
        raise Incomplete("checker/source identity changed during qualification")
    report = {"schema_version": 1, "scope": "metadata-eight-production-modules-and-immediate-consumer",
              "status": "passed" if passed else "failed", "sources": source_identity,
              "policy_sha256": digest(policy_raw), "tool_files": tools, "tool_runtime": versions,
              "tool_version_control": version,
              "tool_admission_path": str(selected_admission), "tool_admission_sha256": digest(admission_raw),
              "tool_admission_scope": "exact-installed-files-and-policy; current metadata source bound separately",
              "imports": sorted(imports), "source_module_count": len(SOURCES),
              "healthy_consumer_count": 1, "negative_control_count": len(FIXTURES) - 1,
              "cases": cases, "runtime_validation": "separate-required-layer",
              "native_package_release": "not-run"}
    (output / "result.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    print(json.dumps({"status": report["status"], "source_module_count": len(SOURCES),
                      "negative_control_count": len(FIXTURES) - 1, "result": str(output / "result.json")}, sort_keys=True))
    return 0 if passed else 1


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--checker-python", type=Path, required=True)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--tool-admission", type=Path, help="prior installed-tool custody receipt; defaults to checker parent admission-v7.json")
    args = parser.parse_args()
    try:
        return execute(args.checker_python, args.output, args.tool_admission)
    except (Incomplete, OSError, ValueError) as error:
        print("Typing qualification incomplete: " + str(error))
        return 2


if __name__ == "__main__":
    raise SystemExit(main())
