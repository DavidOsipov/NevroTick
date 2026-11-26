#!/usr/bin/env python3
# SPDX-License-Identifier: GPL-2.0-only
"""Read back the exact unsigned S0.6 fixture archive without extracting it."""

from __future__ import annotations

import argparse
from base64 import b64encode
from codecs import BOM_UTF8
from collections.abc import Callable
from dataclasses import dataclass
from hashlib import sha256
from io import BytesIO
import json
import os
from pathlib import Path
import re
import stat
import struct
from xml.etree import ElementTree as ET
import zipfile
import zlib


FOUNDATION = "{http://schemas.microsoft.com/appx/manifest/foundation/windows10}"
UAP = "{http://schemas.microsoft.com/appx/manifest/uap/windows10}"
UAP10 = "{http://schemas.microsoft.com/appx/manifest/uap/windows10/10}"
DESKTOP6 = "{http://schemas.microsoft.com/appx/manifest/desktop/windows10/6}"
RESCAP = "{http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities}"
MANIFEST_NAMESPACES: dict[str, str] = {
    "": FOUNDATION[1:-1],
    "uap": UAP[1:-1],
    "uap10": UAP10[1:-1],
    "desktop6": DESKTOP6[1:-1],
    "rescap": RESCAP[1:-1],
}
BLOCK_MAP = "{http://schemas.microsoft.com/appx/2010/blockmap}"
FILE_HASH = "{http://schemas.microsoft.com/appx/2021/blockmap}FileHash"
CONTENT_TYPES = "{http://schemas.openxmlformats.org/package/2006/content-types}"

IDENTITY = {"Name": "NevroTick.S06.ScmFixture", "Publisher": "CN=NevroTick S06 Laboratory",
            "Version": "1.0.0.0", "ProcessorArchitecture": "x64"}
SERVICES = {
    "nevrotick-lab-s02-engine.exe": "NevroTick-Lab-S02-Engine",
    "nevrotick-lab-s02-broker.exe": "NevroTick-Lab-S02-Broker",
}
DISPLAY_NAME = "NevroTick S0.6 SCM Fixture"
MANIFEST = "AppxManifest.xml"
BLOCK_MAP_FILE = "AppxBlockMap.xml"
CONTENT_TYPES_FILE = "[Content_Types].xml"
EVENT_RESOURCE: str = "nevrotick-test-events.dll"
LOGO = "Assets/PackageLogo.png"
SMALL_LOGO = "Assets/Square44x44Logo.png"
LARGE_LOGO = "Assets/Square150x150Logo.png"
ASSETS = {LOGO, SMALL_LOGO, LARGE_LOGO}
BASE_PAYLOAD: set[str] = {MANIFEST, "Inert.exe", *SERVICES, *ASSETS}
SDK_FILES = {
    "makeappx.exe",
    "appxpackaging.dll",
    "opcservices.dll",
    "Microsoft.Windows.Build.Appx.AppxPackaging.dll.manifest",
    "Microsoft.Windows.Build.Appx.OpcServices.dll.manifest",
    "en-US/AppxPackaging.dll.mui",
}
MAX_ARCHIVE = 8 * 1024 * 1024
MAX_EXPANDED = 8 * 1024 * 1024
MAX_FILE = 2 * 1024 * 1024
MAX_XML = 64 * 1024
MAX_INPUT_SET = 8 * 1024
BLOCK_BYTES = 64 * 1024
# The admitted MakeAppx stream ends with an empty final block after its
# independently decodable, byte-aligned package blocks.
DEFLATE_END = b"\x03\x00"
ZIP_END = struct.Struct("<4s4H2IH")
ZIP64_LOCATOR = struct.Struct("<4sIQI")
ZIP64_END = struct.Struct("<4sQ2H2I4Q")
ZIP64_LEGACY_FIELDS = (0xFFFF,) * 4 + (0xFFFFFFFF,) * 2
ZIP64_CENTRAL_MEMBER = struct.Struct("<HHQQQ")
ZIP32_MAX = 0xFFFFFFFF
CENTRAL_HEADER_BYTES = 46


class Refused(ValueError):
    """Input did not match the unsigned fixture contract."""


