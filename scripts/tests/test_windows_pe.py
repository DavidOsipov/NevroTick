"""Focused, inert Windows image readback controls."""
from __future__ import annotations

import hashlib
import importlib.util
import os
from pathlib import Path
import struct
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location("windows_pe", Path(__file__).resolve().parents[1] / "windows_pe.py")
PE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(PE)

# Literal positions in the synthetic image. Directory pointers below are RVAs,
# and the CFG table pointer in the load configuration is a virtual address.
_OPTIONAL = 0x98
_SECTION = 0x188
_LOAD = 0x240
_IMAGE_BASE = 0x140000000
_UI_GUID = bytes.fromhex("78563412bc9af0de123456789abcdef0")
_OBSERVE_GUID = bytes.fromhex("21436587a9cb0fed21436587a9cbed0f")
_MSF_BLOCK_SIZE = 4096


def _image(subsystem: int, *, pdb_path: str = "test_pilot.pdb",
           guid: bytes = _UI_GUID, age: int = 1) -> bytearray:
    image = bytearray(0x600)

    def put(fmt: str, at: int, *values: int) -> None:
        struct.pack_into("<" + fmt, image, at, *values)

    image[:2] = b"MZ"
    put("I", 60, 0x80)
    image[0x80:0x84] = b"PE\0\0"
    put("HH", 0x84, 0x8664, 1)
    put("HH", 0x94, 240, 0x22)
    put("H", _OPTIONAL, 0x20B)
    put("Q", _OPTIONAL + 24, _IMAGE_BASE)
    put("I", _OPTIONAL + 56, 0x2000)  # SizeOfImage
    put("HH", _OPTIONAL + 68, subsystem, 0x4160)
    put("I", _OPTIONAL + 108, 16)
    put("II", _OPTIONAL + 112 + 5 * 8, 0x1020, 12)
    put("II", _OPTIONAL + 112 + 6 * 8, 0x1100, 56)
    put("II", _OPTIONAL + 112 + 10 * 8, 0x1040, 148)
    put("IIII", _SECTION + 8, 0x400, 0x1000, 0x400, 0x200)
    put("I", _SECTION + 36, 0x60000020)
    put("IIHH", 0x220, 0x1000, 12, 0xA010, 0)  # One DIR64 relocation and ABSOLUTE padding.
    put("I", _LOAD, 148)
    put("H", _LOAD + 78, 0xA00)
    put("QQI", _LOAD + 128, _IMAGE_BASE + 0x13F6, 2, 0x10000500)
    put("IBIB", 0x5F6, 0x1010, 0, 0x1020, 0)  # Two 5-byte CFG records ending at the file boundary.
    put("IIII", 0x30C, 20, 4, 0x1140, 0x340)
    put("I", 0x340, 3)
    pdb = b"RSDS" + guid + struct.pack("<I", age) + pdb_path.encode("ascii") + b"\0"
    put("IIII", 0x328, 2, len(pdb), 0x1150, 0x350)
    image[0x350:0x350 + len(pdb)] = pdb
    return image


def _pdb(*, guid: bytes = _UI_GUID, age: int = 1) -> bytearray:
    """Encode an inert MSF 7 fixture with a VC70 PDB info stream."""
    block = _MSF_BLOCK_SIZE
    data = bytearray(6 * block)
    data[:32] = b"Microsoft C/C++ MSF 7.00\r\n\x1aDS\0\0\0"
    # Blocks 1 and 2 are free-page maps, 3 lists directory block 4,
    # and directory stream 1 uses info block 5. Stream 0 is absent.
    struct.pack_into("<6I", data, 32, block, 1, 6, 16, 0, 3)
    for page_map in (1, 2):
        data[page_map * block:(page_map + 1) * block] = b"\xc0" + b"\xff" * (block - 1)
    struct.pack_into("<I", data, 3 * block, 4)
    struct.pack_into("<4I", data, 4 * block, 2, 0xFFFFFFFF, 48, 5)
    struct.pack_into("<III16s", data, 5 * block, 20000404, 0x1000, age, guid)
    # Empty string table, empty hash table and empty present/deleted bitsets.
    struct.pack_into("<5I", data, 5 * block + 28, 0, 0, 1, 0, 0)
    return data


