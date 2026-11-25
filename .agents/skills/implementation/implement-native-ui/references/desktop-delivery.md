# Native tray and notification delivery

Use for reached tray/toast adapter changes. Resolve the actual backend and current
[GUI handoff](../../../../../tasks/slint-gui-handoff.md#desktop-delivery-hardening--25-november-2026),
then the existing source/native checkpoint. Preserve UI-M0/D1–D3 boundaries,
rich-toast deferral and execution holds; guidance adds no production presenter,
cold activation, dependency or required UI-M0 feature.

## Bound recovery at the native receiver

Treat registered/window messages as untrusted hints, not action authority.
Retain code-owned commands, current interaction/owner guards and ordinary-window
reachability. A coalesced UI wake or one pending event cannot bound Shell work
performed before enqueue. Before registration/recovery calls, freeze finite
attempt/rate/work budgets per recovery window, one pending intent and a locally
owned retry cutoff. Incoming repeats cannot renew them; an earlier burst cannot
permanently prevent later legitimate recovery.

Track actual Add/SetVersion/Modify outcomes and loss generation. Keep stop intent,
native cleanup, timeout and joined completion distinct; pressure cannot authorize
an action or hide unresolved ownership. Apply the
[tray recovery contract](../../../../../docs/nevrotick-gui-native-input-residuals.md#tray-recovery-admission--25-november-2026).
Pair genuine Explorer recreation (and reached display-change behavior) with
repeated/forged hints, failed Shell registration with later useful recovery,
and storms with disable/Quit. Observe
native call/work bounds and cleanup before teardown; portable counters establish
only their exercised seam. Inspect the selected callback window/backend instead
of assuming an upstream tray implementation is the reached product path.

## Preserve submission and activation meaning

Use [W11-W09](../../../../../tasks/windows11-integration.md#w11-w09-add-truthful-native-notifications-at-the-deferred-ui-stage)
for incident policy and selected activation entries. Keep explicit tests separate
from live incidents; log importance/history is no fresh incident authority.
Use bounded literal text, fixed document shape and closed navigation targets.
Activation rechecks current user/session, observer, notice/generation and state;
it grants no Apply/Retry/Start/Stop, elevation or clock authority. Bound callback
queues/work and preserve apartment/thread and handler-token ownership.

For the selected local WinRT path, set per-toast
[NotificationMirroring](https://learn.microsoft.com/en-us/uwp/api/windows.ui.notifications.toastnotification.notificationmirroring)
to `Disabled` before Show, including the engineering example. Policy-establishment
failure means preparation refusal without Show; ordinary status/required logging
continues. Do not infer the property's default or actual mirroring from an absent
setter, or promise control of unrelated capture/sync paths. Keep public-safe text.

Distinguish refusal, actual attempt, OS acceptance/uncertainty and observed
presentation. Unknown visibility does not authorize replay of an attempted notice.
Keep running callbacks separate from any selected packaged/cold activation entry;
an unimplemented entry remains unavailable.

## Retire only the owned presentation

Separate handler detachment, banner hiding and Notification Center retirement.
[Hide](https://learn.microsoft.com/en-us/uwp/api/windows.ui.notifications.toastnotifier.hide)
success does not establish desktop history removal. If the history adapter uses
[Remove](https://learn.microsoft.com/en-us/uwp/api/windows.ui.notifications.toastnotificationhistory.remove),
bind bounded nonsecret code-owned tag/group/app identifiers to current ownership
and generation. Freeze bounded per-user/session observer coordination before live
delivery; duplicate launches or late old-instance work cannot duplicate one
eligible delivery or retire a successor/unrelated notice. Automatic owned cleanup
must not fall back to clearing every app notification. Do not infer a single-instance
product policy from this requirement.

Retain unresolved cleanup and safe stale-activation handling. Pair banner and
history-only entries, foreground/background close, owned/unrelated/successor
notices, invalid activation and native failure with useful current navigation and
permitted presentation. API returns alone prove neither displayed/read content
nor independently observed history removal. Keep actual lifecycle observations
separate from source/portable checks and package qualification.
