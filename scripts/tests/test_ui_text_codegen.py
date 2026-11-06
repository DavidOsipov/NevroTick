"""Hostile metadata and private-output controls for protected identifier codegen.

These execute Python only. Generated Rust/Slint are inert review artifacts; no
locale approval, Cargo, window, OS service or clock effect is involved.
"""
import copy
import hashlib
import importlib.util
import json
from pathlib import Path
import shutil
import stat
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

SCRIPTS = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(SCRIPTS))
SPEC = importlib.util.spec_from_file_location('ui_text_codegen_under_test', SCRIPTS / 'ui_text_codegen.py')
GEN = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GEN)
FIXTURES = SCRIPTS / 'tests/fixtures'


class CodeGeneration(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.work = Path(temporary.name)
        self.repo = self.work / 'repo'
        self.repo.mkdir()
        self.raw = GEN._read_regular(FIXTURES / 'ui-text-codegen-contract.json', GEN.MAX_CONTRACT_BYTES)
        self.template = GEN._read_regular(SCRIPTS / 'ui-text-templates/projection-header.rs.in', GEN.MAX_TEMPLATE_BYTES)
        self.pin = hashlib.sha256(self.raw).hexdigest()
        for relative, value in [(GEN.CONTRACT_PATH, self.raw), (GEN.TEMPLATE_PATH, self.template)]:
            target = self.repo / relative
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(value)
        self.baseline = self.repo / GEN.CONTRACT_PATH.parent / 'text-contract-v1-baseline.json'
        self.baseline.write_bytes(b'Inert baseline sentinel; never decoded or changed by codegen.')
        self.approvals = self.repo / GEN.CONTRACT_PATH.parent / 'approvals.rs'
        self.approvals.write_bytes(b'Inert approval ledger sentinel; no grant can be written here.')

    def candidate(self, change):
        data = GEN.protected_json.read(self.raw)
        change(data)
        raw = (json.dumps(data, ensure_ascii=False, indent=2) + '\n').encode('utf-8')
        return raw, hashlib.sha256(raw).hexdigest()

    def emit(self, raw=None, pin=None):
        return GEN._emit_files(self.raw if raw is None else raw, self.pin if pin is None else pin, self.template)

    def test_exact_output_matches_independent_pinned_legacy_oracle(self):
        oracle = GEN.protected_json.read(GEN._read_regular(FIXTURES / 'ui-text-codegen-output-sha256.json', 65536))
        self.assertEqual(oracle['contract_fixture_sha256'], self.pin)
        files = self.emit()
        self.assertEqual({name: hashlib.sha256(value).hexdigest() for name, value in files.items()}, oracle['files'])
        self.assertEqual(len(files), 16)
        self.assertLess(sum(map(len, files.values())), GEN.MAX_OUTPUT_BYTES)
        self.assertIn(b'texts: [string]', files['localization-copy.slint'])
        self.assertNotIn(b'UiFixtureCopy', files['localization-copy.slint'])
        slots=GEN.protected_json.read(files['output-slot-map.json'])
        self.assertEqual(slots['static_count'],394)
        self.assertEqual(slots['output_count'],423)
        self.assertEqual(len(slots['feedback']),29)

    def test_native_v2_fixture_preserves_v1_and_shallow_output_contract(self):
        raw = GEN._read_regular(FIXTURES / 'ui-text-codegen-contract-v2.json', GEN.MAX_CONTRACT_BYTES)
        pin = hashlib.sha256(raw).hexdigest()
        self.assertEqual(pin, 'b04017b539b23afbecc0c27ae9216bb301c34396bf8a4a9efcab8251fe10a13e')
        previous = GEN.protected_json.read(self.raw)
        current = GEN.protected_json.read(raw)
        self.assertEqual((current['catalog_schema_version'], current['text_contract_version'], current['gui_build_version']), (2, 2, 2))
        self.assertEqual(current['bounds'], previous['bounds'])
        self.assertEqual(len(current['messages']), 596)
        self.assertEqual(sum(row['active'] for row in current['messages']), 509)
        self.assertEqual([(row['key'], row['ordinal']) for row in current['messages'][:585]], [(row['key'], row['ordinal']) for row in previous['messages']])
        retired = next(row for row in current['messages'] if row['key'] == 'ui.shell.no-notification-submission')
        self.assertFalse(retired['active'])
        files = GEN._emit_files(raw, pin, self.template)
        oracle = GEN.protected_json.read(GEN._read_regular(FIXTURES / 'ui-text-codegen-output-v2-sha256.json', 65536))
        self.assertEqual({name: hashlib.sha256(value).hexdigest() for name, value in files.items()}, oracle)
        slots = GEN.protected_json.read(files['output-slot-map.json'])
        self.assertEqual((slots['static_count'], slots['output_count'], len(slots['feedback'])), (397, 426, 29))
        self.assertIn(b'texts: [string]', files['localization-copy.slint'])
        self.assertNotIn(b'UiFixtureCopy', files['localization-copy.slint'])
        self.assertNotIn('no-notification-submission', slots['fields']['shell'])

    def test_generate_uses_new_private_output_and_preserves_repo_baseline_and_ledger(self):
        before = {str(path.relative_to(self.repo)): hashlib.sha256(path.read_bytes()).hexdigest()
                  for path in self.repo.rglob('*') if path.is_file()}
        first = GEN.generate(self.repo, self.pin, self.work)
        second = GEN.generate(self.repo, self.pin, self.work)
        self.assertNotEqual(first, second)
        self.assertFalse(first.is_relative_to(self.repo))
        self.assertEqual(stat.S_IMODE(first.stat().st_mode), 0o700)
        self.assertEqual({path.name: path.read_bytes() for path in first.iterdir()}, self.emit())
        self.assertTrue(all(stat.S_IMODE(path.stat().st_mode) == 0o600 for path in first.iterdir()))
        self.assertEqual(before, {str(path.relative_to(self.repo)): hashlib.sha256(path.read_bytes()).hexdigest()
                                 for path in self.repo.rglob('*') if path.is_file()})

    def test_wrong_identity_rejects_before_metadata_decode_or_output(self):
        with patch.object(GEN.protected_json, 'read', side_effect=AssertionError('wrong bytes cannot be decoded')):
            with self.assertRaises(GEN.Invalid): GEN.generate(self.repo, '0' * 64, self.work)
        self.assertEqual(list(self.work.glob('nevrotick-ui-text-*')), [])

    def test_original_byte_bound_precedes_open_and_decode(self):
        (self.repo / GEN.CONTRACT_PATH).write_bytes(b' ' * (GEN.MAX_CONTRACT_BYTES + 1))
        with patch.object(GEN.os, 'open', side_effect=AssertionError('oversized file cannot be opened')):
            with self.assertRaises(GEN.Invalid): GEN.generate(self.repo, self.pin, self.work)
        self.assertEqual(list(self.work.glob('nevrotick-ui-text-*')), [])

    def test_source_and_template_symlinks_are_denied(self):
        for relative in (GEN.CONTRACT_PATH, GEN.TEMPLATE_PATH):
            with self.subTest(relative=relative):
                target = self.repo / relative
                original = target.read_bytes()
                outside = self.work / ('outside-' + target.name)
                outside.write_bytes(original)
                target.unlink()
                target.symlink_to(outside)
                with self.assertRaises(GEN.Invalid): GEN.generate(self.repo, self.pin, self.work)
                target.unlink()
                target.write_bytes(original)

    def test_template_identity_output_parent_and_aggregate_bounds_reject(self):
        with self.assertRaises(GEN.Invalid): GEN._emit_files(self.raw, self.pin, self.template + b'// substituted')
        with self.assertRaises(GEN.Invalid): GEN.generate(self.repo, self.pin, self.repo)
        link = self.work / 'output-alias'
        link.symlink_to(self.work, target_is_directory=True)
        with self.assertRaises(GEN.Invalid): GEN.generate(self.repo, self.pin, link)
        with patch.object(GEN, 'MAX_OUTPUT_BYTES', 32):
            with self.assertRaises(GEN.Invalid): GEN.generate(self.repo, self.pin, self.work)
        self.assertEqual(list(self.work.glob('nevrotick-ui-text-*')), [])

    def test_utf8_bom_duplicates_and_emoji_are_denied(self):
        malformed = [b'\xef\xbb\xbf' + self.raw, b'\xff' + self.raw,
                     self.raw.replace(b'"schema_version": 1,', b'"schema_version": 1, "\\u0073chema_version": 1,', 1)]
        emoji, _ = self.candidate(lambda data: data['messages'][0]['reference'].__setitem__('text', '\U0001f600'))
        malformed.append(emoji)
        for raw in malformed:
            with self.subTest(prefix=raw[:8]):
                with self.assertRaises(GEN.Invalid): self.emit(raw, hashlib.sha256(raw).hexdigest())

    def test_closed_schema_numeric_types_and_identifier_grammar_are_not_asserts(self):
        changes = [
            lambda data: data.__setitem__('schema_version', True),
            lambda data: data.__setitem__('gui_build_version', True),
            lambda data: data.__setitem__('text_contract_version', True),
            lambda data: data.__setitem__('catalog_schema_version', 1),
            lambda data: data['bounds'].__setitem__('MAX_MAP', 768),
            lambda data: data.__setitem__('unexpected_plugin', 'inert'),
            lambda data: data['messages'][0].__setitem__('symbol', 'BAD; panic!("metadata"); //'),
            lambda data: data['messages'][0].__setitem__('key', 'ui.bad";type.injection'),
        ]
        for change in changes:
            raw, pin = self.candidate(change)
            with self.subTest(change=change):
                with self.assertRaises(GEN.Invalid): self.emit(raw, pin)

    def test_reference_english_is_inert_and_never_emitted_into_source(self):
        sentinel = 'REFERENCE_CONTENT_SENTINEL"; panic!("payload"); //'
        raw, pin = self.candidate(lambda data: next(row for row in data['messages'] if row['active'] and row['kind'] == 'Text')['reference'].__setitem__('text', sentinel))
        files = self.emit(raw, pin)
        for name, value in files.items():
            if name.endswith(('.rs', '.slint')): self.assertNotIn(b'REFERENCE_CONTENT_SENTINEL', value)
        self.assertIn(b'REFERENCE_CONTENT_SENTINEL', files['contract-fingerprint-input.bin'])
        self.assertNotEqual(self.emit()['contract.rs'], files['contract.rs'])

    def test_binding_collisions_and_action_slot_swaps_reject(self):
        data = GEN.protected_json.read(self.raw)
        scalar = next(row for row in data['messages'] if row['active'] and row['slint'] is not None and row['slint']['slot'] is None)
        duplicate = copy.deepcopy(scalar)
        duplicate.update(key='ui.generator.duplicate', symbol='GENERATOR_DUPLICATE', ordinal=len(data['messages']) + 1)
        data['messages'].append(duplicate)
        raw = (json.dumps(data, indent=2) + '\n').encode()
        with self.assertRaises(GEN.Invalid): self.emit(raw, hashlib.sha256(raw).hexdigest())
        def swap(candidate):
            for row in candidate['messages']:
                if row['active'] and row['slint'] is not None and row['slint']['global'] == 'SourceCopy' and row['slint']['property'] == 'protocols':
                    row['slint']['slot'] = 1 - row['slint']['slot']
        raw, pin = self.candidate(swap)
        with self.assertRaises(GEN.Invalid): self.emit(raw, pin)
        slots = {row['slint']['slot'] for row in GEN.protected_json.read(self.raw)['messages']
                 if row['active'] and row['slint'] is not None and row['slint']['global'] == 'PreferencesCopy' and row['slint']['property'] == 'interval-options'}
        self.assertEqual(slots, {0, 4})
        self.assertTrue(self.emit())

    def test_optimized_python_still_rejects_code_emission_grammar_fault(self):
        raw, _ = self.candidate(lambda data: data['messages'][0].__setitem__('symbol', 'BAD; panic!("metadata"); //'))
        path = self.work / 'invalid-protected-metadata.json'
        path.write_bytes(raw)
        template = self.repo / GEN.TEMPLATE_PATH
        code = '''from pathlib import Path
import hashlib,sys
sys.path.insert(0,sys.argv[1])
import ui_text_codegen as generator
raw=Path(sys.argv[2]).read_bytes()
try: generator._emit_files(raw,hashlib.sha256(raw).hexdigest(),Path(sys.argv[3]).read_bytes())
except generator.Invalid: print('rejected')
else: raise SystemExit('fault accepted under optimized Python')
'''
        result = subprocess.run([sys.executable, '-O', '-B', '-c', code, str(SCRIPTS), str(path), str(template)], capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout.strip(), 'rejected')

    def test_cli_prepares_only_an_unapproved_review_candidate(self):
        result = subprocess.run([sys.executable, '-B', str(SCRIPTS / 'generate-ui-text.py'), '--root', str(self.repo), '--expected-contract-sha256', self.pin, '--output-parent', str(self.work)], capture_output=True, text=True, timeout=10)
        self.assertEqual(result.returncode, 0, result.stderr)
        answer = GEN.protected_json.read(result.stdout.encode())
        self.assertEqual(answer['status'], 'unapproved-review-candidate')
        self.assertTrue(Path(answer['output']).is_dir())


if __name__ == '__main__':
    unittest.main()
