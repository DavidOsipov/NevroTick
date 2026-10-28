---
name: "security-native-ui"
description: "Implement or review NevroTick normal-user native UI trust presentation, bounded service commands, narrow elevated helper requests, safe text/actions and diagnostic exports."
---

# Keep presentation and privileged authority separate

Read [the security contract](../CONTRACT.md), architecture sections 4.2, 5, 8.3, 12 and 16. Use [implement-native-ui](../../implementation/implement-native-ui/SKILL.md) for native control/accessibility implementation.

1. Trace displayed/entered values through IPC/effects. Normal UI stays `asInvoker`; closing/hiding leaves background synchronization running. Enabled controls, signed UI binaries or client-side validation grant no service authority. Receiving trusted service validates/authorizes every protected request.
2. Prefer native semantic controls and bounded literal text. Separate untrusted server names/errors/source labels from format strings, shell arguments, command identifiers and filesystem/URI effects. Reuse typed action/target policy; construct no shell commands. Architecture selects native Win32 foundations; this port introduces no WebView, HTML sanitizer or browser policy.
3. Distinguish authentication, certificate-time basis, agreement, freshness, uncertainty and current correction state. Service liveness alone proves no synchronized time. Bootstrap text states unauthenticated date estimate is checking secure servers; system clock has not changed. Never label two hints secure quorum or NTS absolute accuracy guarantee.
4. `Sync now` requests earlier measurement under existing policy; no caller-supplied clock value/server/address. Machine-source, bounds or bootstrap-policy changes use narrow elevated helper; helper rereads authoritative state/expected revision. No user-writable command file, arbitrary path/service/registry/script or general execution API. UAC cancellation preserves policy.
5. Bind exceptional corrections to exact current proposal/evidence/identity/revision/generation and single-use expiry. Present denied/stale/outcome-unknown states honestly; never retry non-idempotent steps to hide lost acknowledgements. UI preferences cannot change machine trust policy.
6. UI generates diagnostic exports from bounded redacted service snapshots/pages. Service endpoints receive no arbitrary output path. Review clipboard, links, drag/drop, file dialogs and error details only where features exist; require no speculative surfaces.

Use real native UI/service paths for changed claims: allowed/denied identities, UAC cancellation, stale snapshots, keyboard/assistive-technology behavior and service continuity after UI close. UI Automation/accessibility support requires no broad UIAccess or elevation. Report actual actions/sinks, authority owner, visible trust behavior and evidence limits.
