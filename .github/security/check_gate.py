#!/usr/bin/env python3
"""Require explicit per-language success for the exact security candidate."""

import json
import os
import re
import sys


LANGUAGES = {"python": "python", "javascript": "javascript", "rust": "rust"}


def evaluate(results, candidate):
    expected = {"source-inventory", "workflow-policy", "codeql-actions", "codeql-python",
                "codeql-javascript", "codeql-rust", "secrets"}
    if not isinstance(candidate, str) or not re.fullmatch(r"[a-f0-9]{40}", candidate):
        raise ValueError("Missing exact candidate identity.")
    if not isinstance(results, dict) or set(results) != expected:
        raise ValueError("Missing or unreviewed required job.")
    for key in ("source-inventory", "workflow-policy", "secrets", "codeql-actions"):
        if results[key].get("result") != "success":
            raise ValueError("Required security job did not succeed.")
    inventory = results["source-inventory"].get("outputs", {})
    if inventory.get("candidate") != candidate:
        raise ValueError("Source inventory does not identify this candidate.")
    for language in LANGUAGES:
        present = inventory.get(language)
        if present not in ("true", "false"):
            raise ValueError("Source presence was not established.")
        job = results[f"codeql-{language}"]
        if job.get("result") != ("success" if present == "true" else "skipped"):
            raise ValueError("A required language failed or was skipped without an empty-source inventory.")
        if present == "true" and job.get("outputs", {}).get("candidate") != candidate:
            raise ValueError("Language analysis does not identify this candidate.")
    if results["codeql-actions"].get("outputs", {}).get("candidate") != candidate:
        raise ValueError("Workflow analysis does not identify this candidate.")


def main():
    try:
        evaluate(json.loads(os.environ["REQUIRED_RESULTS"]), os.environ["EXPECTED_COMMIT"])
    except (ValueError, KeyError, TypeError, AttributeError):
        print("FAILED: Missing, failed, cancelled, skipped or stale required security evidence.", file=sys.stderr)
        return 1
    print("All expected repository security jobs supplied successful evidence for this candidate.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
