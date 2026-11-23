"""Report the already admitted normalized data without implicit raw-file merging."""

from __future__ import annotations

import json
from pathlib import Path
import sys
from tempfile import TemporaryDirectory
from typing import Any, TextIO

from coverage import Coverage, CoverageData


def main() -> int:
    output: Path = Path(sys.argv[1])
    root: Path = Path(sys.argv[2])
    manifest: dict[str, Any] = json.loads((root / "metadata/qualification/manifest.json").read_text())
    measured: Coverage = Coverage(config_file=str(output / ".coveragerc"))
    measured.load()
    source_modules: list[str] = manifest["source_modules"]
    directory: str
    with TemporaryDirectory(dir=output) as directory:
        staged: Path = Path(directory) / "coverage-raw.json"
        # Coverage.py format 3 repeats the entire file's context map in every
        # function and class. Qualification only consumes contexts per file.
        measured.json_report(morfs=[str(root / name) for name in source_modules],
                             outfile=str(staged), show_contexts=False)
        report: dict[str, Any] = json.loads(staged.read_text(encoding="utf-8"))
        files: dict[str, dict[str, object]] = report["files"]
        if len(files) != len(source_modules):
            raise ValueError("raw-report-inventory-mismatch")
        data: CoverageData = measured.get_data()
        measured_files: set[str] = data.measured_files()
        name: str
        for name in source_modules:
            absolute: str = str(root / name)
            aliases: set[str] = {name, absolute}
            report_keys: set[str] = files.keys() & aliases
            if len(report_keys) != 1:
                raise ValueError("missing-or-duplicate-raw-module-observation:" + name)
            filename: str
            filename, *_ = measured.analysis2(absolute)
            data_keys: set[str] = measured_files & aliases
            if filename not in aliases or (data_keys and data_keys != {filename}):
                raise ValueError("ambiguous-measured-module-observation:" + name)
            files[report_keys.pop()]["contexts"] = data.contexts_by_lineno(filename)
        report["meta"]["show_contexts"] = True
        file: TextIO
        with staged.open("w", encoding="utf-8") as file:
            json.dump(report, file)
        staged.replace(output / "coverage-raw.json")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
