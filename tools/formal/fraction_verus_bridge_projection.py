"""Prepare an unproved predicate-correspondence candidate, without running tools.

The exact product closure is retained separately. Nothing here imports a callee
contract, changes a verifier profile, or supplies mathematical proof evidence.
"""
from __future__ import annotations

import ast
import hashlib
import json
from pathlib import Path
import re


ARITHMETIC_SHA256 = "9092cf40b90f60971b835bf573070fb2faa10e6cc9826f485cb6e6919da80046"
COMPOSITION_SHA256 = "fa2d61b0f428f572150303ce29b98f57bcf044769a5aba02b50d1b5c259c39b0"
PREFIX_SHA256 = "223b8b8526ec579bbf43352af899fc76f015432626862645e1e4f423d72add11"
# The independent ghost-specification owner supplies the complete reviewed bytes.
GHOST_SHA256 = "fa9dc1f45f9e6fda44abab22d3a8cc4a61db4910c014a934585b4312e107bfa5"
BIT_LENGTH_LEMMAS_SHA256 = "b2fddd6592ff98ccd8fc450821d71f462966c8ce70e4322044f2df9d8acafce0"
BIT_LENGTH_SLICE_SHA256 = "f8f90170153ec3bb7a49b0a3bdcd8bf770bb511d3f59429dadd427d2b67cc947"
BIT_LENGTH_MARKER = b"fn composition_constructor_post("
CONSTRUCTOR_POST_SLICE_SHA256 = "5b8758b1faaa516db3b849dc24d4822519dbbf0dfa55834c91605d8f4bb15217"
CONSTRUCTOR_POST_MARKER = b"// Total even on arbitrary raw inputs"
BIT_LENGTH_LEMMAS = "tools/formal/fraction_verus_bridge_lemmas.rs"
COMPLEMENT_LEMMAS = "tools/formal/fraction_verus_complement_lemmas.rs"
COMPLEMENT_LEMMAS_SHA256 = "55578940ea86be1ddf20693143c12afe7229e9cbfd053c2a06138ec2449abf85"
COMPLEMENT_SLICE_SHA256 = "f8a1e46efbbbf8b252d2dae75fb73493613f9fea5f745af85bc8a45616890d0d"
COMPLEMENT_SOURCE_SPANS = ((0, 1202), (2684, 3731))
RATIONAL_LEMMAS = "tools/formal/fraction_verus_rational_lemmas.rs"
RATIONAL_LEMMAS_SHA256 = "82d35d5e2c6289912c62cdb7a2d40a7f72e787cbee0c4604869a66c532a2880b"
RATIONAL_SOURCE_SPANS = ((0, 2333), (2684, 3731))
RATIONAL_SLICE_SHA256 = "b5e0a0dcdf924add0f9447bfce57555f39c5bc73deae656bec9993a576796a8d"
PREFIX_MARKER = b"// Six flattened alternatives:"
TYPE_NAMES = ("interval_error", "enclosure_error", "fraction")
SPAN_NAMES = ("fraction", "zero", "new", "compare", "complement", "fraction_sum",
              "addition", "enclosure_error", "interval_error")
