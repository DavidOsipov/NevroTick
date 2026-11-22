// SPDX-License-Identifier: GPL-2.0-only
//! Checked DNS query names and bounded address observations.
//!
//! The native adapter owns query completion. The consumer retains authority
//! for route, endpoint, and current source policy.

use std::sync::atomic::{AtomicBool, Ordering};
use std::{error::Error, fmt, net::IpAddr};
use std::{
    sync::mpsc::{Receiver, RecvTimeoutError, TryRecvError},
    time::Duration,
};

/// Maximum ASCII presentation length, including the final root separator.
const MAX_ABSOLUTE_NAME_BYTES: usize = 254;
/// Retained space for an admitted absolute name and its zero terminator.
const WIDE_NAME_UNITS: usize = MAX_ABSOLUTE_NAME_BYTES + 1;
/// Maximum label length on the DNS wire.
const MAX_LABEL_BYTES: usize = 63;
/// Maximum number of distinct addresses retained from one result.
const MAX_ADDRESSES: usize = 3;

/// An absolute ASCII DNS hostname, folded to lowercase and terminated in UTF-16.
#[must_use = "pass the checked name to the actual native query owner"]
pub struct DnsQueryName {
    /// Private storage keeps the terminator and text validation together.
    wide: [u16; WIDE_NAME_UNITS],
}

impl DnsQueryName {
    /// Admit an absolute ASCII hostname with a final root separator.
    ///
    /// # Errors
    /// Refuse a missing root separator, oversized name, invalid label, or
    /// a name composed solely of numeric address-like components.
    pub fn new(input: &str) -> Result<Self, DnsNameError> {
        if input.len() > MAX_ABSOLUTE_NAME_BYTES {
            return Err(DnsNameError::NameTooLong);
        }
        let labels: &[u8] = input
            .as_bytes()
            .strip_suffix(b".")
            .ok_or(DnsNameError::MissingRoot)?;
        if !labels.split(|byte| *byte == b'.').all(valid_label) {
            return Err(DnsNameError::InvalidLabel);
        }
        if labels.split(|byte| *byte == b'.').all(numeric_component) {
            return Err(DnsNameError::AddressLiteral);
        }

        let mut wide: [u16; WIDE_NAME_UNITS] = [0; WIDE_NAME_UNITS];
        for (unit, byte) in wide.iter_mut().zip(input.bytes()) {
            *unit = u16::from(byte.to_ascii_lowercase());
        }
        Ok(Self { wide })
    }

    /// Borrow a stable terminated pointer for the lifetime of this storage.
    #[cfg(windows)]
    pub(super) const fn as_ptr(&self) -> *const u16 {
        self.wide.as_ptr()
    }

    /// Recognize the exact checked absolute localhost name.
    #[cfg(windows)]
    pub(super) fn is_localhost(&self) -> bool {
        let localhost: &str = "localhost.\0";
        self.wide
            .iter()
            .copied()
            .take(localhost.len())
            .eq(localhost.encode_utf16())
    }

    /// Compare an OS-reported DNS name using ASCII folding and one final root.
    /// The reported slice excludes the terminating zero from the native buffer.
    #[cfg(any(windows, test))]
    pub(super) fn matches_reported_dns_name(&self, reported: &[u16]) -> bool {
        let root: u16 = u16::from(b'.');
        let reported: &[u16] = reported.strip_suffix(&[root]).unwrap_or(reported);
        let Some((name, suffix)): Option<(&[u16], &[u16])> =
            self.wide.split_at_checked(reported.len())
        else {
            return false;
        };
        suffix.starts_with(&[root, 0])
            && name.iter().zip(reported).all(|(checked, unit)| {
                u8::try_from(*unit)
                    .is_ok_and(|byte: u8| u16::from(byte.to_ascii_lowercase()) == *checked)
            })
    }
}

impl fmt::Debug for DnsQueryName {
    /// Keep query names out of diagnostic formatting.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DnsQueryName")
            .finish_non_exhaustive()
    }
}

/// Check one nonempty hostname label before materializing any UTF-16 storage.
fn valid_label(label: &[u8]) -> bool {
    (1..=MAX_LABEL_BYTES).contains(&label.len())
        && label.first().is_some_and(u8::is_ascii_alphanumeric)
        && label.last().is_some_and(u8::is_ascii_alphanumeric)
        && label
            .iter()
            .all(|byte| byte.is_ascii_alphanumeric() || *byte == b'-')
}

