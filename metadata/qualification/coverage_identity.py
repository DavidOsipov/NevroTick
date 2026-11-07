"""Record byte identity once while retaining coverage.py's native Python tracer."""

from __future__ import annotations

from coverage_plugin import IdentityPlugin


class NativeIdentityPlugin(IdentityPlugin):
    def file_tracer(self, filename: str) -> None:
        super().file_tracer(filename)
        # Keep ordinary C-tracer line/arc semantics. Copied filenames are joined
        # to their admitted source only after raw per-process data is persisted.
        return None


def coverage_init(reg: object, options: object) -> None:
    plugin = NativeIdentityPlugin()
    reg.add_file_tracer(plugin)
    reg.add_configurer(plugin)
