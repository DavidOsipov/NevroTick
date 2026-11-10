"""Pure four-owner analysis controls; no product, compiler or analyzer execution."""
from __future__ import annotations

from contextlib import contextmanager
import copy
import difflib
import importlib.util
import hashlib
import json
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location("frequency_prior_controls", ROOT / "scripts/verify-core-analysis.py")
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


# These tiny context windows are synthetic parser fixtures, not complete Rust
# source or reusable historical evidence. Blank padding retains diagnostic line
# and byte locations. Only exact fixture bytes receive the old identity below.
HISTORICAL_CONTEXTS = {
    'crates/time-core/src/reference_input/rate_delivery.rs': [
        (450, 17637, '            CursorProgress::Unresolved => FixtureRateCursorStatus::Unresolved,\n            CursorProgress::Exhausted => FixtureRateCursorStatus::Exhausted,\n        }\n    }\n\n    /// Copy the sole admissible local identity, absent after either terminal state.\n    #[must_use]\n    pub const fn next_ordinal(&self) -> Option<NonZeroU64> {\n        match self.progress {\n            CursorProgress::Pending(value) => Some(value),\n            _ => None,\n        }\n    }\n\n    /// Return the last known active endpoint without inventing Unknown delivery.\n    #[must_use]\n    pub const fn last_active_end(&self) -> Option<ActiveAwakeNs> {\n        self.last_active\n    }\n\n    /// Return the independently ordered last known effective endpoint.\n'),
        (594, 23498, '    FixtureChartFrames::new(\n        first.source_frame(),\n        first.source_generation(),\n        interval.frames.target_frame(),\n        interval.frames.target_generation(),\n    )\n    .map_err(|source| FixtureRateDeliveryError::Chart { source })?;\n    if let Some(last) = cursor.last_active\n        && interval.span.active.0 < last\n    {\n        return Err(FixtureRateDeliveryError::ActiveOverlap);\n    }\n    if let Some(last) = cursor.last_monotonic\n        && interval.span.effective.0 < last\n    {\n        return Err(FixtureRateDeliveryError::MonotonicOverlap);\n    }\n    Ok(())\n}\n\n/// Admit the whole live relationship and calculate checked nonwrapping progress.\n/// # Errors\n'),
    ],
    'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs': [
        (466, 18397, '    /// Copy original fitted rate SD, not a gain weight.\n    #[must_use]\n    pub const fn rate_standard_deviation_seconds_per_second(&self) -> f64 {\n        self.model.rate_sd\n    }\n    /// Copy the original actual model-producing emission.\n    #[must_use]\n    pub const fn emitted_round(&self) -> NonZeroU64 {\n        self.model.round\n    }\n    /// Copy original source identity without changing its generation.\n    #[must_use]\n    pub const fn source_context(&self) -> FixtureContext {\n        self.model.source\n    }\n    /// Copy current mathematical frame tags, separate from original source tags.\n    #[must_use]\n    pub const fn frames(&self) -> FixtureChartFrames {\n'),
        (735, 28771, '    }\n\n    /// Retain a committed source/readiness refusal without inventing a model.\n    pub(super) fn no_ready(&mut self, round: Option<NonZeroU64>) {\n        if !self.is_enabled() {\n            return;\n        }\n        if let Some(round) = round\n            && let Err(reason) = self.consume(round)\n        {\n            self.admission = FixtureFrequencyAdmission::Refused {\n                emitted_round: round,\n                reason,\n            };\n            return;\n        }\n        self.admission = FixtureFrequencyAdmission::NoReady {\n'),
        (803, 31223, "        chart: &FixtureAffineChart,\n        now: MonotonicNs,\n        profile: FixtureFrequencyProfile,\n    ) {\n        if self.unavailable.is_some() {\n            return;\n        }\n        if let Some(model) = self.model\n            && let Err(error) = predict(&model, profile, chart, now)\n        {\n            self.unavailable = Some(match error {\n                FixtureFrequencyError::QueryBeforeReference\n                | FixtureFrequencyError::QueryOutsideDomain => {\n                    FixtureFrequencyError::AnchorRebaseUnavailable\n                }\n                other => other,\n            });\n        }\n    }\n\n    /// Stage a whole fresh record and its current support without publishing fields.\n    /// # Errors\n    /// Preserve old model on unsupported initialization/update/anchor/support.\n    fn prepare_model(\n        &self,\n        estimate: &FixtureChartCombinedEstimate<'_, '_, '_>,\n        round: NonZeroU64,\n"),
    ],
    'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs': [
        (409, 16645, '    /// Original numerical mean error, separate from statistics.\n    #[must_use]\n    pub const fn phase_error_seconds(&self) -> f64 {\n        self.error\n    }\n    /// Actual local emission ordinal of the exclusively leased producer.\n    #[must_use]\n    pub const fn emitted_round(&self) -> NonZeroU64 {\n        self.round\n    }\n    /// Original source context without remapping or source authority.\n    #[must_use]\n    pub const fn source_context(&self) -> FixtureContext {\n        self.context\n    }\n    /// Mathematical descriptor tags at acceptance.\n    #[must_use]\n'),
        (487, 19421, "    /// Borrow of its original finite obligation and unchanged deadline metadata.\n    obligation: &'view Obligation,\n}\n\nimpl FixturePhaseProposal<'_> {\n    /// Existing cursor identity reserved for this issued operation.\n    #[must_use]\n    pub const fn ordinal(&self) -> NonZeroU64 {\n        self.issued.ordinal\n    }\n    /// Requested transient, never a delivery acknowledgement.\n    #[must_use]\n    pub fn transient_rate(&self) -> f64 {\n        self.issued.rates.transient_rate()\n    }\n    /// Numerical error of the transient proposal alone.\n    #[must_use]\n"),
    ],
}
HISTORICAL_SOURCE_DIGESTS = {
    "crates/time-core/src/reference_input/rate_delivery.rs": "5a6ed0953bf6e0be88338c52251ed594fcbaaf5cca530e1f541236eb3f8e042d",
    "crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs": "a0a6af42cba3f012545d05ad5949f8934c28bd4bd65e503f695cc7fdf150c23a",
    "crates/time-core/src/reference_input/rate_delivery/phase_plan.rs": "a9770f085170b8c9ae52a5957761f99173ee3ff02294df56cb1e8e0c8b888b48",
    "crates/time-core/src/regression/chart/roundoff/frequency_prior.rs": "c29c5461fc90820de0aef29503174a64b70dc18aca131b8592b247e123ad247b",
}
HISTORICAL_CHANGED_DIGESTS = {
    ("crates/time-core/src/reference_input/rate_delivery.rs", 602): "c9790db06dd14bd2409e339ae202bae11e51f38506437e7f987a74ac072f3381",
    ("crates/time-core/src/reference_input/rate_delivery.rs", 607): "bd80e359086a537ae652178540578f146caf255d0cfaa7acd8018e33b1973837",
    ("crates/time-core/src/reference_input/rate_delivery.rs", 458): "8d8bbc09d0612b20712bda840de6dc0d4f0b4a09608624a519b6e047c9cc20a1",
    ("crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs", 743): "92eb3ad3f67bfb40754f78780cba36576e4162473502a71a6060450b7c440cb9",
    ("crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs", 811): "6faae4ce7b6695e9d32a35323caca83d278747932be5680cf08f12b14a3ff7a5",
    ("crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs", 474): "669650ef337acf934c5ca71bc4ce069be7535d6a32df30a5e2c4922aec542daa",
    ("crates/time-core/src/reference_input/rate_delivery/phase_plan.rs", 417): "c1bf2936a5841f59736a1977f747a73e1dad3defcdb664002d02322ecf1d1695",
    ("crates/time-core/src/reference_input/rate_delivery/phase_plan.rs", 495): "2255974b6312cd40c668f3fd2f71f1c8a27645bd87fd8226ee3f6475d29f6ae5",
}
REAL_SHA256 = hashlib.sha256
SYNTHETIC_CANDIDATE = {"synthetic_historical_parser_fixture": "no product admission"}


def historical_source(filename):
    """Build only immutable diagnostic windows, never a historical product copy."""
    if filename not in HISTORICAL_CONTEXTS:
        if filename not in HISTORICAL_SOURCE_DIGESTS:
            raise ValueError("Unknown synthetic historical source.")
        return b"// synthetic numerical-source identity; no executable Rust fixture\n"
    data, line_count = b"", 0
    for line, offset, text in HISTORICAL_CONTEXTS[filename]:
        missing_lines, missing_bytes = line - 1 - line_count, offset - len(data)
        if missing_lines <= 0 or missing_bytes < missing_lines:
            raise ValueError("Synthetic historical context overlaps its predecessor.")
        data += b" " * (missing_bytes - missing_lines) + b"\n" * missing_lines + text.encode()
        line_count = line - 1 + len(text.splitlines())
    return data


def historical_changed(filename, line):
    """Apply the exact reviewed operator/body payload to synthetic context only."""
    lines = historical_source(filename).decode().splitlines(keepends=True)
    if line in (602, 607, 743, 811):
        lines[line - 1] = lines[line - 1].replace("&&", "|| /* ~ changed by cargo-mutants ~ */", 1)
    elif line == 458:
        lines[457:461] = ["        Some(1.try_into().unwrap()) /* ~ changed by cargo-mutants ~ */\n"]
    else:
        lines[line - 1] = "        1.try_into().unwrap() /* ~ changed by cargo-mutants ~ */\n"
    return "".join(lines).encode()


@contextmanager
def historical_hashes(root, *, candidate=False):
    """Inject only exact synthetic identities for parser mechanics, never ROOT."""
    if root.resolve() == ROOT:
        raise ValueError("Synthetic identity adapters cannot wrap the current checkout.")
    bindings = {historical_source(name): digest for name, digest in HISTORICAL_SOURCE_DIGESTS.items()}
    bindings.update({historical_changed(name, line): digest
                     for (name, line), digest in HISTORICAL_CHANGED_DIGESTS.items()})
    allowed = {(root / prefix / name).resolve() for name in HISTORICAL_SOURCE_DIGESTS
               for prefix in (Path(), Path("source"))}
    real_file_hash = MODULE.COMMON.PRODUCT.sha256
    real_admit_product = MODULE.COMMON.PRODUCT.admit_product
    if candidate:
        marker = json.dumps(SYNTHETIC_CANDIDATE, sort_keys=True, separators=(",", ":")).encode()
        bindings[marker] = "3c8ae8f191cba09742259957d491bb362f8301a39c7506c1a029e02be4ae7e90"

    def exact_bytes(data=b"", *args, **kwargs):
        if type(data) is bytes and data in bindings:
            return SimpleNamespace(hexdigest=lambda: bindings[data])
        return REAL_SHA256(data, *args, **kwargs)

    def exact_file(path):
        if Path(path).resolve() in allowed:
            data = Path(path).read_bytes()
            if data in bindings:
                return bindings[data]
        return real_file_hash(path)

    def synthetic_candidate(path):
        if Path(path).resolve() == root.resolve():
            return {}, copy.deepcopy(SYNTHETIC_CANDIDATE)
        return real_admit_product(path)

    with patch.object(MODULE.hashlib, "sha256", side_effect=exact_bytes), \
         patch.object(MODULE.COMMON.PRODUCT, "sha256", side_effect=exact_file):
        if candidate:
            with patch.object(MODULE.COMMON.PRODUCT, "admit_product", side_effect=synthetic_candidate):
                yield
        else:
            yield


def historical_review_root(root):
    """Tiny synthetic admission mechanics; the maintained review stays unchanged."""
    for name in HISTORICAL_SOURCE_DIGESTS:
        path = root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(historical_source(name))
    proof = "tasks/phase-plan-analysis-equivalences.md"
    path = root / proof
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_bytes((ROOT / proof).read_bytes())
    return root


def parser_cause(work, outcome, raw, identity, payload, *, current=False):
    """Keep real current-checkout rejection outside every synthetic adapter."""
    if current:
        captured = {name: REAL_SHA256((ROOT / name).read_bytes()).hexdigest()
                    for name in MODULE.FREQUENCY_PRIOR_SUBJECTS.values()}
        return MODULE._frequency_log_cause(outcome, raw, ROOT, captured, policy(), mutant_diff=payload)
    with historical_hashes(work):
        return MODULE._frequency_log_cause(outcome, raw, work / "source", identity, policy(), mutant_diff=payload)


