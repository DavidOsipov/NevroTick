"""Read selected Windows metadata from bounded resource bytes or llvm-readobj output.

The caller owns the executable, reader and report identities and compares the
decoded values with its independently reviewed metadata expectations.
"""
from __future__ import annotations

import hashlib
import re
import struct
from typing import NamedTuple, TypedDict
from xml.etree import ElementTree as ET


_MAX_REPORT_CHARS = 32 * 1024
_MAX_UI_REPORT_CHARS = 64 * 1024
_MAX_IMPORT_REPORT_BYTES = 64 * 1024
_MAX_RESOURCE_BYTES = 4096
_RESOURCE_HEADER = re.compile(
    r"  Total Number of Resources: (?P<count>[12])\n"
    r"  Base Table Address: 0x[0-9A-F]{1,8}\n"
    r" *\n"
    r"  Number of String Entries: 0\n"
    r"  Number of ID Entries: (?P=count)\n"
)
_RESOURCE_LEAF = re.compile(
    r"  Type: (?P<type>VERSIONINFO \(ID 16\)|MANIFEST \(ID 24\)) \[\n"
    r"    Table Offset: 0x(?P<type_offset>[0-9A-F]{1,8})\n"
    r"    Number of String Entries: 0\n"
    r"    Number of ID Entries: 1\n"
    r"    Name: \(ID 1\) \[\n"
    r"      Table Offset: 0x(?P<name_offset>[0-9A-F]{1,8})\n"
    r"      Number of String Entries: 0\n"
    r"      Number of ID Entries: 1\n"
    r"      Language: \(ID 1033\) \[\n"
    r"        Entry Offset: 0x(?P<data_entry_offset>[0-9A-F]{1,8})\n"
    r"        Time/Date Stamp: [^\n]{1,80}\n"
    r"        Major Version: [0-9]{1,5}\n"
    r"        Minor Version: [0-9]{1,5}\n"
    r"        Characteristics: [0-9]{1,10}\n"
    r"        Data \[\n"
    r"          DataRVA: 0x(?P<rva>[0-9A-F]{1,8})\n"
    r"          DataSize: (?P<size>[1-9][0-9]{0,3})\n"
    r"          Codepage: 0\n"
    r"          Reserved: 0\n"
    r"          Data \(\n"
    r"(?P<dump>(?: {12}[^\n]{1,64}\n){1,256})"
    r"          \)\n"
    r"        \]\n"
    r"      \]\n"
    r"    \]\n"
    r"  \]\n"
)
_DUMP_ROW = re.compile(r" {12}([0-9A-F]{4}): ([0-9A-F ]{2,37})\|([ -~]{1,16})\|")
_TRANSLATION = (1033, 1200)
_GUI = "IMAGE_SUBSYSTEM_WINDOWS_GUI (0x2)"
_SUBSYSTEMS = {"ui": _GUI, "observe": "IMAGE_SUBSYSTEM_WINDOWS_CUI (0x3)"}
_MANIFEST_DECLARATION = b'<?xml version="1.0" standalone="yes"?>'
_ASSEMBLY = "{urn:schemas-microsoft-com:asm.v1}"
_TRUST = "{urn:schemas-microsoft-com:asm.v3}"
_IMPORT_MODULE = r"(?P<name>[A-Za-z0-9](?:[A-Za-z0-9_.-]{0,125}[A-Za-z0-9])?)"
_IMPORT_RVA = r"0x[0-9A-F]{1,8}"
_IMPORT_SYMBOL = r"[ -~]{0,512} \([0-9]{1,5}\)"
_DELAY_THUNK = r"\n  Import \{\n    Symbol: " + _IMPORT_SYMBOL + r"\n    Address: 0x[0-9A-F]{1,16}\n  \}"
_DIRECT_IMPORT = re.compile(
    r"Import \{\n  Name: " + _IMPORT_MODULE + r"\n"
    r"  ImportLookupTableRVA: " + _IMPORT_RVA + r"\n"
    r"  ImportAddressTableRVA: " + _IMPORT_RVA
    + r"(?:\n  Symbol: " + _IMPORT_SYMBOL + r")+"
)
_DELAY_IMPORT = re.compile(
    r"DelayImport \{\n  Name: " + _IMPORT_MODULE + r"\n  Attributes: 0x1"
    + "".join(r"\n  " + field + r": " + _IMPORT_RVA for field in (
        "ModuleHandle", "ImportAddressTable", "ImportNameTable",
        "BoundDelayImportTable", "UnloadDelayImportTable",
    ))
    + r"(?:" + _DELAY_THUNK + r")+"
)