@dataclass(frozen=True)
class PackageMember:
    data: bytes
    header_size: int
    deflated: bytes | None


def require(condition: bool, reason: str) -> None:
    if not condition:
        raise Refused(reason)


def identity(info: os.stat_result) -> tuple[int, ...]:
    return info.st_dev, info.st_ino, info.st_mode, info.st_size, info.st_mtime_ns, info.st_ctime_ns


def read_regular(path: str | Path, limit: int, label: str, parent: int | None = None) -> bytes:
    flags = os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC
    with os.fdopen(os.open(path, flags, dir_fd=parent), "rb") as stream:
        before = os.fstat(stream.fileno())
        require(stat.S_ISREG(before.st_mode) and 0 < before.st_size <= limit,
                f"{label} must be a bounded regular file")
        data = stream.read(before.st_size + 1)
        after = os.stat(path, dir_fd=parent, follow_symlinks=False)
        require(len(data) == before.st_size and identity(before) == identity(after)
                == identity(os.fstat(stream.fileno())), f"{label} changed during readback")
    return data


def fixture_member_limit(name: str) -> int:
    return MAX_XML if name.endswith(".xml") else MAX_FILE


def require_directory_roster(directory: int, expected: set[str], reason: str) -> None:
    """Match the held directory, stopping before any unbounded enumeration."""
    remaining: set[str] = expected.copy()
    with os.scandir(directory) as entries:
        for entry in entries:
            require(entry.name in remaining, reason)
            remaining.remove(entry.name)
    require(not remaining, reason)


def read_payload(
    path: Path, payload_names: set[str], *, member_limit: Callable[[str], int] = fixture_member_limit,
) -> dict[str, bytes]:
    assets_names: set[str] = {name for name in payload_names if name.startswith("Assets/")}
    flags = os.O_RDONLY | os.O_DIRECTORY | os.O_NOFOLLOW | os.O_CLOEXEC
    directory = os.open(path, flags)
    try:
        before = os.fstat(directory)
        root_files: set[str] = payload_names - assets_names
        require_directory_roster(directory, root_files | {"Assets"}, "authored payload root inventory differs")
        data: dict[str, bytes] = {
            name: read_regular(name, member_limit(name), name, directory) for name in sorted(root_files)
        }
        assets = os.open("Assets", flags, dir_fd=directory)
        try:
            assets_before = os.fstat(assets)
            require_directory_roster(assets, {Path(name).name for name in assets_names},
                                     "authored asset inventory differs")
            for name in sorted(assets_names):
                data[name] = read_regular(Path(name).name, member_limit(name), name, assets)
            require(identity(assets_before) == identity(os.stat("Assets", dir_fd=directory,
                                                                 follow_symlinks=False)),
                    "authored asset directory changed during readback")
        finally:
            os.close(assets)
        require(identity(before) == identity(path.stat(follow_symlinks=False)),
                "authored payload directory changed during readback")
        return data
    finally:
        os.close(directory)


def is_sha256(value: object) -> bool:
    return (type(value) is str and len(value) == 64
            and all(char in "0123456789abcdef" for char in value))


def unique_object(pairs: list[tuple[str, object]]) -> dict[str, object]:
    fields = dict(pairs)
    require(len(fields) == len(pairs), "frozen input set contains duplicate keys")
    return fields


def read_input_set(path: Path, expected_sha256: str, payload_names: set[str]) -> dict[str, str]:
    """Bind the original packer's input inventory before using its payload digests."""
    data = read_regular(path, MAX_INPUT_SET, "frozen input set")
    require(sha256(data).hexdigest() == expected_sha256, "frozen input set SHA-256 differs")
    fields = json.loads(data.decode("utf-8"), object_pairs_hook=unique_object)
    require(type(fields) is dict and set(fields) == {"sdk", "payload"},
            "frozen input set fields differ")
    for group, names in (("sdk", SDK_FILES), ("payload", payload_names)):
        digests = fields[group]
        require(type(digests) is dict and set(digests) == names
                and all(is_sha256(digest) for digest in digests.values()),
                f"frozen input set {group} inventory or digests differ")
    return fields["payload"]


