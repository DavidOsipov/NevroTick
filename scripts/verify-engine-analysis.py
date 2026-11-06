#!/usr/bin/env python3
"""Bounded host engine coverage and mutation; enumeration grants no qualification.

GPL-2.0-only. Reuse the admitted shared analysis namespace, tools, source custody
and native outcome accounting. No acquisition, new graph or profile exception.
"""
from __future__ import annotations

import argparse
from collections import Counter
import importlib.util
import json
import math
from pathlib import Path
import re
import shutil
import sys
import time
import tomllib
from typing import NoReturn

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
SPEC = importlib.util.spec_from_file_location("engine_analysis_shared", ROOT / "scripts/portable-analysis.py")
COMMON = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COMMON)
PRODUCT = COMMON.PRODUCT
Incomplete = COMMON.Incomplete
POLICY = "policy/engine-analysis.json"
WRAPPER = "scripts/verify-engine-analysis.py"
TESTS = "scripts/tests/test_verify_engine_analysis.py"
SUBJECTS = {
    "library": "crates/nevrotick-engine/src/lib.rs",
    "cli": "crates/nevrotick-engine/src/bin/nevrotick-observe.rs",
    "measurement": "crates/nevrotick-engine/src/measurement.rs",
}
SCOPE = COMMON.Scope("nevrotick-engine", tuple(SUBJECTS.items()), POLICY)
CONTRACT_SCOPE = COMMON.Scope(
    "nevrotick-engine", (("library", "crates/nevrotick-engine/src/lib.rs"),), POLICY)
