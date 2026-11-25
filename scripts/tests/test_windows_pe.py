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


def _ui_resource_image(version: bytes, manifest: bytes) -> bytearray:
    """Write literal Type/ID/Language tables in a second, file-backed section."""
    image: bytearray = _image(2, pdb_path=r"C:\build\nevrotick_ui.pdb")
    raw: int = 0x600
    rva: int = 0x2000
    version_offset: int = 0xA0
    manifest_offset: int = (version_offset + len(version) + 3) & ~3
    resources_size: int = manifest_offset + len(manifest)
    raw_size: int = (resources_size + 0x1FF) & ~0x1FF
    image.extend(bytes(raw_size))
    struct.pack_into("<H", image, 0x86, 2)
    struct.pack_into("<II", image, _OPTIONAL + 32, 0x1000, 0x200)
    struct.pack_into("<II", image, _OPTIONAL + 56, 0x3000, 0x200)
    struct.pack_into("<II", image, _OPTIONAL + 112 + 2 * 8, rva, resources_size)
    image[_SECTION:_SECTION + 8] = b".text\0\0\0"
    second: int = _SECTION + 40
    image[second:second + 8] = b".rsrc\0\0\0"
    struct.pack_into("<IIII", image, second + 8, resources_size, rva, raw_size, raw)
    struct.pack_into("<I", image, second + 36, 0x40000040)
    struct.pack_into("<HHIIII", image, raw + 12, 0, 2, 16, 0x80000020, 24, 0x80000038)
    struct.pack_into("<HHII", image, raw + 0x20 + 12, 0, 1, 1, 0x80000050)
    struct.pack_into("<HHII", image, raw + 0x38 + 12, 0, 1, 1, 0x80000068)
    struct.pack_into("<HHII", image, raw + 0x50 + 12, 0, 1, 1033, 0x80)
    struct.pack_into("<HHII", image, raw + 0x68 + 12, 0, 1, 1033, 0x90)
    struct.pack_into("<4I", image, raw + 0x80, rva + version_offset, len(version), 0, 0)
    struct.pack_into("<4I", image, raw + 0x90, rva + manifest_offset, len(manifest), 0, 0)
    image[raw + version_offset:raw + version_offset + len(version)] = version
    image[raw + manifest_offset:raw + manifest_offset + len(manifest)] = manifest
    return image


def with_import_modules(image: bytearray, *, direct: tuple[str, ...] = (),
                        delay: tuple[str, ...] = ()) -> bytearray:
    """Put literal import-name rows in the shared fixtures' unused file span."""
    result: bytearray = bytearray(image)
    names: int = 0x4E0
    for table, directory, width, name_field, modules in (
        (0x400, _OPTIONAL + 120, 20, 12, direct),
        (0x460, _OPTIONAL + 216, 32, 4, delay),
    ):
        if not modules:
            continue
        assert len(modules) <= 3
        struct.pack_into("<II", result, directory, table + 0xE00, (len(modules) + 1) * width)
        for index, module in enumerate(modules):
            encoded: bytes = module.encode("ascii") + b"\0"
            end: int = names + len(encoded)
            assert end <= 0x5F6
            row: int = table + index * width
            if table == 0x460:
                struct.pack_into("<I", result, row, 1)
            struct.pack_into("<I", result, row + name_field, names + 0xE00)
            result[names:end] = encoded
            names = end
    return result


