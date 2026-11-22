// SPDX-License-Identifier: GPL-2.0-only
//! Literal diagnostic and invocation controls for the read-only process observation.

use std::cell::Cell;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use nevrotick_engine::{Config, ControlHandle, ControlSchedule, PreparedPilot, prepare};

use super::adjustment_observation::AdjustmentState;
use super::adjustment_observation_tests::RecordSink;
use super::leap_presentation::Observation;
use super::{CliError, observe, write_bound};

/// Preserve fixture and sink failures through the selected regression cases.
type Check = Result<(), Box<dyn std::error::Error>>;

/// Literal diagnostic for the supported default mode.
const COMPATIBILITY: &str = "event=leap_presentation status=observed mode=compatibility flags=0 reserved=0 mapping=unavailable\n";

/// Preserve each distinct observation, including the original failure and invalid fields.
///
/// # Errors
/// Return an unexpected output error.
/// # Panics
/// Reject any changed literal output or loss of an original value.
#[test]
fn presentation_projection_retains_exact_original_outcomes() -> Check {
    let cases: [(Observation, &str); 4] = [
        (Observation::Compatibility, COMPATIBILITY),
        (
            Observation::SixtySecond,
            "event=leap_presentation status=unexpected mode=sixty_second flags=1 reserved=0 mapping=unavailable\n",
        ),
        (
            Observation::QueryFailed(0),
            "event=leap_presentation status=unavailable reason=query_failed error=0 mapping=unavailable\n",
        ),
        (
            Observation::InvalidReply {
                flags: u32::MAX,
                reserved: 17,
            },
            "event=leap_presentation status=unavailable reason=invalid_reply flags=4294967295 reserved=17 mapping=unavailable\n",
        ),
    ];
    for (observation, expected) in cases {
        let mut output: Vec<u8> = Vec::new();
        observation.write(&mut output)?;
        assert_eq!(output, expected.as_bytes());
    }
    Ok(())
}

/// Outcomes of the actual CLI orchestration with injected observation and cleanup.
#[derive(Clone, Copy, Eq, PartialEq)]
enum Attempt {
    /// Normal publication, stopped execution and both diagnostics requested.
    Both,
    /// The first attempted bound record is refused.
    BoundRefused,
    /// The Engine refuses its entered runtime, then cleanup fails.
    EngineRefused,
    /// The Engine, cleanup and output all fail.
    EngineAndOutputRefused,
    /// The engine completes but the presentation record cannot be written.
    OutputRefused,
}

/// Require the one observation to follow binding, precede cleanup, survive the
/// original Engine failure and leave the original sink refusal in charge.
///
/// # Errors
/// Preserve a fixture setup, Engine preparation or output decoding failure.
/// # Panics
/// Reject a changed observation, operation or sink disposition.
#[test]
fn presentation_cli_orders_observation_cleanup_and_output() -> Check {
    for attempt in [
        Attempt::Both,
        Attempt::BoundRefused,
        Attempt::EngineRefused,
        Attempt::EngineAndOutputRefused,
        Attempt::OutputRefused,
    ] {
        assert_attempt(attempt)?;
    }
    Ok(())
}