FIXED_CONTRACT_POLICY = {
    "schema_version": 1,
    "admission_status": "admitted-engine-external-contracts-only",
    "package": "nevrotick-engine", "source_file_count": 168,
    "critical_files": ["crates/nevrotick-engine/src/lib.rs"],
    "tool_admission_profile": "stable-core",
    "contracts_wall_seconds": 300, "jobs": 6, "test_threads": 1,
    "mutation_equivalences": [], "mutation_error_values": {},
}
BOUND_INPUTS = (
    "Cargo.toml", "Cargo.lock", "crates/nevrotick-engine/Cargo.toml",
    *SUBJECTS.values(), "crates/nevrotick-engine/tests/runtime_boundary.rs",
    "crates/nevrotick-engine/tests/prepare_release.rs", "crates/nevrotick-engine/tests/prepare_entered.rs",
    "crates/nevrotick-engine/tests/execute_entered.rs",
    "crates/nevrotick-engine/tests/private/processing_guards.rs",
    "crates/nevrotick-engine/tests/private/argument_guards.rs",
    "crates/nevrotick-engine/tests/private/measurement.rs",
    "scripts/portable-analysis.py", "scripts/analysis-runtime.py", "scripts/check-analysis.py",
    "scripts/prepare-product.py", "policy/analysis-tools.json", "policy/verification-manifest.toml",
)
REQUIRED_TESTS = frozenset({
    "processing_guard_tests::processing_uses_fresh_deadline_and_current_control",
    "processing_guard_tests::terminal_io_causes_and_send_history_remain_distinct",
    "processing_guard_tests::configuration_enforces_endpoint_and_timeout_boundaries",
    "processing_guard_tests::control_schedule_preserves_legal_zero_and_maximum_delays",
    "processing_guard_tests::watch_controls_latch_stop_and_reject_generation_wrap",
    "processing_guard_tests::failure_summaries_preserve_fixed_redacted_cause_projections",
    "processing_guard_tests::owned_timers_deliver_immediate_actions_and_cancel_future_actions",
    "processing_guard_tests::literal_header_rejections_preserve_the_actual_pending_neighbor",
    "processing_guard_tests::entropy_admission_checks_each_half_and_exact_request_bytes",
    "processing_guard_tests::local_endpoint_admission_requires_both_loopback_and_nonzero_port",
    "processing_guard_tests::read_classification_keeps_platform_oversize_and_native_errors_distinct",
    "processing_guard_tests::completed_send_results_preserve_rearm_causes_and_one_way_history",
    "processing_guard_tests::receive_cap_preserves_sixty_three_and_retires_sixty_four",
    "processing_guard_tests::selected_read_accounting_survives_retirement_and_typed_oversize",
    "processing_guard_tests::commit_processing_accepts_equal_read_and_rejects_its_predecessor",
    "processing_guard_tests::completed_cleanup_results_preserve_primary_and_each_independent_error",
    "processing_guard_tests::staged_reply_cannot_cross_socket_owner_or_commit_twice",
    "processing_guard_tests::complete_header_survives_actual_staging_and_terminal_projection",
    "processing_guard_tests::real_udp_measurement_matches_independent_captured_coordinates",
    "processing_guard_tests::measurement_refusal_consumes_the_correlated_request_once",
    "measurement::tests::exact_dyadic_results_ignore_the_wire_token",
    "measurement::tests::peer_shift_changes_sign_without_changing_rtt",
    "measurement::tests::negative_raw_rtt_and_half_q32_phase_are_preserved",
    "measurement::tests::fractional_conversion_residuals_reconstruct_nominal_values",
    "measurement::tests::pre_unix_and_post_unix_100_ns_have_literal_residuals",
    "measurement::tests::absent_profile_is_a_distinct_transport_only_reason",
    "measurement::tests::declared_coherence_limit_has_an_exact_healthy_neighbor",
    "measurement::tests::nonzero_capture_intervals_keep_inside_and_outside_neighbors",
    "measurement::tests::reversed_brackets_and_processing_are_not_ages",
    "measurement::tests::out_of_scope_local_coordinate_is_not_rebased",
    "measurement::tests::send_after_outside_scope_cannot_hide_between_valid_endpoints",
    "measurement::tests::era_zero_and_multiple_candidates_remain_distinct",
    "measurement::tests::inert_header_does_not_bypass_the_time_reply_guard",
    "measurement::tests::profile_bounds_and_maximum_allowance_are_checked",
    "measurement::tests::fresh_owner_age_stamp_preserves_the_actual_arithmetic",
    "measurement::tests::refusal_codes_and_display_remain_literal_and_redacted",
    "measurement::tests::software_window_retains_each_local_and_peer_extremum",
    "measurement::tests::coherence_helper_rejects_each_reversed_bracket",
    "measurement::tests::decimal_remainders_change_the_exact_nominal_phase",
    "measurement::tests::asymmetric_decimal_captures_keep_exact_signed_nominal_values",
    "measurement::tests::pre_unix_nominal_remainders_keep_epoch_and_sign_association",
    "measurement::tests::nominal_arithmetic_rejects_each_remainder_boundary_without_partial_result",
    "measurement::tests::nominal_result_garde_enforces_both_strict_numerator_bounds",
    "measurement::tests::nominal_arithmetic_retains_full_coordinate_domain_without_i64_projection",
    "measurement::tests::nominal_seconds_preserve_corrected_capture_units_and_signed_rtt",
    "measurement::tests::nominal_seconds_checks_scale_and_retains_the_signed_integer_endpoint",
    "prepare_outside_runtime_owns_and_releases_standard_sockets",
    "prepare_rejects_entered_runtime_before_native_startup",
    "execute_rejects_entered_runtime_without_nested_runtime_creation",
    "cli_worker_binary_preserves_literal_transport_contracts",
    "cli_worker_binary_rejects_arguments_and_observes_sink_failures",
    "argument_guard_tests::pair_budget_admits_six_and_preserves_exhausted_failure",
    "argument_guard_tests::argument_text_preserves_exact_byte_boundaries_and_original_borrows",
    "argument_guard_tests::argument_text_rejects_unix_nontext_in_both_roles",
    "argument_guard_tests::decimal_profile_fields_preserve_raw_digit_boundaries",
})
FIXED_POLICY = {
    "schema_version": 1, "package": "nevrotick-engine",
    "critical_files": list(SUBJECTS.values()),
    "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
    "maximum_mutants": 512, "maximum_listed_per_file": 4096, "maximum_listed_total": 8192,
    "coverage_wall_seconds": 300, "enumeration_wall_seconds": 240,
    "mutation_wall_seconds": 1200, "mutation_build_seconds": 90, "mutation_test_seconds": 60,
    "jobs": 6, "test_threads": 1, "coverage_test_threads": 1,
    "mutation_jobs": 10, "mutation_process_limit": 512, "mutation_message_format": "json",
    "tool_admission_profile": "full", "mutation_equivalences": [],
}


def invalid_number(value: str) -> NoReturn:
    """Reject nonfinite tokens at every depth of policy and native report input."""
    raise Incomplete("Engine JSON contains a nonfinite number.")


def finite_number(value: str) -> float:
    """Retain finite native timing values without accepting exponent overflow."""
    number = float(value)
    if not math.isfinite(number):
        invalid_number(value)
    return number


