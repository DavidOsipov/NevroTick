//! Generated protected output slots; no catalog text can generate source.
use crate::localization::feedback::{FeedbackSlot, PreparedFeedback};
use crate::localization::{Error, MessageId, PlainText, Snapshot};
use slint::{Model, ModelRc, ModelTracker, SharedString};
use std::num::NonZeroU64;

/// Maximum retained UTF-8 bytes across one complete copy publication.
const PROJECTION_BYTES: usize = 131_072;
/// Maximum retained UTF-16 units across one complete copy publication.
const PROJECTION_UTF16: usize = 65_536;

/// One shared output budget, independent of string/model representation.
#[derive(Default)]
struct OutputBudget {
    /// All static, feedback and revision UTF-8 bytes.
    bytes: usize,
    /// All static, feedback and revision UTF-16 units.
    utf16: usize,
}
impl OutputBudget {
    /// Charge both limits before mutating either counter.
    ///
    /// # Errors
    /// Rejects checked arithmetic or aggregate exhaustion before retention.
    fn charge(&mut self, text: &str) -> Result<(), Error> {
        let bytes = self
            .bytes
            .checked_add(text.len())
            .filter(|value| *value <= PROJECTION_BYTES)
            .ok_or(Error::Bounds)?;
        let utf16 = self
            .utf16
            .checked_add(text.encode_utf16().count())
            .filter(|value| *value <= PROJECTION_UTF16)
            .ok_or(Error::Bounds)?;
        self.bytes = bytes;
        self.utf16 = utf16;
        Ok(())
    }

    /// Retain an already checked literal only after complete output accounting.
    ///
    /// # Errors
    /// Rejects aggregate output exhaustion before allocating toolkit text.
    fn retain(&mut self, text: &PlainText) -> Result<SharedString, Error> {
        self.charge(text.as_str())?;
        Ok(text.as_str().into())
    }
}

/// One static preparation borrows a single immutable catalog generation.
struct CopyPreparation<'a, 'catalog> {
    /// Every protected message comes from the same snapshot.
    snapshot: &'a Snapshot<'catalog>,
    /// One counter pair spans the complete publication.
    budget: OutputBudget,
}
impl CopyPreparation<'_, '_> {
    /// Render one checked argument-free message through the shared budget.
    ///
    /// # Errors
    /// Rejects message-kind mismatch or bounded rendering/output failure.
    fn text(&mut self, id: MessageId) -> Result<SharedString, Error> {
        self.budget.retain(&self.snapshot.text(id)?)
    }

    /// Render only a positive code-owned minute choice.
    ///
    /// # Errors
    /// Rejects zero, wrong arguments or bounded rendering/output failure.
    fn minutes(&mut self, id: MessageId, value: u64) -> Result<SharedString, Error> {
        let minutes = NonZeroU64::new(value).ok_or(Error::Arguments)?;
        self.budget.retain(&self.snapshot.minutes(id, minutes)?)
    }
}

/// Immutable shared rows make generated property getters shallow and cheap.
/// No mutable cell, model notification or retained window exists in this owner.
#[derive(Debug)]
struct ImmutableCopyModel {
    /// Fixed code-owned slot order; the model is replaced as a whole.
    rows: Box<[SharedString]>,
}
impl Model for ImmutableCopyModel {
    /// Each bounded row is inert text, never an action or identity selector.
    type Data = SharedString;

    /// Report the fixed shape of this fully prepared immutable generation.
    fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Clone only the requested reference-counted string, not the whole catalog.
    fn row_data(&self, row: usize) -> Option<Self::Data> {
        self.rows.get(row).cloned()
    }

    /// Public model writes cannot mutate approved or privately retained copy.
    fn set_row_data(&self, _row: usize, _data: Self::Data) {}

    /// Constant rows have no mutation tracker; replacing the carrier invalidates bindings.
    fn model_tracker(&self) -> &dyn ModelTracker {
        &()
    }
}

