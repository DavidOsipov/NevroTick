// SPDX-License-Identifier: GPL-2.0-only
//! Restore bounded transport causes without losing an earlier TLS refusal.

use std::io;

use super::Failure;

/// The first transport refusal survives libraries that retry or suppress errors.
#[derive(Clone, Copy)]
pub(super) enum TransportFault {
    /// The original monotonic/cancellation guard closed this job.
    Control(Failure),
    /// One native operation failed; no later native operation is admitted.
    Io(io::ErrorKind),
}

/// Resolve a library result against the independently retained transport fault.
///
/// # Errors
/// Preserve Rustls' primary invalid-data refusal: its last-gasp alert I/O can
/// fail afterward. Otherwise restore the first fault, including faults obscured
/// by library retries or nominal success. With no fault, preserve the result.
pub(super) fn resolve<T>(
    fault: Option<TransportFault>,
    result: Result<T, Failure>,
    stage: fn(io::ErrorKind) -> Failure,
) -> Result<T, Failure> {
    let tls_primary = matches!(result, Err(Failure::Response(io::ErrorKind::InvalidData)));
    #[cfg(windows)]
    let tls_primary = tls_primary
        || matches!(
            result,
            Err(Failure::Handshake(io::ErrorKind::InvalidData)
                | Failure::Request(io::ErrorKind::InvalidData))
        );
    if tls_primary {
        return result;
    }
    match fault {
        Some(TransportFault::Control(failure)) => Err(failure),
        Some(TransportFault::Io(kind)) => Err(stage(kind)),
        None => result,
    }
}

#[cfg(test)]
mod tests {
    use super::{Failure, TransportFault, io, resolve};

    /// A known protocol refusal precedes a failing best-effort TLS alert write.
    ///
    /// # Panics
    /// Panics if alert I/O replaces the earlier TLS invalid-data category.
    #[test]
    fn tls_refusal_precedes_alert_write_failure() {
        let result: Result<(), Failure> = Err(Failure::Response(io::ErrorKind::InvalidData));
        assert_eq!(
            resolve(
                Some(TransportFault::Io(io::ErrorKind::BrokenPipe)),
                result,
                Failure::Response,
            ),
            result,
        );
    }

    /// A deadline reached during the alert cannot replace its earlier TLS cause.
    ///
    /// # Panics
    /// Panics if a later alert deadline replaces the primary TLS refusal.
    #[test]
    fn tls_refusal_precedes_alert_deadline() {
        let result: Result<(), Failure> = Err(Failure::Response(io::ErrorKind::InvalidData));
        assert_eq!(
            resolve(
                Some(TransportFault::Control(Failure::Deadline)),
                result,
                Failure::Response,
            ),
            result,
        );
    }

    /// Nominal library success cannot hide any independently observed fault.
    ///
    /// # Panics
    /// Panics if nominal success escapes a retained transport/control refusal.
    #[test]
    fn suppressed_fault_never_becomes_success() {
        for (fault, expected) in [
            (
                TransportFault::Io(io::ErrorKind::BrokenPipe),
                Failure::Response(io::ErrorKind::BrokenPipe),
            ),
            (
                TransportFault::Control(Failure::Cancelled),
                Failure::Cancelled,
            ),
            (
                TransportFault::Control(Failure::Deadline),
                Failure::Deadline,
            ),
        ] {
            assert_eq!(
                resolve(Some(fault), Ok(7_u8), Failure::Response),
                Err(expected)
            );
        }
    }

    /// A library retry's generic error still restores the actual first refusal.
    ///
    /// # Panics
    /// Panics if retry-obscured errors lose the original cancellation/I/O cause.
    #[test]
    fn retry_obscured_fault_keeps_original_category() {
        for kind in [io::ErrorKind::Other, io::ErrorKind::Interrupted] {
            let result: Result<(), Failure> = Err(Failure::Response(kind));
            assert_eq!(
                resolve(
                    Some(TransportFault::Control(Failure::Cancelled)),
                    result,
                    Failure::Response,
                ),
                Err(Failure::Cancelled),
            );
            assert_eq!(
                resolve(
                    Some(TransportFault::Io(io::ErrorKind::TimedOut)),
                    result,
                    Failure::Response,
                ),
                Err(Failure::Response(io::ErrorKind::TimedOut)),
            );
        }
    }

    /// Without an independent fault, neither success nor rejection is rewritten.
    ///
    /// # Panics
    /// Panics if an unfaulted success or original failure is changed.
    #[test]
    fn unfaulted_results_are_preserved() {
        assert_eq!(resolve(None, Ok(7_u8), Failure::Response), Ok(7));
        for error in [
            Failure::Cookies,
            Failure::Response(io::ErrorKind::InvalidData),
        ] {
            let result: Result<(), Failure> = Err(error);
            assert_eq!(resolve(None, result, Failure::Response), result);
        }
    }
}
