#!/usr/bin/env python3
"""Measure selected portable service fixture owners (GPL-2.0-only).

The fixture runner owns tool admission and isolated execution. This module reuses
only existing pure metric parsers and evidence export; it changes no shared gate.
Windows execution, installation, networking and formal work are not lanes here.
"""
from __future__ import annotations

import sys

if sys.flags.optimize:
    raise SystemExit("Fixture metrics refuse optimized Python.")
sys.dont_write_bytecode = True

import argparse
from contextlib import contextmanager
import fcntl
import hashlib
import importlib.util
import json
import math
import os
from pathlib import Path, PurePosixPath
import re
import resource
import shutil
import signal
import stat
import time
from typing import NamedTuple

HERE = Path(__file__).resolve().parent


def load(name, path):
    """Import a repository helper without invoking its command-line entry point."""
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


Q = load("fixture_metric_qualification", HERE / "qualification.py")
C = Q.C
E = load("fixture_metric_export", Q.ROOT / "scripts/run-service-analysis.py")
Incomplete = C.Incomplete
PACKAGE = "nevrotick-windows-service-fixture"
LIBRARY = "nevrotick_windows_service_fixture"
FIXTURE_TARGETS = {LIBRARY: ("lib", "src/lib.rs"), "entrypoints": ("test", "tests/entrypoints.rs"),
                   "nevrotick-lab-s02-engine": ("bin", "src/bin/engine.rs"),
                   "nevrotick-lab-s02-broker": ("bin", "src/bin/broker.rs")}
NATIVE_SUBJECTS = {"host": "src/host.rs", "error": "src/error.rs", "diagnostics": "src/diagnostics.rs"}
SUBJECTS = {**NATIVE_SUBJECTS, "engine_worker": "src/engine_worker.rs",
            "diagnostics_thread": "src/diagnostics_thread.rs"}
CLOCK_SUBJECTS = {"broker_clock_change": "src/broker_clock_change.rs",
                  "broker_clock_continuity": "src/broker_clock_continuity.rs"}
DIRECT_SUBJECTS = SUBJECTS | CLOCK_SUBJECTS
COVERAGE_LANES = {"coverage": SUBJECTS, "broker-clock-coverage": CLOCK_SUBJECTS}
SCOPE = C.Scope(PACKAGE, tuple(NATIVE_SUBJECTS.items()), "fixture-local")


class DirectOwner(NamedTuple):
    """Bind a configured direct lane to one exact fixture source and schema."""

    prefix: str
    subject: str
    schema: str

    @property
    def file(self) -> str:
        return DIRECT_SUBJECTS[self.subject]

    @property
    def path(self) -> str:
        return Q.TOOL + "/" + self.file

    @property
    def leaf(self) -> str:
        return PurePosixPath(self.file).name


WORKER_DIRECT = DirectOwner("worker", "engine_worker", "fixture-worker-direct-replay-v1")
WRITER_DIRECT = DirectOwner("writer", "diagnostics_thread", "fixture-writer-direct-replay-v1")
CHANGE_DIRECT = DirectOwner("broker-clock-change", "broker_clock_change",
                            "fixture-broker-clock-change-direct-replay-v1")
CONTINUITY_DIRECT = DirectOwner("broker-clock-continuity", "broker_clock_continuity",
                                "fixture-broker-clock-continuity-direct-replay-v1")
DIRECT_LANES = {f"{owner.prefix}-{suffix}": owner
                for owner in (WORKER_DIRECT, WRITER_DIRECT, CHANGE_DIRECT, CONTINUITY_DIRECT)
                for suffix in ("enumerate", "replay-pilot", "replay")}
METRIC_LANES = (*COVERAGE_LANES, "enumerate", "mutation", *DIRECT_LANES)
WORKER_ROUND_ASSERTIONS = {
    "engine_worker::tests::address_round::" + name: "src/engine_worker_round_tests.rs"
    for name in (
        "managed_round_fallback_retains_source_and_join_with_stale_and_stopped_neighbors",
        "managed_round_expired_service_cutoff_refuses_without_entropy_or_traffic",
        "round_health_keeps_bounded_dispositions_separate_from_operation_failures",
    )
}
WRITER_ASSERTIONS = {
    "diagnostics::threaded::queue_control::one_queued_original_obeys_the_drain_cutoff":
        "src/diagnostics_thread_tests.rs",
    "host::tests::worker_tests::threaded_writer_controls::actual_writer_final_wake_retains_physical_join":
        "src/host_tests.rs",
    "host::tests::worker_tests::threaded_writer_controls::refused_startup_never_starts_the_writer_backend":
        "src/host_tests.rs",
}
CLOCK_TRANSITION_ASSERTIONS = {
    "broker_clock_change::continuity::tests::" + name: "src/broker_clock_continuity_tests.rs"
    for name in ("adjustment_change_retires_the_original_observation",
                 "presentation_change_retires_the_original_observation")
}
CLOCK_ASSERTIONS = {
    CHANGE_DIRECT: {
        **CLOCK_TRANSITION_ASSERTIONS,
        **{"broker_clock_change::tests::" + name: "src/broker_clock_reader_tests.rs" for name in (
            "crossing_read_keeps_the_original_failure_under_its_revision",
            "native_failure_revokes_the_sample_pair",
            "newer_notice_wins_over_a_sampled_contradiction",
            "notice_or_stop_wins_over_a_sampled_getter_failure",
            "sampled_invalidation_closes_at_revision_exhaustion",
            "terminal_read_edge_withdraws_the_original_revision_immediately",
            "time_change_controls_are_role_and_lifecycle_bound")},
        **{"broker_clock_change::power_tests::" + name: "src/broker_clock_power_tests.rs" for name in (
            "power_delivery_is_role_lifecycle_and_event_scoped",
            "suspend_blocks_capture_until_automatic_resume_begins_a_fresh_ordinal",
            "suspended_ordinal_exhaustion_is_irreversible")},
        **{"host::tests::broker_clock_change_tests::" + name: path for name, path in (
            ("availability_tests::broker_reports_only_current_availability_before_cleanup",
             "src/broker_availability_tests.rs"),
            ("sampling_tests::cutoff_tests::sampled_broker_refuses_work_at_the_original_cutoff",
             "src/broker_clock_sampling_cutoff_tests.rs"),
            ("sampling_tests::selected_broker_samples_quietly_and_retains_observed_breaks",
             "src/broker_clock_sampling_tests.rs"),
            ("selected_broker_power_resumes_across_repeated_callbacks", "src/broker_clock_change_tests.rs"),
            ("selected_broker_renews_after_crossing_and_refuses_failed_running", "src/broker_clock_change_tests.rs"))},
    },
    CONTINUITY_DIRECT: {
        **CLOCK_TRANSITION_ASSERTIONS,
        **{"broker_clock_change::continuity::tests::" + name: "src/broker_clock_continuity_tests.rs"
           for name in ("changed_frequency_precedes_other_conflicts",
                        "compares_each_adjustment_field_exactly",
                        "compares_only_known_presentation_changes_after_adjustment",
                        "counter_order_uses_the_two_inner_edges",
                        "filetime_order_refuses_only_strict_regression",
                        "signed_tick_and_unsigned_wall_extremes_remain_exact")},
    },
}
DIRECT_ASSERTIONS = {WORKER_DIRECT: WORKER_ROUND_ASSERTIONS, WRITER_DIRECT: WRITER_ASSERTIONS, **CLOCK_ASSERTIONS}
# These private test modules are registered by host_tests.rs. Require their
# admitted libtest names as well as the source file supplying the assertion.
HOST_ASSERTION_MODULES: dict[str, str] = {
    "host::tests::broker_clock_change_tests::": "src/broker_clock_change_tests.rs",
    "host::tests::broker_endpoint_host_tests::": "src/broker_endpoint_host_tests.rs",
    "host::tests::broker_observation_tests::": "src/broker_observation_tests.rs",
}
CLOCK_NATIVE_GAPS = {
    "broker_clock_change": "GNU libtest excludes Windows-only native sampling attachment and reader construction.",
    "broker_clock_continuity": "GNU libtest excludes Windows-only raw sample getters and native sample projection.",
}
COVERAGE_SCOPES = {lane: C.Scope(PACKAGE, tuple((owner, Q.TOOL + "/" + path) for owner, path in subjects.items()),
                                "fixture-local")
                   for lane, subjects in COVERAGE_LANES.items()}
LIMITS = {"coverage_wall_seconds": 300, "enumeration_wall_seconds": 120,
          "mutation_wall_seconds": 1200, "build_seconds": 90, "test_seconds": 60,
          "maximum_mutants": 512, "mutation_jobs": 4, "inner_cargo_jobs": 1,
          "test_threads": 1, "coverage_minimum_line_percent": 90,
          "mutation_minimum_viable_killed_percent": 95}
EXPORT_HELPER = "scripts/run-service-analysis.py"


def require(condition, message):
    """Reject incomplete evidence independently of Python assertion settings."""
    if not condition:
        raise Incomplete(message)


def decode_record(data):
    """Parse the already-owned bounded record bytes with finite strict JSON."""
    try:
        text = data.decode("utf-8")
        depth = markers = 0
        quoted = escaped = False
        for character in text:
            if quoted:
                if escaped:
                    escaped = False
                elif character == "\\":
                    escaped = True
                elif character == '"':
                    quoted = False
            elif character == '"':
                quoted = True
            else:
                depth += int(character in "[{") - int(character in "]}")
                markers += int(character in "[]{},:")
                require(0 <= depth <= C.ANALYSIS_JSON_MAX_DEPTH and markers <= C.ANALYSIS_JSON_MAX_MARKERS,
                        "Received JSON exceeds its parser work bound.")

        def number(token, *, floating):
            require(len(token) <= 128, "Received JSON number exceeds its token bound.")
            result = float(token) if floating else int(token)
            require(not floating or math.isfinite(result), "Received JSON number is not finite.")
            return result

        value = json.loads(text, object_pairs_hook=C.ANALYSIS.unique_object,
                           parse_int=lambda token: number(token, floating=False),
                           parse_float=lambda token: number(token, floating=True),
                           parse_constant=lambda token: (_ for _ in ()).throw(Incomplete("Nonfinite received JSON.")))
    except (UnicodeError, ValueError, RecursionError) as error:
        raise Incomplete("Received evidence is not one strict bounded JSON object.") from error
    require(type(value) is dict, "Received evidence must be a JSON object.")
    return value


def executed_tests(text, expected=None, *, allow_doctests=False, allow_empty=False):
    """Require named successful tests and exact zero-skipped native summaries."""
    require(type(text) is str, "Test output must be actual text.")
    names, summaries = [], []
    for line in text.splitlines():
        if line.startswith("test result:"):
            match = re.fullmatch(r"test result: ok\. (\d+) passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;.*", line)
            require(match is not None, "Test execution failed, skipped or filtered cases.")
            summaries.append(int(match[1]))
        elif line.startswith("test "):
            match = re.fullmatch(r"test ([^\r\n]+?) \.\.\. ok", line)
            require(match is not None, "Test execution lacks a named successful completion.")
            names.append(match[1])
    require(summaries and (names or allow_empty) and sum(summaries) == len(names) and len(set(names)) == len(names),
            "Test execution is empty, duplicated or inconsistent.")
    if expected is not None:
        actual = [name for name in names if not (allow_doctests and " - (line " in name)]
        require(sorted(actual) == sorted(expected), "Test execution omits or adds ordinary cases.")
    return sorted(names)


def successful(result, *, mutation=False):
    """Retain native 2 provisionally only for complete mutation reconciliation."""
    require(type(result) is dict and type(result.get("exit_code")) is int
            and result["exit_code"] in ((0, 2) if mutation else (0,))
            and result.get("failure", "missing") is None and type(result.get("output")) is str,
            "The selected bounded command did not complete successfully.")