def policy():
    """Complete synthetic rosters are mechanics controls, never candidate evidence."""
    ordinary = sorted(MODULE.FREQUENCY_PRIOR_REQUIRED_TESTS | {"unrelated_control"})
    docs = []
    line = 10
    for filename, owner, negative, count in MODULE.FREQUENCY_PRIOR_REQUIRED_DOCTESTS:
        for _ in range(count):
            docs.append(f"{filename} - {owner} (line {line})" + (" - compile fail" if negative else ""))
            line += 1
    return {**copy.deepcopy(MODULE.FREQUENCY_PRIOR_FIXED), "source_file_count": 1,
            "candidate_sha256": "a" * 64, "inputs_sha256": {}, "scope": "Synthetic controls only.",
            "test_inventories": {"coverage": ordinary, "baseline": sorted([*ordinary, *docs])},
            "mutation_inventories": {name: {"count": 200, "sha256": "b" * 64}
                                     for name in MODULE.FREQUENCY_PRIOR_SUBJECTS},
            "mutation_equivalences": []}


def phases(summary):
    """Literal pinned native phase schema; no command is executed."""
    values = []
    for phase in (["Build"] if summary == "Unviable" else ["Build", "Test"]):
        argv = ["cargo", "test", *(["--no-run"] if phase == "Build" else []), "--verbose",
                "--package=time-core@0.0.0", "--message-format=json", "--frozen"]
        failed = summary == "Unviable" or (summary == "CaughtMutant" and phase == "Test")
        values.append({"phase": phase, "argv": argv, "duration": 0.01,
                       "process_status": {"Failure": 101} if failed else "Success"})
    return values


def campaign_fixture(directory):
    """Literal Session-shaped sandbox record, with no compiler/tool execution."""
    root = Path(directory)
    work, stable, vendor, tools = root / "leaf/work", root / "stable", root / "vendor", root / "leaf/tools"
    for path in (work, work / "source", work / "home", work / "tmp", work / "cargo-home", work / "target",
                 vendor, tools, *(stable / part for part in ("bin", "etc", "lib", "libexec", "share"))):
        path.mkdir(mode=0o700, parents=True, exist_ok=True)
    readonly = [stable / part for part in ("bin", "etc", "lib", "libexec", "share")]
    before = '[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory=' + json.dumps(str(vendor)) + '\n'
    after = before + "\n[profile.dev]\ndebug = 0\n[profile.test]\ndebug = 0\n"
    (work / "cargo-home/config.toml").write_text(after)
    receipt = {"analysis_admission": {"readonly_roots": [str(path) for path in readonly]},
        "mutation_debug_configuration": {"before_text": before, "after_text": after,
            "before_sha256": hashlib.sha256(before.encode()).hexdigest(),
            "after_sha256": hashlib.sha256(after.encode()).hexdigest(),
            "profile_settings": {"dev": {"debug": 0}, "test": {"debug": 0}},
            "other_profile_settings_changed": False}}
    inner = [str(tools / "cargo-mutants"), "mutants", "--package", "time-core", "--file",
        "crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs",
        "--test-workspace", "false", "--cap-lints", "false", "--jobs", "10",
        "--jobserver", "true", "--jobserver-tasks", "10", "--cargo-arg=--message-format=json",
        "--baseline", "run", "--build-timeout", "90", "--timeout", "60", "--output", "/work/mutation/frequency_state"]
    command = ["/usr/bin/bwrap", "--unshare-all", "--die-with-parent", "--new-session", "--cap-drop", "ALL",
        "--clearenv", "--bind", str(work), "/work", "--bind", str(work / "tmp"), "/tmp"]
    for public in ("/usr", "/bin", "/lib", "/lib64", "/etc/alternatives", "/etc/ld.so.cache"):
        if Path(public).exists():
            command.extend(["--ro-bind", public, public])
    for path in [*readonly, vendor, tools]:
        command.extend(["--ro-bind", str(path), str(path)])
    command.extend(["--ro-bind", str(work / "source"), "/work/source", "--ro-bind",
        str(work / "cargo-home/config.toml"), "/work/cargo-home/config.toml", "--proc", "/proc",
        "--dev", "/dev", "--chdir", "/work/source"])
    for name, value in [("PATH", f"{stable}/bin:{tools}:/usr/bin:/bin"), ("LANG", "C.UTF-8"),
        ("HOME", "/work/home"), ("TMPDIR", "/work/tmp"), ("TMP", "/work/tmp"), ("TEMP", "/work/tmp"),
        ("CARGO_HOME", "/work/cargo-home"), ("CARGO_NET_OFFLINE", "true"), ("CARGO_INCREMENTAL", "0"),
        ("CARGO_BUILD_JOBS", "1"), ("CARGO_TERM_COLOR", "never"), ("RUSTUP_AUTO_INSTALL", "0"),
        ("RUSTC", str(stable / "bin/rustc")), ("RUSTDOC", str(stable / "bin/rustdoc")), ("RUST_TEST_THREADS", "1")]:
        command.extend(["--setenv", name, value])
    command.extend(["--", "/usr/bin/prlimit", "--as=8589934592", "--cpu=600", "--nproc=512", "--nofile=256",
        "--fsize=268435456", "--", *inner])
    return work, receipt, {"command": command, "exit_code": 2, "failure": None, "output": "native observation"}, \
        {"stable": stable, "readonly_roots": readonly}, inner


def reviewed_profile_policy():
    """Historical identity metadata for synthetic parser mechanics, not evidence."""
    p = policy()
    path = "tasks/phase-plan-analysis-equivalences.md"
    proof_sha = hashlib.sha256((ROOT / path).read_bytes()).hexdigest()
    candidate = "3c8ae8f191cba09742259957d491bb362f8301a39c7506c1a029e02be4ae7e90"
    filename = MODULE.FREQUENCY_PRIOR_SUBJECTS["phase_plan"]
    mutant = {"file": filename, "package": "time-core", "genre": "BinaryOperator", "replacement": ">=",
        "name": f"{filename}:178:42: replace > with >= in profile_valid",
        "function": {"function_name": "profile_valid", "return_type": "-> bool",
            "span": {"start": {"line": 171, "column": 1}, "end": {"line": 179, "column": 2}}},
        "span": {"start": {"line": 178, "column": 42}, "end": {"line": 178, "column": 43}}}
    p.update(candidate_sha256=candidate, inputs_sha256={path: proof_sha}, mutation_equivalences=[{
        "id": "FREQUENCY-PROFILE-NORMAL-POSITIVE", "candidate_sha256": candidate,
        "source_sha256": "a9770f085170b8c9ae52a5957761f99173ee3ff02294df56cb1e8e0c8b888b48", "mutant": mutant,
        "requirement": "REQ-8/10/11/14/15; CHECK-7/8/10/11/14/25; AC-19",
        "rationale": "Normality excludes both signed zeros, subnormals, infinities and every NaN before the comparison; "
            "positive and negative normal binary64 values give identical >0 and >=0 results. Earlier guards, "
            "short-circuit order, validation, errors and effects are unchanged. This current-source argument "
            "grants zero kills; the exact mutant must still be an actual MissedMutant.",
        "proof": {"path": path, "sha256": proof_sha, "section": "FREQUENCY-PROFILE-NORMAL-POSITIVE",
            "reviewer": "/root; /root/series_checkpoint_design_review; /root/series_checkpoint_verification",
            "verdict": "independently-justified-equivalent"}}])
    return p


def reviewed_four_policy():
    """Four exact native catalog identities; source review grants no observed outcome."""
    p = reviewed_profile_policy()
    records = [{'id': 'FREQUENCY-GROWTH-NORMAL-POSITIVE',
      'source_sha256': 'a0a6af42cba3f012545d05ad5949f8934c28bd4bd65e503f695cc7fdf150c23a',
      'mutant': {'file': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs',
                 'function': {'function_name': 'profile_valid',
                              'return_type': '-> bool',
                              'span': {'end': {'column': 2, 'line': 183},
                                       'start': {'column': 1, 'line': 173}}},
                 'genre': 'BinaryOperator',
                 'name': 'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs:180:19: '
                         'replace > with >= in profile_valid',
                 'package': 'time-core',
                 'replacement': '>=',
                 'span': {'end': {'column': 20, 'line': 180}, 'start': {'column': 19, 'line': 180}}},
      'rationale': 'Normality rejects both signed zeros, subnormals, infinities and every NaN before '
                   'the growth comparison; positive and negative normal binary64 values give identical '
                   '>0 and >=0 results. All other profile fields, short-circuit order, validation, '
                   'errors and effects are unchanged. This current-source argument grants zero kills; '
                   'the exact mutant must still be an actual MissedMutant.'},
     {'id': 'FREQUENCY-ANCHOR-BOUND-ABS-SIGN',
      'source_sha256': 'c29c5461fc90820de0aef29503174a64b70dc18aca131b8592b247e123ad247b',
      'mutant': {'file': 'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs',
                 'function': {'function_name': 'FrequencyAnchor::rebase',
                              'return_type': '-> Result<Self, FixtureFrequencyNumericError>',
                              'span': {'end': {'column': 6, 'line': 632},
                                       'start': {'column': 5, 'line': 605}}},
                 'genre': 'UnaryOperator',
                 'name': 'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs:625:64: '
                         'delete - in FrequencyAnchor::rebase',
                 'package': 'time-core',
                 'replacement': '',
                 'span': {'end': {'column': 65, 'line': 625}, 'start': {'column': 64, 'line': 625}}},
      'rationale': 'Only the coefficient-bound argument changes sign; the phase subtraction and '
                   'earlier admissions are unchanged. The pinned parent coefficient_addition_error '
                   'applies abs to that operand before every bound operation, yielding identical '
                   'operands, error order, output fields and effects for every admitted anchor and raw '
                   'effect. Signed-zero and all failure paths are unchanged. This current-source '
                   'argument grants zero kills; the exact mutant must still be an actual '
                   'MissedMutant.'},
     {'id': 'FREQUENCY-INTERVAL-WHOLE-ZERO-PRODUCT',
      'source_sha256': 'c29c5461fc90820de0aef29503174a64b70dc18aca131b8592b247e123ad247b',
      'mutant': {'file': 'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs',
                 'function': {'function_name': 'Interval::multiply',
                              'return_type': '-> Result<Self, FixtureFrequencyNumericError>',
                              'span': {'end': {'column': 6, 'line': 1221},
                                       'start': {'column': 5, 'line': 1188}}},
                 'genre': 'BinaryOperator',
                 'name': 'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs:1196:27: '
                         'replace || with && in Interval::multiply',
                 'package': 'time-core',
                 'replacement': '&&',
                 'span': {'end': {'column': 29, 'line': 1196}, 'start': {'column': 27, 'line': 1196}}},
      'rationale': 'If exactly one finite ordered interval is whole zero, the unchanged inner endpoint '
                   'OR shortcut returns literal positive zero for all four pairs; reduction therefore '
                   'returns the same canonical ZERO. With both or neither whole zero, the paths are '
                   'identical. Outputs, refusals and effects agree over the full admitted interval '
                   'domain; maximum work remains four pairs, without a timing-equality claim. This '
                   'current-source argument grants zero kills; the exact mutant must still be an '
                   'actual MissedMutant.'}]
    template = p["mutation_equivalences"][0]
    for record in records:
        entry = copy.deepcopy(template)
        entry.update(record)
        entry["proof"]["section"] = entry["id"]
        p["mutation_equivalences"].append(entry)
    p["inputs_sha256"]["crates/time-core/src/regression/chart/roundoff.rs"] = (
        "4bea2d3fbd596ec5c28b7febd56d21efe42b19c7de2fdbd84f804d5451ff476f")
    return p


def phase_log(outcome, test="", build=None):
    """Exact argv banners and JSON completion records precede synthetic bodies."""
    chunks = []
    for row in outcome["phase_results"]:
        chunks.append("*** " + " ".join(row["argv"]) + "\n")
        chunks.append(build if row["phase"] == "Build" and build is not None else
                      '{"reason":"build-finished","success":true}\n')
        if row["phase"] == "Test": chunks.append(test)
    return "".join(chunks)


def failed_test():
    """One complete actual-format assertion section, never native evidence."""
    return ("running 1 test\ntest detector ... FAILED\n\nfailures:\n\n---- detector stdout ----\n\n"
            "thread 'detector' panicked at crates/time-core/tests/frequency_prior.rs:3:5:\n"
            "assertion `left == right` failed\n  left: 0\n right: 1\n\nfailures:\n    detector\n\n"
            "test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n"
            "error: test failed, to rerun pass `-p time-core --test frequency_prior`\n")


def captured_sources(work):
    """Small immutable source fixture supplies exact diagnostic locations only."""
    values = {"crates/time-core/src/lib.rs": b"pub mod subject { pub fn value() -> u8 { 1 } }\n",
              MODULE.FREQUENCY_PRIOR_SUBJECTS["frequency_state"]: b"pub fn value() -> u8 { 1 }\n",
              "crates/time-core/tests/frequency_prior.rs":
              b"#[test]\nfn detector() {\n    assert_eq!(time_core::subject::value(), 1);\n}\n"}
    for name, data in values.items():
        path = work / "source" / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_bytes(data)
    return {name: hashlib.sha256(data).hexdigest() for name, data in values.items()}


