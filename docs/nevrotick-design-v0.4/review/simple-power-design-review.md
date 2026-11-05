# NevroTick — Simple / Power user design review

**Basis:** current v0.3 source; the user's explicit correction requests two modes and configurable synchronization periodicity. Source/design review only; no browser or native qualification.

## 1. Selected split

Use **one shared scenario/state model and two presentations**. Default to **Simple**, with a visible, ordinary **Simple / Power user** switch inside the application. Mode changes alter information density, not authority, authentication, source policy or service state.

| Surface | Simple | Power user |
| --- | --- | --- |
| Navigation | Status / Settings | Existing Status / Sources / Diagnostics / Settings; About secondary |
| Status | One compact panel: profile, plain headline, current consequence, restrained offset + uncertainty, last confirmed estimate, disabled Sync now | Existing evidence, activity, calendar, source agreement, ages, recovery ancestry and detailed actions |
| Settings | Synchronization-check interval; brief machine-policy notice; minimal window preferences | Same cadence control plus source policy, correction/recovery, logging and package details |
| Failures | Plain cause, impact and one useful next action; details reachable through the mode switch | Full evidence, source reasons and diagnostic/export flows |
| Reviewer chrome | One collapsed “Сценарии макета” disclosure | Same disclosure; never embedded among product settings |

## 2. Clutter to remove from Simple

- Remove the separate Evidence, Clock activity, UTC calendar and freshness/history cards, repeated section kickers and duplicate “View sources” actions.
- Remove implementation/qualification essays from the normal product surface; retain one small “Prototype — sample data” marker and a collapsed reviewer explanation.
- Avoid a second page title repeating the state headline. Keep normal operation to one brief status sentence and one next-check line.
- Do not render four large metric/statistic tiles, an always-visible mascot, or a decorative clock. Offset and uncertainty belong on one quiet line.
- Keep source/provider/majority terms out of routine prose; “Authenticated NTS · sources agree” is sufficient when accurate.

## 3. Truth that Simple must retain

- Plain NTP stays visibly **Unauthenticated NTP**. Observe-only says **Clock changes disabled**; it must not imply Windows Time was re-enabled.
- Holdover says the last estimate is being used; expiry says no usable estimate remains. Neither is generic healthy green.
- Disconnection says **Current status unavailable** with the last-snapshot age; service health remains unknown unless separately confirmed.
- Conflict, blocked ownership, pending/expired approval and unknown applied outcome retain their actual consequence and next action.
- A leap guard remains a rechecking state; unknown future coverage can be a short qualifier. Do not turn metadata into a fabricated outage.
- Future recovery scenes keep **Future design** and state that bootstrap did not change Windows time. A healthy-looking headline must not erase unauthenticated recovery ancestry; keep a short “Recovery basis needs review” link/qualifier where material.

## 4. Synchronization periodicity

Label the control **Synchronization checks** or **Check interval**; provide **Automatic** and illustrative fixed intervals. Explain briefly: “Requests fresh measurements; the service still enforces provider limits.” This is not a frequency-control/clock-step knob, an accuracy promise, or permission to exceed a server's limits.

Store the requested cadence in the prototype's state and show it consistently in both modes. Distinguish **Active: Automatic (sample)** from **Preview: Every …**. Keep real **Apply changes** disabled. Selecting a preview interval must not update the active source fixture, pretend a service accepted it, or silently rewrite the live-looking next-check countdown. Exact allowed intervals and effective scheduling remain a production policy decision; the new user request authorizes the design control, not invented qualified constants.

## 5. Mode-change and data continuity

- Add an explicit mode action; do not implement it by calling `initialState()` or the scenario-reset action.
- Preserve the scenario, source drafts, requested cadence, source/event filters, settings section and reviewed values when switching modes.
- Leaving a Power-only page opens Simple Status; remember the last Power page for return. Stay on Status/Settings when either is already valid in both modes.
- Keep draft state discoverable with a small “Preview changes retained” note in Simple Settings; changing mode must not apply or discard it.
- An open source editor must not lose raw input on a mode rerender. Keep mode controls inaccessible while the modal is open, or store/repopulate the raw editor state.
- Preserve chosen mode when selecting a different review scenario. An explicit **Reset scene** may reset fixture drafts/cadence, but say so and keep the user's chosen presentation mode.

## 6. Bounded checks for the next candidate

Check Simple's initial DOM contains only two navigation destinations and one status panel; Power retains the detailed pages. For each scenario compare profile, headline meaning, correction permission and stale/future qualifiers across both presentations. Exercise Power draft → Simple → Power, cadence change → mode switch, invalid cadence rejection, scenario reset, and a modal opened during mode changes. Confirm all actual effects remain local editing/explicit export; protected actions stay disabled. Source checks do not establish rendered layout or native accessibility.