/// Recognize decimal or hexadecimal components with address-like spelling.
fn numeric_component(label: &[u8]) -> bool {
    if let Some(hex_digits) = label
        .strip_prefix(b"0x")
        .or_else(|| label.strip_prefix(b"0X"))
    {
        return !hex_digits.is_empty() && hex_digits.iter().all(u8::is_ascii_hexdigit);
    }
    label.iter().all(u8::is_ascii_digit)
}

/// Bounded reason for rejecting a proposed DNS hostname.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DnsNameError {
    /// The presentation exceeded the maximum absolute name length.
    NameTooLong,
    /// The final root separator was missing.
    MissingRoot,
    /// An empty, oversized, or syntactically invalid hostname label was present.
    InvalidLabel,
    /// The absolute name consisted only of numeric address-like components.
    AddressLiteral,
}

impl fmt::Display for DnsNameError {
    /// Report the failure category without exposing the proposed name.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::NameTooLong => "DNS query name exceeds the length limit",
            Self::MissingRoot => "DNS query name requires a final root separator",
            Self::InvalidLabel => "DNS query name has an invalid label",
            Self::AddressLiteral => "DNS query name resembles an address literal",
        })
    }
}

impl Error for DnsNameError {}

/// Requested address family for one DNS query.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DnsAddressFamily {
    /// IPv4 from type A records.
    V4,
    /// IPv6 from type AAAA records.
    V6,
}

impl DnsAddressFamily {
    /// Select the native query and result record type for this family.
    #[cfg(any(windows, test))]
    pub(super) const fn wire_record_type(self) -> u16 {
        match self {
            Self::V4 => 1,
            Self::V6 => 28,
        }
    }
}

/// Ordered address observations retained from one DNS result.
///
/// Values retain their observed address family. The consumer owns their
/// endpoint, scope, provenance and authorization checks.
#[must_use = "consume the observed addresses under current endpoint policy"]
pub struct DnsAddresses {
    /// Dense prefix of at most three distinct addresses, followed by empty slots.
    entries: [Option<IpAddr>; MAX_ADDRESSES],
}

impl DnsAddresses {
    /// Start the native adapter's local accumulation before publication.
    pub(super) const fn empty() -> Self {
        Self {
            entries: [None; MAX_ADDRESSES],
        }
    }

    /// Accumulate one address in first-observed order.
    ///
    /// # Errors
    /// Refuse a fourth distinct address, preserving the original staged set.
    pub(super) fn push(&mut self, address: IpAddr) -> Result<(), DnsAddressError> {
        if self.entries.contains(&Some(address)) {
            return Ok(());
        }
        let slot: &mut Option<IpAddr> = self
            .entries
            .iter_mut()
            .find(|entry| entry.is_none())
            .ok_or(DnsAddressError::Capacity)?;
        *slot = Some(address);
        Ok(())
    }

    /// Read observed addresses in first-observed order.
    pub fn iter(&self) -> impl Iterator<Item = IpAddr> + '_ {
        self.entries.iter().flatten().copied()
    }

    /// Count the distinct observed addresses.
    #[must_use]
    pub fn len(&self) -> usize {
        self.iter().count()
    }

    /// Check whether the result has any observed addresses.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.iter().next().is_none()
    }
}

impl fmt::Debug for DnsAddresses {
    /// Keep observed addresses out of diagnostic formatting.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("DnsAddresses")
            .finish_non_exhaustive()
    }
}

/// Bounded failure from native query admission or actual completion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DnsAddressError {
    /// Preserve the native nonzero DNS status.
    Native(i32),
    /// The result could not be read under the admitted record contract.
    Malformed,
    /// The result exceeded the retained work or address capacity.
    Capacity,
    /// The completed result contained no usable address of the requested family.
    Empty,
    /// The native query owner already holds its bounded work allowance.
    Busy,
}

impl fmt::Display for DnsAddressError {
    /// Report the failure category without exposing query contents.
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Native(_) => "native DNS query failed",
            Self::Malformed => "native DNS record did not match the expected layout",
            Self::Capacity => "native DNS result exceeded the capacity limit",
            Self::Empty => "native DNS result contained no usable address",
            Self::Busy => "native DNS query capacity is busy",
        })
    }
}

impl Error for DnsAddressError {}

