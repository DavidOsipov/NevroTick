#!/usr/bin/env python3
"""Run the admitted UI-M0, wire, time-domain, exchange and synthetic regression rows in an isolated sandbox.

Acquisition is separate. Missing tools, vendor bytes or product tests fail closed.
Windows cross-linking supplies no native UI, service or release qualification.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import socket
import sys
import tempfile
import time
import tomllib
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("product_environment", ROOT / "scripts/check-environment.py")
ENV = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(ENV)
STRICT_SPEC = importlib.util.spec_from_file_location("product_strict", ROOT / "scripts/check-strict.py")
STRICT = importlib.util.module_from_spec(STRICT_SPEC)
STRICT_SPEC.loader.exec_module(STRICT)
PACKAGE = "nevrotick-ui"
MEMBER = "crates/nevrotick-ui"
PACKAGES = {PACKAGE: MEMBER, "ntp-wire": "crates/ntp-wire", "time-domain": "crates/time-domain",
            "ntp-exchange": "crates/ntp-exchange", "time-core": "crates/time-core"}
OPTIMIZED_DISCRIMINATORS = {
    "time-domain": "release_build_preserves_checked_admission",
    "ntp-exchange": "release_build_preserves_exchange_admission",
    "time-core": "release_build_preserves_regression_admission",
}
POINTER = Path.home() / ".local/share/nevrotick/ui-m0.json"
SOURCE_FILES = ("Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "rustfmt.toml", "clippy.toml",
                "deny.toml", ".cargo/config.toml", ".config/nextest.toml")
CHECKER_FILES = ("scripts/verify.py", "scripts/check-environment.py", "scripts/check-strict.py")


def sha256(path: Path) -> str:
    """Hash a regular input without loading its complete content."""
    if path.is_symlink() or not path.is_file():
        raise ENV.Incomplete(f"Expected regular admitted input: {path.name}")
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for data in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(data)
    return digest.hexdigest()


def inventory(directory: Path) -> dict[str, str]:
    """Capture regular tree bytes; symlinks and special files are not admitted."""
    if directory.is_symlink() or not directory.is_dir():
        raise ENV.Incomplete("Missing regular input directory.")
    result = {}
    for path in sorted(directory.rglob("*")):
        if path.is_symlink() or not (path.is_file() or path.is_dir()):
            raise ENV.Incomplete("Linked or special input is not admitted.")
        if path.is_file():
            result[str(path.relative_to(directory))] = sha256(path)
    return result


def admit_product(root: Path) -> tuple[dict, dict[str, str]]:
    """Admit the five reviewed members, policy inheritance and exact locked graph."""
    try:
        STRICT.read_configuration(root)
    except STRICT.ENV.Incomplete as error:
        raise ENV.Incomplete(str(error)) from error
    for member_path in (root / member for member in PACKAGES.values()):
        if (not member_path.resolve().is_relative_to(root.resolve())
                or any(path.is_symlink() for path in (root / "crates", member_path))):
            raise ENV.Incomplete("Linked product ancestors are not admitted.")
    for name in SOURCE_FILES:
        sha256(root / name)
    workspace = tomllib.loads((root / "Cargo.toml").read_text())
    fragment = tomllib.loads((root / "policy/rust-workspace.toml").read_text())
    if workspace.get("workspace", {}).get("members") != list(PACKAGES.values()):
        raise ENV.Incomplete("Expected exactly the reviewed UI-M0, NTP-wire, time-domain, exchange and time-core workspace members.")
    if (workspace["workspace"].get("lints") != fragment["workspace"]["lints"]
            or workspace.get("profile") != fragment["profile"]):
        raise ENV.Incomplete("Workspace lint/profile policy differs from its owner.")
    local_packages = set()
    for package, directory in PACKAGES.items():
        member = tomllib.loads((root / directory / "Cargo.toml").read_text())
        if (member.get("package", {}).get("name") != package
                or member.get("lints") != {"workspace": True}):
            raise ENV.Incomplete("A member identity or inherited lints are missing.")
        if member["package"].get("build") not in (None, False) or (root / directory / "build.rs").exists():
            raise ENV.Incomplete("An unadmitted first-party build script is present.")
        if package == "ntp-wire" and set(member) != {"package", "lints"}:
            raise ENV.Incomplete("The pure NTP-wire member may not add dependencies, features or targets.")
        if package in {"time-domain", "time-core"}:
            required = {"garde": {"version": "=0.23.0", "default-features": False, "features": ["derive"]}}
            if package == "time-core":
                required["time-domain"] = {"version": "=0.0.0", "path": "../time-domain"}
            if (set(member) != {"package", "lints", "dependencies"}
                    or member["package"].get("version") != "0.0.0"
                    or member.get("dependencies") != required
                    or type(member.get("dependencies", {}).get("garde", {}).get("default-features")) is not bool):
                if package == "time-domain":
                    raise ENV.Incomplete("The time-domain member requires only exact garde with the reviewed features.")
                raise ENV.Incomplete("The time-core member requires its exact reviewed garde/local dependency graph.")
        if package == "ntp-exchange":
            required = {"ntp-wire": {"version": "=0.0.0", "path": "../ntp-wire"},
                        "time-domain": {"version": "=0.0.0", "path": "../time-domain"}}
            required_dev = {"time-core": {"version": "=0.0.0", "path": "../time-core"}}
            if (set(member) != {"package", "lints", "dependencies", "dev-dependencies"}
                    or member["package"].get("version") != "0.0.0"
                    or member.get("dependencies") != required
                    or member.get("dev-dependencies") != required_dev):
                raise ENV.Incomplete("The ntp-exchange member requires only exact local ntp-wire and time-domain normal dependencies and the time-core dev dependency.")
        local_packages.add((package, member["package"]["version"]))
    policy = json.loads((root / "policy/ui-m0-dependencies.json").read_text())
    if policy.get("schema_version") != 1 or policy.get("lockfile_sha256") != sha256(root / "Cargo.lock"):
        raise ENV.Incomplete("The product lockfile differs from its dependency admission.")
    manifests = {name: sha256(root / name)
                 for name in ("Cargo.toml", *(member + "/Cargo.toml" for member in PACKAGES.values()))}
    if policy.get("manifests_sha256") != manifests:
        raise ENV.Incomplete("Product manifest/features differ from the reviewed dependency admission.")
    expected = {(p["name"], p["version"], p["checksum"]) for p in policy["packages"]}
    if not expected or len(expected) != len(policy["packages"]):
        raise ENV.Incomplete("Dependency admission must contain unique nonempty packages.")
    packages = tomllib.loads((root / "Cargo.lock").read_text()).get("package", [])
    external = {(p["name"], p["version"], p.get("checksum")) for p in packages if "source" in p}
    local = [p for p in packages if "source" not in p]
    if (external != expected or len(local) != len(local_packages)
            or {(p["name"], p["version"]) for p in local} != local_packages
            or any(p.get("dependencies") for p in local if p["name"] == "ntp-wire")
            or any(p.get("dependencies") != ["garde"] for p in local if p["name"] == "time-domain")
            or any(p.get("dependencies") != ["garde", "time-domain"] for p in local if p["name"] == "time-core")
            # Cargo.lock combines dependency kinds; the manifest guard above owns their roles.
            or any(p.get("dependencies") != ["ntp-wire", "time-core", "time-domain"] for p in local if p["name"] == "ntp-exchange")
            or not any(p["name"] == "garde" and p["version"] == "0.23.0" for p in packages if "source" in p)):
        raise ENV.Incomplete("The lock graph contains missing or unexpected packages.")
    if any(p.get("source") != "registry+https://github.com/rust-lang/crates.io-index" for p in packages if "source" in p):
        raise ENV.Incomplete("Unadmitted dependency origin.")
    inputs = {name: sha256(root / name) for name in SOURCE_FILES}
    for member in PACKAGES.values():
        for name, digest in inventory(root / member).items():
            if (name.endswith("Cargo.toml") and name != "Cargo.toml") or ".cargo" in Path(name).parts:
                raise ENV.Incomplete("Unexpected nested manifest or Cargo configuration.")
            inputs[f"{member}/{name}"] = digest
        if f"{member}/src/lib.rs" not in inputs:
            raise ENV.Incomplete("A required product library is absent.")
    for name in (*STRICT.CONFIG_FILES, *CHECKER_FILES, "policy/ui-m0-dependencies.json", "policy/tool-manifest.json"):
        inputs[name] = sha256(root / name)
    return policy, inputs


def product_deny(base: str, policy: dict) -> str:
    """Add only this reviewed graph's license grants and exact build-script versions."""
    parsed = tomllib.loads(base)
    expressions = {entry["selected_license"] for entry in policy["packages"]}
    if expressions != {"BSL-1.0", "MIT", "MIT AND Unicode-3.0"}:
        raise ENV.Incomplete("The product license/build-script admission needs a reviewed policy change.")
    # Preserve the complete conjunctive unicode-ident grant; neither operand
    # alone is this package's selected expression.
    licenses = ["BSL-1.0", "MIT", "Unicode-3.0"]
    # cargo-deny 0.20.2 PackageSpec uses '@' for an exact version. '=' alone
    # remains part of the crate name and would reject the reviewed scripts.
    builds = sorted(f"{entry['name']}@{entry['version']}" for entry in policy["packages"] if entry.get("build_script"))
    if builds != ["proc-macro2@1.0.107", "quote@1.0.47", "rustversion@1.0.23"]:
        raise ENV.Incomplete("The product license/build-script admission needs a reviewed policy change.")
    if parsed["licenses"]["allow"] != ["GPL-2.0-only"] or parsed["bans"]["build"]["allow-build-scripts"]:
        raise ENV.Incomplete("Bootstrap dependency policy was unexpectedly broadened.")
    before = 'allow = ["GPL-2.0-only"]'
    build_before = "allow-build-scripts = []"
    if base.count(before) != 1 or base.count(build_before) != 1:
        raise ENV.Incomplete("Dependency policy generation anchors changed.")
    result = base.replace(before, "allow = " + json.dumps(["GPL-2.0-only", *licenses]), 1)
    result = result.replace(build_before, "allow-build-scripts = " + json.dumps(builds), 1)
    parsed["licenses"]["allow"] = ["GPL-2.0-only", *licenses]
    parsed["bans"]["build"]["allow-build-scripts"] = builds
    if tomllib.loads(result) != parsed:
        raise ENV.Incomplete("Product policy generation changed an unrelated dependency rule.")
    return result


