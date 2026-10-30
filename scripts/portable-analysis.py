#!/usr/bin/env python3
"""Fixed-scope portable evidence and sealed execution helpers (GPL-2.0-only).

Extracted/adapted from NevroTick scripts/verify-wire-analysis.py (29 October
2026), David Osipov with AI assistance. Existing repository licence applies.
Original source SHA-256:
4b642c9ff544f46c84277cb3005d91168a26d5f30b798785199e30806f097ab1.
The original wire oracle/policy remains with its wrapper. Namespace, tool/source
admission and bounded archive extraction remain with their canonical scripts.
No function installs tools, downloads dependencies or grants new host effects.
"""
from __future__ import annotations

import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path
import re
import shutil
import sys
import time
import tomllib
from typing import NamedTuple

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
FUZZ_CARGO_CONFIG = ('[source.crates-io]\nreplace-with="fuzz-reviewed"\n'
                     '[source.fuzz-reviewed]\ndirectory="/work/fuzz-vendor"\n')
MUTATION_LIST_WARNING = (
    " WARN --jobs=12 is probably too high and may overload your machine: each job runs a separate "
    "`cargo` process, and cargo may internally start many threads and subprocesses; values <= 8 "
    "are usually safe\n"
)

def load_module(name: str, path: Path):
    """Load reviewed repository tooling without running its CLI entry point."""
    specification = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module

ANALYSIS = load_module("portable_analysis_admission", ROOT / "scripts/check-analysis.py")
PRODUCT = load_module("portable_product_admission", ROOT / "scripts/verify.py")
PREPARE = load_module("portable_archive_admission", ROOT / "scripts/prepare-product.py")
RUNTIME = ANALYSIS.RUNTIME
Incomplete = ANALYSIS.Incomplete


class Scope(NamedTuple):
    """Immutable exact package/file scope, supplied by a reviewed wrapper."""
    package: str
    subjects: tuple[tuple[str, str], ...]
    policy: str

    def select(self, subject: str) -> dict[str, str]:
        """Select fixed reviewed files; never accept arbitrary paths."""
        mapping = dict(self.subjects)
        if len(mapping) != len(self.subjects) or not mapping or len(set(mapping.values())) != len(mapping):
            raise Incomplete("Invalid duplicate or empty analysis scope.")
        if subject == "all":
            return mapping
        if subject not in mapping:
            raise Incomplete("Unknown portable analysis subject.")
        return {subject: mapping[subject]}

    def selected_file(self, filename: str) -> str:
        """Attribute exact observed paths; never suffix-match an owner."""
        relative = filename.removeprefix("/work/source/")
        if relative not in self.select("all").values():
            raise Incomplete("Analysis identifies an unreviewed source file.")
        return relative


def require_unchanged(root: Path, identity: dict[str, str]) -> None:
    """Reject changed, removed, linked or replaced admitted input bytes."""
    if any(PRODUCT.sha256(root / name) != expected for name, expected in identity.items()):
        raise Incomplete("Source, policy or analysis input changed during execution.")


def require_current_candidate(root: Path, identity: dict[str, str]) -> None:
    """Re-admit the complete candidate set so additions cannot evade old hashes."""
    if PRODUCT.admit_product(root)[1] != identity:
        raise Incomplete("The complete candidate input set changed during analysis.")


def coverage_counts(scope: Scope, report: dict, expected_file: str) -> dict:
    """Require nonempty actual parser line data; a repository average cannot pass."""
    filename = scope.selected_file(expected_file)
    files = [item for section in report.get("data", []) for item in section.get("files", [])
             if item.get("filename") == expected_file]
    if len(files) != 1:
        raise Incomplete("Coverage must identify the exact selected parser file once.")
    lines = files[0].get("summary", {}).get("lines", {})
    total, covered = lines.get("count"), lines.get("covered")
    if type(total) is not int or type(covered) is not int or not 0 < total or not 0 <= covered <= total:
        raise Incomplete("Parser coverage has an empty or invalid executable-line denominator.")
    percent = 100 * covered / total
    if percent < 90:
        raise Incomplete("Parser executable-line coverage is below 90 percent.")
    return {"file": filename, "executable_lines": total, "covered_lines": covered,
            "line_percent": percent, "branch_evidence": files[0].get("summary", {}).get("branches")}


def mutation_listing_warning(expected_jobs: int) -> str | None:
    """Return only the pinned warning for the admitted high worker count."""
    jobs = mutation_jobs({"mutation_jobs": expected_jobs})
    return MUTATION_LIST_WARNING.replace("--jobs=12", "--jobs=" + str(jobs)) if jobs > 8 else None


