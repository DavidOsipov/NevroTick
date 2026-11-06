//! Unexecuted source controls. Synthetic test authority is not release approval.
use super::*;

/// Borrow the exact staged source snapshot; it has no semantic approval.
const ENGLISH: &[u8] = include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/locales/en.json"));

/// Decode the full source through the intended receiving boundary.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
fn english() -> Result<Catalog, Error> {
    Catalog::checked(json::read(ENGLISH)?)
}
/// Admit independent fixed bytes through the actual mechanism with test-only authority.
/// Production callers cannot choose this authority or populate the review ledger.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
fn synthetic_prior() -> Result<CatalogStore, Error> {
    let bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/test-only-english.cbor"
    ));
    CatalogStore::bootstrap_with_authority(bytes, approvals::Authority::SYNTHETIC_FIXTURE)
}
/// Make one bounded textual mutation from the known staged source.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
fn changed(old: &str, new: &str) -> Result<Vec<u8>, Error> {
    let text = std::str::from_utf8(ENGLISH).map_err(|_| Error::Utf8)?;
    if !text.contains(old) {
        return Err(Error::Schema);
    }
    Ok(text.replacen(old, new, 1).into_bytes())
}

#[test]
/// Equivalent field order and JSON escapes preserve the exact canonical bytes.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
///
/// # Panics
/// Fails when the receiving boundary violates the independently stated assertion.
fn source_and_escape_order_are_canonical() -> Result<(), Error> {
    let source = english()?;
    let escaped = std::str::from_utf8(ENGLISH)
        .map_err(|_| Error::Utf8)?
        .replace('—', "\\u2014");
    let reordered = escaped.replace(
        "\"schema_version\": 2,\n  \"text_contract_version\": 2,",
        "\"text_contract_version\": 2,\n  \"schema_version\": 2,",
    );
    assert_ne!(
        reordered, escaped,
        "the metadata-order control must mutate the source"
    );
    let alternate = Catalog::checked(json::read(reordered.as_bytes())?)?;
    assert_eq!(
        cbor::encode(&source.wire()?)?,
        cbor::encode(&alternate.wire()?)?
    );
    Ok(())
}

#[test]
/// Duplicate decoded keys at each object layer reject; the complete source is the neighbor.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
///
/// # Panics
/// Fails when the receiving boundary violates the independently stated assertion.
fn decoded_duplicates_reject_at_every_object_layer() -> Result<(), Error> {
    let pairs = [
        (
            "root",
            "\"domain\": \"nevrotick.ui\"",
            "\"domain\": \"nevrotick.ui\", \"\\u0064omain\": \"nevrotick.ui\"",
        ),
        (
            "messages",
            "\"ui.fixture.sample-label\": {\n      \"args\": {},\n      \"text\": \"Prototype — sample data\"\n    }",
            "\"ui.fixture.sample-label\": {\n      \"args\": {},\n      \"text\": \"Prototype — sample data\"\n    }, \"\\u0075i.fixture.sample-label\": {\n      \"args\": {},\n      \"text\": \"Prototype — sample data\"\n    }",
        ),
        (
            "message fields",
            "\"args\": {}",
            "\"args\": {}, \"\\u0061rgs\": {}",
        ),
        (
            "arguments",
            "\"minutes\": \"minutes-u64\"",
            "\"minutes\": \"minutes-u64\", \"\\u006dinutes\": \"minutes-u64\"",
        ),
        (
            "plural forms",
            "\"one\": \"Every {minutes} (custom)\"",
            "\"one\": \"Every {minutes} (custom)\", \"\\u006fne\": \"Every {minutes} (custom)\"",
        ),
    ];
    for (layer, old, new) in pairs {
        let error = json::read(&changed(old, new)?).err();
        assert_eq!(error, Some(Error::Duplicate), "decoded {layer} duplicate");
    }
    assert!(english().is_ok());
    Ok(())
}

