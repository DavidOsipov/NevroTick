# Version, persistence and lifecycle transitions

Record each required old/new writer, reader and stored-version combination as
supported, deliberately unsupported or unverified, with evidence. Include permitted
UI/broker/engine/helper and MSI maintenance combinations. Complete packages do not
prove simultaneous process replacement. Architecture sections 11, 12 and 19 own
lifecycle/protected restoration.

Identify canonical writer, version decoder, constructor, authorization context,
publication point and consumers per affected record. Preserve units/sign/exact
integers; JSON serialization/parsing must introduce no floating intermediate.
Test normalization collisions, discriminator changes, absent/null/defaults and
deliberate projection loss against independent fixtures.

Define transition before new writes:

- Reader/writer order, supported window, explicit version negotiation/rejection,
  old-process behavior during replacement.
- Owner-protected staging/atomic replacement, prepublication checks,
  interruption/startup reconciliation for partial/malformed state.
- Downgrade readability; whether rollback can restore consumed trust, budgets or
  one-use authority.
- Separate retirement evidence for old writers, readers, data and migration code.

Migrations validate source version/provenance, construct validated new values and
preserve required audit evidence. Never grant rights from defaults, reset
replay/cumulative budgets on read failure or reinterpret old timestamps as fresh.
Optional cache invalidation differs from mandatory policy/journal recovery.
Roundtrips prove no semantic compatibility if converters share mistakes.

Name protected persistence phases: preparation/validation, authorized staging,
checked write/flush/close, publication, acknowledgement/model refresh, recovery
cleanup. Compare failures with relevant phase snapshot; promise no universal
pre-call rollback. Reconcile disk/service model before retry when replacement
preceded acknowledgement failure. Atomic replacement differs from power-loss
durability; flushing alone proves no durable rename/metadata on every filesystem.
Use valid neighboring state/fault-hit observations to reach each publication boundary;
early schema/path rejection cannot test late recovery.

Rehearse interrupted updates/rollback and mixed generations in proper isolated
Windows lane before lifecycle qualification. Automatic TUF updating remains
unadopted research; routine schema migration must add no updater state/authority.
Future updater needs separate design decision and threat-model/applicability update.
