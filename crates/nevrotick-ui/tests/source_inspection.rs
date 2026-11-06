//! Independent bounded query/filter/detail expectations over the checked owner.
#![forbid(unsafe_code)]

use nevrotick_ui::source_drafts::{
    Protocol, RawSourceInput, SourceDraftError, SourceDrafts, SourceId,
};
use nevrotick_ui::source_inspection::{
    AbsentFact, DetailFact, InspectionError, MAX_QUERY_BYTES, SourceFilter, SourceInspection,
    SourceQuery,
};

/// Project stable authority IDs, never filtered row positions.
fn ids(inspection: &SourceInspection, owner: &SourceDrafts) -> Vec<i32> {
    inspection
        .visible_ids(owner)
        .into_iter()
        .map(SourceId::view_id)
        .collect()
}

/// Raw query is ASCII/byte-bounded and rejection leaves the entire state intact.
///
/// # Errors
/// Propagates unexpected valid query or fixture admission failure.
///
/// # Panics
/// Fails when trimming/truncation/control admission changes expected visible data.
#[test]
fn query_admission_is_bounded_exact_and_atomic() -> Result<(), InspectionError> {
    let owner = SourceDrafts::fixture()?;
    let original_owner = owner.clone();
    let mut inspection = SourceInspection::new();
    assert_eq!(inspection.filter(), SourceFilter::All);
    assert_eq!(inspection.query().text(), "");
    assert_eq!(ids(&inspection, &owner), [1, 2, 3]);
    inspection.set_query("NORTH")?;
    assert_eq!(ids(&inspection, &owner), [1]);
    inspection.open_details(&owner, 1)?;
    let before = inspection.clone();
    let oversized = "x".repeat(MAX_QUERY_BYTES + 1);
    for raw in ["\0", "new\nline", "tab\t", "\u{7f}", "é", "🙂", &oversized] {
        assert_eq!(
            inspection.set_query(raw),
            Err(InspectionError::InvalidQuery)
        );
        assert_eq!(inspection, before);
        assert_eq!(ids(&inspection, &owner), [1]);
    }
    inspection.set_query(" North ")?;
    assert_eq!(inspection.query().text(), " North ");
    assert_eq!(ids(&inspection, &owner), Vec::<i32>::new());
    let maximum = "x".repeat(MAX_QUERY_BYTES);
    inspection.set_query(&maximum)?;
    assert_eq!(inspection.query().text().len(), 253);
    inspection.set_query(" ")?;
    assert_eq!(ids(&inspection, &owner), [1, 2, 3]);
    assert!(SourceQuery::parse("!~").is_ok());
    inspection.clear_query();
    assert_eq!(ids(&inspection, &owner), [1, 2, 3]);
    assert_eq!(owner, original_owner);
    Ok(())
}

/// Known role filters remain unavailable at raw and typed receiving entry points.
///
/// # Errors
/// Propagates unexpected valid setup/All-filter failure.
///
/// # Panics
/// Fails when editor selection or a direct typed call manufactures Used/Other.
#[test]
fn roles_unavailable_does_not_block_independent_query_or_details() -> Result<(), InspectionError> {
    let owner = SourceDrafts::fixture()?;
    let mut inspection = SourceInspection::new();
    inspection.set_query("north")?;
    inspection.open_details(&owner, 1)?;
    let before = inspection.clone();
    for (choice, filter) in [(1, SourceFilter::Used), (2, SourceFilter::Other)] {
        assert_eq!(SourceFilter::from_view(choice)?, filter);
        assert!(filter.unavailable_reason().is_some());
        assert_eq!(
            inspection.set_filter_choice(choice),
            Err(InspectionError::RolesUnavailable)
        );
        assert_eq!(inspection, before);
        assert_eq!(
            inspection.set_filter(filter),
            Err(InspectionError::RolesUnavailable)
        );
        assert_eq!(inspection, before);
    }
    for choice in [i32::MIN, -1, 3, i32::MAX] {
        assert_eq!(
            inspection.set_filter_choice(choice),
            Err(InspectionError::InvalidFilter)
        );
        assert_eq!(inspection, before);
    }
    inspection.set_filter_choice(0)?;
    assert!(SourceFilter::All.unavailable_reason().is_none());
    assert_eq!(inspection, before);
    inspection.set_query("west")?;
    assert_eq!(ids(&inspection, &owner), [3]);
    let detail = inspection
        .details(&owner)?
        .ok_or(InspectionError::HiddenSource)?;
    assert_eq!(detail.source().id().view_id(), 1);
    assert!(detail.hidden_by_query());
    assert_eq!(
        detail.fact(DetailFact::SelectionRole),
        AbsentFact::NotEvaluated
    );
    Ok(())
}

