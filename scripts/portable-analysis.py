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
import stat
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
CHART_ENCLOSURE_ERROR_VALUE = (
    "{ let _ = (projection, enclose_labeled_scalars, "
    "FixtureChartProjection::enclosure_distance_seconds); FixtureChartError::Invariant }"
)
CHART_ERROR_SUBJECTS = (
    ("chart", "crates/time-core/src/regression/chart.rs"),
    ("chart_roundoff", "crates/time-core/src/regression/chart/roundoff.rs"),
    ("chart_enclosure", "crates/time-core/src/chart_enclosure.rs"),
    ("enclosure", "crates/time-core/src/projection_enclosure.rs"),
)
CHART_BATCH_SUBJECTS = (("chart_batch", "crates/time-core/src/chart_batch.rs"),)
CHART_PRINCIPAL_SUBJECTS = (
    ("principal", "crates/time-core/src/principal.rs"),
    ("chart_principal", "crates/time-core/src/principal/chart.rs"),
    ("chart_batch", "crates/time-core/src/chart_batch.rs"),
)
CHART_COMBINATION_SUBJECTS = (
    ("combination", "crates/time-core/src/combination.rs"),
    ("combination_admission", "crates/time-core/src/combination/admission.rs"),
    ("combination_lifecycle", "crates/time-core/src/combination/lifecycle.rs"),
    ("chart", "crates/time-core/src/regression/chart.rs"),
    ("chart_roundoff", "crates/time-core/src/regression/chart/roundoff.rs"),
    ("chart_combination_roundoff", "crates/time-core/src/combination/chart/roundoff.rs"),
    ("chart_combination_receiving", "crates/time-core/src/combination/chart/receiving.rs"),
    ("chart_combination_reduction", "crates/time-core/src/combination/chart/reduction.rs"),
    ("chart_combination_lifecycle", "crates/time-core/src/combination/lifecycle/chart.rs"),
)
REFERENCE_INPUT_SUBJECTS = (("reference_input", "crates/time-core/src/reference_input.rs"),)
SETTLED_RATE_SUBJECTS = (
    ("reference_input", "crates/time-core/src/reference_input.rs"),
    ("rate_delivery", "crates/time-core/src/reference_input/rate_delivery.rs"),
    ("chart", "crates/time-core/src/regression/chart.rs"),
    ("chart_roundoff", "crates/time-core/src/regression/chart/roundoff.rs"),
)
CHART_COMPOSITION_SUBJECTS = (
    ("chart", "crates/time-core/src/regression/chart.rs"),
    ("chart_roundoff", "crates/time-core/src/regression/chart/roundoff.rs"),
    ("chart_batch", "crates/time-core/src/chart_batch.rs"),
)
DOMAIN_LEAP_SUBJECTS = (
    ("coordinate", "crates/time-domain/src/coordinate.rs"),
    ("era", "crates/time-domain/src/era.rs"),
    ("exchange", "crates/time-domain/src/exchange.rs"),
    ("clock", "crates/time-domain/src/clock.rs"),
    ("rounding", "crates/time-domain/src/rounding.rs"),
    ("leap", "crates/time-domain/src/leap.rs"),
    ("leap_validation", "crates/time-domain/src/leap/validation.rs"),
    ("leap_interpretation", "crates/time-domain/src/leap/interpretation.rs"),
    ("leap_bundled", "crates/time-domain/src/leap/bundled.rs"),
)
DOMAIN_LEAP_ERROR_VALUE = (
    "{ let _ = (CONTINUOUS_FROM, DATA_REVISION, FINITE_FUTURE_BEFORE, HISTORY_COMPLETE_BEFORE, "
    "LEAP_RECORDS, PROVENANCE_SHA256, RAW_SHA256, LeapContinuousFrom::new, LeapDataIdentity::new, "
    "LeapFutureExpiry::new, LeapHistoryBefore::new); LeapError::InvalidRecords }"
)
DOMAIN_LEAP_ERROR_SUBJECTS = ("leap_bundled",)
ENGINE_CONTRACT_SUBJECTS = (("library", "crates/nevrotick-engine/src/lib.rs"),)
# Ordinary compiler contracts have their own reviewed current product snapshot.
# Historical coverage/mutation/fuzz candidates and their closed gates stay fixed.
CONTRACT_PRODUCT_BINDING = {
    "source_file_count": 595,
    "candidate_sha256": "82a19ffa0810d9fb9da8575c825ba9a0dd0bc3ac107d5584e9df6881db283db7",
}
SERVICE_LIFECYCLE_SUBJECTS = (("lifecycle", "crates/windows-platform/src/service.rs"),)
SERVICE_SCM_SUBJECTS = (*SERVICE_LIFECYCLE_SUBJECTS, ("scm", "crates/windows-platform/src/scm.rs"))
SERVICE_PRIVILEGE_SUBJECTS = (*SERVICE_SCM_SUBJECTS,
                            ("privilege", "crates/windows-platform/src/privilege_observation.rs"))

FREQUENCY_PRIOR_SUBJECTS = (
    ("frequency_state", "crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs"),
    ("frequency_numeric", "crates/time-core/src/regression/chart/roundoff/frequency_prior.rs"),
    ("phase_plan", "crates/time-core/src/reference_input/rate_delivery/phase_plan.rs"),
    ("rate_delivery", "crates/time-core/src/reference_input/rate_delivery.rs"),
)

BASELINE_FEEDBACK_SUBJECTS = (
    ("frequency_state", "crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs"),
    ("numeric", "crates/time-core/src/regression/chart/roundoff/frequency_prior.rs"),
    ("phase_plan", "crates/time-core/src/reference_input/rate_delivery/phase_plan.rs"),
    ("rate_delivery", "crates/time-core/src/reference_input/rate_delivery.rs"),
    ("chart", "crates/time-core/src/regression/chart.rs"),
)

