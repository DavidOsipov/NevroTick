# SPDX-License-Identifier: GPL-2.0-only
"""Stage the fixed unsigned ArtworkLab payload from independently pinned inputs."""

import argparse
import json
from pathlib import Path
import sys
from typing import Any

sys.path.insert(0, str(Path(__file__).resolve().parent))
import prepare_payload as stage
import verify
from windows_pe import MAX_PE_BYTES, read_export_pair, ui_preentry_declarations, ui_resource_payloads

IDENTITY: str = "NevroTick.S06.ArtworkLab"
UI_IMAGE: str = "nevrotick-ui.exe"
ARTWORK_PREFIX: str = "msix-inputs/"
PROPERTIES: str = "msix-authoring/authoring/msix-properties.xml"
GROUPS: dict[str, int] = {"PackageLogo.png": 3, "Square150x150Logo.png": 3, "Square44x44Logo.png": 45}
# Reviewed build of inert.c retained in the unsigned MakeAppx fixture evidence.
INERT_SHA256: str = "466e48e377946eed00c03469c8a0e24d51e1774a391b221b26136cefc26abe98"
# The disabled-service authoring owns the exact extension block.
ROLE_MANIFEST_SHA256: str = "76f52cde2c3fc40b39a2150f3529df0bcab76d99886293fef81e71e33881b41c"
XML_DECLARATION: bytes = b'<?xml version="1.0" encoding="utf-8"?>\n'
TRUSTED_LAUNCH_NS: bytes = b' xmlns:trustedlaunch="http://schemas.microsoft.com/appx/manifest/trustedlaunch/windows10"'
MANIFEST_TOP: bytes = (
    XML_DECLARATION
    + b'<Package xmlns="http://schemas.microsoft.com/appx/manifest/foundation/windows10"'
    b' xmlns:uap="http://schemas.microsoft.com/appx/manifest/uap/windows10"'
    b' xmlns:uap10="http://schemas.microsoft.com/appx/manifest/uap/windows10/10"'
    b' xmlns:rescap="http://schemas.microsoft.com/appx/manifest/foundation/windows10/restrictedcapabilities"'
    + TRUSTED_LAUNCH_NS
    + b' IgnorableNamespaces="uap uap10 rescap trustedlaunch">\n'
    + b'  <Identity Name="NevroTick.S06.ArtworkLab" Publisher="CN=NevroTick S06 Laboratory"'
    b' Version="1.0.0.0" ProcessorArchitecture="x64" />\n'
)
MANIFEST_BOTTOM: bytes = br'''  <Dependencies><TargetDeviceFamily Name="Windows.Desktop" MinVersion="10.0.26100.0" MaxVersionTested="10.0.26100.0" /></Dependencies>
  <Resources><Resource Language="en-US" /></Resources>
  <Applications>
    <Application Id="Inert" Executable="Inert.exe" uap10:RuntimeBehavior="packagedClassicApp" uap10:TrustLevel="mediumIL">
      <uap:VisualElements DisplayName="NevroTick artwork laboratory" Description="Offline resource assembly fixture"
        BackgroundColor="transparent" Square44x44Logo="Assets\Square44x44Logo.png"
        Square150x150Logo="Assets\Square150x150Logo.png" AppListEntry="none" />
    </Application>
  </Applications>
  <Capabilities><rescap:Capability Name="runFullTrust" /></Capabilities>
</Package>
'''