def require_source_receipt(receipt, binding, source):
    """Check the producer's unchanged source and index without reusing its live index."""
    require(type(receipt) is dict and type(receipt.get("schema_version")) is int
            and receipt["schema_version"] == 1 and receipt.get("scope") == "source-only inert service fixture"
            and receipt.get("native_execution") is False and receipt.get("status") == "passed"
            and receipt.get("bookends_passed") is True and receipt.get("index_bytes_unchanged") is True
            and not receipt.get("custody_error"), "Source receipt lacks successful producer custody.")
    require(receipt.get("source_sha256") == source and receipt.get("graph_binding") == binding
            and {key: receipt.get(key) for key in binding} == binding,
            "Source receipt differs from the admitted source or graph binding.")
    epoch = receipt.get("index_before")
    require(type(epoch) is dict and set(epoch) == {"path", "sha256"}
            and type(epoch["path"]) is str and PurePosixPath(epoch["path"]).is_absolute()
            and type(epoch["sha256"]) is str and re.fullmatch(r"[0-9a-f]{64}", epoch["sha256"])
            and receipt.get("index_after") == epoch, "Source receipt lacks unchanged producer index custody.")


def source_command_custody(row, argv, receipt, binding):
    """Check the recorded sandbox's admitted Cargo suffix and source working directory."""
    require(type(row) is dict and row.get("argv") == row.get("namespace_argv") == argv,
            "Ordinary fixture test receipt command or namespace differs.")
    successful(row)
    wrapper = row.get("command")
    require(type(wrapper) is list and wrapper and all(type(part) is str for part in wrapper)
            and "--" in wrapper, "Source receipt omits its actual process command.")
    split = wrapper.index("--")
    options = wrapper[:split]
    helpers = binding["tools"]["helpers"]
    limiter = ["--", helpers["prlimit"]["path"], "--as=8589934592", f"--cpu={Q.SECONDS}",
               "--nproc=128", "--nofile=256", "--fsize=268435456", "--", *argv]
    work = receipt.get("work")
    require(type(work) is str and PurePosixPath(work).is_absolute()
            and wrapper[0] == helpers["bwrap"]["path"] and wrapper[split:] == limiter
            and options.count("--chdir") == 1
            and options[options.index("--chdir") + 1:][:1] == ["/work/source"]
            and options.count("/work/source") == 2
            and any(options[i:i + 3] == ["--ro-bind", work + "/source", "/work/source"]
                    for i in range(len(options))),
            "Source receipt process differs from the admitted Cargo cwd, source or command.")


def source_test_roster(output, argv):
    """Read selected package/profile launches and each ordinary executable result."""
    def require_launch_before_results(launch):
        require(not any(line.startswith("test ") for line in output[:launch.start()].splitlines()),
                "Source test results precede their launch.")

    if "--doc" in argv:
        package = LIBRARY if "--manifest-path" in argv else "windows_platform"
        launches = list(re.finditer(r"^   Doc-tests (\S+)$", output, re.M))
        require(len(launches) == 1 and launches[0][1] == package,
                "Source doctest output differs from the selected package.")
        require_launch_before_results(launches[0])
        return executed_tests(output, allow_empty=package == LIBRARY)
    profile = "release" if "--release" in argv else "debug"
    fixture = "--manifest-path" in argv
    package = PACKAGE if fixture else "windows-platform"
    runs = list(re.finditer(r"^     Running (.+)$", output, re.M))
    roles = []
    for run in runs:
        match = re.fullmatch(
            r"(unittests src/|tests/)([a-z0-9_/]+)\.rs \(/work/target/" + profile + r"/build/"
            + package + r"/([0-9a-f]{16})/out/([a-z0-9_]+)-\3\)", run[1])
        require(match is not None, "Source test launch differs from the selected package or profile.")
        category, source, _, executable = match.groups()
        path = ("src/" if category.startswith("unittests") else category) + source + ".rs"
        name = package.replace("-", "_") if path == "src/lib.rs" else source.rsplit("/", 1)[-1]
        if fixture and source in ("bin/broker", "bin/engine"):
            name = "nevrotick_lab_s02_" + name
        require(executable == name, "Source test launch names a different target executable.")
        roles.append(path)
    require(roles and roles[0] == "src/lib.rs" and len(roles) == len(set(roles))
            and (not fixture or set(roles) == {source for _, source in FIXTURE_TARGETS.values()}),
            "Source test output lacks the selected executable roles.")
    require_launch_before_results(runs[0])
    for current, end in zip(runs, [run.start() for run in runs[1:]] + [len(output)], strict=True):
        block = output[current.end():end]
        require(sum(line.startswith("test result:") for line in block.splitlines()) == 1,
                "Source test executable lacks its own result.")
        executed_tests(block, allow_empty=True)
    return executed_tests(output)


def ordinary_fixture_tests(receipt, binding, source, graph_path, graph_digest):
    """Receive one ordinary fixture roster from a complete current source lane."""
    require_source_receipt(receipt, binding, source)
    lane = receipt.get("lane")
    require(lane in ("fixture-test", "privilege-checkpoint")
            and receipt.get("received_graph_path") == graph_path
            and receipt.get("received_graph_sha256") == graph_digest,
            "Metrics require an ordinary fixture test receipt bound to the selected graph.")
    cargo = "/toolchain/bin/cargo"
    expected = Q.lane_commands(lane, cargo)
    ordinary, = Q.lane_commands("fixture-test", cargo)
    require(type(receipt.get("commands")) is list and len(receipt["commands"]) == len(expected),
            "Ordinary fixture test receipt has incomplete commands.")
    rosters = {}
    for row, command in zip(receipt["commands"], expected, strict=True):
        source_command_custody(row, command, receipt, binding)
        if command[1] == "test":
            rosters[tuple(command)] = source_test_roster(row["output"], command)
    if lane == "privilege-checkpoint":
        for package in ("platform", "fixture"):
            debug, = Q.lane_commands(package + "-test", cargo)
            optimized, = Q.lane_commands(package + "-optimized", cargo)
            require(rosters[tuple(debug)] == rosters[tuple(optimized)],
                    "Source debug and optimized ordinary test rosters differ.")
    return rosters[tuple(ordinary)]


def metric_coverage(report, lane: str = "coverage"):
    """Score exact owner files independently, preserving all other raw line data."""
    require(lane in COVERAGE_LANES, "Unknown fixture coverage lane.")
    return {name: C.coverage_counts(COVERAGE_SCOPES[lane], report, "/work/source/" + Q.TOOL + "/" + filename)
            for name, filename in COVERAGE_LANES[lane].items()}


def inventory_listing(text, subject):
    """Admit every native descriptor, with no arbitrary filters or empty owner."""
    require(subject in NATIVE_SUBJECTS, "Unknown fixture owner.")
    items, warning = C.parse_mutation_listing(text, expected_jobs=4)
    require(warning is None, "Unexpected mutation enumeration warning.")
    inventory = C.mutation_inventory(SCOPE, items, LIMITS["maximum_mutants"], SUBJECTS[subject])
    for item in inventory.values():
        function = item.get("function")
        require(C.exact_mutant_span(item.get("span")) and type(item.get("replacement")) is str
                and type(item.get("genre")) is str and bool(item["genre"])
                and type(function) is dict and C.exact_mutant_span(function.get("span"))
                and type(function.get("function_name")) is str and bool(function["function_name"])
                and type(function.get("return_type")) is str,
                "Mutation descriptor lacks its full source/function/replacement identity.")
    return inventory


def inventory_summary(inventories):
    """Hash all native descriptor members except the optional rendered diff."""
    require(type(inventories) is dict and set(inventories) == set(NATIVE_SUBJECTS),
            "All complete fixture owner inventories are required.")
    C.combined_mutant_count(inventories, LIMITS["maximum_mutants"])
    return {name: {"count": len(rows), "sha256": C.canonical_sha256({
                key: {field: value for field, value in row.items() if field != "diff"}
                for key, row in rows.items()})} for name, rows in inventories.items()}


def receive_review(review, candidate, summaries):
    """Require an independent source/inventory review before launching mutations.

Reconciliation is source review evidence, not additional generated kills. Its
remaining gaps remain qualification gaps even when the numeric score passes.
"""
    require(type(review) is dict and set(review) == {"schema_version", "candidate_sha256", "inventories",
                "reviewer", "reconciliation"} and type(review.get("schema_version")) is int
            and review["schema_version"] == 1
            and review.get("candidate_sha256") == candidate and review.get("inventories") == summaries
            and type(review.get("reviewer")) is str and 0 < len(review["reviewer"]) <= 200,
            "Mutation requires the exact independently reviewed candidate and inventories.")
    reconciled = review["reconciliation"]
    require(type(review["inventories"]) is dict and set(review["inventories"]) == set(NATIVE_SUBJECTS)
            and all(type(row) is dict and set(row) == {"count", "sha256"}
                    and type(row["count"]) is int and type(row["sha256"]) is str
                    for row in review["inventories"].values()), "Review inventory counters must be exact integers.")
    require(type(reconciled) is dict and set(reconciled) == set(NATIVE_SUBJECTS),
            "Source reconciliation must cover every fixture owner.")
    for item in reconciled.values():
        require(type(item) is dict and set(item) == {"functions_and_guards", "derives_and_macros",
                    "unsafe_and_cfg_omissions", "independent_tests", "remaining_gaps"}
                and all(type(item.get(key)) is list for key in item)
                and all(type(value) is str and value.strip() and len(value) <= 8192
                        for values in item.values() for value in values)
                and item["functions_and_guards"] and item["derives_and_macros"]
                and item["unsafe_and_cfg_omissions"] and item["independent_tests"],
                "Review must reconcile functions/guards, derives/macros, unsafe/cfg omissions and independent tests.")
    return review