def parse_xml(data: bytes, label: str) -> ET.Element:
    """Validate the fixture's UTF-8 profile and parse the original XML bytes."""
    require(0 < len(data) <= MAX_XML, f"{label} exceeds the XML input bound")
    encoding_error = f"{label} must use UTF-8 XML encoding"
    try:
        data.decode("utf-8")
    except UnicodeDecodeError as error:
        raise Refused(encoding_error) from error
    # NUL is invalid in XML and can trigger a UTF-16/UTF-32 autodetection.
    require(b"\0" not in data, encoding_error)
    require(b"<!DOCTYPE" not in data and b"<!ENTITY" not in data,
            f"{label} contains a document type or entity declaration")

    start = data.removeprefix(BOM_UTF8)
    if re.match(rb"<\?xml[ \t\r\n]", start):
        declaration = start.partition(b"?>")[0]
        encoding = re.search(
            rb"""[ \t\r\n]encoding[ \t\r\n]*=[ \t\r\n]*(['"])([A-Za-z][A-Za-z0-9._-]*)\1""",
            declaration,
        )
        require(encoding is None or encoding[2].lower() == b"utf-8", encoding_error)
    try:
        return ET.fromstring(data)
    except (ET.ParseError, LookupError, ValueError) as error:
        raise Refused(f"invalid {label} XML") from error


def root_namespace_bindings(data: bytes) -> dict[str, str]:
    """Read the declarations on the root of an already validated XML document."""
    bindings: dict[str, str] = {}
    for event, value in ET.iterparse(BytesIO(data), events=("start-ns", "start")):
        if event == "start":
            break
        prefix, uri = value
        bindings[prefix] = uri
    return bindings


def exact_children(node: ET.Element, tags: set[str], label: str) -> None:
    require(len(node) == len(tags) and {child.tag for child in node} == tags,
            f"{label} children differ from the fixture contract")
    require_xml_spacing(node, label)


def require_xml_spacing(node: ET.Element, label: str) -> None:
    """Accept XML 1.0 spacing between elements and refuse character data."""
    spacing: str = " \t\r\n"
    require(not (node.text or "").strip(spacing) and all(not (child.tail or "").strip(spacing) for child in node),
            f"unexpected text within {label}")


def singleton(node: ET.Element, tag: str) -> ET.Element:
    child = node.find(tag)
    require(child is not None, f"missing {tag.rsplit('}', 1)[-1]}")
    return child


def exact_leaf(node: ET.Element, attrs: dict[str, str], label: str) -> None:
    require(node.attrib == attrs, f"{label} attributes differ from the fixture contract")
    exact_children(node, set(), label)


def attribute_path(value: str) -> str:
    return value.replace("\\", "/")