/// Result of asking the system DNS Client to cancel a pending lookup.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[must_use = "retain cancellation separately from the actual DNS completion"]
pub enum DnsCancellationRequest {
    /// Completion won, or no pending query admitted a cancellation.
    Skipped,
    /// The native request returned success; completion is still independent.
    Requested,
    /// The native request returned a nonzero DNS status.
    Rejected(i32),
}

/// Delay between retirement observations on the admitted blocking worker.
const RETIREMENT_CHECK: Duration = Duration::from_millis(10);

/// Wait for the provider's actual result. Retirement permits one request, then
/// the worker waits solely for completion with no further timer wakeups.
pub(super) fn wait_for_dns_result(
    receiver: &Receiver<Result<DnsAddresses, DnsAddressError>>,
    mut retired: impl FnMut() -> bool,
    request: impl FnOnce() -> DnsCancellationRequest,
) -> (
    Result<DnsAddresses, DnsAddressError>,
    DnsCancellationRequest,
) {
    loop {
        if retired() {
            return match receiver.try_recv() {
                Ok(outcome) => (outcome, DnsCancellationRequest::Skipped),
                Err(TryRecvError::Disconnected) => (
                    Err(DnsAddressError::Malformed),
                    DnsCancellationRequest::Skipped,
                ),
                Err(TryRecvError::Empty) => {
                    let cancellation: DnsCancellationRequest = request();
                    let result: Result<DnsAddresses, DnsAddressError> =
                        receiver.recv().unwrap_or(Err(DnsAddressError::Malformed));
                    (result, cancellation)
                }
            };
        }
        match receiver.recv_timeout(RETIREMENT_CHECK) {
            Ok(outcome) => return (outcome, DnsCancellationRequest::Skipped),
            Err(RecvTimeoutError::Disconnected) => {
                return (
                    Err(DnsAddressError::Malformed),
                    DnsCancellationRequest::Skipped,
                );
            }
            Err(RecvTimeoutError::Timeout) => {}
        }
    }
}

/// One native-operation permit. Keep it with the provider storage until completion.
pub(super) struct DnsQueryPermit<'a> {
    /// Actual process admission cell.
    slot: &'a AtomicBool,
}

impl<'a> DnsQueryPermit<'a> {
    /// Reserve one source slot without waiting or allocating.
    /// # Errors
    /// Refuse while the actual previous operation retains its permit.
    pub(super) fn acquire(slot: &'a AtomicBool) -> Result<Self, DnsAddressError> {
        slot.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed)
            .map(|_| Self { slot })
            .map_err(|_| DnsAddressError::Busy)
    }
}

impl Drop for DnsQueryPermit<'_> {
    fn drop(&mut self) {
        self.slot.store(false, Ordering::Release);
    }
}