DECLARATIONS = (
    (b"fn composition_constructor_admitted(numerator: u64, shift: u32) -> bool {",
     b"bridge_constructor_admitted(numerator, shift)"),
    (b"fn composition_checked(value: Fraction) -> bool {", b"bridge_checked(value)"),
    (b"fn composition_constructor_post(\n    numerator: u64,\n    shift: u32,\n"
     b"    result: &Result<Fraction, FixtureEnclosureError>,\n) -> bool {",
     b"bridge_constructor_post(numerator, shift, result)"),
    (b"fn composition_rational_order(left: Fraction, right: Fraction) -> Ordering {",
     b"bridge_rational_order(left, right)"),
    (b"fn composition_comparison_post(\n    left: Fraction,\n    right: Fraction,\n"
     b"    result: &Result<Ordering, FixtureEnclosureError>,\n) -> bool {",
     b"bridge_comparison_post(left, right, result)"),
    (b"fn composition_complement_post(\n    input: Fraction,\n"
     b"    result: &Result<Fraction, FixtureEnclosureError>,\n) -> bool {",
     b"bridge_complement_post(input, result)"),
)
BIT_LENGTH_HOOKS = (
    ("composition_constructor_admitted", b"    let bits = 64 - numerator.leading_zeros();\n",
     b"    proof {\n"
     b"        axiom_u64_leading_zeros(numerator);\n"
     b"        lemma_bridge_bit_length_power(numerator, 53);\n"
     b"        lemma_bridge_bit_length_power(numerator, shift);\n"
     b"    }\n"),
    ("composition_checked", b"    let bits = 64 - value.numerator.leading_zeros();\n",
     b"    proof {\n"
     b"        axiom_u64_leading_zeros(value.numerator);\n"
     b"        lemma_bridge_bit_length_power(value.numerator, 53);\n"
     b"        lemma_bridge_bit_length_power(value.numerator, value.shift);\n"
     b"    }\n"),
)
COMPLEMENT_HOOKS = (
    ("composition_complement_post",
     b"    let denominator = if 1 <= input.shift && input.shift <= 53 {\n",
     b"    proof {\n"
     b"        lemma_bridge_u128_shift_power(input.shift);\n"
     b"        lemma_bridge_complement_reject_outside(input.numerator, input.shift);\n"
     b"        lemma_bridge_u128_shift_power(52);\n"
     b"        assert((1_u128 << 52_u32) == 4_503_599_627_370_496_u128) by (bit_vector);\n"
     b"        assert(pow(2, 52) == 4_503_599_627_370_496);\n"
     b"        assert((input.numerator as u128) as int == input.numerator as int);\n"
     b"        assert(0 <= 2 * (input.numerator as int) <= 36_893_488_147_419_103_230);\n"
     b"        assert(36_893_488_147_419_103_230 < u128::MAX as int);\n"
     b"    }\n"),
    ("composition_complement_post",
     b"    match result {\n        Ok(value) => match denominator {\n",
     b"    proof {\n"
     b"        match denominator {\n"
     b"            Some(value) => {\n"
     b"                assert(value as int == bridge_denominator(input.shift));\n"
     b"                assert((2 * (input.numerator as u128)) as int\n"
     b"                    == 2 * (input.numerator as int));\n"
     b"            }\n"
     b"            None => {\n"
     b"                assert(2 * (input.numerator as int) < bridge_denominator(input.shift));\n"
     b"            }\n"
     b"        }\n"
     b"        assert(required_success\n"
     b"            == (2 * (input.numerator as int) >= bridge_denominator(input.shift)));\n"
     b"    }\n"),
    ("composition_complement_post",
     b"            Some(denominator) => {\n",
     b"                proof {\n"
     b"                    assert((value.numerator as u128) as int == value.numerator as int);\n"
     b"                    assert(0 <= (value.numerator as int) + (input.numerator as int)\n"
     b"                        <= 36_893_488_147_419_103_230);\n"
     b"                    assert(36_893_488_147_419_103_230 < u128::MAX as int);\n"
     b"                    assert(((value.numerator as u128) + (input.numerator as u128)) as int\n"
     b"                        == (value.numerator as int) + (input.numerator as int));\n"
     b"                    assert(denominator as int == bridge_denominator(input.shift));\n"
     b"                }\n"),
)
RATIONAL_HOOKS = (
    ("composition_rational_order",
     b"    if !composition_checked(left) || !composition_checked(right) {\n",
     b"    proof {\n"
     b"        reveal(bridge_rational_order);\n"
     b"        reveal(bridge_checked);\n"
     b"        lemma_bridge_denominator_factor(left.shift, right.shift);\n"
     b"        lemma_bridge_denominator_factor(right.shift, left.shift);\n"
     b"        lemma_bridge_positive_factor_order(0, left.numerator as int,\n"
     b"            bridge_denominator(right.shift));\n"
     b"        lemma_bridge_positive_factor_order(0, right.numerator as int,\n"
     b"            bridge_denominator(left.shift));\n"
     b"    }\n"),
    ("composition_rational_order",
     b"        return left.numerator.cmp(&right.numerator);\n",
     b"        proof {\n"
     b"            lemma_bridge_positive_factor_order(left.numerator as int,\n"
     b"                right.numerator as int, bridge_denominator(left.shift));\n"
     b"        }\n"),
    ("composition_rational_order",
     b"            (u128::from(left.numerator) << gap).cmp(&u128::from(right.numerator))\n",
     b"            proof {\n"
     b"                assert(gap as int == right.shift as int - left.shift as int);\n"
     b"                lemma_bridge_u128_shifted_u64(left.numerator, gap);\n"
     b"                assert((left.numerator as u128) as int == left.numerator as int);\n"
     b"                assert((right.numerator as u128) as int == right.numerator as int);\n"
     b"                assert(bridge_denominator(right.shift) == bridge_denominator(left.shift)\n"
     b"                    * pow(2, gap as nat));\n"
     b"                vstd::arithmetic::mul::lemma_mul_is_commutative(\n"
     b"                    bridge_denominator(left.shift), pow(2, gap as nat));\n"
     b"                vstd::arithmetic::mul::lemma_mul_is_associative(left.numerator as int,\n"
     b"                    pow(2, gap as nat), bridge_denominator(left.shift));\n"
     b"                lemma_bridge_positive_factor_order((left.numerator as int) * pow(2, gap as nat),\n"
     b"                    right.numerator as int, bridge_denominator(left.shift));\n"
     b"            }\n"),
    ("composition_rational_order",
     b"            Ordering::Greater\n        }\n",
     b"            proof {\n"
     b"                assert(gap as int == right.shift as int - left.shift as int);\n"
     b"                lemma_bridge_rational_gap_dominates(left.numerator, right.numerator, gap);\n"
     b"                assert(bridge_denominator(right.shift) == bridge_denominator(left.shift)\n"
     b"                    * pow(2, gap as nat));\n"
     b"                vstd::arithmetic::mul::lemma_mul_is_commutative(\n"
     b"                    bridge_denominator(left.shift), pow(2, gap as nat));\n"
     b"                vstd::arithmetic::mul::lemma_mul_is_associative(left.numerator as int,\n"
     b"                    pow(2, gap as nat), bridge_denominator(left.shift));\n"
     b"                lemma_bridge_positive_factor_order((left.numerator as int) * pow(2, gap as nat),\n"
     b"                    right.numerator as int, bridge_denominator(left.shift));\n"
     b"            }\n"),
    ("composition_rational_order",
     b"            u128::from(left.numerator).cmp(&(u128::from(right.numerator) << gap))\n",
     b"            proof {\n"
     b"                assert(gap as int == left.shift as int - right.shift as int);\n"
     b"                lemma_bridge_u128_shifted_u64(right.numerator, gap);\n"
     b"                assert((left.numerator as u128) as int == left.numerator as int);\n"
     b"                assert((right.numerator as u128) as int == right.numerator as int);\n"
     b"                assert(bridge_denominator(left.shift) == bridge_denominator(right.shift)\n"
     b"                    * pow(2, gap as nat));\n"
     b"                vstd::arithmetic::mul::lemma_mul_is_commutative(\n"
     b"                    bridge_denominator(right.shift), pow(2, gap as nat));\n"
     b"                vstd::arithmetic::mul::lemma_mul_is_associative(right.numerator as int,\n"
     b"                    pow(2, gap as nat), bridge_denominator(right.shift));\n"
     b"                lemma_bridge_positive_factor_order(left.numerator as int,\n"
     b"                    (right.numerator as int) * pow(2, gap as nat), bridge_denominator(right.shift));\n"
     b"            }\n"),
    ("composition_rational_order",
     b"            Ordering::Less\n        }\n    }\n",
     b"            proof {\n"
     b"                assert(gap as int == left.shift as int - right.shift as int);\n"
     b"                lemma_bridge_rational_gap_dominates(right.numerator, left.numerator, gap);\n"
     b"                assert(bridge_denominator(left.shift) == bridge_denominator(right.shift)\n"
     b"                    * pow(2, gap as nat));\n"
     b"                vstd::arithmetic::mul::lemma_mul_is_commutative(\n"
     b"                    bridge_denominator(right.shift), pow(2, gap as nat));\n"
     b"                vstd::arithmetic::mul::lemma_mul_is_associative(right.numerator as int,\n"
     b"                    pow(2, gap as nat), bridge_denominator(right.shift));\n"
     b"                lemma_bridge_positive_factor_order(left.numerator as int,\n"
     b"                    (right.numerator as int) * pow(2, gap as nat), bridge_denominator(right.shift));\n"
     b"            }\n"),
)


