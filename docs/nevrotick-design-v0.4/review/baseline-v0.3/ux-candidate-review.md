# NevroTick v0.3 — independent candidate UX/source review

**Reviewed:** 5 November 2026, Asia/Tbilisi  
**Target:** `NevroTick-UI-v0.3/src/{model.js,views.js,app.js,styles.css}`, `build.cjs`, and the generated standalone HTML.  
**Mode:** read-only inspection and bounded Node checks. Implementation was being revised concurrently; findings below were sent promptly to the coordinator. No implementation file was edited by this reviewer.  
**Limit:** no new browser rendering, UI automation, native Windows observation, assistive-technology test or visual qualification was attempted. The cloud browser restriction remains in force.

## Overall assessment

The new prototype follows the requested direction: an offline utility with recognizable Status/Sources/Settings navigation, discoverable Diagnostics, an in-memory editor, final brand assets, English product copy and Russian review controls. The main trust distinctions are visible. Protected commands are disabled, and the actual permitted effects are local specimen editing, explicit clipboard copying and explicit sample-report download.

The most useful corrections concern consistency between scenario evidence and subsidiary views, a malformed IPv6 input path, and a few local interaction details. They are bounded changes; none requires production services, network access or restoring native D1–D3 work.

## Findings sent to the coordinator

### C-01 — Scenario amendments were temporarily placed before initialization (blocking execution)

During correction of earlier findings, an `amend()` block was inserted above the declaration of `SCENARIOS`. Requiring `model.js` immediately failed with `ReferenceError: Cannot access 'SCENARIOS' before initialization`. This was reproduced by Node and reported immediately. Place the amendments after scenario construction and before derived source/event construction. The coordinator must rerun the model import and build after the correction.

This is a transient implementation finding from the concurrent edit, not a claim that the final artifact retains it.

### C-02 — IPv6 validation admitted invalid edge-colon forms (P1)

`validHost()` counted nonempty groups after filtering empty strings. The following malformed values returned `true`:

- `1:2:3:4:5:6:7:8:`
- `:1:2:3:4:5:6:7:8`
- `1::2:`
- `:1::2`

Positive neighboring controls `2001:db8::1` and `1:2:3:4:5:6:7:8` passed, while `999.1.2.3` and `https://ntp.example` were rejected. Reject a lone leading/trailing colon unless it belongs to the one allowed `::` compression. If IPv4-embedded IPv6 is intentionally unsupported, either support it or state that limited accepted syntax; the initial implementation rejected `::ffff:192.0.2.1` despite presenting an unrestricted “IP address” label.

Scope is local form correctness. No production validation or network security claim follows from passing this check.

### C-03 — Plain NTP inherited an NTS certificate-time basis (P1)

The initial NTP scenario correctly changed packet authentication and ports, but retained `recovery: Strict NTS verification` and the baseline association note. Diagnostics, the recovery card and the export could therefore claim NTS certificate ancestry for plain NTP. Set explicit `Not applicable — plain NTP` context and corresponding explanatory copy. The coordinator began this correction during review.

### C-04 — Historical and expired evidence needs consistency in every view (P1)

Three related initial inconsistencies were reported:

1. The stale snapshot had `18 s ago` source-reply/accepted-measurement strings. The Sources page's historical notice helped, but its detached source-details dialog did not carry that qualification. Use `18 s at last snapshot`, historical authentication/role wording where appropriate, and a visible dialog qualifier.
2. The expired correction scenario still showed `3 of 4 · minimum + majority met` while all of its source rows were Waiting with expired evidence. Report no current agreement, or explicitly mark the agreement as historical and non-authorizing.
3. Observe-only stored `Read-only observation` as the **clock owner**. Mode and owner are separate dimensions. Use the known owner (for example NevroTick in this fixture) and separately state that product clock changes are disabled and Windows Time is not automatically re-enabled.

The coordinator began these corrections during review. Validate Status, Sources, source details, recovery and exported text together after integration.

### C-05 — Full rerenders lose focus on local filter/section actions (P2)

`dispatch()` replaces the entire `mount.innerHTML`. It focuses the page heading for page/scenario changes and restores an opener after modal closure, but source-filter, event-filter, compact-window and Settings-section actions have no equivalent focus preservation. The clicked/focused control is removed from the DOM. This is visible in source; actual browser behavior was not exercised.

Retain action/id information and focus the replacement control after render, or focus the new Settings section heading. The search-input path already updates only `.source-panel`, which is a better continuity pattern. This recommendation concerns ordinary HTML interaction, not native Narrator/UIA qualification.

### C-06 — New preview entries need a convenient edit path (P2)

Draft entries initially offered only **Undo**. Active entries could open **Edit preview**, and the editor already reads existing drafts, but an added draft had no exposed Edit action. Add **Edit preview** to nonremoval draft rows, or explain the remove-and-readd limitation. In review, include display-name changes: the initial old/new comparison displayed only host/protocol/port even though names were editable.