#[test]
/// Malformed encoding, BOM, bad surrogates and valid-but-forbidden scalars use separate guards.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
///
/// # Panics
/// Fails when the receiving boundary violates the independently stated assertion.
fn raw_utf8_and_decoded_character_failures_stay_distinct() -> Result<(), Error> {
    let mut invalid = ENGLISH.to_vec();
    let at = invalid
        .iter()
        .position(|b| *b == b'N')
        .ok_or(Error::Schema)?;
    *invalid.get_mut(at).ok_or(Error::Schema)? = 0xff;
    assert!(matches!(json::read(&invalid), Err(Error::Utf8)));
    let mut bom = vec![0xef, 0xbb, 0xbf];
    bom.extend_from_slice(ENGLISH);
    assert!(matches!(json::read(&bom), Err(Error::Bom)));
    for scalar in [
        "\\uFFFD", "\\uFEFF", "\\uE000", "\\u202E", "\\u200B", "\\u0378", "\\uFDD0",
    ] {
        let value = format!("\"{scalar}\"");
        assert!(matches!(
            json::read(&changed("\"Prototype — sample data\"", &value)?),
            Err(Error::Character)
        ));
    }
    assert!(matches!(
        json::read(&changed("\"Prototype — sample data\"", "\"\\uD83D\"")?),
        Err(Error::Json)
    ));
    // A legitimate supplementary letter decodes correctly, then fails the
    // initial English repertoire; this is not a surrogate/UTF8 error.
    assert!(matches!(
        json::read(&changed(
            "\"Prototype — sample data\"",
            "\"\\uD801\\uDC00\""
        )?),
        Err(Error::Character)
    ));
    Ok(())
}

#[test]
/// Literal/escaped emoji and components reject while the frozen punctuation/digit neighbors admit.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
///
/// # Panics
/// Fails when the receiving boundary violates the independently stated assertion.
fn emoji_variants_and_escaped_forms_reject_but_ascii_neighbors_work() -> Result<(), Error> {
    for text in [
        "😀",
        "\\uD83D\\uDE00",
        "🇬🇧",
        "1\\uFE0F\\u20E3",
        "*\\u20E3",
        "👍🏽",
        "👩‍🔬",
        "©\\uFE0E",
        "™",
        "🏴\\uDB40\\uDC67",
    ] {
        assert!(matches!(
            json::read(&changed(
                "\"Prototype — sample data\"",
                &format!("\"{text}\"")
            )?),
            Err(Error::Character)
        ));
    }
    let neighbor = changed(
        "\"Prototype — sample data\"",
        "\"ASCII 0123456789 # * · – —\"",
    )?;
    assert!(Catalog::checked(json::read(&neighbor)?).is_ok());
    Ok(())
}

#[test]
/// Closed fields/types/integers and exact document consumption reject one-fault candidates.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
///
/// # Panics
/// Fails when the receiving boundary violates the independently stated assertion.
fn closed_shape_numbers_and_single_document_rules_reject() -> Result<(), Error> {
    for token in [
        "1.0",
        "1e0",
        "-0",
        "+1",
        "01",
        "4294967296",
        "null",
        "true",
        "\"1\"",
    ] {
        let candidate = changed(
            "\"schema_version\": 2",
            &format!("\"schema_version\": {token}"),
        )?;
        assert!(json::read(&candidate).and_then(Catalog::checked).is_err());
    }
    for (old, new) in [
        (
            "\"domain\": \"nevrotick.ui\"",
            "\"domain\": \"nevrotick.ui\", \"verified\": \"true\"",
        ),
        ("\"locale\": \"en\"", "\"locale\": \"ru\""),
        ("\"args\": {}", "\"args\": {\"injected\": \"u32\"}"),
        (
            "\"text\": \"Prototype — sample data\"",
            "\"text\": \"Prototype — sample data\", \"forms\": {}",
        ),
        ("\"ui.fixture.sample-label\"", "\"ui.not-a-contracted-id\""),
    ] {
        assert!(
            json::read(&changed(old, new)?)
                .and_then(Catalog::checked)
                .is_err()
        );
    }
    let mut trailing = ENGLISH.to_vec();
    trailing.extend_from_slice(b" {}");
    assert!(json::read(&trailing).is_err());
    let mut whitespace = ENGLISH.to_vec();
    whitespace.extend_from_slice(b" \r\n\t");
    assert!(Catalog::checked(json::read(&whitespace)?).is_ok());
    Ok(())
}