def check_manifest(data: bytes) -> None:
    package = parse_xml(data, MANIFEST)
    require(root_namespace_bindings(data) == MANIFEST_NAMESPACES,
            "manifest root namespace bindings differ")
    require(package.tag == FOUNDATION + "Package"
            and package.attrib == {"IgnorableNamespaces": "uap uap10 desktop6 rescap"},
            "manifest package root or attributes differ")
    exact_children(package, {FOUNDATION + name for name in (
        "Identity", "Properties", "Dependencies", "Resources", "Applications", "Capabilities",
    )}, "Package")
    exact_leaf(singleton(package, FOUNDATION + "Identity"), IDENTITY, "Identity")
    properties = singleton(package, FOUNDATION + "Properties")
    require(not properties.attrib, "unexpected Properties attributes")
    exact_children(properties, {FOUNDATION + name for name in (
        "DisplayName", "PublisherDisplayName", "Logo",
    )} | {UAP10 + "PackageIntegrity"}, "Properties")
    for name, expected in {"DisplayName": DISPLAY_NAME, "PublisherDisplayName": "NevroTick Laboratory",
                           "Logo": LOGO}.items():
        field = singleton(properties, FOUNDATION + name)
        value = field.text or ""
        if name == "Logo":
            value = attribute_path(value)
        require(not field.attrib and len(field) == 0 and value == expected,
                f"package {name} differs from the fixture contract")
    integrity = singleton(properties, UAP10 + "PackageIntegrity")
    require(not integrity.attrib, "unexpected PackageIntegrity attributes")
    exact_children(integrity, {UAP10 + "Content"}, "PackageIntegrity")
    exact_leaf(singleton(integrity, UAP10 + "Content"), {"Enforcement": "on"}, "Content")

    dependencies = singleton(package, FOUNDATION + "Dependencies")
    require(not dependencies.attrib, "unexpected Dependencies attributes")
    exact_children(dependencies, {FOUNDATION + "TargetDeviceFamily"}, "Dependencies")
    exact_leaf(singleton(dependencies, FOUNDATION + "TargetDeviceFamily"),
               {"Name": "Windows.Desktop", "MinVersion": "10.0.26100.0",
                "MaxVersionTested": "10.0.26100.0"}, "TargetDeviceFamily")
    resources = singleton(package, FOUNDATION + "Resources")
    require(not resources.attrib, "unexpected Resources attributes")
    exact_children(resources, {FOUNDATION + "Resource"}, "Resources")
    exact_leaf(singleton(resources, FOUNDATION + "Resource"), {"Language": "en-US"}, "Resource")

    applications = singleton(package, FOUNDATION + "Applications")
    require(not applications.attrib, "unexpected Applications attributes")
    exact_children(applications, {FOUNDATION + "Application"}, "Applications")
    app = singleton(applications, FOUNDATION + "Application")
    require(app.attrib == {"Id": "Inert", "Executable": "Inert.exe",
                           UAP10 + "RuntimeBehavior": "packagedClassicApp",
                           UAP10 + "TrustLevel": "mediumIL"}, "Inert application behavior differs")
    exact_children(app, {UAP + "VisualElements", FOUNDATION + "Extensions"}, "Inert application")
    visual = singleton(app, UAP + "VisualElements")
    actual_visual = dict(visual.attrib)
    for name in ("Square44x44Logo", "Square150x150Logo"):
        actual_visual[name] = attribute_path(actual_visual.get(name, ""))
    require(actual_visual == {"AppListEntry": "none", "DisplayName": DISPLAY_NAME,
                              "Description": "Disabled service configuration fixture",
                              "BackgroundColor": "transparent", "Square44x44Logo": SMALL_LOGO,
                              "Square150x150Logo": LARGE_LOGO}, "Inert visual elements differ")
    exact_children(visual, set(), "VisualElements")

    extensions = singleton(app, FOUNDATION + "Extensions")
    require(not extensions.attrib and len(extensions) == len(SERVICES),
            "expected exactly two application-scoped service extensions")
    require_xml_spacing(extensions, "Extensions")
    seen = set()
    for extension in extensions:
        executable = extension.get("Executable", "")
        require(extension.tag == DESKTOP6 + "Extension" and executable in SERVICES
                and executable not in seen and extension.attrib == {
                    "Category": "windows.service", "Executable": executable,
                    "EntryPoint": "Windows.FullTrustApplication",
                }, "service extension category, executable or activation differs")
        seen.add(executable)
        exact_children(extension, {DESKTOP6 + "Service"}, "service extension")
        exact_leaf(singleton(extension, DESKTOP6 + "Service"),
                   {"Name": SERVICES[executable], "StartupType": "disabled",
                    "StartAccount": "localService"}, f"{SERVICES[executable]} service")

    capabilities = singleton(package, FOUNDATION + "Capabilities")
    require(not capabilities.attrib and len(capabilities) == 2
            and {capability.get("Name") for capability in capabilities}
            == {"runFullTrust", "packagedServices"}, "fixture capabilities differ")
    require_xml_spacing(capabilities, "Capabilities")
    for capability in capabilities:
        require(capability.tag == RESCAP + "Capability", "unexpected capability namespace")
        exact_leaf(capability, {"Name": capability.attrib["Name"]}, "Capability")


def inflate_exact(stream: bytes, size: int, reason: str) -> bytes:
    decoder = zlib.decompressobj(-zlib.MAX_WBITS)
    try:
        expanded = decoder.decompress(stream, size + 1)
    except zlib.error as error:
        raise Refused(reason) from error
    require(len(expanded) == size and decoder.eof
            and not decoder.unused_data and not decoder.unconsumed_tail, reason)
    return expanded


