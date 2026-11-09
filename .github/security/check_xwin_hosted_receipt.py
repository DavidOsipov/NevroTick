#!/usr/bin/env python3
"""Read back a hosted acquisition independently of its success summary."""
from __future__ import annotations

import argparse
import fcntl
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import re
import stat
import sys

ROOT = Path(__file__).resolve().parents[2]
# Independent pre-existing SDK observations: receipt d8a7b7a2d16dee0302b99e18e1f86534
# cd53cc70c773311e139eb7f3e1a4d047. Catalog/tool/selector fields match current
# policy; its later-added terms URL is not reused as present terms acceptance.
LIBRARIES = {
    "crt/lib/x86_64/msvcrt.lib": "d3f725923c6dd639bb5e5c52f3122c6fe084471f7ab0ef5f27d054f56b4d8a7e",
    "sdk/lib/um/x86_64/kernel32.lib": "8ba8dab9adec752047d4f136852e8136d01836bff39bbb56a20ac5f2d56415d8",
    "sdk/lib/ucrt/x86_64/ucrt.lib": "7ef4eac926bf597d2f243f16cdfed7e0db22cb3ca34a1d7e088a84c994a03d66",
}


def load(filename: str):
    spec = importlib.util.spec_from_file_location("xwin_readback_" + filename.replace(".", "_"), ROOT / "scripts" / filename)
    if spec is None or spec.loader is None:
        raise RuntimeError("Acquisition admission helper unavailable")
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    return module


BOUNDED = load("bounded_download.py")
HOSTED = load("xwin_hosted.py")


