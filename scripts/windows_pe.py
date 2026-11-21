"""Read static declarations from the admitted AMD64 Windows executable images.

Derived from the repository's cross-link-service-fixture.py PE reader.
Existing GPL-2.0-only repository licence applies.
"""
from __future__ import annotations

import hashlib
import os
from pathlib import Path
import stat
import struct
from uuid import UUID

MAX_PE_BYTES = 64 * 1024 * 1024
# Matches the existing per-file build sandbox ceiling.
MAX_PDB_BYTES = 256 * 1024 * 1024
_MSF_MAGIC = b"Microsoft C/C++ MSF 7.00\r\n\x1aDS\0\0\0"
_MSF_BLOCK_SIZE = 4096
_SUBSYSTEMS = {"gui": 2, "console": 3}


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


def pe_readback(path: Path, *, subsystem: str) -> dict[str, str | int]:
    """Read bounded raw PE structures independently of Cargo/LLVM text output.

    Enforce AMD64/PE32+, relocations, declared CFG/ASLR/NX/high-entropy VA,
    strict CET metadata, restrictive loader flags and no writable executable
    section. These are image declarations, never effective Windows policy.
    """
    return _pe_declarations(_read_artifact(path, maximum=MAX_PE_BYTES, kind="PE"), path, subsystem)


