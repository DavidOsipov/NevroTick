#!/usr/bin/env python3
"""Acquire the pinned x64 Microsoft development libraries, without system installers."""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import tempfile
import time
import urllib.parse

ROOT = Path(__file__).resolve().parents[1]
LOCATION = Path.home() / ".local/share/nevrotick/xwin.json"
DOWNLOAD_SPEC = importlib.util.spec_from_file_location(
    "xwin_bounded_download", Path(__file__).with_name("bounded_download.py")
)
if DOWNLOAD_SPEC is None or DOWNLOAD_SPEC.loader is None:
    raise RuntimeError("Bounded acquisition helper is unavailable.")
BOUNDED = importlib.util.module_from_spec(DOWNLOAD_SPEC)
DOWNLOAD_SPEC.loader.exec_module(BOUNDED)
HOSTED_SPEC = importlib.util.spec_from_file_location("xwin_hosted", Path(__file__).with_name("xwin_hosted.py"))
if HOSTED_SPEC is None or HOSTED_SPEC.loader is None:
    raise RuntimeError("Hosted acquisition boundary is unavailable.")
HOSTED = importlib.util.module_from_spec(HOSTED_SPEC)
sys.modules[HOSTED_SPEC.name] = HOSTED
HOSTED_SPEC.loader.exec_module(HOSTED)
ARCHIVE_MAXIMUM = 32 * 1024 ** 2
CHANNEL_MAXIMUM = 16 * 1024 ** 2
PACKAGE_MAXIMUM = 64 * 1024 ** 2
TERMS_MAXIMUM = 1024 ** 2


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def download(url: str, destination: Path, expected: str | None = None, *,
             maximum_bytes: int = ARCHIVE_MAXIMUM, allowed_origins=()) -> bytes:
    """Return one small bounded acquisition only after digest-checked publication."""
    BOUNDED.download_to_file(url, destination, expected, maximum_bytes=maximum_bytes,
                             allowed_origins=allowed_origins)
    data = BOUNDED.bounded_file_bytes(destination, maximum_bytes)
    if expected is not None and hashlib.sha256(data).hexdigest() != expected.lower():
        raise RuntimeError("Downloaded bytes changed before digest-checked consumption.")
    return data


def install_xwin(binary_bytes: bytes, executable: Path, expected_version: str, backup_dir: Path, *, version_runner=None) -> str:
    if executable.is_symlink() or (executable.exists() and not executable.is_file()):
        raise RuntimeError(f"Refusing non-regular xwin target: {executable}")
    executable.parent.mkdir(parents=True, exist_ok=True)
    wanted_hash = hashlib.sha256(binary_bytes).hexdigest()
    if executable.is_file() and digest(executable) == wanted_hash:
        version = (subprocess.check_output([str(executable), "--version"], text=True)
                   if version_runner is None else version_runner(executable)).strip()
        if version != "xwin " + expected_version:
            raise RuntimeError("Installed xwin version does not match policy.")
        return version
    fd, temporary_name = tempfile.mkstemp(prefix=".nevrotick-xwin-", dir=executable.parent)
    temporary = Path(temporary_name)
    try:
        with os.fdopen(fd, "wb") as stream:
            stream.write(binary_bytes)
            os.fchmod(stream.fileno(), 0o755)
        version = (subprocess.check_output([str(temporary), "--version"], text=True)
                   if version_runner is None else version_runner(temporary)).strip()
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
    entries = packages.get("packages") if type(packages) is dict else None
    if type(entries) is not list or len(entries) > 65_536:
        raise RuntimeError("Package catalog count/type differs.")
    expected, count = set(), 0
    for item in entries:
        payloads = item.get("payloads", []) if type(item) is dict else None
        if type(payloads) is not list or len(payloads) > 4096:
            raise RuntimeError("Catalog payload count/type differs.")
        for payload in payloads:
            count += 1
            if count > 262_144 or type(payload) is not dict:
                raise RuntimeError("Catalog aggregate count/type differs.")
            value = payload.get("sha256")
            if value is not None:
                if type(value) is not str or re.fullmatch(r"[0-9a-fA-F]{64}", value) is None:
                    raise RuntimeError("Catalog payload digest is malformed.")
                expected.add(value.lower())
    expected.add(package_manifest_sha256.lower())
    downloaded = []
    for relative, record in HOSTED.inventory(cache_dir / "dl").items():
        if record["sha256"] not in expected:
            raise RuntimeError("Downloaded payload is absent from the pinned catalog.")
        downloaded.append({"path": str(cache_dir / "dl" / relative), **record})
    return downloaded


