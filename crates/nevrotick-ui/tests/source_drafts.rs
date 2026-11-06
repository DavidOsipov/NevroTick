//! Independent local-source workflow expectations from v0.4 and architecture §6.2.
#![forbid(unsafe_code)]

use nevrotick_ui::model::ModelError;
use nevrotick_ui::source_drafts::{
    ChangeKind, MAX_SOURCE_CHANGES, MAX_SOURCES, PortUpdate, Protocol, ProtocolPortDraft,
    RawSourceInput, SourceDraftError, SourceDrafts,
};

/// Create an inert test input; admission is performed by the actual public API.
fn input<'a>(
    label: &'a str,
    host: &'a str,
    protocol_choice: i32,
    port: &'a str,
) -> RawSourceInput<'a> {
    RawSourceInput {
        label,
        host,
        protocol_choice,
        port,
    }
}

/// Empty is a legal preview, not a claim of usable or qualified synchronization.
///
/// # Errors
/// Propagates unexpected admission failure for legal empty/nonempty fixtures.
///
/// # Panics
/// Fails when capacity, source defaults or review-baseline separation regresses.
#[test]
fn baseline_and_review_remain_separate_without_apply() -> Result<(), SourceDraftError> {
    let empty = SourceDrafts::from_baseline(&[])?;
    assert_eq!(
        empty.preview(),
        Vec::<&nevrotick_ui::source_drafts::DraftSource>::new()
    );
    let mut drafts = SourceDrafts::fixture()?;
    let baseline = drafts.baseline().to_vec();
    assert_eq!(baseline.len(), 3);
    assert_eq!(drafts.review(), []);
    let id = drafts.add(input("Delta", "delta.example.invalid", 0, "4460"))?;
    assert!(id.view_id() > 0);
    assert_eq!(drafts.preview().len(), 4);
    assert_eq!(drafts.baseline(), baseline.as_slice());
    assert_eq!(drafts.review().len(), 1);
    let change = drafts
        .review()
        .first()
        .ok_or(SourceDraftError::UnknownSource)?;
    assert_eq!(change.kind(), ChangeKind::Add);
    assert_eq!(change.id(), id);
    assert!(change.before().is_none());
    let after = change.after().ok_or(SourceDraftError::UnknownSource)?;
    assert_eq!(after.fields().label(), "Delta");
    assert_eq!(after.fields().port(), 4460);
    assert_eq!(after.protocol(), Protocol::Nts);
    assert_eq!(MAX_SOURCES, 16);
    assert_eq!(MAX_SOURCE_CHANGES, 32);
    drafts.discard();
    assert_eq!(drafts.review(), []);
    assert_eq!(drafts.preview().len(), 3);
    assert_eq!(drafts.baseline(), baseline.as_slice());
    Ok(())
}

