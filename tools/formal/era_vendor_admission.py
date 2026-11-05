"""Authenticate and copy only the reviewed EraWindow/Garde vendor closure.

No Cargo, build script, proc macro, compiler or verifier is executed. Authenticity
uses the existing verify.py.admit_vendor rule: complete actual source hashes must
match Cargo's file map AND the independently reviewed policy file-map digest.
Descriptor/path bookends detect observed drift, not an adversarial transaction
against a compromised host or concurrent same-UID writer.
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path, PurePosixPath
import re
import stat
import sys
import tomllib


ROOT = Path(__file__).resolve().parents[2]
SELF = "tools/formal/era_vendor_admission.py"
PROJECTION_SOURCE = "tools/formal/era_constructor_projection.py"
CUSTODY_SOURCE = "tools/crypto-aes-siv-witness/fuzz/input_custody.py"
SEMANTICS_SOURCE = "scripts/verify.py"
MAX_CONTEXT_BYTES = 4 * 1024 ** 2
MAX_FILE_BYTES = 16 * 1024 ** 2
MAX_CHECKSUM_BYTES = 4 * 1024 ** 2
MAX_MANIFEST_BYTES = 8 * 1024 ** 2
MAX_PACKAGE_FILES = 4096
MAX_TOTAL_FILES = 20_000
MAX_TOTAL_BYTES = 256 * 1024 ** 2
HEX = re.compile(r"[0-9a-f]{64}\Z")
sys.dont_write_bytecode = True


class Incomplete(RuntimeError):
    """No complete selected vendor admission follows from these inputs."""


def _module(name: str, relative: str):
    specification = importlib.util.spec_from_file_location(name, ROOT / relative)
    if specification is None or specification.loader is None:
        raise Incomplete("Missing frozen vendor admission primitive")
    loaded = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(loaded)
    return loaded


PROJECTION = _module("era_vendor_projection", PROJECTION_SOURCE)
CUSTODY = _module("era_vendor_custody", CUSTODY_SOURCE)


def input_names() -> tuple[str, ...]:
    """The complete repository data/code closure read by this helper."""
    return tuple(sorted({SELF, PROJECTION_SOURCE, CUSTODY_SOURCE, SEMANTICS_SOURCE,
                         *PROJECTION.SOURCE_SHA256}))


def _digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def _json(data: bytes):
    def unique(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise Incomplete("Duplicate vendor metadata key")
            result[key] = value
        return result

    def invalid(_):
        raise Incomplete("Nonfinite vendor metadata")

    return json.loads(data, object_pairs_hook=unique, parse_constant=invalid)


def _encoded(value) -> bytes:
    return (json.dumps(value, sort_keys=True, separators=(",", ":"), allow_nan=False) + "\n").encode()


def _uid() -> int:
    return os.geteuid()


def _canonical(value, *, private=False) -> Path:
    path = Path(value)
    text = str(path)
    if (not path.is_absolute() or ":" in text
            or any(ord(char) < 32 or 127 <= ord(char) <= 159 for char in text)
            or path != path.resolve(strict=True)):
        raise Incomplete("Vendor path must be canonical, absolute and printable")
    info = path.lstat()
    if (not stat.S_ISDIR(info.st_mode) or info.st_uid != _uid() or path.is_mount()
            or stat.S_IMODE(info.st_mode) & (0o077 if private else 0o022)):
        raise Incomplete("Vendor directory owner/mode/scope is not admitted")
    return path


def _ancestors(path: Path) -> dict:
    result = {}
    for parent in path.parents:
        info = parent.lstat()
        sticky_root = info.st_uid == 0 and info.st_mode & stat.S_ISVTX
        if (not stat.S_ISDIR(info.st_mode) or info.st_uid not in {0, _uid()}
                or (stat.S_IMODE(info.st_mode) & 0o022 and not sticky_root)):
            raise Incomplete("Vendor ancestor has unsafe ownership or write authority")
        result[str(parent)] = tuple(getattr(info, field) for field in
                                   ("st_dev", "st_ino", "st_mode", "st_uid", "st_gid"))
    return result


def _overlap(left: Path, right: Path) -> bool:
    return left.is_relative_to(right) or right.is_relative_to(left)


def _scope(root: Path, destination: Path, work_root=None, source=None):
    excluded = [root, ROOT]
    if work_root is not None:
        excluded.append(_canonical(work_root, private=True))
    if source is not None:
        excluded.append(source)
    if any(_overlap(destination, other) for other in excluded):
        raise Incomplete("Vendor bundle must be separate from checkout, work and source cache")


def _contract(root: Path) -> dict:
    """Bind the selected graph and grants to exact existing reviewed inputs."""
    root = _canonical(root)
    files = {name: CUSTODY.read_file(root / name, maximum=MAX_CONTEXT_BYTES) for name in input_names()}
    hashes = {name: _digest(data) for name, data in files.items()}
    for name, expected in PROJECTION.SOURCE_SHA256.items():
        if hashes[name] != expected:
            raise Incomplete("Reviewed constructor context changed: " + name)
    for name in (SELF, PROJECTION_SOURCE, CUSTODY_SOURCE):
        if CUSTODY.digest(ROOT / name, maximum=MAX_CONTEXT_BYTES) != hashes[name]:
            raise Incomplete("Loaded admission code differs from the declared frozen context")
    graph, _ = PROJECTION.selected_registry_graph(files["Cargo.lock"])
    policy = _json(files["policy/ui-m0-dependencies.json"])
    if policy["lockfile_sha256"] != hashes["Cargo.lock"]:
        raise Incomplete("Dependency policy does not bind the selected lock")
    records = {item["name"]: item for item in policy["packages"]}
    if len(records) != len(policy["packages"]):
        raise Incomplete("Duplicate dependency policy record")
    selected = []
    for package in graph:
        admitted = records[package["name"]]
        if (any(package[key] != admitted[key] for key in ("name", "version", "checksum", "license"))
                or type(admitted["build_script"]) is not bool
                or admitted["build_script"] != bool(package["build_script"])
                or not HEX.fullmatch(admitted["files_sha256"])):
            raise Incomplete("Selected dependency grants differ from the exact constructor graph")
        selected.append({**admitted, "build_path": package["build_script"],
                         "proc_macro": package["proc_macro"],
                         "expected_features": policy["rows"]["x86_64-unknown-linux-gnu"]["features"][
                             package["name"] + "@" + package["version"]]})
    if len(selected) != 14:
        raise Incomplete("Constructor vendor closure must have exactly fourteen packages")
    return {"inputs_sha256": hashes, "packages": selected,
            "policy_sha256": hashes["policy/ui-m0-dependencies.json"], "lock_sha256": hashes["Cargo.lock"],
            "projection_sha256": hashes[PROJECTION_SOURCE], "verifier_semantics_sha256": hashes[SEMANTICS_SOURCE]}


def _relative(text: str) -> PurePosixPath:
    if type(text) is not str or not text or len(text) > 4096 or "\\" in text or ":" in text:
        raise Incomplete("Invalid vendor file-map path")
    path = PurePosixPath(text)
    if (path.is_absolute() or path.as_posix() != text or any(part in {".", ".."} for part in path.parts)
            or any(ord(char) < 32 or 127 <= ord(char) <= 159 for char in text)):
        raise Incomplete("Vendor file map escapes or aliases its package")
    return path


def _snapshot(directory: Path, *, sealed=False) -> dict:
    """Inspect names/types/ownership without following aliases or reading payloads."""
    result = {}
    identities = set()

    def visit(path):
        info = path.lstat()
        if len(result) >= MAX_TOTAL_FILES + 8192 or info.st_uid != _uid():
            raise Incomplete("Vendor tree exceeds its entry budget or contains an unowned entry")
        relative = path.relative_to(directory).as_posix()
        if stat.S_ISDIR(info.st_mode):
            if path.is_mount() or stat.S_IMODE(info.st_mode) & 0o022 or (sealed and stat.S_IMODE(info.st_mode) != 0o500):
                raise Incomplete("Vendor directory mode/mount is not admitted")
            kind = "directory"
        elif stat.S_ISREG(info.st_mode):
            if (info.st_nlink != 1 or not 0 <= info.st_size <= MAX_FILE_BYTES
                    or stat.S_IMODE(info.st_mode) & 0o022 or (sealed and stat.S_IMODE(info.st_mode) != 0o400)):
                raise Incomplete("Vendor regular file has an alias, unsafe mode or excessive size")
            kind = "file"
        else:
            raise Incomplete("Vendor symlinks and special files are forbidden")
        inode = (info.st_dev, info.st_ino)
        if inode in identities:
            raise Incomplete("Vendor tree contains an inode alias")
        identities.add(inode)
        result[relative] = {"kind": kind, "identity": CUSTODY._identity(info), "bytes": info.st_size}
        if kind == "directory":
            with os.scandir(path) as entries:
                for entry in entries:
                    visit(Path(entry.path))

    visit(directory)
    return result


def _package(package: Path, record: dict, budget: dict, *, sealed=False) -> dict:
    before = _snapshot(package, sealed=sealed)
    checksum_path = package / ".cargo-checksum.json"
    checksum_bytes = CUSTODY.read_file(checksum_path, maximum=MAX_CHECKSUM_BYTES)
    checksum = _json(checksum_bytes)
    if (type(checksum) is not dict or set(checksum) != {"files", "package"}
            or checksum["package"] != record["checksum"] or type(checksum["files"]) is not dict
            or not 0 < len(checksum["files"]) <= MAX_PACKAGE_FILES):
        raise Incomplete("Vendor archive/checksum schema differs from the reviewed lock")
    expected = checksum["files"]
    for name, value in expected.items():
        _relative(name)
        if name == ".cargo-checksum.json" or type(value) is not str or not HEX.fullmatch(value):
            raise Incomplete("Vendor file map has an invalid hash or self reference")
    # This independent policy pin closes self-consistent source/map substitution.
    map_sha = _digest(json.dumps(expected, sort_keys=True, separators=(",", ":")).encode())
    if map_sha != record["files_sha256"]:
        raise Incomplete("Vendor file map differs from the reviewed archive extraction")
    wanted_files = set(expected) | {".cargo-checksum.json"}
    wanted_dirs = {"."}
    for name in wanted_files:
        wanted_dirs.update(parent.as_posix() for parent in PurePosixPath(name).parents)
    actual_files = {name for name, item in before.items() if item["kind"] == "file"}
    actual_dirs = {name for name, item in before.items() if item["kind"] == "directory"}
    if actual_files != wanted_files or actual_dirs != wanted_dirs:
        raise Incomplete("Vendor package has missing or unexpected files/directories")
    hashes = {}
    for name in sorted(wanted_files):
        size = before[name]["bytes"]
        budget["files"] += 1
        budget["bytes"] += size
        if budget["files"] > MAX_TOTAL_FILES or budget["bytes"] > MAX_TOTAL_BYTES:
            raise Incomplete("Selected vendor closure exceeds its aggregate budget")
        actual = _digest(checksum_bytes) if name == ".cargo-checksum.json" else CUSTODY.digest(
            package / name, maximum=MAX_FILE_BYTES)
        if name in expected and actual != expected[name]:
            raise Incomplete("Actual vendor bytes differ from the authenticated file map")
        hashes[name] = actual
    manifest = tomllib.loads(CUSTODY.read_file(package / "Cargo.toml", maximum=MAX_CHECKSUM_BYTES).decode())
    declared = manifest["package"]
    if (any(declared.get(key) != record[key] for key in ("name", "version", "license"))
            or declared.get("build", False) != (record["build_path"] or False)
            or bool(manifest.get("lib", {}).get("proc-macro", False)) != record["proc_macro"]):
        raise Incomplete("Actual Cargo identity/license/build/proc-macro inputs differ from the grants")
    after = _snapshot(package, sealed=sealed)
    if before != after:
        raise Incomplete("Vendor package namespace or metadata changed during admission")
    return {"name": record["name"], "version": record["version"], "grants": record,
            "files": hashes, "reviewed_map_sha256": map_sha,
            "checksum_metadata_sha256": _digest(checksum_bytes), "tree_identity": before}


def _selected(source: Path, contract: dict, *, sealed=False) -> dict:
    ancestors = _ancestors(source)
    root_before = tuple(getattr(source.lstat(), field) for field in
                        ("st_dev", "st_ino", "st_mode", "st_uid", "st_gid"))
    budget = {"files": 0, "bytes": 0}
    packages = {}
    for record in contract["packages"]:
        name = record["name"] + "-" + record["version"]
        packages[name] = _package(source / name, record, budget, sealed=sealed)
    root_after = tuple(getattr(source.lstat(), field) for field in
                       ("st_dev", "st_ino", "st_mode", "st_uid", "st_gid"))
    if ancestors != _ancestors(source) or root_before != root_after:
        raise Incomplete("Selected source ancestor authority changed during admission")
    return {"packages": packages, "files": budget["files"], "bytes": budget["bytes"],
            "ancestor_identity": ancestors, "root_identity": root_before}


def cargo_config(vendor_root: Path) -> bytes:
    """Match the prepared proof project's only offline source replacement."""
    return ('[net]\noffline = true\n\n[source.crates-io]\nreplace-with = "formal-vendor"\n'
            '\n[source.formal-vendor]\ndirectory = ' + json.dumps(str(vendor_root)) + '\n').encode()


