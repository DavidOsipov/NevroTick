#!/usr/bin/env python3
"""Fixed request-timing entrypoint; importing starts no admission or native job."""
from __future__ import annotations

import importlib.util
from pathlib import Path
import sys

sys.dont_write_bytecode = True

_root = Path(__file__).resolve().parents[2]
_spec = importlib.util.spec_from_file_location(
    "request_timing_source_binding", Path(__file__).with_name("python_source_binding.py"))
_binding_module = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_binding_module)
_binding = _binding_module.SourceBinding(_root, sys._getframe())
_runner = _binding.load_module("request_timing_captured_runner", Path(__file__).with_name("verify.py"))


def run_request_timing_campaign() -> int:
    """Dispatch exactly this finite fresh five-job Kani-proof/Cadical card."""
    return _runner.main([
        "--output-dir", str(_runner.request_timing_output()),
        "--subject", "request-timing", "--tool-admission-profile", "kani-proof",
        "--solver-backend", "cadical",
    ])


def main() -> int:
    """Reject all CLI overrides before creating evidence or admitting tools."""
    if sys.argv[1:]:
        raise _runner.Incomplete("This finite card accepts no CLI, tool, profile, budget or output overrides")
    return run_request_timing_campaign()


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (OSError, ValueError, _runner.Incomplete) as error:
        print(f"Request-timing verification incomplete: {error}", file=sys.stderr)
        raise SystemExit(1)