def read_json(path: Path, *, maximum: int = 524288) -> dict:
    """Read bounded regular, nonlinked JSON; reject duplicates and nonfinite values."""
    if path != path.resolve(strict=True):
        raise Incomplete("Engine JSON input cannot use a linked path.")
    data = COMMON.ANALYSIS.read_once(path, maximum=maximum)
    try:
        value = json.loads(data, object_pairs_hook=COMMON.ANALYSIS.unique_object,
                           parse_float=finite_number, parse_constant=invalid_number)
    except (ValueError, UnicodeError, RecursionError) as error:
        raise Incomplete("Invalid bounded engine JSON input.") from error
    if not isinstance(value, dict):
        raise Incomplete("Engine JSON input must be an object.")
    return value


def inventory_bindings(policy: dict, *, required: bool = False) -> dict:
    """Admit either no review yet or complete exact nonempty per-file review pins."""
    bindings = policy.get("mutation_inventories")
    if not isinstance(bindings, dict):
        raise Incomplete("Engine mutation inventory bindings must be an object.")
    if not bindings and not required:
        return {}
    if set(bindings) != set(SUBJECTS):
        raise Incomplete("Campaign requires reviewed nonempty inventories for all three engine files.")
    for binding in bindings.values():
        if (not isinstance(binding, dict) or set(binding) != {"count", "sha256"}
                or type(binding["count"]) is not int or not 0 < binding["count"] <= policy["maximum_mutants"]
                or not isinstance(binding["sha256"], str)
                or not re.fullmatch(r"[a-f0-9]{64}", binding["sha256"])):
            raise Incomplete("Engine inventory review requires exact positive counts and SHA-256 pins.")
    return bindings


def validate_equivalence_scope(entries: object) -> None:
    """Permit no E or only both reviewed coherence-equality cases, without skips.

    Shared admission still binds each complete descriptor, candidate, source and
    independent-review artifact. This scope check supplies no semantic verdict.
    """
    replacements = {"ENGINE-COHERENCE-LOWER-EQUALITY": "<=",
                    "ENGINE-COHERENCE-UPPER-EQUALITY": ">="}
    if type(entries) is not list:
        raise Incomplete("Engine equivalences require an exact list.")
    if not entries:
        return
    if (len(entries) != 2 or any(type(entry) is not dict for entry in entries)
            or any(type(entry.get("id")) is not str for entry in entries)
            or {entry.get("id") for entry in entries} != set(replacements)):
        raise Incomplete("Engine equivalences require both exact reviewed coherence equality IDs.")
    for entry in entries:
        mutant, proof = entry.get("mutant"), entry.get("proof")
        if (type(mutant) is not dict or type(mutant.get("function")) is not dict
                or mutant.get("file") != SUBJECTS["measurement"]
                or mutant["function"].get("function_name") != "coherence_residual"
                or mutant.get("genre") != "BinaryOperator"
                or mutant.get("replacement") != replacements[entry["id"]]
                or type(proof) is not dict
                or proof.get("path") != "tasks/engine-analysis-equivalences.md"):
            raise Incomplete("Engine equivalence leaves its exact coherence owner or review artifact.")


def read_policy(root: Path) -> dict:
    """Bind the fixed engine scope to the actual whole product and protected helpers."""
    policy = read_json(root / POLICY)
    extras = {"candidate_sha256", "inputs_sha256", "mutation_inventories", "scope", "external_contracts"}
    if (set(policy) != set(FIXED_POLICY) | extras
            or any(type(policy.get(key)) is not type(value) or policy.get(key) != value
                   for key, value in FIXED_POLICY.items() if key != "mutation_equivalences")
            or not isinstance(policy["scope"], str) or not 0 < len(policy["scope"]) <= 2048):
        raise Incomplete("Unreviewed engine analysis scope, threshold, budget or profile.")
    validate_equivalence_scope(policy["mutation_equivalences"])
    validate_contract_policy(policy["external_contracts"])
    _, identity = PRODUCT.admit_product(root)
    if (not isinstance(policy["candidate_sha256"], str)
            or policy["candidate_sha256"] != COMMON.canonical_sha256(identity)):
        raise Incomplete("Engine analysis policy differs from the complete current product candidate.")
    pins = policy["inputs_sha256"]
    if (not isinstance(pins, dict) or set(pins) != set(BOUND_INPUTS)
            or any(not isinstance(digest, str) or not re.fullmatch(r"[a-f0-9]{64}", digest)
                   or PRODUCT.sha256(root / name) != digest for name, digest in pins.items())):
        raise Incomplete("Engine analysis protected source/helper bindings changed.")
    manifest = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    for filename in SUBJECTS.values():
        if (manifest.get("critical_files", []).count(filename) != 1
                or sum(module.get("package") == SCOPE.package and filename in module.get("files", [])
                       for module in manifest.get("security_modules", [])) != 1):
            raise Incomplete("Engine file must have one matching critical/security owner.")
    inventory_bindings(policy)
    COMMON.admit_mutation_equivalences(root, SCOPE, policy)
    return policy


