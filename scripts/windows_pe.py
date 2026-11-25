"""Read static declarations from the admitted AMD64 Windows executable images.

Derived from the repository's cross-link-service-fixture.py PE reader.
Existing GPL-2.0-only repository licence applies.
"""
from __future__ import annotations

import hashlib
import os
from pathlib import Path
import re
import stat
import struct
from uuid import UUID

MAX_PE_BYTES = 64 * 1024 * 1024
# Matches the existing per-file build sandbox ceiling.
MAX_PDB_BYTES = 256 * 1024 * 1024
SERVICE_ENTRY_NAME = "nevrotick_service_entry"
SERVICE_ENTRY_OBJECT = "service-entry.obj"
_MSF_MAGIC = b"Microsoft C/C++ MSF 7.00\r\n\x1aDS\0\0\0"
_MSF_BLOCK_SIZE = 4096
_SUBSYSTEMS = {"gui": 2, "console": 3}
_RESOURCE_SUBDIRECTORY = 0x80000000
_MAX_UI_RESOURCE_BYTES = 4096
# The emitted LLVM import report is itself capped at 64 KiB.
_MAX_IMPORT_DESCRIPTORS = 4096
_MAX_IMPORT_NAME_BYTES = 127
# Match the module-name grammar of the independent LLVM text reader.
_IMPORT_MODULE = re.compile(rb"[A-Za-z0-9](?:[A-Za-z0-9_.-]{0,125}[A-Za-z0-9])?")
_ImportLayout = tuple[int, int, int, str]
_IMPORT_LAYOUTS: dict[str, _ImportLayout] = {
    "direct": (1, 20, 12, "PE direct import directory"),
    "delay": (13, 32, 4, "PE delay import directory"),
}
_Section = tuple[int, int, int, int]


def _require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def _identity(entry: os.stat_result) -> tuple[int, ...]:
    """Identify the observed filesystem object and version around the read."""
    return (entry.st_dev, entry.st_ino, entry.st_mode, entry.st_size, entry.st_mtime_ns, entry.st_ctime_ns)


def _read_artifact(path: Path, *, maximum: int, kind: str) -> bytes:
    """Capture one bounded regular artifact without following its final link."""
    try:
        opened = os.open(path, os.O_RDONLY | os.O_NOFOLLOW | os.O_NONBLOCK | os.O_CLOEXEC)
    except OSError as error:
        raise ValueError(f"{kind} must be a bounded regular emitted file.") from error
    with os.fdopen(opened, "rb") as file:
        before = os.fstat(file.fileno())
        _require(stat.S_ISREG(before.st_mode) and 0 < before.st_size <= maximum,
                 f"{kind} must be a bounded regular emitted file.")
        data = file.read(before.st_size)
        after = os.fstat(file.fileno())
        try:
            current = path.stat(follow_symlinks=False)
        except OSError as error:
            raise ValueError(f"{kind} identity changed during readback.") from error
        _require(len(data) == before.st_size and _identity(before) == _identity(after) == _identity(current),
                 f"{kind} identity changed during readback.")
    return data


def pe_readback(path: Path, *, subsystem: str, imports: bool = False) -> dict[str, str | int]:
    """Read bounded raw PE structures independently of Cargo/LLVM text output.

    Enforce AMD64/PE32+, relocations, declared CFG/ASLR/NX/high-entropy VA,
    strict CET metadata, restrictive loader flags and no writable executable
    section. The optional LLD import-table shape check binds a subsequent
    reader's descriptor inventory. These are static image declarations.
    """
    return pe_image_declarations(_read_artifact(path, maximum=MAX_PE_BYTES, kind="PE"),
                                 path, subsystem=subsystem, imports=imports)


