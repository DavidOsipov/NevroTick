#!/usr/bin/env python3
"""Generate a frozen, offline Rust dependency inventory in external storage.

This creates crate-level CycloneDX files and an input receipt. It does not scan
native/package payloads, authorize publication or claim a complete product SBOM.
"""

from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import tomllib
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("sbom_environment", ROOT / "scripts/check-environment.py")
ENV = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENV)
POLICY = {"schema_version": 1, "generator": "cargo-cyclonedx", "format": "json",
          "spec_version": "1.5", "describe": "crate", "target": "x86_64-pc-windows-msvc",
          "all_transitive_dependencies": True, "include_build_dependencies": True,
          "strict_license_parsing": True, "offline": True, "maximum_snapshot_bytes": 268435456}
MAX_REPORT_BYTES = 16 * 1024 * 1024
MAX_TOTAL_REPORT_BYTES = 64 * 1024 * 1024
MAX_REPORTS = 128
MAX_REPORT_DEPTH = 64
MAX_REPORT_NODES = 262_144
MAX_REPORT_NUMBER_LENGTH = 128


def read_report(path: Path, remaining: int) -> bytes:
    if path.is_symlink() or not path.is_file() or remaining <= 0:
        raise ENV.Incomplete("SBOM report must be a regular file within its byte budget.")
    limit = min(MAX_REPORT_BYTES, remaining)
    with path.open("rb") as source:
        data = source.read(limit + 1)
    if len(data) > limit:
        raise ENV.Incomplete("SBOM report exceeds its byte budget.")
    return data


def bound_report_structure(text: str) -> None:
    """Bound depth and token starts before allocating decoded JSON containers."""
    depth: int = 0
    nodes: int = 0
    quoted: bool = False
    escaped: bool = False
    scalar: bool = False
    for character in text:
        if quoted:
            if escaped:
                escaped = False
            elif character == "\\":
                escaped = True
            elif character == '"':
                quoted = False
            continue
        if character == '"':
            quoted = True
            nodes += 1
        elif character in "[{":
            depth += 1
            nodes += 1
        elif character in "]}":
            depth -= 1
        elif character not in " \t\r\n,:":
            if not scalar:
                nodes += 1
            scalar = True
            if nodes > MAX_REPORT_NODES:
                raise ENV.Incomplete("SBOM report exceeds its JSON node bound.")
            continue
        scalar = False
        if not 0 <= depth <= MAX_REPORT_DEPTH:
            raise ENV.Incomplete("SBOM report exceeds its JSON nesting bound or is unbalanced.")
        if nodes > MAX_REPORT_NODES:
            raise ENV.Incomplete("SBOM report exceeds its JSON node bound.")
    if depth or quoted:
        raise ENV.Incomplete("SBOM report is not a complete JSON document.")


def decode_report(data: bytes) -> dict[str, Any]:
    """Admit one bounded UTF-8 JSON object and its consumed CycloneDX fields."""
    if not 0 < len(data) <= MAX_REPORT_BYTES:
        raise ENV.Incomplete("SBOM report is empty or exceeds its byte budget.")

    def unique_fields(pairs: list[tuple[str, Any]]) -> dict[str, Any]:
        result: dict[str, Any] = {}
        for key, value in pairs:
            if key in result:
                raise ENV.Incomplete("SBOM report contains a duplicate JSON field.")
            result[key] = value
        return result

    def reject_nonfinite(_token: str) -> None:
        raise ENV.Incomplete("SBOM report contains a nonfinite JSON number.")

    def number(token: str, *, floating: bool) -> int | float:
        if len(token) > MAX_REPORT_NUMBER_LENGTH:
            raise ENV.Incomplete("SBOM report exceeds its JSON number length bound.")
        if not floating:
            return int(token)
        converted: float = float(token)
        if not math.isfinite(converted):
            reject_nonfinite(token)
        return converted

    try:
        text: str = data.decode("utf-8")
        bound_report_structure(text)
        decoded: Any = json.loads(text, object_pairs_hook=unique_fields,
                                  parse_constant=reject_nonfinite,
                                  parse_int=lambda token: number(token, floating=False),
                                  parse_float=lambda token: number(token, floating=True))
    except (ValueError, RecursionError) as error:
        raise ENV.Incomplete("SBOM report is not one complete strict UTF-8 JSON document.") from error
    validate_bom(decoded)
    return decoded


