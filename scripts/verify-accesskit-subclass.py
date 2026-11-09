#!/usr/bin/env python3
"""Run the actual AccessKit subclass module against its portable Win32 facade.

No Cargo graph, native Windows, network acquisition or host fallback is allowed.
This exercises the mock's stated API contract, not USER32/UIPI/COM/UIA behavior.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import stat
import subprocess
import sys
import time

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("subclass_product", ROOT / "scripts/verify.py")
PRODUCT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PRODUCT)
ENV = PRODUCT.ENV
SUBCLASS = "third-party/accesskit_windows/src/subclass.rs"
HARNESS = "third-party/accesskit_windows/tests/subclass_lifecycle_harness.rs"
PROJECTION = (SUBCLASS, HARNESS, "rustfmt.toml")
INPUTS = tuple(dict.fromkeys((*PROJECTION, "scripts/verify-accesskit-subclass.py",
                             *PRODUCT.CHECKER_FILES, "policy/tool-manifest.json", "rust-toolchain.toml")))


def require(condition, message):
    if not condition:
        raise ENV.Incomplete(message)


def read_source(path):
    """Bound each fixed repository input before hashing/copying; follow no final link."""
    descriptor = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK)
    with os.fdopen(descriptor, "rb") as stream:
        require(stat.S_ISREG(os.fstat(stream.fileno()).st_mode), "Source is not regular.")
        data = stream.read(4 * 1024 * 1024 + 1)
    require(len(data) <= 4 * 1024 * 1024, "Source exceeds its read budget.")
    return data


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--toolchain-dir", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    report = {"status": "incomplete", "scope": "actual subclass module with inert Win32/accesskit facade",
              "native_windows_execution": False, "cargo_executed": False, "commands": [],
              "limits": {"wall_seconds_per_command": 120, "output_bytes_per_command": ENV.MAX_OUTPUT},
              "lint_scope": "rustc deny-warnings only; no workspace Clippy/Windows dependency lint claim"}
    owned, before = None, None
    try:
        require(not sys.flags.optimize and not ENV.overridden_names(os.environ), "Ambient compiler override or optimized Python.")
        toolchain = args.toolchain_dir.expanduser().resolve(strict=True)
        destination = args.output_dir.expanduser().resolve()
        require(not destination.is_relative_to(ROOT) and not destination.exists(), "Need a fresh external output directory.")
        destination.mkdir(mode=0o700)
        owned = destination
        before = {name: hashlib.sha256(read_source(ROOT / name)).hexdigest() for name in INPUTS}
        report["source_sha256"] = before
        # Reuse base binary admission without adding nextest/Cargo or an MSVC job.
        tools = PRODUCT.admit_tools(ROOT, toolchain, None, "windows")
        report["tools_sha256"] = tools
        source, work = destination / "source", destination / "work"
        for folder in (source, work, work / "fixture", work / "target", work / "cargo-home"):
            folder.mkdir(mode=0o700)
        for name in PROJECTION:
            path = source / name
            path.parent.mkdir(parents=True, exist_ok=True)
            data = read_source(ROOT / name)
            require(hashlib.sha256(data).hexdigest() == before[name], "Source changed before projection.")
            path.write_bytes(data)

        def run(label, command):
            invocation = ENV.sandbox_command(toolchain, work, command)
            position = invocation.index("--")
            invocation[position:position] = ["--ro-bind", str(source), "/work/fixture",
                                           "--setenv", "RUST_TEST_THREADS", "1"]
            result = ENV.run(invocation, cwd=ROOT, timeout=120, max_output=ENV.MAX_OUTPUT)
            report["commands"].append({"label": label, **result})
            return ENV.require_ok(result)

        PRODUCT.boundary_control(run)
        policy = json.loads(read_source(ROOT / "policy/tool-manifest.json"))
        ENV.require_admitted_compiler(policy["rust"]["channel"], run("compiler", ["rustc", "-vV"]), policy)
        run("format", ["rustfmt", "--check", "--edition=2024", "--config-path", "/work/fixture/rustfmt.toml",
                       "/work/fixture/" + SUBCLASS, "/work/fixture/" + HARNESS])
        run("compile", ["rustc", "--test", "--edition=2024", "--deny", "warnings", "--target", PRODUCT.HOST_TARGET,
                        "/work/fixture/" + HARNESS, "-o", "/work/target/accesskit-subclass-controls"])
        listing = run("list", ["/work/target/accesskit-subclass-controls", "--list", "--color=never"])
        names = re.findall(r"^([A-Za-z_][A-Za-z_0-9:]*): test$", listing, re.MULTILINE)
        expected = re.findall(r"^#\[test\]\s+fn ([A-Za-z_][A-Za-z_0-9]*)\(", read_source(source / HARNESS).decode(), re.MULTILINE)
        require(11 <= len(names) <= 64 and len(set(names)) == len(names) and sorted(names) == sorted(expected)
                and re.search(rf"^{len(names)} tests, 0 benchmarks$", listing, re.MULTILINE), "Test discovery differs from admitted source.")
        output = run("tests", ["/work/target/accesskit-subclass-controls", "--test-threads=1", "--color=never"])
        passed = re.findall(r"^test ([A-Za-z_][A-Za-z_0-9:]*) \.\.\. ok$", output, re.MULTILINE)
        require(sorted(passed) == sorted(names) and len(passed) == len(names)
                and re.search(rf"test result: ok\. {len(names)} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;", output),
                "Actual execution omitted, ignored or failed a discovered test.")
        report["actual_tests"] = passed
        require(all(hashlib.sha256(read_source(source / name)).hexdigest() == before[name] for name in PROJECTION),
                "Projected source changed during verification.")
        require(PRODUCT.admit_tools(ROOT, toolchain, None, "windows") == tools, "Admitted tools changed.")
        report["status"] = "passed"
    except (OSError, ValueError, KeyError, RuntimeError, subprocess.SubprocessError) as error:
        report["error"] = str(error)
    finally:
        if before is not None:
            try:
                report["source_bookends_passed"] = before == {name: hashlib.sha256(read_source(ROOT / name)).hexdigest() for name in INPUTS}
            except (OSError, RuntimeError):
                report["source_bookends_passed"] = False
            if not report["source_bookends_passed"]:
                report.update(status="incomplete", error="Source changed during verification.")
        report["finished_unix"] = time.time()
        if owned is not None:
            (owned / "result.json").write_text(json.dumps(report, indent=2) + "\n")
    print(report["status"].upper() + ": " + report["scope"])
    if "error" in report:
        print(report["error"][:3000], file=sys.stderr)
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