def _pe_layout(data: bytes, subsystem: str) -> tuple[int, int, int, list[_Section]]:
    """Admit the headers and file-backed sections shared by the two readbacks."""
    _require(type(subsystem) is str and subsystem in _SUBSYSTEMS, "Unknown selected executable subsystem.")
    _require(len(data) >= 64 and data[:2] == b"MZ", "PE has no complete DOS header.")
    pe: int = struct.unpack_from("<I", data, 60)[0]
    optional: int = pe + 24
    _require(64 <= pe <= 1024 * 1024 and optional <= len(data) and data[pe:pe + 4] == b"PE\0\0",
             "Invalid PE header location.")
    _require(struct.unpack_from("<H", data, pe + 4)[0] == 0x8664, "Image is not AMD64.")
    count: int = struct.unpack_from("<H", data, pe + 6)[0]
    optional_size: int = struct.unpack_from("<H", data, pe + 20)[0]
    characteristics: int = struct.unpack_from("<H", data, pe + 22)[0]
    _require(1 <= count <= 96 and optional_size >= 240
             and optional + optional_size + count * 40 <= len(data)
             and struct.unpack_from("<H", data, optional)[0] == 0x20B,
             "Image is not a complete bounded PE32+ image.")
    _require(characteristics & 0x22 == 0x22 and characteristics & 0x2001 == 0,
              "Image lacks executable/large-address support or is a DLL/relocation-stripped image.")
    dll: int = struct.unpack_from("<H", data, optional + 70)[0]
    _require(dll & 0x4160 == 0x4160, "Required CFG/ASLR/NX/high-entropy declarations are missing.")
    _require(struct.unpack_from("<H", data, optional + 68)[0] == _SUBSYSTEMS[subsystem],
             "Selected executable subsystem changed.")
    _require(struct.unpack_from("<I", data, optional + 108)[0] >= 16, "PE directory table is incomplete.")
    sections: list[_Section] = []
    for number in range(count):
        section: int = optional + optional_size + number * 40
        virtual_size: int
        rva: int
        raw_size: int
        raw_offset: int
        virtual_size, rva, raw_size, raw_offset = struct.unpack_from("<4I", data, section + 8)
        flags: int = struct.unpack_from("<I", data, section + 36)[0]
        _require(raw_offset <= len(data) and raw_size <= len(data) - raw_offset, "Section exceeds file.")
        _require(flags & 0xA0000000 != 0xA0000000, "Image has a writable executable section.")
        sections.append((rva, virtual_size, raw_offset, raw_size))
    return optional, characteristics, dll, sections


def _file_span(sections: list[_Section], rva: int, size: int, subject: str) -> tuple[int, int]:
    """Return the mapped offset and its section's exclusive file-backed end."""
    _require(0 < rva < rva + size <= 1 << 32, f"{subject} has an invalid RVA extent.")
    overlaps: list[tuple[int, int, int]] = [
        (base, raw, length) for base, virtual, raw, length in sections
        if max(base, rva) < min(base + max(virtual, length), rva + size)
    ]
    _require(len(overlaps) == 1, f"{subject} has no unique file-backed section.")
    base: int
    raw: int
    length: int
    base, raw, length = overlaps[0]
    _require(base <= rva and rva + size <= base + length,
             f"{subject} exceeds its file-backed section.")
    return raw + rva - base, raw + length


def _file_offset(sections: list[_Section], rva: int, size: int, subject: str = "PE") -> int:
    """Map a complete RVA extent through one unambiguous file-backed section."""
    start: int
    start, _ = _file_span(sections, rva, size, subject)
    return start


def _directory(data: bytes, optional: int, number: int) -> tuple[int, int]:
    """Read one fixed PE32+ data directory from the admitted optional header."""
    rva: int
    size: int
    rva, size = struct.unpack_from("<II", data, optional + 112 + number * 8)
    return rva, size


def _import_descriptor_offsets(data: bytes, optional: int, sections: list[_Section],
                               layout: _ImportLayout, *, maximum: int | None = None) -> range:
    """Admit the selected LLD table extent, active rows, and final null row."""
    number: int
    width: int
    name_field: int
    subject: str
    number, width, name_field, subject = layout
    rva: int
    size: int
    rva, size = _directory(data, optional, number)
    if rva == size == 0:
        return range(0)
    _require(size >= 2 * width and size % width == 0, f"{subject} extent is invalid.")
    start: int = _file_offset(sections, rva, size, subject)
    last: int = start + size - width
    _require(data[last:last + width] == bytes(width), f"{subject} has no null terminator.")
    rows: range = range(start, last, width)
    _require(maximum is None or len(rows) <= maximum, f"{subject} has too many descriptors.")
    _require(all(struct.unpack_from("<I", data, row + name_field)[0] != 0 for row in rows),
             f"{subject} has an unnamed active descriptor.")
    return rows