def read_local_member(data: bytes, item: zipfile.ZipInfo, end: int) -> PackageMember:
    """Bind the local record to its central member, including standard descriptors."""
    name = item.orig_filename
    begin = item.header_offset
    require(0 <= begin < end <= len(data) and begin + 30 <= end
            and data[begin:begin + 4] == b"PK\x03\x04", f"{name} local header is invalid")
    extract_version, reserved, flags, method = struct.unpack_from("<BBHH", data, begin + 4)
    require((extract_version, reserved) == (item.extract_version, item.reserved),
            f"{name} local ZIP extract version differs")
    name_bytes, extra_bytes = struct.unpack_from("<HH", data, begin + 26)
    require(extra_bytes == 0, f"{name} local ZIP extra fields differ")
    header_size = 30 + name_bytes
    body_end = begin + header_size + item.compress_size
    require(flags == item.flag_bits and method == item.compress_type and body_end <= end
            and data[begin + 30:begin + 30 + name_bytes] == name.encode("ascii"),
            f"{name} local header differs from the ZIP directory")
    lengths = (item.CRC, item.compress_size, item.file_size)
    local_lengths = struct.unpack_from("<III", data, begin + 14)
    require(local_lengths == ((0, 0, 0) if flags & 8 else lengths),
            f"{name} local ZIP CRC or lengths differ")
    descriptor = data[body_end:end]
    if flags & 8:
        allowed = {struct.pack(fmt, *lengths) for fmt in ("<III", "<IQQ")}
        allowed.update(b"PK\x07\x08" + entry for entry in tuple(allowed))
        require(descriptor in allowed, f"{name} ZIP data descriptor differs")
    else:
        require(not descriptor, f"unexpected bytes after ZIP member {name}")
    body = data[begin + header_size:body_end]
    deflated = None
    if method == zipfile.ZIP_DEFLATED:
        deflated = body
        body = inflate_exact(body, item.file_size, f"{name} has an invalid DEFLATE stream")
    require(len(body) == item.file_size and zlib.crc32(body) == item.CRC,
            f"{name} expanded length or CRC differs")
    return PackageMember(body, header_size, deflated)


def check_central_member(
    data: bytes, position: int, name_size: int, extra_size: int, *, zip64_archive: bool,
) -> tuple[int, int, int]:
    """Read sizes and local offset from the admitted classic or SDK ZIP64 member profile."""
    packed_size, unpacked_size = struct.unpack_from("<II", data, position + 20)
    local_offset, = struct.unpack_from("<I", data, position + 42)
    sizes_and_offset = (unpacked_size, packed_size, local_offset)
    if ZIP32_MAX not in sizes_and_offset:
        require(extra_size == 0, "central ZIP extra fields differ")
        return sizes_and_offset

    reason = "central ZIP64 member fields differ"
    require(zip64_archive and sizes_and_offset == (ZIP32_MAX,) * 3
            and extra_size == ZIP64_CENTRAL_MEMBER.size, reason)
    extra_offset = position + CENTRAL_HEADER_BYTES + name_size
    tag, length, unpacked_size, packed_size, local_offset = ZIP64_CENTRAL_MEMBER.unpack_from(data, extra_offset)
    require(tag == 1 and length == ZIP64_CENTRAL_MEMBER.size - 4, reason)
    return unpacked_size, packed_size, local_offset


