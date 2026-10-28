---
name: "security-privacy-logging"
description: "Implement or review NevroTick redacted events, diagnostic snapshots and exports, local retention, secret-safe tracing and optional telemetry decisions."
---

# Log useful facts without copying secret input

Read [the security contract](../CONTRACT.md) and architecture section 12. Inspect actual writers, subscribers, event sinks, export adapters and error chains; a redacting helper unused by the real path proves nothing.

For Windows Event Log providers/channels or optional ETW, read the relevant [Windows host integration](../references/windows-host-integration.md#event-log) sections for installer-owned resources, effective access, wrong-clock event ordering, runtime lifecycle and native evidence.

1. State each field's purpose, sensitivity, owner, recipients and retention. Keep keys, NTS cookies, approval tokens, raw secret-bearing packets, unnecessary user identifiers and private paths out of logs/exports. Prefer bounded codes/counters plus selected context; do not serialize entire wire/domain objects on rejection.
2. Inspect derived `Debug`, error-source chains, panic reports, tracing macros and enabled features. `tracing::instrument` records arguments by default; use explicit field policy/skip controls at the real owner. Redacted `Debug` does not protect a separate explicit field, formatted inner value or downstream sink.
3. Bound record/field size and aggregate queues, disk retention, rotation and repeated-denial rates. Preserve first relevant cause and counters without allowing a packet flood to exhaust resources. Use a correlation/monotonic sequence for event ordering because repaired wall time can move backward; timestamps do not alone establish freshness or causality.
4. Separate security events, user-readable status and diagnostic exports. Services publish bounded redacted snapshots; UI owns a user-selected export. Observe permissions and handle/path safety at actual sinks. Do not grant a privileged service arbitrary destination paths to simplify export.
5. Logging failure must not turn denial into success, block necessary safe clock cleanup or silently corrupt state. Define backpressure/loss behavior and bounded fallback according to the affected event's requirement. A crash dump or OS trace may have separate collection/access policy; ordinary redaction does not prove its contents safe.
6. Keep telemetry off unless explicitly adopted. Local logging alone is not ASVS's separate analysis/detection/alerting proof; evaluate the actual Windows event/managed forwarding boundary and retain any desktop-profile gap without silently sending data.

Use synthetic recognizable secrets through real success/error/export paths and inspect all selected outputs, with a legitimate useful diagnostic control. Exercise disk/queue failure and rate bounds only for changed claims. Return fields/sinks/retention, observed exclusions, lost-event behavior and remaining privacy/analysis limits. Do not publish, email or upload diagnostics without authorization.
