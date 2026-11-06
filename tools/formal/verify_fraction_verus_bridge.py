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
_COMPLEMENT_POST_CONTROLLER = "tools/formal/verify_fraction_complement_post.py"
_COMPLEMENT_POST_TESTS = "tools/formal/test_verify_fraction_complement_post.py"
_COMPLEMENT_POST_MODES = (
    ("proof::lemma_bridge_bit_length_power", "proof"),
    ("proof::lemma_bridge_u128_shift_power", "proof"),
    ("proof::lemma_bridge_complement_reject_outside", "proof"),
    ("proof::composition_constructor_admitted", "exec"),
    ("proof::composition_checked", "exec"),
    ("proof::composition_constructor_post", "exec"),
    ("proof::composition_complement_post", "exec"),
    ("proof::IntervalError::clone", "exec"),
    ("proof::FixtureEnclosureError::clone", "exec"),
    ("proof::Fraction::clone", "exec"),
)
_COMPLEMENT_POST_RECIPE = _Recipe(
    Path.home() / ".local/share/nevrotick/formal-core-fraction-complement-post-20261106-v2",
    1793935038999026301, 31706832453750, 35306832453750,
    "d3fc5615-e867-45cd-b74f-4a7413afc5ab", _COMPLEMENT_POST_MODES,
    (("healthy", None), ("shift_wrong_theorem", "proof::lemma_bridge_u128_shift_power"),
     ("outside_wrong_theorem", "proof::lemma_bridge_complement_reject_outside"),
     ("complement_wrong_error", "proof::composition_complement_post"),
     ("complement_wrong_scale", "proof::composition_complement_post"),
     ("complement_wrong_conservation", "proof::composition_complement_post"), ("restored", None)))
_RATIONAL_ORDER_CONTROLLER = "tools/formal/verify_fraction_rational_order.py"
_RATIONAL_ORDER_TESTS = "tools/formal/test_verify_fraction_rational_order.py"
_RATIONAL_ORDER_MODES = (
    ("proof::lemma_bridge_bit_length_power", "proof"),
    ("proof::lemma_bridge_u128_shift_power", "proof"),
    ("proof::lemma_bridge_complement_reject_outside", "proof"),
    ("proof::lemma_bridge_u128_shifted_u64", "proof"),
    ("proof::lemma_bridge_denominator_factor", "proof"),
    ("proof::lemma_bridge_positive_factor_order", "proof"),
    ("proof::lemma_bridge_rational_gap_dominates", "proof"),
    ("proof::composition_constructor_admitted", "exec"),
    ("proof::composition_checked", "exec"),
    ("proof::composition_constructor_post", "exec"),
    ("proof::composition_rational_order", "exec"),
    ("proof::composition_complement_post", "exec"),
    ("proof::IntervalError::clone", "exec"),
    ("proof::FixtureEnclosureError::clone", "exec"),
    ("proof::Fraction::clone", "exec"),
)
_RATIONAL_ORDER_RECIPE = _Recipe(
    Path.home() / ".local/share/nevrotick/formal-core-fraction-rational-order-20261106-v1",
    1793937385474478571, 34053307946337, 37653307946337,
    "d3fc5615-e867-45cd-b74f-4a7413afc5ab", _RATIONAL_ORDER_MODES,
    (("healthy", None), ("shift_wrong_theorem", "proof::lemma_bridge_u128_shift_power"),
     ("outside_wrong_theorem", "proof::lemma_bridge_complement_reject_outside"),
     ("complement_wrong_error", "proof::composition_complement_post"),
     ("complement_wrong_scale", "proof::composition_complement_post"),
     ("complement_wrong_conservation", "proof::composition_complement_post"),
     ("variable_shift_wrong_theorem", "proof::lemma_bridge_u128_shifted_u64"),
     ("factor_wrong_theorem", "proof::lemma_bridge_denominator_factor"),
     ("positive_order_wrong_theorem", "proof::lemma_bridge_positive_factor_order"),
     ("domination_wrong_theorem", "proof::lemma_bridge_rational_gap_dominates"),
     ("rational_invalid", "proof::composition_rational_order"),
     ("rational_left_zero", "proof::composition_rational_order"),
     ("rational_right_zero", "proof::composition_rational_order"),
     ("rational_equal_shifts", "proof::composition_rational_order"),
     ("rational_left_small_gap", "proof::composition_rational_order"),
     ("rational_right_small_gap", "proof::composition_rational_order"),
     ("rational_left_large_gap", "proof::composition_rational_order"),
     ("rational_right_large_gap", "proof::composition_rational_order"), ("restored", None)))

_LEMMA_THEOREM = (b"    ensures\n"
    b"        (64 - u64_leading_zeros(n) <= s as int)\n"
    b"            <==> ((n as int) < pow(2, s as nat)),\n")
_WRONG_LEMMA_THEOREM = (b"    ensures\n"
    b"        ((64 - u64_leading_zeros(n) <= s as int)\n"
    b"            <==> ((n as int) < pow(2, s as nat)))\n"
    b"            && n != 0,\n")
