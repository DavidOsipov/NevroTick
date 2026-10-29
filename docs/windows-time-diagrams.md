# Rust Windows Time Client — Architecture diagrams

**Plan:** RWTC-ARCH-001, revision 1.4, 28 October 2026  
**Status:** selected design; no application or Windows runtime has been qualified

These editable Mermaid views are extracted from the [canonical architecture and threat model](windows-time-architecture.md). Keep each diagram in that plan authoritative and regenerate this companion when the architecture changes. GitHub and other Mermaid-capable Markdown renderers display the diagrams directly; the fenced source is also available to edit.

**Distribution contract:** one signed complete MSI per supported architecture contains the native UI, engine service, privileged clock broker, helper and required payloads. One product owns install/update/repair/removal. Runtime process separation restricts authority; it does not require separate packages. Any future MSIX must satisfy the same complete-product contract.

## Reading the views

The runtime view shows where authority changes. The sequence view shows who validates a proposed correction and acknowledges applied state. The validation view traces bounded input through domain construction to current authority and effects. The recovery view keeps provisional TLS time separate from the Windows clock. The lifecycle view explains ownership and rollback. Revision 1.4 preserves all five diagram source blocks and their process/installation boundaries. The full review reconciles conditional native HTTP retrieval, entropy exceptions and verifier date-range qualification in the main plan and worksheet; it does not create a new process or package. See the [full review report](windows-time-review.md). Full invariants, threat scenarios, limitations and test gates remain in the main plan.

## 1. Runtime architecture

```mermaid
flowchart TD
    N["Configured NTP and NTS endpoints"]
    O["Local users and processes"]
    subgraph PRODUCT["One installed product"]
        direction TB
        U["Native UI - normal user"]
        E["Engine service - no clock privilege"]
        B["Clock broker - narrow time privilege"]
        A["Admin helper - elevated on demand"]
        ES[("Engine cache and NTS secrets")]
        BS[("Broker policy and ownership journal")]
    end
    W["Windows system clock"]
    L["SCM and W32Time lifecycle"]
    N <-->|"F1 / BND-1,2: NTP, NTS and TLS"| E
    O -->|"F2 / BND-4: untrusted requests"| U
    O -->|"F2 / BND-4: direct public pipe requests"| B
    U <-->|"F2 / BND-4: authenticated local IPC"| B
    E <-->|"F3 / BND-3: proposals, policy and acknowledgements"| B
    U -->|"F4: request specific UAC action"| A
    A <-->|"F4 / BND-5: authorized policy transaction"| B
    E <-->|"F5 / BND-6: service-owned state"| ES
    B <-->|"F6 / BND-6: protected state"| BS
    B -->|"F7 / BND-7: bounded clock writes"| W
    A -->|"F8 / BND-5,7: installation and restoration"| L
```

The outer box is an **installation boundary**, not a shared privilege boundary. UI, engine, broker and helper run under different effective authorities. The numbered crossings map to the flow inventory in section 14. Other local processes can call the public pipe directly; the official UI is not an authorization boundary. F1 also covers conditional certificate AIA/root/revocation retrieval, potentially performed by delegated Windows services; the held verifier route and actual egress owner must qualify under section 18.7.2. The time-endpoint node is not proof that all retrieval is engine-only or limited to those endpoints. The installer also creates protected files and service configuration, as detailed in section 19.

## 2. Correction request and acknowledgement

```mermaid
sequenceDiagram
    participant U as Native UI
    participant B as Clock broker
    participant E as Engine service
    participant N as Time sources
    participant W as Windows clock
    U->>B: Request refresh under current policy
    B->>B: Authenticate caller, authorize and coalesce
    B->>E: Request measurements for current generation
    E->>N: Bounded NTP or NTS exchanges
    N-->>E: Untrusted responses
    E->>E: Admit samples, estimate and select agreement
    E->>B: Typed bounded correction proposal
    B->>B: Check identity, ownership, epochs and total budget
    alt Current proposal is authorized and within bounds
        B->>W: Apply permitted correction
        W-->>B: API result and adjustment readback
        B-->>E: Acknowledge applied state and generation
        E->>E: Reconcile model with acknowledged actuation
    else Admission or ownership fails
        B-->>E: Reject with no new clock write
    end
    B-->>U: Coherent status, freshness and reason
```

An API success/readback acknowledges what the backend applied or accepted; it does not itself measure end-to-end UTC accuracy. An unknown acknowledgement is reconciled against broker state before retry. The UI request carries no target timestamp or destination. Both diagrams describe required future behavior, not tested implementation.

## 3. 6.1.1 Input-to-effect validation flow