def product_nextest(base: str) -> str:
    """Put nextest's separate store in owned output without changing test policy."""
    expected = tomllib.loads(base)
    if "store" in expected:
        raise ENV.Incomplete("Unexpected nextest store override in the admitted base policy.")
    # nextest's store is relative to the workspace, independently of Cargo's
    # CARGO_TARGET_DIR. An absolute store keeps the source snapshot read-only.
    result = base + '\n[store]\ndir = "/work/target/nextest"\n'
    expected["store"] = {"dir": "/work/target/nextest"}
    if tomllib.loads(result) != expected:
        raise ENV.Incomplete("Nextest storage configuration changed test policy.")
    return result


def admit_vendor(directory: Path, policy: dict) -> dict[str, str]:
    """Verify each vendored file against its reviewed package and Cargo checksum map."""
    wanted = {f"{p['name']}-{p['version']}": p for p in policy["packages"]}
    if not directory.is_dir() or {p.name for p in directory.iterdir()} != set(wanted):
        raise ENV.Incomplete("Vendor directory has missing or unexpected packages.")
    for name, admission in wanted.items():
        package = directory / name
        files = inventory(package)
        record = json.loads((package / ".cargo-checksum.json").read_text())
        if record.get("package") != admission["checksum"]:
            raise ENV.Incomplete("Vendor archive identity differs from the admitted lock graph.")
        files.pop(".cargo-checksum.json", None)
        if not files or files != record.get("files"):
            raise ENV.Incomplete("Vendor source bytes differ from their checksum inventory.")
        map_hash = hashlib.sha256(json.dumps(files, sort_keys=True, separators=(",", ":")).encode()).hexdigest()
        if admission.get("files_sha256") != map_hash:
            raise ENV.Incomplete("Vendor checksum map differs from the reviewed archive extraction.")
    return inventory(directory)