/// Filtering returns stable IDs without editing all-source/Statistics/draft data.
///
/// # Errors
/// Propagates unexpected legal query or source transition failure.
///
/// # Panics
/// Fails when a row index aliases an identity or hidden details change targets.
#[test]
fn visible_projection_and_hidden_details_never_rewrite_the_source_owner()
-> Result<(), InspectionError> {
    let mut owner = SourceDrafts::fixture()?;
    let added = owner.add(RawSourceInput {
        label: "Extra",
        host: "extra.example.invalid",
        protocol_choice: 1,
        port: "123",
    })?;
    let original_owner = owner.clone();
    let mut inspection = SourceInspection::new();
    inspection.set_query("south")?;
    assert_eq!(ids(&inspection, &owner), [2]);
    assert_eq!(owner.preview().len(), 4);
    assert_eq!(owner, original_owner);
    assert!(inspection.source_visible(&owner, 2)?);
    assert!(!inspection.source_visible(&owner, 1)?);
    inspection.open_details(&owner, 2)?;
    let before = inspection.clone();
    assert_eq!(
        inspection.open_details(&owner, 1),
        Err(InspectionError::HiddenSource)
    );
    assert_eq!(inspection, before);
    for invalid in [i32::MIN, -1, 0, i32::MAX] {
        assert_eq!(
            inspection.open_details(&owner, invalid),
            Err(InspectionError::Source(SourceDraftError::UnknownSource))
        );
        assert_eq!(inspection, before);
    }
    let detail = inspection
        .details(&owner)?
        .ok_or(InspectionError::HiddenSource)?;
    assert_eq!(detail.source().fields().label(), "Sample South");
    inspection.clear_query();
    assert_eq!(ids(&inspection, &owner), [1, 2, 3, added.view_id()]);
    assert_eq!(owner, original_owner);
    Ok(())
}

/// Resolve detail identity again after edits/removal, retaining honest context.
///
/// # Errors
/// Propagates unexpected valid query/detail/source operation failure.
///
/// # Panics
/// Fails when stale detail caches or recycled IDs silently display another row.
#[test]
fn details_resolve_current_membership_and_fields_after_source_mutations()
-> Result<(), InspectionError> {
    let mut owner = SourceDrafts::fixture()?;
    let mut inspection = SourceInspection::new();
    inspection.set_query("west")?;
    inspection.open_details(&owner, 3)?;
    owner.edit(
        3,
        RawSourceInput {
            label: "Moved",
            host: "moved.example.invalid",
            protocol_choice: 1,
            port: "124",
        },
    )?;
    let current = inspection
        .details(&owner)?
        .ok_or(InspectionError::HiddenSource)?;
    assert_eq!(current.source().id().view_id(), 3);
    assert_eq!(current.source().fields().host(), "moved.example.invalid");
    assert_eq!(current.source().protocol(), Protocol::PlainNtp);
    assert!(current.hidden_by_query());
    let before = inspection.clone();
    owner.remove(3)?;
    assert_eq!(
        inspection.details(&owner),
        Err(InspectionError::Source(SourceDraftError::UnknownSource))
    );
    assert_eq!(inspection, before);
    owner.undo(3)?;
    let restored = inspection
        .details(&owner)?
        .ok_or(InspectionError::HiddenSource)?;
    assert_eq!(restored.source().fields().host(), "west.example.invalid");
    assert!(!restored.hidden_by_query());
    inspection.clear_query();
    let removed = owner.add(RawSourceInput {
        label: "Removed addition",
        host: "removed.example.invalid",
        protocol_choice: 0,
        port: "4460",
    })?;
    inspection.open_details(&owner, removed.view_id())?;
    owner.remove(removed.view_id())?;
    let next = owner.add(RawSourceInput {
        label: "New addition",
        host: "new.example.invalid",
        protocol_choice: 0,
        port: "4460",
    })?;
    assert!(next.view_id() > removed.view_id());
    assert_eq!(
        inspection.details(&owner),
        Err(InspectionError::Source(SourceDraftError::UnknownSource))
    );
    inspection.open_details(&owner, next.view_id())?;
    inspection.close_details();
    assert!(inspection.details(&owner)?.is_none());
    Ok(())
}