def _import_module_name(data: bytes, sections: list[_Section], rva: int, subject: str) -> str:
    """Read one bounded ASCII module name and validate its entire RVA extent."""
    start: int
    end: int
    start, end = _file_span(sections, rva, 1, subject)
    terminator: int = data.find(b"\0", start, min(end, start + _MAX_IMPORT_NAME_BYTES + 1))
    _require(terminator >= start, f"{subject} has no bounded null terminator.")
    _file_offset(sections, rva, terminator - start + 1, subject)
    module: bytes = data[start:terminator]
    _require(_IMPORT_MODULE.fullmatch(module) is not None, f"{subject} has an invalid module name.")
    return module.decode("ascii").lower()


def static_import_modules(data: bytes, *, subsystem: str) -> dict[str, list[str]]:
    """Read sorted direct and delay module names from the held PE32+ bytes.

    Preserve one name per descriptor, including duplicates. This observes the
    selected LLD import declarations, without resolving any runtime dependency.
    """
    _require(type(data) is bytes and 0 < len(data) <= MAX_PE_BYTES,
             "Static import readback requires bounded immutable PE bytes.")
    optional: int
    sections: list[_Section]
    optional, _, _, sections = _pe_layout(data, subsystem)
    observed: dict[str, list[str]] = {}
    for kind, layout in _IMPORT_LAYOUTS.items():
        name_field: int
        subject: str
        _, _, name_field, subject = layout
        names: list[str] = []
        rows: range = _import_descriptor_offsets(data, optional, sections, layout,
                                                 maximum=_MAX_IMPORT_DESCRIPTORS)
        for row in rows:
            if kind == "delay":
                attributes: int = struct.unpack_from("<I", data, row)[0]
                _require(attributes == 1, f"{subject} has unsupported attributes.")
            name_rva: int = struct.unpack_from("<I", data, row + name_field)[0]
            names.append(_import_module_name(data, sections, name_rva, f"PE {kind} import name"))
        observed[kind] = sorted(names)
    return observed


def _check_callback_relocations(
    data: bytes, optional: int, sections: list[_Section], image_size: int,
    callback_field: int, callback_rva: int, role: str,
) -> None:
    """Preserve a null field or the original empty callback table after relocation."""
    rva: int
    size: int
    rva, size = _directory(data, optional, 5)
    _require(size >= 8 and 0 < rva <= image_size - size, f"{role} relocation directory has an invalid extent.")
    offset: int = _file_offset(sections, rva, size, f"{role} relocation directory")
    end: int = offset + size
    field_relocated: bool = False
    while offset < end:
        _require(end - offset >= 8, f"Incomplete {role.lower()} relocation block.")
        page: int
        block_size: int
        page, block_size = struct.unpack_from("<II", data, offset)
        _require(page % 4096 == 0 and page < image_size and 8 <= block_size <= end - offset
                 and block_size % 4 == 0, f"Invalid {role.lower()} relocation block.")
        for position in range(offset + 8, offset + block_size, 2):
            entry: int = struct.unpack_from("<H", data, position)[0]
            kind: int = entry >> 12
            if kind == 0:  # IMAGE_REL_BASED_ABSOLUTE padding.
                continue
            target: int = page + (entry & 0xFFF)
            _require(kind == 10 and target <= image_size - 8, f"Unsupported {role.lower()} relocation.")
            if target < callback_field + 8 and callback_field < target + 8:
                _require(callback_rva != 0 and target == callback_field and not field_relocated,
                         f"Invalid relocation of the {role.lower()} TLS callback field.")
                field_relocated = True
            _require(callback_rva == 0 or target + 8 <= callback_rva or target >= callback_rva + 8,
                     f"{role} TLS callback slot is covered by a base relocation.")
        offset += block_size
    _require(callback_rva == 0 or field_relocated, f"{role} TLS callback field lacks a base relocation.")