def unique(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("Duplicate input key")
        result[key] = value
    return result


def read_json(path: Path, maximum: int):
    return decode_json(BOUNDED.bounded_file_bytes(path, maximum))


def finite_float(text: str) -> float:
    result = float(text)
    if not math.isfinite(result):
        raise ValueError("Non-finite JSON number")
    return result


def forbidden_constant(text: str):
    raise ValueError("Non-JSON number: " + text)


def decode_json(data: bytes):
    return json.loads(data, object_pairs_hook=unique, parse_float=finite_float, parse_constant=forbidden_constant)


def require(condition, message):
    if not condition:
        raise ValueError(message)


def read_at(directory: int, relative: str, maximum: int) -> bytes:
    """Anchor every untrusted path component to a held directory descriptor."""
    require(type(maximum) is int and 0 < maximum <= 64 * 1024 ** 2, "Readback budget differs")
    parts = relative.split("/")
    require(parts and all(part not in ("", ".", "..") for part in parts), "Invalid readback path")
    current = os.dup(directory)
    device = os.fstat(directory).st_dev
    try:
        for part in parts[:-1]:
            nested = os.open(part, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC, dir_fd=current)
            os.close(current)
            current = nested
            require(os.fstat(current).st_dev == device, "Readback crosses its admitted filesystem")
        descriptor = os.open(parts[-1], os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC,
                             dir_fd=current)
        try:
            info = os.fstat(descriptor)
            require(stat.S_ISREG(info.st_mode) and info.st_dev == device and info.st_nlink == 1,
                    "Readback is linked, special or on another device")
            if not 0 <= info.st_size <= maximum:
                raise RuntimeError("Readback exceeds its admitted byte budget")
            body = bytearray()
            while len(body) < info.st_size:
                block = os.read(descriptor, min(65536, info.st_size - len(body)))
                require(block, "Readback body ended early")
                body.extend(block)
            require(not os.read(descriptor, 1), "Readback body grew")
            after = os.fstat(descriptor)
            require(all(getattr(info, name) == getattr(after, name) for name in
                        ("st_dev", "st_ino", "st_mode", "st_nlink", "st_size", "st_mtime_ns", "st_ctime_ns")),
                    "Opened readback body changed")
            return bytes(body)
        finally:
            os.close(descriptor)
    finally:
        os.close(current)


def lock_owner(profile_descriptor: int) -> int:
    """Acquire the producer's existing lock; caller closes the returned FD."""
    descriptor = os.open(".owner.lock", os.O_RDWR | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC,
                         dir_fd=profile_descriptor)
    try:
        info = os.fstat(descriptor)
        require(stat.S_ISREG(info.st_mode) and info.st_uid == os.geteuid()
                and stat.S_IMODE(info.st_mode) == 0o600 and info.st_nlink == 1
                and info.st_size <= 4096, "Producer lock custody differs")
        fcntl.flock(descriptor, fcntl.LOCK_EX | fcntl.LOCK_NB)
        return descriptor
    except BaseException:
        os.close(descriptor)
        raise


def check(profile: Path, receipt: Path) -> dict:
    """Observe pinned inputs, actual finite files and three independent answers.

    Reuse the challenged no-follow inventory mechanism, with independent expected
    paths/pins and a separate post-producer invocation. No acquisition, compiler,
    native Windows code or artifact export runs here. The caller bounds wall time.
    """
    require(sys.platform == "linux" and os.getuid() == os.geteuid() != 0, "Ordinary Linux runner required")
    require(os.environ.get("GITHUB_ACTIONS") == "true"
            and os.environ.get("RUNNER_ENVIRONMENT") == "github-hosted"
            and os.environ.get("RUNNER_OS") == "Linux", "Hosted Linux declaration missing")
    require(profile.is_absolute() and profile.resolve(strict=True) == profile, "Noncanonical profile")
    require(profile.parent == Path(os.environ["RUNNER_TEMP"])
            and not profile.is_relative_to(ROOT), "Unexpected profile owner")
    info = profile.lstat()
    require(stat.S_ISDIR(info.st_mode) and info.st_uid == os.getuid()
            and stat.S_IMODE(info.st_mode) == 0o700, "Profile custody changed")
    require(receipt == profile / "payload/receipt.json", "Unexpected receipt destination")
    payload = profile / "payload"
    require(payload.resolve(strict=True) == payload, "Linked payload root")
    profile_descriptor = os.open(profile, os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC)
    descriptor = None
    cache_descriptor = None
    owner_descriptor = None
    try:
        require((os.fstat(profile_descriptor).st_dev, os.fstat(profile_descriptor).st_ino)
                == (info.st_dev, info.st_ino), "Profile changed before readback")
        owner_descriptor = lock_owner(profile_descriptor)
        descriptor = os.open("payload", os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC,
                             dir_fd=profile_descriptor)
        observed = os.fstat(descriptor)
        capacity = os.fstatvfs(descriptor)
        total_bytes = capacity.f_frsize * capacity.f_blocks
        require(observed.st_uid == os.getuid() and stat.S_IMODE(observed.st_mode) == 0o700
                and observed.st_dev != info.st_dev and 0 < total_bytes <= 2 * 1024 ** 3
                and 0 < capacity.f_files <= 65_536, "Actual quota/custody exceeds contract")
        mounts = []
        with open("/proc/self/mountinfo", "rb") as stream:
            data = stream.read(4 * 1024 ** 2 + 1)
        require(len(data) <= 4 * 1024 ** 2, "Mount table exceeds bound")
        for line in data.decode().splitlines():
            before, after = line.split(" - ", 1)
            fields, filesystem = before.split(), after.split()
            mountpoint = re.sub(r"\\([0-7]{3})", lambda m: chr(int(m[1], 8)), fields[4])
            if mountpoint == str(payload):
                mounts.append((fields, filesystem))
            require(fields[2] != f"{os.major(observed.st_dev)}:{os.minor(observed.st_dev)}"
                    or mountpoint == str(payload), "Aliased payload filesystem")
            require(not mountpoint.startswith(str(payload) + "/"), "Nested payload mount")
        require(len(mounts) == 1, "Dedicated mount missing or shadowed")
        fields, filesystem = mounts[0]
        require(filesystem[0] == "tmpfs" and fields[3] == "/"
                and {"noexec", "nodev", "nosuid"} <= set(fields[5].split(","))
                and fields[2] == f"{os.major(observed.st_dev)}:{os.minor(observed.st_dev)}",
                "Effective mount differs from requested profile")
        policy = read_json(ROOT / "policy/tool-manifest.json", 1024 ** 2)["cross_toolchain"]
        receipt_data = read_at(descriptor, "receipt.json", 2 * 1024 ** 2)
        result = decode_json(receipt_data)
        cache = payload / "cache"
        splat = cache / "splat"
        require(type(result["schema_version"]) is int and result["schema_version"] == 1 and result["root"] == str(splat)
                and result["microsoft_inputs"] == policy["microsoft_inputs"], "Pinned input identity differs")
        require(all(result["xwin"][key] == value for key, value in policy["xwin"].items()), "Tool pin differs")
        require(result["library_paths"] == [str(splat / suffix) for suffix in
                ("crt/lib/x86_64", "sdk/lib/um/x86_64", "sdk/lib/ucrt/x86_64")], "Library route differs")
        hosted = result["hosted"]
        require(hosted["profile"] == str(profile) and hosted["payload"] == str(payload)
                and hosted["cache"] == str(cache) and type(hosted["uid"]) is int
                and hosted["uid"] == os.getuid(), "Hosted owner differs")
        mount = hosted["mount"]
        require(type(mount["mount_id"]) is int and mount["mount_id"] == int(fields[0])
                and mount["device"] == fields[2] and mount["fstype"] == "tmpfs"
                and mount["options"] == sorted(fields[5].split(","))
                and type(mount["total_bytes"]) is int and mount["total_bytes"] == total_bytes
                and type(mount["total_inodes"]) is int and mount["total_inodes"] == capacity.f_files,
                "Parent mount facts differ from actual readback")
        supervisor = hosted["supervisor"]
        for key, value in {"whole_seconds": 600, "exit_code": 0, "max_output_bytes": 2 * 1024 ** 2,
                           "as_bytes": 1024 ** 3, "cpu_seconds": 600, "nproc": 128,
                           "nofile": 256, "fsize": 2 * 1024 ** 3}.items():
            require(type(supervisor[key]) is int and supervisor[key] == value, "Execution limit differs: " + key)
        require(supervisor["failure"] is None and supervisor["pid_namespace"] is True
                and type(supervisor["elapsed_seconds"]) in (int, float)
                and 0 <= supervisor["elapsed_seconds"] < 600
                and type(supervisor["output_bytes"]) is int and 0 <= supervisor["output_bytes"] <= 2 * 1024 ** 2,
                "Actual supervisor outcome differs")
        inventory = result["inventory"]
        require(type(inventory["schema_version"]) is int and inventory["schema_version"] == 1
                and type(inventory["bytes"]) is int and type(inventory["file_count"]) is int
                and inventory["path"] == str(payload / "inventory.json")
                and inventory["scope_root"] == str(cache), "Inventory route differs")
        raw = read_at(descriptor, "inventory.json", 16 * 1024 ** 2)
        require(len(raw) == inventory["bytes"] and hashlib.sha256(raw).hexdigest() == inventory["sha256"], "Inventory bytes differ")
        expected = decode_json(raw)
        require(set(expected) == {"schema_version", "files"} and type(expected["schema_version"]) is int
                and expected["schema_version"] == 1, "Inventory schema differs")
        cache_descriptor = os.open("cache", os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC,
                                   dir_fd=descriptor)
        require(os.fstat(cache_descriptor).st_dev == observed.st_dev, "Cache device differs")
        actual = HOSTED.inventory(cache, directory_fd=cache_descriptor)
        require(actual == expected["files"] and len(actual) == inventory["file_count"], "Actual cache inventory differs")
        for relative, expected_hash in LIBRARIES.items():
            require(actual["splat/" + relative]["sha256"] == expected_hash, "Independent library answer differs")
        require(result["library_files"] == [{"path": str(splat / relative), **actual["splat/" + relative]}
                for relative in LIBRARIES], "Required library observations differ")
        inputs = policy["microsoft_inputs"]
        channel = "provenance/manifest_" + inputs["manifest_version"] + ".json"
        declared = inputs.get("package_manifest_declared_sha256", inputs["package_manifest_sha256"])
        package = "dl/pkg_manifest_" + declared.lower() + ".vsman"
        require(result["channel_manifest"] == str(cache / channel)
                and actual[channel]["sha256"] == inputs["channel_manifest_sha256"]
                and result["package_manifest"] == str(cache / package)
                and actual[package]["sha256"] == inputs["package_manifest_sha256"], "Actual pinned metadata differs")
        terms = result["terms"]
        require(terms["path"] == str(cache / "provenance/microsoft-buildtools-terms.docx"), "Terms route differs")
        require(terms["accepted_for_development_acquisition"] is True and terms["redistribution_clearance"] is False
                and terms["url"] == policy["microsoft_inputs"]["terms_document_url"], "Current terms differ")
        require(hashlib.sha256(read_at(cache_descriptor, "provenance/microsoft-buildtools-terms.docx", 1024 ** 2)).hexdigest()
                == policy["microsoft_inputs"]["terms_document_sha256"], "Actual terms differ")
        archive = profile / "bootstrap" / policy["xwin"]["url"].rsplit("/", 1)[1]
        tool_bytes = read_at(profile_descriptor, str(archive.relative_to(profile)), 32 * 1024 ** 2)
        require(hashlib.sha256(tool_bytes).hexdigest() == policy["xwin"]["sha256"], "Actual bootstrap archive differs")
        members = HOSTED.tar_members(tool_bytes, policy["xwin"]["version"])
        executable = profile / "bootstrap/bin/xwin"
        require(result["xwin"]["binary"] == str(executable), "Executable route differs")
        executable_hash = hashlib.sha256(read_at(profile_descriptor, "bootstrap/bin/xwin", 64 * 1024 ** 2)).hexdigest()
        require(executable_hash == hashlib.sha256(members["xwin"]).hexdigest()
                == result["xwin"]["binary_sha256"], "Installed executable differs from pinned archive")
        final_profile = profile.lstat()
        final_payload = os.stat("payload", dir_fd=profile_descriptor, follow_symlinks=False)
        final_cache = os.stat("cache", dir_fd=descriptor, follow_symlinks=False)
        final_lock = os.stat(".owner.lock", dir_fd=profile_descriptor, follow_symlinks=False)
        held_cache = os.fstat(cache_descriptor)
        held_lock = os.fstat(owner_descriptor)
        require((final_profile.st_dev, final_profile.st_ino) == (info.st_dev, info.st_ino)
                and (final_payload.st_dev, final_payload.st_ino) == (observed.st_dev, observed.st_ino)
                and (final_cache.st_dev, final_cache.st_ino) == (held_cache.st_dev, held_cache.st_ino)
                and (final_lock.st_dev, final_lock.st_ino) == (held_lock.st_dev, held_lock.st_ino),
                "Readback roots changed before publication")
        return {"status": "XWIN_HOSTED_READBACK_OK", "total_bytes": total_bytes,
                "total_inodes": capacity.f_files, "actual_cache_files": len(actual),
                "actual_cache_bytes": sum(row["bytes"] for row in actual.values()),
                "receipt_sha256": hashlib.sha256(receipt_data).hexdigest(),
                "inventory_sha256": inventory["sha256"], "tool_archive_sha256": policy["xwin"]["sha256"],
                "xwin_binary_sha256": executable_hash,
                "channel_sha256": actual[channel]["sha256"], "package_sha256": actual[package]["sha256"],
                "terms_sha256": actual["provenance/microsoft-buildtools-terms.docx"]["sha256"],
                "independent_libraries": LIBRARIES, "native_windows": False, "retained_sdk_export": False}
    finally:
        if cache_descriptor is not None:
            os.close(cache_descriptor)
        if descriptor is not None:
            os.close(descriptor)
        if owner_descriptor is not None:
            os.close(owner_descriptor)
        os.close(profile_descriptor)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--profile", type=Path, required=True)
    parser.add_argument("--receipt", type=Path, required=True)
    args = parser.parse_args()
    print(json.dumps(check(args.profile, args.receipt), sort_keys=True))
