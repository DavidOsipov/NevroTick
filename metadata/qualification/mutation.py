"""Qualify the complete default Cosmic Ray inventory on isolated metadata copies.

Run with the admitted, hash-pinned Cosmic Ray interpreter. This adapter uses the
vendor's unchanged enumeration and mutation implementations; no operator, source
line, function or generated mutant is filtered. Timeout/import/syntax failures
are reported separately and never become kills. A context map chooses tests
which reached the mutated line (or its containing function); unmatched scope
falls back to the complete suite. Unchanged survivors are not exclusions.
"""

from __future__ import annotations

import argparse
import ast
from collections import Counter
from concurrent.futures import ThreadPoolExecutor, as_completed
import difflib
import hashlib
import importlib.metadata
import json
import os
from pathlib import Path
import queue
import shutil
import signal
import subprocess
import sys
import time
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
RUNNER = Path(__file__).with_name("mutation_runner.py")
TOOL_VERSION = "8.7.0"
CATALOGUE = "docs/supporting-reviews/windows-service-metadata-catalogue.md"
TABLE_TEST = (
    "test_text_policy.TextPolicyTests."
    "test_embedded_unicode_table_is_structural_and_matches_the_pinned_derivation"
)
CRITICAL_FILES = {"metadata/pipeline/" + name for name in
                  ("__main__.py", "contract.py", "delivery.py", "fields.py", "generate.py", "text_policy.py")}
MODEL_TESTS = ["test_field_coverage.ExpandedModelTests." + name for name in (
    "test_all_seven_record_types_reject_ordinary_assignment_and_deletion",
    "test_all_seven_record_types_have_fixed_attribute_layout",
    "test_complete_field_register_has_reviewed_shape_values_and_catalogue_binding",
)]
ANNOTATION_TEST = (
    "test_typing_contracts.AnnotationContractTests."
    "test_owned_annotations_are_resolved_and_public_interfaces_are_present"
)


def digest(raw: bytes) -> str:
    return hashlib.sha256(raw).hexdigest()


def write_json(path: Path, value: Any) -> None:
    path.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")


def snapshot(root: Path, names: list[str]) -> dict[str, bytes]:
    """Freeze exact named bytes, refusing links and files outside this checkout."""
    result = {}
    for name in names:
        path = root / name
        if path.is_symlink() or not path.is_file() or path.resolve().is_relative_to(root) is False:
            raise ValueError("Nonregular or outside source")
        result[name] = path.read_bytes()
    return result


def functions(source: bytes) -> list[dict[str, Any]]:
    """Record every explicit function, including methods and nested helpers."""
    tree = ast.parse(source)
    result = []

    def visit(node: ast.AST, prefix: str = "") -> None:
        for child in ast.iter_child_nodes(node):
            if isinstance(child, (ast.FunctionDef, ast.AsyncFunctionDef, ast.ClassDef)):
                name = prefix + child.name
                if isinstance(child, (ast.FunctionDef, ast.AsyncFunctionDef)):
                    result.append({"name": name, "start": child.lineno, "end": child.end_lineno})
                visit(child, name + ".")
            else:
                visit(child, prefix)

    visit(tree)
    return result


def enumerate_mutants(source_root: Path, files: list[str]) -> tuple[list[dict[str, Any]], list[str]]:
    """Consume the pinned vendor's default inventory without replacement faults."""
    from cosmic_ray.commands.init import _all_work_items, _operators

    rows = []
    for work in _all_work_items([source_root / name for name in files], {}):
        item = work.mutations[0]
        name = item.module_path.relative_to(source_root).as_posix()
        identity = {
            "file": name,
            "operator": item.operator_name,
            "arguments": item.operator_args,
            "occurrence": item.occurrence,
            "start": list(item.start_pos),
            "end": list(item.end_pos),
            "definition": item.definition_name,
        }
        identity["id"] = digest(json.dumps(identity, sort_keys=True).encode())[:24]
        rows.append(identity)
    return rows, [name for name, _args, _operator in _operators({})]