def _check_executable_rva(
    data: bytes, optional: int, sections: list[_Section], image_size: int, rva: int, subject: str,
) -> tuple[int, int]:
    """Find the section number and offset of an executable mapped byte."""
    _require(0 < rva < image_size, f"{subject} lies outside the image.")
    _file_offset(sections, rva, 1, subject)
    section_table: int = optional + struct.unpack_from("<H", data, optional - 4)[0]
    for number, (base, virtual_size, _, raw_size) in enumerate(sections):
        if (base <= rva < base + min(virtual_size, raw_size)
                and struct.unpack_from("<I", data, section_table + number * 40 + 36)[0] & 0x20000000):
            return number + 1, rva - base
    raise ValueError(f"{subject} has no executable file-backed section.")


def _preentry_declarations(data: bytes, *, subsystem: str, role: str) -> dict[str, object]:
    """Read actual entry bytes and loader declarations for one selected subsystem."""
    _require(type(data) is bytes and 0 < len(data) <= MAX_PE_BYTES,
             f"{role} startup readback requires bounded immutable PE bytes.")
    optional: int
    sections: list[_Section]
    optional, _, _, sections = _pe_layout(data, subsystem)
    entry_rva: int = struct.unpack_from("<I", data, optional + 16)[0]
    image_base: int = struct.unpack_from("<Q", data, optional + 24)[0]
    image_size: int = struct.unpack_from("<I", data, optional + 56)[0]
    _check_executable_rva(data, optional, sections, image_size, entry_rva, f"{role} entry")

    tls_rva: int
    tls_size: int
    tls_rva, tls_size = _directory(data, optional, 9)
    callback_rva: int = 0
    if tls_rva or tls_size:
        _require(tls_size == 40 and 0 < tls_rva <= image_size - tls_size,
                 f"{role} TLS directory has an invalid extent.")
        tls: int = _file_offset(sections, tls_rva, tls_size, f"{role} TLS directory")
        callback_va: int = struct.unpack_from("<Q", data, tls + 24)[0]
        if callback_va:
            callback_rva = callback_va - image_base
            _require(0 < callback_rva <= image_size - 8,
                     f"{role} TLS callback table lies outside the image.")
            callback: int = _file_offset(sections, callback_rva, 8, f"{role} TLS callback table")
            _require(struct.unpack_from("<Q", data, callback)[0] == 0,
                     f"{role} startup refuses pre-entry TLS callbacks.")
        _check_callback_relocations(data, optional, sections, image_size, tls_rva + 24, callback_rva, role)

    imports: dict[str, list[str]] = static_import_modules(data, subsystem=subsystem)
    _require(not imports["delay"], f"{role} startup refuses delay imports.")
    return {"entry_rva": entry_rva, "image_base": image_base,
            "tls_callback_table_rva": callback_rva, "direct_import_modules": imports["direct"]}


def service_preentry_declarations(data: bytes) -> dict[str, object]:
    """Verify the file-backed console entry and declare the pre-entry loader inputs.

    An image may retain its TLS template and index, with either no callback
    table or a table whose first pointer is null. Refuse pre-entry callbacks,
    delay imports, and malformed extents before returning any observations.
    This checks image bytes only. The caller binds the entry code and imports
    to the selected service link and qualifies any loaded runtime separately.
    """
    return _preentry_declarations(data, subsystem="console", role="Service")