/// Challenge the actual provider latch independently of receiver delivery.
/// # Errors
/// Return the original error from either required successful reservation.
/// # Panics
/// Refuse early reuse after the consumer has abandoned delivery.
#[cfg(test)]
#[test]
fn receiver_retirement_does_not_release_provider_permit() -> Result<(), DnsAddressError> {
    use std::sync::mpsc::{TrySendError, sync_channel};

    let slot: AtomicBool = AtomicBool::new(false);
    let owner: DnsQueryPermit<'_> = DnsQueryPermit::acquire(&slot)?;
    let (completion, receiver) = sync_channel::<()>(1);
    drop(receiver);
    assert!(matches!(
        completion.try_send(()),
        Err(TrySendError::Disconnected(()))
    ));
    assert!(matches!(
        DnsQueryPermit::acquire(&slot),
        Err(DnsAddressError::Busy)
    ));
    drop(owner);
    let _next: DnsQueryPermit<'_> = DnsQueryPermit::acquire(&slot)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        net::{Ipv4Addr, Ipv6Addr},
        sync::mpsc::{SyncSender, TrySendError, sync_channel},
    };

    /// Provider channel message retains the original result independently of cancellation.
    type Outcome = Result<DnsAddresses, DnsAddressError>;

    /// Both observations returned by the blocking waiter.
    type Waited = (Outcome, DnsCancellationRequest);

    /// A ready provider result wins even when the caller has already retired.
    /// # Errors
    /// Return a failed fixture channel send.
    /// # Panics
    /// Reject a spurious request or replacement of the provider result.
    #[test]
    fn completed_dns_outcome_never_requests_cancellation() -> Result<(), Box<dyn Error>> {
        let called: std::cell::Cell<bool> = std::cell::Cell::new(false);
        let (completion, receiver): (SyncSender<Outcome>, Receiver<Outcome>) = sync_channel(1);
        completion.send(Err(DnsAddressError::Native(9_003)))?;
        let (outcome, cancellation): Waited = wait_for_dns_result(
            &receiver,
            || true,
            || {
                called.set(true);
                DnsCancellationRequest::Requested
            },
        );
        assert!(!called.get());
        assert!(matches!(outcome, Err(DnsAddressError::Native(9_003))));
        assert_eq!(cancellation, DnsCancellationRequest::Skipped);
        Ok(())
    }

    /// Both successful and refused requests preserve the original provider result.
    /// # Errors
    /// Return a bounded channel or unexpected worker failure.
    /// # Panics
    /// Detect early worker return or any change to the separation of outcomes.
    #[test]
    fn cancellation_request_waits_for_provider_completion() -> Result<(), Box<dyn Error>> {
        use std::{
            io,
            sync::Arc,
            thread::{self, JoinHandle},
        };

        for expected in [
            DnsCancellationRequest::Requested,
            DnsCancellationRequest::Rejected(87),
        ] {
            let (completion, receiver): (SyncSender<Outcome>, Receiver<Outcome>) = sync_channel(1);
            let (requested, observed): (
                SyncSender<DnsCancellationRequest>,
                Receiver<DnsCancellationRequest>,
            ) = sync_channel(1);
            let (waiting, observed_waiting): (SyncSender<bool>, Receiver<bool>) = sync_channel(1);
            let retired: Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
            let retirement: Arc<AtomicBool> = Arc::clone(&retired);
            let worker: JoinHandle<Waited> = thread::Builder::new().spawn(move || {
                let mut initial: Option<SyncSender<bool>> = Some(waiting);
                wait_for_dns_result(
                    &receiver,
                    || {
                        let current: bool = retirement.load(Ordering::Acquire);
                        if let Some(waiting) = initial.take() {
                            let _notice: Result<(), TrySendError<bool>> = waiting.try_send(current);
                        }
                        current
                    },
                    || {
                        let _notice: Result<(), TrySendError<DnsCancellationRequest>> =
                            requested.try_send(expected);
                        expected
                    },
                )
            })?;
            assert!(!observed_waiting.recv_timeout(Duration::from_secs(5))?);
            retired.store(true, Ordering::Release);
            assert_eq!(observed.recv_timeout(Duration::from_secs(5))?, expected);
            assert!(!worker.is_finished());
            completion.send(Err(DnsAddressError::Native(9_503)))?;
            let (outcome, cancellation): Waited = worker
                .join()
                .map_err(|_| io::Error::other("DNS wait control panicked"))?;
            assert!(matches!(outcome, Err(DnsAddressError::Native(9_503))));
            assert_eq!(cancellation, expected);
        }
        Ok(())
    }

    /// Require the exact Windows DNS name after ASCII folding and root handling.
    /// # Errors
    /// Propagate an invalid healthy query fixture.
    /// # Panics
    /// Reject partial names, suffixes, multiple roots and invalid reported units.
    #[test]
    fn reported_dns_name_matches_only_exact_absolute_name() -> Result<(), DnsNameError> {
        let qualified: DnsQueryName = DnsQueryName::new("Node.Example.")?;
        let domainless: DnsQueryName = DnsQueryName::new("Node.")?;
        for reported in ["node.example", "NODE.EXAMPLE."] {
            let units: Vec<u16> = reported.encode_utf16().collect();
            assert!(qualified.matches_reported_dns_name(&units));
            assert!(!domainless.matches_reported_dns_name(&units));
        }
        for reported in ["node", "NODE."] {
            let units: Vec<u16> = reported.encode_utf16().collect();
            assert!(domainless.matches_reported_dns_name(&units));
            assert!(!qualified.matches_reported_dns_name(&units));
        }
        for reported in [
            "",
            "node.example..",
            "other.node.example",
            "node.example.invalid",
            "node.example\0",
            "nóde.example",
        ] {
            let units: Vec<u16> = reported.encode_utf16().collect();
            assert!(!qualified.matches_reported_dns_name(&units));
        }
        Ok(())
    }

    /// Reject ambiguous or unsupported names beside admitted absolute hostnames.
    /// # Panics
    /// Fail if either an accepted name or a named refusal changes behavior.
    #[test]
    fn query_names_enforce_absolute_ascii_hostname_syntax() {
        for input in [
            "node.",
            "node-2.Example.",
            "127.0.0.1.example.",
            "xn--bcher-kva.example.",
        ] {
            assert!(DnsQueryName::new(input).is_ok(), "{input}");
        }
        for (input, error) in [
            ("node.example", DnsNameError::MissingRoot),
            (".", DnsNameError::InvalidLabel),
            ("node..example.", DnsNameError::InvalidLabel),
            ("node.example..", DnsNameError::InvalidLabel),
            ("-node.example.", DnsNameError::InvalidLabel),
            ("node-.example.", DnsNameError::InvalidLabel),
            ("node_.example.", DnsNameError::InvalidLabel),
            ("node/example.", DnsNameError::InvalidLabel),
            ("node\\example.", DnsNameError::InvalidLabel),
            ("node\0.example.", DnsNameError::InvalidLabel),
            ("node.example. ", DnsNameError::MissingRoot),
            ("*.example.", DnsNameError::InvalidLabel),
            ("bücher.example.", DnsNameError::InvalidLabel),
            ("127.0.0.1.", DnsNameError::AddressLiteral),
            ("127.000.000.001.", DnsNameError::AddressLiteral),
            ("0x7f000001.", DnsNameError::AddressLiteral),
            ("::1.", DnsNameError::InvalidLabel),
        ] {
            assert_eq!(DnsQueryName::new(input).err(), Some(error), "{input}");
        }
    }

    /// Assert the exact name and terminator boundary independently of the API.
    /// # Errors
    /// Return an unexpected refusal of a known valid hostname.
    /// # Panics
    /// Fail if the stored labels, lengths, terminator or redaction differ.
    #[test]
    fn name_storage_preserves_root_and_terminator() -> Result<(), DnsNameError> {
        let label: String = "x".repeat(63);
        let longest: String = format!("{label}.{label}.{label}.{}.", "y".repeat(61));
        let longest_name: DnsQueryName = DnsQueryName::new(&longest)?;
        assert_eq!(longest_name.wide.get(253), Some(&u16::from(b'.')));
        assert_eq!(longest_name.wide.get(254), Some(&0));
        let too_long: String = format!("{label}.{label}.{label}.{}.", "y".repeat(62));
        assert_eq!(
            DnsQueryName::new(&too_long).err(),
            Some(DnsNameError::NameTooLong)
        );
        let too_wide: String = format!("{}.example.", "x".repeat(64));
        assert_eq!(
            DnsQueryName::new(&too_wide).err(),
            Some(DnsNameError::InvalidLabel)
        );

        let folded: DnsQueryName = DnsQueryName::new("A.Example.")?;
        let expected: Vec<u16> = "a.example.".encode_utf16().collect();
        assert!(folded.wide.starts_with(&expected));
        assert_eq!(folded.wide.get(expected.len()), Some(&0));
        assert_eq!(format!("{folded:?}"), "DnsQueryName { .. }");
        Ok(())
    }

    /// Check A and AAAA selection beside bounded, deduplicated record collection.
    /// # Errors
    /// Return a failure while inserting a record within the admitted capacity.
    /// # Panics
    /// Fail if the capacity, order, record types or redaction differ.
    #[test]
    fn address_collection_retains_order_and_refuses_partial_overflow() -> Result<(), DnsAddressError>
    {
        assert_eq!(DnsAddressFamily::V4.wire_record_type(), 1);
        assert_eq!(DnsAddressFamily::V6.wire_record_type(), 28);
        let first: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 0, 2, 1));
        let second: IpAddr = IpAddr::V6(Ipv6Addr::LOCALHOST);
        let third: IpAddr = IpAddr::V4(Ipv4Addr::new(192, 0, 2, 3));
        let fourth: IpAddr = IpAddr::V6(Ipv6Addr::UNSPECIFIED);
        let mut addresses: DnsAddresses = DnsAddresses::empty();
        assert!(addresses.is_empty());
        for address in [first, second, first, third, second] {
            addresses.push(address)?;
        }
        assert_eq!(addresses.len(), 3);
        assert_eq!(addresses.push(fourth), Err(DnsAddressError::Capacity));
        assert!(addresses.iter().eq([first, second, third]));
        assert_eq!(format!("{addresses:?}"), "DnsAddresses { .. }");
        Ok(())
    }
}
