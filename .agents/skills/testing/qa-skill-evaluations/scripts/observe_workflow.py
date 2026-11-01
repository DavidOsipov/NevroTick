#!/usr/bin/env python3
"""Observe an explicit workflow ledger offline; exit zero means analysis only.

GPL-2.0-only. No hooks, interventions, model calls or implicit session access.
Receipt pins identify reported bytes; this observer does not qualify a candidate.
"""
from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
from pathlib import Path
import re
import stat
import sys

MAX_LEDGER_BYTES = 4 * 1024 * 1024
MAX_RECEIPT_BYTES = 8 * 1024 * 1024
MAX_TOTAL_RECEIPT_BYTES = 64 * 1024 * 1024
MAX_EVENTS = 4096
ROLES = {"implementation", "review", "repair", "observer", "verification"}
OUTCOMES = {"passed", "expected-failure", "unexpected-failure", "incomplete", "unknown"}
PRODUCT_SCOPES = {
    "UI-M0, wire, domain, exchange, synthetic regression and controlled-loopback engine pilot "
    + lane + " verification" for lane in ("portable", "windows", "dependencies")
}


def require(condition, message):
    """Reject malformed observations without converting them into evidence."""
    if not condition:
        raise ValueError(message)


def fields(value, required, optional=()):
    """Require one closed object shape, including exact JSON boolean callers."""
    require(type(value) is dict and set(required) <= value.keys()
            and value.keys() <= set(required) | set(optional), "Invalid object fields.")


def text_id(value):
    """Keep identifiers finite and safe to emit as JSON, without interpreting them."""
    require(type(value) is str and 0 < len(value) <= 256
            and all(ord(char) >= 32 for char in value), "Invalid identifier.")
    return value


def number(value, *, integer=False):
    """Reject booleans, nonfinite, negative and excessive accounting values."""
    require(type(value) is int if integer else type(value) in (int, float), "Invalid number type.")
    require(math.isfinite(value) and 0 <= value <= 10**15, "Invalid accounting value.")
    return value


def unique_object(pairs):
    """A repeated JSON key must not silently replace a result or identity."""
    result = {}
    for key, value in pairs:
        require(key not in result, "Duplicate JSON field.")
        result[key] = value
    return result


def read_json(path, limit, budget=None):
    """Read a finite regular file; nonblocking open rejects FIFO/device inputs."""
    fd = os.open(path, os.O_RDONLY | os.O_NONBLOCK)
    with os.fdopen(fd, "rb") as stream:
        require(stat.S_ISREG(os.fstat(stream.fileno()).st_mode), "Input must be a regular file.")
        raw = stream.read(limit + 1)
    if budget is not None:
        budget[0] += len(raw)
    require(len(raw) <= limit, "Input exceeds byte allowance.")
    value = json.loads(raw, object_pairs_hook=unique_object,
                       parse_constant=lambda _: require(False, "Nonfinite JSON number."))
    return value, raw


def validate(ledger):
    """Validate finite task/worker/event identities before any receipt is opened."""
    fields(ledger, ("schema_version", "workers", "tasks", "events"))
    require(type(ledger["schema_version"]) is int and ledger["schema_version"] == 1, "Unsupported ledger version.")
    workers, tasks, events = ledger["workers"], ledger["tasks"], ledger["events"]
    require(type(workers) is list and 0 < len(workers) <= 256, "Invalid worker list.")
    require(len({text_id(worker) for worker in workers}) == len(workers), "Duplicate worker.")
    require(type(tasks) is list and 0 < len(tasks) <= 1024, "Invalid task list.")
    task_ids = set()
    for task in tasks:
        fields(task, ("id", "outcome", "accounting_complete"), ("wall_seconds",))
        require(text_id(task["id"]) not in task_ids, "Duplicate task.")
        task_ids.add(task["id"])
        require(task["outcome"] in {"accepted", "failed", "incomplete"}, "Invalid reviewed task outcome.")
        require(type(task["accounting_complete"]) is bool, "accounting_complete must be a JSON boolean.")
        if "wall_seconds" in task:
            number(task["wall_seconds"])
    require(type(events) is list and len(events) <= MAX_EVENTS, "Invalid event count.")
    event_ids = set()
    for event in events:
        fields(event, ("id", "task", "worker", "role", "context", "outcome"),
               ("receipt", "failure", "remedy", "observation", "expected_cause", "effort_seconds", "wait_seconds", "usage"))
        require(text_id(event["id"]) not in event_ids, "Duplicate event.")
        event_ids.add(event["id"])
        require(event["task"] in task_ids and event["worker"] in workers, "Undeclared task or worker.")
        require(event["role"] in ROLES and event["outcome"] in OUTCOMES, "Invalid event role or outcome.")
        text_id(event["context"])
        for key in ("failure", "remedy", "observation"):
            if key in event:
                text_id(event[key])
        for key in ("effort_seconds", "wait_seconds"):
            if key in event:
                number(event[key])
        if "expected_cause" in event:
            cause = event["expected_cause"]
            fields(cause, ("reviewed", "intended", "observed"))
            require(type(cause["reviewed"]) is bool, "reviewed must be a JSON boolean.")
            text_id(cause["intended"]); text_id(cause["observed"])
        if "receipt" in event:
            receipt = event["receipt"]
            fields(receipt, ("kind", "path", "sha256"))
            require(receipt["kind"] in {"product", "engine-analysis"}, "Unsupported receipt adapter.")
            require(type(receipt["path"]) is str and len(receipt["path"]) <= 4096
                    and Path(receipt["path"]).is_absolute(), "Receipt path must be absolute.")
            require(type(receipt["sha256"]) is str and re.fullmatch(r"[0-9a-f]{64}", receipt["sha256"]), "Invalid receipt pin.")
        if "usage" in event:
            usage = event["usage"]
            fields(usage, (), ("input_tokens", "cached_input_tokens", "output_tokens", "cost_usd", "pricing_basis"))
            for key in ("input_tokens", "cached_input_tokens", "output_tokens", "cost_usd"):
                if key in usage:
                    number(usage[key], integer=key != "cost_usd")
            if {"input_tokens", "cached_input_tokens"} <= usage.keys():
                require(usage["cached_input_tokens"] <= usage["input_tokens"], "Cache exceeds inclusive input tokens.")
            require(("cost_usd" in usage) == ("pricing_basis" in usage), "Cost requires its pricing basis.")
            if "pricing_basis" in usage:
                require(usage["pricing_basis"] in {"recorded", "estimate"}, "Invalid pricing basis.")