class VersionInfo(TypedDict):
    """Decoded VERSIONINFO facts from the selected UI resource."""

    fixed: tuple[int, ...]
    strings: dict[str, str]
    translation: tuple[int, int]


class UiExecutionManifest(TypedDict):
    """Execution settings and digest of the selected UI manifest resource."""

    level: str
    ui_access: bool
    sha256: str


class _Block(NamedTuple):
    kind: int
    value: bytes
    children: dict[str, _Block]


def _field(report: str, label: str) -> str:
    """Read one field at the selected indentation and reject duplicates."""
    prefix = label + ": "
    values = [line[len(prefix):] for line in report.split("\n") if line.startswith(prefix)]
    if len(values) != 1:
        raise ValueError("missing or duplicate PE header field")
    return values[0]


def _header(report: str, name: str) -> str:
    """Select the uniquely named llvm-readobj header block."""
    blocks = re.findall(r"(?m)^" + name + r" \{\n((?: [^\n]*\n)+)\}$", report)
    if len(blocks) != 1:
        raise ValueError("missing or duplicate PE header block")
    return blocks[0]


def decode_imports(report: str, *, image: str) -> dict[str, list[str]]:
    """Decode bounded direct and delay import module names for one linked image.

    Receivers bind the pinned readobj call to the immutable PE and compare both
    counts against the observed binary table shapes. Names are inert observations.
    """
    if not report.isascii() or len(report) > _MAX_IMPORT_REPORT_BYTES:
        raise ValueError("invalid or oversized llvm-readobj import report")
    # The maintained runner strips the surrounding line breaks from stdout.
    report = report.strip("\n") + "\n"
    prefix = f"File: {image}\nFormat: COFF-x86-64\nArch: x86_64\nAddressSize: 64bit\n"
    if not report.startswith(prefix):
        raise ValueError("PE import report identifies a different image")
    *records, tail = report[len(prefix):].split("\n}\n")
    if tail:
        raise ValueError("incomplete PE import report")
    direct: list[str] = []
    delay: list[str] = []
    for record in records:
        ordinary = _DIRECT_IMPORT.fullmatch(record)
        if ordinary is not None:
            direct.append(ordinary["name"].lower())
            continue
        delayed = _DELAY_IMPORT.fullmatch(record)
        if delayed is None:
            raise ValueError("unsupported PE import descriptor output")
        delay.append(delayed["name"].lower())
    return {"direct": sorted(direct), "delay": sorted(delay)}


def _resource_payloads(report: str, subsystem: str, *, ui_manifest: bool = False) -> tuple[bytes, ...]:
    """Read only the selected role's resource tree and disjoint payloads."""
    header, marker, resources = report.strip("\n").partition("\nResources [\n")
    if not marker or resources.strip() in {"]", "Total Number of Resources: 0\n]"}:
        raise ValueError("VERSIONINFO resource absent")
    image = _field(header, "File")
    if (_field(header, "Format") != "COFF-x86-64"
            or _field(header, "Arch") != "x86_64"
            or _field(header, "AddressSize") != "64bit"
            or not image.lower().endswith(".exe")):
        raise ValueError("readback is outside the selected AMD64 PE executable profile")
    if ui_manifest and image.replace("\\", "/").rsplit("/", 1)[-1] != "nevrotick-ui.exe":
        raise ValueError("resource report identifies a different UI executable")
    file_header = _header(header, "ImageFileHeader")
    optional = _header(header, "ImageOptionalHeader")
    flags = re.findall(r"(?m)^  Characteristics \[ \(0x([0-9A-F]{1,4})\)$", file_header)
    if (_field(file_header, "  Machine") != "IMAGE_FILE_MACHINE_AMD64 (0x8664)"
            or len(flags) != 1 or int(flags[0], 16) & 0x2002 != 0x0002
            or _field(optional, "  Magic") != "0x20B"
            or _field(optional, "  Subsystem") != subsystem):
        raise ValueError("readback is outside the selected AMD64 PE32+ subsystem profile")
    types = ("VERSIONINFO (ID 16)", "MANIFEST (ID 24)") if ui_manifest else ("VERSIONINFO (ID 16)",)
    tree = _RESOURCE_HEADER.match(resources)
    tree_error = "unexpected, duplicate or incomplete PE resource tree"
    if tree is None or int(tree["count"]) != len(types):
        raise ValueError(tree_error)
    table_values = [_field(optional, "    ResourceTable" + name) for name in ("RVA", "Size")]
    if any(re.fullmatch(r"0x[0-9A-F]{1,8}", value) is None for value in table_values):
        raise ValueError("invalid PE resource directory bounds")
    table_rva, table_size = (int(value, 16) for value in table_values)
    table_end = table_rva + table_size
    if table_rva == 0 or table_end > 1 << 32:
        raise ValueError("invalid PE resource directory bounds")
    cursor = tree.end()
    previous_end = table_rva
    payloads = []
    # The selected dual-resource root has a 16-byte header and two 8-byte entries.
    ui_spans = [(0, 32)]
    for resource_type in types:
        selection = _RESOURCE_LEAF.match(resources, cursor)
        if selection is None or selection["type"] != resource_type:
            raise ValueError(tree_error)
        start = int(selection["rva"], 16)
        end = start + int(selection["size"])
        if start < previous_end or end > table_end:
            raise ValueError("PE resource payload overlaps or exceeds its admitted bounds")
        if ui_manifest:
            # Type and name tables hold one directory entry each. The leaf
            # itself holds a 16-byte data entry, separate from its payload.
            for field, width in (("type_offset", 24), ("name_offset", 24), ("data_entry_offset", 16)):
                offset = int(selection[field], 16)
                ui_spans.append((offset, offset + width))
            ui_spans.append((start - table_rva, end - table_rva))
        payloads.append(_dump_payload(selection))
        previous_end = end
        cursor = selection.end()
    if resources[cursor:] != "]":
        raise ValueError(tree_error)
    if ui_manifest:
        previous_offset_end = 0
        for offset, offset_end in sorted(ui_spans):
            if offset < previous_offset_end or offset_end > table_size:
                raise ValueError("overlapping or out-of-bounds UI resource directory spans")
            previous_offset_end = offset_end
    return tuple(payloads)


