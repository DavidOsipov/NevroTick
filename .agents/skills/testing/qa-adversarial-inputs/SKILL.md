---
name: "qa-adversarial-inputs"
description: "Design bounded hostile NevroTick input cases that challenge the actual protocol, IPC, persistence and resource admission path."
---

# Challenge hostile input admission

Read [the testing contract](../CONTRACT.md).

Start with a small accepted scaffold and independently known result. Mutate a named grammar, identity, length, state or authority rule while preserving the bytes needed to reach the intended seam. Keep valid-shaped unauthorized cases separate from malformed syntax.

Target duplicate/escaped keys, array substitution, long numeric/escaped tokens, truncation, nested shape/depth, integer overflow, hostile display/error names, binary extension length arithmetic, packet replay, fake endpoints and request storms. Keep parser/crypto and contextual effects separate; syntactic validity is no authority proof.

Observe bounded allocation/work/queues/logging and forbidden effects, not just no-crash survival. Reset owned state between cases and retain exact original/minimized bytes plus failure mechanism. Preserve the healthy neighbor so rejecting everything cannot pass.

Use [numerical/protocol truth](../references/numerical-protocol.md) and optional [Rust techniques](../references/rust-techniques.md). Exploratory tools and stronger native effects require their actual prerequisites, not automatic installation or live-host mutation.
