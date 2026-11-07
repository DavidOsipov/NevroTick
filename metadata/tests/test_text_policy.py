"""Independent metadata encoding/repertoire controls; no native or build effects."""
from __future__ import annotations

import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import tomllib
import unittest

ROOT = Path(__file__).absolute().parents[2]
sys.path.insert(0, str(ROOT))
from metadata.pipeline import text_policy as POLICY


class TextPolicyTests(unittest.TestCase):
    """Check rejected representations beside literal, international neighbors."""

    def test_utf8_roundtrip_preserves_normal_international_text(self):
        value = 'café cafe\u0301 Я Привет 漢字 العربية नमस्ते \U00010400 # * 0123456789 — •'
        self.assertEqual(POLICY.validate_source(value.encode('utf-8')), value)
        self.assertIsNone(POLICY.require_text(value))

    def test_pinned_assigned_letters_do_not_use_the_host_unicode_database(self):
        # The pinned Unicode 18 DGC gives these Lo/Lm/Mn; Python 3.12/UCD14
        # calls them unassigned. The data table must decide, not the host UCD.
        for codepoint in (0x1E6C0, 0x323B0, 0x16D40, 0x11B60):
            with self.subTest(codepoint=codepoint):
                POLICY.require_text(chr(codepoint))

    def test_empty_decoded_values_remain_available_for_explicit_pending_fields(self):
        self.assertIsNone(POLICY.require_text(''))
        self.assertEqual(POLICY.validate_source(b''), '')

    def test_ascii_printables_are_not_banned_by_the_broad_emoji_property(self):
        value = ''.join(chr(codepoint) for codepoint in range(0x20, 0x7F))
        POLICY.require_text(value)
        self.assertEqual(POLICY.validate_source(value.encode('ascii')), value)

    def test_document_line_endings_and_tab_remain_syntax(self):
        value = '# comment\r\nvalue = "a"\t# trailing\n'
        self.assertEqual(POLICY.validate_source(value.encode('ascii')), value)

    def test_invalid_utf8_is_rejected_without_replacement(self):
        cases = (b'\x80', b'\xc0\xaf', b'\xe2\x82', b'\xed\xa0\x80',
                 b'\xf4\x90\x80\x80', b'\xff\xfev\x00', b'\x00\x00\xfe\xff')
        for value in cases:
            with self.subTest(value=value):
                with self.assertRaises(POLICY.TextPolicyError):
                    POLICY.validate_source(value)
        self.assertEqual(POLICY.validate_source('\ufffd'.encode()), '\ufffd')

    def test_bom_is_rejected_at_start_and_inside_source(self):
        for value in ('\ufeffvalue = "a"', '# \ufeff hidden', 'a\ufeffb'):
            with self.subTest(value=value):
                with self.assertRaises(POLICY.TextPolicyError):
                    POLICY.validate_source(value.encode('utf-8'))

    def test_emoji_forms_and_components_are_rejected(self):
        cases = ('\U0001F600', '\u2600', '\U0001F1E6\U0001F1E7', '\U0001F3FB',
                 '\U0001F469\u200D\U0001F4BB', '1\uFE0F\u20E3', '\u20E3',
                 '\uFE0F', '\U000E0100', '\U000E0061', '\U0001FAEA')
        for value in cases:
            with self.subTest(value=value):
                with self.assertRaises(POLICY.TextPolicyError):
                    POLICY.require_text(value)

    def test_raw_comments_are_checked_but_ascii_comment_explanations_are_data(self):
        with self.assertRaises(POLICY.TextPolicyError):
            POLICY.validate_source('# \U0001F600\n'.encode('utf-8'))
        self.assertEqual(POLICY.validate_source(br'# \U0001F600'), r'# \U0001F600')

    def test_ascii_toml_escapes_must_be_checked_after_decoding(self):
        cases = (r'value = "\U0001F600"', r'value = "\u200B"',
                 r'value = "\uFE0F"', r'value = "\uFDD0"')
        for raw in cases:
            with self.subTest(raw=raw):
                decoded = tomllib.loads(POLICY.validate_source(raw.encode('ascii')))
                with self.assertRaises(POLICY.TextPolicyError):
                    POLICY.require_text(decoded['value'])
        decoded = tomllib.loads(POLICY.validate_source(br'value = "\u042F"'))
        self.assertEqual(decoded['value'], 'Я')
        POLICY.require_text(decoded['value'])

    def test_decoded_keys_use_the_same_policy(self):
        decoded = tomllib.loads(POLICY.validate_source(br'"\U0001F600" = "value"'))
        with self.assertRaises(POLICY.TextPolicyError):
            POLICY.require_text(next(iter(decoded)))

    def test_invisible_formatting_and_filler_characters_are_rejected(self):
        for codepoint in (0x00AD, 0x034F, 0x061C, 0x115F, 0x1160, 0x180E,
                          0x200B, 0x200C, 0x200D, 0x200E, 0x202E, 0x2066,
                          0x2069, 0xFEFF, 0xFFF9, 0xE0001):
            with self.subTest(codepoint=codepoint):
                with self.assertRaises(POLICY.TextPolicyError):
                    POLICY.require_text(chr(codepoint))

    def test_controls_except_toml_line_syntax_are_rejected(self):
        for codepoint in (*range(0x20), *range(0x7F, 0xA0), 0x2028, 0x2029):
            if codepoint in (0x09, 0x0A, 0x0D):
                continue
            with self.subTest(codepoint=codepoint):
                with self.assertRaises(POLICY.TextPolicyError):
                    POLICY.require_text(chr(codepoint))

    def test_private_use_unassigned_and_surrogates_are_rejected(self):
        for codepoint in (0x0378, 0xE000, 0xF0000, 0x100000, 0xD800, 0xDFFF):
            with self.subTest(codepoint=codepoint):
                with self.assertRaises(POLICY.TextPolicyError):
                    POLICY.require_text(chr(codepoint))

    def test_all_66_noncharacters_are_rejected(self):
        values = list(range(0xFDD0, 0xFDF0))
        values += [plane * 0x10000 + tail for plane in range(17)
                   for tail in (0xFFFE, 0xFFFF)]
        self.assertEqual(len(values), 66)
        for codepoint in values:
            with self.subTest(codepoint=codepoint):
                with self.assertRaises(POLICY.TextPolicyError):
                    POLICY.require_text(chr(codepoint))
        POLICY.require_text('\ufffd')

    def test_strict_extended_pictographic_policy_includes_text_style_marks(self):
        for value in ('\u00a9', '\u00ae', '\u2122'):
            with self.subTest(value=value):
                with self.assertRaises(POLICY.TextPolicyError):
                    POLICY.require_text(value)
        POLICY.require_text('Copyright; registered; trademark')

    def test_source_and_decoded_text_budgets_have_exact_healthy_neighbors(self):
        value = '#' + 'a' * (POLICY.MAX_SOURCE_BYTES - 1)
        self.assertEqual(len(POLICY.validate_source(value.encode())), POLICY.MAX_SOURCE_BYTES)
        with self.assertRaises(POLICY.TextPolicyError):
            POLICY.validate_source((value + 'a').encode())
        POLICY.require_text('a' * POLICY.MAX_TEXT_CHARACTERS)
        with self.assertRaises(POLICY.TextPolicyError):
            POLICY.require_text('a' * (POLICY.MAX_TEXT_CHARACTERS + 1))

    def test_exact_input_types_prevent_subclass_method_execution(self):
        class ForgedText(str):
            def __iter__(self):
                raise AssertionError('forged text was evaluated')

        class ForgedBytes(bytes):
            def decode(self, *arguments, **keywords):
                raise AssertionError('forged bytes were evaluated')

        for value in (None, 1, [], ForgedText('text')):
            with self.subTest(value=type(value)):
                with self.assertRaises(POLICY.TextPolicyError):
                    POLICY.require_text(value)
        for value in ('text', bytearray(b'text'), ForgedBytes(b'text')):
            with self.subTest(value=type(value)):
                with self.assertRaises(POLICY.TextPolicyError):
                    POLICY.validate_source(value)

    def test_rejection_does_not_echo_the_input(self):
        for action, value in ((POLICY.require_text, 'secret\U0001F600'),
                              (POLICY.validate_source, b'secret\xff')):
            with self.subTest(action=action):
                with self.assertRaises(POLICY.TextPolicyError) as error:
                    action(value)
                self.assertNotIn('secret', str(error.exception))

    def test_embedded_unicode_table_is_structural_and_matches_the_pinned_derivation(self):
        rows = POLICY._FORBIDDEN_RANGES
        self.assertIs(type(rows), tuple)
        previous = -2
        for low, high in rows:
            self.assertIs(type(low), int)
            self.assertIs(type(high), int)
            self.assertGreater(low, previous + 1)
            self.assertLessEqual(low, high)
            self.assertLessEqual(high, 0x10FFFF)
            previous = high
        encoded = json.dumps(rows, separators=(',', ':')).encode('ascii')
        self.assertEqual(len(rows), 867)
        self.assertEqual(sum(high - low + 1 for low, high in rows), 943179)
        self.assertEqual(hashlib.sha256(encoded).hexdigest(),
                         '2822c400f5939bc78e2807a15975ec71ceee6e791b083672b705948c64da448f')
        self.assertEqual(POLICY.UNICODE_VERSION, '18.0.0')


