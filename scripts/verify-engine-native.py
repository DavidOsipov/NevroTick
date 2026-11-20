#!/usr/bin/env python3
"""Build and separately supervise bounded Engine Windows tests (GPL-2.0-only).

The build lane executes only admitted offline WSL tools. It grants no native
execution, installation, elevation, clock, service or security-setting effects.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
from pathlib import Path
import re
import shutil
import stat
import sys
import time
import tomllib

sys.dont_write_bytecode = True
ROOT = Path(__file__).resolve().parents[1]
SELF = "scripts/verify-engine-native.py"
LAUNCHER = "tools/windows-sandbox/gated-console-launcher.rs"
CPU = "tools/windows-sandbox/cpu-os-v3-admission.rs"
FACADE_CONSUMER = "tools/windows-sandbox/engine-facade-consumer.rs"
FACADE_EOF_MODE = "facade-loopback-eof"
FACADE_EOF_WITNESS = "facade-peer=clienthello-eof-joined\n"
FACADE_EOF_PREFIX = "scope=engine-ke-prototype eligible=0 fresh_sync=0 stage=handshake cause=unexpected-eof "
FACADE_CASES = {
    "facade-arguments": (2, "scope=engine-ke-prototype eligible=0 fresh_sync=0 stage=admission cause=arguments "
                         "cleanup_stage=none cleanup_cause=none admission_stage=none admission_cause=none\n"),
    "facade-entered-runtime": (1, "scope=engine-ke-prototype eligible=0 fresh_sync=0 stage=caller-context "
                               "cause=entered-runtime retirement_confirmed=0\n"),
    FACADE_EOF_MODE: (1, FACADE_EOF_PREFIX +
                      "cleanup_stage=none cleanup_cause=none admission_stage=none admission_cause=none\n"),
}
FACADE_EOF_BUFFERED = FACADE_EOF_PREFIX + ("cleanup_stage=response cleanup_cause=unexpected-eof "
                                         "admission_stage=none admission_cause=none\n")
MANAGED_EOF_MODE = "facade-managed-protected-eof"
MANAGED_EOF_ROW = ("scope=managed-protected-native eligible=0 fresh_sync=0 stale=configuration-changed "
                   "current=handshake/unexpected-eof cleanup=none admission=none retained=none "
                   "power_stop_failed=0 observer_close_error=none peer_joined=1 owned_udp_received=0\n")
MANAGED_EOF_BUFFERED = MANAGED_EOF_ROW.replace("cleanup=none", "cleanup=response/unexpected-eof")
FACADE_CASES[MANAGED_EOF_MODE] = (0, MANAGED_EOF_ROW)
TARGET = "x86_64-pc-windows-msvc"
TEST_ROSTER_NAME = "windows-test-roster.json"
MAX_TEST_ROSTER_BYTES = 1024 * 1024
MAX_LAUNCHER_SOURCE_BYTES = 64 * 1024
FACADE_FAULT_SOURCE = "crates/nevrotick-engine/src/nts_ke_experiment/mod.rs"
FACADE_FAULT_SOURCE_SHA256 = "fda1bb48ac9751bb70a31f9501f7f8cca04de82b4481c1beab9c5e456c173287"
FACADE_FAULT_OUTCOMES_SHA256 = "cee574bd36df9cbf1b7f2042ea2cf4aafb7f194366a19fe686e5fe94e70d3ca0"
FACADE_HEALTHY_FIELDS = ("product_source_sha256", "extra_inputs_sha256", "candidate_sha256", "tools_sha256",
                         "stable_payload", "toolchain", "vendor", "xwin_config", "xwin_inventory", "windows_inputs",
                         "profile", "diagnostic_profile", "product_target_flags", "facade_target_flags",
                         "diagnostic_target_flags")
# Fixed V19 descriptors, not caller-selected Rust spans or replacement programs.
FACADE_FAULTS = (
    ("run-default", "run", "-> Result<datagram::ReceivedCompletion, RunFailed>",
     (889, 1, 897, 2), (894, 5, 896, 7), "Ok(Default::default())",
     "286aa05c9850dc196b7802b37c71ce420ba93bb9b5666b92fd784dee21593f92"),
    ("report-run-failure-default", "report_run_failure", "-> std::process::ExitCode",
     (947, 1, 952, 2), (950, 5, 951, 36), "Default::default()",
     "70f2389f06be3dd4072feafcd04aec04cb90913f02824c063a445954a80d83a6"),
    ("report-failure-default", "report_failure", "-> std::process::ExitCode",
     (1063, 1, 1068, 2), (1066, 5, 1067, 9), "Default::default()",
     "3e05f53f0fd4cc424b1ae2dfa7482a30bddb075e3a3e5a192f47bedee6c4b5b7"),
    ("entry-default", "entry", "-> std::process::ExitCode",
     (1251, 1, 1274, 2), (1258, 5, 1273, 6), "Default::default()",
     "aba04af7225bf2d9850e76db7aea2c5606175442f4790acfd501d4308f89bdd5"),
)
HELD_KE_TESTS = (
    "nts_ke_experiment::job::tests::job_native_foreign_origin_records_control_before_actual_retirement",
    "nts_ke_experiment::job::tests::provider_native_authoritative_once_preserves_permanent_admission",
    "nts_ke_experiment::job::tests::provider_native_foreign_origins_refuse_before_effect_factory",
)
# These deliberate ignored fixtures need their own peer/trust/isolated-process
# controllers. They remain required native work outside this scripted selection.
MANUAL_NATIVE_TESTS = (
    "nts_ke_experiment::transport::certificate_time_fixture::pre_epoch_certificate_handshake_preserves_local_clock_failure",
    "nts_ke_experiment::transport::certificate_time_fixture::ordinary_certificate_handshake_preserves_certificate_failure",
    "nts_ke_experiment::transport::managed_close_fixture::managed_protected_native_read_keeps_stale_revision_as_its_only_tls_failure",
    "nts_ke_experiment::transport::managed_close_fixture::managed_protected_public_probe_joins_one_authenticated_reply",
    "nts_ke_experiment::datagram::tests::recovery_tests::worker_tests::native_provider_managed_loopback_successor_commits_exact_deny_after_normal_join",
)
NESTED_TEST_MODES = dict(zip(("job-native-foreign", "provider-native-once", "provider-native-foreign"),
                           HELD_KE_TESTS, strict=True))
NESTED_CONTROL_MODES = ("control-nested-healthy", "control-nested-fail", "control-nested-wait")


# The default profile is a selected reply-guard child, not the complete library.
# Every mode runs one code-owned exact case under the unchanged flat process cap.
DEFAULT_COHORT_CASES = {
    'default-pilot-stop': 'processing_guard_tests::pilot_final_guard_stop_rejects_selected_reply_once',
    'default-pilot-invalidate': 'processing_guard_tests::pilot_final_guard_invalidation_rejects_selected_reply_once',
    'default-pilot-healthy': 'processing_guard_tests::processing_uses_fresh_deadline_and_current_control',
    'default-series-stop': 'series::tests::reply_final_guard_stop_keeps_its_actual_receiving_checkpoint',
    'default-series-invalidate': 'series::tests::reply_final_guard_invalidation_keeps_its_actual_receiving_checkpoint',
    'default-series-healthy': 'series::tests::reply_postcommit_stop_preserves_the_historical_receiving_checkpoint',
    'default-writable-stop': 'series::tests::writable_final_guard_stop_keeps_its_actual_receiving_checkpoint',
    'default-writable-invalidate': 'series::tests::writable_final_guard_invalidation_keeps_its_actual_receiving_checkpoint',
    'default-policy-poison': 'source_policy::tests::final_commit_poisoned_policy_counts_the_selected_reply_once',
    'default-policy-reenable': 'source_policy::tests::final_commit_reenabled_policy_counts_the_selected_reply_once',
    'default-session-checkpoints': 'session::tests::public_session_receiving_checkpoint_races_keep_reuse_and_retirement',
    'default-policy-receive-chronology':
        'source_policy::tests::managed_plain_reversed_receive_leaves_control_pending_until_ordered_reply',
    'default-resume-register-close':
        'processing_guard_tests::native_resume_subscription_registers_and_closes',
}


# This separate plain selection exercises the writer without a CLI subprocess.
PLACEMENT_COHORT_CASES = {
    "placement-managed-writer": "source_policy::tests::managed_header_writes_receive_to_commit_age_only_for_ordered_capture",
    "placement-nominal-writer": "processing_guard_tests::real_udp_measurement_matches_independent_captured_coordinates",
}


def default_cohort_requested(args):
    """Select only the explicit default cohort, separately from the KE facade."""
    selected = getattr(args, "default_cohort", False)
    require(type(selected) is bool and not (selected and getattr(args, "facade_consumer", False)),
            "Default cohort and facade consumer are distinct closed selections.")
    return selected


def placement_cohort_requested(args):
    """Admit only the independent plain application placement selection."""
    selected = getattr(args, "placement_cohort", False)
    require(type(selected) is bool, "Placement cohort selection must be Boolean.")
    require(not (selected and (getattr(args, "default_cohort", False) or getattr(args, "facade_consumer", False))),
            "Placement cohort excludes default and facade selections.")
    return selected


def facade_loopback_eof_requested(args):
    """Add the owned loopback observation to the native facade invocation only."""
    selected = getattr(args, "facade_loopback_eof", False)
    require(type(selected) is bool
            and (not selected or (args.lane == "native" and args.facade_consumer is True)),
            "Facade loopback EOF requires the native facade cohort.")
    return selected


def facade_managed_protected_eof_requested(args):
    """Select one real public managed caller independently of the raw EOF case."""
    selected = getattr(args, "facade_managed_protected_eof", False)
    require(type(selected) is bool
            and (not selected or (args.lane == "native" and args.facade_consumer is True
                                  and getattr(args, "facade_loopback_eof", False) is False)),
            "Managed Protected EOF requires its separate native facade selection.")
    return selected


def facade_native_modes(loopback_eof, managed_protected_eof=False):
    """Keep provider-free defaults; add only one explicit fixed loopback case."""
    require(type(loopback_eof) is bool and type(managed_protected_eof) is bool
            and not (loopback_eof and managed_protected_eof),
            "Facade EOF selections must be separate Booleans.")
    return tuple(mode for mode in FACADE_CASES
                 if (mode != FACADE_EOF_MODE or loopback_eof)
                 and (mode != MANAGED_EOF_MODE or managed_protected_eof))


def engine_test_command(jobs, *, default_cohort=False):
    """Construct the exact offline Windows library build for one closed feature set."""
    require(type(jobs) is int and 1 <= jobs <= 12 and type(default_cohort) is bool,
            "Unadmitted Engine build concurrency or default selection.")
    features = (["--no-default-features"] if default_cohort
                else ["--features", "ke-association-prototype"])
    return ["cargo", "--config", f'target.{TARGET}.linker="/work/linker"', "test",
            "--package", "nevrotick-engine", "--lib", *features, "--frozen", "--jobs", str(jobs),
            "--target", TARGET, "--no-run", "--message-format=json"]


def mode_profile(mode):
    """Own the finite test topology; requests cannot supply counts, images or arguments."""
    if type(mode) is str and (mode in DEFAULT_COHORT_CASES or mode in PLACEMENT_COHORT_CASES):
        return 4, 2, "engine-tests.exe"
    if type(mode) is str and mode in NESTED_TEST_MODES:
        return 6, 3, "engine-tests.exe"
    if type(mode) is str and mode in NESTED_CONTROL_MODES:
        return 6, 3, "gate-control.exe"
    if type(mode) is str and mode in FACADE_CASES:
        return 4, 2, "engine-facade-consumer.exe"
    if type(mode) is str and mode in ("cpu", "list", "ke", "control-healthy", "control-fail", "control-wait"):
        return 4, 2, ("cpu-v3-probe.exe" if mode == "cpu" else
                      "engine-tests.exe" if mode in ("list", "ke") else "gate-control.exe")
    raise Incomplete("Unadmitted fixed native process mode.")


def load(name, relative):
    """Import one fixed repository helper without invoking its CLI."""
    specification = importlib.util.spec_from_file_location(name, ROOT / relative)
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


ALLOCATION = load("engine_native_allocation", "scripts/verify-time-domain-windows-allocation.py")
PRODUCT, ANALYSIS, ENV = ALLOCATION.PRODUCT, ALLOCATION.ANALYSIS, ALLOCATION.ENV
PE = load("engine_native_pe", "scripts/cross-link-service-fixture.py")
Incomplete = ENV.Incomplete
EXTRAS = (SELF, LAUNCHER, CPU, FACADE_CONSUMER, "scripts/verify-time-domain-windows-allocation.py",
          "scripts/cross-link-service-fixture.py", "tools/windows-service-fixture/qualification.py",
          "scripts/portable-analysis.py", "scripts/prepare-product.py", "scripts/check-analysis.py", "scripts/analysis-runtime.py",
          "scripts/analysis-controls.py", "policy/analysis-tools.json", "scripts/tests/test_engine_native.py",
          "scripts/tests/fixtures/engine-facade-fault-v19.rs.txt")
CONTROL_SOURCE = '''//! Fixed healthy, failing and waiting native-supervisor controls.
use std::{
    env,
    ffi::OsStr,
    process::{Command, ExitCode, Stdio},
    thread,
    time::Duration,
};

/// Start one exact same-image descendant and preserve its finite exit or live wait.
/// The supervisor owns timeout cleanup; metadata grace is no acknowledgement.
fn nested(mode: &str) -> ExitCode {
    let (argument, output) = match mode {
        "healthy" => ("nested-child-healthy", "GATE_NESTED_HEALTHY"),
        "fail" => ("nested-child-fail", "GATE_NESTED_FAIL"),
        "wait" => ("nested-child-wait", "GATE_NESTED_WAIT"),
        _ => return ExitCode::from(126),
    };
    let Ok(image) = env::current_exe() else {
        return ExitCode::from(126);
    };
    let Ok(mut child) = Command::new(image)
        .arg(argument)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .spawn()
    else {
        return ExitCode::from(126);
    };
    println!("{output}");
    let status = child.wait();
    thread::sleep(Duration::from_millis(250));
    drop(child);
    match status.ok().and_then(|value| value.code()) {
        Some(0) => ExitCode::SUCCESS,
        Some(7) => ExitCode::from(7),
        _ => ExitCode::from(126),
    }
}

/// Accept one closed mode and perform only its finite synthetic behavior.
fn main() -> ExitCode {
    let mut arguments = env::args_os().skip(1);
    let selected = arguments.next();
    if arguments.next().is_some() {
        return ExitCode::from(126);
    }
    match selected.as_deref() {
        Some(value) if value == OsStr::new("healthy") => {
            println!("GATE_CONTROL_HEALTHY");
            ExitCode::SUCCESS
        }
        Some(value) if value == OsStr::new("fail") => {
            println!("GATE_CONTROL_FAIL");
            ExitCode::from(7)
        }
        Some(value) if value == OsStr::new("wait") => {
            println!("GATE_CONTROL_WAIT");
            thread::sleep(Duration::from_secs(60));
            ExitCode::SUCCESS
        }
        Some(value) if value == OsStr::new("nested-healthy") => nested("healthy"),
        Some(value) if value == OsStr::new("nested-fail") => nested("fail"),
        Some(value) if value == OsStr::new("nested-wait") => nested("wait"),
        Some(value) if value == OsStr::new("nested-child-healthy") => {
            thread::sleep(Duration::from_millis(250));
            ExitCode::SUCCESS
        }
        Some(value) if value == OsStr::new("nested-child-fail") => {
            thread::sleep(Duration::from_millis(250));
            ExitCode::from(7)
        }
        Some(value) if value == OsStr::new("nested-child-wait") => {
            thread::sleep(Duration::from_secs(60));
            ExitCode::SUCCESS
        }
        _ => ExitCode::from(126),
    }
}
'''


def require(condition, message):
    """Reject incomplete admission even under optimized Python."""
    if not condition:
        raise Incomplete(message)


def read_record(path):
    """Reuse bounded descriptor/ancestor custody and strict analyzer JSON admission."""
    return PE.Q.C.read_analysis_json(path, maximum=ALLOCATION.MAX_JSON)


def facade_fault_sources(source: bytes, outcomes_record: dict) -> list[dict]:
    """Prepare only four exact V19 overlays; grant no build or execution result."""
    require(type(source) is bytes and len(source) <= 256 * 1024
            and hashlib.sha256(source).hexdigest() == FACADE_FAULT_SOURCE_SHA256,
            "Facade fault preparation requires the exact unchanged Engine module.")
    require(type(outcomes_record) is dict and type(outcomes_record.get("outcomes")) is list
            and len(outcomes_record["outcomes"]) <= 513,
            "Facade fault outcomes must be a bounded record list.")
    missed = []
    for row in outcomes_record["outcomes"]:
        require(type(row) is dict, "Facade outcome row must be an object.")
        if row.get("summary") == "MissedMutant":
            scenario = row.get("scenario")
            require(type(scenario) is dict and set(scenario) == {"Mutant"}
                    and type(scenario["Mutant"]) is dict,
                    "Facade missed outcome lacks one complete descriptor.")
            missed.append(scenario["Mutant"])
    require(len(missed) == 4, "Facade preparation requires exactly four actual missed descriptors.")
    try:
        identities = [json.dumps(item, sort_keys=True, separators=(",", ":"), allow_nan=False)
                      for item in missed]
    except (TypeError, ValueError, RecursionError) as error:
        raise Incomplete("Facade descriptor is not a bounded JSON identity.") from error
    require(len(set(identities)) == 4, "Duplicate facade missed descriptor.")
    lines = source.splitlines(keepends=True)
    prepared = []
    for identifier, function, returns, function_span, span, replacement, diff_sha in FACADE_FAULTS:
        def coordinates(value):
            return {"start": {"line": value[0], "column": value[1]},
                    "end": {"line": value[2], "column": value[3]}}
        name = f"{FACADE_FAULT_SOURCE}:{span[0]}:{span[1]}: replace {function} {returns} with {replacement}"
        mutant = {"name": name, "package": "nevrotick-engine", "file": FACADE_FAULT_SOURCE,
                  "function": {"function_name": function, "return_type": returns,
                               "span": coordinates(function_span)},
                  "span": coordinates(span), "replacement": replacement, "genre": "FnValue"}
        identity = json.dumps(mutant, sort_keys=True, separators=(",", ":"))
        require(identity in identities, "Missing, extra or altered facade descriptor.")
        start = sum(map(len, lines[:span[0] - 1])) + span[1] - 1
        end = sum(map(len, lines[:span[2] - 1])) + span[3] - 1
        changed = source[:start] + (replacement + " /* ~ changed by cargo-mutants ~ */").encode() + source[end:]
        # Serialize the single code-owned span directly. Sequence matching can
        # misalign entry's identical inner/outer braces despite identical bytes.
        first, last = span[0] - 1, span[2]
        context_start, context_end = max(0, first - 8), min(len(lines), last + 8)
        old_count = context_end - context_start
        new_count = old_count - (last - first) + 1
        header = (f"--- {FACADE_FAULT_SOURCE}\n"
                  f"+++ replace {function} {returns} with {replacement}\n"
                  f"@@ -{context_start + 1},{old_count} +{context_start + 1},{new_count} @@\n").encode()
        diff = (header + b"".join(b" " + line for line in lines[context_start:first])
                + b"".join(b"-" + line for line in lines[first:last])
                + b"+" + changed.splitlines(keepends=True)[first]
                + b"".join(b" " + line for line in lines[last:context_end]))
        require(hashlib.sha256(diff).hexdigest() == diff_sha,
                "Generated facade overlay differs from the exact retained raw diff.")
        raw_diff = "diff/crates__nevrotick-engine__src__nts_ke_experiment__mod.rs_line_" + str(span[0]) + "_col_" + str(span[1]) + ".diff"
        prepared.append({"id": identifier, "mutant": mutant, "source": changed, "diff": diff,
                         "raw_diff_path": raw_diff, "source_sha256": hashlib.sha256(changed).hexdigest(),
                         "diff_sha256": diff_sha})
    return prepared


def prepare_facade_faults(args, report):
    """Seal exact future overlay inputs without invoking tools, builds or native work."""
    read = PE.Q.C.read_analysis_bytes
    source_path = ROOT / FACADE_FAULT_SOURCE
    raw_path = None
    output_binding = None
    before = None
    source = None
    helpers = None
    raw_inputs = {}
    written = {}
    original_error = None
    report.update(scope="Four exact Windows facade source overlays; preparation only, no execution or qualification.",
                  source_custody_unchanged=False, helper_custody_unchanged=False,
                  raw_custody_unchanged=False, output_custody_unchanged=False)
    try:
        require(isinstance(getattr(args, "raw_outcomes", None), Path),
                "Facade preparation requires the retained raw outcomes path.")
        raw_path = args.raw_outcomes.absolute()
        output_info = args.output_dir.lstat()
        require(stat.S_ISDIR(output_info.st_mode) and not stat.S_ISLNK(output_info.st_mode)
                and args.output_dir == args.output_dir.resolve(strict=True),
                "Facade preparation output must be its fresh physical directory.")
        output_binding = (output_info.st_dev, output_info.st_ino, output_info.st_mode)
        source = read(source_path, maximum=256 * 1024)
        helpers = {name: hashlib.sha256(read(ROOT / name, maximum=4 * 1024 * 1024)).hexdigest()
                   for name in EXTRAS}
        before = PRODUCT.admit_product(ROOT)[1]
        raw = read(raw_path, maximum=ALLOCATION.MAX_JSON)
        raw_inputs[raw_path] = hashlib.sha256(raw).hexdigest()
        require(raw_inputs[raw_path] == FACADE_FAULT_OUTCOMES_SHA256,
                "Facade outcomes differ from the exact retained V19 bytes.")
        faults = facade_fault_sources(source, ALLOCATION.decode(raw))
        for fault in faults:
            path = raw_path.parent / fault["raw_diff_path"]
            data = read(path, maximum=64 * 1024)
            raw_inputs[path] = hashlib.sha256(data).hexdigest()
            require(data == fault["diff"] and raw_inputs[path] == fault["diff_sha256"],
                    "Retained facade diff differs from the code-owned overlay.")
        records = []
        for fault in faults:
            directory = args.output_dir / fault["id"]
            directory.mkdir(mode=0o700)
            record = {key: fault[key] for key in ("id", "mutant", "raw_diff_path", "source_sha256", "diff_sha256")}
            for field, name in (("source", "mod.rs"), ("diff", "mutation.diff")):
                relative = fault["id"] + "/" + name
                destination = args.output_dir / relative
                with destination.open("xb") as stream:
                    written[relative] = fault[field]
                    stream.write(fault[field])
                record[field + "_path"] = relative
            records.append(record)
        manifest = {"schema_version": 1, "status": "prepared-not-executed",
                    "source_path": FACADE_FAULT_SOURCE, "source_sha256": FACADE_FAULT_SOURCE_SHA256,
                    "candidate_sha256": ALLOCATION.canonical_digest(before), "source_file_count": len(before),
                    "extra_inputs_sha256": helpers, "raw_outcomes_sha256": FACADE_FAULT_OUTCOMES_SHA256,
                    "faults": records,
                    "scope": "Future isolated build overlay supply only; compiler classification and native sensitivity remain open."}
        data = (json.dumps(manifest, indent=2) + "\n").encode()
        with (args.output_dir / "facade-faults.json").open("xb") as stream:
            written["facade-faults.json"] = data
            stream.write(data)
        report.update(status="prepared-not-executed", manifest_path=str(args.output_dir / "facade-faults.json"),
                      manifest_sha256=hashlib.sha256(data).hexdigest(), prepared_fault_count=len(records))
    except BaseException as error:
        original_error = error
        report["status"] = "incomplete"
        raise
    finally:
        errors = []
        checks = (
            ("source_custody_unchanged", lambda: source is not None and before is not None
             and read(source_path, maximum=256 * 1024) == source and PRODUCT.admit_product(ROOT)[1] == before),
            ("helper_custody_unchanged", lambda: helpers is not None and helpers == {
                name: hashlib.sha256(read(ROOT / name, maximum=4 * 1024 * 1024)).hexdigest() for name in EXTRAS}),
            ("raw_custody_unchanged", lambda: bool(raw_inputs) and all(
                hashlib.sha256(read(path, maximum=ALLOCATION.MAX_JSON)).hexdigest() == digest
                for path, digest in raw_inputs.items())),
        )
        for key, check in checks:
            try:
                report[key] = check()
                if not report[key]:
                    errors.append(key)
            except Exception:
                errors.append(key)
        try:
            info = args.output_dir.lstat()
            require(output_binding == (info.st_dev, info.st_ino, info.st_mode)
                    and not stat.S_ISLNK(info.st_mode)
                    and args.output_dir == args.output_dir.resolve(strict=True),
                    "Facade preparation output directory changed.")
            files, directories = set(), {"."}
            for index, path in enumerate(args.output_dir.rglob("*"), 1):
                require(index <= 32, "Facade preparation output exceeds its fixed entry bound.")
                info = path.lstat()
                relative = str(path.relative_to(args.output_dir))
                require(not stat.S_ISLNK(info.st_mode), "Redirected facade preparation output.")
                if stat.S_ISDIR(info.st_mode):
                    directories.add(relative)
                else:
                    require(stat.S_ISREG(info.st_mode), "Nonregular facade preparation output.")
                    files.add(relative)
                    require(relative in written and read(path, maximum=256 * 1024) == written[relative],
                            "Facade preparation output bytes changed.")
            expected_dirs = {".", *(str(Path(name).parent) for name in written)}
            require(files == set(written) and directories == expected_dirs,
                    "Facade preparation output paths changed.")
            report["output_custody_unchanged"] = True
        except Exception:
            errors.append("output_custody_unchanged")
        report["custody_errors"] = errors
        if errors:
            report["status"] = "incomplete"
            if original_error is None:
                raise Incomplete("Facade preparation custody changed: " + ", ".join(errors))


def read_prepared_facade_fault(receipt_path: Path, fault_id: str, inputs: dict, extras: dict) -> dict:
    """Admit all four current physical overlays before returning one fixed selection."""
    require(type(fault_id) is str and fault_id in {row[0] for row in FACADE_FAULTS},
            "Unadmitted facade fault ID.")
    read = PE.Q.C.read_analysis_bytes
    path = receipt_path.absolute()
    raw = read(path, maximum=ALLOCATION.MAX_JSON)
    receipt = ALLOCATION.decode(raw)
    require(type(receipt) is dict and type(receipt.get("schema_version")) is int and receipt["schema_version"] == 1
            and receipt.get("lane") == "prepare-facade-faults"
            and receipt.get("status") == "prepared-not-executed" and "error" not in receipt
            and type(receipt.get("prepared_fault_count")) is int and receipt["prepared_fault_count"] == 4
            and receipt.get("custody_errors") == []
            and all(receipt.get(key) is True for key in ("source_custody_unchanged", "helper_custody_unchanged",
                                                        "raw_custody_unchanged", "output_custody_unchanged")),
            "Facade preparation receipt is incomplete or lacks successful custody.")
    manifest_path = path.parent / "facade-faults.json"
    manifest_raw = read(manifest_path, maximum=ALLOCATION.MAX_JSON)
    digest = hashlib.sha256(manifest_raw).hexdigest()
    require(receipt.get("manifest_path") == str(manifest_path) and receipt.get("manifest_sha256") == digest,
            "Facade manifest differs from its preparation receipt.")
    manifest = ALLOCATION.decode(manifest_raw)
    require(type(manifest) is dict and set(manifest) == {
                "schema_version", "status", "source_path", "source_sha256", "candidate_sha256",
                "source_file_count", "extra_inputs_sha256", "raw_outcomes_sha256", "faults", "scope"}
            and type(manifest.get("schema_version")) is int and manifest["schema_version"] == 1
            and manifest.get("status") == "prepared-not-executed"
            and manifest.get("source_path") == FACADE_FAULT_SOURCE
            and manifest.get("source_sha256") == FACADE_FAULT_SOURCE_SHA256
            and manifest.get("candidate_sha256") == ALLOCATION.canonical_digest(inputs)
            and type(manifest.get("source_file_count")) is int and manifest["source_file_count"] == len(inputs)
            and manifest.get("extra_inputs_sha256") == extras
            and manifest.get("raw_outcomes_sha256") == FACADE_FAULT_OUTCOMES_SHA256
            and type(manifest.get("faults")) is list and len(manifest["faults"]) == 4,
            "Facade preparation does not bind the current product and helpers.")
    records = manifest["faults"]
    require(all(type(row) is dict and set(row) == {"id", "mutant", "raw_diff_path", "source_sha256",
                                                  "diff_sha256", "source_path", "diff_path"} for row in records),
            "Facade manifest has altered overlay fields.")
    original = read(ROOT / FACADE_FAULT_SOURCE, maximum=256 * 1024)
    regenerated = facade_fault_sources(original, {"outcomes": [
        {"summary": "MissedMutant", "scenario": {"Mutant": row["mutant"]}} for row in records]})
    custody = {path: hashlib.sha256(raw).hexdigest(), manifest_path: digest}
    selected = None
    for fault, row in zip(regenerated, records, strict=True):
        expected = {key: fault[key] for key in ("id", "mutant", "raw_diff_path", "source_sha256", "diff_sha256")}
        expected.update(source_path=fault["id"] + "/mod.rs", diff_path=fault["id"] + "/mutation.diff")
        require(ALLOCATION.canonical_digest(row) == ALLOCATION.canonical_digest(expected),
                "Prepared facade descriptor, paths or hashes changed.")
        for field in ("source", "diff"):
            member = path.parent / expected[field + "_path"]
            data = read(member, maximum=256 * 1024)
            require(data == fault[field], "Prepared facade overlay bytes changed.")
            custody[member] = hashlib.sha256(data).hexdigest()
        if fault["id"] == fault_id:
            selected = fault
    require(selected is not None, "Selected facade fault is absent.")
    return {key: selected[key] for key in ("id", "mutant", "source", "source_sha256", "diff_sha256")} | {
        "receipt_sha256": hashlib.sha256(raw).hexdigest(), "manifest_sha256": digest, "custody_inputs": custody}


def validate_healthy_facade_build(record: dict, expected: dict) -> None:
    """Require an ordinary successful current facade build in the same tool environment."""
    require(type(record) is dict and type(record.get("schema_version")) is int and record["schema_version"] == 1
            and all(type(key) is str for key in record)
            and record.get("lane") == "build" and record.get("status") == "passed"
            and record.get("facade_consumer") is True and record.get("bookends_passed") is True
            and record.get("native_windows_execution") is False
            and not any(key.startswith(("fault", "facade_fault", "variant_", "prepared_faults", "healthy_build"))
                        for key in record),
            "Healthy parent must be an ordinary successful facade build, without fault metadata.")
    input_labels = {"product", "helpers", "copied_source", "control", "vendor", "tools", "stable_payload",
                    "windows_libraries", "sdk_records"}
    require(all(record.get(key) is True for key in ("input_bookends_passed", "normal_library_bookends",
                                                    "exported_artifact_bookends"))
            and record.get("custody_errors") == []
            and type(record.get("input_bookends")) is dict and set(record["input_bookends"]) == input_labels
            and all(value is True for value in record["input_bookends"].values()),
            "Healthy facade parent has incomplete or contradictory detailed custody.")
    require(type(expected) is dict and set(expected) == set(FACADE_HEALTHY_FIELDS),
            "Healthy facade comparison requires the complete current environment.")
    def path_value(value):
        if isinstance(value, Path):
            return str(value)
        raise TypeError("Unsupported healthy environment value.")
    try:
        same = all(key in record and json.dumps(record[key], sort_keys=True, separators=(",", ":"), allow_nan=False)
                   == json.dumps(value, sort_keys=True, separators=(",", ":"), allow_nan=False, default=path_value)
                   for key, value in expected.items())
    except (TypeError, ValueError, RecursionError) as error:
        raise Incomplete("Healthy facade environment is not an exact JSON/Path identity.") from error
    require(same,
            "Healthy facade build differs from current product, helpers or tool environment.")


def read_healthy_facade_build(receipt_path: Path, expected: dict) -> dict:
    """Bind the healthy receipt and all four exported PE/PDB pairs physically."""
    read = PE.Q.C.read_analysis_bytes
    path = receipt_path.absolute()
    raw = read(path, maximum=ALLOCATION.MAX_JSON)
    record = ALLOCATION.decode(raw)
    validate_healthy_facade_build(record, expected)
    names = {"gated-launcher.exe", "cpu-v3-probe.exe", "gate-control.exe", "engine-facade-consumer.exe"}
    artifacts = record.get("artifacts")
    require(type(artifacts) is dict and set(artifacts) == names, "Healthy facade images are incomplete.")
    custody = {path: hashlib.sha256(raw).hexdigest()}
    for name in sorted(names):
        facts = artifacts[name]
        image = path.parent / name
        require(type(facts) is dict and facts.get("path") == str(image), "Healthy image path changed.")
        for member, key in ((image, "sha256"), (image.with_suffix(".pdb"), "pdb_sha256")):
            data = read(member, maximum=64 * 1024 * 1024)
            digest = hashlib.sha256(data).hexdigest()
            require(facts.get(key) == digest, "Healthy exported image or PDB bytes changed.")
            custody[member] = digest
    return {"receipt": record, "receipt_sha256": hashlib.sha256(raw).hexdigest(), "custody_inputs": custody}


def validate_test_roster(value, candidate, extras, *, default_cohort=False, placement_cohort=False):
    """Admit the complete Windows library prediction and one closed selection."""
    keys = {"schema_version", "package", "target", "features", "candidate_sha256",
            "extra_inputs_sha256", "tests"}
    require(type(default_cohort) is bool, "Default roster selection must be Boolean.")
    require(type(placement_cohort) is bool and not (default_cohort and placement_cohort),
            "Placement roster selection must be Boolean and exclude the default cohort.")
    if placement_cohort:
        keys.add("selected_placement_tests")
    elif default_cohort:
        keys.add("selected_default_tests")
    else:
        keys.update(("ke_tests", "selected_ke_tests", "held_ke_tests", "manual_native_tests"))
    require(type(value) is dict and set(value) == keys
            and type(value["schema_version"]) is int and value["schema_version"] == 1
            and value["package"] == "nevrotick-engine" and value["target"] == TARGET
            and value["features"] == ([] if default_cohort or placement_cohort else ["ke-association-prototype"])
            and value["candidate_sha256"] == candidate
            and type(value["extra_inputs_sha256"]) is dict
            and value["extra_inputs_sha256"] == extras,
            "Windows test roster does not bind the exact source, helpers, target and feature.")
    names = value["tests"]
    require(type(names) is list and 1 <= len(names) <= 4096
            and all(type(name) is str and 0 < len(name) <= 4096
                    and re.fullmatch(r"[A-Za-z_][A-Za-z0-9_]*(?:::[A-Za-z_][A-Za-z0-9_]*)*", name)
                    for name in names)
            and names == sorted(set(names)),
            "Windows test roster must contain bounded exact sorted unique library identities.")
    if placement_cohort:
        expected = sorted(PLACEMENT_COHORT_CASES.values())
        require(not any(name.startswith("nts_ke_experiment::") for name in names)
                and value["selected_placement_tests"] == expected
                and all(name in names for name in expected),
                "Placement cohort requires both fixed writer cases and no KE feature.")
        return value
    if default_cohort:
        expected = sorted(DEFAULT_COHORT_CASES.values())
        require(not any(name.startswith("nts_ke_experiment::") for name in names)
                and value["selected_default_tests"] == expected
                and all(name in names for name in expected),
                "Default cohort requires every fixed reply-guard case and no KE feature.")
        return value
    expected_ke = [name for name in names if name.startswith("nts_ke_experiment::")]
    require(type(value["ke_tests"]) is list and expected_ke
            and value["ke_tests"] == expected_ke,
            "Windows KE roster must be the complete exact selected library subset.")
    nested, manual = set(HELD_KE_TESTS), set(MANUAL_NATIVE_TESTS)
    require(len(HELD_KE_TESTS) == len(nested) == 3
            and len(MANUAL_NATIVE_TESTS) == len(manual) == 5
            and nested.isdisjoint(manual)
            and (nested | manual).issubset(expected_ke)
            and value["held_ke_tests"] == list(HELD_KE_TESTS)
            and value["manual_native_tests"] == list(MANUAL_NATIVE_TESTS)
            and value["selected_ke_tests"] == [name for name in expected_ke if name not in nested | manual]
            and value["selected_ke_tests"],
            "Windows KE selection must retain its exact three nested and five separate manual native cases.")
    return value


def validate_ke_launcher_selection(source):
    """Check bounded fixed source-vector consistency; this is not a Rust parser."""
    require(type(source) is str and len(source.encode("utf-8")) <= MAX_LAUNCHER_SOURCE_BYTES,
            "Fixed KE launcher source exceeds its static consistency bound.")
    before, after = "            Self::Ke => (", "            Self::ControlHealthy =>"
    require(source.count(before) == source.count(after) == 1,
            "Fixed KE launcher selection anchors changed.")
    body = source.split(before, 1)[1].split(after, 1)[0]
    expected = ["engine-tests.exe", "nts_ke_experiment::", "--test-threads=1"]
    for name in (*HELD_KE_TESTS, *MANUAL_NATIVE_TESTS):
        expected.extend(("--skip", name))
    require(re.findall(r'"([^"\n]*)"', body) == expected,
            "Fixed KE launcher arguments differ from the exact scripted/manual partition.")


def read_test_roster(path, candidate, extras, *, default_cohort=False, placement_cohort=False):
    """Read one bounded nonlinked manifest once; discovery cannot supply its oracle."""
    raw = PE.Q.C.read_analysis_bytes(path, maximum=MAX_TEST_ROSTER_BYTES)
    return validate_test_roster(ALLOCATION.decode(raw), candidate, extras,
                                default_cohort=default_cohort, placement_cohort=placement_cohort), raw


def validate_test_listing(output, roster):
    """Require the entire actual library listing to equal the independent prediction."""
    require(type(output) is str, "Native library listing must be text.")
    lines = [line for line in output.splitlines() if line]
    names = [line.removesuffix(": test") for line in lines if line.endswith(": test")]
    summaries = [line for line in lines if re.fullmatch(r"[0-9]+ tests?, 0 benchmarks", line)]
    require(sorted(names) == roster["tests"] and len(summaries) <= 1
            and (not summaries or int(summaries[0].split()[0]) == len(roster["tests"]))
            and len(lines) == len(names) + len(summaries),
            "Windows library discovery differs from its exact independent source-bound roster.")
    return names


def validate_ke_execution(output, roster):
    """Require exact selected successes, zero skips and the complete filtered count."""
    require(type(output) is str, "Native KE results must be text.")
    expected = roster["selected_ke_tests"]
    filtered = len(roster["tests"]) - len(expected)
    output = "\n".join(output.splitlines())
    summaries = re.findall(r"^test result: ok\. ([0-9]+) passed; 0 failed; 0 ignored; 0 measured; ([0-9]+) filtered out;(?: .*)?$", output, re.M)
    entries = re.findall(r"^test (\S+) \.\.\. (.+)$", output, re.M)
    require(summaries == [(str(len(expected)), str(filtered))]
            and sorted(name for name, status in entries if status == "ok") == expected
            and len(entries) == len(expected),
            "Native KE results do not equal the independent selected identities and counts.")


def validate_nested_execution(output, roster, mode):
    """Require one exact outer test; its checked parent alone attests inner exit 86."""
    require(type(mode) is str and mode in NESTED_TEST_MODES, "Unadmitted exact nested test mode.")
    name = NESTED_TEST_MODES[mode]
    require(name in roster["held_ke_tests"] and name in roster["tests"], "Nested test is outside its frozen roster.")
    validate_ke_execution(output, {**roster, "selected_ke_tests": [name]})
    return {"test": name, "tests_executed": 1, "filtered_tests": len(roster["tests"]) - 1,
            "inner_completion_scope": "Actual test parent requires isolated child exit 86; not an external exit-code observation."}


def validate_default_execution(output, roster, mode, *, placement_cohort=False):
    """Require one exact code-owned plain library success and all other tests filtered."""
    require(type(placement_cohort) is bool, "Placement cohort selection must be Boolean.")
    if placement_cohort:
        require(type(mode) is str and mode in PLACEMENT_COHORT_CASES,
                "Unadmitted exact placement test mode.")
        name = PLACEMENT_COHORT_CASES[mode]
        require(name in roster["selected_placement_tests"] and name in roster["tests"],
                "Placement test is outside its frozen independent roster.")
    else:
        require(type(mode) is str and mode in DEFAULT_COHORT_CASES,
                "Unadmitted exact default test mode.")
        name = DEFAULT_COHORT_CASES[mode]
        require(name in roster["selected_default_tests"] and name in roster["tests"],
                "Default test is outside its frozen independent roster.")
    validate_ke_execution(output, {"tests": roster["tests"], "selected_ke_tests": [name]})
    return name


def native_directory(path):
    """Admit one fresh lexical/canonical Windows-owned child without creating it."""
    directory = path.absolute()
    prefix = Path("/mnt/c/Users/perso/AppData/Local/NevroTick")
    require(".." not in directory.parts and directory.is_relative_to(prefix)
            and directory != prefix and not directory.exists(),
            "Native directory must be a fresh Windows NevroTick child.")
    ancestor = directory.parent
    while ancestor != Path("/"):
        require(not ancestor.is_symlink(), "Native directory has a redirected ancestor.")
        ancestor = ancestor.parent
    require(directory.resolve().is_relative_to(prefix.resolve()),
            "Native directory escapes its owned root.")
    return directory


def stable_identity(admitted):
    """Select immutable complete-payload identities, excluding live module objects."""
    return {key: admitted[key] for key in ("stable", "inventories", "readonly_roots",
                                          "policy_sha256", "installation_pointer_sha256")}


def engine_artifact(output, work, *, features=("ke-association-prototype",)):
    """Require exactly one Cargo-attributed Windows Engine libtest executable."""
    require(type(features) is tuple and features in ((), ("ke-association-prototype",)),
            "Unadmitted Engine libtest feature set.")
    found = []
    for line in output.splitlines():
        if not line.startswith("{"):
            continue
        item = ALLOCATION.decode(line.encode())
        if (item.get("reason") == "compiler-artifact"
                and item.get("target", {}).get("name") == "nevrotick_engine"
                and item.get("target", {}).get("kind") == ["lib"]
                and item.get("profile", {}).get("test") is True
                and item.get("executable") is not None):
            require(item.get("package_id") == "path+file:///work/fixture/crates/nevrotick-engine#0.0.0"
                    and item.get("manifest_path") == "/work/fixture/crates/nevrotick-engine/Cargo.toml"
                    and item.get("target", {}).get("crate_types") == ["lib"]
                    and item.get("profile", {}).get("opt_level") == "0"
                    and item.get("profile", {}).get("debug_assertions") is True
                    and item.get("profile", {}).get("overflow_checks") is True
                    and item.get("features") == list(features),
                    "Engine libtest artifact belongs to an unexpected source package.")
            value = item["executable"]
            require(type(value) is str and re.fullmatch(
                    re.escape(f"/work/target/{TARGET}/debug/")
                    + r"(?:deps/|build/nevrotick-engine/[a-f0-9]{16}/out/)"
                    + r"nevrotick_engine(?:-[a-f0-9]{16})?\.exe", value)
                    and type(item.get("filenames")) is list
                    and all(type(path) is str for path in item["filenames"])
                    and [path for path in item["filenames"] if path.endswith(".exe")] == [value],
                    "Unexpected Engine libtest output path.")
            found.append(work / value.removeprefix("/work/"))
    require(len(found) == 1 and found[0].is_file() and found[0].resolve(strict=True) == found[0],
            "Expected one actual emitted Engine libtest executable.")
    return found[0]


def engine_libraries(output, work, source, *, vendor, evidence=None):
    """Attribute normal Windows libraries and their bounded Cargo search directories.

    The caller has admitted/frozen the complete graph. This reader additionally
    binds every selected artifact to that lock and manifest, never a file glob.
    """
    require(type(output) is str and len(output) <= 16 * 1024**2,
            "Normal library output exceeds its finite text bound.")
    require(vendor.is_dir() and not vendor.is_symlink() and vendor.resolve() == vendor,
            "Normal library vendor must be the canonical admitted directory.")
    locked = tomllib.loads((source / "Cargo.lock").read_text())["package"]
    registry = "registry+https://github.com/rust-lang/crates.io-index"
    owners = {(p["name"], p["version"], p.get("source")) for p in locked}
    require(len(owners) == len(locked), "Duplicate locked normal-library owner.")
    engine_manifest = tomllib.loads((source / "crates/nevrotick-engine/Cargo.toml").read_text())
    requirement = engine_manifest["dependencies"]["tokio"]["version"]
    require(type(requirement) is str and re.fullmatch(r"=[0-9]+\.[0-9]+\.[0-9]+", requirement),
            "Consumer Tokio must retain the Engine's exact admitted version.")
    libraries, directories, paths, units = {}, set(), set(), set()
    for line in output.splitlines():
        if not line.startswith("{"):
            continue
        item = ALLOCATION.decode(line.encode())
        require(type(item) is dict, "Normal Cargo event must be an object.")
        if item.get("reason") != "compiler-artifact":
            continue
        target, profile = item.get("target", {}), item.get("profile", {})
        require(type(target) is dict and type(profile) is dict, "Malformed normal Cargo target/profile.")
        kind = target.get("kind")
        if kind == ["custom-build"]:
            continue
        require(kind in (["lib"], ["proc-macro"]) and target.get("crate_types") == kind
                and profile.get("test") is False and item.get("executable") is None,
                "Expected normal library/proc-macro artifacts, not libtests or executables.")
        package, manifest = item.get("package_id"), item.get("manifest_path")
        require(type(package) is str and type(manifest) is str, "Missing normal-library owner.")
        match = re.fullmatch(re.escape(registry) + r"#([A-Za-z0-9_-]+)@([^\s/@]+)", package)
        if match:
            name, version = match.groups()
            require((name, version, registry) in owners
                    and manifest == f"/vendor/{name}-{version}/Cargo.toml",
                    "Normal registry library differs from the locked manifest.")
            declared_path = vendor / f"{name}-{version}" / "Cargo.toml"
            require(declared_path.is_file() and not declared_path.is_symlink()
                    and declared_path.resolve() == declared_path,
                    "Normal registry manifest is absent or redirected.")
            declared = tomllib.loads(declared_path.read_text())
            require(declared["package"]["name"] == name and declared["package"]["version"] == version,
                    "Normal registry manifest has a different locked owner.")
            crate = declared.get("lib", {}).get("name", name.replace("-", "_"))
        else:
            require(manifest.startswith("/work/fixture/") and manifest.endswith("/Cargo.toml"),
                    "Normal local library has a foreign manifest.")
            relative = manifest.removeprefix("/work/fixture/")
            require(".." not in Path(relative).parts, "Normal local manifest escapes its source.")
            local = source / relative
            require(local.is_file() and not local.is_symlink() and local.resolve() == local,
                    "Normal local manifest is absent or redirected.")
            declared = tomllib.loads(local.read_text())
            name, version = declared["package"]["name"], declared["package"]["version"]
            fragment = version if Path(manifest).parent.name == name else name + "@" + version
            require((name, version, None) in owners
                    and package == "path+file://" + manifest.removesuffix("/Cargo.toml") + "#" + fragment,
                    "Normal local library differs from the locked source owner.")
            crate = declared.get("lib", {}).get("name", name.replace("-", "_"))
        macro = declared.get("lib", {}).get("proc-macro", False)
        require(type(macro) is bool and macro is (kind == ["proc-macro"]),
                "Normal library/proc-macro kind differs from its declared manifest.")
        require(target.get("name") == crate, "Normal library crate name differs from its owner.")
        files = item.get("filenames")
        require(type(files) is list and 0 < len(files) <= 2
                and all(type(p) is str for p in files) and len(set(files)) == len(files),
                "Normal library filenames are missing, duplicated or unbounded.")
        host = all(p.startswith("/work/target/debug/") for p in files)
        prefix = "/work/target/debug/" if host else f"/work/target/{TARGET}/debug/"
        require(kind != ["proc-macro"] or host, "Proc macro must belong to the admitted host compiler.")
        unit = (package, crate, host)
        require(unit not in units, "Duplicate normal library compilation unit.")
        units.add(unit)
        extensions = {Path(p).suffix for p in files}
        require(extensions == {".so"} if kind == ["proc-macro"] else
                ".rlib" in extensions and extensions <= {".rlib", ".rmeta"},
                "Normal library filenames have the wrong artifact kinds.")
        selected = []
        for emitted in files:
            top = name == "nevrotick-engine" and emitted == prefix + "libnevrotick_engine.rlib"
            require(top or re.fullmatch(re.escape(prefix) + r"(?:deps/|build/" + re.escape(name)
                    + r"/[a-f0-9]{16}/out/)lib" + re.escape(crate)
                    + r"-[a-f0-9]{16}\.(?:rlib|rmeta|so)", emitted),
                    "Normal library path differs from its exact host/target owner.")
            local = work / emitted.removeprefix("/work/")
            require(emitted not in paths and local.is_file() and not local.is_symlink()
                    and local.resolve() == local and local.stat().st_size <= 256 * 1024**2,
                    "Normal library is duplicated, absent, redirected or oversized.")
            paths.add(emitted); directories.add(local.parent)
            if evidence is not None:
                evidence[emitted] = {"sha256": PRODUCT.sha256(local), "package_id": package,
                                     "manifest_path": manifest, "host": host}
            if local.suffix == ".rlib":
                selected.append(local)
        if name in ("nevrotick-engine", "tokio"):
            require(type(item.get("features")) is list
                    and all(type(f) is str for f in item["features"]), "Invalid normal library features.")
            require(not host and kind == ["lib"] and len(selected) == 1,
                    "Consumer requires one normal Windows library for each actual owner.")
            require(profile.get("opt_level") == "0" and profile.get("overflow_checks") is True
                    and profile.get("debug_assertions") is True,
                    "Consumer requires the admitted checked debug library profile.")
            if name == "nevrotick-engine":
                require(manifest == "/work/fixture/crates/nevrotick-engine/Cargo.toml"
                        and item.get("features") == ["ke-association-prototype"],
                        "Consumer Engine lacks the exact production KE feature.")
            else:
                require(version == requirement[1:] and "rt" in item["features"],
                        "Consumer Tokio differs from the Engine version/runtime requirement.")
            require(crate not in libraries, "Ambiguous selected normal library.")
            libraries[crate] = selected[0]
    require(set(libraries) == {"nevrotick_engine", "tokio"}, "Missing normal Engine or Tokio library.")
    return libraries, sorted(directories)


def build(args, report):
    """Freeze exact inputs, build offline, inspect emitted PE declarations, and re-admit."""
    facade = getattr(args, "facade_consumer", False)
    require(type(facade) is bool, "Facade profile must be an explicit Boolean.")
    default = default_cohort_requested(args)
    placement = placement_cohort_requested(args)
    if not (facade or default or placement):
        validate_ke_launcher_selection(PE.Q.C.read_analysis_bytes(
            ROOT / LAUNCHER, maximum=MAX_LAUNCHER_SOURCE_BYTES).decode("utf-8"))
    test_features = () if default or placement else ("ke-association-prototype",)
    fault_id = getattr(args, "facade_fault", None)
    fault_flags = (fault_id, getattr(args, "prepared_faults", None), getattr(args, "healthy_build_receipt", None))
    require(not any(value is not None for value in fault_flags)
            or (facade and all(value is not None for value in fault_flags)
                and type(fault_id) is str and fault_id in {row[0] for row in FACADE_FAULTS}),
            "Fault builds require one fixed ID, prepared inputs and a healthy facade parent.")
    require(all(getattr(args, name) is not None for name in
                ("vendor_dir", "toolchain_dir", "xwin_config", "xwin_inventory"))
            and (facade or args.test_roster is not None),
            "Build requires explicit vendor, toolchain, xwin and independent test-roster paths.")
    require(shutil.disk_usage(args.output_dir).free >= 4 * 1024 ** 3,
            "Build requires four GiB of free output space.")
    require(type(args.jobs) is int and 1 <= args.jobs <= 12, "Build jobs must be within 1–12.")
    policy, inputs = PRODUCT.admit_product(ROOT)
    extras = {name: PRODUCT.sha256(ROOT / name) for name in EXTRAS}
    roster_raw = None
    if not facade:
        _, roster_raw = read_test_roster(args.test_roster.absolute(), ALLOCATION.canonical_digest(inputs), extras,
                                         default_cohort=default, placement_cohort=placement)
        roster_path = args.output_dir / TEST_ROSTER_NAME
        roster_path.write_bytes(roster_raw)
    vendor, toolchain = args.vendor_dir.resolve(strict=True), args.toolchain_dir.resolve(strict=True)
    vendor_before = PRODUCT.admit_vendor(vendor, policy)
    tools_before = PRODUCT.admit_tools(ROOT, toolchain, None, "windows")
    admission = ANALYSIS.admit(ROOT, profile="stable-core")
    require(admission["stable"] == toolchain, "Selected toolchain differs from admitted stable payload.")
    stable_before = stable_identity(admission)
    sdk_before = {str(path): PRODUCT.sha256(path) for path in (args.xwin_config, args.xwin_inventory)}
    report.update(product_source_sha256=inputs, extra_inputs_sha256=extras,
                  candidate_sha256=ALLOCATION.canonical_digest(inputs), tools_sha256=tools_before,
                  stable_payload=stable_before, toolchain=str(toolchain), vendor=str(vendor),
                  xwin_config=str(args.xwin_config.resolve(strict=True)),
                  xwin_inventory=str(args.xwin_inventory.resolve(strict=True)), profile="debug",
                  diagnostic_profile="release", build_jobs=args.jobs, lanes=[],
                  facade_consumer=facade, default_cohort=default,
                  test_features=list(test_features))
    if placement:
        report["placement_cohort"] = True
    if roster_raw is not None:
        report["test_roster"] = {"path": str(roster_path), "sha256": hashlib.sha256(roster_raw).hexdigest()}
    source, work = args.output_dir / "source", args.output_dir / "work"
    source.mkdir(mode=0o700); work.mkdir(mode=0o700)
    (work / "fixture").mkdir(mode=0o700)
    (work / "cargo-home").mkdir(mode=0o700)
    for name, checksum in {**inputs, **extras}.items():
        destination = source / name
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / name, destination)
        require(PRODUCT.sha256(destination) == checksum, "Source changed while freezing.")
    control = source / "gate-control.rs"
    control.write_text(CONTROL_SOURCE)
    report["control_source_sha256"] = PRODUCT.sha256(control)
    libraries, windows_library_inputs = PRODUCT.windows_libraries(
        ROOT, toolchain, args.xwin_config, work, args.xwin_inventory)
    report["windows_inputs"] = {"records_sha256": sdk_before, "library_files": len(windows_library_inputs),
                                "library_map_sha256": ALLOCATION.canonical_digest(windows_library_inputs)}
    (work / "cargo-home/config.toml").write_text(
        '[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory="/vendor"\n')

    source_expected = {**inputs, **extras}
    prepared_fault = healthy_build = None
    fault_inputs = {}
    normal_library_inputs = {}
    normal_attributed = False
    build_completed = False
    primary_error = None
    report.update(input_bookends_passed=False, normal_library_bookends="not-reached" if facade else "not-applicable",
                  bookends_passed=False, native_windows_execution=False)

    def run(label, command):
        """Retain the first raw command outcome before classifying failure."""
        result = ENV.run(PRODUCT.sandbox(toolchain, work, source, vendor, command, None, libraries),
                         cwd=ROOT, timeout=300,
                         environment={"PATH": "/usr/bin:/bin", "NEVROTICK_SYNTHETIC_CREDENTIAL": "synthetic-control"})
        row = dict(result, lane=label, selected_command=command)
        raw_path = args.output_dir / (label + ".json")
        raw_path.write_text(json.dumps(row, indent=2) + "\n")
        report["lanes"].append({"lane": label, "selected_command": command,
                                "exit_code": result["exit_code"], "failure": result["failure"],
                                "raw_path": str(raw_path), "raw_sha256": PRODUCT.sha256(raw_path)})
        return ENV.require_ok(result)

    if fault_id is not None:
        report["facade_fault"] = fault_id
    try:
        if fault_id is not None:
            prepared_fault = read_prepared_facade_fault(args.prepared_faults, fault_id, inputs, extras)
            expected_parent = {key: report[key] for key in (
                "product_source_sha256", "extra_inputs_sha256", "candidate_sha256", "tools_sha256", "stable_payload",
                "toolchain", "vendor", "xwin_config", "xwin_inventory", "windows_inputs", "profile", "diagnostic_profile")}
            expected_parent.update(product_target_flags=ENV.MSVC_FLAGS, facade_target_flags=ENV.MSVC_FLAGS)
            diagnostic_flags = list(ENV.MSVC_FLAGS)
            diagnostic_flags[diagnostic_flags.index("target-cpu=x86-64-v3")] = "target-cpu=x86-64"
            expected_parent["diagnostic_target_flags"] = diagnostic_flags
            fault_inputs.update(prepared_fault["custody_inputs"])
            healthy_build = read_healthy_facade_build(args.healthy_build_receipt, expected_parent)
            fault_inputs.update(healthy_build["custody_inputs"])
            report.update(facade_fault=fault_id, prepared_faults_sha256=prepared_fault["receipt_sha256"],
                          prepared_manifest_sha256=prepared_fault["manifest_sha256"],
                          healthy_build_receipt_sha256=healthy_build["receipt_sha256"],
                          variant_source_sha256={**inputs, FACADE_FAULT_SOURCE: prepared_fault["source_sha256"]})
            report["variant_candidate_sha256"] = ALLOCATION.canonical_digest(report["variant_source_sha256"])
        PRODUCT.boundary_control(run)
        run("runner-controls", ["python3", "-X", "dev", "-Werror", "-B", "-m", "unittest", "discover",
                                "-s", "/work/fixture/scripts/tests", "-p", "test_engine_native.py", "-v"])
        version = run("compiler-version", ["rustc", "-vV"])
        ENV.require_admitted_compiler(policy=json.loads((ROOT / "policy/tool-manifest.json").read_text()),
                                      channel=tomllib.loads((source / "rust-toolchain.toml").read_text())["toolchain"]["channel"],
                                      compiler_info=version)
        cargo = engine_test_command(args.jobs, default_cohort=not test_features)
        workspace = tomllib.loads((source / "Cargo.toml").read_text())
        flags = list(ENV.MSVC_FLAGS)
        flags[flags.index("target-cpu=x86-64-v3")] = "target-cpu=x86-64"
        report["product_target_flags"] = ENV.MSVC_FLAGS
        report["diagnostic_target_flags"] = flags
        direct = ["--edition=2024", "--target", TARGET, "-C", "linker=/work/linker",
                  *flags, *ALLOCATION.profile_flags("release"), *ALLOCATION.lint_flags(workspace)]
        images = {}
        for name, relative in (("gated-launcher", LAUNCHER), ("cpu-v3-probe", CPU), ("gate-control", "gate-control.rs")):
            source_arg = "/work/fixture/" + relative
            run(name + "-format", ["rustfmt", "--edition", "2024", "--check", source_arg])
            run(name + "-clippy", ["clippy-driver", *direct, "--crate-name", name.replace("-", "_"),
                                  "--emit=metadata", "-o", "/work/" + name + ".rmeta", source_arg])
            image = work / (name + ".exe")
            run(name + "-link", ["rustc", *direct, "--crate-name", name.replace("-", "_"),
                                "-o", "/work/" + image.name, source_arg])
            images[name] = image
        if prepared_fault is not None:
            (source / FACADE_FAULT_SOURCE).write_bytes(prepared_fault["source"])
            source_expected[FACADE_FAULT_SOURCE] = prepared_fault["source_sha256"]
            require(PRODUCT.sha256(source / FACADE_FAULT_SOURCE) == prepared_fault["source_sha256"],
                    "Copied facade overlay differs before Engine compilation.")
        # Qualify small diagnostic consumers before the substantial Engine link.
        if facade:
            normal_cargo = [*cargo]
            normal_cargo[normal_cargo.index("test")] = "build"
            normal_cargo.remove("--no-run")
            normal_output = run("engine-library-link", normal_cargo)
            report["normal_library_inputs"] = normal_library_inputs
            linked, search = engine_libraries(normal_output, work, source, vendor=vendor, evidence=normal_library_inputs)
            normal_attributed = True
            report["normal_libraries"] = {name: {"path": str(path), "sha256": PRODUCT.sha256(path)}
                                          for name, path in linked.items()}
            dependency_flags = []
            for directory in search:
                dependency_flags += ["-L", "dependency=/work/" + str(directory.relative_to(work))]
            for name, path in linked.items():
                dependency_flags += ["--extern", name + "=/work/" + str(path.relative_to(work))]
            consumer_flags = ["--edition=2024", "--target", TARGET, "-C", "linker=/work/linker",
                              *ENV.MSVC_FLAGS, *ALLOCATION.profile_flags("debug"),
                              *ALLOCATION.lint_flags(workspace), *dependency_flags]
            consumer_source = "/work/fixture/" + FACADE_CONSUMER
            run("facade-format", ["rustfmt", "--edition", "2024", "--check", consumer_source])
            run("facade-clippy", ["clippy-driver", *consumer_flags, "--crate-name", "engine_facade_consumer",
                                 "--emit=metadata", "-o", "/work/facade.rmeta", consumer_source])
            run("facade-link", ["rustc", *consumer_flags, "--crate-name", "engine_facade_consumer",
                               "-o", "/work/engine-facade-consumer.exe", consumer_source])
            images["engine-facade-consumer"] = work / "engine-facade-consumer.exe"
            report["facade_target_flags"] = ENV.MSVC_FLAGS
        else:
            images["engine-tests"] = engine_artifact(run("engine-libtest-link", cargo), work,
                                                       features=test_features)
        report["artifacts"] = {}
        inspector = "/toolchain/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-readobj"
        for name, image in images.items():
            raw = PE.pe_readback(image)
            run(name + "-pe", [inspector, "--file-headers", "--coff-load-config", "--coff-debug-directory",
                               "--coff-imports", "/work/" + str(image.relative_to(work))])
            pdb = image.with_suffix(".pdb")
            require(pdb.is_file() and not pdb.is_symlink(), "Matching emitted PDB is absent.")
            for original, destination in ((image, args.output_dir / (name + ".exe")),
                                          (pdb, args.output_dir / (name + ".pdb"))):
                shutil.copyfile(original, destination)
                require(PRODUCT.sha256(destination) == PRODUCT.sha256(original), "Export changed emitted bytes.")
            report["artifacts"][name + ".exe"] = {"path": str(args.output_dir / (name + ".exe")),
                                                 "sha256": PRODUCT.sha256(image), "pe": raw,
                                                 "pdb_sha256": PRODUCT.sha256(pdb)}
        build_completed = True
    except BaseException as error:
        primary_error = error
        report["status"] = "incomplete"
        raise
    finally:
        checks = [
            ("product", lambda: PRODUCT.admit_product(ROOT)[1] == inputs),
            ("helpers", lambda: all(PRODUCT.sha256(ROOT / name) == digest for name, digest in extras.items())),
            ("copied_source", lambda: all(PRODUCT.sha256(source / name) == digest for name, digest in source_expected.items())),
            ("control", lambda: PRODUCT.sha256(control) == report["control_source_sha256"]),
            ("vendor", lambda: PRODUCT.admit_vendor(vendor, policy) == vendor_before),
            ("tools", lambda: PRODUCT.admit_tools(ROOT, toolchain, None, "windows") == tools_before),
            ("stable_payload", lambda: stable_identity(ANALYSIS.admit(ROOT, profile="stable-core")) == stable_before),
            ("windows_libraries", lambda: PRODUCT.library_inventory(libraries) == windows_library_inputs),
            ("sdk_records", lambda: all(PRODUCT.sha256(Path(path)) == digest for path, digest in sdk_before.items())),
        ]
        if not facade:
            checks.extend([
                ("original_roster", lambda: read_test_roster(args.test_roster.absolute(), report["candidate_sha256"], extras,
                                                            default_cohort=default,
                                                            placement_cohort=placement)[1] == roster_raw),
                ("copied_roster", lambda: read_test_roster(roster_path, report["candidate_sha256"], extras,
                                                          default_cohort=default,
                                                          placement_cohort=placement)[1] == roster_raw),
            ])
        if prepared_fault is not None:
            checks.append(("fault_inputs", lambda: all(hashlib.sha256(PE.Q.C.read_analysis_bytes(
                path, maximum=64 * 1024 * 1024)).hexdigest() == digest for path, digest in fault_inputs.items())))
        facts, errors = {}, []
        for name, check in checks:
            try:
                facts[name] = check() is True
            except Exception:
                facts[name] = False
            if not facts[name]:
                errors.append(name)
        report["input_bookends"] = facts
        report["input_bookends_passed"] = all(facts.values())
        if facade and normal_attributed:
            try:
                require(bool(normal_library_inputs) and all(
                    PRODUCT.sha256(work / path.removeprefix("/work/")) == item["sha256"]
                    for path, item in normal_library_inputs.items()),
                    "Attributed normal-library bytes changed during consumer linking.")
                report["normal_library_bookends"] = True
            except Exception:
                report["normal_library_bookends"] = False
                errors.append("normal_libraries")
        exported = report.get("artifacts")
        report["exported_artifact_bookends"] = "not-reached"
        if exported:
            try:
                require(all(PRODUCT.sha256(Path(item["path"])) == item["sha256"]
                            and PRODUCT.sha256(Path(item["path"]).with_suffix(".pdb")) == item["pdb_sha256"]
                            for item in exported.values()), "Exported image or PDB bytes changed.")
                report["exported_artifact_bookends"] = True
            except Exception:
                report["exported_artifact_bookends"] = False
                errors.append("exported_artifacts")
        report["custody_errors"] = errors
        report["bookends_passed"] = (build_completed and report["input_bookends_passed"]
                                      and (not facade or report["normal_library_bookends"] is True)
                                      and report["exported_artifact_bookends"] is True)
        if not report["bookends_passed"]:
            report["status"] = "incomplete"
            if primary_error is None:
                raise Incomplete("Build custody incomplete: " + ", ".join(errors or ["unfinished artifacts"]))
    report["status"] = "fault-build-completed" if fault_id is not None else "passed"


NATIVE_JOB_OBSERVER = r"""
    // Closed per-mode hypothesis: three authored processes and at most three
    // OS console helpers. Existing flat modes retain their original four cap.
    static uint ProcessLimit(string mode) {
        switch (mode) {
            case "job-native-foreign": case "provider-native-once": case "provider-native-foreign":
            case "control-nested-healthy": case "control-nested-fail": case "control-nested-wait": return 6;
            case "cpu": case "list": case "ke":
            case "default-pilot-stop":
            case "default-pilot-invalidate":
            case "default-pilot-healthy":
            case "default-series-stop":
            case "default-series-invalidate":
            case "default-series-healthy":
            case "default-writable-stop":
            case "default-writable-invalidate":
            case "default-policy-poison":
            case "default-policy-reenable":
            case "default-session-checkpoints":
            case "default-policy-receive-chronology":
            case "default-resume-register-close":
            case "placement-managed-writer":
            case "placement-nominal-writer":
            case "facade-arguments": case "facade-entered-runtime": case "facade-loopback-eof":
            case "facade-managed-protected-eof":
            case "control-healthy": case "control-fail": case "control-wait": return 4;
            default: throw new InvalidDataException();
        }
    }
    public class Member {
        public long pid; public string basename, image_role, read_failure_type; public bool system_console;
    }
    public sealed class Notice : Member { public uint kind; }
    [StructLayout(LayoutKind.Sequential)] struct PortAssociation { public IntPtr key, port; }
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool QueryInformationJobObject(IntPtr job, int kind, IntPtr buffer, uint length, out uint returned);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool IsProcessInJob(IntPtr process, IntPtr job, out bool contained);
    [DllImport("kernel32.dll", SetLastError=true)] static extern IntPtr OpenProcess(uint access, bool inherit, uint pid);
    [DllImport("kernel32.dll", CharSet=CharSet.Unicode, SetLastError=true)]
    static extern bool QueryFullProcessImageNameW(IntPtr process, uint flags, StringBuilder path, ref uint capacity);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern IntPtr CreateIoCompletionPort(IntPtr file, IntPtr existing, UIntPtr key, uint threads);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool SetInformationJobObject(IntPtr job, int kind, ref PortAssociation info, uint length);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool GetQueuedCompletionStatus(IntPtr port, out uint kind, out UIntPtr key, out IntPtr value, uint milliseconds);
    [DllImport("kernel32.dll", SetLastError=true)]
    static extern bool PostQueuedCompletionStatus(IntPtr port, uint kind, UIntPtr key, IntPtr value);
    // Kernel-reported Win32 path, bound to a currently owned-job process handle.
    // Raw paths are never emitted. File hashes are OS provenance, not mapped-image hashes.
    static Member ReadMember(IntPtr job, long pid, string launcher, string child, string console) {
        Member member = new Member {pid=pid}; IntPtr process = IntPtr.Zero;
        try {
            if (pid <= 0 || pid > UInt32.MaxValue) throw new InvalidDataException();
            process = OpenProcess(0x1000, false, (uint)pid); Check(process != IntPtr.Zero);
            bool contained; Check(IsProcessInJob(process, job, out contained));
            if (!contained) throw new InvalidOperationException();
            StringBuilder path = new StringBuilder(32768); uint capacity = 32768;
            Check(QueryFullProcessImageNameW(process, 0, path, ref capacity));
            if (capacity == 0 || capacity >= 32768) throw new InvalidDataException();
            string actual = path.ToString(); member.basename = Path.GetFileName(actual);
            if (String.IsNullOrEmpty(member.basename)) throw new InvalidDataException();
            member.system_console = String.Equals(actual, console, StringComparison.OrdinalIgnoreCase);
            member.image_role = String.Equals(actual, launcher, StringComparison.OrdinalIgnoreCase) ? "launcher" :
                String.Equals(actual, child, StringComparison.OrdinalIgnoreCase) ? "child" :
                member.system_console ? "system-console" : "unknown";
        } catch (Exception exception) { member.read_failure_type = exception.GetType().Name; }
        finally {
            if (process != IntPtr.Zero && !CloseHandle(process)) member.read_failure_type = "Win32Exception";
        }
        return member;
    }
    // One bounded snapshot; counters and membership are separate observations.
    static void ObserveBeforeGate(IntPtr job, Result result, string launcher, string child, string console) {
        IntPtr buffer = IntPtr.Zero;
        try {
            Accounting counts;
            Check(QueryInformationJobObject(job, 1, out counts, (uint)Marshal.SizeOf(typeof(Accounting)), IntPtr.Zero));
            result.total_processes_before_gate = counts.totalProcesses;
            result.active_processes_before_gate = counts.activeProcesses;
            if (IntPtr.Size != 8) throw new InvalidOperationException();
            const int bytes = 8 + 8 * 8; buffer = Marshal.AllocHGlobal(bytes); uint returned;
            Check(QueryInformationJobObject(job, 3, buffer, bytes, out returned));
            if (returned < 8 || returned > bytes) throw new InvalidDataException();
            int assigned = Marshal.ReadInt32(buffer, 0), listed = Marshal.ReadInt32(buffer, 4);
            result.assigned_before_gate = assigned; result.listed_before_gate = listed;
            if (assigned < 1 || assigned > 8 || listed != assigned || returned < 8 + listed * 8)
                throw new InvalidDataException();
            Member[] members = new Member[listed]; result.members_before_gate = members;
            for (int index = 0; index < listed; index++) {
                Member member = ReadMember(job, Marshal.ReadInt64(buffer, 8 + index * 8), launcher, child, console);
                members[index] = member;
                if (member.read_failure_type != null || (member.image_role != "launcher" && member.image_role != "system-console"))
                    throw new InvalidDataException();
            }
        } catch (Exception exception) { result.membership_read_failure_type = exception.GetType().Name; throw; }
        finally { if (buffer != IntPtr.Zero) Marshal.FreeHGlobal(buffer); }
    }
    // Best-effort notifications are supplemental identity evidence. Absence is
    // never inferred: Python requires every lifetime association to have one NEW.
    sealed class JobEvents {
        readonly IntPtr job; readonly string launcher, child, console; readonly int deadlineMilliseconds;
        IntPtr port; Task worker; volatile bool forceStop; int faulted;
        readonly System.Collections.Generic.List<Notice> notices = new System.Collections.Generic.List<Notice>(16);
        bool limitReached; string readFailure;
        public JobEvents(IntPtr ownedJob, string launcherPath, string childPath, string consolePath, string mode) {
            job=ownedJob; launcher=launcherPath; child=childPath; console=consolePath;
            ProcessLimit(mode);
            deadlineMilliseconds = mode == "ke" ? 75000 : 45000;
        }
        public int DeadlineMilliseconds { get { return deadlineMilliseconds; } }
        public void Start() {
            port = CreateIoCompletionPort(new IntPtr(-1), IntPtr.Zero, UIntPtr.Zero, 1); Check(port != IntPtr.Zero);
            PortAssociation association = new PortAssociation {key=new IntPtr(1), port=port};
            Check(SetInformationJobObject(job, 7, ref association, (uint)Marshal.SizeOf(typeof(PortAssociation))));
            worker = Task.Run(() => Read());
        }
        public void ThrowIfFaulted() {
            if (Volatile.Read(ref faulted) != 0) throw new InvalidOperationException("Owned process identity observation failed.");
        }
        void Read() {
            Stopwatch watch = Stopwatch.StartNew();
            try {
                while (!forceStop && watch.ElapsedMilliseconds < deadlineMilliseconds) {
                    uint kind; UIntPtr key; IntPtr value;
                    if (!GetQueuedCompletionStatus(port, out kind, out key, out value, 50)) {
                        int error = Marshal.GetLastWin32Error(); if (error == 258) continue;
                        throw new Win32Exception(error);
                    }
                    if (kind == 0 && key.ToUInt64() == 2 && value == IntPtr.Zero) return;
                    if (key.ToUInt64() != 1) throw new InvalidDataException();
                    Notice notice = new Notice {kind=kind};
                    if (kind == 6 || kind == 7 || kind == 8) notice.pid = value.ToInt64();
                    if (kind == 6) {
                        Member member = ReadMember(job, notice.pid, launcher, child, console);
                        notice.basename=member.basename; notice.image_role=member.image_role;
                        notice.system_console=member.system_console; notice.read_failure_type=member.read_failure_type;
                    }
                    notices.Add(notice);
                    // EXIT is a PID join to historical NEW identity, never a reopen.
                    if ((kind != 4 && kind != 6 && kind != 7 && kind != 8) ||
                        (kind == 6 && (notice.read_failure_type != null || notice.image_role == "unknown")))
                        throw new InvalidDataException();
                    if (notices.Count == 16) { limitReached = true; throw new InvalidDataException(); }
                }
                if (!forceStop) throw new TimeoutException();
            } catch (Exception exception) { readFailure=exception.GetType().Name; Interlocked.Exchange(ref faulted, 1); }
        }
        // Retire all query handles before the caller's final empty-job observation.
        public bool Stop(Result result) {
            try {
                if (worker != null && !worker.IsCompleted) {
                    if (!PostQueuedCompletionStatus(port, 0, new UIntPtr(2), IntPtr.Zero)) {
                        result.event_shutdown_failure_type="Win32Exception"; forceStop=true;
                    }
                    if (!worker.Wait(5000)) {
                        result.event_shutdown_failure_type="TimeoutException"; forceStop=true; return false;
                    }
                }
                result.event_reader_joined=true; result.job_events=notices.ToArray();
                result.event_limit_reached=limitReached; result.event_read_failure_type=readFailure;
                if (port != IntPtr.Zero) {
                    if (!CloseHandle(port)) { result.event_shutdown_failure_type="Win32Exception"; return false; }
                    port=IntPtr.Zero;
                }
                result.event_port_closed=true;
            } catch (Exception exception) { result.event_shutdown_failure_type=exception.GetType().Name; return false; }
            return result.event_shutdown_failure_type == null;
        }
    }