def digest(data: bytes) -> str:
    """Identify bytes without decoding or normalization."""
    return hashlib.sha256(data).hexdigest()


def input_names() -> tuple[str, ...]:
    """The complete finite source-only preparation input inventory."""
    return ("crates/time-core/src/projection_enclosure.rs", "crates/time-core/src/selection.rs",
            "tools/formal/fraction_arithmetic_projection.py", "tools/formal/fraction_composition_spec.rs",
            "tools/formal/fraction_verus_bridge_spec.rs", "tools/formal/fraction_verus_bridge_projection.py",
            "tools/formal/test_fraction_verus_bridge_projection.py")


def dependency_input_names() -> tuple[str, ...]:
    """The separate bit-length path adds exactly the pinned ghost lemma source."""
    return (*input_names(), BIT_LENGTH_LEMMAS)


def complement_input_names() -> tuple[str, ...]:
    """The closed complement path adds one separately pinned ghost source."""
    return (*dependency_input_names(), COMPLEMENT_LEMMAS)


def rational_input_names() -> tuple[str, ...]:
    """The additive rational path preserves all earlier source dependencies."""
    return (*complement_input_names(), RATIONAL_LEMMAS)


def _plain_path(path: Path) -> Path:
    """Refuse symlinks in every existing component before reading or writing."""
    absolute = path.absolute()
    if any(item.is_symlink() for item in (absolute, *absolute.parents)):
        raise ValueError("Bridge source/output path redirects")
    if absolute != absolute.resolve(strict=False):
        raise ValueError("Bridge source/output path is not canonical")
    return absolute


def _annotation(declaration: bytes, relation: bytes) -> bytes:
    return (b"#[verifier::allow_in_spec]\n" + declaration[:-2]
            + b"\n    returns " + relation + b",\n{")


def annotated(prefix: bytes) -> bytes:
    """Insert only the six reviewed attributes and exact returns clauses."""
    if digest(prefix) != PREFIX_SHA256:
        raise ValueError("Exact six-predicate prefix changed")
    result = prefix
    for declaration, relation in DECLARATIONS:
        if result.count(declaration) != 1:
            raise ValueError("Predicate declaration is not unique")
        result = result.replace(declaration, _annotation(declaration, relation), 1)
    if erase_annotations(result) != prefix:
        raise ValueError("Predicate annotations do not erase exactly")
    return result


def erase_annotations(candidate: bytes) -> bytes:
    """Reject any body/signature edit, even when annotations erase cleanly."""
    result = candidate
    for declaration, relation in DECLARATIONS:
        inserted = _annotation(declaration, relation)
        if result.count(inserted) != 1:
            raise ValueError("Exact bridge annotation missing or altered")
        result = result.replace(inserted, declaration, 1)
    if digest(result) != PREFIX_SHA256:
        raise ValueError("Erased executable predicate bytes changed")
    return result


def annotated_bit_length(fragment: bytes) -> bytes:
    """Insert two closed pre-subtraction proof blocks and two exact annotations."""
    if digest(fragment) != BIT_LENGTH_SLICE_SHA256:
        raise ValueError("Exact two-predicate slice changed")
    result = fragment
    for declaration, relation in DECLARATIONS[:2]:
        if result.count(declaration) != 1:
            raise ValueError("Bit-length predicate declaration is not unique")
        result = result.replace(declaration, _annotation(declaration, relation), 1)
    for _, anchor, block in BIT_LENGTH_HOOKS:
        if result.count(anchor) != 1:
            raise ValueError("Bit-length proof insertion anchor is not unique")
        result = result.replace(anchor, block + anchor, 1)
    if erase_bit_length_annotations(result) != fragment:
        raise ValueError("Bit-length insertions do not erase exactly")
    return result


def erase_bit_length_annotations(candidate: bytes) -> bytes:
    """Erase only exact approved hooks; reject every other executable byte edit."""
    result = candidate
    for _, anchor, block in BIT_LENGTH_HOOKS:
        if result.count(block + anchor) != 1:
            raise ValueError("Exact bit-length proof hook missing or altered")
        result = result.replace(block + anchor, anchor, 1)
    for declaration, relation in DECLARATIONS[:2]:
        inserted = _annotation(declaration, relation)
        if result.count(inserted) != 1:
            raise ValueError("Exact bit-length annotation missing or altered")
        result = result.replace(inserted, declaration, 1)
    if digest(result) != BIT_LENGTH_SLICE_SHA256:
        raise ValueError("Erased two-predicate executable bytes changed")
    return result


def annotated_constructor_post(fragment: bytes) -> bytes:
    """Annotate three exact predicates with only the two existing proof hooks."""
    if digest(fragment) != CONSTRUCTOR_POST_SLICE_SHA256:
        raise ValueError("Exact three-predicate slice changed")
    result = fragment
    for declaration, relation in DECLARATIONS[:3]:
        if result.count(declaration) != 1:
            raise ValueError("Constructor-Post predicate declaration is not unique")
        result = result.replace(declaration, _annotation(declaration, relation), 1)
    for _, anchor, block in BIT_LENGTH_HOOKS:
        if result.count(anchor) != 1:
            raise ValueError("Constructor-Post proof insertion anchor is not unique")
        result = result.replace(anchor, block + anchor, 1)
    if erase_constructor_post_annotations(result) != fragment:
        raise ValueError("Constructor-Post insertions do not erase exactly")
    return result


def erase_constructor_post_annotations(candidate: bytes) -> bytes:
    """Erase three exact annotations and old hooks; require original raw bytes."""
    result = candidate
    for _, anchor, block in BIT_LENGTH_HOOKS:
        if result.count(block + anchor) != 1:
            raise ValueError("Exact constructor-Post proof hook missing or altered")
        result = result.replace(block + anchor, anchor, 1)
    for declaration, relation in DECLARATIONS[:3]:
        inserted = _annotation(declaration, relation)
        if result.count(inserted) != 1:
            raise ValueError("Exact constructor-Post annotation missing or altered")
        result = result.replace(inserted, declaration, 1)
    if digest(result) != CONSTRUCTOR_POST_SLICE_SHA256:
        raise ValueError("Erased three-predicate executable bytes changed")
    return result