def validate_bom(bom: object) -> None:
    """Check the consumed structure/closure; this is not full schema validation."""
    if (type(bom) is not dict or bom.get("bomFormat") != "CycloneDX" or bom.get("specVersion") != "1.5"
            or type(bom.get("version")) is not int or bom["version"] < 1):
        raise ENV.Incomplete("Expected a versioned CycloneDX 1.5 JSON document.")
    metadata: object = bom.get("metadata")
    if type(metadata) is not dict or type(metadata.get("component")) is not dict:
        raise ENV.Incomplete("SBOM lacks an identified root crate.")
    product: dict[str, Any] = metadata["component"]
    components: object = bom.get("components", [])
    if type(components) is not list:
        raise ENV.Incomplete("SBOM components must be an array.")

    def nonempty_string(value: object) -> bool:
        return type(value) is str and bool(value)

    references: set[str] = set()
    for component in [product, *components]:
        if type(component) is not dict or not all(nonempty_string(component.get(field))
                                                  for field in ("bom-ref", "name", "version")):
            raise ENV.Incomplete("SBOM component identity must contain nonempty strings.")
        reference: str = component["bom-ref"]
        if reference in references:
            raise ENV.Incomplete("SBOM component reference is duplicated.")
        references.add(reference)
        children: object = component.get("components", [])
        if type(children) is not list or any(type(child) is not dict for child in children):
            raise ENV.Incomplete("SBOM nested components must be an array of objects.")
        for child in children:
            if "bom-ref" in child:
                child_ref: object = child["bom-ref"]
                if not nonempty_string(child_ref) or child_ref in references:
                    raise ENV.Incomplete("SBOM nested component reference is invalid or duplicated.")
                references.add(child["bom-ref"])
    dependencies: object = bom.get("dependencies")
    if type(dependencies) is not list or not dependencies:
        raise ENV.Incomplete("SBOM dependency graph is empty.")
    for edge in dependencies:
        if (type(edge) is not dict or not nonempty_string(edge.get("ref"))
                or type(edge.get("dependsOn", [])) is not list
                or any(not nonempty_string(ref) for ref in edge.get("dependsOn", []))):
            raise ENV.Incomplete("SBOM dependency entry has an invalid reference or edge list.")
        if edge["ref"] not in references or any(ref not in references for ref in edge.get("dependsOn", [])):
            raise ENV.Incomplete("SBOM dependency graph contains a dangling reference.")
    if product["bom-ref"] not in {edge["ref"] for edge in dependencies}:
        raise ENV.Incomplete("SBOM dependency graph omits its root crate.")


def source_snapshot(root: Path) -> dict[str, bytes]:
    result = ENV.run(["git", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], cwd=root)
    paths = ENV.require_ok(result).split("\0")
    snapshot = {}
    total = 0
    for name in paths:
        if not name:
            continue
        path = root / name
        if path.is_symlink() or not path.resolve().is_relative_to(root):
            raise ENV.Incomplete("SBOM input snapshot cannot follow source symlinks.")
        with path.open("rb") as source:
            data = source.read(POLICY["maximum_snapshot_bytes"] - total + 1)
        total += len(data)
        if total > POLICY["maximum_snapshot_bytes"]:
            raise ENV.Incomplete("SBOM input snapshot exceeds its declared byte budget.")
        snapshot[name] = data
    if not {"Cargo.toml", "Cargo.lock"}.issubset(snapshot):
        raise ENV.Incomplete("SBOM generation requires the real Cargo.toml and Cargo.lock.")
    return snapshot


