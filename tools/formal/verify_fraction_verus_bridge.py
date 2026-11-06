#!/usr/bin/env python3
"""Execute the one fixed, no-cheating, five-command pure predicate bridge card.

Frozen Python is evidence only: imports use reviewed current repository sources.
No product callee is imported, no old campaign is resumed, and nothing is installed.
"""
from __future__ import annotations

import sys

sys.dont_write_bytecode = True

import hashlib
from collections import namedtuple
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import stat
import time


ROOT = Path(__file__).resolve().parents[2]
_BINDING_FILES = (
    "tools/formal/python_source_binding.py",
    "tools/formal/test_python_source_binding.py",
    "tools/formal/test_verify_fraction_source_binding.py",
)
if "__source_binding__" in globals():
    SOURCE_BINDING = __source_binding__
    SOURCE_BINDING.bind_frame(sys._getframe())
else:
    _binding_spec = importlib.util.spec_from_file_location(
        "fraction_bridge_source_binding", ROOT / _BINDING_FILES[0])
    _binding_module = importlib.util.module_from_spec(_binding_spec)
    _binding_spec.loader.exec_module(_binding_module)
    SOURCE_BINDING = _binding_module.SourceBinding(ROOT, sys._getframe())

WORK = Path.home() / ".local/share/nevrotick/formal-core-fraction-verus-bridge-20261106-v2"
STARTED_UNIX_NS = 1793926103866069869
STARTED_MONOTONIC_NS = 22771699496998
DEADLINE_MONOTONIC_NS = 26371699496998
BOOT_ID = "d3fc5615-e867-45cd-b74f-4a7413afc5ab"
TOTAL_SECONDS = 3600
COMMAND_SECONDS = 180
MAXIMUM_BYTES = 128 * 1024**2
RESERVE_BYTES = 3 * 1024**3
CONTROLLER = "tools/formal/verify_fraction_verus_bridge.py"
TESTS = "tools/formal/test_verify_fraction_verus_bridge.py"
EXPECTED_IDENTITY = {
    "profile": "release", "version": "0.2026.11.02.8bf68ee",
    "platform": {"os": "linux", "arch": "x86_64"},
    "toolchain": "1.99.1-x86_64-unknown-linux-gnu (overridden by environment variable RUSTUP_TOOLCHAIN)",
    "commit": "8bf68ee741df833ea5d0fe6a2255126fdea87e47",
}


def require(condition, message):
    """Fail closed when a declared finite-card obligation is not established."""
    if not condition:
        raise RuntimeError(message)


def module(name, relative):
    """Execute captured source bytes and bind every transitive project import."""
    return SOURCE_BINDING.load_module(name, ROOT / relative)


PILOT = module("fraction_bridge_current_verus_primitives", "tools/formal/verus_pilot.py")
PROJECTION = module("fraction_bridge_current_projection", "tools/formal/fraction_verus_bridge_projection.py")
CHECK = PILOT.CHECK
SOURCE_NAMES = tuple(sorted(set(PILOT.SOURCES) | set(PROJECTION.input_names())
                            | set(_BINDING_FILES) | {CONTROLLER, TESTS}))
FUNCTIONS = tuple("proof::" + declaration.split(b"fn ", 1)[1].split(b"(", 1)[0].decode()
                  for declaration, _ in PROJECTION.DECLARATIONS) + (
    "proof::IntervalError::clone", "proof::FixtureEnclosureError::clone", "proof::Fraction::clone")
JOBS = (
    {"id": "healthy", "target": None},
    {"id": "constructor_wrong_zero", "target": "proof::composition_constructor_post",
     "old": b"&& value.shift == if numerator == 0 { 0 } else { shift }",
     "new": b"&& value.shift == shift"},
    {"id": "comparison_reversed", "target": "proof::composition_comparison_post",
     "old": b"Ok(order) => *order == bridge_rational_order(left, right),",
     "new": b"Ok(order) => *order == bridge_rational_order(right, left),"},
    {"id": "complement_wrong_error", "target": "proof::composition_complement_post",
     "old": b"Err(FixtureEnclosureError::Invariant) => !required_success,",
     "new": b"Err(FixtureEnclosureError::Invariant) => true,"},
    {"id": "restored", "target": None},
)

# Only these immutable singletons select the additive dependency cards. Callers
# cannot supply paths, clocks, modules, callbacks or verification profiles.
_Recipe = namedtuple("_Recipe", "work started_unix_ns started_monotonic_ns deadline_monotonic_ns boot_id modes jobs")
_BIT_LENGTH_CONTROLLER = "tools/formal/verify_fraction_bit_length.py"
_BIT_LENGTH_TESTS = "tools/formal/test_verify_fraction_bit_length.py"
_BIT_LENGTH_MODES = (
    ("proof::lemma_bridge_bit_length_power", "proof"),
    ("proof::composition_constructor_admitted", "exec"),
    ("proof::composition_checked", "exec"),
    ("proof::IntervalError::clone", "exec"),
    ("proof::FixtureEnclosureError::clone", "exec"),
    ("proof::Fraction::clone", "exec"),
)
_BIT_LENGTH_RECIPE = _Recipe(
    Path.home() / ".local/share/nevrotick/formal-core-fraction-bit-length-20261106-v1",
    1793928357646410371, 25025479837880, 28625479837880,
    "d3fc5615-e867-45cd-b74f-4a7413afc5ab", _BIT_LENGTH_MODES,
    (("healthy", None), ("lemma_wrong_theorem", "proof::lemma_bridge_bit_length_power"),
     ("constructor_wrong_bound", "proof::composition_constructor_admitted"),
     ("checked_wrong_zero", "proof::composition_checked"), ("restored", None)))
_CONSTRUCTOR_POST_CONTROLLER = "tools/formal/verify_fraction_constructor_post.py"
_CONSTRUCTOR_POST_TESTS = "tools/formal/test_verify_fraction_constructor_post.py"
_CONSTRUCTOR_POST_MODES = (
    ("proof::lemma_bridge_bit_length_power", "proof"),
    ("proof::composition_constructor_admitted", "exec"),
    ("proof::composition_checked", "exec"),
    ("proof::composition_constructor_post", "exec"),
    ("proof::IntervalError::clone", "exec"),
    ("proof::FixtureEnclosureError::clone", "exec"),
    ("proof::Fraction::clone", "exec"),
)
_CONSTRUCTOR_POST_RECIPE = _Recipe(
    Path.home() / ".local/share/nevrotick/formal-core-fraction-constructor-post-20261106-v1",
    1793930612007295967, 27279840722916, 30879840722916,
    "d3fc5615-e867-45cd-b74f-4a7413afc5ab", _CONSTRUCTOR_POST_MODES,
    (("healthy", None), ("constructor_wrong_zero", "proof::composition_constructor_post"),
     ("constructor_wrong_invariant", "proof::composition_constructor_post"),
     ("constructor_wrong_numerator", "proof::composition_constructor_post"), ("restored", None)))
_LEMMA_THEOREM = (b"    ensures\n"
    b"        (64 - u64_leading_zeros(n) <= s as int)\n"
    b"            <==> ((n as int) < pow(2, s as nat)),\n")
