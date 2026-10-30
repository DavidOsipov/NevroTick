#!/usr/bin/env python3
"""Qualify the admitted S1.1d portable association analysis scope.

GPL-2.0-only, David Osipov with AI assistance, 29 October 2026. Orchestration
adapts verify-time-domain-analysis.py; shared custody and accounting remain in
portable-analysis.py. No acquisition, extra dependency graph or host fallback.
"""
from __future__ import annotations

import argparse
import importlib.util
import json
from pathlib import Path
import shutil
import sys
import time
import tomllib

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("exchange_analysis_shared", ROOT / "scripts/portable-analysis.py")
COMMON = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COMMON)
RUNTIME = COMMON.RUNTIME
PRODUCT = COMMON.PRODUCT
Incomplete = COMMON.Incomplete
SUBJECTS = {name: f"crates/ntp-exchange/src/{name}.rs" for name in ("context", "association")}
POLICY = "policy/ntp-exchange-analysis.json"
SCOPE = COMMON.Scope("ntp-exchange", tuple(SUBJECTS.items()), POLICY)
OBSERVER = "policy/ntp-exchange-resource-observer.rs"
BOUND_INPUTS = (
    "Cargo.toml", "Cargo.lock", "crates/ntp-exchange/Cargo.toml",
    "crates/ntp-exchange/src/lib.rs", *SUBJECTS.values(),
    "policy/ui-m0-dependencies.json", "policy/verification-manifest.toml",
    "tasks/s1.1-exchange-vectors.json", OBSERVER,
)
LIBRARIES = ("ntp-exchange", "time-domain", "ntp-wire")
SNAPSHOT_COUNTERS = (
    "alloc_requests", "zeroed_requests", "realloc_requests", "requested_bytes",
    "successful_requests", "successful_requested_bytes", "failed_requests", "deallocations",
)
SNAPSHOT_FIELDS = (*SNAPSHOT_COUNTERS, "live_payload_bytes", "peak_live_payload_bytes", "counter_error")
RESOURCE_COUNTS = {
    "hostile_receive": (4096, 4096, 4096, 4096, 0, 0, 4096, 0),
    "complete": (64, 64, 0, 32, 32, 0, 64, 0),
    "cancel": (64, 128, 64, 64, 0, 64, 64, 64),
    "expire": (64, 128, 64, 64, 0, 64, 64, 64),
    "context": (64, 128, 128, 64, 0, 64, 64, 64),
}
RESOURCE_COUNT_FIELDS = (
    "iterations", "arms", "rejected_receives", "time_events", "control_events",
    "retirements", "duplicates", "rejected_lifecycle",
)


def read_policy(root):
    """Admit exact reviewed source/configuration scope and unchanged gate budgets."""
    policy = json.loads((root / POLICY).read_text())
    required = {"schema_version": 1, "package": "ntp-exchange", "critical_files": list(SUBJECTS.values()),
                "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
                "maximum_mutants": 512, "coverage_wall_seconds": 300, "mutation_wall_seconds": 1200,
                "mutation_build_seconds": 90, "mutation_test_seconds": 60, "contracts_wall_seconds": 300,
                "jobs": RUNTIME.MAX_JOBS, "test_threads": RUNTIME.MAX_JOBS, "resource_wall_seconds": 300,
                "resource_profile": "single-thread-fixed-fixture-v1"}
    if (set(policy) != {*required, "mutation_equivalences", "inputs_sha256", "scope"}
            or any(type(policy.get(key)) is not type(value) or policy.get(key) != value
                   for key, value in required.items())):
        raise Incomplete("Unreviewed exchange analysis scope, threshold or budget.")
    COMMON.admit_mutation_equivalences(root, SCOPE, policy)
    if not isinstance(policy["inputs_sha256"], dict) or set(policy["inputs_sha256"]) != set(BOUND_INPUTS):
        raise Incomplete("Exchange policy omits or extends its exact source/configuration bindings.")
    for name, digest in policy["inputs_sha256"].items():
        if PRODUCT.sha256(root / name) != digest:
            raise Incomplete("Exchange source, graph, fixture or critical manifest changed.")
    inventory = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    for filename in SUBJECTS.values():
        if (inventory.get("critical_files", []).count(filename) != 1
                or sum(module.get("package") == "ntp-exchange" and filename in module.get("files", [])
                       for module in inventory.get("security_modules", [])) != 1):
            raise Incomplete("An exchange owner is absent/duplicated in the security/critical inventory.")
    return policy


