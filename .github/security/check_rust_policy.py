#!/usr/bin/env python3
"""Admit the exact installed compiler and real Cargo policy before Rust extraction.

No tool installation or product compilation occurs here. Missing pinned tools
fail closed; WSL-specific host-helper identities are intentionally not imposed
on a hosted runner. Windows cfg/native behavior remains a separate gate.
"""

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import sys
import subprocess
import tomllib


ROOT = Path(__file__).resolve().parents[2]


def main():
    try:
        policy = json.loads((ROOT / "policy/tool-manifest.json").read_text())
        version = tomllib.loads((ROOT / "rust-toolchain.toml").read_text())["toolchain"]["channel"]
        if version != policy["rust"]["channel"]:
            raise ValueError("Rust pin and manifest differ.")
        rustup = Path(os.environ.get("RUSTUP_HOME", str(Path.home() / ".rustup")))
        binary_dir = rustup / "toolchains" / f"{version}-x86_64-unknown-linux-gnu" / "bin"
        for name in ("cargo", "rustc", "rustdoc"):
            actual = hashlib.sha256((binary_dir / name).read_bytes()).hexdigest()
            if actual != policy["rust"]["linux_binary_sha256"][name]:
                raise ValueError("Installed Rust executable differs from the reviewed distribution.")
        if not (ROOT / "Cargo.lock").is_file():
            raise ValueError("A versioned Cargo.lock is required for actual product analysis.")
        if subprocess.run(["git", "ls-files", "--error-unmatch", "Cargo.lock"], cwd=ROOT,
                          stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                          timeout=10).returncode != 0:
            raise ValueError("Cargo.lock exists but is not versioned.")
        spec = importlib.util.spec_from_file_location("strict_policy", ROOT / "scripts/check-strict.py")
        strict = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(strict)
        expected = {"target": {strict.ENV.WINDOWS_TARGET: {"rustflags": strict.ENV.MSVC_FLAGS}}}
        if tomllib.loads((ROOT / ".cargo/config.toml").read_text()) != expected:
            raise ValueError("Repository Cargo configuration differs from admitted target policy.")
        config = strict.read_configuration(ROOT)
        adopted = strict.require_workspace_adoption(
            ROOT, config["policy/rust-workspace.toml"],
            {"tools": {"cargo": {"path": str(binary_dir / "cargo")}}},
        )
        # CodeQL's Rust extractor must resolve this admitted compiler directly.
        path_file = os.environ.get("GITHUB_PATH")
        if path_file:
            with Path(path_file).open("a", encoding="utf-8") as output:
                output.write(str(binary_dir) + "\n")
        print(f"Admitted Rust {version}; {len(adopted['members'])} member(s) inherit the lint/profile policy.")
    except (OSError, ValueError, KeyError, RuntimeError, subprocess.SubprocessError) as error:
        print(f"FAILED: Rust extraction prerequisites are incomplete: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