_WRONG_LEMMA_THEOREM = (b"    ensures\n"
    b"        ((64 - u64_leading_zeros(n) <= s as int)\n"
    b"            <==> ((n as int) < pow(2, s as nat)))\n"
    b"            && n != 0,\n")


def _closed(recipe):
    """Accept the legacy default or one reviewed immutable dependency card."""
    require(recipe is None or recipe is _BIT_LENGTH_RECIPE or recipe is _CONSTRUCTOR_POST_RECIPE,
            "Unrecognized finite recipe")
    return recipe


def _work(recipe):
    """Retain the legacy dynamic default without configuring module globals."""
    return WORK if _closed(recipe) is None else recipe.work


def _recipe_options(recipe):
    """Preserve legacy no-keyword calls; forward only the closed new singleton."""
    return {} if _closed(recipe) is None else {"_recipe": recipe}


def _source_names(recipe):
    """Bind exactly the current closure for the selected closed source path."""
    if _closed(recipe) is None:
        return SOURCE_NAMES
    if recipe is _CONSTRUCTOR_POST_RECIPE:
        return tuple(sorted(set(PILOT.SOURCES) | set(PROJECTION.dependency_input_names()) | set(_BINDING_FILES)
                            | {CONTROLLER, TESTS, _BIT_LENGTH_CONTROLLER, _BIT_LENGTH_TESTS,
                               _CONSTRUCTOR_POST_CONTROLLER, _CONSTRUCTOR_POST_TESTS}))
    return tuple(sorted(set(PILOT.SOURCES) | set(PROJECTION.dependency_input_names()) | set(_BINDING_FILES)
                        | {CONTROLLER, TESTS, _BIT_LENGTH_CONTROLLER, _BIT_LENGTH_TESTS}))


def _jobs(recipe):
    """Return fresh descriptions of the fixed five-job dependency schedule."""
    if _closed(recipe) is None:
        return JOBS
    return tuple({"id": name, "target": target} for name, target in recipe.jobs)


def digest(data):
    """Identify exact bytes without normalization."""
    return hashlib.sha256(data).hexdigest()


def write(path, value, *, _recipe=None):
    """Create one fresh, private regular receipt without aliases or overwrite."""
    work = _work(_recipe)
    plain(work)
    require(identity(work)["kind"] == "directory" and stat.S_IMODE(work.stat().st_mode) == 0o700,
            "Receipt work root is not an owned private directory")
    require(path.is_absolute() and path.is_relative_to(work) and path != work,
            "Receipt must be strictly inside owned work")
    plain(path.parent)
    require(identity(path.parent)["kind"] == "directory", "Receipt parent is not an owned directory")
    data = (json.dumps(value, indent=2, sort_keys=True, default=str, allow_nan=False) + "\n").encode()
    descriptor = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL | os.O_CLOEXEC | os.O_NOFOLLOW, 0o600)
    with os.fdopen(descriptor, "wb") as stream:
        stream.write(data)


def remaining(*, _recipe=None):
    """Charge all preparation and execution to the one original boot/clock."""
    recipe = _closed(_recipe)
    boot = BOOT_ID if recipe is None else recipe.boot_id
    started_unix = STARTED_UNIX_NS if recipe is None else recipe.started_unix_ns
    started_monotonic = STARTED_MONOTONIC_NS if recipe is None else recipe.started_monotonic_ns
    deadline_monotonic = DEADLINE_MONOTONIC_NS if recipe is None else recipe.deadline_monotonic_ns
    require(Path("/proc/sys/kernel/random/boot_id").read_text().strip() == boot, "Campaign boot changed")
    monotonic, unix = time.monotonic_ns(), time.time_ns()
    require(started_monotonic <= monotonic < deadline_monotonic
            and started_unix <= unix < started_unix + TOTAL_SECONDS * 10**9,
            "Original preparation/integration clock exhausted or moved backwards")
    return min(deadline_monotonic - monotonic,
               started_unix + TOTAL_SECONDS * 10**9 - unix) / 10**9


def plain(path):
    """Reject redirects in every path component, including a root replacement."""
    require(path.is_absolute() and path == path.resolve(strict=True)
            and not any(item.is_symlink() for item in (path, *path.parents)), "Input path redirects")
    return path


def identity(path, *, frozen=False):
    """Observe type, owner, mode and inode before bounded content inspection."""
    info = path.lstat()
    directory = stat.S_ISDIR(info.st_mode)
    require(info.st_uid == os.geteuid() and (directory or stat.S_ISREG(info.st_mode)),
            "Unowned, redirected or special evidence entry")
    require(directory or info.st_nlink == 1, "Evidence file has a hardlink alias")
    mode = stat.S_IMODE(info.st_mode)
    if frozen:
        require(mode == (0o500 if directory else 0o400), "Frozen evidence permission drift")
    row = {"kind": "directory" if directory else "file", "uid": info.st_uid,
           "mode": mode, "device": info.st_dev, "inode": info.st_ino, "links": info.st_nlink}
    if not directory:
        data = CHECK.read_once(path, maximum=2 * 1024**2)
        after = path.lstat()
        require((after.st_dev, after.st_ino, after.st_mode, after.st_uid, after.st_gid,
                 after.st_nlink, after.st_size, after.st_mtime_ns, after.st_ctime_ns)
                == (info.st_dev, info.st_ino, info.st_mode, info.st_uid, info.st_gid,
                    info.st_nlink, info.st_size, info.st_mtime_ns, info.st_ctime_ns),
                "Evidence identity changed during read")
        row.update(bytes=len(data), sha256=digest(data), mtime_ns=info.st_mtime_ns, ctime_ns=info.st_ctime_ns)
    return row


def tree(path, *, frozen=True):
    """Inventory every immutable entry; caches and unexpected members are drift."""
    plain(path)
    result = {}

    def inspect(item):
        require(item.name != "__pycache__" and item.suffix not in (".pyc", ".pyo"), "Bytecode cache in evidence")
        row = identity(item, frozen=frozen)
        result[item.relative_to(path).as_posix()] = row
        if row["kind"] == "directory":
            for child in sorted(item.iterdir()):
                inspect(child)

    inspect(path)
    return result


def original_sources(*, _recipe=None):
    """Reject drift from executed Python before admitting current source bytes."""
    names = _source_names(_recipe)
    SOURCE_BINDING.verify(names)
    records = {name: identity(plain(ROOT / name)) for name in names}
    SOURCE_BINDING.verify(names, records)
    return records


def seal(path, *, _recipe=None):
    """Seal owned copied inputs or completed outputs; never a live source tree."""
    work = _work(_recipe)
    plain(work)
    require(identity(work)["kind"] == "directory" and stat.S_IMODE(work.stat().st_mode) == 0o700,
            "Seal work root is not an owned private directory")
    require(path.is_relative_to(work) and path != work, "Refusing to seal outside owned work")
    plain(path)
    require(identity(path)["kind"] == "directory", "Seal root is not a regular directory")
    for directory, subdirs, files in os.walk(path, topdown=False, followlinks=False):
        for name in files:
            item = Path(directory) / name
            identity(item)
            os.chmod(item, 0o400)
        for name in subdirs:
            item = Path(directory) / name
            require(identity(item)["kind"] == "directory", "Copied directory redirects")
            os.chmod(item, 0o500)
        os.chmod(directory, 0o500)