def validate_contract_policy(value: dict) -> dict:
    """Admit only the current inert external-contract role, never old campaigns."""
    if (type(value) is not dict or set(value) != set(FIXED_CONTRACT_POLICY)
            or CONTRACT_SCOPE.subjects != COMMON.ENGINE_CONTRACT_SUBJECTS
            or any(type(value.get(key)) is not type(expected) or value[key] != expected
                   for key, expected in FIXED_CONTRACT_POLICY.items())):
        raise Incomplete("Engine external contracts require the exact current inert stable role.")
    return value


def read_contract_policy(root: Path) -> dict:
    """Keep the historical candidate labels separate from current source custody."""
    parent = read_json(root / POLICY)
    extras = {"candidate_sha256", "inputs_sha256", "mutation_inventories", "scope", "external_contracts"}
    if set(parent) != set(FIXED_POLICY) | extras:
        raise Incomplete("Engine historical/contract policy fields are missing or unknown.")
    policy = validate_contract_policy(parent["external_contracts"])
    manifest = tomllib.loads((root / "policy/verification-manifest.toml").read_text())
    filename = policy["critical_files"][0]
    if (manifest.get("critical_files", []).count(filename) != 1
            or sum(module.get("package") == CONTRACT_SCOPE.package and filename in module.get("files", [])
                   for module in manifest.get("security_modules", [])) != 1):
        raise Incomplete("Engine constructor owner lacks its exact critical/security registration.")
    return policy


def contract_examples() -> dict:
    """Exercise private checked plans without creating any runtime or socket.

    Negative APIs are compile-only, including the uncalled schedule helper.
    Every causal statement remains on line four, with all setup values used.
    """
    setup = (
        "let peers = [SocketAddr::from(([127, 0, 0, 1], 41_001)), "
        "SocketAddr::from(([127, 0, 0, 1], 41_002))]; "
        "let timeout = Duration::from_millis(250); "
        "let config = Config::new(peers, timeout)?; "
        "let schedule = ControlSchedule::new(None, None)?; "
        "let _ = (&peers, timeout, &config, &schedule);"
    )
    marker = "legal-engine-contracts:checked-only-no-runtime"
    legal = (
        "assert!(Config::new(peers, Duration::from_secs(30)).is_ok()); "
        "assert!(matches!(Config::new([peers[0], peers[0]], timeout), Err(PilotError::DuplicatePeer))); "
        "assert!(matches!(Config::new(peers, Duration::ZERO), Err(PilotError::InvalidTimeout))); "
        "assert!(matches!(Config::new(peers, Duration::from_secs(30) + Duration::from_nanos(1)), "
        "Err(PilotError::InvalidTimeout))); "
        "let zero_port = SocketAddr::from(([127, 0, 0, 1], 0)); "
        "assert!(matches!(Config::new([zero_port, peers[1]], timeout), Err(PilotError::InvalidPeer))); "
        "assert!(ControlSchedule::new(None, None).is_ok()); "
        "assert!(ControlSchedule::new(Some(Duration::ZERO), Some(Duration::from_secs(30))).is_ok()); "
        "let excessive = Some(Duration::from_secs(30) + Duration::from_nanos(1)); "
        "assert!(matches!(ControlSchedule::new(excessive, None), Err(PilotError::InvalidTimeout))); "
        "assert!(matches!(ControlSchedule::new(None, excessive), Err(PilotError::InvalidTimeout))); "
        'println!("' + marker + '");'
    )
    examples = {
        "legal": {"statement": legal, "code": None, "tokens": [], "success_marker": marker},
        "config_construction": {
            "statement": "let _ = Config { peers: [peers[0], peers[0]], timeout: Duration::ZERO, "
                         "experimental_profile: None };",
            "code": "E0451", "tokens": ["peers", "timeout", "experimental_profile", "private"]},
        "schedule_construction": {
            "statement": "let _ = ControlSchedule { stop_after: Some(Duration::from_secs(31)), "
                         "invalidate_after: Some(Duration::from_secs(31)) };",
            "code": "E0451", "tokens": ["stop_after", "invalidate_after", "private"]},
        "raw_config_prepare": {
            "statement": "let _ = prepare(&(peers, timeout));",
            "code": "E0308", "tokens": ["Config", "mismatched types"]},
        "raw_schedule_execute": {
            "statement": "fn rejected(pilot: PreparedPilot) { let _ = execute(pilot, "
                         "(None::<Duration>, None::<Duration>)); }",
            "code": "E0308", "tokens": ["ControlSchedule", "mismatched types"]},
    }
    for name, value, field, replacement in (
            ("config_peers", "config", "peers", "[peers[0], peers[0]]"),
            ("config_timeout", "config", "timeout", "Duration::ZERO"),
            ("schedule_stop", "schedule", "stop_after", "Some(Duration::from_secs(31))"),
            ("schedule_invalidate", "schedule", "invalidate_after", "Some(Duration::from_secs(31))")):
        examples[name + "_mutation"] = {
            "statement": f"let mut value = {value}; value.{field} = {replacement}; let _ = &value;",
            "code": "E0616", "tokens": [field, "private"]}
    for name, typename, raw in (
            ("config", "Config", "(peers, timeout)"),
            ("schedule", "ControlSchedule", "(None::<Duration>, None::<Duration>)")):
        examples[name + "_default"] = {
            "statement": f"let _: {typename} = Default::default();",
            "code": "E0277", "tokens": [typename, "Default"]}
        examples[name + "_from"] = {
            "statement": f"let _: {typename} = {raw}.into();",
            "code": "E0277", "tokens": [typename, "From"]}
    for specimen in examples.values():
        specimen["source"] = "\n".join([
            "use nevrotick_engine::*; use std::{net::SocketAddr, time::Duration};",
            "fn main() -> Result<(), PilotError> {", setup, specimen.pop("statement"), "Ok(())", "}"]) + "\n"
        specimen["line"] = 4
    return examples


