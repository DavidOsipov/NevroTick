#!/usr/bin/env python3
"""Fixed bit-length dependency entrypoint; no admission or execution on import."""
from __future__ import annotations

import importlib.util
from pathlib import Path
import sys

sys.dont_write_bytecode = True

_root = Path(__file__).resolve().parents[2]
_spec = importlib.util.spec_from_file_location(
    "fraction_bit_length_source_binding", Path(__file__).with_name("python_source_binding.py"))
_binding_module = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(_binding_module)
_binding = _binding_module.SourceBinding(_root, sys._getframe())
_bridge = _binding.load_module("verify_fraction_verus_bridge", Path(__file__).with_name("verify_fraction_verus_bridge.py"))
require = _bridge.require
run_bit_length_campaign = _bridge.run_bit_length_campaign


if __name__ == "__main__":
    require(sys.argv[1:] == [], "This finite card accepts no CLI, tool, profile, budget or output overrides")
    sys.exit(run_bit_length_campaign())
