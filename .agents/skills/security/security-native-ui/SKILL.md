---
name: "security-native-ui"
description: "Implement or review NevroTick normal-user native UI trust presentation, bounded service commands, narrow elevated helper requests, safe text/actions and diagnostic exports."
---

# Keep presentation and privileged authority separate

Read [the security contract](../CONTRACT.md), architecture sections 4.2, 5, 8.3, 12 and 16. Use [implement-native-ui](../../implementation/implement-native-ui/SKILL.md) for native control/accessibility implementation.

1. Trace displayed and entered values through IPC and effects. The normal UI stays `asInvoker`; closing or hiding it leaves background synchronization running. A control's enabled state, signed UI binary or client-side validation never grants service authority. The receiving trusted service validates and authorizes every protected request.
2. Prefer native semantic controls and bounded literal text. Keep untrusted server names/errors/source labels separate from format strings, shell arguments, command identifiers and filesystem/URI effects. Reuse a typed action/target policy rather than constructing shell commands. The architecture selects native Win32 foundations; no WebView, HTML sanitizer or browser policy is introduced by this port.
3. Make authentication, certificate-time basis, agreement, freshness, uncertainty and current correction state distinct. Service liveness alone is not synchronized time. Bootstrap text states that an unauthenticated date estimate is checking secure servers and the system clock has not changed. Do not label two hints secure quorum or NTS an absolute accuracy guarantee.
4. `Sync now` requests earlier measurement under existing policy with no caller-supplied clock value/server/address. Machine-source, bounds or bootstrap-policy changes use the narrow elevated helper; the helper rereads authoritative state and expected revision. No user-writable command file, arbitrary path/service/registry/script or general execution API. UAC cancellation leaves policy unchanged.
5. Bind exceptional corrections to the exact current proposal/evidence/identity/revision/generation and single-use expiry. Present denied/stale/outcome-unknown states honestly; do not retry a non-idempotent step to hide a lost acknowledgement. UI preferences cannot change machine trust policy.
6. Generate diagnostic exports in the UI from bounded redacted service snapshots/pages. Service endpoints receive no arbitrary output path. Review clipboard, links, drag/drop, file dialogs and error details only when those actual features exist; no speculative surfaces are required.

Use real native UI and service paths for changed claims, including allowed/denied identities, UAC cancellation, stale snapshots, keyboard/assistive-technology behavior and service continuity after UI close. UI Automation/accessibility support does not require granting the application broad UIAccess or elevation. Return actual actions/sinks, authority owner, user-visible trust behavior and evidence limits.
