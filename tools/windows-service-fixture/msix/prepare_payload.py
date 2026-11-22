# SPDX-License-Identifier: GPL-2.0-only
"""Stage the disabled-service fixture and its optional pinned packer inputs."""

import argparse
import ctypes
import errno
import hashlib
import json
import os
from pathlib import Path
import stat
import struct
import sys
import zlib

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
RECEIPTS = ROOT / "artifacts/windows-services/recovery-20261107-v1"
BINS = ("nevrotick-lab-s02-engine", "nevrotick-lab-s02-broker")
# The six files from the previously admitted offline MakeAppx fixture run.
SDK_SHA256 = {
    "Microsoft.Windows.Build.Appx.AppxPackaging.dll.manifest":
        "b50d228857d9cd6509854feacdc153a78804be4312bf699298dbf6a5bd846ddd",
    "Microsoft.Windows.Build.Appx.OpcServices.dll.manifest":
        "7babce6a576b434c71b93797ac6968963a9c23feea56d8d45faeeaa8175907ec",
    "appxpackaging.dll": "3437f43e925cdf3af77ffba2e85a97d0413644b35428335f0f8b1f52fcd1ba12",
    "en-US/AppxPackaging.dll.mui": "faf1505fb4a8ca5518808841801c6ff9f9e97de65feb0a77aa2b41c3e34501a0",
    "makeappx.exe": "98ddf5a4fdb38a1d32c2ca7b350a2012ed430d277baa1bf03ac91ef82365f1d3",
    "opcservices.dll": "11e20c4673d403c6aa2ee2f7ea85194c2e430e5d4245fcbaba207a6a80e48f24",
}
sys.path.insert(0, str(ROOT / "scripts"))
sys.path.insert(0, str(ROOT))
from metadata.pipeline.generate import (
    PublicationUncertainError, _new_staging_directory, _open_directory,
    _outside_source, _parent_unchanged, _remove_staging, _write_payload,
)
from windows_pe import read_export_pair


def require(condition, message):
    """Refuse a mismatched fixture input before producing payload files."""
    if not condition:
        raise ValueError(message)


def digest(data):
    """Return a lowercase SHA-256 of the actual bytes."""
    return hashlib.sha256(data).hexdigest()


def file_version(info):
    """Bind object identity, mode and content timestamps without access time."""
    return (info.st_dev, info.st_ino, info.st_mode, info.st_size, info.st_mtime_ns, info.st_ctime_ns)


def read_bound(path, ceiling, expected):
    """Read a small regular non-linked file and match its previously admitted hash."""
    require(path.is_file() and path.resolve(strict=True) == path, "Expected a regular owned input.")
    flags = os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC
    with os.fdopen(os.open(path, flags), "rb") as stream:
        before = os.fstat(stream.fileno())
        require(stat.S_ISREG(before.st_mode) and 0 < before.st_size <= ceiling,
                "Expected a bounded regular input.")
        data = stream.read(ceiling + 1)
        require(file_version(before) == file_version(os.fstat(stream.fileno()))
                == file_version(path.lstat()),
                "Fixture input changed during read.")
    require(0 < len(data) <= ceiling and digest(data) == expected, "Fixture input hash or size differs.")
    return data


def role_images(receipt, receipt_hash):
    """Bind exact fixed-role images and their symbols to the admitted link stage."""
    require(receipt.parent == RECEIPTS and receipt.name.startswith("cross-")
            and receipt.suffix == ".json", "Use a configured cross-link receipt.")
    data = json.loads(read_bound(receipt, 4 * 1024**2, receipt_hash))
    require(data["status"] == "passed" and data["lane"] == "link"
            and data["bookends_passed"] is True and data["index_bytes_unchanged"] is True,
            "Expected a passed, complete two-role link receipt.")
    rows = data["artifacts"]
    require(set(rows) == set(BINS), "The exact two service roles are required.")
    linked = receipt.parent / (receipt.stem + "-artifacts")
    require(linked.resolve(strict=True) == linked, "The export directory must be canonical.")
    images = {}
    for stem in BINS:
        row = rows[stem]
        image = linked / (stem + ".exe")
        symbols = linked / (stem.replace("-", "_") + ".pdb")
        require(ROOT / row["image"] == image and ROOT / row["pdb"] == symbols,
                "The receipt maps a role to a different image or symbol file.")
        (payload, image_hash), (_, symbol_hash), _ = read_export_pair(
            image, symbols, subsystem="console",
        )
        require(image_hash == row["image_sha256"] and symbol_hash == row["pdb_sha256"],
                "A service image or matching symbol file differs from the link receipt.")
        images[image.name] = payload
    require(len({digest(payload) for payload in images.values()}) == len(BINS),
            "Each service requires a distinct image.")
    return images


def png_logo(size):
    """Generate a deterministic square, solid-color PNG for the hidden lab app."""
    def chunk(kind, data):
        """Encode a PNG chunk with its own length and CRC."""
        return struct.pack(">I", len(data)) + kind + data + struct.pack(
            ">I", zlib.crc32(kind + data),
        )

    header = struct.pack(">IIBBBBB", size, size, 8, 2, 0, 0, 0)
    rows = (b"\0" + bytes((25, 58, 94)) * size) * size
    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", header)
            + chunk(b"IDAT", zlib.compress(rows, level=9)) + chunk(b"IEND", b""))