def parse_mutation_listing(output: str, *, expected_jobs=RUNTIME.MAX_JOBS) -> tuple[list, str | None]:
    """Admit one optional exact known warning followed by one strict JSON array.

    Native status and raw output custody remain owned by the session. No other
    diagnostic, duplicate key, nonfinite token or trailing JSON is discarded.
    """
    if not isinstance(output, str):
        raise Incomplete("Mutation listing is not a text result.")
    expected_warning = mutation_listing_warning(expected_jobs)
    warning = expected_warning if expected_warning is not None and output.startswith(expected_warning) else None
    payload = output[len(warning):] if warning else output

    def unique_object(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise ValueError("duplicate JSON object member")
            result[key] = value
        return result

    def invalid_constant(_value):
        raise ValueError("nonfinite JSON number")

    def finite_float(value):
        number = float(value)
        if not math.isfinite(number):
            raise ValueError("JSON number outside finite float range")
        return number

    try:
        items = json.loads(payload, object_pairs_hook=unique_object, parse_constant=invalid_constant,
                           parse_float=finite_float)
    except (ValueError, RecursionError) as error:
        raise Incomplete("Mutation listing is not one complete strict JSON value.") from error
    if not isinstance(items, list):
        raise Incomplete("Mutation listing must be a JSON array.")
    return items, warning


def mutation_inventory(scope: Scope, items: list, maximum: int, expected_file: str) -> dict[str, dict]:
    """Admit a finite, unique full parser enumeration before testing mutants."""
    filename = scope.selected_file(expected_file)
    if not isinstance(items, list) or not 0 < len(items) <= maximum:
        raise Incomplete("Mutation enumeration is empty or exceeds its fixed budget.")
    result = {}
    for item in items:
        if (not isinstance(item, dict) or item.get("file") != filename
                or item.get("package") != scope.package or not isinstance(item.get("name"), str)
                or not item["name"] or item["name"] in result):
            raise Incomplete("Mutation enumeration contains duplicate or out-of-scope entries.")
        result[item["name"]] = item
    return result


def canonical_sha256(value: dict) -> str:
    """Bind complete input/identity dictionaries using one stable JSON encoding."""
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


def admit_reviewed_mutation_inventories(scope: Scope, policy: dict, inventories: dict) -> dict | None:
    """Check an optional complete scope map against every selected actual inventory.

    Hash the name-keyed admitted descriptors, preserving every member except the
    optional top-level rendered diff. No campaign is launched by this check.
    """
    if "mutation_inventories" not in policy:
        return None
    reviewed = policy["mutation_inventories"]
    maximum = policy.get("maximum_mutants")
    scope_names = set(scope.select("all"))
    if (not isinstance(reviewed, dict) or set(reviewed) != scope_names
            or type(maximum) is not int or not 0 < maximum <= 512
            or not isinstance(inventories, dict) or not inventories
            or not set(inventories) <= scope_names):
        raise Incomplete("Reviewed mutation inventory map or scope is invalid.")
    for spec in reviewed.values():
        if (not isinstance(spec, dict) or set(spec) != {"count", "sha256"}
                or type(spec["count"]) is not int or not 0 < spec["count"] <= maximum
                or not isinstance(spec["sha256"], str)
                or not re.fullmatch(r"[a-f0-9]{64}", spec["sha256"])):
            raise Incomplete("Reviewed mutation inventory specification is invalid.")
    admitted = {}
    for name, inventory in inventories.items():
        if not isinstance(inventory, dict) or any(
                not isinstance(item, dict) or item.get("name") != key
                for key, item in inventory.items()):
            raise Incomplete("Observed mutation inventory is invalid.")
        normalized = {key: {field: value for field, value in item.items() if field != "diff"}
                      for key, item in inventory.items()}
        actual = {"count": len(normalized), "sha256": canonical_sha256(normalized)}
        if actual != reviewed[name]:
            raise Incomplete("Observed mutation inventory differs from its reviewed count or hash.")
        admitted[name] = actual
    return admitted


def exact_mutant_span(value) -> bool:
    """Require pinned positive integer source positions, excluding bool/float aliases."""
    return (isinstance(value, dict) and set(value) == {"start", "end"}
            and all(isinstance(value[side], dict) and set(value[side]) == {"line", "column"}
                    and all(type(number) is int and number > 0 for number in value[side].values())
                    for side in ("start", "end"))
            and (value["start"]["line"], value["start"]["column"])
            <= (value["end"]["line"], value["end"]["column"]))


def admit_mutation_equivalences(root: Path, scope: Scope, policy: dict) -> list[dict]:
    """Bind exact reviewed E records to current source, candidate and proof bytes.

    This validates custody and explicit review references, not the mathematical
    substance of a proof. Independent review remains a coordinator obligation.
    Empty E is the default; no pattern, function skip or inferred approval exists.
    """
    entries = policy.get("mutation_equivalences", [])
    if not isinstance(entries, list) or len(entries) > policy.get("maximum_mutants", 512):
        raise Incomplete("Invalid or oversized exact mutation-equivalence inventory.")
    if not entries:
        return []
    root = root.resolve(strict=True)
    candidate = canonical_sha256(PRODUCT.admit_product(root)[1])
    ids, faults = set(), set()
    for entry in entries:
        required = {"id", "candidate_sha256", "source_sha256", "mutant", "requirement", "rationale", "proof"}
        if not isinstance(entry, dict) or set(entry) != required:
            raise Incomplete("An exact equivalence record has missing or unknown fields.")
        identifier, mutant, proof = entry["id"], entry["mutant"], entry["proof"]
        if (not isinstance(identifier, str) or not re.fullmatch(r"[A-Z][A-Z0-9-]{0,63}", identifier)
                or identifier in ids or not isinstance(mutant, dict)
                or set(mutant) != {"name", "package", "file", "function", "span", "replacement", "genre"}
                or mutant.get("package") != scope.package or not isinstance(mutant.get("name"), str)
                or not mutant["name"] or mutant["name"] in faults):
            raise Incomplete("An equivalence identity is missing, duplicated or out of scope.")
        if not isinstance(mutant.get("file"), str):
            raise Incomplete("Equivalence source identity is missing.")
        function = mutant["function"]
        if (not exact_mutant_span(mutant["span"]) or not isinstance(function, dict)
                or set(function) != {"function_name", "return_type", "span"}
                or not exact_mutant_span(function["span"])
                or any(not isinstance(function[key], str) or not 0 < len(function[key]) <= 4096
                       for key in ("function_name", "return_type"))
                or not isinstance(mutant["genre"], str) or not 0 < len(mutant["genre"]) <= 4096
                or not isinstance(mutant["replacement"], str) or len(mutant["replacement"]) > 4096
                # cargo-mutants represents unary-operator deletion with an empty replacement.
                or (not mutant["replacement"] and mutant["genre"] != "UnaryOperator")):
            raise Incomplete("Equivalence lacks its full typed frozen mutant source identity.")
        filename = scope.selected_file(mutant["file"])
        if mutant["file"] != filename:
            raise Incomplete("Equivalence source must use its exact canonical relative path.")
        for key in ("candidate_sha256", "source_sha256"):
            if not isinstance(entry[key], str) or not re.fullmatch(r"[a-f0-9]{64}", entry[key]):
                raise Incomplete("Missing exact equivalence source/candidate SHA-256.")
        source = root / filename
        if (source.resolve(strict=True) != source or PRODUCT.sha256(source) != entry["source_sha256"]
                or entry["candidate_sha256"] != candidate):
            raise Incomplete("Equivalence source or complete candidate differs from its reviewed bytes.")
        for key, maximum in (("requirement", 256), ("rationale", 4096)):
            if not isinstance(entry[key], str) or not 0 < len(entry[key].strip()) <= maximum:
                raise Incomplete("Equivalence lacks bounded requirement-grounded rationale.")
        if (not isinstance(proof, dict) or set(proof) != {"path", "sha256", "section", "reviewer", "verdict"}
                or proof.get("verdict") != "independently-justified-equivalent"
                or proof.get("section") != identifier
                or not isinstance(proof.get("reviewer"), str) or not 0 < len(proof["reviewer"].strip()) <= 128
                or not isinstance(proof.get("path"), str) or not isinstance(proof.get("sha256"), str)
                or not re.fullmatch(r"[a-f0-9]{64}", proof["sha256"])):
            raise Incomplete("Equivalence lacks an exact independent-review proof reference.")
        relative = Path(proof["path"])
        if (relative.is_absolute() or relative.parts[:1] != ("tasks",) or ".." in relative.parts
                or relative.as_posix() != proof["path"] or relative.suffix != ".md"):
            raise Incomplete("Equivalence proof must be a canonical maintained task artifact.")
        artifact = root / relative
        if (artifact.resolve(strict=True) != artifact or artifact.stat().st_size > 65536
                or PRODUCT.sha256(artifact) != proof["sha256"]
                or not re.search(r"^## " + re.escape(identifier) + r"(?:\s|$)", artifact.read_text(), re.MULTILINE)):
            raise Incomplete("Equivalence proof bytes or exact section differ from review.")
        ids.add(identifier); faults.add(mutant["name"])
    return json.loads(json.dumps(entries))


def mutation_phases(scope: Scope, outcome: dict) -> None:
    """Require the pinned Cargo build/test phases and matching native statuses."""
    summary = outcome.get("summary")
    expected = (["Build"] if summary == "Unviable" else ["Build", "Test"])
    phases = outcome.get("phase_results")
    if (not isinstance(phases, list) or any(not isinstance(phase, dict) for phase in phases)
            or [phase.get("phase") for phase in phases] != expected):
        raise Incomplete("Mutation outcome lacks its exact actual build/test phases.")
    observed_format = None
    for phase in phases:
        arguments = ["cargo", "test", *(["--no-run"] if phase["phase"] == "Build" else []),
                     "--verbose", "--package=" + scope.package + "@0.0.0", "--frozen"]
        json_arguments = [*arguments[:-1], "--message-format=json", arguments[-1]]
        actual_format = ("human" if phase.get("argv") == arguments else
                         "json" if phase.get("argv") == json_arguments else None)
        if actual_format is None or (observed_format is not None and actual_format != observed_format):
            raise Incomplete("Mutation phases do not use one exact admitted Cargo message format.")
        observed_format = actual_format
        failed = summary == "Unviable" or (summary == "CaughtMutant" and phase["phase"] == "Test")
        expected_status = {"Failure": 101} if failed else "Success"
        duration = phase.get("duration")
        status = phase.get("process_status")
        if (status != expected_status
                or (failed and (not isinstance(status, dict) or type(status.get("Failure")) is not int))
                or type(duration) not in (int, float) or not math.isfinite(duration) or duration < 0):
            raise Incomplete("Mutation summary, selected Cargo phase or native phase result disagree.")


def mutation_counts(scope: Scope, report: dict, expected: dict[str, dict], expected_file: str,
                    *, native_exit_code: int = 0, equivalences=(), defer_gate: bool = False) -> dict:
    """Reconcile complete K/M/U/T/E data; defer only a complete custom-gate failure.

    Malformed data, timeouts, failed baselines and native-code mismatches are
    immediately incomplete. A deferred result always reports its failing gate;
    orchestration must collect remaining files then reject the aggregate.
    """
    if not isinstance(report, dict) or not isinstance(expected, dict) or not isinstance(equivalences, (list, tuple)):
        raise Incomplete("Mutation report, inventory or exact equivalence list has an invalid shape.")
    filename = scope.selected_file(expected_file)
    if not expected or any(item.get("file") != filename for item in expected.values()):
        raise Incomplete("Mutation expectations do not cover the selected parser file.")
    outcomes = report.get("outcomes", [])
    if not isinstance(outcomes, list) or any(not isinstance(item, dict) for item in outcomes):
        raise Incomplete("Mutation outcomes must be a complete admitted report list.")
    baseline = [item for item in outcomes if item.get("scenario") == "Baseline"]
    if (report.get("cargo_mutants_version") != "27.1.0" or len(baseline) != 1
            or baseline[0].get("summary") != "Success"):
        raise Incomplete("Mutation campaign lacks its successful actual baseline.")
    mutation_phases(scope, baseline[0])
    counts = {"CaughtMutant": 0, "MissedMutant": 0, "Unviable": 0, "Timeout": 0}
    observed, missed = set(), {}
    for item in outcomes:
        if item in baseline:
            continue
        scenario = item.get("scenario")
        mutant = scenario.get("Mutant") if isinstance(scenario, dict) else None
        if (not isinstance(mutant, dict) or mutant.get("file") != filename
                or mutant.get("package") != scope.package or mutant.get("name") not in expected
                or mutant["name"] in observed or item.get("summary") not in counts):
            raise Incomplete("Mutation outcome is missing, duplicated, unclassified or out of scope.")
        # List output optionally adds a rendered diff. The actual fault's name,
        # source span, replacement and remaining metadata must match unchanged.
        frozen = {key: value for key, value in expected[mutant["name"]].items() if key != "diff"}
        if canonical_sha256(mutant) != canonical_sha256(frozen):
            raise Incomplete("Mutation outcome differs from its complete frozen fault identity.")
        observed.add(mutant["name"])
        counts[item["summary"]] += 1
        if item["summary"] == "Timeout":
            raise Incomplete("Mutation qualification has unresolved native timeouts.")
        mutation_phases(scope, item)
        if item["summary"] == "MissedMutant":
            missed[mutant["name"]] = mutant
    if (observed != set(expected) or type(report.get("total_mutants")) is not int
            or report["total_mutants"] != len(expected)):
        raise Incomplete("Mutation outcomes do not cover the complete frozen enumeration.")
    for key, symbol in (("caught", "CaughtMutant"), ("missed", "MissedMutant"),
                        ("unviable", "Unviable"), ("timeout", "Timeout")):
        if type(report.get(key)) is not int or report[key] != counts[symbol]:
            raise Incomplete("Mutation summary counters disagree with complete individual outcomes.")
    if type(report.get("success")) is not int or report["success"] != 0:
        raise Incomplete("Mutation report contains an unclassified successful mutant.")
    expected_exit = 2 if missed else 0
    if type(native_exit_code) is not int or native_exit_code != expected_exit:
        raise Incomplete("Mutation native exit disagrees with its complete missed-mutant report.")
    excluded, excluded_names = [], set()
    for entry in equivalences:
        if (not isinstance(entry, dict) or not isinstance(entry.get("mutant"), dict)
                or entry["mutant"].get("file") != filename):
            raise Incomplete("Equivalence refers to a different critical source file.")
        mutant = entry["mutant"]; name = mutant.get("name")
        if (name in excluded_names or name not in missed
                or canonical_sha256(missed[name]) != canonical_sha256(mutant)):
            raise Incomplete("Equivalence is duplicated, stale, changed or not an actual missed mutant.")
        excluded.append(entry); excluded_names.add(name)
    viable = counts["CaughtMutant"] + counts["MissedMutant"]
    if not viable:
        raise Incomplete("Mutation qualification has no viable denominator or unresolved timeouts.")
    adjusted_denominator = viable - len(excluded)
    if not adjusted_denominator:
        raise Incomplete("Adjusted mutation denominator is empty.")
    percent = 100 * counts["CaughtMutant"] / viable
    adjusted = 100 * counts["CaughtMutant"] / adjusted_denominator
    unresolved = sorted(set(missed) - excluded_names)
    passed = adjusted >= 95 and not unresolved
    if not passed and not defer_gate:
        raise Incomplete("Mutation has unresolved survivors or an adjusted kill score below 95 percent.")
    return {"file": filename, "generated": len(expected), "viable": viable,
            "killed": counts["CaughtMutant"], "survived": counts["MissedMutant"],
            "timed_out": counts["Timeout"], "unviable": counts["Unviable"],
            "excluded": len(excluded), "excluded_cases": excluded,
            "raw_viable_killed_percent": percent, "viable_killed_percent": percent,
            "raw_denominator": viable, "adjusted_denominator": adjusted_denominator,
            "adjusted_viable_killed_percent": adjusted, "unresolved_survivors": unresolved,
            "native_exit_code": native_exit_code, "gate_passed": passed}


def mutation_jobs(policy: dict) -> int:
    """Admit an exact bounded mutation-worker count, defaulting to twelve."""
    value = policy.get("mutation_jobs", RUNTIME.MAX_JOBS)
    if type(value) is not int or not 1 <= value <= RUNTIME.MAX_JOBS:
        raise Incomplete("Mutation jobs must be an integer between 1 and 12.")
    return value


def mutation_process_limit(policy: dict) -> int:
    """Admit a fixed mutation-only task profile, preserving the 128 default."""
    value = policy.get("mutation_process_limit", 128)
    if type(value) is not int or value not in (128, 512):
        raise Incomplete("Mutation process limit must be exactly 128 or 512 tasks.")
    return value


def mutation_message_format(policy: dict) -> str:
    """Admit the reviewed Cargo diagnostic formats, defaulting to human."""
    value = policy.get("mutation_message_format", "human")
    if type(value) is not str or value not in ("human", "json"):
        raise Incomplete("Unknown mutation Cargo message format.")
    return value


def mutation_campaign_command(scope: Scope, tools: Path, subject: str, command: list,
                              *, expected_jobs=RUNTIME.MAX_JOBS, expected_message_format="human") -> None:
    """Reject command substitution before a mutation subprocess may be launched."""
    jobs = mutation_jobs({"mutation_jobs": expected_jobs})
    message_format = mutation_message_format({"mutation_message_format": expected_message_format})
    filename = scope.select(subject)[subject]
    expected = [str(tools / "cargo-mutants"), "mutants", "--package", scope.package, "--file", filename,
                "--test-workspace", "false", "--cap-lints", "false", "--jobs", str(jobs),
                "--jobserver", "true", "--jobserver-tasks", str(jobs)]
    if message_format == "json":
        expected.append("--cargo-arg=--message-format=json")
    expected.extend(["--baseline", "run", "--build-timeout", "90", "--timeout", "60",
                     "--output", "/work/mutation/" + subject])
    if command != expected:
        raise Incomplete("Mutation command is not the exact admitted campaign invocation.")


def mutation_campaign_result(scope: Scope, tools: Path, subject: str, command: list, result: dict,
                             *, expected_jobs=RUNTIME.MAX_JOBS, expected_message_format="human") -> dict:
    """Retain provisional native 0/2 only for complete-report reconciliation.

    Exit 2 is never a command success. Setup/errors/baseline/timeout/signal
    statuses remain immediate incomplete results.
    """
    mutation_campaign_command(scope, tools, subject, command, expected_jobs=expected_jobs,
                              expected_message_format=expected_message_format)
    if (not isinstance(result, dict) or not {"exit_code", "failure", "output"} <= result.keys()
            or not isinstance(result["output"], str) or result["failure"] is not None
            or type(result.get("exit_code")) is not int or result["exit_code"] not in (0, 2)):
        raise Incomplete("Mutation command failed or is not the exact admitted campaign invocation.")
    return result


def combined_mutant_count(inventories: dict[str, dict], maximum: int) -> int:
    """Cap the complete selected enumeration before any mutation campaign starts."""
    total = sum(len(inventory) for inventory in inventories.values())
    if not inventories or not 0 < total <= maximum:
        raise Incomplete("Combined parser mutation enumeration exceeds its fixed budget or is empty.")
    return total


def remaining_timeout(deadline: float, maximum: int) -> int:
    """Keep all selected mutation commands inside one unchanged wall-time budget."""
    remaining = int(deadline - time.monotonic())
    if remaining <= 0:
        raise Incomplete("The shared parser mutation wall-time budget is exhausted.")
    return min(maximum, remaining)


def test_count(output: str) -> int:
    """Require actual nonempty libtest execution without ignored cases."""
    counts = re.findall(r"test result: ok\. (\d+) passed; 0 failed; (\d+) ignored;", output)
    if not counts or any(int(ignored) for _, ignored in counts) or not sum(int(n) for n, _ in counts):
        raise Incomplete("Analysis ran no observed passing tests or ignored required cases.")
    return sum(int(n) for n, _ in counts)


def tool_path(root: Path, name: str) -> Path:
    """Select only the installed hash-pinned extension; never acquire a tool."""
    found = shutil.which(name)
    if not found:
        raise Incomplete(f"Missing installed analysis extension: {name}.")
    path = Path(found).resolve(strict=True)
    entry = next(item for item in json.loads((root / "policy/tool-manifest.json").read_text())["tools"]
                 if item["name"] == name)
    PRODUCT.ENV.require_admitted_binary(path, entry["binary_sha256"], name)
    return path


def privacy_error(result: dict, expected: str) -> None:
    """Require the intended compiler privacy diagnostic, not any compile failure."""
    codes = set(re.findall(r"error\[(E\d+)\]", result.get("output", "")))
    if (result.get("failure") or result.get("exit_code") == 0 or codes != {expected}):
        raise Incomplete("External consumer did not fail for its intended privacy restriction.")


def fuzz_input_snapshot(work: Path, inputs: list[Path]) -> dict:
    """Admit only the exact owned fuzz Cargo config, retaining ancestry guards.

    The bootstrap snapshot helper deliberately forbids every Cargo-home config;
    this product caller needs one fixed, sealed vendor mapping. Inspect that
    exception explicitly, then reuse the unchanged helper on each input island.
    All other work/home configuration remains forbidden and aggregate bounds stay
    equal to the bootstrap helper's limits.
    """
    if len(inputs) != 2 or set(inputs) != {work / "fuzz", work / "fuzz-vendor"}:
        raise Incomplete("The complete pair of fuzz input islands is required.")
    for unexpected in (work / ".cargo", work / "controls/.cargo", work / "home/.cargo",
                       work / "cargo-home/config"):
        if unexpected.exists() or unexpected.is_symlink():
            raise Incomplete("Unexpected Cargo ancestry/home configuration in fuzz work.")
    config = work / "cargo-home/config.toml"
    if ANALYSIS.read_once(config, maximum=4096) != FUZZ_CARGO_CONFIG.encode():
        raise Incomplete("The owned fuzz Cargo-home configuration differs from its fixed admission.")
    result = {"cargo-home/config.toml": {"kind": "file", "bytes": config.stat().st_size,
                                         "sha256": ANALYSIS.digest(config)}}
    for island in inputs:
        if island not in (work / "fuzz", work / "fuzz-vendor") or island.parent != work:
            raise Incomplete("Unexpected fuzz input island.")
        for relative, entry in ANALYSIS.input_snapshot(island, [island]).items():
            key = (Path(island.name) / relative).as_posix()
            if key in result:
                raise Incomplete("Duplicate fuzz input island entry.")
            result[key] = entry
    if len(result) > 100000 or sum(entry.get("bytes", 0) for entry in result.values()) > 2 * 1024 ** 3:
        raise Incomplete("Fuzz input snapshot exceeds aggregate entry/byte bounds.")
    return result


def prepare_fuzz_vendor(original: Path, archive: Path, package: dict, destination: Path) -> None:
    """Prepare one new owned package; reject source/archive drift before publication."""
    cached = PRODUCT.inventory(original)
    # These are Cargo-generated cache records, not published package source.
    source_files = {name: digest for name, digest in cached.items()
                    if name not in {".cargo-ok", ".cargo-checksum.json"}}
    specification = {**package, "files_sha256": PREPARE.files_digest(source_files)}
    data = ANALYSIS.read_once(archive, maximum=PREPARE.MAX_ARCHIVE)
    files = PREPARE.archive_files(data, specification)
    destination.mkdir(mode=0o700)
    for name, contents in files.items():
        target = destination / name
        target.parent.mkdir(parents=True, exist_ok=True, mode=0o700)
        target.write_bytes(contents)
    checksums = {"files": {name: PREPARE.sha(contents) for name, contents in files.items()},
                 "package": package["checksum"]}
    (destination / ".cargo-checksum.json").write_text(json.dumps(checksums, sort_keys=True) + "\n")


def library_artifact(output: str, package: str, manifest: str) -> str:
    """Select one real Cargo-emitted rlib for the exact local manifest/target."""
    expected_id = "path+file://" + str(Path(manifest).parent) + "#0.0.0"
    found = []
    for line in output.splitlines():
        try:
            item = json.loads(line)
        except json.JSONDecodeError:
            continue
        if item.get("reason") != "compiler-artifact" or item.get("manifest_path") != manifest:
            continue
        target = item.get("target", {})
        if (item.get("package_id") != expected_id or target.get("name") != package.replace("-", "_")
                or target.get("kind") != ["lib"]):
            raise Incomplete("Cargo artifact has the wrong local identity or target.")
        libraries = [name for name in item.get("filenames", []) if name.endswith(".rlib")]
        if len(libraries) != 1:
            raise Incomplete("Cargo artifact has no unique emitted library.")
        path = Path(libraries[0])
        if not path.is_absolute() or ".." in path.parts or not path.is_relative_to("/work/target"):
            raise Incomplete("Cargo library artifact escapes owned build output.")
        found.append(str(path))
    if len(found) != 1:
        raise Incomplete("Cargo did not attribute exactly one actual local library artifact.")
    return found[0]


def compiler_cause(result: dict, specimen: dict, source_path: str) -> None:
    """Require exact error codes and a named cause at the intended source line."""
    diagnostics = []
    for line in result.get("output", "").splitlines():
        try:
            item = json.loads(line)
        except json.JSONDecodeError:
            continue
        if isinstance(item, dict) and item.get("level") == "error" and item.get("code"):
            diagnostics.append(item)
    if (result.get("failure") or result.get("exit_code") == 0 or not diagnostics
            or {item.get("code", {}).get("code") for item in diagnostics} != {specimen["code"]}):
        raise Incomplete("External consumer failed without its intended compiler code.")
    def text(item):
        return (item.get("message", "") + " "
                + " ".join(span.get("label") or "" for span in item.get("spans", [])) + " "
                + " ".join(text(child) for child in item.get("children", [])))
    for item in diagnostics:
        if (all(token in text(item) for token in specimen["tokens"])
                and any(span.get("is_primary") and span.get("file_name") == source_path
                        and span.get("line_start") == specimen["line"] for span in item.get("spans", []))):
            return
    raise Incomplete("External consumer diagnostic has an unrelated type, field or primary span.")


class MutationEnumeration(NamedTuple):
    """Caller-owned listing facts, not baseline, campaign or review admission."""
    inventories: dict[str, dict]
    commands: dict[str, list[str]]
    total: int
    observations: dict[str, dict]
    campaign_budget_exceeded: bool


def enumerate_mutation_inventories(scope, subjects, policy, tools, run, report, *,
                                   deadline, maximum_per_file, maximum_total) -> MutationEnumeration:
    """List each exact selected owner once through a source-bound custodian.

    The caller admits source/graph/tool bytes and supplies the unchanged absolute
    wall deadline plus explicit finite listing allowances. Each command contains
    only `--list --json`, with at most 120 seconds remaining from that deadline;
    this helper launches neither a baseline nor a campaign and creates no files.
    Raw output and warning observations remain in `report` even on parse failure.

    Listing discovery may reserve a larger aggregate than the current campaign
    cap. That condition is reported, never converted into campaign admission.
    Reviewed hashes, equivalences, passing baselines and full resource reservation
    remain separate pre-campaign obligations. Returned dictionaries belong to
    the caller and must stay unchanged while those obligations are reconciled.
    """
    allowed = scope.select("all")
    if (not isinstance(subjects, dict) or not subjects
            or any(name not in allowed or allowed[name] != filename for name, filename in subjects.items())):
        raise Incomplete("Mutation listing subjects differ from the exact admitted scope.")
    campaign_maximum = policy.get("maximum_mutants")
    try:
        finite_deadline = type(deadline) in (int, float) and math.isfinite(deadline)
    except OverflowError:
        finite_deadline = False
    if (type(maximum_per_file) is not int or not 0 < maximum_per_file <= 512
            or type(maximum_total) is not int or not 0 < maximum_total <= maximum_per_file * len(subjects)
            or type(campaign_maximum) is not int or not 0 < campaign_maximum <= 512
            or not finite_deadline):
        raise Incomplete("Mutation listing requires explicit finite allowances and a finite deadline.")
    workers = mutation_jobs(policy)
    mutation_process_limit(policy)
    message_format = mutation_message_format(policy)
    inventories, commands = {}, {}
    observations = report["mutation_enumeration"] = {}
    report.pop("mutation_enumerated_total", None)
    report.pop("mutation_campaign_budget_exceeded", None)
    for name, filename in subjects.items():
        # Bound independent mutant copies and the GNU jobserver separately.
        common = [str(tools / "cargo-mutants"), "mutants", "--package", scope.package, "--file", filename,
                  "--test-workspace", "false", "--cap-lints", "false", "--jobs", str(workers),
                  "--jobserver", "true", "--jobserver-tasks", str(workers)]
        if message_format == "json":
            common.append("--cargo-arg=--message-format=json")
        commands[name] = common
        listed = run("mutation-enumeration:" + name, [*common, "--list", "--json"],
                     remaining_timeout(deadline, 120))
        expected_warning = mutation_listing_warning(workers)
        observed_warning = (expected_warning if expected_warning is not None
                            and isinstance(listed, str) and listed.startswith(expected_warning) else None)
        observations[name] = {"raw_output": listed, "warning": observed_warning}
        items, warning = parse_mutation_listing(listed, expected_jobs=workers)
        if warning != observed_warning:
            raise Incomplete("Mutation listing warning observation disagrees with parser admission.")
        inventories[name] = mutation_inventory(scope, items, maximum_per_file, filename)
    total = combined_mutant_count(inventories, maximum_total)
    report["mutation_enumerated_total"] = total
    exceeded = total > campaign_maximum
    report["mutation_campaign_budget_exceeded"] = exceeded
    return MutationEnumeration(inventories, commands, total, observations, exceeded)


def run_analysis_lanes(scope, subjects, policy, work, tools, run, report, lane):
    """Execute finite coverage and complete per-file mutation through a supplied custodian."""
    if lane in {"mutation", "all"}:
        mutation_jobs(policy)
        mutation_process_limit(policy)
        mutation_message_format(policy)
    if lane in {"coverage", "all"}:
        command = [str(tools / "cargo-llvm-cov"), "llvm-cov", "--package", scope.package,
                   "--frozen", "--jobs", str(RUNTIME.MAX_JOBS), "--lib", "--tests", "--json",
                   "--output-path", "/work/coverage.json", "--failure-mode", "any",
                   "--ignore-filename-regex", "(^|/)tests/", "--fail-under-file-lines", "90"]
        text = run("coverage", command, policy["coverage_wall_seconds"])
        coverage = json.loads((work / "coverage.json").read_text())
        executed = test_count(text)
        report["coverage"] = {
            name: {**coverage_counts(scope, coverage, "/work/source/" + filename),
                   "tests_executed": executed, "shared_execution": "coverage"}
            for name, filename in subjects.items()}
        print("PASSED: portable coverage", flush=True)
    if lane in {"mutation", "all"}:
        equivalences = admit_mutation_equivalences(work / "source", scope, policy)
        (work / "mutation").mkdir()
        deadline = time.monotonic() + policy["mutation_wall_seconds"]
        enumerated = enumerate_mutation_inventories(
            scope, subjects, policy, tools, run, report, deadline=deadline,
            maximum_per_file=policy["maximum_mutants"], maximum_total=policy["maximum_mutants"])
        inventories, commands = enumerated.inventories, enumerated.commands
        report["mutation_enumerated_total"] = combined_mutant_count(inventories, policy["maximum_mutants"])
        reviewed = admit_reviewed_mutation_inventories(scope, policy, inventories)
        if reviewed is not None:
            report["mutation_reviewed_inventories"] = reviewed
        for entry in equivalences:
            filename, mutant = entry["mutant"]["file"], entry["mutant"]
            selected_name = next((name for name, selected in subjects.items() if selected == filename), None)
            if selected_name is not None:
                actual = inventories[selected_name].get(mutant["name"], {})
                if canonical_sha256({key: value for key, value in actual.items() if key != "diff"}) != canonical_sha256(mutant):
                    raise Incomplete("Equivalence does not match the complete frozen mutation enumeration.")
        report["mutation"] = {}
        for name, filename in subjects.items():
            campaign = run("mutation-campaign:" + name,
                [*commands[name], "--baseline", "run", "--build-timeout", "90", "--timeout", "60",
                 "--output", "/work/mutation/" + name], remaining_timeout(deadline, policy["mutation_wall_seconds"]),
                mutation_subject=name)
            outcome_directory = work / "mutation" / name / "mutants.out"
            outcomes = json.loads((outcome_directory / "outcomes.json").read_text())
            selected_equivalences = [entry for entry in equivalences if entry["mutant"]["file"] == filename]
            result = mutation_counts(scope, outcomes, inventories[name], filename,
                                     native_exit_code=campaign["exit_code"], equivalences=selected_equivalences,
                                     defer_gate=True)
            baseline = next(item for item in outcomes["outcomes"] if item["scenario"] == "Baseline")
            relative_log = Path(baseline["log_path"])
            if relative_log.is_absolute() or ".." in relative_log.parts:
                raise Incomplete("Mutation baseline log escapes its owned output directory.")
            baseline_log = outcome_directory / relative_log
            PRODUCT.sha256(baseline_log)
            result["baseline_tests_executed"] = test_count(baseline_log.read_text())
            report["mutation"][name] = result
        failed = [name for name, result in report["mutation"].items() if not result["gate_passed"]]
        if failed:
            raise Incomplete("Complete mutation accounting has unresolved custom gates: " + ", ".join(failed))
        print("PASSED: portable mutation", flush=True)

class Session:
    """One immutable candidate, admitted graph/tools and private bounded work tree."""

    def __init__(self, root, args, report, scope, policy, extra_inputs):
        self.root, self.report, self.scope, self.policy = Path(root), report, scope, policy
        self.dependency_policy, self.product_identity = PRODUCT.admit_product(self.root)
        self.identity = dict(self.product_identity)
        equivalences = admit_mutation_equivalences(self.root, scope, policy)
        proof_inputs = tuple(entry["proof"]["path"] for entry in equivalences)
        names = (scope.policy, "scripts/portable-analysis.py",
                 "scripts/prepare-product.py", "scripts/analysis-runtime.py", "scripts/check-analysis.py",
                 "policy/analysis-tools.json", "policy/verification-manifest.toml", *extra_inputs, *proof_inputs)
        self.identity.update({name: PRODUCT.sha256(self.root / name) for name in names})
        self.admitted = ANALYSIS.admit(self.root)
        self.base, self.stable = self.admitted["base"], self.admitted["stable"]
        self.vendor = args.vendor_dir.resolve(strict=True)
        self.vendor_identity = PRODUCT.admit_vendor(self.vendor, self.dependency_policy)
        self.output = args.output_dir
        if (not self.output.is_absolute() or self.output.parent != self.output.parent.resolve(strict=True)
                or self.output.is_relative_to(self.root) or self.output.is_relative_to(Path("/mnt"))):
            raise Incomplete("Use a fresh external canonical private Linux output directory.")
        if shutil.disk_usage(self.output.parent).free < 2 * 1024 ** 3:
            raise Incomplete("Portable analysis requires at least 2 GiB free Linux work space.")
        self.output.mkdir(mode=0o700)
        report["owned_output"] = str(self.output)
        self.work = self.output / "work"; self.work.mkdir(mode=0o700)
        self.source = self.work / "source"; self.source.mkdir(mode=0o700)
        for name in self.identity:
            destination = self.source / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(self.root / name, destination)
        require_unchanged(self.source, self.identity)
        self.source_inventory = PRODUCT.inventory(self.source)
        mutations = tomllib.loads((self.source / ".cargo/mutants.toml").read_text())
        if mutations.get("additional_cargo_args") != ["--frozen"] or mutations.get("cap_lints") is not False:
            raise Incomplete("Mutation requires unchanged frozen Cargo and lint policy.")
        self.tools = self.output / "tools"; self.tools.mkdir(mode=0o700)
        selected = (("cargo-llvm-cov", "cargo-mutants") if args.lane == "all" else
                    ("cargo-llvm-cov",) if args.lane == "coverage" else
                    ("cargo-mutants",) if args.lane == "mutation" else ())
        self.protected_tools = {}
        for name in selected:
            origin = tool_path(self.root, name)
            shutil.copyfile(origin, self.tools / name); (self.tools / name).chmod(0o500)
            if PRODUCT.sha256(origin) != PRODUCT.sha256(self.tools / name):
                raise Incomplete("Analysis tool changed while its sealed copy was prepared.")
        self.protected_tools[self.tools] = PRODUCT.inventory(self.tools)
        (self.work / "cargo-home").mkdir(mode=0o700)
        self.configure('[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory='
                       + json.dumps(str(self.vendor)) + '\n')
        self.protected_islands = {}
        report.update(source_sha256=self.identity, analysis_policy_sha256=self.admitted["policy_sha256"],
                      tools_sha256=self.protected_tools[self.tools], lanes=[], work=str(self.work))

    def configure(self, text):
        """Publish exactly the caller-reviewed sealed vendor mapping between lanes."""
        self.config = self.work / "cargo-home/config.toml"
        self.config.write_text(text)
        self.config_sha256 = PRODUCT.sha256(self.config)

    def protect(self, path):
        """Retain complete immutable copied dependency/input identity."""
        self.protected_islands[path] = PRODUCT.inventory(path)

    def check(self):
        """Re-admit complete candidate plus every copied sealed input island."""
        require_unchanged(self.root, self.identity)
        require_current_candidate(self.root, self.product_identity)
        if PRODUCT.inventory(self.source) != self.source_inventory:
            raise Incomplete("Copied source inventory changed during analysis.")
        if PRODUCT.sha256(self.config) != self.config_sha256:
            raise Incomplete("Owned Cargo configuration changed during analysis.")
        for path, expected in {**self.protected_tools, **self.protected_islands}.items():
            if PRODUCT.inventory(path) != expected:
                raise Incomplete("A sealed tool, dependency or input island changed.")

    def run(self, label, command, timeout, *, sealed=(), compiler=None, extra_tools=None,
            mode="ordinary", negative=None, expected_error=None, cwd=None, specimen=None, mutation_subject=None,
            sanitizer_profile="legacy"):
        """Keep command execution under unchanged admission, namespace and resource custody."""
        self.check()
        if mutation_subject is not None:
            if (label != "mutation-campaign:" + mutation_subject or specimen or expected_error or negative
                    or compiler is not None or extra_tools is not None or mode != "ordinary" or cwd is not None
                    or sanitizer_profile != "legacy"):
                raise Incomplete("Mutation campaign cannot borrow another execution mode or failure handler.")
            mutation_campaign_command(self.scope, self.tools, mutation_subject, command,
                                      expected_jobs=mutation_jobs(self.policy),
                                      expected_message_format=mutation_message_format(self.policy))
        compiler = compiler or self.stable
        night = self.base / self.admitted["policy"]["common_nightly"]
        if compiler not in (self.stable, night):
            raise Incomplete("Unadmitted analysis compiler.")
        if extra_tools is not None and extra_tools not in self.protected_tools:
            raise Incomplete("Unadmitted extra tool island.")
        # Mutants parallelize independent copies; each nested Cargo/test process
        # gets one slot rather than multiplying twelve workers by twelve jobs.
        inner_jobs = 1 if label.startswith("mutation-") else RUNTIME.MAX_JOBS
        environment = {"RUSTC": str(compiler / "bin/rustc"), "RUSTDOC": str(compiler / "bin/rustdoc"),
                       "RUST_TEST_THREADS": str(inner_jobs)}
        if compiler != self.stable:
            environment.update(RUSTUP_HOME=str(self.base / "common/rustup"), RUSTUP_TOOLCHAIN=ANALYSIS.NIGHTLY)
        readonly = [self.base, self.stable, self.vendor, self.tools, *([extra_tools] if extra_tools else [])]
        paths = [*([extra_tools] if extra_tools else []), compiler / "bin", self.tools]
        islands = []
        for path in [self.source, self.config, *self.protected_islands, *sealed]:
            if any(path == other or path.is_relative_to(other) for other in islands):
                continue
            islands = [other for other in islands if not other.is_relative_to(path)]
            islands.append(path)
        profile_options = {} if sanitizer_profile == "legacy" else {"sanitizer_profile": sanitizer_profile}
        if mutation_subject is not None:
            # Each copied mutant worker must own its Cargo artifacts.
            profile_options["cargo_target_mode"] = "worker"
            profile_options["process_limit"] = mutation_process_limit(self.policy)
        result = RUNTIME.run(self.root, self.work, command, readonly=readonly,
                             sealed=islands, path_dirs=paths,
                             cwd=cwd or self.source, environment=environment, mode=mode,
                             timeout=timeout, cpu_seconds=min(timeout, 600), build_jobs=inner_jobs,
                             **profile_options)
        self.report["lanes"].append({"lane": label, "command": command, **result})
        (self.output / "progress.json").write_text(json.dumps(self.report, indent=2) + "\n")
        self.check()
        if mutation_subject is not None:
            return mutation_campaign_result(self.scope, self.tools, mutation_subject, command, result,
                                            expected_jobs=mutation_jobs(self.policy),
                                            expected_message_format=mutation_message_format(self.policy))
        if specimen:
            compiler_cause(result, specimen, command[command.index("--error-format=json") + 1])
            return result["output"]
        if expected_error:
            privacy_error(result, expected_error)
            return result["output"]
        if negative:
            if result["failure"] or result["exit_code"] == 0 or negative not in result["output"]:
                raise Incomplete("Seeded control did not reach its intended actual failure.")
            return result["output"]
        if label.startswith("mutation-enumeration:") or label == "external-legal-execution":
            # Strict counter/receipt records need the child's actual delimiters.
            # Admission still requires a successful native result before exposure.
            PRODUCT.ENV.require_ok(result)
            return result["output"]
        return PRODUCT.ENV.require_ok(result)

    def finish(self):
        """Full source/vendor/tool/payload admission is required after every selected lane."""
        self.check()
        if PRODUCT.admit_vendor(self.vendor, self.dependency_policy) != self.vendor_identity:
            raise Incomplete("Vendor bytes changed during portable qualification.")
        after = ANALYSIS.admit(self.root)
        if any(after[key] != self.admitted[key] for key in ("base", "stable", "policy_sha256", "inventories")):
            raise Incomplete("Analysis installation/payload identities changed during execution.")
