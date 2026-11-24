# SPDX-License-Identifier: GPL-2.0-only
"""Stage the fixed unsigned ArtworkLab payload from independently pinned inputs."""

import argparse
from collections.abc import Iterator
from contextlib import contextmanager
import json
import os
from pathlib import Path
import sys
from typing import Any

sys.path.insert(0, str(Path(__file__).resolve().parent))
import prepare_payload as stage
import verify

IDENTITY: str = "NevroTick.S06.ArtworkLab"
ARTWORK_PREFIX: str = "msix-inputs/"
PROPERTIES: str = "msix-authoring/authoring/msix-properties.xml"
GROUPS: dict[str, int] = {"PackageLogo.png": 3, "Square150x150Logo.png": 3, "Square44x44Logo.png": 45}
# Reviewed build of inert.c retained in the unsigned MakeAppx fixture evidence.
INERT_SHA256: str = "466e48e377946eed00c03469c8a0e24d51e1774a391b221b26136cefc26abe98"
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


@contextmanager
def held_directory(path: Path) -> Iterator[tuple[int, Path]]:
    """Keep subsequent leaf reads on the same directory admitted by the owner."""
    descriptor: int = stage._open_directory(path, private=True)
    try:
        yield descriptor, Path(f"/proc/self/fd/{descriptor}")
        stage._parent_unchanged(path, descriptor)
    finally:
        os.close(descriptor)


def handoff_snapshot(path: Path, pin: str) -> tuple[dict[str, bytes], bytes, str]:
    """Recheck the selected art and Properties leaves against the inspected export."""
    descriptor: int
    anchor: Path
    with held_directory(path) as (descriptor, anchor):
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
        return selected, properties, observed["generation_receipt_sha256"]


def pri_snapshot(path: Path, pin: str, artwork: dict[str, bytes]) -> bytes:
    """Reconcile the independently pinned native readback with the selected art."""
    descriptor: int
    anchor: Path
    with held_directory(path) as (descriptor, anchor):
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


def lab_manifest(properties: bytes) -> bytes:
    """Insert the checked Properties fragment under the fixed lab identity."""
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
    return MANIFEST_TOP + start + separator + children + MANIFEST_BOTTOM


def main() -> int:
    parser: argparse.ArgumentParser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--handoff", type=Path, required=True)
    parser.add_argument("--handoff-receipt-sha256", required=True)
    parser.add_argument("--pri-dir", type=Path, required=True)
    parser.add_argument("--pri-receipt-sha256", required=True)
    parser.add_argument("--inert", type=Path, required=True)
    parser.add_argument("--inert-sha256", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args: argparse.Namespace = parser.parse_args()
    handoff: Path = args.handoff.absolute()
    pri_dir: Path = args.pri_dir.absolute()
    inert: Path = args.inert.absolute()
    output: Path = args.output.absolute()
    try:
        for source in (handoff, pri_dir, inert):
            stage._outside_source(source, output)
        stage.require(args.inert_sha256 == INERT_SHA256, "ArtworkLab requires the reviewed inert image.")
        artwork: dict[str, bytes]
        properties: bytes
        generation: str
        artwork, properties, generation = handoff_snapshot(handoff, args.handoff_receipt_sha256)
        pri: bytes = pri_snapshot(pri_dir, args.pri_receipt_sha256, artwork)
        inert_bytes: bytes = stage.read_bound(inert, 64 * 1024, INERT_SHA256)
        payload: dict[str, bytes] = {
            **artwork, "AppxManifest.xml": lab_manifest(properties),
            "Inert.exe": inert_bytes, "resources.pri": pri,
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
    print(json.dumps({
        "status": "unsigned-artwork-lab-inputs-staged", "assembly_ready": False,
        "lab_identity": IDENTITY, "handoff_receipt_sha256": args.handoff_receipt_sha256,
        "generation_receipt_sha256": generation, "pri_readback_sha256": args.pri_receipt_sha256,
        "properties_sha256": stage.digest(properties), "payload": files,
    }, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