def check_zip_directory(data: bytes, member_count: int) -> tuple[int, list[tuple[int, int, int]]]:
    """Bind the complete footer and the exact member records before opening the archive."""
    end_offset = len(data) - ZIP_END.size
    reason = "unexpected ZIP comment, trailer or end record"
    require(end_offset >= 0, reason)
    signature, *fields, comment_size = ZIP_END.unpack_from(data, end_offset)
    require(signature == b"PK\x05\x06" and comment_size == 0, reason)

    directory_end = end_offset
    locator_offset = end_offset - ZIP64_LOCATOR.size
    legacy_fields = tuple(fields)
    has_locator = locator_offset >= 0 and data.startswith(b"PK\x06\x07", locator_offset)
    if has_locator or legacy_fields == ZIP64_LEGACY_FIELDS:
        # The two admitted MakeAppx archives use the fixed ZIP64 record and
        # put placeholders in every legacy field other than the comment size.
        require(legacy_fields == ZIP64_LEGACY_FIELDS, "ZIP64 legacy fields differ")
        directory_end = locator_offset - ZIP64_END.size
        require(has_locator and directory_end >= 0, "ZIP64 locator differs")
        _, disk, record_offset, disk_count = ZIP64_LOCATOR.unpack_from(data, locator_offset)
        require(disk == 0 and record_offset == directory_end and disk_count == 1,
                "ZIP64 locator differs")
        signature, size, _, _, *fields = ZIP64_END.unpack_from(data, directory_end)
        require(signature == b"PK\x06\x06" and size == ZIP64_END.size - 12,
                "ZIP64 end record differs")

    disk, directory_disk, disk_entries, total_entries, directory_size, directory_start = fields
    require(disk == directory_disk == 0, "ZIP disk fields differ")
    require(disk_entries == total_entries == member_count, "ZIP entry counts differ")
    reason = "ZIP central directory extent differs"
    require(0 < directory_start < directory_end and directory_start + directory_size == directory_end,
            reason)
    position = directory_start
    directory_members: list[tuple[int, int, int]] = []
    for _ in range(total_entries):
        require(position + CENTRAL_HEADER_BYTES <= directory_end
                and data.startswith(b"PK\x01\x02", position), reason)
        name_size, extra_size, comment_size = struct.unpack_from("<3H", data, position + 28)
        record_end = position + CENTRAL_HEADER_BYTES + name_size + extra_size + comment_size
        require(record_end <= directory_end, reason)
        require(comment_size == 0, "central ZIP member comment is not admitted")
        directory_members.append(check_central_member(
            data, position, name_size, extra_size, zip64_archive=has_locator,
        ))
        position = record_end
    require(position == directory_end, reason)
    return directory_start, directory_members


def read_archive(
    data: bytes,
    archive_names: set[str],
    *,
    maximum_archive_bytes: int = MAX_ARCHIVE,
    maximum_expanded_bytes: int = MAX_EXPANDED,
    member_limit: Callable[[str], int] = fixture_member_limit,
) -> dict[str, PackageMember]:
    require(len(data) <= maximum_archive_bytes, "package exceeds the input bound")
    directory_start, directory_members = check_zip_directory(data, len(archive_names))
    try:
        package = zipfile.ZipFile(BytesIO(data))
    except NotImplementedError as error:
        raise Refused("unsupported ZIP extract version") from error
    with package:
        members = package.infolist()
        require(len(members) == len(archive_names)
                and {item.orig_filename for item in members} == archive_names,
                "package inventory differs or contains duplicate, unsafe or signed members")
        require(sum(item.file_size for item in members) <= maximum_expanded_bytes,
                "expanded package exceeds the input bound")
        require(package.start_dir == directory_start, "ZIP central directory extent differs")
        require(all((item.file_size, item.compress_size, item.header_offset) == row
                    for item, row in zip(members, directory_members, strict=True)),
                "ZIP central member size or offset differs")
        require(all(item.volume == 0 for item in members), "ZIP disk fields differ")
        physical_members = sorted(members, key=lambda item: item.header_offset)
        require(physical_members[0].header_offset == 0, "unexpected bytes before ZIP payload")
        file_ends = [item.header_offset for item in physical_members[1:]] + [directory_start]
        members_by_name = {}
        for item, end in zip(physical_members, file_ends, strict=True):
            name = item.orig_filename
            require(0 < item.file_size <= member_limit(name)
                    and 0 < item.compress_size <= maximum_archive_bytes,
                    f"{name} exceeds the package member bound")
            mode = stat.S_IFMT(item.external_attr >> 16)
            require(mode in (0, stat.S_IFREG) and not item.external_attr & 0x10
                    and not item.flag_bits & ~0x080E
                    and item.compress_type in (zipfile.ZIP_STORED, zipfile.ZIP_DEFLATED),
                    f"{name} has an unsupported ZIP member type")
            members_by_name[name] = read_local_member(data, item, end)
        return members_by_name