_SHIFT_THEOREM = (b"    ensures\n"
    b"        (1 <= s && s <= 53)\n"
    b"            ==> ((1_u128 << s) as int == pow(2, s as nat)),\n")
_WRONG_SHIFT_THEOREM = (b"    ensures\n"
    b"        ((1 <= s && s <= 53)\n"
    b"            ==> ((1_u128 << s) as int == pow(2, s as nat)))\n"
    b"            && s != 1,\n")
_OUTSIDE_THEOREM = (b"    ensures\n"
    b"        (bridge_checked(Fraction { numerator: n, shift: s }) && (s == 0 || s > 53))\n"
    b"            ==> (2 * (n as int) < bridge_denominator(s)),\n")
_WRONG_OUTSIDE_THEOREM = (b"    ensures\n"
    b"        ((bridge_checked(Fraction { numerator: n, shift: s }) && (s == 0 || s > 53))\n"
    b"            ==> (2 * (n as int) < bridge_denominator(s)))\n"
    b"            && (n != 0 || s != 0),\n")


# Each false conclusion is additive: retained caller facts remain available.
_RATIONAL_PROOF_CONTRACTS = (
    ("variable_shift_wrong_theorem", b"proof fn lemma_bridge_u128_shifted_u64(n: u64, g: u32)",
     b"    ensures\n"
     b"        g < 64 ==> ((n as u128) << g) == ((n as u128) << (g as u128)),\n"
     b"        g < 64 ==> (((n as u128) << g) as int == (n as int) * pow(2, g as nat)),\n"
     b"        g < 64 ==> (0 <= (n as int) * pow(2, g as nat) < pow(2, 127)),\n",
     b"        g < 64 ==> (((n as u128) << g) as int == (n as int) * pow(2, (g as nat) + 1)),\n",
     b"    if g < 64 {\n"),
    ("factor_wrong_theorem", b"proof fn lemma_bridge_denominator_factor(lo: u32, hi: u32)",
     b"    ensures\n"
     b"        0 < bridge_denominator(lo),\n"
     b"        0 < bridge_denominator(hi),\n"
     b"        lo <= hi ==> bridge_denominator(hi)\n"
     b"            == bridge_denominator(lo) * pow(2, (hi as int - lo as int) as nat),\n",
     b"        lo <= hi ==> bridge_denominator(hi) == bridge_denominator(lo) * pow(2, hi as nat),\n",
     b"    if lo <= hi {\n"),
    ("positive_order_wrong_theorem", b"proof fn lemma_bridge_positive_factor_order(a: int, b: int, k: int)",
     b"    ensures\n"
     b"        k > 0 ==> ((a * k < b * k) <==> (a < b)),\n"
     b"        k > 0 ==> ((a * k > b * k) <==> (a > b)),\n"
     b"        k > 0 ==> ((a * k == b * k) <==> (a == b)),\n",
     b"        k > 0 ==> ((a * k < b * k) <==> (a > b)),\n",
     b"    if k > 0 {\n"),
    ("domination_wrong_theorem", b"proof fn lemma_bridge_rational_gap_dominates(n: u64, m: u64, g: u32)",
     b"    ensures\n"
     b"        (1 <= n && (m as int) < pow(2, 53) && 64 <= g)\n"
     b"            ==> ((m as int) < (n as int) * pow(2, g as nat)),\n",
     b"        (1 <= n && (m as int) < pow(2, 53) && 64 <= g) ==> ((m as int) > (n as int) * pow(2, g as nat)),\n",
     b"    if 1 <= n && (m as int) < pow(2, 53) && 64 <= g {\n"),
)
_RATIONAL_VALID_POSITIVE = (b"bridge_checked(left) && bridge_checked(right)"
                            b" && left.numerator != 0 && right.numerator != 0")
_RATIONAL_BRANCH_GUARDS = (
    ("rational_invalid", b"!bridge_checked(left) || !bridge_checked(right)"),
    ("rational_left_zero", b"bridge_checked(left) && bridge_checked(right) && left.numerator == 0"),
    ("rational_right_zero", b"bridge_checked(left) && bridge_checked(right)"
                            b" && left.numerator != 0 && right.numerator == 0"),
    ("rational_equal_shifts", _RATIONAL_VALID_POSITIVE + b" && left.shift == right.shift"),
    ("rational_left_small_gap", _RATIONAL_VALID_POSITIVE
        + b" && left.shift < right.shift && (right.shift as int) - (left.shift as int) < 64"),
    ("rational_right_small_gap", _RATIONAL_VALID_POSITIVE
        + b" && right.shift < left.shift && (left.shift as int) - (right.shift as int) < 64"),
    ("rational_left_large_gap", _RATIONAL_VALID_POSITIVE
        + b" && left.shift < right.shift && 64 <= (right.shift as int) - (left.shift as int)"),
    ("rational_right_large_gap", _RATIONAL_VALID_POSITIVE
        + b" && right.shift < left.shift && 64 <= (left.shift as int) - (right.shift as int)"),
)


