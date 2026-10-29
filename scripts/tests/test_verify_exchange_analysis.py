"""Independent exchange-runner admission/custody controls; no product execution."""
from __future__ import annotations

import importlib.util
import json
from pathlib import Path
import shutil
import tempfile
import unittest
from types import SimpleNamespace
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("exchange_analysis_test", ROOT / "scripts/verify-exchange-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class ExchangeAnalysisTests(unittest.TestCase):
    """Valid neighboring policies and artifacts discriminate unsafe admissions."""

    def fixture(self, root):
        for name in (MODULE.POLICY, *MODULE.BOUND_INPUTS):
            target = root / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, target)
        policy = json.loads((root / MODULE.POLICY).read_text())
        # Bind this isolated control fixture, never rewrite the live product policy.
        policy["inputs_sha256"] = {name: MODULE.PRODUCT.sha256(root / name) for name in MODULE.BOUND_INPUTS}
        (root / MODULE.POLICY).write_text(json.dumps(policy))
        return policy

    def test_policy_healthy_control_and_exact_scope(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); self.fixture(root)
            policy = MODULE.read_policy(root)
        self.assertEqual(policy["critical_files"], [
            "crates/ntp-exchange/src/context.rs", "crates/ntp-exchange/src/association.rs"])
        self.assertEqual(policy["mutation_equivalences"], [])
        self.assertNotIn("coverage_guided_fuzzing", policy)
        self.assertEqual(MODULE.SCOPE.select("all"), MODULE.SUBJECTS)
        with self.assertRaises(MODULE.Incomplete): MODULE.SCOPE.select("../time-domain")

    def test_policy_rejects_weakened_thresholds_budgets_and_unknown_fields(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy = self.fixture(root)
            for key, value in [("coverage_minimum_line_percent", 89),
                               ("mutation_minimum_viable_killed_percent", 94),
                               ("maximum_mutants", 1), ("jobs", True),
                               ("mutation_test_seconds", 0), ("scope_override", "all")]:
                bad = json.loads(json.dumps(healthy)); bad[key] = value
                (root / MODULE.POLICY).write_text(json.dumps(bad))
                with self.subTest(key=key), self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root)

    def test_policy_rejects_missing_duplicated_or_wrong_package_inventory(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy = self.fixture(root)
            path = root / "policy/verification-manifest.toml"
            original = path.read_text()
            changed = [
                original.replace('"crates/ntp-exchange/src/context.rs"', '"crates/ntp-exchange/src/missing.rs"'),
                original.replace('package = "ntp-exchange"', 'package = "wrong-owner"'),
                original.replace('    "crates/ntp-exchange/src/association.rs",',
                                 '    "crates/ntp-exchange/src/association.rs",\n    "crates/ntp-exchange/src/association.rs",'),
            ]
            for text in changed:
                path.write_text(text)
                policy = json.loads(json.dumps(healthy))
                policy["inputs_sha256"]["policy/verification-manifest.toml"] = MODULE.PRODUCT.sha256(path)
                (root / MODULE.POLICY).write_text(json.dumps(policy))
                with self.subTest(text=text[:35]), self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root)

    def test_policy_rejects_missing_extra_or_changed_bound_inputs(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp); healthy = self.fixture(root)
            for change in ["missing", "extra", "source"]:
                policy = json.loads(json.dumps(healthy))
                if change == "missing": policy["inputs_sha256"].pop("Cargo.lock")
                if change == "extra": policy["inputs_sha256"]["unreviewed"] = "0" * 64
                if change == "source":
                    path = root / "crates/ntp-exchange/src/context.rs"
                    path.write_text(path.read_text() + "\n// changed\n")
                (root / MODULE.POLICY).write_text(json.dumps(policy))
                with self.subTest(change=change), self.assertRaises(MODULE.Incomplete): MODULE.read_policy(root)

    def test_policy_cannot_skip_common_exact_equivalence_admission(self):
        with patch.object(MODULE.COMMON, "admit_mutation_equivalences",
                          side_effect=MODULE.Incomplete("exact proof rejected")) as admission:
            with self.assertRaisesRegex(MODULE.Incomplete, "exact proof rejected"): MODULE.read_policy(ROOT)
            self.assertEqual(admission.call_args.args[:2], (ROOT, MODULE.SCOPE))

    def test_external_specimens_cover_private_roles_and_current_owner_barriers(self):
        cases = MODULE.contract_examples()
        self.assertIsNone(cases["legal"]["code"])
        for name in ["source", "socket", "request", "configuration", "association_generation", "clock",
                     "generations", "binding", "context", "token", "timing", "eras", "receipt",
                     "completion", "observation", "control"]:
            for suffix, code in [("construction", "E0451"), ("mutation", "E0616"), ("default", "E0277")]:
                case = cases[name + "_" + suffix]
                self.assertEqual(case["code"], code)
                self.assertEqual(case["line"], 5)
                self.assertTrue(case["source"].splitlines()[4].strip())
        for name in ["owner_clone", "owner_copy", "raw_token_arm", "token_raw_getter", "source_socket_mixed",
                     "generation_roles_mixed", "active_awake_timing", "mapping_generation_mixed",
                     "raw_saved_t1", "missing_era_context", "omitted_era_context", "header_receive_bypass"]:
            self.assertIn(name, cases)
            self.assertTrue(cases[name]["tokens"])
        self.assertIn('legal-exchange-contracts:time-control-once', cases["legal"]["source"])

    def emitted_fixture(self, work):
        messages = []
        for package in MODULE.LIBRARIES:
            filename = '/work/target/debug/deps/lib' + package.replace('-', '_') + '-123.rlib'
            path = work / filename.removeprefix('/work/')
            path.parent.mkdir(parents=True, exist_ok=True); path.write_text(package)
            messages.append({"reason": "compiler-artifact",
                "package_id": f"path+file:///work/source/crates/{package}#0.0.0",
                "manifest_path": f"/work/source/crates/{package}/Cargo.toml",
                "target": {"name": package.replace('-', '_'), "kind": ["lib"]},
                "filenames": [filename]})
        return messages

    def test_dependency_copies_are_actual_emitted_sealed_files_without_aliases(self):
        with tempfile.TemporaryDirectory() as temp:
            work = Path(temp); messages = self.emitted_fixture(work)
            junk = work / 'target/debug/deps/libunattributed.rlib'; junk.write_text('unrelated')
            protected = []
            session = SimpleNamespace(work=work, protect=protected.append)
            copied, externs, actual = MODULE.copy_contract_dependencies(session, '\n'.join(map(json.dumps, messages)))
            self.assertEqual(set(externs), {'ntp_exchange', 'time_domain', 'ntp_wire'})
            self.assertEqual(set(actual), set(MODULE.LIBRARIES))
            self.assertEqual(protected, [copied])
            self.assertNotIn(junk.name, [item.name for item in copied.iterdir()])
            for package, emitted in actual.items():
                original = work / emitted.removeprefix('/work/')
                target = copied / original.name
                self.assertNotEqual(original.stat().st_ino, target.stat().st_ino)
                self.assertEqual(target.stat().st_mode & 0o777, 0o400)
                original.write_text('changed after copying')
                self.assertEqual(target.read_text(), package)

    def test_dependency_admission_rejects_missing_duplicate_redirected_and_escaped(self):
        for kind in ['missing', 'duplicate', 'redirected', 'escape']:
            with self.subTest(kind=kind), tempfile.TemporaryDirectory() as temp:
                work = Path(temp); messages = self.emitted_fixture(work)
                if kind == 'missing': messages.pop()
                if kind == 'duplicate': messages.append(dict(messages[0]))
                if kind == 'redirected':
                    path = work / messages[0]['filenames'][0].removeprefix('/work/')
                    saved = work / 'redirected.rlib'; path.rename(saved); path.symlink_to(saved)
                if kind == 'escape': messages[0]['filenames'] = ['/work/elsewhere/libntp_exchange.rlib']
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.copy_contract_dependencies(SimpleNamespace(work=work, protect=lambda _: None),
                                                       '\n'.join(map(json.dumps, messages)))

    def resource_fixture(self):
        base = {name: 0 for name in MODULE.SNAPSHOT_FIELDS}
        base.update(live_payload_bytes=10000, peak_live_payload_bytes=10000, counter_error=False)
        snapshot = lambda **changes: {**base, **changes}
        initial = snapshot()
        box = {'before': snapshot(), 'held': snapshot(alloc_requests=1, requested_bytes=4096,
                successful_requests=1, successful_requested_bytes=4096, live_payload_bytes=14096, peak_live_payload_bytes=14096),
               'contents_match': True}
        base.update(alloc_requests=1, requested_bytes=4096, successful_requests=1,
                    successful_requested_bytes=4096, deallocations=1, peak_live_payload_bytes=14096)
        box['dropped'] = snapshot()
        vec = {'before': snapshot(peak_live_payload_bytes=10000), 'contents_match': True}
        base.update(zeroed_requests=1, requested_bytes=4352, successful_requests=2,
                    successful_requested_bytes=4352, live_payload_bytes=10256, peak_live_payload_bytes=10256)
        vec['zeroed'] = snapshot()
        base.update(realloc_requests=1, requested_bytes=8448, successful_requests=3,
                    successful_requested_bytes=8448, live_payload_bytes=14096, peak_live_payload_bytes=14096)
        vec['grown'] = snapshot()
        base.update(realloc_requests=2, requested_bytes=8576, successful_requests=4,
                    successful_requested_bytes=8576, live_payload_bytes=10128)
        vec['shrunk'] = snapshot()
        base.update(deallocations=2, live_payload_bytes=10000)
        vec['dropped'] = snapshot()
        base['peak_live_payload_bytes'] = 10000
        samples = []
        for name, counts in MODULE.RESOURCE_COUNTS.items():
            samples.append({'name': name, **dict(zip(MODULE.RESOURCE_COUNT_FIELDS, counts, strict=True)),
                            'mismatches': 0, 'pending_empty': True, 'before': snapshot(), 'after': snapshot()})
        return {'schema_version': 1, 'scope': 's1.1d-host-requested-payload',
                'profile': 'single-thread-fixed-fixture-v1', 'debug_assertions': True,
                'observation_initial': initial, 'box_control': box, 'vec_control': vec,
                'samples': samples, 'observation_final': snapshot()}

    def test_resource_observation_requires_actual_nonzero_controls_and_zero_interval_allocation(self):
        healthy = self.resource_fixture()
        assessment = MODULE.validate_resource_observation(healthy)
        self.assertEqual(assessment['hostile_receive_attempts'], 4096)
        self.assertEqual(assessment['retire_rearm_cycles'], 256)
        self.assertEqual(assessment['live_requested_payload_baseline'], 10000)
        for fault in ['elided_box', 'retained_box', 'elided_vec', 'retained_product', 'temporary_product',
                      'missing_case', 'wrong_count', 'unexpected_result', 'pending', 'counter_error', 'counter_regression',
                      'unmeasured_destructor']:
            bad = json.loads(json.dumps(healthy))
            if fault == 'elided_box': bad['box_control']['held'] = dict(bad['box_control']['before'])
            if fault == 'retained_box': bad['box_control']['dropped']['live_payload_bytes'] += 1
            if fault == 'elided_vec': bad['vec_control']['grown']['realloc_requests'] = 0
            if fault == 'retained_product': bad['samples'][0]['after']['live_payload_bytes'] += 1
            if fault == 'temporary_product':
                bad['samples'][0]['after']['alloc_requests'] += 1
                bad['samples'][0]['after']['deallocations'] += 1
            if fault == 'missing_case': bad['samples'].pop()
            if fault == 'wrong_count': bad['samples'][0]['iterations'] = 4095
            if fault == 'unexpected_result': bad['samples'][0]['mismatches'] = 1
            if fault == 'pending': bad['samples'][1]['pending_empty'] = False
            if fault == 'counter_error': bad['samples'][0]['after']['counter_error'] = True
            if fault == 'counter_regression': bad['observation_final']['requested_bytes'] = 0
            if fault == 'unmeasured_destructor':
                for sample in bad['samples'][1:]:
                    for point in ['before', 'after']:
                        sample[point]['alloc_requests'] += 1
                        sample[point]['deallocations'] += 1
                bad['observation_final']['alloc_requests'] += 1
                bad['observation_final']['deallocations'] += 1
            with self.subTest(fault=fault), self.assertRaises(MODULE.Incomplete):
                MODULE.validate_resource_observation(bad)

    def test_resource_evidence_rejects_bool_counters_duplicate_keys_nonfinite_and_oversized_json(self):
        self.assertEqual(MODULE.decode_resource_output(json.dumps(self.resource_fixture())), self.resource_fixture())
        for text in ['{"x":1,"x":2}', '{"x":NaN}', ' ' * 131073]:
            with self.subTest(text=text[:35]), self.assertRaises(MODULE.Incomplete): MODULE.decode_resource_output(text)
        for location in ['schema', 'counter', 'count']:
            bad = self.resource_fixture()
            if location == 'schema': bad['schema_version'] = True
            if location == 'counter': bad['samples'][0]['before']['alloc_requests'] = True
            if location == 'count': bad['samples'][0]['mismatches'] = False
            with self.subTest(location=location), self.assertRaises(MODULE.Incomplete): MODULE.validate_resource_observation(bad)

    def test_external_lint_policy_is_carried_and_weakened_levels_rejected(self):
        good = {'workspace': {'lints': {'rust': {'warnings': {'level': 'deny', 'priority': -1}, 'unsafe_code': 'deny'},
                                       'clippy': {'all': {'level': 'deny', 'priority': -1}, 'panic': 'deny'}}}}
        self.assertEqual(MODULE.observer_lint_flags(good),
                         ['--deny', 'warnings', '--deny', 'unsafe_code', '--deny', 'clippy::all', '--deny', 'clippy::panic'])
        good['workspace']['lints']['rust']['unsafe_code'] = 'allow'
        with self.assertRaises(MODULE.Incomplete): MODULE.observer_lint_flags(good)

    def test_execute_keeps_resource_and_model_claims_separate_and_finishes_failed_lane(self):
        class Session:
            def __init__(self, *args):
                self.work = Path('/unused'); self.tools = Path('/unused'); self.run = None
            def finish(self): finished.append(True)
        finished = []
        args = SimpleNamespace(lane='resources', subject='all')
        report = {}
        with patch.object(MODULE, 'read_policy', return_value={}), patch.object(MODULE.COMMON, 'Session', Session), \
                patch.object(MODULE.COMMON, 'run_analysis_lanes') as analysis, \
                patch.object(MODULE, 'resources', side_effect=MODULE.Incomplete('observed retained payload')):
            with self.assertRaisesRegex(MODULE.Incomplete, 'retained payload'): MODULE.execute(args, report)
        self.assertEqual(finished, [True])
        self.assertEqual(analysis.call_args.args[-1], 'resources')
        self.assertIn('Separate admitted integration-test evidence', report['generated_model'])
        self.assertNotIn('passed', report.values())


if __name__ == '__main__':
    unittest.main()