use super::generated::UiCopy;
/// Number of fixed static output slots.
pub(super) const STATIC_SLOT_COUNT: usize = 397;
/// Complete fixed static and typed-feedback shape.
pub(super) const OUTPUT_SLOT_COUNT: usize = 426;
/// Protected output position; no catalog key or action can select it.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) struct CopySlot(usize);
impl CopySlot {
    /// Return this code-owned model position.
    #[must_use]
    pub(super) const fn index(self) -> usize {
        self.0
    }
    /// Fixed output for `fixture_window_title`.
    pub(super) const FIXTURE_WINDOW_TITLE: Self = Self(0);
    /// Fixed output for `fixture_sample_label`.
    pub(super) const FIXTURE_SAMPLE_LABEL: Self = Self(1);
    /// Fixed output for `fixture_local_boundary`.
    pub(super) const FIXTURE_LOCAL_BOUNDARY: Self = Self(2);
    /// Fixed output for `fixture_memory_boundary`.
    pub(super) const FIXTURE_MEMORY_BOUNDARY: Self = Self(3);
    /// Fixed output for `fixture_simple_mode`.
    pub(super) const FIXTURE_SIMPLE_MODE: Self = Self(4);
    /// Fixed output for `fixture_power_mode`.
    pub(super) const FIXTURE_POWER_MODE: Self = Self(5);
    /// Fixed output for `fixture_status_tab`.
    pub(super) const FIXTURE_STATUS_TAB: Self = Self(6);
    /// Fixed output for `fixture_sources_tab`.
    pub(super) const FIXTURE_SOURCES_TAB: Self = Self(7);
    /// Fixed output for `fixture_statistics_tab`.
    pub(super) const FIXTURE_STATISTICS_TAB: Self = Self(8);
    /// Fixed output for `fixture_diagnostics_tab`.
    pub(super) const FIXTURE_DIAGNOSTICS_TAB: Self = Self(9);
    /// Fixed output for `fixture_settings_tab`.
    pub(super) const FIXTURE_SETTINGS_TAB: Self = Self(10);
    /// Fixed output for `fixture_about_tab`.
    pub(super) const FIXTURE_ABOUT_TAB: Self = Self(11);
    /// Fixed output for `fixture_preview_badge`.
    pub(super) const FIXTURE_PREVIEW_BADGE: Self = Self(12);
    /// Fixed output for `fixture_not_measured`.
    pub(super) const FIXTURE_NOT_MEASURED: Self = Self(13);
    /// Fixed output for `fixture_not_evaluated`.
    pub(super) const FIXTURE_NOT_EVALUATED: Self = Self(14);
    /// Fixed output for `fixture_unavailable`.
    pub(super) const FIXTURE_UNAVAILABLE: Self = Self(15);
    /// Fixed output for `fixture_no_live_connection`.
    pub(super) const FIXTURE_NO_LIVE_CONNECTION: Self = Self(16);
    /// Fixed output for `fixture_status_kicker`.
    pub(super) const FIXTURE_STATUS_KICKER: Self = Self(17);
    /// Fixed output for `fixture_status_heading`.
    pub(super) const FIXTURE_STATUS_HEADING: Self = Self(18);
    /// Fixed output for `fixture_status_qualifier`.
    pub(super) const FIXTURE_STATUS_QUALIFIER: Self = Self(19);
    /// Fixed output for `fixture_difference_label`.
    pub(super) const FIXTURE_DIFFERENCE_LABEL: Self = Self(20);
    /// Fixed output for `fixture_difference_note`.
    pub(super) const FIXTURE_DIFFERENCE_NOTE: Self = Self(21);
    /// Fixed output for `fixture_uncertainty_label`.
    pub(super) const FIXTURE_UNCERTAINTY_LABEL: Self = Self(22);
    /// Fixed output for `fixture_uncertainty_note`.
    pub(super) const FIXTURE_UNCERTAINTY_NOTE: Self = Self(23);
    /// Fixed output for `fixture_authentication_label`.
    pub(super) const FIXTURE_AUTHENTICATION_LABEL: Self = Self(24);
    /// Fixed output for `fixture_authentication_value`.
    pub(super) const FIXTURE_AUTHENTICATION_VALUE: Self = Self(25);
    /// Fixed output for `fixture_authentication_note`.
    pub(super) const FIXTURE_AUTHENTICATION_NOTE: Self = Self(26);
    /// Fixed output for `fixture_agreement_label`.
    pub(super) const FIXTURE_AGREEMENT_LABEL: Self = Self(27);
    /// Fixed output for `fixture_agreement_note`.
    pub(super) const FIXTURE_AGREEMENT_NOTE: Self = Self(28);
    /// Fixed output for `fixture_qualified_label`.
    pub(super) const FIXTURE_QUALIFIED_LABEL: Self = Self(29);
    /// Fixed output for `fixture_sync_now`.
    pub(super) const FIXTURE_SYNC_NOW: Self = Self(30);
    /// Fixed output for `fixture_sync_unavailable`.
    pub(super) const FIXTURE_SYNC_UNAVAILABLE: Self = Self(31);
    /// Fixed output for `fixture_sources_heading`.
    pub(super) const FIXTURE_SOURCES_HEADING: Self = Self(32);
    /// Fixed output for `fixture_sources_description`.
    pub(super) const FIXTURE_SOURCES_DESCRIPTION: Self = Self(33);
    /// Fixed output for `fixture_source_list_note`.
    pub(super) const FIXTURE_SOURCE_LIST_NOTE: Self = Self(34);
    /// Fixed output for `fixture_edit_source`.
    pub(super) const FIXTURE_EDIT_SOURCE: Self = Self(35);
    /// Fixed output for `fixture_select_source`.
    pub(super) const FIXTURE_SELECT_SOURCE: Self = Self(36);
    /// Fixed output for `fixture_selected`.
    pub(super) const FIXTURE_SELECTED: Self = Self(37);
    /// Fixed output for `fixture_server_identity`.
    pub(super) const FIXTURE_SERVER_IDENTITY: Self = Self(38);
    /// Fixed output for `fixture_display_name`.
    pub(super) const FIXTURE_DISPLAY_NAME: Self = Self(39);
    /// Fixed output for `fixture_nts_port`.
    pub(super) const FIXTURE_NTS_PORT: Self = Self(40);
    /// Fixed output for `fixture_protocol_label`.
    pub(super) const FIXTURE_PROTOCOL_LABEL: Self = Self(41);
    /// Fixed output for `fixture_nts_preview`.
    pub(super) const FIXTURE_NTS_PREVIEW: Self = Self(42);
    /// Fixed output for `fixture_source_role`.
    pub(super) const FIXTURE_SOURCE_ROLE: Self = Self(43);
    /// Fixed output for `fixture_last_reply`.
    pub(super) const FIXTURE_LAST_REPLY: Self = Self(44);
    /// Fixed output for `fixture_editor_description`.
    pub(super) const FIXTURE_EDITOR_DESCRIPTION: Self = Self(45);
    /// Fixed output for `fixture_label_help`.
    pub(super) const FIXTURE_LABEL_HELP: Self = Self(46);
    /// Fixed output for `fixture_host_help`.
    pub(super) const FIXTURE_HOST_HELP: Self = Self(47);
    /// Fixed output for `fixture_editor_unavailable`.
    pub(super) const FIXTURE_EDITOR_UNAVAILABLE: Self = Self(48);
    /// Fixed output for `fixture_update_preview`.
    pub(super) const FIXTURE_UPDATE_PREVIEW: Self = Self(49);
    /// Fixed output for `fixture_cancel`.
    pub(super) const FIXTURE_CANCEL: Self = Self(50);
    /// Fixed output for `fixture_statistics_heading`.
    pub(super) const FIXTURE_STATISTICS_HEADING: Self = Self(51);
    /// Fixed output for `fixture_statistics_description`.
    pub(super) const FIXTURE_STATISTICS_DESCRIPTION: Self = Self(52);
    /// Fixed output for `fixture_statistics_empty_title`.
    pub(super) const FIXTURE_STATISTICS_EMPTY_TITLE: Self = Self(53);
    /// Fixed output for `fixture_one_snapshot`.
    pub(super) const FIXTURE_ONE_SNAPSHOT: Self = Self(54);
    /// Fixed output for `fixture_offset_label`.
    pub(super) const FIXTURE_OFFSET_LABEL: Self = Self(55);
    /// Fixed output for `fixture_positive_offset_note`.
    pub(super) const FIXTURE_POSITIVE_OFFSET_NOTE: Self = Self(56);
    /// Fixed output for `fixture_diagnostics_heading`.
    pub(super) const FIXTURE_DIAGNOSTICS_HEADING: Self = Self(57);
    /// Fixed output for `fixture_diagnostics_description`.
    pub(super) const FIXTURE_DIAGNOSTICS_DESCRIPTION: Self = Self(58);
    /// Fixed output for `fixture_observation_description`.
    pub(super) const FIXTURE_OBSERVATION_DESCRIPTION: Self = Self(59);
    /// Fixed output for `fixture_freshness_heading`.
    pub(super) const FIXTURE_FRESHNESS_HEADING: Self = Self(60);
    /// Fixed output for `fixture_freshness_note`.
    pub(super) const FIXTURE_FRESHNESS_NOTE: Self = Self(61);
    /// Fixed output for `fixture_applied_label`.
    pub(super) const FIXTURE_APPLIED_LABEL: Self = Self(62);
    /// Fixed output for `fixture_no_correction`.
    pub(super) const FIXTURE_NO_CORRECTION: Self = Self(63);
    /// Fixed output for `fixture_snapshot_label`.
    pub(super) const FIXTURE_SNAPSHOT_LABEL: Self = Self(64);
    /// Fixed output for `fixture_fixture_snapshot`.
    pub(super) const FIXTURE_FIXTURE_SNAPSHOT: Self = Self(65);
    /// Fixed output for `fixture_diagnostics_events`.
    pub(super) const FIXTURE_DIAGNOSTICS_EVENTS: Self = Self(66);
    /// Fixed output for `fixture_settings_description`.
    pub(super) const FIXTURE_SETTINGS_DESCRIPTION: Self = Self(67);
    /// Fixed output for `fixture_schedule_heading`.
    pub(super) const FIXTURE_SCHEDULE_HEADING: Self = Self(68);
    /// Fixed output for `fixture_interval_label`.
    pub(super) const FIXTURE_INTERVAL_LABEL: Self = Self(69);
    /// Fixed output for `fixture_service_heading`.
    pub(super) const FIXTURE_SERVICE_HEADING: Self = Self(70);
    /// Fixed output for `fixture_service_description`.
    pub(super) const FIXTURE_SERVICE_DESCRIPTION: Self = Self(71);
    /// Fixed output for `fixture_runtime_label`.
    pub(super) const FIXTURE_RUNTIME_LABEL: Self = Self(72);
    /// Fixed output for `fixture_runtime_value`.
    pub(super) const FIXTURE_RUNTIME_VALUE: Self = Self(73);
    /// Fixed output for `fixture_startup_label`.
    pub(super) const FIXTURE_STARTUP_LABEL: Self = Self(74);
    /// Fixed output for `fixture_startup_value`.
    pub(super) const FIXTURE_STARTUP_VALUE: Self = Self(75);
    /// Fixed output for `fixture_owner_label`.
    pub(super) const FIXTURE_OWNER_LABEL: Self = Self(76);
    /// Fixed output for `fixture_owner_value`.
    pub(super) const FIXTURE_OWNER_VALUE: Self = Self(77);
    /// Fixed output for `fixture_synchronization_label`.
    pub(super) const FIXTURE_SYNCHRONIZATION_LABEL: Self = Self(78);
    /// Fixed output for `fixture_synchronization_value`.
    pub(super) const FIXTURE_SYNCHRONIZATION_VALUE: Self = Self(79);
    /// Fixed output for `fixture_admin_required`.
    pub(super) const FIXTURE_ADMIN_REQUIRED: Self = Self(80);
    /// Fixed output for `fixture_service_protected_note`.
    pub(super) const FIXTURE_SERVICE_PROTECTED_NOTE: Self = Self(81);
    /// Fixed output for `fixture_start_service`.
    pub(super) const FIXTURE_START_SERVICE: Self = Self(82);
    /// Fixed output for `fixture_stop_service`.
    pub(super) const FIXTURE_STOP_SERVICE: Self = Self(83);
    /// Fixed output for `fixture_restart_service`.
    pub(super) const FIXTURE_RESTART_SERVICE: Self = Self(84);
    /// Fixed output for `fixture_disable_service`.
    pub(super) const FIXTURE_DISABLE_SERVICE: Self = Self(85);
    /// Fixed output for `fixture_lifecycle_note`.
    pub(super) const FIXTURE_LIFECYCLE_NOTE: Self = Self(86);
    /// Fixed output for `fixture_about_heading`.
    pub(super) const FIXTURE_ABOUT_HEADING: Self = Self(87);
    /// Fixed output for `fixture_about_tagline`.
    pub(super) const FIXTURE_ABOUT_TAGLINE: Self = Self(88);
    /// Fixed output for `fixture_about_title`.
    pub(super) const FIXTURE_ABOUT_TITLE: Self = Self(89);
    /// Fixed output for `fixture_about_description`.
    pub(super) const FIXTURE_ABOUT_DESCRIPTION: Self = Self(90);
    /// Fixed output for `fixture_about_stage`.
    pub(super) const FIXTURE_ABOUT_STAGE: Self = Self(91);
    /// Fixed output for `fixture_author_label`.
    pub(super) const FIXTURE_AUTHOR_LABEL: Self = Self(92);
    /// Fixed output for `fixture_repository_label`.
    pub(super) const FIXTURE_REPOSITORY_LABEL: Self = Self(93);
    /// Fixed output for `fixture_version_label`.
    pub(super) const FIXTURE_VERSION_LABEL: Self = Self(94);
    /// Fixed output for `fixture_version_value`.
    pub(super) const FIXTURE_VERSION_VALUE: Self = Self(95);
    /// Fixed output for `fixture_platform_label`.
    pub(super) const FIXTURE_PLATFORM_LABEL: Self = Self(96);
    /// Fixed output for `fixture_code_license_label`.
    pub(super) const FIXTURE_CODE_LICENSE_LABEL: Self = Self(97);
    /// Fixed output for `fixture_technologies_heading`.
    pub(super) const FIXTURE_TECHNOLOGIES_HEADING: Self = Self(98);
    /// Fixed output for `fixture_technologies_description`.
    pub(super) const FIXTURE_TECHNOLOGIES_DESCRIPTION: Self = Self(99);
    /// Fixed output for `fixture_chrony_note`.
    pub(super) const FIXTURE_CHRONY_NOTE: Self = Self(100);
    /// Fixed output for `fixture_privacy_heading`.
    pub(super) const FIXTURE_PRIVACY_HEADING: Self = Self(101);
    /// Fixed output for `fixture_privacy_description`.
    pub(super) const FIXTURE_PRIVACY_DESCRIPTION: Self = Self(102);
    /// Fixed output for `fixture_artwork_heading`.
    pub(super) const FIXTURE_ARTWORK_HEADING: Self = Self(103);
    /// Fixed output for `fixture_artwork_description`.
    pub(super) const FIXTURE_ARTWORK_DESCRIPTION: Self = Self(104);
    /// Fixed output for `fixture_open_repository`.
    pub(super) const FIXTURE_OPEN_REPOSITORY: Self = Self(105);
    /// Fixed output for `fixture_repository_help`.
    pub(super) const FIXTURE_REPOSITORY_HELP: Self = Self(106);
    /// Fixed output for `fixture_repository_error`.
    pub(super) const FIXTURE_REPOSITORY_ERROR: Self = Self(107);
    /// Fixed output for `fixture_gui_build_label`.
    pub(super) const FIXTURE_GUI_BUILD_LABEL: Self = Self(108);
    /// Fixed output for `fixture_text_contract_label`.
    pub(super) const FIXTURE_TEXT_CONTRACT_LABEL: Self = Self(109);
    /// Fixed output for `preferences_schedule_description`.
    pub(super) const PREFERENCES_SCHEDULE_DESCRIPTION: Self = Self(110);
    /// Fixed output for `preferences_scheduling_note`.
    pub(super) const PREFERENCES_SCHEDULING_NOTE: Self = Self(111);
    /// Fixed output for `preferences_custom_label`.
    pub(super) const PREFERENCES_CUSTOM_LABEL: Self = Self(112);
    /// Fixed output for `preferences_custom_help`.
    pub(super) const PREFERENCES_CUSTOM_HELP: Self = Self(113);
    /// Fixed output for `preferences_accepted_interval`.
    pub(super) const PREFERENCES_ACCEPTED_INTERVAL: Self = Self(114);
    /// Fixed output for `preferences_preferences_boundary`.
    pub(super) const PREFERENCES_PREFERENCES_BOUNDARY: Self = Self(115);
    /// Fixed output for `preferences_host_unavailable`.
    pub(super) const PREFERENCES_HOST_UNAVAILABLE: Self = Self(116);
    /// Fixed output for `preferences_notifications_heading`.
    pub(super) const PREFERENCES_NOTIFICATIONS_HEADING: Self = Self(117);
    /// Fixed output for `preferences_notifications_label`.
    pub(super) const PREFERENCES_NOTIFICATIONS_LABEL: Self = Self(118);
    /// Fixed output for `preferences_notifications_help`.
    pub(super) const PREFERENCES_NOTIFICATIONS_HELP: Self = Self(119);
    /// Fixed output for `preferences_windows_availability`.
    pub(super) const PREFERENCES_WINDOWS_AVAILABILITY: Self = Self(120);
    /// Fixed output for `preferences_unknown_availability`.
    pub(super) const PREFERENCES_UNKNOWN_AVAILABILITY: Self = Self(121);
    /// Fixed output for `preferences_availability_help`.
    pub(super) const PREFERENCES_AVAILABILITY_HELP: Self = Self(122);
    /// Fixed output for `preferences_observer_label`.
    pub(super) const PREFERENCES_OBSERVER_LABEL: Self = Self(123);
    /// Fixed output for `preferences_observer_unavailable`.
    pub(super) const PREFERENCES_OBSERVER_UNAVAILABLE: Self = Self(124);
    /// Fixed output for `preferences_observer_help`.
    pub(super) const PREFERENCES_OBSERVER_HELP: Self = Self(125);
    /// Fixed output for `preferences_test_notification`.
    pub(super) const PREFERENCES_TEST_NOTIFICATION: Self = Self(126);
    /// Fixed output for `preferences_notification_test_unavailable`.
    pub(super) const PREFERENCES_NOTIFICATION_TEST_UNAVAILABLE: Self = Self(127);
    /// Fixed output for `preferences_preferences_heading`.
    pub(super) const PREFERENCES_PREFERENCES_HEADING: Self = Self(128);
    /// Fixed output for `preferences_tray_label`.
    pub(super) const PREFERENCES_TRAY_LABEL: Self = Self(129);
    /// Fixed output for `preferences_tray_help`.
    pub(super) const PREFERENCES_TRAY_HELP: Self = Self(130);
    /// Fixed output for `preferences_sign_in_label`.
    pub(super) const PREFERENCES_SIGN_IN_LABEL: Self = Self(131);
    /// Fixed output for `preferences_sign_in_help`.
    pub(super) const PREFERENCES_SIGN_IN_HELP: Self = Self(132);
    /// Fixed output for `preferences_tray_adapter_unavailable`.
    pub(super) const PREFERENCES_TRAY_ADAPTER_UNAVAILABLE: Self = Self(133);
    /// Fixed output for `preferences_interval_options_0`.
    pub(super) const PREFERENCES_INTERVAL_OPTIONS_0: Self = Self(134);
    /// Fixed output for `preferences_interval_options_1`.
    pub(super) const PREFERENCES_INTERVAL_OPTIONS_1: Self = Self(135);
    /// Fixed output for `preferences_interval_options_2`.
    pub(super) const PREFERENCES_INTERVAL_OPTIONS_2: Self = Self(136);
    /// Fixed output for `preferences_interval_options_3`.
    pub(super) const PREFERENCES_INTERVAL_OPTIONS_3: Self = Self(137);
    /// Fixed output for `preferences_interval_options_4`.
    pub(super) const PREFERENCES_INTERVAL_OPTIONS_4: Self = Self(138);
    /// Fixed output for `preferences_incidents_heading`.
    pub(super) const PREFERENCES_INCIDENTS_HEADING: Self = Self(139);
    /// Fixed output for `preferences_incidents_unavailable`.
    pub(super) const PREFERENCES_INCIDENTS_UNAVAILABLE: Self = Self(140);
    /// Fixed output for `preferences_language_label`.
    pub(super) const PREFERENCES_LANGUAGE_LABEL: Self = Self(141);
    /// Fixed output for `preferences_language_help`.
    pub(super) const PREFERENCES_LANGUAGE_HELP: Self = Self(142);
    /// Fixed output for `preferences_notification_windows_allowed`.
    pub(super) const PREFERENCES_NOTIFICATION_WINDOWS_ALLOWED: Self = Self(143);
    /// Fixed output for `preferences_notification_windows_blocked`.
    pub(super) const PREFERENCES_NOTIFICATION_WINDOWS_BLOCKED: Self = Self(144);
    /// Fixed output for `preferences_notification_observer_running`.
    pub(super) const PREFERENCES_NOTIFICATION_OBSERVER_RUNNING: Self = Self(145);
    /// Fixed output for `preferences_tray_ready_note`.
    pub(super) const PREFERENCES_TRAY_READY_NOTE: Self = Self(146);
    /// Fixed output for `source_profile_value`.
    pub(super) const SOURCE_PROFILE_VALUE: Self = Self(147);
    /// Fixed output for `source_preview_note`.
    pub(super) const SOURCE_PREVIEW_NOTE: Self = Self(148);
    /// Fixed output for `source_add_source`.
    pub(super) const SOURCE_ADD_SOURCE: Self = Self(149);
    /// Fixed output for `source_remove_source`.
    pub(super) const SOURCE_REMOVE_SOURCE: Self = Self(150);
    /// Fixed output for `source_review_changes`.
    pub(super) const SOURCE_REVIEW_CHANGES: Self = Self(151);
    /// Fixed output for `source_review_heading`.
    pub(super) const SOURCE_REVIEW_HEADING: Self = Self(152);
    /// Fixed output for `source_review_description`.
    pub(super) const SOURCE_REVIEW_DESCRIPTION: Self = Self(153);
    /// Fixed output for `source_before_label`.
    pub(super) const SOURCE_BEFORE_LABEL: Self = Self(154);
    /// Fixed output for `source_after_label`.
    pub(super) const SOURCE_AFTER_LABEL: Self = Self(155);
    /// Fixed output for `source_undo`.
    pub(super) const SOURCE_UNDO: Self = Self(156);
    /// Fixed output for `source_discard`.
    pub(super) const SOURCE_DISCARD: Self = Self(157);
    /// Fixed output for `source_close_review`.
    pub(super) const SOURCE_CLOSE_REVIEW: Self = Self(158);
    /// Fixed output for `source_apply`.
    pub(super) const SOURCE_APPLY: Self = Self(159);
    /// Fixed output for `source_apply_unavailable`.
    pub(super) const SOURCE_APPLY_UNAVAILABLE: Self = Self(160);
    /// Fixed output for `source_empty_heading`.
    pub(super) const SOURCE_EMPTY_HEADING: Self = Self(161);
    /// Fixed output for `source_empty_description`.
    pub(super) const SOURCE_EMPTY_DESCRIPTION: Self = Self(162);
    /// Fixed output for `source_empty_review`.
    pub(super) const SOURCE_EMPTY_REVIEW: Self = Self(163);
    /// Fixed output for `source_add_title`.
    pub(super) const SOURCE_ADD_TITLE: Self = Self(164);
    /// Fixed output for `source_edit_title`.
    pub(super) const SOURCE_EDIT_TITLE: Self = Self(165);
    /// Fixed output for `source_protocols_0`.
    pub(super) const SOURCE_PROTOCOLS_0: Self = Self(166);
    /// Fixed output for `source_protocols_1`.
    pub(super) const SOURCE_PROTOCOLS_1: Self = Self(167);
    /// Fixed output for `source_protocol_help`.
    pub(super) const SOURCE_PROTOCOL_HELP: Self = Self(168);
    /// Fixed output for `source_ntp_warning`.
    pub(super) const SOURCE_NTP_WARNING: Self = Self(169);
    /// Fixed output for `source_ntp_port`.
    pub(super) const SOURCE_NTP_PORT: Self = Self(170);
    /// Fixed output for `source_port_help`.
    pub(super) const SOURCE_PORT_HELP: Self = Self(171);
    /// Fixed output for `source_statistics_description`.
    pub(super) const SOURCE_STATISTICS_DESCRIPTION: Self = Self(172);
    /// Fixed output for `source_change_labels_0`.
    pub(super) const SOURCE_CHANGE_LABELS_0: Self = Self(173);
    /// Fixed output for `source_change_labels_1`.
    pub(super) const SOURCE_CHANGE_LABELS_1: Self = Self(174);
    /// Fixed output for `source_change_labels_2`.
    pub(super) const SOURCE_CHANGE_LABELS_2: Self = Self(175);
    /// Fixed output for `source_change_labels_3`.
    pub(super) const SOURCE_CHANGE_LABELS_3: Self = Self(176);
    /// Fixed output for `source_review_none`.
    pub(super) const SOURCE_REVIEW_NONE: Self = Self(177);
    /// Fixed output for `source_page_required`.
    pub(super) const SOURCE_PAGE_REQUIRED: Self = Self(178);
    /// Fixed output for `source_list_required`.
    pub(super) const SOURCE_LIST_REQUIRED: Self = Self(179);
    /// Fixed output for `source_review_required`.
    pub(super) const SOURCE_REVIEW_REQUIRED: Self = Self(180);
    /// Fixed output for `recovery_title`.
    pub(super) const RECOVERY_TITLE: Self = Self(181);
    /// Fixed output for `recovery_description`.
    pub(super) const RECOVERY_DESCRIPTION: Self = Self(182);
    /// Fixed output for `recovery_back`.
    pub(super) const RECOVERY_BACK: Self = Self(183);
    /// Fixed output for `recovery_settings_description`.
    pub(super) const RECOVERY_SETTINGS_DESCRIPTION: Self = Self(184);
    /// Fixed output for `recovery_context_description`.
    pub(super) const RECOVERY_CONTEXT_DESCRIPTION: Self = Self(185);
    /// Fixed output for `recovery_open_details`.
    pub(super) const RECOVERY_OPEN_DETAILS: Self = Self(186);
    /// Fixed output for `recovery_leave_title`.
    pub(super) const RECOVERY_LEAVE_TITLE: Self = Self(187);
    /// Fixed output for `recovery_leave_description`.
    pub(super) const RECOVERY_LEAVE_DESCRIPTION: Self = Self(188);
    /// Fixed output for `recovery_leave_keep`.
    pub(super) const RECOVERY_LEAVE_KEEP: Self = Self(189);
    /// Fixed output for `recovery_leave_discard`.
    pub(super) const RECOVERY_LEAVE_DISCARD: Self = Self(190);
    /// Fixed output for `recovery_leave_stay`.
    pub(super) const RECOVERY_LEAVE_STAY: Self = Self(191);
    /// Fixed output for `recovery_roughtime_title`.
    pub(super) const RECOVERY_ROUGHTIME_TITLE: Self = Self(192);
    /// Fixed output for `recovery_roughtime_description`.
    pub(super) const RECOVERY_ROUGHTIME_DESCRIPTION: Self = Self(193);
    /// Fixed output for `recovery_no_effect`.
    pub(super) const RECOVERY_NO_EFFECT: Self = Self(194);
    /// Fixed output for `recovery_unavailable`.
    pub(super) const RECOVERY_UNAVAILABLE: Self = Self(195);
    /// Fixed output for `recovery_signatures`.
    pub(super) const RECOVERY_SIGNATURES: Self = Self(196);
    /// Fixed output for `recovery_date`.
    pub(super) const RECOVERY_DATE: Self = Self(197);
    /// Fixed output for `recovery_handoff`.
    pub(super) const RECOVERY_HANDOFF: Self = Self(198);
    /// Fixed output for `recovery_unobserved`.
    pub(super) const RECOVERY_UNOBSERVED: Self = Self(199);
    /// Fixed output for `recovery_no_date`.
    pub(super) const RECOVERY_NO_DATE: Self = Self(200);
    /// Fixed output for `recovery_show_path`.
    pub(super) const RECOVERY_SHOW_PATH: Self = Self(201);
    /// Fixed output for `recovery_hide_path`.
    pub(super) const RECOVERY_HIDE_PATH: Self = Self(202);
    /// Fixed output for `recovery_retained_step`.
    pub(super) const RECOVERY_RETAINED_STEP: Self = Self(203);
    /// Fixed output for `recovery_date_step`.
    pub(super) const RECOVERY_DATE_STEP: Self = Self(204);
    /// Fixed output for `recovery_nts_step`.
    pub(super) const RECOVERY_NTS_STEP: Self = Self(205);
    /// Fixed output for `recovery_agreement_step`.
    pub(super) const RECOVERY_AGREEMENT_STEP: Self = Self(206);
    /// Fixed output for `recovery_authority_step`.
    pub(super) const RECOVERY_AUTHORITY_STEP: Self = Self(207);
    /// Fixed output for `recovery_helpers_title`.
    pub(super) const RECOVERY_HELPERS_TITLE: Self = Self(208);
    /// Fixed output for `recovery_helpers_description`.
    pub(super) const RECOVERY_HELPERS_DESCRIPTION: Self = Self(209);
    /// Fixed output for `recovery_one_helper_warning`.
    pub(super) const RECOVERY_ONE_HELPER_WARNING: Self = Self(210);
    /// Fixed output for `recovery_two_helper_warning`.
    pub(super) const RECOVERY_TWO_HELPER_WARNING: Self = Self(211);
    /// Fixed output for `recovery_count_label`.
    pub(super) const RECOVERY_COUNT_LABEL: Self = Self(212);
    /// Fixed output for `recovery_counts_0`.
    pub(super) const RECOVERY_COUNTS_0: Self = Self(213);
    /// Fixed output for `recovery_counts_1`.
    pub(super) const RECOVERY_COUNTS_1: Self = Self(214);
    /// Fixed output for `recovery_counts_2`.
    pub(super) const RECOVERY_COUNTS_2: Self = Self(215);
    /// Fixed output for `recovery_first_label`.
    pub(super) const RECOVERY_FIRST_LABEL: Self = Self(216);
    /// Fixed output for `recovery_second_label`.
    pub(super) const RECOVERY_SECOND_LABEL: Self = Self(217);
    /// Fixed output for `recovery_host_help`.
    pub(super) const RECOVERY_HOST_HELP: Self = Self(218);
    /// Fixed output for `recovery_preview`.
    pub(super) const RECOVERY_PREVIEW: Self = Self(219);
    /// Fixed output for `recovery_cancel`.
    pub(super) const RECOVERY_CANCEL: Self = Self(220);
    /// Fixed output for `recovery_review_title`.
    pub(super) const RECOVERY_REVIEW_TITLE: Self = Self(221);
    /// Fixed output for `recovery_off`.
    pub(super) const RECOVERY_OFF: Self = Self(222);
    /// Fixed output for `recovery_boundary`.
    pub(super) const RECOVERY_BOUNDARY: Self = Self(223);
    /// Fixed output for `recovery_keep_editing`.
    pub(super) const RECOVERY_KEEP_EDITING: Self = Self(224);
    /// Fixed output for `recovery_discard`.
    pub(super) const RECOVERY_DISCARD: Self = Self(225);
    /// Fixed output for `recovery_apply`.
    pub(super) const RECOVERY_APPLY: Self = Self(226);
    /// Fixed output for `recovery_apply_unavailable`.
    pub(super) const RECOVERY_APPLY_UNAVAILABLE: Self = Self(227);
    /// Fixed output for `shell_quit_ui`.
    pub(super) const SHELL_QUIT_UI: Self = Self(228);
    /// Fixed output for `shell_quit_title`.
    pub(super) const SHELL_QUIT_TITLE: Self = Self(229);
    /// Fixed output for `shell_keep_open`.
    pub(super) const SHELL_KEEP_OPEN: Self = Self(230);
    /// Fixed output for `shell_manage_service`.
    pub(super) const SHELL_MANAGE_SERVICE: Self = Self(231);
    /// Fixed output for `shell_quit_only`.
    pub(super) const SHELL_QUIT_ONLY: Self = Self(232);
    /// Fixed output for `shell_unknown_consequence`.
    pub(super) const SHELL_UNKNOWN_CONSEQUENCE: Self = Self(233);
    /// Fixed output for `shell_draft_loss_title`.
    pub(super) const SHELL_DRAFT_LOSS_TITLE: Self = Self(234);
    /// Fixed output for `shell_draft_loss`.
    pub(super) const SHELL_DRAFT_LOSS: Self = Self(235);
    /// Fixed output for `shell_observer_consequence`.
    pub(super) const SHELL_OBSERVER_CONSEQUENCE: Self = Self(236);
    /// Fixed output for `shell_stop_path`.
    pub(super) const SHELL_STOP_PATH: Self = Self(237);
    /// Fixed output for `shell_disable_path`.
    pub(super) const SHELL_DISABLE_PATH: Self = Self(238);
    /// Fixed output for `shell_separate_management`.
    pub(super) const SHELL_SEPARATE_MANAGEMENT: Self = Self(239);
    /// Fixed output for `shell_exit_requested`.
    pub(super) const SHELL_EXIT_REQUESTED: Self = Self(240);
    /// Fixed output for `shell_background_service`.
    pub(super) const SHELL_BACKGROUND_SERVICE: Self = Self(241);
    /// Fixed output for `shell_notifications`.
    pub(super) const SHELL_NOTIFICATIONS: Self = Self(242);
    /// Fixed output for `shell_main_content`.
    pub(super) const SHELL_MAIN_CONTENT: Self = Self(243);
    /// Fixed output for `shell_service_description`.
    pub(super) const SHELL_SERVICE_DESCRIPTION: Self = Self(244);
    /// Fixed output for `shell_service_summary`.
    pub(super) const SHELL_SERVICE_SUMMARY: Self = Self(245);
    /// Fixed output for `shell_manage_power`.
    pub(super) const SHELL_MANAGE_POWER: Self = Self(246);
    /// Fixed output for `shell_service_unavailable`.
    pub(super) const SHELL_SERVICE_UNAVAILABLE: Self = Self(247);
    /// Fixed output for `shell_diagnose`.
    pub(super) const SHELL_DIAGNOSE: Self = Self(248);
    /// Fixed output for `shell_diagnose_power`.
    pub(super) const SHELL_DIAGNOSE_POWER: Self = Self(249);
    /// Fixed output for `shell_diagnose_note`.
    pub(super) const SHELL_DIAGNOSE_NOTE: Self = Self(250);
    /// Fixed output for `shell_startup_title`.
    pub(super) const SHELL_STARTUP_TITLE: Self = Self(251);
    /// Fixed output for `shell_startup_note`.
    pub(super) const SHELL_STARTUP_NOTE: Self = Self(252);
    /// Fixed output for `shell_autostart_on`.
    pub(super) const SHELL_AUTOSTART_ON: Self = Self(253);
    /// Fixed output for `shell_autostart_off`.
    pub(super) const SHELL_AUTOSTART_OFF: Self = Self(254);
    /// Fixed output for `shell_disable_note`.
    pub(super) const SHELL_DISABLE_NOTE: Self = Self(255);
    /// Fixed output for `shell_time_manager_title`.
    pub(super) const SHELL_TIME_MANAGER_TITLE: Self = Self(256);
    /// Fixed output for `shell_use_windows`.
    pub(super) const SHELL_USE_WINDOWS: Self = Self(257);
    /// Fixed output for `shell_use_nevrotick`.
    pub(super) const SHELL_USE_NEVROTICK: Self = Self(258);
    /// Fixed output for `shell_handover_note`.
    pub(super) const SHELL_HANDOVER_NOTE: Self = Self(259);
    /// Fixed output for `shell_synchronization_settings`.
    pub(super) const SHELL_SYNCHRONIZATION_SETTINGS: Self = Self(260);
    /// Fixed output for `shell_time_policy`.
    pub(super) const SHELL_TIME_POLICY: Self = Self(261);
    /// Fixed output for `shell_time_policy_description`.
    pub(super) const SHELL_TIME_POLICY_DESCRIPTION: Self = Self(262);
    /// Fixed output for `shell_time_policy_power`.
    pub(super) const SHELL_TIME_POLICY_POWER: Self = Self(263);
    /// Fixed output for `shell_personal_preferences`.
    pub(super) const SHELL_PERSONAL_PREFERENCES: Self = Self(264);
    /// Fixed output for `shell_personal_preferences_description`.
    pub(super) const SHELL_PERSONAL_PREFERENCES_DESCRIPTION: Self = Self(265);
    /// Fixed output for `policy_heading`.
    pub(super) const POLICY_HEADING: Self = Self(266);
    /// Fixed output for `policy_description`.
    pub(super) const POLICY_DESCRIPTION: Self = Self(267);
    /// Fixed output for `policy_baseline`.
    pub(super) const POLICY_BASELINE: Self = Self(268);
    /// Fixed output for `policy_preview`.
    pub(super) const POLICY_PREVIEW: Self = Self(269);
    /// Fixed output for `policy_no_preview`.
    pub(super) const POLICY_NO_PREVIEW: Self = Self(270);
    /// Fixed output for `policy_nts`.
    pub(super) const POLICY_NTS: Self = Self(271);
    /// Fixed output for `policy_ntp`.
    pub(super) const POLICY_NTP: Self = Self(272);
    /// Fixed output for `policy_observe`.
    pub(super) const POLICY_OBSERVE: Self = Self(273);
    /// Fixed output for `policy_choose`.
    pub(super) const POLICY_CHOOSE: Self = Self(274);
    /// Fixed output for `policy_edit`.
    pub(super) const POLICY_EDIT: Self = Self(275);
    /// Fixed output for `policy_review`.
    pub(super) const POLICY_REVIEW: Self = Self(276);
    /// Fixed output for `policy_keep`.
    pub(super) const POLICY_KEEP: Self = Self(277);
    /// Fixed output for `policy_discard`.
    pub(super) const POLICY_DISCARD: Self = Self(278);
    /// Fixed output for `policy_cancel`.
    pub(super) const POLICY_CANCEL: Self = Self(279);
    /// Fixed output for `policy_choice_label`.
    pub(super) const POLICY_CHOICE_LABEL: Self = Self(280);
    /// Fixed output for `policy_nts_consequence`.
    pub(super) const POLICY_NTS_CONSEQUENCE: Self = Self(281);
    /// Fixed output for `policy_ntp_consequence`.
    pub(super) const POLICY_NTP_CONSEQUENCE: Self = Self(282);
    /// Fixed output for `policy_observe_consequence`.
    pub(super) const POLICY_OBSERVE_CONSEQUENCE: Self = Self(283);
    /// Fixed output for `policy_authority`.
    pub(super) const POLICY_AUTHORITY: Self = Self(284);
    /// Fixed output for `policy_apply`.
    pub(super) const POLICY_APPLY: Self = Self(285);
    /// Fixed output for `policy_apply_unavailable`.
    pub(super) const POLICY_APPLY_UNAVAILABLE: Self = Self(286);
    /// Fixed output for `policy_fixture_note`.
    pub(super) const POLICY_FIXTURE_NOTE: Self = Self(287);
    /// Fixed output for `policy_preview_note`.
    pub(super) const POLICY_PREVIEW_NOTE: Self = Self(288);
    /// Fixed output for `policy_phase_required`.
    pub(super) const POLICY_PHASE_REQUIRED: Self = Self(289);
    /// Fixed output for `policy_invalid_choice`.
    pub(super) const POLICY_INVALID_CHOICE: Self = Self(290);
    /// Fixed output for `policy_no_preview_error`.
    pub(super) const POLICY_NO_PREVIEW_ERROR: Self = Self(291);
    /// Fixed output for `policy_page_required`.
    pub(super) const POLICY_PAGE_REQUIRED: Self = Self(292);
    /// Fixed output for `policy_editor_active`.
    pub(super) const POLICY_EDITOR_ACTIVE: Self = Self(293);
    /// Fixed output for `inspection_query_label`.
    pub(super) const INSPECTION_QUERY_LABEL: Self = Self(294);
    /// Fixed output for `inspection_query_placeholder`.
    pub(super) const INSPECTION_QUERY_PLACEHOLDER: Self = Self(295);
    /// Fixed output for `inspection_query_help`.
    pub(super) const INSPECTION_QUERY_HELP: Self = Self(296);
    /// Fixed output for `inspection_search`.
    pub(super) const INSPECTION_SEARCH: Self = Self(297);
    /// Fixed output for `inspection_clear`.
    pub(super) const INSPECTION_CLEAR: Self = Self(298);
    /// Fixed output for `inspection_all`.
    pub(super) const INSPECTION_ALL: Self = Self(299);
    /// Fixed output for `inspection_used`.
    pub(super) const INSPECTION_USED: Self = Self(300);
    /// Fixed output for `inspection_other`.
    pub(super) const INSPECTION_OTHER: Self = Self(301);
    /// Fixed output for `inspection_roles_unavailable`.
    pub(super) const INSPECTION_ROLES_UNAVAILABLE: Self = Self(302);
    /// Fixed output for `inspection_no_matches_heading`.
    pub(super) const INSPECTION_NO_MATCHES_HEADING: Self = Self(303);
    /// Fixed output for `inspection_no_matches`.
    pub(super) const INSPECTION_NO_MATCHES: Self = Self(304);
    /// Fixed output for `inspection_accepted_query`.
    pub(super) const INSPECTION_ACCEPTED_QUERY: Self = Self(305);
    /// Fixed output for `inspection_details`.
    pub(super) const INSPECTION_DETAILS: Self = Self(306);
    /// Fixed output for `inspection_details_heading`.
    pub(super) const INSPECTION_DETAILS_HEADING: Self = Self(307);
    /// Fixed output for `inspection_close_details`.
    pub(super) const INSPECTION_CLOSE_DETAILS: Self = Self(308);
    /// Fixed output for `inspection_details_note`.
    pub(super) const INSPECTION_DETAILS_NOTE: Self = Self(309);
    /// Fixed output for `inspection_hidden_details`.
    pub(super) const INSPECTION_HIDDEN_DETAILS: Self = Self(310);
    /// Fixed output for `inspection_selected_hidden`.
    pub(super) const INSPECTION_SELECTED_HIDDEN: Self = Self(311);
    /// Fixed output for `inspection_identity`.
    pub(super) const INSPECTION_IDENTITY: Self = Self(312);
    /// Fixed output for `inspection_label`.
    pub(super) const INSPECTION_LABEL: Self = Self(313);
    /// Fixed output for `inspection_protocol`.
    pub(super) const INSPECTION_PROTOCOL: Self = Self(314);
    /// Fixed output for `inspection_port`.
    pub(super) const INSPECTION_PORT: Self = Self(315);
    /// Fixed output for `inspection_local_id_note`.
    pub(super) const INSPECTION_LOCAL_ID_NOTE: Self = Self(316);
    /// Fixed output for `inspection_authentication`.
    pub(super) const INSPECTION_AUTHENTICATION: Self = Self(317);
    /// Fixed output for `inspection_auth_measurement`.
    pub(super) const INSPECTION_AUTH_MEASUREMENT: Self = Self(318);
    /// Fixed output for `inspection_role`.
    pub(super) const INSPECTION_ROLE: Self = Self(319);
    /// Fixed output for `inspection_operator`.
    pub(super) const INSPECTION_OPERATOR: Self = Self(320);
    /// Fixed output for `inspection_endpoint`.
    pub(super) const INSPECTION_ENDPOINT: Self = Self(321);
    /// Fixed output for `inspection_vote_identity`.
    pub(super) const INSPECTION_VOTE_IDENTITY: Self = Self(322);
    /// Fixed output for `inspection_request_status`.
    pub(super) const INSPECTION_REQUEST_STATUS: Self = Self(323);
    /// Fixed output for `inspection_last_reply`.
    pub(super) const INSPECTION_LAST_REPLY: Self = Self(324);
    /// Fixed output for `inspection_last_accepted`.
    pub(super) const INSPECTION_LAST_ACCEPTED: Self = Self(325);
    /// Fixed output for `inspection_offset`.
    pub(super) const INSPECTION_OFFSET: Self = Self(326);
    /// Fixed output for `inspection_delay`.
    pub(super) const INSPECTION_DELAY: Self = Self(327);
    /// Fixed output for `inspection_uncertainty`.
    pub(super) const INSPECTION_UNCERTAINTY: Self = Self(328);
    /// Fixed output for `inspection_certificate`.
    pub(super) const INSPECTION_CERTIFICATE: Self = Self(329);
    /// Fixed output for `inspection_scale`.
    pub(super) const INSPECTION_SCALE: Self = Self(330);
    /// Fixed output for `inspection_leap`.
    pub(super) const INSPECTION_LEAP: Self = Self(331);
    /// Fixed output for `inspection_not_observed`.
    pub(super) const INSPECTION_NOT_OBSERVED: Self = Self(332);
    /// Fixed output for `inspection_not_measured`.
    pub(super) const INSPECTION_NOT_MEASURED: Self = Self(333);
    /// Fixed output for `inspection_not_established`.
    pub(super) const INSPECTION_NOT_ESTABLISHED: Self = Self(334);
    /// Fixed output for `inspection_not_evaluated`.
    pub(super) const INSPECTION_NOT_EVALUATED: Self = Self(335);
    /// Fixed output for `inspection_not_applicable`.
    pub(super) const INSPECTION_NOT_APPLICABLE: Self = Self(336);
    /// Fixed output for `inspection_unauthenticated`.
    pub(super) const INSPECTION_UNAUTHENTICATED: Self = Self(337);
    /// Fixed output for `inspection_nts`.
    pub(super) const INSPECTION_NTS: Self = Self(338);
    /// Fixed output for `inspection_ntp`.
    pub(super) const INSPECTION_NTP: Self = Self(339);
    /// Fixed output for `inspection_invalid_query`.
    pub(super) const INSPECTION_INVALID_QUERY: Self = Self(340);
    /// Fixed output for `inspection_invalid_filter`.
    pub(super) const INSPECTION_INVALID_FILTER: Self = Self(341);
    /// Fixed output for `inspection_hidden_source`.
    pub(super) const INSPECTION_HIDDEN_SOURCE: Self = Self(342);
    /// Fixed output for `inspection_page_required`.
    pub(super) const INSPECTION_PAGE_REQUIRED: Self = Self(343);
    /// Fixed output for `inspection_editor_active`.
    pub(super) const INSPECTION_EDITOR_ACTIVE: Self = Self(344);
    /// Fixed output for `diagnostic_back`.
    pub(super) const DIAGNOSTIC_BACK: Self = Self(345);
    /// Fixed output for `diagnostic_problem`.
    pub(super) const DIAGNOSTIC_PROBLEM: Self = Self(346);
    /// Fixed output for `diagnostic_effect`.
    pub(super) const DIAGNOSTIC_EFFECT: Self = Self(347);
    /// Fixed output for `diagnostic_remedy`.
    pub(super) const DIAGNOSTIC_REMEDY: Self = Self(348);
    /// Fixed output for `diagnostic_service`.
    pub(super) const DIAGNOSTIC_SERVICE: Self = Self(349);
    /// Fixed output for `diagnostic_events_empty`.
    pub(super) const DIAGNOSTIC_EVENTS_EMPTY: Self = Self(350);
    /// Fixed output for `diagnostic_events_note`.
    pub(super) const DIAGNOSTIC_EVENTS_NOTE: Self = Self(351);
    /// Fixed output for `diagnostic_report_open`.
    pub(super) const DIAGNOSTIC_REPORT_OPEN: Self = Self(352);
    /// Fixed output for `diagnostic_report_title`.
    pub(super) const DIAGNOSTIC_REPORT_TITLE: Self = Self(353);
    /// Fixed output for `diagnostic_report_note`.
    pub(super) const DIAGNOSTIC_REPORT_NOTE: Self = Self(354);
    /// Fixed output for `diagnostic_copy`.
    pub(super) const DIAGNOSTIC_COPY: Self = Self(355);
    /// Fixed output for `diagnostic_export`.
    pub(super) const DIAGNOSTIC_EXPORT: Self = Self(356);
    /// Fixed output for `diagnostic_export_reason`.
    pub(super) const DIAGNOSTIC_EXPORT_REASON: Self = Self(357);
    /// Fixed output for `diagnostic_collection_open`.
    pub(super) const DIAGNOSTIC_COLLECTION_OPEN: Self = Self(358);
    /// Fixed output for `diagnostic_collection_title`.
    pub(super) const DIAGNOSTIC_COLLECTION_TITLE: Self = Self(359);
    /// Fixed output for `diagnostic_collection_note`.
    pub(super) const DIAGNOSTIC_COLLECTION_NOTE: Self = Self(360);
    /// Fixed output for `diagnostic_scope`.
    pub(super) const DIAGNOSTIC_SCOPE: Self = Self(361);
    /// Fixed output for `diagnostic_scopes_0`.
    pub(super) const DIAGNOSTIC_SCOPES_0: Self = Self(362);
    /// Fixed output for `diagnostic_scopes_1`.
    pub(super) const DIAGNOSTIC_SCOPES_1: Self = Self(363);
    /// Fixed output for `diagnostic_scopes_2`.
    pub(super) const DIAGNOSTIC_SCOPES_2: Self = Self(364);
    /// Fixed output for `diagnostic_duration`.
    pub(super) const DIAGNOSTIC_DURATION: Self = Self(365);
    /// Fixed output for `diagnostic_collection_state`.
    pub(super) const DIAGNOSTIC_COLLECTION_STATE: Self = Self(366);
    /// Fixed output for `diagnostic_effective_scope`.
    pub(super) const DIAGNOSTIC_EFFECTIVE_SCOPE: Self = Self(367);
    /// Fixed output for `diagnostic_remaining`.
    pub(super) const DIAGNOSTIC_REMAINING: Self = Self(368);
    /// Fixed output for `diagnostic_unavailable`.
    pub(super) const DIAGNOSTIC_UNAVAILABLE: Self = Self(369);
    /// Fixed output for `diagnostic_collection_reason`.
    pub(super) const DIAGNOSTIC_COLLECTION_REASON: Self = Self(370);
    /// Fixed output for `diagnostic_required_events`.
    pub(super) const DIAGNOSTIC_REQUIRED_EVENTS: Self = Self(371);
    /// Fixed output for `diagnostic_start`.
    pub(super) const DIAGNOSTIC_START: Self = Self(372);
    /// Fixed output for `diagnostic_stop`.
    pub(super) const DIAGNOSTIC_STOP: Self = Self(373);
    /// Fixed output for `diagnostic_correction_open`.
    pub(super) const DIAGNOSTIC_CORRECTION_OPEN: Self = Self(374);
    /// Fixed output for `diagnostic_correction_title`.
    pub(super) const DIAGNOSTIC_CORRECTION_TITLE: Self = Self(375);
    /// Fixed output for `diagnostic_correction_empty`.
    pub(super) const DIAGNOSTIC_CORRECTION_EMPTY: Self = Self(376);
    /// Fixed output for `diagnostic_correction_note`.
    pub(super) const DIAGNOSTIC_CORRECTION_NOTE: Self = Self(377);
    /// Fixed output for `diagnostic_correction_rule`.
    pub(super) const DIAGNOSTIC_CORRECTION_RULE: Self = Self(378);
    /// Fixed output for `diagnostic_approve`.
    pub(super) const DIAGNOSTIC_APPROVE: Self = Self(379);
    /// Fixed output for `diagnostic_unavailable_action`.
    pub(super) const DIAGNOSTIC_UNAVAILABLE_ACTION: Self = Self(380);
    /// Fixed output for `diagnostic_invalid_choice`.
    pub(super) const DIAGNOSTIC_INVALID_CHOICE: Self = Self(381);
    /// Fixed output for `diagnostic_durations_0`.
    pub(super) const DIAGNOSTIC_DURATIONS_0: Self = Self(382);
    /// Fixed output for `diagnostic_durations_1`.
    pub(super) const DIAGNOSTIC_DURATIONS_1: Self = Self(383);
    /// Fixed output for `diagnostic_durations_2`.
    pub(super) const DIAGNOSTIC_DURATIONS_2: Self = Self(384);
    /// Fixed output for `raw_too_long`.
    pub(super) const RAW_TOO_LONG: Self = Self(385);
    /// Fixed output for `raw_character`.
    pub(super) const RAW_CHARACTER: Self = Self(386);
    /// Fixed output for `raw_invalid_option`.
    pub(super) const RAW_INVALID_OPTION: Self = Self(387);
    /// Fixed output for `raw_context`.
    pub(super) const RAW_CONTEXT: Self = Self(388);
    /// Fixed output for `raw_unavailable`.
    pub(super) const RAW_UNAVAILABLE: Self = Self(389);
    /// Fixed output for `raw_stale`.
    pub(super) const RAW_STALE: Self = Self(390);
    /// Fixed output for `raw_action_undo`.
    pub(super) const RAW_ACTION_UNDO: Self = Self(391);
    /// Fixed output for `raw_action_redo`.
    pub(super) const RAW_ACTION_REDO: Self = Self(392);
    /// Fixed output for `raw_action_cut`.
    pub(super) const RAW_ACTION_CUT: Self = Self(393);
    /// Fixed output for `raw_action_copy`.
    pub(super) const RAW_ACTION_COPY: Self = Self(394);
    /// Fixed output for `raw_action_paste`.
    pub(super) const RAW_ACTION_PASTE: Self = Self(395);
    /// Fixed output for `raw_action_select_all`.
    pub(super) const RAW_ACTION_SELECT_ALL: Self = Self(396);
    /// Typed feedback output for editor-error.
    pub(super) const FEEDBACK_EDITOR_ERROR: Self = Self(397);
    /// Typed feedback output for editor-feedback.
    pub(super) const FEEDBACK_EDITOR_FEEDBACK: Self = Self(398);
    /// Typed feedback output for interval-error.
    pub(super) const FEEDBACK_INTERVAL_ERROR: Self = Self(399);
    /// Typed feedback output for interval-feedback.
    pub(super) const FEEDBACK_INTERVAL_FEEDBACK: Self = Self(400);
    /// Typed feedback output for preference-error.
    pub(super) const FEEDBACK_PREFERENCE_ERROR: Self = Self(401);
    /// Typed feedback output for preference-feedback.
    pub(super) const FEEDBACK_PREFERENCE_FEEDBACK: Self = Self(402);
    /// Typed feedback output for recovery-error.
    pub(super) const FEEDBACK_RECOVERY_ERROR: Self = Self(403);
    /// Typed feedback output for recovery-feedback.
    pub(super) const FEEDBACK_RECOVERY_FEEDBACK: Self = Self(404);
    /// Typed feedback output for recovery-back-label.
    pub(super) const FEEDBACK_RECOVERY_BACK_LABEL: Self = Self(405);
    /// Typed feedback output for quit-headline.
    pub(super) const FEEDBACK_QUIT_HEADLINE: Self = Self(406);
    /// Typed feedback output for quit-consequence.
    pub(super) const FEEDBACK_QUIT_CONSEQUENCE: Self = Self(407);
    /// Typed feedback output for shell-feedback.
    pub(super) const FEEDBACK_SHELL_FEEDBACK: Self = Self(408);
    /// Typed feedback output for service-feedback.
    pub(super) const FEEDBACK_SERVICE_FEEDBACK: Self = Self(409);
    /// Typed feedback output for notification-feedback.
    pub(super) const FEEDBACK_NOTIFICATION_FEEDBACK: Self = Self(410);
    /// Typed feedback output for diagnostic-feedback.
    pub(super) const FEEDBACK_DIAGNOSTIC_FEEDBACK: Self = Self(411);
    /// Typed feedback output for policy-error.
    pub(super) const FEEDBACK_POLICY_ERROR: Self = Self(412);
    /// Typed feedback output for source-query-error.
    pub(super) const FEEDBACK_SOURCE_QUERY_ERROR: Self = Self(413);
    /// Typed feedback output for accepted-interval-label.
    pub(super) const FEEDBACK_ACCEPTED_INTERVAL_LABEL: Self = Self(414);
    /// Typed feedback output for status-feedback.
    pub(super) const FEEDBACK_STATUS_FEEDBACK: Self = Self(415);
    /// Typed feedback output for tray-feedback.
    pub(super) const FEEDBACK_TRAY_FEEDBACK: Self = Self(416);
    /// Typed feedback output for status-headline.
    pub(super) const FEEDBACK_STATUS_HEADLINE: Self = Self(417);
    /// Typed feedback output for status-consequence.
    pub(super) const FEEDBACK_STATUS_CONSEQUENCE: Self = Self(418);
    /// Typed feedback output for status-profile-label.
    pub(super) const FEEDBACK_STATUS_PROFILE_LABEL: Self = Self(419);
    /// Typed feedback output for status-profile.
    pub(super) const FEEDBACK_STATUS_PROFILE: Self = Self(420);
    /// Typed feedback output for status-authentication.
    pub(super) const FEEDBACK_STATUS_AUTHENTICATION: Self = Self(421);
    /// Typed feedback output for status-agreement.
    pub(super) const FEEDBACK_STATUS_AGREEMENT: Self = Self(422);
    /// Typed feedback output for status-freshness.
    pub(super) const FEEDBACK_STATUS_FRESHNESS: Self = Self(423);
    /// Typed feedback output for status-check-label.
    pub(super) const FEEDBACK_STATUS_CHECK_LABEL: Self = Self(424);
    /// Typed feedback output for status-remedy-label.
    pub(super) const FEEDBACK_STATUS_REMEDY_LABEL: Self = Self(425);
}