# Complete changed critical files for the adaptive/explicit-step checkpoint.
# Earlier role tuples and their historical bindings remain separate.
PHASE_STEP_SUBJECTS = (
    ("phase_plan", "crates/time-core/src/reference_input/rate_delivery/phase_plan.rs"),
    ("phase_numeric", "crates/time-core/src/regression/chart/roundoff/phase_plan.rs"),
    ("step", "crates/time-core/src/reference_input/rate_delivery/phase_plan/step.rs"),
    ("rate_delivery", "crates/time-core/src/reference_input/rate_delivery.rs"),
    ("reference_input", "crates/time-core/src/reference_input.rs"),
    ("adaptive_horizon", "crates/time-core/src/reference_input/rate_delivery/phase_plan/adaptive.rs"),
    ("frequency_numeric", "crates/time-core/src/regression/chart/roundoff/frequency_prior.rs"),
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
ANALYSIS_JSON_MAX_BYTES = 64 * 1024 * 1024
ANALYSIS_LOG_MAX_BYTES = 16 * 1024 * 1024
ANALYSIS_JSON_MAX_DEPTH = 64
ANALYSIS_JSON_MAX_MARKERS = 4_000_000
ANALYSIS_PATH_MAX_BYTES = 4096
ANALYSIS_PATH_MAX_COMPONENTS = 64


def analysis_output_path(value: str | Path) -> Path:
    """Bound raw UTF-8 path text before encoding/splitting, without any I/O.

    Relative output names are admitted here so their callers can retain their
    existing escape checks; the file reader separately requires absolute paths.
    """
    if isinstance(value, str):
        text = value
    elif isinstance(value, Path):
        text = str(value)
    else:
        raise Incomplete("Analysis output path must be text or a Path.")
    if len(text) > ANALYSIS_PATH_MAX_BYTES:
        raise Incomplete("Analysis output path exceeds its byte bound.")
    try:
        encoded = text.encode("utf-8")
    except UnicodeError as error:
        raise Incomplete("Analysis output path is not strict UTF-8.") from error
    if len(encoded) > ANALYSIS_PATH_MAX_BYTES:
        raise Incomplete("Analysis output path exceeds its byte bound.")
    path = Path(text)
    if len(path.parts) - bool(path.anchor) > ANALYSIS_PATH_MAX_COMPONENTS:
        raise Incomplete("Analysis output path exceeds its component bound.")
    return path


def read_analysis_bytes(path: Path, *, maximum: int) -> bytes:
    """Snapshot one bounded, owned regular output through nonlinked directories.

    Callers select their fixed private output paths and retain any relative-path
    escape guards. Byte/component limits apply before opening any descriptor.
    Directory descriptors pin every component from /; attachment
    rechecks reject replacement while the leaf is read. No FIFO/device is read,
    even after a race. The returned bytes, including their digest if needed, own
    the observation; reopening the path would create a different observation.
    """
    path = analysis_output_path(path)
    if (not path.is_absolute() or ".." in path.parts or len(path.parts) < 2
            or type(maximum) is not int or not 0 < maximum <= ANALYSIS_JSON_MAX_BYTES):
        raise Incomplete("Analysis output requires an absolute bounded file path.")
    descriptors = []
    bindings = []

    def identity(item):
        return (item.st_dev, item.st_ino, item.st_mode, item.st_uid, item.st_gid,
                item.st_nlink, item.st_size, item.st_mtime_ns, item.st_ctime_ns)

    try:
        flags = os.O_RDONLY | os.O_CLOEXEC | os.O_NOFOLLOW | os.O_NONBLOCK
        directories = flags | os.O_DIRECTORY
        descriptors.append(os.open("/", directories))
        for name in path.parts[1:-1]:
            parent = descriptors[-1]
            descriptor = os.open(name, directories, dir_fd=parent)
            descriptors.append(descriptor)
            bindings.append((parent, name, os.fstat(descriptor)))
        parent = descriptors[-1]
        descriptor = os.open(path.name, flags, dir_fd=parent)
        descriptors.append(descriptor)
        before = os.fstat(descriptor)
        if (not stat.S_ISREG(before.st_mode) or before.st_uid != os.geteuid()
                or before.st_nlink != 1 or before.st_size > maximum):
            raise Incomplete("Analysis output must be one owned bounded regular file.")
        chunks, size = [], 0
        while size <= maximum:
            chunk = os.read(descriptor, min(64 * 1024, maximum + 1 - size))
            if not chunk:
                break
            chunks.append(chunk)
            size += len(chunk)
        after = os.fstat(descriptor)
        attached = os.stat(path.name, dir_fd=parent, follow_symlinks=False)
        if (size > maximum or size != before.st_size or identity(before) != identity(after)
                or identity(after) != identity(attached)):
            raise Incomplete("Analysis output changed during its bounded read.")
        for parent, name, opened in bindings:
            attached = os.stat(name, dir_fd=parent, follow_symlinks=False)
            if (not stat.S_ISDIR(attached.st_mode)
                    or (attached.st_dev, attached.st_ino) != (opened.st_dev, opened.st_ino)):
                raise Incomplete("Analysis output directory changed during its bounded read.")
        return b"".join(chunks)
    except (OSError, ValueError, UnicodeError) as error:
        raise Incomplete("Analysis output is unavailable or violates file custody.") from error
    finally:
        for descriptor in reversed(descriptors):
            os.close(descriptor)


def read_analysis_text(path: Path, *, maximum: int = ANALYSIS_LOG_MAX_BYTES) -> str:
    """Decode only the immutable bounded snapshot, with strict UTF-8 admission."""
    try:
        return read_analysis_bytes(path, maximum=maximum).decode("utf-8")
    except UnicodeError as error:
        raise Incomplete("Analysis output is not strict UTF-8 text.") from error


def read_analysis_json(path: Path, *, maximum: int = ANALYSIS_JSON_MAX_BYTES) -> dict:
    """Admit finite strict analyzer JSON after bounding bytes and parser work.

    The pre-scan ignores punctuation inside strings and caps container depth and
    structural markers before json.loads can allocate nested containers/items.
    Byte and marker bounds also cap string/whitespace scanning and total nodes.
    Native file/count/phase/descriptor semantics remain with their existing gates.
    """
    text = read_analysis_text(path, maximum=maximum)
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
            if character in "[{":
                depth += 1
            elif character in "]}":
                depth -= 1
            if character in "[]{},:":
                markers += 1
            if depth < 0 or depth > ANALYSIS_JSON_MAX_DEPTH or markers > ANALYSIS_JSON_MAX_MARKERS:
                raise Incomplete("Analysis JSON exceeds its structural work or depth bound.")

    def pairs(items):
        result = {}
        for key, value in items:
            if key in result:
                raise Incomplete("Analysis JSON contains a duplicate field.")
            result[key] = value
        return result

    def invalid_constant(_token):
        raise Incomplete("Analysis JSON contains a nonfinite number.")

    def number(token, *, floating):
        if len(token) > 128:
            raise Incomplete("Analysis JSON number exceeds its token bound.")
        value = float(token) if floating else int(token)
        if floating and not math.isfinite(value):
            invalid_constant(token)
        return value

    try:
        value = json.loads(text, object_pairs_hook=pairs, parse_constant=invalid_constant,
                           parse_int=lambda token: number(token, floating=False),
                           parse_float=lambda token: number(token, floating=True))
    except (ValueError, UnicodeError, RecursionError) as error:
        raise Incomplete("Analysis output is not one complete strict JSON object.") from error
    if not isinstance(value, dict):
        raise Incomplete("Analysis output JSON must be an object.")
    return value


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



ENGINE_KE_POLICY = "policy/engine-ke-association-analysis.json"
ENGINE_KE_SUBJECTS = (
    ("ke_core", "crates/nevrotick-engine/src/nts_ke_experiment/mod.rs"),
    ("ke_association", "crates/nevrotick-engine/src/nts_ke_experiment/association.rs"),
    ("ke_process", "crates/nevrotick-engine/src/nts_ke_experiment/process.rs"),
    ("ke_handoff", "crates/nevrotick-engine/src/nts_ke_experiment/handoff.rs"),
    ("ke_request", "crates/nevrotick-engine/src/nts_ke_experiment/request.rs"),
    ("ke_fault", "crates/nevrotick-engine/src/nts_ke_experiment/transport_fault.rs"),
    ("ke_ledger", "crates/nevrotick-engine/src/nts_ke_experiment/association/ledger.rs"),
    ("ke_retirement", "crates/nevrotick-engine/src/nts_ke_experiment/association/ledger/retirement.rs"),
    ("ke_response", "crates/nevrotick-engine/src/nts_ke_experiment/response.rs"),
    ("ke_datagram", "crates/nevrotick-engine/src/nts_ke_experiment/datagram.rs"),
    ("ke_recovery", "crates/nevrotick-engine/src/nts_ke_experiment/recovery.rs"),
    ("ke_job", "crates/nevrotick-engine/src/nts_ke_experiment/job.rs"),
)
ENGINE_KE_CRITICAL = (
    "crates/nevrotick-engine/src/lib.rs",
    *(path for _name, path in ENGINE_KE_SUBJECTS),
    "crates/nevrotick-engine/src/nts_ke_experiment/profile.rs",
    "crates/nevrotick-engine/src/nts_ke_experiment/transport.rs",
    "tools/crypto-cng-lifecycle/src/bin/nts-ke-probe.rs",
)

# Exact current source, complete typed descriptors and independently challenged
# caller arguments. Any source/candidate/proof drift requires a new review.
ENGINE_KE_EQUIVALENCE_PROOF = "tasks/engine-ke-association-equivalences.md"
ENGINE_KE_EQUIVALENCE_REQUIREMENT = "REQ-8/BND-2; AC-04; CHECK-6/8/9/17/21/25/26/27"
ENGINE_KE_EQUIVALENCE_REVIEWERS = '/root; /root/ledger_boundary_domain; /root/resource_envelope_review'
ENGINE_KE_EQUIVALENCE_IDS = frozenset({
    'KE-RETURN-POOL-CAPACITY-OR',
    'KE-RETURN-POOL-BYTES-OR',
    'KE-SEND-PREDECESSOR-VALUE-OR',
    'KE-SEND-ARCHIVE-VALUE-OR',
    'KE-SEND-ARCHIVE-CAPACITY-OR',
    'KE-SEND-ARCHIVE-BYTES-OR',
    'KE-SEND-ARCHIVE-BYTES-EQUALITY',
    'KE-SEND-ARCHIVE-BYTES-INCLUSIVE',
    'KE-RECEIVE-STABLE-SLOT-OR',
})
ENGINE_KE_EQUIVALENCE_CANDIDATE = '964a9573143e5668a63d264bb44906fa103d5b4c3ddf1fc82cecea33012c5468'
ENGINE_KE_EQUIVALENCE_PROOF_SHA256 = 'f67b973c6ed616a8206c0f0a80f7737d4b459aa1106eb09b9b00f729441cd695'
ENGINE_KE_EQUIVALENCE_REVIEWS = {
    'KE-RETURN-POOL-CAPACITY-OR': ('8c689ee8e42441a7aa64c69098970aa7d32d24db45c02ab732047bd492781c36', '565ae40e381f980a35548caa0ce690dcc1f2b8658cfa0e47672bb5b2fd0c7dfe', 'The complete admitted return domain includes genuine Reserved, Preparing and Prepared tickets, failed preparation, exact pool-byte equality, cancellation and expiry; it does not assume successful encoding. After the unchanged guards, reservation has removed one exact pool entry and no admitted writer can shrink or replace that pool before return. Its remaining length is at most seven and its capacity still exceeds that length; its reserved-plus-available byte sum remains within 131072. The changed connector combines only false predicates. Both forms preserve the same returned allocation, any retired UID, spent attempts, original deadline and error paths.'),
    'KE-RETURN-POOL-BYTES-OR': ('8c689ee8e42441a7aa64c69098970aa7d32d24db45c02ab732047bd492781c36', 'ed4cf1652fa16495dbfdd843f34d528630bddd5fd9288461ec597581f0c6087a', 'After the unchanged ticket, Published-control and pre-invocation phase checks, the exact reserved cookie was removed from a checked pool of at most eight entries. No admitted operation changes that pool while the slot remains reserved. Available count is therefore at most seven, available-plus-reserved payload remains at most 131072 bytes, and removal preserves an allocated slot beyond the current length. All three rejection predicates are false, including strict byte excess at exact equality. Both operator forms move back the same cookie without copying, exposing it, refunding work or renewing its original expiry. Earlier refusal paths are unchanged.'),
    'KE-SEND-PREDECESSOR-VALUE-OR': ('8c689ee8e42441a7aa64c69098970aa7d32d24db45c02ab732047bd492781c36', '6ded4b8df9e6af0867252161051f49d0e4856528022e7cfbf9049a8dce5bd3d8', 'The stored predecessor set is filtered before reservation selects the exact cookie. It is immutable thereafter; install_previous requires the untouched reservation allowance and cannot replace it once any ticket has been issued. Consequently the selected Prepared cookie is not a member of the stored predecessor, whether that set is empty, arbitrary or complete. Local count, capacity, encoded-byte and local-membership predicates are also false. Replacing the final connector with AND therefore changes no error, archive move, callback, no-reuse or lifecycle outcome. This argument does not certify completeness or provenance of the predecessor set; actual consuming retirement and checked projection enforce that separate producer contract.'),
    'KE-SEND-ARCHIVE-VALUE-OR': ('8c689ee8e42441a7aa64c69098970aa7d32d24db45c02ab732047bd492781c36', 'faecc1877ce8bfd6a3ada4a9fa96ab3f7d6407ce1c54f6e5d49b30bd7eaecf9f', 'Reservation filters every value in the current local used archive before moving its selected cookie into the single occupied slot. No other invocation can append a local archive entry while that ticket owns the slot, and pre-send return consumes the ticket without refunding its checkout. Thus the local membership predicate is false at the unchanged Prepared/currentness guard. Count, capacity, encoded-byte and predecessor predicates are also false in that domain. Changing this connector only changes pure bounded reads; both forms archive the same cookie before callback exposure and preserve all errors, receipt absence, budgets and completion custody.'),
    'KE-SEND-ARCHIVE-CAPACITY-OR': ('8c689ee8e42441a7aa64c69098970aa7d32d24db45c02ab732047bd492781c36', '835047e112b1b1bb8fb21647a053968639703703f3aa7a8f247fcf6f427ac40e', 'On the whole admitted Prepared-ticket domain, including the last checkout, local archive capacity is at least 32 and pre-push length is at most 31. Return and failure never refund a reservation or shrink this archive. The encoded-byte ceiling makes excess false; reservation and immutable predecessor installation make both membership predicates false. The changed connector therefore joins false conditions only. Neither form changes the moved cookie, Invoked transition, callback arguments, unknown-completion custody or subsequent original-control checks. This is a local-context argument: predecessor history remains a separate bounded owner, not extra entries in the local Vec.'),
    'KE-SEND-ARCHIVE-BYTES-OR': ('8c689ee8e42441a7aa64c69098970aa7d32d24db45c02ab732047bd492781c36', 'c6792fd2f33e35cf60db7fc48415f709eded660721391e6c91926de366a2d91d', 'Only a checked same-owner Prepared packet reaches the archive predicate. The current checkout is one of at most 32 irreversible local reservations and is not archived yet, so existing local length is at most 31 and capacity is at least 32. Every sendable cookie fits the 4096-byte algorithm-15 packet after at least 128 bytes of header, UID, cookie header and authenticator overhead, hence each is at most 3968 bytes and the post-push local total is at most 126976, strictly below 131072. Reservation excludes both local and stored-predecessor members. All five local/predecessor rejection predicates are false. Both variants archive before the same callback and preserve every completion, failure, cancellation and no-reuse disposition.'),
    'KE-SEND-ARCHIVE-BYTES-EQUALITY': ('8c689ee8e42441a7aa64c69098970aa7d32d24db45c02ab732047bd492781c36', '8d5545f1e57acfc22966622c010df9dc0718ea65b243425c02a8538dbe755f8f', 'The unchanged checked packet formatter admits at most 3968 cookie bytes per actual prepared send and at most 32 such cookies enter one local archive. Its checked post-push sum is at most 126976, so both sum > 131072 and sum == 131072 are false without overflow. Earlier ticket, phase, expiry and preparation refusals remain unchanged. The mutation supplies no permission to narrow the raw history-bounds checker or return-pool domain: those independently admit exact 131072-byte data and retain their own negative and healthy controls.'),
    'KE-SEND-ARCHIVE-BYTES-INCLUSIVE': ('8c689ee8e42441a7aa64c69098970aa7d32d24db45c02ab732047bd492781c36', '5bc3655bf490aeb9efa2180785d299ca5774c427233b389e236c1a495e061398', 'For every admitted Prepared invocation, including the 32nd local reservation, the packet/attempt bounds give a post-push local archive maximum of 126976 bytes. Both sum > 131072 and sum >= 131072 are therefore false. The same cookie and packet reach the same callback after one archive move; no error, budget, expiry, cancellation or completion disposition differs. The bound concerns only this formatter-produced local archive, not arbitrary bounded data, total predecessor-plus-current memory, TLS traffic, native allocations or a wider future profile.'),
    'KE-RECEIVE-STABLE-SLOT-OR': ('8c689ee8e42441a7aa64c69098970aa7d32d24db45c02ab732047bd492781c36', 'a25a4a9e6b3396c7336c9a8358aaf0836d189daf96e497637967efc1ea453f2a', 'The complete receive_reply input domain is guarded at the actual charged receiver. Its initial Published guard requires Pending and captures that active serial, UID, wire token, allowance and send observation before charging. Successful stage_current_reply and stage_replacement only immutably borrow the owner, use its own keys and checked issued-UID prefix, and expose no effectful publisher, await or owner-mutating callback. Staging errors may retire work but exit before the final comparison. The final fresh Published guard rejects stop or expiry before entering the inline publication closure. Whenever the changed expression executes, the actual serial and Pending phase still match the captured values, so both mismatch predicates are false. Both variants atomically publish the same cookies/receipt once; physical timing and native completion are not established.'),
}


def engine_ke_equivalences_valid(policy: dict) -> bool:
    """Allow no E, or all nine exact independently reviewed caller identities.

    Full current product/source/proof admission remains mandatory in
    admit_mutation_equivalences. No checked-caller argument narrows another
    helper's input domain, grants a kill or silently changes the 95-percent gate.
    """
    if type(policy) is not dict or type(policy.get("mutation_equivalences")) is not list:
        return False
    entries = policy["mutation_equivalences"]
    if not entries:
        return True
    if (len(entries) != 9 or set(ENGINE_KE_EQUIVALENCE_REVIEWS) != ENGINE_KE_EQUIVALENCE_IDS
            or re.fullmatch(r"[a-f0-9]{64}", ENGINE_KE_EQUIVALENCE_CANDIDATE) is None
            or re.fullmatch(r"[a-f0-9]{64}", ENGINE_KE_EQUIVALENCE_PROOF_SHA256) is None
            or type(policy.get("binding")) is not dict
            or policy["binding"].get("candidate_sha256") != ENGINE_KE_EQUIVALENCE_CANDIDATE):
        return False
    fields = {"id", "candidate_sha256", "source_sha256", "mutant", "requirement", "rationale", "proof"}
    seen = set()
    for entry in entries:
        if type(entry) is not dict or set(entry) != fields:
            return False
        identifier = entry["id"]
        if type(identifier) is not str or identifier not in ENGINE_KE_EQUIVALENCE_IDS or identifier in seen:
            return False
        review = ENGINE_KE_EQUIVALENCE_REVIEWS[identifier]
        if (type(review) is not tuple or len(review) != 3
                or any(type(value) is not str for value in review)
                or any(re.fullmatch(r"[a-f0-9]{64}", value) is None for value in review[:2])):
            return False
        source, descriptor, rationale = review
        if (entry["candidate_sha256"] != ENGINE_KE_EQUIVALENCE_CANDIDATE or entry["source_sha256"] != source
                or entry["requirement"] != ENGINE_KE_EQUIVALENCE_REQUIREMENT or entry["rationale"] != rationale
                or type(entry["mutant"]) is not dict):
            return False
        try:
            if canonical_sha256(entry["mutant"]) != descriptor:
                return False
        except (TypeError, ValueError):
            return False
        if entry["proof"] != {"path": ENGINE_KE_EQUIVALENCE_PROOF,
                "sha256": ENGINE_KE_EQUIVALENCE_PROOF_SHA256, "section": identifier,
                "reviewer": ENGINE_KE_EQUIVALENCE_REVIEWERS, "verdict": "independently-justified-equivalent"}:
            return False
        seen.add(identifier)
    return seen == ENGINE_KE_EQUIVALENCE_IDS


def engine_ke_features(scope: Scope) -> tuple[str, ...]:
    """Grant the feature only to its exact package, policy and closed owner set."""
    if scope.policy != ENGINE_KE_POLICY and scope.subjects != ENGINE_KE_SUBJECTS:
        return ()
    if (type(scope) is not Scope or scope.package != "nevrotick-engine"
            or scope.policy != ENGINE_KE_POLICY or scope.subjects != ENGINE_KE_SUBJECTS):
        raise Incomplete("Engine KE feature requires its exact closed analysis scope.")
    return ("ke-association-prototype",)


def engine_ke_role_facts() -> dict:
    """Keep the full critical inventory separate from the reached GNU selection."""
    return {
        "schema_version": 1, "package": "nevrotick-engine",
        "admission_status": "admitted-engine-ke-source-analysis",
        "subjects": dict(ENGINE_KE_SUBJECTS), "critical_files": list(ENGINE_KE_CRITICAL),
        "features": ["ke-association-prototype"], "test_workspace": False,
        "tool_admission_profile": "stable-core",
        "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
        "maximum_mutants": 512, "maximum_listed_per_file": 512, "maximum_listed_total": 3072,
        "baseline_wall_seconds": 300, "coverage_wall_seconds": 300,
        "enumeration_wall_seconds": 240, "mutation_wall_seconds": 1200,
        "mutation_build_seconds": 90, "mutation_test_seconds": 60,
        "jobs": 6, "test_threads": 1, "coverage_test_threads": 1,
        "mutation_jobs": 6, "mutation_process_limit": 512, "mutation_message_format": "json",
        "minimum_free_bytes": 5 * 1024**3, "stop_free_bytes": 4 * 1024**3,
        "owned_output_maximum_allocated_bytes": 768 * 1024**2,
        "owned_output_maximum_entries": 32768, "mutation_equivalences": [],
    }


def require_engine_ke_role(scope: Scope, policy: dict) -> None:
    """Refuse a renamed scope, weaker gate, foreign feature or premature campaign."""
    engine_ke_features(scope)
    facts = engine_ke_role_facts()
    if (not engine_ke_features(scope) or type(policy) is not dict
            or set(policy) != set(facts) | {"binding", "mutation_inventories", "campaigns_admitted"}
            or any(type(policy.get(key)) is not type(value) or policy[key] != value
                   for key, value in facts.items() if key != "mutation_equivalences")
            or not engine_ke_equivalences_valid(policy)
            or type(policy["campaigns_admitted"]) is not bool):
        raise Incomplete("Engine KE analysis requires its exact role, feature and unchanged gates.")
    reviewed = policy["mutation_inventories"]
    if type(reviewed) is not dict or (reviewed and set(reviewed) != set(dict(ENGINE_KE_SUBJECTS))):
        raise Incomplete("Engine KE reviewed inventory must cover all selected host owners.")
    if any(type(spec) is not dict or set(spec) != {"count", "sha256"}
           or type(spec["count"]) is not int or not 0 < spec["count"] <= facts["maximum_listed_per_file"]
           or type(spec["sha256"]) is not str or re.fullmatch(r"[a-f0-9]{64}", spec["sha256"]) is None
           for spec in reviewed.values()):
        raise Incomplete("Engine KE reviewed inventory records need exact positive counts and hashes.")
    if policy["campaigns_admitted"] and not reviewed:
        raise Incomplete("Engine KE campaigns require the independently reviewed complete enumeration.")

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


def _phase_step_scope(scope: Scope) -> bool:
    """Recognize only the complete code-owned phase/Step role, never a lookalike."""
    return (type(scope) is Scope and scope.package == "time-core"
            and scope.policy == "policy/phase-step-analysis.json" and scope.subjects == PHASE_STEP_SUBJECTS)


def admit_reviewed_mutation_inventories(scope: Scope, policy: dict, inventories: dict) -> dict | None:
    """Check an optional complete scope map against every selected actual inventory.

    Hash the name-keyed admitted descriptors, preserving every member except the
    optional top-level rendered diff. No campaign is launched by this check.
    """
    if "mutation_inventories" not in policy:
        return None
    phase_step = _phase_step_scope(scope)
    if phase_step and not phase_step_equivalences_valid(policy):
        raise Incomplete("Phase-step inventory has an unreviewed equivalence catalog.")
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
        if phase_step:
            if any(item.get("file") != scope.select("all")[name] for item in inventory.values()):
                raise Incomplete("Phase-step inventory contains a foreign owner file.")
            for entry in policy["mutation_equivalences"]:
                if entry["mutant"]["file"] != scope.select("all")[name]:
                    continue
                item = inventory.get(entry["mutant"]["name"])
                facts = PHASE_STEP_EQUIVALENCE_REVIEWS[entry["id"]]
                if (type(item) is not dict or type(item.get("diff")) is not str
                        or hashlib.sha256(item["diff"].encode()).hexdigest() != facts["diff_sha256"]):
                    raise Incomplete("Phase-step equivalence differs from its exact rendered inventory diff.")
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
    phase_step = _phase_step_scope(scope)
    if phase_step and not phase_step_equivalences_valid(policy):
        raise Incomplete("Phase-step equivalences differ from the closed current-source reviews.")
    root = root.resolve(strict=True)
    product_identity = PRODUCT.admit_product(root)[1]
    candidate = canonical_sha256(product_identity)
    if phase_step and any(product_identity.get(name) != digest
            for facts in PHASE_STEP_EQUIVALENCE_REVIEWS.values()
            for name, digest in {**facts["dependencies"], **facts.get("product_dependencies", {})}.items()):
        raise Incomplete("Phase-step equivalence dependency differs from its current product bytes.")
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
                or not isinstance(function["function_name"], str)
                or not 0 < len(function["function_name"]) <= 4096
                # Native cargo-mutants uses an empty return type for implicit unit.
                or not isinstance(function["return_type"], str) or len(function["return_type"]) > 4096
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
        if phase_step and proof["path"] == PHASE_STEP_EQUIVALENCE_PROOF:
            data = read_analysis_bytes(artifact, maximum=2 * 1024 * 1024)
            if (hashlib.sha256(data).hexdigest() != proof["sha256"]
                    or len(re.findall(r"^#### " + re.escape(identifier) + r"(?:\s|$)",
                                      data.decode("utf-8"), re.MULTILINE)) != 1):
                raise Incomplete("Phase-step review bytes or unique exact section differ.")
        elif (artifact.resolve(strict=True) != artifact or artifact.stat().st_size > 65536
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
                     "--verbose", "--package=" + scope.package + "@0.0.0",
                     *("--features=" + value for value in engine_ke_features(scope)), "--frozen"]
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
    A complete nonempty all-unviable cohort is unscored only in explicit
    deferred mode; it never satisfies the mutation gate.
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
    if not viable and defer_gate is not True:
        raise Incomplete("Mutation qualification has no viable denominator or unresolved timeouts.")
    adjusted_denominator = viable - len(excluded)
    if viable and not adjusted_denominator:
        raise Incomplete("Adjusted mutation denominator is empty.")
    percent = 100 * counts["CaughtMutant"] / viable if viable else None
    adjusted = 100 * counts["CaughtMutant"] / adjusted_denominator if adjusted_denominator else None
    unresolved = sorted(set(missed) - excluded_names)
    passed = adjusted is not None and adjusted >= 95 and not unresolved
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


def mutation_jobserver_tasks(scope: Scope, policy: dict, *, workers: int) -> int:
    """Keep legacy tokens unchanged; this exact GNU role adds no shared tokens.

    Cargo supplies each child its own implicit slot. The admitted Unix
    cargo-mutants/jobserver implementation accepts an empty token pipe; zero
    additional shared tokens preserves one inner Cargo slot per outer worker.
    This is not a Windows jobserver profile or an observed concurrency result.
    """
    mutation_jobs({"mutation_jobs": workers})
    if scope.policy == "policy/phase-step-analysis.json" or scope.subjects == PHASE_STEP_SUBJECTS:
        require_phase_step_role(scope, policy)
        if workers != mutation_jobs(policy):
            raise Incomplete("Phase-step caller workers differ from its exact admitted profile.")
        return policy["mutation_jobserver_tasks"]
    if "mutation_jobserver_tasks" in policy:
        raise Incomplete("Only the exact GNU phase-step role admits an extra-token override.")
    return workers


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


def chart_composition_equivalences_valid(entries: object) -> bool:
    """Recognize only no E or the one complete checked-type-domain proof.

    Actual candidate, source and proof custody remain independently checked by
    admit_mutation_equivalences. Inventory callers must still require no E.
    """
    if type(entries) is list and not entries:
        return True
    required = {'id', 'candidate_sha256', 'source_sha256', 'mutant',
                'requirement', 'rationale', 'proof'}
    if type(entries) is not list or len(entries) != 1 or type(entries[0]) is not dict:
        return False
    entry = entries[0]
    expected = {
        'id': 'S14B-R-E-001',
        'candidate_sha256': '6f5471fe21d35455a0be6b8f85800cb44d2b1e2f8effd8aec3492e840f0bf8c6',
        'source_sha256': '1aa537417ef06238d85e7614124e399a301f1e8ecd8e6fdf74b6506e58e395bc',
        'requirement': 'CHECK-7/8/14/25'}
    proof = {'path': 'tasks/s1.4b-chart-composition-equivalence.md',
             'sha256': '67a3b6eb23ef5f476d7f7af09ffb803863a470d2c9bafd3735e28f1397046231',
             'section': 'S14B-R-E-001', 'reviewer': 'root; roundoff_survivor_review',
             'verdict': 'independently-justified-equivalent'}
    if (set(entry) != required
            or any(type(entry.get(key)) is not str or entry[key] != value for key, value in expected.items())
            or type(entry.get('rationale')) is not str or not entry['rationale'].strip()
            or len(entry['rationale']) > 4096 or type(entry.get('mutant')) is not dict
            or type(entry.get('proof')) is not dict or entry['proof'] != proof):
        return False
    try:
        return canonical_sha256(entry['mutant']) == '34d87ebc1eb820b9790f382f344e41534ea2bddaf5ebd3254e691749e97cd2a7'
    except (TypeError, ValueError, RecursionError):
        return False


def require_domain_leap_role(scope: Scope, policy: dict) -> None:
    """Admit the same exact domain role at tool and mutation-command boundaries."""
    facts = {
        "schema_version": 4, "package": "time-domain", "source_file_count": 166,
        "critical_files": list(dict(DOMAIN_LEAP_SUBJECTS).values()),
        "tool_admission_profile": "stable-core", "mutation_equivalences": [],
        "mutation_error_values": {name: [DOMAIN_LEAP_ERROR_VALUE] for name in DOMAIN_LEAP_ERROR_SUBJECTS},
        "maximum_mutants": 512,
        "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
        "coverage_wall_seconds": 300, "mutation_wall_seconds": 1200,
        "mutation_build_seconds": 90, "mutation_test_seconds": 60,
        "contracts_wall_seconds": 300, "jobs": 6, "test_threads": 12,
        "mutation_jobs": 10, "mutation_process_limit": 512, "mutation_message_format": "json",
    }
    if (type(scope) is not Scope
            or type(scope.package) is not str or scope.package != "time-domain"
            or type(scope.policy) is not str or scope.policy != "policy/time-domain-analysis.json"
            or type(scope.subjects) is not tuple or scope.subjects != DOMAIN_LEAP_SUBJECTS
            or type(policy) is not dict
            or any(type(policy.get(key)) is not type(value) or policy[key] != value
                   for key, value in facts.items())):
        raise Incomplete("Leap-domain analysis requires its exact nine-owner stable role and literal error map.")


def per_subject_mutation_error_args(scope: Scope, filename: str, policy: dict) -> list[str]:
    """Permit only the fixed literal errors for their exact reviewed owner roles.

    An absent field preserves legacy scopes. Current chart/reference roles need
    explicit empty maps; domain leap selects only its bundled-data adapter.
    A present field admits no
    arbitrary Rust expression, another package/file, or renamed policy role.
    """
    if not isinstance(policy, dict):
        raise Incomplete("Mutation error policy is not a mapping.")
    if (type(scope) is Scope and (scope.subjects == DOMAIN_LEAP_SUBJECTS
            or (scope.policy == "policy/time-domain-analysis.json" and policy.get("schema_version") == 4))):
        require_domain_leap_role(scope, policy)
        if type(filename) is not str or filename not in dict(DOMAIN_LEAP_SUBJECTS).values():
            raise Incomplete("Leap-domain mutation identifies an unreviewed source file.")
        selected = {dict(DOMAIN_LEAP_SUBJECTS)[name] for name in DOMAIN_LEAP_ERROR_SUBJECTS}
        return ["--error", DOMAIN_LEAP_ERROR_VALUE] if filename in selected else []
    settled_rate_role = (
        policy.get("admission_status") == "admitted-settled-rate-four-owner-inventory-only"
        or (type(scope) is Scope and (
            scope.subjects == SETTLED_RATE_SUBJECTS
            or scope.policy == "tasks/settled-rate-inventory-admission.json"
            or (scope.policy == "policy/time-core-analysis.json"
                and (policy.get("schema_version") == 26
                     or policy.get("critical_files") == list(dict(SETTLED_RATE_SUBJECTS).values()))))))
    if "mutation_error_values" not in policy:
        if (settled_rate_role or policy.get("admission_status") in {
                    "admitted-retained-chart-three-owner-inventory-only",
                    "admitted-chart-combination-nine-owner-inventory-only",
                    "admitted-reference-input-single-owner-inventory-only",
                    "admitted-achieved-phase-single-owner-inventory-only",
                    "admitted-settled-rate-four-owner-inventory-only",
                    "admitted-chart-composition-three-owner-inventory-only"}
                or (type(scope) is Scope and (
                    scope.subjects == CHART_PRINCIPAL_SUBJECTS
                    or scope.subjects == CHART_COMBINATION_SUBJECTS
                    or scope.subjects == REFERENCE_INPUT_SUBJECTS
                    or scope.subjects == SETTLED_RATE_SUBJECTS
                    or scope.subjects == CHART_COMPOSITION_SUBJECTS
                    or scope.policy == "tasks/s1.3-chart-principal-inventory-admission.json"
                    or scope.policy == "tasks/s1.3-chart-combination-inventory-admission.json"
                    or scope.policy == "tasks/s1.4a-reference-input-inventory-admission.json"
                    or scope.policy == "tasks/s1.4b-chart-composition-inventory-admission.json"
                    or scope.policy == "tasks/s1.4c-achieved-phase-inventory-admission.json"
                    or scope.policy == "tasks/settled-rate-inventory-admission.json"
                    or (scope.package == "time-core" and scope.subjects == CHART_BATCH_SUBJECTS)))):
            raise Incomplete("S155/S157/S158/S1.4a/S1.4b/S1.4c/settled-rate require an explicit empty mutation error map.")
        return []
    values = policy["mutation_error_values"]
    if (type(policy) is dict and type(scope) is Scope
            and type(scope.package) is str and scope.package == "time-core"
            and type(scope.subjects) is tuple and scope.subjects == SETTLED_RATE_SUBJECTS
            and type(scope.policy) is str
            and type(policy.get("admission_status")) is str
            and type(policy.get("schema_version")) is int
            and type(policy.get("source_file_count")) is int and policy["source_file_count"] == 158
            and ((scope.policy == "tasks/settled-rate-inventory-admission.json"
                  and policy.get("admission_status") == "admitted-settled-rate-four-owner-inventory-only"
                  and policy["schema_version"] == 17
                  and type(policy.get("subjects")) is dict and policy["subjects"] == dict(SETTLED_RATE_SUBJECTS))
                 or (scope.policy == "policy/time-core-analysis.json"
                     and policy.get("admission_status") == "admitted-final-source"
                     and policy["schema_version"] == 26
                     and type(policy.get("critical_files")) is list
                     and policy["critical_files"] == list(dict(SETTLED_RATE_SUBJECTS).values())))
            and type(policy.get("package")) is str and policy["package"] == "time-core"
            and type(policy.get("tool_admission_profile")) is str and policy["tool_admission_profile"] == "stable-core"
            and type(policy.get("mutation_jobs")) is int and policy["mutation_jobs"] == 10
            and type(policy.get("mutation_process_limit")) is int and policy["mutation_process_limit"] == 512
            and type(policy.get("mutation_message_format")) is str and policy["mutation_message_format"] == "json"
            and type(policy.get("maximum_mutants")) is int and policy["maximum_mutants"] == 512
            and type(policy.get("mutation_equivalences")) is list and not policy["mutation_equivalences"]
            and type(filename) is str and filename in dict(SETTLED_RATE_SUBJECTS).values()
            and type(values) is dict and not values):
        return []
    if settled_rate_role:
        raise Incomplete("Settled-rate error policy requires its exact current four-owner role and empty maps.")
    if (type(scope) is Scope and type(scope.package) is str and scope.package == "time-core"
            and type(scope.subjects) is tuple and scope.subjects == CHART_COMPOSITION_SUBJECTS
            and type(scope.policy) is str
            and ((scope.policy == "tasks/s1.4b-chart-composition-inventory-admission.json"
                  and policy.get("admission_status") == "admitted-chart-composition-three-owner-inventory-only"
                  and type(policy.get("schema_version")) is int and policy["schema_version"] == 15)
                 or (scope.policy == "policy/time-core-analysis.json"
                     and policy.get("admission_status") == "admitted-final-source"
                     and type(policy.get("schema_version")) is int and policy["schema_version"] == 24))
            and type(policy.get("source_file_count")) is int and policy["source_file_count"] == 156
            and type(policy.get("package")) is str and policy["package"] == "time-core"
            and (type(policy.get("mutation_equivalences")) is list and not policy["mutation_equivalences"]
                 or (scope.policy == "policy/time-core-analysis.json"
                     and chart_composition_equivalences_valid(policy.get("mutation_equivalences"))))
            and type(filename) is str and filename in dict(CHART_COMPOSITION_SUBJECTS).values()
            and type(values) is dict and not values):
        return []
    if (type(scope) is Scope and type(scope.package) is str and scope.package == "time-core"
            and type(scope.subjects) is tuple and scope.subjects == REFERENCE_INPUT_SUBJECTS
            and type(scope.policy) is str
            and type(policy.get("schema_version")) is int
            and type(policy.get("source_file_count")) is int
            and ((scope.policy == "tasks/s1.4a-reference-input-inventory-admission.json"
                  and policy.get("admission_status") == "admitted-reference-input-single-owner-inventory-only"
                  and policy["schema_version"] == 14 and policy["source_file_count"] == 154)
                 or (scope.policy == "tasks/s1.4c-achieved-phase-inventory-admission.json"
                     and policy.get("admission_status") == "admitted-achieved-phase-single-owner-inventory-only"
                     and policy["schema_version"] == 16 and policy["source_file_count"] == 156)
                 or (scope.policy == "policy/time-core-analysis.json"
                     and policy.get("admission_status") == "admitted-final-source"
                     and (policy["schema_version"], policy["source_file_count"]) in {(23, 154), (25, 156)}))
            and type(policy.get("package")) is str and policy["package"] == "time-core"
            and type(policy.get("mutation_equivalences")) is list and not policy["mutation_equivalences"]
            and type(filename) is str and filename == dict(REFERENCE_INPUT_SUBJECTS)["reference_input"]
            and type(values) is dict and not values):
        return []
    if (type(scope) is Scope and type(scope.package) is str and scope.package == "time-core"
            and type(scope.subjects) is tuple and scope.subjects == CHART_COMBINATION_SUBJECTS
            and type(scope.policy) is str
            and ((scope.policy == "tasks/s1.3-chart-combination-inventory-admission.json"
                  and policy.get("admission_status") == "admitted-chart-combination-nine-owner-inventory-only")
                 or (scope.policy == "policy/time-core-analysis.json"
                     and policy.get("admission_status") == "admitted-final-source"))
            and type(policy.get("package")) is str and policy["package"] == "time-core"
            and type(policy.get("mutation_equivalences")) is list
            and (not policy["mutation_equivalences"]
                 or (scope.policy == "policy/time-core-analysis.json"
                     and policy.get("admission_status") == "admitted-final-source"))
            and type(filename) is str and filename in dict(CHART_COMBINATION_SUBJECTS).values()
            and type(values) is dict and not values):
        return []
    if (type(scope) is Scope and scope.package == "time-core"
            and type(scope.subjects) is tuple and scope.subjects == CHART_PRINCIPAL_SUBJECTS
            and type(scope.policy) is str and scope.policy in {
                "policy/time-core-analysis.json", "tasks/s1.3-chart-principal-inventory-admission.json"}
            and type(policy.get("package")) is str and policy["package"] == "time-core"
            and type(filename) is str and filename in dict(CHART_PRINCIPAL_SUBJECTS).values()
            and type(values) is dict and not values):
        return []
    if (type(scope) is Scope and scope.package == "time-core"
            and type(scope.subjects) is tuple and scope.subjects == CHART_BATCH_SUBJECTS
            and type(scope.policy) is str and scope.policy in {
                "policy/time-core-analysis.json", "tasks/s1.2-chart-batch-inventory-admission.json"}
            and type(policy.get("package")) is str and policy["package"] == "time-core"
            and type(filename) is str and filename == dict(CHART_BATCH_SUBJECTS)["chart_batch"]
            and type(values) is dict and not values):
        return []
    if (type(scope) is not Scope or type(scope.package) is not str or scope.package != "time-core"
            or type(policy.get("package")) is not str or policy["package"] != "time-core"
            or type(scope.subjects) is not tuple or scope.subjects != CHART_ERROR_SUBJECTS
            or type(scope.policy) is not str or scope.policy not in {"policy/time-core-analysis.json",
                                    "tasks/s1.2-chart-inventory-admission-v2.json"}
            or type(filename) is not str or filename not in dict(CHART_ERROR_SUBJECTS).values()
            or type(values) is not dict or set(values) != {"chart_enclosure"}
            or type(values["chart_enclosure"]) is not list or len(values["chart_enclosure"]) != 1
            or type(values["chart_enclosure"][0]) is not str
            or values["chart_enclosure"][0] != CHART_ENCLOSURE_ERROR_VALUE):
        raise Incomplete("Mutation error values differ from the exact chart-enclosure scope and expression.")
    return (["--error", CHART_ENCLOSURE_ERROR_VALUE]
            if filename == dict(CHART_ERROR_SUBJECTS)["chart_enclosure"] else [])


def mutation_campaign_command(scope: Scope, tools: Path, subject: str, command: list,
                              *, expected_jobs=RUNTIME.MAX_JOBS, expected_message_format="human",
                              policy=None) -> None:
    """Reject command substitution before a mutation subprocess may be launched."""
    jobs = mutation_jobs({"mutation_jobs": expected_jobs})
    message_format = mutation_message_format({"mutation_message_format": expected_message_format})
    filename = scope.select(subject)[subject]
    if scope.policy == "policy/phase-step-analysis.json" or scope.subjects == PHASE_STEP_SUBJECTS:
        if policy is None:
            raise Incomplete("Phase-step campaign requires its exact current role policy.")
        require_phase_step_role(scope, policy)
        if not policy["campaigns_admitted"]:
            raise Incomplete("Phase-step mutation admission is closed until contracts and complete inventories are reviewed.")
    tokens = mutation_jobserver_tasks(scope, {} if policy is None else policy, workers=jobs)
    expected = [str(tools / "cargo-mutants"), "mutants", "--package", scope.package, "--file", filename,
                "--test-workspace", "false", "--cap-lints", "false", "--jobs", str(jobs),
                "--jobserver", "true", "--jobserver-tasks", str(tokens)]
    if message_format == "json":
        expected.append("--cargo-arg=--message-format=json")
    for feature in engine_ke_features(scope):
        expected.extend(["--features", feature])
    if engine_ke_features(scope):
        if policy is None:
            raise Incomplete("Engine KE campaign requires its reviewed role policy.")
        require_engine_ke_role(scope, policy)
        if jobs != mutation_jobs(policy) or message_format != mutation_message_format(policy):
            raise Incomplete("Engine KE caller parameters differ from the actual role policy.")
        if not policy["campaigns_admitted"] or not policy["mutation_inventories"]:
            raise Incomplete("Engine KE campaign admission remains closed at the effect boundary.")
    expected.extend(per_subject_mutation_error_args(scope, filename, {} if policy is None else policy))
    domain_campaign = (type(scope) is Scope and scope.package == "time-domain"
                       and scope.policy == "policy/time-domain-analysis.json"
                       and scope.subjects == DOMAIN_LEAP_SUBJECTS)
    if (policy is not None and "mutation_error_values" in policy
            and scope.policy != "policy/time-core-analysis.json" and not domain_campaign):
        raise Incomplete("The chart error inventory card admits listing only, never a campaign.")
    expected.extend(["--baseline", "run", "--build-timeout", "90", "--timeout", "60",
                     "--output", "/work/mutation/" + subject])
    if command != expected:
        raise Incomplete("Mutation command is not the exact admitted campaign invocation.")


def mutation_campaign_result(scope: Scope, tools: Path, subject: str, command: list, result: dict,
                             *, expected_jobs=RUNTIME.MAX_JOBS, expected_message_format="human",
                             policy=None) -> dict:
    """Retain provisional native 0/2 only for complete-report reconciliation.

    Exit 2 is never a command success. Setup/errors/baseline/timeout/signal
    statuses remain immediate incomplete results.
    """
    mutation_campaign_command(scope, tools, subject, command, expected_jobs=expected_jobs,
                              expected_message_format=expected_message_format, policy=policy)
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


def copy_cargo_contract_dependencies(session, output: str, libraries: dict[str, str]) -> Path:
    """Seal only attributed debug host-build libraries, on either Cargo layout.

    The invocation is a frozen ordinary host `cargo build --lib`, without a
    target override. The sealed lock/manifests own package and target identity;
    Cargo messages own the emitted paths. Complete rlib/rmeta units and proc
    macros are required. This is not a reconstruction of optional-feature
    resolution: an external consumer must still reach its intended contract.
    """
    registry = "registry+https://github.com/rust-lang/crates.io-index"
    locked = tomllib.loads((session.source / "Cargo.lock").read_text())["package"]
    copied, units, emitted = {}, set(), {}
    def unique_pairs(pairs):
        result = {}
        for key, value in pairs:
            if key in result:
                raise Incomplete("Duplicate Cargo artifact JSON key.")
            result[key] = value
        return result
    def invalid_constant(_value):
        raise Incomplete("Nonfinite Cargo artifact JSON value.")
    def manifest_document(path, root):
        try:
            if (path.resolve(strict=True) != path or not path.is_file()
                    or not path.is_relative_to(root)):
                raise Incomplete("Cargo artifact manifest is redirected or outside its admitted root.")
            return tomllib.loads(path.read_text())
        except (OSError, ValueError, UnicodeError) as error:
            raise Incomplete("Cargo artifact manifest is unavailable or invalid.") from error
    for line in output.splitlines():
        if not line.startswith("{"):
            continue
        if len(line.encode()) > 1024 * 1024:
            raise Incomplete("Cargo artifact record exceeds its byte budget.")
        try:
            item = json.loads(line, object_pairs_hook=unique_pairs, parse_constant=invalid_constant)
        except (ValueError, RecursionError) as error:
            raise Incomplete("Malformed Cargo artifact JSON.") from error
        if type(item) is not dict:
            raise Incomplete("Cargo artifact record is not an object.")
        if item.get("reason") != "compiler-artifact":
            continue
        manifest, identifier = item.get("manifest_path"), item.get("package_id")
        if (type(manifest) is not str or type(identifier) is not str
                or str(Path(manifest)) != manifest or not Path(manifest).is_absolute()
                or ".." in Path(manifest).parts or Path(manifest).name != "Cargo.toml"):
            raise Incomplete("Cargo artifact lacks a package/manifest identity.")
        if manifest.startswith("/work/source/") and ".." not in Path(manifest).parts:
            local_manifest = session.source / manifest.removeprefix("/work/source/")
            declared = manifest_document(local_manifest, session.source)
            owners = [row for row in locked if "source" not in row
                      and row["name"] == declared.get("package", {}).get("name")
                      and identifier == "path+file://" + str(Path(manifest).parent) + "#" + row["version"]]
        else:
            owners = [row for row in locked if row.get("source") == registry
                      and identifier == registry + "#" + row["name"] + "@" + row["version"]
                      and manifest == str(session.vendor / (row["name"] + "-" + row["version"]) / "Cargo.toml")]
            local_manifest = Path(manifest)
        if len(owners) != 1:
            raise Incomplete("Cargo artifact package/manifest is outside the frozen lock.")
        owner = owners[0]
        declared = manifest_document(local_manifest, session.source if manifest.startswith("/work/source/") else session.vendor)
        if (declared.get("package", {}).get("name") != owner["name"]
                or declared.get("package", {}).get("version") != owner["version"]):
            raise Incomplete("Cargo artifact manifest differs from its locked package.")
        target, profile, files = item.get("target", {}), item.get("profile", {}), item.get("filenames")
        if (type(target) is not dict or type(profile) is not dict or type(files) is not list
                or not files or any(type(path) is not str for path in files)
                or type(target.get("name")) is not str
                or len(set(files)) != len(files)
                or profile.get("test") is not False or profile.get("opt_level") != "0"
                or profile.get("debug_assertions") is not True or profile.get("overflow_checks") is not True):
            raise Incomplete("Cargo artifact has the wrong debug build profile or file inventory.")
        kind = target.get("kind")
        unit = (identifier, target.get("name"), str(kind))
        if unit in units:
            raise Incomplete("Duplicate Cargo artifact compilation unit.")
        units.add(unit)
        package = re.escape(owner["name"])
        new_root = r"/work/target/debug/build/" + package + r"/[a-f0-9]{16}/out/"
        if kind == ["custom-build"]:
            build_path = declared["package"].get("build", "build.rs")
            if (type(build_path) is not str or target.get("name") != "build-script-build"
                    or target.get("crate_types") != ["bin"]
                    or target.get("src_path") != str(Path(manifest).parent / build_path)
                    or len(files) != 1
                    or not re.fullmatch(r"(?:/work/target/debug/build/" + package
                        + r"-[a-f0-9]{16}/build-script-build|" + new_root
                        + r"build_script_build(?:-[a-f0-9]{16})?)", files[0])):
                raise Incomplete("Cargo build-script artifact has the wrong owner/layout.")
            continue
        library = declared.get("lib", {})
        macro = library.get("proc-macro", False)
        expected_kind = "proc-macro" if macro else "lib"
        crate = library.get("name", owner["name"].replace("-", "_"))
        if (type(macro) is not bool or kind != [expected_kind] or target.get("crate_types") != [expected_kind]
                or target.get("name") != crate
                or target.get("src_path") != str(Path(manifest).parent / library.get("path", "src/lib.rs"))):
            raise Incomplete("Cargo library artifact differs from its declared target.")
        suffixes = {".so"} if macro else {".rlib", ".rmeta"}
        if len(files) != len(suffixes) or {Path(path).suffix for path in files} != suffixes:
            raise Incomplete("Cargo library/proc-macro artifact inventory is incomplete.")
        for path in files:
            # Root rlib exports coexist with intermediate metadata; no recursive
            # directory walk or un-attributed search input is admitted.
            intermediate = r"(?:/work/target/debug/deps/|" + new_root + r")lib" + re.escape(crate) + r"-[a-f0-9]{16}\.(?:rlib|rmeta|so)"
            exported = "/work/target/debug/lib" + crate + ".rlib"
            if not re.fullmatch(intermediate, path) and not (not macro and path == exported):
                raise Incomplete("Cargo library artifact has the wrong package/host root.")
            if path in emitted or Path(path).name in copied:
                raise Incomplete("Cargo artifact path or copied basename is ambiguous.")
            emitted[path] = crate; copied[Path(path).name] = path
    if (not libraries or any(type(name) is not str or not re.fullmatch(r"lib[A-Za-z0-9_]+\.rlib", name)
            or type(emitted_path) is not str or emitted_path not in emitted
            or Path(emitted_path).suffix != ".rlib" or name != "lib" + emitted[emitted_path] + ".rlib"
            for name, emitted_path in libraries.items())):
        raise Incomplete("Required external-contract library is absent from Cargo attribution.")
    for name, emitted_path in libraries.items():
        if name in copied and copied[name] != emitted_path:
            raise Incomplete("External-contract library alias collides with another artifact.")
        copied[name] = emitted_path
    destination = session.work / "contract-deps"
    destination.mkdir(mode=0o700)
    try:
        for name, path in sorted(copied.items()):
            source = session.work / path.removeprefix("/work/")
            if (source.resolve(strict=True) != source or not source.is_file()
                    or not source.is_relative_to(session.work / "target")):
                raise Incomplete("Cargo contract dependency is redirected or escaped.")
            digest = PRODUCT.sha256(source)
            target = destination / name
            shutil.copyfile(source, target); target.chmod(0o400)
            if (PRODUCT.sha256(source) != digest or PRODUCT.sha256(target) != digest
                    or target.stat().st_nlink != 1
                    or (target.stat().st_dev, target.stat().st_ino) == (source.stat().st_dev, source.stat().st_ino)):
                raise Incomplete("Contract dependency changed or aliases its immutable copy.")
    except OSError as error:
        raise Incomplete("Cargo contract dependency inventory is unavailable.") from error
    session.protect(destination)
    return destination


def compiler_cause(result: dict, specimen: dict, source_path: str) -> None:
    """Validate every rustc record and require every error's intended cause.

    The admitted external probes have no macro expansion spans. Owned
    unused-import/dead-code warnings are informational. Only an exact uncoded
    abort summary with matching error/warning counts and its exact explain note
    may accompany the causal errors; neither can establish a compiler cause.
    Multiple matching errors are legitimate (one existing field-mutation probe
    emits two). Rendering and error explanations never establish meaning.
    """
    if (type(result) is not dict or not {"output", "exit_code", "failure"} <= result.keys()
            or type(specimen) is not dict):
        raise Incomplete("External consumer result or intended diagnostic is malformed.")
    output = result.get("output")
    if (result.get("failure") is not None or type(result.get("exit_code")) is not int
            or result["exit_code"] <= 0 or type(output) is not str
            or not output or len(output) > 4 * 1024 * 1024
            or type(specimen.get("code")) is not str
            or re.fullmatch(r"E[0-9]{4}", specimen["code"]) is None
            or type(specimen.get("line")) is not int or specimen["line"] < 1
            or type(specimen.get("tokens")) is not list or not specimen["tokens"]
            or any(type(token) is not str or not token for token in specimen["tokens"])
            or type(source_path) is not str or not source_path):
        raise Incomplete("External consumer lacks a bounded intended compiler failure.")
    fields = {"message", "code", "level", "spans", "children", "rendered"}
    span_fields = {"file_name", "byte_start", "byte_end", "line_start", "line_end",
                   "column_start", "column_end", "is_primary", "text", "label",
                   "suggested_replacement", "suggestion_applicability", "expansion"}
    infrastructure = re.compile(
        r"resource temporarily unavailable|os error \d+|permission denied|could not execute process|"
        r"failed to spawn|no space left|cannot allocate memory|out of memory|"
        r"failed to run custom build command|linker|linking|internal compiler error|"
        r"unexpectedly panicked|panicked at|fatal runtime error|terminated by signal|failed to load manifest",
        re.IGNORECASE)
    inspected = 0

    def diagnostic_text(item, *, top=False, depth=0):
        """Validate the pinned diagnostic tree before reading its semantic text."""
        nonlocal inspected
        inspected += 1
        if (depth > 16 or inspected > 256 or type(item) is not dict
                or set(item) != (fields | {"$message_type"} if top else fields)
                or (top and item["$message_type"] != "diagnostic")
                or type(item["message"]) is not str or not item["message"]
                or type(item["level"]) is not str
                or item["level"] not in ({"error", "warning", "failure-note"} if top else {"note", "help"})
                or type(item["children"]) is not list or type(item["spans"]) is not list
                or (item["rendered"] is not None and type(item["rendered"]) is not str)):
            raise Incomplete("External compiler diagnostic is malformed or unqualified.")
        code = item["code"]
        if code is not None and (not top or type(code) is not dict
                or set(code) != {"code", "explanation"} or type(code["code"]) is not str
                or (code["explanation"] is not None and type(code["explanation"]) is not str)):
            raise Incomplete("External compiler diagnostic code is malformed.")
        text = item["message"]
        if len(item["spans"]) > 256:
            raise Incomplete("External compiler diagnostic span count exceeds its bound.")
        for span in item["spans"]:
            if (type(span) is not dict or set(span) != span_fields
                    or type(span["file_name"]) is not str or not span["file_name"]
                    or type(span["is_primary"]) is not bool
                    or any(type(span[key]) is not int or span[key] < minimum for key, minimum in (
                        ("byte_start", 0), ("byte_end", 0), ("line_start", 1),
                        ("line_end", 1), ("column_start", 1), ("column_end", 1)))
                    or span["byte_end"] < span["byte_start"] or span["line_end"] < span["line_start"]
                    or any(span[key] is not None and type(span[key]) is not str for key in (
                        "label", "suggested_replacement", "suggestion_applicability"))
                    or span["expansion"] is not None or type(span["text"]) is not list
                    or len(span["text"]) > 256):
                raise Incomplete("External compiler diagnostic span is malformed.")
            for source_line in span["text"]:
                if (type(source_line) is not dict or set(source_line) != {"text", "highlight_start", "highlight_end"}
                        or type(source_line["text"]) is not str
                        or any(type(source_line[key]) is not int or source_line[key] < 1
                               for key in ("highlight_start", "highlight_end"))
                        or source_line["highlight_end"] < source_line["highlight_start"]):
                    raise Incomplete("External compiler diagnostic source text is malformed.")
            text += " " + (span["label"] or "")
        text += " " + " ".join(diagnostic_text(child, depth=depth + 1) for child in item["children"])
        if infrastructure.search(text):
            raise Incomplete("External compiler diagnostic contains a tool or infrastructure failure.")
        return text

    errors = warnings = 0
    summarized = explained = False
    records = output.splitlines()
    if len(records) > 256:
        raise Incomplete("External compiler diagnostic record count exceeds its bound.")
    for line in records:
        try:
            item = ANALYSIS.decode(line)
        except (Incomplete, RecursionError) as error:
            raise Incomplete("External compiler output contains malformed or non-JSON records.") from error
        text = diagnostic_text(item, top=True)
        code, level = item["code"], item["level"]
        if level == "error" and code is not None:
            primary = [span for span in item["spans"] if span["is_primary"]]
            if (summarized or explained or code["code"] != specimen["code"]
                    or not all(token in text for token in specimen["tokens"]) or not primary
                    or any(span["file_name"] != source_path or span["line_start"] != specimen["line"]
                           for span in primary)):
                raise Incomplete("External consumer diagnostic has an unrelated code, cause or primary span.")
            errors += 1
        elif level == "warning":
            if (summarized or explained or code is None or code["code"] not in {"unused_imports", "dead_code"}
                    or not item["spans"]
                    or any(span["file_name"] != source_path for span in item["spans"])):
                raise Incomplete("External compiler warning is outside the admitted fixture lint scope.")
            warnings += 1
        elif level == "error" and code is None:
            expected = f"aborting due to {errors} previous error" + ("s" if errors != 1 else "")
            if warnings:
                expected += f"; {warnings} warning" + ("s" if warnings != 1 else "") + " emitted"
            if (not errors or summarized or explained or item["message"] != expected
                    or item["spans"] or item["children"]):
                raise Incomplete("External compiler uncoded error is not its exact counted abort summary.")
            summarized = True
        elif level == "failure-note":
            expected = f"For more information about this error, try `rustc --explain {specimen['code']}`."
            if (not summarized or explained or code is not None or item["message"] != expected
                    or item["spans"] or item["children"]):
                raise Incomplete("External compiler failure-note is not the intended explain summary.")
            explained = True
    if not errors:
        raise Incomplete("External consumer failed without its intended compiler code.")


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
    this helper launches neither a baseline nor a campaign. It performs no direct
    filesystem writes; the supplied custodian/tool may retain owned output.
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
    tokens = mutation_jobserver_tasks(scope, policy, workers=workers)
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
                  "--jobserver", "true", "--jobserver-tasks", str(tokens)]
        if message_format == "json":
            common.append("--cargo-arg=--message-format=json")
        for feature in engine_ke_features(scope):
            common.extend(["--features", feature])
        common.extend(per_subject_mutation_error_args(scope, filename, policy))
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
    coverage_test_args = []
    if "coverage_test_threads" in policy:
        if type(policy["coverage_test_threads"]) is not int or policy["coverage_test_threads"] != 1:
            raise Incomplete("Coverage test-thread override must be exactly integer one.")
        coverage_test_args = ["--", "--test-threads=1"]
    if lane in {"mutation", "all"}:
        mutation_jobs(policy)
        mutation_process_limit(policy)
        mutation_message_format(policy)
    if lane in {"coverage", "all"}:
        # The pinned tool can retain Rust sysroot sources under their recorded
        # toolchain path. Keep its native threshold scoped away from that code;
        # coverage_counts still requires every exact declared owner below.
        command = [str(tools / "cargo-llvm-cov"), "llvm-cov", "--package", scope.package,
                   "--frozen", "--jobs", str(policy.get("jobs", RUNTIME.MAX_JOBS)), "--lib", "--tests", "--json",
                   "--output-path", "/work/coverage.json", "--failure-mode", "any",
                   "--ignore-filename-regex", "(^|/)tests/|/rustlib/src/rust/library/",
                   "--fail-under-file-lines", "90"]
        command.extend(coverage_test_args)
        text = run("coverage", command, policy["coverage_wall_seconds"])
        coverage = read_analysis_json(work / "coverage.json")
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
            outcomes = read_analysis_json(outcome_directory / "outcomes.json")
            selected_equivalences = [entry for entry in equivalences if entry["mutant"]["file"] == filename]
            result = mutation_counts(scope, outcomes, inventories[name], filename,
                                     native_exit_code=campaign["exit_code"], equivalences=selected_equivalences,
                                     defer_gate=True)
            baseline = next(item for item in outcomes["outcomes"] if item["scenario"] == "Baseline")
            relative_log = analysis_output_path(baseline["log_path"])
            if relative_log.is_absolute() or not relative_log.parts or ".." in relative_log.parts:
                raise Incomplete("Mutation baseline log escapes its owned output directory.")
            baseline_log = outcome_directory / relative_log
            result["baseline_tests_executed"] = test_count(read_analysis_text(baseline_log))
            report["mutation"][name] = result
        failed = [name for name, result in report["mutation"].items() if not result["gate_passed"]]
        if failed:
            raise Incomplete("Complete mutation accounting has unresolved custom gates: " + ", ".join(failed))
        print("PASSED: portable mutation", flush=True)


