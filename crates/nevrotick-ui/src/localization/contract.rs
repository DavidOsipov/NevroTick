//! Generated only from a protected source contract; never from catalog data.
use super::Error;

/// Code-owned coherent critical-copy group.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Group {
    /// Protected previewfacts meaning/action group.
    PreviewFacts,
    /// Protected sourceedit meaning/action group.
    SourceEdit,
    /// Protected intervals meaning/action group.
    Intervals,
    /// Protected lifecycle meaning/action group.
    Lifecycle,
    /// Protected notifications meaning/action group.
    Notifications,
    /// Protected policy meaning/action group.
    Policy,
    /// Protected notices meaning/action group.
    Notices,
    /// Protected recovery meaning/action group.
    Recovery,
    /// Protected quit meaning/action group.
    Quit,
}

/// Independently selected argument and plural representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Kind {
    /// Argument-free inert text.
    Text,
    /// Exactly one checked positive whole-minute argument in both English forms.
    Minutes,
}

/// Private dense storage identity, not a wire ordinal or action.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct MessageId(usize);

/// Protected active definition; stable ordinals are never reused.
#[derive(Clone, Copy, Debug)]
pub struct Spec {
    /// Private checked storage identity.
    pub id: MessageId,
    /// Exact stable ASCII wire key.
    pub key: &'static str,
    /// Original append-only identity ordinal, including retired slots.
    pub ordinal: u16,
    /// Exact selected argument representation.
    pub kind: Kind,
    /// Coherent protected critical group, if any.
    pub group: Option<Group>,
}
/// Compatibility fingerprint; grants no semantic approval.
pub const FINGERPRINT: &str = "34c97f8a74a9559ae998f55389bedd9f92bb3f878e72df84d090372053a69e80";
/// Fixed product catalog domain, never a path.
pub const DOMAIN: &str = "nevrotick.ui";
/// Fixed English cardinal rule and checked integer domain.
pub const PLURAL: &str = "english-u64-cardinal-v1";
/// Exact selected JSON/CBOR file representation version.
pub const CATALOG_SCHEMA_VERSION: u32 = 2;
/// Code-owned engineering GUI build version, distinct from file schema.
pub const GUI_BUILD_VERSION: u32 = 2;
/// Code-owned text meaning/type contract version, distinct from build/schema.
pub const TEXT_CONTRACT_VERSION: u32 = 2;
/// Number of active messages, not registry ordinals.
pub const COUNT: usize = 509;
impl MessageId {
    /// Resolve one exact active ID without aliases or normalization.
    ///
    /// # Errors
    /// Rejects unknown and retired identities with a fixed schema error.
    pub fn from_key(key: &str) -> Result<Self, Error> {
        SPECS
            .iter()
            .find(|spec| spec.key == key)
            .map(|spec| spec.id)
            .ok_or(Error::Schema)
    }
    /// Return the private dense slot, never the stable external ordinal.
    #[must_use]
    pub const fn index(self) -> usize {
        self.0
    }
    /// Borrow the independently protected definition.
    ///
    /// # Errors
    /// Rejects an impossible absent storage identity instead of indexing unchecked.
    pub fn spec(self) -> Result<&'static Spec, Error> {
        SPECS.get(self.0).ok_or(Error::Schema)
    }
    /// ui.fixture.window-title; stable ordinal 2.
    pub const FIXTURE_WINDOW_TITLE: Self = Self(0);
    /// ui.fixture.sample-label; stable ordinal 3.
    pub const FIXTURE_SAMPLE_LABEL: Self = Self(1);
    /// ui.fixture.local-boundary; stable ordinal 4.
    pub const FIXTURE_LOCAL_BOUNDARY: Self = Self(2);
    /// ui.fixture.memory-boundary; stable ordinal 5.
    pub const FIXTURE_MEMORY_BOUNDARY: Self = Self(3);
    /// ui.fixture.simple-mode; stable ordinal 6.
    pub const FIXTURE_SIMPLE_MODE: Self = Self(4);
    /// ui.fixture.power-mode; stable ordinal 7.
    pub const FIXTURE_POWER_MODE: Self = Self(5);
    /// ui.fixture.status-tab; stable ordinal 8.
    pub const FIXTURE_STATUS_TAB: Self = Self(6);
    /// ui.fixture.sources-tab; stable ordinal 9.
    pub const FIXTURE_SOURCES_TAB: Self = Self(7);
    /// ui.fixture.statistics-tab; stable ordinal 10.
    pub const FIXTURE_STATISTICS_TAB: Self = Self(8);
    /// ui.fixture.diagnostics-tab; stable ordinal 11.
    pub const FIXTURE_DIAGNOSTICS_TAB: Self = Self(9);
    /// ui.fixture.settings-tab; stable ordinal 12.
    pub const FIXTURE_SETTINGS_TAB: Self = Self(10);
    /// ui.fixture.about-tab; stable ordinal 13.
    pub const FIXTURE_ABOUT_TAB: Self = Self(11);
    /// ui.fixture.preview-badge; stable ordinal 14.
    pub const FIXTURE_PREVIEW_BADGE: Self = Self(12);
    /// ui.fixture.not-measured; stable ordinal 15.
    pub const FIXTURE_NOT_MEASURED: Self = Self(13);
    /// ui.fixture.not-evaluated; stable ordinal 16.
    pub const FIXTURE_NOT_EVALUATED: Self = Self(14);
    /// ui.fixture.unavailable; stable ordinal 17.
    pub const FIXTURE_UNAVAILABLE: Self = Self(15);
    /// ui.fixture.no-live-connection; stable ordinal 18.
    pub const FIXTURE_NO_LIVE_CONNECTION: Self = Self(16);
    /// ui.fixture.status-kicker; stable ordinal 19.
    pub const FIXTURE_STATUS_KICKER: Self = Self(17);
    /// ui.fixture.status-heading; stable ordinal 20.
    pub const FIXTURE_STATUS_HEADING: Self = Self(18);
    /// ui.fixture.status-qualifier; stable ordinal 24.
    pub const FIXTURE_STATUS_QUALIFIER: Self = Self(19);
    /// ui.fixture.difference-label; stable ordinal 26.
    pub const FIXTURE_DIFFERENCE_LABEL: Self = Self(20);
    /// ui.fixture.difference-note; stable ordinal 27.
    pub const FIXTURE_DIFFERENCE_NOTE: Self = Self(21);
    /// ui.fixture.uncertainty-label; stable ordinal 28.
    pub const FIXTURE_UNCERTAINTY_LABEL: Self = Self(22);
    /// ui.fixture.uncertainty-note; stable ordinal 29.
    pub const FIXTURE_UNCERTAINTY_NOTE: Self = Self(23);
    /// ui.fixture.authentication-label; stable ordinal 30.
    pub const FIXTURE_AUTHENTICATION_LABEL: Self = Self(24);
    /// ui.fixture.authentication-value; stable ordinal 31.
    pub const FIXTURE_AUTHENTICATION_VALUE: Self = Self(25);
    /// ui.fixture.authentication-note; stable ordinal 32.
    pub const FIXTURE_AUTHENTICATION_NOTE: Self = Self(26);
    /// ui.fixture.agreement-label; stable ordinal 33.
    pub const FIXTURE_AGREEMENT_LABEL: Self = Self(27);
    /// ui.fixture.agreement-note; stable ordinal 34.
    pub const FIXTURE_AGREEMENT_NOTE: Self = Self(28);
    /// ui.fixture.qualified-label; stable ordinal 35.
    pub const FIXTURE_QUALIFIED_LABEL: Self = Self(29);
    /// ui.fixture.sync-now; stable ordinal 36.
    pub const FIXTURE_SYNC_NOW: Self = Self(30);
    /// ui.fixture.sync-unavailable; stable ordinal 37.
    pub const FIXTURE_SYNC_UNAVAILABLE: Self = Self(31);
    /// ui.fixture.sources-heading; stable ordinal 38.
    pub const FIXTURE_SOURCES_HEADING: Self = Self(32);
    /// ui.fixture.sources-description; stable ordinal 39.
    pub const FIXTURE_SOURCES_DESCRIPTION: Self = Self(33);
    /// ui.fixture.source-list-note; stable ordinal 42.
    pub const FIXTURE_SOURCE_LIST_NOTE: Self = Self(34);
    /// ui.fixture.edit-source; stable ordinal 43.
    pub const FIXTURE_EDIT_SOURCE: Self = Self(35);
    /// ui.fixture.select-source; stable ordinal 45.
    pub const FIXTURE_SELECT_SOURCE: Self = Self(36);
    /// ui.fixture.selected; stable ordinal 46.
    pub const FIXTURE_SELECTED: Self = Self(37);
    /// ui.fixture.server-identity; stable ordinal 47.
    pub const FIXTURE_SERVER_IDENTITY: Self = Self(38);
    /// ui.fixture.display-name; stable ordinal 48.
    pub const FIXTURE_DISPLAY_NAME: Self = Self(39);
    /// ui.fixture.nts-port; stable ordinal 49.
    pub const FIXTURE_NTS_PORT: Self = Self(40);
    /// ui.fixture.protocol-label; stable ordinal 50.
    pub const FIXTURE_PROTOCOL_LABEL: Self = Self(41);
    /// ui.fixture.nts-preview; stable ordinal 51.
    pub const FIXTURE_NTS_PREVIEW: Self = Self(42);
    /// ui.fixture.source-role; stable ordinal 52.
    pub const FIXTURE_SOURCE_ROLE: Self = Self(43);
    /// ui.fixture.last-reply; stable ordinal 53.
    pub const FIXTURE_LAST_REPLY: Self = Self(44);
    /// ui.fixture.editor-description; stable ordinal 55.
    pub const FIXTURE_EDITOR_DESCRIPTION: Self = Self(45);
    /// ui.fixture.label-help; stable ordinal 56.
    pub const FIXTURE_LABEL_HELP: Self = Self(46);
    /// ui.fixture.host-help; stable ordinal 57.
    pub const FIXTURE_HOST_HELP: Self = Self(47);
    /// ui.fixture.editor-unavailable; stable ordinal 60.
    pub const FIXTURE_EDITOR_UNAVAILABLE: Self = Self(48);
    /// ui.fixture.update-preview; stable ordinal 61.
    pub const FIXTURE_UPDATE_PREVIEW: Self = Self(49);
    /// ui.fixture.cancel; stable ordinal 62.
    pub const FIXTURE_CANCEL: Self = Self(50);
    /// ui.fixture.statistics-heading; stable ordinal 63.
    pub const FIXTURE_STATISTICS_HEADING: Self = Self(51);
    /// ui.fixture.statistics-description; stable ordinal 64.
    pub const FIXTURE_STATISTICS_DESCRIPTION: Self = Self(52);
    /// ui.fixture.statistics-empty-title; stable ordinal 65.
    pub const FIXTURE_STATISTICS_EMPTY_TITLE: Self = Self(53);
    /// ui.fixture.one-snapshot; stable ordinal 67.
    pub const FIXTURE_ONE_SNAPSHOT: Self = Self(54);
    /// ui.fixture.offset-label; stable ordinal 68.
    pub const FIXTURE_OFFSET_LABEL: Self = Self(55);
    /// ui.fixture.positive-offset-note; stable ordinal 69.
    pub const FIXTURE_POSITIVE_OFFSET_NOTE: Self = Self(56);
    /// ui.fixture.diagnostics-heading; stable ordinal 70.
    pub const FIXTURE_DIAGNOSTICS_HEADING: Self = Self(57);
    /// ui.fixture.diagnostics-description; stable ordinal 71.
    pub const FIXTURE_DIAGNOSTICS_DESCRIPTION: Self = Self(58);
    /// ui.fixture.observation-description; stable ordinal 73.
    pub const FIXTURE_OBSERVATION_DESCRIPTION: Self = Self(59);
    /// ui.fixture.freshness-heading; stable ordinal 74.
    pub const FIXTURE_FRESHNESS_HEADING: Self = Self(60);
    /// ui.fixture.freshness-note; stable ordinal 75.
    pub const FIXTURE_FRESHNESS_NOTE: Self = Self(61);
    /// ui.fixture.applied-label; stable ordinal 76.
    pub const FIXTURE_APPLIED_LABEL: Self = Self(62);
    /// ui.fixture.no-correction; stable ordinal 77.
    pub const FIXTURE_NO_CORRECTION: Self = Self(63);
    /// ui.fixture.snapshot-label; stable ordinal 78.
    pub const FIXTURE_SNAPSHOT_LABEL: Self = Self(64);
    /// ui.fixture.fixture-snapshot; stable ordinal 79.
    pub const FIXTURE_FIXTURE_SNAPSHOT: Self = Self(65);
    /// ui.fixture.diagnostics-events; stable ordinal 80.
    pub const FIXTURE_DIAGNOSTICS_EVENTS: Self = Self(66);
    /// ui.fixture.settings-description; stable ordinal 85.
    pub const FIXTURE_SETTINGS_DESCRIPTION: Self = Self(67);
    /// ui.fixture.schedule-heading; stable ordinal 86.
    pub const FIXTURE_SCHEDULE_HEADING: Self = Self(68);
    /// ui.fixture.interval-label; stable ordinal 87.
    pub const FIXTURE_INTERVAL_LABEL: Self = Self(69);
    /// ui.fixture.service-heading; stable ordinal 90.
    pub const FIXTURE_SERVICE_HEADING: Self = Self(70);
    /// ui.fixture.service-description; stable ordinal 91.
    pub const FIXTURE_SERVICE_DESCRIPTION: Self = Self(71);
    /// ui.fixture.runtime-label; stable ordinal 92.
    pub const FIXTURE_RUNTIME_LABEL: Self = Self(72);
    /// ui.fixture.runtime-value; stable ordinal 93.
    pub const FIXTURE_RUNTIME_VALUE: Self = Self(73);
    /// ui.fixture.startup-label; stable ordinal 94.
    pub const FIXTURE_STARTUP_LABEL: Self = Self(74);
    /// ui.fixture.startup-value; stable ordinal 95.
    pub const FIXTURE_STARTUP_VALUE: Self = Self(75);
    /// ui.fixture.owner-label; stable ordinal 96.
    pub const FIXTURE_OWNER_LABEL: Self = Self(76);
    /// ui.fixture.owner-value; stable ordinal 97.
    pub const FIXTURE_OWNER_VALUE: Self = Self(77);
    /// ui.fixture.synchronization-label; stable ordinal 98.
    pub const FIXTURE_SYNCHRONIZATION_LABEL: Self = Self(78);
    /// ui.fixture.synchronization-value; stable ordinal 99.
    pub const FIXTURE_SYNCHRONIZATION_VALUE: Self = Self(79);
    /// ui.fixture.admin-required; stable ordinal 100.
    pub const FIXTURE_ADMIN_REQUIRED: Self = Self(80);
    /// ui.fixture.service-protected-note; stable ordinal 101.
    pub const FIXTURE_SERVICE_PROTECTED_NOTE: Self = Self(81);
    /// ui.fixture.start-service; stable ordinal 102.
    pub const FIXTURE_START_SERVICE: Self = Self(82);
    /// ui.fixture.stop-service; stable ordinal 103.
    pub const FIXTURE_STOP_SERVICE: Self = Self(83);
    /// ui.fixture.restart-service; stable ordinal 104.
    pub const FIXTURE_RESTART_SERVICE: Self = Self(84);
    /// ui.fixture.disable-service; stable ordinal 105.
    pub const FIXTURE_DISABLE_SERVICE: Self = Self(85);
    /// ui.fixture.lifecycle-note; stable ordinal 106.
    pub const FIXTURE_LIFECYCLE_NOTE: Self = Self(86);
    /// ui.fixture.about-heading; stable ordinal 111.
    pub const FIXTURE_ABOUT_HEADING: Self = Self(87);
    /// ui.fixture.about-tagline; stable ordinal 112.
    pub const FIXTURE_ABOUT_TAGLINE: Self = Self(88);
    /// ui.fixture.about-title; stable ordinal 113.
    pub const FIXTURE_ABOUT_TITLE: Self = Self(89);
    /// ui.fixture.about-description; stable ordinal 114.
    pub const FIXTURE_ABOUT_DESCRIPTION: Self = Self(90);
    /// ui.fixture.about-stage; stable ordinal 115.
    pub const FIXTURE_ABOUT_STAGE: Self = Self(91);
    /// ui.fixture.author-label; stable ordinal 116.
    pub const FIXTURE_AUTHOR_LABEL: Self = Self(92);
    /// ui.fixture.repository-label; stable ordinal 118.
    pub const FIXTURE_REPOSITORY_LABEL: Self = Self(93);
    /// ui.fixture.version-label; stable ordinal 120.
    pub const FIXTURE_VERSION_LABEL: Self = Self(94);
    /// ui.fixture.version-value; stable ordinal 121.
    pub const FIXTURE_VERSION_VALUE: Self = Self(95);
    /// ui.fixture.platform-label; stable ordinal 122.
    pub const FIXTURE_PLATFORM_LABEL: Self = Self(96);
    /// ui.fixture.code-license-label; stable ordinal 124.
    pub const FIXTURE_CODE_LICENSE_LABEL: Self = Self(97);
    /// ui.fixture.technologies-heading; stable ordinal 126.
    pub const FIXTURE_TECHNOLOGIES_HEADING: Self = Self(98);
    /// ui.fixture.technologies-description; stable ordinal 127.
    pub const FIXTURE_TECHNOLOGIES_DESCRIPTION: Self = Self(99);
    /// ui.fixture.chrony-note; stable ordinal 128.
    pub const FIXTURE_CHRONY_NOTE: Self = Self(100);
    /// ui.fixture.privacy-heading; stable ordinal 129.
    pub const FIXTURE_PRIVACY_HEADING: Self = Self(101);
    /// ui.fixture.privacy-description; stable ordinal 130.
    pub const FIXTURE_PRIVACY_DESCRIPTION: Self = Self(102);
    /// ui.fixture.artwork-heading; stable ordinal 131.
    pub const FIXTURE_ARTWORK_HEADING: Self = Self(103);
    /// ui.fixture.artwork-description; stable ordinal 132.
    pub const FIXTURE_ARTWORK_DESCRIPTION: Self = Self(104);
    /// ui.preferences.schedule-description; stable ordinal 134.
    pub const PREFERENCES_SCHEDULE_DESCRIPTION: Self = Self(105);
    /// ui.preferences.scheduling-note; stable ordinal 135.
    pub const PREFERENCES_SCHEDULING_NOTE: Self = Self(106);
    /// ui.preferences.custom-label; stable ordinal 136.
    pub const PREFERENCES_CUSTOM_LABEL: Self = Self(107);
    /// ui.preferences.custom-help; stable ordinal 137.
    pub const PREFERENCES_CUSTOM_HELP: Self = Self(108);
    /// ui.preferences.accepted-interval; stable ordinal 138.
    pub const PREFERENCES_ACCEPTED_INTERVAL: Self = Self(109);
    /// ui.preferences.preferences-boundary; stable ordinal 139.
    pub const PREFERENCES_PREFERENCES_BOUNDARY: Self = Self(110);
    /// ui.preferences.host-unavailable; stable ordinal 140.
    pub const PREFERENCES_HOST_UNAVAILABLE: Self = Self(111);
    /// ui.preferences.notifications-heading; stable ordinal 141.
    pub const PREFERENCES_NOTIFICATIONS_HEADING: Self = Self(112);
    /// ui.preferences.notifications-label; stable ordinal 142.
    pub const PREFERENCES_NOTIFICATIONS_LABEL: Self = Self(113);
    /// ui.preferences.notifications-help; stable ordinal 143.
    pub const PREFERENCES_NOTIFICATIONS_HELP: Self = Self(114);
    /// ui.preferences.windows-availability; stable ordinal 144.
    pub const PREFERENCES_WINDOWS_AVAILABILITY: Self = Self(115);
    /// ui.preferences.unknown-availability; stable ordinal 145.
    pub const PREFERENCES_UNKNOWN_AVAILABILITY: Self = Self(116);
    /// ui.preferences.availability-help; stable ordinal 146.
    pub const PREFERENCES_AVAILABILITY_HELP: Self = Self(117);
    /// ui.preferences.observer-label; stable ordinal 147.
    pub const PREFERENCES_OBSERVER_LABEL: Self = Self(118);
    /// ui.preferences.observer-unavailable; stable ordinal 148.
    pub const PREFERENCES_OBSERVER_UNAVAILABLE: Self = Self(119);
    /// ui.preferences.observer-help; stable ordinal 149.
    pub const PREFERENCES_OBSERVER_HELP: Self = Self(120);
    /// ui.preferences.test-notification; stable ordinal 150.
    pub const PREFERENCES_TEST_NOTIFICATION: Self = Self(121);
    /// ui.preferences.notification-test-unavailable; stable ordinal 151.
    pub const PREFERENCES_NOTIFICATION_TEST_UNAVAILABLE: Self = Self(122);
    /// ui.preferences.preferences-heading; stable ordinal 152.
    pub const PREFERENCES_PREFERENCES_HEADING: Self = Self(123);
    /// ui.preferences.tray-label; stable ordinal 153.
    pub const PREFERENCES_TRAY_LABEL: Self = Self(124);
    /// ui.preferences.tray-help; stable ordinal 154.
    pub const PREFERENCES_TRAY_HELP: Self = Self(125);
    /// ui.preferences.sign-in-label; stable ordinal 155.
    pub const PREFERENCES_SIGN_IN_LABEL: Self = Self(126);
    /// ui.preferences.sign-in-help; stable ordinal 156.
    pub const PREFERENCES_SIGN_IN_HELP: Self = Self(127);
    /// ui.preferences.tray-adapter-unavailable; stable ordinal 157.
    pub const PREFERENCES_TRAY_ADAPTER_UNAVAILABLE: Self = Self(128);
    /// ui.preferences.interval-option.automatic; stable ordinal 158.
    pub const PREFERENCES_INTERVAL_OPTION_AUTOMATIC: Self = Self(129);
    /// ui.preferences.interval-option.custom; stable ordinal 162.
    pub const PREFERENCES_INTERVAL_OPTION_CUSTOM: Self = Self(130);
    /// ui.error.page-unavailable; stable ordinal 163.
    pub const ERROR_PAGE_UNAVAILABLE: Self = Self(131);
    /// ui.error.invalid-label; stable ordinal 164.
    pub const ERROR_INVALID_LABEL: Self = Self(132);
    /// ui.error.invalid-host; stable ordinal 165.
    pub const ERROR_INVALID_HOST: Self = Self(133);
    /// ui.error.invalid-port; stable ordinal 166.
    pub const ERROR_INVALID_PORT: Self = Self(134);
    /// ui.error.invalid-selection; stable ordinal 167.
    pub const ERROR_INVALID_SELECTION: Self = Self(135);
    /// ui.feedback.source-cancelled; stable ordinal 169.
    pub const FEEDBACK_SOURCE_CANCELLED: Self = Self(136);
    /// ui.error.invalid-custom-minutes; stable ordinal 170.
    pub const ERROR_INVALID_CUSTOM_MINUTES: Self = Self(137);
    /// ui.error.tray-required; stable ordinal 171.
    pub const ERROR_TRAY_REQUIRED: Self = Self(138);
    /// ui.error.edit-before-select; stable ordinal 172.
    pub const ERROR_EDIT_BEFORE_SELECT: Self = Self(139);
    /// ui.error.editor-closed; stable ordinal 173.
    pub const ERROR_EDITOR_CLOSED: Self = Self(140);
    /// ui.interval.automatic; stable ordinal 174.
    pub const INTERVAL_AUTOMATIC: Self = Self(141);
    /// ui.error.interval-choice; stable ordinal 178.
    pub const ERROR_INTERVAL_CHOICE: Self = Self(142);
    /// ui.feedback.preference-updated; stable ordinal 179.
    pub const FEEDBACK_PREFERENCE_UPDATED: Self = Self(143);
    /// ui.feedback.interval-cancelled; stable ordinal 180.
    pub const FEEDBACK_INTERVAL_CANCELLED: Self = Self(144);
    /// ui.help.invalid-label; stable ordinal 181.
    pub const HELP_INVALID_LABEL: Self = Self(145);
    /// ui.help.invalid-host; stable ordinal 182.
    pub const HELP_INVALID_HOST: Self = Self(146);
    /// ui.help.invalid-port; stable ordinal 183.
    pub const HELP_INVALID_PORT: Self = Self(147);
    /// ui.interval.custom; stable ordinal 184.
    pub const INTERVAL_CUSTOM: Self = Self(148);
    /// ui.fixture.open-repository; stable ordinal 185.
    pub const FIXTURE_OPEN_REPOSITORY: Self = Self(149);
    /// ui.fixture.repository-help; stable ordinal 186.
    pub const FIXTURE_REPOSITORY_HELP: Self = Self(150);
    /// ui.fixture.repository-error; stable ordinal 187.
    pub const FIXTURE_REPOSITORY_ERROR: Self = Self(151);
    /// ui.source.profile-value; stable ordinal 188.
    pub const SOURCE_PROFILE_VALUE: Self = Self(152);
    /// ui.source.preview-note; stable ordinal 189.
    pub const SOURCE_PREVIEW_NOTE: Self = Self(153);
    /// ui.source.add-source; stable ordinal 190.
    pub const SOURCE_ADD_SOURCE: Self = Self(154);
    /// ui.source.remove-source; stable ordinal 191.
    pub const SOURCE_REMOVE_SOURCE: Self = Self(155);
    /// ui.source.review-changes; stable ordinal 192.
    pub const SOURCE_REVIEW_CHANGES: Self = Self(156);
    /// ui.source.review-heading; stable ordinal 193.
    pub const SOURCE_REVIEW_HEADING: Self = Self(157);
    /// ui.source.review-description; stable ordinal 194.
    pub const SOURCE_REVIEW_DESCRIPTION: Self = Self(158);
    /// ui.source.before-label; stable ordinal 195.
    pub const SOURCE_BEFORE_LABEL: Self = Self(159);
    /// ui.source.after-label; stable ordinal 196.
    pub const SOURCE_AFTER_LABEL: Self = Self(160);
    /// ui.source.undo; stable ordinal 197.
    pub const SOURCE_UNDO: Self = Self(161);
    /// ui.source.discard; stable ordinal 198.
    pub const SOURCE_DISCARD: Self = Self(162);
    /// ui.source.close-review; stable ordinal 199.
    pub const SOURCE_CLOSE_REVIEW: Self = Self(163);
    /// ui.source.apply; stable ordinal 200.
    pub const SOURCE_APPLY: Self = Self(164);
    /// ui.source.apply-unavailable; stable ordinal 201.
    pub const SOURCE_APPLY_UNAVAILABLE: Self = Self(165);
    /// ui.source.empty-heading; stable ordinal 202.
    pub const SOURCE_EMPTY_HEADING: Self = Self(166);
    /// ui.source.empty-description; stable ordinal 203.
    pub const SOURCE_EMPTY_DESCRIPTION: Self = Self(167);
    /// ui.source.empty-review; stable ordinal 204.
    pub const SOURCE_EMPTY_REVIEW: Self = Self(168);
    /// ui.source.add-title; stable ordinal 205.
    pub const SOURCE_ADD_TITLE: Self = Self(169);
    /// ui.source.edit-title; stable ordinal 206.
    pub const SOURCE_EDIT_TITLE: Self = Self(170);
    /// ui.source.protocol-option.nts; stable ordinal 207.
    pub const SOURCE_PROTOCOL_OPTION_NTS: Self = Self(171);
    /// ui.source.protocol-option.ntp; stable ordinal 208.
    pub const SOURCE_PROTOCOL_OPTION_NTP: Self = Self(172);
    /// ui.source.protocol-help; stable ordinal 209.
    pub const SOURCE_PROTOCOL_HELP: Self = Self(173);
    /// ui.source.ntp-warning; stable ordinal 210.
    pub const SOURCE_NTP_WARNING: Self = Self(174);
    /// ui.source.ntp-port; stable ordinal 211.
    pub const SOURCE_NTP_PORT: Self = Self(175);
    /// ui.source.port-help; stable ordinal 212.
    pub const SOURCE_PORT_HELP: Self = Self(176);
    /// ui.source.statistics-description; stable ordinal 213.
    pub const SOURCE_STATISTICS_DESCRIPTION: Self = Self(177);
    /// ui.recovery.title; stable ordinal 214.
    pub const RECOVERY_TITLE: Self = Self(178);
    /// ui.recovery.description; stable ordinal 215.
    pub const RECOVERY_DESCRIPTION: Self = Self(179);
    /// ui.recovery.back; stable ordinal 216.
    pub const RECOVERY_BACK: Self = Self(180);
    /// ui.recovery.settings-description; stable ordinal 217.
    pub const RECOVERY_SETTINGS_DESCRIPTION: Self = Self(181);
    /// ui.recovery.context-description; stable ordinal 218.
    pub const RECOVERY_CONTEXT_DESCRIPTION: Self = Self(182);
    /// ui.recovery.open-details; stable ordinal 219.
    pub const RECOVERY_OPEN_DETAILS: Self = Self(183);
    /// ui.recovery.leave-title; stable ordinal 220.
    pub const RECOVERY_LEAVE_TITLE: Self = Self(184);
    /// ui.recovery.leave-description; stable ordinal 221.
    pub const RECOVERY_LEAVE_DESCRIPTION: Self = Self(185);
    /// ui.recovery.leave-keep; stable ordinal 222.
    pub const RECOVERY_LEAVE_KEEP: Self = Self(186);
    /// ui.recovery.leave-discard; stable ordinal 223.
    pub const RECOVERY_LEAVE_DISCARD: Self = Self(187);
    /// ui.recovery.leave-stay; stable ordinal 224.
    pub const RECOVERY_LEAVE_STAY: Self = Self(188);
    /// ui.recovery.roughtime-title; stable ordinal 225.
    pub const RECOVERY_ROUGHTIME_TITLE: Self = Self(189);
    /// ui.recovery.roughtime-description; stable ordinal 226.
    pub const RECOVERY_ROUGHTIME_DESCRIPTION: Self = Self(190);
    /// ui.recovery.no-effect; stable ordinal 227.
    pub const RECOVERY_NO_EFFECT: Self = Self(191);
    /// ui.recovery.unavailable; stable ordinal 228.
    pub const RECOVERY_UNAVAILABLE: Self = Self(192);
    /// ui.recovery.signatures; stable ordinal 229.
    pub const RECOVERY_SIGNATURES: Self = Self(193);
    /// ui.recovery.date; stable ordinal 230.
    pub const RECOVERY_DATE: Self = Self(194);
    /// ui.recovery.handoff; stable ordinal 231.
    pub const RECOVERY_HANDOFF: Self = Self(195);
    /// ui.recovery.unobserved; stable ordinal 232.
    pub const RECOVERY_UNOBSERVED: Self = Self(196);
    /// ui.recovery.no-date; stable ordinal 233.
    pub const RECOVERY_NO_DATE: Self = Self(197);
    /// ui.recovery.show-path; stable ordinal 234.
    pub const RECOVERY_SHOW_PATH: Self = Self(198);
    /// ui.recovery.hide-path; stable ordinal 235.
    pub const RECOVERY_HIDE_PATH: Self = Self(199);
    /// ui.recovery.retained-step; stable ordinal 236.
    pub const RECOVERY_RETAINED_STEP: Self = Self(200);
    /// ui.recovery.date-step; stable ordinal 237.
    pub const RECOVERY_DATE_STEP: Self = Self(201);
    /// ui.recovery.nts-step; stable ordinal 238.
    pub const RECOVERY_NTS_STEP: Self = Self(202);
    /// ui.recovery.agreement-step; stable ordinal 239.
    pub const RECOVERY_AGREEMENT_STEP: Self = Self(203);
    /// ui.recovery.authority-step; stable ordinal 240.
    pub const RECOVERY_AUTHORITY_STEP: Self = Self(204);
    /// ui.recovery.helpers-title; stable ordinal 241.
    pub const RECOVERY_HELPERS_TITLE: Self = Self(205);
    /// ui.recovery.helpers-description; stable ordinal 242.
    pub const RECOVERY_HELPERS_DESCRIPTION: Self = Self(206);
    /// ui.recovery.one-helper-warning; stable ordinal 243.
    pub const RECOVERY_ONE_HELPER_WARNING: Self = Self(207);
    /// ui.recovery.two-helper-warning; stable ordinal 244.
    pub const RECOVERY_TWO_HELPER_WARNING: Self = Self(208);
    /// ui.recovery.count-label; stable ordinal 245.
    pub const RECOVERY_COUNT_LABEL: Self = Self(209);
    /// ui.recovery.count-option.off; stable ordinal 246.
    pub const RECOVERY_COUNT_OPTION_OFF: Self = Self(210);
    /// ui.recovery.count-option.one; stable ordinal 247.
    pub const RECOVERY_COUNT_OPTION_ONE: Self = Self(211);
    /// ui.recovery.count-option.two; stable ordinal 248.
    pub const RECOVERY_COUNT_OPTION_TWO: Self = Self(212);
    /// ui.recovery.first-label; stable ordinal 249.
    pub const RECOVERY_FIRST_LABEL: Self = Self(213);
    /// ui.recovery.second-label; stable ordinal 250.
    pub const RECOVERY_SECOND_LABEL: Self = Self(214);
    /// ui.recovery.host-help; stable ordinal 251.
    pub const RECOVERY_HOST_HELP: Self = Self(215);
    /// ui.recovery.preview; stable ordinal 252.
    pub const RECOVERY_PREVIEW: Self = Self(216);
    /// ui.recovery.cancel; stable ordinal 253.
    pub const RECOVERY_CANCEL: Self = Self(217);
    /// ui.recovery.review-title; stable ordinal 254.
    pub const RECOVERY_REVIEW_TITLE: Self = Self(218);
    /// ui.recovery.off; stable ordinal 255.
    pub const RECOVERY_OFF: Self = Self(219);
    /// ui.recovery.boundary; stable ordinal 256.
    pub const RECOVERY_BOUNDARY: Self = Self(220);
    /// ui.recovery.keep-editing; stable ordinal 257.
    pub const RECOVERY_KEEP_EDITING: Self = Self(221);
    /// ui.recovery.discard; stable ordinal 258.
    pub const RECOVERY_DISCARD: Self = Self(222);
    /// ui.recovery.apply; stable ordinal 259.
    pub const RECOVERY_APPLY: Self = Self(223);
    /// ui.recovery.apply-unavailable; stable ordinal 260.
    pub const RECOVERY_APPLY_UNAVAILABLE: Self = Self(224);
    /// ui.shell.quit-ui; stable ordinal 261.
    pub const SHELL_QUIT_UI: Self = Self(225);
    /// ui.shell.quit-title; stable ordinal 262.
    pub const SHELL_QUIT_TITLE: Self = Self(226);
    /// ui.shell.keep-open; stable ordinal 263.
    pub const SHELL_KEEP_OPEN: Self = Self(227);
    /// ui.shell.manage-service; stable ordinal 264.
    pub const SHELL_MANAGE_SERVICE: Self = Self(228);
    /// ui.shell.quit-only; stable ordinal 265.
    pub const SHELL_QUIT_ONLY: Self = Self(229);
    /// ui.shell.unknown-consequence; stable ordinal 266.
    pub const SHELL_UNKNOWN_CONSEQUENCE: Self = Self(230);
    /// ui.shell.draft-loss-title; stable ordinal 267.
    pub const SHELL_DRAFT_LOSS_TITLE: Self = Self(231);
    /// ui.shell.draft-loss; stable ordinal 268.
    pub const SHELL_DRAFT_LOSS: Self = Self(232);
    /// ui.shell.observer-consequence; stable ordinal 269.
    pub const SHELL_OBSERVER_CONSEQUENCE: Self = Self(233);
    /// ui.shell.stop-path; stable ordinal 270.
    pub const SHELL_STOP_PATH: Self = Self(234);
    /// ui.shell.disable-path; stable ordinal 271.
    pub const SHELL_DISABLE_PATH: Self = Self(235);
    /// ui.shell.separate-management; stable ordinal 272.
    pub const SHELL_SEPARATE_MANAGEMENT: Self = Self(236);
    /// ui.shell.exit-requested; stable ordinal 273.
    pub const SHELL_EXIT_REQUESTED: Self = Self(237);
    /// ui.shell.background-service; stable ordinal 275.
    pub const SHELL_BACKGROUND_SERVICE: Self = Self(238);
    /// ui.shell.notifications; stable ordinal 276.
    pub const SHELL_NOTIFICATIONS: Self = Self(239);
    /// ui.shell.main-content; stable ordinal 277.
    pub const SHELL_MAIN_CONTENT: Self = Self(240);
    /// ui.shell.service-description; stable ordinal 278.
    pub const SHELL_SERVICE_DESCRIPTION: Self = Self(241);
    /// ui.shell.service-summary; stable ordinal 279.
    pub const SHELL_SERVICE_SUMMARY: Self = Self(242);
    /// ui.shell.manage-power; stable ordinal 280.
    pub const SHELL_MANAGE_POWER: Self = Self(243);
    /// ui.shell.service-unavailable; stable ordinal 281.
    pub const SHELL_SERVICE_UNAVAILABLE: Self = Self(244);
    /// ui.shell.diagnose; stable ordinal 282.
    pub const SHELL_DIAGNOSE: Self = Self(245);
    /// ui.shell.diagnose-power; stable ordinal 283.
    pub const SHELL_DIAGNOSE_POWER: Self = Self(246);
    /// ui.shell.diagnose-note; stable ordinal 284.
    pub const SHELL_DIAGNOSE_NOTE: Self = Self(247);
    /// ui.shell.startup-title; stable ordinal 285.
    pub const SHELL_STARTUP_TITLE: Self = Self(248);
    /// ui.shell.startup-note; stable ordinal 286.
    pub const SHELL_STARTUP_NOTE: Self = Self(249);
    /// ui.shell.autostart-on; stable ordinal 287.
    pub const SHELL_AUTOSTART_ON: Self = Self(250);
    /// ui.shell.autostart-off; stable ordinal 288.
    pub const SHELL_AUTOSTART_OFF: Self = Self(251);
    /// ui.shell.disable-note; stable ordinal 289.
    pub const SHELL_DISABLE_NOTE: Self = Self(252);
    /// ui.shell.time-manager-title; stable ordinal 290.
    pub const SHELL_TIME_MANAGER_TITLE: Self = Self(253);
    /// ui.shell.use-windows; stable ordinal 291.
    pub const SHELL_USE_WINDOWS: Self = Self(254);
    /// ui.shell.use-nevrotick; stable ordinal 292.
    pub const SHELL_USE_NEVROTICK: Self = Self(255);
    /// ui.shell.handover-note; stable ordinal 293.
    pub const SHELL_HANDOVER_NOTE: Self = Self(256);
    /// ui.policy.heading; stable ordinal 295.
    pub const POLICY_HEADING: Self = Self(257);
    /// ui.policy.description; stable ordinal 296.
    pub const POLICY_DESCRIPTION: Self = Self(258);
    /// ui.policy.baseline; stable ordinal 297.
    pub const POLICY_BASELINE: Self = Self(259);
    /// ui.policy.preview; stable ordinal 298.
    pub const POLICY_PREVIEW: Self = Self(260);
    /// ui.policy.no-preview; stable ordinal 299.
    pub const POLICY_NO_PREVIEW: Self = Self(261);
    /// ui.policy.nts; stable ordinal 300.
    pub const POLICY_NTS: Self = Self(262);
    /// ui.policy.ntp; stable ordinal 301.
    pub const POLICY_NTP: Self = Self(263);
    /// ui.policy.observe; stable ordinal 302.
    pub const POLICY_OBSERVE: Self = Self(264);
    /// ui.policy.choose; stable ordinal 303.
    pub const POLICY_CHOOSE: Self = Self(265);
    /// ui.policy.edit; stable ordinal 304.
    pub const POLICY_EDIT: Self = Self(266);
    /// ui.policy.review; stable ordinal 305.
    pub const POLICY_REVIEW: Self = Self(267);
    /// ui.policy.keep; stable ordinal 306.
    pub const POLICY_KEEP: Self = Self(268);
    /// ui.policy.discard; stable ordinal 307.
    pub const POLICY_DISCARD: Self = Self(269);
    /// ui.policy.cancel; stable ordinal 308.
    pub const POLICY_CANCEL: Self = Self(270);
    /// ui.policy.choice-label; stable ordinal 309.
    pub const POLICY_CHOICE_LABEL: Self = Self(271);
    /// ui.policy.nts-consequence; stable ordinal 310.
    pub const POLICY_NTS_CONSEQUENCE: Self = Self(272);
    /// ui.policy.ntp-consequence; stable ordinal 311.
    pub const POLICY_NTP_CONSEQUENCE: Self = Self(273);
    /// ui.policy.observe-consequence; stable ordinal 312.
    pub const POLICY_OBSERVE_CONSEQUENCE: Self = Self(274);
    /// ui.policy.authority; stable ordinal 313.
    pub const POLICY_AUTHORITY: Self = Self(275);
    /// ui.policy.apply; stable ordinal 314.
    pub const POLICY_APPLY: Self = Self(276);
    /// ui.policy.apply-unavailable; stable ordinal 315.
    pub const POLICY_APPLY_UNAVAILABLE: Self = Self(277);
    /// ui.policy.fixture-note; stable ordinal 316.
    pub const POLICY_FIXTURE_NOTE: Self = Self(278);
    /// ui.policy.preview-note; stable ordinal 317.
    pub const POLICY_PREVIEW_NOTE: Self = Self(279);
    /// ui.policy.phase-required; stable ordinal 319.
    pub const POLICY_PHASE_REQUIRED: Self = Self(280);
    /// ui.policy.invalid-choice; stable ordinal 320.
    pub const POLICY_INVALID_CHOICE: Self = Self(281);
    /// ui.policy.no-preview-error; stable ordinal 321.
    pub const POLICY_NO_PREVIEW_ERROR: Self = Self(282);
    /// ui.policy.page-required; stable ordinal 322.
    pub const POLICY_PAGE_REQUIRED: Self = Self(283);
    /// ui.inspection.query-label; stable ordinal 323.
    pub const INSPECTION_QUERY_LABEL: Self = Self(284);
    /// ui.inspection.query-placeholder; stable ordinal 324.
    pub const INSPECTION_QUERY_PLACEHOLDER: Self = Self(285);
    /// ui.inspection.query-help; stable ordinal 325.
    pub const INSPECTION_QUERY_HELP: Self = Self(286);
    /// ui.inspection.search; stable ordinal 326.
    pub const INSPECTION_SEARCH: Self = Self(287);
    /// ui.inspection.clear; stable ordinal 327.
    pub const INSPECTION_CLEAR: Self = Self(288);
    /// ui.inspection.all; stable ordinal 328.
    pub const INSPECTION_ALL: Self = Self(289);
    /// ui.inspection.used; stable ordinal 329.
    pub const INSPECTION_USED: Self = Self(290);
    /// ui.inspection.other; stable ordinal 330.
    pub const INSPECTION_OTHER: Self = Self(291);
    /// ui.inspection.roles-unavailable; stable ordinal 331.
    pub const INSPECTION_ROLES_UNAVAILABLE: Self = Self(292);
    /// ui.inspection.no-matches-heading; stable ordinal 332.
    pub const INSPECTION_NO_MATCHES_HEADING: Self = Self(293);
    /// ui.inspection.no-matches; stable ordinal 333.
    pub const INSPECTION_NO_MATCHES: Self = Self(294);
    /// ui.inspection.accepted-query; stable ordinal 334.
    pub const INSPECTION_ACCEPTED_QUERY: Self = Self(295);
    /// ui.inspection.details; stable ordinal 335.
    pub const INSPECTION_DETAILS: Self = Self(296);
    /// ui.inspection.details-heading; stable ordinal 336.
    pub const INSPECTION_DETAILS_HEADING: Self = Self(297);
    /// ui.inspection.close-details; stable ordinal 337.
    pub const INSPECTION_CLOSE_DETAILS: Self = Self(298);
    /// ui.inspection.details-note; stable ordinal 338.
    pub const INSPECTION_DETAILS_NOTE: Self = Self(299);
    /// ui.inspection.hidden-details; stable ordinal 339.
    pub const INSPECTION_HIDDEN_DETAILS: Self = Self(300);
    /// ui.inspection.selected-hidden; stable ordinal 340.
    pub const INSPECTION_SELECTED_HIDDEN: Self = Self(301);
    /// ui.inspection.identity; stable ordinal 341.
    pub const INSPECTION_IDENTITY: Self = Self(302);
    /// ui.inspection.label; stable ordinal 342.
    pub const INSPECTION_LABEL: Self = Self(303);
    /// ui.inspection.protocol; stable ordinal 343.
    pub const INSPECTION_PROTOCOL: Self = Self(304);
    /// ui.inspection.port; stable ordinal 344.
    pub const INSPECTION_PORT: Self = Self(305);
    /// ui.inspection.local-id-note; stable ordinal 345.
    pub const INSPECTION_LOCAL_ID_NOTE: Self = Self(306);
    /// ui.inspection.authentication; stable ordinal 346.
    pub const INSPECTION_AUTHENTICATION: Self = Self(307);
    /// ui.inspection.auth-measurement; stable ordinal 347.
    pub const INSPECTION_AUTH_MEASUREMENT: Self = Self(308);
    /// ui.inspection.role; stable ordinal 348.
    pub const INSPECTION_ROLE: Self = Self(309);
    /// ui.inspection.operator; stable ordinal 349.
    pub const INSPECTION_OPERATOR: Self = Self(310);
    /// ui.inspection.endpoint; stable ordinal 350.
    pub const INSPECTION_ENDPOINT: Self = Self(311);
    /// ui.inspection.vote-identity; stable ordinal 351.
    pub const INSPECTION_VOTE_IDENTITY: Self = Self(312);
    /// ui.inspection.request-status; stable ordinal 352.
    pub const INSPECTION_REQUEST_STATUS: Self = Self(313);
    /// ui.inspection.last-reply; stable ordinal 353.
    pub const INSPECTION_LAST_REPLY: Self = Self(314);
    /// ui.inspection.last-accepted; stable ordinal 354.
    pub const INSPECTION_LAST_ACCEPTED: Self = Self(315);
    /// ui.inspection.offset; stable ordinal 355.
    pub const INSPECTION_OFFSET: Self = Self(316);
    /// ui.inspection.delay; stable ordinal 356.
    pub const INSPECTION_DELAY: Self = Self(317);
    /// ui.inspection.uncertainty; stable ordinal 357.
    pub const INSPECTION_UNCERTAINTY: Self = Self(318);
    /// ui.inspection.certificate; stable ordinal 358.
    pub const INSPECTION_CERTIFICATE: Self = Self(319);
    /// ui.inspection.scale; stable ordinal 359.
    pub const INSPECTION_SCALE: Self = Self(320);
    /// ui.inspection.leap; stable ordinal 360.
    pub const INSPECTION_LEAP: Self = Self(321);
    /// ui.inspection.not-observed; stable ordinal 361.
    pub const INSPECTION_NOT_OBSERVED: Self = Self(322);
    /// ui.inspection.not-measured; stable ordinal 362.
    pub const INSPECTION_NOT_MEASURED: Self = Self(323);
    /// ui.inspection.not-established; stable ordinal 363.
    pub const INSPECTION_NOT_ESTABLISHED: Self = Self(324);
    /// ui.inspection.not-evaluated; stable ordinal 364.
    pub const INSPECTION_NOT_EVALUATED: Self = Self(325);
    /// ui.inspection.not-applicable; stable ordinal 365.
    pub const INSPECTION_NOT_APPLICABLE: Self = Self(326);
    /// ui.inspection.unauthenticated; stable ordinal 366.
    pub const INSPECTION_UNAUTHENTICATED: Self = Self(327);
    /// ui.inspection.nts; stable ordinal 367.
    pub const INSPECTION_NTS: Self = Self(328);
    /// ui.inspection.ntp; stable ordinal 368.
    pub const INSPECTION_NTP: Self = Self(329);
    /// ui.inspection.invalid-query; stable ordinal 369.
    pub const INSPECTION_INVALID_QUERY: Self = Self(330);
    /// ui.inspection.invalid-filter; stable ordinal 370.
    pub const INSPECTION_INVALID_FILTER: Self = Self(331);
    /// ui.inspection.hidden-source; stable ordinal 371.
    pub const INSPECTION_HIDDEN_SOURCE: Self = Self(332);
    /// ui.inspection.page-required; stable ordinal 372.
    pub const INSPECTION_PAGE_REQUIRED: Self = Self(333);
    /// ui.interval.every; stable ordinal 373.
    pub const INTERVAL_EVERY: Self = Self(334);
    /// ui.rust.notifications-off; stable ordinal 374.
    pub const RUST_NOTIFICATIONS_OFF: Self = Self(335);
    /// ui.rust.observer-unconfirmed; stable ordinal 375.
    pub const RUST_OBSERVER_UNCONFIRMED: Self = Self(336);
    /// ui.rust.notification-os-unconfirmed; stable ordinal 376.
    pub const RUST_NOTIFICATION_OS_UNCONFIRMED: Self = Self(337);
    /// ui.rust.notifications-blocked; stable ordinal 377.
    pub const RUST_NOTIFICATIONS_BLOCKED: Self = Self(338);
    /// ui.rust.notifications-native-unavailable; stable ordinal 378.
    pub const RUST_NOTIFICATIONS_NATIVE_UNAVAILABLE: Self = Self(339);
    /// ui.rust.helper-count; stable ordinal 380.
    pub const RUST_HELPER_COUNT: Self = Self(340);
    /// ui.rust.quit-running-title; stable ordinal 381.
    pub const RUST_QUIT_RUNNING_TITLE: Self = Self(341);
    /// ui.rust.quit-stopped-title; stable ordinal 382.
    pub const RUST_QUIT_STOPPED_TITLE: Self = Self(342);
    /// ui.rust.quit-partial-title; stable ordinal 383.
    pub const RUST_QUIT_PARTIAL_TITLE: Self = Self(343);
    /// ui.rust.quit-pending-title; stable ordinal 384.
    pub const RUST_QUIT_PENDING_TITLE: Self = Self(344);
    /// ui.rust.quit-unknown-title; stable ordinal 385.
    pub const RUST_QUIT_UNKNOWN_TITLE: Self = Self(345);
    /// ui.rust.quit-running-consequence; stable ordinal 386.
    pub const RUST_QUIT_RUNNING_CONSEQUENCE: Self = Self(346);
    /// ui.rust.quit-stopped-consequence; stable ordinal 387.
    pub const RUST_QUIT_STOPPED_CONSEQUENCE: Self = Self(347);
    /// ui.rust.quit-partial-consequence; stable ordinal 388.
    pub const RUST_QUIT_PARTIAL_CONSEQUENCE: Self = Self(348);
    /// ui.rust.quit-pending-consequence; stable ordinal 389.
    pub const RUST_QUIT_PENDING_CONSEQUENCE: Self = Self(349);
    /// ui.rust.notification-preference-updated; stable ordinal 393.
    pub const RUST_NOTIFICATION_PREFERENCE_UPDATED: Self = Self(350);
    /// ui.rust.tray-preference-updated; stable ordinal 394.
    pub const RUST_TRAY_PREFERENCE_UPDATED: Self = Self(351);
    /// ui.rust.sign-in-preference-updated; stable ordinal 395.
    pub const RUST_SIGN_IN_PREFERENCE_UPDATED: Self = Self(352);
    /// ui.rust.change-add; stable ordinal 397.
    pub const RUST_CHANGE_ADD: Self = Self(353);
    /// ui.rust.change-edit; stable ordinal 398.
    pub const RUST_CHANGE_EDIT: Self = Self(354);
    /// ui.rust.change-baseline; stable ordinal 399.
    pub const RUST_CHANGE_BASELINE: Self = Self(355);
    /// ui.rust.review-none; stable ordinal 400.
    pub const RUST_REVIEW_NONE: Self = Self(356);
    /// ui.rust.change-remove; stable ordinal 401.
    pub const RUST_CHANGE_REMOVE: Self = Self(357);
    /// ui.rust.source-preview-updated; stable ordinal 402.
    pub const RUST_SOURCE_PREVIEW_UPDATED: Self = Self(358);
    /// ui.rust.source-review-only; stable ordinal 403.
    pub const RUST_SOURCE_REVIEW_ONLY: Self = Self(359);
    /// ui.rust.source-discarded; stable ordinal 404.
    pub const RUST_SOURCE_DISCARDED: Self = Self(360);
    /// ui.rust.source-operation-updated; stable ordinal 405.
    pub const RUST_SOURCE_OPERATION_UPDATED: Self = Self(361);
    /// ui.rust.protocol-choice; stable ordinal 406.
    pub const RUST_PROTOCOL_CHOICE: Self = Self(362);
    /// ui.rust.source-choice; stable ordinal 407.
    pub const RUST_SOURCE_CHOICE: Self = Self(363);
    /// ui.rust.duplicate-source; stable ordinal 408.
    pub const RUST_DUPLICATE_SOURCE: Self = Self(364);
    /// ui.rust.source-capacity; stable ordinal 409.
    pub const RUST_SOURCE_CAPACITY: Self = Self(365);
    /// ui.rust.source-id-exhausted; stable ordinal 410.
    pub const RUST_SOURCE_ID_EXHAUSTED: Self = Self(366);
    /// ui.rust.recovery-page-required; stable ordinal 411.
    pub const RUST_RECOVERY_PAGE_REQUIRED: Self = Self(367);
    /// ui.rust.recovery-leave-required; stable ordinal 412.
    pub const RUST_RECOVERY_LEAVE_REQUIRED: Self = Self(368);
    /// ui.rust.recovery-review-required; stable ordinal 413.
    pub const RUST_RECOVERY_REVIEW_REQUIRED: Self = Self(369);
    /// ui.rust.recovery-edit-required; stable ordinal 414.
    pub const RUST_RECOVERY_EDIT_REQUIRED: Self = Self(370);
    /// ui.rust.back-status; stable ordinal 415.
    pub const RUST_BACK_STATUS: Self = Self(371);
    /// ui.rust.back-sources; stable ordinal 416.
    pub const RUST_BACK_SOURCES: Self = Self(372);
    /// ui.rust.back-statistics; stable ordinal 417.
    pub const RUST_BACK_STATISTICS: Self = Self(373);
    /// ui.rust.back-diagnostics; stable ordinal 418.
    pub const RUST_BACK_DIAGNOSTICS: Self = Self(374);
    /// ui.rust.back-about; stable ordinal 419.
    pub const RUST_BACK_ABOUT: Self = Self(375);
    /// ui.rust.back-settings; stable ordinal 420.
    pub const RUST_BACK_SETTINGS: Self = Self(376);
    /// ui.rust.helper-preview-retained; stable ordinal 421.
    pub const RUST_HELPER_PREVIEW_RETAINED: Self = Self(377);
    /// ui.rust.helper-raw-discarded; stable ordinal 422.
    pub const RUST_HELPER_RAW_DISCARDED: Self = Self(378);
    /// ui.rust.helper-preview-edit; stable ordinal 423.
    pub const RUST_HELPER_PREVIEW_EDIT: Self = Self(379);
    /// ui.rust.helper-preview-discarded; stable ordinal 424.
    pub const RUST_HELPER_PREVIEW_DISCARDED: Self = Self(380);
    /// ui.rust.quit-show-cleanup-failed; stable ordinal 425.
    pub const RUST_QUIT_SHOW_CLEANUP_FAILED: Self = Self(381);
    /// ui.rust.quit-show-failed; stable ordinal 426.
    pub const RUST_QUIT_SHOW_FAILED: Self = Self(382);
    /// ui.rust.close-restored; stable ordinal 427.
    pub const RUST_CLOSE_RESTORED: Self = Self(383);
    /// ui.rust.close-restore-failed; stable ordinal 428.
    pub const RUST_CLOSE_RESTORE_FAILED: Self = Self(384);
    /// ui.rust.exit-restored; stable ordinal 429.
    pub const RUST_EXIT_RESTORED: Self = Self(385);
    /// ui.rust.exit-restore-failed; stable ordinal 430.
    pub const RUST_EXIT_RESTORE_FAILED: Self = Self(386);
    /// ui.rust.tray-unavailable; stable ordinal 431.
    pub const RUST_TRAY_UNAVAILABLE: Self = Self(387);
    /// ui.rust.quit-review-required; stable ordinal 432.
    pub const RUST_QUIT_REVIEW_REQUIRED: Self = Self(388);
    /// ui.rust.quit-cancelled; stable ordinal 433.
    pub const RUST_QUIT_CANCELLED: Self = Self(389);
    /// ui.rust.settings-section; stable ordinal 434.
    pub const RUST_SETTINGS_SECTION: Self = Self(390);
    /// ui.rust.service-action; stable ordinal 435.
    pub const RUST_SERVICE_ACTION: Self = Self(391);
    /// ui.rust.service-unavailable; stable ordinal 436.
    pub const RUST_SERVICE_UNAVAILABLE: Self = Self(392);
    /// ui.diagnostic.back; stable ordinal 437.
    pub const DIAGNOSTIC_BACK: Self = Self(393);
    /// ui.diagnostic.problem; stable ordinal 438.
    pub const DIAGNOSTIC_PROBLEM: Self = Self(394);
    /// ui.diagnostic.effect; stable ordinal 439.
    pub const DIAGNOSTIC_EFFECT: Self = Self(395);
    /// ui.diagnostic.remedy; stable ordinal 440.
    pub const DIAGNOSTIC_REMEDY: Self = Self(396);
    /// ui.diagnostic.service; stable ordinal 441.
    pub const DIAGNOSTIC_SERVICE: Self = Self(397);
    /// ui.diagnostic.events-empty; stable ordinal 442.
    pub const DIAGNOSTIC_EVENTS_EMPTY: Self = Self(398);
    /// ui.diagnostic.events-note; stable ordinal 443.
    pub const DIAGNOSTIC_EVENTS_NOTE: Self = Self(399);
    /// ui.diagnostic.report-open; stable ordinal 444.
    pub const DIAGNOSTIC_REPORT_OPEN: Self = Self(400);
    /// ui.diagnostic.report-title; stable ordinal 445.
    pub const DIAGNOSTIC_REPORT_TITLE: Self = Self(401);
    /// ui.diagnostic.report-note; stable ordinal 446.
    pub const DIAGNOSTIC_REPORT_NOTE: Self = Self(402);
    /// ui.diagnostic.copy; stable ordinal 447.
    pub const DIAGNOSTIC_COPY: Self = Self(403);
    /// ui.diagnostic.export; stable ordinal 448.
    pub const DIAGNOSTIC_EXPORT: Self = Self(404);
    /// ui.diagnostic.export-reason; stable ordinal 449.
    pub const DIAGNOSTIC_EXPORT_REASON: Self = Self(405);
    /// ui.diagnostic.collection-open; stable ordinal 450.
    pub const DIAGNOSTIC_COLLECTION_OPEN: Self = Self(406);
    /// ui.diagnostic.collection-title; stable ordinal 451.
    pub const DIAGNOSTIC_COLLECTION_TITLE: Self = Self(407);
    /// ui.diagnostic.collection-note; stable ordinal 452.
    pub const DIAGNOSTIC_COLLECTION_NOTE: Self = Self(408);
    /// ui.diagnostic.scope; stable ordinal 453.
    pub const DIAGNOSTIC_SCOPE: Self = Self(409);
    /// ui.diagnostic.scope-option.ui; stable ordinal 454.
    pub const DIAGNOSTIC_SCOPE_OPTION_UI: Self = Self(410);
    /// ui.diagnostic.scope-option.service; stable ordinal 455.
    pub const DIAGNOSTIC_SCOPE_OPTION_SERVICE: Self = Self(411);
    /// ui.diagnostic.scope-option.combined; stable ordinal 456.
    pub const DIAGNOSTIC_SCOPE_OPTION_COMBINED: Self = Self(412);
    /// ui.diagnostic.duration; stable ordinal 457.
    pub const DIAGNOSTIC_DURATION: Self = Self(413);
    /// ui.diagnostic.collection-state; stable ordinal 458.
    pub const DIAGNOSTIC_COLLECTION_STATE: Self = Self(414);
    /// ui.diagnostic.effective-scope; stable ordinal 459.
    pub const DIAGNOSTIC_EFFECTIVE_SCOPE: Self = Self(415);
    /// ui.diagnostic.remaining; stable ordinal 460.
    pub const DIAGNOSTIC_REMAINING: Self = Self(416);
    /// ui.diagnostic.unavailable; stable ordinal 461.
    pub const DIAGNOSTIC_UNAVAILABLE: Self = Self(417);
    /// ui.diagnostic.collection-reason; stable ordinal 462.
    pub const DIAGNOSTIC_COLLECTION_REASON: Self = Self(418);
    /// ui.diagnostic.required-events; stable ordinal 463.
    pub const DIAGNOSTIC_REQUIRED_EVENTS: Self = Self(419);
    /// ui.diagnostic.start; stable ordinal 464.
    pub const DIAGNOSTIC_START: Self = Self(420);
    /// ui.diagnostic.stop; stable ordinal 465.
    pub const DIAGNOSTIC_STOP: Self = Self(421);
    /// ui.diagnostic.correction-open; stable ordinal 466.
    pub const DIAGNOSTIC_CORRECTION_OPEN: Self = Self(422);
    /// ui.diagnostic.correction-title; stable ordinal 467.
    pub const DIAGNOSTIC_CORRECTION_TITLE: Self = Self(423);
    /// ui.diagnostic.correction-empty; stable ordinal 468.
    pub const DIAGNOSTIC_CORRECTION_EMPTY: Self = Self(424);
    /// ui.diagnostic.correction-note; stable ordinal 469.
    pub const DIAGNOSTIC_CORRECTION_NOTE: Self = Self(425);
    /// ui.diagnostic.correction-rule; stable ordinal 470.
    pub const DIAGNOSTIC_CORRECTION_RULE: Self = Self(426);
    /// ui.diagnostic.approve; stable ordinal 471.
    pub const DIAGNOSTIC_APPROVE: Self = Self(427);
    /// ui.diagnostic.unavailable-action; stable ordinal 472.
    pub const DIAGNOSTIC_UNAVAILABLE_ACTION: Self = Self(428);
    /// ui.diagnostic.invalid-choice; stable ordinal 473.
    pub const DIAGNOSTIC_INVALID_CHOICE: Self = Self(429);
    /// ui.duration.minutes; stable ordinal 475.
    pub const DURATION_MINUTES: Self = Self(430);
    /// ui.preferences.incidents-heading; stable ordinal 476.
    pub const PREFERENCES_INCIDENTS_HEADING: Self = Self(431);
    /// ui.preferences.incidents-unavailable; stable ordinal 477.
    pub const PREFERENCES_INCIDENTS_UNAVAILABLE: Self = Self(432);
    /// ui.policy.editor-active; stable ordinal 478.
    pub const POLICY_EDITOR_ACTIVE: Self = Self(433);
    /// ui.inspection.editor-active; stable ordinal 479.
    pub const INSPECTION_EDITOR_ACTIVE: Self = Self(434);
    /// ui.source.page-required; stable ordinal 480.
    pub const SOURCE_PAGE_REQUIRED: Self = Self(435);
    /// ui.tray.menu.open; stable ordinal 516.
    pub const TRAY_MENU_OPEN: Self = Self(436);
    /// ui.tray.menu.diagnostics; stable ordinal 518.
    pub const TRAY_MENU_DIAGNOSTICS: Self = Self(437);
    /// ui.status.headline.unknown; stable ordinal 526.
    pub const STATUS_HEADLINE_UNKNOWN: Self = Self(438);
    /// ui.status.headline.initializing; stable ordinal 527.
    pub const STATUS_HEADLINE_INITIALIZING: Self = Self(439);
    /// ui.status.headline.acquiring; stable ordinal 528.
    pub const STATUS_HEADLINE_ACQUIRING: Self = Self(440);
    /// ui.status.headline.observe-only; stable ordinal 529.
    pub const STATUS_HEADLINE_OBSERVE_ONLY: Self = Self(441);
    /// ui.status.headline.tracking; stable ordinal 530.
    pub const STATUS_HEADLINE_TRACKING: Self = Self(442);
    /// ui.status.headline.holdover; stable ordinal 531.
    pub const STATUS_HEADLINE_HOLDOVER: Self = Self(443);
    /// ui.status.headline.expired; stable ordinal 532.
    pub const STATUS_HEADLINE_EXPIRED: Self = Self(444);
    /// ui.status.headline.conflict; stable ordinal 533.
    pub const STATUS_HEADLINE_CONFLICT: Self = Self(445);
    /// ui.status.headline.blocked; stable ordinal 534.
    pub const STATUS_HEADLINE_BLOCKED: Self = Self(446);
    /// ui.status.headline.service-stopped; stable ordinal 535.
    pub const STATUS_HEADLINE_SERVICE_STOPPED: Self = Self(447);
    /// ui.status.headline.windows-managed; stable ordinal 536.
    pub const STATUS_HEADLINE_WINDOWS_MANAGED: Self = Self(448);
    /// ui.status.headline.correction-pending; stable ordinal 537.
    pub const STATUS_HEADLINE_CORRECTION_PENDING: Self = Self(449);
    /// ui.status.headline.uncertain-result; stable ordinal 538.
    pub const STATUS_HEADLINE_UNCERTAIN_RESULT: Self = Self(450);
    /// ui.status.headline.rate-limited; stable ordinal 539.
    pub const STATUS_HEADLINE_RATE_LIMITED: Self = Self(451);
    /// ui.status.headline.reacquiring; stable ordinal 540.
    pub const STATUS_HEADLINE_REACQUIRING: Self = Self(452);
    /// ui.status.headline.leap-guard; stable ordinal 541.
    pub const STATUS_HEADLINE_LEAP_GUARD: Self = Self(453);
    /// ui.status.headline.date-recovery; stable ordinal 542.
    pub const STATUS_HEADLINE_DATE_RECOVERY: Self = Self(454);
    /// ui.status.detail.unavailable; stable ordinal 543.
    pub const STATUS_DETAIL_UNAVAILABLE: Self = Self(455);
    /// ui.status.detail.waiting; stable ordinal 544.
    pub const STATUS_DETAIL_WAITING: Self = Self(456);
    /// ui.status.detail.tracking; stable ordinal 545.
    pub const STATUS_DETAIL_TRACKING: Self = Self(457);
    /// ui.status.detail.holdover; stable ordinal 546.
    pub const STATUS_DETAIL_HOLDOVER: Self = Self(458);
    /// ui.status.detail.withheld; stable ordinal 547.
    pub const STATUS_DETAIL_WITHHELD: Self = Self(459);
    /// ui.status.detail.stopped; stable ordinal 548.
    pub const STATUS_DETAIL_STOPPED: Self = Self(460);
    /// ui.status.detail.windows-managed; stable ordinal 549.
    pub const STATUS_DETAIL_WINDOWS_MANAGED: Self = Self(461);
    /// ui.status.detail.review; stable ordinal 550.
    pub const STATUS_DETAIL_REVIEW: Self = Self(462);
    /// ui.status.detail.uncertain-result; stable ordinal 551.
    pub const STATUS_DETAIL_UNCERTAIN_RESULT: Self = Self(463);
    /// ui.status.detail.rate-limited; stable ordinal 552.
    pub const STATUS_DETAIL_RATE_LIMITED: Self = Self(464);
    /// ui.status.detail.date-recovery; stable ordinal 553.
    pub const STATUS_DETAIL_DATE_RECOVERY: Self = Self(465);
    /// ui.status.evidence.authenticated; stable ordinal 554.
    pub const STATUS_EVIDENCE_AUTHENTICATED: Self = Self(466);
    /// ui.status.evidence.historical; stable ordinal 555.
    pub const STATUS_EVIDENCE_HISTORICAL: Self = Self(467);
    /// ui.status.evidence.agreement-satisfied; stable ordinal 556.
    pub const STATUS_EVIDENCE_AGREEMENT_SATISFIED: Self = Self(468);
    /// ui.status.evidence.current-estimate; stable ordinal 557.
    pub const STATUS_EVIDENCE_CURRENT_ESTIMATE: Self = Self(469);
    /// ui.status.check.sources; stable ordinal 558.
    pub const STATUS_CHECK_SOURCES: Self = Self(470);
    /// ui.status.action.context-required; stable ordinal 559.
    pub const STATUS_ACTION_CONTEXT_REQUIRED: Self = Self(471);
    /// ui.status.profile.label; stable ordinal 560.
    pub const STATUS_PROFILE_LABEL: Self = Self(472);
    /// ui.raw.too-long; stable ordinal 561.
    pub const RAW_TOO_LONG: Self = Self(473);
    /// ui.raw.character; stable ordinal 562.
    pub const RAW_CHARACTER: Self = Self(474);
    /// ui.raw.invalid-option; stable ordinal 563.
    pub const RAW_INVALID_OPTION: Self = Self(475);
    /// ui.raw.context; stable ordinal 564.
    pub const RAW_CONTEXT: Self = Self(476);
    /// ui.feedback.no-notification-submitted; stable ordinal 565.
    pub const FEEDBACK_NO_NOTIFICATION_SUBMITTED: Self = Self(477);
    /// ui.fixture.gui-build-label; stable ordinal 566.
    pub const FIXTURE_GUI_BUILD_LABEL: Self = Self(478);
    /// ui.fixture.text-contract-label; stable ordinal 567.
    pub const FIXTURE_TEXT_CONTRACT_LABEL: Self = Self(479);
    /// ui.preferences.language-label; stable ordinal 568.
    pub const PREFERENCES_LANGUAGE_LABEL: Self = Self(480);
    /// ui.preferences.language-help; stable ordinal 569.
    pub const PREFERENCES_LANGUAGE_HELP: Self = Self(481);
    /// ui.source.list-required; stable ordinal 570.
    pub const SOURCE_LIST_REQUIRED: Self = Self(482);
    /// ui.source.review-required; stable ordinal 571.
    pub const SOURCE_REVIEW_REQUIRED: Self = Self(483);
    /// ui.shell.synchronization-settings; stable ordinal 572.
    pub const SHELL_SYNCHRONIZATION_SETTINGS: Self = Self(484);
    /// ui.shell.time-policy; stable ordinal 573.
    pub const SHELL_TIME_POLICY: Self = Self(485);
    /// ui.shell.time-policy-description; stable ordinal 574.
    pub const SHELL_TIME_POLICY_DESCRIPTION: Self = Self(486);
    /// ui.shell.time-policy-power; stable ordinal 575.
    pub const SHELL_TIME_POLICY_POWER: Self = Self(487);
    /// ui.shell.personal-preferences; stable ordinal 576.
    pub const SHELL_PERSONAL_PREFERENCES: Self = Self(488);
    /// ui.shell.personal-preferences-description; stable ordinal 577.
    pub const SHELL_PERSONAL_PREFERENCES_DESCRIPTION: Self = Self(489);
    /// ui.raw.unavailable; stable ordinal 578.
    pub const RAW_UNAVAILABLE: Self = Self(490);
    /// ui.raw.stale; stable ordinal 579.
    pub const RAW_STALE: Self = Self(491);
    /// ui.raw.action-undo; stable ordinal 580.
    pub const RAW_ACTION_UNDO: Self = Self(492);
    /// ui.raw.action-redo; stable ordinal 581.
    pub const RAW_ACTION_REDO: Self = Self(493);
    /// ui.raw.action-cut; stable ordinal 582.
    pub const RAW_ACTION_CUT: Self = Self(494);
    /// ui.raw.action-copy; stable ordinal 583.
    pub const RAW_ACTION_COPY: Self = Self(495);
    /// ui.raw.action-paste; stable ordinal 584.
    pub const RAW_ACTION_PASTE: Self = Self(496);
    /// ui.raw.action-select-all; stable ordinal 585.
    pub const RAW_ACTION_SELECT_ALL: Self = Self(497);
    /// ui.native-notification.test-title; stable ordinal 586.
    pub const NATIVE_NOTIFICATION_TEST_TITLE: Self = Self(498);
    /// ui.native-notification.test-body; stable ordinal 587.
    pub const NATIVE_NOTIFICATION_TEST_BODY: Self = Self(499);
    /// ui.preferences.notification-windows-allowed; stable ordinal 588.
    pub const PREFERENCES_NOTIFICATION_WINDOWS_ALLOWED: Self = Self(500);
    /// ui.preferences.notification-windows-blocked; stable ordinal 589.
    pub const PREFERENCES_NOTIFICATION_WINDOWS_BLOCKED: Self = Self(501);
    /// ui.preferences.notification-observer-running; stable ordinal 590.
    pub const PREFERENCES_NOTIFICATION_OBSERVER_RUNNING: Self = Self(502);
    /// ui.native-notification.test-accepted; stable ordinal 591.
    pub const NATIVE_NOTIFICATION_TEST_ACCEPTED: Self = Self(503);
    /// ui.native-notification.test-indeterminate; stable ordinal 592.
    pub const NATIVE_NOTIFICATION_TEST_INDETERMINATE: Self = Self(504);
    /// ui.native-notification.test-coalesced; stable ordinal 593.
    pub const NATIVE_NOTIFICATION_TEST_COALESCED: Self = Self(505);
    /// ui.preferences.tray-ready-note; stable ordinal 594.
    pub const PREFERENCES_TRAY_READY_NOTE: Self = Self(506);
    /// ui.native-observer.stop-timeout; stable ordinal 595.
    pub const NATIVE_OBSERVER_STOP_TIMEOUT: Self = Self(507);
    /// ui.native-observer.stop-failed; stable ordinal 596.
    pub const NATIVE_OBSERVER_STOP_FAILED: Self = Self(508);
}