```mermaid
flowchart TD
    I["IPC bytes or protected file bytes"] --> B["Bounded read and decoder"]
    B --> W["Strict inert wire record"]
    W --> D["Private fallible construction: garde and domain rules"]
    D --> G["Current authority and state guard"]
    C["OS caller identity and broker policy"] --> G
    G --> E["Serialized protected effect"]
    E --> A["Actual result and acknowledgement"]
    B -->|"invalid"| R["Bounded redacted rejection"]
    D -->|"invalid"| R
    G -->|"denied or stale"| R
```

The guard uses trusted caller identity, current policy/ownership and broker-maintained budgets; none is supplied by the wire record as proof. Transport authentication and coarse endpoint access checks already occur before a client is allowed expensive work. Decode and construction failures have no protected effect. A successful clock operation updates the model only through the actual acknowledgement. A file read has a file/schema-specific authority context, not a fabricated IPC caller.

The constructor obligatorily invokes shared garde DTO/settings validation and the
domain rules before publishing a value. Internal function/module contracts carry
unchanged immutable invariants through typed APIs and check transformed outputs
or changed context as required by [architecture section 6.3.1](windows-time-architecture.md#631-contracts-for-every-function-and-module).
This is the adopted design, not an executed application validation result.

## 4. Recovery decision flow

```mermaid
flowchart TD
    S["Need authenticated time"] --> C{"Usable cached NTS association?"}
    C -->|"Yes"| P["Request fresh protected NTS samples"]
    C -->|"No or rejected"| K["Attempt strict NTS-KE"]
    K -->|"Valid TLS"| P
    K -->|"Failure"| D{"Certificate-date circularity only?"}
    D -->|"No"| H["Holdover or blocked; report cause"]
    D -->|"Yes"| O{"Bootstrap explicitly enabled?"}
    O -->|"No"| H
    O -->|"Yes"| N["Collect one or two configured plain NTP hints"]
    N --> V{"Bounded date interval and helper policy pass?"}
    V -->|"No"| H
    V -->|"Yes"| T["Isolated TLS verifier uses candidate time"]
    T -->|"All other certificate checks pass"| A["Collect protected NTS samples"]
    T -->|"Failure"| H
    A --> F["Verify source agreement and certificate-time consistency"]
    F -->|"Pass"| R["Fresh isolated TLS validation at authenticated candidate time"]
    F -->|"Fail"| H
    R -->|"Pass; retain bootstrap provenance"| B["Broker checks ownership and correction policy"]
    R -->|"Fail"| H
    P -->|"Cached association rejected"| K
    P --> Q{"Fresh evidence meets source policy?"}
    Q -->|"Yes"| B
    Q -->|"No"| H
    B -->|"Authorized"| W["Apply and acknowledge bounded correction"]
    B -->|"Rejected"| H
```

The operating-system clock is unchanged throughout candidate-time TLS work. Plain hints never enter the ordinary source population or authorize a clock write. Two-helper mode requires both helpers and does not silently become one-helper mode. Agreement and a subsequent fresh handshake improve consistency; they cannot erase the residual expired-compromised-key attack or manufacture trusted UTC from unauthenticated hints. The detailed admission rules in sections 8.1-8.4 remain authoritative.

## 5. Installation and removal ownership flow

```mermaid
flowchart TD
    P["One complete signed MSI"] --> V["Preflight complete payload, identities, policy and recovery"]
    V -->|"Fail or required rollback disabled"| X["Stop before clock takeover"]
    V -->|"Pass"| J["Durably record prior W32Time state and ownership generation"]
    J --> O["Quiesce prior owner and activate one broker generation"]
    O --> C{"Activation verified?"}
    C -->|"Yes"| A["One installed product: UI, engine, broker and helper"]
    C -->|"No"| R["Recover transaction using current ownership evidence"]
    A -->|"Update or repair"| Q["Quiesce and validate complete replacement"]
    Q -->|"Compatible and verified"| A
    Q -->|"Failure"| R
    A -->|"Uninstall"| U["Quiesce owned corrections; begin removal"]
    U --> D{"Captured restoration still authorized?"}
    R --> K{"Prior working product can be restored safely?"}
    K -->|"Yes"| A
    K -->|"No"| D
    D -->|"Yes"| W["Restore prior W32Time state and commit result"]
    W -->|"Durable success"| F["Finalize removal of product components or rollback cleanup"]
    W -->|"Restoration failed"| B["Preserve recovery mechanism; report incomplete state"]
    D -->|"No or uncertain"| X2{"Safe removal without restoration established?"}
    X2 -->|"Yes; record later administrator or policy choice"| F
    X2 -->|"No"| B
```

The removal node is a transaction, not an instruction to delete recovery code first: section 11 and section 19.2 require durable restoration results before removing the last recovery mechanism. Update-driven removal of the old version does not release ownership to W32Time. Every rate reset, including cleanup, requires current ownership. A private journal cannot overrule a later administrator or Group Policy decision.