/// Source edits, removal and Undo affect one delta per stable session identity.
///
/// # Errors
/// Propagates an unexpected legal-operation failure.
///
/// # Panics
/// Fails when edit/remove/Undo alters the baseline or reuses a discarded ID.
#[test]
fn edit_remove_undo_and_discard_use_stable_non_recycled_ids() -> Result<(), SourceDraftError> {
    let mut drafts = SourceDrafts::fixture()?;
    let baseline = drafts.baseline().to_vec();
    let first = baseline.first().ok_or(SourceDraftError::UnknownSource)?;
    let first_id = first.id().view_id();
    drafts.edit(first_id, input("Renamed", "new.example.invalid", 1, "123"))?;
    drafts.edit(
        first_id,
        input("Renamed twice", "newer.example.invalid", 1, "124"),
    )?;
    assert_eq!(drafts.review().len(), 1);
    let edit = drafts
        .review()
        .first()
        .ok_or(SourceDraftError::UnknownSource)?;
    assert_eq!(edit.kind(), ChangeKind::Edit);
    assert_eq!(edit.before(), Some(first));
    assert_eq!(drafts.source(first_id)?.id(), first.id());
    drafts.remove(first_id)?;
    let removal = drafts
        .review()
        .first()
        .ok_or(SourceDraftError::UnknownSource)?;
    assert_eq!(removal.kind(), ChangeKind::Remove);
    assert!(removal.after().is_none());
    assert_eq!(
        drafts.source(first_id),
        Err(SourceDraftError::UnknownSource)
    );
    drafts.undo(first_id)?;
    assert_eq!(drafts.review(), []);
    assert_eq!(drafts.source(first_id)?, first);
    let added = drafts.add(input("Added", "added.example.invalid", 0, "4460"))?;
    drafts.edit(
        added.view_id(),
        input("Added edited", "edited.example.invalid", 0, "4461"),
    )?;
    assert_eq!(
        drafts
            .review()
            .first()
            .ok_or(SourceDraftError::UnknownSource)?
            .kind(),
        ChangeKind::Add
    );
    drafts.remove(added.view_id())?;
    assert_eq!(drafts.review(), []);
    let next = drafts.add(input("Next", "next.example.invalid", 0, "4460"))?;
    assert!(next.view_id() > added.view_id());
    drafts.undo(next.view_id())?;
    let after_undo = drafts.add(input("Third", "third.example.invalid", 0, "4460"))?;
    assert!(after_undo.view_id() > next.view_id());
    drafts.discard();
    let after_discard = drafts.add(input("Fourth", "fourth.example.invalid", 0, "4460"))?;
    assert!(after_discard.view_id() > after_undo.view_id());
    assert_eq!(drafts.baseline(), baseline.as_slice());
    Ok(())
}

/// The selected duplicate key is normalized identity+protocol, not label/port.
///
/// # Errors
/// Propagates unexpected admission failure for valid controls.
///
/// # Panics
/// Fails when DNS/IP aliases of a literal evade duplicate checks or protocols mix.
#[test]
fn dns_and_numeric_ip_equivalence_reject_duplicate_representations() -> Result<(), SourceDraftError>
{
    let mut drafts = SourceDrafts::from_baseline(&[
        input("DNS", "CLOCK.Example.invalid", 0, "4460"),
        input("IPv4", "192.0.2.1", 0, "4460"),
        input("IPv6", "2001:db8::1", 0, "4460"),
    ])?;
    for host in [
        "clock.example.invalid",
        "CLOCK.EXAMPLE.INVALID.",
        "192.0.2.1",
        "192.0.2.1.",
        "2001:0DB8:0:0:0:0:0:1",
    ] {
        let before = drafts.clone();
        assert_eq!(
            drafts.add(input("Different label and port", host, 0, "9999")),
            Err(SourceDraftError::DuplicateIdentity)
        );
        assert_eq!(drafts, before);
    }
    // A separately explicit NTP profile is never silently inferred or downgraded.
    let ntp = drafts.add(input("Explicit NTP", "clock.example.invalid.", 1, "123"))?;
    assert_eq!(drafts.source(ntp.view_id())?.protocol(), Protocol::PlainNtp);
    // Different numeric addresses are useful accepted neighbors.
    drafts.add(input("Next IP", "2001:db8::2", 0, "4460"))?;
    drafts.add(input("Next IPv4", "192.0.2.2", 0, "4460"))?;
    // DNS identity is not resolved into an IP and never claims independence.
    drafts.add(input(
        "DNS alias unknown",
        "another.example.invalid",
        0,
        "4460",
    ))?;
    Ok(())
}

