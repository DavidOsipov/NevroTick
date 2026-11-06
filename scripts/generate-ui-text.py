#!/usr/bin/env python3
"""Prepare protected Slint identifier bindings for code review, without adoption."""
from pathlib import Path
import argparse
import json

import ui_text_codegen


def main() -> int:
    """Read the fixed contract under the selected repo and print its private output."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=Path(__file__).resolve().parent.parent)
    parser.add_argument('--expected-contract-sha256', required=True)
    parser.add_argument('--output-parent', type=Path)
    args = parser.parse_args()
    try:
        output = ui_text_codegen.generate(args.root, args.expected_contract_sha256, args.output_parent)
    except (ui_text_codegen.Invalid, OSError, UnicodeError, ValueError) as error:
        parser.exit(1, 'UI text generation rejected: ' + type(error).__name__ + '.\n')
    print(json.dumps({'status': 'unapproved-review-candidate', 'output': str(output)}))
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
