"""Project byte-exact resolver/type/getter spans; replace no executable body.

The selected consumer proof explicitly assumes an admitted immutable window.
The public garde constructor and its error/drop graph are omitted, not stubbed
or claimed verified. Full coordinate construction and the output constructor run.
"""
from __future__ import annotations

import hashlib
import json
from pathlib import Path

ERA_SHA256 = '366397df0c6bcd260d566b5bac310c882796734887af886a5fdf99a023bb1dc8'
COORDINATE_SHA256 = '63f7230f915491973d4d682ee66afa5dc0c22bd8bc38fcc375d1840b3c046e80'
ERROR_ENUM_SHA256 = 'a63f855634262baf2586e21312378f935a52e9e0ea02e77805ffd1083c01fd35'
SPANS = (
    (6, 9, '1133002046c6797add1a6c70fff4f6a841396144425c471a867a0f4cb2df832e'),
    (59, 67, 'b69405a5c200fb953260bfa19eebfd2679fbb5581c019eaefc7bc679903377c9'),
    (89, 145, '2824f418d5260d333c159b1dfecfcd07e574456098f42734f6513f34e5840eda'),
)


def digest(data: bytes) -> str:
    """Return exact byte identity, without normalizing Rust source."""
    return hashlib.sha256(data).hexdigest()


def prepare_era(source_root: Path, inputs: Path, case: str, harness_path: Path) -> dict:
    """Require frozen reviewed spans, emit a complete source-bound proof input.

    Reject source drift before emitting the proof. Healthy code is copied byte
    for byte. Non-healthy cases make exactly one documented compiled mutation.
    The proof-only constructor asserts actual coordinate admission before using
    the original private aggregate fields; it substitutes no runtime API.
    """
    domain = source_root / 'crates/time-domain/src'
    era = (domain / 'era.rs').read_bytes()
    coordinate = (domain / 'coordinate.rs').read_bytes()
    if digest(era) != ERA_SHA256 or digest(coordinate) != COORDINATE_SHA256:
        raise ValueError('Era/coordinate source differs from reviewed correspondence')
    lines = era.splitlines(keepends=True)
    parts, correspondence = [], []
    for first, last, expected in SPANS:
        part = b''.join(lines[first - 1:last])
        if digest(part) != expected:
            raise ValueError('Reviewed era span no longer matches')
        parts.append(part)
        correspondence.append({'source': 'crates/time-domain/src/era.rs',
                               'lines_inclusive': [first, last], 'sha256': expected})
    subject = b''.join(parts).decode()
    mutation = None
    edits = {
        'always_error': ('    let raw = i128::from',
                         '    return Err(DomainError::NoEraCandidate);\n    let raw = i128::from'),
        'wrong_coordinate': ('    UtcCoordinateQ32::new(ticks)',
                             '    UtcCoordinateQ32::new(ticks + 1)'),
        'wrong_error': ('return Err(DomainError::NoEraCandidate);',
                        'return Err(DomainError::AmbiguousEra);'),
    }
    if case != 'healthy':
        if case not in edits:
            raise ValueError('Unknown era control')
        old, new = edits[case]
        if old not in subject:
            raise ValueError('Reviewed era mutation point missing')
        subject = subject.replace(old, new, 1)
        mutation = {'before': old, 'after': new}
    subject += '''
/// Proof-only private assembly, never a substitute product constructor.
pub(crate) fn proof_window(lower: i128, upper: i128) -> EraWindow {
    assert!(lower <= upper, "valid ordered private window");
    let low = UtcCoordinateQ32::new(lower);
    let high = UtcCoordinateQ32::new(upper);
    assert!(low.is_ok() && high.is_ok(), "actual coordinate constructors accept");
    match (low, high) {
        (Ok(earliest), Ok(latest)) => EraWindow { earliest, latest },
        _ => { assert!(false, "coordinate construction required"); panic!("invalid proof input") }
    }
}
'''
    lib_lines = (domain / 'lib.rs').read_bytes().splitlines(keepends=True)
    error_enum = b''.join(lib_lines[25:45])
    if digest(error_enum) != ERROR_ENUM_SHA256:
        raise ValueError('Actual DomainError enum changed')
    inputs.mkdir(parents=True, exist_ok=True)
    (inputs / 'coordinate.rs').write_bytes(coordinate)
    (inputs / 'era_selected.rs').write_text(subject)
    proof = ('#![allow(dead_code)]\n' + error_enum.decode()
             + '\nmod coordinate;\nuse coordinate::UtcCoordinateQ32;\n'
             + '#[path="era_selected.rs"]\nmod era;\n'
             + harness_path.read_text().replace('//!', '//'))
    (inputs / 'proof.rs').write_text(proof)
    details = {'entrypoint': inputs / 'proof.rs', 'correspondence': correspondence,
               'original_era_sha256': digest(era), 'coordinate_sha256': digest(coordinate),
               'domain_error_enum_sha256': digest(error_enum), 'mutation': mutation,
               'omitted': ['EraWindowCandidate', 'endpoint_rule', 'ordered_endpoints', 'EraWindow::new'],
               'scope': 'actual selected resolver body under valid immutable-window invariant; no garde constructor proof'}
    (inputs / 'correspondence.json').write_text(json.dumps({**details, 'entrypoint': 'proof.rs'}, indent=2)+'\n')
    return details