def admit_equivalence_review(review, candidate, inventories, identity, root):
    """Bind exact fixture equivalences to the current source, inventory and proof.

The shared admission helper assumes product-workspace-relative sources and its
product-only candidate. This standalone owner follows the same exact entry
schema, retaining native fixture-relative descriptors and the full Q candidate.
Admission checks custody; the named independent reviewer owns the domain
argument, counterexample search, effects and invalidating assumptions.
"""
    require(type(review) is dict and set(review) == {"schema_version", "candidate_sha256", "inventories",
                "mutation_equivalences"} and type(review.get("schema_version")) is int
            and review["schema_version"] == 1 and review.get("candidate_sha256") == candidate
            and type(candidate) is str and re.fullmatch(r"[a-f0-9]{64}", candidate)
            and review.get("inventories") == inventory_summary(inventories),
            "Equivalence review differs from the exact fixture candidate or full inventories.")
    require(type(review["inventories"]) is dict and set(review["inventories"]) == set(NATIVE_SUBJECTS)
            and all(type(row) is dict and set(row) == {"count", "sha256"}
                    and type(row["count"]) is int and type(row["sha256"]) is str
                    for row in review["inventories"].values()),
            "Equivalence inventory counters must be exact integers.")
    entries = review["mutation_equivalences"]
    require(type(entries) is list and len(entries) <= LIMITS["maximum_mutants"],
            "Exact equivalence inventory is invalid or oversized.")
    root = root.resolve(strict=True)
    identifiers, names = set(), set()
    for entry in entries:
        require(type(entry) is dict and set(entry) == {"id", "candidate_sha256", "source_sha256", "mutant",
                    "requirement", "rationale", "proof"}, "Exact equivalence fields are incomplete or unknown.")
        identifier, mutant, proof = entry["id"], entry["mutant"], entry["proof"]
        require(type(identifier) is str and re.fullmatch(r"[A-Z][A-Z0-9-]{0,63}", identifier)
                and identifier not in identifiers and type(mutant) is dict
                and set(mutant) == {"name", "package", "file", "function", "span", "replacement", "genre"}
                and mutant.get("package") == PACKAGE and mutant.get("file") in SUBJECTS.values()
                and type(mutant.get("name")) is str and mutant["name"] and mutant["name"] not in names,
                "Equivalence identity is duplicated, missing or outside the fixture owners.")
        function = mutant["function"]
        require(C.exact_mutant_span(mutant["span"]) and type(function) is dict
                and set(function) == {"function_name", "return_type", "span"}
                and C.exact_mutant_span(function["span"])
                and type(function["function_name"]) is str and 0 < len(function["function_name"]) <= 4096
                and type(function["return_type"]) is str and len(function["return_type"]) <= 4096
                and type(mutant["genre"]) is str and 0 < len(mutant["genre"]) <= 4096
                and type(mutant["replacement"]) is str and len(mutant["replacement"]) <= 4096
                and (mutant["replacement"] or mutant["genre"] == "UnaryOperator"),
                "Equivalence lacks the complete native function/span/replacement identity.")
        subject = next(name for name, filename in SUBJECTS.items() if filename == mutant["file"])
        actual = inventories[subject].get(mutant["name"])
        require(type(actual) is dict and {key: value for key, value in actual.items() if key != "diff"} == mutant,
                "Equivalence is not an exact fault in the complete frozen inventory.")
        source_name = Q.TOOL + "/" + mutant["file"]
        require(entry["candidate_sha256"] == candidate and type(entry["source_sha256"]) is str
                and re.fullmatch(r"[a-f0-9]{64}", entry["source_sha256"])
                and entry["source_sha256"] == identity.get(source_name)
                and C.PRODUCT.sha256(root / source_name) == entry["source_sha256"],
                "Equivalence source or full fixture candidate differs from review.")
        require(all(type(entry[key]) is str and 0 < len(entry[key].strip()) <= maximum
                    for key, maximum in (("requirement", 256), ("rationale", 4096))),
                "Equivalence requires a bounded requirement-grounded rationale.")
        require(type(proof) is dict and set(proof) == {"path", "sha256", "section", "reviewer", "verdict"}
                and proof["verdict"] == "independently-justified-equivalent" and proof["section"] == identifier
                and type(proof["reviewer"]) is str and 0 < len(proof["reviewer"].strip()) <= 128
                and type(proof["path"]) is str and type(proof["sha256"]) is str
                and re.fullmatch(r"[a-f0-9]{64}", proof["sha256"]),
                "Equivalence lacks an exact independent-review proof reference.")
        relative = Path(proof["path"])
        require(not relative.is_absolute() and relative.parts[:1] == ("tasks",) and ".." not in relative.parts
                and relative.as_posix() == proof["path"] and relative.suffix == ".md",
                "Equivalence proof must be a canonical maintained task document.")
        data = C.read_analysis_bytes(root / relative, maximum=1024 * 1024)
        require(hashlib.sha256(data).hexdigest() == proof["sha256"], "Equivalence proof bytes differ from review.")
        text = data.decode("utf-8")
        heading = re.search(r"^(#{2,5}) " + re.escape(identifier) + r"(?:[ \t][^\n]*)?$", text, re.M)
        require(heading is not None, "Equivalence proof lacks its exact maintained section.")
        fragment = text[heading.end():]
        following = re.search(r"^#{1," + str(len(heading[1])) + r"} ", fragment, re.M)
        if following:
            fragment = fragment[:following.start()]
        require(bool(fragment.strip()), "Equivalence proof section is empty.")
        identifiers.add(identifier)
        names.add(mutant["name"])
    return json.loads(json.dumps(entries))


def equivalent_misses(outcomes, equivalences):
    """Require every proposed equivalence to remain an exact raw missed mutant."""
    require(type(equivalences) in (list, tuple), "Invalid exact equivalence list.")
    rows = outcomes.get("outcomes")
    require(type(rows) is list and all(type(row) is dict for row in rows), "Mutation outcomes are incomplete.")
    missed = {}
    for row in rows:
        scenario = row.get("scenario")
        mutant = scenario.get("Mutant") if type(scenario) is dict else None
        if row.get("summary") == "MissedMutant":
            require(type(mutant) is dict and type(mutant.get("name")) is str and mutant["name"] not in missed,
                    "Raw missed-mutant identities are missing or duplicated.")
            missed[mutant["name"]] = row
    accepted = {}
    for entry in equivalences:
        require(type(entry) is dict and type(entry.get("mutant")) is dict,
                "Exact equivalence entry is incomplete.")
        mutant = entry["mutant"]
        name = mutant.get("name")
        require(type(name) is str and name not in accepted and name in missed
                and missed[name]["scenario"]["Mutant"] == mutant,
                "Equivalence is unknown, duplicated, changed or not an actual missed mutant.")
        accepted[name] = (entry, missed[name])
    return accepted


def selected_span(value, *, files=None):
    """Recognize only source/test paths in this standalone fixture namespace."""
    if type(value) is not str or ".." in PurePosixPath(value).parts:
        return False
    if files is None:
        files = ("src/host.rs", "src/error.rs", "src/diagnostics.rs", "src/host_tests.rs",
                 "src/error_tests.rs", "src/diagnostics_tests.rs", "src/engine_worker.rs",
                 "src/engine_worker_tests.rs", "tests/entrypoints.rs")
    return value in files or any(value == prefix + filename for filename in files
               for prefix in ("/work/source/" + Q.TOOL + "/", Q.TOOL + "/"))


def owned_log(directory, value):
    """Read a bounded actual log without following paths outside its campaign."""
    relative = C.analysis_output_path(value)
    require(not relative.is_absolute() and relative.parts and ".." not in relative.parts,
            "Mutation log path escapes its campaign.")
    return C.read_analysis_bytes(directory / relative, maximum=16 * 1024 * 1024)


def native_log_phases(row: dict, text: str) -> dict[str, str]:
    """Bind one pinned cargo-mutants log to the scenario, commands, results and tested build."""
    require(type(row) is dict and type(text) is str, "Native log requires an outcome and text.")
    C.mutation_phases(SCOPE, row)
    scenario = row.get("scenario")
    baseline = scenario == "Baseline"
    mutant = scenario.get("Mutant") if type(scenario) is dict else {}
    require((baseline and row.get("summary") == "Success") or (
                type(mutant) is dict and row.get("summary") in ("CaughtMutant", "MissedMutant", "Unviable")
                and mutant.get("package") == PACKAGE and mutant.get("file") in NATIVE_SUBJECTS.values()
                and type(mutant.get("name")) is str and mutant["name"]),
            "Native log belongs to an unknown fixture owner.")
    phases = row["phase_results"]
    markers = ["*** baseline"] if baseline else ["*** " + mutant["name"], "*** mutation diff:"]
    for phase in phases:
        require("--message-format=json" in phase["argv"], "Native fixture mutation requires Cargo JSON.")
        markers.extend(["*** " + " ".join(phase["argv"]),
                        "*** result: Success" if phase["process_status"] == "Success" else "*** result: Failure(101)"])
    lines = text.splitlines()
    delimiters = [i for i, line in enumerate(lines) if line.startswith("*** ")]
    require([lines[i] for i in delimiters] == markers and not any(lines[:delimiters[0]])
            and not any(lines[delimiters[-1] + 1:]), "Native log scenario, phase commands or results disagree.")
    sections = {}
    phase_bounds = delimiters[1 if baseline else 2:]
    for phase, start, end in zip(phases, phase_bounds[::2], phase_bounds[1::2], strict=True):
        sections[phase["phase"]] = "\n".join(lines[start + 1:end])
    if len(phases) == 2:
        require(not any(lines[phase_bounds[1] + 1:phase_bounds[2]]), "Unexpected data between native phases.")
    reject_infrastructure_failure(text)
    if row["summary"] == "Unviable":
        finished = [line for line in sections["Build"].splitlines() if line.startswith('{"reason":"build-finished"')]
        require(len(finished) == 1 and decode_record(finished[0].encode()).get("success") is False,
                "Unviable mutation lacks its failed Cargo Build.")
        return sections
    require(not re.search(r"error\[E\d+\]|could not compile", sections["Build"]),
            "Successful fixture Build contains a compiler failure.")
    root = "/work/native-source/" + Q.TOOL
    artifacts = fixture_build_artifacts(sections["Build"], root,
                                        subject_file=mutant.get("file"), allow_cached=baseline)
    sections["Test"] = fixture_test_runtime(sections["Test"], root, artifacts,
        complete=row["summary"] != "CaughtMutant", verbose=True, subject=mutant.get("file"))
    return sections


def mutation_causes(directory, outcomes, expected_tests, equivalences=()):
    """Read each raw mutant once and bind cause or survival to its own admitted phases."""
    causes = {}
    equivalents = equivalent_misses(outcomes, equivalences)
    for row in outcomes["outcomes"]:
        if row.get("scenario") == "Baseline":
            continue
        data = owned_log(directory, row.get("log_path", ""))
        sections = native_log_phases(row, data.decode("utf-8"))
        name = row["scenario"]["Mutant"]["name"]
        require(name not in causes, "Duplicate native mutation evidence.")
        if row["summary"] == "MissedMutant":
            executed_tests(sections["Test"], expected_tests, allow_doctests=True)
            cause = {"kind": "survived", "kill_credit": 0}
            if name in equivalents:
                entry, _ = equivalents[name]
                cause.update(kind="reviewed-equivalent-survivor", equivalence_id=entry["id"], proof=entry["proof"])
        elif row["summary"] == "Unviable":
            cause = classify_cause(sections["Build"], expected_tests, compiler=True, native_subject=row["scenario"]["Mutant"]["file"])
        else:
            require(row["summary"] == "CaughtMutant", "Unclassified native mutation evidence.")
            cause = classify_cause(sections["Test"], expected_tests)
        causes[name] = {**cause, "log_sha256": hashlib.sha256(data).hexdigest()}
    return causes


def reject_infrastructure_failure(text: str) -> None:
    """Deny qualification when native output records a tool, resource or custody failure."""
    require(not re.search(r"HARNESS_INCOMPLETE|Cannot allocate memory|memory allocation of [^\n]*failed|"
                          r"No space left on device|No such file or directory|Permission denied|"
                          r"failed to (?:run custom build command|execute process)|could not execute process|"
                          r"internal compiler error", text),
            "Mutation failure has an infrastructure cause.")


