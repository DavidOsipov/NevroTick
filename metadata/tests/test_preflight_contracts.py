"""Raw lexical admission protects the TOML parser and preserves quoted data."""

from __future__ import annotations

import importlib
import sys
import tomllib
import unittest
from unittest import mock

from test_pipeline import REPOSITORY


class PreflightContractTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls) -> None:
        sys.path.insert(0, str(REPOSITORY))
        try:
            cls.contract = importlib.import_module("metadata.pipeline.contract")
        finally:
            sys.path.remove(str(REPOSITORY))

    def test_empty_quoted_array_entries_do_not_change_adjacent_depth(self) -> None:
        nested: object = 0
        for _ in range(15):
            nested = [nested]
        for quote in ('"', "'", '"""', "'''"):
            text = f"a=[{quote}{quote}]\nb=[{quote}{quote}]\nc=" + "[" * 15 + "0" + "]" * 15 + "\n"
            expected = {"a": [""], "b": [""], "c": nested}
            with self.subTest(quote=quote):
                self.assertEqual(tomllib.loads(text), expected)
                self.assertEqual(self.contract._preflight(text.encode(), "owned-preflight"), text)
                self.assertEqual(self.contract._decode(text.encode(), "owned-preflight"), expected)

    def test_multiline_quote_and_bracket_payloads_remain_literal(self) -> None:
        for quote in ('"""', "'''"):
            for leading in ("", quote[0], quote[0] * 2):
                literal = leading + "[" * 20
                text = f"a={quote}{literal}{quote}\nb=0\n"
                with self.subTest(quote=quote, leading=leading):
                    self.assertEqual(tomllib.loads(text), {"a": literal, "b": 0})
                    self.assertEqual(self.contract._preflight(text.encode(), "owned-preflight"), text)
                    self.assertEqual(self.contract._decode(text.encode(), "owned-preflight"), {"a": literal, "b": 0})
            literal = "[" * 20
            text = f"a=[{quote}{literal}{quote}]\nb=[{quote}{literal}{quote}]\nc=0\n"
            with self.subTest(quote=quote, adjacent_arrays=True):
                self.assertEqual(tomllib.loads(text), {"a": [literal], "b": [literal], "c": 0})
                self.assertEqual(self.contract._preflight(text.encode(), "owned-preflight"), text)
                self.assertEqual(self.contract._decode(text.encode(), "owned-preflight"), {"a": [literal], "b": [literal], "c": 0})

    def test_lexical_depth_below_and_at_sixteen_is_admitted_exactly(self) -> None:
        # The lexical budget is sixteen containers; decoded-tree depth is a
        # separate later boundary and must not replace this receiving guard.
        self.assertEqual(self.contract.MAX_DEPTH, 16)
        for depth in (15, 16):
            text = "a=" + "[" * depth + "0" + "]" * depth + "\n"
            with self.subTest(depth=depth):
                self.assertIsInstance(tomllib.loads(text)["a"], list)
                self.assertEqual(self.contract._preflight(text.encode(), "owned-preflight"), text)

    def require_depth_refusal_before_parser(self, literals: tuple[tuple[str, str], ...]) -> None:
        for quote, value in literals:
            text = f"a=[{quote}{value}{quote}," + "[" * 16 + "0" + "]" * 16 + "]\n"
            with self.subTest(quote=quote, value=value):
                # This is valid TOML. Refusal is the earlier resource contract,
                # rather than a syntax error or the later decoded-depth guard.
                self.assertIsInstance(tomllib.loads(text)["a"], list)
                with mock.patch.object(self.contract.tomllib, "loads", side_effect=AssertionError("parser reached before lexical admission")) as parser:
                    with self.assertRaisesRegex(self.contract.MetadataError, "lexical nesting budget"):
                        self.contract._decode(text.encode(), "owned-preflight")
                    parser.assert_not_called()

    def test_seventeenth_container_after_quotes_is_refused_before_parser(self) -> None:
        self.require_depth_refusal_before_parser((('"', ""), ("'", ""), ('"""', ""), ("'''", ""),
                                                 ('"""', '""'), ("'''", "''")))

    def test_seventeenth_container_after_single_literal_quote_is_refused_before_parser(self) -> None:
        self.require_depth_refusal_before_parser((('"""', '"'), ("'''", "'")))

    def test_multiline_leading_and_trailing_quotes_preserve_depth_neighbors(self) -> None:
        for quote in ('"""', "'''"):
            for leading in range(3):
                for trailing in range(3):
                    literal = quote[0] * leading + "[" * 20 + "]" * 20 + "#.{" + quote[0] * trailing
                    prefix = f"a={quote}{literal}{quote}\n"
                    for depth in (15, 16, 17):
                        text = prefix + "b=" + "[" * depth + "0" + "]" * depth + "\n"
                        nested: object = 0
                        for _ in range(depth):
                            nested = [nested]
                        expected = {"a": literal, "b": nested}
                        with self.subTest(quote=quote, leading=leading, trailing=trailing, depth=depth):
                            self.assertEqual(tomllib.loads(text), expected)
                            if depth == 17:
                                with mock.patch.object(self.contract.tomllib, "loads", side_effect=AssertionError("parser reached before lexical admission")) as parser:
                                    with self.assertRaisesRegex(self.contract.MetadataError, "lexical nesting budget"):
                                        self.contract._decode(text.encode(), "owned-preflight")
                                    parser.assert_not_called()
                            else:
                                self.assertEqual(self.contract._preflight(text.encode(), "owned-preflight"), text)
                                if depth == 15:
                                    self.assertEqual(self.contract._decode(text.encode(), "owned-preflight"), expected)
                                else:
                                    # Sixteen lexical containers are allowed; the
                                    # scalar is at decoded depth seventeen.
                                    with mock.patch.object(self.contract.tomllib, "loads", wraps=tomllib.loads) as parser:
                                        with self.assertRaisesRegex(self.contract.MetadataError, "decoded structure budget"):
                                            self.contract._decode(text.encode(), "owned-preflight")
                                        parser.assert_called_once_with(text)

    def test_multiline_basic_escapes_preserve_comments_and_dotted_key_budget(self) -> None:
        for trailing in range(3):
            body = r'\"' + "[" * 20 + r'\\' + '"' * trailing
            literal = '"' + "[" * 20 + "\\" + '"' * trailing
            prefix = 'a="""' + body + '"""\n# ' + '[' * 20 + '.' * 20 + '"' * 9 + "\n"
            for depth in (15, 16, 17):
                text = prefix + "k." * (depth - 1) + "leaf=0\n"
                expected: dict[str, object] = {"leaf": 0}
                for _ in range(depth - 1):
                    expected = {"k": expected}
                with self.subTest(trailing=trailing, dotted_depth=depth):
                    self.assertEqual(tomllib.loads(text), {"a": literal, **expected})
                    if depth == 17:
                        with mock.patch.object(self.contract.tomllib, "loads", side_effect=AssertionError("parser reached before dotted-key admission")) as parser:
                            with self.assertRaisesRegex(self.contract.MetadataError, "dotted-key depth budget"):
                                self.contract._decode(text.encode(), "owned-preflight")
                            parser.assert_not_called()
                    else:
                        self.assertEqual(self.contract._decode(text.encode(), "owned-preflight"), {"a": literal, **expected})

    def test_overlong_multiline_quote_runs_remain_syntax_errors(self) -> None:
        for character in ('"', "'"):
            for count in (9, 10):
                text = "a=" + character * count + "\nb=0\n"
                with self.subTest(character=character, count=count):
                    with self.assertRaises(tomllib.TOMLDecodeError):
                        tomllib.loads(text)
                    self.assertEqual(self.contract._preflight(text.encode(), "owned-preflight"), text)
                    with mock.patch.object(self.contract.tomllib, "loads", wraps=tomllib.loads) as parser:
                        with self.assertRaisesRegex(self.contract.MetadataError, "malformed TOML"):
                            self.contract._decode(text.encode(), "owned-preflight")
                        parser.assert_called_once_with(text)


if __name__ == "__main__":
    unittest.main()
