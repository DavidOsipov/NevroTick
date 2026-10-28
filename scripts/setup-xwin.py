#!/usr/bin/env python3
"""Acquire the pinned x64 Microsoft development libraries, without system installers."""
from __future__ import annotations

import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import tarfile
import tempfile
import time
import urllib.parse
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
LOCATION = Path.home() / ".local/share/nevrotick/xwin.json"


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def download(url: str, destination: Path, expected: str | None = None) -> bytes:
    if destination.is_file() and (expected is None or digest(destination) == expected.lower()):
        return destination.read_bytes()
    with urllib.request.urlopen(url, timeout=120) as response:
        body = response.read()
    if expected and hashlib.sha256(body).hexdigest() != expected.lower():
        raise RuntimeError(f"SHA256 mismatch: {url}")
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(body)
    return body


def install_xwin(binary_bytes: bytes, executable: Path, expected_version: str, backup_dir: Path) -> str:
    if executable.is_symlink() or (executable.exists() and not executable.is_file()):
        raise RuntimeError(f"Refusing non-regular xwin target: {executable}")
    executable.parent.mkdir(parents=True, exist_ok=True)
    wanted_hash = hashlib.sha256(binary_bytes).hexdigest()
    if executable.is_file() and digest(executable) == wanted_hash:
        version = subprocess.check_output([str(executable), "--version"], text=True).strip()
        if version != "xwin " + expected_version:
            raise RuntimeError("Installed xwin version does not match policy.")
        return version
    fd, temporary_name = tempfile.mkstemp(prefix=".nevrotick-xwin-", dir=executable.parent)
    temporary = Path(temporary_name)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(binary_bytes)
            os.fchmod(stream.fileno(), 0o755)
        version = subprocess.check_output([str(temporary), "--version"], text=True).strip()
        if version != "xwin " + expected_version:
            raise RuntimeError("Downloaded xwin version does not match policy; existing target preserved.")
        if executable.is_symlink():
            raise RuntimeError("xwin target became a symlink; refusing replacement.")
        if executable.exists():
            backup_dir.mkdir(parents=True, exist_ok=True)
            previous_hash = digest(executable)
            backup = backup_dir / ("previous-xwin-" + previous_hash)
            if backup.exists() or backup.is_symlink():
                if backup.is_symlink() or not backup.is_file() or digest(backup) != previous_hash:
                    raise RuntimeError("Existing xwin backup is not the expected regular file.")
            else:
                with executable.open("rb") as source, backup.open("xb") as destination:
                    shutil.copyfileobj(source, destination)
        os.replace(temporary, executable)
        return version
    finally:
        temporary.unlink(missing_ok=True)


def verify_catalog_payloads(cache_dir: Path, packages: dict, package_manifest_sha256: str) -> list[dict]:
    expected = {p["sha256"].lower() for item in packages["packages"]
                for p in item.get("payloads", []) if p.get("sha256")}
    expected.add(package_manifest_sha256.lower())
    downloaded = []
    for path in sorted((cache_dir / "dl").rglob("*")):
        if not path.is_file():
            continue
        sha = digest(path)
        if sha not in expected:
            raise RuntimeError(f"Downloaded payload is absent from the pinned catalog: {path}")
        downloaded.append({"path": str(path), "sha256": sha, "bytes": path.stat().st_size})
    return downloaded


def prune_generated_cache(cache_dir: Path, downloaded: list[dict], package_path: Path) -> dict:
    """Remove verified payload copies and matching unpack trees; retain unknown entries."""
    dl = (cache_dir / "dl").resolve()
    selected = []
    for item in downloaded:
        path = Path(item["path"])
        if path.is_symlink() or dl not in path.resolve().parents:
            raise RuntimeError(f"Unexpected payload path during cache cleanup: {path}")
        if path.is_file() and digest(path) != item["sha256"]:
            raise RuntimeError(f"Payload changed before cleanup: {path}")
        if path != package_path:
            selected.append(path)
    names = {Path(item["path"]).name for item in downloaded}
    unpack = cache_dir / "unpack"
    if unpack.is_symlink():
        raise RuntimeError("Refusing to clean a symlinked unpack directory.")
    generated = [p for p in unpack.iterdir() if p.name in names] if unpack.is_dir() else []
    if any(p.is_symlink() for p in generated):
        raise RuntimeError("Refusing to clean a symlinked unpack entry.")
    removed = []
    for path in generated:
        if path.is_dir():
            shutil.rmtree(path)
        else:
            path.unlink()
        removed.append(str(path))
    for path in selected:
        path.unlink(missing_ok=True)
        removed.append(str(path))
    if unpack.is_dir() and not any(unpack.iterdir()):
        unpack.rmdir()
    return {"removed_paths": removed, "raw_payload_copies_retained": False,
            "scope": "Only catalog-verified payload copies and their matching generated unpack trees; SDK/provenance retained"}


