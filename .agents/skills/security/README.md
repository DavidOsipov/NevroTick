# Native security implementation skills

Read [CONTRACT.md](CONTRACT.md) for security work. Select changed input-to-effect path; native controls replace website mechanisms rather than inherit names.

| Surface | Owner |
| --- | --- |
| Service identity/token, privilege, process access, DLL loading, clock ownership | [security-windows-privileges](security-windows-privileges/SKILL.md) |
| UDP/NTP/NTS bytes, DNS/destination policy, configuration and protected files | [security-protocol-inputs](security-protocol-inputs/SKILL.md) |
| TLS/AEAD/RNG, NTS associations, protected secrets, wrong-date recovery | [security-crypto-secrets](security-crypto-secrets/SKILL.md) |
| Named-pipe identity, frame admission, operation rights and stale requests | [security-local-ipc](security-local-ipc/SKILL.md) |
| Normal-user native UI, displayed trust, narrow elevated requests and exports | [security-native-ui](security-native-ui/SKILL.md) |
| Redaction, local events, retention, diagnostics and telemetry choices | [security-privacy-logging](security-privacy-logging/SKILL.md) |
| Cargo/build trust, final binaries, signing, complete MSI lifecycle | [security-build-deployment](security-build-deployment/SKILL.md) |

[plan-threat-model](../planning/plan-threat-model/SKILL.md) owns changed threat decisions; [qa-security-boundaries](../testing/qa-security-boundaries/SKILL.md) owns verification; [dependency-updates](../environment/dependency-updates/SKILL.md) owns updates. Load [native proof requirements](references/native-proof.md) only for Windows enforcement claims; [persistence/resource guidance](references/persistence-and-resources.md) for those boundaries. [SOURCES.md](SOURCES.md) retains research/provenance and full source mapping.
