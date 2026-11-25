# SPDX-License-Identifier: GPL-2.0-only
"""Read back the single unsigned ArtworkLab 1.0.1.0 combined lab profile."""

import argparse
from hashlib import sha256
from io import BytesIO
import json
from pathlib import Path
import struct
import sys
import xml.etree.ElementTree as ET
import zipfile
import zlib

sys.dont_write_bytecode = True
sys.path.insert(0, str(Path(__file__).resolve().parent))
import check_package as reader


UI: str = "nevrotick-ui.exe"
PRI: str = "resources.pri"
MANIFEST: str = reader.MANIFEST
ROLES: dict[str, str] = reader.SERVICES
BASE_ART: set[str] = {
    f"Assets/{family}{qualifier}.png"
    for family in ("PackageLogo", "Square150x150Logo", "Square44x44Logo")
    for qualifier in ("", ".scale-200", ".scale-400")
}
TARGET_ART: set[str] = {
    f"Assets/Square44x44Logo.targetsize-{size}{qualifier}.png"
    for size in (16, 20, 24, 30, 32, 36, 40, 48, 60, 64, 72, 80, 96, 256)
    for qualifier in ("", "_altform-unplated", "_altform-lightunplated")
}
ART: set[str] = BASE_ART | TARGET_ART
PINNED_NAMES: set[str] = ART | {PRI, UI, *ROLES}
PAYLOAD_NAMES: set[str] = PINNED_NAMES | {MANIFEST}
ARCHIVE_NAMES: set[str] = PAYLOAD_NAMES | {reader.BLOCK_MAP_FILE, reader.CONTENT_TYPES_FILE}
MAX_ARCHIVE: int = 32 * 1024 * 1024
MAX_EXPANDED: int = 32 * 1024 * 1024
MAX_UI: int = 24 * 1024 * 1024
FOOTPRINT_NS: dict[str, str] = {
    "": reader.FOUNDATION[1:-1],
    "uap": reader.UAP[1:-1],
    "uap10": reader.UAP10[1:-1],
    "desktop6": reader.DESKTOP6[1:-1],
    "rescap": reader.RESCAP[1:-1],
    "trustedlaunch": "http://schemas.microsoft.com/appx/manifest/trustedlaunch/windows10",
}
TRUSTED: str = "{" + FOOTPRINT_NS["trustedlaunch"] + "}"
IDENTITY: dict[str, str] = {
    "Name": "NevroTick.S06.ArtworkLab",
    "Publisher": "CN=NevroTick S06 Laboratory",
    "Version": "1.0.1.0",
    "ProcessorArchitecture": "x64",
}


def member_limit(name: str) -> int:
    if name == UI:
        return MAX_UI
    if name in ART:
        return 256 * 1024
    if name == PRI:
        return 64 * 1024
    return reader.fixture_member_limit(name)


def read_pins(path: Path, pin_sha: str) -> dict[str, str]:
    raw: bytes = reader.read_regular(path, reader.MAX_INPUT_SET, "source hash set")
    reader.require(sha256(raw).hexdigest() == pin_sha, "source hash set SHA-256 differs")
    pins: object = json.loads(raw.decode("utf-8"), object_pairs_hook=reader.unique_object)
    invalid: str = "source hash set inventory or digest syntax differs"
    if type(pins) is not dict:
        raise reader.Refused(invalid)
    reader.require(set(pins) == PINNED_NAMES and all(reader.is_sha256(value) for value in pins.values()), invalid)
    reader.require(len({pins[name] for name in {UI, *ROLES}}) == 3, "source role hashes alias")
    return pins


def exact_container(node: ET.Element, children: set[str], label: str) -> None:
    reader.require(not node.attrib, f"{label} attributes differ")
    reader.exact_children(node, children, label)


def exact_text(node: ET.Element, value: str, label: str) -> None:
    reader.require(not node.attrib and len(node) == 0 and node.text == value,
                   f"{label} differs")


