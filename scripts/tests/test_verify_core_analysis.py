"""Independent inert controls for core admission/accounting; no Cargo/product runs.

All fake source files, reports and command observations are explicitly synthetic.
These controls exercise actual Python readers/parsers/loops, not Rust behavior or
actual tool admission. The coordinator owns execution and receipt collection.
"""
from __future__ import annotations

import contextlib
import copy
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("core_analysis_controls", ROOT / "scripts/verify-core-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class PrincipalCheckpointTests(unittest.TestCase):
    """Current principal records must be charged, never silently ignored."""

    FIRST = ('CORE_PRINCIPAL_PROFILE case=principal_seed_and_genuine_update_hysteresis '
             'histories=2 appends=10 observations=10 contexts=2 monotonic=33 coordinates=1 '
             'exchanges=1 getters=259 terms=18 projections=18 enclosures=18 batches=9 '
             'minima=1 parameters=1 strata=2 seeds=1 advances=9\n')

    @classmethod
    def complete_rows(cls):
        """Literal actual ten-case native recipes, independent of parser tables."""
        fields = ('histories','appends','observations','contexts','monotonic','coordinates',
                  'exchanges','getters','terms','projections','enclosures','batches','minima',
                  'parameters','strata','seeds','advances','state_clones','value_equalities','state_formats')
        rows = (
            ('principal_threshold_exact_and_neighbors',(22,66,66,22,99,44,22,153,22,22,22,11,3,3,22,3,11,0,0,0)),
            ('principal_unrelated_event_and_pending_wait',(24,72,72,24,104,48,24,128,24,24,24,8,1,1,24,1,8,0,0,0)),
            ('principal_lifecycle_rejection_is_atomic_and_recovers',(28,84,84,28,126,56,28,136,28,28,28,14,1,1,28,1,20,18,24,6)),
            ('principal_agreement_loss_preserves_minimum_and_observation_identity',(29,87,87,29,126,58,29,179,29,29,29,10,5,5,29,5,15,0,0,0)),
            ('principal_cached_absence_and_present_nonmember_keep_score_order',(11,33,33,11,48,22,11,68,11,11,11,4,1,1,11,1,4,2,0,0)),
            ('principal_raw_scalar_and_stratum_neighbors',(0,0,0,0,0,0,0,24,0,0,0,0,0,20,5,0,0,0,1,0)),
            ('principal_metadata_and_roster_rejections_preserve_state',(4,12,12,4,18,8,4,57,4,4,4,3,1,1,5,5,11,5,10,5)),
            ('principal_numeric_rejection_is_atomic_and_recovers',(6,18,18,6,27,12,6,28,6,6,6,3,1,1,6,1,4,2,2,1)),
            ('principal_cardinality_zero_distance_and_stratum_weight_neighbors',(36,108,108,36,149,72,36,231,36,36,36,5,5,5,36,5,5,0,0,0)),
        )
        return cls.FIRST + ''.join('CORE_PRINCIPAL_NEXT_PROFILE case=' + case + ''.join(
            ' ' + field + '=' + str(value) for field, value in zip(fields, values, strict=True)) + '\n'
            for case, values in rows)

    def test_real_principal_profile_adds_its_own_producer_work(self):
        legacy = 'CORE_PROFILE_COUNTS fits=1 points=3\n'
        self.assertEqual(MODULE.profile_counts(legacy)['charged_fit_work_upper'], 1)
        counts = MODULE.profile_counts(legacy + self.FIRST)
        self.assertEqual(counts['charged_fit_work_upper'], 11)
        self.assertEqual(counts['point_admissions_upper'], 643)
        self.assertEqual(counts.get('principal_counter_records'), 1)

    def test_unknown_principal_marker_is_rejected_in_a_healthy_legacy_stream(self):
        legacy = 'CORE_PROFILE_COUNTS fits=1 points=3\n'
        self.assertEqual(MODULE.profile_counts(legacy)['point_admissions_upper'], 3)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(legacy + self.FIRST.replace(
                'case=principal_seed_and_genuine_update_hysteresis', 'case=unadmitted_case'))

    def test_all_means_the_two_owner_checkpoint_without_a_full_manifest_claim(self):
        self.assertEqual(set(MODULE.SCOPE.select('all')), {'principal', 'regression'})
        claim = MODULE.checkpoint_scope_claim(MODULE.SCOPE.select('all'))
        self.assertTrue(claim['checkpoint_complete_selection'])
        self.assertFalse(claim['critical_manifest_complete_selection'])
        self.assertIn('two changed critical owners', claim['qualification_scope'])

    def test_full_actual_rows_and_partial_prefixes_stay_distinct(self):
        complete = MODULE.principal_counts(self.complete_rows(), require_complete=True)
        self.assertEqual(complete['principal_counter_records'], 10)
        self.assertEqual(complete['principal_observed_calls']['getters'], 1263)
        self.assertEqual(complete['principal_observed_calls']['value_equalities'], 37)
        partial = MODULE.principal_counts(self.FIRST)
        self.assertIsNone(partial['principal_observed_calls'])
        self.assertEqual(len(partial['principal_missing_cases']), 9)
        for altered in (self.complete_rows() + self.FIRST, self.complete_rows().rstrip('\n'),
                        self.complete_rows().replace('getters=259', 'getters=18446744073709551616'),
                        self.complete_rows().replace('value_equalities=37', 'state_equalities=37')):
            # The last replacement is independently covered below because no
            # single actual row contains the aggregate37 value.
            if altered == self.complete_rows():
                continue
            with self.subTest(value=altered[:70]), self.assertRaises(MODULE.Incomplete):
                MODULE.principal_counts(altered, require_complete=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.principal_counts(self.complete_rows().replace('value_equalities=', 'state_equalities='))

    def test_successful_baseline_cannot_omit_the_principal_family(self):
        helper = CoreAnalysisTests()
        output = helper.completed_mutation_log(helper.complete_runs_rows()).replace('96 passed', '148 passed')
        outcome = {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                   'phase_results': helper.phase_results('Success')}
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'baseline.log'
            for text, valid in ((output, True), (output.replace(self.complete_rows(), ''), False)):
                path.write_text(text)
                report = {}
                MODULE.record_counts(report, 'principal:mutation-baseline', {
                    'charged_fit_work_upper': 5990, 'point_admissions_upper': 269245,
                    **MODULE.selection_reservation(package=True), 'observation_pending': True})
                if valid:
                    MODULE.observe_mutation_outcome(report, path.parent / 'outcomes.json',
                        'principal:mutation-baseline', outcome, report['core_invocations'][0])
                else:
                    with self.assertRaisesRegex(MODULE.Incomplete, 'Principal'):
                        MODULE.observe_mutation_outcome(report, path.parent / 'outcomes.json',
                            'principal:mutation-baseline', outcome, report['core_invocations'][0])

    def test_principal_only_timeout_routes_known_and_unknown_markers_to_parser(self):
        helper = CoreAnalysisTests()
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'timeout.log'
            outcome = {'scenario': {'Mutant': {'name': 'stalled'}}, 'summary': 'Timeout', 'log_path': path.name}
            for prefix, valid in ((self.FIRST, True), ('CORE_PRINCIPAL_FAKE value=1\n', False)):
                path.write_text(helper.mutation_phase_header('Test') + prefix)
                report = {}
                MODULE.record_counts(report, 'principal:mutation-1', {
                    'charged_fit_work_upper': 5990, 'point_admissions_upper': 269245,
                    **MODULE.selection_reservation(package=True), 'observation_pending': True})
                row = report['core_invocations'][0]
                if valid:
                    MODULE.observe_mutation_outcome(report, path.parent / 'outcomes.json',
                                                   'principal:mutation-1', outcome, row)
                    self.assertEqual(row['observation_status'], 'timeout-prefix-unqualified')
                else:
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.observe_mutation_outcome(report, path.parent / 'outcomes.json',
                                                       'principal:mutation-1', outcome, row)
                self.assertTrue(row['observation_pending'])


class CoreAnalysisTests(unittest.TestCase):
    """Challenge actual admission/counter/cause behavior with valid neighbors."""

    @staticmethod
    def repaired_selection_equivalence_scope():
        """Independent fresh-listing identities; not native outcome evidence."""
        names = (
            ('TC-S13-E-001', '694:26: replace || with && in maximum_components'),
            ('TC-S13-E-002', '756:52: replace > with < in common_witness'),
            ('TC-S13-E-003', '756:62: replace || with && in common_witness'),
        )
        replacements = {'TC-S13-E-001': '&&', 'TC-S13-E-002': '<', 'TC-S13-E-003': '&&'}
        return [{'id': identifier, 'mutant': {'name': 'crates/time-core/src/selection.rs:' + suffix,
                                            'file': 'crates/time-core/src/selection.rs',
                                            'replacement': replacements[identifier]},
                 'source_sha256': 'da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e',
                 'proof': {'path': 'tasks/s1.3-selection-equivalences.md'}}
                for identifier, suffix in names]

    @classmethod
    def combined_equivalence_scope(cls):
        """Literal reviewed own-domain identities; not a native score or outcome."""
        names = (
            ('TC-S13-E-004', '155:15: replace >> with << in center_admissible', '<<'),
            ('TC-S13-E-005', '166:15: replace >> with << in distance_admissible', '<<'),
            ('TC-S13-E-006', '251:31: replace < with <= in Fraction::compare', '<='),
            ('TC-S13-E-007', '314:28: replace | with ^ in ScaledValue::from_bits', '^'),
        )
        return cls.repaired_selection_equivalence_scope() + [
            {'id': identifier, 'mutant': {'name': 'crates/time-core/src/projection_enclosure.rs:' + suffix,
                                        'file': 'crates/time-core/src/projection_enclosure.rs',
                                        'replacement': replacement},
             'source_sha256': '219142ff1ce58475b4707306e5f85909eb03918d9b3e21d0f5181e2bee49fe36',
             'proof': {'path': 'tasks/s1.3-enclosure-equivalences.md'}}
            for identifier, suffix, replacement in names]

    def test_core_equivalence_scope_accepts_empty_or_exact_repaired_trio(self):
        MODULE.admit_core_equivalence_scope([])
        MODULE.admit_core_equivalence_scope(self.repaired_selection_equivalence_scope())
        MODULE.admit_core_equivalence_scope(list(reversed(self.repaired_selection_equivalence_scope())))

    def test_core_equivalence_scope_accepts_only_the_complete_combined_seven(self):
        good = self.combined_equivalence_scope()
        MODULE.admit_core_equivalence_scope(good)
        MODULE.admit_core_equivalence_scope(list(reversed(good)))
        for subset in (good[3:], good[:6], good[1:], good[:2] + good[3:4], good[:3] + [good[3]] * 4):
            with self.subTest(subset=subset), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_core_equivalence_scope(subset)

    def test_combined_equivalence_scope_rejects_wrong_owners_operators_and_unreviewed_complement(self):
        good = self.combined_equivalence_scope()
        for index in range(7):
            for owner, field, value in (
                ('mutant', 'file', 'crates/time-core/src/regression.rs'),
                ('mutant', 'replacement', '!='),
                ('proof', 'path', 'tasks/other-equivalence.md'),
            ):
                bad = copy.deepcopy(good)
                bad[index][owner][field] = value
                with self.subTest(index=index, owner=owner, field=field), self.assertRaises(MODULE.Incomplete):
                    MODULE.admit_core_equivalence_scope(bad)
            bad = copy.deepcopy(good)
            bad[index]['source_sha256'] = ('219142ff1ce58475b4707306e5f85909eb03918d9b3e21d0f5181e2bee49fe36'
                                           if index < 3 else 'da8b87fe69215b71419ff370d4032f0dc49363131c75e70076cb2cce6305d19e')
            with self.subTest(index=index, field='source_sha256'), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_core_equivalence_scope(bad)
        bad = copy.deepcopy(good)
        bad[3]['mutant'].update({
            'name': 'crates/time-core/src/projection_enclosure.rs:275:62: replace || with && in Fraction::complement',
            'replacement': '&&'})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.admit_core_equivalence_scope(bad)

    def test_core_equivalence_scope_rejects_old_or_different_faults_and_unbound_proofs(self):
        good = self.repaired_selection_equivalence_scope()
        bad = [None, False, {}, good[:2], good + [good[0]], [good[0]] * 3]
        for field, value in (
            ('id', 'TC-S13-E-004'),
            ('source_sha256', '0071a1a407a922f6d2019741a12aea24663e0a3b9e37f7e3a135aefcdae94840'),
            ('mutant', None), ('proof', None),
            ('proof', {'path': 'tasks/old-proof.md'}),
        ):
            changed = copy.deepcopy(good)
            changed[0][field] = value
            bad.append(changed)
        for wrong_fault in (
            'crates/time-core/src/selection.rs:691:26: replace || with && in maximum_components',
            'crates/time-core/src/selection.rs:652:41: replace || with && in maximum_components',
            'crates/time-core/src/selection.rs:756:52: replace > with == in common_witness',
        ):
            changed = copy.deepcopy(good)
            changed[0]['mutant']['name'] = wrong_fault
            bad.append(changed)
        for value in bad:
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete):
                MODULE.admit_core_equivalence_scope(value)

    def setUp(self):
        """Replace only adjacent product admission for inert reader fixtures."""
        admission = patch.object(MODULE.PRODUCT, 'admit_product', side_effect=self.inert_product)
        admission.start()
        self.addCleanup(admission.stop)
        runs_gate = patch.object(MODULE, 'RUNS_TELEMETRY_STATUS',
                                 'admitted-reviewed-runs-telemetry-and-campaign-budget')
        runs_gate.start()
        self.addCleanup(runs_gate.stop)
        # These are historical synthetic parser fixtures, not new discovery.
        for mapping, values in (
            (MODULE.FIXED, {'graph_status': 'admitted-final-source-inventory-and-cohort-binding'}),
            (MODULE.PROJECTION_TELEMETRY, {'status': 'admitted-source-owned-observer'}),
            (MODULE.COORDINATOR_COHORTS, {'status': 'admitted-exact-source-and-cohorts'}),
            (MODULE.ENCLOSURE_TELEMETRY, {'status': 'admitted-source-owned-observer',
                                       'case_recipes': MODULE.ENCLOSURE.case_recipes(),
                                       'source_owned_private_work_per_package': MODULE.ENCLOSURE.SOURCE_PRIVATE_WORK,
                                       'literal_doctests_added': 0}),
            (MODULE.PROJECTED_BATCH_TELEMETRY, {
                'status': 'admitted-source-owned-observer',
                # Deliberate inert parser allowances, never candidate quotas.
                'global_limits': {key: amount * 1000
                                  for key, amount in MODULE.projected_batch_budget_vector().items()}}),
            (MODULE.FINAL_PACKAGE_TEST_COUNTS, {'ordinary': 94, 'doctests': 2, 'baseline': 96}),
        ):
            fixture = patch.dict(mapping, values)
            fixture.start(); self.addCleanup(fixture.stop)

    @staticmethod
    def norm_row(**changes):
        """Independent literal nine-field transport recipe, not module schema reuse."""
        values = dict(normalizations=1, observations=3, contexts=1, monotonic=3,
                      coordinates=12, exchanges=3, getters=5, stats=0, points=0)
        values.update(changes)
        return 'CORE_NORMALIZATION_PROFILE_COUNTS ' + ' '.join(f'{key}={value}' for key, value in values.items())

    @classmethod
    def fuzz_row(cls, raw=1, **changes):
        """One actual-parser cumulative fuzz control; no simulated Rust claim."""
        changes.setdefault('stats', 1)
        return cls.norm_row(**changes).replace('CORE_NORMALIZATION_PROFILE_COUNTS ',
            f'CORE_NORMALIZATION_FUZZ_COUNTS raw={raw} ')

    @staticmethod
    def complete_projection_rows():
        """Independent authored fixture recipes; no native result is inferred."""
        recipes = (
            ('terms_admission', (0,0,0,0,0,0,0,36,48,0)),
            ('linear_identity', (1,3,3,1,7,12,3,240,1,5)),
            ('shape_controls', (2,6,6,1,8,24,6,288,1,2)),
            ('origin_durations', (2,6,6,1,10,24,6,288,1,2)),
            ('integer_envelope', (1,3,3,1,7,12,3,168,1,3)),
            ('error_precedence', (3,4,4,4,9,16,4,204,2,12)),
            ('evicted_anchor', (1,65,65,1,68,260,65,1488,1,2)),
            ('numeric_failures', (1,3,3,1,7,12,3,168,5,6)),
            ('signed_zero_consumption', (1,3,3,1,5,12,3,504,8,16)),
            ('projection_private_predicate', (0,)*10),
            ('projection_private_candidate', (0,)*10),
            ('projection_private_arithmetic', (0,)*10),
            ('projection_private_errors', (0,)*10),
        )
        fields = ('histories','appends','observations','contexts','monotonic',
                  'coordinates','exchanges','getters','terms','projections')
        return ''.join('CORE_PROJECTION_PROFILE case=' + case + ''.join(
            f' {field}={value}' for field,value in zip(fields, values, strict=True)) + '\n'
            for case,values in recipes)

    @staticmethod
    def complete_runs_rows():
        """Independent synthetic complete scopes, never Rust execution evidence."""
        external = [(75,3,3),(21,3,3),(12,1,1),(12,1,1),(12,1,1),
                    (3,1,1),(15,5,5),(6,2,2),(3,1,1)]
        guards = [(64,0,0,0,0,0,0,0,0),(1,13,0,0,0,0,0,0,0),
                  (0,0,21,0,0,0,0,0,0,0),(0,0,0,64,1,5,0,0,196)]
        rows = [f'CORE_RUNS_PROFILE points={p} inputs={i} selections={s}\n' for p,i,s in external]
        facts = ('Success(SuccessFacts { start: 8, count: 4, frame: 17, stop: RunsPassed, '
                 'decision_start: 4, decision_count: 8, decision_runs: 3, fit_runs: 3, '
                 'statistics_frame: 17, statistics_count: 4, degrees: 2, line_frame: 17, '
                 'numeric_bits: [13799016943732968640, 13805783338290302156, 4553143622606572749, '
                 '4548640022979202253, 4548640022979202253] })')
        rows.append('CORE_RUNS_SIGN_RECEIPT counts=Counts { points: 24, inputs: 2, selectors: 2 } '
                    'observer=SignObserver { positive: Some(4517753797628002304), '
                    'negative: Some(13751419776488144896), positive_seen: Once, '
                    'negative_seen: Once, calls: 102 } observed=' + facts + ' ordinary=' + facts +
                    ' preservation=[true, true, true, true, true, true] '
                    'identities=[((17, 12), (17, 12)), ((17, 12), (17, 12))]\n')
        rows.append('CORE_RUNS_SIGN_PROFILE points=24 inputs=2 selectors=2\n')
        rows.extend('CORE_RUNS_GUARDS table={} runs={} contexts={} points={} inputs={} prepares={} direct_fits={} selectors={} returned_window_rows={}\n'.format(*row) for row in guards)
        return ''.join(rows)

    @staticmethod
    def mutation_phase_header(phase):
        argv = ['cargo', 'test', *(['--no-run'] if phase == 'Build' else []),
                '--verbose', '--package=time-core@0.0.0', '--message-format=json', '--frozen']
        return '*** ' + ' '.join(argv) + '\n'

    @staticmethod
    def complete_selection_rows():
        """Literal independent package marker recipes; no Rust execution claim."""
        return ('S13_SELECTION_PROFILE scope=interval intervals=8 minima=0 batches=0 selections=0\n'
                'S13_SELECTION_PROFILE scope=minimum intervals=0 minima=6 batches=0 selections=0\n'
                'S13_SELECTION_PROFILE scope=batch intervals=19 minima=1 batches=8 selections=1\n'
                'S13_SELECTION_PROFILE scope=literal_permutations intervals=446 minima=65 batches=65 selections=65\n'
                'S13_BATCH_GUARD_PROFILE {"case":"capacity","intervals":1,"batches":1}\n'
                'S13_BATCH_GUARD_PROFILE {"case":"capacity_neighbor","intervals":16,"batches":2}\n'
                'S13_BATCH_GUARD_PROFILE {"case":"duplicate_recovery","intervals":2,"batches":5}\n'
                'SELECTION_HELPER_PROFILE case=capacity_neighbor read_adapters=2 prefix_sorts=3 '
                'cell_depths=2 label_projections=5 endpoint_collections=2 depth_builds=2 '
                'component_scans=3 witness_builds=4\n'
                'SELECTION_HELPER_PROFILE case=duplicate_recovery read_adapters=0 prefix_sorts=0 '
                'cell_depths=0 label_projections=0 endpoint_collections=3 depth_builds=3 '
                'component_scans=2 witness_builds=0\n')

    def completed_mutation_log(self, test_output, *, include_selection=True):
        finished = '{"reason":"build-finished","success":true}\n'
        if include_selection:
            test_output += self.complete_selection_rows()
            test_output += self.complete_projection_rows()
            # The new family is independently fixed by its source recipe parser.
            test_output += ''.join(
                'CORE_ENCLOSURE_PROFILE case=' + case + ''.join(
                    ' ' + field + '=' + str(value)
                    for field, value in zip(MODULE.ENCLOSURE.FIELDS, values, strict=True)) + '\n'
                for case, values in MODULE.ENCLOSURE.SOURCE_RECIPES)
            test_output += ProjectedBatchWrapperTests.complete_rows()
            test_output += PrincipalCheckpointTests.complete_rows()
        if 'test result:' not in test_output:
            test_output += 'test result: ok. 96 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;\n'
        return (self.mutation_phase_header('Build') + finished
                + self.mutation_phase_header('Test') + finished + test_output)

    def semantic_record(self):
        """Synthetic relocation of the independently captured native Cargo E0308 shape."""
        root = '/work/tmp/cargo-mutants-source-v2TRlL.tmp'
        return {
            'reason': 'compiler-message',
            'package_id': 'path+file://' + root + '/crates/time-core#0.0.0',
            'manifest_path': root + '/crates/time-core/Cargo.toml',
            'target': {'kind': ['lib'], 'crate_types': ['lib'], 'name': 'time_core',
                       'src_path': root + '/crates/time-core/src/lib.rs',
                       'edition': '2024', 'doc': True, 'doctest': True, 'test': True},
            'message': {'$message_type': 'diagnostic', 'level': 'error',
                        'message': 'mismatched types', 'rendered': 'synthetic diagnostic',
                        'children': [], 'code': {'code': 'E0308', 'explanation': None},
                        'spans': [{'file_name': 'crates/time-core/src/regression.rs',
                                   'is_primary': True, 'line_start': 42, 'column_start': 7}]},
        }

    def semantic_mutation_log(self, record=None):
        record = self.semantic_record() if record is None else record
        note = copy.deepcopy(record)
        note['message'] = {'$message_type': 'diagnostic', 'level': 'failure-note',
                           'message': 'For more information, try rustc --explain E0308.',
                           'rendered': None, 'children': [], 'code': None, 'spans': []}
        return (self.mutation_phase_header('Build') + json.dumps(record) + '\n' + json.dumps(note) + '\n'
                + '{"reason":"build-finished","success":false}\n')

    def unused_comparison_record(self, operator='<'):
        """Literal native selection lint, relocated to the fixed synthetic worker."""
        record = self.semantic_record()
        record['message'].update(
            code={'code': 'unused_comparisons', 'explanation': None},
            message='comparison is useless due to type limits',
            spans=[{'file_name': 'crates/time-core/src/selection.rs', 'is_primary': True,
                    'line_start': 526, 'column_start': 15,
                    'text': [{'text': '        while position ' + operator
                              + ' /* ~ changed by cargo-mutants ~ */ 0 {'}]}],
            children=[{'level': level, 'message': message, 'children': [],
                       'code': None, 'spans': [], 'rendered': None}
                      for level, message in (
                          ('note', '`-D unused-comparisons` implied by `-D warnings`'),
                          ('help', 'to override `-D warnings` add `#[allow(unused_comparisons)]`'))])
        return record

    @staticmethod
    def build_script_record():
        """Observed native rustversion record, with a synthetic inert worker path."""
        return {
            'reason': 'build-script-executed',
            'package_id': 'registry+https://github.com/rust-lang/crates.io-index#rustversion@1.0.23',
            'linked_libs': [], 'linked_paths': [], 'cfgs': [], 'env': [],
            'out_dir': '/work/tmp/cargo-mutants-source-yfcbrg.tmp/target/debug/build/'
                       'rustversion-d2854a6eb6777e68/out',
        }

    def test_runs_admission_remains_pending_without_coordinator_binding(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); self.fixture(root)
            with patch.object(MODULE, 'RUNS_TELEMETRY_STATUS', 'pending-reviewed-runs-telemetry-and-campaign-budget'):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(root, 'contracts')
            self.assertEqual(MODULE.read_policy(root, 'contracts')['source_file_count'], 95)

    def test_final_projected_batch_admission_requires_every_finite_nominal_allowance(self):
        required = MODULE.projected_batch_budget_vector()
        self.assertEqual(len(required), 68)
        key = 'projected_batch.call.batches'
        bad = [None, {}, {name: amount for name, amount in required.items() if name != key},
               {**required, 'projected_batch.extra': 1}]
        bad.extend({**required, key: amount} for amount in (True, -1, 1 << 64, required[key] - 1))
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            with patch.dict(MODULE.PROJECTED_BATCH_TELEMETRY, {'global_limits': required}):
                policy = self.fixture(root)
                self.assertEqual(MODULE.read_policy(root, 'contracts')['projected_batch_telemetry']['global_limits'], required)
            for limits in bad:
                with self.subTest(limits=limits), patch.dict(
                        MODULE.PROJECTED_BATCH_TELEMETRY, {'global_limits': limits}):
                    changed = copy.deepcopy(policy)
                    changed['projected_batch_telemetry']['global_limits'] = limits
                    self.write_policy(root, changed)
                    with self.assertRaisesRegex(MODULE.Incomplete, 'Projected-batch cumulative allowances'):
                        MODULE.read_policy(root, 'contracts')

    def test_runs_counter_families_require_exact_complete_scopes_and_charge_prefixes(self):
        text = self.complete_runs_rows()
        complete = MODULE.runs_counts(text, require_complete=True)
        self.assertEqual((complete['runs_points'], complete['runs_inputs'], complete['runs_selections']), (247, 21, 20))
        self.assertEqual((complete['runs_guard_candidates'], complete['runs_guard_returned_window_rows']), (5, 196))
        self.assertEqual(complete['runs_guard_count_calls'], 13)
        self.assertEqual(complete['runs_fit_upper'], 1245)
        self.assertEqual(complete['runs_point_copy_upper'], 41860)
        self.assertEqual(complete['runs_decision_residual_upper'], 62280)
        self.assertEqual(complete['runs_observer_hooks_upper'], 3098)
        self.assertEqual(complete['runs_observer_pair_comparisons_upper'], 12392)
        self.assertEqual((complete['runs_input_slots_upper'],complete['runs_residual_slots_upper']), (79680,79680))
        for altered in [text.replace('CORE_RUNS_SIGN_PROFILE points=24 inputs=2 selectors=2\n', ''),
                        text + 'CORE_RUNS_SIGN_PROFILE points=24 inputs=2 selectors=2\n',
                        text.replace('CORE_RUNS_PROFILE points=75 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=75 inputs=3 selections=3 extra=0'),
                        text.replace('CORE_RUNS_PROFILE points=21 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=3 inputs=1 selections=1'),
                        text.replace('CORE_RUNS_PROFILE points=21 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=12 inputs=2 selections=2'),
                        text.replace('CORE_RUNS_PROFILE points=21 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=20 inputs=3 selections=3'),
                        text.replace('CORE_RUNS_PROFILE points=21 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=21 inputs=2 selections=3'),
                        text.replace('CORE_RUNS_PROFILE points=21 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=21 inputs=3 selections=2'),
                        text.replace('CORE_RUNS_GUARDS table=64', 'CORE_RUNS_GUARDS table=65'),
                        text.replace('runs=13 contexts=0', 'runs=11 contexts=0'),
                        text.replace('runs=13 contexts=0', 'runs=14 contexts=0'),
                        text.replace('prepares=5 direct_fits=0', 'prepares=4 direct_fits=0'),
                        text.replace('prepares=5 direct_fits=0', 'prepares=6 direct_fits=0'),
                        text.replace('inputs=1 prepares=5', 'inputs=0 prepares=5'),
                        text.replace('prepares=5 direct_fits=0', 'prepares=5 direct_fits=1'),
                        text.replace('returned_window_rows=196', 'returned_window_rows=257'),
                        text.replace('CORE_RUNS_SIGN_PROFILE points=24', 'CORE_RUNS_SIGN_PROFILE points=18446744073709551616'),
                        text.replace('positive: Some(4517753797628002304)', 'positive: Some(0)'),
                        text.replace('counts=Counts { points: 24', 'counts=Counts { points: 23'),
                        text.replace('positive_seen: Once', 'positive_seen: Duplicate'),
                        text.replace('calls: 102', 'calls: 103'),
                        text.replace('preservation=[true, true, true, true, true, true]', 'preservation=[true, true, true, true, true, false]'),
                        text.replace(' identities=[', ' extra=x identities=['),
                        text.replace('CORE_RUNS_PROFILE points=75 inputs=3 selections=3', 'CORE_RUNS_PROFILE points=7 inputs=1 selections=1')]:
            with self.subTest(value=altered[:80]), self.assertRaises(MODULE.Incomplete):
                MODULE.runs_counts(altered, require_complete=True)
        receipt = next(line for line in text.splitlines(keepends=True) if line.startswith('CORE_RUNS_SIGN_RECEIPT '))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.runs_counts(text.replace(receipt, '') + receipt.rstrip('\n'), require_complete=True)
        placeholder = 'CORE_RUNS_SIGN_RECEIPT counts=Counts { synthetic } observer=SignObserver { synthetic } observed=ok ordinary=ok preservation=ok identities=[]\n'
        with self.assertRaises(MODULE.Incomplete):
            MODULE.runs_counts(text.replace(receipt, placeholder), require_complete=True)
        partial = MODULE.runs_counts('CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n')
        self.assertEqual((partial['runs_points'], partial['runs_selections'], partial['runs_fit_upper']), (2, 0, 0))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.runs_counts('CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n', require_complete=True)
        combined = MODULE.profile_counts(text, require_runs=True)
        self.assertEqual(combined['charged_fit_work_upper'], 1245)
        self.assertEqual(combined['point_admissions_upper'], 42107)
        legacy = 'CORE_PROFILE_COUNTS fits=640 points=65533\n'
        endpoint = MODULE.account_literal_doctests(MODULE.profile_counts(legacy + text, require_runs=True))
        self.assertEqual(endpoint['charged_fit_work_upper'], 1885)
        self.assertEqual(endpoint['point_admissions_upper'], 107643)
        with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 1884}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(legacy + text, require_runs=True)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(legacy + text + self.complete_projection_rows(),
                                      require_runs=True, require_projection=True)
        with patch.dict(MODULE.PROFILE_LIMITS, {'point_admissions_upper': 107642}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.account_literal_doctests(MODULE.profile_counts(legacy + text, require_runs=True))
            with self.assertRaises(MODULE.Incomplete):
                MODULE.account_literal_doctests(MODULE.profile_counts(
                    legacy + text + self.complete_projection_rows(), require_runs=True,
                    require_projection=True))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(legacy.replace('fits=640', 'fits=641') + text, require_runs=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.account_literal_doctests(MODULE.profile_counts(
                legacy.replace('points=65533', 'points=65534') + text, require_runs=True))

    def test_runs_partial_guard_bounds_charge_all_five_preparations(self):
        row = ('CORE_RUNS_GUARDS table=0 runs=13 contexts=0 points=64 inputs=1 '
               'prepares=5 direct_fits=0 selectors=0 returned_window_rows=196\n')
        counts = MODULE.runs_counts(row)
        self.assertEqual((counts['runs_guard_count_calls'], counts['runs_guard_candidates']), (13, 5))
        self.assertEqual((counts['runs_fit_upper'], counts['runs_point_copy_upper']), (5, 320))
        for changed in (row.replace('runs=13', 'runs=14'),
                        row.replace('prepares=5', 'prepares=6'),
                        row.replace('inputs=1 prepares=5', 'inputs=0 prepares=5')):
            with self.subTest(row=changed), self.assertRaises(MODULE.Incomplete):
                MODULE.runs_counts(changed)

    def test_runs_receipt_matches_profile_and_all_new_counter_rows_require_newline(self):
        full = self.complete_runs_rows()
        MODULE.runs_counts(full, require_complete=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.runs_counts(full, require_complete=True, validate_receipt=False)
        contradiction = full.replace(
            'CORE_RUNS_SIGN_PROFILE points=24 inputs=2 selectors=2\n',
            'CORE_RUNS_SIGN_PROFILE points=0 inputs=0 selectors=0\n')
        with self.assertRaises(MODULE.Incomplete):
            MODULE.runs_counts(contradiction)
        for marker in ('CORE_RUNS_PROFILE ', 'CORE_RUNS_SIGN_PROFILE ', 'CORE_RUNS_GUARDS '):
            lines = full.splitlines(keepends=True)
            index = next(i for i, line in enumerate(lines) if line.startswith(marker))
            last = lines.pop(index)
            truncated = ''.join(lines) + last.rstrip('\n')
            with self.subTest(marker=marker), self.assertRaises(MODULE.Incomplete):
                MODULE.runs_counts(truncated, require_complete=True)
        prefix = MODULE.runs_counts('CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n')
        self.assertEqual(prefix['runs_points'], 2)

    def test_runs_legal_and_privacy_specimens_preserve_distinct_causes(self):
        cases = MODULE.runs_contract_examples()
        legal = cases['legal_runs_selection_and_error']
        self.assertIsNone(legal['code'])
        self.assertEqual((legal['calls']['runs_points'], legal['calls']['runs_inputs'],
                          legal['calls']['runs_selections']), (3,1,2))
        for name, code in [('runs_selection_default','E0277'),
                           ('runs_selection_private_update','E0451'),
                           ('runs_selection_mutate_start','E0616'),
                           ('runs_error_private_update','E0451'),
                           ('runs_error_mutate_point_index','E0616'),
                           ('runs_statistics_mutate','E0594'),
                           ('runs_raw_input','E0308'),('runs_line_injection','E0061'),
                           ('runs_private_terminalcandidate','E0432'),
                           ('runs_private_runsobserver','E0432'),
                           ('runs_private_select_fixture_runs_observed','E0432')]:
            self.assertEqual(cases[name]['code'], code)
            self.assertTrue(cases[name]['tokens'])
            self.assertTrue(cases[name]['source'].splitlines()[cases[name]['line']-1].strip())

    def test_seven_owner_all_scope_cannot_reuse_the_historical_global_allowance(self):
        with patch.dict(MODULE.SELECTION_GLOBAL_LIMITS, {'intervals': 255354}), self.assertRaises(MODULE.Incomplete):
            MODULE.prospective_scope_reservation('all', dict(MODULE.SUBJECTS))
        proof = MODULE.prospective_scope_reservation('coverage', dict(MODULE.SUBJECTS))
        self.assertEqual(proof['mutation_package_invocations_upper'], 0)
        self.assertEqual(proof['coverage_package_invocations_upper'], 1)
        self.assertEqual(proof['charged_upper'], {'charged_fit_work_upper': 5990,
                                                 'point_admissions_upper': 269245})
        self.assertEqual(proof['package_test_count_upper_each'], 96)
        for lane, subjects in [('host', dict(MODULE.SUBJECTS)), ('all', {}),
                               ('all', {'selection': 'crates/time-core/src/regression.rs'})]:
            with self.subTest(lane=lane, subjects=subjects), self.assertRaises(MODULE.Incomplete):
                MODULE.prospective_scope_reservation(lane, subjects)
        with patch.dict(MODULE.GLOBAL_LIMITS, {'point_admissions_upper': 226748}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.prospective_scope_reservation('coverage', dict(MODULE.SUBJECTS))

    def test_focused_mutation_reserves_actual_inventory_and_its_own_baseline(self):
        proof = MODULE.prospective_scope_reservation('mutation', {'principal': MODULE.SUBJECTS['principal']})
        self.assertEqual(proof['mutation_package_invocations_upper'], 218)
        self.assertEqual(proof['coverage_package_invocations_upper'], 0)
        self.assertEqual(proof['package_invocations_upper'], 218)
        self.assertEqual(proof['legal_consumer_executions_upper'], 0)
        self.assertEqual(proof['charged_upper'], {'charged_fit_work_upper': 1305820,
                                                  'point_admissions_upper': 58696064})
        with patch.dict(MODULE.GLOBAL_LIMITS, {'point_admissions_upper': 58696063}):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.prospective_scope_reservation('mutation', {'principal': MODULE.SUBJECTS['principal']})

    def test_selection_complete_package_and_missing_private_recipe_remain_distinct(self):
        full = self.complete_runs_rows() + self.complete_selection_rows()
        counts = MODULE.profile_counts(full, require_runs=True, require_selection=True)
        self.assertEqual(counts['selection_observed_calls'],
                         {'intervals': 492, 'minima': 72, 'batches': 81, 'selections': 66})
        self.assertEqual(counts['selection_helper_observed_calls'],
                          {'read_adapters': 2, 'prefix_sorts': 3, 'cell_depths': 2, 'label_projections': 5,
                           'endpoint_collections': 5, 'depth_builds': 5, 'component_scans': 5, 'witness_builds': 4})
        self.assertEqual(counts['selection_helper_records'], 2)
        self.assertEqual(sum(counts['selection_helper_observed_calls'].values()), 31)
        missing = full.replace(
            'SELECTION_HELPER_PROFILE case=capacity_neighbor read_adapters=2 prefix_sorts=3 '
            'cell_depths=2 label_projections=5 endpoint_collections=2 depth_builds=2 '
            'component_scans=3 witness_builds=4\n', '')
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(missing, require_selection=True)
        partial = MODULE.profile_counts(missing)
        self.assertIsNone(partial['selection_helper_observed_calls'])
        self.assertTrue(partial['selection_helper_observation_pending'])
        self.assertEqual(partial['selection_helper_records'], 1)
        self.assertEqual(partial['selection_helper_missing_cases'], ['capacity_neighbor'])
        self.assertEqual(partial['selection_helper_recorded_call_subtotals']['endpoint_collections'], 3)
        malformed = full + '{"reason":"compiler-message","message":{"rendered":"S13_SELECTION_PROFILE scope=interval intervals=8 minima=0 batches=0 selections=0"}}\n'
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(malformed)

    def test_selection_external_consumer_rows_are_closed_separate_and_bounded(self):
        row = 'CORE_SELECTION_EXTERNAL_PROFILE intervals=6 minima=3 batches=4 selections=2\n'
        counts = MODULE.profile_counts(row)
        self.assertEqual(counts['selection_external_consumer_calls'],
                         {'intervals': 6, 'minima': 3, 'batches': 4, 'selections': 2})
        self.assertEqual(counts['direct_fit_calls'], 0)
        self.assertIsNone(counts['selection_observed_calls'])
        self.assertEqual(MODULE.selection_external_counts('')['selection_external_consumer_calls'], None)
        for wrong in [row+row, row.rstrip('\n'), row.replace('6', '06', 1),
                      row.replace('intervals=6', 'intervals=7'), row.replace('minima=3', 'minima=4'),
                      row.replace('batches=4', 'batches=3'), row.replace('selections=2', 'selections=true'),
                      row.replace(' selections=', ' extra=0 selections='), 'prefix '+row,
                      '{"rendered":'+json.dumps(row)+'}\n']:
            with self.subTest(wrong=wrong), self.assertRaises(MODULE.Incomplete):
                MODULE.selection_external_counts(wrong)
        for mixed in (row + 'CORE_PROFILE_COUNTS fits=0 points=0\n', row + self.complete_selection_rows()):
            with self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(mixed)
        prefix = MODULE.selection_external_counts('CORE_SELECTION_EXTERNAL_PROFILE intervals=1 minima=0 batches=0 selections=0\n')
        self.assertEqual(prefix['selection_external_consumer_calls']['intervals'], 1)

    def test_selection_reservation_retains_missing_observations_without_zeroing(self):
        report = {}
        MODULE.record_counts(report, 'package', {
            'charged_fit_work_upper': 5990, 'point_admissions_upper': 269245,
            **MODULE.selection_reservation(package=True), 'observation_pending': True})
        self.assertEqual(report['core_selection_reserved_totals']['intervals'], 492)
        self.assertIsNone(report['core_selection_package_observed_calls'])
        self.assertEqual(report['core_observation_pending_invocations'], ['package'])
        row = report['core_invocations'][0]
        row['observed_counts'] = MODULE.profile_counts(
            'S13_SELECTION_PROFILE scope=interval intervals=8 minima=0 batches=0 selections=0\n',
            require_fit=False)
        row['observation_pending'] = False
        MODULE.refresh_accounting(report)
        self.assertEqual(report['core_observation_pending_invocations'], [])
        self.assertEqual(report['core_selection_pending_observation_invocations'], ['package'])
        self.assertIsNone(report['core_selection_package_observed_calls'])
        self.assertEqual(report['core_selection_reserved_totals']['intervals'], 492)
        self.assertEqual(report['core_selection_reported_record_subtotals']['intervals'], 8)
        bad = {**MODULE.selection_reservation(package=True), 'charged_fit_work_upper': 0,
               'point_admissions_upper': 0}
        bad['selection_reserved_calls']['intervals'] = 0
        with self.assertRaises(MODULE.Incomplete):
            MODULE.record_counts({}, 'bad', bad)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.selection_reservation(external={'intervals': True, 'minima': 3, 'batches': 4, 'selections': 2})

    def test_raw_helper_canonical_projection_has_its_own_reserved_scope_and_rejects_forgery(self):
        expected = {
            'direct_read_adapter_attempts_upper': 2, 'endpoint_collection_writes_upper': 160,
            'prefix_and_endpoint_sort_comparisons_upper': 3968, 'endpoint_dedup_comparisons_upper': 155,
            'cell_visits_upper': 315, 'membership_probes_upper': 5072,
            'component_visits_upper': 315, 'witness_probes_upper': 64,
            'label_sort_comparisons_upper': 1080, 'label_prefix_assignments_upper': 144,
            'intersection_reductions_upper': 120}
        package = MODULE.selection_reservation(package=True)
        external = MODULE.selection_reservation(external={'intervals': 6, 'minima': 3, 'batches': 4, 'selections': 2})
        for label, reservation, literal, fits, points, forged in (
                ('package', package, expected, 5990, 269245, 0),
                ('external', external, dict.fromkeys(expected, 0), 0, 0, 1)):
            with self.subTest(scope=label):
                self.assertEqual(reservation['selection_reserved_helper_canonical_source_work_uppers'], literal)
                report = {}
                counts = {'charged_fit_work_upper': fits, 'point_admissions_upper': points,
                          **reservation, 'observation_pending': True}
                MODULE.record_counts(report, label, counts)
                self.assertEqual(report['core_selection_helper_canonical_source_work_uppers'], literal)
                self.assertIsNone(report['core_selection_helper_observed_calls'])
                bad = copy.deepcopy(counts)
                bad['selection_reserved_helper_canonical_source_work_uppers']['membership_probes_upper'] = forged
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.record_counts({}, 'forged-' + label, bad)
        package['selection_reserved_helper_canonical_source_work_uppers']['membership_probes_upper'] = 0
        self.assertEqual(MODULE.selection_reservation(package=True)
                         ['selection_reserved_helper_canonical_source_work_uppers'], expected)

    def test_selection_external_catalog_pins_intended_causes_and_receiving_operations(self):
        cases = MODULE.SELECTION_CONTRACTS.contract_examples()
        self.assertEqual(len(cases), 27)
        legal = cases['legal_selection_construction_and_output']
        self.assertIsNone(legal['code'])
        self.assertEqual(legal['selection_calls'], {'intervals': 6, 'minima': 3, 'batches': 4, 'selections': 2})
        self.assertEqual(sum(item['code'] is None for item in MODULE.legacy_contract_examples().values()), 10)
        for name, specimen in cases.items():
            if specimen['code'] is not None:
                line = specimen['source'].splitlines()[specimen['line']-1]
                self.assertTrue(line.strip(), name)
                self.assertIn(specimen['code'], {'E0277', 'E0451', 'E0616', 'E0603', 'E0308'})
                self.assertTrue(specimen['tokens'])

    def test_healthy_mutation_package_requires_exact_96_tests_and_all_selection_records(self):
        with tempfile.TemporaryDirectory() as temp:
            output = Path(temp)
            outcome = {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                       'phase_results': self.phase_results('Success')}
            complete = self.completed_mutation_log(self.complete_runs_rows())
            for text, message in [(complete.replace('96 passed', '95 passed'), '96'),
                                  (self.completed_mutation_log(self.complete_runs_rows(), include_selection=False), 'Selection')]:
                (output/'baseline.log').write_text(text)
                report = {}
                MODULE.record_counts(report, 'selection:mutation-baseline', {
                    'charged_fit_work_upper': 5990, 'point_admissions_upper': 269245,
                    **MODULE.selection_reservation(package=True), 'observation_pending': True})
                with self.subTest(message=message), self.assertRaisesRegex(MODULE.Incomplete, message):
                    MODULE.observe_mutation_outcome(report, output/'outcomes.json',
                        'selection:mutation-baseline', outcome, report['core_invocations'][0])
                self.assertTrue(report['core_invocations'][0]['observation_pending'])

    def test_timeout_selector_only_prefixes_are_parsed_and_never_qualified(self):
        prefixes = [
            'S13_SELECTION_PROFILE scope=interval intervals=1 minima=0 batches=0 selections=0\n',
            'S13_BATCH_GUARD_PROFILE {"case":"capacity","intervals":1,"batches":0}\n',
            'SELECTION_HELPER_PROFILE case=capacity_neighbor read_adapters=1 prefix_sorts=0 '
            'cell_depths=0 label_projections=0 endpoint_collections=0 depth_builds=0 '
            'component_scans=0 witness_builds=0\n',
            'SELECTION_HELPER_PROFILE case=duplicate_recovery read_adapters=0 prefix_sorts=0 '
            'cell_depths=0 label_projections=0 endpoint_collections=1 depth_builds=0 '
            'component_scans=0 witness_builds=0\n',
        ]
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary)
            outcome = {'scenario': {'Mutant': {'name': 'stalled'}}, 'summary': 'Timeout',
                       'log_path': 'timeout.log'}
            for prefix in prefixes:
                for text, valid in [(prefix, True), (prefix+prefix, False), (prefix.rstrip('\n'), False)]:
                    (output/'timeout.log').write_text(self.mutation_phase_header('Test')+text)
                    report = {}
                    MODULE.record_counts(report, 'selection:mutation-1', {
                        'charged_fit_work_upper': 5990, 'point_admissions_upper': 269245,
                        **MODULE.selection_reservation(package=True), 'observation_pending': True})
                    row = report['core_invocations'][0]
                    with self.subTest(prefix=prefix, valid=valid):
                        if valid:
                            MODULE.observe_mutation_outcome(report, output/'outcomes.json',
                                                            'selection:mutation-1', outcome, row)
                            self.assertEqual(row['observation_status'], 'timeout-prefix-unqualified')
                        else:
                            with self.assertRaises(MODULE.Incomplete):
                                MODULE.observe_mutation_outcome(report, output/'outcomes.json',
                                                                'selection:mutation-1', outcome, row)
                        self.assertTrue(row['observation_pending'])
                        self.assertEqual(row['charged_fit_work_upper'], 5990)
                        self.assertEqual(row['selection_reserved_calls']['intervals'], 492)
                        self.assertNotIn('observed_counts', row)

    def inert_product(self, root):
        """Return the synthetic product inventory; never qualify a real graph."""
        return {}, {name: MODULE.PRODUCT.sha256(root / name) for name in MODULE.BOUND_INPUTS}

    def fixture(self, root):
        """Create the full95-file synthetic shape and retained ten-owner manifest."""
        for name in MODULE.BOUND_INPUTS:
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text('// isolated synthetic input; never compiled\n')
        critical = {**MODULE.HISTORICAL_SUBJECTS, 'principal': MODULE.SUBJECTS['principal']}
        manifest = 'critical_files=[' + ','.join(json.dumps(n) for n in critical.values()) + ']\n'
        for name in critical.values():
            manifest += '[[security_modules]]\npackage="time-core"\nfiles=[' + json.dumps(name) + ']\n'
        (root / 'policy/verification-manifest.toml').write_text(manifest)
        policy = json.loads((ROOT / MODULE.POLICY).read_text())
        # This inert source tree has no reviewed product-equivalence candidate.
        # Exact repaired-source E admission has its own controls above.
        policy['mutation_equivalences'] = []
        policy.update(copy.deepcopy(MODULE.FIXED))
        policy['admission_status'] = 'admitted-final-source'
        policy['normalization_telemetry'] = copy.deepcopy(MODULE.NORMALIZATION_TELEMETRY)
        self.rebind(root, policy)
        return policy

    def rebind(self, root, policy):
        """Rebind deliberate synthetic bytes only; production policy stays pending."""
        policy['inputs_sha256'] = self.inert_product(root)[1]
        self.write_policy(root, policy)

    def write_policy(self, root, policy):
        """Write one owned temporary policy fixture, never candidate policy."""
        (root / MODULE.POLICY).write_text(json.dumps(policy))

    def fuzz_fixture(self, root, policy):
        """Satisfy adjacent cohort metadata in an inert positive reader control."""
        for name in MODULE.FUZZ_INPUTS:
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            if not path.exists():
                path.write_text('inert standalone cohort input; never executed\n')
        cohort = json.loads((ROOT / 'policy/time-domain-analysis.json').read_text())
        (root / 'policy/time-domain-analysis.json').write_text(json.dumps(cohort))
        selected = policy['coverage_guided_fuzzing']
        selected['status'] = 'admitted-test-only-source'
        selected['runtime_profile_reuse_review'] = 'reviewed-history-source-and-resource-envelope'
        selected['profile'] = 'time-core-history-s12d'
        selected['fuzzer_flags'] = ['-runs=4096','-max_len=1561','-timeout=2','-rss_limit_mb=512','-malloc_limit_mb=256','-max_total_time=15']
        selected['target_contract'] = 'FixtureHistory::try_append-and-committed-observation-report-public-entrypoints'
        selected['inputs_sha256'] = {name: MODULE.PRODUCT.sha256(root / name) for name in MODULE.FUZZ_INPUTS}
        self.write_policy(root, policy)

    @staticmethod
    def historical_scope(*names):
        """Select inert legacy parser owners without widening the checkpoint."""
        selected = names or tuple(MODULE.HISTORICAL_SUBJECTS)
        return MODULE.COMMON.Scope('time-core', tuple(
            (name, MODULE.HISTORICAL_SUBJECTS[name]) for name in selected), MODULE.POLICY)

    def test_pending_source_is_not_a_pass_and_fixed_synthetic_neighbor_works(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); policy = self.fixture(root)
            admitted = MODULE.read_policy(root, 'contracts')
            self.assertEqual(admitted['critical_files'], list(MODULE.SUBJECTS.values()))
            self.assertEqual(MODULE.SCOPE.select('regression'), {'regression': MODULE.SUBJECTS['regression']})
            self.assertEqual(MODULE.SCOPE.select('principal'), {'principal': MODULE.SUBJECTS['principal']})
            self.assertEqual(MODULE.SCOPE.select('all'), MODULE.SUBJECTS)
            self.assertEqual(len(admitted['inputs_sha256']), 95)
            policy['admission_status'] = 'pending-final-source-and-inventory-binding'
            self.write_policy(root, policy)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.read_policy(root, 'contracts')

    def test_source_binding_drift_and_scope_threshold_budget_changes_reject(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy = self.fixture(root)
            for key, value in [('coverage_minimum_line_percent', 89), ('mutation_minimum_viable_killed_percent', 94),
                               ('maximum_mutants', 513), ('jobs', True), ('mutation_jobs', 12),
                               ('mutation_jobs', 2), ('mutation_jobs', True), ('mutation_jobs', 0),
                               ('mutation_process_limit', 128), ('mutation_process_limit', True),
                               ('mutation_process_limit', 512.0),
                               ('mutation_process_limit', 256), ('mutation_process_limit', 0),
                               ('mutation_message_format', 'human'), ('unexpected_lane', 'host'),
                               ('mutation_wall_seconds', 2400), ('source_file_count', 68)]:
                policy = copy.deepcopy(healthy); policy[key] = value
                self.write_policy(root, policy)
                with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(root, 'coverage')
            self.write_policy(root, healthy)
            for name in ['crates/time-core/src/regression.rs', 'crates/time-core/tests/residuals.rs',
                         'crates/time-core/tests/support/residual_facts.rs', 'crates/time-core/PROVENANCE.md']:
                path = root / name; original = path.read_bytes(); path.write_bytes(original + b'changed\n')
                with self.subTest(changed=name), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(root, 'coverage')
                path.write_bytes(original)

    def test_missing_bindings_equivalences_and_inline_coverage_padding_reject(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy = self.fixture(root)
            for change in ['missing', 'null', 'equivalence']:
                policy = copy.deepcopy(healthy)
                if change == 'missing': policy['inputs_sha256'].pop('Cargo.lock')
                if change == 'null': policy['inputs_sha256']['Cargo.lock'] = None
                if change == 'equivalence': policy['mutation_equivalences'] = [{'historical': 'C'}]
                self.write_policy(root, policy)
                with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(root, 'coverage')
            for name in MODULE.SUBJECTS.values():
                policy = copy.deepcopy(healthy); path = root / name; original = path.read_bytes()
                path.write_text('#[test]\nfn padding_is_not_production_coverage(){}\n')
                self.rebind(root, policy)
                with self.subTest(inline=name), self.assertRaises(MODULE.Incomplete):
                    MODULE.read_policy(root, 'coverage')
                path.write_bytes(original)

    def test_complete_inventory_rejects_missing_extra_or_changed_product_shape(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); policy = self.fixture(root); actual = self.inert_product(root)[1]
            for name in ['crates/time-core/src/regression/residuals.rs',
                         'crates/time-core/tests/private/residual_guards.rs']:
                changed = dict(actual); changed.pop(name)
                with self.subTest(missing=name), patch.object(MODULE.PRODUCT, 'admit_product', return_value=({}, changed)):
                    with self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root, 'coverage')
            changed = dict(actual, unexpected='f' * 64)
            with patch.object(MODULE.PRODUCT, 'admit_product', return_value=({}, changed)):
                with self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root, 'coverage')
            changed = dict(actual); changed['crates/time-core/tests/residuals.rs'] = 'f' * 64
            with patch.object(MODULE.PRODUCT, 'admit_product', return_value=({}, changed)):
                with self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root, 'coverage')
            self.assertEqual(MODULE.read_policy(root, 'coverage')['inputs_sha256'], policy['inputs_sha256'])

    def test_each_critical_file_requires_unique_security_and_critical_registration(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy = self.fixture(root); path = root / 'policy/verification-manifest.toml'
            original = path.read_text()
            for filename in MODULE.SUBJECTS.values():
                quoted = json.dumps(filename)
                for change in [original.replace(quoted, '"missing.rs"', 1),
                               original.replace('critical_files=[', 'critical_files=[' + quoted + ',', 1),
                               original + '[[security_modules]]\npackage="time-core"\nfiles=[' + quoted + ']\n']:
                    policy = copy.deepcopy(healthy); path.write_text(change); self.rebind(root, policy)
                    with self.subTest(file=filename), self.assertRaises(MODULE.Incomplete):
                        MODULE.read_policy(root, 'coverage')
                path.write_text(original)
            self.rebind(root, healthy)
            MODULE.read_policy(root, 'coverage')

    def test_profile_counters_preserve_zero_private_guard_rows_and_sum_real_executables(self):
        text = ('CORE_PROFILE_COUNTS fits=0 points=0\nCORE_PROFILE_COUNTS fits=285 points=1329\n'
                'CORE_STATS_PROFILE_COUNTS stats=169 points=1330\n')
        result = MODULE.profile_counts(text)
        self.assertEqual(result['direct_fit_calls'], 285)
        self.assertEqual(result['fit_calls'], 285)
        self.assertEqual(result['stats_calls'], 169)
        self.assertEqual(result['internal_fit_upper_bound'], 169)
        self.assertEqual(result['observed_charged_fit_work'], 454)
        self.assertEqual(result['charged_fit_work_upper'], 454)
        self.assertEqual(result['point_admissions'], 2659)
        self.assertEqual(result['counter_records'], 3)
        self.assertEqual(MODULE.profile_counts('CORE_STATS_PROFILE_COUNTS stats=0 points=0', require_fit=False)['stats_calls'], 0)
        for value in ['', 'test result: ok. 1 passed; 0 failed; 0 ignored;',
                      'CORE_PROFILE_COUNTS fits=0 points=3', 'CORE_PROFILE_COUNTS fits=641 points=3',
                      'CORE_PROFILE_COUNTS fits=1 points=65537',
                      'CORE_PROFILE_COUNTS fits=1 points=3\nCORE_PROFILE_COUNTS fits=-1 points=4',
                      'CORE_STATS_PROFILE_COUNTS stats=-1 points=3',
                      'CORE_STATS_PROFILE_COUNTS stats=1 points=3\nCORE_STATS_PROFILE_COUNTS incomplete',
                      '--- STDERR:\nCORE_PROFILE_COUNTS fits=1 points=3',
                      'Summary [0s]\nCORE_STATS_PROFILE_COUNTS stats=1 points=3']:
            with self.subTest(value=value), self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(value)

    def test_profile_work_and_point_endpoints_include_both_families(self):
        for stats, accepted in [(354, True), (355, True), (356, False)]:
            text = f'CORE_PROFILE_COUNTS fits=285 points=1329\nCORE_STATS_PROFILE_COUNTS stats={stats} points=3\n'
            if accepted: self.assertEqual(MODULE.profile_counts(text)['charged_fit_work_upper'], 285 + stats)
            else:
                with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(text)
        for points, accepted in [(65535, True), (65536, True), (65537, False)]:
            text = f'CORE_PROFILE_COUNTS fits=1 points=3\nCORE_STATS_PROFILE_COUNTS stats=1 points={points - 3}\n'
            if accepted: self.assertEqual(MODULE.profile_counts(text)['point_admissions_upper'], points)
            else:
                with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(text)

    def test_literal_doctests_are_upper_allowances_without_fabricating_observations(self):
        observed = MODULE.profile_counts('CORE_PROFILE_COUNTS fits=285 points=1329\nCORE_STATS_PROFILE_COUNTS stats=169 points=1330\n')
        charged = MODULE.account_literal_doctests(observed)
        self.assertEqual(charged['direct_fit_calls'], 285)
        self.assertEqual(charged['stats_calls'], 169)
        self.assertEqual(charged['observed_charged_fit_work'], 454)
        self.assertEqual(charged['charged_fit_work_upper'], 454)
        self.assertEqual(charged['point_admissions'], 2659)
        self.assertEqual(charged['point_admissions_upper'], 2662)
        self.assertEqual(charged['internal_fit_upper_bound'], 169)
        self.assertNotIn('literal_doctest_stats_calls_upper', observed)
        self.assertEqual(MODULE.account_literal_doctests(MODULE.profile_counts('CORE_PROFILE_COUNTS fits=640 points=0'))['charged_fit_work_upper'],640)
        for text in ['CORE_STATS_PROFILE_COUNTS stats=1 points=65534']:
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete):
                MODULE.account_literal_doctests(MODULE.profile_counts(text))

    def test_fuzz_distinguishes_raw_point_and_stats_counters_and_incomplete_controls(self):
        healthy = '\n'.join(self.fuzz_row(raw=n) for n in range(1, 4097)) + '\nDone 4096 runs\n'
        result = MODULE.fuzz_counts(healthy, complete=True)
        self.assertEqual(result['raw_driver_runs'], 4096)
        self.assertEqual(result['stats_calls'], 1)
        self.assertEqual(result['direct_fit_calls'], 0)
        self.assertEqual(result['fit_calls'], 0)
        self.assertEqual(result['internal_fit_upper_bound'], 1)
        self.assertEqual(result['point_admissions'], 0)
        self.assertEqual(result['generated_point_admissions_upper'], 64)
        control = self.fuzz_row() + '\nindependent normalization oracle'
        self.assertEqual(MODULE.fuzz_counts(control, complete=False)['stats_calls'], 1)
        for output in [control, self.fuzz_row(normalizations=0,stats=0,getters=0),
                       self.fuzz_row(observations=65), self.fuzz_row(stats=2),
                       self.fuzz_row()+'\n'+self.fuzz_row(),
                       'CORE_FUZZ_COUNTS raw=1 points=3 fits=1',
                       self.fuzz_row(raw=2)]:
            with self.subTest(output=output[:70]), self.assertRaises(MODULE.Incomplete): MODULE.fuzz_counts(output, complete=True)
        for tail in [self.fuzz_row(raw=2,observations=2),self.fuzz_row(raw=2,stats=0)]:
            with self.subTest(decrease=tail), self.assertRaises(MODULE.Incomplete):
                MODULE.fuzz_counts(control.splitlines()[0] + '\n' + tail, complete=False)

    def test_checkpoint_refuses_fuzz_even_with_historical_resource_profile_and_cohort(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy = self.fixture(root); self.fuzz_fixture(root, healthy)
            MODULE.read_policy(root, 'contracts')
            with self.assertRaisesRegex(MODULE.Incomplete, 'two named mutation cohorts only'):
                MODULE.read_policy(root, 'fuzz')
            for key, value in [('profile', 'unreviewed-profile'), ('target_contract', 'fit-only'),
                               ('runtime_profile_reuse_review', 'pending-independent-resource-contract-equivalence'),
                               ('fuzzer_flags', ['-runs=1000000']), ('status', 'pending-s12b-target-source-binding')]:
                changed = copy.deepcopy(healthy); changed['coverage_guided_fuzzing'][key] = value
                self.write_policy(root, changed)
                with self.subTest(key=key), self.assertRaisesRegex(MODULE.Incomplete, 'two named mutation cohorts only'):
                    MODULE.read_policy(root, 'fuzz')
            changed = copy.deepcopy(healthy); changed['coverage_guided_fuzzing']['invocation_limits']['wall_seconds'] = 16
            self.write_policy(root, changed)
            with self.assertRaisesRegex(MODULE.Incomplete, 'two named mutation cohorts only'): MODULE.read_policy(root, 'fuzz')
            changed = copy.deepcopy(healthy); changed['coverage_guided_fuzzing']['selected_grants'] = {}
            self.write_policy(root, changed)
            with self.assertRaisesRegex(MODULE.Incomplete, 'two named mutation cohorts only'): MODULE.read_policy(root, 'fuzz')

    def test_real_consumer_specimens_have_ten_legal_controls_and_exact_declared_causes(self):
        cases = MODULE.legacy_contract_examples(); legal = [v for v in cases.values() if v['code'] is None]
        self.assertEqual(len(legal), 10)
        self.assertEqual(sum(v['code'] is not None for v in cases.values()), 120)
        self.assertEqual({k: sum(v['calls'][k] for v in legal) for k in ['direct_fit_calls', 'stats_calls', 'point_admissions']},
                         {'direct_fit_calls': 2, 'stats_calls': 2, 'point_admissions': 15})
        for name in ['point_construction', 'line_construction', 'residual_raw_update']:
            self.assertEqual(cases[name]['code'], 'E0451')
        for name in ['point_mutation_relative_seconds', 'input_mutation_len', 'line_mutation_phase_seconds']:
            self.assertEqual(cases[name]['code'], 'E0616')
        for name, case in cases.items():
            if case['code']:
                self.assertTrue(case['source'].splitlines()[case['line'] - 1].strip(), name)
                self.assertTrue(case['tokens'], name)
                source = '/work/contract-inputs/' + name + '.rs'
                diagnostic = {'level': 'error', 'code': {'code': case['code']}, 'message': ' '.join(case['tokens']),
                              'spans': [{'is_primary': True, 'file_name': source, 'line_start': case['line']}], 'children': []}
                good = {'exit_code': 1, 'failure': None, 'output': json.dumps(diagnostic)}
                MODULE.COMMON.compiler_cause(good, case, source)
                wrong_code = 'E0308' if case['code'] == 'E0432' else 'E0432'
                for altered in [dict(diagnostic, code={'code': wrong_code}), dict(diagnostic, message='unrelated setup failure'),
                                dict(diagnostic, spans=[{'is_primary': True, 'file_name': source, 'line_start': 999}]),
                                dict(diagnostic, spans=[{'is_primary': False, 'file_name': source, 'line_start': case['line']}])]:
                    with self.subTest(case=name), self.assertRaises(MODULE.Incomplete):
                        MODULE.COMMON.compiler_cause(dict(good, output=json.dumps(altered)), case, source)
                with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.compiler_cause(dict(good, exit_code=0), case, source)
                with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.compiler_cause(dict(good, failure='timeout'), case, source)

    def test_owned_mutation_raw_accounting_retains_two_files_and_unviable_survivors(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp)
            for subject in MODULE.SUBJECTS:
                out = work / 'mutation' / subject / 'mutants.out'; out.mkdir(parents=True)
                outcomes = [{'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                             'phase_results': self.phase_results('Success')},
                            {'scenario': {'Mutant': {'name': 'guard_removed'}}, 'summary': 'CaughtMutant', 'log_path': 'killed.log',
                             'phase_results': self.phase_results('CaughtMutant')},
                            {'scenario': {'Mutant': {'name': 'wrong_sign'}}, 'summary': 'MissedMutant', 'log_path': 'missed.log',
                             'phase_results': self.phase_results('MissedMutant')},
                            {'scenario': {'Mutant': {'name': 'type_error'}}, 'summary': 'Unviable', 'log_path': 'compile.log',
                             'phase_results': self.phase_results('Unviable')}]
                common = ('CORE_PROFILE_COUNTS fits=1 points=3\n'
                          'CORE_STATS_PROFILE_COUNTS stats=1 points=3\n' + self.complete_runs_rows())
                for name in ['baseline.log', 'missed.log']:
                    (out / name).write_text(self.completed_mutation_log(common))
                (out / 'killed.log').write_text(self.completed_mutation_log(
                    common + '\nfailures:\n\n    frozen_assertion\n\n'
                    'test result: FAILED. 0 passed; 1 failed; 0 ignored;\n'))
                (out / 'compile.log').write_text(self.semantic_mutation_log())
                (out / 'outcomes.json').write_text(json.dumps({'total_mutants': 3, 'outcomes': outcomes}))
            report = {}
            # Inert parser control only; the real 640-work policy remains unchanged.
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            self.assertFalse(report['mutation_full_manifest_selected'])
            self.assertTrue(report['mutation_checkpoint_complete_selection'])
            self.assertEqual(report['mutation_baselines_required'], 2)
            self.assertEqual(report['mutation_baselines_observed'], 2)
            self.assertEqual(report['mutation_missing_selected_files'], [])
            for subject in MODULE.SUBJECTS:
                row = report['mutation_full_accounting'][subject]
                self.assertEqual(row['counts'], {'K': 1, 'M': 1, 'U': 1, 'T': 0, 'X': 0, 'E': 0})
                self.assertEqual(row['accounted'], 3)
            self.assertEqual(report['core_charged_subset_totals'], {'charged_fit_work_upper': 35940, 'point_admissions_upper': 1615488})
            self.assertEqual(report['core_direct_and_stats_observations'], {'direct_fit_calls': 6, 'stats_calls': 6, 'point_admissions': 36})
            self.assertEqual(report['core_runs_external_observations']['runs_selections'], 120)
            self.assertTrue(report['core_recorded_observations_parsed'])
            self.assertFalse(report['mutation_counter_observation_complete'])
            self.assertEqual(report['core_invocations'][0]['observed_counts']['runs_selections'],20)
            baseline = work / 'mutation/principal/mutants.out/baseline.log'
            original = baseline.read_text(); baseline.write_text(self.completed_mutation_log(
                'test result: ok. 96 passed; 0 failed; 0 ignored;'))
            incomplete = {}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                with self.assertRaises(MODULE.Incomplete): MODULE.mutation_accounting(incomplete, work)
            self.assertEqual(incomplete['mutation_full_accounting']['principal']['accounted'], 3)
            self.assertEqual(incomplete['core_invocations'][0]['charged_fit_work_upper'],5990)
            self.assertNotIn('observed_counts',incomplete['core_invocations'][0])
            self.assertFalse(incomplete['core_recorded_observations_parsed'])
            baseline.write_text(original)
            path = work / 'mutation/regression/mutants.out/outcomes.json'; raw = json.loads(path.read_text())
            raw['outcomes'][1]['log_path'] = '../outside.log'; path.write_text(json.dumps(raw))
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                with self.assertRaises(MODULE.Incomplete): MODULE.mutation_accounting({}, work)

    def test_timeout_remains_unqualified_and_full_charge_without_complete_receipt(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            (output / 'baseline.log').write_text(self.completed_mutation_log(self.complete_runs_rows()))
            (output / 'timeout.log').write_text(
                self.mutation_phase_header('Test') + 'CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n')
            outcomes = [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')},
                {'scenario': {'Mutant': {'name': 'stalled'}}, 'summary': 'Timeout', 'log_path': 'timeout.log'},
                {'scenario': {'Mutant': {'name': 'missing'}}, 'summary': 'Timeout', 'log_path': 'missing.log'},
            ]
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 2, 'outcomes': outcomes}))
            report = {'subjects': self.historical_scope('runs').select('all')}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            self.assertEqual(report['core_charged_subset_totals'],
                             {'charged_fit_work_upper': 17970, 'point_admissions_upper': 807744})
            self.assertEqual(report['mutation_full_accounting']['runs']['counts']['T'], 2)
            self.assertEqual(report['mutation_full_accounting']['runs']['counts']['K'], 0)
            self.assertFalse(report['mutation_counter_observation_complete'])
            self.assertEqual(report['core_observation_pending_invocations'], ['runs:mutation-1', 'runs:mutation-2'])
            for row in report['core_invocations'][1:]:
                self.assertEqual((row['charged_fit_work_upper'], row['point_admissions_upper']), (5990, 269248))
                self.assertTrue(row['observation_pending'])
                self.assertNotIn('observed_counts', row)
            self.assertEqual(report['core_invocations'][1]['observation_status'], 'timeout-prefix-unqualified')
            self.assertEqual(report['core_invocations'][2]['observation_status'], 'timeout-log-missing')

    def test_failed_parse_keeps_full_pending_debit_and_explicit_incomplete_status(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            (output / 'bad.log').write_text(self.completed_mutation_log(
                'CORE_RUNS_PROFILE points=2 inputs=0 selections=0 extra=1\n'))
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 0, 'outcomes': [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'bad.log',
                 'phase_results': self.phase_results('Success')}]}))
            report = {'subjects': self.historical_scope('runs').select('all')}
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_accounting(report, work)
            self.assertEqual(report['core_charged_subset_totals'],
                             {'charged_fit_work_upper': 5990, 'point_admissions_upper': 269248})
            self.assertFalse(report['core_recorded_observations_parsed'])
            self.assertFalse(report['mutation_counter_observation_complete'])
            self.assertEqual(report['core_observation_pending_invocations'], ['runs:mutation-baseline'])
            self.assertNotIn('observed_counts', report['core_invocations'][0])

    def test_timeout_with_existing_log_but_no_counter_prefix_remains_pending(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            (output / 'baseline.log').write_text(self.completed_mutation_log(self.complete_runs_rows()))
            (output / 'timeout.log').write_text('child stopped before emitting a counter\n')
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')},
                {'scenario': {'Mutant': {'name': 'no_prefix'}}, 'summary': 'Timeout', 'log_path': 'timeout.log'},
            ]}))
            report = {'subjects': self.historical_scope('runs').select('all')}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            self.assertEqual(report['core_charged_subset_totals'],
                             {'charged_fit_work_upper': 11980, 'point_admissions_upper': 538496})
            row = report['core_invocations'][1]
            self.assertEqual(row['observation_status'], 'timeout-no-counter-prefix')
            self.assertTrue(row['observation_pending'])
            self.assertNotIn('observed_counts', row)
            self.assertEqual(row['raw_log_sha256'], hashlib.sha256(
                b'child stopped before emitting a counter\n').hexdigest())
            self.assertFalse(report['mutation_counter_observation_complete'])

    def test_bad_early_subject_preserves_later_known_subject_pending_charge(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp)
            for subject in ('regression', 'principal'):
                output = work / 'mutation' / subject / 'mutants.out'
                output.mkdir(parents=True)
                text = ('CORE_RUNS_PROFILE points=2 inputs=0 selections=0 extra=1\n'
                        if subject == 'regression' else self.complete_runs_rows())
                (output / 'baseline.log').write_text(self.completed_mutation_log(text))
                (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 0, 'outcomes': [
                    {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                     'phase_results': self.phase_results('Success')}]}))
            report = {'subjects': {name: MODULE.SUBJECTS[name] for name in ('regression', 'principal')}}
            with self.assertRaises(MODULE.Incomplete):
                MODULE.mutation_accounting(report, work)
            self.assertEqual(report['core_charged_subset_totals'],
                             {'charged_fit_work_upper': 11980, 'point_admissions_upper': 538496})
            self.assertEqual(report['core_observation_pending_invocations'],
                             ['regression:mutation-baseline', 'principal:mutation-baseline'])
            self.assertFalse(report['mutation_counter_observation_complete'])
            self.assertTrue(all('observed_counts' not in row for row in report['core_invocations']))

    def test_timeout_can_end_after_sign_receipt_before_owner_drop(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            (output / 'baseline.log').write_text(self.completed_mutation_log(self.complete_runs_rows()))
            (output / 'timeout.log').write_text(
                self.mutation_phase_header('Test') + 'CORE_RUNS_SIGN_RECEIPT partial-fixture-result\n')
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')},
                {'scenario': {'Mutant': {'name': 'before_drop'}},
                 'summary': 'Timeout', 'log_path': 'timeout.log'}]}))
            report = {'subjects': self.historical_scope('runs').select('all')}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            row = report['core_invocations'][1]
            self.assertEqual(row['observation_status'], 'timeout-receipt-unqualified')
            self.assertTrue(row['observation_pending'])
            self.assertNotIn('observed_counts', row)
            self.assertEqual(row['charged_fit_work_upper'], 5990)
            self.assertFalse(report['mutation_counter_observation_complete'])

    def test_all_known_outcomes_are_charged_even_when_subset_cap_is_exceeded(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp)
            for subject in ('regression', 'principal'):
                output = work / 'mutation' / subject / 'mutants.out'
                output.mkdir(parents=True)
                (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 0, 'outcomes': [
                    {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'unread.log',
                     'phase_results': self.phase_results('Success')}]}))
            report = {'subjects': {name: MODULE.SUBJECTS[name] for name in ('regression', 'principal')}}
            with patch.dict(MODULE.GLOBAL_LIMITS, {'charged_fit_work_upper': 5990}):
                with self.assertRaisesRegex(MODULE.Incomplete, 'global charged allowance'):
                    MODULE.mutation_accounting(report, work)
            self.assertEqual(report['core_charged_subset_totals'],
                             {'charged_fit_work_upper': 11980, 'point_admissions_upper': 538496})
            self.assertEqual(len(report['core_observation_pending_invocations']), 2)
            self.assertFalse(report['mutation_counter_observation_complete'])

    @contextlib.contextmanager
    def admitted_mutation_execution_fixture(self, session, report):
        """Mock adjacent admission/custody only, to reach native-error cleanup.

        The complete coordinator reader has separate closed-plan controls. This
        inert fixture represents its admitted focused result and launches no tool.
        """
        plan_path = ROOT / 'tasks/inert-coordinator-plan.json'
        args = SimpleNamespace(lane='mutation', subject='principal', cohort_plan=plan_path,
                               cohort_id='principal', output_dir=Path('/inert-output'))
        policy = {'mutation_inventories': {name: {'count': 1, 'sha256': 'a'*64}
                                           for name in MODULE.SUBJECTS}}
        plan = {'candidate_sha256': 'b'*64, 'recipe_sha256': 'c'*64,
                'product_inputs_sha256': {}, 'tool_inputs_sha256': {}, 'tool_payloads_sha256': {},
                'cohorts': [{'id': 'principal', 'subject': 'principal', 'output_dir': '/inert-output'}]}
        session.product_identity = {}; session.identity = {}; session.source = Path('/inert-source')
        report['tools_sha256'] = {}
        with patch.object(MODULE, 'read_policy', return_value=policy), \
             patch.object(MODULE, 'read_cohort_plan', return_value=(plan, {'synthetic_reservation': True}, plan_path)), \
             patch.object(MODULE, 'inventory_document', return_value=plan), \
             patch.object(MODULE.COMMON, 'Session', return_value=session):
            yield args

    def test_campaign_primary_error_survives_accounting_and_cleanup_failures(self):
        session = Mock(work=Path('/inert'), tools=Path('/inert-tools'))
        session.finish.side_effect = OSError('cleanup-failed')
        report = {}
        with self.admitted_mutation_execution_fixture(session, report) as args, \
             patch.object(MODULE.COMMON, 'run_analysis_lanes', side_effect=OSError('native-EAGAIN')), \
             patch.object(MODULE, 'mutation_accounting', side_effect=MODULE.Incomplete('accounting-malformed')):
            with self.assertRaisesRegex(OSError, 'native-EAGAIN'):
                MODULE.execute(args, report)
        self.assertEqual(report['secondary_errors'], [
            {'phase': 'mutation_accounting', 'type': 'Incomplete', 'error': 'accounting-malformed'},
            {'phase': 'session_finish', 'type': 'OSError', 'error': 'cleanup-failed'},
        ])
        session.finish.assert_called_once()

    def test_timeout_cannot_be_promoted_to_pass_after_common_lane_returns(self):
        session = Mock(work=Path('/inert'), tools=Path('/inert-tools'))
        report = {}
        def accounting(target, _work):
            target['mutation_missing_selected_files'] = []
            target['mutation_baselines_observed'] = 1
            target['mutation_full_accounting'] = {'principal': {'counts': {'T': 1}}}
        with self.admitted_mutation_execution_fixture(session, report) as args, \
             patch.object(MODULE.COMMON, 'run_analysis_lanes', return_value=None), \
             patch.object(MODULE, 'mutation_accounting', side_effect=accounting):
            with self.assertRaisesRegex(MODULE.Incomplete, 'unresolved native timeouts'):
                MODULE.execute(args, report)
        session.finish.assert_called_once()

    def test_accounting_error_remains_primary_when_only_cleanup_also_fails(self):
        session = Mock(work=Path('/inert'), tools=Path('/inert-tools'))
        session.finish.side_effect = OSError('cleanup-failed')
        report = {}
        with self.admitted_mutation_execution_fixture(session, report) as args, \
             patch.object(MODULE.COMMON, 'run_analysis_lanes', return_value=None), \
             patch.object(MODULE, 'mutation_accounting', side_effect=MODULE.Incomplete('accounting-malformed')):
            with self.assertRaisesRegex(MODULE.Incomplete, 'accounting-malformed'):
                MODULE.execute(args, report)
        self.assertEqual(report['secondary_errors'],
                         [{'phase': 'session_finish', 'type': 'OSError', 'error': 'cleanup-failed'}])

    def test_unviable_rejects_disjoint_or_forged_human_log_fragments(self):
        """Synthetic text fragments cannot independently establish native provenance."""
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        owned = ' --> crates/time-core/src/regression/runs.rs:42:7\n'
        foreign = ' --> foreign/src/lib.rs:9:2\n'
        compile_end = ('error: could not compile ' + chr(96) + 'time-core' + chr(96)
                       + ' due to 1 previous error\n')
        dependency_end = ('error: could not compile ' + chr(96) + 'unrelated-dependency'
                          + chr(96) + ' due to 1 previous error\n')
        cases = {
            'disjoint_error_and_owned_note':
                'error[E0308]: mismatched types\n' + foreign
                + 'note: unrelated owned source\n' + owned + compile_end,
            'fake_diff_header':
                '--- displayed mutant diff, not compiler output ---\n'
                + 'error[E0308]: mismatched types\n' + owned + compile_end,
            'dependency_error_plus_owned_note':
                'error[E0308]: dependency type mismatch\n' + foreign
                + dependency_end + 'note: unrelated owned source\n' + owned,
            'semantic_plus_infrastructure_error':
                'error[E0308]: mismatched types\n' + owned + compile_end
                + 'error: failed to load manifest for workspace member: permission denied\n',
        }
        for name, text in cases.items():
            with self.subTest(case=name), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(outcome, text)

    def test_unviable_json_accepts_observed_unused_imports(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        record = self.semantic_record()
        record['message']['code'] = {'code': 'unused_imports', 'explanation': None}
        record['message']['message'] = 'unused imports: `ArithmeticStage` and `arithmetic`'
        record['message']['spans'] = [
            {'file_name': 'crates/time-core/src/regression/residuals.rs',
             'is_primary': True, 'line_start': 48, 'column_start': 26},
            {'file_name': 'crates/time-core/src/regression/residuals.rs',
             'is_primary': True, 'line_start': 49, 'column_start': 5}]
        MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log(record))

    def test_unviable_json_accepts_only_observed_owned_unused_comparisons(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        for operator in ('<', '>='):
            record = self.unused_comparison_record(operator)
            log = (self.mutation_phase_header('Build') + json.dumps(record) + '\n'
                   + '{"reason":"build-finished","success":false}\n')
            with self.subTest(operator=operator):
                MODULE.unviable_compile_failure(outcome, log)

    def test_unused_comparison_lint_keeps_exact_cause_provenance_and_failure_guards(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        for change in ('unknown_code', 'message', 'explanation', 'code_shape', 'package',
                       'manifest', 'target', 'unowned_span', 'nonprimary_span', 'warning',
                       'infrastructure_child', 'infrastructure_raw', 'success', 'duplicate_json'):
            record = self.unused_comparison_record()
            if change == 'unknown_code':
                record['message']['code']['code'] = 'unused_comparisons_extra'
            elif change == 'message':
                record['message']['message'] += ' unrelated cause'
            elif change == 'explanation':
                record['message']['code']['explanation'] = 'unexpected explanation'
            elif change == 'code_shape':
                record['message']['code']['unexpected'] = None
            elif change == 'package':
                record['package_id'] = 'registry+https://example.invalid/dependency#1.0.0'
            elif change == 'manifest':
                record['manifest_path'] = '/work/tmp/cargo-mutants-source-v2TRlL.tmp/foreign/Cargo.toml'
            elif change == 'target':
                record['target']['kind'] = ['test']
            elif change == 'unowned_span':
                record['message']['spans'][0]['file_name'] = 'foreign/src/lib.rs'
            elif change == 'nonprimary_span':
                record['message']['spans'][0]['is_primary'] = False
            elif change == 'warning':
                record['message']['level'] = 'warning'
            elif change == 'infrastructure_child':
                record['message']['children'][0]['message'] = 'failed to spawn helper process'
            log = (self.mutation_phase_header('Build') + json.dumps(record) + '\n'
                   + '{"reason":"build-finished","success":false}\n')
            if change == 'infrastructure_raw':
                log += 'error: could not execute process: Resource temporarily unavailable (os error 11)\n'
            elif change == 'success':
                log = log.replace('"success":false', '"success":true')
            elif change == 'duplicate_json':
                log = log.replace('"code": "unused_comparisons"',
                                  '"code": "unused_comparisons", "code": "unused_comparisons"')
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(outcome, log)

    def test_unviable_json_accepts_observed_owned_test_relative_path(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        record = self.semantic_record()
        record['message']['code'] = {'code': 'arithmetic_overflow', 'explanation': None}
        record['message']['message'] = 'this arithmetic operation will overflow'
        record['message']['spans'][0] = {
            'file_name': 'crates/time-core/src/regression/../../tests/private/normalization_guards.rs',
            'is_primary': True, 'line_start': 340, 'column_start': 10}
        MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log(record))

    def test_unviable_json_accepts_observed_const_trait_companion(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        coded = self.semantic_record()
        coded['message']['code'] = {'code': 'E0658', 'explanation': 'An unstable feature was used.'}
        coded['message']['message'] = ('cannot call conditionally-const method '
                                       '`<u64 as TryInto<NonZero<u64>>>::try_into` in constant functions')
        coded['message']['spans'][0] = {
            'file_name': 'crates/time-core/src/regression/residuals.rs',
            'is_primary': True, 'line_start': 349, 'column_start': 11}
        companion = copy.deepcopy(coded)
        companion['message']['code'] = None
        companion['message']['message'] = '`TryInto` is not yet stable as a const trait'
        companion['message']['spans'][0]['column_start'] = 9
        log = self.semantic_mutation_log(coded).replace(
            '{"reason":"build-finished","success":false}',
            json.dumps(companion) + '\n{"reason":"build-finished","success":false}')
        MODULE.unviable_compile_failure(outcome, log)

    def test_unviable_json_new_diagnostic_forms_fail_closed(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        base = self.semantic_record()
        for filename in ('crates/time-core/src/regression/../../../foreign.rs',
                         'crates/time-core/src/regression/../../tests/private/unowned.rs',
                         'crates/time-core/src/regression/../../../../crates/time-core/src/regression.rs'):
            record = copy.deepcopy(base)
            record['message']['code'] = {'code': 'arithmetic_overflow', 'explanation': None}
            record['message']['spans'][0]['file_name'] = filename
            with self.subTest(filename=filename), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log(record))
        for code in ('unknown_lint', None):
            record = copy.deepcopy(base)
            record['message']['code'] = None if code is None else {'code': code, 'explanation': None}
            with self.subTest(code=code), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log(record))
        coded = copy.deepcopy(base)
        coded['message']['code'] = {'code': 'E0658', 'explanation': None}
        coded['message']['message'] = ('cannot call conditionally-const method '
                                       '`<u64 as TryInto<NonZero<u64>>>::try_into` in constant functions')
        coded['message']['spans'][0].update(
            file_name='crates/time-core/src/regression/residuals.rs', line_start=349)
        companion = copy.deepcopy(coded)
        companion['message']['code'] = None
        companion['message']['message'] = '`TryInto` is not yet stable as a const trait'
        with self.assertRaises(MODULE.Incomplete):
            MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log(companion))
        wrong_coded = copy.deepcopy(coded)
        wrong_coded['message']['code']['code'] = 'E0308'
        log = self.semantic_mutation_log(wrong_coded).replace(
            '{"reason":"build-finished","success":false}',
            json.dumps(companion) + '\n{"reason":"build-finished","success":false}')
        with self.assertRaises(MODULE.Incomplete):
            MODULE.unviable_compile_failure(outcome, log)
        for change in ('message', 'unowned', 'other_line', 'dependency', 'infrastructure'):
            invalid = copy.deepcopy(companion)
            if change == 'message':
                invalid['message']['message'] += ' unexpected'
            elif change == 'unowned':
                invalid['message']['spans'][0]['file_name'] = 'foreign/src/lib.rs'
            elif change == 'other_line':
                invalid['message']['spans'][0]['line_start'] = 350
            elif change == 'dependency':
                invalid['package_id'] = 'registry+https://example.invalid/dependency#1.0.0'
            else:
                invalid['message']['children'] = [{
                    'level': 'note', 'message': 'failed to spawn helper process',
                    'children': [], 'code': None, 'spans': [], 'rendered': None}]
            log = self.semantic_mutation_log(coded).replace(
                '{"reason":"build-finished","success":false}',
                json.dumps(invalid) + '\n{"reason":"build-finished","success":false}')
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(outcome, log)

    def test_unviable_json_requires_one_owned_same_record_and_build_failure(self):
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log())
        for code in ('unused_variables', 'dead_code'):
            lint = self.semantic_record()
            lint['message']['code'] = {'code': code, 'explanation': None}
            lint['message']['children'] = [{
                'message': 'the lint level is defined here', 'level': 'note', 'children': [],
                'code': None, 'rendered': None, 'spans': []}]
            with self.subTest(admitted_lint=code):
                MODULE.unviable_compile_failure(outcome, self.semantic_mutation_log(lint))
        base = self.semantic_record()
        wrong_package = copy.deepcopy(base)
        wrong_package['package_id'] = 'registry+https://example.invalid/dependency#1.0.0'
        wrong_manifest = copy.deepcopy(base)
        wrong_manifest['manifest_path'] = '/work/tmp/cargo-mutants-source-v2TRlL.tmp/foreign/Cargo.toml'
        wrong_target = copy.deepcopy(base)
        wrong_target['target']['src_path'] = '/work/tmp/cargo-mutants-source-v2TRlL.tmp/foreign/src/lib.rs'
        wrong_span = copy.deepcopy(base)
        wrong_span['message']['spans'][0]['file_name'] = 'foreign/src/lib.rs'
        null_code = copy.deepcopy(base)
        null_code['message']['code'] = None
        unknown_code = copy.deepcopy(base)
        unknown_code['message']['code']['code'] = 'unsupported_lint'
        mixed_infrastructure = copy.deepcopy(base)
        mixed_infrastructure['message']['children'] = [{
            'level': 'note', 'message': 'failed to spawn helper process',
            'children': [], 'code': None, 'spans': [], 'rendered': None}]
        wrong_flag = copy.deepcopy(outcome)
        wrong_flag['phase_results'][0]['argv'].remove('--message-format=json')
        bad_status = copy.deepcopy(outcome)
        bad_status['phase_results'][0]['process_status'] = {'Failure': 2}
        duplicate = self.semantic_mutation_log().replace(
            '"level": "error"', '"level": "error", "level": "error"')
        nonfinite = self.semantic_mutation_log().replace('"line_start": 42', '"line_start": NaN')
        overflow = self.semantic_mutation_log().replace('"line_start": 42', '"line_start": 1e9999')
        dependency = copy.deepcopy(base)
        dependency['package_id'] = 'registry+https://example.invalid/dependency#1.0.0'
        mixed = self.semantic_mutation_log().replace(
            '{"reason":"build-finished","success":false}',
            json.dumps(dependency) + '\n{"reason":"build-finished","success":false}')
        cases = [
            (outcome, self.semantic_mutation_log(wrong_package)),
            (outcome, self.semantic_mutation_log(wrong_manifest)),
            (outcome, self.semantic_mutation_log(wrong_target)),
            (outcome, self.semantic_mutation_log(wrong_span)),
            (outcome, self.semantic_mutation_log(null_code)),
            (outcome, self.semantic_mutation_log(unknown_code)),
            (outcome, self.semantic_mutation_log(mixed_infrastructure)),
            (wrong_flag, self.semantic_mutation_log()),
            (bad_status, self.semantic_mutation_log()),
            (outcome, duplicate), (outcome, nonfinite), (outcome, overflow), (outcome, mixed),
            (outcome, self.semantic_mutation_log().replace('"success":false', '"success":true')),
            (outcome, self.semantic_mutation_log() + self.mutation_phase_header('Test')),
        ]
        for changed, log in cases:
            with self.subTest(log=log[:120]), self.assertRaises(MODULE.Incomplete):
                MODULE.unviable_compile_failure(changed, log)

    def test_cargo_json_payload_cannot_impersonate_test_counters_or_failure(self):
        record = self.semantic_record()
        record['message']['rendered'] = (
            'CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n'
            'failures:\n    forged_test\n\n'
            'test result: FAILED. 0 passed; 1 failed; 0 ignored;\n')
        raw = 'before\r\n' + json.dumps(record) + '\r\n' + 'after\r\n'
        records, clean = MODULE.cargo_json_records(raw)
        self.assertEqual(len(records), 1)
        self.assertEqual(clean, 'before\r\nafter\r\n')
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(clean, require_fit=False)
        outcome = {'summary': 'CaughtMutant', 'phase_results': self.phase_results('CaughtMutant')}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.caught_native_test_failure(outcome, clean)
        for invalid in ['{"reason":"build-finished","success":false',
                        '{"reason":"unknown"}\n',
                        '{"reason":"build-finished","success":false,"success":true}\n',
                        '{"reason":"build-finished","success":NaN}\n']:
            with self.subTest(raw=invalid), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(invalid)

    def test_cargo_json_test_context_preserves_only_literal_assertion_tails(self):
        """Literal transport controls preserve panic bytes; they prove no Rust failure or kill."""
        tails = (
            '    { stage: SelectionInvariantStage::Component }))\n',
            '    { stage: SelectionInvariantStage::Support }))\n',
            '    { stage: SelectionInvariantStage::Intersection }))\n',
        )
        for tail in tails:
            panic = (
                "thread 'selection::selection_guards::capacity_neighbor_copies_all_values_in_order' "
                '(12567) panicked at crates/time-core/src/../tests/private/selection_guards.rs:365:5:\n'
                'assertion failed: matches!(cell_depth(batch, 1, 0),\n'
                '    Err(SelectionError::Invariant\n' + tail
                + 'note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace\n')
            for newline in ('\n', '\r\n'):
                raw = ('native-before\n' + panic + 'native-after\n').replace('\n', newline)
                stream = ('native-before\n{"reason":"build-finished","success":true}\n'
                          + panic + 'native-after\n').replace('\n', newline)
                with self.subTest(tail=tail, newline=repr(newline)):
                    records, cleaned = MODULE.cargo_json_records(stream, test_phase=True)
                    self.assertEqual(records, [{'reason': 'build-finished', 'success': True}])
                    self.assertEqual(cleaned, raw)
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.cargo_json_records(stream)
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.cargo_json_records(stream, test_phase=False)
        for mode in (None, 0, 1, 1.0, 'true', [], {}):
            with self.subTest(mode=mode), self.assertRaisesRegex(MODULE.Incomplete, 'mode must be Boolean'):
                MODULE.cargo_json_records('{"reason":"build-finished","success":true}\n', test_phase=mode)

    def test_cargo_json_test_context_keeps_near_misses_and_malformed_records_closed(self):
        tail = '    { stage: SelectionInvariantStage::Component }))\n'
        invalid = [
            tail.lstrip(),
            tail[1:],
            ' ' + tail,
            '\t' + tail.lstrip(),
            tail.replace('::Component', '::Slot'),
            tail.replace('::Component', '::Unknown'),
            tail.replace('SelectionInvariantStage::', ''),
            tail.replace(' }))', ' })) extra'),
            tail.replace(' }))', ' })) '),
            tail.replace(' }))', ' })'),
            tail.rstrip('\n'),
            tail.rstrip('\n') + '\r',
            '{"reason":"build-finished","success":true',
            '{"reason":"build-finished","success":false,"success":true}\n',
            '{"reason":"build-finished","success":NaN}\n',
            '{"reason":"build-finished","success":Infinity}\n',
            '{"reason":"build-finished","success":1e999}\n',
            '{"reason":"unknown"}\n',
            '{"reason":"build-finished"}\n',
            '{"reason":"build-finished","success":true,"extra":null}\n',
            '{"reason":"build-finished","success":1}\n',
        ]
        for record in invalid:
            with self.subTest(record=repr(record)), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(tail + record, test_phase=True)

    def test_literal_assertion_tail_alone_establishes_neither_compile_failure_nor_caught_test(self):
        tail = '    { stage: SelectionInvariantStage::Component }))\n'
        records, raw = MODULE.cargo_json_records(tail, test_phase=True)
        self.assertEqual(records, [])
        self.assertEqual(raw, tail)
        caught = {'summary': 'CaughtMutant', 'phase_results': self.phase_results('CaughtMutant')}
        with self.assertRaises(MODULE.Incomplete):
            MODULE.caught_native_test_failure(caught, raw)
        failure = ('failures:\n\n'
                   '    selection::selection_guards::capacity_neighbor_copies_all_values_in_order\n\n'
                   'test result: FAILED. 0 passed; 1 failed; 0 ignored;\n')
        MODULE.caught_native_test_failure(caught, failure)
        unviable = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        MODULE.unviable_compile_failure(unviable, self.semantic_mutation_log())
        build = (self.mutation_phase_header('Build') + tail
                 + '{"reason":"build-finished","success":false}\n')
        with self.assertRaises(MODULE.Incomplete):
            MODULE.unviable_compile_failure(unviable, build)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.unviable_compile_failure(unviable, self.semantic_mutation_log() + tail)

    def test_mutation_test_consumers_opt_in_to_literal_assertion_transport(self):
        """Synthetic phase-routing controls do not qualify a real campaign or native cause."""
        panic = (
            "thread 'selection::selection_guards::capacity_neighbor_copies_all_values_in_order' "
            '(12567) panicked at crates/time-core/src/../tests/private/selection_guards.rs:365:5:\n'
            'assertion failed: matches!(cell_depth(batch, 1, 0),\n'
            '    Err(SelectionError::Invariant\n'
            '    { stage: SelectionInvariantStage::Component }))\n')
        profile = 'S13_SELECTION_PROFILE scope=interval intervals=1 minima=0 batches=0 selections=0\n'
        failure = ('failures:\n\n'
                   '    selection::selection_guards::capacity_neighbor_copies_all_values_in_order\n\n'
                   'test result: FAILED. 0 passed; 1 failed; 0 ignored;\n')
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary)
            caught = {'scenario': {'Mutant': {'name': 'literal-assertion-tail'}},
                      'summary': 'CaughtMutant', 'log_path': 'caught.log',
                      'phase_results': self.phase_results('CaughtMutant')}
            (output / 'caught.log').write_text(self.completed_mutation_log(
                panic + profile + failure, include_selection=False))
            timeout = {'scenario': {'Mutant': {'name': 'literal-assertion-tail-timeout'}},
                       'summary': 'Timeout', 'log_path': 'timeout.log'}
            (output / 'timeout.log').write_text(
                self.mutation_phase_header('Test')
                + '{"reason":"build-finished","success":true}\n' + panic + profile)
            for outcome in (caught, timeout):
                report = {}
                label = 'selection:' + outcome['summary']
                MODULE.record_counts(report, label, {
                    'charged_fit_work_upper': 5990, 'point_admissions_upper': 269245,
                    **MODULE.selection_reservation(package=True), 'observation_pending': True})
                row = report['core_invocations'][0]
                with self.subTest(summary=outcome['summary']):
                    MODULE.observe_mutation_outcome(report, output / 'outcomes.json', label, outcome, row)
                    self.assertEqual(row['charged_fit_work_upper'], 5990)
                    self.assertEqual(row['point_admissions_upper'], 269245)
                    if outcome['summary'] == 'CaughtMutant':
                        self.assertFalse(row['observation_pending'])
                        self.assertEqual(row['observed_counts']['selection_recorded_call_subtotals']['intervals'], 1)
                        self.assertIsNone(row['observed_counts']['selection_observed_calls'])
                        self.assertEqual(report['mutation_counter_prefix_observations'], [label])
                    else:
                        self.assertTrue(row['observation_pending'])
                        self.assertEqual(row['observation_status'], 'timeout-prefix-unqualified')
                        self.assertNotIn('observed_counts', row)

    def test_observed_build_script_transport_is_bounded_and_never_an_error_or_counter(self):
        record = self.build_script_record()
        payload = 'before\r\n' + json.dumps(record) + '\r\nafter\r\n'
        parsed, raw = MODULE.cargo_json_records(payload)
        self.assertEqual(parsed, [record])
        self.assertEqual(raw, 'before\r\nafter\r\n')
        outcome = {'summary': 'Unviable', 'phase_results': self.phase_results('Unviable')}
        semantic = self.semantic_mutation_log().replace(
            self.mutation_phase_header('Build'),
            self.mutation_phase_header('Build') + json.dumps(record) + '\n', 1)
        MODULE.unviable_compile_failure(outcome, semantic)
        script_only = (self.mutation_phase_header('Build') + json.dumps(record) + '\n'
                       + '{"reason":"build-finished","success":false}\n')
        with self.assertRaises(MODULE.Incomplete):
            MODULE.unviable_compile_failure(outcome, script_only)
        synthetic_env = copy.deepcopy(record)
        synthetic_env['env'] = [['KEY', 'CORE_RUNS_PROFILE points=2 inputs=0 selections=0']]
        _, cleaned = MODULE.cargo_json_records(json.dumps(synthetic_env) + '\n')
        self.assertEqual(cleaned, '')
        bad = []
        for env in ([['KEY']], [['KEY', 1]], {'KEY': 'value'}, [['a', 'b', 'c']], [['a', 'b']] * 257):
            changed = copy.deepcopy(record); changed['env'] = env; bad.append(changed)
        missing = copy.deepcopy(record); del missing['cfgs']; bad.append(missing)
        extra = copy.deepcopy(record); extra['unreviewed'] = True; bad.append(extra)
        foreign = copy.deepcopy(record); foreign['package_id'] = 'path+file:///foreign#0.0.0'; bad.append(foreign)
        path = copy.deepcopy(record); path['out_dir'] = '/tmp/unowned/out'; bad.append(path)
        overlong = copy.deepcopy(record); overlong['cfgs'] = ['x' * 4097]; bad.append(overlong)
        unknown = copy.deepcopy(record); unknown['reason'] = 'unknown'; bad.append(unknown)
        for changed in bad:
            with self.subTest(record=repr(changed)[:120]), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(json.dumps(changed) + '\n')

    def test_caught_outcome_rejects_json_error_despite_real_failed_test_block(self):
        with tempfile.TemporaryDirectory() as temp:
            output = Path(temp)
            failure = ('CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n'
                       '\nfailures:\n    actual_test\n\n'
                       'test result: FAILED. 0 passed; 1 failed; 0 ignored;\n')
            log = self.completed_mutation_log(json.dumps(self.semantic_record()) + '\n' + failure)
            (output / 'mutant.log').write_text(log)
            outcome = {'scenario': {'Mutant': {'name': 'one'}}, 'summary': 'CaughtMutant',
                       'log_path': 'mutant.log', 'phase_results': self.phase_results('CaughtMutant')}
            report = {}
            MODULE.record_counts(report, 'runs:mutation-1', {
                'charged_fit_work_upper': 5990, 'point_admissions_upper': 269245,
                'observation_pending': True})
            with self.assertRaisesRegex(MODULE.Incomplete, 'conflicts with an error diagnostic'):
                MODULE.observe_mutation_outcome(report, output / 'outcomes.json',
                                                'runs:mutation-1', outcome, report['core_invocations'][0])
            self.assertTrue(report['core_invocations'][0]['observation_pending'])
            self.assertNotIn('observed_counts', report['core_invocations'][0])

    def test_unviable_requires_native_compile_diagnostic_not_environment_failure(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            (output / 'baseline.log').write_text(self.completed_mutation_log(self.complete_runs_rows()))
            outcome = {'scenario': {'Mutant': {'name': 'build_failure'}}, 'summary': 'Unviable',
                       'log_path': 'compile.log', 'phase_results': self.phase_results('Unviable')}
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')}, outcome]}))
            diagnostic = self.semantic_mutation_log()
            (output / 'compile.log').write_text(diagnostic)
            report = {'subjects': self.historical_scope('runs').select('all')}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            self.assertEqual(report['mutation_full_accounting']['runs']['counts']['U'], 1)
            self.assertEqual(report['core_invocations'][1]['charged_fit_work_upper'], 0)
            for invalid in ['error: could not execute process: Resource temporarily unavailable (os error 11)\n',
                            diagnostic + 'error: could not execute process: Resource temporarily unavailable (os error 11)\n',
                            diagnostic + 'CORE_RUNS_PROFILE points=1 inputs=0 selections=0\n',
                            diagnostic + 'error: internal compiler error: unexpected panic\n',
                            diagnostic + "thread 'rustc' panicked at compiler.rs:1:1\n",
                            diagnostic.replace('crates/time-core/src/regression.rs', 'foreign/src/lib.rs')]:
                (output / 'compile.log').write_text(invalid)
                incomplete = {'subjects': self.historical_scope('runs').select('all')}
                with self.subTest(log=invalid), patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.mutation_accounting(incomplete, work)
                self.assertEqual(incomplete['core_invocations'][1]['charged_fit_work_upper'], 5990)
                self.assertTrue(incomplete['core_invocations'][1]['observation_pending'])
            (output / 'compile.log').write_text(diagnostic)
            outcome['phase_results'][0]['process_status'] = {'Failure': 2}
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')}, outcome]}))
            incomplete = {'subjects': self.historical_scope('runs').select('all')}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_accounting(incomplete, work)
            self.assertEqual(incomplete['core_invocations'][1]['charged_fit_work_upper'], 5990)
            self.assertTrue(incomplete['core_invocations'][1]['observation_pending'])

    def test_failed_native_mutant_retains_valid_prefix_and_rejects_setup_or_resource_failures(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp)
            output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            (output / 'baseline.log').write_text(self.completed_mutation_log(self.complete_runs_rows()))
            partial = 'CORE_RUNS_PROFILE points=2 inputs=0 selections=0\n'
            failure = '\nfailures:\n    frozen_assertion\n\ntest result: FAILED. 0 passed; 1 failed; 0 ignored;\n'
            (output / 'killed.log').write_text(self.completed_mutation_log(partial + failure))
            outcomes = [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')},
                {'scenario': {'Mutant': {'name': 'one_fixture_change'}},
                 'summary': 'CaughtMutant', 'log_path': 'killed.log',
                 'phase_results': self.phase_results('CaughtMutant')},
            ]
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': outcomes}))
            report = {'subjects': self.historical_scope('runs').select('all')}
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            self.assertEqual(report['core_charged_subset_totals'],
                             {'charged_fit_work_upper': 11980, 'point_admissions_upper': 538496})
            caught = report['core_invocations'][1]
            self.assertEqual(caught['observed_counts']['runs_points'], 2)
            self.assertEqual(caught['raw_log_sha256'],
                             hashlib.sha256(self.completed_mutation_log(partial + failure).encode()).hexdigest())
            self.assertEqual(report['mutation_full_accounting']['runs']['counts']['K'], 1)
            self.assertEqual(report['mutation_counter_prefix_observations'], ['runs:mutation-1'])
            for invalid in [partial,
                            partial.replace('selections=0', 'selections=0 extra=1') + failure,
                            partial.replace('points=2', 'points=129') + failure,
                            partial + failure.replace('    frozen_assertion', '    src/lib.rs - (line 3)')]:
                (output / 'killed.log').write_text(self.completed_mutation_log(invalid))
                with self.subTest(log=invalid), patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                    with self.assertRaises(MODULE.Incomplete):
                        MODULE.mutation_accounting({'subjects': self.historical_scope('runs').select('all')}, work)
            (output / 'killed.log').write_text(self.completed_mutation_log(partial + failure))
            # Well-formed missing runs rows remain incomplete for a survivor.
            outcomes[1]['summary'] = 'MissedMutant'
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': outcomes}))
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.mutation_accounting({'subjects': self.historical_scope('runs').select('all')}, work)

    def test_mixed_direct_and_nested_observations_are_summed_once_with_pending_state(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp)
            output = work / 'mutation/runs/mutants.out'
            output.mkdir(parents=True)
            text = ('CORE_PROFILE_COUNTS fits=1 points=3\n'
                    'CORE_STATS_PROFILE_COUNTS stats=1 points=3\n' + self.complete_runs_rows())
            for name in ['baseline.log', 'survivor.log']:
                (output / name).write_text(self.completed_mutation_log(text))
            (output / 'outcomes.json').write_text(json.dumps({'total_mutants': 1, 'outcomes': [
                {'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                 'phase_results': self.phase_results('Success')},
                {'scenario': {'Mutant': {'name': 'unchanged_fixture'}},
                 'summary': 'MissedMutant', 'log_path': 'survivor.log',
                 'phase_results': self.phase_results('MissedMutant')}]}))
            report = {'subjects': self.historical_scope('runs').select('all')}
            MODULE.record_counts(report, 'known_direct', {
                'charged_fit_work_upper': 3, 'point_admissions_upper': 9,
                'direct_fit_calls': 2, 'stats_calls': 1, 'point_admissions': 9})
            with patch.dict(MODULE.PROFILE_LIMITS, {'charged_fit_work_upper': 2000}):
                MODULE.mutation_accounting(report, work)
            expected = {'direct_fit_calls': 4, 'stats_calls': 3, 'point_admissions': 21}
            self.assertEqual(report['core_direct_and_stats_observations'], expected)
            self.assertEqual(report['core_runs_external_observations']['runs_selections'], 40)
            self.assertEqual(report['core_charged_subset_totals'],
                             {'charged_fit_work_upper': 11983, 'point_admissions_upper': 538505})
            self.assertTrue(report['core_recorded_observations_parsed'])
            MODULE.refresh_accounting(report)
            self.assertEqual(report['core_direct_and_stats_observations'], expected)
            self.assertEqual(report['core_runs_external_observations']['runs_selections'], 40)
            MODULE.record_counts(report, 'not_observed_yet', {
                'charged_fit_work_upper': 3, 'point_admissions_upper': 9, 'observation_pending': True})
            self.assertFalse(report['core_recorded_observations_parsed'])
            self.assertEqual(report['core_observation_pending_invocations'], ['not_observed_yet'])
            self.assertEqual(report['core_direct_and_stats_observations'], expected)
            report['core_invocations'][-1]['observed_counts'] = {
                'direct_fit_calls': 3, 'stats_calls': 0, 'point_admissions': 9}
            report['core_invocations'][-1]['observation_pending'] = False
            MODULE.refresh_accounting(report)
            self.assertTrue(report['core_recorded_observations_parsed'])
            self.assertEqual(report['core_direct_and_stats_observations'],
                             {'direct_fit_calls': 7, 'stats_calls': 3, 'point_admissions': 30})

    def test_missing_mutation_cohort_is_distinct_from_parsed_recorded_observations(self):
        with tempfile.TemporaryDirectory() as temp:
            report = {'subjects': self.historical_scope('runs').select('all')}
            MODULE.record_counts(report, 'known_direct', {
                'charged_fit_work_upper': 1, 'point_admissions_upper': 3,
                'direct_fit_calls': 1, 'stats_calls': 0, 'point_admissions': 3})
            MODULE.mutation_accounting(report, Path(temp))
            self.assertTrue(report['core_recorded_observations_parsed'])
            self.assertFalse(report['mutation_counter_observation_complete'])
            self.assertEqual(report['mutation_missing_selected_files'], ['runs'])

    def phase_results(self, summary):
        """Create inert pinned build/test phase observations for actual gate controls."""
        phases = []
        for phase in (['Build'] if summary == 'Unviable' else ['Build', 'Test']):
            failed = summary == 'Unviable' or (summary == 'CaughtMutant' and phase == 'Test')
            phases.append({'phase': phase, 'duration': 0.01,
                           'argv': ['cargo', 'test', *(['--no-run'] if phase == 'Build' else []),
                                    '--verbose', '--package=time-core@0.0.0', '--message-format=json', '--frozen'],
                           'process_status': {'Failure': 101} if failed else 'Success'})
        return phases

    def mutation_report(self, filename, caught, missed):
        """Produce complete independent synthetic fault identities and raw outcomes."""
        expected = {}; outcomes = [{'scenario': 'Baseline', 'summary': 'Success', 'log_path': 'baseline.log',
                                    'phase_results': self.phase_results('Success')}]
        for index in range(caught + missed):
            mutant = {'file': filename, 'package': 'time-core', 'name': filename + ':' + str(index),
                      'function': {'function_name': 'checked_guard', 'return_type': '-> bool',
                                   'span': {'start': {'line': 1, 'column': 1}, 'end': {'line': 2, 'column': 2}}},
                      'span': {'start': {'line': 2, 'column': 1}, 'end': {'line': 2, 'column': 2}},
                      'replacement': 'false', 'genre': 'FnValue'}
            expected[mutant['name']] = mutant
            summary = 'CaughtMutant' if index < caught else 'MissedMutant'
            outcomes.append({'scenario': {'Mutant': mutant}, 'summary': summary,
                             'phase_results': self.phase_results(summary)})
        return ({'cargo_mutants_version': '27.1.0', 'total_mutants': caught + missed, 'caught': caught,
                 'missed': missed, 'unviable': 0, 'timeout': 0, 'success': 0, 'outcomes': outcomes}, expected)

    def test_each_file_gate_preserves_95_percent_and_no_unresolved_rule(self):
        for filename in MODULE.SUBJECTS.values():
            bad, expected = self.mutation_report(filename, 19, 1)
            result = MODULE.COMMON.mutation_counts(MODULE.SCOPE, bad, expected, filename, native_exit_code=2, defer_gate=True)
            self.assertEqual(result['adjusted_viable_killed_percent'], 95)
            self.assertFalse(result['gate_passed'])
            self.assertEqual(len(result['unresolved_survivors']), 1)
            with self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.mutation_counts(MODULE.SCOPE, bad, expected, filename, native_exit_code=2)
            healthy, expected = self.mutation_report(filename, 3, 0)
            self.assertTrue(MODULE.COMMON.mutation_counts(MODULE.SCOPE, healthy, expected, filename)['gate_passed'])
            for changed in [dict(healthy, outcomes=healthy['outcomes'][1:]), dict(healthy, total_mutants=4),
                            dict(healthy, outcomes=healthy['outcomes'] + healthy['outcomes'][-1:]),
                            dict(healthy, timeout=1)]:
                with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.mutation_counts(MODULE.SCOPE, changed, expected, filename)
            with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.mutation_counts(MODULE.SCOPE, healthy, {}, filename)

    def test_complete_mutation_loop_has_two_package_baselines_and_one_shared_deadline(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); report = {}; clock = [100.0]; launches = []
            def run(label, command, timeout, **options):
                launches.append((label, list(command), timeout, options))
                subject = label.split(':')[1]; filename = MODULE.SUBJECTS[subject]
                raw, expected = self.mutation_report(filename, 1, 0)
                if label.startswith('mutation-enumeration:'):
                    clock[0] += 50
                    return json.dumps(list(expected.values()))
                out = work / 'mutation' / subject / 'mutants.out'; out.mkdir(parents=True)
                (out / 'outcomes.json').write_text(json.dumps(raw))
                (out / 'baseline.log').write_text('test result: ok. 2 passed; 0 failed; 0 ignored;\n')
                clock[0] += 75
                return {'exit_code': 0, 'failure': None, 'output': 'inert actual-loop control'}
            with patch.object(MODULE.COMMON.time, 'monotonic', side_effect=lambda: clock[0]):
                MODULE.COMMON.run_analysis_lanes(MODULE.SCOPE, MODULE.SUBJECTS,
                    {'mutation_equivalences': [], 'maximum_mutants': 512, 'mutation_wall_seconds': 1200},
                    work, Path('/inert-tools'), run, report, 'mutation')
            campaigns = [row for row in launches if row[0].startswith('mutation-campaign:')]
            self.assertEqual([row[0] for row in campaigns], ['mutation-campaign:principal', 'mutation-campaign:regression'])
            self.assertEqual([row[2] for row in campaigns], [1100, 1025])
            for _, command, _, _ in campaigns:
                self.assertEqual(command[command.index('--baseline') + 1], 'run')
                self.assertEqual(command[command.index('--build-timeout') + 1], '90')
                self.assertEqual(command[command.index('--timeout') + 1], '60')
            self.assertEqual(set(report['mutation']), set(MODULE.SUBJECTS))
            self.assertTrue(all(row['gate_passed'] for row in report['mutation'].values()))

    def test_combined_variant_cap_and_separate_global_custody_are_explicit(self):
        first = {str(i): {} for i in range(256)}; second = dict(first)
        self.assertEqual(MODULE.COMMON.combined_mutant_count({'regression': first, 'residuals': second}, 512), 512)
        second['extra'] = {}
        with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.combined_mutant_count({'regression': first, 'residuals': second}, 512)
        with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.combined_mutant_count({}, 512)
        # Cross-invocation global512 reservation is the coordinator's obligation;
        # this actual helper cap is per complete selected enumeration, not a fake
        # persistent ledger or permission to reset the global budget on retry.

    def test_corpus_has_frozen_representable_inputs_and_bounded_raw_extents(self):
        seeds = MODULE.fuzz_seeds()
        self.assertEqual(sum(name.startswith('normalization-') for name in seeds), 57)
        for name in ['normalization-one_ns_admitted', 'normalization-count_zero', 'malformed-count65', 'malformed-truncated']:
            self.assertIn(name, seeds)
        self.assertEqual(len(seeds['normalization-count_64_uniform']), 1537)
        self.assertEqual(len(seeds['normalization-one_ns_admitted']), 73)
        self.assertTrue(all(len(data) <= 1537 for data in seeds.values()))
        self.assertEqual(seeds['normalization-one_ns_admitted'][25:33], (62499999).to_bytes(8,'big'))
        with self.assertRaises(MODULE.Incomplete): MODULE.encode_case([(0.0, 0.0, 1.0)] * 65)
        with self.assertRaises(MODULE.Incomplete): MODULE.encode_bits_case([(0, 0, 0)] * 65)
        with self.assertRaises(MODULE.Incomplete): MODULE.encode_bits_case([(0, True, 0)])

    def test_global_subset_budget_counts_invocations_without_claiming_global_ownership(self):
        report = {}
        for label in ['coverage', 'mutation-baseline']:
            MODULE.record_counts(report, label, MODULE.profile_counts('CORE_PROFILE_COUNTS fits=285 points=1329\nCORE_STATS_PROFILE_COUNTS stats=169 points=1330\n'))
        self.assertEqual(report['core_charged_subset_totals'], {'charged_fit_work_upper': 908, 'point_admissions_upper': 5318})
        self.assertEqual(report['core_direct_and_stats_observations'], {'direct_fit_calls': 570, 'stats_calls': 338, 'point_admissions': 5318})
        self.assertIn('Coordinator', report['global_campaign_custody'])
        for changed in [{'charged_fit_work_upper': MODULE.GLOBAL_LIMITS['charged_fit_work_upper'], 'point_admissions_upper': 0},
                        {'charged_fit_work_upper': 0, 'point_admissions_upper': MODULE.GLOBAL_LIMITS['point_admissions_upper']},
                        {'fit_calls': 1, 'point_admissions': 3},
                        {'charged_fit_work_upper': True, 'point_admissions_upper': 0}]:
            with self.subTest(counts=changed), self.assertRaises(MODULE.Incomplete):
                MODULE.record_counts(copy.deepcopy(report), 'excess', changed)

    def test_cli_defaults_to_both_checkpoint_subjects_and_named_diagnostics_remain_narrow(self):
        selected = []
        def capture(args, report):
            selected.append(args.subject)
            raise MODULE.Incomplete('inert argument-selection probe')
        with tempfile.TemporaryDirectory() as temp:
            args = ['--output-dir', temp, '--vendor-dir', temp]
            with patch.object(MODULE, 'execute', side_effect=capture), contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
                self.assertEqual(MODULE.main(args), 1)
                self.assertEqual(MODULE.main(args + ['--subject', 'principal']), 1)
                self.assertEqual(MODULE.main(args + ['--subject', 'regression']), 1)
        self.assertEqual(selected, ['all', 'principal', 'regression'])
        self.assertEqual(len(MODULE.SCOPE.select('all')), 2)
        self.assertEqual(len(MODULE.SCOPE.select('regression')), 1)
        with self.assertRaises(MODULE.Incomplete): MODULE.SCOPE.select('../residuals')

    def test_fuzz_supervisor_uses_reviewed_wall_limit_for_both_processes(self):
        """Observe actual orchestration using explicitly synthetic process receipts."""
        with tempfile.TemporaryDirectory() as temp:
            base = Path(temp)
            root, work, output = base / 'root', base / 'work', base / 'output'
            for directory in [root, work, output]:
                directory.mkdir(mode=0o700)
            for name in MODULE.FUZZ_PATHS.values():
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text('isolated orchestration fixture; never executed\n')
            installation = base / 'installation'
            launcher = installation / 'common/bin/cargo-fuzz'
            launcher.parent.mkdir(parents=True)
            launcher.write_bytes(b'isolated tool-copy fixture; never executed\n')
            calls = []
            report = {'lanes': []}

            def synthetic_history_row(raw):
                """Literal cumulative recipe: one history and three appends per callback."""
                return (f'CORE_HISTORY_FUZZ_COUNTS raw={raw} histories={raw} '
                        f'appends={3*raw} observations={3*raw} contexts={raw} '
                        f'monotonic={3*raw} coordinates={12*raw} exchanges={3*raw} '
                        f'getters={15*raw} normalizations=0 stats=0 fits=0 points=0\n')

            healthy_output = ''.join(synthetic_history_row(raw) for raw in range(1, 4097))
            healthy_output += ('HISTORY_FUZZ_KNOWN_REPORT_CHECKED synthetic-orchestration-fixture\n'
                               'Done 4096 runs\n')
            defect_output = (synthetic_history_row(1)
                             + "thread '<unnamed>' panicked at target.rs:1:1:\n"
                             'independent history oracle: frozen published report bits\n')

            class FakeSession:
                """Record commands, inert build bytes and explicitly synthetic lane receipts."""

                def configure(self, text):
                    self.configuration = text

                def protect(self, path):
                    pass

                def run(self, label, command, timeout, **options):
                    calls.append((label, tuple(command), timeout, options))
                    if label == 'fuzz-graph':
                        return '{}'
                    if label.startswith('fuzz-build-'):
                        binary = work / 'target-fuzz' / MODULE.ANALYSIS.PLATFORM / 'release/fit'
                        binary.parent.mkdir(parents=True, exist_ok=True)
                        binary.write_bytes(label.encode())
                        return ''
                    if label in {'fuzz-healthy', 'fuzz-defect'}:
                        text = healthy_output if label == 'fuzz-healthy' else defect_output
                        report['lanes'].append({'lane': label,
                            'exit_code': 0 if label == 'fuzz-healthy' else 77,
                            'failure': None, 'output': text})
                        # Real Session.run preserves the receipt's raw output,
                        # while require_ok strips its healthy return value.
                        return text if label == 'fuzz-defect' else text.strip()
                    raise AssertionError('Unexpected orchestration command: ' + label)

            session = FakeSession()
            session.work, session.output, session.base = work, output, installation
            session.vendor = base / 'unused-vendor'
            session.protected_tools = {}
            session.admitted = {'policy': {'common_nightly': 'fixture-nightly',
                'tools': {'fuzz': {'sha256': hashlib.sha256(launcher.read_bytes()).hexdigest()}}}}
            selected = {**MODULE.FUZZ_PATHS, 'locked_registry_packages': [],
                        'invocation_limits': {'wall_seconds': 15}}
            # Mock adjacent admission only. Real fuzz() selects every timeout,
            # binds each unique receipt, classifies its cause and parses counters.
            # These synthetic observations establish no Rust/native execution.
            with patch.object(MODULE, 'ROOT', root), \
                 patch.object(MODULE, 'admit_fuzz_sources', return_value={'locked_registry_packages': []}), \
                 patch.object(MODULE, 'fuzz_metadata', return_value={}), \
                 patch.object(MODULE.COMMON, 'fuzz_input_snapshot', return_value={'fixture': 'unchanged'}):
                MODULE.fuzz(session, selected, report)
            self.assertEqual([timeout for label, _, timeout, _ in calls if label == 'fuzz-graph'], [120])
            self.assertEqual([timeout for label, _, timeout, _ in calls if label.startswith('fuzz-build-')], [600, 600])
            actual = [(label, timeout) for label, _, timeout, _ in calls
                      if label in {'fuzz-healthy', 'fuzz-defect'}]
            self.assertEqual(actual, [('fuzz-healthy', 15), ('fuzz-defect', 15)])
            for label, _, _, options in calls:
                if label in {'fuzz-healthy', 'fuzz-defect'}:
                    self.assertEqual(options['mode'], 'sanitizer')
                    self.assertEqual(options['sanitizer_profile'], 'time-core-history-s12d')
                    self.assertEqual(options['negative'],
                        'independent history oracle: frozen published report bits'
                        if label == 'fuzz-defect' else None)

    def test_normalization_only_work_and_private_zero_records_are_distinct(self):
        result = MODULE.profile_counts(self.norm_row())
        self.assertEqual(result['normalization_requests'], 1)
        self.assertEqual(result['direct_fit_calls'], 0)
        self.assertEqual(result['charged_fit_work_upper'], 0)
        self.assertEqual(result['point_admissions'], 0)
        self.assertEqual(result['generated_point_admissions_upper'], 64)
        self.assertEqual(result['duration_operations_upper'], 64)
        self.assertEqual(result['fit_input_constructions_upper'], 1)
        zero = self.norm_row(**{key: 0 for key in ['normalizations','observations','contexts','monotonic','coordinates','exchanges','getters','stats','points']})
        with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(zero)
        observed = MODULE.profile_counts(zero+'\n'+zero, require_fit=False)
        self.assertEqual(observed['normalization_counter_records'], 2)
        self.assertEqual(observed['point_admissions_upper'], 0)
        # Identical zero scopes are legitimate. Missing whole zero rows or replay
        # of an otherwise identical named scope needs root discovery custody.

    def test_normalization_transport_rejects_malformed_wrong_schema_and_replay(self):
        valid = self.norm_row()
        bad = [valid.replace('observations=3','observations=-1'), valid.replace('contexts=1','contexts=True'),
               valid.replace('monotonic=3','monotonic=3.0'), valid.replace(' coordinates=12',''),
               valid+' raw=0', valid+'\nCORE_NORMALIZATION_PROFILE_COUNTS incomplete',
               valid.replace('getters=5','getters='+('9'*100)), valid+' '+valid,
               valid.replace('stats=0 points=0','points=0 stats=0'),
               '--- STDERR:\n'+valid, 'Summary [0s]\n'+valid,
               valid+'\nCORE_NORMALIZATION_FUZZ_COUNTS raw=1 normalizations=1']
        for text in bad:
            with self.subTest(text=text[:100]), self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(text)

    def test_every_normalization_profile_cap_has_a_neighbor(self):
        caps = {'normalizations':128,'observations':8320,'contexts':256,'monotonic':8320,
                'coordinates':33280,'exchanges':8320,'getters':8448,'stats':16}
        for field, maximum in caps.items():
            changes = {field:maximum, 'normalizations':128}
            good = MODULE.profile_counts(self.norm_row(**changes))
            self.assertEqual(good['normalization_requests'],128)
            changes[field] = maximum+1
            with self.subTest(field=field), self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(self.norm_row(**changes))
        with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(self.norm_row(getters=67))

    def test_disjoint_actual_points_and_generated_upper_share_the_point_budget(self):
        exact = MODULE.profile_counts(self.norm_row(points=65536-64))
        self.assertEqual(exact['point_admissions'],65472)
        self.assertEqual(exact['point_admissions_upper'],65536)
        with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(self.norm_row(points=65536-63))
        legacy='CORE_PROFILE_COUNTS fits=285 points=1329\nCORE_STATS_PROFILE_COUNTS stats=169 points=1330\n'
        counts=MODULE.profile_counts(legacy+self.norm_row(normalizations=128,stats=16))
        self.assertEqual(counts['stats_calls'],185)
        self.assertEqual(counts['charged_fit_work_upper'],470)
        self.assertEqual(counts['point_admissions'],2659)
        self.assertEqual(counts['point_admissions_upper'],10851)
        with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts('CORE_PROFILE_COUNTS fits=640 points=0\n'+self.norm_row(stats=1))

    def test_normalization_doc_allowances_do_not_fabricate_actual_calls(self):
        observed=MODULE.profile_counts(self.norm_row())
        counts=MODULE.account_literal_doctests(observed)
        self.assertEqual(counts['normalization_requests'],1)
        self.assertEqual(counts['stats_calls'],0)
        self.assertEqual(counts['point_admissions'],0)
        self.assertEqual(counts['literal_doctest_normalization_requests_upper'],0)
        self.assertEqual(counts['literal_doctest_utc_coordinate_constructor_attempts_upper'],0)
        self.assertEqual(counts['literal_doctest_point_getter_requests_upper'],0)
        self.assertEqual(counts['point_admissions_upper'],67)
        self.assertNotIn('literal_doctest_normalization_requests_upper', observed)

    def test_pending_observer_and_normalization_path_drift_cannot_be_admitted(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp); policy=self.fixture(root)
            policy['normalization_telemetry']['status']='pending-source-owned-observer-and-literal-audit'
            self.write_policy(root,policy)
            with self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root,'contracts')
            policy=self.fixture(root)
            for name in ['crates/time-core/src/regression/normalization.rs','crates/time-core/tests/normalization.rs',
                         'crates/time-core/tests/support/normalization_facts.rs','crates/time-core/tests/private/normalization_guards.rs']:
                path=root/name; original=path.read_bytes(); path.write_bytes(original+b'drift')
                with self.subTest(path=name), self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root,'contracts')
                path.write_bytes(original)

    def test_normalization_legal_consumers_and_eight_causal_negatives_have_finite_recipes(self):
        cases=MODULE.normalization_contract_examples(); legal=[case for case in cases.values() if case['code'] is None]
        self.assertEqual(len(legal),2); self.assertEqual(len(cases)-len(legal),8)
        self.assertEqual(sum(case['calls']['normalization_requests'] for case in legal),2)
        self.assertEqual(sum(case['calls']['observation_constructor_attempts'] for case in legal),5)
        self.assertEqual(sum(case['calls']['utc_coordinate_constructor_attempts'] for case in legal),20)
        self.assertEqual(sum(case['calls']['point_getter_requests'] for case in legal),5)
        for case in legal:
            self.assertEqual(case['calls']['stats_calls']+case['calls']['direct_fit_calls'],0)
            self.assertLessEqual(case['calls']['observation_constructor_attempts'],3)
            self.assertLessEqual(case['calls']['point_admissions']+case['internal_upper']['generated_point_admissions_upper'],3)
        self.assertEqual(cases['norm_raw_q33_constructor']['code'],'E0451')
        self.assertEqual(cases['norm_utc_substitution']['code'],'E0308')
        self.assertEqual(cases['norm_immutable_point']['code'],'E0594')

    def test_real_domain_artifact_selector_rejects_wrong_manifest_identity_or_path(self):
        manifest='/work/source/crates/time-domain/Cargo.toml'
        item={'reason':'compiler-artifact','manifest_path':manifest,
              'package_id':'path+file:///work/source/crates/time-domain#0.0.0',
              'target':{'name':'time_domain','kind':['lib']},
              'filenames':['/work/target/debug/deps/libtime_domain-reviewed.rlib']}
        text=json.dumps(item)
        self.assertEqual(MODULE.COMMON.library_artifact(text,'time-domain',manifest),item['filenames'][0])
        for altered in [dict(item,manifest_path='/wrong/Cargo.toml'),dict(item,package_id='registry+wrong'),
                        dict(item,target={'name':'time_core','kind':['lib']}),
                        dict(item,filenames=['/outside/libtime_domain.rlib']),
                        dict(item,filenames=[]),dict(item,filenames=[*item['filenames'],'/work/target/other.rlib'])]:
            with self.subTest(altered=altered), self.assertRaises(MODULE.Incomplete): MODULE.COMMON.library_artifact(json.dumps(altered),'time-domain',manifest)
        with self.assertRaises(MODULE.Incomplete): MODULE.COMMON.library_artifact(text+'\n'+text,'time-domain',manifest)

    def test_core_and_domain_artifacts_are_copied_and_sealed_without_aliases(self):
        with tempfile.TemporaryDirectory() as temp:
            work=Path(temp); deps=work/'target/debug/deps'; deps.mkdir(parents=True)
            core=deps/'libtime_core-hash.rlib'; domain=deps/'libtime_domain-hash.rlib'
            core.write_bytes(b'explicit inert core artifact'); domain.write_bytes(b'explicit inert domain artifact')
            (deps/'cargo-owned-domain-alias.rlib').hardlink_to(domain)
            class Session:
                def protect(self,path): self.sealed=MODULE.PRODUCT.inventory(path)
            session=Session(); session.work=work
            result=MODULE.copy_contract_dependencies(session,'/work/target/debug/deps/'+core.name,'/work/target/debug/deps/'+domain.name)
            self.assertEqual((result/'libtime_core.rlib').read_bytes(),core.read_bytes())
            self.assertEqual((result/'libtime_domain.rlib').read_bytes(),domain.read_bytes())
            self.assertEqual(session.sealed,MODULE.PRODUCT.inventory(result))
            self.assertNotEqual((result/'libtime_domain.rlib').stat().st_ino,domain.stat().st_ino)
            self.assertEqual((result/'libtime_domain.rlib').stat().st_mode & 0o222,0)

    def test_redirected_or_escaped_domain_artifacts_are_rejected(self):
        for mode in ['symlink','escaped']:
            with tempfile.TemporaryDirectory() as temp:
                work=Path(temp); deps=work/'target/debug/deps'; deps.mkdir(parents=True)
                core=deps/'core.rlib'; core.write_bytes(b'inert core')
                actual=deps/'actual.rlib'; actual.write_bytes(b'inert domain')
                domain=deps/'domain.rlib'
                if mode=='symlink': domain.symlink_to(actual)
                else: domain=work/'outside.rlib'; domain.write_bytes(b'escaped fixture')
                session=type('Session',(),{'work':work,'protect':lambda self,path:None})()
                relative='/work/'+str(domain.relative_to(work))
                with self.subTest(mode=mode), self.assertRaises(MODULE.Incomplete):
                    MODULE.copy_contract_dependencies(session,'/work/target/debug/deps/core.rlib',relative)

    def test_actual_contract_orchestrator_passes_both_explicit_externs(self):
        class Captured(Exception): pass
        with tempfile.TemporaryDirectory() as temp:
            work=Path(temp); commands=[]
            class Session:
                def protect(self,path): pass
                def run(self,label,command,timeout,**options):
                    commands.append((label,command,options))
                    if label=='contract-library': return 'inert compiler-artifact envelope selected by adjacent stub'
                    if label.startswith('external-consumer-'): raise Captured
                    raise AssertionError(label)
            session=Session(); session.work=work; session.stable=Path('/inert-stable')
            with patch.object(MODULE.COMMON,'library_artifact',side_effect=['/work/target/core.rlib','/work/target/domain.rlib']) as selector, \
                 patch.object(MODULE,'copy_contract_dependencies',return_value=work/'contract-deps') as copier:
                with self.assertRaises(Captured): MODULE.contracts(session,{'contracts_wall_seconds':300},{})
            self.assertEqual(selector.call_args_list[1].args[1:],('time-domain','/work/source/crates/time-domain/Cargo.toml'))
            self.assertEqual(copier.call_args.args[1:],('/work/target/core.rlib','/work/target/domain.rlib'))
            command=commands[-1][1]
            self.assertIn('time_core=/work/contract-deps/libtime_core.rlib',command)
            self.assertIn('time_domain=/work/contract-deps/libtime_domain.rlib',command)
            self.assertEqual(command.count('--extern'),2)
            self.assertEqual(len(commands[-1][2]['sealed']),2)

    def test_normalization_wire_encoder_preserves_raw_bits_and_rejects_unbounded_shapes(self):
        row=(2**64-1,-1,0x8000000000000000)
        data=MODULE.encode_normalization_case([row])
        self.assertEqual(data,b'\x01'+(2**64-1).to_bytes(8,'big')+(-1).to_bytes(8,'big',signed=True)+bytes.fromhex('8000000000000000'))
        self.assertEqual(len(MODULE.encode_normalization_case([row]*64)),1537)
        for rows in [[row]*65,[(True,0,0)],[(0,True,0)],[(0,0,True)],[(2**64,0,0)],[(0,2**63,0)],
                     [(0,-2**63,0),(0,-2**63+1,0)]]:
            with self.subTest(rows=str(rows)[:80]), self.assertRaises(MODULE.Incomplete): MODULE.encode_normalization_case(rows)

    def test_normalization_fuzz_rejects_every_callback_bound_and_wrong_owner(self):
        for key,value in [('normalizations',2),('observations',65),('contexts',3),('monotonic',65),
                          ('coordinates',257),('exchanges',65),('getters',67),('stats',2),('points',1)]:
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete): MODULE.fuzz_counts(self.fuzz_row(**{key:value}),complete=False)
        good=self.fuzz_row()
        for text in [good+' extra=0',good+'\nCORE_NORMALIZATION_FUZZ_COUNTS broken',
                     good+'\nCORE_STATS_FUZZ_COUNTS raw=1 points=3 stats=1',
                     '--- STDERR:\n'+good,good.replace('raw=1','raw=18446744073709551616')]:
            with self.subTest(text=text), self.assertRaises(MODULE.Incomplete): MODULE.fuzz_counts(text,complete=False)

    def test_three_local_fuzz_graph_and_registry_edges_are_exact(self):
        selected=json.loads((ROOT/MODULE.POLICY).read_text())['coverage_guided_fuzzing']
        locked=selected['locked_registry_packages']; edges=selected['gnu_dependency_edges']; features=selected['gnu_reachable_features']
        packages=[{'id':item['name'],'name':item['name'],'source':item['source'],'version':item['version']} for item in locked]
        packages += [{'id':name,'name':name,'source':None,'version':'0.0.0'} for name in ['time-core','time-domain','time-core-fuzz']]
        nodes=[{'id':item['id'],'features':features.get(item['name'],[]),
                'deps':[{'pkg':name} for name in edges.get(item['name'],[])]} for item in packages]
        metadata={'packages':packages,'resolve':{'root':'time-core-fuzz','nodes':nodes}}
        result=MODULE.fuzz_metadata(metadata,selected)
        self.assertEqual(result['dependency_edges']['time-domain'],['garde'])
        self.assertEqual(result['dependency_edges']['time-core'],['garde','time-domain'])
        for change in ['edge','feature','local-source','root']:
            altered=copy.deepcopy(metadata)
            if change=='edge': next(node for node in altered['resolve']['nodes'] if node['id']=='time-core')['deps']=[]
            if change=='feature': next(node for node in altered['resolve']['nodes'] if node['id']=='time-domain')['features']=['unexpected']
            if change=='local-source': next(item for item in altered['packages'] if item['id']=='time-domain')['source']='registry+wrong'
            if change=='root': altered['resolve']['root']='time-domain'
            with self.subTest(change=change), self.assertRaises(MODULE.Incomplete): MODULE.fuzz_metadata(altered,selected)


    @staticmethod
    def history_row(**changes):
        """Independent closed twelve-field transport, never real Rust telemetry."""
        values = dict(histories=1, appends=3, observations=3, contexts=1, monotonic=3,
                      coordinates=12, exchanges=3, getters=20, normalizations=0,
                      stats=0, fits=0, points=0)
        values.update(changes)
        return 'CORE_HISTORY_PROFILE_COUNTS ' + ' '.join(f'{key}={value}' for key,value in values.items())

    def test_history_observed_external_fields_are_separate_from_nested_upper_costs(self):
        counts=MODULE.profile_counts(self.history_row())
        self.assertEqual(counts.get('history_constructions'),1)
        self.assertEqual(counts.get('history_append_requests'),3)
        self.assertEqual(counts.get('history_product_getter_requests'),20)
        self.assertEqual(counts['normalization_requests'],0)
        self.assertEqual(counts['direct_fit_calls'],0)
        self.assertEqual(counts['stats_calls'],0)
        self.assertEqual(counts['observed_charged_fit_work'],0)
        self.assertEqual(counts['charged_fit_work_upper'],3)
        self.assertEqual(counts['internal_fit_upper_bound'],3)
        self.assertEqual(counts['history_normalization_entries_upper'],3)
        self.assertEqual(counts['history_statistics_entries_upper'],3)
        self.assertEqual(counts['generated_point_admissions_upper'],192)
        self.assertEqual(counts['history_staging_Copy_assignments_upper'],381)
        self.assertEqual((counts['counter_records'],counts['history_counter_records']),(1,1))

    def test_history_complete_source_recipe_charges_actual_appends_not_fake_observed_stats(self):
        legacy='CORE_PROFILE_COUNTS fits=285 points=1329\nCORE_STATS_PROFILE_COUNTS stats=169 points=1330\n'
        normal=self.norm_row(normalizations=113,observations=4449,contexts=201,monotonic=4449,
                             coordinates=17796,exchanges=4449,getters=186,stats=3)
        history=self.history_row(histories=17,appends=144,observations=144,contexts=21,monotonic=144,
                                 coordinates=576,exchanges=144,getters=32786)
        counts=MODULE.profile_counts(legacy+normal+'\n'+history)
        self.assertEqual(counts.get('history_append_requests'),144)
        self.assertEqual(counts['stats_calls'],172)
        self.assertEqual(counts['observed_charged_fit_work'],457)
        self.assertEqual(counts['charged_fit_work_upper'],601)
        self.assertEqual(counts['point_admissions'],2659)
        self.assertEqual(counts['point_admissions_upper'],19107)
        self.assertEqual(counts['history_statistics_entries_upper'],144)
        self.assertEqual(counts['history_direct_stats_calls'],0)

    def test_history_zero_constructor_and_raw_guard_scopes_remain_distinct(self):
        empty=self.history_row(histories=1,appends=0,observations=0,contexts=1,monotonic=0,coordinates=0,exchanges=0,getters=4)
        self.assertEqual(MODULE.profile_counts(empty)['history_constructions'],1)
        zero=self.history_row(histories=0,appends=0,observations=0,contexts=0,monotonic=0,coordinates=0,exchanges=0,getters=0)
        with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(zero)
        self.assertEqual(MODULE.profile_counts(zero,require_fit=False)['history_counter_records'],1)

    def test_history_closed_transport_rejects_malformed_extra_wrong_owner_and_replay(self):
        good=self.history_row()
        bad=[good.replace(' appends=3',''),good+' extra=0',good.replace(' appends=3',' appends=-1'),
             good.replace(' appends=3',' appends=18446744073709551616'),good+'\nCORE_HISTORY_PROFILE_COUNTS broken',
             good+'\nCORE_HISTORY_FUZZ_COUNTS raw=1','Summary [replay]\n'+good,'--- STDERR:\n'+good]
        for text in bad:
            with self.subTest(text=text[-80:]),self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(text)

    def test_history_observation_ownership_rejects_untracked_internal_or_setup_fields(self):
        for changes in [dict(normalizations=1),dict(stats=1),dict(fits=1),dict(points=1),
                        dict(monotonic=2),dict(coordinates=11),dict(exchanges=2)]:
            with self.subTest(changes=changes),self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(self.history_row(**changes))

    def test_history_explicit_profile_caps_have_valid_neighboring_records(self):
        good=self.history_row(histories=32,appends=160,observations=256,contexts=64,
                              monotonic=256,coordinates=1024,exchanges=256,getters=65536)
        self.assertEqual(MODULE.profile_counts(good)['history_append_requests'],160)
        for key,value in [('histories',33),('appends',161),('contexts',65),('getters',65537)]:
            with self.subTest(key=key),self.assertRaises(MODULE.Incomplete):
                MODULE.profile_counts(good.replace(f'{key}='+{'histories':'32','appends':'160','contexts':'64','getters':'65536'}[key],f'{key}={value}'))
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(self.history_row(observations=257,monotonic=257,coordinates=1028,exchanges=257))

    def test_history_combined_work_and_point_caps_charge_derived_work_once(self):
        h=self.history_row(appends=160,observations=0,monotonic=0,coordinates=0,exchanges=0)
        for fits,accepted in [(479,True),(480,True),(481,False)]:
            text=f'CORE_PROFILE_COUNTS fits={fits} points=0\n'+h
            if accepted:
                counts=MODULE.profile_counts(text)
                self.assertEqual(counts['charged_fit_work_upper'],fits+160)
                self.assertEqual(counts['observed_charged_fit_work'],fits)
            else:
                with self.assertRaises(MODULE.Incomplete): MODULE.profile_counts(text)
        self.assertEqual(MODULE.profile_counts('CORE_PROFILE_COUNTS fits=1 points=55296\n'+h)['point_admissions_upper'],65536)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts('CORE_PROFILE_COUNTS fits=1 points=55297\n'+h)

    def test_history_actual_literal_docs_never_fabricate_unused_history_or_normalizer_calls(self):
        observed=MODULE.profile_counts(self.history_row())
        charged=MODULE.account_literal_doctests(observed)
        self.assertEqual(MODULE.LITERAL_DOCTEST_LIMITS['total_source_doctests_maximum'],5)
        self.assertEqual(MODULE.LITERAL_DOCTEST_LIMITS['legal_source_doctests_maximum'],2)
        self.assertEqual(charged['history_append_requests'],3)
        self.assertEqual(charged['stats_calls'],0)
        self.assertEqual(charged['charged_fit_work_upper'],3)
        self.assertEqual(charged['point_admissions_upper'],195)
        for key in ['literal_doctest_history_append_requests_upper','literal_doctest_normalization_requests_upper',
                    'literal_doctest_stats_calls_upper','literal_doctest_observation_constructor_attempts_upper']:
            self.assertEqual(charged[key],0)
        self.assertEqual(charged['literal_doctest_points_upper'],3)
        self.assertEqual(charged['literal_doctest_projection_terms_constructor_attempts_upper'],1)
        self.assertEqual(charged['literal_doctest_projection_terms_getter_requests_upper'],2)
        self.assertEqual(charged['literal_doctest_projection_term_garde_rule_callbacks_upper'],3)
        self.assertEqual(charged['literal_doctest_projection_term_predicate_calls_upper'],6)

    def test_history_pending_and_fully_bound_synthetic_schema_have_causal_neighbors(self):
        with tempfile.TemporaryDirectory() as temp:
            root=Path(temp);policy=self.fixture(root)
            self.assertEqual(MODULE.read_policy(root,'contracts')['source_file_count'],95)
            historical = self.historical_scope().select('all')
            self.assertEqual(historical['history'],'crates/time-core/src/regression/history.rs')
            self.assertEqual(list(historical),['regression','residuals','normalization','history','runs','selection','projection','enclosure','projected_batch'])
            self.assertEqual(list(MODULE.SCOPE.select('all')), ['principal', 'regression'])
            for key in ['history_telemetry','normalization_telemetry']:
                bad=copy.deepcopy(policy);bad[key]['status']='pending-actual-source'
                self.write_policy(root,bad)
                with self.subTest(owner=key),self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root,'contracts')
            self.write_policy(root,policy)
            for name in ['crates/time-core/src/regression/history.rs','crates/time-core/tests/history.rs','crates/time-core/tests/support/history_facts.rs']:
                path=root/name;old=path.read_bytes();path.write_bytes(old+b'drift')
                with self.subTest(path=name),self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root,'contracts')
                path.write_bytes(old)

    def test_history_record_aggregation_retains_observed_and_upper_buckets(self):
        counts=MODULE.profile_counts(self.history_row())
        report={};MODULE.record_counts(report,'inert-history',counts)
        self.assertEqual(report['core_direct_and_stats_observations'],{'direct_fit_calls':0,'stats_calls':0,'point_admissions':0})
        self.assertEqual(report['core_history_external_observations']['history_append_requests'],3)
        self.assertEqual(report['core_history_source_owned_uppers']['history_statistics_entries_upper'],3)
        self.assertEqual(report['core_charged_subset_totals']['charged_fit_work_upper'],3)

    def test_history_two_actual_legal_and_eight_primary_diagnostics_have_finite_recipes(self):
        cases=MODULE.history_contract_examples();legal=[item for item in cases.values() if item['code'] is None]
        self.assertEqual(len(legal),2);self.assertEqual(len(cases)-len(legal),8)
        for field,total in [('history_constructions',2),('history_append_requests',5),
                            ('history_observation_constructor_attempts',5),('history_context_constructions',2),
                            ('history_utc_coordinate_constructor_attempts',20),('history_product_getter_requests',68)]:
            self.assertEqual(sum(item['calls'][field] for item in legal),total,field)
        self.assertEqual(sum(item['internal_upper']['history_statistics_entries_upper'] for item in legal),1)
        self.assertEqual(sum(item['internal_upper']['history_generated_point_admissions_upper'] for item in legal),3)
        for case in legal:
            self.assertEqual(case['calls']['direct_fit_calls']+case['calls']['stats_calls']+case['calls']['point_admissions'],0)
            self.assertLessEqual(case['calls']['history_product_getter_requests'],128)
        self.assertEqual(cases['history_append_borrow']['code'],'E0502')
        self.assertEqual(cases['history_report_private_construction']['code'],'E0451')
        self.assertEqual(cases['history_raw_append']['code'],'E0308')

    def test_nine_owner_variant_accounting_respects_one_selected_512_ceiling(self):
        historical = self.historical_scope().select('all')
        for amounts,accepted in [([57,57,57,57,57,57,57,57,56],True),([58,57,57,57,57,57,57,57,56],False)]:
            with tempfile.TemporaryDirectory() as temp:
                work=Path(temp)
                for subject,amount in zip(historical,amounts,strict=True):
                    out=work/'mutation'/subject/'mutants.out';out.mkdir(parents=True)
                    (out/'outcomes.json').write_text(json.dumps({'total_mutants':amount,'outcomes':[]}))
                if accepted:
                    report={'subjects': historical};MODULE.mutation_accounting(report,work)
                    self.assertEqual(report['mutation_generated_variants_selected_total'],512)
                    self.assertEqual(report['mutation_global_variant_ceiling'],512)
                else:
                    with self.assertRaisesRegex(MODULE.Incomplete, 'global512 ceiling'):
                        MODULE.mutation_accounting({'subjects': historical},work)


    def test_projection_missing_observations_and_nested_source_work_are_distinct(self):
        old = MODULE.profile_counts(self.history_row())
        self.assertIsNone(old['projection_recorded_call_subtotals'])
        self.assertIsNone(old['projection_source_owned_work_uppers'])
        counts = MODULE.profile_counts(self.complete_projection_rows(), require_projection=True)
        self.assertEqual(counts['projection_observed_calls'], {
            'histories': 12, 'appends': 93, 'observations': 93, 'contexts': 11,
            'monotonic': 121, 'coordinates': 372, 'exchanges': 93, 'getters': 3384,
            'terms': 68, 'projections': 48})
        self.assertEqual(counts['direct_fit_calls'], 0)
        self.assertEqual(counts['stats_calls'], 0)
        self.assertEqual(counts['charged_fit_work_upper'], 93)
        self.assertEqual(counts['point_admissions_upper'], 5952)
        self.assertEqual(counts['projection_source_owned_work_uppers']['projection_term_predicate_calls_upper'], 408)

    def test_complete_package_cannot_omit_projection_preeffect_charges(self):
        healthy = {'charged_fit_work_upper': 5990, 'point_admissions_upper': 269245,
                   **MODULE.selection_reservation(package=True), 'observation_pending': True}
        MODULE.record_counts({}, 'synthetic-package', healthy)
        for field in ('projection_reserved_calls', 'projection_reserved_source_work_uppers',
                      'projection_reserved_private_source_work', 'enclosure_reserved_calls',
                      'enclosure_reserved_source_work_uppers', 'enclosure_reserved_private_source_work',
                      'charged_fit_work_upper'):
            bad = copy.deepcopy(healthy)
            if field == 'charged_fit_work_upper': bad[field] -= 1
            else: del bad[field]
            with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                MODULE.record_counts({}, 'synthetic-package', bad)

    def test_all_mutation_union_above_512_refuses_before_session_effects(self):
        policy = {'mutation_inventories': {name: {'count': 300, 'sha256': 'a'*64}
                                           for name in MODULE.SUBJECTS}}
        args = SimpleNamespace(lane='mutation', subject='all', cohort_plan=Path('/inert-plan'),
                               cohort_id='all', output_dir=Path('/inert-output'))
        with patch.object(MODULE, 'read_policy', return_value=policy), \
             patch.object(MODULE.COMMON, 'Session') as session:
            with self.assertRaisesRegex(MODULE.Incomplete, 'exceeds512'):
                MODULE.execute(args, {})
            session.assert_not_called()


