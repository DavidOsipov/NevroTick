"""Read the selected event fixture's Windows RC and unsigned rust-lld outputs."""

from __future__ import annotations

import hashlib
import struct

from .pipeline.contract import MetadataError

_MAX_ARTIFACT = 1024 * 1024
_MAX_LEAF = 64 * 1024
_MESSAGE_TABLE = 11
_EVENT_TEMPLATE = "WEVT_TEMPLATE"
_NAME = 1
_EN_US = 0x0409
_DIRECTORY = 0x80000000
_FILE_ALIGNMENT = 512
_SECTION_ALIGNMENT = 4096
_RESOURCE_RVA = _SECTION_ALIGNMENT
_PE_OFFSET = 120
_OPTIONAL_SIZE = 240
# The selected rust-lld emits a fixed DOS compatibility stub. Pin it so its
# code and reserved bytes cannot carry an additional payload.
_DOS_STUB_SHA256 = "bbec96f83a7ebfe16114934dab38b796ae296bfcc8ff6c8bc97511f872dec86d"

_ResourcePath = tuple[int | str, int, int]


def _require(condition: bool, detail: str) -> None:
    if not condition:
        raise MetadataError(f"Event resource: {detail}")


def _artifact(data: bytes) -> None:
    _require(type(data) is bytes and 0 < len(data) <= _MAX_ARTIFACT,
             "expected bounded immutable artifact bytes")


def _region(data: bytes, offset: int, size: int) -> bytes:
    _require(0 <= offset <= offset + size <= len(data), "structure exceeds artifact bounds")
    return data[offset:offset + size]


def _u32(data: bytes, offset: int) -> int:
    return int.from_bytes(_region(data, offset, 4), "little")


def _align(size: int, alignment: int) -> int:
    return (size + alignment - 1) // alignment * alignment


def read_resource_object(data: bytes) -> dict[_ResourcePath, bytes]:
    """Read the two selected MS RC resources before passing the .res to the linker."""
    _artifact(data)
    # Resource files start with a null record. This signature also excludes
    # ordinary COFF objects, archives and response files from this input lane.
    marker = struct.pack("<IIHHHHIIII", 0, 32, 0xFFFF, 0, 0xFFFF, 0, 0, 0, 0, 0)
    _require(data.startswith(marker), "missing MS RC null resource record")
    offset = len(marker)
    result: dict[_ResourcePath, bytes] = {}
    for resource_type, encoded_type in (
        (_MESSAGE_TABLE, struct.pack("<HH", 0xFFFF, _MESSAGE_TABLE)),
        (_EVENT_TEMPLATE, (_EVENT_TEMPLATE + "\0").encode("utf-16-le")),
    ):
        size = _u32(data, offset)
        header_size = _u32(data, offset + 4)
        # Both selected type encodings and the ordinal name end at a DWORD.
        expected_fields = encoded_type + struct.pack("<HHIHHII", 0xFFFF, _NAME, 0, 0x30, _EN_US, 0, 0)
        _require(header_size == 8 + len(expected_fields)
                 and _region(data, offset + 8, len(expected_fields)) == expected_fields,
                 "unexpected MS RC resource header, type, name or language")
        _require(0 < size <= _MAX_LEAF, "invalid MS RC resource size")
        start = offset + header_size
        result[(resource_type, _NAME, _EN_US)] = _region(data, start, size)
        padding = _align(size, 4) - size
        _require(not any(_region(data, start + size, padding)), "nonzero MS RC resource padding")
        offset = start + size + padding
    _require(offset == len(data), "trailing MS RC resource or payload")
    return result