def contract_examples():
    """External legal use and exact private, default, clone and unit barriers."""
    setup = (
        'let one = std::num::NonZeroU64::new(1).ok_or("nonzero fixture")?; '
        'let source = SourceId::new(one); let socket = SocketId::new(one); '
        'let request = RequestId::new(one); '
        'let configuration = ConfigurationGeneration::new(0); '
        'let association = AssociationGeneration::new(0); let clock = ClockGeneration::new(0); '
        'let generations = Generations::new(configuration, association, clock); '
        'let endpoint = std::net::SocketAddr::from(([127,0,0,1],123)); '
        'let binding = Binding::new(source, socket, endpoint); '
        'let context = AssociationContext::new(binding, generations); '
        'let send = MonotonicNs::new(1); let deadline = MonotonicNs::new(10); '
        'let timing = RequestTiming::new(send, deadline)?; '
        'let utc = UtcCoordinateQ32::new(1)?; let window = EraWindow::new(1, 1)?; '
        'let eras = PeerEraContext::new(window, window); let token = WireToken::new(1)?; '
        'let receipt = ReceiptFacts::new(binding, clock, send, send, utc); '
        'let mut owner = Association::new_plain_header_fixture(context, request); '
        'assert_eq!(owner.arm(utc, token, timing, eras)?, request); '
        'let mut packet = [0_u8; 48]; packet[0] = 0x24; packet[1] = 1; '
        'packet[31] = 1; packet[39] = 1; packet[47] = 1; '
        'let mut observation = match owner.receive(&packet, receipt)? { '
        'FixtureEvent::Time(value) => value, FixtureEvent::Control(_) => return Err("expected time".into()) }; '
        'let mut control_owner = Association::new_plain_header_fixture(context, request); '
        'control_owner.arm(utc, WireToken::new(1)?, timing, eras)?; packet[1] = 0; '
        'let mut control = match control_owner.receive(&packet, receipt)? { '
        'FixtureEvent::Control(value) => value, FixtureEvent::Time(_) => return Err("expected control".into()) };'
    )
    facts = ('CompletionFacts { binding, request_id: request, generations, mapping_generation: clock, '
             'send, arrival: send, processing_now: send, deadline }')
    observation_literal = (
        'UnauthenticatedFixtureObservation { facts: ' + facts + ', saved_real_t1: utc, peer_t2: utc, '
        'peer_t3: utc, local_t4: utc, offset: observation.offset(), '
        'signed_round_trip: observation.signed_round_trip(), leap_indicator: 0, stratum: 1, '
        'poll: 0, precision: 0, root_delay_bits: 0, root_dispersion_bits: 0, '
        'reference_id: 0, reference_timestamp_bits: 0 }'
    )
    legal = (
        'assert_eq!(source.value(), one); assert_eq!(socket.value(), one); assert_eq!(request.value(), one); '
        'assert_eq!(generations.configuration().value(), 0); assert_eq!(generations.association().value(), 0); '
        'assert_eq!(generations.clock().value(), 0); assert_eq!(context.binding(), binding); '
        'assert_eq!(binding.source_id(), source); assert_eq!(binding.socket_id(), socket); '
        'assert_eq!(binding.endpoint(), endpoint); assert_eq!(timing.send(), send); '
        'assert_eq!(timing.deadline(), deadline); assert_eq!(eras.receive_window(), window); '
        'assert_eq!(eras.transmit_window(), window); assert_eq!(receipt.local_t4(), utc); '
        'assert_eq!(observation.facts().request_id(), request); '
        'assert_eq!(observation.facts().binding(), binding); '
        'assert_eq!(observation.saved_real_t1(), utc); assert_eq!(observation.peer_t2(), utc); '
        'assert_eq!(observation.peer_t3(), utc); assert_eq!(observation.local_t4(), utc); '
        'assert_eq!(observation.offset().numerator(), 0); '
        'assert_eq!(observation.signed_round_trip().numerator(), 0); '
        'assert_eq!(observation.leap_indicator(), 0); assert_eq!(observation.stratum(), 1); '
        'assert_eq!(control.code_bits(), 0); assert_eq!(control.facts().request_id(), request); '
        'assert!(owner.pending_request_id().is_none()); assert!(control_owner.pending_request_id().is_none()); '
        'assert_eq!(owner.receive(&packet, receipt), Err(ReceiveError::NoPending)); '
        'println!("legal-exchange-contracts:time-control-once");'
    )
    examples = {"legal": {"statement": legal, "code": None, "tokens": []}}
    representations = [
        ("source", "SourceId", "value", 'SourceId { value: one }', 'source', 'one'),
        ("socket", "SocketId", "value", 'SocketId { value: one }', 'socket', 'one'),
        ("request", "RequestId", "value", 'RequestId { value: one }', 'request', 'one'),
        ("configuration", "ConfigurationGeneration", "value", 'ConfigurationGeneration { value: 0 }', 'configuration', '0'),
        ("association_generation", "AssociationGeneration", "value", 'AssociationGeneration { value: 0 }', 'association', '0'),
        ("clock", "ClockGeneration", "value", 'ClockGeneration { value: 0 }', 'clock', '0'),
        ("generations", "Generations", "configuration", 'Generations { configuration, association, clock }', 'generations', 'configuration'),
        ("binding", "Binding", "source_id", 'Binding { source_id: source, socket_id: socket, endpoint }', 'binding', 'source'),
        ("context", "AssociationContext", "binding", 'AssociationContext { binding, generations }', 'context', 'binding'),
        ("token", "WireToken", "value", 'WireToken { value: one }', 'WireToken::new(1)?', 'one'),
        ("timing", "RequestTiming", "send", 'RequestTiming { send, deadline }', 'timing', 'send'),
        ("eras", "PeerEraContext", "receive", 'PeerEraContext { receive: window, transmit: window }', 'eras', 'window'),
        ("receipt", "ReceiptFacts", "binding", 'ReceiptFacts { binding, mapping_generation: clock, arrival: send, processing_now: send, local_t4: utc }', 'receipt', 'binding'),
        ("completion", "CompletionFacts", "binding", facts, 'observation.facts()', 'binding'),
        ("observation", "UnauthenticatedFixtureObservation", "saved_real_t1", observation_literal, 'observation', 'utc'),
        ("control", "OpaqueControl", "code_bits", 'OpaqueControl { facts: ' + facts + ', code_bits: 0 }', 'control', '0'),
    ]
    for name, typename, field, constructor, value, replacement in representations:
        examples[name + "_construction"] = {"statement": 'let _ = ' + constructor + ';', "code": "E0451", "tokens": [typename, field]}
        examples[name + "_mutation"] = {"statement": f'let mut value = {value}; value.{field} = {replacement};', "code": "E0616", "tokens": [typename, field]}
        examples[name + "_default"] = {"statement": f'let _: {typename} = Default::default();', "code": "E0277", "tokens": [typename, "Default"]}
    forbidden = [
        ("owner_mutation", 'owner.context = context;', "E0616", ["Association", "context"]),
        ("owner_default", 'let _: Association = Default::default();', "E0277", ["Association", "Default"]),
        ("owner_clone", 'let _ = owner.clone();', "E0599", ["Association", "clone"]),
        ("owner_copy", 'require_copy::<Association>();', "E0277", ["Association", "Copy"]),
        ("token_raw_getter", 'let _ = token.value();', "E0599", ["WireToken", "value"]),
        ("raw_token_arm", 'let _ = owner.arm(utc, 1_u64, timing, eras);', "E0308", ["WireToken", "u64"]),
        ("raw_token_from", 'let _: WireToken = 1_u64.into();', "E0277", ["WireToken", "u64"]),
        ("raw_request", 'let _ = Association::new_plain_header_fixture(context, one);', "E0308", ["RequestId", "NonZero"]),
        ("source_socket_mixed", 'let _ = Binding::new(socket, source, endpoint);', "E0308", ["SourceId", "SocketId"]),
        ("generation_roles_mixed", 'let _ = Generations::new(clock, association, configuration);', "E0308", ["ConfigurationGeneration", "ClockGeneration"]),
        ("active_awake_timing", 'let _ = RequestTiming::new(ActiveAwakeNs::new(1), deadline);', "E0308", ["MonotonicNs", "ActiveAwakeNs"]),
        ("utc_timing", 'let _ = RequestTiming::new(utc, deadline);', "E0308", ["MonotonicNs", "UtcCoordinateQ32"]),
        ("mapping_generation_mixed", 'let _ = ReceiptFacts::new(binding, configuration, send, send, utc);', "E0308", ["ClockGeneration", "ConfigurationGeneration"]),
        ("raw_saved_t1", 'let _ = owner.arm(1_i128, WireToken::new(1)?, timing, eras);', "E0308", ["UtcCoordinateQ32", "i128"]),
        ("missing_era_context", 'let _ = owner.arm(utc, WireToken::new(1)?, timing, None);', "E0308", ["PeerEraContext", "Option"]),
        ("omitted_era_context", 'let _ = owner.arm(utc, WireToken::new(1)?, timing);', "E0061", ["PeerEraContext"]),
        ("header_receive_bypass", 'let raw = ntp_wire::decode_fixed_header(&packet).map_err(|_| "fixture header failure")?; let _ = owner.receive(raw, receipt);', "E0308", ["RawHeader", "[u8]"]),
    ]
    for name, statement, code, tokens in forbidden:
        examples[name] = {"statement": statement, "code": code, "tokens": tokens}
    for item in examples.values():
        item["source"] = '\n'.join([
            'use ntp_exchange::*; use time_domain::*;',
            'fn require_copy<T: Copy>() {}',
            'fn main() -> Result<(), Box<dyn std::error::Error>> {', setup,
            item.pop("statement"), 'Ok(())', '}',
        ]) + '\n'
        item["line"] = 5
    return examples


