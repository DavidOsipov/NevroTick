# SPDX-License-Identifier: GPL-2.0-only
"""Read back the single unsigned ArtworkLab 1.0.1.0 combined lab profile."""

import argparse
from hashlib import sha256
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
PNG_SIGNATURE: bytes = b"\x89PNG\r\n\x1a\n"
PRI_SIGNATURE: bytes = b"mrm_pri2"
FOOTPRINT_NS: dict[str, str] = {
    **reader.MANIFEST_NAMESPACES,
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
    ignorable: list[str] = root.get("IgnorableNamespaces", "").split()
    reader.require(reader.root_namespace_bindings(raw) == FOOTPRINT_NS, "combined manifest namespace bindings differ")
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


def payload_signing_inventory(members: dict[str, reader.PackageMember]) -> dict[str, dict[str, str]]:
    """Derive signing obligations for the pinned payload and checked XML manifest."""
    inventory: dict[str, dict[str, str]] = {}
    for name in sorted(PAYLOAD_NAMES):
        raw: bytes = members[name].data
        signature_requirement: str = "package-signature"
        artifact_format: str
        if name == UI or name in ROLES:
            check_pe(name, raw)
            artifact_format = "pe32+"
            signature_requirement = "embedded-authenticode"
        elif name in ART:
            reader.require(raw.startswith(PNG_SIGNATURE), f"{name} PNG signature differs")
            artifact_format = "png"
        elif name == PRI:
            reader.require(raw.startswith(PRI_SIGNATURE), f"{name} PRI signature differs")
            artifact_format = "pri"
        elif name == MANIFEST:
            artifact_format = "xml"
        else:
            raise reader.Refused("no signing policy for an authored member")
        inventory[name] = {"format": artifact_format, "signature_requirement": signature_requirement}
    return inventory


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


def guarded_startup_inventory(
    members: dict[str, reader.PackageMember],
    ui_receipt: Path,
    ui_receipt_pin: str,
    ui_source_pin: str,
    ui_map: Path,
    role_receipt: Path,
    role_receipt_pin: str,
) -> dict[str, object]:
    """Join original guarded link and UI resource evidence to the checked members."""
    # The original unsigned reader is also used as a standalone two-file CLI.
    import prepare_payload as stage
    from metadata.pipeline.delivery import _equal
    from prepare_artwork_payload import UiBuildInputs, ui_build_inputs
    from windows_pe import service_preentry_declarations, ui_preentry_declarations, ui_resource_payloads
    from windows_versioninfo import UiExecutionManifest, VersionInfo, decode_ui_resource_payloads

    raw: bytes = reader.read_regular(ui_receipt, 8 * 1024**2, "guarded UI link receipt")
    reader.require(sha256(raw).hexdigest() == ui_receipt_pin, "guarded UI link receipt SHA-256 differs")
    report: object = json.loads(raw.decode("utf-8"), object_pairs_hook=reader.unique_object)
    if type(report) is not dict:
        raise reader.Refused("guarded UI link receipt must be an object")
    artifact: object = report.get("artifact")
    entry: object = report.get("ui_preentry")
    image: bytes = members[UI].data
    image_pin: str = sha256(image).hexdigest()
    invalid_ui: str = "guarded UI link receipt differs from the archived UI"
    if (type(artifact) is not dict or type(entry) is not dict
            or report.get("status") != "passed"
            or report.get("native_windows_execution") != "not performed"
            or artifact.get("component_role") != "ui" or artifact.get("source_state") != "fixture"
            or artifact.get("profile") != "release" or artifact.get("target") != "x86_64-pc-windows-msvc"
            or artifact.get("sha256") != image_pin or entry.get("image_sha256") != image_pin
            or not reader.is_sha256(entry.get("object_sha256"))
            or not reader.is_sha256(entry.get("map_sha256"))):
        raise reader.Refused(invalid_ui)
    map_data: bytes = reader.read_regular(ui_map, 32 * 1024**2, "guarded UI link map")
    reader.require(sha256(map_data).hexdigest() == entry["map_sha256"], "guarded UI link map SHA-256 differs")
    try:
        observed_ui: dict[str, object] = ui_preentry_declarations(
            image, map_data.decode("utf-8"), object_sha256=entry["object_sha256"],
        )
    except ValueError as error:
        raise reader.Refused("archived UI failed guarded PE/map readback") from error
    reader.require(all(type(entry.get(key)) is type(value) and entry[key] == value
                       for key, value in observed_ui.items()),
                   "guarded UI startup facts differ from the archived PE/map")
    try:
        version: VersionInfo
        manifest: UiExecutionManifest
        version, manifest = decode_ui_resource_payloads(*ui_resource_payloads(image))
    except ValueError as error:
        raise reader.Refused("archived UI failed VERSIONINFO and execution manifest readback") from error
    claimed_resources: object = report.get("ui_versioninfo")
    observed_resources: dict[str, object] = {
        "image_sha256": image_pin, "fixed": list(version["fixed"]),
        "strings": version["strings"], "translation": list(version["translation"]),
        "execution_manifest": manifest,
    }
    reader.require(type(claimed_resources) is dict
                   and all(_equal(claimed_resources.get(key), value)
                           for key, value in observed_resources.items()),
                   "guarded UI resource facts differ from the archived PE")
    observed: dict[str, dict[str, object]] = {
        UI: {"sha256": image_pin, "map_sha256": entry["map_sha256"], **observed_ui},
    }

    try:
        ui_build: UiBuildInputs = ui_build_inputs(report)
    except ValueError as error:
        raise reader.Refused("guarded UI build input evidence refused") from error
    source_map: bytes = json.dumps(
        ui_build.source_sha256, sort_keys=True, separators=(",", ":"), ensure_ascii=True,
    ).encode("ascii")
    reader.require(sha256(source_map).hexdigest() == ui_source_pin,
                   "guarded UI source map SHA-256 differs")
    try:
        roles: dict[str, bytes] = stage.role_images(
            role_receipt.absolute(), role_receipt_pin, require_baseline_entry=True,
            matching_sources=ui_build.service_matching_sources(),
        )
    except (KeyError, ValueError) as error:
        raise reader.Refused("guarded service link evidence refused") from error
    reader.require(roles.keys() == ROLES.keys(), "guarded service role inventory differs")
    for name in sorted(ROLES):
        archived: bytes = members[name].data
        reader.require(roles[name] == archived, f"archived service differs from guarded link: {name}")
        # role_images has already bound these exact bytes to the role's original
        # PDB, linker map, gate object and the producer's entry/CRT facts.
        observed[name] = {"sha256": sha256(archived).hexdigest(),
                          **service_preentry_declarations(archived)}
    return {"scope": "static-guarded-archive-member-declarations",
            "ui_link_receipt_sha256": ui_receipt_pin, "service_link_receipt_sha256": role_receipt_pin,
            "members": observed}


def main() -> int:
    cli: argparse.ArgumentParser = argparse.ArgumentParser(description=__doc__)
    cli.add_argument("--payload", required=True, type=Path)
    cli.add_argument("--package", required=True, type=Path)
    cli.add_argument("--package-sha256", required=True)
    cli.add_argument("--pins", required=True, type=Path, help="flat source-name/SHA-256 JSON map")
    cli.add_argument("--pins-sha256", required=True)
    cli.add_argument("--manifest-sha256", required=True, help="digest from the independently held authored manifest")
    cli.add_argument("--guarded-ui-receipt", type=Path)
    cli.add_argument("--guarded-ui-sha256")
    cli.add_argument("--guarded-ui-source-sha256",
                     help="independently retained SHA-256 of the original UI source_sha256 map as sorted compact ASCII JSON")
    cli.add_argument("--guarded-ui-link-map", type=Path)
    cli.add_argument("--guarded-role-receipt", type=Path)
    cli.add_argument("--guarded-role-sha256")
    args: argparse.Namespace = cli.parse_args()
    try:
        reader.require(all(reader.is_sha256(value) for value in
                           (args.package_sha256, args.pins_sha256, args.manifest_sha256)),
                       "external SHA-256 digests must be lowercase")
        guarded: tuple[Path | str | None, ...] = (
            args.guarded_ui_receipt, args.guarded_ui_sha256, args.guarded_ui_link_map,
            args.guarded_role_receipt, args.guarded_role_sha256, args.guarded_ui_source_sha256,
        )
        check_guards: bool = any(value is not None for value in guarded)
        if check_guards:
            reader.require(all(value is not None for value in guarded)
                           and reader.is_sha256(args.guarded_ui_sha256)
                           and reader.is_sha256(args.guarded_ui_source_sha256)
                           and reader.is_sha256(args.guarded_role_sha256),
                           "guarded startup requires both pinned link receipts, the original UI map and the UI source digest")
        pins: dict[str, str] = read_pins(args.pins, args.pins_sha256)
        pins[MANIFEST] = args.manifest_sha256
        package: bytes = reader.read_regular(args.package, MAX_ARCHIVE, "package")
        package_sha: str = sha256(package).hexdigest()
        reader.require(package_sha == args.package_sha256, "package SHA-256 differs")
        payload: dict[str, bytes] = reader.read_payload(args.payload, PAYLOAD_NAMES, member_limit=member_limit)
        members: dict[str, reader.PackageMember] = reader.read_archive(
            package, ARCHIVE_NAMES, maximum_archive_bytes=MAX_ARCHIVE, maximum_expanded_bytes=MAX_EXPANDED,
            member_limit=member_limit,
        )
        for name in sorted(PAYLOAD_NAMES):
            emitted: bytes = members[name].data
            reader.require(emitted == payload[name], f"archive/authored member differs: {name}")
            reader.require(sha256(emitted).hexdigest() == pins[name],
                           f"member differs from independent source pin: {name}")
        check_manifest(members[MANIFEST].data)
        signing: dict[str, dict[str, str]] = payload_signing_inventory(members)
        reader.check_block_map(members, PAYLOAD_NAMES)
        check_content_types(members[reader.CONTENT_TYPES_FILE].data)
        startup: dict[str, object] | None = guarded_startup_inventory(
            members, args.guarded_ui_receipt, args.guarded_ui_sha256, args.guarded_ui_source_sha256,
            args.guarded_ui_link_map,
            args.guarded_role_receipt, args.guarded_role_sha256,
        ) if check_guards else None
    except reader.Refused as error:
        cli.exit(1, f"Combined fixture package refused: {error}\n")
    except (OSError, zipfile.BadZipFile, zlib.error, EOFError, ValueError, RecursionError) as error:
        cli.exit(1, f"Combined fixture package refused: unreadable input ({type(error).__name__})\n")

    payload_catalog: bytes = "".join(
        f"{name}\t{len(members[name].data)}\t{pins[name]}\n" for name in sorted(PAYLOAD_NAMES)
    ).encode("ascii")
    report: dict[str, object] = {
        "status": "unsigned-combined-artworklab-checked",
        "identity": IDENTITY,
        "package_bytes": len(package), "package_sha256": package_sha,
        "pins_sha256": args.pins_sha256,
        "payload_sha256": sha256(payload_catalog).hexdigest(),
        "payload_count": len(PAYLOAD_NAMES), "artwork_count": len(ART),
        "members": {name: {"bytes": len(member.data), "sha256": sha256(member.data).hexdigest()}
                    for name, member in sorted(members.items())},
        "signing_inventory": {
            "verification": "not-run",
            "package": {"format": "msix", "signature_requirement": "package-signature"},
            "payload": signing,
        },
    }
    if startup is not None:
        report["startup_inventory"] = startup
    print(json.dumps(report, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
