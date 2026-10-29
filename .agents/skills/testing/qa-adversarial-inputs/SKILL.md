---
name: "qa-adversarial-inputs"
description: "Design bounded hostile NevroTick input cases that challenge the actual protocol, IPC, persistence and resource admission path."
---

# Challenge hostile input admission

Read [the testing contract](../CONTRACT.md).

Treat user-entered product input and external programmatic input as potentially malicious by default, including local/authenticated/administrator and previously accepted data. Select actual typing/paste, direct UI-handler bypass, IPC, peer, file and helper/native-result seams as applicable; origin alone proves neither payload safety nor current authority. Preserve UI-M0 deferrals and do not add surfaces to exercise this policy.

Start with small accepted scaffold/independently known result. Mutate named grammar/identity/length/state/authority rule; preserve bytes reaching intended seam. Separate malformed syntax from valid-shaped unauthorized, stale or resource-abusive cases. Trace bounded decoding, mandatory shared garde/domain construction and current trusted-owner guards through to the sink; challenge the handler independently of UI-only validation.

Target duplicate/escaped keys, array substitution, long numeric/escaped tokens, truncation, nested shape/depth, integer overflow, hostile display/error names, binary extension length arithmetic, packet replay, fake endpoints and request storms. Keep parser/crypto and contextual effects separate; syntactic validity is no authority proof.

Observe bounded allocation/work/queues, redacted error/log output, cleanup, prior valid state and forbidden effects; no-crash survival insufficient. Exercise repeated individually valid requests against aggregate budgets across the relevant clients/tasks. Reset owned state between cases, not within a sequence intended to exhaust a shared budget; retain exact original/minimized bytes/failure mechanism. Preserve healthy neighbor; reject-all must fail.

Use [numerical/protocol truth](../references/numerical-protocol.md), optional [Rust techniques](../references/rust-techniques.md). Exploratory tools/stronger native effects need actual prerequisites; no automatic installation/live-host mutation.