/// Effective entries, not obsolete baseline values, determine draft uniqueness.
///
/// # Errors
/// Propagates unexpected fixture or accepted-neighbor failure.
///
/// # Panics
/// Fails when a phantom duplicate blocks a valid change or Undo corrupts drafts.
#[test]
fn moved_or_removed_identity_can_be_reused_but_conflicting_undo_is_atomic()
-> Result<(), SourceDraftError> {
    let mut drafts =
        SourceDrafts::from_baseline(&[input("Old", "old.example.invalid", 0, "4460")])?;
    let original = drafts
        .baseline()
        .first()
        .ok_or(SourceDraftError::UnknownSource)?
        .id();
    drafts.edit(
        original.view_id(),
        input("Moved", "moved.example.invalid", 0, "4460"),
    )?;
    let replacement = drafts.add(input("Reused", "OLD.EXAMPLE.INVALID.", 0, "4460"))?;
    let before = drafts.clone();
    assert_eq!(
        drafts.undo(original.view_id()),
        Err(SourceDraftError::DuplicateIdentity)
    );
    assert_eq!(drafts, before);
    drafts.remove(replacement.view_id())?;
    drafts.undo(original.view_id())?;
    drafts.remove(original.view_id())?;
    let reused = drafts.add(input(
        "Reuse after remove",
        "old.example.invalid",
        0,
        "4460",
    ))?;
    let before = drafts.clone();
    assert_eq!(
        drafts.undo(original.view_id()),
        Err(SourceDraftError::DuplicateIdentity)
    );
    assert_eq!(drafts, before);
    drafts.remove(reused.view_id())?;
    drafts.undo(original.view_id())?;
    assert_eq!(drafts.preview().len(), 1);
    assert_eq!(drafts.review(), []);
    Ok(())
}

/// Actual receiving APIs reject hostile fields/choices and preserve all state.
///
/// # Errors
/// Propagates unexpected fixture or valid-neighbor failure.
///
/// # Panics
/// Fails when direct-handler input bypasses a guard or partial state is retained.
#[test]
fn malformed_direct_add_and_edit_preserve_every_old_draft_and_id() -> Result<(), SourceDraftError> {
    let mut drafts = SourceDrafts::fixture()?;
    let baseline_id = drafts
        .baseline()
        .first()
        .ok_or(SourceDraftError::UnknownSource)?
        .id();
    drafts.edit(
        baseline_id.view_id(),
        input("Pending valid", "pending.example.invalid", 1, "123"),
    )?;
    let oversized_label = "x".repeat(65);
    let oversized_host = "x".repeat(254);
    let oversized_port = "1".repeat(6);
    let rejected = [
        input("", "ok.example.invalid", 0, "4460"),
        input(" edge", "ok.example.invalid", 0, "4460"),
        input("Control\n", "ok.example.invalid", 0, "4460"),
        input("Юникод", "ok.example.invalid", 0, "4460"),
        input(&oversized_label, "ok.example.invalid", 0, "4460"),
        input("Valid", &oversized_host, 0, "4460"),
        input("Valid", "https://example.invalid", 0, "4460"),
        input("Valid", "host..invalid", 0, "4460"),
        input("Valid", "999.999.999.999", 0, "4460"),
        input("Valid", "192.000.2.1", 0, "4460"),
        input("Valid", "::1.", 0, "4460"),
        input("Valid", "[::1]", 0, "4460"),
        input("Valid", "fe80::1%zone", 0, "4460"),
        input("Valid", "valid.example.invalid", -1, "4460"),
        input("Valid", "valid.example.invalid", 2, "4460"),
        input("Valid", "valid.example.invalid", i32::MAX, "4460"),
        input("Valid", "valid.example.invalid", 0, "0"),
        input("Valid", "valid.example.invalid", 0, "65536"),
        input("Valid", "valid.example.invalid", 0, "+123"),
        input("Valid", "valid.example.invalid", 0, "1.0"),
        input("Valid", "valid.example.invalid", 0, "1e2"),
        input("Valid", "valid.example.invalid", 0, " 123"),
        input("Valid", "valid.example.invalid", 0, "１２３"),
        input("Valid", "valid.example.invalid", 0, &oversized_port),
    ];
    for raw in rejected {
        let before = drafts.clone();
        assert!(drafts.add(raw).is_err());
        assert_eq!(drafts, before);
        assert!(drafts.edit(baseline_id.view_id(), raw).is_err());
        assert_eq!(drafts, before);
    }
    let neighbor = drafts.add(input(
        "Valid neighbor",
        "neighbor.example.invalid",
        0,
        "65535",
    ))?;
    assert_eq!(drafts.source(neighbor.view_id())?.fields().port(), 65535);
    drafts.edit(
        neighbor.view_id(),
        input("Shortest port", "neighbor.example.invalid", 1, "1"),
    )?;
    assert_eq!(drafts.source(neighbor.view_id())?.fields().port(), 1);
    Ok(())
}