/// Active protected definitions in private dense storage order.
pub const SPECS: &[Spec] = &[
    Spec {
        id: MessageId::FIXTURE_WINDOW_TITLE,
        key: "ui.fixture.window-title",
        ordinal: 2,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_SAMPLE_LABEL,
        key: "ui.fixture.sample-label",
        ordinal: 3,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_LOCAL_BOUNDARY,
        key: "ui.fixture.local-boundary",
        ordinal: 4,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_MEMORY_BOUNDARY,
        key: "ui.fixture.memory-boundary",
        ordinal: 5,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_SIMPLE_MODE,
        key: "ui.fixture.simple-mode",
        ordinal: 6,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_POWER_MODE,
        key: "ui.fixture.power-mode",
        ordinal: 7,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_STATUS_TAB,
        key: "ui.fixture.status-tab",
        ordinal: 8,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_SOURCES_TAB,
        key: "ui.fixture.sources-tab",
        ordinal: 9,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FIXTURE_STATISTICS_TAB,
        key: "ui.fixture.statistics-tab",
        ordinal: 10,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_DIAGNOSTICS_TAB,
        key: "ui.fixture.diagnostics-tab",
        ordinal: 11,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_SETTINGS_TAB,
        key: "ui.fixture.settings-tab",
        ordinal: 12,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_ABOUT_TAB,
        key: "ui.fixture.about-tab",
        ordinal: 13,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_PREVIEW_BADGE,
        key: "ui.fixture.preview-badge",
        ordinal: 14,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_NOT_MEASURED,
        key: "ui.fixture.not-measured",
        ordinal: 15,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_NOT_EVALUATED,
        key: "ui.fixture.not-evaluated",
        ordinal: 16,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_UNAVAILABLE,
        key: "ui.fixture.unavailable",
        ordinal: 17,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_NO_LIVE_CONNECTION,
        key: "ui.fixture.no-live-connection",
        ordinal: 18,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_STATUS_KICKER,
        key: "ui.fixture.status-kicker",
        ordinal: 19,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_STATUS_HEADING,
        key: "ui.fixture.status-heading",
        ordinal: 20,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_STATUS_QUALIFIER,
        key: "ui.fixture.status-qualifier",
        ordinal: 24,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_DIFFERENCE_LABEL,
        key: "ui.fixture.difference-label",
        ordinal: 26,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_DIFFERENCE_NOTE,
        key: "ui.fixture.difference-note",
        ordinal: 27,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_UNCERTAINTY_LABEL,
        key: "ui.fixture.uncertainty-label",
        ordinal: 28,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_UNCERTAINTY_NOTE,
        key: "ui.fixture.uncertainty-note",
        ordinal: 29,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_AUTHENTICATION_LABEL,
        key: "ui.fixture.authentication-label",
        ordinal: 30,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_AUTHENTICATION_VALUE,
        key: "ui.fixture.authentication-value",
        ordinal: 31,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_AUTHENTICATION_NOTE,
        key: "ui.fixture.authentication-note",
        ordinal: 32,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_AGREEMENT_LABEL,
        key: "ui.fixture.agreement-label",
        ordinal: 33,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_AGREEMENT_NOTE,
        key: "ui.fixture.agreement-note",
        ordinal: 34,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_QUALIFIED_LABEL,
        key: "ui.fixture.qualified-label",
        ordinal: 35,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_SYNC_NOW,
        key: "ui.fixture.sync-now",
        ordinal: 36,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_SYNC_UNAVAILABLE,
        key: "ui.fixture.sync-unavailable",
        ordinal: 37,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_SOURCES_HEADING,
        key: "ui.fixture.sources-heading",
        ordinal: 38,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FIXTURE_SOURCES_DESCRIPTION,
        key: "ui.fixture.sources-description",
        ordinal: 39,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FIXTURE_SOURCE_LIST_NOTE,
        key: "ui.fixture.source-list-note",
        ordinal: 42,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FIXTURE_EDIT_SOURCE,
        key: "ui.fixture.edit-source",
        ordinal: 43,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FIXTURE_SELECT_SOURCE,
        key: "ui.fixture.select-source",
        ordinal: 45,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FIXTURE_SELECTED,
        key: "ui.fixture.selected",
        ordinal: 46,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_SERVER_IDENTITY,
        key: "ui.fixture.server-identity",
        ordinal: 47,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_DISPLAY_NAME,
        key: "ui.fixture.display-name",
        ordinal: 48,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_NTS_PORT,
        key: "ui.fixture.nts-port",
        ordinal: 49,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_PROTOCOL_LABEL,
        key: "ui.fixture.protocol-label",
        ordinal: 50,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_NTS_PREVIEW,
        key: "ui.fixture.nts-preview",
        ordinal: 51,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_SOURCE_ROLE,
        key: "ui.fixture.source-role",
        ordinal: 52,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FIXTURE_LAST_REPLY,
        key: "ui.fixture.last-reply",
        ordinal: 53,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_EDITOR_DESCRIPTION,
        key: "ui.fixture.editor-description",
        ordinal: 55,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FIXTURE_LABEL_HELP,
        key: "ui.fixture.label-help",
        ordinal: 56,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FIXTURE_HOST_HELP,
        key: "ui.fixture.host-help",
        ordinal: 57,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FIXTURE_EDITOR_UNAVAILABLE,
        key: "ui.fixture.editor-unavailable",
        ordinal: 60,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FIXTURE_UPDATE_PREVIEW,
        key: "ui.fixture.update-preview",
        ordinal: 61,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FIXTURE_CANCEL,
        key: "ui.fixture.cancel",
        ordinal: 62,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FIXTURE_STATISTICS_HEADING,
        key: "ui.fixture.statistics-heading",
        ordinal: 63,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_STATISTICS_DESCRIPTION,
        key: "ui.fixture.statistics-description",
        ordinal: 64,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_STATISTICS_EMPTY_TITLE,
        key: "ui.fixture.statistics-empty-title",
        ordinal: 65,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_ONE_SNAPSHOT,
        key: "ui.fixture.one-snapshot",
        ordinal: 67,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_OFFSET_LABEL,
        key: "ui.fixture.offset-label",
        ordinal: 68,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_POSITIVE_OFFSET_NOTE,
        key: "ui.fixture.positive-offset-note",
        ordinal: 69,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_DIAGNOSTICS_HEADING,
        key: "ui.fixture.diagnostics-heading",
        ordinal: 70,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_DIAGNOSTICS_DESCRIPTION,
        key: "ui.fixture.diagnostics-description",
        ordinal: 71,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_OBSERVATION_DESCRIPTION,
        key: "ui.fixture.observation-description",
        ordinal: 73,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_FRESHNESS_HEADING,
        key: "ui.fixture.freshness-heading",
        ordinal: 74,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_FRESHNESS_NOTE,
        key: "ui.fixture.freshness-note",
        ordinal: 75,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_APPLIED_LABEL,
        key: "ui.fixture.applied-label",
        ordinal: 76,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_NO_CORRECTION,
        key: "ui.fixture.no-correction",
        ordinal: 77,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_SNAPSHOT_LABEL,
        key: "ui.fixture.snapshot-label",
        ordinal: 78,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_FIXTURE_SNAPSHOT,
        key: "ui.fixture.fixture-snapshot",
        ordinal: 79,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_DIAGNOSTICS_EVENTS,
        key: "ui.fixture.diagnostics-events",
        ordinal: 80,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_SETTINGS_DESCRIPTION,
        key: "ui.fixture.settings-description",
        ordinal: 85,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_SCHEDULE_HEADING,
        key: "ui.fixture.schedule-heading",
        ordinal: 86,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::FIXTURE_INTERVAL_LABEL,
        key: "ui.fixture.interval-label",
        ordinal: 87,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::FIXTURE_SERVICE_HEADING,
        key: "ui.fixture.service-heading",
        ordinal: 90,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_SERVICE_DESCRIPTION,
        key: "ui.fixture.service-description",
        ordinal: 91,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_RUNTIME_LABEL,
        key: "ui.fixture.runtime-label",
        ordinal: 92,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_RUNTIME_VALUE,
        key: "ui.fixture.runtime-value",
        ordinal: 93,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_STARTUP_LABEL,
        key: "ui.fixture.startup-label",
        ordinal: 94,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_STARTUP_VALUE,
        key: "ui.fixture.startup-value",
        ordinal: 95,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_OWNER_LABEL,
        key: "ui.fixture.owner-label",
        ordinal: 96,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_OWNER_VALUE,
        key: "ui.fixture.owner-value",
        ordinal: 97,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_SYNCHRONIZATION_LABEL,
        key: "ui.fixture.synchronization-label",
        ordinal: 98,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_SYNCHRONIZATION_VALUE,
        key: "ui.fixture.synchronization-value",
        ordinal: 99,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::FIXTURE_ADMIN_REQUIRED,
        key: "ui.fixture.admin-required",
        ordinal: 100,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::FIXTURE_SERVICE_PROTECTED_NOTE,
        key: "ui.fixture.service-protected-note",
        ordinal: 101,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_START_SERVICE,
        key: "ui.fixture.start-service",
        ordinal: 102,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_STOP_SERVICE,
        key: "ui.fixture.stop-service",
        ordinal: 103,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_RESTART_SERVICE,
        key: "ui.fixture.restart-service",
        ordinal: 104,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_DISABLE_SERVICE,
        key: "ui.fixture.disable-service",
        ordinal: 105,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_LIFECYCLE_NOTE,
        key: "ui.fixture.lifecycle-note",
        ordinal: 106,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::FIXTURE_ABOUT_HEADING,
        key: "ui.fixture.about-heading",
        ordinal: 111,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_ABOUT_TAGLINE,
        key: "ui.fixture.about-tagline",
        ordinal: 112,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_ABOUT_TITLE,
        key: "ui.fixture.about-title",
        ordinal: 113,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_ABOUT_DESCRIPTION,
        key: "ui.fixture.about-description",
        ordinal: 114,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_ABOUT_STAGE,
        key: "ui.fixture.about-stage",
        ordinal: 115,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_AUTHOR_LABEL,
        key: "ui.fixture.author-label",
        ordinal: 116,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_REPOSITORY_LABEL,
        key: "ui.fixture.repository-label",
        ordinal: 118,
        kind: Kind::Text,
        group: Some(Group::Notices),
    },
    Spec {
        id: MessageId::FIXTURE_VERSION_LABEL,
        key: "ui.fixture.version-label",
        ordinal: 120,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_VERSION_VALUE,
        key: "ui.fixture.version-value",
        ordinal: 121,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_PLATFORM_LABEL,
        key: "ui.fixture.platform-label",
        ordinal: 122,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_CODE_LICENSE_LABEL,
        key: "ui.fixture.code-license-label",
        ordinal: 124,
        kind: Kind::Text,
        group: Some(Group::Notices),
    },
    Spec {
        id: MessageId::FIXTURE_TECHNOLOGIES_HEADING,
        key: "ui.fixture.technologies-heading",
        ordinal: 126,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_TECHNOLOGIES_DESCRIPTION,
        key: "ui.fixture.technologies-description",
        ordinal: 127,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_CHRONY_NOTE,
        key: "ui.fixture.chrony-note",
        ordinal: 128,
        kind: Kind::Text,
        group: Some(Group::Notices),
    },
    Spec {
        id: MessageId::FIXTURE_PRIVACY_HEADING,
        key: "ui.fixture.privacy-heading",
        ordinal: 129,
        kind: Kind::Text,
        group: Some(Group::Notices),
    },
    Spec {
        id: MessageId::FIXTURE_PRIVACY_DESCRIPTION,
        key: "ui.fixture.privacy-description",
        ordinal: 130,
        kind: Kind::Text,
        group: Some(Group::Notices),
    },
    Spec {
        id: MessageId::FIXTURE_ARTWORK_HEADING,
        key: "ui.fixture.artwork-heading",
        ordinal: 131,
        kind: Kind::Text,
        group: Some(Group::Notices),
    },
    Spec {
        id: MessageId::FIXTURE_ARTWORK_DESCRIPTION,
        key: "ui.fixture.artwork-description",
        ordinal: 132,
        kind: Kind::Text,
        group: Some(Group::Notices),
    },
    Spec {
        id: MessageId::PREFERENCES_SCHEDULE_DESCRIPTION,
        key: "ui.preferences.schedule-description",
        ordinal: 134,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::PREFERENCES_SCHEDULING_NOTE,
        key: "ui.preferences.scheduling-note",
        ordinal: 135,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::PREFERENCES_CUSTOM_LABEL,
        key: "ui.preferences.custom-label",
        ordinal: 136,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::PREFERENCES_CUSTOM_HELP,
        key: "ui.preferences.custom-help",
        ordinal: 137,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::PREFERENCES_ACCEPTED_INTERVAL,
        key: "ui.preferences.accepted-interval",
        ordinal: 138,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::PREFERENCES_PREFERENCES_BOUNDARY,
        key: "ui.preferences.preferences-boundary",
        ordinal: 139,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::PREFERENCES_HOST_UNAVAILABLE,
        key: "ui.preferences.host-unavailable",
        ordinal: 140,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::PREFERENCES_NOTIFICATIONS_HEADING,
        key: "ui.preferences.notifications-heading",
        ordinal: 141,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_NOTIFICATIONS_LABEL,
        key: "ui.preferences.notifications-label",
        ordinal: 142,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_NOTIFICATIONS_HELP,
        key: "ui.preferences.notifications-help",
        ordinal: 143,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_WINDOWS_AVAILABILITY,
        key: "ui.preferences.windows-availability",
        ordinal: 144,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_UNKNOWN_AVAILABILITY,
        key: "ui.preferences.unknown-availability",
        ordinal: 145,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_AVAILABILITY_HELP,
        key: "ui.preferences.availability-help",
        ordinal: 146,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_OBSERVER_LABEL,
        key: "ui.preferences.observer-label",
        ordinal: 147,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_OBSERVER_UNAVAILABLE,
        key: "ui.preferences.observer-unavailable",
        ordinal: 148,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_OBSERVER_HELP,
        key: "ui.preferences.observer-help",
        ordinal: 149,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_TEST_NOTIFICATION,
        key: "ui.preferences.test-notification",
        ordinal: 150,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_NOTIFICATION_TEST_UNAVAILABLE,
        key: "ui.preferences.notification-test-unavailable",
        ordinal: 151,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_PREFERENCES_HEADING,
        key: "ui.preferences.preferences-heading",
        ordinal: 152,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::PREFERENCES_TRAY_LABEL,
        key: "ui.preferences.tray-label",
        ordinal: 153,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::PREFERENCES_TRAY_HELP,
        key: "ui.preferences.tray-help",
        ordinal: 154,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::PREFERENCES_SIGN_IN_LABEL,
        key: "ui.preferences.sign-in-label",
        ordinal: 155,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::PREFERENCES_SIGN_IN_HELP,
        key: "ui.preferences.sign-in-help",
        ordinal: 156,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::PREFERENCES_TRAY_ADAPTER_UNAVAILABLE,
        key: "ui.preferences.tray-adapter-unavailable",
        ordinal: 157,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::PREFERENCES_INTERVAL_OPTION_AUTOMATIC,
        key: "ui.preferences.interval-option.automatic",
        ordinal: 158,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::PREFERENCES_INTERVAL_OPTION_CUSTOM,
        key: "ui.preferences.interval-option.custom",
        ordinal: 162,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::ERROR_PAGE_UNAVAILABLE,
        key: "ui.error.page-unavailable",
        ordinal: 163,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::ERROR_INVALID_LABEL,
        key: "ui.error.invalid-label",
        ordinal: 164,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::ERROR_INVALID_HOST,
        key: "ui.error.invalid-host",
        ordinal: 165,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::ERROR_INVALID_PORT,
        key: "ui.error.invalid-port",
        ordinal: 166,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::ERROR_INVALID_SELECTION,
        key: "ui.error.invalid-selection",
        ordinal: 167,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FEEDBACK_SOURCE_CANCELLED,
        key: "ui.feedback.source-cancelled",
        ordinal: 169,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::ERROR_INVALID_CUSTOM_MINUTES,
        key: "ui.error.invalid-custom-minutes",
        ordinal: 170,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::ERROR_TRAY_REQUIRED,
        key: "ui.error.tray-required",
        ordinal: 171,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::ERROR_EDIT_BEFORE_SELECT,
        key: "ui.error.edit-before-select",
        ordinal: 172,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::ERROR_EDITOR_CLOSED,
        key: "ui.error.editor-closed",
        ordinal: 173,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INTERVAL_AUTOMATIC,
        key: "ui.interval.automatic",
        ordinal: 174,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::ERROR_INTERVAL_CHOICE,
        key: "ui.error.interval-choice",
        ordinal: 178,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::FEEDBACK_PREFERENCE_UPDATED,
        key: "ui.feedback.preference-updated",
        ordinal: 179,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::FEEDBACK_INTERVAL_CANCELLED,
        key: "ui.feedback.interval-cancelled",
        ordinal: 180,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::HELP_INVALID_LABEL,
        key: "ui.help.invalid-label",
        ordinal: 181,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::HELP_INVALID_HOST,
        key: "ui.help.invalid-host",
        ordinal: 182,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::HELP_INVALID_PORT,
        key: "ui.help.invalid-port",
        ordinal: 183,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INTERVAL_CUSTOM,
        key: "ui.interval.custom",
        ordinal: 184,
        kind: Kind::Minutes,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::FIXTURE_OPEN_REPOSITORY,
        key: "ui.fixture.open-repository",
        ordinal: 185,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_REPOSITORY_HELP,
        key: "ui.fixture.repository-help",
        ordinal: 186,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_REPOSITORY_ERROR,
        key: "ui.fixture.repository-error",
        ordinal: 187,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::SOURCE_PROFILE_VALUE,
        key: "ui.source.profile-value",
        ordinal: 188,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_PREVIEW_NOTE,
        key: "ui.source.preview-note",
        ordinal: 189,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_ADD_SOURCE,
        key: "ui.source.add-source",
        ordinal: 190,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_REMOVE_SOURCE,
        key: "ui.source.remove-source",
        ordinal: 191,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_REVIEW_CHANGES,
        key: "ui.source.review-changes",
        ordinal: 192,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_REVIEW_HEADING,
        key: "ui.source.review-heading",
        ordinal: 193,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_REVIEW_DESCRIPTION,
        key: "ui.source.review-description",
        ordinal: 194,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_BEFORE_LABEL,
        key: "ui.source.before-label",
        ordinal: 195,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_AFTER_LABEL,
        key: "ui.source.after-label",
        ordinal: 196,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_UNDO,
        key: "ui.source.undo",
        ordinal: 197,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_DISCARD,
        key: "ui.source.discard",
        ordinal: 198,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_CLOSE_REVIEW,
        key: "ui.source.close-review",
        ordinal: 199,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_APPLY,
        key: "ui.source.apply",
        ordinal: 200,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_APPLY_UNAVAILABLE,
        key: "ui.source.apply-unavailable",
        ordinal: 201,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_EMPTY_HEADING,
        key: "ui.source.empty-heading",
        ordinal: 202,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_EMPTY_DESCRIPTION,
        key: "ui.source.empty-description",
        ordinal: 203,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_EMPTY_REVIEW,
        key: "ui.source.empty-review",
        ordinal: 204,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_ADD_TITLE,
        key: "ui.source.add-title",
        ordinal: 205,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_EDIT_TITLE,
        key: "ui.source.edit-title",
        ordinal: 206,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_PROTOCOL_OPTION_NTS,
        key: "ui.source.protocol-option.nts",
        ordinal: 207,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_PROTOCOL_OPTION_NTP,
        key: "ui.source.protocol-option.ntp",
        ordinal: 208,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_PROTOCOL_HELP,
        key: "ui.source.protocol-help",
        ordinal: 209,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_NTP_WARNING,
        key: "ui.source.ntp-warning",
        ordinal: 210,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_NTP_PORT,
        key: "ui.source.ntp-port",
        ordinal: 211,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_PORT_HELP,
        key: "ui.source.port-help",
        ordinal: 212,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_STATISTICS_DESCRIPTION,
        key: "ui.source.statistics-description",
        ordinal: 213,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RECOVERY_TITLE,
        key: "ui.recovery.title",
        ordinal: 214,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_DESCRIPTION,
        key: "ui.recovery.description",
        ordinal: 215,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_BACK,
        key: "ui.recovery.back",
        ordinal: 216,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_SETTINGS_DESCRIPTION,
        key: "ui.recovery.settings-description",
        ordinal: 217,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_CONTEXT_DESCRIPTION,
        key: "ui.recovery.context-description",
        ordinal: 218,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_OPEN_DETAILS,
        key: "ui.recovery.open-details",
        ordinal: 219,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_LEAVE_TITLE,
        key: "ui.recovery.leave-title",
        ordinal: 220,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_LEAVE_DESCRIPTION,
        key: "ui.recovery.leave-description",
        ordinal: 221,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_LEAVE_KEEP,
        key: "ui.recovery.leave-keep",
        ordinal: 222,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_LEAVE_DISCARD,
        key: "ui.recovery.leave-discard",
        ordinal: 223,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_LEAVE_STAY,
        key: "ui.recovery.leave-stay",
        ordinal: 224,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_ROUGHTIME_TITLE,
        key: "ui.recovery.roughtime-title",
        ordinal: 225,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_ROUGHTIME_DESCRIPTION,
        key: "ui.recovery.roughtime-description",
        ordinal: 226,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_NO_EFFECT,
        key: "ui.recovery.no-effect",
        ordinal: 227,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_UNAVAILABLE,
        key: "ui.recovery.unavailable",
        ordinal: 228,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_SIGNATURES,
        key: "ui.recovery.signatures",
        ordinal: 229,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_DATE,
        key: "ui.recovery.date",
        ordinal: 230,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_HANDOFF,
        key: "ui.recovery.handoff",
        ordinal: 231,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_UNOBSERVED,
        key: "ui.recovery.unobserved",
        ordinal: 232,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_NO_DATE,
        key: "ui.recovery.no-date",
        ordinal: 233,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_SHOW_PATH,
        key: "ui.recovery.show-path",
        ordinal: 234,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_HIDE_PATH,
        key: "ui.recovery.hide-path",
        ordinal: 235,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_RETAINED_STEP,
        key: "ui.recovery.retained-step",
        ordinal: 236,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_DATE_STEP,
        key: "ui.recovery.date-step",
        ordinal: 237,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_NTS_STEP,
        key: "ui.recovery.nts-step",
        ordinal: 238,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_AGREEMENT_STEP,
        key: "ui.recovery.agreement-step",
        ordinal: 239,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_AUTHORITY_STEP,
        key: "ui.recovery.authority-step",
        ordinal: 240,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_HELPERS_TITLE,
        key: "ui.recovery.helpers-title",
        ordinal: 241,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_HELPERS_DESCRIPTION,
        key: "ui.recovery.helpers-description",
        ordinal: 242,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_ONE_HELPER_WARNING,
        key: "ui.recovery.one-helper-warning",
        ordinal: 243,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_TWO_HELPER_WARNING,
        key: "ui.recovery.two-helper-warning",
        ordinal: 244,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_COUNT_LABEL,
        key: "ui.recovery.count-label",
        ordinal: 245,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_COUNT_OPTION_OFF,
        key: "ui.recovery.count-option.off",
        ordinal: 246,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_COUNT_OPTION_ONE,
        key: "ui.recovery.count-option.one",
        ordinal: 247,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_COUNT_OPTION_TWO,
        key: "ui.recovery.count-option.two",
        ordinal: 248,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_FIRST_LABEL,
        key: "ui.recovery.first-label",
        ordinal: 249,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_SECOND_LABEL,
        key: "ui.recovery.second-label",
        ordinal: 250,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_HOST_HELP,
        key: "ui.recovery.host-help",
        ordinal: 251,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_PREVIEW,
        key: "ui.recovery.preview",
        ordinal: 252,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_CANCEL,
        key: "ui.recovery.cancel",
        ordinal: 253,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_REVIEW_TITLE,
        key: "ui.recovery.review-title",
        ordinal: 254,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_OFF,
        key: "ui.recovery.off",
        ordinal: 255,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_BOUNDARY,
        key: "ui.recovery.boundary",
        ordinal: 256,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_KEEP_EDITING,
        key: "ui.recovery.keep-editing",
        ordinal: 257,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_DISCARD,
        key: "ui.recovery.discard",
        ordinal: 258,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_APPLY,
        key: "ui.recovery.apply",
        ordinal: 259,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RECOVERY_APPLY_UNAVAILABLE,
        key: "ui.recovery.apply-unavailable",
        ordinal: 260,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::SHELL_QUIT_UI,
        key: "ui.shell.quit-ui",
        ordinal: 261,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::SHELL_QUIT_TITLE,
        key: "ui.shell.quit-title",
        ordinal: 262,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::SHELL_KEEP_OPEN,
        key: "ui.shell.keep-open",
        ordinal: 263,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::SHELL_MANAGE_SERVICE,
        key: "ui.shell.manage-service",
        ordinal: 264,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::SHELL_QUIT_ONLY,
        key: "ui.shell.quit-only",
        ordinal: 265,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::SHELL_UNKNOWN_CONSEQUENCE,
        key: "ui.shell.unknown-consequence",
        ordinal: 266,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::SHELL_DRAFT_LOSS_TITLE,
        key: "ui.shell.draft-loss-title",
        ordinal: 267,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::SHELL_DRAFT_LOSS,
        key: "ui.shell.draft-loss",
        ordinal: 268,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::SHELL_OBSERVER_CONSEQUENCE,
        key: "ui.shell.observer-consequence",
        ordinal: 269,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::SHELL_STOP_PATH,
        key: "ui.shell.stop-path",
        ordinal: 270,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::SHELL_DISABLE_PATH,
        key: "ui.shell.disable-path",
        ordinal: 271,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::SHELL_SEPARATE_MANAGEMENT,
        key: "ui.shell.separate-management",
        ordinal: 272,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::SHELL_EXIT_REQUESTED,
        key: "ui.shell.exit-requested",
        ordinal: 273,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::SHELL_BACKGROUND_SERVICE,
        key: "ui.shell.background-service",
        ordinal: 275,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_NOTIFICATIONS,
        key: "ui.shell.notifications",
        ordinal: 276,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::SHELL_MAIN_CONTENT,
        key: "ui.shell.main-content",
        ordinal: 277,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_SERVICE_DESCRIPTION,
        key: "ui.shell.service-description",
        ordinal: 278,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_SERVICE_SUMMARY,
        key: "ui.shell.service-summary",
        ordinal: 279,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_MANAGE_POWER,
        key: "ui.shell.manage-power",
        ordinal: 280,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_SERVICE_UNAVAILABLE,
        key: "ui.shell.service-unavailable",
        ordinal: 281,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_DIAGNOSE,
        key: "ui.shell.diagnose",
        ordinal: 282,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_DIAGNOSE_POWER,
        key: "ui.shell.diagnose-power",
        ordinal: 283,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_DIAGNOSE_NOTE,
        key: "ui.shell.diagnose-note",
        ordinal: 284,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_STARTUP_TITLE,
        key: "ui.shell.startup-title",
        ordinal: 285,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_STARTUP_NOTE,
        key: "ui.shell.startup-note",
        ordinal: 286,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_AUTOSTART_ON,
        key: "ui.shell.autostart-on",
        ordinal: 287,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_AUTOSTART_OFF,
        key: "ui.shell.autostart-off",
        ordinal: 288,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_DISABLE_NOTE,
        key: "ui.shell.disable-note",
        ordinal: 289,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_TIME_MANAGER_TITLE,
        key: "ui.shell.time-manager-title",
        ordinal: 290,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_USE_WINDOWS,
        key: "ui.shell.use-windows",
        ordinal: 291,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_USE_NEVROTICK,
        key: "ui.shell.use-nevrotick",
        ordinal: 292,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_HANDOVER_NOTE,
        key: "ui.shell.handover-note",
        ordinal: 293,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::POLICY_HEADING,
        key: "ui.policy.heading",
        ordinal: 295,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_DESCRIPTION,
        key: "ui.policy.description",
        ordinal: 296,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_BASELINE,
        key: "ui.policy.baseline",
        ordinal: 297,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_PREVIEW,
        key: "ui.policy.preview",
        ordinal: 298,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_NO_PREVIEW,
        key: "ui.policy.no-preview",
        ordinal: 299,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_NTS,
        key: "ui.policy.nts",
        ordinal: 300,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_NTP,
        key: "ui.policy.ntp",
        ordinal: 301,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_OBSERVE,
        key: "ui.policy.observe",
        ordinal: 302,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_CHOOSE,
        key: "ui.policy.choose",
        ordinal: 303,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_EDIT,
        key: "ui.policy.edit",
        ordinal: 304,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_REVIEW,
        key: "ui.policy.review",
        ordinal: 305,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_KEEP,
        key: "ui.policy.keep",
        ordinal: 306,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_DISCARD,
        key: "ui.policy.discard",
        ordinal: 307,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_CANCEL,
        key: "ui.policy.cancel",
        ordinal: 308,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_CHOICE_LABEL,
        key: "ui.policy.choice-label",
        ordinal: 309,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_NTS_CONSEQUENCE,
        key: "ui.policy.nts-consequence",
        ordinal: 310,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_NTP_CONSEQUENCE,
        key: "ui.policy.ntp-consequence",
        ordinal: 311,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_OBSERVE_CONSEQUENCE,
        key: "ui.policy.observe-consequence",
        ordinal: 312,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_AUTHORITY,
        key: "ui.policy.authority",
        ordinal: 313,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_APPLY,
        key: "ui.policy.apply",
        ordinal: 314,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_APPLY_UNAVAILABLE,
        key: "ui.policy.apply-unavailable",
        ordinal: 315,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_FIXTURE_NOTE,
        key: "ui.policy.fixture-note",
        ordinal: 316,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_PREVIEW_NOTE,
        key: "ui.policy.preview-note",
        ordinal: 317,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_PHASE_REQUIRED,
        key: "ui.policy.phase-required",
        ordinal: 319,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_INVALID_CHOICE,
        key: "ui.policy.invalid-choice",
        ordinal: 320,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_NO_PREVIEW_ERROR,
        key: "ui.policy.no-preview-error",
        ordinal: 321,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::POLICY_PAGE_REQUIRED,
        key: "ui.policy.page-required",
        ordinal: 322,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::INSPECTION_QUERY_LABEL,
        key: "ui.inspection.query-label",
        ordinal: 323,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_QUERY_PLACEHOLDER,
        key: "ui.inspection.query-placeholder",
        ordinal: 324,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_QUERY_HELP,
        key: "ui.inspection.query-help",
        ordinal: 325,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_SEARCH,
        key: "ui.inspection.search",
        ordinal: 326,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_CLEAR,
        key: "ui.inspection.clear",
        ordinal: 327,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_ALL,
        key: "ui.inspection.all",
        ordinal: 328,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_USED,
        key: "ui.inspection.used",
        ordinal: 329,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_OTHER,
        key: "ui.inspection.other",
        ordinal: 330,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_ROLES_UNAVAILABLE,
        key: "ui.inspection.roles-unavailable",
        ordinal: 331,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_NO_MATCHES_HEADING,
        key: "ui.inspection.no-matches-heading",
        ordinal: 332,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_NO_MATCHES,
        key: "ui.inspection.no-matches",
        ordinal: 333,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_ACCEPTED_QUERY,
        key: "ui.inspection.accepted-query",
        ordinal: 334,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_DETAILS,
        key: "ui.inspection.details",
        ordinal: 335,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_DETAILS_HEADING,
        key: "ui.inspection.details-heading",
        ordinal: 336,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_CLOSE_DETAILS,
        key: "ui.inspection.close-details",
        ordinal: 337,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_DETAILS_NOTE,
        key: "ui.inspection.details-note",
        ordinal: 338,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_HIDDEN_DETAILS,
        key: "ui.inspection.hidden-details",
        ordinal: 339,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_SELECTED_HIDDEN,
        key: "ui.inspection.selected-hidden",
        ordinal: 340,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_IDENTITY,
        key: "ui.inspection.identity",
        ordinal: 341,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_LABEL,
        key: "ui.inspection.label",
        ordinal: 342,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_PROTOCOL,
        key: "ui.inspection.protocol",
        ordinal: 343,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_PORT,
        key: "ui.inspection.port",
        ordinal: 344,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_LOCAL_ID_NOTE,
        key: "ui.inspection.local-id-note",
        ordinal: 345,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_AUTHENTICATION,
        key: "ui.inspection.authentication",
        ordinal: 346,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_AUTH_MEASUREMENT,
        key: "ui.inspection.auth-measurement",
        ordinal: 347,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_ROLE,
        key: "ui.inspection.role",
        ordinal: 348,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_OPERATOR,
        key: "ui.inspection.operator",
        ordinal: 349,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_ENDPOINT,
        key: "ui.inspection.endpoint",
        ordinal: 350,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_VOTE_IDENTITY,
        key: "ui.inspection.vote-identity",
        ordinal: 351,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_REQUEST_STATUS,
        key: "ui.inspection.request-status",
        ordinal: 352,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_LAST_REPLY,
        key: "ui.inspection.last-reply",
        ordinal: 353,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_LAST_ACCEPTED,
        key: "ui.inspection.last-accepted",
        ordinal: 354,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_OFFSET,
        key: "ui.inspection.offset",
        ordinal: 355,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_DELAY,
        key: "ui.inspection.delay",
        ordinal: 356,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_UNCERTAINTY,
        key: "ui.inspection.uncertainty",
        ordinal: 357,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_CERTIFICATE,
        key: "ui.inspection.certificate",
        ordinal: 358,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_SCALE,
        key: "ui.inspection.scale",
        ordinal: 359,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_LEAP,
        key: "ui.inspection.leap",
        ordinal: 360,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_NOT_OBSERVED,
        key: "ui.inspection.not-observed",
        ordinal: 361,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_NOT_MEASURED,
        key: "ui.inspection.not-measured",
        ordinal: 362,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_NOT_ESTABLISHED,
        key: "ui.inspection.not-established",
        ordinal: 363,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_NOT_EVALUATED,
        key: "ui.inspection.not-evaluated",
        ordinal: 364,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_NOT_APPLICABLE,
        key: "ui.inspection.not-applicable",
        ordinal: 365,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_UNAUTHENTICATED,
        key: "ui.inspection.unauthenticated",
        ordinal: 366,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_NTS,
        key: "ui.inspection.nts",
        ordinal: 367,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_NTP,
        key: "ui.inspection.ntp",
        ordinal: 368,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_INVALID_QUERY,
        key: "ui.inspection.invalid-query",
        ordinal: 369,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_INVALID_FILTER,
        key: "ui.inspection.invalid-filter",
        ordinal: 370,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_HIDDEN_SOURCE,
        key: "ui.inspection.hidden-source",
        ordinal: 371,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INSPECTION_PAGE_REQUIRED,
        key: "ui.inspection.page-required",
        ordinal: 372,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::INTERVAL_EVERY,
        key: "ui.interval.every",
        ordinal: 373,
        kind: Kind::Minutes,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::RUST_NOTIFICATIONS_OFF,
        key: "ui.rust.notifications-off",
        ordinal: 374,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::RUST_OBSERVER_UNCONFIRMED,
        key: "ui.rust.observer-unconfirmed",
        ordinal: 375,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::RUST_NOTIFICATION_OS_UNCONFIRMED,
        key: "ui.rust.notification-os-unconfirmed",
        ordinal: 376,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::RUST_NOTIFICATIONS_BLOCKED,
        key: "ui.rust.notifications-blocked",
        ordinal: 377,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::RUST_NOTIFICATIONS_NATIVE_UNAVAILABLE,
        key: "ui.rust.notifications-native-unavailable",
        ordinal: 378,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::RUST_HELPER_COUNT,
        key: "ui.rust.helper-count",
        ordinal: 380,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_QUIT_RUNNING_TITLE,
        key: "ui.rust.quit-running-title",
        ordinal: 381,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_QUIT_STOPPED_TITLE,
        key: "ui.rust.quit-stopped-title",
        ordinal: 382,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_QUIT_PARTIAL_TITLE,
        key: "ui.rust.quit-partial-title",
        ordinal: 383,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_QUIT_PENDING_TITLE,
        key: "ui.rust.quit-pending-title",
        ordinal: 384,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_QUIT_UNKNOWN_TITLE,
        key: "ui.rust.quit-unknown-title",
        ordinal: 385,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_QUIT_RUNNING_CONSEQUENCE,
        key: "ui.rust.quit-running-consequence",
        ordinal: 386,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_QUIT_STOPPED_CONSEQUENCE,
        key: "ui.rust.quit-stopped-consequence",
        ordinal: 387,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_QUIT_PARTIAL_CONSEQUENCE,
        key: "ui.rust.quit-partial-consequence",
        ordinal: 388,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_QUIT_PENDING_CONSEQUENCE,
        key: "ui.rust.quit-pending-consequence",
        ordinal: 389,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_NOTIFICATION_PREFERENCE_UPDATED,
        key: "ui.rust.notification-preference-updated",
        ordinal: 393,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::RUST_TRAY_PREFERENCE_UPDATED,
        key: "ui.rust.tray-preference-updated",
        ordinal: 394,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::RUST_SIGN_IN_PREFERENCE_UPDATED,
        key: "ui.rust.sign-in-preference-updated",
        ordinal: 395,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::RUST_CHANGE_ADD,
        key: "ui.rust.change-add",
        ordinal: 397,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_CHANGE_EDIT,
        key: "ui.rust.change-edit",
        ordinal: 398,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_CHANGE_BASELINE,
        key: "ui.rust.change-baseline",
        ordinal: 399,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_REVIEW_NONE,
        key: "ui.rust.review-none",
        ordinal: 400,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_CHANGE_REMOVE,
        key: "ui.rust.change-remove",
        ordinal: 401,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_SOURCE_PREVIEW_UPDATED,
        key: "ui.rust.source-preview-updated",
        ordinal: 402,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_SOURCE_REVIEW_ONLY,
        key: "ui.rust.source-review-only",
        ordinal: 403,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_SOURCE_DISCARDED,
        key: "ui.rust.source-discarded",
        ordinal: 404,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_SOURCE_OPERATION_UPDATED,
        key: "ui.rust.source-operation-updated",
        ordinal: 405,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_PROTOCOL_CHOICE,
        key: "ui.rust.protocol-choice",
        ordinal: 406,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_SOURCE_CHOICE,
        key: "ui.rust.source-choice",
        ordinal: 407,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_DUPLICATE_SOURCE,
        key: "ui.rust.duplicate-source",
        ordinal: 408,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_SOURCE_CAPACITY,
        key: "ui.rust.source-capacity",
        ordinal: 409,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_SOURCE_ID_EXHAUSTED,
        key: "ui.rust.source-id-exhausted",
        ordinal: 410,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RUST_RECOVERY_PAGE_REQUIRED,
        key: "ui.rust.recovery-page-required",
        ordinal: 411,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_RECOVERY_LEAVE_REQUIRED,
        key: "ui.rust.recovery-leave-required",
        ordinal: 412,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_RECOVERY_REVIEW_REQUIRED,
        key: "ui.rust.recovery-review-required",
        ordinal: 413,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_RECOVERY_EDIT_REQUIRED,
        key: "ui.rust.recovery-edit-required",
        ordinal: 414,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_BACK_STATUS,
        key: "ui.rust.back-status",
        ordinal: 415,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_BACK_SOURCES,
        key: "ui.rust.back-sources",
        ordinal: 416,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_BACK_STATISTICS,
        key: "ui.rust.back-statistics",
        ordinal: 417,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_BACK_DIAGNOSTICS,
        key: "ui.rust.back-diagnostics",
        ordinal: 418,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_BACK_ABOUT,
        key: "ui.rust.back-about",
        ordinal: 419,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_BACK_SETTINGS,
        key: "ui.rust.back-settings",
        ordinal: 420,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_HELPER_PREVIEW_RETAINED,
        key: "ui.rust.helper-preview-retained",
        ordinal: 421,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_HELPER_RAW_DISCARDED,
        key: "ui.rust.helper-raw-discarded",
        ordinal: 422,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_HELPER_PREVIEW_EDIT,
        key: "ui.rust.helper-preview-edit",
        ordinal: 423,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_HELPER_PREVIEW_DISCARDED,
        key: "ui.rust.helper-preview-discarded",
        ordinal: 424,
        kind: Kind::Text,
        group: Some(Group::Recovery),
    },
    Spec {
        id: MessageId::RUST_QUIT_SHOW_CLEANUP_FAILED,
        key: "ui.rust.quit-show-cleanup-failed",
        ordinal: 425,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_QUIT_SHOW_FAILED,
        key: "ui.rust.quit-show-failed",
        ordinal: 426,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_CLOSE_RESTORED,
        key: "ui.rust.close-restored",
        ordinal: 427,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_CLOSE_RESTORE_FAILED,
        key: "ui.rust.close-restore-failed",
        ordinal: 428,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_EXIT_RESTORED,
        key: "ui.rust.exit-restored",
        ordinal: 429,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_EXIT_RESTORE_FAILED,
        key: "ui.rust.exit-restore-failed",
        ordinal: 430,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_TRAY_UNAVAILABLE,
        key: "ui.rust.tray-unavailable",
        ordinal: 431,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_QUIT_REVIEW_REQUIRED,
        key: "ui.rust.quit-review-required",
        ordinal: 432,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_QUIT_CANCELLED,
        key: "ui.rust.quit-cancelled",
        ordinal: 433,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::RUST_SETTINGS_SECTION,
        key: "ui.rust.settings-section",
        ordinal: 434,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::RUST_SERVICE_ACTION,
        key: "ui.rust.service-action",
        ordinal: 435,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::RUST_SERVICE_UNAVAILABLE,
        key: "ui.rust.service-unavailable",
        ordinal: 436,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_BACK,
        key: "ui.diagnostic.back",
        ordinal: 437,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_PROBLEM,
        key: "ui.diagnostic.problem",
        ordinal: 438,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_EFFECT,
        key: "ui.diagnostic.effect",
        ordinal: 439,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_REMEDY,
        key: "ui.diagnostic.remedy",
        ordinal: 440,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_SERVICE,
        key: "ui.diagnostic.service",
        ordinal: 441,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_EVENTS_EMPTY,
        key: "ui.diagnostic.events-empty",
        ordinal: 442,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_EVENTS_NOTE,
        key: "ui.diagnostic.events-note",
        ordinal: 443,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_REPORT_OPEN,
        key: "ui.diagnostic.report-open",
        ordinal: 444,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_REPORT_TITLE,
        key: "ui.diagnostic.report-title",
        ordinal: 445,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_REPORT_NOTE,
        key: "ui.diagnostic.report-note",
        ordinal: 446,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_COPY,
        key: "ui.diagnostic.copy",
        ordinal: 447,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_EXPORT,
        key: "ui.diagnostic.export",
        ordinal: 448,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_EXPORT_REASON,
        key: "ui.diagnostic.export-reason",
        ordinal: 449,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_COLLECTION_OPEN,
        key: "ui.diagnostic.collection-open",
        ordinal: 450,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_COLLECTION_TITLE,
        key: "ui.diagnostic.collection-title",
        ordinal: 451,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_COLLECTION_NOTE,
        key: "ui.diagnostic.collection-note",
        ordinal: 452,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_SCOPE,
        key: "ui.diagnostic.scope",
        ordinal: 453,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_SCOPE_OPTION_UI,
        key: "ui.diagnostic.scope-option.ui",
        ordinal: 454,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_SCOPE_OPTION_SERVICE,
        key: "ui.diagnostic.scope-option.service",
        ordinal: 455,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_SCOPE_OPTION_COMBINED,
        key: "ui.diagnostic.scope-option.combined",
        ordinal: 456,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_DURATION,
        key: "ui.diagnostic.duration",
        ordinal: 457,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_COLLECTION_STATE,
        key: "ui.diagnostic.collection-state",
        ordinal: 458,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_EFFECTIVE_SCOPE,
        key: "ui.diagnostic.effective-scope",
        ordinal: 459,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_REMAINING,
        key: "ui.diagnostic.remaining",
        ordinal: 460,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_UNAVAILABLE,
        key: "ui.diagnostic.unavailable",
        ordinal: 461,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_COLLECTION_REASON,
        key: "ui.diagnostic.collection-reason",
        ordinal: 462,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_REQUIRED_EVENTS,
        key: "ui.diagnostic.required-events",
        ordinal: 463,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_START,
        key: "ui.diagnostic.start",
        ordinal: 464,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_STOP,
        key: "ui.diagnostic.stop",
        ordinal: 465,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_CORRECTION_OPEN,
        key: "ui.diagnostic.correction-open",
        ordinal: 466,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_CORRECTION_TITLE,
        key: "ui.diagnostic.correction-title",
        ordinal: 467,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_CORRECTION_EMPTY,
        key: "ui.diagnostic.correction-empty",
        ordinal: 468,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_CORRECTION_NOTE,
        key: "ui.diagnostic.correction-note",
        ordinal: 469,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_CORRECTION_RULE,
        key: "ui.diagnostic.correction-rule",
        ordinal: 470,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_APPROVE,
        key: "ui.diagnostic.approve",
        ordinal: 471,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_UNAVAILABLE_ACTION,
        key: "ui.diagnostic.unavailable-action",
        ordinal: 472,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DIAGNOSTIC_INVALID_CHOICE,
        key: "ui.diagnostic.invalid-choice",
        ordinal: 473,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::DURATION_MINUTES,
        key: "ui.duration.minutes",
        ordinal: 475,
        kind: Kind::Minutes,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::PREFERENCES_INCIDENTS_HEADING,
        key: "ui.preferences.incidents-heading",
        ordinal: 476,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::PREFERENCES_INCIDENTS_UNAVAILABLE,
        key: "ui.preferences.incidents-unavailable",
        ordinal: 477,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::POLICY_EDITOR_ACTIVE,
        key: "ui.policy.editor-active",
        ordinal: 478,
        kind: Kind::Text,
        group: Some(Group::Policy),
    },
    Spec {
        id: MessageId::INSPECTION_EDITOR_ACTIVE,
        key: "ui.inspection.editor-active",
        ordinal: 479,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_PAGE_REQUIRED,
        key: "ui.source.page-required",
        ordinal: 480,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::TRAY_MENU_OPEN,
        key: "ui.tray.menu.open",
        ordinal: 516,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::TRAY_MENU_DIAGNOSTICS,
        key: "ui.tray.menu.diagnostics",
        ordinal: 518,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_UNKNOWN,
        key: "ui.status.headline.unknown",
        ordinal: 526,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_INITIALIZING,
        key: "ui.status.headline.initializing",
        ordinal: 527,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_ACQUIRING,
        key: "ui.status.headline.acquiring",
        ordinal: 528,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_OBSERVE_ONLY,
        key: "ui.status.headline.observe-only",
        ordinal: 529,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_TRACKING,
        key: "ui.status.headline.tracking",
        ordinal: 530,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_HOLDOVER,
        key: "ui.status.headline.holdover",
        ordinal: 531,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_EXPIRED,
        key: "ui.status.headline.expired",
        ordinal: 532,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_CONFLICT,
        key: "ui.status.headline.conflict",
        ordinal: 533,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_BLOCKED,
        key: "ui.status.headline.blocked",
        ordinal: 534,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_SERVICE_STOPPED,
        key: "ui.status.headline.service-stopped",
        ordinal: 535,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_WINDOWS_MANAGED,
        key: "ui.status.headline.windows-managed",
        ordinal: 536,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_CORRECTION_PENDING,
        key: "ui.status.headline.correction-pending",
        ordinal: 537,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_UNCERTAIN_RESULT,
        key: "ui.status.headline.uncertain-result",
        ordinal: 538,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_RATE_LIMITED,
        key: "ui.status.headline.rate-limited",
        ordinal: 539,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_REACQUIRING,
        key: "ui.status.headline.reacquiring",
        ordinal: 540,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_LEAP_GUARD,
        key: "ui.status.headline.leap-guard",
        ordinal: 541,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_HEADLINE_DATE_RECOVERY,
        key: "ui.status.headline.date-recovery",
        ordinal: 542,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_DETAIL_UNAVAILABLE,
        key: "ui.status.detail.unavailable",
        ordinal: 543,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_DETAIL_WAITING,
        key: "ui.status.detail.waiting",
        ordinal: 544,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_DETAIL_TRACKING,
        key: "ui.status.detail.tracking",
        ordinal: 545,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_DETAIL_HOLDOVER,
        key: "ui.status.detail.holdover",
        ordinal: 546,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_DETAIL_WITHHELD,
        key: "ui.status.detail.withheld",
        ordinal: 547,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_DETAIL_STOPPED,
        key: "ui.status.detail.stopped",
        ordinal: 548,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_DETAIL_WINDOWS_MANAGED,
        key: "ui.status.detail.windows-managed",
        ordinal: 549,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_DETAIL_REVIEW,
        key: "ui.status.detail.review",
        ordinal: 550,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_DETAIL_UNCERTAIN_RESULT,
        key: "ui.status.detail.uncertain-result",
        ordinal: 551,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_DETAIL_RATE_LIMITED,
        key: "ui.status.detail.rate-limited",
        ordinal: 552,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_DETAIL_DATE_RECOVERY,
        key: "ui.status.detail.date-recovery",
        ordinal: 553,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_EVIDENCE_AUTHENTICATED,
        key: "ui.status.evidence.authenticated",
        ordinal: 554,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_EVIDENCE_HISTORICAL,
        key: "ui.status.evidence.historical",
        ordinal: 555,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_EVIDENCE_AGREEMENT_SATISFIED,
        key: "ui.status.evidence.agreement-satisfied",
        ordinal: 556,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_EVIDENCE_CURRENT_ESTIMATE,
        key: "ui.status.evidence.current-estimate",
        ordinal: 557,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_CHECK_SOURCES,
        key: "ui.status.check.sources",
        ordinal: 558,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_ACTION_CONTEXT_REQUIRED,
        key: "ui.status.action.context-required",
        ordinal: 559,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::STATUS_PROFILE_LABEL,
        key: "ui.status.profile.label",
        ordinal: 560,
        kind: Kind::Text,
        group: Some(Group::PreviewFacts),
    },
    Spec {
        id: MessageId::RAW_TOO_LONG,
        key: "ui.raw.too-long",
        ordinal: 561,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RAW_CHARACTER,
        key: "ui.raw.character",
        ordinal: 562,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RAW_INVALID_OPTION,
        key: "ui.raw.invalid-option",
        ordinal: 563,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RAW_CONTEXT,
        key: "ui.raw.context",
        ordinal: 564,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::FEEDBACK_NO_NOTIFICATION_SUBMITTED,
        key: "ui.feedback.no-notification-submitted",
        ordinal: 565,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::FIXTURE_GUI_BUILD_LABEL,
        key: "ui.fixture.gui-build-label",
        ordinal: 566,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::FIXTURE_TEXT_CONTRACT_LABEL,
        key: "ui.fixture.text-contract-label",
        ordinal: 567,
        kind: Kind::Text,
        group: None,
    },
    Spec {
        id: MessageId::PREFERENCES_LANGUAGE_LABEL,
        key: "ui.preferences.language-label",
        ordinal: 568,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::PREFERENCES_LANGUAGE_HELP,
        key: "ui.preferences.language-help",
        ordinal: 569,
        kind: Kind::Text,
        group: Some(Group::Intervals),
    },
    Spec {
        id: MessageId::SOURCE_LIST_REQUIRED,
        key: "ui.source.list-required",
        ordinal: 570,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SOURCE_REVIEW_REQUIRED,
        key: "ui.source.review-required",
        ordinal: 571,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::SHELL_SYNCHRONIZATION_SETTINGS,
        key: "ui.shell.synchronization-settings",
        ordinal: 572,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_TIME_POLICY,
        key: "ui.shell.time-policy",
        ordinal: 573,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_TIME_POLICY_DESCRIPTION,
        key: "ui.shell.time-policy-description",
        ordinal: 574,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_TIME_POLICY_POWER,
        key: "ui.shell.time-policy-power",
        ordinal: 575,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_PERSONAL_PREFERENCES,
        key: "ui.shell.personal-preferences",
        ordinal: 576,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::SHELL_PERSONAL_PREFERENCES_DESCRIPTION,
        key: "ui.shell.personal-preferences-description",
        ordinal: 577,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::RAW_UNAVAILABLE,
        key: "ui.raw.unavailable",
        ordinal: 578,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RAW_STALE,
        key: "ui.raw.stale",
        ordinal: 579,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RAW_ACTION_UNDO,
        key: "ui.raw.action-undo",
        ordinal: 580,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RAW_ACTION_REDO,
        key: "ui.raw.action-redo",
        ordinal: 581,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RAW_ACTION_CUT,
        key: "ui.raw.action-cut",
        ordinal: 582,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RAW_ACTION_COPY,
        key: "ui.raw.action-copy",
        ordinal: 583,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RAW_ACTION_PASTE,
        key: "ui.raw.action-paste",
        ordinal: 584,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::RAW_ACTION_SELECT_ALL,
        key: "ui.raw.action-select-all",
        ordinal: 585,
        kind: Kind::Text,
        group: Some(Group::SourceEdit),
    },
    Spec {
        id: MessageId::NATIVE_NOTIFICATION_TEST_TITLE,
        key: "ui.native-notification.test-title",
        ordinal: 586,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::NATIVE_NOTIFICATION_TEST_BODY,
        key: "ui.native-notification.test-body",
        ordinal: 587,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_NOTIFICATION_WINDOWS_ALLOWED,
        key: "ui.preferences.notification-windows-allowed",
        ordinal: 588,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_NOTIFICATION_WINDOWS_BLOCKED,
        key: "ui.preferences.notification-windows-blocked",
        ordinal: 589,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_NOTIFICATION_OBSERVER_RUNNING,
        key: "ui.preferences.notification-observer-running",
        ordinal: 590,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::NATIVE_NOTIFICATION_TEST_ACCEPTED,
        key: "ui.native-notification.test-accepted",
        ordinal: 591,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::NATIVE_NOTIFICATION_TEST_INDETERMINATE,
        key: "ui.native-notification.test-indeterminate",
        ordinal: 592,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::NATIVE_NOTIFICATION_TEST_COALESCED,
        key: "ui.native-notification.test-coalesced",
        ordinal: 593,
        kind: Kind::Text,
        group: Some(Group::Notifications),
    },
    Spec {
        id: MessageId::PREFERENCES_TRAY_READY_NOTE,
        key: "ui.preferences.tray-ready-note",
        ordinal: 594,
        kind: Kind::Text,
        group: Some(Group::Lifecycle),
    },
    Spec {
        id: MessageId::NATIVE_OBSERVER_STOP_TIMEOUT,
        key: "ui.native-observer.stop-timeout",
        ordinal: 595,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
    Spec {
        id: MessageId::NATIVE_OBSERVER_STOP_FAILED,
        key: "ui.native-observer.stop-failed",
        ordinal: 596,
        kind: Kind::Text,
        group: Some(Group::Quit),
    },
];
