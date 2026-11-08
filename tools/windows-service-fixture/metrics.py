#!/usr/bin/env python3
"""Measure only the changed portable fixture host/error owners (GPL-2.0-only).

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
import time

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
SUBJECTS = {"host": "src/host.rs", "error": "src/error.rs"}
SCOPE = C.Scope(PACKAGE, tuple(SUBJECTS.items()), "fixture-local")
COVERAGE_SCOPE = C.Scope(PACKAGE, tuple((k, Q.TOOL + "/" + v) for k, v in SUBJECTS.items()), "fixture-local")
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


def executed_tests(text, expected=None, *, allow_doctests=False):
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
    require(summaries and names and sum(summaries) == len(names) and len(set(names)) == len(names),
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


def metric_coverage(report):
    """Score exact owner files independently, preserving all other raw line data."""
    return {name: C.coverage_counts(COVERAGE_SCOPE, report, "/work/source/" + Q.TOOL + "/" + filename)
            for name, filename in SUBJECTS.items()}


def inventory_listing(text, subject):
    """Admit every native descriptor, with no arbitrary filters or empty owner."""
    require(subject in SUBJECTS, "Unknown fixture owner.")
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
    require(type(inventories) is dict and set(inventories) == set(SUBJECTS),
            "Both complete fixture inventories are required.")
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
    require(type(review["inventories"]) is dict and set(review["inventories"]) == set(SUBJECTS)
            and all(type(row) is dict and set(row) == {"count", "sha256"}
                    and type(row["count"]) is int and type(row["sha256"]) is str
                    for row in review["inventories"].values()), "Review inventory counters must be exact integers.")
    require(type(reconciled) is dict and set(reconciled) == set(SUBJECTS),
            "Source reconciliation must cover both owners.")
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
    require(type(review["inventories"]) is dict and set(review["inventories"]) == set(SUBJECTS)
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


def selected_span(value):
    """Recognize only source/test paths in this standalone fixture namespace."""
    if type(value) is not str or ".." in PurePosixPath(value).parts:
        return False
    files = ("src/host.rs", "src/error.rs", "src/host_tests.rs", "src/error_tests.rs", "tests/entrypoints.rs")
    return value in files or any(value == prefix + filename for filename in files
               for prefix in ("/work/source/" + Q.TOOL + "/", Q.TOOL + "/"))


def owned_log(directory, value):
    """Read a bounded actual log without following paths outside its campaign."""
    relative = C.analysis_output_path(value)
    require(not relative.is_absolute() and relative.parts and ".." not in relative.parts,
            "Mutation log path escapes its campaign.")
    return C.read_analysis_bytes(directory / relative, maximum=16 * 1024 * 1024)


def mutation_causes(directory, outcomes, expected_tests, equivalences=()):
    """Separate assertions, compiler-unviable faults and zero-credit equivalents."""
    causes = {}
    for name, (entry, row) in equivalent_misses(outcomes, equivalences).items():
        data = owned_log(directory, row.get("log_path", ""))
        causes[name] = {"kind": "reviewed-equivalent-survivor", "kill_credit": 0,
                        "equivalence_id": entry["id"], "proof": entry["proof"],
                        "log_sha256": hashlib.sha256(data).hexdigest()}
    for row in outcomes["outcomes"]:
        if row.get("summary") not in {"CaughtMutant", "Unviable"}:
            continue
        mutant = row["scenario"]["Mutant"]
        data = owned_log(directory, row.get("log_path", ""))
        text = data.decode("utf-8")
        require(not re.search(r"HARNESS_INCOMPLETE|Cannot allocate memory|memory allocation of [^\n]*failed|"
                              r"No space left on device|No such file or directory|Permission denied|"
                              r"failed to (?:run custom build command|execute process)|could not execute process|"
                              r"internal compiler error", text),
                "Mutation failure has an infrastructure cause.")
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
        digest = hashlib.sha256(data).hexdigest()
        if row["summary"] == "Unviable":
            attributed = []
            for item in diagnostics:
                spans = item["message"].get("spans")
                require(type(spans) is list and all(type(span) is dict for span in spans),
                        "Compiler error lacks source attribution.")
                primary = [span for span in spans if span.get("is_primary") is True]
                require(type(item.get("target")) is dict and item["target"].get("name") in {
                            "nevrotick_windows_service_fixture", "nevrotick-lab-s02-engine",
                            "nevrotick-lab-s02-broker", "entrypoints"}
                        and all(selected_span(span.get("file_name")) for span in primary),
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
            causes[mutant["name"]] = {"kind": "compiler-unviable", "kill_credit": 0, "log_sha256": digest}
            continue
        require(not diagnostics and not re.search(r"error\[E\d+\]|could not compile", text),
                "Caught mutant contains a compiler failure.")
        assertions = []
        for match in re.finditer(r"^---- ([^\r\n]+) stdout ----\n(.*?)(?=^---- |^failures:|\Z)", text, re.M | re.S):
            name, body = match.groups()
            if name not in expected_tests or f"test {name} ... FAILED" not in text:
                continue
            assertion = re.search(r"panicked at ([^\n]+?):(\d+):\d+:\nassertion[^\n]*failed", body)
            if assertion is not None and selected_span(assertion[1]):
                assertions.append({"test": name, "source": assertion[1], "line": int(assertion[2])})
        # Mutation-only nocapture can retain a meaningful assertion before a
        # later fatal custody guard aborts libtest. The abort itself supplies no
        # cause: require the actual ordinary test's named thread, selected source
        # and immediate assertion diagnostic. Generic panic/Result text is not
        # a substitute for this evidence.
        for match in re.finditer(r"^(?:test ([^\r\n]+?) \.\.\. )?thread '([^'\r\n]+)'(?: \(\d+\))? "
                                 r"panicked at ([^\r\n]+?):(\d+):\d+:\nassertion[^\r\n]*failed[^\r\n]*", text, re.M):
            prefix, name, source, line = match.groups()
            if name in expected_tests and (prefix is None or prefix == name) and selected_span(source):
                if not any(row["test"] == name and row["source"] == source and row["line"] == int(line)
                           for row in assertions):
                    assertions.append({"test": name, "source": source, "line": int(line),
                                       "kind": "unbuffered-semantic-assertion"})
        require(assertions, "Caught mutant lacks an actual selected semantic assertion.")
        causes[mutant["name"]] = {"kind": "behavioral-assertion", "causes": assertions, "log_sha256": digest}
    return causes


def mutation_base_command(admitted, subject):
    """Construct one exact standalone owner command, with lint capping disabled."""
    require(subject in SUBJECTS, "Unknown fixture mutation owner.")
    return [str(admitted["extensions"]["cargo-mutants"]), "mutants", "--package", PACKAGE,
            "--file", SUBJECTS[subject], "--test-workspace", "false", "--cap-lints", "false",
            "--jobs", "4", "--jobserver", "true", "--jobserver-tasks", "4",
            "--cargo-arg=--message-format=json", "--cargo-arg=--frozen"]


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
        Q.receive_graph(graph, self.binding, self.policy, "/toolchain/bin/cargo")
        tests = self.receipt(args.test_receipt)
        require(tests.get("status") == "passed" and tests.get("lane") == "fixture-test"
                and tests.get("bookends_passed") is True and tests.get("index_bytes_unchanged") is True
                and tests.get("graph_binding") == self.binding and not tests.get("custody_error"),
                "Metrics require current ordinary fixture tests first.")
        expected = Q.lane_commands("fixture-test", "/toolchain/bin/cargo")
        require(type(tests.get("commands")) is list and len(tests["commands"]) == len(expected),
                "Ordinary fixture test receipt has incomplete commands.")
        names = []
        for row, command in zip(tests["commands"], expected, strict=True):
            require(row.get("argv") == command, "Ordinary fixture test command differs.")
            successful(row)
            names.extend(executed_tests(row["output"]))
        self.tests = sorted(names)
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
        require(shutil.disk_usage(Q.ROOT).free >= Q.DISK_FLOOR, "Less than 2 GiB before metric command.")
        result = Q.run_command(self.work, command, self.admitted, self.vendor, self.source,
                               timeout=seconds, cpu_seconds=seconds, mutation=mutation,
                               cwd=self.source / Q.TOOL)
        self.record["commands"].append({"label": label, "argv": command, **result})
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
    """Enumerate both owners completely under one finite wall-clock allowance."""
    deadline, inventories = time.monotonic() + LIMITS["enumeration_wall_seconds"], {}
    for subject in SUBJECTS:
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
    if args.lane == "coverage":
        command = [str(session.admitted["extensions"]["cargo-llvm-cov"]), "llvm-cov", "--package", PACKAGE,
                   "--frozen", "--offline", "--jobs", "1", "--lib", "--tests", "--json",
                   "--output-path", "/work/coverage.json", "--failure-mode", "any", "--", "--test-threads=1"]
        result = session.run("coverage", command, LIMITS["coverage_wall_seconds"])
        record["coverage_tests"] = executed_tests(result["output"], session.tests)
        record["coverage"] = metric_coverage(C.read_analysis_json(session.work / "coverage.json"))
        record["metrics_passed"] = True
        return
    inventories, summaries = enumerate_owners(session)
    if args.lane == "enumerate":
        record["review_template"] = {"schema_version": 1, "candidate_sha256": record["candidate_sha256"],
            "inventories": summaries, "reviewer": "", "reconciliation": {name: {
                "functions_and_guards": [], "derives_and_macros": [], "unsafe_and_cfg_omissions": [],
                "independent_tests": [], "remaining_gaps": []} for name in SUBJECTS}}
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
    for subject, filename in SUBJECTS.items():
        result = session.run("mutation:" + subject, [*mutation_base_command(session.admitted, subject),
                             "--baseline", "run", "--build-timeout", "90", "--timeout", "60",
                             "--output", "/work/mutation/" + subject],
                             C.remaining_timeout(deadline, LIMITS["mutation_wall_seconds"]), mutation=True)
        directory = session.work / "mutation" / subject / "mutants.out"
        outcomes = C.read_analysis_json(directory / "outcomes.json")
        selected_equivalences = [entry for entry in equivalences if entry["mutant"]["file"] == filename]
        equivalent_misses(outcomes, selected_equivalences)
        score = C.mutation_counts(SCOPE, outcomes, inventories[subject], filename,
                                 native_exit_code=result["exit_code"], equivalences=selected_equivalences, defer_gate=True)
        baseline = next(row for row in outcomes["outcomes"] if row["scenario"] == "Baseline")
        score["baseline_tests"] = executed_tests(owned_log(directory, baseline.get("log_path", "")).decode("utf-8"),
                                                 session.tests, allow_doctests=True)
        score["failure_causes"] = mutation_causes(directory, outcomes, session.tests, selected_equivalences)
        record["mutation"][subject] = score
    require(all(row["gate_passed"] for row in record["mutation"].values()),
            "Complete fixture mutation accounting has unresolved survivors or a score below 95 percent.")
    record["metrics_passed"] = True


def run(args):
    """Own one locked fresh epoch and retain complete or failed metric evidence."""
    Q.no_ambient_flags()
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))
    require(re.fullmatch(r"[a-z0-9][a-z0-9-]{0,63}", args.label) is not None, "Invalid fresh metric label.")
    require(args.lane in {"coverage", "enumerate", "mutation"}, "Unknown fixture metric lane.")
    require(args.lane != "mutation" or (args.inventory_receipt is not None and args.reviewed_inventory is not None),
            "Mutation requires an enumeration receipt and an independent inventory review.")
    require(args.equivalence_review is None or args.lane == "mutation",
            "Exact equivalence review applies only to the mutation lane.")
    Q.CACHE.mkdir(mode=0o700, parents=True, exist_ok=True)
    E.directory(Q.CACHE)
    require(Q.CACHE.stat().st_mode & 0o077 == 0 and Q.RECORDS.is_dir(), "Private cache/record root is not ready.")
    require(shutil.disk_usage(Q.ROOT).free >= Q.DISK_FLOOR, "Less than 2 GiB before metric admission.")
    destination = Q.RECORDS / (args.label + ".json")
    evidence = Q.RECORDS / (args.label + "-evidence")
    archive = evidence / "raw"
    output = Q.CACHE / args.label
    require(not destination.exists() and not evidence.exists() and not output.exists(), "Metric label already owns evidence.")
    writer = os.open(Q.CACHE / "writer.lock", os.O_CREAT | os.O_RDWR | os.O_CLOEXEC | os.O_NOFOLLOW, 0o600)
    try:
        fcntl.flock(writer, fcntl.LOCK_EX | fcntl.LOCK_NB)
        output.mkdir(mode=0o700)
        evidence.mkdir(mode=0o700)
        record = {"schema_version": 1, "scope": "portable fixture host/error only", "lane": args.lane,
                  "status": "incomplete", "started_unix": time.time(), "commands": [], "limits": LIMITS,
                  "work": str(output / "work"), "subjects": SUBJECTS, "metrics_passed": False,
                  "qualification_passed": False, "bookends_passed": False, "index_bytes_unchanged": False,
                  "native_execution": False, "formal_execution": False,
                  "remaining_qualification": ["Independent function/guard, derive/macro and unsafe/cfg omission reconciliation.",
                                              "Windows-native service and memory/lifecycle evidence remain separate."]}
        errors = []
        session = None
        try:
            session = Session(args, record, output)
            execute(session)
            session.finish()
            record["status"] = "passed"
        except Exception as error:
            errors.append(type(error).__name__ + ": " + str(error))
        finally:
            try:
                E.export_tree(output, archive, evidence)
                record.update(export_complete=True, archive=str(archive))
            except Exception as error:
                errors.append("Evidence export: " + type(error).__name__ + ": " + str(error))
            record.update(errors=errors, finished_unix=time.time())
            if errors:
                record["status"] = "incomplete"
                record["metrics_passed"] = False
            Q.write_new(destination, record)
            print(json.dumps({"record": str(destination), "status": record["status"],
                              "metrics_passed": record["metrics_passed"]}), flush=True)
        return record["status"]
    finally:
        os.close(writer)


def main():
    """Accept fixed lanes and fresh evidence selections, never arbitrary commands."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("lane", choices=("coverage", "enumerate", "mutation"))
    parser.add_argument("--label", required=True)
    parser.add_argument("--graph-receipt", type=Path, required=True)
    parser.add_argument("--test-receipt", type=Path, required=True)
    parser.add_argument("--vendor-dir", type=Path, required=True)
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