def admit_tools(root: Path, toolchain: Path, extension: Path | None, lane: str) -> dict:
    """Hash selected tools before execution; qualify host isolation separately."""
    if sys.platform != "linux" or os.geteuid() == 0:
        raise ENV.Incomplete("Product verification requires an ordinary Linux/WSL user.")
    policy = json.loads((root / "policy/tool-manifest.json").read_text())
    pinned = tomllib.loads((root / "rust-toolchain.toml").read_text())["toolchain"]
    if pinned["channel"] != policy["rust"]["channel"]:
        raise ENV.Incomplete("Compiler pins disagree.")
    identities = {}
    for name in ("cargo", "rustc", "rustdoc", "rustfmt", "cargo-fmt", "cargo-clippy", "clippy-driver"):
        identities[name] = ENV.require_admitted_binary(toolchain / "bin" / name,
                                                      policy["rust"]["linux_binary_sha256"][name], name)
    if lane in {"portable", "dependencies"}:
        name = "cargo-nextest" if lane == "portable" else "cargo-deny"
        if extension is None:
            raise ENV.Incomplete(f"The pinned {name} is missing.")
        entry = next(p for p in policy["tools"] if p["name"] == name)
        identities[name] = ENV.require_admitted_binary(extension, entry["binary_sha256"], name)
    for name in ("bwrap", "prlimit", "cc", "ld", "python3"):
        found = shutil.which(name)
        if not found:
            raise ENV.Incomplete(f"Required host capability is missing: {name}")
        identities[name] = sha256(Path(found).resolve())
    return identities