def handoff_snapshot(
    path: Path, pin: str, *, include_ui: bool = False,
) -> tuple[dict[str, bytes], bytes, str]:
    """Recheck the selected art, Properties and optional UI on one held export."""
    descriptor: int
    anchor: Path
    with stage.held_directory(path) as (descriptor, anchor):
        observed: dict[str, Any] = verify.inspect_msix_handoff(stage.ROOT, path, pin)
        stage._parent_unchanged(path, descriptor)
        files: dict[str, dict[str, Any]] = observed["files"]
        artwork: dict[str, str] = {
            name: row["sha256"] for name, row in files.items()
            if row["disposition"] == "unadmitted-artwork"
        }
        required: set[str] = {
            ARTWORK_PREFIX + "Assets/" + name
            for name in verify.MSIX_BASE_ARTWORK.keys() | verify.MSIX_VARIANT_ARTWORK.keys()
        }
        stage.require(
            observed["assembly_ready"] is False and observed["package_identity"] is None
            and artwork.keys() == required
            and files.get(PROPERTIES, {}).get("disposition") == "private-authoring",
            "ArtworkLab handoff selection differs.",
        )
        selected: dict[str, bytes] = {
            name.removeprefix(ARTWORK_PREFIX): verify.read_bound_msix_leaf(
                anchor / name, image_digest, staging_descriptor=descriptor, maximum=256 * 1024,
            )
            for name, image_digest in artwork.items()
        }
        properties: bytes = verify.read_bound_msix_leaf(
            anchor / PROPERTIES, files[PROPERTIES]["sha256"], staging_descriptor=descriptor,
        )
        if include_ui:
            image: dict[str, Any] = files.get(UI_IMAGE, {})
            stage.require(image.get("disposition") == "unadmitted-ui-image",
                          "ArtworkLab requires the checked UI image from this handoff.")
            selected[UI_IMAGE] = verify.read_bound_msix_leaf(
                anchor / UI_IMAGE, image["sha256"], staging_descriptor=descriptor, maximum=MAX_PE_BYTES,
            )
        return selected, properties, observed["generation_receipt_sha256"]


def guarded_ui_snapshot(
    path: Path, pin: str, map_path: Path, *, generation: str, matching_ui: bytes,
) -> bytes:
    """Join a separately verified guarded UI to the same checked authoring generation."""
    descriptor: int
    anchor: Path
    with stage.held_directory(path) as (descriptor, anchor):
        receipt: bytes = verify.read_bound_msix_leaf(
            anchor / "verification.json", pin, staging_descriptor=descriptor,
            maximum=verify.MSIX_HANDOFF_RECEIPT_MAX_BYTES,
        )
        report: dict[str, Any] = verify.decode_msix_authoring(receipt)
        inspection: object = report.get("msix_assembly_inputs")
        entry: object = report.get("ui_preentry")
        stage.require(
            report.get("status") == "passed" and report.get("native_windows_execution") == "not performed"
            and type(inspection) is dict and inspection.get("generation_receipt_sha256") == generation
            and type(entry) is dict,
            "Guarded UI requires the same authoring generation and its baseline entry receipt.",
        )
        artifact: dict[str, Any] = report["artifact"]
        image: str
        symbols: str
        image, symbols = verify._msix_handoff_image(
            artifact, report["ui_versioninfo"], "ui", "fixture", "nevrotick-ui", generation,
        )
        data: bytes
        image_pin: str
        symbol_facts: dict[str, str | int]
        (data, image_pin), _, symbol_facts = read_export_pair(anchor / image, anchor / symbols, subsystem="gui")
        stage.require(image_pin == artifact["sha256"] == entry.get("image_sha256")
                      and symbol_facts == {name: artifact["symbols"][name] for name in ("sha256", "guid", "age")},
                      "Guarded UI image or symbols differ from the pinned export.")
        link_map: bytes = stage.read_bound(map_path, 32 * 1024**2, entry["map_sha256"])
        observed: dict[str, object] = ui_preentry_declarations(
            data, link_map.decode("utf-8"), object_sha256=entry["object_sha256"],
        )
        stage.require(all(type(entry.get(name)) is type(value) and entry[name] == value
                          for name, value in observed.items()),
                      "Guarded UI PE entry differs from the pinned link receipt.")
        stage.require(ui_resource_payloads(data) == ui_resource_payloads(matching_ui),
                      "Guarded UI embedded resources differ from the checked authoring handoff.")
        return data


def pri_snapshot(path: Path, pin: str, artwork: dict[str, bytes]) -> bytes:
    """Reconcile the independently pinned native readback with the selected art."""
    descriptor: int
    anchor: Path
    with stage.held_directory(path) as (descriptor, anchor):
        raw: bytes = verify.read_bound_msix_leaf(
            anchor / "readback.json", pin, staging_descriptor=descriptor, maximum=32 * 1024,
        )
        report: dict[str, Any] = verify.decode_msix_authoring(raw)
        rows: object = report.get("rows")
        stage.require(
            report.get("status") == "passed" and report.get("map") == IDENTITY
            and report.get("candidate_count") == len(artwork) == sum(GROUPS.values())
            and report.get("groups") == GROUPS and type(rows) is dict and rows.keys() == artwork.keys()
            and all(type(rows[name]) is dict and rows[name].get("bytes") == len(data)
                    and rows[name].get("sha256") == stage.digest(data) for name, data in artwork.items()),
            "Native PRI readback differs from the exact ArtworkLab cohort.",
        )
        try:
            verify.read_bound_msix_leaf(
                anchor / "complete.xml", report["dump_sha256"], staging_descriptor=descriptor, maximum=64 * 1024,
            )
            return verify.read_bound_msix_leaf(
                anchor / "complete.pri", report["pri_sha256"], staging_descriptor=descriptor, maximum=64 * 1024,
            )
        except verify.ENV.Incomplete as error:
            raise ValueError("ArtworkLab PRI or detailed dump differs from the pinned readback.") from error


