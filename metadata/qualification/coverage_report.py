"""Report the already admitted normalized data without implicit raw-file merging."""

from __future__ import annotations

import json
from pathlib import Path
import sys

from coverage import Coverage


def main() -> int:
    output = Path(sys.argv[1])
    root = Path(sys.argv[2])
    manifest = json.loads((root / "metadata/qualification/manifest.json").read_text())
    measured = Coverage(config_file=str(output / ".coveragerc"))
    measured.load()
    measured.json_report(morfs=[str(root / name) for name in manifest["source_modules"]],
                         outfile=str(output / "coverage-raw.json"), show_contexts=True)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