/// Invalid signed IDs and stale IDs cannot select, mutate or Undo other rows.
///
/// # Errors
/// Propagates unexpected legal fixture/operation failure.
///
/// # Panics
/// Fails when an index-like or stale ID bypasses current membership.
#[test]
fn signed_unknown_removed_and_discarded_ids_are_checked_at_consumers()
-> Result<(), SourceDraftError> {
    let mut drafts = SourceDrafts::fixture()?;
    for invalid in [i32::MIN, -1, 0, 17, i32::MAX] {
        let before = drafts.clone();
        assert_eq!(drafts.source(invalid), Err(SourceDraftError::UnknownSource));
        assert_eq!(
            drafts.edit(invalid, input("Valid", "valid.example.invalid", 0, "4460")),
            Err(SourceDraftError::UnknownSource)
        );
        assert_eq!(drafts.remove(invalid), Err(SourceDraftError::UnknownSource));
        assert_eq!(drafts.undo(invalid), Err(SourceDraftError::UnknownSource));
        assert_eq!(drafts, before);
    }
    let id = drafts.add(input("Stale", "stale.example.invalid", 0, "4460"))?;
    drafts.discard();
    let before = drafts.clone();
    assert_eq!(
        drafts.remove(id.view_id()),
        Err(SourceDraftError::UnknownSource)
    );
    assert_eq!(
        drafts.undo(id.view_id()),
        Err(SourceDraftError::UnknownSource)
    );
    assert_eq!(drafts, before);
    Ok(())
}

/// The selected source limit constrains baseline, effective preview and Undo.
///
/// # Errors
/// Propagates an unexpected valid-capacity transition failure.
///
/// # Panics
/// Fails when aggregate bounds or full replacement semantics regress.
#[test]
fn sixteen_source_limit_and_derived_delta_budget_hold_across_replacement()
-> Result<(), SourceDraftError> {
    let mut drafts = SourceDrafts::from_baseline(&[])?;
    for index in 0..MAX_SOURCES {
        let host = format!("source-{index}.example.invalid");
        drafts.add(input("Source", &host, 0, "4460"))?;
    }
    let before = drafts.clone();
    assert_eq!(
        drafts.add(input("Too many", "overflow.example.invalid", 0, "4460")),
        Err(SourceDraftError::Capacity)
    );
    assert_eq!(drafts, before);
    let hosts: Vec<String> = (0..MAX_SOURCES)
        .map(|index| format!("baseline-{index}.example.invalid"))
        .collect();
    let raw: Vec<RawSourceInput<'_>> = hosts
        .iter()
        .map(|host| input("Baseline", host, 0, "4460"))
        .collect();
    let mut drafts = SourceDrafts::from_baseline(&raw)?;
    let ids: Vec<i32> = drafts
        .baseline()
        .iter()
        .map(|source| source.id().view_id())
        .collect();
    for id in &ids {
        drafts.remove(*id)?;
    }
    assert_eq!(
        drafts.preview(),
        Vec::<&nevrotick_ui::source_drafts::DraftSource>::new()
    );
    for index in 0..MAX_SOURCES {
        let host = format!("replacement-{index}.example.invalid");
        drafts.add(input("Replacement", &host, 1, "123"))?;
    }
    assert_eq!(drafts.review().len(), MAX_SOURCE_CHANGES);
    assert_eq!(drafts.preview().len(), MAX_SOURCES);
    let before = drafts.clone();
    let first = ids
        .first()
        .copied()
        .ok_or(SourceDraftError::UnknownSource)?;
    assert_eq!(drafts.undo(first), Err(SourceDraftError::Capacity));
    assert_eq!(drafts, before);
    drafts.discard();
    assert_eq!(drafts.preview().len(), MAX_SOURCES);
    assert_eq!(drafts.review(), []);
    let too_many = [input("Duplicate", "same.example.invalid", 0, "4460"); MAX_SOURCES + 1];
    assert_eq!(
        SourceDrafts::from_baseline(&too_many),
        Err(SourceDraftError::Capacity)
    );
    Ok(())
}

