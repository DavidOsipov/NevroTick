// SPDX-License-Identifier: GPL-2.0-only
//! Structure-aware checks of the native address decoder's borrowed boundary.

use std::{
    cell::Cell,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    ptr::from_mut,
};

use super::{
    DnsAddressError, DnsAddressFamily, DnsAddresses, DnsQueryName, FixtureRecord, copy_fixture,
};
use crate::windows_dns::is_local_dns_name;

/// Exercise the same completion context supplied by both production entry points.
///
/// # Errors
/// Preserve refusal to construct the local name or copy a healthy result.
/// # Panics
/// Reject the same singleton through callback completion and permit ordinary answers.
#[test]
fn local_singleton_requires_immediate_completion() -> Result<(), Box<dyn std::error::Error>> {
    let name: DnsQueryName = DnsQueryName::new("localhost.")?;
    let mut record: FixtureRecord = FixtureRecord::new(0, 4, [127, 0, 0, 1]);
    let checked: Cell<u32> = Cell::new(0);
    let local_name = || {
        checked.set(checked.get() + 1);
        is_local_dns_name(&name)
    };

    let result: DnsAddresses =
        copy_fixture(&mut record, DnsAddressFamily::V4, Some(0), local_name)?;
    assert!(result.iter().eq([IpAddr::V4(Ipv4Addr::LOCALHOST)]));
    assert_eq!(checked.get(), 1);
    assert!(matches!(
        copy_fixture(&mut record, DnsAddressFamily::V4, None, local_name),
        Err(DnsAddressError::Empty)
    ));
    assert_eq!(
        checked.get(),
        1,
        "callback cannot ask for the local-name exception"
    );

    record.header.flags = 1;
    let result: DnsAddresses = copy_fixture(&mut record, DnsAddressFamily::V4, None, local_name)?;
    assert!(result.iter().eq([IpAddr::V4(Ipv4Addr::LOCALHOST)]));
    assert_eq!(checked.get(), 1, "Answer must bypass the local-name check");
    Ok(())
}

/// Build a forward list only after all initialized records are in their final storage.
fn link_records(records: &mut [FixtureRecord]) {
    let mut next: *mut super::RecordHeader = std::ptr::null_mut();
    for record in records.iter_mut().rev() {
        record.header.next = next;
        next = from_mut(record).cast();
    }
}

/// Count all inspected nodes, deduplicate answers and refuse excess distinct addresses.
///
/// # Errors
/// Preserve refusal of the healthy list.
/// # Panics
/// Reject wrong traversal, reordered/dropped addresses, or an exceeded bound accepted as a list.
#[test]
fn callback_answers_keep_order_and_native_list_bounds() -> Result<(), DnsAddressError> {
    let mut records: [FixtureRecord; 33] =
        std::array::from_fn(|_| FixtureRecord::new(2, 4, [192, 0, 2, 9]));
    records[0] = FixtureRecord::new(1, 4, [192, 0, 2, 6]);
    records[1] = FixtureRecord::new(1, 4, [192, 0, 2, 4]);
    records[15] = FixtureRecord::new(1, 4, [192, 0, 2, 6]);
    records[31] = FixtureRecord::new(1, 4, [192, 0, 2, 5]);
    link_records(&mut records[..32]);
    let copied: DnsAddresses = copy_fixture(&mut records[0], DnsAddressFamily::V4, None, || false)?;
    assert!(
        copied
            .iter()
            .eq([6, 4, 5].map(|last: u8| IpAddr::V4(Ipv4Addr::new(192, 0, 2, last))))
    );
    link_records(&mut records);
    assert!(matches!(
        copy_fixture(&mut records[0], DnsAddressFamily::V4, None, || false),
        Err(DnsAddressError::Capacity)
    ));
    records[30].header.flags = 1;
    link_records(&mut records[..32]);
    assert!(matches!(
        copy_fixture(&mut records[0], DnsAddressFamily::V4, None, || false),
        Err(DnsAddressError::Capacity)
    ));
    Ok(())
}