def capacity(work, *, _recipe=None):
    """Check retained allocated bytes and free reserve, independently of acquisition."""
    remaining(**_recipe_options(_recipe))
    total = 0
    for directory, subdirs, files in os.walk(work, followlinks=False):
        for item in (Path(directory), *(Path(directory) / name for name in files + subdirs)):
            info = item.lstat()
            require(info.st_uid == os.geteuid() and (stat.S_ISREG(info.st_mode) or stat.S_ISDIR(info.st_mode)),
                    "Work entry is redirected, unowned or special")
            require(not stat.S_ISREG(info.st_mode) or info.st_nlink == 1, "Work file has an alias")
            # Directory entries occur once as a parent and once as a child.
            if item == Path(directory) or not stat.S_ISDIR(info.st_mode):
                total += info.st_blocks * 512
    require(total <= MAXIMUM_BYTES, "128-MiB newly retained work budget exhausted")
    require(shutil.disk_usage(work).free >= RESERVE_BYTES, "Three-GiB free-space reserve exhausted")
    return total


def freeze_sources(work, *, _recipe=None):
    """Copy the complete finite current-source closure, then seal copies only."""
    before = original_sources(**_recipe_options(_recipe))
    source = work / "source"
    source.mkdir(mode=0o700)
    for name in _source_names(_recipe):
        target = source / name
        target.parent.mkdir(mode=0o700, parents=True, exist_ok=True)
        data = CHECK.read_once(ROOT / name, maximum=2 * 1024**2)
        require(digest(data) == before[name]["sha256"], "Source changed while freezing")
        with target.open("xb") as stream:
            stream.write(data)
    require(original_sources(**_recipe_options(_recipe)) == before, "Original source closure changed during freeze")
    seal(source, **_recipe_options(_recipe))
    return before, tree(source)


def validate_metadata(details, source, generated, *, _recipe=None):
    """Require the exact six relations and complete original/generated byte bindings."""
    if _closed(_recipe) is _CONSTRUCTOR_POST_RECIPE:
        return _validate_constructor_post_metadata(details, source, generated)
    if _recipe is _BIT_LENGTH_RECIPE:
        return _validate_bit_length_metadata(details, source, generated)
    require(type(details.get("schema_version")) is int and details["schema_version"] == 1
            and details.get("status") == "unproved_source_candidate"
            and type(details.get("proof_commands")) is int and details["proof_commands"] == 0,
            "Projection status or schema differs")
    expected_sources = {name: digest(CHECK.read_once(source / name)) for name in PROJECTION.input_names()}
    require(details.get("source_files_sha256") == expected_sources
            and details.get("predicate_relations") == [relation.decode() for _, relation in PROJECTION.DECLARATIONS]
            and details.get("ghost_sha256") == PROJECTION.GHOST_SHA256
            and details.get("prefix_sha256") == PROJECTION.PREFIX_SHA256,
            "Projection input or six-relation metadata differs")
    originals, spans = PROJECTION._source_support(CHECK.read_once(source / "tools/formal/fraction_arithmetic_projection.py"))
    expected_spans = [{"item": name, "source": "crates/time-core/src/" + span["source"],
                       "byte_offsets_half_open": list(span["offsets"]), "sha256": span["sha256"],
                       "bridge_type": name in PROJECTION.TYPE_NAMES} for name, span in spans.items()]
    prefix = CHECK.read_once(source / "tools/formal/fraction_composition_spec.rs").split(PROJECTION.PREFIX_MARKER, 1)[0]
    require(details.get("original_sources_sha256") == originals and details.get("correspondence") == expected_spans
            and details.get("annotated_prefix_sha256") == digest(PROJECTION.annotated(prefix)),
            "Complete original product/span/prefix metadata differs")
    require(details.get("generated_sha256") == {
        name: digest(CHECK.read_once(generated / name)) for name in ("bridge.rs", "actual_source.rs")},
        "Generated bridge/product closure hash differs")
    require(CHECK.decode(CHECK.read_once(generated / "correspondence.json")) == details,
            "Persisted projection metadata differs")