/// Exercise the actual bound-publication and Engine execution kernel.
///
/// # Errors
/// Preserve an unexpected setup or diagnostic decoding failure.
/// # Panics
/// Reject changed invocation ordering, attempted outputs or error precedence.
fn assert_attempt(attempt: Attempt) -> Check {
    let peers: [UdpSocket; 2] = [
        UdpSocket::bind("127.0.0.1:0")?,
        UdpSocket::bind("127.0.0.1:0")?,
    ];
    let endpoints: [SocketAddr; 2] = [peers[0].local_addr()?, peers[1].local_addr()?];
    let config: Config = Config::new(endpoints, Duration::from_secs(1))?;
    let (pilot, control): (PreparedPilot, ControlHandle) = prepare(&config)?;
    control.stop();
    let schedule: ControlSchedule = ControlSchedule::new(None, None)?;
    let runtime: tokio::runtime::Runtime = tokio::runtime::Builder::new_current_thread().build()?;
    let engine_refuses: bool = matches!(
        attempt,
        Attempt::EngineRefused | Attempt::EngineAndOutputRefused
    );
    let entered: Option<tokio::runtime::EnterGuard<'_>> = engine_refuses.then(|| runtime.enter());
    let step: Cell<u8> = Cell::new(0);
    let mut bound: RecordSink = if attempt == Attempt::BoundRefused {
        RecordSink::refusing(0)
    } else {
        RecordSink::default()
    };
    let mut output: RecordSink = match attempt {
        Attempt::EngineAndOutputRefused | Attempt::OutputRefused => RecordSink::refusing(0),
        _ => RecordSink::default(),
    };
    let result: Result<(), CliError> = observe(
        pilot,
        schedule,
        |pilot: &PreparedPilot| {
            assert_eq!(step.replace(1), 0);
            write_bound(pilot, &mut bound)
        },
        || {
            let expected: u8 = match attempt {
                Attempt::Both => 4,
                Attempt::BoundRefused => 1,
                Attempt::EngineRefused
                | Attempt::EngineAndOutputRefused
                | Attempt::OutputRefused => 3,
            };
            assert_eq!(step.replace(5), expected);
            if engine_refuses { Err(()) } else { Ok(()) }
        },
        || {
            assert_eq!(step.replace(6), 5);
            &mut output
        },
        (attempt == Attempt::Both).then_some(|| {
            if step.get() == 1 {
                step.set(2);
            } else {
                assert_eq!(step.replace(4), 3);
            }
            Ok(AdjustmentState {
                adjustment: 21,
                increment: 7,
                periodic_adjustment_disabled: true,
            })
        }),
        Some(|| {
            let expected: u8 = if attempt == Attempt::Both { 2 } else { 1 };
            assert_eq!(step.replace(3), expected);
            Observation::Compatibility
        }),
    );
    drop(entered);
    assert_result(attempt, &result, step.get(), &bound, &output)
}

/// Check the exact attempt disposition and the output that reached the public sink.
///
/// # Errors
/// Preserve invalid diagnostic encoding.
/// # Panics
/// Reject changed observations, continued writes or a false completion.
fn assert_result(
    attempt: Attempt,
    result: &Result<(), CliError>,
    step: u8,
    bound: &RecordSink,
    output: &RecordSink,
) -> Check {
    match attempt {
        Attempt::Both => {
            assert!(result.is_ok());
            assert_eq!(step, 6);
            assert_eq!(std::str::from_utf8(&bound.bytes)?.lines().count(), 2);
            let text: &str = std::str::from_utf8(&output.bytes)?;
            let (adjustment, normal): (&str, &str) = text
                .split_once(COMPATIBILITY)
                .ok_or("missing original presentation record")?;
            assert_eq!(adjustment.lines().count(), 3);
            assert!(adjustment.starts_with("event=clock_adjustment point=before "));
            assert!(adjustment.ends_with("ordering=ordered mapping=unavailable\n"));
            assert_eq!(normal.matches(" status=stopped ").count(), 2);
            assert_eq!(normal.matches("event=done elapsed_ns=").count(), 1);
        }
        Attempt::BoundRefused => {
            assert!(matches!(result, Err(CliError::Output)));
            assert_eq!(step, 5);
            assert_eq!(bound.failures, 1);
            assert_eq!(output.bytes.as_slice(), b"");
            assert_eq!(output.failures, 0);
        }
        Attempt::EngineRefused | Attempt::EngineAndOutputRefused => {
            assert!(matches!(
                result,
                Err(CliError::ResumeUnregistration {
                    primary: Some("entered_runtime")
                })
            ));
            assert_eq!(step, 6);
            if attempt == Attempt::EngineRefused {
                assert_eq!(output.bytes, COMPATIBILITY.as_bytes());
                assert_eq!(output.failures, 0);
            } else {
                assert_eq!(output.bytes.as_slice(), b"");
                assert_eq!(output.failures, 1);
            }
        }
        Attempt::OutputRefused => {
            assert!(matches!(result, Err(CliError::Output)));
            assert_eq!(step, 6);
            assert_eq!(output.bytes.as_slice(), b"");
            assert_eq!(output.failures, 1);
        }
    }
    Ok(())
}
