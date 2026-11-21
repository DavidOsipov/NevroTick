"""Link and independently check one admitted MD-P4 event resource DLL on WSL."""

from __future__ import annotations

import argparse
import ctypes
import errno
import hashlib
import importlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile

ROOT = Path(__file__).absolute().parent.parent
if __package__ in (None, ""):
    sys.path.insert(0, str(ROOT))

from metadata.pipeline.contract import MetadataError, load_metadata  # noqa: E402
from metadata.pipeline.delivery import _json  # noqa: E402
from metadata.event_resource_events import verify_event_resources  # noqa: E402
from metadata.event_resource_pe import read_resource_dll, read_resource_object  # noqa: E402
from metadata.pipeline.generate import (  # noqa: E402
    PublicationUncertainError, _expected, _new_staging_directory, _open_directory,
    _outside_source, _parent_unchanged, _read_regular, _remove_staging, _write_payload, check,
)

ENV = importlib.import_module("scripts.check-environment")
ANALYSIS = importlib.import_module("scripts.check-analysis")
PROFILE = "windows-sdk-10.0.28000.2705-amd64"
SDK_TOOLS = {
    "mc.exe": "dddc00b028a88051c8994fffee08f6c76db5efd538a21a5698a43880b9d543c9",
    "rc.exe": "89c1cf330befb84d617f91f6ba56e618c95f8b5b6feb3176d575d9960f90a863",
    "rcdll.dll": "c1fcbec6c9912a6b2826d6dfce2b12f84ac517febe7b5dacf6563bc099bec2c2",
    "ServicingCommon.dll": "35aba8c878650c1fb1dacf43caa40fdaa99ab5c9d200163fe486810b27611535",
}
COMPILER_COMMANDS = {
    "mc": ("mc.exe", "-um service-events.man"),
    "rc": ("rc.exe", "/x /fo service-events.res service-events.rc"),
}
MANIFEST = "authoring/service-events.man"
RESOURCE_MAX = 64 * 1024
TARGET = "x86_64-pc-windows-msvc"
LINK_SECONDS = 15
LINK_FLAGS = ("-flavor", "link", "/dll", "/noentry", "/machine:x64", "/nodefaultlib",
              "/manifest:no", "/noimplib", "/timestamp:0", "/threads:1", "/WX")
BUILD_TOOLS = (
    "metadata/build_event_resource.py",
    "metadata/event_resource_events.py",
    "metadata/event_resource_pe.py",
    "scripts/check-environment.py",
    "scripts/check-analysis.py",
    "scripts/analysis-runtime.py",
    "scripts/analysis-controls.py",
    "policy/tool-manifest.json",
)


