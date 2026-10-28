#!/usr/bin/env python3
"""Acquire the reviewed Rust toolchain only inside an owned hosted-runner area."""

from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import sys
import tomllib
import urllib.request


ROOT = Path(__file__).resolve().parents[2]


def download(url, expected):
    """Pin the official origin, bounded bytes and digest before execution."""
    if not url.startswith("https://static.rust-lang.org/"):
        raise ValueError("Unadmitted Rust origin.")
    with urllib.request.urlopen(url, timeout=60) as response:
        data = response.read(64 * 1024 * 1024 + 1)
    if len(data) > 64 * 1024 * 1024 or hashlib.sha256(data).hexdigest() != expected:
        raise ValueError("Rust acquisition bytes differ from the reviewed digest.")
    return data


def private_area(runner_temp):
    runner_temp = runner_temp.resolve(strict=True)
    if runner_temp == ROOT or runner_temp.is_relative_to(ROOT):
        raise ValueError("Rust preparation must be outside the checkout.")
    work = runner_temp / "nevrotick-rust"
    work.mkdir(mode=0o700)  # Existing paths, including symlinks, fail closed.
    return work


def main():
    try:
        if sys.platform != "linux" or os.geteuid() == 0:
            raise ValueError("Rust preparation requires the ordinary Linux runner user.")
        runner_temp = Path(os.environ["RUNNER_TEMP"])
        work = private_area(runner_temp)
        policy = json.loads((ROOT / "policy/tool-manifest.json").read_text())["rust"]
        pin = tomllib.loads((ROOT / "rust-toolchain.toml").read_text())["toolchain"]
        if pin["channel"] != policy["channel"]:
            raise ValueError("Rust toolchain pins disagree.")
        # Check the distribution metadata before the manager sees any package.
        download(policy["manifest_url"], policy["manifest_sha256"])
        installer = work / "rustup-init"
        installer.write_bytes(download(policy["rustup_linux_archive_url"], policy["rustup_linux_sha256"]))
        installer.chmod(0o700)
        cargo_home, rustup_home = work / "cargo", work / "rustup"
        home = work / "home"
        home.mkdir()
        environment = {"PATH": "/usr/bin:/bin", "HOME": str(home), "LANG": "C.UTF-8",
                       "CARGO_HOME": str(cargo_home), "RUSTUP_HOME": str(rustup_home)}
        spec = importlib.util.spec_from_file_location("bounded_process", ROOT / "scripts/check-environment.py")
        checks = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(checks)
        checks.require_ok(checks.run(
            [str(installer), "-y", "--no-modify-path", "--default-toolchain", "none"],
            cwd=work, environment=environment, timeout=120,
        ))
        manager = cargo_home / "bin/rustup"
        if hashlib.sha256(manager.read_bytes()).hexdigest() != policy["rustup_linux_sha256"]:
            raise ValueError("Prepared Rust manager differs from its admitted installer.")
        checks.require_ok(checks.run(
            [str(manager), "toolchain", "install", pin["channel"], "--no-self-update",
             "--profile", pin["profile"], "--component", ",".join(pin["components"]),
             "--target", ",".join(pin["targets"])],
            cwd=work, environment=environment, timeout=1800,
        ))
        with Path(os.environ["GITHUB_ENV"]).open("a", encoding="utf-8") as output:
            output.write(f"CARGO_HOME={cargo_home}\nRUSTUP_HOME={rustup_home}\nRUSTUP_TOOLCHAIN={pin['channel']}\n")
        print(f"Prepared reviewed Rust {pin['channel']} in the private ephemeral runner area.")
    except (OSError, ValueError, KeyError, RuntimeError) as error:
        print(f"FAILED: Rust acquisition is incomplete: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