def copy_contract_dependencies(session, output, directory="contract-deps"):
    """Seal actual emitted local libraries and their attributed dependencies."""
    if directory not in {"contract-deps", "resource-deps"}:
        raise Incomplete("Unreviewed external dependency island.")
    selected = {package: COMMON.library_artifact(output, package,
                f"/work/source/crates/{package}/Cargo.toml") for package in LIBRARIES}
    attributed = set()
    for line in output.splitlines():
        try:
            item = json.loads(line)
        except json.JSONDecodeError:
            continue
        if item.get("reason") != "compiler-artifact":
            continue
        for name in item.get("filenames", []):
            path = Path(name)
            if path.suffix not in {".rlib", ".rmeta", ".so"}:
                continue
            if not path.is_absolute() or ".." in path.parts or not path.is_relative_to("/work/target"):
                raise Incomplete("Emitted dependency escapes the owned target directory.")
            attributed.add(name)
    if not attributed or not set(selected.values()).issubset(attributed):
        raise Incomplete("Emitted contract dependencies omit a required local library.")
    destination = session.work / directory
    destination.mkdir(mode=0o700)
    copied = {}
    for name in sorted(attributed):
        source = session.work / name.removeprefix("/work/")
        if source.resolve(strict=True) != source or not source.is_file():
            raise Incomplete("Emitted dependency was redirected or is not a regular file.")
        digest = PRODUCT.sha256(source)
        target = destination / source.name
        if target.name in copied:
            raise Incomplete("Emitted dependency basenames collide.")
        shutil.copyfile(source, target); target.chmod(0o400)
        if PRODUCT.sha256(target) != digest:
            raise Incomplete("Emitted dependency changed during immutable copying.")
        copied[target.name] = digest
    externs = {package.replace("-", "_"): "/work/" + directory + "/" + Path(path).name
               for package, path in selected.items()}
    session.protect(destination)
    return destination, externs, selected


