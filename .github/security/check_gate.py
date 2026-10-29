#!/usr/bin/env python3
"""Require mandatory checks and explicit CodeQL deferral for this candidate."""

import json
import os
import re
import sys


LANGUAGES = ("python", "javascript", "rust")
CODEQL_JOBS = ("codeql-actions", "codeql-python", "codeql-javascript", "codeql-rust")
# Independent of the workflow output: re-enabling CodeQL needs a reviewed gate change.
CODEQL_STATUS = "deferred"


def evaluate(results, candidate):
    expected = {"source-inventory", "workflow-policy", "codeql-actions", "codeql-python",
                "codeql-javascript", "codeql-rust", "secrets"}
    if not isinstance(candidate, str) or not re.fullmatch(r"[a-f0-9]{40}", candidate):
        raise ValueError("Missing exact candidate identity.")
    if not isinstance(results, dict) or set(results) != expected:
        raise ValueError("Missing or unreviewed required job.")
    for key in ("source-inventory", "workflow-policy", "secrets"):
        if results[key].get("result") != "success":
            raise ValueError("Required security job did not succeed.")
    inventory = results["source-inventory"].get("outputs", {})
    if inventory.get("candidate") != candidate:
        raise ValueError("Source inventory does not identify this candidate.")
    if inventory.get("codeql") != CODEQL_STATUS:
        raise ValueError("CodeQL policy does not match the reviewed explicit deferral.")
    for language in LANGUAGES:
        present = inventory.get(language)
        if present not in ("true", "false"):
            raise ValueError("Source presence was not established.")
    for key in CODEQL_JOBS:
        job = results[key]
        if job.get("result") != "skipped" or job.get("outputs", {}):
            raise ValueError("A deferred CodeQL job ran or claimed analysis evidence.")


def main():
    try:
        evaluate(json.loads(os.environ["REQUIRED_RESULTS"]), os.environ["EXPECTED_COMMIT"])
    except (ValueError, KeyError, TypeError, AttributeError):
        print("FAILED: Missing, failed, skipped or stale mandatory evidence, or invalid CodeQL deferral.", file=sys.stderr)
        return 1
    print("Required workflow and secret checks succeeded for this candidate. "
          "CodeQL is deferred; no CodeQL SAST evidence was produced.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
