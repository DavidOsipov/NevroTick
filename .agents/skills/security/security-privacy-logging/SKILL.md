---
name: "security-privacy-logging"
description: "Implement or review NevroTick redacted events, diagnostic snapshots and exports, local retention, secret-safe tracing and optional telemetry decisions."
---

# Log useful facts without copying secret input

Read [the security contract](../CONTRACT.md) and architecture section 12. Inspect actual writers, subscribers, event sinks, export adapters and error chains; unused redaction helpers prove nothing.

For error-to-output mappings, dependency causes or panic/cleanup reports, apply
[error handling and safe diagnostics](../../implementation/implement-rust/references/error-handling.md).
The shared diagnostic policy must constrain typed fields before formatting and
buffering; a central string scrubber cannot establish privacy. Keep native
capture in the platform adapter and log the outcome once at its decision owner.

For Windows Event Log providers/channels or optional ETW, read relevant [Windows host integration](../references/windows-host-integration.md#event-log) sections: installer-owned resources, effective access, wrong-clock event ordering, runtime lifecycle and native evidence.

1. State each field's purpose, sensitivity, owner, recipients and retention. Exclude keys, NTS cookies, approval tokens, raw secret-bearing packets, unnecessary user identifiers and private paths from logs/exports. Prefer bounded codes/counters plus selected context; never serialize whole wire/domain objects on rejection.
2. Inspect derived `Debug`, error-source chains, panic reports, tracing macros and enabled features. `tracing::instrument` records arguments by default; apply explicit field policy/skip controls at real owner. Redacted `Debug` cannot protect separate explicit fields, formatted inner values or downstream sinks.
3. Bound record/field size, aggregate queues, disk retention, rotation and repeated-denial rates. Preserve first relevant cause/counters without flood-driven resource exhaustion. Use correlation/monotonic sequence for event ordering: repaired wall time may move backward; timestamps alone prove no freshness/causality.
4. Separate security events, readable status and diagnostic exports. Services publish bounded redacted snapshots; UI owns user-selected export. Observe permissions and handle/path safety at actual sinks. Export convenience grants no arbitrary destination paths to privileged services.
5. Logging failure must not turn denial into success, block necessary safe clock cleanup or silently corrupt state. Define bounded backpressure/loss/fallback per affected event requirement. Crash dumps/OS traces may need separate collection/access policy; ordinary redaction proves no content safety there.
6. Keep telemetry off unless explicitly adopted. Local logging alone does not prove ASVS's separate analysis/detection/alerting requirements; assess actual Windows event/managed forwarding boundary. Retain desktop-profile gaps without silently sending data.

Send synthetic recognizable secrets through real success/error/export paths; inspect all selected outputs with useful legitimate diagnostic control. Exercise disk/queue failures/rate bounds only for changed claims. Report fields/sinks/retention, observed exclusions, lost-event behavior and privacy/analysis limits. Never publish, email or upload diagnostics without authorization.

For uncertain secret propagation, use
[CodeQL data-flow analysis](../../testing/references/codeql-data-flow.md) from
key/cookie/token inputs through clones, formatting, error conversion and event
fields to every selected log/export sink. Treat redaction as a barrier only for
the exact output it protects; inspect tracing macros and unmodeled native sinks
directly. Confirm findings with synthetic-secret output observations; empty
queries cannot establish absence of disclosure.