def _require_private_output(root, output):
    """Recheck canonical external Linux output and free space without creating it."""
    if (not output.is_absolute() or output.parent != output.parent.resolve(strict=True)
            or output.is_relative_to(root) or output.is_relative_to(Path("/mnt"))):
        raise Incomplete("Use a fresh external canonical private Linux output directory.")
    if shutil.disk_usage(output.parent).free < 2 * 1024 ** 3:
        raise Incomplete("Portable analysis requires at least 2 GiB free Linux work space.")


def require_engine_contract_role(scope: Scope, policy: dict) -> None:
    """Admit current inert constructor probes, without historical engine gates."""
    facts = {
        "schema_version": 1, "admission_status": "admitted-engine-external-contracts-only",
        "package": "nevrotick-engine", **CONTRACT_PRODUCT_BINDING,
        "critical_files": ["crates/nevrotick-engine/src/lib.rs"],
        "tool_admission_profile": "stable-core", "contracts_wall_seconds": 300,
        "jobs": 6, "test_threads": 1, "mutation_equivalences": [], "mutation_error_values": {},
    }
    if (type(scope) is not Scope or type(scope.package) is not str or scope.package != "nevrotick-engine"
            or type(scope.policy) is not str or scope.policy != "policy/engine-analysis.json"
            or type(scope.subjects) is not tuple or scope.subjects != ENGINE_CONTRACT_SUBJECTS
            or type(policy) is not dict or set(policy) != set(facts)
            or any(type(policy.get(key)) is not type(value) or policy[key] != value
                   for key, value in facts.items())):
        raise Incomplete("Engine external contracts require their exact inert library-only role.")