def annotated_complement_post(fragment: bytes) -> bytes:
    """Annotate two exact disjoint spans with only the five approved ghost hooks."""
    if digest(fragment) != COMPLEMENT_SLICE_SHA256:
        raise ValueError("Exact four-predicate complement slice changed")
    first = annotated_constructor_post(fragment[:1202])
    complement = fragment[1202:]
    declaration, relation = DECLARATIONS[5]
    if complement.count(declaration) != 1:
        raise ValueError("Complement-Post predicate declaration is not unique")
    complement = complement.replace(declaration, _annotation(declaration, relation), 1)
    for _, anchor, block in COMPLEMENT_HOOKS:
        if complement.count(anchor) != 1:
            raise ValueError("Complement-Post proof insertion anchor is not unique")
        if anchor == b"            Some(denominator) => {\n":
            complement = complement.replace(anchor, anchor + block, 1)
        else:
            complement = complement.replace(anchor, block + anchor, 1)
    result = first + complement
    if erase_complement_post_annotations(result) != fragment:
        raise ValueError("Complement-Post insertions do not erase exactly")
    return result


def erase_complement_post_annotations(candidate: bytes) -> bytes:
    """Erase exact scoped ghost insertions and independently recover both spans."""
    declaration, relation = DECLARATIONS[5]
    inserted = _annotation(declaration, relation)
    if candidate.count(inserted) != 1:
        raise ValueError("Exact complement-Post annotation missing or altered")
    first, complement = candidate.split(inserted, 1)
    first = erase_constructor_post_annotations(first)
    complement = inserted + complement
    for _, anchor, block in COMPLEMENT_HOOKS:
        after = anchor == b"            Some(denominator) => {\n"
        approved = anchor + block if after else block + anchor
        if complement.count(approved) != 1:
            raise ValueError("Exact complement-Post proof hook missing or altered")
        complement = complement.replace(approved, anchor, 1)
    complement = complement.replace(inserted, declaration, 1)
    if digest(complement) != "a9e8e582cc3b7630f37df1b7e446bb18c12b4192bd7a2dbb47dd423225b683fa":
        raise ValueError("Erased complement executable span changed")
    result = first + complement
    if digest(result) != COMPLEMENT_SLICE_SHA256:
        raise ValueError("Erased four-predicate complement executable bytes changed")
    return result


def annotated_rational_order(fragment: bytes) -> bytes:
    """Add one predicate and six exact hooks while preserving old annotations."""
    if digest(fragment) != RATIONAL_SLICE_SHA256:
        raise ValueError("Exact five-predicate rational slice changed")
    old = annotated_complement_post(fragment[:1202] + fragment[2333:])
    complement_annotation = _annotation(*DECLARATIONS[5])
    first, complement = old.split(complement_annotation, 1)
    rational = fragment[1202:2333]
    declaration, relation = DECLARATIONS[3]
    if rational.count(declaration) != 1:
        raise ValueError("Rational-order predicate declaration is not unique")
    rational = rational.replace(declaration, _annotation(declaration, relation), 1)
    for _, anchor, block in RATIONAL_HOOKS:
        if rational.count(anchor) != 1:
            raise ValueError("Rational-order proof insertion anchor is not unique")
        rational = rational.replace(anchor, block + anchor, 1)
    result = first + rational + complement_annotation + complement
    if erase_rational_order_annotations(result) != fragment:
        raise ValueError("Rational-order insertions do not erase exactly")
    return result


def erase_rational_order_annotations(candidate: bytes) -> bytes:
    """Erase only approved annotations/hooks and recover both exact raw spans."""
    rational_annotation = _annotation(*DECLARATIONS[3])
    complement_annotation = _annotation(*DECLARATIONS[5])
    if candidate.count(rational_annotation) != 1 or candidate.count(complement_annotation) != 1:
        raise ValueError("Exact rational-order annotation missing or altered")
    first, tail = candidate.split(rational_annotation, 1)
    rational, complement = tail.split(complement_annotation, 1)
    if first.count(CONSTRUCTOR_POST_MARKER) != 1:
        raise ValueError("Exact rational-order leading comments missing or altered")
    first, leading_comments = first.split(CONSTRUCTOR_POST_MARKER, 1)
    rational = CONSTRUCTOR_POST_MARKER + leading_comments + rational_annotation + rational
    for _, anchor, block in RATIONAL_HOOKS:
        if rational.count(block + anchor) != 1:
            raise ValueError("Exact rational-order proof hook missing or altered")
        rational = rational.replace(block + anchor, anchor, 1)
    rational = rational.replace(rational_annotation, DECLARATIONS[3][0], 1)
    if digest(rational) != "bb726637b89b2f6d2fce1c76538894a04ac728e8915ad663b2f71399fb8cd7de":
        raise ValueError("Erased rational-order executable span changed")
    old = erase_complement_post_annotations(first + complement_annotation + complement)
    result = old[:1202] + rational + old[1202:]
    if digest(result[:2333]) != "c1eb218abfc039caf065c00391d1f9832b668229397f6ff71c3385e89318352d":
        raise ValueError("Erased rational first executable span changed")
    if digest(result) != RATIONAL_SLICE_SHA256:
        raise ValueError("Erased five-predicate rational executable bytes changed")
    return result


def _source_support(data: bytes) -> tuple[dict, dict]:
    """Read just two literal constants; never import the old preparation module."""
    if digest(data) != ARITHMETIC_SHA256:
        raise ValueError("Pinned arithmetic projection changed")
    constants = {}
    for node in ast.parse(data).body:
        if isinstance(node, ast.Assign):
            for target in node.targets:
                if isinstance(target, ast.Name) and target.id in {"SOURCE_SHA256", "SPANS"}:
                    if target.id in constants:
                        raise ValueError("Arithmetic source constants are ambiguous")
                    constants[target.id] = ast.literal_eval(node.value)
    if set(constants) != {"SOURCE_SHA256", "SPANS"}:
        raise ValueError("Arithmetic source constants missing")
    sources, spans = constants["SOURCE_SHA256"], constants["SPANS"]
    if set(sources) != {"projection_enclosure.rs", "selection.rs"} or tuple(spans) != SPAN_NAMES:
        raise ValueError("Arithmetic source inventory changed")
    return sources, spans