def _closed(recipe):
    """Accept the legacy default or one reviewed immutable dependency card."""
    require(recipe is None or recipe is _BIT_LENGTH_RECIPE or recipe is _CONSTRUCTOR_POST_RECIPE
            or recipe is _COMPLEMENT_POST_RECIPE or recipe is _RATIONAL_ORDER_RECIPE,
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
    if recipe is _RATIONAL_ORDER_RECIPE:
        return tuple(sorted(set(PILOT.SOURCES) | set(PROJECTION.rational_input_names()) | set(_BINDING_FILES)
                            | {CONTROLLER, TESTS, _BIT_LENGTH_CONTROLLER, _BIT_LENGTH_TESTS,
                               _CONSTRUCTOR_POST_CONTROLLER, _CONSTRUCTOR_POST_TESTS,
                               _COMPLEMENT_POST_CONTROLLER, _COMPLEMENT_POST_TESTS,
                               _RATIONAL_ORDER_CONTROLLER, _RATIONAL_ORDER_TESTS}))
    if recipe is _COMPLEMENT_POST_RECIPE:
        return tuple(sorted(set(PILOT.SOURCES) | set(PROJECTION.complement_input_names()) | set(_BINDING_FILES)
                            | {CONTROLLER, TESTS, _BIT_LENGTH_CONTROLLER, _BIT_LENGTH_TESTS,
                               _CONSTRUCTOR_POST_CONTROLLER, _CONSTRUCTOR_POST_TESTS,
                               _COMPLEMENT_POST_CONTROLLER, _COMPLEMENT_POST_TESTS}))
    if recipe is _CONSTRUCTOR_POST_RECIPE:
        return tuple(sorted(set(PILOT.SOURCES) | set(PROJECTION.dependency_input_names()) | set(_BINDING_FILES)
                            | {CONTROLLER, TESTS, _BIT_LENGTH_CONTROLLER, _BIT_LENGTH_TESTS,
                               _CONSTRUCTOR_POST_CONTROLLER, _CONSTRUCTOR_POST_TESTS}))
    return tuple(sorted(set(PILOT.SOURCES) | set(PROJECTION.dependency_input_names()) | set(_BINDING_FILES)
                        | {CONTROLLER, TESTS, _BIT_LENGTH_CONTROLLER, _BIT_LENGTH_TESTS}))


def _command_seconds(recipe):
    """Only the new rational card tightens the existing per-command wall/CPU cap."""
    return 90 if _closed(recipe) is _RATIONAL_ORDER_RECIPE else COMMAND_SECONDS


def _rational_dispatch_capacity(recipe):
    """Reserve the complete fixed rational schedule before its first admission."""
    if _closed(recipe) is _RATIONAL_ORDER_RECIPE:
        require(remaining(_recipe=recipe) >= 19 * 90 + 120,
                "Insufficient original clock for the complete nineteen-command schedule")


def _jobs(recipe):
    """Return fresh descriptions of only the selected fixed dependency schedule."""
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
    if _closed(_recipe) is _RATIONAL_ORDER_RECIPE:
        return _validate_rational_order_metadata(details, source, generated)
    if _recipe is _COMPLEMENT_POST_RECIPE:
        return _validate_complement_post_metadata(details, source, generated)
    if _recipe is _CONSTRUCTOR_POST_RECIPE:
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


def _validate_complement_post_metadata(details, source, generated):
    """Independently reconstruct all raw complement spans, lemmas, hooks and claims."""
    names = PROJECTION.complement_input_names()
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
    fragment_spans = ((0, 1202), (2684, 3731))
    fragment = b"".join(prefix[first:last] for first, last in fragment_spans)
    predicates = PROJECTION.annotated_complement_post(fragment)
    require(len(fragment) == 2249
            and digest(fragment) == "f8a1e46efbbbf8b252d2dae75fb73493613f9fea5f745af85bc8a45616890d0d"
            and PROJECTION.erase_complement_post_annotations(predicates) == fragment,
            "Dependency fragment erasure differs")
    ghost = sources["tools/formal/fraction_verus_bridge_spec.rs"]
    lemma = sources[PROJECTION.BIT_LENGTH_LEMMAS]
    require(digest(ghost) == PROJECTION.GHOST_SHA256 and digest(lemma) == PROJECTION.BIT_LENGTH_LEMMAS_SHA256,
            "Dependency ghost/lemma source differs")
    complement_lemma = sources[PROJECTION.COMPLEMENT_LEMMAS]
    require(digest(complement_lemma) == PROJECTION.COMPLEMENT_LEMMAS_SHA256,
            "Complement lemma source differs")
    types = b"\n".join(selected[name] for name in PROJECTION.TYPE_NAMES)
    proof = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
             b"// UNPROVED complement-Post dependency source; full bridge acceptance remains open.\n"
             b"use std::cmp::Ordering;\nuse vstd::prelude::*;\n\nverus! {\n\n"
             + types + b"\n\n" + ghost + b"\n" + lemma + b"\n" + complement_lemma
             + b"\n" + predicates + b"\n}\n")
    actual = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
              b"// Correspondence input only; no caller-import entrypoint or permission.\n"
              b"use std::cmp::Ordering;\n\n" + types + b"\nimpl Fraction {\n"
              + b"\n".join(selected[name] for name in ("zero", "new", "compare", "complement"))
              + b"\n}\n\n" + selected["fraction_sum"] + b"\n" + selected["addition"])
    hooks = [{"function": "proof::" + name, "source_byte_offset": prefix.index(anchor),
              "insertion": ("before unchanged leading_zeros subtraction" if index < 2
                            else "at approved complement proof site"),
              "anchor_sha256": digest(anchor), "proof_block_sha256": digest(block)}
             for index, (name, anchor, block) in enumerate((*PROJECTION.BIT_LENGTH_HOOKS, *PROJECTION.COMPLEMENT_HOOKS))]
    expected = {
        "schema_version": 1, "status": "unproved_source_candidate", "proof_commands": 0,
        "scope": "bit-length and complement lemmas and four unrestricted predicate equalities; source preparation only",
        "source_files_sha256": source_hashes, "original_sources_sha256": originals,
        "correspondence": expected_spans, "prefix_sha256": digest(prefix), "prefix_bytes": 3731,
        "fragment_source_spans": [{"source_byte_offsets_half_open": list(offsets),
                                   "sha256": digest(prefix[offsets[0]:offsets[1]])}
                                  for offsets in fragment_spans], "fragment_sha256": digest(fragment),
        "annotated_fragment_sha256": digest(predicates), "ghost_sha256": digest(ghost),
        "lemma_sha256": digest(lemma), "complement_lemma_sha256": digest(complement_lemma),
        "predicate_relations": [PROJECTION.DECLARATIONS[index][1].decode() for index in (0, 1, 2, 5)],
        "full_bridge_predicate_relations": [relation.decode() for _, relation in PROJECTION.DECLARATIONS],
        "expected_function_modes": dict(_COMPLEMENT_POST_RECIPE.modes), "proof_hooks": hooks,
        "raw_domains": {"lemma": "every n:u64 and s:u32; no precondition",
                        "constructor_admitted": "every numerator:u64 and shift:u32; no precondition",
                        "checked": "every raw Fraction { numerator:u64, shift:u32 }; no precondition",
                        "constructor_post": "every numerator:u64, shift:u32 and independently chosen &Result<Fraction, FixtureEnclosureError>; raw Ok fields and every error variant; no precondition",
                        "shift_power": "every s:u32; no precondition; guarded shift/power implication for 1 <= s <= 53",
                        "reject_outside": "every n:u64 and s:u32; no precondition; guarded checked outside-window implication",
                        "complement_post": "every raw Fraction { numerator:u64, shift:u32 } and independently chosen &Result<Fraction, FixtureEnclosureError>; raw Ok fields and every error variant; no precondition"},
        "upstream_dependencies": {
            "commit": "8bf68ee741df833ea5d0fe6a2255126fdea87e47",
            "paths": ["source/vstd/std_specs/bits.rs", "source/vstd/arithmetic/power.rs",
                      "source/vstd/arithmetic/power2.rs", "source/vstd/arithmetic/div_mod.rs",
                      "source/vstd/bits.rs", "source/vstd/std_specs/convert.rs"],
            "status": "source API references only; no new admission or proof"},
        "remaining": ["exact Verus compatibility and all lemma assertions/termination",
                      "u32 leading_zeros result and subtraction-to-int inference",
                      "four completed full-raw predicate equalities and wrong-contract/restoration controls",
                      "all six full-domain bridge equalities and their controls",
                      "native Ordering equality correspondence, rational alignment and complement",
                      "separate caller-import profile admission and original addition union"],
    }
    require(set(details) == {*expected, "generated_sha256"}
            and type(details.get("schema_version")) is int and type(details.get("proof_commands")) is int
            and type(details.get("prefix_bytes")) is int
            and type(details.get("fragment_source_spans")) is list
            and all(details.get(key) == value for key, value in expected.items()),
            "Dependency projection metadata differs")
    for span in details["fragment_source_spans"]:
        require(type(span.get("source_byte_offsets_half_open")) is list
                and all(type(value) is int for value in span["source_byte_offsets_half_open"]),
                "Complement fragment offsets differ")
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


