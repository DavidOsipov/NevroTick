// SPDX-License-Identifier: GPL-2.0-only
//! Select one bounded ordered trio before a controlled measurement starts.

use super::{ExchangeError, PEERS, RetryRoundPeer, RoundError};
use std::{array, num::NonZeroU32, time::Instant};

/// The existing controlled active-source cap also bounds selection work.
const MAX_CANDIDATES: usize = 16;
/// Every unordered trio is considered at most once.
const MAX_TRIOS: usize = MAX_CANDIDATES * (MAX_CANDIDATES - 1) * (MAX_CANDIDATES - 2) / 6;
/// Bound random draws even when the randomness source never returns an admissible rank.
const MAX_DRAWS: usize = 8;
/// Each index represents one possible schedule for the same selected trio.
const ORDERS: [[usize; PEERS]; 6] = [
    [0, 1, 2],
    [0, 2, 1],
    [1, 0, 2],
    [1, 2, 0],
    [2, 0, 1],
    [2, 1, 0],
];

/// Select and reborrow three original bindings without issuing network work.
///
/// # Errors
/// Refuses invalid roster cardinality or identity, lack of a feasible trio, or entropy failure.
pub(super) fn select<'a>(
    candidates: &'a mut [RetryRoundPeer<'_>],
    observed_at: Instant,
    fallback: bool,
) -> Result<[RetryRoundPeer<'a>; PEERS], RoundError> {
    let selected: [usize; PEERS] = choose_indices(candidates, observed_at, fallback, || {
        getrandom::u32().map_err(|_| ExchangeError::Entropy)
    })?;
    let [first, second, third]: [&mut RetryRoundPeer<'_>; PEERS] = candidates
        .get_disjoint_mut(selected)
        .map_err(|_| RoundError::Configuration)?;
    Ok([first.reborrow(), second.reborrow(), third.reborrow()])
}

/// Enumerate feasible unordered trios and draw one of their ordered schedules.
///
/// # Errors
/// Refuses invalid roster cardinality or identity, lack of a feasible trio, or entropy failure.
fn choose_indices(
    candidates: &[RetryRoundPeer<'_>],
    observed_at: Instant,
    fallback: bool,
    draw: impl FnMut() -> Result<u32, ExchangeError>,
) -> Result<[usize; PEERS], RoundError> {
    if !(PEERS..=MAX_CANDIDATES).contains(&candidates.len()) {
        return Err(RoundError::Configuration);
    }
    for (index, candidate) in candidates.iter().enumerate() {
        if candidates[..index]
            .iter()
            .any(|prior| !candidate.peer.distinct_identity(prior.peer))
        {
            return Err(RoundError::Configuration);
        }
    }
    let ready: [bool; MAX_CANDIDATES] = array::from_fn(|index| {
        candidates
            .get(index)
            .is_some_and(|candidate| candidate.retry.check(observed_at).is_ok())
    });
    let mut trios: [[usize; PEERS]; MAX_TRIOS] = [[0; PEERS]; MAX_TRIOS];
    let mut count: usize = 0;
    for (first, left) in candidates.iter().enumerate() {
        if !ready[first] {
            continue;
        }
        for (second, middle) in candidates.iter().enumerate().skip(first + 1) {
            if !ready[second] || !compatible(left, middle, fallback) {
                continue;
            }
            for (third, right) in candidates.iter().enumerate().skip(second + 1) {
                if !ready[third]
                    || !compatible(left, right, fallback)
                    || !compatible(middle, right, fallback)
                {
                    continue;
                }
                if fallback && [left, middle, right].iter().all(|peer| peer.tcp.is_none()) {
                    continue;
                }
                trios[count] = [first, second, third];
                count += 1;
            }
        }
    }
    let orders: NonZeroU32 = u32::try_from(count * ORDERS.len())
        .ok()
        .and_then(NonZeroU32::new)
        .ok_or(RoundError::SelectionUnavailable)?;
    let rank: usize = draw_rank(orders, draw)?;
    let trio: [usize; PEERS] = trios[rank / ORDERS.len()];
    Ok(ORDERS[rank % ORDERS.len()].map(|position| trio[position]))
}

/// Check both independent identities, including the feasible one-peer route changes.
fn compatible(left: &RetryRoundPeer<'_>, right: &RetryRoundPeer<'_>, fallback: bool) -> bool {
    left.peer.independent(right.peer)
        && (!fallback
            || (left.tcp != Some(right.peer.endpoint) && right.tcp != Some(left.peer.endpoint)))
}

/// Produce a uniform rank by accepting only complete buckets of the draw range.
///
/// # Errors
/// Refuses exhausted redraws, a failed entropy draw or an unrepresentable rank.
fn draw_rank(
    orders: NonZeroU32,
    mut draw: impl FnMut() -> Result<u32, ExchangeError>,
) -> Result<usize, RoundError> {
    let entire_range: u64 = u64::from(u32::MAX) + 1;
    let cutoff: u64 = entire_range - entire_range % u64::from(orders.get());
    for _ in 0..MAX_DRAWS {
        let value: u32 = draw().map_err(RoundError::Exchange)?;
        if u64::from(value) < cutoff {
            return usize::try_from(value % orders).map_err(|_| RoundError::Configuration);
        }
    }
    Err(RoundError::Exchange(ExchangeError::Entropy))
}

#[cfg(test)]
mod tests {
    use super::{ExchangeError, NonZeroU32, RetryRoundPeer, RoundError, choose_indices, draw_rank};
    use crate::roughtime::{RootKey, exchange::retry::RetryState, round::RoundPeer};
    use ed25519_dalek::SigningKey;
    use std::{array, net::SocketAddr, num::NonZeroU64, time::Instant};

    /// Every rank selects a distinct ordering of the only pairwise independent trio.
    ///
    /// # Panics
    /// Panics if an eligible order is lost or the first, incompatible peer is chosen.
    #[test]
    fn ranks_reach_each_order_of_the_feasible_trio() {
        let roots: [RootKey; 4] = [11_u8, 22, 33, 44]
            .map(|seed| RootKey(SigningKey::from_bytes(&[seed; 32]).verifying_key()));
        let mut retries: [RetryState; 4] = array::from_fn(|_| RetryState::default());
        let one: NonZeroU64 = NonZeroU64::MIN;
        let two: NonZeroU64 = one.saturating_add(1);
        let three: NonZeroU64 = two.saturating_add(1);
        let groups: [(NonZeroU64, NonZeroU64); 4] =
            [(one, one), (one, two), (two, one), (three, three)];
        let mut candidates: Vec<RetryRoundPeer<'_>> = [5319_u16, 5320, 5321, 5322]
            .into_iter()
            .zip(&roots)
            .zip(&mut retries)
            .zip(groups)
            .map(
                |(((port, root), retry), (operator, clock))| RetryRoundPeer {
                    peer: RoundPeer::new(
                        SocketAddr::from(([127, 0, 0, 1], port)),
                        root,
                        operator,
                        clock,
                    ),
                    retry,
                    tcp: None,
                },
            )
            .collect();
        let expected: [[usize; 3]; 6] = [
            [1, 2, 3],
            [1, 3, 2],
            [2, 1, 3],
            [2, 3, 1],
            [3, 1, 2],
            [3, 2, 1],
        ];
        let now: Instant = Instant::now();
        for (rank, expected) in (0..6_u32).zip(expected) {
            assert!(
                choose_indices(&candidates, now, false, || Ok(rank))
                    .is_ok_and(|found| found == expected)
            );
        }
        let mut draws: usize = 0;
        assert!(matches!(
            choose_indices(&candidates, now, true, || {
                draws += 1;
                Ok(0)
            }),
            Err(RoundError::SelectionUnavailable)
        ));
        assert_eq!(draws, 0);
        candidates[1].tcp = Some(candidates[1].peer.endpoint);
        assert!(
            choose_indices(&candidates, now, true, || Ok(0)).is_ok_and(|found| found == [1, 2, 3])
        );
        candidates[1].tcp = Some(candidates[2].peer.endpoint);
        assert!(matches!(
            choose_indices(&candidates, now, true, || Ok(0)),
            Err(RoundError::SelectionUnavailable)
        ));
        candidates[0].peer.root = candidates[2].peer.root;
        assert!(matches!(
            choose_indices(&candidates, now, false, || Ok(0)),
            Err(RoundError::Configuration)
        ));
    }

    /// The rank crosses trio boundaries and reaches the final schedule at the roster cap.
    ///
    /// # Panics
    /// Panics if the selector loses a feasible trio or maps a rank to the wrong schedule.
    #[test]
    fn ranks_span_the_full_sixteen_peer_roster() {
        let roots: [RootKey; 16] = array::from_fn(|index| {
            let seed: u8 = u8::try_from(index).unwrap_or(0);
            RootKey(SigningKey::from_bytes(&[seed; 32]).verifying_key())
        });
        let mut retries: [RetryState; 16] = array::from_fn(|_| RetryState::default());
        let candidates: Vec<RetryRoundPeer<'_>> = (5400_u16..5416)
            .zip(&roots)
            .zip(&mut retries)
            .map(|((port, root), retry)| {
                let group: NonZeroU64 = NonZeroU64::MIN.saturating_add(u64::from(port));
                RetryRoundPeer {
                    peer: RoundPeer::new(
                        SocketAddr::from(([127, 0, 0, 1], port)),
                        root,
                        group,
                        group,
                    ),
                    retry,
                    tcp: None,
                }
            })
            .collect();
        let now: Instant = Instant::now();
        let cases: [(u32, [usize; 3]); 3] = [(5, [2, 1, 0]), (6, [0, 1, 3]), (3359, [15, 14, 13])];
        for (rank, expected) in cases {
            assert!(
                choose_indices(&candidates, now, false, || Ok(rank))
                    .is_ok_and(|found| found == expected)
            );
        }
    }

    /// Reject the incomplete tail, bound redraws, and preserve a failed entropy draw.
    ///
    /// # Errors
    /// Returns a fixture error if the selected rank count is unexpectedly zero.
    ///
    /// # Panics
    /// Panics on acceptance of a biased tail or incorrect failure accounting.
    #[test]
    fn rank_sampling_rejects_tail_and_stays_bounded() -> Result<(), &'static str> {
        let count: NonZeroU32 = NonZeroU32::new(6).ok_or("zero fixture count")?;
        let mut draws: array::IntoIter<u32, 2> = [u32::MAX, 5].into_iter();
        assert!(
            draw_rank(count, || draws.next().ok_or(ExchangeError::Entropy))
                .is_ok_and(|rank| rank == 5)
        );
        assert_eq!(draws.len(), 0);
        let mut attempts: usize = 0;
        assert!(matches!(
            draw_rank(count, || {
                attempts += 1;
                Ok(u32::MAX)
            }),
            Err(RoundError::Exchange(ExchangeError::Entropy))
        ));
        assert_eq!(attempts, 8);
        assert!(matches!(
            draw_rank(count, || Err(ExchangeError::Entropy)),
            Err(RoundError::Exchange(ExchangeError::Entropy))
        ));
        Ok(())
    }
}