def commands(lane: str) -> list[tuple[str, list[str]]]:
    """Select explicit product commands; never replace execution by bootstrap controls."""
    cargo = ["cargo", "--config", 'source.crates-io.replace-with="reviewed"',
             "--config", 'source.reviewed.directory="/vendor"']
    base = ["--workspace", "--frozen", "--jobs", "12"]
    if lane == "portable":
        return [
            ("format", ["cargo", "fmt", "--all", "--", "--check"]),
            ("compiler", [*cargo, "check", *base, "--lib", "--tests"]),
            ("clippy", [*cargo, "clippy", *base, "--lib", "--tests", "--", "-D", "warnings"]),
            ("rustdoc", [*cargo, "doc", *base, "--lib", "--no-deps"]),
        ] + [
            ("product-tests:" + package, [*cargo, "nextest", "run", "--package", package,
                                         "--frozen", "--build-jobs", "12", "--lib", "--tests",
                                         "--user-config-file", "none", "--no-tests=fail"])
            for package in PACKAGES
        ] + [("doctests:" + package, [*cargo, "test", "--package", package,
                                      "--frozen", "--jobs", "12", "--doc"])
             for package in PACKAGES] + [
            ("optimized-tests:" + package, [*cargo, "nextest", "run", "--package", package,
                                           "--cargo-profile", "release", "--frozen", "--build-jobs", "12",
                                           "--lib", "--tests", "--user-config-file", "none", "--no-tests=fail"])
            for package in OPTIMIZED_DISCRIMINATORS]
    if lane == "windows":
        target = [*base, "--target", ENV.WINDOWS_TARGET]
        return [("windows-compiler", [*cargo, "check", *target, "--all-targets"]),
                ("windows-clippy", [*cargo, "clippy", *target, "--all-targets", "--", "-D", "warnings"]),
                 ("windows-link", [*cargo, "--config", f'target.{ENV.WINDOWS_TARGET}.linker="/work/linker"',
                                   "build", "--release", "--package", PACKAGE, "--frozen", "--jobs", "12",
                                   "--target", ENV.WINDOWS_TARGET, "--bin", PACKAGE])]
    if lane == "dependencies":
        return [("dependency-" + target, ["/tools/cargo-deny", "--frozen", "--target", target,
                                         "check", "--deny", "warnings"])
                for target in ("x86_64-unknown-linux-gnu", ENV.WINDOWS_TARGET)]
    raise ENV.Incomplete("Unknown product verification row.")


def export_windows_artifacts(work: Path, output: Path) -> dict:
    """Export the release executable and its unique matching PDB to fresh output.

    Admit both regular files before copying; missing/ambiguous symbols or existing
    destinations fail. Cargo may use the binary name or normalized crate name for
    the PDB. This records cross-linked artifacts, not native Windows execution.
    """
    build = work / "target" / ENV.WINDOWS_TARGET / "release"
    binary = build / f"{PACKAGE}.exe"
    names = {f"{PACKAGE}.pdb", f"{PACKAGE.replace('-', '_')}.pdb"}
    symbols = [build / name for name in sorted(names) if (build / name).is_file()]
    if len(symbols) != 1:
        raise ENV.Incomplete("Release Windows artifact needs exactly one matching PDB.")
    pdb = symbols[0]
    identities = {source: sha256(source) for source in (binary, pdb)}
    if any((output / source.name).exists() or (output / source.name).is_symlink()
           for source in identities):
        raise ENV.Incomplete("Windows artifact export refuses existing destinations.")
    for source in identities:
        shutil.copyfile(source, output / source.name)
    return {"path": str(output / binary.name), "sha256": identities[binary],
            "profile": "release", "target": ENV.WINDOWS_TARGET,
            "symbols": {"path": str(output / pdb.name), "sha256": identities[pdb]}}


