"""Observe every actual isolated Python CLI child for subprocess reconciliation."""

from __future__ import annotations

import json
import os
from pathlib import Path
import subprocess
import sys


def install_process_ledger() -> None:
    original = subprocess.Popen

    class ObservedPopen(original):
        def __init__(self, *args: object, **kwargs: object) -> None:
            command = args[0] if args else kwargs.get("args")
            if isinstance(command, (list, tuple)) and command and command[0] == sys.executable and sys.pycache_prefix:
                command = [command[0], "-X", "pycache_prefix=" + sys.pycache_prefix, *command[1:]]
                if args:
                    args = (command, *args[1:])
                else:
                    kwargs["args"] = command
            super().__init__(*args, **kwargs)
            if isinstance(command, (list, tuple)) and "-I" in command and "-B" in command:
                with Path(os.environ["NEVROTICK_COVERAGE_PROCESS_LEDGER"]).open("a") as stream:
                    stream.write(json.dumps({"pid": self.pid, "command": list(map(str, command)),
                                             "context": os.environ.get("NEVROTICK_COVERAGE_CONTEXT", "")}) + "\n")

    subprocess.Popen = ObservedPopen