def _manifest_properties(properties: bytes) -> bytes:
    """Project checked Properties under the package's own Trusted Launch prefix."""
    stage.require(properties.startswith(XML_DECLARATION), "ArtworkLab Properties XML declaration differs.")
    start: bytes
    separator: bytes
    children: bytes
    start, separator, children = properties[len(XML_DECLARATION):].partition(b"\n")
    stage.require(
        start.startswith(b"<Properties ") and start.endswith(TRUSTED_LAUNCH_NS + b">")
        and properties.endswith(b"</Properties>\n"),
        "ArtworkLab Properties uses a different namespace serialization.",
    )
    # The fixed package root already declares this prefix for IgnorableNamespaces.
    start = start.removesuffix(TRUSTED_LAUNCH_NS + b">") + b">"
    return start + separator + children


def lab_manifest(properties: bytes) -> bytes:
    """Insert the checked Properties fragment under the fixed lab identity."""
    return MANIFEST_TOP + _manifest_properties(properties) + MANIFEST_BOTTOM


def _replace_once(document: bytes, marker: bytes, replacement: bytes) -> bytes:
    """Replace one unique fixed authoring marker or refuse a template change."""
    stage.require(document.count(marker) == 1, "Combined ArtworkLab manifest template differs.")
    return document.replace(marker, replacement)


def combined_manifest(properties: bytes) -> bytes:
    """Retain the checked ArtworkLab identity with both exact disabled extensions."""
    roles: bytes = stage.read_bound(stage.HERE / "AppxManifest.xml", 64 * 1024, ROLE_MANIFEST_SHA256)
    start: bytes = b"      <Extensions>\n"
    end: bytes = b"      </Extensions>\n"
    stage.require(roles.count(start) == roles.count(end) == 1, "The disabled service extension block differs.")
    extensions: bytes = start + roles.split(start, 1)[1].split(end, 1)[0] + end
    top: bytes = _replace_once(
        MANIFEST_TOP, b' IgnorableNamespaces="uap uap10 rescap trustedlaunch"',
        b' xmlns:desktop6="http://schemas.microsoft.com/appx/manifest/desktop/windows10/6"'
        b' IgnorableNamespaces="uap uap10 rescap trustedlaunch desktop6"',
    )
    top = _replace_once(top, b'Version="1.0.0.0"', b'Version="1.0.1.0"')
    bottom: bytes = _replace_once(
        MANIFEST_BOTTOM, b'Id="Inert" Executable="Inert.exe"', b'Id="Observe" Executable="nevrotick-ui.exe"',
    )
    bottom = _replace_once(bottom, b"    </Application>\n", extensions + b"    </Application>\n")
    bottom = _replace_once(bottom, b'<rescap:Capability Name="runFullTrust" />',
                           b'<rescap:Capability Name="runFullTrust" /><rescap:Capability Name="packagedServices" />')
    return top + _manifest_properties(properties) + bottom