def validate_test_result(lane: str, output: str, work: Path) -> dict | None:
    """Reject empty, wrong-package or skipped behavior despite a zero runner exit."""
    kind, _, package = lane.partition(":")
    if kind not in {"product-tests", "doctests", "optimized-tests"}:
        return None
    if package not in PACKAGES:
        raise ENV.Incomplete("Test accounting identifies an unreviewed package.")
    if kind == "optimized-tests" and package not in OPTIMIZED_DISCRIMINATORS:
        raise ENV.Incomplete("Optimized test accounting identifies an unreviewed package/profile.")
    if kind in {"product-tests", "optimized-tests"}:
        report = work / "target/nextest/default/junit.xml"
        if not report.is_file():
            raise ENV.Incomplete("Product test execution lacks its JUnit accounting.")
        root = ET.parse(report).getroot()
        suites = list(root.iter("testsuite"))
        cases = [case for suite in suites for case in suite.findall("testcase")]
        if (not cases or len(cases) != len(list(root.iter("testcase")))
                or any(suite.get("name") != package
                       and not suite.get("name", "").startswith(package + "::") for suite in suites)
                or any(any(child.tag in {"skipped", "failure", "error"} for child in case) for case in cases)):
            raise ENV.Incomplete("Required product tests are empty, skipped or failed.")
        names = [case.get("name") for case in cases]
        if kind == "optimized-tests" and OPTIMIZED_DISCRIMINATORS[package] not in names:
            raise ENV.Incomplete("Optimized tests did not execute the release admission discriminator.")
        return {"package": package, "executed": len(cases), "test_names": names}
    if kind == "doctests":
        counts = re.findall(r"test result: ok\. (\d+) passed; 0 failed; (\d+) ignored;", output)
        if not counts or sum(int(passed) for passed, _ in counts) == 0 or any(int(ignored) for _, ignored in counts):
            raise ENV.Incomplete("Required doctests did not execute a nonempty passing selection.")
        return {"package": package, "executed": sum(int(passed) for passed, _ in counts)}
    return None


def clear_test_report(lane: str, work: Path) -> None:
    """Remove a prior package's report so it cannot stand in for a missing run."""
    if lane.startswith(("product-tests:", "optimized-tests:")):
        report = work / "target/nextest/default/junit.xml"
        if report.is_symlink():
            raise ENV.Incomplete("A linked product test report is not admitted.")
        report.unlink(missing_ok=True)


def summarize_test_counts(records: list[dict]) -> dict:
    """Require ordinary, doctest and selected optimized evidence before aggregating."""
    counts = {package: {} for package in PACKAGES}
    for record in records:
        kind, _, package = record["lane"].partition(":")
        if kind not in {"product-tests", "doctests", "optimized-tests"}:
            continue
        result = record.get("tests", {})
        if (package not in counts or kind in counts[package] or result.get("package") != package
                or type(result.get("executed")) is not int or result["executed"] <= 0
                or kind == "optimized-tests" and package not in OPTIMIZED_DISCRIMINATORS):
            raise ENV.Incomplete("Per-package product test accounting is incomplete or duplicated.")
        counts[package][kind] = result["executed"]
    for package, result in counts.items():
        required = {"product-tests", "doctests"} | ({"optimized-tests"} if package in OPTIMIZED_DISCRIMINATORS else set())
        if set(result) != required:
            raise ENV.Incomplete("Every package requires ordinary tests/doctests and each selected owner requires optimized tests.")
    return {"by_package": counts,
            "tests_executed": sum(result["product-tests"] for result in counts.values()),
            "doctests_executed": sum(result["doctests"] for result in counts.values()),
            "optimized_tests_executed": sum(counts[package]["optimized-tests"] for package in OPTIMIZED_DISCRIMINATORS)}


def sandbox(toolchain: Path, work: Path, source: Path, vendor: Path,
            command: list[str], extension: Path | None, libraries: Path | None = None,
            extension_name: str = "cargo-nextest") -> list[str]:
    """Mount only reviewed inputs and owned output; keep the source snapshot read-only."""
    invocation = ENV.sandbox_command(toolchain, work, command)
    index = invocation.index("--")
    extra = ["--ro-bind", str(source), "/work/fixture", "--ro-bind", str(vendor), "/vendor",
             "--dir", "/tools", "--setenv", "PATH", "/tools:/toolchain/bin:/usr/bin:/bin"]
    if extension:
        extra += ["--ro-bind", str(extension), "/tools/" + extension_name]
    if libraries:
        extra += ["--ro-bind", str(libraries), "/xwin"]
    return invocation[:index] + extra + invocation[index:]