def classify_cause(text: str, expected_tests: list[str], *, compiler: bool = False,
                   required_source: str | None = None, native_subject: str | None = None,
                   assertion_files: dict[str, str] | None = None,
                   allow_legacy_assertions: bool = False) -> dict:
    """Attribute actual failures; shared by native results and tagged direct replay."""
    reject_infrastructure_failure(text)
    diagnostics = []
    for line in text.splitlines():
        if line.startswith('{"reason":') and '"compiler-message"' in line:
            try:
                item = json.loads(line)
            except (ValueError, RecursionError) as error:
                raise Incomplete("Malformed mutation compiler diagnostic.") from error
            require(type(item) is dict and type(item.get("message")) is dict,
                    "Mutation compiler diagnostic is incomplete.")
            if item["message"].get("level") == "error":
                diagnostics.append(item)
    digest = hashlib.sha256(text.encode("utf-8")).hexdigest()
    if compiler:
        root = ("/work/native-source/" if native_subject is not None else "/work/source/") + Q.TOOL
        # On GNU these private modules and their callers compile only into the fixture libtest.
        callers = {"src/host.rs": {"src/host.rs", "src/host_tests.rs", "src/engine_worker.rs", "src/engine_worker_tests.rs"},
                   "src/diagnostics.rs": {"src/diagnostics.rs", "src/diagnostics_tests.rs", "src/host.rs",
                                          "src/host_tests.rs", "src/engine_worker_tests.rs",
                                          "src/diagnostics_thread.rs", "src/diagnostics_thread_tests.rs"}}
        if native_subject in callers:
            failures = [line for line in text.splitlines() if line.startswith("error:")]
            require(failures and all(line.startswith("error: could not compile `" + PACKAGE + "` (lib test) due to ")
                                     for line in failures), "Cfg-only mutation did not fail in the fixture libtest.")
        attributed = []
        for item in diagnostics:
            spans = item["message"].get("spans")
            require(type(spans) is list and all(type(span) is dict for span in spans),
                    "Compiler error lacks source attribution.")
            primary = [span for span in spans if span.get("is_primary") is True]
            target = item.get("target")
            name = target.get("name") if type(target) is dict else None
            require(type(name) is str and name in FIXTURE_TARGETS
                    and item.get("package_id") == "path+file://" + root + "#" + PACKAGE + "@0.0.0"
                    and item.get("manifest_path") == root + "/Cargo.toml",
                    "Mutation compiler diagnostic has a different package, manifest or target.")
            kind, target_source = FIXTURE_TARGETS[name]
            require(target.get("kind") == [kind] and target.get("src_path") == root + "/" + target_source
                    and (required_source is None or name == LIBRARY),
                    "Mutation compiler diagnostic has a different target role or source.")
            if native_subject in callers:
                accepted: set[str] = callers[native_subject]
                code: object = item["message"].get("code")
                # Removing host dispatch, emission or shutdown can orphan its diagnostic callees.
                if native_subject == "src/host.rs" and type(code) is dict and code.get("code") == "dead_code":
                    accepted = accepted | {"src/diagnostics.rs"}
                paths: set[str] = {prefix + path for path in accepted for prefix in ("", Q.TOOL + "/", root + "/")}
                require(name == LIBRARY and all(span.get("file_name") in paths for span in primary),
                        "Cfg-only mutation compiler diagnostic has a different target or primary source.")
            if required_source is not None:
                attributed_source = all(selected_span(span.get("file_name"), files=(required_source,))
                                        for span in primary)
            else:
                attributed_source = native_subject in callers or all(
                    selected_span(span.get("file_name")) for span in primary)
            require(attributed_source,
                    "Unviable mutant contains an unrelated compiler error.")
            if primary:
                attributed.append(item["message"])
            else:
                require(not spans and re.fullmatch(r"aborting due to \d+ previous errors?(?:; \d+ warnings? emitted)?",
                            item["message"].get("message", "")) is not None,
                        "Unviable mutant has an unattributed compiler error.")
        require(attributed, "Unviable mutant lacks an actual selected compiler error.")
        require(all(re.fullmatch(r"error: could not compile `nevrotick-windows-service-fixture`(?: \([^\n)]+\))? due to \d+ previous errors?(?:; \d+ warnings? emitted)?", line)
                    for line in text.splitlines() if line.startswith("error:")),
                "Unviable mutant has an unclassified native error.")
        return {"kind": "compiler-unviable", "kill_credit": 0, "log_sha256": digest}
    require(not diagnostics and not re.search(r"error\[E\d+\]|could not compile", text),
            "Caught mutant contains a compiler failure.")
    def admitted_assertion(name: str, path: str) -> bool:
        if name not in expected_tests:
            return False
        if assertion_files is not None:
            if name in assertion_files:
                return selected_span(path, files=(assertion_files[name],))
            return allow_legacy_assertions and selected_span(path)
        if selected_span(path):
            return True
        for owners in DIRECT_ASSERTIONS.values():
            if name in owners:
                return selected_span(path, files=(owners[name],))
        for module, source in HOST_ASSERTION_MODULES.items():
            if name.startswith(module):
                return selected_span(path, files=(source,))
        return False

    assertions = []
    for match in re.finditer(r"^---- ([^\r\n]+) stdout ----\n(.*?)(?=^---- |^failures:|\Z)", text, re.M | re.S):
        name, body = match.groups()
        if f"test {name} ... FAILED" not in text:
            continue
        assertion = re.search(r"panicked at ([^\n]+?):(\d+):\d+:\nassertion[^\n]*failed", body)
        if assertion is not None and admitted_assertion(name, assertion[1]):
            assertions.append({"test": name, "source": assertion[1], "line": int(assertion[2])})
    # Mutation-only nocapture can retain a meaningful assertion before a
    # later fatal custody guard aborts libtest. The abort itself supplies no
    # cause: require the actual ordinary test's named thread, selected source
    # and immediate assertion diagnostic. Generic panic/Result text is not
    # a substitute for this evidence.
    for match in re.finditer(r"^(?:test ([^\r\n]+?) \.\.\. )?thread '([^'\r\n]+)'(?: \(\d+\))? "
                             r"panicked at ([^\r\n]+?):(\d+):\d+:\nassertion[^\r\n]*failed[^\r\n]*", text, re.M):
        prefix, name, source, line = match.groups()
        if (prefix is None or prefix == name) and admitted_assertion(name, source):
            if not any(row["test"] == name and row["source"] == source and row["line"] == int(line)
                       for row in assertions):
                assertions.append({"test": name, "source": source, "line": int(line),
                                   "kind": "unbuffered-semantic-assertion"})
    require(assertions, "Caught mutant lacks an actual selected semantic assertion.")
    return {"kind": "behavioral-assertion", "causes": assertions, "log_sha256": digest}


def direct_inventory(text: str, source: bytes, *, owner: DirectOwner = WORKER_DIRECT) -> dict:
    """Validate every raw direct row and its exact ASCII single-span rendered diff."""
    require(type(text) is str and len(text.encode()) <= 4 * 1024 * 1024
            and type(source) is bytes and 0 < len(source) <= 256 * 1024
            and source.isascii() and source.endswith(b"\n"), "Direct input exceeds its ASCII/byte contract.")
    rows = decode_record(('{"rows":' + text + '}').encode())["rows"]
    require(type(rows) is list and 0 < len(rows) <= LIMITS["maximum_mutants"],
            "Direct inventory is empty or oversized.")
    lines, inventory = source.splitlines(keepends=True), {}
    genres = {"FnValue", "BinaryOperator", "UnaryOperator", "MatchArmGuard"}
    if owner == CHANGE_DIRECT:
        genres.add("MatchArm")

    def offset(point: dict) -> int:
        require(type(point) is dict and set(point) == {"line", "column"}
                and all(type(value) is int and value > 0 for value in point.values())
                and point["line"] <= len(lines)
                and point["column"] <= len(lines[point["line"] - 1]), "Direct span is outside source.")
        return sum(map(len, lines[:point["line"] - 1])) + point["column"] - 1

    def bounds(span: dict) -> tuple[int, int]:
        require(type(span) is dict and set(span) == {"start", "end"}, "Direct span fields differ.")
        first, last = offset(span["start"]), offset(span["end"])
        require(first < last, "Direct span must be ordered and nonempty.")
        return first, last

    for row in rows:
        require(type(row) is dict and set(row) == {"name", "package", "file", "function", "span",
                    "replacement", "genre", "diff"} and row["package"] == "single_file"
                and row["file"] == owner.leaf
                and all(type(row[key]) is str and len(row[key]) <= 65536
                        for key in ("name", "replacement", "genre", "diff"))
                and row["name"] and row["genre"] in genres
                and row["name"] not in inventory and row["replacement"].isascii()
                and "\n" not in row["replacement"] and "\r" not in row["replacement"],
                "Direct descriptor is incomplete, duplicated or outside its owner.")
        function = row["function"]
        require(type(function) is dict and set(function) == {"function_name", "return_type", "span"}
                and type(function["function_name"]) is str and 0 < len(function["function_name"]) <= 512
                and type(function["return_type"]) is str and len(function["return_type"]) <= 512,
                "Direct function identity is invalid.")
        first, last = bounds(row["span"])
        outer_first, outer_last = bounds(function["span"])
        require(outer_first <= first < last <= outer_last, "Direct mutation escapes its function.")
        coordinate = row["span"]["start"]
        prefix = f"{owner.leaf}:{coordinate['line']}:{coordinate['column']}: "
        require(row["name"].startswith(prefix), "Direct name differs from its exact span.")
        description = row["name"][len(prefix):]
        original = source[first:last].decode("ascii")
        function_name, replacement = function["function_name"], row["replacement"]
        if row["genre"] == "FnValue":
            returns = (" " + function["return_type"]) if function["return_type"] else ""
            expected_description = f"replace {function_name}{returns} with {replacement}"
        elif row["genre"] == "MatchArm":
            # The pinned generator names these deletions from the dispatch arm pattern alone.
            pattern = re.match(r"(\(TIME_CHANGE, _\)|\(POWER_EVENT, (?:SUSPENDING|AUTOMATIC_RESUME)\)) => ", original)
            require(not replacement and function_name == "deliver" and pattern is not None,
                    "Direct match-arm deletion differs from the broker control dispatch.")
            expected_description = f"delete match arm {pattern[1]} in {function_name}"
        elif row["genre"] == "UnaryOperator":
            require(replacement == "", "Unary deletion must preserve the empty raw replacement.")
            expected_description = f"delete {original} in {function_name}"
        else:
            guard = "match guard " if row["genre"] == "MatchArmGuard" else ""
            expected_description = f"replace {guard}{original} with {replacement} in {function_name}"
        require(description == expected_description, "Direct name differs from its complete source descriptor.")
        changed = source[:first] + (row["replacement"] + " /* ~ changed by cargo-mutants ~ */").encode() + source[last:]
        low, high = coordinate["line"] - 1, row["span"]["end"]["line"]
        context_low, context_high = max(0, low - 8), min(len(lines), high + 8)
        changed_lines = changed.splitlines(keepends=True)
        delta = len(changed_lines) - len(lines)
        old_count, new_count = context_high - context_low, context_high - context_low + delta
        def count(value: int) -> str:
            return str(context_low + 1) + (f",{value}" if value != 1 else "")
        rendered = (f"--- {owner.leaf}\n+++ {description}\n@@ -{count(old_count)} +{count(new_count)} @@\n".encode()
                    + b"".join(b" " + line for line in lines[context_low:low])
                    + b"".join(b"-" + line for line in lines[low:high])
                    + b"".join(b"+" + line for line in changed_lines[low:high + delta])
                    + b"".join(b" " + line for line in lines[high:context_high]))
        require(rendered == row["diff"].encode(), "Direct rendered diff differs from the exact source overlay.")
        inventory[row["name"]] = {"raw": row, "source": changed,
                                   "source_sha256": hashlib.sha256(changed).hexdigest()}
    return inventory


def direct_summary(inventory: dict, *, owner: DirectOwner = WORKER_DIRECT) -> dict:
    """Keep direct raw identities separate from native cargo-mutants descriptors."""
    require(inventory and all(row["raw"].get("file") == owner.leaf for row in inventory.values()),
            "Direct summary contains another owner's descriptors.")
    return {"schema": owner.schema, "count": len(inventory),
            "sha256": Q.canonical({name: row["raw"] for name, row in inventory.items()})}



def replay_clean_command() -> list[str]:
    """Let Cargo invalidate only this fixture package before each source build."""
    return ["/toolchain/bin/cargo", "clean", "--manifest-path", "/work/source/" + Q.MANIFEST,
            "--target-dir", "/work/target", "-p", PACKAGE, "--frozen", "--offline"]


