# Version, persistence and lifecycle transitions

Record each required combination of old/new writer, old/new reader and stored
version as supported, deliberately unsupported or unverified, with its evidence.
Include UI/broker/engine/helper and MSI maintenance combinations actually allowed
by the packaging design. A single complete package does not prove process
replacement is simultaneous. Architecture sections 11, 12 and 19 own lifecycle
and protected restoration.

For each affected record identify the canonical writer, version decoder,
constructor, authorization context, publication point and consumers. Preserve
explicit unit/sign semantics and exact integers; JSON number serialization and
parsing must not introduce a floating intermediate. Test normalization collisions,
discriminator changes, absent/null/default distinctions and deliberate projection
loss with independent fixtures.

Define the transition before new writes begin:

- Reader/writer order and supported window; explicit version negotiation or
  rejection; old process behavior during replacement.
- Protected staging and atomic replacement at the owner; checks before publishing;
  interruption and startup reconciliation for partial/malformed state.
- Whether downgrade can read the new representation and whether rollback may
  restore trust, budgets or one-use authority that has already been consumed.
- Retirement evidence for old writers, readers, data and migration code separately.

Migrations validate source version and provenance, construct a new validated value
and preserve required audit evidence. Never grant new rights from a data default,
reset replay/cumulative budgets on read failure, or reinterpret an old timestamp
as a fresh observation. Optional cache invalidation is distinct from mandatory
policy/journal recovery. Round-trip equality alone cannot establish old/new semantic
compatibility if both converters share the same mistake.

For protected persistence name the phases explicitly: preparation/validation,
authorized staging, checked write/flush/close, publication, acknowledgement/model
refresh and recovery cleanup. Compare failures with the relevant phase snapshot,
not an invented promise that every error restores the pre-call state. If bytes
were replaced but acknowledgement failed, reconcile the disk/service model before
retry. Atomic replacement and power-loss durability are distinct claims; a file
flush alone does not prove durable rename/metadata under every filesystem.
Use valid neighboring state and observable fault-hit evidence to reach each
publication boundary; an earlier schema/path rejection cannot test late recovery.

Rehearse interrupted update/rollback and mixed generations in the proper isolated
Windows lane before claiming lifecycle qualification. Automatic TUF updating is
retained research, not adopted architecture; do not add its state or authority as
part of a routine schema migration. Any future updater requires its own design
decision and threat-model/applicability update.