def _validate_rational_order_metadata(details, source, generated):
    """Independently reconstruct all raw rational spans, lemmas, hooks and claims."""
    names = PROJECTION.rational_input_names()
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
    fragment_spans = ((0, 2333), (2684, 3731))
    fragment = b"".join(prefix[first:last] for first, last in fragment_spans)
    predicates = PROJECTION.annotated_rational_order(fragment)
    require(len(fragment) == 3380
            and digest(fragment) == "b5e0a0dcdf924add0f9447bfce57555f39c5bc73deae656bec9993a576796a8d"
            and PROJECTION.erase_rational_order_annotations(predicates) == fragment,
            "Dependency fragment erasure differs")
    ghost = sources["tools/formal/fraction_verus_bridge_spec.rs"]
    lemma = sources[PROJECTION.BIT_LENGTH_LEMMAS]
    require(digest(ghost) == PROJECTION.GHOST_SHA256 and digest(lemma) == PROJECTION.BIT_LENGTH_LEMMAS_SHA256,
            "Dependency ghost/lemma source differs")
    complement_lemma = sources[PROJECTION.COMPLEMENT_LEMMAS]
    require(digest(complement_lemma) == PROJECTION.COMPLEMENT_LEMMAS_SHA256,
            "Complement lemma source differs")
    rational_lemma = sources[PROJECTION.RATIONAL_LEMMAS]
    require(digest(rational_lemma) == PROJECTION.RATIONAL_LEMMAS_SHA256, "Rational lemma source differs")
    types = b"\n".join(selected[name] for name in PROJECTION.TYPE_NAMES)
    proof = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
             b"// UNPROVED rational-order dependency source; full bridge acceptance remains open.\n"
             b"use std::cmp::Ordering;\nuse vstd::prelude::*;\n\nverus! {\n\n"
             + types + b"\n\n" + ghost + b"\n" + lemma + b"\n" + complement_lemma
             + b"\n" + rational_lemma + b"\n" + predicates + b"\n}\n")
    actual = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
              b"// Correspondence input only; no caller-import entrypoint or permission.\n"
              b"use std::cmp::Ordering;\n\n" + types + b"\nimpl Fraction {\n"
              + b"\n".join(selected[name] for name in ("zero", "new", "compare", "complement"))
              + b"\n}\n\n" + selected["fraction_sum"] + b"\n" + selected["addition"])
    hooks = [{"function": "proof::" + name, "source_byte_offset": prefix.index(anchor),
              "insertion": ("before unchanged leading_zeros subtraction" if index < 2
                            else "at approved complement proof site" if index < 5
                            else "at approved rational-order proof site"),
              "anchor_sha256": digest(anchor), "proof_block_sha256": digest(block)}
             for index, (name, anchor, block) in enumerate((*PROJECTION.BIT_LENGTH_HOOKS, *PROJECTION.COMPLEMENT_HOOKS,
                                                               *PROJECTION.RATIONAL_HOOKS))]
    expected = {
        "schema_version": 1, "status": "unproved_source_candidate", "proof_commands": 0,
        "scope": "bit-length, complement and rational lemmas and five unrestricted predicate equalities; source preparation only",
        "source_files_sha256": source_hashes, "original_sources_sha256": originals,
        "correspondence": expected_spans, "prefix_sha256": digest(prefix), "prefix_bytes": 3731,
        "fragment_source_spans": [{"source_byte_offsets_half_open": list(offsets),
                                   "sha256": digest(prefix[offsets[0]:offsets[1]])}
                                  for offsets in fragment_spans], "fragment_sha256": digest(fragment),
        "annotated_fragment_sha256": digest(predicates), "ghost_sha256": digest(ghost),
        "lemma_sha256": digest(lemma), "complement_lemma_sha256": digest(complement_lemma),
        "rational_lemma_sha256": digest(rational_lemma),
        "predicate_relations": [PROJECTION.DECLARATIONS[index][1].decode() for index in (0, 1, 2, 3, 5)],
        "full_bridge_predicate_relations": [relation.decode() for _, relation in PROJECTION.DECLARATIONS],
        "expected_function_modes": dict(_RATIONAL_ORDER_RECIPE.modes), "proof_hooks": hooks,
        "raw_domains": {"lemma": "every n:u64 and s:u32; no precondition",
                        "constructor_admitted": "every numerator:u64 and shift:u32; no precondition",
                        "checked": "every raw Fraction { numerator:u64, shift:u32 }; no precondition",
                        "constructor_post": "every numerator:u64, shift:u32 and independently chosen &Result<Fraction, FixtureEnclosureError>; raw Ok fields and every error variant; no precondition",
                        "shift_power": "every s:u32; no precondition; guarded shift/power implication for 1 <= s <= 53",
                        "reject_outside": "every n:u64 and s:u32; no precondition; guarded checked outside-window implication",
                        "shifted_u64": "every n:u64 and g:u32; no precondition; exact u128 shift/product and u32/u128 shift identity guarded by g < 64",
                        "denominator_factor": "every lo:u32 and hi:u32; no precondition; both powers positive and factorization guarded by lo <= hi",
                        "positive_factor_order": "every a:int, b:int and k:int; no precondition; complete order equivalence guarded by k > 0",
                        "gap_dominates": "every n:u64, m:u64 and g:u32; no precondition; dominance guarded by n >= 1, m < pow(2,53) and g >= 64",
                        "rational_order": "every independent left/right raw Fraction { numerator:u64, shift:u32 }; invalid either side maps to Equal; no precondition",
                        "complement_post": "every raw Fraction { numerator:u64, shift:u32 } and independently chosen &Result<Fraction, FixtureEnclosureError>; raw Ok fields and every error variant; no precondition"},
        "upstream_dependencies": {
            "commit": "8bf68ee741df833ea5d0fe6a2255126fdea87e47",
            "paths": ["source/vstd/std_specs/bits.rs", "source/vstd/arithmetic/power.rs",
                      "source/vstd/arithmetic/power2.rs", "source/vstd/arithmetic/div_mod.rs",
                      "source/vstd/bits.rs", "source/vstd/std_specs/convert.rs",
                      "source/vstd/arithmetic/mul.rs", "source/vstd/std_specs/cmp.rs",
                      "source/vstd/std_specs/num.rs"],
            "status": "source API references only; no new admission or proof"},
        "remaining": ["exact Verus compatibility and all lemma assertions/termination",
                      "u32 leading_zeros result and subtraction-to-int inference",
                      "five completed full-raw predicate equalities and wrong-contract/restoration controls",
                      "all six full-domain bridge equalities and their controls",
                      "native Ordering equality correspondence and comparison Post",
                      "separate caller-import profile admission and original addition union"],
    }
    require(set(details) == {*expected, "generated_sha256"}
            and type(details.get("schema_version")) is int and type(details.get("proof_commands")) is int
            and type(details.get("prefix_bytes")) is int
            and type(details.get("fragment_source_spans")) is list
            and all(details.get(key) == value for key, value in expected.items()),
            "Dependency projection metadata differs")
    for span in details["fragment_source_spans"]:
        require(type(span.get("source_byte_offsets_half_open")) is list
                and all(type(value) is int for value in span["source_byte_offsets_half_open"]),
                "Complement fragment offsets differ")
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