def _reviewed_payload(source_root: Path, inputs: Path, names: tuple[str, ...]) -> tuple:
    """Build the existing payload in memory, retaining its original validation."""
    source_root, inputs = _plain_path(source_root), _plain_path(inputs)
    if not source_root.is_dir() or inputs.exists() or not inputs.parent.is_dir():
        raise ValueError("Bridge preparation requires source and a fresh output leaf")
    if inputs.is_relative_to(source_root) or source_root.is_relative_to(inputs):
        raise ValueError("Bridge output must be outside its source tree")
    sources = {}
    for name in names:
        path = _plain_path(source_root / name)
        if not path.is_file():
            raise ValueError("Bridge source input is not a regular file: " + name)
        sources[name] = path.read_bytes()
    if sources["tools/formal/fraction_verus_bridge_projection.py"] != Path(__file__).read_bytes():
        raise ValueError("Prepared projection differs from this executing source")
    source_hashes, spans = _source_support(sources["tools/formal/fraction_arithmetic_projection.py"])
    originals = {name: sources["crates/time-core/src/" + name] for name in source_hashes}
    for name, expected in source_hashes.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete product source changed: " + name)
    selected = {}
    for name, span in spans.items():
        original = originals[span["source"]]
        first, last = span["offsets"]
        selected[name] = original[first:last]
        if (original.count(span["start"]) != 1 or original.count(span["following"]) != 1
                or original.index(span["start"]) != first or original.index(span["following"]) < last
                or digest(selected[name]) != span["sha256"]):
            raise ValueError("Reviewed product span changed: " + name)
    composition = sources["tools/formal/fraction_composition_spec.rs"]
    if digest(composition) != COMPOSITION_SHA256 or composition.count(PREFIX_MARKER) != 1:
        raise ValueError("Pinned composition specification changed")
    prefix = composition.split(PREFIX_MARKER, 1)[0]
    predicates = annotated(prefix)
    ghost = sources["tools/formal/fraction_verus_bridge_spec.rs"]
    if digest(ghost) != GHOST_SHA256:
        raise ValueError("Pinned independent ghost specification changed")
    # The two hash-pinned sources use only line comments and contain no strings
    # with comment delimiters. This closed-source token check is no Rust parser
    # and establishes nothing about vstd or its trusted base.
    forbidden = rb"\b(?:assume|admit|external_body|external_fn_specification|assume_specification|axiom|requires)\b"
    local_code = re.sub(rb"(?m)//[^\n]*", b"", ghost + predicates)
    if re.search(forbidden, local_code):
        raise ValueError("Bridge candidate contains a forbidden local assumption or precondition")
    bridge = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
              b"// UNPROVED source-only candidate; no caller contract is imported.\n"
              b"use std::cmp::Ordering;\nuse vstd::prelude::*;\n\nverus! {\n\n"
              + b"\n".join(selected[name] for name in TYPE_NAMES) + b"\n\n"
              + ghost + b"\n" + predicates + b"\n}\n")
    actual = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
              b"// Correspondence input only; no caller-import entrypoint or permission.\n"
              b"use std::cmp::Ordering;\n\n"
              + b"\n".join(selected[name] for name in TYPE_NAMES) + b"\nimpl Fraction {\n"
              + b"\n".join(selected[name] for name in ("zero", "new", "compare", "complement"))
              + b"\n}\n\n" + selected["fraction_sum"] + b"\n" + selected["addition"])
    details = {
        "schema_version": 1, "status": "unproved_source_candidate", "proof_commands": 0,
        "scope": "six unrestricted executable predicate/specification equalities; source preparation only",
        "remaining": ["exact Verus compatibility", "six completed full-domain equality proofs",
                      "mathematical rational/power/shift lemmas", "termination and effect correspondence",
                      "wrong-contract controls", "separate caller-import profile admission",
                      "original universal/safety/witness/defect/restoration union"],
        "source_files_sha256": {name: digest(data) for name, data in sources.items()},
        "original_sources_sha256": source_hashes, "prefix_sha256": digest(prefix),
        "annotated_prefix_sha256": digest(predicates), "ghost_sha256": digest(ghost),
        "correspondence": [{"item": name, "source": "crates/time-core/src/" + span["source"],
                            "byte_offsets_half_open": list(span["offsets"]), "sha256": digest(selected[name]),
                            "bridge_type": name in TYPE_NAMES}
                           for name, span in spans.items()],
        "predicate_relations": [relation.decode() for _, relation in DECLARATIONS],
        "generated_sha256": {"bridge.rs": digest(bridge), "actual_source.rs": digest(actual)},
    }
    payload = {"bridge.rs": bridge, "actual_source.rs": actual,
               "correspondence.json": (json.dumps(details, indent=2, sort_keys=True) + "\n").encode()}
    return inputs, payload, details, selected, prefix, ghost, sources


def _write_payload(inputs: Path, payload: dict[str, bytes]) -> None:
    """Write one already validated candidate; never adopt or overwrite a leaf."""
    inputs.mkdir(mode=0o700)
    for name, content in payload.items():
        with (inputs / name).open("xb") as stream:
            stream.write(content)


def prepare(source_root: Path, inputs: Path) -> dict:
    """Prepare the original seven-input, six-predicate source-only candidate.

    Validation errors create no output directory. An existing output is never
    adopted or overwritten. No compiler, solver, runner or admission is invoked.
    """
    inputs, payload, details, *_ = _reviewed_payload(source_root, inputs, input_names())
    _write_payload(inputs, payload)
    return details


