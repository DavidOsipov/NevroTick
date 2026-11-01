"""Literal lifecycle panic transport controls; no native/tool execution."""
from pathlib import Path
import importlib.util
import unittest


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    'core_lifecycle_transport_controls', ROOT / 'scripts/verify-core-analysis.py')
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)

# Captured verbatim from s13-lifecycle-json-controls-v2/faulty/test.log. The
# known one-token fault changes DistanceLimit to RateLimit in a private copy.
ASSERTION = (
    'assertion failed: matches!(rejected,\n'
    '    Err(FixtureCombinationLifecycleError::Arithmetic\n'
    '    {\n'
    '        step, source: FitError::Arithmetic\n'
    '        {\n'
    '            stage: ArithmeticStage::Combination, operation, issue:\n'
    '            ArithmeticIssue::NonFinite,\n'
    '        },\n'
    '    }) if step == fact.step && operation == fact.operation)\n'
)
CARGO = '{"reason":"build-finished","success":true}\n'


# Literal native assertions in retained lifecycle-v2 outcomes8,17,87,104.
ADDITIONAL_ASSERTIONS = ('assertion failed: matches!(rejected,\n'
 '    Err(FixtureCombinationLifecycleError::InvalidParameter\n'
 '    {\n'
 '        field: FixtureCombinationParameterField::MaxClockError, issue: actual,\n'
 '    }) if actual == issue)\n',
 'assertion failed: matches!(rejected,\n'
 '    Err(FixtureCombinationLifecycleError::InvalidParameter\n'
 '    { field: FixtureCombinationParameterField::CombineLimit, issue: actual, })\n'
 '    if actual == issue)\n',
 'assertion failed: matches!(rejected,\n'
 '    Err(FixtureCombinationLifecycleError::Combination\n'
 '    {\n'
 '        source: FixtureCombinationError::ZeroDenominator\n'
 '        { index: 0, field: FixtureCombinationDenominatorField::RootDistance }\n'
 '    }))\n',
 'assertion failed: matches!(rejected,\n'
 '    Err(FixtureCombinationLifecycleError::Arithmetic\n'
 '    {\n'
 '        step: FixtureCombinationLifecycleStep::RateLimit, source:\n'
 '        FitError::Arithmetic\n'
 '        {\n'
 '            stage: ArithmeticStage::Combination, operation:\n'
 '            ArithmeticOperation::Multiply, issue: ArithmeticIssue::NonFinite\n'
 '        }\n'
 '    }))\n',
 'assertion failed: actual.is_some_and(|cause|\n'
 '        {\n'
 '            cause.downcast_ref::<FitError>().is_some_and(|inner|\n'
 '                    std::ptr::eq(inner, source))\n'
 '        })\n',
 'assertion failed: actual.is_some_and(|cause|\n'
 '        {\n'
 '            cause.downcast_ref::<FixtureCombinationError>().is_some_and(|inner|\n'
 '                    std::ptr::eq(inner, source))\n'
 '        })\n',
 'assertion failed: actual.is_some_and(|cause|\n'
 '        {\n'
 '            cause.downcast_ref::<FixturePrincipalError>().is_some_and(|inner|\n'
 '                    std::ptr::eq(inner, source))\n'
 '        })\n')

class LifecycleAssertionTransportTests(unittest.TestCase):
    def test_healthy_neighbor_retains_only_native_text(self):
        records, raw = MODULE.cargo_json_records(CARGO + 'test healthy ... ok\n', test_phase=True)
        self.assertEqual(records, [{'reason': 'build-finished', 'success': True}])
        self.assertEqual(raw, 'test healthy ... ok\n')

    def test_complete_observed_assertion_is_native_test_text(self):
        for newline in ('\n', '\r\n'):
            raw = ('before\n' + ASSERTION + 'after\n').replace('\n', newline)
            with self.subTest(newline=repr(newline)):
                records, observed = MODULE.cargo_json_records(
                    CARGO.replace('\n', newline) + raw, test_phase=True)
                self.assertEqual(records, [{'reason': 'build-finished', 'success': True}])
                self.assertEqual(observed, raw)

    def test_additional_observed_blocks_are_preserved_only_in_test_phase(self):
        for block in ADDITIONAL_ASSERTIONS:
            for newline in ('\n', '\r\n'):
                with self.subTest(block=block, newline=repr(newline)):
                    expected = block.replace('\n', newline)
                    records, raw = MODULE.cargo_json_records(
                        CARGO.replace('\n', newline) + expected, test_phase=True)
                    self.assertEqual(records, [{'reason': 'build-finished', 'success': True}])
                    self.assertEqual(raw, expected)
            with self.subTest(context='Build', block=block), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(CARGO + block)
            for invalid in (block.rstrip('\n'), block.rsplit('\n', 2)[0] + '\n',
                            block.replace('assertion failed:', 'assertion failed: altered', 1)):
                with self.subTest(invalid=invalid), self.assertRaises(MODULE.Incomplete):
                    MODULE.cargo_json_records(CARGO + invalid, test_phase=True)

    def test_build_transport_keeps_the_assertion_closed(self):
        with self.assertRaises(MODULE.Incomplete):
            MODULE.cargo_json_records(CARGO + ASSERTION)

    def test_partial_unknown_and_near_miss_assertions_remain_closed(self):
        invalid = (
            '    {\n', ASSERTION.splitlines(keepends=True)[2],
            ASSERTION.replace('FixtureCombinationLifecycleError', 'UnknownError'),
            ASSERTION.replace('ArithmeticStage::Combination', 'ArithmeticStage::Principal'),
            ASSERTION.replace('step == fact.step', 'step != fact.step'),
            ASSERTION.replace('ArithmeticIssue::NonFinite', 'ArithmeticIssue::Unknown'),
            ASSERTION.rsplit('\n', 2)[0] + '\n', ASSERTION.rstrip('\n'),
            ASSERTION.replace('        {\n', '        {oops\n'),
        )
        for raw in invalid:
            with self.subTest(raw=raw), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(CARGO + raw, test_phase=True)

    def test_literal_block_does_not_mask_malformed_cargo_json(self):
        for invalid in ('{oops}\n', '{"reason":"unknown"}\n',
                        '{"reason":"build-finished","success":true',
                        '{"reason":"build-finished","success":NaN}\n'):
            with self.subTest(invalid=invalid), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(CARGO + ASSERTION + invalid, test_phase=True)

    def test_assertion_alone_is_not_a_qualified_caught_test(self):
        outcome = {'summary': 'CaughtMutant', 'phase_results': []}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.caught_native_test_failure(outcome, ASSERTION)


if __name__ == '__main__':
    unittest.main()
