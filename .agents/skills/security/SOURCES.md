# Security adaptation provenance

Checked 28 October 2026. `docs/windows-time-architecture.md` and explicitly unimplemented verification plans retain authority. Read website working-tree sources without edits. Copied no browser constitution/amendments, implementation paths, test scripts or evidence archives as NevroTick policy.

| Website source under `.agents/skills/security/` | NevroTick target | Native adaptation |
| --- | --- | --- |
| `security-browser-policy` | `security-windows-privileges` | Effective Windows process/service permissions, protected objects, clock ownership and code-loading policy replace browser-enforced policy. |
| `security-build-deployment` | `security-build-deployment` | Cargo host/build trust, emitted Windows binaries, signing and complete MSI lifecycle replace edge/site publication. |
| `security-content-assets` | `security-protocol-inputs` | Raw network/protocol/configuration admission, resource bounds and protected persistence replace content compilation/assets. |
| `security-crypto-secrets` | `security-crypto-secrets` | TLS/NTS provider/identity, association state, wrong-date recovery, OS RNG and service-scoped protected secrets. |
| `security-dom-sanitization` | `security-native-ui` | Literal native presentation, typed allowed actions, normal-user UI and narrow elevated helper replace DOM/sanitizer sinks. |
| `security-edge-api` | `security-local-ipc` | Mutually authenticated named pipes, operation rights, strict frames and current broker guards replace HTTP handlers. |
| `security-privacy-logging` | `security-privacy-logging` | Local native event/tracing/export/retention paths and explicit desktop analysis gaps replace website telemetry/consent assumptions. |

Retains scoped modes, canonical owner, threat/control/check handoff, legitimate/hostile controls, evidence limits and inherited coverage/mutation targets under canonical architecture. Selective references replace duplicated website trees/helpers.

| Primary source inspected | Finding used | Qualification limit |
| --- | --- | --- |
| [Named pipe security/access rights](https://learn.microsoft.com/en-us/windows/win32/ipc/named-pipe-security-and-access-rights) | Explicit descriptors, precise rights, generic-write/instance-creation issue. | Native DACL and peer/operation proofs remain required. |
| [Required service privileges](https://learn.microsoft.com/en-us/windows/win32/api/winsvc/ns-winsvc-service_required_privileges_infow) and [service security rights](https://learn.microsoft.com/en-us/windows/win32/services/service-security-and-access-rights) | Inspect effective startup privileges and object-specific SCM/service rights. | Does not prove proposed accounts meet isolation. |
| [SetSystemTime](https://learn.microsoft.com/en-us/windows/win32/api/sysinfoapi/nf-sysinfoapi-setsystemtime) | The API can enable a present change-system-time privilege. | Disabled-in-token is not absent; new adapter needs native proof. |
| [Dynamic-library security](https://learn.microsoft.com/en-us/windows/win32/dlls/dynamic-link-library-security) | Fully qualified protected paths and reviewed load-search policy reduce planting opportunities. | Final imports/loads and mitigations need observation. |
| [MSI rollback](https://learn.microsoft.com/en-us/windows/win32/msi/rollback-installation) | Installer rollback exists, but custom actions require additional rollback authoring. | Whole product/W32Time restoration is not automatic. |
| [RFC 8915](https://www.rfc-editor.org/rfc/rfc8915.html) | TLS key establishment, authenticated NTP, certificate-time bootstrapping and delay/downgrade limits. | NevroTick's stricter selected bootstrap flow remains canonical. |
| [CryptProtectData](https://learn.microsoft.com/en-us/windows/win32/api/dpapi/nf-dpapi-cryptprotectdata) | Machine-scoped data can be decrypted by other machine users with blob access, so ACLs still matter. | Service restart/account/migration behavior needs qualification. |
| [tracing instrument](https://docs.rs/tracing/latest/tracing/attr.instrument.html) | Arguments are recorded by default; explicit field/skip policy is necessary. | No dependency/version choice or actual sink proof is implied. |
| [UI Automation security](https://learn.microsoft.com/en-us/windows/win32/winauto/uiauto-securityoverview) | Cross-integrity automation and UIAccess have distinct security behavior. | Accessibility does not justify broadly elevating the product UI. |
| [GitHub Actions secure use](https://docs.github.com/en/actions/reference/security/secure-use) and [Cargo build scripts](https://doc.rust-lang.org/cargo/reference/build-scripts.html) | Minimize workflow authority, isolate untrusted code and inspect host build execution. | No workflow/security provider has been installed or qualified here. |
| [ntpd-rs protocol entrypoint](https://github.com/pendulum-project/ntpd-rs/blob/main/ntp-proto/src/lib.rs) | Protocol/algorithm crate forbids unsafe and warns its API is not stable/public. | Upstream reference only, not a selected dependency or Windows backend. |

Also inspected the user's prior port at [PdfCraft PR 2912](https://github.com/storytold/pdfcraft/pull/2912), using pinned head `c9a02b378271f92a73cb44be9fd9e4acc6921516`: [architecture/data flows](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-architecture-data-flows/references/architecture.md), [transactions](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-safe-filesystem-persistence/references/transactions.md), and [security/resources](https://github.com/DavidOsipov/pdfcraft/blob/c9a02b378271f92a73cb44be9fd9e4acc6921516/.agents/skills/QA/pdfcraft-safe-filesystem-persistence/references/security-resources.md). Adapted phase-specific publication/acknowledgement observations, exact raw inputs, aggregate budgets and cancellation/completion/release distinctions. No PDF semantics, filesystem-root policy, observed product defects, code or tool dependencies were imported. That closed unmerged proposal is guidance provenance, not an adopted NevroTick architecture or upstream endorsement.

Consulted local ASVS applicability profile (retains pinned full clauses, draft dispositions, attribution/open gaps) and explicitly unadopted TUF assessment. Pack reproduces no standard or compliance/update implementation from research.
