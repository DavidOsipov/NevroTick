"""Attribute copied Python frames only to byte-identical admitted source files."""

from __future__ import annotations

import atexit
import hashlib
import json
import os
from pathlib import Path

from coverage import CoveragePlugin, FileTracer


class IdentityTracer(FileTracer):
    def __init__(self, source: str) -> None:
        self.source = source

    def source_filename(self) -> str:
        return self.source


class IdentityPlugin(CoveragePlugin):
    def __init__(self) -> None:
        self.binding = json.loads(Path(os.environ["NEVROTICK_COVERAGE_BINDING"]).read_text())
        self.root = Path(self.binding["root"])
        self.audit: list[dict[str, object]] = []
        atexit.register(self.save_audit)

    def configure(self, config: object) -> None:
        # Each child reconfigures the serialized coverage settings. The parent
        # runner updates this inherited variable at the actual test boundary.
        config.set_option("run:context", os.environ.get("NEVROTICK_COVERAGE_CONTEXT", ""))

    def file_tracer(self, filename: str) -> FileTracer | None:
        path = Path(filename)
        relative = next((name for name in self.binding["source_hashes"]
                         if filename.endswith("/" + name)), None)
        if relative is None:
            return None
        digest = hashlib.sha256(path.read_bytes()).hexdigest()
        admitted = digest == self.binding["source_hashes"][relative]
        self.audit.append({"pid": os.getpid(), "path": filename, "module": relative,
                           "sha256": digest, "admitted": admitted,
                           "context": os.environ.get("NEVROTICK_COVERAGE_CONTEXT", "")})
        # An altered producer/driver fixture gets its own out-of-scope filename;
        # its line numbers can never inflate the healthy module denominator.
        source = self.root / relative if admitted else Path(self.binding["audit"]) / "altered" / digest / relative
        return IdentityTracer(str(source))

    def file_reporter(self, filename: str) -> str:
        return "python"

    def save_audit(self) -> None:
        destination = Path(self.binding["audit"]) / (str(os.getpid()) + ".json")
        destination.write_text(json.dumps(self.audit, sort_keys=True) + "\n")


def coverage_init(reg: object, options: object) -> None:
    plugin = IdentityPlugin()
    reg.add_file_tracer(plugin)
    reg.add_configurer(plugin)
