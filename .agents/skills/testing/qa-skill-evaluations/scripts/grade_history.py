#!/usr/bin/env python3
"""Grade the inspected, synthetic history exercise with an admitted host rustc.

Creates a new caller-owned output directory; never modifies the input candidate.
This standard-library-only runner is not a sandbox for hostile Rust source.
Exit 0: five checks passed; 1: executed assertion failures; 2: incomplete/setup.
"""

from __future__ import annotations

import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess


GRADER = r'''#![forbid(unsafe_code)]
#[path = "candidate.rs"]
mod subject;

#[cfg(test)]
mod grading {
    use super::subject::{History, OutOfRange};

    #[test]
    fn initial_and_healthy_append() {
        let mut h = History::new();
        assert_eq!(h.samples(), &[]);
        for sample in [-4, 0, 7] {
            assert_eq!(h.push(sample), Ok(()));
        }
        assert_eq!(h.samples(), &[-4, 0, 7]);
    }

    #[test]
    fn full_rollover_is_exactly_once() {
        let mut h = History::new();
        for sample in [11, 22, 33, 44, 55] {
            assert_eq!(h.push(sample), Ok(()));
        }
        assert_eq!(h.samples(), &[33, 44, 55]);
    }

    #[test]
    fn inclusive_boundaries_and_duplicates() {
        let mut h = History::new();
        for sample in [-100, 100, 100] {
            assert_eq!(h.push(sample), Ok(()));
        }
        assert_eq!(h.samples(), &[-100, 100, 100]);
    }

    #[test]
    fn rejection_preserves_partial_history() {
        for prefix in [vec![], vec![8], vec![8, -9]] {
            for invalid in [i32::MIN, -101, 101, i32::MAX] {
                let mut h = History::new();
                for &sample in &prefix {
                    assert_eq!(h.push(sample), Ok(()));
                }
                assert_eq!(h.push(invalid), Err(OutOfRange));
                assert_eq!(h.samples(), prefix.as_slice());
            }
        }
    }

    #[test]
    fn rejection_preserves_full_history_and_recovery() {
        let mut h = History::new();
        for sample in [3, 6, 9] {
            assert_eq!(h.push(sample), Ok(()));
        }
        for invalid in [-101, 101, i32::MIN, i32::MAX] {
            assert_eq!(h.push(invalid), Err(OutOfRange));
            assert_eq!(h.samples(), &[3, 6, 9]);
        }
        assert_eq!(h.push(12), Ok(()));
        assert_eq!(h.samples(), &[6, 9, 12]);
    }
}
'''

EXPECTED = {
    "grading::initial_and_healthy_append",
    "grading::full_rollover_is_exactly_once",
    "grading::inclusive_boundaries_and_duplicates",
    "grading::rejection_preserves_partial_history",
    "grading::rejection_preserves_full_history_and_recovery",
}


def digest(data: bytes) -> str:
    """Identify exact supplied bytes; a digest is not source review."""
    return hashlib.sha256(data).hexdigest()


def main() -> int:
    """Run finite compiler/discovery/test stages and retain each exit and log."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--candidate", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--rustc", type=Path, required=True)
    args = parser.parse_args()
    rustc = args.rustc.absolute()  # Preserve a rustup proxy's invocation name.
    output = args.out_dir.absolute()
    try:
        candidate = args.candidate.resolve(strict=True)
        source = candidate.read_bytes()
        output.mkdir(parents=True, exist_ok=False)
    except OSError as exc:
        parser.error(f"incomplete setup: {exc}")
    (output / "candidate.rs").write_bytes(source)
    (output / "grader.rs").write_text(GRADER, encoding="utf-8")
    result = {
        "status": "incomplete",
        "candidate_sha256": digest(source),
        "runner_sha256": digest(Path(__file__).read_bytes()),
        "grader_sha256": digest(GRADER.encode()),
        "expected_tests": sorted(EXPECTED),
        "stages": [],
    }

    def run(stage: str, command: list[str], seconds: int) -> tuple[int | None, str]:
        """Retain partial output on timeout; never turn setup failure into a pass."""
        log = output / f"{stage}.log"
        error = None
        with log.open("w", encoding="utf-8") as stream:
            try:
                code = subprocess.run(
                    command, cwd=output, stdin=subprocess.DEVNULL,
                    stdout=stream, stderr=subprocess.STDOUT,
                    timeout=seconds, check=False,
                ).returncode
            except (OSError, subprocess.TimeoutExpired) as exc:
                code, error = None, str(exc)
        result["stages"].append({
            "name": stage, "command": command, "exit_code": code,
            "timeout_seconds": seconds, "error": error, "log": str(log),
        })
        return code, log.read_text(encoding="utf-8", errors="replace")

    def finish(status: str, exit_code: int) -> int:
        """Publish a scoped result without replacing an earlier output directory."""
        result["status"] = status
        (output / "result.json").write_text(
            json.dumps(result, indent=2) + "\n", encoding="utf-8",
        )
        print(f"{status}: {output / 'result.json'}")
        return exit_code

    code, _ = run("compiler-version", [str(rustc), "--version"], 10)
    if code != 0:
        return finish("incomplete", 2)
    binary = output / "history-tests"
    code, _ = run("compile", [str(rustc), "--edition=2024", "--test",
                              "grader.rs", "-o", str(binary)], 30)
    if code != 0:
        return finish("incomplete", 2)
    code, listing = run("discovery", [str(binary), "grading::", "--list"], 10)
    discovered = re.findall(r"^(.+): test$", listing, re.MULTILINE)
    if code != 0 or len(discovered) != len(EXPECTED) or set(discovered) != EXPECTED:
        return finish("incomplete", 2)
    code, report = run("tests", [str(binary), "grading::", "--test-threads=1"], 10)
    result["discovered_tests"] = sorted(discovered)
    summaries = re.findall(
        r"^test result: (ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored;",
        report, re.MULTILINE,
    )
    if len(summaries) != 1:
        return finish("incomplete", 2)
    verdict, passed, failed, ignored = summaries[0]
    result["counts"] = dict(passed=int(passed), failed=int(failed), ignored=int(ignored))
    if int(passed) + int(failed) != 5 or int(ignored) != 0:
        return finish("incomplete", 2)
    if code == 0 and verdict == "ok" and int(passed) == 5:
        return finish("pass", 0)
    if code == 101 and verdict == "FAILED" and int(failed) > 0:
        return finish("test_failure", 1)
    return finish("incomplete", 2)


if __name__ == "__main__":
    raise SystemExit(main())
