//! Fixed fixture presentation and checked control-to-model interactions.
//!
//! No production snapshot, policy draft, authority or persistence is represented.
//! Every native call occurs after releasing any portable-model borrow.

use std::ffi::c_void;

use windows_sys::Win32::Foundation::{HINSTANCE, HWND, LPARAM, WPARAM};
use windows_sys::Win32::UI::WindowsAndMessaging::{BN_CLICKED, LBN_SELCHANGE};

use super::controls::{Controls, Id, InputError, Widget, client_size};
use super::{NativeError, State, parameter_pointer};

/// Small mutually exclusive fixture views; no service connection state exists.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Page {
    /// Illustrative time/health values with a disabled synchronization action.
    Status,
    /// Deterministic source list and transient local edit preview.
    Sources,
    /// Explanation of unavailable machine policy and fixture defaults.
    Settings,
}

/// Fixed child specification; all handles remain owned by the root window.
type ControlSpec = (Id, Option<Page>, Widget, &'static str);

/// Persistent, compact header and local navigation shared by every page.
const COMMON_CONTROLS: &[ControlSpec] = &[
    (Id::Brand, None, Widget::Label, "NevroTick"),
    (
        Id::Banner,
        None,
        Widget::Label,
        "Prototype — sample data. No connection or clock change is made.",
    ),
    (Id::Status, None, Widget::Button, "Status"),
    (Id::Sources, None, Widget::Button, "Sources"),
    (Id::Settings, None, Widget::Button, "Settings"),
];

/// Fixed illustrative evidence; none of these values describes the host computer.
const STATUS_CONTROLS: &[ControlSpec] = &[
    (
        Id::StatusText,
        Some(Page::Status),
        Widget::Label,
        "Clock status",
    ),
    (
        Id::StatusProfile,
        Some(Page::Status),
        Widget::Label,
        "Sample profile: NTS only · UTC — unsmeared",
    ),
    (
        Id::StatusHeadline,
        Some(Page::Status),
        Widget::Label,
        "Time sync active — sample Tracking state",
    ),
    (
        Id::SampleTime,
        Some(Page::Status),
        Widget::Label,
        "Sample time: 29 October 2026, 12:34:56 UTC\r\nFixed illustration; these values do not describe your computer.",
    ),
    (
        Id::EvidenceGroup,
        Some(Page::Status),
        Widget::GroupBox,
        "Sample evidence",
    ),
    (
        Id::EvidenceText,
        Some(Page::Status),
        Widget::Label,
        "Authentication: NTS authenticated (sample)\r\nSource agreement: 3 sample sources agree\r\nApplied correction: none — this prototype cannot change the clock",
    ),
    (
        Id::OffsetGroup,
        Some(Page::Status),
        Widget::GroupBox,
        "Estimated offset (sample)",
    ),
    (
        Id::OffsetValue,
        Some(Page::Status),
        Widget::Label,
        "+0.42 ms\r\nWindows is behind the sample reference",
    ),
    (
        Id::UncertaintyGroup,
        Some(Page::Status),
        Widget::GroupBox,
        "Estimated uncertainty (sample)",
    ),
    (
        Id::UncertaintyValue,
        Some(Page::Status),
        Widget::Label,
        "±1.8 ms\r\nIllustrative estimate, not a precision guarantee",
    ),
    (
        Id::Sync,
        Some(Page::Status),
        Widget::DisabledButton,
        "Sync now",
    ),
    (
        Id::ViewSources,
        Some(Page::Status),
        Widget::Button,
        "View sources",
    ),
    (
        Id::SyncReason,
        Some(Page::Status),
        Widget::Label,
        "Sync now is unavailable in this prototype. Live measurements and synchronization are not connected.",
    ),
];

/// Source selection, separate accepted details and an in-memory edit preview.
const SOURCE_CONTROLS: &[ControlSpec] = &[
    (
        Id::SourcesTitle,
        Some(Page::Sources),
        Widget::Label,
        "Time servers",
    ),
    (
        Id::SourcesProfile,
        Some(Page::Sources),
        Widget::Label,
        "Sample profile: NTS only · 3 illustrative sources · No connection is made",
    ),
    (
        Id::SourceList,
        Some(Page::Sources),
        Widget::List,
        "Sample sources",
    ),
    (
        Id::DetailsGroup,
        Some(Page::Sources),
        Widget::GroupBox,
        "Selected source",
    ),
    (Id::Details, Some(Page::Sources), Widget::Label, ""),
    (
        Id::EditorTitle,
        Some(Page::Sources),
        Widget::GroupBox,
        "Edit local preview",
    ),
    (
        Id::LabelCaption,
        Some(Page::Sources),
        Widget::Label,
        "Display name",
    ),
    (
        Id::HostCaption,
        Some(Page::Sources),
        Widget::Label,
        "Server hostname or IP address",
    ),
    (
        Id::PortCaption,
        Some(Page::Sources),
        Widget::Label,
        "NTS-KE TCP port",
    ),
    (Id::Label, Some(Page::Sources), Widget::Edit, ""),
    (Id::Host, Some(Page::Sources), Widget::Edit, ""),
    (Id::Port, Some(Page::Sources), Widget::Edit, ""),
    (
        Id::Update,
        Some(Page::Sources),
        Widget::Button,
        "Update preview",
    ),
    (Id::Cancel, Some(Page::Sources), Widget::Button, "Cancel"),
    (Id::EditorMessage, Some(Page::Sources), Widget::Label, ""),
    (
        Id::EditorHint,
        Some(Page::Sources),
        Widget::Label,
        "In-memory preview only. Close discards edits.",
    ),
];

/// Read-only grouping; the doubled group-box ampersand renders as a literal one.
const SETTINGS_CONTROLS: &[ControlSpec] = &[
    (
        Id::SettingsText,
        Some(Page::Settings),
        Widget::Label,
        "Settings",
    ),
    (
        Id::SettingsIntro,
        Some(Page::Settings),
        Widget::Label,
        "Sample settings — no machine policy is configured",
    ),
    (
        Id::TrustGroup,
        Some(Page::Settings),
        Widget::GroupBox,
        "Time sources && trust",
    ),
    (
        Id::TrustText,
        Some(Page::Settings),
        Widget::Label,
        "Sample profile: NTS only\r\nTime scale: UTC — unsmeared (sample)\r\nThree illustrative sources; no trust policy is installed.",
    ),
    (
        Id::CorrectionGroup,
        Some(Page::Settings),
        Widget::GroupBox,
        "Clock correction",
    ),
    (
        Id::CorrectionText,
        Some(Page::Settings),
        Widget::Label,
        "Unavailable in this prototype. No clock change or service configuration is possible.\r\nNo administrator access is requested.",
    ),
    (
        Id::PreferencesGroup,
        Some(Page::Settings),
        Widget::GroupBox,
        "Prototype preferences — read only",
    ),
    (
        Id::PreferencesText,
        Some(Page::Settings),
        Widget::Label,
        "Appearance: standard Windows controls\r\nStorage: in memory only; preview edits disappear on Close",
    ),
    (
        Id::Policy,
        Some(Page::Settings),
        Widget::DisabledButton,
        "Change machine policy",
    ),
    (
        Id::PolicyReason,
        Some(Page::Settings),
        Widget::Label,
        "Unavailable in this prototype.\r\nMachine policy cannot be changed.",
    ),
];

impl Controls {
    /// Construct the finite standard-control inventory under one owning parent.
    ///
    /// # Errors
    /// Returns child creation/setup failure; the parent still owns partial children.
    pub(super) fn create(parent: HWND, instance: HINSTANCE) -> Result<Self, NativeError> {
        let specifications = COMMON_CONTROLS
            .iter()
            .chain(STATUS_CONTROLS)
            .chain(SOURCE_CONTROLS)
            .chain(SETTINGS_CONTROLS);
        let mut controls = Self {
            items: Vec::with_capacity(
                COMMON_CONTROLS.len()
                    + STATUS_CONTROLS.len()
                    + SOURCE_CONTROLS.len()
                    + SETTINGS_CONTROLS.len(),
            ),
        };
        for &(id, page, widget, text) in specifications {
            controls.add(parent, instance, id, page, widget, text)?;
        }
        Ok(controls)
    }
}

impl State {
    /// Require the once-created fixed inventory; never constructs fallback state.
    ///
    /// # Errors
    /// Returns an uninitialized-inventory contract failure.
    fn controls(&self) -> Result<&Controls, NativeError> {
        self.controls.get().ok_or(NativeError::Contract(
            "Native controls are not initialized.",
        ))
    }

    /// Show one page while preserving the persistent sample context/navigation.
    ///
    /// # Errors
    /// Returns missing controls or native geometry/layout failure.
    pub(super) fn show_page(&self, page: Page) -> Result<(), NativeError> {
        self.page.set(page);
        for control in &self.controls()?.items {
            control.show(control.page.is_none_or(|owner| owner == page));
        }
        self.layout()
    }

    /// Fit the fixed view inside the minimum reference rectangle; zero is minimized.
    ///
    /// # Errors
    /// Returns unsupported dimensions, rectangle retrieval or control-move failure.
    pub(super) fn layout(&self) -> Result<(), NativeError> {
        let Some(controls) = self.controls.get() else {
            return Ok(());
        };
        let (width, height) = client_size(self.window.get())?;
        if width == 0 || height == 0 {
            return Ok(());
        }
        // WM_GETMINMAXINFO constrains interactive resizing. A programmatically
        // smaller window is rejected rather than overflowing signed geometry.
        if width < 700 || height < 550 {
            return Err(NativeError::Contract(
                "Window is smaller than the fixture layout supports.",
            ));
        }
        for control in &controls.items {
            control.place(rectangle(control.id, width, height)?)?;
        }
        Ok(())
    }

    /// Repaint accepted selected values after dropping the immutable model borrow.
    ///
    /// # Errors
    /// Returns reentrant model access, missing controls or native text/list failure.
    pub(super) fn refresh_sources(&self) -> Result<(), NativeError> {
        let (labels, selected, details, label, host, port) = {
            let model = self
                .model
                .try_borrow()
                .map_err(|_| NativeError::Contract("Reentrant source-model read."))?;
            let source = model.selected();
            let labels = model
                .sources()
                .iter()
                .map(|item| format!("{} — {}", item.label(), item.host()))
                .collect::<Vec<_>>();
            let details = format!(
                "{}\r\n\r\nServer identity\r\n{}:{}\r\n\r\nNTS authentication — sample only\r\nUTC — unsmeared (sample)\r\n\r\nNo connection is made to this address.",
                source.label(),
                source.host(),
                source.port(),
            );
            (
                labels,
                model.selected_index(),
                details,
                source.label().to_owned(),
                source.host().to_owned(),
                source.port().to_string(),
            )
        };
        let controls = self.controls()?;
        controls
            .get(Id::SourceList)?
            .set_sources(&labels, selected)?;
        controls.get(Id::Details)?.set_text(&details)?;
        controls.get(Id::Label)?.set_text(&label)?;
        controls.get(Id::Host)?.set_text(&host)?;
        controls.get(Id::Port)?.set_text(&port)?;
        Ok(())
    }

    /// Admit only commands from the matching owned child; semantic guards still
    /// run for direct/programmatic Update preview requests.
    ///
    /// # Errors
    /// Returns invalid native selection, reentrant model access or a failed
    /// control operation. Unknown senders/commands cause no effect and return Ok.
    pub(super) fn command(&self, word: WPARAM, long: LPARAM) -> Result<(), NativeError> {
        let Some(controls) = self.controls.get() else {
            return Ok(());
        };
        let identifier = u16::try_from(word & 0xffff)
            .map_err(|_| NativeError::Contract("Invalid native control identifier."))?;
        let notification = u32::try_from((word >> 16) & 0xffff)
            .map_err(|_| NativeError::Contract("Invalid native notification."))?;
        let sender = parameter_pointer::<c_void>(long).cast_mut();
        let Some(control) = controls
            .items
            .iter()
            .find(|item| item.id.value() == identifier && item.handle == sender)
        else {
            return Ok(());
        };
        if control.id == Id::SourceList && notification == LBN_SELCHANGE {
            let selected = control.selection()?;
            self.model
                .try_borrow_mut()
                .map_err(|_| NativeError::Contract("Reentrant source-model selection."))?
                .select(selected)
                .map_err(|_| {
                    NativeError::Contract("Native source selection is outside the fixture.")
                })?;
            self.refresh_sources()?;
            return controls.get(Id::EditorMessage)?.set_text("");
        }
        if notification != BN_CLICKED {
            return Ok(());
        }
        match control.id {
            Id::Status => self.show_page(Page::Status),
            Id::Sources | Id::ViewSources => self.show_page(Page::Sources),
            Id::Settings => self.show_page(Page::Settings),
            Id::Update => self.update_preview(),
            Id::Cancel => {
                self.refresh_sources()?;
                controls
                    .get(Id::EditorMessage)?
                    .set_text("Unaccepted editor changes discarded.")
            }
            _ => Ok(()),
        }
    }

    /// Collect bounded raw values, invoke the sole checked constructor path and
    /// publish only its accepted preview. Invalid input leaves the model intact.
    ///
    /// # Errors
    /// Returns missing controls, native read/display failure or reentrant model
    /// access. User-input rejection displays a bounded message and returns Ok.
    fn update_preview(&self) -> Result<(), NativeError> {
        let controls = self.controls()?;
        let inputs = (|| {
            Ok::<_, InputError>((
                controls
                    .get(Id::Label)
                    .map_err(InputError::Native)?
                    .input()?,
                controls
                    .get(Id::Host)
                    .map_err(InputError::Native)?
                    .input()?,
                controls
                    .get(Id::Port)
                    .map_err(InputError::Native)?
                    .input()?,
            ))
        })();
        let (label, host, port) = match inputs {
            Ok(values) => values,
            Err(InputError::Native(error)) => return Err(error),
            Err(InputError::TooLong) => {
                return controls
                    .get(Id::EditorMessage)?
                    .set_text("Cannot update preview: input exceeds the native text budget.");
            }
            Err(InputError::InvalidUnicode) => {
                return controls
                    .get(Id::EditorMessage)?
                    .set_text("Cannot update preview: input contains invalid Unicode.");
            }
        };
        let result = self
            .model
            .try_borrow_mut()
            .map_err(|_| NativeError::Contract("Reentrant preview update."))?
            .update_selected(&label, &host, &port);
        match result {
            Ok(()) => {
                self.refresh_sources()?;
                controls.get(Id::EditorMessage)?.set_text(
                    "Preview updated in memory only. No connection or policy change was made.",
                )
            }
            Err(error) => controls
                .get(Id::EditorMessage)?
                .set_text(&format!("Cannot update preview: {error}")),
        }
    }
}

/// Map a fixed control identity into admitted client geometry. Each result is
/// checked to fit before native movement; no page-specific height is discarded.
///
/// # Errors
/// Returns unsupported client dimensions, invalid geometry or out-of-client bounds.
fn rectangle(id: Id, width: i32, height: i32) -> Result<[i32; 4], NativeError> {
    if width < 700 || height < 550 {
        return Err(NativeError::Contract(
            "Unsupported fixture client dimensions.",
        ));
    }
    let column = (width - 56) / 2;
    let geometry = match id {
        Id::Brand => [20, 12, width - 40, 22],
        Id::Banner => [20, 38, width - 40, 22],
        Id::Status => [20, 68, 104, 28],
        Id::Sources => [132, 68, 104, 28],
        Id::Settings => [244, 68, 104, 28],
        Id::StatusText | Id::SourcesTitle | Id::SettingsText => [20, 112, width - 40, 24],
        Id::StatusProfile | Id::SourcesProfile | Id::SettingsIntro => [20, 142, width - 40, 22],
        Id::StatusHeadline => [20, 174, width - 40, 22],
        Id::SampleTime => [20, 204, width - 40, 36],
        Id::EvidenceGroup => [20, 244, width - 40, 92],
        Id::EvidenceText => [32, 266, width - 64, 58],
        Id::OffsetGroup => [20, 348, column, 88],
        Id::OffsetValue => [32, 374, column - 24, 48],
        Id::UncertaintyGroup => [36 + column, 348, column, 88],
        Id::UncertaintyValue => [48 + column, 374, column - 24, 48],
        Id::Sync => [20, height - 90, 136, 28],
        Id::ViewSources => [168, height - 90, 136, 28],
        Id::SyncReason => [20, height - 52, width - 40, 40],
        Id::SourceList => [20, 172, 280, 84],
        Id::DetailsGroup => [20, 270, 280, height - 290],
        Id::Details => [32, 292, 256, height - 332],
        Id::EditorTitle => [316, 172, width - 336, height - 192],
        Id::LabelCaption => [332, 202, width - 368, 20],
        Id::HostCaption => [332, 264, width - 368, 20],
        Id::PortCaption => [332, 326, width - 368, 20],
        Id::Label => [332, 224, width - 368, 26],
        Id::Host => [332, 286, width - 368, 26],
        Id::Port => [332, 348, 100, 26],
        Id::Update => [332, 390, 136, 28],
        Id::Cancel => [480, 390, 100, 28],
        Id::EditorMessage => [332, 438, width - 368, height - 482],
        Id::EditorHint => [332, height - 44, width - 368, 22],
        Id::TrustGroup => [20, 174, width - 40, 104],
        Id::TrustText => [32, 198, width - 64, 68],
        Id::CorrectionGroup => [20, 290, width - 40, 92],
        Id::CorrectionText => [32, 314, width - 64, 56],
        Id::PreferencesGroup => [20, 394, width - 40, 82],
        Id::PreferencesText => [32, 418, width - 64, 44],
        Id::Policy => [20, height - 62, 196, 28],
        Id::PolicyReason => [232, height - 64, width - 252, 44],
    };
    let [x, y, control_width, control_height] = geometry;
    if x < 0
        || y < 0
        || control_width <= 0
        || control_height <= 0
        || x.checked_add(control_width)
            .is_none_or(|right| right > width)
        || y.checked_add(control_height)
            .is_none_or(|bottom| bottom > height)
    {
        return Err(NativeError::Contract(
            "Fixture control exceeds client bounds.",
        ));
    }
    Ok(geometry)
}