def generate(root: Path, output: Path, features: list[str], no_default: bool) -> None:
    if not (root / "Cargo.toml").is_file() or not (root / "Cargo.lock").is_file():
        raise ENV.Incomplete("No application Cargo.toml/Cargo.lock exists; product SBOM generation is pending.")
    if output.exists() or output.is_relative_to(root):
        raise ENV.Incomplete("Use a new output directory outside the checkout.")
    if any(not re.fullmatch(r"[A-Za-z0-9_./-]+", feature) for feature in features):
        raise ENV.Incomplete("Invalid explicit Cargo feature selection.")
    config = tomllib.loads((root / "policy/sbom.toml").read_text())
    if config != POLICY:
        raise ENV.Incomplete("SBOM configuration differs from the admitted policy.")
    metadata = ENV.inspect(root)
    entries = json.loads((root / "policy/tool-manifest.json").read_text())["release_tools"]
    entry = next(item for item in entries if item["name"] == POLICY["generator"])
    tool = Path(shutil.which(entry["name"]) or "/missing-cargo-cyclonedx").resolve()
    ENV.require_admitted_binary(tool, entry["binary_sha256"], entry["name"])
    inputs = source_snapshot(root)
    revision = ENV.run(["git", "show", "-s", "--format=%H %ct", "HEAD"], cwd=root)
    identity = ENV.require_ok(revision).split() if revision["exit_code"] == 0 else []
    commit, epoch = identity if len(identity) == 2 else (None, "0")
    # No generator output is written beside user manifests. Cargo metadata and
    # the generator see the same private snapshot, immutable lock, cache and row.
    with tempfile.TemporaryDirectory(prefix="nevrotick-sbom-") as temporary:
        work = Path(temporary)
        fixture = work / "fixture"
        for name, data in inputs.items():
            path = fixture / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        (work / "cargo-home").mkdir()
        toolchain = Path(metadata["tools"]["rustc"]["path"]).parent.parent
        cargo_home = Path(os.environ.get("CARGO_HOME", Path.home() / ".cargo")).resolve()
        flags = (["--no-default-features"] if no_default else [])
        if features:
            flags += ["--features", ",".join(features)]

        def execute(command):
            args = ENV.sandbox_command(toolchain, work, command)
            mounts = ["--dir", "/tools", "--ro-bind", str(tool), "/tools/cargo-cyclonedx",
                      "--ro-bind", str(fixture / "Cargo.lock"), "/work/fixture/Cargo.lock",
                      "--setenv", "CARGO_NET_OFFLINE", "true", "--setenv", "SOURCE_DATE_EPOCH", epoch]
            for name in ("registry", "git"):
                if (cargo_home / name).is_dir():
                    mounts += ["--ro-bind", str(cargo_home / name), "/work/cargo-home/" + name]
            args[args.index("--"):args.index("--")] = mounts
            result = ENV.run(args, cwd=root, timeout=120)
            text = ENV.require_ok(result)
            if re.search(r"(?im)^(?:warning(?:\[|:)|\[[^\]\n]*\bWARN\b|WARN\b)", text):
                raise ENV.Incomplete("SBOM analysis reported a warning; inventory is incomplete.")
            return text

        graph = json.loads(execute(["cargo", "metadata", "--frozen", "--format-version", "1",
                                   "--filter-platform", POLICY["target"], *flags]))
        members = graph["workspace_members"]
        expected_members = {item["id"]: (item["name"], item["version"])
                            for item in graph["packages"] if item["id"] in members}
        if not members or len(set(members)) != len(members) or set(expected_members) != set(members):
            raise ENV.Incomplete("Cargo metadata has no complete unique workspace inventory.")
        execute(["/tools/cargo-cyclonedx", "cyclonedx", "--manifest-path", "/work/fixture/Cargo.toml",
                 "--format", POLICY["format"], "--spec-version", POLICY["spec_version"],
                 "--describe", POLICY["describe"], "--all", "--license-strict",
                 "--target", POLICY["target"], "--target-in-filename", *flags])
        if any((fixture / name).read_bytes() != data for name, data in inputs.items()):
            raise ENV.Incomplete("SBOM generator changed its source/lockfile snapshot.")
        generated = {}
        seen_members = set()
        report_bytes = 0
        for path in fixture.rglob("*.json"):
            if path.relative_to(fixture).as_posix() in inputs:
                continue
            if len(generated) >= MAX_REPORTS:
                raise ENV.Incomplete("SBOM report count exceeds its budget.")
            data = read_report(path, MAX_TOTAL_REPORT_BYTES - report_bytes)
            report_bytes += len(data)
            bom: dict[str, Any] = decode_report(data)
            product = bom["metadata"]["component"]
            member = product["bom-ref"]
            if (member not in expected_members or member in seen_members
                    or (product["name"], product["version"]) != expected_members[member]):
                raise ENV.Incomplete("SBOM root is duplicated or differs from its workspace member.")
            seen_members.add(member)
            tree = execute(["cargo", "tree", "--frozen", "--target", POLICY["target"],
                            "--edges", "normal,build", "--prefix", "none", "--format", "{p}",
                            "--package", product["name"] + "@" + product["version"], *flags])
            expected = set()
            for line in tree.splitlines():
                if line in {"[build-dependencies]", "[proc-macro]"}:
                    continue
                match = re.match(r"^([A-Za-z0-9_-]+) v([^\s]+)(?:\s|$)", line)
                if not match:
                    raise ENV.Incomplete("Cargo tree returned an unrecognized package identity.")
                expected.add(match.groups())
            observed = {(item["name"], item["version"]) for item in [product, *bom.get("components", [])]}
            if not expected or observed != expected:
                raise ENV.Incomplete("SBOM components disagree with the actual target/feature Cargo tree; inventory withheld.")
            generated[path.relative_to(fixture).as_posix()] = data
        if seen_members != set(expected_members):
            raise ENV.Incomplete("Generator omitted an intended workspace member inventory.")
        if source_snapshot(root) != inputs:
            raise ENV.Incomplete("Source changed during SBOM generation; inventory is stale.")
        output.mkdir(mode=0o700)
        for name, data in generated.items():
            path = output / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        receipt = {"scope": ("Rust dependency inventory only. Reconcile native and generated files "
                             "and resources with the built, signed and directly distributed MSIX "
                             "and the installed payload."),
                   "source_commit": commit, "source_date_epoch": int(epoch),
                   "generator": entry, "target": POLICY["target"], "features": features,
                   "default_features": not no_default,
                   "inputs_sha256": {name: hashlib.sha256(data).hexdigest() for name, data in inputs.items()},
                   "sbom_sha256": {name: hashlib.sha256(data).hexdigest() for name, data in generated.items()}}
        (output / "sbom-inputs.json").write_text(json.dumps(receipt, indent=2) + "\n")
        print(f"Generated {len(generated)} Rust CycloneDX inventory file(s) in {output}.")


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--features", action="append", default=[], help="one actual build feature; repeat as needed")
    parser.add_argument("--no-default-features", action="store_true")
    args = parser.parse_args(argv)
    try:
        generate(ROOT, args.output_dir.resolve(), args.features, args.no_default_features)
    except (ENV.Incomplete, OSError, ValueError, KeyError, StopIteration, subprocess.SubprocessError) as error:
        print(f"INCOMPLETE: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