def _linked_preentry_declarations(
    data: bytes, map_text: str, *, subsystem: str, role: str, gate_name: str,
    gate_origins: tuple[str, str], map_limit: int,
) -> dict[str, object]:
    """Reconcile the LLD public addresses and offsets with executable PE bytes."""
    observed: dict[str, object] = _preentry_declarations(data, subsystem=subsystem, role=role)
    _require(type(map_text) is str and 0 < len(map_text) <= map_limit,
             f"{role} startup readback requires a bounded LLD map.")
    public_headers: list[re.Match[str]] = list(re.finditer(
        r"(?m)^[ \t]+Address[ \t]+Publics by Value[ \t]+Rva\+Base[ \t]+Lib:Object[ \t]*\r?$", map_text))
    _require(len(public_headers) == 1, f"{role} LLD map has no unique public symbol table.")
    public_header: re.Match[str] = public_headers[0]
    preferred: list[str] = re.findall(
        r"(?m)^[ \t]+Preferred load address is ([0-9A-Fa-f]{16})[ \t]*\r?$", map_text[:public_header.start()])
    optional: int
    sections: list[_Section]
    optional, _, _, sections = _pe_layout(data, subsystem)
    base: int = struct.unpack_from("<Q", data, optional + 24)[0]
    _require(len(preferred) == 1 and int(preferred[0], 16) == base,
             f"{role} LLD map preferred address differs from the PE image base.")
    image_size: int = struct.unpack_from("<I", data, optional + 56)[0]
    map_entry: re.Match[str] | None = re.compile(
        r"(?m)^[ \t]+entry point at[ \t]+([0-9A-Fa-f]{4}):([0-9A-Fa-f]{8})[ \t]*\r?$"
    ).search(map_text, public_header.end())
    if map_entry is None:
        raise ValueError(f"{role} LLD map has no public symbol table terminator and entry point.")
    public_rows: str = map_text[public_header.end():map_entry.start()]

    rvas: dict[str, int] = {}
    for name in (gate_name, "mainCRTStartup"):
        matches: list[tuple[str, str, str, str]] = re.findall(
            rf"(?m)^[ \t]+([0-9A-Fa-f]{{4}}):([0-9A-Fa-f]{{8}})[ \t]+{name}"
            r"[ \t]+([0-9A-Fa-f]{16})[ \t]+([^\r\n]+)\r?$", public_rows)
        _require(len(matches) == 1, f"Linked {role} {name} public symbol is absent or ambiguous.")
        segment: str
        offset: str
        address: str
        origin: str
        segment, offset, address, origin = matches[0]
        rva: int = int(address, 16) - base
        mapped: tuple[int, int] = _check_executable_rva(data, optional, sections, image_size, rva, f"{role} {name}")
        _require((int(segment, 16), int(offset, 16)) == mapped,
                 f"Linked {role} {name} segment and preferred address disagree.")
        if name == gate_name:
            _require(origin.strip() in gate_origins, f"Linked {role} gate object differs.")
            _require((int(map_entry[1], 16), int(map_entry[2], 16)) == mapped,
                     f"{role} LLD map entry point differs from the gate symbol.")
        rvas[name] = rva
    gate: int = rvas[gate_name]
    crt: int = rvas["mainCRTStartup"]
    _require(gate == observed["entry_rva"] and crt != gate,
             f"PE entry is not the distinct {role} baseline gate with the original CRT symbol.")
    return {**observed, "crt_startup_rva": crt}


def service_link_preentry_declarations(data: bytes, map_text: str) -> dict[str, object]:
    """Bind the fixed console gate and distinct CRT public to executable image bytes."""
    return _linked_preentry_declarations(
        data, map_text, subsystem="console", role="Service", gate_name=SERVICE_ENTRY_NAME,
        gate_origins=(SERVICE_ENTRY_OBJECT, "/work/" + SERVICE_ENTRY_OBJECT), map_limit=2 * 1024**2,
    )


def ui_preentry_declarations(data: bytes, map_text: str, *, object_sha256: str) -> dict[str, object]:
    """Bind the guarded GUI entry and distinct CRT public to executable image bytes.

    Independent disassembly checks the transfer and startup instructions.
    Runtime dependencies qualify separately.
    """
    _require(type(object_sha256) is str and re.fullmatch(r"[0-9a-f]{64}", object_sha256) is not None,
             "UI startup readback requires the exact gate object digest.")
    leaf: str = f"nevrotick-ui-entry-{object_sha256}.obj"
    return _linked_preentry_declarations(
        data, map_text, subsystem="gui", role="UI", gate_name="nevrotick_ui_entry",
        gate_origins=(leaf, "/work/metadata/" + leaf), map_limit=64 * 1024**2,
    )