def _dump_payload(selection: re.Match[str]) -> bytes:
    """Decode a bounded selected payload from contiguous hexadecimal rows."""
    size = int(selection["size"])
    if size > _MAX_RESOURCE_BYTES:
        raise ValueError("PE resource payload exceeds its admitted bounds")
    payload = bytearray()
    for row in selection["dump"].splitlines():
        match = _DUMP_ROW.fullmatch(row)
        if match is None or int(match[1], 16) != len(payload):
            raise ValueError("invalid or noncontiguous PE resource hex row")
        chunk = bytes.fromhex(match[2])
        if len(chunk) != min(16, size - len(payload)) or len(match[3]) != len(chunk):
            raise ValueError("PE resource hex row has the wrong length")
        payload.extend(chunk)
    if len(payload) != size:
        raise ValueError("truncated PE resource payload")
    return bytes(payload)


def _align(offset: int) -> int:
    """Align the next block or value to a DWORD boundary."""
    return (offset + 3) & ~3


def _block(data: bytes, start: int, limit: int, depth: int = 0) -> tuple[str, _Block, int]:
    """Read one version block within its parent and the selected nesting depth."""
    if depth > 3 or start + 8 > limit:
        raise ValueError("VERSIONINFO block exceeds its bounds")
    length, units, kind = struct.unpack_from("<HHH", data, start)
    end = start + length
    if not start + 8 <= end <= limit or kind not in (0, 1):
        raise ValueError("invalid VERSIONINFO block header")
    key_start = start + 6
    terminator = next((i for i in range(key_start, end - 1, 2) if data[i:i + 2] == b"\0\0"), None)
    if terminator is None or terminator == key_start:
        raise ValueError("invalid VERSIONINFO block name")
    key = data[key_start:terminator].decode("utf-16-le")
    value_start = _align(terminator + 2)
    value_end = value_start + units * (2 if kind == 1 else 1)
    cursor = _align(value_end)
    if (value_end > end or any(data[terminator + 2:value_start])
            or any(data[value_end:min(cursor, end)])):
        raise ValueError("invalid VERSIONINFO value bounds or padding")
    children: dict[str, _Block] = {}
    while cursor < end:
        child_key, child, child_end = _block(data, cursor, end, depth + 1)
        if child_key in children:
            raise ValueError("duplicate VERSIONINFO block name")
        children[child_key] = child
        cursor = _align(child_end)
        if any(data[child_end:min(cursor, end)]):
            raise ValueError("invalid VERSIONINFO child padding")
    return key, _Block(kind, data[value_start:value_end], children), end


def decode(report: str, *, role: str = "ui") -> VersionInfo:
    """Decode the sole en-US VERSIONINFO leaf from the selected role's PE report.

    Accept the pinned reader's selected single-resource grammar. Raise
    ValueError for an absent, ambiguous, malformed, oversized or wrong-profile
    report. The returned fields preserve the decoded values for caller checks.
    """
    if not isinstance(report, str) or len(report) > _MAX_REPORT_CHARS:
        raise ValueError("invalid or oversized llvm-readobj report")
    if role not in _SUBSYSTEMS:
        raise ValueError("unsupported VERSIONINFO role")
    data, = _resource_payloads(report, _SUBSYSTEMS[role])
    return _decode_versioninfo(data)


