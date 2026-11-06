#!/usr/bin/env python3
"""Admit the pinned Gitleaks executable, challenge detection, then scan read-only."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile


ROOT = Path(__file__).resolve().parents[2]


def run(command, environment, *, timeout=300):
    """Do not log matched source, values or reports; bound owned process lifetime."""
    process = subprocess.Popen(command, cwd=ROOT, env=environment,
                               stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
                               start_new_session=True)
    try:
        return process.wait(timeout=timeout)
    except subprocess.TimeoutExpired:
        return 124
    finally:
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        process.wait()


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--controls-only", action="store_true")
    args = parser.parse_args(argv)
    try:
        pin = json.loads(Path(__file__).with_name("tool-pins.json").read_text())["gitleaks"]
        binary = args.binary.resolve(strict=True)
        if hashlib.sha256(binary.read_bytes()).hexdigest() != pin["binary_sha256"]:
            raise ValueError("Gitleaks executable differs from the reviewed pin.")
        with tempfile.TemporaryDirectory(prefix="nevrotick-secret-controls-") as folder:
            scratch = Path(folder)
            ignore = scratch / "empty-ignore"
            ignore.write_text("")
            environment = {"PATH": "/usr/bin:/bin", "HOME": folder,
                           "XDG_CONFIG_HOME": folder, "LANG": "C.UTF-8",
                           "GOMAXPROCS": "12"}
            flags = ["--config", str(ROOT / ".gitleaks.toml"), "--redact=100",
                     "--ignore-gitleaks-allow", "--gitleaks-ignore-path", str(ignore),
                     "--max-target-megabytes=0", "--max-decode-depth=3",
                     "--no-banner", "--log-level=error", "--exit-code=1"]
            control = scratch / "fixture"
            control.mkdir()
            sample = control / "configuration.txt"
            sample.write_text("token = harmless-placeholder\n")
            command = [str(binary), "dir", *flags, str(control)]
            if run(command, environment, timeout=30) != 0:
                raise ValueError("Healthy secret-scan control failed.")
            sample.write_text("token = " + "ghp_" + "AbCd0123" * 4 + "EfGh\n")
            if run(command, environment, timeout=30) != 1:
                raise ValueError("Synthetic secret was not detected with the intended exit code.")
            sample.write_text("token = harmless-placeholder\n")
            if run(command, environment, timeout=30) != 0:
                raise ValueError("Restored healthy secret-scan control failed.")
            docs = control / "docs"
            docs.mkdir()
            prose = ("| Native control mapping | CTRL-KEYS: key inventory, access separation, "
                     "rekey/disposal and no plaintext secret outputs; any persisted association "
                     "requires explicit protected-storage policy. |")
            document = docs / "asvs-5.0.0-applicability.md"
            document.write_text("Heading\n" + prose + "\n")
            if run(command, environment, timeout=30) != 0:
                raise ValueError("Reviewed exact-prose disposition did not match.")
            generic_secret = "api_" + "key = " + "AbCd0123" * 4 + "EfGh5678"
            for text in (prose + "\n" + generic_secret, prose + " " + generic_secret):
                document.write_text(text + "\n")
                if run(command, environment, timeout=30) != 1:
                    raise ValueError("Prose exception hid a real token in the same document.")
            document.write_text(prose + "\n")
            unrelated = control / "unrelated.md"
            unrelated.write_text(prose + "\n")
            if run(command, environment, timeout=30) != 1:
                raise ValueError("Prose exception escaped its document scope.")
            unrelated.unlink()
            # Independently reviewed public prose, a recomputed source digest,
            # and a public u64 wire fixture are not credentials. Construct the
            # trigger words in pieces so these definitions do not become new
            # scanner findings; the complete payload is challenged below.
            reviewed_cases = (
                ("tasks/s0.5-ui-comparison.md",
                 "Independent audit recomputed every selected statistic, "
                 "sample admission, " + "to" + "ken,\n"
                 "geometry/focus and identity check. All six selected trials "
                 "closed normally with"),
                ("docs/windows-time-architecture.md",
                 "For a later selected OpenPGP MSI alternative, admission "
                 "also needs current trusted-" + "ke" + "y distribution, "
                 "revocation/expiry"),
                ("docs/windows-time-architecture.md",
                 "OpenPGP admission also needs current trusted-" + "ke" +
                 "y distribution, revocation/expiry"),
                ("tasks/s1.2-worker-concurrency-pilot.json",
                 '    ".github/security/check_' + "se" + 'crets.py": "'
                 "80994c615da3f41d24596f5b6af6ba52733f6aec711314ea171fcf4e585b0d0d"
                 '",'),
                ("crates/ntp-exchange/tests/association.rs",
                 "    let to" + "ken = 0x0f1e_2d3c_4b5a_6978_u64;"),
                ("scripts/native_input_source.py",
                 'INPUT_PRODUCTION_TO' + 'KENS_SHA256 = "'
                 'e3afe117f4b5ef1ee13cfeb9be376aa5c3da67c88e6d31ed50523c394a87cba8"'),
                ("policy/slint-linux-sdk.json",
                 '    "usr/include/freetype2/freetype/ft' + 'modapi.h": "'
                 '3b05e210b4705017d68e00def6820749fd0fca2f185ab2bb5df06d71928e29eb",'),
            )
            for relative, text in reviewed_cases:
                reviewed = control / relative
                reviewed.parent.mkdir(parents=True, exist_ok=True)
                for prefix in ("", "Heading\n"):
                    reviewed.write_text(prefix + text + "\n")
                    if run(command, environment, timeout=30) != 0:
                        raise ValueError("Reviewed public-text disposition did not match.")
                lines = text.splitlines()
                challenges = [text + "\n" + generic_secret]
                for index in range(len(lines)):
                    modified = lines.copy()
                    modified[index] += " " + generic_secret
                    challenges.append("\n".join(modified))
                for challenge in challenges:
                    reviewed.write_text(challenge + "\n")
                    if run(command, environment, timeout=30) != 1:
                        raise ValueError("Public-text exception hid an adjacent or same-line token.")
                reviewed.unlink()
                unrelated.write_text(text + "\n")
                if run(command, environment, timeout=30) != 1:
                    raise ValueError(f"Public-text exception escaped its exact path: {relative}.")
                unrelated.unlink()
                reviewed.write_text(text + "\n")
                if run(command, environment, timeout=30) != 0:
                    raise ValueError("Restored public-text disposition control failed.")
                reviewed.unlink()
            print("Gitleaks: healthy, synthetic-secret, restored and exact-text disposition controls passed.")
            if not args.controls_only:
                for name, invocation in (
                    ("full retained history", [str(binary), "git", "--log-opts=--all", *flags, str(ROOT)]),
                    ("working tree", [str(binary), "dir", *flags, str(ROOT)]),
                ):
                    result = run(invocation, environment)
                    if result != 0:
                        raise ValueError(f"Gitleaks {name} scan returned {result}; findings remain blocking.")
                    print(f"Gitleaks: {name} scan passed (redacted, no baseline).")
    except (OSError, ValueError, KeyError, subprocess.SubprocessError) as error:
        print(f"FAILED: {error}", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