/// Closed typed-feedback order, independent of translated words.
const FEEDBACK_SLOTS: &[(FeedbackSlot, CopySlot)] = &[
    (FeedbackSlot::EditorError, CopySlot::FEEDBACK_EDITOR_ERROR),
    (
        FeedbackSlot::EditorFeedback,
        CopySlot::FEEDBACK_EDITOR_FEEDBACK,
    ),
    (
        FeedbackSlot::IntervalError,
        CopySlot::FEEDBACK_INTERVAL_ERROR,
    ),
    (
        FeedbackSlot::IntervalFeedback,
        CopySlot::FEEDBACK_INTERVAL_FEEDBACK,
    ),
    (
        FeedbackSlot::PreferenceError,
        CopySlot::FEEDBACK_PREFERENCE_ERROR,
    ),
    (
        FeedbackSlot::PreferenceFeedback,
        CopySlot::FEEDBACK_PREFERENCE_FEEDBACK,
    ),
    (
        FeedbackSlot::RecoveryError,
        CopySlot::FEEDBACK_RECOVERY_ERROR,
    ),
    (
        FeedbackSlot::RecoveryFeedback,
        CopySlot::FEEDBACK_RECOVERY_FEEDBACK,
    ),
    (
        FeedbackSlot::RecoveryBackLabel,
        CopySlot::FEEDBACK_RECOVERY_BACK_LABEL,
    ),
    (FeedbackSlot::QuitHeadline, CopySlot::FEEDBACK_QUIT_HEADLINE),
    (
        FeedbackSlot::QuitConsequence,
        CopySlot::FEEDBACK_QUIT_CONSEQUENCE,
    ),
    (
        FeedbackSlot::ShellFeedback,
        CopySlot::FEEDBACK_SHELL_FEEDBACK,
    ),
    (
        FeedbackSlot::ServiceFeedback,
        CopySlot::FEEDBACK_SERVICE_FEEDBACK,
    ),
    (
        FeedbackSlot::NotificationFeedback,
        CopySlot::FEEDBACK_NOTIFICATION_FEEDBACK,
    ),
    (
        FeedbackSlot::DiagnosticFeedback,
        CopySlot::FEEDBACK_DIAGNOSTIC_FEEDBACK,
    ),
    (FeedbackSlot::PolicyError, CopySlot::FEEDBACK_POLICY_ERROR),
    (
        FeedbackSlot::SourceQueryError,
        CopySlot::FEEDBACK_SOURCE_QUERY_ERROR,
    ),
    (
        FeedbackSlot::AcceptedIntervalLabel,
        CopySlot::FEEDBACK_ACCEPTED_INTERVAL_LABEL,
    ),
    (
        FeedbackSlot::StatusFeedback,
        CopySlot::FEEDBACK_STATUS_FEEDBACK,
    ),
    (FeedbackSlot::TrayFeedback, CopySlot::FEEDBACK_TRAY_FEEDBACK),
    (
        FeedbackSlot::StatusHeadline,
        CopySlot::FEEDBACK_STATUS_HEADLINE,
    ),
    (
        FeedbackSlot::StatusConsequence,
        CopySlot::FEEDBACK_STATUS_CONSEQUENCE,
    ),
    (
        FeedbackSlot::StatusProfileLabel,
        CopySlot::FEEDBACK_STATUS_PROFILE_LABEL,
    ),
    (
        FeedbackSlot::StatusProfile,
        CopySlot::FEEDBACK_STATUS_PROFILE,
    ),
    (
        FeedbackSlot::StatusAuthentication,
        CopySlot::FEEDBACK_STATUS_AUTHENTICATION,
    ),
    (
        FeedbackSlot::StatusAgreement,
        CopySlot::FEEDBACK_STATUS_AGREEMENT,
    ),
    (
        FeedbackSlot::StatusFreshness,
        CopySlot::FEEDBACK_STATUS_FRESHNESS,
    ),
    (
        FeedbackSlot::StatusCheckLabel,
        CopySlot::FEEDBACK_STATUS_CHECK_LABEL,
    ),
    (
        FeedbackSlot::StatusRemedyLabel,
        CopySlot::FEEDBACK_STATUS_REMEDY_LABEL,
    ),
];

