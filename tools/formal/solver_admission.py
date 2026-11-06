"""Admit the pinned private Z3 candidate without executing any solver.

Use existing strict JSON, namespace-scope and descriptor/path custody primitives.
Every call streams fresh complete payload hashes and checks exact store bookends.
The acquisition manifest is machine-specific provenance, not a native control.
Public host libraries retain the existing runtime trust boundary; no current
library digest or successful Z3/Kani execution is asserted by this admission.
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import stat
import sys


ROOT = Path(__file__).resolve().parents[2]
POLICY = "policy/formal-solvers.json"
CUSTODY_SOURCE = "tools/crypto-aes-siv-witness/fuzz/input_custody.py"
MAX_POLICY_BYTES = 64 * 1024


def _module(name: str, path: Path):
    """Load an existing repository primitive, never candidate-provided Python."""
    specification = importlib.util.spec_from_file_location(name, path)
    if specification is None or specification.loader is None:
        raise RuntimeError("Cannot load repository admission primitive")
    loaded = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(loaded)
    return loaded


CHECK = _module("formal_solver_analysis", ROOT / "scripts/check-analysis.py")
CUSTODY = _module("formal_solver_input_custody", ROOT / CUSTODY_SOURCE)
Incomplete = CHECK.Incomplete

EXPECTED_POLICY = {
    "schema_version": 1, "name": "z3", "version": "5.3.3",
    "admission_status": "private-machine-specific-candidate",
    "default_root": "/home/david_osipov/.local/share/nevrotick/formal-solvers/z3-5.3.3-linux-x64-candidate",
    "release": {
        "asset_id": 688774600, "tag": "z3-5.3.3",
        "commit": "430d19e697e2ec9bad79ff210d41b4a7a5fcfa3e",
        "url": "https://github.com/Z3Prover/z3/releases/download/z3-5.3.3/z3-5.3.3-x64-glibc-2.39.zip",
        "bytes": 69671358,
        "sha256": "d56f60b06f432f33de5810f7f1a384a339131f361e2d54d08ea83eea14326880",
    },
    "files": {
        "bin/z3": {"bytes": 41746104, "mode": "0500",
                   "sha256": "fcd0ca765d1c0b4eac776f442ec0f10294b9a6416be91fad4e52fa19012c8d01"},
        "LICENSE.txt": {"bytes": 1096, "mode": "0400",
                        "sha256": "e617cad2ab9347e3129c2b171e87909332174e17961c5c3412d0799469111337"},
        "manifest.json": {"bytes": 16776, "mode": "0600",
                          "sha256": "3c5565093f3b0d605ae0ea3dbf531b61b89d1b6b1d2aad7282c8ad49c73efa50"},
    },
    "directories": {"": "0700", "bin": "0700"},
    "host_runtime": "existing-public-host-trusted-base-not-rehashed",
    "native_solver_controls": "pending",
}


def _canonical_directory(value, label: str) -> Path:
    """Require an existing literal absolute directory without redirected parents."""
    text = os.fspath(value)
    if (not isinstance(text, str) or ":" in text
            or any(ord(char) < 32 or 127 <= ord(char) <= 159 for char in text)):
        raise Incomplete(label + " contains a PATH separator or control character")
    path = Path(text)
    if (not path.is_absolute() or path != path.resolve(strict=True)
            or not path.is_dir()):
        raise Incomplete(label + " must be a canonical absolute directory")
    return path


def _ancestors(folder: Path) -> dict:
    """Reject another UID's rename authority over the private solver store.

    Root/current-UID owners are trusted. Group/world-writable ancestors require
    root-owned sticky semantics, such as /tmp, and each child owner is checked.
    Ignore unrelated directory entry churn; retain inode and protection identity.
    """
    identities = {}
    for path in folder.parents:
        info = path.lstat()
        writable = stat.S_IMODE(info.st_mode) & 0o022
        trusted_sticky = info.st_uid == 0 and info.st_mode & stat.S_ISVTX
        if (not stat.S_ISDIR(info.st_mode) or info.st_uid not in {0, os.geteuid()}
                or (writable and not trusted_sticky)):
            raise Incomplete("Solver ancestor has untrusted owner/mode: " + str(path))
        identities[str(path)] = tuple(getattr(info, key) for key in
                                      ("st_dev", "st_ino", "st_mode", "st_uid", "st_gid"))
    return identities


def _snapshot(folder: Path, policy: dict) -> dict:
    """Check every allowed directory/terminal, exact membership and identities."""
    identities = {}
    for relative, mode in policy["directories"].items():
        path = folder / relative
        info = path.lstat()
        if (not stat.S_ISDIR(info.st_mode) or info.st_uid != os.geteuid()
                or stat.S_IMODE(info.st_mode) != int(mode, 8)):
            raise Incomplete("Solver directory owner/kind/mode mismatch: " + relative)
        identities[relative] = CUSTODY._identity(info)
    for relative, expected in policy["files"].items():
        info = (folder / relative).lstat()
        if (not stat.S_ISREG(info.st_mode) or info.st_uid != os.geteuid()
                or info.st_nlink != 1 or info.st_size != expected["bytes"]
                or stat.S_IMODE(info.st_mode) != int(expected["mode"], 8)):
            raise Incomplete("Solver file owner/kind/mode/size/link mismatch: " + relative)
        identities[relative] = CUSTODY._identity(info)
    for relative, expected in (("", {"bin", "LICENSE.txt", "manifest.json"}),
                               ("bin", {"z3"})):
        names = set()
        with os.scandir(folder / relative) as entries:
            for entry in entries:
                if entry.name not in expected or entry.name in names:
                    raise Incomplete("Solver store membership mismatch: " + relative)
                names.add(entry.name)
        if names != expected:
            raise Incomplete("Solver store membership mismatch: " + relative)
    return identities


def _manifest_contract(manifest: dict, policy: dict) -> None:
    """Require acquisition provenance to describe the separately pinned payload."""
    release = policy["release"]
    facts = {"schema_version": 1, "status": "acquired-and-statically-inspected-not-executed",
             "executed": False, "version": policy["version"], "asset_id": release["asset_id"],
             "source_commit": release["commit"], "source_url": release["url"]}
    if (type(manifest) is not dict or any(type(manifest.get(key)) is not type(value)
                                        or manifest[key] != value for key, value in facts.items())):
        raise Incomplete("Solver acquisition manifest/provenance mismatch")
    archive = manifest.get("archive_verified")
    if (type(archive) is not dict or any(type(archive.get(key)) is not type(release[key])
                                       or archive[key] != release[key] for key in ("bytes", "sha256"))):
        raise Incomplete("Solver acquisition archive provenance mismatch")
    payload = manifest.get("payload_files")
    if type(payload) is not dict or set(payload) != {"bin/z3", "LICENSE.txt"}:
        raise Incomplete("Solver acquisition payload inventory mismatch")
    for name, row in payload.items():
        facts = {"bytes": policy["files"][name]["bytes"], "sha256": policy["files"][name]["sha256"],
                 "mode": oct(int(policy["files"][name]["mode"], 8)),
                 "owner_uid": os.geteuid(), "hard_links": 1}
        if (type(row) is not dict or any(type(row.get(key)) is not type(value)
                                       or row[key] != value for key, value in facts.items())):
            raise Incomplete("Solver manifest does not describe admitted payload: " + name)


def admit(root, solver_root=None) -> dict:
    """Return one exact readonly Z3 root after fresh complete bounded admission.

    Reject missing, redirected, aliased, extra, unowned, incorrectly protected or
    changed inputs. Cache no file results, execute nothing and change no files.
    The caller retains work-overlap, namespace, command/profile and post-execution
    custody checks; the returned root must never be broadened to its parent.
    """
    if os.name != "posix" or not sys.platform.startswith("linux") or os.geteuid() == 0:
        raise Incomplete("Solver admission requires an ordinary Linux user")
    checkout = _canonical_directory(root, "Checkout")
    # Reject PATH splitting/control characters and unsafe scope before any hash.
    # The actual record is checked against this fixed policy before byte admission.
    folder = _canonical_directory(EXPECTED_POLICY["default_root"] if solver_root is None else solver_root,
                                  "Solver root")
    CHECK.RUNTIME._narrow(folder)
    if folder.is_relative_to(checkout) or checkout.is_relative_to(folder):
        raise Incomplete("Solver root must be outside and separate from the checkout")
    ancestors_before = _ancestors(folder)
    before = _snapshot(folder, EXPECTED_POLICY)
    policy_path = checkout / POLICY
    policy_identity = CUSTODY._identity(policy_path.lstat())
    policy_bytes = CUSTODY.read_file(policy_path, maximum=MAX_POLICY_BYTES)
    policy = CHECK.decode(policy_bytes)
    # JSON serialization preserves bool/int/float distinctions that Python's
    # ordinary container equality would erase. Keys and every fixed value close.
    if json.dumps(policy, sort_keys=True) != json.dumps(EXPECTED_POLICY, sort_keys=True):
        raise Incomplete("Unreviewed solver policy/schema/pins")
    inventory = {}
    for relative, expected in policy["files"].items():
        path = folder / relative
        observed = CUSTODY.digest(path, maximum=expected["bytes"])
        if observed != expected["sha256"]:
            raise Incomplete("Solver payload SHA256 mismatch: " + relative)
        inventory[relative] = {**expected, "identity": before[relative]}
    manifest_bytes = CUSTODY.read_file(folder / "manifest.json",
                                      maximum=policy["files"]["manifest.json"]["bytes"])
    manifest = CHECK.decode(manifest_bytes)
    _manifest_contract(manifest, policy)
    custody_sha = CUSTODY.digest(ROOT / CUSTODY_SOURCE, maximum=MAX_POLICY_BYTES)
    if (CUSTODY.read_file(policy_path, maximum=MAX_POLICY_BYTES) != policy_bytes
            or CUSTODY._identity(policy_path.lstat()) != policy_identity):
        raise Incomplete("Solver policy changed during admission")
    after = _snapshot(folder, policy)
    ancestors_after = _ancestors(folder)
    if before != after or ancestors_before != ancestors_after:
        raise Incomplete("Solver store or ancestor identity changed during admission")
    executable = folder / "bin/z3"
    return {
        "name": policy["name"], "version": policy["version"],
        "readonly_root": folder, "executable": executable, "path_dir": folder / "bin",
        "sha256": inventory["bin/z3"]["sha256"],
        "executable_sha256": inventory["bin/z3"]["sha256"],
        "policy_path": policy_path, "policy_sha256": hashlib.sha256(policy_bytes).hexdigest(),
        "manifest_sha256": hashlib.sha256(manifest_bytes).hexdigest(),
        "inventory": inventory, "store_identity_before": before, "store_identity_after": after,
        "ancestor_identity_before": ancestors_before, "ancestor_identity_after": ancestors_after,
        "closure": {"host_runtime": policy["host_runtime"],
                    "per_command_library_hashes_checked": False,
                    "public_library_acquisition_snapshot": manifest.get("elf", {}).get("public_library_closure", {}),
                    "custody_source": CUSTODY_SOURCE, "custody_source_sha256": custody_sha},
        "executed": False,
    }