def _pe_resource_section(data: bytes) -> bytes:
    """Admit the unsigned single-section image and return its exact virtual resource span."""
    _artifact(data)
    _require(hashlib.sha256(_region(data, 0, _PE_OFFSET)).hexdigest() == _DOS_STUB_SHA256,
             "unexpected rust-lld DOS header or stub")
    # Deterministic timestamp, AMD64, one section, no COFF symbols, PE32+
    # optional header and executable-image/large-address/DLL file flags.
    coff = b"PE\0\0" + struct.pack("<HHIIIHH", 0x8664, 1, 0, 0, 0, _OPTIONAL_SIZE, 0x2022)
    _require(_region(data, _PE_OFFSET, len(coff)) == coff,
             "expected the selected AMD64 resource-only DLL file header")
    optional = _PE_OFFSET + len(coff)
    directories = optional + 112
    resource_size = _u32(data, directories + 20)
    _require(0 < resource_size <= _MAX_ARTIFACT - _FILE_ALIGNMENT,
             "missing or oversized PE resource directory")
    raw_size = _align(resource_size, _FILE_ALIGNMENT)
    # The selected linker emits initialized data only, with no entry point or
    # code, then its fixed GUI-subsystem DLL defaults and 16 directories.
    standard = struct.pack("<HBBIIIIIQ", 0x20B, 14, 0, 0, raw_size, 0, 0, 0, 0x180000000)
    loader = struct.pack("<II6HIIIIHH4QII", _SECTION_ALIGNMENT, _FILE_ALIGNMENT,
                         6, 0, 0, 0, 6, 0, 0, _RESOURCE_RVA + _align(resource_size, _SECTION_ALIGNMENT),
                         _FILE_ALIGNMENT, 0, 2, 0x160, 0x100000, 0x1000, 0x100000, 0x1000, 0, 16)
    _require(_region(data, optional, directories - optional) == standard + loader,
             "unexpected PE32+ defaults, code or entry point")
    expected_directories = bytes(16) + struct.pack("<II", _RESOURCE_RVA, resource_size) + bytes(13 * 8)
    _require(_region(data, directories, len(expected_directories)) == expected_directories,
             "unexpected PE data directory, import, export or certificate")
    section_offset = optional + _OPTIONAL_SIZE
    section = struct.pack("<8sIIIIIIHHI", b".rsrc", resource_size, _RESOURCE_RVA, raw_size,
                          _FILE_ALIGNMENT, 0, 0, 0, 0, 0x40000040)
    _require(_region(data, section_offset, len(section)) == section,
             "expected one read-only initialized .rsrc section")
    header_end = section_offset + len(section)
    _require(len(data) == _FILE_ALIGNMENT + raw_size, "unexpected PE file extent or overlay")
    _require(not any(data[header_end:_FILE_ALIGNMENT]) and not any(data[_FILE_ALIGNMENT + resource_size:]),
             "nonzero PE section or header padding")
    return _region(data, _FILE_ALIGNMENT, resource_size)


def read_resource_dll(data: bytes) -> dict[_ResourcePath, bytes]:
    """Read only the selected, unambiguous three-level resource paths from an inert DLL."""
    section = _pe_resource_section(data)
    extents: list[tuple[int, int]] = []

    def claim(offset: int, size: int, alignment: int) -> bytes:
        _require(offset % alignment == 0, "misaligned PE resource structure")
        value = _region(section, offset, size)
        extents.append((offset, offset + size))
        return value

    def table(offset: int, named: int, ordinal: int) -> bytes:
        records = claim(offset, 16 + (named + ordinal) * 8, 4)
        _require(records[:16] == struct.pack("<IIIHH", 0, 0, 0, named, ordinal),
                 "unexpected PE resource table flags or entries")
        return records[16:]

    def subdirectory(pointer: int) -> bytes:
        _require(bool(pointer & _DIRECTORY), "PE resource path ends before its language")
        return table(pointer ^ _DIRECTORY, 0, 1)

    root = table(0, 1, 1)
    named_type = _u32(root, 0)
    _require(bool(named_type & _DIRECTORY) and _u32(root, 8) == _MESSAGE_TABLE,
             "unexpected PE event resource types")
    name_bytes = _EVENT_TEMPLATE.encode("utf-16-le")
    name = claim(named_type ^ _DIRECTORY, 2 + len(name_bytes), 2)
    _require(name == struct.pack("<H", len(_EVENT_TEMPLATE)) + name_bytes,
             "unexpected PE event template type name")

    result: dict[_ResourcePath, bytes] = {}
    for resource_type, pointer in ((_EVENT_TEMPLATE, _u32(root, 4)), (_MESSAGE_TABLE, _u32(root, 12))):
        names = subdirectory(pointer)
        _require(_u32(names, 0) == _NAME, "unexpected PE event resource name")
        languages = subdirectory(_u32(names, 4))
        _require(_u32(languages, 0) == _EN_US, "unexpected PE event resource language")
        descriptor = claim(_u32(languages, 4), 16, 4)
        size = _u32(descriptor, 4)
        _require(0 < size <= _MAX_LEAF and descriptor[8:] == bytes(8),
                 "unexpected PE resource data size, code page or reserved field")
        offset = _u32(descriptor, 0) - _RESOURCE_RVA
        result[(resource_type, _NAME, _EN_US)] = claim(offset, size, 8)

    end = 0
    for start, stop in sorted(extents):
        _require(end <= start < end + 8 and not any(section[end:start]),
                 "overlapping PE resource structures or unexpected intervening payload")
        end = stop
    _require(_align(end, 8) == len(section) and not any(section[end:]),
             "trailing PE resource payload")
    return result