class WindowsPeTests(unittest.TestCase):
    def setUp(self) -> None:
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.path = Path(directory.name) / "test-pilot.exe"

    def readback(self, image: bytearray, subsystem: str = "gui") -> dict:
        self.path.write_bytes(image)
        return PE.pe_readback(self.path, subsystem=subsystem)

    def test_matching_subsystems_bind_exact_payload(self) -> None:
        for number, role in ((2, "gui"), (3, "console")):
            with self.subTest(role=role):
                image = _image(number)
                readback = self.readback(image, role)
                self.assertEqual(readback["subsystem"], role)
                self.assertEqual(readback["image_sha256"], hashlib.sha256(image).hexdigest())
                self.assertEqual(readback["guard_function_count"], 2)
                self.assertEqual(readback["codeview_path"], "test_pilot.pdb")
                self.assertEqual(readback["codeview_guid"], "12345678-9abc-def0-1234-56789abcdef0")
                self.assertEqual(readback["codeview_age"], 1)

    def test_import_counts_require_real_terminators_and_file_backed_tables(self) -> None:
        empty = _image(2)
        self.path.write_bytes(empty)
        observed = PE.pe_readback(self.path, subsystem="gui", imports=True)
        self.assertEqual(observed["direct_import_descriptors"], 0)
        self.assertEqual(observed["delay_import_descriptors"], 0)

        image = _image(2)
        direct_directory, delay_directory = _OPTIONAL + 120, _OPTIONAL + 216
        struct.pack_into("<II", image, direct_directory, 0x1200, 40)
        struct.pack_into("<II", image, delay_directory, 0x1240, 64)
        struct.pack_into("<I", image, 0x400 + 12, 0x1300)
        struct.pack_into("<II", image, 0x440, 1, 0x1320)
        self.path.write_bytes(image)
        observed = PE.pe_readback(self.path, subsystem="gui", imports=True)
        self.assertEqual(observed["direct_import_descriptors"], 1)
        self.assertEqual(observed["delay_import_descriptors"], 1)
        controls = (
            (0x400 + 20, 1, "direct import directory has no null"),
            (0x440 + 32, 1, "delay import directory has no null"),
            (0x400 + 12, 0, "direct import directory has an unnamed"),
            (0x440 + 4, 0, "delay import directory has an unnamed"),
            (delay_directory + 4, 63, "delay import directory extent"),
            (delay_directory, 0x13F0, "delay import directory exceeds"),
        )
        for offset, word, reason in controls:
            with self.subTest(offset=offset), self.assertRaisesRegex(ValueError, reason):
                wrong = bytearray(image)
                struct.pack_into("<I", wrong, offset, word)
                self.path.write_bytes(wrong)
                PE.pe_readback(self.path, subsystem="gui", imports=True)

    def test_wrong_subsystem_and_unselected_role(self) -> None:
        with self.assertRaisesRegex(ValueError, "subsystem"):
            self.readback(_image(3))
        with self.assertRaisesRegex(ValueError, "subsystem"):
            self.readback(_image(2), "native")

    def test_static_declarations_and_structural_bounds(self) -> None:
        controls = (
            ("H", 0x84, 0x14C, "AMD64"),
            ("H", _OPTIONAL, 0x10B, "PE32\\+"),
            ("H", 0x96, 0x23, "relocation-stripped"),
            ("H", _OPTIONAL + 70, 0x160, "CFG/ASLR/NX"),
            ("H", _OPTIONAL + 70, 0x4120, "CFG/ASLR/NX"),
            ("H", _OPTIONAL + 70, 0x4140, "CFG/ASLR/NX"),
            ("I", _OPTIONAL + 112 + 5 * 8, 0, "Relocation directory"),
            ("H", _LOAD + 78, 0, "dependent-load"),
            ("I", _SECTION + 36, 0xE0000020, "writable executable"),
            ("I", _LOAD + 144, 0x10000400, "CFG instrumentation"),
            ("I", 0x340, 1, "Strict CET"),
            ("I", 0x340, 7, "Strict CET"),
            ("I", 0x358 - 8, 0, "CodeView"),
            ("I", _SECTION + 16, 0x401, "Section exceeds file"),
        )
        for fmt, address, value, message in controls:
            with self.subTest(address=address, value=value):
                image = _image(2)
                struct.pack_into("<" + fmt, image, address, value)
                with self.assertRaisesRegex(ValueError, message):
                    self.readback(image)

    def test_cfg_honors_image_base_and_bounded_stride(self) -> None:
        controls = (
            ("Q", _LOAD + 128, _IMAGE_BASE - 1),
            ("Q", _LOAD + 128, _IMAGE_BASE + 0x100000000),
            ("Q", _LOAD + 136, 2**64 - 1),
            ("I", _LOAD + 144, 0x20000500),  # The sixth byte crosses the section boundary.
        )
        for fmt, address, value in controls:
            with self.subTest(address=address, value=value):
                image = _image(2)
                struct.pack_into("<" + fmt, image, address, value)
                with self.assertRaisesRegex(ValueError, "CFG function table"):
                    self.readback(image)

    def test_debug_directory_address_and_raw_pointer_agree(self) -> None:
        for address, message in ((0x314, "Extended DLL characteristics"), (0x330, "CodeView")):
            with self.subTest(address=address):
                image = _image(2)
                # Point the RVA at a different file-backed range. Leave the original raw bytes intact.
                struct.pack_into("<I", image, address, 0x1180)
                with self.assertRaisesRegex(ValueError, message):
                    self.readback(image)

    def test_truncated_images_are_refused(self) -> None:
        for cut in (0, 63, 0x84, _SECTION, 0x5FF):
            with self.subTest(cut=cut), self.assertRaises(ValueError):
                self.readback(_image(2)[:cut])

    def test_nonregular_and_oversized_inputs_are_refused(self) -> None:
        self.path.symlink_to(self.path.parent / "absent")
        with self.assertRaisesRegex(ValueError, "bounded regular"):
            PE.pe_readback(self.path, subsystem="gui")
        self.path.unlink()
        os.mkfifo(self.path)
        with self.assertRaisesRegex(ValueError, "bounded regular"):
            PE.pe_readback(self.path, subsystem="gui")
        self.path.unlink()
        with self.path.open("wb") as image:
            image.truncate(PE.MAX_PE_BYTES + 1)
        with self.assertRaisesRegex(ValueError, "bounded regular"):
            PE.pe_readback(self.path, subsystem="gui")


