# SPDX-License-Identifier: GPL-2.0-only
"""Stage the disabled-service fixture and its optional pinned packer inputs."""

import argparse
from collections.abc import Iterator
from contextlib import contextmanager
import ctypes
import errno
import hashlib
import json
import os
from pathlib import Path
import re
import stat
import struct
import sys
import zlib

ROOT = Path(__file__).resolve().parents[3]
HERE = Path(__file__).resolve().parent
RECEIPTS = ROOT / "artifacts/windows-services/recovery-20261107-v1"
BINS = ("nevrotick-lab-s02-engine", "nevrotick-lab-s02-broker")
EVENT_RESOURCE_FILE: str = "nevrotick-test-events.dll"
EVENT_RESOURCE_MAX: int = 64 * 1024  # Both files in the selected MD-P4 build output.
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
from metadata.event_resource_events import verify_event_resources
from metadata.event_resource_pe import read_resource_dll
from metadata.pipeline.contract import EventProvider, Metadata, MetadataError
from metadata.pipeline.delivery import _equal, _json
from metadata.pipeline.generate import (
    PublicationUncertainError, _new_staging_directory, _open_directory,
    _outside_source, _parent_unchanged, _read_checked_generation, _read_regular,
    _remove_staging, _write_payload,
)
from windows_pe import read_export_pair, service_preentry_declarations


def require(condition, message):
    """Refuse a mismatched fixture input before producing payload files."""
    if not condition:
        raise ValueError(message)


def digest(data):
    """Return a lowercase SHA-256 of the actual bytes."""
    return hashlib.sha256(data).hexdigest()


def file_version(info: os.stat_result) -> tuple[int, int, int, int, int, int]:
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


@contextmanager
def held_directory(path: Path) -> Iterator[tuple[int, Path]]:
    """Read leaves from one owned directory and confirm its path before returning."""
    descriptor: int = _open_directory(path, private=True)
    try:
        yield descriptor, Path(f"/proc/self/fd/{descriptor}")
        _parent_unchanged(path, descriptor)
    finally:
        os.close(descriptor)


def _checked_role_sidecar(directory: int, name: str, pin: object, maximum: int) -> bytes:
    """Bind the held sidecar bytes to the independently selected link receipt."""
    require(type(pin) is str and re.fullmatch(r"[0-9a-f]{64}", pin) is not None,
            f"The guarded link receipt lacks the {name} digest.")
    data: bytes = _read_regular(directory, name, maximum)
    require(bool(data) and digest(data) == pin, f"Guarded link sidecar differs: {name}.")
    return data


def _check_role_preentry(directory: int, stem: str, row: dict[str, object], payload: bytes) -> None:
    """Recheck the held PE facts and map identity from the guarded link producer.

    The independently pinned link receipt supplies its gate and CRT symbol
    reconciliation. This checks the same map and PE bytes at staging.
    """
    raw_pe: object = row.get("raw_pe")
    guard: object = raw_pe.get("baseline_entry") if type(raw_pe) is dict else None
    require(type(guard) is dict, f"The guarded link receipt lacks {stem} baseline entry facts.")
    observed: dict[str, object] = service_preentry_declarations(payload)
    crt: object = guard.get("crt_startup_rva")
    require(guard.keys() == observed.keys() | {"crt_startup_rva", "map_sha256"}
            and type(crt) is int and 0 < crt < 2**32 and crt != observed["entry_rva"]
            and all(type(guard[key]) is type(value) and guard[key] == value for key, value in observed.items()),
            f"Guarded link facts differ from the held {stem} PE.")
    map_stem: str = stem.replace("-", "_")
    map_data: bytes = _checked_role_sidecar(directory, map_stem + ".map", guard["map_sha256"], 2 * 1024**2)
    require(map_data.partition(b"\n")[0].strip() == map_stem.encode("ascii"),
            f"Guarded link map names a different role: {stem}.")


def _read_roles(rows: dict[str, object], directory: int, anchor: Path, logical: Path, *,
                require_baseline_entry: bool) -> dict[str, bytes]:
    """Check the producer's fixed logical paths and read the matching held pairs."""
    images: dict[str, bytes] = {}
    for stem in BINS:
        row: object = rows[stem]
        image: Path = anchor / (stem + ".exe")
        symbols: Path = anchor / (stem.replace("-", "_") + ".pdb")
        require(type(row) is dict and row.get("image") == (logical / image.name).as_posix()
                and row.get("pdb") == (logical / symbols.name).as_posix(),
                "The receipt maps a role to a different image or symbol file.")
        payload: bytes
        image_hash: str
        symbol_hash: str
        (payload, image_hash), (_, symbol_hash), _ = read_export_pair(
            image, symbols, subsystem="console",
        )
        require(image_hash == row["image_sha256"] and symbol_hash == row["pdb_sha256"],
                "A service image or matching symbol file differs from the link receipt.")
        if require_baseline_entry:
            _check_role_preentry(directory, stem, row, payload)
        images[image.name] = payload
    require(len({digest(payload) for payload in images.values()}) == len(BINS),
            "Each service requires a distinct image.")
    return images