def contracts(session, policy, report):
    """Execute a legal external consumer and require each exact negative cause."""
    deadline = time.monotonic() + policy["contracts_wall_seconds"]
    output = session.run("contract-library", [str(session.stable / "bin/cargo"), "build", "--package", "ntp-exchange",
                         "--frozen", "--jobs", str(RUNTIME.MAX_JOBS), "--lib", "--message-format=json"],
                         COMMON.remaining_timeout(deadline, 120))
    dependencies, externs, artifacts = copy_contract_dependencies(session, output)
    consumers = session.work / "contract-inputs"; consumers.mkdir(mode=0o700)
    (session.work / "contracts").mkdir(mode=0o700)
    examples = contract_examples()
    for name, specimen in examples.items():
        (consumers / (name + ".rs")).write_text(specimen["source"])
    session.protect(consumers)
    for name, specimen in examples.items():
        command = [str(session.stable / "bin/rustc"), "--edition", "2024", "--crate-name", name,
                   "--error-format=json", "/work/contract-inputs/" + name + ".rs"]
        for package, artifact in externs.items():
            command.extend(["--extern", package + "=" + artifact])
        command.extend(["-L", "dependency=/work/contract-deps", "--out-dir", "/work/contracts"])
        session.run("external-consumer-" + name, command, COMMON.remaining_timeout(deadline, 60),
                    sealed=[consumers, dependencies], specimen=specimen if specimen["code"] else None)
        if specimen["code"] is None:
            text = session.run("external-legal-execution", ["/work/contracts/" + name],
                               COMMON.remaining_timeout(deadline, 30), sealed=[consumers, dependencies])
            if "legal-exchange-contracts:time-control-once" not in text:
                raise Incomplete("Legal external consumer did not complete its required observations.")
    report["contracts"] = {
        "legal_consumers_compiled_and_executed": 1, "library_artifacts": artifacts,
        "immutable_dependencies_sha256": PRODUCT.inventory(dependencies),
        "negative_consumers": len(examples) - 1,
        "required_diagnostics": {name: {key: item[key] for key in ("code", "tokens", "line")}
                                 for name, item in examples.items() if item["code"]},
    }


