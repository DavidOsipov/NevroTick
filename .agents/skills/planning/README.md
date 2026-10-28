# Planning skills

Use existing request/authorization. Start [plan-code-change](plan-code-change/SKILL.md) for uncertain impact; small understood edits need only concise change/files/check/unknowns/next-action record.

| Question | Owner |
| --- | --- |
| What bounded stage can be implemented next? | [plan-code-change](plan-code-change/SKILL.md) |
| Who owns a rule, state, interface or effect? | [plan-architecture](plan-architecture/SKILL.md) |
| What attacker path or failure changes protection? | [plan-threat-model](plan-threat-model/SKILL.md) |
| Is the named plan stage ready? | [plan-independent-review](plan-independent-review/SKILL.md) |

Read `docs/README.md` and relevant canonical architecture first. Plans prove neither implementation, successful Windows execution nor standards conformance. Suggested crate names remain ownership proposals until source exists. Reuse architecture `REQ/BND/DEC/THR/CHECK` IDs; distinguish new local IDs; preserve canonical decisions.

Keep one plan/evidence chain. For substantial work, use [the plan template](templates/change-plan.md) in existing user-selected location, or `.agents/plans/YYYY-MM-DD-<subject>.md` if absent. Default: keep working plans outside checksummed `docs/` bundle. Read [SOURCES.md](SOURCES.md) for adaptation provenance/maintenance.
