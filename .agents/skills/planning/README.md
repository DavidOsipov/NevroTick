# Planning skills

Use the existing request and authorization. Start with [plan-code-change](plan-code-change/SKILL.md) for work with uncertain impact; a small understood edit needs only its concise change/files/check/unknowns/next-action record.

| Question | Owner |
| --- | --- |
| What bounded stage can be implemented next? | [plan-code-change](plan-code-change/SKILL.md) |
| Who owns a rule, state, interface or effect? | [plan-architecture](plan-architecture/SKILL.md) |
| What attacker path or failure changes protection? | [plan-threat-model](plan-threat-model/SKILL.md) |
| Is the named plan stage ready? | [plan-independent-review](plan-independent-review/SKILL.md) |

Read `docs/README.md` and the relevant canonical architecture sections first. Plans do not establish implementation, successful Windows execution or standards conformance. The current suggested crate names are ownership proposals until source exists. Reuse architecture `REQ/BND/DEC/THR/CHECK` IDs; distinguish new local IDs instead of overwriting canonical decisions.

Keep one plan and one evidence chain. Substantial work can use [the plan template](templates/change-plan.md) in an existing user-selected location, or `.agents/plans/YYYY-MM-DD-<subject>.md` if none exists. Do not put working plans into the checksummed `docs/` bundle by default. Read [SOURCES.md](SOURCES.md) for adaptation provenance and maintenance.