def _complement_post_control(healthy, ghost, job):
    """Apply exactly one closed lemma or complement ghost defect with full reversal."""
    lemma = CHECK.read_once(ROOT / PROJECTION.BIT_LENGTH_LEMMAS)
    complement_lemma = CHECK.read_once(ROOT / PROJECTION.COMPLEMENT_LEMMAS)
    prefix = CHECK.read_once(ROOT / "tools/formal/fraction_composition_spec.rs").split(PROJECTION.PREFIX_MARKER, 1)[0]
    fragment = prefix[:1202] + prefix[2684:3731]
    executable = PROJECTION.annotated_complement_post(fragment)
    require(digest(ghost) == PROJECTION.GHOST_SHA256 and healthy.count(ghost) == 1
            and digest(lemma) == PROJECTION.BIT_LENGTH_LEMMAS_SHA256 and healthy.count(lemma) == 1
            and digest(complement_lemma) == PROJECTION.COMPLEMENT_LEMMAS_SHA256
            and healthy.count(complement_lemma) == 1, "Complement-Post ghost/lemma segment identity differs")
    require(healthy.count(executable) == 1 and PROJECTION.erase_complement_post_annotations(executable) == fragment,
            "Complement-Post executable erasure differs")
    descriptions = {item["id"]: item for item in _jobs(_COMPLEMENT_POST_RECIPE)}
    require(job.get("id") in descriptions and job.get("target") == descriptions[job["id"]]["target"],
            "Complement-Post control identity differs")
    if job["target"] is None:
        return healthy
    if job["id"] == "shift_wrong_theorem":
        segment, old, new = complement_lemma, _SHIFT_THEOREM, _WRONG_SHIFT_THEOREM
    elif job["id"] == "outside_wrong_theorem":
        segment, old, new = complement_lemma, _OUTSIDE_THEOREM, _WRONG_OUTSIDE_THEOREM
    else:
        first = b"spec fn bridge_complement_post("
        require(ghost.count(first) == 1, "Complement-Post ghost scope is ambiguous")
        segment = ghost[ghost.index(first):]
        replacements = {
            "complement_wrong_error": (b"Err(FixtureEnclosureError::Invariant) => !required_success,",
                                       b"Err(FixtureEnclosureError::Invariant) => true,"),
            "complement_wrong_scale": (b"&& value.shift == input.shift", b"&& value.shift >= input.shift"),
            "complement_wrong_conservation": (
                b"&& (value.numerator as int) + (input.numerator as int) == denominator",
                b"&& (value.numerator as int) + (input.numerator as int) <= denominator"),
        }
        old, new = replacements[job["id"]]
    require(healthy.count(segment) == 1 and segment.count(old) == 1 and new not in segment,
            "Complement-Post control replacement is ambiguous")
    faulty = segment.replace(old, new, 1)
    require(faulty.replace(new, old, 1) == segment, "Complement-Post control reversal differs")
    candidate = healthy.replace(segment, faulty, 1)
    require(candidate.replace(faulty, segment, 1) == healthy and candidate.count(executable) == 1
            and candidate.count(lemma) == 1, "Complement-Post control changed executable or restored healthy bytes")
    return candidate


