# SPDX-License-Identifier: GPL-2.0-only
"""Exercise the unsigned S0.6 package reader with private synthetic ZIP files."""

from base64 import b64encode
from codecs import BOM_UTF8
from hashlib import sha256
from io import BytesIO
import json
from pathlib import Path
import struct
import subprocess
import sys
import tempfile
import unittest
from xml.etree import ElementTree as ET
from zipfile import ZIP_DEFLATED, ZIP_STORED, ZipFile
import zlib

from check_package import Refused, parse_xml


HERE = Path(__file__).resolve().parent
MANIFEST = "AppxManifest.xml"
BLOCK_MAP = "AppxBlockMap.xml"
CONTENT_TYPES = "[Content_Types].xml"
INERT = "Inert.exe"
LOGO = "Assets/PackageLogo.png"
BLOCK_BYTES = 65536
XML_BYTES_LIMIT = 64 * 1024
XML_DECLARATION = b'<?xml version="1.0" encoding="UTF-8"?>\n'
SDK_NAMES = (
    "makeappx.exe",
    "appxpackaging.dll",
    "opcservices.dll",
    "Microsoft.Windows.Build.Appx.AppxPackaging.dll.manifest",
    "Microsoft.Windows.Build.Appx.OpcServices.dll.manifest",
    "en-US/AppxPackaging.dll.mui",
)


def payload_blocks(data):
    return (data[start:start + BLOCK_BYTES] for start in range(0, len(data), BLOCK_BYTES))


def deflate_blocks(data):
    """Write byte-aligned, independently decodable spans for the mixed ZIP."""
    compressor = zlib.compressobj(wbits=-zlib.MAX_WBITS)
    spans = [
        compressor.compress(block) + compressor.flush(zlib.Z_FULL_FLUSH)
        for block in payload_blocks(data)
    ]
    return spans, compressor.flush(zlib.Z_FINISH)


def block_map(payload, compressed_sizes, *, stored_size=False):
    root = ET.Element("BlockMap", {
        "xmlns": "http://schemas.microsoft.com/appx/2010/blockmap",
        "HashMethod": "http://www.w3.org/2001/04/xmlenc#sha256",
    })
    for name, data in payload.items():
        record = ET.SubElement(root, "File", {
            "Name": name.replace("/", "\\"),
            "Size": str(len(data)),
            "LfhSize": str(30 + len(name)),
        })
        for index, block in enumerate(payload_blocks(data)):
            attrs = {"Hash": b64encode(sha256(block).digest()).decode("ascii")}
            if name == INERT and compressed_sizes is not None:
                size = compressed_sizes[index]
                if size is not None:
                    attrs["Size"] = str(size)
            if name == LOGO and stored_size:
                attrs["Size"] = str(len(block))
            ET.SubElement(record, "Block", attrs)
    return ET.tostring(root, encoding="utf-8")


def content_types():
    return b"""<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="exe" ContentType="application/octet-stream"/>
  <Default Extension="png" ContentType="image/png"/>
  <Override PartName="/AppxManifest.xml" ContentType="application/vnd.ms-appx.manifest+xml"/>
  <Override PartName="/AppxBlockMap.xml" ContentType="application/vnd.ms-appx.blockmap+xml"/>
</Types>
"""


def make_zip(members, deflated):
    """Write classic ZIP local records and the central directory."""
    local = BytesIO()
    directory = BytesIO()
    for name, data in members.items():
        encoded_name = name.encode("ascii")
        compressed = name in deflated
        flags = 8 if compressed else 0
        method = ZIP_DEFLATED if compressed else ZIP_STORED
        body = deflated.get(name, data)
        lengths = (zlib.crc32(data), len(body), len(data))
        record_offset = local.tell()
        # ZIP version 2.0, time 00:00:00, date 1980-01-01. The streamed
        # member finishes with a signature-prefixed 32-bit data descriptor.
        local_lengths = (0, 0, 0) if compressed else lengths
        local.write(struct.pack("<4s5H3I2H", b"PK\x03\x04", 20, flags, method,
                                0, 33, *local_lengths, len(encoded_name), 0))
        local.write(encoded_name)
        local.write(body)
        if compressed:
            local.write(struct.pack("<4s3I", b"PK\x07\x08", *lengths))
        directory.write(struct.pack("<4s6H3I5H2I", b"PK\x01\x02", 20, 20, flags,
                                    method, 0, 33, *lengths, len(encoded_name),
                                    0, 0, 0, 0, 0, record_offset))
        directory.write(encoded_name)
    end_record = struct.pack("<4s4H2IH", b"PK\x05\x06", 0, 0, len(members),
                             len(members), directory.tell(), local.tell(), 0)
    return local.getvalue() + directory.getvalue() + end_record