def decode_resource_output(text):
    """Require one finite bounded JSON object without duplicate evidence keys."""
    if not isinstance(text, str) or len(text.encode()) > 131072:
        raise Incomplete("Resource output exceeds the fixed JSON budget.")

    def pairs(values):
        result = {}
        for key, value in values:
            if key in result:
                raise Incomplete("Duplicate resource evidence key.")
            result[key] = value
        return result

    return json.loads(text, object_pairs_hook=pairs,
                      parse_constant=lambda _: (_ for _ in ()).throw(Incomplete("Nonfinite resource evidence.")))


def resource_snapshot(value):
    """Validate actual finite counters, retaining exact bytes rather than summaries."""
    if (not isinstance(value, dict) or set(value) != set(SNAPSHOT_FIELDS)
            or value["counter_error"] is not False
            or any(type(value[key]) is not int or not 0 <= value[key] <= 2**64 - 1
                   for key in SNAPSHOT_FIELDS if key != "counter_error")
            or value["peak_live_payload_bytes"] < value["live_payload_bytes"]):
        raise Incomplete("Invalid or inconsistent resource snapshot counters.")
    return value


def resource_continuity(before, after):
    """Cumulative request/accounting counters cannot regress across observations."""
    if any(after[key] < before[key] for key in SNAPSHOT_COUNTERS):
        raise Incomplete("Resource accounting counters regressed.")


