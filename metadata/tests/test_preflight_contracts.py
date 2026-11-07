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


if __name__ == "__main__":
    unittest.main()