def _validate_bit_length_metadata(details, source, generated):
    """Reconstruct the dependency packet from the eight sealed source inputs."""
    names = PROJECTION.dependency_input_names()
    sources = {name: CHECK.read_once(source / name) for name in names}
    source_hashes = {name: digest(data) for name, data in sources.items()}
    originals, spans = PROJECTION._source_support(sources["tools/formal/fraction_arithmetic_projection.py"])
    selected = {}
    for name, span in spans.items():
        original = sources["crates/time-core/src/" + span["source"]]
        first, last = span["offsets"]
        selected[name] = original[first:last]
        require(digest(original) == originals[span["source"]]
                and original.count(span["start"]) == original.count(span["following"]) == 1
                and original.index(span["start"]) == first and original.index(span["following"]) >= last
                and digest(selected[name]) == span["sha256"], "Dependency original source/span differs")
    expected_spans = [{"item": name, "source": "crates/time-core/src/" + span["source"],
                       "byte_offsets_half_open": list(span["offsets"]), "sha256": span["sha256"],
                       "bridge_type": name in PROJECTION.TYPE_NAMES} for name, span in spans.items()]
    composition = sources["tools/formal/fraction_composition_spec.rs"]
    require(digest(composition) == PROJECTION.COMPOSITION_SHA256
            and composition.count(PROJECTION.PREFIX_MARKER) == 1, "Dependency composition source differs")
    prefix = composition.split(PROJECTION.PREFIX_MARKER, 1)[0]
    require(len(prefix) == 3731 and digest(prefix) == PROJECTION.PREFIX_SHA256
            and prefix.count(PROJECTION.BIT_LENGTH_MARKER) == 1, "Dependency complete predicate prefix differs")
    fragment = prefix.split(PROJECTION.BIT_LENGTH_MARKER, 1)[0]
    predicates = PROJECTION.annotated_bit_length(fragment)
    require(len(fragment) == 661 and PROJECTION.erase_bit_length_annotations(predicates) == fragment,
            "Dependency fragment erasure differs")
    ghost = sources["tools/formal/fraction_verus_bridge_spec.rs"]
    lemma = sources[PROJECTION.BIT_LENGTH_LEMMAS]
    require(digest(ghost) == PROJECTION.GHOST_SHA256 and digest(lemma) == PROJECTION.BIT_LENGTH_LEMMAS_SHA256,
            "Dependency ghost/lemma source differs")
    types = b"\n".join(selected[name] for name in PROJECTION.TYPE_NAMES)
    proof = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
             b"// UNPROVED bit-length dependency source; full bridge acceptance remains open.\n"
             b"use std::cmp::Ordering;\nuse vstd::prelude::*;\n\nverus! {\n\n"
             + types + b"\n\n" + ghost + b"\n" + lemma + b"\n" + predicates + b"\n}\n")
    actual = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
              b"// Correspondence input only; no caller-import entrypoint or permission.\n"
              b"use std::cmp::Ordering;\n\n" + types + b"\nimpl Fraction {\n"
              + b"\n".join(selected[name] for name in ("zero", "new", "compare", "complement"))
              + b"\n}\n\n" + selected["fraction_sum"] + b"\n" + selected["addition"])
    hooks = [{"function": "proof::" + name, "source_byte_offset": fragment.index(anchor),
              "insertion": "before unchanged leading_zeros subtraction",
              "anchor_sha256": digest(anchor), "proof_block_sha256": digest(block)}
             for name, anchor, block in PROJECTION.BIT_LENGTH_HOOKS]
    expected = {
        "schema_version": 1, "status": "unproved_source_candidate", "proof_commands": 0,
        "scope": "bit-length lemma and two unrestricted predicate equalities; source preparation only",
        "source_files_sha256": source_hashes, "original_sources_sha256": originals,
        "correspondence": expected_spans, "prefix_sha256": digest(prefix), "prefix_bytes": 3731,
        "fragment_source_byte_offsets_half_open": [0, 661], "fragment_sha256": digest(fragment),
        "annotated_fragment_sha256": digest(predicates), "ghost_sha256": digest(ghost),
        "lemma_sha256": digest(lemma),
        "predicate_relations": [relation.decode() for _, relation in PROJECTION.DECLARATIONS[:2]],
        "full_bridge_predicate_relations": [relation.decode() for _, relation in PROJECTION.DECLARATIONS],
        "expected_function_modes": dict(_BIT_LENGTH_RECIPE.modes), "proof_hooks": hooks,
        "raw_domains": {"lemma": "every n:u64 and s:u32; no precondition",
                        "constructor_admitted": "every numerator:u64 and shift:u32; no precondition",
                        "checked": "every raw Fraction { numerator:u64, shift:u32 }; no precondition"},
        "upstream_dependencies": {
            "commit": "8bf68ee741df833ea5d0fe6a2255126fdea87e47",
            "paths": ["source/vstd/std_specs/bits.rs", "source/vstd/arithmetic/power.rs",
                      "source/vstd/arithmetic/power2.rs", "source/vstd/arithmetic/div_mod.rs"],
            "status": "source API references only; no new admission or proof"},
        "remaining": ["exact Verus compatibility and all lemma assertions/termination",
                      "u32 leading_zeros result and subtraction-to-int inference",
                      "two completed full-raw predicate equalities and wrong-contract/restoration controls",
                      "all six full-domain bridge equalities and their controls",
                      "native Ordering equality correspondence, rational alignment and complement",
                      "separate caller-import profile admission and original addition union"],
    }
    require(set(details) == {*expected, "generated_sha256"}
            and type(details.get("schema_version")) is int and type(details.get("proof_commands")) is int
            and type(details.get("prefix_bytes")) is int
            and type(details.get("fragment_source_byte_offsets_half_open")) is list
            and all(type(value) is int for value in details["fragment_source_byte_offsets_half_open"])
            and all(details.get(key) == value for key, value in expected.items()),
            "Dependency projection metadata differs")
    for span in details["correspondence"]:
        require(type(span.get("bridge_type")) is bool
                and type(span.get("byte_offsets_half_open")) is list
                and all(type(value) is int for value in span["byte_offsets_half_open"]),
                "Dependency correspondence offset/type differs")
    for hook in details["proof_hooks"]:
        require(type(hook.get("source_byte_offset")) is int, "Dependency hook offset is not an integer")
    artifacts = {"proof.rs": proof, "actual_source.rs": actual, "predicate_source.rs": prefix}
    require(all(CHECK.read_once(generated / name) == data for name, data in artifacts.items())
            and details.get("generated_sha256") == {name: digest(data) for name, data in artifacts.items()},
            "Dependency generated proof/product/prefix differs")
    require(CHECK.decode(CHECK.read_once(generated / "correspondence.json")) == details,
            "Persisted dependency metadata differs")