/// Finish one complete private model; no property setter is called here.
///
/// # Errors
/// Rejects wrong shape or aggregate output before any model can be published.
fn finish(revision: SharedString, rows: Vec<SharedString>) -> Result<UiCopy, Error> {
    if rows.len() != OUTPUT_SLOT_COUNT {
        return Err(Error::Schema);
    }
    let value = UiCopy {
        catalog_ready: true,
        revision,
        texts: ModelRc::new(ImmutableCopyModel {
            rows: rows.into_boxed_slice(),
        }),
    };
    check(&value)?;
    Ok(value)
}

/// Retain only the next code-owned output position.
///
/// # Errors
/// Rejects a programming mismatch before it can produce a published row table.
fn push(rows: &mut Vec<SharedString>, slot: CopySlot, text: SharedString) -> Result<(), Error> {
    if rows.len() != slot.index() {
        return Err(Error::Schema);
    }
    rows.push(text);
    Ok(())
}

/// Append every closed feedback slot through the same complete output budget.
///
/// # Errors
/// Rejects aggregate exhaustion before retaining another toolkit string.
fn append_feedback(
    rows: &mut Vec<SharedString>,
    budget: &mut OutputBudget,
    feedback: &PreparedFeedback,
) -> Result<(), Error> {
    for (slot, output) in FEEDBACK_SLOTS {
        let value = match feedback.get(*slot) {
            Some(text) => budget.retain(text)?,
            None => SharedString::default(),
        };
        push(rows, *output, value)?;
    }
    Ok(())
}