class PackageReaderTests(unittest.TestCase):
    def setUp(self):
        self.root = Path(self.enterContext(tempfile.TemporaryDirectory()))
        self.payload_dir = self.root / "payload"
        self.package = self.root / "fixture.msix"
        self.input_set = self.root / "inputs.json"
        self.payload = {
            MANIFEST: (HERE / MANIFEST).read_bytes(),
            INERT: b"A" * BLOCK_BYTES + b"second expanded block",
            "nevrotick-lab-s02-engine.exe": b"engine marker",
            "nevrotick-lab-s02-broker.exe": b"broker marker",
            LOGO: b"package logo marker",
            "Assets/Square44x44Logo.png": b"small logo marker",
            "Assets/Square150x150Logo.png": b"large logo marker",
        }
        for name, data in self.payload.items():
            path = self.payload_dir / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(data)
        inputs = json.dumps({
            "payload": {name: sha256(data).hexdigest() for name, data in self.payload.items()},
            "sdk": {name: sha256(name.encode("ascii")).hexdigest() for name in SDK_NAMES},
        }).encode("utf-8")
        self.input_set.write_bytes(inputs)
        self.input_set_digest = sha256(inputs).hexdigest()

    def write_package(self, *, payload=None, compressed=False, size_case=None, xml_prefix=None):
        if payload is None:
            payload = self.payload
        sizes = None
        compressed_members = {}
        if compressed:
            spans, footer = deflate_blocks(payload[INERT])
            compressed_members[INERT] = b"".join(spans) + footer
            sizes = [len(span) for span in spans]
            if size_case == "missing":
                sizes[0] = None
            elif size_case == "shifted":
                # The whole stream and total declared length stay equal, but
                # the first declared span loses one byte to the second.
                sizes[0] -= 1
                sizes[1] += 1
        members = {
            **payload,
            BLOCK_MAP: block_map(payload, sizes, stored_size=size_case == "stored"),
            CONTENT_TYPES: content_types(),
        }
        if xml_prefix is not None:
            name, prefix = xml_prefix
            members[name] = prefix + members[name]
        self.package.write_bytes(make_zip(members, compressed_members))
        return members

    def run_reader(self, *, expected_package_digest=None):
        if expected_package_digest is None:
            expected_package_digest = sha256(self.package.read_bytes()).hexdigest()
        return subprocess.run(
            [sys.executable, "-I", "-B", str(HERE / "check_package.py"),
             "--payload", str(self.payload_dir), "--package", str(self.package),
             "--input-set", str(self.input_set), "--input-set-sha256", self.input_set_digest,
             "--package-sha256", expected_package_digest],
            capture_output=True, text=True, check=False, timeout=10,
        )

    def assert_accepted(self, members):
        with ZipFile(self.package) as archive:
            self.assertEqual({name: archive.read(name) for name in archive.namelist()}, members)
        result = self.run_reader()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stderr, "")
        package_bytes = self.package.read_bytes()
        self.assertEqual(json.loads(result.stdout), {
            "status": "unsigned-fixture-package-checked",
            "package_identity": {
                "Name": "NevroTick.S06.ScmFixture",
                "Publisher": "CN=NevroTick S06 Laboratory",
                "Version": "1.0.0.0",
                "ProcessorArchitecture": "x64",
            },
            "input_set_sha256": self.input_set_digest,
            "package_bytes": len(package_bytes),
            "package_sha256": sha256(package_bytes).hexdigest(),
            "members": {
                name: {"bytes": len(data), "sha256": sha256(data).hexdigest()}
                for name, data in members.items()
            },
        })

    def assert_refused(self, reason, *, expected_package_digest=None):
        result = self.run_reader(expected_package_digest=expected_package_digest)
        self.assertEqual(result.returncode, 1, result.stderr or "reader accepted the fixture")
        self.assertEqual(result.stdout, "")
        self.assertTrue(result.stderr.startswith("Fixture package refused: "), result.stderr)
        self.assertIn(reason, result.stderr)
        self.assertNotIn("Traceback", result.stderr)

    def test_admits_stored_package(self):
        self.assert_accepted(self.write_package())

    def test_admits_streamed_deflate_with_two_blocks(self):
        self.assert_accepted(self.write_package(compressed=True))

    def test_archive_digest_and_payload_mismatches_are_distinct(self):
        self.write_package()
        original_digest = sha256(self.package.read_bytes()).hexdigest()
        alternate_logo = b"alternate package logo marker"
        self.write_package(payload={**self.payload, LOGO: alternate_logo})
        self.assert_refused("package SHA-256 differs", expected_package_digest=original_digest)
        self.assert_refused(f"emitted package member differs: {LOGO}")
        (self.payload_dir / LOGO).write_bytes(alternate_logo)
        self.assert_refused(f"authored payload member differs from the frozen input set: {LOGO}")

    def test_rejects_contradictory_local_header_fields(self):
        for compressed in (False, True):
            self.write_package(compressed=compressed)
            original = self.package.read_bytes()
            with ZipFile(self.package) as archive:
                local_header = archive.getinfo(INERT).header_offset
            for field, offset in (("CRC", 14), ("compressed size", 18), ("file size", 22)):
                with self.subTest(streamed=compressed, field=field):
                    changed = bytearray(original)
                    changed[local_header + offset] ^= 1
                    self.package.write_bytes(changed)
                    self.assert_refused(f"{INERT} local ZIP CRC or lengths differ")

    def test_rejects_missing_and_false_compressed_block_sizes(self):
        for size_case, reason in (
            ("missing", f"{INERT} requires a compressed block Size"),
            ("shifted", f"{INERT} compressed block bytes do not match at expanded offset 0"),
            ("stored", f"{LOGO} has a Size on a stored block"),
        ):
            with self.subTest(size_case=size_case):
                self.write_package(compressed=True, size_case=size_case)
                self.assert_refused(reason)

    def test_admits_utf8_xml_declarations_with_optional_bom(self):
        for name in (BLOCK_MAP, CONTENT_TYPES):
            with self.subTest(member=name, bom=False):
                self.assert_accepted(self.write_package(xml_prefix=(name, XML_DECLARATION)))
            with self.subTest(member=name, bom=True):
                prefix = BOM_UTF8 + XML_DECLARATION.replace(b'"UTF-8"', b"'uTf-8'")
                self.assert_accepted(self.write_package(xml_prefix=(name, prefix)))

    def test_rejects_utf8_members_declaring_utf16(self):
        for name in (BLOCK_MAP, CONTENT_TYPES):
            for bom in (b"", BOM_UTF8):
                with self.subTest(member=name, bom=bool(bom)):
                    prefix = bom + XML_DECLARATION.replace(b"UTF-8", b"UTF-16")
                    self.write_package(xml_prefix=(name, prefix))
                    self.assert_refused(f"{name} must use UTF-8 XML encoding")