def copy_contract_dependencies(session, artifact: str) -> Path:
    """Freeze actual Cargo-emitted libraries as independent immutable copies."""
    source = session.work / artifact.removeprefix("/work/")
    if (source.resolve(strict=True) != source or not source.is_file()
            or not source.is_relative_to(session.work / "target")):
        raise Incomplete("Actual engine contract library escaped its emitted target.")
    destination = session.work / "contract-deps"
    destination.mkdir(mode=0o700)
    for item in sorted((session.work / "target/debug/deps").iterdir()):
        if item.suffix not in {".rlib", ".rmeta", ".so"}:
            continue
        digest = PRODUCT.sha256(item)
        target = destination / item.name
        shutil.copyfile(item, target)
        target.chmod(0o400)
        if PRODUCT.sha256(target) != digest:
            raise Incomplete("Engine contract dependency changed while copied.")
    library = destination / "libnevrotick_engine.rlib"
    digest = PRODUCT.sha256(source)
    shutil.copyfile(source, library)
    library.chmod(0o400)
    if PRODUCT.sha256(library) != digest:
        raise Incomplete("The copied engine library differs from its actual Cargo artifact.")
    session.protect(destination)
    return destination


def contracts(session, policy: dict, report: dict) -> None:
    """Compile intended external causes and execute only checked constructors."""
    deadline = time.monotonic() + policy["contracts_wall_seconds"]
    output = session.run(
        "contract-library", [str(session.stable / "bin/cargo"), "build", "--package", "nevrotick-engine",
        "--frozen", "--jobs", str(policy["jobs"]), "--lib", "--message-format=json"],
        COMMON.remaining_timeout(deadline, 120))
    artifact = COMMON.library_artifact(output, "nevrotick-engine",
                                       "/work/source/crates/nevrotick-engine/Cargo.toml")
    dependencies = copy_contract_dependencies(session, artifact)
    consumers = session.work / "contract-inputs"
    consumers.mkdir(mode=0o700)
    (session.work / "contracts").mkdir(mode=0o700)
    examples = contract_examples()
    for name, specimen in examples.items():
        (consumers / (name + ".rs")).write_text(specimen["source"])
    session.protect(consumers)
    for name, specimen in examples.items():
        command = [str(session.stable / "bin/rustc"), "--edition", "2024", "--crate-name", name,
                   "--error-format=json", "/work/contract-inputs/" + name + ".rs",
                   "--extern", "nevrotick_engine=/work/contract-deps/libnevrotick_engine.rlib",
                   "-L", "dependency=/work/contract-deps", "--out-dir", "/work/contracts"]
        session.run("external-consumer-" + name, command, COMMON.remaining_timeout(deadline, 60),
                    sealed=[consumers, dependencies], specimen=specimen if specimen["code"] else None)
        if specimen["code"] is None:
            text = session.run("external-legal-execution", ["/work/contracts/" + name],
                               COMMON.remaining_timeout(deadline, 30), sealed=[consumers, dependencies])
            if specimen["success_marker"] not in text:
                raise Incomplete("Legal engine consumer did not reach checked constructor neighbors.")
    report["contracts"] = {
        "legal_consumers_compiled_and_executed": 1,
        "library_artifact": artifact,
        "immutable_dependencies_sha256": PRODUCT.inventory(dependencies),
        "required_diagnostics": {name: {key: specimen[key] for key in ("code", "tokens", "line")}
                                 for name, specimen in examples.items() if specimen["code"]},
        "effect_scope": "Config/ControlSchedule construction only; no runtime, socket or execute calls",
    }