def boundary_control(run) -> None:
    """Challenge filesystem, inherited environment and host-network isolation before Cargo."""
    with socket.socket() as listener:
        listener.bind(("127.0.0.1", 0))
        listener.listen(1)
        port = listener.getsockname()[1]
        with socket.create_connection(("127.0.0.1", port), timeout=2):
            accepted, _ = listener.accept()
            accepted.close()
        probe = (
            "import os,pathlib,socket; "
            "assert 'HOME' not in os.environ; "
            "assert 'NEVROTICK_SYNTHETIC_CREDENTIAL' not in os.environ; "
            "assert not pathlib.Path('/run/docker.sock').exists(); "
            f"assert not pathlib.Path({str(Path.home())!r}).exists(); "
            "s=socket.socket(); s.settimeout(1)\n"
            "pathlib.Path('/work/output-canary').write_text('owned writable output')\n"
            "pathlib.Path('/work/output-canary').unlink()\n"
            "try: pathlib.Path('/work/fixture/source-write-canary').write_text('must fail')\n"
            "except OSError: pass\n"
            "else: raise SystemExit('source snapshot is writable')\n"
            f"try: s.connect(('127.0.0.1',{port}))\n"
            "except OSError: print('PRODUCT_BOUNDARY_OK')\n"
            "else: raise SystemExit('host network visible')\n"
            "finally: s.close()\n"
        )
        if "PRODUCT_BOUNDARY_OK" not in run("boundary-control", ["python3", "-c", probe]):
            raise ENV.Incomplete("The isolation observer did not complete.")


def library_inventory(directory: Path) -> dict[str, str]:
    """Hash all selected Microsoft libraries, including filenames and directory scope."""
    result = {}
    for suffix in ("crt/lib/x86_64", "sdk/lib/um/x86_64", "sdk/lib/ucrt/x86_64"):
        for name, digest in inventory(directory / suffix).items():
            result[suffix + "/" + name] = digest
    if not result:
        raise ENV.Incomplete("Microsoft library inventory is empty.")
    return result


def windows_libraries(root: Path, toolchain: Path, config_path: Path, work: Path,
                      inventory_path: Path | None) -> tuple[Path, dict[str, str]]:
    """Admit the configured Microsoft input identity and pinned bundled linker."""
    policy = json.loads((root / "policy/tool-manifest.json").read_text())
    config = json.loads(config_path.read_text())
    expected = policy["cross_toolchain"]["microsoft_inputs"]
    for key in ("architecture", "variant", "sdk_version", "crt_version", "channel_manifest_sha256",
                "package_manifest_sha256", "package_manifest_declared_sha256"):
        if config["microsoft_inputs"][key] != expected[key]:
            raise ENV.Incomplete("Microsoft input identity differs from the reviewed cross lane.")
    directory = Path(config["root"]).resolve(strict=True)
    suffixes = ("crt/lib/x86_64", "sdk/lib/um/x86_64", "sdk/lib/ucrt/x86_64")
    if [str(directory / suffix) for suffix in suffixes] != config["library_paths"]:
        raise ENV.Incomplete("Unexpected Microsoft library search paths.")
    for suffix, name in zip(suffixes, ("msvcrt.lib", "kernel32.lib", "ucrt.lib")):
        sha256(directory / suffix / name)
    if inventory_path is None:
        raise ENV.Incomplete("Windows linking requires the admitted complete library inventory.")
    files = library_inventory(directory)
    acquisition = json.loads(inventory_path.read_text())
    if acquisition.get("acquisition_receipt_sha256") != sha256(config_path):
        raise ENV.Incomplete("Microsoft library inventory refers to a different acquisition receipt.")
    if acquisition.get("files") != files:
        raise ENV.Incomplete("Microsoft library bytes differ from the admitted acquisition inventory.")
    relative = f"lib/rustlib/{policy['rust']['host']}/bin/rust-lld"
    ENV.require_admitted_binary(toolchain / relative, policy["cross_toolchain"]["linker"]["binary_sha256"], "rust-lld")
    linker = work / "linker"
    args = [f"/toolchain/{relative}", "-flavor", "link", *("/libpath:/xwin/" + suffix for suffix in suffixes)]
    linker.write_text("#!/usr/bin/python3\nimport os,sys\n" + f"os.execv({args[0]!r}, {args!r} + sys.argv[1:])\n")
    linker.chmod(0o700)
    return directory, files