def _rational_order_control(healthy, ghost, job):
    """Change one closed ghost conclusion/relation and preserve every executable byte."""
    lemma = CHECK.read_once(ROOT / PROJECTION.BIT_LENGTH_LEMMAS)
    complement = CHECK.read_once(ROOT / PROJECTION.COMPLEMENT_LEMMAS)
    rational = CHECK.read_once(ROOT / PROJECTION.RATIONAL_LEMMAS)
    prefix = CHECK.read_once(ROOT / "tools/formal/fraction_composition_spec.rs").split(PROJECTION.PREFIX_MARKER, 1)[0]
    fragment = prefix[:2333] + prefix[2684:3731]
    executable = PROJECTION.annotated_rational_order(fragment)
    require(digest(ghost) == PROJECTION.GHOST_SHA256 and healthy.count(ghost) == 1
            and digest(lemma) == PROJECTION.BIT_LENGTH_LEMMAS_SHA256 and healthy.count(lemma) == 1
            and digest(complement) == PROJECTION.COMPLEMENT_LEMMAS_SHA256 and healthy.count(complement) == 1
            and digest(rational) == PROJECTION.RATIONAL_LEMMAS_SHA256 and healthy.count(rational) == 1,
            "Rational ghost/lemma segment identity differs")
    require(healthy.count(executable) == 1 and PROJECTION.erase_rational_order_annotations(executable) == fragment,
            "Rational executable erasure differs")
    descriptions = dict(_RATIONAL_ORDER_RECIPE.jobs)
    require(job.get("id") in descriptions and job.get("target") == descriptions[job["id"]],
            "Rational control identity differs")
    if job["target"] is None:
        return healthy
    identifier = job["id"]
    if identifier == "shift_wrong_theorem":
        segment, old, new = complement, _SHIFT_THEOREM, _WRONG_SHIFT_THEOREM
    elif identifier == "outside_wrong_theorem":
        segment, old, new = complement, _OUTSIDE_THEOREM, _WRONG_OUTSIDE_THEOREM
    elif identifier.startswith("complement_"):
        segment = ghost[ghost.index(b"spec fn bridge_complement_post("):]
        replacements = {
            "complement_wrong_error": (b"Err(FixtureEnclosureError::Invariant) => !required_success,",
                                       b"Err(FixtureEnclosureError::Invariant) => true,"),
            "complement_wrong_scale": (b"&& value.shift == input.shift", b"&& value.shift >= input.shift"),
            "complement_wrong_conservation": (
                b"&& (value.numerator as int) + (input.numerator as int) == denominator",
                b"&& (value.numerator as int) + (input.numerator as int) <= denominator"),
        }
        old, new = replacements[identifier]
    elif identifier in {item[0] for item in _RATIONAL_PROOF_CONTRACTS}:
        _, _, old, extra, _ = next(item for item in _RATIONAL_PROOF_CONTRACTS if item[0] == identifier)
        segment, new = rational, old + extra
    else:
        guards = dict(_RATIONAL_BRANCH_GUARDS)
        require(identifier in guards, "Unknown rational branch control")
        first = b"spec fn bridge_rational_order(left: Fraction, right: Fraction) -> Ordering {\n"
        following = b"\n/// All checked pairs require the exact native ordering and reject every error."
        require(ghost.count(first) == ghost.count(following) == 1, "Rational ghost scope is ambiguous")
        segment = ghost[ghost.index(first):ghost.index(following)]
        require(segment.endswith(b"}\n"), "Rational ghost closing boundary differs")
        body = segment[len(first):-2]
        old = segment
        new = (first + b"    let original = {\n" + body + b"    };\n"
               + b"    if " + guards[identifier] + b" {\n"
               + b"        match original {\n"
               + b"            Ordering::Less => Ordering::Greater,\n"
               + b"            Ordering::Greater => Ordering::Less,\n"
               + b"            Ordering::Equal => Ordering::Less,\n"
               + b"        }\n    } else {\n        original\n    }\n}\n")
    require(healthy.count(segment) == 1 and segment.count(old) == 1 and new not in segment,
            "Rational control replacement is ambiguous")
    faulty = segment.replace(old, new, 1)
    require(faulty.replace(new, old, 1) == segment, "Rational control reversal differs")
    candidate = healthy.replace(segment, faulty, 1)
    require(candidate.replace(faulty, segment, 1) == healthy and candidate.count(executable) == 1
            and candidate.count(lemma) == 1,
            "Rational control changed executable or restored healthy bytes")
    return candidate


