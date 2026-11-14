"""Pure additive role/listing controls; no Rust, compiler or analyzer is executed."""
from __future__ import annotations

import copy
from contextlib import ExitStack
import difflib
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import tempfile
import time
import tomllib
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("phase_step_controls", ROOT / "scripts/verify-core-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)
ROLE = MODULE._phase_step_role()
OWNERS = MODULE.PHASE_STEP_SUBJECTS


# Exact retained-producer diagnostic literals. Current-source decoder controls
# below do not replay/adopt that historical execution or grant kill credit.
_PHASE_STEP_CAUSAL_SPECIMENS = {'608:9': {'mutant': {'name': 'crates/time-core/src/reference_input.rs:608:9: replace '
                              "FixtureReferenceInput<'_, '_, '_, '_>::emitted_round -> NonZeroU64 with "
                              '1.try_into().unwrap()',
                      'package': 'time-core',
                      'file': 'crates/time-core/src/reference_input.rs',
                      'function': {'function_name': "FixtureReferenceInput<'_, '_, '_, '_>::emitted_round",
                                   'return_type': '-> NonZeroU64',
                                   'span': {'start': {'line': 605, 'column': 5},
                                            'end': {'line': 609, 'column': 6}}},
                      'span': {'start': {'line': 608, 'column': 9}, 'end': {'line': 608, 'column': 27}},
                      'replacement': '1.try_into().unwrap()',
                      'genre': 'FnValue'},
           'diff': '--- crates/time-core/src/reference_input.rs\n'
                   "+++ replace FixtureReferenceInput<'_, '_, '_, '_>::emitted_round -> NonZeroU64 with "
                   '1.try_into().unwrap()\n'
                   '@@ -600,11 +600,11 @@\n'
                   '     #[must_use]\n'
                   "     pub const fn combined(&self) -> &FixtureChartCombinedEstimate<'_, '_, '_> {\n"
                   '         self.combined\n'
                   '     }\n'
                   ' \n'
                   '     /// Copy the original nonzero emitted round without creating further credit.\n'
                   '     #[must_use]\n'
                   '     pub const fn emitted_round(&self) -> NonZeroU64 {\n'
                   '-        self.emitted_round\n'
                   '+        1.try_into().unwrap() /* ~ changed by cargo-mutants ~ */\n'
                   '     }\n'
                   ' }\n',
           'records': [{'reason': 'compiler-message',
                        'package_id': 'path+file:///work/tmp/cargo-mutants-source-cBg5iy.tmp/crates/time-core#0.0.0',
                        'manifest_path': '/work/tmp/cargo-mutants-source-cBg5iy.tmp/crates/time-core/Cargo.toml',
                        'target': {'kind': ['lib'],
                                   'crate_types': ['lib'],
                                   'name': 'time_core',
                                   'src_path': '/work/tmp/cargo-mutants-source-cBg5iy.tmp/crates/time-core/src/lib.rs',
                                   'edition': '2024',
                                   'doc': True,
                                   'doctest': True,
                                   'test': True},
                        'message': {'rendered': 'error[E0658]: cannot call conditionally-const method `<u64 '
                                                'as TryInto<NonZero<u64>>>::try_into` in constant functions\n'
                                                '   --> crates/time-core/src/reference_input.rs:608:11\n'
                                                '    |\n'
                                                '608 |         1.try_into().unwrap() /* ~ changed by '
                                                'cargo-mutants ~ */\n'
                                                '    |           ^^^^^^^^^^\n'
                                                '    |\n'
                                                '    = note: calls in constant functions are limited to '
                                                'constant functions, tuple structs and tuple variants\n'
                                                '    = note: see issue #143874 '
                                                '<https://github.com/rust-lang/rust/issues/143874> for more '
                                                'information\n'
                                                '\n',
                                    '$message_type': 'diagnostic',
                                    'children': [{'children': [],
                                                  'code': None,
                                                  'level': 'note',
                                                  'message': 'calls in constant functions are limited to '
                                                             'constant functions, tuple structs and tuple '
                                                             'variants',
                                                  'rendered': None,
                                                  'spans': []},
                                                 {'children': [],
                                                  'code': None,
                                                  'level': 'note',
                                                  'message': 'see issue #143874 '
                                                             '<https://github.com/rust-lang/rust/issues/143874> '
                                                             'for more information',
                                                  'rendered': None,
                                                  'spans': []}],
                                    'level': 'error',
                                    'message': 'cannot call conditionally-const method `<u64 as '
                                               'TryInto<NonZero<u64>>>::try_into` in constant functions',
                                    'spans': [{'byte_end': 26827,
                                               'byte_start': 26817,
                                               'column_end': 21,
                                               'column_start': 11,
                                               'expansion': None,
                                               'file_name': 'crates/time-core/src/reference_input.rs',
                                               'is_primary': True,
                                               'label': None,
                                               'line_end': 608,
                                               'line_start': 608,
                                               'suggested_replacement': None,
                                               'suggestion_applicability': None,
                                               'text': [{'highlight_end': 21,
                                                         'highlight_start': 11,
                                                         'text': '        1.try_into().unwrap() /* ~ changed '
                                                                 'by cargo-mutants ~ */'}]}],
                                    'code': {'code': 'E0658',
                                             'explanation': 'An unstable feature was used.\n'
                                                            '\n'
                                                            'Erroneous code example:\n'
                                                            '\n'
                                                            '```compile_fail,E0658\n'
                                                            'use std::intrinsics; // error: use of unstable '
                                                            'library feature `core_intrinsics`\n'
                                                            '```\n'
                                                            '\n'
                                                            "If you're using a stable or a beta version of "
                                                            "rustc, you won't be able to use\n"
                                                            'any unstable features. In order to do so, '
                                                            'please switch to a nightly version of\n'
                                                            'rustc (by using [rustup]).\n'
                                                            '\n'
                                                            "If you're using a nightly version of rustc, "
                                                            'just add the corresponding feature\n'
                                                            'to be able to use it:\n'
                                                            '\n'
                                                            '```\n'
                                                            '#![feature(core_intrinsics)]\n'
                                                            '\n'
                                                            'use std::intrinsics; // ok!\n'
                                                            '```\n'
                                                            '\n'
                                                            '[rustup]: '
                                                            'https://rust-lang.github.io/rustup/concepts/channels.html\n'}}},
                       {'reason': 'compiler-message',
                        'package_id': 'path+file:///work/tmp/cargo-mutants-source-cBg5iy.tmp/crates/time-core#0.0.0',
                        'manifest_path': '/work/tmp/cargo-mutants-source-cBg5iy.tmp/crates/time-core/Cargo.toml',
                        'target': {'kind': ['lib'],
                                   'crate_types': ['lib'],
                                   'name': 'time_core',
                                   'src_path': '/work/tmp/cargo-mutants-source-cBg5iy.tmp/crates/time-core/src/lib.rs',
                                   'edition': '2024',
                                   'doc': True,
                                   'doctest': True,
                                   'test': True},
                        'message': {'rendered': 'error: `TryInto` is not yet stable as a const trait\n'
                                                '   --> crates/time-core/src/reference_input.rs:608:9\n'
                                                '    |\n'
                                                '608 |         1.try_into().unwrap() /* ~ changed by '
                                                'cargo-mutants ~ */\n'
                                                '    |         ^^^^^^^^^^^^\n'
                                                '\n',
                                    '$message_type': 'diagnostic',
                                    'children': [],
                                    'level': 'error',
                                    'message': '`TryInto` is not yet stable as a const trait',
                                    'spans': [{'byte_end': 26827,
                                               'byte_start': 26815,
                                               'column_end': 21,
                                               'column_start': 9,
                                               'expansion': None,
                                               'file_name': 'crates/time-core/src/reference_input.rs',
                                               'is_primary': True,
                                               'label': None,
                                               'line_end': 608,
                                               'line_start': 608,
                                               'suggested_replacement': None,
                                               'suggestion_applicability': None,
                                               'text': [{'highlight_end': 21,
                                                         'highlight_start': 9,
                                                         'text': '        1.try_into().unwrap() /* ~ changed '
                                                                 'by cargo-mutants ~ */'}]}],
                                    'code': None}},
                       {'reason': 'compiler-message',
                        'package_id': 'path+file:///work/tmp/cargo-mutants-source-cBg5iy.tmp/crates/time-core#0.0.0',
                        'manifest_path': '/work/tmp/cargo-mutants-source-cBg5iy.tmp/crates/time-core/Cargo.toml',
                        'target': {'kind': ['lib'],
                                   'crate_types': ['lib'],
                                   'name': 'time_core',
                                   'src_path': '/work/tmp/cargo-mutants-source-cBg5iy.tmp/crates/time-core/src/lib.rs',
                                   'edition': '2024',
                                   'doc': True,
                                   'doctest': True,
                                   'test': True},
                        'message': {'rendered': 'error[E0015]: cannot call non-const method '
                                                '`Result::<NonZero<u64>, TryFromIntError>::unwrap` in '
                                                'constant functions\n'
                                                '   --> crates/time-core/src/reference_input.rs:608:22\n'
                                                '    |\n'
                                                '608 |         1.try_into().unwrap() /* ~ changed by '
                                                'cargo-mutants ~ */\n'
                                                '    |                      ^^^^^^^^\n'
                                                '    |\n'
                                                '    = note: calls in constant functions are limited to '
                                                'constant functions, tuple structs and tuple variants\n'
                                                '\n',
                                    '$message_type': 'diagnostic',
                                    'children': [{'children': [],
                                                  'code': None,
                                                  'level': 'note',
                                                  'message': 'calls in constant functions are limited to '
                                                             'constant functions, tuple structs and tuple '
                                                             'variants',
                                                  'rendered': None,
                                                  'spans': []}],
                                    'level': 'error',
                                    'message': 'cannot call non-const method `Result::<NonZero<u64>, '
                                               'TryFromIntError>::unwrap` in constant functions',
                                    'spans': [{'byte_end': 26836,
                                               'byte_start': 26828,
                                               'column_end': 30,
                                               'column_start': 22,
                                               'expansion': None,
                                               'file_name': 'crates/time-core/src/reference_input.rs',
                                               'is_primary': True,
                                               'label': None,
                                               'line_end': 608,
                                               'line_start': 608,
                                               'suggested_replacement': None,
                                               'suggestion_applicability': None,
                                               'text': [{'highlight_end': 30,
                                                         'highlight_start': 22,
                                                         'text': '        1.try_into().unwrap() /* ~ changed '
                                                                 'by cargo-mutants ~ */'}]}],
                                    'code': {'code': 'E0015',
                                             'explanation': 'A non-`const` function was called in a `const` '
                                                            'context.\n'
                                                            '\n'
                                                            'Erroneous code example:\n'
                                                            '\n'
                                                            '```compile_fail,E0015\n'
                                                            'fn create_some() -> Option<u8> {\n'
                                                            '    Some(1)\n'
                                                            '}\n'
                                                            '\n'
                                                            '// error: cannot call non-const function '
                                                            '`create_some` in constants\n'
                                                            'const FOO: Option<u8> = create_some();\n'
                                                            '```\n'
                                                            '\n'
                                                            'All functions used in a `const` context '
                                                            '(constant or static expression) must\n'
                                                            'be marked `const`.\n'
                                                            '\n'
                                                            'To fix this error, you can declare '
                                                            '`create_some` as a constant function:\n'
                                                            '\n'
                                                            '```\n'
                                                            '// declared as a `const` function:\n'
                                                            'const fn create_some() -> Option<u8> {\n'
                                                            '    Some(1)\n'
                                                            '}\n'
                                                            '\n'
                                                            'const FOO: Option<u8> = create_some(); // no '
                                                            'error!\n'
                                                            '```\n'}}}]},
 '324:42': {'mutant': {'name': 'crates/time-core/src/reference_input.rs:324:42: replace != with == in '
                               'reference_chart_mapping',
                       'package': 'time-core',
                       'file': 'crates/time-core/src/reference_input.rs',
                       'function': {'function_name': 'reference_chart_mapping',
                                    'return_type': '-> Result<(), FixtureReferenceInputError>',
                                    'span': {'start': {'line': 315, 'column': 1},
                                             'end': {'line': 337, 'column': 2}}},
                       'span': {'start': {'line': 324, 'column': 42}, 'end': {'line': 324, 'column': 44}},
                       'replacement': '==',
                       'genre': 'BinaryOperator'},
            'diff': '--- crates/time-core/src/reference_input.rs\n'
                    '+++ replace != with == in reference_chart_mapping\n'
                    '@@ -316,17 +316,17 @@\n'
                    ' /// # Errors\n'
                    ' /// Mapping mismatch precedes complete domain/reference mismatch; no effects occur.\n'
                    ' fn reference_chart_mapping(\n'
                    '     predecessor: &FixtureAffineChart,\n'
                    '     frames: FixtureChartFrames,\n'
                    '     domain: FixtureChartDomain,\n'
                    ' ) -> Result<(), FixtureReferenceInputError> {\n'
                    '     let predecessor_frames = predecessor.frames();\n'
                    '-    if predecessor_frames.target_frame() != frames.source_frame()\n'
                    '+    if predecessor_frames.target_frame() == /* ~ changed by cargo-mutants ~ */ '
                    'frames.source_frame()\n'
                    '         || predecessor_frames.target_generation() != frames.source_generation()\n'
                    '     {\n'
                    '         return Err(FixtureReferenceInputError::Chart {\n'
                    '             source: FixtureChartError::CompositionMappingMismatch,\n'
                    '         });\n'
                    '     }\n'
                    '     if predecessor.domain() != domain {\n'
                    '         return Err(FixtureReferenceInputError::Chart {\n'},
 '325:51': {'mutant': {'name': 'crates/time-core/src/reference_input.rs:325:51: replace != with == in '
                               'reference_chart_mapping',
                       'package': 'time-core',
                       'file': 'crates/time-core/src/reference_input.rs',
                       'function': {'function_name': 'reference_chart_mapping',
                                    'return_type': '-> Result<(), FixtureReferenceInputError>',
                                    'span': {'start': {'line': 315, 'column': 1},
                                             'end': {'line': 337, 'column': 2}}},
                       'span': {'start': {'line': 325, 'column': 51}, 'end': {'line': 325, 'column': 53}},
                       'replacement': '==',
                       'genre': 'BinaryOperator'},
            'diff': '--- crates/time-core/src/reference_input.rs\n'
                    '+++ replace != with == in reference_chart_mapping\n'
                    '@@ -317,17 +317,17 @@\n'
                    ' /// Mapping mismatch precedes complete domain/reference mismatch; no effects occur.\n'
                    ' fn reference_chart_mapping(\n'
                    '     predecessor: &FixtureAffineChart,\n'
                    '     frames: FixtureChartFrames,\n'
                    '     domain: FixtureChartDomain,\n'
                    ' ) -> Result<(), FixtureReferenceInputError> {\n'
                    '     let predecessor_frames = predecessor.frames();\n'
                    '     if predecessor_frames.target_frame() != frames.source_frame()\n'
                    '-        || predecessor_frames.target_generation() != frames.source_generation()\n'
                    '+        || predecessor_frames.target_generation() == /* ~ changed by cargo-mutants ~ '
                    '*/ frames.source_generation()\n'
                    '     {\n'
                    '         return Err(FixtureReferenceInputError::Chart {\n'
                    '             source: FixtureChartError::CompositionMappingMismatch,\n'
                    '         });\n'
                    '     }\n'
                    '     if predecessor.domain() != domain {\n'
                    '         return Err(FixtureReferenceInputError::Chart {\n'
                    '             source: FixtureChartError::CompositionDomainMismatch,\n'},
 '331:29': {'mutant': {'name': 'crates/time-core/src/reference_input.rs:331:29: replace != with == in '
                               'reference_chart_mapping',
                       'package': 'time-core',
                       'file': 'crates/time-core/src/reference_input.rs',
                       'function': {'function_name': 'reference_chart_mapping',
                                    'return_type': '-> Result<(), FixtureReferenceInputError>',
                                    'span': {'start': {'line': 315, 'column': 1},
                                             'end': {'line': 337, 'column': 2}}},
                       'span': {'start': {'line': 331, 'column': 29}, 'end': {'line': 331, 'column': 31}},
                       'replacement': '==',
                       'genre': 'BinaryOperator'},
            'diff': '--- crates/time-core/src/reference_input.rs\n'
                    '+++ replace != with == in reference_chart_mapping\n'
                    '@@ -323,17 +323,17 @@\n'
                    '     let predecessor_frames = predecessor.frames();\n'
                    '     if predecessor_frames.target_frame() != frames.source_frame()\n'
                    '         || predecessor_frames.target_generation() != frames.source_generation()\n'
                    '     {\n'
                    '         return Err(FixtureReferenceInputError::Chart {\n'
                    '             source: FixtureChartError::CompositionMappingMismatch,\n'
                    '         });\n'
                    '     }\n'
                    '-    if predecessor.domain() != domain {\n'
                    '+    if predecessor.domain() == /* ~ changed by cargo-mutants ~ */ domain {\n'
                    '         return Err(FixtureReferenceInputError::Chart {\n'
                    '             source: FixtureChartError::CompositionDomainMismatch,\n'
                    '         });\n'
                    '     }\n'
                    '     Ok(())\n'
                    ' }\n'
                    ' \n'
                    ' /// Mandatory Garde context used and consumed inside the exclusive receiver.\n'},
 'rate:464:9': {'mutant': {'name': 'crates/time-core/src/reference_input/rate_delivery.rs:464:9: replace '
                                   'FixtureRateSettlementCursor::next_ordinal -> Option<NonZeroU64> with '
                                   'Some(1.try_into().unwrap())',
                           'package': 'time-core',
                           'file': 'crates/time-core/src/reference_input/rate_delivery.rs',
                           'function': {'function_name': 'FixtureRateSettlementCursor::next_ordinal',
                                        'return_type': '-> Option<NonZeroU64>',
                                        'span': {'start': {'line': 461, 'column': 5},
                                                 'end': {'line': 468, 'column': 6}}},
                           'span': {'start': {'line': 464, 'column': 9}, 'end': {'line': 467, 'column': 10}},
                           'replacement': 'Some(1.try_into().unwrap())',
                           'genre': 'FnValue'},
                'diff': '--- crates/time-core/src/reference_input/rate_delivery.rs\n'
                        '+++ replace FixtureRateSettlementCursor::next_ordinal -> Option<NonZeroU64> with '
                        'Some(1.try_into().unwrap())\n'
                        '@@ -456,20 +456,17 @@\n'
                        '             CursorProgress::Unresolved => FixtureRateCursorStatus::Unresolved,\n'
                        '             CursorProgress::Exhausted => FixtureRateCursorStatus::Exhausted,\n'
                        '         }\n'
                        '     }\n'
                        ' \n'
                        '     /// Copy the sole admissible local identity, absent after either terminal '
                        'state.\n'
                        '     #[must_use]\n'
                        '     pub const fn next_ordinal(&self) -> Option<NonZeroU64> {\n'
                        '-        match self.progress {\n'
                        '-            CursorProgress::Pending(value) => Some(value),\n'
                        '-            _ => None,\n'
                        '-        }\n'
                        '+        Some(1.try_into().unwrap()) /* ~ changed by cargo-mutants ~ */\n'
                        '     }\n'
                        ' \n'
                        '     /// Return the last known active endpoint without inventing Unknown delivery.\n'
                        '     #[must_use]\n'
                        '     pub const fn last_active_end(&self) -> Option<ActiveAwakeNs> {\n'
                        '         self.last_active\n'
                        '     }\n'
                        ' \n',
                'records': [{'reason': 'compiler-message',
                             'package_id': 'path+file:///work/tmp/cargo-mutants-source-847ijm.tmp/crates/time-core#0.0.0',
                             'manifest_path': '/work/tmp/cargo-mutants-source-847ijm.tmp/crates/time-core/Cargo.toml',
                             'target': {'kind': ['lib'],
                                        'crate_types': ['lib'],
                                        'name': 'time_core',
                                        'src_path': '/work/tmp/cargo-mutants-source-847ijm.tmp/crates/time-core/src/lib.rs',
                                        'edition': '2024',
                                        'doc': True,
                                        'doctest': True,
                                        'test': True},
                             'message': {'rendered': 'error[E0658]: cannot call conditionally-const method '
                                                     '`<u64 as TryInto<NonZero<u64>>>::try_into` in constant '
                                                     'functions\n'
                                                     '   --> '
                                                     'crates/time-core/src/reference_input/rate_delivery.rs:464:16\n'
                                                     '    |\n'
                                                     '464 |         Some(1.try_into().unwrap()) /* ~ changed '
                                                     'by cargo-mutants ~ */\n'
                                                     '    |                ^^^^^^^^^^\n'
                                                     '    |\n'
                                                     '    = note: calls in constant functions are limited to '
                                                     'constant functions, tuple structs and tuple variants\n'
                                                     '    = note: see issue #143874 '
                                                     '<https://github.com/rust-lang/rust/issues/143874> for '
                                                     'more information\n'
                                                     '\n',
                                         '$message_type': 'diagnostic',
                                         'children': [{'children': [],
                                                       'code': None,
                                                       'level': 'note',
                                                       'message': 'calls in constant functions are limited '
                                                                  'to constant functions, tuple structs and '
                                                                  'tuple variants',
                                                       'rendered': None,
                                                       'spans': []},
                                                      {'children': [],
                                                       'code': None,
                                                       'level': 'note',
                                                       'message': 'see issue #143874 '
                                                                  '<https://github.com/rust-lang/rust/issues/143874> '
                                                                  'for more information',
                                                       'rendered': None,
                                                       'spans': []}],
                                         'level': 'error',
                                         'message': 'cannot call conditionally-const method `<u64 as '
                                                    'TryInto<NonZero<u64>>>::try_into` in constant functions',
                                         'spans': [{'byte_end': 18464,
                                                    'byte_start': 18454,
                                                    'column_end': 26,
                                                    'column_start': 16,
                                                    'expansion': None,
                                                    'file_name': 'crates/time-core/src/reference_input/rate_delivery.rs',
                                                    'is_primary': True,
                                                    'label': None,
                                                    'line_end': 464,
                                                    'line_start': 464,
                                                    'suggested_replacement': None,
                                                    'suggestion_applicability': None,
                                                    'text': [{'highlight_end': 26,
                                                              'highlight_start': 16,
                                                              'text': '        Some(1.try_into().unwrap()) '
                                                                      '/* ~ changed by cargo-mutants ~ '
                                                                      '*/'}]}],
                                         'code': {'code': 'E0658',
                                                  'explanation': 'An unstable feature was used.\n'
                                                                 '\n'
                                                                 'Erroneous code example:\n'
                                                                 '\n'
                                                                 '```compile_fail,E0658\n'
                                                                 'use std::intrinsics; // error: use of '
                                                                 'unstable library feature '
                                                                 '`core_intrinsics`\n'
                                                                 '```\n'
                                                                 '\n'
                                                                 "If you're using a stable or a beta version "
                                                                 "of rustc, you won't be able to use\n"
                                                                 'any unstable features. In order to do so, '
                                                                 'please switch to a nightly version of\n'
                                                                 'rustc (by using [rustup]).\n'
                                                                 '\n'
                                                                 "If you're using a nightly version of "
                                                                 'rustc, just add the corresponding feature\n'
                                                                 'to be able to use it:\n'
                                                                 '\n'
                                                                 '```\n'
                                                                 '#![feature(core_intrinsics)]\n'
                                                                 '\n'
                                                                 'use std::intrinsics; // ok!\n'
                                                                 '```\n'
                                                                 '\n'
                                                                 '[rustup]: '
                                                                 'https://rust-lang.github.io/rustup/concepts/channels.html\n'}}},
                            {'reason': 'compiler-message',
                             'package_id': 'path+file:///work/tmp/cargo-mutants-source-847ijm.tmp/crates/time-core#0.0.0',
                             'manifest_path': '/work/tmp/cargo-mutants-source-847ijm.tmp/crates/time-core/Cargo.toml',
                             'target': {'kind': ['lib'],
                                        'crate_types': ['lib'],
                                        'name': 'time_core',
                                        'src_path': '/work/tmp/cargo-mutants-source-847ijm.tmp/crates/time-core/src/lib.rs',
                                        'edition': '2024',
                                        'doc': True,
                                        'doctest': True,
                                        'test': True},
                             'message': {'rendered': 'error: `TryInto` is not yet stable as a const trait\n'
                                                     '   --> '
                                                     'crates/time-core/src/reference_input/rate_delivery.rs:464:14\n'
                                                     '    |\n'
                                                     '464 |         Some(1.try_into().unwrap()) /* ~ changed '
                                                     'by cargo-mutants ~ */\n'
                                                     '    |              ^^^^^^^^^^^^\n'
                                                     '\n',
                                         '$message_type': 'diagnostic',
                                         'children': [],
                                         'level': 'error',
                                         'message': '`TryInto` is not yet stable as a const trait',
                                         'spans': [{'byte_end': 18464,
                                                    'byte_start': 18452,
                                                    'column_end': 26,
                                                    'column_start': 14,
                                                    'expansion': None,
                                                    'file_name': 'crates/time-core/src/reference_input/rate_delivery.rs',
                                                    'is_primary': True,
                                                    'label': None,
                                                    'line_end': 464,
                                                    'line_start': 464,
                                                    'suggested_replacement': None,
                                                    'suggestion_applicability': None,
                                                    'text': [{'highlight_end': 26,
                                                              'highlight_start': 14,
                                                              'text': '        Some(1.try_into().unwrap()) '
                                                                      '/* ~ changed by cargo-mutants ~ '
                                                                      '*/'}]}],
                                         'code': None}},
                            {'reason': 'compiler-message',
                             'package_id': 'path+file:///work/tmp/cargo-mutants-source-847ijm.tmp/crates/time-core#0.0.0',
                             'manifest_path': '/work/tmp/cargo-mutants-source-847ijm.tmp/crates/time-core/Cargo.toml',
                             'target': {'kind': ['lib'],
                                        'crate_types': ['lib'],
                                        'name': 'time_core',
                                        'src_path': '/work/tmp/cargo-mutants-source-847ijm.tmp/crates/time-core/src/lib.rs',
                                        'edition': '2024',
                                        'doc': True,
                                        'doctest': True,
                                        'test': True},
                             'message': {'rendered': 'error[E0015]: cannot call non-const method '
                                                     '`Result::<NonZero<u64>, TryFromIntError>::unwrap` in '
                                                     'constant functions\n'
                                                     '   --> '
                                                     'crates/time-core/src/reference_input/rate_delivery.rs:464:27\n'
                                                     '    |\n'
                                                     '464 |         Some(1.try_into().unwrap()) /* ~ changed '
                                                     'by cargo-mutants ~ */\n'
                                                     '    |                           ^^^^^^^^\n'
                                                     '    |\n'
                                                     '    = note: calls in constant functions are limited to '
                                                     'constant functions, tuple structs and tuple variants\n'
                                                     '\n',
                                         '$message_type': 'diagnostic',
                                         'children': [{'children': [],
                                                       'code': None,
                                                       'level': 'note',
                                                       'message': 'calls in constant functions are limited '
                                                                  'to constant functions, tuple structs and '
                                                                  'tuple variants',
                                                       'rendered': None,
                                                       'spans': []}],
                                         'level': 'error',
                                         'message': 'cannot call non-const method `Result::<NonZero<u64>, '
                                                    'TryFromIntError>::unwrap` in constant functions',
                                         'spans': [{'byte_end': 18473,
                                                    'byte_start': 18465,
                                                    'column_end': 35,
                                                    'column_start': 27,
                                                    'expansion': None,
                                                    'file_name': 'crates/time-core/src/reference_input/rate_delivery.rs',
                                                    'is_primary': True,
                                                    'label': None,
                                                    'line_end': 464,
                                                    'line_start': 464,
                                                    'suggested_replacement': None,
                                                    'suggestion_applicability': None,
                                                    'text': [{'highlight_end': 35,
                                                              'highlight_start': 27,
                                                              'text': '        Some(1.try_into().unwrap()) '
                                                                      '/* ~ changed by cargo-mutants ~ '
                                                                      '*/'}]}],
                                         'code': {'code': 'E0015',
                                                  'explanation': 'A non-`const` function was called in a '
                                                                 '`const` context.\n'
                                                                 '\n'
                                                                 'Erroneous code example:\n'
                                                                 '\n'
                                                                 '```compile_fail,E0015\n'
                                                                 'fn create_some() -> Option<u8> {\n'
                                                                 '    Some(1)\n'
                                                                 '}\n'
                                                                 '\n'
                                                                 '// error: cannot call non-const function '
                                                                 '`create_some` in constants\n'
                                                                 'const FOO: Option<u8> = create_some();\n'
                                                                 '```\n'
                                                                 '\n'
                                                                 'All functions used in a `const` context '
                                                                 '(constant or static expression) must\n'
                                                                 'be marked `const`.\n'
                                                                 '\n'
                                                                 'To fix this error, you can declare '
                                                                 '`create_some` as a constant function:\n'
                                                                 '\n'
                                                                 '```\n'
                                                                 '// declared as a `const` function:\n'
                                                                 'const fn create_some() -> Option<u8> {\n'
                                                                 '    Some(1)\n'
                                                                 '}\n'
                                                                 '\n'
                                                                 'const FOO: Option<u8> = create_some(); // '
                                                                 'no error!\n'
                                                                 '```\n'}}}]},
 'rate:546:9': {'mutant': {'name': 'crates/time-core/src/reference_input/rate_delivery.rs:546:9: replace '
                                   'FixtureRateSettlementCompletion::replacement -> '
                                   'Result<Option<&FixtureAffineChart>, FixtureChartError> with Ok(None)',
                           'package': 'time-core',
                           'file': 'crates/time-core/src/reference_input/rate_delivery.rs',
                           'function': {'function_name': 'FixtureRateSettlementCompletion::replacement',
                                        'return_type': '-> Result<Option<&FixtureAffineChart>, '
                                                       'FixtureChartError>',
                                        'span': {'start': {'line': 542, 'column': 5},
                                                 'end': {'line': 551, 'column': 6}}},
                           'span': {'start': {'line': 546, 'column': 9}, 'end': {'line': 550, 'column': 10}},
                           'replacement': 'Ok(None)',
                           'genre': 'FnValue'},
                'diff': '--- crates/time-core/src/reference_input/rate_delivery.rs\n'
                        '+++ replace FixtureRateSettlementCompletion::replacement -> '
                        'Result<Option<&FixtureAffineChart>, FixtureChartError> with Ok(None)\n'
                        '@@ -538,21 +538,17 @@\n'
                        '     pub const fn kind(&self) -> FixtureRateSettlementKind {\n'
                        '         self.kind\n'
                        '     }\n'
                        ' \n'
                        '     /// Borrow the replacement, explicit absence or original bounded failure.\n'
                        '     /// # Errors\n'
                        '     /// Preserve post-admission integration/bound/composition failure; no retry.\n'
                        '     pub const fn replacement(&self) -> Result<Option<&FixtureAffineChart>, '
                        'FixtureChartError> {\n'
                        '-        match &self.replacement {\n'
                        '-            Ok(Some(chart)) => Ok(Some(chart)),\n'
                        '-            Ok(None) => Ok(None),\n'
                        '-            Err(error) => Err(*error),\n'
                        '-        }\n'
                        '+        Ok(None) /* ~ changed by cargo-mutants ~ */\n'
                        '     }\n'
                        ' \n'
                        '     /// Consume this result and transfer the same checked replacement or failure.\n'
                        '     /// # Errors\n'
                        '     /// Preserve the original admitted failure without restoring either owner.\n'
                        '     pub fn into_replacement(self) -> Result<Option<FixtureAffineChart>, '
                        'FixtureChartError> {\n'
                        '         self.replacement\n'
                        '     }\n',
                'result_sections': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map',
                                     'body': 'Error: Invariant',
                                     'section_sha256': 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'}]},
 'rate:608:9': {'mutant': {'name': 'crates/time-core/src/reference_input/rate_delivery.rs:608:9: replace && '
                                   'with || in known_relationship',
                           'package': 'time-core',
                           'file': 'crates/time-core/src/reference_input/rate_delivery.rs',
                           'function': {'function_name': 'known_relationship',
                                        'return_type': '-> Result<(), FixtureRateDeliveryError>',
                                        'span': {'start': {'line': 588, 'column': 1},
                                                 'end': {'line': 618, 'column': 2}}},
                           'span': {'start': {'line': 608, 'column': 9}, 'end': {'line': 608, 'column': 11}},
                           'replacement': '||',
                           'genre': 'BinaryOperator'},
                'diff': '--- crates/time-core/src/reference_input/rate_delivery.rs\n'
                        '+++ replace && with || in known_relationship\n'
                        '@@ -600,17 +600,17 @@\n'
                        '     FixtureChartFrames::new(\n'
                        '         first.source_frame(),\n'
                        '         first.source_generation(),\n'
                        '         interval.frames.target_frame(),\n'
                        '         interval.frames.target_generation(),\n'
                        '     )\n'
                        '     .map_err(|source| FixtureRateDeliveryError::Chart { source })?;\n'
                        '     if let Some(last) = cursor.last_active\n'
                        '-        && interval.span.active.0 < last\n'
                        '+        || /* ~ changed by cargo-mutants ~ */ interval.span.active.0 < last\n'
                        '     {\n'
                        '         return Err(FixtureRateDeliveryError::ActiveOverlap);\n'
                        '     }\n'
                        '     if let Some(last) = cursor.last_monotonic\n'
                        '         && interval.span.effective.0 < last\n'
                        '     {\n'
                        '         return Err(FixtureRateDeliveryError::MonotonicOverlap);\n'
                        '     }\n',
                'records': [{'reason': 'compiler-message',
                             'package_id': 'path+file:///work/tmp/cargo-mutants-source-NWsXcH.tmp/crates/time-core#0.0.0',
                             'manifest_path': '/work/tmp/cargo-mutants-source-NWsXcH.tmp/crates/time-core/Cargo.toml',
                             'target': {'kind': ['lib'],
                                        'crate_types': ['lib'],
                                        'name': 'time_core',
                                        'src_path': '/work/tmp/cargo-mutants-source-NWsXcH.tmp/crates/time-core/src/lib.rs',
                                        'edition': '2024',
                                        'doc': True,
                                        'doctest': True,
                                        'test': True},
                             'message': {'rendered': 'error: `||` operators are not supported in let chain '
                                                     'conditions\n'
                                                     '   --> '
                                                     'crates/time-core/src/reference_input/rate_delivery.rs:608:9\n'
                                                     '    |\n'
                                                     '608 |         || /* ~ changed by cargo-mutants ~ */ '
                                                     'interval.span.active.0 < last\n'
                                                     '    |         ^^\n'
                                                     '\n',
                                         '$message_type': 'diagnostic',
                                         'children': [],
                                         'level': 'error',
                                         'message': '`||` operators are not supported in let chain '
                                                    'conditions',
                                         'spans': [{'byte_end': 24271,
                                                    'byte_start': 24269,
                                                    'column_end': 11,
                                                    'column_start': 9,
                                                    'expansion': None,
                                                    'file_name': 'crates/time-core/src/reference_input/rate_delivery.rs',
                                                    'is_primary': True,
                                                    'label': None,
                                                    'line_end': 608,
                                                    'line_start': 608,
                                                    'suggested_replacement': None,
                                                    'suggestion_applicability': None,
                                                    'text': [{'highlight_end': 11,
                                                              'highlight_start': 9,
                                                              'text': '        || /* ~ changed by '
                                                                      'cargo-mutants ~ */ '
                                                                      'interval.span.active.0 < last'}]}],
                                         'code': None}}]},
 'rate:613:9': {'mutant': {'name': 'crates/time-core/src/reference_input/rate_delivery.rs:613:9: replace && '
                                   'with || in known_relationship',
                           'package': 'time-core',
                           'file': 'crates/time-core/src/reference_input/rate_delivery.rs',
                           'function': {'function_name': 'known_relationship',
                                        'return_type': '-> Result<(), FixtureRateDeliveryError>',
                                        'span': {'start': {'line': 588, 'column': 1},
                                                 'end': {'line': 618, 'column': 2}}},
                           'span': {'start': {'line': 613, 'column': 9}, 'end': {'line': 613, 'column': 11}},
                           'replacement': '||',
                           'genre': 'BinaryOperator'},
                'diff': '--- crates/time-core/src/reference_input/rate_delivery.rs\n'
                        '+++ replace && with || in known_relationship\n'
                        '@@ -605,17 +605,17 @@\n'
                        '     )\n'
                        '     .map_err(|source| FixtureRateDeliveryError::Chart { source })?;\n'
                        '     if let Some(last) = cursor.last_active\n'
                        '         && interval.span.active.0 < last\n'
                        '     {\n'
                        '         return Err(FixtureRateDeliveryError::ActiveOverlap);\n'
                        '     }\n'
                        '     if let Some(last) = cursor.last_monotonic\n'
                        '-        && interval.span.effective.0 < last\n'
                        '+        || /* ~ changed by cargo-mutants ~ */ interval.span.effective.0 < last\n'
                        '     {\n'
                        '         return Err(FixtureRateDeliveryError::MonotonicOverlap);\n'
                        '     }\n'
                        '     Ok(())\n'
                        ' }\n'
                        ' \n'
                        ' /// Admit the whole live relationship and calculate checked nonwrapping progress.\n'
                        ' /// # Errors\n',
                'records': [{'reason': 'compiler-message',
                             'package_id': 'path+file:///work/tmp/cargo-mutants-source-a286Jz.tmp/crates/time-core#0.0.0',
                             'manifest_path': '/work/tmp/cargo-mutants-source-a286Jz.tmp/crates/time-core/Cargo.toml',
                             'target': {'kind': ['lib'],
                                        'crate_types': ['lib'],
                                        'name': 'time_core',
                                        'src_path': '/work/tmp/cargo-mutants-source-a286Jz.tmp/crates/time-core/src/lib.rs',
                                        'edition': '2024',
                                        'doc': True,
                                        'doctest': True,
                                        'test': True},
                             'message': {'rendered': 'error: `||` operators are not supported in let chain '
                                                     'conditions\n'
                                                     '   --> '
                                                     'crates/time-core/src/reference_input/rate_delivery.rs:613:9\n'
                                                     '    |\n'
                                                     '613 |         || /* ~ changed by cargo-mutants ~ */ '
                                                     'interval.span.effective.0 < last\n'
                                                     '    |         ^^\n'
                                                     '\n',
                                         '$message_type': 'diagnostic',
                                         'children': [],
                                         'level': 'error',
                                         'message': '`||` operators are not supported in let chain '
                                                    'conditions',
                                         'spans': [{'byte_end': 24431,
                                                    'byte_start': 24429,
                                                    'column_end': 11,
                                                    'column_start': 9,
                                                    'expansion': None,
                                                    'file_name': 'crates/time-core/src/reference_input/rate_delivery.rs',
                                                    'is_primary': True,
                                                    'label': None,
                                                    'line_end': 613,
                                                    'line_start': 613,
                                                    'suggested_replacement': None,
                                                    'suggestion_applicability': None,
                                                    'text': [{'highlight_end': 11,
                                                              'highlight_start': 9,
                                                              'text': '        || /* ~ changed by '
                                                                      'cargo-mutants ~ */ '
                                                                      'interval.span.effective.0 < last'}]}],
                                         'code': None}}]},
 'rate:800:40': {'mutant': {'name': 'crates/time-core/src/reference_input/rate_delivery.rs:800:40: replace '
                                    '!= with == in '
                                    "FixtureReferenceInputState<'_>::receive_baseline_settlement",
                            'package': 'time-core',
                            'file': 'crates/time-core/src/reference_input/rate_delivery.rs',
                            'function': {'function_name': "FixtureReferenceInputState<'_>::receive_baseline_settlement",
                                         'return_type': '-> Result<FixtureRateSettlementCompletion, '
                                                        'FixtureRateDeliveryError>',
                                         'span': {'start': {'line': 788, 'column': 5},
                                                  'end': {'line': 823, 'column': 6}}},
                            'span': {'start': {'line': 800, 'column': 40},
                                     'end': {'line': 800, 'column': 42}},
                            'replacement': '==',
                            'genre': 'BinaryOperator'},
                 'diff': '--- crates/time-core/src/reference_input/rate_delivery.rs\n'
                         '+++ replace != with == in '
                         "FixtureReferenceInputState<'_>::receive_baseline_settlement\n"
                         '@@ -792,17 +792,17 @@\n'
                         '     /// Reject the complete frame chain before existing current-state admission.\n'
                         '     fn receive_baseline_settlement(\n'
                         '         &mut self,\n'
                         '         cursor: &mut FixtureRateSettlementCursor,\n'
                         '         predecessor: &FixtureAffineChart,\n'
                         '         interval: FixtureSettledRateInterval,\n'
                         '         final_frames: FixtureChartFrames,\n'
                         '     ) -> Result<FixtureRateSettlementCompletion, FixtureRateDeliveryError> {\n'
                         '-        if final_frames.source_frame() != interval.frames.target_frame()\n'
                         '+        if final_frames.source_frame() == /* ~ changed by cargo-mutants ~ */ '
                         'interval.frames.target_frame()\n'
                         '             || final_frames.source_generation() != '
                         'interval.frames.target_generation()\n'
                         '         {\n'
                         '             return Err(FixtureRateDeliveryError::Chart {\n'
                         '                 source: FixtureChartError::CompositionMappingMismatch,\n'
                         '             });\n'
                         '         }\n'
                         '         let first = predecessor.frames();\n'
                         '         FixtureChartFrames::new(\n',
                 'result_sections': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts',
                                      'body': 'Error: Custom { kind: Other, error: "numeric failure cannot '
                                              'undo a Known step" }',
                                      'section_sha256': 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'}]},
 'rate:801:49': {'mutant': {'name': 'crates/time-core/src/reference_input/rate_delivery.rs:801:49: replace '
                                    '!= with == in '
                                    "FixtureReferenceInputState<'_>::receive_baseline_settlement",
                            'package': 'time-core',
                            'file': 'crates/time-core/src/reference_input/rate_delivery.rs',
                            'function': {'function_name': "FixtureReferenceInputState<'_>::receive_baseline_settlement",
                                         'return_type': '-> Result<FixtureRateSettlementCompletion, '
                                                        'FixtureRateDeliveryError>',
                                         'span': {'start': {'line': 788, 'column': 5},
                                                  'end': {'line': 823, 'column': 6}}},
                            'span': {'start': {'line': 801, 'column': 49},
                                     'end': {'line': 801, 'column': 51}},
                            'replacement': '==',
                            'genre': 'BinaryOperator'},
                 'diff': '--- crates/time-core/src/reference_input/rate_delivery.rs\n'
                         '+++ replace != with == in '
                         "FixtureReferenceInputState<'_>::receive_baseline_settlement\n"
                         '@@ -793,17 +793,17 @@\n'
                         '     fn receive_baseline_settlement(\n'
                         '         &mut self,\n'
                         '         cursor: &mut FixtureRateSettlementCursor,\n'
                         '         predecessor: &FixtureAffineChart,\n'
                         '         interval: FixtureSettledRateInterval,\n'
                         '         final_frames: FixtureChartFrames,\n'
                         '     ) -> Result<FixtureRateSettlementCompletion, FixtureRateDeliveryError> {\n'
                         '         if final_frames.source_frame() != interval.frames.target_frame()\n'
                         '-            || final_frames.source_generation() != '
                         'interval.frames.target_generation()\n'
                         '+            || final_frames.source_generation() == /* ~ changed by cargo-mutants '
                         '~ */ interval.frames.target_generation()\n'
                         '         {\n'
                         '             return Err(FixtureRateDeliveryError::Chart {\n'
                         '                 source: FixtureChartError::CompositionMappingMismatch,\n'
                         '             });\n'
                         '         }\n'
                         '         let first = predecessor.frames();\n'
                         '         FixtureChartFrames::new(\n'
                         '             first.source_frame(),\n',
                 'result_sections': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts',
                                      'body': 'Error: Custom { kind: Other, error: "numeric failure cannot '
                                              'undo a Known step" }',
                                      'section_sha256': 'aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa'}]}}

def synthetic_policy():
    """Explicit synthetic metadata, never current-product or native-execution facts."""
    return {
        **copy.deepcopy(MODULE.PHASE_STEP_FIXED),
        "source_file_count": 1, "candidate_sha256": "a" * 64,
        "inputs_sha256": {}, "mutation_inventories": {}, "mutation_equivalences": [],
        "test_inventories": {"baseline": [], "coverage": []},
        "scope": "Synthetic pure Python role-admission fixture; no product qualification.",
    }


def reviewed_maps(count=100):
    """Synthetic whole-file descriptors; no real generated variants are claimed."""
    return {owner: {"count": count, "sha256": "b" * 64} for owner in OWNERS}


def synthetic_rosters():
    """Construct minimum source-owned example names, not observed native identities."""
    native = sorted(ROLE.required_tests | {"synthetic_extra_ordinary_neighbor"})
    docs = []
    for filename, owner, negative, count in ROLE.required_doctests:
        docs.extend(f"{filename} - {owner} (line {line + 1})" + (" - compile fail" if negative else "")
                    for line in range(count))
    return {"baseline": sorted([*native, *docs]), "coverage": native}


def campaign_policy():
    """Synthetic complete prerequisites; no real campaign or passing result."""
    policy = synthetic_policy()
    policy.update(campaigns_admitted=True, mutation_inventories=reviewed_maps(),
                  test_inventories=synthetic_rosters(),
                  inputs_sha256={name: "c" * 64 for name in ROLE.bound_inputs})
    return policy


def arguments(lane, **changes):
    """Exact argument shape with a nonexistent output; execution is always mocked."""
    values = dict(subject="phase_step", lane=lane, tool_admission_profile="stable-core",
                  inventory_admission=None, cohort_plan=None, cohort_id=None, cohort_receipt=[],
                  vendor_dir=Path("/synthetic/vendor"), output_dir=Path("/synthetic/never-created"))
    return SimpleNamespace(**(values | changes))


def listing(names):
    """Literal native --list shape only; this is not a successful test transcript."""
    return "".join(name + ": test\n" for name in names) + f"\n{len(names)} tests, 0 benchmarks\n"


def literal_fixture(directory, data=None, *, filename="crates/time-core/src/lib.rs"):
    """Pinned literal-comment parser data, without any compiled Rust/source claim."""
    if data is None:
        data = (b"//! ```\n//! let healthy = 1;\n//! ```\n//!\n"
                b"/// ```compile_fail,E0451\n/// unavailable();\n/// ```\n")
    root = Path(directory)
    path = root / filename
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes(data)
    return root, {filename: hashlib.sha256(data).hexdigest()}


def bound_fixture(directory, *, pending=True):
    """Build an isolated fake source map for policy mechanics, never the checkout."""
    root = Path(directory)
    for name in ROLE.bound_inputs:
        path = root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text("// synthetic bound-input fixture; no executable Rust\n")
    manifest = root / "policy/verification-manifest.toml"
    manifest.write_text("critical_files = " + json.dumps(list(OWNERS.values())) + "\nsecurity_modules = [\n"
                        + "".join('{ package = "time-core", files = [' + json.dumps(name) + "] },\n"
                                  for name in OWNERS.values()) + "]\n")
    policy = synthetic_policy()
    identity = {"synthetic_product_fixture": "c" * 64}
    policy["candidate_sha256"] = MODULE.COMMON.canonical_sha256(identity)
    policy["inputs_sha256"] = {name: hashlib.sha256((root / name).read_bytes()).hexdigest()
                               for name in ROLE.bound_inputs}
    if not pending:
        policy["test_inventories"] = synthetic_rosters()
    path = root / MODULE.PHASE_STEP_POLICY
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(policy))
    return root, policy, identity