def fixture_build_artifacts(output: str, root: str, *, subject_file: str | None = None,
                            allow_cached: bool = False) -> dict:
    """Identify every fixture target once and require the roles affected by this build to recompile."""
    require(type(output) is str, "Fixture Build requires actual Cargo JSON output.")
    expected = {(name, test) for name in FIXTURE_TARGETS for test in (False, True) if name != "entrypoints" or test}
    # On GNU, cfg(any(windows, test)) includes host and diagnostics only in the
    # fixture libtest. The common error module also feeds the normal library and its users.
    rebuilt = {(LIBRARY, True)} if subject_file in {"src/host.rs", "src/diagnostics.rs"} else expected
    manifest = root + "/Cargo.toml"
    package_id = "path+file://" + root + "#" + PACKAGE + "@0.0.0"
    observed = {}
    finished = False
    for line in output.splitlines():
        if not line.startswith("{"):
            continue
        item = decode_record(line.encode())
        require(not finished, "Fixture Build emitted Cargo data after its completion.")
        if item.get("reason") == "build-finished":
            require(item.get("success") is True, "Fixture Build reports unsuccessful compilation.")
            finished = True
        if item.get("reason") == "compiler-message":
            require(type(item.get("message")) is dict and item["message"].get("level") != "error",
                    "Successful fixture Build contains a compiler error.")
        if item.get("reason") != "compiler-artifact":
            continue
        if item.get("manifest_path") != manifest and item.get("package_id") != package_id:
            continue
        target, profile = item.get("target"), item.get("profile")
        require(item.get("manifest_path") == manifest and item.get("package_id") == package_id
                and type(target) is dict and type(target.get("name")) is str and target["name"] in FIXTURE_TARGETS
                and type(profile) is dict and type(profile.get("test")) is bool,
                "Fixture Build artifact package, manifest or target differs.")
        name, test = target["name"], profile["test"]
        role = (name, test)
        kind, source = FIXTURE_TARGETS[name]
        require(role in expected and role not in observed and target.get("kind") == [kind]
                and target.get("src_path") == root + "/" + source and type(item.get("fresh")) is bool,
                "Fixture Build artifact is duplicated, malformed or has the wrong source role.")
        require(allow_cached or role not in rebuilt or item["fresh"] is False,
                "Fixture Build cached an artifact that must compile the selected source.")
        executable = item.get("executable")
        if test:
            require(type(executable) is str and re.fullmatch(
                        r"/work/target/debug/build/" + PACKAGE + r"/([0-9a-f]{16})/out/"
                        + re.escape(name.replace("-", "_")) + r"-\1", executable)
                    and item.get("filenames") == [executable],
                    "Fixture Build lacks the selected test executable.")
        observed[role] = executable
    require(finished and set(observed) == expected, "Fixture Build lacks completion or seven distinct target roles.")
    return observed


def fixture_test_runtime(output: str, root: str, built: dict, *, complete: bool, verbose: bool, subject: str | None = None) -> str:
    """Bind Test's own artifact declarations and observed launches to the selected Build roles."""
    reject_infrastructure_failure(output)
    require(not re.search(r"error\[E\d+\]|could not compile", output), "Fixture Test contains a compiler failure.")
    # The Cargo producer emits this JSON record before displaying any actual libtest invocation.
    boundaries = list(re.finditer(r'^\{"reason": *"build-finished"[^\n]*\}$', output, re.M))
    require(len(boundaries) == 1, "Fixture Test lacks a unique Cargo compilation boundary.")
    boundary = boundaries[0].end()
    # A Test invocation may compile the same role again from its active variant.
    # Its declared executable must still match the Build declaration and the actual launch.
    tested = fixture_build_artifacts(output[:boundary], root, allow_cached=True)
    library = "nevrotick_windows_service_fixture"
    sources = {library: "unittests src/lib.rs", "nevrotick-lab-s02-engine": "unittests src/bin/engine.rs",
               "nevrotick-lab-s02-broker": "unittests src/bin/broker.rs", "entrypoints": "tests/entrypoints.rs"}
    programs = {}
    for name, source in sources.items():
        path = tested[(name, True)]
        require(path == built[(name, True)], "Fixture Test executable differs from its selected Build role.")
        display = f"`{path}`" if verbose else f"{source} ({path})"
        programs["     Running " + display] = name

    lines = output[boundary:].splitlines()
    launches = []
    for index, line in enumerate(lines):
        if line.startswith("     Running "):
            if verbose and line.startswith("     Running `/toolchain/bin/rustdoc "):
                require(complete and {role for _, role in launches} == set(sources),
                        "Fixture doctests cannot supply an ordinary test failure.")
                launches.append((index, "rustdoc"))
            else:
                require(line in programs, "Fixture Test ran an unexpected executable.")
                launches.append((index, programs[line]))
    roles = [role for _, role in launches]
    require(roles and roles[0] == library and len(roles) == len(set(roles))
            and (not complete or set(sources) <= set(roles)), "Fixture Test lacks its selected executable launches.")
    # Each GNU libtest emits its own summary. A successful predecessor cannot
    # supply the assertion for a later failure in a distinct executable.
    blocks = ["\n".join(lines[start:end]) for (start, _), end in
              zip(launches, [index for index, _ in launches[1:]] + [len(lines)], strict=True)]
    for block in (blocks if complete else blocks[:-1]):
        require(sum(line.startswith("test result:") for line in block.splitlines()) == 1,
                "An earlier or successful fixture program lacks its own result.")
        executed_tests(block, allow_empty=True)
    if complete:
        return "\n".join(blocks)
    require((subject == "src/error.rs" or roles[-1] == library)
            and not re.search(r"^test result: ok\.", blocks[-1], re.M),
            "Failed fixture Test does not identify an affected failing program.")
    return blocks[-1]


def fresh_fixture_build(output: str) -> int:
    """A package-cleaned replay must emit seven rebuilt fixture roles; dependencies may be cached."""
    return len(fixture_build_artifacts(output, "/work/source/" + Q.TOOL))


def initial_replay_target(work: Path) -> dict:
    """Admit a new empty private target; Cargo owns its eventual cache marker."""
    require(work.is_dir() and work.resolve(strict=True) == work,
            "Replay work owner is not canonical.")
    target = work / "target"
    require(not target.is_symlink(), "Initial replay target is a symlink.")
    if target.exists():
        require(target.is_dir() and target.resolve(strict=True) == target and not any(target.iterdir()),
                "Initial replay target must be an empty owned directory.")
        state = "empty-directory"
    else:
        state = "absent"
    return {"path": "/work/target", "state": state, "artifacts": 0, "clean_skipped": True}


def replay_commands() -> list[list[str]]:
    """One Cargo job and all fixture cases, with compiler JSON and visible assertions."""
    common = ["/toolchain/bin/cargo", "test", "--manifest-path", "/work/source/" + Q.MANIFEST,
              "--frozen", "--offline", "--jobs", "1", "--target-dir", "/work/target",
              "--all-targets", "--message-format=json"]
    return [[*common, "--no-run"], [*common, "--", "--test-threads=1", "--nocapture"]]


def replay_outcome(build: dict, test: dict | None, expected: list[str], *,
                   owner: DirectOwner = WORKER_DIRECT) -> dict:
    """Classify actual direct Build/Test observations without native outcome translation."""
    for result in (build, test):
        if result is None:
            continue
        require(type(result) is dict and type(result.get("exit_code")) is int
                and result.get("failure", "missing") is None and type(result.get("output")) is str,
                "Replay command is timed out, missing or incomplete.")
    if build["exit_code"] == 101:
        require(test is None, "Compiler-unviable replay must not invent a Test observation.")
        return {"kind": "compiler-unviable", "cause": classify_cause(build["output"], expected,
                    compiler=True, required_source=owner.file), "kill_credit": 0}
    require(build["exit_code"] == 0 and test is not None, "Replay Build did not complete.")
    require(test["exit_code"] in (0, 101), "Replay Test has an unclassified native exit.")
    root = "/work/source/" + Q.TOOL
    artifacts = fixture_build_artifacts(build["output"], root)
    runtime = fixture_test_runtime(test["output"], root, artifacts, complete=test["exit_code"] == 0, verbose=False)
    if test["exit_code"] == 0:
        return {"kind": "survived", "tests": executed_tests(runtime, expected), "kill_credit": 0}
    return {"kind": "killed", "cause": classify_cause(runtime, expected, assertion_files=DIRECT_ASSERTIONS[owner],
                                                    allow_legacy_assertions=owner == WORKER_DIRECT),
            "kill_credit": 1}


def replay_score(rows: dict, inventory: dict, *, complete: bool, owner: DirectOwner = WORKER_DIRECT) -> dict:
    """A pilot has zero full-campaign credit; only complete viable rows can pass 95%."""
    require(type(complete) is bool and type(rows) is dict and rows and set(rows) <= set(inventory),
            "Replay score has empty or unknown rows.")
    require(all(type(row) is dict and row.get("kind") in {"killed", "survived", "compiler-unviable"}
                and type(row.get("kill_credit")) is int
                and row["kill_credit"] == int(row["kind"] == "killed") for row in rows.values()),
            "Replay verdict/counter is incomplete or unknown.")
    require(not complete or set(rows) == set(inventory), "Full replay omits generated rows.")
    killed = sum(row["kind"] == "killed" for row in rows.values())
    viable = sum(row["kind"] != "compiler-unviable" for row in rows.values())
    return {"schema": owner.schema, "generated": len(inventory), "executed": len(rows),
            "killed": killed, "viable": viable, "unviable": len(rows) - viable,
            "survived": viable - killed, "complete": complete, "excluded": 0,
            "viable_killed_percent": 100 * killed / viable if viable and complete else None,
            "gate_passed": bool(complete and viable and 100 * killed >= 95 * viable and killed == viable)}


def check_variant(session, source: Path, identity: dict) -> None:
    """Original and derivative bookends bind every file, including unrelated unchanged inputs."""
    session.check()
    require(source != session.source and source.is_relative_to(session.work)
            and source.resolve(strict=True) == source and C.PRODUCT.inventory(source) == identity,
            "Owned replay derivative differs from its exact variant identity.")


def recorded_command(session, source, label, command, seconds, *, variant_sha256=None, **options):
    """Retain an available bounded observation once, before a durable deadline can unwind."""
    identity = {"label": label, "argv": command}
    if variant_sha256 is not None:
        identity["variant_sha256"] = variant_sha256

    def retain(result):
        session.record["commands"].append({**identity, **result})

    deadline = getattr(getattr(session, "args", None), "deadline_monotonic", None)
    if deadline is not None:
        options.update(deadline_monotonic=deadline, on_group_return=retain)
    result = Q.run_command(session.work, command, session.admitted, session.vendor, source,
                           timeout=seconds, cpu_seconds=seconds, **options)
    if deadline is None:
        retain(result)
    return result


def replay_command(session, source: Path, identity: dict, label: str, command: list[str], seconds: int) -> dict:
    """Retain raw failure observations before attribution; never relax Session.check."""
    check_variant(session, source, identity)
    result = recorded_command(session, source, label, command, seconds, mutation=True,
                              cwd=source / Q.TOOL, variant_sha256=Q.canonical(identity))
    check_variant(session, source, identity)
    return result



def retained_variant(session, directory: Path, manifest: dict, *, owner: DirectOwner = WORKER_DIRECT) -> None:
    """Verify strict sparse evidence reconstructs the complete bound variant."""
    E.directory(directory)
    require(set(C.PRODUCT.inventory(directory)) == {"manifest.json", owner.leaf},
            "Sparse evidence has missing, linked or unexpected files.")
    actual = decode_record(C.read_analysis_bytes(directory / "manifest.json", maximum=1024 * 1024))
    require(actual == manifest and Q.canonical(actual) == Q.canonical(manifest)
            and actual.get("schema") == owner.schema + ":retained-variant",
            "Retained variant manifest is missing, altered or has unknown fields.")
    source = C.read_analysis_bytes(directory / owner.leaf, maximum=256 * 1024)
    retained_hash = actual[f"{owner.prefix}_sha256"]
    retained_path = actual[f"{owner.prefix}_path"]
    require(hashlib.sha256(source).hexdigest() == retained_hash
            and actual["original_source_sha256"] == session.identity
            and actual["original_sha256"] == Q.canonical(session.identity),
            "Retained owner bytes or original full source identity differ.")
    reconstructed = {**actual["original_source_sha256"], retained_path: retained_hash}
    require(retained_path == owner.path and owner.path in session.identity
            and retained_hash != session.identity[owner.path]
            and reconstructed == actual["variant_source_sha256"]
            and Q.canonical(reconstructed) == actual["variant_sha256"],
            "Sparse variant does not reconstruct exactly one changed direct owner.")
    session.check()