def _validate_constructor_post_metadata(details, source, generated):
    """Independently reconstruct the full-raw constructor-Post source packet."""
    names = PROJECTION.dependency_input_names()
    sources = {name: CHECK.read_once(source / name) for name in names}
    source_hashes = {name: digest(data) for name, data in sources.items()}
    originals, spans = PROJECTION._source_support(sources["tools/formal/fraction_arithmetic_projection.py"])
    selected = {}
    for name, span in spans.items():
        original = sources["crates/time-core/src/" + span["source"]]
        first, last = span["offsets"]
        selected[name] = original[first:last]
        require(digest(original) == originals[span["source"]]
                and original.count(span["start"]) == original.count(span["following"]) == 1
                and original.index(span["start"]) == first and original.index(span["following"]) >= last
                and digest(selected[name]) == span["sha256"], "Dependency original source/span differs")
    expected_spans = [{"item": name, "source": "crates/time-core/src/" + span["source"],
                       "byte_offsets_half_open": list(span["offsets"]), "sha256": span["sha256"],
                       "bridge_type": name in PROJECTION.TYPE_NAMES} for name, span in spans.items()]
    composition = sources["tools/formal/fraction_composition_spec.rs"]
    require(digest(composition) == PROJECTION.COMPOSITION_SHA256
            and composition.count(PROJECTION.PREFIX_MARKER) == 1, "Dependency composition source differs")
    prefix = composition.split(PROJECTION.PREFIX_MARKER, 1)[0]
    require(len(prefix) == 3731 and digest(prefix) == PROJECTION.PREFIX_SHA256
            and prefix.count(PROJECTION.CONSTRUCTOR_POST_MARKER) == 1, "Dependency complete predicate prefix differs")
    fragment = prefix.split(PROJECTION.CONSTRUCTOR_POST_MARKER, 1)[0]
    predicates = PROJECTION.annotated_constructor_post(fragment)
    require(len(fragment) == 1202 and digest(fragment) == PROJECTION.CONSTRUCTOR_POST_SLICE_SHA256
            and PROJECTION.erase_constructor_post_annotations(predicates) == fragment,
            "Dependency fragment erasure differs")
    ghost = sources["tools/formal/fraction_verus_bridge_spec.rs"]
    lemma = sources[PROJECTION.BIT_LENGTH_LEMMAS]
    require(digest(ghost) == PROJECTION.GHOST_SHA256 and digest(lemma) == PROJECTION.BIT_LENGTH_LEMMAS_SHA256,
            "Dependency ghost/lemma source differs")
    types = b"\n".join(selected[name] for name in PROJECTION.TYPE_NAMES)
    proof = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
             b"// UNPROVED constructor-Post dependency source; full bridge acceptance remains open.\n"
             b"use std::cmp::Ordering;\nuse vstd::prelude::*;\n\nverus! {\n\n"
             + types + b"\n\n" + ghost + b"\n" + lemma + b"\n" + predicates + b"\n}\n")
    actual = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
              b"// Correspondence input only; no caller-import entrypoint or permission.\n"
              b"use std::cmp::Ordering;\n\n" + types + b"\nimpl Fraction {\n"
              + b"\n".join(selected[name] for name in ("zero", "new", "compare", "complement"))
              + b"\n}\n\n" + selected["fraction_sum"] + b"\n" + selected["addition"])
    hooks = [{"function": "proof::" + name, "source_byte_offset": fragment.index(anchor),
              "insertion": "before unchanged leading_zeros subtraction",
              "anchor_sha256": digest(anchor), "proof_block_sha256": digest(block)}
             for name, anchor, block in PROJECTION.BIT_LENGTH_HOOKS]
    expected = {
        "schema_version": 1, "status": "unproved_source_candidate", "proof_commands": 0,
        "scope": "bit-length lemma and three unrestricted predicate equalities; source preparation only",
        "source_files_sha256": source_hashes, "original_sources_sha256": originals,
        "correspondence": expected_spans, "prefix_sha256": digest(prefix), "prefix_bytes": 3731,
        "fragment_source_byte_offsets_half_open": [0, 1202], "fragment_sha256": digest(fragment),
        "annotated_fragment_sha256": digest(predicates), "ghost_sha256": digest(ghost),
        "lemma_sha256": digest(lemma),
        "predicate_relations": [relation.decode() for _, relation in PROJECTION.DECLARATIONS[:3]],
        "full_bridge_predicate_relations": [relation.decode() for _, relation in PROJECTION.DECLARATIONS],
        "expected_function_modes": dict(_CONSTRUCTOR_POST_RECIPE.modes), "proof_hooks": hooks,
        "raw_domains": {"lemma": "every n:u64 and s:u32; no precondition",
                        "constructor_admitted": "every numerator:u64 and shift:u32; no precondition",
                        "checked": "every raw Fraction { numerator:u64, shift:u32 }; no precondition",
                        "constructor_post": "every numerator:u64, shift:u32 and independently chosen &Result<Fraction, FixtureEnclosureError>; raw Ok fields and every error variant; no precondition"},
        "upstream_dependencies": {
            "commit": "8bf68ee741df833ea5d0fe6a2255126fdea87e47",
            "paths": ["source/vstd/std_specs/bits.rs", "source/vstd/arithmetic/power.rs",
                      "source/vstd/arithmetic/power2.rs", "source/vstd/arithmetic/div_mod.rs"],
            "status": "source API references only; no new admission or proof"},
        "remaining": ["exact Verus compatibility and all lemma assertions/termination",
                      "u32 leading_zeros result and subtraction-to-int inference",
                      "three completed full-raw predicate equalities and wrong-contract/restoration controls",
                      "all six full-domain bridge equalities and their controls",
                      "native Ordering equality correspondence, rational alignment and complement",
                      "separate caller-import profile admission and original addition union"],
    }
    require(set(details) == {*expected, "generated_sha256"}
            and type(details.get("schema_version")) is int and type(details.get("proof_commands")) is int
            and type(details.get("prefix_bytes")) is int
            and type(details.get("fragment_source_byte_offsets_half_open")) is list
            and all(type(value) is int for value in details["fragment_source_byte_offsets_half_open"])
            and all(details.get(key) == value for key, value in expected.items()),
            "Dependency projection metadata differs")
    for span in details["correspondence"]:
        require(type(span.get("bridge_type")) is bool
                and type(span.get("byte_offsets_half_open")) is list
                and all(type(value) is int for value in span["byte_offsets_half_open"]),
                "Dependency correspondence offset/type differs")
    for hook in details["proof_hooks"]:
        require(type(hook.get("source_byte_offset")) is int, "Dependency hook offset is not an integer")
    artifacts = {"proof.rs": proof, "actual_source.rs": actual, "predicate_source.rs": prefix}
    require(all(CHECK.read_once(generated / name) == data for name, data in artifacts.items())
            and details.get("generated_sha256") == {name: digest(data) for name, data in artifacts.items()},
            "Dependency generated proof/product/prefix differs")
    require(CHECK.decode(CHECK.read_once(generated / "correspondence.json")) == details,
            "Persisted dependency metadata differs")


def _bit_length_control(healthy, ghost, job):
    """Change exactly one closed ghost/theorem segment, with inverse erasure."""
    lemma = CHECK.read_once(ROOT / PROJECTION.BIT_LENGTH_LEMMAS)
    prefix = CHECK.read_once(ROOT / "tools/formal/fraction_composition_spec.rs").split(PROJECTION.PREFIX_MARKER, 1)[0]
    fragment = prefix.split(PROJECTION.BIT_LENGTH_MARKER, 1)[0]
    executable = PROJECTION.annotated_bit_length(fragment)
    require(digest(ghost) == PROJECTION.GHOST_SHA256 and healthy.count(ghost) == 1
            and digest(lemma) == PROJECTION.BIT_LENGTH_LEMMAS_SHA256 and healthy.count(lemma) == 1,
            "Dependency ghost/lemma segment identity differs")
    require(healthy.count(executable) == 1 and PROJECTION.erase_bit_length_annotations(executable) == fragment,
            "Dependency executable erasure differs")
    descriptions = {item["id"]: item for item in _jobs(_BIT_LENGTH_RECIPE)}
    require(job.get("id") in descriptions and job.get("target") == descriptions[job["id"]]["target"],
            "Dependency control identity differs")
    if job["target"] is None:
        return healthy
    if job["id"] == "lemma_wrong_theorem":
        segment, old, new = lemma, _LEMMA_THEOREM, _WRONG_LEMMA_THEOREM
    else:
        if job["id"] == "constructor_wrong_bound":
            first, following = b"spec fn bridge_constructor_admitted(", b"/// Canonical zero"
            old, new = b"shift <= 1041", b"shift < 1041"
        else:
            first, following = b"spec fn bridge_checked(", b"/// Exact successful fields"
            old, new = b"value.shift == 0", b"value.shift == 1"
        require(ghost.count(first) == ghost.count(following) == 1, "Dependency ghost scope is ambiguous")
        segment = ghost[ghost.index(first):ghost.index(following)]
    require(healthy.count(segment) == 1 and segment.count(old) == 1 and new not in segment,
            "Dependency control replacement is ambiguous")
    faulty = segment.replace(old, new, 1)
    require(faulty.replace(new, old, 1) == segment, "Dependency control reversal differs")
    candidate = healthy.replace(segment, faulty, 1)
    require(candidate.replace(faulty, segment, 1) == healthy and candidate.count(executable) == 1,
            "Dependency control changed executable or restored healthy bytes")
    return candidate


