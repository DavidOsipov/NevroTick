#!/usr/bin/env python3
"""Bound registry-age decisions before existing graph/source publication.

This module supplies no source, licence, provider or build-script approval.
The existing code-owned dependency policy remains the acceptance owner. Cargo
retains locked versions and permits missing publication times; this additional
admission rejects incomplete changed-graph evidence rather than calling either
case an age pass. The exact pre-policy baseline is grandfathered once.
"""
from __future__ import annotations

import argparse
from datetime import datetime, timedelta, timezone
from functools import lru_cache
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import tempfile
import tomllib

ROOT = Path(__file__).resolve().parents[1]
REGISTRY = "registry+https://github.com/rust-lang/crates.io-index"
BASELINE_LOCK = "2ccabde33a800b745c42b4fe96dc4a6c300b1961ac87e149d313cae41fde5f2e"
BASELINE_REGISTRY = "622bd793fc19f6edea04a104463028739ec80a842da17a04981ff5b7d15e68b7"
POLICY = {"minimum_age": "14 days", "incompatible_publish_age": "deny"}
# Only an individually accepted OD-04 decision may add its complete immutable
# receipt hash here. No package exemptions or caller-provided approval map.
APPROVED_EXCEPTIONS: dict[str, str] = {}
# A graph owner accepting an update registers its three-hash checkpoint in the
# same code review as the existing policy pin. This stores no duplicate graph.
# Prior concrete exception IDs stay attached to that exact accepted checkpoint.
APPROVED_BASELINES: dict[str, dict] = {}
MAX_LOCK_BYTES = 4 * 1024 * 1024
MAX_RECORD_BYTES = 2 * 1024 * 1024
MAX_MANIFEST_BYTES = 512 * 1024
MAX_CONFIGURATION_BYTES = 64 * 1024
MAX_TOOL_BYTES = 128 * 1024 * 1024
MAX_PACKAGES = 768
HEX = re.compile(r"[a-f0-9]{64}")
NAME = re.compile(r"[A-Za-z0-9_-]{1,128}")
VERSION = re.compile(r"(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)(?:[-+][0-9A-Za-z.-]+)?")


class Incomplete(ValueError):
    """Age selection/admission is incomplete, never an implicit exception."""


def canonical(value) -> str:
    """Hash finite JSON without an embedded self-pin."""
    try:
        data = json.dumps(value, sort_keys=True, separators=(",", ":"),
                          ensure_ascii=True, allow_nan=False).encode()
    except (TypeError, ValueError, UnicodeError, RecursionError) as error:
        raise Incomplete("Invalid age-admission representation.") from error
    if len(data) > MAX_RECORD_BYTES:
        raise Incomplete("Age-admission record exceeds its byte budget.")
    return hashlib.sha256(data).hexdigest()


def _hash(value):
    return type(value) is str and HEX.fullmatch(value) is not None


def _identity(item):
    if (type(item) is not dict or set(item) != {"name", "version", "source", "checksum"}
            or type(item["name"]) is not str or NAME.fullmatch(item["name"]) is None
            or type(item["version"]) is not str or VERSION.fullmatch(item["version"]) is None
            or item["source"] != REGISTRY or not _hash(item["checksum"])):
        raise Incomplete("Unadmitted registry identity in cooldown evidence.")
    return tuple(item[key] for key in ("name", "version", "source", "checksum"))


def _records(items):
    if type(items) is not list or len(items) > MAX_PACKAGES:
        raise Incomplete("Registry age inventory exceeds its package budget.")
    keys = [_identity(item) for item in items]
    if len(keys) != len(set(keys)) or keys != sorted(keys):
        raise Incomplete("Registry age identities must be sorted and unique.")
    return keys


def _ordered(keys):
    return [dict(zip(("name", "version", "source", "checksum"), key)) for key in sorted(keys)]


def lock_document(data: bytes):
    """Read a bounded literal graph, preserving every package/edge for replay."""
    if type(data) is not bytes or len(data) > MAX_LOCK_BYTES:
        raise Incomplete("Lockfile exceeds its byte budget.")
    try:
        value = tomllib.loads(data.decode("utf-8"))
    except (ValueError, UnicodeError) as error:
        raise Incomplete("Invalid lockfile representation.") from error
    if (set(value) != {"version", "package"} or value["version"] != 4
            or type(value["package"]) is not list or len(value["package"]) > MAX_PACKAGES):
        raise Incomplete("Cooldown resolution requires an admitted version-4 lock graph.")
    identities = []
    for item in value["package"]:
        if (type(item) is not dict or not {"name", "version"} <= set(item)
                or set(item) - {"name", "version", "source", "checksum", "dependencies"}
                or type(item["name"]) is not str or NAME.fullmatch(item["name"]) is None
                or type(item["version"]) is not str or VERSION.fullmatch(item["version"]) is None
                or type(item.get("dependencies", [])) is not list
                or any(type(edge) is not str or len(edge) > 512 for edge in item.get("dependencies", []))):
            raise Incomplete("Unadmitted locked package/edge representation.")
        identities.append((item["name"], item["version"], item.get("source")))
        if "source" in item:
            _identity({key: item.get(key) for key in ("name", "version", "source", "checksum")})
        elif "checksum" in item:
            raise Incomplete("Local locked packages cannot acquire a registry checksum.")
    if len(identities) != len(set(identities)):
        raise Incomplete("Duplicate locked package identity.")
    return value