def logical_bytes(directory: Path) -> int:
    return sum(os.stat(Path(base) / name).st_size
               for base, _, names in os.walk(directory) for name in names)


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache-dir", type=Path, required=True,
                        help="Cache on a Windows drive, outside this checkout; e.g. /mnt/c/.../NevroTick/xwin-cache")
    parser.add_argument("--accept-license", action="store_true",
                        help="Accept applicable Microsoft acquisition terms for this selected development-tool route")
    parser.add_argument("--receipt", type=Path)
    parser.add_argument("--keep-cache", action="store_true",
                        help="Retain verified CAB/VSIX/MSI copies and unpack intermediates for offline reacquisition")
    args = parser.parse_args()
    if not args.accept_license:
        parser.error("Review the Microsoft terms recorded in policy/tool-manifest.json; explicit --accept-license is required.")
    cache = args.cache_dir.expanduser().resolve()
    if not re.match(r"^/mnt/[a-z](?:/|$)", str(cache)) or cache == ROOT or ROOT in cache.parents:
        parser.error("Use a Windows-drive cache outside the checkout to keep payloads off the WSL root disk.")
    if args.receipt and (args.receipt.resolve() == ROOT or ROOT in args.receipt.resolve().parents):
        parser.error("Keep machine-specific setup receipts outside the checkout.")
    policy = json.loads((ROOT / "policy/tool-manifest.json").read_text())
    cross = policy["cross_toolchain"]
    tool, inputs = cross["xwin"], cross["microsoft_inputs"]
    if inputs["architecture"] != "x86_64" or inputs["variant"] != "desktop":
        raise RuntimeError("This setup supports only the admitted x64 desktop cohort.")
    if not inputs["sdk_version"] or not inputs["crt_version"]:
        raise RuntimeError("Resolve and review exact SDK/CRT selectors before installing.")
    cache.mkdir(parents=True, exist_ok=True)
    provenance = cache / "provenance"
    provenance.mkdir(exist_ok=True)

    archive = cache / "bootstrap" / Path(urllib.parse.urlparse(tool["url"]).path).name
    blob = download(tool["url"], archive, tool["sha256"])
    with tarfile.open(fileobj=io.BytesIO(blob), mode="r:gz") as bundle:
        binary = [item for item in bundle.getmembers() if item.isfile() and Path(item.name).name == "xwin"]
        if len(binary) != 1:
            raise RuntimeError("Expected exactly one regular xwin executable in the verified archive.")
        binary_bytes = bundle.extractfile(binary[0]).read()
        for item in bundle.getmembers():
            if item.isfile() and Path(item.name).name in {"LICENSE-APACHE", "LICENSE-MIT", "README.md"}:
                (provenance / ("xwin-" + Path(item.name).name)).write_bytes(bundle.extractfile(item).read())
    executable = Path.home() / ".cargo/bin/xwin"
    install_xwin(binary_bytes, executable, tool["version"], cache / "bootstrap")

    channel_url = inputs.get("channel_manifest_url") or (
        f"https://aka.ms/vs/{inputs['manifest_version']}/stable/channel"
    )
    channel_path = provenance / ("manifest_" + inputs["manifest_version"] + ".json")
    channel_bytes = download(channel_url, channel_path, inputs["channel_manifest_sha256"])
    channel = json.loads(channel_bytes)
    if str(channel["info"].get("productMilestoneIsPreRelease")).lower() != "false":
        raise RuntimeError("Refusing a preview Microsoft channel manifest.")
    manifests = [item for item in channel["channelItems"] if item["type"] == "Manifest"]
    payload = manifests[0]["payloads"][0]
    # xwin's pinned source documents Microsoft's incorrect declared .vsman hash.
    # Keep that declaration pinned separately; verify the actual raw metadata
    # against the independently reviewed content digest, never checksum=None.
    declared = inputs.get("package_manifest_declared_sha256", inputs["package_manifest_sha256"])
    if payload["sha256"].lower() != declared.lower():
        raise RuntimeError("Package-manifest declaration differs from policy.")
    package_path = cache / "dl" / ("pkg_manifest_" + payload["sha256"].lower() + ".vsman")
    packages = json.loads(download(payload["url"], package_path, inputs["package_manifest_sha256"]))

    # Retain the current catalog-linked terms as well as xwin's licence notices.
    # Microsoft's terms are separate from xwin's MIT/Apache licences.
    terms_page = download(inputs["terms_url"], provenance / "microsoft-terms.html").decode("utf-8")
    encoded = re.search(r"https%3A%2F%2F[^\s\"'<>]+\.docx", terms_page)
    if not encoded:
        raise RuntimeError("Could not identify the current Microsoft terms document.")
    terms_url = urllib.parse.unquote(encoded.group())
    terms_path = provenance / "microsoft-buildtools-terms.docx"
    download(terms_url, terms_path, inputs["terms_document_sha256"])

    splat = cache / "splat"
    command = [str(executable), "--accept-license", "--cache-dir", str(cache),
               "--manifest", str(channel_path), "--arch", "x86_64", "--variant", "desktop",
               "--sdk-version", inputs["sdk_version"], "--crt-version", inputs["crt_version"],
               "--timeout", "120s", "splat", "--disable-symlinks", "--output", str(splat)]
    log = cache / "setup-xwin.log"
    with log.open("w") as output:
        result = subprocess.run(command, stdout=output, stderr=subprocess.STDOUT, check=False)
    if result.returncode:
        raise RuntimeError(f"xwin exited {result.returncode}; inspect {log}")

    libraries = [splat / "crt/lib/x86_64", splat / "sdk/lib/um/x86_64", splat / "sdk/lib/ucrt/x86_64"]
    for directory, name in zip(libraries, ["msvcrt.lib", "kernel32.lib", "ucrt.lib"]):
        if not (directory / name).is_file():
            raise RuntimeError(f"Required linker input missing: {directory / name}")
    downloaded = verify_catalog_payloads(cache, packages, inputs["package_manifest_sha256"])
    receipt = {
        "schema_version": 1, "created_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()),
        "root": str(splat), "library_paths": [str(p) for p in libraries],
        "xwin": {**tool, "binary": str(executable), "binary_sha256": digest(executable)},
        "microsoft_inputs": inputs, "channel_manifest": str(channel_path),
        "package_manifest": str(package_path), "channel_info": channel["info"],
        "terms": {"accepted_for_development_acquisition": True, "url": terms_url,
                  "path": str(terms_path), "sha256": digest(terms_path),
                  "redistribution_clearance": False},
        "command": command, "log": str(log), "payloads": downloaded,
        "splat_logical_bytes": logical_bytes(splat),
        "downloaded_payload_bytes": sum(p["bytes"] for p in downloaded),
        "debug_libraries": False, "debug_symbols": False, "system_installers": False,
    }
    if args.keep_cache:
        receipt["cache_cleanup"] = {"raw_payload_copies_retained": True, "removed_paths": []}
    else:
        receipt["cache_cleanup"] = prune_generated_cache(cache, downloaded, package_path)
    receipt["cache_logical_bytes"] = logical_bytes(cache)
    LOCATION.parent.mkdir(parents=True, exist_ok=True)
    LOCATION.write_text(json.dumps(receipt, indent=2) + "\n")
    if args.receipt:
        args.receipt.parent.mkdir(parents=True, exist_ok=True)
        args.receipt.write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps({"location": str(LOCATION), "root": str(splat),
                      "sdk": inputs["sdk_version"], "crt": inputs["crt_version"],
                      "splat_logical_bytes": receipt["splat_logical_bytes"],
                      "downloaded_payload_bytes": receipt["downloaded_payload_bytes"],
                      "cache_logical_bytes": receipt["cache_logical_bytes"]}, indent=2))


if __name__ == "__main__":
    main()