/// A generated raw record and an independent expectation for its boundary result.
struct RecordCase {
    record: FixtureRecord,
    immediate: Option<i32>,
    family: DnsAddressFamily,
    local: bool,
    expected: Result<IpAddr, DnsAddressError>,
    identity_checks: u32,
}

/// Select one semantic partition first, then vary fields within it.
fn record_case(case: u32, seed: u64) -> RecordCase {
    let bytes: [u8; 8] = seed.to_le_bytes();
    let v4: [u8; 4] = [192, 0, bytes[0], bytes[7]];
    let mut sample: RecordCase = RecordCase {
        record: FixtureRecord::new(1, 4, v4),
        immediate: None,
        family: DnsAddressFamily::V4,
        local: true,
        expected: Ok(IpAddr::V4(v4.into())),
        identity_checks: 0,
    };
    match case % 8 {
        0 => {}
        1 => {
            let v6: [u8; 16] = u128::from(seed)
                .wrapping_mul(0x3a8e_7b94_f001_2cd1)
                .to_le_bytes();
            sample.record.header.kind = 28;
            sample.record.header.data_len = 16;
            sample.record.address = v6;
            sample.family = DnsAddressFamily::V6;
            sample.expected = Ok(IpAddr::V6(Ipv6Addr::from(v6)));
        }
        2 => {
            sample.record = FixtureRecord::new(0, 4, [127, 0, 0, 1]);
            sample.immediate = Some(0);
            sample.expected = Ok(IpAddr::V4(Ipv4Addr::LOCALHOST));
            sample.identity_checks = 1;
        }
        3 => {
            sample.record = FixtureRecord::new(0, 4, [127, 0, 0, 1]);
            sample.expected = Err(DnsAddressError::Empty);
        }
        4 => {
            sample.record = FixtureRecord::new(0, 4, [127, 0, 0, 1]);
            sample.immediate = Some(0);
            sample.local = false;
            sample.expected = Err(DnsAddressError::Empty);
            sample.identity_checks = 1;
        }
        5 => {
            sample.record.header.data_len = 5 + u16::from(bytes[2]);
            sample.expected = Err(DnsAddressError::Malformed);
        }
        6 => {
            sample.record.header.flags = 2 + u32::from(bytes[1] & 1);
            sample.immediate = Some(0);
            sample.expected = Err(DnsAddressError::Empty);
        }
        _ => {
            sample.record.header.kind = 5;
            sample.expected = Err(DnsAddressError::Empty);
        }
    }
    sample.record.header.flags |= u32::from_le_bytes([0, bytes[3], bytes[4], bytes[5]]);
    sample
}

/// Vary initialized payloads, lengths and non-section flag bits around independent cases.
///
/// Each replayable case selects a named contract outcome before randomizing the fields
/// that can vary without changing that outcome. All fixture pointers belong to Rust.
///
/// # Panics
/// Fail on a wrong refusal, altered successful address or unexpected identity check.
#[test]
fn structured_record_fuzz_preserves_completion_and_address_admission() {
    let mut seed: u64 = 0xf2c4_95e1_b883_71a0;
    for case in 0_u32..4096 {
        seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
        let mut sample: RecordCase = record_case(case, seed);
        let called: Cell<u32> = Cell::new(0);
        let result: Result<DnsAddresses, DnsAddressError> =
            copy_fixture(&mut sample.record, sample.family, sample.immediate, || {
                called.set(called.get() + 1);
                sample.local
            });
        let observed: Result<Vec<IpAddr>, DnsAddressError> =
            result.map(|addresses: DnsAddresses| addresses.iter().collect());
        assert_eq!(
            observed,
            sample.expected.map(|address: IpAddr| vec![address]),
            "case {case}"
        );
        assert_eq!(
            called.get(),
            sample.identity_checks,
            "identity checks for structured case {case}"
        );
    }
}