def regions(source: bytes) -> list[dict[str, Any]]:
    """Expose module data, decorators and generated class behavior separately."""
    result = []
    for node in ast.parse(source).body:
        if isinstance(node, (ast.Assign, ast.AnnAssign, ast.ClassDef, ast.If)):
            label = type(node).__name__
            if isinstance(node, ast.ClassDef):
                label += ":" + node.name
            elif isinstance(node, ast.Assign):
                label += ":" + ",".join(ast.unparse(target) for target in node.targets)
            elif isinstance(node, ast.AnnAssign):
                label += ":" + ast.unparse(node.target)
            start = min([node.lineno] + [item.lineno for item in getattr(node, "decorator_list", [])])
            result.append({"region": label, "start": start, "end": node.end_lineno})
    return result


def annotation_spans(source: bytes) -> list[tuple[tuple[int, int], tuple[int, int]]]:
    """Locate authored annotation metadata for independent readback selection."""
    annotations = []
    for node in ast.walk(ast.parse(source)):
        if isinstance(node, ast.arg) and node.annotation:
            annotations.append(node.annotation)
        elif isinstance(node, (ast.FunctionDef, ast.AsyncFunctionDef)) and node.returns:
            annotations.append(node.returns)
        elif isinstance(node, ast.AnnAssign):
            annotations.append(node.annotation)
    return [((node.lineno, node.col_offset), (node.end_lineno, node.end_col_offset))
            for node in annotations]


def copy_source(destination: Path, sources: dict[str, bytes]) -> None:
    destination.mkdir()
    for name, raw in sources.items():
        target = destination / name
        target.parent.mkdir(parents=True, exist_ok=True)
        target.write_bytes(raw)


def process(command: list[str], cwd: Path, timeout: float, log: Path) -> dict[str, Any]:
    """Bound an owned process group, retaining its original output and exit."""
    started = time.monotonic()
    with log.open("wb") as output:
        child = subprocess.Popen(command, cwd=cwd, stdout=output, stderr=subprocess.STDOUT,
                                 start_new_session=True)
        timed_out = False
        try:
            code = child.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            timed_out = True
            os.killpg(child.pid, signal.SIGKILL)
            code = child.wait()
    return {"exit": code, "timeout": timed_out, "elapsed": time.monotonic() - started}


def execute_tests(python: Path, source: Path, names: list[str] | None,
                  directory: Path, timeout: float) -> dict[str, Any]:
    directory.mkdir()
    report = directory / "tests.json"
    command = [str(python), "-I", "-B", str(RUNNER), "--root", str(source), "--report", str(report)]
    if names:
        selection = directory / "selection.json"
        write_json(selection, names)
        command += ["--selection", str(selection)]
    result = process(command, source, timeout, directory / "tests.log")
    result["report"] = json.loads(report.read_text()) if report.exists() else None
    result["command"] = command
    return result


def context_selection(row: dict[str, Any], contexts: dict[str, Any],
                      scopes: dict[str, list[dict[str, Any]]]) -> tuple[list[str] | None, str]:
    """Use measured line reachability, with whole-function/full-suite fallback."""
    name = row["file"]
    if row.get("annotation_only"):
        return [ANNOTATION_TEST], "independent-stdlib-authored-type-contract-readback"
    if name.endswith("/fields.py") or any(low <= row["start"][0] <= high
                                             for low, high in contexts.get("record_spans", {}).get(name, [])):
        return MODEL_TESTS, "independent-seven-record-immutability-layout-and-complete-registry-cohort"
    # The table is executed during import, before unittest test contexts exist.
    # Its independent complete digest/shape test is a direct assertion on every
    # one of its values. This choice is explicit and retained in each work item.
    if name.endswith("/text_policy.py") and contexts.get("unicode_table_span", [0, 0])[0] <= row["start"][0] <= contexts.get("unicode_table_span", [0, 0])[1]:
        return [TABLE_TEST], "independent-complete-unicode-table-integrity"
    files = contexts.get("files", {})
    lines = files.get(name, {})
    reached = set()
    for line in range(row["start"][0], row["end"][0] + 1):
        reached.update(lines.get(str(line), []))
    rationale = "measured-mutated-line"
    if not reached:
        for function in scopes[name]:
            if function["start"] <= row["start"][0] <= function["end"]:
                for line in range(function["start"], function["end"] + 1):
                    reached.update(lines.get(str(line), []))
        rationale = "measured-containing-function"
    reached.discard("")
    if not reached:
        return None, "complete-suite-unmeasured-or-module-level-scope"
    durations = contexts.get("durations", {})
    return sorted(reached, key=lambda test: (durations.get(test, 1e6), test)), rationale