def _pdb(*, guid: bytes = _UI_GUID, age: int = 1, block: int = _MSF_BLOCK_SIZE) -> bytearray:
    """Encode an inert MSF 7 fixture with a VC70 PDB info stream."""
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

    def test_ui_resources_require_the_fixed_file_backed_tree(self) -> None:
        version: bytes = b"version data"
        manifest: bytes = b"manifest data"
        image: bytearray = _ui_resource_image(version, manifest)
        self.assertEqual(PE.ui_resource_payloads(bytes(image)), (version, manifest))

        resource_table: int = _OPTIONAL + 112 + 2 * 8
        controls: tuple[tuple[str, int, int, str], ...] = (
            ("I", resource_table, 0, "invalid RVA extent"),
            ("I", resource_table + 4, 0x201, "exceeds its file-backed section"),
            ("I", _SECTION + 12, 0x2000, "no unique file-backed section"),
            ("H", 0x600 + 12, 1, "entry counts"),
            ("I", 0x600 + 0x18, 16, "identifier or link kind"),
            ("I", 0x600 + 0x1C, 0x38, "identifier or link kind"),
            ("I", 0x600 + 0x1C, 0x800001FF, "exceeds its directory bounds"),
            ("I", 0x600 + 0x20 + 16, 2, "identifier or link kind"),
            ("I", 0x600 + 0x68 + 16, 1041, "identifier or link kind"),
            ("I", 0x600 + 0x68 + 20, 0x80, "Overlapping UI resource"),
            ("I", 0x600 + 0x90, 0x2090, "Overlapping UI resource"),
            ("I", 0x600 + 0x90, 0x1FFF, "exceeds its directory bounds"),
            ("I", 0x600 + 0x90 + 4, 4097, "payload size"),
            ("I", 0x600 + 0x90 + 8, 1200, "code page"),
            ("I", 0x600 + 0x90 + 12, 1, "reserved field"),
        )
        for layout, offset, value, reason in controls:
            with self.subTest(offset=offset, value=value), self.assertRaisesRegex(ValueError, reason):
                wrong: bytearray = bytearray(image)
                struct.pack_into("<" + layout, wrong, offset, value)
                PE.ui_resource_payloads(bytes(wrong))
        self.assertEqual(PE.ui_resource_payloads(bytes(image)), (version, manifest))

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

    def test_import_names_are_sorted_and_preserve_one_entry_per_descriptor(self) -> None:
        empty: bytearray = _image(3)
        self.assertEqual(PE.static_import_modules(bytes(empty), subsystem="console"),
                         {"direct": [], "delay": []})
        longest: str = "A" * 123 + ".DLL"
        image: bytearray = with_import_modules(
            empty, direct=("Z.dll", "KERNEL32.dll", "kernel32.DLL"), delay=(longest, "X"))
        self.assertEqual(PE.static_import_modules(bytes(image), subsystem="console"),
                         {"direct": ["kernel32.dll", "kernel32.dll", "z.dll"],
                          "delay": [longest.lower(), "x"]})

    def test_service_preentry_accepts_absent_and_empty_tls_callbacks(self) -> None:
        image: bytearray = with_import_modules(_image(3), direct=("KERNEL32.dll",))
        struct.pack_into("<I", image, _OPTIONAL + 16, 0x1010)
        expected: dict[str, object] = {"entry_rva": 0x1010, "image_base": _IMAGE_BASE,
                                       "tls_callback_table_rva": 0, "direct_import_modules": ["kernel32.dll"]}
        self.assertEqual(PE.service_preentry_declarations(bytes(image)), expected)
        struct.pack_into("<II", image, _OPTIONAL + 112 + 9 * 8, 0x12A0, 40)
        # The loader may retain a nonempty TLS template and an index word.
        struct.pack_into("<QQQQII", image, 0x4A0, _IMAGE_BASE + 0x1300, _IMAGE_BASE + 0x1308,
                         _IMAGE_BASE + 0x1308, 0, 0, 0)
        self.assertEqual(PE.service_preentry_declarations(bytes(image)), expected)
        struct.pack_into("<Q", image, 0x4B8, _IMAGE_BASE + 0x12C8)
        struct.pack_into("<H", image, 0x22A, 0xA2B8)
        self.assertEqual(PE.service_preentry_declarations(bytes(image)),
                         {**expected, "tls_callback_table_rva": 0x12C8})

    def test_service_preentry_refuses_callbacks_and_unbacked_callback_words(self) -> None:
        image: bytearray = _image(3)
        struct.pack_into("<I", image, _OPTIONAL + 16, 0x1010)
        struct.pack_into("<II", image, _OPTIONAL + 112 + 9 * 8, 0x12A0, 40)
        struct.pack_into("<Q", image, 0x4B8, _IMAGE_BASE + 0x12C8)
        struct.pack_into("<H", image, 0x22A, 0xA2B8)
        self.assertEqual(PE.service_preentry_declarations(bytes(image))["tls_callback_table_rva"], 0x12C8)
        for offset, pointer, reason in (
            (0x4C8, _IMAGE_BASE + 0x1010, "refuses pre-entry TLS callbacks"),
            (0x4B8, _IMAGE_BASE - 8, "TLS callback table lies outside the image"),
            (0x4B8, _IMAGE_BASE + 0x2000, "TLS callback table lies outside the image"),
            (0x4B8, _IMAGE_BASE + 0x13FC, "TLS callback table exceeds its file-backed section"),
        ):
            with self.subTest(offset=offset, pointer=pointer), self.assertRaisesRegex(ValueError, reason):
                wrong: bytearray = bytearray(image)
                struct.pack_into("<Q", wrong, offset, pointer)
                PE.service_preentry_declarations(bytes(wrong))
        struct.pack_into("<I", image, _OPTIONAL + 112 + 9 * 8 + 4, 39)
        with self.assertRaisesRegex(ValueError, "TLS directory has an invalid extent"):
            PE.service_preentry_declarations(bytes(image))

    def test_service_preentry_requires_an_unrelocated_empty_callback_slot(self) -> None:
        image: bytearray = _image(3)
        struct.pack_into("<I", image, _OPTIONAL + 16, 0x1010)
        struct.pack_into("<II", image, _OPTIONAL + 112 + 9 * 8, 0x12A0, 40)
        struct.pack_into("<Q", image, 0x4B8, _IMAGE_BASE + 0x12C8)
        struct.pack_into("<H", image, 0x22A, 0xA2B8)
        controls: tuple[tuple[int, str], ...] = (
            (0xA2C0, ""),  # The preceding pointer ends at the start of the callback slot.
            (0xA2D0, ""),  # The following pointer is outside the slot.
            (0xA2C7, "TLS callback slot is covered"),
            (0xA2C8, "TLS callback slot is covered"),
            (0xA2CF, "TLS callback slot is covered"),
            (0x32C8, "Unsupported service relocation"),
        )
        for relocation, reason in controls:
            with self.subTest(relocation=relocation):
                struct.pack_into("<H", image, 0x228, relocation)
                if reason:
                    with self.assertRaisesRegex(ValueError, reason):
                        PE.service_preentry_declarations(bytes(image))
                else:
                    self.assertEqual(PE.service_preentry_declarations(bytes(image))["tls_callback_table_rva"],
                                     0x12C8)
        struct.pack_into("<I", image, 0x224, 16)
        with self.assertRaisesRegex(ValueError, "Invalid service relocation block"):
            PE.service_preentry_declarations(bytes(image))

    def test_service_preentry_requires_the_original_callback_field_after_relocation(self) -> None:
        image: bytearray = _image(3)
        struct.pack_into("<I", image, _OPTIONAL + 16, 0x1010)
        struct.pack_into("<II", image, _OPTIONAL + 112 + 9 * 8, 0x12A0, 40)
        controls: tuple[tuple[int, tuple[int, int], str], ...] = (
            (0, (0xA010, 0), ""),
            (0, (0xA2B8, 0), "Invalid relocation of the service TLS callback field"),
            (0x12C8, (0xA010, 0xA2B8), ""),
            (0x12C8, (0xA010, 0), "callback field lacks a base relocation"),
            (0x12C8, (0xA2B8, 0xA2B8), "Invalid relocation of the service TLS callback field"),
            (0x12C8, (0xA2B7, 0xA2B8), "Invalid relocation of the service TLS callback field"),
        )
        for callback, entries, reason in controls:
            with self.subTest(callback=callback, entries=entries):
                struct.pack_into("<Q", image, 0x4B8, _IMAGE_BASE + callback if callback else 0)
                struct.pack_into("<HH", image, 0x228, *entries)
                if reason:
                    with self.assertRaisesRegex(ValueError, reason):
                        PE.service_preentry_declarations(bytes(image))
                else:
                    self.assertEqual(PE.service_preentry_declarations(bytes(image))["tls_callback_table_rva"],
                                     callback)

    def test_service_preentry_refuses_missing_or_nonexecutable_entry_and_delay_imports(self) -> None:
        image: bytearray = _image(3)
        for entry, reason in (
            (0, "entry lies outside the image"),
            (0x1400, "entry has no unique file-backed section"),
        ):
            with self.subTest(entry=entry), self.assertRaisesRegex(ValueError, reason):
                struct.pack_into("<I", image, _OPTIONAL + 16, entry)
                PE.service_preentry_declarations(bytes(image))
        struct.pack_into("<I", image, _OPTIONAL + 16, 0x1010)
        self.assertEqual(PE.service_preentry_declarations(bytes(image))["entry_rva"], 0x1010)
        wrong: bytearray = bytearray(image)
        struct.pack_into("<I", wrong, _SECTION + 36, 0x40000040)
        with self.assertRaisesRegex(ValueError, "entry has no executable file-backed section"):
            PE.service_preentry_declarations(bytes(wrong))
        delayed: bytearray = with_import_modules(image, delay=("KERNEL32.dll",))
        self.assertEqual(PE.static_import_modules(bytes(delayed), subsystem="console")["delay"],
                         ["kernel32.dll"])
        with self.assertRaisesRegex(ValueError, "refuses delay imports"):
            PE.service_preentry_declarations(bytes(delayed))

    def test_ui_preentry_binds_the_selected_linked_publics_to_the_gui_image(self) -> None:
        image: bytearray = with_import_modules(_image(2), direct=("KERNEL32.dll",))
        struct.pack_into("<I", image, _OPTIONAL + 16, 0x1010)
        gate_digest: str = "a" * 64
        gate_name: str = f"nevrotick-ui-entry-{gate_digest}.obj"
        gate_row: str = f" 0001:00000010 nevrotick_ui_entry 0000000140001010 {gate_name}"
        crt_row: str = " 0001:00000050 mainCRTStartup 0000000140001050 msvcrt:exe_main.obj"
        map_text: str = "\n".join((
            " nevrotick_ui",
            " Preferred load address is 0000000140000000",
            "  Address         Publics by Value              Rva+Base               Lib:Object",
            gate_row,
            crt_row,
            " entry point at         0001:00000010",
            " Static symbols",
            "",
        ))
        expected: dict[str, object] = {
            "entry_rva": 0x1010, "image_base": _IMAGE_BASE, "crt_startup_rva": 0x1050,
            "tls_callback_table_rva": 0, "direct_import_modules": ["kernel32.dll"],
        }
        self.assertEqual(PE.ui_preentry_declarations(bytes(image), map_text, object_sha256=gate_digest), expected)
        mapped_path: str = map_text.replace(gate_name, "/work/metadata/" + gate_name).replace("\n", "\r\n")
        self.assertEqual(PE.ui_preentry_declarations(bytes(image), mapped_path, object_sha256=gate_digest), expected)
        for faulty_map, reason in (
            (map_text.replace(gate_name, "foreign.obj"), "gate object differs"),
            (map_text.replace("entry point at         0001:00000010",
                              "entry point at         0001:00000050"), "entry point differs"),
            (map_text.replace("140000000", "150000000"), "preferred address differs"),
            (map_text.replace(gate_row, gate_row + "\n" + gate_row), "public symbol is absent or ambiguous"),
            (map_text.replace(crt_row, ""), "mainCRTStartup public symbol is absent"),
            (map_text.replace("00000050 mainCRTStartup", "00000060 mainCRTStartup"), "segment and preferred"),
            (map_text.replace("00000050 mainCRTStartup 0000000140001050",
                              "00000400 mainCRTStartup 0000000140001400"), "no unique file-backed section"),
            (map_text.replace("00000050 mainCRTStartup 0000000140001050",
                              "00000010 mainCRTStartup 0000000140001010"), "distinct UI baseline gate"),
        ):
            with self.subTest(reason=reason), self.assertRaisesRegex(ValueError, reason):
                PE.ui_preentry_declarations(bytes(image), faulty_map, object_sha256=gate_digest)
        crossed: bytearray = bytearray(image)
        struct.pack_into("<I", crossed, _OPTIONAL + 16, 0x1050)
        with self.assertRaisesRegex(ValueError, "distinct UI baseline gate"):
            PE.ui_preentry_declarations(bytes(crossed), map_text, object_sha256=gate_digest)
        struct.pack_into("<H", crossed, _OPTIONAL + 68, 3)
        with self.assertRaisesRegex(ValueError, "subsystem changed"):
            PE.ui_preentry_declarations(bytes(crossed), map_text, object_sha256=gate_digest)

    def test_ui_preentry_checks_callbacks_and_delay_imports_before_returning_a_gate(self) -> None:
        image: bytearray = _image(2)
        struct.pack_into("<I", image, _OPTIONAL + 16, 0x1010)
        struct.pack_into("<II", image, _OPTIONAL + 112 + 9 * 8, 0x12A0, 40)
        struct.pack_into("<Q", image, 0x4B8, _IMAGE_BASE + 0x12C8)
        struct.pack_into("<H", image, 0x22A, 0xA2B8)
        gate_digest: str = "b" * 64
        map_text: str = "\n".join((
            " Preferred load address is 0000000140000000",
            "  Address         Publics by Value              Rva+Base               Lib:Object",
            f" 0001:00000010 nevrotick_ui_entry 0000000140001010 nevrotick-ui-entry-{gate_digest}.obj",
            " 0001:00000050 mainCRTStartup 0000000140001050 msvcrt:exe_main.obj",
            " entry point at         0001:00000010",
            " Static symbols",
            "",
        ))
        observed: dict[str, object] = PE.ui_preentry_declarations(bytes(image), map_text, object_sha256=gate_digest)
        self.assertEqual(observed["tls_callback_table_rva"], 0x12C8)
        callback: bytearray = bytearray(image)
        struct.pack_into("<Q", callback, 0x4C8, _IMAGE_BASE + 0x1010)
        relocation: bytearray = bytearray(image)
        struct.pack_into("<H", relocation, 0x228, 0xA2C7)
        delayed: bytearray = with_import_modules(image, delay=("KERNEL32.dll",))
        for refused, reason in (
            (callback, "UI startup refuses pre-entry TLS callbacks"),
            (relocation, "UI TLS callback slot is covered by a base relocation"),
            (delayed, "UI startup refuses delay imports"),
        ):
            with self.subTest(reason=reason), self.assertRaisesRegex(ValueError, reason):
                PE.ui_preentry_declarations(bytes(refused), map_text, object_sha256=gate_digest)
        self.assertEqual(PE.ui_preentry_declarations(bytes(image), map_text, object_sha256=gate_digest), observed)

    def test_static_import_descriptor_work_has_an_inclusive_limit(self) -> None:
        # One name per row, all routed to the same independently placed module.
        table_raw: int = 0x600
        table_rva: int = 0x2000
        name: bytes = b"kernel32.dll\0"
        for count in (4096, 4097):
            with self.subTest(count=count):
                image: bytearray = _image(3)
                table_size: int = (count + 1) * 20
                name_rva: int = table_rva + table_size
                rows: bytes = struct.pack("<5I", 0, 0, 0, name_rva, 0) * count
                image.extend(rows + bytes(20) + name)
                section_size: int = len(image) - table_raw
                struct.pack_into("<H", image, 0x86, 2)
                struct.pack_into("<IIII", image, _SECTION + 40 + 8,
                                 section_size, table_rva, section_size, table_raw)
                struct.pack_into("<II", image, _OPTIONAL + 120, table_rva, table_size)
                if count == 4096:
                    self.assertEqual(PE.static_import_modules(bytes(image), subsystem="console"),
                                     {"direct": ["kernel32.dll"] * count, "delay": []})
                else:
                    with self.assertRaisesRegex(ValueError, "direct import directory has too many descriptors"):
                        PE.static_import_modules(bytes(image), subsystem="console")

    def test_import_names_refuse_unbacked_or_ambiguous_rva_extents(self) -> None:
        image: bytearray = with_import_modules(
            _ui_resource_image(b"version", b"manifest"), direct=("KERNEL32.dll",))
        self.assertEqual(PE.static_import_modules(bytes(image), subsystem="gui"),
                         {"direct": ["kernel32.dll"], "delay": []})
        controls: tuple[tuple[int, int, str], ...] = (
            (0x40C, 0x13FD, "direct import name has no bounded null terminator"),
            (0x40C, 0x5000, "direct import name has no unique file-backed section"),
            (_SECTION + 40 + 12, 0x12E3, "direct import name has no unique file-backed section"),
        )
        for offset, value, reason in controls:
            with self.subTest(offset=offset, value=value), self.assertRaisesRegex(ValueError, reason):
                wrong: bytearray = bytearray(image)
                wrong[0x5FD:0x600] = b"ABC"
                struct.pack_into("<I", wrong, offset, value)
                PE.static_import_modules(bytes(wrong), subsystem="gui")

    def test_import_names_refuse_unsupported_bytes_and_delay_attributes(self) -> None:
        image: bytearray = with_import_modules(_image(2), delay=("KERNEL32.dll",))
        for name, reason in (
            (b"\0", "invalid module name"),
            (b"/K.dll\0", "invalid module name"),
            (b"K.DLL.\0", "invalid module name"),
            (b"K\xff.dll\0", "invalid module name"),
            (b"K" * 128 + b"\0", "no bounded null terminator"),
        ):
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, reason):
                wrong: bytearray = bytearray(image)
                wrong[0x4E0:0x4E0 + len(name)] = name
                PE.static_import_modules(bytes(wrong), subsystem="gui")
        for attributes in (0, 3):
            with self.subTest(attributes=attributes):
                wrong: bytearray = bytearray(image)
                struct.pack_into("<I", wrong, 0x460, attributes)
                self.path.write_bytes(wrong)
                counted: dict = PE.pe_readback(self.path, subsystem="gui", imports=True)
                self.assertEqual(counted["delay_import_descriptors"], 1)
                with self.assertRaisesRegex(ValueError, "delay import directory has unsupported attributes"):
                    PE.static_import_modules(bytes(wrong), subsystem="gui")
        self.assertEqual(PE.static_import_modules(bytes(image), subsystem="gui"),
                         {"direct": [], "delay": ["kernel32.dll"]})

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

    def test_pair_layout_requires_the_exact_selected_page_size(self) -> None:
        image: Path = self.path.with_suffix(".exe")
        image.write_bytes(_image(3, pdb_path=self.path.name, guid=_OBSERVE_GUID))
        for size, crossed in ((4096, 8192), (8192, 4096)):
            with self.subTest(size=size):
                expected: bytearray = _pdb(guid=_OBSERVE_GUID, block=size)
                self.path.write_bytes(expected)
                _, (symbols, digest), identity = PE.read_export_pair(
                    image, self.path, subsystem="console", expected_pdb_page_size=size,
                )
                self.assertEqual(symbols, expected)
                self.assertEqual(identity["sha256"], digest)
                self.assertEqual(digest, hashlib.sha256(expected).hexdigest())
                with self.assertRaisesRegex(ValueError, "unsupported block size"):
                    PE.read_export_pair(image, self.path, subsystem="console", expected_pdb_page_size=crossed)
                for invalid in (8192.0, 16384, True):
                    with self.assertRaisesRegex(ValueError, "outside the supported profiles"):
                        PE.read_export_pair(image, self.path, subsystem="console", expected_pdb_page_size=invalid)
        with self.assertRaisesRegex(ValueError, "unsupported block size"):
            PE.pdb_readback(self.path)

    def test_large_layout_checks_repeating_reserved_pages_at_its_own_stride(self) -> None:
        image: Path = self.path.with_suffix(".exe")
        image.write_bytes(_image(3, pdb_path=self.path.name))
        data: bytearray = _pdb(block=8192)
        struct.pack_into("<I", data, 40, 8195)
        data[4 * 8192 + 12:4 * 8192 + 16] = struct.pack("<I", 4097)
        data_start: int = 4097 * 8192
        with self.path.open("wb") as destination:
            destination.write(data)
            destination.seek(data_start)
            destination.write(data[5 * 8192:5 * 8192 + 48])
            destination.truncate(8195 * 8192)
        # Page 4097 is regular data in this format, while pages 8193/8194 are reserved.
        PE.read_export_pair(image, self.path, subsystem="console", expected_pdb_page_size=8192)
        for reserved in (8193, 8194):
            with self.subTest(reserved=reserved):
                with self.path.open("r+b") as destination:
                    destination.seek(4 * 8192 + 12)
                    destination.write(struct.pack("<I", reserved))
                with self.assertRaisesRegex(ValueError, "stream blocks are invalid or overlap"):
                    PE.read_export_pair(image, self.path, subsystem="console", expected_pdb_page_size=8192)

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
