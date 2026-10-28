#!/usr/bin/env python3
"""Install the reviewed Rust pin and Linux tool archives; build no project code."""
from __future__ import annotations

import argparse
import hashlib
import io
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import tarfile
import tempfile
import time
import tomllib
import urllib.request

ROOT = Path(__file__).resolve().parents[1]


def sha256(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def fetch(url: str, expected: str) -> bytes:
    if not url.startswith(("https://github.com/", "https://static.rust-lang.org/")):
        raise ValueError("Unadmitted tool origin")
    with urllib.request.urlopen(url, timeout=60) as response:
        data = response.read(160 * 1024 * 1024 + 1)
    if len(data) > 160 * 1024 * 1024 or sha256(data) != expected:
        raise ValueError(f"Artifact size/digest mismatch: {url}")
    return data


def install_tool(tool: dict, bin_dir: Path, cache: Path, backup: Path) -> dict:
    name = tool["name"]
    if name not in {"cargo-nextest", "cargo-deny", "cargo-audit", "cargo-mutants", "cargo-llvm-cov"}:
        raise ValueError("Unexpected executable in tool manifest")
    print(f"Verifying {name} {tool['version']}...", flush=True)
    archive_path = cache / (tool["sha256"] + ".tar.gz")
    if archive_path.exists() and archive_path.stat().st_size > 160 * 1024 * 1024:
        raise ValueError(f"Cached archive exceeds the download budget: {name}")
    data = archive_path.read_bytes() if archive_path.exists() else fetch(tool["url"], tool["sha256"])
    if sha256(data) != tool["sha256"]:
        raise ValueError(f"Cached archive digest mismatch: {name}")
    archive_path.write_bytes(data)
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as archive:
        binaries = [member for member in archive.getmembers()
                    if member.isfile() and Path(member.name).name == name]
        if len(binaries) != 1 or binaries[0].size > 256 * 1024 * 1024:
            raise ValueError(f"Ambiguous/oversized executable: {name}")
        source = archive.extractfile(binaries[0])
        if source is None:
            raise ValueError(f"Missing executable: {name}")
        payload = source.read()
        for member in archive.getmembers():
            if member.isfile() and Path(member.name).name.upper().startswith(("LICENSE", "COPYING", "NOTICE")):
                if member.size > 1024 * 1024:
                    raise ValueError("Oversized tool notice")
                notice = archive.extractfile(member)
                if notice is not None:
                    notice_dir = cache / (name + "-" + tool["version"] + "-notices")
                    notice_dir.mkdir(exist_ok=True)
                    (notice_dir / Path(member.name).name).write_bytes(notice.read())
    target = bin_dir / name
    if target.is_symlink():
        raise ValueError(f"Refusing to replace a symlink: {target}")
    previous = backup / name
    existed = target.exists()
    if existed:
        shutil.copy2(target, previous)
    descriptor, temporary_name = tempfile.mkstemp(prefix=name + ".", dir=bin_dir)
    temporary = Path(temporary_name)
    try:
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(payload)
        temporary.chmod(0o755)
        os.replace(temporary, target)
        command = [str(target), *tool["version_args"]]
        result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, timeout=30, check=True)
        version = result.stdout.strip()
        if tool["version"] not in version.splitlines()[0].split():
            raise ValueError(f"Wrong executable version: {name}: {version}")
    except BaseException:
        if existed:
            shutil.copy2(previous, target)
        else:
            target.unlink(missing_ok=True)
        raise
    finally:
        temporary.unlink(missing_ok=True)
    print(version.splitlines()[0], flush=True)
    return {"name": name, "version": tool["version"], "path": str(target),
            "archive_sha256": tool["sha256"], "binary_sha256": sha256(payload),
            "previous_binary": str(previous) if existed else None}


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--tools-only", action="store_true", help="Rust pin is already installed")
    parser.add_argument("--json", type=Path, help="Write installation receipt outside the source tree")
    args = parser.parse_args()
    if platform.system() != "Linux" or platform.machine() != "x86_64":
        parser.error("This admitted archive set is Linux x86_64 only")
    if os.geteuid() == 0:
        parser.error("Run tool installation as the ordinary developer, without sudo")
    overrides = [name for name in ("RUSTUP_DIST_SERVER", "RUSTUP_UPDATE_ROOT", "RUSTUP_TOOLCHAIN")
                 if name in os.environ]
    if overrides:
        parser.error("Unadmitted Rust installation overrides: " + ", ".join(overrides))
    manifest = json.loads((ROOT / "policy/tool-manifest.json").read_text())
    pin = tomllib.loads((ROOT / "rust-toolchain.toml").read_text())["toolchain"]
    if pin["channel"] != manifest["rust"]["channel"]:
        parser.error("Toolchain and admission manifest disagree")
    rustup = shutil.which("rustup")
    if rustup is None:
        parser.error("Install rustup from https://rustup.rs first")
    manager = subprocess.run([rustup, "--version"], cwd=ROOT, capture_output=True,
                             text=True, check=True, timeout=30)
    if not manager.stdout.startswith("rustup " + manifest["rust"]["rustup_version"] + " "):
        parser.error("Update rustup to the reviewed manifest version before installing the toolchain")
    fetch(manifest["rust"]["manifest_url"], manifest["rust"]["manifest_sha256"])
    if not args.tools_only:
        subprocess.run([rustup, "toolchain", "install", pin["channel"], "--no-self-update", "--profile", pin["profile"],
                        "--component", ",".join(pin["components"]),
                        "--target", ",".join(pin["targets"])], cwd=ROOT, check=True, timeout=1200)
    cargo_home = Path(os.environ.get("CARGO_HOME", str(Path.home() / ".cargo")))
    bin_dir = cargo_home / "bin"
    bin_dir.mkdir(parents=True, exist_ok=True)
    state = Path.home() / ".local/share/nevrotick/tooling"
    cache = state / "archives"
    cache.mkdir(parents=True, exist_ok=True)
    backup = Path(tempfile.mkdtemp(prefix=time.strftime("%Y%m%d-%H%M%S-"), dir=state))
    print(f"Backups: {backup}", flush=True)
    receipt = {"scope": "Verified upstream tool installation and version smoke; no product analysis",
               "toolchain": pin["channel"], "tools": [], "backup_directory": str(backup),
               "status": "incomplete"}

    def save_receipt() -> None:
        if args.json:
            args.json.parent.mkdir(parents=True, exist_ok=True)
            args.json.write_text(json.dumps(receipt, indent=2) + "\n")

    save_receipt()
    for tool in manifest["tools"]:
        try:
            receipt["tools"].append(install_tool(tool, bin_dir, cache, backup))
        except BaseException:
            receipt["failed_tool"] = tool["name"]
            save_receipt()
            raise
        save_receipt()
    receipt["status"] = "complete"
    save_receipt()
    print(f"Installed {len(receipt['tools'])} pinned current-stable tools. Backups: {backup}")


if __name__ == "__main__":
    main()