class InventoryAdmissionTests(unittest.TestCase):
    """Synthetic discovery admission/receiving controls; no tool or Rust runs."""

    NOW = 1700000100
    SUBJECTS = {
        'principal': 'crates/time-core/src/principal.rs',
        'regression': 'crates/time-core/src/regression.rs',
    }

    def fixture(self, root):
        """Create inert bytes and literal admission limits, not an admitted graph."""
        names = set(MODULE.BOUND_INPUTS) | {
            'crates/time-core/src/selection.rs', 'crates/time-core/tests/selection.rs',
            'crates/time-core/tests/private/selection_guards.rs'}
        tools = {
            'scripts/verify-core-analysis.py', 'scripts/portable-analysis.py',
            'scripts/verify-time-domain-analysis.py', 'scripts/analysis-runtime.py',
            'scripts/core-selection-accounting.py', 'scripts/core-selection-contracts.py',
            'scripts/core-projection-accounting.py', 'scripts/core-analysis-cohorts.py',
            'scripts/core-enclosure-accounting.py', 'scripts/core-projected-batch-accounting.py',
            'scripts/check-analysis.py', 'scripts/analysis-controls.py',
            'scripts/prepare-product.py', 'scripts/verify.py',
            'scripts/check-environment.py', 'scripts/check-strict.py',
            'policy/analysis-tools.json'}
        for name in names | tools:
            path = root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text('inert input ' + name + '\n')
        product = {name: MODULE.PRODUCT.sha256(root / name) for name in names}
        policy = {
            'schema_version': 1, 'admission_status': 'admitted-two-owner-inventory-only',
            'package': 'time-core', 'source_file_count': 95, 'subjects': dict(self.SUBJECTS),
            'maximum_mutants': 512, 'listing_maximum_per_file': 512,
            'listing_maximum_total': 3072, 'inventory_wall_seconds': 600,
            'mutation_jobs': 10, 'mutation_process_limit': 512, 'listing_process_limit': 128,
            'mutation_message_format': 'json', 'mutation_equivalences': [],
            'native_test_invocations': 0, 'campaigns_admitted': False,
            'root': str(root), 'created_unix_seconds': 1700000000,
            'expires_unix_seconds': 1700003600, 'product_inputs_sha256': product,
            'tool_inputs_sha256': {name: MODULE.PRODUCT.sha256(root / name) for name in tools},
        }
        path = root / MODULE.INVENTORY_ADMISSION
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(policy, indent=2) + '\n')
        return policy, path

    def read(self, root, path, policy):
        with patch.object(MODULE.PRODUCT, 'admit_product', return_value=({}, policy['product_inputs_sha256'])), \
             patch.object(MODULE.time, 'time', return_value=self.NOW):
            return MODULE.read_inventory_admission(root, path)

    def test_exact_current_source_and_tools_admit_discovery_without_campaign_permission(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, path = self.fixture(root)
            self.assertEqual(self.read(root, path, policy), policy)
            self.assertEqual(MODULE.INVENTORY_SUBJECTS, self.SUBJECTS)
            self.assertEqual(len(policy['product_inputs_sha256']), 95)
            self.assertEqual(len(MODULE.SUBJECTS), 2)
            self.assertIn('principal', MODULE.SUBJECTS)
            self.assertFalse(policy['campaigns_admitted'])
            self.assertEqual(MODULE.SOURCE_FILE_COUNT, 95)

    def test_unknown_fields_duplicate_fields_and_nonfinite_json_reject(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, path = self.fixture(root)
            original = path.read_text()
            invalid = [json.dumps({**policy, 'unknown': 1}),
                       original.replace('"schema_version": 1', '"schema_version": 1, "schema_version": 1', 1),
                       original.replace('"schema_version": 1', '"schema_version": NaN', 1),
                       '[]', 'null']
            for text in invalid:
                path.write_text(text)
                with self.subTest(text=text[:60]), self.assertRaises(MODULE.Incomplete):
                    self.read(root, path, policy)
            path.write_text(original)
            self.assertEqual(self.read(root, path, policy), policy)

    def test_fixed_limits_scope_status_and_timestamp_types_cannot_widen(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); healthy, path = self.fixture(root)
            changes = [('listing_maximum_per_file', True), ('listing_maximum_per_file', 513),
                       ('listing_maximum_total', 0), ('listing_maximum_total', 3073),
                       ('inventory_wall_seconds', 601), ('mutation_jobs', 8),
                       ('mutation_process_limit', 128), ('listing_process_limit', 512),
                       ('native_test_invocations', True), ('campaigns_admitted', 0),
                       ('source_file_count', 80), ('mutation_equivalences', [{}]),
                       ('admission_status', 'admitted-final-source'), ('subjects', {}),
                       ('root', str(root / 'different')), ('created_unix_seconds', True),
                       ('created_unix_seconds', self.NOW + 1), ('expires_unix_seconds', self.NOW),
                       ('expires_unix_seconds', 1700003601)]
            for name, value in changes:
                changed = copy.deepcopy(healthy); changed[name] = value
                path.write_text(json.dumps(changed))
                with self.subTest(name=name, value=value), self.assertRaises(MODULE.Incomplete):
                    self.read(root, path, healthy)

    def test_product_tool_drift_missing_hash_extra_closure_and_redirect_reject(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); healthy, path = self.fixture(root)
            for field, name in [('product_inputs_sha256', 'crates/time-core/src/selection.rs'),
                                ('tool_inputs_sha256', 'scripts/portable-analysis.py')]:
                for value in (None, '0' * 63, 'A' * 64, '0' * 64):
                    changed = copy.deepcopy(healthy); changed[field][name] = value
                    path.write_text(json.dumps(changed))
                    with self.subTest(field=field, value=value), self.assertRaises(MODULE.Incomplete):
                        self.read(root, path, healthy)
                changed = copy.deepcopy(healthy); del changed[field][name]
                path.write_text(json.dumps(changed))
                with self.assertRaises(MODULE.Incomplete): self.read(root, path, healthy)
            path.write_text(json.dumps(healthy))
            source = root / 'crates/time-core/src/selection.rs'
            original = source.read_bytes(); source.write_bytes(original + b'changed')
            with self.assertRaises(MODULE.Incomplete): self.read(root, path, healthy)
            source.write_bytes(original)
            extra = root / 'scripts/new-tool.py'; extra.write_text('inert extra tooling')
            with self.assertRaises(MODULE.Incomplete): self.read(root, path, healthy)
            extra.unlink()
            redirect = root / 'tasks/redirect.json'; redirect.write_text(path.read_text())
            with self.assertRaises(MODULE.Incomplete): self.read(root, redirect, healthy)
            path.unlink(); path.symlink_to(redirect)
            with self.assertRaises(MODULE.Incomplete): self.read(root, path, healthy)

    def test_inventory_execution_only_lists_both_checkpoint_owners_and_reports_campaign_overflow(self):
        for per_file, total, exceeded in [(1, 2, False), (300, 600, True)]:
            with self.subTest(per_file=per_file), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary); policy, path = self.fixture(root)
                session = Mock(tools=Path('/inert-sealed-tools'), source=root,
                               product_identity=dict(policy['product_inputs_sha256']),
                               identity=dict(policy['tool_inputs_sha256']))
                def listing(label, command, timeout):
                    name = label.removeprefix('mutation-enumeration:')
                    self.assertEqual(command[-2:], ['--list', '--json'])
                    self.assertNotIn('--baseline', command)
                    self.assertLessEqual(timeout, 120)
                    items = [{'package': 'time-core', 'file': self.SUBJECTS[name],
                              'name': name + '-' + str(index)} for index in range(per_file)]
                    return MODULE.COMMON.mutation_listing_warning(10) + json.dumps(items)
                session.run.side_effect = listing
                args = SimpleNamespace(lane='inventory', subject='all', inventory_admission=path,
                                       vendor_dir=root / 'unused-vendor', output_dir=root.parent / 'inert-output')
                report = {}
                with patch.object(MODULE, 'ROOT', root), \
                     patch.object(MODULE.PRODUCT, 'admit_product', return_value=({}, policy['product_inputs_sha256'])), \
                     patch.object(MODULE.time, 'time', return_value=self.NOW), \
                     patch.object(MODULE.COMMON, 'Session', return_value=session) as factory, \
                     patch.object(MODULE, 'read_policy', side_effect=AssertionError('ordinary policy used')), \
                     patch.object(MODULE.COMMON, 'run_analysis_lanes', side_effect=AssertionError('campaign used')), \
                     patch.object(MODULE, 'contracts', side_effect=AssertionError('contracts used')), \
                     patch.object(MODULE, 'fuzz', side_effect=AssertionError('fuzz used')):
                    MODULE.execute(args, report)
                self.assertEqual(factory.call_args.args[1].lane, 'mutation')
                self.assertEqual(factory.call_args.args[3].policy, 'tasks/s1.3-principal-inventory-admission.json')
                self.assertEqual(session.run.call_count, 2)
                self.assertTrue(all(not call.kwargs for call in session.run.call_args_list))
                session.finish.assert_called_once()
                self.assertEqual(report['inventory_discovery']['total'], total)
                self.assertEqual(report['inventory_discovery']['exceeds_current_campaign_cap'], exceeded)
                self.assertEqual(set(report['inventory_discovery']['inventories']), set(self.SUBJECTS))
                self.assertIs(report['campaigns_admitted'], False)
                self.assertEqual(report['native_test_invocations'], 0)
                self.assertNotIn('mutation', report)

    def test_listing_receiving_guard_refuses_campaign_wrong_owner_and_profile_options(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, path = self.fixture(root)
            for change in ('campaign', 'wrong-label', 'wrong-file', 'profile'):
                session = Mock(tools=Path('/inert-sealed-tools'), source=root,
                               product_identity=dict(policy['product_inputs_sha256']),
                               identity=dict(policy['tool_inputs_sha256']))
                args = SimpleNamespace(lane='inventory', subject='all', inventory_admission=path)
                def attempt(scope, subjects, admitted, tools, run, report, **_kwargs):
                    name = 'principal'; label = 'mutation-enumeration:' + name
                    command = [str(tools / 'cargo-mutants'), 'mutants', '--package', 'time-core',
                               '--file', self.SUBJECTS[name], '--test-workspace', 'false',
                               '--cap-lints', 'false', '--jobs', '10', '--jobserver', 'true',
                               '--jobserver-tasks', '10', '--cargo-arg=--message-format=json', '--list', '--json']
                    options = {}
                    if change == 'campaign': command[-2:] = ['--baseline', 'run']
                    if change == 'wrong-label': label = 'mutation-campaign:principal'
                    if change == 'wrong-file': command[5] = 'crates/other.rs'
                    if change == 'profile': options['mutation_subject'] = 'principal'
                    run(label, command, 5, **options)
                with patch.object(MODULE, 'ROOT', root), \
                     patch.object(MODULE, 'read_inventory_admission', return_value=policy), \
                     patch.object(MODULE.time, 'time', return_value=self.NOW), \
                     patch.object(MODULE.COMMON, 'Session', return_value=session), \
                     patch.object(MODULE.COMMON, 'enumerate_mutation_inventories', side_effect=attempt):
                    with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                        MODULE.execute(args, {})
                session.run.assert_not_called()
                session.finish.assert_called_once()

    def test_reader_to_session_handoff_rejects_changed_document_product_and_tool_maps(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary); policy, path = self.fixture(root)
            original = path.read_text()
            for change in ('document', 'duplicate-document', 'product', 'tool', 'expiry'):
                path.write_text(original)
                session = Mock(tools=Path('/inert-sealed-tools'), source=root,
                               product_identity=dict(policy['product_inputs_sha256']),
                               identity=dict(policy['tool_inputs_sha256']))
                args = SimpleNamespace(lane='inventory', subject='all', inventory_admission=path)
                if change == 'document':
                    changed = copy.deepcopy(policy); changed['created_unix_seconds'] += 1
                    path.write_text(json.dumps(changed))
                if change == 'duplicate-document':
                    path.write_text(original.replace('"schema_version": 1',
                        '"schema_version": 1, "schema_version": 1', 1))
                if change == 'product': session.product_identity['Cargo.lock'] = '0' * 64
                if change == 'tool': session.identity['scripts/portable-analysis.py'] = '0' * 64
                now = policy['expires_unix_seconds'] if change == 'expiry' else self.NOW
                with patch.object(MODULE, 'ROOT', root), \
                     patch.object(MODULE, 'read_inventory_admission', return_value=policy), \
                     patch.object(MODULE.time, 'time', return_value=now), \
                     patch.object(MODULE.COMMON, 'Session', return_value=session), \
                     patch.object(MODULE.COMMON, 'enumerate_mutation_inventories') as listing:
                    with self.subTest(change=change), self.assertRaises(MODULE.Incomplete):
                        MODULE.execute(args, {})
                listing.assert_not_called(); session.run.assert_not_called()
                session.finish.assert_called_once()

    def test_invalid_admission_and_old_lane_fail_before_inventory_or_session(self):
        session = Mock()
        with patch.object(MODULE, 'read_inventory_admission', side_effect=MODULE.Incomplete('bad inventory admission')), \
             patch.object(MODULE.COMMON, 'Session', session), \
             patch.object(MODULE.COMMON, 'enumerate_mutation_inventories') as listings:
            with self.assertRaisesRegex(MODULE.Incomplete, 'bad inventory admission'):
                MODULE.execute(SimpleNamespace(lane='inventory', subject='all', inventory_admission=Path('bad')), {})
        session.assert_not_called(); listings.assert_not_called()
        with patch.object(MODULE, 'read_policy', side_effect=MODULE.Incomplete('old five-owner gate')), \
             patch.object(MODULE, 'read_inventory_admission') as admission:
            with self.assertRaisesRegex(MODULE.Incomplete, 'old five-owner gate'):
                MODULE.execute(SimpleNamespace(lane='mutation', subject='all'), {})
        admission.assert_not_called()

    def test_cli_requires_explicit_inventory_admission_and_complete_subjects(self):
        with tempfile.TemporaryDirectory() as temporary:
            base = ['--output-dir', temporary, '--vendor-dir', temporary]
            for options in (['--lane', 'inventory'],
                            ['--lane', 'inventory', '--inventory-admission', 'tasks/s1.3-inventory-admission.json', '--subject', 'runs'],
                            ['--lane', 'coverage', '--inventory-admission', 'tasks/s1.3-inventory-admission.json']):
                with self.subTest(options=options), contextlib.redirect_stderr(io.StringIO()), \
                     self.assertRaises(SystemExit) as error:
                    MODULE.main(base + options)
                self.assertEqual(error.exception.code, 2)


class CoreDocumentReaderTests(unittest.TestCase):
    """Transport controls only; adjacent candidate/union admission stays separate."""

    @staticmethod
    def padded_json(path, total_bytes):
        """Write an independently sized complete ASCII JSON control."""
        prefix, suffix = b'{"padding":"', b'"}'
        if type(total_bytes) is not int or total_bytes < len(prefix) + len(suffix):
            raise ValueError("Invalid finite document fixture size.")
        path.write_bytes(prefix + b"x" * (total_bytes - len(prefix) - len(suffix)) + suffix)

    def test_execute_union_uses_native_receipt_limit(self):
        """Behavioral RED on the existing entrypoint before the reader repair.

        One large raw object isolates transport from the separately controlled
        complete seven-owner union. The unchanged receiving entrypoint must pass
        its whole decoded object to that adjacent owner without any product run.
        """
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            receipt = base / "verification.json"
            self.padded_json(receipt, 600 * 1024)
            args = SimpleNamespace(cohort_plan=base / "unused-plan.json",
                                   cohort_receipt=[receipt], output_dir=base / "union")
            report = {}
            expected_union = {"status": "synthetic-transport-only"}
            with patch.object(MODULE, "read_policy", return_value={}), \
                 patch.object(MODULE, "read_cohort_plan", return_value=({}, {}, args.cohort_plan)), \
                 patch.object(MODULE.COHORTS, "reconcile_receipts", return_value=expected_union) as union, \
                 patch.object(MODULE.COMMON, "Session") as session, \
                 patch.object(MODULE.COMMON, "run_analysis_lanes") as lanes:
                MODULE.execute_cohort_union(args, report)
            union.assert_called_once()
            self.assertEqual(len(union.call_args.args[1]), 1)
            self.assertEqual(len(union.call_args.args[1][0]["padding"]), 600 * 1024 - 14)
            self.assertEqual(report["core_cohort_union"], expected_union)
            self.assertEqual(report["owned_output"], str(args.output_dir))
            session.assert_not_called()
            lanes.assert_not_called()
            with self.assertRaisesRegex(MODULE.Incomplete, "fixed byte allowance"):
                MODULE.inventory_document(receipt)

    def test_inventory_exact_512_kib_and_native_positive_neighbor(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "document.json"
            self.padded_json(path, 512 * 1024)
            self.assertEqual(len(MODULE.inventory_document(path)["padding"]), 512 * 1024 - 14)
            self.padded_json(path, 512 * 1024 + 1)
            self.assertEqual(len(MODULE.native_receipt_document(path)["padding"]), 512 * 1024 - 13)
            with self.assertRaisesRegex(MODULE.Incomplete, "fixed byte allowance"):
                MODULE.inventory_document(path)

    def test_native_exact_eight_mib_neighbors_and_overflow(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "verification.json"
            for length in (8 * 1024 * 1024 - 1, 8 * 1024 * 1024):
                with self.subTest(length=length):
                    self.padded_json(path, length)
                    self.assertEqual(len(MODULE.native_receipt_document(path)["padding"]), length - 14)
            self.padded_json(path, 8 * 1024 * 1024 + 1)
            with self.assertRaisesRegex(MODULE.Incomplete, "fixed byte allowance"):
                MODULE.native_receipt_document(path)

    def test_native_bound_counts_utf8_bytes_and_reads_the_complete_document(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "verification.json"
            prefix, suffix = b'{"padding":"', b'"}'
            payload = prefix + "\u00e9".encode("utf-8") * ((8 * 1024 * 1024 - 14) // 2) + suffix
            self.assertEqual(len(payload), 8 * 1024 * 1024)
            path.write_bytes(payload)
            self.assertEqual(len(MODULE.native_receipt_document(path)["padding"]), (8 * 1024 * 1024 - 14) // 2)
            path.write_bytes(payload + b" ")
            with self.assertRaisesRegex(MODULE.Incomplete, "fixed byte allowance"):
                MODULE.native_receipt_document(path)

    def test_both_roles_reject_duplicate_fields_and_nonfinite_constants(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "document.json"
            for reader in (MODULE.inventory_document, MODULE.native_receipt_document):
                path.write_bytes(b'{"number":0.5,"nested":{"value":1}}')
                self.assertEqual(reader(path)["number"], 0.5)
                for raw in (b'{"x":0,"x":1}', b'{"outer":{"x":0,"x":1}}',
                            b'{"x":NaN}', b'{"x":Infinity}', b'{"x":-Infinity}'):
                    path.write_bytes(raw)
                    with self.subTest(reader=reader.__name__, raw=raw), self.assertRaises(MODULE.Incomplete):
                        reader(path)

    def test_both_roles_reject_truncated_trailing_and_invalid_utf8_bytes(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / "document.json"
            for reader in (MODULE.inventory_document, MODULE.native_receipt_document):
                for raw in (b'{"x":', b'{}{}', b'{"x":0}ignored', b'{"x":"\xff"}'):
                    path.write_bytes(raw)
                    with self.subTest(reader=reader.__name__, raw=raw), self.assertRaises((MODULE.Incomplete, ValueError)):
                        reader(path)

    def test_both_roles_preserve_canonical_regular_path_checks(self):
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            path = base / "document.json"
            path.write_bytes(b'{"value":1}')
            leaf_link = base / "linked.json"
            leaf_link.symlink_to(path)
            directory_link = base / "linked-directory"
            directory_link.symlink_to(base, target_is_directory=True)
            child = base / "child"
            child.mkdir()
            for reader in (MODULE.inventory_document, MODULE.native_receipt_document):
                self.assertEqual(reader(path), {"value": 1})
                for rejected in (base, leaf_link, directory_link / path.name, child / ".." / path.name):
                    with self.subTest(reader=reader.__name__, rejected=str(rejected)), self.assertRaises(MODULE.Incomplete):
                        reader(rejected)

    def test_unknown_roles_fail_before_any_path_operation(self):
        for role in (None, True, 0, [], {}, "native", "receipt", "inventory ", "native-receipt "):
            path = Mock()
            with self.subTest(role=role), self.assertRaises(MODULE.Incomplete):
                MODULE._core_json_document(path, role=role)
            path.resolve.assert_not_called()
            path.open.assert_not_called()


class ProjectedBatchWrapperTests(unittest.TestCase):
    """Independent literal new-family controls; no Rust or native outcomes."""

    FIELDS = ('histories', 'appends', 'observations', 'contexts', 'monotonic',
              'coordinates', 'exchanges', 'getters', 'terms', 'projections',
              'enclosures', 'batches', 'minima', 'selections')
    RECIPES = (
        ('single_compatible', (1,3,3,1,6,12,3,24,1,1,1,1,1,1)),
        ('linear_selection', (3,9,9,3,16,36,9,414,3,3,3,2,2,3)),
        ('capacity_neighbors', (17,51,51,17,86,204,51,2171,17,17,17,5,1,3)),
        ('context_admission', (8,24,24,8,42,96,24,662,8,8,8,12,1,1)),
        ('duplicate_precedence', (3,9,9,3,16,36,9,297,3,3,3,3,1,1)),
        ('permutation_retention', (3,9,9,3,16,36,9,603,3,3,3,3,1,6)),
        ('subtick_overlap', (2,6,6,2,11,24,6,225,2,2,2,1,1,1)),
        ('projected_batch_private_admission', (1,3,3,1,5,12,3,6,1,1,1,3,0,0)),
        ('projected_batch_private_priority', (3,9,9,3,14,36,9,18,3,3,3,0,0,0)),
        ('projected_batch_private_delegation', (2,6,6,2,9,24,6,14,2,2,2,2,0,0)),
        ('projected_batch_private_errors', (0,) * 14),
    )

    @classmethod
    def row(cls, case, values):
        return 'CORE_PROJECTED_BATCH_PROFILE case=' + case + ''.join(
            ' ' + field + '=' + str(value) for field, value in zip(cls.FIELDS, values, strict=True)) + '\n'

    @classmethod
    def complete_rows(cls):
        public = ''.join(cls.row(case, values) for case, values in cls.RECIPES)
        guards = ''.join('CORE_PROJECTED_BATCH_GUARD_PROFILE case=projected_batch_private_' + case
                         + ' garde_callbacks=' + str(callbacks) + ' entry_read_attempts=' + str(reads) + '\n'
                         for case, callbacks, reads in (
                             ('admission', 3, 5), ('priority', 9, 20), ('delegation', 2, 12), ('errors', 0, 0)))
        return public + guards

    def test_disjoint_attempts_reuse_each_producer_and_selector_work_owner_once(self):
        counts = MODULE.profile_counts(self.complete_rows(), require_projected_batch=True)
        self.assertEqual(counts['projected_batch_observed_calls'], dict(zip(
            self.FIELDS, (43,129,129,43,221,516,129,4434,43,43,43,32,8,16), strict=True)))
        self.assertEqual((counts['direct_fit_calls'], counts['stats_calls']), (0, 0))
        self.assertEqual((counts['charged_fit_work_upper'], counts['point_admissions_upper']), (129, 8256))
        self.assertIsNone(counts['projection_observed_calls'])
        self.assertIsNone(counts['enclosure_observed_calls'])
        work = counts['projected_batch_source_owned_work_uppers']
        self.assertEqual(len(work), 49)
        self.assertEqual(work['enclosure_interval_constructor_attempts_upper'], 43)
        self.assertEqual(work['selection_batch_copy_adapter_attempts_upper'], 512)
        self.assertEqual(work['selection_membership_probes_upper'], 16128)
        self.assertEqual(work['minimum_garde_validate_calls_upper'], 8)
        self.assertEqual(work['scratch_Copy_assignments_upper'], 992)
        self.assertEqual(work['wrapper_debug_format_calls_upper'], 4434)

    def test_observers_are_actual_overlapping_observations_and_add_no_charges(self):
        public = ''.join(self.row(case, values) for case, values in self.RECIPES)
        without = MODULE.profile_counts(public)
        with_guards = MODULE.profile_counts(self.complete_rows(), require_projected_batch=True)
        self.assertEqual(with_guards['projected_batch_guard_observed_calls'],
                         {'garde_callbacks': 14, 'entry_read_attempts': 37})
        for field in ('charged_fit_work_upper', 'point_admissions_upper', 'projected_batch_source_owned_work_uppers'):
            self.assertEqual(without[field], with_guards[field])
        self.assertIsNone(without['projected_batch_guard_observed_calls'])
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(public, require_projected_batch=True)

    def test_missing_and_partial_attempts_keep_full_preeffect_reservations(self):
        missing = MODULE.profile_counts('CORE_PROFILE_COUNTS fits=1 points=3\n')
        self.assertIsNone(missing['projected_batch_observed_calls'])
        self.assertIsNone(missing['projected_batch_source_owned_work_uppers'])
        reservation = MODULE.selection_reservation(package=True)
        self.assertEqual(reservation['projected_batch_reserved_calls']['appends'], 129)
        self.assertEqual(reservation['projected_batch_reserved_private_source_work']['direct_raw_authored_helper_entries_upper'], 91)
        report = {}
        MODULE.record_counts(report, 'synthetic-package', {
            **MODULE.package_charged_upper(), **reservation, 'observation_pending': True})
        self.assertIsNone(report['core_projected_batch_recorded_call_subtotals'])
        prefix = MODULE.profile_counts(self.row('single_compatible', (1,) + (0,) * 13))
        row = report['core_invocations'][0]
        row.update(observed_counts=prefix, observation_pending=False)
        MODULE.refresh_accounting(report)
        self.assertEqual(report['core_projected_batch_recorded_call_subtotals']['histories'], 1)
        self.assertIsNone(report['core_projected_batch_complete_observed_call_subtotals'])
        self.assertEqual(report['core_projected_batch_pending_observation_invocations'], ['synthetic-package'])
        self.assertEqual(report['core_projected_batch_reserved_totals']['projected_batch.call.appends'], 129)
        self.assertEqual(report['core_charged_subset_totals'],
                         {'charged_fit_work_upper': 5990, 'point_admissions_upper': 269245})

    def test_package_reservation_rejects_each_missing_or_forged_new_work_owner(self):
        good = {**MODULE.package_charged_upper(), **MODULE.selection_reservation(package=True),
                'observation_pending': True}
        for field in ('projected_batch_reserved_calls', 'projected_batch_reserved_module_work_uppers',
                      'projected_batch_reserved_source_work_uppers', 'projected_batch_reserved_private_source_work'):
            for change in ('missing', 'zero'):
                bad = copy.deepcopy(good)
                if change == 'missing':
                    del bad[field]
                else:
                    bad[field] = {key: 0 for key in bad[field]}
                with self.subTest(field=field, change=change), self.assertRaises(MODULE.Incomplete):
                    MODULE.record_counts({}, 'forged', bad)
        bad = copy.deepcopy(good)
        del bad['selection_expected']
        with self.assertRaises(MODULE.Incomplete):
            MODULE.record_counts({}, 'unowned', bad)

    def test_guard_only_and_mixed_standalone_selection_streams_fail_closed(self):
        guard = 'CORE_PROJECTED_BATCH_GUARD_PROFILE case=projected_batch_private_errors garde_callbacks=0 entry_read_attempts=0\n'
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(guard)
        external = 'CORE_SELECTION_EXTERNAL_PROFILE intervals=1 minima=0 batches=0 selections=0\n'
        with self.assertRaises(MODULE.Incomplete):
            MODULE.profile_counts(external + self.complete_rows())

    def test_new_external_registry_is_compile_only_and_explicitly_paired(self):
        cases = MODULE.projected_batch_contract_examples()
        self.assertEqual(len(cases), 14)
        self.assertTrue(all(item['code'] is not None and 'calls' not in item for item in cases.values()))
        for name, item in cases.items():
            with self.subTest(name=name):
                self.assertEqual(item['external_legal_neighbor'], {
                    'file': 'crates/time-core/tests/projected_batch.rs',
                    'test': 'genuine_compatible_projection_reaches_real_batch_selection'})
                self.assertGreater(item['line'], 3)
                self.assertNotIn('unsafe', item['source'])
        self.assertEqual(cases['projected_batch_escaped_borrow']['code'], 'E0597')
        self.assertIn('batch.entries()', cases['projected_batch_escaped_borrow']['source'])

    def test_actual_elided_mutability_diagnostic_matches_the_intended_entries_probe(self):
        # Actual contracts-v1 receipt aa79e6fb...7734b35, final native lane134.
        # Preserve every field consumed by compiler_cause, independently of the
        # registry tokens. Rustc elides the identical underlying type as `_`.
        source = '/work/contract-inputs/projected_batch_mutable_entries.rs'
        diagnostic = {
            '$message_type': 'diagnostic', 'message': 'mismatched types',
            'code': {'code': 'E0308', 'explanation': None}, 'level': 'error',
            'spans': [
                {'file_name': source, 'line_start': 5, 'line_end': 5,
                 'column_start': 41, 'column_end': 56, 'is_primary': True,
                 'label': 'types differ in mutability',
                 'text': [{'text': ' let _: &mut [FixtureProjectedInterval]=batch.entries();',
                           'highlight_start': 41, 'highlight_end': 56}]},
                {'file_name': source, 'line_start': 5, 'line_end': 5,
                 'column_start': 9, 'column_end': 40, 'is_primary': False,
                 'label': 'expected due to this'},
            ],
            'children': [{'message': 'expected mutable reference `&mut _`\n           found reference `&_`',
                          'code': None, 'level': 'note', 'spans': [], 'children': [], 'rendered': None}],
        }
        specimen = MODULE.projected_batch_contract_examples()['projected_batch_mutable_entries']
        self.assertEqual(specimen['code'], 'E0308')
        self.assertEqual(specimen['line'], 5)
        self.assertEqual(specimen['source'].splitlines()[4],
                         ' let _: &mut [FixtureProjectedInterval]=batch.entries();')
        wrong_type = copy.deepcopy(diagnostic)
        wrong_type['spans'][0]['label'] = 'expected bool, found u64'
        wrong_type['children'][0]['message'] = 'expected `bool`, found `u64`'
        wrong_span = copy.deepcopy(diagnostic)
        wrong_span['spans'][0]['line_start'] = 6
        for rejected in (wrong_type, wrong_span):
            with self.subTest(neighbor=rejected), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.compiler_cause(
                    {'exit_code': 1, 'failure': None, 'output': json.dumps(rejected)}, specimen, source)
        try:
            MODULE.COMMON.compiler_cause(
                {'exit_code': 1, 'failure': None, 'output': json.dumps(diagnostic)}, specimen, source)
        except MODULE.Incomplete as error:
            self.fail('Actual intended mutability rejection was not admitted: ' + str(error))

    def test_nonmutation_prospective_source_vector_is_checked_before_effects(self):
        required = MODULE.projected_batch_budget_vector()
        subjects = dict(MODULE.SUBJECTS)
        with patch.dict(MODULE.FINAL_PACKAGE_TEST_COUNTS, {'baseline': 1}):
            with patch.dict(MODULE.PROJECTED_BATCH_TELEMETRY, {'global_limits': required}):
                coverage = MODULE.prospective_scope_reservation('coverage', subjects)
                self.assertEqual(coverage['projected_batch_source_work_upper'], required)
                reserved = MODULE.prospective_scope_reservation('contracts', subjects)
                self.assertEqual(reserved['projected_batch_source_work_upper'],
                                 {key: 0 for key in required})
                for lane in ('all', 'fuzz'):
                    with self.subTest(lane=lane), self.assertRaisesRegex(MODULE.Incomplete, 'exact admitted lanes/owners'):
                        MODULE.prospective_scope_reservation(lane, subjects)
                # Nominal one-package admission cannot fund both named cohorts.
                with self.assertRaisesRegex(MODULE.Incomplete, 'finite reservation'):
                    MODULE.prospective_scope_reservation('mutation', subjects)
            for limits in (None, {**required, 'projected_batch.call.batches': required['projected_batch.call.batches'] - 1}):
                with self.subTest(limits=limits), patch.dict(
                        MODULE.PROJECTED_BATCH_TELEMETRY, {'global_limits': limits}), self.assertRaises(MODULE.Incomplete):
                    MODULE.prospective_scope_reservation('coverage', subjects)


class CheckedInCorePolicyTests(unittest.TestCase):
    """Exercise the real checked-in source admission outside synthetic fixtures."""

    def test_checked_in_checkpoint_policy_admits_the_current_complete_product(self):
        # No adjacent reader, identity map, FIXED metadata or environment is
        # patched. Admission reads the actual manifests, sources, guards,
        # dependency graph, fuzz inputs and current candidate bindings.
        policy = MODULE.read_policy(ROOT, 'coverage')
        self.assertEqual(policy['source_file_count'], 95)
        self.assertEqual(len(policy['inputs_sha256']), 95)
        self.assertEqual(policy['final_package_test_counts'],
                         {'ordinary': 143, 'doctests': 5, 'baseline': 148})
        self.assertEqual(policy['selection_telemetry']['package_tests'],
                         {'ordinary': 143, 'doctests': 5, 'baseline': 148})
        self.assertEqual(policy['mutation_equivalences'], [])
        self.assertEqual(set(policy['critical_files']),
                         {'crates/time-core/src/principal.rs', 'crates/time-core/src/regression.rs'})
        self.assertEqual(MODULE.COMMON.canonical_sha256(policy['inputs_sha256']),
                         '225da73a1d0bd8f6be12e45b9de20c3ec3b889d475ef72a0604d16f1d977750f')

    def test_nonmutation_session_binds_enclosure_helper_before_native_work(self):
        """Capture the real execution boundary before any tool can be admitted."""
        class CapturedBeforeNative(Exception):
            """Terminate at Session construction without starting a subprocess."""

        for lane in ('coverage', 'contracts'):
            with self.subTest(lane=lane):
                captured = []

                def stop_before_native(root, args, report, scope, policy, extra):
                    captured.extend(extra)
                    raise CapturedBeforeNative

                with patch.object(MODULE.COMMON, 'Session', side_effect=stop_before_native):
                    with self.assertRaises(CapturedBeforeNative):
                        MODULE.execute(SimpleNamespace(lane=lane, subject='all'), {})
                # Healthy neighboring tool owners already cross this actual
                # boundary. The enclosure observer must receive the same custody.
                self.assertIn('scripts/core-projection-accounting.py', captured)
                self.assertIn('scripts/core-analysis-cohorts.py', captured)
                self.assertIn('scripts/core-enclosure-accounting.py', captured)
                self.assertIn('scripts/core-projected-batch-accounting.py', captured)


if __name__ == '__main__':
    unittest.main()