def load_contexts(path: Path | None, sources: dict[str, bytes]) -> dict[str, Any]:
    """Adapt coverage.py's retained per-line contexts without inventing reaches."""
    result: dict[str, Any] = {"files": {}, "durations": {}}
    if path:
        raw = json.loads(path.read_text())
        result["files"] = {name: row.get("contexts", row) for name, row in raw.get("files", {}).items()}
        test_result = path.with_name("test-result.json")
        result["durations"] = (json.loads(test_result.read_text()).get("test_durations", {})
                               if test_result.exists() else raw.get("durations", {}))
    for node in ast.parse(sources["metadata/pipeline/text_policy.py"]).body:
        targets = node.targets if isinstance(node, ast.Assign) else [node.target] if isinstance(node, ast.AnnAssign) else []
        if any(isinstance(target, ast.Name) and target.id == "_FORBIDDEN_RANGES" for target in targets):
            result["unicode_table_span"] = [node.lineno, node.end_lineno]
    result["record_spans"] = {}
    for name in ("metadata/pipeline/contract.py", "metadata/pipeline/fields.py"):
        spans = []
        for node in ast.parse(sources[name]).body:
            if isinstance(node, ast.ClassDef) and any(
                isinstance(decorator, ast.Call) and isinstance(decorator.func, ast.Name)
                and decorator.func.id == "dataclass" for decorator in node.decorator_list
            ):
                spans.append([min(decorator.lineno for decorator in node.decorator_list), node.end_lineno])
        result["record_spans"][name] = spans
    return result


def classify(result: dict[str, Any]) -> str:
    if result["timeout"]:
        return "timed-out"
    report = result["report"]
    if not report or report["run"] == 0 or report["discovered"] == 0:
        return "unviable-test-setup"
    if any("_FailedTest" in item["test"] for item in report["errors"]):
        return "unviable-test-import"
    if report["skipped"]:
        return "incomplete-skipped-tests"
    if report.get("expected_failures") or report.get("unexpected_successes"):
        return "incomplete-expected-failure-or-unexpected-success"
    traces = "\n".join(item["traceback"] for item in report["failures"] + report["errors"])
    if "subprocess.TimeoutExpired:" in traces or "TimeoutError:" in traces:
        return "timed-out-test-child"
    if "ModuleNotFoundError:" in traces or "ImportError:" in traces:
        return "startup-review-required"
    if result["exit"] == 0 and report["successful"]:
        return "survived"
    if report["failures"]:
        return "killed-assertion"
    if report["errors"]:
        return "killed-behavior-error"
    return "unviable-runner"


def fixture_anchor_failure(result: dict[str, Any]) -> bool:
    """Recognize only the reviewed production-source injection preconditions.

    A mutator changing the exact source text used by a different fault-injection
    fixture is not evidence that the tested product rejected the mutant.
    """
    report = result.get("report")
    if not report or report["errors"] or len(report["failures"]) != 1:
        return False
    failure = report["failures"][0]
    trace = failure["traceback"]
    return ((failure["test"].startswith("test_delivery.ProducerDeliveryCliTests.")
             and "self.assertEqual(text.count(old), 1)" in trace)
            or (failure["test"].startswith(("test_lifecycle.CodeIdentityLifecycleTests.",
                                           "test_lifecycle.PublicationFaultTests."))
                and "self.assertEqual(text.count(marker), 1)" in trace))