def execute_contracts(args, report: dict) -> None:
    """Qualify only external admission contracts on a separately admitted candidate."""
    policy = read_contract_policy(ROOT)
    if args.subject not in {"library", "all"}:
        raise Incomplete("External engine contracts belong only to the library owner.")
    COMMON.admission_profile(args, CONTRACT_SCOPE, policy)
    report.update(subjects=CONTRACT_SCOPE.select(args.subject), qualification_passed=False,
                  contracts_qualification_passed=False, historical_campaign_bindings_reused=False)
    session = COMMON.Session(ROOT, args, report, CONTRACT_SCOPE, policy, (WRAPPER, TESTS))
    try:
        if read_contract_policy(session.source) != policy:
            raise Incomplete("Captured engine contract policy differs from its admitted current role.")
        report["candidate_sha256"] = COMMON.canonical_sha256(session.product_identity)
        contracts(session, policy, report)
    finally:
        session.finish()
    report["contracts_qualification_passed"] = True


def readiness_proposal() -> dict:
    """Describe a finite single-worker measurement; never execute or admit it."""
    return {
        "kind": "proposal", "executed": False, "outer_workers": 1,
        "inner_cargo_jobs": 1, "test_threads": 1,
        "build_seconds": 90, "test_seconds": 60,
        "commands": [
            ["cargo", "test", "--no-run", "--verbose", "--package=nevrotick-engine@0.0.0", "--frozen"],
            ["cargo", "test", "--verbose", "--package=nevrotick-engine@0.0.0", "--frozen"],
        ],
        "required_observations": ["actual selected tests", "peak allocated worker tree bytes",
                                  "minimum normal-user available bytes", "process custody and actual exits"],
        "ten_worker_readiness": "not assessed; reserve ten measured workers plus stop margin before execution",
        "limits": "existing rlimits and sampled storage guards are not a hard aggregate RAM/disk quota",
    }


def listed_summary(inventories: dict) -> dict:
    """Hash every observed descriptor field except its optional rendered diff."""
    return {name: {"count": len(items), "sha256": COMMON.canonical_sha256({
        key: {field: value for field, value in item.items() if field != "diff"}
        for key, item in items.items()})} for name, items in inventories.items()}


def mutation_prefix(tools: Path, subject: str, policy: dict) -> list[str]:
    """Use the unchanged exact shared campaign validator for the listing prefix."""
    filename = SCOPE.select(subject)[subject]
    prefix = [str(tools / "cargo-mutants"), "mutants", "--package", SCOPE.package,
              "--file", filename, "--test-workspace", "false", "--cap-lints", "false",
              "--jobs", str(policy["mutation_jobs"]), "--jobserver", "true",
              "--jobserver-tasks", str(policy["mutation_jobs"]), "--cargo-arg=--message-format=json"]
    campaign = [*prefix, "--baseline", "run", "--build-timeout", "90", "--timeout", "60",
                "--output", "/work/mutation/" + subject]
    COMMON.mutation_campaign_command(SCOPE, tools, subject, campaign,
                                    expected_jobs=policy["mutation_jobs"], expected_message_format="json")
    return prefix


