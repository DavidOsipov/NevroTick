"""Prepare actual ScaledValue representation methods without running a verifier.

Ten exact source spans retain constants, private types, actual split/new callees
and typed errors. Every mutation changes one method-body site only. Raw-domain
contracts and covers remain lexically inside the private proof.rs module.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path


SOURCE_SHA256 = {'projection_enclosure.rs': '6b7b079f7d820e611bd13bb96361c243973d231a7f65a07fb6d4350a63282023',
 'selection.rs': 'da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e'}
SPANS = {'fraction': {'source': 'projection_enclosure.rs',
              'offsets': (8107, 8415),
              'sha256': '4742bce4685790e469ecdf5720cf569d71d9ed47c6a23ec808c4b39b14ff99a4',
              'start': b'/// Canonical proper nonnegative dyadic fraction',
              'following': b'impl Fraction {'},
 'zero': {'source': 'projection_enclosure.rs',
          'offsets': (8432, 8595),
          'sha256': '9fcc4427cb030845d0a527fbd2cc00b3037eba2e44d01009c96aed86f74fe6d4',
          'start': b'    /// Canonical exact zero;',
          'following': b'    /// Check a transformed numerator/denominator'},
 'new': {'source': 'projection_enclosure.rs',
         'offsets': (8596, 9327),
         'sha256': '4bdb19b66cf6e80a173cd05ba5553242c45bf1280600c24e8da49e25d7aaa6f2',
         'start': b'    /// Check a transformed numerator/denominator',
         'following': b'    /// Compare two checked fractions exactly'},
 'scaled_value': {'source': 'projection_enclosure.rs',
                  'offsets': (11718, 12113),
                  'sha256': 'f6f5a446aea1b101939f8d861185a61d4d5ceb9795497e82a803b7a648aa93d8',
                  'start': b'/// Exact sign and magnitude of one admitted operand multiplied by',
                  'following': b'impl ScaledValue {'},
 'split': {'source': 'projection_enclosure.rs',
           'offsets': (13289, 14550),
           'sha256': 'b1bb449fc7055801f776adc2b048a54de9ee3b02922af78da7c44f0690d6ed9f',
           'start': b'    /// Split an IEEE significand and scale',
           'following': b'    /// Copy the exact signed integral component;'},
 'enclosure_error': {'source': 'projection_enclosure.rs',
                     'offsets': (1608, 2231),
                     'sha256': '628d29b282da93e9bd20b15fb6b7af99a82e0985ac8330705bb6518dcffea901',
                     'start': b'/// Bounded enclosure failure',
                     'following': b'impl fmt::Debug for FixtureEnclosureError'},
 'interval_error': {'source': 'selection.rs',
                    'offsets': (1796, 2163),
                    'sha256': 'b4afc7fa071599702078052cab5205d145e01ee1c0ea91a1cfccaef540130399',
                    'start': b'/// Bounded, payload-free interval-construction',
                    'following': b'/// Bounded, payload-free fixture-minimum'},
 'maximum': {'source': 'projection_enclosure.rs',
             'offsets': (1140, 1267),
             'sha256': '97f78bd5761bf362eea995a0efe20dda5c644133fc038b70eec61069cfc1be8a',
             'start': b'/// Largest admitted scaled operand magnitude,',
             'following': b'/// Whether one exact represented endpoint'},
 'from_bits': {'source': 'projection_enclosure.rs',
               'offsets': (12133, 13288),
               'sha256': '75dd681ca71d90ccdb9d016ed1aca592eddcad37e131efd1292c7d9d405462e1',
               'start': b'    /// Decompose finite admitted bits',
               'following': b'    /// Split an IEEE significand and scale'},
 'signed_whole': {'source': 'projection_enclosure.rs',
                  'offsets': (14551, 14800),
                  'sha256': 'd3ffa87abac01c8ffe7c302d5b4a61b036f9b0504fe29ad3df9f92cee9811824',
                  'start': b'    /// Copy the exact signed integral component;',
                  'following': b'}\n\n/// Two immutable scaled operands'}}
SUBJECTS = ("signed_whole", "from_bits")
FULL_HARNESSES = {
    "signed_whole": "scaled_value_representation::signed_whole_full_domain",
    "from_bits": "scaled_value_representation::from_bits_full_domain",
}
CONTROLS = {
    "signed_whole": {
        "omit_negation": {
            "before": "-i128::from(self.whole)",
            "after": "i128::from(self.whole)",
            "harness": "scaled_value_representation::signed_whole_control_sign",
            "intended_failure": "signed whole strict sign",
        },
        "narrow_magnitude": {
            "before": "} else {\n            i128::from(self.whole)\n        }",
            "after": "} else {\n            i128::from(self.whole as i64)\n        }",
            "harness": "scaled_value_representation::signed_whole_control_magnitude",
            "intended_failure": "signed whole exact magnitude",
        },
        "use_fraction": {
            "before": "} else {\n            i128::from(self.whole)\n        }",
            "after": "} else {\n            i128::from(self.whole) + i128::from(self.fraction.numerator)\n        }",
            "harness": "scaled_value_representation::signed_whole_control_fraction",
            "intended_failure": "signed whole fraction independence",
        },
    },
    "from_bits": {
        "wrong_subnormal_scale": {
            "before": "(mantissa, -1041_i64)",
            "after": "(mantissa, -1040_i64)",
            "harness": "scaled_value_representation::from_bits_control_subnormal",
            "intended_failure": "from bits nonzero shift retained",
        },
        "lose_sign": {
            "before": "negative: bits >> 63 != 0,",
            "after": "negative: false,",
            "harness": "scaled_value_representation::from_bits_control_sign",
            "intended_failure": "from bits original sign retained",
        },
        "wrong_error": {
            "before": "if whole > MAX_SCALED_MAGNITUDE {\n"
                      "            return Err(FixtureEnclosureError::Invariant);\n        }",
            "after": "if whole > MAX_SCALED_MAGNITUDE {\n"
                     "            return Err(FixtureEnclosureError::Unrepresentable);\n        }",
            "harness": "scaled_value_representation::from_bits_control_error",
            "intended_failure": "from bits rejection classification",
        },
    },
}
REQUIRED_ASSERTIONS = {
    FULL_HARNESSES["signed_whole"]: (
        "signed whole exact magnitude", "signed whole strict sign", "signed whole zero equivalence",
        "signed whole raw envelope", "signed whole fraction independence",
    ),
    FULL_HARNESSES["from_bits"]: (
        "from bits required success", "from bits rejection classification", "from bits original sign retained",
        "from bits exact integral magnitude", "from bits integral zero fraction",
        "from bits exact quotient remainder", "from bits strict remainder bound",
        "from bits large denominator whole zero", "from bits large denominator numerator retained",
        "from bits zero canonicalization", "from bits nonzero shift retained", "from bits whole envelope",
        "from bits output numerator bound", "from bits output shift bound", "from bits output proper dyadic",
    ),
}
REQUIRED_COVERS = {
    FULL_HARNESSES["signed_whole"]: (
        "signed whole positive zero", "signed whole negative zero", "signed whole positive one",
        "signed whole negative one", "signed whole positive two", "signed whole negative two",
        "signed whole positive i64 maximum", "signed whole negative i64 maximum",
        "signed whole positive two to sixty three", "signed whole negative two to sixty three",
        "signed whole positive above two to sixty three", "signed whole negative above two to sixty three",
        "signed whole positive below u64 maximum", "signed whole negative below u64 maximum",
        "signed whole positive u64 maximum", "signed whole negative u64 maximum",
        "signed whole positive fractional zero", "signed whole negative fractional zero",
        "signed whole ignores raw oversized fraction", "signed whole ignores raw maximum fraction shift",
    ),
    FULL_HARNESSES["from_bits"]: (
        "from bits positive zero", "from bits negative zero", "from bits positive minimum subnormal",
        "from bits negative minimum subnormal", "from bits maximum subnormal", "from bits minimum normal",
        "from bits half quantum", "from bits below quantum", "from bits integral quantum",
        "from bits above quantum", "from bits denominator sixty three", "from bits denominator sixty four",
        "from bits positive magnitude boundary", "from bits negative magnitude boundary",
        "from bits below magnitude boundary", "from bits above magnitude boundary rejected",
        "from bits positive infinity rejected", "from bits negative infinity rejected",
        "from bits quiet NaN rejected", "from bits negative signalling NaN rejected",
    ),
}


def digest(data: bytes) -> str:
    """Identify unchanged original, projected and harness bytes."""
    return hashlib.sha256(data).hexdigest()


def schedule() -> list[dict]:
    """Two independent fixed five-job groups, signed-whole before from-bits."""
    jobs = []
    for subject in SUBJECTS:
        jobs.append({"subject": subject, "case": "healthy", "harness": FULL_HARNESSES[subject],
                     "intended_failure": None, "restored_healthy": False})
        for case, control in CONTROLS[subject].items():
            jobs.append({"subject": subject, "case": case, "harness": control["harness"],
                         "intended_failure": control["intended_failure"], "restored_healthy": False})
        jobs.append({"subject": subject, "case": "healthy", "harness": FULL_HARNESSES[subject],
                     "intended_failure": None, "restored_healthy": True})
    return jobs


def _extract(originals: dict[str, bytes], name: str) -> bytes:
    """Reject ambiguous contexts, changed offsets and changed executable spans."""
    span = SPANS[name]
    source = originals[span["source"]]
    first, last = span["offsets"]
    if source.count(span["start"]) != 1 or source.count(span["following"]) != 1:
        raise ValueError("Actual representation extraction is ambiguous: " + name)
    selected = source[first:last]
    if (source.index(span["start"]) != first or source.index(span["following"]) < last
            or digest(selected) != span["sha256"]):
        raise ValueError("Reviewed representation source span changed: " + name)
    return selected


def prepare_scaled_value_representation(root: Path, inputs: Path, subject: str, case: str,
                                        harness: Path) -> dict:
    """Prepare one selected actual method with one permitted semantic mutation.

    All source and output-collision checks precede output creation. Arbitrary
    fraction fields are a private signed-whole proof-domain extension only;
    this grants no construction permission to product callers.
    """
    if subject not in SUBJECTS:
        raise ValueError("Unknown scaled-value representation subject")
    if case != "healthy" and case not in CONTROLS[subject]:
        raise ValueError("Unknown scaled-value representation control")
    core = root / "crates/time-core/src"
    originals = {name: (core / name).read_bytes() for name in SOURCE_SHA256}
    for name, expected in SOURCE_SHA256.items():
        if digest(originals[name]) != expected:
            raise ValueError("Reviewed complete source changed: " + name)
    selected = {name: _extract(originals, name) for name in SPANS}
    methods = {name: selected[name] for name in ("from_bits", "split", "signed_whole")}
    mutation = None
    if case != "healthy":
        mutation = dict(CONTROLS[subject][case])
        before, after = mutation["before"].encode(), mutation["after"].encode()
        if methods[subject].count(before) != 1:
            raise ValueError("Reviewed representation mutation point is not unique")
        methods[subject] = methods[subject].replace(before, after, 1)
    harness_bytes = harness.read_bytes()
    definitions = (b"// SPDX-License-Identifier: GPL-2.0-only\n"
                   b"// Byte-exact selected owners; correspondence.json retains original locations.\n"
                   + selected["interval_error"] + b"\n" + selected["enclosure_error"] + b"\n"
                   + selected["maximum"] + b"\n" + selected["fraction"] + b"\nimpl Fraction {\n"
                   + selected["zero"] + b"\n" + selected["new"] + b"}\n"
                   + selected["scaled_value"] + b"\nimpl ScaledValue {\n"
                   + methods["from_bits"] + b"\n" + methods["split"] + b"\n"
                   + methods["signed_whole"] + b"}\n")
    proof = (b"#![allow(dead_code)]\nmod scaled_value_representation {\n"
             b'include!("scaled_value_representation_selected.rs");\n' + harness_bytes + b"\n}\n")
    destinations = [inputs / name for name in
                    ("proof.rs", "scaled_value_representation_selected.rs", "correspondence.json")]
    if inputs.is_symlink() or (inputs.exists() and not inputs.is_dir()):
        raise ValueError("Representation input directory redirects or is not a directory")
    if any(path.exists() or path.is_symlink() for path in destinations):
        raise ValueError("Refusing to overwrite existing representation proof inputs")
    details = {
        "subject": subject, "entrypoint": inputs / "proof.rs",
        "original_sources_sha256": dict(SOURCE_SHA256),
        "correspondence": [{"item": name, "source": "crates/time-core/src/" + span["source"],
                            "byte_offsets_half_open": list(span["offsets"]),
                            "sha256": digest(selected[name])} for name, span in SPANS.items()],
        "original_helper_preserved": case == "healthy",
        "compiled_helper_sha256": digest(methods[subject]),
        "compiled_methods_sha256": {name: digest(data) for name, data in methods.items()},
        "original_other_owners_preserved": True, "harness_inside_private_module": True,
        "raw_private_payload_extension": subject == "signed_whole",
        "harness_sha256": digest(harness_bytes), "harness_location": "proof.rs",
        "compiled_definitions_sha256": digest(definitions), "entrypoint_sha256": digest(proof),
        "mutation": mutation, "assumptions": [],
        "omitted": ["The other representation leaf requires its own selected proof despite included source",
                    "Fraction::compare, Fraction::complement and fraction addition",
                    "EndpointParts construction/rounding and complete enclosure composition",
                    "Garde/raw-operand admission and checked interval construction",
                    "error formatting and error-trait implementations",
                    "chart roundoff, selection and complete Core/P1",
                    "production acquisition, native Windows, clock and release authority"],
        "scope": ("full raw bool/u64 whole/arbitrary fraction signed_whole magnitude and sign only"
                  if subject == "signed_whole" else
                  "full raw u64 IEEE from_bits admission, exact Q33 decomposition and errors only"),
    }
    inputs.mkdir(mode=0o700, parents=True, exist_ok=True)
    destinations[0].write_bytes(proof)
    destinations[1].write_bytes(definitions)
    destinations[2].write_text(json.dumps({**details, "entrypoint": "proof.rs"}, indent=2) + "\n")
    return details