# Exact reduced failure sections/diffs from fourteen retained producer-reader
# refusals. Unit controls bind current reviewed test owners separately; their
# diagnostic literals are not current-candidate execution evidence.
_PHASE_STEP_PARENT_CAUSAL_SPECIMENS = {'899:50:==': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:899:50: replace < '
                       'with == in now_relationship',
               'guard_assertions': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map',
                                     'source': 'crates/time-core/src/reference_input/rate_delivery/../../../tests/private/step_counter.rs',
                                     'line': 179,
                                     'column': 23,
                                     'message': 'valid NoEffect fixture was refused: Some("fixture step: '
                                                'phase owner refused")'}]},
 '899:50:<=': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:899:50: replace < '
                       'with <= in now_relationship',
               'guard_assertions': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map',
                                     'source': 'crates/time-core/src/reference_input/rate_delivery/../../../tests/private/step_counter.rs',
                                     'line': 179,
                                     'column': 23,
                                     'message': 'valid NoEffect fixture was refused: Some("fixture step: '
                                                'phase owner refused")'}]},
 '909:53:==': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:909:53: replace < '
                       'with == in now_relationship',
               'guard_assertions': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map',
                                     'source': 'crates/time-core/src/reference_input/rate_delivery/../../../tests/private/step_counter.rs',
                                     'line': 179,
                                     'column': 23,
                                     'message': 'valid NoEffect fixture was refused: Some("fixture step: '
                                                'phase owner refused")'}]},
 '909:53:<=': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:909:53: replace < '
                       'with <= in now_relationship',
               'guard_assertions': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map',
                                     'source': 'crates/time-core/src/reference_input/rate_delivery/../../../tests/private/step_counter.rs',
                                     'line': 179,
                                     'column': 23,
                                     'message': 'valid NoEffect fixture was refused: Some("fixture step: '
                                                'phase owner refused")'}]},
 '1841:9:None': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:1841:9: replace '
                         "FixturePhaseRetirement<'producer>::cursor -> Option<&FixtureRateSettlementCursor> "
                         'with None',
                 'guard_assertions': [],
                 'mutant': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:1841:9: '
                                    "replace FixturePhaseRetirement<'producer>::cursor -> "
                                    'Option<&FixtureRateSettlementCursor> with None',
                            'package': 'time-core',
                            'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs',
                            'function': {'function_name': "FixturePhaseRetirement<'producer>::cursor",
                                         'return_type': '-> Option<&FixtureRateSettlementCursor>',
                                         'span': {'start': {'line': 1838, 'column': 5},
                                                  'end': {'line': 1842, 'column': 6}}},
                            'span': {'start': {'line': 1841, 'column': 9},
                                     'end': {'line': 1841, 'column': 54}},
                            'replacement': 'None',
                            'genre': 'FnValue'},
                 'diff': '--- crates/time-core/src/reference_input/rate_delivery/phase_plan.rs\n'
                         "+++ replace FixturePhaseRetirement<'producer>::cursor -> "
                         'Option<&FixtureRateSettlementCursor> with None\n'
                         '@@ -1833,17 +1833,17 @@\n'
                         '     /// Borrow the original typed known terminal event, including its delivered '
                         'span.\n'
                         '     #[must_use]\n'
                         '     pub const fn event(&self) -> &FixtureRateSettlement {\n'
                         '         &self.event\n'
                         '     }\n'
                         '     /// Borrow the real cursor only until the take-once transfer consumes it.\n'
                         '     #[must_use]\n'
                         '     pub fn cursor(&self) -> Option<&FixtureRateSettlementCursor> {\n'
                         '-        self.continuation.as_ref().map(|v| &v.cursor)\n'
                         '+        None /* ~ changed by cargo-mutants ~ */\n'
                         '     }\n'
                         '     /// Historical original source metadata survives a successful transfer.\n'
                         '     #[must_use]\n'
                         '     pub const fn provenance(&self) -> Option<&FixturePhaseProvenance> {\n'
                         '         self.provenance.as_ref()\n'
                         '     }\n'
                         '     /// Copy the historical accounted remainder, even if policy prohibits its '
                         'use.\n'
                         '     #[must_use]\n',
                 'result_sections': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts',
                                      'body': 'Error: Custom { kind: Other, error: "lost committed cursor" }',
                                      'section_sha256': '96594ec657989e81b395b90c9891ce7c89f4d51312e6be72311f369a577b031a'}]},
 '1846:9:None': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:1846:9: replace '
                         "FixturePhaseRetirement<'producer>::provenance -> Option<&FixturePhaseProvenance> "
                         'with None',
                 'guard_assertions': [],
                 'mutant': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:1846:9: '
                                    "replace FixturePhaseRetirement<'producer>::provenance -> "
                                    'Option<&FixturePhaseProvenance> with None',
                            'package': 'time-core',
                            'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs',
                            'function': {'function_name': "FixturePhaseRetirement<'producer>::provenance",
                                         'return_type': '-> Option<&FixturePhaseProvenance>',
                                         'span': {'start': {'line': 1843, 'column': 5},
                                                  'end': {'line': 1847, 'column': 6}}},
                            'span': {'start': {'line': 1846, 'column': 9},
                                     'end': {'line': 1846, 'column': 33}},
                            'replacement': 'None',
                            'genre': 'FnValue'},
                 'diff': '--- crates/time-core/src/reference_input/rate_delivery/phase_plan.rs\n'
                         "+++ replace FixturePhaseRetirement<'producer>::provenance -> "
                         'Option<&FixturePhaseProvenance> with None\n'
                         '@@ -1838,17 +1838,17 @@\n'
                         '     /// Borrow the real cursor only until the take-once transfer consumes it.\n'
                         '     #[must_use]\n'
                         '     pub fn cursor(&self) -> Option<&FixtureRateSettlementCursor> {\n'
                         '         self.continuation.as_ref().map(|v| &v.cursor)\n'
                         '     }\n'
                         '     /// Historical original source metadata survives a successful transfer.\n'
                         '     #[must_use]\n'
                         '     pub const fn provenance(&self) -> Option<&FixturePhaseProvenance> {\n'
                         '-        self.provenance.as_ref()\n'
                         '+        None /* ~ changed by cargo-mutants ~ */\n'
                         '     }\n'
                         '     /// Copy the historical accounted remainder, even if policy prohibits its '
                         'use.\n'
                         '     #[must_use]\n'
                         '     pub fn remaining_phase_seconds(&self) -> Option<f64> {\n'
                         '         self.debt.as_ref().ok().map(PhaseDebt::point)\n'
                         '     }\n'
                         '     /// Copy the separately retained historical numerical bound.\n'
                         '     #[must_use]\n',
                 'result_sections': [{'test': 'step_diagnostics::matched_step_retirement_diagnostics_preserve_provenance_after_sole_transfer',
                                      'body': 'Error: Custom { kind: Other, error: "retirement lost its '
                                              'original source metadata" }',
                                      'section_sha256': 'b534431700ca0977625293205c1ecbb5c8e62f68b72e3d7316e97429df280cb9'}]},
 '1919:34:==': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:1919:34: replace '
                        "!= with == in FixturePhaseRetirement<'producer>::admit_transfer",
                'guard_assertions': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map',
                                      'source': 'crates/time-core/src/reference_input/rate_delivery/../../../tests/private/step_counter.rs',
                                      'line': 179,
                                      'column': 23,
                                      'message': 'valid NoEffect fixture was refused: Some("fixture step: '
                                                 'phase owner refused")'}]},
 '2033:46:<': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:2033:46: replace > '
                       "with < in FixturePhaseRetirement<'producer>::resume_fresh",
               'guard_assertions': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map',
                                     'source': 'crates/time-core/src/reference_input/rate_delivery/../../../tests/private/step_counter.rs',
                                     'line': 179,
                                     'column': 23,
                                     'message': 'valid NoEffect fixture was refused: Some("fixture step: '
                                                'phase owner refused")'}]},
 '2193:17:==': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:2193:17: replace '
                        '!= with == in ControllerRatioWitness::admits',
                'guard_assertions': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map',
                                      'source': 'crates/time-core/src/reference_input/rate_delivery/../../../tests/private/step_counter.rs',
                                      'line': 124,
                                      'column': 5,
                                      'message': 'valid NoEffect fixture was refused: Some("fixture '
                                                 'baseline: clock map refused")'}]},
 '3111:47:<': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:3111:47: replace >= '
                       "with < in FixturePhasePlanOwner<'producer, 'chart>::settle_baseline",
               'guard_assertions': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts',
                                     'source': 'crates/time-core/src/reference_input/rate_delivery/../../../tests/private/step_counter.rs',
                                     'line': 195,
                                     'column': 5,
                                     'message': 'valid NoEffect fixture was refused: Some("fixture step: '
                                                'phase owner refused")'},
                                    {'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map',
                                     'source': 'crates/time-core/src/reference_input/rate_delivery/../../../tests/private/step_counter.rs',
                                     'line': 158,
                                     'column': 18,
                                     'message': 'valid NoEffect fixture was refused: Some("fixture step: '
                                                'phase owner refused")'}]},
 '3112:42:<': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:3112:42: replace >= '
                       "with < in FixturePhasePlanOwner<'producer, 'chart>::settle_baseline",
               'guard_assertions': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts',
                                     'source': 'crates/time-core/src/reference_input/rate_delivery/../../../tests/private/step_counter.rs',
                                     'line': 195,
                                     'column': 5,
                                     'message': 'valid NoEffect fixture was refused: Some("fixture step: '
                                                'phase owner refused")'},
                                    {'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map',
                                     'source': 'crates/time-core/src/reference_input/rate_delivery/../../../tests/private/step_counter.rs',
                                     'line': 158,
                                     'column': 18,
                                     'message': 'valid NoEffect fixture was refused: Some("fixture step: '
                                                'phase owner refused")'}]},
 '3154:20:false': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:3154:20: '
                           'replace match guard now.monotonic <= issued.map.valid_until with false in '
                           "FixturePhasePlanOwner<'producer, 'chart>::baseline_terminal_relation",
                   'guard_assertions': [],
                   'mutant': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:3154:20: '
                                      'replace match guard now.monotonic <= issued.map.valid_until with '
                                      "false in FixturePhasePlanOwner<'producer, "
                                      "'chart>::baseline_terminal_relation",
                              'package': 'time-core',
                              'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs',
                              'function': {'function_name': "FixturePhasePlanOwner<'producer, "
                                                            "'chart>::baseline_terminal_relation",
                                           'return_type': '-> Result<Option<ControllerClockRelation>, '
                                                          'FixtureBaselineError>',
                                           'span': {'start': {'line': 3133, 'column': 5},
                                                    'end': {'line': 3170, 'column': 6}}},
                              'span': {'start': {'line': 3154, 'column': 20},
                                       'end': {'line': 3154, 'column': 59}},
                              'replacement': 'false',
                              'genre': 'MatchArmGuard'},
                   'diff': '--- crates/time-core/src/reference_input/rate_delivery/phase_plan.rs\n'
                           '+++ replace match guard now.monotonic <= issued.map.valid_until with false in '
                           "FixturePhasePlanOwner<'producer, 'chart>::baseline_terminal_relation\n"
                           '@@ -3146,17 +3146,17 @@\n'
                           '             {\n'
                           '                 Some(FixturePhaseNow::new(\n'
                           '                     receipt.delivery.context,\n'
                           '                     receipt.delivery.span.active.1,\n'
                           '                     receipt.delivery.span.effective.1,\n'
                           '                 ))\n'
                           '             }\n'
                           '             FixtureBaselineFeedback::ProvenNoEffect { .. }\n'
                           '-                if now.monotonic <= issued.map.valid_until =>\n'
                           '+                if false /* ~ changed by cargo-mutants ~ */ =>\n'
                           '             {\n'
                           '                 Some(now)\n'
                           '             }\n'
                           '             _ => None,\n'
                           '         };\n'
                           '         match (self.clock_relation, observed) {\n'
                           '             (Some(relation), Some(now)) => relation\n'
                           '                 .observe(\n',
                   'result_sections': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts',
                                        'body': 'Error: Custom { kind: Other, error: "missing actual counter '
                                                'witness" }',
                                        'section_sha256': '511cb2fd26bf914f32c276a217c188c71b8311877595c37bce59449672fb7ab3'},
                                       {'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map',
                                        'body': 'Error: Custom { kind: Other, error: "missing actual counter '
                                                'witness" }',
                                        'section_sha256': '031e139e20f2c640cec290e4269073b027028481faa379836c75e9883d8cd486'}]},
 '3154:34:>': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:3154:34: replace <= '
                       "with > in FixturePhasePlanOwner<'producer, 'chart>::baseline_terminal_relation",
               'guard_assertions': [],
               'mutant': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:3154:34: '
                                  "replace <= with > in FixturePhasePlanOwner<'producer, "
                                  "'chart>::baseline_terminal_relation",
                          'package': 'time-core',
                          'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs',
                          'function': {'function_name': "FixturePhasePlanOwner<'producer, "
                                                        "'chart>::baseline_terminal_relation",
                                       'return_type': '-> Result<Option<ControllerClockRelation>, '
                                                      'FixtureBaselineError>',
                                       'span': {'start': {'line': 3133, 'column': 5},
                                                'end': {'line': 3170, 'column': 6}}},
                          'span': {'start': {'line': 3154, 'column': 34},
                                   'end': {'line': 3154, 'column': 36}},
                          'replacement': '>',
                          'genre': 'BinaryOperator'},
               'diff': '--- crates/time-core/src/reference_input/rate_delivery/phase_plan.rs\n'
                       "+++ replace <= with > in FixturePhasePlanOwner<'producer, "
                       "'chart>::baseline_terminal_relation\n"
                       '@@ -3146,17 +3146,17 @@\n'
                       '             {\n'
                       '                 Some(FixturePhaseNow::new(\n'
                       '                     receipt.delivery.context,\n'
                       '                     receipt.delivery.span.active.1,\n'
                       '                     receipt.delivery.span.effective.1,\n'
                       '                 ))\n'
                       '             }\n'
                       '             FixtureBaselineFeedback::ProvenNoEffect { .. }\n'
                       '-                if now.monotonic <= issued.map.valid_until =>\n'
                       '+                if now.monotonic > /* ~ changed by cargo-mutants ~ */ '
                       'issued.map.valid_until =>\n'
                       '             {\n'
                       '                 Some(now)\n'
                       '             }\n'
                       '             _ => None,\n'
                       '         };\n'
                       '         match (self.clock_relation, observed) {\n'
                       '             (Some(relation), Some(now)) => relation\n'
                       '                 .observe(\n',
               'result_sections': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts',
                                    'body': 'Error: Custom { kind: Other, error: "missing actual counter '
                                            'witness" }',
                                    'section_sha256': '511cb2fd26bf914f32c276a217c188c71b8311877595c37bce59449672fb7ab3'},
                                   {'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map',
                                    'body': 'Error: Custom { kind: Other, error: "missing actual counter '
                                            'witness" }',
                                    'section_sha256': '031e139e20f2c640cec290e4269073b027028481faa379836c75e9883d8cd486'}]},
 '3161:13:': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:3161:13: delete '
                      "match arm (Some(relation), Some(now)) in FixturePhasePlanOwner<'producer, "
                      "'chart>::baseline_terminal_relation",
              'guard_assertions': [],
              'mutant': {'name': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs:3161:13: '
                                 'delete match arm (Some(relation), Some(now)) in '
                                 "FixturePhasePlanOwner<'producer, 'chart>::baseline_terminal_relation",
                         'package': 'time-core',
                         'file': 'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs',
                         'function': {'function_name': "FixturePhasePlanOwner<'producer, "
                                                       "'chart>::baseline_terminal_relation",
                                      'return_type': '-> Result<Option<ControllerClockRelation>, '
                                                     'FixtureBaselineError>',
                                      'span': {'start': {'line': 3133, 'column': 5},
                                               'end': {'line': 3170, 'column': 6}}},
                         'span': {'start': {'line': 3161, 'column': 13}, 'end': {'line': 3167, 'column': 28}},
                         'replacement': '',
                         'genre': 'MatchArm'},
              'diff': '--- crates/time-core/src/reference_input/rate_delivery/phase_plan.rs\n'
                      "+++ delete match arm (Some(relation), Some(now)) in FixturePhasePlanOwner<'producer, "
                      "'chart>::baseline_terminal_relation\n"
                      '@@ -3153,23 +3153,17 @@\n'
                      '             FixtureBaselineFeedback::ProvenNoEffect { .. }\n'
                      '                 if now.monotonic <= issued.map.valid_until =>\n'
                      '             {\n'
                      '                 Some(now)\n'
                      '             }\n'
                      '             _ => None,\n'
                      '         };\n'
                      '         match (self.clock_relation, observed) {\n'
                      '-            (Some(relation), Some(now)) => relation\n'
                      '-                .observe(\n'
                      '-                    now,\n'
                      '-                    issued.map.numbers.ratio(),\n'
                      '-                    issued.map.numbers.ratio_error(),\n'
                      '-                )\n'
                      '-                .map(Some),\n'
                      '+             /* ~ changed by cargo-mutants ~ */\n'
                      '             _ => Ok(self.clock_relation),\n'
                      '         }\n'
                      '     }\n'
                      ' \n'
                      '     /// Finish post-consumption arithmetic; no outer rejection or refund is '
                      'possible.\n'
                      '     fn finish_baseline(\n'
                      '         mut self,\n'
                      '         predecessor: &FixtureAffineChart,\n',
              'result_sections': [{'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts',
                                   'body': 'Error: Custom { kind: Other, error: "missing actual counter '
                                           'witness" }',
                                   'section_sha256': '511cb2fd26bf914f32c276a217c188c71b8311877595c37bce59449672fb7ab3'},
                                  {'test': 'reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map',
                                   'body': 'Error: Custom { kind: Other, error: "missing actual counter '
                                           'witness" }',
                                   'section_sha256': '031e139e20f2c640cec290e4269073b027028481faa379836c75e9883d8cd486'}]}}