def enumerate_only(session, subjects: dict, policy: dict, report: dict) -> None:
    """Retain complete bounded listings; launch no baseline or mutation campaign."""
    deadline = time.monotonic() + policy["enumeration_wall_seconds"]
    inventories = {}
    observations = report["mutation_enumeration"] = {}
    for name, filename in subjects.items():
        command = [*mutation_prefix(session.tools, name, policy), "--list", "--json"]
        raw = session.run("mutation-enumeration:" + name, command, COMMON.remaining_timeout(deadline, 120))
        observations[name] = {"raw_output": raw}
        items, warning = COMMON.parse_mutation_listing(raw, expected_jobs=policy["mutation_jobs"])
        observations[name]["warning"] = warning
        inventories[name] = COMMON.mutation_inventory(SCOPE, items, policy["maximum_listed_per_file"], filename)
    total = COMMON.combined_mutant_count(inventories, policy["maximum_listed_total"])
    report["listed_inventories"] = inventories
    report["listed_inventory_bindings"] = listed_summary(inventories)
    report["listed_total"] = total
    report["campaign_cap_exceeded"] = total > policy["maximum_mutants"]
    report["campaign_admission"] = False
    report["qualification_passed"] = False


def validate_test_execution(text: str, *, doctests: bool) -> dict:
    """Require actual successful libtest records, summaries and the required cases."""
    records = re.findall(r"^test ([^\r\n]+?) \.\.\. ok\r?$", text, re.MULTILINE)
    summaries = [line for line in text.splitlines() if line.startswith("test result:")]
    pattern = r"test result: ok\. (\d+) passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;.*"
    counts = [re.fullmatch(pattern, line) for line in summaries]
    if (not summaries or any(match is None for match in counts) or len(records) != len(set(records))
            or sum(int(match[1]) for match in counts if match is not None) != len(records)
            or not REQUIRED_TESTS <= set(records)):
        raise Incomplete("Engine execution omits required actual passing unit/integration records or complete summaries.")
    docs = [name for name in records if re.fullmatch(
        r"[^\r\n]+\.rs - [^\r\n]+ \(line [1-9][0-9]*\)(?: - compile fail)?", name)]
    doc_pattern = re.compile(
        r"(?:[^\r\n]+/)?crates/nevrotick-engine/src/(lib|measurement)\.rs - "
        r"(Config::new|ControlSchedule::new|measurement::ExperimentalProfile|"
        r"measurement::ExperimentalMeasurement|measurement::NominalPhaseNs|"
        r"measurement::NominalRoundTripNs) \(line [1-9][0-9]*\)( - compile fail)?")
    signatures = Counter((match[1], match[2], bool(match[3]))
                         for name in docs if (match := doc_pattern.fullmatch(name)) is not None)
    expected = Counter({("lib", "Config::new", False): 1,
                        ("lib", "ControlSchedule::new", False): 1,
                        ("measurement", "measurement::ExperimentalProfile", False): 1,
                        ("measurement", "measurement::ExperimentalProfile", True): 4,
                        ("measurement", "measurement::ExperimentalMeasurement", False): 1,
                        ("measurement", "measurement::ExperimentalMeasurement", True): 4,
                        ("measurement", "measurement::NominalPhaseNs", False): 1,
                        ("measurement", "measurement::NominalPhaseNs", True): 4,
                        ("measurement", "measurement::NominalRoundTripNs", False): 1,
                        ("measurement", "measurement::NominalRoundTripNs", True): 4})
    if doctests and (signatures != expected or len(docs) != sum(expected.values())):
        raise Incomplete("Engine mutation baseline omits its actual construction/privacy doctests.")
    if not doctests and docs:
        raise Incomplete("Coverage selection unexpectedly includes doctests.")
    return {"tests_executed": len(records), "required_tests": sorted(REQUIRED_TESTS), "doctests": len(docs)}


def validate_baseline(work: Path, subject: str) -> dict:
    """Inspect the tool's actual source-bound baseline before accepting any score."""
    directory = work / "mutation" / subject / "mutants.out"
    outcomes = read_json(directory / "outcomes.json", maximum=64 * 1024 * 1024)
    baseline = [item for item in outcomes.get("outcomes", [])
                if isinstance(item, dict) and item.get("scenario") == "Baseline"]
    if len(baseline) != 1 or baseline[0].get("summary") != "Success":
        raise Incomplete("Engine mutation requires exactly one successful actual baseline.")
    relative = Path(baseline[0].get("log_path", ""))
    if relative.is_absolute() or not relative.parts or ".." in relative.parts:
        raise Incomplete("Engine baseline log escapes its owned output.")
    path = directory / relative
    if path != path.resolve(strict=True):
        raise Incomplete("Engine baseline log cannot use a linked path.")
    data = COMMON.ANALYSIS.read_once(path, maximum=16 * 1024 * 1024)
    return {**validate_test_execution(data.decode("utf-8"), doctests=True), "log_sha256": PRODUCT.sha256(path)}