def role_images(receipt: Path, receipt_hash: str, *, require_baseline_entry: bool = False) -> dict[str, bytes]:
    """Recheck the fixed link export in place or after a byte-exact relocation."""
    require(receipt.name.startswith("cross-") and receipt.suffix == ".json",
            "Use a configured cross-link receipt.")
    linked: Path = receipt.parent / (receipt.stem + "-artifacts")
    logical: Path = RECEIPTS.relative_to(ROOT) / linked.name
    with held_directory(receipt.parent) as (parent, _):
        raw: bytes = _read_regular(parent, receipt.name, 4 * 1024**2)
        require(digest(raw) == receipt_hash, "Cross-link receipt hash differs.")
        data: object = _json(raw)
        require(type(data) is dict and data.get("status") == "passed" and data.get("lane") == "link"
                and data.get("bookends_passed") is True and data.get("index_bytes_unchanged") is True,
                "Expected a passed, complete two-role link receipt.")
        rows: object = data["artifacts"]
        require(type(rows) is dict and rows.keys() == set(BINS), "The exact two service roles are required.")
        baseline: object = data.get("baseline_entry")
        with held_directory(linked) as (descriptor, anchor):
            # Compare with the child in the original receipt directory before readback.
            current: os.stat_result = os.stat(linked.name, dir_fd=parent, follow_symlinks=False)
            require(os.path.samestat(current, os.fstat(descriptor)), "The link export directory changed.")
            if require_baseline_entry:
                require(type(baseline) is dict, "Expected a guarded two-role link receipt.")
                _checked_role_sidecar(descriptor, "service-entry.obj", baseline.get("object_sha256"), 64 * 1024)
            return _read_roles(rows, descriptor, anchor, logical, require_baseline_entry=require_baseline_entry)