def resource_quiescence(before, after):
    """No hidden allocation or owner-destructor release may occur between intervals."""
    if any(before[key] != after[key] for key in (*SNAPSHOT_COUNTERS, "live_payload_bytes")):
        raise Incomplete("Resource activity occurred outside a declared measured interval.")


def validate_resource_observation(value):
    """Discriminate real allocation controls and every declared product interval."""
    required = {"schema_version", "scope", "profile", "debug_assertions", "samples", "box_control",
                "vec_control", "observation_initial", "observation_final"}
    if (not isinstance(value, dict) or set(value) != required
            or type(value["schema_version"]) is not int or value["schema_version"] != 1
            or value["scope"] != "s1.1d-host-requested-payload"
            or value["profile"] != "single-thread-fixed-fixture-v1"
            or value["debug_assertions"] is not True):
        raise Incomplete("Unreviewed host resource observation schema or build profile.")
    initial = resource_snapshot(value["observation_initial"])
    final = resource_snapshot(value["observation_final"])
    box = value["box_control"]
    if (not isinstance(box, dict) or set(box) != {"before", "held", "dropped", "contents_match"}
            or box["contents_match"] is not True):
        raise Incomplete("Missing actual held/released Box control.")
    for name in ("before", "held", "dropped"):
        resource_snapshot(box[name])
    resource_quiescence(initial, box["before"])
    resource_continuity(box["before"], box["held"])
    resource_continuity(box["held"], box["dropped"])
    if (box["before"]["live_payload_bytes"] != initial["live_payload_bytes"]
            or box["held"]["live_payload_bytes"] - box["before"]["live_payload_bytes"] < 4096
            or box["held"]["successful_requests"] <= box["before"]["successful_requests"]
            or box["dropped"]["live_payload_bytes"] != box["before"]["live_payload_bytes"]
            or box["dropped"]["deallocations"] <= box["held"]["deallocations"]):
        raise Incomplete("Box control did not discriminate real retention and release.")
    vec = value["vec_control"]
    if (not isinstance(vec, dict) or set(vec) != {"before", "zeroed", "grown", "shrunk", "dropped", "contents_match"}
            or vec["contents_match"] is not True):
        raise Incomplete("Missing actual Vec allocation/reallocation control.")
    previous = box["dropped"]
    for name in ("before", "zeroed", "grown", "shrunk", "dropped"):
        observed = resource_snapshot(vec[name])
        if name == "before":
            resource_quiescence(previous, observed)
        resource_continuity(previous, observed); previous = observed
    if (vec["before"]["live_payload_bytes"] != initial["live_payload_bytes"]
            or vec["zeroed"]["zeroed_requests"] <= vec["before"]["zeroed_requests"]
            or vec["zeroed"]["live_payload_bytes"] <= vec["before"]["live_payload_bytes"]
            or vec["grown"]["realloc_requests"] <= vec["zeroed"]["realloc_requests"]
            or vec["grown"]["live_payload_bytes"] <= vec["zeroed"]["live_payload_bytes"]
            or vec["shrunk"]["realloc_requests"] <= vec["grown"]["realloc_requests"]
            or not vec["before"]["live_payload_bytes"] < vec["shrunk"]["live_payload_bytes"] < vec["grown"]["live_payload_bytes"]
            or vec["dropped"]["live_payload_bytes"] != vec["before"]["live_payload_bytes"]
            or vec["dropped"]["deallocations"] <= vec["shrunk"]["deallocations"]):
        raise Incomplete("Vec control did not discriminate allocation, growth, shrink and release.")
    samples = value["samples"]
    if not isinstance(samples, list) or len(samples) != len(RESOURCE_COUNTS):
        raise Incomplete("Missing or extra resource workload samples.")
    for sample, (name, counts) in zip(samples, RESOURCE_COUNTS.items(), strict=True):
        if (not isinstance(sample, dict)
                or set(sample) != {"name", *RESOURCE_COUNT_FIELDS, "mismatches", "pending_empty", "before", "after"}
                or sample["name"] != name or type(sample["mismatches"]) is not int or sample["mismatches"] != 0
                or sample["pending_empty"] is not True
                or any(type(sample[key]) is not int or sample[key] != expected
                       for key, expected in zip(RESOURCE_COUNT_FIELDS, counts, strict=True))):
            raise Incomplete("Resource workload counts, state results or independent expectations differ.")
        before, after = resource_snapshot(sample["before"]), resource_snapshot(sample["after"])
        resource_quiescence(previous, before); resource_continuity(before, after)
        if (before["live_payload_bytes"] != initial["live_payload_bytes"] or before["peak_live_payload_bytes"] != before["live_payload_bytes"]
                or any(after[key] != before[key] for key in SNAPSHOT_COUNTERS)
                or after["live_payload_bytes"] != before["live_payload_bytes"] or after["peak_live_payload_bytes"] != before["live_payload_bytes"]):
            raise Incomplete("A product interval requested/released allocation or retained payload.")
        previous = after
    resource_quiescence(previous, final)
    if final["live_payload_bytes"] != initial["live_payload_bytes"]:
        raise Incomplete("Resource payload was not released before observer teardown.")
    return {"hostile_receive_attempts": 4096, "retire_rearm_cycles": 256,
            "sample_count": len(samples), "live_requested_payload_baseline": initial["live_payload_bytes"],
            "final_live_requested_payload": final["live_payload_bytes"],
            "zero_interval_requests_and_retained_delta": True,
            "scope": "Observed host allocator requests before output/teardown; no native Windows/RSS/heap-overhead claim"}