def registry_records(lock: bytes):
    return _ordered(_identity({key: item[key] for key in ("name", "version", "source", "checksum")})
                    for item in lock_document(lock)["package"] if "source" in item)


def require_reproduced_lock(generated, proposed=None):
    """An imported lock must reproduce every package/edge, not just versions."""
    document = lock_document(generated)
    if proposed is not None and document != lock_document(proposed):
        raise Incomplete("Imported lock does not reproduce the complete accepted-baseline graph/edges.")
    return document


def policy_records(policy):
    """Use existing source records; do not duplicate a complete provenance map."""
    records = [dict(name=p["name"], version=p["version"], source=REGISTRY, checksum=p["checksum"])
               for field in ("packages", "metadata_only_packages") for p in policy[field]]
    result = _ordered(_identity(item) for item in records)
    _records(result)
    return result


def baseline_admission():
    """Only the exact previously accepted graph receives grandfather status."""
    return {"schema_version": 1, "policy": dict(POLICY), "status": "accepted-baseline",
            "baseline_lockfile_sha256": BASELINE_LOCK,
            "baseline_registry_sha256": BASELINE_REGISTRY, "resolution": None}


def validate_admission_shape(value):
    """Check the closed record form; this is expressly not age authority."""
    if (type(value) is not dict or set(value) != set(baseline_admission())
            or type(value["schema_version"]) is not int or value["schema_version"] != 1
            or value["policy"] != POLICY or not _hash(value["baseline_lockfile_sha256"])
            or not _hash(value["baseline_registry_sha256"])
            or value["status"] not in ("accepted-baseline", "resolved")
            or value["status"] == "accepted-baseline" and value["resolution"] is not None
            or value["status"] == "resolved" and type(value["resolution"]) is not dict):
        raise Incomplete("Dependency policy lacks its selected 14-day graph admission shape.")
    canonical(value)


def accepted_checkpoint(admission_sha256):
    """Resolve only a code-owned previously accepted graph/admission pair."""
    initial = {"lockfile_sha256": BASELINE_LOCK, "registry_sha256": BASELINE_REGISTRY,
               "exception_decisions": []}
    if admission_sha256 == canonical(baseline_admission()):
        return initial
    value = APPROVED_BASELINES.get(admission_sha256)
    if (not _hash(admission_sha256) or type(value) is not dict
            or set(value) != set(initial) or not _hash(value["lockfile_sha256"])
            or not _hash(value["registry_sha256"]) or type(value["exception_decisions"]) is not list
            or any(type(item) is not str or not 0 < len(item) <= 256 for item in value["exception_decisions"])
            or value["exception_decisions"] != sorted(set(value["exception_decisions"]))):
        raise Incomplete("Baseline graph/admission is not a code-owned accepted checkpoint.")
    return value


def _utc(value):
    if type(value) is not str or not re.fullmatch(r"\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ", value):
        raise Incomplete("Publication/resolution time must be explicit UTC seconds.")
    try:
        return datetime.strptime(value, "%Y-%m-%dT%H:%M:%SZ").replace(tzinfo=timezone.utc)
    except ValueError as error:
        raise Incomplete("Invalid publication/resolution UTC time.") from error


