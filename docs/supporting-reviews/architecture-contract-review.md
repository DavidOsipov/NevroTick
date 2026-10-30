# Critical assessment of architecture contracts

**Date:** 30 October 2026  
**Baseline:** checkout at `12db217` plus identified working-tree amendments  
**Scope:** design/source review and primary-source verification; no native qualification

The supplied assessment is substantially sound. Retain the process split, pure
core, broker-owned authority, acknowledgement-based feedback, strict construction
and separate authentication/agreement/freshness/uncertainty. Its strongest findings
are missing precision in contracts, rather than demonstrated vulnerabilities.

The [architecture](../windows-time-architecture.md) remains authoritative. Its
[acceptance refinements](../windows-time-architecture.md#2021-contract-review-acceptance-refinements)
and [hardening matrix](../windows-time-development-hardening.md#910-contract-review-qualification-matrix)
turn accepted points into observable cases using existing CHECK identifiers.
The [execution plan](../plans/development-plan.md#82-contract-review-workstream-and-interface-gates)
places them at dependent stages. This review neither restarts implementation nor
qualifies current fixture work as production behavior.

## Findings and disposition

| Finding | Critical conclusion and adopted refinement |
| --- | --- |
| 1. LocalService owner rights | Valid conditional gap. A narrow DACL alone need not isolate a shared-account owner. Qualify default owner/OWNER RIGHTS or the existing distinct-account alternative through creation/replacement and relevant objects. Test a second LocalService service; not every restricted token can necessarily exploit this. Architecture §§4.1/12.1; CHECK-1/15. |
| 2. Durable correction transaction | Valid ordering gap. Reserve identified authority durably before the effect, persist known outcome before successful acknowledgement, retain charged unknowns and deduplicate replay. Separate journal/Windows calls cannot guarantee exactly-once successful effects across arbitrary crashes. Slew delivery/settlement and persistent rate after broker death remain distinct. §10.2.1; CHECK-10/12/15. |
| 3. Adjustment modes | Valid API distinction. `TRUE` releases programmed adjustment to Windows internal synchronization; it does not merely subtract transient correction. Model owned base, owned base plus transient and explicit release. Qualify mode/value and trajectory, without inventing a neutral constant. §§9/10.1; CHECK-10/11/12. |
| 4. NTS authentication lifetime | Valid refinement of acknowledged research. Track observation, cookie, key and original full-authentication ages plus trust revision. Bound cumulative resumption and association lifetime before reuse. Chrony's four-week default is not adopted; original certificate `notAfter` alone is not an association expiry rule. §§7.4/18.7.2; R-05, CHECK-8/9/15. |
| 5. Incorrect intact floor | Valid semantic recovery gap. Define the floor's uncertainty/evidence assumptions and narrow authorized supersession, retaining history and unrelated budgets/ownership. Integrity is not truth. Exact recovery authority/policy remains held. §§8.1/12.1; R-02, CHECK-9/15. |
| 6. Quorum/freshness | Valid precision gap in existing majority/minimum rules. Freeze configured/eligible/independent populations, timeouts, observation age and routine/exceptional authority. Historical prediction is not fresh corroboration. Two malicious agreeing sources plus two blocked honest ones requires a declared fault model; it does not refute a one-fault guarantee. §7.2; R-02, CHECK-7/8/14. |
| 7. Queued SCM restart | Already explicitly covered in the service guide and maintenance plan. Retain disable/quiescence and guarded configuration restoration; make durable maintenance startup inhibition explicit in canonical lifecycle text. Stopped status alone is insufficient. §11.2; CHECK-13/23. |
| 8. Mutation denominator | Valid gate/reporting gap. Reconcile critical functions/guards with generated and absent mutants plus alternative evidence. A nonempty score can omit unsafe/generated paths. Preserve 95%, distinguish absent code from equivalent missed mutants and retain scoped pure-safe-core receipts. §18.4; hardening §9.4, CHECK-19/25. |
| 9. Broken-clock signing | Valid refinement of existing extreme-date/revocation/project-identity requirements. Exercise the full lifecycle, actual verifier/time/retrieval policy, finite work and refusal/recovery. Timestamping does not settle every trust check; `gpgv` success supplies no key expiry/revocation policy; Foundation publisher text alone supplies no project identity. §19.5; signing guide §5.1, CHECK-9/18/23. |

## Additional caveats and limits

- **Correction budget:** 100 ppm over 86,400 seconds is 8.64 seconds. This chooses
  no allowance. OPEN-5 must reconcile measured legitimate drift with residual
  compromised-engine authority and recovery availability.
- **v3 prerequisite:** already covered by OD-07, hardening §6.7 and the CPU skill.
  The compatible checker cannot inherit v3 assumptions or compile-fold its own
  detection. VM feature masking and runnable repair/restoration remain required
  qualification. No compiler configuration changed in this review.
- **Fast Startup:** valid added epoch case. Kernel restoration means user-visible
  shutdown does not prove a new budget epoch. Qualify the selected identity across
  cold boot, restart, sleep, hibernation and VM restore.
- **Bundle mismatch:** the external default-branch observation does not describe
  this checkout: OD-07 §11 and hardening §6.7 exist at `12db217`. Freeze commit
  plus relevant dirty bytes at handoff. There is no root `docs/SHA256SUMS.txt`;
  no bundle-integrity pass is claimed. Archived copies remain historical.

No new quorum, TTL, trust relaxation, reset privilege or wider correction envelope
is selected here. OPEN-5/6/8/11/14 and NV-DEC-001 retain their owners. Close the
first six affected contracts before freezing their production interfaces;
independent bounded P0 and current pure fixture work can continue.

## Evidence and verification

Primary sources are in architecture Appendix A, S114–S120: Microsoft clock,
ownership, SCM and signing contracts; RFC 8915/8446; Chrony 4.9 and pinned source;
cargo-mutants 27.1.0; GnuPG and SignPath. The assessment's Old New Thing link
returned HTTP 403; accessible Microsoft documentation supports the owner-rights
conclusion. Three researchers examined complementary Windows, trust and mutation
questions using inherited context; agreement is not independent experimental
evidence. A separate fresh-context review checks the integrated amendment.

Checks concern links/anchors, canonical/diagram consistency, skill validity and
preserved identifiers/statuses. Candidate hashes and actual validation/review
results are retained in
`~/.local/share/nevrotick/architecture-review-20261030-5j6ykbis/`.
Executed document checks passed: **377 local links/anchors**, five identical
Mermaid pairs, all **110 architecture identifiers** and **345 ASVS identifiers**
preserved, all ASVS table rows unchanged, **61 skills** validated and scoped
whitespace clean. These results apply only to the amended documentation/skill
candidate; final hashes and the separate reviewer receipt identify its bytes.

No application build, mutation campaign, native object/SCM/clock experiment,
installer, trust-store change, signing or publication was performed for this
assessment. Those gates remain open.
