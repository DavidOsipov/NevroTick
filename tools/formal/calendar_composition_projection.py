"""Prepare four jointly necessary calendar obligations from exact source spans.

The complete private helper is preserved. Fixed reviewed byte strings enforce
structural P/S correspondence; this is not a machine H==factor theorem.
Its exact compiled prefix is copied into a
bool-returning observation wrapper; its original early return remains unchanged.
Its exact suffix is copied into a cycle-day wrapper. No solver executes here.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path


SOURCE = "crates/time-domain/src/leap/validation.rs"
SOURCE_SHA256 = "969c082cc83fe013bf6cb64319dc99eb1c6d288430b3dddb6b9580fe5a69939a"
FUNCTION_SHA256 = "284fc2fba9cdecc029fc512ece3de31c5949a73d5597bbb5518f809dd631fba2"
SUBJECT = "calendar_composition"
FUNCTION_HEADER = b"fn is_month_start(seconds: u64) -> bool {\n"
CONTEXT_SHA256 = {
    "crates/time-domain/src/lib.rs": "a6b697df2267e6e72f99df1bcb9b0798dada3af11fbbaa24160415ae73b0861b",
    "crates/time-domain/src/leap.rs": "977aa5762990fd7e3e0ed2bb536e3bff3ab89259b1af9f2370b2a068866e3e5b",
}
# Each complete byte string was reviewed for the same closed scalar-only shape.
# These pins are a source-specific allowlist, not a general Rust equivalence tool.
REVIEWED_FACTORS = {
    "healthy": (
        "284fc2fba9cdecc029fc512ece3de31c5949a73d5597bbb5518f809dd631fba2",
        "e3b128831a6423c97a9c76e23b7056be4428b82eea2bcfbd673c0ab08e1504be",
        "3d4cbee74fe1cf232b11fbf45ee556a6a183cc4d0e3f3bf4320f078e5832f4ad"),
    "reject_month_start": (
        "0d77dcdb4da2ab86eea32845614a1c57509cd0af3018d2775d07215a27f9ce60",
        "67cd524b7599b0bf9ec91fc7527fcfda29c84288d0323461bef92c8753c84e93",
        "3d4cbee74fe1cf232b11fbf45ee556a6a183cc4d0e3f3bf4320f078e5832f4ad"),
    "omit_alignment": (
        "7e64ebb3d7bdc5704de24d1d6d383293929b9bb74bd79d1e438b4509770dfacd",
        "e73081e8d5d25d37dad3aa37aa1a30de5eb65ae32c9643b3aff336f0374339a1",
        "3d4cbee74fe1cf232b11fbf45ee556a6a183cc4d0e3f3bf4320f078e5832f4ad"),
    "omit_century_adjustment": (
        "263afd40418fef622dd59b4b8158c07b6363c40ad41a997c72a7a95bf7bb096f",
        "e3b128831a6423c97a9c76e23b7056be4428b82eea2bcfbd673c0ab08e1504be",
        "1c8ded40879477b85bc5531830a097f93101316edf45b45bafcc1d25c982c705"),
}
PREFIX_HEAD = (b"\n/// Exact compiled prefix; caller supplies None; the early return remains bool.\n"
               b"pub(crate) fn proof_prefix(seconds: u64, output: &mut Option<i128>) -> bool {\n")
PREFIX_TAIL = b"    *output = Some(day_in_cycle);\n    true\n}\n"
SUFFIX_HEAD = (b"\n/// Exact compiled suffix; input must be a cycle day in [0,146097).\n"
               b"pub(crate) fn proof_suffix(day_in_cycle: i128) -> bool {\n")
SUFFIX_TAIL = b"}\n\n"
ORIGINAL_WRAPPER = b"pub(crate) fn proof_original(seconds: u64) -> bool { is_month_start(seconds) }\n"
REQUIRED_ASSERTIONS = {
    "calendar_source_full_domain": ("prefix alignment result", "prefix whole-day envelope", "prefix epoch conversion fits", "prefix positive normalization envelope", "prefix exact epoch mapping", "prefix cycle domain"),
    "calendar_representation_full_domain": ("forward search year bracket", "cycle day has forward representation", "forward ordinal domain"),
    "calendar_suffix_full_domain": ("generated cycle domain", "generated suffix month equivalence"),
    "calendar_gregorian_laws": ("Gregorian segment year recurrence", "Gregorian March month recurrence", "March prefix origin", "March segment total length", "Gregorian cycle period", "arbitrary cycle forward law", "NTP March epoch anchor", "NTP cycle epoch position"),
}
REQUIRED_COVERS = {
    "calendar_source_full_domain": ("source NTP epoch succeeds", "source subday rejects", "source upper word rejects", "source high month succeeds"),
    "calendar_representation_full_domain": ("forward first day represented", "forward final day represented", "forward century day represented", "forward leap February start represented"),
    "calendar_suffix_full_domain": ("suffix cycle origin succeeds", "suffix century March succeeds", "suffix final leap day rejects", "suffix February start succeeds"),
    "calendar_gregorian_laws": ("laws ordinary leap month", "laws non-400 century March", "laws 400-year century March", "laws maximum cycle model"),
}
CONTROLS = {
    "reject_month_start": ("if !seconds.is_multiple_of(86_400) {", "if seconds == 2_274_739_200 || !seconds.is_multiple_of(86_400) {", "calendar_composition_control_success", "required month-start success"),
    "omit_alignment": ("if !seconds.is_multiple_of(86_400) {", "if seconds == u64::MAX {", "calendar_composition_control_alignment", "sub-day rejection"),
    "omit_century_adjustment": (" + day_in_cycle / 36_524", "", "calendar_composition_control_century", "century month-start boundary"),
}


def digest(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def schedule() -> list[dict]:
    result = []
    for restored in (False, True):
        if restored:
            for case, (_, _, harness, intended) in CONTROLS.items():
                result.append({"subject": SUBJECT, "case": case, "harness": harness,
                               "intended_failure": intended, "restored_healthy": False})
        for harness in REQUIRED_ASSERTIONS:
            result.append({"subject": SUBJECT, "case": "healthy", "harness": harness,
                           "intended_failure": None, "restored_healthy": restored})
    return result


def reviewed_factors(function: bytes, case: str) -> tuple[bytes, bytes]:
    """Enforce one reviewed complete byte string and exact factor reconstruction.

    The reviewed shape is scalar-only P then S: P has one bool early return,
    immutable i128 locals and no effects; S has only day_in_cycle as a free value,
    immutable i128 locals and builtin matches!. Metadata explains that review;
    the closed byte allowlist enforces it. No mathematical claim is assumed.
    """
    expected = REVIEWED_FACTORS.get(case)
    if expected is None or digest(function) != expected[0]:
        raise ValueError("Unreviewed complete calendar function shape")
    separator = b"    let year_in_cycle = "
    if (not function.startswith(FUNCTION_HEADER) or not function.endswith(b"}\n")
            or function.count(separator) != 1):
        raise ValueError("Calendar factor boundaries are ambiguous")
    split = function.index(separator)
    prefix, suffix = function[len(FUNCTION_HEADER):split], function[split:-2]
    if (FUNCTION_HEADER + prefix + suffix + b"}\n" != function
            or digest(prefix) != expected[1] or digest(suffix) != expected[2]
            or b"seconds" in suffix or b"march_days" in suffix):
        raise ValueError("Calendar factors differ from reviewed bindings")
    return prefix, suffix


def factor_wrappers(prefix: bytes, suffix: bytes) -> bytes:
    """Emit only the reviewed observation/visibility templates around exact factors."""
    return PREFIX_HEAD + prefix + PREFIX_TAIL + SUFFIX_HEAD + suffix + SUFFIX_TAIL + ORIGINAL_WRAPPER


def prepare_calendar_composition(source_root: Path, inputs: Path, case: str,
                                 harness_path: Path) -> dict:
    if case != "healthy" and case not in CONTROLS:
        raise ValueError("Unknown calendar-composition control")
    original = (source_root / SOURCE).read_bytes()
    if digest(original) != SOURCE_SHA256:
        raise ValueError("Reviewed leap-validation source changed")
    selected = b"".join(original.splitlines(keepends=True)[82:98])
    if digest(selected) != FUNCTION_SHA256:
        raise ValueError("Reviewed calendar helper changed")
    context = {}
    for name, expected in CONTEXT_SHA256.items():
        data = (source_root / name).read_bytes()
        if digest(data) != expected:
            raise ValueError("Reviewed primitive/macro source context changed: " + name)
        context[name] = digest(data)
    prefix, suffix = reviewed_factors(selected, "healthy")
    split = len(FUNCTION_HEADER) + len(prefix)
    compiled = selected
    mutation = None
    if case != "healthy":
        before, after, _, _ = CONTROLS[case]
        if compiled.count(before.encode()) != 1:
            raise ValueError("Calendar control point is not unique")
        compiled = compiled.replace(before.encode(), after.encode(), 1)
        mutation = {"before": before, "after": after,
                    "scope": "complete compiled helper and its matching recut prefix/suffix"}
    compiled_prefix, compiled_suffix = reviewed_factors(compiled, case)
    appended = factor_wrappers(compiled_prefix, compiled_suffix)
    harness = harness_path.read_bytes()
    if not harness.strip():
        raise ValueError("Calendar-composition harness is empty")
    proof = (b'#![allow(dead_code)]\n#[path="calendar_selected.rs"]\nmod calendar;\n'
             + harness.replace(b"//!", b"//"))
    records = [
        {"source": SOURCE, "selection": "complete original private helper", "lines_inclusive": [83, 98], "sha256": digest(selected)},
        {"source": SOURCE, "selection": "exact original guard/day prefix", "sha256": digest(prefix), "function_byte_offsets_half_open": [len(FUNCTION_HEADER), split]},
        {"source": SOURCE, "selection": "exact original cycle-day suffix", "sha256": digest(suffix), "function_byte_offsets_half_open": [split, len(selected) - 2]},
    ]
    details = {
        "entrypoint": inputs / "proof.rs", "correspondence": records,
        "original_validation_sha256": digest(original), "harness_sha256": digest(harness),
        "original_function_sha256": digest(selected), "compiled_function_sha256": digest(compiled),
        "mutation": mutation, "assumptions": [],
        "structural_binding": {
            "status": "checked-reviewed-byte-family", "rule_version": 1,
            "kind": "trusted fixed-source structural composition; not general Rust equivalence",
            "machine_H_equals_factor_theorem": False,
            "source_context_sha256": context,
            "compiled_function_sha256": digest(compiled),
            "compiled_prefix_sha256": digest(compiled_prefix),
            "compiled_suffix_sha256": digest(compiled_suffix),
            "compiled_fragments_reconstruct_function": FUNCTION_HEADER + compiled_prefix + compiled_suffix + b"}\n" == compiled,
            "wrapper_templates_sha256": digest(PREFIX_HEAD + PREFIX_TAIL + SUFFIX_HEAD + SUFFIX_TAIL + ORIGINAL_WRAPPER),
            "appended_wrappers_sha256": digest(appended),
            "reviewed_semantics": {
                "P": "one bool early-return guard and two immutable primitive i128 locals; no callbacks/state/loops/labels/?/observable Drop",
                "S": "sole free value day_in_cycle:i128; immutable year/ordinal locals and tail builtin matches!",
                "bindings": "normal std prelude and primitive inherent methods in the pinned enclosing sources and generated module",
                "instrumentation": "only output=Some(actual day) then true after compiled P; caller starts None; S input binding remains i128",
            },
            "trusted_components": ["review of the four exact Rust byte strings and primitive control-flow rule",
                                   "frozen Python allowlist/extraction/wrapper builder and source custody",
                                   "existing admitted compiler/verifier/stdlib and host-runtime trusted base"],
            "mathematics": "no normalization, range, Gregorian or coverage property is supplied by this structural rule",
        },
        "composite_contract": {
            "required_healthy_theorems": list(REQUIRED_ASSERTIONS),
            "required_restored_theorems": list(REQUIRED_ASSERTIONS), "fixed_jobs": schedule(),
            "cycle_days": 146_097, "epoch_days_from_March_AD0": 693_901,
            "source_sha256": SOURCE_SHA256, "original_function_sha256": FUNCTION_SHA256,
            "acceptance": "all eleven jobs and checked reviewed structural binding with matching source/oracle/factor/context identities; no individual leaf closes full-u64 calendar acceptance",
            "composition": "full-u64 machine prefix theorem produces a valid cycle day; trusted fixed-source P/S correspondence binds it to actual H; forward coverage, generated suffix and Gregorian laws establish civil-month membership",
            "oracle_cycle_scope": "arbitrary u64 cycle counts retained, not only physically reachable cycles",
            "theorem_domains": {
                "calendar_source_full_domain": "every raw u64 second; fresh None observation slot; prefix arithmetic only, with separate trusted structural H binding",
                "calendar_representation_full_domain": "every raw u32 cycle day guarded by day < 146097",
                "calendar_suffix_full_domain": "all u16 years/ordinals guarded by year < 400 and ordinal < Gregorian segment length",
                "calendar_gregorian_laws": "all 400 years, twelve months and arbitrary u64 cycle counts",
            },
        },
        "omitted": ["Garde/ledger/scope admission", "UTC dataset truth and native clock semantics",
                    "a machine-checked universal H == factor theorem; replaced by explicit reviewed structural source binding"],
        "scope": "full raw-u64 calendar contract via four machine obligations plus explicit trusted fixed-source correspondence",
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    destinations = [inputs / name for name in ("calendar_selected.rs", "proof.rs", "correspondence.json")]
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Refusing to overwrite calendar-composition inputs")
    destinations[0].write_bytes(compiled + appended)
    destinations[1].write_bytes(proof)
    destinations[2].write_text(json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