def check_block_map(members: dict[str, PackageMember], payload_names: set[str]) -> None:
    data = members[BLOCK_MAP_FILE].data
    block_map = parse_xml(data, BLOCK_MAP_FILE)
    hash_method = "http://www.w3.org/2001/04/xmlenc#sha256"
    require(block_map.tag == BLOCK_MAP + "BlockMap" and block_map.get("HashMethod") == hash_method
            and set(block_map.attrib) <= {"HashMethod", "IgnorableNamespaces"},
            "expected a SHA-256 AppxBlockMap")
    require(len(block_map) == len(payload_names), "block-map file count differs")
    require_xml_spacing(block_map, "BlockMap")
    seen = set()
    for record in block_map:
        name = attribute_path(record.get("Name", ""))
        require(record.tag == BLOCK_MAP + "File" and name in payload_names and name not in seen,
                "block-map file inventory differs or contains aliases")
        seen.add(name)
        member = members[name]
        payload_bytes = member.data
        require(set(record.attrib) == {"Name", "Size", "LfhSize"}
                and record.get("Size") == str(len(payload_bytes))
                and record.get("LfhSize") == str(member.header_size),
                f"block-map size or local header size differs for {name}")
        require_xml_spacing(record, "block-map File")
        offsets = range(0, len(payload_bytes), BLOCK_BYTES)
        blocks = list(record)
        if blocks and blocks[-1].tag == FILE_HASH:
            file_hash = blocks.pop()
            digest = b64encode(sha256(payload_bytes).digest()).decode("ascii")
            exact_leaf(file_hash, {"Hash": digest}, f"block-map file hash for {name}")
        require(len(blocks) == len(offsets), f"block-map block count differs for {name}")
        compressed_offset = 0
        compressed = member.deflated
        for block, offset in zip(blocks, offsets, strict=True):
            expected = payload_bytes[offset:offset + BLOCK_BYTES]
            digest = b64encode(sha256(expected).digest()).decode("ascii")
            require(block.tag == BLOCK_MAP + "Block" and len(block) == 0
                    and {"Hash"} <= set(block.attrib) <= {"Hash", "Size"}
                    and block.get("Hash") == digest, f"block-map hash differs for {name}")
            require_xml_spacing(block, "block-map Block")
            size = block.get("Size")
            if compressed is None:
                require(size is None, f"{name} has a Size on a stored block")
                continue
            if size is None:
                raise Refused(f"{name} requires a compressed block Size")
            require(size.isascii() and size.isdecimal() and len(size) <= 7,
                    f"invalid block-map compressed size for {name}")
            compressed_end = compressed_offset + int(size)
            reason = f"{name} compressed block bytes do not match at expanded offset {offset}"
            require(compressed_offset < compressed_end <= len(compressed) - len(DEFLATE_END), reason)
            block_stream = compressed[compressed_offset:compressed_end] + DEFLATE_END
            require(inflate_exact(block_stream, len(expected), reason) == expected, reason)
            compressed_offset = compressed_end
        if compressed is not None:
            require(compressed[compressed_offset:] == DEFLATE_END,
                    f"{name} compressed block map does not leave the expected DEFLATE terminator")