def admit_advisories(receipt_path: Path | None, now: float, expected_lock: str) -> tuple[Path, dict[str, str]]:
    """Require the separately acquired complete metadata snapshot and its freshness."""
    if receipt_path is None:
        raise ENV.Incomplete("Dependency checks require a current acquisition receipt.")
    receipt = json.loads(receipt_path.read_text())
    if receipt.get("lockfile_sha256") != expected_lock:
        raise ENV.Incomplete("Advisory/index acquisition identifies a different product lockfile.")
    acquired = receipt.get("acquired_unix")
    if (isinstance(acquired, bool) or not isinstance(acquired, (int, float))
            or not 0 <= now - acquired <= 86400):
        raise ENV.Incomplete("Advisory/index acquisition is missing, future-dated or older than 24 hours.")
    cargo_home = Path(receipt["cargo_home"]).resolve(strict=True)
    files = inventory(cargo_home)
    if (not files or files != receipt.get("files") or "config" in files or "config.toml" in files
            or not any(name.startswith("advisory-dbs/") for name in files)
            or not any(name.startswith("registry/index/") for name in files)):
        raise ENV.Incomplete("Advisory/index snapshot bytes or required scope are incomplete.")
    return cargo_home, files


def execute(root: Path, args, report: dict) -> None:
    """Freeze candidate inputs, run the selected row and reject input drift."""
    policy, source_identity = admit_product(root)
    vendor = args.vendor_dir.resolve(strict=True)
    vendor_identity = admit_vendor(vendor, policy)
    toolchain = args.toolchain_dir.resolve(strict=True)
    selected = args.deny if args.lane == "dependencies" else args.nextest if args.lane == "portable" else None
    extension = selected.resolve(strict=True) if selected else None
    report["tools"] = admit_tools(root, toolchain, extension, args.lane)
    report["source_sha256"] = source_identity
    report["lanes"] = []
    scratch = args.scratch_root.resolve(strict=True)
    if scratch == root or scratch.is_relative_to(root):
        raise ENV.Incomplete("Product scratch must be outside the checkout.")
    advisory = admit_advisories(args.advisory_receipt, time.time(), source_identity["Cargo.lock"]) if args.lane == "dependencies" else None
    if advisory:
        report["advisory_receipt_sha256"] = sha256(args.advisory_receipt)
    with tempfile.TemporaryDirectory(prefix="nevrotick-product-", dir=scratch) as temporary:
        temporary = Path(temporary)
        source, work = temporary / "source", temporary / "work"
        source.mkdir()
        work.mkdir()
        if advisory:
            shutil.copytree(advisory[0], work / "cargo-home")
            if inventory(work / "cargo-home") != advisory[1]:
                raise ENV.Incomplete("Advisory/index snapshot changed during copying.")
        else:
            (work / "cargo-home").mkdir()
        (work / "cargo-home/config.toml").write_text(
            '[source.crates-io]\nreplace-with = "reviewed"\n[source.reviewed]\ndirectory = "/vendor"\n')
        (work / "fixture").mkdir()
        for name in source_identity:
            destination = source / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(root / name, destination)
        if {name: sha256(source / name) for name in source_identity} != source_identity:
            raise ENV.Incomplete("Product inputs changed during snapshotting.")
        if args.lane == "dependencies":
            (source / "deny.toml").write_text(product_deny((source / "deny.toml").read_text(), policy))
        if args.lane == "portable":
            config = source / ".config/nextest.toml"
            config.write_text(product_nextest(config.read_text()))
        if args.lane == "windows":
            if args.xwin_inventory is None:
                raise ENV.Incomplete("Windows linking requires the admitted complete library inventory.")
            report["xwin_config_sha256"] = sha256(args.xwin_config)
            report["xwin_inventory_sha256"] = sha256(args.xwin_inventory)
        library_inputs = windows_libraries(root, toolchain, args.xwin_config, work, args.xwin_inventory) if args.lane == "windows" else None
        libraries = library_inputs[0] if library_inputs else None
        if library_inputs:
            report["windows_library_sha256"] = library_inputs[1]
        report["effective_deny_sha256"] = sha256(source / "deny.toml")
        report["effective_nextest_sha256"] = sha256(source / ".config/nextest.toml")
        report["checker_sha256"] = {name: source_identity[name] for name in CHECKER_FILES}
        def run(label, command):
            clear_test_report(label, work)
            result = ENV.run(sandbox(toolchain, work, source, vendor, command, extension, libraries,
                                    "cargo-deny" if args.lane == "dependencies" else "cargo-nextest"),
                             cwd=root, timeout=300,
                             environment={"PATH": "/usr/bin:/bin", "NEVROTICK_SYNTHETIC_CREDENTIAL": "synthetic-control"})
            record = {"lane": label, "command": command, "exit_code": result["exit_code"],
                      "failure": result["failure"], "output": result["output"]}
            report["lanes"].append(record)
            output = ENV.require_ok(result)
            counts = validate_test_result(label, output, work)
            if counts:
                record["tests"] = counts
            print(f"PASSED: {label}", flush=True)
            return output
        boundary_control(run)
        for label, command in commands(args.lane):
            run(label, command)
        if args.lane == "portable":
            report["test_counts"] = summarize_test_counts(report["lanes"])
        if admit_product(root)[1] != source_identity or admit_vendor(vendor, policy) != vendor_identity:
            raise ENV.Incomplete("Source or dependency inputs changed during product verification.")
        if library_inputs and (library_inventory(libraries) != library_inputs[1]
                               or sha256(args.xwin_inventory) != report["xwin_inventory_sha256"]
                               or sha256(args.xwin_config) != report["xwin_config_sha256"]):
            raise ENV.Incomplete("Microsoft library inputs changed during product verification.")
        if advisory:
            if (admit_advisories(args.advisory_receipt, time.time(), source_identity["Cargo.lock"]) != advisory
                    or sha256(args.advisory_receipt) != report["advisory_receipt_sha256"]):
                raise ENV.Incomplete("Advisory/index snapshot changed or expired during verification.")
            report["advisory_freshness"] = "separate acquisition receipt and snapshot verified; at most 24 hours old"
        report["status"] = "passed"
        if args.output_dir:
            output = args.output_dir.resolve()
            if output == root or output.is_relative_to(root):
                raise ENV.Incomplete("Artifact output must be outside the checkout.")
            output.mkdir(mode=0o700)
            if args.lane == "windows":
                report["artifact"] = export_windows_artifacts(work, output)
            (output / "verification.json").write_text(json.dumps(report, indent=2) + "\n")


