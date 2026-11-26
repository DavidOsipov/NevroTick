# SPDX-License-Identifier: GPL-2.0-only
"""Exercise the S0.6 package reader with private synthetic ZIP files."""

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
SIGNATURE_PART = "AppxSignature.p7x"
CATALOG_PART = "AppxMetadata/CodeIntegrity.cat"
SIGNATURE_TYPE = (
    b'  <Override PartName="/AppxSignature.p7x" ContentType="application/vnd.ms-appx.signature"/>\n'
)
CATALOG_TYPE = (
    b'  <Override PartName="/AppxMetadata/CodeIntegrity.cat" ContentType="application/vnd.ms-pkiseccat"/>\n'
)
INERT = "Inert.exe"
EVENT_RESOURCE = "nevrotick-test-events.dll"
EVENT_DEFAULT_TYPE = b'  <Default Extension="dll" ContentType="application/x-msdownload"/>\n'
EVENT_OVERRIDE_TYPE = (
    b'  <Override PartName="/nevrotick-test-events.dll" ContentType="application/octet-stream"/>\n'
)
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


def block_map(payload, compressed_sizes, *, stored_size=False, local_extras=None):
    local_extras = {} if local_extras is None else local_extras
    root = ET.Element("BlockMap", {
        "xmlns": "http://schemas.microsoft.com/appx/2010/blockmap",
        "HashMethod": "http://www.w3.org/2001/04/xmlenc#sha256",
    })
    for name, data in payload.items():
        record = ET.SubElement(root, "File", {
            "Name": name.replace("/", "\\"),
            "Size": str(len(data)),
            "LfhSize": str(30 + len(name) + len(local_extras.get(name, b""))),
        })
        for index, block in enumerate(payload_blocks(data)):
            attrs = {"Hash": b64encode(sha256(block).digest()).decode("ascii")}
            if name in compressed_sizes:
                size = compressed_sizes[name][index]
                if size is not None:
                    attrs["Size"] = str(size)
            if name == LOGO and stored_size:
                attrs["Size"] = str(len(block))
            ET.SubElement(record, "Block", attrs)
    return ET.tostring(root, encoding="utf-8")


def content_types(extra_declaration=b""):
    return b"""<Types xmlns="http://schemas.openxmlformats.org/package/2006/content-types">
  <Default Extension="exe" ContentType="application/octet-stream"/>
  <Default Extension="png" ContentType="image/png"/>
  <Override PartName="/AppxManifest.xml" ContentType="application/vnd.ms-appx.manifest+xml"/>
  <Override PartName="/AppxBlockMap.xml" ContentType="application/vnd.ms-appx.blockmap+xml"/>
""" + extra_declaration + b"</Types>\n"


def make_zip(members, deflated, *, local_extras=None, central_extras=None, comments=None,
             zip64_members=False, duplicate_member=None):
    """Write fixture ZIP records with optional independently chosen member metadata."""
    local_extras = {} if local_extras is None else local_extras
    central_extras = {} if central_extras is None else central_extras
    comments = {} if comments is None else comments
    local = BytesIO()
    directory = BytesIO()
    for name, data in members.items():
        if duplicate_member is not None and name == duplicate_member[0]:
            name = duplicate_member[1]
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
        local_extra = local_extras.get(name, b"")
        local.write(struct.pack("<4s5H3I2H", b"PK\x03\x04", 20, flags, method,
                                0, 33, *local_lengths, len(encoded_name), len(local_extra)))
        local.write(encoded_name)
        local.write(local_extra)
        local.write(body)
        if compressed:
            local.write(struct.pack("<4s3I", b"PK\x07\x08", *lengths))
        crc, packed_size, unpacked_size = lengths
        central_extra = central_extras.get(name, b"")
        comment = comments.get(name, b"")
        if zip64_members:
            central_extra = struct.pack(
                "<HHQQQ", 1, 24, unpacked_size, packed_size, record_offset,
            ) + central_extra
            packed_size = unpacked_size = record_offset = 0xFFFFFFFF
        directory.write(struct.pack("<4s6H3I5H2I", b"PK\x01\x02", 20, 20, flags,
                                    method, 0, 33, crc, packed_size, unpacked_size, len(encoded_name),
                                    len(central_extra), len(comment), 0, 0, 0, record_offset))
        directory.write(encoded_name)
        directory.write(central_extra)
        directory.write(comment)
    end_record = struct.pack("<4s4H2IH", b"PK\x05\x06", 0, 0, len(members),
                             len(members), directory.tell(), local.tell(), 0)
    return local.getvalue() + directory.getvalue() + end_record