class XmlDeclarationTests(unittest.TestCase):
    def test_direct_invalid_declaration_controls(self):
        body = "<root>\u00e9</root>".encode("utf-8")
        self.assertEqual(parse_xml(XML_DECLARATION + body, "test input").text, "\u00e9")
        for declaration in (
            XML_DECLARATION.replace(b"UTF-8", b"ISO-8859-1"),
            XML_DECLARATION.replace(b"UTF-8", b"x-unknown"),
            XML_DECLARATION.replace(b'encoding="UTF-8"', b'encoding="UTF 8"'),
        ):
            with self.subTest(declaration=declaration):
                with self.assertRaisesRegex(Refused, "test input"):
                    parse_xml(declaration + body, "test input")

    def test_rejects_bomless_utf16_instead_of_detecting_its_byte_order(self):
        self.assertEqual(parse_xml(b"<root/>", "test input").tag, "root")
        for encoding in ("utf-16-le", "utf-16-be"):
            with self.subTest(encoding=encoding):
                with self.assertRaisesRegex(Refused, "test input"):
                    parse_xml("<root/>".encode(encoding), "test input")

    def test_xml_size_limit_counts_original_bytes(self):
        declaration = BOM_UTF8 + b'<?xml version="1.0"'
        closing = b"?><root/>"
        exact_limit = declaration.ljust(XML_BYTES_LIMIT - len(closing), b" ") + closing
        self.assertEqual(parse_xml(exact_limit, "test input").tag, "root")
        with self.assertRaisesRegex(Refused, "XML input bound"):
            parse_xml(exact_limit + b" ", "test input")

    def test_refuses_dtd_with_inert_entity(self):
        data = b'<!DOCTYPE root [<!ENTITY marker "fixture">]><root>&marker;</root>'
        with self.assertRaisesRegex(Refused, "document type or entity declaration"):
            parse_xml(data, "test input")