def prepare_bit_length(source_root: Path, inputs: Path) -> dict:
    """Prepare only the full-raw bit-length dependency unit, without execution.

    Keep the whole six-predicate prefix and nine product spans as correspondence
    inputs. This path changes neither the old candidate nor its acceptance scope.
    """
    inputs, old_payload, original, selected, prefix, ghost, sources = _reviewed_payload(
        source_root, inputs, dependency_input_names())
    lemmas = sources[BIT_LENGTH_LEMMAS]
    if digest(lemmas) != BIT_LENGTH_LEMMAS_SHA256:
        raise ValueError("Pinned bit-length ghost lemma changed")
    if prefix.count(BIT_LENGTH_MARKER) != 1:
        raise ValueError("Two-predicate boundary is not unique")
    fragment = prefix.split(BIT_LENGTH_MARKER, 1)[0]
    predicates = annotated_bit_length(fragment)
    bridge = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
              b"// UNPROVED bit-length dependency source; full bridge acceptance remains open.\n"
              b"use std::cmp::Ordering;\nuse vstd::prelude::*;\n\nverus! {\n\n"
              + b"\n".join(selected[name] for name in TYPE_NAMES) + b"\n\n"
              + ghost + b"\n" + lemmas + b"\n" + predicates + b"\n}\n")
    modes = {"proof::lemma_bridge_bit_length_power": "proof",
             "proof::composition_constructor_admitted": "exec",
             "proof::composition_checked": "exec",
             "proof::IntervalError::clone": "exec",
             "proof::FixtureEnclosureError::clone": "exec",
             "proof::Fraction::clone": "exec"}
    hooks = [{"function": "proof::" + name,
              "source_byte_offset": fragment.index(anchor),
              "insertion": "before unchanged leading_zeros subtraction",
              "anchor_sha256": digest(anchor), "proof_block_sha256": digest(block)}
             for name, anchor, block in BIT_LENGTH_HOOKS]
    generated = {"proof.rs": bridge, "actual_source.rs": old_payload["actual_source.rs"],
                 "predicate_source.rs": prefix}
    details = {
        "schema_version": 1, "status": "unproved_source_candidate", "proof_commands": 0,
        "scope": "bit-length lemma and two unrestricted predicate equalities; source preparation only",
        "source_files_sha256": original["source_files_sha256"],
        "original_sources_sha256": original["original_sources_sha256"],
        "correspondence": original["correspondence"],
        "prefix_sha256": digest(prefix), "prefix_bytes": len(prefix),
        "fragment_source_byte_offsets_half_open": [0, len(fragment)],
        "fragment_sha256": digest(fragment), "annotated_fragment_sha256": digest(predicates),
        "ghost_sha256": digest(ghost), "lemma_sha256": digest(lemmas),
        "predicate_relations": [relation.decode() for _, relation in DECLARATIONS[:2]],
        "full_bridge_predicate_relations": original["predicate_relations"],
        "expected_function_modes": modes, "proof_hooks": hooks,
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
        "generated_sha256": {name: digest(data) for name, data in generated.items()},
    }
    generated["correspondence.json"] = (json.dumps(details, indent=2, sort_keys=True) + "\n").encode()
    _write_payload(inputs, generated)
    return details


def prepare_constructor_post(source_root: Path, inputs: Path) -> dict:
    """Prepare the closed raw constructor-Post dependency unit, without execution.

    The original complete product closure and six-predicate prefix remain inputs.
    No constructor-produced-result or checked-Fraction premise is introduced.
    """
    inputs, old_payload, original, selected, prefix, ghost, sources = _reviewed_payload(
        source_root, inputs, dependency_input_names())
    lemmas = sources[BIT_LENGTH_LEMMAS]
    if digest(lemmas) != BIT_LENGTH_LEMMAS_SHA256:
        raise ValueError("Pinned bit-length ghost lemma changed")
    if prefix.count(CONSTRUCTOR_POST_MARKER) != 1:
        raise ValueError("Three-predicate boundary is not unique")
    fragment = prefix.split(CONSTRUCTOR_POST_MARKER, 1)[0]
    predicates = annotated_constructor_post(fragment)
    bridge = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
              b"// UNPROVED constructor-Post dependency source; full bridge acceptance remains open.\n"
              b"use std::cmp::Ordering;\nuse vstd::prelude::*;\n\nverus! {\n\n"
              + b"\n".join(selected[name] for name in TYPE_NAMES) + b"\n\n"
              + ghost + b"\n" + lemmas + b"\n" + predicates + b"\n}\n")
    modes = {"proof::lemma_bridge_bit_length_power": "proof",
             "proof::composition_constructor_admitted": "exec",
             "proof::composition_checked": "exec",
             "proof::composition_constructor_post": "exec",
             "proof::IntervalError::clone": "exec",
             "proof::FixtureEnclosureError::clone": "exec",
             "proof::Fraction::clone": "exec"}
    hooks = [{"function": "proof::" + name,
              "source_byte_offset": fragment.index(anchor),
              "insertion": "before unchanged leading_zeros subtraction",
              "anchor_sha256": digest(anchor), "proof_block_sha256": digest(block)}
             for name, anchor, block in BIT_LENGTH_HOOKS]
    generated = {"proof.rs": bridge, "actual_source.rs": old_payload["actual_source.rs"],
                 "predicate_source.rs": prefix}
    details = {
        "schema_version": 1, "status": "unproved_source_candidate", "proof_commands": 0,
        "scope": "bit-length lemma and three unrestricted predicate equalities; source preparation only",
        "source_files_sha256": original["source_files_sha256"],
        "original_sources_sha256": original["original_sources_sha256"],
        "correspondence": original["correspondence"],
        "prefix_sha256": digest(prefix), "prefix_bytes": len(prefix),
        "fragment_source_byte_offsets_half_open": [0, len(fragment)],
        "fragment_sha256": digest(fragment), "annotated_fragment_sha256": digest(predicates),
        "ghost_sha256": digest(ghost), "lemma_sha256": digest(lemmas),
        "predicate_relations": [relation.decode() for _, relation in DECLARATIONS[:3]],
        "full_bridge_predicate_relations": original["predicate_relations"],
        "expected_function_modes": modes, "proof_hooks": hooks,
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
        "generated_sha256": {name: digest(data) for name, data in generated.items()},
    }
    generated["correspondence.json"] = (json.dumps(details, indent=2, sort_keys=True) + "\n").encode()
    _write_payload(inputs, generated)
    return details