def observer_lint_flags(workspace):
    """Apply the actual workspace deny policy to this external Rust consumer."""
    flags = []
    for namespace in ("rust", "clippy"):
        values = workspace["workspace"]["lints"][namespace]
        for name, value in sorted(values.items(), key=lambda pair: (
                pair[1].get("priority", 0) if isinstance(pair[1], dict) else 0, pair[0])):
            if (value["level"] if isinstance(value, dict) else value) != "deny":
                raise Incomplete("Unexpected external observer lint-policy level.")
            flags.extend(["--deny", ("clippy::" if namespace == "clippy" else "") + name])
    return flags


def resources(session, policy, report):
    """Build and run a separate fixed host allocator observer under existing custody."""
    deadline = time.monotonic() + policy["resource_wall_seconds"]
    run = session.run
    output = run("resource-library", [str(session.stable / "bin/cargo"), "build", "--package", "ntp-exchange",
                 "--frozen", "--jobs", str(RUNTIME.MAX_JOBS), "--lib", "--message-format=json"],
                 COMMON.remaining_timeout(deadline, 120))
    dependencies, externs, artifacts = copy_contract_dependencies(session, output, "resource-deps")
    source = "/work/source/" + OBSERVER
    run("resource-format", [str(session.stable / "bin/rustfmt"), "--edition", "2024", "--check", source],
        COMMON.remaining_timeout(deadline, 30))
    flags = ["--edition=2024", "--crate-name", "ntp_exchange_resource_observer",
             "-C", "opt-level=0", "-C", "debug-assertions=yes", "-C", "overflow-checks=yes",
             "-C", "panic=unwind", "-C", "debuginfo=2", "-C", "strip=none"]
    for package, artifact in externs.items():
        flags.extend(["--extern", package + "=" + artifact])
    flags.extend(["-L", "dependency=/work/resource-deps",
                  *observer_lint_flags(tomllib.loads((session.source / "Cargo.toml").read_text()))])
    run("resource-clippy", [str(session.stable / "bin/clippy-driver"), *flags,
                           "--emit=metadata", "-o", "/work/resource.rmeta", source],
        COMMON.remaining_timeout(deadline, 60), sealed=[dependencies])
    run("resource-link", [str(session.stable / "bin/rustc"), *flags,
                         "-o", "/work/exchange-resource-observer", source],
        COMMON.remaining_timeout(deadline, 60), sealed=[dependencies])
    # Execute a new sealed copy; source/build output aliases cannot rewrite the observed image.
    image = session.work / "resource-image"; image.mkdir(mode=0o700)
    emitted = session.work / "exchange-resource-observer"
    artifact = image / "observer"
    digest = PRODUCT.sha256(emitted)
    shutil.copyfile(emitted, artifact); artifact.chmod(0o500)
    if PRODUCT.sha256(artifact) != digest:
        raise Incomplete("Resource executable changed while preparing its sealed copy.")
    session.protect(image)
    output = run("resource-observation", ["/work/resource-image/observer"],
                 COMMON.remaining_timeout(deadline, 60), sealed=[dependencies, image])
    observation = decode_resource_output(output)
    assessment = validate_resource_observation(observation)
    report["resources"] = {"assessment": assessment, "observation": observation,
                           "observer_sha256": PRODUCT.sha256(session.source / OBSERVER),
                           "executable_sha256": digest, "library_artifacts": artifacts,
                           "immutable_dependencies_sha256": PRODUCT.inventory(dependencies),
                           "profile": policy["resource_profile"], "direct_flags": flags}


