"""Prepare a sealed Cargo source correspondence; execute no tool or dependency.

This is not a Cargo-mode admission implementation. A generated offline source
pointer and registry lock pin do not admit the vendor tree, build scripts, proc
macros, Cargo, compiler, allocator model or any unwinding configuration.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path
import re
import tomllib


PACKAGE_NAME = "nevrotick-era-constructor-proof"
REGISTRY = "registry+https://github.com/rust-lang/crates.io-index"
DEFAULT_VENDOR_ROOT = Path(
    "/home/david_osipov/.local/share/nevrotick/s12e-composition-20261030T082648Z/"
    "s24-integrated-engine-dependency-check-v2/vendor"
)
SOURCE_SHA256 = {
    "Cargo.toml": "2410193ccead664712fe69ee36c3ba0f88fe5b0d61aedcba65ad668a1844083b",
    "Cargo.lock": "416b349d87cc0e53562d79d494316fd20c210ede2e45c2568f5973652753ec8f",
    "policy/ui-m0-dependencies.json": "7fd786ea6e874c518405e4b7c2c0a64bea932f42dff52bd3e8a37801ed911f93",
    "crates/time-domain/Cargo.toml": "e0cb947e288754b7a85d3ca059f048d931bfca6d1264b7dd472fdb62569163ac",
    "crates/time-domain/src/era.rs": "366397df0c6bcd260d566b5bac310c882796734887af886a5fdf99a023bb1dc8",
    "crates/time-domain/src/coordinate.rs": "63f7230f915491973d4d682ee66afa5dc0c22bd8bc38fcc375d1840b3c046e80",
    "crates/time-domain/src/lib.rs": "a6b697df2267e6e72f99df1bcb9b0798dada3af11fbbaa24160415ae73b0861b",
}
ERROR_ENUM_SHA256 = "a63f855634262baf2586e21312378f935a52e9e0ea02e77805ffd1083c01fd35"
GRAPH_VERSIONS = {
    "castaway": "0.2.4", "cfg-if": "1.0.5", "compact_str": "0.9.1",
    "garde": "0.23.0", "garde_derive": "0.23.0", "itoa": "1.0.18",
    "proc-macro2": "1.0.107", "quote": "1.0.47", "rustversion": "1.0.23",
    "ryu": "1.0.23", "smallvec": "1.16.4", "static_assertions": "1.1.0",
    "syn": "2.0.119", "unicode-ident": "1.0.27",
}
LICENSES = {
    "castaway": "MIT", "compact_str": "MIT", "ryu": "Apache-2.0 OR BSL-1.0",
    "unicode-ident": "(MIT OR Apache-2.0) AND Unicode-3.0",
}
BUILD_SCRIPTS = {"proc-macro2": "build.rs", "quote": "build.rs", "rustversion": "build/build.rs"}
PROC_MACROS = ("garde_derive", "rustversion")
GARDE_DEPENDENCY = {"version": "=0.23.0", "default-features": False, "features": ["derive"]}
CONTROLS = {
    "reject_singleton": {
        "before": "    earliest <= latest\n", "after": "    earliest < latest\n",
        "harness": "proof::era_constructor_control_singleton",
        "intended_failure": "singleton public window admitted",
    },
    "swap_retained_endpoints": {
        "before": "            earliest: UtcCoordinateQ32::new(earliest)?,\n"
                  "            latest: UtcCoordinateQ32::new(latest)?,",
        "after": "            earliest: UtcCoordinateQ32::new(latest)?,\n"
                 "            latest: UtcCoordinateQ32::new(earliest)?,",
        "harness": "proof::era_constructor_control_retention",
        "intended_failure": "ordered lower endpoint retained",
    },
    "wrong_validation_error": {
        "before": ".map_err(|_| DomainError::InvalidEraWindow)?;",
        "after": ".map_err(|_| DomainError::CoordinateOutOfRange)?;",
        "harness": "proof::era_constructor_control_error",
        "intended_failure": "invalid endpoint mapped to public window error",
    },
}
REQUIRED_ASSERTIONS = {
    "proof::era_constructor_full_domain": (
        "public window admission iff full raw contract", "exact earliest coordinate retained",
        "exact latest coordinate retained", "constructed endpoints remain ordered",
        "every raw constructor rejection is InvalidEraWindow",
    ),
}
REQUIRED_COVERS = {
    "proof::era_constructor_full_domain": (
        "zero singleton is admitted", "minimum singleton is admitted", "maximum singleton is admitted",
        "complete numeric window is admitted", "inverted valid neighbors rejected",
        "only lower endpoint fails Garde", "only upper endpoint fails Garde",
        "both endpoint report entries are reachable", "raw i128 extremes are rejected",
    ),
}
KANI_SOURCE_BASE = "https://github.com/model-checking/kani/blob/kani-0.68.0/kani-driver/src/"
KANI_SOURCE_EVIDENCE = {
    "version": "0.68.0", "ref": "kani-0.68.0",
    "evidence_status": "Official version-tag source inspected; not a native Cargo-mode qualification or binary/source attestation.",
    "compile_only_argument": KANI_SOURCE_BASE + "args/mod.rs#L305-L307",
    "compile_only_skips_verification": KANI_SOURCE_BASE + "main.rs#L110-L111",
    "compile_only_rejects_json_export": KANI_SOURCE_BASE + "args/mod.rs#L809-L819",
    "cargo_mode_argv1_dispatch": KANI_SOURCE_BASE + "main.rs#L220-L229",
    "manifest_and_lib_arguments": KANI_SOURCE_BASE + "args/cargo.rs#L14-L106",
    "release_private_cargo": KANI_SOURCE_BASE + "session.rs#L423-L425",
    "cargo_rustc_and_external_target": KANI_SOURCE_BASE + "call_cargo.rs#L128-L177",
    "compile_only_candidate_argv": ["<admitted-kani-driver>", "kani", "--manifest-path",
                                    "<sealed-project>/Cargo.toml", "--lib", "--only-codegen",
                                    "--target-dir", "<owned-work>/target-kani"],
    "candidate_argv_executed": False,
    "candidate_environment_not_yet_admitted": {
        "CARGO_NET_OFFLINE": "true", "CARGO_BUILD_JOBS": "2",
        "must_remove_inherited": ["RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS"],
    },
}
PREREQUISITES = (
    "Admit a dedicated Cargo-mode Kani runner, its actual Cargo/compiler identity and argument transport.",
    "Do not reuse standalone RUSTFLAGS --edition: Cargo owns each package's edition.",
    "Bind and admit all selected vendor source files and metadata; package checksum labels alone are insufficient.",
    "Qualify real garde_derive/rustversion proc macros and proc-macro2/quote/rustversion build scripts.",
    "Keep the lock/config immutable and offline; pinned Kani Cargo mode has no admitted --locked/--frozen forwarding.",
    "Keep Cargo output, caches and target directories outside this read-only sealed source tree.",
    "Design a selected vendor namespace closure; the configured whole vendor parent is not a mount entitlement.",
    "Run separately budgeted code-generation-only preflight without an incompatible JSON-export flag.",
    "Inspect emitted Report/Vec/Path/Arc/CompactString allocator/drop loops before proposing an unwind profile.",
    "Unwind 1 is unqualified here; any finite bound needs successful unwinding checks over the full input domain.",
    "Retain all memory, overflow, sanity and unwinding checks; do not add assumptions to hide rejected inputs.",
    "Pinned Kani assumes successful allocation (--no-malloc-may-fail); no host OOM recovery is proved.",
    "Freeze reviewed theorem, graph, compiler, solver and profile before healthy/defect/restored native execution.",
    "Observe exact Cargo-mode harness metadata before using candidate fully qualified filter names.",
)


def digest(data: bytes) -> str:
    """Hash exact bytes without normalizing executable source or lock records."""
    return hashlib.sha256(data).hexdigest()


def schedule() -> list[dict]:
    """Describe five future commands; this module admits or executes none."""
    jobs = [{"subject": "era_constructor", "case": "healthy",
             "harness": "proof::era_constructor_full_domain", "intended_failure": None,
             "restored_healthy": False}]
    jobs.extend({"subject": "era_constructor", "case": case, "harness": control["harness"],
                 "intended_failure": control["intended_failure"], "restored_healthy": False}
                for case, control in CONTROLS.items())
    jobs.append({**jobs[0], "restored_healthy": True})
    return jobs


def selected_registry_graph(lock_bytes: bytes) -> tuple[list[dict], bytes]:
    """Copy the complete 14-package locked Garde closure, not a Cargo resolution."""
    lock = tomllib.loads(lock_bytes.decode())
    if lock.get("version") != 4:
        raise ValueError("Unreviewed lock format")
    packages = {}
    blocks = {}
    for block in lock_bytes.split(b"[[package]]\n")[1:]:
        package = tomllib.loads("[[package]]\n" + block.decode())["package"][0]
        name = package["name"]
        if name in packages:
            raise ValueError("Ambiguous locked package: " + name)
        packages[name] = package
        blocks[name] = b"[[package]]\n" + block
    selected, pending = set(), ["garde"]
    while pending:
        name = pending.pop()
        if name in selected:
            continue
        if name not in packages:
            raise ValueError("Missing locked dependency: " + name)
        selected.add(name)
        pending.extend(dependency.split()[0] for dependency in packages[name].get("dependencies", []))
    if {name: packages[name]["version"] for name in selected} != GRAPH_VERSIONS:
        raise ValueError("Garde registry closure differs from reviewed 14-package graph")
    graph = []
    for name in sorted(selected):
        package = packages[name]
        if package.get("source") != REGISTRY or not re.fullmatch("[0-9a-f]{64}", package.get("checksum", "")):
            raise ValueError("Unpinned registry source: " + name)
        graph.append({**package, "license": LICENSES.get(name, "MIT OR Apache-2.0"),
                      "build_script": BUILD_SCRIPTS.get(name), "proc_macro": name in PROC_MACROS})
    # Ordinary offline metadata still rewrites noncanonical lock serialization.
    # Render Cargo's format without executing it or changing the locked graph.
    root_block = (f'[[package]]\nname = "{PACKAGE_NAME}"\nversion = "0.0.0"\n'
                  'dependencies = [\n "garde",\n]\n').encode()
    rendered = {name: blocks[name].rstrip(b"\n") for name in selected}
    rendered[PACKAGE_NAME] = root_block.rstrip(b"\n")
    generated = (b"# This file is automatically @generated by Cargo.\n"
                 b"# It is not intended for manual editing.\nversion = 4\n\n"
                 + b"\n\n".join(rendered[name] for name in sorted(rendered)) + b"\n")
    parsed = tomllib.loads(generated.decode())
    if len(parsed["package"]) != 15:
        raise ValueError("Generated dedicated lock has an unexpected package count")
    return graph, generated


def observe_vendor_metadata(vendor_root: Path, graph: list[dict]) -> dict:
    """Observe small metadata only; do not admit or hash the whole vendor payload."""
    if not vendor_root.is_absolute() or vendor_root.resolve(strict=True) != vendor_root:
        raise ValueError("Vendor metadata root must be canonical and absolute")
    observations = []
    for package in graph:
        directory = vendor_root / (package["name"] + "-" + package["version"])
        manifest_path, checksums_path = directory / "Cargo.toml", directory / ".cargo-checksum.json"
        if directory.is_symlink() or manifest_path.is_symlink() or checksums_path.is_symlink():
            raise ValueError("Vendor metadata aliases are not accepted")
        manifest, checksum_bytes = manifest_path.read_bytes(), checksums_path.read_bytes()
        candidate, checksums = tomllib.loads(manifest.decode()), json.loads(checksum_bytes)
        declared = candidate["package"]
        if (declared.get("name"), declared.get("version"), declared.get("license")) != (
                package["name"], package["version"], package["license"]):
            raise ValueError("Vendor package identity/license differs from locked graph")
        if (checksums.get("package") != package["checksum"]
                or checksums.get("files", {}).get("Cargo.toml") != digest(manifest)):
            raise ValueError("Vendor package checksum metadata mismatch")
        if (declared.get("build", False) != (package["build_script"] or False)
                or bool(candidate.get("lib", {}).get("proc-macro", False)) != package["proc_macro"]):
            raise ValueError("Vendor executable build inputs differ from reviewed graph")
        observations.append({"name": package["name"], "version": package["version"],
                             "manifest_sha256": digest(manifest),
                             "checksum_metadata_sha256": digest(checksum_bytes)})
    return {"root": str(vendor_root), "metadata_observed": True, "packages": observations,
            "payload_files_hashed": False, "admitted": False,
            "limit": "Self-consistent metadata is not a trusted whole-source inventory or execution admission."}


def prepare_era_constructor(source_root: Path, inputs: Path, case: str,
                            harness_path: Path, *, vendor_root: Path | None = None) -> dict:
    """Seal actual modules and a dedicated offline Cargo candidate, without execution."""
    if case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown public era constructor control")
    if inputs.exists() or inputs.is_symlink():
        raise ValueError("Refusing to overwrite an existing Cargo preparation")
    originals = {name: (source_root / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    workspace = tomllib.loads(originals["Cargo.toml"].decode())["workspace"]["package"]
    domain_manifest = tomllib.loads(originals["crates/time-domain/Cargo.toml"].decode())
    if domain_manifest.get("dependencies") != {"garde": GARDE_DEPENDENCY}:
        raise ValueError("Direct Garde dependency/features changed")
    graph, lock = selected_registry_graph(originals["Cargo.lock"])
    policy = json.loads(originals["policy/ui-m0-dependencies.json"])
    if policy.get("lockfile_sha256") != SOURCE_SHA256["Cargo.lock"]:
        raise ValueError("Retained dependency policy does not bind the current lock")
    inventory = {package["name"]: package for package in policy["packages"]}
    expected_features = policy["rows"]["x86_64-unknown-linux-gnu"]["features"]
    for package in graph:
        provenance = inventory[package["name"]]
        if any(package[key] != provenance[key] for key in ("version", "checksum", "license")):
            raise ValueError("Retained dependency inventory differs from constructor graph")
        package["retained_inventory"] = {key: provenance[key] for key in
                                          ("files_sha256", "upstream_revision", "selected_license")}
        package["expected_features"] = expected_features[package["name"] + "@" + package["version"]]
    configured_vendor = vendor_root if vendor_root is not None else DEFAULT_VENDOR_ROOT
    if (not configured_vendor.is_absolute()
            or any(ord(char) < 32 or 127 <= ord(char) <= 159 for char in str(configured_vendor))):
        raise ValueError("Vendor source pointer must be an absolute printable path")
    observation = (observe_vendor_metadata(configured_vendor, graph) if vendor_root is not None else
                   {"root": str(configured_vendor), "metadata_observed": False,
                    "payload_files_hashed": False, "admitted": False})
    subject = originals["crates/time-domain/src/era.rs"]
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if subject.count(before) != 1:
            raise ValueError("Reviewed constructor mutation point is not unique")
        subject = subject.replace(before, after, 1)
    library = originals["crates/time-domain/src/lib.rs"]
    first, last = b"/// Fixed redacted domain failures", b"impl std::fmt::Display for DomainError"
    if library.count(first) != 1 or library.count(last) != 1:
        raise ValueError("Actual DomainError extraction is ambiguous")
    error_enum = library[library.index(first):library.index(last)].rstrip(b"\n") + b"\n"
    if digest(error_enum) != ERROR_ENUM_SHA256:
        raise ValueError("Actual DomainError enum differs from reviewed bytes")
    manifest = (
        f'[package]\nname = "{PACKAGE_NAME}"\nversion = "0.0.0"\npublish = false\n'
        f'edition = {json.dumps(workspace["edition"])}\nrust-version = {json.dumps(workspace["rust-version"])}\n'
        f'license = {json.dumps(workspace["license"])}\nrepository = {json.dumps(workspace["repository"])}\n'
        '\n[workspace]\n\n[lib]\npath = "src/lib.rs"\ndoctest = false\n'
        '\n[dependencies]\ngarde = { version = "=0.23.0", default-features = false, features = ["derive"] }\n'
    ).encode()
    config = (
        '[net]\noffline = true\n\n[source.crates-io]\nreplace-with = "formal-vendor"\n'
        '\n[source.formal-vendor]\ndirectory = ' + json.dumps(str(configured_vendor)) + '\n'
    ).encode()
    tomllib.loads(manifest.decode())
    tomllib.loads(config.decode())
    files = {
        "Cargo.toml": manifest, "Cargo.lock": lock, ".cargo/config.toml": config,
        "src/era.rs": subject, "src/coordinate.rs": originals["crates/time-domain/src/coordinate.rs"],
        "src/proof.rs": harness_path.read_bytes(),
        "src/lib.rs": (b'//! Actual-source EraWindow/Garde proof candidate; no authority or provenance.\n'
                       b'#![allow(dead_code, reason = "Complete actual modules retain separate resolver entry points.")]\n'
                       + error_enum + b'\nmod coordinate;\npub use coordinate::UtcCoordinateQ32;\n'
                       b'mod era;\npub use era::EraWindow;\n#[cfg(kani)]\nmod proof;\n'),
    }
    details = {
        "entrypoint": "Cargo.toml", "package": PACKAGE_NAME, "preparation_only": True,
        "original_sources_sha256": dict(SOURCE_SHA256), "complete_era_module_preserved": case == "healthy",
        "complete_coordinate_module_preserved": True, "domain_error_enum_sha256": digest(error_enum),
        "compiled_era_sha256": digest(subject), "mutation": mutation, "registry_graph": graph,
        "registry_graph_status": "Exact current lock closure copied; Cargo resolution/features not freshly observed.",
        "expected_features_source": "Pinned policy/ui-m0-dependencies.json Linux row, restricted to the selected Garde closure.",
        "garde_dependency": dict(GARDE_DEPENDENCY), "vendor": observation,
        "kani_source_evidence": KANI_SOURCE_EVIDENCE,
        "shipping_graph_qualified": False,
        "workspace_lints_and_build_config_preserved": False,
        "configuration_limit": "Dedicated source-correspondence crate retains edition/rust-version/license and the exact Garde dependency; it does not inherit shipping workspace lints, .cargo/config.toml or release profiles.",
        "generated_files_sha256": {name: digest(data) for name, data in files.items()},
        "input_assumptions": [], "cargo_mode_admitted": False, "native_profile_qualified": False,
        "native_commands_executed": 0, "unwind_bound": None, "prerequisites": list(PREREQUISITES),
        "omitted": ["unfold_ntp resolver theorem", "anchor/date-scope provenance", "clock/native authority",
                    "host allocator exhaustion behavior", "full source vendor admission and Cargo-mode execution"],
        "scope": "Actual public EraWindow raw-i128 admission, exact endpoints and required error classification only.",
    }
    files["correspondence.json"] = (json.dumps(details, indent=2) + "\n").encode()
    inputs.mkdir(mode=0o700, parents=True)
    for name, data in files.items():
        destination = inputs / name
        destination.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
        with destination.open("xb") as output:
            output.write(data)
        destination.chmod(0o400)
    for directory in (inputs / "src", inputs / ".cargo", inputs):
        directory.chmod(0o500)
    return {**details, "entrypoint": inputs / "Cargo.toml",
            "correspondence_sha256": digest(files["correspondence.json"])}
