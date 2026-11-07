"""Join raw per-process arcs to exact admitted source identities and test contexts."""

from __future__ import annotations

import json
from pathlib import Path
import re
import sys

from coverage import CoverageData


def main() -> int:
    output = Path(sys.argv[1])
    binding = json.loads((output / "binding.json").read_text())
    destination = CoverageData(basename=str(output / ".coverage"))
    inventory = []
    for raw in sorted(output.glob(".coverage.*")):
        match = re.search(r"\.pid([0-9]+)\.", raw.name)
        if match is None:
            raise ValueError("unrecognized-process-data-file")
        audit = json.loads((output / "audit" / (match[1] + ".json")).read_text())
        mapped = {row["path"]: str(Path(binding["root"]) / row["module"])
                  for row in audit if row["admitted"] and row["sha256"] == binding["source_hashes"][row["module"]]}
        source = CoverageData(basename=str(raw))
        source.read()
        if source.measured_files() and not source.has_arcs():
            raise ValueError("branch-instrumentation-missing")
        for context in sorted(source.measured_contexts()):
            source.set_query_context(context)
            destination.set_context(context)
            for filename in source.measured_files():
                if filename in mapped:
                    arcs = source.arcs(filename)
                    if arcs:
                        destination.add_arcs({mapped[filename]: arcs})
        inventory.append({"data_file": raw.name, "pid": int(match[1]),
                          "admitted_files": mapped,
                          "excluded_files": sorted(set(source.measured_files()) - set(mapped))})
    destination.write()
    (output / "normalization.json").write_text(json.dumps(inventory, indent=2, sort_keys=True) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