def domain_contract_facts() -> dict:
    """Return only the exact ordinary nine-owner compiler role, with no campaigns."""
    return {
        "schema_version": 1, "admission_status": "admitted-domain-external-contracts-only",
        "package": "time-domain", **CONTRACT_PRODUCT_BINDING,
        "critical_files": list(dict(DOMAIN_LEAP_SUBJECTS).values()),
        "tool_admission_profile": "stable-core", "contracts_wall_seconds": 300,
        "jobs": 6, "test_threads": 1, "mutation_equivalences": [], "mutation_error_values": {},
    }


def require_domain_contract_role(scope: Scope, policy: dict) -> None:
    """Reject scope aliases or fields that could borrow legacy fuzz/metric authority."""
    facts = domain_contract_facts()
    if (type(scope) is not Scope or type(scope.package) is not str or scope.package != "time-domain"
            or type(scope.policy) is not str or scope.policy != "policy/time-domain-analysis.json"
            or type(scope.subjects) is not tuple or scope.subjects != DOMAIN_LEAP_SUBJECTS
            or type(policy) is not dict or set(policy) != set(facts)
            or any(type(policy.get(k)) is not type(v) or policy[k] != v for k, v in facts.items())):
        raise Incomplete("Domain external contracts require their exact inert nine-owner role.")


def require_contract_candidate(identity: dict, policy: dict) -> None:
    """Bind every actual Product input byte and path; a matching count is insufficient."""
    if (type(identity) is not dict or len(identity) != policy["source_file_count"]
            or any(type(n) is not str or not n or len(n) > 4096 or Path(n).is_absolute()
                   or ".." in Path(n).parts or type(h) is not str or re.fullmatch(r"[a-f0-9]{64}", h) is None
                   for n, h in identity.items())
            or canonical_sha256(identity) != policy["candidate_sha256"]):
        raise Incomplete("The complete contract candidate path/byte digest differs from its reviewed snapshot.")