def retire_variant(session, derivative: Path, identity: dict, number: int, row: dict, *,
                   owner: DirectOwner = WORKER_DIRECT) -> dict:
    """Retain complete sparse evidence before removing one completed derivative.

    Validation failure leaves its derivative intact. A deletion failure preserves
    all sparse/base evidence and the possibly partial derivative, records that
    state and stops; cleanup does not attempt additional restoration I/O.
    """
    require(type(number) is int and 0 <= number < LIMITS["maximum_mutants"]
            and derivative == session.work / f"{owner.prefix}-variant-{number:03d}"
            and derivative.parent == session.work and derivative != session.source,
            "Retirement requires the exact numbered owned derivative child.")
    E.directory(derivative)
    attachment = derivative.stat()
    check_variant(session, derivative, identity)
    require(type(row) is dict and type(row.get("raw")) is dict
            and session.record.get("direct_inventory", {}).get(row["raw"].get("name")) == row["raw"]
            and row.get("source_sha256") == identity.get(owner.path)
            and row.get("variant_sha256") == Q.canonical(identity),
            "Retirement lacks its admitted raw descriptor and exact variant identity.")
    successful(row.get("Clean"))
    classified = replay_outcome(row.get("Build"), row.get("Test"), session.tests, owner=owner)
    observed = {key: row.get(key) for key in classified}
    require(observed == classified and Q.canonical(observed) == Q.canonical(classified),
            "Retirement requires an actual completed classified replay.")
    labels = []
    for phase in ("Clean", "Build", "Test"):
        if row.get(phase) is None:
            continue
        label = f"{owner.prefix}:{number}:{phase}"
        commands = [item for item in session.record["commands"] if item.get("label") == label]
        require(len(commands) == 1 and commands[0].get("variant_sha256") == Q.canonical(identity)
                and all(commands[0].get(key) == value for key, value in row[phase].items()),
                "Retirement command observation is missing, duplicated or altered.")
        labels.append(label)
    retained_root = session.work / f"{owner.prefix}-replay"
    retained_root.mkdir(mode=0o700, exist_ok=True)
    E.directory(retained_root)
    directory = retained_root / f"{number:03d}"
    directory.mkdir(mode=0o700)
    source = C.read_analysis_bytes(derivative / owner.path, maximum=256 * 1024)
    with (directory / owner.leaf).open("xb") as stream:
        stream.write(source)
    manifest = {"schema": owner.schema + ":retained-variant", "original_source_sha256": session.identity,
                "original_sha256": Q.canonical(session.identity), "variant_source_sha256": identity,
                "variant_sha256": Q.canonical(identity), f"{owner.prefix}_path": owner.path,
                f"{owner.prefix}_sha256": identity[owner.path], "raw": row["raw"], "command_labels": labels}
    Q.write_new(directory / "manifest.json", manifest)
    retained_variant(session, directory, manifest, owner=owner)
    check_variant(session, derivative, identity)
    E.directory(derivative)
    current = derivative.stat()
    require((current.st_dev, current.st_ino) == (attachment.st_dev, attachment.st_ino),
            "Derivative attachment changed before retirement.")
    retention = {"path": directory.relative_to(session.work).as_posix(),
                 "manifest_sha256": C.PRODUCT.sha256(directory / "manifest.json"), "status": "retiring"}
    row["retention"] = retention
    try:
        shutil.rmtree(derivative)
        require(not derivative.exists(), "Retired derivative remains attached.")
        retained_variant(session, directory, manifest, owner=owner)
    except Exception as error:
        retention.update(status="incomplete-possibly-partial-derivative",
                         retirement_error=type(error).__name__ + ": " + str(error))
        raise
    retention["status"] = "retired"
    return retention


def execute_direct(session, owner: DirectOwner) -> None:
    """Enumerate or replay the selected owner's exact bounded direct rows."""
    record, args = session.record, session.args
    prefix = owner.prefix
    require(DIRECT_LANES.get(args.lane) == owner, "Direct lane differs from its configured source owner.")
    if owner == WRITER_DIRECT:
        require(WRITER_ASSERTIONS.keys() <= set(session.tests), "Admitted test roster omits a writer control.")
    if owner in CLOCK_ASSERTIONS:
        require(CLOCK_ASSERTIONS[owner].keys() <= set(session.tests), "Admitted test roster omits a broker clock control.")
    result = session.run(f"{prefix}-direct-generated", [str(session.admitted["extensions"]["cargo-mutants"]),
                          "mutants", "--Zmutate-file", owner.file, "--json"], 120)
    source = C.read_analysis_bytes(session.source / owner.path, maximum=256 * 1024)
    inventory = direct_inventory(result["output"], source, owner=owner)
    summary = direct_summary(inventory, owner=owner)
    record.update(direct_schema=owner.schema, direct_inventory={name: row["raw"] for name, row in inventory.items()},
                  direct_summary=summary)
    if args.lane == f"{prefix}-enumerate":
        record["review_template"] = {"schema": owner.schema, "candidate_sha256": record["candidate_sha256"],
                                    "summary": summary, "reviewer": "", "reconciliation": {}}
        return
    previous = session.receipt(args.direct_inventory)
    require(previous.get("status") == "passed" and previous.get("lane") == f"{prefix}-enumerate"
            and previous.get("bookends_passed") is True and previous.get("export_complete") is True
            and previous.get("graph_binding") == session.binding
            and previous.get("direct_schema") == owner.schema and previous.get("direct_summary") == summary
            and (owner == WORKER_DIRECT or previous.get("subjects") == {owner.subject: owner.file})
            and previous.get("direct_inventory") == record["direct_inventory"], "Replay differs from current direct enumeration.")
    if owner in CLOCK_ASSERTIONS:
        require(previous.get("candidate_sha256") == record["candidate_sha256"]
                and previous.get("source_sha256") == session.identity,
                "Broker clock enumeration differs from the complete current source.")
    review = session.receipt(args.reviewed_inventory)
    require(type(review) is dict and set(review) == {"schema", "candidate_sha256", "summary", "reviewer", "reconciliation"}
            and review.get("schema") == owner.schema and review.get("candidate_sha256") == record["candidate_sha256"]
            and review.get("summary") == summary and type(review.get("summary", {}).get("count")) is int
            and type(review.get("reviewer")) is str and 0 < len(review["reviewer"]) <= 200,
            "Replay requires exact independently reviewed source and complete direct inventory.")
    reconciliation = review["reconciliation"]
    require(type(reconciliation) is dict and set(reconciliation) == {"functions_and_guards", "derives_and_macros",
                "unsafe_and_cfg_omissions", "independent_tests", "remaining_gaps"}
            and all(type(values) is list and all(type(value) is str and 0 < len(value) <= 8192 for value in values)
                    for values in reconciliation.values())
            and all(reconciliation[key] for key in reconciliation if key != "remaining_gaps"),
            "Replay review must retain function/cfg/generation gaps and independent controls.")
    record["inventory_review"] = review
    complete = args.lane == f"{prefix}-replay"
    require(complete or (type(args.pilot_mutant) is str and args.pilot_mutant in inventory),
            "Pilot requires one exact complete-inventory mutant name.")
    names = list(inventory) if complete else [args.pilot_mutant]
    deadline = time.monotonic() + (LIMITS["mutation_wall_seconds"] if complete else 900)
    commands = replay_commands()
    def seconds(limit: int) -> int:
        return C.remaining_timeout(deadline, limit)
    # Cargo owns invalidation; original source mtimes and dependency artifacts stay untouched.
    record["initial_target"] = initial_replay_target(session.work)
    # Full baseline uses the immutable original. Doctests remain a distinct command.
    for label, command in zip(("Build", "Test"), commands, strict=True):
        baseline = session.run(f"{prefix}-baseline:" + label, command, seconds(90 if label == "Build" else 60))
        if label == "Build":
            baseline_artifacts = fixture_build_artifacts(baseline["output"], "/work/source/" + Q.TOOL)
            record["baseline_fresh_fixture_artifacts"] = len(baseline_artifacts)
        if label == "Test":
            runtime = fixture_test_runtime(baseline["output"], "/work/source/" + Q.TOOL, baseline_artifacts,
                                           complete=True, verbose=False)
            executed_tests(runtime, session.tests)
    for command in Q.lane_commands("fixture-doc", "/toolchain/bin/cargo"):
        docs = session.run(f"{prefix}-baseline:doc", command, seconds(60))
        if re.search(r"^test result: ok\. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;", docs["output"], re.M):
            require(not re.search(r"^test (?!result:)", docs["output"], re.M), "Empty doctest report includes a case.")
            record["baseline_doctests"] = {"executed": 0, "behavior_credit": 0}
        else:
            record["baseline_doctests"] = {"names": executed_tests(docs["output"]), "behavior_credit": 0}
    rows = {}
    record["replay_rows"] = rows
    replay_error = None
    try:
        for number, name in enumerate(names):
            derivative = session.work / f"{prefix}-variant-{number:03d}"
            shutil.copytree(session.source, derivative)
            (derivative / owner.path).write_bytes(inventory[name]["source"])
            identity = {**session.identity, owner.path: inventory[name]["source_sha256"]}
            clean = replay_command(session, derivative, identity, f"{prefix}:{number}:Clean", replay_clean_command(), seconds(90))
            successful(clean)
            build = replay_command(session, derivative, identity, f"{prefix}:{number}:Build", commands[0], seconds(90))
            test = None
            if type(build.get("exit_code")) is int and build["exit_code"] == 0 and build.get("failure") is None:
                fresh_fixture_build(build["output"])
                test = replay_command(session, derivative, identity, f"{prefix}:{number}:Test", commands[1], seconds(60))
            row = {"raw": inventory[name]["raw"], "variant_sha256": Q.canonical(identity),
                   "source_sha256": inventory[name]["source_sha256"], "Clean": clean, "Build": build, "Test": test}
            rows[name] = row
            row.update(replay_outcome(build, test, session.tests, owner=owner))
            check_variant(session, derivative, identity)
            retire_variant(session, derivative, identity, number, row, owner=owner)
    except Exception as error:
        replay_error = error
        raise
    finally:
        # Fresh original Build/Test proves exact restoration and a healthy closing neighbor.
        try:
            session.run(f"{prefix}-closing:Clean", replay_clean_command(), seconds(90))
            for label, command in zip(("Build", "Test"), commands, strict=True):
                closing = session.run(f"{prefix}-closing:" + label, command, seconds(90 if label == "Build" else 60))
                if label == "Build":
                    closing_artifacts = fixture_build_artifacts(closing["output"], "/work/source/" + Q.TOOL)
                    record["closing_fresh_fixture_artifacts"] = len(closing_artifacts)
                if label == "Test":
                    runtime = fixture_test_runtime(closing["output"], "/work/source/" + Q.TOOL, closing_artifacts,
                                                   complete=True, verbose=False)
                    executed_tests(runtime, session.tests)
        except Exception as closing_error:
            record["closing_error"] = type(closing_error).__name__ + ": " + str(closing_error)
            if replay_error is None:
                raise
    record["replay_score"] = replay_score(rows, inventory, complete=complete, owner=owner)
    if complete:
        require(record["replay_score"]["gate_passed"], f"Full direct {prefix} replay remains below its 95% gate.")
        record["metrics_passed"] = True
    else:
        require(any(row["kind"] == "killed" for row in rows.values()), "Representative pilot has no semantic fault detection.")
        record["pilot_passed"] = True


def mutation_base_command(admitted, subject):
    """Construct one exact standalone owner command, with lint capping disabled."""
    require(subject in NATIVE_SUBJECTS, "Unknown fixture mutation owner.")
    return [str(admitted["extensions"]["cargo-mutants"]), "mutants", "--package", PACKAGE,
            "--file", SUBJECTS[subject], "--test-workspace", "false", "--cap-lints", "false",
            "--jobs", "4", "--jobserver", "true", "--jobserver-tasks", "4",
            "--cargo-arg=--message-format=json", "--cargo-arg=--frozen"]