class ProductAdmissionIntegrationTests(unittest.TestCase):
    """Exercise the actual source CLI once its shared hook is integrated."""

    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='nevrotick-text-policy-test-')
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name) / 'source'
        self.root.mkdir()
        for name in ('metadata/product.toml', 'Cargo.toml',
                     'crates/nevrotick-ui/Cargo.toml', 'crates/nevrotick-engine/Cargo.toml'):
            destination = self.root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(ROOT / name, destination)
        self.source = self.root / 'metadata/product.toml'

    def run_cli(self, source):
        self.source.write_bytes(source)
        return subprocess.run(
            [sys.executable, '-I', '-B', str(ROOT / 'metadata/pipeline/__main__.py'),
             'validate', '--root', str(self.root)], capture_output=True, text=True,
            timeout=15, check=False)

    def changed_description(self, literal):
        original = self.source.read_text(encoding='utf-8')
        selector = 'description = "A Windows time synchronization client under development."'
        self.assertEqual(original.count(selector), 1)
        return original.replace(selector, 'description = ' + literal, 1).encode('utf-8')

    def test_ordinary_international_description_is_a_healthy_control(self):
        literal = json.dumps('Metadata café 漢字 Я \U00010400', ensure_ascii=False)
        result = self.run_cli(self.changed_description(literal))
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_literal_and_escaped_emoji_cannot_enter_product_values(self):
        for literal in ('"Metadata \U0001F600"', r'"Metadata \U0001F600"'):
            with self.subTest(literal=literal):
                # Reload the original for each independent source mutation.
                shutil.copyfile(ROOT / 'metadata/product.toml', self.source)
                result = self.run_cli(self.changed_description(literal))
                self.assertNotEqual(result.returncode, 0)
                self.assertNotIn('Metadata', result.stdout)

    def test_emoji_inside_comments_is_rejected(self):
        raw = self.source.read_bytes() + '# \U0001F600\n'.encode('utf-8')
        self.assertNotEqual(self.run_cli(raw).returncode, 0)

    def test_ascii_hidden_character_escape_is_rejected(self):
        result = self.run_cli(self.changed_description(r'"Metadata \u200B"'))
        self.assertNotEqual(result.returncode, 0)

    def test_bom_is_rejected_before_product_toml_parsing(self):
        self.assertNotEqual(self.run_cli(b'\xef\xbb\xbf' + self.source.read_bytes()).returncode, 0)


if __name__ == '__main__':
    unittest.main()