def prepare_complement_post(source_root: Path, inputs: Path) -> dict:
    """Prepare the exact four-predicate complement dependency unit, without tools.

    Both independently hashed source spans erase exactly. Raw input/result fields
    remain independent; every existing preparation API retains its output Rust.
    """
    inputs, old_payload, original, selected, prefix, ghost, sources = _reviewed_payload(
        source_root, inputs, complement_input_names())
    lemmas, complement_lemmas = sources[BIT_LENGTH_LEMMAS], sources[COMPLEMENT_LEMMAS]
    if digest(lemmas) != BIT_LENGTH_LEMMAS_SHA256:
        raise ValueError("Pinned bit-length ghost lemma changed")
    if digest(complement_lemmas) != COMPLEMENT_LEMMAS_SHA256:
        raise ValueError("Pinned complement ghost lemmas changed")
    span_hashes = (CONSTRUCTOR_POST_SLICE_SHA256,
                   "a9e8e582cc3b7630f37df1b7e446bb18c12b4192bd7a2dbb47dd423225b683fa")
    fragments = [prefix[first:last] for first, last in COMPLEMENT_SOURCE_SPANS]
    if tuple(digest(part) for part in fragments) != span_hashes:
        raise ValueError("Exact complement source spans changed")
    fragment = b"".join(fragments)
    predicates = annotated_complement_post(fragment)
    local_code = re.sub(rb"(?m)//[^\n]*", b"", complement_lemmas + predicates)
    if re.search(rb"\b(?:assume|admit|external_body|external_fn_specification|assume_specification|axiom|requires)\b",
                 local_code):
        raise ValueError("Complement candidate contains a forbidden local assumption or precondition")
    bridge = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
              b"// UNPROVED complement-Post dependency source; full bridge acceptance remains open.\n"
              b"use std::cmp::Ordering;\nuse vstd::prelude::*;\n\nverus! {\n\n"
              + b"\n".join(selected[name] for name in TYPE_NAMES) + b"\n\n"
              + ghost + b"\n" + lemmas + b"\n" + complement_lemmas + b"\n" + predicates + b"\n}\n")
    modes = {"proof::lemma_bridge_bit_length_power": "proof",
             "proof::lemma_bridge_u128_shift_power": "proof",
             "proof::lemma_bridge_complement_reject_outside": "proof",
             "proof::composition_constructor_admitted": "exec",
             "proof::composition_checked": "exec",
             "proof::composition_constructor_post": "exec",
             "proof::composition_complement_post": "exec",
             "proof::IntervalError::clone": "exec",
             "proof::FixtureEnclosureError::clone": "exec",
             "proof::Fraction::clone": "exec"}
    hooks = [{"function": "proof::" + name,
              "source_byte_offset": prefix.index(anchor),
              "insertion": ("before unchanged leading_zeros subtraction" if index < 2
                            else "at approved complement proof site"),
              "anchor_sha256": digest(anchor), "proof_block_sha256": digest(block)}
             for index, (name, anchor, block) in enumerate((*BIT_LENGTH_HOOKS, *COMPLEMENT_HOOKS))]
    generated = {"proof.rs": bridge, "actual_source.rs": old_payload["actual_source.rs"],
                 "predicate_source.rs": prefix}
    details = {
        "schema_version": 1, "status": "unproved_source_candidate", "proof_commands": 0,
        "scope": "bit-length and complement lemmas and four unrestricted predicate equalities; source preparation only",
        "source_files_sha256": original["source_files_sha256"],
        "original_sources_sha256": original["original_sources_sha256"],
        "correspondence": original["correspondence"],
        "prefix_sha256": digest(prefix), "prefix_bytes": len(prefix),
        "fragment_source_spans": [
            {"source_byte_offsets_half_open": list(span), "sha256": expected}
            for span, expected in zip(COMPLEMENT_SOURCE_SPANS, span_hashes)],
        "fragment_sha256": digest(fragment), "annotated_fragment_sha256": digest(predicates),
        "ghost_sha256": digest(ghost), "lemma_sha256": digest(lemmas),
        "complement_lemma_sha256": digest(complement_lemmas),
        "predicate_relations": [DECLARATIONS[index][1].decode() for index in (0, 1, 2, 5)],
        "full_bridge_predicate_relations": original["predicate_relations"],
        "expected_function_modes": modes, "proof_hooks": hooks,
        "raw_domains": {
            "lemma": "every n:u64 and s:u32; no precondition",
            "shift_power": "every s:u32; no precondition; guarded shift/power implication for 1 <= s <= 53",
            "reject_outside": "every n:u64 and s:u32; no precondition; guarded checked outside-window implication",
            "constructor_admitted": "every numerator:u64 and shift:u32; no precondition",
            "checked": "every raw Fraction { numerator:u64, shift:u32 }; no precondition",
            "constructor_post": "every numerator:u64, shift:u32 and independently chosen &Result<Fraction, FixtureEnclosureError>; raw Ok fields and every error variant; no precondition",
            "complement_post": "every raw Fraction { numerator:u64, shift:u32 } and independently chosen &Result<Fraction, FixtureEnclosureError>; raw Ok fields and every error variant; no precondition",
        },
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
        "generated_sha256": {name: digest(data) for name, data in generated.items()},
    }
    generated["correspondence.json"] = (json.dumps(details, indent=2, sort_keys=True) + "\n").encode()
    _write_payload(inputs, generated)
    return details


