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
mod tests;