def with_zip64_end_records(classic, *, signed=False):
    """Give an ordinary fixture the fixed MakeAppx or SignTool end-record layout."""
    footer_offset = len(classic) - 22
    count, size, offset = struct.unpack_from("<HII", classic, footer_offset + 10)
    record = struct.pack("<4sQ2H2I4Q", b"PK\x06\x06", 44, 45, 45, 0, 0,
                         count, count, size, offset)
    locator = struct.pack("<4sIQI", b"PK\x06\x07", 0, footer_offset, 1)
    disk_fields = b"\0" * 4 if signed else b"\xff" * 4
    legacy = b"PK\x05\x06" + disk_fields + b"\xff" * 12 + b"\0\0"
    return classic[:footer_offset] + record + locator + legacy


def opaque_der(marker: bytes) -> bytes:
    """Create an inert length-delimited sequence without authenticated content."""
    body = marker * 256
    return b"\x30\x82" + struct.pack(">H", len(body)) + body


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
        self.write_input_set(self.payload)

    def write_input_set(self, payload):
        inputs = json.dumps({
            "payload": {name: sha256(data).hexdigest() for name, data in payload.items()},
            "sdk": {name: sha256(name.encode("ascii")).hexdigest() for name in SDK_NAMES},
        }).encode("utf-8")
        self.input_set.write_bytes(inputs)
        self.input_set_digest = sha256(inputs).hexdigest()

    def write_package(self, *, payload=None, compressed_member=None, size_case=None,
                      xml_prefix=None, types_xml=None, local_extras=None, central_extras=None,
                      comments=None, zip64_members=False):
        if payload is None:
            payload = self.payload
        sizes = {}
        compressed_members = {}
        if compressed_member is not None:
            spans, footer = deflate_blocks(payload[compressed_member])
            compressed_members[compressed_member] = b"".join(spans) + footer
            block_sizes = [len(span) for span in spans]
            if size_case == "missing":
                block_sizes[0] = None
            elif size_case == "shifted":
                # The whole stream and total declared length stay equal, but
                # the first declared span loses one byte to the second.
                block_sizes[0] -= 1
                block_sizes[1] += 1
            sizes[compressed_member] = block_sizes
        members = {
            **payload,
            BLOCK_MAP: block_map(
                payload, sizes, stored_size=size_case == "stored", local_extras=local_extras,
            ),
            CONTENT_TYPES: content_types() if types_xml is None else types_xml,
        }
        if xml_prefix is not None:
            name, prefix = xml_prefix
            members[name] = prefix + members[name]
        archive = make_zip(members, compressed_members, local_extras=local_extras,
                           central_extras=central_extras, comments=comments, zip64_members=zip64_members)
        self.package.write_bytes(with_zip64_end_records(archive) if zip64_members else archive)
        return members

    def run_reader(self, *, expected_package_digest=None, include_event_resource=False,
                   signed_fixture=False):
        if expected_package_digest is None:
            expected_package_digest = sha256(self.package.read_bytes()).hexdigest()
        command = [
            sys.executable, "-I", "-B", str(HERE / "check_package.py"),
            "--payload", str(self.payload_dir), "--package", str(self.package),
            "--input-set", str(self.input_set), "--input-set-sha256", self.input_set_digest,
            "--package-sha256", expected_package_digest,
        ]
        if include_event_resource:
            command.append("--include-event-resource")
        if signed_fixture:
            command.append("--signed-fixture")
        return subprocess.run(
            command,
            capture_output=True, text=True, check=False, timeout=10,
        )

    def assert_accepted(self, members: dict[str, bytes], *,
                        include_event_resource: bool = False) -> dict[str, object]:
        with ZipFile(self.package) as archive:
            self.assertEqual({name: archive.read(name) for name in archive.namelist()}, members)
        result: subprocess.CompletedProcess[str] = self.run_reader(
            include_event_resource=include_event_resource,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stderr, "")
        package_bytes: bytes = self.package.read_bytes()
        catalog: bytes = b"\n".join(
            b"\t".join((name.encode("ascii"), str(len(data)).encode("ascii"),
                         sha256(data).hexdigest().encode("ascii")))
            for name, data in sorted(self.payload.items())
        ) + b"\n"
        report: dict[str, object] = json.loads(result.stdout)
        self.assertEqual(report, {
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
            "payload_sha256": sha256(catalog).hexdigest(),
            "members": {
                name: {"bytes": len(data), "sha256": sha256(data).hexdigest()}
                for name, data in members.items()
            },
        })
        return report

    def assert_refused(self, reason, *, expected_package_digest=None, include_event_resource=False,
                       signed_fixture=False):
        result = self.run_reader(expected_package_digest=expected_package_digest,
                                 include_event_resource=include_event_resource,
                                 signed_fixture=signed_fixture)
        self.assertEqual(result.returncode, 1, result.stderr or "reader accepted the fixture")
        self.assertEqual(result.stdout, "")
        self.assertTrue(result.stderr.startswith("Fixture package refused: "), result.stderr)
        self.assertIn(reason, result.stderr)
        self.assertNotIn("Traceback", result.stderr)

    def test_admits_stored_package(self):
        self.assert_accepted(self.write_package())

    def test_admits_streamed_deflate_with_two_blocks(self):
        self.assert_accepted(self.write_package(compressed_member=INERT))

    def signed_members(self, payload: dict[str, bytes]) -> dict[str, bytes]:
        """Build independent stand-ins for the two SignTool footprint parts."""
        return {
            **payload,
            CONTENT_TYPES: content_types(SIGNATURE_TYPE + CATALOG_TYPE),
            CATALOG_PART: opaque_der(b"C"),
            SIGNATURE_PART: b"PKCX" + opaque_der(b"S"),
        }

    def write_signed_members(self, members: dict[str, bytes], **zip_options) -> bytes:
        """Publish private synthetic members using the SignTool ZIP64 footer."""
        package = with_zip64_end_records(make_zip(members, {}, **zip_options), signed=True)
        self.package.write_bytes(package)
        return package

    def test_signed_fixture_requires_explicit_profile_and_preserves_payload_identity(self):
        unsigned_members = self.write_package(zip64_members=True)
        unsigned_package = self.package.read_bytes()
        unsigned_report = self.assert_accepted(unsigned_members)
        self.assert_refused("ZIP64 legacy fields differ", signed_fixture=True)

        signed_members = self.signed_members(unsigned_members)
        signed_package = self.write_signed_members(signed_members)
        self.assert_refused("ZIP64 legacy fields differ")
        result = self.run_reader(signed_fixture=True)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stderr, "")
        with ZipFile(BytesIO(signed_package)) as archive:
            self.assertEqual({name: archive.read(name) for name in archive.namelist()}, signed_members)
        report = json.loads(result.stdout)
        self.assertEqual(report["status"], "signed-scmfixture-structure-checked")
        self.assertEqual(report["signature_verification"], "not-run")
        self.assertEqual(report["package_sha256"], sha256(signed_package).hexdigest())
        self.assertEqual(report["package_bytes"], len(signed_package))
        for field in ("package_identity", "input_set_sha256", "payload_sha256"):
            self.assertEqual(report[field], unsigned_report[field])
        self.assertEqual(report["members"], {
            name: {"bytes": len(data), "sha256": sha256(data).hexdigest()}
            for name, data in signed_members.items()
        })
        self.assert_refused("package SHA-256 differs", signed_fixture=True,
                            expected_package_digest=sha256(unsigned_package).hexdigest())

    def test_signed_fixture_checks_footer_and_exact_roster_after_matching_pin(self):
        members = self.signed_members(self.write_package())
        original = self.write_signed_members(members)
        self.assertEqual(self.run_reader(signed_fixture=True).returncode, 0)
        # Offset 12 in the 56-byte ZIP64 end record is the made-with word.
        version_changed = bytearray(original)
        version_changed[-98 + 12] = 44
        cases = (
            ("classic footer", make_zip(members, {}), "signed fixture requires SignTool ZIP64 footer"),
            ("ZIP64 end version", version_changed, "SignTool ZIP64 version differs"),
            ("ZIP64 central extra", with_zip64_end_records(
                make_zip(members, {}, zip64_members=True), signed=True,
            ), "central ZIP64 member fields differ"),
            ("extra member", with_zip64_end_records(
                make_zip({**members, "AppxMetadata/extra.bin": b"inert marker"}, {}), signed=True,
            ), "ZIP entry counts differ"),
            ("duplicate signature", with_zip64_end_records(
                make_zip(members, {}, duplicate_member=(CATALOG_PART, SIGNATURE_PART)), signed=True,
            ), "package inventory differs"),
        )
        for label, raw, reason in cases:
            with self.subTest(case=label):
                self.package.write_bytes(raw)
                self.assert_refused(reason, signed_fixture=True)
        self.write_signed_members(members)
        self.assertEqual(self.run_reader(signed_fixture=True).returncode, 0)

    def test_signed_fixture_checks_footprint_shapes_and_exact_types_after_matching_pin(self):
        members = self.signed_members(self.write_package())
        self.write_signed_members(members)
        self.assertEqual(self.run_reader(signed_fixture=True).returncode, 0)
        cases = (
            ("signature marker", SIGNATURE_PART, b"NOPE" + opaque_der(b"S"), SIGNATURE_PART),
            ("catalog length", CATALOG_PART, opaque_der(b"C") + b"trailer", CATALOG_PART),
            ("signature type", CONTENT_TYPES,
             content_types(SIGNATURE_TYPE.replace(b"application/vnd.ms-appx.signature", b"image/png")
                           + CATALOG_TYPE), "content type"),
            ("missing catalog type", CONTENT_TYPES, content_types(SIGNATURE_TYPE), "content type"),
            ("signature default beside valid override", CONTENT_TYPES,
             content_types(SIGNATURE_TYPE + CATALOG_TYPE
                           + b'<Default Extension="p7x" ContentType="application/octet-stream"/>'),
             "unexpected content-type default for AppxSignature.p7x"),
        )
        for label, name, replacement, reason in cases:
            with self.subTest(case=label):
                self.write_signed_members({**members, name: replacement})
                self.assert_refused(reason, signed_fixture=True)
        self.write_signed_members(members)
        self.assertEqual(self.run_reader(signed_fixture=True).returncode, 0)

    def test_payload_catalog_is_independent_of_zip_order_and_compression(self) -> None:
        stored: dict[str, object] = self.assert_accepted(self.write_package())
        reordered: dict[str, bytes] = dict(reversed(tuple(self.payload.items())))
        compressed: dict[str, object] = self.assert_accepted(
            self.write_package(payload=reordered, compressed_member=INERT),
        )

        self.assertNotEqual(stored["package_sha256"], compressed["package_sha256"])
        self.assertEqual(stored["payload_sha256"], compressed["payload_sha256"])

    def test_payload_catalog_preserves_raw_authored_manifest(self) -> None:
        original: dict[str, object] = self.assert_accepted(self.write_package())
        self.payload[MANIFEST] += b"\n"
        (self.payload_dir / MANIFEST).write_bytes(self.payload[MANIFEST])
        self.write_input_set(self.payload)
        changed: dict[str, object] = self.assert_accepted(self.write_package())

        self.assertEqual(original["package_identity"], changed["package_identity"])
        self.assertNotEqual(original["payload_sha256"], changed["payload_sha256"])

    def test_requires_the_declared_ignorable_namespace_binding(self) -> None:
        self.assert_accepted(self.write_package())
        original: bytes = self.payload[MANIFEST]
        renamed: bytes = original.replace(b"xmlns:desktop6=", b"xmlns:service=").replace(
            b"desktop6:", b"service:",
        )
        self.assertNotEqual(renamed, original)
        self.assertEqual(ET.tostring(ET.fromstring(renamed)), ET.tostring(ET.fromstring(original)))
        changed: dict[str, bytes] = {**self.payload, MANIFEST: renamed}
        (self.payload_dir / MANIFEST).write_bytes(renamed)
        self.write_input_set(changed)
        self.write_package(payload=changed)
        self.assert_refused("manifest root namespace bindings differ")

    def test_requires_element_only_package_metadata(self) -> None:
        members: dict[str, bytes] = self.write_package()
        cases: tuple[tuple[str, tuple[int, ...], str, str, str], ...] = (
            (BLOCK_MAP, (), "text", "caption", "unexpected text within BlockMap"),
            (BLOCK_MAP, (0,), "tail", "caption", "unexpected text within BlockMap"),
            (BLOCK_MAP, (0,), "text", "caption", "unexpected text within block-map File"),
            (BLOCK_MAP, (0, 0), "tail", "caption", "unexpected text within block-map File"),
            (BLOCK_MAP, (0, 0), "text", "caption", "unexpected text within block-map Block"),
            (CONTENT_TYPES, (), "text", "caption", "unexpected text within content-types document"),
            (CONTENT_TYPES, (0,), "tail", "caption", "unexpected text within content-types document"),
            (CONTENT_TYPES, (0,), "text", "\u00a0", "unexpected text within content-type declaration"),
        )
        name: str
        path: tuple[int, ...]
        attribute: str
        text: str
        reason: str
        for name, path, attribute, text, reason in cases:
            with self.subTest(name=name, path=path, attribute=attribute):
                root: ET.Element = ET.fromstring(members[name])
                node: ET.Element = root
                index: int
                for index in path:
                    node = node[index]
                setattr(node, attribute, text)
                changed: dict[str, bytes] = {**members, name: ET.tostring(root, encoding="utf-8")}
                self.package.write_bytes(make_zip(changed, {}))
                self.assert_refused(reason)

    def test_admits_xml_spacing_in_both_package_metadata_parts(self) -> None:
        members: dict[str, bytes] = self.write_package()
        name: str
        for name in (BLOCK_MAP, CONTENT_TYPES):
            root: ET.Element = ET.fromstring(members[name])
            element: ET.Element
            for element in root.iter():
                element.text = "\t\n "
                if element is not root:
                    element.tail = "\n \t"
            members[name] = ET.tostring(root, encoding="utf-8")
        self.package.write_bytes(make_zip(members, {}))
        self.assert_accepted(members)

    def test_requires_element_only_manifest_service_and_capability_containers(self) -> None:
        cases: tuple[tuple[bytes, bytes, str], ...] = (
            (b"<Extensions>", b"<Extensions>caption", "unexpected text within Extensions"),
            (b"</desktop6:Extension>", b"</desktop6:Extension>caption", "unexpected text within Extensions"),
            (b"<Capabilities>", b"<Capabilities>caption", "unexpected text within Capabilities"),
            (b'<rescap:Capability Name="runFullTrust" />',
             b'<rescap:Capability Name="runFullTrust" />caption', "unexpected text within Capabilities"),
        )
        original: bytes
        replacement: bytes
        reason: str
        for original, replacement, reason in cases:
            with self.subTest(original=original):
                changed_manifest: bytes = self.payload[MANIFEST].replace(original, replacement, 1)
                self.assertNotEqual(changed_manifest, self.payload[MANIFEST])
                changed_payload: dict[str, bytes] = {**self.payload, MANIFEST: changed_manifest}
                (self.payload_dir / MANIFEST).write_bytes(changed_manifest)
                self.write_input_set(changed_payload)
                self.write_package(payload=changed_payload)
                self.assert_refused(reason)

    def add_event_resource(self):
        data = b"R" * BLOCK_BYTES + b"event resource marker"
        self.payload[EVENT_RESOURCE] = data
        resource = self.payload_dir / EVENT_RESOURCE
        resource.write_bytes(data)
        self.write_input_set(self.payload)
        return resource

    def test_requires_explicit_matching_event_resource_profile(self):
        self.assert_accepted(self.write_package())
        self.assert_refused("frozen input set payload inventory",
                            include_event_resource=True)
        self.add_event_resource()
        types = content_types(EVENT_DEFAULT_TYPE)
        members = self.write_package(compressed_member=EVENT_RESOURCE, types_xml=types)
        self.assert_refused("frozen input set payload inventory")
        self.assert_accepted(members, include_event_resource=True)
        self.write_package(compressed_member=EVENT_RESOURCE, size_case="shifted", types_xml=types)
        self.assert_refused(f"{EVENT_RESOURCE} compressed block bytes do not match at expanded offset 0",
                            include_event_resource=True)

    def test_binds_event_resource_to_frozen_input_and_authored_bytes(self):
        resource = self.add_event_resource()
        types = content_types(EVENT_OVERRIDE_TYPE)
        members = self.write_package(types_xml=types)
        self.assert_accepted(members, include_event_resource=True)
        changed = b"different event resource"
        self.write_package(payload={**self.payload, EVENT_RESOURCE: changed}, types_xml=types)
        self.assert_refused(f"emitted package member differs: {EVENT_RESOURCE}",
                            include_event_resource=True)
        resource.write_bytes(changed)
        self.assert_refused(f"authored payload member differs from the frozen input set: {EVENT_RESOURCE}",
                            include_event_resource=True)
        resource.write_bytes(self.payload[EVENT_RESOURCE])
        self.assert_accepted(self.write_package(types_xml=types), include_event_resource=True)

    def test_requires_exact_event_resource_rosters(self):
        resource = self.add_event_resource()
        alias = EVENT_RESOURCE.upper()
        types = content_types(EVENT_DEFAULT_TYPE)
        members = self.write_package(types_xml=types)
        self.assert_accepted(members, include_event_resource=True)
        base = {name: data for name, data in self.payload.items() if name != EVENT_RESOURCE}
        aliased = {**base, alias: self.payload[EVENT_RESOURCE]}
        extra = {**self.payload, alias: self.payload[EVENT_RESOURCE]}

        cases = (
            ("missing", base, "ZIP entry counts differ"),
            ("case alias", aliased, "package inventory differs"),
            ("extra", extra, "ZIP entry counts differ"),
        )
        for case, payload, _ in cases:
            with self.subTest(input_set=case):
                self.write_input_set(payload)
                self.assert_refused("frozen input set payload inventory or digests differ",
                                    include_event_resource=True)
        self.write_input_set(self.payload)

        resource.unlink()
        self.assert_refused("authored payload root inventory differs",
                            include_event_resource=True)
        (self.payload_dir / alias).write_bytes(self.payload[EVENT_RESOURCE])
        self.assert_refused("authored payload root inventory differs",
                            include_event_resource=True)
        resource.write_bytes(self.payload[EVENT_RESOURCE])
        self.assert_refused("authored payload root inventory differs",
                            include_event_resource=True)
        (self.payload_dir / alias).unlink()

        for case, payload, reason in cases:
            with self.subTest(archive=case):
                self.write_package(payload=payload, types_xml=types)
                self.assert_refused(reason, include_event_resource=True)
        self.assert_accepted(self.write_package(types_xml=types), include_event_resource=True)

    def test_checks_event_resource_block_map_and_content_type(self):
        self.add_event_resource()
        types = content_types(EVENT_DEFAULT_TYPE)
        members = self.write_package(types_xml=types)
        self.assert_accepted(members, include_event_resource=True)
        block_map_data = members[BLOCK_MAP]
        base_payload = {name: data for name, data in self.payload.items() if name != EVENT_RESOURCE}
        resource_hash = b64encode(sha256(self.payload[EVENT_RESOURCE][:BLOCK_BYTES]).digest())
        wrong_override = EVENT_OVERRIDE_TYPE.replace(b"application/octet-stream", b"image/png")
        cases = (
            (BLOCK_MAP, block_map(base_payload, {}), "block-map file count differs"),
            (BLOCK_MAP, block_map_data.replace(EVENT_RESOURCE.encode(), EVENT_RESOURCE.upper().encode()),
             "block-map file inventory differs"),
            (BLOCK_MAP, block_map_data.replace(resource_hash, b64encode(bytes(32))),
             f"block-map hash differs for {EVENT_RESOURCE}"),
            (CONTENT_TYPES, content_types(), f"content type differs or is missing for {EVENT_RESOURCE}"),
            (CONTENT_TYPES, content_types(EVENT_DEFAULT_TYPE + wrong_override),
             f"content type differs or is missing for {EVENT_RESOURCE}"),
        )
        for name, data, reason in cases:
            with self.subTest(reason=reason):
                self.package.write_bytes(make_zip({**members, name: data}, {}))
                self.assert_refused(reason, include_event_resource=True)
        self.assert_accepted(self.write_package(types_xml=types), include_event_resource=True)

    def test_rejects_inconsistent_classic_zip_end_record(self):
        members = self.write_package()
        original = self.package.read_bytes()
        self.assert_accepted(members)
        footer = len(original) - 22
        for label, offset, reason in (
            ("disk number", 4, "ZIP disk fields differ"),
            ("directory disk", 6, "ZIP disk fields differ"),
            ("disk entry count", 8, "ZIP entry counts differ"),
            ("total entry count", 10, "ZIP entry counts differ"),
            ("directory size", 12, "ZIP central directory extent differs"),
            ("directory offset", 16, "ZIP central directory extent differs"),
        ):
            with self.subTest(field=label):
                changed = bytearray(original)
                changed[footer + offset] ^= 1
                self.package.write_bytes(changed)
                self.assert_refused(reason)
        self.package.write_bytes(original)
        self.assert_accepted(members)

    def test_rejects_inconsistent_zip64_end_records(self):
        members = self.write_package(compressed_member=INERT)
        original = with_zip64_end_records(self.package.read_bytes())
        self.package.write_bytes(original)
        self.assert_accepted(members)
        record = len(original) - 98
        for label, offset, reason in (
            ("record size", 4, "ZIP64 end record differs"),
            ("disk number", 16, "ZIP disk fields differ"),
            ("directory disk", 20, "ZIP disk fields differ"),
            ("disk entry count", 24, "ZIP entry counts differ"),
            ("total entry count", 32, "ZIP entry counts differ"),
            ("directory size", 40, "ZIP central directory extent differs"),
            ("directory offset", 48, "ZIP central directory extent differs"),
            ("locator disk", 60, "ZIP64 locator differs"),
            ("locator offset", 64, "ZIP64 locator differs"),
            ("locator disk count", 72, "ZIP64 locator differs"),
            ("legacy placeholder", 86, "ZIP64 legacy fields differ"),
        ):
            with self.subTest(field=label):
                changed = bytearray(original)
                changed[record + offset] ^= 1
                self.package.write_bytes(changed)
                self.assert_refused(reason)
        self.package.write_bytes(original)
        self.assert_accepted(members)

    def test_member_metadata_has_a_closed_classic_profile(self):
        healthy = self.write_package(compressed_member=INERT)
        self.assert_accepted(healthy)
        note = struct.pack("<HH", 0xA001, 4) + b"note"
        for options, reason in (
            ({"local_extras": {INERT: note}}, "local ZIP extra fields differ"),
            ({"central_extras": {INERT: note}}, "central ZIP extra fields differ"),
            ({"comments": {INERT: b"note"}}, "central ZIP member comment is not admitted"),
        ):
            with self.subTest(options=options):
                changed = self.write_package(compressed_member=INERT, **options)
                with ZipFile(self.package) as archive:
                    self.assertEqual({name: archive.read(name) for name in archive.namelist()}, changed)
                self.assert_refused(reason)
        self.assert_accepted(self.write_package(compressed_member=INERT))

    def test_member_metadata_has_a_closed_zip64_profile(self):
        healthy = self.write_package(compressed_member=INERT, zip64_members=True)
        original = self.package.read_bytes()
        self.assert_accepted(healthy)
        # Find the selected central entry from the already decoded footer span.
        end_record = len(original) - 98
        position, = struct.unpack_from("<Q", original, end_record + 48)
        name_size, = struct.unpack_from("<H", original, position + 28)
        extra = position + 46 + name_size
        for label, offset in (
            ("partial placeholder", position + 20),
            ("unexpected ZIP64 tag", extra),
            ("incomplete ZIP64 record", extra + 2),
        ):
            with self.subTest(field=label):
                changed = bytearray(original)
                changed[offset] ^= 1
                self.package.write_bytes(changed)
                self.assert_refused("central ZIP64 member fields differ")
        self.package.write_bytes(original)
        self.assert_accepted(healthy)

    def test_rejects_central_record_extending_past_the_directory(self):
        members = self.write_package()
        original = self.package.read_bytes()
        self.assert_accepted(members)
        last_header = original.rindex(b"PK\x01\x02", 0, len(original) - 22)
        changed = bytearray(original)
        # A one-byte comment is declared, but the record ends with the filename.
        changed[last_header + 32] = 1
        self.package.write_bytes(changed)
        self.assert_refused("ZIP central directory extent differs")
        self.package.write_bytes(original)
        self.assert_accepted(members)

    def test_rejects_central_entry_on_another_disk(self):
        members = self.write_package()
        original = self.package.read_bytes()
        self.assert_accepted(members)
        directory_start, = struct.unpack_from("<I", original, len(original) - 6)
        changed = bytearray(original)
        changed[directory_start + 34] = 1
        self.package.write_bytes(changed)
        self.assert_refused("ZIP disk fields differ")
        self.package.write_bytes(original)
        self.assert_accepted(members)

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
            self.write_package(compressed_member=INERT if compressed else None)
            original = self.package.read_bytes()
            with ZipFile(self.package) as archive:
                local_header = archive.getinfo(INERT).header_offset
            for field, offset in (("CRC", 14), ("compressed size", 18), ("file size", 22)):
                with self.subTest(streamed=compressed, field=field):
                    changed = bytearray(original)
                    changed[local_header + offset] ^= 1
                    self.package.write_bytes(changed)
                    self.assert_refused(f"{INERT} local ZIP CRC or lengths differ")

    def test_rejects_local_extract_version_mismatch(self) -> None:
        for compressed in (False, True):
            members = self.write_package(compressed_member=INERT if compressed else None)
            self.assert_accepted(members)
            original: bytes = self.package.read_bytes()
            with ZipFile(self.package) as archive:
                local_header: int = archive.getinfo(INERT).header_offset
            for offset in (4, 5):
                with self.subTest(streamed=compressed, version_byte=offset):
                    changed = bytearray(original)
                    changed[local_header + offset] ^= 1
                    self.package.write_bytes(changed)
                    self.assert_refused(f"{INERT} local ZIP extract version differs")
            self.package.write_bytes(original)
            self.assert_accepted(members)

    def test_unsupported_central_extract_version_refuses_cleanly(self) -> None:
        members = self.write_package()
        self.assert_accepted(members)
        original: bytes = self.package.read_bytes()
        directory_start, = struct.unpack_from("<I", original, len(original) - 6)
        changed = bytearray(original)
        changed[directory_start + 6] = 255
        self.package.write_bytes(changed)
        self.assert_refused("unsupported ZIP extract version")

    def test_rejects_missing_and_false_compressed_block_sizes(self):
        for size_case, reason in (
            ("missing", f"{INERT} requires a compressed block Size"),
            ("shifted", f"{INERT} compressed block bytes do not match at expanded offset 0"),
            ("stored", f"{LOGO} has a Size on a stored block"),
        ):
            with self.subTest(size_case=size_case):
                self.write_package(compressed_member=INERT, size_case=size_case)
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