def decode_ui_resources(report: str) -> tuple[VersionInfo, UiExecutionManifest]:
    """Require the UI's version and execution manifest leaves on one AMD64 PE.

    Raise ValueError for an invalid resource tree or unapproved execution
    settings. The caller binds the report to the actual linked image.
    """
    if not isinstance(report, str) or len(report) > _MAX_UI_REPORT_CHARS:
        raise ValueError("invalid or oversized llvm-readobj UI resource report")
    version_data, manifest_data = _resource_payloads(report, _GUI, ui_manifest=True)
    return decode_ui_resource_payloads(version_data, manifest_data)


def decode_ui_resource_payloads(version_data: bytes, manifest_data: bytes) -> tuple[VersionInfo, UiExecutionManifest]:
    """Decode the two bounded payloads selected by the calling resource reader."""
    if not (0 < len(version_data) <= _MAX_RESOURCE_BYTES and 0 < len(manifest_data) <= _MAX_RESOURCE_BYTES):
        raise ValueError("invalid or oversized UI resource payload")
    return _decode_versioninfo(version_data), _decode_ui_execution_manifest(manifest_data)


def _decode_ui_execution_manifest(data: bytes) -> UiExecutionManifest:
    """Parse the selected linker's minimal manifest from the observed bytes."""
    declaration, newline, body = data.partition(b"\n")
    if (not data.isascii() or b"\0" in data or not newline
            or declaration.rstrip(b"\r") != _MANIFEST_DECLARATION
            or any(mark in body for mark in (b"<!", b"<?", b"&"))):
        raise ValueError("invalid UI execution manifest encoding or XML declaration")
    try:
        element = ET.fromstring(data)
    except ET.ParseError as error:
        raise ValueError("malformed UI execution manifest XML") from error
    manifest = (
        (_ASSEMBLY + "assembly", {"manifestVersion": "1.0"}),
        (_TRUST + "trustInfo", {}),
        (_TRUST + "security", {}),
        (_TRUST + "requestedPrivileges", {}),
        (_TRUST + "requestedExecutionLevel", {"level": "asInvoker", "uiAccess": "false"}),
    )
    for index, (tag, attributes) in enumerate(manifest):
        children = int(index < len(manifest) - 1)
        if (element.tag != tag or element.attrib != attributes or len(element) != children
                or (element.text or "").strip() or (element.tail or "").strip()):
            raise ValueError("wrong UI execution manifest structure or settings")
        if children:
            element = element[0]
    return {"level": element.attrib["level"], "ui_access": element.attrib["uiAccess"] == "true",
            "sha256": hashlib.sha256(data).hexdigest()}


def _decode_versioninfo(data: bytes) -> VersionInfo:
    """Decode the version bytes independently of the containing resource tree."""
    name, root, end = _block(data, 0, len(data))
    if (name != "VS_VERSION_INFO" or root.kind != 0 or end != len(data)
            or len(root.value) != 52 or set(root.children) != {"StringFileInfo", "VarFileInfo"}):
        raise ValueError("invalid VERSIONINFO root")
    fixed = struct.unpack("<13I", root.value)
    if fixed[:2] != (0xFEEF04BD, 0x00010000):
        raise ValueError("invalid VERSIONINFO fixed-value identity")
    translation = _Block(0, struct.pack("<HH", *_TRANSLATION), {})
    if root.children["VarFileInfo"] != _Block(1, b"", {"Translation": translation}):
        raise ValueError("wrong VERSIONINFO translation")

    strings = root.children["StringFileInfo"]
    if strings.kind != 1 or strings.value or set(strings.children) != {"040904B0"}:
        raise ValueError("wrong VERSIONINFO string table identity")
    table = strings.children["040904B0"]
    if table.kind != 1 or table.value or not table.children:
        raise ValueError("invalid VERSIONINFO string table")
    decoded: dict[str, str] = {}
    for field, entry in table.children.items():
        if entry.kind != 1 or entry.children:
            raise ValueError("invalid VERSIONINFO string block")
        value = entry.value.decode("utf-16-le")
        if not value.endswith("\0") or "\0" in value[:-1]:
            raise ValueError("invalid VERSIONINFO string terminator")
        decoded[field] = value[:-1]
    return {"fixed": fixed, "strings": decoded, "translation": _TRANSLATION}