def write_group(staging: int, name: str, files: dict[str, bytes]) -> None:
    """Write a complete fixed group below the owned staging directory."""
    os.mkdir(name, mode=0o700, dir_fd=staging)
    flags = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC
    group = os.open(name, flags, dir_fd=staging)
    try:
        for relative, data in files.items():
            _write_payload(group, relative, data)
        # Nested leaves flush their own directory. Commit those directory
        # entries in this group before it becomes part of the final tree.
        os.fsync(group)
    finally:
        os.close(group)


def publish_new(parent: int, temporary: str, name: str) -> None:
    """Publish on Linux without replacing even a concurrently created empty slot."""
    try:
        rename = ctypes.CDLL(None, use_errno=True).renameat2
    except AttributeError as error:
        raise OSError(errno.ENOSYS, "Atomic no-replace publication unavailable") from error
    rename.argtypes = [ctypes.c_int, ctypes.c_char_p, ctypes.c_int, ctypes.c_char_p, ctypes.c_uint]
    rename.restype = ctypes.c_int
    rename_noreplace = 1
    if rename(parent, os.fsencode(temporary), parent, os.fsencode(name), rename_noreplace) != 0:
        code = ctypes.get_errno()
        raise OSError(code, os.strerror(code))


def stage_payload(
    output: Path, payload: dict[str, bytes], sdk_source: Path | None,
) -> dict[str, str | dict[str, str]]:
    """Commit a complete payload or pack-input tree in an owned external parent.

    A precommit error leaves the final slot intact. After rename, a parent-flush
    failure retains the complete tree and reports uncertain durability.
    """
    output = _outside_source(ROOT, output)
    parent = _open_directory(output.parent, private=True)
    temporary = staging = None
    try:
        try:
            os.stat(output.name, dir_fd=parent, follow_symlinks=False)
        except FileNotFoundError:
            pass
        else:
            raise FileExistsError(errno.EEXIST, os.strerror(errno.EEXIST))

        payload_hashes = {name: digest(data) for name, data in payload.items()}
        temporary, staging = _new_staging_directory(parent)
        if sdk_source is None:
            for name, data in payload.items():
                _write_payload(staging, name, data)
            report = payload_hashes
        else:
            sdk = {name: read_bound(sdk_source / name, 3 * 1024**2, expected)
                   for name, expected in SDK_SHA256.items()}
            write_group(staging, "payload", payload)
            write_group(staging, "sdk", sdk)
            inventory = {"sdk": SDK_SHA256, "payload": payload_hashes}
            inventory_bytes = (json.dumps(inventory, sort_keys=True) + "\n").encode("utf-8")
            _write_payload(staging, "input-set.json", inventory_bytes)
            report = {**inventory, "input_set_sha256": digest(inventory_bytes)}

        os.fsync(staging)
        _parent_unchanged(output.parent, parent)
        publish_new(parent, temporary, output.name)
        temporary = None
        try:
            os.fsync(parent)
        except OSError as error:
            raise PublicationUncertainError() from error
        return report
    except BaseException as error:
        if temporary is not None and staging is not None:
            try:
                _remove_staging(parent, temporary, staging)
            except (OSError, ValueError) as cleanup:
                error.add_note(f"Private staging cleanup failed: {cleanup}")
        raise
    finally:
        try:
            if staging is not None:
                os.close(staging)
        finally:
            os.close(parent)


def main():
    """Build only the private source payload; the Windows packer runs separately."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--receipt-sha256", required=True)
    parser.add_argument("--inert", type=Path, required=True)
    parser.add_argument("--inert-sha256", required=True)
    parser.add_argument("--sdk-source", type=Path,
                        help="stage the six pinned MakeAppx inputs beside payload/ and input-set.json")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    try:
        payload = role_images(args.receipt.absolute(), args.receipt_sha256)
        payload["AppxManifest.xml"] = (HERE / "AppxManifest.xml").read_bytes()
        payload["Inert.exe"] = read_bound(args.inert.absolute(), 64 * 1024, args.inert_sha256)
        for name, size in (("PackageLogo", 50), ("Square44x44Logo", 44), ("Square150x150Logo", 150)):
            payload[f"Assets/{name}.png"] = png_logo(size)
        sdk_source = args.sdk_source.absolute() if args.sdk_source is not None else None
        report = stage_payload(args.output, payload, sdk_source)
    except PublicationUncertainError:
        parser.exit(3, "Pack input publication durability unconfirmed. Check the existing output before retrying.\n")
    except (OSError, KeyError, TypeError, ValueError) as error:
        messages = (str(error), *getattr(error, "__notes__", ()))
        parser.exit(1, "\n".join(messages) + "\n")
    print(json.dumps(report, sort_keys=True))


if __name__ == "__main__":
    main()