def check_manifest(raw: bytes) -> None:
    root: ET.Element = reader.parse_xml(raw, "combined manifest")
    namespace_bindings: dict[str, str] = {}
    for event, value in ET.iterparse(BytesIO(raw), events=("start-ns", "start")):
        if event == "start":
            break
        prefix, uri = value
        namespace_bindings[prefix] = uri
    ignorable: list[str] = root.get("IgnorableNamespaces", "").split()
    reader.require(namespace_bindings == FOOTPRINT_NS, "combined manifest namespace bindings differ")
    foundation: str = reader.FOUNDATION
    uap: str = reader.UAP
    uap10: str = reader.UAP10
    desktop6: str = reader.DESKTOP6
    rescap: str = reader.RESCAP
    reader.require(root.tag == foundation + "Package" and set(root.attrib) == {"IgnorableNamespaces"}
                   and sorted(ignorable) == sorted(set(FOOTPRINT_NS) - {""}),
                   "combined manifest Package attributes differ")
    reader.exact_children(root, {foundation + name for name in (
        "Identity", "Properties", "Dependencies", "Resources", "Applications", "Capabilities"
    )}, "Package")
    reader.exact_leaf(reader.singleton(root, foundation + "Identity"), IDENTITY, "Identity")

    properties: ET.Element = reader.singleton(root, foundation + "Properties")
    expected_text: dict[str, str] = {
        foundation + "DisplayName": "NevroTick",
        foundation + "PublisherDisplayName": "David Osipov",
        foundation + "Description": "A Windows time synchronization client under development.",
        foundation + "Logo": r"Assets\PackageLogo.png",
        TRUSTED + "TrustedLaunch": "true",
    }
    integrity_tag: str = uap10 + "PackageIntegrity"
    exact_container(properties, set(expected_text) | {integrity_tag}, "Properties")
    for tag, value in expected_text.items():
        exact_text(reader.singleton(properties, tag), value, tag.rsplit("}", 1)[-1])
    integrity: ET.Element = reader.singleton(properties, integrity_tag)
    exact_container(integrity, {uap10 + "Content"}, "PackageIntegrity")
    reader.exact_leaf(reader.singleton(integrity, uap10 + "Content"),
                      {"Enforcement": "on"}, "PackageIntegrity Content")

    dependencies: ET.Element = reader.singleton(root, foundation + "Dependencies")
    exact_container(dependencies, {foundation + "TargetDeviceFamily"}, "Dependencies")
    reader.exact_leaf(reader.singleton(dependencies, foundation + "TargetDeviceFamily"), {
        "Name": "Windows.Desktop", "MinVersion": "10.0.26100.0", "MaxVersionTested": "10.0.26100.0",
    }, "TargetDeviceFamily")
    resources: ET.Element = reader.singleton(root, foundation + "Resources")
    exact_container(resources, {foundation + "Resource"}, "Resources")
    reader.exact_leaf(reader.singleton(resources, foundation + "Resource"), {"Language": "en-US"}, "Resource")

    applications: ET.Element = reader.singleton(root, foundation + "Applications")
    exact_container(applications, {foundation + "Application"}, "Applications")
    app: ET.Element = reader.singleton(applications, foundation + "Application")
    reader.require(app.attrib == {
        "Id": "Observe", "Executable": UI,
        uap10 + "RuntimeBehavior": "packagedClassicApp", uap10 + "TrustLevel": "mediumIL",
    }, "Observe application profile differs")
    reader.exact_children(app, {uap + "VisualElements", foundation + "Extensions"}, "Application")
    reader.exact_leaf(reader.singleton(app, uap + "VisualElements"), {
        "DisplayName": "NevroTick artwork laboratory", "Description": "Offline resource assembly fixture",
        "BackgroundColor": "transparent", "AppListEntry": "none",
        "Square44x44Logo": r"Assets\Square44x44Logo.png",
        "Square150x150Logo": r"Assets\Square150x150Logo.png",
    }, "Observe VisualElements")
    extensions: ET.Element = reader.singleton(app, foundation + "Extensions")
    reader.require(not extensions.attrib and len(extensions) == len(ROLES)
                   and {entry.get("Executable") for entry in extensions} == set(ROLES),
                   "service extension inventory differs")
    reader.require_xml_spacing(extensions, "Extensions")
    for entry in extensions:
        executable: str = entry.attrib["Executable"]
        reader.require(entry.tag == desktop6 + "Extension" and entry.attrib == {
            "Category": "windows.service", "Executable": executable,
            "EntryPoint": "Windows.FullTrustApplication",
        }, f"{executable} extension profile differs")
        reader.exact_children(entry, {desktop6 + "Service"}, f"{executable} Extension")
        reader.exact_leaf(reader.singleton(entry, desktop6 + "Service"), {
            "Name": ROLES[executable], "StartupType": "disabled", "StartAccount": "localService",
        }, f"{executable} service profile")

    capabilities: ET.Element = reader.singleton(root, foundation + "Capabilities")
    capability_names: set[str] = {"runFullTrust", "packagedServices"}
    reader.require(not capabilities.attrib and len(capabilities) == len(capability_names)
                   and {entry.get("Name") for entry in capabilities} == capability_names,
                   "capability inventory differs")
    reader.require_xml_spacing(capabilities, "Capabilities")
    for entry in capabilities:
        reader.require(entry.tag == rescap + "Capability", "capability namespace differs")
        reader.exact_leaf(entry, {"Name": entry.get("Name")}, "Capability")


