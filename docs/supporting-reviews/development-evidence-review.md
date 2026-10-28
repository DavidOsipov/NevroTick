# Independent review: CI trust, isolation and test evidence

Review date: 28 October 2026. Reviewer: `/root/review_hardening_evidence`.

## Scope, independence and method

I read `deliverables/windows-time-development-hardening.md` before reading any research-worker reports; no research-worker reports were needed for this review. The initial proposal inspected was revision 1.0, SHA-256 `108b74a2fc723cdfaa5da0d71486da51e947f1b244b5297a49b5d33ba22c305b`. The canonical architecture inspected was revision 1.4, SHA-256 `a9bd1432ecb2d04e55b77e1a270eb1a135027d9f4a7988d58d9299367a6ca075`.

This review covers proposed CI/supply-chain authority, isolation, test discovery/outcome semantics, coverage/mutation gates, Linux-versus-Windows claims, phase readiness and agent authority. It is a document review, not a product security audit or executed qualification. Only bounded local text reads, hashing and primary-source browsing were performed. No tools were installed, no source was built, no scanners/tests/native experiments were run, and no deployment or repository setting was changed. The coordinator owns all deliverable edits; this reviewer owns only this evidence file.

I read the pinned `architecture-research/website-guidance/AGENTS.md`, security `CONTRACT.md` and candidate `policy-amendments.md`, testing `CONTRACT.md`, `qa-independent-review/SKILL.md`, `qa-mutation-testing/SKILL.md`, `security-build-deployment/SKILL.md`, and the available shared QA phase contract. The build/deployment leaf's referenced `references/implementation.md` was absent from the supplied snapshot; an exact file search did not find an equivalent local file. I did not treat that missing optional implementation reference as permission to relax the available mandatory contracts.

Canonical cross-checks included architecture §§18.4, 19.5, 20–23 and the exact crypto hold in §18.5. Web checks used three lookup batches and no Context7 calls.

## First-pass findings — retained

These findings concern specification precision and future implementation traps. They are not claims that a product repository or deployed system already contains the described failures.

### E-01 — Medium: survivor disposition was weaker than the canonical contract

**Location:** Initial proposal §9.4, outcome table and paragraph defining the 95% gate.

**Observed text and conflict:** The first proposal required a disposition for every survivor and prohibited accepting a known non-equivalent security defect merely because the score passed. Architecture §18.4 requires survivors to receive stronger tests or independently justified equivalence/exclusion. A generic disposition is a weaker instruction: a developer could record a backlog entry for a non-equivalent survivor in a critical file and still claim the adjusted numerical gate passed.

**Consequence:** The preserved 95% number could coexist with a weakened survivor acceptance rule. This is a contract mismatch, not evidence that any actual test suite has missed a bug.

**Required correction:** State the architecture's survivor rule explicitly. Require stronger tests or independently justified equivalence/exclusion for every survivor; a backlog entry, generic risk acceptance or passing numerical score does not discharge it. Keep raw and adjusted outcomes visible and preserve 95% per critical source file. Do not turn compile failures/timeouts into kills.