def ui_resource_payloads(data: bytes) -> tuple[bytes, bytes]:
    """Extract the VERSIONINFO and execution manifest from a held UI image.

    Follow the fixed Type/ID/Language tree and directory-relative links.
    https://learn.microsoft.com/windows/win32/debug/pe-format#the-rsrc-section
    """
    _require(len(data) <= MAX_PE_BYTES, "UI resource readback requires a bounded PE.")
    optional: int
    sections: list[_Section]
    optional, _, _, sections = _pe_layout(data, "gui")
    resource_rva: int
    resource_size: int
    resource_rva, resource_size = _directory(data, optional, 2)
    start: int = _file_offset(sections, resource_rva, resource_size, "UI resource directory")
    spans: list[tuple[int, int]] = []

    def span(offset: int, size: int) -> int:
        """Claim a disjoint complete structure or payload within the directory."""
        end: int = offset + size
        _require(0 <= offset < end <= resource_size, "UI resource exceeds its directory bounds.")
        _require(all(end <= left or right <= offset for left, right in spans),
                 "Overlapping UI resource directory spans.")
        spans.append((offset, end))
        return start + offset

    def directory(offset: int, count: int) -> int:
        """Require the selected number of integer entries and return their location."""
        table: int = span(offset, 16 + 8 * count)
        _require(struct.unpack_from("<HH", data, table + 12) == (0, count),
                 "Unexpected UI resource directory entry counts.")
        return table + 16

    def entry(offset: int, identifier: int, *, subdirectory: bool) -> int:
        """Read a fixed integer ID and a directory-relative link of the required kind."""
        observed: int
        link: int
        observed, link = struct.unpack_from("<II", data, offset)
        _require(observed == identifier and bool(link & _RESOURCE_SUBDIRECTORY) == subdirectory,
                 "Unexpected UI resource identifier or link kind.")
        return link & (_RESOURCE_SUBDIRECTORY - 1)

    def payload(offset: int, resource_type: int) -> tuple[int, bytes]:
        """Follow exactly one type, resource ID 1 and en-US (1033) language."""
        type_table: int = directory(entry(offset, resource_type, subdirectory=True), 1)
        name_table: int = directory(entry(type_table, 1, subdirectory=True), 1)
        leaf: int = span(entry(name_table, 1033, subdirectory=False), 16)
        address: int
        size: int
        codepage: int
        reserved: int
        address, size, codepage, reserved = struct.unpack_from("<4I", data, leaf)
        _require(0 < size <= _MAX_UI_RESOURCE_BYTES and codepage == reserved == 0,
                 "Unexpected UI resource payload size, code page or reserved field.")
        raw: int = span(address - resource_rva, size)
        return address, data[raw:raw + size]

    root: int = directory(0, 2)
    version_address: int
    version: bytes
    manifest_address: int
    manifest: bytes
    version_address, version = payload(root, 16)
    manifest_address, manifest = payload(root + 8, 24)
    _require(version_address + len(version) <= manifest_address, "Unexpected UI resource payload order.")
    return version, manifest