def _inventory(selected: dict) -> dict:
    return {name + "/" + relative: digest for name, package in selected["packages"].items()
            for relative, digest in package["files"].items()}


def _write_new(path: Path, payload: bytes):
    if path.parent != path.parent.resolve(strict=True):
        raise Incomplete("Materialization parent redirects")
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC, 0o600)
    try:
        info = os.fstat(descriptor)
        if not stat.S_ISREG(info.st_mode) or info.st_uid != _uid() or info.st_nlink != 1:
            raise Incomplete("Materialized output is not a fresh owned regular file")
        offset = 0
        while offset < len(payload):
            written = os.write(descriptor, payload[offset:offset + CUSTODY.CHUNK_SIZE])
            if written <= 0:
                raise Incomplete("Materialization write did not complete")
            offset += written
        os.fchmod(descriptor, 0o400)
    finally:
        os.close(descriptor)


def prepare(root: Path, destination: Path, vendor_root=None, *, work_root=None) -> dict:
    """Create one private selected bundle; reject existing destinations, execute nothing."""
    root = _canonical(root)
    destination = Path(destination)
    if (not destination.is_absolute() or destination.exists() or destination.is_symlink()
            or ":" in str(destination)
            or any(ord(char) < 32 or 127 <= ord(char) <= 159 for char in str(destination))):
        raise Incomplete("Vendor destination must be a new absolute leaf")
    _canonical(destination.parent, private=True)
    _ancestors(destination)
    source = _canonical(PROJECTION.DEFAULT_VENDOR_ROOT if vendor_root is None else vendor_root)
    _scope(root, destination, work_root, source)
    contract = _contract(root)
    selected = _selected(source, contract)
    source_roots = {name: _snapshot(source / name) for name in selected["packages"]}
    destination.mkdir(mode=0o700)
    vendor = destination / "vendor"
    vendor.mkdir(mode=0o700)
    for name, package in selected["packages"].items():
        target = vendor / name
        target.mkdir(mode=0o700)
        directories = {PurePosixPath(relative).parent for relative in package["files"]}
        directories |= {ancestor for relative in package["files"] for ancestor in PurePosixPath(relative).parents}
        for relative in sorted(directories, key=lambda item: (len(item.parts), item.as_posix())):
            if relative != PurePosixPath("."):
                (target / relative).mkdir(mode=0o700, exist_ok=True)
        for relative, expected in package["files"].items():
            payload = CUSTODY.read_file(source / name / relative, maximum=MAX_FILE_BYTES)
            if _digest(payload) != expected:
                raise Incomplete("Authenticated source bytes changed before copying")
            _write_new(target / relative, payload)
    for name, before in source_roots.items():
        if _snapshot(source / name) != before or before != selected["packages"][name]["tree_identity"]:
            raise Incomplete("Selected source changed during materialization")
    root_after = tuple(getattr(source.lstat(), field) for field in
                       ("st_dev", "st_ino", "st_mode", "st_uid", "st_gid"))
    if (_contract(root) != contract or _ancestors(source) != selected["ancestor_identity"]
            or root_after != selected["root_identity"]):
        raise Incomplete("Source context or ancestor authority changed during materialization")
    config = cargo_config(vendor)
    manifest = {"schema_version": 1, "scope": "private EraWindow/Garde selected vendor source only",
                "contract": contract, "vendor_root": str(vendor), "config_sha256": _digest(config),
                "inventory": _inventory(selected), "source_observation": {
                    "root": str(source), "selected_inventory_sha256": _digest(_encoded(_inventory(selected))),
                    "namespace_bookends_sha256": _digest(_encoded(source_roots)),
                    "authority": {"root": selected["root_identity"], "ancestors": selected["ancestor_identity"]}},
                "native_commands_executed": 0, "shipping_graph_qualified": False,
                "allocation_model": "Kani allocation-success assumption retained; host OOM recovery unproved."}
    _write_new(destination / "cargo-config.toml", config)
    _write_new(destination / "manifest.json", _encoded(manifest))
    directories = [path for path in destination.rglob("*") if path.is_dir()]
    for directory in sorted(directories, key=lambda item: len(item.parts), reverse=True):
        directory.chmod(0o500)
    destination.chmod(0o500)
    return admit(root, destination, work_root=work_root)