def check_pe(name: str, raw: bytes) -> None:
    """Check the architecture declaration of the already pinned image bytes."""
    reader.require(len(raw) >= 64 and raw[:2] == b"MZ", f"{name} DOS header differs")
    offset: int = struct.unpack_from("<I", raw, 60)[0]
    reader.require(64 <= offset <= len(raw) - 26 and raw[offset:offset + 4] == b"PE\0\0",
                   f"{name} PE header differs")
    machine: int = struct.unpack_from("<H", raw, offset + 4)[0]
    magic: int = struct.unpack_from("<H", raw, offset + 24)[0]
    reader.require(machine == 0x8664 and magic == 0x20B, f"{name} is not AMD64 PE32+")


def check_content_types(raw: bytes) -> None:
    tag: str = reader.CONTENT_TYPES
    node: ET.Element = reader.parse_xml(raw, reader.CONTENT_TYPES_FILE)
    reader.require(node.tag == tag + "Types" and not node.attrib,
                   "content-types root differs")
    reader.require_xml_spacing(node, "content types")
    expected: dict[tuple[str, str], str] = {
        ("Extension", "png"): "image/png",
        ("Extension", "exe"): "application/x-msdownload",
        ("Extension", "pri"): "application/octet-stream",
        ("Extension", "xml"): "application/vnd.ms-appx.manifest+xml",
        ("PartName", "/AppxBlockMap.xml"): "application/vnd.ms-appx.blockmap+xml",
    }
    for entry in node:
        key: str = "Extension" if entry.tag == tag + "Default" else "PartName"
        reader.require(entry.tag in {tag + "Default", tag + "Override"},
                       "unknown content-type declaration")
        value: str = entry.get(key, "")
        media_type: str | None = expected.pop((key, value), None)
        reader.require(media_type is not None, "unexpected or repeated content-type declaration")
        reader.exact_leaf(entry, {key: value, "ContentType": media_type}, "content type")
    reader.require(not expected, "content-type inventory is incomplete")


def main() -> int:
    cli: argparse.ArgumentParser = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("--payload", required=True, type=Path)
    cli.add_argument("--package", required=True, type=Path)
    cli.add_argument("--package-sha256", required=True)
    cli.add_argument("--pins", required=True, type=Path, help="flat source-name/SHA-256 JSON map")
    cli.add_argument("--pins-sha256", required=True)
    cli.add_argument("--manifest-sha256", required=True, help="digest from the independently held authored manifest")
    args: argparse.Namespace = cli.parse_args()
    try:
        reader.require(all(reader.is_sha256(value) for value in
                           (args.package_sha256, args.pins_sha256, args.manifest_sha256)),
                       "external SHA-256 digests must be lowercase")
        pins: dict[str, str] = read_pins(args.pins, args.pins_sha256)
        pins[MANIFEST] = args.manifest_sha256
        package: bytes = reader.read_regular(args.package, MAX_ARCHIVE, "package")
        package_sha: str = sha256(package).hexdigest()
        reader.require(package_sha == args.package_sha256, "package SHA-256 differs")
        payload: dict[str, bytes] = reader.read_payload(args.payload, PAYLOAD_NAMES, member_limit=member_limit)
        try:
            members: dict[str, reader.PackageMember] = reader.read_archive(
                package, ARCHIVE_NAMES, maximum_archive_bytes=MAX_ARCHIVE, maximum_expanded_bytes=MAX_EXPANDED,
                member_limit=member_limit,
            )
        except NotImplementedError as error:
            raise reader.Refused("unsupported ZIP extract version") from error
        for name in sorted(PAYLOAD_NAMES):
            emitted: bytes = members[name].data
            reader.require(emitted == payload[name], f"archive/authored member differs: {name}")
            reader.require(sha256(emitted).hexdigest() == pins[name],
                           f"member differs from independent source pin: {name}")
        for name in (UI, *ROLES):
            check_pe(name, members[name].data)
        check_manifest(members[MANIFEST].data)
        reader.check_block_map(members, PAYLOAD_NAMES)
        check_content_types(members[reader.CONTENT_TYPES_FILE].data)
    except reader.Refused as error:
        cli.exit(1, f"Combined fixture package refused: {error}\n")
    except (OSError, zipfile.BadZipFile, zlib.error, EOFError, ValueError, RecursionError) as error:
        cli.exit(1, f"Combined fixture package refused: unreadable input ({type(error).__name__})\n")

    payload_catalog: bytes = "".join(
        f"{name}\t{len(members[name].data)}\t{pins[name]}\n" for name in sorted(PAYLOAD_NAMES)
    ).encode("ascii")
    print(json.dumps({
        "status": "unsigned-combined-artworklab-checked",
        "identity": IDENTITY,
        "package_bytes": len(package), "package_sha256": package_sha,
        "pins_sha256": args.pins_sha256,
        "payload_sha256": sha256(payload_catalog).hexdigest(),
        "payload_count": len(PAYLOAD_NAMES), "artwork_count": len(ART),
        "members": {name: {"bytes": len(member.data), "sha256": sha256(member.data).hexdigest()}
                    for name, member in sorted(members.items())},
    }, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