def product_report_state(report):
    """Recognize verify.py's outer shape and reject contradictory pass records."""
    if report.get("scope") not in PRODUCT_SCOPES:
        return "unknown"
    if report.get("status") == "incomplete":
        return "incomplete"
    sources, lanes = report.get("source_sha256"), report.get("lanes")
    valid_sources = type(sources) is dict and bool(sources) and all(
        type(name) is str and bool(name) and type(digest) is str
        and re.fullmatch(r"[0-9a-f]{64}", digest) for name, digest in sources.items())
    # verify.py requires every outer lane to succeed, including its boundary
    # probe. This differs from engine analysis' intentional negative children.
    valid_lanes = type(lanes) is list and bool(lanes) and all(
        type(lane) is dict and {"lane", "command", "exit_code", "failure", "output"} <= lane.keys()
        and type(lane["lane"]) is str and bool(lane["lane"])
        and type(lane["command"]) is list and bool(lane["command"])
        and all(type(argument) is str for argument in lane["command"])
        and type(lane["exit_code"]) is int and lane["exit_code"] == 0
        and lane["failure"] is None and type(lane["output"]) is str for lane in lanes)
    return ("reported-passed" if report.get("status") == "passed" and "error" not in report
            and valid_sources and valid_lanes else "unknown")


def receipt_state(spec, cache, budget):
    """Read pinned known outer reports; never classify child return codes."""
    if spec is None:
        return "unknown"
    key = (spec["path"], spec["sha256"], spec["kind"])
    if key in cache:
        return cache[key]
    state = "unknown"
    try:
        remaining = MAX_TOTAL_RECEIPT_BYTES - budget[0]
        require(remaining > 1, "Aggregate receipt byte allowance exhausted.")
        report, raw = read_json(spec["path"], min(MAX_RECEIPT_BYTES, remaining - 1), budget)
        if hashlib.sha256(raw).hexdigest() != spec["sha256"]:
            state = "drifted"
        elif type(report) is dict and type(report.get("scope")) is str:
            status = report.get("status")
            if spec["kind"] == "engine-analysis":
                lane, qualified = report.get("selected_lane"), report.get("qualification_passed")
                if (lane in {"enumerate", "coverage", "mutation"} and type(qualified) is bool
                        and report["scope"] == "Bounded host engine " + lane):
                    if status == "enumerated" and lane == "enumerate" and not qualified and "error" not in report:
                        state = "enumerated"
                    elif status == "passed" and lane != "enumerate" and qualified and "error" not in report:
                        state = "reported-passed"
                    elif status == "incomplete" and not qualified:
                        state = "incomplete"
            else:
                state = product_report_state(report)
    except (OSError, ValueError, TypeError, RecursionError):
        state = "unavailable"
    cache[key] = state
    return state


def observed_outcome(event, state):
    """Preserve uncertainty and require reviewed intended-cause controls."""
    declared = event["outcome"]
    if state == "enumerated":
        return "enumerated"
    if state == "reported-passed":
        if declared == "passed":
            return "passed"
        cause = event.get("expected_cause", {})
        if (declared == "expected-failure" and cause.get("reviewed") is True
                and cause.get("intended") == cause.get("observed") and cause.get("intended")):
            return "expected-failure"
    if state == "incomplete":
        return "unexpected-failure" if declared == "unexpected-failure" else "incomplete"
    return "unknown"