#[test]
/// Exact fact multiplicity and typed units survive singular and maximal integer formatting.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
///
/// # Panics
/// Fails when the receiving boundary violates the independently stated assertion.
fn placeholders_are_typed_nonrecursive_and_required_per_plural_arm() -> Result<(), Error> {
    assert!(placeholders("literal {{minutes}}", false).is_ok());
    assert!(placeholders("{minutes}", true).is_ok());
    for text in [
        "no required fact",
        "{minutes}{minutes}",
        "{unknown}",
        "{minutes:code}",
        "{nested{minutes}}",
    ] {
        assert!(placeholders(text, true).is_err());
    }
    let one = NonZeroU64::new(1).ok_or(Error::Arguments)?;
    let many = NonZeroU64::new(u64::MAX).ok_or(Error::Arguments)?;
    let message = Message::Minutes {
        one: "Every {minutes} (custom)".into(),
        other: "Every {minutes} (custom)".into(),
    };
    assert_eq!(
        render(&message, Some(one))?.as_str(),
        "Every 1 minute (custom)"
    );
    assert_eq!(
        render(&message, Some(many))?.as_str(),
        "Every 18446744073709551615 minutes (custom)"
    );
    assert!(render(&message, None).is_err());
    Ok(())
}

#[test]
/// Oversized bytes, strings, nesting and declared CBOR lengths reject before unbounded growth.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
///
/// # Panics
/// Fails when the receiving boundary violates the independently stated assertion.
fn encoded_string_depth_and_declared_lengths_are_bounded() -> Result<(), Error> {
    assert!(matches!(
        json::read(&vec![b' '; JSON_BYTES + 1]),
        Err(Error::Size)
    ));
    let too_long = changed(
        "\"Prototype — sample data\"",
        &format!("\"{}\"", "A".repeat(STRING_BYTES + 1)),
    )?;
    assert!(json::read(&too_long).is_err());
    assert!(matches!(
        json::read(br#"{"a":{"b":{"c":{"d":{}}}}}"#),
        Err(Error::Bounds)
    ));
    assert!(cbor::read(&[0xbb, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]).is_err());
    assert!(cbor::read(&[0x7b, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff]).is_err());
    Ok(())
}

#[test]
/// Hand-authored CBOR bytes independently check canonical primitives and denied alternatives.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
///
/// # Panics
/// Fails when the receiving boundary violates the independently stated assertion.
fn independent_small_cbor_vector_and_forbidden_alternatives() -> Result<(), Error> {
    // Independently written RFC8949 vector: {"a":1,"b":"x"}.
    let vector = [0xa2, 0x61, b'a', 0x01, 0x61, b'b', 0x61, b'x'];
    let wire = cbor::read(&vector)?;
    assert_eq!(cbor::encode(&wire)?, vector);
    for bad in [
        vec![0xbf, 0xff],
        vec![0xc0, 0xa0],
        vec![0x81, 0x01],
        vec![0xf4],
        vec![0xf6],
        vec![0x20],
        vec![0x18, 0x01],
        vec![0xa1, 0x18, 0x01, 0x01],
        vec![0xa2, 0x61, b'a', 0x01, 0x61, b'a', 0x02],
        vec![0xa2, 0x61, b'b', 0x01, 0x61, b'a', 0x02],
    ] {
        assert!(cbor::read(&bad).is_err());
    }
    Ok(())
}

#[test]
/// Runtime rechecks decoded emoji and retains a synthetic prior on malformed candidates.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
///
/// # Panics
/// Fails when the receiving boundary violates the independently stated assertion.
fn runtime_cbor_rechecks_emoji_and_preserves_prior_on_malformed_data() -> Result<(), Error> {
    let mut store = synthetic_prior()?;
    let valid = cbor::encode(&store.english.wire()?)?;
    let needle = b"\x78\x19Prototype \xe2\x80\x94 sample data";
    let at = valid
        .windows(needle.len())
        .position(|v| v == needle)
        .ok_or(Error::Schema)?;
    let mut bad = valid.clone();
    drop(bad.splice(at..at + needle.len(), [0x64, 0xf0, 0x9f, 0x98, 0x80]));
    assert!(matches!(store.replace(&bad), Err(Error::Character)));
    assert_eq!(cbor::encode(&store.snapshot().catalog.wire()?)?, valid);
    let mut trailing = valid.clone();
    trailing.push(0);
    assert!(store.replace(&trailing).is_err());
    assert_eq!(cbor::encode(&store.snapshot().catalog.wire()?)?, valid);
    Ok(())
}

#[test]
/// Technically valid lying wording has no grant and falls back coherently to test English.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
///
/// # Panics
/// Fails when the receiving boundary violates the independently stated assertion.
fn compatibility_and_valid_syntax_cannot_self_approve_english() -> Result<(), Error> {
    let canonical = cbor::encode(&english()?.wire()?)?;
    assert!(matches!(pack_approved_json(ENGLISH), Err(Error::Approval)));
    assert!(matches!(
        CatalogStore::bootstrap_english(&canonical),
        Err(Error::Approval)
    ));
    let mut store = synthetic_prior()?;
    // Use a semantically wrong, syntactically safe wording while retaining all
    // metadata/types/IDs. It must never become displayed critical content.
    let candidate = changed(
        "\"text\": \"Not established\"",
        "\"text\": \"Authenticated and safe\"",
    )?;
    let technically_valid = Catalog::checked(json::read(&candidate)?)?;
    let unapproved = cbor::encode(&technically_valid.wire()?)?;
    let prior = cbor::encode(&store.snapshot().catalog.wire()?)?;
    assert_eq!(store.replace(&unapproved)?, Update::EnglishFallback);
    assert_eq!(cbor::encode(&store.snapshot().catalog.wire()?)?, prior);
    Ok(())
}

#[test]
/// Explicit fixed test authority proves the mechanism without a production grant.
///
/// # Errors
/// Propagates a fixed setup or admission failure while preparing the discriminating control.
///
/// # Panics
/// Fails when the receiving boundary violates the independently stated assertion.
fn fixed_independent_approval_fixture_and_unapproved_candidate_are_distinct() -> Result<(), Error> {
    let bytes = include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/fixtures/test-only-english.cbor"
    ));
    let candidate = prepare_unapproved_json(ENGLISH)?;
    assert_eq!(candidate.bytes_for_review(), bytes);
    assert!(matches!(
        CatalogStore::bootstrap_english(candidate.bytes_for_review()),
        Err(Error::Approval)
    ));
    let approved_for_test = pack_with_authority(ENGLISH, approvals::Authority::SYNTHETIC_FIXTURE)?;
    assert_eq!(approved_for_test.bytes(), bytes);
    let store = synthetic_prior()?;
    let id = MessageId::from_key("ui.fixture.sample-label")?;
    assert_eq!(
        store.snapshot().text(id)?.as_str(),
        "Prototype — sample data"
    );
    Ok(())
}

