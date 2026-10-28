# Protected persistence and aggregate work

Use architecture sections 6, 10–12 and actual operation guarantee. Name canonical writer, source/installed/policy/cache/journal class, authorized fixed path/handle, schema/version, ACL, current revision/generation and recovery owner. UI preferences, optional drift priors and mandatory broker security journals need distinct failure policies.

Separate admission, candidate preparation, validation, staging write/sync, publication, durable state and acknowledgement. Error returns alone prove no rollback boundary. Reach intended late stage after valid no-fault control. Observe original/published bytes, authoritative revision, journal progress, owned staging artifacts and next allowed action. Publication may precede acknowledgement failure; reconcile authoritative state before retry/unchanged claims. Generic rename/`fsync` alone proves no required Windows filesystem/power-loss guarantee.

Check relevant traversal, Windows drive/share/device forms, reparse points, replacement races, inherited rights and open-handle sharing. Confine fixtures to owned temporary paths. OS ACL/path proofs need native execution; string allowlists prove no hardened filesystem sandbox. Never log sensitive paths to prove rejection.

Bounds need owners/aggregate accounting: frames/fields, clients/in-flight tasks, queued/completed work, retained histories/snapshots/caches, nested crypto/parsers, diagnostics and task replacements. Checked arithmetic/fixed-capacity containers alone cannot bound total process memory/work. Distinguish byte/character/element counts; track sleeping, abandoned/cancelled work still live.

Cancellation requested, result invalidated, operation completed, task exited and resources released remain distinct observations. Dropped handles are no joins; unwinding cannot catch every abort/OOM or undo external effects. Keep hostile/resource cases within owned process/host limits; record peak/cumulative work for those claims. Restore owned fault dependencies and perform bounded recovery only; successful retries erase neither first failure nor cleanup cause.

Use exact raw bytes at challenged parser seam. Generic JSON maps collapse duplicate keys; replacement decoding hides malformed UTF-8. Pair hostile cases with valid neighbors; inspect forbidden state/effects beyond error wording. Selected supported Miri/sanitizers observe executed paths only; they prove no native OS behavior, universal FFI soundness or aggregate hostile-input safety.