/// Prepare one immutable fixed-slot generation before the workflow commits it.
///
/// # Errors
/// Rejects any argument, model shape or complete-output failure without publication.
pub(super) fn prepare(
    snapshot: &Snapshot<'_>,
    feedback: &PreparedFeedback,
) -> Result<UiCopy, Error> {
    let revision: SharedString = snapshot.revision().to_string().into();
    let mut budget = OutputBudget::default();
    budget.charge(&revision)?;
    let mut copy = CopyPreparation { snapshot, budget };
    let mut rows = Vec::with_capacity(OUTPUT_SLOT_COUNT);
    fill_1(&mut copy, &mut rows)?;
    fill_2(&mut copy, &mut rows)?;
    fill_3(&mut copy, &mut rows)?;
    fill_4(&mut copy, &mut rows)?;
    fill_5(&mut copy, &mut rows)?;
    fill_6(&mut copy, &mut rows)?;
    fill_7(&mut copy, &mut rows)?;
    fill_8(&mut copy, &mut rows)?;
    fill_9(&mut copy, &mut rows)?;
    fill_10(&mut copy, &mut rows)?;
    fill_11(&mut copy, &mut rows)?;
    fill_12(&mut copy, &mut rows)?;
    fill_13(&mut copy, &mut rows)?;
    fill_14(&mut copy, &mut rows)?;
    fill_15(&mut copy, &mut rows)?;
    fill_16(&mut copy, &mut rows)?;
    fill_17(&mut copy, &mut rows)?;
    fill_18(&mut copy, &mut rows)?;
    fill_19(&mut copy, &mut rows)?;
    fill_20(&mut copy, &mut rows)?;
    fill_21(&mut copy, &mut rows)?;
    fill_22(&mut copy, &mut rows)?;
    fill_23(&mut copy, &mut rows)?;
    fill_24(&mut copy, &mut rows)?;
    fill_25(&mut copy, &mut rows)?;
    fill_26(&mut copy, &mut rows)?;
    fill_27(&mut copy, &mut rows)?;
    fill_28(&mut copy, &mut rows)?;
    fill_29(&mut copy, &mut rows)?;
    fill_30(&mut copy, &mut rows)?;
    fill_31(&mut copy, &mut rows)?;
    fill_32(&mut copy, &mut rows)?;
    fill_33(&mut copy, &mut rows)?;
    fill_34(&mut copy, &mut rows)?;
    append_feedback(&mut rows, &mut copy.budget, feedback)?;
    finish(revision, rows)
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_1(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::FIXTURE_WINDOW_TITLE,
        copy.text(MessageId::FIXTURE_WINDOW_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SAMPLE_LABEL,
        copy.text(MessageId::FIXTURE_SAMPLE_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_LOCAL_BOUNDARY,
        copy.text(MessageId::FIXTURE_LOCAL_BOUNDARY)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_MEMORY_BOUNDARY,
        copy.text(MessageId::FIXTURE_MEMORY_BOUNDARY)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SIMPLE_MODE,
        copy.text(MessageId::FIXTURE_SIMPLE_MODE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_POWER_MODE,
        copy.text(MessageId::FIXTURE_POWER_MODE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_STATUS_TAB,
        copy.text(MessageId::FIXTURE_STATUS_TAB)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SOURCES_TAB,
        copy.text(MessageId::FIXTURE_SOURCES_TAB)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_STATISTICS_TAB,
        copy.text(MessageId::FIXTURE_STATISTICS_TAB)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_DIAGNOSTICS_TAB,
        copy.text(MessageId::FIXTURE_DIAGNOSTICS_TAB)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SETTINGS_TAB,
        copy.text(MessageId::FIXTURE_SETTINGS_TAB)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_ABOUT_TAB,
        copy.text(MessageId::FIXTURE_ABOUT_TAB)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_2(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::FIXTURE_PREVIEW_BADGE,
        copy.text(MessageId::FIXTURE_PREVIEW_BADGE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_NOT_MEASURED,
        copy.text(MessageId::FIXTURE_NOT_MEASURED)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_NOT_EVALUATED,
        copy.text(MessageId::FIXTURE_NOT_EVALUATED)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_UNAVAILABLE,
        copy.text(MessageId::FIXTURE_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_NO_LIVE_CONNECTION,
        copy.text(MessageId::FIXTURE_NO_LIVE_CONNECTION)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_STATUS_KICKER,
        copy.text(MessageId::FIXTURE_STATUS_KICKER)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_STATUS_HEADING,
        copy.text(MessageId::FIXTURE_STATUS_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_STATUS_QUALIFIER,
        copy.text(MessageId::FIXTURE_STATUS_QUALIFIER)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_DIFFERENCE_LABEL,
        copy.text(MessageId::FIXTURE_DIFFERENCE_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_DIFFERENCE_NOTE,
        copy.text(MessageId::FIXTURE_DIFFERENCE_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_UNCERTAINTY_LABEL,
        copy.text(MessageId::FIXTURE_UNCERTAINTY_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_UNCERTAINTY_NOTE,
        copy.text(MessageId::FIXTURE_UNCERTAINTY_NOTE)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_3(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::FIXTURE_AUTHENTICATION_LABEL,
        copy.text(MessageId::FIXTURE_AUTHENTICATION_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_AUTHENTICATION_VALUE,
        copy.text(MessageId::FIXTURE_AUTHENTICATION_VALUE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_AUTHENTICATION_NOTE,
        copy.text(MessageId::FIXTURE_AUTHENTICATION_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_AGREEMENT_LABEL,
        copy.text(MessageId::FIXTURE_AGREEMENT_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_AGREEMENT_NOTE,
        copy.text(MessageId::FIXTURE_AGREEMENT_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_QUALIFIED_LABEL,
        copy.text(MessageId::FIXTURE_QUALIFIED_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SYNC_NOW,
        copy.text(MessageId::FIXTURE_SYNC_NOW)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SYNC_UNAVAILABLE,
        copy.text(MessageId::FIXTURE_SYNC_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SOURCES_HEADING,
        copy.text(MessageId::FIXTURE_SOURCES_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SOURCES_DESCRIPTION,
        copy.text(MessageId::FIXTURE_SOURCES_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SOURCE_LIST_NOTE,
        copy.text(MessageId::FIXTURE_SOURCE_LIST_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_EDIT_SOURCE,
        copy.text(MessageId::FIXTURE_EDIT_SOURCE)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_4(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::FIXTURE_SELECT_SOURCE,
        copy.text(MessageId::FIXTURE_SELECT_SOURCE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SELECTED,
        copy.text(MessageId::FIXTURE_SELECTED)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SERVER_IDENTITY,
        copy.text(MessageId::FIXTURE_SERVER_IDENTITY)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_DISPLAY_NAME,
        copy.text(MessageId::FIXTURE_DISPLAY_NAME)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_NTS_PORT,
        copy.text(MessageId::FIXTURE_NTS_PORT)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_PROTOCOL_LABEL,
        copy.text(MessageId::FIXTURE_PROTOCOL_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_NTS_PREVIEW,
        copy.text(MessageId::FIXTURE_NTS_PREVIEW)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SOURCE_ROLE,
        copy.text(MessageId::FIXTURE_SOURCE_ROLE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_LAST_REPLY,
        copy.text(MessageId::FIXTURE_LAST_REPLY)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_EDITOR_DESCRIPTION,
        copy.text(MessageId::FIXTURE_EDITOR_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_LABEL_HELP,
        copy.text(MessageId::FIXTURE_LABEL_HELP)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_HOST_HELP,
        copy.text(MessageId::FIXTURE_HOST_HELP)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_5(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::FIXTURE_EDITOR_UNAVAILABLE,
        copy.text(MessageId::FIXTURE_EDITOR_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_UPDATE_PREVIEW,
        copy.text(MessageId::FIXTURE_UPDATE_PREVIEW)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_CANCEL,
        copy.text(MessageId::FIXTURE_CANCEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_STATISTICS_HEADING,
        copy.text(MessageId::FIXTURE_STATISTICS_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_STATISTICS_DESCRIPTION,
        copy.text(MessageId::FIXTURE_STATISTICS_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_STATISTICS_EMPTY_TITLE,
        copy.text(MessageId::FIXTURE_STATISTICS_EMPTY_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_ONE_SNAPSHOT,
        copy.text(MessageId::FIXTURE_ONE_SNAPSHOT)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_OFFSET_LABEL,
        copy.text(MessageId::FIXTURE_OFFSET_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_POSITIVE_OFFSET_NOTE,
        copy.text(MessageId::FIXTURE_POSITIVE_OFFSET_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_DIAGNOSTICS_HEADING,
        copy.text(MessageId::FIXTURE_DIAGNOSTICS_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_DIAGNOSTICS_DESCRIPTION,
        copy.text(MessageId::FIXTURE_DIAGNOSTICS_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_OBSERVATION_DESCRIPTION,
        copy.text(MessageId::FIXTURE_OBSERVATION_DESCRIPTION)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_6(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::FIXTURE_FRESHNESS_HEADING,
        copy.text(MessageId::FIXTURE_FRESHNESS_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_FRESHNESS_NOTE,
        copy.text(MessageId::FIXTURE_FRESHNESS_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_APPLIED_LABEL,
        copy.text(MessageId::FIXTURE_APPLIED_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_NO_CORRECTION,
        copy.text(MessageId::FIXTURE_NO_CORRECTION)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SNAPSHOT_LABEL,
        copy.text(MessageId::FIXTURE_SNAPSHOT_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_FIXTURE_SNAPSHOT,
        copy.text(MessageId::FIXTURE_FIXTURE_SNAPSHOT)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_DIAGNOSTICS_EVENTS,
        copy.text(MessageId::FIXTURE_DIAGNOSTICS_EVENTS)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SETTINGS_DESCRIPTION,
        copy.text(MessageId::FIXTURE_SETTINGS_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SCHEDULE_HEADING,
        copy.text(MessageId::FIXTURE_SCHEDULE_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_INTERVAL_LABEL,
        copy.text(MessageId::FIXTURE_INTERVAL_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SERVICE_HEADING,
        copy.text(MessageId::FIXTURE_SERVICE_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SERVICE_DESCRIPTION,
        copy.text(MessageId::FIXTURE_SERVICE_DESCRIPTION)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_7(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::FIXTURE_RUNTIME_LABEL,
        copy.text(MessageId::FIXTURE_RUNTIME_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_RUNTIME_VALUE,
        copy.text(MessageId::FIXTURE_RUNTIME_VALUE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_STARTUP_LABEL,
        copy.text(MessageId::FIXTURE_STARTUP_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_STARTUP_VALUE,
        copy.text(MessageId::FIXTURE_STARTUP_VALUE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_OWNER_LABEL,
        copy.text(MessageId::FIXTURE_OWNER_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_OWNER_VALUE,
        copy.text(MessageId::FIXTURE_OWNER_VALUE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SYNCHRONIZATION_LABEL,
        copy.text(MessageId::FIXTURE_SYNCHRONIZATION_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SYNCHRONIZATION_VALUE,
        copy.text(MessageId::FIXTURE_SYNCHRONIZATION_VALUE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_ADMIN_REQUIRED,
        copy.text(MessageId::FIXTURE_ADMIN_REQUIRED)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_SERVICE_PROTECTED_NOTE,
        copy.text(MessageId::FIXTURE_SERVICE_PROTECTED_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_START_SERVICE,
        copy.text(MessageId::FIXTURE_START_SERVICE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_STOP_SERVICE,
        copy.text(MessageId::FIXTURE_STOP_SERVICE)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_8(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::FIXTURE_RESTART_SERVICE,
        copy.text(MessageId::FIXTURE_RESTART_SERVICE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_DISABLE_SERVICE,
        copy.text(MessageId::FIXTURE_DISABLE_SERVICE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_LIFECYCLE_NOTE,
        copy.text(MessageId::FIXTURE_LIFECYCLE_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_ABOUT_HEADING,
        copy.text(MessageId::FIXTURE_ABOUT_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_ABOUT_TAGLINE,
        copy.text(MessageId::FIXTURE_ABOUT_TAGLINE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_ABOUT_TITLE,
        copy.text(MessageId::FIXTURE_ABOUT_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_ABOUT_DESCRIPTION,
        copy.text(MessageId::FIXTURE_ABOUT_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_ABOUT_STAGE,
        copy.text(MessageId::FIXTURE_ABOUT_STAGE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_AUTHOR_LABEL,
        copy.text(MessageId::FIXTURE_AUTHOR_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_REPOSITORY_LABEL,
        copy.text(MessageId::FIXTURE_REPOSITORY_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_VERSION_LABEL,
        copy.text(MessageId::FIXTURE_VERSION_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_VERSION_VALUE,
        copy.text(MessageId::FIXTURE_VERSION_VALUE)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_9(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::FIXTURE_PLATFORM_LABEL,
        copy.text(MessageId::FIXTURE_PLATFORM_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_CODE_LICENSE_LABEL,
        copy.text(MessageId::FIXTURE_CODE_LICENSE_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_TECHNOLOGIES_HEADING,
        copy.text(MessageId::FIXTURE_TECHNOLOGIES_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_TECHNOLOGIES_DESCRIPTION,
        copy.text(MessageId::FIXTURE_TECHNOLOGIES_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_CHRONY_NOTE,
        copy.text(MessageId::FIXTURE_CHRONY_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_PRIVACY_HEADING,
        copy.text(MessageId::FIXTURE_PRIVACY_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_PRIVACY_DESCRIPTION,
        copy.text(MessageId::FIXTURE_PRIVACY_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_ARTWORK_HEADING,
        copy.text(MessageId::FIXTURE_ARTWORK_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_ARTWORK_DESCRIPTION,
        copy.text(MessageId::FIXTURE_ARTWORK_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_OPEN_REPOSITORY,
        copy.text(MessageId::FIXTURE_OPEN_REPOSITORY)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_REPOSITORY_HELP,
        copy.text(MessageId::FIXTURE_REPOSITORY_HELP)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_REPOSITORY_ERROR,
        copy.text(MessageId::FIXTURE_REPOSITORY_ERROR)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_10(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::FIXTURE_GUI_BUILD_LABEL,
        copy.text(MessageId::FIXTURE_GUI_BUILD_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::FIXTURE_TEXT_CONTRACT_LABEL,
        copy.text(MessageId::FIXTURE_TEXT_CONTRACT_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_SCHEDULE_DESCRIPTION,
        copy.text(MessageId::PREFERENCES_SCHEDULE_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_SCHEDULING_NOTE,
        copy.text(MessageId::PREFERENCES_SCHEDULING_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_CUSTOM_LABEL,
        copy.text(MessageId::PREFERENCES_CUSTOM_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_CUSTOM_HELP,
        copy.text(MessageId::PREFERENCES_CUSTOM_HELP)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_ACCEPTED_INTERVAL,
        copy.text(MessageId::PREFERENCES_ACCEPTED_INTERVAL)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_PREFERENCES_BOUNDARY,
        copy.text(MessageId::PREFERENCES_PREFERENCES_BOUNDARY)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_HOST_UNAVAILABLE,
        copy.text(MessageId::PREFERENCES_HOST_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_NOTIFICATIONS_HEADING,
        copy.text(MessageId::PREFERENCES_NOTIFICATIONS_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_NOTIFICATIONS_LABEL,
        copy.text(MessageId::PREFERENCES_NOTIFICATIONS_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_NOTIFICATIONS_HELP,
        copy.text(MessageId::PREFERENCES_NOTIFICATIONS_HELP)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_11(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::PREFERENCES_WINDOWS_AVAILABILITY,
        copy.text(MessageId::PREFERENCES_WINDOWS_AVAILABILITY)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_UNKNOWN_AVAILABILITY,
        copy.text(MessageId::PREFERENCES_UNKNOWN_AVAILABILITY)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_AVAILABILITY_HELP,
        copy.text(MessageId::PREFERENCES_AVAILABILITY_HELP)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_OBSERVER_LABEL,
        copy.text(MessageId::PREFERENCES_OBSERVER_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_OBSERVER_UNAVAILABLE,
        copy.text(MessageId::PREFERENCES_OBSERVER_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_OBSERVER_HELP,
        copy.text(MessageId::PREFERENCES_OBSERVER_HELP)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_TEST_NOTIFICATION,
        copy.text(MessageId::PREFERENCES_TEST_NOTIFICATION)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_NOTIFICATION_TEST_UNAVAILABLE,
        copy.text(MessageId::PREFERENCES_NOTIFICATION_TEST_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_PREFERENCES_HEADING,
        copy.text(MessageId::PREFERENCES_PREFERENCES_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_TRAY_LABEL,
        copy.text(MessageId::PREFERENCES_TRAY_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_TRAY_HELP,
        copy.text(MessageId::PREFERENCES_TRAY_HELP)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_SIGN_IN_LABEL,
        copy.text(MessageId::PREFERENCES_SIGN_IN_LABEL)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_12(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::PREFERENCES_SIGN_IN_HELP,
        copy.text(MessageId::PREFERENCES_SIGN_IN_HELP)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_TRAY_ADAPTER_UNAVAILABLE,
        copy.text(MessageId::PREFERENCES_TRAY_ADAPTER_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_INTERVAL_OPTIONS_0,
        copy.text(MessageId::PREFERENCES_INTERVAL_OPTION_AUTOMATIC)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_INTERVAL_OPTIONS_1,
        copy.minutes(MessageId::INTERVAL_EVERY, 2)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_INTERVAL_OPTIONS_2,
        copy.minutes(MessageId::INTERVAL_EVERY, 5)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_INTERVAL_OPTIONS_3,
        copy.minutes(MessageId::INTERVAL_EVERY, 15)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_INTERVAL_OPTIONS_4,
        copy.text(MessageId::PREFERENCES_INTERVAL_OPTION_CUSTOM)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_INCIDENTS_HEADING,
        copy.text(MessageId::PREFERENCES_INCIDENTS_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_INCIDENTS_UNAVAILABLE,
        copy.text(MessageId::PREFERENCES_INCIDENTS_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_LANGUAGE_LABEL,
        copy.text(MessageId::PREFERENCES_LANGUAGE_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_LANGUAGE_HELP,
        copy.text(MessageId::PREFERENCES_LANGUAGE_HELP)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_NOTIFICATION_WINDOWS_ALLOWED,
        copy.text(MessageId::PREFERENCES_NOTIFICATION_WINDOWS_ALLOWED)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_13(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::PREFERENCES_NOTIFICATION_WINDOWS_BLOCKED,
        copy.text(MessageId::PREFERENCES_NOTIFICATION_WINDOWS_BLOCKED)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_NOTIFICATION_OBSERVER_RUNNING,
        copy.text(MessageId::PREFERENCES_NOTIFICATION_OBSERVER_RUNNING)?,
    )?;
    push(
        rows,
        CopySlot::PREFERENCES_TRAY_READY_NOTE,
        copy.text(MessageId::PREFERENCES_TRAY_READY_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_PROFILE_VALUE,
        copy.text(MessageId::SOURCE_PROFILE_VALUE)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_PREVIEW_NOTE,
        copy.text(MessageId::SOURCE_PREVIEW_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_ADD_SOURCE,
        copy.text(MessageId::SOURCE_ADD_SOURCE)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_REMOVE_SOURCE,
        copy.text(MessageId::SOURCE_REMOVE_SOURCE)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_REVIEW_CHANGES,
        copy.text(MessageId::SOURCE_REVIEW_CHANGES)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_REVIEW_HEADING,
        copy.text(MessageId::SOURCE_REVIEW_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_REVIEW_DESCRIPTION,
        copy.text(MessageId::SOURCE_REVIEW_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_BEFORE_LABEL,
        copy.text(MessageId::SOURCE_BEFORE_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_AFTER_LABEL,
        copy.text(MessageId::SOURCE_AFTER_LABEL)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_14(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::SOURCE_UNDO,
        copy.text(MessageId::SOURCE_UNDO)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_DISCARD,
        copy.text(MessageId::SOURCE_DISCARD)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_CLOSE_REVIEW,
        copy.text(MessageId::SOURCE_CLOSE_REVIEW)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_APPLY,
        copy.text(MessageId::SOURCE_APPLY)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_APPLY_UNAVAILABLE,
        copy.text(MessageId::SOURCE_APPLY_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_EMPTY_HEADING,
        copy.text(MessageId::SOURCE_EMPTY_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_EMPTY_DESCRIPTION,
        copy.text(MessageId::SOURCE_EMPTY_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_EMPTY_REVIEW,
        copy.text(MessageId::SOURCE_EMPTY_REVIEW)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_ADD_TITLE,
        copy.text(MessageId::SOURCE_ADD_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_EDIT_TITLE,
        copy.text(MessageId::SOURCE_EDIT_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_PROTOCOLS_0,
        copy.text(MessageId::SOURCE_PROTOCOL_OPTION_NTS)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_PROTOCOLS_1,
        copy.text(MessageId::SOURCE_PROTOCOL_OPTION_NTP)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_15(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::SOURCE_PROTOCOL_HELP,
        copy.text(MessageId::SOURCE_PROTOCOL_HELP)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_NTP_WARNING,
        copy.text(MessageId::SOURCE_NTP_WARNING)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_NTP_PORT,
        copy.text(MessageId::SOURCE_NTP_PORT)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_PORT_HELP,
        copy.text(MessageId::SOURCE_PORT_HELP)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_STATISTICS_DESCRIPTION,
        copy.text(MessageId::SOURCE_STATISTICS_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_CHANGE_LABELS_0,
        copy.text(MessageId::RUST_CHANGE_BASELINE)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_CHANGE_LABELS_1,
        copy.text(MessageId::RUST_CHANGE_ADD)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_CHANGE_LABELS_2,
        copy.text(MessageId::RUST_CHANGE_EDIT)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_CHANGE_LABELS_3,
        copy.text(MessageId::RUST_CHANGE_REMOVE)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_REVIEW_NONE,
        copy.text(MessageId::RUST_REVIEW_NONE)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_PAGE_REQUIRED,
        copy.text(MessageId::SOURCE_PAGE_REQUIRED)?,
    )?;
    push(
        rows,
        CopySlot::SOURCE_LIST_REQUIRED,
        copy.text(MessageId::SOURCE_LIST_REQUIRED)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_16(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::SOURCE_REVIEW_REQUIRED,
        copy.text(MessageId::SOURCE_REVIEW_REQUIRED)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_TITLE,
        copy.text(MessageId::RECOVERY_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_DESCRIPTION,
        copy.text(MessageId::RECOVERY_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_BACK,
        copy.text(MessageId::RECOVERY_BACK)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_SETTINGS_DESCRIPTION,
        copy.text(MessageId::RECOVERY_SETTINGS_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_CONTEXT_DESCRIPTION,
        copy.text(MessageId::RECOVERY_CONTEXT_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_OPEN_DETAILS,
        copy.text(MessageId::RECOVERY_OPEN_DETAILS)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_LEAVE_TITLE,
        copy.text(MessageId::RECOVERY_LEAVE_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_LEAVE_DESCRIPTION,
        copy.text(MessageId::RECOVERY_LEAVE_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_LEAVE_KEEP,
        copy.text(MessageId::RECOVERY_LEAVE_KEEP)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_LEAVE_DISCARD,
        copy.text(MessageId::RECOVERY_LEAVE_DISCARD)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_LEAVE_STAY,
        copy.text(MessageId::RECOVERY_LEAVE_STAY)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_17(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::RECOVERY_ROUGHTIME_TITLE,
        copy.text(MessageId::RECOVERY_ROUGHTIME_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_ROUGHTIME_DESCRIPTION,
        copy.text(MessageId::RECOVERY_ROUGHTIME_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_NO_EFFECT,
        copy.text(MessageId::RECOVERY_NO_EFFECT)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_UNAVAILABLE,
        copy.text(MessageId::RECOVERY_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_SIGNATURES,
        copy.text(MessageId::RECOVERY_SIGNATURES)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_DATE,
        copy.text(MessageId::RECOVERY_DATE)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_HANDOFF,
        copy.text(MessageId::RECOVERY_HANDOFF)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_UNOBSERVED,
        copy.text(MessageId::RECOVERY_UNOBSERVED)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_NO_DATE,
        copy.text(MessageId::RECOVERY_NO_DATE)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_SHOW_PATH,
        copy.text(MessageId::RECOVERY_SHOW_PATH)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_HIDE_PATH,
        copy.text(MessageId::RECOVERY_HIDE_PATH)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_RETAINED_STEP,
        copy.text(MessageId::RECOVERY_RETAINED_STEP)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_18(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::RECOVERY_DATE_STEP,
        copy.text(MessageId::RECOVERY_DATE_STEP)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_NTS_STEP,
        copy.text(MessageId::RECOVERY_NTS_STEP)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_AGREEMENT_STEP,
        copy.text(MessageId::RECOVERY_AGREEMENT_STEP)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_AUTHORITY_STEP,
        copy.text(MessageId::RECOVERY_AUTHORITY_STEP)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_HELPERS_TITLE,
        copy.text(MessageId::RECOVERY_HELPERS_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_HELPERS_DESCRIPTION,
        copy.text(MessageId::RECOVERY_HELPERS_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_ONE_HELPER_WARNING,
        copy.text(MessageId::RECOVERY_ONE_HELPER_WARNING)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_TWO_HELPER_WARNING,
        copy.text(MessageId::RECOVERY_TWO_HELPER_WARNING)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_COUNT_LABEL,
        copy.text(MessageId::RECOVERY_COUNT_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_COUNTS_0,
        copy.text(MessageId::RECOVERY_COUNT_OPTION_OFF)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_COUNTS_1,
        copy.text(MessageId::RECOVERY_COUNT_OPTION_ONE)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_COUNTS_2,
        copy.text(MessageId::RECOVERY_COUNT_OPTION_TWO)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_19(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::RECOVERY_FIRST_LABEL,
        copy.text(MessageId::RECOVERY_FIRST_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_SECOND_LABEL,
        copy.text(MessageId::RECOVERY_SECOND_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_HOST_HELP,
        copy.text(MessageId::RECOVERY_HOST_HELP)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_PREVIEW,
        copy.text(MessageId::RECOVERY_PREVIEW)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_CANCEL,
        copy.text(MessageId::RECOVERY_CANCEL)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_REVIEW_TITLE,
        copy.text(MessageId::RECOVERY_REVIEW_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_OFF,
        copy.text(MessageId::RECOVERY_OFF)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_BOUNDARY,
        copy.text(MessageId::RECOVERY_BOUNDARY)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_KEEP_EDITING,
        copy.text(MessageId::RECOVERY_KEEP_EDITING)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_DISCARD,
        copy.text(MessageId::RECOVERY_DISCARD)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_APPLY,
        copy.text(MessageId::RECOVERY_APPLY)?,
    )?;
    push(
        rows,
        CopySlot::RECOVERY_APPLY_UNAVAILABLE,
        copy.text(MessageId::RECOVERY_APPLY_UNAVAILABLE)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_20(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::SHELL_QUIT_UI,
        copy.text(MessageId::SHELL_QUIT_UI)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_QUIT_TITLE,
        copy.text(MessageId::SHELL_QUIT_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_KEEP_OPEN,
        copy.text(MessageId::SHELL_KEEP_OPEN)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_MANAGE_SERVICE,
        copy.text(MessageId::SHELL_MANAGE_SERVICE)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_QUIT_ONLY,
        copy.text(MessageId::SHELL_QUIT_ONLY)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_UNKNOWN_CONSEQUENCE,
        copy.text(MessageId::SHELL_UNKNOWN_CONSEQUENCE)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_DRAFT_LOSS_TITLE,
        copy.text(MessageId::SHELL_DRAFT_LOSS_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_DRAFT_LOSS,
        copy.text(MessageId::SHELL_DRAFT_LOSS)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_OBSERVER_CONSEQUENCE,
        copy.text(MessageId::SHELL_OBSERVER_CONSEQUENCE)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_STOP_PATH,
        copy.text(MessageId::SHELL_STOP_PATH)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_DISABLE_PATH,
        copy.text(MessageId::SHELL_DISABLE_PATH)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_SEPARATE_MANAGEMENT,
        copy.text(MessageId::SHELL_SEPARATE_MANAGEMENT)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_21(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::SHELL_EXIT_REQUESTED,
        copy.text(MessageId::SHELL_EXIT_REQUESTED)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_BACKGROUND_SERVICE,
        copy.text(MessageId::SHELL_BACKGROUND_SERVICE)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_NOTIFICATIONS,
        copy.text(MessageId::SHELL_NOTIFICATIONS)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_MAIN_CONTENT,
        copy.text(MessageId::SHELL_MAIN_CONTENT)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_SERVICE_DESCRIPTION,
        copy.text(MessageId::SHELL_SERVICE_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_SERVICE_SUMMARY,
        copy.text(MessageId::SHELL_SERVICE_SUMMARY)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_MANAGE_POWER,
        copy.text(MessageId::SHELL_MANAGE_POWER)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_SERVICE_UNAVAILABLE,
        copy.text(MessageId::SHELL_SERVICE_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_DIAGNOSE,
        copy.text(MessageId::SHELL_DIAGNOSE)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_DIAGNOSE_POWER,
        copy.text(MessageId::SHELL_DIAGNOSE_POWER)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_DIAGNOSE_NOTE,
        copy.text(MessageId::SHELL_DIAGNOSE_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_STARTUP_TITLE,
        copy.text(MessageId::SHELL_STARTUP_TITLE)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_22(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::SHELL_STARTUP_NOTE,
        copy.text(MessageId::SHELL_STARTUP_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_AUTOSTART_ON,
        copy.text(MessageId::SHELL_AUTOSTART_ON)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_AUTOSTART_OFF,
        copy.text(MessageId::SHELL_AUTOSTART_OFF)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_DISABLE_NOTE,
        copy.text(MessageId::SHELL_DISABLE_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_TIME_MANAGER_TITLE,
        copy.text(MessageId::SHELL_TIME_MANAGER_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_USE_WINDOWS,
        copy.text(MessageId::SHELL_USE_WINDOWS)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_USE_NEVROTICK,
        copy.text(MessageId::SHELL_USE_NEVROTICK)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_HANDOVER_NOTE,
        copy.text(MessageId::SHELL_HANDOVER_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_SYNCHRONIZATION_SETTINGS,
        copy.text(MessageId::SHELL_SYNCHRONIZATION_SETTINGS)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_TIME_POLICY,
        copy.text(MessageId::SHELL_TIME_POLICY)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_TIME_POLICY_DESCRIPTION,
        copy.text(MessageId::SHELL_TIME_POLICY_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_TIME_POLICY_POWER,
        copy.text(MessageId::SHELL_TIME_POLICY_POWER)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_23(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::SHELL_PERSONAL_PREFERENCES,
        copy.text(MessageId::SHELL_PERSONAL_PREFERENCES)?,
    )?;
    push(
        rows,
        CopySlot::SHELL_PERSONAL_PREFERENCES_DESCRIPTION,
        copy.text(MessageId::SHELL_PERSONAL_PREFERENCES_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_HEADING,
        copy.text(MessageId::POLICY_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_DESCRIPTION,
        copy.text(MessageId::POLICY_DESCRIPTION)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_BASELINE,
        copy.text(MessageId::POLICY_BASELINE)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_PREVIEW,
        copy.text(MessageId::POLICY_PREVIEW)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_NO_PREVIEW,
        copy.text(MessageId::POLICY_NO_PREVIEW)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_NTS,
        copy.text(MessageId::POLICY_NTS)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_NTP,
        copy.text(MessageId::POLICY_NTP)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_OBSERVE,
        copy.text(MessageId::POLICY_OBSERVE)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_CHOOSE,
        copy.text(MessageId::POLICY_CHOOSE)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_EDIT,
        copy.text(MessageId::POLICY_EDIT)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_24(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::POLICY_REVIEW,
        copy.text(MessageId::POLICY_REVIEW)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_KEEP,
        copy.text(MessageId::POLICY_KEEP)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_DISCARD,
        copy.text(MessageId::POLICY_DISCARD)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_CANCEL,
        copy.text(MessageId::POLICY_CANCEL)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_CHOICE_LABEL,
        copy.text(MessageId::POLICY_CHOICE_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_NTS_CONSEQUENCE,
        copy.text(MessageId::POLICY_NTS_CONSEQUENCE)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_NTP_CONSEQUENCE,
        copy.text(MessageId::POLICY_NTP_CONSEQUENCE)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_OBSERVE_CONSEQUENCE,
        copy.text(MessageId::POLICY_OBSERVE_CONSEQUENCE)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_AUTHORITY,
        copy.text(MessageId::POLICY_AUTHORITY)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_APPLY,
        copy.text(MessageId::POLICY_APPLY)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_APPLY_UNAVAILABLE,
        copy.text(MessageId::POLICY_APPLY_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_FIXTURE_NOTE,
        copy.text(MessageId::POLICY_FIXTURE_NOTE)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_25(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::POLICY_PREVIEW_NOTE,
        copy.text(MessageId::POLICY_PREVIEW_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_PHASE_REQUIRED,
        copy.text(MessageId::POLICY_PHASE_REQUIRED)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_INVALID_CHOICE,
        copy.text(MessageId::POLICY_INVALID_CHOICE)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_NO_PREVIEW_ERROR,
        copy.text(MessageId::POLICY_NO_PREVIEW_ERROR)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_PAGE_REQUIRED,
        copy.text(MessageId::POLICY_PAGE_REQUIRED)?,
    )?;
    push(
        rows,
        CopySlot::POLICY_EDITOR_ACTIVE,
        copy.text(MessageId::POLICY_EDITOR_ACTIVE)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_QUERY_LABEL,
        copy.text(MessageId::INSPECTION_QUERY_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_QUERY_PLACEHOLDER,
        copy.text(MessageId::INSPECTION_QUERY_PLACEHOLDER)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_QUERY_HELP,
        copy.text(MessageId::INSPECTION_QUERY_HELP)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_SEARCH,
        copy.text(MessageId::INSPECTION_SEARCH)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_CLEAR,
        copy.text(MessageId::INSPECTION_CLEAR)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_ALL,
        copy.text(MessageId::INSPECTION_ALL)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_26(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::INSPECTION_USED,
        copy.text(MessageId::INSPECTION_USED)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_OTHER,
        copy.text(MessageId::INSPECTION_OTHER)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_ROLES_UNAVAILABLE,
        copy.text(MessageId::INSPECTION_ROLES_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_NO_MATCHES_HEADING,
        copy.text(MessageId::INSPECTION_NO_MATCHES_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_NO_MATCHES,
        copy.text(MessageId::INSPECTION_NO_MATCHES)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_ACCEPTED_QUERY,
        copy.text(MessageId::INSPECTION_ACCEPTED_QUERY)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_DETAILS,
        copy.text(MessageId::INSPECTION_DETAILS)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_DETAILS_HEADING,
        copy.text(MessageId::INSPECTION_DETAILS_HEADING)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_CLOSE_DETAILS,
        copy.text(MessageId::INSPECTION_CLOSE_DETAILS)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_DETAILS_NOTE,
        copy.text(MessageId::INSPECTION_DETAILS_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_HIDDEN_DETAILS,
        copy.text(MessageId::INSPECTION_HIDDEN_DETAILS)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_SELECTED_HIDDEN,
        copy.text(MessageId::INSPECTION_SELECTED_HIDDEN)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_27(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::INSPECTION_IDENTITY,
        copy.text(MessageId::INSPECTION_IDENTITY)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_LABEL,
        copy.text(MessageId::INSPECTION_LABEL)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_PROTOCOL,
        copy.text(MessageId::INSPECTION_PROTOCOL)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_PORT,
        copy.text(MessageId::INSPECTION_PORT)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_LOCAL_ID_NOTE,
        copy.text(MessageId::INSPECTION_LOCAL_ID_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_AUTHENTICATION,
        copy.text(MessageId::INSPECTION_AUTHENTICATION)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_AUTH_MEASUREMENT,
        copy.text(MessageId::INSPECTION_AUTH_MEASUREMENT)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_ROLE,
        copy.text(MessageId::INSPECTION_ROLE)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_OPERATOR,
        copy.text(MessageId::INSPECTION_OPERATOR)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_ENDPOINT,
        copy.text(MessageId::INSPECTION_ENDPOINT)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_VOTE_IDENTITY,
        copy.text(MessageId::INSPECTION_VOTE_IDENTITY)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_REQUEST_STATUS,
        copy.text(MessageId::INSPECTION_REQUEST_STATUS)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_28(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::INSPECTION_LAST_REPLY,
        copy.text(MessageId::INSPECTION_LAST_REPLY)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_LAST_ACCEPTED,
        copy.text(MessageId::INSPECTION_LAST_ACCEPTED)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_OFFSET,
        copy.text(MessageId::INSPECTION_OFFSET)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_DELAY,
        copy.text(MessageId::INSPECTION_DELAY)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_UNCERTAINTY,
        copy.text(MessageId::INSPECTION_UNCERTAINTY)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_CERTIFICATE,
        copy.text(MessageId::INSPECTION_CERTIFICATE)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_SCALE,
        copy.text(MessageId::INSPECTION_SCALE)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_LEAP,
        copy.text(MessageId::INSPECTION_LEAP)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_NOT_OBSERVED,
        copy.text(MessageId::INSPECTION_NOT_OBSERVED)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_NOT_MEASURED,
        copy.text(MessageId::INSPECTION_NOT_MEASURED)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_NOT_ESTABLISHED,
        copy.text(MessageId::INSPECTION_NOT_ESTABLISHED)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_NOT_EVALUATED,
        copy.text(MessageId::INSPECTION_NOT_EVALUATED)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_29(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::INSPECTION_NOT_APPLICABLE,
        copy.text(MessageId::INSPECTION_NOT_APPLICABLE)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_UNAUTHENTICATED,
        copy.text(MessageId::INSPECTION_UNAUTHENTICATED)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_NTS,
        copy.text(MessageId::INSPECTION_NTS)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_NTP,
        copy.text(MessageId::INSPECTION_NTP)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_INVALID_QUERY,
        copy.text(MessageId::INSPECTION_INVALID_QUERY)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_INVALID_FILTER,
        copy.text(MessageId::INSPECTION_INVALID_FILTER)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_HIDDEN_SOURCE,
        copy.text(MessageId::INSPECTION_HIDDEN_SOURCE)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_PAGE_REQUIRED,
        copy.text(MessageId::INSPECTION_PAGE_REQUIRED)?,
    )?;
    push(
        rows,
        CopySlot::INSPECTION_EDITOR_ACTIVE,
        copy.text(MessageId::INSPECTION_EDITOR_ACTIVE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_BACK,
        copy.text(MessageId::DIAGNOSTIC_BACK)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_PROBLEM,
        copy.text(MessageId::DIAGNOSTIC_PROBLEM)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_EFFECT,
        copy.text(MessageId::DIAGNOSTIC_EFFECT)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_30(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::DIAGNOSTIC_REMEDY,
        copy.text(MessageId::DIAGNOSTIC_REMEDY)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_SERVICE,
        copy.text(MessageId::DIAGNOSTIC_SERVICE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_EVENTS_EMPTY,
        copy.text(MessageId::DIAGNOSTIC_EVENTS_EMPTY)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_EVENTS_NOTE,
        copy.text(MessageId::DIAGNOSTIC_EVENTS_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_REPORT_OPEN,
        copy.text(MessageId::DIAGNOSTIC_REPORT_OPEN)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_REPORT_TITLE,
        copy.text(MessageId::DIAGNOSTIC_REPORT_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_REPORT_NOTE,
        copy.text(MessageId::DIAGNOSTIC_REPORT_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_COPY,
        copy.text(MessageId::DIAGNOSTIC_COPY)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_EXPORT,
        copy.text(MessageId::DIAGNOSTIC_EXPORT)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_EXPORT_REASON,
        copy.text(MessageId::DIAGNOSTIC_EXPORT_REASON)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_COLLECTION_OPEN,
        copy.text(MessageId::DIAGNOSTIC_COLLECTION_OPEN)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_COLLECTION_TITLE,
        copy.text(MessageId::DIAGNOSTIC_COLLECTION_TITLE)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_31(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::DIAGNOSTIC_COLLECTION_NOTE,
        copy.text(MessageId::DIAGNOSTIC_COLLECTION_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_SCOPE,
        copy.text(MessageId::DIAGNOSTIC_SCOPE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_SCOPES_0,
        copy.text(MessageId::DIAGNOSTIC_SCOPE_OPTION_UI)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_SCOPES_1,
        copy.text(MessageId::DIAGNOSTIC_SCOPE_OPTION_SERVICE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_SCOPES_2,
        copy.text(MessageId::DIAGNOSTIC_SCOPE_OPTION_COMBINED)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_DURATION,
        copy.text(MessageId::DIAGNOSTIC_DURATION)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_COLLECTION_STATE,
        copy.text(MessageId::DIAGNOSTIC_COLLECTION_STATE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_EFFECTIVE_SCOPE,
        copy.text(MessageId::DIAGNOSTIC_EFFECTIVE_SCOPE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_REMAINING,
        copy.text(MessageId::DIAGNOSTIC_REMAINING)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_UNAVAILABLE,
        copy.text(MessageId::DIAGNOSTIC_UNAVAILABLE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_COLLECTION_REASON,
        copy.text(MessageId::DIAGNOSTIC_COLLECTION_REASON)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_REQUIRED_EVENTS,
        copy.text(MessageId::DIAGNOSTIC_REQUIRED_EVENTS)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_32(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::DIAGNOSTIC_START,
        copy.text(MessageId::DIAGNOSTIC_START)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_STOP,
        copy.text(MessageId::DIAGNOSTIC_STOP)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_CORRECTION_OPEN,
        copy.text(MessageId::DIAGNOSTIC_CORRECTION_OPEN)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_CORRECTION_TITLE,
        copy.text(MessageId::DIAGNOSTIC_CORRECTION_TITLE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_CORRECTION_EMPTY,
        copy.text(MessageId::DIAGNOSTIC_CORRECTION_EMPTY)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_CORRECTION_NOTE,
        copy.text(MessageId::DIAGNOSTIC_CORRECTION_NOTE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_CORRECTION_RULE,
        copy.text(MessageId::DIAGNOSTIC_CORRECTION_RULE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_APPROVE,
        copy.text(MessageId::DIAGNOSTIC_APPROVE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_UNAVAILABLE_ACTION,
        copy.text(MessageId::DIAGNOSTIC_UNAVAILABLE_ACTION)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_INVALID_CHOICE,
        copy.text(MessageId::DIAGNOSTIC_INVALID_CHOICE)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_DURATIONS_0,
        copy.minutes(MessageId::DURATION_MINUTES, 5)?,
    )?;
    push(
        rows,
        CopySlot::DIAGNOSTIC_DURATIONS_1,
        copy.minutes(MessageId::DURATION_MINUTES, 15)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_33(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::DIAGNOSTIC_DURATIONS_2,
        copy.minutes(MessageId::DURATION_MINUTES, 30)?,
    )?;
    push(
        rows,
        CopySlot::RAW_TOO_LONG,
        copy.text(MessageId::RAW_TOO_LONG)?,
    )?;
    push(
        rows,
        CopySlot::RAW_CHARACTER,
        copy.text(MessageId::RAW_CHARACTER)?,
    )?;
    push(
        rows,
        CopySlot::RAW_INVALID_OPTION,
        copy.text(MessageId::RAW_INVALID_OPTION)?,
    )?;
    push(
        rows,
        CopySlot::RAW_CONTEXT,
        copy.text(MessageId::RAW_CONTEXT)?,
    )?;
    push(
        rows,
        CopySlot::RAW_UNAVAILABLE,
        copy.text(MessageId::RAW_UNAVAILABLE)?,
    )?;
    push(rows, CopySlot::RAW_STALE, copy.text(MessageId::RAW_STALE)?)?;
    push(
        rows,
        CopySlot::RAW_ACTION_UNDO,
        copy.text(MessageId::RAW_ACTION_UNDO)?,
    )?;
    push(
        rows,
        CopySlot::RAW_ACTION_REDO,
        copy.text(MessageId::RAW_ACTION_REDO)?,
    )?;
    push(
        rows,
        CopySlot::RAW_ACTION_CUT,
        copy.text(MessageId::RAW_ACTION_CUT)?,
    )?;
    push(
        rows,
        CopySlot::RAW_ACTION_COPY,
        copy.text(MessageId::RAW_ACTION_COPY)?,
    )?;
    push(
        rows,
        CopySlot::RAW_ACTION_PASTE,
        copy.text(MessageId::RAW_ACTION_PASTE)?,
    )?;
    Ok(())
}

/// Render one closed output cohort without resetting the preparation budget.
///
/// # Errors
/// Propagates protected rendering or aggregate exhaustion before publication.
fn fill_34(copy: &mut CopyPreparation<'_, '_>, rows: &mut Vec<SharedString>) -> Result<(), Error> {
    push(
        rows,
        CopySlot::RAW_ACTION_SELECT_ALL,
        copy.text(MessageId::RAW_ACTION_SELECT_ALL)?,
    )?;
    Ok(())
}

/// Replace feedback by constructing a new immutable model; old readers retain theirs.
///
/// # Errors
/// Rejects a malformed prior shape or complete-output failure before publication.
pub(super) fn replace_feedback(
    value: &UiCopy,
    feedback: &PreparedFeedback,
) -> Result<UiCopy, Error> {
    check(value)?;
    let mut budget = OutputBudget::default();
    budget.charge(&value.revision)?;
    let mut rows = Vec::with_capacity(OUTPUT_SLOT_COUNT);
    for index in 0..STATIC_SLOT_COUNT {
        let text = value.texts.row_data(index).ok_or(Error::Schema)?;
        budget.charge(&text)?;
        rows.push(text);
    }
    append_feedback(&mut rows, &mut budget, feedback)?;
    finish(value.revision.clone(), rows)
}

/// Check the complete static/feedback shape and both aggregate output limits.
///
/// # Errors
/// Rejects missing rows, empty required static text, overflow or output exhaustion.
pub(super) fn check(value: &UiCopy) -> Result<(), Error> {
    if !value.catalog_ready || value.texts.row_count() != OUTPUT_SLOT_COUNT {
        return Err(Error::Schema);
    }
    let mut budget = OutputBudget::default();
    budget.charge(&value.revision)?;
    for index in 0..OUTPUT_SLOT_COUNT {
        let text = value.texts.row_data(index).ok_or(Error::Schema)?;
        if index < STATIC_SLOT_COUNT && text.is_empty() {
            return Err(Error::Schema);
        }
        budget.charge(&text)?;
    }
    Ok(())
}
