# Independent review of the basic feature plan

**Date:** 28 October 2026  
**Reviewer:** Independent child reviewer `/root/review_basic_features`; did not author the integrated plan or either contributor report.  
**Scope:** Proposed feature-plan handoff for the user's request, “great! now think about basic features and plan them.” Planning only.  
**First-pass verdict:** **REVISE** for two small requirements clarifications below. This is not a blocked architecture decision and is not a judgment that runtime checks failed.

**Final verdict after recheck:** **READY for feature-plan handoff** at SHA-256 `7e227274669ad75e0c4d5c4534638393e09f5ad8982a923df5300e755a1c8f96`. Both first-pass findings are resolved; no finding remains open in this review scope. This is not implementation or public-release readiness.

## Reviewed identity and authority

The integrated feature plan was read before any contributor report. Neither `ux.md` nor `safety.md` was used to derive the first-pass findings. The findings below are retained as the first-pass record; a subsequent recheck must append its outcome rather than rewrite that history.

| Source | Revision / SHA-256 |
| --- | --- |
| `deliverables/windows-time-features.md` | RWTC-FEATURES-001 revision 1.0; `f2d9cdb4d6a796ccc5b4221b7c4781c4aa26860204c3709096ce58bb9ca90e1c` |
| `deliverables/windows-time-architecture.md` | RWTC-ARCH-001 revision 1.4; `a9bd1432ecb2d04e55b77e1a270eb1a135027d9f4a7988d58d9299367a6ca075` |
| `deliverables/windows-time-development-hardening.md` | `fdc0e88f04d31ebca05c739f79155fd14216c28733b1876ff99c78526a54c904` |

The feature-plan hash was unchanged between the initial read and the first-pass structural check. The shared documentation directory is not a Git worktree: `git status --short` and `git rev-parse` returned “not a git repository.” Exact file hashes, rather than an invented HEAD or clean-worktree claim, identify this review.

Authority inspected: architecture §§1, 4.2, 5–12, 16–17, 19–23; the development-hardening acceptance/gate provisions; and the pinned website guidance's `AGENTS.md`, `plan-independent-review/SKILL.md`, testing `CONTRACT.md`, security `CONTRACT.md`, and selected Security Constitution provisions. The native adaptation in architecture §23 governs; website commands, browser-specific policy and automatic locale expansion were not imported into this product.

## First-pass findings

### BF-REV-1 — Low: distinguish observe-only startup from transition cleanup

**Requirement / boundary:** F-06/F-07, AC-08; observe-only enforcement and current clock ownership.

**Evidence:** In the reviewed feature plan, AC-08 begins, “Enter/restart observe-only without any product clock write.” Its next sentence requires cancelling attributable work when entering from active mode. F-06 separately requires safe cancellation during that change. Architecture §10.1 treats cancellation/reset as clock writes requiring current ownership, and §9 requires read-only initialization for a process that is already in observe-only mode.

**Problem:** The first AC-08 sentence is an unqualified forbidden-effect assertion, so it can be read to forbid the very clock write required to cancel a known active product adjustment during a transition. This is a conflict in the written acceptance boundary, not evidence of an implemented clock bug. A test author needs to distinguish an already-observing startup from a transition out of active control.

**Smallest correction:** State that startup/restart **already in observe-only** makes no product clock write. For an active-to-observe-only transition, cancel attributable work while current ownership is proven, before entering the read-only state. If ownership is lost or ambiguous, stop pending internal work and report the condition without a global reset. Keep the existing rule that this is not an implicit Windows Time handback. Owner: integrated-plan author; no architecture change required.

### BF-REV-2 — Low: include certificate-time provenance in the visible status contract

**Requirement / boundary:** F-04/F-08, source details, state presentation and AC-09/10; distinguish packet authentication from certificate-time basis.

**Evidence:** Architecture §8.3 requires separate packet-authentication and certificate-time-basis fields, retained bootstrap history, and a discoverable history note after successful recovery. Section 8.2 requires an explicit explanation of the weaker one-helper policy. The feature plan's F-08 correctly limits helper authority, and AC-10 retains weaker one-helper provenance. However, its Sources field list and feature-facing status task do not explicitly carry the certificate-time basis or successful recovery history to a user-visible consumer. Journey C's explicit retention instruction is attached to a failed gate. The provisional copy says “approximate date” without itself stating its unauthenticated basis.

**Problem:** The feature plan's detailed presentation requirements could be satisfied with a plain “NTS authenticated” view after successful hint-assisted recovery while the required certificate-time provenance exists only in internal state. The underlying architecture still forbids losing that information, so this is a localized UI/acceptance omission, not a proposed NTS downgrade or a change in the security architecture.

**Smallest correction:** Add one explicit presentation requirement that Sources/Diagnostics expose packet authentication separately from the certificate-time basis and retain a discoverable recovery-history note after success. Explain on one-helper opt-in that an unauthenticated source supplies the date used to check certificates, and identify that basis during recovery. Extend AC-09 or AC-10 to observe the retained, accessible note after successful recovery. Reuse existing source details/diagnostics; no new screen, service or feature is needed. Owner: integrated-plan author.

## Scope and consistency findings

No further finding in the inspected scope:

