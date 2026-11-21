"""Independent raw resource and pinned-reader grammar controls."""
from pathlib import Path
import struct
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from windows_versioninfo import decode


_FIXED = (0xFEEF04BD, 0x10000, 0x10002, 0x30004, 0x50006, 0x70008, 0x3F, 2, 0x40004, 1, 0, 0, 0)
_DESCRIPTION = "A café | 漢字 𐐀"


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


def readout(payload):
    """Present actual raw bytes in the bounded reader grammar."""
    rows = []
    for offset in range(0, len(payload), 16):
        chunk = payload[offset:offset + 16]
        groups = " ".join(chunk[index:index + 4].hex().upper() for index in range(0, len(chunk), 4))
        rows.append(f"            {offset:04X}: {groups:<37}|{'.' * len(chunk)}|\n")
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
        f"    ResourceTableSize: 0x{len(payload)+0x60:X}\n"
        "  }\n"
        "}\n"
        "Resources [\n"
        "  Total Number of Resources: 1\n"
        "  Base Table Address: 0x400\n"
        "  \n"
        "  Number of String Entries: 0\n"
        "  Number of ID Entries: 1\n"
        "  Type: VERSIONINFO (ID 16) [\n"
        "    Table Offset: 0x18\n"
        "    Number of String Entries: 0\n"
        "    Number of ID Entries: 1\n"
        "    Name: (ID 1) [\n"
        "      Table Offset: 0x30\n"
        "      Number of String Entries: 0\n"
        "      Number of ID Entries: 1\n"
        "      Language: (ID 1033) [\n"
        "        Entry Offset: 0x48\n"
        "        Time/Date Stamp: 1970-01-01 00:00:00 (0x0)\n"
        "        Major Version: 0\n"
        "        Minor Version: 0\n"
        "        Characteristics: 0\n"
        "        Data [\n"
        "          DataRVA: 0x2060\n"
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
        "]\n"
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


if __name__ == "__main__":
    unittest.main()