def _pe_declarations(data: bytes, path: Path, subsystem: str) -> dict[str, str | int]:
    """Read one immutable image and its matching RSDS identity."""
    _require(type(subsystem) is str and subsystem in _SUBSYSTEMS, "Unknown selected executable subsystem.")

    def unpack(fmt, offset):
        """Decode only complete fields inside this bounded immutable image."""
        size = struct.calcsize(fmt)
        _require(type(offset) is int and 0 <= offset <= len(data) - size, "PE field exceeds file bounds.")
        return struct.unpack_from(fmt, data, offset)[0]

    _require(len(data) >= 64 and data[:2] == b"MZ", "PE has no complete DOS header.")
    pe = unpack("<I", 60)
    _require(64 <= pe <= 1024 * 1024 and data[pe:pe + 4] == b"PE\0\0", "Invalid PE header location.")
    _require(unpack("<H", pe + 4) == 0x8664, "Image is not AMD64.")
    count, optional_size = unpack("<H", pe + 6), unpack("<H", pe + 20)
    characteristics = unpack("<H", pe + 22)
    optional = pe + 24
    _require(1 <= count <= 96 and optional_size >= 240
             and optional + optional_size + count * 40 <= len(data)
             and unpack("<H", optional) == 0x20B, "Image is not a complete bounded PE32+ image.")
    _require(characteristics & 0x22 == 0x22 and characteristics & 0x2001 == 0,
              "Image lacks executable/large-address support or is a DLL/relocation-stripped image.")
    dll = unpack("<H", optional + 70)
    _require(dll & 0x4160 == 0x4160, "Required CFG/ASLR/NX/high-entropy declarations are missing.")
    _require(unpack("<H", optional + 68) == _SUBSYSTEMS[subsystem], "Selected executable subsystem changed.")
    _require(unpack("<I", optional + 108) >= 16, "PE directory table is incomplete.")
    sections = []
    for number in range(count):
        section = optional + optional_size + number * 40
        virtual_size, rva = unpack("<I", section + 8), unpack("<I", section + 12)
        raw_size, raw_offset = unpack("<I", section + 16), unpack("<I", section + 20)
        flags = unpack("<I", section + 36)
        _require(raw_offset <= len(data) and raw_size <= len(data) - raw_offset, "Section exceeds file.")
        _require(flags & 0xA0000000 != 0xA0000000, "Image has a writable executable section.")
        sections.append((rva, virtual_size, raw_offset, raw_size))

    def file_offset(rva, size, subject="PE"):
        """Map a complete RVA extent through one unambiguous file-backed section."""
        _require(0 < rva < rva + size <= 1 << 32, f"{subject} has an invalid RVA extent.")
        overlaps = [(base, raw, length) for base, virtual, raw, length in sections
                    if max(base, rva) < min(base + max(virtual, length), rva + size)]
        _require(len(overlaps) == 1, f"{subject} has no unique file-backed section.")
        base, raw, length = overlaps[0]
        _require(base <= rva and rva + size <= base + length,
                 f"{subject} exceeds its file-backed section.")
        return raw + rva - base

    def directory(number):
        """Read one fixed PE32+ data-directory pair."""
        return unpack("<I", optional + 112 + number * 8), unpack("<I", optional + 116 + number * 8)

    reloc_rva, reloc_size = directory(5)
    _require(reloc_rva != 0 and reloc_size >= 8, "Relocation directory is missing.")
    file_offset(reloc_rva, reloc_size)
    config_rva, config_size = directory(10)
    _require(config_rva != 0 and config_size >= 148, "Complete CFG load configuration is missing.")
    load = file_offset(config_rva, config_size)
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
    file_offset(table_rva, table_count * table_stride, "CFG function table")
    debug_rva, debug_size = directory(6)
    _require(debug_size > 0 and debug_size % 28 == 0 and debug_size <= 28 * 128,
              "Debug-directory extent is missing or invalid.")
    debug = file_offset(debug_rva, debug_size)
    cet, codeview = [], []
    for number in range(debug_size // 28):
        entry = debug + number * 28
        kind, size = unpack("<I", entry + 12), unpack("<I", entry + 16)
        address, raw = unpack("<I", entry + 20), unpack("<I", entry + 24)
        if kind == 20:
            _require(size == 4, "Extended DLL characteristics have an unexpected shape.")
            _require(file_offset(address, size, "Extended DLL characteristics") == raw,
                     "Extended DLL characteristics address and raw pointer disagree.")
            cet.append(unpack("<I", raw))
        elif kind == 2:
            _require(25 <= size <= 4096 and file_offset(address, size, "CodeView") == raw
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
    return {"machine": "AMD64", "format": "PE32+", "subsystem": subsystem, "image_sha256": hashlib.sha256(data).hexdigest(), "dll_characteristics": dll,
            "file_characteristics": characteristics, "sections": count, "relocations_bytes": reloc_size,
            "dependent_load_flags": dependent, "guard_flags": guard, "guard_function_count": table_count,
            "extended_dll_characteristics": cet[0], "codeview_path": pdb_path,
            "codeview_guid": guid, "codeview_age": age,
            "limit": "Static image declarations only; no native mitigation or service behavior is qualified."}


def _pdb_info(data: bytes) -> dict[str, str | int]:
    """Read the GUID and age from the emitted MSF7/VC70 Info stream.

    See https://llvm.org/docs/PDB/MsfFile.html and
    https://llvm.org/docs/PDB/PdbStream.html. Validate the page routing needed
    for readback here. Debug contents and stream-specific payloads are opaque.
    """
    _require(len(data) >= 56 and data[:32] == _MSF_MAGIC, "PDB lacks a complete MSF 7 superblock.")
    block_size, free_map, blocks, directory_size, _, block_map = struct.unpack_from("<6I", data, 32)
    _require(block_size == _MSF_BLOCK_SIZE and free_map in (1, 2) and blocks * block_size == len(data),
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


def read_export_pair(image: Path, symbols: Path, *, subsystem: str):
    """Admit a matching pair and return the exact snapshots to publish."""
    image_bytes = _read_artifact(image, maximum=MAX_PE_BYTES, kind="PE")
    pe = _pe_declarations(image_bytes, image, subsystem)
    basename = str(pe["codeview_path"]).replace("\\", "/").split("/")[-1]
    _require(symbols.name == basename, "Windows symbol filename differs from the inspected CodeView basename.")
    pdb_bytes = _read_artifact(symbols, maximum=MAX_PDB_BYTES, kind="PDB")
    pdb = _pdb_info(pdb_bytes)
    _require((pdb["guid"], pdb["age"]) == (pe["codeview_guid"], pe["codeview_age"]),
             "Windows PDB GUID or age differs from its executable CodeView record.")
    return (image_bytes, pe["image_sha256"]), (pdb_bytes, pdb["sha256"]), pdb