def _constructor_post_control(healthy, ghost, job):
    """Change one fixed constructor ghost leaf while retaining all executable bytes."""
    lemma = CHECK.read_once(ROOT / PROJECTION.BIT_LENGTH_LEMMAS)
    prefix = CHECK.read_once(ROOT / "tools/formal/fraction_composition_spec.rs").split(PROJECTION.PREFIX_MARKER, 1)[0]
    fragment = prefix.split(PROJECTION.CONSTRUCTOR_POST_MARKER, 1)[0]
    executable = PROJECTION.annotated_constructor_post(fragment)
    require(digest(ghost) == PROJECTION.GHOST_SHA256 and healthy.count(ghost) == 1
            and digest(lemma) == PROJECTION.BIT_LENGTH_LEMMAS_SHA256 and healthy.count(lemma) == 1,
            "Constructor-Post ghost/lemma segment identity differs")
    require(healthy.count(executable) == 1
            and PROJECTION.erase_constructor_post_annotations(executable) == fragment,
            "Constructor-Post executable erasure differs")
    descriptions = {item["id"]: item for item in _jobs(_CONSTRUCTOR_POST_RECIPE)}
    require(job.get("id") in descriptions and job.get("target") == descriptions[job["id"]]["target"],
            "Constructor-Post control identity differs")
    if job["target"] is None:
        return healthy
    first, following = b"spec fn bridge_constructor_post(", b"/// Total raw helper:"
    require(ghost.count(first) == ghost.count(following) == 1,
            "Constructor-Post ghost scope is ambiguous")
    segment = ghost[ghost.index(first):ghost.index(following)]
    replacements = {
        "constructor_wrong_zero": (b"&& value.shift == if numerator == 0 { 0 } else { shift }",
                                   b"&& value.shift == shift"),
        "constructor_wrong_invariant": (b"Err(FixtureEnclosureError::Invariant) => !admitted,",
                                        b"Err(FixtureEnclosureError::Invariant) => true,"),
        "constructor_wrong_numerator": (b"&& value.numerator == numerator", b"&& value.numerator <= numerator"),
    }
    old, new = replacements[job["id"]]
    require(healthy.count(segment) == 1 and segment.count(old) == 1 and new not in segment,
            "Constructor-Post control replacement is ambiguous")
    faulty = segment.replace(old, new, 1)
    require(faulty.replace(new, old, 1) == segment, "Constructor-Post control reversal differs")
    candidate = healthy.replace(segment, faulty, 1)
    require(candidate.replace(faulty, segment, 1) == healthy and candidate.count(executable) == 1
            and candidate.count(lemma) == 1, "Constructor-Post control changed executable or restored healthy bytes")
    return candidate


def control_bytes(healthy, ghost, job, *, _recipe=None):
    """Change one exact ghost-only leaf and prove reversal and executable erasure."""
    if _closed(_recipe) is _CONSTRUCTOR_POST_RECIPE:
        return _constructor_post_control(healthy, ghost, job)
    if _recipe is _BIT_LENGTH_RECIPE:
        return _bit_length_control(healthy, ghost, job)
    require(digest(ghost) == PROJECTION.GHOST_SHA256 and healthy.count(ghost) == 1, "Ghost segment identity differs")
    prefix = CHECK.read_once(ROOT / "tools/formal/fraction_composition_spec.rs").split(PROJECTION.PREFIX_MARKER, 1)[0]
    executable = PROJECTION.annotated(prefix)
    require(healthy.count(executable) == 1 and PROJECTION.erase_annotations(executable) == prefix,
            "Healthy executable prefix erasure differs")
    if job["target"] is None:
        return healthy
    old, new = job["old"], job["new"]
    require(ghost.count(old) == 1 and new not in ghost, "Ghost control replacement is ambiguous")
    faulty = ghost.replace(old, new, 1)
    restored = faulty.replace(new, old, 1)
    require(restored == ghost and digest(restored) == PROJECTION.GHOST_SHA256, "Ghost control reversal differs")
    candidate = healthy.replace(ghost, faulty, 1)
    require(candidate.replace(faulty, restored, 1) == healthy and candidate.count(executable) == 1,
            "Control changed executable or restored healthy bytes")
    return candidate


def function_spans(data, *, _recipe=None):
    """Bind six exact returns clauses and their closed executable body ranges."""
    dependency = _closed(_recipe) is not None
    result = {}
    if _recipe is _CONSTRUCTOR_POST_RECIPE:
        declarations = PROJECTION.DECLARATIONS[:3]
    else:
        declarations = PROJECTION.DECLARATIONS[:2] if dependency else PROJECTION.DECLARATIONS
    for declaration, relation in declarations:
        annotation = PROJECTION._annotation(declaration, relation)
        require(data.count(annotation) == 1, "Generated function annotation differs")
        start = data.index(annotation)
        opening = start + len(annotation) - 1
        depth, position = 1, opening + 1
        while depth and position < len(data):
            depth += (data[position] == ord("{")) - (data[position] == ord("}"))
            position += 1
        require(depth == 0, "Generated function body is incomplete")
        name = "proof::" + declaration.split(b"fn ", 1)[1].split(b"(", 1)[0].decode()
        returns = start + annotation.index(b"returns ")
        body = data[opening + 1:position - 1]
        tail = b"    composition_checked(left)" if name.endswith("comparison_post") else b"    match result {"
        tail_start = opening + 1 + body.index(tail) if name.endswith("_post") else opening + 1
        result[name] = {"returns_line": data[:returns].count(b"\n") + 1,
                        "return_start": data[:tail_start].count(b"\n") + 1,
                        "return_end": data[:position].count(b"\n") + 1}
    if dependency:
        declaration = b"proof fn lemma_bridge_bit_length_power(n: u64, s: u32)"
        require(data.count(declaration) == 1, "Dependency lemma declaration differs")
        start = data.index(declaration)
        clause = data.index(b"    ensures\n", start) + len(b"    ensures\n")
        clause_end = data.index(b",\n    decreases n,\n", clause)
        opening = data.index(b"\n{\n", clause_end) + 1
        depth, position = 1, opening + 1
        while depth and position < len(data):
            depth += (data[position] == ord("{")) - (data[position] == ord("}"))
            position += 1
        require(depth == 0, "Dependency lemma body is incomplete")
        result["proof::lemma_bridge_bit_length_power"] = {
            "clause_start": data[:clause].count(b"\n") + 1,
            "clause_end": data[:clause_end].count(b"\n") + 1,
            "return_start": data[:opening].count(b"\n") + 1,
            "return_end": data[:position].count(b"\n") + 1}
    return result


def prepare_inputs(work, *, _recipe=None):
    """Generate all five closed candidates from frozen data without frozen imports."""
    dependency = _closed(_recipe) is not None
    inputs = work / "inputs"
    inputs.mkdir(mode=0o700)
    generated = inputs / "baseline"
    if _recipe is _CONSTRUCTOR_POST_RECIPE:
        details = PROJECTION.prepare_constructor_post(work / "source", generated)
    else:
        details = (PROJECTION.prepare_bit_length(work / "source", generated) if dependency
                   else PROJECTION.prepare(work / "source", generated))
    validate_metadata(details, work / "source", generated, **_recipe_options(_recipe))
    healthy = CHECK.read_once(generated / ("proof.rs" if dependency else "bridge.rs"))
    ghost = CHECK.read_once(work / "source/tools/formal/fraction_verus_bridge_spec.rs")
    planned = []
    for index, job in enumerate(_jobs(_recipe), 1):
        folder = inputs / f"{index:02d}-{job['id']}"
        folder.mkdir(mode=0o700)
        data = control_bytes(healthy, ghost, job, **_recipe_options(_recipe))
        copied = ("actual_source.rs", "predicate_source.rs", "correspondence.json") if dependency else (
            "actual_source.rs", "correspondence.json")
        for name, content in (("proof.rs", data), *((name, CHECK.read_once(generated / name)) for name in copied)):
            with (folder / name).open("xb") as stream:
                stream.write(content)
        binding = {"index": index, "id": job["id"], "target": job["target"],
                   "proof_sha256": digest(data), "healthy_sha256": digest(healthy),
                   "spans": function_spans(data, **_recipe_options(_recipe))}
        write(folder / "control.json", binding, **_recipe_options(_recipe))
        planned.append({**binding, "inputs": folder})
    require(CHECK.read_once(planned[0]["inputs"] / "proof.rs")
            == CHECK.read_once(planned[-1]["inputs"] / "proof.rs"), "Restoration is not byte-identical")
    seal(inputs, **_recipe_options(_recipe))
    return planned, tree(inputs)