def _stamp(value):
    return value.astimezone(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def resolution_command(cargo, operation, package=None, version=None):
    """Construct one fixed metadata-only operation; no free-form Cargo argv."""
    if operation == "precise":
        parts = package.split("@") if type(package) is str else []
        if (len(parts) not in (1, 2) or NAME.fullmatch(parts[0]) is None or parts[0].startswith("-")
                or len(parts) == 2 and VERSION.fullmatch(parts[1]) is None
                or type(version) is not str or VERSION.fullmatch(version) is None):
            raise Incomplete("Precise resolution requires an exact package/version.")
        return [str(cargo), "update", "--package", package, "--precise", version]
    if operation not in {"workspace", "initial"} or package is not None or version is not None:
        raise Incomplete("Unknown or unstructured dependency resolution operation.")
    return [str(cargo), *( ["update", "--workspace"] if operation == "workspace" else ["generate-lockfile"])]


def validate_exception(value):
    """Require one already approved complete decision, never a supplied name."""
    if (type(value) is not dict or set(value) != {"decision_id", "decision_ref", "decision_sha256",
            "requested_package", "requested_version", "reason", "primary_fix_evidence", "targets",
            "features", "delta_sha256"}
            or type(value["decision_id"]) is not str
            or APPROVED_EXCEPTIONS.get(value["decision_id"]) != canonical(value)
            or not _hash(value["decision_sha256"]) or not _hash(value["delta_sha256"])):
        raise Incomplete("No individually admitted OD-04 cooldown exception matches this receipt.")
    return value


def resolution_environment(toolchain, cargo_home, temporary, exception=None):
    """Build a fresh child environment; an override cannot leak to another call."""
    selected = {"PATH": f"{toolchain}/bin:/usr/bin:/bin", "CARGO_HOME": str(cargo_home),
                "RUSTC": str(Path(toolchain) / "bin/rustc"), "LANG": "C.UTF-8",
                "HOME": str(temporary), "TMPDIR": str(temporary), "CARGO_NET_RETRY": "0",
                "CARGO_HTTP_TIMEOUT": "8", "CARGO_BUILD_JOBS": "1", "RUST_TEST_THREADS": "1",
                "CARGO_REGISTRIES_CRATES_IO_PROTOCOL": "sparse"}
    if exception is not None:
        validate_exception(exception)
        selected["CARGO_RESOLVER_INCOMPATIBLE_PUBLISH_AGE"] = "allow"
    return selected


def _index_path(name):
    name = name.lower()
    if len(name) == 1:
        return "1/" + name
    if len(name) == 2:
        return "2/" + name
    if len(name) == 3:
        return "3/" + name[0] + "/" + name
    return name[:2] + "/" + name[2:4] + "/" + name


def validate_resolution(record, candidate, *, scope="product", baseline_lock=None,
                        baseline_registry=None):
    """Verify complete delta reconstruction and every newly selected age."""
    fields = {"scope", "baseline_lockfile_sha256", "baseline_registry_sha256", "candidate_lockfile_sha256",
              "candidate_graph_sha256", "candidate_registry_sha256", "operation", "package", "version",
              "started_utc", "finished_utc", "cutoff_utc", "cargo_sha256", "configuration_sha256",
              "manifests_sha256", "command", "exit_code", "failure", "added", "removed", "publication",
              "exception", "metadata_inventory_sha256", "result_sha256", "generated_lockfile_sha256"}
    fields.add("proposed_lockfile_sha256")
    fields.update({"baseline_admission_sha256", "retained_exception_decisions", "acquisition_filesystem"})
    if type(record) is not dict or set(record) != fields:
        raise Incomplete("Incomplete changed-graph cooldown decision.")
    if scope == "product":
        checkpoint = accepted_checkpoint(record["baseline_admission_sha256"])
        if (record["retained_exception_decisions"] != checkpoint["exception_decisions"]
                or baseline_lock is not None and baseline_lock != checkpoint["lockfile_sha256"]
                or baseline_registry is not None and baseline_registry != checkpoint["registry_sha256"]):
            raise Incomplete("Resolution substitutes its approved baseline or prior exception scope.")
        baseline_lock, baseline_registry = checkpoint["lockfile_sha256"], checkpoint["registry_sha256"]
    elif scope != "standalone" or record["baseline_admission_sha256"] is not None or record["retained_exception_decisions"]:
        raise Incomplete("Standalone selection cannot claim product baseline authority.")
    if (record["scope"] != scope or record["baseline_lockfile_sha256"] != baseline_lock
            or record["baseline_registry_sha256"] != baseline_registry
            or type(record["exit_code"]) is not int or record["exit_code"] != 0 or record["failure"] is not None):
        raise Incomplete("Changed graph lacks successful baseline reproduction.")
    if (record["generated_lockfile_sha256"] != record["candidate_lockfile_sha256"]
            or record["proposed_lockfile_sha256"] is not None and not _hash(record["proposed_lockfile_sha256"])):
        raise Incomplete("Age evidence substitutes an imported lock for the reproduced candidate.")
    capacity = record["acquisition_filesystem"]
    if (type(capacity) is not dict or set(capacity) != {"bytes", "inodes", "device"}
            or any(type(capacity[key]) is not int for key in capacity)
            or not 0 < capacity["bytes"] <= 512 * 1024 ** 2
            or not 0 < capacity["inodes"] <= 16384 or capacity["device"] <= 0):
        raise Incomplete("Resolution lacks its finite acquisition filesystem identity.")
    for key in ("candidate_lockfile_sha256", "candidate_graph_sha256", "candidate_registry_sha256", "cargo_sha256",
                "configuration_sha256", "metadata_inventory_sha256", "result_sha256", "generated_lockfile_sha256"):
        if not _hash(record[key]):
            raise Incomplete("Changed graph lacks a complete input/result identity.")
    if (type(record["command"]) is not list or not record["command"] or record["command"][0] != "/toolchain/bin/cargo"
            or record["command"] != resolution_command(record["command"][0], record["operation"], record["package"], record["version"])):
        raise Incomplete("Unreviewed Cargo configuration/argument override in age evidence.")
    start, end, cutoff = (_utc(record[key]) for key in ("started_utc", "finished_utc", "cutoff_utc"))
    if end < start or end - start > timedelta(seconds=300) or cutoff != start - timedelta(days=14):
        raise Incomplete("Age decision has the wrong interval or cutoff.")
    current, added, removed = set(_records(candidate)), set(_records(record["added"])), set(_records(record["removed"]))
    if (not added <= current or removed & current or added & removed
            or canonical(_ordered((current - added) | removed)) != baseline_registry
            or record["candidate_registry_sha256"] != canonical(candidate)):
        raise Incomplete("Changed graph omits, substitutes or invents a baseline/transitive delta.")
    exception = record["exception"]
    if exception is not None:
        validate_exception(exception)
        if (record["operation"] != "precise" or record["package"] != exception["requested_package"]
                or record["version"] != exception["requested_version"]
                or exception["delta_sha256"] != canonical({"added": record["added"], "removed": record["removed"]})):
            raise Incomplete("Urgent approval does not bind the complete actual resolution delta.")
    publication = record["publication"]
    if type(publication) is not list or len(publication) != len(added):
        raise Incomplete("Publication-time evidence omits a changed registry version.")
    seen = set()
    for item in publication:
        if (type(item) is not dict or set(item) != {"package", "published_utc", "source_url", "index_record_sha256", "index_record"}
                or not _hash(item["index_record_sha256"])):
            raise Incomplete("Invalid registry publication evidence.")
        key = _identity(item["package"])
        if key not in added or key in seen or item["source_url"] != "https://index.crates.io/" + _index_path(key[0]):
            raise Incomplete("Publication evidence names a wrong or duplicate registry version/source.")
        seen.add(key)
        published = item["published_utc"]
        raw = item["index_record"]
        if (type(raw) is not dict or canonical(raw) != item["index_record_sha256"]
                or (raw.get("name"), raw.get("vers"), raw.get("cksum"), raw.get("pubtime"))
                   != (key[0], key[1], key[3], published)):
            raise Incomplete("Selected raw index record/publication fields do not match age evidence.")
        if exception is None and (published is None or _utc(published) > cutoff):
            raise Incomplete("New registry version has unverified or insufficient publication age.")
        if published is not None:
            _utc(published)
    if (type(record["manifests_sha256"]) is not dict or not record["manifests_sha256"]
            or len(record["manifests_sha256"]) > 512
            or any(type(name) is not str or not name.endswith("Cargo.toml") or Path(name).is_absolute()
                   or ".." in Path(name).parts or not _hash(value)
                   for name, value in record["manifests_sha256"].items())):
        raise Incomplete("Changed graph lacks exact candidate manifests.")
    canonical(record)


def validate_policy_admission(policy):
    """Shared pre-publication boundary for policy, lock and vendor consumers."""
    value = policy.get("cooldown_admission")
    validate_admission_shape(value)
    expected = baseline_admission()
    if (type(value) is not dict or set(value) != set(expected)
            or type(value["schema_version"]) is not int or value["schema_version"] != 1 or value["policy"] != POLICY):
        raise Incomplete("Dependency policy lacks its selected 14-day graph admission.")
    candidate = policy_records(policy)
    if value["status"] == "accepted-baseline":
        if (value != expected or policy["lockfile_sha256"] != BASELINE_LOCK
                or canonical(candidate) != BASELINE_REGISTRY):
            raise Incomplete("Only the exact accepted pre-cooldown graph is grandfathered.")
    elif value["status"] == "resolved":
        record = value["resolution"]
        if (type(record) is not dict or value["baseline_lockfile_sha256"] != record.get("baseline_lockfile_sha256")
                or value["baseline_registry_sha256"] != record.get("baseline_registry_sha256")):
            raise Incomplete("Changed graph substitutes its accepted baseline.")
        validate_resolution(record, candidate)
        if (record["candidate_lockfile_sha256"] != policy["lockfile_sha256"]
                or any(record["manifests_sha256"].get(name) != digest for name, digest in policy["manifests_sha256"].items())):
            raise Incomplete("Age decision belongs to another lockfile/manifest candidate.")
    else:
        raise Incomplete("Unknown graph cooldown status.")


def validate_root_admission(root, policy):
    """Bind reviewed policy age evidence to actual current lock/config bytes."""
    validate_policy_admission(policy)
    root = Path(root)
    for name, digest in policy["manifests_sha256"].items():
        path = Path(name)
        if (path.is_absolute() or ".." in path.parts or not name.endswith("Cargo.toml")
                or hashlib.sha256(read_input(root / path, MAX_MANIFEST_BYTES)).hexdigest() != digest):
            raise Incomplete("Actual manifest context differs from its accepted graph.")
    environment = _load("cooldown_root_environment", ROOT / "scripts/check-environment.py")
    config = read_input(root / ".cargo/config.toml", MAX_CONFIGURATION_BYTES)
    environment.require_cargo_configuration(tomllib.loads(config.decode("utf-8")))
    lock = read_input(root / "Cargo.lock", MAX_LOCK_BYTES)
    if hashlib.sha256(lock).hexdigest() != policy["lockfile_sha256"] or registry_records(lock) != policy_records(policy):
        raise Incomplete("Actual graph differs from its age/source admission.")
    record = policy["cooldown_admission"]["resolution"]
    if record is not None:
        tools = _strict_json(read_input(root / "policy/tool-manifest.json", MAX_RECORD_BYTES))
        if (record["cargo_sha256"] != tools["rust"]["linux_binary_sha256"]["cargo"]
                or record["configuration_sha256"] != hashlib.sha256(config).hexdigest()
                or record["candidate_graph_sha256"] != canonical(lock_document(lock))):
            raise Incomplete("Actual compiler/configuration/full lock graph differs from age evidence.")
        for name, digest in record["manifests_sha256"].items():
            if hashlib.sha256(read_input(root / name, MAX_MANIFEST_BYTES)).hexdigest() != digest:
                raise Incomplete("Actual complete resolution manifest context differs.")
    return policy["cooldown_admission"]


def _load(name, path):
    specification = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


@lru_cache(maxsize=1)
def _input_custody():
    return _load("cooldown_bounded_input_custody", ROOT / "tools/crypto-aes-siv-witness/fuzz/input_custody.py")


def read_input(path, maximum):
    """Reuse descriptor/ancestor custody and reserve size before body allocation."""
    try:
        return _input_custody().read_file(Path(path), maximum=maximum)
    except RuntimeError as error:
        raise Incomplete("Cooldown input is unavailable, redirected or exceeds its byte bound.") from error


def digest_input(path, maximum):
    """Hash through the same bounded descriptor owner without body allocation."""
    try:
        return _input_custody().digest(Path(path), maximum=maximum)
    except RuntimeError as error:
        raise Incomplete("Cooldown input identity is unavailable, redirected or exceeds its byte bound.") from error


def _tool_digest(path, expected=None):
    digest = digest_input(path, MAX_TOOL_BYTES)
    if expected is not None and digest != expected:
        raise Incomplete("Resolution executable digest differs from the admitted compiler cohort.")
    return digest


def _strict_json(data):
    if type(data) is not bytes or len(data) > MAX_RECORD_BYTES:
        raise Incomplete("Raw cooldown JSON exceeds its byte bound.")
    def pairs(items):
        value = {}
        for key, item in items:
            if key in value:
                raise Incomplete("Duplicate JSON field in cooldown evidence.")
            value[key] = item
        return value
    def constant(_value):
        raise Incomplete("Nonfinite JSON field in cooldown evidence.")
    try:
        return json.loads(data, object_pairs_hook=pairs, parse_constant=constant)
    except (ValueError, UnicodeError, RecursionError) as error:
        raise Incomplete("Invalid JSON cooldown evidence.") from error


def _publication(cargo_home, added, snapshot):
    """Read exact checksum/version records Cargo acquired from its sparse index."""
    result = []
    for package in added:
        matches = list((cargo_home / "registry/index").glob("*/.cache/" + _index_path(package["name"])))
        if len(matches) != 1:
            raise Incomplete("Missing or ambiguous acquired sparse publication metadata.")
        data = snapshot.CUSTODY.read_file(matches[0], maximum=snapshot.MAX_FILE_BYTES)
        rows = []
        # Cargo's local sparse cache has NUL-delimited header/version/JSON
        # records. A future incompatible format fails closed, never guesses age.
        for field in data.split(b"\0"):
            if field.startswith(b"{"):
                item = _strict_json(field)
                if item.get("name") == package["name"] and item.get("vers") == package["version"]:
                    rows.append(item)
        if len(rows) != 1 or rows[0].get("cksum") != package["checksum"]:
            raise Incomplete("Acquired publication metadata differs from the selected checksum/version.")
        row = rows[0]
        published = row.get("pubtime")
        if published is not None:
            _utc(published)
        result.append({"package": package, "published_utc": published,
                       "source_url": "https://index.crates.io/" + _index_path(package["name"]),
                       "index_record_sha256": canonical(row), "index_record": row})
    return result


def _manifest_context(project, files):
    """Bound the actual manifest context and reject unselected acquisition routes."""
    manifests = {}
    for name, digest in files.items():
        if name.endswith("Cargo.toml"):
            manifests[name] = digest
            document = tomllib.loads(read_input(project / name, MAX_MANIFEST_BYTES).decode("utf-8"))
            def dependency(item):
                if type(item) is not dict:
                    return
                if "git" in item or item.get("registry", "crates-io") != "crates-io":
                    raise Incomplete("Git/custom-registry transitions need separate source/age admission.")
                if "path" in item:
                    path = item["path"]
                    if (type(path) is not str or Path(path).is_absolute()
                            or not (project / name).parent.joinpath(path).resolve(strict=True).is_relative_to(project)):
                        raise Incomplete("Local dependency path escapes the owned candidate.")
            def inspect(value):
                if type(value) is not dict:
                    return
                for key, item in value.items():
                    if key in {"dependencies", "dev-dependencies", "build-dependencies", "dev_dependencies", "build_dependencies"} and type(item) is dict:
                        for declared in item.values():
                            dependency(declared)
                    elif key == "patch" and type(item) is dict:
                        for origin, declarations in item.items():
                            if origin != "crates-io" or type(declarations) is not dict:
                                raise Incomplete("Custom-registry patch transitions need separate admission.")
                            for declared in declarations.values():
                                dependency(declared)
                    elif key == "replace":
                        raise Incomplete("Replacement routing is not an admitted cooldown update path.")
                    inspect(item)
            inspect(document)
    if not manifests or "Cargo.toml" not in manifests or len(manifests) > 512:
        raise Incomplete("Missing or excessive candidate manifest inventory.")
    return manifests


def _sandbox(toolchain, work, environment, command):
    """Isolate metadata acquisition with public network but no host write mounts."""
    bwrap, prlimit = shutil.which("bwrap"), shutil.which("prlimit")
    if not bwrap or not prlimit:
        raise Incomplete("Cooldown resolution requires the existing isolated acquisition tools.")
    argv = [bwrap, "--unshare-all", "--share-net", "--die-with-parent", "--new-session",
            "--cap-drop", "ALL", "--clearenv"]
    for path in ("/usr", "/bin", "/lib", "/lib64", "/etc/alternatives", "/etc/ld.so.cache",
                 "/etc/ssl/certs", "/etc/resolv.conf", "/etc/hosts", "/etc/nsswitch.conf"):
        if Path(path).exists():
            argv += ["--ro-bind", path, path]
    argv += ["--ro-bind", str(toolchain), "/toolchain", "--bind", str(work), "/work",
             "--proc", "/proc", "--dev", "/dev", "--tmpfs", "/tmp", "--chdir", "/work/project"]
    for key, value in environment.items():
        argv += ["--setenv", key, value]
    return argv + ["--", prlimit, "--nproc=128", "--", *command]


def _output_parent(output, snapshot):
    """Bind a private canonical parent; aliases never become publication paths."""
    before = snapshot.regular_root(output.parent)
    if (output.resolve(strict=False) != output or before.st_uid != os.getuid()
            or before.st_mode & 0o077 or output.exists() or output.is_symlink()):
        raise Incomplete("Fresh resolution output requires a private owned canonical parent.")
    return snapshot.stamp(before)


def _publish(output, payloads, snapshot, before):
    """Publish through owned descriptors after rechecking parent custody."""
    if (type(payloads) is not dict or set(payloads) != {"Cargo.lock", "cooldown-resolution.json", "resolver-result.json"}
            or any(type(data) is not bytes or len(data) > (MAX_LOCK_BYTES if name == "Cargo.lock" else MAX_RECORD_BYTES)
                   for name, data in payloads.items())):
        raise Incomplete("Resolution publication requires its exact bounded lock/decision/result inventory.")
    if _output_parent(output, snapshot) != before:
        raise Incomplete("Resolution output parent changed before publication.")
    parent = os.open(output.parent, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC)
    child = None
    created = False
    try:
        if snapshot.stamp(os.fstat(parent)) != before:
            raise Incomplete("Resolution output parent descriptor differs.")
        os.mkdir(output.name, mode=0o700, dir_fd=parent)
        created = True
        child = os.open(output.name, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC, dir_fd=parent)
        for name, data in payloads.items():
            descriptor = os.open(name, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600, dir_fd=child)
            with os.fdopen(descriptor, "wb") as stream:
                stream.write(data)
        parent_path = snapshot.regular_root(output.parent)
        if (parent_path.st_dev, parent_path.st_ino) != (os.fstat(parent).st_dev, os.fstat(parent).st_ino):
            raise Incomplete("Resolution output parent was replaced during publication.")
    except BaseException:
        if created and child is not None:
            for name in payloads:
                try:
                    os.unlink(name, dir_fd=child)
                except FileNotFoundError:
                    pass
            os.rmdir(output.name, dir_fd=parent)
        raise
    finally:
        if child is not None:
            os.close(child)
        os.close(parent)


def resolve(args):
    """Reproduce one structured update, then export a new lock/decision only.

    The caller owns/provisions the existing finite acquisition filesystem and a
    private candidate source snapshot. This function neither edits the accepted
    checkout/policy nor approves or vendors the resulting graph. Imported locks
    are compared with the complete parsed reproduced graph before publication.
    """
    import fcntl
    environment = _load("cooldown_environment", ROOT / "scripts/check-environment.py")
    snapshot = _load("cooldown_snapshot", ROOT / "scripts/advisory_snapshot.py")
    candidate = args.candidate_root.expanduser().absolute()
    snapshot.regular_root(candidate)
    if (candidate == ROOT or candidate.is_relative_to(ROOT) or candidate.stat().st_uid != os.getuid()
            or candidate.stat().st_mode & 0o077):
        raise Incomplete("Resolution candidate must be a private external owned snapshot.")
    cargo_context = environment.require_cargo_context(candidate)
    output = args.output_dir.expanduser().absolute()
    quota = args.quota_root.expanduser().absolute()
    if (output.exists() or output.is_relative_to(ROOT) or output.is_relative_to(candidate)
            or candidate.is_relative_to(output) or output.is_relative_to(quota) or quota.is_relative_to(output)
            or candidate.is_relative_to(quota) or quota.is_relative_to(candidate)):
        raise Incomplete("Resolution inputs, quota and fresh output need separate custody.")
    output_parent = _output_parent(output, snapshot)
    tool_policy_bytes = read_input(ROOT / "policy/tool-manifest.json", MAX_RECORD_BYTES)
    policy = _strict_json(tool_policy_bytes)
    producer_inputs = {name: digest_input(ROOT / name, MAX_RECORD_BYTES) for name in (
        "scripts/dependency_cooldown.py", "scripts/check-environment.py", "scripts/advisory_snapshot.py",
        "tools/crypto-aes-siv-witness/fuzz/input_custody.py")}
    version = policy["rust"]["channel"]
    if tuple(map(int, version.split("."))) < (1, 100, 0):
        raise Incomplete("Publication-age enforcement requires the admitted Cargo 1.100+ cohort.")
    toolchain = args.toolchain_dir.expanduser().resolve(strict=True)
    for name in ("cargo", "rustc"):
        _tool_digest(toolchain / "bin" / name, policy["rust"]["linux_binary_sha256"][name])
    original = snapshot.inventory(candidate)
    if original.get(".cargo/config.toml") != cargo_context["config_sha256"]:
        raise Incomplete("Candidate configuration differs from its admitted ordinary context.")
    if any(part in {".git", "target", ".hg", ".svn"} for name in original for part in Path(name).parts):
        raise Incomplete("Resolution snapshot must omit repository metadata/build outputs.")
    if (".cargo/config.toml" not in original
            or any(name.endswith((".cargo/config", ".cargo/config.toml")) and name != ".cargo/config.toml" for name in original)):
        raise Incomplete("Resolution has an unreviewed legacy/nested Cargo configuration.")
    environment.require_cargo_configuration(tomllib.loads(read_input(candidate / ".cargo/config.toml", MAX_CONFIGURATION_BYTES).decode("utf-8")))
    manifests = _manifest_context(candidate, original)
    baseline = None
    if args.operation == "initial":
        if args.scope != "standalone" or args.baseline_lock is not None or "Cargo.lock" in original:
            raise Incomplete("Initial resolution requires a new separately owned workspace without a lock.")
        baseline = b"version = 4\npackage = []\n"
    elif args.baseline_lock is None:
        raise Incomplete("Updates require the accepted baseline lock; never delete it for resolution.")
    else:
        baseline = snapshot.CUSTODY.read_file(args.baseline_lock, maximum=MAX_LOCK_BYTES)
    baseline_registry = registry_records(baseline)
    baseline_admission_sha256, retained_exceptions, accepted_policy_bytes = None, [], None
    if args.scope == "product":
        owner = _load("cooldown_current_policy_owner", ROOT / "scripts/slint_dependency_profile.py")
        accepted_policy_bytes = read_input(ROOT / "policy/ui-m0-dependencies.json", MAX_RECORD_BYTES)
        current = _strict_json(accepted_policy_bytes)
        owner.validate_policy(current)
        validate_root_admission(ROOT, current)
        baseline_admission_sha256 = canonical(current["cooldown_admission"])
        checkpoint = accepted_checkpoint(baseline_admission_sha256)
        if (hashlib.sha256(baseline).hexdigest() != current["lockfile_sha256"]
                or checkpoint["lockfile_sha256"] != current["lockfile_sha256"]
                or canonical(baseline_registry) != checkpoint["registry_sha256"]):
            raise Incomplete("Product resolution must start at the current accepted root checkpoint.")
        retained_exceptions = checkpoint["exception_decisions"]
    proposed = (snapshot.CUSTODY.read_file(args.proposed_lock, maximum=MAX_LOCK_BYTES)
                if args.proposed_lock is not None else None)
    if proposed is not None:
        lock_document(proposed)
    exception = (_strict_json(snapshot.CUSTODY.read_file(args.exception, maximum=MAX_RECORD_BYTES))
                 if args.exception is not None else None)
    command = resolution_command("/toolchain/bin/cargo", args.operation, args.package, args.version)
    if exception is not None:
        validate_exception(exception)
        if (args.operation != "precise" or args.package != exception["requested_package"]
                or args.version != exception["requested_version"]):
            raise Incomplete("Urgent receipt does not name this precise child invocation.")
    snapshot.regular_root(quota)
    descriptor = os.open(quota, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC)
    try:
        fcntl.flock(descriptor, fcntl.LOCK_EX | fcntl.LOCK_NB)
        capacity = snapshot.quota_root(quota)
        if snapshot.stamp(os.fstat(descriptor)) != snapshot.stamp(quota.lstat()):
            raise Incomplete("Acquisition root changed before resolution admission.")
        with tempfile.TemporaryDirectory(prefix="cooldown-", dir=quota) as temporary:
            work = Path(temporary)
            project = work / "project"
            project.mkdir(mode=0o700)
            snapshot.export(candidate, project, original)
            if args.operation != "initial":
                (project / "Cargo.lock").write_bytes(baseline)
            for name in ("cargo-home", "tmp"):
                (work / name).mkdir(mode=0o700)
            selected = resolution_environment("/toolchain", "/work/cargo-home", "/work/tmp", exception)
            started = datetime.now(timezone.utc).replace(microsecond=0)
            raw = snapshot.run(_sandbox(toolchain, work, selected, command), cwd=ROOT,
                               environment={"PATH": "/usr/bin:/bin", "LANG": "C.UTF-8"})
            finished = datetime.now(timezone.utc).replace(microsecond=0)
            environment.require_ok(raw)
            generated = snapshot.CUSTODY.read_file(project / "Cargo.lock", maximum=MAX_LOCK_BYTES)
            generated_document = require_reproduced_lock(generated, proposed)
            actual = snapshot.inventory(project)
            if {name: digest for name, digest in actual.items() if name != "Cargo.lock"} != {name: digest for name, digest in original.items() if name != "Cargo.lock"}:
                raise Incomplete("Resolution changed inputs outside its owned lockfile.")
            metadata = snapshot.inventory(work / "cargo-home")
            if any(name.startswith(("registry/cache/", "registry/src/", "git/")) for name in metadata):
                raise Incomplete("Resolution acquired source/archive payloads instead of metadata only.")
            # Imported bytes are evidence of the proposal, never a replacement
            # for the actual reproduced lock accepted by the graph owner.
            candidate_lock = generated
            records = registry_records(candidate_lock)
            added = _ordered(set(_records(records)) - set(_records(baseline_registry)))
            removed = _ordered(set(_records(baseline_registry)) - set(_records(records)))
            record = {"scope": args.scope, "baseline_lockfile_sha256": hashlib.sha256(baseline).hexdigest(),
                "baseline_admission_sha256": baseline_admission_sha256, "retained_exception_decisions": retained_exceptions,
                "acquisition_filesystem": capacity,
                "baseline_registry_sha256": canonical(baseline_registry),
                "candidate_lockfile_sha256": hashlib.sha256(candidate_lock).hexdigest(),
                "candidate_graph_sha256": canonical(generated_document), "candidate_registry_sha256": canonical(records),
                "operation": args.operation, "package": args.package, "version": args.version,
                "started_utc": _stamp(started), "finished_utc": _stamp(finished), "cutoff_utc": _stamp(started - timedelta(days=14)),
                "cargo_sha256": _tool_digest(toolchain / "bin/cargo"),
                "configuration_sha256": original[".cargo/config.toml"], "manifests_sha256": manifests,
                "command": command, "exit_code": raw["exit_code"], "failure": raw["failure"],
                "added": added, "removed": removed, "publication": _publication(work / "cargo-home", added, snapshot),
                "exception": exception, "metadata_inventory_sha256": canonical(metadata), "result_sha256": canonical(raw),
                "generated_lockfile_sha256": hashlib.sha256(generated).hexdigest(),
                "proposed_lockfile_sha256": None if proposed is None else hashlib.sha256(proposed).hexdigest()}
            validate_resolution(record, records, scope=args.scope,
                baseline_lock=hashlib.sha256(baseline).hexdigest(), baseline_registry=canonical(baseline_registry))
            if snapshot.inventory(candidate) != original or (args.baseline_lock is not None
                    and snapshot.CUSTODY.read_file(args.baseline_lock, maximum=MAX_LOCK_BYTES) != baseline):
                raise Incomplete("Resolution input baseline/source changed during acquisition.")
            if environment.require_cargo_context(candidate) != cargo_context:
                raise Incomplete("Ordinary Cargo context changed during resolution.")
            if args.proposed_lock is not None and snapshot.CUSTODY.read_file(args.proposed_lock, maximum=MAX_LOCK_BYTES) != proposed:
                raise Incomplete("Imported proposal changed during reproduction.")
            if (read_input(ROOT / "policy/tool-manifest.json", MAX_RECORD_BYTES) != tool_policy_bytes
                    or any(digest_input(ROOT / name, MAX_RECORD_BYTES) != digest for name, digest in producer_inputs.items())
                    or accepted_policy_bytes is not None and read_input(ROOT / "policy/ui-m0-dependencies.json", MAX_RECORD_BYTES) != accepted_policy_bytes):
                raise Incomplete("Resolution producer/tool/accepted-policy inputs changed during acquisition.")
            for name in ("cargo", "rustc"):
                _tool_digest(toolchain / "bin" / name, policy["rust"]["linux_binary_sha256"][name])
            _publish(output, {"resolver-result.json": (json.dumps(raw, indent=2) + "\n").encode(),
                "Cargo.lock": candidate_lock,
                "cooldown-resolution.json": (json.dumps(record, indent=2) + "\n").encode()}, snapshot, output_parent)
            return record
    finally:
        os.close(descriptor)


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--candidate-root", type=Path, required=True)
    parser.add_argument("--baseline-lock", type=Path)
    parser.add_argument("--proposed-lock", type=Path)
    parser.add_argument("--quota-root", type=Path, required=True)
    parser.add_argument("--output-dir", type=Path, required=True)
    parser.add_argument("--toolchain-dir", type=Path, required=True)
    parser.add_argument("--operation", choices=("precise", "workspace", "initial"), required=True)
    parser.add_argument("--scope", choices=("product", "standalone"), default="product")
    parser.add_argument("--package")
    parser.add_argument("--version")
    parser.add_argument("--exception", type=Path)
    args = parser.parse_args(argv)
    try:
        record = resolve(args)
        print(json.dumps({"status": "resolved-not-approved", "scope": record["scope"],
            "candidate_lockfile_sha256": record["candidate_lockfile_sha256"],
            "added": len(record["added"]), "removed": len(record["removed"]),
            "exception_decision": None if record["exception"] is None else record["exception"]["decision_id"]}))
        return 0
    except (Incomplete, OSError, ValueError, KeyError, RuntimeError) as error:
        print("INCOMPLETE: cooldown resolution: " + str(error))
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