def main() -> int:
    parser: argparse.ArgumentParser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--handoff", type=Path, required=True)
    parser.add_argument("--handoff-receipt-sha256", required=True)
    parser.add_argument("--pri-dir", type=Path, required=True)
    parser.add_argument("--pri-receipt-sha256", required=True)
    image_choice: argparse._MutuallyExclusiveGroup = parser.add_mutually_exclusive_group(required=True)
    image_choice.add_argument("--inert", type=Path)
    image_choice.add_argument("--role-link-receipt", type=Path, help="stage the checked Slint UI and disabled service pair")
    digest_choice: argparse._MutuallyExclusiveGroup = parser.add_mutually_exclusive_group(required=True)
    digest_choice.add_argument("--inert-sha256")
    digest_choice.add_argument("--role-link-receipt-sha256")
    parser.add_argument("--guarded-ui-export", type=Path, help="separately retained guarded UI link export")
    parser.add_argument("--guarded-ui-receipt-sha256")
    parser.add_argument("--guarded-ui-link-map", type=Path)
    parser.add_argument("--output", type=Path, required=True)
    args: argparse.Namespace = parser.parse_args()
    handoff: Path = args.handoff.absolute()
    pri_dir: Path = args.pri_dir.absolute()
    inert: Path | None = args.inert
    role_receipt: Path | None = args.role_link_receipt
    role_pin: str | None = args.role_link_receipt_sha256
    output: Path = args.output.absolute()
    combined: bool = role_receipt is not None
    guarded_export: Path | None = args.guarded_ui_export
    guarded_pin: str | None = args.guarded_ui_receipt_sha256
    guarded_map: Path | None = args.guarded_ui_link_map
    if combined != (role_pin is not None):
        parser.error("the selected image input and its digest must name the same ArtworkLab profile")
    if any(value is not None for value in (guarded_export, guarded_pin, guarded_map)) and (
        not combined or guarded_export is None or guarded_pin is None or guarded_map is None
    ):
        parser.error("the guarded UI selection requires combined roles, its export, receipt digest and link map")
    try:
        stage._outside_source(handoff, output)
        stage._outside_source(pri_dir, output)
        if role_receipt is not None:
            role_receipt = role_receipt.absolute()
            stage._outside_source(role_receipt.parent, output)
        if inert is not None:
            inert = inert.absolute()
            stage._outside_source(inert, output)
            stage.require(args.inert_sha256 == INERT_SHA256, "ArtworkLab requires the reviewed inert image.")
        selected: dict[str, bytes]
        properties: bytes
        generation: str
        selected, properties, generation = handoff_snapshot(handoff, args.handoff_receipt_sha256, include_ui=combined)
        ui: bytes | None = selected.pop(UI_IMAGE, None)
        pri: bytes = pri_snapshot(pri_dir, args.pri_receipt_sha256, selected)
        images: dict[str, bytes]
        manifest: bytes
        if role_receipt is not None and role_pin is not None and ui is not None:
            if guarded_export is not None and guarded_pin is not None and guarded_map is not None:
                guarded_export = guarded_export.absolute()
                guarded_map = guarded_map.absolute()
                stage._outside_source(guarded_export, output)
                stage._outside_source(guarded_map, output)
                ui = guarded_ui_snapshot(guarded_export, guarded_pin, guarded_map,
                                         generation=generation, matching_ui=ui)
                images = stage.role_images(role_receipt, role_pin, require_baseline_entry=True)
            else:
                images = stage.role_images(role_receipt, role_pin)
            images[UI_IMAGE] = ui
            manifest = combined_manifest(properties)
        elif inert is not None:
            images = {"Inert.exe": stage.read_bound(inert, 64 * 1024, INERT_SHA256)}
            manifest = lab_manifest(properties)
        else:
            raise ValueError("The combined ArtworkLab selections are incomplete.")
        payload: dict[str, bytes] = {
            **selected, **images, "AppxManifest.xml": manifest, "resources.pri": pri,
        }
        files: dict[str, str | dict[str, str]] = stage.stage_payload(output, payload, None)
    except stage.PublicationUncertainError as error:
        messages: tuple[str, ...] = (
            "ArtworkLab payload rename completed. Publication durability, path or cleanup is unconfirmed.",
            *getattr(error, "__notes__", ()),
        )
        print("\n".join(messages), file=sys.stderr)
        return 3
    except (OSError, ValueError, KeyError, TypeError, RuntimeError) as error:
        messages: tuple[str, ...] = (f"ArtworkLab staging refused: {error}", *getattr(error, "__notes__", ()))
        print("\n".join(messages), file=sys.stderr)
        return 1
    report: dict[str, object] = {
        "status": "unsigned-combined-artwork-lab-inputs-staged" if combined else "unsigned-artwork-lab-inputs-staged",
        "assembly_ready": False,
        "lab_identity": IDENTITY, "handoff_receipt_sha256": args.handoff_receipt_sha256,
        "generation_receipt_sha256": generation, "pri_readback_sha256": args.pri_receipt_sha256,
        "properties_sha256": stage.digest(properties), "payload": files,
    }
    if combined:
        report["role_link_receipt_sha256"] = role_pin
    if guarded_pin is not None:
        report["guarded_ui_receipt_sha256"] = guarded_pin
    print(json.dumps(report, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