def objects(output):
    """Decode complete JSON objects; malformed/truncated JSON is never skipped."""
    require(type(output) is str and "\ufffd" not in output, "Native output is not intact UTF-8 text")
    decoder = json.JSONDecoder(object_pairs_hook=CHECK.unique_object,
        parse_constant=lambda value: (_ for _ in ()).throw(RuntimeError("Nonfinite native JSON")))
    result, position = [], 0
    while position < len(output):
        if output[position].isspace():
            position += 1
            continue
        if output[position] == "{":
            item, position = decoder.raw_decode(output, position)
            require(isinstance(item, dict), "Native JSON object differs")
            result.append(item)
        else:
            end = output.find("\n", position)
            end = len(output) if end < 0 else end
            require(re.fullmatch(r"verification results:: \d+ verified, \d+ errors", output[position:end]) is not None,
                    "Unparsed native output")
            position = end
    return result


def qualify(native, job, expected_identity, source, *, _recipe=None):
    """Require complete whole-crate results and the one intended semantic defect."""
    modes = ({name: "exec" for name in FUNCTIONS} if _closed(_recipe) is None else dict(_recipe.modes))
    fault = job["target"] is not None
    require(native.get("failure") is None and type(native.get("exit_code")) is int
            and native["exit_code"] == (1 if fault else 0), "Native command failed or lost custody")
    decoded = objects(native.get("output"))
    reports = [item for item in decoded if "verification-results" in item]
    require(len(reports) == 1, "Missing or duplicate Verus aggregate")
    report = reports[0]
    require(report.get("verus") == expected_identity == EXPECTED_IDENTITY, "Verus aggregate identity differs")
    values = report["verification-results"]
    flags = {"encountered-error": fault, "encountered-vir-error": False, "success": not fault,
             "errors": 1 if fault else 0, "is-verifying-entire-crate": True}
    require(isinstance(values, dict) and set(values) == {*flags, "verified"}
            and all(type(values.get(key)) is type(value) and values[key] == value for key, value in flags.items())
            and type(values.get("verified")) is int and values["verified"] >= len(modes) - int(fault),
            "Incomplete or unexpected whole-crate query results")
    times = report.get("times-ms")
    require(isinstance(times, dict) and isinstance(times.get("smt"), dict), "Malformed SMT timing metadata")
    groups = times["smt"].get("smt-run-module-times")
    require(isinstance(groups, list) and groups, "Missing per-function verification inventory")
    functions = {}
    for group in groups:
        require(isinstance(group, dict) and isinstance(group.get("function-breakdown"), list)
                and group["function-breakdown"], "Malformed function breakdown")
        for item in group["function-breakdown"]:
            require(isinstance(item, dict) and type(item.get("function")) is str
                    and type(item.get("success")) is bool and item["function"] in modes
                    and item.get("mode:") == modes[item["function"]],
                    "Malformed function identity/mode/status")
            name = item["function"]
            require(name not in functions, "Duplicate full function identity")
            functions[name] = item["success"]
    require(functions == {name: name != job["target"] for name in modes},
            "Expected selected predicates/lemma and three derived Clone results differ")
    diagnostics = [item for item in decoded if item.get("$message_type") == "diagnostic"]
    require(len(decoded) == len(diagnostics) + 1, "Unknown native JSON object")
    errors = [item for item in diagnostics if item.get("level") == "error"]
    if not fault:
        require(not errors, "Healthy bridge contains a compiler/verifier error")
    else:
        semantic = [item for item in errors if item.get("message") == "postcondition not satisfied"]
        semantic_keys = [json.dumps(item, sort_keys=True, allow_nan=False) for item in semantic]
        aborts = [item for item in errors if item not in semantic]
        abort_pattern = (rf"aborting due to {len(semantic)} previous error"
                         + ("s" if len(semantic) != 1 else "") + r"(?:; \d+ warnings? emitted)?")
        require(semantic and len(set(semantic_keys)) == len(semantic) and len(aborts) <= 1
                and all(re.fullmatch(abort_pattern, str(item.get("message", ""))) for item in aborts),
                "Control has a missing, duplicate or competing error")
        expected = job["spans"][job["target"]]

        def local(span):
            return (isinstance(span, dict) and span.get("file_name") == source
                    and type(span.get("line_start")) is int and type(span.get("line_end")) is int
                    and span["line_start"] <= span["line_end"])

        def in_return(span):
            return (local(span) and expected["return_start"] <= span["line_start"]
                    <= span["line_end"] <= expected["return_end"])

        def at_clause(span):
            if "clause_start" in expected:
                return (local(span) and expected["clause_start"] <= span["line_start"]
                        <= span["line_end"] <= expected["clause_end"])
            return local(span) and span["line_start"] == span["line_end"] == expected["returns_line"]

        for item in semantic:
            spans = item.get("spans")
            require(isinstance(spans, list) and all(isinstance(span, dict)
                        and type(span.get("is_primary")) is bool for span in spans),
                    "Semantic diagnostic spans are missing or malformed")
            require(all(in_return(span) or at_clause(span) for span in spans),
                    "Semantic diagnostic span is outside its target clause or return expression")
            primaries = [span for span in spans if span["is_primary"]]
            require(primaries and all(in_return(span) or at_clause(span) for span in primaries),
                    "Control primary span is outside its target clause or return expression")
            require(any(in_return(span) for span in spans),
                    "Control is not linked to its target return expression")
            require(any(at_clause(span) and span.get("label") == "failed this postcondition" for span in spans),
                    "Control is not linked to its target returns clause")
    return {"qualified": True, "verification_results": values, "functions": functions}


def payload_identity(payload):
    """Bind output identity to the same fully hash-admitted payload metadata."""
    value = CHECK.decode(CHECK.read_once(payload / "version.json", maximum=4096))
    require(value == {"verus": EXPECTED_IDENTITY}, "Pinned payload version metadata differs")
    return value["verus"]


def guard(work, state, *, _recipe=None):
    """Reobserve live/frozen source, all inputs and every completed receipt."""
    require(original_sources(**_recipe_options(_recipe)) == state["original"] and tree(work / "source") == state["source"],
            "Original/frozen source drift")
    if state["inputs"] is not None:
        require(tree(work / "inputs") == state["inputs"], "Generated input drift")
    for path, expected in state["completed"]:
        require(tree(path) == expected, "Completed evidence drift")
    CHECK.RUNTIME._owned_work(work, ROOT)
    capacity(work, **_recipe_options(_recipe))


