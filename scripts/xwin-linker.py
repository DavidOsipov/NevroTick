#!/usr/bin/env python3
"""Use pinned Rust's bundled LLD with the admitted xwin libraries; preserve all link args."""
from __future__ import annotations

import json
import hashlib
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[1]


def digest(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def main() -> None:
    policy = json.loads((ROOT / "policy/tool-manifest.json").read_text())
    config_path = Path(os.environ.get("NEVROTICK_XWIN_CONFIG", str(Path.home() / ".local/share/nevrotick/xwin.json")))
    if not config_path.is_file():
        raise RuntimeError("Run scripts/setup-xwin.py with the selected cache before linking.")
    config = json.loads(config_path.read_text())
    inputs = policy["cross_toolchain"]["microsoft_inputs"]
    for key in ["architecture", "variant", "sdk_version", "crt_version", "channel_manifest_sha256", "package_manifest_sha256", "package_manifest_declared_sha256"]:
        if config["microsoft_inputs"][key] != inputs[key]:
            raise RuntimeError(f"xwin setup differs from policy ({key}); rerun reviewed setup.")
    rust = policy["rust"]
    toolchain = rust["channel"] + "-" + rust["host"]
    rustup = Path.home() / ".cargo/bin/rustup"
    if digest(rustup) != rust["rustup_linux_sha256"]:
        raise RuntimeError("Rustup binary differs from the admitted digest; refusing execution.")
    # This metadata-only helper does not participate in Cargo's jobserver.
    # subprocess closes its inherited descriptors; forwarding their environment
    # names would make rustc warn about invalid jobserver handles. Keep the
    # original environment intact for the eventual linker exec.
    helper_environment = dict(os.environ)
    helper_environment.pop("CARGO_MAKEFLAGS", None)
    helper_environment.pop("MAKEFLAGS", None)
    sysroot = Path(subprocess.check_output([str(rustup), "run", toolchain, "rustc", "--print", "sysroot"],
                                          text=True, env=helper_environment).strip())
    linker = sysroot / "lib/rustlib" / rust["host"] / "bin/rust-lld"
    if not linker.is_file():
        raise RuntimeError(f"Pinned Rust bundled linker missing: {linker}")
    if digest(linker) != policy["cross_toolchain"]["linker"]["binary_sha256"]:
        raise RuntimeError("Bundled linker differs from the admitted digest; refusing execution.")
    splat = Path(config["root"])
    libraries = [splat / "crt/lib/x86_64", splat / "sdk/lib/um/x86_64", splat / "sdk/lib/ucrt/x86_64"]
    if [str(p) for p in libraries] != config["library_paths"] or any(not p.is_dir() for p in libraries):
        raise RuntimeError("Incomplete xwin library directories; rerun setup.")
    # Exact argv, no shell, argument filtering, global flags or persistent Cargo config.
    # The MSVC target sends link.exe-style arguments; LLD uses its link driver.
    os.execv(linker, [str(linker), "-flavor", "link",
                     *("/libpath:" + str(p) for p in libraries), *sys.argv[1:]])


if __name__ == "__main__":
    try:
        main()
    except (RuntimeError, KeyError, OSError, subprocess.SubprocessError) as error:
        print(f"NevroTick xwin linker: {error}", file=sys.stderr)
        sys.exit(1)