def execute(args, report: dict) -> None:
    """Execute only the fixed selected lane, with full shared custody bookends."""
    if args.lane == "contracts":
        execute_contracts(args, report)
        return
    if args.lane not in {"enumerate", "coverage", "mutation", "all"}:
        raise Incomplete("Unknown engine analysis lane.")
    policy = read_policy(ROOT)
    subjects = SCOPE.select(args.subject)
    if args.lane in {"mutation", "all"}:
        bindings = inventory_bindings(policy, required=True)
        if sum(bindings[name]["count"] for name in subjects) > policy["maximum_mutants"]:
            raise Incomplete("Selected complete mutation inventory exceeds the shared campaign cap.")
    session_args = argparse.Namespace(**vars(args))
    if args.lane == "enumerate":
        session_args.lane = "mutation"  # Select only the admitted extension, not campaign execution.
    report.update(subjects=subjects, candidate_sha256=policy["candidate_sha256"],
                  qualification_passed=False, readiness_proposal=readiness_proposal())
    session = COMMON.Session(ROOT, session_args, report, SCOPE, policy, (WRAPPER, TESTS, *BOUND_INPUTS))

    def run(label, command, timeout, **options):
        result = session.run(label, command, timeout, **options)
        if label == "coverage":
            report["coverage_execution"] = validate_test_execution(result, doctests=False)
        elif label.startswith("mutation-campaign:"):
            subject = label.removeprefix("mutation-campaign:")
            report.setdefault("baseline_execution", {})[subject] = validate_baseline(session.work, subject)
        return result

    try:
        if COMMON.canonical_sha256(session.product_identity) != policy["candidate_sha256"]:
            raise Incomplete("Captured engine Session differs from the policy's product candidate.")
        if any(PRODUCT.sha256(session.source / name) != policy["inputs_sha256"][name]
               for name in BOUND_INPUTS):
            raise Incomplete("Captured engine Session protected input pins changed.")
        if COMMON.canonical_sha256(read_json(session.source / POLICY)) != COMMON.canonical_sha256(policy):
            raise Incomplete("Captured engine Session policy differs from the admitted policy.")
        if args.lane == "enumerate":
            enumerate_only(session, subjects, policy, report)
        else:
            # Share the exact source/tool snapshot and final custody boundary.
            # A failed coverage gate never starts a mutation campaign.
            lanes = ("coverage", "mutation") if args.lane == "all" else (args.lane,)
            for lane in lanes:
                COMMON.run_analysis_lanes(SCOPE, subjects, policy, session.work, session.tools, run, report, lane)
    finally:
        session.finish()
    if args.lane != "enumerate":
        report["qualification_passed"] = True


def main(argv=None) -> int:
    """Keep listing completion distinct from selected host coverage/mutation gates."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", choices=("contracts", "enumerate", "coverage", "mutation", "all"), required=True)
    parser.add_argument("--subject", choices=(*SUBJECTS, "all"), default="all")
    parser.add_argument("--tool-admission-profile", choices=("full", "stable-core"), default="full")
    parser.add_argument("--vendor-dir", type=Path)
    parser.add_argument("--output-dir", type=Path, required=True)
    args = parser.parse_args(argv)
    report = {"status": "incomplete", "scope": "Bounded host engine " + args.lane,
              "selected_lane": args.lane, "qualification_passed": False,
              "native_windows_qualification": "not performed"}
    try:
        if args.vendor_dir is None:
            args.vendor_dir = Path(read_json(PRODUCT.POINTER)["vendor_dir"])
        execute(args, report)
        report["status"] = "enumerated" if args.lane == "enumerate" else "passed"
    except (Incomplete, PRODUCT.ENV.Incomplete, OSError, ValueError, KeyError, TypeError, StopIteration) as error:
        report["qualification_passed"] = False
        report["contracts_qualification_passed"] = False
        report["error"] = str(error)
    if report.get("owned_output") == str(args.output_dir):
        (args.output_dir / "verification.json").write_text(json.dumps(report, indent=2, allow_nan=False) + "\n")
    print(report["status"].upper() + ": " + report["scope"], flush=True)
    if report.get("error"):
        print(report["error"], file=sys.stderr)
    return 0 if report["status"] in {"enumerated", "passed"} else 1


if __name__ == "__main__":
    raise SystemExit(main())