def execute_job(work, state, job, record, *, _recipe=None):
    """Run exactly one admitted command with source/tool/receipt bookends."""
    guard(work, state, **_recipe_options(_recipe))
    folder = work / "jobs" / f"{job['index']:02d}-{job['id']}"
    folder.mkdir(mode=0o700)
    try:
        tick = time.monotonic()
        stable, payload = PILOT.admit()
        expected_identity = payload_identity(payload)
        state["last_admission"] = (stable, payload, expected_identity)
        write(folder / "admission.json", stable, **_recipe_options(_recipe))
        os.chmod(folder / "admission.json", 0o400)
        admission_before = identity(folder / "admission.json", frozen=True)
        record["admission_seconds"] = time.monotonic() - tick
        relative = "/work/" + folder.relative_to(work).as_posix()
        source = "/work/" + (job["inputs"] / "proof.rs").relative_to(work).as_posix()
        argv = PILOT.command(payload, source, relative + "/exec.trace")
        record["command"] = argv
        sealed = [work / "source", work / "inputs", folder / "admission.json",
                  *(path for path, _ in state["completed"])]
        guard(work, state, **_recipe_options(_recipe))
        tick = time.monotonic()
        try:
            record["command_attempted"] = True
            native = CHECK.RUNTIME.run(ROOT, work, argv,
                readonly=[*stable["readonly_roots"], payload], sealed=sealed, cwd=job["inputs"],
                environment={"VERUS_USE_RUSTUP": "0", "VERUS_ROOT": str(payload),
                             "VERUS_Z3_PATH": str(payload / "z3")},
                timeout=min(COMMAND_SECONDS, remaining(**_recipe_options(_recipe))), cpu_seconds=COMMAND_SECONDS,
                address_space_bytes=8 * 1024**3, build_jobs=1, process_limit=128)
            record.update(native=native, execute_seconds=time.monotonic() - tick)
        finally:
            # Source/resource failure cannot suppress the independent tool bookend.
            try:
                guard(work, state, **_recipe_options(_recipe))
            finally:
                stable_after, payload_after = PILOT.admit()
                require(stable_after == stable and payload_after == payload
                        and payload_identity(payload_after) == expected_identity
                        and identity(folder / "admission.json", frozen=True) == admission_before,
                        "Tool identity or admission receipt changed during command")
                guard(work, state, **_recipe_options(_recipe))
        observed = PILOT.check_trace(folder / "exec.trace", payload)
        require(set(observed) == {"/usr/bin/env", *(str(payload / name) for name in ("verus", "rust_verify", "z3"))},
                "A required executable identity was not observed")
        record["observed_executables"] = observed
        record.update(qualify(native, job, expected_identity, source, **_recipe_options(_recipe)))
    except Exception as error:
        record["error"] = f"{type(error).__name__}: {error}"
        raise
    finally:
        try:
            write(folder / "execution.json", record, **_recipe_options(_recipe))
        finally:
            # A failed first command still becomes immutable retained evidence.
            seal(folder, **_recipe_options(_recipe))
            state["completed"].append((folder, tree(folder)))
    return record


def run_campaign(*, _recipe=None):
    """Create the sole attempt; stop on the first unqualified job, without repair."""
    recipe = _closed(_recipe)
    work = _work(recipe)
    started_unix = STARTED_UNIX_NS if recipe is None else recipe.started_unix_ns
    started_monotonic = STARTED_MONOTONIC_NS if recipe is None else recipe.started_monotonic_ns
    deadline_monotonic = DEADLINE_MONOTONIC_NS if recipe is None else recipe.deadline_monotonic_ns
    boot = BOOT_ID if recipe is None else recipe.boot_id
    if recipe is _CONSTRUCTOR_POST_RECIPE:
        passed = "constructor-post-dependency-checkpoint-passed"
    else:
        passed = "pure-bridge-checkpoint-passed" if recipe is None else "bit-length-dependency-checkpoint-passed"
    remaining(**_recipe_options(recipe))
    require(work.parent == work.parent.resolve(strict=True) and not work.exists() and not work.is_symlink(),
            "Use only the fresh fixed work root; retries are forbidden")
    work.mkdir(mode=0o700)
    summary = {"status": "DEFER", "adopted": False, "caller_imports": 0, "jobs": [],
               "maximum_commands": 5, "retries": 0, "started_unix_ns": started_unix,
               "started_monotonic_ns": started_monotonic, "deadline_monotonic_ns": deadline_monotonic,
               "boot_id": boot}
    state = None
    try:
        capacity(work, **_recipe_options(recipe))
        original, source = freeze_sources(work, **_recipe_options(recipe))
        state = {"original": original, "source": source, "inputs": None, "completed": [], "last_admission": None}
        planned, inputs = prepare_inputs(work, **_recipe_options(recipe))
        state["inputs"] = inputs
        summary.update(source_files=original, frozen_source=source, generated_inputs=inputs,
                       executed_python_sources=SOURCE_BINDING.snapshot())
        (work / "jobs").mkdir(mode=0o700)
        for job in planned:
            record = {key: value for key, value in job.items() if key != "inputs"}
            record["qualified"] = False
            summary["jobs"].append(record)
            execute_job(work, state, job, record, **_recipe_options(recipe))
        require(len(summary["jobs"]) == 5 and all(job["qualified"] for job in summary["jobs"]),
                "The fixed five-command schedule is incomplete")
        summary["status"] = passed
    except Exception as error:
        summary["error"] = f"{type(error).__name__}: {error}"
    finally:
        try:
            if state is not None:
                try:
                    guard(work, state, **_recipe_options(recipe))
                finally:
                    if state["last_admission"] is not None:
                        stable, payload = PILOT.admit()
                        expected_stable, expected_payload, expected_identity = state["last_admission"]
                        require(stable == expected_stable and payload == expected_payload
                                and payload_identity(payload) == expected_identity, "Final tool identity drift")
                        summary["final_admission"] = stable
                        guard(work, state, **_recipe_options(recipe))
                summary["source_input_receipt_bookends_unchanged"] = True
            summary["retained_bytes"] = capacity(work, **_recipe_options(recipe))
        except Exception as error:
            summary["status"] = "DEFER"
            summary["final_error"] = f"{type(error).__name__}: {error}"
        summary["elapsed_from_preparation_seconds"] = (time.monotonic_ns() - started_monotonic) / 10**9
        write(work / "summary.json", summary, **_recipe_options(recipe))
    print(json.dumps({"status": summary["status"], "attempted_jobs": len(summary["jobs"]),
                      "dispatched_commands": sum(job.get("command_attempted", False) for job in summary["jobs"]),
                      "qualified_jobs": sum(job["qualified"] for job in summary["jobs"]),
                      "summary": str(work / "summary.json"), "error": summary.get("error"),
                      "final_error": summary.get("final_error")}))
    return 0 if summary["status"] == passed else 1


def run_bit_length_campaign():
    """Execute only the distinct fixed full-raw dependency card; never resume v2."""
    return run_campaign(_recipe=_BIT_LENGTH_RECIPE)


def run_constructor_post_campaign():
    """Execute only the fixed unrestricted constructor-Post dependency card."""
    return run_campaign(_recipe=_CONSTRUCTOR_POST_RECIPE)


if __name__ == "__main__":
    require(sys.argv[1:] == [], "This finite card accepts no CLI, tool, profile, budget or output overrides")
    sys.exit(run_campaign())