def check_content_types(data: bytes, archive_names: set[str]) -> None:
    types = parse_xml(data, CONTENT_TYPES_FILE)
    require(types.tag == CONTENT_TYPES + "Types" and not types.attrib, "content-types root differs")
    require_xml_spacing(types, "content-types document")
    defaults = {}
    overrides = {}
    part_names: set[str] = archive_names - {CONTENT_TYPES_FILE}
    extensions: set[str] = {name.rsplit(".", 1)[1] for name in part_names}
    for declaration in types:
        require(len(declaration) == 0, "content-type declaration contains unexpected content")
        require_xml_spacing(declaration, "content-type declaration")
        value = declaration.get("ContentType", "")
        require(value and "/" in value and value.isascii() and not any(c.isspace() for c in value),
                "content-type declaration has an invalid media type")
        if declaration.tag == CONTENT_TYPES + "Default":
            kind = declaration.get("Extension", "")
            require(set(declaration.attrib) == {"Extension", "ContentType"}
                    and kind in extensions and kind not in defaults, "content-type default differs")
            defaults[kind] = value
        else:
            part = declaration.get("PartName", "")
            require(declaration.tag == CONTENT_TYPES + "Override"
                    and set(declaration.attrib) == {"PartName", "ContentType"}
                    and part.startswith("/") and part[1:] in part_names
                    and part[1:] not in overrides, "content-type override differs")
            overrides[part[1:]] = value

    permitted = {MANIFEST: {"application/vnd.ms-appx.manifest+xml"},
                 BLOCK_MAP_FILE: {"application/vnd.ms-appx.blockmap+xml"}}
    for name in ASSETS:
        permitted[name] = {"image/png", "application/octet-stream"}
    binary_names: set[str] = {"Inert.exe", *SERVICES}
    if EVENT_RESOURCE in archive_names:
        binary_names.add(EVENT_RESOURCE)
    for name in binary_names:
        permitted[name] = {"application/x-msdownload", "application/octet-stream"}
    for name, allowed in permitted.items():
        kind = overrides.get(name, defaults.get(name.rsplit(".", 1)[1]))
        require(kind in allowed, f"content type differs or is missing for {name}")


def main() -> int:
    cli = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("--payload", required=True, type=Path)
    cli.add_argument("--package", required=True, type=Path)
    cli.add_argument("--input-set", required=True, type=Path)
    cli.add_argument("--input-set-sha256", required=True, help="digest from the admitted pack inputs")
    cli.add_argument("--package-sha256", required=True, help="digest from the packer's retained output record")
    cli.add_argument("--include-event-resource", action="store_true",
                     help=f"require the laboratory payload with {EVENT_RESOURCE}")
    args = cli.parse_args()
    # The caller selects a fixed profile before any package inputs are read.
    payload_names: set[str] = BASE_PAYLOAD | {EVENT_RESOURCE} if args.include_event_resource else BASE_PAYLOAD
    archive_names: set[str] = payload_names | {BLOCK_MAP_FILE, CONTENT_TYPES_FILE}
    try:
        require(is_sha256(args.input_set_sha256) and is_sha256(args.package_sha256),
                "expected lowercase SHA-256 digests for the frozen input set and package")
        expected: dict[str, str] = read_input_set(args.input_set, args.input_set_sha256, payload_names)
        payload: dict[str, bytes] = read_payload(args.payload, payload_names)
        package = read_regular(args.package, MAX_ARCHIVE, "package")
        package_sha256 = sha256(package).hexdigest()
        require(package_sha256 == args.package_sha256, "package SHA-256 differs from the pack output record")
        members: dict[str, PackageMember] = read_archive(package, archive_names)
        for name in sorted(payload_names):
            require(sha256(payload[name]).hexdigest() == expected[name],
                    f"authored payload member differs from the frozen input set: {name}")
            require(members[name].data == payload[name], f"emitted package member differs: {name}")
        check_manifest(members[MANIFEST].data)
        check_block_map(members, payload_names)
        check_content_types(members[CONTENT_TYPES_FILE].data, archive_names)
    except Refused as error:
        cli.exit(1, f"Fixture package refused: {error}\n")
    except (OSError, zipfile.BadZipFile, zlib.error, EOFError, ValueError, RecursionError) as error:
        cli.exit(1, f"Fixture package refused: unreadable input ({type(error).__name__})\n")
    # The selected roster uses canonical ASCII names. Each frozen digest now
    # matches the original authored bytes and the corresponding archive member.
    payload_catalog: bytes = "".join(
        f"{name}\t{len(members[name].data)}\t{expected[name]}\n" for name in sorted(payload_names)
    ).encode("ascii")
    print(json.dumps({"status": "unsigned-fixture-package-checked", "package_identity": IDENTITY,
                      "input_set_sha256": args.input_set_sha256,
                      "package_bytes": len(package), "package_sha256": package_sha256,
                      "payload_sha256": sha256(payload_catalog).hexdigest(),
                      "members": {name: {"bytes": len(member.data), "sha256": sha256(member.data).hexdigest()}
                                  for name, member in sorted(members.items())}}, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
