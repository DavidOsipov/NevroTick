# Threat method

Start from changed flow/canonical threat model. Describe concrete attacker-controlled input → protected effect path: required identity, existing access, timing, shared-state assumptions. Distinguish unauthenticated network attacker, compromised engine, ordinary local user, delegated operator, administrator; powers/security goals differ.

Test finding-changing competing explanations: identity can reach path? Previous guard rejects? Proposed design only? Authorized rights already permit same effect? Test fails before product behavior? Distinguish design concern, verified implementation defect, missing evidence, residual limitation.

Useful native paths include packet replay/delay/source correlation, wrong-date TLS recovery, DNS/negotiated-destination changes, pipe squatting/server restart, schema/constructor bypass, stale policy/generation, revoked product grants, correction-budget laundering, ownership loss, filesystem replacement/reparse races, service token/injection access, MSI repair properties and untrusted build execution.

Record existing control/actual owner, bypass assumptions, expected rejection, legitimate near-neighbor, forbidden side effect, cleanup. Authentication establishes origin/integrity under its trust basis; proves no source independence, eliminates no delay attacks. Private product mutex cannot constrain another administrator/VM host. Document residuals; do not overclaim OS guarantees.

Keep ASVS pinned denominator/exact affected clauses. Unknown is not N/A; adapted Windows mechanism does not automatically mean literal compliance. Source inspection supports source claim; planned tests support no executed-behavior claim.