**Basis:** Architecture §18.4; pinned mandatory testing contract and mutation-testing guidance. The upstream [cargo-mutants outcome documentation](https://mutants.rs/using-results.html) explains the distinct native outcomes but does not supersede the stronger project contract.

**Correction recheck:** The coordinator reported editing §9.4. A subsequent direct read confirmed the explicit stronger-tests/equivalence/exclusion rule and the rejection of backlog/generic-risk/percentage-only discharge. The M/E outcome descriptions were aligned. **Resolved as a documentation finding.** No mutation execution was performed or implied.

### E-02 — Medium: required-status orchestration must reject GitHub's skipped-success semantics

**Location:** §§8.3, 11.1 and 11.2, with the evidence-state rule in §11.3.

**Observed gap:** The proposal correctly says skipped required work cannot be PASS and requires checks through the actual protected status. It does not yet identify the GitHub orchestration behavior that can bypass an ordinary dependent final job: a conditionally skipped job can satisfy a required check, and a dependent job can itself be skipped after an upstream failure.

**Concrete failure path:** A required final gate declares `needs` but runs only under the default successful-dependency condition. A native test job fails; the final gate is skipped; the service can treat that skip as a successful required check. A receipt rule inside a gate that never runs cannot correct this. This is a documented platform behavior, not an observed execution here.

**Required correction:** Specify one final gate that is scheduled even when dependencies fail or are skipped, evaluates the expected job/matrix/shard set against explicit successful outcomes and validated identity-bound receipts, and rejects cancellation, neutral/skipped states, missing output and missing scope. `always()` is a scheduling ingredient, not a verdict; the evaluator must fail when required dependencies did not succeed. Keep the required check bound to the expected application/origin and correct PR/merge candidate. If merge queues are used, qualify the required `merge_group` path. Add qualification controls for a failed prerequisite, a deliberately skipped required job, a cancelled or absent shard, and the valid complete neighboring run. Do not use `continue-on-error` to erase the relevant failure.

**Basis:** GitHub's [troubleshooting required status checks](https://docs.github.com/en/pull-requests/how-tos/merge-and-close-pull-requests/troubleshooting-required-status-checks), especially the skipped-check and unexpected-source sections; canonical CHECK-19/CHECK-24 and pinned test-evidence rules. GitHub's [workflow syntax](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax) documents the `needs`/`always()` scheduling mechanism.

**Status at first-pass closeout:** Open; coordinator notified. No repository ruleset or CI gate was executed or inspected.

### E-03 — Medium: qualify the privilege boundary of the proposed hosted Windows build

**Location:** §§3.2–3.3, §8.1, DEV-01/DEV-05 in §12.2.

**Observed gap:** The policy correctly requires unprivileged execution and separates the Windows laboratory from the hosted signing flow. It should explicitly reconcile those requirements with the selected hosted environment: GitHub documents that its Windows VMs run as administrator with UAC disabled, while conventional Linux/macOS VMs offer passwordless sudo. Merely selecting a fresh hosted VM is therefore not evidence that the target-controlled compiler, proc macro, build script or analyzer runs inside the required unprivileged boundary.

**Concrete risk:** An implementation could equate ephemeral hosted execution with §3.3 compliance and run hostile build inputs under the default administrative identity. Such inputs can affect machine state, gate inputs and later steps within that job. Separately, the standard x64 Windows hosted images are server-image families; their success does not establish the intended Windows 11 client behavior. This inference follows from the provider defaults and the document's intended use; no attack was executed.

**Required correction:** Add the provider defaults as a named DEV-01 qualification item. Require an actually demonstrated reduced-authority execution boundary or a separately approved compatible hosted design, including absence of an elevation route and protection of trusted gate/tool inputs from target writes. Do not claim that a reduced token or OS container alone satisfies every filesystem/network/process constraint. Retain separate Windows 11 native/runtime/lifecycle evidence and the SignPath-approved origin flow. If this combination cannot be demonstrated, mark the dependent hosted lane blocked rather than silently relaxing isolation or claiming the server runner proves client behavior.

**Basis:** GitHub's [hosted runner reference](https://docs.github.com/en/actions/reference/runners/github-hosted-runners), administrative-privilege and supported-image sections; [secure-use reference](https://docs.github.com/en/actions/reference/security/secure-use), including shared-directory/process compromise paths; canonical architecture §§19.5, 20.3 and P0/P6/P7. GitHub's [CodeQL Rust build-mode documentation](https://docs.github.com/en/code-security/reference/code-scanning/codeql/build-options-for-compiled-languages) confirms that `build-mode: none` still compiles/runs build scripts and compiles macro code.

**Status at first-pass closeout:** Open; coordinator notified. No runner configuration, reduced token, container or provider approval was qualified.

## Requirements preserved and useful existing controls

- The exact 90% executable-line threshold remains per security module; branch coverage is separate. The exact 95% mutation gate remains per critical file. Empty or unsupported required scope is incomplete, not a pass.
- The source-derived GPL-2.0-only hold on the named AWS-LC/ring production graphs remains explicit. Nothing in development-tool licensing or scanning clears the linked distribution.
- TUF remains unadopted, automatic privileged updates remain outside the selected design, and the product still ships as one complete MSI.
- Rust/Windows and Linux evidence are separated. Windows-only mutation enumeration is not execution; Linux coverage does not establish `cfg(windows)` coverage. Native authority, cleanup, accessibility and installer evidence remain mandatory.
- The document recognizes CodeQL Rust's target-controlled execution, zizmor SARIF exit-code suppression, exact feature graphs, cache/report trust, secret separation and post-signing identity changes.
- Phase gates allow bounded P0 work without pretending optional model tools, public signing or later complete-product checks already exist. Agent proposals cannot approve their own weakened gates; privileged experiments require bounded independently observed ownership.

## Remaining limitations

This review establishes only text-level consistency and the cited current provider/tool behaviors. It does not validate the exact future Rust/SDK graph, cargo-deny schema, coverage denominator, mutant reports, all tool versions, sandbox enforcement, hosted settings, signing eligibility or final MSI lineage. No percentage of bug reduction can be inferred. All actual checker controls, native tests, thresholds and release gates remain planned until executed on an identified candidate.

The source snapshot and rolling upstream documentation are research evidence, not an installed or executed qualification. No absence-of-vulnerability, ASVS conformance or release-readiness conclusion is issued.

## Correction recheck and scoped conclusion

After the coordinator's corrections, I directly reread §§3.3, 8.3, 9.4, 11.2 and the new primary-source entries. The checked proposal has SHA-256 `763286b2bfa4c19271314039de187710f9667e4bedaf2bbd36ded591328078b5`; the architecture hash remains unchanged.

| Finding | Directly observed correction | Documentation disposition |
| --- | --- | --- |
| E-01 | §9.4 M/E definitions and the survivor paragraph explicitly retain stronger tests or independently justified equivalence/exclusion; generic disposition cannot discharge the requirement | Resolved |
| E-02 | §8.3 requires a protected always-scheduled final gate with explicit successful expected scope and identity-bound evidence, names disallowed outcome states and correct events/check source; §11.2 adds failed/skipped/cancelled/missing/forged-output controls | Resolved |
| E-03 | §3.3 names hosted Windows administrator/UAC defaults, requires an actually qualified reduced-authority boundary or approved alternative, and separates Windows 11 client qualification; §11.2 adds hosted-authority escape controls | Resolved |

The first-pass findings above are retained to preserve their original basis and correction history. I have no remaining substantive finding in the assigned document-review scope after these corrections. This closes only the written-policy review: all future execution, isolation, checker, source/artifact and release qualifications remain pending. No new web lookup, tool installation or behavioral execution was needed for the correction recheck.

### Final identity recheck after compiler-review clarification

I directly reread the added §5.2 paragraph qualifying `clippy::exit` at `main` and requiring explicit shutdown review plus a fatal-path control. It is consistent with this review's distinction between a lint result and actual behavioral evidence. The compiler review owns its upstream lint-semantics check; this bounded recheck performed no additional web research.

I also directly reread the E-01/E-02/E-03 correction paragraphs and their new §11.2 controls; all remain present and unchanged in substance. Final proposal SHA-256: `fdc0e88f04d31ebca05c739f79155fd14216c28733b1876ff99c78526a54c904`. Architecture SHA-256 remains `a9bd1432ecb2d04e55b77e1a270eb1a135027d9f4a7988d58d9299367a6ca075`. The scoped conclusion is unchanged: all three documentation findings are resolved, with no remaining substantive finding in this review scope and all actual qualification still pending.
