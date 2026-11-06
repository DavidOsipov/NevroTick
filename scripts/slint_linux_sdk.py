"""Admit the optional private cloud Fontconfig SDK, never caller-provided pins."""
from __future__ import annotations

import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import stat

POLICY = "policy/slint-linux-sdk.json"
REVIEWED_POLICY_SHA256 = "d1337318868e9aa66ee02606f3d77e89ec6f8b8e8319ce4e414f5c40ec7a82e5"
MAX_FILES = 512
MAX_BYTES = 32 * 1024 * 1024


class Incomplete(ValueError):
    """The optional native input is not the exact reviewed SDK."""


def digest(path: Path) -> str:
    """Hash one finite regular file; links never supply their target implicitly."""
    if path.is_symlink() or not path.is_file() or path.stat().st_size > MAX_BYTES:
        raise Incomplete("Native SDK expected a bounded regular file.")
    return hashlib.sha256(path.read_bytes()).hexdigest()


def inventory(directory: Path) -> dict:
    """Bound all entries and bytes, reject special files and escaping links."""
    if directory.is_symlink() or not directory.is_dir():
        raise Incomplete("Native SDK root must be a regular directory.")
    directory = directory.resolve(strict=True)
    files, links, directories = {}, {}, []
    total = 0
    for count, path in enumerate(directory.rglob("*"), 1):
        if count > MAX_FILES:
            raise Incomplete("Native SDK entry count exceeds its bound.")
        name = path.relative_to(directory).as_posix()
        if len(name) > 512:
            raise Incomplete("Native SDK path exceeds its bound.")
        mode = path.lstat().st_mode
        if stat.S_ISLNK(mode):
            target = os.readlink(path)
            if (len(target) > 512 or PurePosixPath(target).is_absolute()
                    or not path.resolve(strict=True).is_relative_to(directory)):
                raise Incomplete("Native SDK link leaves the reviewed root.")
            links[name] = target
        elif stat.S_ISDIR(mode):
            directories.append(name)
        elif stat.S_ISREG(mode):
            total += path.stat().st_size
            if total > MAX_BYTES:
                raise Incomplete("Native SDK aggregate bytes exceed their bound.")
            files[name] = digest(path)
        else:
            raise Incomplete("Native SDK contains a special file.")
    return {"files": files, "symlinks": links, "directories": sorted(directories)}


def admit(directory: Path, policy_path: Path, source_root: Path | None = None) -> dict:
    """Compare every SDK/native-tool input to independently pinned source data."""
    if REVIEWED_POLICY_SHA256 is None or digest(policy_path) != REVIEWED_POLICY_SHA256:
        raise Incomplete("Native SDK policy is not independently admitted.")
    if policy_path.stat().st_size > 256 * 1024:
        raise Incomplete("Native SDK policy exceeds its bound.")
    policy = json.loads(policy_path.read_bytes().decode("utf-8"))
    expected = {"schema_version", "scope", "files", "symlinks", "directories",
                "pkg_config_libdir", "pkg_config_sysroot_dir", "system_runtime_files", "pkg_config",
                "system_runtime_aliases", "source_files"}
    if (type(policy) is not dict or set(policy) != expected
            or type(policy["schema_version"]) is not int or policy["schema_version"] != 1
            or policy["pkg_config_libdir"] != "/native/usr/lib/x86_64-linux-gnu/pkgconfig:/native/usr/share/pkgconfig"
            or policy["pkg_config_sysroot_dir"] != "/native"):
        raise Incomplete("Native SDK policy has an unselected schema or path.")
    observed = inventory(directory)
    if any(observed[key] != policy[key] for key in observed):
        raise Incomplete("Native SDK bytes, links or directories differ from review.")
    for name, expected_hash in policy["system_runtime_files"].items():
        if digest(Path(name)) != expected_hash:
            raise Incomplete("Native SDK host runtime differs from review.")
    for name, target in policy["system_runtime_aliases"].items():
        if not Path(name).is_symlink() or os.readlink(name) != target:
            raise Incomplete("Native SDK host runtime alias differs from review.")
    source_root = source_root if source_root is not None else policy_path.parent.parent
    for name, expected_hash in policy["source_files"].items():
        relative = PurePosixPath(name)
        if relative.is_absolute() or ".." in relative.parts or digest(source_root / name) != expected_hash:
            raise Incomplete("Native SDK font/XML inputs differ from review.")
    font_directory = "crates/nevrotick-ui/tests/fixtures/fonts"
    font_prefix = font_directory + "/"
    font_files = {name.removeprefix(font_prefix): expected_hash
                  for name, expected_hash in policy["source_files"].items() if name.startswith(font_prefix)}
    if font_files:
        fonts = inventory(source_root / font_directory)
        if fonts != {"files":font_files,"symlinks":{},"directories":[]}:
            raise Incomplete("Native SDK font directory contains unreviewed inputs.")
    tool = policy["pkg_config"]
    if digest(Path(tool["path"])) != tool["sha256"]:
        raise Incomplete("Native SDK pkg-config differs from review.")
    return {"policy_sha256": REVIEWED_POLICY_SHA256, "inventory": observed,
            "pkg_config": tool, "system_runtime_files": policy["system_runtime_files"],
            "system_runtime_aliases": policy["system_runtime_aliases"], "source_files": policy["source_files"]}


def sandbox_arguments(directory: Path) -> list[str]:
    """Mount an already admitted SDK read-only and select its real .pc/ELF files."""
    return ["--ro-bind", str(directory), "/native",
            "--setenv", "PKG_CONFIG", "/usr/bin/pkgconf",
            "--setenv", "PKG_CONFIG_LIBDIR", "/native/usr/lib/x86_64-linux-gnu/pkgconfig:/native/usr/share/pkgconfig",
            "--setenv", "PKG_CONFIG_SYSROOT_DIR", "/native",
            "--setenv", "LD_LIBRARY_PATH", "/native/usr/lib/x86_64-linux-gnu",
            "--setenv", "FONTCONFIG_FILE", "/work/fixture/crates/nevrotick-ui/tests/fixtures/fonts.conf",
            "--setenv", "SLINT_DEFAULT_FONT", "/work/fixture/crates/nevrotick-ui/tests/fixtures/fonts/Inter-VariableFont.ttf"]