def diagnostic_record():
    """Independent E0308 record, owned by the actual-format Core worker target."""
    workspace = "/work/tmp/cargo-mutants-source-ABC123.tmp"
    return {"reason": "compiler-message", "package_id": "path+file://" + workspace + "/crates/time-core#0.0.0",
            "manifest_path": workspace + "/crates/time-core/Cargo.toml",
            "target": {"kind": ["lib"], "crate_types": ["lib"], "name": "time_core",
                       "src_path": workspace + "/crates/time-core/src/lib.rs", "edition": "2024",
                       "doc": True, "doctest": True, "test": True},
            "message": {"$message_type": "diagnostic", "level": "error", "message": "mismatched types",
                        "code": {"code": "E0308", "explanation": None}, "children": [], "rendered": None,
                        "spans": [{"file_name": MODULE.FREQUENCY_PRIOR_SUBJECTS["frequency_state"],
                                   "is_primary": True, "line_start": 1, "line_end": 1,
                                   "column_start": 1, "column_end": 2, "byte_start": 0, "byte_end": 1,
                                   "text": [{"text": "pub fn value() -> u8 { 1 }", "highlight_start": 1, "highlight_end": 2}]}]}}


def let_chain_fixture(work, line):
    """Reconstruct a historical diagnostic window; never execute or admit evidence."""
    identity = captured_sources(work)
    filename = "crates/time-core/src/reference_input/rate_delivery.rs"
    data = historical_source(filename)
    destination = work / "source" / filename
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_bytes(data)
    identity[filename] = HISTORICAL_SOURCE_DIGESTS[filename]
    before = data.decode().splitlines(keepends=True)
    after = before.copy()
    after[line - 1] = after[line - 1].replace("&&", "|| /* ~ changed by cargo-mutants ~ */", 1)
    diff = "".join(difflib.unified_diff(before, after, fromfile=filename,
        tofile="replace && with || in known_relationship", n=8))
    mutant = {"name": f"{filename}:{line}:9: replace && with || in known_relationship",
        "package": "time-core", "file": filename, "genre": "BinaryOperator", "replacement": "||",
        "span": {"start": {"line": line, "column": 9}, "end": {"line": line, "column": 11}},
        "function": {"function_name": "known_relationship", "return_type": "-> Result<(), FixtureRateDeliveryError>",
            "span": {"start": {"line": 582, "column": 1}, "end": {"line": 612, "column": 2}}}}
    byte_start = len("".join(after[:line - 1]).encode()) + 8
    record = diagnostic_record()
    record["message"].update(code=None, message="`||` operators are not supported in let chain conditions",
        spans=[{"file_name": filename, "is_primary": True, "line_start": line, "line_end": line,
                "column_start": 9, "column_end": 11, "byte_start": byte_start, "byte_end": byte_start + 2,
                "text": [{"highlight_start": 9, "highlight_end": 11, "text": after[line - 1].rstrip("\n")}]}])
    outcome = {"summary": "Unviable", "phase_results": phases("Unviable"), "scenario": {"Mutant": mutant}}
    return identity, outcome, record, diff


def retained_config(work):
    """Literal vendor-only mapping plus the sole admitted debug-zero derivation."""
    vendor = work / "vendor"
    vendor.mkdir()
    before = '[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory=' + json.dumps(str(vendor)) + "\n"
    after = before + "\n[profile.dev]\ndebug = 0\n[profile.test]\ndebug = 0\n"
    path = work / "cargo-home/config.toml"
    path.parent.mkdir()
    path.write_text(after)
    return {"mutation_debug_configuration": {"before_text": before, "before_sha256": hashlib.sha256(before.encode()).hexdigest(),
            "after_text": after, "after_sha256": hashlib.sha256(after.encode()).hexdigest(),
            "profile_settings": {"dev": {"debug": 0}, "test": {"debug": 0}}, "other_profile_settings_changed": False}}


