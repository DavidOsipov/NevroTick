"""Independent raw resource and pinned-reader grammar controls."""
import hashlib
from pathlib import Path
import struct
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from windows_versioninfo import decode, decode_imports, decode_ui_resources


_FIXED = (0xFEEF04BD, 0x10000, 0x10002, 0x30004, 0x50006, 0x70008, 0x3F, 2, 0x40004, 1, 0, 0, 0)
_DESCRIPTION = "A café | 漢字 𐐀"
_MANIFEST = (
    b'<?xml version="1.0" standalone="yes"?>\n'
    b'<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">\n'
    b'  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">\n'
    b'    <security><requestedPrivileges>\n'
    b'      <requestedExecutionLevel level="asInvoker" uiAccess="false"/>\n'
    b'    </requestedPrivileges></security>\n'
    b'  </trustInfo>\n'
    b'</assembly>\n'
)


def record(key, value, kind, *children):
    """Author a Win32 VERSIONINFO test record from literal fields."""
    body = bytearray(b"\0" * 6)
    body.extend((key + "\0").encode("utf-16-le"))
    body.extend(b"\0" * (-len(body) % 4))
    body.extend(value)
    for child in children:
        body.extend(b"\0" * (-len(body) % 4))
        body.extend(child)
    struct.pack_into("<HHH", body, 0, len(body), len(value) // (2 if kind else 1), kind)
    return bytes(body)


def version_resource(*, fixed=_FIXED, translation=(1033, 1200),
                     table="040904B0", fields=None):
    """Build a literal positive resource with replaceable malformed neighbors."""
    if fields is None:
        fields = (
            record("FileDescription", (_DESCRIPTION + "\0").encode("utf-16-le"), 1),
            record("OriginalFilename", "nevrotick-ui.exe\0".encode("utf-16-le"), 1),
        )
    strings = record("StringFileInfo", b"", 1, record(table, b"", 1, *fields))
    var = record("VarFileInfo", b"", 1, record("Translation", struct.pack("<HH", *translation), 0))
    return record("VS_VERSION_INFO", struct.pack("<13I", *fixed), 0, strings, var)


def readout_leaf(payload, kind, rva, offsets):
    """Describe one leaf and its raw bytes in the pinned reader grammar."""
    rows = []
    for offset in range(0, len(payload), 16):
        chunk = payload[offset:offset + 16]
        groups = " ".join(chunk[index:index + 4].hex().upper() for index in range(0, len(chunk), 4))
        rows.append(f"            {offset:04X}: {groups:<37}|{'.' * len(chunk)}|\n")
    type_offset, name_offset, entry_offset = offsets
    return (
        f"  Type: {kind} [\n"
        f"    Table Offset: 0x{type_offset:X}\n"
        "    Number of String Entries: 0\n"
        "    Number of ID Entries: 1\n"
        "    Name: (ID 1) [\n"
        f"      Table Offset: 0x{name_offset:X}\n"
        "      Number of String Entries: 0\n"
        "      Number of ID Entries: 1\n"
        "      Language: (ID 1033) [\n"
        f"        Entry Offset: 0x{entry_offset:X}\n"
        "        Time/Date Stamp: 1970-01-01 00:00:00 (0x0)\n"
        "        Major Version: 0\n"
        "        Minor Version: 0\n"
        "        Characteristics: 0\n"
        "        Data [\n"
        f"          DataRVA: 0x{rva:X}\n"
        f"          DataSize: {len(payload)}\n"
        "          Codepage: 0\n"
        "          Reserved: 0\n"
        "          Data (\n"
        + "".join(rows)
        + "          )\n"
        "        ]\n"
        "      ]\n"
        "    ]\n"
        "  ]\n"
    )


def readout(payload, *, manifest=None):
    """Present raw version bytes and the optional adjacent manifest leaf."""
    count = 1
    size = 0x60 + len(payload)
    leaves = readout_leaf(payload, "VERSIONINFO (ID 16)", 0x2060, (0x18, 0x30, 0x48))
    if manifest is not None:
        count = 2
        manifest_rva = 0x20A0 + len(payload) + (-len(payload) % 4)
        size = manifest_rva - 0x2000 + len(manifest)
        leaves = (readout_leaf(payload, "VERSIONINFO (ID 16)", 0x20A0, (0x20, 0x50, 0x80))
                  + readout_leaf(manifest, "MANIFEST (ID 24)", manifest_rva, (0x38, 0x68, 0x90)))
    return (
        "File: nevrotick-ui.exe\n"
        "Format: COFF-x86-64\n"
        "Arch: x86_64\n"
        "AddressSize: 64bit\n"
        "ImageFileHeader {\n"
        "  Machine: IMAGE_FILE_MACHINE_AMD64 (0x8664)\n"
        "  Characteristics [ (0x22)\n"
        "    IMAGE_FILE_EXECUTABLE_IMAGE (0x2)\n"
        "    IMAGE_FILE_LARGE_ADDRESS_AWARE (0x20)\n"
        "  ]\n"
        "}\n"
        "ImageOptionalHeader {\n"
        "  Magic: 0x20B\n"
        "  Subsystem: IMAGE_SUBSYSTEM_WINDOWS_GUI (0x2)\n"
        "  DataDirectory {\n"
        "    ResourceTableRVA: 0x2000\n"
        f"    ResourceTableSize: 0x{size:X}\n"
        "  }\n"
        "}\n"
        "Resources [\n"
        f"  Total Number of Resources: {count}\n"
        "  Base Table Address: 0x400\n"
        "  \n"
        "  Number of String Entries: 0\n"
        f"  Number of ID Entries: {count}\n"
        + leaves
        + "]\n"
    )


class WindowsVersionInfoTests(unittest.TestCase):
    def setUp(self):
        self.payload = version_resource()
        self.report = readout(self.payload)

    def test_reads_fixed_fields_and_supplementary_utf16(self):
        self.assertEqual(decode(self.report), {
            "fixed": _FIXED,
            "strings": {"FileDescription": _DESCRIPTION, "OriginalFilename": "nevrotick-ui.exe"},
            "translation": (1033, 1200),
        })

    def test_preserves_changed_version_and_wrong_component_for_caller_comparison(self):
        # Both records are structurally valid. The caller must see the actual
        # mismatched versions and component, including equal-count substitution.
        changed = _FIXED[:2] + (7,) + _FIXED[3:]
        fields = (
            record("FileDescription", "Observe diagnostic\0".encode("utf-16-le"), 1),
            record("OriginalFilename", "nevrotick-observe.exe\0".encode("utf-16-le"), 1),
        )
        result = decode(readout(version_resource(fixed=changed, fields=fields)))
        self.assertEqual(result["fixed"], changed)
        self.assertEqual(result["strings"]["OriginalFilename"], "nevrotick-observe.exe")
        self.assertEqual(len(result["strings"]), len(decode(self.report)["strings"]))

    def test_rejects_wrong_component_image_profile_and_duplicate_headers(self):
        for source, replacement in (
            ("nevrotick-ui.exe", "nevrotick-ui.dll"),
            ("COFF-x86-64", "COFF-ARM64"),
            ("IMAGE_FILE_MACHINE_AMD64 (0x8664)", "IMAGE_FILE_MACHINE_I386 (0x14C)"),
            ("Characteristics [ (0x22)", "Characteristics [ (0x2022)"),
            ("Magic: 0x20B", "Magic: 0x10B"),
            ("IMAGE_SUBSYSTEM_WINDOWS_GUI (0x2)", "IMAGE_SUBSYSTEM_WINDOWS_CUI (0x3)"),
            ("Format: COFF-x86-64", "Format: COFF-x86-64\nFormat: COFF-x86-64"),
        ):
            with self.subTest(replacement=replacement), self.assertRaises(ValueError):
                decode(self.report.replace(source, replacement, 1))

    def test_observe_requires_the_console_subsystem(self):
        console = self.report.replace("IMAGE_SUBSYSTEM_WINDOWS_GUI (0x2)", "IMAGE_SUBSYSTEM_WINDOWS_CUI (0x3)")
        self.assertEqual(decode(console, role="observe"), decode(self.report))
        with self.assertRaisesRegex(ValueError, "PE32\\+"):
            decode(self.report, role="observe")
        with self.assertRaisesRegex(ValueError, "PE32\\+"):
            decode(console, role="ui")
        with self.assertRaisesRegex(ValueError, "role"):
            decode(console, role="broker")

    def test_rejects_missing_duplicate_extra_or_wrong_resource_identity(self):
        header, mark, _ = self.report.partition("Resources [\n")
        self.assertTrue(mark)
        with self.assertRaisesRegex(ValueError, "absent"):
            decode(header + mark + "]\n")
        for source, replacement in (
            ("Total Number of Resources: 1", "Total Number of Resources: 2"),
            ("VERSIONINFO (ID 16)", "MANIFEST (ID 24)"),
            ("Name: (ID 1)", "Name: (ID 2)"),
            ("Language: (ID 1033)", "Language: (ID 1031)"),
            ("      Number of ID Entries: 1", "      Number of ID Entries: 2"),
            ("        Data [", "        Data [\n        Data ["),
        ):
            with self.subTest(replacement=replacement), self.assertRaisesRegex(ValueError, "resource tree"):
                decode(self.report.replace(source, replacement, 1))

    def test_rejects_truncated_disordered_and_nonhex_dump_rows(self):
        lines = self.report.splitlines(keepends=True)
        last_row = next(index for index in range(len(lines) - 1, -1, -1)
                        if lines[index].startswith("            "))
        del lines[last_row]
        with self.assertRaisesRegex(ValueError, "truncated"):
            decode("".join(lines))
        for source, replacement in (("0010:", "0020:"), ("0000: ", "0000: X")):
            with self.subTest(replacement=replacement), self.assertRaisesRegex(ValueError, "hex row"):
                decode(self.report.replace(source, replacement, 1))

    def test_accepts_exactly_4096_bytes_without_losing_the_final_row(self):
        empty = version_resource(fields=(record("Comments", b"\0\0", 1),))
        text = "x" * ((4096 - len(empty)) // 2)
        payload = version_resource(fields=(record("Comments", (text + "\0").encode("utf-16-le"), 1),))
        self.assertEqual(len(payload), 4096)
        self.assertEqual(decode(readout(payload))["strings"], {"Comments": text})

    def test_rejects_payload_span_size_and_report_overflow(self):
        for source, replacement in (
            ("DataRVA: 0x2060", "DataRVA: 0x1FFF"),
            (f"DataSize: {len(self.payload)}", "DataSize: 4097"),
        ):
            with self.subTest(replacement=replacement), self.assertRaisesRegex(ValueError, "bounds"):
                decode(self.report.replace(source, replacement, 1))
        with self.assertRaisesRegex(ValueError, "oversized"):
            decode(self.report + " " * 32_768)

    def test_rejects_inconsistent_root_table_translation_and_fixed_header(self):
        cases = (
            (b"\xff\xff" + self.payload[2:], "block header"),
            (version_resource(table="040904E4"), "string table identity"),
            (version_resource(translation=(1031, 1200)), "translation"),
            (version_resource(fixed=(0,) + _FIXED[1:]), "fixed-value identity"),
        )
        for payload, reason in cases:
            with self.subTest(reason=reason), self.assertRaisesRegex(ValueError, reason):
                decode(readout(payload))

    def test_rejects_duplicate_string_fields_and_invalid_utf16(self):
        text = record("FileDescription", b"x\0\0\0", 1)
        with self.assertRaisesRegex(ValueError, "duplicate VERSIONINFO block"):
            decode(readout(version_resource(fields=(text, text))))
        for value in (b"\0\xd8\0\0", b"\0\xdc\0\0"):
            with self.subTest(value=value), self.assertRaises(UnicodeDecodeError):
                decode(readout(version_resource(fields=(record("FileDescription", value, 1),))))

    def test_rejects_missing_and_interior_string_terminators(self):
        for val in (b"x\0", b"x\0\0\0y\0\0\0"):
            with self.subTest(value=val), self.assertRaisesRegex(ValueError, "string terminator"):
                decode(readout(version_resource(fields=(record("FileDescription", val, 1),))))

    def test_rejects_ambiguous_padding_and_excessive_nesting(self):
        # Root key's first alignment byte must be zero.
        padded = self.payload[:38] + b"\x01" + self.payload[39:]
        with self.assertRaisesRegex(ValueError, "padding"):
            decode(readout(padded))
        child = record("inner", b"x\0\0\0", 1)
        nested = record("outer", b"", 1, child)
        with self.assertRaisesRegex(ValueError, "block exceeds"):
            decode(readout(version_resource(fields=(nested,))))


class WindowsUiResourceTests(unittest.TestCase):
    def setUp(self):
        self.payload = version_resource()
        self.report = readout(self.payload, manifest=_MANIFEST)

    def test_reads_both_leaves_and_hashes_original_manifest_bytes(self):
        version = decode(readout(self.payload))
        for data in (_MANIFEST, _MANIFEST.replace(b"\n", b"\r\n")):
            with self.subTest(data=data):
                self.assertEqual(decode_ui_resources(readout(self.payload, manifest=data)), (
                    version, {"level": "asInvoker", "ui_access": False,
                              "sha256": hashlib.sha256(data).hexdigest()},
                ))

    def test_keeps_single_resource_legacy_readers_and_dual_ui_separate(self):
        single = readout(self.payload)
        console = "IMAGE_SUBSYSTEM_WINDOWS_CUI (0x3)"
        gui = "IMAGE_SUBSYSTEM_WINDOWS_GUI (0x2)"
        self.assertEqual(decode(single), decode(single.replace(gui, console), role="observe"))
        with self.assertRaisesRegex(ValueError, "resource tree"):
            decode_ui_resources(single)
        with self.assertRaisesRegex(ValueError, "resource tree"):
            decode(self.report)
        with self.assertRaisesRegex(ValueError, "resource tree"):
            decode(self.report.replace(gui, console), role="observe")
        with self.assertRaisesRegex(ValueError, "PE32\\+"):
            decode_ui_resources(self.report.replace(gui, console))
        with self.assertRaisesRegex(ValueError, "different UI executable"):
            decode_ui_resources(self.report.replace("nevrotick-ui.exe", "nevrotick-observe.exe", 1))

    def test_rejects_wrong_duplicate_missing_or_unexpected_leaf_identity(self):
        version_report, mark, manifest_report = self.report.partition("  Type: MANIFEST (ID 24) [\n")
        self.assertTrue(mark)
        for report in (
            version_report + "]\n",
            version_report + mark + manifest_report.replace("Name: (ID 1)", "Name: (ID 2)"),
            version_report + mark + manifest_report.replace("Language: (ID 1033)", "Language: (ID 1031)"),
            version_report + "  Type: VERSIONINFO (ID 16) [\n" + manifest_report,
            version_report + "  Type: ICON (ID 3) [\n" + manifest_report,
            self.report[:-2] + mark + manifest_report,
            self.report.replace("Total Number of Resources: 2", "Total Number of Resources: 3"),
            self.report.replace("  Number of ID Entries: 2", "  Number of ID Entries: 1"),
        ):
            with self.subTest(report=report[-120:]), self.assertRaisesRegex(ValueError, "resource tree"):
                decode_ui_resources(report)

    def test_refuses_aliases_and_out_of_bounds_manifest_payloads(self):
        prefix, mark, manifest = self.report.partition("  Type: MANIFEST (ID 24) [\n")
        self.assertTrue(mark)
        rva = next(line for line in manifest.splitlines() if "DataRVA: " in line)
        for target in ("0x20A0", "0xFFFFFFFF"):
            with self.subTest(target=target), self.assertRaisesRegex(ValueError, "overlaps or exceeds"):
                decode_ui_resources(prefix + mark + manifest.replace(rva, "          DataRVA: " + target))
        oversized = readout(self.payload, manifest=b"x" * 4096)
        size = next(line for line in oversized.splitlines() if "ResourceTableSize:" in line)
        oversized = oversized.replace(size, "    ResourceTableSize: 0x2000")
        with self.assertRaisesRegex(ValueError, "admitted bounds"):
            decode_ui_resources(oversized.replace("DataSize: 4096", "DataSize: 4097"))
        with self.assertRaisesRegex(ValueError, "oversized"):
            decode_ui_resources(self.report + " " * 65_536)

    def test_rejects_overlapping_ui_resource_directory_and_payload_spans(self):
        for original, offset in (
            ("    Table Offset: 0x20\n", "0x18"),
            ("    Table Offset: 0x20\n", "0x38"),
            ("      Table Offset: 0x68\n", "0x20"),
            ("        Entry Offset: 0x90\n", "0xA0"),
        ):
            self.assertIn(original, self.report)
            changed = self.report.replace(original, original.partition("0x")[0] + offset + "\n", 1)
            with self.subTest(original=original, offset=offset), self.assertRaisesRegex(ValueError, "directory spans"):
                decode_ui_resources(changed)

    def test_rejects_out_of_bounds_ui_resource_directories_and_data_entry(self):
        for original in ("    Table Offset: 0x20", "      Table Offset: 0x68", "        Entry Offset: 0x90"):
            self.assertIn(original + "\n", self.report)
            changed = self.report.replace(original + "\n", original.partition("0x")[0] + "0xFFFFFFF8\n", 1)
            with self.subTest(original=original), self.assertRaisesRegex(ValueError, "directory spans"):
                decode_ui_resources(changed)

    def test_rejects_wrong_or_missing_execution_attributes(self):
        for before, after in (
            (b'level="asInvoker"', b'level="requireAdministrator"'),
            (b'level="asInvoker"', b'level="highestAvailable"'),
            (b' level="asInvoker"', b""),
            (b'uiAccess="false"', b'uiAccess="true"'),
            (b' uiAccess="false"', b""),
            (b'uiAccess="false"', b'uiAccess="0"'),
            (b'uiAccess="false"', b'uiAccess="false" unexpected="false"'),
        ):
            with self.subTest(after=after), self.assertRaisesRegex(ValueError, "structure or settings"):
                decode_ui_resources(readout(self.payload, manifest=_MANIFEST.replace(before, after)))

    def test_rejects_other_authority_xml_and_extra_content(self):
        for before, after in (
            (b"asm.v3", b"asm.v2"),
            (b'manifestVersion="1.0"', b'manifestVersion="2.0"'),
            (b"</security>", b"</security><security/>"),
            (b"</requestedPrivileges>", b"<requestedExecutionLevel level='asInvoker' uiAccess='false'/>"
                                        b"</requestedPrivileges>"),
            (b"</trustInfo>", b"</trustInfo><compatibility/>"),
            (b"</security>", b"unexpected</security>"),
        ):
            with self.subTest(after=after), self.assertRaisesRegex(ValueError, "structure or settings"):
                decode_ui_resources(readout(self.payload, manifest=_MANIFEST.replace(before, after)))
        bad = _MANIFEST.replace(b'uiAccess="false"', b'uiAccess="false" uiAccess="true"')
        with self.assertRaisesRegex(ValueError, "malformed UI execution manifest XML"):
            decode_ui_resources(readout(self.payload, manifest=bad))

    def test_rejects_forbidden_xml_constructs_and_encoding_before_xml_parse(self):
        for manifest in (
            _MANIFEST.replace(b"<assembly", b'<!DOCTYPE assembly SYSTEM "unused"><assembly'),
            _MANIFEST.replace(b'asInvoker', b'as&#73;nvoker'),
            _MANIFEST.replace(b"<assembly", b"<?other value?><assembly"),
            _MANIFEST.replace(b"<assembly", b"<!--ignored--><assembly"),
            _MANIFEST.replace(b"</assembly>", b"</assembly>\n<?other value?>"),
            _MANIFEST.replace(b'standalone="yes"', b'encoding="US-ASCII"'),
            _MANIFEST.decode("ascii").encode("utf-16"),
            b"\xef\xbb\xbf" + _MANIFEST,
            _MANIFEST + b"\0",
        ):
            with self.subTest(data=manifest[:64]), self.assertRaisesRegex(ValueError, "encoding or XML declaration"):
                decode_ui_resources(readout(self.payload, manifest=manifest))


class WindowsImportTests(unittest.TestCase):
    image = "/work/target/x86_64-pc-windows-msvc/release/nevrotick-ui.exe"
    head = f"\nFile: {image}\nFormat: COFF-x86-64\nArch: x86_64\nAddressSize: 64bit\n"
    direct = ("Import {\n  Name: KERNEL32.dll\n  ImportLookupTableRVA: 0x3088\n"
              "  ImportAddressTableRVA: 0x3098\n  Symbol: ExitProcess (0)\n}\n")
    delay = ("DelayImport {\n  Name: user32.dll\n  Attributes: 0x1\n"
             "  ModuleHandle: 0x4018\n  ImportAddressTable: 0x4038\n  ImportNameTable: 0x3040\n"
             "  BoundDelayImportTable: 0x0\n  UnloadDelayImportTable: 0x0\n"
             "  Import {\n    Symbol: GetSystemMetrics (0)\n    Address: 0x140001030\n  }\n}\n")

    def test_keeps_direct_and_delayed_modules_separate(self):
        genuine = self.head + self.direct + self.delay
        for output in (genuine, genuine.strip()):
            self.assertEqual(decode_imports(output, image=self.image),
                             {"direct": ["kernel32.dll"], "delay": ["user32.dll"]})
        self.assertEqual(decode_imports(self.head + self.direct, image=self.image),
                         {"direct": ["kernel32.dll"], "delay": []})

    def test_refuses_wrong_image_partial_records_and_unsupported_names(self):
        genuine = self.head + self.direct + self.delay
        for report, reason in (
            (genuine.replace("nevrotick-ui.exe", "nevrotick-observe.exe", 1), "different image"),
            (genuine[:-2], "incomplete"),
            (genuine.replace("user32.dll", "dir/user32.dll", 1), "descriptor output"),
            (genuine.replace("  Attributes: 0x1", "  Attributes: 0x2", 1), "descriptor output"),
            (genuine + " " * 65_536, "oversized"),
        ):
            with self.subTest(reason=reason), self.assertRaisesRegex(ValueError, reason):
                decode_imports(report, image=self.image)


if __name__ == "__main__":
    unittest.main()