def frequency_prior_equivalences_valid(policy):
    """Admit only unique subsets of four current full-domain source reviews."""
    entries = policy.get("mutation_equivalences")
    if type(entries) is not list or len(entries) > 4:
        return False
    if not entries:
        return True
    fields = {"id", "candidate_sha256", "source_sha256", "mutant", "requirement", "rationale", "proof"}
    candidate = "3c8ae8f191cba09742259957d491bb362f8301a39c7506c1a029e02be4ae7e90"
    path = "tasks/phase-plan-analysis-equivalences.md"
    reviewed = {
        "FREQUENCY-PROFILE-NORMAL-POSITIVE": (
            "a9770f085170b8c9ae52a5957761f99173ee3ff02294df56cb1e8e0c8b888b48",
            "87ebc63219626344ce56ac51b015c23fb2d7e2cf8ae69695757b6ce33ba11d04",
            "Normality excludes both signed zeros, subnormals, infinities and every NaN before the comparison; "
            "positive and negative normal binary64 values give identical >0 and >=0 results. Earlier guards, "
            "short-circuit order, validation, errors and effects are unchanged. This current-source argument "
            "grants zero kills; the exact mutant must still be an actual MissedMutant."),
        "FREQUENCY-GROWTH-NORMAL-POSITIVE": (
            "a0a6af42cba3f012545d05ad5949f8934c28bd4bd65e503f695cc7fdf150c23a",
            "b5a0a6e706ec4a2cbe7f87283276804d97af40ccc03e1b1c1f8b4dbb1fe95355",
            "Normality rejects both signed zeros, subnormals, infinities and every NaN before the growth comparison; "
            "positive and negative normal binary64 values give identical >0 and >=0 results. All other profile "
            "fields, short-circuit order, validation, errors and effects are unchanged. This current-source argument "
            "grants zero kills; the exact mutant must still be an actual MissedMutant."),
        "FREQUENCY-ANCHOR-BOUND-ABS-SIGN": (
            "c29c5461fc90820de0aef29503174a64b70dc18aca131b8592b247e123ad247b",
            "22517d32dfe7600ed511c0e3e94937445c1c7079b9cbc6bc277e494927283f1c",
            "Only the coefficient-bound argument changes sign; the phase subtraction and earlier admissions are "
            "unchanged. The pinned parent coefficient_addition_error applies abs to that operand before every "
            "bound operation, yielding identical operands, error order, output fields and effects for every "
            "admitted anchor and raw effect. Signed-zero and all failure paths are unchanged. This current-source "
            "argument grants zero kills; the exact mutant must still be an actual MissedMutant."),
        "FREQUENCY-INTERVAL-WHOLE-ZERO-PRODUCT": (
            "c29c5461fc90820de0aef29503174a64b70dc18aca131b8592b247e123ad247b",
            "31b9588c3950a1a8c2069d4bc14ebf95ed991885d9ffaf8b2bf9658716181872",
            "If exactly one finite ordered interval is whole zero, the unchanged inner endpoint OR shortcut "
            "returns literal positive zero for all four pairs; reduction therefore returns the same canonical ZERO. "
            "With both or neither whole zero, the paths are identical. Outputs, refusals and effects agree over the "
            "full admitted interval domain; maximum work remains four pairs, without a timing-equality claim. "
            "This current-source argument grants zero kills; the exact mutant must still be an actual MissedMutant."),
    }
    seen = set()
    for entry in entries:
        if type(entry) is not dict or set(entry) != fields:
            return False
        identifier = entry.get("id")
        if type(identifier) is not str or identifier not in reviewed or identifier in seen:
            return False
        source, descriptor, rationale = reviewed[identifier]
        if (entry.get("candidate_sha256") != candidate or policy.get("candidate_sha256") != candidate
                or entry.get("source_sha256") != source or type(entry.get("mutant")) is not dict
                or canonical_sha256(entry["mutant"]) != descriptor
                or entry.get("requirement") != "REQ-8/10/11/14/15; CHECK-7/8/10/11/14/25; AC-19"
                or entry.get("rationale") != rationale):
            return False
        proof = entry.get("proof")
        if not (type(proof) is dict and set(proof) == {"path", "sha256", "section", "reviewer", "verdict"}
                and proof.get("path") == path and proof.get("section") == identifier
                and proof.get("reviewer") == "/root; /root/series_checkpoint_design_review; /root/series_checkpoint_verification"
                and proof.get("verdict") == "independently-justified-equivalent"
                and type(proof.get("sha256")) is str and re.fullmatch(r"[a-f0-9]{64}", proof["sha256"]) is not None
                and type(policy.get("inputs_sha256")) is dict and policy["inputs_sha256"].get(path) == proof["sha256"]):
            return False
        if (identifier == "FREQUENCY-ANCHOR-BOUND-ABS-SIGN"
                and policy["inputs_sha256"].get("crates/time-core/src/regression/chart/roundoff.rs")
                    != "4bea2d3fbd596ec5c28b7febd56d21efe42b19c7de2fdbd84f804d5451ff476f"):
            return False
        seen.add(identifier)
    return True


def frequency_prior_role_facts():
    """Return the unchanged stable four-owner facts for explicit role admission."""
    return {
        "schema_version": 1, "package": "time-core",
        "critical_files": list(dict(FREQUENCY_PRIOR_SUBJECTS).values()),
        "tool_admission_profile": "stable-core", "coverage_minimum_line_percent": 90,
        "mutation_minimum_viable_killed_percent": 95, "baseline_wall_seconds": 300,
        "coverage_wall_seconds": 300, "enumeration_wall_seconds": 240,
        "mutation_wall_seconds": 1200, "mutation_build_seconds": 90, "mutation_test_seconds": 60,
        "jobs": 6, "test_threads": 1, "coverage_test_threads": 1, "mutation_jobs": 10,
        "mutation_process_limit": 512, "mutation_message_format": "json",
        "mutation_minimum_free_bytes": 5 * 1024 ** 3, "maximum_mutants": 512,
        "maximum_listed_per_file": 512, "maximum_listed_total": 2048,
        "mutation_debug_info": 0, "contracts_wall_seconds": 300, "general_jobs": 12,
        "external_consumer_limits": {"legal_executions": 2, "negative_compilations": 18},
    }


def require_frequency_prior_role(scope, policy):
    """Admit only the four-owner stable-only child, never a historical role alias."""
    facts = frequency_prior_role_facts()
    extra = {"source_file_count", "candidate_sha256", "inputs_sha256", "mutation_inventories",
             "mutation_equivalences", "test_inventories", "scope"}
    if (type(scope) is not Scope or scope.package != "time-core"
            or scope.policy != "policy/frequency-prior-analysis.json"
            or scope.subjects != FREQUENCY_PRIOR_SUBJECTS or type(policy) is not dict
            or set(policy) != set(facts) | extra
            or any(type(policy.get(k)) is not type(v) or policy[k] != v for k, v in facts.items())
            or not frequency_prior_equivalences_valid(policy)):
        raise Incomplete("Frequency-prior requires its exact four-owner stable role and unchanged gates.")


# These ordinary source reviews apply only to the exact current five-owner role.
# A later catalog extension requires independent review and a source change here.
BASELINE_FEEDBACK_EQUIVALENCE_PROOF = "tasks/phase-plan-analysis-equivalences.md"
BASELINE_FEEDBACK_EQUIVALENCE_REQUIREMENT = "REQ-8/10/11/14/15; CHECK-7/8/10/11/14/25; AC-19"
BASELINE_FEEDBACK_EQUIVALENCE_REVIEWERS = "/root; /root/handoff_final_review; /root/handoff_runner"
BASELINE_FEEDBACK_EQUIVALENCE_REVIEWS = {'BF-STATE-GROWTH-NORMAL-POSITIVE': ('81810b88c2e856e8bcd861a53b775deee2d479f7ed494281659ed70dffd536c7',
                                     'b5a0a6e706ec4a2cbe7f87283276804d97af40ccc03e1b1c1f8b4dbb1fe95355',
                                     'Normality excludes signed zeros, subnormals, infinities and every NaN '
                                     'before the growth comparison; positive and negative normal binary64 '
                                     'values give identical >0 and >=0 results. Earlier profile fields, '
                                     'short-circuit order, validation, errors and effects are unchanged. '
                                     'This current-source review grants zero kills; the exact mutant must '
                                     'still be an actual MissedMutant.',
                                     {'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs': '81810b88c2e856e8bcd861a53b775deee2d479f7ed494281659ed70dffd536c7'}),
 'BF-STATE-REBASE-INTERMEDIATE-LABELS': ('81810b88c2e856e8bcd861a53b775deee2d479f7ed494281659ed70dffd536c7',
                                         '5c785761ecf495dad31ec98bfa6c55c6373e872a211c5dbde3867c1aa7b55508',
                                         'The unchanged checked intersection composition rejects each '
                                         'invalid intermediate label, and exact recomputed complete Basis '
                                         'matching rejects each invalid replacement endpoint, before '
                                         'anchor/prior transformation. All such composition failures map to '
                                         'AnchorRebaseUnavailable. Returned cause, model and effects agree '
                                         'over all admitted private operands; extra immutable recomposition '
                                         'stays bounded without a timing or allocation-trace equality claim. '
                                         'This current-source review grants zero kills; the exact mutant '
                                         'must still be an actual MissedMutant.',
                                         {'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs': '81810b88c2e856e8bcd861a53b775deee2d479f7ed494281659ed70dffd536c7',
                                          'crates/time-core/src/regression/chart.rs': '598f7870c27f69f1e4fe6abd7d820dd8ad85bef6b811f01b1ea90966c46c89ba'}),
 'BF-STATE-REBASE-STEP-GENERATION-SOURCE-FRAME': ('81810b88c2e856e8bcd861a53b775deee2d479f7ed494281659ed70dffd536c7',
                                                  '856113b1e9e16e87c4f72b6ec8ebbb7a2e40733b8dcc139ff932721bb5bd9d66',
                                                  'The unchanged checked intersection composition rejects '
                                                  'each invalid intermediate label, and exact recomputed '
                                                  'complete Basis matching rejects each invalid replacement '
                                                  'endpoint, before anchor/prior transformation. All such '
                                                  'composition failures map to AnchorRebaseUnavailable. '
                                                  'Returned cause, model and effects agree over all admitted '
                                                  'private operands; extra immutable recomposition stays '
                                                  'bounded without a timing or allocation-trace equality '
                                                  'claim. This current-source review grants zero kills; the '
                                                  'exact mutant must still be an actual MissedMutant.',
                                                  {'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs': '81810b88c2e856e8bcd861a53b775deee2d479f7ed494281659ed70dffd536c7',
                                                   'crates/time-core/src/regression/chart.rs': '598f7870c27f69f1e4fe6abd7d820dd8ad85bef6b811f01b1ea90966c46c89ba'}),
 'BF-STATE-REBASE-SOURCE-LABELS': ('81810b88c2e856e8bcd861a53b775deee2d479f7ed494281659ed70dffd536c7',
                                   '2d0000995867fd3e8186e4d7035d776910069bbe78dd71054ffcf0dbc2f4c0c1',
                                   'The unchanged checked intersection composition rejects each invalid '
                                   'intermediate label, and exact recomputed complete Basis matching rejects '
                                   'each invalid replacement endpoint, before anchor/prior transformation. '
                                   'All such composition failures map to AnchorRebaseUnavailable. Returned '
                                   'cause, model and effects agree over all admitted private operands; extra '
                                   'immutable recomposition stays bounded without a timing or '
                                   'allocation-trace equality claim. This current-source review grants zero '
                                   'kills; the exact mutant must still be an actual MissedMutant.',
                                   {'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs': '81810b88c2e856e8bcd861a53b775deee2d479f7ed494281659ed70dffd536c7',
                                    'crates/time-core/src/regression/chart.rs': '598f7870c27f69f1e4fe6abd7d820dd8ad85bef6b811f01b1ea90966c46c89ba'}),
 'BF-STATE-REBASE-SOURCE-GENERATION-TARGET-FRAME': ('81810b88c2e856e8bcd861a53b775deee2d479f7ed494281659ed70dffd536c7',
                                                    '0618126bbf35bad79ab81e6056df17aed1728d67952c23b46a51bed234e75b44',
                                                    'The unchanged checked intersection composition rejects '
                                                    'each invalid intermediate label, and exact recomputed '
                                                    'complete Basis matching rejects each invalid '
                                                    'replacement endpoint, before anchor/prior '
                                                    'transformation. All such composition failures map to '
                                                    'AnchorRebaseUnavailable. Returned cause, model and '
                                                    'effects agree over all admitted private operands; extra '
                                                    'immutable recomposition stays bounded without a timing '
                                                    'or allocation-trace equality claim. This current-source '
                                                    'review grants zero kills; the exact mutant must still '
                                                    'be an actual MissedMutant.',
                                                    {'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs': '81810b88c2e856e8bcd861a53b775deee2d479f7ed494281659ed70dffd536c7',
                                                     'crates/time-core/src/regression/chart.rs': '598f7870c27f69f1e4fe6abd7d820dd8ad85bef6b811f01b1ea90966c46c89ba'}),
 'BF-STATE-REBASE-TARGET-LABELS': ('81810b88c2e856e8bcd861a53b775deee2d479f7ed494281659ed70dffd536c7',
                                   '83e3f3e4d5a88097f4d2720d16bb09b47c96bd28165900b05f6d179727ee33c0',
                                   'The unchanged checked intersection composition rejects each invalid '
                                   'intermediate label, and exact recomputed complete Basis matching rejects '
                                   'each invalid replacement endpoint, before anchor/prior transformation. '
                                   'All such composition failures map to AnchorRebaseUnavailable. Returned '
                                   'cause, model and effects agree over all admitted private operands; extra '
                                   'immutable recomposition stays bounded without a timing or '
                                   'allocation-trace equality claim. This current-source review grants zero '
                                   'kills; the exact mutant must still be an actual MissedMutant.',
                                   {'crates/time-core/src/reference_input/rate_delivery/frequency_prior.rs': '81810b88c2e856e8bcd861a53b775deee2d479f7ed494281659ed70dffd536c7',
                                    'crates/time-core/src/regression/chart.rs': '598f7870c27f69f1e4fe6abd7d820dd8ad85bef6b811f01b1ea90966c46c89ba'}),
 'BF-NUMERIC-ANCHOR-BOUND-ABS-SIGN': ('6dde6d91d6b70f1334f04f5ef8ade4062ec767c6762b15b399d4fdf302025cd0',
                                      '634b2295f78ea61d46e336309fd366e8c04bd3bf2e2ecf0f16a5683798ab153e',
                                      'Only the coefficient-bound operand changes sign; checked effect '
                                      'admission, phase subtraction and the zero shortcut are unchanged. The '
                                      'pinned parent coefficient_addition_error immediately takes both '
                                      'operand magnitudes, yielding identical bound operands, failure order '
                                      'and result bits for every admitted signed point including signed '
                                      'zero. This current-source review grants zero kills; the exact mutant '
                                      'must still be an actual MissedMutant.',
                                      {'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs': '6dde6d91d6b70f1334f04f5ef8ade4062ec767c6762b15b399d4fdf302025cd0',
                                       'crates/time-core/src/regression/chart/roundoff.rs': '38405a55e5a85a34a04bd751cc51ef08be5363f21681691e6a955a4b6983128e'}),
 'BF-NUMERIC-INTERVAL-WHOLE-ZERO-PRODUCT': ('6dde6d91d6b70f1334f04f5ef8ade4062ec767c6762b15b399d4fdf302025cd0',
                                            '6106e65c018c06850ad7b2e57d45ccb4caab74f15a792e8d0020e57b2bd5120a',
                                            'Both whole-zero intervals take the same ZERO shortcut; neither '
                                            'whole-zero interval uses identical four-pair work. With exactly '
                                            'one whole-zero interval, the unchanged inner endpoint OR '
                                            'returns literal positive-zero bounds for every pair and min/max '
                                            'reduction returns canonical ZERO. Outputs, errors and effects '
                                            'agree over finite ordered intervals; work stays at most four '
                                            'pairs without timing equality. This current-source review '
                                            'grants zero kills; the exact mutant must still be an actual '
                                            'MissedMutant.',
                                            {'crates/time-core/src/regression/chart/roundoff/frequency_prior.rs': '6dde6d91d6b70f1334f04f5ef8ade4062ec767c6762b15b399d4fdf302025cd0'}),
 'BF-PHASE-PROFILE-NORMAL-POSITIVE': ('3ddf8f1a78e19874f18d03ba564e8117a3f7aa88356b1441601d2215278a7d43',
                                      '315fd436863d362ccf54569c33f7d00c5769dce339afb8ce8e07d28ca7e9e508',
                                      'For every raw phase-cap binary64 value, unchanged normality excludes '
                                      'both signed zeros, subnormals, infinities and every NaN before the '
                                      'comparison. Positive normal values satisfy both >0 and >=0 and '
                                      'negative normal values satisfy neither. Horizons, iteration order, '
                                      'validation, refusal and effects are unchanged over the full private '
                                      'profile domain. This current-source review grants zero kills; the '
                                      'exact mutant must still be an actual MissedMutant.',
                                      {'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs': '3ddf8f1a78e19874f18d03ba564e8117a3f7aa88356b1441601d2215278a7d43'})}


def baseline_feedback_equivalences_valid(policy):
    """Admit no E, or the complete code-owned current-source review catalog.

    Actual product/source/document hashing and section admission remain mandatory
    in admit_mutation_equivalences. This check supplies no historical inheritance
    or policy-controlled semantic approval and guesses no future candidate hash.
    """
    if type(policy) is not dict or type(policy.get("mutation_equivalences")) is not list:
        return False
    entries = policy["mutation_equivalences"]
    if not entries:
        return True
    if len(entries) != len(BASELINE_FEEDBACK_EQUIVALENCE_REVIEWS):
        return False
    candidate, pins = policy.get("candidate_sha256"), policy.get("inputs_sha256")
    if (type(candidate) is not str or re.fullmatch(r"[0-9a-f]{64}", candidate) is None
            or type(pins) is not dict):
        return False
    fields = {"id", "candidate_sha256", "source_sha256", "mutant", "requirement", "rationale", "proof"}
    seen = set()
    for entry in entries:
        if type(entry) is not dict or set(entry) != fields:
            return False
        identifier = entry["id"]
        if (type(identifier) is not str or identifier not in BASELINE_FEEDBACK_EQUIVALENCE_REVIEWS
                or identifier in seen):
            return False
        source, descriptor, rationale, dependencies = BASELINE_FEEDBACK_EQUIVALENCE_REVIEWS[identifier]
        if (entry["candidate_sha256"] != candidate or entry["source_sha256"] != source
                or type(entry["mutant"]) is not dict or canonical_sha256(entry["mutant"]) != descriptor
                or entry["requirement"] != BASELINE_FEEDBACK_EQUIVALENCE_REQUIREMENT
                or entry["rationale"] != rationale
                or any(pins.get(name) != digest for name, digest in dependencies.items())):
            return False
        proof = entry["proof"]
        if (type(proof) is not dict or set(proof) != {"path", "sha256", "section", "reviewer", "verdict"}
                or proof["path"] != BASELINE_FEEDBACK_EQUIVALENCE_PROOF or proof["section"] != identifier
                or proof["reviewer"] != BASELINE_FEEDBACK_EQUIVALENCE_REVIEWERS
                or proof["verdict"] != "independently-justified-equivalent"
                or type(proof["sha256"]) is not str or re.fullmatch(r"[0-9a-f]{64}", proof["sha256"]) is None
                or pins.get(BASELINE_FEEDBACK_EQUIVALENCE_PROOF) != proof["sha256"]):
            return False
        seen.add(identifier)
    return seen == set(BASELINE_FEEDBACK_EQUIVALENCE_REVIEWS)