def control_bytes(healthy, ghost, job, *, _recipe=None):
    """Change one exact ghost-only leaf and prove reversal and executable erasure."""
    if _closed(_recipe) is _RATIONAL_ORDER_RECIPE:
        return _rational_order_control(healthy, ghost, job)
    if _recipe is _COMPLEMENT_POST_RECIPE:
        return _complement_post_control(healthy, ghost, job)
    if _recipe is _CONSTRUCTOR_POST_RECIPE:
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
    if _recipe is _RATIONAL_ORDER_RECIPE:
        declarations = tuple(PROJECTION.DECLARATIONS[index] for index in (0, 1, 2, 3, 5))
    elif _recipe is _COMPLEMENT_POST_RECIPE:
        declarations = tuple(PROJECTION.DECLARATIONS[index] for index in (0, 1, 2, 5))
    elif _recipe is _CONSTRUCTOR_POST_RECIPE:
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
    if _recipe is _COMPLEMENT_POST_RECIPE or _recipe is _RATIONAL_ORDER_RECIPE:
        closed = (
            (b"proof fn lemma_bridge_u128_shift_power(s: u32)", _SHIFT_THEOREM, _WRONG_SHIFT_THEOREM,
             b"    if 1 <= s && s <= 53 {\n"),
            (b"proof fn lemma_bridge_complement_reject_outside(n: u64, s: u32)",
             _OUTSIDE_THEOREM, _WRONG_OUTSIDE_THEOREM,
             b"    if bridge_checked(Fraction { numerator: n, shift: s }) && (s == 0 || s > 53) {\n"),
        )
        if _recipe is _RATIONAL_ORDER_RECIPE:
            closed += tuple((declaration, theorem, theorem + extra, tail)
                            for _, declaration, theorem, extra, tail in _RATIONAL_PROOF_CONTRACTS)
        for declaration, theorem, altered, tail in closed:
            require(data.count(declaration) == 1, "Complement proof declaration differs")
            start = data.index(declaration) + len(declaration)
            require(data[start:start + 1] == b"\n", "Complement proof header differs")
            header = start + 1
            matches = [item for item in (theorem, altered) if data.startswith(item + b"{\n", header)]
            require(len(matches) == 1, "Complement proof ensures/body boundary differs")
            contract = matches[0]
            clause = header + len(b"    ensures\n")
            clause_end = header + len(contract) - 2
            opening = header + len(contract)
            depth, position = 1, opening + 1
            while depth and position < len(data):
                depth += (data[position] == ord("{")) - (data[position] == ord("}"))
                position += 1
            require(depth == 0, "Complement proof body is incomplete")
            body = data[opening + 1:position - 1]
            require(body.count(tail) == 1, "Complement proof guarded tail differs")
            tail_start = opening + 1 + body.index(tail)
            name = "proof::" + declaration.split(b"fn ", 1)[1].split(b"(", 1)[0].decode()
            result[name] = {"clause_start": data[:clause].count(b"\n") + 1,
                            "clause_end": data[:clause_end].count(b"\n") + 1,
                            "return_start": data[:tail_start].count(b"\n") + 1,
                            "return_end": data[:position].count(b"\n") + 1}
    return result


