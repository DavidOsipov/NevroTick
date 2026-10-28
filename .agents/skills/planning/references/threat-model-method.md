# Threat method

Use the changed flow and canonical threat model as the starting point. Describe a concrete path from attacker-controlled input to a protected effect, including required identity, existing access, timing and shared-state assumptions. Separate an unauthenticated network attacker, a compromised engine, an ordinary local user, a delegated operator and an administrator; their powers and security goals differ.

Test a competing explanation when it would change a finding: is the path reachable by that identity, does a previous guard reject it, is the claim only a proposed design, can the same effect already be exercised by authorized rights, or does the test fail before product behavior? Label design concern, verified implementation defect, missing evidence and residual limitation distinctly.

Useful native paths include packet replay/delay/source correlation, wrong-date TLS recovery, DNS/negotiated-destination changes, pipe squatting/server restart, schema/constructor bypass, stale policy/generation, revoked product grants, correction-budget laundering, ownership loss, filesystem replacement/reparse races, service token/injection access, MSI repair properties and untrusted build execution.

Record existing control and actual owner, bypass assumptions, expected rejection, legitimate near-neighbor, forbidden side effect and cleanup. Authentication establishes origin/integrity under its trust basis; it cannot prove source independence or eliminate delay attacks. A private product mutex cannot constrain another administrator or VM host. Document those residuals rather than proposing controls that overclaim OS guarantees.

Keep the ASVS pinned denominator and exact affected clauses. Unknown is not N/A, and an adapted Windows mechanism is not automatically literal compliance. Source inspection supports a source claim; planned tests support no executed-behavior claim.