def require_baseline_feedback_role(scope, policy):
    """Admit only the current five owners, without historical equivalence credit."""
    facts = {**frequency_prior_role_facts(),
             "critical_files": list(dict(BASELINE_FEEDBACK_SUBJECTS).values()),
             "external_consumer_limits": {"legal_executions": 2, "negative_compilations": 23}}
    extra = {"source_file_count", "candidate_sha256", "inputs_sha256", "mutation_inventories",
             "mutation_equivalences", "test_inventories", "scope"}
    if (type(scope) is not Scope or scope.package != "time-core"
            or scope.policy != "policy/baseline-feedback-analysis.json"
            or scope.subjects != BASELINE_FEEDBACK_SUBJECTS or type(policy) is not dict
            or set(policy) != set(facts) | extra
            or any(type(policy.get(k)) is not type(v) or policy[k] != v for k, v in facts.items())
            or not baseline_feedback_equivalences_valid(policy)):
        raise Incomplete("Baseline-feedback requires its exact five-owner stable role and current closed equivalences.")


PHASE_STEP_EQUIVALENCE_PROOF = 'tasks/s1.3-source-selection-readiness.md'
PHASE_STEP_EQUIVALENCE_REQUIREMENT = 'REQ-8/10/11/14/15; CHECK-7/8/10/11/14/25; AC-19'
PHASE_STEP_EQUIVALENCE_REVIEWERS = '/root; /root/next_product_core'
PHASE_STEP_EQUIVALENCE_REVIEWS = {'PHASE-STEP-PROFILE-NORMAL-POSITIVE': {'source_sha256': '7604ff59b5d5247d7f884e797a97c279be9df6618034a1dedd081ca7bc466e8a',
                                        'descriptor_sha256': 'd7e777452cd768d189f2e1895111408e9d23b95540dd179d9b1a57ea423c9552',
                                        'diff_sha256': '781048460708376801627f75368594b99f2bfe8add27ac0eac57f423a695fd14',
                                        'rationale': 'Normality excludes signed zeros, subnormals, '
                                                     'infinities and every NaN before comparison. For every '
                                                     'remaining positive or negative normal, >0 and >=0 '
                                                     'agree; guards, order, validation, errors and effects '
                                                     'are unchanged. This ordinary current-source review '
                                                     'grants zero additional kills.',
                                        'dependencies': {}},
 'PHASE-STEP-DURATION-MIN-EQUALITY': {'source_sha256': '7604ff59b5d5247d7f884e797a97c279be9df6618034a1dedd081ca7bc466e8a',
                                      'descriptor_sha256': '4c0ac0f3862d4fa7ab1bd34ebdb381a1763f6c9e64c63e554e3c84686ea47c1e',
                                      'diff_sha256': '52144c072c55f196d21e61fc474f0648f9aff0f67299eeb5a80e8f411c183b7e',
                                      'rationale': 'Only equal nanosecond counts choose different branches. '
                                                   'ActiveAwakeDurationNs is Copy with one private u64 '
                                                   'field, so either branch returns the identical value; '
                                                   'requested duration, deadline, state and effects agree. '
                                                   'The separate rate denominator is unchanged. Zero '
                                                   'additional kills.',
                                      'dependencies': {'crates/time-domain/src/clock.rs': 'beca67da84fc7a4717dd6f35325ec1308d4a7e5b6c2ad30dfc65392f94bbde51'}},
 'PHASE-STEP-CLOCK-ORDER-RECHECK': {'source_sha256': '7604ff59b5d5247d7f884e797a97c279be9df6618034a1dedd081ca7bc466e8a',
                                    'descriptor_sha256': 'a1ad9f41b00eb2e23257158b34509f2ff5d327adf4a0d2f258d5090627b4d364',
                                    'diff_sha256': 'b37c235f1fcda25050f71731d6a0010f14568f65fa873cd72645ffa4a005542b',
                                    'rationale': 'Context disagreement and both-backwards clocks still '
                                                 'reject immediately. One backwards clock instead rejects '
                                                 'through checked duration_since with the same Map error '
                                                 'before numerical work or witness mutation. '
                                                 'Neither-backwards paths agree; no allocation, panic, Drop '
                                                 'or state effect changes. Existing worst-case work remains '
                                                 'bounded; no timing equality or additional kill is claimed.',
                                    'dependencies': {'crates/time-domain/src/clock.rs': 'beca67da84fc7a4717dd6f35325ec1308d4a7e5b6c2ad30dfc65392f94bbde51'},
                                    'product_dependencies': {'crates/time-domain/src/lib.rs': 'd1314aa80fc705d410e747de555d79d0a8a63d16b2ff5877e1012ce60ef039ab'}},
 'PHASE-STEP-ANCHOR-ABS-SIGN': {'source_sha256': 'c8af09a8f7eb5e1f05cc9aaeb92b3533a05a688c8dac2e2338007160e821fbf9',
                                'descriptor_sha256': '634b2295f78ea61d46e336309fd366e8c04bd3bf2e2ecf0f16a5683798ab153e',
                                'diff_sha256': 'f9da51f1b88bf9967da7d8564972a0791053760d75382a80b56477d11a26d8c5',
                                'rationale': "Only the coefficient-bound operand's unary sign changes. The "
                                             'pinned parent takes right.abs() before every bound operation, '
                                             'producing identical operands, signed-zero behavior, errors and '
                                             'outputs. Actual phase subtraction and earlier admissions are '
                                             'unchanged. Zero additional kills.',
                                'dependencies': {'crates/time-core/src/regression/chart/roundoff.rs': '38405a55e5a85a34a04bd751cc51ef08be5363f21681691e6a955a4b6983128e'}},
 'PHASE-STEP-INTERVAL-ZERO-PRODUCT': {'source_sha256': 'c8af09a8f7eb5e1f05cc9aaeb92b3533a05a688c8dac2e2338007160e821fbf9',
                                      'descriptor_sha256': '6106e65c018c06850ad7b2e57d45ccb4caab74f15a792e8d0020e57b2bd5120a',
                                      'diff_sha256': '90f0a559dc37f069799729ce3c413645631dd0dcf5590fa498dd6912489cf270',
                                      'rationale': 'Both-whole-zero and neither paths agree. With exactly '
                                                   'one whole-zero interval, all four pairs hit the '
                                                   'unchanged inner zero shortcut and return literal '
                                                   'positive zeros; reduction returns canonical zero without '
                                                   'multiplication or outward error. Signed-zero, subnormal '
                                                   'and finite extremes, state, errors and the four-pair '
                                                   'maximum agree. No timing equality or additional kill is '
                                                   'claimed.',
                                      'dependencies': {}},
 'PHASE-STEP-ISSUED-POSITIVE-DIRECTION': {'source_sha256': '0de5d97e268bc2833749b1ea0194014b250d51822e2febf00e33a5b8c04df811',
                                          'descriptor_sha256': '381cb2e3f952d6af6b2d5788024488b712ce157b45fe1413ffceed501d2aedd3',
                                          'diff_sha256': '981f6c193de8aabdd32e05724148d7bf344c36f9c25adbb0ec6e69c536b8a9d9',
                                          'rationale': 'Private StepIssued is constructed only after '
                                                       'mandatory checked request admission requires '
                                                       'abs(D)>E>=0. Checked PhaseDebt and scalar admission '
                                                       'exclude NaN, infinity and subnormal points and '
                                                       'negative error; hence issued D is neither signed '
                                                       'zero. Replacing the requested-debt sign comparison '
                                                       'with its nonstrict form agrees for both admitted '
                                                       'signs. The immutable request, actual-step/debt side '
                                                       'conditions, branch order, errors and historical '
                                                       'effects are unchanged. Invalid raw StepIssued '
                                                       'fabrication is outside this admitted helper '
                                                       'boundary. This ordinary current-source review grants '
                                                       'zero additional kills.',
                                          'dependencies': {'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs': '7604ff59b5d5247d7f884e797a97c279be9df6618034a1dedd081ca7bc466e8a',
                                                           'crates/time-core/src/regression/chart/roundoff/phase_plan.rs': 'e17ae1f71891d7235355da9b9caf7506e12df7ac8e789a948d0948cc7d60fcf6',
                                                           'crates/time-core/src/regression.rs': '282e21edbf6d3fffe6d53254de3339eba0a0dd3a711503e009cca411fe0bbeb7'}},
 'PHASE-STEP-ISSUED-NEGATIVE-DIRECTION': {'source_sha256': '0de5d97e268bc2833749b1ea0194014b250d51822e2febf00e33a5b8c04df811',
                                          'descriptor_sha256': '1ce225d51195ebad9a84648575faeed02f92d66a92be4d97b1c4fa5f81fa6e9a',
                                          'diff_sha256': '195dddc709b2edf1ef0cae6309d71aa88b749d2acca0d2d0577f223ef28dddaa',
                                          'rationale': 'Private StepIssued is constructed only after '
                                                       'mandatory checked request admission requires '
                                                       'abs(D)>E>=0. Checked PhaseDebt and scalar admission '
                                                       'exclude NaN, infinity and subnormal points and '
                                                       'negative error; hence issued D is neither signed '
                                                       'zero. Replacing the requested-debt sign comparison '
                                                       'with its nonstrict form agrees for both admitted '
                                                       'signs. The immutable request, actual-step/debt side '
                                                       'conditions, branch order, errors and historical '
                                                       'effects are unchanged. Invalid raw StepIssued '
                                                       'fabrication is outside this admitted helper '
                                                       'boundary. This ordinary current-source review grants '
                                                       'zero additional kills.',
                                          'dependencies': {'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs': '7604ff59b5d5247d7f884e797a97c279be9df6618034a1dedd081ca7bc466e8a',
                                                           'crates/time-core/src/regression/chart/roundoff/phase_plan.rs': 'e17ae1f71891d7235355da9b9caf7506e12df7ac8e789a948d0948cc7d60fcf6',
                                                           'crates/time-core/src/regression.rs': '282e21edbf6d3fffe6d53254de3339eba0a0dd3a711503e009cca411fe0bbeb7'}},
 'PHASE-STEP-ISSUED-POSITIVE-OVERSHOOT': {'source_sha256': '0de5d97e268bc2833749b1ea0194014b250d51822e2febf00e33a5b8c04df811',
                                          'descriptor_sha256': '76b9be08010417558ce0c2f5db28b6288ab3d4703fae9b5de300f94f99e87c63',
                                          'diff_sha256': '6b8a0aa54e5ff15b3e5c28c9ea592c8ca22da060f673ed9147160cad40dd1cf3',
                                          'rationale': 'Private StepIssued is constructed only after '
                                                       'mandatory checked request admission requires '
                                                       'abs(D)>E>=0. Checked PhaseDebt and scalar admission '
                                                       'exclude NaN, infinity and subnormal points and '
                                                       'negative error; hence issued D is neither signed '
                                                       'zero. Replacing the requested-debt sign comparison '
                                                       'with its nonstrict form agrees for both admitted '
                                                       'signs. The immutable request, actual-step/debt side '
                                                       'conditions, branch order, errors and historical '
                                                       'effects are unchanged. Invalid raw StepIssued '
                                                       'fabrication is outside this admitted helper '
                                                       'boundary. This ordinary current-source review grants '
                                                       'zero additional kills.',
                                          'dependencies': {'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs': '7604ff59b5d5247d7f884e797a97c279be9df6618034a1dedd081ca7bc466e8a',
                                                           'crates/time-core/src/regression/chart/roundoff/phase_plan.rs': 'e17ae1f71891d7235355da9b9caf7506e12df7ac8e789a948d0948cc7d60fcf6',
                                                           'crates/time-core/src/regression.rs': '282e21edbf6d3fffe6d53254de3339eba0a0dd3a711503e009cca411fe0bbeb7'}},
 'PHASE-STEP-ISSUED-NEGATIVE-OVERSHOOT': {'source_sha256': '0de5d97e268bc2833749b1ea0194014b250d51822e2febf00e33a5b8c04df811',
                                          'descriptor_sha256': 'c8d9a9a515a060778d8ad8539fbd8f0aabfa2dd1764398032ec8178f3baad108',
                                          'diff_sha256': '9ef0f6d808b34342b167bf5aa80bb77897e3ccb644413c23c8b9c3f896520fbb',
                                          'rationale': 'Private StepIssued is constructed only after '
                                                       'mandatory checked request admission requires '
                                                       'abs(D)>E>=0. Checked PhaseDebt and scalar admission '
                                                       'exclude NaN, infinity and subnormal points and '
                                                       'negative error; hence issued D is neither signed '
                                                       'zero. Replacing the requested-debt sign comparison '
                                                       'with its nonstrict form agrees for both admitted '
                                                       'signs. The immutable request, actual-step/debt side '
                                                       'conditions, branch order, errors and historical '
                                                       'effects are unchanged. Invalid raw StepIssued '
                                                       'fabrication is outside this admitted helper '
                                                       'boundary. This ordinary current-source review grants '
                                                       'zero additional kills.',
                                          'dependencies': {'crates/time-core/src/reference_input/rate_delivery/phase_plan.rs': '7604ff59b5d5247d7f884e797a97c279be9df6618034a1dedd081ca7bc466e8a',
                                                           'crates/time-core/src/regression/chart/roundoff/phase_plan.rs': 'e17ae1f71891d7235355da9b9caf7506e12df7ac8e789a948d0948cc7d60fcf6',
                                                           'crates/time-core/src/regression.rs': '282e21edbf6d3fffe6d53254de3339eba0a0dd3a711503e009cca411fe0bbeb7'}},
 'PHASE-STEP-ADAPTIVE-NORMAL-POSITIVE': {'source_sha256': 'e530725ed2b122af12e11f3a01ade69f11fa92547c4a091fc5f86a7cb6544ba0',
                                         'descriptor_sha256': 'f10ebc3cf37f017059af6bd88559bcfe238f3a1dffeaa91ace78213a906b7e76',
                                         'diff_sha256': '58b683c17c812e023c29f8aedeca1471874da3ded39381d05f22f3b93209dd1b',
                                         'rationale': 'The unchanged is_normal check excludes both signed '
                                                      'zeros, subnormals, infinities and NaNs before the '
                                                      'ratio comparison. Every remaining positive normal '
                                                      'satisfies both >0 and >=0, and every negative normal '
                                                      'satisfies neither. All other raw profile admission, '
                                                      'validation, errors and effects agree over the full '
                                                      'private domain. Zero additional kills.',
                                         'dependencies': {}},
 'PHASE-STEP-ADAPTIVE-DISJOINT-MANTISSA': {'source_sha256': 'e530725ed2b122af12e11f3a01ade69f11fa92547c4a091fc5f86a7cb6544ba0',
                                           'descriptor_sha256': '2132c58e9497679f4d3fca0cfdbc9b38f3cdd5435b64e5025115dd147d79e02d',
                                           'diff_sha256': 'a243b91fcb97db38fa229832698b71c7095bf764427cee69f0524c3653c99d91',
                                           'rationale': 'The fraction mask retains only bits 0 through 51, '
                                                        'so its result and implicit significand bit 52 are '
                                                        'disjoint for every 64-bit input pattern. OR and XOR '
                                                        'produce identical integers before the same checked '
                                                        'product, shifts, ceiling and admissions. No caller '
                                                        'restriction, timing equality or additional kill is '
                                                        'claimed.',
                                           'dependencies': {}},
 'PHASE-STEP-ADAPTIVE-UNREACHABLE-CEILING': {'source_sha256': 'e530725ed2b122af12e11f3a01ade69f11fa92547c4a091fc5f86a7cb6544ba0',
                                             'descriptor_sha256': '9321330df6f6e0408784bdcac473b384032d38308936c2897c27e64a853752ab',
                                             'diff_sha256': '733f67c20d871bec3449887a9db532f96cb2242e1cccbc8fe44326b68c111a9a',
                                             'rationale': 'The comparison differs only at exact ceiling '
                                                          '2^53. Positive representable seconds would lie in '
                                                          '((2^53-1)/1e9,2^53/1e9]. Consecutive binary64 '
                                                          'values 0x41612e0be826d694 and 0x41612e0be826d695 '
                                                          'straddle that entire interval: scaled exact '
                                                          'values are 2^53-341223/262144 and '
                                                          '2^53+588233/1048576. No finite normal input '
                                                          'reaches equality; earlier invalid-input and '
                                                          'checked-u128 refusals and every other '
                                                          'result/effect agree. No upper-clamp assumption or '
                                                          'additional kill is claimed.',
                                             'dependencies': {}},
 'PHASE-STEP-SETTLED-BOUND-ABS-SIGN': {'source_sha256': 'e17ae1f71891d7235355da9b9caf7506e12df7ac8e789a948d0948cc7d60fcf6',
                                       'descriptor_sha256': 'dcbe2a04a93d1883bb238ce9ff85b08ca820f535d52ded4a7d734e9b63c54d9d',
                                       'diff_sha256': 'f3ba647075214ad6579b37160923da0fe4f53c2bb7120a19026ca05a796278a4',
                                       'rationale': 'Only the bound-helper right operand loses its unary '
                                                    'sign. Actual point subtraction and prior admission are '
                                                    'unchanged. The pinned parent coefficient_addition_error '
                                                    'immediately uses right.abs(), so both signs yield '
                                                    'identical bound operands, signed-zero behavior, failure '
                                                    'order and output bits over the admitted helper domain. '
                                                    'This ordinary source review grants zero additional '
                                                    'kills.',
                                       'dependencies': {'crates/time-core/src/regression/chart/roundoff.rs': '38405a55e5a85a34a04bd751cc51ef08be5363f21681691e6a955a4b6983128e'}},
 'PHASE-STEP-STEPPED-BOUND-ABS-SIGN': {'source_sha256': 'e17ae1f71891d7235355da9b9caf7506e12df7ac8e789a948d0948cc7d60fcf6',
                                       'descriptor_sha256': '696b292a1d280d254103b4f9c333f9d0a5223987b69cbb43c89e7ec462a2d756',
                                       'diff_sha256': 'a43517c43a9be58f4937888c9487688811246995cd4e7d52399421a34890abe4',
                                       'rationale': 'Only the bound-helper right operand loses its unary '
                                                    'sign. Actual point subtraction and prior admission are '
                                                    'unchanged. The pinned parent coefficient_addition_error '
                                                    'immediately uses right.abs(), so both signs yield '
                                                    'identical bound operands, signed-zero behavior, failure '
                                                    'order and output bits over the admitted helper domain. '
                                                    'This ordinary source review grants zero additional '
                                                    'kills.',
                                       'dependencies': {'crates/time-core/src/regression/chart/roundoff.rs': '38405a55e5a85a34a04bd751cc51ef08be5363f21681691e6a955a4b6983128e'}},
 'PHASE-STEP-POSITIVE-CLAMP-EQUALITY': {'source_sha256': 'e17ae1f71891d7235355da9b9caf7506e12df7ac8e789a948d0948cc7d60fcf6',
                                        'descriptor_sha256': 'f5cefac55b3ec17ef6c0a1cefe71089e621e4cdc35ee31fe74c00182d4761a10',
                                        'diff_sha256': '2bfa17c890d525074bec1bd96a9957bbb15b643b982eb07a0b866796031119da',
                                        'rationale': 'Only equality with the finite strictly positive '
                                                     'transient cap selects a different branch. The admitted '
                                                     'quotient then has the identical nonzero positive-cap '
                                                     'or negative-cap bits, so selecting the bound or the '
                                                     'quotient returns the same point. Signed-zero ties are '
                                                     'excluded by positive normal cap admission; later '
                                                     'aggregate and error calculations, branch order and '
                                                     'effects agree. Zero additional kills.',
                                        'dependencies': {}},
 'PHASE-STEP-NEGATIVE-CLAMP-EQUALITY': {'source_sha256': 'e17ae1f71891d7235355da9b9caf7506e12df7ac8e789a948d0948cc7d60fcf6',
                                        'descriptor_sha256': 'deb23ed259f4e8c67b4f0905770c5cd78e19dddeaf00c4bb3cab82b7bf9bdc2f',
                                        'diff_sha256': '1a2e3b9b58028a80f1db6150a8b75104c6b467fdc704a1f6c5cb262315b53183',
                                        'rationale': 'Only equality with the finite strictly positive '
                                                     'transient cap selects a different branch. The admitted '
                                                     'quotient then has the identical nonzero positive-cap '
                                                     'or negative-cap bits, so selecting the bound or the '
                                                     'quotient returns the same point. Signed-zero ties are '
                                                     'excluded by positive normal cap admission; later '
                                                     'aggregate and error calculations, branch order and '
                                                     'effects agree. Zero additional kills.',
                                        'dependencies': {}}}