def prepare_inputs(work, *, _recipe=None):
    """Generate the closed candidates from frozen data without frozen imports."""
    dependency = _closed(_recipe) is not None
    inputs = work / "inputs"
    inputs.mkdir(mode=0o700)
    generated = inputs / "baseline"
    if _recipe is _RATIONAL_ORDER_RECIPE:
        details = PROJECTION.prepare_rational_order(work / "source", generated)
    elif _recipe is _COMPLEMENT_POST_RECIPE:
        details = PROJECTION.prepare_complement_post(work / "source", generated)
    elif _recipe is _CONSTRUCTOR_POST_RECIPE:
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
        if not state["completed"]:
            _rational_dispatch_capacity(_recipe)
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
                timeout=min(_command_seconds(_recipe), remaining(**_recipe_options(_recipe))),
                cpu_seconds=_command_seconds(_recipe),
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
    maximum_commands = 19 if recipe is _RATIONAL_ORDER_RECIPE else (7 if recipe is _COMPLEMENT_POST_RECIPE else 5)
    if recipe is _RATIONAL_ORDER_RECIPE:
        passed = "rational-order-dependency-checkpoint-passed"
    elif recipe is _COMPLEMENT_POST_RECIPE:
        passed = "complement-post-dependency-checkpoint-passed"
    elif recipe is _CONSTRUCTOR_POST_RECIPE:
        passed = "constructor-post-dependency-checkpoint-passed"
    else:
        passed = "pure-bridge-checkpoint-passed" if recipe is None else "bit-length-dependency-checkpoint-passed"
    remaining(**_recipe_options(recipe))
    _rational_dispatch_capacity(recipe)
    require(work.parent == work.parent.resolve(strict=True) and not work.exists() and not work.is_symlink(),
            "Use only the fresh fixed work root; retries are forbidden")
    work.mkdir(mode=0o700)
    summary = {"status": "DEFER", "adopted": False, "caller_imports": 0, "jobs": [],
               "maximum_commands": maximum_commands, "retries": 0, "started_unix_ns": started_unix,
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
        _rational_dispatch_capacity(recipe)
        (work / "jobs").mkdir(mode=0o700)
        for job in planned:
            record = {key: value for key, value in job.items() if key != "inputs"}
            record["qualified"] = False
            summary["jobs"].append(record)
            execute_job(work, state, job, record, **_recipe_options(recipe))
        require(len(summary["jobs"]) == maximum_commands and all(job["qualified"] for job in summary["jobs"]),
                "The fixed nineteen-command schedule is incomplete" if recipe is _RATIONAL_ORDER_RECIPE
                else "The fixed seven-command schedule is incomplete" if recipe is _COMPLEMENT_POST_RECIPE
                else "The fixed five-command schedule is incomplete")
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


def run_complement_post_campaign():
    """Execute only the fixed unrestricted complement-Post dependency card."""
    return run_campaign(_recipe=_COMPLEMENT_POST_RECIPE)


def run_rational_order_campaign():
    """Execute only the fixed full-raw rational-order dependency card."""
    return run_campaign(_recipe=_RATIONAL_ORDER_RECIPE)


if __name__ == "__main__":
    require(sys.argv[1:] == [], "This finite card accepts no CLI, tool, profile, budget or output overrides")
    sys.exit(run_campaign())