class WindowsPdbTests(unittest.TestCase):
    def setUp(self) -> None:
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.path = Path(directory.name) / "test-pilot.pdb"

    def readback(self, data: bytearray) -> dict:
        self.path.write_bytes(data)
        return PE.pdb_readback(self.path)

    def test_reads_vc70_guid_age_and_exact_bytes(self) -> None:
        data = _pdb(guid=_OBSERVE_GUID, age=7)
        result = self.readback(data)
        self.assertEqual(result["guid"], "87654321-cba9-ed0f-2143-6587a9cbed0f")
        self.assertEqual(result["age"], 7)
        self.assertEqual(result["sha256"], hashlib.sha256(data).hexdigest())

    def test_directory_and_stream_reject_outside_or_reserved_blocks(self) -> None:
        block = _MSF_BLOCK_SIZE
        for subject, offset, value, message in (
            ("directory map outside the file", 52, 6, "directory map has an invalid extent"),
            ("directory map aliases reserved free-page map", 52, 1, "directory map has an invalid extent"),
            ("directory page aliases its block map", 3 * block, 3, "directory blocks are invalid or overlap"),
            ("information stream aliases its directory", 4 * block + 12, 4, "stream blocks are invalid or overlap"),
            ("truncated information header", 4 * block + 8, 27, "Info stream is absent or truncated"),
        ):
            with self.subTest(subject=subject):
                data = _pdb()
                struct.pack_into("<I", data, offset, value)
                with self.assertRaisesRegex(ValueError, message):
                    self.readback(data)
        self.assertEqual(self.readback(_pdb())["guid"], "12345678-9abc-def0-1234-56789abcdef0")

    def test_pdb_rejects_link_fifo_and_oversized_file_before_parsing(self) -> None:
        self.path.symlink_to(self.path.parent / "absent")
        with self.assertRaisesRegex(ValueError, "PDB must be a bounded regular emitted file"):
            PE.pdb_readback(self.path)
        self.path.unlink()
        os.mkfifo(self.path)
        with self.assertRaisesRegex(ValueError, "PDB must be a bounded regular emitted file"):
            PE.pdb_readback(self.path)
        self.path.unlink()
        data = _pdb()
        self.path.write_bytes(data)
        # The configured test sandbox's file-size cap equals the production bound.
        with patch.object(PE, "MAX_PDB_BYTES", len(data) - 1), \
                self.assertRaisesRegex(ValueError, "PDB must be a bounded regular emitted file"):
            PE.pdb_readback(self.path)
        self.assertEqual(PE.pdb_readback(self.path)["sha256"], hashlib.sha256(data).hexdigest())