class FrequencyPriorAnalysisTests(unittest.TestCase):
    """Require closed ownership, honest partial scopes and unchanged historical roles."""

    def test_synthetic_identity_adapter_is_exact_and_unknown_bytes_keep_real_hashes(self):
        filename = MODULE.FREQUENCY_PRIOR_SUBJECTS["frequency_state"]
        data = historical_source(filename)
        with tempfile.TemporaryDirectory() as directory:
            work = Path(directory)
            source = work / "source" / filename
            source.parent.mkdir(parents=True)
            source.write_bytes(data)
            with historical_hashes(work):
                self.assertEqual(hashlib.sha256(data).hexdigest(), HISTORICAL_SOURCE_DIGESTS[filename])
                self.assertEqual(MODULE.COMMON.PRODUCT.sha256(source), HISTORICAL_SOURCE_DIGESTS[filename])
                for unknown in (b"unknown", data + b"// tampered\n"):
                    self.assertEqual(hashlib.sha256(unknown).hexdigest(), REAL_SHA256(unknown).hexdigest())
                    source.write_bytes(unknown)
                    self.assertEqual(MODULE.COMMON.PRODUCT.sha256(source), REAL_SHA256(unknown).hexdigest())
                foreign = {"unrelated": True}
                encoded = json.dumps(foreign, sort_keys=True, separators=(",", ":")).encode()
                self.assertEqual(MODULE.COMMON.canonical_sha256(foreign), REAL_SHA256(encoded).hexdigest())
            with historical_hashes(work, candidate=True):
                self.assertEqual(MODULE.COMMON.canonical_sha256(SYNTHETIC_CANDIDATE),
                                 "3c8ae8f191cba09742259957d491bb362f8301a39c7506c1a029e02be4ae7e90")
                tampered = {**SYNTHETIC_CANDIDATE, "tampered": True}
                encoded = json.dumps(tampered, sort_keys=True, separators=(",", ":")).encode()
                self.assertEqual(MODULE.COMMON.canonical_sha256(tampered), REAL_SHA256(encoded).hexdigest())
        with self.assertRaisesRegex(ValueError, "cannot wrap the current checkout"):
            with historical_hashes(ROOT):
                self.fail("The current checkout cannot enter an identity adapter.")
        current = (ROOT / filename).read_bytes()
        self.assertEqual(hashlib.sha256(current).hexdigest(), REAL_SHA256(current).hexdigest())

    def test_private_build_target_layout_is_explicit_and_closed(self):
        row = {"reason": "build-script-executed",
               "package_id": "registry+https://github.com/rust-lang/crates.io-index#quote@1.0.47",
               "linked_libs": [], "linked_paths": [], "cfgs": [], "env": [],
               "out_dir": "/work/target/debug/build/quote-0123456789abcdef/out"}
        raw = json.dumps(row) + "\n"
        with self.assertRaises(MODULE.Incomplete):
            MODULE.cargo_json_records(raw)
        self.assertEqual(MODULE.cargo_json_records(raw, private_target=True), ([row], ""))
        for path in ("/tmp/target/debug/build/quote-0123456789abcdef/out",
                     "/work/target/debug/build/../quote-0123456789abcdef/out",
                     "/work/target/debug/build/foreign-0123456789abcdef/out"):
            with self.subTest(path=path), self.assertRaises(MODULE.Incomplete):
                MODULE.cargo_json_records(json.dumps({**row, "out_dir": path}) + "\n",
                                          private_target=True)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.cargo_json_records(raw, private_target=1)

    def test_exact_scope_and_essential_behavior_registry(self):
        role = MODULE._frequency_prior_role()
        self.assertEqual(role.scope.subjects, MODULE.COMMON.FREQUENCY_PRIOR_SUBJECTS)
        self.assertEqual(role.coverage_tests, ("frequency_prior", "phase_plan", "rate_delivery"))
        self.assertEqual(len(role.scope.select("all")), 4)
        self.assertEqual(len(role.required_tests), 120)
        self.assertIn(
            "reference_input::rate_delivery::phase_plan::tests::construction_error_projection_preserves_frequency_causes_without_owner_state",
            role.required_tests,
        )
        self.assertEqual(sum(row[3] for row in role.required_doctests), 16)
        self.assertTrue(MODULE.PHASE_PLAN_REQUIRED_TESTS <= role.required_tests)
        self.assertEqual(role.fixed["external_consumer_limits"], {"legal_executions": 2, "negative_compilations": 18})
        self.assertEqual(role.fixed["maximum_listed_total"], 2048)
        self.assertEqual(role.fixed["mutation_jobs"], 10)
        self.assertEqual(role.fixed["mutation_debug_info"], 0)
        self.assertEqual(role.fixed["jobs"], 6)
        self.assertEqual(role.fixed["general_jobs"], 12)

    def test_actual_hardening_identities_cannot_be_omitted_from_either_roster(self):
        required = {
            'constructor_coordinate_refusals_keep_cursor_and_exact_cause',
            'foreign_target_cursor_refusals_keep_frequency_cause_and_retry',
            'reference_input::rate_delivery::frequency_prior::tests::basis_matches_requires_each_checked_geometry_component',
            'reference_input::rate_delivery::frequency_prior::tests::checked_domain_boundaries_and_query_refusal_order_are_exact',
            'reference_input::rate_delivery::frequency_prior::tests::checked_relationship_rejects_each_supplied_source_coordinate',
            'reference_input::rate_delivery::frequency_prior::tests::disabled_latches_are_noops_and_enabled_empty_state_retains_no_model',
            'reference_input::rate_delivery::frequency_prior::tests::first_hard_retirement_survives_later_reasons_and_pending_queries',
            'reference_input::rate_delivery::frequency_prior::tests::rebase_checks_each_descriptor_relationship_and_preserves_whole_history',
            'reference_input::rate_delivery::frequency_prior::tests::rebase_missing_or_failed_descriptors_preserve_the_complete_record',
            'reference_input::rate_delivery::frequency_prior::tests::rebase_projected_epoch_support_has_both_inclusive_neighbors',
            'reference_input::rate_delivery::frequency_prior::tests::unknown_component_latch_preserves_genuine_history_and_block_precedence',
            'reference_input::rate_delivery::phase_plan::tests::actual_unknown_reception_latches_frequency_before_private_projection',
            'regression::chart::roundoff::frequency_prior::tests::absolute_value_preserves_dominant_cross_zero_magnitude',
            'regression::chart::roundoff::frequency_prior::tests::around_intervals_enclose_independent_dyadic_endpoints',
            'regression::chart::roundoff::frequency_prior::tests::gain_interval_zero_support_preserves_singular_and_exact_cases',
            'regression::chart::roundoff::frequency_prior::tests::multiplication_preserves_exact_zero_endpoints',
            'regression::chart::roundoff::frequency_prior::tests::whole_zero_products_keep_canonical_bits_and_nonzero_neighbors',
        }
        role = MODULE._frequency_prior_role()
        self.assertTrue(required <= role.required_tests)
        self.assertEqual(len(MODULE.FREQUENCY_PRIOR_BOUND_INPUTS), 58)
        for name in required:
            changed = policy()
            for lane in ("baseline", "coverage"):
                changed["test_inventories"][lane].remove(name)
            with self.subTest(name=name), self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_test_inventories(changed, role)

    def test_loader_closure_and_new_inputs_are_unique_and_complete(self):
        inputs = MODULE.FREQUENCY_PRIOR_BOUND_INPUTS
        self.assertEqual(len(inputs), len(set(inputs)))
        self.assertTrue(set(MODULE.PHASE_PLAN_HELPER_INPUTS) <= set(inputs))
        self.assertTrue(set(MODULE.FREQUENCY_PRIOR_SUBJECTS.values()) <= set(inputs))
        self.assertTrue({"scripts/analysis-controls.py", "scripts/core-frequency-contracts.py",
                         "scripts/tests/test_core_frequency_contracts.py",
                         "scripts/tests/test_frequency_prior_analysis.py",
                         "crates/time-core/tests/frequency_prior.rs",
                         "crates/time-core/tests/private/frequency_prior_numeric.rs",
                         "crates/time-core/tests/private/frequency_prior_state.rs"} <= set(inputs))

    def test_historical_roles_and_cli_dispatch_are_isolated(self):
        old = copy.deepcopy((MODULE.PHASE_PLAN_FIXED, MODULE.CONDITIONED_WINDOW_FIXED,
                             MODULE.CONDITIONED_FIT_FIXED, MODULE.SUBJECTS, MODULE.FIXED))
        with patch.object(MODULE, "main_frequency_prior", return_value=0) as selected, \
             patch.object(MODULE, "main_phase_plan") as legacy:
            self.assertEqual(MODULE.main(["--subject", "frequency_prior", "--lane", "enumerate",
                "--tool-admission-profile", "stable-core", "--output-dir", "/owned/output"]), 0)
            selected.assert_called_once()
            legacy.assert_not_called()
        self.assertEqual(old, (MODULE.PHASE_PLAN_FIXED, MODULE.CONDITIONED_WINDOW_FIXED,
                              MODULE.CONDITIONED_FIT_FIXED, MODULE.SUBJECTS, MODULE.FIXED))
        self.assertEqual(MODULE.PHASE_PLAN_FIXED["tool_admission_profile"], "full")
        self.assertEqual(MODULE.CONDITIONED_WINDOW_FIXED["mutation_jobs"], 8)

    def test_pending_policy_stops_before_any_admission_or_process(self):
        pending = policy()
        pending.update(candidate_sha256=None, source_file_count=None, inputs_sha256={})
        with patch.object(MODULE, "inventory_document", return_value=pending), \
             patch.object(MODULE.PRODUCT, "admit_product") as product, \
             patch.object(MODULE.COMMON, "Session") as session, \
             patch("subprocess.Popen") as process:
            with self.assertRaises(MODULE.Incomplete):
                MODULE.read_frequency_prior_policy(ROOT)
            product.assert_not_called()
            session.assert_not_called()
            process.assert_not_called()

    def test_stable_profile_is_exact_and_cannot_borrow_an_old_scope(self):
        scope = MODULE.FREQUENCY_PRIOR_SCOPE
        good = policy()
        self.assertEqual(MODULE.COMMON.admission_profile(
            SimpleNamespace(tool_admission_profile="stable-core", lane="contracts"), scope, good), "stable-core")
        for key, changed in (("mutation_jobs", 8), ("mutation_process_limit", 128),
                             ("coverage_minimum_line_percent", 89), ("mutation_minimum_viable_killed_percent", 94),
                             ("tool_admission_profile", "full"), ("maximum_mutants", 513),
                             ("mutation_debug_info", False), ("mutation_equivalences", [{}])):
            altered = copy.deepcopy(good)
            altered[key] = changed
            with self.subTest(key=key), self.assertRaises(MODULE.Incomplete):
                MODULE.COMMON.require_frequency_prior_role(scope, altered)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.admission_profile(SimpleNamespace(tool_admission_profile="full", lane="mutation"), scope, good)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.COMMON.require_frequency_prior_role(MODULE.PHASE_PLAN_SCOPE, good)

    def test_single_historical_profile_review_has_exact_scope_and_synthetic_custody(self):
        p = reviewed_profile_policy()
        self.assertTrue(MODULE.COMMON.frequency_prior_equivalences_valid(policy()))
        self.assertTrue(MODULE.COMMON.frequency_prior_equivalences_valid(p))
        MODULE.COMMON.require_frequency_prior_role(MODULE.FREQUENCY_PRIOR_SCOPE, p)
        with tempfile.TemporaryDirectory() as directory:
            root = historical_review_root(Path(directory))
            with historical_hashes(root, candidate=True), patch("subprocess.Popen", side_effect=AssertionError("Parser mechanics forbid execution")):
                self.assertEqual(MODULE.COMMON.admit_mutation_equivalences(root, MODULE.FREQUENCY_PRIOR_SCOPE, p),
                    p["mutation_equivalences"])
        with self.assertRaisesRegex(MODULE.Incomplete, "source or complete candidate differs"):
            MODULE.COMMON.admit_mutation_equivalences(ROOT, MODULE.FREQUENCY_PRIOR_SCOPE, p)
        for fault in ("duplicate", "historical", "descriptor", "source", "candidate", "policy_candidate",
                      "proof_section", "proof_path", "proof_hash", "reviewer", "rationale"):
            changed = copy.deepcopy(p)
            entry = changed["mutation_equivalences"][0]
            if fault == "duplicate": changed["mutation_equivalences"] *= 2
            elif fault == "historical": entry["id"] = "PHASE-PROFILE-NORMAL-POSITIVE"
            elif fault == "descriptor": entry["mutant"]["replacement"] = "=="
            elif fault == "source": entry["source_sha256"] = "a" * 64
            elif fault == "candidate": entry["candidate_sha256"] = "a" * 64
            elif fault == "policy_candidate": changed["candidate_sha256"] = "a" * 64
            elif fault == "proof_section": entry["proof"]["section"] = "PHASE-PROFILE-NORMAL-POSITIVE"
            elif fault == "proof_path": entry["proof"]["path"] = "tasks/s1.3-source-selection-readiness.md"
            elif fault == "proof_hash": entry["proof"]["sha256"] = "a" * 64
            elif fault == "reviewer": entry["proof"]["reviewer"] = "unreviewed"
            else: entry["rationale"] += " presumed miss"
            with self.subTest(fault=fault):
                self.assertFalse(MODULE.COMMON.frequency_prior_equivalences_valid(changed))
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.COMMON.require_frequency_prior_role(MODULE.FREQUENCY_PRIOR_SCOPE, changed)

    def test_four_historical_reviews_require_unique_exact_cases_and_anchor_parent(self):
        p = reviewed_four_policy()
        entries = p["mutation_equivalences"]
        self.assertEqual(len(entries), 4)
        for mask in range(16):
            selected = copy.deepcopy(p)
            selected["mutation_equivalences"] = [entry for index, entry in enumerate(entries) if mask & (1 << index)]
            with self.subTest(subset=mask):
                self.assertTrue(MODULE.COMMON.frequency_prior_equivalences_valid(selected))
                MODULE.COMMON.require_frequency_prior_role(MODULE.FREQUENCY_PRIOR_SCOPE, selected)
        with tempfile.TemporaryDirectory() as directory:
            root = historical_review_root(Path(directory))
            with historical_hashes(root, candidate=True), patch("subprocess.Popen", side_effect=AssertionError("Parser mechanics must not execute commands")):
                self.assertEqual(MODULE.COMMON.admit_mutation_equivalences(root, MODULE.FREQUENCY_PRIOR_SCOPE, p), entries)
        with self.assertRaisesRegex(MODULE.Incomplete, "source or complete candidate differs"):
            MODULE.COMMON.admit_mutation_equivalences(ROOT, MODULE.FREQUENCY_PRIOR_SCOPE, p)
        for index in range(4):
            for fault in ("duplicate", "unknown", "source", "descriptor", "candidate", "proof", "rationale"):
                changed = copy.deepcopy(p)
                entry = changed["mutation_equivalences"][index]
                if fault == "duplicate": changed["mutation_equivalences"] = [entry, copy.deepcopy(entry)]
                elif fault == "unknown": entry["id"] = "UNREVIEWED-FREQUENCY-FAULT"
                elif fault == "source": entry["source_sha256"] = "a" * 64
                elif fault == "descriptor": entry["mutant"]["replacement"] += " foreign"
                elif fault == "candidate": entry["candidate_sha256"] = "a" * 64
                elif fault == "proof": entry["proof"]["sha256"] = "a" * 64
                else: entry["rationale"] += " presumed exclusion"
                with self.subTest(index=index, fault=fault):
                    self.assertFalse(MODULE.COMMON.frequency_prior_equivalences_valid(changed))
        for parent in (None, "a" * 64):
            changed = copy.deepcopy(p)
            path = "crates/time-core/src/regression/chart/roundoff.rs"
            if parent is None: changed["inputs_sha256"].pop(path)
            else: changed["inputs_sha256"][path] = parent
            self.assertFalse(MODULE.COMMON.frequency_prior_equivalences_valid(changed))
        changed = copy.deepcopy(p)
        path = "tasks/phase-plan-analysis-equivalences.md"
        changed["inputs_sha256"][path] = "a" * 64
        for entry in changed["mutation_equivalences"]:
            entry["proof"]["sha256"] = "a" * 64
        self.assertTrue(MODULE.COMMON.frequency_prior_equivalences_valid(changed))
        with tempfile.TemporaryDirectory() as directory:
            root = historical_review_root(Path(directory))
            with historical_hashes(root, candidate=True), \
                 patch("subprocess.Popen", side_effect=AssertionError("Parser mechanics must not execute commands")), \
                 self.assertRaisesRegex(MODULE.Incomplete, "proof bytes or exact section differ"):
                MODULE.COMMON.admit_mutation_equivalences(root, MODULE.FREQUENCY_PRIOR_SCOPE, changed)

    def test_common_equivalence_requires_actual_miss_without_filtering_or_kill_credit(self):
        for entry in reviewed_four_policy()["mutation_equivalences"]:
            equivalent = entry["mutant"]
            neighbor = copy.deepcopy(equivalent)
            neighbor.update(replacement="<", name=equivalent["name"] + " neighboring control")
            expected = {value["name"]: value for value in (equivalent, neighbor)}
            def native(summary, equivalent=equivalent, neighbor=neighbor):
                missed = int(summary == "MissedMutant")
                return {"cargo_mutants_version": "27.1.0", "total_mutants": 2,
                    "caught": 1 + int(summary == "CaughtMutant"), "missed": missed,
                    "unviable": int(summary == "Unviable"), "timeout": 0, "success": 0,
                    "outcomes": [{"scenario": "Baseline", "summary": "Success", "phase_results": phases("Success")},
                        {"scenario": {"Mutant": equivalent}, "summary": summary, "phase_results": phases(summary)},
                        {"scenario": {"Mutant": neighbor}, "summary": "CaughtMutant", "phase_results": phases("CaughtMutant")} ]}
            with self.subTest(identifier=entry["id"]):
                result = MODULE.COMMON.mutation_counts(MODULE.FREQUENCY_PRIOR_SCOPE, native("MissedMutant"), expected,
                    equivalent["file"], native_exit_code=2, equivalences=[entry])
                self.assertEqual((result["generated"], result["killed"], result["survived"], result["excluded"]), (2, 1, 1, 1))
                self.assertEqual((result["raw_denominator"], result["adjusted_denominator"]), (2, 1))
                self.assertTrue(result["gate_passed"])
                for summary in ("CaughtMutant", "Unviable"):
                    with self.subTest(summary=summary), self.assertRaises(MODULE.Incomplete):
                        MODULE.COMMON.mutation_counts(MODULE.FREQUENCY_PRIOR_SCOPE, native(summary), expected,
                            equivalent["file"], native_exit_code=0, equivalences=[entry])

    def test_inventory_discovery_has_no_fabricated_bindings_or_campaign_credit(self):
        p = policy()
        p["mutation_inventories"] = {}
        self.assertEqual(MODULE.frequency_inventory_bindings(p), {})
        with self.assertRaises(MODULE.Incomplete):
            MODULE.frequency_inventory_bindings(p, required=True)
        report = {}
        actual = SimpleNamespace(inventories={"frequency_state": {"fault": {"name": "fault", "diff": "raw"}}},
                                 total=1, campaign_budget_exceeded=False)
        with patch.object(MODULE.COMMON, "enumerate_mutation_inventories", return_value=actual) as discover:
            MODULE._enumerate_conditioned(SimpleNamespace(tools=Path("/tools"), run=object()), p, report,
                                          MODULE._frequency_prior_role())
        self.assertEqual(discover.call_args.kwargs["maximum_per_file"], 512)
        self.assertEqual(discover.call_args.kwargs["maximum_total"], 2048)
        self.assertFalse(report["campaign_admission"])
        self.assertFalse(report["qualification_passed"])
        self.assertNotIn("mutation", report)

    def test_whole_owner_cohorts_require_complete_union_and_keep_all_owners(self):
        p = policy()
        self.assertEqual(len(MODULE.frequency_inventory_bindings(p, required=True)), 4)
        with self.assertRaises(MODULE.Incomplete):
            MODULE.frequency_selected_owners(p, None)
        selected = {}
        for owner, filename in MODULE.FREQUENCY_PRIOR_SUBJECTS.items():
            choice = MODULE.frequency_selected_owners(p, owner)
            self.assertEqual(choice, {owner: filename})
            selected.update(choice)
        self.assertEqual(selected, MODULE.FREQUENCY_PRIOR_SUBJECTS)
        for bad in ("unknown", "frequency_state+frequency_numeric", [], True):
            with self.subTest(bad=bad), self.assertRaises(MODULE.Incomplete):
                MODULE.frequency_selected_owners(p, bad)
        for fault in ("partial", "extra", "zero", "overflow", "bool", "hash"):
            altered = copy.deepcopy(p)
            rows = altered["mutation_inventories"]
            first = next(iter(rows))
            if fault == "partial": rows.pop(first)
            elif fault == "extra": rows["foreign"] = rows[first]
            elif fault == "zero": rows[first]["count"] = 0
            elif fault == "overflow": rows[first]["count"] = 513
            elif fault == "bool": rows[first]["count"] = True
            else: rows[first]["sha256"] = "unbound"
            with self.subTest(fault=fault), self.assertRaises(MODULE.Incomplete):
                MODULE.frequency_inventory_bindings(altered, required=True)
        p["mutation_inventories"] = {name: {"count": 128, "sha256": "b" * 64}
                                     for name in MODULE.FREQUENCY_PRIOR_SUBJECTS}
        self.assertEqual(MODULE.frequency_selected_owners(p, None), MODULE.FREQUENCY_PRIOR_SUBJECTS)

    def test_exact_rosters_keep_all_ordinary_privacy_cases_and_reject_failed_logs(self):
        p = policy()
        role = MODULE._frequency_prior_role()
        MODULE._conditioned_test_inventories(p, role)
        for lane in ("coverage", "baseline"):
            changed = copy.deepcopy(p)
            changed["test_inventories"][lane].pop(0)
            with self.subTest(lane=lane), self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_test_inventories(changed, role)
        names = p["test_inventories"]["coverage"]
        text = f"running {len(names)} tests\n" + "".join(f"test {name} ... ok\n" for name in names)
        text += f"test result: ok. {len(names)} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n"
        self.assertEqual(MODULE._conditioned_test_execution(text, p, role, doctests=False)["tests_executed"], len(names))
        for bad in (text.replace(" ... ok", " ... FAILED", 1), text.replace("0 filtered out", "1 filtered out"),
                    text + "thread 'foreign' panicked at foreign.rs:1:1:\nforeign\n"):
            with self.assertRaises(MODULE.Incomplete):
                MODULE._conditioned_test_execution(bad, p, role, doctests=False)

    def test_union_metadata_rejects_foreign_source_and_partial_or_failed_receipts(self):
        p = policy()
        admitted = {"inventories": {"stable": "c" * 64}, "policy_sha256": "d" * 64}
        identity = {"source": "e" * 64}
        for owner in MODULE.FREQUENCY_PRIOR_SUBJECTS:
            r = {"cohort_id": owner, "status": "passed", "selected_lane": "mutation", "cohort_complete": True,
                 "subjects": {owner: MODULE.FREQUENCY_PRIOR_SUBJECTS[owner]},
                 "complete_subjects": MODULE.FREQUENCY_PRIOR_SUBJECTS,
                 "candidate_sha256": p["candidate_sha256"], "source_sha256": identity,
                 "tool_admission_profile": "stable-core", "all_reviewed_inventory_bindings": p["mutation_inventories"],
                 "analysis_policy_sha256": admitted["policy_sha256"],
                 "analysis_admission": {"profile": "stable-core", "inventories": admitted["inventories"]},
                 "analysis_admission_after": {"profile": "stable-core", "inventories": admitted["inventories"], "unchanged": True}}
            self.assertEqual(MODULE.frequency_cohort_receipt_owner(r, p, identity, admitted), owner)
            for key, value in (("status", "incomplete"), ("cohort_complete", False), ("cohort_id", "foreign"),
                               ("source_sha256", {"source": "0" * 64}), ("candidate_sha256", "0" * 64),
                               ("complete_subjects", {}), ("analysis_admission_after", {"unchanged": False}),
                               ("analysis_policy_sha256", "0" * 64)):
                bad = copy.deepcopy(r)
                bad[key] = value
                with self.subTest(owner=owner, key=key), self.assertRaises(MODULE.Incomplete):
                    MODULE.frequency_cohort_receipt_owner(bad, p, identity, admitted)

    def test_union_missing_and_duplicate_paths_start_no_process_or_output(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / "receipt.json"
            path.write_text("{}")
            for paths in ([path] * 4, [path] * 3):
                args = SimpleNamespace(cohort_id=None, cohort_receipt=paths)
                with patch("subprocess.Popen") as process, self.assertRaises(MODULE.Incomplete):
                    MODULE.frequency_cohort_union(args, {}, policy(), MODULE._frequency_prior_role())
                process.assert_not_called()

    def test_actual_source_helper_omission_and_drift_stop_at_protected_binding(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            helper = "scripts/analysis-controls.py"
            bound = (helper, "scripts/core-frequency-contracts.py", *MODULE.FREQUENCY_PRIOR_SUBJECTS.values())
            originals = {name: b"// synthetic immutable owner\n" if name.endswith(".rs") else b"# helper\n" for name in bound}
            for name, data in originals.items():
                target = root / name
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(data)
            (root / "policy").mkdir()
            (root / "policy/verification-manifest.toml").write_text(
                "critical_files = " + json.dumps(list(MODULE.FREQUENCY_PRIOR_SUBJECTS.values()))
                + '\nsecurity_modules = [{package = "time-core", files = '
                + json.dumps(list(MODULE.FREQUENCY_PRIOR_SUBJECTS.values())) + "}]\n")
            identity = {"candidate.rs": "c" * 64}
            p = policy()
            p.update(candidate_sha256=MODULE.COMMON.canonical_sha256(identity),
                     inputs_sha256={name: hashlib.sha256(data).hexdigest() for name, data in originals.items()})
            target = root / MODULE.FREQUENCY_PRIOR_POLICY
            with patch.object(MODULE, "FREQUENCY_PRIOR_BOUND_INPUTS", bound), \
                 patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, identity)), \
                 patch("subprocess.Popen") as process:
                target.write_text(json.dumps(p))
                self.assertEqual(MODULE.read_frequency_prior_policy(root), p)
                for fault in ("omission", "drift"):
                    altered = copy.deepcopy(p)
                    if fault == "omission":
                        altered["inputs_sha256"].pop(helper)
                    else:
                        (root / helper).write_text("# changed helper\n")
                    target.write_text(json.dumps(altered))
                    with self.subTest(fault=fault), self.assertRaisesRegex(MODULE.Incomplete, "protected source/helper bindings changed"):
                        MODULE.read_frequency_prior_policy(root)
                process.assert_not_called()

    def test_low_disk_stops_before_session_with_unchanged_five_gib_floor(self):
        args = SimpleNamespace(subject="frequency_prior", tool_admission_profile="stable-core", lane="mutation",
            inventory_admission=None, cohort_plan=None, cohort_receipt=[], cohort_id="frequency_state",
            output_dir=Path("/owned/output"))
        with patch.object(MODULE, "read_frequency_prior_policy", return_value=policy()), \
             patch.object(MODULE.COMMON, "_require_private_output"), \
             patch.object(MODULE.shutil, "disk_usage", return_value=SimpleNamespace(free=5 * 1024 ** 3 - 1)), \
             patch.object(MODULE.COMMON, "Session") as session:
            with self.assertRaisesRegex(MODULE.Incomplete, "ten-worker.*five GiB"):
                MODULE.execute_frequency_prior(args, {})
            session.assert_not_called()

    def test_complete_union_reuses_all_raw_leaf_validators_and_rejects_duplicate_owner(self):
        # Plumbing only: native phase/cause validators are mocked, not claimed executed.
        p, role = policy(), MODULE._frequency_prior_role()
        admitted = {"inventories": {"stable": "c" * 64}, "policy_sha256": "d" * 64}
        identity = {name: "b" * 64 for name in ("Cargo.toml", role.scope.policy, *role.bound_inputs)}
        result, baseline = {"gate_passed": True}, {"tests_executed": 3}
        payloads = {name: {"name": name + "_fault", "file": filename, "package": "time-core", "diff": "synthetic"}
                    for name, filename in MODULE.FREQUENCY_PRIOR_SUBJECTS.items()}
        p["mutation_inventories"] = {name: {"count": 1, "sha256": MODULE.COMMON.canonical_sha256({
            item["name"]: {key: value for key, value in item.items() if key != "diff"}})}
            for name, item in payloads.items()}
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            documents, paths = {}, []
            for owner, filename in MODULE.FREQUENCY_PRIOR_SUBJECTS.items():
                parent = root / owner
                parent.mkdir()
                path = parent / "verification.json"
                path.write_text("{}")
                paths.append(path)
                documents[str(path)] = {"cohort_id": owner, "status": "passed", "selected_lane": "mutation",
                    "cohort_complete": True, "subjects": {owner: filename},
                    "complete_subjects": MODULE.FREQUENCY_PRIOR_SUBJECTS,
                    "candidate_sha256": p["candidate_sha256"], "source_sha256": identity,
                    "tool_admission_profile": "stable-core", "all_reviewed_inventory_bindings": p["mutation_inventories"],
                    "analysis_policy_sha256": admitted["policy_sha256"],
                    "analysis_admission": {"profile": "stable-core", "inventories": admitted["inventories"]},
                    "analysis_admission_after": {"profile": "stable-core", "inventories": admitted["inventories"], "unchanged": True},
                    "owned_output": str(parent), "work": str(parent / "work"),
                    "tools_sha256": {"cargo-mutants": "b" * 64},
                    "mutation_enumeration": {owner: {"raw_output": json.dumps([payloads[owner]])}},
                    "lanes": [{"lane": "mutation-campaign:" + owner, "command": ["synthetic"], "exit_code": 0}],
                    "mutation": {owner: {**result, "baseline_tests_executed": 3}},
                    "baseline_executions": {owner: baseline}, "frequency_artifacts": {owner: {"synthetic": True}}}
            def read(path):
                if path.name != "outcomes.json":
                    MODULE.COMMON.read_analysis_bytes(path, maximum=16 * 1024 * 1024)
                    return documents[str(path)]
                return {"synthetic_outcomes": True}
            args = SimpleNamespace(cohort_id=None, cohort_receipt=paths, output_dir=root / "union")
            with patch.object(MODULE.PRODUCT, "admit_product", return_value=({}, {"Cargo.toml": "b" * 64})), \
                 patch.object(MODULE.PRODUCT, "sha256", return_value="b" * 64), \
                 patch.object(MODULE.COMMON.ANALYSIS, "admit", return_value=admitted), \
                 patch.object(MODULE.COMMON, "read_analysis_json", side_effect=read), \
                 patch.object(MODULE.COMMON, "require_unchanged") as captured, \
                 patch.object(MODULE.COMMON, "tool_path", return_value=Path("/tools/cargo-mutants")), \
                 patch.object(MODULE, "frequency_campaign_result") as envelopes, \
                 patch.object(MODULE.COMMON, "mutation_counts", return_value=result) as counts, \
                 patch.object(MODULE, "_conditioned_baseline", return_value=baseline) as baselines, \
                 patch.object(MODULE, "frequency_artifact_manifest", return_value={"synthetic": True}) as artifacts, \
                 patch.object(MODULE.COMMON, "_require_private_output"), patch("subprocess.Popen") as process:
                alias = root / "linked-receipt.json"
                alias.symlink_to(paths[0])
                linked = SimpleNamespace(cohort_id=None, cohort_receipt=[alias, *paths[1:]], output_dir=root / "linked-union")
                with self.assertRaises(MODULE.Incomplete):
                    MODULE.frequency_cohort_union(linked, {}, p, role)
                report = {}
                MODULE.frequency_cohort_union(args, report, p, role)
                self.assertTrue(report["mutation_union_passed"])
                self.assertFalse(report["full_role_qualification"])
                self.assertEqual(set(report["mutation_cohort_union"]), set(MODULE.FREQUENCY_PRIOR_SUBJECTS))
                for validator in (captured, envelopes, counts, baselines, artifacts):
                    self.assertEqual(validator.call_count, 4)
                first, second = paths[:2]
                bad = copy.deepcopy(documents[str(second)])
                bad["cohort_id"] = documents[str(first)]["cohort_id"]
                bad["subjects"] = documents[str(first)]["subjects"]
                documents[str(second)] = bad
                with self.assertRaisesRegex(MODULE.Incomplete, "duplicates an owner"):
                    MODULE.frequency_cohort_union(args, {}, p, role)
                process.assert_not_called()

    def test_literal_union_campaign_accepts_both_exact_argv_layers(self):
        with tempfile.TemporaryDirectory() as directory:
            work, receipt, campaign, admitted, inner = campaign_fixture(directory)
            role, p = MODULE._frequency_prior_role(), policy()
            with patch.object(MODULE.PRODUCT, "admit_vendor", return_value={"fixture": "b" * 64}) as vendor, \
                 patch.object(MODULE.COMMON.RUNTIME, "_helpers", return_value={
                     "bwrap": "/usr/bin/bwrap", "prlimit": "/usr/bin/prlimit"}), \
                 patch("subprocess.Popen", side_effect=AssertionError("native execution forbidden")) as process:
                # The original union call rejects this genuine-shaped outer argv:
                # its unchanged validator accepts only the separate inner argv.
                with self.assertRaisesRegex(MODULE.Incomplete, "exact admitted campaign invocation"):
                    MODULE.COMMON.mutation_campaign_result(role.scope, work.parent / "tools", "frequency_state",
                        campaign["command"], campaign, expected_jobs=10, expected_message_format="json", policy=p)
                self.assertIs(MODULE.COMMON.mutation_campaign_result(role.scope, work.parent / "tools", "frequency_state",
                    inner, campaign, expected_jobs=10, expected_message_format="json", policy=p), campaign)
                self.assertIs(MODULE.frequency_campaign_result(work, receipt, campaign, "frequency_state",
                    role, p, admitted, {"synthetic_dependencies": True}), campaign)
                self.assertEqual(vendor.call_args.args, (work.parent.parent / "vendor", {"synthetic_dependencies": True}))
                process.assert_not_called()

    def test_literal_union_campaign_rejects_outer_and_inner_substitution(self):
        with tempfile.TemporaryDirectory() as directory:
            work, receipt, campaign, admitted, inner = campaign_fixture(directory)
            command = campaign["command"]
            variants = {}
            for old, new in [("--unshare-all", "--share-net"), ("--clearenv", "--new-session"),
                ("/work/source", "/work/foreign"), ("--as=8589934592", "--as=17179869184"),
                ("--cpu=600", "--cpu=601"), ("--nproc=512", "--nproc=128"),
                ("--nofile=256", "--nofile=4096"), ("--fsize=268435456", "--fsize=536870912"),
                ("CARGO_NET_OFFLINE", "CARGO_HTTP_PROXY")]:
                changed = command.copy(); changed[changed.index(old)] = new; variants[old] = changed
            for key, value in [("RUST_TEST_THREADS", "10"), ("CARGO_BUILD_JOBS", "10"), ("RUSTC", "/usr/bin/rustc")]:
                changed = command.copy(); changed[changed.index(key) + 1] = value; variants[key] = changed
            first_mount = command.index("--ro-bind")
            variants["writable tool input"] = [*command[:first_mount], "--bind", *command[first_mount + 1:]]
            changed = command.copy(); changed[0], changed[1] = changed[1], changed[0]; variants["order"] = changed
            variants["arbitrary prefix"] = ["/usr/bin/env", *command]
            variants["target override"] = [*command[:command.index("--")], "--setenv", "CARGO_TARGET_DIR", "/work/target",
                *command[command.index("--"):]]
            for key, value in [("--file", "crates/time-core/src/lib.rs"), ("--jobs", "8"), ("--baseline", "skip")]:
                changed = command.copy(); changed[changed.index(key) + 1] = value; variants[key] = changed
            variants["test filter"] = [*command, "--", "one_test", "--", "--exact"]
            variants["inner without sandbox"] = inner
            with patch.object(MODULE.PRODUCT, "admit_vendor", return_value={"fixture": "b" * 64}), \
                 patch.object(MODULE.COMMON.RUNTIME, "_helpers", return_value={
                     "bwrap": "/usr/bin/bwrap", "prlimit": "/usr/bin/prlimit"}), \
                 patch("subprocess.Popen", side_effect=AssertionError("native execution forbidden")) as process:
                for label, changed in variants.items():
                    with self.subTest(label=label), self.assertRaisesRegex(MODULE.Incomplete, "exact admitted envelope"):
                        MODULE.frequency_campaign_result(work, receipt, {**campaign, "command": changed},
                            "frequency_state", MODULE._frequency_prior_role(), policy(), admitted, {})
                for changes in ({"exit_code": 1}, {"failure": "timeout"}, {"exit_code": False}):
                    with self.subTest(changes=changes), self.assertRaises(MODULE.Incomplete):
                        MODULE.frequency_campaign_result(work, receipt, {**campaign, **changes},
                            "frequency_state", MODULE._frequency_prior_role(), policy(), admitted, {})
                process.assert_not_called()

    def test_union_campaign_replay_preserves_missing_dirs_and_rejects_foreign_inputs(self):
        with tempfile.TemporaryDirectory() as directory:
            work, receipt, campaign, admitted, _ = campaign_fixture(directory)
            with patch.object(MODULE.PRODUCT, "admit_vendor", return_value={"fixture": "b" * 64}), \
                 patch.object(MODULE.COMMON.RUNTIME, "_helpers", return_value={
                     "bwrap": "/usr/bin/bwrap", "prlimit": "/usr/bin/prlimit"}), \
                 patch.object(MODULE.COMMON.RUNTIME, "sandbox_command", wraps=MODULE.COMMON.RUNTIME.sandbox_command) as builder, \
                 patch("subprocess.Popen", side_effect=AssertionError("native execution forbidden")) as process:
                for name in ("home", "tmp", "target"):
                    path = work / name; path.rmdir()
                    with self.subTest(missing=name), self.assertRaises(OSError):
                        MODULE.frequency_campaign_result(work, receipt, campaign, "frequency_state",
                            MODULE._frequency_prior_role(), policy(), admitted, {})
                    self.assertFalse(path.exists())
                    path.mkdir(mode=0o700)
                cargo_home, saved = work / "cargo-home", work / "retained-cargo-home"
                cargo_home.rename(saved)
                try:
                    with self.assertRaises(OSError):
                        MODULE.frequency_campaign_result(work, receipt, campaign, "frequency_state",
                            MODULE._frequency_prior_role(), policy(), admitted, {})
                    self.assertFalse(cargo_home.exists())
                finally:
                    saved.rename(cargo_home)
                builder.assert_not_called()
                bad = copy.deepcopy(receipt); bad["analysis_admission"]["readonly_roots"].reverse()
                with self.assertRaisesRegex(MODULE.Incomplete, "readonly roots"):
                    MODULE.frequency_campaign_result(work, bad, campaign, "frequency_state",
                        MODULE._frequency_prior_role(), policy(), admitted, {})
                (work / "cargo-home/config.toml").write_text("[profile.test]\ndebug = 2\n")
                with self.assertRaisesRegex(MODULE.Incomplete, "sealed config"):
                    MODULE.frequency_campaign_result(work, receipt, campaign, "frequency_state",
                        MODULE._frequency_prior_role(), policy(), admitted, {})
                builder.assert_not_called()
                process.assert_not_called()

    def test_literal_caught_requires_the_owned_assertion_and_complete_failed_sections(self):
        with tempfile.TemporaryDirectory() as directory:
            work = Path(directory)
            identity = captured_sources(work)
            p = policy()
            p["test_inventories"]["baseline"].append("detector")
            outcome = {"summary": "CaughtMutant", "phase_results": phases("CaughtMutant")}
            text = phase_log(outcome, failed_test())
            causes = MODULE._frequency_log_cause(outcome, text, work / "source", identity, p)
            self.assertEqual([(c["test"], c["source"], c["line"], c["column"], c["kind"]) for c in causes],
                [("detector", "crates/time-core/tests/frequency_prior.rs", 3, 5, "assert_eq!")])
            bad = [text + "thread 'foreign' panicked at foreign.rs:1:1:\nforeign\n",
                   text + "Error: unrelated harness failure\n",
                   text.replace("thread 'detector'", "thread 'foreign'"),
                   text.replace(".rs:3:5:", ".rs:3:4:"),
                   text.replace("assertion `left == right` failed", "attempt to add with overflow"),
                   text.replace("tests/frequency_prior.rs:3:5", "src/lib.rs:1:1"),
                   text.replace("--test frequency_prior`", "--test foreign`"),
                   text.replace("--test frequency_prior`", "--lib`"),
                   text + "error: test failed, to rerun pass `-p time-core --test frequency_prior`\n",
                   text.replace("    detector\n\ntest result", "    missing\n\ntest result")]
            for marker in ("resource temporarily unavailable", "out of memory",
                           "could not execute process: failed to spawn",
                           "error: could not compile `foreign`", "error[E0001]: foreign failure"):
                bad.extend((text.replace("thread 'detector'", marker + "\nthread 'detector'", 1),
                            text + "\n" + marker + "\n"))
            for value in bad:
                with self.subTest(raw=value), self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_log_cause(outcome, value, work / "source", identity, p)
            record = diagnostic_record()
            compiled_error = text.replace('{"reason":"build-finished","success":true}\n',
                                          json.dumps(record) + '\n{"reason":"build-finished","success":true}\n', 1)
            with self.assertRaises(MODULE.Incomplete):
                MODULE._frequency_log_cause(outcome, compiled_error, work / "source", identity, p)
            private = "crates/time-core/tests/private/frequency_prior_numeric.rs"
            data = (work / "source/crates/time-core/tests/frequency_prior.rs").read_bytes()
            (work / "source" / private).parent.mkdir()
            (work / "source" / private).write_bytes(data)
            identity[private] = hashlib.sha256(data).hexdigest()
            library = work / "source/crates/time-core/src/lib.rs"
            data = library.read_bytes() + b'#[path = "../tests/private/frequency_prior_numeric.rs"] mod tests;\n'
            library.write_bytes(data)
            identity["crates/time-core/src/lib.rs"] = hashlib.sha256(data).hexdigest()
            p["test_inventories"]["baseline"].append("tests::detector")
            native = text.replace("detector", "tests::detector").replace("tests/frequency_prior.rs:3:5", private.removeprefix("crates/time-core/") + ":3:5")
            native = native.replace("--test frequency_prior`", "--lib`")
            self.assertEqual(MODULE._frequency_log_cause(outcome, native, work / "source", identity, p)[0]["source"], private)
            with self.assertRaises(MODULE.Incomplete):
                MODULE._frequency_log_cause(outcome, native.replace("--lib`", "--test frequency_prior`"), work / "source", identity, p)

    def test_literal_unviable_requires_coded_owned_compiler_cause_without_infrastructure(self):
        with tempfile.TemporaryDirectory() as directory:
            work = Path(directory)
            identity = captured_sources(work)
            outcome = {"summary": "Unviable", "phase_results": phases("Unviable")}
            def raw(record):
                return phase_log(outcome, build=json.dumps(record) + '\n{"reason":"build-finished","success":false}\n')
            good = diagnostic_record()
            self.assertEqual(MODULE._frequency_log_cause(outcome, raw(good), work / "source", identity, policy())[0]["code"], "E0308")
            owned_footer = "error: could not compile `time-core` (lib) due to 1 previous error\n"
            self.assertEqual(MODULE._frequency_log_cause(outcome, raw(good) + owned_footer,
                work / "source", identity, policy())[0]["code"], "E0308")
            self.assertEqual(len(MODULE._frequency_log_cause(outcome, raw(good) + owned_footer
                + owned_footer.replace("(lib)", "(lib test)"), work / "source", identity, policy())), 1)
            for marker in ("error: could not compile `foreign` (lib) due to 1 previous error",
                           "error[E0001]: foreign failure", owned_footer * 2,
                           owned_footer.replace("(lib)", '(test "foreign")')):
                with self.subTest(marker=marker), self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_log_cause(outcome, raw(good) + marker,
                        work / "source", identity, policy())
            no_test = copy.deepcopy(good)
            no_test["target"]["test"] = False
            with self.assertRaises(MODULE.Incomplete):
                MODULE._frequency_log_cause(outcome, raw(no_test) + owned_footer.replace("(lib)", "(lib test)"),
                    work / "source", identity, policy())
            integration = copy.deepcopy(good)
            integration["target"].update(kind=["test"], crate_types=["bin"], name="frequency_prior",
                src_path="/work/tmp/cargo-mutants-source-ABC123.tmp/crates/time-core/tests/frequency_prior.rs")
            target_footer = owned_footer.replace("(lib)", '(test "frequency_prior")')
            self.assertEqual(MODULE._frequency_log_cause(outcome, raw(integration) + target_footer,
                work / "source", identity, policy())[0]["cargo_targets"], ['test "frequency_prior"'])
            with self.assertRaises(MODULE.Incomplete):
                MODULE._frequency_log_cause(outcome, raw(integration) + owned_footer, work / "source", identity, policy())
            for fault in ("package", "span", "null", "infrastructure", "line", "column", "finished"):
                bad = copy.deepcopy(good)
                if fault == "package": bad["package_id"] = "registry+foreign"
                elif fault == "span": bad["message"]["spans"][0]["file_name"] = "rustlib/core/src/lib.rs"
                elif fault == "null": bad["message"]["code"] = None
                elif fault == "infrastructure": bad["message"]["message"] = "linking failed: no space left"
                elif fault == "line": bad["message"]["spans"][0]["line_start"] = 99
                elif fault == "column": bad["message"]["spans"][0]["column_start"] = 999
                text = raw(bad)
                if fault == "finished": text = text.replace('"success":false', '"success":true')
                with self.subTest(fault=fault), self.assertRaises(MODULE.Incomplete):
                    MODULE._frequency_log_cause(outcome, text, work / "source", identity, policy())

    def test_compiler_spans_use_only_the_exact_reviewed_mutated_source(self):
        with tempfile.TemporaryDirectory() as directory:
            work = Path(directory)
            captured = captured_sources(work)
            filename = MODULE.FREQUENCY_PRIOR_SUBJECTS["frequency_state"]
            before = "pub fn value() -> u8 { 1 }\n"
            after = "pub fn value() -> u8 { Some(Box::leak(Box::new(Default::default()))) }\n"
            title = "replace value -> u8 with Some(Box::leak(Box::new(Default::default())))"
            diff = f"--- {filename}\n+++ {title}\n@@ -1 +1 @@\n-{before}+{after}"
            mutant = {"name": f"{filename}:1:23: {title}", "file": filename, "package": "time-core",
                "function": None, "genre": "FnValue", "replacement": "Some(Box::leak(Box::new(Default::default())))",
                "span": {"start": {"line": 1, "column": 23}, "end": {"line": 1, "column": 24}}}
            outcome = {"summary": "Unviable", "phase_results": phases("Unviable"), "scenario": {"Mutant": mutant}}
            record = diagnostic_record()
            start = after.index("Default::default()")
            end = start + len("Default::default()")
            record["message"]["spans"][0].update(column_start=start + 1, column_end=end + 1,
                byte_start=start, byte_end=end, text=[{"text": after.rstrip("\n"),
                    "highlight_start": start + 1, "highlight_end": end + 1}])
            def read(payload=diff, diagnostic=record):
                raw = phase_log(outcome, build=json.dumps(diagnostic) + '\n{"reason":"build-finished","success":false}\n')
                return MODULE._frequency_log_cause(outcome, raw, work / "source", captured, policy(), mutant_diff=payload)
            self.assertGreater(start + 1, len(before.rstrip("\n")))
            location = read()[0]["primary"][0]
            self.assertEqual(location["source_sha256"], hashlib.sha256(before.encode()).hexdigest())
            self.assertEqual(location["diagnostic_source_sha256"], hashlib.sha256(after.encode()).hexdigest())
            self.assertNotEqual(location["source_sha256"], location["diagnostic_source_sha256"])
            for bad in (None, diff.replace("--- " + filename, "--- foreign.rs"),
                        diff.replace("+++ " + title, "+++ foreign"),
                        diff.replace("@@ -1 +1 @@", "@@ -2 +1 @@"),
                        diff.replace("@@ -1 +1 @@", "@@ -1,2 +1 @@"),
                        diff.replace("@@ -1 +1 @@", "@@ -1 +2 @@"),
                        diff.replace("-" + before, "-foreign\n"), diff + diff,
                        diff + "--- foreign.rs\n", diff.rstrip("\n")):
                with self.subTest(payload=bad), self.assertRaises(MODULE.Incomplete):
                    read(bad)
            for field in ("column_end", "line_end", "byte_start", "text"):
                diagnostic = copy.deepcopy(record)
                span = diagnostic["message"]["spans"][0]
                if field == "text": span[field][0]["text"] = before.rstrip("\n")
                elif field == "column_end": span[field] = len(after) + 50
                else: span[field] += 1
                with self.subTest(field=field), self.assertRaises(MODULE.Incomplete):
                    read(diagnostic=diagnostic)

    def test_only_two_exact_historical_let_chain_null_causes_are_unviable(self):
        expected = {
            602: ("1da25cb416e19236586a5c55b0193b3387abd5b83d6d83bba784dbed2c12779a",
                  "0f2849df1d7ad6e4bfec917615afab2801d136b42caf4c0dfb69e4f29fa8596c"),
            607: ("cbc4c28727bafe833e2bb94f30d4a5194972248109a7d1d61d1501daa1bf15c7",
                  "2ad958303cc0b51f07caa692a7bccc80c9f1937cbfd7c947457c7747bccbbdb3"),
        }
        for line, (descriptor_sha, diff_sha) in expected.items():
            with self.subTest(line=line), tempfile.TemporaryDirectory() as directory:
                work = Path(directory)
                identity, outcome, record, diff = let_chain_fixture(work, line)
                self.assertEqual(MODULE.COMMON.canonical_sha256(outcome["scenario"]["Mutant"]), descriptor_sha)
                self.assertEqual(hashlib.sha256(diff.encode()).hexdigest(), diff_sha)
                self.assertEqual(identity[record["message"]["spans"][0]["file_name"]],
                    "5a6ed0953bf6e0be88338c52251ed594fcbaaf5cca530e1f541236eb3f8e042d")
                def read(row=outcome, diagnostic=record, payload=diff, captured=identity, *, current=False):
                    raw = phase_log(row, build=json.dumps(diagnostic) + '\n{"reason":"build-finished","success":false}\n'
                        + 'error: could not compile `time-core` (lib) due to 1 previous error\n')
                    return parser_cause(work, row, raw, captured, payload, current=current)
                self.assertIsNone(read()[0]["code"])
                with self.assertRaises(MODULE.Incomplete): read(current=True)
                for fault in ("missing_diff", "diff", "descriptor", "missing_context", "message", "span", "end",
                              "byte", "text", "source", "extra_primary", "package", "target"):
                    row, diagnostic, captured = copy.deepcopy(outcome), copy.deepcopy(record), dict(identity)
                    payload = diff
                    if fault == "missing_diff": payload = None
                    elif fault == "diff": payload += "\n"
                    elif fault == "descriptor": row["scenario"]["Mutant"]["replacement"] = "&&"
                    elif fault == "missing_context": del row["scenario"]
                    elif fault == "message": diagnostic["message"]["message"] += " foreign"
                    elif fault == "span": diagnostic["message"]["spans"][0]["line_start"] += 1
                    elif fault == "end": diagnostic["message"]["spans"][0]["column_end"] += 1
                    elif fault == "byte": diagnostic["message"]["spans"][0]["byte_start"] += 1
                    elif fault == "text": diagnostic["message"]["spans"][0]["text"][0]["text"] += " foreign"
                    elif fault == "source": captured[record["message"]["spans"][0]["file_name"]] = "a" * 64
                    elif fault == "extra_primary": diagnostic["message"]["spans"] *= 2
                    elif fault == "package": diagnostic["package_id"] = "registry+foreign"
                    elif fault == "target": diagnostic["target"]["test"] = False
                    with self.subTest(fault=fault), self.assertRaises(MODULE.Incomplete):
                        read(row, diagnostic, payload, captured)
                receiver = work / "source" / record["message"]["spans"][0]["file_name"]
                receiver.write_bytes(receiver.read_bytes() + b"// changed preimage\n")
                with self.assertRaises(MODULE.Incomplete):
                    read()

    def test_two_prepared_state_let_chains_require_exact_source_and_marker_span(self):
        # Only receiver counterparts have native evidence; these remain format preparation.
        cases = [(743, "FrequencyState::no_ready", 737, 754,
                  "739c84e9c3f210c9c78ff9c909008878c6b5993b59151086c6655425cf4fad4d",
                  "efc129fc2666132bcd2a9e8a3027d24a20982bfa95c4c7a4a156621809550fd2"),
                 (811, "FrequencyState::refresh_previous", 800, 821,
                  "6bd1273186c15a107768e3c9b4ffdf1a537dfbdc3b5b39c6aea2a8315bc47ab9",
                  "3d8cfd71ad2bffba7c378f6963e32d3b1dd496f0371b8f4599ed47ea9176c0d0")]
        for line, function, first, last, descriptor_sha, diff_sha in cases:
            with self.subTest(line=line), tempfile.TemporaryDirectory() as directory:
                work = Path(directory)
                captured = captured_sources(work)
                filename = MODULE.FREQUENCY_PRIOR_SUBJECTS["frequency_state"]
                before = historical_source(filename).decode().splitlines(keepends=True)
                (work / "source" / filename).write_text("".join(before))
                captured[filename] = HISTORICAL_SOURCE_DIGESTS[filename]
                after = before.copy()
                after[line - 1] = after[line - 1].replace("&&", "|| /* ~ changed by cargo-mutants ~ */", 1)
                title = f"replace && with || in {function}"
                mutant = {"name": f"{filename}:{line}:13: {title}", "package": "time-core", "file": filename,
                    "function": {"function_name": function, "return_type": "",
                        "span": {"start": {"line": first, "column": 5}, "end": {"line": last, "column": 6}}},
                    "span": {"start": {"line": line, "column": 13}, "end": {"line": line, "column": 15}},
                    "replacement": "||", "genre": "BinaryOperator"}
                diff = "".join(difflib.unified_diff(before, after, fromfile=filename, tofile=title, n=8))
                self.assertEqual(MODULE.COMMON.canonical_sha256(mutant), descriptor_sha)
                self.assertEqual(hashlib.sha256(diff.encode()).hexdigest(), diff_sha)
                offset = len("".join(after[:line - 1]).encode()) + 12
                record = diagnostic_record()
                record["message"].update(code=None, message="`||` operators are not supported in let chain conditions",
                    spans=[{"file_name": filename, "is_primary": True, "line_start": line, "line_end": line,
                        "column_start": 13, "column_end": 15, "byte_start": offset, "byte_end": offset + 2,
                        "text": [{"text": after[line - 1].rstrip("\n"), "highlight_start": 13, "highlight_end": 15}]}])
                row = {"summary": "Unviable", "phase_results": phases("Unviable"), "scenario": {"Mutant": mutant}}
                def read(diagnostic=record, payload=diff, *, current=False):
                    raw = phase_log(row, build=json.dumps(diagnostic) + '\n{"reason":"build-finished","success":false}\n')
                    return parser_cause(work, row, raw, captured, payload, current=current)
                self.assertIsNone(read()[0]["code"])
                with self.assertRaises(MODULE.Incomplete): read(current=True)
                for field in ("message", "column", "byte", "text"):
                    bad = copy.deepcopy(record)
                    if field == "message": bad["message"]["message"] += " foreign"
                    elif field == "column": bad["message"]["spans"][0]["column_end"] += 1
                    elif field == "byte": bad["message"]["spans"][0]["byte_start"] += 1
                    else: bad["message"]["spans"][0]["text"][0]["text"] += " foreign"
                    with self.subTest(field=field), self.assertRaises(MODULE.Incomplete): read(bad)
                with self.assertRaises(MODULE.Incomplete): read(payload=diff + "\n")

    def test_const_trait_null_requires_exact_payload_and_balanced_coded_companions(self):
        with tempfile.TemporaryDirectory() as directory:
            work = Path(directory)
            captured = captured_sources(work)
            filename = "crates/time-core/src/reference_input/rate_delivery.rs"
            before = historical_source(filename).decode().splitlines(keepends=True)
            after = before.copy()
            changed_line = "        Some(1.try_into().unwrap()) /* ~ changed by cargo-mutants ~ */\n"
            after[457:461] = [changed_line]
            source = work / "source" / filename
            source.parent.mkdir(parents=True, exist_ok=True)
            source.write_text("".join(before))
            captured[filename] = HISTORICAL_SOURCE_DIGESTS[filename]
            title = "replace FixtureRateSettlementCursor::next_ordinal -> Option<NonZeroU64> with Some(1.try_into().unwrap())"
            mutant = {"name": f"{filename}:458:9: {title}", "package": "time-core", "file": filename,
                "function": {"function_name": "FixtureRateSettlementCursor::next_ordinal", "return_type": "-> Option<NonZeroU64>",
                    "span": {"start": {"line": 455, "column": 5}, "end": {"line": 462, "column": 6}}},
                "span": {"start": {"line": 458, "column": 9}, "end": {"line": 461, "column": 10}},
                "replacement": "Some(1.try_into().unwrap())", "genre": "FnValue"}
            diff = "".join(difflib.unified_diff(before, after, fromfile=filename, tofile=title, n=8))
            self.assertEqual(MODULE.COMMON.canonical_sha256(mutant), "bd96f1630c2456f8e3772aeef1762864b72b069b952977eaaa1c576cccb5da5d")
            self.assertEqual(hashlib.sha256(diff.encode()).hexdigest(), "723d4ffee269e48a8707d559098cffb94f64b43492b29f1a65188b47f39cbc54")
            details = [
                ("E0658", "cannot call conditionally-const method `<u64 as TryInto<NonZero<u64>>>::try_into` in constant functions", 16, 26),
                (None, "`TryInto` is not yet stable as a const trait", 14, 26),
                ("E0015", "cannot call non-const method `Result::<NonZero<u64>, TryFromIntError>::unwrap` in constant functions", 27, 35),
            ]
            records = []
            for code, message, start, end in details:
                record = diagnostic_record()
                offset = len("".join(after[:457]).encode())
                record["message"].update(code={"code": code, "explanation": None} if code else None,
                    message=message, spans=[{"file_name": filename, "is_primary": True, "line_start": 458, "line_end": 458,
                        "column_start": start, "column_end": end, "byte_start": offset + start - 1,
                        "byte_end": offset + end - 1, "text": [{"text": changed_line.rstrip("\n"),
                            "highlight_start": start, "highlight_end": end}]}])
                records.append(record)
            outcome = {"summary": "Unviable", "phase_results": phases("Unviable"), "scenario": {"Mutant": mutant}}
            def read(values, payload=diff, *, current=False):
                raw = phase_log(outcome, build="".join(json.dumps(v) + "\n" for v in values)
                    + '{"reason":"build-finished","success":false}\n')
                return parser_cause(work, outcome, raw, captured, payload, current=current)
            self.assertEqual([v["code"] for v in read(records * 2)], ["E0658", None, "E0015"] * 2)
            with self.assertRaises(MODULE.Incomplete): read(records * 2, current=True)
            for bad in (records[1:], records[:2], records * 2 + [records[1]], records[:1] + records[2:]):
                with self.subTest(count=len(bad)):
                    if all(v["message"]["code"] is not None for v in bad):
                        self.assertTrue(read(bad))
                    else:
                        with self.assertRaises(MODULE.Incomplete): read(bad)
            for fault in ("message", "package", "span"):
                bad = copy.deepcopy(records * 2)
                if fault == "message": bad[0]["message"]["message"] += " foreign"
                elif fault == "package": bad[0]["package_id"] = "registry+foreign"
                else: bad[0]["message"]["spans"][0]["column_start"] += 1
                with self.subTest(fault=fault), self.assertRaises(MODULE.Incomplete): read(bad)

    def test_three_prepared_const_getter_bindings_remain_closed_and_execution_unverified(self):
        # These are parser-format controls derived from reviewed source/diffs,
        # not assertions that the three native compiler outcomes were observed.
        cases = [
            ("frequency_state", "FixtureFrequencyHistory::emitted_round", 474, 25, 471, 475,
             "b4a8e086768d821986b49d35a8e83abb608622a07135c546f3ff8c78f3ffc4cc",
             "acd2dccf2275dab8bd86450d96783c9d815ccec8d95ddf4b7cb074da4c2a5519", 18716),
            ("phase_plan", "FixturePhaseProvenance::emitted_round", 417, 19, 414, 418,
             "4cf549f72bd867c230bb7cda531a88ee11599ff6cda1317de59e141148b94942",
             "239121f694f3d071cf24f691f7cec824fea365013edaa1c0629af5a1c86ef775", 16956),
            ("phase_plan", "FixturePhaseProposal<'_>::ordinal", 495, 28, 492, 496,
             "2ddfa326dd703b5748dc0a4faa2a7703813bf170a1296bd443c0523fc2d4d6ad",
             "97e70b1874a3f24367d91df0294225729b0e8513e6f4d23873b56ebd96427a93", 19714),
        ]
        for owner, function, line, last, first_fn, last_fn, descriptor_sha, diff_sha, expected_offset in cases:
            with self.subTest(owner=owner, line=line), tempfile.TemporaryDirectory() as directory:
                work = Path(directory)
                captured = captured_sources(work)
                filename = MODULE.FREQUENCY_PRIOR_SUBJECTS[owner]
                before = historical_source(filename).decode().splitlines(keepends=True)
                source = work / "source" / filename
                source.parent.mkdir(parents=True, exist_ok=True)
                source.write_text("".join(before))
                captured[filename] = HISTORICAL_SOURCE_DIGESTS[filename]
                after = before.copy()
                after[line - 1] = "        1.try_into().unwrap() /* ~ changed by cargo-mutants ~ */\n"
                offset = len("".join(after[:line - 1]).encode()) + 8
                self.assertEqual(offset, expected_offset)
                title = f"replace {function} -> NonZeroU64 with 1.try_into().unwrap()"
                mutant = {"name": f"{filename}:{line}:9: {title}", "file": filename, "package": "time-core",
                    "function": {"function_name": function, "return_type": "-> NonZeroU64",
                        "span": {"start": {"line": first_fn, "column": 5}, "end": {"line": last_fn, "column": 6}}},
                    "span": {"start": {"line": line, "column": 9}, "end": {"line": line, "column": last}},
                    "genre": "FnValue", "replacement": "1.try_into().unwrap()"}
                diff = "".join(difflib.unified_diff(before, after, fromfile=filename, tofile=title, n=8))
                self.assertEqual(MODULE.COMMON.canonical_sha256(mutant), descriptor_sha)
                self.assertEqual(hashlib.sha256(diff.encode()).hexdigest(), diff_sha)
                specs = [
                    ("E0658", "cannot call conditionally-const method `<u64 as TryInto<NonZero<u64>>>::try_into` in constant functions", 11, 21),
                    (None, "`TryInto` is not yet stable as a const trait", 9, 21),
                    ("E0015", "cannot call non-const method `Result::<NonZero<u64>, TryFromIntError>::unwrap` in constant functions", 22, 30),
                ]
                records = []
                for code, message, start, end in specs:
                    record = diagnostic_record()
                    record["message"].update(code={"code": code, "explanation": None} if code else None, message=message,
                        spans=[{"file_name": filename, "is_primary": True, "line_start": line, "line_end": line,
                            "column_start": start, "column_end": end, "byte_start": offset + start - 9,
                            "byte_end": offset + end - 9, "text": [{"text": after[line - 1].rstrip("\n"),
                                "highlight_start": start, "highlight_end": end}]}])
                    records.append(record)
                outcome = {"summary": "Unviable", "phase_results": phases("Unviable"), "scenario": {"Mutant": mutant}}
                def read(values=records, row=outcome, payload=diff, *, current=False):
                    raw = phase_log(row, build="".join(json.dumps(v) + "\n" for v in values)
                        + '{"reason":"build-finished","success":false}\n')
                    return parser_cause(work, row, raw, captured, payload, current=current)
                self.assertEqual([v["code"] for v in read(records * 2)], ["E0658", None, "E0015"] * 2)
                with self.assertRaises(MODULE.Incomplete): read(records * 2, current=True)
                foreign = copy.deepcopy(records)
                foreign[0]["target"]["name"] = "foreign"
                wrong_span = copy.deepcopy(records)
                wrong_span[2]["message"]["spans"][0]["byte_end"] += 1
                for values in (records[1:], records * 2 + [records[1]], foreign, wrong_span):
                    with self.assertRaises(MODULE.Incomplete): read(values)
                wrong_context = copy.deepcopy(outcome)
                wrong_context["scenario"]["Mutant"]["replacement"] += " "
                with self.assertRaises(MODULE.Incomplete): read(row=wrong_context)
                with self.assertRaises(MODULE.Incomplete): read(payload=diff + "\n")

    def test_retained_config_is_exact_debug_only_and_custody_checked(self):
        with tempfile.TemporaryDirectory() as directory:
            work = Path(directory)
            report = retained_config(work)
            config = work / "cargo-home/config.toml"
            healthy = config.read_bytes()
            result = MODULE._frequency_config(work, report, {}, {"synthetic_vendor": "pin"})
            self.assertEqual(result["sha256"], hashlib.sha256(healthy).hexdigest())
            for fault in ("changed", "link", "profiles", "vendor", "missing"):
                altered = copy.deepcopy(report)
                if fault == "changed": config.write_text("unreviewed config")
                elif fault == "link":
                    other = work / "other-config"
                    other.write_bytes(healthy)
                    config.unlink()
                    config.symlink_to(other)
                elif fault == "profiles": altered["mutation_debug_configuration"]["other_profile_settings_changed"] = True
                elif fault == "vendor":
                    before = '[source.crates-io]\nreplace-with="reviewed"\n[source.reviewed]\ndirectory="relative"\n'
                    altered["mutation_debug_configuration"].update(before_text=before, before_sha256=hashlib.sha256(before.encode()).hexdigest())
                else: config.unlink()
                with self.subTest(fault=fault), self.assertRaises((MODULE.Incomplete, OSError)):
                    MODULE._frequency_config(work, altered, {}, {"synthetic_vendor": "pin"})
                if config.exists() or config.is_symlink(): config.unlink()
                config.write_bytes(healthy)

    def test_closed_raw_manifest_hashes_every_artifact_and_refuses_tampered_neighbors(self):
        with tempfile.TemporaryDirectory() as directory:
            work = Path(directory)
            identity = captured_sources(work)
            report = retained_config(work)
            p = policy()
            p["test_inventories"]["baseline"].append("detector")
            p["test_inventories"]["baseline"].sort()
            owner = "frequency_state"
            item = {"name": "owned_fault", "file": MODULE.FREQUENCY_PRIOR_SUBJECTS[owner], "package": "time-core",
                    "function": {"function_name": "value", "return_type": "-> u8", "span": {"start": {"line": 1, "column": 1}, "end": {"line": 1, "column": 34}}},
                    "span": {"start": {"line": 1, "column": 1}, "end": {"line": 1, "column": 2}},
                    "replacement": "0", "genre": "FnValue", "diff": "literal reviewed raw diff\n"}
            inventory = {item["name"]: item}
            root = work / "mutation" / owner / "mutants.out"
            (root / "log").mkdir(parents=True)
            (root / "diff").mkdir()
            baseline = {"scenario": "Baseline", "summary": "Success", "log_path": "log/baseline.log", "diff_path": None,
                        "phase_results": phases("Success")}
            mutant = {"scenario": {"Mutant": {k: v for k, v in item.items() if k != "diff"}}, "summary": "CaughtMutant",
                      "log_path": "log/fault.log", "diff_path": "diff/fault.diff", "phase_results": phases("CaughtMutant")}
            names = p["test_inventories"]["baseline"]
            healthy = "running " + str(len(names)) + " tests\n" + "".join("test " + name + " ... ok\n" for name in names)
            healthy += f"test result: ok. {len(names)} passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s\n"
            (root / "log/baseline.log").write_text(phase_log(baseline, healthy))
            log = root / "log/fault.log"
            log.write_text(phase_log(mutant, failed_test()))
            diff = root / "diff/fault.diff"
            diff.write_text(item["diff"])
            outcome = root / "outcomes.json"
            outcome.write_text(json.dumps({"end_time": "2026-11-08T00:00:00Z", "outcomes": [baseline, mutant]}))
            def read():
                return MODULE.frequency_artifact_manifest(work, report, p, owner, inventory, identity, {}, {"synthetic_vendor": "pin"})
            manifest = read()
            self.assertEqual(len(manifest["files"]), 5)
            self.assertEqual(manifest["causal_observations"]["owned_fault"]["causes"][0]["test"], "detector")
            original = log.read_bytes()
            log.write_bytes(original + b"\n")
            self.assertNotEqual(read()["files"], manifest["files"])
            log.write_bytes(original)
            for fault in ("diff", "missing", "link", "extra", "source", "outcomes"):
                if fault == "diff": diff.write_text("changed diff")
                elif fault == "missing": log.unlink()
                elif fault == "link":
                    (root / "linked").write_bytes(original)
                    log.unlink()
                    log.symlink_to(root / "linked")
                elif fault == "extra": (root / "log/foreign.log").write_text("foreign")
                elif fault == "source": (work / "source/foreign.rs").write_text("foreign")
                else: outcome.write_text("{}")
                with self.subTest(fault=fault), self.assertRaises((MODULE.Incomplete, OSError)):
                    read()
                if log.exists() or log.is_symlink(): log.unlink()
                log.write_bytes(original)
                diff.write_text(item["diff"])
                for extra in (root / "log/foreign.log", work / "source/foreign.rs"):
                    if extra.exists(): extra.unlink()
                outcome.write_text(json.dumps({"end_time": "2026-11-08T00:00:00Z", "outcomes": [baseline, mutant]}))

    def test_every_lane_failure_finishes_the_owned_session(self):
        args = SimpleNamespace(subject="frequency_prior", tool_admission_profile="stable-core", lane="baseline",
                               inventory_admission=None, cohort_plan=None, cohort_receipt=[], cohort_id=None)
        session = SimpleNamespace(run=lambda *_a, **_k: (_ for _ in ()).throw(MODULE.Incomplete("primary")))
        session.finish = unittest.mock.Mock()
        with patch.object(MODULE, "read_frequency_prior_policy", return_value=policy()), \
             patch.object(MODULE.COMMON, "Session", return_value=session), \
             patch.object(MODULE, "_frequency_captured"):
            with self.assertRaisesRegex(MODULE.Incomplete, "primary"):
                MODULE.execute_frequency_prior(args, {})
        session.finish.assert_called_once()


if __name__ == "__main__":
    unittest.main()