def event_resource_image(
    build: Path, receipt_hash: str, generation: Path, root: Path, source: Path | None,
) -> bytes:
    """Read and independently verify the fixed lab DLL before any staging writes.

    The externally supplied receipt digest pins input bytes for this fixture.
    Artifact authenticity and installed provider policy have separate owners.
    """
    model: Metadata
    generated: dict[str, bytes]
    model, generated = _read_checked_generation(root, generation, source, stage="source")
    provider: EventProvider | None = model.event_provider
    if provider is None or provider.package_resource_file != EVENT_RESOURCE_FILE:
        raise ValueError("The checked generation must select the fixed lab event resource.")

    directory: int = _open_directory(build)
    try:
        before: tuple[int, int, int, int, int, int] = file_version(os.fstat(directory))
        missing: set[str] = {"receipt.json", EVENT_RESOURCE_FILE}
        entries: Iterator[os.DirEntry[str]]
        with os.scandir(directory) as entries:
            entry: os.DirEntry[str]
            for entry in entries:
                require(entry.name in missing, "Unexpected event resource build inventory.")
                missing.remove(entry.name)
        require(not missing, "Incomplete event resource build inventory.")
        raw_receipt: bytes = _read_regular(directory, "receipt.json", EVENT_RESOURCE_MAX)
        require(digest(raw_receipt) == receipt_hash, "Event resource receipt SHA-256 differs.")
        image: bytes = _read_regular(directory, EVENT_RESOURCE_FILE, EVENT_RESOURCE_MAX)
        require(file_version(os.fstat(directory)) == before, "Event resource build directory changed.")
        _parent_unchanged(build, directory)
    finally:
        os.close(directory)

    try:
        receipt: object = _json(raw_receipt)
    except RecursionError as error:
        raise ValueError("Event resource receipt nesting exceeds parser limits.") from error
    if type(receipt) is not dict:
        raise ValueError("Unexpected event resource receipt profile.")
    require(type(receipt.get("schema_version")) is int and receipt["schema_version"] == 1
            and receipt.get("status") == "linked-and-read-back"
            and receipt.get("target") == "x86_64-pc-windows-msvc"
            and receipt.get("file") == EVENT_RESOURCE_FILE, "Unexpected event resource receipt profile.")
    require(receipt.get("manifest_sha256") == digest(generated["authoring/service-events.man"])
            and receipt.get("generation_receipt_sha256") == digest(generated["receipt.json"]),
            "Event resource receipt does not match the checked generation.")
    require(type(receipt.get("bytes")) is int and 0 < len(image) == receipt["bytes"]
            and receipt.get("sha256") == digest(image), "Event resource DLL size or digest differs.")
    observed: dict[str, object] = verify_event_resources(provider, read_resource_dll(image))
    require(_equal(receipt.get("readback"), observed), "Event resource receipt readback differs from the DLL.")
    return image


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

    A precommit error leaves the final slot intact. After rename, failed
    durability or final path confirmation preserves the tree for reconciliation.
    """
    output = _outside_source(ROOT, output)
    parent: int = _open_directory(output.parent, private=True)
    temporary: str | None = None
    staging: int | None = None
    failure: BaseException | None = None
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
            _parent_unchanged(output.parent, parent)
            published: os.stat_result = os.stat(output.name, dir_fd=parent, follow_symlinks=False)
            if not os.path.samestat(os.fstat(staging), published):
                raise MetadataError("payload-final-slot-changed")
        except (OSError, MetadataError) as error:
            raise PublicationUncertainError() from error
        return report
    except BaseException as error:
        failure = error
        if temporary is not None and staging is not None:
            try:
                _remove_staging(parent, temporary, staging)
            except (OSError, ValueError) as cleanup:
                error.add_note(f"Private staging cleanup failed: {cleanup}")
        raise
    finally:
        release_failure: PublicationUncertainError | None = None
        descriptor: int | None
        for descriptor in (staging, parent):
            if descriptor is None:
                continue
            try:
                os.close(descriptor)
            except OSError as error:
                if failure is None:
                    release_failure = PublicationUncertainError()
                    failure = release_failure
                failure.add_note(f"Payload directory close failed: {error}")
        if release_failure is not None:
            raise release_failure


def main() -> None:
    """Build only the private source payload; the Windows packer runs separately."""
    parser: argparse.ArgumentParser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--receipt", type=Path, required=True)
    parser.add_argument("--receipt-sha256", required=True)
    parser.add_argument("--inert", type=Path, required=True)
    parser.add_argument("--inert-sha256", required=True)
    parser.add_argument("--sdk-source", type=Path,
                        help="stage the six pinned MakeAppx inputs beside payload/ and input-set.json")
    parser.add_argument("--event-resource-build", type=Path,
                        help="directory containing the linked lab DLL and its build receipt")
    parser.add_argument("--event-resource-receipt-sha256",
                        help="independently supplied SHA-256 of the event resource build receipt")
    parser.add_argument("--event-resource-generation", type=Path,
                        help="source generation used to build the event resource")
    parser.add_argument("--event-resource-source-root", type=Path,
                        help="metadata source root used for the generation")
    parser.add_argument("--event-resource-source", type=Path,
                        help="optional metadata source file within that root")
    parser.add_argument("--output", type=Path, required=True)
    args: argparse.Namespace = parser.parse_args()
    event_build: Path | None = args.event_resource_build
    event_receipt_hash: str | None = args.event_resource_receipt_sha256
    event_generation: Path | None = args.event_resource_generation
    event_root: Path | None = args.event_resource_source_root
    event_source: Path | None = args.event_resource_source
    selected: bool = any(
        item is not None for item in (event_build, event_receipt_hash, event_generation, event_root, event_source)
    )
    event_inputs: tuple[Path, str, Path, Path] | None = None
    if selected:
        if event_build is None or event_receipt_hash is None or event_generation is None or event_root is None:
            parser.error("event resource options require --event-resource-build, --event-resource-receipt-sha256, "
                         "--event-resource-generation, and --event-resource-source-root together")
        event_inputs = (event_build, event_receipt_hash, event_generation, event_root)
    try:
        receipt: Path = args.receipt.absolute()
        _outside_source(receipt.parent, args.output)
        payload: dict[str, bytes] = role_images(receipt, args.receipt_sha256)
        payload["AppxManifest.xml"] = (HERE / "AppxManifest.xml").read_bytes()
        payload["Inert.exe"] = read_bound(args.inert.absolute(), 64 * 1024, args.inert_sha256)
        for name, size in (("PackageLogo", 50), ("Square44x44Logo", 44), ("Square150x150Logo", 150)):
            payload[f"Assets/{name}.png"] = png_logo(size)
        if event_inputs is not None:
            protected: Path
            for protected in (event_inputs[0], event_inputs[2], event_inputs[3]):
                _outside_source(protected, args.output)
            payload[EVENT_RESOURCE_FILE] = event_resource_image(*event_inputs, event_source)
        sdk_source = args.sdk_source.absolute() if args.sdk_source is not None else None
        report = stage_payload(args.output, payload, sdk_source)
    except PublicationUncertainError as error:
        messages: tuple[str, ...] = (
            "Pack input rename completed. Publication durability, path or cleanup is unconfirmed.",
            *getattr(error, "__notes__", ()),
        )
        parser.exit(3, "\n".join(messages) + "\n")
    except (OSError, KeyError, TypeError, ValueError) as error:
        messages = (str(error), *getattr(error, "__notes__", ()))
        parser.exit(1, "\n".join(messages) + "\n")
    print(json.dumps(report, sort_keys=True))


if __name__ == "__main__":
    main()