PHASE_STEP_EQUIVALENCE_IDS = frozenset(PHASE_STEP_EQUIVALENCE_REVIEWS)


def phase_step_equivalences_valid(policy):
    """Admit no exclusions, or the complete closed current-source review catalog.

    Candidate and proof bytes are re-admitted by admit_mutation_equivalences.
    No historical candidate, old receipt or policy-controlled review is adopted.
    """
    if type(policy) is not dict or type(policy.get("mutation_equivalences")) is not list:
        return False
    entries = policy["mutation_equivalences"]
    if not entries:
        return True
    if (len(entries) != len(PHASE_STEP_EQUIVALENCE_IDS)
            or set(PHASE_STEP_EQUIVALENCE_REVIEWS) != PHASE_STEP_EQUIVALENCE_IDS):
        return False
    candidate, pins = policy.get("candidate_sha256"), policy.get("inputs_sha256")
    if (type(candidate) is not str or re.fullmatch(r"[a-f0-9]{64}", candidate) is None
            or type(pins) is not dict):
        return False
    fields = {"id", "candidate_sha256", "source_sha256", "mutant", "requirement", "rationale", "proof"}
    seen = set()
    for entry in entries:
        if type(entry) is not dict or set(entry) != fields:
            return False
        identifier = entry["id"]
        if (type(identifier) is not str or identifier not in PHASE_STEP_EQUIVALENCE_REVIEWS
                or identifier in seen):
            return False
        facts = PHASE_STEP_EQUIVALENCE_REVIEWS[identifier]
        if (entry["candidate_sha256"] != candidate or entry["source_sha256"] != facts["source_sha256"]
                or type(entry["mutant"]) is not dict
                or canonical_sha256(entry["mutant"]) != facts["descriptor_sha256"]
                or pins.get(entry["mutant"].get("file")) != facts["source_sha256"]
                or entry["requirement"] != PHASE_STEP_EQUIVALENCE_REQUIREMENT
                or entry["rationale"] != facts["rationale"]
                or any(pins.get(name) != digest for name, digest in facts["dependencies"].items())):
            return False
        proof = entry["proof"]
        if (type(proof) is not dict or set(proof) != {"path", "sha256", "section", "reviewer", "verdict"}
                or proof["path"] != PHASE_STEP_EQUIVALENCE_PROOF or proof["section"] != identifier
                or proof["reviewer"] != PHASE_STEP_EQUIVALENCE_REVIEWERS
                or proof["verdict"] != "independently-justified-equivalent"
                or type(proof["sha256"]) is not str or re.fullmatch(r"[a-f0-9]{64}", proof["sha256"]) is None
                or pins.get(PHASE_STEP_EQUIVALENCE_PROOF) != proof["sha256"]):
            return False
        seen.add(identifier)
    return seen == PHASE_STEP_EQUIVALENCE_IDS


def phase_step_role_facts():
    """Admit the exact external contracts; grant no inherited score or mutation authority."""
    return {
        **frequency_prior_role_facts(),
        "critical_files": list(dict(PHASE_STEP_SUBJECTS).values()),
        "external_consumer_limits": {"legal_executions": 3, "negative_compilations": 32},
        "contracts_admitted": True,
        "validation_mechanisms_admitted": True,
        "validation_mechanism_limits": {
            "mechanisms": 7, "specimens_per_mechanism": 3, "test_executions": 21},
        "mutation_jobserver_tasks": 0,
        "campaigns_admitted": False,
    }


def require_phase_step_role(scope, policy):
    """Admit seven complete files, unchanged limits and only closed current reviews."""
    facts = phase_step_role_facts()
    extra = {"source_file_count", "candidate_sha256", "inputs_sha256", "mutation_inventories",
             "mutation_equivalences", "test_inventories", "scope"}
    if (type(scope) is not Scope or scope.package != "time-core"
            or scope.policy != "policy/phase-step-analysis.json"
            or scope.subjects != PHASE_STEP_SUBJECTS or type(policy) is not dict
            or set(policy) != set(facts) | extra
            or any(type(policy.get(key)) is not type(value) or policy[key] != value
                   for key, value in facts.items() if key not in {"campaigns_admitted", "mutation_jobs"})
            or type(policy.get("campaigns_admitted")) is not bool
            or type(policy.get("mutation_jobs")) is not int or policy["mutation_jobs"] not in {10, 8, 6}
            or any(type(value) is not int for value in policy["external_consumer_limits"].values())
            or not phase_step_equivalences_valid(policy)):
        raise Incomplete("Phase-step requires its exact seven-file stable role and unchanged gates.")
    if policy["campaigns_admitted"]:
        _phase_step_campaign_metadata(policy)


def _phase_step_campaign_metadata(policy):
    """Require complete bound inputs, rosters and whole-file inventories, never a flag alone.

    The verifier additionally re-admits the exact current product and protected
    source/helper set before a Session or direct union; this effect-boundary
    check cannot substitute a policy-controlled name for that admission.
    """
    owners = dict(PHASE_STEP_SUBJECTS)
    pins, rows = policy.get("inputs_sha256"), policy.get("mutation_inventories")
    rosters = policy.get("test_inventories")
    if (type(policy.get("source_file_count")) is not int
            or not 0 < policy["source_file_count"] <= 4096
            or type(policy.get("candidate_sha256")) is not str
            or re.fullmatch(r"[a-f0-9]{64}", policy["candidate_sha256"]) is None
            or type(pins) is not dict or not set(owners.values()) <= set(pins)
            or any(type(name) is not str or not name or type(digest) is not str
                   or re.fullmatch(r"[a-f0-9]{64}", digest) is None for name, digest in pins.items())
            or type(rows) is not dict or set(rows) != set(owners)
            or any(type(row) is not dict or set(row) != {"count", "sha256"}
                   or type(row["count"]) is not int or not 0 < row["count"] <= 512
                   or type(row["sha256"]) is not str
                   or re.fullmatch(r"[a-f0-9]{64}", row["sha256"]) is None for row in rows.values())
            or sum(row["count"] for row in rows.values()) > 2048
            or type(rosters) is not dict or set(rosters) != {"baseline", "coverage"}):
        raise Incomplete("Phase-step campaign requires complete current source, rosters and seven reviewed inventories.")
    for names in rosters.values():
        if (type(names) is not list or not 0 < len(names) <= 4096
                or any(type(name) is not str or not 0 < len(name) <= 512
                       or "\n" in name or "\r" in name for name in names)
                or names != sorted(set(names))):
            raise Incomplete("Phase-step campaign requires bounded frozen nonempty test rosters.")
    baseline, coverage = rosters["baseline"], rosters["coverage"]
    if (coverage != [name for name in baseline if " (line " not in name]
            or not any(" (line " in name for name in baseline)):
        raise Incomplete("Phase-step campaign requires the complete ordinary roster and source-classified doctests.")


