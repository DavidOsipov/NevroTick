"""Source-only complete source-state constructor correspondence; no tool dispatch.

Unsealed selected vendor bytes are authenticated at observation time. They confer
no execution admission, target-feature observation, allocator model or proof.
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import os
from pathlib import Path
import re
import stat
import sys
import tomllib

ROOT = Path(__file__).resolve().parents[2]
PACKAGE_NAME = "nevrotick-source-state-constructor-proof"
SELF = "tools/formal/source_state_constructor_projection.py"
HARNESS = "tools/formal/source_state_constructor_harness.rs"
TEST = "tools/formal/test_source_state_constructor.py"
BINDING_SOURCE = "tools/formal/python_source_binding.py"
HELPER_SHA256 = {
    BINDING_SOURCE: "8d7bd9276b9322eefe325974f9926d9ecb32275ecae943ae18264f81ee480d4f",
    "tools/formal/era_vendor_admission.py": "6957baf55377544a1c1a8f35dad4dc41c4fd6ffe2fbbfd823ff19f7fbbc35d9e",
    "tools/formal/era_constructor_projection.py": "21390aa4bc2b469eb8cbd9ce1b09e5ba5a6a51326272360678e3d721bc33a079",
    "tools/crypto-aes-siv-witness/fuzz/input_custody.py": "9e9a5848cf1ccdc7782d282e9bfa786ab10b8a4c947b5bfb7b1000321b3c8e62",
}


def digest(data: bytes) -> str:
    """Identify exact source bytes."""
    return hashlib.sha256(data).hexdigest()


def _binding_buffer(path: Path) -> bytes:
    """Bound only this pinned bootstrap read; later reads use SourceBinding."""
    maximum = 2 * 1024**2  # Pinned SourceBinding.MAX_SOURCE_BYTES.
    if (not path.is_absolute() or path != path.resolve(strict=True)
            or any(item.is_symlink() for item in (path, *path.parents))):
        raise ValueError("Binding bootstrap source redirects")
    before_path = path.lstat()
    descriptor = os.open(path, os.O_RDONLY | os.O_CLOEXEC | os.O_NOFOLLOW | os.O_NONBLOCK)
    try:
        before = os.fstat(descriptor)
        if (not stat.S_ISREG(before.st_mode) or before.st_uid != os.geteuid()
                or before.st_nlink != 1 or not 0 <= before.st_size <= maximum
                or (before.st_dev, before.st_ino) != (before_path.st_dev, before_path.st_ino)):
            raise ValueError("Binding bootstrap source is aliased, unowned, special or oversized")
        with os.fdopen(descriptor, "rb", closefd=False) as stream:
            data = stream.read(maximum + 1)
        after, after_path = os.fstat(descriptor), path.lstat()
        fields = ("st_dev", "st_ino", "st_mode", "st_uid", "st_gid", "st_nlink",
                  "st_size", "st_mtime_ns", "st_ctime_ns")
        identity = lambda item: tuple(getattr(item, name) for name in fields)
        if (len(data) != before.st_size or len(data) > maximum
                or identity(before) != identity(after) or identity(before) != identity(after_path)):
            raise ValueError("Binding bootstrap source changed during read")
        return data
    finally:
        os.close(descriptor)


# Bootstrap the existing binding from its exact pinned buffer, never cached pyc.
_binding_path = ROOT / BINDING_SOURCE
_binding_bytes = _binding_buffer(_binding_path)
if digest(_binding_bytes) != HELPER_SHA256[BINDING_SOURCE]:
    raise ValueError("Python source-binding helper changed")
_binding_spec = importlib.util.spec_from_file_location("source_constructor_binding", _binding_path)
_binding_module = importlib.util.module_from_spec(_binding_spec)
exec(compile(_binding_bytes, str(_binding_path), "exec", dont_inherit=True,
             optimize=sys.flags.optimize), _binding_module.__dict__)
BINDING = _binding_module.SourceBinding(ROOT, sys._getframe())
_previous_bytecode = sys.dont_write_bytecode
try:
    sys.dont_write_bytecode = True
    for _name, _expected in HELPER_SHA256.items():
        if digest((ROOT / _name).read_bytes()) != _expected:
            raise ValueError("Pinned Python helper changed: " + _name)
    VENDOR = BINDING.load_module("source_constructor_vendor", ROOT / "tools/formal/era_vendor_admission.py")
finally:
    sys.dont_write_bytecode = _previous_bytecode
DECLARED_PYTHON = tuple(sorted({SELF, *HELPER_SHA256}))
BINDING.verify(DECLARED_PYTHON)
PROJECTION = VENDOR.PROJECTION
DEFAULT_VENDOR_ROOT = PROJECTION.DEFAULT_VENDOR_ROOT

SOURCE_SHA256 = {'Cargo.lock': '416b349d87cc0e53562d79d494316fd20c210ede2e45c2568f5973652753ec8f',
 'Cargo.toml': '2410193ccead664712fe69ee36c3ba0f88fe5b0d61aedcba65ad668a1844083b',
 'crates/time-core/Cargo.toml': '8b51f9fdfb2b65c757a7ad9e9c592340e2873719db14a19fddeb6747b69bcea6',
 'crates/time-core/src/combination.rs': 'd33493baf6505782e6c9061e04a6204d1dc65b1a05f0acf0120dc2831f698326',
 'crates/time-core/src/combination/lifecycle.rs': '6e415208ccfd93ebe976c09c887c8c42090f85bfec313a17c19594a891dc5081',
 'crates/time-core/src/lib.rs': '1c6496d9a4a40f77fec75087dbe1811194fff262117e883f88fad6f90f9bb458',
 'crates/time-core/src/principal.rs': 'cf4962c8d3536a528b7c783543379bdaeaf2e4070355fbeb7921fb5329b63d1d',
 'crates/time-core/src/regression.rs': '52223fcf848927b344673ec810ca0c784045c8e61835a01e9451de236d8f28d4',
 'crates/time-core/src/selection.rs': 'da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e',
 'policy/ui-m0-dependencies.json': '7fd786ea6e874c518405e4b7c2c0a64bea932f42dff52bd3e8a37801ed911f93',
 'rust-toolchain.toml': '2b58ea5089dfdfacf78ae2a5f52dfce89a583339ccc2c7cc0b40e96230dc9ccc'}
SPANS = {'combination_errors': {'following': b'impl fmt::Debug for FixtureCombinationEr'
                                     b'ror {\n    /// Format',
                        'offsets': (4658, 10807),
                        'sha256': '41d9727c848f4c56d15d1485be410a4f2e3161c91d71d7998c497412fc781b10',
                        'source': 'crates/time-core/src/combination.rs',
                        'start': b'/// Supplied uncertainty role, in its constr'
                                 b"uctor's first-er"},
 'constants': {'following': b'\n/// Additive tracking lifecycle failure without'
                            b' raw entries',
               'offsets': (3114, 3310),
               'sha256': '1e17644c506c8ccaae958f5c844f42efc39bf07a24264ba454eb3f77e42a1cb9',
               'source': 'crates/time-core/src/combination/lifecycle.rs',
               'start': b'/// Chrony reachability-history width used only for '
                        b'fixture '},
 'constructor': {'following': b'/// Prepare a nonwrapping next ordinal over '
                              b'the complete raw',
                 'offsets': (18571, 22383),
                 'sha256': '0f3ef73828c9a57a3df4835b63b2d3f385d6b74bd84c8c678d1f497c895db70c',
                 'source': 'crates/time-core/src/combination/lifecycle.rs',
                 'start': b'/// State rules cover the relationship between m'
                          b'aturity, wat'},
 'lifecycle_errors': {'following': b'impl fmt::Debug for FixtureCombinationLi'
                                   b'fecycleError {\n    /',
                      'offsets': (6261, 9644),
                      'sha256': 'a046ca293c8107e68e3272b47397eb4e8542e69b7a3ddf1e1163a26c96e852e2',
                      'source': 'crates/time-core/src/combination/lifecycle.rs',
                      'start': b'/// Explicit uncalibrated fixture parameter '
                               b'whose constructo'},
 'principal_errors': {'following': b'impl fmt::Debug for FixturePrincipalErro'
                                   b'r {\n    /// Emit a f',
                      'offsets': (2988, 7799),
                      'sha256': '62c2cda5cb9499047bab014ccd21e672264d7f110bf454fc9be4dc6ff10ee29e',
                      'source': 'crates/time-core/src/principal.rs',
                      'start': b'/// Explicit parameter whose shared scalar r'
                               b'ule rejected a r'},
 'regression_errors': {'following': b'impl fmt::Display for FitError {\n    ///'
                                    b' Format only fixed t',
                       'offsets': (4900, 9375),
                       'sha256': '1b4afdc9cd09e8d1d7f80e436b5f5eb39fd2502d30c95bee8bbabe50ccf0f7ca',
                       'source': 'crates/time-core/src/regression.rs',
                       'start': b'/// Constructor argument role, in the declar'
                                b'ed first-error p'},
 'selection_errors': {'following': b'/// Implement fixed-prefix formatting an'
                                   b'd leaf error chainin',
                      'offsets': (2840, 3497),
                      'sha256': 'a170a559189752048cff5d05311e826a5fd8a139d867ba9f40ca8f429bf2c1fb',
                      'source': 'crates/time-core/src/selection.rs',
                      'start': b'/// Stage of a checked selection postconditi'
                               b'on failure.\n#[de'}}

FULL_HARNESS = "source_constructor::proof::source_state_constructor_full_domain"
CONTROLS = {
    "strict_penalty": {
        "before": "value.1 <= DISTANT_PENALTY", "after": "value.1 < DISTANT_PENALTY",
        "harness": "source_constructor::proof::source_state_constructor_control_penalty",
        "intended_failure": "constructor boundary success",
    },
    "wrong_retained_penalty": {
        "before": "            penalty,\n            last_reachability_ordinal,\n        })",
        "after": "            penalty: 0,\n            last_reachability_ordinal,\n        })",
        "harness": "source_constructor::proof::source_state_constructor_control_retention",
        "intended_failure": "constructor exact retained penalty",
    },
    "wrong_error": {
        "before": "return Err(FixtureCombinationLifecycleError::Invariant);",
        "after": "return Err(FixtureCombinationLifecycleError::ValidationInvariant);",
        "harness": "source_constructor::proof::source_state_constructor_control_error",
        "intended_failure": "constructor exact invalid error",
    },
    "validation_bypass": {
        "before": "|value: &(u8, u8, u64), (): &()| source_fields_rule(*value)",
        "after": "|value: &(u8, u8, u64), (): &()| { let _ = value; Ok::<(), garde::Error>(()) }",
        "harness": "source_constructor::proof::source_state_constructor_control_validation",
        "intended_failure": "actual derived validation rejects invalid tuple",
    },
}


def schedule() -> list[dict]:
    """Describe the six unexecuted jobs without a command or runner profile."""
    healthy = {"subject": "source_state_constructor", "case": "healthy", "harness": FULL_HARNESS,
               "intended_failure": None, "restored_healthy": False, "compiled": False}
    return [healthy, *({"subject": "source_state_constructor", "case": case,
                        "harness": control["harness"], "intended_failure": control["intended_failure"],
                        "restored_healthy": False, "compiled": False}
                       for case, control in CONTROLS.items()), {**healthy, "restored_healthy": True}]


def _verify_python() -> dict:
    """Verify observed buffers, independent helper pins and current own-file identities."""
    BINDING.verify(DECLARED_PYTHON)
    captured = BINDING.snapshot()
    for name, expected in HELPER_SHA256.items():
        if captured.get(name, {}).get("sha256") != expected:
            raise ValueError("Observed helper differs from pin: " + name)
    return captured


def _read_context(source_root: Path) -> dict:
    source_root = Path(source_root)
    if (not source_root.is_absolute() or source_root != source_root.resolve(strict=True)
            or any(path.is_symlink() for path in (source_root, *source_root.parents))):
        raise ValueError("Source root redirects")
    originals = {}
    for name, expected in SOURCE_SHA256.items():
        data = VENDOR.CUSTODY.read_file(source_root / name, maximum=8 * 1024**2)
        if digest(data) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
        originals[name] = data
    manifest = tomllib.loads(originals["crates/time-core/Cargo.toml"].decode())
    if manifest.get("dependencies") != {
            "garde": PROJECTION.GARDE_DEPENDENCY,
            "time-domain": {"version": "=0.0.0", "path": "../time-domain"}}:
        raise ValueError("Time-core direct graph changed")
    return originals


def _selection(originals: dict) -> dict:
    selected = {}
    for name, span in SPANS.items():
        data = originals[span["source"]]
        first, last = span["offsets"]
        piece = data[first:last]
        if (digest(piece) != span["sha256"] or not piece.startswith(span["start"])
                or not data[last:].startswith(span["following"])):
            raise ValueError("Reviewed original span changed: " + name)
        selected[name] = piece
    if sum(map(len, selected.values())) != 23483:
        raise ValueError("Complete source selection byte count changed")
    errors = b"".join(data for name, data in selected.items() if name.endswith("_errors"))
    if len(re.findall(rb"\bpub enum ", errors)) != 23:
        raise ValueError("Actual complete error enum closure changed")
    return selected


def _graph_and_grants(originals: dict) -> tuple[list, list, bytes]:
    graph, _discarded_era_lock = PROJECTION.selected_registry_graph(originals["Cargo.lock"])
    policy = VENDOR._json(originals["policy/ui-m0-dependencies.json"])
    if policy["lockfile_sha256"] != SOURCE_SHA256["Cargo.lock"]:
        raise ValueError("Policy lock identity differs")
    records = {entry["name"]: entry for entry in policy["packages"]}
    if len(records) != len(policy["packages"]):
        raise ValueError("Duplicate policy grants")
    grants = []
    for package in graph:
        record = records[package["name"]]
        if (any(record[key] != package[key] for key in ("name", "version", "checksum", "license"))
                or type(record["build_script"]) is not bool
                or record["build_script"] != bool(package["build_script"])
                or not re.fullmatch(r"[0-9a-f]{64}", record["files_sha256"])):
            raise ValueError("Policy differs from selected package identity or map pin")
        grants.append({**record, "build_path": package["build_script"],
                       "proc_macro": package["proc_macro"],
                       "expected_features": policy["rows"]["x86_64-unknown-linux-gnu"]["features"][
                           package["name"] + "@" + package["version"]]})
    if len(grants) != 14 or len({(x["name"], x["version"]) for x in grants}) != 14:
        raise ValueError("Expected fourteen unique selected grants")
    blocks = {}
    for block in originals["Cargo.lock"].split(b"[[package]]\n")[1:]:
        parsed = tomllib.loads("[[package]]\n" + block.decode())["package"][0]
        if parsed["name"] in {item["name"] for item in graph}:
            blocks[parsed["name"]] = b"[[package]]\n" + block.rstrip(b"\n")
    blocks[PACKAGE_NAME] = (f'[[package]]\nname = "{PACKAGE_NAME}"\nversion = "0.0.0"\n'
                            'dependencies = [\n "garde",\n]').encode()
    lock = (b"# This file is automatically @generated by Cargo.\n"
            b"# It is not intended for manual editing.\nversion = 4\n\n"
            + b"\n\n".join(blocks[name] for name in sorted(blocks)) + b"\n")
    parsed = tomllib.loads(lock.decode())
    wanted = [{key: package[key] for key in ("name", "version", "source", "checksum", "dependencies")
               if key in package} for package in graph]
    wanted.append({"name": PACKAGE_NAME, "version": "0.0.0", "dependencies": ["garde"]})
    if parsed != {"version": 4, "package": sorted(wanted, key=lambda entry: entry["name"])}:
        raise ValueError("Dedicated fifteen-package lock differs from exact original graph")
    return graph, grants, lock


def _output_scope(inputs: Path, source_root: Path, vendor_root: Path):
    if (not inputs.is_absolute() or inputs.exists() or inputs.is_symlink()
            or inputs.parent != inputs.parent.resolve(strict=True)
            or any(path.is_symlink() for path in inputs.parents)
            or any(inputs.is_relative_to(root) or root.is_relative_to(inputs)
                   for root in (source_root, ROOT, vendor_root))):
        raise ValueError("Fresh output must be unaliased and separate from source/cache")


def prepare_source_state_constructor(source_root: Path, inputs: Path, case: str,
                                     harness_path: Path, *, vendor_root: Path = DEFAULT_VENDOR_ROOT) -> dict:
    """Prepare fresh private source files; read/hash only the fourteen vendor packages."""
    if case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown source-state constructor control")
    source_root, inputs, harness_path = Path(source_root), Path(inputs), Path(harness_path)
    captured = _verify_python()
    originals = _read_context(source_root)
    selected = _selection(originals)
    graph, grants, lock = _graph_and_grants(originals)
    vendor_root = VENDOR._canonical(vendor_root)
    _output_scope(inputs, source_root, vendor_root)
    before_vendor = VENDOR._selected(vendor_root, {"packages": grants})
    harness = VENDOR.CUSTODY.read_file(harness_path, maximum=2 * 1024**2)
    actual_harness = VENDOR.CUSTODY.read_file(ROOT / HARNESS, maximum=2 * 1024**2)
    if harness != actual_harness:
        raise ValueError("Harness differs from current reviewed source")
    own_hashes = {name: digest(VENDOR.CUSTODY.read_file(ROOT / name, maximum=2 * 1024**2))
                  for name in (SELF, HARNESS, TEST)}
    constructor = selected["constructor"]
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[case])
        old, new = mutation["before"].encode(), mutation["after"].encode()
        if constructor.count(old) != 1:
            raise ValueError("Actual constructor mutation point is not unique")
        constructor = constructor.replace(old, new, 1)
    definitions = (
        b"// SPDX-License-Identifier: GPL-2.0-only\n"
        b"// Exact original error declarations; formatting implementations omitted.\n"
        b"mod regression {\n" + selected["regression_errors"] + b"}\n"
        b"pub use regression::FitError;\nmod selection {\n" + selected["selection_errors"] + b"}\n"
        b"pub use selection::SelectionError;\nmod principal {\nuse crate::{FitError, SelectionError};\n"
        + selected["principal_errors"] + b"}\npub use principal::FixturePrincipalError;\n"
        b"mod combination {\nuse crate::FitError;\n" + selected["combination_errors"] + b"}\n")
    owner = selected["lifecycle_errors"] + selected["constants"] + constructor
    library = (b'#![allow(dead_code, reason = "Complete original declarations retain separately owned APIs.")]\n'
               b"include!(\"errors.rs\");\n"
               b"mod source_constructor {\nuse std::{fmt, num::NonZeroU64};\nuse garde::Validate;\n"
               b"use crate::{FitError, FixturePrincipalError};\n"
               b"use crate::combination::FixtureCombinationError;\n"
               b"include!(\"owner.rs\");\n#[cfg(kani)]\nmod proof {\nuse super::*;\n"
               b"include!(\"harness.rs\");\n}\n}\n")
    workspace = tomllib.loads(originals["Cargo.toml"].decode())["workspace"]["package"]
    manifest = (f'[package]\nname = "{PACKAGE_NAME}"\nversion = "0.0.0"\npublish = false\n'
                + "".join(f'{key} = {json.dumps(workspace[key])}\n'
                          for key in ("edition", "rust-version", "license", "repository"))
                + '\n[workspace]\n\n[lib]\npath = "src/lib.rs"\ndoctest = false\n'
                + '\n[dependencies]\ngarde = { version = "=0.23.0", default-features = false, features = ["derive"] }\n').encode()
    config = ('[net]\noffline = true\n\n[source.crates-io]\nreplace-with = "formal-vendor"\n'
              '\n[source.formal-vendor]\ndirectory = ' + json.dumps(str(vendor_root)) + '\n').encode()
    files = {"Cargo.toml": manifest, "Cargo.lock": lock, ".cargo/config.toml": config,
             "src/lib.rs": library, "src/errors.rs": definitions, "src/owner.rs": owner,
             "src/harness.rs": harness}
    details = {
        "package_name": PACKAGE_NAME, "case": case, "entrypoint": "Cargo.toml",
        "original_sources_sha256": dict(SOURCE_SHA256), "current_preparation_sources_sha256": own_hashes,
        "observed_python_sources": captured, "declared_python_inputs": list(DECLARED_PYTHON),
        "source_correspondence": [{"owner": name, "source": span["source"],
            "byte_offsets_half_open": list(span["offsets"]), "sha256": span["sha256"]}
            for name, span in SPANS.items()],
        "selected_original_bytes": 23483, "actual_error_enums": 23,
        "actual_error_enum_names": sorted(name.decode() for name in re.findall(
            rb"\bpub enum (\w+)", definitions + selected["lifecycle_errors"])),
        "mutation": mutation, "generated_files_sha256": {name: digest(data) for name, data in files.items()},
        "proof_spec_sha256": digest(harness), "proof_sha256": digest(library),
        "validation_path": "FixtureCombinationSourceState::new -> RawSourceFields::validate -> derived counters callback -> source_fields_rule -> source_fields_valid",
        "classification_path": "new -> independent source_fields_valid classification -> Invariant or validation.map_err(ValidationInvariant) -> exact fields",
        "retained_getters": ["source_id", "reachability_updates", "penalty", "last_reachability_ordinal"],
        "domain": {"label": "every actual NonZeroU64", "raw_counters": ["u8", "u8", "u64"],
                   "counter_assumptions": [], "label_zero_outside_domain": True},
        "independent_relation": "p<=32 && ((n<8 && u128(m)==u128(n)) || (n>=8 && m==8))",
        "selected_registry_graph": graph, "selected_package_grants": grants,
        "vendor_observation_before": before_vendor,
        "recommended_schedule": schedule(), "source_payload_authenticated_at_observation_time": True,
        "vendor_source_pointer": str(vendor_root), "vendor_namespace_mount_entitlement": False,
        "source_is_sealed": False, "execution_ready": False, "execution_admitted": False, "admitted": False,
        "proof_qualified": False, "target_features_observed": False, "native_commands_executed": 0,
        "compilation_observed": False, "controls_observed": False,
        "time_domain_dependency": "time-core declares time-domain; selected exact source closure has no time-domain calls",
        "configuration_limit": "Dedicated source correspondence crate omits shipping workspace lints/config/release profiles; source pointer is a broader unsealed cache",
        "assumptions": ["trusted Python interpreter and non-compromised host for source observation"],
        "allocator_assumptions": "None qualified; normal Garde Report/Vec/Path/Arc/CompactString allocation and drop are retained, allocator exhaustion is outside this preparation claim",
        "remaining_obligations": ["independent specification and closure review", "target feature observation",
            "real derive/build/proc-macro execution", "allocator/drop and unwinding qualification",
            "ordinary unmodified Kani feasibility", "full-domain completed healthy/defect/restored proofs",
            "Era padded-union defect and private patch holds remain; no private repair adoption",
            "with_penalty/event/caller/state transitions and Windows/native/product qualification"],
    }
    # Bookend the full authenticated selected records; do not copy or seal the cache.
    after_vendor = VENDOR._selected(vendor_root, {"packages": grants})
    if before_vendor != after_vendor:
        raise ValueError("Selected authenticated vendor source changed during generation")
    if _read_context(source_root) != originals or _verify_python() != captured:
        raise ValueError("Bound input closure changed during generation")
    if own_hashes != {name: digest(VENDOR.CUSTODY.read_file(ROOT / name, maximum=2 * 1024**2))
                     for name in own_hashes}:
        raise ValueError("Current preparation source changed during generation")
    details["vendor_observation_after"] = after_vendor
    files["correspondence.json"] = (json.dumps(details, indent=2, sort_keys=True) + "\n").encode()
    inputs.mkdir(mode=0o700)
    for name, data in files.items():
        destination = inputs / name
        destination.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
        descriptor = os.open(destination, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_NOFOLLOW, 0o600)
        with os.fdopen(descriptor, "wb") as stream:
            stream.write(data)
    return {**details, "all_generated_files_sha256": {name: digest(data) for name, data in files.items()},
            "correspondence_sha256": digest(files["correspondence.json"])}