def pe_image_declarations(data: bytes, path: Path, *, subsystem: str,
                          imports: bool = False) -> dict[str, str | int]:
    """Read the declarations and RSDS identity from already held image bytes."""
    _require(type(data) is bytes and 0 < len(data) <= MAX_PE_BYTES,
             "PE declaration readback requires bounded immutable image bytes.")
    optional: int
    characteristics: int
    dll: int
    sections: list[_Section]
    optional, characteristics, dll, sections = _pe_layout(data, subsystem)

    def unpack(fmt: str, offset: int) -> int:
        """Decode only complete fields inside this bounded immutable image."""
        size: int = struct.calcsize(fmt)
        _require(type(offset) is int and 0 <= offset <= len(data) - size, "PE field exceeds file bounds.")
        return struct.unpack_from(fmt, data, offset)[0]

    reloc_rva: int
    reloc_size: int
    reloc_rva, reloc_size = _directory(data, optional, 5)
    _require(reloc_rva != 0 and reloc_size >= 8, "Relocation directory is missing.")
    _file_offset(sections, reloc_rva, reloc_size)
    config_rva: int
    config_size: int
    config_rva, config_size = _directory(data, optional, 10)
    _require(config_rva != 0 and config_size >= 148, "Complete CFG load configuration is missing.")
    load: int = _file_offset(sections, config_rva, config_size)
    _require(148 <= unpack("<I", load) <= config_size, "Load configuration size is inconsistent.")
    dependent = unpack("<H", load + 78)
    guard = unpack("<I", load + 144)
    _require(dependent == 0xA00, "Restrictive dependent-load declaration changed.")
    table_va, table_count = unpack("<Q", load + 128), unpack("<Q", load + 136)
    _require(guard & 0x500 == 0x500 and table_va != 0 and table_count > 0,
             "CFG instrumentation/function-table declarations are missing.")
    # The load config stores a VA. GuardFlags bits 28–31 give bytes beyond the 4-byte RVA.
    # https://learn.microsoft.com/windows/win32/secbp/pe-metadata
    table_rva = table_va - unpack("<Q", optional + 24)
    table_stride = 4 + (guard >> 28)
    _file_offset(sections, table_rva, table_count * table_stride, "CFG function table")
    debug_rva: int
    debug_size: int
    debug_rva, debug_size = _directory(data, optional, 6)
    _require(debug_size > 0 and debug_size % 28 == 0 and debug_size <= 28 * 128,
              "Debug-directory extent is missing or invalid.")
    debug: int = _file_offset(sections, debug_rva, debug_size)
    cet, codeview = [], []
    for number in range(debug_size // 28):
        entry = debug + number * 28
        kind, size = unpack("<I", entry + 12), unpack("<I", entry + 16)
        address, raw = unpack("<I", entry + 20), unpack("<I", entry + 24)
        if kind == 20:
            _require(size == 4, "Extended DLL characteristics have an unexpected shape.")
            _require(_file_offset(sections, address, size, "Extended DLL characteristics") == raw,
                     "Extended DLL characteristics address and raw pointer disagree.")
            cet.append(unpack("<I", raw))
        elif kind == 2:
            _require(25 <= size <= 4096 and _file_offset(sections, address, size, "CodeView") == raw
                     and data[raw:raw + 4] == b"RSDS", "CodeView record is incomplete.")
            name, terminator, _ = data[raw + 24:raw + size].partition(b"\0")
            _require(bool(terminator), "CodeView PDB path is not terminated.")
            codeview.append((name.decode("utf-8"), str(UUID(bytes_le=data[raw + 4:raw + 20])),
                             unpack("<I", raw + 20)))
    _require(len(cet) == 1 and cet[0] & 3 == 3 and cet[0] & 0x3C == 0,
              "Strict CET compatibility is missing or relaxed/downgraded.")
    pdb_names = {path.with_suffix(".pdb").name, path.stem.replace("-", "_") + ".pdb"}
    _require(len(codeview) == 1 and codeview[0][0].replace("\\", "/").split("/")[-1] in pdb_names,
              "Image does not identify its matching PDB basename.")
    pdb_path, guid, age = codeview[0]
    observed = {"machine": "AMD64", "format": "PE32+", "subsystem": subsystem, "image_sha256": hashlib.sha256(data).hexdigest(), "dll_characteristics": dll,
            "file_characteristics": characteristics, "sections": len(sections), "relocations_bytes": reloc_size,
            "dependent_load_flags": dependent, "guard_flags": guard, "guard_function_count": table_count,
            "extended_dll_characteristics": cet[0], "codeview_path": pdb_path,
            "codeview_guid": guid, "codeview_age": age,
            "limit": "Static image declarations only; no native mitigation or service behavior is qualified."}
    if imports:
        for kind, layout in _IMPORT_LAYOUTS.items():
            observed[f"{kind}_import_descriptors"] = len(_import_descriptor_offsets(data, optional, sections, layout))
    return observed


def _pdb_info(data: bytes, expected_pdb_page_size: int = _MSF_BLOCK_SIZE) -> dict[str, str | int]:
    """Read the GUID and age from the emitted MSF7/VC70 Info stream.

    See https://llvm.org/docs/PDB/MsfFile.html and
    https://llvm.org/docs/PDB/PdbStream.html. Validate the page routing needed
    for readback here. Debug contents and stream-specific payloads are opaque.
    """
    _require(type(expected_pdb_page_size) is int and expected_pdb_page_size in (_MSF_BLOCK_SIZE, 8192),
             "Expected PDB page size is outside the supported profiles.")
    _require(len(data) >= 56 and data[:32] == _MSF_MAGIC, "PDB lacks a complete MSF 7 superblock.")
    block_size, free_map, blocks, directory_size, _, block_map = struct.unpack_from("<6I", data, 32)
    _require(block_size == expected_pdb_page_size and free_map in (1, 2) and blocks * block_size == len(data),
             "PDB has an unsupported block size, free map or file extent.")

    def data_block(block: int) -> bool:
        """Exclude the superblock and repeating free-map pages."""
        return 0 < block < blocks and block % block_size not in (1, 2)

    pages = (directory_size + block_size - 1) // block_size
    _require(12 <= directory_size and directory_size % 4 == 0 and pages * 4 <= block_size
             and data_block(block_map), "PDB stream directory map has an invalid extent.")
    directory_blocks = struct.unpack_from(f"<{pages}I", data, block_map * block_size)
    used = {block_map, *directory_blocks}
    _require(len(used) == pages + 1 and all(data_block(block) for block in directory_blocks),
             "PDB stream directory blocks are invalid or overlap.")
    directory = memoryview(b"".join(data[block * block_size:(block + 1) * block_size]
                                    for block in directory_blocks))[:directory_size]
    stream_count = struct.unpack_from("<I", directory)[0]
    table_end = 4 + 4 * stream_count
    _require(2 <= stream_count and table_end <= directory_size, "PDB stream size table is incomplete.")
    cursor, info_block = table_end, None
    sizes = directory[4:table_end]
    for index, (length,) in enumerate(struct.iter_unpack("<I", sizes)):
        # The all-ones size marks an absent stream and consumes no pages.
        page_count = 0 if length == 0xFFFFFFFF else (length + block_size - 1) // block_size
        end = cursor + page_count * 4
        _require(end <= directory_size, "PDB stream block list is incomplete.")
        if index == 1:
            _require(28 <= length < 0xFFFFFFFF, "PDB Info stream is absent or truncated.")
            info_block = struct.unpack_from("<I", directory, cursor)[0]
        for (block,) in struct.iter_unpack("<I", directory[cursor:end]):
            _require(data_block(block) and block not in used, "PDB stream blocks are invalid or overlap.")
            used.add(block)
        cursor = end
    _require(cursor == directory_size and info_block is not None, "PDB stream directory size is inconsistent.")
    version, _, age, raw_guid = struct.unpack_from("<III16s", data, info_block * block_size)
    _require(version == 20000404, "PDB Info stream version is unsupported.")
    return {"guid": str(UUID(bytes_le=raw_guid)), "age": age, "sha256": hashlib.sha256(data).hexdigest()}


def pdb_readback(path: Path) -> dict[str, str | int]:
    """Read bounded symbol identity and hash from the same file bytes."""
    return _pdb_info(_read_artifact(path, maximum=MAX_PDB_BYTES, kind="PDB"))


def read_export_pair(image: Path, symbols: Path, *, subsystem: str,
                     expected_pdb_page_size: int = _MSF_BLOCK_SIZE):
    """Admit a matching pair and return the exact snapshots to publish."""
    image_bytes = _read_artifact(image, maximum=MAX_PE_BYTES, kind="PE")
    pe = pe_image_declarations(image_bytes, image, subsystem=subsystem)
    basename = str(pe["codeview_path"]).replace("\\", "/").split("/")[-1]
    _require(symbols.name == basename, "Windows symbol filename differs from the inspected CodeView basename.")
    pdb_bytes = _read_artifact(symbols, maximum=MAX_PDB_BYTES, kind="PDB")
    pdb = _pdb_info(pdb_bytes, expected_pdb_page_size)
    _require((pdb["guid"], pdb["age"]) == (pe["codeview_guid"], pe["codeview_age"]),
             "Windows PDB GUID or age differs from its executable CodeView record.")
    return (image_bytes, pe["image_sha256"]), (pdb_bytes, pdb["sha256"]), pdb