def execute(args, report):
    """Run fixed admitted contracts/analysis; retain generated-model evidence elsewhere."""
    policy = read_policy(ROOT)
    extra = ("scripts/verify-exchange-analysis.py", *BOUND_INPUTS)
    session = COMMON.Session(ROOT, args, report, SCOPE, policy, extra)
    subjects = SCOPE.select(args.subject)
    report["subjects"] = subjects
    report["generated_model"] = "Separate admitted integration-test evidence; not discharged by this analysis receipt"
    report["resource_observation"] = "Host allocator observation only when the resources lane is selected"
    try:
        if args.lane in {"contracts", "all"}:
            contracts(session, policy, report)
        COMMON.run_analysis_lanes(SCOPE, subjects, policy, session.work, session.tools, session.run, report, args.lane)
        if args.lane in {"resources", "all"}:
            resources(session, policy, report)
    finally:
        session.finish()


def main(argv=None):
    """Keep finite explicit lanes, fresh private outputs and incomplete failures."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", choices=("contracts", "coverage", "mutation", "resources", "all"), default="all")
    parser.add_argument("--subject", choices=(*SUBJECTS, "all"), default="all")
    parser.add_argument("--vendor-dir", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    report = {"status": "incomplete", "selected_lane": args.lane,
              "scope": "S1.1d pure exchange " + args.lane,
              "native_qualification": "not performed"}
    try:
        if args.vendor_dir is None:
            args.vendor_dir = Path(json.loads(PRODUCT.POINTER.read_text())["vendor_dir"])
        execute(args, report)
        report["status"] = "passed"
    except (Incomplete, PRODUCT.ENV.Incomplete, OSError, ValueError, KeyError, TypeError, StopIteration) as error:
        report["error"] = str(error)
    if report.get("owned_output") == str(args.output_dir):
        (args.output_dir / "verification.json").write_text(json.dumps(report, indent=2) + "\n")
    print(report["status"].upper() + ": " + report["scope"], flush=True)
    if report.get("error"):
        print(report["error"], file=sys.stderr)
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
