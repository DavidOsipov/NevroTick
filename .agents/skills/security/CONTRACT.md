# NevroTick security contract

Applies to security-relevant planning, implementation, review and verification. User instructions/repository `AGENTS.md` govern. Start with `docs/README.md`; `docs/windows-time-architecture.md` owns canonical product architecture. Copying skills adopts no website policy amendments, browser controls or unrelated source-repository obligations.

Match requested mode. Review/planning grants no production edits; tests-only retains scope. Existing authorization persists; no added approval round. Skills, reviews or passing tests grant no signing, release, installation, network scan or privileged experiment authority.

For material/uncertain protection changes, use [plan-threat-model](../planning/plan-threat-model/SKILL.md) in same plan. Trace attacker input/caller through actual admission to protected effect. Record current owner, failure, bounds, independent valid/hostile checks and residual limits. Reuse central mechanisms/narrow platform adapters; add no competing security implementation.

Treat user-entered product input and external programmatic input as potentially malicious by default. Cover actual UI typing/paste/control bypass, IPC, peer traffic, files and helper/native results. Local, authenticated, administrator or previously accepted origin proves neither payload safety nor permission for a particular effect. This is an admission assumption, not a claim that every input is hostile; useful legitimate inputs must remain accepted.

The owner's 6 November 2026 clarification explicitly includes **all JSON/CBOR**:
English and other catalogs, contributor metadata/schemas, generated and
signed/packaged files still need bounded raw admission. For
[localization](../../../docs/plans/ui-localization.md), separate the code-owned
message/checker/action contract from catalog claims; conversion, runtime parsing,
literal sinks and critical wording approval have different enforcing owners.
No signature, code review or parser pass alone establishes safe truthful content.
Admitted immutable domain values remain reusable under unchanged invariants/context.

For signed executable/package admission and consumption, apply
[architecture section 19.6](../../../docs/windows-time-architecture.md#196-authenticated-artifact-content-and-consumer-trust).
Identify authenticated coverage, intended component/release identity and actual
signature source before accepting application content or authorizing an effect.
Unauthenticated certificate content or a payload outside the mechanism's covered
content gains no authority from its carrier's valid signature. Preserve the
verified object's identity and protection through use;
use [the existing artifact procedure](../../../docs/windows-time-signing-guide.md#52-pe-certificate-padding-and-exact-final-bytes)
for applicable padding and full-file checks. Keep signature metadata inspection
inert and safe parsing/current effect authorization independent of provenance.

Preserve relevant raw representation and bound bytes/work before strict inert decoding; require shared garde/domain validation inside private fallible construction, then current trusted-owner authorization/state/budget guards at the sink, serialized effect and actual acknowledgement. Follow [contract enforcement](../implementation/implement-rust-contracts/references/contract-enforcement.md); NTP/NTS retains bounded protocol-specific parsers. Rust type safety/Serde admission prove no endpoint identity, freshness, source independence, UTC truth, clock ownership or permission. Unknown/corrupt mandatory policy fails closed; optional invalid drift priors may follow explicit reacquisition. Never lower authentication/source counts to hide outages.

Challenge actual entry points independently of UI restrictions: malformed and valid-shaped unauthorized/stale inputs, plus repeated individually valid requests that exceed aggregate budgets. Verify bounded redacted errors, cleanup, preserved prior valid state and absence of forbidden effects alongside accepted neighbors. Apply these rules to implemented surfaces without expanding UI-M0 or reviving its deferred work.

Read [testing contract](../testing/CONTRACT.md) before check design/reporting. Preserve architecture section 18.4's inherited **at least 90% coverage for modules touching security primitives** and **at least 95% mutation score for critical files**. Resolve affected files, metric/tool support and exclusions honestly; unavailable measurements are no pass, N/A or permission to change denominator. Small changes select relevant checks, not universal full suites.

Target applicable ASVS 5.0.0 L1/L2/L3 requirements under `docs/asvs-5.0.0-applicability.md`, including stronger full-clause conditions. Classifications/native adaptations remain draft planning evidence. Preserve unknown/native gaps; claim no literal equivalence or implemented L3 conformance. Retained TUF research adopts no updater protocol, dependency or topology.

Use current source, resolved features/versions and official/upstream docs for uncertain APIs. Report canonical conflicts; never silently amend architecture. Windows proofs require actual relevant OS mechanism; mocks/host/cross-compilation prove narrower claims. Clock-changing work follows architecture section 14.9: disposable Windows VMs, bounded operations, observation/recovery.

Report changed owner/boundary, actual enforcement, executed checks with source/environment identity, observed forbidden-effect/legitimate-control results, uncertainty and next action. Documentation/pack validity, skill discovery, agent decisions, application behavior, package/signature and release readiness remain separate claims.