/// Inherent transport meaning never becomes fabricated authentication/evidence.
///
/// # Errors
/// Propagates unexpected valid source/details admission failure.
///
/// # Panics
/// Fails when checked identity/port/editor selection manufactures observations.
#[test]
fn detail_facts_are_honestly_absent_and_distinguish_plain_ntp() -> Result<(), InspectionError> {
    let owner = SourceDrafts::from_baseline(&[
        RawSourceInput {
            label: "Secure preview",
            host: "clock.example.invalid",
            protocol_choice: 0,
            port: "4460",
        },
        RawSourceInput {
            label: "Plain preview",
            host: "192.0.2.1",
            protocol_choice: 1,
            port: "123",
        },
    ])?;
    let mut inspection = SourceInspection::new();
    inspection.open_details(&owner, 1)?;
    let nts = inspection
        .details(&owner)?
        .ok_or(InspectionError::HiddenSource)?;
    assert_eq!(nts.source().fields().host(), "clock.example.invalid");
    assert_eq!(nts.source().fields().port(), 4460);
    assert_eq!(
        nts.fact(DetailFact::PacketAuthentication),
        AbsentFact::NotEstablished
    );
    assert_eq!(
        nts.fact(DetailFact::LastAuthenticatedMeasurement),
        AbsentFact::NotObserved
    );
    assert_eq!(
        nts.fact(DetailFact::CertificateTimeBasis),
        AbsentFact::NotObserved
    );
    assert_eq!(
        nts.fact(DetailFact::SelectionRole),
        AbsentFact::NotEvaluated
    );
    for fact in [
        DetailFact::OperatorGroup,
        DetailFact::IndependentVoteIdentity,
    ] {
        assert_eq!(nts.fact(fact), AbsentFact::NotEstablished);
    }
    for fact in [
        DetailFact::ActualEndpoint,
        DetailFact::RequestStatus,
        DetailFact::TimeScale,
        DetailFact::LeapAnnouncement,
    ] {
        assert_eq!(nts.fact(fact), AbsentFact::NotObserved);
    }
    for fact in [
        DetailFact::LastReply,
        DetailFact::LastAcceptedMeasurement,
        DetailFact::Offset,
        DetailFact::Delay,
        DetailFact::Uncertainty,
    ] {
        assert_eq!(nts.fact(fact), AbsentFact::NotMeasured);
    }
    inspection.open_details(&owner, 2)?;
    let ntp = inspection
        .details(&owner)?
        .ok_or(InspectionError::HiddenSource)?;
    assert_eq!(ntp.source().protocol(), Protocol::PlainNtp);
    assert_eq!(
        ntp.fact(DetailFact::PacketAuthentication),
        AbsentFact::UnauthenticatedProtocol
    );
    assert_eq!(
        ntp.fact(DetailFact::LastAuthenticatedMeasurement),
        AbsentFact::NotApplicable
    );
    assert_eq!(
        ntp.fact(DetailFact::CertificateTimeBasis),
        AbsentFact::NotApplicable
    );
    assert_eq!(
        ntp.fact(DetailFact::ActualEndpoint),
        AbsentFact::NotObserved
    );
    Ok(())
}
