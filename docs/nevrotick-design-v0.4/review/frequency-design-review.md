# Synchronization frequency — bounded design recommendation

**Date:** 5 November 2026 (Asia/Tbilisi). **Basis:** NevroTick `8a66af924c509e898a9bab9bb3282f11b24fba93`.
**Scope:** design-only response to David's explicit new frequency-setting request; no implementation changes, browser use or runtime qualification.

## Recommendation

Provide **Sync frequency** in Settings in both Simple and Power user modes. Default the preview selection to **Automatic (recommended)**; reveal the interval selector only for **Custom**. This is compatible with F-02: users should not *need* to choose an interval, which is different from prohibiting an optional control. Simple/Power changes presentation only, never authentication, source counts, recovery or clock authority.

| Control | Suggested English copy | Meaning |
| --- | --- | --- |
| Settings group | **Synchronization** | Routine time-source measurements. |
| Main field | **Sync frequency** | Familiar user-facing name. |
| Choice 1 | **Automatic (recommended)** | Service adapts measurements within the admitted policy. |
| Choice 2 | **Custom** | Reveal one preference field; not an exact execution guarantee. |
| Custom field | **Preferred check interval** | Desired ordinary measurement cadence, not a clock-step interval. |
| Compact illustrative presets | **2 minutes · 5 minutes · 15 minutes** | Review examples: 120/300/900 seconds lie inside the documented 64–1024-second starting range. These are not qualified production presets or new min/max limits. |
| Single helper line | **Controls time checks. Recovery and server limits may change the actual interval.** | Avoid a permanent wall of safety/implementation text. |
| Preview save | **Update preview** | Change only the in-memory design preference; no service-policy success. |

Do not add arbitrary seconds/minimum/maximum knobs or a long list of intervals to this revision. A short preset selector satisfies the request and fits the simpler interface. If a free-form custom field is wanted later, its accepted range, units and backend mapping need an explicit policy; do not present an invented range as safe for all providers.

## Interaction and semantics

1. Automatic shows only its selected value and one short explanation. Custom exposes the three presets. Retain the user's last manual choice in memory when toggling Automatic/Custom; page reload may reset this offline preview.
2. The picker represents a **requested measurement interval**, not a command to set Windows time on every tick. It cannot change the source profile, NTS agreement, correction budget or exceptional-approval behavior.
3. Do not schedule a new check immediately after every selection, mode switch or page navigation. The future engine's single scheduler handles all triggers, coalescing and budgets.
4. In the preview, keep requested preference separate from the active scenario's timing. Selecting 5 minutes must not silently relabel an existing 40-second fixture countdown as an acknowledged service change.
5. In production the service validates and activates scheduling policy through the existing machine-policy path; viewing/editing does not elevate. Protected Apply remains outside this artifact.
6. Keep effective scheduling details out of Simple status. In Power source details, optional read-only text can distinguish **Preferred interval: 5 min** from **Next permitted check: delayed by server request**. Show an actual countdown only when supplied by the current fixture/service; do not manufacture one from the selector.

## Essential guardrails

- `RATE` asks for less frequent queries; a requested shorter interval or Sync now cannot override it. `DENY`/`RSTR` cannot become recurring polling at the chosen cadence.
- Recovery bursts, retries and ordinary polls share per-source and aggregate admission. A custom preference does not add an independent timer loop or reset backoff.
- Long intervals do not preserve time confidence indefinitely: estimate age/uncertainty still govern holdover/expiry. A source-limit delay changes cadence, not authentication or quorum.
- “Automatic” is a practical design recommendation; the example presets are only review values. Exact adopted defaults, permitted envelopes and scheduler mapping remain implementation decisions requiring the existing checks.

## Primary repository anchors

- [F-02 — background synchronization](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/windows-time-features.md#f-02--synchronize-automatically-in-the-background): bounded polling, no required manual tuning, useful server-limit explanations.
- [Architecture §7.3 — NTP client behavior](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/windows-time-architecture.md#73-ntp-client-behavior): KoD requirements; one scheduler for polls, recovery, network events and user refresh; current-generation/per-source/aggregate guards.
- [Architecture §17 — resource targets](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/windows-time-architecture.md#17-resource-and-performance-targets): documented 64–1024-second starting range, adaptation within policy and bounded startup/reacquisition.
- [F-05 — Sync now](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/windows-time-features.md#f-05--request-a-fresh-check-with-sync-now): measurement request is separate from correction authority; coalescing/rate limits.
- [Architecture §6.3](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/windows-time-architecture.md#63-domain-construction-and-invariant-preservation) and [§12.2](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/windows-time-architecture.md#122-configuration-semantics): checked PollInterval, cross-field bounds, explicit units and atomic policy activation.
- [F-06 — source/profile/settings management](https://github.com/DavidOsipov/NevroTick/blob/8a66af924c509e898a9bab9bb3282f11b24fba93/docs/windows-time-features.md#f-06--inspect-and-change-sources-and-profiles-safely): machine policy versus per-user presentation preferences.

**Minimal review acceptance:** mode switching never changes policy; Automatic/Custom and preset selection work locally; active fixture evidence is unchanged by draft preference; server-limit scenes can explain a longer effective interval; no live request or privileged Apply is introduced.