def admit(root: Path, bundle_root: Path, *, work_root=None) -> dict:
    """Fresh whole-file admission of the exact selected view; write/execute nothing."""
    root = _canonical(root)
    bundle = _canonical(bundle_root, private=True)
    _scope(root, bundle, work_root)
    ancestors_before = _ancestors(bundle)
    before = _snapshot(bundle, sealed=True)
    if {path.name for path in bundle.iterdir()} != {"vendor", "cargo-config.toml", "manifest.json"}:
        raise Incomplete("Vendor bundle has missing or unexpected members")
    contract = _contract(root)
    vendor = bundle / "vendor"
    wanted = {record["name"] + "-" + record["version"] for record in contract["packages"]}
    if {path.name for path in vendor.iterdir()} != wanted:
        raise Incomplete("Selected vendor view has missing or unexpected packages")
    selected = _selected(vendor, contract, sealed=True)
    config_bytes = CUSTODY.read_file(bundle / "cargo-config.toml", maximum=MAX_CONTEXT_BYTES)
    if config_bytes != cargo_config(vendor):
        raise Incomplete("Cargo configuration does not map only the selected offline view")
    manifest_bytes = CUSTODY.read_file(bundle / "manifest.json", maximum=MAX_MANIFEST_BYTES)
    manifest = _json(manifest_bytes)
    expected_keys = {"schema_version", "scope", "contract", "vendor_root", "config_sha256", "inventory",
                     "source_observation", "native_commands_executed", "shipping_graph_qualified", "allocation_model"}
    if (type(manifest) is not dict or set(manifest) != expected_keys or type(manifest["schema_version"]) is not int
            or manifest["schema_version"] != 1 or _encoded(manifest["contract"]) != _encoded(contract)
            or manifest["scope"] != "private EraWindow/Garde selected vendor source only"
            or manifest["vendor_root"] != str(vendor) or manifest["config_sha256"] != _digest(config_bytes)
            or manifest["inventory"] != _inventory(selected) or type(manifest["native_commands_executed"]) is not int
            or manifest["native_commands_executed"] != 0 or manifest["shipping_graph_qualified"] is not False
            or manifest["allocation_model"] != "Kani allocation-success assumption retained; host OOM recovery unproved."):
        raise Incomplete("Vendor bundle manifest differs from its authenticated current contents")
    observation = manifest["source_observation"]
    if (type(observation) is not dict or set(observation) != {
            "root", "selected_inventory_sha256", "namespace_bookends_sha256", "authority"}
            or type(observation["root"]) is not str or not Path(observation["root"]).is_absolute()
            or type(observation["namespace_bookends_sha256"]) is not str
            or not HEX.fullmatch(observation["namespace_bookends_sha256"])
            or type(observation["authority"]) is not dict
            or set(observation["authority"]) != {"root", "ancestors"}
            or observation["selected_inventory_sha256"] != _digest(_encoded(_inventory(selected)))):
        raise Incomplete("Vendor source observation does not bind the selected inventory")
    after = _snapshot(bundle, sealed=True)
    ancestors_after = _ancestors(bundle)
    if before != after or ancestors_before != ancestors_after or _contract(root) != contract:
        raise Incomplete("Selected vendor view or authority changed during admission")
    inventory = {"vendor/" + name: digest for name, digest in _inventory(selected).items()}
    inventory.update({"cargo-config.toml": _digest(config_bytes), "manifest.json": _digest(manifest_bytes)})
    return {"readonly_root": bundle, "vendor_root": vendor, "config_path": bundle / "cargo-config.toml",
            "manifest_path": bundle / "manifest.json", **{key: contract[key] for key in (
                "policy_sha256", "lock_sha256", "projection_sha256", "verifier_semantics_sha256")},
            "manifest_sha256": _digest(manifest_bytes), "config_sha256": _digest(config_bytes),
            "selected_packages": selected["packages"], "inventory": inventory,
            "tree_identity_before": before, "tree_identity_after": after,
            "ancestor_identity_before": ancestors_before, "ancestor_identity_after": ancestors_after,
            "selected_file_count": selected["files"], "selected_bytes": selected["bytes"],
            "native_commands_executed": 0, "shipping_graph_qualified": False,
            "work_scope_checked": work_root is not None}


def identity(admitted: dict) -> dict:
    """Stable complete payload/context/authority record for command bookends."""
    return {key: admitted[key] for key in (
        "readonly_root", "vendor_root", "config_path", "manifest_path", "policy_sha256", "lock_sha256",
        "projection_sha256", "verifier_semantics_sha256", "manifest_sha256", "config_sha256", "selected_packages",
        "inventory", "tree_identity_before", "tree_identity_after", "ancestor_identity_before", "ancestor_identity_after")}