def admission_profile(args, scope, policy):
    """Require an exact stable role before narrowing analysis-tool custody."""
    profile = getattr(args, "tool_admission_profile", "full")
    if type(profile) is not str or profile not in ANALYSIS.ADMISSION_PROFILES:
        raise Incomplete("Unknown analysis admission profile.")
    if policy.get("tool_admission_profile", "full") != profile:
        raise Incomplete("Analysis admission profile differs from the current policy.")
    if (scope.policy == "policy/phase-step-analysis.json"
            or scope.subjects == PHASE_STEP_SUBJECTS):
        require_phase_step_role(scope, policy)
        if profile != "stable-core" or args.lane not in {"contracts", "coverage", "mutation"}:
            raise Incomplete("Phase-step admits only its fixed stable compiler/coverage/listing stages.")
        return profile
    if engine_ke_features(scope):
        require_engine_ke_role(scope, policy)
        if (profile != "stable-core" or args.lane not in {"contracts", "baseline", "coverage", "mutation", "all"}
                or (args.lane == "contracts" and getattr(args, "subject", None) not in ("ke_process", "ke_job"))):
            raise Incomplete("Engine KE admits only its stable analysis owners and fixed process/job compiler contracts.")
        return profile
    if (scope.policy == "policy/baseline-feedback-analysis.json"
            or scope.subjects == BASELINE_FEEDBACK_SUBJECTS):
        require_baseline_feedback_role(scope, policy)
        if profile != "stable-core" or args.lane not in {"contracts", "coverage", "mutation", "all"}:
            raise Incomplete("Baseline-feedback admits only stable compiler/coverage/mutation lanes.")
        return profile
    if (scope.policy == "policy/frequency-prior-analysis.json"
            or scope.subjects == FREQUENCY_PRIOR_SUBJECTS):
        require_frequency_prior_role(scope, policy)
        if profile != "stable-core" or args.lane not in {"contracts", "coverage", "mutation", "all"}:
            raise Incomplete("Frequency-prior admits only its stable compiler/coverage/mutation lanes.")
        return profile
    if (scope.package == "windows-platform" or scope.policy == "policy/service-analysis.json"
            or scope.subjects in (SERVICE_LIFECYCLE_SUBJECTS, SERVICE_SCM_SUBJECTS, SERVICE_PRIVILEGE_SUBJECTS)
            or policy.get("package") == "windows-platform"):
        # This exact portable owner uses the complete admitted stable payload.
        # No nightly, proof tool or unrelated analyzer is executed or borrowed.
        # Preserve the exact historical rows. Schema 5 adds only the checked
        # privilege owner to schema 4's four-worker, mutation-only 512-task row.
        schema = policy.get("schema_version")
        if type(schema) is not int or schema not in {1, 2, 3, 4, 5}:
            raise Incomplete("Unknown exact service analysis schema.")
        workers, process_limit = {1: (4, 128), 2: (4, 128), 3: (1, 128), 4: (4, 512), 5: (4, 512)}[schema]
        subjects = (SERVICE_LIFECYCLE_SUBJECTS if schema == 1 else
                    SERVICE_PRIVILEGE_SUBJECTS if schema == 5 else SERVICE_SCM_SUBJECTS)
        facts = {
            "schema_version": schema, "package": "windows-platform", "subjects": dict(subjects),
            "critical_files": list(dict(subjects).values()), "tool_admission_profile": "stable-core",
            "coverage_minimum_line_percent": 90, "mutation_minimum_viable_killed_percent": 95,
            "coverage_wall_seconds": 300, "enumeration_wall_seconds": 120,
            "mutation_wall_seconds": 1200, "mutation_build_seconds": 90, "mutation_test_seconds": 60,
            "jobs": 6, "test_threads": 1, "coverage_test_threads": 1, "mutation_jobs": workers,
            "mutation_process_limit": process_limit, "mutation_message_format": "json", "maximum_mutants": 512,
            "mutation_equivalences": [],
        }
        if (profile != "stable-core" or args.lane not in {"coverage", "mutation", "all"}
                or type(scope) is not Scope or scope.package != "windows-platform"
                or scope.policy != "policy/service-analysis.json" or scope.subjects != subjects
                or type(policy) is not dict or set(policy) != set(facts) | {"binding", "mutation_inventories"}
                or any(type(policy.get(key)) is not type(value) or policy[key] != value for key, value in facts.items())):
            raise Incomplete("Service analysis requires its exact stable-only lifecycle role, gates and budgets.")
        return profile
    if (scope.subjects == ENGINE_CONTRACT_SUBJECTS
            or policy.get("admission_status") == "admitted-engine-external-contracts-only"):
        if profile != "stable-core" or args.lane != "contracts":
            raise Incomplete("The engine external-contract role admits only stable constructor contracts.")
        require_engine_contract_role(scope, policy)
        return profile
    if policy.get("admission_status") == "admitted-domain-external-contracts-only":
        if profile != "stable-core" or args.lane != "contracts":
            raise Incomplete("The domain external-contract role admits only stable constructor contracts.")
        require_domain_contract_role(scope, policy)
        return profile
    if (scope.subjects == DOMAIN_LEAP_SUBJECTS
            or (scope.policy == "policy/time-domain-analysis.json"
                and (profile == "stable-core" or policy.get("schema_version") == 4))):
        if profile != "stable-core" or args.lane not in {"coverage", "mutation"}:
            raise Incomplete("Leap-domain analysis requires its exact nine-owner stable coverage/mutation role.")
        require_domain_leap_role(scope, policy)
        return profile
    if (scope.policy == "tasks/settled-rate-inventory-admission.json"
            or policy.get("admission_status") == "admitted-settled-rate-four-owner-inventory-only"):
        facts = {
            "schema_version": 17, "admission_status": "admitted-settled-rate-four-owner-inventory-only",
            "package": "time-core", "source_file_count": 158,
            "subjects": dict(SETTLED_RATE_SUBJECTS), "mutation_equivalences": [],
            "mutation_error_values": {}, "native_test_invocations": 0,
            "campaigns_admitted": False, "tool_admission_profile": "stable-core",
            "maximum_mutants": 512, "listing_maximum_per_file": 512,
            "listing_maximum_total": 2048, "inventory_wall_seconds": 600,
            "listing_process_limit": 128, "mutation_process_limit": 512,
            "mutation_jobs": 10, "mutation_message_format": "json",
        }
        if (profile != "stable-core" or type(scope) is not Scope
                or type(scope.package) is not str or scope.package != "time-core"
                or type(scope.policy) is not str or scope.policy != "tasks/settled-rate-inventory-admission.json"
                or args.lane != "mutation" or type(scope.subjects) is not tuple
                or scope.subjects != SETTLED_RATE_SUBJECTS or type(policy) is not dict
                or any(type(policy.get(key)) is not type(value) or policy[key] != value
                       for key, value in facts.items())):
            raise Incomplete("Settled-rate inventory requires its exact stable-core four-owner listing-only card.")
        for _, filename in SETTLED_RATE_SUBJECTS:
            per_subject_mutation_error_args(scope, filename, policy)
        return profile
    if (scope.subjects == SETTLED_RATE_SUBJECTS
            or (scope.policy == "policy/time-core-analysis.json"
                and (policy.get("critical_files") == list(dict(SETTLED_RATE_SUBJECTS).values())
                     or policy.get("schema_version") == 26))):
        facts = {
            "schema_version": 26, "admission_status": "admitted-final-source",
            "package": "time-core", "source_file_count": 158,
            "critical_files": list(dict(SETTLED_RATE_SUBJECTS).values()),
            "mutation_equivalences": [], "mutation_error_values": {},
            "tool_admission_profile": "stable-core", "maximum_mutants": 512,
            "mutation_jobs": 10, "mutation_process_limit": 512, "mutation_message_format": "json",
        }
        if (profile != "stable-core" or type(scope) is not Scope
                or type(scope.package) is not str or scope.package != "time-core"
                or type(scope.policy) is not str or scope.policy != "policy/time-core-analysis.json"
                or type(scope.subjects) is not tuple or scope.subjects != SETTLED_RATE_SUBJECTS
                or args.lane not in {"contracts", "coverage", "mutation"} or type(policy) is not dict
                or any(type(policy.get(key)) is not type(value) or policy[key] != value
                       for key, value in facts.items())):
            raise Incomplete("Settled-rate campaigns require their exact stable-core final-source four-owner role.")
        for _, filename in SETTLED_RATE_SUBJECTS:
            per_subject_mutation_error_args(scope, filename, policy)
        return profile
    if (scope.policy == "tasks/s1.4b-chart-composition-inventory-admission.json"
            or policy.get("admission_status") == "admitted-chart-composition-three-owner-inventory-only"):
        facts = {
            "schema_version": 15,
            "admission_status": "admitted-chart-composition-three-owner-inventory-only",
            "package": "time-core", "source_file_count": 156,
            "subjects": dict(CHART_COMPOSITION_SUBJECTS), "mutation_equivalences": [],
            "mutation_error_values": {}, "native_test_invocations": 0,
            "campaigns_admitted": False, "tool_admission_profile": "stable-core",
            "maximum_mutants": 512, "listing_maximum_per_file": 512,
            "listing_maximum_total": 1536, "inventory_wall_seconds": 600,
            "listing_process_limit": 128, "mutation_process_limit": 512,
            "mutation_jobs": 10, "mutation_message_format": "json",
        }
        if (profile != "stable-core" or type(scope) is not Scope
                or type(scope.package) is not str or scope.package != "time-core"
                or type(scope.policy) is not str
                or scope.policy != "tasks/s1.4b-chart-composition-inventory-admission.json"
                or args.lane != "mutation" or type(scope.subjects) is not tuple
                or scope.subjects != CHART_COMPOSITION_SUBJECTS
                or type(policy) is not dict
                or any(type(policy.get(key)) is not type(value) or policy[key] != value
                       for key, value in facts.items())):
            raise Incomplete("Chart-composition inventory requires its exact stable-core three-owner listing-only card.")
        for _, filename in CHART_COMPOSITION_SUBJECTS:
            per_subject_mutation_error_args(scope, filename, policy)
        return profile
    if (scope.subjects == CHART_COMPOSITION_SUBJECTS
            or (scope.policy == "policy/time-core-analysis.json"
                and policy.get("critical_files") == list(dict(CHART_COMPOSITION_SUBJECTS).values()))):
        facts = {
            "schema_version": 24, "admission_status": "admitted-final-source",
            "package": "time-core", "source_file_count": 156,
            "critical_files": list(dict(CHART_COMPOSITION_SUBJECTS).values()),
            "mutation_error_values": {},
            "tool_admission_profile": "stable-core", "mutation_jobs": 10,
            "mutation_process_limit": 512, "mutation_message_format": "json",
        }
        if (profile != "stable-core" or type(scope) is not Scope
                or type(scope.package) is not str or scope.package != "time-core"
                or type(scope.policy) is not str or scope.policy != "policy/time-core-analysis.json"
                or type(scope.subjects) is not tuple or scope.subjects != CHART_COMPOSITION_SUBJECTS
                or args.lane not in {"contracts", "coverage", "mutation"}
                or type(policy) is not dict
                or not chart_composition_equivalences_valid(policy.get("mutation_equivalences"))
                or any(type(policy.get(key)) is not type(value) or policy[key] != value
                       for key, value in facts.items())):
            raise Incomplete("Chart-composition campaigns require their exact stable-core final-source three-owner role.")
        for _, filename in CHART_COMPOSITION_SUBJECTS:
            per_subject_mutation_error_args(scope, filename, policy)
        return profile
    if (scope.policy in ("tasks/s1.4a-reference-input-inventory-admission.json",
                         "tasks/s1.4c-achieved-phase-inventory-admission.json")
            or policy.get("admission_status") in ("admitted-reference-input-single-owner-inventory-only",
                                                "admitted-achieved-phase-single-owner-inventory-only")):
        achieved_phase = scope.policy == "tasks/s1.4c-achieved-phase-inventory-admission.json"
        facts = {
            "schema_version": 16 if achieved_phase else 14,
            "admission_status": ("admitted-achieved-phase-single-owner-inventory-only" if achieved_phase
                                 else "admitted-reference-input-single-owner-inventory-only"),
            "package": "time-core", "source_file_count": 156 if achieved_phase else 154,
            "subjects": dict(REFERENCE_INPUT_SUBJECTS), "mutation_equivalences": [],
            "mutation_error_values": {}, "native_test_invocations": 0,
            "campaigns_admitted": False, "tool_admission_profile": "stable-core",
            "maximum_mutants": 512, "listing_maximum_per_file": 512,
            "listing_maximum_total": 512, "inventory_wall_seconds": 600,
            "listing_process_limit": 128, "mutation_process_limit": 512,
            "mutation_jobs": 10, "mutation_message_format": "json",
        }
        if (profile != "stable-core" or type(scope) is not Scope
                or type(scope.package) is not str or scope.package != "time-core"
                or type(scope.policy) is not str
                or scope.policy not in {"tasks/s1.4a-reference-input-inventory-admission.json",
                                        "tasks/s1.4c-achieved-phase-inventory-admission.json"}
                or args.lane != "mutation" or type(scope.subjects) is not tuple
                or scope.subjects != REFERENCE_INPUT_SUBJECTS
                or type(policy) is not dict
                or any(type(policy.get(key)) is not type(value) or policy[key] != value
                       for key, value in facts.items())):
            raise Incomplete("Reference-input inventory requires its exact stable-core singleton listing-only card.")
        per_subject_mutation_error_args(scope, REFERENCE_INPUT_SUBJECTS[0][1], policy)
        return profile
    if (scope.subjects == REFERENCE_INPUT_SUBJECTS
            or (scope.policy == "policy/time-core-analysis.json"
                and policy.get("critical_files") == [REFERENCE_INPUT_SUBJECTS[0][1]])):
        facts = {
            "admission_status": "admitted-final-source", "package": "time-core",
            "critical_files": [REFERENCE_INPUT_SUBJECTS[0][1]], "mutation_equivalences": [],
            "mutation_error_values": {}, "tool_admission_profile": "stable-core",
        }
        if policy.get("schema_version") == 25:
            facts.update(mutation_jobs=10, mutation_process_limit=512, mutation_message_format="json")
        if (profile != "stable-core" or type(scope) is not Scope
                or type(scope.package) is not str or scope.package != "time-core"
                or type(scope.policy) is not str or scope.policy != "policy/time-core-analysis.json"
                or type(scope.subjects) is not tuple or scope.subjects != REFERENCE_INPUT_SUBJECTS
                or args.lane not in {"contracts", "coverage", "mutation"}
                or type(policy) is not dict
                or type(policy.get("schema_version")) is not int
                or type(policy.get("source_file_count")) is not int
                or (policy["schema_version"], policy["source_file_count"]) not in {(23, 154), (25, 156)}
                or any(type(policy.get(key)) is not type(value) or policy[key] != value
                       for key, value in facts.items())):
            raise Incomplete("Reference-input campaigns require their exact stable-core final-source singleton role.")
        per_subject_mutation_error_args(scope, REFERENCE_INPUT_SUBJECTS[0][1], policy)
        return profile
    if (scope.policy == "tasks/s1.3-chart-combination-inventory-admission.json"
            or policy.get("admission_status") == "admitted-chart-combination-nine-owner-inventory-only"):
        facts = {
            "schema_version": 13,
            "admission_status": "admitted-chart-combination-nine-owner-inventory-only",
            "package": "time-core", "source_file_count": 152,
            "subjects": dict(CHART_COMBINATION_SUBJECTS), "mutation_equivalences": [],
            "mutation_error_values": {}, "native_test_invocations": 0,
            "campaigns_admitted": False, "tool_admission_profile": "stable-core",
            "maximum_mutants": 512, "listing_maximum_per_file": 512,
            "listing_maximum_total": 4608, "inventory_wall_seconds": 600,
            "listing_process_limit": 128, "mutation_process_limit": 512,
            "mutation_jobs": 10, "mutation_message_format": "json",
        }
        if (profile != "stable-core" or type(scope) is not Scope
                or type(scope.package) is not str or scope.package != "time-core"
                or type(scope.policy) is not str
                or scope.policy != "tasks/s1.3-chart-combination-inventory-admission.json"
                or args.lane != "mutation" or type(scope.subjects) is not tuple
                or scope.subjects != CHART_COMBINATION_SUBJECTS
                or type(policy) is not dict
                or any(type(policy.get(key)) is not type(value) or policy[key] != value
                       for key, value in facts.items())):
            raise Incomplete("S158 inventory requires its exact stable-core nine-owner listing-only card.")
        for filename in dict(CHART_COMBINATION_SUBJECTS).values():
            per_subject_mutation_error_args(scope, filename, policy)
        return profile
    if (scope.policy == "tasks/s1.3-chart-principal-inventory-admission.json"
            or policy.get("admission_status") == "admitted-retained-chart-three-owner-inventory-only"):
        facts = {
            "schema_version": 12,
            "admission_status": "admitted-retained-chart-three-owner-inventory-only",
            "package": "time-core", "source_file_count": 142,
            "subjects": dict(CHART_PRINCIPAL_SUBJECTS), "mutation_equivalences": [],
            "mutation_error_values": {}, "native_test_invocations": 0,
            "campaigns_admitted": False,
        }
        if (profile != "stable-core" or type(scope) is not Scope or scope.package != "time-core"
                or scope.policy != "tasks/s1.3-chart-principal-inventory-admission.json"
                or args.lane != "mutation" or scope.subjects != CHART_PRINCIPAL_SUBJECTS
                or any(type(policy.get(key)) is not type(value) or policy[key] != value
                       for key, value in facts.items())):
            raise Incomplete("S157 inventory requires its exact stable-core three-owner listing card.")
        for filename in dict(CHART_PRINCIPAL_SUBJECTS).values():
            per_subject_mutation_error_args(scope, filename, policy)
        return profile
    if (scope.policy == "tasks/s1.2-chart-batch-inventory-admission.json"
            or policy.get("admission_status") == "admitted-single-owner-chart-batch-inventory-only"):
        facts = {
            "schema_version": 11,
            "admission_status": "admitted-single-owner-chart-batch-inventory-only",
            "package": "time-core", "source_file_count": 139,
            "subjects": dict(CHART_BATCH_SUBJECTS), "mutation_equivalences": [],
            "mutation_error_values": {}, "native_test_invocations": 0,
            "campaigns_admitted": False,
        }
        if (profile != "stable-core" or type(scope) is not Scope or scope.package != "time-core"
                or scope.policy != "tasks/s1.2-chart-batch-inventory-admission.json"
                or args.lane != "mutation" or scope.subjects != CHART_BATCH_SUBJECTS
                or any(type(policy.get(key)) is not type(value) or policy[key] != value
                       for key, value in facts.items())):
            raise Incomplete("S155 inventory requires its exact stable-core single-owner listing card.")
        per_subject_mutation_error_args(scope, dict(CHART_BATCH_SUBJECTS)["chart_batch"], policy)
        return profile
    if (scope.policy == "tasks/s1.2-chart-inventory-admission-v2.json"
            or policy.get("admission_status") == "admitted-four-owner-chart-error-inventory-only"):
        facts = {"schema_version": 10, "admission_status": "admitted-four-owner-chart-error-inventory-only",
                 "package": "time-core", "source_file_count": 136, "subjects": dict(CHART_ERROR_SUBJECTS),
                 "mutation_equivalences": [], "native_test_invocations": 0, "campaigns_admitted": False}
        if (profile != "stable-core" or scope.package != "time-core"
                or scope.policy != "tasks/s1.2-chart-inventory-admission-v2.json"
                or args.lane != "mutation" or scope.subjects != CHART_ERROR_SUBJECTS
                or "mutation_error_values" not in policy
                or any(type(policy.get(key)) is not type(value) or policy[key] != value
                       for key, value in facts.items())):
            raise Incomplete("S154c error inventory requires its exact stable-core listing-only card.")
        per_subject_mutation_error_args(scope, dict(CHART_ERROR_SUBJECTS)["chart_enclosure"], policy)
        return profile
    if scope.policy == "tasks/s1.3-tracking-inventory-admission.json":
        # The wrapper has already admitted the strict, fresh, source-bound card.
        # This second boundary permits only its exact listing role; the Session
        # mutation lane selects the sealed cargo-mutants executable, while the
        # wrapper admits only --list --json without mutation runtime options.
        subjects = (("projection", "crates/time-core/src/regression/projection.rs"),
                    ("combination", "crates/time-core/src/combination.rs"),
                    ("combination_admission", "crates/time-core/src/combination/admission.rs"),
                    ("combination_lifecycle", "crates/time-core/src/combination/lifecycle.rs"))
        facts = {"schema_version": 8, "admission_status": "admitted-four-owner-inventory-only",
                 "package": "time-core", "subjects": dict(subjects),
                 "native_test_invocations": 0, "campaigns_admitted": False}
        if (profile != "stable-core" or scope.package != "time-core"
                or args.lane != "mutation" or scope.subjects != subjects
                or any(type(policy.get(key)) is not type(value) or policy[key] != value
                       for key, value in facts.items())):
            raise Incomplete("S152 inventory requires its exact stable-core listing role and noncampaign card.")
        return profile
    if (scope.policy == "tasks/s1.2-chart-inventory-admission.json"
            or policy.get("admission_status") == "admitted-four-owner-chart-inventory-only"):
        # A chart listing card cannot borrow the generic campaign-policy path or
        # the full-profile fallback. The strict wrapper separately binds every
        # source/tool byte, freshness and exact listing command before Session.
        subjects = (("chart", "crates/time-core/src/regression/chart.rs"),
                    ("chart_roundoff", "crates/time-core/src/regression/chart/roundoff.rs"),
                    ("chart_enclosure", "crates/time-core/src/chart_enclosure.rs"),
                    ("enclosure", "crates/time-core/src/projection_enclosure.rs"))
        facts = {"schema_version": 9, "admission_status": "admitted-four-owner-chart-inventory-only",
                 "package": "time-core", "source_file_count": 136, "subjects": dict(subjects),
                 "mutation_equivalences": [], "native_test_invocations": 0, "campaigns_admitted": False}
        if (profile != "stable-core" or scope.package != "time-core"
                or scope.policy != "tasks/s1.2-chart-inventory-admission.json"
                or args.lane != "mutation" or scope.subjects != subjects
                or any(type(policy.get(key)) is not type(value) or policy[key] != value
                       for key, value in facts.items())):
            raise Incomplete("S154 inventory requires its exact stable-core chart listing role and noncampaign card.")
        return profile
    if profile == "stable-core" and (
            scope.package != "time-core" or scope.policy != "policy/time-core-analysis.json"
            or args.lane not in {"contracts", "coverage", "mutation"}):
        raise Incomplete("Stable-core admission is restricted to native core contracts/coverage/mutation.")
    return profile


class Session:
    """One immutable candidate, admitted graph/tools and private bounded work tree."""

    def __init__(self, root, args, report, scope, policy, extra_inputs):
        self.root, self.report, self.scope, self.policy = Path(root), report, scope, policy
        self.admission_profile = admission_profile(args, scope, policy)
        _require_private_output(self.root, args.output_dir)
        self.dependency_policy, self.product_identity = PRODUCT.admit_product(self.root)
        external_contracts = policy.get("admission_status") in {
            "admitted-engine-external-contracts-only", "admitted-domain-external-contracts-only"}
        if external_contracts:
            require_contract_candidate(self.product_identity, policy)
        elif (scope.subjects in (DOMAIN_LEAP_SUBJECTS, ENGINE_CONTRACT_SUBJECTS)
                and len(self.product_identity) != policy["source_file_count"]):
            raise Incomplete("Contract-role product input count differs from its admitted role.")
        self.identity = dict(self.product_identity)
        equivalences = admit_mutation_equivalences(self.root, scope, policy)
        proof_inputs = tuple(entry["proof"]["path"] for entry in equivalences)
        names = (scope.policy, "scripts/portable-analysis.py",
                 "scripts/prepare-product.py", "scripts/analysis-runtime.py", "scripts/check-analysis.py",
                 "policy/analysis-tools.json", "policy/verification-manifest.toml", *extra_inputs, *proof_inputs)
        self.identity.update({name: PRODUCT.sha256(self.root / name) for name in names})
        self.admitted = (ANALYSIS.admit(self.root, profile="stable-core")
                         if self.admission_profile == "stable-core" else ANALYSIS.admit(self.root))
        self.base, self.stable = self.admitted["base"], self.admitted["stable"]
        self.payload_readonly = (self.admitted["readonly_roots"] if self.admission_profile == "stable-core"
                                 else (self.base, self.stable))
        self.vendor = args.vendor_dir.resolve(strict=True)
        self.vendor_identity = PRODUCT.admit_vendor(self.vendor, self.dependency_policy)
        self.output = args.output_dir
        _require_private_output(self.root, self.output)
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
        if external_contracts:
            report.update(product_candidate_sha256=canonical_sha256(self.product_identity),
                          product_source_file_count=len(self.product_identity))
        report["tool_admission_profile"] = self.admission_profile
        report["analysis_admission"] = {
            "profile": self.admission_profile,
            "inventories": self.admitted["inventories"],
            "readonly_roots": [str(path) for path in self.payload_readonly],
        }
        if self.admission_profile == "stable-core":
            report["analysis_admission"].update({
                key: self.admitted[key] for key in ("installation_pointer", "installation_pointer_sha256")})

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
                                      expected_message_format=mutation_message_format(self.policy), policy=self.policy)
        compiler = compiler or self.stable
        night = self.base / self.admitted["policy"]["common_nightly"]
        if self.admission_profile == "stable-core" and (compiler != self.stable or extra_tools is not None):
            raise Incomplete("Stable-core cannot use a nightly compiler or extra tool island.")
        if compiler not in (self.stable, night):
            raise Incomplete("Unadmitted analysis compiler.")
        if extra_tools is not None and extra_tools not in self.protected_tools:
            raise Incomplete("Unadmitted extra tool island.")
        # Mutants parallelize independent copies; each nested Cargo/test process
        # gets one slot rather than multiplying twelve workers by twelve jobs.
        inner_jobs = 1 if label.startswith("mutation-") else self.policy.get("jobs", RUNTIME.MAX_JOBS)
        environment = {"RUSTC": str(compiler / "bin/rustc"), "RUSTDOC": str(compiler / "bin/rustdoc"),
                       "RUST_TEST_THREADS": str(inner_jobs)}
        if compiler != self.stable:
            environment.update(RUSTUP_HOME=str(self.base / "common/rustup"), RUSTUP_TOOLCHAIN=ANALYSIS.NIGHTLY)
        readonly = [*self.payload_readonly, self.vendor, self.tools, *([extra_tools] if extra_tools else [])]
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
                                            expected_message_format=mutation_message_format(self.policy), policy=self.policy)
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
        if (label.startswith("mutation-enumeration:") or label.startswith("ke-history-contract-")
                or label in ("external-legal-execution", "external-adaptive-legal-execution",
                             "external-chart-legal-execution",
                             "external-chart-combination-legal-execution")):
            # Strict counter/receipt records need the child's actual delimiters.
            # Admission still requires a successful child result before exposure.
            PRODUCT.ENV.require_ok(result)
            return result["output"]
        return PRODUCT.ENV.require_ok(result)

    def finish(self):
        """Re-admit all source/vendor/tool inputs and the same selected payload closure."""
        self.check()
        if PRODUCT.admit_vendor(self.vendor, self.dependency_policy) != self.vendor_identity:
            raise Incomplete("Vendor bytes changed during portable qualification.")
        after = (ANALYSIS.admit(self.root, profile="stable-core")
                 if self.admission_profile == "stable-core" else ANALYSIS.admit(self.root))
        keys = ("base", "stable", "policy_sha256", "inventories")
        if self.admission_profile == "stable-core":
            keys += ("admission_profile", "readonly_roots", "installation_pointer", "installation_pointer_sha256")
        if any(after[key] != self.admitted[key] for key in keys):
            raise Incomplete("Analysis installation/payload identities changed during execution.")
        self.report["analysis_admission_after"] = {
            "profile": self.admission_profile, "inventories": after["inventories"],
            "unchanged": True,
        }