| Area | Review conclusion |
| --- | --- |
| Basic product priorities | Background operation, diagnosis, recovery, bounded control and a usable small UI directly address the stated product problem. No unsupported accuracy or calendar commitment was introduced. |
| Feature count and defaults | The overview and detailed sections both contain exactly F-01 through F-10; AC-01 through AC-20 and S-0 through S-6 are unique. NTS default, deliberate NTP, helper-off default, minimum-two-plus-majority, four-provider target, English first, no default UI retention and two separate tray opt-ins are consistent. |
| Health and evidence | Service availability, authentication, agreement, current quality and four freshness meanings remain distinct. Historical snapshots cannot become current merely through reconnect. Requested correction is not presented as applied correction. BF-REV-2 covers the remaining recovery-provenance presentation gap. |
| Sync now | Requests current-policy measurement, carries neither a destination nor target time, remains bounded and cannot grant exceptional correction authority or reset budgets. |
| Source and recovery trust | No fastest-peer substitution, silent single-source fallback, mixed authenticated/plain voting, helper vote, helper clock write or NTS-to-NTP downgrade. Windows verifier and date/era gates remain unresolved where required. |
| Ownership and recovery | Managed observation, current-owner checks, no automatic Windows Time fallback, generation invalidation and durable budgets are retained. BF-REV-1 concerns only the acceptance wording for transition cleanup. |
| UI closure and visibility | Default closure leaves no UI process. Tray is opt-in, per-user, separately measured and not an independent watchdog. The inability to promise an immediate warning with no surviving observer is stated plainly. |
| Settings and support | Ordinary inspection and preferences are separate from elevated policy operations. Draft/active revisions, conflict rejection and UAC cancellation are explicit. Redacted support export is written by the ordinary UI and grants no arbitrary privileged path. |
| Distribution and scope | One complete MSI owns install/update/repair/uninstall. No hidden updater, second product, dependency selection, license clearance, TUF adoption or architecture change is introduced. Deferred features remain deferred. |
| Delivery and evidence | The plan supplies integrated slices, independent expectations and valid/failure controls; later numeric/provider/native/signing decisions remain gates. Prototype/mocked/document evidence is not promoted to a runtime pass. |

## Checks performed and limits

The review used read-only local text inspection, `rg`, bounded reads, SHA-256 identity checks, and one small read-only document check for feature/acceptance/slice IDs and existing local Markdown link targets. All three local linked Markdown deliverables were present. The read-only Git identity probe's unavailable result is recorded above.

No contributor verdict was used as evidence. No tool was installed, no product code was executed, no native Windows state was changed, and no signing or package operation occurred. No external research was needed to resolve these local plan-to-architecture questions. The source documents' external claims were not independently re-browsed during this review.

All Windows, performance, accessibility, security, license, provider and signing acceptance gates remain planned/unexecuted as the feature plan states. Their absence is expected in this planning task, not a document defect. Readiness here applies only to feature-plan handoff; it grants no implementation, native execution, publication or release authorization.

**Next permitted action:** The integrated-plan owner makes the two narrow corrections, then the independent reviewer checks the affected sections and records the exact corrected hash and final handoff verdict below.

## Recheck record

### Recheck 1

Reviewed feature-plan SHA-256: `8d092e5d097f877642875084e365f66ad76a24866c3db021aaf01fd281e1fd63`. Architecture and development-hardening hashes remained unchanged.

- **BF-REV-1 resolved:** AC-08 now distinguishes startup/restart already in observe-only from cleanup during an active-to-observe transition. Cleanup is conditioned on valid ownership, lost/ambiguous ownership prevents a blind reset, and no automatic Windows Time handback was added.
- **BF-REV-2 mostly resolved:** §3.2 now carries service-published certificate-time basis separately from packet authentication and preserves discoverable successful history for both helper modes. AC-09/10 check retention after success. The first-pass requirement for an explicit weaker one-helper opt-in explanation and an unauthenticated label during recovery remained implicit; the same finding was returned for that final sentence.
- **Additional author clarifications checked:** F-01 now discloses configured time-server and admitted DNS/certificate-validation traffic. F-06 limits ordinary settings/source inspection to installation-policy-permitted summaries and leaves protected material service-owned. These narrow additions are consistent with the existing authority, egress and privacy contracts and do not add telemetry or an elevated read path.

No additional finding was introduced. Final recheck pending the remaining BF-REV-2 sentence.

### Recheck 2 — final

**Reviewed:** RWTC-FEATURES-001 revision 1.0, final feature-plan SHA-256 `7e227274669ad75e0c4d5c4534638393e09f5ad8982a923df5300e755a1c8f96`. The canonical architecture and development-hardening hashes remain the originals recorded above.

F-08 now explicitly requires a weaker-policy explanation when enabling one-helper mode: one unauthenticated source supplies the date used to check certificates. Both helper modes identify the unauthenticated date basis during recovery, and the text expressly denies that two agreeing helpers authenticate the hint. The §4 recovery headline now includes “unauthenticated approximate date.” These statements agree with the retained certificate-time basis and successful history in §3.2 and AC-09/10. **BF-REV-2 is resolved.** BF-REV-1's corrected acceptance boundary remains intact.

The bounded final structural check still finds the same ten feature IDs in the overview and detailed sections, twenty acceptance cases, and seven delivery slices. The corrections do not introduce another feature, authority path, dependency, updater or release exception.

**Verdict:** **READY for feature-plan handoff.**  
**Open findings:** none within the named scope.  
**Next:** deliver the proposed basic-feature plan and existing architectural gates to the user. Repository creation and implementation remain separate work when requested. No further research, product execution, native experiment or package operation is required to complete this document review; all such implementation/release evidence remains unexecuted and required at its existing dependent gate.