def native_mutation_run(session, subject, seconds):
    """Mutate only an owned complete derivative; retain every failed/dirty observation."""
    require(subject in NATIVE_SUBJECTS, "Unknown native fixture owner.")
    session.check()
    derivative = session.work / "native-source"
    if "native_derivative" not in session.record:
        require(not derivative.exists() and not derivative.is_symlink(), "Native derivative already exists.")
        shutil.copytree(session.source, derivative)
        session.record["native_derivative"] = {"path": str(derivative), "identity_sha256": Q.canonical(session.identity),
                                               "runs": [], "retired": False, "mutation_jobs": 1}
    state = session.record["native_derivative"]
    require(state["path"] == str(derivative) and not state["retired"]
            and derivative.resolve(strict=True) == derivative
            and C.PRODUCT.inventory(derivative) == session.identity,
            "Native derivative is not completely restored before mutation.")
    observation = {"subject": subject, "before_sha256": Q.canonical(session.identity), "restored": False}
    state["runs"].append(observation)
    command = Q.native_mutation_command(session.admitted, subject)
    try:
        result = recorded_command(session, derivative, "mutation:" + subject, command, seconds,
                                  mutation=True, cwd=derivative / Q.TOOL, native_subject=subject)
    except Exception as error:
        observation["command_error"] = type(error).__name__ + ": " + str(error)
        raise
    finally:
        try:
            session.check()
            actual = C.PRODUCT.inventory(derivative)
            observation["after_sha256"] = Q.canonical(actual)
            require(actual == session.identity, "Native mutation did not restore its complete derivative.")
            observation["restored"] = True
        except Exception as error:
            observation["custody_error"] = type(error).__name__ + ": " + str(error)
            raise
    successful(result, mutation=True)
    return result


def retire_native_derivative(session):
    """Retire a completely restored private copy once; never heal or delete dirty input."""
    session.check()
    derivative = session.work / "native-source"
    state = session.record["native_derivative"]
    require(state["path"] == str(derivative) and not state["retired"]
            and derivative.resolve(strict=True) == derivative
            and C.PRODUCT.inventory(derivative) == session.identity,
            "Native derivative is dirty or outside its retirement owner.")
    shutil.rmtree(derivative)
    state["retired"] = True
    session.check()


class Session:
    """One fresh fixture metric epoch, sharing the service runner's cache lock."""

    def __init__(self, args, record, output):
        self.args, self.record, self.output = args, record, output
        self.work = output / "work"
        self.work.mkdir(mode=0o700)
        self.policy, self.identity = Q.source_identity()
        self.admitted = Q.admit_service_tools()
        self.tools = Q.tool_identity(self.admitted)
        self.vendor = args.vendor_dir.resolve(strict=True)
        self.vendor_identity = C.PRODUCT.admit_vendor(self.vendor, self.policy)
        self.observations = {}
        for name, value in (("toolchain-inventory.json", self.admitted["snapshot"]),
                            ("vendor-inventory.json", self.vendor_identity)):
            path = self.work / name
            Q.write_new(path, value)
            self.observations[name] = C.PRODUCT.sha256(path)
        self.source = self.work / "source"
        self.source.mkdir(mode=0o700)
        for name in self.identity:
            target = self.source / name
            target.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(Q.ROOT / name, target)
        (self.work / "cargo-home").mkdir(mode=0o700)
        self.config = self.work / "cargo-home/config.toml"
        self.config.write_text('[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory="/vendor"\n')
        self.receipts = {}
        self.proofs = {}
        record.update(source_sha256=self.identity, candidate_sha256=Q.canonical(self.identity),
                      recipe_sha256=self.identity[Q.SELF], config_sha256=C.PRODUCT.sha256(self.config),
                      feature_delta=Q.manifest_admission(self.policy), tools=self.tools,
                      vendor={"path": str(self.vendor), "inventory_sha256": Q.canonical(self.vendor_identity),
                              "files": len(self.vendor_identity)}, index_before=Q.index_identity(),
                      retention_helper_sha256=C.PRODUCT.sha256(Q.ROOT / EXPORT_HELPER),
                      retained_inventory_sha256=self.observations)
        self.binding = {key: record[key] for key in ("candidate_sha256", "recipe_sha256", "config_sha256",
                                                    "feature_delta", "vendor", "tools")}
        record["graph_binding"] = self.binding
        graph = self.receipt(args.graph_receipt)
        require_source_receipt(graph, self.binding, self.identity)
        Q.receive_graph(graph, self.binding, self.policy, "/toolchain/bin/cargo")
        graph_path = str(args.graph_receipt)
        self.tests = ordinary_fixture_tests(self.receipt(args.test_receipt), self.binding, self.identity,
                                            graph_path, self.receipts[graph_path])
        record["ordinary_tests"] = self.tests
        self.check()

    def receipt(self, path):
        """Bind a main-repository immutable record before consuming it."""
        require(path is not None and path.is_absolute() and path.resolve(strict=True) == path
                and path.parent == Q.RECORDS, "Select a canonical existing main-repository receipt.")
        data = C.read_analysis_bytes(path, maximum=64 * 1024 * 1024)
        self.receipts[str(path)] = hashlib.sha256(data).hexdigest()
        self.record["received_receipts"] = dict(self.receipts)
        return decode_record(data)

    def check(self):
        """Recheck source/configuration/staging and received evidence bookends."""
        require(Q.source_identity()[1] == self.identity and C.PRODUCT.inventory(self.source) == self.identity,
                "Complete fixture/product source inventory changed during metrics.")
        require(C.PRODUCT.sha256(self.config) == self.record["config_sha256"]
                and Q.index_identity() == self.record["index_before"], "Metric configuration or staging changed.")
        require(C.PRODUCT.sha256(Q.ROOT / EXPORT_HELPER) == self.record["retention_helper_sha256"],
                "Evidence export helper changed during metrics.")
        require(all(C.PRODUCT.sha256(self.work / name) == digest for name, digest in self.observations.items()),
                "Retained tool/vendor inventory observation changed during metrics.")
        require(all(C.PRODUCT.sha256(Path(path)) == digest for path, digest in self.receipts.items()),
                "Received metric evidence changed during metrics.")
        require(all(hashlib.sha256(C.read_analysis_bytes(Q.ROOT / path, maximum=1024 * 1024)).hexdigest() == digest
                    for path, digest in self.proofs.items()), "Reviewed equivalence proof changed during metrics.")
        require(all(hashlib.sha256(C.read_analysis_bytes(self.work / "equivalence-proofs" / path,
                                                        maximum=1024 * 1024)).hexdigest() == digest
                    for path, digest in self.proofs.items()), "Retained equivalence proof differs from reviewed bytes.")

    def run(self, label, command, seconds, *, mutation=False):
        """Retain actual command output even when the native command fails."""
        self.check()
        require(shutil.disk_usage(Q.ROOT).free >= Q.DISK_FLOOR, "Less than 4 GiB before metric command.")
        result = recorded_command(self, self.source, label, command, seconds,
                                  mutation=mutation, cwd=self.source / Q.TOOL)
        self.check()
        successful(result, mutation=mutation)
        return result

    def finish(self):
        """Complete vendor/tool/source custody without requiring historical machine hashes."""
        self.check()
        require(C.PRODUCT.admit_vendor(self.vendor, self.policy) == self.vendor_identity,
                "Reviewed vendor changed during metrics.")
        require(Q.tool_identity(Q.admit_service_tools()) == self.tools, "Tool payload changed during metrics.")
        self.check()
        self.record.update(bookends_passed=True, index_bytes_unchanged=True, index_after=Q.index_identity())


def enumerate_owners(session):
    """Enumerate all admitted owners under one finite wall-clock allowance."""
    deadline, inventories = time.monotonic() + LIMITS["enumeration_wall_seconds"], {}
    for subject in NATIVE_SUBJECTS:
        seconds = C.remaining_timeout(deadline, LIMITS["enumeration_wall_seconds"])
        result = session.run("enumerate:" + subject, [*mutation_base_command(session.admitted, subject),
                             "--list", "--json"], seconds)
        inventories[subject] = inventory_listing(result["output"], subject)
    summaries = inventory_summary(inventories)
    session.record.update(inventories=inventories, inventory_summary=summaries)
    return inventories, summaries


def execute(session):
    """Run one fixed finite metric lane after ordinary test and graph admission."""
    record, args = session.record, session.args
    owner = DIRECT_LANES.get(args.lane)
    if owner is not None:
        execute_direct(session, owner)
        return
    if args.lane in COVERAGE_LANES:
        if args.lane == "coverage":
            require(WRITER_ASSERTIONS.keys() <= set(session.tests), "Admitted test roster omits a writer control.")
        else:
            require(all(assertions.keys() <= set(session.tests) for assertions in CLOCK_ASSERTIONS.values()),
                    "Admitted test roster omits a broker clock control.")
        command = [str(session.admitted["extensions"]["cargo-llvm-cov"]), "llvm-cov", "--package", PACKAGE,
                   "--frozen", "--offline", "--jobs", "1", "--lib", "--tests", "--json",
                   "--output-path", "/work/coverage.json", "--failure-mode", "any", "--", "--test-threads=1"]
        result = session.run("coverage", command, LIMITS["coverage_wall_seconds"])
        record["coverage_tests"] = executed_tests(result["output"], session.tests)
        record["coverage"] = metric_coverage(C.read_analysis_json(session.work / "coverage.json"), args.lane)
        record["metrics_passed"] = True
        return
    inventories, summaries = enumerate_owners(session)
    if args.lane == "enumerate":
        record["review_template"] = {"schema_version": 1, "candidate_sha256": record["candidate_sha256"],
            "inventories": summaries, "reviewer": "", "reconciliation": {name: {
                "functions_and_guards": [], "derives_and_macros": [], "unsafe_and_cfg_omissions": [],
                "independent_tests": [], "remaining_gaps": []} for name in NATIVE_SUBJECTS}}
        return
    previous = session.receipt(args.inventory_receipt)
    require(previous.get("status") == "passed" and previous.get("lane") == "enumerate"
            and previous.get("bookends_passed") is True and previous.get("export_complete") is True
            and previous.get("graph_binding") == session.binding and previous.get("inventories") == inventories
            and previous.get("inventory_summary") == summaries,
            "Campaign inventory differs from the retained complete enumeration.")
    review = receive_review(session.receipt(args.reviewed_inventory), record["candidate_sha256"], summaries)
    record["inventory_review"] = review
    equivalences = []
    if args.equivalence_review is not None:
        equivalence_review = session.receipt(args.equivalence_review)
        equivalences = admit_equivalence_review(equivalence_review, record["candidate_sha256"],
                                                inventories, session.identity, Q.ROOT)
        record["equivalence_review"] = equivalence_review
        for entry in equivalences:
            proof = entry["proof"]
            session.proofs[proof["path"]] = proof["sha256"]
        for name, digest in session.proofs.items():
            data = C.read_analysis_bytes(Q.ROOT / name, maximum=1024 * 1024)
            require(hashlib.sha256(data).hexdigest() == digest, "Proof changed while retaining reviewed bytes.")
            destination = session.work / "equivalence-proofs" / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            with destination.open("xb") as stream:
                stream.write(data)
        record["retained_equivalence_proofs"] = dict(session.proofs)
    record["mutation_equivalences"] = equivalences
    (session.work / "mutation").mkdir(mode=0o700)
    deadline = time.monotonic() + LIMITS["mutation_wall_seconds"]
    record["mutation"] = {}
    for subject, filename in NATIVE_SUBJECTS.items():
        result = native_mutation_run(session, subject,
                                     C.remaining_timeout(deadline, LIMITS["mutation_wall_seconds"]))
        directory = session.work / "mutation" / subject / "mutants.out"
        outcomes = C.read_analysis_json(directory / "outcomes.json")
        selected_equivalences = [entry for entry in equivalences if entry["mutant"]["file"] == filename]
        score = C.mutation_counts(SCOPE, outcomes, inventories[subject], filename,
                                 native_exit_code=result["exit_code"], equivalences=selected_equivalences, defer_gate=True)
        baseline = next(row for row in outcomes["outcomes"] if row["scenario"] == "Baseline")
        baseline_log = owned_log(directory, baseline.get("log_path", "")).decode("utf-8")
        score["baseline_tests"] = executed_tests(native_log_phases(baseline, baseline_log)["Test"],
                                                 session.tests, allow_doctests=True)
        score["failure_causes"] = mutation_causes(directory, outcomes, session.tests, selected_equivalences)
        record["mutation"][subject] = score
    retire_native_derivative(session)
    require(all(row["gate_passed"] for row in record["mutation"].values()),
            "Complete fixture mutation accounting has unresolved survivors or a score below 95 percent.")
    record["metrics_passed"] = True