### C-07 — Preserve legibility as information increases (design recommendation)

Many current CSS declarations set primary table/reason/freshness content to 10–11 px, with some mobile helper text at 9 px. This is a source-based readability concern, not a measured contrast or rendered-overflow failure. Prefer at least 12 px for ordinary table/status content and 11 px for secondary explanations where practical. Keep the smallest lettering for nonessential reviewer captions.

## Positive checks actually executed

Using Node and the model/view modules, the reviewer observed:

| Check | Observed result / limit |
| --- | --- |
| All scenario/page view combinations | 23 scenarios × 6 pages returned HTML without exception on the pre-amendment candidate; this is render-function execution, not browser layout verification |
| Main-page ID inventory | No duplicate IDs in those nonmodal render combinations; the coordinator separately corrected the modal boundary ID |
| Draft navigation | A new source draft survived Sources → Diagnostics → Sources |
| Active versus preview separation | Valid draft editing and invalid hostname rejection left the active fixture unchanged; an invalid edit preserved the existing draft |
| Search/filter composition | Query `operator b` selected Beta; combining it with the Waiting/Not used filter returned no matches as expected |
| Literal text handling | `<b>display</b>` was escaped in the Sources draft render and was not inserted as a tag |
| Offline standalone packaging | Generated script bytes matched the embedded CSP SHA-256; `connect-src 'none'` present; no remote `src`/`href`; no unresolved `assets/` references |
| Clipboard/download implementation | Source inspection found user-initiated local operations, an explicit clipboard-denied message, and no promise that a browser-controlled download had already been written successfully; no browser operation executed |

Protected buttons carry the disabled attribute and visible specimen context. The generated report identifies itself as a design fixture and does not include user-entered source names or addresses. These observations support the local-preview boundary, not a full security audit.

## Historical visual references inspected

The reviewer opened the repository's existing `docs/nevrotick-design-v0.2/screens/status.jpg` and `sources.jpg`. They are historical references, not captures of v0.3.

Useful properties to preserve are the quiet utility layout, top-level tabs, paired offset and uncertainty, stable source rows, explicit protocol/role text, and restrained branding. The new separate Diagnostics destination and grouped review selector fit the expanded evidence without requiring a general dashboard. The current-source typography concern above should be checked visually when permitted browser access is available.

## Final handoff requirement

The coordinator should resolve the execution and evidence-consistency findings, rebuild the self-contained artifact, rerun the affected Node checks on those exact bytes, and state the remaining browser limitation. Do not describe this review as proof of native UI behavior, source independence, current time accuracy, Store availability or completed accessibility gates.

## Final disposition after the coordinator's corrections

The coordinator identified a stable candidate for recheck. The reviewer independently reran **11 focused Node checks; all passed**, and inspected the amended interaction and style source. This section supersedes the earlier open status of the specific findings below.

| Finding | Final disposition in the checked candidate |
| --- | --- |
| C-01 initialization failure | **Corrected.** `model.js` initializes and exports 23 scenarios; scenario amendments run after construction. |
| C-02 malformed IPv6 | **Reported defect corrected.** All four invalid edge-colon forms are rejected. `2001:db8::1`, the full eight-group address, `::1` and `::` remain accepted. This check does not qualify a complete production IP/identity parser; IPv4-embedded IPv6 is still outside this small preview parser. |
| C-03 NTP certificate ancestry | **Corrected.** The NTP fixture explicitly reports `Not applicable — plain NTP`. |
| C-04 evidence consistency | **Corrected for the reported cases.** Observe-only retains NevroTick as known owner with a separate no-writes explanation; stale source ages say “at last snapshot” and details carry a historical notice; an expired proposal has zero current agreeing groups and no current agreement. |
| C-05 focus continuity | **Correction present in source.** `dispatch()` captures the focused element's ID or action/value and focuses its replacement after relevant nonmodal rerenders. Browser execution remains unverified. |
| C-06 draft interaction | **Edit path corrected.** New draft rows expose **Edit preview**, verified in rendered HTML. The old/new review table still omits display names; a name-only edit can look identical in its comparison cells. This is a minor, disclosed presentation limitation and does not change the active fixture. |
| C-07 readability | **CSS correction present.** Final overrides raise normal product paragraphs/table content/facts to 12 px and secondary text to 11 px, with selected source names at 13 px. Actual wrapping, clipping and visual balance remain unverified without a browser. |

The standalone artifact's executable script matched the current model/view/application source bytes after asset embedding, and its SHA-256 CSP hash matched that actual embedded script. Checked standalone SHA-256:

`9d232652c35c2bb0eff9888c96b6be1a2dbc529e450ef5d6426450ae79257878`

This digest identifies the bytes reviewed at this checkpoint, not any later rebuilt artifact. No browser or native qualification claim is added by this final disposition.
