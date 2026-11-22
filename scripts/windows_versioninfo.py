"""Read selected Windows metadata from bounded llvm-readobj output.

The caller owns the executable, reader and report identities and compares the
decoded values with its independently reviewed metadata expectations.
"""
from __future__ import annotations

import re
import struct
from typing import NamedTuple, TypedDict


_MAX_REPORT_CHARS = 32 * 1024
_MAX_IMPORT_REPORT_BYTES = 64 * 1024
_MAX_RESOURCE_BYTES = 4096
_RESOURCE = re.compile(
    r"  Total Number of Resources: 1\n"
    r"  Base Table Address: 0x[0-9A-F]{1,8}\n"
    r" *\n"
    r"  Number of String Entries: 0\n"
    r"  Number of ID Entries: 1\n"
    r"  Type: VERSIONINFO \(ID 16\) \[\n"
    r"    Table Offset: 0x[0-9A-F]{1,8}\n"
    r"    Number of String Entries: 0\n"
    r"    Number of ID Entries: 1\n"
    r"    Name: \(ID 1\) \[\n"
    r"      Table Offset: 0x[0-9A-F]{1,8}\n"
    r"      Number of String Entries: 0\n"
    r"      Number of ID Entries: 1\n"
    r"      Language: \(ID 1033\) \[\n"
    r"        Entry Offset: 0x[0-9A-F]{1,8}\n"
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
    r"\]"
)
_DUMP_ROW = re.compile(r" {12}([0-9A-F]{4}): ([0-9A-F ]{2,37})\|([ -~]{1,16})\|")
_TRANSLATION = (1033, 1200)
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


def _resource_payload(report: str, subsystem: str) -> bytes:
    """Validate the image and resource selectors before reading bounded hex."""
    header, marker, resources = report.partition("\nResources [\n")
    if not marker or resources.strip() in {"]", "Total Number of Resources: 0\n]"}:
        raise ValueError("VERSIONINFO resource absent")
    if (_field(header, "Format") != "COFF-x86-64"
            or _field(header, "Arch") != "x86_64"
            or _field(header, "AddressSize") != "64bit"
            or not _field(header, "File").lower().endswith(".exe")):
        raise ValueError("readback is outside the selected AMD64 PE executable profile")
    file_header = _header(header, "ImageFileHeader")
    optional = _header(header, "ImageOptionalHeader")
    flags = re.findall(r"(?m)^  Characteristics \[ \(0x([0-9A-F]{1,4})\)$", file_header)
    if (_field(file_header, "  Machine") != "IMAGE_FILE_MACHINE_AMD64 (0x8664)"
            or len(flags) != 1 or int(flags[0], 16) & 0x2002 != 0x0002
            or _field(optional, "  Magic") != "0x20B"
            or _field(optional, "  Subsystem") != subsystem):
        raise ValueError("readback is outside the selected AMD64 PE32+ subsystem profile")
    selection = _RESOURCE.fullmatch(resources)
    if selection is None:
        raise ValueError("unexpected, duplicate or incomplete VERSIONINFO resource tree")
    size = int(selection["size"])
    table_values = [_field(optional, "    ResourceTable" + name) for name in ("RVA", "Size")]
    if any(re.fullmatch(r"0x[0-9A-F]{1,8}", value) is None for value in table_values):
        raise ValueError("invalid PE resource directory bounds")
    table_rva, table_size = (int(value, 16) for value in table_values)
    rva = int(selection["rva"], 16)
    if (size > _MAX_RESOURCE_BYTES or table_rva == 0 or rva < table_rva
            or rva + size > table_rva + table_size or table_rva + table_size > 1 << 32):
        raise ValueError("VERSIONINFO payload exceeds its admitted bounds")

    payload = bytearray()
    for row in selection["dump"].splitlines():
        match = _DUMP_ROW.fullmatch(row)
        if match is None or int(match[1], 16) != len(payload):
            raise ValueError("invalid or noncontiguous VERSIONINFO hex row")
        chunk = bytes.fromhex(match[2])
        if len(chunk) != min(16, size - len(payload)) or len(match[3]) != len(chunk):
            raise ValueError("VERSIONINFO hex row has the wrong length")
        payload.extend(chunk)
    if len(payload) != size:
        raise ValueError("truncated VERSIONINFO payload")
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
    subsystems = {"ui": "IMAGE_SUBSYSTEM_WINDOWS_GUI (0x2)", "observe": "IMAGE_SUBSYSTEM_WINDOWS_CUI (0x3)"}
    if role not in subsystems:
        raise ValueError("unsupported VERSIONINFO role")
    data = _resource_payload(report.strip("\n"), subsystems[role])
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