def metric(events, name):
    """A known subtotal is not a complete total; omissions are never zero work."""
    values = [event[name] if name in event else event.get("usage", {}).get(name) for event in events]
    known = [value for value in values if value is not None]
    return {"known_subtotal": sum(known) if known else None, "known_events": len(known),
            "total_events": len(events), "complete": bool(events) and len(known) == len(events)}


def analyze(ledger):
    """Return shadow advisories and attributed partial accounting, no effects."""
    validate(ledger)
    events, tasks = ledger["events"], ledger["tasks"]
    observed = {event["worker"] for event in events}
    unobserved = sorted(set(ledger["workers"]) - observed)
    cache, budget, chains, advisories, observations = {}, [0], {}, [], []
    for event in events:
        state = receipt_state(event.get("receipt"), cache, budget)
        outcome = observed_outcome(event, state)
        observations.append({"id": event["id"], "worker": event["worker"], "task": event["task"],
                             "receipt_state": state, "outcome": outcome})
        owner = (event["worker"], event["task"])
        identity = tuple(event.get(key) for key in ("context", "failure", "remedy", "observation"))
        if outcome == "unexpected-failure" and all(identity):
            previous, attempts = chains.get(owner, (None, []))
            attempts = [*attempts, event["id"]] if previous == identity else [event["id"]]
            chains[owner] = (identity, attempts)
            if len(attempts) == 2:
                advisories.append({"task": owner[1], "worker": owner[0], "events": attempts,
                                   "kind": "repeated-unexpected-failure", "failure_cause": "unclassified",
                                   "action": "Review retained evidence and choose a discriminating next observation."})
        else:
            chains.pop(owner, None)
    accounting, cost_tasks = {}, []
    for task in tasks:
        selected = [event for event in events if event["task"] == task["id"]]
        complete = bool(selected) and task["accounting_complete"] and not unobserved
        costs_complete = complete and all("cost_usd" in event.get("usage", {}) for event in selected)
        if costs_complete:
            cost_tasks.append(task["id"])
        accounting[task["id"]] = {"declared_complete": task["accounting_complete"],
                                  "cost_complete": costs_complete, "events": len(selected),
                                  "wall_seconds": task.get("wall_seconds")}
    metrics = ("effort_seconds", "wait_seconds", "input_tokens", "cached_input_tokens", "output_tokens")
    costs = {basis: metric([event for event in events if event.get("usage", {}).get("pricing_basis") == basis], "cost_usd")
             for basis in ("recorded", "estimate")}
    complete_costs = {basis: metric([event for event in events if event["task"] in cost_tasks
                                    and event.get("usage", {}).get("pricing_basis") == basis], "cost_usd")
                      for basis in ("recorded", "estimate")}
    return {"schema_version": 1, "mode": "shadow", "qualification": False,
            "quality": {"assigned": len(tasks), **{label: sum(task["outcome"] == label for task in tasks)
                                                   for label in ("accepted", "failed", "incomplete")}},
            "visibility": {"declared_workers": ledger["workers"], "observed_workers": sorted(observed),
                           "unobserved_workers": unobserved,
                           "unobserved_tasks": [task["id"] for task in tasks if not accounting[task["id"]]["events"]]},
            "accounting": {"tasks": accounting, "complete_cost_task_ids": cost_tasks,
                           "cost_task_denominator": len(cost_tasks), "quality_task_denominator": len(tasks),
                           "metrics": {name: metric(events, name) for name in metrics},
                           "cost_coverage": metric(events, "cost_usd"),
                           "costs_by_basis": costs,
                           "complete_cost_subset_by_basis": complete_costs,
                           "complete": bool(events) and not unobserved and all(task["accounting_complete"] for task in tasks)
                           and all(accounting[task["id"]]["events"] for task in tasks)
                           and all(metric(events, name)["complete"] for name in metrics)
                           and all("cost_usd" in event.get("usage", {}) for event in events)},
            "observations": observations, "advisories": advisories,
            "limits": {"receipt_bytes_read": budget[0]},
            "scope": "Explicit ledger only; reviewed task outcomes supplied by caller, receipts not requalified."}


def main(argv=None):
    """Print JSON only; malformed input exits two, successful observation exits zero."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ledger", required=True, type=Path)
    args = parser.parse_args(argv)
    try:
        ledger, raw = read_json(args.ledger, MAX_LEDGER_BYTES)
        report = analyze(ledger)
        report["ledger_sha256"] = hashlib.sha256(raw).hexdigest()
    except (OSError, ValueError, TypeError, RecursionError, OverflowError) as error:
        print(json.dumps({"mode": "shadow", "qualification": False, "error": str(error)}))
        return 2
    print(json.dumps(report, allow_nan=False, sort_keys=True))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