def prune_generated_cache(cache_dir: Path, downloaded: list[dict], package_path: Path) -> dict:
    """Remove verified payload copies and matching unpack trees; retain unknown entries."""
    HOSTED.inventory(cache_dir)
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
    return sum(row["bytes"] for row in HOSTED.inventory(directory).values())


def main(*, _hosted_worker=False) -> None:
    if _hosted_worker:
        HOSTED.require_worker_boundary()
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--cache-dir", type=Path, required=True,
                        help="Cache on a Windows drive, outside this checkout; e.g. /mnt/c/.../NevroTick/xwin-cache")
    parser.add_argument("--accept-license", action="store_true",
                        help="Accept applicable Microsoft acquisition terms for this selected development-tool route")
    parser.add_argument("--receipt", type=Path)
    parser.add_argument("--keep-cache", action="store_true",
                        help="Retain verified CAB/VSIX/MSI copies and unpack intermediates for offline reacquisition")
    parser.add_argument("--hosted-runner", action="store_true", help="Use a precreated owned hosted Linux quota profile")
    args = parser.parse_args()
    if not args.accept_license:
        parser.error("Review the Microsoft terms recorded in policy/tool-manifest.json; explicit --accept-license is required.")
    layout = None
    if _hosted_worker and not args.hosted_runner:
        parser.error("Internal worker requires the hosted profile.")
    if args.hosted_runner:
        if args.receipt is None:
            parser.error("Hosted acquisition requires a new explicit receipt inside its payload.")
        layout = HOSTED.admit_layout(args.cache_dir, args.receipt, worker=_hosted_worker)
        if not _hosted_worker:
            HOSTED.dispatch(ROOT, layout, keep_cache=args.keep_cache)
            return
        HOSTED.remaining_deadline(os.environ)
        cache = layout.cache
    else:
        cache = args.cache_dir.expanduser().resolve()
        if not re.match(r"^/mnt/[a-z](?:/|$)", str(cache)) or cache == ROOT or ROOT in cache.parents:
            parser.error("Use a Windows-drive cache outside the checkout to keep payloads off the WSL root disk.")
    if args.receipt and (args.receipt.resolve() == ROOT or ROOT in args.receipt.resolve().parents):
        parser.error("Keep machine-specific setup receipts outside the checkout.")
    policy = json.loads(BOUNDED.bounded_file_bytes(ROOT / "policy/tool-manifest.json", 2 * 1024 ** 2))
    cross = policy["cross_toolchain"]
    tool, inputs = cross["xwin"], cross["microsoft_inputs"]
    if inputs["architecture"] != "x86_64" or inputs["variant"] != "desktop":
        raise RuntimeError("This setup supports only the admitted x64 desktop cohort.")
    if not inputs["sdk_version"] or not inputs["crt_version"]:
        raise RuntimeError("Resolve and review exact SDK/CRT selectors before installing.")
    cache.mkdir(parents=True, exist_ok=True, mode=0o700 if layout else 0o755)
    provenance = cache / "provenance"
    provenance.mkdir(exist_ok=True)

    archive = (layout.bootstrap if layout else cache / "bootstrap") / Path(urllib.parse.urlparse(tool["url"]).path).name
    blob = download(tool["url"], archive, tool["sha256"], maximum_bytes=ARCHIVE_MAXIMUM,
                    allowed_origins=("https://github.com", "https://release-assets.githubusercontent.com"))
    members = HOSTED.tar_members(blob, tool["version"])
    binary_bytes = members["xwin"]
    for name in ["LICENSE-APACHE", "LICENSE-MIT", "README.md"]:
        (provenance / ("xwin-" + name)).write_bytes(members[name])
    executable = layout.bootstrap / "bin/xwin" if layout else Path.home() / ".cargo/bin/xwin"
    def hosted_process(command, timeout):
        environment = HOSTED.worker_environment(layout, float(os.environ["NEVROTICK_XWIN_DEADLINE"]))
        return HOSTED.run_checked(HOSTED.namespace(ROOT, layout, worker=False, environment=environment) + command,
            cwd=layout.payload, environment=environment,
            timeout=min(timeout, HOSTED.remaining_deadline(os.environ)), runner=HOSTED.process_module(ROOT).run)
    if layout:
        install_xwin(binary_bytes, executable, tool["version"], layout.bootstrap,
                     version_runner=lambda path: hosted_process([str(path), "--version"], 10))
    else:
        install_xwin(binary_bytes, executable, tool["version"], cache / "bootstrap")

    channel_url = inputs.get("channel_manifest_url") or (
        f"https://aka.ms/vs/{inputs['manifest_version']}/stable/channel"
    )
    channel_path = provenance / ("manifest_" + inputs["manifest_version"] + ".json")
    channel_bytes = download(channel_url, channel_path, inputs["channel_manifest_sha256"],
                             maximum_bytes=CHANNEL_MAXIMUM,
                             allowed_origins=("https://aka.ms", "https://download.visualstudio.microsoft.com"))
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
    packages = json.loads(download(payload["url"], package_path, inputs["package_manifest_sha256"],
                                   maximum_bytes=PACKAGE_MAXIMUM,
                                   allowed_origins=("https://download.visualstudio.microsoft.com",)))

    # Terms are a reviewed URL/content pair. Mutable HTML supplies no authority
    # for another outbound request. The old terms_url field is provenance only.
    terms_url = inputs["terms_document_url"]
    terms_path = provenance / "microsoft-buildtools-terms.docx"
    download(terms_url, terms_path, inputs["terms_document_sha256"], maximum_bytes=TERMS_MAXIMUM,
             allowed_origins=("https://visualstudio.microsoft.com",))

    splat = cache / "splat"
    command = [str(executable), "--accept-license", "--cache-dir", str(cache),
               "--manifest", str(channel_path), "--arch", "x86_64", "--variant", "desktop",
               "--sdk-version", inputs["sdk_version"], "--crt-version", inputs["crt_version"],
               "--timeout", "120s", "splat", "--disable-symlinks", "--output", str(splat)]
    log = cache / "setup-xwin.log"
    if layout:
        output = hosted_process(command, HOSTED.WHOLE_SECONDS)
        with log.open("x") as stream: stream.write(output)
    else:
        with log.open("w") as output:
            result = subprocess.run(command, stdout=output, stderr=subprocess.STDOUT, check=False)
        if result.returncode:
            raise RuntimeError(f"xwin exited {result.returncode}; inspect {log}")

    libraries = [splat / "crt/lib/x86_64", splat / "sdk/lib/um/x86_64", splat / "sdk/lib/ucrt/x86_64"]
    for directory, name in zip(libraries, ["msvcrt.lib", "kernel32.lib", "ucrt.lib"]):
        candidate = directory / name
        if candidate.is_symlink() or not candidate.is_file():
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
    location = layout.locator if layout else LOCATION
    if layout:
        files = HOSTED.inventory(cache)
        receipt["library_files"] = [{"path": str(directory / name), **files[str((directory / name).relative_to(cache))]}
            for directory, name in zip(libraries, ["msvcrt.lib", "kernel32.lib", "ucrt.lib"])]
        receipt["inventory"] = {"schema_version": 1, "scope_root": str(cache), "file_count": len(files),
            **HOSTED.write_json(layout.payload / "inventory.json", {"schema_version": 1, "files": files}, HOSTED.MAX_INVENTORY)}
        receipt["hosted"] = {"profile": str(layout.profile), "payload": str(layout.payload), "cache": str(cache),
            "uid": os.geteuid(), "mount": HOSTED.quota_facts(layout, worker=True),
            "supervisor": {"status": "internal-worker-only; parent completion required"}}
        HOSTED.write_json(location, receipt, 2 * 1024 ** 2)
        HOSTED.write_json(args.receipt, receipt, 2 * 1024 ** 2)
    else:
        location.parent.mkdir(parents=True, exist_ok=True)
        location.write_text(json.dumps(receipt, indent=2) + "\n")
        if args.receipt:
            args.receipt.parent.mkdir(parents=True, exist_ok=True)
            args.receipt.write_text(json.dumps(receipt, indent=2) + "\n")
    print(json.dumps({"location": str(location), "root": str(splat),
                      "sdk": inputs["sdk_version"], "crt": inputs["crt_version"],
                      "splat_logical_bytes": receipt["splat_logical_bytes"],
                      "downloaded_payload_bytes": receipt["downloaded_payload_bytes"],
                      "cache_logical_bytes": receipt["cache_logical_bytes"]}, indent=2))


if __name__ == "__main__":
    main()