def apply_reviews(results: list[dict[str, Any]], sources: dict[str, bytes],
                  paths: list[Path]) -> list[dict[str, Any]]:
    """Apply only independently reviewed, exact-source/exact-mutant evidence.

    Review supplies the semantic argument. This reader checks custody and never
    invents an equivalence, changes an observed outcome or gives equivalent
    survivors kill credit.
    """
    by_id = {item["id"]: item for item in results}
    accepted = []
    seen = set()
    for path in paths:
        review = json.loads(path.read_text())
        if not review.get("independent_review"):
            raise ValueError("An independent review identity is required")
        evidence = Path(review["evidence"])
        for category, target in (("startup_semantic_kills", "killed-application-startup"),
                                 ("equivalent_survivors", "equivalent-reviewed")):
            for entry in review.get(category, []):
                identity = entry["id"]
                if identity not in by_id:
                    continue  # A bounded pilot may not select this reviewed ID.
                if identity in seen:
                    raise ValueError("Duplicate reviewed mutant")
                observed = by_id[identity]
                prior_path = evidence / "outcomes" / identity / "outcome.json"
                if digest(prior_path.read_bytes()) != entry["outcome_sha256"]:
                    raise ValueError("Independent outcome evidence changed")
                prior = json.loads(prior_path.read_text())
                if (review["source_sha256"] != digest(sources[observed["file"]])
                        or entry["mutant_sha256"] != observed["mutant_sha256"]
                        or prior["mutant_sha256"] != observed["mutant_sha256"]
                        or observed["outcome"] != entry["raw_outcome"]
                        or prior["outcome"] != entry["raw_outcome"]
                        or not entry.get("argument")):
                    raise ValueError("Reviewed disposition does not match actual source/mutant/outcome")
                if target == "equivalent-reviewed":
                    if observed["outcome"] != "survived" or not entry.get("invalidation"):
                        raise ValueError("Only a source-bound reviewed survivor can be equivalent")
                else:
                    probe = observed.get("startup_cli_positive", {}).get("report", {})
                    if (observed["outcome"] != "startup-review-required" or probe.get("run") != 1
                            or len(probe.get("failures", [])) != 1 or probe.get("errors")):
                        raise ValueError("A semantic startup kill needs its actual CLI positive assertion")
                observed["reviewed_outcome"] = target
                accepted.append({"id": identity, "decision": target, "review": str(path),
                                 "review_sha256": digest(path.read_bytes()), "argument": entry["argument"]})
                seen.add(identity)
    return accepted


