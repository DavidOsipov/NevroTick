"""Synthetic receipt transport controls; no native, Cargo or tool execution."""
import contextlib
import importlib.util
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch


ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location(
    'core_native_receipt_controls', ROOT / 'scripts/verify-core-analysis.py')
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


class NativeReceiptTransportTests(unittest.TestCase):
    def test_reader_accepts_the_retained_receipt_size_without_widening_inventory(self):
        # The actual retained 213-row receiving report is 16,393,749 bytes.
        # This independent payload tests the byte-reader role, not cohort truth.
        size = 16_393_749
        prefix, suffix = b'{"padding":"', b'"}'
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'verification.json'
            path.write_bytes(prefix + b'x' * (size - len(prefix) - len(suffix)) + suffix)
            self.assertEqual(path.stat().st_size, size)
            self.assertEqual(len(MODULE.native_receipt_document(path)['padding']), size - 14)
            with self.assertRaisesRegex(MODULE.Incomplete, 'fixed byte allowance'):
                MODULE.inventory_document(path)

    @staticmethod
    def emit_synthetic(payload, output):
        def execute(args, report):
            args.output_dir.mkdir(mode=0o700)
            report.update(payload, owned_output=str(args.output_dir))

        with patch.object(MODULE, 'execute', execute), contextlib.redirect_stdout(io.StringIO()):
            return MODULE.main(['--lane', 'coverage', '--vendor-dir', str(output.parent / 'unused'),
                                '--output-dir', str(output)])

    def test_real_final_writer_preserves_all_fields_in_one_compact_record(self):
        payload = {'synthetic_marker': 'receipt-unit-control',
                   'opaque': 'line one\nline two \u00e9 \u03a9',
                   'facts': [None, True, False, 0, (1 << 64) - 1, 1.25, {'nested': [2, 3]}]}
        with tempfile.TemporaryDirectory() as temporary:
            output = Path(temporary) / 'owned'
            self.assertEqual(self.emit_synthetic(payload, output), 0)
            wire = (output / 'verification.json').read_bytes()
            self.assertTrue(wire.endswith(b'\n'))
            self.assertEqual(wire.count(b'\n'), 1)
            decoded = json.loads(wire)
            for name, value in payload.items():
                self.assertEqual(decoded[name], value)

    def test_real_final_writer_cannot_publish_nonfinite_json(self):
        for value in (float('nan'), float('inf'), -float('inf')):
            with self.subTest(value=value), tempfile.TemporaryDirectory() as temporary:
                output = Path(temporary) / 'owned'
                with self.assertRaises(MODULE.Incomplete):
                    self.emit_synthetic({'invalid_scalar': value}, output)
                self.assertFalse((output / 'verification.json').exists())

    def test_writer_rejects_encoded_content_beyond_the_native_byte_allowance(self):
        with self.assertRaisesRegex(MODULE.Incomplete, 'fixed byte allowance'):
            MODULE.native_receipt_text({'padding': 'x' * (64 * 1024 * 1024)})

    def test_both_receipt_roles_reject_exponent_overflow_beside_a_finite_neighbor(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'document.json'
            for reader in (MODULE.inventory_document, MODULE.native_receipt_document):
                path.write_text('{"elapsed":1e3}')
                self.assertEqual(reader(path), {'elapsed': 1000.0})
                for token in ('1e999', '-1e999'):
                    with self.subTest(reader=reader.__name__, token=token):
                        path.write_text('{"elapsed":' + token + '}')
                        with self.assertRaises(MODULE.Incomplete):
                            reader(path)


if __name__ == '__main__':
    unittest.main()