/// Stable retired identities cannot be revived by otherwise valid catalog data.
///
/// # Panics
/// Fails if an ended meaning is silently reused or accepted as active.
#[test]
fn retired_ids_remain_closed() {
    for key in [
        "ui.fixture.repository-value",
        "ui.fixture.artwork-unavailable",
        "ui.fixture.profile-value",
        "ui.fixture.statistics-empty-description",
        "ui.fixture.port-help",
        "ui.interval.two-minutes",
        "ui.preferences.interval-option.two-minutes",
    ] {
        assert!(matches!(MessageId::from_key(key), Err(Error::Schema)));
    }
}

/// File schema and text contract are exact; GUI build is not payload authority.
///
/// # Errors
/// Propagates unexpected admission/setup failure in the complete healthy control.
///
/// # Panics
/// Fails if an old/future or informational-build field can bypass compatibility.
#[test]
fn schema_text_versions_and_exact_fingerprint_fail_closed() -> Result<(), Error> {
    assert!(english().is_ok());
    for (old, new) in [
        ("\"schema_version\": 2", "\"schema_version\": 1"),
        ("\"schema_version\": 2", "\"schema_version\": 3"),
        (
            "\"text_contract_version\": 2",
            "\"text_contract_version\": 1",
        ),
        (
            "\"text_contract_version\": 2",
            "\"text_contract_version\": 3",
        ),
        (
            "\"text_contract_version\": 2",
            "\"text_contract_version\": 2.0",
        ),
        (
            "\"text_contract_version\": 2",
            "\"text_contract_version\": 2e0",
        ),
        (
            "\"text_contract_version\": 2",
            "\"text_contract_version\": \"2\"",
        ),
        (
            "\"domain\": \"nevrotick.ui\"",
            "\"domain\": \"nevrotick.ui\", \"gui_build_version\": 1",
        ),
    ] {
        assert!(
            json::read(&changed(old, new)?)
                .and_then(Catalog::checked)
                .is_err()
        );
    }
    let wrong = if FINGERPRINT.starts_with('0') {
        "1".repeat(64)
    } else {
        "0".repeat(64)
    };
    let bad = changed(FINGERPRINT, &wrong)?;
    assert!(matches!(
        json::read(&bad).and_then(Catalog::checked),
        Err(Error::Schema)
    ));
    Ok(())
}