def _digest(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def _read(path: Path, maximum: int) -> bytes:
    directory = _open_directory(path.parent)
    try:
        raw = _read_regular(directory, path.name, maximum)
        _parent_unchanged(path.parent, directory)
        return raw
    finally:
        os.close(directory)


def _compiler_artifacts(
    receipt: bytes, receipt_sha256: str, manifest: bytes,
) -> dict[str, tuple[int, str]]:
    """Reconcile an externally admitted MC/RC receipt with the checked manifest."""
    if _digest(receipt) != receipt_sha256:
        raise MetadataError("event-compiler-receipt-digest")
    record = _json(receipt)
    expected_inputs = {**SDK_TOOLS, "service-events.man": _digest(manifest)}
    if (type(record) is not dict or record.get("status") != "compiled"
            or record.get("failure", False) is not None
            or record.get("input_sha256") != expected_inputs
            or record.get("copied_inputs_rechecked") is not True
            or record.get("scratch_cleanup") != "removed"):
        raise MetadataError("event-compiler-receipt-inputs")
    commands = record.get("commands")
    if type(commands) is not dict or commands.keys() != COMPILER_COMMANDS.keys():
        raise MetadataError("event-compiler-receipt-commands")
    for phase, (image, arguments) in COMPILER_COMMANDS.items():
        command = commands[phase]
        if (type(command) is not dict or command.get("image") != image
                or command.get("arguments") != arguments
                or type(command.get("pid")) is not int or command["pid"] <= 0
                or type(command.get("exit_code")) is not int or command["exit_code"] != 0
                or command.get("reaped") is not True or command.get("timed_out") is not False
                or command.get("kill_attempted") is not False
                or command.get("error", False) is not None):
            raise MetadataError("event-compiler-receipt-commands")

    names = {"service-events.res", "MSG00001.bin", "service-eventsTEMP.BIN",
             "service-events.h", "service-events.rc"}
    artifacts = record.get("artifacts")
    if (type(artifacts) is not list or len(artifacts) != len(names)
            or type(record.get("artifact_count")) is not int or record["artifact_count"] != len(names)):
        raise MetadataError("event-compiler-receipt-artifacts")
    inventory: dict[str, tuple[int, str]] = {}
    for item in artifacts:
        if type(item) is not dict or item.keys() != {"name", "bytes", "sha256"}:
            raise MetadataError("event-compiler-receipt-artifacts")
        name, size, digest = item["name"], item["bytes"], item["sha256"]
        if (type(name) is not str or name not in names or name in inventory
                or type(size) is not int or not 0 < size <= RESOURCE_MAX
                or type(digest) is not str or not re.fullmatch(r"[0-9a-f]{64}", digest)):
            raise MetadataError("event-compiler-receipt-artifacts")
        inventory[name] = (size, digest)
    return inventory


def _linker() -> tuple[Path, str, tuple[Path, ...], str]:
    """Select the repository's pinned Linux linker and existing isolation tools."""
    policy = _json(_read(ROOT / "policy/tool-manifest.json", RESOURCE_MAX))
    rust = policy["rust"]
    if rust["host"] != "x86_64-unknown-linux-gnu" or rust["native_target"] != TARGET:
        raise MetadataError("event-linker-target")
    admitted = ANALYSIS.admit(ROOT, profile="stable-core")
    linker = admitted["stable"] / "lib/rustlib" / rust["host"] / "bin/rust-lld"
    expected = policy["cross_toolchain"]["linker"]["binary_sha256"]
    if _digest(_read(linker, 64 * 1024 * 1024)) != expected:
        raise MetadataError("event-linker-digest")
    identity = _digest(json.dumps(
        [admitted[key] for key in ("policy_sha256", "installation_pointer_sha256", "inventories")],
        sort_keys=True,
    ).encode("utf-8"))
    return linker, expected, admitted["readonly_roots"], identity


def _link(linker: Path, readonly: tuple[Path, ...], resource: bytes, name: str, parent: Path) -> bytes:
    """Run the selected resource-only link without Cargo, SDK libraries or imports."""
    with tempfile.TemporaryDirectory(prefix=".nevrotick-event-link-", dir=parent) as scratch:
        work = Path(scratch)
        fixture = work / "fixture"
        fixture.mkdir(mode=0o700)
        link_input = fixture / "service-events.res"
        link_input.write_bytes(resource)
        argv = [str(linker), *LINK_FLAGS, "/out:" + name, "service-events.res"]
        command = ANALYSIS.RUNTIME.sandbox_command(
            ROOT, work, argv, readonly=readonly, sealed=(link_input,), cwd=fixture,
            cpu_seconds=10, address_space_bytes=1024 ** 3, build_jobs=1,
        )
        observation = ENV.run(command, cwd=work, timeout=LINK_SECONDS, max_output=16 * 1024,
                              environment={"LANG": "C.UTF-8", "PATH": "/usr/bin:/bin"},
                              stdin=subprocess.DEVNULL)
        if observation["exit_code"] != 0 or observation["failure"] is not None:
            raise MetadataError("event-link-failed")
        if set(os.listdir(fixture)) != {"service-events.res", name}:
            raise MetadataError("event-link-output-set")
        if _read(link_input, RESOURCE_MAX) != resource:
            raise MetadataError("event-link-input-changed")
        return _read(fixture / name, RESOURCE_MAX)


def _require_absent(parent: int, name: str) -> None:
    try:
        os.stat(name, dir_fd=parent, follow_symlinks=False)
    except FileNotFoundError:
        return
    raise MetadataError("event-output-already-exists")


def _publish_new(parent: int, staging: str, name: str) -> None:
    """Atomically publish the two-file tree without replacing a concurrent output."""
    # Linux renameat2(RENAME_NOREPLACE) keeps the parent-FD and absent-target
    # requirements in one operation. A filesystem without support fails closed.
    try:
        rename = ctypes.CDLL(None, use_errno=True).renameat2
    except AttributeError as error:
        raise MetadataError("event-atomic-publication-unavailable") from error
    rename.argtypes = [ctypes.c_int, ctypes.c_char_p, ctypes.c_int, ctypes.c_char_p, ctypes.c_uint]
    rename.restype = ctypes.c_int
    rename_noreplace = 1
    if rename(parent, os.fsencode(staging), parent, os.fsencode(name), rename_noreplace) != 0:
        if ctypes.get_errno() == errno.EEXIST:
            raise MetadataError("event-output-already-exists")
        raise MetadataError("event-atomic-publication-failed")


def build_event_resource(
    root: Path, source: Path | None, generation: Path, resource_file: Path,
    compiler_receipt: Path, compiler_sha256: str, output: Path,
) -> dict[str, object]:
    """Publish only an independently matched DLL and receipt in a fresh directory."""
    output = _outside_source(ROOT, output)
    for protected in (root, generation):
        _outside_source(protected, output)
    model = load_metadata(root, source)
    provider = model.event_provider
    if provider is None:
        raise MetadataError("event-provider-unselected")
    expected = _expected(model)
    check(root, generation, source)
    tools = {name: _digest(_read(ROOT / name, RESOURCE_MAX)) for name in BUILD_TOOLS}
    receipt = _read(compiler_receipt, RESOURCE_MAX)
    inventory = _compiler_artifacts(receipt, compiler_sha256, expected[MANIFEST])
    resource = _read(resource_file, RESOURCE_MAX)
    if inventory["service-events.res"] != (len(resource), _digest(resource)):
        raise MetadataError("event-compiler-resource-digest")
    leaves = read_resource_object(resource)
    for key, name in (((11, 1, 1033), "MSG00001.bin"),
                      (("WEVT_TEMPLATE", 1, 1033), "service-eventsTEMP.BIN")):
        leaf = leaves[key]
        if inventory[name] != (len(leaf), _digest(leaf)):
            raise MetadataError("event-compiler-resource-leaf-digest")
    verify_event_resources(provider, leaves)
    parent = _open_directory(output.parent, private=True)
    temporary: str | None = None
    staging: int | None = None
    try:
        _require_absent(parent, output.name)
        linker, linker_sha256, readonly, admission_sha256 = _linker()
        _parent_unchanged(output.parent, parent)
        image = _link(linker, readonly, resource, provider.package_resource_file, output.parent)
        observed_leaves = read_resource_dll(image)
        if observed_leaves != leaves:
            raise MetadataError("event-linked-resource-leaves")
        observed_events = verify_event_resources(provider, observed_leaves)
        report = {
            "schema_version": 1, "status": "linked-and-read-back", "target": TARGET,
            "sdk_profile": PROFILE, "compiler_receipt_sha256": compiler_sha256,
            "manifest_sha256": _digest(expected[MANIFEST]),
            "generation_receipt_sha256": _digest(expected["receipt.json"]),
            "resource_sha256": _digest(resource), "linker_sha256": linker_sha256,
            "tool_admission_sha256": admission_sha256,
            "link": {"flags": LINK_FLAGS, "exit_code": 0, "wall_budget_seconds": LINK_SECONDS},
            "tool_sources": tools, "file": provider.package_resource_file,
            "bytes": len(image), "sha256": _digest(image), "readback": observed_events,
        }
        temporary, staging = _new_staging_directory(parent)
        _write_payload(staging, provider.package_resource_file, image)
        _write_payload(staging, "receipt.json", (json.dumps(report, sort_keys=True) + "\n").encode("utf-8"))
        if (load_metadata(root, source) != model or _expected(model) != expected
                or _read(compiler_receipt, RESOURCE_MAX) != receipt
                or _read(resource_file, RESOURCE_MAX) != resource
                or _linker() != (linker, linker_sha256, readonly, admission_sha256)
                or any(_digest(_read(ROOT / name, RESOURCE_MAX)) != digest
                       for name, digest in tools.items())):
            raise MetadataError("event-build-inputs-changed")
        check(root, generation, source)
        _parent_unchanged(output.parent, parent)
        _publish_new(parent, temporary, output.name)
        temporary = None
        try:
            os.fsync(parent)
        except OSError as error:
            raise PublicationUncertainError() from error
        return report
    finally:
        try:
            if temporary is not None and staging is not None:
                _remove_staging(parent, temporary, staging)
        finally:
            if staging is not None:
                os.close(staging)
            os.close(parent)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    parser.add_argument("--source", type=Path)
    parser.add_argument("--generation", type=Path, required=True)
    parser.add_argument("--resource-object", type=Path, required=True)
    parser.add_argument("--compiler-receipt", type=Path, required=True)
    parser.add_argument("--compiler-receipt-sha256", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    try:
        report = build_event_resource(
            args.root, args.source, args.generation, args.resource_object, args.compiler_receipt,
            args.compiler_receipt_sha256, args.output,
        )
    except PublicationUncertainError:
        print(json.dumps({"schema_version": 1, "status": "publication-uncertain",
                          "durability": "unconfirmed",
                          "action": "check-existing-output-before-retry"}, sort_keys=True))
        return 3
    except MetadataError as error:
        # Pipeline errors have closed codes. Parser/OS diagnostics and paths
        # are deliberately omitted from this packaging-stage interface.
        reason = str(error)
        print("Event resource error: " + (reason if re.fullmatch(r"[a-z][a-z0-9-]{0,99}", reason)
                                          else "invalid-input"), file=sys.stderr)
        return 1
    except (ANALYSIS.Incomplete, ENV.Incomplete, subprocess.SubprocessError,
            KeyError, OSError, RecursionError, TypeError, ValueError):
        print("Event resource error: input-or-environment-invalid", file=sys.stderr)
        return 1
    print(json.dumps(report, sort_keys=True, allow_nan=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