class PhaseStepAnalysisControls(unittest.TestCase):
    def test_new_exact_seven_owner_namespace_and_legacy_neighbors(self):
        self.assertEqual(tuple(OWNERS), ("phase_plan", "phase_numeric", "step", "rate_delivery", "reference_input", "adaptive_horizon", "frequency_numeric"))
        self.assertEqual(ROLE.scope.subjects, MODULE.COMMON.PHASE_STEP_SUBJECTS)
        self.assertEqual(ROLE.scope.policy, "policy/phase-step-analysis.json")
        self.assertEqual(len(set(OWNERS.values())), 7)
        self.assertEqual(len(MODULE.PHASE_PLAN_SUBJECTS), 2)
        self.assertEqual(len(MODULE.FREQUENCY_PRIOR_SUBJECTS), 4)
        self.assertEqual(len(MODULE.BASELINE_FEEDBACK_SUBJECTS), 5)
        self.assertNotIn(OWNERS["step"], MODULE.PHASE_PLAN_SUBJECTS.values())
        self.assertNotIn(OWNERS["phase_numeric"], MODULE.BASELINE_FEEDBACK_SUBJECTS.values())

    def test_complete_frequency_numeric_owner_requires_actual_clock_and_raw_controls(self):
        numeric = "crates/time-core/src/regression/chart/roundoff/frequency_prior.rs"
        self.assertEqual(OWNERS["frequency_numeric"], numeric)
        self.assertEqual(MODULE.BASELINE_FEEDBACK_SUBJECTS["numeric"], numeric)
        for name in (numeric, "crates/time-core/tests/frequency_prior.rs",
                     "crates/time-core/tests/private/frequency_prior_numeric.rs"):
            self.assertIn(name, ROLE.bound_inputs)
        names = {
            "active_ratio_span_signature_preserves_distinct_clock_duration_types",
            "active_ratio_span_checks_nonunit_zero_and_exact_integer_boundaries",
            "raw_active_ratio_validation_requires_supported_positive_enclosure",
            "raw_affine_effect_validation_checks_each_point_and_bound",
            "raw_baseline_change_validation_checks_both_achieved_points_and_ratio",
            "raw_baseline_map_validation_checks_each_coefficient_and_bound",
            "raw_baseline_request_validation_checks_fields_and_exact_clipping",
            "raw_rate_shift_validation_checks_point_and_error_independently",
        }
        prefix = "regression::chart::roundoff::frequency_prior::tests::"
        self.assertLessEqual({prefix + name for name in names}, ROLE.required_tests)
        source = (ROOT / "crates/time-core/tests/private/frequency_prior_numeric.rs").read_text()
        for name in names:
            with self.subTest(control=name):
                self.assertIn("fn " + name + "(", source)

    def test_old_six_owner_policy_and_scope_cannot_admit_changed_role(self):
        policy = synthetic_policy()
        policy["critical_files"] = policy["critical_files"][:-1]
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.require_phase_step_role(ROLE.scope, policy)
        policy = synthetic_policy()
        old_scope = MODULE.COMMON.Scope("time-core", ROLE.scope.subjects[:-1], ROLE.scope.policy)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.require_phase_step_role(old_scope, policy)

    def test_complete_adaptive_owner_has_moved_tests_doctests_and_private_input(self):
        adaptive = "crates/time-core/src/reference_input/rate_delivery/phase_plan/adaptive.rs"
        self.assertEqual(OWNERS["adaptive_horizon"], adaptive)
        for name in (adaptive, "crates/time-core/tests/private/adaptive_horizon.rs"):
            self.assertIn(name, ROLE.bound_inputs)
        names = {
            "adaptive_duration_ceiling_preserves_fractional_nanoseconds_at_large_seconds",
            "adaptive_policy_uses_exact_selected_horizon_for_its_existing_rate_bound",
            "adaptive_neutral_debt_preserves_baseline_without_overflowing_area_division",
            "adaptive_clamps_keep_original_integer_limits_and_refuse_nonzero_underflow",
            "adaptive_near_minimum_quantizes_before_integer_clamping",
            "adaptive_small_debt_threshold_equality_and_adjacent_bits_are_distinct",
        }
        prefix = "reference_input::rate_delivery::phase_plan::adaptive::tests::"
        self.assertTrue({prefix + name for name in names}.issubset(ROLE.required_tests))
        old_prefix = "reference_input::rate_delivery::phase_plan::tests::"
        self.assertTrue({old_prefix + name for name in names}.isdisjoint(ROLE.required_tests))
        owner = "reference_input::rate_delivery::phase_plan::adaptive::FixtureAdaptiveHorizonProfile"
        self.assertIn((adaptive, owner, False, 1), ROLE.required_doctests)
        self.assertIn((adaptive, owner, True, 2), ROLE.required_doctests)
        self.assertNotIn(adaptive, MODULE.PHASE_PLAN_SUBJECTS.values())
        self.assertNotIn(adaptive, MODULE.FREQUENCY_PRIOR_SUBJECTS.values())
        self.assertNotIn(adaptive, MODULE.BASELINE_FEEDBACK_SUBJECTS.values())

    def test_new_step_controls_require_real_declarations_and_cannot_be_omitted(self):
        controls = {
            "step_contracts": (
                "matched_step_original_phase_cap_is_inclusive_at_request_and_retirement",
                "matched_step_limit_and_original_cap_remain_independent",
                "matched_step_aggregate_baseline_cap_is_inclusive_for_both_signs",
                "matched_step_bidirectional_permission_still_refuses_opposite_actual_sign",
                "matched_step_zero_full_and_overshoot_are_distinct_for_both_signs",
                "matched_step_each_original_clock_deadline_refuses_new_issuance",
            ),
            "step_qualification": ("matched_step_error_getters_retain_positive_bounded_numerical_evidence",),
            "step_counter": (
                "step_no_effect_latches_each_original_deadline_before_later_admission",
                "known_step_preserves_negative_zero_rate_in_actual_replacement_geometry",
                "known_step_frequency_history_remains_historical_after_fresh_owner_is_dropped",
            ),
        }
        policy = campaign_policy()
        MODULE._conditioned_test_inventories(policy, ROLE)
        count = 0
        for module, names in controls.items():
            source = (ROOT / f"crates/time-core/tests/private/{module}.rs").read_text()
            prefix = ("reference_input::rate_delivery::phase_plan::tests::step_counter::"
                if module == "step_counter" else module + "::")
            for name in names:
                count += 1
                identifier = prefix + name
                with self.subTest(identifier=identifier):
                    self.assertIn(identifier, ROLE.required_tests)
                    self.assertEqual(len(re.findall(r"(?m)^#\[test\]\s*fn " + re.escape(name) + r"\s*\(", source)), 1)
                    changed = copy.deepcopy(policy)
                    for roster in changed["test_inventories"].values():
                        roster.remove(identifier)
                    with self.assertRaisesRegex(MODULE.Incomplete, "omit selected ordinary/private controls"):
                        MODULE._conditioned_test_inventories(changed, ROLE)
        self.assertEqual(count, 10)

    def test_artifact_equivalence_guard_uses_only_its_exact_role_validator(self):
        roles = (
            (ROLE, "phase_step_equivalences_valid"),
            (MODULE._baseline_feedback_role(), "baseline_feedback_equivalences_valid"),
            (MODULE._frequency_prior_role(), "frequency_prior_equivalences_valid"),
        )
        names = [name for _, name in roles]
        policy = {"mutation_equivalences": []}
        for role, selected in roles:
            for accepted in (False, True):
                with self.subTest(role=selected, accepted=accepted), ExitStack() as stack:
                    validators = {name: stack.enter_context(patch.object(MODULE.COMMON, name,
                        return_value=accepted if name == selected else False, create=True)) for name in names}
                    reached = stack.enter_context(patch.object(MODULE.PRODUCT, "inventory",
                        side_effect=RuntimeError("ordinary source admission reached")))
                    error, message = (RuntimeError, "ordinary source admission reached") if accepted else (
                        MODULE.Incomplete, "unreviewed equivalence scope")
                    with self.assertRaisesRegex(error, message):
                        MODULE.frequency_artifact_manifest(Path("/synthetic/work"), {}, policy,
                            "step", {}, {}, {}, role=role)
                    validators[selected].assert_called_once_with(policy)
                    for name in names:
                        if name != selected:
                            validators[name].assert_not_called()
                    self.assertEqual(reached.call_count, int(accepted))

    def test_fixed_limits_admit_exact_contracts_and_keep_campaigns_closed(self):
        policy = synthetic_policy()
        MODULE.COMMON.require_phase_step_role(ROLE.scope, policy)
        expected = {"coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
                    "jobs": 6, "mutation_jobs": 10, "test_threads": 1, "coverage_test_threads": 1,
                    "maximum_mutants": 512, "maximum_listed_per_file": 512, "maximum_listed_total": 2048,
                    "mutation_process_limit": 512, "mutation_build_seconds": 90, "mutation_test_seconds": 60,
                    "mutation_wall_seconds": 1200, "contracts_admitted": True, "campaigns_admitted": False}
        for field, value in expected.items():
            self.assertEqual(policy[field], value)
            with self.subTest(field=field):
                changed = copy.deepcopy(policy)
                changed[field] = not value if type(value) is bool else value + 1
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.require_phase_step_role(ROLE.scope, changed)
        for field, value in (("maximum_mutants", True), ("critical_files", list(reversed(policy["critical_files"]))),
                             ("tool_admission_profile", "full"), ("mutation_equivalences", [{}]),
                             ("contracts_admitted", 1),
                             ("external_consumer_limits", {"legal_executions": 3.0, "negative_compilations": 32}),
                             ("external_consumer_limits", {"legal_executions": 3, "negative_compilations": 32.0}),
                             ("external_consumer_limits", {"legal_executions": True, "negative_compilations": 32})):
            with self.subTest(field=field):
                changed = copy.deepcopy(policy)
                changed[field] = value
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.require_phase_step_role(ROLE.scope, changed)

    def test_foreign_policy_scope_and_unknown_fields_fail_closed(self):
        policy = synthetic_policy()
        for scope in (MODULE.FREQUENCY_PRIOR_SCOPE, MODULE.BASELINE_FEEDBACK_SCOPE,
                      MODULE.COMMON.Scope("time-core", ROLE.scope.subjects, "policy/time-core-analysis.json"),
                      MODULE.COMMON.Scope("time-core", ROLE.scope.subjects[:-1], ROLE.scope.policy)):
            with self.subTest(scope=scope):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.require_phase_step_role(scope, policy)
        policy["arbitrary_file_filter"] = "only-new-functions"
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.require_phase_step_role(ROLE.scope, policy)

    def test_initial_pending_policy_rejects_before_product_or_session_admission(self):
        pending = synthetic_policy()
        pending.update(source_file_count=0, candidate_sha256=None)
        with patch.object(MODULE, "inventory_document", return_value=pending), patch.object(MODULE.PRODUCT, "admit_product") as product, patch.object(MODULE.COMMON, "Session") as session:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.read_phase_step_policy(ROOT, allow_pending_rosters=True)
        product.assert_not_called()
        session.assert_not_called()

    def test_bound_inventory_can_discover_pending_rosters_but_baseline_cannot(self):
        with tempfile.TemporaryDirectory() as directory:
            root, policy, identity = bound_fixture(directory)
            with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
                self.assertEqual(MODULE.read_phase_step_policy(root, allow_pending_rosters=True), policy)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.read_phase_step_policy(root)
            changed = root / OWNERS["step"]
            changed.write_text("// changed after synthetic freeze\n")
            with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.read_phase_step_policy(root, allow_pending_rosters=True)

    def test_complete_rosters_allow_fresh_bound_noncampaign_stages(self):
        with tempfile.TemporaryDirectory() as directory:
            root, policy, identity = bound_fixture(directory, pending=False)
            with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
                self.assertEqual(MODULE.read_phase_step_policy(root), policy)
            changed = copy.deepcopy(policy)
            changed["test_inventories"]["coverage"].remove("synthetic_extra_ordinary_neighbor")
            with self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_test_inventories(changed, ROLE)

    def test_pending_roster_exception_never_changes_legacy_roles(self):
        with patch.object(MODULE, "inventory_document", return_value=synthetic_policy()):
            with self.assertRaises(MODULE.Incomplete):
                MODULE._read_conditioned_policy(ROOT, MODULE._phase_plan_role(), allow_pending_rosters=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE._read_conditioned_policy(ROOT, ROLE, allow_pending_rosters=1)

    def test_campaign_and_union_refuse_before_any_session(self):
        for lane in ("mutation", "all", "cohort-union"):
            with self.subTest(lane=lane), patch.object(MODULE, "read_phase_step_policy", return_value=synthetic_policy()) as read, patch.object(MODULE.COMMON, "Session") as session:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.execute_phase_step(arguments(lane), {})
                if lane == "all": read.assert_not_called()
                session.assert_not_called()

    def test_direct_union_cannot_bypass_source_closed_admission_with_forged_policy(self):
        for admitted in (False, True, 1, "true", None):
            policy = synthetic_policy()
            policy["campaigns_admitted"] = admitted
            with self.subTest(admitted=admitted), patch.object(MODULE, "frequency_inventory_bindings") as inventories, patch.object(MODULE.COMMON, "_require_private_output") as output:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.frequency_cohort_union(arguments("cohort-union"), {}, policy, ROLE)
            inventories.assert_not_called()
            output.assert_not_called()

    def test_true_campaign_flag_requires_all_complete_prerequisites(self):
        healthy = campaign_policy()
        MODULE.COMMON.require_phase_step_role(ROLE.scope, healthy)
        MODULE._phase_step_campaign_admission(healthy)
        variants = []
        for key, value in (("campaigns_admitted", 1), ("source_file_count", True),
                           ("candidate_sha256", "bad"), ("inputs_sha256", {}),
                           ("mutation_inventories", {}),
                           ("test_inventories", {"baseline": [], "coverage": []})):
            changed = copy.deepcopy(healthy)
            changed[key] = value
            variants.append(changed)
        for field, value in (("count", 513), ("count", True), ("sha256", "bad"), ("extra", 1)):
            changed = copy.deepcopy(healthy)
            changed["mutation_inventories"]["step"][field] = value
            variants.append(changed)
        missing = copy.deepcopy(healthy)
        missing["mutation_inventories"].pop("adaptive_horizon")
        variants.append(missing)
        wrong_roster = copy.deepcopy(healthy)
        wrong_roster["test_inventories"]["coverage"].remove("synthetic_extra_ordinary_neighbor")
        variants.append(wrong_roster)
        for policy in variants:
            with self.subTest(policy=policy), patch.object(MODULE.COMMON, "Session") as session:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE._phase_step_campaign_admission(policy)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.mutation_campaign_command(ROLE.scope, Path("/synthetic/tools"), "step", [],
                        expected_jobs=10, expected_message_format="json", policy=policy)
                session.assert_not_called()

    def test_current_source_binding_is_still_required_when_campaigns_are_admitted(self):
        with tempfile.TemporaryDirectory() as directory:
            root, policy, identity = bound_fixture(directory, pending=False)
            policy.update(campaigns_admitted=True, mutation_jobs=8, mutation_inventories=reviewed_maps())
            path = root / MODULE.PHASE_STEP_POLICY
            path.write_text(json.dumps(policy))
            with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)):
                self.assertEqual(MODULE.read_phase_step_policy(root), policy)
                (root / "scripts/core-frequency-contracts.py").write_text("// drift\n")
                with self.assertRaisesRegex(MODULE.Incomplete, "bindings changed"):
                    MODULE.read_phase_step_policy(root)

    def test_mutation_requires_an_explicit_exact_whole_owner(self):
        for cohort in (None, "all", "step::new", "foreign", True):
            with self.subTest(cohort=cohort), patch.object(MODULE, "read_phase_step_policy", return_value=campaign_policy()), patch.object(MODULE.COMMON, "Session") as session:
                with self.assertRaisesRegex(MODULE.Incomplete, "explicit complete named-owner"):
                    MODULE.execute_phase_step(arguments("mutation", cohort_id=cohort), {})
                session.assert_not_called()

    def test_all_seven_named_cohorts_use_the_existing_campaign_and_artifact_chain(self):
        policy = campaign_policy()
        policy["mutation_jobs"] = 8
        for owner, filename in OWNERS.items():
            with self.subTest(owner=owner), ExitStack() as stack:
                session = Mock(work=Path("/synthetic/work"), tools=Path("/synthetic/tools"))
                report = {}
                def run(*args):
                    report["mutation_enumeration"] = {owner: {"raw_output": "synthetic"}}
                stack.enter_context(patch.object(MODULE, "read_phase_step_policy", return_value=policy))
                stack.enter_context(patch.object(MODULE.COMMON, "Session", return_value=session))
                stack.enter_context(patch.object(MODULE.COMMON, "_require_private_output"))
                stack.enter_context(patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=6 * 1024 ** 3)))
                stack.enter_context(patch.object(MODULE, "_frequency_captured"))
                def configure(*args): report["mutation_debug_configuration"] = {}
                stack.enter_context(patch.object(MODULE, "configure_phase_plan_mutation", side_effect=configure))
                campaign = stack.enter_context(patch.object(MODULE.COMMON, "run_analysis_lanes", side_effect=run))
                listing_parser = stack.enter_context(patch.object(MODULE.COMMON, "parse_mutation_listing", return_value=([], None)))
                stack.enter_context(patch.object(MODULE.COMMON, "mutation_inventory", return_value={}))
                artifacts = stack.enter_context(patch.object(MODULE, "frequency_artifact_manifest", return_value={"synthetic": True}))
                MODULE.execute_phase_step(arguments("mutation", cohort_id=owner), report)
                self.assertEqual(campaign.call_args.args[1], {owner: filename})
                self.assertEqual(campaign.call_args.args[-1], "mutation")
                self.assertEqual(artifacts.call_args.args[3], owner)
                listing_parser.assert_called_once_with("synthetic", expected_jobs=8)
                self.assertTrue(report["cohort_complete"])
                self.assertFalse(report["qualification_passed"])
                self.assertFalse(report["mutation_union_passed"])
                self.assertFalse(report["full_role_qualification"])
                session.finish.assert_called_once_with()

    def test_direct_union_rechecks_current_policy_before_receipts_or_output(self):
        policy = campaign_policy()
        changed = copy.deepcopy(policy)
        changed["candidate_sha256"] = "0" * 64
        with patch.object(MODULE, "read_phase_step_policy", return_value=changed), patch.object(MODULE.COMMON, "read_analysis_json") as receipts, patch.object(MODULE.COMMON, "_require_private_output") as output:
            with self.assertRaisesRegex(MODULE.Incomplete, "current complete source"):
                MODULE.frequency_cohort_union(arguments("cohort-union"), {}, policy, ROLE)
            receipts.assert_not_called()
            output.assert_not_called()

    def test_exact_contracts_dispatch_uses_one_current_callback_and_no_mutation(self):
        policy = synthetic_policy()
        self.assertEqual(policy["external_consumer_limits"], {"legal_executions": 3, "negative_compilations": 32})
        fake = Mock()
        report = {}
        with patch.object(MODULE, "read_phase_step_policy", return_value=policy), patch.object(MODULE.COMMON, "Session", return_value=fake) as session, patch.object(MODULE, "_frequency_captured") as captured, patch.object(MODULE, "_phase_step_contracts") as contracts, patch.object(MODULE, "configure_phase_plan_mutation") as mutation:
            MODULE.execute_phase_step(arguments("contracts"), report)
        session.assert_called_once()
        self.assertEqual(session.call_args.args[1].lane, "contracts")
        captured.assert_called_once_with(fake, policy, ROLE)
        contracts.assert_called_once_with(fake, policy, report)
        mutation.assert_not_called()
        fake.run.assert_not_called()
        fake.finish.assert_called_once_with()
        self.assertFalse(report["qualification_passed"])
        self.assertFalse(report["full_role_qualification"])

    def test_contract_callback_loads_only_the_captured_current_registry(self):
        session = SimpleNamespace(source=Path("/synthetic/captured"))
        policy, report, helper = synthetic_policy(), {}, Mock()
        helper.phase_step_contracts.side_effect = lambda *args: args[3]("registry-arguments")
        with patch.object(MODULE.COMMON, "load_module", return_value=helper) as load, \
                patch.object(MODULE.time, "monotonic", return_value=10), \
                patch.object(MODULE, "_external_contract_registry") as registry:
            MODULE._phase_step_contracts(session, policy, report)
            registry.assert_called_once_with("registry-arguments", deadline=310)
        load.assert_called_once_with("core_phase_step_contracts", session.source / "scripts/core-frequency-contracts.py")
        call = helper.phase_step_contracts.call_args.args
        self.assertEqual(call[:3], (session, policy, report))
        self.assertEqual(call[4], MODULE.phase_plan_contract_examples)
        helper.phase_step_validation_mechanisms.assert_called_once_with(session, policy, report, deadline=310)

    def test_failed_compiler_registry_never_starts_validation_mechanisms(self):
        helper = Mock()
        helper.phase_step_contracts.side_effect = ValueError("synthetic registry failure")
        with patch.object(MODULE.COMMON, "load_module", return_value=helper):
            with self.assertRaisesRegex(ValueError, "synthetic registry failure"):
                MODULE._phase_step_contracts(SimpleNamespace(source=Path("/synthetic")), synthetic_policy(), {})
        helper.phase_step_validation_mechanisms.assert_not_called()

    def test_shared_contract_deadline_cannot_extend_or_disable_budget(self):
        examples = {str(index): {"code": None if index < 3 else "E0451"} for index in range(35)}
        markers = {str(index): "synthetic-marker" for index in range(3)}
        for deadline in (True, 0, 311, float("nan"), float("inf")):
            with self.subTest(deadline=deadline), patch.object(MODULE.time, "monotonic", return_value=10):
                session = Mock()
                with self.assertRaisesRegex(MODULE.Incomplete, "bounded shared deadline"):
                    MODULE._external_contract_registry(session, synthetic_policy(), {}, examples, markers,
                                                       "synthetic scope", deadline=deadline)
                session.run.assert_not_called()

    def test_contract_input_or_owner_drift_refuses_before_session(self):
        with tempfile.TemporaryDirectory() as directory:
            root, policy, identity = bound_fixture(directory, pending=False)
            missing = copy.deepcopy(policy)
            missing["inputs_sha256"].pop("scripts/core-frequency-contracts.py")
            changed = copy.deepcopy(policy)
            changed["inputs_sha256"]["scripts/tests/test_phase_step_contracts.py"] = "0" * 64
            reordered = copy.deepcopy(policy)
            reordered["critical_files"].reverse()
            wrong_bool = copy.deepcopy(policy)
            wrong_bool["contracts_admitted"] = 1
            for invalid in (missing, changed, reordered, wrong_bool):
                with self.subTest(invalid=invalid), patch.object(MODULE, "ROOT", root), patch.object(MODULE, "inventory_document", return_value=invalid), patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)), patch.object(MODULE.COMMON, "Session") as session:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.execute_phase_step(arguments("contracts"), {})
                session.assert_not_called()

    def test_effect_boundary_cannot_bypass_closed_campaign_with_seven_maps(self):
        policy = synthetic_policy()
        policy["mutation_inventories"] = reviewed_maps()
        with self.assertRaisesRegex(MODULE.Incomplete, "admission is closed"):
            MODULE.COMMON.mutation_campaign_command(ROLE.scope, Path("/synthetic/tools"), "step", [],
                                                    expected_jobs=10, expected_message_format="json", policy=policy)

    def test_inventory_admits_only_exact_stable_session_profile(self):
        policy = synthetic_policy()
        self.assertEqual(MODULE.COMMON.admission_profile(arguments("contracts"), ROLE.scope, policy), "stable-core")
        self.assertEqual(MODULE.COMMON.admission_profile(arguments("mutation"), ROLE.scope, policy), "stable-core")
        for lane, profile in (("inventory", "stable-core"), ("all", "stable-core"), ("contracts", "full")):
            with self.subTest(lane=lane, profile=profile):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.admission_profile(arguments(lane, tool_admission_profile=profile), ROLE.scope, policy)

    def test_all_seven_whole_inventories_required_and_caps_unchanged(self):
        policy = synthetic_policy()
        self.assertEqual(MODULE.frequency_inventory_bindings(policy, role=ROLE), {})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.frequency_inventory_bindings(policy, required=True, role=ROLE)
        policy["mutation_inventories"] = reviewed_maps(120)
        MODULE.frequency_inventory_bindings(policy, required=True, role=ROLE)
        for owner, path in OWNERS.items():
            self.assertEqual(MODULE.frequency_selected_owners(policy, owner, role=ROLE), {owner: path})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.frequency_selected_owners(policy, None, role=ROLE)
        for maps in (reviewed_maps(513), reviewed_maps(512), dict(list(reviewed_maps().items())[:-1])):
            with self.subTest(maps=maps):
                changed = copy.deepcopy(policy)
                changed["mutation_inventories"] = maps
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.frequency_inventory_bindings(changed, required=True, role=ROLE)

    def test_seven_owner_total_cap_admits_exact_2048_but_refuses_next_variant(self):
        policy = synthetic_policy()
        policy["mutation_inventories"] = reviewed_maps(292)
        policy["mutation_inventories"]["frequency_numeric"]["count"] = 296
        self.assertEqual(sum(row["count"] for row in policy["mutation_inventories"].values()), 2048)
        MODULE.frequency_inventory_bindings(policy, required=True, role=ROLE)
        policy["mutation_inventories"]["frequency_numeric"]["count"] += 1
        with self.assertRaisesRegex(MODULE.Incomplete, "exceeds 2048"):
            MODULE.frequency_inventory_bindings(policy, required=True, role=ROLE)

    def test_listing_observes_names_not_successes(self):
        ordinary = ["z_case", "module::a_case"]
        self.assertEqual(MODULE.phase_step_listing_names(listing(ordinary) + "\n0 tests, 0 benchmarks\n", doctests=False), sorted(ordinary))
        docs = ["crates/time-core/src/lib.rs - Example (line 4)",
                "crates/time-core/src/lib.rs - Example (line 8) - compile fail"]
        self.assertEqual(MODULE.phase_step_listing_names(listing(docs), doctests=True), docs)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.phase_step_listing_names("running 1 test\ntest healthy ... ok\ntest result: ok. 1 passed\n", doctests=False)

    def test_root_module_doctest_and_compile_fail_are_valid_native_neighbors(self):
        root = "crates/time-core/src/lib.rs - (line 17)"
        failed = "crates/time-core/src/lib.rs - (line 31) - compile fail"
        self.assertEqual(MODULE.phase_step_listing_names(root + ": test\n\n1 test, 0 benchmarks\n", doctests=True), [root])
        self.assertEqual(MODULE.phase_step_listing_names(failed + ": test\n\n1 test, 0 benchmarks\n", doctests=True), [failed])
        self.assertEqual(MODULE.phase_step_listing_names(listing([root, failed]), doctests=True), [root, failed])

    def test_new_runtime_controls_and_contract_source_are_in_protected_scope(self):
        for name in ("scripts/core-frequency-contracts.py", "scripts/tests/test_phase_step_contracts.py",
                     "scripts/tests/test_phase_step_analysis.py", "scripts/tests/test_portable_analysis.py",
                     "crates/time-core/tests/private/step_qualification.rs",
                     "crates/time-core/tests/private/step_diagnostics.rs"):
            self.assertIn(name, ROLE.bound_inputs)
            self.assertEqual(ROLE.bound_inputs.count(name), 1)
        self.assertEqual(len(ROLE.bound_inputs), 64)
        expected = {
            "adaptive_step_fresh_reference_uses_original_accepted_interval",
            "adaptive_admitted_no_plan_preserves_accepted_stamp_across_fresh_rows",
            "step_qualification::matched_step_rate_overflow_retains_known_effect_beside_finite_neighbor",
            "step_qualification::matched_step_terminal_span_and_current_readings_preserve_pending_owner",
            "step_qualification::matched_step_lower_consumed_ordinal_rejects_beside_current_completion",
            "step_qualification::matched_step_neutral_reference_refuses_beside_sign_certain_neighbor",
            "step_diagnostics::matched_step_constructor_diagnostics_are_opaque_and_preserve_checked_receipts",
            "step_diagnostics::matched_step_request_diagnostics_keep_exact_refusals_and_pending_custody",
            "step_diagnostics::matched_step_neutral_diagnostic_has_no_delegated_cause",
            "step_diagnostics::matched_step_retirement_diagnostics_preserve_provenance_after_sole_transfer",
            "step_diagnostics::matched_step_terminal_diagnostics_do_not_conflate_no_effect_and_unknown",
            "reference_input::rate_delivery::phase_plan::adaptive::tests::adaptive_small_debt_threshold_equality_and_adjacent_bits_are_distinct",
            "reference_input::rate_delivery::phase_plan::adaptive::tests::adaptive_profile_and_leaf_failures_keep_bounded_redacted_diagnostics",
            "reference_input::rate_delivery::phase_plan::adaptive::tests::adaptive_arithmetic_failure_preserves_original_typed_source",
            "reference_input::rate_delivery::phase_plan::adaptive::tests::adaptive_private_numeric_boundaries_keep_healthy_neighbors",
            "reference_input::rate_delivery::phase_plan::adaptive::tests::adaptive_private_area_boundaries_keep_healthy_neighbors",
            "reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts",
        }
        self.assertLessEqual(expected, ROLE.required_tests)

    def test_listing_rejects_duplicate_missing_foreign_and_failure_neighbors(self):
        cases = [listing(["same", "same"]), "healthy: test\n", listing(["healthy"]).replace("1 tests", "2 tests"),
                 listing(["healthy"]) + "error: test failed\n", "healthy: benchmark\n1 tests, 1 benchmarks\n",
                 listing(["/foreign/path"]), listing(["crates/other/src/lib.rs - Example (line 4)"])]
        for text in cases:
            with self.subTest(text=text):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.phase_step_listing_names(text, doctests=False)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.phase_step_listing_names(listing(["healthy"]), doctests=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.phase_step_listing_names(listing(["crates/other/src/lib.rs - Example (line 4)"]), doctests=True)

    def test_inventory_uses_fixed_full_commands_and_grants_no_execution_pass(self):
        docs = ["crates/time-core/src/lib.rs - Example (line 4)"]
        report = {"qualification_passed": False}
        with tempfile.TemporaryDirectory() as directory:
            root, identity = literal_fixture(directory, b"// synthetic parser fixture\n// header\n// header\n//! ```\n//! let healthy = 1;\n//! ```\n")
            session = SimpleNamespace(source=root, product_identity=identity,
                                      run=Mock(side_effect=[listing(["healthy"]), listing(docs)]))
            MODULE._phase_step_inventory(session, synthetic_policy(), report)
        common = ["cargo", "test", "--package", "time-core", "--frozen", "--jobs", "6"]
        self.assertEqual(session.run.call_args_list[0].args[:2], ("inventory-native", [*common, "--lib", "--tests", "--", "--list"]))
        self.assertEqual(session.run.call_args_list[1].args[:2], ("inventory-doctests", [*common, "--doc", "--", "--list"]))
        discovery = report["test_inventory_discovery"]
        self.assertEqual(discovery["inventories"], {"baseline": sorted(["healthy", *docs]), "coverage": ["healthy"]})
        self.assertEqual(discovery["native_tests_executed"], 0)
        self.assertFalse(report["qualification_passed"])
        self.assertNotIn("baseline_execution", report)
        classification = discovery["doctest_declaration_classification"]
        self.assertEqual(classification["native_tests_executed"], 0)
        self.assertEqual(classification["names"], docs)
        self.assertTrue(classification["declarations"][0]["declaration_only"])
        for lane, names in discovery["inventories"].items():
            self.assertEqual(discovery["bindings"][lane], {"count": len(names), "sha256": MODULE.COMMON.canonical_sha256(names)})

    def test_pinned_literal_declarations_classify_plain_list_names_without_execution(self):
        legal = "crates/time-core/src/lib.rs - (line 1)"
        negative = "crates/time-core/src/lib.rs - Example (line 5)"
        with tempfile.TemporaryDirectory() as directory:
            root, identity = literal_fixture(directory)
            result = MODULE.phase_step_classify_doctests(root, identity, [legal, negative])
        self.assertEqual(result["names"], [legal, negative + " - compile fail"])
        self.assertEqual(result["product_identity_sha256"], MODULE.COMMON.canonical_sha256(identity))
        self.assertEqual(result["product_source_file_count"], 1)
        self.assertEqual(result["native_tests_executed"], 0)
        self.assertEqual([row["declaration"] for row in result["declarations"]], ["rust", "compile_fail"])
        self.assertEqual([row["opening_fence_line"] for row in result["declarations"]], [1, 5])
        self.assertEqual([row["closing_fence_line"] for row in result["declarations"]], [3, 7])
        self.assertEqual([row["comment_marker"] for row in result["declarations"]], ["//!", "///"])
        self.assertEqual([row["observed_name"] for row in result["declarations"]], [legal, negative])
        self.assertTrue(all(row["declaration_only"] for row in result["declarations"]))
        self.assertEqual(result["declarations"][1]["source_sha256"], identity["crates/time-core/src/lib.rs"])

    def test_literal_declaration_classification_rejects_foreign_hash_and_wrong_lines(self):
        with tempfile.TemporaryDirectory() as directory:
            root, identity = literal_fixture(directory)
            for name in ("crates/other/src/lib.rs - (line 1)",
                         "crates/time-core/src/foreign.rs - (line 1)",
                         "crates/time-core/src/../lib.rs - (line 1)",
                         "crates/time-core/src/lib.rs - (line 0)",
                         "crates/time-core/src/lib.rs - (line 2)",
                         "crates/time-core/src/lib.rs - (line 3)",
                         "crates/time-core/src/lib.rs - (line 999)"):
                with self.subTest(name=name), self.assertRaises(MODULE.Incomplete):
                    MODULE.phase_step_classify_doctests(root, identity, [name])
            (root / "crates/time-core/src/lib.rs").write_bytes(b"//! ```compile_fail\n//! changed();\n//! ```\n")
            with patch.object(MODULE, "_phase_step_literal_fences") as parser:
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.phase_step_classify_doctests(root, identity, ["crates/time-core/src/lib.rs - (line 1)"])
            parser.assert_not_called()

    def test_literal_declaration_classification_rejects_fence_and_suffix_mismatches(self):
        name = "crates/time-core/src/lib.rs - (line 1)"
        cases = [b"//! ```text\n//! not a Rust declaration\n//! ```\n",
                 b"//! ```compile_fail_extra\n//! unavailable();\n//! ```\n",
                 b"//! ```compile_fail,E0451\n//! unavailable();\n",
                 b"//! ```\nnot_a_doc_comment();\n//! ```\n",
                 b"//! ```\n//! healthy();\n/// ```\n"]
        for data in cases:
            with self.subTest(data=data), tempfile.TemporaryDirectory() as directory:
                root, identity = literal_fixture(directory, data)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.phase_step_classify_doctests(root, identity, [name])
        with tempfile.TemporaryDirectory() as directory:
            root, identity = literal_fixture(directory)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.phase_step_classify_doctests(root, identity, [name + " - compile fail"])
            negative = "crates/time-core/src/lib.rs - Example (line 5) - compile fail"
            self.assertEqual(MODULE.phase_step_classify_doctests(root, identity, [negative])["names"], [negative])

    def test_literal_declaration_classification_rejects_duplicate_sources_and_unbounded_reads(self):
        name = "crates/time-core/src/lib.rs - (line 1)"
        with tempfile.TemporaryDirectory() as directory:
            root, identity = literal_fixture(directory)
            for names in ([name, name], [name, "crates/time-core/src/lib.rs - DifferentOwner (line 1)"]):
                with self.subTest(names=names), self.assertRaises(MODULE.Incomplete):
                    MODULE.phase_step_classify_doctests(root, identity, names)
            changed = copy.deepcopy(identity)
            changed["crates/time-core/src/lib.rs"] = "foreign hash"
            with self.assertRaises(MODULE.Incomplete):
                MODULE.phase_step_classify_doctests(root, changed, [name])
        with tempfile.TemporaryDirectory() as directory:
            root, identity = literal_fixture(directory, b"x" * (512 * 1024 + 1))
            with self.assertRaises(MODULE.Incomplete):
                MODULE.phase_step_classify_doctests(root, identity, [name])

    def test_inventory_dispatch_uses_sealed_session_and_pending_roster_exception_only(self):
        policy = synthetic_policy()
        fake = Mock()
        report = {}
        with patch.object(MODULE, "read_phase_step_policy", return_value=policy) as read, patch.object(MODULE.COMMON, "Session", return_value=fake) as session, patch.object(MODULE, "_frequency_captured") as captured, patch.object(MODULE, "_phase_step_inventory") as inventory:
            MODULE.execute_phase_step(arguments("inventory"), report)
        read.assert_called_once_with(ROOT, allow_pending_rosters=True)
        self.assertEqual(session.call_args.args[1].lane, "contracts")
        self.assertEqual(session.call_args.args[3], ROLE.scope)
        captured.assert_called_once_with(fake, policy, ROLE)
        inventory.assert_called_once_with(fake, policy, report)
        fake.finish.assert_called_once_with()
        self.assertFalse(report["qualification_passed"])

    def test_coverage_dispatch_selects_full_test_graph_and_every_complete_file(self):
        policy = synthetic_policy()
        fake = Mock(work=Path("/synthetic/work"), tools=Path("/synthetic/tools"))
        fake.run.return_value = "synthetic transcript checked by separate parser controls"
        report = {}
        with patch.object(MODULE, "read_phase_step_policy", return_value=policy), patch.object(MODULE.COMMON, "Session", return_value=fake), patch.object(MODULE, "_frequency_captured"), patch.object(MODULE, "_conditioned_test_execution", return_value={"synthetic": True}), patch.object(MODULE.COMMON, "read_analysis_json", return_value={}), patch.object(MODULE.COMMON, "coverage_counts", return_value={"synthetic": True}) as counts:
            MODULE.execute_phase_step(arguments("coverage"), report)
        command = fake.run.call_args.args[1]
        self.assertIn("--lib", command)
        self.assertIn("--tests", command)
        self.assertNotIn("--test", command)
        self.assertEqual(set(report["coverage"]), set(OWNERS))
        self.assertEqual([call.args[2] for call in counts.call_args_list],
                         ["/work/source/" + path for path in OWNERS.values()])
        self.assertFalse(report["qualification_passed"])
        fake.finish.assert_called_once_with()


class PhaseStepCampaignReaderControls(unittest.TestCase):
    def command(self, policy, tools=Path("/synthetic/leaf/tools")):
        """Independent literal GNU recipe, without launching its executable."""
        return [str(tools / "cargo-mutants"), "mutants", "--package", "time-core", "--file", OWNERS["step"],
            "--test-workspace", "false", "--cap-lints", "false", "--jobs", str(policy["mutation_jobs"]),
            "--jobserver", "true", "--jobserver-tasks", "0", "--cargo-arg=--message-format=json",
            "--baseline", "run", "--build-timeout", "90", "--timeout", "60", "--output", "/work/mutation/step"]

    def test_only_three_gnu_worker_profiles_admit_zero_additional_tokens(self):
        self.assertEqual(MODULE.PHASE_STEP_FIXED["mutation_jobs"], 10)
        self.assertEqual(MODULE.PHASE_STEP_FIXED["mutation_jobserver_tasks"], 0)
        for workers in (10, 8, 6):
            policy = campaign_policy()
            policy["mutation_jobs"] = workers
            with self.subTest(workers=workers):
                MODULE.COMMON.require_phase_step_role(ROLE.scope, policy)
                self.assertEqual(MODULE.COMMON.mutation_jobserver_tasks(ROLE.scope, policy, workers=workers), 0)
                MODULE.COMMON.mutation_campaign_command(ROLE.scope, Path("/synthetic/leaf/tools"), "step",
                    self.command(policy), expected_jobs=workers, expected_message_format="json", policy=policy)
        for field, values in (("mutation_jobs", (True, 0, 1, 7, 9, 11, 12, 8.0)),
                              ("mutation_jobserver_tasks", (False, True, 1, 6, 8, 10, 0.0, None))):
            for value in values:
                policy = campaign_policy()
                policy[field] = value
                with self.subTest(field=field, value=value), self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.require_phase_step_role(ROLE.scope, policy)
        legacy = copy.deepcopy(MODULE.FREQUENCY_PRIOR_FIXED)
        self.assertEqual(MODULE.COMMON.mutation_jobserver_tasks(MODULE.FREQUENCY_PRIOR_SCOPE, legacy, workers=10), 10)
        legacy["mutation_jobserver_tasks"] = 0
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.mutation_jobserver_tasks(MODULE.FREQUENCY_PRIOR_SCOPE, legacy, workers=10)
        policy = campaign_policy()
        for workers in (8, 6):
            with self.subTest(foreign_workers=workers), self.assertRaisesRegex(MODULE.Incomplete, "caller workers"):
                MODULE.COMMON.mutation_campaign_command(ROLE.scope, Path("/synthetic/leaf/tools"), "step",
                    self.command(policy), expected_jobs=workers, expected_message_format="json", policy=policy)

    def test_each_worker_profile_lists_the_complete_owner_with_zero_shared_tokens(self):
        for workers in (10, 8, 6):
            policy = campaign_policy()
            policy["mutation_jobs"] = workers
            item = {"name": "synthetic_step_fault", "file": OWNERS["step"], "package": "time-core"}
            calls = []
            def run(label, command, timeout):
                calls.append((label, command, timeout))
                return json.dumps([item])
            with self.subTest(workers=workers):
                report = {}
                result = MODULE.COMMON.enumerate_mutation_inventories(ROLE.scope, {"step": OWNERS["step"]},
                    policy, Path("/synthetic/leaf/tools"), run, report, deadline=time.monotonic() + 120,
                    maximum_per_file=512, maximum_total=512)
                self.assertEqual(set(result.inventories["step"]), {"synthetic_step_fault"})
                command = self.command(policy)
                self.assertEqual(calls[0][1], [*command[:command.index("--baseline")], "--list", "--json"])
                self.assertNotIn("--cargo-arg=--jobs=1", calls[0][1])
                self.assertEqual(MODULE.COMMON.parse_mutation_listing(json.dumps([item]),
                    expected_jobs=workers)[0], [item])

    def test_retained_campaign_envelope_binds_outer_workers_and_zero_tokens(self):
        work, stable = Path("/synthetic/leaf/work"), Path("/synthetic/stable")
        admitted = {"stable": stable, "readonly_roots": []}
        receipt = {"analysis_admission": {"readonly_roots": []}}
        for workers in (10, 8, 6):
            policy = campaign_policy()
            policy["mutation_jobs"] = workers
            command = self.command(policy)
            campaign = {"command": ["synthetic-envelope", *command], "exit_code": 2,
                        "failure": None, "output": "synthetic result; no mutation executed"}
            with self.subTest(workers=workers), patch.object(MODULE.COMMON.RUNTIME, "_directory"), \
                 patch.object(MODULE, "_frequency_config", return_value={"vendor": "/synthetic/vendor"}), \
                 patch.object(MODULE.COMMON.RUNTIME, "sandbox_command", side_effect=lambda *args, **kw: ["synthetic-envelope", *args[2]]) as sandbox:
                self.assertIs(MODULE.frequency_campaign_result(work, receipt, campaign, "step", ROLE,
                    policy, admitted, {}), campaign)
                self.assertEqual(sandbox.call_args.kwargs["build_jobs"], 1)
                self.assertEqual(sandbox.call_args.kwargs["environment"]["RUST_TEST_THREADS"], "1")
                for flag, foreign in (("--jobs", "6" if workers != 6 else "8"), ("--jobserver-tasks", str(workers))):
                    changed = copy.deepcopy(campaign)
                    index = changed["command"].index(flag) + 1
                    changed["command"][index] = foreign
                    with self.subTest(flag=flag), self.assertRaisesRegex(MODULE.Incomplete, "sandbox command differs"):
                        MODULE.frequency_campaign_result(work, receipt, changed, "step", ROLE, policy, admitted, {})

    def test_complete_seven_receipt_union_reuses_every_existing_leaf_check(self):
        policy = campaign_policy()
        policy["mutation_jobs"] = 8
        admitted = {"inventories": {"stable": "c" * 64}, "policy_sha256": "d" * 64}
        identity = {name: "b" * 64 for name in ("Cargo.toml", ROLE.scope.policy, *ROLE.bound_inputs)}
        result, baseline = {"gate_passed": True}, {"tests_executed": 710}
        payloads = {owner: {"name": owner + "_fault", "file": filename, "package": "time-core", "diff": "synthetic"}
                    for owner, filename in OWNERS.items()}
        policy["mutation_inventories"] = {owner: {"count": 1, "sha256": MODULE.COMMON.canonical_sha256({
            item["name"]: {key: value for key, value in item.items() if key != "diff"}})}
            for owner, item in payloads.items()}
        with tempfile.TemporaryDirectory() as directory, ExitStack() as stack:
            root, paths, documents = Path(directory), [], {}
            for owner in OWNERS:
                parent = root / owner
                parent.mkdir()
                path = parent / "verification.json"
                path.write_text("{}")
                paths.append(path)
                documents[str(path)] = dict(status="passed", selected_lane="mutation", cohort_complete=True,
                    cohort_id=owner, subjects={owner: OWNERS[owner]}, complete_subjects=OWNERS,
                    candidate_sha256=policy["candidate_sha256"], source_sha256=identity,
                    tool_admission_profile="stable-core", all_reviewed_inventory_bindings=policy["mutation_inventories"],
                    analysis_policy_sha256=admitted["policy_sha256"],
                    analysis_admission={"profile": "stable-core", "inventories": admitted["inventories"]},
                    analysis_admission_after={"profile": "stable-core", "inventories": admitted["inventories"], "unchanged": True},
                    owned_output=str(parent), work=str(parent / "work"), tools_sha256={"cargo-mutants": "b" * 64},
                    mutation_enumeration={owner: {"raw_output": json.dumps([payloads[owner]])}},
                    lanes=[{"lane": "mutation-campaign:" + owner, "command": ["synthetic"], "exit_code": 0}],
                    mutation={owner: {**result, "baseline_tests_executed": 710}}, baseline_executions={owner: baseline},
                    phase_step_artifacts={owner: {"synthetic": owner}})
            def read(path):
                if path.name == "outcomes.json": return {"synthetic_outcomes": True}
                return documents[str(path)]
            def artifacts(*args, **kwargs):
                self.assertEqual(kwargs["role"], ROLE)
                return {"synthetic": args[3]}
            for target, name, options in (
                (MODULE, "read_phase_step_policy", {"return_value": policy}),
                (MODULE.PRODUCT, "admit_product", {"return_value": ({}, {"Cargo.toml": "b" * 64})}),
                (MODULE.PRODUCT, "sha256", {"return_value": "b" * 64}),
                (MODULE.COMMON.ANALYSIS, "admit", {"return_value": admitted}),
                (MODULE.COMMON, "read_analysis_json", {"side_effect": read}),
                (MODULE.COMMON, "require_unchanged", {}),
                (MODULE.COMMON, "tool_path", {"return_value": Path("/tools/cargo-mutants")}),
                (MODULE.COMMON, "_require_private_output", {})):
                stack.enter_context(patch.object(target, name, **options))
            envelopes = stack.enter_context(patch.object(MODULE, "frequency_campaign_result"))
            listings = stack.enter_context(patch.object(MODULE.COMMON, "parse_mutation_listing",
                wraps=MODULE.COMMON.parse_mutation_listing))
            counts = stack.enter_context(patch.object(MODULE.COMMON, "mutation_counts", return_value=result))
            baselines = stack.enter_context(patch.object(MODULE, "_conditioned_baseline", return_value=baseline))
            raw = stack.enter_context(patch.object(MODULE, "frequency_artifact_manifest", side_effect=artifacts))
            process = stack.enter_context(patch("subprocess.Popen"))
            args = arguments("cohort-union", cohort_receipt=paths, output_dir=root / "union")
            report = {}
            MODULE.frequency_cohort_union(args, report, policy, ROLE)
            self.assertEqual(set(report["mutation_cohort_union"]), set(OWNERS))
            self.assertTrue(report["mutation_union_passed"])
            self.assertFalse(report["full_role_qualification"])
            for guard in (envelopes, counts, baselines, raw): self.assertEqual(guard.call_count, 7)
            self.assertEqual([call.kwargs["expected_jobs"] for call in listings.call_args_list], [8] * 7)
            for receipts in (paths[:-1], [*paths[:-1], paths[0]]):
                with self.subTest(receipts=receipts), self.assertRaises(MODULE.Incomplete):
                    MODULE.frequency_cohort_union(arguments("cohort-union", cohort_receipt=receipts, output_dir=root / "missing"), {}, policy, ROLE)
            documents[str(paths[0])]["candidate_sha256"] = "0" * 64
            with self.assertRaises(MODULE.Incomplete):
                MODULE.frequency_cohort_union(arguments("cohort-union", cohort_receipt=paths, output_dir=root / "stale"), {}, policy, ROLE)
            process.assert_not_called()

    def test_debug_profile_accepts_only_the_exact_phase_step_scope(self):
        with tempfile.TemporaryDirectory() as directory:
            config = Path(directory) / "config.toml"
            before = '[source.crates-io]\nreplace-with = "reviewed"\n[source.reviewed]\ndirectory = "/synthetic/vendor"\n'
            config.write_text(before)
            session = SimpleNamespace(config=config, vendor=Path("/synthetic/vendor"),
                config_sha256=hashlib.sha256(before.encode()).hexdigest())
            def configure(text):
                config.write_text(text)
                session.config_sha256 = hashlib.sha256(text.encode()).hexdigest()
            session.configure = configure
            report = {}
            MODULE.configure_phase_plan_mutation(session, campaign_policy(), report, ROLE)
            self.assertEqual(tomllib.loads(config.read_text())["profile"],
                             {"dev": {"debug": 0}, "test": {"debug": 0}})
            self.assertFalse(report["mutation_debug_configuration"]["other_profile_settings_changed"])
            for wrong_scope in (MODULE.FREQUENCY_PRIOR_SCOPE,
                                MODULE.COMMON.Scope("time-core", ROLE.scope.subjects[:-1], ROLE.scope.policy)):
                wrong = MODULE.ConditionedAnalysisRole(ROLE.selector, ROLE.label, wrong_scope,
                    ROLE.fixed, ROLE.bound_inputs, ROLE.required_tests, ROLE.required_doctests, ROLE.coverage_tests)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.configure_phase_plan_mutation(session, campaign_policy(), {}, wrong)

    def private_fixture(self, directory):
        """Copy only current literal test/include bytes; compile no Rust fixture."""
        root = Path(directory)
        names = set(MODULE._PHASE_STEP_PRIVATE_TEST_BINDINGS)
        names.update(edge[0] for row in MODULE._PHASE_STEP_PRIVATE_TEST_BINDINGS.values() for edge in row[2])
        identity = {}
        for name in names:
            raw = (ROOT / name).read_bytes()
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(raw)
            identity[name] = hashlib.sha256(raw).hexdigest()
        return root, identity

    def test_actual_private_chains_resolve_adjacent_tests_and_top_level_owner(self):
        cases = (
            ("step_contracts", "matched_step_raw_limits_and_receipts_reject_bypasses_before_valid_issue", "--test phase_plan"),
            ("step_qualification", "matched_step_rate_overflow_retains_known_effect_beside_finite_neighbor", "--test phase_plan"),
            ("step_diagnostics", "matched_step_constructor_diagnostics_are_opaque_and_preserve_checked_receipts", "--test phase_plan"),
            ("step_counter", "known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map", "--lib"),
            ("adaptive_horizon", "adaptive_near_minimum_quantizes_before_integer_clamping", "--lib"),
        )
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.private_fixture(directory)
            for module, name, expected in cases:
                source = f"crates/time-core/tests/private/{module}.rs"
                namespace = ("reference_input::rate_delivery::phase_plan::tests::step_counter" if module == "step_counter" else
                    "reference_input::rate_delivery::phase_plan::adaptive::tests" if module == "adaptive_horizon" else module)
                cause = {"source": source, "test": namespace + "::" + name}
                with self.subTest(module=module):
                    self.assertEqual(MODULE._frequency_failed_target(cause, root, identity, role=ROLE), expected)
            direct = {"source": MODULE._PHASE_STEP_TEST_ROOT, "test": "actual_top_level_test"}
            self.assertEqual(MODULE._frequency_failed_target(direct, root, identity, role=ROLE), "--test phase_plan")
            source = "crates/time-core/tests/private/step_diagnostics.rs"
            cause = {"source": source, "test": "step_diagnostics::matched_step_neutral_diagnostic_has_no_delegated_cause"}
            self.assertEqual(MODULE._frequency_failed_target(cause, root, identity, role=ROLE), "--test phase_plan")

    def test_actual_nested_step_counter_tests_keep_the_parent_helper_cause(self):
        """Actual reference-input 206/211/222 faults reach this checked parent helper."""
        tests = (
            "known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts",
            "known_step_fresh_recovery_retains_counter_witness_and_rejects_old_absolute_map",
        )
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.private_fixture(directory)
            parent = MODULE._PHASE_STEP_LIFECYCLE
            child = "crates/time-core/tests/private/step_counter.rs"
            self.assertIn("fn prepared_private_baseline_receiver", (root / parent).read_text())
            self.assertIn("prepared_private_baseline_receiver(fixture, chart)?", (root / child).read_text())
            self.assertTrue((root / parent).read_text().splitlines()[1153].lstrip().startswith("assert!("))
            for name in tests:
                cause = {"source": parent, "line": 1154, "column": 5,
                    "test": "reference_input::rate_delivery::phase_plan::tests::step_counter::" + name}
                with self.subTest(name=name):
                    self.assertEqual(MODULE._frequency_failed_target(cause, root, identity, role=ROLE), "--lib")
                    self.assertEqual(MODULE._frequency_failed_target({**cause, "source": child},
                        root, identity, role=ROLE), "--lib")

    def test_actual_reference_input_readiness_uses_its_integration_include(self):
        """Actual caught reference-input faults use this existing independent test owner."""
        source = "crates/time-core/tests/private/reference_input_readiness.rs"
        parent = "crates/time-core/tests/chart_combination_lifecycle.rs"
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.private_fixture(directory)
            for name in (source, parent):
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes((ROOT / name).read_bytes())
                identity[name] = hashlib.sha256(path.read_bytes()).hexdigest()
            for test in ("achieved_phase_dyadics_change_coordinates_and_preserve_rate",
                         "achieved_phase_context_and_scalar_rejections_preserve_ready_owner"):
                with self.subTest(test=test):
                    cause = {"source": source, "test": "reference_input_readiness::" + test}
                    self.assertEqual(MODULE._frequency_failed_target(cause, root, identity, role=ROLE),
                                     "--test chart_combination_lifecycle")

    def test_parent_helper_cannot_borrow_a_sibling_descendant_or_foreign_namespace(self):
        parent = MODULE._PHASE_STEP_LIFECYCLE
        child = "crates/time-core/tests/private/step_counter.rs"
        namespace = "reference_input::rate_delivery::phase_plan::tests"
        cases = (
            (parent, namespace + "::adaptive::tests::adaptive_near_minimum_quantizes_before_integer_clamping"),
            (child, namespace + "::actual_unknown_reception_latches_frequency_before_private_projection"),
            (parent, namespace + "::step_counter::foreign::known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts"),
        )
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.private_fixture(directory)
            for source, test in cases:
                with self.subTest(test=test), self.assertRaisesRegex(MODULE.Incomplete, "foreign module"):
                    MODULE._frequency_failed_target({"source": source, "test": test}, root, identity, role=ROLE)

    def test_nested_helper_requires_the_child_test_declaration_and_both_source_hashes(self):
        parent = MODULE._PHASE_STEP_LIFECYCLE
        child = "crates/time-core/tests/private/step_counter.rs"
        name = "known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts"
        cause = {"source": parent, "test": "reference_input::rate_delivery::phase_plan::tests::step_counter::" + name}
        for variant in ("removed", "duplicate", "child_drift", "parent_drift", "include_duplicate"):
            with self.subTest(variant=variant), tempfile.TemporaryDirectory() as directory:
                root, identity = self.private_fixture(directory)
                changed = parent if variant in {"parent_drift", "include_duplicate"} else child
                path = root / changed
                original = path.read_text()
                if variant == "removed":
                    replacement = original.replace("#[test]\nfn " + name, "fn " + name)
                elif variant == "duplicate":
                    replacement = original + "\n#[test]\nfn " + name + "() {}\n"
                elif variant == "include_duplicate":
                    replacement = original + '\n#[path = "step_counter.rs"]\nmod duplicate;\n'
                else:
                    replacement = original + "\n// drift after source capture\n"
                self.assertNotEqual(original, replacement)
                path.write_text(replacement)
                if not variant.endswith("_drift"):
                    identity[changed] = hashlib.sha256(path.read_bytes()).hexdigest()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_failed_target(cause, root, identity, role=ROLE)

    def test_reference_input_integration_owner_rejects_changed_and_duplicate_includes(self):
        source = "crates/time-core/tests/private/reference_input_readiness.rs"
        parent = "crates/time-core/tests/chart_combination_lifecycle.rs"
        cause = {"source": source, "test": "reference_input_readiness::achieved_phase_dyadics_change_coordinates_and_preserve_rate"}
        for variant in ("module", "duplicate", "source_drift"):
            with self.subTest(variant=variant), tempfile.TemporaryDirectory() as directory:
                root, identity = self.private_fixture(directory)
                path = root / (source if variant == "source_drift" else parent)
                text = path.read_text()
                if variant == "module":
                    path.write_text(text.replace("mod reference_input_readiness;", "mod foreign;"))
                elif variant == "duplicate":
                    path.write_text(text + '\n#[path = "private/reference_input_readiness.rs"]\nmod duplicate;\n')
                else:
                    path.write_text(text + "\n// changed after capture\n")
                if variant != "source_drift":
                    identity[parent] = hashlib.sha256(path.read_bytes()).hexdigest()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_failed_target(cause, root, identity, role=ROLE)

    def causal_fixture(self, directory):
        """Current real Rust bytes support fixed diagnostic specimens without compilation."""
        root, identity = self.private_fixture(directory)
        for name in ("crates/time-core/src/reference_input.rs", "crates/time-core/src/lib.rs",
                     "crates/time-core/src/reference_input/rate_delivery.rs"):
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes((ROOT / name).read_bytes())
            identity[name] = hashlib.sha256(path.read_bytes()).hexdigest()
        return root, identity

    def result_sections(self):
        return [{"test": "reference_input::rate_delivery::phase_plan::tests::step_counter::known_step_arithmetic_failure_commits_final_cursor_target_and_actual_facts",
                 "body": 'Error: Custom { kind: Other, error: "numeric failure cannot undo a Known step" }',
                 "section_sha256": "a" * 64}]

    def adaptive_auxiliary_fixture(self, directory):
        """Retain the actual sole adaptive edit and current owned test/include bytes."""
        root, identity = self.causal_fixture(directory)
        filename = OWNERS["adaptive_horizon"]
        mutant = {"name": filename + ":279:35: replace / with * in adaptive_ceil_nanoseconds",
            "package": "time-core", "file": filename, "function": {
                "function_name": "adaptive_ceil_nanoseconds",
                "return_type": "-> Result<u64, FixturePhaseError>",
                "span": {"start": {"line": 251, "column": 1}, "end": {"line": 289, "column": 2}}},
            "span": {"start": {"line": 279, "column": 35}, "end": {"line": 279, "column": 36}},
            "replacement": "*", "genre": "BinaryOperator"}
        source = (root / filename).read_text()
        changed = source.replace("let whole = numerator / denominator;",
            "let whole = numerator * /* ~ changed by cargo-mutants ~ */ denominator;")
        diff = "".join(difflib.unified_diff(source.splitlines(keepends=True),
            changed.splitlines(keepends=True), fromfile=filename,
            tofile="replace / with * in adaptive_ceil_nanoseconds", n=8))
        self.assertEqual(MODULE.COMMON.canonical_sha256(mutant),
            "3d7b8cefa055743424d769a21990b58b68abf5b452c4835f84c2c20d42415582")
        self.assertEqual(hashlib.sha256(diff.encode()).hexdigest(),
            "b2f387332cd3d53f202d274f70cff7548d313ab712370dd4fed5964a333347cd")
        manifest = "policy/tool-manifest.json"
        path = root / manifest
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes((ROOT / manifest).read_bytes())
        identity[manifest] = hashlib.sha256(path.read_bytes()).hexdigest()
        return root, identity, mutant, diff

    def adaptive_auxiliary_output(self, root, *, include_assertions=True, omit_auxiliary=False):
        """Join four actual overflow frames with two independently owned healthy assertions."""
        prefix = "reference_input::rate_delivery::phase_plan::adaptive::tests::"
        auxiliary = [prefix + name for name in (
            "adaptive_clamps_keep_original_integer_limits_and_refuse_nonzero_underflow",
            "adaptive_policy_uses_exact_selected_horizon_for_its_existing_rate_bound",
            "adaptive_private_numeric_boundaries_keep_healthy_neighbors",
            "adaptive_small_debt_threshold_equality_and_adjacent_bits_are_distinct")]
        bodies = {test: f"thread '{test}' panicked at {OWNERS['adaptive_horizon']}:279:25:\n"
            "attempt to multiply with overflow\n" for test in auxiliary}
        if omit_auxiliary:
            del bodies[auxiliary[-1]]
        if include_assertions:
            source = "crates/time-core/tests/private/adaptive_horizon.rs"
            lines = (root / source).read_text().splitlines()
            for name, statement in (
                ("adaptive_duration_ceiling_preserves_fractional_nanoseconds_at_large_seconds",
                 "product_ok(super::adaptive_ceil_nanoseconds(seconds))?"),
                ("adaptive_near_minimum_quantizes_before_integer_clamping",
                 "product_ok(area.horizon(policy, debt))?")):
                start = next(i for i, line in enumerate(lines) if line.startswith("fn " + name + "("))
                line = next(i for i in range(start + 1, len(lines)) if statement in lines[i])
                column = lines[line].index("product_ok") + 1
                test = prefix + name
                bodies[test] = (f"thread '{test}' panicked at {source}:{line + 1}:{column}:\n"
                    'valid NoEffect fixture was refused: Some("fixture phase: adaptive horizon rejected")\n')
        names = sorted(bodies)
        text = "".join(f"test {test} ... FAILED\n" for test in names) + "\nfailures:\n\n"
        text += "".join(f"---- {test} stdout ----\n{bodies[test]}\n" for test in names)
        text += "failures:\n" + "".join(f"    {test}\n" for test in names)
        text += (f"\ntest result: FAILED. 0 passed; {len(names)} failed; 0 ignored; 0 measured; "
                 "0 filtered out; finished in 0.00s\n"
                 "error: test failed, to rerun pass `-p time-core --lib`\n")
        return text, {"test_inventories": {"baseline": names}}

    def test_actual_adaptive_overflow_frames_remain_zero_credit_beside_real_assertions(self):
        with tempfile.TemporaryDirectory() as directory:
            root, identity, mutant, diff = self.adaptive_auxiliary_fixture(directory)
            text, policy = self.adaptive_auxiliary_output(root)
            observations = []
            _, causes = MODULE._frequency_caught_text(text, root, identity, policy,
                mutant=mutant, mutant_diff=diff, role=ROLE, uncredited=observations)
            self.assertEqual(len(causes), 2)
            self.assertTrue(all(cause["kind"] == "product_ok" for cause in causes))
            self.assertEqual(len(observations), 4)
            self.assertTrue(all(row["kind"] == "auxiliary_arithmetic_panic"
                and row["kill_credit"] == 0 and row["cargo_target"] == "--lib" for row in observations))

    def test_adaptive_auxiliary_panics_require_complete_roster_and_independent_assertion(self):
        for assertions, omit, retained in ((False, False, True), (True, True, True), (True, False, False)):
            with self.subTest(assertions=assertions, omit=omit, retained=retained), tempfile.TemporaryDirectory() as directory:
                root, identity, mutant, diff = self.adaptive_auxiliary_fixture(directory)
                text, policy = self.adaptive_auxiliary_output(root,
                    include_assertions=assertions, omit_auxiliary=omit)
                with self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_caught_text(text, root, identity, policy,
                        mutant=mutant, mutant_diff=diff, role=ROLE, uncredited=[] if retained else None)

    def test_adaptive_auxiliary_frame_rejects_foreign_changed_or_infrastructure_causes(self):
        variants = ("role", "descriptor", "diff", "span", "source", "test_source", "compiler",
            "test", "location", "message", "prefix", "suffix", "duplicate", "include")
        for variant in variants:
            with self.subTest(variant=variant), tempfile.TemporaryDirectory() as directory:
                root, identity, mutant, diff = self.adaptive_auxiliary_fixture(directory)
                test = ("reference_input::rate_delivery::phase_plan::adaptive::tests::"
                    "adaptive_clamps_keep_original_integer_limits_and_refuse_nonzero_underflow")
                body = f"thread '{test}' panicked at {OWNERS['adaptive_horizon']}:279:25:\nattempt to multiply with overflow\n"
                role = ROLE
                if variant == "role":
                    text, policy = self.adaptive_auxiliary_output(root)
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE._frequency_caught_text(text, root, identity, policy, mutant=mutant,
                            mutant_diff=diff, role=MODULE._baseline_feedback_role(), uncredited=[])
                    continue
                elif variant == "descriptor": mutant["name"] += " foreign"
                elif variant == "diff": diff += "\n"
                elif variant == "span": mutant["span"]["end"]["column"] += 1
                elif variant == "test": test = "foreign::" + test; body = body.replace("thread '", "thread 'foreign::")
                elif variant == "location": body = body.replace(":279:25:", ":279:26:")
                elif variant == "message": body = body.replace("multiply with overflow", "divide by zero")
                elif variant == "prefix": body = "failed to spawn\n" + body
                elif variant == "suffix": body += "Error: out of memory\n"
                elif variant == "duplicate": body += body
                elif variant in {"source", "test_source", "compiler"}:
                    name = {"source": OWNERS["adaptive_horizon"], "compiler": "policy/tool-manifest.json",
                        "test_source": "crates/time-core/tests/private/adaptive_horizon.rs"}[variant]
                    path = root / name
                    path.write_bytes(path.read_bytes() + b"\n// changed source\n")
                else:
                    path = root / OWNERS["adaptive_horizon"]
                    path.write_text(path.read_text() + '\n#[path = "../../../../tests/private/adaptive_horizon.rs"]\nmod duplicate;\n')
                    identity[OWNERS["adaptive_horizon"]] = hashlib.sha256(path.read_bytes()).hexdigest()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE._baseline_feedback_auxiliary_panic(body, test, root, identity, mutant, diff, role)

    def test_actual_reference_mapping_result_sections_are_explicitly_zero_credit(self):
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.causal_fixture(directory)
            for key in ("324:42", "325:51", "331:29"):
                case = _PHASE_STEP_CAUSAL_SPECIMENS[key]
                with self.subTest(key=key):
                    observations = MODULE._baseline_feedback_result_sections(self.result_sections(),
                        root, identity, case["mutant"], case["diff"], ROLE)
                    self.assertEqual(len(observations), 1)
                    self.assertEqual(observations[0]["kill_credit"], 0)
                    self.assertEqual(observations[0]["cargo_target"], "--lib")

    def test_actual_reference_round_const_trait_triplet_keeps_all_owned_diagnostics(self):
        case = _PHASE_STEP_CAUSAL_SPECIMENS["608:9"]
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.causal_fixture(directory)
            causes = MODULE._frequency_compiler_errors(copy.deepcopy(case["records"]), root, identity,
                mutant=case["mutant"], mutant_diff=case["diff"], role=ROLE)
            self.assertEqual([cause["code"] for cause in causes], ["E0658", None, "E0015"])
            self.assertTrue(all(cause["primary"][0]["source"] == "crates/time-core/src/reference_input.rs"
                                and cause["primary"][0]["line"] == 608 for cause in causes))

    def test_phase_step_result_observations_reject_foreign_and_changed_causal_inputs(self):
        case = _PHASE_STEP_CAUSAL_SPECIMENS["324:42"]
        for variant in ("unknown_mutant", "absent_fault", "diff", "body", "foreign_test", "extra_section", "empty", "nondict", "section_hash", "source", "include"):
            with self.subTest(variant=variant), tempfile.TemporaryDirectory() as directory:
                root, identity = self.causal_fixture(directory)
                mutant, diff = copy.deepcopy(case["mutant"]), case["diff"]
                sections = self.result_sections()
                if variant == "unknown_mutant":
                    mutant["name"] += " foreign descriptor"
                elif variant == "absent_fault":
                    mutant = diff = None
                elif variant == "diff":
                    diff += "\n"
                elif variant == "body":
                    sections[0]["body"] += "\nError: failed to spawn"
                elif variant == "foreign_test":
                    sections[0]["test"] = "foreign::" + sections[0]["test"]
                elif variant == "extra_section":
                    sections *= 2
                elif variant == "empty":
                    sections = []
                elif variant == "nondict":
                    sections = [None]
                elif variant == "section_hash":
                    sections[0]["section_sha256"] = "not a digest"
                else:
                    name = ("crates/time-core/tests/private/step_counter.rs" if variant == "source"
                            else MODULE._PHASE_STEP_LIFECYCLE)
                    path = root / name
                    path.write_text(path.read_text() + ("\n// source drift\n" if variant == "source"
                        else '\n#[path = "step_counter.rs"]\nmod duplicate;\n'))
                    identity[name] = hashlib.sha256(path.read_bytes()).hexdigest()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE._baseline_feedback_result_sections(sections, root, identity, mutant, diff, ROLE)
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.causal_fixture(directory)
            with self.assertRaises(MODULE.Incomplete):
                MODULE._baseline_feedback_result_sections(self.result_sections(), root, identity,
                    case["mutant"], case["diff"], MODULE._frequency_prior_role())

    def test_result_error_without_a_separate_assertion_still_cannot_kill(self):
        case = _PHASE_STEP_CAUSAL_SPECIMENS["324:42"]
        section = self.result_sections()[0]
        test, body = section["test"], section["body"]
        text = (f"test {test} ... FAILED\n\nfailures:\n\n---- {test} stdout ----\n{body}\n\n"
                f"failures:\n    {test}\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n"
                "error: test failed, to rerun pass `-p time-core --lib`\n")
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.causal_fixture(directory)
            policy = {"test_inventories": {"baseline": [test]}}
            with self.assertRaisesRegex(MODULE.Incomplete, "no separate owned semantic assertion"):
                MODULE._frequency_caught_text(text, root, identity, policy, mutant=case["mutant"],
                    mutant_diff=case["diff"], role=ROLE, uncredited=[])

    def test_phase_step_const_companion_refuses_unbalanced_foreign_or_changed_triplets(self):
        case = _PHASE_STEP_CAUSAL_SPECIMENS["608:9"]
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.causal_fixture(directory)
            compiled = MODULE._frequency_mutation_source(root, identity, case["mutant"], case["diff"], role=ROLE)
            for variant in ("missing", "duplicate", "double_triplet", "message", "span", "target", "mutant", "diff", "compiled", "source", "role"):
                records, mutant = copy.deepcopy(case["records"]), copy.deepcopy(case["mutant"])
                changed, diff, bindings, role = compiled, case["diff"], dict(identity), ROLE
                if variant == "missing":
                    records.pop(0)
                elif variant == "duplicate":
                    records.append(copy.deepcopy(records[0]))
                elif variant == "double_triplet":
                    records *= 2
                elif variant == "message":
                    records[0]["message"]["message"] += " unrelated"
                elif variant == "span":
                    records[0]["message"]["spans"][0]["column_start"] += 1
                elif variant == "target":
                    for record in records:
                        record["target"]["test"] = False
                elif variant == "mutant":
                    mutant["name"] += " unknown"
                elif variant == "diff":
                    diff += "\n"
                elif variant == "compiled":
                    changed = (compiled[0], compiled[1] + b"\n")
                elif variant == "source":
                    bindings[compiled[0]] = "a" * 64
                elif variant == "role":
                    role = MODULE._baseline_feedback_role()
                uncoded = next(record for record in records if record["message"]["code"] is None)
                with self.subTest(variant=variant):
                    try:
                        admitted = MODULE._phase_step_compiler_cause(records, uncoded,
                            mutant, diff, changed, root, bindings, role)
                    except MODULE.Incomplete:
                        admitted = False
                    self.assertFalse(admitted)
            for variant in ("duplicate", "message"):
                records = copy.deepcopy(case["records"])
                if variant == "duplicate":
                    records += copy.deepcopy(records)
                else:
                    records[1]["message"]["message"] = "arbitrary uncoded error"
                with self.subTest(receiver=variant), self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_compiler_errors(records, root, identity, mutant=case["mutant"],
                        mutant_diff=case["diff"], role=ROLE)

    def test_actual_rate_delivery_compiler_family_neighbors_remain_unviable_only(self):
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.causal_fixture(directory)
            for key, expected in (("rate:464:9", ["E0658", None, "E0015"]),
                                  ("rate:608:9", [None]), ("rate:613:9", [None])):
                case = _PHASE_STEP_CAUSAL_SPECIMENS[key]
                with self.subTest(key=key):
                    causes = MODULE._frequency_compiler_errors(copy.deepcopy(case["records"]), root, identity,
                        mutant=case["mutant"], mutant_diff=case["diff"], role=ROLE)
                    self.assertEqual([cause["code"] for cause in causes], expected)

    def test_actual_rate_delivery_result_neighbors_are_explicitly_zero_credit(self):
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.causal_fixture(directory)
            for key in ("rate:546:9", "rate:800:40", "rate:801:49"):
                case = _PHASE_STEP_CAUSAL_SPECIMENS[key]
                with self.subTest(key=key):
                    observed = MODULE._baseline_feedback_result_sections(case["result_sections"],
                        root, identity, case["mutant"], case["diff"], ROLE)
                    self.assertEqual([value["kill_credit"] for value in observed], [0])
                    self.assertEqual([value["cargo_target"] for value in observed], ["--lib"])

    def test_actual_nine_parent_imported_guard_rows_keep_their_real_assertion_owner(self):
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.causal_fixture(directory)
            observed = 0
            for key, case in _PHASE_STEP_PARENT_CAUSAL_SPECIMENS.items():
                if not case["guard_assertions"]:
                    continue
                observed += 1
                with self.subTest(key=key):
                    for witness in case["guard_assertions"]:
                        witness = self.current_counter_witness(root, witness)
                        cause = MODULE._frequency_assertion(root, identity, witness["source"],
                            witness["line"], witness["column"], witness["message"],
                            test=witness["test"], role=ROLE)
                        self.assertEqual(cause["kind"], "product_ok")
                        cause["test"] = witness["test"]
                        self.assertEqual(MODULE._frequency_failed_target(cause, root, identity, role=ROLE), "--lib")
            self.assertEqual(observed, 9)

    def current_counter_witness(self, root, witness):
        """Locate the same retained call statement in current source for a decoder unit control.

        The original literal keeps its historical coordinates. This copied input
        is explicitly synthetic after test edits; no raw receipt is reclassified.
        """
        anchors = {
            124: "    product_ok(controller_map_rule(owner.chart(), &healthy))?;",
            158: "    let issued = product_ok(owner.issue_step(",
            179: "    let mut current = product_ok(retired.resume_fresh(controller_clock_point(2, 8)?))?;",
            195: "    product_ok(owner.issue_step(",
        }
        source = (root / MODULE._PHASE_STEP_COUNTER_SOURCE[0]).read_text()
        matches = list(re.finditer(r"(?m)^" + re.escape(anchors[witness["line"]]) + r"$", source))
        if len(matches) != 1:
            name = witness["test"].rsplit("::", 1)[-1]
            declaration = list(re.finditer(r"(?m)^fn " + re.escape(name) + r"\s*\(", source))
            self.assertEqual(len(declaration), 1)
            first = declaration[0].start()
            last = source.index("\n}", first)
            matches = [match for match in matches if first <= match.start() < last]
        self.assertEqual(len(matches), 1)
        return {**witness, "line": source[:matches[0].start()].count("\n") + 1}

    def test_actual_five_parent_result_rows_remain_zero_credit_with_exact_targets(self):
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.causal_fixture(directory)
            observed = sections = 0
            for key, case in _PHASE_STEP_PARENT_CAUSAL_SPECIMENS.items():
                if "result_sections" not in case:
                    continue
                observed += 1
                sections += len(case["result_sections"])
                with self.subTest(key=key):
                    output = MODULE._baseline_feedback_result_sections(case["result_sections"],
                        root, identity, case["mutant"], case["diff"], ROLE)
                    self.assertEqual(len(output), len(case["result_sections"]))
                    self.assertEqual([row["kill_credit"] for row in output], [0] * len(output))
                    target = "--test phase_plan" if key.startswith("1846:") else "--lib"
                    self.assertEqual([row["cargo_target"] for row in output], [target] * len(output))
            self.assertEqual((observed, sections), (5, 8))

    def test_imported_guard_refuses_changed_sources_and_foreign_owners(self):
        witness = _PHASE_STEP_PARENT_CAUSAL_SPECIMENS["899:50:=="]["guard_assertions"][0]
        for variant in ("missing_import", "alias", "parent_drift", "parent_identity", "child_identity",
                        "foreign_test", "foreign_role", "message", "sibling"):
            with self.subTest(variant=variant), tempfile.TemporaryDirectory() as directory:
                root, identity = self.causal_fixture(directory)
                case, role = self.current_counter_witness(root, witness), ROLE
                child = MODULE._PHASE_STEP_COUNTER_SOURCE[0]
                parent = MODULE._PHASE_STEP_COUNTER_GUARD_PARENT[0]
                if variant in {"missing_import", "alias"}:
                    path = root / child
                    replacement = "producer," if variant == "missing_import" else "producer, product_ok as other_guard,"
                    path.write_text(path.read_text().replace("producer, product_ok,", replacement, 1))
                    identity[child] = hashlib.sha256(path.read_bytes()).hexdigest()
                elif variant == "parent_drift":
                    path = root / parent
                    path.write_text(path.read_text() + "\n// changed helper owner\n")
                    identity[parent] = hashlib.sha256(path.read_bytes()).hexdigest()
                elif variant == "parent_identity":
                    identity[parent] = "a" * 64
                elif variant == "child_identity":
                    identity[child] = "a" * 64
                elif variant == "foreign_test":
                    case["test"] = "reference_input::rate_delivery::phase_plan::tests::private_controller_map_observe_rechecks_context_and_expiry"
                elif variant == "foreign_role":
                    role = MODULE._frequency_prior_role()
                elif variant == "message":
                    case["message"] = "a foreign construction error"
                else:
                    sibling = "crates/time-core/tests/private/step_diagnostics.rs"
                    self.assertEqual(MODULE._phase_step_tracked_guard_source(root, identity, sibling,
                        (root / sibling).read_text(), case["test"], role), (root / sibling).read_text())
                    continue
                with self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_assertion(root, identity, case["source"], case["line"],
                        case["column"], case["message"], test=case["test"], role=role)

    def test_imported_guard_shape_checks_survive_isolated_test_only_pin_changes(self):
        """Reach the actual structural guards independently of their immutable source pins."""
        witness = _PHASE_STEP_PARENT_CAUSAL_SPECIMENS["899:50:=="]["guard_assertions"][0]
        for variant in ("missing_import", "alias", "wrong_helper", "nontrack", "changed_prefix", "duplicate_helper"):
            with self.subTest(variant=variant), tempfile.TemporaryDirectory() as directory:
                root, identity = self.causal_fixture(directory)
                case = self.current_counter_witness(root, witness)
                child = MODULE._PHASE_STEP_COUNTER_SOURCE[0]
                parent = MODULE._PHASE_STEP_COUNTER_GUARD_PARENT[0]
                name = child if variant in {"missing_import", "alias"} else parent
                path = root / name
                source = path.read_text()
                if variant == "missing_import":
                    source = source.replace("producer, product_ok,", "producer,", 1)
                elif variant == "alias":
                    source = source.replace("producer, product_ok,", "producer, product_ok as other_guard,", 1)
                elif variant == "wrong_helper":
                    source = source.replace("fn product_ok<", "fn unrelated_guard<", 1)
                elif variant == "nontrack":
                    source = source.replace("#[track_caller]\nfn product_ok", "\nfn product_ok", 1)
                elif variant == "changed_prefix":
                    source = source.replace("valid NoEffect fixture was refused:", "foreign guard was refused:", 1)
                else:
                    source += "\n#[track_caller]\nfn product_ok(value: Result<(), ()>) {}\n"
                path.write_text(source)
                identity[name] = hashlib.sha256(path.read_bytes()).hexdigest()
                constant = "_PHASE_STEP_COUNTER_SOURCE" if name == child else "_PHASE_STEP_COUNTER_GUARD_PARENT"
                with patch.object(MODULE, constant, (name, identity[name])), self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_assertion(root, identity, case["source"], case["line"],
                        case["column"], case["message"], test=case["test"], role=ROLE)

    def test_parent_result_pairs_refuse_changed_body_source_test_and_duplicate_neighbors(self):
        for key in ("1841:9:None", "1846:9:None", "3154:20:false"):
            case = _PHASE_STEP_PARENT_CAUSAL_SPECIMENS[key]
            for variant in ("body", "infrastructure", "sibling", "duplicate", "source", "foreign_role"):
                with self.subTest(key=key, variant=variant), tempfile.TemporaryDirectory() as directory:
                    root, identity = self.causal_fixture(directory)
                    sections, role = copy.deepcopy(case["result_sections"]), ROLE
                    if variant == "body":
                        sections[0]["body"] += " changed"
                    elif variant == "infrastructure":
                        sections[0]["body"] += "\nError: failed to spawn"
                    elif variant == "sibling":
                        sections[0]["test"] = "step_diagnostics::matched_step_neutral_diagnostic_has_no_delegated_cause"
                    elif variant == "duplicate":
                        sections.append(copy.deepcopy(sections[0]))
                    elif variant == "source":
                        name = ("crates/time-core/tests/private/step_diagnostics.rs" if key.startswith("1846:")
                            else MODULE._PHASE_STEP_COUNTER_SOURCE[0])
                        path = root / name
                        path.write_text(path.read_text() + "\n// changed Result branch\n")
                        identity[name] = hashlib.sha256(path.read_bytes()).hexdigest()
                    else:
                        role = MODULE._frequency_prior_role()
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE._baseline_feedback_result_sections(sections, root, identity,
                            case["mutant"], case["diff"], role)

    def test_new_parent_result_pairs_without_a_real_assertion_cannot_kill(self):
        for key, case in _PHASE_STEP_PARENT_CAUSAL_SPECIMENS.items():
            if "result_sections" not in case:
                continue
            sections = case["result_sections"]
            names = [row["test"] for row in sections]
            text = "".join(f"test {name} ... FAILED\n" for name in names) + "\nfailures:\n\n"
            text += "".join(f"---- {row['test']} stdout ----\n{row['body']}\n\n" for row in sections)
            text += "failures:\n" + "".join(f"    {name}\n" for name in names)
            text += f"\ntest result: FAILED. 0 passed; {len(names)} failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\n"
            text += "error: test failed, to rerun pass `-p time-core --lib`\n"
            with self.subTest(key=key), tempfile.TemporaryDirectory() as directory:
                root, identity = self.causal_fixture(directory)
                with self.assertRaisesRegex(MODULE.Incomplete, "no separate owned semantic assertion"):
                    MODULE._frequency_caught_text(text, root, identity, {"test_inventories": {"baseline": names}},
                        mutant=case["mutant"], mutant_diff=case["diff"], role=ROLE, uncredited=[])

    def integration_guard_fixture(self, directory, module, function):
        """Locate the real call in its current declared test, independent of line movement."""
        root, identity = self.causal_fixture(directory)
        name = f"crates/time-core/tests/private/{module}.rs"
        source = (root / name).read_text()
        declarations = list(re.finditer(r"(?m)^fn " + function + r"\s*\(", source))
        self.assertEqual(len(declarations), 1)
        start = declarations[0].start()
        end = source.index("\n}", start)
        token = re.search(r"\bproduct_ok\s*\(", source[start:end])
        self.assertIsNotNone(token)
        before = source[:start + token.start()]
        witness = {"source": name, "line": before.count("\n") + 1,
            "column": len(before.rsplit("\n", 1)[-1]) + 1, "test": module + "::" + function,
            "message": "the independently specified valid phase-plan trajectory must succeed; bounded refusal: Some(\"fixture step: direction refused\")"}
        return root, identity, witness

    def test_actual_four_step_mutant_guard_refusals_have_their_real_integration_parent(self):
        cases = (
            ("66:44:<", "step_contracts", "matched_step_known_actual_violations_remain_owned_historical_facts"),
            ("259:18:>=", "step_contracts", "matched_step_raw_limits_and_receipts_reject_bypasses_before_valid_issue"),
            ("786:47:<", "step_qualification", "matched_step_lower_consumed_ordinal_rejects_beside_current_completion"),
            ("787:42:<", "step_qualification", "matched_step_lower_consumed_ordinal_rejects_beside_current_completion"),
        )
        for key, module, function in cases:
            with self.subTest(actual_mutant=key), tempfile.TemporaryDirectory() as directory:
                root, identity, witness = self.integration_guard_fixture(directory, module, function)
                with patch.object(MODULE, "_PHASE_STEP_INTEGRATION_GUARD_CHILDREN", frozenset()), \
                     self.assertRaisesRegex(MODULE.Incomplete, "unique fixed success assertion"):
                    MODULE._frequency_assertion(root, identity, witness["source"], witness["line"],
                        witness["column"], witness["message"], test=witness["test"], role=ROLE)
                cause = MODULE._frequency_assertion(root, identity, witness["source"], witness["line"],
                    witness["column"], witness["message"], test=witness["test"], role=ROLE)
                self.assertEqual((cause["source"], cause["kind"]), (witness["source"], "product_ok"))
                self.assertEqual(MODULE._frequency_failed_target({**cause, "test": witness["test"]},
                    root, identity, role=ROLE), "--test phase_plan")

    def test_integration_parent_guard_rejects_foreign_duplicate_and_changed_owners(self):
        for module in ("step_contracts", "step_qualification"):
            function = ("matched_step_known_actual_violations_remain_owned_historical_facts" if module == "step_contracts"
                else "matched_step_lower_consumed_ordinal_rejects_beside_current_completion")
            for variant in ("missing_import", "alias", "duplicate_import", "parent_drift", "parent_identity",
                            "wrong_helper", "nontrack", "wrong_guard", "wrong_prefix", "duplicate_helper",
                            "foreign_test", "foreign_role", "duplicate_include", "wrong_include"):
                with self.subTest(module=module, variant=variant), tempfile.TemporaryDirectory() as directory:
                    root, identity, witness = self.integration_guard_fixture(directory, module, function)
                    role, child, parent = ROLE, root / witness["source"], root / MODULE._PHASE_STEP_TEST_ROOT
                    if variant in {"missing_import", "alias", "duplicate_import"}:
                        replacement = {"missing_import": "", "alias": "product_ok as another_guard,",
                            "duplicate_import": "product_ok, product_ok,"}[variant]
                        child.write_text(child.read_text().replace("product_ok,", replacement, 1))
                        identity[witness["source"]] = hashlib.sha256(child.read_bytes()).hexdigest()
                    elif variant == "parent_identity":
                        identity[MODULE._PHASE_STEP_TEST_ROOT] = "a" * 64
                    elif variant == "foreign_test":
                        witness["test"] = "step_diagnostics::matched_step_constructor_diagnostics_are_opaque_and_preserve_checked_receipts"
                    elif variant == "foreign_role":
                        role = MODULE._frequency_prior_role()
                    else:
                        text = parent.read_text()
                        if variant == "parent_drift":
                            text += "\n// unadmitted parent byte drift\n"
                        elif variant == "wrong_helper":
                            text = text.replace("fn product_ok<", "fn unrelated_guard<", 1)
                        elif variant == "nontrack":
                            text = text.replace("#[track_caller]\nfn product_ok", "\nfn product_ok", 1)
                        elif variant == "wrong_guard":
                            text = text.replace("value.is_ok(),", "value.is_err(),", 1)
                        elif variant == "wrong_prefix":
                            text = text.replace("the independently specified valid phase-plan trajectory", "foreign trajectory", 1)
                        elif variant == "duplicate_helper":
                            text += "\n#[track_caller]\nfn product_ok(value: Result<(), ()>) {}\n"
                        elif variant == "duplicate_include":
                            text += f'\n#[path = "private/{module}.rs"]\nmod foreign_duplicate;\n'
                        else:
                            text = text.replace(f"mod {module};", "mod foreign_module;", 1)
                        parent.write_text(text)
                        if variant != "parent_drift":
                            identity[MODULE._PHASE_STEP_TEST_ROOT] = hashlib.sha256(parent.read_bytes()).hexdigest()
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE._frequency_assertion(root, identity, witness["source"], witness["line"],
                            witness["column"], witness["message"], test=witness["test"], role=role)

    def source_diff(self, mutant, before, after):
        """Build an independent whole-file diff retaining the descriptor's literal headers."""
        return "".join(difflib.unified_diff(before.splitlines(keepends=True), after.splitlines(keepends=True),
            fromfile=mutant["file"], tofile=mutant["name"].split(": ", 1)[1], n=5))

    def test_structural_families_reject_a_second_edit_outside_the_descriptor_span(self):
        for key in ("rate:464:9", "rate:608:9", "rate:546:9"):
            case = _PHASE_STEP_CAUSAL_SPECIMENS[key]
            with self.subTest(key=key), tempfile.TemporaryDirectory() as directory:
                root, identity = self.causal_fixture(directory)
                filename, changed = MODULE._frequency_mutation_source(root, identity,
                    case["mutant"], case["diff"], role=ROLE)
                pristine = (root / filename).read_text()
                # The extra edit is otherwise a valid, source-bound unified diff.
                foreign = changed.decode() + "\n// unlisted second edit\n"
                diff = self.source_diff(case["mutant"], pristine, foreign)
                with self.assertRaisesRegex(MODULE.Incomplete, "sole exact descriptor-span"):
                    MODULE._frequency_mutation_source(root, identity, case["mutant"], diff, role=ROLE)

    def test_compiler_families_use_relocated_utf8_source_coordinates(self):
        prefix = "// independent UTF-8 relocation: \u03c0\n"
        for key, expected in (("608:9", ["E0658", None, "E0015"]),
                              ("rate:464:9", ["E0658", None, "E0015"]), ("rate:608:9", [None])):
            case = copy.deepcopy(_PHASE_STEP_CAUSAL_SPECIMENS[key])
            with self.subTest(key=key), tempfile.TemporaryDirectory() as directory:
                root, identity = self.causal_fixture(directory)
                filename, changed = MODULE._frequency_mutation_source(root, identity,
                    case["mutant"], case["diff"], role=ROLE)
                path = root / filename
                pristine = prefix + path.read_text()
                path.write_text(pristine)
                identity[filename] = hashlib.sha256(path.read_bytes()).hexdigest()
                mutant = case["mutant"]
                for span in (mutant["span"], mutant["function"]["span"]):
                    for point in span.values():
                        point["line"] += 1
                tail = mutant["name"].split(": ", 1)[1]
                start = mutant["span"]["start"]
                mutant["name"] = f"{filename}:{start['line']}:{start['column']}: {tail}"
                for record in case["records"]:
                    for span in record["message"]["spans"]:
                        span["line_start"] += 1
                        span["line_end"] += 1
                        span["byte_start"] += len(prefix.encode())
                        span["byte_end"] += len(prefix.encode())
                diff = self.source_diff(mutant, pristine, prefix + changed.decode())
                causes = MODULE._frequency_compiler_errors(case["records"], root, identity,
                    mutant=mutant, mutant_diff=diff, role=ROLE)
                self.assertEqual([cause["code"] for cause in causes], expected)

    def test_const_family_requires_its_actual_function_and_closed_payload(self):
        case = _PHASE_STEP_CAUSAL_SPECIMENS["rate:464:9"]
        for variant in ("function_name", "function_span", "return_type", "untyped_return", "nonconst",
                        "foreign_genre", "payload", "foreign_source", "foreign_package"):
            with self.subTest(variant=variant), tempfile.TemporaryDirectory() as directory:
                root, identity = self.causal_fixture(directory)
                mutant, diff = copy.deepcopy(case["mutant"]), case["diff"]
                records = copy.deepcopy(case["records"])
                if variant == "function_name":
                    mutant["function"]["function_name"] = "foreign::other_function"
                elif variant == "function_span":
                    mutant["function"]["span"]["start"]["line"] = 468
                elif variant == "return_type":
                    mutant["function"]["return_type"] = "-> NonZeroU64"
                elif variant == "untyped_return":
                    mutant["function"]["return_type"] = None
                elif variant == "nonconst":
                    path = root / mutant["file"]
                    source = MODULE._frequency_mutation_source(root, identity, mutant, diff, role=ROLE)[1].decode()
                    before = path.read_text().replace("pub const fn next_ordinal", "pub       fn next_ordinal")
                    path.write_text(before)
                    identity[mutant["file"]] = hashlib.sha256(path.read_bytes()).hexdigest()
                    diff = self.source_diff(mutant, before,
                        source.replace("pub const fn next_ordinal", "pub       fn next_ordinal"))
                elif variant == "foreign_genre":
                    mutant["genre"] = "BinaryOperator"
                elif variant == "payload":
                    mutant["replacement"] = "Some(2.try_into().unwrap())"
                elif variant == "foreign_source":
                    mutant["file"] = "crates/time-core/src/lib.rs"
                else:
                    mutant["package"] = "foreign-package"
                try:
                    compiled = MODULE._frequency_mutation_source(root, identity, mutant, diff, role=ROLE)
                    uncoded = next(row for row in records if row["message"]["code"] is None)
                    admitted = MODULE._phase_step_compiler_cause(records, uncoded, mutant, diff,
                        compiled, root, identity, ROLE)
                except MODULE.Incomplete:
                    admitted = False
                self.assertFalse(admitted)

    def test_let_chain_family_refuses_foreign_unbalanced_or_changed_compiler_facts(self):
        case = _PHASE_STEP_CAUSAL_SPECIMENS["rate:608:9"]
        for variant in ("duplicate", "extra_coded", "message", "span", "highlight", "producer",
                        "manifest", "target", "genre", "operator", "function", "role"):
            with self.subTest(variant=variant), tempfile.TemporaryDirectory() as directory:
                root, identity = self.causal_fixture(directory)
                mutant, records = copy.deepcopy(case["mutant"]), copy.deepcopy(case["records"])
                role = ROLE
                if variant == "duplicate":
                    records += copy.deepcopy(records)
                elif variant == "extra_coded":
                    extra = copy.deepcopy(records[0])
                    extra["message"]["code"] = {"code": "E0015", "explanation": None}
                    records.append(extra)
                elif variant == "message":
                    records[0]["message"]["message"] += " foreign failure"
                elif variant == "span":
                    records[0]["message"]["spans"][0]["byte_start"] += 1
                elif variant == "highlight":
                    records[0]["message"]["spans"][0]["text"][0]["text"] += " foreign"
                elif variant == "producer":
                    extra = copy.deepcopy(records[0])
                    extra["package_id"] += "foreign"
                    records.append(extra)
                elif variant == "manifest":
                    records[0]["manifest_path"] = "/foreign/Cargo.toml"
                    # A lone foreign producer is rejected by the outer Cargo reader.
                elif variant == "target":
                    records[0]["target"]["test"] = False
                elif variant == "genre":
                    mutant["genre"] = "FnValue"
                elif variant == "operator":
                    mutant["replacement"] = "&&"
                elif variant == "function":
                    mutant["function"]["function_name"] = "other_function"
                else:
                    role = MODULE._frequency_prior_role()
                try:
                    MODULE._frequency_compiler_errors(records, root, identity, mutant=mutant,
                        mutant_diff=case["diff"], role=role)
                except MODULE.Incomplete:
                    pass
                else:
                    self.fail("Changed let-chain compiler facts were admitted: " + variant)

    def test_changed_wrong_duplicate_and_foreign_private_includes_refuse(self):
        source = "crates/time-core/tests/private/step_contracts.rs"
        good = {"source": source, "test": "step_contracts::matched_step_raw_limits_and_receipts_reject_bypasses_before_valid_issue"}
        variants = (
            ('mod step_contracts;', 'mod wrong_owner;'),
            ('private/step_contracts.rs', 'private/wrong_step_contracts.rs'),
            ('mod step_contracts;', 'mod step_contracts;\n#[path = "private/step_contracts.rs"]\nmod second_owner;'),
        )
        for old, new in variants:
            with self.subTest(new=new), tempfile.TemporaryDirectory() as directory:
                root, identity = self.private_fixture(directory)
                path = root / MODULE._PHASE_STEP_TEST_ROOT
                path.write_text(path.read_text().replace(old, new))
                # Even newly hash-bound malformed module declarations are refused.
                identity[MODULE._PHASE_STEP_TEST_ROOT] = hashlib.sha256(path.read_bytes()).hexdigest()
                with self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_failed_target(good, root, identity, role=ROLE)
        with tempfile.TemporaryDirectory() as directory:
            root, identity = self.private_fixture(directory)
            for test in ("step_diagnostics::matched_step_raw_limits_and_receipts_reject_bypasses_before_valid_issue",
                         "step_contracts::nonexistent_neighbor", "step_contracts::outer_owner", None):
                with self.subTest(test=test), self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_failed_target({**good, "test": test}, root, identity, role=ROLE)
            (root / MODULE._PHASE_STEP_TEST_ROOT).write_text("// drift after freeze\n")
            with self.assertRaisesRegex(MODULE.Incomplete, "source changed"):
                MODULE._frequency_failed_target(good, root, identity, role=ROLE)


if __name__ == "__main__":
    unittest.main()
