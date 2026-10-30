#!/usr/bin/env python3
"""Run pinned workflow analyzers, including actual healthy/violation controls.

Downloads go to an owned temporary directory; no shared tools are installed.
--tools-dir reuses already acquired executables only after digest admission.
Analysis has no credentials and uses no user config, ignores or severity floors.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import platform
import re
import sys
import tarfile
import tempfile
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("workflow_environment", ROOT / "scripts/check-environment.py")
ENV = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENV)
PIN_FILE = Path(__file__).with_name("workflow-tool-pins.json")
MAX_ARCHIVE = 32 * 1024 * 1024
MAX_BINARY = 64 * 1024 * 1024


def acquire(entry: dict, destination: Path) -> Path:
    """Admit the archive before parsing and copy exactly one regular file."""
    archive = destination / (entry["name"] + ".archive")
    if not entry["url"].startswith("https://github.com/"):
        raise ENV.Incomplete("Workflow tool acquisition requires the pinned HTTPS origin.")
    with urllib.request.urlopen(entry["url"], timeout=30) as response:
        data = response.read(MAX_ARCHIVE + 1)
    if len(data) > MAX_ARCHIVE:
        raise ENV.Incomplete("Workflow tool archive exceeds its acquisition budget.")
    if hashlib.sha256(data).hexdigest() != entry["archive_sha256"]:
        raise ENV.Incomplete("Workflow tool archive digest mismatch.")
    archive.write_bytes(data)
    binary = destination / entry["name"]
    with tarfile.open(archive) as bundle:
        matches = [member for member in bundle if member.name == entry["member"]]
        if len(matches) != 1 or not matches[0].isfile() or matches[0].size > MAX_BINARY:
            raise ENV.Incomplete("Workflow tool must be one bounded regular archive member.")
        with bundle.extractfile(matches[0]) as source:
            payload = source.read(MAX_BINARY + 1)
    if hashlib.sha256(payload).hexdigest() != entry["binary_sha256"]:
        raise ENV.Incomplete("Workflow tool binary digest mismatch.")
    binary.write_bytes(payload)
    binary.chmod(0o700)
    return binary


def admit(entry: dict, directory: Path) -> Path:
    binary = (directory / entry["name"]).resolve(strict=True)
    ENV.require_admitted_binary(binary, entry["binary_sha256"], entry["name"])
    return binary


def analyze(tools: dict, root: Path, scratch: Path) -> None:
    # actionlint has no no-config switch: supply an explicit empty config.
    config = scratch / "actionlint.yaml"
    config.write_text("{}\n", encoding="utf-8")
    wrapper = scratch / "shellcheck-strict"
    wrapper.write_text(
        f"#!{sys.executable}\nimport os, sys\n"
        f"os.execv({str(tools['shellcheck'])!r}, "
        f"[{str(tools['shellcheck'])!r}, '--norc', '--enable=all', '--severity=style', *sys.argv[1:]])\n",
        encoding="utf-8",
    )
    wrapper.chmod(0o700)
    # Private HOME prevents additional git/analyzer configuration discovery.
    # Explicit --offline is important: no ambient GitHub token reaches analysis.
    # Bound Go's actionlint scheduler; external ShellCheck processes are separate.
    environment = {"PATH": "/usr/bin:/bin", "HOME": str(scratch), "LANG": "C.UTF-8",
                   "GOMAXPROCS": "12"}
    actionlint = [str(tools["actionlint"]), "-oneline", "-config-file", str(config),
                  "-shellcheck", str(wrapper), "-pyflakes="]
    zizmor = [str(tools["zizmor"]), "--offline", "--pedantic", "--no-config",
              "--no-ignores", "--no-progress", "--format=plain"]

    def run(command: list[str], expected: str | None = None) -> None:
        result = ENV.run(command, cwd=root, timeout=90, environment=environment)
        if expected is None:
            output = ENV.require_ok(result)
            if re.search(r"(?im)^warning(?:\[|:)", output):
                raise ENV.Incomplete("Workflow analyzer succeeded with a warning.")
        elif result["failure"] or result["exit_code"] == 0 or expected not in result["output"]:
            raise ENV.Incomplete(f"Workflow analyzer failed its {expected} violation control.")

    for name, args in (("actionlint", ["-version"]), ("zizmor", ["--version"]),
                       ("shellcheck", ["--version"])):
        run([str(tools[name]), *args])

    fixture = scratch / "control.yml"
    healthy = (
        "name: Control\non: push\npermissions: {}\n"
        "concurrency:\n  group: control-${{ github.ref }}\n  cancel-in-progress: true\njobs:\n"
        "  check:\n    name: Control\n    runs-on: ubuntu-24.04\n    timeout-minutes: 1\n"
        "    steps:\n      - name: Observe\n        run: echo 'healthy'\n"
    )
    fixture.write_text(healthy, encoding="utf-8")
    run([*actionlint, str(fixture)])
    run([*zizmor, str(fixture)])
    fixture.write_text(healthy.replace("ubuntu-24.04", "unknown-runner"), encoding="utf-8")
    run([*actionlint, str(fixture)], "runner-label")
    fixture.write_text(healthy.replace("echo 'healthy'", "echo $unquoted"), encoding="utf-8")
    run([*actionlint, str(fixture)], "SC2086")
    fixture.write_text(healthy.replace("echo 'healthy'", "echo '${{ github.event.head_commit.message }}'"), encoding="utf-8")
    run([*zizmor, str(fixture)], "template-injection")

    workflows = sorted([*(root / ".github/workflows").glob("*.yml"),
                        *(root / ".github/workflows").glob("*.yaml")])
    if not workflows:
        raise ENV.Incomplete("No workflow inputs; empty analysis is not a passing gate.")
    run([*actionlint, *(str(path) for path in workflows)])
    run([*zizmor, *(str(path) for path in workflows)])
    print(f"PASSED: {len(workflows)} workflow(s), actionlint + ShellCheck + pedantic offline zizmor; "
          "2 healthy and 3 rejection controls. Online audits and Python lint helpers are not run.")


def main(argv=None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tools-dir", type=Path, help="reuse digest-admitted Linux x64 binaries")
    args = parser.parse_args(argv)
    try:
        if sys.platform != "linux" or platform.machine() != "x86_64":
            raise ENV.Incomplete("Pinned workflow analyzer binaries require Linux x64.")
        manifest = json.loads(PIN_FILE.read_text(encoding="utf-8"))
        if manifest["schema_version"] != 1 or manifest["platform"] != "x86_64-linux":
            raise ENV.Incomplete("Unsupported workflow tool manifest.")
        entries = {entry["name"]: entry for entry in manifest["tools"]}
        if set(entries) != {"actionlint", "zizmor", "shellcheck"} or len(manifest["tools"]) != 3:
            raise ENV.Incomplete("Workflow tool selection must be complete and unique.")
        with tempfile.TemporaryDirectory(prefix="nevrotick-workflow-check-") as temporary:
            scratch = Path(temporary)
            directory = args.tools_dir or scratch
            if args.tools_dir is None:
                for entry in entries.values():
                    acquire(entry, directory)
            tools = {name: admit(entry, directory) for name, entry in entries.items()}
            analyze(tools, ROOT, scratch)
    except (ENV.Incomplete, OSError, ValueError, KeyError, tarfile.TarError) as error:
        print(f"INCOMPLETE: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