/// Required English is complete and meaningful; no unknown/retired ID revives.
///
/// # Errors
/// Propagates unexpected source/setup failure in the healthy complete control.
///
/// # Panics
/// Fails if an orphan, revived ID, empty or whitespace-only required value admits.
#[test]
fn required_ids_and_nonblank_values_are_closed() -> Result<(), Error> {
    for candidate in [
        changed("\"ui.fixture.sample-label\"", "\"ui.no-owned-sink.text\"")?,
        changed(
            "\"ui.fixture.sample-label\"",
            "\"ui.fixture.repository-value\"",
        )?,
        changed("\"text\": \"Prototype — sample data\"", "\"text\": \"\"")?,
        changed("\"text\": \"Prototype — sample data\"", "\"text\": \"   \"")?,
        changed(
            "\"messages\": {",
            "\"messages\": {\"ui.no-owned-sink.text\": {\"args\": {}, \"text\": \"Extra\"},",
        )?,
    ] {
        assert!(json::read(&candidate).and_then(Catalog::checked).is_err());
    }
    let entry = "    \"ui.fixture.sample-label\": {\n      \"args\": {},\n      \"text\": \"Prototype — sample data\"\n    },\n";
    assert!(matches!(
        json::read(&changed(entry, "")?).and_then(Catalog::checked),
        Err(Error::IncompleteEnglish)
    ));
    assert!(english().is_ok());
    Ok(())
}

/// Independently retained English remains selected on malformed binary metadata.
///
/// # Errors
/// Propagates unexpected fixture admission, wire setup or canonical encoding failure.
///
/// # Panics
/// Fails if a malformed binary replacement changes its previous admitted snapshot.
#[test]
fn runtime_version_and_missing_text_failures_preserve_prior() -> Result<(), Error> {
    let mut store = synthetic_prior()?;
    let prior = cbor::encode(&store.snapshot().catalog.wire()?)?;
    for (field, value) in [
        ("schema_version", 1),
        ("schema_version", 3),
        ("text_contract_version", 1),
        ("text_contract_version", 3),
    ] {
        let mut fields = english()?.wire()?.map()?;
        let slot = fields
            .iter_mut()
            .find(|(name, _)| name == field)
            .ok_or(Error::Schema)?;
        slot.1 = Wire::Uint(value);
        let bad = cbor::encode(&Wire::Map(fields))?;
        assert!(matches!(store.replace(&bad), Err(Error::Schema)));
        assert_eq!(cbor::encode(&store.snapshot().catalog.wire()?)?, prior);
    }
    let mut fields = english()?.wire()?.map()?;
    let messages = fields
        .iter_mut()
        .find(|(name, _)| name == "messages")
        .ok_or(Error::Schema)?;
    let Wire::Map(entries) = &mut messages.1 else {
        return Err(Error::Schema);
    };
    let at = entries
        .iter()
        .position(|(name, _)| name == "ui.fixture.sample-label")
        .ok_or(Error::Schema)?;
    drop(entries.remove(at));
    let missing = cbor::encode(&Wire::Map(fields))?;
    assert!(matches!(
        store.replace(&missing),
        Err(Error::IncompleteEnglish)
    ));
    assert_eq!(cbor::encode(&store.snapshot().catalog.wire()?)?, prior);
    Ok(())
}

/// Native test feedback preserves the original result and appends only a fixed tail.
///
/// # Errors
/// Propagates unexpected exact-fixture admission or protected rendering failure.
///
/// # Panics
/// Fails if a repeated test loses its prior certainty or introduces arbitrary text.
#[test]
fn native_notification_feedback_keeps_closed_sticky_meanings() -> Result<(), Error> {
    let store = synthetic_prior()?;
    let snapshot = store.snapshot();
    for (accepted, coalesced, expected) in [
        (
            true,
            false,
            "Windows accepted the test request. A banner or reading is not confirmed.",
        ),
        (
            false,
            false,
            "The test request outcome is unconfirmed. A banner or reading is not confirmed.",
        ),
        (
            true,
            true,
            "Windows accepted the test request. A banner or reading is not confirmed. This test will not be submitted again during this app session.",
        ),
        (
            false,
            true,
            "The test request outcome is unconfirmed. A banner or reading is not confirmed. This test will not be submitted again during this app session.",
        ),
    ] {
        assert_eq!(
            UiText::notification_test_feedback(accepted, coalesced)?
                .render(&snapshot)?
                .as_str(),
            expected
        );
    }
    assert_eq!(snapshot.revision(), 2);
    assert!(matches!(
        MessageId::from_key("ui.shell.no-notification-submission"),
        Err(Error::Schema)
    ));
    Ok(())
}