def prepare_rational_order(source_root: Path, inputs: Path) -> dict:
    """Prepare seven total lemmas and five full-raw predicate equalities only."""
    inputs, old_payload, original, selected, prefix, ghost, sources = _reviewed_payload(
        source_root, inputs, rational_input_names())
    lemmas = sources[BIT_LENGTH_LEMMAS]
    complement_lemmas = sources[COMPLEMENT_LEMMAS]
    rational_lemmas = sources[RATIONAL_LEMMAS]
    if digest(lemmas) != BIT_LENGTH_LEMMAS_SHA256:
        raise ValueError("Pinned bit-length ghost lemma changed")
    if digest(complement_lemmas) != COMPLEMENT_LEMMAS_SHA256:
        raise ValueError("Pinned complement ghost lemmas changed")
    if digest(rational_lemmas) != RATIONAL_LEMMAS_SHA256:
        raise ValueError("Pinned rational ghost lemmas changed")
    span_hashes = ("c1eb218abfc039caf065c00391d1f9832b668229397f6ff71c3385e89318352d",
                   "a9e8e582cc3b7630f37df1b7e446bb18c12b4192bd7a2dbb47dd423225b683fa")
    fragments = [prefix[first:last] for first, last in RATIONAL_SOURCE_SPANS]
    if tuple(digest(part) for part in fragments) != span_hashes:
        raise ValueError("Exact rational source spans changed")
    fragment = b"".join(fragments)
    predicates = annotated_rational_order(fragment)
    local_code = re.sub(rb"(?m)//[^\n]*", b"", rational_lemmas + complement_lemmas + predicates)
    if re.search(rb"\b(?:assume|admit|external_body|external_fn_specification|assume_specification|axiom|requires)\b",
                 local_code):
        raise ValueError("Rational candidate contains a forbidden local assumption or precondition")
    bridge = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
              b"// UNPROVED rational-order dependency source; full bridge acceptance remains open.\n"
              b"use std::cmp::Ordering;\nuse vstd::prelude::*;\n\nverus! {\n\n"
              + b"\n".join(selected[name] for name in TYPE_NAMES) + b"\n\n"
              + ghost + b"\n" + lemmas + b"\n" + complement_lemmas + b"\n"
              + rational_lemmas + b"\n" + predicates + b"\n}\n")
    modes = {"proof::lemma_bridge_bit_length_power": "proof",
             "proof::lemma_bridge_u128_shift_power": "proof",
             "proof::lemma_bridge_complement_reject_outside": "proof",
             "proof::lemma_bridge_u128_shifted_u64": "proof",
             "proof::lemma_bridge_denominator_factor": "proof",
             "proof::lemma_bridge_positive_factor_order": "proof",
             "proof::lemma_bridge_rational_gap_dominates": "proof",
             "proof::composition_constructor_admitted": "exec",
             "proof::composition_checked": "exec",
             "proof::composition_constructor_post": "exec",
             "proof::composition_rational_order": "exec",
             "proof::composition_complement_post": "exec",
             "proof::IntervalError::clone": "exec",
             "proof::FixtureEnclosureError::clone": "exec",
             "proof::Fraction::clone": "exec"}
    hooks = [{"function": "proof::" + name,
              "source_byte_offset": prefix.index(anchor),
              "insertion": ("before unchanged leading_zeros subtraction" if index < 2
                            else "at approved complement proof site" if index < 5
                            else "at approved rational-order proof site"),
              "anchor_sha256": digest(anchor), "proof_block_sha256": digest(block)}
             for index, (name, anchor, block) in enumerate(
                 (*BIT_LENGTH_HOOKS, *COMPLEMENT_HOOKS, *RATIONAL_HOOKS))]
    generated = {"proof.rs": bridge, "actual_source.rs": old_payload["actual_source.rs"],
                 "predicate_source.rs": prefix}
    details = {
        "schema_version": 1, "status": "unproved_source_candidate", "proof_commands": 0,
        "scope": "bit-length, complement and rational lemmas and five unrestricted predicate equalities; source preparation only",
        "source_files_sha256": original["source_files_sha256"],
        "original_sources_sha256": original["original_sources_sha256"],
        "correspondence": original["correspondence"],
        "prefix_sha256": digest(prefix), "prefix_bytes": len(prefix),
        "fragment_source_spans": [
            {"source_byte_offsets_half_open": list(span), "sha256": expected}
            for span, expected in zip(RATIONAL_SOURCE_SPANS, span_hashes)],
        "fragment_sha256": digest(fragment), "annotated_fragment_sha256": digest(predicates),
        "ghost_sha256": digest(ghost), "lemma_sha256": digest(lemmas),
        "complement_lemma_sha256": digest(complement_lemmas),
        "rational_lemma_sha256": digest(rational_lemmas),
        "predicate_relations": [DECLARATIONS[index][1].decode() for index in (0, 1, 2, 3, 5)],
        "full_bridge_predicate_relations": original["predicate_relations"],
        "expected_function_modes": modes, "proof_hooks": hooks,
        "raw_domains": {
            "lemma": "every n:u64 and s:u32; no precondition",
            "shift_power": "every s:u32; no precondition; guarded shift/power implication for 1 <= s <= 53",
            "reject_outside": "every n:u64 and s:u32; no precondition; guarded checked outside-window implication",
            "constructor_admitted": "every numerator:u64 and shift:u32; no precondition",
            "checked": "every raw Fraction { numerator:u64, shift:u32 }; no precondition",
            "constructor_post": "every numerator:u64, shift:u32 and independently chosen &Result<Fraction, FixtureEnclosureError>; raw Ok fields and every error variant; no precondition",
            "complement_post": "every raw Fraction { numerator:u64, shift:u32 } and independently chosen &Result<Fraction, FixtureEnclosureError>; raw Ok fields and every error variant; no precondition",
            "rational_order": "every independent left/right raw Fraction { numerator:u64, shift:u32 }; invalid either side maps to Equal; no precondition",
            "shifted_u64": "every n:u64 and g:u32; no precondition; exact u128 shift/product and u32/u128 shift identity guarded by g < 64",
            "denominator_factor": "every lo:u32 and hi:u32; no precondition; both powers positive and factorization guarded by lo <= hi",
            "positive_factor_order": "every a:int, b:int and k:int; no precondition; complete order equivalence guarded by k > 0",
            "gap_dominates": "every n:u64, m:u64 and g:u32; no precondition; dominance guarded by n >= 1, m < pow(2,53) and g >= 64",
        },
        "upstream_dependencies": {
            "commit": "8bf68ee741df833ea5d0fe6a2255126fdea87e47",
            "paths": ["source/vstd/std_specs/bits.rs", "source/vstd/arithmetic/power.rs",
                      "source/vstd/arithmetic/power2.rs", "source/vstd/arithmetic/div_mod.rs",
                      "source/vstd/bits.rs", "source/vstd/std_specs/convert.rs",
                      "source/vstd/arithmetic/mul.rs", "source/vstd/std_specs/cmp.rs", "source/vstd/std_specs/num.rs"],
            "status": "source API references only; no new admission or proof"},
        "remaining": ["exact Verus compatibility and all lemma assertions/termination",
                      "u32 leading_zeros result and subtraction-to-int inference",
                      "five completed full-raw predicate equalities and wrong-contract/restoration controls",
                      "all six full-domain bridge equalities and their controls",
                      "native Ordering equality correspondence and comparison Post",
                      "separate caller-import profile admission and original addition union"],
        "generated_sha256": {name: digest(data) for name, data in generated.items()},
    }
    generated["correspondence.json"] = (json.dumps(details, indent=2, sort_keys=True) + "\n").encode()
    _write_payload(inputs, generated)
    return details