/// Protocol switches preserve manually accepted ports, even a standard value.
///
/// # Errors
/// Propagates unexpected valid editor or source admission failure.
///
/// # Panics
/// Fails when protocol/default provenance or rejecting neighbors regress.
#[test]
fn protocol_defaults_follow_only_untouched_ports() -> Result<(), SourceDraftError> {
    let mut editor = ProtocolPortDraft::new(Protocol::Nts);
    assert_eq!(editor.port(), 4460);
    assert!(editor.follows_default());
    assert_eq!(editor.switch_protocol(1)?, PortUpdate::Default(123));
    assert_eq!(editor.protocol(), Protocol::PlainNtp);
    assert_eq!(editor.port(), 123);
    assert_eq!(editor.switch_protocol(0)?, PortUpdate::Default(4460));
    assert_eq!(editor.port(), 4460);
    editor.set_port("4460")?;
    assert!(!editor.follows_default());
    assert_eq!(editor.switch_protocol(1)?, PortUpdate::PreserveDraft);
    assert_eq!(editor.port(), 4460);
    editor.set_port("12345")?;
    assert_eq!(editor.switch_protocol(0)?, PortUpdate::PreserveDraft);
    assert_eq!(editor.port(), 12345);
    let before = editor.clone();
    assert_eq!(
        editor.switch_protocol(-1),
        Err(SourceDraftError::InvalidProtocol)
    );
    assert_eq!(
        editor.switch_protocol(2),
        Err(SourceDraftError::InvalidProtocol)
    );
    for raw in ["", "0", "65536", "123456", "+123", "1.5", "1e2", "１２３"] {
        assert_eq!(
            editor.set_port(raw),
            Err(SourceDraftError::Field(ModelError::InvalidPort))
        );
        assert_eq!(editor, before);
    }
    assert_eq!(editor, before);
    let drafts = SourceDrafts::fixture()?;
    let source = drafts
        .baseline()
        .first()
        .ok_or(SourceDraftError::UnknownSource)?;
    let opened = ProtocolPortDraft::from_source(source);
    assert_eq!(opened.protocol(), Protocol::Nts);
    assert_eq!(opened.port(), 4460);
    assert!(opened.follows_default());
    Ok(())
}

/// Baseline admission and errors use the same checked boundary as later edits.
///
/// # Panics
/// Fails when construction bypasses protocol/identity guards or errors echo input.
#[test]
fn baseline_has_no_duplicate_or_unknown_protocol_bypass_and_errors_are_redacted() {
    let duplicate = [
        input("First", "Clock.example.invalid", 0, "4460"),
        input("Second", "CLOCK.EXAMPLE.INVALID.", 0, "123"),
    ];
    assert_eq!(
        SourceDrafts::from_baseline(&duplicate),
        Err(SourceDraftError::DuplicateIdentity)
    );
    assert_eq!(
        SourceDrafts::from_baseline(&[input("Valid", "clock.example.invalid", 99, "123")]),
        Err(SourceDraftError::InvalidProtocol)
    );
    assert_eq!(
        Protocol::from_view(i32::MIN),
        Err(SourceDraftError::InvalidProtocol)
    );
    assert_eq!(
        Protocol::from_view(i32::MAX),
        Err(SourceDraftError::InvalidProtocol)
    );
    for error in [
        SourceDraftError::Field(ModelError::InvalidLabel),
        SourceDraftError::Field(ModelError::InvalidHost),
        SourceDraftError::Field(ModelError::InvalidPort),
        SourceDraftError::UnknownSource,
        SourceDraftError::DuplicateIdentity,
        SourceDraftError::Capacity,
        SourceDraftError::IdExhausted,
    ] {
        let message = error.to_string();
        assert!(!message.contains("example.invalid"));
        assert!(!message.contains("999.999.999.999"));
        assert!(message.len() < 160);
    }
}