def durable_roots(cache, records, label):
    """Admit existing evidence roots on the WSL distro's block-backed ext4 root mount."""
    def mount_id(directory):
        descriptor = os.open(directory, os.O_DIRECTORY | os.O_NOFOLLOW)
        try:
            info = Path(f"/proc/self/fdinfo/{descriptor}").read_text()
            match = re.search(r"^mnt_id:\s*(\d+)$", info, re.MULTILINE)
            require(match is not None, "Cannot identify the retained directory mount.")
            return match[1]
        finally:
            os.close(descriptor)

    require(not cache.is_relative_to(records) and not records.is_relative_to(cache),
            "Durable work and record directories must be separate.")
    root_id = mount_id(Path("/"))
    for path in (cache, records):
        E.directory(path)
        require(mount_id(path) == root_id, "Durable metrics require the distro root mount for work and receipts.")
    selected = (cache / "writer.lock", cache / label, records / (label + "-evidence"), records / (label + ".json"),
                records / "durable-metric-stages")
    with Path("/proc/self/mountinfo").open("rb") as stream:
        table = stream.read(1024 ** 2 + 1)
    require(len(table) <= 1024 ** 2, "Mount table exceeds observation bound.")
    fields, backing = [], []
    for line in table.decode().splitlines():
        mount, filesystem = line.split(" - ")
        entry = mount.split()
        point = Path(re.sub(r"\\([0-7]{3})", lambda match: chr(int(match[1], 8)), entry[4]))
        require(not any(point.is_relative_to(path) for path in selected),
                "Selected durable metric paths contain a nested mount.")
        if entry[0] == root_id:
            fields, backing = entry, filesystem.split()
    require(fields[3:5] == ["/", "/"] and backing[0] == "ext4"
            and re.fullmatch(r"/dev/(sd[a-z]+[0-9]*|nvme[0-9]+n[0-9]+(?:p[0-9]+)?)", backing[1]),
            "Durable metrics require the supported ext4 distro disk.")
    device = Path(backing[1]).stat()
    require(stat.S_ISBLK(device.st_mode) and device.st_rdev == Path("/").stat().st_dev,
            "Distro root mount backing device differs.")
    return {"mount_id": root_id, "device": backing[1], "filesystem": backing[0],
            "work_root": str(cache), "record_root": str(records)}


def boot_id():
    """Observe the kernel clock epoch used by an absolute monotonic deadline."""
    value = Path("/proc/sys/kernel/random/boot_id").read_text().strip()
    require(re.fullmatch(r"[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}", value), "Invalid Linux boot ID.")
    return value


def bind_stage(records, stage_id, deadline, *, create):
    """Pin the first stage/boot/deadline once; repeated labels use the same bytes."""
    expected = {"stage_id": stage_id, "boot_id": boot_id(), "deadline_monotonic": deadline}
    directory = records / "durable-metric-stages"
    if create:
        directory.mkdir(mode=0o700, exist_ok=True)
    E.directory(directory)
    stage_directory = directory / stage_id
    path = stage_directory / "binding.json"
    if create:
        try:
            stage_directory.mkdir(mode=0o700)
        except FileExistsError:
            pass
        else:
            Q.write_new(path, expected)
    E.directory(stage_directory)
    data = C.read_analysis_bytes(path, maximum=4096)
    require(decode_record(data) == expected, "Durable metric stage boot or absolute deadline changed.")
    return {**expected, "binding_sha256": hashlib.sha256(data).hexdigest()}


@contextmanager
def stage_time(deadline, activity):
    """Interrupt in-process metric work on the same absolute cutoff as its children."""
    if deadline is None:
        yield
        return
    require(signal.getitimer(signal.ITIMER_REAL) == (0.0, 0.0),
            "Durable metrics require ownership of the process timer.")
    previous = signal.signal(signal.SIGALRM, Q.stage_expired)
    try:
        signal.setitimer(signal.ITIMER_REAL, Q.wall_timeout(math.inf, deadline))
        yield
        Q.wall_timeout(1, deadline)
    except Q.StageExpired as error:
        raise Incomplete(f"Cumulative metric deadline exhausted during {activity}.") from error
    finally:
        signal.setitimer(signal.ITIMER_REAL, 0)
        signal.signal(signal.SIGALRM, previous)


def run(args):
    """Own one locked fresh epoch and retain complete or failed metric evidence."""
    Q.no_ambient_flags()
    core_policy = Q.disable_core_dumps()
    require(re.fullmatch(r"[a-z0-9][a-z0-9-]{0,63}", args.label) is not None, "Invalid fresh metric label.")
    require(args.lane in METRIC_LANES, "Unknown fixture metric lane.")
    owner = DIRECT_LANES.get(args.lane)
    require(args.lane != "mutation" or (args.inventory_receipt is not None and args.reviewed_inventory is not None),
            "Mutation requires an enumeration receipt and an independent inventory review.")
    require(args.equivalence_review is None or args.lane == "mutation",
            "Exact equivalence review applies only to the mutation lane.")
    require(owner is None or args.lane == f"{owner.prefix}-enumerate" or (args.direct_inventory is not None
            and args.reviewed_inventory is not None), "Replay requires direct enumeration and independent review.")
    deadline = getattr(args, "deadline_monotonic", None)
    stage_id = getattr(args, "durable_stage", None)
    require((stage_id is None) == (deadline is None), "Durable metrics require both stage ID and absolute deadline.")
    if stage_id is not None:
        require(re.fullmatch(r"[a-z0-9][a-z0-9-]{0,63}", stage_id), "Invalid durable metric stage ID.")
        Q.wall_timeout(1, deadline)
    durable = durable_roots(Q.CACHE, Q.RECORDS, args.label) if stage_id is not None else None
    Q.CACHE.mkdir(mode=0o700, parents=True, exist_ok=True)
    E.directory(Q.CACHE)
    require(Q.CACHE.stat().st_mode & 0o077 == 0 and Q.RECORDS.is_dir(), "Private cache/record root is not ready.")
    if stage_id is None:
        require(shutil.disk_usage(Q.ROOT).free >= Q.DISK_FLOOR, "Less than 4 GiB before metric admission.")
    destination = Q.RECORDS / (args.label + ".json")
    evidence = Q.RECORDS / (args.label + "-evidence")
    archive = evidence / "raw"
    output = Q.CACHE / args.label
    writer = os.open(Q.CACHE / "writer.lock", os.O_CREAT | os.O_RDWR | os.O_CLOEXEC | os.O_NOFOLLOW, 0o600)
    try:
        fcntl.flock(writer, fcntl.LOCK_EX | fcntl.LOCK_NB)
        if durable is not None:
            stage = bind_stage(Q.RECORDS, stage_id, deadline, create=True)
            Q.wall_timeout(1, deadline)
            require(shutil.disk_usage(Q.CACHE).free >= 6 * 1024 ** 3,
                    "Durable fixture metrics require 6 GiB before launch.")
        require(not destination.exists() and not evidence.exists() and not output.exists(), "Metric label already owns evidence.")
        output.mkdir(mode=0o700)
        evidence.mkdir(mode=0o700)
        selected = ({owner.subject: owner.file} if owner is not None
                    else COVERAGE_LANES.get(args.lane, NATIVE_SUBJECTS))
        record = {"schema_version": 1, "scope": "portable fixture selected owners only", "lane": args.lane,
                  "core_dump_policy": core_policy,
                  "status": "incomplete", "started_unix": time.time(), "commands": [], "limits": LIMITS,
                  "work": str(output / "work"), "subjects": selected, "metrics_passed": False,
                  "qualification_passed": False, "bookends_passed": False, "index_bytes_unchanged": False,
                  "native_execution": False, "formal_execution": False,
                  "remaining_qualification": ["Independent function/guard, derive/macro and unsafe/cfg omission reconciliation.",
                                              "Windows-native service and memory/lifecycle evidence remain separate."]}
        record["remaining_qualification"].extend(CLOCK_NATIVE_GAPS[subject]
                                                 for subject in selected if subject in CLOCK_NATIVE_GAPS)
        if durable is not None:
            record.update(durable_roots=durable, deadline_monotonic=deadline, durable_stage=stage)
        errors = []
        session = None
        try:
            with stage_time(deadline, "metric admission, execution and bookends"):
                session = Session(args, record, output)
                execute(session)
                session.finish()
            record["status"] = "passed"
        except (KeyboardInterrupt, SystemExit) as error:
            errors.append("Metric execution interrupted: " + type(error).__name__)
            raise
        except Exception as error:
            errors.append(type(error).__name__ + ": " + str(error))
        finally:
            try:
                with stage_time(deadline, "export"):
                    E.export_tree(output, archive, evidence, disk_floor=Q.DISK_FLOOR)
                    if durable is not None:
                        require(durable_roots(Q.CACHE, Q.RECORDS, args.label) == durable, "Durable metric mount identity changed.")
                        require(bind_stage(Q.RECORDS, stage_id, deadline, create=False) == stage,
                                "Durable metric stage binding changed during execution.")
                record.update(export_complete=True, archive=str(archive))
            except (KeyboardInterrupt, SystemExit) as error:
                errors.append("Evidence export interrupted: " + type(error).__name__)
                raise
            except Exception as error:
                errors.append("Evidence export: " + type(error).__name__ + ": " + str(error))
            finally:
                record.update(errors=errors, finished_unix=time.time())
                if errors or record["status"] != "passed":
                    record.update(status="incomplete", metrics_passed=False, qualification_passed=False)
                    if "pilot_passed" in record:
                        record["pilot_passed"] = False
                Q.write_new(destination, record)
                print(json.dumps({"record": str(destination), "status": record["status"],
                                  "metrics_passed": record["metrics_passed"]}), flush=True)
        return record["status"]
    finally:
        os.close(writer)


def main():
    """Accept fixed lanes and fresh evidence selections, never arbitrary commands."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("lane", choices=METRIC_LANES)
    parser.add_argument("--label", required=True)
    parser.add_argument("--graph-receipt", type=Path, required=True)
    parser.add_argument("--test-receipt", type=Path, required=True)
    parser.add_argument("--vendor-dir", type=Path, required=True)
    parser.add_argument("--durable-stage", help="Stable ID for all lanes under one original boot and deadline.")
    parser.add_argument("--deadline-monotonic", type=float,
                        help="Use the durable WSL distro disk and the original stage's absolute monotonic deadline.")
    parser.add_argument("--direct-inventory", type=Path)
    parser.add_argument("--pilot-mutant")
    parser.add_argument("--inventory-receipt", type=Path)
    parser.add_argument("--reviewed-inventory", type=Path)
    parser.add_argument("--equivalence-review", type=Path,
                        help="Optional exact independently reviewed source/inventory/proof-bound equivalents.")
    args = parser.parse_args()
    try:
        return 0 if run(args) == "passed" else 1
    except Exception as error:
        print(type(error).__name__ + ": " + str(error), file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