"""



def native_supervisor():
    """Extend the frozen gated observer for fixed children; fail on recipe drift."""
    source = ALLOCATION.NATIVE_SUPERVISOR

    def replace(before, after):
        """Require one exact upstream anchor; never silently alter another lane."""
        nonlocal source
        if source.count(before) != 1:
            raise Incomplete("The reused native-supervisor recipe changed.")
        source = source.replace(before, after)

    source = source.replace("NevroTickAllocationSupervisor", "NevroTickEngineSupervisor")
    replace("No AppVerifier/PageHeap configuration selected for requested-payload counting",
            "No AppVerifier/PageHeap configuration selected for scripted component tests")
    replace("public uint? active_processes_before_handle_release;",
            "public int child_deadline_milliseconds, event_deadline_milliseconds; "
            "public uint? process_limit, total_processes, total_processes_before_gate, active_processes_before_gate; "
            "public int? assigned_before_gate, listed_before_gate; public Member[] members_before_gate; "
            "public string membership_read_failure_type; public Notice[] job_events; "
            "public bool event_limit_reached, event_reader_joined, event_port_closed, event_job_handle_retained; "
            "public string event_read_failure_type, event_shutdown_failure_type; "
            "public uint? active_processes_before_handle_release;")
    replace("static uint ActiveProcesses(IntPtr job) {", """static uint TotalProcesses(IntPtr job) {
        Accounting accounting;
        Check(QueryInformationJobObject(job, 1, out accounting, (uint)Marshal.SizeOf(typeof(Accounting)), IntPtr.Zero));
        return accounting.totalProcesses;
    }
    static uint ActiveProcesses(IntPtr job) {""")
    replace("    static uint TotalProcesses(IntPtr job) {",
            NATIVE_JOB_OBSERVER + "\n    static uint TotalProcesses(IntPtr job) {")
    replace("public static Result Run(string image, string directory, string[] arguments) {",
            "public static Result Run(string image, string directory, string[] arguments, string gate, string child, string console, string mode) {")
    replace("Result result = new Result(); Process process = null; IntPtr job = IntPtr.Zero;",
            "Result result = new Result(); Process process = null; IntPtr job = IntPtr.Zero; JobEvents observer = null;")
    replace("ExtendedLimit limits = new ExtendedLimit(); limits.basic.flags = 0x2000;",
            "ExtendedLimit limits = new ExtendedLimit(); limits.basic.flags = 0x2008; limits.basic.activeLimit = ProcessLimit(mode); result.process_limit = limits.basic.activeLimit; "
            "result.child_deadline_milliseconds = mode == \"ke\" ? 60000 : 30000;")
    replace("            Check(SetInformationJobObject(job, 9, ref limits, (uint)Marshal.SizeOf(typeof(ExtendedLimit))));",
            "            Check(SetInformationJobObject(job, 9, ref limits, (uint)Marshal.SizeOf(typeof(ExtendedLimit))));\n"
            "            observer = new JobEvents(job, image, child, console, mode); "
            "result.event_deadline_milliseconds = observer.DeadlineMilliseconds; observer.Start();")
    replace("            // The observer has at most the BOM and still needs G; token capture",
            "            ObserveBeforeGate(job, result, image, child, console); observer.ThrowIfFaulted();\n"
            "            // The observer has at most the BOM and still needs G; token capture")
    replace("input.Write('G'); input.Flush(); input.Close(); result.gate_sent = true;", """if (gate == "good") input.Write('G');
            else if (gate == "bad") input.Write('X');
            input.Flush(); input.Close(); result.gate_sent = gate == "good";""")
    replace("if (watch.ElapsedMilliseconds > 30000) throw new TimeoutException(\"Owned child deadline exceeded.\");",
            "if (watch.ElapsedMilliseconds > result.child_deadline_milliseconds) throw new TimeoutException(\"Owned child deadline exceeded.\");")
    replace("            while (!process.WaitForExit(25)) {",
            "            while (!process.WaitForExit(25)) {\n                observer.ThrowIfFaulted();")
    replace("""            process.Dispose(); process = null; result.process_handle_released = true;
            uint remaining;""", """            process.Dispose(); process = null; result.process_handle_released = true;
            if (!observer.Stop(result)) throw new InvalidOperationException("Owned event observer shutdown incomplete.");
            observer.ThrowIfFaulted(); observer = null;
            uint remaining;""")
    replace("result.active_processes_after_cleanup = remaining;",
            "result.active_processes_after_cleanup = remaining; result.total_processes = TotalProcesses(job);")
    replace("""            if (process != null) {
                try {
                    // Failure cleanup closes every owned writer""", """            if (process != null) {
                try {
                    if (assigned) result.active_processes_before_cleanup = ActiveProcesses(job);
                    // Failure cleanup closes every owned writer""")
    replace("""                    if (ActiveProcesses(job) != 0) Check(TerminateJobObject(job, 1));
                    uint remaining;""", """                    if (ActiveProcesses(job) != 0) Check(TerminateJobObject(job, 1));
                    if (observer != null) {
                        if (!observer.Stop(result)) {
                            cleanupOk = false; CleanupFailure(result, new InvalidOperationException("Owned event observer shutdown incomplete."));
                        }
                        if (result.event_reader_joined && result.event_port_closed) observer = null;
                    }
                    uint remaining;""")
    replace("                    if (!CloseHandle(job)) {", """                    if (observer != null && !result.event_reader_joined) {
                        result.event_job_handle_retained = true; cleanupOk = false;
                    } else if (!CloseHandle(job)) {""")
    replace("    $report.runtime_files_sha256 = @{}", """    $systemConsolePath = Join-Path ([Environment]::GetFolderPath([Environment+SpecialFolder]::System)) 'conhost.exe'
    $report.system_console_path = $systemConsolePath
    $report.system_console_provenance = 'Pre/post OS file digest only; mapped-image bytes are not measured.'
    $report.runtime_files_sha256 = @{}""")
    replace("[Diagnostics.Process]::GetCurrentProcess().MainModule.FileName)) {",
            "[Diagnostics.Process]::GetCurrentProcess().MainModule.FileName, $systemConsolePath)) {")
    replace("    $adapterPath = Join-Path $destination 'native-supervisor-adapter.dll'", """    $requestFields = @('image_path','image_sha256','child_sha256','mode','gate')
    if (@($request.psobject.Properties.Name).Count -ne $requestFields.Count -or
        @($request.psobject.Properties.Name | Where-Object { $_ -cnotin $requestFields }).Count -ne 0 -or
        $request.mode -cnotin @('cpu','list','ke','control-healthy','control-fail','control-wait',
            'default-pilot-stop',
            'default-pilot-invalidate',
            'default-pilot-healthy',
            'default-series-stop',
            'default-series-invalidate',
            'default-series-healthy',
            'default-writable-stop',
            'default-writable-invalidate',
            'default-policy-poison',
            'default-policy-reenable',
            'default-session-checkpoints',
            'default-policy-receive-chronology',
            'default-resume-register-close',
            'placement-managed-writer','placement-nominal-writer',
            'job-native-foreign','provider-native-once','provider-native-foreign',
            'control-nested-healthy','control-nested-fail','control-nested-wait',
            'facade-arguments','facade-entered-runtime','facade-loopback-eof','facade-managed-protected-eof') -or
        $request.gate -cnotin @('good','bad','missing')) { throw 'Unadmitted console mode or gate.' }
    $childName = if ($request.mode -ceq 'cpu') { 'cpu-v3-probe.exe' }
        elseif ($request.mode -cin @('list','ke','job-native-foreign','provider-native-once','provider-native-foreign',
            'default-pilot-stop','default-pilot-invalidate','default-pilot-healthy',
            'default-series-stop','default-series-invalidate','default-series-healthy',
            'default-writable-stop','default-writable-invalidate',
            'default-policy-poison','default-policy-reenable','default-session-checkpoints',
            'default-policy-receive-chronology',
            'default-resume-register-close',
            'placement-managed-writer','placement-nominal-writer')) { 'engine-tests.exe' }
        elseif ($request.mode -cin @('facade-arguments','facade-entered-runtime','facade-loopback-eof','facade-managed-protected-eof')) { 'engine-facade-consumer.exe' }
        else { 'gate-control.exe' }
    $child = Get-Item -LiteralPath (Join-Path $destination $childName)
    if ($child.PSIsContainer -or (($child.Attributes -band [IO.FileAttributes]::ReparsePoint) -ne 0)) {
        throw 'Unadmitted fixed child image.'
    }
    if (@(Get-Process -Name $child.BaseName -ErrorAction SilentlyContinue).Count -ne 0) { throw 'Fixed child basename is already active.' }
    $report.child_image_settings_before = @(Inspect-ImageSettings $child.Name $child.FullName)
    if ($report.child_image_settings_before.Count -ne 0) { throw 'Fixed child has inherited image settings.' }
    $report.child_image_sha256_before = (Get-FileHash -LiteralPath $child.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($report.child_image_sha256_before -cne $request.child_sha256) { throw 'Fixed child differs from the build.' }
    $memory = Get-CimInstance Win32_OperatingSystem
    $report.available_memory_kib = [long]$memory.FreePhysicalMemory
    if ($report.available_memory_kib -lt 2097152) { throw 'Main Windows available memory is below 2 GiB.' }
    $adapterPath = Join-Path $destination 'native-supervisor-adapter.dll'""")
    replace("""    $arguments = @($request.claims.candidate_sha256, $request.claims.observer_source_sha256,
        $request.claims.build_receipt_sha256, $request.claims.profile)
    foreach ($argument in $arguments[0..2]) { if ($argument -cnotmatch '^[a-f0-9]{64}$') { throw 'Unadmitted native identity claim.' } }
    if ($arguments[3] -cnotin @('debug','release','verify-release')) { throw 'Unadmitted native profile.' }""",
            "    $arguments = @($request.mode)")
    replace("""    try {
        $report.execution = [NevroTickEngineSupervisor]::Run($image.FullName, $destination, $arguments)
        $report.image_sha256_after = (Get-FileHash -LiteralPath $image.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
    } finally { $imageLock.Dispose() }
    $report.image_settings_after = @(Inspect-ImageSettings $image.Name $image.FullName)""", """    try {
        $childLock = [IO.File]::Open($child.FullName, [IO.FileMode]::Open, [IO.FileAccess]::Read, [IO.FileShare]::Read)
        try {
            if ((Get-FileHash -LiteralPath $image.FullName -Algorithm SHA256).Hash.ToLowerInvariant() -cne $request.image_sha256 -or
                (Get-FileHash -LiteralPath $child.FullName -Algorithm SHA256).Hash.ToLowerInvariant() -cne $request.child_sha256) {
                throw 'Locked image bytes differ from the frozen build.'
            }
            $report.execution = [NevroTickEngineSupervisor]::Run($image.FullName, $destination, $arguments, $request.gate, $child.FullName, $systemConsolePath, $request.mode)
            $report.image_sha256_after = (Get-FileHash -LiteralPath $image.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
            $report.child_image_sha256_after = (Get-FileHash -LiteralPath $child.FullName -Algorithm SHA256).Hash.ToLowerInvariant()
        } finally { $childLock.Dispose() }
    } finally { $imageLock.Dispose() }
    $report.child_image_settings_after = @(Inspect-ImageSettings $child.Name $child.FullName)
    if ($report.child_image_settings_after.Count -ne 0 -or $report.child_image_sha256_after -cne $report.child_image_sha256_before) {
        throw 'Fixed child image or inherited settings changed.'
    }
    $report.image_settings_after = @(Inspect-ImageSettings $image.Name $image.FullName)""")
    return source


def observed_child_pid(stderr, authored, *, mode=None):
    """Join one fixed launcher marker to kernel identity; it grants no identity alone."""
    import re
    require(type(authored) is int and authored in (1, 2, 3), "Unadmitted authored-process count.")
    require(type(stderr) is str and len(stderr) <= 512, "Native stderr must be bounded text.")
    if authored == 1:
        require(stderr == "launcher-stage=gate;os-error=none\n", "Gate refusal exposed a child marker or another failure.")
        return None
    if mode is not None:
        mode_profile(mode)
    if mode in FACADE_CASES:
        require(authored == 2, "Facade has exactly one authored child.")
        diagnostics = (FACADE_CASES[mode][1],)
        lines = stderr.splitlines(keepends=True)
        if mode == FACADE_EOF_MODE:
            diagnostics += (FACADE_EOF_BUFFERED,)
            require(lines.count(FACADE_EOF_WITNESS) == 1
                    and any(row in diagnostics for row in lines[:lines.index(FACADE_EOF_WITNESS)]),
                    "Facade loopback EOF needs a joined peer witness after the real Engine refusal.")
            lines.remove(FACADE_EOF_WITNESS)
        if mode == MANAGED_EOF_MODE:
            diagnostics += (MANAGED_EOF_BUFFERED,)
        matching = [line for line in lines if line in diagnostics]
        require(len(lines) == 2 and len(matching) == 1,
                "Facade needs its exact diagnostic and one launcher marker.")
        # Spawned child and launcher can write either complete line first.
        lines.remove(matching[0])
        stderr = lines[0]
    match = re.fullmatch(r"launcher-child-pid=([1-9][0-9]{0,9})\n", stderr)
    require(match is not None, "Native child needs exactly one bounded PID marker.")
    pid = int(match[1])
    require(pid <= 0xffffffff, "Native child PID exceeds u32.")
    return pid


def validate_processes(value, request, result, authored):
    """Require complete NEW identities, bounded OS helpers and pre-gate exclusion.

    Notification delivery is best-effort. Missing evidence refuses qualification;
    EXIT IDs join historical NEW identities and are never reopened by the collector.
    """
    from pathlib import PureWindowsPath

    def uint(item, maximum=0xffffffff, minimum=0):
        return type(item) is int and minimum <= item <= maximum

    root = result["pid"]
    maximum, after_gate_authored, child_name = mode_profile(request["mode"])
    require(authored == (after_gate_authored if request["gate"] == "good" else 1),
            "Authored-process count differs from the closed mode and gate.")
    child = observed_child_pid(result.get("stderr"), authored, mode=request["mode"])
    require(child != root, "Launcher and child PIDs cannot alias.")
    root_name = PureWindowsPath(request["image_path"]).name.casefold()
    console_path = value.get("system_console_path")
    require(type(console_path) is str and PureWindowsPath(console_path).is_absolute()
            and PureWindowsPath(console_path).parent.name.casefold() == "system32"
            and PureWindowsPath(console_path).name.casefold() == "conhost.exe"
            and console_path in value["runtime_files_sha256"], "Missing pre/post System32 console-file provenance.")
    for name in ("event_reader_joined", "event_port_closed"):
        require(result.get(name) is True, "Owned event reader or port did not retire.")
    for name in ("event_limit_reached", "event_job_handle_retained"):
        require(result.get(name) is False, "Owned process observation exceeded a bound or retained custody.")
    for name in ("event_read_failure_type", "event_shutdown_failure_type", "membership_read_failure_type"):
        require(result.get(name) is None, "Owned process observation has an unresolved read or shutdown failure.")
    total = result.get("total_processes")
    require(uint(total, maximum, authored), "Unexpected lifetime association count.")
    require(uint(result.get("total_processes_before_gate"), maximum, 1)
            and result["total_processes_before_gate"] <= total
            and uint(result.get("active_processes_before_gate"), maximum, 1)
            and result["active_processes_before_gate"] <= result["total_processes_before_gate"],
            "Invalid pre-gate accounting.")

    def identity(member, *, before_gate=False):
        require(type(member) is dict and uint(member.get("pid"), minimum=1)
                and member.get("read_failure_type") is None, "Unverified owned process identity.")
        pid = member["pid"]
        if pid == root:
            role, basename = "launcher", root_name
        elif child is not None and pid == child and not before_gate:
            role, basename = "child", child_name
        elif authored == 3 and not before_gate and member.get("image_role") == "child":
            role, basename = "nested-child", child_name
        else:
            role, basename = "system-console", "conhost.exe"
        require(type(member.get("basename")) is str and member["basename"].casefold() == basename
                and member.get("image_role") == ("child" if role == "nested-child" else role)
                and member.get("system_console") is (role == "system-console"),
                "Unexpected image path identity or child present before its gate.")
        return pid, role, member["basename"].casefold()

    events = result.get("job_events")
    require(type(events) is list and 1 <= len(events) < 16, "Incomplete or over-limit job notifications.")
    new = {}
    exits = set()
    for event in events:
        require(type(event) is dict and uint(event.get("kind")) and event["kind"] in (4, 6, 7, 8),
                "Unknown job notification.")
        if event["kind"] == 6:
            pid, role, basename = identity(event)
            require(pid not in new, "Duplicate NEW process identity.")
            new[pid] = (role, basename)
        else:
            require(event.get("basename") is None and event.get("image_role") is None
                    and event.get("system_console") is False and event.get("read_failure_type") is None,
                    "EXIT/zero event must not reopen or invent process identity.")
            if event["kind"] == 4:
                require(event.get("pid") == 0 and type(event.get("pid")) is int, "Zero-active event has a PID.")
            else:
                require(uint(event.get("pid"), minimum=1), "Exit event has no bounded PID.")
                pair = (event["kind"], event["pid"])
                require(pair not in exits, "Duplicate exit notification.")
                exits.add(pair)
    require(len(new) == total and root in new and (child is None or child in new)
            and sum(role == "system-console" for role, _ in new.values()) <= authored
            and sum(role == "nested-child" for role, _ in new.values()) == (1 if authored == 3 else 0)
            and all(pid in new for _, pid in exits), "Missed, unknown or excessive lifetime process associations.")
    before = result.get("members_before_gate")
    require(type(before) is list and 1 <= len(before) <= maximum
            and uint(result.get("assigned_before_gate"), maximum, 1)
            and uint(result.get("listed_before_gate"), maximum, 1)
            and result["assigned_before_gate"] == result["listed_before_gate"] == len(before),
            "Incomplete pre-gate membership snapshot.")
    seen = set()
    for member in before:
        pid, role, basename = identity(member, before_gate=True)
        require(pid not in seen and new.get(pid) == (role, basename), "Pre-gate identity is duplicate or unaccounted.")
        seen.add(pid)
    require(root in seen, "Launcher absent from pre-gate custody.")



def validate_console(value, request, supervisor_hash, request_hash, *, expected_exit=0,
                     expected_processes=2, timeout_control=False):
    """Classify exact intended controls separately from genuine completed tests."""
    import re
    require(type(request) is dict and set(request) == {"image_path", "image_sha256", "child_sha256", "mode", "gate"}
            and request["gate"] in ("good", "bad", "missing"), "Unadmitted fixed native request fields or gate.")
    maximum, _, _ = mode_profile(request["mode"])
    if request["mode"] in FACADE_CASES and request["gate"] == "good":
        require(expected_exit == FACADE_CASES[request["mode"]][0] and expected_processes == 2,
                "Facade disposition must match its fixed expected refusal.")
    require(type(value) is dict, "Native supervisor result must be an object.")
    if (type(value.get("schema_version")) is int and value["schema_version"] == 1
            and value.get("status") == "incomplete" and value.get("execution") is None
            and type(value.get("available_memory_kib")) is int and 0 <= value["available_memory_kib"] < 2097152
            and value.get("error") == "Main Windows available memory is below 2 GiB."):
        raise Incomplete("Main Windows available memory is below 2 GiB; no test process started.")
    for name in ("os", "execution"):
        require(type(value.get(name)) is dict, "Native observation has a malformed nested object.")
    expected_status = "completed" if expected_exit == 0 else "incomplete"
    if (type(value.get("schema_version")) is not int or value["schema_version"] != 1
            or value.get("status") != expected_status
            or value.get("registry_writes") is not False
            or value.get("image_settings_before") != [] or value.get("image_settings_after") != []
            or value.get("child_image_settings_before") != [] or value.get("child_image_settings_after") != []
            or value.get("image_sha256_before") != request["image_sha256"]
            or value.get("image_sha256_after") != request["image_sha256"]
            or value.get("child_image_sha256_before") != request["child_sha256"]
            or value.get("child_image_sha256_after") != request["child_sha256"]
            or value.get("supervisor_sha256") != supervisor_hash or value.get("request_sha256") != request_hash
            or value.get("os", {}).get("is_64_bit") is not True
            or type(value.get("available_memory_kib")) is not int or value["available_memory_kib"] < 2097152
            or value.get("runtime_inputs_unchanged") is not True
            or type(value.get("adapter_sha256")) is not str
            or not re.fullmatch("[a-f0-9]{64}", value["adapter_sha256"])
            or type(value.get("runtime_files_sha256")) is not dict or not value["runtime_files_sha256"]
            or any(type(name) is not str or type(digest) is not str or not re.fullmatch("[a-f0-9]{64}", digest)
                   for name, digest in value["runtime_files_sha256"].items())):
        raise Incomplete("Native image, settings, runtime or completed-state admission failed.")
    result = value.get("execution", {})
    if request["mode"] in FACADE_CASES and request["gate"] == "good":
        require(type(result) is dict and result.get("stdout") == "",
                "Refused facade call emitted an unexpected stdout payload.")
    if (type(result.get("pid")) is not int or result["pid"] <= 0
            or type(result.get("exit_code")) is not int or result["exit_code"] != expected_exit
            or result.get("cleanup_failure") is not None or result.get("output_overflow") is not False
            or result.get("cleanup_complete") is not True or result.get("streams_released") is not True
            or result.get("process_handle_released") is not True
            or type(result.get("child_deadline_milliseconds")) is not int
            or result["child_deadline_milliseconds"] != (60000 if request["mode"] == "ke" else 30000)
            or type(result.get("event_deadline_milliseconds")) is not int
            or result["event_deadline_milliseconds"] != (75000 if request["mode"] == "ke" else 45000)
            or type(result.get("process_limit")) is not int or result["process_limit"] != maximum
            or result.get("gate_sent") is not (request["gate"] == "good")
            or result.get("stdin_encoding") != "utf-8" or result.get("stdin_preamble_hex") != "EF-BB-BF"
            or type(result.get("active_processes_before_cleanup")) is not int
            or (not timeout_control and result.get("active_processes_before_cleanup") != 0)
            or (timeout_control and (type(result.get("active_processes_before_cleanup")) is not int
                                     or not 1 <= result["active_processes_before_cleanup"] <= maximum))
            or type(result.get("active_processes_after_cleanup")) is not int
            or result["active_processes_after_cleanup"] != 0
            or type(result.get("total_processes")) is not int or not 1 <= result["total_processes"] <= maximum):
        raise Incomplete("Owned child failed its exact exit, gate, containment or cleanup contract.")
    failure = result.get("failure")
    if timeout_control:
        if failure != "TimeoutException: Owned child deadline exceeded.":
            raise Incomplete("Waiting control did not reach its intended supervisor timeout.")
    elif failure is not None:
        raise Incomplete("Unexpected supervisor failure in a completed control/test.")
    parent, launcher = result.get("parent_token", {}), result.get("child_token", {})
    for token in (parent, launcher):
        require(type(token) is dict, "Native token observation must be an object.")
        if (not re.fullmatch(r"S-1-\d+(?:-\d+)+", token.get("user_sid", ""))
                or token.get("elevated") is not False or token.get("administrator") is not False
                or type(token.get("elevation_type")) is not int or token["elevation_type"] not in (1, 3)):
            raise Incomplete("Expected ordinary-user supervisor/launcher token was not observed.")
    if parent["user_sid"] != launcher["user_sid"]:
        raise Incomplete("Launcher user differs from the ordinary supervisor.")
    validate_processes(value, request, result, expected_processes)
    return result


def native(args, report):
    """Run fixed controls, CPU and exact discovery before the selected cohort."""
    import re
    import uuid
    if args.build_receipt is None or args.native_dir is None:
        raise Incomplete("Native lane requires --build-receipt and a fresh --native-dir.")
    build_path = args.build_receipt.absolute()
    build = read_record(build_path)
    require(type(build) is dict, "Native build receipt must be an object.")
    require(not any(type(key) is str and key.startswith(
        ("fault", "facade_fault", "variant_", "prepared_faults", "healthy_build")) for key in build),
        "Fault-build metadata is not admitted by the ordinary native lane.")
    policy, source_before = PRODUCT.admit_product(ROOT)
    extras_before = {name: PRODUCT.sha256(ROOT / name) for name in EXTRAS}
    facade = getattr(args, "facade_consumer", False)
    loopback_eof = facade_loopback_eof_requested(args)
    managed_eof = facade_managed_protected_eof_requested(args)
    require(type(facade) is bool and build.get("facade_consumer", False) is facade,
            "Native facade selection differs from the exact build profile.")
    default = default_cohort_requested(args)
    placement = placement_cohort_requested(args)
    require(type(build.get("placement_cohort", False)) is bool
            and build.get("placement_cohort", False) is placement,
            "Native placement selection differs from the exact build profile.")
    require(type(build.get("default_cohort", False)) is bool
            and build.get("default_cohort", False) is default,
            "Native default selection differs from the exact build profile.")
    if default:
        require(build.get("test_features") == [] and build.get("bookends_passed") is True
                and build.get("product_target_flags") == ENV.MSVC_FLAGS,
                "Default build lacks exact empty features, custody or product v3 flags.")
    if placement:
        require(build.get("test_features") == [] and build.get("bookends_passed") is True
                and build.get("product_target_flags") == ENV.MSVC_FLAGS,
                "Placement build lacks exact empty features, custody or product v3 flags.")
    if (build.get("status") != "passed" or build.get("lane") != "build"
            or build.get("product_source_sha256") != source_before
            or build.get("extra_inputs_sha256") != extras_before
            or build.get("candidate_sha256") != ALLOCATION.canonical_digest(source_before)):
        raise Incomplete("Native build receipt does not bind the complete current candidate and runner.")
    roster_record, roster, roster_raw = None, None, None
    if facade:
        require(build.get("bookends_passed") is True and build.get("facade_target_flags") == ENV.MSVC_FLAGS,
                "Facade build lacks its success bookend or product v3 flags.")
    else:
        roster_record = build.get("test_roster")
        roster_path = build_path.parent / TEST_ROSTER_NAME
        require(type(roster_record) is dict and set(roster_record) == {"path", "sha256"}
                and roster_record["path"] == str(roster_path),
                "Native build lacks its frozen independent test roster.")
        roster, roster_raw = read_test_roster(roster_path, ALLOCATION.canonical_digest(source_before), extras_before,
                                              default_cohort=default, placement_cohort=placement)
        require(hashlib.sha256(roster_raw).hexdigest() == roster_record["sha256"],
                "Native independent test roster differs from the build binding.")
    artifacts = build.get("artifacts", {})
    names = {"engine-tests.exe", "cpu-v3-probe.exe", "gated-launcher.exe", "gate-control.exe"}
    if facade:
        names = (names - {"engine-tests.exe"}) | {"engine-facade-consumer.exe"}
    if type(artifacts) is not dict or set(artifacts) != names:
        raise Incomplete("Native build does not contain the four fixed bounded images.")
    for name, item in artifacts.items():
        require(type(item) is dict and type(item.get("path")) is str,
                "Native artifact admission must be a typed record.")
        path = Path(item["path"])
        if (path.is_symlink() or not path.is_file() or path.stat().st_size > 64 * 1024 * 1024
                or path.parent != build_path.parent or PRODUCT.sha256(path) != item.get("sha256")):
            raise Incomplete("Native emitted image is linked, absent, unbounded or changed.")
    if not (facade or default or placement):
        validate_ke_launcher_selection(PE.Q.C.read_analysis_bytes(
            ROOT / LAUNCHER, maximum=MAX_LAUNCHER_SOURCE_BYTES).decode("utf-8"))
    directory = native_directory(args.native_dir)
    # The host directory must be an owned fresh LocalAppData child, not UNC/WSL.
    prefix = Path("/mnt/c/Users/perso/AppData/Local/NevroTick")
    if shutil.disk_usage(prefix).free < 2 * 1024**3:
        raise Incomplete("Windows native output needs at least 2 GiB free disk.")
    directory.mkdir(mode=0o700, parents=True)
    powershell = "/mnt/c/Windows/System32/WindowsPowerShell/v1.0/powershell.exe"
    ps_before = PRODUCT.sha256(Path(powershell))
    report.update(candidate_sha256=build["candidate_sha256"], build_receipt=str(build_path),
                  build_receipt_sha256=PRODUCT.sha256(build_path), windows_directory=str(directory),
                  powershell_sha256=ps_before, product_source_sha256=source_before,
                  extra_inputs_sha256=extras_before, observations=[],
                  test_roster=roster_record,
                  full_native_ke_qualification=False,
                  scripted_ke_roster_complete=False, completed_nested_tests=[],
                  required_held_tests=[] if facade or default or placement else roster["held_ke_tests"],
                  selected_ke_tests=[] if facade or default or placement else roster["selected_ke_tests"],
                  default_cohort=default, default_cohort_complete=False, completed_default_tests=[],
                  selected_default_tests=roster["selected_default_tests"] if default else [],
                  facade_consumer=facade, facade_refusal_cases_complete=False,
                  token_scope="Supervisor and launcher directly observed; fixed child inherits primary token.",
                  scope="Scripted KE component tests and controlled loopback UDP with three separately checked nested-child cases. Manual native TLS/provider fixtures remain separate; no full native, production, service or clock qualification.")
    if placement:
        report.update(placement_cohort=True, placement_cohort_complete=False, completed_placement_tests=[],
                      selected_placement_tests=roster["selected_placement_tests"])
    elif not (facade or default):
        report.update(manual_native_tests=roster["manual_native_tests"],
                      unexecuted_manual_native_tests=list(roster["manual_native_tests"]))
    script = native_supervisor()
    supervisor_hash = hashlib.sha256(script.encode()).hexdigest()
    cases = (
            ("missing-gate", "control-healthy", "missing", 126, 1),
            ("wrong-gate", "control-healthy", "bad", 126, 1),
            ("healthy-control", "control-healthy", "good", 0, 2),
            ("failure-control", "control-fail", "good", 7, 2),
            ("timeout-control", "control-wait", "good", 1, 2),
            ("nested-healthy-control", "control-nested-healthy", "good", 0, 3),
            ("nested-failure-control", "control-nested-fail", "good", 7, 3),
            ("nested-timeout-control", "control-nested-wait", "good", 1, 3),
            ("cpu", "cpu", "good", 0, 2),
            ("list", "list", "good", 0, 2),
            ("ke", "ke", "good", 0, 2),
            ("native-job-foreign", "job-native-foreign", "good", 0, 3),
            ("native-provider-once", "provider-native-once", "good", 0, 3),
            ("native-provider-foreign", "provider-native-foreign", "good", 0, 3))
    if default or placement:
        plain_cases = PLACEMENT_COHORT_CASES if placement else DEFAULT_COHORT_CASES
        cases = (*cases[:5], ("cpu", "cpu", "good", 0, 2), ("list", "list", "good", 0, 2),
                 *((mode, mode, "good", 0, 2) for mode in plain_cases))
        if placement:
            report["scope"] = ("Plain application placement and report writer with real loopback UDP fixtures; "
                               "no physical clock calibration or actual CLI process qualification.")
        else:
            report["scope"] = ("Selected default Engine reply-guard cohort with real loopback UDP fixtures; "
                               "no public network, provider adoption, service, clock or whole Core/Engine qualification.")
    if facade:
        cases = (*cases[:5], ("cpu", "cpu", "good", 0, 2),
                 *((mode, mode, "good", FACADE_CASES[mode][0], 2)
                   for mode in facade_native_modes(loopback_eof, managed_eof)))
        report["scope"] = "Two fixed production-facade refusals before provider effects; no mutation or full native qualification."
        if loopback_eof:
            report.update(facade_loopback_eof_observed=False,
                          scope="Fixed facade refusals and one owned loopback ClientHello/EOF. No healthy TLS or release qualification.")
        if managed_eof:
            report.update(facade_managed_protected_eof_observed=False,
                          scope="Public managed stale-definition refusal and current loopback EOF with no reported observer error; no healthy TLS, protected UDP or mapping qualification.")
    for label, mode, gate, expected_exit, expected_count in cases:
        observation = directory / label
        observation.mkdir(mode=0o700)
        launcher = observation / ("NevroTick-" + uuid.uuid4().hex + ".exe")
        selected_child = mode_profile(mode)[2]
        for name, destination in (("gated-launcher.exe", launcher),
                                  (selected_child, observation / selected_child)):
            shutil.copyfile(artifacts[name]["path"], destination)
            if PRODUCT.sha256(destination) != artifacts[name]["sha256"]:
                raise Incomplete("Native staging image changed while copying.")
        supervisor = observation / "supervise.ps1"
        supervisor.write_text(script, encoding="utf-8", newline="")
        request = {"image_path": ALLOCATION.windows_path(launcher), "image_sha256": artifacts["gated-launcher.exe"]["sha256"],
                   "child_sha256": artifacts[selected_child]["sha256"], "mode": mode, "gate": gate}
        request_path = observation / "request.json"
        request_path.write_text(json.dumps(request, indent=2) + "\n")
        request_hash = PRODUCT.sha256(request_path)
        raw = ENV.run([powershell, "-NoLogo", "-NoProfile", "-NonInteractive", "-File",
                       ALLOCATION.windows_path(supervisor), "-RequestPath", ALLOCATION.windows_path(request_path)],
                      cwd=observation, timeout=90, environment={"PATH": "/usr/bin:/bin", "WSLENV": ""})
        (args.output_dir / (label + "-invocation.json")).write_text(json.dumps(raw, indent=2) + "\n")
        record = {"label": label, "mode": mode, "gate": gate, "invocation": raw,
                  "raw_path": str(observation / "native-supervision.json")}
        report["observations"].append(record)
        raw_path = observation / "native-supervision.json"
        if raw_path.is_symlink() or not raw_path.is_file() or raw_path.stat().st_size > 1024**2:
            raise Incomplete("Native supervisor produced no bounded regular result.")
        value = read_record(raw_path)
        record["supervision"] = value
        (args.output_dir / (label + "-supervision.json")).write_text(json.dumps(value, indent=2) + "\n")
        expected_parent_exit = 0 if expected_exit == 0 else 1
        if raw["failure"] is not None or raw["exit_code"] != expected_parent_exit:
            raise Incomplete("Native supervisor did not preserve its intended invocation outcome.")
        result = validate_console(value, request, supervisor_hash, request_hash,
                                  expected_exit=expected_exit, expected_processes=expected_count,
                                  timeout_control=label in ("timeout-control", "nested-timeout-control"))
        output, error = result["stdout"], result["stderr"]
        observed_child_pid(error, expected_count, mode=mode)
        if mode in FACADE_CASES and output != "":
            raise Incomplete("Refused facade call emitted an unexpected stdout payload.")
        expected_output = {"missing-gate": "", "wrong-gate": "",
                           "healthy-control": "GATE_CONTROL_HEALTHY\n",
                           "failure-control": "GATE_CONTROL_FAIL\n",
                           "timeout-control": "GATE_CONTROL_WAIT\n",
                           "nested-healthy-control": "GATE_NESTED_HEALTHY\n",
                           "nested-failure-control": "GATE_NESTED_FAIL\n",
                           "nested-timeout-control": "GATE_NESTED_WAIT\n",
                           "cpu": "cpu-os-v3=accepted\n"}.get(label)
        if expected_output is not None and output != expected_output:
            raise Incomplete("Native control/CPU output did not match independent expected facts.")
        if label == "list":
            report["windows_roster"] = validate_test_listing(output, roster)
        if label == "ke":
            validate_ke_execution(output, roster)
        if mode in DEFAULT_COHORT_CASES:
            record["exact_default_test"] = validate_default_execution(output, roster, mode)
            report["completed_default_tests"].append(record["exact_default_test"])
        if mode in PLACEMENT_COHORT_CASES:
            record["exact_placement_test"] = validate_default_execution(output, roster, mode, placement_cohort=True)
            report["completed_placement_tests"].append(record["exact_placement_test"])
        if mode in NESTED_TEST_MODES:
            record["exact_test"] = validate_nested_execution(output, roster, mode)
            report["completed_nested_tests"].append(NESTED_TEST_MODES[mode])
            report["required_held_tests"] = [name for name in roster["held_ke_tests"]
                                              if name not in report["completed_nested_tests"]]
        record["classification"] = "expected-rejection" if expected_exit else "completed"
        if mode == FACADE_EOF_MODE:
            record["peer_clienthello_eof_joined"] = True
    if (PRODUCT.admit_product(ROOT)[1] != source_before
            or {name: PRODUCT.sha256(ROOT / name) for name in EXTRAS} != extras_before
            or PRODUCT.sha256(build_path) != report["build_receipt_sha256"]
            or PRODUCT.sha256(Path(powershell)) != ps_before
            or (not facade and read_test_roster(roster_path, build["candidate_sha256"], extras_before,
                                                default_cohort=default, placement_cohort=placement)[1] != roster_raw)
            or any(PRODUCT.sha256(Path(item["path"])) != item["sha256"] for item in artifacts.values())):
        raise Incomplete("Native source, images, receipt, supervisor recipe or PowerShell drifted.")
    if facade:
        report["facade_refusal_cases_complete"] = True
        if loopback_eof:
            report["facade_loopback_eof_observed"] = True
        if managed_eof:
            report["facade_managed_protected_eof_observed"] = True
    elif placement:
        require(report["completed_placement_tests"] == list(PLACEMENT_COHORT_CASES.values()),
                "Not both fixed placement writer cases completed.")
        report["placement_cohort_complete"] = True
    elif default:
        require(report["completed_default_tests"] == list(DEFAULT_COHORT_CASES.values()),
                "Not every fixed default reply-guard case completed.")
        report["default_cohort_complete"] = True
    else:
        report["scripted_ke_roster_complete"] = report["required_held_tests"] == []
    report["status"] = "passed"


def main():
    """Select one closed lane and retain its partial outcome in fresh owned output."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--lane", required=True, choices=("build", "native", "prepare-facade-faults"))
    parser.add_argument("--raw-outcomes", type=Path,
                        help="Read only the exact retained V19 outcomes for source-only facade preparation.")
    parser.add_argument("--default-cohort", action="store_true",
                        help="Select only the fixed default-profile reply-guard cohort; no complete-library claim.")
    parser.add_argument("--placement-cohort", action="store_true",
                        help="Select only two fixed plain application placement writer cases.")
    parser.add_argument("--facade-consumer", action="store_true",
                        help="Select two fixed provider-free production facade refusal cases; no mutation scoring.")
    parser.add_argument("--facade-loopback-eof", action="store_true",
                        help="Also observe one real loopback ClientHello/EOF with the native facade consumer.")
    parser.add_argument("--facade-managed-protected-eof", action="store_true",
                        help="Select the public managed stale/current EOF pair instead of the raw EOF case.")
    parser.add_argument("--facade-fault", choices=tuple(row[0] for row in FACADE_FAULTS),
                        help="Build one admitted prepared overlay; no native execution or classification.")
    parser.add_argument("--jobs", type=int, choices=range(1, 13), default=6,
                        help="Bound Cargo build concurrency; default 6, maximum 12.")
    for name in ("vendor-dir", "toolchain-dir", "xwin-config", "xwin-inventory", "output-dir",
                 "build-receipt", "native-dir", "test-roster", "prepared-faults", "healthy-build-receipt"):
        parser.add_argument("--" + name, type=Path, required=name == "output-dir")
    args = parser.parse_args()
    report = {"schema_version": 1, "lane": args.lane, "status": "incomplete", "started_unix": time.time(),
              "scope": "Bounded Windows Engine scripted KE libtest; no product or release qualification."}
    output = None
    try:
        facade_loopback_eof_requested(args)
        facade_managed_protected_eof_requested(args)
        require(not args.default_cohort or (args.lane in ("build", "native") and not args.facade_consumer),
                "Default cohort requires build/native and excludes facade selections.")
        if args.default_cohort:
            report["scope"] = "Selected default Windows Engine reply-guard cohort; native execution remains separate."
        if placement_cohort_requested(args):
            require(args.lane in ("build", "native"), "Placement cohort requires build/native.")
            report["scope"] = "Selected plain application placement writers; native execution remains separate."
        fault_options = (args.facade_fault, args.prepared_faults, args.healthy_build_receipt)
        require(not any(value is not None for value in fault_options)
                or (args.lane == "build" and args.facade_consumer and all(value is not None for value in fault_options)),
                "Facade fault flags require build, facade consumer and both admitted parent receipts.")
        output = ALLOCATION.new_output(args.output_dir)
        args.output_dir = output
        if args.lane == "prepare-facade-faults":
            prepare_facade_faults(args, report)
        else:
            (build if args.lane == "build" else native)(args, report)
    except (Incomplete, PE.Incomplete, ANALYSIS.Incomplete, OSError, ValueError, KeyError, TypeError) as error:
        report["error"] = str(error)
    report["completed_unix"] = time.time()
    if output is not None:
        (output / "verification.json").write_text(json.dumps(report, indent=2, default=str) + "\n")
    print(json.dumps({"status": report["status"], "lane": args.lane,
                      "error": report.get("error"), "verification_path": str(output / "verification.json") if output else None}))
    expected_status = ("prepared-not-executed" if args.lane == "prepare-facade-faults" else
                       "fault-build-completed" if args.lane == "build" and args.facade_fault is not None else "passed")
    return 0 if report["status"] == expected_status else 1


if __name__ == "__main__":
    raise SystemExit(main())