def main(argv=None) -> int:
    """Resolve explicit/default paths without installing or fetching anything."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", choices=("portable", "windows", "dependencies"), default="portable")
    for name in ("vendor-dir", "scratch-root", "toolchain-dir", "nextest", "deny", "xwin-config", "xwin-inventory",
                 "advisory-receipt", "output-dir", "json"):
        parser.add_argument("--" + name, type=Path)
    args = parser.parse_args(argv)
    report = {"scope": f"UI-M0, NTP-wire, time-domain, exchange and synthetic regression {args.lane} product verification", "status": "incomplete",
              "native_windows_execution": "not performed", "advisory_freshness": "not checked by this row"}
    try:
        defaults = json.loads(POINTER.read_text()) if POINTER.is_file() else {}
        rust = json.loads((ROOT / "policy/tool-manifest.json").read_text())["rust"]
        defaults.setdefault("toolchain_dir", str(Path.home() / ".rustup/toolchains" / (rust["channel"] + "-" + rust["host"])))
        defaults.setdefault("scratch_root", tempfile.gettempdir())
        defaults.setdefault("nextest", shutil.which("cargo-nextest"))
        defaults.setdefault("deny", shutil.which("cargo-deny"))
        defaults.setdefault("xwin_config", str(Path.home() / ".local/share/nevrotick/xwin.json"))
        for key in ("vendor_dir", "scratch_root", "toolchain_dir", "nextest", "deny", "xwin_config", "xwin_inventory", "advisory_receipt"):
            if getattr(args, key) is None and defaults.get(key):
                setattr(args, key, Path(defaults[key]))
        if args.vendor_dir is None:
            raise ENV.Incomplete("No reviewed vendor directory; run the documented separate acquisition first.")
        execute(ROOT, args, report)
        report["status"] = "passed"
    except (ENV.Incomplete, OSError, ValueError, KeyError, TypeError, StopIteration, ET.ParseError) as error:
        report["status"] = "incomplete"
        report["error"] = str(error)
    if args.json:
        try:
            ENV.external_receipt(args.json, ROOT).write_text(json.dumps(report, indent=2) + "\n")
        except (ENV.Incomplete, OSError) as error:
            report["status"], report["error"] = "incomplete", str(error)
    print(f"{report['status'].upper()}: {report['scope']}")
    if report.get("error"):
        print(report["error"], file=sys.stderr)
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
