"""Run the owned build-time metadata pipeline with an isolated Python interpreter."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Literal
import sys

# `python -I path/to/__main__.py` deliberately omits the script directory from
# imports. Add only the fixed checkout containing this entry, never argv data.
if __package__ in (None, ""):
    sys.path.insert(0, str(Path(__file__).absolute().parents[2]))

from metadata.pipeline.contract import MetadataError, load_metadata  # noqa: E402
from metadata.pipeline.generate import PublicationUncertainError, check, field_counts, generate  # noqa: E402


class _Arguments(argparse.Namespace):
    """Typed destinations of this fixed command parser."""
    command: Literal["validate", "generate", "check"]
    root: Path
    source: Path | None
    stage: Literal["source", "release"]
    output: Path | None


def main(arguments: list[str] | None = None) -> int:
    """Validate, generate or compare one candidate without native/build side effects."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("command", choices=("validate", "generate", "check"))
    parser.add_argument("--root", type=Path, default=Path(__file__).absolute().parents[2])
    parser.add_argument("--source", type=Path)
    parser.add_argument("--stage", choices=("source", "release"), default="source")
    parser.add_argument("--output", type=Path)
    args = parser.parse_args(arguments, namespace=_Arguments())
    if (args.command == "validate") != (args.output is None):
        parser.error("validate has no output; generate/check require --output")
    report: dict[str, object]
    try:
        if args.command == "validate":
            model = load_metadata(args.root, args.source, stage=args.stage)
            report = {"status": "valid-" + args.stage, "component_count": len(model.components),
                      "pending_count": len(model.pending), **field_counts(model)}
        elif args.command == "generate":
            assert args.output is not None
            report = dict(generate(args.root, args.output, args.source, stage=args.stage))
        else:
            assert args.output is not None
            report = dict(check(args.root, args.output, args.source, stage=args.stage))
    except PublicationUncertainError:
        # A complete rename is already visible. Report the distinct outcome
        # without deleting it, exposing raw errors or automatically retrying.
        uncertain: dict[str, object] = {"schema_version": 1, "status": "publication-uncertain",
                          "durability": "unconfirmed",
                          "action": "check-existing-output-before-retry"}
        print(json.dumps(uncertain, sort_keys=True, allow_nan=False))
        return 3
    except (MetadataError, OSError, UnicodeError, RecursionError, ValueError):
        # Raw rejected values, source paths and platform errors may be private.
        # Detailed valid pending owners are part of metadata.json, not stderr.
        print("Metadata error: metadata-input-or-output-invalid", file=sys.stderr)
        return 1
    print(json.dumps(report, sort_keys=True, allow_nan=False))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