/// Invalid raw view edits also stop default replacement, without admitting them.
///
/// # Errors
/// Propagates unexpected valid protocol switching failure.
///
/// # Panics
/// Fails when a protocol switch would erase manually entered invalid raw text.
#[test]
fn manual_raw_edit_preserves_view_text_even_before_valid_port_admission()
-> Result<(), SourceDraftError> {
    let mut editor = ProtocolPortDraft::new(Protocol::Nts);
    editor.mark_port_edited();
    let before = editor.clone();
    assert_eq!(
        editor.set_port("invalid"),
        Err(SourceDraftError::Field(ModelError::InvalidPort))
    );
    assert_eq!(editor, before);
    assert_eq!(editor.switch_protocol(1)?, PortUpdate::PreserveDraft);
    assert_eq!(editor.protocol(), Protocol::PlainNtp);
    assert_eq!(editor.port(), 4460);
    assert!(!editor.follows_default());
    Ok(())
}

/// Original raw length cannot be shortened into admission by root-dot stripping.
///
/// # Errors
/// Propagates unexpected acceptance-neighbor failure.
///
/// # Panics
/// Fails when raw representation or repeated-dot rejection is bypassed.
#[test]
fn raw_host_byte_bound_and_root_dot_rules_precede_normalization() -> Result<(), SourceDraftError> {
    // Four valid DNS labels total exactly 253 ASCII bytes including separators.
    let maximal = format!(
        "{}.{}.{}.{}",
        "a".repeat(63),
        "b".repeat(63),
        "c".repeat(63),
        "d".repeat(61)
    );
    assert_eq!(maximal.len(), 253);
    let mut drafts = SourceDrafts::from_baseline(&[])?;
    let with_root_dot = format!("{maximal}.");
    assert_eq!(with_root_dot.len(), 254);
    let before = drafts.clone();
    assert_eq!(
        drafts.add(input(
            "Too long before normalization",
            &with_root_dot,
            0,
            "4460"
        )),
        Err(SourceDraftError::Field(ModelError::InvalidHost))
    );
    assert_eq!(drafts, before);
    drafts.add(input("Maximal valid", &maximal, 0, "4460"))?;
    for host in [
        "clock.example.invalid..",
        "clock.example.invalid...",
        ".",
        "..",
        " clock.example.invalid.",
        "clock.example.invalid. ",
        "clock.example.invalid.\n",
        "clöck.example.invalid.",
        "https://clock.example.invalid.",
    ] {
        let before = drafts.clone();
        assert_eq!(
            drafts.add(input("Invalid raw", host, 0, "4460")),
            Err(SourceDraftError::Field(ModelError::InvalidHost))
        );
        assert_eq!(drafts, before);
    }
    drafts.add(input("Single root dot", "root.example.invalid.", 0, "4460"))?;
    drafts.add(input(
        "Different protocol",
        "root.example.invalid",
        1,
        "123",
    ))?;
    let before = drafts.clone();
    assert_eq!(
        drafts.add(input(
            "Duplicate root spelling",
            "ROOT.EXAMPLE.INVALID",
            0,
            "4461"
        )),
        Err(SourceDraftError::DuplicateIdentity)
    );
    assert_eq!(drafts, before);
    Ok(())
}

/// `IpAddr` family equality is explicit; no cross-family alias policy is invented.
///
/// # Errors
/// Propagates unexpected legal literal admission failure.
///
/// # Panics
/// Fails when equivalent IPv6 byte spellings evade checks or families collapse.
#[test]
fn ip_families_remain_distinct_within_family_ipv6_forms_compare_by_bytes()
-> Result<(), SourceDraftError> {
    let mut drafts = SourceDrafts::from_baseline(&[input("IPv4", "192.0.2.1", 0, "4460")])?;
    drafts.add(input("Mapped IPv6", "::ffff:192.0.2.1", 0, "4460"))?;
    let before = drafts.clone();
    assert_eq!(
        drafts.add(input(
            "Same mapped IPv6",
            "0:0:0:0:0:ffff:c000:201",
            0,
            "4461"
        )),
        Err(SourceDraftError::DuplicateIdentity)
    );
    assert_eq!(drafts, before);
    drafts.add(input(
        "Different mapped IPv6",
        "::ffff:192.0.2.2",
        0,
        "4460",
    ))?;
    Ok(())
}