def self_controls(python: Path, output: Path) -> dict[str, Any]:
    """Challenge runner classifications; these are tooling controls, not kills."""
    directory = output / "runner-controls"
    directory.mkdir()
    source = directory / "source"
    copy_source(source, {"metadata/tests/test_runner_control.py": (
        "import unittest\nclass Control(unittest.TestCase):\n"
        " def test_healthy(self): self.assertEqual(2+2,4)\n"
        " def test_assertion(self): self.assertEqual(2+2,5)\n"
        " @unittest.skip('owned-tooling-control')\n"
        " def test_skipped(self): pass\n"
        " @unittest.expectedFailure\n"
        " def test_expected(self): self.fail('owned-tooling-control')\n"
        " @unittest.expectedFailure\n"
        " def test_unexpected(self): pass\n"
    ).encode(), "metadata/tests/test_import_control.py": b"raise ImportError('owned-tooling-control')\n"})
    cases = {}
    for name, expected in (("test_runner_control.Control.test_healthy", "survived"),
                           ("test_runner_control.Control.test_assertion", "killed-assertion"),
                           ("test_runner_control.Control.test_skipped", "incomplete-skipped-tests"),
                           ("test_runner_control.Control.test_expected", "incomplete-expected-failure-or-unexpected-success"),
                           ("test_runner_control.Control.test_unexpected", "incomplete-expected-failure-or-unexpected-success"),
                           ("test_import_control", "unviable-test-import")):
        result = execute_tests(python, source, [name], directory / name.split(".")[-1], 10)
        observed = classify(result)
        cases[name] = {"expected": expected, "observed": observed, "result": result}
        if observed != expected:
            raise ValueError("Runner classification control failed")
    timeout = process([str(python), "-I", "-B", "-c", "import time;time.sleep(30)"],
                      source, 0.1, directory / "timeout.log")
    if not timeout["timeout"] or timeout["exit"] == 0:
        raise ValueError("Owned process-group timeout control failed")
    cases["timeout"] = timeout
    write_json(directory / "results.json", cases)
    return cases


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--python", type=Path, default=Path(sys.executable))
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--contexts", type=Path)
    parser.add_argument("--context-binding", type=Path,
                        help="Coverage binding.json proving exact canonical source hashes for the context map")
    parser.add_argument("--workers", type=int, default=10)
    parser.add_argument("--timeout", type=float, default=180)
    parser.add_argument("--limit", type=int)
    parser.add_argument("--enumerate-only", action="store_true")
    parser.add_argument("--baseline-only", action="store_true")
    parser.add_argument("--baseline-from", type=Path,
                        help="Reuse a passing full baseline with identical bound source/test/runner bytes")
    parser.add_argument("--controls-only", action="store_true")
    parser.add_argument("--ids", type=Path, help="Explicit replay IDs; inventory remains complete")
    parser.add_argument("--reviewed-dispositions", type=Path, action="append", default=[],
                        help="Independently reviewed exact-mutant startup/equivalence records")
    args = parser.parse_args()
    if importlib.metadata.version("cosmic-ray") != TOOL_VERSION:
        parser.error("The admitted Cosmic Ray version is required")
    if not 1 <= args.workers <= 10 or not 0 < args.timeout <= 600:
        parser.error("Workers must be 1..10 and timeout must be 0..600 seconds")
    if shutil.disk_usage(args.output.parent).free < 1024 ** 3:
        parser.error("At least 1GiB free disk is required")
    args.output.mkdir()
    self_controls(args.python, args.output)
    if args.controls_only:
        print(json.dumps({"controls": "passed", "output": str(args.output)}))
        return 0
    manifest = json.loads((ROOT / "metadata/qualification/manifest.json").read_text())
    if set(manifest["critical_files"]) != CRITICAL_FILES or set(manifest["source_modules"]) != CRITICAL_FILES | {"metadata/__init__.py", "metadata/pipeline/__init__.py"}:
        parser.error("The complete independent eight-module/six-critical-file scope is required")
    if manifest["minimum_adjusted_mutation_percent"] < 95:
        parser.error("The per-critical-file 95 percent minimum cannot be reduced")
    names = (manifest["source_modules"] + manifest["input_files"]
             + [path.relative_to(ROOT).as_posix() for path in sorted((ROOT / "metadata/tests").glob("*.py"))]
             + [CATALOGUE])
    sources = snapshot(ROOT, names)
    if args.contexts:
        if not args.context_binding:
            parser.error("A source-hash context binding is required with --contexts")
        binding = json.loads(args.context_binding.read_text())
        if binding["source_hashes"] != {name: digest(sources[name]) for name in manifest["source_modules"]}:
            parser.error("Coverage contexts do not match this frozen production source")
        write_json(args.output / "selection-source.json", {
            "contexts": str(args.contexts), "contexts_sha256": digest(args.contexts.read_bytes()),
            "binding": str(args.context_binding), "binding_sha256": digest(args.context_binding.read_bytes()),
            "source_hashes": binding["source_hashes"],
        })
    write_json(args.output / "candidate.json", {
        "sources": {name: digest(raw) for name, raw in sources.items()},
        "manifest_sha256": digest((ROOT / "metadata/qualification/manifest.json").read_bytes()),
        "driver_sha256": digest(Path(__file__).read_bytes()),
        "runner_sha256": digest(RUNNER.read_bytes()),
        "cosmic_ray": TOOL_VERSION, "python": sys.version,
    })
    frozen = args.output / "frozen"
    copy_source(frozen, sources)
    started = time.monotonic()
    rows, operators = enumerate_mutants(frozen, manifest["critical_files"])
    for name in manifest["critical_files"]:
        spans = annotation_spans(sources[name])
        for row in rows:
            if row["file"] == name:
                row["annotation_only"] = any(low <= tuple(row["start"]) and tuple(row["end"]) <= high
                                             for low, high in spans)
    scopes = {name: functions(sources[name]) for name in manifest["critical_files"]}
    reconciliation = []
    for name, functions_for_file in scopes.items():
        for function in functions_for_file:
            ids = [row["id"] for row in rows if row["file"] == name
                   and function["start"] <= row["start"][0] <= function["end"]]
            reconciliation.append({"file": name, **function, "generated_ids": ids,
                                   "alternative_required": not ids})
    write_json(args.output / "inventory.json", rows)
    write_json(args.output / "functions.json", reconciliation)
    data_regions = []
    for name in manifest["critical_files"]:
        for region in regions(sources[name]):
            ids = [row["id"] for row in rows if row["file"] == name
                   and region["start"] <= row["start"][0] <= region["end"]]
            data_regions.append({"file": name, **region, "generated_ids": ids,
                                 "alternative_required": not ids})
    write_json(args.output / "regions.json", data_regions)
    from cosmic_ray.plugins import get_operator, operator_names
    dormant = {name: [{"name": item.name, "description": item.description}
                      for item in get_operator(name).arguments()]
               for name in operator_names() if name not in operators}
    write_json(args.output / "generation.json", {"operators": operators,
               "unconfigured_parameterized_operators": dormant,
               "counts": dict(Counter(row["file"] for row in rows)),
               "elapsed": time.monotonic() - started,
               "filters": [], "operator_configuration": {}, "generated": len(rows)})
    if args.enumerate_only:
        print(json.dumps({"generated": len(rows), "output": str(args.output)}))
        return 0
    if args.baseline_from:
        previous = json.loads((args.baseline_from / "candidate.json").read_text())
        current = json.loads((args.output / "candidate.json").read_text())
        for key in ("sources", "manifest_sha256", "runner_sha256", "cosmic_ray", "python"):
            if previous[key] != current[key]:
                raise ValueError("Passing baseline identity no longer matches " + key)
        healthy = json.loads((args.baseline_from / "baseline.json").read_text())
        write_json(args.output / "baseline-reuse.json", {"from": str(args.baseline_from),
                   "candidate_sha256": digest((args.baseline_from / "candidate.json").read_bytes()),
                   "baseline_sha256": digest((args.baseline_from / "baseline.json").read_bytes())})
    else:
        healthy = execute_tests(args.python, frozen, None, args.output / "baseline", args.timeout)
    write_json(args.output / "baseline.json", healthy)
    if classify(healthy) != "survived":
        raise SystemExit("Unmodified complete suite did not pass; no mutants executed")
    healthy_ids = set(healthy["report"]["discovered_ids"])
    if args.baseline_only:
        print(json.dumps({"baseline": {key: healthy["report"][key]
                                      for key in ("discovered", "run", "successful", "elapsed")},
                          "output": str(args.output)}))
        return 0
    contexts = load_contexts(args.contexts, sources)
    contexts["durations"] = {**contexts.get("durations", {}), **healthy["report"]["test_durations"]}
    if args.contexts:
        write_json(args.output / "contexts.json", contexts)
    selected_rows = rows
    if args.ids:
        requested = set(json.loads(args.ids.read_text()))
        selected_rows = [row for row in rows if row["id"] in requested]
        if not requested or {row["id"] for row in selected_rows} != requested:
            parser.error("Nonempty replay IDs must all exist in this exact candidate inventory")
    if args.limit:
        selected_rows = selected_rows[:args.limit]
    selected = []
    for row in selected_rows:
        tests, reason = context_selection(row, contexts, scopes)
        if tests and not set(tests).issubset(healthy_ids):
            raise ValueError("Selected tests are outside the passing baseline discovery")
        row["tests"] = tests
        row["selection_reason"] = reason
        selected.append(row)
    write_json(args.output / "selected.json", selected)
    workers: queue.Queue[Path] = queue.Queue()
    for index in range(args.workers):
        worker = args.output / f"worker-{index}"
        copy_source(worker, sources)
        workers.put(worker)
    write_json(args.output / "cache-custody.json", {
        "copied_files": list(sources), "copied_cache_files": [],
        "python_flags": ["-I", "-B"],
        "rule": "Only explicit .py/TOML/catalogue files are copied; each worker is checked before and after every mutation for .pyc and __pycache__.",
    })
    outcomes = args.output / "outcomes"
    outcomes.mkdir()

    def run(row: dict[str, Any]) -> dict[str, Any]:
        from cosmic_ray.mutating import mutate_code
        from cosmic_ray.plugins import get_operator

        worker = workers.get()
        path = worker / row["file"]
        directory = outcomes / row["id"]
        directory.mkdir()
        original = sources[row["file"]].decode("utf-8")
        begun = time.monotonic()
        record = {"id": row["id"], "file": row["file"], "tests": row["tests"],
                  "selection_reason": row["selection_reason"]}
        try:
            if list(worker.rglob("*.pyc")) or list(worker.rglob("__pycache__")):
                raise ValueError("Unexpected Python cache in mutation source snapshot")
            mutated = mutate_code(original, get_operator(row["operator"])(**row["arguments"]), row["occurrence"])
            if mutated is None or mutated == original:
                record["outcome"] = "unviable-no-change"
                return record
            record["mutant_sha256"] = digest(mutated.encode())
            (directory / "mutation.diff").write_text("".join(difflib.unified_diff(
                original.splitlines(keepends=True), mutated.splitlines(keepends=True),
                fromfile="a/" + row["file"], tofile="b/" + row["file"])))
            try:
                compile(mutated, row["file"], "exec")
            except (SyntaxError, ValueError, TypeError) as error:
                record["outcome"] = "unviable-syntax"
                record["reason"] = repr(error)
                return record
            path.write_text(mutated, encoding="utf-8")
            import_code = ("import sys,importlib;sys.path.insert(0," + repr(str(worker))
                           + ");importlib.import_module(" + repr(row["file"][:-3].replace("/", ".")) + ")")
            imported = process([str(args.python), "-I", "-B", "-c", import_code],
                               worker, min(args.timeout, 15), directory / "import.log")
            record["import"] = imported
            if imported["timeout"]:
                record["outcome"] = "timed-out-import"
                return record
            if imported["exit"] != 0:
                # Collection/import failure alone supplies no behavioral kill.
                # Probe the external entrypoint with a known valid source using
                # a test module that does not import the production package.
                # The causal source/diff/trace needs independent disposition.
                probe = execute_tests(args.python, worker,
                                      ["test_pipeline.MetadataCliTests.test_source_stage_is_the_default"],
                                      directory / "startup-cli-positive", args.timeout)
                record["startup_cli_positive"] = probe
                record["startup_probe_classification"] = classify(probe)
                record["outcome"] = "startup-review-required"
                return record
            remaining = row["tests"]
            executions = []
            fixture_failures = []
            test_started = time.monotonic()
            while True:
                budget = args.timeout - (time.monotonic() - test_started)
                if budget <= 0:
                    record["outcome"] = "timed-out"
                    break
                result = execute_tests(args.python, worker, remaining,
                                       directory / f"execution-{len(executions)}", budget)
                executions.append(result)
                if not fixture_anchor_failure(result):
                    record["outcome"] = classify(result)
                    break
                fixture_failures.extend(result["report"]["failures"])
                executed = set(result["report"]["executed_ids"])
                remaining = [name for name in (remaining or healthy["report"]["discovered_ids"])
                             if name not in executed]
                if not remaining:
                    record["outcome"] = "survived"
                    record["fixture_only_unresolved"] = True
                    break
            record["executions"] = executions
            record["fixture_anchor_failures"] = fixture_failures
            if record["outcome"].startswith("unviable-test-"):
                record["setup_cli_positive"] = execute_tests(
                    args.python, worker, ["test_pipeline.MetadataCliTests.test_source_stage_is_the_default"],
                    directory / "setup-cli-positive", args.timeout)
                record["outcome"] = "setup-review-required"
            return record
        except Exception as error:
            record["outcome"] = "unviable-driver-exception"
            record["reason"] = repr(error)
            return record
        finally:
            path.write_bytes(sources[row["file"]])
            caches = [str(path.relative_to(worker)) for pattern in ("*.pyc", "__pycache__")
                      for path in worker.rglob(pattern)]
            record["unexpected_cache_files"] = caches
            if caches:
                record["outcome"] = "incomplete-source-cache-custody"
            record["elapsed"] = time.monotonic() - begun
            write_json(directory / "outcome.json", record)
            workers.put(worker)

    results = []
    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        futures = [pool.submit(run, row) for row in selected]
        for future in as_completed(futures):
            results.append(future.result())
            if len(results) % 100 == 0:
                print(json.dumps({"completed": len(results), "selected": len(selected),
                                  "outcomes": dict(Counter(item["outcome"] for item in results))}), flush=True)
    reviews = apply_reviews(results, sources, args.reviewed_dispositions)
    write_json(args.output / "reviewed-dispositions.json", reviews)
    per_file = {}
    for name in manifest["critical_files"]:
        counts = Counter(item["outcome"] for item in results if item["file"] == name)
        reviewed_counts = Counter(item.get("reviewed_outcome", item["outcome"])
                                  for item in results if item["file"] == name)
        killed = sum(value for key, value in reviewed_counts.items() if key.startswith("killed-"))
        unresolved = sum(value for key, value in reviewed_counts.items()
                         if key.endswith("review-required") or key.startswith(("timed-out", "incomplete-")))
        excluded = reviewed_counts["equivalent-reviewed"]
        viable = killed + reviewed_counts["survived"] + excluded + unresolved
        adjusted_viable = viable - excluded
        per_file[name] = {"generated": sum(row["file"] == name for row in rows),
                          "outcomes": dict(counts), "reviewed_outcomes": dict(reviewed_counts),
                          "viable": viable, "killed": killed,
                          "raw_percent": 100 * killed / viable if viable else None,
                          "adjusted_percent": 100 * killed / adjusted_viable if adjusted_viable else None,
                          "excluded_equivalent": excluded}
    minimum = max(95, manifest["minimum_adjusted_mutation_percent"])
    summary = {"selected": len(selected), "completed": len(results), "generated": len(rows),
               "minimum_per_file_percent": minimum,
               "complete_inventory_executed": len(results) == len(rows),
               "per_file": per_file, "elapsed": time.monotonic() - started,
               "equivalence_dispositions": reviews,
               "gate_satisfied": len(results) == len(rows) and all(
                   item["adjusted_percent"] is not None and item["adjusted_percent"] >= minimum
                   for item in per_file.values()) and not any(
                       (item.get("reviewed_outcome", item["outcome"]).startswith(("timed-out", "incomplete-", "unviable-driver"))
                        or item.get("reviewed_outcome", item["outcome"]).endswith("review-required"))
                       for item in results)}
    write_json(args.output / "summary.json", summary)
    print(json.dumps(summary, sort_keys=True))
    return 0 if summary["gate_satisfied"] else 1


if __name__ == "__main__":
    raise SystemExit(main())
